// tfm2_ai_adjust_2.rs — ai_adjust 린(lean) 재설계 (2026-07-19, 게임 0.5.1 buildid 24215274 기준).
//
// ★설계 원칙: "재현하지 않기". 원본(tfm2_ai_adjust)의 대체스택(condgate/dd7700/poke/disc4/engage 재현)을
//   전부 제거하고, 활성 튜닝(2026-07-19 감사 확정분)만 3계층으로 구현:
//   [A] 게임 원본 상수 → byte-patch (훅 0, 런타임 비용 0): oi_*(objective 13사이트) + d19(15사이트) + vis_window(1사이트)
//   [B] 게임에 없는 신규 판단(numbers/tower/stat 후퇴) → movepri "원본 실행 후" 출력 override (게임이 계산, 우리는 덧칠)
//   [C] recall(fc59a0)만 잔존 재현 — rc_* 튜닝이 게임 계산식 내부 상수라 불가피. 검증본(my_recall_mult) 그대로 포팅.
//   미포함(활성 cfg가 게임 원본값이라 불필요): engage(eng_role*=원본값)/disc4(d4_*=원본값)/poke(pk_/ep_=원본값)/
//   condgate/dd7700 재현. ⚠dd_*(라인전) 6개·ec_* 2개 튜닝은 이 모드가 아직 미커버(Phase2: imm 사이트 발굴 예정).
//
// ⚠ 원본 tfm2_ai_adjust와 동시 활성 금지 — 같은 함수(movepri/fc59a0)를 후킹하고 byte-patch가 겹침.
// 빌드: rustc 직접(nightly-2026-05-24, -C opt-level=1 -C overflow-checks=off), sdk_051.

use mod_api::*;
use std::path::PathBuf;
use std::fs;
use std::io::Write;
use std::sync::atomic::{AtomicBool, AtomicI64, AtomicU8, AtomicU64, AtomicUsize, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

const MOD_ID: &str = "tfm2_ai_adjust_2";

// ════════════════ RVA (0.5.1 buildid 24215274; 패치 시 이 블록만 갱신) ════════════════
const RVA_MOVEPRI: usize = 0x1cbc220;   // facet#4 movepriority 디스패처. 프롤로그 13B(6push+sub rsp,0x48), rip-rel無(원본 모드 0.5.1 검증).
const ORIG_LEN_MOVEPRI: usize = 13;
const RVA_FC59A0: usize = 0x1e2c980;    // recall_rng_score. 프롤로그 12B(8push), rip-rel無.
const ORIG_LEN_FC59A0: usize = 12;
// byte-patch 사이트 RVA는 각 apply_* 함수 내부에 (원본 모드에서 검증된 표 그대로).

// ════════════════ VEH lockless 안전 read/write (원본 mem_safety level2 경로만 채택) ════════════════
core::arch::global_asm!(
    ".globl pr_rd8", ".globl pr_rd8_f", ".globl pr_rd8_l",
    ".globl pr_rd4", ".globl pr_rd4_f", ".globl pr_rd4_l",
    ".globl pr_rd1", ".globl pr_rd1_f", ".globl pr_rd1_l",
    "pr_rd8:", "pr_rd8_f:", "mov rax, qword ptr [rcx]", "mov qword ptr [rdx], rax", "mov eax, 1", "ret",
    "pr_rd8_l:", "xor eax, eax", "ret",
    "pr_rd4:", "pr_rd4_f:", "mov eax, dword ptr [rcx]", "mov dword ptr [rdx], eax", "mov eax, 1", "ret",
    "pr_rd4_l:", "xor eax, eax", "ret",
    "pr_rd1:", "pr_rd1_f:", "movzx eax, byte ptr [rcx]", "mov byte ptr [rdx], al", "mov eax, 1", "ret",
    "pr_rd1_l:", "xor eax, eax", "ret",
);
extern "C" {
    fn pr_rd8(addr: usize, out: *mut u64) -> u32;
    fn pr_rd4(addr: usize, out: *mut u32) -> u32;
    fn pr_rd1(addr: usize, out: *mut u8) -> u32;
    static pr_rd8_f: u8; static pr_rd8_l: u8;
    static pr_rd4_f: u8; static pr_rd4_l: u8;
    static pr_rd1_f: u8; static pr_rd1_l: u8;
}
core::arch::global_asm!(
    ".globl pr_wr8", ".globl pr_wr8_f", ".globl pr_wr8_l",
    ".globl pr_wr4", ".globl pr_wr4_f", ".globl pr_wr4_l",
    "pr_wr8:", "pr_wr8_f:", "mov qword ptr [rcx], rdx", "mov eax, 1", "ret",
    "pr_wr8_l:", "xor eax, eax", "ret",
    "pr_wr4:", "pr_wr4_f:", "mov dword ptr [rcx], edx", "mov eax, 1", "ret",
    "pr_wr4_l:", "xor eax, eax", "ret",
);
extern "C" {
    fn pr_wr8(addr: usize, val: u64) -> u32;
    fn pr_wr4(addr: usize, val: u32) -> u32;
    static pr_wr8_f: u8; static pr_wr8_l: u8;
    static pr_wr4_f: u8; static pr_wr4_l: u8;
}

#[repr(C)] struct ExceptionRecord { code: u32, _flags: u32, _rec: u64, _addr: u64, _np: u32, _pad: u32, _params: [u64; 15] }
#[repr(C)] struct ExceptionPointers { rec: *mut ExceptionRecord, ctx: *mut core::ffi::c_void }
type VehHandler = extern "system" fn(*mut ExceptionPointers) -> i32;
extern "system" {
    fn AddVectoredExceptionHandler(first: u32, handler: VehHandler) -> usize;
    fn GetModuleHandleW(name: *const u16) -> usize;
    fn GetModuleHandleExW(flags: u32, addr: *const u16, out: *mut usize) -> i32;
    fn GetModuleFileNameW(h: usize, out: *mut u16, cap: u32) -> u32;
    fn VirtualAlloc(addr: usize, size: usize, ty: u32, prot: u32) -> usize;
    fn VirtualProtect(addr: usize, size: usize, prot: u32, old: *mut u32) -> i32;
    fn FlushInstructionCache(h: isize, addr: usize, size: usize) -> i32;
    fn GetCurrentProcess() -> isize;
    fn GetCurrentThreadId() -> u32;
    fn SetUnhandledExceptionFilter(f: usize) -> usize;
    fn CreateFileW(name: *const u16, access: u32, share: u32, sec: usize, disp: u32, flags: u32, tmpl: usize) -> isize;
    fn WriteFile(h: isize, buf: *const u8, len: u32, written: *mut u32, ov: usize) -> i32;
    fn SetFilePointer(h: isize, lo: i32, hi: usize, method: u32) -> u32;
    fn CloseHandle(h: isize) -> i32;
    fn GetCurrentProcessId() -> u32;
}

static SEH_INSTALLED: AtomicBool = AtomicBool::new(false);
extern "system" fn seh_veh(p: *mut ExceptionPointers) -> i32 {
    const CONTINUE_EXECUTION: i32 = -1; const CONTINUE_SEARCH: i32 = 0;
    unsafe {
        if p.is_null() { return CONTINUE_SEARCH; }
        let rec = (*p).rec;
        if rec.is_null() || (*rec).code != 0xC0000005 { return CONTINUE_SEARCH; }
        let ctx = (*p).ctx as usize;
        if ctx == 0 { return CONTINUE_SEARCH; }
        let rip = *((ctx + 0xF8) as *const u64) as usize;
        let land = if rip == core::ptr::addr_of!(pr_rd8_f) as usize { core::ptr::addr_of!(pr_rd8_l) as usize }
                   else if rip == core::ptr::addr_of!(pr_rd4_f) as usize { core::ptr::addr_of!(pr_rd4_l) as usize }
                   else if rip == core::ptr::addr_of!(pr_rd1_f) as usize { core::ptr::addr_of!(pr_rd1_l) as usize }
                   else if rip == core::ptr::addr_of!(pr_wr8_f) as usize { core::ptr::addr_of!(pr_wr8_l) as usize }
                   else if rip == core::ptr::addr_of!(pr_wr4_f) as usize { core::ptr::addr_of!(pr_wr4_l) as usize }
                   else { 0 };
        if land != 0 { *((ctx + 0xF8) as *mut u64) = land as u64; return CONTINUE_EXECUTION; }
        CONTINUE_SEARCH
    }
}
fn seh_install() { if SEH_INSTALLED.swap(true, Ordering::Relaxed) { return; } unsafe { AddVectoredExceptionHandler(1, seh_veh); } }

#[inline] fn ptr_ok(a: usize) -> bool { a >= 0x10000 && a < 0x0001_0000_0000_0000 }
#[inline] unsafe fn rd_u64(a: usize) -> Option<u64> { if a < 0x10000 { return None; } let mut o = 0u64; if pr_rd8(a, &mut o) != 0 { Some(o) } else { None } }
#[inline] unsafe fn rd_i64(a: usize) -> Option<i64> { rd_u64(a).map(|v| v as i64) }
#[inline] unsafe fn rd_i32(a: usize) -> Option<i32> { if a < 0x10000 { return None; } let mut o = 0u32; if pr_rd4(a, &mut o) != 0 { Some(o as i32) } else { None } }
#[inline] unsafe fn rd_u32(a: usize) -> u32 { if a < 0x10000 { return 0; } let mut o = 0u32; if pr_rd4(a, &mut o) != 0 { o } else { 0 } }
#[inline] unsafe fn rd_u8(a: usize) -> u8 { if a < 0x10000 { return 0; } let mut o = 0u8; if pr_rd1(a, &mut o) != 0 { o } else { 0 } }
#[inline] unsafe fn wr_u64(a: usize, v: u64) -> bool { a >= 0x10000 && pr_wr8(a, v) != 0 }
#[inline] unsafe fn wr_u32(a: usize, v: u32) -> bool { a >= 0x10000 && pr_wr4(a, v) != 0 }
// 범위 커밋 확인(원본 readable() fast_guard=1 의미론): 시작 + 걸치는 페이지 경계 프로브
#[inline] unsafe fn probe_ok(addr: usize, len: usize) -> bool {
    if addr < 0x10000 || len == 0 { return false; }
    let mut b = 0u8;
    if pr_rd1(addr, &mut b) == 0 { return false; }
    let last = addr + len - 1;
    let mut p = (addr | 0xfff) + 1;
    while p <= last { if pr_rd1(p, &mut b) == 0 { return false; } p += 0x1000; }
    true
}
// ★[07-20 최적화] "객체범위 1회 프로브 + raw 읽기" — 원본 dd7_slot_a8에서 확립된 안전 패턴.
//   probe_ok 직후 **같은 동기 스코프**에서만 사용(시간축 캐시 아님 = 06-21 힙캐시 TOCTOU 판정과 무관).
//   VEH 경유 읽기(pr_rd* CALL, read당 ~5-8ns)를 객체당 프로브 1-2회로 압축. 필드 4개 객체 기준 ~절반.
//   의미동치: 프로브 실패=그 객체 skip ≡ 구 rd_* fault→unwrap_or(0)→skip (유효 sim에서 값 동일).
#[inline] unsafe fn raw_u64(a: usize) -> u64 { std::ptr::read_unaligned(a as *const u64) }
#[inline] unsafe fn raw_i64(a: usize) -> i64 { std::ptr::read_unaligned(a as *const i64) }
#[inline] unsafe fn raw_i32(a: usize) -> i32 { std::ptr::read_unaligned(a as *const i32) }
#[inline] unsafe fn raw_u8(a: usize) -> u8 { std::ptr::read_unaligned(a as *const u8) }

