//! diag.rs — 크래시 진단용 VEH(Vectored Exception Handler).
//! 처리되지 않은 AV/스택오버플로 예외의 주소(RVA)를 crash_log.txt에 raw로 기록.
//! 목적: "밴픽→경기 전환" 크래시가 게임의 어느 함수에서 나는지 특정.
//!
//! 안전수칙(CLAUDE.md): VEH 핸들러 안엔 패닉유발 코드(format!/lock/Vec/push) 절대 금지.
//!   → 미리 연 파일 핸들에 스택 버퍼로 수동 hex 변환해 WriteFile만 한다.
//!   → first=0(리스트 끝)로 등록 = 다른 VEH(safe_read 등)가 CONTINUE_EXECUTION으로
//!     처리하는 정상 AV는 내게 안 오고, "아무도 처리 못한 진짜 크래시"만 기록된다.

use std::sync::atomic::{AtomicIsize, AtomicUsize, AtomicU32, Ordering};

#[repr(C)]
struct ExceptionRecord {
    code: u32,
    flags: u32,
    record: usize,
    address: usize,
    number_params: u32,
    _pad: u32,
    info: [usize; 15],
}
#[repr(C)]
struct ExceptionPointers {
    exception_record: *mut ExceptionRecord,
    context_record: usize,
}

const GENERIC_WRITE: u32 = 0x4000_0000;
const FILE_SHARE_RW: u32 = 0x3;
const OPEN_ALWAYS: u32 = 4;
const FILE_APPEND: u32 = 0x0004; // FILE_APPEND_DATA
const INVALID_HANDLE: isize = -1;
const FILE_FLAG_WRITE_THROUGH: u32 = 0x8000_0000;

const EXC_AV: u32 = 0xC000_0005;
const EXC_STACK_OVERFLOW: u32 = 0xC000_00FD;
const EXC_ILLEGAL: u32 = 0xC000_001D;
const EXC_PRIV: u32 = 0xC000_0096;

#[link(name = "kernel32")]
extern "system" {
    fn AddVectoredExceptionHandler(first: u32, handler: usize) -> usize;
    fn CreateFileW(
        name: *const u16,
        access: u32,
        share: u32,
        sec: usize,
        disp: u32,
        flags: u32,
        template: usize,
    ) -> isize;
    fn WriteFile(h: isize, buf: *const u8, n: u32, written: *mut u32, ov: usize) -> i32;
    fn SetFilePointer(h: isize, lo: i32, hi: usize, method: u32) -> u32;
    fn VirtualProtect(addr: usize, size: usize, prot: u32, old: *mut u32) -> i32;
    fn FlushInstructionCache(h: isize, addr: usize, size: usize) -> i32;
    fn GetCurrentProcess() -> isize;
    fn VirtualAlloc(addr: usize, size: usize, ty: u32, prot: u32) -> usize;
    fn GetCurrentThread() -> isize;
    fn GetThreadContext(h: isize, ctx: *mut u8) -> i32;
    fn SetThreadContext(h: isize, ctx: *const u8) -> i32;
    fn VirtualQuery(addr: usize, buf: *mut u8, len: usize) -> usize;
}

/// base 부터 안전하게 읽을 수 있는 길이(≤want)를 반환. 커밋·읽기가능·가드아님 검사.
/// 러너 블록 스냅샷이 힙 할당 끝을 넘어 unmapped 페이지를 읽지 않게 하는 가드.
pub unsafe fn region_cap(base: usize, want: usize) -> usize {
    // MEMORY_BASIC_INFORMATION(x64): +0x00 BaseAddress +0x18 RegionSize
    // +0x20 State +0x24 Protect (총 0x30)
    let mut mbi = [0u8; 0x30];
    if VirtualQuery(base, mbi.as_mut_ptr(), 0x30) == 0 {
        return 0;
    }
    let region_base = core::ptr::read(mbi.as_ptr() as *const usize);
    let region_size = core::ptr::read(mbi.as_ptr().add(0x18) as *const usize);
    let state = core::ptr::read(mbi.as_ptr().add(0x20) as *const u32);
    let protect = core::ptr::read(mbi.as_ptr().add(0x24) as *const u32);
    if state != 0x1000 {
        return 0; // MEM_COMMIT 아님
    }
    if protect & 0x100 != 0 || protect == 0x01 {
        return 0; // PAGE_GUARD / PAGE_NOACCESS
    }
    let end = region_base.wrapping_add(region_size);
    if end <= base {
        return 0;
    }
    (end - base).min(want)
}

const EXC_BP: u32 = 0x8000_0003; // EXCEPTION_BREAKPOINT (int3)
/// 전환함수(0.5.2 0x11d8ef0 / 0.5.3 0x1bcf010) 내 panic 호출 사이트 (RVA) — RE §17f.
/// ⚠**0.5.3 미재핀**(디버그 전용 라벨이라 기능 영향 없음): 0.5.2 값은 0.5.3에서 무의미하므로
/// 0으로 비워 오라벨을 막는다. 필요해지면 컨테이너 0x1bcf010 안에서 패턴 재탐색할 것.
const PANIC_SITES: [usize; 6] = [0, 0, 0, 0, 0, 0];
static PANIC_MARKED: AtomicU32 = AtomicU32::new(0);

