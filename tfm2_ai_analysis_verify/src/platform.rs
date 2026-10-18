//! Windows x64 전용 최소 플랫폼 계층. 게임/모드의 일반 예외를 복구하지 않는다.
//! 안전 읽기는 현재 페이지를 검사하고 고정 load 한 명령의 fault만 VEH로 처리한다.
//! 읽는 사이 변경/해제될 수 있으므로 실패한 snapshot은 호출자가 전부 버려야 한다.
use std::ffi::c_void;
use std::path::PathBuf;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicU8, AtomicU64, AtomicUsize, Ordering};

type Handle = *mut c_void;
const MEM_COMMIT: u32 = 0x1000;
const MEM_RESERVE: u32 = 0x2000;
const MEM_RELEASE: u32 = 0x8000;
const PAGE_NOACCESS: u32 = 1;
const PAGE_READWRITE: u32 = 4;
const PAGE_EXECUTE_READ: u32 = 0x20;
const PAGE_GUARD: u32 = 0x100;
const MAX_READ: usize = 16 * 1024 * 1024;
const MAX_THREADS: usize = 1024;

#[repr(C)]
#[derive(Default)]
struct MemoryInfo {
    base: usize, allocation_base: usize, allocation_protect: u32, alignment1: u32,
    region_size: usize, state: u32, protect: u32, kind: u32, alignment2: u32,
}
#[repr(C)]
struct ExceptionRecord {
    code: u32, flags: u32, nested: usize, address: usize,
    number_parameters: u32, alignment: u32, information: [usize; 15],
}
#[repr(C)]
struct ExceptionPointers { record: *mut ExceptionRecord, context: *mut u8 }
#[repr(C)]
#[derive(Clone, Copy, Default)]
struct ThreadEntry {
    size: u32, usage: u32, id: u32, owner_pid: u32,
    base_priority: i32, delta_priority: i32, flags: u32,
}
// AMD64 CONTEXT의 Windows SDK 크기/정렬. CONTROL만 요청한다.
#[repr(C, align(16))]
struct Context([u8; 1232]);

#[link(name = "kernel32")]
extern "system" {
    fn VirtualQuery(address: *const c_void, info: *mut MemoryInfo, size: usize) -> usize;
    fn VirtualProtect(address: *mut c_void, len: usize, protect: u32, old: *mut u32) -> i32;
    fn VirtualAlloc(address: *mut c_void, len: usize, allocation: u32, protect: u32) -> *mut c_void;
    fn VirtualFree(address: *mut c_void, len: usize, free_type: u32) -> i32;
    fn FlushInstructionCache(process: Handle, address: *const c_void, len: usize) -> i32;
    fn WriteProcessMemory(process: Handle, address: *mut c_void, source: *const c_void, len: usize, written: *mut usize) -> i32;
    fn GetCurrentProcess() -> Handle;
    fn GetCurrentProcessId() -> u32;
    fn GetCurrentThreadId() -> u32;
    fn IsDebuggerPresent() -> i32;
    fn GetModuleHandleW(name: *const u16) -> Handle;
    fn GetModuleHandleExW(flags: u32, address: *const u16, module: *mut Handle) -> i32;
    fn GetModuleFileNameW(module: Handle, buffer: *mut u16, capacity: u32) -> u32;
    fn AddVectoredExceptionHandler(first: u32, handler: unsafe extern "system" fn(*mut ExceptionPointers) -> i32) -> Handle;
    fn CreateToolhelp32Snapshot(flags: u32, pid: u32) -> Handle;
    fn Thread32First(snapshot: Handle, entry: *mut ThreadEntry) -> i32;
    fn Thread32Next(snapshot: Handle, entry: *mut ThreadEntry) -> i32;
    fn GetLastError() -> u32;
    fn OpenThread(access: u32, inherit: i32, id: u32) -> Handle;
    fn GetProcessIdOfThread(thread: Handle) -> u32;
    fn GetThreadId(thread: Handle) -> u32;
    fn SuspendThread(thread: Handle) -> u32;
    fn ResumeThread(thread: Handle) -> u32;
    fn GetThreadContext(thread: Handle, context: *mut Context) -> i32;
    fn CloseHandle(handle: Handle) -> i32;
}

