// tfm2_move_guard.rs — 이동/변위 이펙트 None-패닉 가드 (백그라운드 sim 멈춤 수정)
// =====================================================================================
// 버그: 데이터-이펙트 VM의 이동/변위 이펙트 apply(RushMoveToBack/MoveTo/MoveBack/RushTime/Airborne)가
//   대상 엔티티 핸들을 None 가드 없이 unwrap → 대상이 apply 직전에 죽어 레지스트리에서 사라지면
//   패닉("panic occurred!") → sim 스레드 사망 → 백그라운드 배치(일정넘김) 멈춤.
//   렌더링 실경기에선 안 남(데이터-이펙트 경로=모드 챔프 전용, 바닐라 하드코딩 경로는 면역).
//   런타임 프로브(tfm2_panic_probe)로 실측 확정. leef #1~#3(squirrel/tale/amazon/hellion 등).
//
// 수정: 각 이펙트 핸들러의 "대상 None → 패닉" 분기 JZ의 목적지를, 형제 resolve가 이미 쓰는
//   None-스킵(clean return) 지점으로 리다이렉트. rel32만 변경(명령 길이·스택깊이 불변).
//   대상이 None이면 이동 이펙트를 조용히 스킵 후 정상 종료. 정상 케이스·쿨타임 무영향
//   (쿨타임은 시전 시점에 이미 확정, 이펙트 apply는 그 뒤). 게임 자신의 형제-resolve 처리와 동형.
//
// 빌드: powershell -ExecutionPolicy Bypass -File C:\tfm2mods\build_inj.ps1 -Src C:\tfm2mods\tfm2_move_guard\src\tfm2_move_guard.rs -ModId tfm2_move_guard
// =====================================================================================

use mod_api::*;
use std::path::PathBuf;
use std::fs;
use std::io::Write;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

const MOD_ID: &str = "tfm2_move_guard";

// ── 패치 테이블(0.4.14 핫픽스, image_base 0x140000000) ──
// 각 항목 = 이동/변위 이펙트 핸들러의 "대상 resolve→None→가드없는 unwrap 패닉" 분기 JZ를
//   형제 resolve의 스킵(clean return) 목적지로 돌림. rel만 바뀜(6B near JZ / 2B short JE).
//   ghidra 전수조사 확정(스택깊이 일치 검증). 크래시 케이스(대상 None)만 스킵, 정상 무영향.
struct Patch { name: &'static str, rva: usize, orig: &'static [u8], fixed: &'static [u8] }
// ★0.5.0(buildid 24102827): 이동이펙트는 0.4.14와 동일한 per-effect 파일(rush_move_to_back.rs/moveback.rs/move_to.rs)
//   그대로, 주소만 이동. 프로브 실측으로 확정(caller_rva→file:line). 각 apply가 target 핸들(vtable slot+0x158)을
//   null 가드 없이 unwrap→패닉헬퍼 0x2bee240. 형제 source(slot+0x150)는 null이면 skip(비대칭=버그 존속).
//   패치=패닉 JZ 목적지를 clean epilogue(순수 return)로 rel변경. 3사이트.
//   (⚠앞선 small_action.rs 0x231e3f0/0x231e4a5=오조사=엉뚱한 함수, 폐기. move_to 24/33/29→0.5.0 단일통합,
//    rush_time/airborne→0.5.0 패치불요[통합 or 이미 가드]. 상세=기록.)
const PATCHES: &[Patch] = &[
    // rush_move_to_back.rs:29 (RushMoveToBack, squirrel/tale) — je 0x1e7d775(panic)→0x1e7d761(epilogue)
    Patch { name: "rush_move_to_back:29", rva: 0x1e7d3c6,
            orig: &[0x0f,0x84,0xa9,0x03,0x00,0x00], fixed: &[0x0f,0x84,0x95,0x03,0x00,0x00] },
    // moveback.rs:20 (MoveBack, tale/hellion) — SHORT je(2B) 0x1f69fef(panic)→0x1f69fe3(epilogue). rel8 27→1b
    Patch { name: "moveback:20", rva: 0x1f69fc6,
            orig: &[0x74,0x27], fixed: &[0x74,0x1b] },
    // move_to.rs:83 (MoveTo, 0.5.0 단일 case) — je 0x1f9e119(panic)→0x1f9dec0(epilogue)
    Patch { name: "move_to:83", rva: 0x1f9df1c,
            orig: &[0x0f,0x84,0xf7,0x01,0x00,0x00], fixed: &[0x0f,0x84,0x9e,0xff,0xff,0xff] },
];

type BOOL = i32; type DWORD = u32; type HMODULE = usize;
extern "system" {
    fn GetModuleHandleW(name: *const u16) -> usize;
    fn GetModuleHandleExW(flags: u32, addr: *const u16, h: *mut HMODULE) -> BOOL;
    fn GetModuleFileNameW(h: HMODULE, buf: *mut u16, sz: DWORD) -> DWORD;
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
    if let Some(mut p) = dir() {
        p.push("tfm2_move_guard.txt");
        if let Ok(mut f) = fs::OpenOptions::new().create(true).append(true).open(&p) { let _ = write!(f, "{}", s); let _ = f.flush(); }
    }
}

unsafe fn apply_one(p: &Patch) -> Result<&'static str, String> {
    let base = exe_base(); if base == 0 { return Err("module 0".into()); }
    let n = p.orig.len();                                     // ★ 사이트별 길이(6=near JZ, 2=short JE)
    let addr = base + p.rva;
    if !readable(addr, n) { return Err("addr unreadable".into()); }
    let mut buf = [0u8; 8];
    core::ptr::copy_nonoverlapping(addr as *const u8, buf.as_mut_ptr(), n);
    let cur = &buf[..n];
    if cur == p.fixed { return Ok("already"); }              // 멱등
    if cur != p.orig { return Err(format!("byte mismatch (RVA stale?) cur={:02x?}", cur)); }
    const RWX: u32 = 0x40;
    let mut old: u32 = 0;
    if VirtualProtect(addr, n, RWX, &mut old) == 0 { return Err("VirtualProtect".into()); }
    core::ptr::copy_nonoverlapping(p.fixed.as_ptr(), addr as *mut u8, n);
    VirtualProtect(addr, n, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), addr, n);
    Ok("patched")
}

fn init(_ctx: &GameCtx) -> ModRegistration {
    log(&format!("[{}ms] === tfm2_move_guard INIT ({} patches) ===\n", now_ms(), PATCHES.len()));
    unsafe {
        for p in PATCHES {
            match apply_one(p) {
                Ok(st) => log(&format!("[patch] {} @0x{:x} {}\n", p.name, p.rva, st)),
                Err(e) => log(&format!("[patch] {} @0x{:x} 실패: {}\n", p.name, p.rva, e)),
            }
        }
    }
    ModRegistration::new(MOD_ID)
}
declare_mod!(init);
