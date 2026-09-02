// ★VEH 크래시 로거 (2026-08-27 이식 — 참조구현 = tfm2_comptest_unlock).
//   champ_pos_lock 은 크래시 로거가 없어 "설정에서 누르면 크래시"의 흔적이 전혀 안 남았고,
//   그래서 미재핀 상수를 근거 없이 원인으로 추측하는 낭비를 했다. ⟹ RIP 를 직접 잡는다.
//   ⚠VEH 안에서는 할당·락·format! 금지 → 고정 배열 + 수동 hex + raw WriteFile 만 쓴다.
//   경로는 설치 시점에 UTF-16 으로 미리 만들어 둔다.
#[repr(C)]
struct ExcRecord {
    code: u32,
    flags: u32,
    next: usize,
    addr: usize,
    np: u32,
    _pad: u32,
    params: [usize; 15],
}
#[repr(C)]
struct ExcPointers {
    rec: *mut ExcRecord,
    ctx: *mut core::ffi::c_void,
}
use core::sync::atomic::{AtomicUsize, AtomicU64, Ordering};

static mut CRASH_PATH_W: [u16; 520] = [0u16; 520];
static CRASH_PATH_LEN: AtomicUsize = AtomicUsize::new(0);
static CRASH_WROTE: AtomicU64 = AtomicU64::new(0);
pub(crate) static MOD_BASE: AtomicUsize = AtomicUsize::new(0);

extern "system" {
    fn CreateFileW(name: *const u16, access: u32, share: u32, sa: usize,
                   disp: u32, flags: u32, tmpl: usize) -> usize;
    fn WriteFile(h: usize, buf: *const u8, n: u32, written: *mut u32, ovl: usize) -> i32;
    fn SetFilePointer(h: usize, lo: i32, hi: *mut i32, method: u32) -> u32;
    fn CloseHandle(h: usize) -> i32;
    fn GetModuleHandleW(name: *const u16) -> usize;
    fn GetModuleHandleExW(f: u32, name: *const u16, h: *mut usize) -> i32;
    fn RtlCaptureStackBackTrace(skip: u32, n: u32, frames: *mut usize, hash: *mut u32) -> u16;
    fn AddVectoredExceptionHandler(first: u32, h: extern "system" fn(*mut ExcPointers) -> i32) -> usize;
}

#[inline]
fn hexb(buf: &mut [u8], pos: &mut usize, mut v: u64) {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut tmp = [0u8; 16];
    let mut k = 0;
    if v == 0 { tmp[0] = b'0'; k = 1; }
    while v > 0 { tmp[k] = HEX[(v & 0xf) as usize]; v >>= 4; k += 1; }
    if *pos + k + 2 >= buf.len() { return; }
    buf[*pos] = b'0'; buf[*pos + 1] = b'x'; *pos += 2;
    while k > 0 { k -= 1; buf[*pos] = tmp[k]; *pos += 1; }
}
#[inline]
fn puts(buf: &mut [u8], pos: &mut usize, s: &[u8]) {
    for &c in s {
        if *pos >= buf.len() { return; }
        buf[*pos] = c; *pos += 1;
    }
}

