// mem_safety.rs — 메모리 안전 인프라: VEH 안전읽기/쓰기 + 크래시/패닉 로깅.
// 들어있는 것: rd_*/wr_*/safe_rd_*/lr_*/lw_*(VEH경유 read/write), readable/writable/ptr_ok/safe_copy,
//   seh_veh/seh_install(VEH), crash_*/panic_hook_install(크래시로거), cb_*(crash버퍼), perf_guard/bench_reads, now_ms.
// 언제 손대나: 안전읽기 경로(fast_read)·VEH·크래시로깅 변경 시. ⚠VEH핸들러 안엔 패닉유발코드 금지([[tfm2-mod-safety]]).

struct PerfGuard { idx: usize, t: Instant }

#[inline] fn perf_guard(idx: usize) -> Option<PerfGuard> {
    if PERF_ON.load(Ordering::Relaxed) { Some(PerfGuard { idx, t: Instant::now() }) } else { None }
}


#[repr(C)] #[derive(Default)]
struct MemBasicInfo { base: usize, alloc_base: usize, alloc_protect: u32, _p0: u32,
    region_size: usize, state: u32, protect: u32, typ: u32, _p1: u32 }

#[repr(C)] struct ExceptionPointers { rec: *mut ExceptionRecord, ctx: *mut core::ffi::c_void }

#[inline] unsafe fn lr_u64(a: usize) -> Option<u64> { let mut o=0u64; if pr_rd8(a, &mut o)!=0 {Some(o)} else {None} }

#[inline] unsafe fn lr_i32(a: usize) -> Option<i32> { let mut o=0u32; if pr_rd4(a, &mut o)!=0 {Some(o as i32)} else {None} }

#[inline] unsafe fn lr_u8(a: usize)  -> Option<u8>  { let mut o=0u8;  if pr_rd1(a, &mut o)!=0 {Some(o)} else {None} }

#[inline] unsafe fn lw_u64(a: usize, v: u64) -> bool { pr_wr8(a, v) != 0 }

#[inline] unsafe fn lw_u32(a: usize, v: u32) -> bool { pr_wr4(a, v) != 0 }

#[allow(dead_code)] #[inline] unsafe fn lw_u8(a: usize, v: u8) -> bool { pr_wr1(a, v) != 0 }

// ★쓰기 디스패처(rd_* 미러): fast_read=2=lockless VEH(최속,VQ0) / 0·1=writable VQ+raw(폴백). 둘다 AV-safe. 합법주소=동일write·불법=무쓰기 → writable가드와 동의미(valid sim서 비트동일).
unsafe fn wr_u64(a: usize, v: u64) -> bool { if a < 0x10000 { return false; } match FAST_READ.load(Ordering::Relaxed) { 2 => lw_u64(a, v), 3 => { std::ptr::write_unaligned(a as *mut u64, v); true }, _ => if writable(a, 8) { std::ptr::write_unaligned(a as *mut u64, v); true } else { false } } }

unsafe fn wr_u32(a: usize, v: u32) -> bool { if a < 0x10000 { return false; } match FAST_READ.load(Ordering::Relaxed) { 2 => lw_u32(a, v), 3 => { std::ptr::write_unaligned(a as *mut u32, v); true }, _ => if writable(a, 4) { std::ptr::write_unaligned(a as *mut u32, v); true } else { false } } }

#[allow(dead_code)]
unsafe fn wr_u8(a: usize, v: u8) -> bool { if a < 0x10000 { return false; } match FAST_READ.load(Ordering::Relaxed) { 2 => lw_u8(a, v), 3 => { std::ptr::write_unaligned(a as *mut u8, v); true }, _ => if writable(a, 1) { std::ptr::write_unaligned(a as *mut u8, v); true } else { false } } }

unsafe fn bench_reads() {
    if BENCH_DONE.swap(true, Ordering::Relaxed) { return; }
    let probe = core::ptr::addr_of!(SEH) as usize;   // 우리 static = 확실히 읽기가능
    let n = 300_000u64;
    let mut acc = 0u64;
    let t = Instant::now(); for _ in 0..n { if readable(probe,8) { acc = acc.wrapping_add(std::ptr::read_unaligned(probe as *const u64)); } } let vq = t.elapsed().as_nanos() as u64 / n;
    let t = Instant::now(); for _ in 0..n { acc = acc.wrapping_add(safe_rd_u64(probe).unwrap_or(0)); } let l1 = t.elapsed().as_nanos() as u64 / n;
    let t = Instant::now(); for _ in 0..n { acc = acc.wrapping_add(lr_u64(probe).unwrap_or(0)); } let l2 = t.elapsed().as_nanos() as u64 / n;
    write_named("readbench.txt", &format!("=== read path 벤치 (acc={} reads/path={}) ===\nVirtualQuery(level0): {} ns/read\nVEH spinlock(level1): {} ns/read\nVEH lockless(level2): {} ns/read\n", acc, n, vq, l1, l2));
}

extern "system" fn seh_veh(p: *mut ExceptionPointers) -> i32 {
    const CONTINUE_EXECUTION: i32 = -1; const CONTINUE_SEARCH: i32 = 0;
    unsafe {
        if p.is_null() { return CONTINUE_SEARCH; }
        let rec = (*p).rec;
        if rec.is_null() { return CONTINUE_SEARCH; }
        // ★[08-09] 0x80000001(STATUS_GUARD_PAGE_VIOLATION) 추가 — PAGE_GUARD 페이지를 프로브가 읽으면 OS는
        //   AV가 아니라 가드 위반을 던진다(가드비트는 예외 전달 전에 이미 소거됨). AV만 받던 구버전은 이게
        //   VEH를 통과해 미처리 예외로 즉사했다(08-09 데모화면 크래시: WER MOD+0x101c=pr_rd1_f, faultAddr=
        //   0xfe343f0000 스택 가드페이지). VirtualQuery 경로는 GUARD를 "읽기 불가"로 배제하므로(아래 readable)
        //   프로브 경로도 같은 의미(실패 반환)가 되도록 우리 RIP 한정으로 처리한다. 그 외 RIP는 종전대로 통과.
        let code = (*rec).code;
        if code != 0xC0000005 && code != 0x8000_0001 { return CONTINUE_SEARCH; }
        let ctx = (*p).ctx as usize;
        if ctx == 0 { return CONTINUE_SEARCH; }
        let rip = *((ctx + 0xF8) as *const u64) as usize;                           // CONTEXT.Rip@0xF8
        // ── lockless 읽기(level2): 고정 load주소 → land로 (상태/rsp복원 불요, 스택 무변경이라 land의 ret가 정상복귀) ──
        let land = if rip == core::ptr::addr_of!(pr_rd8_f) as usize { core::ptr::addr_of!(pr_rd8_l) as usize }
                   else if rip == core::ptr::addr_of!(pr_rd4_f) as usize { core::ptr::addr_of!(pr_rd4_l) as usize }
                   else if rip == core::ptr::addr_of!(pr_rd1_f) as usize { core::ptr::addr_of!(pr_rd1_l) as usize }
                   else if rip == core::ptr::addr_of!(pr_wr8_f) as usize { core::ptr::addr_of!(pr_wr8_l) as usize }   // ★B-3 쓰기 land(읽기와 동일 경로)
                   else if rip == core::ptr::addr_of!(pr_wr4_f) as usize { core::ptr::addr_of!(pr_wr4_l) as usize }
                   else if rip == core::ptr::addr_of!(pr_wr1_f) as usize { core::ptr::addr_of!(pr_wr1_l) as usize }
                   else { 0 };
        if land != 0 {
            if code == 0x8000_0001 { seh_rearm_guard(rec); }
            *((ctx + 0xF8) as *mut u64) = land as u64; return CONTINUE_EXECUTION;
        }
        // ── spinlock 읽기(level1): SEH[] 활성 + 우리스레드 + 우리 asm범위 ──
        let g = core::ptr::addr_of!(SEH) as *const u64;
        if *g.add(0) == 0 { return CONTINUE_SEARCH; }                                // inactive
        if *g.add(1) != GetCurrentThreadId() as u64 { return CONTINUE_SEARCH; }      // 다른 스레드
        if (rip as u64) < *g.add(5) || (rip as u64) >= *g.add(6) { return CONTINUE_SEARCH; }  // 범위 밖
        if code == 0x8000_0001 { seh_rearm_guard(rec); }
        *((ctx + 0xF8) as *mut u64) = *g.add(2);                                     // Rip → land
        *((ctx + 0x98) as *mut u64) = *g.add(3);                                     // Rsp 복원
        *((ctx + 0xA0) as *mut u64) = *g.add(4);                                     // Rbp 복원
        let gm = core::ptr::addr_of_mut!(SEH) as *mut u64;
        *gm.add(7) += 1;
        CONTINUE_EXECUTION
    }
}