// ════════════════ 크래시 로거 + panic hook (원본 축약판) ════════════════
static CRASH_MOD_BASE: AtomicU64 = AtomicU64::new(0);
static CRASH_PREV: AtomicU64 = AtomicU64::new(0);
static CRASH_INSTALLED: AtomicBool = AtomicBool::new(false);
static CRASH_LOGGED: AtomicBool = AtomicBool::new(false);
static mut CRASH_PATH: [u16; 600] = [0u16; 600];
static PANIC_N: AtomicU64 = AtomicU64::new(0);
static EXE_BASE: AtomicU64 = AtomicU64::new(0);
#[inline] unsafe fn exe_base() -> usize {
    let v = EXE_BASE.load(Ordering::Relaxed) as usize;
    if v != 0 { return v; }
    let b = GetModuleHandleW(core::ptr::null());
    EXE_BASE.store(b as u64, Ordering::Relaxed);
    b
}
#[inline] fn cb_put(buf: &mut [u8; 2048], pos: &mut usize, b: u8) { if *pos < 2048 { buf[*pos] = b; *pos += 1; } }
fn cb_str(buf: &mut [u8; 2048], pos: &mut usize, s: &[u8]) { for &c in s { cb_put(buf, pos, c); } }
fn cb_hex(buf: &mut [u8; 2048], pos: &mut usize, v: u64) {
    cb_str(buf, pos, b"0x");
    let mut started = false; let mut sh = 60i32;
    while sh >= 0 {
        let d = ((v >> sh) & 0xf) as u8;
        if d != 0 || started || sh == 0 { cb_put(buf, pos, if d < 10 { b'0' + d } else { b'a' + d - 10 }); started = true; }
        sh -= 4;
    }
}
unsafe fn crash_write(data: &[u8]) {
    let path = core::ptr::addr_of!(CRASH_PATH) as *const u16;
    if *path == 0 { return; }
    let h = CreateFileW(path, 0x40000000, 3, 0, 4, 0x80, 0);
    if h == -1 || h == 0 { return; }
    SetFilePointer(h, 0, 0, 2);
    let mut wr = 0u32;
    WriteFile(h, data.as_ptr(), data.len() as u32, &mut wr, 0);
    CloseHandle(h);
}
extern "system" fn crash_filter(p: *mut ExceptionPointers) -> i32 {
    unsafe {
        if !CRASH_LOGGED.swap(true, Ordering::Relaxed) && !p.is_null() {
            let rec = (*p).rec; let ctx = (*p).ctx as usize;
            if !rec.is_null() && ctx != 0 {
                let code = (*rec).code as u64;
                let rip = *((ctx + 0xF8) as *const u64);
                let rsp = *((ctx + 0x98) as *const u64);
                let exe = exe_base() as u64;
                let modb = CRASH_MOD_BASE.load(Ordering::Relaxed);
                let mut buf = [0u8; 2048]; let mut pos = 0usize;
                cb_str(&mut buf, &mut pos, b"\n=== CRASH (tfm2_ai_adjust_2) pid="); cb_hex(&mut buf, &mut pos, GetCurrentProcessId() as u64);
                cb_str(&mut buf, &mut pos, b" code="); cb_hex(&mut buf, &mut pos, code);
                cb_str(&mut buf, &mut pos, b"\nRIP="); cb_hex(&mut buf, &mut pos, rip);
                if rip.wrapping_sub(exe) < 0x8000000 { cb_str(&mut buf, &mut pos, b" = exe+"); cb_hex(&mut buf, &mut pos, rip.wrapping_sub(exe)); }
                if modb != 0 && rip.wrapping_sub(modb) < 0x2000000 { cb_str(&mut buf, &mut pos, b" = MOD+"); cb_hex(&mut buf, &mut pos, rip.wrapping_sub(modb)); }
                cb_str(&mut buf, &mut pos, b"\nstack:\n");
                let mut i = 0usize;
                while i < 0x400 && pos < 1900 {
                    let mut v = 0u64;
                    if pr_rd8(rsp.wrapping_add(i as u64) as usize, &mut v) != 0 {
                        let de = v.wrapping_sub(exe); let dm = v.wrapping_sub(modb);
                        if de < 0x8000000 { cb_str(&mut buf, &mut pos, b"  exe+"); cb_hex(&mut buf, &mut pos, de); cb_put(&mut buf, &mut pos, b'\n'); }
                        else if modb != 0 && dm < 0x2000000 { cb_str(&mut buf, &mut pos, b"  MOD+"); cb_hex(&mut buf, &mut pos, dm); cb_put(&mut buf, &mut pos, b'\n'); }
                    }
                    i += 8;
                }
                cb_str(&mut buf, &mut pos, b"=== end ===\n");
                crash_write(&buf[..pos]);
            }
        }
        let prev = CRASH_PREV.load(Ordering::Relaxed);
        if prev != 0 { let f: extern "system" fn(*mut ExceptionPointers) -> i32 = core::mem::transmute(prev as usize); return f(p); }
        0
    }
}
fn panic_hook_install() {
    let prev = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let n = PANIC_N.fetch_add(1, Ordering::Relaxed);
        if n < 50 {
            let loc = info.location().map(|l| format!("{}:{}:{}", l.file(), l.line(), l.column())).unwrap_or_else(|| "?".into());
            if let Some(p) = pth("panic_log.txt") {
                if let Ok(mut f) = fs::OpenOptions::new().create(true).append(true).open(&p) {
                    let _ = write!(f, "[panic #{}] @ {}\n", n + 1, loc);
                }
            }
        }
        prev(info);
    }));
}
unsafe fn crash_install() {
    if CRASH_INSTALLED.swap(true, Ordering::Relaxed) { return; }
    let mut h: usize = 0;
    if GetModuleHandleExW(0x4 | 0x2, crash_filter as *const () as *const u16, &mut h) != 0 && h != 0 {
        CRASH_MOD_BASE.store(h as u64, Ordering::Relaxed);
    }
    if let Some(p) = pth("crash_log.txt") {
        let s = p.to_string_lossy();
        let dst = core::ptr::addr_of_mut!(CRASH_PATH) as *mut u16;
        let mut i = 0usize;
        for c in s.encode_utf16() { if i < 598 { *dst.add(i) = c; i += 1; } }
        *dst.add(i) = 0;
    }
    let prev = SetUnhandledExceptionFilter(crash_filter as usize);
    CRASH_PREV.store(prev as u64, Ordering::Relaxed);
}

// ════════════════ 경로/로그 ════════════════
fn dir() -> Option<PathBuf> { unsafe {
    let modb = CRASH_MOD_BASE.load(Ordering::Relaxed) as usize;
    let mut buf = [0u16; 1024];
    let n = GetModuleFileNameW(modb, buf.as_mut_ptr(), 1024);
    if n == 0 { return None; }
    let s = String::from_utf16_lossy(&buf[..n as usize]);
    let mut p = PathBuf::from(s);
    p.pop();
    Some(p)
} }
fn pth(name: &str) -> Option<PathBuf> { dir().map(|mut p| { p.push(name); p }) }
fn status_write(s: &str) { if let Some(p) = pth("aiadj2_status.txt") { let _ = fs::OpenOptions::new().create(true).append(true).open(&p).map(|mut f| { let _ = f.write_all(s.as_bytes()); }); } }
fn now_ms() -> u64 { SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0) }

// ════════════════ cfg (평면 statics — 핫패스는 atomic load만) ════════════════
const READY_MIN: u64 = 200;
static READY_TICKS: AtomicU64 = AtomicU64::new(0);
// [B] numbers/tower/stat
static TOWER_THREAT: AtomicI64 = AtomicI64::new(0);
static TOWER_RANGE: AtomicU64 = AtomicU64::new(140000);
static TOWER_DPS: AtomicI64 = AtomicI64::new(8000);
static NUMBERS_THREAT: AtomicI64 = AtomicI64::new(0);
static NUMBERS_THREAT_MOVE: AtomicI64 = AtomicI64::new(0);
static NUMBERS_RANGE: AtomicU64 = AtomicU64::new(150000);
static NUMBERS_RANGE_MOVE: AtomicI64 = AtomicI64::new(-1);
static NUMBERS_MIN_ENEMY: AtomicI64 = AtomicI64::new(1);
static NUMBERS_MIN_ENEMY_MOVE: AtomicI64 = AtomicI64::new(-1);
static NUMBERS_MARGIN: AtomicI64 = AtomicI64::new(0);
static ALLY_TOWER_HP: AtomicI64 = AtomicI64::new(0);
static ALLY_TOWER_HP_MOVE: AtomicI64 = AtomicI64::new(-1);
static ALLY_TOWER_DPS: AtomicI64 = AtomicI64::new(0);
static ALLY_TOWER_DPS_MOVE: AtomicI64 = AtomicI64::new(-1);
static ALLY_TOWER_RANGE: AtomicU64 = AtomicU64::new(150000);
static ALLY_TOWER_RANGE_MOVE: AtomicI64 = AtomicI64::new(-1);
static STAT_INFLUENCE: AtomicI64 = AtomicI64::new(0);
static STAT_NEUTRAL: AtomicI64 = AtomicI64::new(50);
static STAT_POS_DIV: AtomicI64 = AtomicI64::new(2);
static STAT_JUDG_REF: AtomicI64 = AtomicI64::new(100);
static STAT_NOISE_SHIFT: AtomicI64 = AtomicI64::new(5);
static NUMBERS_THREAT_SP: [AtomicI64; 18] = [const { AtomicI64::new(-1) }; 18];
// [C] recall (기본값 = 게임 원본, DIFF=0 검증 당시 값)
static RC_U21_INIT: AtomicI64 = AtomicI64::new(-40);
static RC_EHP_T1: AtomicI64 = AtomicI64::new(80);
static RC_EHP_T2: AtomicI64 = AtomicI64::new(60);
static RC_EHP_T3: AtomicI64 = AtomicI64::new(40);
static RC_EHP_V1: AtomicI64 = AtomicI64::new(90);
static RC_EHP_V2: AtomicI64 = AtomicI64::new(80);
static RC_NORP_BONUS: AtomicI64 = AtomicI64::new(35);
static RC_ED_NEAR: AtomicI64 = AtomicI64::new(130000);
static RC_ED_MID: AtomicI64 = AtomicI64::new(160000);
static RC_ED_FAR: AtomicI64 = AtomicI64::new(200000);
static RC_ED_NEAR_PEN: AtomicI64 = AtomicI64::new(60);
static RC_ED_FAR_BONUS: AtomicI64 = AtomicI64::new(20);
static RC_ED_VFAR_BONUS: AtomicI64 = AtomicI64::new(40);
static RC_AHP_T1: AtomicI64 = AtomicI64::new(70);
static RC_AHP_T2: AtomicI64 = AtomicI64::new(50);
static RC_U13_BONUS: AtomicI64 = AtomicI64::new(10);
static RC_AHP2_PEN: AtomicI64 = AtomicI64::new(30);
static RC_AD_NEAR: AtomicI64 = AtomicI64::new(80000);
static RC_AD_MID: AtomicI64 = AtomicI64::new(120001);
static RC_AD_NEAR_BONUS: AtomicI64 = AtomicI64::new(15);
static RC_AD_FAR_PEN: AtomicI64 = AtomicI64::new(25);
static RC_MULT_BONUS: AtomicI64 = AtomicI64::new(20);
static RC_ALLY_HP_MIN: AtomicI64 = AtomicI64::new(40);
static RC_JOIN_WEIGHT: AtomicI64 = AtomicI64::new(0);
static RC_JOIN_ADV: AtomicI64 = AtomicI64::new(10);
static RC_JOIN_RESCUE: AtomicI64 = AtomicI64::new(6);
static RC_JOIN_DNEAR: AtomicI64 = AtomicI64::new(80000);
static RC_JOIN_DMID: AtomicI64 = AtomicI64::new(160000);
static RC_JOIN_OBJ_MULT: AtomicI64 = AtomicI64::new(2);
static RC_RNG_A_BASE: AtomicI64 = AtomicI64::new(1000);
static RC_RNG_SPREAD_DIV: AtomicI64 = AtomicI64::new(20);
static RC_RNG_CENTER: AtomicI64 = AtomicI64::new(100);
static RC_SCORE_DIV: AtomicI64 = AtomicI64::new(100);
static T_RECALL: AtomicI64 = AtomicI64::new(0);
static RECALL_REPL: AtomicBool = AtomicBool::new(true);
static MP_OVERRIDE: AtomicBool = AtomicBool::new(true);
// poke_f6f720 술어 상수(게임 원본)
static PF_EDGE_MARGIN: AtomicI64 = AtomicI64::new(192000);
static PF_CENTER_BAND: AtomicI64 = AtomicI64::new(704000);
static PF_DIAG_FAR: AtomicI64 = AtomicI64::new(95999);
static PF_DIAG_NEAR: AtomicI64 = AtomicI64::new(63999);
static PF_BAND_WIDTH: AtomicI64 = AtomicI64::new(64000);
// [A] byte-patch 값 (init 1회 적용 — 변경 시 게임 재시작 필요)
static OI_ENABLE: AtomicI64 = AtomicI64::new(0);
static OI_DN_COUNT_GATE: AtomicI64 = AtomicI64::new(0x26);
static OI_DN_NEXUS_HP: AtomicI64 = AtomicI64::new(0x32);
static OI_DN_HP_CRIT: AtomicI64 = AtomicI64::new(0x15);
static OI_DN_HP_LOW: AtomicI64 = AtomicI64::new(0x1f);
static OI_DN_LANE_MARGIN: AtomicI64 = AtomicI64::new(0x78);
static OI_AN_COUNT_GATE: AtomicI64 = AtomicI64::new(5);
static OI_DN_NEAR_DIST: AtomicI64 = AtomicI64::new(120000);
static OI_DN_PRED_DIST: AtomicI64 = AtomicI64::new(240000);
static OI_AN_FINISH_HP: AtomicI64 = AtomicI64::new(0x38);
static OI_AN_CULL_DIST: AtomicI64 = AtomicI64::new(0x5f5e0);
static VIS_WINDOW: AtomicI64 = AtomicI64::new(600);
static D19I_ENABLE: AtomicI64 = AtomicI64::new(0);
static D19_RETREAT_HP: AtomicI64 = AtomicI64::new(45);
static D19_SEV: [AtomicI64; 4] = [AtomicI64::new(49), AtomicI64::new(29), AtomicI64::new(17), AtomicI64::new(9)];
static D19_HP: [AtomicI64; 3] = [AtomicI64::new(66), AtomicI64::new(41), AtomicI64::new(26)];
static D19_ALLY_HP: AtomicI64 = AtomicI64::new(50);
static D19_PHASE_THREAT: AtomicI64 = AtomicI64::new(30);
static D19_PHASE_ALLY: AtomicI64 = AtomicI64::new(39);
// [A2] Phase 2 — 라인전(LineAttack disc0/1/3) + EpicCheck(disc12) 상수. 기본값 = 게임 원본 imm.
static DD_COVER_COUNT: AtomicI64 = AtomicI64::new(2);
static DD_RATIO_THR: AtomicI64 = AtomicI64::new(51);
static DD_NEAR_DIST: AtomicI64 = AtomicI64::new(87890624);
static DD_MAIN_NEAR_DIST: AtomicI64 = AtomicI64::new(87890625);
static EC_OZ_HP: AtomicI64 = AtomicI64::new(50);
// ⛔dd_frontier_mult / aggr_lane / dd_early_p3_thr / dd_cover_p3_thr = 개입 불가(위 [D] 철회 주석 참조).
//   cfg에 써도 무시된다(파서에서 제외) — 게임 원본값 고정.

