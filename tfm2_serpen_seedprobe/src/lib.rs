//! tfm2_serpen_seedprobe — 바닐라 발산 체커 (관측 전용, 게임 로직 무변경)
//! ===========================================================================
//! 목적: 다른 모드 전부 끈 순수 게임에서 "같은 경기가 배경 sim vs 관전 sim에서 세르펜 킬
//!   시퀀스가 갈리나(관전≠확정)"를 측정. MobaMode::tick(0x230c290) **하나만** 후킹해
//!   게임의 네이티브 세르펜 킬로그(provider+0xed*)를 읽고, (seed,fp) 파티션별로 킬 시퀀스가
//!   여러 sim에서 달라지는 빈도를 집계. **read-only — 게임 rng·전투·엔티티 일절 미변경.**
//!   (serpen의 검증된 0.5.2 오프셋·fp 로직 이식. 단 serpen의 개입은 없음 = 순수 바닐라 측정.)
//!
//! 안전: entry-observe 훅(원본 무변경 실행). 전 read VEH(SEH) 보호, 본문 catch_unwind.
//!   MobaMode::tick은 배경 워커 다중스레드서 발화 → 공유맵 poison-safe Mutex + KC 캐시로 부하 억제.
//! ===========================================================================
#![allow(unused_imports, unused_variables)]
use mod_api::*;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, AtomicUsize, Ordering};
use std::sync::Mutex;
use std::collections::HashMap;

const MOD_ID: &str = "tfm2_serpen_seedprobe";

// ── MobaMode::tick (매 틱, rcx=World/provider) 0.5.2 (serpen 검증 상수) ──
const MOBATICK_RVA: usize = 0x230c290;
const MOBATICK_PROLOGUE: [u8; 12] = [0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x53];
// provider(=World) 오프셋 (serpen 0.5.2 프로덕션 상수)
const SEED_OFF: usize = 0xeab8;        // 경기 seed(불변)
const SIM_TICK_OFF: usize = 0xeac0;
const KILLS_PTR_OFF: usize = 0xed20;   // Vec<{team:u64,tick:u64}> ptr
const KILLS_LEN_OFF: usize = 0xed28;
const KILLS_BLUE_OFF: usize = 0xed50;
const KILLS_RED_OFF: usize = 0xed58;
// 챔피언 지문(fp)용 슬롯맵 오프셋 (serpen 0.5.2)
const ENTITY_KIND_OFF: usize = 0x68;
const CHAMP_KIND: u64 = 0xd;
const W_CHAMP_DENSE: usize = 0x720;    // ptr / +8 len (stride 0x6a8)
const W_CHAMP_SLOTS: usize = 0x738;    // ptr / +8 len (stride 0x10)
const W_PLAYER_DENSE: usize = 0x840;   // ptr / +8 len (stride 0x8d0)
const P_TEAM: usize = 0x820;
const P_CHAMP_TAG: usize = 0x8b8;
const P_CHAMP_KEY: usize = 0x8c0;
const CHAMP_STRIDE: usize = 0x6a8;
const PLAYER_STRIDE: usize = 0x8d0;
const KC_SLOTS: usize = 64;

static MOBATICK_INSTALLED: AtomicBool = AtomicBool::new(false);
static MAIN_TID: AtomicU32 = AtomicU32::new(0);
static TICK_N: AtomicU64 = AtomicU64::new(0);
static LAST_DUMP_MS: AtomicU64 = AtomicU64::new(0);

