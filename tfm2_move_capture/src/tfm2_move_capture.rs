// tfm2_move_capture.rs — 이동 이펙트 패닉의 param_6(대상 핸들) 실측 캡처
// =====================================================================================
// 목적: RushMoveToBack apply(FUN_141ff8ec0) 진입에서 param_6(변위 대상 핸들, [rsp+0x30])와
//   레지스트리(R8=saved[2])를 읽어, 그 핸들이 무효일 때 정체를 로깅:
//     -1(0xFFFFFFFF)=무타겟 / slot state=0=엔티티 제거됨(append-only 반증) / 범위밖=foreign.
//   "왜 사람마다 크래시 유무가 다른가"의 진짜 트리거 변수를 실측으로 확정.
//   ★move_guard 켠 채로 돌려도 됨(진입훅이 apply 실행 전에 param_6를 읽으므로, 가드가 스킵해도 캡처됨) → 멈춤 없이 수집.
//
// 레지스트리 레이아웃(agent 규명, best-effort 가드 read): slot_table=[reg+0x700], count=[reg+0x6f0],
//   array=[reg+0x6e8], stride 0x6a8, slot16B{state@0,dense@8}, type@entity+0x68, name@entity+0x250.
//
// 빌드: powershell -ExecutionPolicy Bypass -File C:\tfm2mods\build_inj.ps1 -Src C:\tfm2mods\tfm2_move_capture\src\tfm2_move_capture.rs -ModId tfm2_move_capture
// =====================================================================================

use mod_api::*;
use std::path::PathBuf;
use std::fs;
use std::io::Write;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

const MOD_ID: &str = "tfm2_move_capture";

// RushMoveToBack apply 진입(0.4.14). 프롤로그 12B: push rbp/r15/r14/r13/r12/rsi/rdi/rbx
const RVA_RMTB: usize = 0x1ff8ec0;
const RMTB_LEN: usize = 12;
const RMTB_EXPECT: [u8; 12] = [0x55,0x41,0x57,0x41,0x56,0x41,0x55,0x41,0x54,0x56,0x57,0x53];

const PARAM6_OFF: usize = 0x30;   // 진입 시 [rsp+0x30] = param_6(변위 대상 핸들)
const PARAM7_OFF: usize = 0x38;   // [rsp+0x38] = param_7(source/ref)

static IN_HOOK: AtomicBool = AtomicBool::new(false);
static N_ALL: AtomicU64 = AtomicU64::new(0);
static N_BAD: AtomicU64 = AtomicU64::new(0);
const BAD_LOG_LIMIT: u64 = 200;
const ALL_LOG_LIMIT: u64 = 40;    // 정상 케이스도 앞부분 몇 개는 찍어 분포 확인

type BOOL = i32; type DWORD = u32; type HMODULE = usize;
extern "system" {
    fn GetModuleHandleW(name: *const u16) -> usize;
    fn GetModuleHandleExW(flags: u32, addr: *const u16, h: *mut HMODULE) -> BOOL;
    fn GetModuleFileNameW(h: HMODULE, buf: *mut u16, sz: DWORD) -> DWORD;
    fn VirtualAlloc(addr: usize, sz: usize, typ: u32, prot: u32) -> usize;
    fn VirtualProtect(addr: usize, size: usize, new_protect: u32, old: *mut u32) -> BOOL;
    fn VirtualQuery(addr: *const core::ffi::c_void, buf: *mut MemBasicInfo, len: usize) -> usize;
    fn FlushInstructionCache(proc: usize, addr: usize, sz: usize) -> BOOL;
    fn GetCurrentProcess() -> usize;
}
#[repr(C)] #[derive(Default)]
struct MemBasicInfo { base: usize, alloc_base: usize, alloc_protect: u32, _p0: u32,
    region_size: usize, state: u32, protect: u32, typ: u32, _p1: u32 }

