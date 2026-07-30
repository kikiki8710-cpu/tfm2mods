// sylas_hijack PROBE v1 (0.5.2 재규명) — clone/등록 지점 실측.
// 정적 마이그 6/6 NONE(0.5.1 sim 재아키텍처). ghidra-re 후보 seam에 계측 훅을 걸어
//   "id@+0x8/+0x10 + 궁desc@+0x4c8 을 가진 0x748 master record"를 인자로 받는 함수를 실측한다.
// 목표: (1) REG_54a800/DACTION_621a40 중 어느 것이 챔프 등록/clone 지점인가
//       (2) master 레이아웃(+0x8 id / +0x4c8 ult desc)이 0.5.2에서 유효한가
//       (3) 사일러스 등록 시 +0x4c8 이 궁 descriptor(-1 아님)인가
// 쓰기 전무 = 완전 안전(읽기·로깅만). 유저가 경기 1개 로드하면 로그가 나온다.
#![allow(non_snake_case, dead_code)]
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Mutex;
use std::collections::HashSet;
use std::time::Duration;
use mod_api::*;

// ── ghidra-re 후보 seam (0.5.2 RVA) ──
const REG_RVA: usize = 0x54a800;   // FUN_14054a800 champ 등록, rdx=master(추정)
const REG_SIG: [u8; 17] = [0x55,0x41,0x57,0x41,0x56,0x41,0x54,0x56,0x57,0x53,0x48,0x81,0xec,0x00,0x08,0x00,0x00];
const REG_LEN: usize = 17;
const DACT_RVA: usize = 0x621a40;  // FUN_140621a40 DataAction clone 디스패처
const DACT_SIG: [u8; 12] = [0x55,0x41,0x57,0x41,0x56,0x41,0x55,0x41,0x54,0x56,0x57,0x53];
const DACT_LEN: usize = 12;

const B_IDPTR: usize = 0x8;
const B_IDLEN: usize = 0x10;
const E_ULT_DESC: usize = 0x4c8;

type BOOL = i32;
type HMODULE = usize;
extern "system" {
    fn GetModuleHandleW(name: *const u16) -> HMODULE;
    fn VirtualAlloc(addr: usize, sz: usize, typ: u32, prot: u32) -> usize;
    fn VirtualProtect(addr: usize, size: usize, new_protect: u32, old: *mut u32) -> BOOL;
    fn VirtualQuery(addr: usize, buf: *mut MemBasicInfo, len: usize) -> usize;
    fn FlushInstructionCache(proc: usize, addr: usize, size: usize) -> BOOL;
    fn GetCurrentProcess() -> usize;
    fn AddVectoredExceptionHandler(first: u32, handler: VehHandler) -> usize;
}
#[repr(C)]
struct MemBasicInfo { base: usize, alloc_base: usize, alloc_prot: u32, _pad0: u32,
    region: usize, state: u32, protect: u32, typ: u32 }
#[repr(C)] struct ExceptionRecord { code: u32, flags: u32, _rec: usize, addr: usize, _np: u32, _pad: u32 }
#[repr(C)] struct ExceptionPointers { rec: *mut ExceptionRecord, ctx: usize }
type VehHandler = extern "system" fn(*mut ExceptionPointers) -> i32;

fn exe_base() -> usize { unsafe { GetModuleHandleW(core::ptr::null()) } }
fn rva_of(a: u64) -> u64 { let b = exe_base() as u64; if a >= b && a < b + 0x8000000 { a - b } else { a } }

unsafe fn readable(a: usize, len: usize) -> bool {
    if a < 0x10000 { return false; }
    let mut mbi: MemBasicInfo = core::mem::zeroed();
    if VirtualQuery(a, &mut mbi, core::mem::size_of::<MemBasicInfo>()) == 0 { return false; }
    if mbi.state != 0x1000 { return false; }
    const RPROT: u32 = 0x02|0x04|0x08|0x10|0x20|0x40|0x80;
    if mbi.protect & RPROT == 0 || mbi.protect & 0x101 != 0 { return false; }
    let end = mbi.base + mbi.region;
    a.wrapping_add(len) <= end
}

// ── VEH-safe 읽기 (기존 lib.rs 재사용) ──
core::arch::global_asm!(
    ".globl shp_rd8", ".globl shp_rd8_f", ".globl shp_rd8_l",
    ".globl shp_rd1", ".globl shp_rd1_f", ".globl shp_rd1_l",
    "shp_rd8:", "shp_rd8_f:", "mov rax, qword ptr [rcx]", "mov qword ptr [rdx], rax", "mov eax, 1", "ret",
    "shp_rd8_l:", "xor eax, eax", "ret",
    "shp_rd1:", "shp_rd1_f:", "movzx eax, byte ptr [rcx]", "mov byte ptr [rdx], al", "mov eax, 1", "ret",
    "shp_rd1_l:", "xor eax, eax", "ret",
);
extern "C" {
    fn shp_rd8(addr: usize, out: *mut u64) -> u32;
    fn shp_rd1(addr: usize, out: *mut u8) -> u32;
    static shp_rd8_f: u8; static shp_rd8_l: u8;
    static shp_rd1_f: u8; static shp_rd1_l: u8;
}
#[inline] unsafe fn rd_u64(a: usize) -> Option<u64> { if a < 0x10000 { return None; } let mut o=0u64; if shp_rd8(a,&mut o)!=0 {Some(o)} else {None} }
#[inline] unsafe fn rd_u8(a: usize)  -> Option<u8>  { if a < 0x10000 { return None; } let mut o=0u8;  if shp_rd1(a,&mut o)!=0 {Some(o)} else {None} }