extern "system" fn crash_veh(p: *mut ExcPointers) -> i32 {
    const SEARCH: i32 = 0;
    unsafe {
        if p.is_null() { return SEARCH; }
        let rec = (*p).rec;
        if rec.is_null() { return SEARCH; }
        let code = (*rec).code;
        // 1st-chance 소음 제외 — 진짜 죽는 것만
        let fatal = matches!(code, 0xc0000005 | 0xc00000fd | 0xc000001d | 0xc000001e
                                 | 0xc0000096 | 0xc0000094 | 0xc000008e | 0xc0000409 | 0xc000041d);
        if !fatal { return SEARCH; }
        let base0 = GetModuleHandleW(core::ptr::null());
        let mb0 = MOD_BASE.load(Ordering::Relaxed);
        let a0 = (*rec).addr;
        let in_exe = base0 != 0 && a0 > base0 && a0 - base0 < 0x8000000;
        let in_mod = mb0 != 0 && a0 > mb0 && a0 - mb0 < 0x1000000;
        // 다른 모드가 의도적으로 내는 예외까지 "크래시"로 적지 않는다
        if !in_exe && !in_mod { return SEARCH; }
        if CRASH_WROTE.fetch_add(1, Ordering::Relaxed) >= 3 { return SEARCH; }
        if CRASH_PATH_LEN.load(Ordering::Relaxed) == 0 { return SEARCH; }

        let mut buf = [0u8; 1400];
        let mut n = 0usize;
        puts(&mut buf, &mut n, b"\r\n=== CRASH (tfm2_champ_pos_lock) ===\r\ncode=");
        hexb(&mut buf, &mut n, code as u64);
        puts(&mut buf, &mut n, b" addr=");
        hexb(&mut buf, &mut n, a0 as u64);
        puts(&mut buf, &mut n, b"\r\nexe_base=");
        hexb(&mut buf, &mut n, base0 as u64);
        puts(&mut buf, &mut n, b" mod_base=");
        hexb(&mut buf, &mut n, mb0 as u64);
        if in_exe {
            puts(&mut buf, &mut n, b"\r\n*FAULT = exe+");
            hexb(&mut buf, &mut n, (a0 - base0) as u64);
        } else if in_mod {
            puts(&mut buf, &mut n, b"\r\n*FAULT = MOD+");
            hexb(&mut buf, &mut n, (a0 - mb0) as u64);
        }
        if (*rec).np >= 2 {
            puts(&mut buf, &mut n, b"\r\naccess=");
            hexb(&mut buf, &mut n, (*rec).params[0] as u64);
            puts(&mut buf, &mut n, b" fault=");
            hexb(&mut buf, &mut n, (*rec).params[1] as u64);
        }
        // 스택 되추적 — exe 프레임/모드 프레임을 갈라 어느 훅에서 왔는지 본다
        let mut frames = [0usize; 24];
        let cnt = RtlCaptureStackBackTrace(0, 24, frames.as_mut_ptr(), core::ptr::null_mut());
        puts(&mut buf, &mut n, b"\r\nstack:");
        for i in 0..(cnt as usize).min(24) {
            let f = frames[i];
            puts(&mut buf, &mut n, b"\r\n  ");
            if base0 != 0 && f > base0 && f - base0 < 0x8000000 {
                puts(&mut buf, &mut n, b"exe+"); hexb(&mut buf, &mut n, (f - base0) as u64);
            } else if mb0 != 0 && f > mb0 && f - mb0 < 0x1000000 {
                puts(&mut buf, &mut n, b"MOD+"); hexb(&mut buf, &mut n, (f - mb0) as u64);
            } else {
                hexb(&mut buf, &mut n, f as u64);
            }
        }
        puts(&mut buf, &mut n, b"\r\n=== end ===\r\n");

        let h = CreateFileW(core::ptr::addr_of!(CRASH_PATH_W) as *const u16,
                            0x40000000, 0x1 | 0x2, 0, 4, 0x80, 0);
        if h != usize::MAX && h != 0 {
            SetFilePointer(h, 0, core::ptr::null_mut(), 2);   // FILE_END
            let mut w: u32 = 0;
            WriteFile(h, buf.as_ptr(), n as u32, &mut w, 0);
            CloseHandle(h);
        }
        SEARCH
    }
}

pub(crate) fn install() {
    unsafe {
        let mut h: usize = 0;
        if GetModuleHandleExW(0x4 | 0x2, install as *const () as *const u16, &mut h) != 0 {
            MOD_BASE.store(h, Ordering::Relaxed);
        }
        let Some(d) = crate::mod_dir() else { return };
        let path = format!("{d}\\champ_pos_lock_crash.txt");
        let s: Vec<u16> = path.encode_utf16().collect();
        if s.len() >= 519 { return; }
        for (i, &c) in s.iter().enumerate() { CRASH_PATH_W[i] = c; }
        CRASH_PATH_W[s.len()] = 0;
        CRASH_PATH_LEN.store(s.len(), Ordering::Relaxed);
        AddVectoredExceptionHandler(1, crash_veh);   // first=1 = 최우선
        install_panic_hook();   // ★VEH 가 못 잡는 Rust panic 을 잡는다(진짜 범인 후보)
    }
    install_mod_panic_hook();   // ★★모드 자신의 panic(게임 깔때기가 안 잡는 부분)
}


