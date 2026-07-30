//! tfm2_fog — 매치 화면(game view) 위 카메라 동기화 안개 오버레이
//! ===========================================================================
//! 원리:
//!   - 맵/캐릭터/이펙트는 ingame.ui 의 #center_log(960×960) 영역에 코드(Skia)로
//!     직접 렌더되고, UI 노드는 그 "위"에 그려진다(킬로그/Spectator_Chat 실증).
//!   - scrim 의 uinj 프레임워크(@ingame 스코프)로 #center_log 자식에 fog 노드 주입
//!     → 맵→캐릭터→안개 순서. HUD(형제 뒤 노드)는 안개보다 위.
//!   - view::game(RVA 0x722250, rcx=cam 인스턴스)에 asm-스텁 훅(레지스터 저장만,
//!     Rust detour 없음 = ABI/패닉 리스크 0)으로 cam ptr 캡처.
//!   - post_update 매 프레임: cam(center_x@+0xC8, center_y@+0x130, zoom@+0x128)
//!     읽어 fog 노드 rect 를 월드 고정으로 갱신 → 시점 이동/줌 따라 안개가 맵에 붙음.
//! 설정: mods\tfm2_fog\fog.cfg (60프레임마다 핫리로드), 디버그 fog_dbg.txt.
//! ===========================================================================
#![allow(dead_code, unused_imports, unused_variables)]
use mod_api::*;
use std::fs;
use std::sync::atomic::{AtomicBool, AtomicU8, AtomicU32, AtomicUsize, Ordering};
use std::sync::Mutex;

#[path = "C:/tfm2mods/ui_kit/ui_kit.rs"]
mod ui_kit;
use ui_kit::{find, find_mut, set_layout_size, set_layout_xy, set_visible_by_id, rect_set_color, runner_type_name, Rgba};

const MOD_ID: &str = "tfm2_fog";
// game_view::view::game 매프레임 update/cull/render (0.4.14). rcx = cam 인스턴스.
// 패치로 어긋나면 프롤로그 검증이 막아줌(훅 미설치 → 안개는 뷰포트 고정 폴백).
const VIEWGAME_RVA: usize = 0x722250;
// cam 구조체 오프셋 (tfm2-minimap-camera-drag 정본)
const CAM_CX: usize = 0xC8;
const CAM_CY: usize = 0x130;
const CAM_ZOOM: usize = 0x128;
// fog 는 draggable_popup 래퍼(#fog_wrap, 뷰포트 18,64,960×960에 고정) 안의 자식.
// → 좌표는 "래퍼 로컬" (원점=뷰포트 좌상단, 중심=480,480). 래퍼는 game_view 커스텀 러너라
//   자식이 월드 위에 렌더됨(Spectator_Chat draggable_popup 검증). 래퍼는 고정, 자식만 갱신.
const VP: f32 = 960.0;
const VC: f32 = VP * 0.5; // 480 (래퍼 로컬 뷰포트 중심)

// ───────────────────────── WinAPI ─────────────────────────
type BOOL = i32;
#[link(name = "kernel32")]
extern "system" {
    fn GetModuleHandleW(name: *const u16) -> usize;
    fn GetModuleFileNameW(module: usize, buf: *mut u16, size: u32) -> u32;
    fn VirtualAlloc(addr: usize, size: usize, alloc_type: u32, protect: u32) -> usize;
    fn VirtualProtect(addr: usize, size: usize, new_protect: u32, old: *mut u32) -> BOOL;
    fn VirtualQuery(addr: *const core::ffi::c_void, buf: *mut MemBasicInfo, len: usize) -> usize;
    fn FlushInstructionCache(proc: usize, addr: usize, size: usize) -> BOOL;
    fn GetCurrentProcess() -> usize;
    fn GetCurrentThreadId() -> u32;
    fn AddVectoredExceptionHandler(first: u32, handler: VehHandler) -> usize;
}

// ───────────────────────── SEH (shadow-call AV 가드) ─────────────────────────
// tag8 재-dispatch 가 특정 command 에서 skia 내부 AV 를 낼 수 있음(catch_unwind 로 못 잡음).
// VEH 로 AV 를 잡아 landing 으로 점프 → 그 프레임 draw 만 실패로 처리, 게임 계속.
#[repr(C)]
struct ExceptionRecord { code: u32, flags: u32, rec: usize, addr: usize, nparams: u32, _p: u32, params: [usize; 15] }
#[repr(C)]
struct ExceptionPointers { rec: *mut ExceptionRecord, ctx: *mut core::ffi::c_void }
type VehHandler = extern "system" fn(*mut ExceptionPointers) -> i32;
//  SEH[]: 0=active 1=tid 2=land_rip 3=land_rsp 4=land_rbp 5=code_lo 6=code_hi 7=faults
static mut SEH: [u64; 8] = [0u64; 8];
static SEH_INSTALLED: AtomicBool = AtomicBool::new(false);
static SEH_BUSY: AtomicBool = AtomicBool::new(false);

extern "system" fn seh_veh(p: *mut ExceptionPointers) -> i32 {
    const CONTINUE_EXECUTION: i32 = -1;
    const CONTINUE_SEARCH: i32 = 0;
    unsafe {
        if p.is_null() { return CONTINUE_SEARCH; }
        let rec = (*p).rec;
        if rec.is_null() || (*rec).code != 0xC0000005 { return CONTINUE_SEARCH; }
        let g = core::ptr::addr_of!(SEH) as *const u64;
        if *g.add(0) == 0 { return CONTINUE_SEARCH; }
        if *g.add(1) != GetCurrentThreadId() as u64 { return CONTINUE_SEARCH; }
        let ctx = (*p).ctx as usize;
        if ctx == 0 { return CONTINUE_SEARCH; }
        let rip = *((ctx + 0xF8) as *const u64);
        if rip < *g.add(5) || rip >= *g.add(6) { return CONTINUE_SEARCH; }
        *((ctx + 0xF8) as *mut u64) = *g.add(2); // rip = land_rip
        *((ctx + 0x98) as *mut u64) = *g.add(3); // rsp = land_rsp
        *((ctx + 0xA0) as *mut u64) = *g.add(4); // rbp = land_rbp
        let gm = core::ptr::addr_of_mut!(SEH) as *mut u64;
        *gm.add(7) += 1;
        CONTINUE_EXECUTION
    }
}
fn seh_install() {
    if SEH_INSTALLED.swap(true, Ordering::Relaxed) { return; }
    unsafe { AddVectoredExceptionHandler(1, seh_veh); }
}
// f(rcx=a, rdx=b, r8=c, r9=d, [stack]=0.0f32) 를 SEH 보호하에 호출. AV 나면 false.
#[inline(never)]
unsafe fn safe_dispatch(f: usize, a: usize, b: usize, c: usize, d: usize) -> bool {
    if !SEH_INSTALLED.load(Ordering::Relaxed) { return false; }
    while SEH_BUSY.swap(true, Ordering::Acquire) { core::hint::spin_loop(); }
    let g = core::ptr::addr_of_mut!(SEH) as *mut u64;
    *g.add(1) = GetCurrentThreadId() as u64;
    let ok: u64;
    core::arch::asm!(
        "lea rax, [rip + 202f]", "mov [{g} + 16], rax",  // land_rip
        "mov [{g} + 24], rsp",                            // land_rsp (원본)
        "mov [{g} + 32], rbp",                            // land_rbp
        "lea rax, [rip + 200f]", "mov [{g} + 40], rax",  // code_lo
        "lea rax, [rip + 201f]", "mov [{g} + 48], rax",  // code_hi
        "mov qword ptr [{g} + 0], 1",                     // active
        "200:",
        "and rsp, -16",                                   // 16-정렬
        "sub rsp, 0x30",                                  // shadow 0x20 + arg5 슬롯
        "mov dword ptr [rsp + 0x20], 0",                  // arg5 = 0.0f32
        "mov rcx, {a}", "mov rdx, {b}", "mov r8, {c}", "mov r9, {d}",
        "call {f}",
        "201:",
        "mov {ok}, 1",
        "jmp 203f",
        "202:",                                           // AV landing (VEH 가 rsp=land_rsp 복원)
        "mov {ok}, 0",
        "203:",
        "mov rsp, [{g} + 24]",                            // 원본 rsp 복원(양 경로)
        "mov qword ptr [{g} + 0], 0",                     // active=0
        g = in(reg) g, f = in(reg) f,
        a = in(reg) a, b = in(reg) b, c = in(reg) c, d = in(reg) d,
        ok = out(reg) ok,
        out("rax") _, out("rcx") _, out("rdx") _, out("r8") _, out("r9") _, out("r10") _, out("r11") _,
        out("xmm0") _, out("xmm1") _, out("xmm2") _, out("xmm3") _, out("xmm4") _, out("xmm5") _,
    );
    SEH_BUSY.store(false, Ordering::Release);
    ok != 0
}
const MEM_COMMIT_RESERVE: u32 = 0x1000 | 0x2000;
const PAGE_EXECUTE_READWRITE: u32 = 0x40;
#[repr(C)]
#[derive(Default)]
struct MemBasicInfo {
    base: usize, alloc_base: usize, alloc_protect: u32, _pad0: u32,
    region_size: usize, state: u32, protect: u32, mtype: u32, _pad1: u32,
}

