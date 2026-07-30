// sim_probe.rs — 매치 sim 드라이버(FUN_14204f810 @ RVA 0x204f810) 진입 캡처
// 목적: 헤드리스 매치 시뮬 드라이버에 detour 걸어 (a)호출자=드라이버 진입점 (b)param 1~5
//   (c)각 param이 가리키는 메모리 를 덤프 → "모드가 뭘 채워 이 함수를 부르면 5v5 헤드리스 sim이
//   도는가"의 입력 계약을 규명. 매치당 1회 발화라 F3 타이밍 불필요 — 자동 로깅(앞 N회).
// 빌드: sdk_0414_new\mod-sdk\build_mod.bat 로 (nightly-2026-05-24). 배포: mods\sim_probe\.
// 로그: mods\sim_probe\sim_probe.txt
// 트램폴린/세이프리드/덤프 = packet_interceptor 검증코드 그대로(8-PUSH 프롤로그 동일).

use mod_api::*;
use std::path::PathBuf;
use std::fs;
use std::io::Write;
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

const MOD_ID: &str = "sim_probe";
const DRIVER_RVA: usize = 0x204f810;                 // FUN_14204f810 매치 sim 드라이버 (8-PUSH 프롤로그)
const PROLOGUE: [u8; 12] = [0x55,0x41,0x57,0x41,0x56,0x41,0x55,0x41,0x54,0x56,0x57,0x53];

// saved_ptr(스텁이 push한 레지스터) 기준 슬롯 오프셋
const OFF_R9:  usize = 0x10;
const OFF_R8:  usize = 0x18;
const OFF_RDX: usize = 0x20;
const OFF_RCX: usize = 0x28;
// entry_rsp 기준 (스택 인자 p5/p6 + [entry_rsp]=반환주소=호출자)
const OFF_P5: usize = 0x28;
const OFF_P6: usize = 0x30;

const HIT_CAP: u64 = 1;         // 전체 엔트리 덤프라 1매치만
const DUMP_LEN: usize = 0x80;   // 각 param 포인터 덤프 바이트수

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
}
#[link(name = "user32")]
extern "system" { fn GetAsyncKeyState(vkey: i32) -> i16; }
const VK_F3: i32 = 0x72;
const VK_F4: i32 = 0x73;

// ★★★R16 seam 안전계측: int3(0xCC)+VEH — 1바이트 원자패치라 핫 async 함수(0x100c870)도 패치레이스 없음
//   (트램폴린 12B 패치의 로드/F4 크래시 회피). VEH는 atomic stash만(format!/lock/alloc 금지) → post_update서 출력.
extern "system" { fn AddVectoredExceptionHandler(first: u32, handler: usize) -> usize; }
static BP_ADDR: AtomicUsize = AtomicUsize::new(0);
static BP_ORIG: AtomicU64 = AtomicU64::new(0);        // 원본 첫바이트 보존(복원용)
static MB_CAPTURED: AtomicBool = AtomicBool::new(false);
static MB_LOGGED: AtomicBool = AtomicBool::new(false);
static VEH_REGISTERED: AtomicBool = AtomicBool::new(false);
static MB_FLD: [AtomicU64; 13] = [const { AtomicU64::new(0) }; 13]; // [0]=self, [1..=12]=self+{0,8,10,18,20,28,30,38,40,48,50,58}

// VEH: #BP@target → Rcx(future-self) + self 필드 12개를 atomic에 stash → 원바이트 복원 → RIP 되돌림 → CONTINUE.
unsafe extern "system" fn seam_veh(info: usize) -> i32 {
    if info == 0 { return 0; }
    let rec = *(info as *const usize);                 // EXCEPTION_RECORD*
    let ctx = *((info + 8) as *const usize);           // CONTEXT*
    if rec == 0 || ctx == 0 { return 0; }
    if *(rec as *const u32) != 0x80000003 { return 0; } // EXCEPTION_BREAKPOINT
    let exaddr = *((rec + 0x10) as *const usize);       // ExceptionAddress (int3 규약: target 또는 target+1)
    let target = BP_ADDR.load(Ordering::Relaxed);
    if target == 0 || (exaddr != target && exaddr != target + 1) { return 0; } // 우리 BP 아님 → CONTINUE_SEARCH
    // ★VEH선 Rcx(self ptr)만 atomic stash — 필드 read는 fault위험(중첩예외 크래시)이라 post_update(안전)로 미룸.
    //   동시 in-flight 트랩 대비 swap guard, BP_ADDR는 안 지움 → 후속 동시트랩도 아래서 드레인.
    if !MB_CAPTURED.swap(true, Ordering::Relaxed) {
        MB_FLD[0].store(*((ctx + 0x80) as *const usize) as u64, Ordering::Relaxed); // Rcx = future-self
    }
    // ★항상: 원본 복원(idempotent 0x55) + RIP 되돌림 → 동시/후속 트랩 전부 안전 드레인(미처리 int3 크래시 방지).
    *(target as *mut u8) = BP_ORIG.load(Ordering::Relaxed) as u8;
    FlushInstructionCache(GetCurrentProcess(), target, 1);
    *((ctx + 0xf8) as *mut usize) = target;             // Rip = target(복원된 원본 재실행)
    -1  // EXCEPTION_CONTINUE_EXECUTION
}

#[repr(C)] #[derive(Default)]
struct MemBasicInfo { base: usize, alloc_base: usize, alloc_protect: u32, _p0: u32,
    region_size: usize, state: u32, protect: u32, typ: u32, _p1: u32 }

unsafe fn region(addr: usize) -> Option<MemBasicInfo> {
    if addr < 0x10000 { return None; }
    let mut mbi = MemBasicInfo::default();
    if VirtualQuery(addr as *const _, &mut mbi, core::mem::size_of::<MemBasicInfo>()) == 0 { return None; }
    Some(mbi)
}
unsafe fn readable(addr: usize, len: usize) -> bool {
    if addr < 0x10000 || len == 0 { return false; }
    let mbi = match region(addr) { Some(m) => m, None => return false };
    const COMMIT: u32 = 0x1000;
    const RD: u32 = 0x02|0x04|0x20|0x40;
    const GUARD: u32 = 0x01|0x100;
    if mbi.state != COMMIT || mbi.protect & GUARD != 0 || mbi.protect & RD == 0 { return false; }
    addr + len <= mbi.base + mbi.region_size
}
unsafe fn rd_u64(a: usize) -> Option<u64> { if readable(a,8){Some(std::ptr::read_unaligned(a as *const u64))}else{None} }
unsafe fn rd_u8(a: usize) -> Option<u8> { if readable(a,1){Some(std::ptr::read_unaligned(a as *const u8))}else{None} }

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
fn append_log(line: &str) {
    if let Some(mut p) = mod_dir() {
        p.push("sim_probe.txt");
        if let Ok(mut f) = fs::OpenOptions::new().create(true).append(true).open(&p) { let _ = write!(f,"{}",line); }
    }
}

// 힙/스택 유효 포인터처럼 보이는 값인가 (역참조 안전성은 readable 로 별도 확인)
fn looks_ptr(v: u64) -> bool { v >= 0x10000 && v < 0x7fff_ffff_ffff }

// 구조체를 덤프하고, 그 안의 8B 슬롯 중 유효 포인터를 1단계 더 따라가 덤프 (팀/챔프 중첩구조 노출)
unsafe fn dump_deep(tag: &str, ptr: usize, len: usize, s: &mut String) {
    dump_mem(tag, ptr, len, s);
    if ptr <= 0x10000 || !readable(ptr, 8) { return; }
    let n = len.min(0x80);
    let mut off = 0;
    while off + 8 <= n {
        if let Some(v) = rd_u64(ptr + off) {
            if looks_ptr(v) && readable(v as usize, 16) {
                let sub = format!("{}+{:02x}->", tag.trim(), off);
                dump_mem(&sub, v as usize, 0x40, s);
            }
        }
        off += 8;
    }
}

// 매치 엔트리: 본문 len 덤프 + 앞 scan 바이트 내 유효 포인터를 1단계 추적해 각 0x40 덤프.
unsafe fn dump_entry(tag: &str, ptr: usize, len: usize, scan: usize, s: &mut String) {
    dump_mem(tag, ptr, len, s);
    if ptr <= 0x10000 || !readable(ptr, 8) { return; }
    let mut off = 0;
    let mut shown = 0;
    while off + 8 <= scan && shown < 64 {
        if let Some(v) = rd_u64(ptr + off) {
            if looks_ptr(v) && readable(v as usize, 16) {
                let sub = format!("{}+{:04x}->", tag.trim(), off);
                dump_mem(&sub, v as usize, 0x40, s);
                shown += 1;
            }
        }
        off += 8;
    }
}

// 포인터가 가리키는 메모리 첫 len 바이트 hex+ascii (16바이트/줄)
unsafe fn dump_mem(tag: &str, ptr: usize, len: usize, s: &mut String) {
    if ptr <= 0x10000 || !readable(ptr, 16) { s.push_str(&format!("     {} = 0x{:x} (역참조 불가)\n", tag, ptr)); return; }
    s.push_str(&format!("     {} = 0x{:x}:\n", tag, ptr));
    let n = len.min(0x100);
    let mut i = 0;
    while i < n {
        let mut hex = String::new(); let mut asc = String::new();
        for j in 0..16 {
            if i+j >= n { break; }
            match rd_u8(ptr+i+j) {
                Some(b) => { hex.push_str(&format!("{:02x} ", b));
                    asc.push(if (0x20..0x7f).contains(&b) { b as char } else { '.' }); }
                None => { hex.push_str("?? "); asc.push('.'); }
            }
        }
        s.push_str(&format!("       +{:03x}  {:<48} |{}|\n", i, hex.trim_end(), asc));
        i += 16;
    }
}