// ★[08-09] 가드페이지 재무장 — 가드 위반은 예외 전달 전에 PAGE_GUARD 비트가 소거된다. 프로브가 소비한
//   가드를 되돌리지 않으면 그 페이지 원주인(대개 다른 스레드의 스택 성장 가드)이 망가져 나중에 그쪽이
//   0xc00000fd(STACK_OVERFLOW)로 죽는다. VEH 문맥 수칙 준수: VirtualQuery/VirtualProtect는 raw syscall
//   (할당·락·패닉 없음). COMMIT 페이지가 아니면 손대지 않는다(그 사이 decommit된 경우).
unsafe fn seh_rearm_guard(rec: *mut ExceptionRecord) {
    if (*rec)._np < 2 { return; }
    let fa = (*rec)._params[1];
    if fa < 0x10000 || fa >= 0x0001_0000_0000_0000 { return; }
    let mut mbi = MemBasicInfo::default();
    if VirtualQuery(fa as *const _, &mut mbi, core::mem::size_of::<MemBasicInfo>()) == 0 { return; }
    if mbi.state != 0x1000 { return; }   // MEM_COMMIT만
    let mut old = 0u32;
    VirtualProtect(fa & !0xfff, 0x1000, mbi.protect | 0x100, &mut old);   // PAGE_GUARD 재부여
}

fn seh_install() { if SEH_INSTALLED.swap(true, Ordering::Relaxed) { return; } unsafe { AddVectoredExceptionHandler(1, seh_veh); } }


#[inline] fn cb_put(buf: &mut [u8; 4096], pos: &mut usize, b: u8) { if *pos < 4096 { buf[*pos] = b; *pos += 1; } }

fn cb_str(buf: &mut [u8; 4096], pos: &mut usize, s: &[u8]) { for &c in s { cb_put(buf, pos, c); } }

fn cb_hex(buf: &mut [u8; 4096], pos: &mut usize, v: u64) {
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
    let h = CreateFileW(path, 0x40000000, 3, 0, 4, 0x80, 0);   // GENERIC_WRITE, SHARE_RW, OPEN_ALWAYS
    if h == -1 || h == 0 { return; }
    SetFilePointer(h, 0, 0, 2);   // FILE_END = append
    let mut wr = 0u32;
    WriteFile(h, data.as_ptr(), data.len() as u32, &mut wr, 0);
    CloseHandle(h);
}

