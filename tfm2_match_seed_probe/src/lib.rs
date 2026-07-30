//! tfm2_match_seed_probe v3 — "우리 경기 sim 인스턴스"를 launch 시점에 식별 가능한가 (read-only)
//! ===========================================================================
//! 설계 의도(유저): buy를 "생산하는 애(sim 인스턴스=provider)" 자체를 잡는다.
//!   → 태어나는 순간(런처 launch) 1회 판정 → 그 provider의 buy는 전부 우리 것.
//!   → 관전(LIVE_SEED)을 기다릴 필요 없음 = 배경 pre-sim도 커버, 타이밍 문제 소멸.
//!
//! v2 실측 확정:
//!   내 경기 = 배경 pre-sim(idx103 ret=0xd64af1) + 관전(idx104 ret=0x75e5cf), **동일 seed**.
//!   중복 seed = 정확히 1개(=내 경기뿐, 충돌 0) ⇒ seed는 provider identity로 완벽.
//!   ⛔mgmt+0x1cd88 0x268 레코드 경로 = 런타임 부재(v1/v2 반증, 재시도 금지).
//!
//! v3 검증 목표(단 하나):
//!   **런처 진입 시점의 인자(arg5 로스터 blob 0x1548)만으로 "이 sim이 우리 경기냐"를 판정 가능한가.**
//!   방법: 스택인자 슬롯들을 훑어 각 blob에서 내 선발 athlete_id(db.last_starting)를 스캔.
//!   기대: RENDER_SEED와 같은 seed의 배경 launch에서도 my_hits>0, 나머지 배경경기는 0.
//!   ⇒ 성립하면 경기스코프 주입이 프리리드·관전대기 없이 구현 가능.
//!
//! 안전: entry-observe(원본 무변경), SEH r/w, catch_unwind, poison-safe 락. read-only.
//!   ★단독 실행 권장(런처 후킹 모드 OFF).
//! ===========================================================================
#![allow(unused_imports, unused_variables)]
use mod_api::*;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicPtr, AtomicU32, AtomicU64, AtomicUsize, Ordering};
use std::sync::Mutex;
use std::collections::{HashMap, HashSet};

const MOD_ID: &str = "tfm2_match_seed_probe";

const LAUNCHER_RVA: usize = 0x1d96870;
const PROLOGUE12: [u8; 12] = [0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x53];
// 관전(화면 경기) 런처 리턴주소 (serpen v0.4.1 0.5.2 검증본, v2서 0x75e5cf 실발화 확인)
const RENDER_RETS: [usize; 3] = [0x759c36, 0x75e5cf, 0x1555215];
// 스캔할 blob 크기(로스터 blob = 0x1548) + 여유
const BLOB_SCAN: usize = 0x1600;
const STACK_SLOTS: usize = 8; // saved[11..19] = [rsp+8 .. rsp+0x40]

static LAUNCH_INSTALLED: AtomicU32 = AtomicU32::new(0);
static SETUP_DONE: AtomicBool = AtomicBool::new(false);
static MAIN_TID: AtomicU32 = AtomicU32::new(0);
static LAUNCH_N: AtomicU64 = AtomicU64::new(0);
static LAST_DUMP_MS: AtomicU64 = AtomicU64::new(0);
static RENDER_SEED: AtomicU64 = AtomicU64::new(0);

// 내 선발 로스터 (item_tactics v15 패턴 이식)
static MY_ATHLETES: AtomicPtr<HashSet<u64>> = AtomicPtr::new(core::ptr::null_mut());
static MY_ATH_PREV: AtomicPtr<HashSet<u64>> = AtomicPtr::new(core::ptr::null_mut());
static MY_ATH_N: AtomicU64 = AtomicU64::new(0);
static PLAYER_TEAM_ID: AtomicU64 = AtomicU64::new(u64::MAX);
static ROSTER_TICK: AtomicU64 = AtomicU64::new(0);