static BASE_D: AtomicUsize = AtomicUsize::new(0);
static LOG_H: AtomicIsize = AtomicIsize::new(INVALID_HANDLE);
static SEQ_D: AtomicU32 = AtomicU32::new(0);
static INSTALLED: AtomicUsize = AtomicUsize::new(0);
/// lib.rs가 매 프레임 갱신 — 크래시 순간의 컨텍스트(밴픽화면 여부·진행 total).
pub static CTX_IN_BANPICK: AtomicU32 = AtomicU32::new(0);
pub static CTX_LAST_TOTAL: AtomicU32 = AtomicU32::new(0xFFFF_FFFF);

/// usize를 "0x0000000000000000" 형태 18바이트로 스택 버퍼에 기록.
#[inline]
fn hex64(v: usize, out: &mut [u8; 18]) {
    out[0] = b'0';
    out[1] = b'x';
    let d = b"0123456789abcdef";
    for i in 0..16 {
        let nib = (v >> ((15 - i) * 4)) & 0xf;
        out[2 + i] = d[nib];
    }
}

/// u32를 8-hex로.
#[inline]
fn hex32(v: u32, out: &mut [u8; 8]) {
    let d = b"0123456789abcdef";
    for i in 0..8 {
        out[i] = d[((v >> ((7 - i) * 4)) & 0xf) as usize];
    }
}

/// u8을 2-hex로.
#[inline]
fn hex8(v: u8, out: &mut [u8; 2]) {
    let d = b"0123456789abcdef";
    out[0] = d[(v >> 4) as usize];
    out[1] = d[(v & 0xf) as usize];
}

static RECS_DUMPED: AtomicU32 = AtomicU32::new(0);

/// MatchSetInfo(=밴픽 이력 레코드 컨테이너)의 side(+0xf8)/step(+0xf9) 시퀀스 덤프. 1회.
/// info+8 = 레코드배열 base, info+0x10 = count, stride 0x100. 표준 vs 인터리브 비교용.
pub unsafe fn dump_records(info: usize) {
    // 1회 게이트 제거 — 호출자(hook_container)가 count-최대 추적으로 호출 빈도 제어.
    let h = LOG_H.load(Ordering::Relaxed);
    if h == INVALID_HANDLE || h == 0 {
        return;
    }
    let recbase = core::ptr::read((info + 8) as *const usize);
    let count = core::ptr::read((info + 0x10) as *const usize);
    let mut line = [0u8; 700];
    let mut p = 0usize;
    macro_rules! put {
        ($b:expr) => {{
            for &c in ($b).iter() {
                if p < line.len() {
                    line[p] = c;
                    p += 1;
                }
            }
        }};
    }
    // 헤더 무조건: 컨테이너 실제 구조(info/base/count) 확인용.
    let mut h18 = [0u8; 18];
    put!(b"RECS info=");
    hex64(info, &mut h18);
    put!(&h18);
    put!(b" base=");
    hex64(recbase, &mut h18);
    put!(&h18);
    put!(b" count=");
    hex64(count, &mut h18);
    put!(&h18);
    if !(recbase >= 0x10000 && count >= 1 && count <= 40) {
        put!(b"\n");
        let mut w = 0u32;
        WriteFile(h, line.as_ptr(), p as u32, &mut w, 0);
        return;
    }
    // 진짜 밴픽 레코드(마지막) 정본: tag(+0xc0)·side(+0xf8)·step(+0xf9)·4벡터len 하위바이트.
    let rec = recbase + (count - 1) * 0x100;
    let mut hs = [0u8; 2];
    let tag = core::ptr::read((rec + 0xc0) as *const u8);
    let side = core::ptr::read((rec + 0xf8) as *const u8);
    let step = core::ptr::read((rec + 0xf9) as *const u8);
    put!(b" tag=");
    hex8(tag, &mut hs);
    put!(&hs);
    put!(b" side=");
    hex8(side, &mut hs);
    put!(&hs);
    put!(b" step=");
    hex8(step, &mut hs);
    put!(&hs);
    put!(b" vlen=");
    for &vo in &[0x40usize, 0x58, 0x70, 0x88] {
        let l = core::ptr::read((rec + vo) as *const usize);
        hex8((l & 0xff) as u8, &mut hs);
        put!(&hs);
        put!(b".");
    }
    put!(b"\n");
    let mut w = 0u32;
    WriteFile(h, line.as_ptr(), p as u32, &mut w, 0);
}