static CFG_MTIME: AtomicU64 = AtomicU64::new(0);
static CFG_POLL: AtomicU64 = AtomicU64::new(0);
fn mtime_ms(p: &PathBuf) -> u64 {
    fs::metadata(p).and_then(|m| m.modified()).ok()
        .and_then(|t| t.duration_since(UNIX_EPOCH).ok()).map(|d| d.as_millis() as u64).unwrap_or(0)
}
fn load_cfg(force: bool) {
    // 30프레임당 1회만 stat (원본 D-3 최적화 계승)
    if !force && CFG_POLL.fetch_add(1, Ordering::Relaxed) % 30 != 0 { return; }
    let p = match pth(&format!("{}.cfg", MOD_ID)) { Some(p) => p, None => return };
    let mt = mtime_ms(&p);
    if !force && mt == CFG_MTIME.load(Ordering::Relaxed) { return; }
    CFG_MTIME.store(mt, Ordering::Relaxed);
    let txt = match fs::read_to_string(&p) { Ok(t) => t, Err(_) => return };
    for line in txt.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') { continue; }
        let mut it = line.splitn(2, '=');
        let k = match it.next() { Some(k) => k.trim(), None => continue };
        let v: i64 = match it.next().and_then(|v| v.trim().parse().ok()) { Some(v) => v, None => continue };
        // numbers_threat_spN
        if let Some(sp) = k.strip_prefix("numbers_threat_sp") {
            if let Ok(n) = sp.parse::<usize>() { if n < 18 { NUMBERS_THREAT_SP[n].store(v, Ordering::Relaxed); } }
            continue;
        }
        let tgt: &AtomicI64 = match k {
            "tower_threat" => &TOWER_THREAT, "tower_dps" => &TOWER_DPS,
            "numbers_threat" => &NUMBERS_THREAT, "numbers_threat_move" => &NUMBERS_THREAT_MOVE,
            "numbers_range_move" => &NUMBERS_RANGE_MOVE,
            "numbers_min_enemy" => &NUMBERS_MIN_ENEMY, "numbers_min_enemy_move" => &NUMBERS_MIN_ENEMY_MOVE,
            "numbers_margin" => &NUMBERS_MARGIN,
            "ally_tower_hp" => &ALLY_TOWER_HP, "ally_tower_hp_move" => &ALLY_TOWER_HP_MOVE,
            "ally_tower_dps" => &ALLY_TOWER_DPS, "ally_tower_dps_move" => &ALLY_TOWER_DPS_MOVE,
            "ally_tower_range_move" => &ALLY_TOWER_RANGE_MOVE,
            "stat_influence" => &STAT_INFLUENCE, "stat_neutral" => &STAT_NEUTRAL, "stat_pos_div" => &STAT_POS_DIV,
            "stat_judg_ref" => &STAT_JUDG_REF, "stat_noise_shift" => &STAT_NOISE_SHIFT,
            "rc_u21_init" => &RC_U21_INIT, "rc_ehp_t1" => &RC_EHP_T1, "rc_ehp_t2" => &RC_EHP_T2, "rc_ehp_t3" => &RC_EHP_T3,
            "rc_ehp_v1" => &RC_EHP_V1, "rc_ehp_v2" => &RC_EHP_V2, "rc_norp_bonus" => &RC_NORP_BONUS,
            "rc_ed_near" => &RC_ED_NEAR, "rc_ed_mid" => &RC_ED_MID, "rc_ed_far" => &RC_ED_FAR,
            "rc_ed_near_pen" => &RC_ED_NEAR_PEN, "rc_ed_far_bonus" => &RC_ED_FAR_BONUS, "rc_ed_vfar_bonus" => &RC_ED_VFAR_BONUS,
            "rc_ahp_t1" => &RC_AHP_T1, "rc_ahp_t2" => &RC_AHP_T2, "rc_u13_bonus" => &RC_U13_BONUS, "rc_ahp2_pen" => &RC_AHP2_PEN,
            "rc_ad_near" => &RC_AD_NEAR, "rc_ad_mid" => &RC_AD_MID, "rc_ad_near_bonus" => &RC_AD_NEAR_BONUS,
            "rc_ad_far_pen" => &RC_AD_FAR_PEN, "rc_mult_bonus" => &RC_MULT_BONUS, "rc_ally_hp_min" => &RC_ALLY_HP_MIN,
            "rc_join_weight" => &RC_JOIN_WEIGHT, "rc_join_adv" => &RC_JOIN_ADV, "rc_join_rescue" => &RC_JOIN_RESCUE,
            "rc_join_dnear" => &RC_JOIN_DNEAR, "rc_join_dmid" => &RC_JOIN_DMID, "rc_join_obj_mult" => &RC_JOIN_OBJ_MULT,
            "rc_rng_a_base" => &RC_RNG_A_BASE, "rc_rng_spread_div" => &RC_RNG_SPREAD_DIV,
            "rc_rng_center" => &RC_RNG_CENTER, "rc_score_div" => &RC_SCORE_DIV, "t_recall" => &T_RECALL,
            "pf_edge_margin" => &PF_EDGE_MARGIN, "pf_center_band" => &PF_CENTER_BAND,
            "pf_diag_far" => &PF_DIAG_FAR, "pf_diag_near" => &PF_DIAG_NEAR, "pf_band_width" => &PF_BAND_WIDTH,
            "oi_enable" => &OI_ENABLE, "oi_dn_count_gate" => &OI_DN_COUNT_GATE, "oi_dn_nexus_hp" => &OI_DN_NEXUS_HP,
            "oi_dn_hp_crit" => &OI_DN_HP_CRIT, "oi_dn_hp_low" => &OI_DN_HP_LOW, "oi_dn_lane_margin" => &OI_DN_LANE_MARGIN,
            "oi_an_count_gate" => &OI_AN_COUNT_GATE, "oi_dn_near_dist" => &OI_DN_NEAR_DIST, "oi_dn_pred_dist" => &OI_DN_PRED_DIST,
            "oi_an_finish_hp" => &OI_AN_FINISH_HP, "oi_an_cull_dist" => &OI_AN_CULL_DIST,
            "vis_window" => &VIS_WINDOW,
            "d19i_enable" => &D19I_ENABLE, "d19_retreat_hp" => &D19_RETREAT_HP,
            "d19_sev_ratio_0" => &D19_SEV[0], "d19_sev_ratio_1" => &D19_SEV[1], "d19_sev_ratio_2" => &D19_SEV[2], "d19_sev_ratio_3" => &D19_SEV[3],
            "d19_sev_hp_1" => &D19_HP[0], "d19_sev_hp_2" => &D19_HP[1], "d19_sev_hp_3" => &D19_HP[2],
            "d19_ally_hp" => &D19_ALLY_HP, "d19_phase_threat" => &D19_PHASE_THREAT, "d19_phase_ally" => &D19_PHASE_ALLY,
            "dd_cover_count" => &DD_COVER_COUNT, "dd_ratio_thr" => &DD_RATIO_THR,
            "dd_near_dist" => &DD_NEAR_DIST, "dd_main_near_dist" => &DD_MAIN_NEAR_DIST, "ec_oz_hp" => &EC_OZ_HP,
            "recall_repl" => { RECALL_REPL.store(v != 0, Ordering::Relaxed); continue; }
            "mp_override" => { MP_OVERRIDE.store(v != 0, Ordering::Relaxed); continue; }
            "tower_range" => { TOWER_RANGE.store(v.max(0) as u64, Ordering::Relaxed); continue; }
            "numbers_range" => { NUMBERS_RANGE.store(v.max(0) as u64, Ordering::Relaxed); continue; }
            "ally_tower_range" => { ALLY_TOWER_RANGE.store(v.max(0) as u64, Ordering::Relaxed); continue; }
            _ => continue,
        };
        tgt.store(v, Ordering::Relaxed);
    }
}