static EXE_BASE: AtomicU64 = AtomicU64::new(0);
#[inline] unsafe fn exe_base() -> usize {
    let v = EXE_BASE.load(Ordering::Relaxed) as usize;
    if v != 0 { return v; }
    let b = GetModuleHandleW(core::ptr::null());
    EXE_BASE.store(b as u64, Ordering::Relaxed); b
}
#[inline] unsafe fn readable(addr: usize, len: usize) -> bool {
    if addr < 0x10000 || len == 0 { return false; }
    let mut mbi = MemBasicInfo::default();
    if VirtualQuery(addr as *const _, &mut mbi, core::mem::size_of::<MemBasicInfo>()) == 0 { return false; }
    const COMMIT: u32 = 0x1000; const RD: u32 = 0x02|0x04|0x20|0x40; const GUARD: u32 = 0x01|0x100;
    if mbi.state != COMMIT || mbi.protect & GUARD != 0 || mbi.protect & RD == 0 { return false; }
    addr + len <= mbi.base + mbi.region_size
}
#[inline] unsafe fn rd_u64(a: usize) -> Option<u64> { if readable(a,8){Some(core::ptr::read_unaligned(a as *const u64))}else{None} }
#[inline] unsafe fn rd_u32(a: usize) -> Option<u32> { if readable(a,4){Some(core::ptr::read_unaligned(a as *const u32))}else{None} }
#[inline] unsafe fn rd_u8(a: usize) -> Option<u8> { if readable(a,1){Some(core::ptr::read_unaligned(a as *const u8))}else{None} }
#[inline] fn ptr_ok(a: usize) -> bool { a >= 0x10000 && a < (1usize<<48) }

fn now_ms() -> u64 { SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0) }
fn dir() -> Option<PathBuf> { unsafe {
    let mut h: HMODULE = 0;
    if GetModuleHandleExW(0x4|0x2, dir as *const () as *const u16, &mut h) == 0 || h == 0 { return None; }
    let mut b = [0u16; 4096];
    let n = GetModuleFileNameW(h, b.as_mut_ptr(), b.len() as DWORD);
    if n == 0 { return None; }
    let mut p = PathBuf::from(String::from_utf16_lossy(&b[..n as usize])); p.pop(); Some(p)
}}
fn log(s: &str) {
    if let Some(mut p) = dir() { p.push("tfm2_move_capture.txt");
        if let Ok(mut f)=fs::OpenOptions::new().create(true).append(true).open(&p){let _=write!(f,"{}",s);let _=f.flush();}
    }
}
unsafe fn ent_name(e: usize) -> String {
    let p = match rd_u64(e + 0x250) { Some(p)=>p as usize, None=>return String::new() };
    if p < 0x10000 { return String::new(); }
    let mut b = Vec::new();
    for i in 0..40usize { match rd_u8(p+i){Some(0)|None=>break,Some(c)=>b.push(c)} }
    String::from_utf8_lossy(&b).into_owned()
}
// 레지스트리에서 핸들 resolve 시도. 반환 = 사람이 읽는 설명(정상/무효 원인).
unsafe fn resolve_desc(reg: usize, handle: u64) -> String {
    let h = handle as i64;
    if h == -1 || handle == 0xFFFFFFFF { return "NO_TARGET(-1)".into(); }
    if !ptr_ok(reg) { return format!("h=0x{:x} (reg unreadable)", handle); }
    let slot_tbl = match rd_u64(reg + 0x700) { Some(v)=>v as usize, None=>return format!("h=0x{:x} (no slot_tbl@+0x700)", handle) };
    let count = rd_u64(reg + 0x6f0).unwrap_or(0);
    let array = rd_u64(reg + 0x6e8).unwrap_or(0) as usize;
    if !ptr_ok(slot_tbl) || handle > 0x100000 { return format!("h=0x{:x} FOREIGN?(huge/oob, count={})", handle, count); }
    let slot = slot_tbl + (handle as usize) * 0x10;
    let state = match rd_u32(slot) { Some(v)=>v, None=>return format!("h=0x{:x} (slot unreadable, count={})", handle, count) };
    let dense = rd_u64(slot + 8).unwrap_or(u64::MAX);
    if state != 1 {
        return format!("h=0x{:x} ★REAPED(state={} != 1)=엔티티제거됨! dense={}", handle, state, dense);
    }
    if dense >= count { return format!("h=0x{:x} state=1 but dense={}>=count={} (stale)", handle, dense, count); }
    let ent = array + (dense as usize) * 0x6a8;
    let etype = rd_u32(ent + 0x68).unwrap_or(0xffffffff);
    format!("h=0x{:x} VALID type={} name={}", handle, etype, ent_name(ent))
}

