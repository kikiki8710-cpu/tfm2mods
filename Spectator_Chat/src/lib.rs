//! Spectator_Chat — 관전/리플레이 화면 가짜 관중 채팅 (0.6.0 stable 재작성 2026-09-17).
//! 원작(클래식 0.5.8, `C:\tfm2mods\Spectator_Chat`): `Scene::InGame{data}.db()` 의 이벤트 스트림(`GameFrameData.events`)을
//!   raw 오프셋으로 읽어 `{:?}` 문자열 파싱 → 킬/CC/딸피/궁 인덱스 → 재생 tick 에 맞춰 표시. 패널 = ingame.ui 전체 오버라이드 + DraggablePopup raw write.
//! stable 판(이벤트 스트림·Scene 접근 없음 → 설계 변경):
//!   · **이벤트 = `StableMatchHook`(sim 안에서 매 tick)** — `sim_origin().kind` 가 ClientMatchView/ClientSpectate/ClientReplay 인 sim 만 추적
//!     (리플레이도 시드 재시뮬레이션이라 훅이 돈다). kill_log 델타·hp 20% 진입·is_alive 하강·cc_at(에어본/스턴/속박)·궁 쿨다운 상승·타워 수·골드 합.
//!   · **재생 커서 = 화면 `game_time.value` 라벨("MM:SS")** ×30 + 라벨 변경 후 경과 ms 보간. (sim 은 재생보다 앞서 달린다 → tick 게이트로 표시.)
//!   · 패널 = `ingame` 루트에 `sc_panel` 스폰(배경·헤더·줄 라벨 20). 드래그/리사이즈 = Win32 커서·좌클릭 폴링 → `ui_set_properties(x/y/width/height)`.
//!   · 문구/닉/메시지 생성 = 원작 그대로(`phrases.rs`, `chat_lines.txt` 핫리로드).
//! 설정 = mods\Spectator_Chat\Spectator_Chat.cfg (x,y,w,h,visible).
#![allow(dead_code)]
use mod_api_stable::{declare_stable_mod, LogLevel, SimOriginKindV1, StableClient, StableExtension, StableHost, StableMatchHook, StableMod, StableSim};
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicI64, AtomicU64, Ordering};
use std::sync::Mutex;
#[path = r"C:\tfm2mods\ui_kit\ui_kit_stable.rs"]
mod uk;
mod phrases;
use phrases::*;

const MOD_ID: &str = "Spectator_Chat";
const DBG: bool = false; // 09-20 인게임 검증 후 OFF(진단 시 true)
const ROOT: &str = "ingame";
const PANEL: &str = "ingame.sc_panel";
const LINE_SLOTS: usize = 20;
const LINE_H: f32 = 26.0;
const LINE_GAP: f32 = 2.0;
const HEADER_H: f32 = 22.0;
const PAD: f32 = 8.0;
const WIN_MIN_W: f32 = 300.0;
const WIN_MIN_H: f32 = 120.0;
const WINDOW_TICKS: i64 = 600;
const SPREAD: i64 = 15;
const TICKS_PER_SEC: i64 = 30;

// ───────── 설정 ─────────
#[derive(Clone, Copy)]
struct Cfg { x: f32, y: f32, w: f32, h: f32, visible: bool }
static CFG: Mutex<Cfg> = Mutex::new(Cfg { x: 1490.0, y: 120.0, w: 414.0, h: 214.0, visible: true });
fn cfg() -> Cfg { *CFG.lock().unwrap_or_else(|e| e.into_inner()) }
fn cfg_set(f: impl FnOnce(&mut Cfg)) { let mut g = CFG.lock().unwrap_or_else(|e| e.into_inner()); f(&mut g); }
fn cfg_save() { let c = cfg(); if let Some(d) = mod_dir() { let _ = std::fs::write(format!(r"{}\Spectator_Chat.cfg", d), format!("x={}\ny={}\nw={}\nh={}\nvisible={}\n", c.x, c.y, c.w, c.h, c.visible)); } }
fn cfg_load() { if let Some(t) = mod_dir().and_then(|d| std::fs::read_to_string(format!(r"{}\Spectator_Chat.cfg", d)).ok()) { cfg_set(|c| { for l in t.lines() { if let Some((k, v)) = l.split_once('=') { let v = v.trim(); match k.trim() { "x" => c.x = v.parse().unwrap_or(c.x), "y" => c.y = v.parse().unwrap_or(c.y), "w" => c.w = v.parse().unwrap_or(c.w), "h" => c.h = v.parse().unwrap_or(c.h), "visible" => c.visible = v != "false", _ => {} } } } }); } }

