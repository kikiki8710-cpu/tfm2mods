use mod_api::*;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::time::Instant;

const MOD_ID: &str = "community_reaction_mod";

// ═══════════════ 관전 하이라이트 수집 (Spectator_Chat 프레임 스캔 이식) ═══════════════
// 경기 관전 중(리플레이=db.game_view / 즉시보기=raw 오프셋) 프레임 이벤트를 시간예산으로
// 스캔해 연속킬 / n인궁 / 포탑 데스 / 골드 역전을 뽑아, export 텍스트에 세트별 블록으로 붙인다.
// 이벤트 필드 정본: ANA\discovered-game-event-data-reference.md

const HL_MULTIKILL_WINDOW_TICKS: i64 = 600; // 같은 선수 연속킬 인정 간격 (10초, 60tick/s)
const HL_ULT_HIT_WINDOW: usize = 45;        // 궁 시전 후 적중 집계 창 (~0.75초, 프레임) — 넓히면 한타 오탐
const HL_GOLD_COHERENT_FRAMES: usize = 300; // 골드 스냅샷 10명 갱신시점 최대 편차(5초) — 넘으면 반팀 stale=가짜 스파이크
const HL_ULT_MIN_HITS: usize = 3;           // n인궁 하이라이트 최소 인원
const HL_SKILL_MIN_HITS: usize = 4;         // 광역 스킬 하이라이트 최소 인원 (궁보다 흔해서 높게)
const HL_HIT_MIN_DMG: i64 = 100;            // 피해 카운트 최소 데미지 (미니언 잔챙이 칩댐 제외)
const HL_GOLD_COMEBACK_GAP: i64 = 10_000;   // 역전 인정 최대 골드 열세
const HL_SCAN_CHUNK: usize = 12000;         // 프레임당 스캔 상한(주 제한은 시간예산)
const HL_SCAN_BUDGET_US: u128 = 3500;       // 프레임당 스캔 시간예산(µs).
    // 실측: 1.2ms→풀스캔 ~19초(4만 프레임). 배속 시청 시 세트가 먼저 끝나 스캔이 리셋됨
    // → 3.5ms로 ~7초 완주. 관전 중 화면은 여유 있어 스터터 없음(Spectator_Chat 1.5ms와 병행 고려).
const HL_MAX_BLOCKS: usize = 8;             // 보관하는 세트 블록 수 (BO5 + 여유)
const LOG_ENABLED: bool = false;            // 디버그 덤프(mods/<MOD_ID>/hl_debug.txt, hl_scan.txt)

// ★0.5.0 즉시보기(라이브) 절대 오프셋 — SDK PDB 타입정보로 정적 재도출(2026-07-08).
//   client.events Vec = db+5728 (cap@+0 / ptr@+8 / len@+16). SDK-ABI 종속 → 패치마다 재도출.
//   산식: ClientDatabase.scene(+0x1328) + InGame payload(+8) + GameClient.events(+0x330) = 0x1660 = 5728
//   (0.4.14 = 0xF90 + 8 + 0x3D0 = 4968 — 같은 산식으로 구 확정치 재현 확인)
const LIVE_EVENTS_OFF: usize = 5744;  // 0.5.0_3(구5728): ClientDatabase scene 앞 +0x10 필드추가 → db-절대 전부 +0x10(scene 내부 상대는 불변). 런타임 스캔 실증.

// game_view 타입(&Vec<Arc<GameFrameData>>)을 anchor 로 raw 포인터를 &T 캐스팅 (Spectator_Chat 동일 기법)
unsafe fn cast_as<T>(_anchor: &Option<&T>, ptr: usize) -> &'static T { &*(ptr as *const T) }

static SCAN_TOTAL: AtomicUsize = AtomicUsize::new(0);
static SCAN_POS: AtomicUsize = AtomicUsize::new(0);
static SCAN_DONE: AtomicBool = AtomicBool::new(false);
static BC_CNT: AtomicUsize = AtomicUsize::new(0); // 진단 브레드크럼 주기 카운터
static GALLERY_OPENED: AtomicBool = AtomicBool::new(false); // 세이브 로드(InGame 진입)마다 갤러리 1회 오픈 (메뉴 복귀 시 리셋)

fn hl_breadcrumb(msg: &str) {
    if !LOG_ENABLED { return; }
    let c = BC_CNT.fetch_add(1, Ordering::Relaxed);
    if c % 120 != 0 { return; } // ~2초에 1회
    let _ = fs::write(Path::new("mods").join(MOD_ID).join("hl_scan.txt"), msg);
}

static HL_ANCHORS: Mutex<Vec<(usize, i64)>> = Mutex::new(Vec::new()); // (frame, tick) — Score 이벤트 기준 시간축
static HL_KILLS: Mutex<Vec<(usize, Option<String>, String, i64)>> = Mutex::new(Vec::new()); // (frame, killer챔프(None=포탑/처형), killed챔프, killer_team)
static HL_ULTS: Mutex<Vec<(usize, i64)>> = Mutex::new(Vec::new()); // (frame, 궁 시전 entity id)
static HL_SKILLS: Mutex<Vec<(usize, i64)>> = Mutex::new(Vec::new()); // (frame, 스킬 시전 entity id)
static HL_HITS: Mutex<Vec<(usize, i64)>> = Mutex::new(Vec::new()); // (frame, 피해/CC 당한 선수 entity id)
static HL_EFF: Mutex<Vec<(usize, i64, i64)>> = Mutex::new(Vec::new()); // (frame, caster id, target id) — EffectApplyed 시전자 귀속
static HL_GOLDLINE: Mutex<Vec<(i64, i64)>> = Mutex::new(Vec::new()); // (tick, 블루-레드 팀골드차) — 10명 스냅샷 완성 후만
static HL_NEXUS: Mutex<Vec<(i64, i64)>> = Mutex::new(Vec::new()); // (nexus entity id, team)
static HL_NEXUS_DIE: Mutex<Option<i64>> = Mutex::new(None); // 파괴된 넥서스의 team(=패배팀)
static HL_ID2NAME: Mutex<Option<HashMap<i64, String>>> = Mutex::new(None); // entity id → 선수명
static HL_ID2TEAM: Mutex<Option<HashMap<i64, i64>>> = Mutex::new(None); // entity id → 팀(0/1)
static HL_CHAMP2NAME: Mutex<Option<HashMap<String, String>>> = Mutex::new(None); // 챔피언키 → 선수명
static HL_POSGOLD: Mutex<Option<HashMap<(i64, String), (i64, usize)>>> = Mutex::new(None); // (team, 포지션) → (현재 골드, 갱신 frame)
static HL_LASTSCORE: Mutex<(i64, i64)> = Mutex::new((0, 0)); // 마지막 Score.scores [blue, red] — 승자 판정 폴백

struct HlBlock {
    sig: u64,
    match_id: Option<usize>, // 관전 시점에 식별한 매치 id (정확 매칭용; None이면 이름 폴백)
    names: Vec<String>,
    score: (i64, i64),       // 세트 최종 킬스코어(세트번호 매칭용)
    lines: Vec<String>,
}
static HL_BLOCKS: Mutex<Vec<HlBlock>> = Mutex::new(Vec::new()); // 완성 블록 (export 시 로스터 매칭으로 필터)

// ── 대회 형식 설정(GamePlayOption) 캡처 — 서버측 Database 에만 존재(ClientDatabase 미노출) ──
// GamePlayOption = Database+0x0 (0.5.0 크기 0x740, 구 0.4.14 = 0x730). ⚠repr(Rust) 재배치로 u8 클러스터는 구조체 끝:
//   ★0.5.0: league_rule@+0x729 / cup_rule@+0x72a / group_stage_format@+0x72b
//   (ghidra 확정 2026-07-08: GamePlayOption::serialize 0x845280 / deserialize 0x2e9ff0 / clone 0x484b50,
//    Database::serialize 0x4bd6d0 이 GamePlayOption 다음 필드를 +0x740 으로 넘김 = 크기 확증.)
//   0.5.0 에서 데스매치 모드 추가 → u64 신규 2개(deathmatch_time@+0x718, ban_count@+0x720)가
//   u8 클러스터 앞 align-8 그룹에 삽입되어 클러스터 전체가 정확히 +0x10 시프트 (구 0.4.14 = +0x719~71b).
//   구 문서 +0x151~153 = 선언순 가정 오류(폐기) — 그 자리는 scout 필터 내부.
//   enum disc 는 0.4.14와 동일: league_rule 0=double/1=tournament, cup_rule 0=tournament/1=double(역순!),
//   group_stage_format 0=league_group/1=swiss.
// 로컬서버 = 같은 프로세스 → 서버 확장에서 읽어 static 으로 전달.
static OPTION_RAW: Mutex<Option<(u8, u8, u8)>> = Mutex::new(None); // (league_rule, cup_rule, group_stage_format)

static GPO_DUMPED: AtomicBool = AtomicBool::new(false);

fn capture_gpo(ctx: &ServerModContext) {
    unsafe {
        let base = (&*ctx.database) as *const _ as usize;
        let lr = *((base + 0x729) as *const u8);
        let cr = *((base + 0x72a) as *const u8);
        let gf = *((base + 0x72b) as *const u8);
        *OPTION_RAW.lock().unwrap_or_else(|e| e.into_inner()) = Some((lr, cr, gf));
        // 진단 덤프(세션당 1회): u8 클러스터 +0x708..+0x740 — champion_add_count@+0x708(u64),
        // banpick_time_limit@+0x710(u64), deathmatch_time@+0x718(u64), ban_count@+0x720(u64),
        // view_stat@+0x728, 규칙 3종@+0x729~72b, difficulty@+0x733, game_mode@+0x738 등
        if !GPO_DUMPED.swap(true, Ordering::Relaxed) {
            let bytes: Vec<String> = (0x700..0x740)
                .map(|off| format!("{:02x}", *((base + off) as *const u8)))
                .collect();
            let dump = format!(
                "GamePlayOption u8 클러스터 상대덤프\n+0x700..+0x740: {}\nlr(+729)={} cr(+72a)={} gf(+72b)={}\n",
                bytes.join(" "), lr, cr, gf
            );
            let _ = fs::write(Path::new("mods").join(MOD_ID).join("gpo_debug.txt"), dump);
        }
    }
}

struct ReactionServerExt;
impl ModServerExtension for ReactionServerExt {
    fn on_server_start(&self, ctx: &mut ServerModContext) { capture_gpo(ctx); }
    fn before_management_tick(&self, ctx: &mut ServerModContext) { capture_gpo(ctx); }
    fn after_management_tick(&self, ctx: &mut ServerModContext) { capture_gpo(ctx); }
}

// enum 값 → 한국어 라벨 (0.4.14 ghidra disc 확정 + 인게임 대진 실측으로 의미 검증:
//   league_rule = "리그 플레이오프 규칙" — 0=double=더블 엘리미네이션(실측 브래킷 일치)/1=tournament)
fn league_rule_kr(v: u8) -> String {
    match v { 0 => "더블 엘리미네이션".to_string(), 1 => "토너먼트(싱글 엘리미네이션)".to_string(), _ => format!("league_rule={}", v) }
}
fn cup_rule_kr(v: u8) -> String {
    match v { 0 => "토너먼트(싱글 엘리미네이션)".to_string(), 1 => "더블 엘리미네이션".to_string(), _ => format!("cup_rule={}", v) }
}
fn group_stage_format_kr(v: u8) -> String {
    match v { 0 => "조별 리그".to_string(), 1 => "스위스 스테이지".to_string(), _ => format!("group_stage_format={}", v) }
}