/// 크래시 순간 raw write. 할당·패닉 없음. 고정 포맷 1줄 + 콜스택.
unsafe fn write_crash(info: *mut ExceptionPointers, code: u32, addr: usize, rva: usize) {
    let h = LOG_H.load(Ordering::Relaxed);
    if h == INVALID_HANDLE || h == 0 {
        return;
    }
    // 라인: "CRASH ... stack: <게임내 return addr rva들>\n"
    let mut line = [0u8; 512];
    let mut p = 0usize;
    macro_rules! put {
        ($bytes:expr) => {{
            let b = $bytes;
            for &c in b.iter() {
                if p < line.len() {
                    line[p] = c;
                    p += 1;
                }
            }
        }};
    }
    let seq = SEQ_D.fetch_add(1, Ordering::Relaxed);
    let mut h8 = [0u8; 8];
    put!(b"CRASH seq=");
    hex32(seq, &mut h8);
    put!(&h8);
    put!(b" code=");
    hex32(code, &mut h8);
    put!(&h8);
    let mut h18 = [0u8; 18];
    put!(b" addr=");
    hex64(addr, &mut h18);
    put!(&h18);
    put!(b" rva=");
    hex64(rva, &mut h18);
    put!(&h18);
    put!(b" in_bp=");
    put!(if CTX_IN_BANPICK.load(Ordering::Relaxed) != 0 { b"1" } else { b"0" });
    put!(b" total=");
    hex32(CTX_LAST_TOTAL.load(Ordering::Relaxed), &mut h8);
    put!(&h8);
    // AV 시점 레지스터: rip(크래시 명령, 게임내면 'g'+rva)·memcpy 인자 dst(rcx)/src(rdx)/len(r8).
    if !info.is_null() {
        let ctx = (*info).context_record;
        if ctx >= 0x10000 {
            let bs = BASE_D.load(Ordering::Relaxed);
            let rip = core::ptr::read((ctx + 0xF8) as *const usize);
            let rcx = core::ptr::read((ctx + 0x80) as *const usize);
            let rdx = core::ptr::read((ctx + 0x88) as *const usize);
            let r8 = core::ptr::read((ctx + 0xB8) as *const usize);
            put!(b" rip=");
            if bs != 0 && rip >= bs && rip < bs + 0x4400000 {
                put!(b"g");
                hex64(rip - bs, &mut h18);
            } else {
                hex64(rip, &mut h18);
            }
            put!(&h18);
            put!(b" dst=");
            hex64(rcx, &mut h18);
            put!(&h18);
            put!(b" src=");
            hex64(rdx, &mut h18);
            put!(&h18);
            put!(b" len=");
            hex64(r8, &mut h18);
            put!(&h18);
        }
    }
    // 콜스택: 스택에서 게임 exe 범위 return address 후보 수집(게임 내 크래시 유발 지점 추적).
    put!(b" stack:");
    if !info.is_null() {
        let ctx = (*info).context_record;
        if ctx >= 0x10000 {
            let rsp = core::ptr::read((ctx + 0x98) as *const usize); // CONTEXT.Rsp (x64)
            let base = BASE_D.load(Ordering::Relaxed);
            if rsp >= 0x10000 && base != 0 {
                let mut cnt = 0;
                let mut i = 0;
                while i < 256 && cnt < 16 {
                    let sv = core::ptr::read((rsp + i * 8) as *const usize);
                    if sv >= base && sv < base + 0x4400000 {
                        put!(b" ");
                        hex64(sv - base, &mut h18);
                        put!(&h18);
                        cnt += 1;
                    }
                    i += 1;
                }
            }
        }
    }
    put!(b"\n");
    let mut written: u32 = 0;
    WriteFile(h, line.as_ptr(), p as u32, &mut written, 0);
}

/// panic site int3 도달 기록 (1회). crash_log에 "PANIC rva=0x..\n".
unsafe fn write_panic_marker(rva: usize) {
    if PANIC_MARKED.swap(1, Ordering::Relaxed) != 0 {
        return;
    }
    let h = LOG_H.load(Ordering::Relaxed);
    if h == INVALID_HANDLE || h == 0 {
        return;
    }
    let mut line = [0u8; 32];
    let mut p = 0usize;
    for &c in b"PANIC rva=" {
        line[p] = c;
        p += 1;
    }
    let mut h18 = [0u8; 18];
    hex64(rva, &mut h18);
    for &c in h18.iter() {
        line[p] = c;
        p += 1;
    }
    line[p] = b'\n';
    p += 1;
    let mut w = 0u32;
    WriteFile(h, line.as_ptr(), p as u32, &mut w, 0);
}

// ── DR(하드웨어) write BP — 칸 채움색 실제 기록자 포착 (MIGRATION §7.3 §14.5(6)) ──
// 정적 후보 5종이 전부 오답이라 "누가 그 메모리를 쓰는가"를 실행 중 직접 잡는다.
// ⚠DR 레지스터는 스레드별 — arm_watch 는 post_update(메인 스레드)에서 호출해야
//   메인 스레드의 write 를 잡는다. 히트 0 인데 화면이 바뀌면 = 기록자가 딴 스레드.
const EXC_SINGLE_STEP: u32 = 0x8000_0004;
const CONTEXT_DEBUG_REGISTERS: u32 = 0x0010_0010; // CONTEXT_AMD64 | 0x10

/// CONTEXT(x64) raw 버퍼. 필드 오프셋: +0x30 ContextFlags / +0x48 Dr0..+0x60 Dr3 /
/// +0x68 Dr6 / +0x70 Dr7 / +0x98 Rsp / +0xF8 Rip (write_crash 와 동일 규약).
#[repr(C, align(16))]
struct RawContext([u8; 0x4d0]);

static WATCHED: [AtomicUsize; 4] = [const { AtomicUsize::new(0) }; 4];

