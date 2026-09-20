//! banpick_view_plus — 밴픽 화면 개편 (daram2 원작 · 0.6.0 stable 포팅 2026-09-17).
//! 원작(클래식): .ui 오버라이드(layout/blue·red_pick_slot) + post_render 렌더 명령 가로채기(호버 탐지·텍스처 치환·알파 캡·레이더·이름 색)
//!            + 스킨 교체(스프라이트 치환 — **이 판에선 제외**, 유저 지시 09-17).
//! stable 판:
//!   · 레이아웃 = 원작 .ui 를 0.6.0 베이스에 3-way 머지(git merge-file, 0826 베이스, 충돌 0) → `mod.override_info` 로 교체(stable 모드도 적용됨: 실증)
//!   · 텍스트 = `text/bvp.i18n` 을 `asset/base/text/ui` 에 merge(원작 그대로)
//!   · 스플래시 = 픽 슬롯에 `bp_splash:image` 스폰, source = `asset/banpick_view_plus/illust/raw/<pack>/<stem>`(모드 폴더 자동 등록)
//!   · 배경 = `main.bp_hover_bg` image source 교체 + 패널 색 알파 캡(`ui_set_properties color:`)
//!   · 호버 = Win32 커서 → UI 좌표(1920×1080) → 카드/슬롯 rect 판정(stable 에 hover 이벤트 없음)
//!   · 레이더 = `post_render` 에서 `draw_line/circle/text/sprite("UI", …)`
//!   · 이름 색 = 카드 `name` 라벨 color. 픽 슬롯 챔피언 = `done.name` 텍스트 → 표시명→id 맵(i18n)
//!   · 버프/너프·레이더 축 = 현재 `champion_brief` vs 번들 `champion_info` 시트(기본값) — 사거리/공속 축은 현재값 없음 → 0
//! 설정 = mods\banpick_view_plus\banpick_view_plus.cfg / 스플래시 선택 = splash.cfg
#![allow(dead_code)]
use mod_api_stable::{declare_stable_mod, LogLevel, StableClient, StableExtension, StableHost, StableMod, StableSpriteParams, TextAlignXV1, TextAlignYV1};
use serde_json::Value;
use std::collections::{BTreeSet, HashMap};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Mutex;
#[path = r"C:\tfm2mods\ui_kit\draft_scene_stable.rs"]
mod draft_scene;
mod showcase; // ★09-20: tfm2_banpick_illust 쇼케이스(밴/픽 연출 카드 일러) 통합 — 게임 훅 RVA 29(패치마다 재핀)

const MOD_ID: &str = "banpick_view_plus";
const DBG: bool = false; // 09-19 확정 배포(진단 시 true)
const ROOT: &str = "main";
const DISC: &str = "main.header.bp_settings";
const PANEL: &str = "main.bp_settings_panel";
const HOVER_BG: &str = "main.bp_hover_bg";
const CARDS: &str = "main.champions.contents";
const PACK_USER: &str = "User_Splash_Art";
const DEFAULT_ILLUST_PACK: &str = "kahluamik_illust";
const MAX_LEVEL: usize = 12;
// 레이아웃 상수(원작)
const INFO_Y_ON: f32 = 745.0;
const INFO_Y_OFF: f32 = 2000.0;
const SCROLL_Y: f32 = 105.0;
const CONTENTS_Y: f32 = 20.0;
const VIEWPORT_H: f32 = 880.0;
const SPACER_H: f32 = 10.0;
const CARD_H: f32 = 135.0;
const CARD_GAP: f32 = 10.0;
const GRID_COLS: usize = 9;
const SLOT_W: f32 = 300.0;
const SLOT_H: f32 = 174.0;
const BAR: f32 = 15.0;
const ILLUST_ASPECT: f32 = 172.0 / 284.0;
const COLOR_BUFF: &str = "#57db78ff";
const COLOR_NERF: &str = "#f7666bff";
const COLOR_NORMAL: &str = "#e8e8e8ff";
/// (노드, 원색, 배경 켜짐 색)
const DIM_NODES: [(&str, &str, &str); 5] = [
    ("main.background", "#07080bff", "#07080b4d"),
    ("main.header", "#161721ff", "#1617214d"),
    ("main.bottom", "#161721ff", "#1617214d"),
    ("main.champions_bg", "#161721ff", "#1617214d"),
    ("main.champion_info.bp_panel_bg", "#07080bff", "#07080b4d"),
];