extern "system" fn crash_filter(p: *mut ExceptionPointers) -> i32 {
    const CONTINUE_SEARCH: i32 = 0;
    unsafe {
        if !CRASH_LOGGED.swap(true, Ordering::Relaxed) && !p.is_null() {
            let rec = (*p).rec; let ctx = (*p).ctx as usize;
            if !rec.is_null() && ctx != 0 {
                let code = (*rec).code as u64;
                let excaddr = (*rec)._addr as u64;
                let np = (*rec)._np as usize;
                let access = if np >= 1 { (*rec)._params[0] as u64 } else { 0 };
                let fault = if np >= 2 { (*rec)._params[1] as u64 } else { 0 };
                let rip = *((ctx + 0xF8) as *const u64);
                let rsp = *((ctx + 0x98) as *const u64);
                let exe = exe_base() as u64;
                let modb = CRASH_MOD_BASE.load(Ordering::Relaxed);
                let mut buf = [0u8; 4096]; let mut pos = 0usize;
                cb_str(&mut buf, &mut pos, b"\n=== CRASH (tfm2_ai_adjust) pid="); cb_hex(&mut buf, &mut pos, GetCurrentProcessId() as u64);
                cb_str(&mut buf, &mut pos, b" ===\ncode="); cb_hex(&mut buf, &mut pos, code);
                cb_str(&mut buf, &mut pos, b"\nRIP="); cb_hex(&mut buf, &mut pos, rip);
                if rip.wrapping_sub(exe) < 0x8000000 { cb_str(&mut buf, &mut pos, b" = exe+"); cb_hex(&mut buf, &mut pos, rip.wrapping_sub(exe)); }
                // ★[07-30] RIP 가 exe/MOD 어디에도 안 걸리면 = 동적 RWX 스텁(우리 트램폴린)일 수 있다 → 인벤토리 대조.
                //   WER 가 `Faulting module name: unknown` 으로만 남기던 크래시(07-30 실측 2건)를 여기서 훅 단위로 특정한다.
                //   stub_lookup 은 고정배열 순회(할당·락 없음)라 크래시 문맥에서 안전.
                if let Some((tag, off)) = stub_lookup(rip as usize) {
                    cb_str(&mut buf, &mut pos, b" = STUB(tag="); cb_hex(&mut buf, &mut pos, tag as u64);
                    cb_str(&mut buf, &mut pos, b")+"); cb_hex(&mut buf, &mut pos, off as u64);
                }
                if modb != 0 && rip.wrapping_sub(modb) < 0x2000000 { cb_str(&mut buf, &mut pos, b" = MOD+"); cb_hex(&mut buf, &mut pos, rip.wrapping_sub(modb)); }
                cb_str(&mut buf, &mut pos, b"\nexcAddr="); cb_hex(&mut buf, &mut pos, excaddr);
                cb_str(&mut buf, &mut pos, b"  access="); cb_hex(&mut buf, &mut pos, access);
                cb_str(&mut buf, &mut pos, b" faultAddr="); cb_hex(&mut buf, &mut pos, fault);
                cb_str(&mut buf, &mut pos, b"\nexe_base="); cb_hex(&mut buf, &mut pos, exe);
                cb_str(&mut buf, &mut pos, b"  mod_base="); cb_hex(&mut buf, &mut pos, modb);
                cb_str(&mut buf, &mut pos, b"\nstack ret-addrs(exe+ / MOD+):\n");
                let mut i = 0usize;
                while i < 0x600 && pos < 3900 {
                    if let Some(v) = lr_u64(rsp.wrapping_add(i as u64) as usize) {
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
        // 이전 필터(게임/타모드)로 체인 — 우린 로깅만, 원래 크래시 처리 보존
        let prev = CRASH_PREV.load(Ordering::Relaxed);
        if prev != 0 { let f: extern "system" fn(*mut ExceptionPointers) -> i32 = core::mem::transmute(prev as usize); return f(p); }
        CONTINUE_SEARCH
    }
}

fn panic_hook_install() {
    let prev = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let n = PANIC_N.fetch_add(1, Ordering::Relaxed);
        if n < 50 {
            let loc = info.location()
                .map(|l| format!("{}:{}:{}", l.file(), l.line(), l.column()))
                .unwrap_or_else(|| "?".into());
            let msg = info.payload().downcast_ref::<&str>().map(|s| (*s).to_string())
                .or_else(|| info.payload().downcast_ref::<String>().cloned())
                .unwrap_or_else(|| "?".into());
            let tid = unsafe { GetCurrentThreadId() };
            if let Some(p) = pth("panic_log.txt") {
                if let Ok(mut f) = fs::OpenOptions::new().create(true).append(true).open(&p) {
                    let _ = write!(f, "[panic #{} tid={}] {} @ {}\n", n + 1, tid, msg, loc);
                }
            }
        }
        prev(info);
    }));
}

unsafe fn crash_install() {
    if CRASH_INSTALLED.swap(true, Ordering::Relaxed) { return; }
    let mut h: HMODULE = 0;
    if GetModuleHandleExW(0x4|0x2, crash_filter as *const () as *const u16, &mut h) != 0 && h != 0 {
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

#[inline(never)]
unsafe fn safe_copy(dst: *mut u8, src: *const u8, len: usize) -> bool {
    if !SEH_INSTALLED.load(Ordering::Relaxed) { return false; }
    while SEH_BUSY.swap(true, Ordering::Acquire) { core::hint::spin_loop(); }
    let g = core::ptr::addr_of_mut!(SEH) as *mut u64;
    *g.add(1) = GetCurrentThreadId() as u64;
    let ok: u64;
    core::arch::asm!(
        "lea rax, [rip + 200f]", "mov [{g} + 40], rax",   // code_lo = SEH[5]
        "lea rax, [rip + 201f]", "mov [{g} + 48], rax",   // code_hi = SEH[6]
        "lea rax, [rip + 202f]", "mov [{g} + 16], rax",   // land_rip = SEH[2]
        "mov [{g} + 24], rsp", "mov [{g} + 32], rbp",      // land_rsp/rbp = SEH[3]/[4]
        "mov qword ptr [{g} + 0], 1", "cld",               // active
        "200:", "rep movsb", "201:", "mov {ok}, 1", "jmp 203f",
        "202:", "mov {ok}, 0", "203:", "mov qword ptr [{g} + 0], 0",   // inactive
        g = in(reg) g, ok = out(reg) ok,
        inout("rcx") len => _, inout("rdi") dst => _, inout("rsi") src => _, out("rax") _,
    );
    SEH_BUSY.store(false, Ordering::Release);
    ok != 0
}

#[inline] unsafe fn safe_rd_u64(a: usize) -> Option<u64> { let mut b=[0u8;8]; if safe_copy(b.as_mut_ptr(), a as *const u8, 8){Some(u64::from_le_bytes(b))}else{None} }

#[inline] unsafe fn safe_rd_i32(a: usize) -> Option<i32> { let mut b=[0u8;4]; if safe_copy(b.as_mut_ptr(), a as *const u8, 4){Some(i32::from_le_bytes(b))}else{None} }

#[inline] unsafe fn safe_rd_u8(a: usize) -> Option<u8> { let mut b=[0u8;1]; if safe_copy(b.as_mut_ptr(), a as *const u8, 1){Some(b[0])}else{None} }


// ⚠ readable/writable는 매 호출 VirtualQuery (point-in-time 검증). 이걸 영역캐시로 가속하려던 시도는
//   ★실패(2026-06-21): commit 영역 [s,e)를 캐시했다가 게임이 그 영역 sub-page를 나중에 decommit(힙
//   sub-block free)하면 캐시가 여전히 [s,e) 전체를 readable로 보증 → judge가 decommit된 주소 읽기 → AV.
//   경기 시작 직후(alloc/free 격렬, ready_ticks~31) 크래시 실측. ⟹ VirtualQuery 결과는 시간을 가로질러
//   캐시 불가(thread_local/Mutex 무관). 가속은 캐시 대신 '읽기 횟수 자체'를 줄여서(객체범위 1회검증+raw읽기,
//   같은 동기 스코프) 달성 — 예: dd7_slot_a8. readable/writable 본체는 원본(매콜 VirtualQuery) 유지.
unsafe fn readable(addr: usize, len: usize) -> bool {
    if addr < 0x10000 || len == 0 { return false; }
    // ★[07-16 최적화] fast_read=2 + fast_guard=1: VirtualQuery(syscall) 대신 VEH 프로브읽기(수ns).
    //   페이지 단위 커밋이므로 시작바이트 + 걸치는 각 페이지 경계만 프로브하면 [addr, addr+len) 전체 보증.
    //   위 캐시금지 사유(decommit TOCTOU)와 무관 — 프로브는 캐시가 아니라 그 시점 실측(VQ와 동일 시제).
    //   노출등가: readable() 직후 같은 주소를 rd_*(level2=raw+VEH)로 읽는 패턴이라 프로브=같은 접근.
    if FAST_READ.load(Ordering::Relaxed) == 2 && FAST_GUARD.load(Ordering::Relaxed) != 0
        && SEH_INSTALLED.load(Ordering::Relaxed) {
        if lr_u8(addr).is_none() { return false; }
        let last = addr + len - 1;
        let mut p = (addr | 0xfff) + 1;   // 다음 페이지 경계
        while p <= last {
            if lr_u8(p).is_none() { return false; }
            p += 0x1000;
        }
        return true;
    }
    let mut mbi = MemBasicInfo::default();
    if VirtualQuery(addr as *const _, &mut mbi, core::mem::size_of::<MemBasicInfo>()) == 0 { return false; }
    const COMMIT: u32 = 0x1000; const RD: u32 = 0x02|0x04|0x20|0x40; const GUARD: u32 = 0x01|0x100;
    if mbi.state != COMMIT || mbi.protect & GUARD != 0 || mbi.protect & RD == 0 { return false; }
    addr + len <= mbi.base + mbi.region_size
}

// ★쓰기가능 검사(대체모드 RNG 되쓰기 안전가드): PAGE_READWRITE/WRITECOPY/EXECUTE_READWRITE/EXECUTE_WRITECOPY.
//   [07-16 최적화] fast path(fast_read=2+fast_guard): 읽기프로브로 대체. writable() 실사용처는 전부
//   judge 출력버퍼(out param) 가드 = 게임이 직후 직접 쓰는 heap/stack RW 페이지 → "readable인데 readonly"는
//   실전 부재. 실효 방어(garbage 포인터 차단)는 읽기프로브로 동등. 문제시 fast_guard=0 롤백.
unsafe fn writable(addr: usize, len: usize) -> bool {
    if addr < 0x10000 || len == 0 { return false; }
    if FAST_READ.load(Ordering::Relaxed) == 2 && FAST_GUARD.load(Ordering::Relaxed) != 0
        && SEH_INSTALLED.load(Ordering::Relaxed) {
        if lr_u8(addr).is_none() { return false; }
        let last = addr + len - 1;
        let mut p = (addr | 0xfff) + 1;
        while p <= last {
            if lr_u8(p).is_none() { return false; }
            p += 0x1000;
        }
        return true;
    }
    let mut mbi = MemBasicInfo::default();
    if VirtualQuery(addr as *const _, &mut mbi, core::mem::size_of::<MemBasicInfo>()) == 0 { return false; }
    const COMMIT: u32 = 0x1000; const WR: u32 = 0x04|0x08|0x40|0x80; const GUARD: u32 = 0x01|0x100;
    if mbi.state != COMMIT || mbi.protect & GUARD != 0 || mbi.protect & WR == 0 { return false; }
    addr + len <= mbi.base + mbi.region_size
}

// ★fast_read: 2=lockless VEH(최속) / 1=spinlock VEH(검증본) / 0=VirtualQuery. 셋다 AV-safe·동일결과.
#[inline] unsafe fn rd_u64(a: usize) -> Option<u64> { if a < 0x10000 { return None; } match FAST_READ.load(Ordering::Relaxed) { 2 => lr_u64(a), 3 => Some(std::ptr::read_unaligned(a as *const u64)), 1 => safe_rd_u64(a), _ => if readable(a,8){Some(std::ptr::read_unaligned(a as *const u64))}else{None} } }

#[inline] unsafe fn rd_i64(a: usize) -> Option<i64> { if a < 0x10000 { return None; } match FAST_READ.load(Ordering::Relaxed) { 2 => lr_u64(a).map(|v| v as i64), 3 => Some(std::ptr::read_unaligned(a as *const i64)), 1 => safe_rd_u64(a).map(|v| v as i64), _ => if readable(a,8){Some(std::ptr::read_unaligned(a as *const i64))}else{None} } }

#[inline] unsafe fn rd_i32(a: usize) -> Option<i32> { if a < 0x10000 { return None; } match FAST_READ.load(Ordering::Relaxed) { 2 => lr_i32(a), 3 => Some(std::ptr::read_unaligned(a as *const i32)), 1 => safe_rd_i32(a), _ => if readable(a,4){Some(std::ptr::read_unaligned(a as *const i32))}else{None} } }

#[inline] unsafe fn rd_u8(a: usize) -> u8 { if a < 0x10000 { return 0; } match FAST_READ.load(Ordering::Relaxed) { 2 => lr_u8(a).unwrap_or(0), 3 => std::ptr::read_unaligned(a as *const u8), 1 => safe_rd_u8(a).unwrap_or(0), _ => if readable(a,1){std::ptr::read_unaligned(a as *const u8)}else{0} } }

#[inline] unsafe fn rd_u32(a: usize) -> u32 { if a < 0x10000 { return 0; } match FAST_READ.load(Ordering::Relaxed) { 2 => lr_i32(a).map(|v| v as u32).unwrap_or(0), 3 => std::ptr::read_unaligned(a as *const u32), 1 => safe_rd_i32(a).map(|v| v as u32).unwrap_or(0), _ => if readable(a,4){std::ptr::read_unaligned(a as *const u32)}else{0} } }   // ★fast_read=2 분기 누락 수정(RNG 64워드/draw VQ→lockless): 단일 최대 효과

// input ptr(rcx): key[8]@+0, counter u64@+0x20, nonce u64@+0x28. 4블록(counter+0..3) → out[64].
// 버퍼 워드순서는 SIMD변형에 따라 다를 수 있음 → implement-and-diff로 확정(초기추정=block-sequential).
// 유저공간 포인터로 그럴듯한 범위 (오버플로/쓰레기 산술 방지)
#[inline] fn ptr_ok(a: usize) -> bool { a >= 0x10000 && a < 0x0001_0000_0000_0000 }

// ★[07-15 하드닝] 실행가능 코드페이지 검사 — shadow-call(game 함수 CALL) 대상 함수포인터 유효성.
//   PAGE_EXECUTE_*(0x10/0x20/0x40/0x80) 커밋페이지만 true. garbage/데이터 포인터로의 game함수 CALL은
//   하드웨어 AV(catch_unwind 불가)이므로 CALL 전에 이걸로 사전차단. (readable/writable와 동일 VQ 구조.)
unsafe fn executable(addr: usize, len: usize) -> bool {
    if addr < 0x10000 || len == 0 { return false; }
    let mut mbi = MemBasicInfo::default();
    if VirtualQuery(addr as *const _, &mut mbi, core::mem::size_of::<MemBasicInfo>()) == 0 { return false; }
    const COMMIT: u32 = 0x1000; const EXEC: u32 = 0x10|0x20|0x40|0x80; const GUARD: u32 = 0x01|0x100;
    if mbi.state != COMMIT || mbi.protect & GUARD != 0 || mbi.protect & EXEC == 0 { return false; }
    addr + len <= mbi.base + mbi.region_size
}

// ★게임 exe .text 산술범위(RVA 0x1000..0x30a61ff, 0.5.3) — 순수 getter-dispatch 핫패스용(VQ 없이 저렴).
//   유효 게터 슬롯은 전부 이 안. 밖이면 stale/garbage 함수포인터 → 순수 dispatch가 기본값 반환하도록 게이트.
//   ⚠패치 시 갱신: TeamfightManager2.exe PE의 .text VirtualSize 끝(= va 0x1000 + vsz).
const TEXT_END_RVA: usize = 0x32697ff;   // ★0.5.6(2026-08-20 실측 va 0x1000+vsz 0x3268800−1 — 0.5.4/0.5.5 미갱신이었음·진단 분류 전용). 구 ★0.5.3(was 0.5.2 0x2c087ff). PE .text va=0x1000 vsz=0x30a5200 → vsz_end=0x30a6200 실측(capstone/pefile 2026-07-29). ⚠구값을 두면 .text 후반 40% 게터가 전부 "범위밖"으로 차단돼 재현이 조용히 기본값으로 퇴화한다.
#[inline] unsafe fn in_text(a: usize) -> bool {
    let b = exe_base();
    b != 0 && a > b + 0x1000 && a < b + TEXT_END_RVA
}
// ★실제 game함수 CALL(transmute→CALL) 직전 검증: .text 범위 + 실행가능 커밋페이지(2중). 실패 시 CALL 금지.
// ★[07-19 최적화] 검증통과 코드포인터 캐시: exe 이미지 .text 페이지 보호속성은 프로세스 수명 동안 불변
//   (2026-06-21 힙 영역캐시 실패 사유였던 decommit TOCTOU가 이미지 섹션엔 원천 부재 — 이미지 페이지는 모듈
//   로드~종료까지 커밋 유지) → 한 번 executable() 통과한 주소는 영구 유효. 4엔트리 직접매핑 캐시로 재검증
//   VirtualQuery(syscall, fast_guard 무관 상시)를 제거. 히트=동일 true, 미스=기존 2중검증 그대로 → 판정 비트동일.
static CODE_OK_SEEN: [AtomicUsize; 4] = [const { AtomicUsize::new(0) }; 4];
#[inline] unsafe fn code_ptr_ok(a: usize) -> bool {
    if !in_text(a) { return false; }
    let idx = (a >> 4) & 3;
    if CODE_OK_SEEN[idx].load(Ordering::Relaxed) == a { return true; }
    if executable(a, 1) { CODE_OK_SEEN[idx].store(a, Ordering::Relaxed); true } else { false }
}

// p가 가리키는 곳이 정확히 nb 문자열(+null종료)인가
// 힙할당 없이 null종료 문자열을 buf에 읽음 → 길이
// p의 null종료 문자열을 최대 31바이트 읽어 String으로

fn now_ms() -> u64 { SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0) }