static BASE: AtomicUsize = AtomicUsize::new(0);
static HOOK_INSTALLED: AtomicBool = AtomicBool::new(false);
static IN_RECALL: AtomicBool = AtomicBool::new(false);   // ★재호출 중(재귀 차단)
static RECALL_TESTED: AtomicBool = AtomicBool::new(false); // ★make-or-break 1회만
static RECALL_ARMED: AtomicBool = AtomicBool::new(false);  // ★F4로 ARM해야 재호출 (로드중 오발화 방지)
static PREV_F4: AtomicBool = AtomicBool::new(false);
static HIT_TOTAL: AtomicU64 = AtomicU64::new(0);
static LOGGED: AtomicU64 = AtomicU64::new(0);
static LAST_CALLER: AtomicU64 = AtomicU64::new(0);
static BOOTED: AtomicBool = AtomicBool::new(false);
static PREV_F3: AtomicBool = AtomicBool::new(false);

// ★★ make-or-break: 유효 sim 컨텍스트(driver 진입 순간)에서 드라이버를 단독 재호출.
//   p1/p2/p3/p4 = 우리메모리 복사본(실상태 비오염), p5 = live vtable객체(복사불가, 그대로). p6=-2.
//   크래시 없이 복귀+출력나오면 → 경로B(헤드리스) 가능. AV면 게임크래시(=불가 결론).
unsafe fn do_recall_test(base: usize, p1: u64, p2: u64, p3: u64, p4: u64, p5: u64) {
    let alloc = |sz: usize| VirtualAlloc(0, sz, 0x3000, 0x04);
    let copyr = |src: u64, sz: usize| -> usize {
        let d = alloc(sz);
        if d != 0 && src > 0x10000 && readable(src as usize, sz) {
            core::ptr::copy_nonoverlapping(src as *const u8, d as *mut u8, sz);
        }
        d
    };
    let p1c = copyr(p1, 0x400);
    let p2c = copyr(p2, 0x3a80);
    let p3c = copyr(p3, 0x40);
    let p4c = copyr(p4, 0x400);
    append_log(&format!("[recall] ★단독호출 시도 (valid ctx): p1c={:x} p2c={:x} p3c={:x} p4c={:x} p5(live)={:x}\n", p1c, p2c, p3c, p4c, p5));
    if p1c == 0 || p2c == 0 || p3c == 0 || p4c == 0 { append_log("[recall] alloc 실패 — 중단\n"); return; }
    IN_RECALL.store(true, Ordering::Relaxed);
    type DriverFn = unsafe extern "C" fn(usize, usize, usize, usize, usize, usize) -> usize;
    let f: DriverFn = core::mem::transmute(base + DRIVER_RVA);
    let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        f(p1c, p2c, p3c, p4c, p5 as usize, (-2i64) as usize)
    }));
    IN_RECALL.store(false, Ordering::Relaxed);
    match r {
        Ok(ret) => {
            let mut s = format!("[recall] ★★복귀 성공! ret=0x{:x} — 크래시 없이 단독호출 됨!\n", ret);
            dump_mem("recall p1c(출력ctx)", p1c, 0x100, &mut s);
            dump_entry("recall p2c(엔트리 결과)", p2c, 0x400, 0x200, &mut s);
            append_log(&s);
        }
        Err(_) => append_log("[recall] 패닉(catch). AV였다면 이 로그 전에 게임 크래시.\n"),
    }
}

// ★HashMap insert 0x1941af0 (8-push 프롤로그 = 드라이버처럼 안전, chkstk 회피).
//   rcx = 매치엔트리 HashMap(&mut RawTable) = 게임상태+0xf8b0 → 게임상태 = rcx - 0xf8b0.
//   ARM(F4) 후 첫 호출 1회만 캡처(로드중 차단). SAFE(얕은덤프).
const INS_RVA: usize = 0x1941af0;
static INS_DONE: AtomicBool = AtomicBool::new(false);
unsafe extern "C" fn ins_capture(saved_ptr: usize, entry_rsp: usize) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        if saved_ptr < 0x10000 || entry_rsp < 0x10000 { return; }
        if !RECALL_ARMED.load(Ordering::Relaxed) { return; }
        if INS_DONE.swap(true, Ordering::Relaxed) { return; }
        let base = BASE.load(Ordering::Relaxed);
        let caller = rd_u64(entry_rsp).unwrap_or(0);
        let rcx = rd_u64(saved_ptr + OFF_RCX).unwrap_or(0);  // HashMap(&mut RawTable) = 게임상태+0xf8b0
        let rdx = rd_u64(saved_ptr + OFF_RDX).unwrap_or(0);
        let r8  = rd_u64(saved_ptr + OFF_R8).unwrap_or(0);
        let gs = rcx.wrapping_sub(0xf8b0);
        let ctrl = rd_u64(rcx as usize).unwrap_or(0);
        let items = rd_u64(rcx as usize + 0x18).unwrap_or(0);
        let mut s = format!(
            "\n[★INS 0x1941af0 진입 {}ms] caller=0x{:x}(RVA 0x{:x})\n   rcx(HashMap=게임상태+0xf8b0)=0x{:x} → ★게임상태=0x{:x}\n   rdx=0x{:x} r8=0x{:x}\n   HashMap: ctrl=0x{:x}(readable={}) items={}\n",
            now_ms(), caller, caller.wrapping_sub(base as u64), rcx, gs, rdx, r8,
            ctrl, readable(ctrl as usize, 8), items as i64);
        dump_mem("게임상태 +0..0x40", gs as usize, 0x40, &mut s);
        append_log(&s);
    }));
}

// ── 훅 본체: 드라이버 진입 1회당 호출 ── (catch_unwind 로 패닉이 게임 콜스택으로 unwind 차단)
unsafe extern "C" fn sim_capture(saved_ptr: usize, entry_rsp: usize) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        if saved_ptr < 0x10000 || entry_rsp < 0x10000 { return; }
        if IN_RECALL.load(Ordering::Relaxed) { return; } // ★재호출 중 재진입 = 본문만 실행
        let base = BASE.load(Ordering::Relaxed);
        let caller = rd_u64(entry_rsp).unwrap_or(0);
        let caller_rva = caller.wrapping_sub(base as u64);
        let p1 = rd_u64(saved_ptr+OFF_RCX).unwrap_or(0); // rcx (this/ctx)
        let p2 = rd_u64(saved_ptr+OFF_RDX).unwrap_or(0); // rdx
        let p3 = rd_u64(saved_ptr+OFF_R8).unwrap_or(0);  // r8
        let p4 = rd_u64(saved_ptr+OFF_R9).unwrap_or(0);  // r9
        let p5 = rd_u64(entry_rsp+OFF_P5).unwrap_or(0);  // [rsp+0x28]
        let p6 = rd_u64(entry_rsp+OFF_P6).unwrap_or(0);

        // ★드라이버 직접호출은 AV크래시 확정(make-or-break 실패) → 비활성. 대신 등록함수 0x1e2f1c0 경로.
        const ENABLE_DRIVER_RECALL: bool = false;
        if ENABLE_DRIVER_RECALL && RECALL_ARMED.load(Ordering::Relaxed) && !RECALL_TESTED.swap(true, Ordering::Relaxed) {
            do_recall_test(base, p1, p2, p3, p4, p5);
        }

        let hit = HIT_TOTAL.fetch_add(1, Ordering::Relaxed);
        // caller dedup (같은 호출자 연속 진입 1줄로)
        let prev = LAST_CALLER.swap(caller, Ordering::Relaxed);
        let dedup = prev == caller && hit != 0;

        let logged = LOGGED.load(Ordering::Relaxed);
        if logged >= HIT_CAP { return; }
        if dedup && hit > 1 { return; }
        LOGGED.fetch_add(1, Ordering::Relaxed);

        let mut s = format!(
            "\n[match-sim 진입 #{} (hit {}) {}ms]\n   caller = 0x{:x} (RVA 0x{:x})\n   p1(rcx)=0x{:x} p2(rdx)=0x{:x} p3(r8)=0x{:x} p4(r9)=0x{:x} p5=0x{:x} p6=0x{:x}\n",
            logged, hit, now_ms(), caller, caller_rva, p1, p2, p3, p4, p5, p6);
        // ★★스택 워크(2026-07-04 R17): entry_rsp 위로 스캔해 모듈 .text 리턴주소 후보 수집 = 실제 콜체인.
        //   드라이버(0x204f810)는 안정발화 → 상위 프레임(dispatch 0x2053070→진짜 오케스트레이터→async)을 실측.
        //   정적 오병합/async단절 우회 = "누가 sim을 도나"의 empirical 답.
        {
            let lo = base as usize + 0x1000;
            let hi = base as usize + 0x2979000;   // .text 상한 근사(0.4.14 exe)
            // ★★2026-07-06 R17 스폰추적: 창 0x2800→0x10000(64KB), found 40→120.
            //   run_tick_ext(0x1e2f2a0)가 큰 프레임이라 좁은 창서 소진됨 → 넓혀 thunk 0x1e3af50
            //   → 오케 0x913d30 → client_data 0x55a180/0x55ad30 → poll 0x6413f0/0x641750/0x1413dffc0
            //   → async executor(스폰지점 상류)까지 empirical 포착. 알려진 프레임은 라벨.
            s.push_str("   [★스택 리턴주소 체인 (RVA, 위=콜 상류) — 라벨=알려진 프레임]\n");
            let label = |r: u64| -> &'static str {
                match r {
                    0x204f810 => " <driver>",
                    0x2053070..=0x2053377 => " <dispatch 0x2053070>",
                    0x1e2f2a0..=0x1e362a3 => " <run_tick_ext>",
                    0x1e3af50..=0x1e3b0ff => " <thunk 0x1e3af50>",
                    0x1e39270..=0x1e393ff => " <thunk2 0x1e39270>",
                    0x913d30..=0x913fff  => " <★오케 0x913d30(per-frame sim)>",
                    0x55a180..=0x55a2ff  => " <client_data 0x55a180>",
                    0x55ad30..=0x55aeff  => " <client_data 0x55ad30>",
                    0x6413f0..=0x6415ff  => " <poll 0x6413f0>",
                    0x641750..=0x6418ff  => " <poll 0x641750>",
                    0x13dffc0..=0x13e01ff => " <poll 0x13dffc0>",
                    0xc5ccb0..=0xc5ceff  => " <?0xc5ccb0 별개task>",
                    0x100c870..=0x100caff => " <?0x100c870 별개task>",
                    _ => "",
                }
            };
            let mut off = 0usize; let mut found = 0u32; let mut prev_rva = 0u64;
            while off < 0x10000 && found < 120 {
                if let Some(v) = rd_u64(entry_rsp + off) {
                    let vu = v as usize;
                    if vu >= lo && vu < hi {
                        let rva = (vu - base as usize) as u64;
                        if rva != prev_rva {   // 연속 중복 스킵
                            s.push_str(&format!("      rsp+0x{:05x} = RVA 0x{:x}{}\n", off, rva, label(rva)));
                            found += 1; prev_rva = rva;
                        }
                    }
                }
                off += 8;
            }
            s.push_str(&format!("      (스캔 {}프레임, off末 0x{:x})\n", found, off));
        }
        dump_mem("*p1(rcx,ctx)", p1 as usize, DUMP_LEN, &mut s);
        // p2 = 매치 엔트리(stride 0x3a80=14976B) 내부 뷰. 엔트리 헤더 영역(앞 0x2000=8KB)을 통째로 덤프 +
        //   그 안 포인터(로스터/챔프 배열)를 1단계 추적. 챔프/athlete/규모/seed 오프셋 역산용.
        dump_entry("*p2(rdx,엔트리)", p2 as usize, 0x2000, 0x600, &mut s);
        dump_mem("*p3(r8,RNG)", p3 as usize, 0x20, &mut s);     // RNG 상태 (32B)
        dump_deep("*p4(r9,파라미터)", p4 as usize, 0x100, &mut s); // 매치 파라미터 + 중첩
        dump_mem("*p5     ", p5 as usize, DUMP_LEN, &mut s);
        if logged+1 == HIT_CAP { s.push_str(&format!("[{}ms] (캡 {}건 도달 — 로깅 종료. F3로 리셋 가능)\n", now_ms(), HIT_CAP)); }
        append_log(&s);
    }));
}