fn hl_reset_scan() {
    SCAN_DONE.store(false, Ordering::Relaxed);
    SCAN_POS.store(0, Ordering::Relaxed);
    SCAN_TOTAL.store(0, Ordering::Relaxed);
    HL_ANCHORS.lock().unwrap_or_else(|e| e.into_inner()).clear();
    HL_KILLS.lock().unwrap_or_else(|e| e.into_inner()).clear();
    HL_ULTS.lock().unwrap_or_else(|e| e.into_inner()).clear();
    HL_SKILLS.lock().unwrap_or_else(|e| e.into_inner()).clear();
    HL_HITS.lock().unwrap_or_else(|e| e.into_inner()).clear();
    HL_EFF.lock().unwrap_or_else(|e| e.into_inner()).clear();
    HL_GOLDLINE.lock().unwrap_or_else(|e| e.into_inner()).clear();
    HL_NEXUS.lock().unwrap_or_else(|e| e.into_inner()).clear();
    *HL_NEXUS_DIE.lock().unwrap_or_else(|e| e.into_inner()) = None;
    *HL_ID2NAME.lock().unwrap_or_else(|e| e.into_inner()) = None;
    *HL_ID2TEAM.lock().unwrap_or_else(|e| e.into_inner()) = None;
    *HL_CHAMP2NAME.lock().unwrap_or_else(|e| e.into_inner()) = None;
    *HL_POSGOLD.lock().unwrap_or_else(|e| e.into_inner()) = None;
    *HL_LASTSCORE.lock().unwrap_or_else(|e| e.into_inner()) = (0, 0);
}

fn hl_field_i64(s: &str, key: &str) -> Option<i64> {
    let p = s.find(key)? + key.len();
    let r = &s[p..];
    let r = r.trim_start_matches(|c: char| c == ' ' || c == ':');
    let e = r.find(|c: char| !(c.is_ascii_digit() || c == '-')).unwrap_or(r.len());
    if e == 0 { return None; }
    r[..e].parse().ok()
}
fn hl_field_quoted(s: &str, key: &str) -> Option<String> {
    let p = s.find(key)? + key.len();
    let r = &s[p..];
    let q1 = r.find('"')? + 1;
    let q2 = r[q1..].find('"')? + q1;
    Some(r[q1..q2].to_string())
}
fn hl_field_word(s: &str, key: &str) -> Option<String> {
    let p = s.find(key)? + key.len();
    let r = &s[p..];
    let r = r.trim_start_matches(|c: char| c == ' ' || c == ':');
    let e = r.find(|c: char| !(c.is_ascii_alphanumeric() || c == '_')).unwrap_or(r.len());
    if e == 0 { None } else { Some(r[..e].to_string()) }
}

// 프레임 인덱스 → 게임 tick 보간 (anchors = Score 이벤트의 (frame, tick))
fn hl_interp(frame_idx: usize, anchors: &[(usize, i64)]) -> i64 {
    if anchors.is_empty() { return -1; }
    match anchors.binary_search_by_key(&frame_idx, |&(f, _)| f) {
        Ok(i) => anchors[i].1,
        Err(i) => {
            if i == 0 { anchors[0].1 }
            else if i >= anchors.len() { anchors[anchors.len() - 1].1 }
            else {
                let (f0, t0) = anchors[i - 1];
                let (f1, t1) = anchors[i];
                if f1 == f0 { t0 } else { t0 + (t1 - t0) * (frame_idx as i64 - f0 as i64) / (f1 as i64 - f0 as i64) }
            }
        }
    }
}
fn hl_fmt_tick(tk: i64) -> String {
    let s = tk.max(0) / 60; // 60 tick/초
    format!("{:02}:{:02}", s / 60, s % 60)
}

fn node_exists(n: &Node, id: &str) -> bool {
    if n.id == id { return true; }
    n.child.iter().any(|c| node_exists(c, id))
}

// 매 프레임 호출: 관전 화면이면 프레임 이벤트를 시간예산 내에서 점진 스캔.
fn hl_collect(data: &ClientData, root: &Node) {
    // 매치 관전 화면(game_time 노드)이 아니면 스캔 상태만 리셋(완성 블록은 유지).
    // ⚠ 이 게이트가 라이브 raw 포인터 stale-read 방지의 핵심 (Spectator_Chat 동일).
    if !node_exists(root, "game_time") {
        if SCAN_TOTAL.load(Ordering::Relaxed) != 0 || SCAN_POS.load(Ordering::Relaxed) != 0 { hl_reset_scan(); }
        hl_breadcrumb("state=관전화면 아님 (game_time 노드 없음)\n");
        return;
    }
    let db = data.db();

    // 프레임 소스: 리플레이(game_view Some) vs 즉시보기(None → db 절대 오프셋)
    let gv_anchor: Option<&_> = db.game_view.as_ref().map(|gv| &gv.client.events);
    let frames: &Vec<_> = if let Some(gv) = db.game_view.as_ref() {
        &gv.client.events
    } else {
        unsafe {
            let base = (&*db as *const _) as usize;
            let ce = base + LIVE_EVENTS_OFF;
            let rp = *((ce + 8) as *const usize);
            let rl = *((ce + 16) as *const usize);
            if rp > 0x10000 && rp < (1usize << 48) && rl >= 1 && rl <= 10_000_000 {
                cast_as(&gv_anchor, ce)
            } else {
                hl_breadcrumb("state=라이브 frames 미준비 (포인터 무효)\n");
                return; // frames 아직 준비 안 됨
            }
        }
    };

    let total = frames.len();
    hl_breadcrumb(&format!(
        "state=관전중 gv={} total={} pos={} done={}\n",
        db.game_view.is_some(), total, SCAN_POS.load(Ordering::Relaxed), SCAN_DONE.load(Ordering::Relaxed)
    ));
    let prev_total = SCAN_TOTAL.swap(total, Ordering::Relaxed);
    if SCAN_DONE.load(Ordering::Relaxed) { return; }
    if prev_total != total {
        // 아직 프레임 생성 중 — 안정될 때까지 대기. 스캔이 이미 시작됐다면 처음부터 다시.
        if SCAN_POS.load(Ordering::Relaxed) > 0 {
            hl_reset_scan();
            SCAN_TOTAL.store(total, Ordering::Relaxed);
        }
        return;
    }
    if total <= 100 { return; }

    let mut pos = SCAN_POS.load(Ordering::Relaxed);
    let end = (pos + HL_SCAN_CHUNK).min(total);
    {
        let mut anchors = HL_ANCHORS.lock().unwrap_or_else(|e| e.into_inner());
        let mut kills = HL_KILLS.lock().unwrap_or_else(|e| e.into_inner());
        let mut ults = HL_ULTS.lock().unwrap_or_else(|e| e.into_inner());
        let mut skills = HL_SKILLS.lock().unwrap_or_else(|e| e.into_inner());
        let mut hits = HL_HITS.lock().unwrap_or_else(|e| e.into_inner());
        let mut effs = HL_EFF.lock().unwrap_or_else(|e| e.into_inner());
        let mut goldline = HL_GOLDLINE.lock().unwrap_or_else(|e| e.into_inner());
        let mut nexus = HL_NEXUS.lock().unwrap_or_else(|e| e.into_inner());
        let mut nexus_die = HL_NEXUS_DIE.lock().unwrap_or_else(|e| e.into_inner());
        let mut idg = HL_ID2NAME.lock().unwrap_or_else(|e| e.into_inner());
        let id2name = idg.get_or_insert_with(HashMap::new);
        let mut itg = HL_ID2TEAM.lock().unwrap_or_else(|e| e.into_inner());
        let id2team = itg.get_or_insert_with(HashMap::new);
        let mut cng = HL_CHAMP2NAME.lock().unwrap_or_else(|e| e.into_inner());
        let champ2name = cng.get_or_insert_with(HashMap::new);
        let mut pgg = HL_POSGOLD.lock().unwrap_or_else(|e| e.into_inner());
        let posgold = pgg.get_or_insert_with(HashMap::new);

        let t0 = Instant::now();
        while pos < end {
            if pos % 16 == 0 && t0.elapsed().as_micros() > HL_SCAN_BUDGET_US { break; }
            for ev in frames[pos].events.iter() {
                let s = format!("{:?}", ev);
                let b = s.as_bytes();
                if s.starts_with("EntityEvent") {
                    let Some(id) = hl_field_i64(&s, "id:") else { continue; };
                    if s.contains("Damaged(") {
                        // 선수만 + 스킬 피해만(attack_type: Skill — 미니언/포탑/평타=BaseAttack 배제) + 칩댐 제외
                        if id2name.contains_key(&id)
                            && s.contains("attack_type: Skill")
                            && hl_field_i64(&s, "damage:").unwrap_or(0) >= HL_HIT_MIN_DMG {
                            hits.push((pos, id));
                        }
                    } else if s.contains("Stun(") || s.contains("Airborne(") || s.contains("Bind(") {
                        if id2name.contains_key(&id) { hits.push((pos, id)); }
                    } else if s.contains("ty: Die") {
                        if let Some(&(_, nteam)) = nexus.iter().find(|&&(nid, _)| nid == id) {
                            *nexus_die = Some(nteam); // 넥서스 파괴 = 그 팀 패배
                        }
                    } else if s.contains("action: \"ult\"") {
                        if id2name.contains_key(&id) { ults.push((pos, id)); }
                    } else if s.contains("action: \"skill") {
                        if id2name.contains_key(&id) { skills.push((pos, id)); } // skill/skill1/skill2
                    }
                    continue;
                }
                if s.starts_with("EffectApplyed") {
                    // 시전자→대상 귀속 정보 (선수↔선수만)
                    if let (Some(c), Some(t)) = (hl_field_i64(&s, "caster_id:"), hl_field_i64(&s, "target_id:")) {
                        if id2name.contains_key(&c) && id2name.contains_key(&t) { effs.push((pos, c, t)); }
                    }
                    continue;
                }
                if s.starts_with("EntitySpawn") {
                    if let Some(id) = hl_field_i64(&s, "id:") {
                        if let Some(pn) = hl_field_quoted(&s, "player_name:") {
                            if !pn.is_empty() {
                                // 챔피언 (재spawn 마다 새 id → 갱신). 분신(is_illusion)은 챔피언 취급 안 함.
                                if !s.contains("is_illusion: true") {
                                    id2name.insert(id, pn);
                                    if let Some(tp) = s.find("team:") {
                                        if let Some(tm) = hl_field_i64(&s[tp..], "Player(") { id2team.insert(id, tm); }
                                    }
                                }
                            } else if hl_field_quoted(&s, " name:").as_deref() == Some("nexus") {
                                if let Some(tp) = s.find("team:") {
                                    if let Some(tm) = hl_field_i64(&s[tp..], "Player(") { nexus.push((id, tm)); }
                                }
                            }
                        }
                    }
                    continue;
                }
                if b.first() == Some(&b'E') { continue; }
                if s.starts_with("PlayerStatistics") {
                    if let (Some(team), Some(pos_s)) = (hl_field_i64(&s, "team:"), hl_field_word(&s, "position:")) {
                        let g = hl_field_i64(&s, "gold:").unwrap_or(0);
                        posgold.insert((team, pos_s), (g, pos));
                        if posgold.len() >= 10 {
                            // 10명 스냅샷이 최근 5초 내에 모두 갱신된 경우만 기록
                            // (반팀만 최신인 순간의 부분합 = 수만G 가짜 스파이크 방지)
                            let (mut gb, mut gr) = (0i64, 0i64);
                            let (mut fmin, mut fmax) = (usize::MAX, 0usize);
                            for ((t, _), &(v, fu)) in posgold.iter() {
                                if *t == 0 { gb += v; } else { gr += v; }
                                if fu < fmin { fmin = fu; }
                                if fu > fmax { fmax = fu; }
                            }
                            if fmax - fmin <= HL_GOLD_COHERENT_FRAMES {
                                let tk = hl_interp(pos, &anchors);
                                goldline.push((tk, gb - gr));
                            }
                        }
                    }
                    continue;
                }
                if b.first() == Some(&b'P') { continue; }
                if s.starts_with("Score") {
                    if let Some(t) = hl_field_i64(&s, "tick:") { anchors.push((pos, t)); }
                    // scores: [b, r] — 승자 판정용 최종 킬스코어 추적
                    if let Some(p) = s.find("scores:") {
                        let r = &s[p..];
                        if let Some(lb) = r.find('[') {
                            let inner = &r[lb + 1..];
                            let n: Vec<i64> = inner.split(|c| c == ',' || c == ']')
                                .filter_map(|x| x.trim().parse().ok()).take(2).collect();
                            if n.len() == 2 { *HL_LASTSCORE.lock().unwrap_or_else(|e| e.into_inner()) = (n[0], n[1]); }
                        }
                    }
                } else if s.starts_with("KillEvent") {
                    let killer = if s.contains("killer: None") { None } else { hl_field_quoted(&s, "killer:") };
                    let killed = hl_field_quoted(&s, "killed:").unwrap_or_default();
                    let kteam = hl_field_i64(&s, "killer_team:").unwrap_or(-1);
                    kills.push((pos, killer, killed, kteam));
                } else if s.contains("GamePlayerState") {
                    if let (Some(ch), Some(nm)) = (hl_field_quoted(&s, "champion:"), hl_field_quoted(&s, " name:")) {
                        if let Some(ep) = s.find("entity_id:") {
                            if let Some(eid) = hl_field_i64(&s[ep..], "Some(") {
                                id2name.insert(eid, nm.clone());
                                if let Some(tm) = hl_field_i64(&s, "team:") { id2team.insert(eid, tm); }
                            }
                        }
                        champ2name.entry(ch).or_insert(nm);
                    }
                }
            }
            pos += 1;
        }
    }
    SCAN_POS.store(pos, Ordering::Relaxed);
    if pos >= total {
        let mid = hl_identify_match(&*db); // 스캔 완료 시점에 관전 매치 식별
        hl_finalize(total, mid);
        SCAN_DONE.store(true, Ordering::Relaxed);
    }
}