// ════════════════ byte-patch 엔진 + 3종 적용 (init 1회, 사이트표 = 원본 모드 검증본) ════════════════
unsafe fn patch_imm_bytes(addr: usize, prefix: &[u8], imm_off: usize, width: usize, val: u64) -> bool {
    if !probe_ok(addr, imm_off + width) { return false; }
    for (i, &b) in prefix.iter().enumerate() {
        if rd_u8(addr + i) != b { return false; }   // opcode 불일치 = RVA 어긋남 → skip(크래시 방지)
    }
    let site = addr + imm_off;
    let mut old: u32 = 0;
    if VirtualProtect(site, width, 0x40, &mut old) == 0 { return false; }
    let vb = val.to_le_bytes();
    core::ptr::copy_nonoverlapping(vb.as_ptr(), site as *mut u8, width);
    VirtualProtect(site, width, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), site, width);
    true
}
// ★[Phase 2] imm 직접 패치 — prefix 대신 "현재 imm이 기대한 원본값인가"로 검증(사이트 바이트열 일부만 확보된 경우용).
//   already==new면 no-op 성공(재적용 안전). 불일치=RVA 어긋남/패치판 → skip(크래시 방지).
unsafe fn patch_imm_at(imm_addr: usize, width: usize, expect_old: u64, new_val: u64) -> bool {
    if !probe_ok(imm_addr, width) { return false; }
    let cur: u64 = match width {
        1 => rd_u8(imm_addr) as u64,
        4 => rd_u32(imm_addr) as u64,
        8 => match rd_u64(imm_addr) { Some(v) => v, None => return false },
        _ => return false,
    };
    if cur == new_val { return true; }          // 이미 적용됨
    if cur != expect_old { return false; }      // 기대 원본과 다름 → 안전 skip
    let mut old: u32 = 0;
    if VirtualProtect(imm_addr, width, 0x40, &mut old) == 0 { return false; }
    let vb = new_val.to_le_bytes();
    core::ptr::copy_nonoverlapping(vb.as_ptr(), imm_addr as *mut u8, width);
    VirtualProtect(imm_addr, width, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), imm_addr, width);
    true
}
// ★[Phase 2] 라인전 LineAttack(0x2332cf0 = disc 0/1/3 공통 핸들러) 상수 5사이트.
//   ⚠LineSafe(0x232b4a0=disc4)에도 동일 상수가 있으나 **패치하지 않음** — 원본 모드의 dd_*는 disc0/1/3 전용이었고,
//     disc4는 d4_* 소관(활성 cfg=게임 원본). LineSafe까지 건드리면 원본 대비 초과 개입이 됨.
//   사이트 = 2026-07-19 ghidra-re 0.5.1(buildid 24215274) 디스어셈 실측. imm 주소 = 명령시작+오프셋.
unsafe fn apply_dd_imm() {
    let base = exe_base();
    if base == 0 { return; }
    let cc = DD_COVER_COUNT.load(Ordering::Relaxed).clamp(0, 0x7f) as u64;        // cmp r13,N; jae → count>=N
    let rt = DD_RATIO_THR.load(Ordering::Relaxed).clamp(0, 0x7f) as u64;          // cmp rax,N; cmovb → ratio<N
    let nd = (DD_NEAR_DIST.load(Ordering::Relaxed).max(0) as u64) & 0xffff_ffff;  // ac0변(바깥쪽 비교)
    let md = (DD_MAIN_NEAR_DIST.load(Ordering::Relaxed).max(0) as u64) & 0xffff_ffff;
    let mut ok = 0u32;
    ok += patch_imm_at(base + 0x23330d4 + 3, 1, 2, cc) as u32;                    // 49 83 fd 02  커버 발화 적군수
    ok += patch_imm_at(base + 0x2333bd2 + 3, 1, 0x33, rt) as u32;                 // 48 83 f8 33  커버 비율 임계
    ok += patch_imm_at(base + 0x2333406 + 3, 4, 0x53d1ac0, nd) as u32;            // cmp imm32 (거리² >>8, ac0변)
    ok += patch_imm_at(base + 0x2333797 + 3, 4, 0x53d1ac1, md) as u32;            // cmp imm32 (ac1변 #1)
    ok += patch_imm_at(base + 0x23337d2 + 3, 4, 0x53d1ac1, md) as u32;            // cmp imm32 (ac1변 #2)
    if let Some(p) = pth("dd_imm.txt") {
        let _ = fs::write(p, format!(
            "[LineAttack 0x2332cf0] applied={}/5 cover_count={} ratio_thr={} near={} main_near={} @base{:#x}\n\
             ⚠미적용(게임코드 제약): frontier_mult(강도축소 shl5-2x·리터럴無) / lane_margin(imm8=최대127, 600 불가)\n",
            ok, cc, rt, nd, md, base));
    }
}
// ★[Phase 2] EpicCheck(0x1e17570 = disc12) 오브젝트존 HP 임계 2사이트(상보 비교라 함께 패치).
//   ⚠인코딩: 재현부 `hp_pct > ec_oz_hp` → 게임은 `hp_pct >= ec_oz_hp+1`로 컴파일(원본 50 → imm 0x33=51).
//     따라서 게임 imm = ec_oz_hp + 1.
unsafe fn apply_ec_imm() {
    let base = exe_base();
    if base == 0 { return; }
    let v = (EC_OZ_HP.load(Ordering::Relaxed) + 1).clamp(0, 0x7f) as u64;
    let mut ok = 0u32;
    ok += patch_imm_at(base + 0x1e17773 + 7, 1, 0x33, v) as u32;   // cmp [rbp+0x98],imm8; setae  (hp>oz 변)
    ok += patch_imm_at(base + 0x1e177d0 + 7, 1, 0x33, v) as u32;   // cmp [rbp+0x98],imm8; jb     (hp<=oz 변)
    if let Some(p) = pth("ec_imm.txt") {
        let _ = fs::write(p, format!(
            "[EpicCheck 0x1e17570] applied={}/2 ec_oz_hp={} (게임 imm={}=+1 인코딩) @base{:#x}\n\
             ⚠미적용: ec_count_radius(반경이 사전계산 그리드에 baked·imm 부재)\n",
            ok, EC_OZ_HP.load(Ordering::Relaxed), v, base));
    }
}
unsafe fn apply_objective_imm() {
    let enable = OI_ENABLE.load(Ordering::Relaxed) != 0;
    let base = exe_base();
    if base == 0 { return; }
    let (cg, nh, hc, hl, lm, ag, nd, pd, fh, cd) = if enable {
        (OI_DN_COUNT_GATE.load(Ordering::Relaxed), OI_DN_NEXUS_HP.load(Ordering::Relaxed), OI_DN_HP_CRIT.load(Ordering::Relaxed),
         OI_DN_HP_LOW.load(Ordering::Relaxed), OI_DN_LANE_MARGIN.load(Ordering::Relaxed), OI_AN_COUNT_GATE.load(Ordering::Relaxed),
         OI_DN_NEAR_DIST.load(Ordering::Relaxed), OI_DN_PRED_DIST.load(Ordering::Relaxed), OI_AN_FINISH_HP.load(Ordering::Relaxed),
         OI_AN_CULL_DIST.load(Ordering::Relaxed))
    } else { (0x26, 0x32, 0x15, 0x1f, 0x78, 5, 120000, 240000, 0x38, 0x5f5e0) };
    let b1 = |v: i64| (v.max(0).min(0x7f)) as u64;
    let u32c = |v: i64| (v.max(0) as u64) & 0xffff_ffff;
    let sq = |d: i64| { let d = d.max(0) as u64; d.wrapping_mul(d).wrapping_add(1) };
    let mut ok = 0u32;
    ok += patch_imm_bytes(base + 0x21a4aa5, &[0x48,0x83,0x7d,0xb8], 4, 1, b1(cg)) as u32;
    ok += patch_imm_bytes(base + 0x21a4a95, &[0x48,0x83,0xf8], 3, 1, b1(nh)) as u32;
    ok += patch_imm_bytes(base + 0x21a4ad7, &[0x48,0x83,0xf8], 3, 1, b1(nh)) as u32;
    ok += patch_imm_bytes(base + 0x21a4abb, &[0x48,0x83,0x7d,0x08], 4, 1, b1(hc)) as u32;
    ok += patch_imm_bytes(base + 0x21a4b54, &[0x48,0x83,0x7d,0x08], 4, 1, b1(hc)) as u32;
    ok += patch_imm_bytes(base + 0x21a4b26, &[0x48,0x83,0x7d,0x08], 4, 1, b1(hl)) as u32;
    ok += patch_imm_bytes(base + 0x21ee0f5, &[0x49,0x83,0xc6], 3, 1, b1(lm)) as u32;
    ok += patch_imm_bytes(base + 0x231cd04, &[0x48,0x83,0xbb,0xb0,0x05,0x00,0x00], 7, 1, b1(ag)) as u32;
    ok += patch_imm_bytes(base + 0x21a45dc, &[0x48,0xb8], 2, 8, sq(nd)) as u32;
    ok += patch_imm_bytes(base + 0x21a4629, &[0x48,0xb8], 2, 8, sq(nd)) as u32;
    ok += patch_imm_bytes(base + 0x21ee085, &[0x48,0xb8], 2, 8, sq(pd)) as u32;
    ok += patch_imm_bytes(base + 0x1c7df47, &[0x48,0x83,0xf8], 3, 1, b1(fh)) as u32;
    ok += patch_imm_bytes(base + 0x1c7d5f9, &[0x49,0x81,0xf8], 3, 4, u32c(cd)) as u32;
    if let Some(p) = pth("obj_imm.txt") {
        let _ = fs::write(p, format!("oi_enable={} applied={}/13 cg={} nh={} hc={} hl={} lm={} an={} near={} pred={} fh={} cull={} @base{:#x}\n",
            enable, ok, cg, nh, hc, hl, lm, ag, nd, pd, fh, cd, base));
    }
}
unsafe fn apply_vis_imm() {
    let vw = VIS_WINDOW.load(Ordering::Relaxed);
    let base = exe_base();
    if base == 0 { return; }
    let v = (vw.max(0) as u64) & 0xffff_ffff;
    let ok = patch_imm_bytes(base + 0x1caedd3, &[0x48,0x81,0xc6], 3, 4, v);
    if let Some(p) = pth("vis_imm.txt") { let _ = fs::write(p, format!("vis_window={} applied={}/1 @0x1caedd3 @base{:#x}\n", vw, ok as u32, base)); }
}
unsafe fn apply_disc19_imm() {
    let enable = D19I_ENABLE.load(Ordering::Relaxed) != 0;
    let base = exe_base();
    if base == 0 { return; }
    let (sr0, sr1, sr2, sr3) = (D19_SEV[0].load(Ordering::Relaxed), D19_SEV[1].load(Ordering::Relaxed), D19_SEV[2].load(Ordering::Relaxed), D19_SEV[3].load(Ordering::Relaxed));
    let (sh1, sh2, sh3) = (D19_HP[0].load(Ordering::Relaxed), D19_HP[1].load(Ordering::Relaxed), D19_HP[2].load(Ordering::Relaxed));
    let ah = D19_ALLY_HP.load(Ordering::Relaxed);
    let rh = D19_RETREAT_HP.load(Ordering::Relaxed);
    let pt = D19_PHASE_THREAT.load(Ordering::Relaxed);
    let pa = D19_PHASE_ALLY.load(Ordering::Relaxed);
    let b1 = |v: i64| (v.max(0).min(0x7f)) as u64;
    let (p_sr0, p_sr1, p_sr2, p_sr3, p_sh1, p_sh2, p_sh3, p_ah, p_rha, p_rhb, p_pt, p_pa) = if enable {
        (b1(sr0), b1(sr1), b1(sr2), b1(sr3 + 1), b1(sh1 - 1), b1(sh2 - 1), b1(sh3 - 1), b1(ah), b1(rh), b1(rh + 1), b1(pt - 1), b1(pa - 1))
    } else { (0x31, 0x1d, 0x11, 0x0a, 0x41, 0x28, 0x19, 0x32, 0x2d, 0x2e, 0x1d, 0x26) };
    let mut ok = 0u32;
    ok += patch_imm_bytes(base + 0x1e0e503, &[0x48,0x83,0xf8], 3, 1, p_sr0) as u32;
    ok += patch_imm_bytes(base + 0x1e0e50f, &[0x48,0x83,0xf8], 3, 1, p_sr1) as u32;
    ok += patch_imm_bytes(base + 0x1e0e51b, &[0x48,0x83,0xf8], 3, 1, p_sr2) as u32;
    ok += patch_imm_bytes(base + 0x1e0e529, &[0x48,0x83,0xf8], 3, 1, p_sr3) as u32;
    ok += patch_imm_bytes(base + 0x1e0e509, &[0x49,0x83,0xff], 3, 1, p_sh1) as u32;
    ok += patch_imm_bytes(base + 0x1e0e515, &[0x49,0x83,0xff], 3, 1, p_sh2) as u32;
    ok += patch_imm_bytes(base + 0x1e0e523, &[0x49,0x83,0xff], 3, 1, p_sh3) as u32;
    ok += patch_imm_bytes(base + 0x1e0e589, &[0x48,0x83,0xf8], 3, 1, p_ah) as u32;
    ok += patch_imm_bytes(base + 0x1e0e5d5, &[0x48,0x83,0xf8], 3, 1, p_ah) as u32;
    ok += patch_imm_bytes(base + 0x1e0e4b4, &[0x49,0x83,0xff], 3, 1, p_rha) as u32;
    ok += patch_imm_bytes(base + 0x1e0e5e2, &[0x49,0x83,0xff], 3, 1, p_rhb) as u32;
    ok += patch_imm_bytes(base + 0x1e0e2d7, &[0x48,0x83,0xf9], 3, 1, p_pt) as u32;
    ok += patch_imm_bytes(base + 0x1e0e498, &[0x49,0x83,0xf9], 3, 1, p_pa) as u32;
    ok += patch_imm_bytes(base + 0x1e0e532, &[0x49,0x83,0xf9], 3, 1, p_pa) as u32;
    ok += patch_imm_bytes(base + 0x1e0e5c2, &[0x49,0x83,0xf9], 3, 1, p_pa) as u32;
    if let Some(p) = pth("d19_imm.txt") {
        let _ = fs::write(p, format!("d19i_enable={} applied={}/15 sev=[{} {} {} {}] hp=[{} {} {}] ally={} retreat={} phase=[{} {}] @base{:#x}\n",
            enable, ok, sr0, sr1, sr2, sr3, sh1, sh2, sh3, ah, rh, pt, pa, base));
    }
}

