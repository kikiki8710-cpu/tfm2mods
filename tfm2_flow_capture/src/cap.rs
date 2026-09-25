//! cap — sim 캡처(`StableMatchHook`). 클래식 run_tick detour + raw 오프셋(game+0xec90 seed · +0x858 선수 · +0x738 엔티티 ·
//!   +0xb220 kill_logs · +0xb3b0 Strategy …)을 stable sim API 로 1:1 대체한다. 패치마다 재핀할 RVA·오프셋 = 0.
//!
//! 대상 sim(`sim_origin().kind`):
//!   · ClientMatchView / ClientSpectate / ClientReplay = 화면 재생 경기만(09-25 유저 결정 — Tool(조합테스트)·ServerPresim 제외)
//!   · 같은 세트의 선행 sim·재생 sim 은 seed 폴더 중복 제거로 하나만 남긴다(틱 속도 필터는 09-25 폐기 — 재생 sim 도 빠르다)
//!   같은 세트가 여러 sim 으로 돈다(선행 sim·재생 sim·presim — crm hl.rs 09-20 실측) → 파일은 seed 폴더에서 "가장 큰 것 하나"만 남긴다(클래식과 동일).
//! 샘플 = 30틱마다 1행(클래식 SAMPLE_EVERY 30). 락은 샘플 틱에만 잡는다(배경 sim 은 origin 판정만 하고 빠진다).
//! 이름 해석(선수명·팀명·세트 번호)은 sim 스레드에서 할 수 없으므로 슬롯을 끝낸 뒤 클라 post_update(`lib::flush_ready`)가 한다.
use mod_api_stable::{SimOriginKindV1, SimOriginV1, StableMatchHook, StableSim};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;
use std::time::Instant;

pub const SAMPLE_EVERY: usize = 30;
pub const NATH: usize = 10;
const SMAX: usize = 4096; // 30틱 × 4096 ≈ 68분(30틱/초)
const KMAX: usize = 256;
const OMAX: usize = 64;
const MAXNEUT: usize = 24;
const SLOTS: usize = 16;
pub const MAP: i64 = 960_000; // 30×30 셀 × 32000 (클래식 좌표계와 동일 가정 — 첫 실행 로그로 확인)

/// (kind, match_id, replay_id, set_index, seed)
pub type Key = (u32, u64, u64, u64, u64);

#[derive(Clone, Default)]
pub struct PInfo {
    pub pid: usize,
    pub team: usize,
    pub lane: u32, // LaneV1 코드(0 Top 1 Jg 2 Mid 3 Bot 4 Sup), 모르면 9
    pub champ: String,
}
#[derive(Clone, Default)]
pub struct Sample {
    pub tick: u64,
    pub sc: [u64; 2],
    pub gold: [i64; NATH],
    pub pos: [(i64, i64, i64, i64); NATH], // x, y, hp, lv (사망 = 0)
    pub deal: [i64; NATH],
    pub tank: [i64; NATH],
    pub minc: [u32; 8],       // t0[top,mid,bot,기타], t1[…]
    pub wave: [(i64, i64); 6], // t0[top,mid,bot], t1[…] 좌표합(flush 때 카운트로 나눔)
    pub jmask: u32,
}
#[derive(Clone)]
pub struct Slot {
    pub key: Key,
    pub t0_ms: u64,
    pub last_tick: u64,
    pub last_wall: Instant,
    pub first_tick: u64,
    pub first_wall: Instant,
    pub ended: bool,
    pub players: Vec<PInfo>,
    pub samples: Vec<Sample>,
    pub kills: Vec<[u64; 10]>, // tick, killer_team, killer_role, killed_role, assist_n, assist×4(u32::MAX = 없음), _
    pub kill_seen: usize,
    pub objs: Vec<(u64, usize, &'static str, String)>, // tick, team, type, detail
    towers: Vec<(usize, usize, u32, bool)>,            // entity id, team, 라인 역할코드(0/2/3), dead
    epics: Vec<(usize, &'static str, bool)>,           // entity id, type, dead
    last_champ_pos: Vec<(usize, i64, i64)>,             // team, x, y (직전 샘플 — 에픽 처치 팀 추정용)
    pub neut: Vec<(u32, usize, i64, i64, String)>,      // kind(4 정글 / 9 곰 추정), team, x, y, name
    neut_ok: bool,
    pub strat: [Option<String>; 2],
    names_done: bool,
    /// 스캔에서 본 비챔피언·비미니언·비포탑 엔티티 이름(진단 — flush 로그)
    pub names_seen: Vec<String>,
}

static SLOTS_V: Mutex<Vec<Slot>> = Mutex::new(Vec::new());
pub static HITS: AtomicU64 = AtomicU64::new(0);
pub static DROPPED_FULL: AtomicU64 = AtomicU64::new(0);
pub static SAMPLES: AtomicU64 = AtomicU64::new(0);
/// 진단 1회 기록(통계 JSON 키·엔티티 이름 목록 — 필드명·좌표계 확정용)
static DIAG_DONE: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

fn now_ms() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0)
}