// 지금 관전 중인 매치 식별: 관전 scene(ClientScene::InGame)에서 매치 id 직독.
// ★0.5.0 오프셋(2026-07-08 재도출: SDK PDB 타입정보 + exe 디스어셈 2중 확증, 라이브/다시보기 공통):
//   scene 태그(u32) @db+0x1328 == 9 = InGame / MatchType 태그(u64) @db+0x1808 (Normal=1, Practice=2,
//   Tutorial=0, SoloRank=3) / 매치 id @db+0x1810 / match_info.id @db+0x17E8 (동일값 = 교차검증).
//   scene 내부 상대: match_type +0x4E0 / id +0x4E8 / match_info +0x440 (MatchInfo.id +0x80).
//   ⚠ ClientScene 은 niche 최적화 enum → discriminant = 선언순index+3 (InGame 은 6번째지만 tag=9).
//     tag 는 반드시 u32 로 읽을 것(상위 4B는 padding). 0.4.14/0.5.0 모두 tag=9 (exe 히스토그램 확증).
//   구 0.4.14 값: 0xF90 / 0x1510 / 0x1518 / 0x14F0.
// 구 방식(Running 매치 로스터 대조)은 좀비 Running 연습경기·이름중복 오귀속 이슈로 폐기.
fn hl_identify_match(db: &ClientDatabase) -> Option<usize> {
    unsafe {
        let base = (db as *const ClientDatabase) as usize;
        let scene_tag = *((base + 0x1338) as *const u32);   // 0.5.0_3(구0x1328, +0x10)
        if scene_tag != 9 { return None; } // 관전(InGame) scene 아님
        let mt_tag = *((base + 0x1818) as *const u64);       // 0.5.0_3(구0x1808, +0x10)
        if mt_tag != 1 { return None; } // Normal 매치만 (연습/튜토리얼/솔로랭크 제외)
        let mid = *((base + 0x1820) as *const usize);        // 0.5.0_3(구0x1810, +0x10)
        let mid2 = *((base + 0x17F8) as *const usize);       // 0.5.0_3(구0x17E8, +0x10)
        if mid == mid2 && mid < 1_000_000 { Some(mid) } else { None } // 교차검증 실패 시 이름 폴백에 맡김
    }
}

// 스캔 완료 시: 수집 버퍼 → 하이라이트 라인 → HL_BLOCKS 에 시그니처 dedup 으로 push.
fn hl_finalize(total: usize, match_id: Option<usize>) {
    let anchors = HL_ANCHORS.lock().unwrap_or_else(|e| e.into_inner());
    let kills = HL_KILLS.lock().unwrap_or_else(|e| e.into_inner());
    let ults = HL_ULTS.lock().unwrap_or_else(|e| e.into_inner());
    let skills = HL_SKILLS.lock().unwrap_or_else(|e| e.into_inner());
    let hits = HL_HITS.lock().unwrap_or_else(|e| e.into_inner());
    let effs = HL_EFF.lock().unwrap_or_else(|e| e.into_inner());
    let goldline = HL_GOLDLINE.lock().unwrap_or_else(|e| e.into_inner());
    let nexus_die = *HL_NEXUS_DIE.lock().unwrap_or_else(|e| e.into_inner());
    let idg = HL_ID2NAME.lock().unwrap_or_else(|e| e.into_inner());
    let itg = HL_ID2TEAM.lock().unwrap_or_else(|e| e.into_inner());
    let cng = HL_CHAMP2NAME.lock().unwrap_or_else(|e| e.into_inner());

    let name_of = |id: &i64| -> String {
        idg.as_ref().and_then(|m| m.get(id)).cloned().unwrap_or_else(|| format!("#{}", id))
    };
    let team_of = |id: &i64| -> i64 {
        itg.as_ref().and_then(|m| m.get(id)).copied().unwrap_or(-1)
    };
    let player_of_champ = |ck: &str| -> String {
        cng.as_ref().and_then(|m| m.get(ck)).cloned().unwrap_or_else(|| translate_champion_name(ck).to_string())
    };

    let mut ev: Vec<(i64, String)> = Vec::new();

    // 1) 연속킬 — 같은 선수(챔프)의 킬 간격이 10초 이내로 이어지면 멀티킬
    {
        let mut per_killer: HashMap<String, Vec<i64>> = HashMap::new();
        for (f, killer, _killed, _kt) in kills.iter() {
            if let Some(k) = killer {
                per_killer.entry(k.clone()).or_default().push(hl_interp(*f, &anchors));
            }
        }
        for (ck, mut ticks) in per_killer {
            ticks.sort();
            let mut start = 0usize;
            for i in 1..=ticks.len() {
                let brk = i == ticks.len() || ticks[i] - ticks[i - 1] > HL_MULTIKILL_WINDOW_TICKS;
                if brk {
                    let n = i - start;
                    if n >= 2 {
                        let label = match n {
                            2 => "더블킬".to_string(),
                            3 => "트리플킬".to_string(),
                            4 => "쿼드라킬".to_string(),
                            5 => "펜타킬".to_string(),
                            m => format!("{}연속킬", m),
                        };
                        ev.push((ticks[start], format!("{}({}) {}!", player_of_champ(&ck), translate_champion_name(&ck), label)));
                    }
                    start = i;
                }
            }
        }
    }

    // 2) n인 궁 / 광역 스킬 — 시전자 귀속(EffectApplyed caster→target)으로 실제 적중만 카운트.
    //    귀속 정보가 없는 순수 피해형은 창 내에 아군의 다른 시전이 없을 때만 광역 피해로 인정
    //    (한타 중 팀원 스킬 적중을 궁 공적으로 오귀속하는 것 방지). 10초 내 연쇄는 한 줄 병합.
    {
        let mut casts: Vec<(usize, i64, bool)> = ults.iter().map(|&(f, c)| (f, c, true))
            .chain(skills.iter().map(|&(f, c)| (f, c, false)))
            .collect();
        casts.sort_by_key(|c| c.0);
        let mut last_cast: HashMap<(i64, bool), usize> = HashMap::new();
        let mut hl: Vec<(i64, String, usize, bool)> = Vec::new(); // (tick, 선수명, 적중수, is_ult)
        for &(f, cid, is_ult) in casts.iter() {
            if let Some(&lf) = last_cast.get(&(cid, is_ult)) {
                if f.saturating_sub(lf) < 120 { continue; } // 같은 시전자 연속 이벤트 dedup
            }
            last_cast.insert((cid, is_ult), f);
            let tc = team_of(&cid);
            if tc < 0 { continue; }
            // (a) 귀속 적중: 이 시전자의 효과가 적용된 서로 다른 적 선수 수
            let lo_e = effs.partition_point(|(ef, _, _)| *ef < f);
            let mut hit_ids: Vec<i64> = Vec::new();
            for (ef, ec, et) in effs[lo_e..].iter() {
                if *ef > f + HL_ULT_HIT_WINDOW { break; }
                if *ec == cid {
                    let ht = team_of(et);
                    if ht >= 0 && ht != tc && !hit_ids.contains(et) { hit_ids.push(*et); }
                }
            }
            let mut n = hit_ids.len();
            // (b) 단독 시전이면(창 내 아군 다른 시전 없음) 광역 피해 카운트도 인정 — 순수 피해형 궁 커버
            let exclusive = !casts.iter().any(|&(of, oc, _)| oc != cid && team_of(&oc) == tc
                && of + HL_ULT_HIT_WINDOW >= f && of <= f + HL_ULT_HIT_WINDOW);
            if exclusive {
                let lo = hits.partition_point(|(hf, _)| *hf < f);
                let mut amb: Vec<i64> = Vec::new();
                for (hf, hid) in hits[lo..].iter() {
                    if *hf > f + HL_ULT_HIT_WINDOW { break; }
                    if *hid != cid {
                        let ht = team_of(hid);
                        if ht >= 0 && ht != tc && !amb.contains(hid) { amb.push(*hid); }
                    }
                }
                if amb.len() > n { n = amb.len(); }
            }
            let min_hits = if is_ult { HL_ULT_MIN_HITS } else { HL_SKILL_MIN_HITS };
            if n >= min_hits { hl.push((hl_interp(f, &anchors), name_of(&cid), n, is_ult)); }
        }
        hl.sort_by_key(|e| e.0);
        let mut i = 0;
        while i < hl.len() {
            let mut j = i + 1;
            while j < hl.len() && hl[j].0 - hl[j - 1].0 <= 600 { j += 1; }
            if j - i == 1 {
                let (t, nm, k, iu) = &hl[i];
                let what = if *iu { "궁극기" } else { "광역 스킬" };
                ev.push((*t, format!("{} {} — 적 {}명 적중!", nm, what, k)));
            } else {
                let parts: Vec<String> = hl[i..j].iter()
                    .map(|(_, nm, k, iu)| format!("{} {}({}명)", nm, if *iu { "궁" } else { "스킬" }, k)).collect();
                ev.push((hl[i].0, format!("대규모 한타! 궁·스킬 연쇄 {}회 — {}", j - i, parts.join("·"))));
            }
            i = j;
        }
    }

    // 3) 포탑/처형 데스 — KillEvent.killer == None (챔피언이 아닌 처치 주체 = 포탑 등)
    for (f, killer, killed, _kt) in kills.iter() {
        if killer.is_none() && !killed.is_empty() {
            ev.push((hl_interp(*f, &anchors), format!("{}({}) 포탑에 맞아 사망!", player_of_champ(killed), translate_champion_name(killed))));
        }
    }

    // 4) 골드 역전 — 승리팀이 한때 HL_GOLD_COMEBACK_GAP 이상 뒤졌으면 역전.
    //    승자 판정: ①넥서스 파괴 ②최종 킬스코어 우세. 못 정하면 오탐 방지 위해 출력 안 함.
    let last_score = *HL_LASTSCORE.lock().unwrap_or_else(|e| e.into_inner());
    if goldline.len() >= 2 {
        let winner: Option<i64> = match nexus_die {
            Some(loser) => Some(1 - loser),
            None => {
                if last_score.0 > last_score.1 { Some(0) }
                else if last_score.1 > last_score.0 { Some(1) }
                else { None }
            }
        };
        if let Some(winner) = winner {
            let sgn: i64 = if winner == 0 { 1 } else { -1 };
            let (mut min_t, mut min_v) = (0i64, 0i64);
            for &(t, d) in goldline.iter() {
                let v = d * sgn;
                if v < min_v { min_v = v; min_t = t; }
            }
            if min_v <= -HL_GOLD_COMEBACK_GAP {
                let cross = goldline.iter().find(|&&(t, d)| t >= min_t && d * sgn >= 0).map(|&(t, _)| t)
                    .unwrap_or_else(|| goldline.last().map(|&(t, _)| t).unwrap_or(min_t));
                let team_s = if winner == 0 { "블루" } else { "레드" };
                ev.push((cross, format!("{}팀, 최대 {}G 골드 열세를 뒤집고 역전승! (최대 열세 시점 {})", team_s, -min_v, hl_fmt_tick(min_t))));
            }
        }
    }

    if LOG_ENABLED {
        let gl_head: Vec<String> = goldline.iter().take(5).map(|(t, d)| format!("({},{})", t, d)).collect();
        let gl_tail: Vec<String> = goldline.iter().rev().take(5).map(|(t, d)| format!("({},{})", t, d)).collect();
        let dbg = format!(
            "total={} anchors={} last_tick={:?} kills={} ults={} skills={} hits={} effs={} goldline={} nexus={:?} nexus_die={:?} last_score={:?} match_id={:?} players={}\n\
             goldline head={:?} tail(rev)={:?}\nlines({}):\n{}\n",
            total, anchors.len(), anchors.last(), kills.len(), ults.len(), skills.len(), hits.len(), effs.len(), goldline.len(),
            HL_NEXUS.lock().unwrap_or_else(|e| e.into_inner()).clone(), nexus_die, last_score, match_id,
            idg.as_ref().map(|m| m.len()).unwrap_or(0),
            gl_head, gl_tail, ev.len(),
            ev.iter().map(|(t, l)| format!("[{}] {}", hl_fmt_tick(*t), l)).collect::<Vec<_>>().join("\n")
        );
        let _ = fs::write(Path::new("mods").join(MOD_ID).join("hl_debug.txt"), dbg);
    }

    if ev.is_empty() { return; }
    ev.sort_by_key(|e| e.0);
    let lines: Vec<String> = ev.iter().map(|(t, l)| format!("[{}] {}", hl_fmt_tick(*t), l)).collect();
    let last_tick = anchors.last().map(|a| a.1).unwrap_or(0) as u64;
    let sig = (total as u64) ^ ((kills.len() as u64) << 24) ^ (last_tick << 40);
    // 고유 이름만 저장 (리스폰 중복 제거 — 로스터 겹침 카운트 왜곡 방지)
    let mut names: Vec<String> = idg.as_ref().map(|m| m.values().cloned().collect()).unwrap_or_default();
    names.sort();
    names.dedup();
    let mut blocks = HL_BLOCKS.lock().unwrap_or_else(|e| e.into_inner());
    if blocks.iter().any(|b| b.sig == sig) { return; } // 같은 세트 재관전 dedup
    blocks.push(HlBlock { sig, match_id, names, score: last_score, lines });
    while blocks.len() > HL_MAX_BLOCKS { blocks.remove(0); }
}
// ═══════════════ 관전 하이라이트 수집 끝 ═══════════════

