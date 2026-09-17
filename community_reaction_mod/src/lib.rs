//! community_reaction_mod — 최신 우리 팀 경기 요약을 `latest_match.js` 로 상시 내보내고 커뮤니티 반응 갤러리(TFA2_gallery.html)를 연다
//! (원작 클래식 0.5.8 → 0.6.0 stable 재작성 2026-09-17. 원작 소스 = `C:\tfm2mods\community_reaction_mod_classic_058`).
//! 구성:
//!   · 클라(`StableExtension`): 세이브 로드(관리화면 `main.top.right` 존재) 시 갤러리 1회 오픈 / 4초마다 `send_command("crm_export")`
//!     (payload = 오늘 날짜 + 상대 전적 head_to_head — 둘 다 클라 전용 API) / 챔피언 한글명 맵(i18n) 채움 / 서버 응답 이벤트 로그
//!   · 서버(`StableServerExtension::handle_command`): `export::run` — 레코드 JSON 으로 요약 조립 → `mods\community_reaction_mod\latest_match.js`
//!   · sim 훅(`StableMatchHook`): 관전/리플레이 sim 에서 하이라이트 수집(`hl.rs`) — 같은 프로세스 static 으로 서버 export 가 읽음
//! 출력 형식 = 원작·EXPORT_FORMAT.md 그대로(갤러리 파서 호환).
#![allow(dead_code)]
use mod_api_stable::{declare_stable_mod, CommandResultV1, LogLevel, SceneKindV1, StableClient, StableCommand, StableExtension, StableHost, StableMod, StableServerCtx, StableServerExtension};
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicI64, AtomicU64, Ordering};
use std::sync::Mutex;

mod export;
mod hl;
mod names;

const MOD_ID: &str = "community_reaction_mod";
const DBG: bool = true;
const CMD: &str = "crm_export";
const EVT: &str = "crm_result";
const EXPORT_INTERVAL_US: u64 = 4_000_000;
const MAIN_VIEW: &str = "main.top.right";

static FRAME: AtomicU64 = AtomicU64::new(0);
static GALLERY_OPENED: AtomicBool = AtomicBool::new(false);
static ACC_US: AtomicU64 = AtomicU64::new(0);
static LAST_OPP: AtomicI64 = AtomicI64::new(-1);
static NAMES_FILLED: AtomicBool = AtomicBool::new(false);
static CLIENT_NOTE: Mutex<String> = Mutex::new(String::new());
static SERVER_NOTE: Mutex<String> = Mutex::new(String::new());

#[link(name = "kernel32")]
extern "system" { fn GetModuleFileNameW(module: usize, buf: *mut u16, size: u32) -> u32; fn GetModuleHandleExW(flags: u32, addr: *const u16, h: *mut usize) -> i32; }
pub fn mod_dir() -> Option<String> {
    let mut h: usize = 0;
    if unsafe { GetModuleHandleExW(0x4 | 0x2, mod_dir as *const () as *const u16, &mut h) } == 0 || h == 0 { return None; }
    let mut buf = [0u16; 1024];
    let n = unsafe { GetModuleFileNameW(h, buf.as_mut_ptr(), buf.len() as u32) } as usize;
    if n == 0 || n >= buf.len() { return None; }
    let p = String::from_utf16_lossy(&buf[..n]);
    p.rfind(|c| c == '\\' || c == '/').map(|i| p[..i].to_string())
}
pub fn log(s: &str) {
    if !DBG { return; }
    if let Some(d) = mod_dir() { use std::io::Write; if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(format!(r"{}\debug.log", d)) { let _ = writeln!(f, "[f{}] {}", FRAME.load(Ordering::Relaxed), s); } }
}

/// 갤러리 HTML 을 게임 프로세스 트리 밖(explorer 핸드오프)에서 오픈 — 원작 그대로.
/// ⚠ 게임 자식으로 브라우저를 띄우면 스팀이 Job Object 로 실행상태를 추적해 게임 종료 후 "중지중" 무한대기.
fn open_gallery() {
    use std::os::windows::process::CommandExt;
    const DETACHED_PROCESS: u32 = 0x0000_0008;
    const CREATE_NO_WINDOW: u32 = 0x0800_0000;
    const CREATE_BREAKAWAY_FROM_JOB: u32 = 0x0100_0000;
    let Some(d) = mod_dir() else { return };
    let html = format!(r"{}\TFA2_gallery.html", d);
    if !std::path::Path::new(&html).exists() { log("갤러리 HTML 없음 — 오픈 생략"); return; }
    let ok = std::process::Command::new("explorer.exe").arg(&html).creation_flags(DETACHED_PROCESS | CREATE_NO_WINDOW | CREATE_BREAKAWAY_FROM_JOB).spawn().is_ok();
    if !ok { let _ = std::process::Command::new("explorer.exe").arg(&html).creation_flags(DETACHED_PROCESS | CREATE_NO_WINDOW).spawn(); }
    log(&format!("갤러리 오픈 {} ok={}", html, ok));
}

fn fill_champ_names(ctx: &StableClient<'_>) {
    let names = ctx.champion_names();
    if names.is_empty() { return; }
    let mut m: HashMap<String, String> = HashMap::new();
    for id in names { if let Some(n) = ctx.i18n(&format!("#asset/base/text/champion?description.{}.name", id)) { if !n.is_empty() && !n.starts_with('#') { m.insert(id, n); } } }
    log(&format!("챔피언 한글명 {}개", m.len()));
    *names::CHAMP_KR.lock().unwrap_or_else(|e| e.into_inner()) = Some(m);
    NAMES_FILLED.store(true, Ordering::Relaxed);
}