// ───────── 설정 ─────────
#[derive(Clone, Copy)]
struct Cfg { show_panel: bool, name_color: bool, red_noflip: bool, show_bg: bool, hero_bg: bool, showcase: bool }
static CFG: Mutex<Cfg> = Mutex::new(Cfg { show_panel: true, name_color: true, red_noflip: false, show_bg: false, hero_bg: false, showcase: true });
fn cfg() -> Cfg { *CFG.lock().unwrap_or_else(|e| e.into_inner()) }
fn cfg_mut(f: impl FnOnce(&mut Cfg)) { let mut g = CFG.lock().unwrap_or_else(|e| e.into_inner()); f(&mut g); let c = *g; drop(g); save_cfg(&c); }
/// ★설정 저장소 = 원작과 같은 공용 파일 `<게임>\ModData\config.txt`(키: show_panel/banpick_name_color/banpick_red_noflip/banpick_show_bg/banpick_hero_bg).
///   2026-09-17: 모드 폴더 자체 cfg 를 쓰다 보니 원작에서 꺼 두었던 배경 옵션이 켜진 채 남았다(유저 제보) → 원작 파일을 정본으로.
///   다른 뷰플러스 모드 키는 건드리지 않는다(있는 줄은 값만 교체, 없는 키만 끝에 추가).
fn shared_cfg_path() -> Option<String> { exe_dir().map(|d| format!(r"{}\ModData\config.txt", d)) }
fn cfg_apply_line(c: &mut Cfg, l: &str) {
    let Some((k, v)) = l.split_once('=') else { return };
    let b = matches!(v.trim().to_ascii_lowercase().as_str(), "true" | "1" | "on" | "yes");
    match k.trim() { "show_panel" => c.show_panel = b, "banpick_name_color" => c.name_color = b, "banpick_red_noflip" => c.red_noflip = b, "banpick_show_bg" => c.show_bg = b, "banpick_hero_bg" => c.hero_bg = b, "banpick_showcase" => c.showcase = b, _ => {} }
}
fn load_cfg() -> Cfg {
    let mut c = Cfg { show_panel: true, name_color: true, red_noflip: false, show_bg: false, hero_bg: false, showcase: true };
    if let Some(t) = shared_cfg_path().and_then(|p| std::fs::read_to_string(p).ok()) { for l in t.lines() { cfg_apply_line(&mut c, l); } }
    c
}
fn save_cfg(c: &Cfg) {
    let Some(p) = shared_cfg_path() else { return };
    let mut lines: Vec<String> = std::fs::read_to_string(&p).map(|t| t.lines().map(|l| l.to_string()).collect()).unwrap_or_default();
    let pairs = [("show_panel", c.show_panel), ("banpick_name_color", c.name_color), ("banpick_red_noflip", c.red_noflip), ("banpick_show_bg", c.show_bg), ("banpick_hero_bg", c.hero_bg), ("banpick_showcase", c.showcase)];
    for (k, v) in pairs {
        let line = format!("{} = {}", k, v);
        match lines.iter_mut().find(|l| l.split_once('=').map(|(a, _)| a.trim() == k).unwrap_or(false)) { Some(l) => *l = line, None => lines.push(line) }
    }
    if let Some(d) = std::path::Path::new(&p).parent() { let _ = std::fs::create_dir_all(d); }
    let _ = std::fs::write(&p, lines.join("
") + "
");
}
/// base("bg"/챔프id/question_blue…) → "pack/stem"
static SPLASH_SEL: Mutex<Option<HashMap<String, String>>> = Mutex::new(None);
fn splash_sel() -> HashMap<String, String> { SPLASH_SEL.lock().unwrap_or_else(|e| e.into_inner()).get_or_insert_with(load_splash).clone() }
fn splash_path() -> Option<String> { mod_dir().map(|d| format!(r"{}\splash.cfg", d)) }
fn load_splash() -> HashMap<String, String> {
    let mut m = HashMap::new();
    if let Some(t) = splash_path().and_then(|p| std::fs::read_to_string(p).ok()) { for l in t.lines() { if let Some((k, v)) = l.split_once('=') { m.insert(k.trim().to_string(), v.trim().to_string()); } } }
    m
}
fn set_splash(base: &str, key: &str) {
    let mut g = SPLASH_SEL.lock().unwrap_or_else(|e| e.into_inner());
    let m = g.get_or_insert_with(load_splash);
    m.insert(base.to_string(), key.to_string());
    if let Some(p) = splash_path() { let mut s = String::new(); let mut ks: Vec<_> = m.iter().collect(); ks.sort(); for (k, v) in ks { s.push_str(&format!("{}={}\n", k, v)); } let _ = std::fs::write(p, s); }
}

// ───────── 공통 ─────────
static FRAME: AtomicU64 = AtomicU64::new(0);
static ACTIVE: AtomicBool = AtomicBool::new(false);
static CLICKS_REGISTERED: AtomicBool = AtomicBool::new(false);
static SETTINGS_OPEN: AtomicBool = AtomicBool::new(false);
static STAGED: Mutex<Option<usize>> = Mutex::new(None);
static PENDING_CLICKS: Mutex<Vec<String>> = Mutex::new(Vec::new());
/// 픽 슬롯 → 챔프 id (is_blue, n)
static SLOT_CHAMP: Mutex<Option<HashMap<(bool, usize), String>>> = Mutex::new(None);
static HOVER: Mutex<Option<(String, (f32, f32, f32, f32))>> = Mutex::new(None); // 호버 챔프 + 카드 rect
static HOVER_BG_KEY: Mutex<Option<String>> = Mutex::new(None);
static SLOT_GRACE: Mutex<[u64; 10]> = Mutex::new([0; 10]);
// ★성능(2026-09-17 유저 제보 "밴픽이 엄청 느려"): 카드 160장 × 호스트 호출 수백 회/틱을 캐시로 줄인다.
//   · CARDS_CACHE = 카드 id 목록(자식 수가 바뀌거나 90틱마다 갱신)
//   · RECT_CACHE = 카드 rect(첫 카드 rect 가 바뀔 때 = 스크롤 때만 재측정)
//   · FLAG_CACHE = 픽 오버레이(.blue/.red visible) — done 슬롯 집합이 바뀐 뒤 15틱 동안만 재스캔
static CARDS_CACHE: Mutex<Option<(usize, u64, Vec<String>)>> = Mutex::new(None); // (child_count, 갱신 틱, ids)
static RECT_CACHE: Mutex<Option<((f32, f32, f32, f32), Vec<(String, (f32, f32, f32, f32))>)>> = Mutex::new(None); // (첫 카드 rect, [(id, rect)])
static FLAG_CACHE: Mutex<(Vec<(bool, usize)>, u64, Vec<(bool, String)>)> = Mutex::new((Vec::new(), 0, Vec::new())); // (done 집합, 마지막 변화 틱, flagged)
fn cards_cached(ctx: &StableClient<'_>, f: u64) -> Vec<String> {
    let n = ctx.ui_child_count(CARDS).unwrap_or(0);
    let mut g = CARDS_CACHE.lock().unwrap_or_else(|e| e.into_inner());
    if let Some((cn, at, ids)) = g.as_ref() { if *cn == n && f.saturating_sub(*at) < 90 { return ids.clone(); } }
    let ids = ctx.ui_child_names(CARDS);
    *g = Some((n, f, ids.clone()));
    *RECT_CACHE.lock().unwrap_or_else(|e| e.into_inner()) = None;
    ids
}
#[derive(Default)]
struct PickState { slot_champ: HashMap<(bool, usize), String>, known: Vec<(bool, String)> }
static PICK_STATE: Mutex<Option<PickState>> = Mutex::new(None);
/// ★09-18 스왑 추적(유저 제보 "스왑해도 이미지 안 바뀜"): 0.6.0 스왑 표 행(swap_slot_n)의 이름/포지션 텍스트는 stable API 로 못 읽는다(runner None).
///   대신 행 클릭을 등록해 게임의 스왑 조작(행 A 선택 → 행 B 클릭 = A↔B 챔피언 교환 / 같은 행 재클릭 = 선택 해제)을 그대로 따라간다.
///   행 k ↔ 픽 슬롯 = 같은 팀 pick_slot_n 을 y 오름차순으로 정렬한 k 번째(둘 다 라인업 순서 = 탑→서폿). ⚠코치 위임·상대 AI 스왑은 클릭이 없어 추적 불가(한계).
static SWAP_SEL: Mutex<[Option<usize>; 2]> = Mutex::new([None, None]);
static SWAP_ACTIVE: AtomicBool = AtomicBool::new(false);
static SWAP_LAST_CLICK: Mutex<Option<(u64, String)>> = Mutex::new(None);
static NAME_MAP: Mutex<Option<HashMap<String, String>>> = Mutex::new(None); // 표시명 → id
static AXES: Mutex<Option<HashMap<String, [f32; 8]>>> = Mutex::new(None);
static NAME_COLOR: Mutex<Option<HashMap<String, String>>> = Mutex::new(None);
static DIM_STATE: AtomicBool = AtomicBool::new(false);
static SHEET: Mutex<Option<HashMap<String, SheetEntry>>> = Mutex::new(None);
static ILLUST: Mutex<Option<std::sync::Arc<Illust>>> = Mutex::new(None);
static LAST_LAYOUT: Mutex<(f32, f32)> = Mutex::new((-1.0, -1.0));
static LAST_BG: Mutex<Option<String>> = Mutex::new(None);
static LAST_SPLASH: Mutex<Option<HashMap<String, String>>> = Mutex::new(None);

#[link(name = "kernel32")]
extern "system" {
    fn GetModuleFileNameW(module: usize, buf: *mut u16, size: u32) -> u32;
    fn GetCurrentProcessId() -> u32;
}
#[link(name = "user32")]
extern "system" {
    fn GetCursorPos(p: *mut [i32; 2]) -> i32;
    fn GetForegroundWindow() -> usize;
    fn GetWindowThreadProcessId(h: usize, pid: *mut u32) -> u32;
    fn ScreenToClient(h: usize, p: *mut [i32; 2]) -> i32;
    fn GetClientRect(h: usize, r: *mut [i32; 4]) -> i32;
}
fn exe_dir() -> Option<String> {
    let mut buf = [0u16; 520];
    let n = unsafe { GetModuleFileNameW(0, buf.as_mut_ptr(), buf.len() as u32) } as usize;
    if n == 0 || n >= buf.len() { return None; }
    let exe = String::from_utf16_lossy(&buf[..n]);
    exe.rfind(|c| c == '\\' || c == '/').map(|i| exe[..i].to_string())
}
/// 이 dll 이 있는 폴더(워크숍 폴더일 수 있어 exe 기준 mods\ 로 잡지 않는다)
fn mod_dir() -> Option<String> {
    #[link(name = "kernel32")]
    extern "system" { fn GetModuleHandleExW(flags: u32, addr: *const u16, h: *mut usize) -> i32; }
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
    if let Some(d) = mod_dir() {
        use std::io::Write;
        if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(format!(r"{}\debug.log", d)) { let _ = writeln!(f, "[f{}] {}", FRAME.load(Ordering::Relaxed), s); }
    }
}
/// 커서 → UI 좌표(1920×1080). 게임 창이 전면이 아니면 None.
fn cursor_ui() -> Option<(f32, f32)> {
    unsafe {
        let h = GetForegroundWindow(); if h == 0 { return None; }
        let mut pid = 0u32; GetWindowThreadProcessId(h, &mut pid);
        if pid != GetCurrentProcessId() { return None; }
        let mut p = [0i32; 2]; if GetCursorPos(&mut p) == 0 { return None; }
        if ScreenToClient(h, &mut p) == 0 { return None; }
        let mut r = [0i32; 4]; if GetClientRect(h, &mut r) == 0 || r[2] <= 0 || r[3] <= 0 { return None; }
        Some((p[0] as f32 * 1920.0 / r[2] as f32, p[1] as f32 * 1080.0 / r[3] as f32))
    }
}
fn inside(r: (f32, f32, f32, f32), p: (f32, f32)) -> bool { p.0 >= r.0 && p.0 <= r.0 + r.2 && p.1 >= r.1 && p.1 <= r.1 + r.3 }

// ───────── 번들 시트(기본 스탯) ─────────
#[derive(Clone, Default)]
struct SheetEntry { stat: [f32; 6], growth: [f32; 6], cooltime: f32, range: f32 }
fn stat6(v: &Value) -> [f32; 6] { let g = |k: &str| v.get(k).and_then(|x| x.as_f64()).unwrap_or(0.0) as f32; [g("attack"), g("magic_power"), g("hp"), g("defence"), g("magic_resistance"), g("move_speed")] }
fn load_sheet() -> HashMap<String, SheetEntry> {
    use std::io::{Read, Seek, SeekFrom};
    let mut out = HashMap::new();
    let Some(dir) = exe_dir() else { return out };
    let Ok(mut f) = std::fs::File::open(format!(r"{}\bundle.game_data", dir)) else { return out };
    let mut u = [0u8; 4]; if f.read_exact(&mut u).is_err() { return out; }
    let mut body: Option<Vec<u8>> = None;
    for _ in 0..100_000 {
        if f.read_exact(&mut u).is_err() { break; }
        let el = u32::from_le_bytes(u) as usize; if el == 0 || el > 64 { break; }
        let mut e = vec![0u8; el]; if f.read_exact(&mut e).is_err() { break; }
        if f.read_exact(&mut u).is_err() { break; }
        let pl = u32::from_le_bytes(u) as usize; if pl > 1024 { break; }
        let mut path = vec![0u8; pl]; if f.read_exact(&mut path).is_err() { break; }
        if f.read_exact(&mut u).is_err() { break; }
        let bl = u32::from_le_bytes(u) as u64;
        if path == b"asset/base/setting/champion_info" { let mut b = vec![0u8; bl as usize]; if f.read_exact(&mut b).is_ok() { body = Some(b); } break; }
        if f.seek(SeekFrom::Current(bl as i64)).is_err() { break; }
    }
    let Some(b) = body else { return out };
    let Ok(v) = serde_json::from_slice::<Value>(&b) else { return out };
    let mut push = |name: &str, e: &Value| { out.insert(name.to_string(), SheetEntry { stat: e.get("stat").map(stat6).unwrap_or_default(), growth: e.get("growth").map(stat6).unwrap_or_default(), cooltime: e.get("attack").and_then(|a| a.get("cooltime")).and_then(|x| x.as_f64()).unwrap_or(0.0) as f32, range: e.get("attack").and_then(|a| a.get("range")).and_then(|x| x.as_f64()).unwrap_or(0.0) as f32 }); };
    if let Some(m) = v.as_object() { for (k, e) in m { if k == "mod_champions" { if let Some(arr) = e.as_array() { for e in arr { if let Some(id) = e.get("id").and_then(|x| x.as_str()) { push(id, e); } } } } else if e.is_object() { push(k, e); } } }
    out
}
fn sheet() -> HashMap<String, SheetEntry> { SHEET.lock().unwrap_or_else(|e| e.into_inner()).get_or_insert_with(load_sheet).clone() }
/// 축 8개 = [attack, magic, hp, def, mr, move, range, atkspeed] 상대변화(원작 compute_radar). 사거리·공속은 현재값 API 없음 → 0.
fn compute_axes(ctx: &StableClient<'_>) -> HashMap<String, [f32; 8]> {
    let sh = sheet(); let mut out = HashMap::new(); let lv = (MAX_LEVEL - 1) as f32;
    for name in ctx.champion_names() {
        let Some(b) = ctx.champion_brief(&name) else { continue };
        let mut axes = [0f32; 8];
        if let Some(e) = sh.get(&name) {
            let cur = [b.stat.attack, b.stat.magic_power, b.stat.hp, b.stat.defence, b.stat.magic_resistance, b.stat.move_speed].map(|x| x as f32);
            let cg = [b.growth.attack, b.growth.magic_power, b.growth.hp, b.growth.defence, b.growth.magic_resistance, b.growth.move_speed].map(|x| x as f32);
            for k in 0..6 { let isum = e.stat[k] + (e.stat[k] + e.growth[k] * lv); if isum > 0.0 { axes[k] = (cur[k] + (cur[k] + cg[k] * lv) - isum) / isum; } }
        }
        out.insert(name, axes);
    }
    out
}
fn tint_of(axes: &[f32; 8]) -> i8 { let net: f32 = axes.iter().sum(); if net > 0.01 { 1 } else if net < -0.01 { -1 } else { 0 } }

// ───────── 일러스트 인덱스 ─────────
#[derive(Default, Clone)]
struct Illust {
    /// stem → packs (팩 우선순 유지)
    packs: HashMap<String, Vec<String>>,
    pack_order: Vec<String>,
    /// (pack, 표시명, layer n)
    vpacks: Vec<(String, String, u32)>,
}
fn stem_layer(stem: &str) -> u32 { stem.rsplit_once('-').and_then(|(_, n)| n.parse().ok()).unwrap_or(0) }
fn stem_base(stem: &str) -> String { match stem.rsplit_once('-') { Some((b, n)) if n.parse::<u32>().is_ok() => b.to_string(), _ => stem.to_string() } }
fn read_enabled_mods() -> Vec<String> {
    let Some(dir) = exe_dir() else { return Vec::new() };
    let Ok(t) = std::fs::read_to_string(format!(r"{}\config\game\mods.json", dir)) else { return Vec::new() };
    serde_json::from_str::<Value>(&t).ok().and_then(|v| v.get("enabled_mods").and_then(|a| a.as_array()).map(|a| a.iter().filter_map(|x| x.as_str().map(str::to_string)).collect())).unwrap_or_default()
}
fn find_mod_dir(id: &str) -> Option<String> {
    let dir = exe_dir()?;
    let local = format!(r"{}\mods\{}", dir, id);
    if std::path::Path::new(&local).is_dir() { return Some(local); }
    // 워크숍: steamapps/workshop/content/3009300/<item>/  (mod_info.mod_id 대조)
    let ws = format!(r"{}\..\..\workshop\content\3009300", dir);
    if let Ok(rd) = std::fs::read_dir(&ws) {
        for e in rd.flatten() {
            let p = e.path(); let mi = p.join("mod.mod_info");
            if let Ok(t) = std::fs::read_to_string(&mi) { if let Ok(v) = serde_json::from_str::<Value>(&t) { if v.get("mod_id").and_then(|x| x.as_str()) == Some(id) { return p.to_str().map(str::to_string); } } }
        }
    }
    None
}
fn modinfo_name(dir: &str) -> Option<String> { std::fs::read_to_string(format!(r"{}\mod.mod_info", dir)).ok().and_then(|t| serde_json::from_str::<Value>(&t).ok()).and_then(|v| v.get("name").and_then(|x| x.as_str()).map(str::to_string)) }
/// 원작 prepare_illust 축약: ModData\illust\User_Splash_Art + 활성 모드들의 BanPickIllust\ → <mod_dir>\illust\raw\<pack>\<stem>.png 로 복사(크기 다르면 갱신) → 인덱스.
fn prepare_illust() -> Illust {
    let mut il = Illust::default();
    let Some(md) = mod_dir() else { return il };
    let raw = format!(r"{}\illust\raw", md); let _ = std::fs::create_dir_all(&raw);
    let user_src = exe_dir().map(|d| format!(r"{}\ModData\illust\{}", d, PACK_USER));
    if let Some(u) = &user_src { let _ = std::fs::create_dir_all(u); }
    let mut sources: Vec<(String, String, String)> = Vec::new(); // (pack, dir, 표시명)
    if let Some(u) = user_src { sources.push((PACK_USER.into(), u, String::new())); }
    let mut baseline: Option<(String, String, String)> = None;
    for id in read_enabled_mods() {
        if id == MOD_ID { continue; }
        let Some(d) = find_mod_dir(&id) else { continue };
        let bp = format!(r"{}\BanPickIllust", d); if !std::path::Path::new(&bp).is_dir() { continue; }
        let name = modinfo_name(&d).unwrap_or_else(|| id.clone());
        if id == DEFAULT_ILLUST_PACK { baseline = Some((id, bp, name)); } else { sources.push((id, bp, name)); }
    }
    if let Some(b) = baseline { sources.push(b); }
    // 복사(새 파일/크기 변경만)
    let mut copied = 0usize;
    for (pack, dir, _) in &sources {
        let Ok(rd) = std::fs::read_dir(dir) else { continue };
        let pd = format!(r"{}\{}", raw, pack); let _ = std::fs::create_dir_all(&pd);
        for e in rd.flatten() {
            let p = e.path(); if !p.is_file() { continue; }
            let Some(name) = p.file_name().and_then(|s| s.to_str()) else { continue };
            if !name.to_ascii_lowercase().ends_with(".png") || name.starts_with('_') || name.starts_with('.') { continue; }
            let dst = std::path::Path::new(&pd).join(name);
            let need = match (p.metadata(), dst.metadata()) { (Ok(a), Ok(b)) => a.len() != b.len(), (Ok(_), Err(_)) => true, _ => false };
            if need { if std::fs::copy(&p, &dst).is_ok() { copied += 1; } }
        }
    }
    // 인덱스 = raw 폴더 실제 내용(복사 실패/기존 팩 포함). 팩 순서 = sources 순 + 나머지(알파벳)
    let mut order: Vec<String> = sources.iter().map(|s| s.0.clone()).collect();
    let mut names: HashMap<String, String> = sources.iter().map(|s| (s.0.clone(), s.2.clone())).collect();
    if let Ok(rd) = std::fs::read_dir(&raw) {
        let mut extra: Vec<String> = rd.flatten().filter(|e| e.path().is_dir()).filter_map(|e| e.file_name().to_str().map(str::to_string)).filter(|n| !order.contains(n)).collect();
        extra.sort();
        for n in extra { names.entry(n.clone()).or_insert_with(|| n.clone()); order.push(n); }
    }
    for pack in &order {
        let Ok(rd) = std::fs::read_dir(format!(r"{}\{}", raw, pack)) else { continue };
        for e in rd.flatten() {
            let p = e.path(); if !p.is_file() { continue; }
            let Some(stem) = p.file_stem().and_then(|s| s.to_str()) else { continue };
            if !p.extension().map(|x| x.eq_ignore_ascii_case("png")).unwrap_or(false) { continue; }
            let v = il.packs.entry(stem.to_string()).or_default(); if !v.contains(pack) { v.push(pack.clone()); }
        }
    }
    let mut pack_layers: HashMap<String, BTreeSet<u32>> = HashMap::new();
    for (stem, list) in &il.packs { if stem_base(stem) == "bg" { continue; } for p in list { pack_layers.entry(p.clone()).or_default().insert(stem_layer(stem)); } }
    for p in &order { if let Some(ns) = pack_layers.get(p) { let nm = if p == PACK_USER { String::new() } else { names.get(p).cloned().unwrap_or_else(|| p.clone()) }; for n in ns { il.vpacks.push((p.clone(), nm.clone(), *n)); } } }
    il.pack_order = order;
    log(&format!("illust: packs={} stems={} vpacks={} copied={}", il.pack_order.len(), il.packs.len(), il.vpacks.len(), copied));
    il
}
fn illust() -> std::sync::Arc<Illust> { ILLUST.lock().unwrap_or_else(|e| e.into_inner()).get_or_insert_with(|| std::sync::Arc::new(prepare_illust())).clone() }
fn layers_of(il: &Illust, base: &str) -> Vec<String> {
    let mut v: Vec<(u32, String)> = il.packs.keys().filter(|s| stem_base(s) == base).map(|s| (stem_layer(s), s.clone())).collect();
    v.sort(); v.into_iter().map(|x| x.1).collect()
}
fn flat_cands(il: &Illust, base: &str) -> Vec<String> {
    let layers = layers_of(il, base); let mut out = Vec::new();
    for pack in &il.pack_order { for stem in &layers { if il.packs.get(stem).map(|l| l.contains(pack)).unwrap_or(false) { out.push(format!("{}/{}", pack, stem)); } } }
    out
}
fn chosen_key(il: &Illust, base: &str) -> Option<String> {
    let c = flat_cands(il, base); if c.is_empty() { return None; }
    let sel = splash_sel().get(base).cloned();
    Some(match sel { Some(k) if c.contains(&k) => k, _ => c[0].clone() })
}
/// 쇼케이스(연출 카드) 아트 — 픽 슬롯과 같은 선택(팩 순환 반영). 반환 = (에셋 키, flip). 레드 = 엔진 flip(`red_noflip` 이면 안 뒤집음).
pub(crate) fn showcase_art(champ_id: &str, is_blue: bool) -> Option<(String, bool)> {
    let il = illust();
    let k = chosen_key(&il, champ_id)?;
    Some((asset_of(&k), !is_blue && !cfg().red_noflip))
}
fn cycle_illust(base: &str) {
    let il = illust(); let c = flat_cands(&il, base); if c.len() < 2 { return; }
    let cur = chosen_key(&il, base).unwrap_or_default();
    let i = c.iter().position(|k| *k == cur).unwrap_or(0);
    set_splash(base, &c[(i + 1) % c.len()]);
}
fn apply_vpack(idx: usize) {
    let il = illust(); let Some((pack, _, n)) = il.vpacks.get(idx).cloned() else { return };
    for (stem, list) in &il.packs { if stem_layer(stem) != n { continue; } let base = stem_base(stem); if base == "bg" { continue; } if list.contains(&pack) { set_splash(&base, &format!("{}/{}", pack, stem)); } }
}
fn asset_of(key: &str) -> String { format!("asset/{}/illust/raw/{}", MOD_ID, key) }

fn layout_for(card_area: f32, show: bool) -> (f32, f32) {
    if !show { return (INFO_Y_OFF, 0.0); }
    if card_area <= 0.0 { return (INFO_Y_ON, 0.0); }
    let card_end = CONTENTS_Y + card_area; let panel_in_scroll = INFO_Y_ON - SCROLL_Y;
    let spacer = if card_end > panel_in_scroll { card_end + (VIEWPORT_H - panel_in_scroll) + CONTENTS_Y - SPACER_H } else { 0.0 };
    (INFO_Y_ON, spacer)
}
fn push_click(s: &str) { PENDING_CLICKS.lock().unwrap_or_else(|e| e.into_inner()).push(s.to_string()); }
fn name_map(ctx: &StableClient<'_>) -> HashMap<String, String> {
    let mut g = NAME_MAP.lock().unwrap_or_else(|e| e.into_inner());
    g.get_or_insert_with(|| { let mut m = HashMap::new(); for id in ctx.champion_names() { if let Some(n) = ctx.i18n(&format!("#asset/base/text/champion?description.{}.name", id)) { if !n.is_empty() { m.insert(n, id.clone()); } } } log(&format!("이름 맵 {}개", m.len())); m }).clone()
}
fn set_vis(ctx: &mut StableClient<'_>, p: &str, v: bool) { if ctx.ui_exists(p) && ctx.ui_visible(p) != Some(v) { ctx.ui_set_visible(p, v); } }

struct Ext;
impl StableExtension for Ext {
    fn post_update(&self, ctx: &mut StableClient<'_>, _dt: u64) {
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let f = FRAME.fetch_add(1, Ordering::Relaxed);
            if f % 3 != 0 { return; }
            if cfg().showcase { showcase::tick(); } // 늦은 1회 설치(멱등) — 밴픽 화면 진입 전에 설치돼 있어야 첫 연출부터 잡힌다
            if !ctx.ui_exists(DISC) { deactivate(); return; }
            let il = illust();
            if !ACTIVE.swap(true, Ordering::Relaxed) {
                log("밴픽 화면 활성");
                *AXES.lock().unwrap_or_else(|e| e.into_inner()) = Some(compute_axes(ctx));
                *NAME_COLOR.lock().unwrap_or_else(|e| e.into_inner()) = None;
                *LAST_LAYOUT.lock().unwrap_or_else(|e| e.into_inner()) = (-1.0, -1.0);
                *LAST_BG.lock().unwrap_or_else(|e| e.into_inner()) = None;
                *LAST_SPLASH.lock().unwrap_or_else(|e| e.into_inner()) = None;
                DIM_STATE.store(false, Ordering::Relaxed);
                // 스킨 관련 버튼은 이 판에서 제외 → 숨김
                for p in [format!("{}.bp_skin_pack_cycle", PANEL)] { set_vis(ctx, &p, false); }
            }
            // ── 클릭 등록(프로세스당 1회, 경로 키 영구)
            if !CLICKS_REGISTERED.swap(true, Ordering::Relaxed) {
                ctx.ui_register_click(DISC, "", |_| push_click("settings"));
                for k in ["bp_panel_toggle", "bp_namecolor_toggle", "bp_redflip_toggle", "bp_hoverbg_toggle", "bp_follow_toggle", "bp_bg_cycle", "bp_pack_cycle"] {
                    let kk: &'static str = k;
                    ctx.ui_register_click(&format!("{}.{}", PANEL, k), "", move |_| push_click(kk));
                }
                for team in ["blue", "red"] { for n in 0..5 { let tag: String = format!("illust:{}:{}", team, n); let p = format!("main.{}_picks.pick_slot_{}.done.bp_illust_cycle", team, n); let t2 = tag.clone(); ctx.ui_register_click(&p, "", move |_| push_click(&t2)); let _ = tag; } }
                log("클릭 핸들러 등록");
            }
            if let Some(msg) = draft_scene::install_once() { log(&msg); }
            draft_scene::tick();
            // ── 스왑 표 행 클릭 등록(스왑 화면 진입마다 — 표는 매 스왑 재스폰되므로 경로 등록이 남아 있어도 다시 건다; 같은 프레임 중복은 처리부에서 무시)
            {
                let swap_vis = ctx.ui_visible("main.swap").unwrap_or(false);
                if swap_vis && !SWAP_ACTIVE.swap(true, Ordering::Relaxed) {
                    let mut ok = 0;
                    for team in ["blue", "red"] { for n in 0..5 { let tag: String = format!("swap:{}:{}", team, n); let p = format!("main.swap.{}_table.swap_slot_{}", team, n); if ctx.ui_exists(&p) { let t2 = tag.clone(); if ctx.ui_register_click(&p, "", move |_| push_click(&t2)) { ok += 1; } } } }
                    *SWAP_SEL.lock().unwrap_or_else(|e| e.into_inner()) = [None, None];
                    log(&format!("스왑 화면 진입: 행 클릭 등록 {}/10", ok));
                } else if !swap_vis && SWAP_ACTIVE.swap(false, Ordering::Relaxed) {
                    *SWAP_SEL.lock().unwrap_or_else(|e| e.into_inner()) = [None, None];
                }
            }
            // ── 클릭 처리
            let clicks: Vec<String> = std::mem::take(&mut *PENDING_CLICKS.lock().unwrap_or_else(|e| e.into_inner()));
            for c in clicks {
                match c.as_str() {
                    "settings" => { let v = !SETTINGS_OPEN.load(Ordering::Relaxed); SETTINGS_OPEN.store(v, Ordering::Relaxed); }
                    "bp_panel_toggle" => cfg_mut(|c| c.show_panel = !c.show_panel),
                    "bp_namecolor_toggle" => cfg_mut(|c| c.name_color = !c.name_color),
                    "bp_redflip_toggle" => cfg_mut(|c| c.red_noflip = !c.red_noflip),
                    "bp_hoverbg_toggle" => cfg_mut(|c| c.show_bg = !c.show_bg),
                    "bp_follow_toggle" => cfg_mut(|c| c.hero_bg = !c.hero_bg),
                    "bp_bg_cycle" => cycle_illust("bg"),
                    "bp_pack_cycle" => { if !il.vpacks.is_empty() { let mut s = STAGED.lock().unwrap_or_else(|e| e.into_inner()); let n = match *s { Some(i) => (i + 1) % il.vpacks.len(), None => 0 }; *s = Some(n); drop(s); apply_vpack(n); } }
                    s if s.starts_with("illust:") => {
                        let parts: Vec<&str> = s.split(':').collect();
                        if parts.len() == 3 { let is_blue = parts[1] == "blue"; if let Ok(n) = parts[2].parse::<usize>() { if let Some(id) = SLOT_CHAMP.lock().unwrap_or_else(|e| e.into_inner()).as_ref().and_then(|m| m.get(&(is_blue, n)).cloned()) { cycle_illust(&id); } } }
                    }
                    s if s.starts_with("swap:") => {
                        // 같은 프레임 중복 발화(재등록 누적) 무시
                        { let mut lc = SWAP_LAST_CLICK.lock().unwrap_or_else(|e| e.into_inner()); if lc.as_ref().map(|(ff, t)| *ff == f && t == s).unwrap_or(false) { continue; } *lc = Some((f, s.to_string())); }
                        let parts: Vec<&str> = s.split(':').collect();
                        if parts.len() == 3 { if let Ok(n) = parts[2].parse::<usize>() {
                            let is_blue = parts[1] == "blue"; let si = if is_blue { 0 } else { 1 };
                            let mut sel = SWAP_SEL.lock().unwrap_or_else(|e| e.into_inner());
                            match sel[si] {
                                Some(a) if a == n => { sel[si] = None; }
                                Some(a) => {
                                    sel[si] = None;
                                    let team = parts[1];
                                    let mut order: Vec<(usize, f32)> = (0..5).filter_map(|k| ctx.ui_node_rect(&format!("main.{}_picks.pick_slot_{}", team, k)).map(|r| (k, r.1))).collect();
                                    order.sort_by(|x, y| x.1.partial_cmp(&y.1).unwrap_or(std::cmp::Ordering::Equal));
                                    if let (Some(&(ka, _)), Some(&(kb, _))) = (order.get(a), order.get(n)) {
                                        let mut g = PICK_STATE.lock().unwrap_or_else(|e| e.into_inner());
                                        if let Some(st) = g.as_mut() {
                                            let va = st.slot_champ.get(&(is_blue, ka)).cloned(); let vb = st.slot_champ.get(&(is_blue, kb)).cloned();
                                            match (va, vb) { (Some(va), Some(vb)) => { st.slot_champ.insert((is_blue, ka), vb.clone()); st.slot_champ.insert((is_blue, kb), va.clone()); log(&format!("스왑 추적 {}: 행{}↔행{} = 슬롯{}({})↔슬롯{}({})", team, a, n, ka, va, kb, vb)); } _ => log(&format!("스왑 추적 {}: 슬롯 {}/{} 챔피언 미확정 — 건너뜀", team, ka, kb)) }
                                        }
                                    }
                                }
                                None => { sel[si] = Some(n); }
                            }
                        } }
                    }
                    _ => {}
                }
                log(&format!("클릭 {}", c));
            }
            let c = cfg();
            // ── 크롬(카테고리 드롭다운이 보일 때만 필터·⚙)
            let chrome = ctx.ui_visible("main.champion_category").unwrap_or(true);
            set_vis(ctx, "main.champion_tier_filter", chrome);
            set_vis(ctx, DISC, chrome);
            // ── 설정 패널
            let panel_open = chrome && SETTINGS_OPEN.load(Ordering::Relaxed);
            set_vis(ctx, PANEL, panel_open);
            if panel_open {
                for (btn, on) in [("bp_panel_toggle", c.show_panel), ("bp_namecolor_toggle", c.name_color), ("bp_redflip_toggle", c.red_noflip), ("bp_hoverbg_toggle", c.show_bg), ("bp_follow_toggle", c.show_bg && c.hero_bg)] {
                    set_vis(ctx, &format!("{}.{}.on", PANEL, btn), on); set_vis(ctx, &format!("{}.{}.off", PANEL, btn), !on);
                }
                let bg_cands = flat_cands(&il, "bg").len();
                let title = ctx.ui_text(&format!("{}.title", PANEL)).unwrap_or_default();
                let ko = title.contains("환경설정");
                let bg_label = if bg_cands == 0 { "0".to_string() } else { let cur = chosen_key(&il, "bg"); let c = flat_cands(&il, "bg"); let pos = cur.as_deref().and_then(|k| c.iter().position(|x| x == k)).unwrap_or(0); format!("{}/{}", pos + 1, c.len()) };
                ctx.ui_set_text(&format!("{}.bp_bg_cycle.idx", PANEL), &bg_label);
                let staged = *STAGED.lock().unwrap_or_else(|e| e.into_inner());
                let pack_label = match staged {
                    Some(i) => match il.vpacks.get(i) { Some((_, raw, n)) if raw.is_empty() => format!("{}{}", if ko { "내 그림" } else { "My art" }, if *n == 0 { String::new() } else if ko { format!(" (대체{})", n) } else { format!(" (alt{})", n) }), Some((_, raw, n)) => format!("{}{}", raw.split(" (").next().unwrap_or(raw).trim(), if *n == 0 { String::new() } else if ko { format!(" (대체{})", n) } else { format!(" (alt{})", n) }), None => String::new() },
                    None if il.vpacks.is_empty() => (if ko { "적용 가능한 팩 없음" } else { "No packs available" }).to_string(),
                    None => (if ko { "스플래시 아트 팩 선택..." } else { "Select pack..." }).to_string(),
                };
                ctx.ui_set_text(&format!("{}.bp_pack_cycle.label", PANEL), &pack_label);
                set_vis(ctx, &format!("{}.skin_pack_caption", PANEL), false);
                set_vis(ctx, &format!("{}.bp_skin_pack_cycle", PANEL), false);
                for (btn, dis) in [("bp_follow_toggle", !c.show_bg), ("bp_bg_cycle", !(c.show_bg && bg_cands >= 2)), ("bp_pack_cycle", il.vpacks.len() < 2)] { ctx.ui_set_properties(&format!("{}.{}", PANEL, btn), &format!("disabled: {};", dis)); }
            }
            // ── 픽 슬롯 챔피언: done.name 은 **선수 이름**이라 못 쓴다(0.6.0 실측). 카드 그리드의 `blue`/`red` 오버레이(픽 표시)가 새로 켜진
            //    챔피언과 새로 `done` 이 된 슬롯을 같은 틱에서 짝지운다(픽 순서 = 슬롯 완료 순서). 늦게 켠 경우엔 남은 것끼리 순서대로.
            let mut slots: HashMap<(bool, usize), String> = HashMap::new();
            let mut slot_rects: Vec<((bool, usize), (f32, f32, f32, f32))> = Vec::new();
            let mut done_now: Vec<(bool, usize)> = Vec::new();
            for team in ["blue", "red"] {
                let is_blue = team == "blue";
                for n in 0..5 {
                    let slot = format!("main.{}_picks.pick_slot_{}", team, n);
                    if let Some(r) = ctx.ui_node_rect(&slot) { slot_rects.push(((is_blue, n), r)); }
                    if ctx.ui_visible(&format!("{}.done", slot)).unwrap_or(false) { done_now.push((is_blue, n)); }
                    else { slots.insert((is_blue, n), if is_blue { "question_blue".into() } else { "question_red".into() }); }
                }
            }
            let cards: Vec<String> = cards_cached(ctx, f);
            let flagged: Vec<(bool, String)> = {
                let mut fc = FLAG_CACHE.lock().unwrap_or_else(|e| e.into_inner());
                let mut done_sorted = done_now.clone(); done_sorted.sort();
                if fc.0 != done_sorted { fc.0 = done_sorted; fc.1 = f; }
                if f.saturating_sub(fc.1) <= 15 || f % 60 == 0 {
                    let mut fl: Vec<(bool, String)> = Vec::new();
                    for id in &cards {
                        if ctx.ui_visible(&format!("{}.{}.blue", CARDS, id)) == Some(true) { fl.push((true, id.clone())); }
                        if ctx.ui_visible(&format!("{}.{}.red", CARDS, id)) == Some(true) { fl.push((false, id.clone())); }
                    }
                    fc.2 = fl;
                }
                fc.2.clone()
            };
            {
                let mut g = PICK_STATE.lock().unwrap_or_else(|e| e.into_inner());
                let st = g.get_or_insert_with(PickState::default);
                // 사라진 것 정리(새 세트 등)
                st.slot_champ.retain(|k, _| done_now.contains(k));
                st.known.retain(|k| flagged.contains(k));
                for side in [true, false] {
                    let mut new_champs: Vec<String> = flagged.iter().filter(|(b, id)| *b == side && !st.known.contains(&(side, id.clone())) && !st.slot_champ.values().any(|v| v == id)).map(|(_, id)| id.clone()).collect();
                    let mut new_slots: Vec<(bool, usize)> = done_now.iter().filter(|k| k.0 == side && !st.slot_champ.contains_key(k)).copied().collect();
                    new_slots.sort();
                    for (k, id) in new_slots.iter().zip(new_champs.drain(..)) { st.slot_champ.insert(*k, id.clone()); st.known.push((side, id)); log(&format!("슬롯 매칭 {:?} ← {}", k, st.slot_champ[k])); }
                }
                // ★09-18 RE: 스왑 단계(phase 7)부터는 씬 raw order(포지션 p → 픽 인덱스)가 정답 — 유저 클릭·코치 위임·상대 AI 스왑 전부 반영.
                //   y 순 p 번째 픽 슬롯 ← pick[order[p]]. raw 를 못 읽으면(훅 미설치) 클릭 추적(swap:) 결과가 남는다.
                if let Some(raw) = draft_scene::read().filter(|r| r.phase >= draft_scene::PHASE_SWAP) {
                    for side in [true, false] {
                        let lineup = raw.lineup(if side { 0 } else { 1 });
                        if lineup.is_empty() { continue; }
                        let mut order: Vec<(usize, f32)> = slot_rects.iter().filter(|((b, _), _)| *b == side).map(|((_, n), r)| (*n, r.1)).collect();
                        order.sort_by(|x, y| x.1.partial_cmp(&y.1).unwrap_or(std::cmp::Ordering::Equal));
                        for (p, c) in lineup.iter().enumerate() {
                            if let (Some(c), Some(&(n, _))) = (c, order.get(p)) {
                                if st.slot_champ.contains_key(&(side, n)) && st.slot_champ.get(&(side, n)) != Some(c) { log(&format!("스왑 raw: {} 슬롯{} {} → {}", if side { "blue" } else { "red" }, n, st.slot_champ[&(side, n)], c)); }
                                if st.slot_champ.contains_key(&(side, n)) { st.slot_champ.insert((side, n), c.clone()); }
                            }
                        }
                    }
                    if f % 600 == 0 { log(&format!("스왑 raw: phase={} orderA={:?} orderB={:?}", raw.phase, raw.order_a, raw.order_b)); }
                }
                for (k, v) in &st.slot_champ { slots.insert(*k, v.clone()); }
                if f % 600 == 0 { log(&format!("flagged={:?} done={:?} slots={:?}", flagged, done_now, st.slot_champ)); }
            }
            *SLOT_CHAMP.lock().unwrap_or_else(|e| e.into_inner()) = Some(slots.clone());
            // ── 호버(커서 → 카드/슬롯)
            let cur = cursor_ui();
            let mut hov: Option<(String, (f32, f32, f32, f32))> = None;
            if let Some(p) = cur {
                if ctx.ui_visible("main.champions").unwrap_or(false) {
                    let clip = ctx.ui_node_rect("main.champions").unwrap_or((335.0, 105.0, 1250.0, 880.0));
                    if inside(clip, p) {
                        // rect 캐시: 첫 카드 rect(스크롤 지표)가 같으면 캐시 사용, 다르면 전체 재측정
                        let first = cards.first().and_then(|id| ctx.ui_node_rect(&format!("{}.{}", CARDS, id)));
                        let mut rc = RECT_CACHE.lock().unwrap_or_else(|e| e.into_inner());
                        let stale = match (&*rc, first) { (Some((fr, _)), Some(cur)) => *fr != cur, _ => true };
                        if stale {
                            let mut v = Vec::with_capacity(cards.len());
                            for id in &cards { let cp = format!("{}.{}", CARDS, id); if let Some(r) = ctx.ui_node_rect(&cp) { v.push((id.clone(), r)); } }
                            *rc = first.map(|fr| (fr, v));
                        }
                        if let Some((_, v)) = rc.as_ref() {
                            if let Some((id, r)) = v.iter().find(|(_, r)| inside(*r, p)) {
                                let cp = format!("{}.{}", CARDS, id);
                                if ctx.ui_visible(&cp) == Some(true) { hov = Some((id.clone(), *r)); }
                            }
                        }
                    }
                }
                if hov.is_none() { for (k, r) in &slot_rects { if inside(*r, p) { if let Some(id) = slots.get(k) { if !id.starts_with("question") { hov = Some((id.clone(), *r)); } } } } }
            }
            *HOVER.lock().unwrap_or_else(|e| e.into_inner()) = hov.clone();

            // ── 배경
            // ★호버 배경은 **호버 중일 때만**(2026-09-17: 마지막 호버가 끈적하게 남아 "일러스트가 배경에 남는" 제보) — 카드/슬롯 호버가 끝나면 즉시 bg 팩(또는 없음)으로.
            *HOVER_BG_KEY.lock().unwrap_or_else(|e| e.into_inner()) = match &hov { Some((id, _)) if c.show_bg && c.hero_bg => chosen_key(&il, id), _ => None };
            let bg_key: Option<String> = if !c.show_bg { None } else if c.hero_bg { HOVER_BG_KEY.lock().unwrap_or_else(|e| e.into_inner()).clone().or_else(|| chosen_key(&il, "bg")) } else { chosen_key(&il, "bg") };
            {
                let mut last = LAST_BG.lock().unwrap_or_else(|e| e.into_inner());
                if *last != bg_key {
                    match &bg_key {
                        Some(k) => { let h = 1920.0 * ILLUST_ASPECT; ctx.ui_set_properties(HOVER_BG, &format!("source: \"{}\"; x: 0px; y: {}px; width: 1920px; height: {}px; color: #ffffff80;", asset_of(k), ((1080.0 - h) / 2.0) as i32, h as i32)); set_vis(ctx, HOVER_BG, true); }
                        None => set_vis(ctx, HOVER_BG, false),
                    }
                    *last = bg_key.clone();
                }
            }
            let dim = bg_key.is_some();
            if DIM_STATE.load(Ordering::Relaxed) != dim { for (p, orig, dimc) in DIM_NODES { if ctx.ui_exists(p) { ctx.ui_set_properties(p, &format!("color: {};", if dim { dimc } else { orig })); } } DIM_STATE.store(dim, Ordering::Relaxed); }
            // ── 픽 슬롯 스플래시 + 순환 버튼
            let mut want_splash: HashMap<String, String> = HashMap::new();
            let mut grace = SLOT_GRACE.lock().unwrap_or_else(|e| e.into_inner());
            for ((is_blue, n), r) in &slot_rects {
                let team = if *is_blue { "blue" } else { "red" };
                let slot = format!("main.{}_picks.pick_slot_{}", team, n);
                let id = slots.get(&(*is_blue, *n)).cloned().unwrap_or_default();
                let key = chosen_key(&il, &id);
                let sp = format!("{}.done.bp_splash", slot);
                // ★09-17(유저 제보): 레드 대기 슬롯의 "?" 플레이스홀더까지 좌우반전되던 것 → 챔피언 일러만 반전, question_* 은 반전 안 함.
                match &key { Some(k) => { want_splash.insert(sp.clone(), format!("{}|{}", k, !*is_blue && !c.red_noflip && !id.starts_with("question"))); } None => {} }
                let hovered = cur.map(|p| inside(*r, p)).unwrap_or(false);
                let gi = if *is_blue { *n } else { 5 + *n };
                if hovered { grace[gi] = f + 10; }
                let show_btn = f < grace[gi] && !id.starts_with("question") && flat_cands(&il, &id).len() >= 2;
                set_vis(ctx, &format!("{}.done.bp_illust_cycle", slot), show_btn);
                set_vis(ctx, &format!("{}.done.bp_skin_cycle", slot), false);
            }
            drop(grace);
            {
                let mut last = LAST_SPLASH.lock().unwrap_or_else(|e| e.into_inner());
                let lm = last.get_or_insert_with(HashMap::new);
                for ((is_blue, n), _) in &slot_rects {
                    let slot = format!("main.{}_picks.pick_slot_{}", if *is_blue { "blue" } else { "red" }, n);
                    let done = format!("{}.done", slot);
                    let done_vis = ctx.ui_visible(&done).unwrap_or(false);
                    // 픽 완료 = done.bp_splash / 대기·차례 = 슬롯 루트 bp_splash_q (둘 다 .ui 정의, z 300 > 게임 아이콘)
                    let sp = if done_vis { format!("{}.bp_splash", done) } else { format!("{}.bp_splash_q", slot) };
                    let other = if done_vis { format!("{}.bp_splash_q", slot) } else { format!("{}.bp_splash", done) };
                    set_vis(ctx, &other, false);
                    let key_sp = format!("{}.done.bp_splash", slot);
                    let w = format!("{}|{}", want_splash.get(&key_sp).cloned().unwrap_or_default(), done_vis);
                    if lm.get(&key_sp) == Some(&w) { continue; }
                    if !ctx.ui_exists(&sp) { continue; }
                    let w0 = want_splash.get(&key_sp).cloned().unwrap_or_default();
                    if w0.is_empty() { set_vis(ctx, &sp, false); }
                    else {
                        let (k, flip) = w0.split_once('|').unwrap_or((&w0, "false"));
                        let ok = ctx.ui_set_properties(&sp, &format!("source: \"{}\";", asset_of(k)));
                        let _ = ctx.ui_set_properties(&sp, &format!("flip_x: {};", flip));
                        ctx.ui_set_visible(&sp, true);
                        // 이름 라벨을 일러스트 위로(원작: 이름을 일러스트 위 우/좌 정렬)
                        if done_vis { ctx.ui_set_properties(&format!("{}.name", done), "z: 310;"); }
                        if f < 20000 { log(&format!("splash set {} ← {} flip={} → {}", sp, asset_of(k), flip, ok)); }
                    }
                    let sp = key_sp;
                    lm.insert(sp, w);
                }
            }
            // ── 이름 색
            let axes = AXES.lock().unwrap_or_else(|e| e.into_inner()).clone().unwrap_or_default();
            {
                let mut g = NAME_COLOR.lock().unwrap_or_else(|e| e.into_inner());
                let cache = g.get_or_insert_with(HashMap::new);
                for card in cards.iter().cloned() {
                    let want = if c.name_color { match axes.get(&card).map(tint_of).unwrap_or(0) { 1 => COLOR_BUFF, -1 => COLOR_NERF, _ => COLOR_NORMAL } } else { COLOR_NORMAL };
                    if cache.get(&card).map(|x| x == want).unwrap_or(want == COLOR_NORMAL) { continue; }
                    let p = format!("{}.{}.name", CARDS, card);
                    if ctx.ui_exists(&p) { ctx.ui_set_properties(&p, &format!("color: {};", want)); cache.insert(card, want.to_string()); }
                }
            }
            // ── 하단 패널 레이아웃
            let card_area = ctx.ui_node_rect(CARDS).map(|r| r.3).filter(|h| *h > 0.0).unwrap_or_else(|| { let n = ctx.ui_child_count(CARDS).unwrap_or(0); let rows = (n + GRID_COLS - 1) / GRID_COLS; rows as f32 * CARD_H + rows.saturating_sub(1) as f32 * CARD_GAP });
            let (py, sy) = layout_for(card_area, c.show_panel);
            {
                let mut last = LAST_LAYOUT.lock().unwrap_or_else(|e| e.into_inner());
                if (last.0 - py).abs() > 0.5 || (last.1 - sy).abs() > 0.5 {
                    ctx.ui_set_properties("main.champion_info", &format!("y: {}px;", py as i32));
                    ctx.ui_set_properties("main.champions.bp_spacer", &format!("y: {}px;", sy as i32));
                    *last = (py, sy);
                }
            }
        }));
    }
    fn post_render(&self, ctx: &mut StableClient<'_>) {
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if !ACTIVE.load(Ordering::Relaxed) || !cfg().name_color { return; }
            let Some((id, r)) = HOVER.lock().unwrap_or_else(|e| e.into_inner()).clone() else { return };
            let axes = AXES.lock().unwrap_or_else(|e| e.into_inner()).as_ref().and_then(|m| m.get(&id).copied());
            let Some(axes) = axes else { return };
            draw_radar(ctx, &axes, r);
        }));
    }
}
fn deactivate() {
    SWAP_ACTIVE.store(false, Ordering::Relaxed);
    if ACTIVE.swap(false, Ordering::Relaxed) { SETTINGS_OPEN.store(false, Ordering::Relaxed); *PICK_STATE.lock().unwrap_or_else(|e| e.into_inner()) = None; *HOVER.lock().unwrap_or_else(|e| e.into_inner()) = None; *HOVER_BG_KEY.lock().unwrap_or_else(|e| e.into_inner()) = None; }
}
/// 원작 draw_radar_octagon 재현(UI 맵 드로잉). 카드 오른쪽/왼쪽 옆에 그린다(게임 자체 포지션 툴팁은 카드 위·아래).
fn draw_radar(ctx: &mut StableClient<'_>, axes: &[f32; 8], card: (f32, f32, f32, f32)) {
    use std::f32::consts::PI;
    const ORD: [usize; 8] = [0, 1, 7, 2, 3, 4, 6, 5];
    const MAXPCT: f32 = 0.15;
    let r = 60.0f32; let r0 = r * 0.55; let rmin = r * 0.20; let icon_rad = r + 12.0; let ext = r + 24.0;
    // 원작처럼 카드 위(공간 없으면 아래)에 그린다
    let cx = (card.0 + card.2 / 2.0).clamp(ext, 1920.0 - ext);
    let above = card.1 - 5.0 - 2.0 * ext >= 100.0;
    let cy = (if above { card.1 - 5.0 - ext } else { card.1 + card.3 + 5.0 + ext }).clamp(ext, 1080.0 - ext);
    let ang = |i: usize| -PI / 2.0 + i as f32 * (PI / 4.0);
    let pt = |i: usize, rad: f32| (cx + rad * ang(i).cos(), cy + rad * ang(i).sin());
    let cur_r = |i: usize| { let d = (axes[ORD[i]] / MAXPCT).clamp(-1.0, 1.0); if d >= 0.0 { r0 + d * (r - r0) } else { r0 + d * (r0 - rmin) } };
    let net: f32 = axes.iter().sum();
    let (netcol, fillc) = if net > 0.01 { (0x57db78ff, 0x57db7840) } else if net < -0.01 { (0xf7666bff, 0xf7666b40) } else { (0x73b8faff, 0x73b8fa40) };
    let z = 108;
    ctx.draw_circle("UI", cx, cy, icon_rad + 11.0, z, 0x07080bd9);
    for i in 0..8 { let (x, y) = pt(i, r - 2.0); ctx.draw_line("UI", cx, cy, x, y, 1.0, z + 2, 0xd9e0f229); }
    for (rad, col, w) in [(r0, 0x8c99b88cu32, 1.2f32), (r, 0x8c99b88c, 1.2)] { for i in 0..8 { let (x1, y1) = pt(i, rad); let (x2, y2) = pt((i + 1) % 8, rad); ctx.draw_line("UI", x1, y1, x2, y2, w, z + 3, col); } }
    // 채움 근사: 중심→각 꼭짓점 굵은 선(폴리곤 API 없음)
    for i in 0..8 { let (x, y) = pt(i, cur_r(i)); ctx.draw_line("UI", cx, cy, x, y, 6.0, z + 4, fillc); }
    for i in 0..8 { let (x1, y1) = pt(i, cur_r(i)); let (x2, y2) = pt((i + 1) % 8, cur_r((i + 1) % 8)); ctx.draw_line("UI", x1, y1, x2, y2, 2.4, z + 5, netcol); }
    // 스탯 아이콘(게임 시트 asset/base/ui/banpick/champion_stat_icon#sheet, 8칸 가로 0.1125 폭)
    const ICON_UVX: [f32; 8] = [0.0, 0.1125, 0.225, 0.3375, 0.45, 0.5625, 0.675, 0.7875];
    for i in 0..8 {
        let (icx, icy) = pt(i, icon_rad);
        let p = StableSpriteParams { x: icx, y: icy, z: z + 6, pivot_x: 0.5, pivot_y: 0.5, uv: (ICON_UVX[ORD[i]], 0.0, 0.1125, 0.9), sample_nearest: true, ..Default::default() };
        ctx.draw_sprite("UI", "asset/base/ui/banpick/champion_stat_icon#sheet", &p);
    }
    let pct = (net * 100.0).round() as i32;
    ctx.draw_text("UI", &format!("{}{}%", if pct > 0 { "+" } else { "" }, pct), "asset/base/font/set/bold", (cx - 40.0, cy + icon_rad + 14.0, 80.0, 20.0), z + 6, 13.0, netcol, TextAlignXV1::Center, TextAlignYV1::Center);
}

fn init(host: &StableHost) -> StableMod {
    host.log(LogLevel::Info, "banpick_view_plus (stable 0.6.0)");
    *CFG.lock().unwrap_or_else(|e| e.into_inner()) = load_cfg();
    let v = host.game_version();
    log(&format!("INIT game {}.{}.{} host_abi={} mod_dir={:?}", v.major, v.minor, v.patch, host.abi_level(), mod_dir()));
    let _ = illust(); // 팩 복사·인덱스(에셋 등록이 시작 시점이면 다음 실행부터 반영)
    let mut d = StableMod::new(MOD_ID);
    d.set_extension(Ext);
    d
}
declare_stable_mod!(init);