// ═══════════ [07-16] 행(hang) 진단 워치독 — "일정 넘기다 멈춤(크래시 아님)" 원인 규명 ═══════════
// 두 모드 구분:
//   STALL   = judge 하트비트 정지 + CPU 바쁨/훅내부 갇힘 → 스레드가 어딘가 루프/데드락에 갇힘.
//             → 전 스레드 RIP+스택 리턴주소 덤프(exe+RVA/MOD+off)로 갇힌 위치 특정.
//   RUNAWAY = 하트비트가 고속(백그라운드 시뮬 수준)으로 "계속" 진행 = 연산이 안 끝나는 상태.
//             → 경기가 안 끝나는 교착(예: 모드 판단값으로 아무도 넥서스를 못/안 깸) 가능성. 발생율·사이트 분포 덤프.
// 워치독은 독립 스레드(게임 루프와 무관)라 게임이 완전히 멈춰도 동작. 덤프 = hang_diag.txt (LOG_ON 무관).
// 비용: judge 훅당 ~30ns(하트비트+in-flight 마킹), 워치독 1초당 원자읽기 몇 개 = 무시가능. hang_diag=0으로 OFF.
extern "system" {
    fn CreateToolhelp32Snapshot(flags: u32, pid: u32) -> isize;
    fn Thread32First(snap: isize, entry: *mut ThreadEntry32) -> i32;
    fn Thread32Next(snap: isize, entry: *mut ThreadEntry32) -> i32;
    fn OpenThread(access: u32, inherit: i32, tid: u32) -> isize;
    fn SuspendThread(h: isize) -> u32;
    fn ResumeThread(h: isize) -> u32;
    fn GetThreadContext(h: isize, ctx: *mut u8) -> i32;
    fn GetThreadTimes(h: isize, c: *mut u64, e: *mut u64, k: *mut u64, u: *mut u64) -> i32;
    fn GetTickCount64() -> u64;
}
#[repr(C)] struct ThreadEntry32 { size: u32, usage: u32, tid: u32, owner_pid: u32, base_pri: i32, delta_pri: i32, flags: u32 }
#[repr(C, align(16))] struct CtxBuf([u8; 1360]);   // x64 CONTEXT(1232B) 여유포함, 16정렬 필수