struct Ext;
impl StableExtension for Ext {
    fn post_update(&self, ctx: &mut StableClient<'_>, dt: u64) {
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            FRAME.fetch_add(1, Ordering::Relaxed);
            for ev in ctx.take_events() {
                if ev.event != EVT { continue; }
                let p = String::from_utf8_lossy(&ev.payload).to_string();
                if let Some(rest) = p.strip_prefix("opp=") {
                    let (o, note) = rest.split_once('|').unwrap_or((rest, ""));
                    if let Ok(v) = o.trim().parse::<i64>() { LAST_OPP.store(v, Ordering::Relaxed); }
                    let mut ln = CLIENT_NOTE.lock().unwrap_or_else(|e| e.into_inner());
                    if *ln != note { *ln = note.to_string(); log(&format!("[client] 서버 응답: {}", note)); }
                } else { log(&format!("[client] 서버 응답: {}", p)); }
            }
            if ctx.scene_kind() != Some(SceneKindV1::InGame) { GALLERY_OPENED.store(false, Ordering::Relaxed); ACC_US.store(0, Ordering::Relaxed); return; }
            // 세이브 로드 판정 = 관리 화면 루트 존재(타이틀도 InGame 씬이라 씬만으론 구분 불가). 리셋은 타이틀 복귀 때만.
            let loaded = ctx.ui_exists(MAIN_VIEW);
            if loaded && !GALLERY_OPENED.swap(true, Ordering::Relaxed) { open_gallery(); }
            if !GALLERY_OPENED.load(Ordering::Relaxed) { return; }
            if !NAMES_FILLED.load(Ordering::Relaxed) { fill_champ_names(ctx); }
            let acc = ACC_US.fetch_add(dt, Ordering::Relaxed) + dt;
            if acc < EXPORT_INTERVAL_US { return; }
            ACC_US.store(0, Ordering::Relaxed);
            let Some(team) = ctx.player_team_id() else { return };
            let today = ctx.game_date().map(|(y, m, d)| format!("{:04}-{:02}-{:02}", y, m, d)).unwrap_or_default();
            let mut payload = format!("team\t{}\ndate\t{}\n", team, today);
            let opp = LAST_OPP.load(Ordering::Relaxed);
            if opp >= 0 { if let Some((w, l)) = ctx.head_to_head(opp as usize) { payload.push_str(&format!("opp\t{}\t{}\t{}\n", opp, w, l)); } }
            let ok = ctx.send_command(CMD, payload.as_bytes());
            if !ok { log("[client] send_command 실패"); }
        }));
    }
}

struct Srv;
impl StableServerExtension for Srv {
    fn handle_command(&self, ctx: &mut StableServerCtx<'_>, cmd: &StableCommand<'_>) -> CommandResultV1 {
        if cmd.command != CMD { return CommandResultV1::Pass; }
        let reply = cmd.reply_target();
        let p = String::from_utf8_lossy(cmd.payload).to_string();
        let mut req = export::Req { our_team: cmd.sender_team_id.unwrap_or(usize::MAX), today: String::new(), opp: None };
        for l in p.lines() {
            let f: Vec<&str> = l.split('\t').collect();
            match f.first().copied() {
                Some("team") if req.our_team == usize::MAX => { req.our_team = f.get(1).and_then(|x| x.parse().ok()).unwrap_or(usize::MAX); }
                Some("date") => { req.today = f.get(1).unwrap_or(&"").to_string(); }
                Some("opp") if f.len() >= 4 => { if let (Ok(o), Ok(w), Ok(lo)) = (f[1].parse(), f[2].parse(), f[3].parse()) { req.opp = Some((o, w, lo)); } }
                _ => {}
            }
        }
        if req.our_team == usize::MAX { ctx.emit_event(reply, EVT, b"opp=-1|team unknown"); return CommandResultV1::Handled; }
        let t0 = std::time::Instant::now();
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| export::run(ctx, &req)));
        let msg = match r {
            Ok(Ok((opp, note))) => format!("opp={}|{}", opp.map(|o| o as i64).unwrap_or(-1), note),
            Ok(Err(e)) => format!("opp=-1|오류: {}", e),
            Err(_) => "opp=-1|오류: 서버 패닉".to_string(),
        };
        let mut ln = SERVER_NOTE.lock().unwrap_or_else(|e| e.into_inner());
        if *ln != msg { *ln = msg.clone(); log(&format!("[server] {} ({}ms)", msg, t0.elapsed().as_millis())); }
        drop(ln);
        ctx.emit_event(reply, EVT, msg.as_bytes());
        CommandResultV1::Handled
    }
}

fn init(host: &StableHost) -> StableMod {
    host.log(LogLevel::Info, "community_reaction_mod (stable 0.6.0)");
    let v = host.game_version();
    log(&format!("INIT game {}.{}.{} host_abi={}", v.major, v.minor, v.patch, host.abi_level()));
    let mut d = StableMod::new(MOD_ID);
    d.set_extension(Ext);
    d.set_server_extension(Srv);
    d.set_match_hook(hl::Hook);
    d
}
declare_stable_mod!(init);