fn game_mods_dir() -> String {
    let mut buf = [0u16; 520];
    let n = unsafe { GetModuleFileNameW(0, buf.as_mut_ptr(), buf.len() as u32) } as usize;
    if n > 0 && n < buf.len() {
        let exe = String::from_utf16_lossy(&buf[..n]);
        if let Some(i) = exe.rfind(|c| c == '\\' || c == '/') {
            return format!(r"{}\mods", &exe[..i]);
        }
    }
    r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\mods".to_string()
}
fn my_dir() -> String { format!(r"{}\{}", game_mods_dir(), MOD_ID) }

fn dbg_log(s: &str) {
    use std::io::Write;
    if let Ok(mut f) = fs::OpenOptions::new().create(true).append(true)
        .open(format!(r"{}\fog_dbg.txt", my_dir())) {
        let _ = f.write_all(s.as_bytes());
        let _ = f.write_all(b"\n");
    }
}

// ───────────────────────── cam ptr 캡처 훅 (asm 스텁, Rust detour 없음) ─────────────────────────
// 스텁: push rax / mov rax,&CAM_PTR / mov [rax],rcx / pop rax / <원본 12B 재생> / jmp fn+0xc
// → 스택·인자 완전 보존(4개 초과 스택인자도 무관), 패닉 불가.
static CAM_PTR: AtomicUsize = AtomicUsize::new(0);
static HOOK_INSTALLED: AtomicBool = AtomicBool::new(false);
static HOOK_STATE: AtomicU32 = AtomicU32::new(0); // 0=미설치 1=OK 2=prologue mismatch 3=실패

unsafe fn install_cam_hook() {
    if HOOK_INSTALLED.swap(true, Ordering::Relaxed) { return; }
    let base = GetModuleHandleW(core::ptr::null());
    if base == 0 { HOOK_STATE.store(3, Ordering::Relaxed); return; }
    let fn_addr = base + VIEWGAME_RVA;
    let expect: [u8; 12] = [0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x53];
    for i in 0..12 {
        if *((fn_addr + i) as *const u8) != expect[i] {
            HOOK_STATE.store(2, Ordering::Relaxed);
            dbg_log(&format!("[hook] prologue mismatch @+{} (패치로 RVA 이동? migrate 필요) — 안개=뷰포트 고정 폴백", i));
            return;
        }
    }
    let stub = VirtualAlloc(0, 64, MEM_COMMIT_RESERVE, PAGE_EXECUTE_READWRITE);
    if stub == 0 { HOOK_STATE.store(3, Ordering::Relaxed); return; }
    let cam_slot = &CAM_PTR as *const AtomicUsize as usize;
    let mut s: Vec<u8> = Vec::new();
    s.push(0x50);                                          // push rax
    s.extend_from_slice(&[0x48, 0xB8]);                    // mov rax, imm64
    s.extend_from_slice(&cam_slot.to_le_bytes());
    s.extend_from_slice(&[0x48, 0x89, 0x08]);              // mov [rax], rcx
    s.push(0x58);                                          // pop rax
    s.extend_from_slice(&expect);                          // 훔친 프롤로그 12B 재생
    s.extend_from_slice(&[0x48, 0xB8]);                    // mov rax, fn+0xc
    s.extend_from_slice(&(fn_addr + 0xc).to_le_bytes());
    s.extend_from_slice(&[0xFF, 0xE0]);                    // jmp rax
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    let mut patch = [0u8; 12];
    patch[0] = 0x48; patch[1] = 0xB8;                      // mov rax, stub
    patch[2..10].copy_from_slice(&stub.to_le_bytes());
    patch[10] = 0xFF; patch[11] = 0xE0;                    // jmp rax
    let mut old: u32 = 0;
    if VirtualProtect(fn_addr, 12, PAGE_EXECUTE_READWRITE, &mut old) == 0 {
        HOOK_STATE.store(3, Ordering::Relaxed); return;
    }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), fn_addr as *mut u8, 12);
    VirtualProtect(fn_addr, 12, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), fn_addr, 12);
    HOOK_STATE.store(1, Ordering::Relaxed);
}

// ───────────────────────── 안전 read ─────────────────────────
fn readable(addr: usize, len: usize) -> bool {
    if addr < 0x10000 || addr >= (1usize << 47) { return false; }
    let mut mbi = MemBasicInfo::default();
    let r = unsafe { VirtualQuery(addr as *const _, &mut mbi, core::mem::size_of::<MemBasicInfo>()) };
    if r == 0 || mbi.state != 0x1000 { return false; } // MEM_COMMIT
    let p = mbi.protect & 0xFF;
    let ok = matches!(p, 0x02 | 0x04 | 0x08 | 0x20 | 0x40 | 0x80);
    ok && addr.wrapping_add(len) <= mbi.base.wrapping_add(mbi.region_size)
}
fn rd_f32(addr: usize) -> f32 { unsafe { core::ptr::read_volatile(addr as *const f32) } }

// ───────────────────────── 설정 ─────────────────────────
#[derive(Clone)]
struct FogCfg {
    enabled: bool,
    use_img: bool,      // true=fog_img(텍스처) / false=fog_col(반투명 색 폴백)
    auto_center: bool,  // 경기 진입 시 카메라 중심 = 안개 중심
    span_w: f32, span_h: f32,
    wx: f32, wy: f32, ww: f32, wh: f32, // auto_center=0 일 때 절대 월드 rect
    sx: f32, sy: f32,   // 카메라→화면 부호 (안개가 시점과 같은 방향으로 밀리면 뒤집기)
    k: f32,             // 추가 배율 (screen_px = world * zoom * k)
    col: Rgba,          // fog_col 색(런타임 적용, mode=col 일 때)
    debug: bool,
    // ── Skia flush 훅 (마일스톤1 검증) ──
    skia: bool,         // 1 = 렌더 파이프라인 훅 설치(위험, 기본0)
    skia_draw: bool,    // 0 = 덤프만(안전, 화면 안 건드림) / 1 = 실제 재-dispatch(테스트 사각형)
    skia_tag: u32,      // 캡처할 command tag(첫 해당 tag command). 기본 3
    skia_poff: u32,     // 재-dispatch 위치 write 오프셋(기본 0x78)
    skia_alpha: f32,    // 재-dispatch 전역 alpha(투명도). 1.0=불투명 ~ 0.0=투명
    sx0: f32, sy0: f32, sw0: f32, sh0: f32, // 테스트 사각형(논리좌표)
    pokes: Vec<(u32, f32)>, // "off:val,off:val" — 캡처 command 임의 오프셋에 f32 써넣기
    fog_mode: u32,   // 0=색(tag8) / 1=텍스처(tag3 PNG)
    tex_scale: f32,  // 텍스처 UV 크기(uw)
    scroll: u32,     // 0=화면고정 / 1=월드스크롤 / 2=진단(라이브 ctx)
    tex_x: f32, tex_y: f32, tex_dst: f32, // dst 위치·크기(조율)
    anim: bool,      // 1=12장 flipbook 애니메이션(fog_0..fog_11)
    anim_period: u32,// 프레임당 게임프레임 수(작을수록 빠름)
}
fn parse_pokes(v: &str) -> Vec<(u32, f32)> {
    let mut out = Vec::new();
    for part in v.split(',') {
        let part = part.trim();
        if let Some((o, val)) = part.split_once(':') {
            let o = o.trim();
            let off = if let Some(h) = o.strip_prefix("0x") { u32::from_str_radix(h, 16).ok() } else { o.parse::<u32>().ok() };
            if let (Some(off), Ok(fv)) = (off, val.trim().parse::<f32>()) { out.push((off, fv)); }
        }
    }
    out
}
impl Default for FogCfg {
    fn default() -> Self {
        FogCfg { enabled: true, use_img: false, auto_center: true,
                 span_w: 3840.0, span_h: 3840.0,
                 wx: -1920.0, wy: -1920.0, ww: 3840.0, wh: 3840.0,
                 sx: 1.0, sy: 1.0, k: 1.0,
                 col: Rgba::new(0.80, 0.85, 1.0, 0.45), // 옅은 푸른-흰 안개(fog_color 로 조절)
                 debug: true,
                 skia: false, skia_draw: false, skia_tag: 3, skia_poff: 0x78, skia_alpha: 1.0,
                 sx0: 100.0, sy0: 100.0, sw0: 300.0, sh0: 300.0, pokes: Vec::new(),
                 fog_mode: 1, tex_scale: 5.0, scroll: 1, tex_x: -500.0, tex_y: -500.0, tex_dst: 2600.0,
                 anim: false, anim_period: 6 }
    }
}
fn parse_hex_rgba(v: &str) -> Option<Rgba> {
    let h = v.trim().trim_start_matches('#');
    if h.len() != 8 { return None; }
    let n = u32::from_str_radix(h, 16).ok()?;
    Some(Rgba::new(
        ((n >> 24) & 0xff) as f32 / 255.0,
        ((n >> 16) & 0xff) as f32 / 255.0,
        ((n >> 8) & 0xff) as f32 / 255.0,
        (n & 0xff) as f32 / 255.0,
    ))
}
static CFG: Mutex<Option<FogCfg>> = Mutex::new(None);