// ════════════════ 게임 구조 헬퍼 (원본 검증본 포팅, 0.5.1 오프셋) ════════════════
#[inline] fn sqd(x1: u64, y1: u64, x2: u64, y2: u64) -> u64 {
    let dx = if x1 >= x2 { x1 - x2 } else { x2 - x1 };
    let dy = if y1 >= y2 { y1 - y2 } else { y2 - y1 };
    dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy))
}
#[inline] fn isqrt(n: u64) -> u64 {
    if n == 0 { return 0; }
    let mut x = n; let mut y = (x + 1) >> 1;
    while y < x { x = y; y = (x + n / x) >> 1; }
    x
}
// SlotMap 4단 chase: sim(SimulationStateP) + handle → 엔티티 (0.5.0/0.5.1 오프셋)
// ★[07-20 최적화] 헤더 8필드(0x720..0x868)를 프로브 1회 + raw로 배치읽기, 각 테이블 엔트리도 프로브+raw.
//   구 12~13 VEH콜 → 4~5콜. 같은 동기 스코프 = 확립 패턴.
unsafe fn dd7_slot128(sim: usize, h: u64) -> usize {
    if !probe_ok(sim + 0x720, 0x148) { return 0; }
    if h >= raw_u64(sim + 0x860) { return 0; }
    let t1 = raw_u64(sim + 0x858) as usize;
    let e1 = t1 + (h as usize) * 0x10;
    if !ptr_ok(t1) || !probe_ok(e1, 0x10) || raw_i32(e1) != 1 { return 0; }
    let u1 = raw_u64(e1 + 8);
    if u1 >= raw_u64(sim + 0x848) { return 0; }
    let s808 = raw_u64(sim + 0x840) as usize;
    let rec = s808 + (u1 as usize) * 0x8d0;
    if !ptr_ok(s808) || !probe_ok(rec + 0x8b8, 0x10) || raw_u8(rec + 0x8b8) == 0 { return 0; }
    let u2 = raw_u64(rec + 0x8c0);
    if u2 >= raw_u64(sim + 0x740) { return 0; }
    let s700 = raw_u64(sim + 0x738) as usize;
    let e3 = s700 + (u2 as usize) * 0x10;
    if !ptr_ok(s700) || !probe_ok(e3, 0x10) || raw_i32(e3) != 1 { return 0; }
    let u3 = raw_u64(e3 + 8);
    if u3 >= raw_u64(sim + 0x728) { return 0; }
    (u3 as usize) * 0x6a8 + raw_u64(sim + 0x720) as usize
}
// vt0x68 재현: now-visible 체크 (disc19_repro 검증본)
#[inline] unsafe fn geom_vt68(gc: usize, side: usize, key: u64) -> bool {
    if key < rd_u64(gc + 0x740).unwrap_or(0) {
        let t3 = rd_u64(gc + 0x738).unwrap_or(0) as usize;
        if ptr_ok(t3) && rd_i32(t3 + (key as usize) * 0x10).unwrap_or(0) == 1 {
            let u = rd_u64(t3 + (key as usize) * 0x10 + 8).unwrap_or(0);
            if u < rd_u64(gc + 0x728).unwrap_or(0) && side < 2 {
                let e = (u as usize) * 0x6a8 + rd_u64(gc + 0x720).unwrap_or(0) as usize;
                return rd_u64(e + 0x38 + side * 0x18).unwrap_or(1) == 0;
            }
        }
    }
    false
}
// vt0xc0 재현: 시야레코드 lookup (L2 roster 선형탐색)
// ★[07-20 최적화] 레코드당 2 VEH콜 → 프로브 1회+raw 2읽기(선형스캔이 recall 최악비용이라 절반 절감).
#[inline] unsafe fn geom_vtc0(gc: usize, key: u64) -> usize {
    let base = rd_u64(gc + 0x840).unwrap_or(0) as usize;
    let cnt = rd_u64(gc + 0x848).unwrap_or(0) as usize;
    if !ptr_ok(base) { return 0; }
    for i in 0..cnt.min(4096) {
        let rec = base + i * 0x8d0;
        if !probe_ok(rec + 0x8b8, 0x10) { continue; }
        if raw_u64(rec + 0x8b8) == 0 { continue; }
        if raw_u64(rec + 0x8c0) == key { return rec; }
    }
    0
}
// poke 후보 유효성 (원본 cand_valid 검증본)
unsafe fn cand_valid(robj: usize, team: u64, lvar16: usize, cand: usize) -> Option<bool> {
    let uv8 = rd_u64(cand + 0x5a8)? as usize;
    if geom_vt68(robj, team as usize, uv8 as u64) { return Some(true); }
    let a8 = geom_vtc0(robj, uv8 as u64);
    if a8 == 0 { return Some(false); }
    let idx = rd_u32(a8 + 0x8b0) as usize;
    let lv = rd_u64(lvar16 + 0x1e0 + idx * 8)?;
    let timing = rd_i64(robj + 0xeac0).unwrap_or(0) as u64;
    Some(timing <= lv + 0x78)
}
// FUN_141db8960 레인밴드 predicate 3모드 (원본 검증본)
unsafe fn poke_f6f720_m0(node: usize, x: u64, y: u64) -> bool {
    let m = rd_u64(node + 8).unwrap_or(0) as usize;
    if !ptr_ok(m) { return true; }
    let ymax = rd_u64(m + 0x12c0).unwrap_or(0);
    let xmax = rd_u64(m + 0x12b8).unwrap_or(0);
    let edge = PF_EDGE_MARGIN.load(Ordering::Relaxed) as u64;
    let band = PF_CENTER_BAND.load(Ordering::Relaxed) as u64;
    let u5 = ymax.wrapping_sub(y);
    let u6 = if x < u5 { u5 } else { x };
    if u6 <= edge { return true; }
    let u6m = if u5 < x { u5 } else { x };
    if u6m >= xmax.wrapping_sub(edge) { return true; }
    let h1 = xmax.wrapping_sub(band) >> 1;
    let cond_a = h1.wrapping_add(band) < x || x < h1;
    let h2 = ymax.wrapping_sub(band) >> 1;
    let cond_b = h2.wrapping_add(band) < y || y < h2;
    let uvar6 = u5.wrapping_sub(x);
    let mut uvar4: u64;
    if cond_a || cond_b {
        if u5 < edge + 1 || u5 < x { return false; }
        uvar4 = x.wrapping_sub(u5);
    } else {
        uvar4 = if x <= u5 { uvar6 } else { x.wrapping_sub(u5) };
        if u5 < edge + 1 { return false; }
        if u5 < x { return false; }
        if (PF_DIAG_FAR.load(Ordering::Relaxed) as u64) < uvar4 { return false; }
        uvar4 = (0u64).wrapping_sub(uvar6);
    }
    if x < u5 { uvar4 = uvar6; }
    (PF_DIAG_NEAR.load(Ordering::Relaxed) as u64) < uvar4
}
unsafe fn poke_f6f720_m1(node: usize, x: u64, y: u64) -> bool {
    let m = rd_u64(node + 8).unwrap_or(0) as usize;
    if !ptr_ok(m) { return true; }
    let ymax = rd_u64(m + 0x12c0).unwrap_or(0);
    let xmax = rd_u64(m + 0x12b8).unwrap_or(0);
    let edge = PF_EDGE_MARGIN.load(Ordering::Relaxed) as u64;
    let dy = ymax.wrapping_sub(y);
    let big = if x < dy { dy } else { x };
    if big <= edge { return true; }
    let small = if dy < x { dy } else { x };
    if small >= xmax.wrapping_sub(edge) { return true; }
    let d = if dy < x { x.wrapping_sub(dy) } else { dy.wrapping_sub(x) };
    d < PF_BAND_WIDTH.load(Ordering::Relaxed) as u64
}
unsafe fn poke_f6f720_m2(vobj: usize, cx: u64, cy: u64) -> bool {
    // mode2 원본(dd7_f6f720_m2)은 pf_* 미튜닝 상수 하드코딩 버전과 동일 로직 — m0에 위임하지 않고 원본 그대로.
    let m = rd_u64(vobj + 8).unwrap_or(0) as usize;
    if !ptr_ok(m) { return true; }
    let ymax = rd_u64(m + 0x12c0).unwrap_or(0);
    let u5 = ymax.wrapping_sub(cy);
    let mut u6 = if cx < u5 { u5 } else { cx };
    if u6 <= 0x2ee00 { return true; }
    u6 = if u5 < cx { u5 } else { cx };
    let xmax = rd_u64(m + 0x12b8).unwrap_or(0);
    if u6 >= xmax.wrapping_sub(0x2ee00) { return true; }
    let h1 = xmax.wrapping_sub(0xabe00) >> 1;
    let cond1 = h1.wrapping_add(0xabe00) < cx || cx < h1;
    let h2 = ymax.wrapping_sub(0xabe00) >> 1;
    let cond2 = h2.wrapping_add(0xabe00) < cy || cy < h2;
    if cond1 || cond2 {
        if 0x2ee00 < cx {
            let u6b = cx.wrapping_sub(u5);
            if u5 <= cx { return 63999 < u6b; }
        }
        false
    } else {
        let u6c = u5.wrapping_sub(cx);
        let u4 = if cx <= u5 { u6c } else { cx.wrapping_sub(u5) };
        if (cx > u5 || u6c == 0) && 0x2ee00 < cx && u4 < 96000 {
            return 63999 < (0u64).wrapping_sub(u6c);
        }
        false
    }
}
#[inline] unsafe fn poke_f6f720(node: usize, x: u64, y: u64, mode: u8) -> bool {
    match mode { 0 => poke_f6f720_m0(node, x, y), 1 => poke_f6f720_m1(node, x, y), 2 => poke_f6f720_m2(node, x, y), _ => true }
}

// ════════════════ ChaCha12 (recall RNG writeback용, 원본 검증본) ════════════════
static USE_SIMD_CHACHA: AtomicBool = AtomicBool::new(false);
#[inline] fn chacha_qr(s: &mut [u32; 16], a: usize, b: usize, c: usize, d: usize) {
    s[a] = s[a].wrapping_add(s[b]); s[d] ^= s[a]; s[d] = s[d].rotate_left(16);
    s[c] = s[c].wrapping_add(s[d]); s[b] ^= s[c]; s[b] = s[b].rotate_left(12);
    s[a] = s[a].wrapping_add(s[b]); s[d] ^= s[a]; s[d] = s[d].rotate_left(8);
    s[c] = s[c].wrapping_add(s[d]); s[b] ^= s[c]; s[b] = s[b].rotate_left(7);
}
fn chacha12_block(key: &[u32; 8], counter: u64, nonce: u64, out: &mut [u32; 16]) {
    let mut s = [0x61707865u32, 0x3320646e, 0x79622d32, 0x6b206574,
                 key[0], key[1], key[2], key[3], key[4], key[5], key[6], key[7],
                 counter as u32, (counter >> 32) as u32, nonce as u32, (nonce >> 32) as u32];
    let init = s;
    for _ in 0..6 {
        chacha_qr(&mut s, 0, 4, 8, 12); chacha_qr(&mut s, 1, 5, 9, 13); chacha_qr(&mut s, 2, 6, 10, 14); chacha_qr(&mut s, 3, 7, 11, 15);
        chacha_qr(&mut s, 0, 5, 10, 15); chacha_qr(&mut s, 1, 6, 11, 12); chacha_qr(&mut s, 2, 7, 8, 13); chacha_qr(&mut s, 3, 4, 9, 14);
    }
    for i in 0..16 { out[i] = s[i].wrapping_add(init[i]); }
}
#[cfg(target_arch = "x86_64")]
unsafe fn chacha12_4block_sse2(key: &[u32; 8], base: u64, nonce: u64, out: *mut u32) {
    use core::arch::x86_64::*;
    #[inline] unsafe fn vqr(v: &mut [__m128i; 16], a: usize, b: usize, c: usize, d: usize) {
        let (mut va, mut vb, mut vc, mut vd) = (v[a], v[b], v[c], v[d]);
        va = _mm_add_epi32(va, vb); vd = _mm_xor_si128(vd, va); vd = _mm_or_si128(_mm_slli_epi32::<16>(vd), _mm_srli_epi32::<16>(vd));
        vc = _mm_add_epi32(vc, vd); vb = _mm_xor_si128(vb, vc); vb = _mm_or_si128(_mm_slli_epi32::<12>(vb), _mm_srli_epi32::<20>(vb));
        va = _mm_add_epi32(va, vb); vd = _mm_xor_si128(vd, va); vd = _mm_or_si128(_mm_slli_epi32::<8>(vd), _mm_srli_epi32::<24>(vd));
        vc = _mm_add_epi32(vc, vd); vb = _mm_xor_si128(vb, vc); vb = _mm_or_si128(_mm_slli_epi32::<7>(vb), _mm_srli_epi32::<25>(vb));
        v[a] = va; v[b] = vb; v[c] = vc; v[d] = vd;
    }
    let b0 = base; let b1 = base.wrapping_add(1); let b2 = base.wrapping_add(2); let b3 = base.wrapping_add(3);
    let mut v = [
        _mm_set1_epi32(0x61707865u32 as i32), _mm_set1_epi32(0x3320646eu32 as i32),
        _mm_set1_epi32(0x79622d32u32 as i32), _mm_set1_epi32(0x6b206574u32 as i32),
        _mm_set1_epi32(key[0] as i32), _mm_set1_epi32(key[1] as i32), _mm_set1_epi32(key[2] as i32), _mm_set1_epi32(key[3] as i32),
        _mm_set1_epi32(key[4] as i32), _mm_set1_epi32(key[5] as i32), _mm_set1_epi32(key[6] as i32), _mm_set1_epi32(key[7] as i32),
        _mm_setr_epi32(b0 as u32 as i32, b1 as u32 as i32, b2 as u32 as i32, b3 as u32 as i32),
        _mm_setr_epi32((b0 >> 32) as u32 as i32, (b1 >> 32) as u32 as i32, (b2 >> 32) as u32 as i32, (b3 >> 32) as u32 as i32),
        _mm_set1_epi32(nonce as u32 as i32), _mm_set1_epi32((nonce >> 32) as u32 as i32),
    ];
    let init = v;
    for _ in 0..6 {
        vqr(&mut v, 0, 4, 8, 12); vqr(&mut v, 1, 5, 9, 13); vqr(&mut v, 2, 6, 10, 14); vqr(&mut v, 3, 7, 11, 15);
        vqr(&mut v, 0, 5, 10, 15); vqr(&mut v, 1, 6, 11, 12); vqr(&mut v, 2, 7, 8, 13); vqr(&mut v, 3, 4, 9, 14);
    }
    let mut tmp = [0u32; 4];
    for i in 0..16 {
        let s = _mm_add_epi32(v[i], init[i]);
        _mm_storeu_si128(tmp.as_mut_ptr() as *mut __m128i, s);
        *out.add(i) = tmp[0]; *out.add(16 + i) = tmp[1]; *out.add(32 + i) = tmp[2]; *out.add(48 + i) = tmp[3];
    }
}
#[inline] unsafe fn chacha12_4block(key: &[u32; 8], base: u64, nonce: u64, out: *mut u32) {
    #[cfg(target_arch = "x86_64")]
    { if USE_SIMD_CHACHA.load(Ordering::Relaxed) { chacha12_4block_sse2(key, base, nonce, out); return; } }
    for b in 0..4u64 { let mut blk = [0u32; 16]; chacha12_block(key, base.wrapping_add(b), nonce, &mut blk); for w in 0..16 { *out.add(b as usize * 16 + w) = blk[w]; } }
}
fn chacha_simd_selftest() -> bool {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        let cases: [([u32; 8], u64, u64); 4] = [
            ([0, 0, 0, 0, 0, 0, 0, 0], 0, 0),
            ([1, 2, 3, 4, 5, 6, 7, 8], 0xffff_fffeu64, 0x1234_5678_9abc_def0),
            ([0xdead_beef, 0x1234_5678, 0, 0xffff_ffff, 7, 0x8000_0000, 3, 0x5555_5555], 0x0000_0000_ffff_ffff, 0),
            ([9, 8, 7, 6, 5, 4, 3, 2], 0xffff_ffff_ffff_fffd, 0xdead),
        ];
        for (key, base, nonce) in cases.iter() {
            let mut scal = [0u32; 64];
            for b in 0..4u64 { let mut blk = [0u32; 16]; chacha12_block(key, base.wrapping_add(b), *nonce, &mut blk); for w in 0..16 { scal[b as usize * 16 + w] = blk[w]; } }
            let mut simd = [0u32; 64];
            chacha12_4block_sse2(key, *base, *nonce, simd.as_mut_ptr());
            if scal != simd { return false; }
        }
        return true;
    }
    #[allow(unreachable_code)] { false }
}
// u32 gen_range write-back (fc59a0 recall 전용, 원본 검증본): 게임 RNG state를 원본과 비트동일 전진.
unsafe fn rng_advance_writeback_u32(state: usize, lo: i64, range: u64) -> Option<i64> {
    if range == 0 { return None; }
    let mut buf = [0u32; 64];
    let mut idx = rd_u64(state + 0x100)? as usize;
    let mut refills = 0u64;
    let input = state + 0x110;
    let before_counter = rd_u64(input + 0x20)?;
    let mut key = [0u32; 8]; for k in 0..8 { key[k] = rd_u32(input + k * 4); }
    let nonce = rd_u64(input + 0x28).unwrap_or(0);
    let mut iv: i32 = 0x1f; while (range >> iv) == 0 { iv -= 1; if iv < 0 { return None; } }
    let shift = ((!iv) & 0x1f) as u32;
    let zone = ((range << shift).wrapping_sub(1)) & 0xffff_ffff;
    let mut guard = 0;
    let result = loop {
        guard += 1; if guard > 256 { return None; }
        if idx >= 0x40 {
            let base = before_counter.wrapping_add(4u64.wrapping_mul(refills));
            chacha12_4block(&key, base, nonce, buf.as_mut_ptr());
            refills += 1; idx = 0;
        }
        let raw = (if refills == 0 { rd_u32(state + idx * 4) } else { buf[idx] }) as u64; idx += 1;
        let prod = raw.wrapping_mul(range);
        if zone < (prod & 0xffff_ffff) { continue; }
        break lo + (prod >> 32) as i64;
    };
    if refills > 0 {
        for i in 0..64 { if !wr_u32(state + i * 4, buf[i]) { return None; } }
        if !wr_u64(input + 0x20, before_counter.wrapping_add(4u64.wrapping_mul(refills))) { return None; }
    }
    if !wr_u64(state + 0x100, idx as u64) { return None; }
    Some(result)
}