#[derive(Clone, Copy)]
struct LaunchRec {
    idx: u64,
    seed: u64,
    ret_rva: usize,
    stack: [u64; STACK_SLOTS],
    hit_slot: i32,      // 내 선수가 발견된 스택 슬롯 인덱스(-1=없음)
    hits: u32,          // 발견된 내 선수 id 개수(중복 포함)
    distinct: u32,      // 서로 다른 id 개수
    offs: [u32; 6],     // 최초 6개 히트 오프셋
    roster_known: bool, // 스캔 시점에 내 로스터 확보돼 있었나
}

static RECS: Mutex<Option<Vec<LaunchRec>>> = Mutex::new(None);

fn now_ms() -> u64 {
    unsafe {
        let mut c: i64 = 0; let mut f: i64 = 0;
        QueryPerformanceCounter(&mut c); QueryPerformanceFrequency(&mut f);
        if f == 0 { 0 } else { (c as i128 * 1000 / f as i128) as u64 }
    }
}

fn publish_my_athletes(set: HashSet<u64>) {
    MY_ATH_N.store(set.len() as u64, Ordering::Relaxed);
    let boxed = Box::into_raw(Box::new(set));
    let old = MY_ATHLETES.swap(boxed, Ordering::AcqRel);
    let stale = MY_ATH_PREV.swap(old, Ordering::AcqRel);
    if !stale.is_null() { unsafe { drop(Box::from_raw(stale)); } }
}

// blob에서 내 선발 athlete_id를 스캔(u32 정렬 스텝). 한 번의 safe_copy로 벌크 복사 후 메모리 스캔.
unsafe fn scan_blob(ptr: usize, ids: &HashSet<u64>, offs: &mut [u32; 6]) -> (u32, u32) {
    if ptr < 0x10000 || ptr >= (1usize << 47) { return (0, 0); }
    let mut buf = vec![0u8; BLOB_SCAN];
    // 큰 덩어리부터 시도(부분 언매핑 대비 축소 폴백)
    let mut n = BLOB_SCAN;
    let mut ok = false;
    while n >= 0x200 {
        if safe_copy(buf.as_mut_ptr(), ptr as *const u8, n) { ok = true; break; }
        n /= 2;
    }
    if !ok { return (0, 0); }
    let mut hits = 0u32;
    let mut seen: HashSet<u64> = HashSet::new();
    let mut k = 0usize;
    let mut off = 0usize;
    while off + 4 <= n {
        let v = u32::from_le_bytes([buf[off], buf[off + 1], buf[off + 2], buf[off + 3]]) as u64;
        if v != 0 && ids.contains(&v) {
            hits += 1;
            seen.insert(v);
            if k < 6 { offs[k] = off as u32; k += 1; }
        }
        off += 4;
    }
    (hits, seen.len() as u32)
}

unsafe extern "C" fn cap_launcher(saved: *mut u64, _rsp: usize) -> u64 {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        if saved.is_null() { return; }
        let seed = unsafe { *saved.add(2) };          // r8
        let ret = unsafe { *saved.add(10) } as usize; // 원래 [rsp] = retaddr
        let idx = LAUNCH_N.fetch_add(1, Ordering::Relaxed);
        if seed == 0 { return; }
        let base = unsafe { GetModuleHandleW(core::ptr::null()) };
        let ret_rva = if base != 0 && ret > base { ret - base } else { 0 };
        if RENDER_RETS.contains(&ret_rva) { RENDER_SEED.store(seed, Ordering::Relaxed); }

        // 스택 인자 슬롯 수집: saved[11..11+STACK_SLOTS] = [rsp+8], [rsp+0x10], ...
        let mut stack = [0u64; STACK_SLOTS];
        for i in 0..STACK_SLOTS { stack[i] = unsafe { *saved.add(11 + i) }; }

        // 내 로스터 확보 상태면 각 스택 슬롯의 blob을 스캔
        let mut hit_slot: i32 = -1;
        let mut hits = 0u32;
        let mut distinct = 0u32;
        let mut offs = [0u32; 6];
        let idsp = MY_ATHLETES.load(Ordering::Acquire);
        let roster_known = !idsp.is_null() && unsafe { !(*idsp).is_empty() };
        if roster_known {
            let ids = unsafe { &*idsp };
            for i in 0..STACK_SLOTS {
                let mut o = [0u32; 6];
                let (h, d) = unsafe { scan_blob(stack[i] as usize, ids, &mut o) };
                if h > hits { hits = h; distinct = d; hit_slot = i as i32; offs = o; }
            }
        }

        let rec = LaunchRec { idx, seed, ret_rva, stack, hit_slot, hits, distinct, offs, roster_known };
        let mut g = RECS.lock().unwrap_or_else(|e| e.into_inner());
        let v = g.get_or_insert_with(Vec::new);
        if v.len() < 400 { v.push(rec); }
    }));
    0
}