fn load_cfg() -> FogCfg {
    let mut c = FogCfg::default();
    if let Ok(t) = fs::read_to_string(format!(r"{}\fog.cfg", my_dir())) {
        for line in t.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') { continue; }
            let Some((k, v)) = line.split_once('=') else { continue };
            let k = k.trim();
            // ★인라인 주석 제거: 값 뒤 '#'(과 그 뒤) 를 잘라냄. 단 값이 '#'로 시작하면(=#hex 색상) 보존.
            let v = v.trim();
            let v = if v.starts_with('#') { v } else { v.split('#').next().unwrap_or(v).trim() };
            let f = || v.parse::<f32>().ok();
            match k {
                "enabled" => c.enabled = v != "0",
                "mode" => c.use_img = v != "col",
                "auto_center" => c.auto_center = v != "0",
                "span_w" => { if let Some(x) = f() { c.span_w = x } }
                "span_h" => { if let Some(x) = f() { c.span_h = x } }
                "world_x" => { if let Some(x) = f() { c.wx = x } }
                "world_y" => { if let Some(x) = f() { c.wy = x } }
                "world_w" => { if let Some(x) = f() { c.ww = x } }
                "world_h" => { if let Some(x) = f() { c.wh = x } }
                "sign_x" => { if let Some(x) = f() { c.sx = if x < 0.0 { -1.0 } else { 1.0 } } }
                "sign_y" => { if let Some(x) = f() { c.sy = if x < 0.0 { -1.0 } else { 1.0 } } }
                "scale_k" => { if let Some(x) = f() { if x > 0.0 { c.k = x } } }
                "color" | "fog_color" => { if let Some(rgba) = parse_hex_rgba(v) { c.col = rgba } }
                "skia" => c.skia = v != "0",
                "skia_draw" => c.skia_draw = v != "0",
                "skia_tag" => { if let Ok(x) = v.parse::<u32>() { c.skia_tag = x } }
                "skia_alpha" => { if let Some(x) = f() { c.skia_alpha = x } }
                "skia_poke" => { c.pokes = parse_pokes(v); }
                "fog_mode" => { c.fog_mode = if v == "tex" { 1 } else if v == "col" { 0 } else { v.parse().unwrap_or(1) }; }
                "fog_tex_scale" => { if let Some(x) = f() { c.tex_scale = x } }
                "fog_scroll" => { c.scroll = v.parse().unwrap_or(1); }
                "fog_tex_x" => { if let Some(x) = f() { c.tex_x = x } }
                "fog_tex_y" => { if let Some(x) = f() { c.tex_y = x } }
                "fog_tex_dst" => { if let Some(x) = f() { c.tex_dst = x } }
                "fog_anim" => c.anim = v != "0",
                "fog_anim_period" => { if let Ok(x) = v.parse::<u32>() { c.anim_period = x } }
                "skia_poff" => { if let Ok(x) = u32::from_str_radix(v.trim_start_matches("0x"), if v.starts_with("0x") {16} else {10}) { c.skia_poff = x } }
                "skia_x" => { if let Some(x) = f() { c.sx0 = x } }
                "skia_y" => { if let Some(x) = f() { c.sy0 = x } }
                "skia_w" => { if let Some(x) = f() { c.sw0 = x } }
                "skia_h" => { if let Some(x) = f() { c.sh0 = x } }
                "debug" => c.debug = v != "0",
                _ => {}
            }
        }
    }
    c
}

// ───────────────────────── 프레임 로직 ─────────────────────────
static FRAME: AtomicUsize = AtomicUsize::new(0);
static AUTO_DONE: AtomicBool = AtomicBool::new(false);
static AUTO_CX: AtomicU32 = AtomicU32::new(0); // f32 bits
static AUTO_CY: AtomicU32 = AtomicU32::new(0);
static MISS: AtomicUsize = AtomicUsize::new(0); // fog 노드 연속 미발견 프레임(경기 이탈 감지)
static MATCH_FRAMES: AtomicUsize = AtomicUsize::new(0); // 경기화면 연속 프레임(안정화 지연용)
static ONCE_LOG: AtomicBool = AtomicBool::new(false); // 진단 1회 로그

fn fog_tick(ui: &mut GameUI) {
    let fr = FRAME.fetch_add(1, Ordering::Relaxed) + 1;
    // cfg 핫리로드 (60프레임마다)
    {
        let mut g = CFG.lock().unwrap_or_else(|e| e.into_inner());
        if g.is_none() || fr % 60 == 0 { *g = Some(load_cfg()); }
    }
    let cfg = { CFG.lock().unwrap_or_else(|e| e.into_inner()).clone().unwrap_or_default() };

    // Skia flush 훅 (cfg.skia=1 일 때만 설치, 멱등). SR_* = 테스트 사각형 파라미터.
    // ⚠ 재-dispatch(위험)는 경기화면(has_fog)에서만 켬 — 메뉴 크래시 방지(아래 has_fog 확정 후 SR_ON=true).
    SR_ON.store(false, Ordering::Relaxed);
    if cfg.skia {
        SR_X.store(cfg.sx0.to_bits(), Ordering::Relaxed);
        SR_Y.store(cfg.sy0.to_bits(), Ordering::Relaxed);
        SR_W.store(cfg.sw0.to_bits(), Ordering::Relaxed);
        SR_H.store(cfg.sh0.to_bits(), Ordering::Relaxed);
        SR_DRAW.store(cfg.skia_draw, Ordering::Relaxed);
        CAP_TAG.store(cfg.skia_tag, Ordering::Relaxed);
        POFF.store(cfg.skia_poff, Ordering::Relaxed);
        SR_ALPHA.store(cfg.skia_alpha.to_bits(), Ordering::Relaxed);
        FOG_R.store(cfg.col.r.to_bits(), Ordering::Relaxed);
        FOG_G.store(cfg.col.g.to_bits(), Ordering::Relaxed);
        FOG_B.store(cfg.col.b.to_bits(), Ordering::Relaxed);
        FOG_MODE.store(cfg.fog_mode, Ordering::Relaxed);
        TEX_SCROLL.store(cfg.scroll, Ordering::Relaxed);
        // ⚠ FOG_A / TEX_UW / TEX_X / TEX_Y / TEX_DST / TEX_ANIM / TEX_ANIM_PERIOD 는
        //   weather.cfg(weather_tick) 가 관리 — 여기서 fog.cfg 기본값을 쓰면 60프레임마다 잠깐
        //   잘못된 값으로 덮여 깜빡임 발생. 그래서 여기선 건드리지 않음.
        let pn = cfg.pokes.len().min(16);
        for i in 0..pn { POKE_OFF[i].store(cfg.pokes[i].0, Ordering::Relaxed); POKE_VAL[i].store(cfg.pokes[i].1.to_bits(), Ordering::Relaxed); }
        POKE_N.store(pn as u32, Ordering::Relaxed);
        unsafe { install_skia(); }
    }
    // 날씨(weather.cfg) 해석 → 지오메트리/속도/농도 오버라이드 + 변경 시 ingame.ui 로더 재생성.
    //   cfg.skia 블록(위)이 fog.cfg 기본값을 쓴 직후 같은 프레임에 덮어써 blip 없음. 메뉴서도 돎.
    if fr % 60 == 0 || WEATHER_SIG.lock().map(|g| g.is_empty()).unwrap_or(false) { weather_tick(); }
    if cfg.debug && fr % 120 == 0 && cfg.skia {
        dbg_log(&format!("[skia f{}] state={} SR_ON={} inject_cnt={} reject_cnt={} tmpl_locked={} world_cv={:#x}",
            fr, SKIA_STATE.load(Ordering::Relaxed), SR_ON.load(Ordering::Relaxed),
            INJECT_CNT.load(Ordering::Relaxed), REJECT_CNT.load(Ordering::Relaxed),
            TMPL_LOCKED.load(Ordering::Relaxed), WORLD_C.load(Ordering::Relaxed)));
    }

    // 하트비트: 모드 로드/훅 상태/override 적용 여부 진단 (조기리턴 前, 무조건).
    let has_fog = find(&ui.root, "fog_col").is_some();
    if cfg.debug && fr % 120 == 0 {
        dbg_log(&format!("[hb f{}] hook_state={} cam={:#x} fog_col_node={} root_children={}",
            fr, HOOK_STATE.load(Ordering::Relaxed), CAM_PTR.load(Ordering::Relaxed),
            has_fog, ui.root.child.len()));
    }

    // fog 노드 존재 = 경기(관전) 화면. ⚠ has_fog 는 가끔 깜빡임(UI 트리 재구성) → 단발 부재로
    //   초반지연 카운터를 리셋하면 안 됨. 60프레임 연속 부재(=진짜 경기이탈)에만 리셋.
    if !has_fog {
        SR_ON.store(false, Ordering::Relaxed);
        let m = MISS.fetch_add(1, Ordering::Relaxed) + 1;
        if m >= 60 { INJECT_SEEN.store(0, Ordering::Relaxed); } // 지속적 이탈 → 다음 경기서 다시 지연
        return;
    }
    MISS.store(0, Ordering::Relaxed);
    // 경기화면 → 안개 인젝트 허용. 초반 크래시는 inject_live 의 초반지연(INJECT_SEEN)이 담당.
    if cfg.skia { SR_ON.store(true, Ordering::Relaxed); }
}