// (seed,fp) 파티션 → 킬 벡터 (team,tick,idx). serpen track_kills와 동형.
static WORLDS: Mutex<Option<HashMap<(u64, u64), Vec<(u64, u64, u64)>>>> = Mutex::new(None);
// seed → (first_tid, multi-sim(2+스레드), diverged, 관전(메인tid)포함) — 발산율 집계용 seed 단위 dedup.
static DIV_TRACK: Mutex<Option<HashMap<u64, (u32, bool, bool, bool)>>> = Mutex::new(None);
// fp 캐시 (World ptr direct-map, champ dense ptr로 검증)
#[allow(clippy::declare_interior_mutable_const)]
const KC_ZERO: AtomicU64 = AtomicU64::new(0);
static FP_ADDR: [AtomicU64; KC_SLOTS] = [KC_ZERO; KC_SLOTS];
static FP_CHK: [AtomicU64; KC_SLOTS] = [KC_ZERO; KC_SLOTS];
static FP_VAL: [AtomicU64; KC_SLOTS] = [KC_ZERO; KC_SLOTS];
// 킬로그 변화감지 캐시 (안 바뀌면 스킵 = 부하 억제)
static KC_ADDR: [AtomicU64; KC_SLOTS] = [KC_ZERO; KC_SLOTS];
static KC_SIG: [AtomicU64; KC_SLOTS] = [KC_ZERO; KC_SLOTS];

#[inline] fn splitmix64(mut z: u64) -> u64 {
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
    z ^ (z >> 31)
}
fn now_ms() -> u64 {
    unsafe {
        let mut c: i64 = 0; let mut f: i64 = 0;
        QueryPerformanceCounter(&mut c); QueryPerformanceFrequency(&mut f);
        if f == 0 { 0 } else { (c as i128 * 1000 / f as i128) as u64 }
    }
}

// ── 챔피언 구성 지문 (serpen world_fingerprint 이식) — 같은 seed의 다른 경기(Bo시리즈) 분리용 ──
unsafe fn world_fingerprint(w: usize) -> Option<u64> {
    let pp = safe_read_u64(w + W_PLAYER_DENSE)? as usize;
    let pn = safe_read_u64(w + W_PLAYER_DENSE + 8)? as usize;
    if pp < 0x10000 || pn == 0 || pn > 16 { return None; }
    let sp = safe_read_u64(w + W_CHAMP_SLOTS)? as usize;
    let sn = safe_read_u64(w + W_CHAMP_SLOTS + 8)? as usize;
    let cp = safe_read_u64(w + W_CHAMP_DENSE)? as usize;
    let cn = safe_read_u64(w + W_CHAMP_DENSE + 8)? as usize;
    if sp < 0x10000 || cp < 0x10000 || cn == 0 || cn > 4096 { return None; }
    let mut pairs: [u64; 16] = [0; 16];
    let mut np = 0usize;
    for i in 0..pn {
        let p = pp + i * PLAYER_STRIDE;
        let Some(team) = safe_read_u64(p + P_TEAM) else { continue };
        if team > 1 { continue; }
        if safe_read_u64(p + P_CHAMP_TAG).unwrap_or(0) == 0 { continue; }
        let Some(key) = safe_read_u64(p + P_CHAMP_KEY) else { continue };
        let idx = (key & 0xffff_ffff) as usize;
        if idx >= sn { continue; }
        let Some(dense) = safe_read_u64(sp + idx * 0x10 + 8) else { continue };
        if dense as usize >= cn { continue; }
        let ent = cp + dense as usize * CHAMP_STRIDE;
        if safe_read_i32(ent + ENTITY_KIND_OFF) != Some(CHAMP_KIND as i32) { continue; }
        let Some(nl) = safe_read_u64(ent + 0x258) else { continue };
        let nl = (nl as usize).min(32);
        let Some(nptr) = safe_read_u64(ent + 0x250) else { continue };
        if (nptr as usize) < 0x10000 || nl == 0 { continue; }
        let mut nb = [0u8; 32];
        if !safe_copy(nb.as_mut_ptr(), nptr as usize as *const u8, nl) { continue; }
        let mut h = 0xcbf2_9ce4_8422_2325u64;
        for &b in &nb[..nl] { h = (h ^ b as u64).wrapping_mul(0x1000_0000_01b3); }
        if np < 16 { pairs[np] = (team << 62) | (h >> 2); np += 1; }
    }
    if np < 4 { return None; }
    pairs[..np].sort_unstable();
    let mut h = 0x9e37_79b9_7f4a_7c15u64;
    for &v in &pairs[..np] { h = splitmix64(h ^ v); }
    Some(h | 1)
}
unsafe fn fp_for_world(w: usize) -> Option<u64> {
    let chk = safe_read_u64(w + W_CHAMP_DENSE).unwrap_or(0);
    if chk < 0x10000 { return None; }
    let slot = (w >> 4) & (KC_SLOTS - 1);
    if FP_ADDR[slot].load(Ordering::Relaxed) == w as u64 && FP_CHK[slot].load(Ordering::Relaxed) == chk {
        let v = FP_VAL[slot].load(Ordering::Relaxed);
        if v != 0 { return Some(v); }
    }
    let fp = world_fingerprint(w)?;
    FP_ADDR[slot].store(w as u64, Ordering::Relaxed);
    FP_CHK[slot].store(chk, Ordering::Relaxed);
    FP_VAL[slot].store(fp, Ordering::Relaxed);
    Some(fp)
}

