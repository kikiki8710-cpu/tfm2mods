// sylas_hijack PROBE v2 (방법2 = 네이티브 apply 실행, 0.5.2) — datachamp apply 계측.
// ghidra-re 확정(0.5.2, 07-26):
//   datachamp descriptor vtable = 0x3876088 (size 0x1a8), slot26(+0xd0)=apply=0x1fe5cc0.
//   apply 5인자: rcx=descriptor / rdx=caster_handle / r8=sim_state / r9=target_ctx / [rsp+0x28]=casting_ctx.
//   caster resolve = call [sim_state+0x1b8] (rcx=caster_handle). apply: cmp[this+0x48],-1 궁게이트 → exec 0x2004d90.
// 목적(쓰기 없음=안전):
//   ① datachamp apply가 얼마나·어떤 인자로 불리는가 (descriptor/handle/sim_state 형태)
//   ② sim_state+0x1b8 resolve 함수 정체(RVA)
//   ③ descriptor 레이아웃(vtable/+0x48 궁게이트/action 데이터 +0x180..)
//   ④ (cfg probe_resolve=1, 게이트) resolve CALL → entity → name 실측 = caster 식별 검증
// 이걸로 "방법2 판별 = caster 식별"을 배선하기 전 근거를 확보한다.
#![allow(non_snake_case, dead_code)]
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Mutex;
use std::collections::HashSet;
use std::time::Duration;
use mod_api::*;

// ── ghidra-re 확정 seam (0.5.2) ──
const APPLY_RVA: usize = 0x1fe5cc0;
const APPLY_SIG: [u8; 12] = [0x55,0x41,0x57,0x41,0x56,0x41,0x55,0x41,0x54,0x56,0x57,0x53];
const APPLY_LEN: usize = 12;
const VT_DATACHAMP: usize = 0x3876088; // 검증용(로그에 vtable 일치 확인)
const SS_RESOLVE_OFF: usize = 0x1b8;   // sim_state+0x1b8 = caster resolve fn ptr
const D_GATE: usize = 0x48;            // descriptor+0x48 궁/액션 유무 게이트
const E_NAMEPTR: usize = 0x250;        // entity name ptr (0.5.2 검증됨, resolve 반환 entity에서 재확인)
const E_NAMELEN: usize = 0x258;

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
unsafe fn in_exe(a: u64) -> bool { let b = exe_base() as u64; a >= b && a < b + 0x4000000 }

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

// ── VEH-safe 읽기 ──
core::arch::global_asm!(
    ".globl shm_rd8", ".globl shm_rd8_f", ".globl shm_rd8_l",
    ".globl shm_rd1", ".globl shm_rd1_f", ".globl shm_rd1_l",
    "shm_rd8:", "shm_rd8_f:", "mov rax, qword ptr [rcx]", "mov qword ptr [rdx], rax", "mov eax, 1", "ret",
    "shm_rd8_l:", "xor eax, eax", "ret",
    "shm_rd1:", "shm_rd1_f:", "movzx eax, byte ptr [rcx]", "mov byte ptr [rdx], al", "mov eax, 1", "ret",
    "shm_rd1_l:", "xor eax, eax", "ret",
);
extern "C" {
    fn shm_rd8(addr: usize, out: *mut u64) -> u32;
    fn shm_rd1(addr: usize, out: *mut u8) -> u32;
    static shm_rd8_f: u8; static shm_rd8_l: u8;
    static shm_rd1_f: u8; static shm_rd1_l: u8;
}
#[inline] unsafe fn rd_u64(a: usize) -> Option<u64> { if a < 0x10000 { return None; } let mut o=0u64; if shm_rd8(a,&mut o)!=0 {Some(o)} else {None} }
#[inline] unsafe fn rd_u8(a: usize)  -> Option<u8>  { if a < 0x10000 { return None; } let mut o=0u8;  if shm_rd1(a,&mut o)!=0 {Some(o)} else {None} }

static VEH_INSTALLED: AtomicBool = AtomicBool::new(false);
extern "system" fn shm_veh(p: *mut ExceptionPointers) -> i32 {
    const CONTINUE_EXECUTION: i32 = -1; const CONTINUE_SEARCH: i32 = 0;
    unsafe {
        if p.is_null() { return CONTINUE_SEARCH; }
        let rec = (*p).rec;
        if rec.is_null() || (*rec).code != 0xC0000005 { return CONTINUE_SEARCH; }
        let ctx = (*p).ctx;
        if ctx == 0 { return CONTINUE_SEARCH; }
        let rip = *((ctx + 0xF8) as *const u64) as usize;
        let land = if rip == core::ptr::addr_of!(shm_rd8_f) as usize { core::ptr::addr_of!(shm_rd8_l) as usize }
                   else if rip == core::ptr::addr_of!(shm_rd1_f) as usize { core::ptr::addr_of!(shm_rd1_l) as usize }
                   else { 0 };
        if land != 0 { *((ctx + 0xF8) as *mut u64) = land as u64; return CONTINUE_EXECUTION; }
        CONTINUE_SEARCH
    }
}
fn veh_install() { if VEH_INSTALLED.swap(true, Ordering::Relaxed) { return; } unsafe { AddVectoredExceptionHandler(1, shm_veh); } }