fn origin(sim: &StableSim<'_>) -> Option<SimOriginV1> {
    sim.sim_origin()
}
fn wanted(o: &SimOriginV1) -> bool {
    // ★09-25 유저 결정: **화면에 재생되는 경기만** 저장 — 조합테스트(Tool)·서버 선행 계산(ServerPresim) 제외.
    //   같은 종류의 선행 sim(crm 09-20 라이브 관전 실측)은 같은 seed 라 중복 제거에서 하나로 합쳐진다.
    let k = o.kind;
    k == SimOriginKindV1::ClientMatchView as u32 || k == SimOriginKindV1::ClientSpectate as u32 || k == SimOriginKindV1::ClientReplay as u32
}

/// 벽시계 대비 틱 진행 속도(진단 로그용). ⛔재생 판정에 쓰지 않는다 — 화면 재생 sim 도 ≈1170틱/초로 앞서 달린다(09-24 실측).
pub fn ticks_per_sec(s: &Slot) -> f64 {
    let secs = s.last_wall.duration_since(s.first_wall).as_secs_f64();
    let dt = s.last_tick.saturating_sub(s.first_tick) as f64;
    if secs < 0.5 { if dt > 300.0 { f64::INFINITY } else { 0.0 } } else { dt / secs }
}

/// 통계 JSON 스칼라(정수/실수/문자열 숫자) → i64
fn stat_i(p: &mod_api_stable::StablePlayer<'_, '_>, path: &str) -> Option<i64> {
    let v = p.statistics_json(path)?;
    let t = v.trim().trim_matches('"');
    t.parse::<i64>().ok().or_else(|| t.parse::<f64>().ok().map(|f| f as i64))
}

/// 라인 판정(엔티티 좌표) — 넥서스 = 좌하(96000,864000)/우상(864000,96000) ⟹ 미드 = x+y≈MAP 대각, 탑 = 좌·상 변, 바텀 = 우·하 변.
///   0 top / 1 mid / 2 bot / 3 기타(정글 등). 클래식 미니언 lane 바이트(0/1/2)와 같은 순서(과거 flow 파일 wave 중심점으로 확인).
pub fn lane_of(x: i64, y: i64) -> usize {
    let d_top = x.min(y);
    let d_bot = (MAP - x).min(MAP - y);
    let d_mid = ((x + y - MAP).abs() as f64 / std::f64::consts::SQRT_2) as i64;
    let (i, d) = [(0usize, d_top), (1, d_mid), (2, d_bot)].into_iter().min_by_key(|e| e.1).unwrap_or((3, i64::MAX));
    if d > 110_000 { 3 } else { i }
}

