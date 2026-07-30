// sim_probe.rs — run_tick_ext 계수 probe (2026-07-06 rev2, 트램폴린 버전)
// 목적: 일정넘김 백그라운드 sim의 실제 병렬도·처리량·동시매치수를 실측.
//   앵커 = run_tick_ext(FUN_141e2f2a0 @ 0x1e2f2a0) — sim 1틱, 초당 5만~11만회 발화(실측).
//   구 sim_probe SCAN 훅으로 12B 트램폴린 안전 검증됨(8-push 프롤로그, chkstk e8 +17이라 copy_len=12 안전).
//   진입 rdx(param_2) = 매치별 상태(SimState/ServerState) 포인터 → distinct = 동시/총 매치.
//
// ★1차 시도(int3 on 0x100c870/0x100c9d0 job body)는 트랩 0 = 그 함수는 백그라운드 sim 경로 아님
//   (실제 sim job은 런타임 HeapJob 박싱 vtable 뒤). → run_tick_ext(발화 실측 앵커)로 전환.
//
// 측정 지표(1초마다): 틱/초 · distinct 스레드(=사용 코어) · distinct SS(=동시 매치).
//   distinct 스레드≈distinct SS≈5 → 작업기아(매치 부족). distinct SS≫스레드 → 청크로 직렬화.
// 조작: F3 = 카운터 리셋(일정넘김 직전 0점) / F4 = 누적 요약 덤프.  하트비트 3초.
// 리포트: mods\sim_probe\simpar.txt.  빌드: sdk_0414_new\mod-sdk\build_mod.bat.
// (백업: sim_probe_R16seam_backup_20260706.rs)

use mod_api::*;
use std::path::PathBuf;
use std::fs;
use std::io::Write;
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

const MOD_ID: &str = "sim_probe";
const SCAN_RVA: usize = 0x1e2f2a0;      // run_tick_ext, 8-push 프롤로그, copy_len=12 안전
const OFF_RDX: usize = 0x20;            // saved_ptr+0x20 = rdx = param_2 (매치 상태 SS)

type BOOL = i32; type DWORD = u32; type HMODULE = usize;
extern "system" {
    fn GetModuleHandleW(name: *const u16) -> usize;
    fn GetModuleHandleExW(flags: u32, addr: *const u16, h: *mut HMODULE) -> BOOL;
    fn GetModuleFileNameW(h: HMODULE, buf: *mut u16, sz: DWORD) -> DWORD;
    fn VirtualAlloc(addr: usize, sz: usize, typ: u32, prot: u32) -> usize;
    fn VirtualProtect(addr: usize, size: usize, new_protect: u32, old_protect: *mut u32) -> BOOL;
    fn VirtualQuery(addr: *const core::ffi::c_void, buf: *mut MemBasicInfo, len: usize) -> usize;
    fn FlushInstructionCache(proc: usize, addr: usize, sz: usize) -> BOOL;
    fn GetCurrentProcess() -> usize;
    fn GetCurrentThreadId() -> u32;
}
#[link(name = "user32")]
extern "system" { fn GetAsyncKeyState(vkey: i32) -> i16; }
const VK_F3: i32 = 0x72;
const VK_F4: i32 = 0x73;

// ── 계수 상태 (lockless) ──
static TICKS: AtomicU64 = AtomicU64::new(0);          // 누적 틱(전 스레드 합)
static HOOK_OK: AtomicBool = AtomicBool::new(false);
// per-window(1초) 표본 집합: 샘플링 1/16로 삽입, 리포트마다 리셋
static W_THREADS: [AtomicU64; 64] = [const { AtomicU64::new(0) }; 64];   // distinct 스레드(코어)
static W_SS: [AtomicU64; 512] = [const { AtomicU64::new(0) }; 512];       // distinct SS(동시 매치)
// 누적(리셋=F3): 총 distinct SS 관찰(대략 총 매치)
static T_SS: [AtomicU64; 4096] = [const { AtomicU64::new(0) }; 4096];

#[inline]
fn set_insert(set: &[AtomicU64], v: u64) {
    let n = set.len();
    let mut i = 0;
    while i < n {
        let cur = set[i].load(Ordering::Relaxed);
        if cur == v { return; }
        if cur == 0 && set[i].compare_exchange(0, v, Ordering::Relaxed, Ordering::Relaxed).is_ok() { return; }
        i += 1;
    }
}
fn set_count(set: &[AtomicU64]) -> usize { set.iter().filter(|a| a.load(Ordering::Relaxed) != 0).count() }
fn set_clear(set: &[AtomicU64]) { for a in set.iter() { a.store(0, Ordering::Relaxed); } }

