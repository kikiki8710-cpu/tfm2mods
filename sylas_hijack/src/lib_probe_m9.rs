// sylas_hijack PROBE v3 (방법2, 0.5.2) — resolve 재구현 + caster entity 덤프로 champion id 위치 실측.
// ★v2 정정(ghidra-re 07-26): apply 0x1fe5cc0의 rdx = caster_handle 아님 = **world(엔티티 스토리지)**.
//   진짜 caster = resolve(world=rdx, key=*(u64*)(casting_ctx+8)). resolve = 2단 SlotMap(재구현, CALL 불필요).
//   resolve 반환 entity(stride 0x6a8)에서 name@+0x250 무효 → champion id 위치 = 이 프로브로 실측.
// 목적(쓰기 없음=안전, 전부 VEH-safe read):
//   ① casting_ctx의 어느 키가 caster/target인지 (여러 key 오프셋 resolve 시도)
//   ② resolve 반환 entity를 덤프해 champion id/name(=사일러스 식별) 위치 찾기
//      — entity 워드 중 heap ptr이 master(0x748, +8=id char*/+0x10=len) 참조인지 추적
//   ③ 사일러스 entity 확인 시 action POD(apply rcx) 시그니처 = 궁 판별자 후보 수집
#![allow(non_snake_case, dead_code)]
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Mutex;
use std::collections::HashSet;
use std::time::Duration;
use mod_api::*;

// ★v6: 발화 dispatch FUN_1423a76e0 훅 (effect active 순회 발화점, base/data 균일)
//   param: rcx=p1 / rdx=p2(sim) / r8=p3(effect fatptr {data@0,vtable@8}) / r9=p4(caster entity)
//   발화 = call [ [p3+8] + 0xd0 ]. caster=sylas일 때 발화 apply(대상 궁) 로깅 = 방법2 발화 확인.
const APPLY_RVA: usize = 0x1b785d0;
const APPLY_SIG: [u8; 12] = [0x55,0x41,0x57,0x41,0x56,0x41,0x55,0x41,0x54,0x56,0x57,0x53];
const APPLY_LEN: usize = 12;
// world(엔티티 스토리지) SlotMap 오프셋 (ghidra-re 재구현)
const W_DENSE_BASE: usize = 0x720;
const W_DENSE_LEN:  usize = 0x728;
const W_SLOTS:      usize = 0x738;
const W_SLOT_LEN:   usize = 0x740;
const W_FB_KEY:     usize = 0x618; // 폴백 단일엔티티 key
const W_FB_ENT:     usize = 0x70;
const ENT_STRIDE:   usize = 0x6a8;

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
#[inline] fn is_heap(x: u64) -> bool { x >= 0x10000000000 && x < 0x800000000000 }

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

core::arch::global_asm!(
    ".globl sh3_rd8", ".globl sh3_rd8_f", ".globl sh3_rd8_l",
    ".globl sh3_rd1", ".globl sh3_rd1_f", ".globl sh3_rd1_l",
    "sh3_rd8:", "sh3_rd8_f:", "mov rax, qword ptr [rcx]", "mov qword ptr [rdx], rax", "mov eax, 1", "ret",
    "sh3_rd8_l:", "xor eax, eax", "ret",
    "sh3_rd1:", "sh3_rd1_f:", "movzx eax, byte ptr [rcx]", "mov byte ptr [rdx], al", "mov eax, 1", "ret",
    "sh3_rd1_l:", "xor eax, eax", "ret",
);
extern "C" {
    fn sh3_rd8(addr: usize, out: *mut u64) -> u32;
    fn sh3_rd1(addr: usize, out: *mut u8) -> u32;
    static sh3_rd8_f: u8; static sh3_rd8_l: u8;
    static sh3_rd1_f: u8; static sh3_rd1_l: u8;
}
#[inline] unsafe fn rd_u64(a: usize) -> Option<u64> { if a < 0x10000 { return None; } let mut o=0u64; if sh3_rd8(a,&mut o)!=0 {Some(o)} else {None} }
#[inline] unsafe fn rd_u8(a: usize)  -> Option<u8>  { if a < 0x10000 { return None; } let mut o=0u8;  if sh3_rd1(a,&mut o)!=0 {Some(o)} else {None} }