pub struct Hook;
impl StableMatchHook for Hook {
    fn on_match_tick(&self, sim: &mut StableSim<'_>, _seed: u64) {
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let t = sim.tick();
            let end = sim.is_end();
            if t % SAMPLE_EVERY != 0 && !end {
                return;
            }
            let Some(o) = origin(sim) else { return };
            if !wanted(&o) {
                return;
            }
            HITS.fetch_add(1, Ordering::Relaxed);
            let key: Key = (o.kind, o.match_id, o.replay_id, o.set_index, sim.seed());
            let mut g = SLOTS_V.lock().unwrap_or_else(|e| e.into_inner());
            let idx = match g.iter().position(|s| s.key == key && !s.ended) {
                Some(i) => i,
                None => {
                    if g.len() >= SLOTS {
                        // 끝난 슬롯은 flush 대기 중 — 새 경기를 막지 않게 가장 오래된 미종료 슬롯도 밀지 않는다(드롭)
                        DROPPED_FULL.fetch_add(1, Ordering::Relaxed);
                        return;
                    }
                    g.push(Slot {
                        key,
                        t0_ms: now_ms(),
                        last_tick: t as u64,
                        last_wall: Instant::now(),
                        first_tick: t as u64,
                        first_wall: Instant::now(),
                        ended: false,
                        players: Vec::new(),
                        samples: Vec::new(),
                        kills: Vec::new(),
                        kill_seen: 0,
                        objs: Vec::new(),
                        towers: Vec::new(),
                        epics: Vec::new(),
                        last_champ_pos: Vec::new(),
                        neut: Vec::new(),
                        neut_ok: false,
                        strat: [None, None],
                        names_done: false,
                        names_seen: Vec::new(),
                    });
                    g.len() - 1
                }
            };
            let s = &mut g[idx];
            s.last_tick = t as u64;
            s.last_wall = Instant::now();
            sample(sim, s);
            if end {
                s.ended = true;
            }
        }));
    }
}