static VEH_INSTALLED: AtomicBool = AtomicBool::new(false);
extern "system" fn shp_veh(p: *mut ExceptionPointers) -> i32 {
    const CONTINUE_EXECUTION: i32 = -1; const CONTINUE_SEARCH: i32 = 0;
    unsafe {
        if p.is_null() { return CONTINUE_SEARCH; }
        let rec = (*p).rec;
        if rec.is_null() || (*rec).code != 0xC0000005 { return CONTINUE_SEARCH; }
        let ctx = (*p).ctx;
        if ctx == 0 { return CONTINUE_SEARCH; }
        let rip = *((ctx + 0xF8) as *const u64) as usize;
        let land = if rip == core::ptr::addr_of!(shp_rd8_f) as usize { core::ptr::addr_of!(shp_rd8_l) as usize }
                   else if rip == core::ptr::addr_of!(shp_rd1_f) as usize { core::ptr::addr_of!(shp_rd1_l) as usize }
                   else { 0 };
        if land != 0 { *((ctx + 0xF8) as *mut u64) = land as u64; return CONTINUE_EXECUTION; }
        CONTINUE_SEARCH
    }
}
fn veh_install() { if VEH_INSTALLED.swap(true, Ordering::Relaxed) { return; } unsafe { AddVectoredExceptionHandler(1, shp_veh); } }

unsafe fn read_str_len(ptr: usize, len: usize) -> Option<String> {
    if ptr < 0x10000 || len < 2 || len > 40 { return None; }
    let mut buf = Vec::with_capacity(len);
    for i in 0..len {
        let b = rd_u8(ptr + i)?;
        if !(b == b'_' || b.is_ascii_alphanumeric()) { return None; }
        buf.push(b);
    }
    String::from_utf8(buf).ok()
}
unsafe fn read_id(rec: usize) -> Option<String> {
    let idptr = rd_u64(rec + B_IDPTR)? as usize;
    let idlen = rd_u64(rec + B_IDLEN)? as usize;
    read_str_len(idptr, idlen)
}

static WRITE_LOCK: Mutex<()> = Mutex::new(());
fn plog(s: &str) {
    use std::io::Write;
    let _g = WRITE_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true)
        .open("C:\\tfm2mods\\sylas_hijack\\probe052_log.txt") {
        let _ = f.write_all(s.as_bytes());
    }
}

// (site, id) dedup — 같은 챔프를 한 사이트에서 1회만 로깅
static SEEN: Mutex<Option<HashSet<(u8, String)>>> = Mutex::new(None);
static PLOG_N: AtomicU32 = AtomicU32::new(0);

// rcx/rdx/r8/r9 중 master record(id 유효 + +0x4c8 readable)인 것을 찾아 로깅.
unsafe fn probe_common(site: u8, saved: usize) {
    // saved = 트램폴린 진입 시 rsp. push 순서: rcx(+0x28),rdx(+0x20),r8(+0x18),r9(+0x10)
    let regs = [
        ("rcx", *((saved + 0x28) as *const u64) as usize),
        ("rdx", *((saved + 0x20) as *const u64) as usize),
        ("r8",  *((saved + 0x18) as *const u64) as usize),
        ("r9",  *((saved + 0x10) as *const u64) as usize),
    ];
    for (rn, p) in regs {
        if p < 0x10000 { continue; }
        let id = match read_id(p) { Some(v) => v, None => continue };
        // 0x748 master 후보: +0x4c8 궁 desc 읽힘 여부 + 값
        let desc_ok = readable(p + E_ULT_DESC, 8);
        let desc0 = rd_u64(p + E_ULT_DESC).unwrap_or(0xDEAD);
        let is_new = {
            let mut g = SEEN.lock().unwrap_or_else(|e| e.into_inner());
            if g.is_none() { *g = Some(HashSet::new()); }
            g.as_mut().unwrap().insert((site, id.clone()))
        };
        if is_new && PLOG_N.fetch_add(1, Ordering::Relaxed) < 400 {
            let sname = if site == 0 { "REG_54a800" } else { "DACT_621a40" };
            let ult = if desc0 == u64::MAX { "없음(-1)" } else if desc0 == 0xDEAD { "unread" } else { "★있음" };
            plog(&format!("[{}] {}={:#x} id=\"{}\" +0x4c8_readable={} +0x4c8={:#x} 궁={}\n",
                sname, rn, p, id, desc_ok, desc0, ult));
        }
    }
}