// ── MobaMode::tick 관측: 네이티브 킬로그 읽어 (seed,fp) 파티션에 반영 + 발산 집계 ──
unsafe fn track(w: usize) {
    let seed = safe_read_u64(w + SEED_OFF).unwrap_or(0);
    if seed == 0 { return; }
    let klen = safe_read_u64(w + KILLS_LEN_OFF).unwrap_or(0);
    if klen > 128 { return; }
    let Some(kptr) = safe_read_u64(w + KILLS_PTR_OFF) else { return };
    if klen > 0 && (kptr < 0x10000 || kptr >= (1u64 << 47)) { return; }
    // 변화감지: (len, 마지막 원소) 서명 안 바뀌면 스킵
    let sig = if klen == 0 { 1 } else {
        let e = kptr as usize + (klen as usize - 1) * 16;
        klen ^ (safe_read_u64(e).unwrap_or(u64::MAX) << 40) ^ (safe_read_u64(e + 8).unwrap_or(u64::MAX) << 8) ^ 0x5eed
    };
    let slot = (w >> 4) & (KC_SLOTS - 1);
    if KC_ADDR[slot].load(Ordering::Relaxed) == w as u64 && KC_SIG[slot].load(Ordering::Relaxed) == sig { return; }
    let mut v: Vec<(u64, u64, u64)> = Vec::with_capacity(klen as usize);
    for i in 0..klen as usize {
        let e = kptr as usize + i * 16;
        let Some(t) = safe_read_u64(e) else { return };
        let Some(kt) = safe_read_u64(e + 8) else { return };
        if t > 1 { return; }
        v.push((t, kt, i as u64));
    }
    let Some(fp) = fp_for_world(w) else { return };
    let cur_tid = GetCurrentThreadId();
    // 파티션 대조 → 발산 판정
    {
        let mut wg = WORLDS.lock().unwrap_or_else(|e| e.into_inner());
        let map = wg.get_or_insert_with(HashMap::new);
        let stored = map.entry((seed, fp)).or_default();
        let mut diff = false;
        let ov = v.len().min(stored.len());
        for i in 0..ov { if v[i] != stored[i] { diff = true; break; } }
        if v.len() >= stored.len() { *stored = v; }
        drop(wg);
        // seed 단위 발산 집계
        let mut dt = DIV_TRACK.lock().unwrap_or_else(|e| e.into_inner());
        let e = dt.get_or_insert_with(HashMap::new).entry(seed).or_insert((cur_tid, false, false, false));
        if e.0 != cur_tid { e.1 = true; } // 다른 스레드가 같은 경기 sim = 비교가능(배경+관전 등)
        if diff { e.2 = true; }           // 킬 시퀀스 갈림 = 발산
        if cur_tid == MAIN_TID.load(Ordering::Relaxed) { e.3 = true; } // 관전(메인 스레드) 포함
    }
    KC_ADDR[slot].store(w as u64, Ordering::Relaxed);
    KC_SIG[slot].store(sig, Ordering::Relaxed);
}

unsafe extern "C" fn cap_mobatick(saved: *mut u64, _rsp: usize) -> u64 {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        if saved.is_null() { return; }
        TICK_N.fetch_add(1, Ordering::Relaxed);
        let w = unsafe { *saved.add(0) } as usize; // rcx = World/provider
        if w > 0x10000 && w < (1usize << 47) { unsafe { track(w); } }
    }));
    0
}

