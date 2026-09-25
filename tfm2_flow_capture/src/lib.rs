// ═══════════════════════════════════════════════════════════════════════════
// tfm2_flow_capture 0.8 — 경기 흐름 캡처 (stable ABI 재작성 · 2026-09-24 · game 0.6.1)
//
// 목적(불변): 리플레이/관전/내 경기/조합테스트 sim 을 30틱 간격으로 샘플링해 flow\<seed>\f_<seed>_<ms>.txt 로 남긴다
//   → feedback_tools(flow_digest·feedback_flow·match viewer)가 골드 곡선·킬 타임라인·스노우볼을 재구성.
//
// 클래식(0.5.x, `_classic_058\src`) 대비:
//   · 캡처 = `StableMatchHook::on_match_tick`(cap.rs). run_tick detour·game/엔티티/킬로그/전술 raw 오프셋 전부 삭제 ⟹ 재핀 대상 RVA 0.
//   · 경기 식별 = `sim_origin()`(kind·match_id·replay_id·set_index). 클래식의 LAST_WATCHED_SEED/ctor retaddr/MY_ATH 게이트 삭제.
//   · ★09-25 유저 결정: 화면에 재생되는 경기만 저장(관전·리플레이·내 경기 보기). 조합테스트·스킵한 경기(서버 선행 계산)는 저장 안 함.
//   · 이름·세트·시리즈 = 레코드 JSON(`record_get_json` Match/MatchReplay). running_matches raw walk(미확정 오프셋) 삭제.
//   · 밴픽 실시간(banpick_live.json) = 밴픽 UI 카드 상태 읽기(bp.rs). scene_step detour 삭제.
//   · 전술 = `strategy_get_json` 원문 → PJ 행(클래식 P 행 24B 대체 — 분석기가 JSON 을 디코드).
//   · ★09-25 유저 결정: 다시보기 전술 주입(replay_override) 기능 삭제.
//   ⛔미이식: ① 다시보기 편집 팝업(ui_replay) — 전술 주입과 함께 폐기 ② 코치 전술 추천 캡처(game_reco — recommend_strategy_inner detour, stable 대응 지점 없음).
// ═══════════════════════════════════════════════════════════════════════════
mod bp;
mod cap;

use mod_api_stable::{declare_stable_mod, LogLevel, RecordKindV1, SceneKindV1, SimOriginV1, StableClient, StableExtension, StableHost, StableMod};
use serde_json::Value;
use std::collections::HashSet;
use std::fs;
use std::io::Write as _;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Mutex, RwLock};

pub const MOD_ID: &str = "tfm2_flow_capture";
const LOG_ENABLED: bool = true;

#[link(name = "kernel32")]
extern "system" {
    fn GetModuleHandleExW(flags: u32, addr: *const u16, out: *mut usize) -> i32;
    fn GetModuleFileNameW(h: usize, buf: *mut u16, n: u32) -> u32;
}
pub fn mod_dir() -> Option<String> {
    let mut h: usize = 0;
    if unsafe { GetModuleHandleExW(0x4 | 0x2, mod_dir as *const () as *const u16, &mut h) } == 0 || h == 0 {
        return None;
    }
    let mut buf = [0u16; 1024];
    let n = unsafe { GetModuleFileNameW(h, buf.as_mut_ptr(), buf.len() as u32) } as usize;
    if n == 0 || n >= buf.len() {
        return None;
    }
    let p = String::from_utf16_lossy(&buf[..n]);
    p.rfind(|c| c == '\\' || c == '/').map(|i| p[..i].to_string())
}
fn now_ms() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0)
}
pub fn log(s: &str) {
    if !LOG_ENABLED {
        return;
    }
    if let Some(d) = mod_dir() {
        if let Ok(mut f) = fs::OpenOptions::new().create(true).append(true).open(format!("{d}\\flow_capture_log.txt")) {
            let _ = writeln!(f, "[{}ms] {}", now_ms(), s);
        }
    }
}
pub fn write_file(name: &str, body: &str) {
    if let Some(d) = mod_dir() {
        let _ = fs::write(format!("{d}\\{name}"), body);
    }
}