#[no_mangle]
pub extern "C" fn rmtb_cap(saved: *mut u64, entry_rsp: usize) {
    if IN_HOOK.swap(true, Ordering::Relaxed) { return; }
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe {
        if saved.is_null() || entry_rsp == 0 { return; }
        let reg = *saved.add(2) as usize;                       // r8 = registry(resolver self)
        let param6 = rd_u64(entry_rsp + PARAM6_OFF).unwrap_or(0);
        let param7 = rd_u64(entry_rsp + PARAM7_OFF).unwrap_or(0);
        // 무효 판정: -1 or resolve 실패
        let d6 = resolve_desc(reg, param6);
        let bad = d6.contains("NO_TARGET") || d6.contains("REAPED") || d6.contains("FOREIGN") || d6.contains("stale") || d6.contains("unreadable");
        let na = N_ALL.fetch_add(1, Ordering::Relaxed);
        if bad {
            let nb = N_BAD.fetch_add(1, Ordering::Relaxed);
            if nb < BAD_LOG_LIMIT {
                let d7 = resolve_desc(reg, param7);
                log(&format!("[BAD #{} {}ms] RushMoveToBack param6 {} | param7(src) {} | reg=0x{:x}\n", nb, now_ms(), d6, d7, reg));
            }
        } else if na < ALL_LOG_LIMIT {
            log(&format!("[ok  #{}] param6 {}\n", na, d6));
        }
    }));
    IN_HOOK.store(false, Ordering::Relaxed);
}

unsafe fn install(rva: usize, len: usize, expect: &[u8], cap: usize) -> Result<(),String> {
    let base = exe_base(); if base==0 {return Err("mod0".into());}
    let fna = base+rva;
    if !readable(fna, len+4) { return Err("unreadable".into()); }
    let mut cur=vec![0u8;len]; core::ptr::copy_nonoverlapping(fna as *const u8, cur.as_mut_ptr(), len);
    if cur.as_slice()!=expect { return Err(format!("byte mismatch {:02x?}",cur)); }
    const MEM_CR:u32=0x1000|0x2000; const RWX:u32=0x40;
    let stub=VirtualAlloc(0,256,MEM_CR,RWX); if stub==0 {return Err("valloc".into());}
    let ret=fna+len;
    let mut s:Vec<u8>=Vec::new();
    s.extend_from_slice(&[0x49,0x89,0xe2]);                                    // mov r10,rsp
    s.extend_from_slice(&[0x51,0x52,0x41,0x50,0x41,0x51,0x41,0x52,0x41,0x53]); // push rcx rdx r8 r9 r10 r11
    s.extend_from_slice(&[0x48,0x89,0xe1]);                                    // mov rcx,rsp (&saved)
    s.extend_from_slice(&[0x4c,0x89,0xd2]);                                    // mov rdx,r10 (entry_rsp)
    s.extend_from_slice(&[0x48,0x83,0xec,0x28]);                               // sub rsp,0x28
    s.extend_from_slice(&[0x48,0xb8]); s.extend_from_slice(&cap.to_le_bytes());
    s.extend_from_slice(&[0xff,0xd0]);                                         // call rax
    s.extend_from_slice(&[0x48,0x83,0xc4,0x28]);                               // add rsp,0x28
    s.extend_from_slice(&[0x41,0x5b,0x41,0x5a,0x41,0x59,0x41,0x58,0x5a,0x59]); // pop r11 r10 r9 r8 rdx rcx
    let mut orig=vec![0u8;len]; core::ptr::copy_nonoverlapping(fna as *const u8, orig.as_mut_ptr(), len);
    s.extend_from_slice(&orig);
    s.extend_from_slice(&[0x48,0xb8]); s.extend_from_slice(&ret.to_le_bytes());
    s.extend_from_slice(&[0xff,0xe0]);
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    let mut patch=vec![0x90u8;len]; patch[0]=0x48;patch[1]=0xb8;patch[2..10].copy_from_slice(&stub.to_le_bytes());patch[10]=0xff;patch[11]=0xe0;
    let mut old:u32=0;
    if VirtualProtect(fna,len,RWX,&mut old)==0 {return Err("vprotect".into());}
    core::ptr::copy_nonoverlapping(patch.as_ptr(), fna as *mut u8, len);
    VirtualProtect(fna,len,old,&mut old);
    FlushInstructionCache(GetCurrentProcess(),fna,len);
    Ok(())
}

fn init(_ctx: &GameCtx) -> ModRegistration {
    if let Some(mut p)=dir(){ p.push("tfm2_move_capture.txt"); let _=fs::write(&p, format!("[{}ms] === move_capture INIT ===\n", now_ms())); }
    unsafe {
        match install(RVA_RMTB, RMTB_LEN, &RMTB_EXPECT, rmtb_cap as *const() as usize) {
            Ok(())=>log("[hook] RushMoveToBack @0x1ff8ec0 OK\n"),
            Err(e)=>log(&format!("[hook] 실패: {}\n", e)),
        }
    }
    ModRegistration::new(MOD_ID)
}
declare_mod!(init);