// ══════════════════════════════════════════════════════════════════════════
// ★panic 깔때기 훅 (2026-08-27) — 참조 = tfm2_banpick_order/src/diag.rs
//   게임은 Rust `panic=abort` 빌드라 모든 panic 이 `int 0x29` __fastfail 로 즉사한다
//   ⟹ VEH·crash_log·콜스택이 전부 우회된다. "크래시는 나는데 VEH 미포착"이면
//      AV 가 아니라 **Rust panic** 을 먼저 의심할 것(정본 = [[tfm2-crash-diagnosis-panic-hook]]).
//   모든 panic 은 rust_panic_with_hook 을 반드시 통과하고, 진입 시
//      r8 = &Location { file_ptr, file_len, line: u32, col: u32 }
//   ⟹ 여기서 file:line:col 을 남기면 크래시 지점을 게임 소스 단위로 특정할 수 있다.
//   ⚠출력은 16진이다(line/col 도 16진 — 10진으로 오독한 실사례 있음).
const RVA_PANIC_HOOK: usize = 0x2b16554; // 0.5.7 재핀: match_fn UNIQUE size527 동일 (0.5.6=0x2aac7b4)
const PANIC_STEAL: usize = 13;
const PANIC_PROLOGUE: [u8; 13] = [0x55, 0x41, 0x56, 0x56, 0x57, 0x53, 0x48, 0x81, 0xec, 0x80, 0x00, 0x00, 0x00];
static TRAMP_PANIC: AtomicUsize = AtomicUsize::new(0);
static PANIC_LOGGED: AtomicU64 = AtomicU64::new(0);
pub(crate) static PANIC_INSTALL: AtomicU64 = AtomicU64::new(0); // 0=미시도 1=OK 2=프롤로그불일치 3=base0 4=VirtualAlloc실패 5=VirtualProtect실패
pub(crate) static PANIC_SEEN: [AtomicU64; 13] = [const { AtomicU64::new(0) }; 13]; // 프롤로그 실측(불일치 시 원인 판별용)

extern "system" {
    fn VirtualAlloc(addr: usize, size: usize, ty: u32, prot: u32) -> usize;
    fn VirtualProtect(addr: usize, size: usize, prot: u32, old: *mut u32) -> i32;
    fn FlushInstructionCache(proc_: usize, addr: usize, size: usize) -> i32;
    fn GetCurrentProcess() -> usize;
}

unsafe extern "win64" fn hook_panic(rcx: usize, rdx: usize, r8: usize, r9: usize) -> usize {
    if PANIC_LOGGED.load(Ordering::Relaxed) < 3 && r8 >= 0x10000
        && CRASH_PATH_LEN.load(Ordering::Relaxed) != 0
    {
        let n = PANIC_LOGGED.fetch_add(1, Ordering::Relaxed);
        let fptr = core::ptr::read(r8 as *const usize);
        let flen = core::ptr::read((r8 + 8) as *const usize);
        let line = core::ptr::read((r8 + 16) as *const u32);
        let col  = core::ptr::read((r8 + 20) as *const u32);
        let mut buf = [0u8; 600];
        let mut p = 0usize;
        puts(&mut buf, &mut p, b"@@RUST PANIC #");
        hexb(&mut buf, &mut p, n);
        puts(&mut buf, &mut p, b" at ");
        if fptr >= 0x10000 && flen > 0 && flen < 300 {
            for i in 0..flen {
                let c = core::ptr::read((fptr + i) as *const u8);
                if p < buf.len() { buf[p] = c; p += 1; }
            }
        }
        puts(&mut buf, &mut p, b" line=");
        hexb(&mut buf, &mut p, line as u64);
        puts(&mut buf, &mut p, b" col=");
        hexb(&mut buf, &mut p, col as u64);
        puts(&mut buf, &mut p, b"  (line/col HEX)\r\n");
        let h = CreateFileW(core::ptr::addr_of!(CRASH_PATH_W) as *const u16,
                            0x40000000, 0x1 | 0x2, 0, 4, 0x80, 0);
        if h != usize::MAX && h != 0 {
            SetFilePointer(h, 0, core::ptr::null_mut(), 2);
            let mut w: u32 = 0;
            WriteFile(h, buf.as_ptr(), p as u32, &mut w, 0);
            CloseHandle(h);
        }
    }
    let stub = TRAMP_PANIC.load(Ordering::Relaxed);
    if stub == 0 { return 0; }
    let orig: unsafe extern "win64" fn(usize, usize, usize, usize) -> usize =
        core::mem::transmute(stub);
    orig(rcx, rdx, r8, r9)
}