// ═══════════════ Skia flush 훅 (마일스톤1: draw 파이프 개입 검증) ═══════════════
// dispatcher FUN_1409cd2b0(0x9cd2b0): (rcx=ctx, rdx=state, r8=SkCanvas, r9=RenderCommand, [stack]=alpha)
// draw loop FUN_1409d7a60(0x9d7a60): 프레임당 1회, 내부서 dispatcher 반복 호출.
// 관찰스텁 = 매 dispatch 의 ctx/state/canvas 저장 + 프레임 첫 command 를 템플릿으로 캡처(레지스터/스택 완전보존).
// loop 트램폴린 = 원본(전부 draw) 후 템플릿 복사본을 좌표만 바꿔 재-dispatch → 화면 최상단에 사각형.
const DISP_RVA: usize = 0x9cd2b0;
const LOOP_RVA: usize = 0x9d7a60;
static SKIA_INSTALLED: AtomicBool = AtomicBool::new(false);
static SKIA_STATE: AtomicU32 = AtomicU32::new(0); // 0=미설치 1=OK 2=prologue mismatch 3=실패
static CAP_CTX: AtomicUsize = AtomicUsize::new(0);
static CAP_RDX: AtomicUsize = AtomicUsize::new(0);
static CAP_CANVAS: AtomicUsize = AtomicUsize::new(0);
static CAP_TMPL: AtomicUsize = AtomicUsize::new(0);
static CAP_ARMED: AtomicU8 = AtomicU8::new(0);
static DISP_ENTRY: AtomicUsize = AtomicUsize::new(0);
static DISP_TRAMP2: AtomicUsize = AtomicUsize::new(0);
static LOOP_TRAMP2: AtomicUsize = AtomicUsize::new(0);
static SKIA_FRAMES: AtomicUsize = AtomicUsize::new(0);
static INJECT_CNT: AtomicUsize = AtomicUsize::new(0);  // 실제 draw 한 수(진단)
static REJECT_CNT: AtomicUsize = AtomicUsize::new(0);  // 초반지연/거른 수(진단)
static INJECT_SEEN: AtomicUsize = AtomicUsize::new(0); // 경기화면서 inject_live 도달 프레임(초반지연용)
// ★고정 템플릿: 안정적 tag8 하나를 한 번만 캡처해 재사용(가변 게임플레이 command 회피 = 크래시 방지).
static TMPL_LOCKED: AtomicBool = AtomicBool::new(false);
static mut TMPL_BUF: [u8; 0x120] = [0u8; 0x120];
// ★텍스처 안개(tag3/4): 내 PNG 참조하는 command(키="tfm2_fog")를 한 번 캡처해 재사용.
static TEX_LOCKED: AtomicBool = AtomicBool::new(false);
static mut TEX_BUF: [u8; 0x120] = [0u8; 0x120];
// ★딥카피 문자열 버퍼: 캡처 시 key(+0x10)·property(+0x28) 문자열을 여기로 복사하고
//   TEX_BUF의 포인터를 이 안정버퍼로 fixup → 다음 프레임 tag-8 그리기 때도 dangling 없음.
static mut TEX_KEYBUF: [u8; 256] = [0u8; 256];
static mut TEX_STR2BUF: [u8; 256] = [0u8; 256];
static TEX_KEY_LOGGED: AtomicBool = AtomicBool::new(false);
static FOG_MODE: AtomicU32 = AtomicU32::new(1);  // 0=색(tag8) / 1=텍스처(tag3). cfg fog_mode
static TEX_UW: AtomicU32 = AtomicU32::new(0);    // 텍스처 UV 크기(uw=vh). cfg fog_tex_scale
static TEX_SCROLL: AtomicU32 = AtomicU32::new(0);// 0=화면고정 1=월드스크롤 2=진단(라이브ctx)
static TEX_X: AtomicU32 = AtomicU32::new(0);     // dst 위치 x(+0x78). cfg fog_tex_x
static TEX_Y: AtomicU32 = AtomicU32::new(0);     // dst 위치 y(+0x7c). cfg fog_tex_y
static TEX_DST: AtomicU32 = AtomicU32::new(0);   // dst 크기(+0x80/+0x84). cfg fog_tex_dst
static INJECT_TAG: AtomicU32 = AtomicU32::new(0xff);   // 복사한 command 의 tag(진단)
static SR_X: AtomicU32 = AtomicU32::new(0);
static SR_Y: AtomicU32 = AtomicU32::new(0);
static SR_W: AtomicU32 = AtomicU32::new(0);
static SR_H: AtomicU32 = AtomicU32::new(0);
static SR_ON: AtomicBool = AtomicBool::new(false);
static SR_DRAW: AtomicBool = AtomicBool::new(false); // false=덤프만(안전) / true=재-dispatch
static SKIA_DUMPED: AtomicU32 = AtomicU32::new(0);    // 덤프한 command 수(여러 개 샘플)
static DISP_CNT: AtomicU32 = AtomicU32::new(0);      // 프레임 내 dispatch 카운터(스텁이 증가)
static CAP_TAG: AtomicU32 = AtomicU32::new(3);       // 캡처할 command tag(byte[r9+0]). cfg skia_tag
static DISP_TOTAL: AtomicU32 = AtomicU32::new(0);    // 직전 프레임 총 command 수
static POFF: AtomicU32 = AtomicU32::new(0x78);       // 재-dispatch 시 위치 write 오프셋(cfg skia_poff)
static SR_ALPHA: AtomicU32 = AtomicU32::new(0x3f800000); // 재-dispatch 5번째 인자(전역 alpha, f32 bits). 기본 1.0
// ★안개 색(RGBA, f32 bits). inject_live 가 tag8 빔 색으로 씀. 기본 옅은 청백 45%.
static FOG_R: AtomicU32 = AtomicU32::new(0);
static FOG_G: AtomicU32 = AtomicU32::new(0);
static FOG_B: AtomicU32 = AtomicU32::new(0);
static FOG_A: AtomicU32 = AtomicU32::new(0);
// ★안정적 월드 캔버스(맵 지형 tag4 의 ctx/rdx/canvas). base 서피스라 캐시 dangling 없음 → 크래시 회피.
static WORLD_A: AtomicUsize = AtomicUsize::new(0);
static WORLD_B: AtomicUsize = AtomicUsize::new(0);
static WORLD_C: AtomicUsize = AtomicUsize::new(0);  // tag-3(캐릭터) 시점의 Assets 컨테이너(r8)
// ★fog_tex(UI) command 시점의 Assets 컨테이너(r8). 내 텍스처는 여기 로드돼 있을 것.
//   tag-8 재-dispatch 때 이 컨테이너로 조회하면 텍스처를 찾음(월드 캔버스에 그리되 UI 컨테이너 참조).
static TEX_CONTAINER: AtomicUsize = AtomicUsize::new(0);
static CONTAINER_LOGGED: AtomicBool = AtomicBool::new(false);
static LOOKUP_FN: AtomicUsize = AtomicUsize::new(0);   // FUN_140a052b0(container,key_ptr,key_len)->핸들|NULL
static LOOKUP_LOGGED: AtomicBool = AtomicBool::new(false);