// ───────── 공통 ─────────
static FRAME: AtomicU64 = AtomicU64::new(0);
static ACTIVE: AtomicBool = AtomicBool::new(false);
static CLICKS_REGISTERED: AtomicBool = AtomicBool::new(false);
#[link(name = "kernel32")]
extern "system" { fn GetModuleFileNameW(module: usize, buf: *mut u16, size: u32) -> u32; fn GetCurrentProcessId() -> u32; fn GetModuleHandleExW(flags: u32, addr: *const u16, h: *mut usize) -> i32; }
#[link(name = "user32")]
extern "system" { fn GetCursorPos(p: *mut [i32; 2]) -> i32; fn GetForegroundWindow() -> usize; fn GetWindowThreadProcessId(h: usize, pid: *mut u32) -> u32; fn ScreenToClient(h: usize, p: *mut [i32; 2]) -> i32; fn GetClientRect(h: usize, r: *mut [i32; 4]) -> i32; fn GetAsyncKeyState(vk: i32) -> i16; }
pub fn mod_dir() -> Option<String> {
    let mut h: usize = 0;
    if unsafe { GetModuleHandleExW(0x4 | 0x2, mod_dir as *const () as *const u16, &mut h) } == 0 || h == 0 { return None; }
    let mut buf = [0u16; 1024];
    let n = unsafe { GetModuleFileNameW(h, buf.as_mut_ptr(), buf.len() as u32) } as usize;
    if n == 0 || n >= buf.len() { return None; }
    let p = String::from_utf16_lossy(&buf[..n]);
    p.rfind(|c| c == '\\' || c == '/').map(|i| p[..i].to_string())
}
fn log(s: &str) {
    if !DBG { return; }
    if let Some(d) = mod_dir() { use std::io::Write; if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(format!(r"{}\debug.log", d)) { let _ = writeln!(f, "[f{}] {}", FRAME.load(Ordering::Relaxed), s); } }
}
fn cursor_ui() -> Option<(f32, f32)> {
    unsafe {
        let h = GetForegroundWindow(); if h == 0 { return None; }
        let mut pid = 0u32; GetWindowThreadProcessId(h, &mut pid); if pid != GetCurrentProcessId() { return None; }
        let mut p = [0i32; 2]; if GetCursorPos(&mut p) == 0 || ScreenToClient(h, &mut p) == 0 { return None; }
        let mut r = [0i32; 4]; if GetClientRect(h, &mut r) == 0 || r[2] <= 0 || r[3] <= 0 { return None; }
        Some((p[0] as f32 * 1920.0 / r[2] as f32, p[1] as f32 * 1080.0 / r[3] as f32))
    }
}
fn lbutton() -> bool { unsafe { (GetAsyncKeyState(0x01) as u16 & 0x8000) != 0 } }

