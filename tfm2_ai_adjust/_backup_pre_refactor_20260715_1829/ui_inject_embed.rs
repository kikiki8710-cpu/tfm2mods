//! ui_inject_embed — tfm2_ai_adjust 내장 UI 가산주입 프레임워크 (scrim 의 흡수판과 동일 설계).
//! 게임 에셋게터 트램폴린 훅 → main/strategy 템플릿 로드 시 mods/*/ui_inject.txt 조각을 가산주입.
//! scrim 도 동일 프레임워크를 내장 → 둘 중 "먼저 설치한 쪽"이 모든 모드의 매니페스트를 처리(다른 쪽은
//! prologue mismatch 로 안전 bail). 즉 scrim/ai_adjust 어느 하나만 켜도 UI 주입 동작 = 자기완결.
//! 0.4.14 핫픽스 RVA: LOADER 0x540ad0 / PARSER 0x220e100 / ALLOC 0x231fb70.
#![allow(dead_code)]
use mod_api::{Node, GameUI};
use std::fs;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicUsize, Ordering};

const LOADER_RVA: usize = 0x540ad0;
const PARSER_RVA: usize = 0x220e100;
const ALLOC_RVA: usize  = 0x231fb70;
const TARGET: &[u8]  = b"asset/base/ui/layout/main";
const TARGET2: &[u8] = b"asset/base/ui/layout/strategy";   // 전술화면(#row0~#row4)
const NT_SIZE: usize = 0x90;
const BG_BLOCK: &str = "top";

type LoaderFn = extern "win64" fn(usize, *const u8, usize) -> usize;
type ParserFn = extern "win64" fn(*mut u8, *const u8, usize);
type AllocFn  = extern "win64" fn(usize, usize) -> usize;

static BASE: AtomicUsize = AtomicUsize::new(0);
static MAIN_TMPL: AtomicUsize = AtomicUsize::new(0);
static LAST_INJECTED: AtomicUsize = AtomicUsize::new(0);
static INJECT_ATTEMPTS: AtomicUsize = AtomicUsize::new(0);
static STRAT_TMPL: AtomicUsize = AtomicUsize::new(0);
static LAST_INJ_STRAT: AtomicUsize = AtomicUsize::new(0);
static INJ_ATT_STRAT: AtomicUsize = AtomicUsize::new(0);
static TRAMP: AtomicUsize = AtomicUsize::new(0);
static INSTALLED: AtomicBool = AtomicBool::new(false);
static MAIN_TID: AtomicU32 = AtomicU32::new(0);
static INJECTING: AtomicBool = AtomicBool::new(false);
static MODAL_IDS: Mutex<Vec<String>> = Mutex::new(Vec::new());

// ── 디버그 로깅 ──
pub static DBG: AtomicBool = AtomicBool::new(false);
pub static LOG_DIR: Mutex<String> = Mutex::new(String::new());
fn logln(s: &str) {
    if !DBG.load(Ordering::Relaxed) { return; }
    let d = LOG_DIR.lock().unwrap_or_else(|e| e.into_inner()).clone();
    if d.is_empty() { return; }
    use std::io::Write;
    if let Ok(mut f) = fs::OpenOptions::new().create(true).append(true).open(format!("{}\\uinj_log.txt", d)) {
        let _ = f.write_all(s.as_bytes()); let _ = f.write_all(b"\n");
    }
}

type BOOL = i32;
#[link(name = "kernel32")]
extern "system" {
    fn VirtualAlloc(addr: usize, size: usize, alloc_type: u32, protect: u32) -> usize;
    fn VirtualProtect(addr: usize, size: usize, new_protect: u32, old_protect: *mut u32) -> BOOL;
    fn FlushInstructionCache(proc: usize, addr: usize, size: usize) -> BOOL;
    fn GetCurrentProcess() -> usize;
    fn GetCurrentThreadId() -> u32;
    fn GetModuleHandleW(name: *const u16) -> usize;
    fn GetModuleFileNameW(module: usize, buf: *mut u16, size: u32) -> u32;
}
const MEM_COMMIT_RESERVE: u32 = 0x1000 | 0x2000;
const PAGE_EXECUTE_READWRITE: u32 = 0x40;