// ★애니메이션(flipbook): 활성 날씨의 이미지들을 프레임마다 교체. config(weather.cfg) 주도.
const MAX_FRAMES: usize = 24;                              // 한 날씨 최대 장수(로더 노드 풀 크기와 일치)
static TEX_ANIM: AtomicBool = AtomicBool::new(false);      // 장수>1 이면 애니메이션
static TEX_ANIM_PERIOD: AtomicU32 = AtomicU32::new(18);    // 장당 게임프레임 수(클수록 느림)
static ANIM_TICK: AtomicUsize = AtomicUsize::new(0);       // detour_loop 가 매 프레임 증가
static ANIM_FRAMES: AtomicU32 = AtomicU32::new(0);         // 현재 활성 날씨 장수(config)
static mut TEX_KEYBUFS: [[u8; 128]; MAX_FRAMES] = [[0u8; 128]; MAX_FRAMES]; // 이미지 키 버퍼(정본, 로더+사이클 공용)
static mut TEX_KEYLENS: [usize; MAX_FRAMES] = [0usize; MAX_FRAMES];
static TEX_KEYS_BUILT: AtomicBool = AtomicBool::new(false);

const KEYBUF_ROW: usize = 128;
// 풀 키버퍼 i번째 행 시작 포인터(static mut 참조 없이).
unsafe fn keybuf_row(i: usize) -> *mut u8 { (core::ptr::addr_of_mut!(TEX_KEYBUFS) as *mut u8).add(i * KEYBUF_ROW) }
unsafe fn keylen_get(i: usize) -> usize { core::ptr::read((core::ptr::addr_of!(TEX_KEYLENS) as *const usize).add(i)) }
unsafe fn keylen_set(i: usize, v: usize) { core::ptr::write((core::ptr::addr_of_mut!(TEX_KEYLENS) as *mut usize).add(i), v); }

// config 이미지 이름 목록 → 풀 키버퍼에 "asset/tfm2_fog/ui/<name>" 로 구성. 반환=장수(clamp).
unsafe fn build_keys_from(images: &[String]) -> usize {
    const PREFIX: &[u8] = b"asset/tfm2_fog/ui/";
    let n = images.len().min(MAX_FRAMES);
    for i in 0..n {
        let name = images[i].trim().as_bytes();
        let total = PREFIX.len() + name.len();
        let row = keybuf_row(i);
        if total > KEYBUF_ROW { keylen_set(i, 0); continue; }
        core::ptr::copy_nonoverlapping(PREFIX.as_ptr(), row, PREFIX.len());
        core::ptr::copy_nonoverlapping(name.as_ptr(), row.add(PREFIX.len()), name.len());
        keylen_set(i, total);
    }
    ANIM_FRAMES.store(n as u32, Ordering::Relaxed);
    TEX_ANIM.store(n > 1, Ordering::Relaxed);
    TEX_KEYS_BUILT.store(n > 0, Ordering::Relaxed);
    n
}

// ImageRunner 인스턴스의 source(에셋키)를 런타임 교체 — dp+0=len, dp+8=ptr (scrim 검증 레이아웃).
//  ptr 를 정적 키버퍼로 지정(누수 없음). 로더 노드가 렌더되면 그 텍스처가 Assets 맵에 로드됨.
unsafe fn set_img_source_ptr(n: &mut Node, ptr: usize, len: usize) -> bool {
    if !n.runner.type_name().contains("ImageRunner") { return false; }
    let any: &dyn std::any::Any = n.runner.as_any();
    let parts: [usize; 2] = core::mem::transmute::<*const dyn std::any::Any, [usize; 2]>(any as *const dyn std::any::Any);
    let dp = parts[0];
    if dp < 0x10000 { return false; }
    core::ptr::write_unaligned((dp + 8) as *mut u64, ptr as u64);
    core::ptr::write_unaligned(dp as *mut u64, len as u64);
    true
}

// ── weather.cfg (모더 편집): 날씨 라이브러리 + 활성 선택 ──
#[derive(Default)]
struct Weather { name: String, images: Vec<String>,
                 period: Option<u32>, alpha: Option<f32>, size: Option<f32>,
                 x: Option<f32>, y: Option<f32>, scale: Option<f32> }
struct WeatherCfg { active: String, period: u32, alpha: f32, size: f32,
                    x: f32, y: f32, scale: f32, weathers: Vec<Weather> }
impl Default for WeatherCfg {
    fn default() -> Self {
        WeatherCfg { active: String::new(), period: 18, alpha: 0.25, size: 6400.0,
                     x: -160.0, y: -160.0, scale: 5.0, weathers: Vec::new() }
    }
}
// alpha 는 0~255 또는 0~1 둘 다 허용(>1 이면 /255).
fn parse_alpha(v: &str) -> Option<f32> {
    let x = v.trim().parse::<f32>().ok()?;
    Some(if x > 1.0 { x / 255.0 } else { x })
}
fn parse_weather_cfg(text: &str) -> WeatherCfg {
    let mut c = WeatherCfg::default();
    let mut cur: Option<Weather> = None;
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') || line.starts_with("//") { continue; }
        if line.starts_with('[') && line.ends_with(']') {
            if let Some(w) = cur.take() { c.weathers.push(w); }
            let name = line[1..line.len()-1].trim().to_string();
            cur = Some(Weather { name, ..Default::default() });
            continue;
        }
        let Some((k, v)) = line.split_once('=') else { continue };
        let k = k.trim().to_ascii_lowercase();
        let v = { let v = v.trim(); if v.starts_with('#') { v } else { v.split('#').next().unwrap_or(v).trim() } };
        match cur {
            None => match k.as_str() { // 전역 기본값
                "active" => c.active = v.to_string(),
                "period" => { if let Ok(x) = v.parse() { c.period = x } }
                "alpha"  => { if let Some(x) = parse_alpha(v) { c.alpha = x } }
                "size"   => { if let Ok(x) = v.parse() { c.size = x } }
                "x"      => { if let Ok(x) = v.parse() { c.x = x } }
                "y"      => { if let Ok(x) = v.parse() { c.y = x } }
                "scale"  => { if let Ok(x) = v.parse() { c.scale = x } }
                _ => {}
            },
            Some(ref mut w) => match k.as_str() { // 날씨별
                "img" | "image" => w.images.push(v.to_string()),
                "period" => w.period = v.parse().ok(),
                "alpha"  => w.alpha = parse_alpha(v),
                "size"   => w.size = v.parse().ok(),
                "x"      => w.x = v.parse().ok(),
                "y"      => w.y = v.parse().ok(),
                "scale"  => w.scale = v.parse().ok(),
                _ => {}
            },
        }
    }
    if let Some(w) = cur.take() { c.weathers.push(w); }
    c
}
static WEATHER_SIG: Mutex<String> = Mutex::new(String::new()); // 활성 날씨+이미지 시그(변경 감지)

// weather.cfg 로드 → 활성 날씨 해석 → 지오메트리/알파/속도/키버퍼 반영 + (변경 시) ingame.ui 로더 재생성.
//  ★텍스처는 .ui 파싱 때 preload 되므로, 로더 노드 source 는 빌드타임에 박혀 있어야 함(런타임 교체 불가).
//   → 모드가 활성 날씨 이미지들을 ingame.ui 로더 노드로 자동 생성. 변경 시 게임 재시작하면 로드됨.
fn weather_tick() {
    let text = match fs::read_to_string(format!(r"{}\weather.cfg", my_dir())) { Ok(t) => t, Err(_) => return };
    let wc = parse_weather_cfg(&text);
    let w = wc.weathers.iter().find(|w| w.name == wc.active).or_else(|| wc.weathers.first());
    let Some(w) = w else { return };
    let period = w.period.unwrap_or(wc.period).max(1);
    let alpha  = w.alpha.unwrap_or(wc.alpha).clamp(0.0, 1.0);
    let size   = w.size.unwrap_or(wc.size);
    let x      = w.x.unwrap_or(wc.x);
    let y      = w.y.unwrap_or(wc.y);
    let scale  = w.scale.unwrap_or(wc.scale);
    TEX_ANIM_PERIOD.store(period, Ordering::Relaxed);
    FOG_A.store(alpha.to_bits(), Ordering::Relaxed);
    TEX_DST.store(size.to_bits(), Ordering::Relaxed);
    TEX_X.store(x.to_bits(), Ordering::Relaxed);
    TEX_Y.store(y.to_bits(), Ordering::Relaxed);
    TEX_UW.store(scale.to_bits(), Ordering::Relaxed);
    // 이미지 목록 변경 시: 키버퍼 재구성 + ingame.ui 로더 재생성.
    let sig = format!("{}|{}", w.name, w.images.join(","));
    let mut last = WEATHER_SIG.lock().unwrap_or_else(|e| e.into_inner());
    if *last == sig { return; }
    *last = sig;
    let count = unsafe { build_keys_from(&w.images) };
    regen_ui_loaders(&w.images);
    dbg_log(&format!("[weather] active='{}' frames={} period={} alpha={:.2} size={} — .ui 갱신(재시작 후 로드)", w.name, count, period, alpha, size));
}