fn install_hook() {
    if MOBATICK_INSTALLED.swap(true, Ordering::Relaxed) { return; }
    let ok = unsafe { install_stub_generic(MOBATICK_RVA, 12, cap_mobatick as usize, &MOBATICK_PROLOGUE) };
    INSTALL_STATE.store(if ok { 1 } else { 2 }, Ordering::Relaxed);
}
static INSTALL_STATE: AtomicU32 = AtomicU32::new(0);

fn dump() {
    let now = now_ms();
    if now.saturating_sub(LAST_DUMP_MS.load(Ordering::Relaxed)) < 1000 { return; }
    LAST_DUMP_MS.store(now, Ordering::Relaxed);
    let dt = DIV_TRACK.lock().unwrap_or_else(|e| e.into_inner());
    let install = match INSTALL_STATE.load(Ordering::Relaxed) { 1 => "OK", 2 => "프롤로그 mismatch(미설치)", _ => "미시도" };
    let (total, multi, spec, hit, samples) = if let Some(m) = dt.as_ref() {
        let total = m.len();
        let multi = m.values().filter(|(_, mu, _, _)| *mu).count();
        let spec = m.values().filter(|(_, mu, _, sm)| *mu && *sm).count(); // 비교가능 & 관전 포함
        let hit = m.values().filter(|(_, mu, d, _)| *mu && *d).count();
        let mut s: Vec<u64> = m.iter().filter(|(_, (_, mu, d, _))| *mu && *d).map(|(k, _)| *k).take(50).collect();
        s.sort_unstable();
        (total, multi, spec, hit, s)
    } else { (0, 0, 0, 0, Vec::new()) };
    let rate = if multi > 0 { hit as f64 * 100.0 / multi as f64 } else { 0.0 };
    let mut out = format!(
        "# tfm2 발산 체커 — MobaMode::tick {:#x} 후킹 = {}  (read-only, 게임 무변경)\n\
         # ★어떤 모드 조합으로 측정했는지 확인하세요(serpen은 이 함수 충돌 → 반드시 off).\n\
         # MAIN_TID = {}   틱 관측 = {}\n\n\
         # 관측 경기(distinct seed) = {}\n\
         # 그중 2+스레드가 sim(=비교가능) = {}   (그중 관전[메인tid] 포함 = {})\n\
         # 그중 세르펜 킬 시퀀스 발산 = {}   →   ★발산율 = {:.1}%\n\n[발산 경기 seed(최대 50)]\n",
        MOBATICK_RVA, install, MAIN_TID.load(Ordering::Relaxed), TICK_N.load(Ordering::Relaxed),
        total, multi, spec, hit, rate);
    for s in &samples { out.push_str(&format!("  {:#018x}\n", s)); }
    drop(dt);
    write_log("desync_check.txt", &out);
}