// ── JSON 헬퍼 ─────────────────────────────────────────────────────────────
fn ji(v: &Value, k: &str) -> i64 {
    v.get(k).and_then(|x| x.as_i64().or_else(|| x.as_f64().map(|f| f as i64))).unwrap_or(0)
}
fn js(v: &Value, k: &str) -> String {
    v.get(k).map(|x| match x { Value::String(s) => s.clone(), other => other.to_string() }).unwrap_or_default()
}
fn jb(v: &Value, k: &str) -> bool {
    v.get(k).and_then(|x| x.as_bool()).unwrap_or(false)
}
fn jids(v: &Value, k: &str) -> Vec<u64> {
    v.get(k).and_then(|x| x.as_array()).map(|a| a.iter().filter_map(|x| x.as_u64()).collect()).unwrap_or_default()
}
/// {"Normal": 5} → Some(5)
fn slot_team(v: Option<&Value>) -> Option<u64> {
    match v? {
        Value::Object(o) => o.get("Normal").and_then(|x| x.as_u64()),
        Value::Number(n) => n.as_u64(),
        _ => None,
    }
}
fn enum_name(v: Option<&Value>) -> String {
    match v {
        Some(Value::String(s)) => s.clone(),
        Some(Value::Object(o)) => o.keys().next().cloned().unwrap_or_default(),
        _ => String::new(),
    }
}
fn csv(s: &str) -> String {
    s.replace([',', '\n', '\r'], " ")
}

// ── 유저 팀 경기 집합(replay_params.json 용 — 09-25 부터 캡처 게이트엔 안 씀) ───────────────────────────────
static USER_TID: AtomicU64 = AtomicU64::new(u64::MAX);
static USER_MATCHES: RwLock<Option<HashSet<u64>>> = RwLock::new(None);
static MATCH_CHECKED: Mutex<Option<HashSet<usize>>> = Mutex::new(None);
static MATCH_DIAG: AtomicBool = AtomicBool::new(false);
fn match_json(ctx: &StableClient<'_>, mid: u64) -> Option<Value> {
    for k in [RecordKindV1::Match, RecordKindV1::MatchNormal, RecordKindV1::MatchPractice] {
        if let Some(j) = ctx.record_get_json(k, mid as usize, "") {
            if let Ok(v) = serde_json::from_str::<Value>(&j) {
                return Some(v);
            }
        }
    }
    None
}
fn replay_json(ctx: &StableClient<'_>, rid: u64) -> Option<Value> {
    ctx.record_get_json(RecordKindV1::MatchReplay, rid as usize, "").and_then(|j| serde_json::from_str::<Value>(&j).ok())
}
/// 유저 팀 id 갱신 + 새로 생긴 Match 레코드만 검사(한 번에 최대 400건 — 긴 세이브에서도 프레임 부담 분산).
fn refresh_user_matches(ctx: &StableClient<'_>) {
    let Some(tid) = ctx.player_team_id() else { return };
    let tid = tid as u64;
    // ★09-24 실측: 유저 팀 id 가 0 일 수 있다(T1 = team 0, 선수 id 0~4) — 클래식의 "pid 0 = 조합테스트 오염" 가드는 쓰지 않는다.
    if USER_TID.swap(tid, Ordering::Relaxed) != tid {
        *MATCH_CHECKED.lock().unwrap_or_else(|e| e.into_inner()) = None;
        *USER_MATCHES.write().unwrap_or_else(|e| e.into_inner()) = Some(HashSet::new());
    }
    let mut checked = MATCH_CHECKED.lock().unwrap_or_else(|e| e.into_inner());
    let checked = checked.get_or_insert_with(HashSet::new);
    let mut found: Vec<u64> = Vec::new();
    let mut budget = 400usize;
    // ★09-24 실측: Match 종류만 보면 유저 팀 경기가 0건이었다(match 50 = 유저 팀 경기인데 미등록) ⟹ crm 처럼 세 종류를 다 보고,
    //   경로 조회("team1")가 비면 레코드 전체에서 꺼낸다. 첫 호출에 종류별 개수·샘플을 1회 로그.
    let diag = !MATCH_DIAG.swap(true, Ordering::Relaxed);
    for (ki, kind) in [RecordKindV1::Match, RecordKindV1::MatchNormal, RecordKindV1::MatchPractice].into_iter().enumerate() {
        let ids = ctx.record_ids(kind);
        if diag {
            let sample = ids.first().map(|id| {
                let p = ctx.record_get_json(kind, *id, "team1").unwrap_or_else(|| "None".into());
                format!("id={} team1={}", id, p.chars().take(80).collect::<String>())
            });
            log(&format!("user-match diag: kind#{} ids={} sample={:?} tid={}", ki, ids.len(), sample, tid));
        }
        for id in ids {
            let key = ki * 1_000_000_000 + id;
            if checked.contains(&key) {
                continue;
            }
            if budget == 0 {
                break;
            }
            budget -= 1;
            checked.insert(key);
            let get = |k: &str| ctx.record_get_json(kind, id, k).and_then(|j| serde_json::from_str::<Value>(&j).ok());
            let (mut t1, mut t2) = (get("team1"), get("team2"));
            if t1.is_none() && t2.is_none() {
                if let Some(full) = ctx.record_get_json(kind, id, "").and_then(|j| serde_json::from_str::<Value>(&j).ok()) {
                    t1 = full.get("team1").cloned();
                    t2 = full.get("team2").cloned();
                }
            }
            if slot_team(t1.as_ref()) == Some(tid) || slot_team(t2.as_ref()) == Some(tid) {
                found.push(id as u64);
            }
        }
    }
    if !found.is_empty() {
        let mut g = USER_MATCHES.write().unwrap_or_else(|e| e.into_inner());
        let s = g.get_or_insert_with(HashSet::new);
        s.extend(found);
    }
}