struct ChampTierData {
    id: String,
    bans: usize,
    matches: usize,
    wins: usize,
    win_rate: f32,
    presence: f32,
    tier: String,
}

struct ReactionExtension {
    cooldown: Mutex<f32>,
    last_opened_series_id: Mutex<usize>,
}

impl Default for ReactionExtension {
    fn default() -> Self {
        Self {
            cooldown: Mutex::new(0.0),
            last_opened_series_id: Mutex::new(0),
        }
    }
}

// 갤러리 HTML 을 게임 프로세스 트리 밖(explorer 핸드오프)에서 오픈.
// ⚠ 게임 자식으로 브라우저를 띄우면 스팀이 프로세스트리(Job Object)로 실행상태를 추적해
//   게임 종료 후 "중지중" 무한대기 → explorer 에 넘기고 DETACHED/BREAKAWAY 로 트리 분리.
#[cfg(target_os = "windows")]
fn open_gallery() {
    use std::os::windows::process::CommandExt;
    const DETACHED_PROCESS: u32 = 0x0000_0008;
    const CREATE_NO_WINDOW: u32 = 0x0800_0000;
    const CREATE_BREAKAWAY_FROM_JOB: u32 = 0x0100_0000;
    let html_abs = std::env::current_dir()
        .map(|d| d.join("mods").join(MOD_ID).join("TFA2_gallery.html"))
        .unwrap_or_else(|_| Path::new("mods").join(MOD_ID).join("TFA2_gallery.html"));
    if !html_abs.exists() { return; } // 파일 없으면(설치 이상) 조용히 스킵
    let spawn_ok = std::process::Command::new("explorer.exe")
        .arg(&html_abs)
        .creation_flags(DETACHED_PROCESS | CREATE_NO_WINDOW | CREATE_BREAKAWAY_FROM_JOB)
        .spawn()
        .is_ok();
    if !spawn_ok {
        let _ = std::process::Command::new("explorer.exe")
            .arg(&html_abs)
            .creation_flags(DETACHED_PROCESS | CREATE_NO_WINDOW)
            .spawn();
    }
}
#[cfg(not(target_os = "windows"))]
fn open_gallery() {}

impl ModExtension for ReactionExtension {
    fn post_update(&self, scene: &mut Scene, ui: &mut GameUI, _assets: &mut Assets, dt: f32) {
        let Scene::InGame { data } = scene else {
            // InGame 씬 아님 → 세이브 미로드로 간주, 다음 로드 때 다시 열도록 리셋
            GALLERY_OPENED.store(false, Ordering::Relaxed);
            return;
        };

        // ⚠ 타이틀/메인메뉴도 Scene::InGame 이라 씬만으론 "세이브 로드"를 구분 못 함.
        //   실제 로드 판정 = 유효한 플레이어 팀이 db 에 존재(세이브 미로드면 db 가 비어 None).
        //   로드 상태의 첫 프레임에 갤러리 1회 오픈 → 이후 HTML 이 5초 폴링으로 스스로 갱신.
        //   세이브 미로드 상태로 돌아가면 리셋되어 다음 세이브 로드 때 다시 열림.
        let save_loaded = data.db().teams.get(&data.player_team_id()).is_some();
        if save_loaded {
            if !GALLERY_OPENED.swap(true, Ordering::Relaxed) {
                open_gallery();
            }
        } else {
            GALLERY_OPENED.store(false, Ordering::Relaxed);
        }

        // 관전 중이면 프레임 이벤트를 점진 스캔해 하이라이트 수집 (시간예산 1.2ms)
        hl_collect(data, &ui.root);

        if let Ok(mut cooldown) = self.cooldown.lock() {
            *cooldown += dt;
            if *cooldown >= 3.0 {
                let _ = export_match_reactions(data, &self.last_opened_series_id);
                *cooldown = 0.0;
            }
        }
    }
}

fn translate_champion_name(id: &str) -> &str {
    match id {
        "fighter" => "격투가",
        "knight" => "기사",
        "swordman" => "검사",
        "archer" => "궁수",
        "soldier" => "소총수",
        "priest" => "성직자",
        "pythoness" => "무녀",
        "monk" => "몽크",
        "pyromancer" => "화염술사",
        "ice_mage" => "얼음술사",
        "ninja" => "닌자",
        "magic_knight" => "마검사",
        "berserker" => "광전사",
        "executioner" => "처형인",
        "lancer" => "창술사",
        "ogre" => "오우거",
        "dual_blader" => "듀얼블레이더",
        "cavalry_knight" => "기병",
        "gunner" => "총잡이",
        "pole_warrior" => "봉술사",
        "jiangshi" => "강시",
        "gambler" => "도박사",
        "hammerer" => "중보병",
        "demon" => "악마",
        "vampire" => "흡혈귀",
        "spirit_caller" => "정령사",
        "boomerang_hunter" => "부메랑헌터",
        "inquisitor" => "이단심문관",
        "shield_bearer" => "방패병",
        "whip_master" => "채찍술사",
        "werewolf" => "늑대인간",
        "dokkaebi" => "도깨비",
        "necromancer" => "네크로맨서",
        "bard" => "음유시인",
        "barrier_magician" => "결계술사",
        "chef" => "요리사",
        "clown" => "광대",
        "dancer" => "무희",
        "dark_mage" => "흑마술사",
        "exorcist" => "엑소시스트",
        "ghost" => "유령",
        "illusionist" => "환영술사",
        "lightning_mage" => "번개술사",
        "plague_doctor" => "역병의사",
        "poison_dart_hunter" => "독침술사",
        "shadowmancer" => "그림자술사",
        "taoist" => "도사",
        "siege_breaker" => "공성병",
        "android" => "안드로이드",
        "druid" => "드루이드",
        "prisoner" => "죄수",
        "bomber" => "폭탄병",
        "voodoo_shaman" => "부두술사",
        "white_mage" => "백마술사",
        "wind_mage" => "바람술사",
        "enchanter" => "인챈터",
        "hitman" => "히트맨",
        "guardian_spirit" => "수호령",
        "hunter" => "사냥꾼",
        "circus_blade" => "곡예사",
        _ => id,
    }
}

fn get_position_index(pos: &Position) -> usize {
    match pos {
        Position::Top => 0,
        Position::Jungle => 1,
        Position::Mid => 2,
        Position::Bottom => 3,
        Position::Support => 4,
    }
}

fn get_weighted_quantile(sorted_champs: &[ChampTierData], q: f32, total_weight: f32) -> f32 {
    if sorted_champs.is_empty() { return 0.50; }
    let target = q * total_weight;
    let mut cum_weight = 0.0;
    for c in sorted_champs {
        cum_weight += c.presence;
        if cum_weight >= target {
            return c.win_rate;
        }
    }
    sorted_champs.last().map(|c| c.win_rate).unwrap_or(0.50)
}

fn translate_expectation(exp: &str) -> &'static str {
    if exp.contains("Lower") || exp.contains("Low") {
        "낮음"
    } else if exp.contains("Higher") || exp.contains("High") {
        "높음"
    } else {
        "보통"
    }
}

fn translate_satisfaction(sat: &str) -> &'static str {
    if sat.contains("VerySatisfied") {
        "매우 만족"
    } else if sat.contains("Satisfied") {
        "만족"
    } else if sat.contains("VeryUnsatisfied") {
        "매우 불만족"
    } else if sat.contains("Unsatisfied") {
        "불만족"
    } else {
        "보통"
    }
}

fn translate_num_rating(val: usize) -> &'static str {
    match val {
        0..=20 => "매우 낮음(불만)",
        21..=40 => "낮음(불만족)",
        41..=60 => "보통",
        61..=80 => "높음(만족)",
        _ => "매우 높음(만족)",
    }
}

fn translate_item_name_to_stat(id: usize) -> String {
    let category_idx = id / 5;
    let tier_num = (id % 5) + 1;
    
    let stat_name = match category_idx {
        0 => "공격력",
        1 => "공속",
        2 => "방어력",
        3 => "마저",
        4 => "주문력",
        5 => "체력",
        _ => "기타",
    };
    format!("{}[T{}]", stat_name, tier_num)
}

fn translate_league_name(name: &str) -> &str {
    match name {
        "TACK" => "한국 1부 (TACK)",
        "TACK2" => "한국 2부 (TACK2)",
        "TACJ" => "일본 1부 (TACJ)",
        "TACJ2" => "일본 2부 (TACJ2)",
        "TACC" => "중국 1부 (TACC)",
        "TACC2" => "중국 2부 (TACC2)",
        "TACE" => "유럽 1부 (TACE)",
        "TACE2" => "유럽 2부 (TACE2)",
        "TACA" => "북미 1부 (TACA)",
        "TACA2" => "북미 2부 (TACA2)",
        "TACS" => "남미 1부 (TACS)",
        "TACS2" => "남미 2부 (TACS2)",
        _ => name,
    }
}

fn get_weekly_salary_safe(athlete: &Athlete) -> f64 {
    let contract_str = format!("{:?}", athlete.contract);
    if let Some(pos) = contract_str.find("weekly_salary:") {
        let sub = &contract_str[pos + "weekly_salary:".len()..];
        let val_str: String = sub.chars()
            .skip_while(|c| c.is_whitespace())
            .take_while(|c| c.is_numeric() || *c == '.')
            .collect();
        val_str.parse::<f64>().unwrap_or(0.0)
    } else {
        0.0
    }
}