// 일반화 트램폴린: copy_len 바이트(위치독립 가정)를 가로채 cap_fn 호출 후 원본실행→jmp fn+copy_len.
//   ⚠ copy_len 영역에 rip-상대 명령(e8/e9/jcc rel) 없어야 함(호출측이 프롤로그 확인 후 선택).
// rel32 reloc 트램폴린용: 스텁을 target(모듈) ±~1.8GB 안에 할당 (rel32 reach 보장).
unsafe fn alloc_near(target: usize, size: usize) -> usize {
    const MEM_CR: u32 = 0x1000|0x2000; const RWX: u32 = 0x40;
    let step = 0x100000usize; // 1MB
    let aligned = target & !0xffff;
    for i in 0..0x600 { // 아래로 ~1.5GB
        let cand = aligned.wrapping_sub(i * step) & !0xffff;
        if cand < 0x10000 { break; }
        let p = VirtualAlloc(cand, size, MEM_CR, RWX);
        if p != 0 { return p; }
    }
    for i in 1..0x600 { // 위로 ~1.5GB
        let cand = (aligned + i * step) & !0xffff;
        let p = VirtualAlloc(cand, size, MEM_CR, RWX);
        if p != 0 { return p; }
    }
    VirtualAlloc(0, size, MEM_CR, RWX) // 폴백
}

unsafe fn install_hook_at(rva: usize, copy_len: usize, cap_fn: usize, reloc_e8: Option<usize>) -> Result<(), &'static str> {
    let mbase = GetModuleHandleW(core::ptr::null());
    if mbase == 0 { return Err("module base 0"); }
    BASE.store(mbase, Ordering::Relaxed);
    let fn_addr = mbase + rva;
    if !readable(fn_addr, copy_len.max(12)) { return Err("fn not readable"); }
    let mut orig = [0u8; 16];
    for i in 0..copy_len { orig[i] = *((fn_addr+i) as *const u8); }

    const MEM_CR: u32 = 0x1000|0x2000; const RWX: u32 = 0x40;
    // rel32 reloc 필요시 모듈 근처에 할당(rel32 reach), 아니면 아무데나.
    let stub = if reloc_e8.is_some() { alloc_near(fn_addr, 256) } else { VirtualAlloc(0, 256, MEM_CR, RWX) };
    if stub == 0 { return Err("VirtualAlloc 실패"); }
    let ret_addr = fn_addr + copy_len;
    let mut s: Vec<u8> = Vec::new();
    s.extend_from_slice(&[0x49,0x89,0xe2]);                          // mov r10, rsp  (= entry_rsp)
    s.extend_from_slice(&[0x51,0x52]);                              // push rcx; push rdx
    s.extend_from_slice(&[0x41,0x50,0x41,0x51,0x41,0x52,0x41,0x53]);// push r8 r9 r10 r11
    s.extend_from_slice(&[0x48,0x89,0xe1]);                         // mov rcx, rsp  (= saved_ptr)
    s.extend_from_slice(&[0x4c,0x89,0xd2]);                         // mov rdx, r10  (= entry_rsp)
    s.extend_from_slice(&[0x48,0x83,0xec,0x28]);                    // sub rsp, 0x28 (shadow+align)
    s.extend_from_slice(&[0x48,0xb8]); s.extend_from_slice(&cap_fn.to_le_bytes()); // mov rax, cap_fn
    s.extend_from_slice(&[0xff,0xd0]);                              // call rax
    s.extend_from_slice(&[0x48,0x83,0xc4,0x28]);                    // add rsp, 0x28
    s.extend_from_slice(&[0x41,0x5b,0x41,0x5a,0x41,0x59,0x41,0x58]);// pop r11 r10 r9 r8
    s.extend_from_slice(&[0x5a,0x59]);                             // pop rdx; pop rcx
    let copied_at = s.len();                                        // 스텁 내 원본바이트 시작 오프셋
    s.extend_from_slice(&orig[..copy_len]);                         // 원본 가로챈 copy_len 바이트
    // ★ rel32 call(e8) 재배치: 원본의 e8 at copy-offset eo → 스텁 위치에 맞게 displacement 보정
    if let Some(eo) = reloc_e8 {
        if orig[eo] == 0xe8 && eo + 5 <= copy_len {
            let orig_rel = i32::from_le_bytes([orig[eo+1],orig[eo+2],orig[eo+3],orig[eo+4]]) as i64;
            let target = (fn_addr + eo + 5) as i64 + orig_rel;            // 원래 call 목적지(__chkstk)
            let stub_e8 = stub + copied_at + eo;                          // 스텁 내 e8 절대주소
            let disp = target - (stub_e8 as i64 + 5);
            append_log(&format!("[reloc] fn=0x{:x} stub=0x{:x} target(chkstk)=0x{:x} disp=0x{:x} fits_i32={}\n",
                fn_addr, stub, target, disp, disp >= i32::MIN as i64 && disp <= i32::MAX as i64));
            if disp < i32::MIN as i64 || disp > i32::MAX as i64 { return Err("reloc out of range (alloc_near 실패)"); }
            let p = copied_at + eo + 1;
            s[p..p+4].copy_from_slice(&(disp as i32).to_le_bytes());
        } else {
            return Err("reloc_e8 위치에 e8 없음");
        }
    }
    s.extend_from_slice(&[0x48,0xb8]); s.extend_from_slice(&ret_addr.to_le_bytes()); // mov rax, fn+copy_len
    s.extend_from_slice(&[0xff,0xe0]);                              // jmp rax
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());

    let mut patch = [0u8; 12];
    patch[0]=0x48; patch[1]=0xb8; patch[2..10].copy_from_slice(&stub.to_le_bytes()); // mov rax, stub
    patch[10]=0xff; patch[11]=0xe0;                                                  // jmp rax
    let mut old: u32 = 0;
    if VirtualProtect(fn_addr, 12, RWX, &mut old) == 0 { return Err("VirtualProtect 실패"); }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), fn_addr as *mut u8, 12);
    VirtualProtect(fn_addr, 12, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), fn_addr, 12);
    Ok(())
}

