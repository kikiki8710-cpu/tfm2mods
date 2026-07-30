// sylas_hijack — 실제 강탈 (0.5.2, W31 재배선). 프로브 아님 = 슬롯 write + clone_box shadow-call.
// ★ghidra-re 확정 배선(엔티티 슬롯 복사):
//   sylas 궁슬롯(+0x148 action data / +0x150 vtable)을 대상 챔프 궁 action으로 clone_box 복사.
//   대상 = sylas+0x308(강탈 대상 이름)의 라이브 엔티티. 궁 슬롯만. 발화는 자동(dispatch 균일).
// ★안전: shallow copy 금지(dangling) → clone_box([vtbl+0x48], rcx=data)로 fresh 복사 필수.
//   cfg arm=1 게이트(기본 OFF). 두꺼운 타입/ptr 게이트. VEH-safe read/write. clone_box는 catch_unwind.
//   old action은 leak(초기 안전 우선 — drop_in_place 오호출 크래시 회피).
// 훅 = 0x1fe5cc0(base apply, world 접근점). base 챔프가 궁 쓸 때마다 world 순회 → sylas 슬롯 강탈.
#![allow(non_snake_case, dead_code)]
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Mutex;
use std::collections::HashSet;
use std::time::Duration;
use mod_api::*;

const APPLY_RVA: usize = 0x1fe5cc0; // base apply (world=rdx 접근점)
const APPLY_SIG: [u8; 12] = [0x55,0x41,0x57,0x41,0x56,0x41,0x55,0x41,0x54,0x56,0x57,0x53];
const APPLY_LEN: usize = 12;
const W_DENSE_BASE: usize = 0x720;
const W_DENSE_LEN:  usize = 0x728;
const ENT_STRIDE:   usize = 0x6a8;
const E_NAME_PTR: usize = 0x250;
const E_NAME_LEN: usize = 0x258;
const E_TGT_PTR:  usize = 0x308; // 강탈 대상 이름 String ptr
const E_TGT_LEN:  usize = 0x310;
const S_ULT_DATA: usize = 0x148; // 궁슬롯 action data
const S_ULT_VTBL: usize = 0x150; // 궁슬롯 vtable
const V_CLONE_BOX: usize = 0x48; // vtable slot9 = clone_box(rcx=self)->rax=new
const V_APPLY: usize = 0xd0;     // vtable slot26 = apply (effect-action 검증용)

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
unsafe fn writable(a: usize, len: usize) -> bool {
    if a < 0x10000 { return false; }
    let mut mbi: MemBasicInfo = core::mem::zeroed();
    if VirtualQuery(a, &mut mbi, core::mem::size_of::<MemBasicInfo>()) == 0 { return false; }
    if mbi.state != 0x1000 { return false; }
    const WPROT: u32 = 0x04|0x08|0x40|0x80; // RW/WC/RWX/WCX
    if mbi.protect & WPROT == 0 { return false; }
    let end = mbi.base + mbi.region;
    a.wrapping_add(len) <= end
}

// ── VEH-safe read/write ──
core::arch::global_asm!(
    ".globl shj_rd8", ".globl shj_rd8_f", ".globl shj_rd8_l",
    ".globl shj_wr8", ".globl shj_wr8_f", ".globl shj_wr8_l",
    "shj_rd8:", "shj_rd8_f:", "mov rax, qword ptr [rcx]", "mov qword ptr [rdx], rax", "mov eax, 1", "ret",
    "shj_rd8_l:", "xor eax, eax", "ret",
    "shj_wr8:", "shj_wr8_f:", "mov qword ptr [rcx], rdx", "mov eax, 1", "ret",
    "shj_wr8_l:", "xor eax, eax", "ret",
);
extern "C" {
    fn shj_rd8(addr: usize, out: *mut u64) -> u32;
    fn shj_wr8(addr: usize, val: u64) -> u32;
    static shj_rd8_f: u8; static shj_rd8_l: u8;
    static shj_wr8_f: u8; static shj_wr8_l: u8;
}
#[inline] unsafe fn rd_u64(a: usize) -> Option<u64> { if a < 0x10000 { return None; } let mut o=0u64; if shj_rd8(a,&mut o)!=0 {Some(o)} else {None} }
#[inline] unsafe fn wr_u64(a: usize, v: u64) -> bool { if a < 0x10000 { return false; } shj_wr8(a, v) != 0 }
#[inline] unsafe fn rd_u8(a: usize) -> Option<u8> { let w = rd_u64(a & !7)?; Some((w >> ((a & 7)*8)) as u8) }