// ════════════════ [C] recall (fc59a0) 완전대체 — 원본 my_recall_mult/my_fc59a0_full 검증본 포팅 ════════════════
unsafe fn my_recall_mult(sim: usize, p4: usize, mode: u8) -> Option<(i64, bool)> {
    let team = rd_u64(sim + 0x820).unwrap_or(9);
    if team > 1 { return None; }
    let other = 1u64.wrapping_sub(team);
    let l78 = rd_u64(p4).unwrap_or(0) as usize;
    let vobj_f6 = rd_u64(p4 + 8).unwrap_or(0) as usize;
    let geo = rd_u64(p4 + 0x10).unwrap_or(0) as usize;
    if !ptr_ok(l78) || !ptr_ok(vobj_f6) || !ptr_ok(geo) { return None; }
    let self_obj = rd_u64(l78).unwrap_or(0) as usize;
    let rvt = rd_u64(l78 + 8).unwrap_or(0) as usize;
    if !ptr_ok(self_obj) || !ptr_ok(rvt) { return None; }
    // 빌더1 적위협
    let enemy_base = l78 + (other as usize) * 0x28 + 0x1e0;
    let lvar16 = (other as usize) * 0x2e8 + geo;
    let mut enemies = [0usize; 5]; let mut d0 = 0usize;
    for k in 0..5usize {
        let c = rd_u64(enemy_base + k * 8).unwrap_or(0) as usize;
        if c == 0 { continue; }
        let (cx, cy) = (rd_u64(c + 0x648).unwrap_or(0), rd_u64(c + 0x650).unwrap_or(0));
        if !poke_f6f720(vobj_f6, cx, cy, mode) { continue; }
        match cand_valid(self_obj, team, lvar16, c) { Some(true) => { enemies[d0] = c; d0 += 1; }, Some(false) => {}, None => return None }
    }
    if d0 == 0 { return Some((0, false)); }
    // 빌더2 아군오브젝트
    let ally_base = l78 + (team as usize) * 0x28 + 0x1e0;
    let pred_base = geo + (team as usize) * 0x2e8;
    let ally_hp_min = RC_ALLY_HP_MIN.load(Ordering::Relaxed);
    let mut allies = [0usize; 5]; let mut b0 = 0usize;
    for k in 0..5usize {
        if rd_u8(pred_base + 0xf8 + k * 0x20) != 0 || rd_u8(pred_base + 0xf9 + k * 0x20) != mode { continue; }
        let c = rd_u64(ally_base + k * 8).unwrap_or(0) as usize;
        if c == 0 { continue; }
        let mx = rd_u64(c + 0x610).unwrap_or(0); if mx == 0 { continue; }
        if (rd_u64(c + 0x658).unwrap_or(0).wrapping_mul(100) / mx) as i64 > ally_hp_min { allies[b0] = c; b0 += 1; }
    }
    let self_ref = dd7_slot128(self_obj, rd_u64(sim + 0x818).unwrap_or(0));
    if b0 == 0 || b0 + 1 < d0 || self_ref == 0 { return Some((0, false)); }
    // 최근접 적
    let (srx, sry) = (rd_u64(self_ref + 0x648).unwrap_or(0), rd_u64(self_ref + 0x650).unwrap_or(0));
    let mut ne = enemies[0]; let mut nd = sqd(srx, sry, rd_u64(ne + 0x648).unwrap_or(0), rd_u64(ne + 0x650).unwrap_or(0));
    for &e in &enemies[1..d0] { let d = sqd(srx, sry, rd_u64(e + 0x648).unwrap_or(0), rd_u64(e + 0x650).unwrap_or(0)); if d < nd { nd = d; ne = e; } }
    let (ex, ey) = (rd_u64(ne + 0x648).unwrap_or(0), rd_u64(ne + 0x650).unwrap_or(0));
    // 최근접 아군오브젝트
    let mut na = allies[0]; let mut ad = sqd(ex, ey, rd_u64(na + 0x648).unwrap_or(0), rd_u64(na + 0x650).unwrap_or(0));
    for &a in &allies[1..b0] { let d = sqd(ex, ey, rd_u64(a + 0x648).unwrap_or(0), rd_u64(a + 0x650).unwrap_or(0)); if d < ad { ad = d; na = a; } }
    // 블록1: 적 HP%
    let emx = rd_u64(ne + 0x610).unwrap_or(0); if emx == 0 { return None; }
    let ehp = (rd_u64(ne + 0x658).unwrap_or(0).wrapping_mul(100) / emx) as i64;
    let mut u21: i64 = RC_U21_INIT.load(Ordering::Relaxed);
    let ehp_t1 = RC_EHP_T1.load(Ordering::Relaxed);
    let ehp_t2 = RC_EHP_T2.load(Ordering::Relaxed);
    let ehp_t3 = RC_EHP_T3.load(Ordering::Relaxed);
    let ehp_v2 = RC_EHP_V2.load(Ordering::Relaxed);
    if ehp < ehp_t1 {
        if ehp < ehp_t2 { u21 = (if ehp < ehp_t3 { RC_EHP_V1.load(Ordering::Relaxed) } else { ehp_v2 }) - ehp; }
        else { u21 = (ehp_v2 - ehp) >> 1; }
    }
    // 블록2: 리콜포인트 → 적 거리
    let ri = (mode as i64) * 4 - team as i64;
    let mut rp = rd_u64(l78 + ((ri + 0x31) as usize) * 8).unwrap_or(0) as usize;
    if rp == 0 { rp = rd_u64(l78 + ((ri + 0x33) as usize) * 8).unwrap_or(0) as usize; }
    if rp == 0 { u21 += RC_NORP_BONUS.load(Ordering::Relaxed); }
    else {
        let d = isqrt(sqd(rd_u64(rp + 0x648).unwrap_or(0), rd_u64(rp + 0x650).unwrap_or(0), ex, ey));
        if d < RC_ED_NEAR.load(Ordering::Relaxed) as u64 { u21 -= RC_ED_NEAR_PEN.load(Ordering::Relaxed); }
        else if d < RC_ED_MID.load(Ordering::Relaxed) as u64 {}
        else if d < RC_ED_FAR.load(Ordering::Relaxed) as u64 { u21 += RC_ED_FAR_BONUS.load(Ordering::Relaxed); }
        else { u21 += RC_ED_VFAR_BONUS.load(Ordering::Relaxed); }
    }
    // 블록3: 아군오브젝트 HP% + obj→적 거리
    let amx = rd_u64(na + 0x610).unwrap_or(0); if amx == 0 { return None; }
    let ahp = (rd_u64(na + 0x658).unwrap_or(0).wrapping_mul(100) / amx) as i64;
    let mut u13 = u21 + RC_U13_BONUS.load(Ordering::Relaxed);
    if ahp < RC_AHP_T1.load(Ordering::Relaxed) { u13 = u21; }
    if ahp < RC_AHP_T2.load(Ordering::Relaxed) { u13 = u21 - RC_AHP2_PEN.load(Ordering::Relaxed); }
    let ad2 = isqrt(sqd(rd_u64(na + 0x648).unwrap_or(0), rd_u64(na + 0x650).unwrap_or(0), ex, ey));
    let u21f = if ad2 < RC_AD_NEAR.load(Ordering::Relaxed) as u64 { u13 + RC_AD_NEAR_BONUS.load(Ordering::Relaxed) }
               else if ad2 < RC_AD_MID.load(Ordering::Relaxed) as u64 { u13 }
               else { u13 - RC_AD_FAR_PEN.load(Ordering::Relaxed) };
    let base_mult = if (b0 as i64) + 1 <= d0 as i64 { u21f } else { u21f + RC_MULT_BONUS.load(Ordering::Relaxed) };
    // 합류 이득 항(신규): max(체력기반, 합류기반)
    let join_w = RC_JOIN_WEIGHT.load(Ordering::Relaxed);
    let mult = if join_w == 0 { base_mult } else {
        let sit = if (b0 as i64) + 1 > d0 as i64 {
            ((b0 as i64) + 1 - d0 as i64) * RC_JOIN_ADV.load(Ordering::Relaxed)
        } else {
            (d0 as i64 - (b0 as i64)) * RC_JOIN_RESCUE.load(Ordering::Relaxed)
        };
        let jd = isqrt(sqd(srx, sry, rd_u64(na + 0x648).unwrap_or(0), rd_u64(na + 0x650).unwrap_or(0)));
        let distf = if jd < RC_JOIN_DNEAR.load(Ordering::Relaxed) as u64 { 3 }
                    else if jd < RC_JOIN_DMID.load(Ordering::Relaxed) as u64 { 2 } else { 1 };
        let objf = if rp != 0 { RC_JOIN_OBJ_MULT.load(Ordering::Relaxed) } else { 1 };
        let join_score = sit * distf * objf * join_w / 10;
        base_mult.max(join_score)
    };
    Some((mult, true))
}
#[inline(never)] unsafe fn my_fc59a0_full(out: usize, prng: usize, sim: usize, p4: usize, mode: u8, p6: i64) -> Option<()> {
    if !probe_ok(out, 0x10) { return None; }
    let (mult, rng_drawn) = my_recall_mult(sim, p4, mode)?;
    if !rng_drawn {
        std::ptr::write_unaligned(out as *mut i32, 0i32);
        std::ptr::write_unaligned((out + 4) as *mut u8, 0u8);
        std::ptr::write_unaligned((out + 8) as *mut i32, 0i32);
        return Some(());
    }
    let a = rd_i64(sim + 0x218).unwrap_or(-1);
    let a_base = RC_RNG_A_BASE.load(Ordering::Relaxed);
    if a < 0 || a > a_base { return None; }
    let uv7 = ((a_base - a) / RC_RNG_SPREAD_DIV.load(Ordering::Relaxed).max(1)) as u64;
    let m = rng_advance_writeback_u32(prng, RC_RNG_CENTER.load(Ordering::Relaxed) - uv7 as i64, 2 * uv7 + 1)?;
    let score = (((m * mult) / RC_SCORE_DIV.load(Ordering::Relaxed).max(1)) + T_RECALL.load(Ordering::Relaxed)) as i32;
    std::ptr::write_unaligned(out as *mut i32, score);
    std::ptr::write_unaligned((out + 4) as *mut u8, if p6 <= score as i64 { 1u8 } else { 0u8 });
    std::ptr::write_unaligned((out + 8) as *mut i32, mult as i32);
    Some(())
}
const RAX_SENT: i64 = i64::MIN;
static RECALL_N: AtomicU64 = AtomicU64::new(0);
static RECALL_PASS: AtomicU64 = AtomicU64::new(0);
unsafe extern "C" fn fc59a0_cap(saved: usize, entry_rsp: usize) -> i64 {
    if READY_TICKS.load(Ordering::Relaxed) < READY_MIN { return RAX_SENT; }
    if !RECALL_REPL.load(Ordering::Relaxed) { return RAX_SENT; }
    let p1 = rd_u64(saved + 0x28).unwrap_or(0) as usize;      // rcx = out
    let prng = rd_u64(saved + 0x20).unwrap_or(0) as usize;    // rdx = RNG state
    let sim = rd_u64(saved + 0x18).unwrap_or(0) as usize;     // r8  = sim
    let p4 = rd_u64(saved + 0x10).unwrap_or(0) as usize;      // r9  = cand src
    let mode = rd_u8(entry_rsp + 0x28);                       // arg5
    let p6 = rd_i32(entry_rsp + 0x30).unwrap_or(0) as i64;    // arg6 = threshold
    if ptr_ok(p1) && ptr_ok(prng) && ptr_ok(sim) && ptr_ok(p4) && probe_ok(prng + 0x130, 8) && probe_ok(sim + 0x220, 8) {
        let done = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| my_fc59a0_full(p1, prng, sim, p4, mode, p6))).unwrap_or(None).is_some();
        if done { RECALL_N.fetch_add(1, Ordering::Relaxed); return p1 as i64; }   // HANDLED → rax=out, 원본 skip
    }
    RECALL_PASS.fetch_add(1, Ordering::Relaxed);
    RAX_SENT   // passthrough(원본이 출력+RNG 소비 — desync 없음: 아직 미드로우 지점)
}