fn install_hooks() {
    if LAUNCH_INSTALLED.load(Ordering::Relaxed) == 0 {
        let ok = unsafe { install_stub_generic(LAUNCHER_RVA, 12, cap_launcher as usize, &PROLOGUE12) };
        LAUNCH_INSTALLED.store(if ok { 1 } else { 2 }, Ordering::Relaxed);
    }
}

fn istate(v: u32) -> &'static str {
    match v { 1 => "OK", 2 => "프롤로그 mismatch(미설치)", _ => "미시도" }
}

fn dump() {
    let now = now_ms();
    if now.saturating_sub(LAST_DUMP_MS.load(Ordering::Relaxed)) < 1000 { return; }
    LAST_DUMP_MS.store(now, Ordering::Relaxed);

    let rs = RENDER_SEED.load(Ordering::Relaxed);
    let my_ids: Vec<u64> = {
        let p = MY_ATHLETES.load(Ordering::Acquire);
        if p.is_null() { Vec::new() } else { let mut v: Vec<u64> = unsafe { (*p).iter().copied().collect() }; v.sort_unstable(); v }
    };

    let mut out = String::new();
    out.push_str("# tfm2_match_seed_probe v3 — launch 시점에 '우리 경기 sim'을 식별 가능한가 (read-only)\n");
    out.push_str(&format!("# 런처 {:#x} 설치={}   MAIN_TID={}   launch총={}\n",
        LAUNCHER_RVA, istate(LAUNCH_INSTALLED.load(Ordering::Relaxed)),
        MAIN_TID.load(Ordering::Relaxed), LAUNCH_N.load(Ordering::Relaxed)));
    out.push_str(&format!("# 내 팀 team_id={}  선발 로스터={}명  ids={:x?}\n",
        { let p = PLAYER_TEAM_ID.load(Ordering::Relaxed); if p == u64::MAX { "미캡처".to_string() } else { p.to_string() } },
        MY_ATH_N.load(Ordering::Relaxed), my_ids));
    out.push_str(&format!("# RENDER_SEED(관전) = {:#018x}{}\n", rs, if rs == 0 { "  (아직 관전 미포착)" } else { "" }));
    out.push_str("#\n# ★판정 기준: RENDER_SEED와 같은 seed의 launch들(관전 + 그 경기 배경 pre-sim)에서만\n");
    out.push_str("#   my_hits>0 이고, 다른 배경 경기들은 my_hits=0 이면 → launch 시점 식별 성립.\n");

    let recs: Vec<LaunchRec> = {
        let g = RECS.lock().unwrap_or_else(|e| e.into_inner());
        g.as_ref().map(|v| v.clone()).unwrap_or_default()
    };

    // 요약 집계
    let with_hits: Vec<&LaunchRec> = recs.iter().filter(|r| r.hits > 0).collect();
    let same_seed: Vec<&LaunchRec> = recs.iter().filter(|r| rs != 0 && r.seed == rs).collect();
    let scanned = recs.iter().filter(|r| r.roster_known).count();
    out.push_str(&format!("#\n# ── 요약: 기록 launch={}  (로스터 확보상태로 스캔된 것={})\n", recs.len(), scanned));
    out.push_str(&format!("#   my_hits>0 인 launch = {}건\n", with_hits.len()));
    out.push_str(&format!("#   RENDER_SEED와 동일 seed launch = {}건\n", same_seed.len()));
    let ok_all = !same_seed.is_empty() && same_seed.iter().all(|r| r.hits > 0)
        && with_hits.len() == same_seed.len();
    out.push_str(&format!("#   ★★판정 = {}\n", if rs == 0 { "관전 미포착(판정 불가)" }
        else if ok_all { "✅성립 — 내 경기 launch만 정확히 my_hits>0 (배경 포함)" }
        else if with_hits.is_empty() { "✗ 로스터 blob에서 내 선수 미발견 (스택 슬롯/레이아웃 재조사 필요)" }
        else { "△부분 — 아래 상세 대조 필요(오탐 또는 누락)" }));

    // 내 경기 관련 launch 상세
    out.push_str("\n# ── ★RENDER_SEED와 같은 seed의 launch (내 경기 = 배경 pre-sim + 관전) ──\n");
    for r in &same_seed {
        out.push_str(&format!("#   idx={} ret={:#09x} hits={} distinct={} slot={} offs={:x?} roster_known={}\n",
            r.idx, r.ret_rva, r.hits, r.distinct, r.hit_slot, &r.offs[..], r.roster_known));
        out.push_str(&format!("#     stack[rsp+0x8..]: {:#x?}\n", &r.stack[..]));
    }

    // my_hits>0 인 것 전체(오탐 확인)
    out.push_str("\n# ── my_hits>0 인 launch 전체 (내 경기 외에 잡히면 오탐) ──\n");
    for r in with_hits.iter().take(30) {
        let mark = if rs != 0 && r.seed == rs { "  <== 내 경기" } else { "  ← ⚠오탐?" };
        out.push_str(&format!("#   idx={} seed={:#018x} ret={:#09x} hits={} distinct={} slot={} offs={:x?}{}\n",
            r.idx, r.seed, r.ret_rva, r.hits, r.distinct, r.hit_slot, &r.offs[..], mark));
    }

    // 최근 launch 목록(참고)
    out.push_str("\n# ── 최근 launch 40건 [idx seed ret hits] ──\n");
    for r in recs.iter().rev().take(40) {
        out.push_str(&format!("#   idx={:>4} seed={:#018x} ret={:#09x} hits={}\n", r.idx, r.seed, r.ret_rva, r.hits));
    }
    write_log("match_seed_probe.txt", &out);
}