/// 현재 스레드에 4바이트 write BP 를 최대 4개 건다(원하는 집합이 그대로면 no-op).
/// want 가 비면 전부 해제. 주소는 4바이트 정렬이어야 한다(DR 하드웨어 요건).
pub unsafe fn arm_watch(want: &[usize]) {
    let mut w = [0usize; 4];
    for (i, &a) in want.iter().take(4).enumerate() {
        w[i] = a & !3; // 정렬 보정
    }
    if (0..4).all(|i| WATCHED[i].load(Ordering::Relaxed) == w[i]) {
        return;
    }
    let mut ctx = RawContext([0u8; 0x4d0]);
    let p = ctx.0.as_mut_ptr();
    core::ptr::write(p.add(0x30) as *mut u32, CONTEXT_DEBUG_REGISTERS);
    let th = GetCurrentThread();
    if GetThreadContext(th, p) == 0 {
        return;
    }
    let mut dr7: u64 = 0;
    for i in 0..4 {
        core::ptr::write(p.add(0x48 + i * 8) as *mut u64, w[i] as u64);
        if w[i] != 0 {
            dr7 |= 1 << (2 * i); // Ln (local enable)
            dr7 |= 0b01 << (16 + 4 * i); // RW = write 전용
            dr7 |= 0b11 << (18 + 4 * i); // LEN = 4바이트
        }
    }
    core::ptr::write(p.add(0x68) as *mut u64, 0); // Dr6 클리어
    core::ptr::write(p.add(0x70) as *mut u64, dr7);
    core::ptr::write(p.add(0x30) as *mut u32, CONTEXT_DEBUG_REGISTERS);
    if SetThreadContext(th, p) == 0 {
        return;
    }
    for i in 0..4 {
        WATCHED[i].store(w[i], Ordering::Relaxed);
    }
}

const DRBP_MAX: usize = 32;
static DRBP_RIP: [AtomicUsize; DRBP_MAX] = [const { AtomicUsize::new(0) }; DRBP_MAX];
static DRBP_CNT: [AtomicU32; DRBP_MAX] = [const { AtomicU32::new(0) }; DRBP_MAX];

/// 메인 스레드용 스냅샷(할당 OK — VEH 밖에서만 호출).
pub fn drbp_stats() -> Vec<(usize, u32)> {
    let mut out = Vec::new();
    for i in 0..DRBP_MAX {
        let r = DRBP_RIP[i].load(Ordering::Relaxed);
        if r != 0 {
            out.push((r, DRBP_CNT[i].load(Ordering::Relaxed)));
        }
    }
    out
}

/// VEH 내부 — 할당·패닉 금지. 첫 조우 rip 만 로그(이후는 카운트만).
unsafe fn drbp_record(dr6: u64, rip: usize, ctx: usize) {
    for i in 0..DRBP_MAX {
        let cur = DRBP_RIP[i].load(Ordering::Relaxed);
        if cur == rip {
            DRBP_CNT[i].fetch_add(1, Ordering::Relaxed);
            return;
        }
        if cur == 0 {
            if DRBP_RIP[i]
                .compare_exchange(0, rip, Ordering::Relaxed, Ordering::Relaxed)
                .is_err()
            {
                // 경합 — 같은 슬롯을 재검사
                if DRBP_RIP[i].load(Ordering::Relaxed) == rip {
                    DRBP_CNT[i].fetch_add(1, Ordering::Relaxed);
                    return;
                }
                continue;
            }
            DRBP_CNT[i].store(1, Ordering::Relaxed);
            // 로그 1줄: DRBP dr6=.. rip=(게임내면 g+rva) stack: <게임내 ret rva 최대 8>
            let h = LOG_H.load(Ordering::Relaxed);
            if h == INVALID_HANDLE || h == 0 {
                return;
            }
            let base = BASE_D.load(Ordering::Relaxed);
            let mut line = [0u8; 300];
            let mut p = 0usize;
            macro_rules! put {
                ($b:expr) => {{
                    for &c in ($b).iter() {
                        if p < line.len() {
                            line[p] = c;
                            p += 1;
                        }
                    }
                }};
            }
            let mut h8 = [0u8; 8];
            let mut h18 = [0u8; 18];
            put!(b"DRBP dr6=");
            hex32((dr6 & 0xf) as u32, &mut h8);
            put!(&h8);
            put!(b" rip=");
            if base != 0 && rip >= base && rip < base + 0x4400000 {
                put!(b"g");
                hex64(rip - base, &mut h18);
            } else {
                hex64(rip, &mut h18);
            }
            put!(&h18);
            put!(b" stack:");
            let rsp = core::ptr::read((ctx + 0x98) as *const usize);
            if rsp >= 0x10000 && base != 0 {
                let mut cnt = 0;
                let mut k = 0;
                while k < 128 && cnt < 8 {
                    let sv = core::ptr::read((rsp + k * 8) as *const usize);
                    if sv >= base && sv < base + 0x4400000 {
                        put!(b" ");
                        hex64(sv - base, &mut h18);
                        put!(&h18);
                        cnt += 1;
                    }
                    k += 1;
                }
            }
            put!(b"\n");
            let mut w = 0u32;
            WriteFile(h, line.as_ptr(), p as u32, &mut w, 0);
            return;
        }
    }
}