// ════════════════ [B] numbers/tower/stat 후퇴 — movepri 원본 실행 후 출력 override ════════════════
#[inline] fn apos(a: &AtomicI64) -> i64 { a.load(Ordering::Relaxed) }
unsafe fn tower_in_range(tw: usize, sx: u64, sy: u64, tr2: u64) -> bool {
    // ★[07-20] 프로브 1회(+0x68..+0x658) + raw 3읽기 (구 3-4 VEH콜)
    if !ptr_ok(tw) || !probe_ok(tw + 0x68, 0x5f0) { return false; }
    if raw_i32(tw + 0x68) != 2 { return false; }
    let tx = raw_u64(tw + 0x648); let ty = raw_u64(tw + 0x650);
    let dx = if sx >= tx { sx - tx } else { tx - sx };
    let dy = if sy >= ty { sy - ty } else { ty - sy };
    dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy)) < tr2
}
unsafe fn ally_tower_contrib(rh: usize, team: i64, sx: u64, sy: u64, base_code: u8) -> (u128, u128) {
    if team < 0 || team > 1 { return (0, 0); }
    let pick = |base: i64, mv: i64| -> i64 { if base_code == 2 { if mv >= 0 { mv } else { base } } else { base } };
    let hp_w = pick(apos(&ALLY_TOWER_HP), apos(&ALLY_TOWER_HP_MOVE)).clamp(0, 100) as u128;
    let dps_w = pick(apos(&ALLY_TOWER_DPS), apos(&ALLY_TOWER_DPS_MOVE)).clamp(0, 100) as u128;
    if hp_w == 0 && dps_w == 0 { return (0, 0); }
    let q = team as usize;
    let r_base = ALLY_TOWER_RANGE.load(Ordering::Relaxed);
    let r = if base_code == 2 { let m = apos(&ALLY_TOWER_RANGE_MOVE); if m >= 0 { m as u64 } else { r_base } } else { r_base };
    let r2 = r.wrapping_mul(r);
    let dps = TOWER_DPS.load(Ordering::Relaxed).max(0) as u128;
    let (mut sum_hp, mut cnt) = (0u128, 0u128);
    let mut acc = |t: usize| {
        // ★[07-20] 프로브 1회(+0x648..+0x660) + raw 3읽기 (구 3 VEH콜)
        if t < 0x10000 || !probe_ok(t + 0x648, 0x18) { return; }
        let hp = raw_u64(t + 0x658); if hp == 0 { return; }
        let tx = raw_u64(t + 0x648); let ty = raw_u64(t + 0x650);
        let dx = if tx >= sx { tx - sx } else { sx - tx };
        let dy = if ty >= sy { ty - sy } else { sy - ty };
        if dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy)) < r2 { sum_hp = sum_hp.wrapping_add(hp as u128); cnt += 1; }
    };
    for &off in &[0x170usize, 0x180, 0x190, 0x1a0, 0x1b0, 0x1c0, 0x1d0] { acc(rd_u64(rh + off + q * 8).unwrap_or(0) as usize); }
    let vb = rd_u64(rh + 0x130 + q * 0x20).unwrap_or(0) as usize;
    let vl = rd_u64(rh + 0x148 + q * 0x20).unwrap_or(0);
    if ptr_ok(vb) && vl <= 32 { for i in 0..vl as usize { acc(rd_u64(vb + i * 8).unwrap_or(0) as usize); } }
    (sum_hp.wrapping_mul(hp_w) / 100, cnt.wrapping_mul(dps).wrapping_mul(dps_w) / 100)
}
unsafe fn combat_balance(rh: usize, team: i64, sx: u64, sy: u64, base_code: u8) -> Option<(i64, u32, u32)> {
    if !ptr_ok(rh) || team < 0 || team > 1 { return None; }
    let q = team as usize;
    let r_base = NUMBERS_RANGE.load(Ordering::Relaxed);
    let r = if base_code == 2 { let m = apos(&NUMBERS_RANGE_MOVE); if m >= 0 { m as u64 } else { r_base } } else { r_base };
    let r2 = r.wrapping_mul(r);
    let team_force = |t: usize| -> (u128, u128, u32) {
        let (mut hp, mut atk, mut n) = (0u128, 0u128, 0u32);
        for k in 0..5usize {
            let c = rd_u64(rh + 0x1e0 + t * 0x28 + k * 8).unwrap_or(0) as usize;
            if c == 0 { continue; }
            // ★[07-20] 챔프당 프로브 1회(+0x610..+0x660) + raw 4읽기 (구 4 VEH콜)
            if !probe_ok(c + 0x610, 0x50) { continue; }
            let chp = raw_u64(c + 0x658);
            if chp == 0 { continue; }
            let cx = raw_u64(c + 0x648); let cy = raw_u64(c + 0x650);
            let dx = if cx >= sx { cx - sx } else { sx - cx };
            let dy = if cy >= sy { cy - sy } else { sy - cy };
            if dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy)) >= r2 { continue; }
            hp += chp as u128;
            atk += raw_i64(c + 0x610).max(0) as u128;
            n += 1;
        }
        (hp, atk, n)
    };
    // ★[07-20 최적화] 적 먼저 + en==0 조기반환: 근처 적이 없으면 f_enemy=0*0=0 → w=9999로 어차피 동일하고,
    //   an(아군수)은 이 경우 유일 소비처인 게이트③(margin>0 && enemy-ally>=margin)에서 enemy=0이라
    //   an이 무엇이든 불발 → (9999,0,0) 반환=호출부 관측상 값 동일. 아군 합산+포탑 기여(최대 ~44읽기) 통째 생략.
    //   근처에 적 없는 판단(정글·귀환·후방 라인워크)이 다수라 실효 큼.
    let (ehp, eatk, en) = team_force(1 - q);
    if en == 0 { return Some((9999, 0, 0)); }
    let (ahp, aatk, an) = team_force(q);
    let (thp, tatk) = ally_tower_contrib(rh, team, sx, sy, base_code);
    let (ahp, aatk) = (ahp.wrapping_add(thp), aatk.wrapping_add(tatk));
    let f_ally = ahp.wrapping_mul(aatk);
    let f_enemy = ehp.wrapping_mul(eatk);
    let w: i64 = if f_enemy == 0 { 9999 } else if f_ally == 0 { 0 }
                 else { (f_ally.wrapping_mul(100) / f_enemy).min(9999) as i64 };
    Some((w, an, en))
}
#[inline] fn stat_hash(a: u64, b: u64) -> u64 {
    let mut x = a.wrapping_mul(0x9e3779b97f4a7c15).wrapping_add(b.wrapping_mul(0xbf58476d1ce4e5b9));
    x = (x ^ (x >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94d049bb133111eb);
    x ^ (x >> 31)
}
unsafe fn stat_modifiers(p5: usize, sim: usize) -> (i64, i64) {
    let k = STAT_INFLUENCE.load(Ordering::Relaxed);
    if k <= 0 || !ptr_ok(p5) { return (0, 0); }
    // ★[07-20] 프로브 1회(+0x218..+0x240) + raw 3읽기. 프로브 실패=(0,0) ≡ 구 fault폴백(50/50/100→adj0·amp0) 값동일.
    if !probe_ok(p5 + 0x218, 0x28) { return (0, 0); }
    let aggr = raw_i64(p5 + 0x230).clamp(0, 100);
    let ego = raw_i64(p5 + 0x238).clamp(0, 100);
    let judg = raw_i64(p5 + 0x218).clamp(0, 200);
    let neutral = STAT_NEUTRAL.load(Ordering::Relaxed);
    let pos_div = STAT_POS_DIV.load(Ordering::Relaxed).max(1);
    let ca = { let d = aggr - neutral; if d > 0 { d / pos_div } else { d } };
    let ce = { let d = ego - neutral; if d > 0 { d / pos_div } else { d } };
    let stat_adj = (ca + ce) * k / 100;
    let amp = ((STAT_JUDG_REF.load(Ordering::Relaxed) - judg).max(0)) * k / 100;
    let jnoise = if amp > 0 {
        let tick = rd_i64(sim + 0xeac0).unwrap_or(0) as u64;
        let handle = rd_u64(p5 + 0x818).unwrap_or(0);   // ★0.5.1 핸들(원본의 +0x6a0=0.4.x stale 정정)
        (stat_hash(tick >> (STAT_NOISE_SHIFT.load(Ordering::Relaxed) as u32), handle) % (2 * amp as u64 + 1)) as i64 - amp
    } else { 0 };
    (stat_adj, jnoise)
}
// ════════════════ [D] 프론티어 최소재현 = 철회(2026-07-19, 인게임 버그로 폐기) ════════════════
// ⛔**재시도 금지**: dd_frontier_mult를 "커버블록 앞 프론티어 게이트만 재현 → bail이면 출력=2 덮어쓰기"로
//   구현했다가 **경기 시작 후 ~5초간 전 챔피언 정지** 버그 발생(유저 실측 07-19). 즉시 철회.
// 【오판의 정체】 단조성 논거(mult↑ → l15x30↑ → prog↓ → bail↑)는 "언제 발동하나"에 대해선 옳았으나,
//   "발동하면 무엇이 출력되나"를 상수 2로 단정한 것이 **틀렸다**. 원본 my_dd7700_full(L3314~3378) 실측:
//     frontier_bail = 커버블록을 **건너뛰는 플래그**일 뿐 → 그대로 **MAIN BODY(L3383~)로 폴스루** →
//     메인이 웨이포인트 순회로 자기 코드+aux를 새로 계산해 출력한다. 즉 bail의 출력 ≠ 상수 2.
//   (my_dd7700_code의 `return 2`는 code-only 변형에서 "메인으로 간다"를 뜻하는 표식이었음 — 이걸 최종
//    출력코드로 오독한 것이 근본 원인.)
// 【증상 메커니즘】 게임이 다른 경로로 채운 aux 위에 코드만 2로 덮음 → 명령 불일치 → 챔피언 무행동.
//   경기 초반 u19가 작아 prog=0<=s20 → bail 연발 → 정지. u19가 진행되면 bail 멎어 ~5초 후 정상화.
// 【제대로 하려면】 MAIN BODY 전체 재현이 필요 = 애초에 회피하려던 대형 재현. 비용 대비 가치 없음 →
//   dd_frontier_mult는 lane_margin·ec_count_radius와 같은 **개입 불가** 항목으로 확정(게임 원본 30 유지).
static SP_SEEN: [AtomicU64; 18] = [const { AtomicU64::new(0) }; 18];
unsafe fn laner_should_retreat(p6: usize, team: i64, selfe: usize, p5: usize, base_code: u8, disc: i64) -> bool {
    if team < 0 || team > 1 || !ptr_ok(selfe) { return false; }
    let l80 = match rd_u64(p6) { Some(v) if ptr_ok(v as usize) => v as usize, _ => return false };
    let sx = rd_u64(selfe + 0x648).unwrap_or(0); let sy = rd_u64(selfe + 0x650).unwrap_or(0);
    let et = (1 - team) as usize;
    let threat = TOWER_THREAT.load(Ordering::Relaxed);
    let nthreat = {
        let sp = if (0..18).contains(&disc) { NUMBERS_THREAT_SP[disc as usize].load(Ordering::Relaxed) } else { -1 };
        if sp >= 0 { sp }
        else if base_code == 2 { let m = NUMBERS_THREAT_MOVE.load(Ordering::Relaxed); if m >= 0 { m } else { NUMBERS_THREAT.load(Ordering::Relaxed) } }
        else { NUMBERS_THREAT.load(Ordering::Relaxed) }
    };
    let margin = NUMBERS_MARGIN.load(Ordering::Relaxed);
    let (w, ally, enemy) = combat_balance(l80, team, sx, sy, base_code).unwrap_or((9999, 1, 0));
    let (stat_adj, jnoise) = stat_modifiers(p5, rd_u64(l80).unwrap_or(0) as usize);
    // ① 포탑 회피: 적포탑 사거리내 AND tower_threat ≥ 전력승산 → 후퇴
    // ★[07-20 최적화] 조건 순서 교정: 임계비교(순수 산술)를 먼저, 포탑 스캔(최대 6+32 엔티티)은 통과 시에만.
    //   AND의 교환 — 스캔에 부수효과 없음 → 결과 비트동일. 이기는 싸움/평시(threat<w) 프레임에서 스캔 통째 생략.
    if threat > 0 && (threat - stat_adj + jnoise) >= w {
        let trange = TOWER_RANGE.load(Ordering::Relaxed); let tr2 = trange.wrapping_mul(trange);
        for &off in &[0x180usize, 0x190, 0x1a0, 0x1b0, 0x1c0, 0x1d0] {
            if tower_in_range(rd_u64(l80 + off + et * 8).unwrap_or(0) as usize, sx, sy, tr2) { return true; }
        }
        let vbase = rd_u64(l80 + 0x130 + et * 0x20).unwrap_or(0) as usize;
        let vlen = rd_u64(l80 + 0x148 + et * 0x20).unwrap_or(0);
        if ptr_ok(vbase) && vlen <= 32 {
            for i in 0..vlen as usize { if tower_in_range(rd_u64(vbase + i * 8).unwrap_or(0) as usize, sx, sy, tr2) { return true; } }
        }
    }
    // ② 전력(force): numbers_threat ≥ 전력승산 (근처 적 ≥ min_enemy일 때만)
    let min_e = { let m = NUMBERS_MIN_ENEMY_MOVE.load(Ordering::Relaxed); if base_code == 2 && m >= 0 { m.max(1) } else { NUMBERS_MIN_ENEMY.load(Ordering::Relaxed) } };
    if nthreat > 0 && (enemy as i64) >= min_e && (nthreat - stat_adj + jnoise) >= w { return true; }
    // ③ 단순 머릿수(binary)
    if margin > 0 && (enemy as i64 - ally as i64) >= margin { return true; }
    false
}
// stage2 게이트 (원본 mp_stage2_ok — 비정규 sim(튜토리얼 등)서 override OFF)
#[inline] unsafe fn vt30_kind(gvt: usize) -> i64 {
    let b = exe_base();
    if b == 0 { return 0; }
    match gvt.wrapping_sub(b) {
        0x37d9ee0 | 0x386b080 => 0,
        0x37da190 | 0x386ae10 => 1,
        0x37da400 | 0x386aba0 => 2,
        _ => 0,
    }
}
#[inline] unsafe fn mp_stage2_ok(p6: usize) -> bool {
    if !ptr_ok(p6) { return false; }
    let g0 = rd_u64(p6).unwrap_or(0) as usize;
    if !ptr_ok(g0) { return false; }
    let gvt = rd_u64(g0 + 8).unwrap_or(0) as usize;
    if !ptr_ok(gvt) { return false; }
    vt30_kind(gvt) == 2
}
// ★override 대상 disc (원본 모드가 numbers 후퇴를 배선했던 disc와 동일 집합 — 5/6/7/15는 원본서 passthrough 기본이라 제외)
const MP_DISC_MASK: u64 = (1 << 0) | (1 << 1) | (1 << 2) | (1 << 3) | (1 << 4) | (1 << 8) | (1 << 9) | (1 << 10)
    | (1 << 11) | (1 << 12) | (1 << 13) | (1 << 14) | (1 << 16) | (1 << 17);