unsafe fn install_hook() -> Result<(), &'static str> {
    if HOOK_INSTALLED.swap(true, Ordering::Relaxed) { return Err("이미 설치됨"); }
    // 프롤로그 확인(8 PUSH) 후 12B 복사
    let mbase = GetModuleHandleW(core::ptr::null());
    let fn_addr = mbase + DRIVER_RVA;
    if !readable(fn_addr, 16) { return Err("fn not readable"); }
    for i in 0..12 { if *((fn_addr+i) as *const u8) != PROLOGUE[i] { return Err("프롤로그 불일치"); } }
    install_hook_at(DRIVER_RVA, 12, sim_capture as usize, None)
}

// ★등록함수 0x1e2f1c0 캡처: rcx=게임상태(영속), rdx=매치데이터(0x740). 프롤로그 push×3+mov eax+call rel32(chkstk).
//   copy_len=13(call 포함), reloc_e8=8. ARM(F4)된 뒤 첫 호출 1회만(로드중 차단). SAFE(얕은덤프, 포인터추적X).
const REG_RVA: usize = 0x1e2f1c0;
static REG2_DONE: AtomicBool = AtomicBool::new(false);
unsafe extern "C" fn reg2_capture(saved_ptr: usize, entry_rsp: usize) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        if saved_ptr < 0x10000 || entry_rsp < 0x10000 { return; }
        if !RECALL_ARMED.load(Ordering::Relaxed) { return; }     // F4 ARM 전엔 무시(로드중 차단)
        if REG2_DONE.swap(true, Ordering::Relaxed) { return; }
        let base = BASE.load(Ordering::Relaxed);
        let caller = rd_u64(entry_rsp).unwrap_or(0);
        let rcx = rd_u64(saved_ptr + OFF_RCX).unwrap_or(0);  // 게임상태
        let rdx = rd_u64(saved_ptr + OFF_RDX).unwrap_or(0);  // 매치데이터(0x740)
        let mut s = format!(
            "\n[★REG2 0x1e2f1c0 진입 {}ms] caller=0x{:x}(RVA 0x{:x})\n   rcx(게임상태)=0x{:x}  rdx(매치데이터)=0x{:x}\n   게임상태+0xf8b0(HashMap)=0x{:x}  +0x800(큐)=0x{:x}\n",
            now_ms(), caller, caller.wrapping_sub(base as u64), rcx, rdx,
            rcx.wrapping_add(0xf8b0), rcx.wrapping_add(0x800));
        let ctrl = rd_u64(rcx as usize + 0xf8b0).unwrap_or(0);
        let items = rd_u64(rcx as usize + 0xf8c8).unwrap_or(0);
        s.push_str(&format!("   HashMap검증: ctrl=0x{:x}(readable={}) items={}\n", ctrl, readable(ctrl as usize, 8), items as i64));
        dump_mem("게임상태 +0..0x40", rcx as usize, 0x40, &mut s);
        dump_mem("매치데이터 0x740 (앞 0x200)", rdx as usize, 0x200, &mut s);
        append_log(&s);
    }));
}

// ★0x1e38ce0(큐+등록) detour: rcx=게임상태, rdx=매치데이터(0x740). 프롤로그 6push+mov eax imm32=13B(위치독립).
const REG2_RVA: usize = 0x1e38ce0;
static REG2_LOGGED: AtomicU64 = AtomicU64::new(0);
unsafe extern "C" fn reg_capture(saved_ptr: usize, entry_rsp: usize) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        if saved_ptr < 0x10000 || entry_rsp < 0x10000 { return; }
        if REG2_LOGGED.load(Ordering::Relaxed) >= 1 { return; }
        REG2_LOGGED.fetch_add(1, Ordering::Relaxed);
        let base = BASE.load(Ordering::Relaxed);
        let caller = rd_u64(entry_rsp).unwrap_or(0);
        let p1 = rd_u64(saved_ptr+OFF_RCX).unwrap_or(0); // rcx = 게임상태
        let p2 = rd_u64(saved_ptr+OFF_RDX).unwrap_or(0); // rdx = 매치데이터?
        let p3 = rd_u64(saved_ptr+OFF_R8).unwrap_or(0);
        let p4 = rd_u64(saved_ptr+OFF_R9).unwrap_or(0);
        let mut s = format!(
            "\n[★REG 0x1e38ce0 진입 {}ms] caller=0x{:x}(RVA 0x{:x})\n   rcx(게임상태)=0x{:x} rdx=0x{:x} r8=0x{:x} r9=0x{:x}\n",
            now_ms(), caller, caller.wrapping_sub(base as u64), p1, p2, p3, p4);
        s.push_str(&format!("   ※게임상태 베이스=0x{:x} → HashMap@+0xf8b0=0x{:x}, 대기큐@+0x800=0x{:x}\n",
            p1, p1.wrapping_add(0xf8b0), p1.wrapping_add(0x800)));
        dump_deep("*rcx(게임상태 +0..0x100)", p1 as usize, 0x100, &mut s);
        dump_deep("*rdx(매치데이터 0x740)", p2 as usize, 0x740, &mut s);
        append_log(&s);
    }));
}

// ═══════════════════════════════════════════════════════════════════════════
// ★★ 백그라운드 brief-sim 스캐너 스파이크 (2026-06-29)
//   목적: "0x1e51190 펌프가 매 서버틱 상시 도나 vs 일정넘기기때만 도나" 즉답 +
//         ServerState 베이스를 런타임 캡처(어느 레지스터/함수에 있는지).
//   둘 다 8-push 위치독립 프롤로그(exe 직접확인) → install_hook_at(copy_len=12, None) 안전.
//   훅은 최소작업(atomic 증가 + 앞 N회만 캡처). 빈도는 post_update 하트비트로 관찰.
const PUMP_RVA: usize = 0x1e51190;   // 펌프 오케스트레이터 (ServerState 생성→펌프→소멸 추정)
const SCAN_RVA: usize = 0x1e2f2a0;   // 백그라운드 스캐너 (ServerState=param)
static PUMP_HITS: AtomicU64 = AtomicU64::new(0);
static SCAN_HITS: AtomicU64 = AtomicU64::new(0);
static PUMP_CAP: AtomicU64 = AtomicU64::new(0);
static SCAN_CAP: AtomicU64 = AtomicU64::new(0);
const CAP_EACH: u64 = 3;             // 각 함수 앞 3회만 상세캡처
static LAST_HB: AtomicU64 = AtomicU64::new(0);
static LAST_PUMP_HB: AtomicU64 = AtomicU64::new(0);
static LAST_SCAN_HB: AtomicU64 = AtomicU64::new(0);

// 후보 포인터가 ServerState인지 판별할 시그니처 한 줄.
//   ServerState 표식: running_matches.ptr@+0x808 / .len@+0x810 / Arc<ClientDatabase>@+0x11ac8 /
//   게이트 bool@+0x11b08. (대조: 스냅샷db는 +0xf8b0 hashmap, 일정컨테이너는 +0x6e8 vec.)
unsafe fn ss_sig(cand: u64) -> String {
    let c = cand as usize;
    if cand <= 0x10000 || !readable(c, 8) { return format!("0x{:x} (unreadable)", cand); }
    let rm_p = rd_u64(c + 0x808).unwrap_or(0);
    let rm_l = rd_u64(c + 0x810).unwrap_or(0);
    let arc  = rd_u64(c + 0x11ac8).unwrap_or(0);
    let gate = rd_u8(c + 0x11b08).unwrap_or(0xff);
    let hm   = rd_u64(c + 0xf8b0).unwrap_or(0);
    let sch  = rd_u64(c + 0x6e8).unwrap_or(0);
    let is_ss = readable(rm_p as usize, 16) && readable(arc as usize, 16) && (rm_l as i64) >= 0 && (rm_l as i64) < 0x10000;
    format!("0x{:x}: [SS?{}] rm.ptr@808=0x{:x}(rd={}) rm.len@810={} arc@11ac8=0x{:x}(rd={}) gate@11b08={} | hm@f8b0=0x{:x}(rd={}) sched@6e8=0x{:x}",
        cand, if is_ss {"★YES"} else {"no"}, rm_p, readable(rm_p as usize,16), rm_l as i64,
        arc, readable(arc as usize,16), gate, hm, readable(hm as usize,8), sch)
}

unsafe fn is_serverstate(cand: u64) -> bool {
    let c = cand as usize;
    if cand <= 0x10000 || !readable(c, 8) { return false; }
    let rm_p = rd_u64(c + 0x808).unwrap_or(0);
    let rm_l = rd_u64(c + 0x810).unwrap_or(0);
    let arc  = rd_u64(c + 0x11ac8).unwrap_or(0);
    readable(rm_p as usize, 16) && readable(arc as usize, 16) && (rm_l as i64) >= 0 && (rm_l as i64) < 0x10000
}

static DUMP_RQ_DONE: AtomicBool = AtomicBool::new(false);

