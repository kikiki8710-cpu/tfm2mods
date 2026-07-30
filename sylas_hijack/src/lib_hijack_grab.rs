// sylas_hijack — 실제 강탈 v2 (0.5.2, W36식 base apply 직접 CALL). Grab apply 훅.
// ★배선 (4개 병렬 RE + Grab apply 정밀 디컴, 2026-07-29 확정):
//   Grab effect apply(0x1e267b0) = 사일러스 궁 전용 = 궁 시전 트리거. ABI:
//     rcx=effect_def / rdx=미사용 / r8=world / r9=WorldOps(=base+0x38c5d78 상수)
//     [rsp+0x30]=target_key(붙잡은 적) / [rsp+0x38]=casting_ctx{tag@0=0, key@8=sylas_key}
//   강탈: X = resolve(world, target_key) = 붙잡은 적. X의 궁 apply를 sylas caster로 CALL.
//     X_action = [X+0x148], X_vtbl = [X+0x150], X_apply = [X_vtbl+0xd0] (base 궁 apply, 0x20b0460류).
//     base apply 계약: rcx=action_data, rdx=world, r8=WorldOps(0x1438c5d78 상수), r9=target, [rsp+0x28]=casting_ctx.
//     casting_ctx = Grab의 a7 그대로 재사용({tag=0, sylas_key}) → sylas가 caster로 귀속.
//   Grab 원본은 트램폴린이 계속 실행(사일러스 대시) + X궁 강탈 추가 발화.
// ★안전: cfg arm=1 게이트(기본 OFF). shadow-call은 catch_unwind. X_action=X live원본(가짜 금지). VEH-safe read.
#![allow(non_snake_case, dead_code)]
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};
use std::sync::Mutex;
use std::time::Duration;
use mod_api::*;

const GRAB_RVA: usize = 0x1e267b0;
const GRAB_SIG: [u8; 12] = [0x41,0x57,0x41,0x56,0x41,0x54,0x56,0x57,0x53,0x48,0x83,0xec];
const GRAB_LEN: usize = 13; // push6(9)+sub rsp,0x38(4)
const WORLDOPS_RVA: usize = 0x38c5d78; // r9 상수 WorldOps 테이블
// world SlotMap 오프셋 (resolve 재구현)
const W_DENSE_BASE: usize = 0x720;
const W_DENSE_LEN:  usize = 0x728;
const W_SLOTS:      usize = 0x738;
const W_SLOT_LEN:   usize = 0x740;
const W_FB_KEY:     usize = 0x618;
const W_FB_ENT:     usize = 0x70;
const ENT_STRIDE:   usize = 0x6a8;
const E_NAME_PTR: usize = 0x250;
const E_NAME_LEN: usize = 0x258;
const E_ALIVE:    usize = 0x450; // ==0 유효(Grab 실측)
const S_ULT_ACTION: usize = 0x148;
const S_ULT_VTBL:   usize = 0x150;
const V_APPLY_BASE: usize = 0xd0; // base effect-action vtable +0xd0 = apply

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
    fn QueryPerformanceCounter(c: *mut i64) -> BOOL;
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

core::arch::global_asm!(
    ".globl sg_rd8", ".globl sg_rd8_f", ".globl sg_rd8_l",
    ".globl sg_rd1", ".globl sg_rd1_f", ".globl sg_rd1_l",
    "sg_rd8:", "sg_rd8_f:", "mov rax, qword ptr [rcx]", "mov qword ptr [rdx], rax", "mov eax, 1", "ret",
    "sg_rd8_l:", "xor eax, eax", "ret",
    "sg_rd1:", "sg_rd1_f:", "movzx eax, byte ptr [rcx]", "mov byte ptr [rdx], al", "mov eax, 1", "ret",
    "sg_rd1_l:", "xor eax, eax", "ret",
);
extern "C" {
    fn sg_rd8(addr: usize, out: *mut u64) -> u32;
    fn sg_rd1(addr: usize, out: *mut u8) -> u32;
    static sg_rd8_f: u8; static sg_rd8_l: u8;
    static sg_rd1_f: u8; static sg_rd1_l: u8;
}
#[inline] unsafe fn rd_u64(a: usize) -> Option<u64> { if a < 0x10000 { return None; } let mut o=0u64; if sg_rd8(a,&mut o)!=0 {Some(o)} else {None} }
#[inline] unsafe fn rd_u8(a: usize)  -> Option<u8>  { if a < 0x10000 { return None; } let mut o=0u8;  if sg_rd1(a,&mut o)!=0 {Some(o)} else {None} }