// ───────── sim 측 원시 이벤트 ─────────
#[derive(Clone, Debug)]
enum Ev { Kill { killer_pid: usize, killed_pid: usize, assists: usize }, Danger(usize), Death(usize), Cc(usize, u8), Ult(usize), Tower(usize), Serpen(usize), Gold(i64, i64) }
#[derive(Default)]
struct Raw {
    origin: Option<(u32, u64, u64, u64)>,
    /// pid → (champion id, team, lane)
    players: HashMap<usize, (String, usize, u32)>,
    events: Vec<(i64, Ev)>,
    gen: u64,
    // 델타 상태
    kill_seen: usize,
    hp_danger: HashMap<usize, bool>,
    alive: HashMap<usize, bool>,
    cc_seen: HashMap<usize, usize>,
    ult_cd: HashMap<usize, usize>,
    towers: HashMap<usize, usize>, // entity id → team
    serpens: HashMap<usize, (u64, u64)>,
    last_gold_tick: i64,
}
static RAW: Mutex<Option<Raw>> = Mutex::new(None);
fn origin_key(sim: &StableSim<'_>) -> Option<(u32, u64, u64, u64)> {
    let o = sim.sim_origin()?;
    let kind = o.kind;
    if kind != SimOriginKindV1::ClientMatchView as u32 && kind != SimOriginKindV1::ClientSpectate as u32 && kind != SimOriginKindV1::ClientReplay as u32 { return None; }
    Some((kind, o.match_id, o.replay_id, o.set_index))
}
struct Hook;
impl StableMatchHook for Hook {
    fn on_match_start(&self, sim: &mut StableSim<'_>) {
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let Some(key) = origin_key(sim) else { return };
            let mut g = RAW.lock().unwrap_or_else(|e| e.into_inner());
            let gen = g.as_ref().map(|r| r.gen + 1).unwrap_or(1);
            let mut r = Raw { origin: Some(key), gen, ..Default::default() };
            for i in 0..sim.player_count() {
                let Some(p) = sim.player_at(i) else { continue };
                let champ = p.champion().and_then(|e| e.name()).unwrap_or_default();
                r.players.insert(p.id(), (champ, p.team(), p.lane().map(|l| l as u32).unwrap_or(9)));
                r.alive.insert(p.id(), true);
                r.ult_cd.insert(p.id(), p.cooldowns().map(|c| c.3).unwrap_or(0));
            }
            for i in 0..sim.tower_count() { let id = sim.tower_id_at(i); if let Some(e) = sim.get_entity(id) { r.towers.insert(id, e.team()); } }
            log(&format!("[sim] start origin={:?} gen={} players={:?}", key, gen, r.players));
            *g = Some(r);
        }));
    }
    fn on_match_tick(&self, sim: &mut StableSim<'_>, _seed: u64) {
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let Some(key) = origin_key(sim) else { return };
            let mut g = RAW.lock().unwrap_or_else(|e| e.into_inner());
            let Some(r) = g.as_mut() else { return };
            if r.origin != Some(key) { return; }
            let t = sim.tick() as i64;
            // 킬 로그 델타: 포지션+팀 → pid
            let kc = sim.kill_log_count();
            while r.kill_seen < kc {
                if let Some(k) = sim.kill_log_at(r.kill_seen) {
                    let find = |team: usize, pos: u32| r.players.iter().find(|(_, (_, tm, ln))| *tm == team && *ln == pos).map(|(pid, _)| *pid);
                    let killer = find(k.killer_team, k.killer_position);
                    let killed = find(1 - k.killer_team.min(1), k.killed_position);
                    if let (Some(a), Some(b)) = (killer, killed) { r.events.push((k.tick as i64, Ev::Kill { killer_pid: a, killed_pid: b, assists: k.assist_count as usize })); }
                }
                r.kill_seen += 1;
            }
            let pids: Vec<usize> = r.players.keys().copied().collect();
            for pid in pids {
                let Some(p) = sim.get_player(pid) else { continue };
                // ★09-20 실측(crm 과 동일): on_match_start 엔 챔피언 엔티티가 아직 없어 이름이 "" → 첫 tick 들에서 지연 채움(빈 이름이면 채팅에 챔피언명이 안 나온다)
                if r.players.get(&pid).map(|x| x.0.is_empty()).unwrap_or(false) {
                    if let Some(n) = p.champion().and_then(|e| e.name()) { if !n.is_empty() { if let Some(x) = r.players.get_mut(&pid) { x.0 = n; } } }
                }
                let alive = p.is_alive();
                let was = r.alive.insert(pid, alive).unwrap_or(true);
                if was && !alive { r.events.push((t, Ev::Death(pid))); }
                if let Some(e) = p.champion() {
                    let (hp, mx) = e.hp();
                    let low = alive && mx > 0 && hp > 0 && hp * 5 <= mx;
                    let in_d = r.hp_danger.entry(pid).or_insert(false);
                    if low && !*in_d { *in_d = true; r.events.push((t, Ev::Danger(pid))); } else if !low && hp * 5 > mx { *in_d = false; }
                    let cc_n = e.cc_count();
                    let seen = r.cc_seen.entry(pid).or_insert(0);
                    if cc_n > *seen { for i in *seen..cc_n { if let Some(c) = e.cc_at(i) { if c.kind <= 2 { r.events.push((t, Ev::Cc(pid, c.kind as u8))); } } } }
                    *seen = cc_n;
                }
                if let Some(cd) = p.cooldowns() { let prev = r.ult_cd.insert(pid, cd.3).unwrap_or(0); if prev == 0 && cd.3 > 30 { r.events.push((t, Ev::Ult(pid))); } }
            }
            if t % 30 == 0 {
                // 타워
                let mut now: HashMap<usize, usize> = HashMap::new();
                for i in 0..sim.tower_count() { let id = sim.tower_id_at(i); if let Some(e) = sim.get_entity(id) { if e.is_alive() { now.insert(id, e.team()); } } }
                let gone: Vec<usize> = r.towers.iter().filter(|(id, _)| !now.contains_key(id)).map(|(_, tm)| 1 - (*tm).min(1)).collect();
                for killer_team in gone { r.events.push((t, Ev::Tower(killer_team))); }
                r.towers = now;
                // 세르펜(이름에 serpen)
                let mut cur: HashMap<usize, (u64, u64)> = HashMap::new();
                for i in 0..sim.entity_count() { if let Some(e) = sim.entity_at(i) { if e.is_alive() && !e.is_champion() && !e.is_tower() && !e.is_minion() { if e.name().map(|n| n.to_ascii_lowercase().contains("serpen")).unwrap_or(false) { cur.insert(e.id(), e.pos()); } } } }
                for (id, pos) in r.serpens.iter() {
                    if !cur.contains_key(id) {
                        // 가장 가까운 살아있는 챔피언의 팀에게 크레딧
                        let mut best: Option<(u64, usize)> = None;
                        for i in 0..sim.player_count() { if let Some(p) = sim.player_at(i) { if let Some(e) = p.champion() { if e.is_alive() { let (x, y) = e.pos(); let d = (x as i64 - pos.0 as i64).pow(2) as u64 + (y as i64 - pos.1 as i64).pow(2) as u64; if best.map(|b| d < b.0).unwrap_or(true) { best = Some((d, p.team())); } } } } }
                        if let Some((_, tm)) = best { r.events.push((t, Ev::Serpen(tm))); }
                    }
                }
                r.serpens = cur;
            }
            if t - r.last_gold_tick >= 300 {
                r.last_gold_tick = t;
                let (mut gb, mut gr) = (0i64, 0i64);
                for i in 0..sim.player_count() { if let Some(p) = sim.player_at(i) { if p.team() == 0 { gb += p.gold() as i64; } else { gr += p.gold() as i64; } } }
                r.events.push((t, Ev::Gold(gb, gr)));
            }
        }));
    }
}