// ★ServerState의 대기 매치큐(스캐너가 순회: ptr@+0x808 / len@+0x810 / RunningMatchInfo stride 0x758) 덤프.
//   각 엔트리의 후보 필드(state@+0x738, match_id@+0x6a0, gamerule@+0x6a8, flag@+0x740, aux@+0x748) 파싱 +
//   레이아웃 역산용 raw(head + tail). = 주입할 "유효 엔트리"의 정답지.
unsafe fn dump_runqueue(ss: u64, s: &mut String) {
    let base = ss as usize;
    let ptr = rd_u64(base + 0x808).unwrap_or(0);
    let len = rd_u64(base + 0x810).unwrap_or(0);
    let cap = rd_u64(base + 0x800).unwrap_or(0);
    let hm_cnt = rd_u64(base + 0xf8c8).unwrap_or(0); // (대조) HashMap items
    s.push_str(&format!("\n[★RUNQUEUE @ServerState 0x{:x}] cap@800=0x{:x} ptr@808=0x{:x} len@810={} | (대조)matchHM items@f8c8={}\n",
        ss, cap, ptr, len as i64, hm_cnt as i64));
    if ptr <= 0x10000 || !readable(ptr as usize, 0x758) { s.push_str("   ptr unreadable — 큐 비었거나 다른 레이아웃\n"); return; }
    let stride = 0x758usize;
    let n = (len as usize).min(16);
    for i in 0..n {
        let e = ptr as usize + i * stride;
        if !readable(e, stride) { s.push_str(&format!("   rq[{}] @0x{:x} unreadable\n", i, e)); continue; }
        let st  = rd_u64(e + 0x738).unwrap_or(0);
        let mid = rd_u64(e + 0x6a0).unwrap_or(0);
        let gr  = rd_u64(e + 0x6a8).unwrap_or(0);
        let fl  = rd_u64(e + 0x740).unwrap_or(0);
        let aux = rd_u64(e + 0x748).unwrap_or(0);
        s.push_str(&format!("   rq[{}] @0x{:x}  state@738=0x{:x}  mid@6a0={}  gr@6a8={}  flag@740=0x{:x}  aux@748=0x{:x}\n",
            i, e, st, mid as i64, gr as i64, fl, aux));
    }
    // raw: 엔트리 0의 head(0x100) + 엔트리 0/1/2 tail(+0x680..+0x758) — 정확 오프셋 검증용
    dump_mem("rq[0] head +0..", ptr as usize, 0x100, s);
    for i in 0..n.min(3) {
        let e = ptr as usize + i * stride;
        dump_mem(&format!("rq[{}] tail +0x680..", i), e + 0x680, 0xd8, s);
    }
}

unsafe fn pump_scan_cap(name: &str, rva: usize, hits: &AtomicU64, cap: &AtomicU64,
                        saved_ptr: usize, entry_rsp: usize) {
    if saved_ptr < 0x10000 || entry_rsp < 0x10000 { return; }
    hits.fetch_add(1, Ordering::Relaxed);
    if cap.load(Ordering::Relaxed) >= CAP_EACH { return; }
    let n = cap.fetch_add(1, Ordering::Relaxed) + 1;
    let base = BASE.load(Ordering::Relaxed);
    let caller = rd_u64(entry_rsp).unwrap_or(0);
    let p1 = rd_u64(saved_ptr + OFF_RCX).unwrap_or(0);
    let p2 = rd_u64(saved_ptr + OFF_RDX).unwrap_or(0);
    let p3 = rd_u64(saved_ptr + OFF_R8).unwrap_or(0);
    let p4 = rd_u64(saved_ptr + OFF_R9).unwrap_or(0);
    let mut s = format!("\n[★{} 0x{:x} 진입 #{} {}ms] caller=0x{:x}(RVA 0x{:x})\n",
        name, rva, n, now_ms(), caller, caller.wrapping_sub(base as u64));
    s.push_str(&format!("   p1(rcx) {}\n", ss_sig(p1)));
    s.push_str(&format!("   p2(rdx) {}\n", ss_sig(p2)));
    s.push_str(&format!("   p3(r8)  {}\n", ss_sig(p3)));
    s.push_str(&format!("   p4(r9)  {}\n", ss_sig(p4)));
    // ★첫 유효 ServerState에서 대기 매치큐 1회 덤프(F3로 재무장 가능).
    let ss = [p1, p2, p3, p4].into_iter().find(|&c| is_serverstate(c)).unwrap_or(0);
    if ss != 0 && !DUMP_RQ_DONE.swap(true, Ordering::Relaxed) {
        dump_runqueue(ss, &mut s);
    }
    append_log(&s);
}

unsafe extern "C" fn pump_capture(saved_ptr: usize, entry_rsp: usize) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        pump_scan_cap("PUMP", PUMP_RVA, &PUMP_HITS, &PUMP_CAP, saved_ptr, entry_rsp);
    }));
}
unsafe extern "C" fn scan_capture(saved_ptr: usize, entry_rsp: usize) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        pump_scan_cap("SCAN", SCAN_RVA, &SCAN_HITS, &SCAN_CAP, saved_ptr, entry_rsp);
    }));
}

// ═══════════════════════════════════════════════════════════════════════════
// ★★ 갈래A 1단계: 완성 0x740 매치데이터 캡처 (2026-06-29)
//   seed_gen FUN_14205ae50(0x205ae50)은 0x1e2f1c0 안에서 **채워진** match+0x6a8/+0x738을 인자로 받음.
//   → out_base = arg - 0x6a8 으로 완성 0x740 덤프(clone 템플릿).
//   프롤로그 = 7-push + mov eax,imm + e8(chkstk)@offset15 → copy_len=15로 자르면 e8 보존=reloc無 안전.
//   F4 ARM 후에만 캡처(로드중 차단), 앞 SEED_CAP_MAX회.
const SEED_RVA: usize = 0x205ae50;
const SEED_COPYLEN: usize = 15;
static SEED_CAP: AtomicU64 = AtomicU64::new(0);
const SEED_CAP_MAX: u64 = 3;

unsafe extern "C" fn seed_capture(saved_ptr: usize, entry_rsp: usize) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        if saved_ptr < 0x10000 || entry_rsp < 0x10000 { return; }
        if !RECALL_ARMED.load(Ordering::Relaxed) { return; }       // F4 ARM 전엔 무시(로드중 차단)
        if SEED_CAP.load(Ordering::Relaxed) >= SEED_CAP_MAX { return; }
        let n = SEED_CAP.fetch_add(1, Ordering::Relaxed) + 1;
        let base = BASE.load(Ordering::Relaxed);
        let caller = rd_u64(entry_rsp).unwrap_or(0);
        let rcx = rd_u64(saved_ptr + OFF_RCX).unwrap_or(0);
        let rdx = rd_u64(saved_ptr + OFF_RDX).unwrap_or(0);
        let r8  = rd_u64(saved_ptr + OFF_R8).unwrap_or(0);
        let r9  = rd_u64(saved_ptr + OFF_R9).unwrap_or(0);
        let s28 = rd_u64(entry_rsp + 0x28).unwrap_or(0);
        let s30 = rd_u64(entry_rsp + 0x30).unwrap_or(0);
        let s38 = rd_u64(entry_rsp + 0x38).unwrap_or(0);
        let mut s = format!(
            "\n[★SEED 0x205ae50 진입 #{} {}ms] caller=0x{:x}(RVA 0x{:x})\n   rcx=0x{:x} rdx=0x{:x} r8=0x{:x} r9=0x{:x} | [rsp+28]=0x{:x} [+30]=0x{:x} [+38]=0x{:x}\n",
            n, now_ms(), caller, caller.wrapping_sub(base as u64), rcx, rdx, r8, r9, s28, s30, s38);
        // out_base 후보: (어느 인자가 match+0x6a8인지 불확실 → 여러 후보 -0x6a8/-0x738 시도)
        // RE 추정: r9=match+0x6a8, [rsp+28]=match+0x738. 둘로 base 계산해 일치+readable이면 그게 0x740.
        let cands: [(&str, u64); 5] = [
            ("r9-0x6a8", r9.wrapping_sub(0x6a8)),
            ("rsp28-0x738", s28.wrapping_sub(0x738)),
            ("r8-0x6a8", r8.wrapping_sub(0x6a8)),
            ("rdx-0x6a8", rdx.wrapping_sub(0x6a8)),
            ("rsp30-0x738", s30.wrapping_sub(0x738)),
        ];
        let mut dumped = false;
        for (tag, cand) in cands {
            if cand > 0x10000 && readable(cand as usize, 0x740) {
                let mt = rd_u64(cand as usize + 0x738).unwrap_or(0xffff_ffff);
                // match_type/regime은 작은 정수여야 그럴듯(0~10 부근). 큰 값/포인터면 오탐.
                let plausible = (mt & 0xffff_ffff) < 0x40;
                s.push_str(&format!("   cand {} = 0x{:x}  +0x738={:#x}  +0x6a8={:#x}  plausible={}\n",
                    tag, cand, mt, rd_u64(cand as usize + 0x6a8).unwrap_or(0), plausible));
                if plausible && !dumped {
                    dump_entry(&format!("★0x740[{}]", tag), cand as usize, 0x740, 0x400, &mut s);
                    dumped = true;
                }
            }
        }
        if !dumped { s.push_str("   ⚠ 유효 0x740 후보 못 찾음(인자오프셋 재검토 필요)\n"); }
        append_log(&s);
    }));
}

// chkstk류라도 e8가 copy_len 밖이면 reloc 불필요: copy_len 바이트(위치독립 push+mov)만 복사, e8는 원본 잔존.
unsafe fn install_chkstk_safe(rva: usize, copy_len: usize, e8_off: usize, cap_fn: usize, name: &str) {
    let mbase = GetModuleHandleW(core::ptr::null());
    let fn_addr = mbase + rva;
    if !readable(fn_addr, 16) { append_log(&format!("[hook] {} 0x{:x} unreadable\n", name, rva)); return; }
    if *(fn_addr as *const u8) != 0x55 || *((fn_addr + e8_off) as *const u8) != 0xe8 {
        append_log(&format!("[hook] {} 0x{:x} 프롤로그 불일치(55..e8@{:#x} 아님) — 건너뜀\n", name, rva, e8_off));
        return;
    }
    match install_hook_at(rva, copy_len, cap_fn, None) {
        Ok(()) => append_log(&format!("[hook] {} 0x{:x} detour 설치 성공 (copy_len={}, e8@{:#x} 보존)\n", name, rva, copy_len, e8_off)),
        Err(e) => append_log(&format!("[hook] {} 0x{:x} 설치 실패: {}\n", name, rva, e)),
    }
}