unsafe extern "system" fn veh(info: *mut ExceptionPointers) -> i32 {
    if info.is_null() {
        return 0; // EXCEPTION_CONTINUE_SEARCH
    }
    let rec = (*info).exception_record;
    if rec.is_null() {
        return 0;
    }
    let code = (*rec).code;
    let base = BASE_D.load(Ordering::Relaxed);
    // DR write BP 히트 = single-step 예외(데이터 BP는 명령 "실행 후" 트랩 → RF 불요,
    // Dr6 클리어 후 그 자리에서 재개). 우리가 건 게 아니면(dr6 하위 4비트 0) 통과.
    if code == EXC_SINGLE_STEP {
        let ctx = (*info).context_record;
        if ctx >= 0x10000 {
            let dr6 = core::ptr::read((ctx + 0x68) as *const u64);
            if dr6 & 0xf != 0 {
                let rip = core::ptr::read((ctx + 0xF8) as *const usize);
                drbp_record(dr6, rip, ctx);
                core::ptr::write((ctx + 0x68) as *mut u64, 0);
                return -1; // EXCEPTION_CONTINUE_EXECUTION
            }
        }
        return 0;
    }
    // int3 = panic site 도달 포착 → 어느 site인지 기록.
    if code == EXC_BP {
        let rva = (*rec).address.wrapping_sub(base);
        if PANIC_SITES.contains(&rva) {
            write_panic_marker(rva);
        }
        return 0; // 게임 기본 처리로(어차피 크래시) — 어느 site 도달만 기록
    }
    if code == EXC_AV
        || code == EXC_STACK_OVERFLOW
        || code == EXC_ILLEGAL
        || code == EXC_PRIV
    {
        let addr = (*rec).address;
        let rva = addr.wrapping_sub(base);
        write_crash(info, code, addr, rva);
    }
    0 // 항상 CONTINUE_SEARCH — 게임 기본 크래시 처리로 넘김(주소만 기록)
}

/// 완료게이트 시점 씬 4벡터(밴1/밴2/픽1/픽2)의 챔프명을 덤프하고 중복(픽↔밴 교집합
/// 포함) 개수를 crash_log에 기록. 목적: 인터리브 크래시가 "유일성 위반(같은 챔프가
/// 픽+밴 동시 존재)"인지 확증. detour 안에서 호출되나 완료게이트=1회라 부하 무시.
/// scene 4벡터 = ban1{ptr+0x140/len+0x148} ban2{+0x158/+0x160} pick1{+0x170/+0x178}
/// pick2{+0x188/+0x190}, 요소 = String 0x18 {cap, ptr+8, len+0x10}.
pub unsafe fn dump_roster_check(scene: usize) {
    let h = LOG_H.load(Ordering::Relaxed);
    if h == INVALID_HANDLE || h == 0 {
        return;
    }
    // (ptr_off, len_off, tag)
    let vecs: [(usize, usize, u8); 4] = [
        (0x140, 0x148, b'b'), // ban1
        (0x158, 0x160, b'b'), // ban2
        (0x170, 0x178, b'p'), // pick1
        (0x188, 0x190, b'p'), // pick2
    ];
    const MAXN: usize = 24;
    const MAXL: usize = 40;
    let mut names = [[0u8; MAXL]; MAXN];
    let mut lens = [0usize; MAXN];
    let mut tags = [0u8; MAXN];
    let mut n = 0usize;
    for &(poff, loff, tag) in vecs.iter() {
        let vptr = core::ptr::read((scene + poff) as *const usize);
        let vlen = core::ptr::read((scene + loff) as *const usize);
        if vptr < 0x10000 || vlen > MAXN {
            continue;
        }
        for i in 0..vlen {
            if n >= MAXN {
                break;
            }
            let elem = vptr + i * 0x18;
            let np = core::ptr::read((elem + 8) as *const usize);
            let nl = core::ptr::read((elem + 0x10) as *const usize);
            if np < 0x10000 || nl == 0 || nl > MAXL {
                continue;
            }
            core::ptr::copy_nonoverlapping(np as *const u8, names[n].as_mut_ptr(), nl);
            lens[n] = nl;
            tags[n] = tag;
            n += 1;
        }
    }
    // 중복 검사 (전체 픽+밴 집합 내 같은 챔프명). pick↔ban 교집합은 dup_pb로 별도 표기.
    let mut dup = 0u32;
    let mut dup_pb = 0u32;
    for i in 0..n {
        for j in (i + 1)..n {
            if lens[i] == lens[j] && names[i][..lens[i]] == names[j][..lens[j]] {
                dup += 1;
                if tags[i] != tags[j] {
                    dup_pb += 1; // 하나는 pick, 하나는 ban = 유일성 위반 스모킹건
                }
            }
        }
    }
    // 라인: "ROSTER n=NN dup=DD dup_pb=DD : <tag>name <tag>name ...\n"
    let mut line = [0u8; 1400];
    let mut p = 0usize;
    macro_rules! put {
        ($b:expr) => {{
            for &c in ($b).iter() {
                if p < line.len() {
                    line[p] = c;
                    p += 1;
                }
            }
        }};
    }
    let mut h8 = [0u8; 8];
    put!(b"ROSTER n=");
    hex32(n as u32, &mut h8);
    put!(&h8);
    put!(b" dup=");
    hex32(dup, &mut h8);
    put!(&h8);
    put!(b" dup_pb=");
    hex32(dup_pb, &mut h8);
    put!(&h8);
    put!(b" :");
    for i in 0..n {
        put!(b" ");
        put!(&[tags[i]]);
        put!(&names[i][..lens[i]]);
    }
    put!(b"\n");
    let mut written: u32 = 0;
    WriteFile(h, line.as_ptr(), p as u32, &mut written, 0);

    // +0x1d0/1e8 = 전환함수 0x11d8ef0이 실제 소비하는 팀별 플레이어 로스터(Vec<u64>).
    // t1: ptr@+0x1d0 len@+0x1d8 / t2: ptr@+0x1e8 len@+0x1f0. 요소 = 엔티티 핸들 u64.
    let mut l2 = [0u8; 600];
    let mut q = 0usize;
    macro_rules! put2 {
        ($b:expr) => {{
            for &c in ($b).iter() {
                if q < l2.len() {
                    l2[q] = c;
                    q += 1;
                }
            }
        }};
    }
    let mut hx = [0u8; 18];
    let t1p = core::ptr::read((scene + 0x1d0) as *const usize);
    let t1l = core::ptr::read((scene + 0x1d8) as *const usize);
    let t2p = core::ptr::read((scene + 0x1e8) as *const usize);
    let t2l = core::ptr::read((scene + 0x1f0) as *const usize);
    put2!(b"PROSTER t1len=");
    hex64(t1l, &mut hx);
    put2!(&hx);
    put2!(b" t2len=");
    hex64(t2l, &mut hx);
    put2!(&hx);
    put2!(b" t1:");
    if t1p >= 0x10000 && t1l <= 16 {
        for i in 0..t1l {
            let v = core::ptr::read((t1p + i * 8) as *const usize);
            put2!(b" ");
            hex64(v, &mut hx);
            put2!(&hx);
        }
    }
    put2!(b" t2:");
    if t2p >= 0x10000 && t2l <= 16 {
        for i in 0..t2l {
            let v = core::ptr::read((t2p + i * 8) as *const usize);
            put2!(b" ");
            hex64(v, &mut hx);
            put2!(&hx);
        }
    }
    put2!(b"\n");
    WriteFile(h, l2.as_ptr(), q as u32, &mut written, 0);
}