// ───────── 클라: 인덱스·표시 ─────────
struct Indexed { tick: i64, text: String }
static INDEX: Mutex<Vec<Indexed>> = Mutex::new(Vec::new());
static BUILT_GEN: AtomicU64 = AtomicU64::new(0);
static BUILT_N: AtomicU64 = AtomicU64::new(0);
static NAMES: Mutex<Option<HashMap<usize, String>>> = Mutex::new(None); // pid → 표시명(챔피언)
static CHEER: Mutex<Vec<String>> = Mutex::new(Vec::new());
static PLAYED_BASE: AtomicI64 = AtomicI64::new(-1);
static PLAYED_MS: AtomicU64 = AtomicU64::new(0);
static LAST_LABEL: Mutex<String> = Mutex::new(String::new());
static VIS_LINES: AtomicU64 = AtomicU64::new(6);
static DRAG: Mutex<Option<(u8, f32, f32, f32, f32, f32, f32)>> = Mutex::new(None); // (mode 1=drag 2=resize, mouse x,y, x,y,w,h at start)
static LAST_PRESS: AtomicBool = AtomicBool::new(false);
static LAST_BUF: Mutex<Vec<String>> = Mutex::new(Vec::new());

fn parse_game_time(s: &str) -> Option<i64> { let (m, sec) = s.trim().split_once(':')?; Some((m.trim().parse::<i64>().ok()? * 60 + sec.trim().parse::<i64>().ok()?) * TICKS_PER_SEC) }
fn lines_for_height(h: f32) -> usize { (((h - HEADER_H - PAD * 2.0) / (LINE_H + LINE_GAP)).floor() as usize).clamp(1, LINE_SLOTS) }
fn champ_display(ctx: &StableClient<'_>, id: &str) -> String { ctx.i18n(&format!("#asset/base/text/champion?description.{}.name", id)).filter(|s| !s.is_empty()).unwrap_or_else(|| id.to_string()) }

/// RAW 이벤트 → 채팅 인덱스(원작 REBUILD 로직 축약). 과거(tick<=played)는 보존, 미래만 교체.
fn rebuild(ctx: &StableClient<'_>, played: i64) {
    let (events, players, gen) = { let g = RAW.lock().unwrap_or_else(|e| e.into_inner()); let Some(r) = g.as_ref() else { return }; (r.events.clone(), r.players.clone(), r.gen) };
    let mut nm = NAMES.lock().unwrap_or_else(|e| e.into_inner());
    // 이름표가 없거나, 지연 채움 전(빈 챔피언 id)에 만들어졌으면 다시 만든다
    let stale = nm.as_ref().map(|m| m.iter().any(|(pid, v)| v.is_empty() && players.get(pid).map(|p| !p.0.is_empty()).unwrap_or(false))).unwrap_or(true);
    if stale {
        if nm.is_some() { INDEX.lock().unwrap_or_else(|e| e.into_inner()).clear(); } // 빈 이름으로 만들어진 과거 줄까지 다시 생성
        let m: HashMap<usize, String> = players.iter().map(|(pid, (c, _, _))| (*pid, if c.is_empty() { String::new() } else { champ_display(ctx, c) })).collect(); *nm = Some(m);
    }
    let names_map = nm.clone().unwrap_or_default();
    drop(nm);
    let name = |pid: &usize| names_map.get(pid).cloned().unwrap_or_default();
    let team_of = |pid: &usize| players.get(pid).map(|p| p.1 as i64).unwrap_or(0);
    let names: Vec<String> = names_map.values().cloned().collect();
    let mut nidx: Vec<Indexed> = Vec::new();
    let mut kills_b = 0i64; let mut kills_r = 0i64;
    let mut scoreline: Vec<(i64, i64, i64)> = Vec::new();
    let mut danger: Vec<(i64, usize)> = Vec::new(); let mut deaths: Vec<(i64, usize)> = Vec::new(); let mut cc: Vec<(i64, usize, u8)> = Vec::new();
    let mut last_gold_t = -100000i64;
    let mut ult_cnt = 0usize;
    for (t, ev) in &events {
        match ev {
            Ev::Kill { killer_pid, killed_pid, assists } => {
                if team_of(killer_pid) == 0 { kills_b += 1; } else { kills_r += 1; }
                scoreline.push((*t, kills_b, kills_r));
                let cnt = 2 + rng() % 3; for j in 0..cnt { nidx.push(Indexed { tick: t + j as i64 * SPREAD, text: msg_kill(&name(killer_pid), &name(killed_pid), *assists) }); }
                if (kills_b + kills_r) % 5 == 0 { nidx.push(Indexed { tick: t + 3 * SPREAD, text: msg_score(kills_b, kills_r) }); }
            }
            Ev::Danger(pid) => danger.push((*t, *pid)),
            Ev::Death(pid) => deaths.push((*t, *pid)),
            Ev::Cc(pid, k) => cc.push((*t, *pid, *k)),
            Ev::Ult(pid) => { ult_cnt += 1; if rng() % 100 < 40 { nidx.push(Indexed { tick: *t, text: msg_ult(&name(pid)) }); } }
            Ev::Tower(team) => { let cnt = 1 + rng() % 2; for j in 0..cnt { nidx.push(Indexed { tick: t + j as i64 * SPREAD, text: msg_tower(*team as i64) }); } }
            Ev::Serpen(team) => { let cnt = 2 + rng() % 3; for j in 0..cnt { nidx.push(Indexed { tick: t + j as i64 * SPREAD, text: msg_serpen(*team as i64) }); } }
            Ev::Gold(gb, gr) => { let gd = gb - gr; if gd.abs() >= 3000 && t - last_gold_t >= 1800 { let cnt = 1 + rng() % 2; for j in 0..cnt { nidx.push(Indexed { tick: t + j as i64 * SPREAD, text: msg_lead(if gd > 0 { 0 } else { 1 }, false) }); } last_gold_t = *t; } }
        }
    }
    // 생존: 위험 진입 후 600틱 내 죽음 없음
    let mut last_surv: HashMap<usize, i64> = HashMap::new();
    for (dt, pid) in &danger {
        if deaths.iter().any(|(kt, kid)| kid == pid && *kt >= *dt && *kt <= dt + 600) { continue; }
        if last_surv.get(pid).map(|l| dt - l < 600).unwrap_or(false) { continue; }
        last_surv.insert(*pid, *dt);
        let cnt = 2 + rng() % 3; for j in 0..cnt { nidx.push(Indexed { tick: dt + 600 + j as i64 * SPREAD, text: msg_survive(&name(pid)) }); }
    }
    // N인 CC(같은 팀 2명 이상 30틱 내)
    cc.sort_by_key(|x| x.0);
    let mut used = vec![false; cc.len()];
    for i in 0..cc.len() {
        if used[i] { continue; }
        let (t0, p0, k0) = cc[i]; let team0 = team_of(&p0); let mut ids = vec![p0]; used[i] = true;
        for j in (i + 1)..cc.len() { if used[j] { continue; } if cc[j].0 > t0 + 30 { break; } if team_of(&cc[j].1) == team0 && !ids.contains(&cc[j].1) { ids.push(cc[j].1); used[j] = true; } }
        if ids.len() >= 2 { let kind = match k0 { 1 => "스턴", 0 => "에어본", _ => "속박" }; let cnt = 2 + rng() % 2; for jj in 0..cnt { let who = name(&ids[rng() % ids.len()]); nidx.push(Indexed { tick: t0 + jj as i64 * SPREAD, text: msg_cc(ids.len(), kind, jj % 2 == 0, &who) }); } }
    }
    nidx.sort_by_key(|m| m.tick);
    // 잡담·응원
    let score_at = |tk: i64| -> (i64, i64) { let mut br = (0, 0); for &(st, sb, sr) in &scoreline { if st <= tk { br = (sb, sr); } else { break; } } br };
    let mut amb: Vec<Indexed> = Vec::new();
    const AMBIENT_GAP: i64 = 300; const AMBIENT_STEP: i64 = 480;
    if !nidx.is_empty() {
        for w in 0..nidx.len() - 1 { let (a, b) = (nidx[w].tick, nidx[w + 1].tick); if b - a >= AMBIENT_GAP { let mut tk = a + AMBIENT_STEP; while tk < b - SPREAD { let (sb, sr) = score_at(tk); amb.push(Indexed { tick: tk, text: msg_ambient(&names, sb, sr) }); tk += AMBIENT_STEP; } } }
        let mut tk = nidx[0].tick - AMBIENT_STEP; while tk > 30 { amb.push(Indexed { tick: tk, text: msg_cheer(&names) }); tk -= AMBIENT_STEP; }
    } else {
        let mut tk = 60; while tk < 3600 { amb.push(Indexed { tick: tk, text: msg_cheer(&names) }); tk += AMBIENT_STEP; }
    }
    nidx.extend(amb); nidx.sort_by_key(|m| m.tick);
    let mut prev = i64::MIN; for m in nidx.iter_mut() { if m.tick < prev + SPREAD { m.tick = prev + SPREAD; } prev = m.tick; }
    let mut index = INDEX.lock().unwrap_or_else(|e| e.into_inner());
    index.retain(|m| m.tick <= played);
    index.extend(nidx.into_iter().filter(|m| m.tick > played));
    index.sort_by_key(|m| m.tick);
    BUILT_GEN.store(gen, Ordering::Relaxed); BUILT_N.store(events.len() as u64, Ordering::Relaxed);
    log(&format!("rebuild gen={} events={} index={} played={} ult={}", gen, events.len(), index.len(), played, ult_cnt));
}

fn panel_src(c: &Cfg) -> String {
    let mut s = format!("sc_panel:empty {{ x: {}px; y: {}px; width: {}px; height: {}px; z: 800; visible: {};\n", c.x as i32, c.y as i32, c.w as i32, c.h as i32, c.visible);
    s.push_str("  #bg:color { ignore_event: true; width: 100%; height: 100%; color: #0f1016d9; rounding: Uniform { rounding: 8; } }\n");
    s.push_str(&format!("  #header:color {{ ignore_event: true; width: 100%; height: {}px; color: #161721ff; rounding: Uniform {{ rounding: 8; }} }}\n", HEADER_H as i32));
    s.push_str(&format!("  #title:label {{ @\"asset/base/style/main#bold_label\"; ignore_event: true; x: 10px; y: 0px; width: 200px; height: {}px; size: 13; color: #a3a9b6ff; align_y: Center; text: \"관중 채팅\"; }}\n", HEADER_H as i32));
    s.push_str(&format!("  #grip:label {{ @\"asset/base/style/main#label\"; ignore_event: true; anchor_x: 1; pivot_x: 1; anchor_y: 1; pivot_y: 1; x: -4px; y: -2px; width: 16px; height: 16px; size: 12; color: #6a7086ff; align_x: Center; align_y: Center; text: \"◢\"; }}\n"));
    for i in 0..LINE_SLOTS { s.push_str(&format!("  #l{}:label {{ @\"asset/base/style/main#label\"; ignore_event: true; x: {}px; y: {}px; width: {}px; height: {}px; size: 15; color: #e8e8e8ff; align_y: Center; visible: false; text: \"\"; }}\n", i, PAD as i32, (HEADER_H + PAD + i as f32 * (LINE_H + LINE_GAP)) as i32, (c.w - PAD * 2.0) as i32, LINE_H as i32)); }
    s.push('}'); s
}
fn apply_geom(ctx: &mut StableClient<'_>) {
    let c = cfg();
    ctx.ui_set_properties(PANEL, &format!("x: {}px; y: {}px; width: {}px; height: {}px;", c.x as i32, c.y as i32, c.w as i32, c.h as i32));
    for i in 0..LINE_SLOTS { ctx.ui_set_properties(&format!("{}.l{}", PANEL, i), &format!("width: {}px;", (c.w - PAD * 2.0) as i32)); }
    VIS_LINES.store(lines_for_height(c.h) as u64, Ordering::Relaxed);
}

struct Ext;
impl StableExtension for Ext {
    fn post_update(&self, ctx: &mut StableClient<'_>, dt: u64) {
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let f = FRAME.fetch_add(1, Ordering::Relaxed);
            uk::frame_begin();
            if !ctx.ui_exists(ROOT) || !ctx.ui_visible(ROOT).unwrap_or(false) { deactivate(); return; }
            // 재생 커서: game_time.value
            let gt = uk::find_or_rebuild(ctx, "game_time", ROOT);
            let Some(gt) = gt else { if f % 300 == 0 { log("game_time 없음"); } return; };
            let label = ctx.ui_text(&format!("{}.value", gt)).or_else(|| ctx.ui_text(&gt)).unwrap_or_default();
            let Some(base) = parse_game_time(&label) else { return };
            {
                let mut last = LAST_LABEL.lock().unwrap_or_else(|e| e.into_inner());
                if *last != label { *last = label.clone(); PLAYED_BASE.store(base, Ordering::Relaxed); PLAYED_MS.store(0, Ordering::Relaxed); }
                else { PLAYED_MS.fetch_add(dt, Ordering::Relaxed); }
            }
            let played = base + ((PLAYED_MS.load(Ordering::Relaxed) as i64 * TICKS_PER_SEC / 1000).min(TICKS_PER_SEC - 1));
            if !ACTIVE.swap(true, Ordering::Relaxed) { log(&format!("경기 화면 활성 game_time={:?}", gt)); if PHRASES.lock().unwrap_or_else(|e| e.into_inner()).is_none() { load_phrases(); } }
            // 패널
            if !ctx.ui_exists(PANEL) {
                let ok = ctx.ui_spawn_source(ROOT, &panel_src(&cfg()));
                log(&format!("패널 스폰 {}", ok));
                VIS_LINES.store(lines_for_height(cfg().h) as u64, Ordering::Relaxed);
                *LAST_BUF.lock().unwrap_or_else(|e| e.into_inner()) = Vec::new();
                if !CLICKS_REGISTERED.swap(true, Ordering::Relaxed) {
                    // 게임 chat_btn(있으면) → 토글
                    if let Some(p) = uk::find(ctx, "chat_btn") { ctx.ui_register_click(&p, "", |_| { cfg_set(|c| c.visible = !c.visible); cfg_save(); }); log(&format!("chat_btn 등록 {}", p)); }
                }
            }
            // 드래그/리사이즈(좌클릭 폴링)
            let press = lbutton(); let was = LAST_PRESS.swap(press, Ordering::Relaxed);
            let cur = cursor_ui();
            {
                let mut d = DRAG.lock().unwrap_or_else(|e| e.into_inner());
                let c = cfg();
                if press && !was && cfg().visible { if let Some((mx, my)) = cur {
                    let in_x = mx >= c.x && mx <= c.x + c.w; let in_y = my >= c.y && my <= c.y + c.h;
                    if in_x && in_y {
                        // 그립 판정 폭 18→28px(09-20: 18px 은 마우스 폴링 오차로 자주 빗나감)
                        if mx >= c.x + c.w - 28.0 && my >= c.y + c.h - 28.0 { *d = Some((2, mx, my, c.x, c.y, c.w, c.h)); }
                        else if my <= c.y + HEADER_H + 4.0 { *d = Some((1, mx, my, c.x, c.y, c.w, c.h)); }
                    }
                    log(&format!("press edge cur=({:.0},{:.0}) panel=({:.0},{:.0},{:.0},{:.0}) in={} mode={:?}", mx, my, c.x, c.y, c.w, c.h, in_x && in_y, d.map(|v| v.0)));
                } }
                if let Some((mode, mx0, my0, x0, y0, w0, h0)) = *d {
                    if !press { *d = None; cfg_save(); }
                    else if let Some((mx, my)) = cur {
                        if mode == 1 { cfg_set(|c| { c.x = (x0 + mx - mx0).clamp(0.0, 1920.0 - c.w); c.y = (y0 + my - my0).clamp(0.0, 1080.0 - c.h); }); }
                        else { cfg_set(|c| { c.w = (w0 + mx - mx0).clamp(WIN_MIN_W, 1920.0 - c.x); c.h = (h0 + my - my0).clamp(WIN_MIN_H, 1080.0 - c.y); }); }
                        apply_geom(ctx);
                    }
                }
            }
            let vis = cfg().visible;
            if ctx.ui_visible(PANEL) != Some(vis) { ctx.ui_set_visible(PANEL, vis); }
            if !vis { return; }
            // 인덱스 재생성(이벤트가 늘었을 때, 2초마다)
            if f % 120 == 0 {
                let (gen, n) = { let g = RAW.lock().unwrap_or_else(|e| e.into_inner()); g.as_ref().map(|r| (r.gen, r.events.len() as u64)).unwrap_or((0, 0)) };
                if gen != BUILT_GEN.load(Ordering::Relaxed) { INDEX.lock().unwrap_or_else(|e| e.into_inner()).clear(); *NAMES.lock().unwrap_or_else(|e| e.into_inner()) = None; }
                if gen > 0 && (gen != BUILT_GEN.load(Ordering::Relaxed) || n > BUILT_N.load(Ordering::Relaxed)) { rebuild(ctx, played); }
            }
            // 표시(6프레임마다)
            if f % 6 != 0 { return; }
            let cap = VIS_LINES.load(Ordering::Relaxed) as usize;
            let mut buf: Vec<String> = Vec::with_capacity(cap);
            {
                let index = INDEX.lock().unwrap_or_else(|e| e.into_inner());
                if index.is_empty() {
                    let mut cheer = CHEER.lock().unwrap_or_else(|e| e.into_inner());
                    if f % 240 == 0 { cheer.push(msg_cheer(&[])); while cheer.len() > cap { cheer.remove(0); } }
                    buf = cheer.clone();
                } else {
                    let lo = played - WINDOW_TICKS;
                    for m in index.iter() { if m.tick > played { break; } if m.tick >= lo { buf.push(m.text.clone()); if buf.len() > cap { buf.remove(0); } } }
                }
            }
            let mut last = LAST_BUF.lock().unwrap_or_else(|e| e.into_inner());
            if *last != buf || f % 600 == 0 {
                for i in 0..LINE_SLOTS {
                    let p = format!("{}.l{}", PANEL, i);
                    if i < buf.len() { ctx.ui_set_text(&p, &buf[i]); ctx.ui_set_visible(&p, true); } else { ctx.ui_set_visible(&p, false); }
                }
                *last = buf;
            }
        }));
    }
}
fn deactivate() {
    if ACTIVE.swap(false, Ordering::Relaxed) {
        INDEX.lock().unwrap_or_else(|e| e.into_inner()).clear();
        CHEER.lock().unwrap_or_else(|e| e.into_inner()).clear();
        *NAMES.lock().unwrap_or_else(|e| e.into_inner()) = None;
        BUILT_GEN.store(0, Ordering::Relaxed); BUILT_N.store(0, Ordering::Relaxed);
        *LAST_LABEL.lock().unwrap_or_else(|e| e.into_inner()) = String::new();
        uk::index_clear();
    }
}

fn init(host: &StableHost) -> StableMod {
    host.log(LogLevel::Info, "Spectator_Chat (stable 0.6.0)");
    cfg_load();
    let v = host.game_version();
    log(&format!("INIT game {}.{}.{} host_abi={} cfg x={} y={} w={} h={}", v.major, v.minor, v.patch, host.abi_level(), cfg().x, cfg().y, cfg().w, cfg().h));
    let mut d = StableMod::new(MOD_ID);
    d.set_extension(Ext);
    d.set_match_hook(Hook);
    d
}
declare_stable_mod!(init);