// ── flush: 슬롯 → flow 파일 ───────────────────────────────────────────────
static FILES_WRITTEN: AtomicU64 = AtomicU64::new(0);
struct RepAth {
    aid: u64,
    champ: String,
}
struct RepInfo {
    blue_tid: u64,
    red_tid: u64,
    blue: Vec<RepAth>,
    red: Vec<RepAth>,
}
fn parse_rep(v: &Value) -> RepInfo {
    let ath = |k: &str| -> Vec<RepAth> {
        v.get(k)
            .and_then(|x| x.as_array())
            .map(|a| a.iter().map(|p| RepAth { aid: ji(p, "athlete_id") as u64, champ: js(p, "champion").to_ascii_lowercase() }).collect())
            .unwrap_or_default()
    };
    RepInfo { blue_tid: ji(v, "blue_team_id") as u64, red_tid: ji(v, "red_team_id") as u64, blue: ath("blue_team"), red: ath("red_team") }
}

fn flush_slot(ctx: &StableClient<'_>, s: &cap::Slot) {
    let n = s.samples.len();
    if n < 3 {
        return; // 90틱 미만 = 초기화 직후 중단(클래식과 동일)
    }
    // ⛔틱 속도 필터 폐기(09-25): 화면 재생 sim 도 재생보다 앞서 달린다(09-24 리플레이 실측 ≈1170틱/초 · Spectator_Chat 주석 동일)
    //   → 속도로는 선행 sim 과 구분 불가. 같은 세트의 선행/재생 sim 은 seed 폴더 중복 제거로 한 파일만 남는다.
    let (kind, mid, rid, set_idx, seed) = s.key;
    let Some(d) = mod_dir() else { return };
    let dir = format!("{d}\\flow\\{:016x}", seed);
    let _ = fs::create_dir_all(&dir);
    // 레코드 해석: 리플레이(선수 id·팀 id) + 매치(세트·시리즈)
    let mjson = if mid != SimOriginV1::NONE { match_json(ctx, mid) } else { None };
    let mut rjson = if rid != SimOriginV1::NONE { replay_json(ctx, rid) } else { None };
    if rjson.is_none() {
        if let (Some(m), true) = (mjson.as_ref(), set_idx != SimOriginV1::NONE) {
            if let Some(r) = jids(m, "replays").get(set_idx as usize) {
                rjson = replay_json(ctx, *r);
            }
        }
    }
    let rep = rjson.as_ref().map(parse_rep);
    // 리플레이 sim 은 set_index 가 NONE 으로 온다(09-24 실측 set=-1) ⟹ 매치의 replays 목록에서 이 리플레이 위치로 세트 번호를 구한다
    let set_idx = if set_idx == SimOriginV1::NONE && rid != SimOriginV1::NONE {
        mjson.as_ref().and_then(|m| jids(m, "replays").iter().position(|r| *r == rid)).map(|i| i as u64).unwrap_or(set_idx)
    } else {
        set_idx
    };
    let my_tid = ctx.player_team_id().map(|t| t as u64).unwrap_or(u64::MAX);
    let mut out = String::with_capacity(n * 260 + 2048);
    out.push_str(&format!(
        "# tfm2_flow_capture v0.20-stable seed=0x{:x} samples={} kills={} obj={} ce={} captured_t0_ms={} game=0.6.1 origin={}/{}/{}/{}\n",
        seed, n, s.kills.len(), s.objs.len(), s.players.len(), s.t0_ms, kind, mid as i64, rid as i64, set_idx as i64
    ));
    out.push_str(concat!(
        "# T,team0_name,team1_name — 팀 이름(레코드 blue/red_team_id)\n",
        "# R,idx,athlete_id,team,champion,player_name  (골드·딜·탱 컬럼순)\n",
        "# E,idx,team,champion  (좌표/HP/레벨 컬럼 순서 — stable 판은 R 과 같은 순서)\n",
        "# S,tick,score0,score1,gold_x10(R순·누적 획득),(x,y,hp,lv)_x10(E순, 셀=32000),deal_x10(R순),tank_x10(R순, 누적),min_x8(t0[탑,미드,바텀,기타],t1[…]),wave중심점_x12((cx,cy)×6=t0[탑,미드,바텀],t1[…]),jmask\n",
        "# K,tick,killer_team,killer_role,killed_role,assist_n,assist_roles_x4(4294967295=없음) — role: 0Top 1Jg 2Mid 3Bot 4Sup\n",
        "# O,tick,team,type(tower/morgard/serpen),detail(타워라인) — 포탑=잃은 팀(넥서스=detail 빈칸) · 에픽 팀=근접 챔피언 추정(stable API 에 처치 팀 없음) · 30틱 해상도\n",
        "# J,kind(4정글/9곰·이름추정),team,x,y,name — 중립 몹 스폰 좌표\n",
        "# PJ,team,<Strategy JSON> — 팀 전술 원문(클래식 P 행 24B 대체)\n",
        "# U,user_team,user_aids(;구분) — 유저 팀\n",
        "# M,set_no,series_user_wins,series_opp_wins\n"
    ));
    // 선수 id: 리플레이 로스터에서 (진영, 챔피언)으로 매칭 — sim 에는 athlete id 가 없다
    // ⚠선수 id 0 도 유효(09-24 실측 T1 탑) — 못 찾음 = None(-1 로 기록)
    let aid_of = |team: usize, champ: &str| -> Option<u64> {
        let r = rep.as_ref()?;
        let side = if team == 0 { &r.blue } else { &r.red };
        side.iter().find(|a| a.champ == champ.to_ascii_lowercase()).map(|a| a.aid)
    };
    if let Some(r) = rep.as_ref() {
        let uteam = if r.blue_tid == my_tid { Some(0) } else if r.red_tid == my_tid { Some(1) } else { None };
        if let Some(u) = uteam {
            let aids: Vec<String> = (if u == 0 { &r.blue } else { &r.red }).iter().map(|a| a.aid.to_string()).collect();
            out.push_str(&format!("U,{},{}\n", u, aids.join(";")));
        }
        let bn = ctx.team_name(r.blue_tid as usize).unwrap_or_default();
        let rn = ctx.team_name(r.red_tid as usize).unwrap_or_default();
        if !bn.is_empty() || !rn.is_empty() {
            out.push_str(&format!("T,{},{}\n", csv(&bn), csv(&rn)));
        }
    }
    // M: 세트 번호 + 이 세트 이전까지의 시리즈 스코어(유저 기준)
    {
        let set_no = if set_idx != SimOriginV1::NONE { set_idx + 1 } else { 1 };
        let (mut uw, mut ow) = (0u32, 0u32);
        if let Some(m) = mjson.as_ref() {
            for r in jids(m, "replays").iter().take(set_no.saturating_sub(1) as usize) {
                if let Some(v) = replay_json(ctx, *r) {
                    let blue_win = jb(&v, "blue_team_win");
                    let user_blue = ji(&v, "blue_team_id") as u64 == my_tid;
                    if blue_win == user_blue { uw += 1; } else { ow += 1; }
                }
            }
        }
        out.push_str(&format!("M,{},{},{}\n", set_no, uw, ow));
    }
    for (i, p) in s.players.iter().enumerate() {
        let aid = aid_of(p.team, &p.champ);
        let pname = aid.and_then(|a| ctx.athlete_name(a as usize)).unwrap_or_default();
        out.push_str(&format!("R,{},{},{},{},{}\n", i, aid.map(|a| a as i64).unwrap_or(-1), p.team, p.champ, csv(&pname)));
    }
    for (i, p) in s.players.iter().enumerate() {
        out.push_str(&format!("E,{},{},{}\n", i, p.team, p.champ));
    }
    for sm in &s.samples {
        out.push_str(&format!("S,{},{},{}", sm.tick, sm.sc[0], sm.sc[1]));
        for i in 0..cap::NATH { out.push_str(&format!(",{}", sm.gold[i])); }
        for i in 0..cap::NATH {
            let (x, y, hp, lv) = sm.pos[i];
            out.push_str(&format!(",{},{},{},{}", x, y, hp, lv));
        }
        for i in 0..cap::NATH { out.push_str(&format!(",{}", sm.deal[i])); }
        for i in 0..cap::NATH { out.push_str(&format!(",{}", sm.tank[i])); }
        for c in 0..8 { out.push_str(&format!(",{}", sm.minc[c])); }
        for c in 0..6usize {
            let cnt = sm.minc[(c / 3) * 4 + (c % 3)] as i64;
            let (cx, cy) = if cnt > 0 { (sm.wave[c].0 / cnt, sm.wave[c].1 / cnt) } else { (0, 0) };
            out.push_str(&format!(",{},{}", cx, cy));
        }
        out.push_str(&format!(",{}\n", sm.jmask));
    }
    for k in &s.kills {
        out.push_str(&format!("K,{}", k[0]));
        for w in &k[1..9] { out.push_str(&format!(",{}", w)); }
        out.push('\n');
    }
    for (t, tm, ty, det) in &s.objs {
        out.push_str(&format!("O,{},{},{},{}\n", t, tm, ty, det));
    }
    for (tm, st) in s.strat.iter().enumerate() {
        if let Some(j) = st {
            out.push_str(&format!("PJ,{},{}\n", tm, j.replace(['\n', '\r'], " ")));
        }
    }
    for (kd, tm, x, y, name) in &s.neut {
        out.push_str(&format!("J,{},{},{},{},{}\n", kd, *tm as i64, x, y, csv(name))); // 중립 팀 = usize::MAX → -1
    }
    // 세트당 1파일: 같은 seed 폴더에서 더 완전한 파일 하나만 남긴다.
    //   ★09-24: 판정 = 헤더 samples 수(많은 쪽) · 같으면 **새 파일**. 구 규칙(바이트 크기)은 같은 리플레이를 다시 돌렸을 때
    //   내용이 나아진 새 파일(모르가드 O 행 추가)이 행 포맷 변화로 몇 바이트 작다는 이유로 버려졌다.
    let prefix = format!("f_{:016x}_", seed);
    let mut skip = false;
    let mut dups = Vec::new();
    let samples_of = |p: &std::path::Path| -> usize {
        let head = fs::read(p).ok().map(|b| String::from_utf8_lossy(&b[..b.len().min(400)]).to_string()).unwrap_or_default();
        head.split("samples=").nth(1).and_then(|r| r.split_whitespace().next()).and_then(|v| v.parse().ok()).unwrap_or(0)
    };
    if let Ok(rd) = fs::read_dir(&dir) {
        for e in rd.flatten() {
            let fname = e.file_name().to_string_lossy().to_string();
            if !fname.starts_with(&prefix) {
                continue;
            }
            if samples_of(&e.path()) > n { skip = true; } else { dups.push(e.path()); }
        }
    }
    if !skip {
        for p in dups { let _ = fs::remove_file(p); }
        let path = format!("{dir}\\{prefix}{}.txt", now_ms());
        if fs::write(&path, &out).is_ok() {
            FILES_WRITTEN.fetch_add(1, Ordering::Relaxed);
        }
    }
    log(&format!("flush kind={} match={} replay={} set={} seed=0x{:x} n={} kills={} obj={} rep={} skip_dup={} tps={:.0} 엔티티이름={:?}", kind, mid as i64, rid as i64, set_idx as i64, seed, n, s.kills.len(), s.objs.len(), rep.is_some(), skip, cap::ticks_per_sec(s), s.names_seen));
}