// 8-push 위치독립 프롤로그 함수 안전 설치(프롤로그 검증 후 copy_len=12, reloc 없음).
unsafe fn install_8push(rva: usize, cap_fn: usize, name: &str) {
    let mbase = GetModuleHandleW(core::ptr::null());
    let fn_addr = mbase + rva;
    if !readable(fn_addr, 16) { append_log(&format!("[hook] {} 0x{:x} unreadable\n", name, rva)); return; }
    for i in 0..12 {
        if *((fn_addr + i) as *const u8) != PROLOGUE[i] {
            append_log(&format!("[hook] {} 0x{:x} 프롤로그 불일치(8-push 아님) — 설치 건너뜀\n", name, rva));
            return;
        }
    }
    match install_hook_at(rva, 12, cap_fn, None) {
        Ok(()) => append_log(&format!("[hook] {} 0x{:x} detour 설치 성공 (8-push)\n", name, rva)),
        Err(e) => append_log(&format!("[hook] {} 0x{:x} 설치 실패: {}\n", name, rva, e)),
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// ★★ 갈래A 1단계(안전판2): 0x18cbef0(8-push) 진입 detour로 ★입력 인자★ 캡처(forward only).
//   return-hook(반환주소 스왑)=크래시 확정 → 폐기. 대신 입력(rdx/r8/r9/스택=team/GameRule/roster)을
//   안전캡처 → 나중에 우리가 직접 0x18cbef0 호출(정상 call+ret=안전)해 0x740 빌드 가능성 판단.
//   각 입력의 힙(0x2..)/스택(0x34..) 여부 = 재사용(영속) 가능성 판별.
const MDB_RVA: usize = 0x18cbef0;
static MDB_CAP: AtomicU64 = AtomicU64::new(0);
const MDB_CAP_MAX: u64 = 5;
// ★크래시안전: 캡처 시점(advance중)엔 raw값만. roster ptr를 저장→post_update(idle)서 재read해 영속성 확인.
static MDB_ROSTER: AtomicU64 = AtomicU64::new(0);
static MDB_OUT740: AtomicU64 = AtomicU64::new(0);
static MDB_PERSIST_CHECKS: AtomicU64 = AtomicU64::new(0);

unsafe extern "C" fn mdb_entry(saved_ptr: usize, entry_rsp: usize) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        if saved_ptr < 0x10000 || entry_rsp < 0x10000 { return; }
        if !RECALL_ARMED.load(Ordering::Relaxed) { return; }
        if MDB_CAP.load(Ordering::Relaxed) >= MDB_CAP_MAX { return; }
        let n = MDB_CAP.fetch_add(1, Ordering::Relaxed) + 1;
        let base = BASE.load(Ordering::Relaxed);
        // ★raw값만 read(레지스터/스택 슬롯=항상 유효). 포인터가 가리키는 메모리는 추적 안 함(TOCTOU 회피).
        let caller = rd_u64(entry_rsp).unwrap_or(0);
        let rcx = rd_u64(saved_ptr + OFF_RCX).unwrap_or(0); // 출력 0x740버퍼
        let rdx = rd_u64(saved_ptr + OFF_RDX).unwrap_or(0); // disc
        let r8  = rd_u64(saved_ptr + OFF_R8).unwrap_or(0);  // roster
        let r9  = rd_u64(saved_ptr + OFF_R9).unwrap_or(0);  // count
        let s28 = rd_u64(entry_rsp + 0x28).unwrap_or(0);
        let s38 = rd_u64(entry_rsp + 0x38).unwrap_or(0);
        if r8 > 0x10000 { MDB_ROSTER.store(r8, Ordering::Relaxed); }       // 영속성 검사용(post_update)
        if rcx > 0x10000 { MDB_OUT740.store(rcx, Ordering::Relaxed); }
        append_log(&format!(
            "[★MDB #{} {}ms] caller=RVA0x{:x} | rcx(out)=0x{:x} rdx(disc)=0x{:x} r8(roster)=0x{:x} r9(count)={} [rsp28]=0x{:x} [rsp38]=0x{:x}\n",
            n, now_ms(), caller.wrapping_sub(base as u64), rcx, rdx, r8, r9 as i64, s28, s38));
    }));
}

// idle(post_update)서 호출: 저장된 roster ptr를 재read → 챔프名 여전한지 = 영속성 확인. (advance 끝난 뒤라 안전)
unsafe fn mdb_persist_probe(s: &mut String) {
    let r = MDB_ROSTER.load(Ordering::Relaxed);
    if r <= 0x10000 { return; }
    let c = MDB_PERSIST_CHECKS.fetch_add(1, Ordering::Relaxed);
    if c >= 4 { return; } // 4회만
    if readable(r as usize, 0x80) {
        let mut name = String::new();
        for i in 0..16 { match rd_u8(r as usize + i) { Some(b) if (0x20..0x7f).contains(&b) => name.push(b as char), _ => name.push('.') } }
        s.push_str(&format!("[roster영속검사 #{} {}ms] 0x{:x} readable=O 첫16B=\"{}\"\n", c, now_ms(), r, name));
    } else {
        s.push_str(&format!("[roster영속검사 #{} {}ms] 0x{:x} readable=X (해제됨=임시)\n", c, now_ms(), r));
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// ★★R16 메일박스(app+0x24880) 계측 (2026-07-02): 임의매치 헤드리스 spawn 요청블록 포맷 캡처.
//   프레임함수 0x435ff0가 app+0x24880(-1=없음)을 읽고 스폰→즉시 -1리셋(0x441263). 진입 RCX=app 베이스.
//   첫 12B = 순수 8-push(55 41 57 41 56 41 55 41 54 56 57 53), chkstk e8는 +17이라 copy_len=12 안전
//   (REG2/SEED는 chkstk가 copy_len 내라 크래시했음 — 여기는 아님). 진입시점=메일박스 리셋 전이라 요청 확실히 관찰.
//   cap fn은 매프레임 hot → 최소작업: 세 요청슬롯 다 -1이면 즉시 return. 덤프는 요청 있을때만(희귀). F4(ARM) 게이트.
const MAILBOX_RVA: usize = 0x435ff0;
static MAILBOX_DUMPS: AtomicU64 = AtomicU64::new(0);
const MAILBOX_DUMP_CAP: u64 = 40;
unsafe extern "C" fn mailbox_capture(saved_ptr: usize, entry_rsp: usize) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        if saved_ptr < 0x10000 || entry_rsp < 0x10000 { return; }
        if !RECALL_ARMED.load(Ordering::Relaxed) { return; }   // F4 후에만(로드중/부분초기화 차단)
        let app = rd_u64(saved_ptr + OFF_RCX).unwrap_or(0) as usize; // 진입 RCX = app 베이스
        if app <= 0x10000 { return; }
        const NONE: u64 = 0xffff_ffff_ffff_ffff;
        let m1 = rd_u64(app + 0x24880).unwrap_or(NONE);        // 1번 요청슬롯(sentinel)
        let m2 = rd_u64(app + 0x248f0).unwrap_or(NONE);        // 2번 요청슬롯(0x43cffa/0x441da2 소비)
        let m3 = rd_u64(app + 0x3bd98).unwrap_or(NONE);        // 3번 요청슬롯(프롤로그 0x43608d 체크)
        // ★slot3(0x3bd98)=상수 0x10 오판정(진짜 -1센티넬 아님) → 트리거서 제외, 정보로만 로깅.
        //   진짜 요청슬롯 = slot1(0x24880)/slot2(0x248f0)만 -1센티넬. 둘 다 -1이면 요청없음 = 즉시 return.
        if m1 == NONE && m2 == NONE { return; }
        if MAILBOX_DUMPS.load(Ordering::Relaxed) >= MAILBOX_DUMP_CAP { return; }
        MAILBOX_DUMPS.fetch_add(1, Ordering::Relaxed);
        let mut s = format!(
            "\n[★★MAILBOX 요청감지 {}ms] app=0x{:x}\n   slot1@+0x24880=0x{:x}  slot2@+0x248f0=0x{:x}  slot3@+0x3bd98=0x{:x}  (-1=없음)\n",
            now_ms(), app, m1, m2, m3);
        // 요청블록 전체(0x24880..0x24900): sentinel + 0x248b0/c0(xmm) + 0x248d0/d1(플래그) + 0x248e0/e8(매치리스트)
        dump_mem("요청블록 app+0x24880..+0x24900", app + 0x24880, 0x80, &mut s);
        // 추가 매치리스트 ptr(+0x248e0) 타깃 추적 — 여기에 우리가 넣을 매치 식별정보가 있을 것.
        let listp = rd_u64(app + 0x248e0).unwrap_or(0);
        let liste = rd_u64(app + 0x248e8).unwrap_or(0);
        s.push_str(&format!("   listptr@+0x248e0=0x{:x}  listend@+0x248e8=0x{:x}\n", listp, liste));
        if listp > 0x10000 && readable(listp as usize, 0x60) {
            dump_mem("  매치리스트 타깃 앞0x60", listp as usize, 0x60, &mut s);
        }
        append_log(&s);
    }));
}