#[cfg(all(target_os = "windows", target_arch = "x86_64"))]
std::arch::global_asm!(
    ".text",
    ".global tfm2_aiav_read8", ".global tfm2_aiav_read8_fault", ".global tfm2_aiav_read8_land",
    "tfm2_aiav_read8:", "tfm2_aiav_read8_fault:",
    "mov rax, qword ptr [rcx]", "mov qword ptr [rdx], rax", "mov eax, 1", "ret",
    "tfm2_aiav_read8_land:", "xor eax, eax", "ret",
    ".global tfm2_aiav_read4", ".global tfm2_aiav_read4_fault", ".global tfm2_aiav_read4_land",
    "tfm2_aiav_read4:", "tfm2_aiav_read4_fault:",
    "mov eax, dword ptr [rcx]", "mov dword ptr [rdx], eax", "mov eax, 1", "ret",
    "tfm2_aiav_read4_land:", "xor eax, eax", "ret",
    ".global tfm2_aiav_read1", ".global tfm2_aiav_read1_fault", ".global tfm2_aiav_read1_land",
    "tfm2_aiav_read1:", "tfm2_aiav_read1_fault:",
    "mov al, byte ptr [rcx]", "mov byte ptr [rdx], al", "mov eax, 1", "ret",
    "tfm2_aiav_read1_land:", "xor eax, eax", "ret",
);
extern "C" {
    fn tfm2_aiav_read8(address: usize, out: *mut u64) -> i32;
    fn tfm2_aiav_read4(address: usize, out: *mut u32) -> i32;
    fn tfm2_aiav_read1(address: usize, out: *mut u8) -> i32;
    fn tfm2_aiav_read8_fault(); fn tfm2_aiav_read8_land();
    fn tfm2_aiav_read4_fault(); fn tfm2_aiav_read4_land();
    fn tfm2_aiav_read1_fault(); fn tfm2_aiav_read1_land();
}

// 자기 INT3만 원본으로 우회한다. 슬롯/트램폴린은 최초 CC 게시 뒤 영구 보존한다.
// entry=0은 빈 슬롯, MAX는 게시 중이다. handler에는 할당/락/대기/게임 CALL이 없다.
const MAX_BREAKPOINT_GATES: usize = 64;
struct BreakpointGate { entry: AtomicUsize, original: AtomicUsize, hits: AtomicU64 }
static BREAKPOINT_GATES: [BreakpointGate; MAX_BREAKPOINT_GATES] = [const {
    BreakpointGate { entry: AtomicUsize::new(0), original: AtomicUsize::new(0), hits: AtomicU64::new(0) }
}; MAX_BREAKPOINT_GATES];
pub(crate) fn debugger_present() -> bool { unsafe { IsDebuggerPresent() != 0 } }
pub(crate) fn register_breakpoint_gate(entry: usize, original: usize) -> Result<(), &'static str> {
    checked_end(entry, 1)?;
    executable_range(original, 1)?;
    init_safe_reader()?;
    if BREAKPOINT_GATES.iter().any(|g| g.entry.load(Ordering::Acquire) == entry) {
        return Err("breakpoint gate entry already registered");
    }
    for gate in &BREAKPOINT_GATES {
        if gate.entry.compare_exchange(0, usize::MAX, Ordering::AcqRel, Ordering::Acquire).is_ok() {
            gate.original.store(original, Ordering::Relaxed);
            gate.entry.store(entry, Ordering::Release);
            return Ok(());
        }
    }
    Err("permanent breakpoint gate capacity exhausted")
}
pub(crate) fn breakpoint_gate_hits(entry: usize) -> u64 {
    BREAKPOINT_GATES.iter().find(|g| g.entry.load(Ordering::Acquire) == entry)
        .map_or(0, |g| g.hits.load(Ordering::Relaxed))
}
unsafe fn handle_breakpoint(p: &ExceptionPointers, rec: &ExceptionRecord) -> bool {
    if rec.code != 0x80000003 || rec.flags & 1 != 0 { return false; }
    // ExceptionAddress로 고른다. 지연 dispatch는 first byte가 이미 복원된 뒤에도 온다.
    for gate in &BREAKPOINT_GATES {
        let entry = gate.entry.load(Ordering::Acquire);
        if entry != 0 && entry != usize::MAX && entry == rec.address {
            let rip = std::ptr::read_unaligned(p.context.add(0xf8) as *const usize);
            if rip != entry && entry.checked_add(1) != Some(rip) { return false; }
            let original = gate.original.load(Ordering::Relaxed);
            if original == 0 { return false; }
            // CONTEXT.Rip만 변경. Rsp/GPR/SIMD/EFlags 및 TF/RF는 보존한다.
            std::ptr::write_unaligned(p.context.add(0xf8) as *mut usize, original);
            gate.hits.fetch_add(1, Ordering::Relaxed);
            return true;
        }
    }
    false
}