// ── replay_params.json(유저 팀 경기 리플레이만 — 클래식은 세이브 전 리플레이 11k 건을 30초마다 Debug 덤프) ──
fn dump_replays(ctx: &StableClient<'_>) {
    let mids: Vec<u64> = USER_MATCHES.read().ok().and_then(|g| g.as_ref().map(|s| s.iter().copied().collect())).unwrap_or_default();
    let mut items: Vec<String> = Vec::new();
    let mut mids = mids;
    mids.sort_unstable();
    for mid in mids.iter().rev().take(40) {
        let Some(m) = match_json(ctx, *mid) else { continue };
        for (set_idx, rid) in jids(&m, "replays").iter().enumerate() {
            let Some(v) = replay_json(ctx, *rid) else { continue };
            let side = |k: &str| -> String {
                v.get(k).and_then(|x| x.as_array()).map(|a| a.iter().map(|p| {
                    let aid = ji(p, "athlete_id");
                    let name = ctx.athlete_name(aid as usize).unwrap_or_default();
                    format!("{{\"aid\":{},\"name\":{},\"pos\":{},\"champ\":{},\"items\":{}}}", aid, Value::String(name), Value::String(enum_name(p.get("position"))), Value::String(js(p, "champion")), p.get("items").cloned().unwrap_or(Value::Null))
                }).collect::<Vec<_>>().join(",")).unwrap_or_default()
            };
            let bt = ctx.team_name(ji(&v, "blue_team_id") as usize).unwrap_or_default();
            let rt = ctx.team_name(ji(&v, "red_team_id") as usize).unwrap_or_default();
            items.push(format!(
                "{{\"match_id\":{},\"set\":{},\"replay_id\":{},\"seed\":{},\"blue_team\":{},\"red_team\":{},\"blue_win\":{},\"blue_strategy\":{},\"red_strategy\":{},\"blue\":[{}],\"red\":[{}]}}",
                mid, set_idx, rid, v.get("seed").cloned().unwrap_or(Value::Null), Value::String(bt), Value::String(rt), jb(&v, "blue_team_win"),
                v.get("blue_strategy").cloned().unwrap_or(Value::Null), v.get("red_strategy").cloned().unwrap_or(Value::Null), side("blue_team"), side("red_team")
            ));
        }
    }
    write_file("replay_params.json", &format!("[{}]", items.join(",")));
}