#[inline] fn tick_ms() -> u64 { unsafe { GetTickCount64() } }   // 단조 ms(벽시계 점프 무관, ~5ns)

// judge in-flight 마킹: 훅 진입시 (site, tid, 진입시각) 기록, 이탈시 클리어(Drop=패닉 unwind에도 클리어).
// 슬롯 = 스레드당 1개 고정(64슬롯 & 63 — rayon ~8-16스레드라 충돌 실전 무). 워치독이 "N초 넘게 안 나온 훅" 탐지.
struct JudgeMark { slot: usize }
impl Drop for JudgeMark { fn drop(&mut self) { if self.slot < 64 { INFLIGHT_SITE[self.slot].store(0, Ordering::Relaxed); } } }
#[inline] fn judge_mark(site: u64) -> JudgeMark {
    // ★[07-19 최적화] 진단 소비자(hang_watchdog=HANG_DIAG·prof_tick=ADV_PROF) 둘 다 OFF면 마킹 전체 skip —
    //   배포 cfg(둘 다 0)서 매 judge 콜 공유 atomic fetch_add 2회(rayon 멀티스레드 캐시라인 바운싱, 06-23 캠페인
    //   _RAW 카운터 제거와 동일 패턴)가 아무도 안 읽는 값 갱신이었음. 게이트=원자 load 2회(cfg 핫리로드 즉시 반영).
    //   slot=usize::MAX → Drop no-op(기존 slot<64 가드 재사용). 진단전용 상태라 판단 출력 비트동일.
    if HANG_DIAG.load(Ordering::Relaxed) == 0 && ADV_PROF.load(Ordering::Relaxed) == 0 {
        return JudgeMark { slot: usize::MAX };
    }
    HB_JUDGE.fetch_add(1, Ordering::Relaxed);
    if (site as usize) < 5 { SITE_CNT[site as usize].fetch_add(1, Ordering::Relaxed); }   // 활동창 프로파일 사이트분포
    let slot = INFLIGHT_SLOT.with(|c| {
        let mut s = c.get();
        if s == usize::MAX { s = INFLIGHT_NEXT.fetch_add(1, Ordering::Relaxed) & 63; c.set(s); }
        s
    });
    INFLIGHT_SITE[slot].store(site, Ordering::Relaxed);
    INFLIGHT_TS[slot].store(tick_ms(), Ordering::Relaxed);
    INFLIGHT_TID[slot].store(unsafe { GetCurrentThreadId() } as u64, Ordering::Relaxed);
    JudgeMark { slot }
}
const HANG_SITE_NAMES: [&str; 5] = ["?", "retreat", "recall(fc59a0)", "condgate", "movepri"];

fn hang_watchdog_install() {
    if HANG_WD_STARTED.swap(true, Ordering::Relaxed) { return; }
    let _ = std::thread::Builder::new().name("aiadj_hangwd".into()).spawn(|| {
        // 워치독 자체 패닉은 삼키고 계속(진단자가 게임을 해치면 안 됨)
        let mut st = HangSt { last_hb: 0, frozen_ms: 0, run_ms: 0, cpu_prev: proc_cpu_100ns(), n_stall: 0, n_run: 0 };
        let mut ps = ProfSt::new();
        let mut acc_ms: u64 = 0;
        loop {
            // 100ms 틱: 프로파일러 샘플링 / 1000ms마다: 행 감지
            std::thread::sleep(std::time::Duration::from_millis(100));
            let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| prof_tick(&mut ps)));
            acc_ms += 100;
            if acc_ms >= 1000 {
                acc_ms = 0;
                let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| hang_watchdog_step(&mut st)));
            }
        }
    });
}

// ═══════════ [07-16] 활동창 샘플링 프로파일러 — "일정넘김 등 큰 연산이 정확히 어디서 오래 걸리나" ═══════════
// 원리: CPU가 바빠지거나 judge 콜이 돌기 시작하면 "활동창" 시작 → 100ms마다 CPU를 실제 소모중인 스레드만
//   RIP 샘플(짧은 suspend~resume, ~수십µs) → exe+RVA 1KB 버킷 히스토그램 누적 → 활동이 2초 멎으면 창 종료,
//   adv_prof.txt에 요약(소요·CPU·judge 콜분포·TOP 코드위치 %). RVA는 기드라로 바로 함수 특정 가능.
// 항상 켜두는 용도(adv_prof=1 기본): 유휴시 100ms마다 원자읽기 몇개=무비용, 활동중 <1% 코어.
// ⚠배포(zip)용 cfg에선 adv_prof=0으로 끌 것(진단은 개발자용).
struct PThread { tid: u32, h: isize, t_last: u64 }
struct ProfSt {
    active: bool, win_start: u64, idle_ticks: u32, n_win: u32,
    hb_prev: u64, hb_win0: u64, site0: [u64; 5],
    cpu_prev: u64, cpu_win0: u64,
    samples: u64, busy_acc: u64,
    hist_exe: HashMap<u64, u64>, hist_mod: HashMap<u64, u64>, other: u64,
    threads: Vec<PThread>, last_refresh: u64,
    seg_start: u64, seg_hb0: u64, seg_cpu0: u64, seg_site0: [u64; 5], n_seg: u32,   // ★중간 스냅샷용 구간 경계
}
impl ProfSt {
    fn new() -> Self { ProfSt { active: false, win_start: 0, idle_ticks: 0, n_win: 0,
        hb_prev: 0, hb_win0: 0, site0: [0; 5], cpu_prev: proc_cpu_100ns(), cpu_win0: 0,
        samples: 0, busy_acc: 0, hist_exe: HashMap::new(), hist_mod: HashMap::new(), other: 0,
        threads: Vec::new(), last_refresh: 0,
        seg_start: 0, seg_hb0: 0, seg_cpu0: 0, seg_site0: [0; 5], n_seg: 0 } }
}