// ── 트램폴린 capture: 틱 계수 + 1/16 표본으로 스레드/SS 기록. atomic/스택read만(패닉코드 금지). ──
unsafe extern "C" fn tick_capture(saved_ptr: usize, _entry_rsp: usize) {
    let n = TICKS.fetch_add(1, Ordering::Relaxed);
    if n & 0xF != 0 { return; }                          // 1/16 샘플(오버헤드 억제)
    let tid = GetCurrentThreadId() as u64;
    set_insert(&W_THREADS, tid);
    let ss = *((saved_ptr + OFF_RDX) as *const u64);     // 스텁 스택(항상 유효), 값=SS 포인터(역참조X)
    if ss > 0x10000 { set_insert(&W_SS, ss); set_insert(&T_SS, ss); }
}

#[repr(C)] #[derive(Default)]
struct MemBasicInfo { base: usize, alloc_base: usize, alloc_protect: u32, _p0: u32,
    region_size: usize, state: u32, protect: u32, typ: u32, _p1: u32 }
unsafe fn readable(addr: usize, len: usize) -> bool {
    if addr < 0x10000 || len == 0 { return false; }
    let mut mbi = MemBasicInfo::default();
    if VirtualQuery(addr as *const _, &mut mbi, core::mem::size_of::<MemBasicInfo>()) == 0 { return false; }
    const COMMIT: u32 = 0x1000; const RD: u32 = 0x02|0x04|0x20|0x40; const GUARD: u32 = 0x01|0x100;
    if mbi.state != COMMIT || mbi.protect & GUARD != 0 || mbi.protect & RD == 0 { return false; }
    addr + len <= mbi.base + mbi.region_size
}

fn now_ms() -> u64 { SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0) }
fn mod_dir() -> Option<PathBuf> {
    unsafe {
        let addr = mod_dir as *const () as usize;
        let mut h: HMODULE = 0;
        if GetModuleHandleExW(0x4|0x2, addr as *const u16, &mut h) == 0 || h == 0 { return None; }
        let mut buf = [0u16; 4096];
        let len = GetModuleFileNameW(h, buf.as_mut_ptr(), buf.len() as DWORD);
        if len == 0 { return None; }
        let s = String::from_utf16_lossy(&buf[..len as usize]);
        let mut p = PathBuf::from(s); p.pop(); Some(p)
    }
}
fn report(line: &str) {
    if let Some(mut p) = mod_dir() {
        p.push("simpar.txt");
        if let Ok(mut f) = fs::OpenOptions::new().create(true).append(true).open(&p) { let _ = write!(f, "{}", line); }
    }
}

// ── 12B 트램폴린 설치 (구 SCAN 훅 검증코드; 8-push copy_len=12, reloc 불필요) ──
unsafe fn install_scan_hook() -> Result<(), &'static str> {
    let mbase = GetModuleHandleW(core::ptr::null());
    if mbase == 0 { return Err("module base 0"); }
    let fn_addr = mbase + SCAN_RVA;
    if !readable(fn_addr, 16) { return Err("fn not readable"); }
    // 프롤로그 검증: 55 41 57 41 56 41 55 41 54 56 57 53 (push rbp/r15/r14/r13/r12/rsi/rdi/rbx)
    let want: [u8; 12] = [0x55,0x41,0x57,0x41,0x56,0x41,0x55,0x41,0x54,0x56,0x57,0x53];
    for i in 0..12 { if *((fn_addr+i) as *const u8) != want[i] { return Err("프롤로그 불일치(8-push 아님)"); } }
    let mut orig = [0u8; 12];
    for i in 0..12 { orig[i] = *((fn_addr+i) as *const u8); }
    const MEM_CR: u32 = 0x1000|0x2000; const RWX: u32 = 0x40;
    let stub = VirtualAlloc(0, 256, MEM_CR, RWX);
    if stub == 0 { return Err("VirtualAlloc"); }
    let ret_addr = fn_addr + 12;
    let mut s: Vec<u8> = Vec::new();
    s.extend_from_slice(&[0x49,0x89,0xe2]);                          // mov r10, rsp
    s.extend_from_slice(&[0x51,0x52]);                              // push rcx; push rdx
    s.extend_from_slice(&[0x41,0x50,0x41,0x51,0x41,0x52,0x41,0x53]); // push r8 r9 r10 r11
    s.extend_from_slice(&[0x48,0x89,0xe1]);                         // mov rcx, rsp (saved_ptr)
    s.extend_from_slice(&[0x4c,0x89,0xd2]);                         // mov rdx, r10 (entry_rsp)
    s.extend_from_slice(&[0x48,0x83,0xec,0x28]);                    // sub rsp,0x28
    s.extend_from_slice(&[0x48,0xb8]); s.extend_from_slice(&(tick_capture as *const () as usize).to_le_bytes());
    s.extend_from_slice(&[0xff,0xd0]);                              // call rax
    s.extend_from_slice(&[0x48,0x83,0xc4,0x28]);                    // add rsp,0x28
    s.extend_from_slice(&[0x41,0x5b,0x41,0x5a,0x41,0x59,0x41,0x58]); // pop r11 r10 r9 r8
    s.extend_from_slice(&[0x5a,0x59]);                             // pop rdx; pop rcx
    s.extend_from_slice(&orig);                                     // 원본 12B
    s.extend_from_slice(&[0x48,0xb8]); s.extend_from_slice(&ret_addr.to_le_bytes()); // mov rax, fn+12
    s.extend_from_slice(&[0xff,0xe0]);                              // jmp rax
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    let mut patch = [0u8; 12];
    patch[0]=0x48; patch[1]=0xb8; patch[2..10].copy_from_slice(&stub.to_le_bytes()); patch[10]=0xff; patch[11]=0xe0;
    let mut old: u32 = 0;
    if VirtualProtect(fn_addr, 12, RWX, &mut old) == 0 { return Err("VirtualProtect"); }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), fn_addr as *mut u8, 12);
    VirtualProtect(fn_addr, 12, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), fn_addr, 12);
    Ok(())
}