// ingame.ui 의 로더 노드(#fog_ld*) 를 활성 날씨 이미지들로 재생성. #fog_col 마커 뒤를 통째 교체.
//  텍스처는 이 파일이 파싱될 때 preload 되므로 다음 게임 실행/경기진입에 반영됨.
fn regen_ui_loaders(images: &[String]) {
    let path = format!(r"{}\ui\layout\ingame.ui", my_dir());
    let Ok(text) = fs::read_to_string(&path) else { return };
    // #fog_col 마커 줄 끝까지 보존, 그 뒤(기존 로더 + 루트 닫는 '}')는 새로 씀.
    let Some(pos) = text.find("#fog_col") else { return };
    let line_end = text[pos..].find('\n').map(|e| pos + e + 1).unwrap_or(text.len());
    let head = &text[..line_end];
    let mut want = String::new();
    for (i, name) in images.iter().take(MAX_FRAMES).enumerate() {
        want.push_str(&format!(
            "  #fog_ld{}:image {{ x: 18px; y: 64px; width: 960px; height: 960px; source: \"asset/tfm2_fog/ui/{}\"; color: #ffffff00; ignore_event: true; }}\n",
            i, name.trim()));
    }
    let new_text = format!("{}{}}}\n", head, want);
    // 안전성: 루트/마커 보존 확인, 변경 시에만 기록(BOM 없는 UTF-8).
    if new_text != text && new_text.starts_with("ingame:") && new_text.contains("#fog_col") {
        let _ = fs::write(&path, new_text);
    }
}

// 텍스처 맵 조회(shadow-call). container 에 key 가 로드돼 있으면 핸들 반환, 없으면 0.
unsafe fn tex_lookup(container: usize, kptr: usize, klen: usize) -> usize {
    let fnp = LOOKUP_FN.load(Ordering::Relaxed);
    if fnp == 0 || container < 0x10000 || kptr < 0x10000 || klen == 0 { return usize::MAX; }
    let f: extern "win64" fn(usize, usize, usize) -> usize = core::mem::transmute(fnp);
    f(container, kptr, klen)
}
static DREW_FRAME: AtomicBool = AtomicBool::new(false); // 이 프레임 안개 그렸나(1회 제한)
// 범용 poke: 캡처 command 의 임의 오프셋에 f32 써넣기(크기/위치/색 조율, 재빌드 불필요).
static POKE_OFF: [AtomicU32; 16] = [const { AtomicU32::new(0) }; 16];
static POKE_VAL: [AtomicU32; 16] = [const { AtomicU32::new(0) }; 16];
static POKE_N: AtomicU32 = AtomicU32::new(0);

extern "win64" fn detour_loop(a: usize, b: usize, c: usize, d: usize) -> usize {
    DREW_FRAME.store(false, Ordering::Relaxed); // 이 프레임 안개 그리기 재무장
    ANIM_TICK.fetch_add(1, Ordering::Relaxed);  // 애니메이션 프레임 카운터(프레임당 1)
    let tr = LOOP_TRAMP2.load(Ordering::Relaxed);
    if tr == 0 { return 0; }
    let f: extern "win64" fn(usize, usize, usize, usize) -> usize = unsafe { core::mem::transmute(tr) };
    f(a, b, c, d) // 원본: 안개는 disp_detour 가 루프 중간에 그림
}

// dispatcher detour: command 1개 그릴 때마다 호출.
//  - tag3(캐릭터/스프라이트) = 메인 월드 서피스 캔버스 캡처(보임+안정).
//  - tag8(체력바) = 캐릭터 다 그려진 시점 → 이때 캡처한 메인 캔버스로 안개를 루프 중간에 1회 그림
//    (중간이라 화면에 보이고, 캐릭터 위·HUD 아래, 안정 캔버스라 크래시 없음).
extern "win64" fn disp_detour(a: usize, b: usize, c: usize, d: usize, e: f32) -> usize {
    let tr = DISP_TRAMP2.load(Ordering::Relaxed);
    if tr == 0 { return 0; }
    let f: extern "win64" fn(usize, usize, usize, usize, f32) -> usize = unsafe { core::mem::transmute(tr) };
    let ret = f(a, b, c, d, e); // 원본 command draw
    if d > 0x10000 && d < (1usize << 48) {
        let v = unsafe { core::ptr::read_volatile(d as *const u64) };
        let tag = if v & 0x8000_0000_0000_0000 != 0 { (v & 0xff) as u32 } else { 7 };
        if tag == CAP_TAG.load(Ordering::Relaxed) {
            WORLD_A.store(a, Ordering::Relaxed);
            WORLD_B.store(b, Ordering::Relaxed);
            WORLD_C.store(c, Ordering::Relaxed);
        }
        // ★텍스처 안개: 내 키("tfm2_fog") command 가 지나가는 이 순간(라이브=모든 포인터 유효)에
        //   그 command 를 복사→지오메트리만 변조→월드 캔버스에 그림. 라이브라 dangling 없음, 월드캔버스라 보임.
        if (tag == 3 || tag == 4) && FOG_MODE.load(Ordering::Relaxed) == 1 {
            let kptr = unsafe { *((d + 0x10) as *const usize) };
            let klen = unsafe { *((d + 0x18) as *const usize) };
            if kptr > 0x10000 && klen > 4 && klen < 256 && readable(kptr, klen) {
                let key = unsafe { core::slice::from_raw_parts(kptr as *const u8, klen) };
                if key.windows(8).any(|w| w == b"tfm2_fog") {
                    // ★이 command 의 Assets 컨테이너(c=r8) = 내 텍스처가 로드된 곳.
                    TEX_CONTAINER.store(c, Ordering::Relaxed);
                    if !CONTAINER_LOGGED.swap(true, Ordering::Relaxed) {
                        // ★fog_tex 시점 라이브 조회 — 이 command 자신의 lookup 이 성공하는가?
                        let h = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe { tex_lookup(c, kptr, klen) })).unwrap_or(0);
                        dbg_log(&format!("[tex] fog_tex 컨테이너(c)={:#x} 같나={} / 라이브lookup={:#x} ({})",
                            c, c == WORLD_C.load(Ordering::Relaxed), h,
                            if h == 0 {"NULL=텍스처 미로드!"} else if h == usize::MAX {"call-skip"} else {"로드됨✓"}));
                    }
                    if !TEX_KEY_LOGGED.swap(true, Ordering::Relaxed) {
                        let mut s = format!("[tex] key='{}' tag={}\n", String::from_utf8_lossy(key), tag);
                        // 전체 필드 덤프(u64 hex + f32) — from-scratch 구성용.
                        let mut o = 0usize;
                        while o < 0x120 {
                            let q = unsafe { core::ptr::read_unaligned((d + o) as *const u64) };
                            let f0 = unsafe { core::ptr::read_unaligned((d + o) as *const f32) };
                            let f1 = unsafe { core::ptr::read_unaligned((d + o + 4) as *const f32) };
                            s.push_str(&format!("  +{:#04x}: {:016x}  ({:.3}, {:.3})\n", o, q, f0, f1));
                            o += 8;
                        }
                        dbg_log(&s);
                    }
                    // ★캡처만: fog_tex 시점은 UI 레이어(월드 캔버스 마감됨)라 여기서 그리면 안 보임.
                    //   command+문자열을 안정버퍼로 딥카피해두고, 실제 그리기는 다음 프레임 tag-8 시점에.
                    if !TEX_LOCKED.load(Ordering::Relaxed) && readable(d, 0x120) {
                        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe { capture_tex(d) }));
                    }
                }
            }
        }
        if tag == 8 {
            let mode = FOG_MODE.load(Ordering::Relaxed);
            if mode == 0 && !TMPL_LOCKED.load(Ordering::Relaxed) {
                unsafe { core::ptr::copy_nonoverlapping(d as *const u8, core::ptr::addr_of_mut!(TMPL_BUF) as *mut u8, 0x120); }
                TMPL_LOCKED.store(true, Ordering::Relaxed);
            }
            // 첫 tag8 = 캐릭터 다 그려진 시점(월드 캔버스 살아있음) → 여기서 안개 1회.
            //   색모드=tag8 빔, 텍스처모드=딥카피해둔 tex command 를 월드 캔버스에 재-dispatch.
            if SR_ON.load(Ordering::Relaxed) && SR_DRAW.load(Ordering::Relaxed)
                && !DREW_FRAME.swap(true, Ordering::Relaxed) {
                let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe {
                    if mode == 1 { draw_tex_captured(); } else { inject_fog(); }
                }));
            }
        }
    }
    ret
}