fn sample(sim: &StableSim<'_>, s: &mut Slot) {
    let t = sim.tick() as u64;
    // ── 선수(플레이어 순서 = R/E 순서 — 클래식은 선수 배열(R)과 엔티티 배열(E) 순서가 달라 브릿지가 필요했지만 여기선 같다)
    let pc = sim.player_count().min(NATH);
    if s.players.len() != pc || !s.names_done {
        let mut v = Vec::with_capacity(pc);
        for i in 0..pc {
            let Some(p) = sim.player_at(i) else { continue };
            let champ = p.champion().and_then(|e| e.name()).unwrap_or_default();
            v.push(PInfo { pid: p.id(), team: p.team(), lane: p.lane().map(|l| l as u32).unwrap_or(9), champ });
        }
        // ★순서 = (진영, 포지션) — 분석기(feedback_flow)가 E 인덱스를 team*5+role 로 가정한다(킬 행 kt*5+kr 매핑).
        v.sort_by_key(|p| (p.team, p.lane));
        // on_match_start·초기 틱엔 챔피언 엔티티가 없어 이름이 빈다(crm 09-20 실측) → 전원 채워질 때까지 재시도
        s.names_done = v.len() == pc && v.iter().all(|p| !p.champ.is_empty());
        s.players = v;
    }
    // ── 전술 1회(JSON 원문 — 클래식 P 행의 24B 대신 PJ 행)
    for tm in 0..2 {
        if s.strat[tm].is_none() {
            s.strat[tm] = sim.strategy_get_json(tm, "");
        }
    }
    // ── 킬 로그 증분
    let kc = sim.kill_log_count();
    while s.kill_seen < kc && s.kills.len() < KMAX {
        if let Some(k) = sim.kill_log_at(s.kill_seen) {
            let mut row = [u32::MAX as u64; 10];
            row[0] = k.tick as u64;
            row[1] = k.killer_team as u64;
            row[2] = k.killer_position as u64;
            row[3] = k.killed_position as u64;
            row[4] = k.assist_count as u64;
            for j in 0..(k.assist_count as usize).min(4) {
                row[5 + j] = k.assist_positions[j] as u64;
            }
            s.kills.push(row);
        }
        s.kill_seen += 1;
    }
    if s.samples.len() >= SMAX {
        return;
    }
    let mut sm = Sample { tick: t, ..Default::default() };
    let mut champ_pos: Vec<(usize, i64, i64)> = Vec::new();
    for (i, pi) in s.players.iter().enumerate().take(NATH) {
        let Some(p) = sim.get_player(pi.pid) else { continue };
        if pi.team < 2 {
            sm.sc[pi.team] += p.kills() as u64;
        }
        // 골드 = 누적 획득(통계 "gold") — `gold()` 는 보유 골드라 구매 때 급락한다(crm 09-20 실측). 없으면 보유 골드 폴백.
        sm.gold[i] = stat_i(&p, "gold").unwrap_or(p.gold() as i64);
        sm.deal[i] = stat_i(&p, "deal").unwrap_or(0);
        sm.tank[i] = stat_i(&p, "tank").unwrap_or(0);
        if p.is_alive() {
            if let Some(e) = p.champion() {
                let (x, y) = e.pos();
                let (hp, _) = e.hp();
                sm.pos[i] = (x as i64, y as i64, hp as i64, e.level() as i64);
                champ_pos.push((pi.team, x as i64, y as i64));
            }
        }
    }
    // ── 엔티티 스캔: 미니언(라인 카운트·웨이브 중심) / 포탑 / 에픽 / 정글 중립
    let n = sim.entity_count().min(1024);
    let mut neut_now: Vec<(u32, usize, i64, i64, String)> = Vec::new();
    let mut diag_names: Vec<String> = Vec::new();
    for i in 0..n {
        let Some(e) = sim.entity_at(i) else { continue };
        if e.is_champion() {
            continue;
        }
        let (x, y) = e.pos();
        let (x, y) = (x as i64, y as i64);
        let team = e.team();
        if e.is_minion() {
            if team < 2 && e.is_alive() {
                let ln = lane_of(x, y);
                sm.minc[team * 4 + ln] += 1;
                if ln < 3 {
                    let w = &mut sm.wave[team * 3 + ln];
                    w.0 += x;
                    w.1 += y;
                }
            }
            continue;
        }
        let id = e.id();
        let name = e.name().unwrap_or_default().to_ascii_lowercase();
        // 포탑·넥서스 — 넥서스는 분석기가 "detail 빈 tower 이벤트 = 패배 팀" 으로 승패를 판정한다(클래식 tower 로그와 같은 규칙)
        let nexus = name.contains("nexus");
        if e.is_tower() || nexus {
            if !s.towers.iter().any(|tw| tw.0 == id) && e.is_alive() {
                let role = if nexus { 99 } else { match lane_of(x, y) { 0 => 0, 1 => 2, 2 => 3, _ => 9 } };
                s.towers.push((id, team, role, false));
            }
            continue;
        }
        if !DIAG_DONE.load(Ordering::Relaxed) && diag_names.len() < 40 && !diag_names.contains(&name) {
            diag_names.push(name.clone());
        }
        if name.is_empty() {
            continue;
        }
        if !s.names_seen.contains(&name) && s.names_seen.len() < 40 {
            s.names_seen.push(name.clone());
        }
        // ★09-24 실측: 세르펜 = "serpen" 포함. 모르가드는 "morgard" 가 아니었다(첫 리플레이 O 에 0건) — exe 클래스명 epic_monster ⟹ 이름 "epic…" 로 판정
        let epic = if name.contains("morgard") || name.starts_with("epic") { Some("morgard") } else if name.contains("serpen") { Some("serpen") } else { None };
        if let Some(ty) = epic {
            if e.is_alive() && !s.epics.iter().any(|ep| ep.0 == id) {
                s.epics.push((id, ty, false));
            }
            continue;
        }
        // 정글 중립(클래식 kind 4 정글 / 9 곰) — stable 엔 kind 가 없어 이름으로 곰 구분(추정)
        // 소환수(예: small_jiangshi — 09-24 실측)는 진영이 있다 → 중립(team = usize::MAX)만 정글로 친다
        if team == usize::MAX && e.is_alive() && e.hp().0 > 0 {
            let kind = if name.contains("bear") { 9 } else { 4 };
            neut_now.push((kind, team, x, y, name));
        }
    }
    // 정글 캠프 좌표 1회(측당 4 = 8개 이상 보일 때 — 클래식과 같은 조기 커밋 방지)
    if !s.neut_ok && neut_now.len() >= 8 {
        s.neut = neut_now.iter().take(MAXNEUT).cloned().collect();
        s.neut_ok = true;
    }
    if s.neut_ok {
        for nn in &neut_now {
            for (ci, c) in s.neut.iter().enumerate().take(32) {
                if (nn.2 - c.2).abs() < 24_000 && (nn.3 - c.3).abs() < 24_000 {
                    sm.jmask |= 1 << ci;
                    break;
                }
            }
        }
    }
    // 포탑 파괴(30틱 해상도) — O,tick,**포탑 주인 팀**,tower,라인(넥서스 = 빈 문자열). 분석기 규칙: 타워 team = 잃은 팀.
    for tw in s.towers.iter_mut() {
        if tw.3 {
            continue;
        }
        let dead = sim.get_entity(tw.0).map(|e| !e.is_alive()).unwrap_or(true);
        if dead {
            tw.3 = true;
            if s.objs.len() < OMAX {
                let detail = match tw.2 { 0 => "탑", 2 => "미드", 3 => "바텀", _ => "" };
                s.objs.push((t, tw.1, "tower", detail.to_string()));
            }
        }
    }
    // 에픽 처치 — 처치 팀은 API 에 없음 ⟹ 직전 샘플에서 그 위치에 가장 가까운 챔피언의 팀(추정)
    let prev = std::mem::take(&mut s.last_champ_pos);
    for ep in s.epics.iter_mut() {
        if ep.2 {
            continue;
        }
        let ent = sim.get_entity(ep.0);
        let dead = ent.as_ref().map(|e| !e.is_alive()).unwrap_or(true);
        if dead {
            ep.2 = true;
            let (ex, ey) = ent.map(|e| { let (x, y) = e.pos(); (x as i64, y as i64) }).unwrap_or((MAP / 2, MAP / 2));
            let team = prev
                .iter()
                .chain(champ_pos.iter())
                .min_by_key(|c| (c.1 - ex).abs() + (c.2 - ey).abs())
                .map(|c| c.0)
                .unwrap_or(0);
            if s.objs.len() < OMAX {
                s.objs.push((t, team, ep.1, String::new()));
            }
        }
    }
    s.last_champ_pos = champ_pos;
    if !DIAG_DONE.load(Ordering::Relaxed) && t >= 60 && s.names_done {
        DIAG_DONE.store(true, Ordering::Relaxed);
        let st = sim.player_at(0).and_then(|p| p.statistics_json("")).unwrap_or_default();
        crate::log(&format!(
            "[diag] origin kind={} t={} 통계JSON(p0)={} | 기타엔티티={:?} | 포탑 {}개 에픽 {}개 | p0 pos={:?} | strat0={}",
            s.key.0,
            t,
            st.chars().take(1500).collect::<String>(),
            diag_names,
            s.towers.len(),
            s.epics.len(),
            sm.pos[0],
            s.strat[0].clone().unwrap_or_default().chars().take(600).collect::<String>()
        ));
    }
    s.samples.push(sm);
    SAMPLES.fetch_add(1, Ordering::Relaxed);
}

/// flush 대상 꺼내기: 끝난 슬롯 + 벽시계로 `idle_secs` 이상 tick 이 멈춘 슬롯(리플레이를 중간에 닫은 경우).
pub fn take_ready(idle_secs: u64) -> Vec<Slot> {
    let mut g = SLOTS_V.lock().unwrap_or_else(|e| e.into_inner());
    let mut out = Vec::new();
    let mut i = 0;
    while i < g.len() {
        if g[i].ended || g[i].last_wall.elapsed().as_secs() >= idle_secs {
            out.push(g.remove(i));
        } else {
            i += 1;
        }
    }
    out
}

/// 상태 파일용 요약
pub fn active_summary() -> String {
    let g = SLOTS_V.lock().unwrap_or_else(|e| e.into_inner());
    let mut s = String::new();
    for sl in g.iter() {
        s.push_str(&format!(
            " kind={} match={} replay={} set={} seed=0x{:x} n={} tick={} ended={}\n",
            sl.key.0, sl.key.1 as i64, sl.key.2 as i64, sl.key.3 as i64, sl.key.4, sl.samples.len(), sl.last_tick, sl.ended
        ));
    }
    s
}