fn prof_refresh_threads(ps: &mut ProfSt) {
    const TH32CS_SNAPTHREAD: u32 = 0x4;
    const ACCESS: u32 = 0x0002 | 0x0008 | 0x0040;
    unsafe {
        // 죽은 스레드 정리(시간조회 실패 or 종료시간 존재)
        ps.threads.retain(|t| {
            let (mut c, mut e, mut k, mut u) = (0u64, 0u64, 0u64, 0u64);
            let ok = GetThreadTimes(t.h, &mut c, &mut e, &mut k, &mut u) != 0 && e == 0;
            if !ok { CloseHandle(t.h); }
            ok
        });
        let my_pid = GetCurrentProcessId();
        let my_tid = GetCurrentThreadId();
        let snap = CreateToolhelp32Snapshot(TH32CS_SNAPTHREAD, 0);
        if snap == -1 || snap == 0 { return; }
        let mut te = ThreadEntry32 { size: core::mem::size_of::<ThreadEntry32>() as u32, usage: 0, tid: 0, owner_pid: 0, base_pri: 0, delta_pri: 0, flags: 0 };
        if Thread32First(snap, &mut te) != 0 {
            loop {
                if te.owner_pid == my_pid && te.tid != my_tid && ps.threads.len() < 128
                    && !ps.threads.iter().any(|t| t.tid == te.tid) {
                    let h = OpenThread(ACCESS, 0, te.tid);
                    if h != 0 && h != -1 {
                        let (mut c, mut e, mut k, mut u) = (0u64, 0u64, 0u64, 0u64);
                        let t0 = if GetThreadTimes(h, &mut c, &mut e, &mut k, &mut u) != 0 { k.wrapping_add(u) } else { 0 };
                        ps.threads.push(PThread { tid: te.tid, h, t_last: t0 });
                    }
                }
                if Thread32Next(snap, &mut te) == 0 { break; }
            }
        }
        CloseHandle(snap);
    }
}

fn prof_tick(ps: &mut ProfSt) {
    if ADV_PROF.load(Ordering::Relaxed) == 0 {
        if ps.active { ps.active = false; ps.hist_exe.clear(); ps.hist_mod.clear(); }
        return;
    }
    let now = tick_ms();
    let hb = HB_JUDGE.load(Ordering::Relaxed);
    let cpu = proc_cpu_100ns();
    let cpu_d = cpu.saturating_sub(ps.cpu_prev);   // 100ns 단위, 이번 100ms틱 프로세스 CPU
    ps.cpu_prev = cpu;
    let hb_moved = hb != ps.hb_prev;
    ps.hb_prev = hb;
    let busy = hb_moved || cpu_d >= 400_000;   // 100ms틱에 40ms(0.4코어) 이상 소모
    if !ps.active {
        if !busy { return; }
        // ── 활동창 시작 ──
        ps.active = true; ps.win_start = now; ps.idle_ticks = 0; ps.n_seg = 0;
        ps.hb_win0 = hb; ps.cpu_win0 = cpu;
        for i in 0..5 { ps.site0[i] = SITE_CNT[i].load(Ordering::Relaxed); }
        prof_seg_reset(ps, now);   // 구간 경계 초기화(samples/hist도 리셋)
        prof_refresh_threads(ps); ps.last_refresh = now;
    }
    if busy { ps.idle_ticks = 0; } else {
        ps.idle_ticks += 1;
        if ps.idle_ticks >= 20 {   // 2초 무활동 → 창 종료
            let dur = now.saturating_sub(ps.win_start).saturating_sub(2000);
            prof_win_end(ps, dur, now);
            ps.active = false;
            return;
        }
        return;   // 유휴틱은 샘플 안 함
    }
    // ── 샘플: CPU 소모한 스레드만 RIP 채집 ──
    if now.saturating_sub(ps.last_refresh) >= 2000 { prof_refresh_threads(ps); ps.last_refresh = now; }
    let exe = unsafe { exe_base() } as u64;
    let modb = CRASH_MOD_BASE.load(Ordering::Relaxed);
    ps.samples += 1;
    unsafe {
        for t in ps.threads.iter_mut() {
            let (mut c, mut e, mut k, mut u) = (0u64, 0u64, 0u64, 0u64);
            if GetThreadTimes(t.h, &mut c, &mut e, &mut k, &mut u) == 0 { continue; }
            let tt = k.wrapping_add(u);
            let d = tt.saturating_sub(t.t_last);
            t.t_last = tt;
            if d == 0 { continue; }   // 이번 틱에 CPU 안 씀(대기중) → 샘플 제외
            ps.busy_acc += 1;
            if SuspendThread(t.h) == u32::MAX { continue; }
            let mut ctx = CtxBuf([0u8; 1360]);
            *(ctx.0.as_mut_ptr().add(0x30) as *mut u32) = 0x10_0001;
            let rip = if GetThreadContext(t.h, ctx.0.as_mut_ptr()) != 0 {
                *(ctx.0.as_ptr().add(0xF8) as *const u64)
            } else { 0 };
            ResumeThread(t.h);
            // resume 후 히스토그램 갱신(suspend 중 할당금지 준수)
            if rip == 0 { continue; }
            let de = rip.wrapping_sub(exe); let dm = rip.wrapping_sub(modb);
            if de < 0x8000000 {
                if ps.hist_exe.len() < 8192 { *ps.hist_exe.entry(de & !0x3ff).or_insert(0) += 1; }
            } else if modb != 0 && dm < 0x2000000 {
                if ps.hist_mod.len() < 4096 { *ps.hist_mod.entry(dm & !0x3ff).or_insert(0) += 1; }
            } else { ps.other += 1; }
        }
    }
    // ★[07-16] 중간 스냅샷: 활동이 seg 간격 이상 지속되면(=끝나지 않는 창) 구간 덤프 후 히스토그램 리셋.
    let seg_ms = ADV_PROF_SEG.load(Ordering::Relaxed).max(3000) as u64;
    if now.saturating_sub(ps.seg_start) >= seg_ms && ps.n_win < 100 {
        ps.n_seg += 1;
        let n = ps.n_win + 1;  // 아직 win_end 전이므로 다음 창번호로 표기
        let seg = ps.n_seg;
        prof_segment(ps, &format!("활동창 #{} 구간 #{} (진행중)", n, seg), now);
    }
}