static MSI_DUMPED: AtomicU32 = AtomicU32::new(0);

/// 서버 권위 MatchSetInfo(hook_phase_info의 rcx) 4벡터 덤프 + 픽↔밴 중복 검사. 1회.
/// MSI 오프셋: ban1{ptr+0x38/len+0x40} ban2{+0x50/+0x58} pick1{+0x68/+0x70} pick2{+0x80/+0x88}.
pub unsafe fn dump_msi_check(info: usize) {
    if MSI_DUMPED.swap(1, Ordering::Relaxed) != 0 {
        return;
    }
    let h = LOG_H.load(Ordering::Relaxed);
    if h == INVALID_HANDLE || h == 0 {
        return;
    }
    let vecs: [(usize, usize, u8); 4] = [
        (0x38, 0x40, b'b'),
        (0x50, 0x58, b'b'),
        (0x68, 0x70, b'p'),
        (0x80, 0x88, b'p'),
    ];
    const MAXN: usize = 24;
    const MAXL: usize = 40;
    let mut names = [[0u8; MAXL]; MAXN];
    let mut lens = [0usize; MAXN];
    let mut tags = [0u8; MAXN];
    let mut n = 0usize;
    for &(poff, loff, tag) in vecs.iter() {
        let vptr = core::ptr::read((info + poff) as *const usize);
        let vlen = core::ptr::read((info + loff) as *const usize);
        if vptr < 0x10000 || vlen > MAXN {
            continue;
        }
        for i in 0..vlen {
            if n >= MAXN {
                break;
            }
            let elem = vptr + i * 0x18;
            let np = core::ptr::read((elem + 8) as *const usize);
            let nl = core::ptr::read((elem + 0x10) as *const usize);
            if np < 0x10000 || nl == 0 || nl > MAXL {
                continue;
            }
            core::ptr::copy_nonoverlapping(np as *const u8, names[n].as_mut_ptr(), nl);
            lens[n] = nl;
            tags[n] = tag;
            n += 1;
        }
    }
    let mut dup = 0u32;
    let mut dup_pb = 0u32;
    for i in 0..n {
        for j in (i + 1)..n {
            if lens[i] == lens[j] && names[i][..lens[i]] == names[j][..lens[j]] {
                dup += 1;
                if tags[i] != tags[j] {
                    dup_pb += 1;
                }
            }
        }
    }
    let mut line = [0u8; 1400];
    let mut p = 0usize;
    macro_rules! put {
        ($b:expr) => {{
            for &c in ($b).iter() {
                if p < line.len() {
                    line[p] = c;
                    p += 1;
                }
            }
        }};
    }
    let mut h8 = [0u8; 8];
    put!(b"MSI n=");
    hex32(n as u32, &mut h8);
    put!(&h8);
    put!(b" dup=");
    hex32(dup, &mut h8);
    put!(&h8);
    put!(b" dup_pb=");
    hex32(dup_pb, &mut h8);
    put!(&h8);
    put!(b" :");
    for i in 0..n {
        put!(b" ");
        put!(&[tags[i]]);
        put!(&names[i][..lens[i]]);
    }
    put!(b"\n");
    let mut written: u32 = 0;
    WriteFile(h, line.as_ptr(), p as u32, &mut written, 0);
}