static VEH_INSTALLED: AtomicBool = AtomicBool::new(false);
extern "system" fn shj_veh(p: *mut ExceptionPointers) -> i32 {
    const CE: i32 = -1; const CS: i32 = 0;
    unsafe {
        if p.is_null() { return CS; }
        let rec = (*p).rec;
        if rec.is_null() || (*rec).code != 0xC0000005 { return CS; }
        let ctx = (*p).ctx; if ctx == 0 { return CS; }
        let rip = *((ctx + 0xF8) as *const u64) as usize;
        let land = if rip == core::ptr::addr_of!(shj_rd8_f) as usize { core::ptr::addr_of!(shj_rd8_l) as usize }
                   else if rip == core::ptr::addr_of!(shj_wr8_f) as usize { core::ptr::addr_of!(shj_wr8_l) as usize }
                   else { 0 };
        if land != 0 { *((ctx + 0xF8) as *mut u64) = land as u64; return CE; }
        CS
    }
}
fn veh_install() { if VEH_INSTALLED.swap(true, Ordering::Relaxed) { return; } unsafe { AddVectoredExceptionHandler(1, shj_veh); } }

unsafe fn read_id(ptr: usize, len: usize) -> Option<String> {
    if ptr < 0x10000 || len < 2 || len > 24 { return None; }
    let mut buf = Vec::with_capacity(len);
    for i in 0..len {
        let b = rd_u8(ptr + i)?;
        if !(b == b'_' || b.is_ascii_alphanumeric()) { return None; }
        buf.push(b);
    }
    String::from_utf8(buf).ok()
}
unsafe fn ent_name(ent: usize) -> Option<String> {
    let p = rd_u64(ent + E_NAME_PTR)? as usize;
    let l = rd_u64(ent + E_NAME_LEN)? as usize;
    read_id(p, l)
}

static WRITE_LOCK: Mutex<()> = Mutex::new(());
fn hlog(s: &str) {
    use std::io::Write;
    let _g = WRITE_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true)
        .open("C:\\tfm2mods\\sylas_hijack\\hijack052_log.txt") {
        let _ = f.write_all(s.as_bytes());
    }
}

// ── cfg arm 게이트 ──
static ARMED: AtomicBool = AtomicBool::new(false);
fn cfg_refresher() {
    std::thread::spawn(|| loop {
        let s = std::fs::read_to_string("C:\\tfm2mods\\sylas_hijack\\sylas_hijack.cfg").unwrap_or_default();
        ARMED.store(s.contains("arm=1"), Ordering::Relaxed);
        std::thread::sleep(Duration::from_secs(2));
    });
}

// clone_box shadow-call: clone_box(rcx=self_action)->rax=new_action. AV 위험 → catch_unwind + 게이트.
type CloneBoxFn = unsafe extern "C" fn(usize) -> usize;
unsafe fn call_clone_box(clone_fn: usize, self_action: usize) -> Option<usize> {
    let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let f: CloneBoxFn = core::mem::transmute(clone_fn);
        f(self_action)
    }));
    match r { Ok(v) if v >= 0x10000 => Some(v), _ => None }
}

static HJ_LOG: AtomicU32 = AtomicU32::new(0);
// (sylas ent, tgt vtbl) 강탈 완료 기록 — 같은 상태 재강탈 방지(슬롯 재빌드 시 vtbl 바뀌면 재강탈).
static DONE: Mutex<Option<HashSet<(usize, u64)>>> = Mutex::new(None);