static VEH_INSTALLED: AtomicBool = AtomicBool::new(false);
extern "system" fn sg_veh(p: *mut ExceptionPointers) -> i32 {
    const CE: i32 = -1; const CS: i32 = 0;
    unsafe {
        if p.is_null() { return CS; }
        let rec = (*p).rec; if rec.is_null() || (*rec).code != 0xC0000005 { return CS; }
        let ctx = (*p).ctx; if ctx == 0 { return CS; }
        let rip = *((ctx + 0xF8) as *const u64) as usize;
        let land = if rip == core::ptr::addr_of!(sg_rd8_f) as usize { core::ptr::addr_of!(sg_rd8_l) as usize }
                   else if rip == core::ptr::addr_of!(sg_rd1_f) as usize { core::ptr::addr_of!(sg_rd1_l) as usize }
                   else { 0 };
        if land != 0 { *((ctx + 0xF8) as *mut u64) = land as u64; return CE; }
        CS
    }
}
fn veh_install() { if VEH_INSTALLED.swap(true, Ordering::Relaxed) { return; } unsafe { AddVectoredExceptionHandler(1, sg_veh); } }

unsafe fn read_name(ptr: usize, len: usize) -> Option<String> {
    if ptr < 0x10000 || len < 2 || len > 24 { return None; }
    let mut buf = Vec::with_capacity(len);
    for i in 0..len { let b = rd_u8(ptr + i)?; if !(b == b'_' || b.is_ascii_alphanumeric()) { return None; } buf.push(b); }
    String::from_utf8(buf).ok()
}
unsafe fn ent_name(ent: usize) -> Option<String> {
    let p = rd_u64(ent + E_NAME_PTR)? as usize; let l = rd_u64(ent + E_NAME_LEN)? as usize; read_name(p, l)
}

// ★resolve 재구현 (WorldOps resolve_by_key 0x2305520 동형): world+key → entity(stride 0x6a8).
unsafe fn resolve(world: usize, key: u64) -> usize {
    if world < 0x10000 { return 0; }
    let slot_len = rd_u64(world + W_SLOT_LEN).unwrap_or(0);
    if key < slot_len {
        if let Some(slots) = rd_u64(world + W_SLOTS) {
            let entry = (slots as usize).wrapping_add(key as usize * 0x10);
            if rd_u64(entry).unwrap_or(0) as u32 == 1 {
                let di = rd_u64(entry + 8).unwrap_or(u64::MAX);
                if di < rd_u64(world + W_DENSE_LEN).unwrap_or(0) {
                    if let Some(db) = rd_u64(world + W_DENSE_BASE) {
                        let ent = (db as usize).wrapping_add(di as usize * ENT_STRIDE);
                        if readable(ent, ENT_STRIDE) { return ent; }
                    }
                }
            }
        }
    }
    if rd_u64(world + W_FB_KEY).unwrap_or(u64::MAX ^ 1) == key
        && (rd_u64(world + W_FB_ENT).unwrap_or(0) as i32) != -1 { return world + W_FB_ENT; }
    0
}

static WRITE_LOCK: Mutex<()> = Mutex::new(());
fn hlog(s: &str) {
    use std::io::Write;
    let _g = WRITE_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true)
        .open("C:\\tfm2mods\\sylas_hijack\\hijack_grab_log.txt") { let _ = f.write_all(s.as_bytes()); }
}