// fault 명령은 load만이다. out store/임의 게임 RIP의 예외는 절대로 넘겨받지 않는다.
unsafe extern "system" fn read_exception(pointers: *mut ExceptionPointers) -> i32 {
    if pointers.is_null() { return 0; }
    let p = &*pointers;
    if p.record.is_null() || p.context.is_null() { return 0; }
    let rec = &*p.record;
    if handle_breakpoint(p, rec) { return -1; }
    if rec.code != 0xc0000005 && rec.code != 0x80000001 { return 0; }
    let rip_slot = p.context.add(0xf8) as *mut usize;
    let rip = std::ptr::read_unaligned(rip_slot);
    let landing = if rip == tfm2_aiav_read8_fault as *const () as usize {
        tfm2_aiav_read8_land as *const () as usize
    } else if rip == tfm2_aiav_read4_fault as *const () as usize {
        tfm2_aiav_read4_land as *const () as usize
    } else if rip == tfm2_aiav_read1_fault as *const () as usize {
        tfm2_aiav_read1_land as *const () as usize
    } else { return 0; };
    if rec.code == 0x80000001 && rec.number_parameters >= 2 {
        // 사전 검사 후 guard가 붙은 경쟁만 해당한다. 소비된 guard를 최선 노력으로 복원.
        // 다른 소유자의 동시 protection 변경을 완전히 중재한다는 보장은 없다.
        let mut info = MemoryInfo::default();
        let fault = rec.information[1];
        if VirtualQuery(fault as *const c_void, &mut info, std::mem::size_of::<MemoryInfo>()) != 0
            && info.state == MEM_COMMIT && info.protect & PAGE_NOACCESS == 0 && info.protect != 0 {
            let mut old = 0;
            // VirtualProtect는 이 1바이트가 속한 페이지만 반올림한다. 합쳐진 전체
            // VirtualQuery region에 guard를 새로 붙이지 않는다.
            VirtualProtect(fault as *mut c_void, 1, info.protect | PAGE_GUARD, &mut old);
        }
    }
    std::ptr::write_unaligned(rip_slot, landing);
    -1 // EXCEPTION_CONTINUE_EXECUTION
}

static READER: OnceLock<Result<usize, &'static str>> = OnceLock::new();
pub fn init_safe_reader() -> Result<(), &'static str> {
    match READER.get_or_init(|| {
        // VEH/영구 descriptor 코드의 물리 unload를 막는다. 성공 시 process lifetime pin이다.
        pin_own_module(read_exception as *const () as usize)?;
        let h = unsafe { AddVectoredExceptionHandler(1, read_exception) };
        if h.is_null() { Err("safe_read: AddVectoredExceptionHandler failed") } else { Ok(h as usize) }
    }) { Ok(_) => Ok(()), Err(e) => Err(*e) }
}