// ═══════════════════════════════════════════════════════════════════════════
// ★★R16 Game빌더(0x8bc300) 캡처 (2026-07-02): 일정넘김 스폰 경로(0x4382ea→0x8bc300)가 실제 매치스폰.
//   메일박스와 별개=일정넘김이 쓰는 직접경로. 첫 12B=순수 8-push(55 41 57..53), chkstk e8 +17이라 copy_len=12 안전.
//   rdx=mode, r8=cfg(매치config=팀/챔프), rcx=this/out Game. 매치 스폰당 1회(저빈도). F4(ARM) 게이트.
const BUILDER_RVA: usize = 0x8bc300;
static BUILDER_CAP: AtomicU64 = AtomicU64::new(0);
const BUILDER_CAP_MAX: u64 = 20;
unsafe extern "C" fn builder_capture(saved_ptr: usize, entry_rsp: usize) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        if saved_ptr < 0x10000 || entry_rsp < 0x10000 { return; }
        if !RECALL_ARMED.load(Ordering::Relaxed) { return; }
        if BUILDER_CAP.load(Ordering::Relaxed) >= BUILDER_CAP_MAX { return; }
        let n = BUILDER_CAP.fetch_add(1, Ordering::Relaxed) + 1;
        let base = BASE.load(Ordering::Relaxed);
        let caller = rd_u64(entry_rsp).unwrap_or(0);
        let rcx = rd_u64(saved_ptr + OFF_RCX).unwrap_or(0); // this/out Game
        let rdx = rd_u64(saved_ptr + OFF_RDX).unwrap_or(0); // mode
        let r8  = rd_u64(saved_ptr + OFF_R8).unwrap_or(0);  // cfg (매치 config)
        let r9  = rd_u64(saved_ptr + OFF_R9).unwrap_or(0);
        let s28 = rd_u64(entry_rsp + 0x28).unwrap_or(0);
        let s30 = rd_u64(entry_rsp + 0x30).unwrap_or(0);
        let mut s = format!(
            "\n[★★BUILDER 0x8bc300 스폰 #{} {}ms] caller=RVA0x{:x}\n   rcx(this/out)=0x{:x} rdx(mode)=0x{:x} r8(cfg)=0x{:x} r9=0x{:x} [rsp28]=0x{:x} [rsp30]=0x{:x}\n",
            n, now_ms(), caller.wrapping_sub(base as u64), rcx, rdx, r8, r9, s28, s30);
        dump_deep("*rcx(out/this Game 앞0x80)", rcx as usize, 0x80, &mut s);
        dump_deep("*r8(cfg 매치config 앞0x120)", r8 as usize, 0x120, &mut s); // 팀/챔프/시드 역산용
        append_log(&s);
    }));
}

// ═══════════════════════════════════════════════════════════════════════════
// ★★★R16 부활 seam — RESUME future(0x100c870/0x100c9d0) 캡처 (2026-07-04):
//   sim 오케스트레이터 0xc5ccb0의 직접호출자=resume(call사이트 0x100c92f/0x100ca73). 진입 rcx=future-self가
//   매치소스 완전확정 보유(매치리스트@[self+0x20슬라이스], DB/sched핸들@[self+0]/[+8]/[+0x10]/[+0x18]/[+0x50]).
//   프롤로그 7push+sub rsp,0x80(chkstk無, copy_len=17 안전). DB복제는 오케 내부라 resume(직전)=복제前 intercept seam.
//   목표: self 필드 덤프→어느 필드가 ClientDatabase-arc(rm@+0x808/arc@+0x11ac8 시그)/sched/매치리스트인지 매핑.
const RESUME1_RVA: usize = 0x100c870;
const RESUME2_RVA: usize = 0x100c9d0;
static RESUME_CAP: AtomicU64 = AtomicU64::new(0);
const RESUME_CAP_MAX: u64 = 24;
static RESUME_INSTALLED: AtomicBool = AtomicBool::new(false);
unsafe extern "C" fn resume_capture(saved_ptr: usize, entry_rsp: usize) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        if saved_ptr < 0x10000 || entry_rsp < 0x10000 { return; }
        if !RECALL_ARMED.load(Ordering::Relaxed) { return; }
        if RESUME_CAP.load(Ordering::Relaxed) >= RESUME_CAP_MAX { return; }
        let n = RESUME_CAP.fetch_add(1, Ordering::Relaxed) + 1;
        let base = BASE.load(Ordering::Relaxed);
        let caller = rd_u64(entry_rsp).unwrap_or(0);
        let selfp = rd_u64(saved_ptr + OFF_RCX).unwrap_or(0) as usize; // future-self
        if selfp <= 0x10000 { return; }
        let mut s = format!(
            "\n[★★RESUME future-self #{} {}ms] caller=RVA0x{:x} self(rcx)=0x{:x}\n",
            n, now_ms(), caller.wrapping_sub(base as u64), selfp);
        // self 필드별: DB/sched/매치리스트 핸들 후보. ServerState/ClientDatabase 시그(rm@+0x808·arc@+0x11ac8) 판정.
        for &o in &[0usize, 8, 0x10, 0x18, 0x20, 0x28, 0x30, 0x38, 0x40, 0x48, 0x50, 0x58] {
            let v = rd_u64(selfp + o).unwrap_or(0);
            let mut tag = String::new();
            if v > 0x10000 && readable(v as usize, 8) {
                let rm = rd_u64(v as usize + 0x808).unwrap_or(0);
                let arc = rd_u64(v as usize + 0x11ac8).unwrap_or(0);
                if readable(rm as usize, 16) && readable(arc as usize, 8) { tag = "  ★SS후보(rm@808·arc@11ac8 유효)".into(); }
                else if readable(v as usize, 0x18) {
                    // Vec{ptr,len,cap} 후보(매치리스트=len 3)
                    let p = rd_u64(v as usize).unwrap_or(0);
                    let l = rd_u64(v as usize + 8).unwrap_or(0);
                    if readable(p as usize, 8) && (l as i64) >= 0 && (l as i64) < 0x100 { tag = format!("  Vec?(ptr=0x{:x} len={})", p, l as i64); }
                }
            }
            s.push_str(&format!("   [self+0x{:02x}] = 0x{:x}{}\n", o, v, tag));
        }
        dump_deep("*self 앞0xA0(future 상태)", selfp, 0xa0, &mut s);
        append_log(&s);
    }));
}

fn key_down(vk: i32) -> bool { (unsafe { GetAsyncKeyState(vk) } as u16 & 0x8000) != 0 }