static STEP_N: AtomicU32 = AtomicU32::new(0);

/// 스텝 추적: 각 훅이 특정 total에서 무엇을 반환/수행했는지 1줄. 최대 80줄.
/// tag: 3바이트 식별자(b"TRN"/b"CMT"/b"PHS"), v1~v4 = 임의 값(16진).
pub unsafe fn dump_step(tag: &[u8; 3], v1: u64, v2: u64, v3: u64, v4: u64) {
    if STEP_N.fetch_add(1, Ordering::Relaxed) >= 400 {
        return;
    }
    let h = LOG_H.load(Ordering::Relaxed);
    if h == INVALID_HANDLE || h == 0 {
        return;
    }
    let mut buf = [0u8; 96];
    let mut p = 0usize;
    macro_rules! put {
        ($b:expr) => {{
            for &c in ($b).iter() {
                if p < buf.len() {
                    buf[p] = c;
                    p += 1;
                }
            }
        }};
    }
    let mut h8 = [0u8; 8];
    put!(tag);
    for v in [v1, v2, v3, v4] {
        put!(b" ");
        hex32(v as u32, &mut h8);
        put!(&h8);
    }
    put!(b"\n");
    let mut w = 0u32;
    WriteFile(h, buf.as_ptr(), p as u32, &mut w, 0);
}

/// 권위 레코드 진행 덤프: last 레코드의 4벡터 len(t1ban/t2ban/t1pick/t2pick) + 권위 total
/// + 씬 액션수(applier). 씬 total과 권위 total이 벌어지면 = 커밋 거부 발생.
pub unsafe fn dump_auth(last: usize, atot: u64, applier: u64) {
    let h = LOG_H.load(Ordering::Relaxed);
    if h == INVALID_HANDLE || h == 0 {
        return;
    }
    let mut buf = [0u8; 200];
    let mut p = 0usize;
    macro_rules! put {
        ($b:expr) => {{
            for &c in ($b).iter() {
                if p < buf.len() {
                    buf[p] = c;
                    p += 1;
                }
            }
        }};
    }
    let mut h8 = [0u8; 8];
    put!(b"AUTH total=");
    hex32(atot as u32, &mut h8);
    put!(&h8);
    put!(b" applier=");
    hex32(applier as u32, &mut h8);
    put!(&h8);
    put!(b" vec(t1b.t2b.t1p.t2p)=");
    let mut hs = [0u8; 2];
    for &o in &[0x40usize, 0x58, 0x70, 0x88] {
        let l = core::ptr::read((last + o) as *const usize);
        hex8((l & 0xff) as u8, &mut hs);
        put!(&hs);
        put!(b".");
    }
    put!(b"\n");
    let mut w = 0u32;
    WriteFile(h, buf.as_ptr(), p as u32, &mut w, 0);
}

// ── Rust panic 단일 깔때기 훅 (크래시 위치 정확 포착) ──────────────────────
// 게임은 Rust panic=abort 빌드 → 모든 크래시(__fastfail int 0x29)는 반드시
// rust_panic_with_hook(0x25d4764)를 통과한다. r8 = &Location{file_ptr,file_len,line,col}.
// 여기서 file:line:col을 기록하면 panicmap CSV(14,666 사이트)와 대조해 즉시 특정 가능.
// 프롤로그 13B = `55 41 56 56 57 53 48 81 EC 80 00 00 00` (rip-rel 없음, 실측 확인).
const RVA_PANIC_HOOK: usize = 0x2a97074; // 0.5.4 0x29b6a64
const PANIC_STEAL: usize = 13;
static TRAMP_PANIC: AtomicUsize = AtomicUsize::new(0);
static PANIC_LOGGED: AtomicU32 = AtomicU32::new(0);