static ARMED: AtomicBool = AtomicBool::new(false);
fn cfg_refresher() {
    std::thread::spawn(|| loop {
        let s = std::fs::read_to_string("C:\\tfm2mods\\sylas_hijack\\sylas_hijack.cfg").unwrap_or_default();
        ARMED.store(s.contains("arm=1"), Ordering::Relaxed);
        std::thread::sleep(Duration::from_secs(2));
    });
}

// base 궁 apply: rcx=action_data, rdx=world, r8=worldops, r9=target, [rsp+0x28]=casting_ctx. → u64.
type BaseApply = unsafe extern "C" fn(usize, usize, usize, usize, usize) -> u64;
unsafe fn call_base_apply(f_addr: usize, action: usize, world: usize, wops: usize, target: usize, cctx: usize) -> Option<u64> {
    let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let f: BaseApply = core::mem::transmute(f_addr);
        f(action, world, wops, target, cctx)
    }));
    r.ok()
}

static HJ_N: AtomicU32 = AtomicU32::new(0);
static LOG_N: AtomicU32 = AtomicU32::new(0);
static DIAG: AtomicU32 = AtomicU32::new(0);
static SYLAS_DUMPED: AtomicBool = AtomicBool::new(false);
// 중복 발화 방지: 최근 강탈한 (sylas_key) + 시각. 같은 궁 시전(Grab 여러틱)에 1회만.
static LAST_KEY: AtomicU64 = AtomicU64::new(0);
static LAST_TS:  AtomicU64 = AtomicU64::new(0);
#[inline] fn qpc() -> u64 { let mut c=0i64; unsafe { QueryPerformanceCounter(&mut c); } c as u64 }