fn build_single_match_summary(db: &ClientDatabase, replay: &MatchReplayData, is_main: bool, set_num: usize, attention: usize) -> String {
    let mut text = String::new();

    let blue_team = db.teams.get(&replay.blue_team_id);
    let red_team = db.teams.get(&replay.red_team_id);

    let blue_name = blue_team.map(|t| t.name.as_str()).unwrap_or("Unknown Team");
    let red_name = red_team.map(|t| t.name.as_str()).unwrap_or("Unknown Team");

    if is_main {
        text.push_str("==================================================\n");
        text.push_str(&format!("메인 매치 [{}세트] (우리 팀 경기 분석) [주목도: {}/20]\n", set_num, attention));
        text.push_str("==================================================\n");
    } else {
        text.push_str("==================================================\n");
        text.push_str(&format!("동일 라운드 다른 경기 [{}세트] 분석 [주목도: {}/20]\n", set_num, attention));
        text.push_str("==================================================\n");
    }

    text.push_str(&format!("대진: {} vs {}\n\n", blue_name, red_name));

    if let Some(bt) = blue_team {
        text.push_str(&format!("[블루 팀: {} 정보]\n", bt.name));
        text.push_str(&format!(" - 감독: {}\n", bt.manager_name));
        text.push_str(&format!(" - 팬 기대치: {}\n", translate_expectation(&format!("{:?}", bt.fan_expectation))));
        text.push_str(&format!(" - 팬 만족도: {}\n", translate_satisfaction(&format!("{:?}", bt.fan_satisfaction))));
        text.push_str(&format!(" - 팬 규모: {}명\n\n", bt.fan_count));
    }

    if let Some(rt) = red_team {
        text.push_str(&format!("[레드 팀: {} 정보]\n", rt.name));
        text.push_str(&format!(" - 감독: {}\n", rt.manager_name));
        text.push_str(&format!(" - 팬 기대치: {}\n", translate_expectation(&format!("{:?}", rt.fan_expectation))));
        text.push_str(&format!(" - 팬 만족도: {}\n", translate_satisfaction(&format!("{:?}", rt.fan_satisfaction))));
        text.push_str(&format!(" - 팬 규모: {}명\n\n", rt.fan_count));
    }

    text.push_str(&format!("* [{}] 선수단 프로필\n", blue_name));
    for p in &replay.blue_team {
        if let Some(athlete) = db.athletes.get(&p.athlete_id) {
            let weekly_val = get_weekly_salary_safe(athlete);
            let annual_salary_won = (weekly_val * 52.0) as u64; 
            let salary_format = if annual_salary_won >= 100_000_000 {
                format!("{:.2}억원", (annual_salary_won as f64) / 100_000_000.0)
            } else {
                format!("{}만원", annual_salary_won / 10_000)
            };

            text.push_str(&format!(
                " - {} (ID: {}) | 팬 만족도: {}, 기대치: {}, 연봉: {} | 팬 수: {}명\n",
                athlete.name,
                athlete.id,
                translate_num_rating(athlete.management.fan_satisfaction),
                translate_num_rating(athlete.management.fan_expectation),
                salary_format,
                athlete.management.fan_count
            ));
        }
    }
    text.push_str("\n");

    text.push_str(&format!("* [{}] 선수단 프로필\n", red_name));
    for p in &replay.red_team {
        if let Some(athlete) = db.athletes.get(&p.athlete_id) {
            let weekly_val = get_weekly_salary_safe(athlete);
            let annual_salary_won = (weekly_val * 52.0) as u64;
            let salary_format = if annual_salary_won >= 100_000_000 {
                format!("{:.2}억원", (annual_salary_won as f64) / 100_000_000.0)
            } else {
                format!("{}만원", annual_salary_won / 10_000)
            };

            text.push_str(&format!(
                " - {} (ID: {}) | 팬 만족도: {}, 기대치: {}, 연봉: {} | 팬 수: {}명\n",
                athlete.name,
                athlete.id,
                translate_num_rating(athlete.management.fan_satisfaction),
                translate_num_rating(athlete.management.fan_expectation),
                salary_format,
                athlete.management.fan_count
            ));
        }
    }
    text.push_str("\n");

    let winner_name = if replay.blue_team_win { blue_name } else { red_name };
    text.push_str("--- 세부 성적 및 밴픽 ---\n");
    text.push_str(&format!("승자(Winner): {}\n", winner_name));

    let blue_bans_kor: Vec<String> = replay.blue_ban.iter()
        .map(|b| translate_champion_name(b).to_string())
        .collect();
    let red_bans_kor: Vec<String> = replay.red_ban.iter()
        .map(|r| translate_champion_name(r).to_string())
        .collect();

    text.push_str(&format!("블루 밴카드: [{}]\n", blue_bans_kor.join(", ")));
    text.push_str(&format!("레드 밴카드: [{}]\n\n", red_bans_kor.join(", ")));

    let blue_gold = replay.blue_performance.total_gold;
    let red_gold = replay.red_performance.total_gold;
    let gold_diff = (blue_gold as i32 - red_gold as i32).abs();
    let gold_leader = if blue_gold >= red_gold { blue_name } else { red_name };

    text.push_str("--- 팀 종합 지표 및 오브젝트 ---\n");
    text.push_str(&format!("골드 현황: 블루 {}G vs 레드 {}G ({} 골드 리드: {}G 차이)\n", blue_gold, red_gold, gold_leader, gold_diff));

    let first_epic_str = if replay.blue_performance.first_epic { "선취" } else { "-" };
    let first_serpen_str = if replay.blue_performance.first_serpen { "선취" } else { "-" };
    
    text.push_str(&format!(
        "블루팀 오브젝트: 모르가드 {}회 ({}), 세르펜 {}회 ({})\n",
        replay.blue_performance.epic_secured,
        first_epic_str,
        replay.blue_performance.serpen_secured,
        first_serpen_str
    ));

    let first_epic_red = if replay.red_performance.first_epic { "선취" } else { "-" };
    let first_serpen_red = if replay.red_performance.first_serpen { "선취" } else { "-" };
    text.push_str(&format!(
        "레드팀 오브젝트: 모르가드 {}회 ({}), 세르펜 {}회 ({})\n\n",
        replay.red_performance.epic_secured,
        first_epic_red,
        replay.red_performance.serpen_secured,
        first_serpen_red
    ));

    let mut best_score = -999.0;
    let mut pog_champion = "Unknown";
    let mut pog_name = "Unknown";
    let mut pog_kda = String::new();
    let mut pog_deal = 0;

    text.push_str(&format!("<{}> 선수별 인게임 데이터\n", blue_name));
    for p in &replay.blue_team {
        let idx = get_position_index(&p.position);
        let kills = replay.blue_performance.kills.get(idx).cloned().unwrap_or(0);
        let deaths = replay.blue_performance.deaths.get(idx).cloned().unwrap_or(0);
        let assists = p.statistics.assists; 
        let deal = replay.blue_performance.deal.get(idx).cloned().unwrap_or(0);
        let gold = p.statistics.gold;
        let tanking = p.statistics.tanking;
        let healing = p.statistics.healing;
        let pos_str = format!("{:?}", p.position);
        
        let item_names: Vec<String> = p.items.iter()
            .map(|&id| translate_item_name_to_stat(id))
            .collect();
        let items_str = item_names.join(", ");

        let kor_champion = translate_champion_name(&p.champion);
        let name = db.athletes.get(&p.athlete_id).map(|a| a.name.as_str()).unwrap_or("Unknown");

        let deaths_float = if deaths == 0 { 0.7 } else { deaths as f32 };
        let current_score = (kills as f32 + assists as f32) / deaths_float;

        if current_score > best_score {
            best_score = current_score;
            pog_champion = kor_champion;
            pog_name = name;
            pog_kda = format!("{}/{}/{}", kills, deaths, assists);
            pog_deal = deal;
        }

        text.push_str(&format!(
            " - [{}] {} ({}) | K/D/A: {}/{}/{} | 골드: {}G | 딜량: {} | 받은피해: {} | 힐량: {} | 장착 템: [{}]\n",
            pos_str, name, kor_champion, kills, deaths, assists, gold, deal, tanking, healing, items_str
        ));
    }
    text.push_str("\n");

    text.push_str(&format!("<{}> 선수별 인게임 데이터\n", red_name));
    for p in &replay.red_team {
        let idx = get_position_index(&p.position);
        let kills = replay.red_performance.kills.get(idx).cloned().unwrap_or(0);
        let deaths = replay.red_performance.deaths.get(idx).cloned().unwrap_or(0);
        let assists = p.statistics.assists; 
        let deal = replay.red_performance.deal.get(idx).cloned().unwrap_or(0);
        let gold = p.statistics.gold;
        let tanking = p.statistics.tanking;
        let healing = p.statistics.healing;
        let pos_str = format!("{:?}", p.position);
        
        let item_names: Vec<String> = p.items.iter()
            .map(|&id| translate_item_name_to_stat(id))
            .collect();
        let items_str = item_names.join(", ");

        let kor_champion = translate_champion_name(&p.champion);
        let name = db.athletes.get(&p.athlete_id).map(|a| a.name.as_str()).unwrap_or("Unknown");

        let deaths_float = if deaths == 0 { 0.7 } else { deaths as f32 };
        let current_score = (kills as f32 + assists as f32) / deaths_float;

        if current_score > best_score {
            best_score = current_score;
            pog_champion = kor_champion;
            pog_name = name;
            pog_kda = format!("{}/{}/{}", kills, deaths, assists);
            pog_deal = deal;
        }

        text.push_str(&format!(
            " - [{}] {} ({}) | K/D/A: {}/{}/{} | 골드: {}G | 딜량: {} | 받은피해: {} | 힐량: {} | 장착 템: [{}]\n",
            pos_str, name, kor_champion, kills, deaths, assists, gold, deal, tanking, healing, items_str
        ));
    }
    text.push_str("\n");

    text.push_str("--- 오늘의 세트 POG (MVP) ---\n");
    text.push_str(&format!("선정 선수: {} ({}) | 세트 K/D/A: {} (평점: {:.2}) | 딜량: {}\n\n", pog_name, pog_champion, pog_kda, best_score, pog_deal));

    text
}

fn escape_json_string(s: &str) -> String {
    s.replace('\\', "\\\\")
         .replace('"', "\\\"")
         .replace('\n', "\\n")
         .replace('\r', "")
}

fn extract_team_id(team: &impl std::fmt::Debug) -> Option<usize> {
    let s = format!("{:?}", team);
    if s.starts_with("Normal(") {
        let id_str: String = s.chars()
            .skip_while(|c| !c.is_numeric())
            .take_while(|c| c.is_numeric())
            .collect();
        id_str.parse::<usize>().ok()
    } else {
        None
    }
}