// ───────────────────────── asm 스텁 트램폴린 (entry-observe, serpen 검증본) ─────────────────────────
unsafe fn install_stub_generic(rva: usize, orig_len: usize, cap_fn: usize, prologue: &[u8]) -> bool {
    let base = GetModuleHandleW(core::ptr::null());
    if base == 0 { return false; }
    let fn_addr = base + rva;
    for i in 0..prologue.len() { if *((fn_addr + i) as *const u8) != prologue[i] { return false; } }
    let stub = VirtualAlloc(0, 256, 0x3000, 0x40);
    if stub == 0 { return false; }
    let ret_addr = fn_addr + orig_len;
    let mut s: Vec<u8> = Vec::new();
    s.extend_from_slice(&[0x41, 0x54, 0x56, 0x57, 0x53, 0x41, 0x53, 0x41, 0x52, 0x41, 0x51, 0x41, 0x50, 0x52, 0x51]);
    s.extend_from_slice(&[0x48, 0x89, 0xe1]);       // mov rcx, rsp
    s.extend_from_slice(&[0x48, 0x89, 0xe3]);       // mov rbx, rsp
    s.extend_from_slice(&[0x48, 0x83, 0xe4, 0xf0]); // and rsp, -16
    s.extend_from_slice(&[0x48, 0x83, 0xec, 0x20]); // sub rsp, 0x20
    s.extend_from_slice(&[0x48, 0xb8]); s.extend_from_slice(&cap_fn.to_le_bytes());
    s.extend_from_slice(&[0xff, 0xd0]);             // call rax
    s.extend_from_slice(&[0x48, 0x89, 0xdc]);       // mov rsp, rbx
    s.extend_from_slice(&[0x59, 0x5a, 0x41, 0x58, 0x41, 0x59, 0x41, 0x5a, 0x41, 0x5b, 0x5b, 0x5f, 0x5e, 0x41, 0x5c]);
    let mut orig = vec![0u8; orig_len];
    core::ptr::copy_nonoverlapping(fn_addr as *const u8, orig.as_mut_ptr(), orig_len);
    s.extend_from_slice(&orig);
    s.extend_from_slice(&[0x48, 0xb8]); s.extend_from_slice(&ret_addr.to_le_bytes());
    s.extend_from_slice(&[0xff, 0xe0]);
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    let mut patch = vec![0x90u8; orig_len];
    patch[0] = 0x48; patch[1] = 0xb8; patch[2..10].copy_from_slice(&stub.to_le_bytes());
    patch[10] = 0xff; patch[11] = 0xe0;
    let mut old = 0u32;
    if VirtualProtect(fn_addr, orig_len, 0x40, &mut old) == 0 { return false; }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), fn_addr as *mut u8, orig_len);
    VirtualProtect(fn_addr, orig_len, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), fn_addr, orig_len);
    true
}

// ───────────────────────── SEH 안전 r/w (serpen 검증본) ─────────────────────────
static mut SEH: [u64; 8] = [0u64; 8];
static SEH_INSTALLED: AtomicBool = AtomicBool::new(false);
static SEH_BUSY: AtomicBool = AtomicBool::new(false);
#[repr(C)]
struct ExceptionRecord { code: u32, flags: u32, rec: usize, addr: usize, nparams: u32, _p: u32, params: [usize; 15] }
#[repr(C)]
struct ExceptionPointers { rec: *mut ExceptionRecord, ctx: *mut core::ffi::c_void }
type VehHandler = extern "system" fn(*mut ExceptionPointers) -> i32;
extern "system" fn seh_veh(p: *mut ExceptionPointers) -> i32 {
    unsafe {
        if p.is_null() { return 0; }
        let rec = (*p).rec;
        if rec.is_null() || (*rec).code != 0xC0000005 { return 0; }
        let g = core::ptr::addr_of!(SEH) as *const u64;
        if *g.add(0) == 0 { return 0; }
        if *g.add(1) != GetCurrentThreadId() as u64 { return 0; }
        let ctx = (*p).ctx as usize;
        if ctx == 0 { return 0; }
        let rip = *((ctx + 0xF8) as *const u64);
        if rip < *g.add(5) || rip >= *g.add(6) { return 0; }
        *((ctx + 0xF8) as *mut u64) = *g.add(2);
        *((ctx + 0x98) as *mut u64) = *g.add(3);
        *((ctx + 0xA0) as *mut u64) = *g.add(4);
        let gm = core::ptr::addr_of_mut!(SEH) as *mut u64;
        *gm.add(7) += 1;
        -1
    }
}
fn seh_install() { if SEH_INSTALLED.swap(true, Ordering::Relaxed) { return; } unsafe { AddVectoredExceptionHandler(1, seh_veh); } }
#[inline(never)]
unsafe fn safe_copy(dst: *mut u8, src: *const u8, len: usize) -> bool {
    if !SEH_INSTALLED.load(Ordering::Relaxed) { return false; }
    while SEH_BUSY.swap(true, Ordering::Acquire) { core::hint::spin_loop(); }
    let g = core::ptr::addr_of_mut!(SEH) as *mut u64;
    *g.add(1) = GetCurrentThreadId() as u64;
    let ok: u64;
    core::arch::asm!(
        "lea rax, [rip + 200f]", "mov [{g} + 40], rax",
        "lea rax, [rip + 201f]", "mov [{g} + 48], rax",
        "lea rax, [rip + 202f]", "mov [{g} + 16], rax",
        "mov [{g} + 24], rsp", "mov [{g} + 32], rbp",
        "mov qword ptr [{g} + 0], 1",
        "cld",
        "200:", "rep movsb",
        "201:", "mov {ok}, 1", "jmp 203f",
        "202:", "mov {ok}, 0",
        "203:", "mov qword ptr [{g} + 0], 0",
        g = in(reg) g, ok = out(reg) ok,
        inout("rcx") len => _, inout("rdi") dst => _, inout("rsi") src => _, out("rax") _,
    );
    SEH_BUSY.store(false, Ordering::Release);
    ok != 0
}
unsafe fn safe_read_u64(addr: usize) -> Option<u64> {
    let mut b = [0u8; 8];
    if safe_copy(b.as_mut_ptr(), addr as *const u8, 8) { Some(u64::from_le_bytes(b)) } else { None }
}
unsafe fn safe_read_i32(addr: usize) -> Option<i32> {
    let mut b = [0u8; 4];
    if safe_copy(b.as_mut_ptr(), addr as *const u8, 4) { Some(i32::from_le_bytes(b)) } else { None }
}