unsafe fn hijack_grab(saved: usize, e: usize) {
    if !ARMED.load(Ordering::Relaxed) { return; }
    let rcx = *((saved + 0x28) as *const u64) as usize; // a1 effect_def
    let rdx = *((saved + 0x20) as *const u64) as usize; // a2
    let world = *((saved + 0x18) as *const u64) as usize; // r8 = world(추정)
    let r9    = *((saved + 0x10) as *const u64) as usize; // r9 = WorldOps(추정)
    // ★진단: Grab apply 진입 raw + 게이트 통과 실측 (처음 40건, arm 무관 아래 게이트 전)
    let diag = DIAG.fetch_add(1, Ordering::Relaxed) < 40;
    let wops = exe_base() + WORLDOPS_RVA;
    if diag {
        let s0 = rd_u64(e + 0x28).unwrap_or(0);
        let s1 = rd_u64(e + 0x30).unwrap_or(0);
        let s2 = rd_u64(e + 0x38).unwrap_or(0);
        let s3 = rd_u64(e + 0x40).unwrap_or(0);
        let s4 = rd_u64(e + 0xa0).unwrap_or(0);
        hlog(&format!("[진입] rcx={:#x} rdx={:#x} r8={:#x} r9={:#x} wops={:#x} r9match={}\n  stk[+28..+40]={:#x} {:#x} {:#x} {:#x} [+a0]={:#x}\n",
            rcx, rdx, world, r9, wops, r9==wops, s0, s1, s2, s3, s4));
        // ★사일러스 entity 궁슬롯 apply 실측 (1회, world 순회) — 사일러스 궁이 실제 부르는 apply RVA 확정용
        if !SYLAS_DUMPED.load(Ordering::Relaxed) && world >= 0x10000 {
            let db = rd_u64(world + W_DENSE_BASE).unwrap_or(0) as usize;
            let dl = rd_u64(world + W_DENSE_LEN).unwrap_or(0);
            if db >= 0x10000 && dl >= 1 && dl <= 512 {
                for i in 0..dl as usize {
                    let ent = db.wrapping_add(i * ENT_STRIDE);
                    if !readable(ent, ENT_STRIDE) { continue; }
                    if ent_name(ent).as_deref() == Some("sylas") {
                        SYLAS_DUMPED.store(true, Ordering::Relaxed);
                        let cool = rd_u64(ent + 0xC8).unwrap_or(0);
                        let mut s = format!("★[SYLAS덤프] ent={:#x} ult쿨(+0xC8)={:#x}\n", ent, cool);
                        for (lbl, off) in [("atk",0x118usize),("sk",0x128),("sk2",0x138),("ult",0x148)] {
                            let act = rd_u64(ent + off).unwrap_or(0);
                            let vt = rd_u64(ent + off + 8).unwrap_or(0);
                            let a20 = rd_u64(vt as usize + 0x20).unwrap_or(0);
                            let ad0 = rd_u64(vt as usize + 0xd0).unwrap_or(0);
                            s.push_str(&format!("  {} act={:#x} vt=RVA:{:#x} [vt+0x20]=RVA:{:#x} [vt+0xd0]=RVA:{:#x}\n",
                                lbl, act, rva_of(vt), rva_of(a20), rva_of(ad0)));
                        }
                        // 궁슬롯(+0x148) act가 Combine이면 자식 effect들도: act+8=ptr, act+0x10=count, stride 0x10 {data,vt}
                        let ua = rd_u64(ent + 0x148).unwrap_or(0) as usize;
                        if ua >= 0x10000 {
                            let cp = rd_u64(ua + 8).unwrap_or(0) as usize;
                            let cc = rd_u64(ua + 0x10).unwrap_or(0);
                            s.push_str(&format!("  ult자식 ptr={:#x} count={}\n", cp, cc));
                            if cp >= 0x10000 && cc >= 1 && cc <= 16 {
                                for j in 0..cc as usize {
                                    let cvt = rd_u64(cp + j*0x10 + 8).unwrap_or(0);
                                    let cap20 = rd_u64(cvt as usize + 0x20).unwrap_or(0);
                                    s.push_str(&format!("    child[{}] vt=RVA:{:#x} [vt+0x20]=RVA:{:#x}\n", j, rva_of(cvt), rva_of(cap20)));
                                }
                            }
                        }
                        hlog(&s);
                        break;
                    }
                }
            }
        }
    }
    // Grab apply 정합 확인: r9 == base+0x38c5d78 (아니면 다른 함수/오분석 → 중단)
    if r9 != wops { if diag { hlog("  ✗r9!=wops\n"); } return; }
    if world < 0x10000 { if diag { hlog("  ✗world\n"); } return; }
    // 스택 인자: e=orig_rsp. a6=[e+0x30]=target_key, a7=[e+0x38]=casting_ctx
    let target_key = match rd_u64(e + 0x30) { Some(v) => v, None => return };
    let cctx = match rd_u64(e + 0x38) { Some(v) => v as usize, None => return };
    if cctx < 0x10000 { if diag { hlog(&format!("  ✗cctx={:#x}\n", cctx)); } return; }
    let sylas_key = match rd_u64(cctx + 8) { Some(v) => v, None => return };
    // casting_ctx.tag==0 (fresh) 가드
    let tag = rd_u64(cctx).unwrap_or(1) as u32;
    if tag != 0 { if diag { hlog(&format!("  ✗tag={}\n", tag)); } return; }

    // caster = sylas 판별
    let sylas = resolve(world, sylas_key);
    if sylas == 0 { if diag { hlog(&format!("  ✗resolve(sylas_key={}) 실패\n", sylas_key as i64)); } return; }
    let sname = ent_name(sylas);
    if sname.as_deref() != Some("sylas") { if diag { hlog(&format!("  ✗caster={:?}(≠sylas) key={}\n", sname, sylas_key as i64)); } return; }
    if diag { hlog(&format!("  ✓sylas={:#x} tk={} → 계속\n", sylas, target_key as i64)); }

    // X = 붙잡은 적 (강탈 대상)
    if target_key >= (rd_u64(world + W_SLOT_LEN).unwrap_or(0)) { if diag { hlog(&format!("  ✗tk={} ≥ slot_len\n", target_key as i64)); } return; }
    let x = resolve(world, target_key);
    if x == 0 || x == sylas { if diag { hlog(&format!("  ✗X resolve(tk={})={:#x} (0 or ==sylas)\n", target_key as i64, x)); } return; }
    let xname = ent_name(x).unwrap_or_default();
    let xalive = rd_u64(x + E_ALIVE).map(|v| v as i32).unwrap_or(-1);
    if diag { hlog(&format!("  ✓X={:#x} name={:?} alive(+0x450)={}\n", x, xname, xalive)); }
    // X 생존 확인
    if xalive != 0 { if diag { hlog("  ✗X 死\n"); } return; }

    // X 궁 apply + action (base effect-action: [X+0x150 vtable +0xd0] = apply)
    let x_action = match rd_u64(x + S_ULT_ACTION) { Some(v) => v as usize, None => return };
    let x_vtbl   = match rd_u64(x + S_ULT_VTBL) { Some(v) => v as usize, None => return };
    if diag { hlog(&format!("  X_action={:#x} X_vtbl=RVA:{:#x} [vt+0xd0]=RVA:{:#x} [vt+0x20]=RVA:{:#x}\n",
        x_action, rva_of(x_vtbl as u64), rva_of(rd_u64(x_vtbl + 0xd0).unwrap_or(0)), rva_of(rd_u64(x_vtbl + 0x20).unwrap_or(0)))); }
    if x_action < 0x10000 || !in_exe(x_vtbl as u64) { if diag { hlog("  ✗x_action/vtbl\n"); } return; }
    let x_apply = match rd_u64(x_vtbl + V_APPLY_BASE) { Some(v) => v as usize, None => return };
    if !in_exe(x_apply as u64) { if diag { hlog(&format!("  ✗x_apply=RVA:{:#x} !in_exe\n", rva_of(x_apply as u64))); } return; }

    // 중복 방지: 같은 sylas_key + 300ms 내 재발화 skip (Grab이 여러 틱 불림)
    let now = qpc();
    if LAST_KEY.load(Ordering::Relaxed) == sylas_key {
        let dt = now.wrapping_sub(LAST_TS.load(Ordering::Relaxed));
        if dt < 500_000_000 { return; } // QPC~10MHz 가정 넉넉히 (실제 dedup은 tick 게이트로 충분)
    }
    LAST_KEY.store(sylas_key, Ordering::Relaxed);
    LAST_TS.store(now, Ordering::Relaxed);

    // ★강탈: X 궁 base apply를 sylas caster로 CALL. casting_ctx=cctx(그대로={tag0,sylas_key}). target=X.
    let wops = exe_base() + WORLDOPS_RVA;
    let ret = call_base_apply(x_apply, x_action, world, wops, x, cctx);
    let n = HJ_N.fetch_add(1, Ordering::Relaxed);
    if LOG_N.fetch_add(1, Ordering::Relaxed) < 60 {
        hlog(&format!("[강탈#{}] sylas={:#x} ← {} 궁 apply=RVA:{:#x} action={:#x} ret={:?}\n",
            n, sylas, xname, rva_of(x_apply as u64), x_action, ret.map(|v| v & 0xffff_ffff)));
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
    s.extend_from_slice(&orig);                                               // 원본 프롤로그(Grab 계속)
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

unsafe extern "C" fn cap_grab(saved: usize, e: usize) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| hijack_grab(saved, e)));
}

static HOOK_INSTALLED: AtomicBool = AtomicBool::new(false);
fn init(_ctx: &GameCtx) -> ModRegistration {
    std::thread::spawn(|| {
        std::thread::sleep(Duration::from_secs(6));
        if HOOK_INSTALLED.swap(true, Ordering::Relaxed) { return; }
        veh_install();
        cfg_refresher();
        let base = exe_base();
        hlog(&format!("\n===== sylas_hijack GRAB강탈(base apply 직접CALL) 시작 base={:#x} =====\n", base));
        match unsafe { install_detour(GRAB_RVA, GRAB_LEN, &GRAB_SIG, cap_grab as *const () as usize) } {
            Ok(stub) => hlog(&format!("[install] Grab_apply @{:#x} OK stub={:#x}\n", base + GRAB_RVA, stub)),
            Err(e)   => hlog(&format!("[install] Grab_apply 실패: {}\n", e)),
        }
    });
    ModRegistration::new("sylas_hijack")
}

declare_mod!(init);