fn export_match_reactions(data: &ClientData, last_opened_series_id: &Mutex<usize>) -> std::io::Result<()> {
    let db = data.db();
    let player_team_id = data.player_team_id();

    let Some(our_latest_replay) = db.match_replays.values()
        .filter(|r| r.blue_team_id == player_team_id || r.red_team_id == player_team_id)
        .max_by_key(|r| r.id) else {
            return Ok(());
        };

    let Some(our_team) = db.teams.get(&player_team_id) else { return Ok(()); };
    let our_league_id = our_team.league_id;

    let opponent_team_id = if our_latest_replay.blue_team_id == player_team_id {
        our_latest_replay.red_team_id
    } else {
        our_latest_replay.blue_team_id
    };

    let league_name = db.leagues.get(&our_league_id)
        .map(|l| translate_league_name(l.name.as_str()))
        .unwrap_or("알 수 없는 리그");

    let mut our_series: Vec<&MatchReplayData> = db.match_replays.values()
        .filter(|r| {
            (r.id as i32 - our_latest_replay.id as i32).abs() <= 5
                && ((r.blue_team_id == our_latest_replay.blue_team_id && r.red_team_id == our_latest_replay.red_team_id)
                    || (r.blue_team_id == our_latest_replay.red_team_id && r.red_team_id == our_latest_replay.blue_team_id))
        })
        .collect();
    our_series.sort_by_key(|r| r.id);

    let mut need_win = 2;
    let mut match_date = String::from("알 수 없음");
    let mut our_match_opt = None;
    let mut our_match_is_practice = false; // MatchType::Normal 이 아니면 연습/비공식

    for (mt, m) in db.matches.iter() {
        if m.replays.contains(&our_latest_replay.id) {
            need_win = m.need_win;
            match_date = format!("{}", m.date);
            our_match_opt = Some(m);
            our_match_is_practice = !matches!(mt, MatchType::Normal { .. });
            break;
        }
    }

    if our_match_opt.is_none() {
        for (mt, m) in db.matches.iter() {
            // ⚠Running 폴백은 정규 매치만 — 영구 Running 잔존 연습경기(스크림) 오탐 방지
            if !matches!(mt, MatchType::Normal { .. }) { continue; }
            let state_str = format!("{:?}", m.running_state);
            if state_str == "Running" {
                if let Some(t1_id) = extract_team_id(&m.team1) {
                    if t1_id == player_team_id {
                        need_win = m.need_win;
                        match_date = format!("{}", m.date);
                        our_match_opt = Some(m);
                        our_match_is_practice = !matches!(mt, MatchType::Normal { .. });
                        break;
                    }
                }
                if let Some(t2_id) = extract_team_id(&m.team2) {
                    if t2_id == player_team_id {
                        need_win = m.need_win;
                        match_date = format!("{}", m.date);
                        our_match_opt = Some(m);
                        our_match_is_practice = !matches!(mt, MatchType::Normal { .. });
                        break;
                    }
                }
            }
        }
    }

    let our_match_id = our_match_opt.map(|m| m.id);

    let mut our_blue_fans = db.teams.get(&our_latest_replay.blue_team_id).map(|t| t.fan_count).unwrap_or(0);
    for p in &our_latest_replay.blue_team {
        if let Some(athlete) = db.athletes.get(&p.athlete_id) {
            our_blue_fans += athlete.management.fan_count;
        }
    }
    let mut our_red_fans = db.teams.get(&our_latest_replay.red_team_id).map(|t| t.fan_count).unwrap_or(0);
    for p in &our_latest_replay.red_team {
        if let Some(athlete) = db.athletes.get(&p.athlete_id) {
            our_red_fans += athlete.management.fan_count;
        }
    }
    let our_match_total_fans = our_blue_fans + our_red_fans;

    let mut our_attention = 20;
    let mut other_attention = 0;
    let mut other_series: Vec<&MatchReplayData> = Vec::new();

    if let Some(our_match) = our_match_opt {
        let our_date_str = format!("{}", our_match.date);
        let our_day = if our_date_str.len() >= 10 { &our_date_str[0..10] } else { &our_date_str };

        let mut other_match_opt = None;
        for m in db.matches.values() {
            if m.id == our_match.id {
                continue;
            }

            let m_date_str = format!("{}", m.date);
            let m_day = if m_date_str.len() >= 10 { &m_date_str[0..10] } else { &m_date_str };

            if m_day == our_day {
                let mut is_same_league = false;
                if let Some(t1_id) = extract_team_id(&m.team1) {
                    if let Some(t) = db.teams.get(&t1_id) {
                        if t.league_id == our_league_id {
                            is_same_league = true;
                        }
                    }
                }
                if !is_same_league {
                    if let Some(t2_id) = extract_team_id(&m.team2) {
                        if let Some(t) = db.teams.get(&t2_id) {
                            if t.league_id == our_league_id {
                                is_same_league = true;
                            }
                        }
                    }
                }

                if is_same_league {
                    other_match_opt = Some(m);
                    break;
                }
            }
        }

        if let Some(other_match) = other_match_opt {
            for &replay_id in &other_match.replays {
                if let Some(rep) = db.match_replays.get(&replay_id) {
                    other_series.push(rep);
                }
            }
            other_series.sort_by_key(|r| r.id);

            if let Some(&other_latest) = other_series.last() {
                let mut other_blue_fans = db.teams.get(&other_latest.blue_team_id).map(|t| t.fan_count).unwrap_or(0);
                for p in &other_latest.blue_team {
                    if let Some(athlete) = db.athletes.get(&p.athlete_id) {
                        other_blue_fans += athlete.management.fan_count;
                    }
                }
                let mut other_red_fans = db.teams.get(&other_latest.red_team_id).map(|t| t.fan_count).unwrap_or(0);
                for p in &other_latest.red_team {
                    if let Some(athlete) = db.athletes.get(&p.athlete_id) {
                        other_red_fans += athlete.management.fan_count;
                    }
                }
                let other_match_total_fans = other_blue_fans + other_red_fans;
                let grand_total_fans = our_match_total_fans + other_match_total_fans;

                if grand_total_fans > 0 {
                    our_attention = (((our_match_total_fans as f32 / grand_total_fans as f32) * 20.0).ceil() as usize).clamp(1, 20);
                    other_attention = 20 - our_attention;
                }
            }
        }
    }

    let mut text = String::new();

    text.push_str("==================================================\n");
    text.push_str("챔피언 통계 및 티어\n");
    text.push_str("==================================================\n");
    
    if let Some((latest_patch_key, patch_stats)) = db.champion_patch_statistics.keys().max().and_then(|k| {
        db.champion_patch_statistics.get(k).map(|stats| (k, stats))
    }) {
        let n_champions = patch_stats.data.len();
        let presence_threshold = 0.40 * (30.0 / n_champions.max(1) as f32);
        let max_s = 5 + (n_champions.saturating_sub(30) / 15);
        let total_match_f = patch_stats.total_match.max(1) as f32;

        let mut champions_list = Vec::new();

        for (champ_id, champ_season_stats) in &patch_stats.data {
            let bans = champ_season_stats.bans;
            let mut matches = 0;
            let mut wins = 0;
            for pos_stat in champ_season_stats.by_position.values() {
                matches += pos_stat.matches;
                wins += pos_stat.wins;
            }
            
            let win_rate = if matches > 0 { wins as f32 / matches as f32 } else { 0.0 };
            let presence = (matches + bans) as f32 / total_match_f;

            champions_list.push(ChampTierData {
                id: champ_id.clone(),
                bans,
                matches,
                wins,
                win_rate,
                presence,
                tier: String::new(),
            });
        }

        let mut s_candidates = Vec::new();
        let mut others = Vec::new();

        for c in champions_list {
            if c.win_rate > 0.55 && c.presence >= presence_threshold {
                s_candidates.push(c);
            } else {
                others.push(c);
            }
        }

        s_candidates.sort_by(|a, b| b.win_rate.partial_cmp(&a.win_rate).unwrap_or(std::cmp::Ordering::Equal));
        let mut s_tier_list = Vec::new();
        for c in s_candidates {
            if s_tier_list.len() < max_s {
                s_tier_list.push(c);
            } else {
                others.push(c);
            }
        }
        for c in &mut s_tier_list {
            c.tier = "S".to_string();
        }

        let mut non_s = others;
        non_s.sort_by(|a, b| a.win_rate.partial_cmp(&b.win_rate).unwrap_or(std::cmp::Ordering::Equal));

        let total_weight: f32 = non_s.iter().map(|c| c.presence).sum();
        let q20 = get_weighted_quantile(&non_s, 0.20, total_weight);
        let q45 = get_weighted_quantile(&non_s, 0.45, total_weight);
        let q70 = get_weighted_quantile(&non_s, 0.70, total_weight);
        let q75 = get_weighted_quantile(&non_s, 0.75, total_weight);
        let q80 = get_weighted_quantile(&non_s, 0.80, total_weight);
        let q85 = get_weighted_quantile(&non_s, 0.85, total_weight);

        let ab_cutoff = q75.clamp(q70.max(q80 - 0.03), q85);
        let bc_cutoff = q45;
        let cd_cutoff = q20;

        for c in &mut non_s {
            if c.win_rate < cd_cutoff {
                c.tier = "D".to_string();
            } else if c.win_rate < bc_cutoff {
                c.tier = "C".to_string();
            } else if c.win_rate < ab_cutoff {
                c.tier = "B".to_string();
            } else {
                c.tier = "A".to_string();
            }
        }

        let mut all_champs = s_tier_list;
        all_champs.extend(non_s);
        all_champs.sort_by(|a, b| {
            let tier_val = |t: &str| match t {
                "S" => 0,
                "A" => 1,
                "B" => 2,
                "C" => 3,
                _ => 4,
            };
            tier_val(&a.tier).cmp(&tier_val(&b.tier))
        });

        text.push_str(&format!("적용 패치 버전: {}\n", latest_patch_key));
        text.push_str(&format!("총 매치 표본 수: {}경기\n\n", patch_stats.total_match));

        for c in all_champs {
            let kor_champ_name = translate_champion_name(&c.id);
            text.push_str(&format!(
                " - {}: [{}]티어 | 밴 {}회 | 픽 {}회 | 승리 {}회 (승률: {:.1}%, 등장률: {:.1}%)\n",
                kor_champ_name, c.tier, c.bans, c.matches, c.wins, c.win_rate * 100.0, c.presence * 100.0
            ));
        }
    } else {
        text.push_str("패치 통계 데이터를 발견하지 못했습니다.\n");
    }
    text.push_str("\n");

    text.push_str(&format!("리그 명칭: {}\n", league_name));
    text.push_str(&format!("시즌 패치버전: {}\n", our_latest_replay.version));
    text.push_str(&format!("경기 일정: {}\n\n", match_date));

    text.push_str("--- 역대 상대 전적 (Rivalry) ---\n");
    if let Some(&(wins, losses)) = db.head_to_head.get(&opponent_team_id) {
        let opponent_name = db.teams.get(&opponent_team_id).map(|t| t.name.as_str()).unwrap_or("상대 팀");
        text.push_str(&format!("vs {} 상대 전적: {}승 {}패\n\n", opponent_name, wins, losses));
    } else {
        text.push_str("상대 팀과의 공식 경기 기록이 아직 없습니다.\n\n");
    }

    // ── (원본 형식 유지) 기존 리그 순위 섹션 — 원작 모드 출력 그대로, 수정 금지 ──
    let mut our_competition_opt = None;
    if let Some(match_id) = our_match_id {
        our_competition_opt = db.league_competitions.values()
            .find(|comp| comp.matches.contains(&match_id) || comp.playoffs.contains(&match_id)
                || comp.promotion_series.contains(&match_id));
    }
    if our_competition_opt.is_none() {
        our_competition_opt = db.league_competitions.values()
            .filter(|comp| comp.standings.contains_key(&player_team_id))
            .max_by_key(|comp| comp.id);
    }

    text.push_str("--- 현재 리그 순위 (Standings) ---\n");
    if let Some(competition) = our_competition_opt {

        let mut match_stage = String::from("국제전 및 기타 토너먼트");
        if let Some(match_id) = our_match_id {
            if competition.matches.contains(&match_id) {
                match_stage = String::from("정규 리그");
            } else if competition.playoffs.contains(&match_id) {
                if let Some(pos) = competition.playoffs.iter().position(|&id| id == match_id) {
                    let round_str = match pos {
                        0 => "플레이오프 1경기",
                        1 => "플레이오프 2경기",
                        2 => "플레이오프 패자전",
                        3 => "플레이오프 패자전",
                        4 => "플레이오프 2라운드 승자조",
                        5 => "플레이오프 2라운드 패자조",
                        6 => "결승 진출전",
                        7 => "결승전",
                        _ => "플레이오프",
                    };
                    match_stage = round_str.to_string();
                } else {
                    match_stage = String::from("플레이오프");
                }
            } else if competition.promotion_series.contains(&match_id) {
                // 0.5.2 신규: 승강전(promotion_series). 상세 라운드/맥락은 아래 "경기 구분 상세" 섹션에서.
                match_stage = String::from("승강전");
            }
        }
        text.push_str(&format!("대회 형태: {}\n", match_stage));

        if let Some(our_standing) = competition.standings.get(&player_team_id) {
            let played = our_standing.win + our_standing.lose;
            let total_teams = competition.standings.len();
            let total_matches = if total_teams > 0 { (total_teams - 1) * 2 } else { 0 };
            text.push_str(&format!("이번 리그 현재 경기 수: {}/{} 매치\n\n", played, total_matches));
        }

        let mut sorted_standings: Vec<_> = competition.standings.iter()
            .map(|(&team_id, standing)| (team_id, standing))
            .collect();
        sorted_standings.sort_by(|a, b| {
            let a_points = (a.1.set_win as i32) - (a.1.set_lose as i32);
            let b_points = (b.1.set_win as i32) - (b.1.set_lose as i32);
            b.1.win.cmp(&a.1.win)
                .then_with(|| b_points.cmp(&a_points))
                .then_with(|| b.1.kill.cmp(&a.1.kill))
        });

        for (rank, &(team_id, standing)) in sorted_standings.iter().enumerate() {
            let team_name = db.teams.get(&team_id).map(|t| t.name.as_str()).unwrap_or("알 수 없는 팀");

            let set_diff = standing.set_win as i32 - standing.set_lose as i32;
            let set_diff_str = if set_diff > 0 {
                format!("+{}", set_diff)
            } else {
                set_diff.to_string()
            };

            text.push_str(&format!(
                " - {}위: {} ({}승 {}패 | 세트득실: {})\n",
                rank + 1,
                team_name,
                standing.win,
                standing.lose,
                set_diff_str
            ));
        }
    } else {
        text.push_str("현재 리그의 순위표 데이터를 찾을 수 없습니다.\n");
    }
    text.push_str("\n");

    // ── (신규 추가) 경기 구분 상세 — 원본 섹션 뒤에 덧붙이는 확장 정보 ──
    // 리그 대회 = db.league_competitions (matches=정규, playoffs=플옵)
    // 국제전   = db.tournament_competitions (group_matches=조별, tournament_matches=녹아웃)
    {
        fn push_standing_rows(text: &mut String, rows: &mut Vec<(usize, i64, i64, i64, i64)>, db: &ClientDatabase, player_team_id: usize) {
            rows.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| b.3.cmp(&a.3)).then_with(|| b.4.cmp(&a.4)));
            for (rank, (tid, w, l, sd, _k)) in rows.iter().enumerate() {
                let team_name = db.teams.get(tid).map(|t| t.name.as_str()).unwrap_or("알 수 없는 팀");
                let star = if *tid == player_team_id { " ◀ 우리 팀" } else { "" };
                let sds = if *sd > 0 { format!("+{}", sd) } else { sd.to_string() };
                text.push_str(&format!(" - {}위: {} ({}승 {}패 | 세트득실: {}){}\n", rank + 1, team_name, w, l, sds, star));
            }
        }
        // (LeagueStanding 타입이 mod_api 에 재노출 안 돼 인라인 매핑으로 소비)
        macro_rules! standing_rows_of {
            ($m:expr) => {
                $m.iter()
                    .map(|(&tid, st)| (tid, st.win as i64, st.lose as i64, st.set_win as i64 - st.set_lose as i64, st.kill as i64))
                    .collect::<Vec<(usize, i64, i64, i64, i64)>>()
            };
        }
        fn playoff_round_label(pos: usize, total: usize) -> String {
            if total == 8 {
                // 더블엘리 8매치 구조 (2027 TACJ 실측): M1/M2 승자조 1R → M3/M4 패자조 1R
                //   → M5 승자조 결승 → M6 패자조 2R → M7 패자조 결승 → M8 결승
                match pos {
                    0 | 1 => "플레이오프 승자조 1라운드",
                    2 | 3 => "플레이오프 패자조 1라운드",
                    4 => "플레이오프 승자조 결승",
                    5 => "플레이오프 패자조 2라운드",
                    6 => "플레이오프 패자조 결승 (결승 진출전)",
                    _ => "결승전",
                }.to_string()
            } else if total > 0 && pos + 1 == total {
                "결승전".to_string()
            } else {
                format!("플레이오프 {}번째 경기 (총 {}경기)", pos + 1, total.max(pos + 1))
            }
        }
        // 승강전(0.5.2 신규 promotion_series) 라운드 라벨.
        // ⚠ promotion_series 원소 순서의 의미(1차전/2차전인지 대진 순인지)는 미실증 →
        //    플옵처럼 라운드명을 단정하지 않고 위치만 중립 표기. 인게임 확인 후 확정할 것.
        fn promotion_round_label(pos: usize, total: usize) -> String {
            if total <= 1 {
                "단판 승강전".to_string()
            } else if pos + 1 == total {
                format!("승강전 최종 경기 (총 {}경기 중 마지막)", total)
            } else {
                format!("승강전 {}번째 경기 (총 {}경기)", pos + 1, total.max(pos + 1))
            }
        }
        fn tournament_kr(name: &str) -> String {
            match name {
                "Challengers E" => "챌린저스 이스턴".to_string(),
                "Challengers W" => "챌린저스 웨스턴".to_string(),
                "Masters" => "마스터스".to_string(),
                "Champions" => "챔피언스".to_string(),
                other => other.to_string(),
            }
        }

        // ── 플레이오프/녹아웃 대진표 출력기 ──
        // 한 줄 문법(HTML 파싱용 고정): [M{n}] {날짜} | BO{k} | {슬롯1} vs {슬롯2} | {상태}
        // 슬롯: 팀명 / "M{k} 승자(팀명|미정)" / "M{k} 패자(...)" / 시드[...] — WinnerOf/LoserOf 간선으로 브래킷 복원 가능
        fn nums_of(s: &str) -> Vec<i64> {
            let mut out = Vec::new();
            let mut cur = String::new();
            for c in s.chars() {
                if c.is_ascii_digit() { cur.push(c); }
                else if !cur.is_empty() { if let Ok(v) = cur.parse() { out.push(v); } cur.clear(); }
            }
            if !cur.is_empty() { if let Ok(v) = cur.parse() { out.push(v); } }
            out
        }
        let team_name_of = |tid: usize| -> String {
            db.teams.get(&tid).map(|t| t.name.clone()).unwrap_or_else(|| format!("팀{}", tid))
        };
        let match_by_id = |mid: usize| db.matches.values().find(|m| m.id == mid);
        let end_field = |mid: usize, key: &str| -> Option<usize> {
            let m = match_by_id(mid)?;
            let st = format!("{:?}", m.running_state);
            if !st.contains("End") { return None; }
            hl_field_i64(&st, key).map(|v| v as usize)
        };
        let slot_str = |team_dbg: &str, list: &Vec<usize>| -> String {
            let local = |mid: usize| list.iter().position(|&x| x == mid)
                .map(|k| format!("M{}", k + 1)).unwrap_or_else(|| format!("경기#{}", mid));
            if team_dbg.starts_with("Normal(") {
                nums_of(team_dbg).first().map(|&t| team_name_of(t as usize)).unwrap_or_else(|| team_dbg.to_string())
            } else if team_dbg.starts_with("WinnerOf(") {
                let mid = nums_of(team_dbg).first().copied().unwrap_or(-1) as usize;
                let who = end_field(mid, "winner:").map(|t| team_name_of(t)).unwrap_or_else(|| "미정".to_string());
                format!("{} 승자({})", local(mid), who)
            } else if team_dbg.starts_with("LoserOf(") {
                let mid = nums_of(team_dbg).first().copied().unwrap_or(-1) as usize;
                let who = end_field(mid, "loser:").map(|t| team_name_of(t)).unwrap_or_else(|| "미정".to_string());
                format!("{} 패자({})", local(mid), who)
            } else if team_dbg.starts_with("LeagueRank(") {
                // (league_id, rank) — CompetitionRank 실측 (id, rank) 순서와 동일 가정
                nums_of(team_dbg).get(1).map(|r| format!("리그 {}위 시드", r)).unwrap_or_else(|| format!("리그시드{:?}", nums_of(team_dbg)))
            } else if team_dbg.starts_with("CompetitionRank(") {
                // (competition_id, rank) — 2027 TACJ 플옵 실측으로 순서 검증됨(id 고정, 1~6위).
                // 해당 대회 순위표에서 rank위 팀명을 해석해 병기 (플옵 시점 = 정규 최종순위).
                let ns = nums_of(team_dbg);
                match (ns.first().copied(), ns.get(1).copied()) {
                    (Some(cid_), Some(r)) if r >= 1 => {
                        let mut nm = String::new();
                        if let Some(c2) = db.league_competitions.values().find(|c| c.id as i64 == cid_) {
                            let mut rows: Vec<(usize, i64, i64, i64)> = c2.standings.iter()
                                .map(|(&tid, st)| (tid, st.win as i64, st.set_win as i64 - st.set_lose as i64, st.kill as i64))
                                .collect();
                            rows.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| b.2.cmp(&a.2)).then_with(|| b.3.cmp(&a.3)));
                            if let Some(&(tid, ..)) = rows.get((r - 1) as usize) { nm = team_name_of(tid); }
                        }
                        if nm.is_empty() { format!("정규 {}위 시드", r) } else { format!("정규 {}위 시드({})", r, nm) }
                    }
                    _ => format!("대회시드{:?}", ns),
                }
            } else if team_dbg.starts_with("GroupRank(") {
                format!("조시드{:?}", nums_of(team_dbg)) // 튜플 순서 미검증 — raw 유지
            } else {
                team_dbg.to_string()
            }
        };
        let push_bracket = |text: &mut String, list: &Vec<usize>, fmt_label: &str, title: &str| {
            if list.is_empty() { return; }
            // 간선 기반 형식 추정
            let mut has_loser = false;
            let mut has_winner = false;
            for &mid in list.iter() {
                if let Some(m) = match_by_id(mid) {
                    let t1 = format!("{:?}", m.team1);
                    let t2 = format!("{:?}", m.team2);
                    if t1.starts_with("LoserOf") || t2.starts_with("LoserOf") { has_loser = true; }
                    if t1.starts_with("WinnerOf") || t2.starts_with("WinnerOf") { has_winner = true; }
                }
            }
            let any_ended = list.iter().any(|&mid| match_by_id(mid)
                .map_or(false, |m| format!("{:?}", m.running_state).contains("End")));
            let shape = if has_loser { "더블 엘리미네이션형(패자조 간선 존재)" }
                else if has_winner { "녹아웃형(싱글 엘리미네이션 계열)" }
                else if any_ended { "전 경기 확정(간선 정보 소실 — 형식은 설정값 참조)" }
                else { "고정 대진형(조별/스위스 라운드)" };
            text.push_str(&format!("\n{} 형식: {} | 구조: {}\n", title, fmt_label, shape));
            for (i, &mid) in list.iter().enumerate() {
                let Some(m) = match_by_id(mid) else {
                    text.push_str(&format!("[M{}] (매치 {} 정보 없음)\n", i + 1, mid));
                    continue;
                };
                let date_s = format!("{}", m.date);
                let date_s = if date_s.len() >= 10 { date_s[..10].to_string() } else { date_s };
                let bo = m.need_win.max(1) * 2 - 1;
                let s1 = slot_str(&format!("{:?}", m.team1), list);
                let s2 = slot_str(&format!("{:?}", m.team2), list);
                let st = format!("{:?}", m.running_state);
                let state_s = if st.contains("End") {
                    let w = hl_field_i64(&st, "winner:").unwrap_or(-1);
                    let a = hl_field_i64(&st, "team1_score:").unwrap_or(0);
                    let b = hl_field_i64(&st, "team2_score:").unwrap_or(0);
                    format!("종료 {}:{} → 승자 {}", a, b, team_name_of(w.max(0) as usize))
                } else if st.contains("Running") {
                    "진행중".to_string()
                } else {
                    "예정".to_string()
                };
                text.push_str(&format!("[M{}] {} | BO{} | {} vs {} | {}\n", i + 1, date_s, bo, s1, s2, state_s));
            }
        };
        let format_label = |is_league: bool| -> String {
            match *OPTION_RAW.lock().unwrap_or_else(|e| e.into_inner()) {
                Some((lr, cr, gf)) => if is_league {
                    format!("리그 플레이오프 규칙={} [raw {},{},{}]", league_rule_kr(lr), lr, cr, gf)
                } else {
                    format!("국제대회 규칙={} / 그룹 형식={} [raw {},{},{}]", cup_rule_kr(cr), group_stage_format_kr(gf), lr, cr, gf)
                },
                None => "설정 미확인".to_string(),
            }
        };

        text.push_str("--- 경기 구분 상세 (추가 정보) ---\n");
        let mut done = false;
        if our_match_is_practice {
            text.push_str("경기 구분: 연습 경기 (스크림/비공식전 — 리그 순위와 무관)\n");
            done = true;
        }
        if !done { if let Some(match_id) = our_match_id {
            // 1) 자국 리그: 정규시즌 or 리그 플레이오프
            for comp in db.league_competitions.values() {
                let in_regular = comp.matches.contains(&match_id);
                let po_pos = comp.playoffs.iter().position(|&x| x == match_id);
                let ps_pos = comp.promotion_series.iter().position(|&x| x == match_id); // 0.5.2 신규
                if !in_regular && po_pos.is_none() && ps_pos.is_none() { continue; }
                if in_regular {
                    text.push_str(&format!("경기 구분: 자국 리그 정규 시즌 — {}\n", league_name));
                } else if let Some(pp) = ps_pos {
                    text.push_str(&format!("경기 구분: 승강전 — {}\n", promotion_round_label(pp, comp.promotion_series.len())));
                    // 승강전 맥락: 양 팀 소속 리그와 티어(division)를 병기해 무엇이 걸린 경기인지 드러냄.
                    // ⚠ division 값의 방향(작을수록 상위인지)은 미실증 → 서열을 단정하지 않고 사실만 표기.
                    let league_of = |tid: usize| -> Option<(usize, &str, i64)> {
                        let lid = db.teams.get(&tid)?.league_id;
                        let l = db.leagues.get(&lid)?;
                        Some((lid, translate_league_name(l.name.as_str()), l.division as i64))
                    };
                    match (league_of(player_team_id), league_of(opponent_team_id)) {
                        (Some((our_lid, our_ln, our_dv)), Some((opp_lid, opp_ln, opp_dv))) => {
                            text.push_str(&format!(
                                "승강전 대진: 우리 팀 {} (division {}) vs 상대 팀 {} (division {})\n",
                                our_ln, our_dv, opp_ln, opp_dv));
                            if our_lid != opp_lid {
                                text.push_str("→ 서로 다른 리그 소속 간 맞대결 = 상위 리그 잔류권과 승격권이 걸린 경기\n");
                            } else {
                                text.push_str("→ 같은 리그 소속 간 승강전 경기\n");
                            }
                        }
                        _ => { text.push_str(&format!("승강전 대진: {} (상대 팀 소속 리그 확인 불가)\n", league_name)); }
                    }
                    text.push_str(&format!("승강 결과 확정 여부: {}\n",
                        if comp.promotion_series_finalized { "확정됨" } else { "미확정 (진행중)" }));
                } else {
                    text.push_str(&format!("경기 구분: 리그 플레이오프 — {} / {}\n", league_name, playoff_round_label(po_pos.unwrap_or(0), comp.playoffs.len())));
                }
                // 플레이오프 대진표 (생성돼 있으면 정규시즌 말미에도 출력)
                push_bracket(&mut text, &comp.playoffs, &format_label(true), "[플레이오프 대진표]");
                // 승강전 대진표 (0.5.2 신규 — 생성돼 있으면 정규시즌/플옵 중에도 출력)
                push_bracket(&mut text, &comp.promotion_series, &format_label(true), "[승강전 대진표]");
                done = true;
                break;
            }
            // 2) 국제전: 그룹 스테이지 or 녹아웃 (+ 조별 순위는 위 원본 섹션에 없는 신규 정보라 여기서 출력)
            if !done {
                for tc in db.tournament_competitions.values() {
                    let in_group = tc.group_matches.contains(&match_id);
                    let ko_pos = tc.tournament_matches.iter().position(|&x| x == match_id);
                    if !in_group && ko_pos.is_none() { continue; }
                    let mut tname = String::from("국제전");
                    for t in db.tournaments.values() {
                        if t.competitions.contains(&(tc.id as usize)) { tname = tournament_kr(&t.name); break; }
                    }
                    // 스위스 = 비어있지 않은 그룹이 1개뿐이고 8팀 이상 (조별 리그는 소규모 조 여러 개).
                    // ⚠ groups 벡터에 빈 그룹이 미리 할당돼 있을 수 있어 빈 그룹은 제외하고 판정.
                    let filled_groups: Vec<usize> = tc.groups.iter().enumerate()
                        .filter(|(_, g)| !g.is_empty()).map(|(i, _)| i).collect();
                    let is_swiss = filled_groups.len() == 1
                        && tc.groups.get(filled_groups[0]).map_or(false, |g| g.len() >= 8);
                    let stage = if in_group {
                        if is_swiss {
                            "스위스 스테이지".to_string()
                        } else {
                            match tc.groups.iter().position(|g| g.contains_key(&player_team_id)) {
                                Some(gi) => format!("조별 리그 ({}조)", gi + 1),
                                None => "그룹 스테이지".to_string(),
                            }
                        }
                    } else {
                        let pos = ko_pos.unwrap_or(0);
                        let total = tc.tournament_matches.len();
                        if total == 8 { playoff_round_label(pos, 8) }
                        else if total > 0 && pos + 1 == total { "결승전".to_string() }
                        else { format!("녹아웃 스테이지 {}번째 경기 (총 {}경기)", pos + 1, total.max(pos + 1)) }
                    };
                    text.push_str(&format!("경기 구분: 국제전 [{}] — {}\n", tname, stage));
                    if is_swiss {
                        // 스위스: 순위 나열 대신 전적(승-패) 버킷별 표기 (게임 UI와 동일 개념)
                        if let Some(g) = tc.groups.get(filled_groups[0]) {
                            text.push_str(&format!("\n[{} 스위스 스테이지 현황 (3승 진출 / 3패 탈락)]\n", tname));
                            let mut rows = standing_rows_of!(g);
                            rows.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.2.cmp(&b.2)).then_with(|| b.3.cmp(&a.3)));
                            let mut cur: Option<(i64, i64)> = None;
                            for (tid, w, l, sd, _k) in rows {
                                if cur != Some((w, l)) {
                                    cur = Some((w, l));
                                    let tag = if w >= 3 { " — 녹아웃 진출" } else if l >= 3 { " — 탈락" } else { "" };
                                    text.push_str(&format!("[{}-{}]{}\n", w, l, tag));
                                }
                                let team_name = db.teams.get(&tid).map(|t| t.name.as_str()).unwrap_or("알 수 없는 팀");
                                let star = if tid == player_team_id { " ◀ 우리 팀" } else { "" };
                                let sds = if sd > 0 { format!("+{}", sd) } else { sd.to_string() };
                                text.push_str(&format!(" - {} (세트득실 {}){}\n", team_name, sds, star));
                            }
                        }
                        // 스위스 라운드 매치 목록 (HTML 대진표 렌더용 — 라운드는 소비측이 전적 누적으로 계산)
                        push_bracket(&mut text, &tc.group_matches, &format_label(false), "[스위스 라운드 매치]");
                    } else {
                        for (gi, g) in tc.groups.iter().enumerate() {
                            if g.is_empty() { continue; }
                            text.push_str(&format!("\n[{} {}조 순위]\n", tname, gi + 1));
                            let mut rows = standing_rows_of!(g);
                            push_standing_rows(&mut text, &mut rows, &*db, player_team_id);
                        }
                    }
                    // 국제전 녹아웃 대진표
                    push_bracket(&mut text, &tc.tournament_matches, &format_label(false), "[플레이오프 대진표]");
                    done = true;
                    break;
                }
            }
        } }
        if !done {
            text.push_str("경기 구분: 판별 불가(진행 정보 부족)\n");
        }
        text.push_str("\n");
    }

    // ── 경기 하이라이트 (직접 관전한 세트의 프레임 분석 결과) ──
    {
        let mut roster: Vec<String> = Vec::new();
        for p in our_latest_replay.blue_team.iter().chain(our_latest_replay.red_team.iter()) {
            if let Some(a) = db.athletes.get(&p.athlete_id) { roster.push(a.name.clone()); }
        }
        let blocks = HL_BLOCKS.lock().unwrap_or_else(|e| e.into_inner());
        // 1차 = 관전 시점에 식별한 매치 id 정확 매칭 (재경기 혼입 원천 차단).
        // 식별 실패 블록(다시보기 등)만 이름 겹침 ≥8(양 팀 일치) 폴백.
        let matched: Vec<&HlBlock> = blocks.iter()
            .filter(|b| match (b.match_id, our_match_id) {
                (Some(bm), Some(cm)) => bm == cm,
                _ => b.names.iter().filter(|n| roster.contains(n)).count() >= 8,
            })
            .collect();
        if !matched.is_empty() {
            text.push_str("--- 경기 하이라이트 (직접 관전한 세트 프레임 분석) ---\n");
            text.push_str("연속킬 / 다인 궁극기 / 포탑 데스 / 골드 대역전 등 명장면 목록입니다.\n");
            // 세트 번호 특정: 블록의 최종 킬스코어 ↔ 시리즈 리플레이의 팀 킬수 대조
            let mut used_sets: Vec<usize> = Vec::new();
            for (i, b) in matched.iter().enumerate() {
                let set_idx = our_series.iter().enumerate().position(|(si, r)| {
                    !used_sets.contains(&si)
                        && r.blue_performance.total_kills as i64 == b.score.0
                        && r.red_performance.total_kills as i64 == b.score.1
                });
                let label = match set_idx {
                    Some(si) => { used_sets.push(si); format!("[{}세트 하이라이트 (킬스코어 {}:{})]", si + 1, b.score.0, b.score.1) }
                    None => format!("[관전 세트 #{} (킬스코어 {}:{})]", i + 1, b.score.0, b.score.1),
                };
                text.push_str(&format!("{}\n", label));
                for l in &b.lines { text.push_str(&format!(" - {}\n", l)); }
            }
            text.push_str("\n");
        }
    }

    for (i, our_replay) in our_series.iter().enumerate() {
        text.push_str(&build_single_match_summary(&*db, our_replay, true, i + 1, our_attention));
    }

    for (i, other_replay) in other_series.iter().enumerate() {
        text.push_str(&build_single_match_summary(&*db, other_replay, false, i + 1, other_attention));
    }

    let dir_path = Path::new("mods").join(MOD_ID);
    let _ = fs::create_dir_all(&dir_path);

    let escaped_summary = escape_json_string(&text);

    // ★승수는 blue/red 슬롯이 아니라 실제 팀 id 기준으로 센다.
    //   TFM2 는 세트마다 진영(blue/red)을 스왑하므로, 슬롯 기준 카운트는 같은 팀의
    //   1세트 blue승 + 2세트 red승을 blue_wins=1 / red_wins=1 로 나눠 세어(2-0 을 1-1 로) 오판 →
    //   is_series_finished 가 영영 false → 시리즈 종료 반응 자동생성 누락. (진영별 승자 팀 id 로 집계해 해결)
    let team_a = our_latest_replay.blue_team_id; // 기준 팀(임의). 나머지 세트는 이 팀 기준으로 승수 귀속.
    let mut wins_a = 0;
    let mut wins_b = 0;
    for r in &our_series {
        let winner_team_id = if r.blue_team_win { r.blue_team_id } else { r.red_team_id };
        if winner_team_id == team_a { wins_a += 1; } else { wins_b += 1; }
    }
    let max_wins = wins_a.max(wins_b);

    let is_series_finished = max_wins >= need_win;
    let series_id = our_latest_replay.id;

    let js_output = format!(
        r#"var latestMatchData = {{"summary": "{}", "series_id": {}, "is_series_finished": {}, "date": "{}"}};"#,
        escaped_summary,
        series_id,
        is_series_finished,
        match_date
    );
    let file_path = dir_path.join("latest_match.js");
    fs::write(file_path, js_output)?;

    // 브라우저 오픈은 여기서 하지 않음 — 갤러리는 게임 시작 시 open_gallery() 로 1회만 열고,
    // 이후 데이터 갱신은 HTML 이 latest_match.js 를 5초 폴링(캐시버스터)해 스스로 반영한다.
    // (series_id / is_series_finished 는 js 로 그대로 내보내 HTML 자동 반응생성 판단에 사용됨)
    let _ = (is_series_finished, series_id, last_opened_series_id);

    Ok(())
}

fn init(_ctx: &GameCtx) -> ModRegistration {
    let mut reg = ModRegistration::new(MOD_ID);
    reg.set_extension(ReactionExtension::default());
    reg.set_server_extension(ReactionServerExt); // 대회 형식 설정(GamePlayOption) 캡처용
    reg
}

declare_mod!(init);