// 자동정리: 파일 2500 초과 시 작은 것(미완주)부터 지워 2000 으로(클래식과 동일)
fn prune_flow_dir() {
    let Some(d) = mod_dir() else { return };
    let Ok(rd) = fs::read_dir(format!("{d}\\flow")) else { return };
    let mut files: Vec<(std::path::PathBuf, u64)> = Vec::new();
    for sub in rd.flatten() {
        if let Ok(rd2) = fs::read_dir(sub.path()) {
            for e in rd2.flatten() {
                files.push((e.path(), e.metadata().map(|m| m.len()).unwrap_or(0)));
            }
        }
    }
    if files.len() <= 2500 {
        return;
    }
    files.sort_by_key(|f| f.1);
    let drop = files.len() - 2000;
    for (f, _) in files.iter().take(drop) {
        let _ = fs::remove_file(f);
    }
}

fn write_status() {
    let um = USER_MATCHES.read().ok().and_then(|g| g.as_ref().map(|s| s.len())).unwrap_or(0);
    let s = format!(
        "tfm2_flow_capture 0.8(stable) 상태 [{}ms]\n유저 팀 id: {} · 유저 팀 경기 {}건\n캡처 틱(샘플 판정) 발화: {} · 샘플: {}\n파일 저장: {} · 슬롯 만석 드롭: {}\n활성 슬롯:\n{}",
        now_ms(),
        USER_TID.load(Ordering::Relaxed) as i64,
        um,
        cap::HITS.load(Ordering::Relaxed),
        cap::SAMPLES.load(Ordering::Relaxed),
        FILES_WRITTEN.load(Ordering::Relaxed),
        cap::DROPPED_FULL.load(Ordering::Relaxed),
        cap::active_summary()
    );
    write_file("flow_status.txt", &s);
}