static MP_TRAMP: AtomicUsize = AtomicUsize::new(0);
static MP_OVR_N: AtomicU64 = AtomicU64::new(0);
type MpOrigFn = unsafe extern "C" fn(usize, usize, usize, usize, usize, usize, usize, usize) -> usize;
// movepri 디스패처 wrap: 원본을 그대로 실행(트램폴린 경유) → 출력 코드에 numbers/tower/stat 후퇴만 덧칠.
unsafe extern "C" fn mp_wrap(p1: usize, p2: usize, p3: usize, p4: usize, p5: usize, p6: usize, p7: usize, p8: usize) -> usize {
    let tramp = MP_TRAMP.load(Ordering::Relaxed);
    if tramp == 0 { return p1; }   // 방어(설치 실패 시 도달 불가)
    let orig: MpOrigFn = core::mem::transmute(tramp);
    let ret = orig(p1, p2, p3, p4, p5, p6, p7, p8);   // ★게임 원본 판단 실행 (RNG 포함 전부 게임 소관)
    // ── 이하 read-only + 출력 1워드 override — 실패는 전부 조용히 무시(원본 출력 유지) ──
    if READY_TICKS.load(Ordering::Relaxed) < READY_MIN { return ret; }
    if !MP_OVERRIDE.load(Ordering::Relaxed) { return ret; }
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        if !ptr_ok(p1) || !ptr_ok(p2) { return; }
        let disc = match rd_u64(p2) { Some(d) => d, None => return };
        if disc >= 18 || (MP_DISC_MASK >> disc) & 1 == 0 { return; }
        if !mp_stage2_ok(p6) { return; }
        // ── [B] numbers/tower/stat 후퇴 override
        if TOWER_THREAT.load(Ordering::Relaxed) <= 0 && NUMBERS_THREAT.load(Ordering::Relaxed) <= 0
            && NUMBERS_THREAT_MOVE.load(Ordering::Relaxed) < 0 && NUMBERS_MARGIN.load(Ordering::Relaxed) <= 0 { return; }
        let base_code = rd_u8(p1);
        let l80 = rd_u64(p6).unwrap_or(0) as usize;
        if !ptr_ok(l80) { return; }
        let sim = rd_u64(l80).unwrap_or(0) as usize;
        let side = rd_i64(p5 + 0x820).unwrap_or(-1);            // ★0.5.1 오프셋
        if side != 0 && side != 1 { return; }                   // ★[07-20] laner가 어차피 거부 — SlotMap chase(~5콜) 전에 컷
        let selfe = dd7_slot128(sim, rd_u64(p5 + 0x818).unwrap_or(0));
        if ptr_ok(selfe) && laner_should_retreat(p6, side, selfe, p5, base_code, disc as i64) {
            std::ptr::write_unaligned(p1 as *mut u64, 7u64);    // 후퇴 코드 (원본 배선과 동일한 단일 u64 write)
            MP_OVR_N.fetch_add(1, Ordering::Relaxed);
            if (disc as usize) < 18 { SP_SEEN[disc as usize].fetch_add(1, Ordering::Relaxed); }
        }
    }));
    ret
}

// ════════════════ 훅 설치기 (원본 install 계열 포팅) ════════════════
// 원본실행형 wrap: fn 프롤로그를 jmp cap으로 교체, 트램폴린(원본 프롤로그+jmp fn+len) 반환 → cap이 원본 호출.
unsafe fn install_wrap_generic(rva: usize, orig_len: usize, cap_fn: usize) -> Result<usize, &'static str> {
    let mbase = exe_base();
    if mbase == 0 { return Err("module 0"); }
    let fn_addr = mbase + rva;
    if !probe_ok(fn_addr, orig_len + 4) { return Err("fn unreadable"); }
    let b0 = rd_u8(fn_addr); let b1 = rd_u8(fn_addr + 1);
    if b0 == 0x48 && b1 == 0xb8 { return Err("이미 후킹됨(타 모드/구 ai_adjust?) — 동시활성 금지"); }
    if b0 == 0xe9 || b0 == 0xff { return Err("이미 후킹됨(jmp)"); }
    const MEM_CR: u32 = 0x1000 | 0x2000; const RWX: u32 = 0x40;
    let stub = VirtualAlloc(0, 128, MEM_CR, RWX);
    if stub == 0 { return Err("VirtualAlloc"); }
    let ret_addr = fn_addr + orig_len;
    let mut s: Vec<u8> = Vec::new();
    let mut orig = vec![0u8; orig_len];
    core::ptr::copy_nonoverlapping(fn_addr as *const u8, orig.as_mut_ptr(), orig_len);
    s.extend_from_slice(&orig);
    s.extend_from_slice(&[0x48, 0xb8]); s.extend_from_slice(&ret_addr.to_le_bytes()); s.extend_from_slice(&[0xff, 0xe0]);
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    let mut patch = vec![0x90u8; orig_len];
    patch[0] = 0x48; patch[1] = 0xb8; patch[2..10].copy_from_slice(&cap_fn.to_le_bytes()); patch[10] = 0xff; patch[11] = 0xe0;
    let mut old: u32 = 0;
    if VirtualProtect(fn_addr, orig_len, RWX, &mut old) == 0 { return Err("VirtualProtect"); }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), fn_addr as *mut u8, orig_len);
    VirtualProtect(fn_addr, orig_len, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), fn_addr, orig_len);
    Ok(stub)
}
// replace-rax형: cap(saved, entry_rsp)->i64. RAX_SENT=passthrough / 그외=반환값으로 caller 복귀(원본 skip).
unsafe fn install_replace_detour_rax(rva: usize, orig_len: usize, cap_fn: usize) -> Result<(), &'static str> {
    let mbase = exe_base();
    if mbase == 0 { return Err("module 0"); }
    let fn_addr = mbase + rva;
    if !probe_ok(fn_addr, orig_len + 4) { return Err("fn unreadable"); }
    let b0 = rd_u8(fn_addr); let b1 = rd_u8(fn_addr + 1);
    if (b0 == 0x48 && b1 == 0xb8) || b0 == 0xe9 { return Err("이미 후킹됨 — 동시활성 금지"); }
    const MEM_CR: u32 = 0x1000 | 0x2000; const RWX: u32 = 0x40;
    let stub = VirtualAlloc(0, 256, MEM_CR, RWX);
    if stub == 0 { return Err("VirtualAlloc"); }
    let ret_addr = fn_addr + orig_len;
    let mut s: Vec<u8> = Vec::new();
    s.extend_from_slice(&[0x49, 0x89, 0xe2]);
    s.extend_from_slice(&[0x51, 0x52, 0x41, 0x50, 0x41, 0x51, 0x41, 0x52, 0x41, 0x53]);
    s.extend_from_slice(&[0x48, 0x89, 0xe1]);
    s.extend_from_slice(&[0x4c, 0x89, 0xd2]);
    s.extend_from_slice(&[0x48, 0x83, 0xec, 0x28]);
    s.extend_from_slice(&[0x48, 0xb8]); s.extend_from_slice(&cap_fn.to_le_bytes());
    s.extend_from_slice(&[0xff, 0xd0]);
    s.extend_from_slice(&[0x48, 0x83, 0xc4, 0x28]);
    s.extend_from_slice(&[0x49, 0xbb]); s.extend_from_slice(&(RAX_SENT as u64).to_le_bytes());
    s.extend_from_slice(&[0x4c, 0x39, 0xd8]);
    s.extend_from_slice(&[0x74, 0x0b]);
    s.extend_from_slice(&[0x41, 0x5b, 0x41, 0x5a, 0x41, 0x59, 0x41, 0x58, 0x5a, 0x59]);
    s.extend_from_slice(&[0xc3]);
    s.extend_from_slice(&[0x41, 0x5b, 0x41, 0x5a, 0x41, 0x59, 0x41, 0x58, 0x5a, 0x59]);
    let mut orig = vec![0u8; orig_len];
    core::ptr::copy_nonoverlapping(fn_addr as *const u8, orig.as_mut_ptr(), orig_len);
    s.extend_from_slice(&orig);
    s.extend_from_slice(&[0x48, 0xb8]); s.extend_from_slice(&ret_addr.to_le_bytes());
    s.extend_from_slice(&[0xff, 0xe0]);
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    let mut patch = vec![0x90u8; orig_len];
    patch[0] = 0x48; patch[1] = 0xb8; patch[2..10].copy_from_slice(&stub.to_le_bytes()); patch[10] = 0xff; patch[11] = 0xe0;
    let mut old: u32 = 0;
    if VirtualProtect(fn_addr, orig_len, RWX, &mut old) == 0 { return Err("VirtualProtect"); }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), fn_addr as *mut u8, orig_len);
    VirtualProtect(fn_addr, orig_len, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), fn_addr, orig_len);
    Ok(())
}

// ════════════════ init / 등록 ════════════════
struct CfgExt;
impl ModExtension for CfgExt {
    fn post_update(&self, _s: &mut Scene, _u: &mut GameUI, _a: &mut Assets, _dt: f32) {
        READY_TICKS.fetch_add(1, Ordering::Relaxed);
        load_cfg(false);   // [B]/[C] knob 핫리로드(30프레임당 1 stat). [A] byte-patch는 재시작 필요.
    }
}
fn init(_ctx: &GameCtx) -> ModRegistration {
    unsafe {
        seh_install();
        crash_install();
        panic_hook_install();
        let simd_ok = chacha_simd_selftest();
        USE_SIMD_CHACHA.store(simd_ok, Ordering::Relaxed);
        load_cfg(true);
        // [A] byte-patch — 로드시점 1회(sim 실행 전 = .text 쓰기 안전; 원본 d19 타이밍 준수)
        apply_disc19_imm();
        apply_objective_imm();
        apply_vis_imm();
        apply_dd_imm();   // ★Phase 2: 라인전 LineAttack 5사이트
        apply_ec_imm();   // ★Phase 2: EpicCheck 2사이트
        // [B] movepri 원본실행 wrap
        let mp = match install_wrap_generic(RVA_MOVEPRI, ORIG_LEN_MOVEPRI, mp_wrap as *const () as usize) {
            Ok(stub) => { MP_TRAMP.store(stub, Ordering::Relaxed); "OK".to_string() }
            Err(e) => format!("FAIL: {}", e),
        };
        // [C] fc59a0 recall replace-rax
        let rc = match install_replace_detour_rax(RVA_FC59A0, ORIG_LEN_FC59A0, fc59a0_cap as *const () as usize) {
            Ok(()) => "OK".to_string(),
            Err(e) => format!("FAIL: {}", e),
        };
        if let Some(p) = pth("aiadj2_status.txt") {
            let _ = fs::write(p, format!(
                "=== tfm2_ai_adjust_2 INIT ({}) ===\nchacha_simd={}\nmovepri wrap(0x{:x},{}B) = {}\nfc59a0 repl(0x{:x},{}B) = {}\n(byte-patch 결과 = d19_imm/obj_imm/vis_imm.txt)\n",
                now_ms(), simd_ok, RVA_MOVEPRI, ORIG_LEN_MOVEPRI, mp, RVA_FC59A0, ORIG_LEN_FC59A0, rc));
        }
    }
    let mut reg = ModRegistration::new(MOD_ID);
    reg.set_extension(CfgExt);
    reg
}
declare_mod!(init);