// 공용 포맷터: 헤더 + judge분포 + 히스토그램. 창종료·구간스냅샷 공용. hist는 넘긴 구간분(reset 전).
fn prof_report(ps: &ProfSt, header: &str, dur_ms: u64, hb_d: u64, cpu_ms: u64, site_d: &[u64; 5]) {
    let mut s = format!("\n{}\n", header);
    s.push_str(&format!("  judge콜: 총{} ({}콜/s) — retreat {} / recall {} / condgate {} / movepri {}\n",
        hb_d, if dur_ms > 0 { hb_d * 1000 / dur_ms } else { 0 }, site_d[1], site_d[2], site_d[3], site_d[4]));
    let tot: u64 = ps.hist_exe.values().sum::<u64>() + ps.hist_mod.values().sum::<u64>() + ps.other;
    s.push_str(&format!("  샘플 {}틱 · 평균 활성스레드 {:.1}개 · RIP히트 {} · CPU {:.1}코어·초\n",
        ps.samples, if ps.samples > 0 { ps.busy_acc as f64 / ps.samples as f64 } else { 0.0 }, tot, cpu_ms as f64 / 1000.0));
    if tot > 0 {
        let mut v: Vec<(u64, u64)> = ps.hist_exe.iter().map(|(k, c)| (*k, *c)).collect();
        v.sort_by(|a, b| b.1.cmp(&a.1));
        s.push_str("  TOP 게임코드(exe+RVA, 1KB버킷):\n");
        for (rva, c) in v.iter().take(20) {
            s.push_str(&format!("    {:>5.1}%  exe+{:#x}..{:#x}\n", *c as f64 * 100.0 / tot as f64, rva, rva + 0x3ff));
        }
        let mod_tot: u64 = ps.hist_mod.values().sum();
        if mod_tot > 0 {
            let mut m: Vec<(u64, u64)> = ps.hist_mod.iter().map(|(k, c)| (*k, *c)).collect();
            m.sort_by(|a, b| b.1.cmp(&a.1));
            s.push_str(&format!("  모드코드 합계 {:.1}% — top:", mod_tot as f64 * 100.0 / tot as f64));
            for (off, c) in m.iter().take(5) { s.push_str(&format!(" MOD+{:#x}({:.1}%)", off, *c as f64 * 100.0 / tot as f64)); }
            s.push('\n');
        }
        if ps.other > 0 { s.push_str(&format!("  기타(시스템dll 등): {:.1}%\n", ps.other as f64 * 100.0 / tot as f64)); }
    }
    if let Some(p) = pth("adv_prof.txt") {
        if let Ok(mut f) = fs::OpenOptions::new().create(true).append(true).open(&p) { let _ = f.write_all(s.as_bytes()); }
    }
}
fn prof_seg_reset(ps: &mut ProfSt, now: u64) {
    ps.seg_start = now; ps.seg_hb0 = HB_JUDGE.load(Ordering::Relaxed); ps.seg_cpu0 = ps.cpu_prev;
    for i in 0..5 { ps.seg_site0[i] = SITE_CNT[i].load(Ordering::Relaxed); }
    ps.samples = 0; ps.busy_acc = 0; ps.other = 0; ps.hist_exe.clear(); ps.hist_mod.clear();
}
// ★[07-16] 중간 스냅샷 — 끝나지 않는 활동창(일정넘김 안 끝남)에서도 구간마다 강제 덤프.
fn prof_segment(ps: &mut ProfSt, label: &str, now: u64) {
    let dur = now.saturating_sub(ps.seg_start);
    let hb_d = HB_JUDGE.load(Ordering::Relaxed).wrapping_sub(ps.seg_hb0);
    let cpu_ms = ps.cpu_prev.saturating_sub(ps.seg_cpu0) / 10_000;
    let mut site_d = [0u64; 5];
    for i in 0..5 { site_d[i] = SITE_CNT[i].load(Ordering::Relaxed).wrapping_sub(ps.seg_site0[i]); }
    let hdr = format!("══ {} ({}) 길이={:.1}s 평균 {:.1}코어 ══", label, now_ms(), dur as f64 / 1000.0,
        if dur > 0 { cpu_ms as f64 / dur as f64 } else { 0.0 });
    prof_report(ps, &hdr, dur, hb_d, cpu_ms, &site_d);
    prof_seg_reset(ps, now);
}
fn prof_win_end(ps: &mut ProfSt, dur_ms: u64, now: u64) {
    let min = ADV_PROF_MIN.load(Ordering::Relaxed).max(500) as u64;
    // 창 전체가 min 미만이면(그리고 구간 스냅샷도 안 냈으면) 생략
    if ps.n_seg == 0 && dur_ms < min { return; }
    if ps.n_win >= 100 { return; }
    ps.n_win += 1;
    let n = ps.n_win;
    prof_segment(ps, &format!("활동창 #{} 종료구간", n), now);
}
struct HangSt { last_hb: u64, frozen_ms: u64, run_ms: u64, cpu_prev: u64, n_stall: u32, n_run: u32 }

fn proc_cpu_100ns() -> u64 {
    unsafe {
        let (mut c, mut e, mut k, mut u) = (0u64, 0u64, 0u64, 0u64);
        extern "system" { fn GetProcessTimes(h: usize, c: *mut u64, e: *mut u64, k: *mut u64, u: *mut u64) -> i32; }
        if GetProcessTimes(GetCurrentProcess(), &mut c, &mut e, &mut k, &mut u) != 0 { k.wrapping_add(u) } else { 0 }
    }
}

fn hang_watchdog_step(st: &mut HangSt) {
    if HANG_DIAG.load(Ordering::Relaxed) == 0 { st.frozen_ms = 0; st.run_ms = 0; return; }
    let hb = HB_JUDGE.load(Ordering::Relaxed);
    let cpu = proc_cpu_100ns();
    let cpu_d_ms = cpu.saturating_sub(st.cpu_prev) / 10_000;   // 100ns→ms (1초창 CPU 소모 코어·ms)
    st.cpu_prev = cpu;
    let rate = hb.wrapping_sub(st.last_hb);   // 초당 judge 콜수
    st.last_hb = hb;
    if rate > 0 {
        // ── 활동 중: RUNAWAY 감시 (고속 시뮬이 계속 돎 = 연산이 안 끝남) ──
        st.frozen_ms = 0;
        let run_rate = HANG_RUN_RATE.load(Ordering::Relaxed).max(100) as u64;
        if rate >= run_rate { st.run_ms += 1000; } else { st.run_ms = 0; }
        let run_secs = HANG_RUN_SECS.load(Ordering::Relaxed).max(5) as u64;
        if st.run_ms >= run_secs * 1000 && st.n_run < 8 {
            st.n_run += 1; st.run_ms = 0;
            // ★[07-16] RUNAWAY도 스택+판단분포 덤프(전엔 false=in-flight만) — "안 끝나는 연산이 어디서" 규명
            hang_dump(&format!("RUNAWAY(시뮬 계속 진행: {}콜/s ≥ {}/s가 {}초 지속 — 경기 교착/무한시뮬 의심)", rate, run_rate, run_secs), hb, rate, cpu_d_ms, true);
        }
        return;
    }
    // ── 하트비트 정지: STALL 감시 ──
    st.run_ms = 0;
    st.frozen_ms += 1000;
    let stall_secs = HANG_SECS.load(Ordering::Relaxed).max(3) as u64;
    if st.frozen_ms < stall_secs * 1000 || st.n_stall >= 5 { return; }
    // 직접신호: 훅에 들어가서 안 나온 스레드(모드 코드 내부 갇힘)
    let now = tick_ms();
    let mut stuck = false;
    for i in 0..64 {
        if INFLIGHT_SITE[i].load(Ordering::Relaxed) != 0
            && now.saturating_sub(INFLIGHT_TS[i].load(Ordering::Relaxed)) > stall_secs * 1000 { stuck = true; break; }
    }
    let cpu_busy = cpu_d_ms >= 400;   // 1초창에 0.4코어초↑ = 뭔가 계속 계산중
    if stuck || cpu_busy {
        st.n_stall += 1; st.frozen_ms = 0;
        hang_dump(if stuck { "STALL(모드 훅 내부 갇힘 — in-flight 참조)" } else { "STALL(judge 정지+CPU 바쁨 — 게임측 루프/대기 의심)" }, hb, 0, cpu_d_ms, true);
    } else if st.frozen_ms > 3_600_000 { st.frozen_ms = 0; }   // 메뉴 장기 유휴 카운터 리셋(오버플로 방지)
}