static FRAME: AtomicU64 = AtomicU64::new(0);
static STARTED: AtomicBool = AtomicBool::new(false);
struct Ext;
impl StableExtension for Ext {
    fn post_update(&self, ctx: &mut StableClient<'_>, _dt: u64) {
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let f = FRAME.fetch_add(1, Ordering::Relaxed);
            if !STARTED.swap(true, Ordering::Relaxed) {
                log("start 0.8.0 (stable)");
            }
            let in_game = ctx.scene_kind() == Some(SceneKindV1::InGame);
            if in_game && f % 120 == 0 {
                refresh_user_matches(ctx);
            }
            if f % 30 == 15 {
                for s in cap::take_ready(3) {
                    flush_slot(ctx, &s);
                }
            }
            if in_game && f % 15 == 7 {
                bp::tick(ctx);
            }
            if in_game && f % 3600 == 1800 {
                dump_replays(ctx);
            }
            if f % 300 == 299 {
                write_status();
            }
            if f % 18000 == 17999 {
                prune_flow_dir();
            }
        }));
    }
}

fn init(host: &StableHost) -> StableMod {
    let v = host.game_version();
    host.log(LogLevel::Info, "tfm2_flow_capture 0.8 (stable)");
    log(&format!("INIT game {}.{}.{} host_abi={}", v.major, v.minor, v.patch, host.abi_level()));
    let mut d = StableMod::new(MOD_ID);
    d.set_extension(Ext);
    d.set_match_hook(cap::Hook);
    d
}
declare_stable_mod!(init);