fn checked_end(address: usize, len: usize) -> Result<usize, &'static str> {
    let end = address.checked_add(len).ok_or("address range overflow")?;
    // 이 실험은 일반 Windows 사용자 주소만 지원하며 null/저주소는 거절한다.
    if address < 0x10000 || end > 0x0000_8000_0000_0000 { return Err("unsupported user address range"); }
    Ok(end)
}
fn checked_pages(address: usize, len: usize, executable: bool) -> Result<(), &'static str> {
    let end = checked_end(address, len)?;
    let mut at = address;
    while at < end {
        let mut info = MemoryInfo::default();
        let got = unsafe { VirtualQuery(at as *const c_void, &mut info, std::mem::size_of::<MemoryInfo>()) };
        if got != std::mem::size_of::<MemoryInfo>() || info.state != MEM_COMMIT {
            return Err("uncommitted or unqueryable page");
        }
        let mode = info.protect & 0xff;
        if info.protect & PAGE_GUARD != 0 || mode == 0 || mode == PAGE_NOACCESS {
            return Err("guard/noaccess page refused");
        }
        if executable {
            if !matches!(mode, 0x10 | 0x20 | 0x40 | 0x80) { return Err("non-executable page"); }
        } else if !matches!(mode, 0x02 | 0x04 | 0x08 | 0x20 | 0x40 | 0x80) {
            return Err("non-readable page");
        }
        let next = info.base.checked_add(info.region_size).ok_or("page extent overflow")?;
        if next <= at { return Err("invalid page extent"); }
        at = next.min(end);
    }
    Ok(())
}
pub fn executable_range(address: usize, len: usize) -> Result<(), &'static str> {
    if len == 0 { return Err("empty executable range"); }
    checked_pages(address, len, true)
}
pub fn safe_read_bytes(address: usize, out: &mut [u8]) -> Result<(), &'static str> {
    if out.is_empty() { return Ok(()); }
    if out.len() > MAX_READ { return Err("safe_read capture limit exceeded"); }
    init_safe_reader()?;
    checked_pages(address, out.len(), false)?;
    let mut n = 0;
    while n + 8 <= out.len() {
        let mut word = 0u64;
        if unsafe { tfm2_aiav_read8(address + n, &mut word) } == 0 { return Err("safe_read load fault"); }
        out[n..n + 8].copy_from_slice(&word.to_le_bytes()); n += 8;
    }
    if n + 4 <= out.len() {
        let mut word = 0u32;
        if unsafe { tfm2_aiav_read4(address + n, &mut word) } == 0 { return Err("safe_read load fault"); }
        out[n..n + 4].copy_from_slice(&word.to_le_bytes()); n += 4;
    }
    while n < out.len() {
        let mut byte = 0u8;
        if unsafe { tfm2_aiav_read1(address + n, &mut byte) } == 0 { return Err("safe_read load fault"); }
        out[n] = byte; n += 1;
    }
    Ok(())
}
pub fn safe_read_u8(address: usize) -> Result<u8, &'static str> {
    let mut out = [0; 1]; safe_read_bytes(address, &mut out)?; Ok(out[0])
}
pub fn safe_read_u32(address: usize) -> Result<u32, &'static str> {
    let mut out = [0; 4]; safe_read_bytes(address, &mut out)?; Ok(u32::from_le_bytes(out))
}
pub fn safe_read_u64(address: usize) -> Result<u64, &'static str> {
    let mut out = [0; 8]; safe_read_bytes(address, &mut out)?; Ok(u64::from_le_bytes(out))
}
pub fn module_base() -> Result<usize, &'static str> {
    let h = unsafe { GetModuleHandleW(std::ptr::null()) };
    if h.is_null() { Err("GetModuleHandleW failed") } else { Ok(h as usize) }
}
fn path_for_module(module: Handle) -> Result<PathBuf, &'static str> {
    use std::os::windows::ffi::OsStringExt;
    let mut buffer = vec![0u16; 32768];
    let n = unsafe { GetModuleFileNameW(module, buffer.as_mut_ptr(), buffer.len() as u32) } as usize;
    if n == 0 || n >= buffer.len() { return Err("GetModuleFileNameW failed/truncated"); }
    Ok(PathBuf::from(std::ffi::OsString::from_wide(&buffer[..n])))
}
pub fn module_path() -> Result<PathBuf, &'static str> { path_for_module(std::ptr::null_mut()) }
/// GET_MODULE_HANDLE_EX_FLAG_PIN | FROM_ADDRESS. 해제하거나 hot unload하지 않는다.
pub(crate) fn pin_own_module(address: usize) -> Result<(), &'static str> {
    let mut module = std::ptr::null_mut();
    if unsafe { GetModuleHandleExW(0x00000001 | 0x00000004, address as *const u16, &mut module) } == 0
        || module.is_null() {
        return Err("permanent module pin failed");
    }
    Ok(())
}
pub fn own_module_path(address: usize) -> Result<PathBuf, &'static str> {
    let mut module = std::ptr::null_mut();
    // FROM_ADDRESS | UNCHANGED_REFCOUNT: 주소가 속한 DLL의 실제 로드 경로.
    if unsafe { GetModuleHandleExW(0x00000004 | 0x00000002, address as *const u16, &mut module) } == 0 {
        return Err("GetModuleHandleExW failed");
    }
    path_for_module(module)
}