struct ProbeExt;
impl ModExtension for ProbeExt {
    fn post_update(&self, _scene: &mut Scene, _ui: &mut GameUI, _assets: &mut Assets, _dt: f32) {
        if !BOOTED.swap(true, Ordering::Relaxed) {
            append_log(&format!("[{}ms] [ext] 폴링 시작 — F3=캡처 카운터 리셋(추가 캡처). 매치 시뮬되면 자동 로깅됨.\n", now_ms()));
        }
        // ★하트비트: 2초마다 pump/scan 호출 증분 로깅 → "상시틱 vs 일정넘기기때만" 판별.
        //   idle 중에도 증분>0이면 상시 백그라운드 스캐너 = 날짜무관 headless 가능.
        let t = now_ms();
        if t.wrapping_sub(LAST_HB.load(Ordering::Relaxed)) >= 2000 {
            LAST_HB.store(t, Ordering::Relaxed);
            let p = PUMP_HITS.load(Ordering::Relaxed);
            let sc = SCAN_HITS.load(Ordering::Relaxed);
            let dp = p.wrapping_sub(LAST_PUMP_HB.swap(p, Ordering::Relaxed));
            let ds = sc.wrapping_sub(LAST_SCAN_HB.swap(sc, Ordering::Relaxed));
            let mut hb = format!("[hb {}ms] PUMP +{} SCAN +{} (누적 {}/{})\n", t, dp, ds, p, sc);
            unsafe { mdb_persist_probe(&mut hb); } // roster 영속성 idle 검사(안전)
            append_log(&hb);
        }
        // ★★★R16 seam 캡처 결과 출력(VEH가 atomic stash → 여기 안전컨텍스트서 로깅). one-shot.
        if MB_CAPTURED.load(Ordering::Relaxed) && !MB_LOGGED.swap(true, Ordering::Relaxed) {
            let selfp = MB_FLD[0].load(Ordering::Relaxed) as usize;
            let mut s = format!("\n[★★★SEAM future-self 캡처(int3/VEH) {}ms] self(rcx)=0x{:x} readable={}\n",
                now_ms(), selfp, unsafe { readable(selfp, 8) });
            let offs = [0usize,8,0x10,0x18,0x20,0x28,0x30,0x38,0x40,0x48,0x50,0x58,0x60,0x68,0x70,0x78];
            unsafe {
                for &o in offs.iter() {
                    let v = rd_u64(selfp + o).unwrap_or(0);
                    let mut tag = String::new();
                    if v > 0x10000 && readable(v as usize, 8) {
                        let rm = rd_u64(v as usize + 0x808).unwrap_or(0);
                        let arc = rd_u64(v as usize + 0x11ac8).unwrap_or(0);
                        if readable(rm as usize, 16) && readable(arc as usize, 8) { tag = "  ★SS후보(rm@808·arc@11ac8 유효)".into(); }
                        else if readable(v as usize, 0x18) {
                            let p = rd_u64(v as usize).unwrap_or(0); let l = rd_u64(v as usize + 8).unwrap_or(0);
                            if readable(p as usize, 8) && (l as i64) >= 0 && (l as i64) < 0x100 { tag = format!("  Vec?(ptr=0x{:x} len={})", p, l as i64); }
                        }
                    }
                    s.push_str(&format!("   [self+0x{:02x}] = 0x{:x}{}\n", o, v, tag));
                }
            }
            append_log(&s);
        }
        let f3 = key_down(VK_F3);
        if f3 && !PREV_F3.swap(f3, Ordering::Relaxed) {
            LOGGED.store(0, Ordering::Relaxed);
            // ★스캐너/펌프 캡처 + 큐덤프 재무장 (매치 sim 도는 순간 다시 캡처용)
            SCAN_CAP.store(0, Ordering::Relaxed);
            PUMP_CAP.store(0, Ordering::Relaxed);
            SEED_CAP.store(0, Ordering::Relaxed);
            DUMP_RQ_DONE.store(false, Ordering::Relaxed);
            append_log(&format!("\n[{}ms] ===== F3 캡처 리셋 (드라이버+SCAN+PUMP+큐덤프 재무장) | 누적 SCAN {} =====\n",
                now_ms(), SCAN_HITS.load(Ordering::Relaxed)));
        } else if !f3 { PREV_F3.store(false, Ordering::Relaxed); }
        // ★F4 = 재호출 ARM (이후 첫 드라이버 호출 때 단독 재호출 테스트 발화)
        let f4 = key_down(VK_F4);
        if f4 && !PREV_F4.swap(true, Ordering::Relaxed) {
            RECALL_ARMED.store(true, Ordering::Relaxed);
            RECALL_TESTED.store(false, Ordering::Relaxed);
            SEED_CAP.store(0, Ordering::Relaxed);
            MDB_CAP.store(0, Ordering::Relaxed);
            MDB_PERSIST_CHECKS.store(0, Ordering::Relaxed);
            MAILBOX_DUMPS.store(0, Ordering::Relaxed);   // ★메일박스 캡 리셋(재시도용)
            RESUME_CAP.store(0, Ordering::Relaxed);
            // ★★resume 훅 지연설치: init서 걸면 async 로드레이스 크래시 → F4(게임 안정·sim idle) 1회 설치.
            //   F4는 메뉴/일정 화면서 누르므로 0x100c870이 실행중 아님 = 패치 레이스 없음.
            // ★★R16 seam int3+VEH 무장(F4 1회): 트램폴린 대신 1바이트 원자패치(핫 async 레이스 회피).
            if !RESUME_INSTALLED.swap(true, Ordering::Relaxed) {
                unsafe {
                    if !VEH_REGISTERED.swap(true, Ordering::Relaxed) {
                        AddVectoredExceptionHandler(1, seam_veh as usize);
                    }
                    let a = GetModuleHandleW(core::ptr::null()) + RESUME1_RVA; // 0x100c870
                    if readable(a, 1) {
                        BP_ORIG.store(*(a as *const u8) as u64, Ordering::Relaxed);
                        let mut old: u32 = 0;
                        if VirtualProtect(a, 1, 0x40, &mut old) != 0 {
                            *(a as *mut u8) = 0xcc;             // int3 (원자 1바이트)
                            VirtualProtect(a, 1, old, &mut old);
                            FlushInstructionCache(GetCurrentProcess(), a, 1);
                            BP_ADDR.store(a, Ordering::Relaxed);
                            append_log(&format!("[seam] ★int3 무장 @0x{:x} (RESUME1 0x100c870). 경기 진행 시 VEH가 future-self 캡처(one-shot)\n", a));
                        } else { append_log("[seam] VirtualProtect 실패\n"); }
                    } else { append_log("[seam] 0x100c870 unreadable\n"); }
                }
            }
            append_log(&format!("\n[{}ms] ===== F4: ARM됨. 이제 매치 등록(진행하기/solo) 발생시키면 ★완성 0x740 캡처(SEED) =====\n", now_ms()));
        } else if !f4 { PREV_F4.store(false, Ordering::Relaxed); }
    }
}

// ★프롤로그 덤프: 트램폴린 작성용. 각 후보함수 첫 16바이트 + 모듈베이스 로깅(읽기만, detour 아님).
unsafe fn dump_prologues() {
    let mbase = GetModuleHandleW(core::ptr::null());
    let mut s = format!("[{}ms] === 프롤로그 덤프 (module_base=0x{:x}) ===\n", now_ms(), mbase);
    for (name, rva) in [
        ("register 0x1e2f1c0", 0x1e2f1c0usize),
        ("queue+hashmap 0x1e38ce0", 0x1e38ce0),
        ("matchdata_builder 0x18cbef0", 0x18cbef0),
        ("hashmap_insert 0x1941af0", 0x1941af0),
        ("driver 0x204f810", 0x204f810),
    ] {
        let a = mbase + rva;
        if readable(a, 16) {
            let b: Vec<String> = (0..16).map(|i| format!("{:02x}", *((a+i) as *const u8))).collect();
            s.push_str(&format!("  {:<28} @0x{:x}: {}\n", name, a, b.join(" ")));
        } else {
            s.push_str(&format!("  {:<28} @0x{:x}: <unreadable>\n", name, a));
        }
    }
    append_log(&s);
}

fn init(_ctx: &GameCtx) -> ModRegistration {
    append_log(&format!("[{}ms] === sim_probe INIT (driver 0x{:x}) ===\n", now_ms(), DRIVER_RVA));
    unsafe { dump_prologues(); }
    unsafe {
        match install_hook() {
            Ok(()) => append_log("[hook] match-sim 드라이버(FUN_14204f810 @ 0x204f810) detour 설치 성공\n"),
            Err(e) => append_log(&format!("[hook] 설치 실패: {}\n", e)),
        }
        // ★REG2(0x1e2f1c0, chkstk) detour 비활성 — 로드크래시(미해결). 대신 INS(0x1941af0, 8-push 안전).
        const ENABLE_REG2_HOOK: bool = false;
        if ENABLE_REG2_HOOK { let _ = install_hook_at(REG_RVA, 13, reg2_capture as usize, Some(8)); }
        // ★INS 0x1941af0 (HashMap insert, 8-push = 드라이버처럼 안전): 게임상태=rcx-0xf8b0 캡처.
        match install_hook_at(INS_RVA, 12, ins_capture as usize, None) {
            Ok(()) => append_log("[hook] INS 0x1941af0 detour 설치 성공 (8-push). F4 ARM 후 진행하기 시 게임상태 캡처\n"),
            Err(e) => append_log(&format!("[hook] INS 설치 실패: {}\n", e)),
        }
        // ★★ 백그라운드 brief-sim 스파이크: 펌프(0x1e51190)+스캐너(0x1e2f2a0) 둘 다 8-push 안전.
        //   진입 RDX=ServerState(p2) 캡처(ss_sig ★YES 표식) + 하트비트로 호출빈도 관찰.
        install_8push(PUMP_RVA, pump_capture as usize, "PUMP");
        install_8push(SCAN_RVA, scan_capture as usize, "SCAN");
        // ★★R16 메일박스 0x435ff0(프레임함수, app 소비지점): 첫 12B 순수 8-push(chkstk +17) = copy_len=12 안전.
        //   진입 RCX=app. F4 ARM 후 진행하기/매치등록 시 app+0x24880 요청블록 캡처 = 임의매치 spawn 포맷.
        match install_hook_at(MAILBOX_RVA, 12, mailbox_capture as usize, None) {
            Ok(()) => append_log("[hook] ★MAILBOX 0x435ff0 detour 설치 성공 (8-push). F4 ARM 후 진행하기 시 app+0x24880 요청블록 캡처\n"),
            Err(e) => append_log(&format!("[hook] MAILBOX 설치 실패: {}\n", e)),
        }
        // ★★R16 Game빌더 0x8bc300(일정넘김 스폰경로): 첫 12B 순수 8-push(chkstk +17) = copy_len=12 안전.
        //   매치 스폰당 rcx/rdx(mode)/r8(cfg=매치config) 캡처 = 일정넘김이 스폰하는 진짜 매치 입력.
        match install_hook_at(BUILDER_RVA, 12, builder_capture as usize, None) {
            Ok(()) => append_log("[hook] ★BUILDER 0x8bc300 detour 설치 성공 (8-push). F4 ARM 후 진행하기 시 매치 스폰 config 캡처\n"),
            Err(e) => append_log(&format!("[hook] BUILDER 설치 실패: {}\n", e)),
        }
        // ★★★R16 부활 seam RESUME(0x100c870/0x100c9d0) = init 설치 금지(async 로드레이스 크래시 확인 2026-07-04).
        //   → F4(게임 안정·sim idle)에 지연설치. 아래 post_update F4 핸들러 참조.
        // ⚠ SEED 0x205ae50 detour = chkstk라 copy_len=15 우회해도 로드크래시(§203). 비활성.
        let _ = (SEED_RVA, SEED_COPYLEN, seed_capture as usize, install_chkstk_safe as usize);
        // ⚠ roster=임시(영속성검사로 확정, freed+재사용) → standalone 입력재사용 불가. MDB 비활성.
        let _ = (MDB_RVA, mdb_entry as usize, mdb_persist_probe as usize);
    }
    let mut reg = ModRegistration::new(MOD_ID);
    reg.set_extension(ProbeExt);
    reg
}
declare_mod!(init);