// ★캡처: 라이브 fog_tex command(포인터 전부 유효)를 안정버퍼로 딥카피.
//   key(+0x10/+0x18)·property(+0x28/+0x30) 문자열을 static 버퍼로 복사하고 TEX_BUF의 포인터를 fixup
//   → 다음 프레임 tag-8에 재사용해도 dangling 없음. (+0x38 미지 포인터는 원본 유지=최대 1프레임 stale)
unsafe fn capture_tex(cmd_src: usize) {
    if !readable(cmd_src, 0x120) { return; }
    core::ptr::copy_nonoverlapping(cmd_src as *const u8, core::ptr::addr_of_mut!(TEX_BUF) as *mut u8, 0x120);
    let base = core::ptr::addr_of_mut!(TEX_BUF) as *mut u8;
    // key 문자열 딥카피(+0x10 ptr / +0x18 len)
    let kp = core::ptr::read_unaligned(base.add(0x10) as *const usize);
    let kl = core::ptr::read_unaligned(base.add(0x18) as *const usize);
    if kp > 0x10000 && kl > 0 && kl < 256 && readable(kp, kl) {
        core::ptr::copy_nonoverlapping(kp as *const u8, core::ptr::addr_of_mut!(TEX_KEYBUF) as *mut u8, kl);
        core::ptr::write_unaligned(base.add(0x10) as *mut usize, core::ptr::addr_of!(TEX_KEYBUF) as usize);
    }
    // property 문자열 딥카피(+0x28 ptr / +0x30 len)
    let sp = core::ptr::read_unaligned(base.add(0x28) as *const usize);
    let sl = core::ptr::read_unaligned(base.add(0x30) as *const usize);
    if sp > 0x10000 && sl > 0 && sl < 256 && readable(sp, sl) {
        core::ptr::copy_nonoverlapping(sp as *const u8, core::ptr::addr_of_mut!(TEX_STR2BUF) as *mut u8, sl);
        core::ptr::write_unaligned(base.add(0x28) as *mut usize, core::ptr::addr_of!(TEX_STR2BUF) as usize);
    }
    TEX_LOCKED.store(true, Ordering::Relaxed);
    dbg_log("[tex] captured & fixed up → tag-8 시점에 그림");
}

// ★그리기: 딥카피해둔 tex command 를 tag-8 시점(월드 캔버스 살아있음)에 월드 캔버스로 재-dispatch.
unsafe fn draw_tex_captured() {
    if !TEX_LOCKED.load(Ordering::Relaxed) { return; }
    let seen = INJECT_SEEN.fetch_add(1, Ordering::Relaxed);
    if seen < 60 { REJECT_CNT.fetch_add(1, Ordering::Relaxed); return; }
    // ctx=WORLD_A(rcx), canvas=WORLD_B(rdx, 월드 캔버스=미니맵 아래), container=r8.
    // ★container 는 fog_tex(UI) 시점의 것(TEX_CONTAINER)을 씀 — 내 텍스처가 로드된 컨테이너.
    //   없으면 WORLD_C 폴백.
    let ctx = WORLD_A.load(Ordering::Relaxed);
    let st = WORLD_B.load(Ordering::Relaxed);
    let mut cont = TEX_CONTAINER.load(Ordering::Relaxed);
    if cont < 0x10000 { cont = WORLD_C.load(Ordering::Relaxed); }
    let cv = cont;
    if ctx < 0x10000 || st < 0x10000 || cv < 0x10000 { return; }
    // ★tag-8 시점에 내 텍스처가 이 컨테이너에 있나? (딥카피 키로 조회)
    if !LOOKUP_LOGGED.swap(true, Ordering::Relaxed) {
        let kp = core::ptr::addr_of!(TEX_KEYBUF) as usize;
        let kl = core::ptr::read_unaligned((core::ptr::addr_of!(TEX_BUF) as usize + 0x18) as *const usize);
        let h = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| tex_lookup(cv, kp, kl))).unwrap_or(0);
        dbg_log(&format!("[tex] tag-8 lookup cont={:#x} klen={} => {:#x} ({})",
            cv, kl, h, if h == 0 {"NULL=여기선 미로드"} else if h == usize::MAX {"call-skip"} else {"로드됨✓"}));
    }
    let mut buf = [0u8; 0x120];
    core::ptr::copy_nonoverlapping(core::ptr::addr_of!(TEX_BUF) as *const u8, buf.as_mut_ptr(), 0x120);
    let p = buf.as_mut_ptr();
    // ★애니메이션: 프레임마다 키를 fog_0..fog_11 로 교체(텍스처 자체가 바뀜 = flipbook).
    //   각 프레임 텍스처는 이미 로더노드로 상주 로드됨. 키만 갈아끼우면 그 텍스처로 그려짐.
    let nframes = ANIM_FRAMES.load(Ordering::Relaxed) as usize;
    if TEX_ANIM.load(Ordering::Relaxed) && TEX_KEYS_BUILT.load(Ordering::Relaxed) && nframes > 0 {
        let mut period = TEX_ANIM_PERIOD.load(Ordering::Relaxed) as usize;
        if period == 0 { period = 1; }
        let frame = (ANIM_TICK.load(Ordering::Relaxed) / period) % nframes;
        let klen = keylen_get(frame);
        if klen > 0 {
            core::ptr::write_unaligned(p.add(0x10) as *mut usize, keybuf_row(frame) as usize);
            core::ptr::write_unaligned(p.add(0x18) as *mut usize, klen);
        }
        // 진단: 순환 확인 — 프레임 인덱스만 로그(shadow-call 없음=안전). 30 draw 마다.
        let dc = INJECT_CNT.load(Ordering::Relaxed);
        if dc % 30 == 0 {
            dbg_log(&format!("[anim] frame={}/{} period={} tick={}", frame, nframes, period, ANIM_TICK.load(Ordering::Relaxed)));
        }
    }
    let mut uw = f32::from_bits(TEX_UW.load(Ordering::Relaxed));
    if !(uw > 0.0) { uw = 1.0; }
    let (mut u0, mut v0) = (0.0f32, 0.0f32);
    if TEX_SCROLL.load(Ordering::Relaxed) == 1 {
        let cam = CAM_PTR.load(Ordering::Relaxed);
        if cam != 0 && readable(cam + CAM_CX, 0x140) {
            let cx = rd_f32(cam + CAM_CX); let cy = rd_f32(cam + CAM_CY);
            if cx.is_finite() && cy.is_finite() { u0 = cx * 0.001; v0 = cy * 0.001; }
        }
    }
    let tx = f32::from_bits(TEX_X.load(Ordering::Relaxed));
    let ty = f32::from_bits(TEX_Y.load(Ordering::Relaxed));
    let mut td = f32::from_bits(TEX_DST.load(Ordering::Relaxed));
    if !(td > 0.0) { td = 2600.0; }
    *(p.add(0x68) as *mut f32) = u0;   *(p.add(0x6c) as *mut f32) = v0;   // UV 원점(스크롤)
    *(p.add(0x70) as *mut f32) = uw;   *(p.add(0x74) as *mut f32) = uw;   // UV 크기
    *(p.add(0x78) as *mut f32) = tx;   *(p.add(0x7c) as *mut f32) = ty;   // dst 위치
    *(p.add(0x80) as *mut f32) = td;   *(p.add(0x84) as *mut f32) = td;   // dst 크기
    INJECT_CNT.fetch_add(1, Ordering::Relaxed);
    let tr = DISP_TRAMP2.load(Ordering::Relaxed);
    if tr == 0 { return; }
    let f: extern "win64" fn(usize, usize, usize, usize, f32) -> usize = core::mem::transmute(tr);
    // ★5번째 인자=전역 alpha(불투명도). 텍스처는 이걸 곱함 — 0.0이면 완전투명(=안 보임).
    //   FOG_A(cfg fog_color 의 알파, 0~1)로 안개 진하기 조절, 미설정 시 1.0(텍스처 자체 알파).
    let mut ga = f32::from_bits(FOG_A.load(Ordering::Relaxed));
    if !(ga > 0.0) { ga = 1.0; }
    f(ctx, st, cv, buf.as_ptr() as usize, ga);
}