unsafe fn install_panic_hook() {
    let base = GetModuleHandleW(core::ptr::null());
    if base == 0 { return; }
    let fn_addr = base + RVA_PANIC_HOOK;
    for (i, b) in PANIC_PROLOGUE.iter().enumerate() {
        if core::ptr::read((fn_addr + i) as *const u8) != *b { return; }  // 불일치 → 설치 포기
    }
    let stub = VirtualAlloc(0, 64, 0x1000 | 0x2000, 0x40);
    if stub == 0 { return; }
    let mut s = [0u8; 32];
    core::ptr::copy_nonoverlapping(fn_addr as *const u8, s.as_mut_ptr(), PANIC_STEAL);
    let mut q = PANIC_STEAL;
    s[q] = 0x49; s[q + 1] = 0xbb; q += 2;                 // movabs r11, ret
    let ret = fn_addr + PANIC_STEAL;
    core::ptr::copy_nonoverlapping(ret.to_le_bytes().as_ptr(), s.as_mut_ptr().add(q), 8);
    q += 8;
    s[q] = 0x41; s[q + 1] = 0xff; s[q + 2] = 0xe3; q += 3; // jmp r11
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, q);
    TRAMP_PANIC.store(stub, Ordering::Relaxed);

    let mut patch = [0x90u8; PANIC_STEAL];
    patch[0] = 0x48; patch[1] = 0xb8;                      // movabs rax, hook
    patch[2..10].copy_from_slice(&(hook_panic as usize).to_le_bytes());
    patch[10] = 0xff; patch[11] = 0xe0;                    // jmp rax
    let mut old: u32 = 0;
    if VirtualProtect(fn_addr, PANIC_STEAL, 0x40, &mut old) != 0 {
        core::ptr::copy_nonoverlapping(patch.as_ptr(), fn_addr as *mut u8, PANIC_STEAL);
        VirtualProtect(fn_addr, PANIC_STEAL, old, &mut old);
        FlushInstructionCache(GetCurrentProcess(), fn_addr, PANIC_STEAL);
        PANIC_INSTALL.store(1, Ordering::Relaxed);
    } else {
        PANIC_INSTALL.store(5, Ordering::Relaxed);
    }
}

/// config 로드 후 호출 — 설치 결과를 사람이 읽을 수 있게 남긴다.
pub(crate) fn report() {
    let st = PANIC_INSTALL.load(Ordering::Relaxed);
    let name = match st {
        1 => "OK",
        2 => "프롤로그 불일치(다른 모드가 선점했거나 RVA 오류)",
        3 => "exe base 0",
        4 => "VirtualAlloc 실패",
        5 => "VirtualProtect 실패",
        _ => "미시도",
    };
    let mut b = String::new();
    for i in 0..13 { b.push_str(&format!("{:02x}", PANIC_SEEN[i].load(Ordering::Relaxed))); }
    crate::config::dlog(&format!(
        "panic훅 {name} (rva={RVA_PANIC_HOOK:#x} 실측프롤로그={b}) | crash_path_len={} mod_base={:#x}",
        CRASH_PATH_LEN.load(Ordering::Relaxed), MOD_BASE.load(Ordering::Relaxed)));
}

// ══════════════════════════════════════════════════════════════════════════
// ★★모드 **자신**의 panic 훅 (2026-08-27) — 이것이 빠져 있었다.
//   게임 exe 의 rust_panic_with_hook 은 **게임 crate 의 panic 만** 통과한다.
//   모드 dll 은 자체 Rust 런타임(별도 std 인스턴스)이라 그 깔때기를 타지 않는다
//   ⟹ 팝업 그리기처럼 **모드 코드에서 난 panic** 은 게임 훅으로 절대 안 잡힌다.
//   실사고(0.5.7 champ_pos_lock): VEH 미포착 + 게임 panic훅 OK 인데도 미포착 = 이 경우.
pub(crate) fn install_mod_panic_hook() {
    let prev = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        // 파일에 직접 append (dlog 는 debug=0 이면 안 남으므로 쓰지 않는다)
        let msg = if let Some(s) = info.payload().downcast_ref::<&str>() { (*s).to_string() }
                  else if let Some(s) = info.payload().downcast_ref::<String>() { s.clone() }
                  else { String::from("(payload 형식 불명)") };
        let loc = info.location()
            .map(|l| format!("{}:{}:{}", l.file(), l.line(), l.column()))
            .unwrap_or_else(|| String::from("(위치 불명)"));
        if let Some(dir) = crate::mod_dir() {
            use std::io::Write;
            if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true)
                .open(format!("{dir}\\champ_pos_lock_crash.txt"))
            {
                let _ = writeln!(f, "@@MOD PANIC at {loc}
    msg: {msg}");
            }
        }
        prev(info);
    }));
}