fn nfind<'a>(n: &'a Node, id: &str) -> Option<&'a Node> {
    if n.id == id { return Some(n); }
    for c in n.child.iter() { if let Some(f) = nfind(c, id) { return Some(f); } }
    None
}
fn nfind_mut<'a>(n: &'a mut Node, id: &str) -> Option<&'a mut Node> {
    if n.id == id { return Some(n); }
    for c in n.child.iter_mut() { if let Some(f) = nfind_mut(c, id) { return Some(f); } }
    None
}
fn root_id_of(text: &[u8]) -> Option<String> {
    let s = std::str::from_utf8(text).ok()?;
    for line in s.lines() {
        let t = line.trim();
        if t.is_empty() || t.starts_with('#') || t.starts_with("//") { continue; }
        let id: String = t.chars().take_while(|&c| c != ':' && c != ' ' && c != '\t' && c != '{').collect();
        if !id.is_empty() { return Some(id); }
    }
    None
}
fn game_mods_dir() -> String {
    let mut buf = [0u16; 520];
    let n = unsafe { GetModuleFileNameW(0, buf.as_mut_ptr(), buf.len() as u32) } as usize;
    if n > 0 && n < buf.len() {
        let exe = String::from_utf16_lossy(&buf[..n]);
        if let Some(i) = exe.rfind(|c| c == '\\' || c == '/') { return format!(r"{}\mods", &exe[..i]); }
    }
    String::new()
}
fn module_loaded(dll_name: &str) -> bool {
    let w: Vec<u16> = dll_name.encode_utf16().chain(core::iter::once(0)).collect();
    unsafe { GetModuleHandleW(w.as_ptr()) != 0 }
}

extern "win64" fn detour(am: usize, path: *const u8, len: usize) -> usize {
    let tramp_addr = TRAMP.load(Ordering::Relaxed);
    if tramp_addr == 0 { return 0; }
    let tramp: LoaderFn = unsafe { core::mem::transmute(tramp_addr) };
    let r = tramp(am, path, len);
    if !path.is_null() && r > 0x10000 && len < 200 {
        let s = unsafe { core::slice::from_raw_parts(path, len) };
        let mt = MAIN_TID.load(Ordering::Relaxed);
        let on_main = mt == 0 || unsafe { GetCurrentThreadId() } == mt;
        if len == TARGET.len() && s == TARGET {
            MAIN_TMPL.store(r, Ordering::Relaxed);
            if on_main && r != LAST_INJECTED.load(Ordering::Relaxed) {
                let ok = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe { do_inject(r) })).unwrap_or(false);
                if ok { LAST_INJECTED.store(r, Ordering::Relaxed); INJECT_ATTEMPTS.store(0, Ordering::Relaxed); }
            }
        } else if len == TARGET2.len() && s == TARGET2 {
            STRAT_TMPL.store(r, Ordering::Relaxed);
            if on_main && r != LAST_INJ_STRAT.load(Ordering::Relaxed) {
                let ok = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe { do_inject(r) })).unwrap_or(false);
                if ok { LAST_INJ_STRAT.store(r, Ordering::Relaxed); INJ_ATT_STRAT.store(0, Ordering::Relaxed); }
            }
        }
    }
    r
}

pub unsafe fn install() -> Result<(), &'static str> {
    if INSTALLED.swap(true, Ordering::Relaxed) { return Err("already"); }
    let base = GetModuleHandleW(core::ptr::null());
    if base == 0 { return Err("module base 0"); }
    BASE.store(base, Ordering::Relaxed);
    let fn_addr = base + LOADER_RVA;
    let expect: [u8; 12] = [0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x53];
    for i in 0..12 { if *((fn_addr + i) as *const u8) != expect[i] {
        logln(&format!("install bail: prologue @+{} = {:#x}(want {:#x}) — 이미 다른 모드(scrim등)가 후킹함, 그쪽이 주입", i, *((fn_addr + i) as *const u8), expect[i]));
        INSTALLED.store(false, Ordering::Relaxed); return Err("prologue mismatch (already hooked?)");
    } }
    let stub = VirtualAlloc(0, 64, MEM_COMMIT_RESERVE, PAGE_EXECUTE_READWRITE);
    if stub == 0 { return Err("VirtualAlloc"); }
    let mut s: Vec<u8> = Vec::new();
    s.extend_from_slice(&expect);
    s.extend_from_slice(&[0x48, 0xb8]);
    s.extend_from_slice(&(fn_addr + 0xc).to_le_bytes());
    s.extend_from_slice(&[0xff, 0xe0]);
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    TRAMP.store(stub, Ordering::Relaxed);
    let d = detour as usize;
    let mut patch = [0u8; 12];
    patch[0] = 0x48; patch[1] = 0xb8;
    patch[2..10].copy_from_slice(&d.to_le_bytes());
    patch[10] = 0xff; patch[11] = 0xe0;
    let mut old: u32 = 0;
    if VirtualProtect(fn_addr, 12, PAGE_EXECUTE_READWRITE, &mut old) == 0 { return Err("VirtualProtect"); }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), fn_addr as *mut u8, 12);
    VirtualProtect(fn_addr, 12, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), fn_addr, 12);
    logln(&format!("install OK: hooked asset-getter fn={:#x}", fn_addr));
    Ok(())
}