unsafe fn inject_fog() {
    if !SR_ON.load(Ordering::Relaxed) || !SR_DRAW.load(Ordering::Relaxed) { return; }
    // 경기 초반(인트로/전환) 건너뜀 — 안정화 후에만.
    let seen = INJECT_SEEN.fetch_add(1, Ordering::Relaxed);
    if seen < 60 { REJECT_CNT.fetch_add(1, Ordering::Relaxed); return; }
    // 안정적 월드 캔버스. 없으면 스킵.
    let ctx = WORLD_A.load(Ordering::Relaxed);
    let st = WORLD_B.load(Ordering::Relaxed);
    let cv = WORLD_C.load(Ordering::Relaxed);
    if ctx < 0x10000 || cv < 0x10000 { return; }
    // ─── 색 모드(tag8 빔) ───
    if !TMPL_LOCKED.load(Ordering::Relaxed) { return; }
    let mut buf = [0u8; 0x120];
    core::ptr::copy_nonoverlapping(core::ptr::addr_of!(TMPL_BUF) as *const u8, buf.as_mut_ptr(), 0x120);
    INJECT_CNT.fetch_add(1, Ordering::Relaxed);
    let p = buf.as_mut_ptr();
    // ★기본 안개 지오메트리 = tag8 가로 빔(끝점 x -3000~3000 @ y500, 두께 3000). 게임 뷰포트 클립이
    //   정확히 뷰포트로 잘라줌 → 해상도/좌표계 무관하게 뷰포트 꽉 채움. (payload 오프셋: 끝점 0x28~0x34, 두께 0x38)
    *(p.add(0x28) as *mut f32) = -3000.0; *(p.add(0x2c) as *mut f32) = 500.0;
    *(p.add(0x30) as *mut f32) =  3000.0; *(p.add(0x34) as *mut f32) = 500.0;
    *(p.add(0x38) as *mut f32) =  3000.0;
    // 색(gradient 두 스톱 동일 = solid). color0 @0x08, color1 @0x18. RGBA.
    let (r, g, b, a) = (f32::from_bits(FOG_R.load(Ordering::Relaxed)),
                        f32::from_bits(FOG_G.load(Ordering::Relaxed)),
                        f32::from_bits(FOG_B.load(Ordering::Relaxed)),
                        f32::from_bits(FOG_A.load(Ordering::Relaxed)));
    for base in [0x08usize, 0x18] {
        *(p.add(base) as *mut f32) = r; *(p.add(base + 4) as *mut f32) = g;
        *(p.add(base + 8) as *mut f32) = b; *(p.add(base + 12) as *mut f32) = a;
    }
    // 선택: poke 로 오버라이드(고급 튜닝). 기본값 위에 덮어씀.
    let pn = (POKE_N.load(Ordering::Relaxed) as usize).min(16);
    for i in 0..pn {
        let off = POKE_OFF[i].load(Ordering::Relaxed) as usize;
        if off + 4 <= 0x120 { *(p.add(off) as *mut f32) = f32::from_bits(POKE_VAL[i].load(Ordering::Relaxed)); }
    }
    let tr = DISP_TRAMP2.load(Ordering::Relaxed);
    if tr == 0 { return; }
    let f: extern "win64" fn(usize, usize, usize, usize, f32) -> usize = core::mem::transmute(tr);
    f(ctx, st, cv, buf.as_ptr() as usize, 0.0); // 고정 템플릿이라 크래시 command 회피
}

// loop-end: 덤프 전용(그리기는 detour 의 라이브 캔버스에서 처리).
fn skia_replay() {
    if !SR_ON.load(Ordering::Relaxed) { return; }
    if SR_DRAW.load(Ordering::Relaxed) { return; } // 그리기 모드면 여기선 아무것도 안 함
    if CAP_ARMED.load(Ordering::Relaxed) != 1 { return; }
    SKIA_FRAMES.fetch_add(1, Ordering::Relaxed);
    let tm = CAP_TMPL.load(Ordering::Relaxed);
    if tm == 0 || !readable(tm, 0x120) { return; }
    let mut buf = [0u8; 0x120];
    unsafe { core::ptr::copy_nonoverlapping(tm as *const u8, buf.as_mut_ptr(), 0x120); }
    // ~120 프레임마다 1회 덤프(cfg skia_tag 바꾸면 새 tag command 가 찍힘).
    let fno = SKIA_FRAMES.load(Ordering::Relaxed);
    if fno % 120 == 1 {
        let tag_u64 = u64::from_le_bytes([buf[0],buf[1],buf[2],buf[3],buf[4],buf[5],buf[6],buf[7]]);
        let tag = if tag_u64 & 0x8000_0000_0000_0000 != 0 { (tag_u64 & 0xff) as u32 } else { 7 };
        let mut s = format!("[skia dump want_tag={} total={} got_tag={}] tmpl={:#x}\n  hdr[0..0x28]=",
            CAP_TAG.load(Ordering::Relaxed), DISP_TOTAL.load(Ordering::Relaxed), tag, tm);
        for i in 0..0x28 { s.push_str(&format!("{:02x}", buf[i])); }
        s.push_str("\n  f32 +0x40..0xa0:");
        let mut off = 0x40;
        while off < 0xa0 {
            let v = f32::from_le_bytes([buf[off], buf[off+1], buf[off+2], buf[off+3]]);
            s.push_str(&format!(" +{:#x}={:.2}", off, v));
            off += 4;
        }
        s.push_str("\n  bytes +0x78..0x94=");
        for i in 0x78..0x94 { s.push_str(&format!("{:02x}", buf[i])); }
        dbg_log(&s);
    }
}

unsafe fn patch_jmp(target: usize, dest: usize) {
    let mut patch = [0u8; 12];
    patch[0] = 0x48; patch[1] = 0xB8;
    patch[2..10].copy_from_slice(&dest.to_le_bytes());
    patch[10] = 0xFF; patch[11] = 0xE0;
    let mut old: u32 = 0;
    if VirtualProtect(target, 12, PAGE_EXECUTE_READWRITE, &mut old) == 0 { return; }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), target as *mut u8, 12);
    VirtualProtect(target, 12, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), target, 12);
}

unsafe fn install_skia() {
    if SKIA_INSTALLED.swap(true, Ordering::Relaxed) { return; }
    let base = GetModuleHandleW(core::ptr::null());
    if base == 0 { SKIA_STATE.store(3, Ordering::Relaxed); return; }
    let expect: [u8; 12] = [0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x53];
    let disp = base + DISP_RVA;
    let lp = base + LOOP_RVA;
    for i in 0..12 {
        if *((disp + i) as *const u8) != expect[i] || *((lp + i) as *const u8) != expect[i] {
            SKIA_STATE.store(2, Ordering::Relaxed);
            dbg_log("[skia] prologue mismatch (disp/loop) — 미설치(패치로 RVA 이동?)");
            return;
        }
    }
    DISP_ENTRY.store(disp, Ordering::Relaxed);
    LOOKUP_FN.store(base + 0xa052b0, Ordering::Relaxed); // 텍스처 조회 코어(container,key_ptr,key_len)->핸들|NULL
    // ---- dispatcher: 트램폴린 + Rust detour (라이브 캔버스 인젝트) ----
    let dtramp = VirtualAlloc(0, 64, MEM_COMMIT_RESERVE, PAGE_EXECUTE_READWRITE);
    if dtramp == 0 { SKIA_STATE.store(3, Ordering::Relaxed); return; }
    let mut dt: Vec<u8> = Vec::new();
    dt.extend_from_slice(&expect); // 12 stolen (8 push)
    dt.extend_from_slice(&[0x48, 0xB8]); dt.extend_from_slice(&(disp + 0xc).to_le_bytes()); dt.extend_from_slice(&[0xFF, 0xE0]);
    core::ptr::copy_nonoverlapping(dt.as_ptr(), dtramp as *mut u8, dt.len());
    DISP_TRAMP2.store(dtramp, Ordering::Relaxed);
    patch_jmp(disp, disp_detour as usize);
    // ---- draw loop 트램폴린 ----
    let tramp = VirtualAlloc(0, 64, MEM_COMMIT_RESERVE, PAGE_EXECUTE_READWRITE);
    if tramp == 0 { SKIA_STATE.store(3, Ordering::Relaxed); return; }
    let mut t: Vec<u8> = Vec::new();
    t.extend_from_slice(&expect);
    t.extend_from_slice(&[0x48, 0xB8]); t.extend_from_slice(&(lp + 0xc).to_le_bytes()); t.extend_from_slice(&[0xFF, 0xE0]);
    core::ptr::copy_nonoverlapping(t.as_ptr(), tramp as *mut u8, t.len());
    LOOP_TRAMP2.store(tramp, Ordering::Relaxed);
    patch_jmp(lp, detour_loop as usize);
    SKIA_STATE.store(1, Ordering::Relaxed);
    dbg_log(&format!("[skia] installed disp={:#x} loop={:#x}", disp, lp));
}

// ───────────────────────── 모드 등록 ─────────────────────────
struct FogExt;
impl ModExtension for FogExt {
    fn post_update(&self, _scene: &mut Scene, ui: &mut GameUI, _assets: &mut Assets, _dt: f32) {
        unsafe { install_cam_hook(); } // init 서 설치 실패했을 때 보험(1회 가드)
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| fog_tick(ui)));
    }
}

fn init(_ctx: &GameCtx) -> ModRegistration {
    seh_install(); // shadow-call AV 가드 (안개 재-dispatch 보호)
    unsafe { install_cam_hook(); }
    let mut reg = ModRegistration::new(MOD_ID);
    reg.set_extension(FogExt);
    reg
}
declare_mod!(init);