static VEH_INSTALLED: AtomicBool = AtomicBool::new(false);
extern "system" fn sh3_veh(p: *mut ExceptionPointers) -> i32 {
    const CONTINUE_EXECUTION: i32 = -1; const CONTINUE_SEARCH: i32 = 0;
    unsafe {
        if p.is_null() { return CONTINUE_SEARCH; }
        let rec = (*p).rec;
        if rec.is_null() || (*rec).code != 0xC0000005 { return CONTINUE_SEARCH; }
        let ctx = (*p).ctx;
        if ctx == 0 { return CONTINUE_SEARCH; }
        let rip = *((ctx + 0xF8) as *const u64) as usize;
        let land = if rip == core::ptr::addr_of!(sh3_rd8_f) as usize { core::ptr::addr_of!(sh3_rd8_l) as usize }
                   else if rip == core::ptr::addr_of!(sh3_rd1_f) as usize { core::ptr::addr_of!(sh3_rd1_l) as usize }
                   else { 0 };
        if land != 0 { *((ctx + 0xF8) as *mut u64) = land as u64; return CONTINUE_EXECUTION; }
        CONTINUE_SEARCH
    }
}
fn veh_install() { if VEH_INSTALLED.swap(true, Ordering::Relaxed) { return; } unsafe { AddVectoredExceptionHandler(1, sh3_veh); } }

// id/name 문자열 판독 (alnum/_ 만, 2~24자)
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
// heap ptr에서 여러 struct 형태로 id 문자열 추출 시도(champion 이름 확실히 뽑기).
unsafe fn try_id_at(p: usize) -> Option<String> {
    // (a) record{+8=char*, +0x10=len}  (바닐라 "priest_attack" 형태)
    if let (Some(cp), Some(cl)) = (rd_u64(p + 8), rd_u64(p + 0x10)) {
        if let Some(s) = read_id(cp as usize, cl as usize) { return Some(s); }
    }
    // (b) String{+0=char*, +8=len}
    if let (Some(cp), Some(cl)) = (rd_u64(p), rd_u64(p + 8)) {
        if let Some(s) = read_id(cp as usize, cl as usize) { return Some(s); }
    }
    // (c) String{+0=char*, +8=cap, +0x10=len}
    if let (Some(cp), Some(cl)) = (rd_u64(p), rd_u64(p + 0x10)) {
        if let Some(s) = read_id(cp as usize, cl as usize) { return Some(s); }
    }
    None
}

// ★resolve 재구현 (ghidra-re 0x2305520): world+key → entity(stride 0x6a8). CALL 없음.
unsafe fn resolve(world: usize, key: u64) -> usize {
    if world < 0x10000 { return 0; }
    let slot_len = rd_u64(world + W_SLOT_LEN).unwrap_or(0);
    if key < slot_len {
        if let Some(slots) = rd_u64(world + W_SLOTS) {
            let slots = slots as usize;
            let entry = slots.wrapping_add(key as usize * 0x10);
            let tag = rd_u64(entry).unwrap_or(0) as u32; // 하위32 = tag
            if tag == 1 {
                let dense_idx = rd_u64(entry + 8).unwrap_or(u64::MAX);
                let dense_len = rd_u64(world + W_DENSE_LEN).unwrap_or(0);
                if dense_idx < dense_len {
                    if let Some(db) = rd_u64(world + W_DENSE_BASE) {
                        let ent = (db as usize).wrapping_add(dense_idx as usize * ENT_STRIDE);
                        if readable(ent, 0x6a8) { return ent; }
                    }
                }
            }
        }
    }
    // 폴백: 단일 인라인 엔티티
    if rd_u64(world + W_FB_KEY).unwrap_or(u64::MAX ^ 1) == key
        && (rd_u64(world + W_FB_ENT).unwrap_or(0) as i32) != -1 {
        return world + W_FB_ENT;
    }
    0
}

// entity 광범위 스캔 → 모든 문자열(id/champion명) 오프셋별 추출. entity 자체 inline String도 시도.
unsafe fn entity_find_id(ent: usize, out: &mut String) -> Option<String> {
    let mut found: Option<String> = None;
    for i in 0..0x80usize { // 0..0x400, 8B 간격
        let off = i * 8;
        let w = match rd_u64(ent + off) { Some(v) => v, None => continue };
        if !is_heap(w) { continue; }
        // (1) w = ptr → try_id_at (master/String 여러 형태)
        if let Some(id) = try_id_at(w as usize) {
            out.push_str(&format!("    +{:#x}→{:#x} id=\"{}\"\n", off, w, id));
            if found.is_none() { found = Some(id); }
            continue;
        }
        // (2) entity inline String: entity+off=char*(=w), entity+off+8=len
        if let Some(l) = rd_u64(ent + off + 8) {
            if let Some(s) = read_id(w as usize, l as usize) {
                out.push_str(&format!("    +{:#x} inlineStr=\"{}\"\n", off, s));
                if found.is_none() { found = Some(s); }
            }
        }
    }
    found
}

static WRITE_LOCK: Mutex<()> = Mutex::new(());
fn m3log(s: &str) {
    use std::io::Write;
    let _g = WRITE_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true)
        .open("C:\\tfm2mods\\sylas_hijack\\probe_m9_log.txt") {
        let _ = f.write_all(s.as_bytes());
    }
}

// (world,key) dedup, entity dedup
static SEEN: Mutex<Option<HashSet<usize>>> = Mutex::new(None);
static N: AtomicU32 = AtomicU32::new(0);