/// 설치 전용. 반환 RX 메모리는 게시 이후 process lifetime 동안 유지한다.
pub(crate) fn allocate_code(bytes: &[u8]) -> Result<usize, &'static str> {
    if bytes.is_empty() { return Err("empty trampoline"); }
    let p = unsafe { VirtualAlloc(std::ptr::null_mut(), bytes.len(), MEM_COMMIT | MEM_RESERVE, PAGE_READWRITE) };
    if p.is_null() { return Err("trampoline VirtualAlloc failed"); }
    unsafe { std::ptr::copy_nonoverlapping(bytes.as_ptr(), p as *mut u8, bytes.len()); }
    let mut old = 0;
    let protected = unsafe { VirtualProtect(p, bytes.len(), PAGE_EXECUTE_READ, &mut old) } != 0;
    let flushed = protected && unsafe { FlushInstructionCache(GetCurrentProcess(), p, bytes.len()) } != 0;
    if !flushed {
        unsafe { VirtualFree(p, 0, MEM_RELEASE); }
        return Err("trampoline RX/FlushInstructionCache failed");
    }
    Ok(p as usize)
}
pub(crate) unsafe fn free_unpublished_code(address: usize) { VirtualFree(address as *mut c_void, 0, MEM_RELEASE); }

pub(crate) struct PatchOutcome { pub attempted: bool, pub result: Result<(), &'static str> }
/// 성공 여부와 별개로 attempted=true면 original/stub을 절대 철회하면 안 된다.
pub(crate) unsafe fn write_code(address: usize, patch: &[u8]) -> PatchOutcome {
    // 한 protection region 안의 패치만 허용한다. 여러 페이지의 서로 다른 원래
    // protection을 첫 페이지 값 하나로 복원하는 실수를 피한다.
    let mut info = MemoryInfo::default();
    let extent = address.checked_add(patch.len());
    let queried = VirtualQuery(address as *const c_void, &mut info, std::mem::size_of::<MemoryInfo>());
    if queried != std::mem::size_of::<MemoryInfo>() || extent.is_none()
        || info.base.checked_add(info.region_size).map_or(true, |end| extent.unwrap_or(usize::MAX) > end)
        || info.state != MEM_COMMIT || info.protect & (PAGE_GUARD | PAGE_NOACCESS) != 0 {
        return PatchOutcome { attempted: false, result: Err("patch protection region changed/refused before write") };
    }
    let mut old = 0;
    if VirtualProtect(address as *mut c_void, patch.len(), 0x40, &mut old) == 0 {
        return PatchOutcome { attempted: false, result: Err("patch VirtualProtect failed before write") };
    }
    let mut written = 0usize;
    let ok = WriteProcessMemory(GetCurrentProcess(), address as *mut c_void, patch.as_ptr() as *const c_void, patch.len(), &mut written) != 0;
    let mut ignored = 0;
    let restored = VirtualProtect(address as *mut c_void, patch.len(), old, &mut ignored) != 0;
    let flushed = FlushInstructionCache(GetCurrentProcess(), address as *const c_void, patch.len()) != 0;
    let result = if !ok || written != patch.len() { Err("patch write failed/partial; original retained") }
        else if !restored { Err("patch protection restore failed; original retained") }
        else if !flushed { Err("patch instruction cache flush failed; original retained") }
        else { Ok(()) };
    PatchOutcome { attempted: true, result }
}

/// 코드 첫 byte만 원자적으로 교환한다. descriptor/원본은 호출 전에 게시해야 한다.
/// 성공한 교환 뒤 protection/flush 실패도 attempted=true: 안전 rollback으로 해석하지 않는다.
pub(crate) unsafe fn publish_code_byte(address: usize, expected: u8, byte: u8) -> PatchOutcome {
    if executable_range(address, 1).is_err() {
        return PatchOutcome { attempted: false, result: Err("atomic code byte page refused") };
    }
    let mut old = 0;
    if VirtualProtect(address as *mut c_void, 1, 0x40, &mut old) == 0 {
        return PatchOutcome { attempted: false, result: Err("atomic code byte VirtualProtect failed") };
    }
    let changed = (&*(address as *const AtomicU8)).compare_exchange(expected, byte, Ordering::SeqCst, Ordering::SeqCst).is_ok();
    let mut ignored = 0;
    let restored = VirtualProtect(address as *mut c_void, 1, old, &mut ignored) != 0;
    let flushed = FlushInstructionCache(GetCurrentProcess(), address as *const c_void, 1) != 0;
    let result = if !changed { Err("atomic code byte differs from expected") }
        else if !restored { Err("atomic code byte protection restore failed; entry unverified") }
        else if !flushed { Err("atomic code byte cache flush failed; entry unverified") }
        else { Ok(()) };
    PatchOutcome { attempted: changed, result }
}

struct OwnedHandle(Handle);
impl Drop for OwnedHandle { fn drop(&mut self) { if !self.0.is_null() { unsafe { CloseHandle(self.0); } } } }
fn thread_ids(out: &mut Vec<u32>) -> Result<(), &'static str> {
    out.clear();
    let raw = unsafe { CreateToolhelp32Snapshot(4, 0) };
    if raw as usize == usize::MAX { return Err("thread snapshot failed"); }
    let snapshot = OwnedHandle(raw);
    let pid = unsafe { GetCurrentProcessId() };
    let mut entry = ThreadEntry { size: std::mem::size_of::<ThreadEntry>() as u32, ..ThreadEntry::default() };
    if unsafe { Thread32First(snapshot.0, &mut entry) } == 0 { return Err("Thread32First failed"); }
    loop {
        if entry.owner_pid == pid {
            // 호출자는 정지 전에 용량을 확보한다. 정지 중에는 재할당하지 않는다.
            if out.len() == out.capacity() { return Err("thread list exceeds fixed capacity"); }
            out.push(entry.id);
        }
        entry.size = std::mem::size_of::<ThreadEntry>() as u32;
        if unsafe { Thread32Next(snapshot.0, &mut entry) } == 0 {
            if unsafe { GetLastError() } != 18 { return Err("Thread32Next failed"); }
            break;
        }
    }
    out.sort_unstable();
    Ok(())
}
struct StoppedThread { handle: OwnedHandle, suspended: bool }
/// 기존 스레드의 prefix 내 RIP를 검출하는 보수 장벽. 단독으로 새 진입을 막지 않는다.
/// 설치자는 먼저 INT3/영구 원본우회를 게시한다. SuspendThread의 OS/loader 락 위험은 남는다.
pub(crate) struct SuspendedThreads { threads: Vec<StoppedThread> }
impl SuspendedThreads {
    pub(crate) fn acquire(address: usize, patch_len: usize) -> Result<Self, &'static str> {
        let end = checked_end(address, patch_len)?;
        let mut first = Vec::with_capacity(MAX_THREADS);
        let mut second = Vec::with_capacity(MAX_THREADS);
        let mut after = Vec::with_capacity(MAX_THREADS);
        thread_ids(&mut first)?;
        thread_ids(&mut second)?;
        if first != second { return Err("thread list unstable before suspension"); }
        let current = unsafe { GetCurrentThreadId() };
        if !first.contains(&current) { return Err("installer absent from thread snapshot"); }
        let mut guard = Self { threads: Vec::with_capacity(first.len()) };
        // 모든 handle/Vec/CONTEXT를 정지 전에 준비한다.
        for id in first.iter().copied().filter(|id| *id != current) {
            let handle = unsafe { OpenThread(0x0002 | 0x0008 | 0x0040, 0, id) };
            if handle.is_null() { return Err("OpenThread failed; refusing patch"); }
            let owned = OwnedHandle(handle);
            if unsafe { GetProcessIdOfThread(handle) } != unsafe { GetCurrentProcessId() }
                || unsafe { GetThreadId(handle) } != id {
                return Err("thread identity/owner changed; refusing patch");
            }
            guard.threads.push(StoppedThread { handle: owned, suspended: false });
        }
        let mut context = Context([0; 1232]);
        for thread in &mut guard.threads {
            if unsafe { SuspendThread(thread.handle.0) } == u32::MAX { return Err("SuspendThread failed; refusing patch"); }
            thread.suspended = true;
            context.0.fill(0);
            context.0[0x30..0x34].copy_from_slice(&0x0010_0001u32.to_le_bytes());
            if unsafe { GetThreadContext(thread.handle.0, &mut context) } == 0 { return Err("GetThreadContext failed; refusing patch"); }
            let rip = u64::from_le_bytes(context.0[0xf8..0x100].try_into().unwrap()) as usize;
            if rip >= address && rip < end { return Err("thread RIP inside patch range; refusing patch"); }
        }
        // 재할당 없는 추가 검사. 이 검사 뒤 만들어지는 새 스레드는 잔여 경쟁이다.
        thread_ids(&mut after)?;
        if first != after { return Err("thread list changed during suspension; refusing patch"); }
        Ok(guard)
    }
    pub(crate) fn resume_all(&mut self) -> Result<(), &'static str> {
        let mut failed = false;
        for thread in self.threads.iter_mut().rev() {
            if thread.suspended {
                if unsafe { ResumeThread(thread.handle.0) } == u32::MAX { failed = true; }
                else { thread.suspended = false; }
            }
        }
        if failed { Err("ResumeThread failed; manual diagnosis required") } else { Ok(()) }
    }
}
impl Drop for SuspendedThreads { fn drop(&mut self) { let _ = self.resume_all(); } }