// ───────────────────────── 파일 ─────────────────────────
fn dll_path() -> Option<PathBuf> {
    unsafe {
        let addr = dll_path as *const () as usize;
        let mut h: isize = 0;
        if GetModuleHandleExW(0x4 | 0x2, addr as *const u16, &mut h) == 0 || h == 0 { return None; }
        let mut buf = [0u16; 4096];
        let n = GetModuleFileNameW(h, buf.as_mut_ptr(), buf.len() as u32);
        if n == 0 { return None; }
        Some(PathBuf::from(String::from_utf16_lossy(&buf[..n as usize])))
    }
}
fn mod_dir() -> Option<PathBuf> { dll_path()?.parent().map(|p| p.to_path_buf()) }
fn write_log(name: &str, content: &str) {
    if let Some(p) = mod_dir().map(|d| d.join(name)) { let _ = fs::write(p, content); }
}

// ───────────────────────── WinAPI ─────────────────────────
#[link(name = "kernel32")]
extern "system" {
    fn GetModuleHandleW(name: *const u16) -> usize;
    fn GetModuleHandleExW(f: u32, name: *const u16, h: *mut isize) -> i32;
    fn GetModuleFileNameW(h: isize, buf: *mut u16, n: u32) -> u32;
    fn VirtualProtect(addr: usize, size: usize, new_protect: u32, old_protect: *mut u32) -> i32;
    fn VirtualAlloc(addr: usize, size: usize, alloc_type: u32, protect: u32) -> usize;
    fn FlushInstructionCache(proc: usize, addr: usize, size: usize) -> i32;
    fn GetCurrentProcess() -> usize;
    fn GetCurrentThreadId() -> u32;
    fn AddVectoredExceptionHandler(first: u32, handler: VehHandler) -> usize;
    fn QueryPerformanceCounter(c: *mut i64) -> i32;
    fn QueryPerformanceFrequency(f: *mut i64) -> i32;
}

// ───────────────────────── 로더 ABI ─────────────────────────
static SETUP_DONE: AtomicBool = AtomicBool::new(false);
fn setup() {
    if SETUP_DONE.swap(true, Ordering::Relaxed) { return; }
    MAIN_TID.store(unsafe { GetCurrentThreadId() }, Ordering::Relaxed);
    seh_install();
    install_hook();
}
struct Ext;
impl ModExtension for Ext {
    fn on_init(&self, _s: &mut Scene, _u: &mut GameUI, _a: &mut Assets) { setup(); }
    fn post_update(&self, _s: &mut Scene, _u: &mut GameUI, _a: &mut Assets, _dt: f32) {
        setup();
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(dump));
    }
}
struct SExt;
impl ModServerExtension for SExt {
    fn on_server_start(&self, _c: &mut ServerModContext) { setup(); }
}
fn init(_ctx: &GameCtx) -> ModRegistration {
    setup();
    let mut reg = ModRegistration::new(MOD_ID);
    reg.set_extension(Ext);
    reg.set_server_extension(SExt);
    reg
}
declare_mod!(init);