// entity name@+0x250 읽기
unsafe fn ent_name(ent: usize) -> Option<String> {
    if ent < 0x10000 { return None; }
    let np = rd_u64(ent + 0x250)? as usize;
    let nl = rd_u64(ent + 0x258)? as usize;
    read_id(np, nl)
}
// ★v9: native effect apply(0x1b785d0) 훅. 계약(base apply류) = rcx=action/rdx=world/r8=sim/r9=target/[rsp+0x28]=casting_ctx.
//   caster=resolve(world, casting_ctx[8]) → sylas면 action-kind(+0x30)·id·5인자 로깅 = 사일러스 effect apply 통과 확인 + ult 판별 시그.
unsafe fn apply_common(saved: usize, e: usize) {
    let action = *((saved + 0x28) as *const u64) as usize; // rcx = action data
    let world  = *((saved + 0x20) as *const u64) as usize; // rdx = world
    let sim    = *((saved + 0x18) as *const u64) as usize; // r8  = sim_state(=base+0x38c5d78 상수?)
    let target = *((saved + 0x10) as *const u64) as usize; // r9  = target
    let cctx   = rd_u64(e + 0x28).unwrap_or(0) as usize;   // [orig_rsp+0x28] = casting_ctx
    if cctx < 0x10000 || world < 0x10000 { return; }
    // caster resolve: resolve(world, casting_ctx[8])
    let key = rd_u64(cctx + 8).unwrap_or(u64::MAX);
    if key >= 0x100000 { return; }
    let caster = resolve(world, key);
    if caster == 0 { return; }
    let cname = match ent_name(caster) { Some(n) => n, None => return };
    if cname != "sylas" { return; }

    // action 시그니처: +0x30=action-kind(executor jumptable), +0x8/+0x38 후보 id/kind
    let kind30 = rd_u64(action + 0x30).unwrap_or(0);
    let a08 = rd_u64(action + 0x8).unwrap_or(0);
    let a38 = rd_u64(action + 0x38).unwrap_or(0);
    let id08 = try_id_at(action);                         // action이 master류면 id
    let id_at8 = { let p = rd_u64(action+8).unwrap_or(0) as usize; let l = rd_u64(action+0x10).unwrap_or(0) as usize; read_id(p,l) };
    let sim_is_const = sim == exe_base() + 0x38c5d78;

    // dedup: (action-kind + id) 별 — effect 종류별 1회
    let dkey = kind30 as usize ^ (a38 as usize).rotate_left(13) ^ id_at8.as_ref().map(|s| s.bytes().map(|b| b as usize).sum()).unwrap_or(0);
    let is_new = {
        let mut g = SEEN.lock().unwrap_or_else(|x| x.into_inner());
        if g.is_none() { *g = Some(HashSet::new()); }
        let m = g.as_mut().unwrap();
        if m.len() >= 120 { return; }
        m.insert(dkey)
    };
    if !is_new { return; }
    let n = N.fetch_add(1, Ordering::Relaxed);

    // 강탈 대상(sylas+0x308 handle)
    let tgt_key = rd_u64(caster + 0x308).unwrap_or(0);
    let tgt_ent = if tgt_key < 0x100000 { resolve(world, tgt_key) } else { 0 };
    let tgt_name = if tgt_ent != 0 { ent_name(tgt_ent).unwrap_or_default() } else { String::new() };

    let s = format!(
        "[syl#{}] action={:#x} kind30={:#x} a08={:#x} a38={:#x} id0={:?} id8={:?}\n\
         \x20  world={:#x} sim_const={} target={:#x} cctx={:#x} caster_key={} 대상={:?}({:#x})\n",
        n, action, kind30, a08, a38, id08, id_at8,
        world, sim_is_const, target, cctx, key, tgt_name, tgt_ent);
    m3log(&s);
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

unsafe extern "C" fn cap_apply(saved: usize, e: usize) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| apply_common(saved, e)));
}

static HOOK_INSTALLED: AtomicBool = AtomicBool::new(false);
fn init(_ctx: &GameCtx) -> ModRegistration {
    std::thread::spawn(|| {
        std::thread::sleep(Duration::from_secs(6));
        if HOOK_INSTALLED.swap(true, Ordering::Relaxed) { return; }
        veh_install();
        let base = exe_base();
        m3log(&format!("\n===== sylas_hijack PROBE v9 (native effect apply 훅 sylas ult시그) 시작 base={:#x} =====\n", base));
        match unsafe { install_detour(APPLY_RVA, APPLY_LEN, &APPLY_SIG, cap_apply as *const () as usize) } {
            Ok(stub) => m3log(&format!("[install] datachamp_apply @{:#x} OK stub={:#x}\n", base + APPLY_RVA, stub)),
            Err(e)   => m3log(&format!("[install] datachamp_apply 실패: {}\n", e)),
        }
    });
    ModRegistration::new("sylas_hijack")
}

declare_mod!(init);