// ───────────────────────── asm 스텁 트램폴린 (serpen 검증본) ─────────────────────────
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
    s.extend_from_slice(&[0x48, 0x89, 0xe1]);
    s.extend_from_slice(&[0x48, 0x89, 0xe3]);
    s.extend_from_slice(&[0x48, 0x83, 0xe4, 0xf0]);
    s.extend_from_slice(&[0x48, 0x83, 0xec, 0x20]);
    s.extend_from_slice(&[0x48, 0xb8]); s.extend_from_slice(&cap_fn.to_le_bytes());
    s.extend_from_slice(&[0xff, 0xd0]);
    s.extend_from_slice(&[0x48, 0x89, 0xdc]);
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
fn setup() {
    if SETUP_DONE.swap(true, Ordering::Relaxed) { return; }
    MAIN_TID.store(unsafe { GetCurrentThreadId() }, Ordering::Relaxed);
    seh_install();
    install_hooks();
}
struct Ext;
impl ModExtension for Ext {
    fn on_init(&self, _s: &mut Scene, _u: &mut GameUI, _a: &mut Assets) { setup(); }
    fn post_update(&self, scene: &mut Scene, _u: &mut GameUI, _a: &mut Assets, _dt: f32) {
        setup();
        // 내 팀 선발 로스터 게시 (item_tactics v15 패턴)
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if let Scene::InGame { data } = scene {
                let db = data.db();
                let pu = db.player_team_id() as u64;
                if pu != u64::MAX && pu < 10000 { PLAYER_TEAM_ID.store(pu, Ordering::Relaxed); }
                const ROSTER_POLL: u64 = 60;
                let n = ROSTER_TICK.fetch_add(1, Ordering::Relaxed);
                let known = PLAYER_TEAM_ID.load(Ordering::Relaxed);
                if n % ROSTER_POLL == 0 && known != u64::MAX && known < 10000 {
                    let mut my: HashSet<u64> = HashSet::new();
                    if let Some(team) = db.team(known as _) {
                        for slot in team.last_starting.iter() {
                            if let Some(aid) = slot { my.insert(*aid as u64); }
                        }
                    }
                    if !my.is_empty() { publish_my_athletes(my); }
                }
            }
        }));
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