unsafe extern "win64" fn hook_panic(rcx: usize, rdx: usize, r8: usize, r9: usize) -> usize {
    // 최초 3건만 기록(연쇄 패닉 스팸 방지). 할당·락 없이 raw write.
    if PANIC_LOGGED.load(Ordering::Relaxed) < 3 {
        let h = LOG_H.load(Ordering::Relaxed);
        if h != INVALID_HANDLE && h != 0 && r8 >= 0x10000 {
            let n = PANIC_LOGGED.fetch_add(1, Ordering::Relaxed);
            let fptr = core::ptr::read(r8 as *const usize);
            let flen = core::ptr::read((r8 + 8) as *const usize);
            let line = core::ptr::read((r8 + 16) as *const u32);
            let col = core::ptr::read((r8 + 20) as *const u32);
            let mut buf = [0u8; 400];
            let mut p = 0usize;
            macro_rules! put {
                ($b:expr) => {{
                    for &c in ($b).iter() {
                        if p < buf.len() {
                            buf[p] = c;
                            p += 1;
                        }
                    }
                }};
            }
            let mut h8 = [0u8; 8];
            put!(b"PANIC#");
            hex32(n, &mut h8);
            put!(&h8);
            put!(b" at ");
            if fptr >= 0x10000 && flen > 0 && flen < 300 {
                for i in 0..flen {
                    let c = core::ptr::read((fptr + i) as *const u8);
                    if p < buf.len() {
                        buf[p] = c;
                        p += 1;
                    }
                }
            }
            put!(b":");
            hex32(line, &mut h8);
            put!(&h8);
            put!(b":");
            hex32(col, &mut h8);
            put!(&h8);
            put!(b" in_bp=");
            put!(if CTX_IN_BANPICK.load(Ordering::Relaxed) != 0 { b"1" } else { b"0" });
            put!(b"\n");
            let mut w = 0u32;
            WriteFile(h, buf.as_ptr(), p as u32, &mut w, 0);
        }
    }
    let stub = TRAMP_PANIC.load(Ordering::Relaxed);
    if stub == 0 {
        return 0;
    }
    let orig: unsafe extern "win64" fn(usize, usize, usize, usize) -> usize =
        core::mem::transmute(stub);
    orig(rcx, rdx, r8, r9)
}

unsafe fn install_panic_hook(base: usize) {
    let fn_addr = base + RVA_PANIC_HOOK;
    const EXPECT: [u8; 13] = [
        0x55, 0x41, 0x56, 0x56, 0x57, 0x53, 0x48, 0x81, 0xec, 0x80, 0x00, 0x00, 0x00,
    ];
    for (i, b) in EXPECT.iter().enumerate() {
        if core::ptr::read((fn_addr + i) as *const u8) != *b {
            return; // 프롤로그 불일치 → 설치 포기(안전)
        }
    }
    let stub = VirtualAlloc(0, 64, 0x1000 | 0x2000, 0x40);
    if stub == 0 {
        return;
    }
    let mut s = [0u8; 32];
    core::ptr::copy_nonoverlapping(fn_addr as *const u8, s.as_mut_ptr(), PANIC_STEAL);
    let mut q = PANIC_STEAL;
    s[q] = 0x49;
    s[q + 1] = 0xbb; // movabs r11, ret
    q += 2;
    let ret = fn_addr + PANIC_STEAL;
    core::ptr::copy_nonoverlapping(ret.to_le_bytes().as_ptr(), s.as_mut_ptr().add(q), 8);
    q += 8;
    s[q] = 0x41;
    s[q + 1] = 0xff;
    s[q + 2] = 0xe3; // jmp r11
    q += 3;
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, q);
    TRAMP_PANIC.store(stub, Ordering::Relaxed);

    let mut patch = [0x90u8; PANIC_STEAL];
    patch[0] = 0x48;
    patch[1] = 0xb8; // movabs rax, hook
    let t = hook_panic as usize;
    patch[2..10].copy_from_slice(&t.to_le_bytes());
    patch[10] = 0xff;
    patch[11] = 0xe0; // jmp rax
    let mut old: u32 = 0;
    if VirtualProtect(fn_addr, PANIC_STEAL, 0x40, &mut old) != 0 {
        core::ptr::copy_nonoverlapping(patch.as_ptr(), fn_addr as *mut u8, PANIC_STEAL);
        VirtualProtect(fn_addr, PANIC_STEAL, old, &mut old);
        FlushInstructionCache(GetCurrentProcess(), fn_addr, PANIC_STEAL);
    }
}

/// 1회 설치. base = 게임 모듈 핸들, dir = 모드 폴더(로그 경로).
pub fn install(base: usize, dir: &str) {
    if INSTALLED.swap(1, Ordering::Relaxed) != 0 {
        return;
    }
    BASE_D.store(base, Ordering::Relaxed);
    // 로그 파일 append 모드로 열기(WRITE_THROUGH = 크래시 후에도 디스크 보장)
    let path = format!("{dir}\\crash_log.txt");
    let mut w: Vec<u16> = path.encode_utf16().collect();
    w.push(0);
    unsafe {
        let h = CreateFileW(
            w.as_ptr(),
            FILE_APPEND | GENERIC_WRITE,
            FILE_SHARE_RW,
            0,
            OPEN_ALWAYS,
            FILE_FLAG_WRITE_THROUGH,
            0,
        );
        if h != INVALID_HANDLE {
            LOG_H.store(h, Ordering::Relaxed);
            SetFilePointer(h, 0, 0, 2); // FILE_END
            // 설치 마커 1줄
            let hdr = b"--- diag VEH installed ---\n";
            let mut written: u32 = 0;
            WriteFile(h, hdr.as_ptr(), hdr.len() as u32, &mut written, 0);
        }
        AddVectoredExceptionHandler(1, veh as usize); // first=1 = 리스트 맨 앞
        install_panic_hook(base); // Rust panic 단일 깔때기 → 크래시 위치(file:line) 기록
        // ⚠릴리스에서 제거: 전환함수 panic site int3(0xCC) 패치는 조사 전용이었다.
        //   게임 코드에 브레이크포인트를 심는 것이라 실사용에 부적합(조사 완료로 불요).
    }
}