#[cfg(test)]
mod tests {
    use super::*;

    // 이 시험은 VirtualQuery 사전 거절 경로다. 실제 load fault/VEH landing 실행
    // 또는 외부 protection 변경 경쟁을 재현한 검증으로 세지 않는다.
    struct Page(*mut c_void);
    impl Page {
        fn new() -> Self {
            let p = unsafe { VirtualAlloc(std::ptr::null_mut(), 4096, MEM_COMMIT | MEM_RESERVE, PAGE_READWRITE) };
            assert!(!p.is_null());
            unsafe { *(p as *mut u8) = 0x5a; }
            Self(p)
        }
        fn protect(&self, mode: u32) {
            let mut old = 0;
            assert_ne!(unsafe { VirtualProtect(self.0, 4096, mode, &mut old) }, 0);
        }
    }
    impl Drop for Page {
        fn drop(&mut self) { unsafe { VirtualFree(self.0, 0, MEM_RELEASE); } }
    }
    #[test]
    fn address_overflow_is_rejected_without_a_load() {
        assert!(checked_end(usize::MAX - 2, 8).is_err());
        assert!(checked_end(0xffff, 1).is_err());
        let mut output = [0xa5; 8];
        assert!(safe_read_bytes(usize::MAX - 2, &mut output).is_err());
        assert_eq!(output, [0xa5; 8]);
    }
    #[test]
    fn noaccess_page_is_rejected_without_a_load() {
        let p = Page::new();
        assert_eq!(safe_read_u8(p.0 as usize), Ok(0x5a));
        p.protect(PAGE_NOACCESS);
        let mut output = [0xa5];
        assert!(safe_read_bytes(p.0 as usize, &mut output).is_err());
        assert_eq!(output, [0xa5]);
        assert!(executable_range(p.0 as usize, 1).is_err());
    }
    #[test]
    fn guard_page_is_rejected_and_guard_is_not_consumed() {
        let p = Page::new();
        p.protect(PAGE_READWRITE | PAGE_GUARD);
        let mut output = [0xa5];
        assert!(safe_read_bytes(p.0 as usize, &mut output).is_err());
        assert_eq!(output, [0xa5]);
        let mut info = MemoryInfo::default();
        assert_eq!(unsafe { VirtualQuery(p.0, &mut info, std::mem::size_of::<MemoryInfo>()) }, std::mem::size_of::<MemoryInfo>());
        assert_ne!(info.protect & PAGE_GUARD, 0);
    }
}