unsafe fn find_tmpl(node: usize, target: &[u8], depth: usize) -> usize {
    if node <= 0x10000 || depth > 12 { return 0; }
    let idptr = *((node + 0x08) as *const usize);
    let idlen = *((node + 0x10) as *const usize);
    if idlen == target.len() && idptr > 0x10000 {
        if core::slice::from_raw_parts(idptr as *const u8, idlen) == target { return node; }
    }
    let cptr = *((node + 0x50) as *const usize);
    let clen = *((node + 0x58) as *const usize);
    if cptr > 0x10000 && clen < 1000 {
        for i in 0..clen {
            let found = find_tmpl(cptr + i * NT_SIZE, target, depth + 1);
            if found != 0 { return found; }
        }
    }
    0
}
unsafe fn child_index(container: usize, id: &[u8]) -> Option<usize> {
    let cptr = *((container + 0x50) as *const usize);
    let clen = *((container + 0x58) as *const usize);
    if cptr <= 0x10000 || clen > 2000 { return None; }
    for i in 0..clen {
        let c = cptr + i * NT_SIZE;
        let idptr = *((c + 0x08) as *const usize);
        let idlen = *((c + 0x10) as *const usize);
        if idlen == id.len() && idptr > 0x10000 && core::slice::from_raw_parts(idptr as *const u8, idlen) == id {
            return Some(i);
        }
    }
    None
}
// mods/*/ui_inject.txt 스캔: "<ui상대경로> <타깃id> <위치> [modal]".
fn collect_fragments() -> Vec<(String, String, String, bool)> {
    let mut out = Vec::new();
    let md = game_mods_dir();
    if md.is_empty() { return out; }
    let rd = match fs::read_dir(&md) { Ok(r) => r, Err(_) => return out };
    for ent in rd.flatten() {
        let dir = ent.path();
        let txt = match fs::read_to_string(dir.join("ui_inject.txt")) { Ok(t) => t, Err(_) => continue };
        let mod_id = dir.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
        if mod_id != "tfm2_ai_adjust" && !module_loaded(&format!("{}.dll", mod_id)) { continue; }
        for line in txt.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') { continue; }
            let p: Vec<&str> = line.split_whitespace().collect();
            if p.len() >= 2 {
                let ui = dir.join(p[0]).to_string_lossy().into_owned();
                let is_modal = p.iter().skip(2).any(|t| *t == "modal");
                let pos = match p.get(2) { Some(&t) if t != "modal" => t.to_string(), _ => "end".to_string() };
                out.push((ui, p[1].to_string(), pos, is_modal));
            }
        }
    }
    out
}
unsafe fn do_inject(r: usize) -> bool {
    if INJECTING.swap(true, Ordering::Acquire) { return false; }
    struct Guard; impl Drop for Guard { fn drop(&mut self) { INJECTING.store(false, Ordering::Release); } }
    let _g = Guard;
    let base = BASE.load(Ordering::Relaxed);
    if base == 0 || r <= 0x10000 { return false; }
    MODAL_IDS.lock().unwrap_or_else(|e| e.into_inner()).clear();
    let frags = collect_fragments();
    let mut all_ok = true;
    for (ui, target, pos, modal) in &frags {
        if !inject_one(r, base, ui, target, pos, *modal) { all_ok = false; }
    }
    all_ok
}
unsafe fn inject_one(r: usize, base: usize, ui_path: &str, target_id: &str, pos: &str, is_modal: bool) -> bool {
    let text = match fs::read(ui_path) { Ok(t) => t, Err(_) => return true };
    if text.is_empty() { return true; }
    if let Some(rid) = root_id_of(&text) { if find_tmpl(r, rid.as_bytes(), 0) != 0 { return true; } }
    let parser: ParserFn = core::mem::transmute(base + PARSER_RVA);
    let mut out = [0u8; 0x400];
    parser(out.as_mut_ptr(), text.as_ptr(), text.len());
    let f = out.as_ptr().add(0x10);
    if *(f as *const usize) == usize::MAX { logln(&format!("parse ERR '{}'", target_id)); return true; }
    if is_modal {
        if let Some(rid) = root_id_of(&text) {
            let mut ids = MODAL_IDS.lock().unwrap_or_else(|e| e.into_inner());
            if !ids.contains(&rid) { ids.push(rid); }
        }
    }
    let target = if target_id == "root" { r } else {
        let t = find_tmpl(r, target_id.as_bytes(), 0);
        if t != 0 { t } else { return false; }
    };
    let cap_p = (target + 0x48) as *mut usize;
    let ptr_p = (target + 0x50) as *mut usize;
    let len_p = (target + 0x58) as *mut usize;
    let old_ptr = *ptr_p; let len = *len_p;
    if len > 2000 { return true; }
    let idx = if let Some(a) = pos.strip_prefix("after:") {
        child_index(target, a.as_bytes()).map(|i| i + 1).unwrap_or(len)
    } else if let Some(b) = pos.strip_prefix("before:") {
        child_index(target, b.as_bytes()).unwrap_or(len)
    } else if pos == "end" { len } else { pos.parse::<usize>().unwrap_or(len) }.min(len);
    let galloc: AllocFn = core::mem::transmute(base + ALLOC_RVA);
    let new_n = len + 1;
    let new_ptr = galloc(new_n * NT_SIZE, 8);
    if new_ptr == 0 { return true; }
    if old_ptr != 0 && idx != 0 {
        core::ptr::copy_nonoverlapping(old_ptr as *const u8, new_ptr as *mut u8, idx * NT_SIZE);
    }
    core::ptr::copy_nonoverlapping(f, (new_ptr + idx * NT_SIZE) as *mut u8, NT_SIZE);
    if old_ptr != 0 && idx < len {
        core::ptr::copy_nonoverlapping((old_ptr + idx * NT_SIZE) as *const u8, (new_ptr + (idx + 1) * NT_SIZE) as *mut u8, (len - idx) * NT_SIZE);
    }
    *ptr_p = new_ptr; *cap_p = new_n; *len_p = new_n;
    logln(&format!("OK inject '{}' idx{} (child {}→{})", target_id, idx, len, new_n));
    true
}
// post_update: main/strategy 재시도 주입 + 모달 배경차단. (scrim 이 이미 후킹했으면 이쪽 MAIN/STRAT_TMPL=0 이라 무동작.)
pub fn tick(u: &mut GameUI) {
    if MAIN_TID.load(Ordering::Relaxed) == 0 { MAIN_TID.store(unsafe { GetCurrentThreadId() }, Ordering::Relaxed); }
    let r = MAIN_TMPL.load(Ordering::Relaxed);
    if r > 0x10000 && r != LAST_INJECTED.load(Ordering::Relaxed) {
        let n = INJECT_ATTEMPTS.fetch_add(1, Ordering::Relaxed);
        if unsafe { do_inject(r) } || n > 120 { LAST_INJECTED.store(r, Ordering::Relaxed); INJECT_ATTEMPTS.store(0, Ordering::Relaxed); }
    }
    let rs = STRAT_TMPL.load(Ordering::Relaxed);
    if rs > 0x10000 && rs != LAST_INJ_STRAT.load(Ordering::Relaxed) {
        let n = INJ_ATT_STRAT.fetch_add(1, Ordering::Relaxed);
        if unsafe { do_inject(rs) } || n > 120 { LAST_INJ_STRAT.store(rs, Ordering::Relaxed); INJ_ATT_STRAT.store(0, Ordering::Relaxed); }
    }
    let ids = MODAL_IDS.lock().unwrap_or_else(|e| e.into_inner()).clone();
    if !ids.is_empty() {
        let any = ids.iter().any(|id| nfind(&u.root, id).map(|n| n.visible).unwrap_or(false));
        if let Some(top) = nfind_mut(&mut u.root, BG_BLOCK) { top.disabled = any; }
    }
}