unsafe fn read_name(ptr: usize, len: usize) -> Option<String> {
    if ptr < 0x10000 || len < 2 || len > 40 { return None; }
    let mut buf = Vec::with_capacity(len);
    for i in 0..len {
        let b = rd_u8(ptr + i)?;
        if !(b == b'_' || b.is_ascii_alphanumeric()) { return None; }
        buf.push(b);
    }
    String::from_utf8(buf).ok()
}
// entity에서 name 추출(+0x250 ptr / +0x258 len). resolve 반환 entity 검증용.
unsafe fn entity_name(ent: usize) -> Option<String> {
    let p = rd_u64(ent + E_NAMEPTR)? as usize;
    let l = rd_u64(ent + E_NAMELEN)? as usize;
    read_name(p, l)
}

static WRITE_LOCK: Mutex<()> = Mutex::new(());
fn mlog(s: &str) {
    use std::io::Write;
    let _g = WRITE_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true)
        .open("C:\\tfm2mods\\sylas_hijack\\probe_m2_log.txt") {
        let _ = f.write_all(s.as_bytes());
    }
}

// cfg 게이트: probe_resolve=1 이면 resolve CALL 시도(위험, 기본 OFF).
static RESOLVE_ON: AtomicBool = AtomicBool::new(false);
fn cfg_refresher() {
    std::thread::spawn(|| loop {
        let s = std::fs::read_to_string("C:\\tfm2mods\\sylas_hijack\\sylas_hijack.cfg").unwrap_or_default();
        RESOLVE_ON.store(s.contains("probe_resolve=1"), Ordering::Relaxed);
        std::thread::sleep(Duration::from_secs(2));
    });
}

// resolve fn 타입 후보: 대부분 resolve(handle)->entity (rcx=handle). this=sim_state형이면 resolve(sim_state,handle).
type Resolve1 = unsafe extern "C" fn(usize) -> usize;
type Resolve2 = unsafe extern "C" fn(usize, usize) -> usize;

static SEEN: Mutex<Option<HashSet<usize>>> = Mutex::new(None); // descriptor ptr dedup
static N: AtomicU32 = AtomicU32::new(0);

unsafe fn apply_common(saved: usize, e: usize) {
    let desc = *((saved + 0x28) as *const u64) as usize;   // rcx = descriptor(this)
    let chandle = *((saved + 0x20) as *const u64) as usize; // rdx = caster_handle
    let sim = *((saved + 0x18) as *const u64) as usize;     // r8  = sim_state
    let target = *((saved + 0x10) as *const u64) as usize;  // r9  = target_ctx
    let casting_ctx = rd_u64(e + 0x28).unwrap_or(0) as usize; // [orig_rsp+0x28] = 5th

    // descriptor 단위 dedup (재빌드마다 새 ptr = 새 로그) — 폭주 방지
    let is_new = {
        let mut g = SEEN.lock().unwrap_or_else(| x| x.into_inner());
        if g.is_none() { *g = Some(HashSet::new()); }
        let m = g.as_mut().unwrap();
        if m.len() >= 200 { return; }
        m.insert(desc)
    };
    if !is_new { return; }
    if N.fetch_add(1, Ordering::Relaxed) >= 200 { return; }

    let vt = rd_u64(desc).unwrap_or(0);                // descriptor vtable(+0)
    let gate = rd_u64(desc + D_GATE).unwrap_or(0xDEAD); // +0x48 궁게이트(-1=액션없음)
    let a180 = rd_u64(desc + 0x180).unwrap_or(0);
    let a188 = rd_u64(desc + 0x188).unwrap_or(0);
    let a198 = rd_u64(desc + 0x198).unwrap_or(0);
    let resolve_ptr = rd_u64(sim + SS_RESOLVE_OFF).unwrap_or(0); // sim_state+0x1b8
    let cc0 = rd_u64(casting_ctx).unwrap_or(0xDEAD);   // casting_ctx[+0] (init 게이트)
    let vt_match = vt == (exe_base() + VT_DATACHAMP) as u64;

    let mut s = format!(
        "\n[apply#{}] desc={:#x} vt=RVA:{:#x}{} gate={:#x} a180={:#x} a188={:#x} a198={:#x}\n\
         \x20  chandle={:#x} sim={:#x} target={:#x} cctx={:#x} cc0={:#x} resolve_fn=RVA:{:#x}\n",
        N.load(Ordering::Relaxed), desc, rva_of(vt),
        if vt_match {"(=datachamp✓)"} else {""},
        gate, a180, a188, a198,
        chandle, sim, target, casting_ctx, cc0, rva_of(resolve_ptr));

    // ④ (게이트) resolve CALL → entity → name. 위험 → catch_unwind + in_exe 확인.
    if RESOLVE_ON.load(Ordering::Relaxed) && in_exe(resolve_ptr) && chandle >= 0x1000 {
        let base = exe_base();
        // 후보1: resolve(handle) [rcx=handle]
        let r1 = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let f: Resolve1 = core::mem::transmute(resolve_ptr as usize);
            f(chandle)
        })).ok();
        // 후보2: resolve(sim, handle) [rcx=sim, rdx=handle]
        let r2 = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let f: Resolve2 = core::mem::transmute(resolve_ptr as usize);
            f(sim, chandle)
        })).ok();
        let n1 = r1.filter(|&x| x >= 0x10000).and_then(|x| entity_name(x));
        let n2 = r2.filter(|&x| x >= 0x10000).and_then(|x| entity_name(x));
        s.push_str(&format!("\x20  resolve1(handle)={:#x} name={:?} | resolve2(sim,handle)={:#x} name={:?}\n",
            r1.unwrap_or(0), n1, r2.unwrap_or(0), n2));
        let _ = base;
    }
    mlog(&s);
}