unsafe fn hijack_world(world: usize) {
    if !ARMED.load(Ordering::Relaxed) { return; }
    if world < 0x10000 { return; }
    let dense_base = match rd_u64(world + W_DENSE_BASE) { Some(v) => v as usize, None => return };
    let dense_len  = rd_u64(world + W_DENSE_LEN).unwrap_or(0);
    if dense_base < 0x10000 || dense_len == 0 || dense_len > 512 { return; }

    // 1) 전 엔티티 (name → ent) 수집
    let mut ents: Vec<(String, usize)> = Vec::new();
    let mut sylas_list: Vec<usize> = Vec::new();
    for i in 0..dense_len as usize {
        let ent = dense_base.wrapping_add(i * ENT_STRIDE);
        if !readable(ent, ENT_STRIDE) { continue; }
        if let Some(n) = ent_name(ent) {
            if n == "sylas" { sylas_list.push(ent); }
            ents.push((n, ent));
        }
    }
    if sylas_list.is_empty() { return; }

    for sylas in sylas_list {
        // 2) 강탈 대상 이름 (sylas+0x308)
        let tp = match rd_u64(sylas + E_TGT_PTR) { Some(v) => v as usize, None => continue };
        let tl = rd_u64(sylas + E_TGT_LEN).unwrap_or(0) as usize;
        let tgt_name = match read_id(tp, tl) { Some(v) => v, None => continue };
        if tgt_name == "sylas" { continue; }
        // 3) 대상 엔티티
        let tgt = match ents.iter().find(|(n, _)| *n == tgt_name) { Some((_, e)) => *e, None => continue };
        // 4) 대상 궁슬롯
        let tdata = match rd_u64(tgt + S_ULT_DATA) { Some(v) => v as usize, None => continue };
        let tvtbl = match rd_u64(tgt + S_ULT_VTBL) { Some(v) => v, None => continue };
        if tdata < 0x10000 || !in_exe(tvtbl) { continue; }
        // effect-action vtable 검증: [vtbl+0xd0]=apply, [vtbl+0x48]=clone_box 둘 다 exe 코드
        let apply = rd_u64(tvtbl as usize + V_APPLY).unwrap_or(0);
        let clone_fn = rd_u64(tvtbl as usize + V_CLONE_BOX).unwrap_or(0);
        if !in_exe(apply) || !in_exe(clone_fn) { continue; }

        // 5) 이미 이 대상으로 강탈됨? (sylas 슬롯 vtbl == tvtbl) → skip
        let svtbl = rd_u64(sylas + S_ULT_VTBL).unwrap_or(0);
        if svtbl == tvtbl { continue; }
        // 중복 강탈 방지 기록
        let fresh = {
            let mut g = DONE.lock().unwrap_or_else(|e| e.into_inner());
            if g.is_none() { *g = Some(HashSet::new()); }
            g.as_mut().unwrap().insert((sylas, tvtbl))
        };
        if !fresh { continue; }

        // 6) clone_box로 fresh action 복사 (shallow copy 금지)
        let new = match call_clone_box(clone_fn as usize, tdata) { Some(v) => v, None => {
            if HJ_LOG.fetch_add(1, Ordering::Relaxed) < 20 {
                hlog(&format!("[강탈실패] clone_box tgt={} data={:#x} vtbl=RVA:{:#x}\n", tgt_name, tdata, rva_of(tvtbl)));
            }
            continue;
        } };
        // 7) sylas 궁슬롯 write (old leak). +0x148=new data, +0x150=tvtbl
        if !writable(sylas + S_ULT_DATA, 16) { continue; }
        let ok1 = wr_u64(sylas + S_ULT_DATA, new as u64);
        let ok2 = wr_u64(sylas + S_ULT_VTBL, tvtbl);
        if ok1 && ok2 {
            let n = HJ_LOG.fetch_add(1, Ordering::Relaxed);
            if n < 40 {
                hlog(&format!("[강탈#{}] sylas {:#x} ← {} 궁(new={:#x} vtbl=RVA:{:#x} apply=RVA:{:#x})\n",
                    n, sylas, tgt_name, new, rva_of(tvtbl), rva_of(apply)));
            }
        }
    }
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
    s.extend_from_slice(&[0x49,0x89,0xe2]);
    s.extend_from_slice(&[0x51,0x52,0x41,0x50,0x41,0x51,0x41,0x52,0x41,0x53]);
    s.extend_from_slice(&[0x48,0x89,0xe1]);
    s.extend_from_slice(&[0x4c,0x89,0xd2]);
    s.extend_from_slice(&[0x48,0x83,0xec,0x28]);
    s.extend_from_slice(&[0x48,0xb8]); s.extend_from_slice(&cap_fn.to_le_bytes());
    s.extend_from_slice(&[0xff,0xd0]);
    s.extend_from_slice(&[0x48,0x83,0xc4,0x28]);
    s.extend_from_slice(&[0x41,0x5b,0x41,0x5a,0x41,0x59,0x41,0x58,0x5a,0x59]);
    let mut orig = vec![0u8; hook_len];
    core::ptr::copy_nonoverlapping(fn_addr as *const u8, orig.as_mut_ptr(), hook_len);
    s.extend_from_slice(&orig);
    s.extend_from_slice(&[0x48,0xb8]); s.extend_from_slice(&ret_addr.to_le_bytes());
    s.extend_from_slice(&[0xff,0xe0]);
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

unsafe extern "C" fn cap_hijack(saved: usize, _e: usize) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let world = *((saved + 0x20) as *const u64) as usize; // rdx = world
        hijack_world(world);
    }));
}

static HOOK_INSTALLED: AtomicBool = AtomicBool::new(false);
fn init(_ctx: &GameCtx) -> ModRegistration {
    std::thread::spawn(|| {
        std::thread::sleep(Duration::from_secs(6));
        if HOOK_INSTALLED.swap(true, Ordering::Relaxed) { return; }
        veh_install();
        cfg_refresher();
        let base = exe_base();
        hlog(&format!("\n===== sylas_hijack 실제강탈(엔티티 슬롯복사) 시작 base={:#x} arm=cfg =====\n", base));
        match unsafe { install_detour(APPLY_RVA, APPLY_LEN, &APPLY_SIG, cap_hijack as *const () as usize) } {
            Ok(stub) => hlog(&format!("[install] base_apply(world순회) @{:#x} OK stub={:#x}\n", base + APPLY_RVA, stub)),
            Err(e)   => hlog(&format!("[install] 실패: {}\n", e)),
        }
    });
    ModRegistration::new("sylas_hijack")
}

declare_mod!(init);