unsafe fn install_probe(rva: usize, hook_len: usize, expect: &[u8], cap_fn: usize) -> Result<usize, String> {
    let mbase = exe_base();
    if mbase == 0 { return Err("module 0".into()); }
    let fn_addr = mbase + rva;
    if !readable(fn_addr, hook_len + 8) { return Err(format!("fn {:#x} unreadable", fn_addr)); }
    let mut cur = [0u8; 24];
    core::ptr::copy_nonoverlapping(fn_addr as *const u8, cur.as_mut_ptr(), 24);
    if &cur[..expect.len()] != expect {
        return Err(format!("프롤로그 불일치 rva={:#x} 실제={:02x?}", rva, &cur[..expect.len().min(24)]));
    }
    const MEM_CR: u32 = 0x1000 | 0x2000; const RWX: u32 = 0x40;
    let stub = VirtualAlloc(0, 256, MEM_CR, RWX);
    if stub == 0 { return Err("VirtualAlloc".into()); }
    let ret_addr = fn_addr + hook_len;
    let mut s: Vec<u8> = Vec::new();
    s.extend_from_slice(&[0x49,0x89,0xe2]);                                    // mov r10, rsp
    s.extend_from_slice(&[0x51,0x52,0x41,0x50,0x41,0x51,0x41,0x52,0x41,0x53]); // push rcx,rdx,r8,r9,r10,r11
    s.extend_from_slice(&[0x48,0x89,0xe1]);                                    // mov rcx, rsp (saved)
    s.extend_from_slice(&[0x48,0x83,0xec,0x28]);                              // sub rsp,0x28
    s.extend_from_slice(&[0x48,0xb8]); s.extend_from_slice(&cap_fn.to_le_bytes()); // mov rax, cap_fn
    s.extend_from_slice(&[0xff,0xd0]);                                        // call rax
    s.extend_from_slice(&[0x48,0x83,0xc4,0x28]);                              // add rsp,0x28
    s.extend_from_slice(&[0x41,0x5b,0x41,0x5a,0x41,0x59,0x41,0x58,0x5a,0x59]); // pop r11..rcx
    let mut orig = vec![0u8; hook_len];
    core::ptr::copy_nonoverlapping(fn_addr as *const u8, orig.as_mut_ptr(), hook_len);
    s.extend_from_slice(&orig);                                               // 원본 프롤로그
    s.extend_from_slice(&[0x48,0xb8]); s.extend_from_slice(&ret_addr.to_le_bytes()); // mov rax, ret_addr
    s.extend_from_slice(&[0xff,0xe0]);                                        // jmp rax
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    let mut patch = vec![0x90u8; hook_len];
    patch[0]=0x48; patch[1]=0xb8; patch[2..10].copy_from_slice(&stub.to_le_bytes()); patch[10]=0xff; patch[11]=0xe0;
    let mut old: u32 = 0;
    if VirtualProtect(fn_addr, hook_len, RWX, &mut old) == 0 { return Err("VirtualProtect".into()); }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), fn_addr as *mut u8, hook_len);
    VirtualProtect(fn_addr, hook_len, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), fn_addr, hook_len);
    Ok(stub)
}

unsafe extern "C" fn cap_reg(saved: usize, _e: usize) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| probe_common(0, saved)));
}
unsafe extern "C" fn cap_dact(saved: usize, _e: usize) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| probe_common(1, saved)));
}

static HOOK_INSTALLED: AtomicBool = AtomicBool::new(false);
fn init(_ctx: &GameCtx) -> ModRegistration {
    std::thread::spawn(|| {
        std::thread::sleep(Duration::from_secs(6));
        if HOOK_INSTALLED.swap(true, Ordering::Relaxed) { return; }
        veh_install();
        let base = exe_base();
        plog(&format!("\n===== sylas_hijack PROBE v1 (0.5.2 재규명) 시작 base={:#x} =====\n", base));
        match unsafe { install_probe(REG_RVA, REG_LEN, &REG_SIG, cap_reg as *const () as usize) } {
            Ok(stub) => plog(&format!("[install] REG_54a800 OK @{:#x} stub={:#x}\n", base + REG_RVA, stub)),
            Err(e)   => plog(&format!("[install] REG_54a800 실패: {}\n", e)),
        }
        match unsafe { install_probe(DACT_RVA, DACT_LEN, &DACT_SIG, cap_dact as *const () as usize) } {
            Ok(stub) => plog(&format!("[install] DACT_621a40 OK @{:#x} stub={:#x}\n", base + DACT_RVA, stub)),
            Err(e)   => plog(&format!("[install] DACT_621a40 실패: {}\n", e)),
        }
    });
    ModRegistration::new("sylas_hijack")
}

declare_mod!(init);