unsafe fn install_detour(rva: usize, hook_len: usize, expect: &[u8], cap_fn: usize) -> Result<usize, String> {
    let mbase = exe_base();
    if mbase == 0 { return Err("module 0".into()); }
    let fn_addr = mbase + rva;
    if !readable(fn_addr, hook_len + 8) { return Err(format!("fn {:#x} unreadable", fn_addr)); }
    let mut cur = [0u8; 20];
    core::ptr::copy_nonoverlapping(fn_addr as *const u8, cur.as_mut_ptr(), 20);
    if &cur[..expect.len()] != expect {
        return Err(format!("프롤로그 불일치 rva={:#x} 실제={:02x?}", rva, &cur[..expect.len().min(20)]));
    }
    const MEM_CR: u32 = 0x1000 | 0x2000; const RWX: u32 = 0x40;
    let stub = VirtualAlloc(0, 256, MEM_CR, RWX);
    if stub == 0 { return Err("VirtualAlloc".into()); }
    let ret_addr = fn_addr + hook_len;
    let mut s: Vec<u8> = Vec::new();
    s.extend_from_slice(&[0x49,0x89,0xe2]);                                    // mov r10, rsp (orig_rsp)
    s.extend_from_slice(&[0x51,0x52,0x41,0x50,0x41,0x51,0x41,0x52,0x41,0x53]); // push rcx,rdx,r8,r9,r10,r11
    s.extend_from_slice(&[0x48,0x89,0xe1]);                                    // mov rcx, rsp (saved)
    s.extend_from_slice(&[0x4c,0x89,0xd2]);                                    // mov rdx, r10 (e=orig_rsp)
    s.extend_from_slice(&[0x48,0x83,0xec,0x28]);                              // sub rsp, 0x28
    s.extend_from_slice(&[0x48,0xb8]); s.extend_from_slice(&cap_fn.to_le_bytes()); // mov rax, cap_fn
    s.extend_from_slice(&[0xff,0xd0]);                                        // call rax
    s.extend_from_slice(&[0x48,0x83,0xc4,0x28]);                              // add rsp, 0x28
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

unsafe extern "C" fn cap_apply(saved: usize, e: usize) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| apply_common(saved, e)));
}

static HOOK_INSTALLED: AtomicBool = AtomicBool::new(false);
fn init(_ctx: &GameCtx) -> ModRegistration {
    std::thread::spawn(|| {
        std::thread::sleep(Duration::from_secs(6));
        if HOOK_INSTALLED.swap(true, Ordering::Relaxed) { return; }
        veh_install();
        cfg_refresher();
        let base = exe_base();
        mlog(&format!("\n===== sylas_hijack PROBE v2 (방법2 apply계측) 시작 base={:#x} vt_datachamp={:#x} =====\n",
            base, base + VT_DATACHAMP));
        match unsafe { install_detour(APPLY_RVA, APPLY_LEN, &APPLY_SIG, cap_apply as *const () as usize) } {
            Ok(stub) => mlog(&format!("[install] datachamp_apply @{:#x} OK stub={:#x}\n", base + APPLY_RVA, stub)),
            Err(e)   => mlog(&format!("[install] datachamp_apply 실패: {}\n", e)),
        }
    });
    ModRegistration::new("sylas_hijack")
}

declare_mod!(init);