// 덤프: in-flight 표 + 전 스레드 CPU δ(250ms 2패스) + RIP/스택 리턴주소(exe+RVA / MOD+off).
// ⚠스레드 suspend 중엔 할당/락 금지(힙락 데드락) — 고정배열 수집 후 resume하고 나서 포맷.
fn hang_dump(kind: &str, hb: u64, rate: u64, cpu_d_ms: u64, with_stacks: bool) {
    let mut s = format!("\n════ HANG DIAG [{}] hb={} rate={}/s cpu={}ms/s (tick={} wall={}) ════\n",
        kind, hb, rate, cpu_d_ms, tick_ms(), now_ms());
    // 1) in-flight (지금 훅 안에 있는 스레드)
    let now = tick_ms();
    let mut any = false;
    for i in 0..64 {
        let site = INFLIGHT_SITE[i].load(Ordering::Relaxed);
        if site != 0 {
            any = true;
            let age = now.saturating_sub(INFLIGHT_TS[i].load(Ordering::Relaxed));
            s.push_str(&format!("  in-flight: tid={} site={} 경과={}ms{}\n",
                INFLIGHT_TID[i].load(Ordering::Relaxed),
                HANG_SITE_NAMES.get(site as usize).unwrap_or(&"?"), age,
                if age > 3000 { " ★갇힘의심(모드 코드 내부)" } else { "" }));
        }
    }
    if !any { s.push_str("  in-flight: 없음 (모드 훅 내부에 갇힌 스레드 없음 → 게임측)\n"); }
    // 2) judge 사이트 분포 — 어느 판단이 폭주하나(2초 델타)
    let mut c0 = [0u64; 5];
    for i in 0..5 { c0[i] = SITE_CNT[i].load(Ordering::Relaxed); }
    std::thread::sleep(std::time::Duration::from_millis(2000));
    let mut d = [0u64; 5];
    for i in 0..5 { d[i] = SITE_CNT[i].load(Ordering::Relaxed).wrapping_sub(c0[i]); }
    s.push_str(&format!("  judge분포(2초): retreat {} / recall {} / condgate {} / movepri {} (초당 {})\n",
        d[1], d[2], d[3], d[4], (d[1]+d[2]+d[3]+d[4]) / 2));
    if with_stacks {
        unsafe { hang_dump_threads(&mut s); }
    }
    s.push_str("════ end ════\n");
    if let Some(p) = pth("hang_diag.txt") {
        if let Ok(mut f) = fs::OpenOptions::new().create(true).append(true).open(&p) { let _ = f.write_all(s.as_bytes()); }
    }
}

unsafe fn hang_dump_threads(s: &mut String) {
    const TH32CS_SNAPTHREAD: u32 = 0x4;
    const ACCESS: u32 = 0x0002 | 0x0008 | 0x0040;   // SUSPEND_RESUME | GET_CONTEXT | QUERY_INFORMATION
    let my_pid = GetCurrentProcessId();
    let my_tid = GetCurrentThreadId();
    let snap = CreateToolhelp32Snapshot(TH32CS_SNAPTHREAD, 0);
    if snap == -1 || snap == 0 { s.push_str("  (스레드 스냅샷 실패)\n"); return; }
    struct Th { tid: u32, h: isize, t0: u64, busy_ms: u64, rip: u64, nret: usize, rets: [u64; 20] }
    let mut ths: Vec<Th> = Vec::with_capacity(96);
    let mut te = ThreadEntry32 { size: core::mem::size_of::<ThreadEntry32>() as u32, usage: 0, tid: 0, owner_pid: 0, base_pri: 0, delta_pri: 0, flags: 0 };
    if Thread32First(snap, &mut te) != 0 {
        loop {
            if te.owner_pid == my_pid && te.tid != my_tid && ths.len() < 96 {
                let h = OpenThread(ACCESS, 0, te.tid);
                if h != 0 && h != -1 { ths.push(Th { tid: te.tid, h, t0: 0, busy_ms: 0, rip: 0, nret: 0, rets: [0u64; 20] }); }
            }
            if Thread32Next(snap, &mut te) == 0 { break; }
        }
    }
    CloseHandle(snap);
    // CPU 2패스(250ms δ) — 어떤 스레드가 실제로 도는지
    for t in ths.iter_mut() {
        let (mut c, mut e, mut k, mut u) = (0u64, 0u64, 0u64, 0u64);
        if GetThreadTimes(t.h, &mut c, &mut e, &mut k, &mut u) != 0 { t.t0 = k.wrapping_add(u); }
    }
    std::thread::sleep(std::time::Duration::from_millis(250));
    for t in ths.iter_mut() {
        let (mut c, mut e, mut k, mut u) = (0u64, 0u64, 0u64, 0u64);
        if GetThreadTimes(t.h, &mut c, &mut e, &mut k, &mut u) != 0 { t.busy_ms = (k.wrapping_add(u)).saturating_sub(t.t0) / 10_000; }
    }
    // suspend → RIP/스택수집(고정배열만, 할당·락 금지) → resume
    let exe = exe_base() as u64;
    let modb = CRASH_MOD_BASE.load(Ordering::Relaxed);
    for t in ths.iter_mut() {
        if SuspendThread(t.h) == u32::MAX { continue; }
        let mut ctx = CtxBuf([0u8; 1360]);
        *(ctx.0.as_mut_ptr().add(0x30) as *mut u32) = 0x10_0001;   // CONTEXT_AMD64 | CONTEXT_CONTROL
        if GetThreadContext(t.h, ctx.0.as_mut_ptr()) != 0 {
            t.rip = *(ctx.0.as_ptr().add(0xF8) as *const u64);
            let rsp = *(ctx.0.as_ptr().add(0x98) as *const u64) as usize;
            let mut off = 0usize;
            while off < 0x600 && t.nret < 20 {
                if let Some(v) = lr_u64(rsp.wrapping_add(off)) {
                    let de = v.wrapping_sub(exe); let dm = v.wrapping_sub(modb);
                    if de < 0x8000000 || (modb != 0 && dm < 0x2000000) { t.rets[t.nret] = v; t.nret += 1; }
                }
                off += 8;
            }
        }
        ResumeThread(t.h);
    }
    // resume 완료 후 포맷(여기부턴 할당 자유)
    ths.sort_by(|a, b| b.busy_ms.cmp(&a.busy_ms));
    let attr = |v: u64| -> String {
        let de = v.wrapping_sub(exe); let dm = v.wrapping_sub(modb);
        if de < 0x8000000 { format!("exe+{:#x}", de) }
        else if modb != 0 && dm < 0x2000000 { format!("MOD+{:#x}", dm) }
        else { format!("{:#x}", v) }
    };
    let mut shown = 0;
    for t in ths.iter() {
        let interesting = t.busy_ms > 0 || (modb != 0 && t.rip.wrapping_sub(modb) < 0x2000000);
        if !interesting && shown >= 12 { continue; }   // 유휴 스레드는 12개까지만
        shown += 1;
        s.push_str(&format!("  tid={:>6} busy={:>3}ms/250ms RIP={}", t.tid, t.busy_ms, attr(t.rip)));
        if t.nret > 0 {
            s.push_str("  stack:[");
            for i in 0..t.nret.min(12) { if i > 0 { s.push(' '); } s.push_str(&attr(t.rets[i])); }
            s.push(']');
        }
        s.push('\n');
    }
    s.push_str(&format!("  (총 {}스레드 관찰, busy 내림차순)\n", ths.len()));
    for t in ths.iter() { CloseHandle(t.h); }
}