fn key_down(vk: i32) -> bool { (unsafe { GetAsyncKeyState(vk) } as u16 & 0x8000) != 0 }

fn spawn_worker() {
    std::thread::spawn(|| {
        report(&format!("[{}ms] [worker] 리포트 스레드 시작 (F3=리셋 / F4=요약). 1초마다 틱/스레드/SS.\n", now_ms()));
        let mut prev_f3 = false; let mut prev_f4 = false;
        let mut last = now_ms(); let mut last_hb = now_ms(); let mut last_ticks = 0u64;
        loop {
            std::thread::sleep(std::time::Duration::from_millis(100));
            let f3 = key_down(VK_F3);
            if f3 && !prev_f3 {
                TICKS.store(0, Ordering::Relaxed); last_ticks = 0;
                set_clear(&T_SS); set_clear(&W_SS); set_clear(&W_THREADS);
                report(&format!("[{}ms] ===== F3 리셋 (누적 0점). 이제 일정을 넘기세요. =====\n", now_ms()));
            }
            prev_f3 = f3;
            let f4 = key_down(VK_F4);
            if f4 && !prev_f4 {
                report(&format!("[{}ms] ===== F4 요약: 누적 틱={} | 총 distinct SS(≈총 매치)={} | hook_ok={} =====\n",
                    now_ms(), TICKS.load(Ordering::Relaxed), set_count(&T_SS), HOOK_OK.load(Ordering::Relaxed)));
            }
            prev_f4 = f4;
            let now = now_ms();
            if now.saturating_sub(last_hb) >= 3000 {
                report(&format!("[{}ms] [hb] 누적틱={} 총SS={} hook_ok={}\n",
                    now, TICKS.load(Ordering::Relaxed), set_count(&T_SS), HOOK_OK.load(Ordering::Relaxed)));
                last_hb = now;
            }
            if now.saturating_sub(last) >= 1000 {
                let t = TICKS.load(Ordering::Relaxed);
                let dt = t.saturating_sub(last_ticks);
                let secs = (now.saturating_sub(last) as f64)/1000.0;
                let thr = set_count(&W_THREADS);
                let ss = set_count(&W_SS);
                if dt > 0 {
                    report(&format!("[{}ms] 틱/s={:.0} | 동시 스레드(코어)={} | 동시 SS(매치)={} | 누적틱={} 총SS={}\n",
                        now, if secs>0.0 {dt as f64/secs} else {0.0}, thr, ss, t, set_count(&T_SS)));
                }
                // 윈도우 집합 리셋(다음 1초 표본)
                set_clear(&W_THREADS); set_clear(&W_SS);
                last = now; last_ticks = t;
            }
        }
    });
}

fn init(_ctx: &GameCtx) -> ModRegistration {
    report(&format!("\n[{}ms] === sim_probe (run_tick_ext 계수 rev2) INIT ===\n  앵커 0x{:x}. F3=리셋 F4=요약. 하트비트 3초.\n",
        now_ms(), SCAN_RVA));
    unsafe {
        match install_scan_hook() {
            Ok(()) => { HOOK_OK.store(true, Ordering::Relaxed); report("[hook] run_tick_ext(0x1e2f2a0) 트램폴린 설치 성공\n"); }
            Err(e) => report(&format!("[hook] 설치 실패: {}\n", e)),
        }
    }
    spawn_worker();
    ModRegistration::new(MOD_ID)
}
declare_mod!(init);
