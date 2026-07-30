// tfm2_rush_guard.rs — RushMoveToBack None-패닉 가드 (백그라운드 sim 멈춤 수정)
// =====================================================================================
// 버그: RushMoveToBackEffect::apply (FUN_141ff8ec0) 의 resolve#2 가 대시 대상 엔티티 핸들을
//   None 가드 없이 unwrap → 대상이 apply 직전에 죽어 사라지면 패닉("panic occurred!",
//   rush_move_to_back.rs:29:59) → sim 스레드 사망 → 백그라운드 배치(일정넘김) 멈춤.
//   (형제 resolve#1 은 None 이면 그냥 스킵하는데 resolve#2 만 패닉 = 엔진 버그.)
//   런타임 프로브(tfm2_panic_probe)로 실측 확정. squirrel(#1)·tale(#2) 등 RushMoveToBack 챔프.
//
// 수정: 패닉 분기 JZ @0x1ff8f66 (je 0x1ff9315=panic) 의 목적지를 0x1ff9301(정상 종료 epilogue,
//   형제 resolve#1 의 None-스킵과 동일 지점)로 바꾼다. rel32 만 변경 → 바이트 1개(a9→95).
//   대상이 None 이면 대시 효과를 조용히 스킵 후 clean return. 정상 케이스 무영향.
//
//   0x1ff8f66: 0F 84 A9 03 00 00   je 0x1ff9315 (panic)   ← 원본
//              0F 84 95 03 00 00   je 0x1ff9301 (return)   ← 패치
//   (0x1ff9301 - 0x1ff8f6c = 0x395,  0x1ff9315 - 0x1ff8f6c = 0x3a9,  차이 0x14)
//
// 빌드: powershell -ExecutionPolicy Bypass -File C:\tfm2mods\build_inj.ps1 -Src C:\tfm2mods\tfm2_rush_guard\src\tfm2_rush_guard.rs -ModId tfm2_rush_guard
// =====================================================================================

use mod_api::*;
use std::path::PathBuf;
use std::fs;
use std::io::Write;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

const MOD_ID: &str = "tfm2_rush_guard";

// ── 패치 테이블(0.4.14 핫픽스, image_base 0x140000000) ──
// 각 항목 = 이동/변위 이펙트 핸들러의 "대상 resolve→None→가드없는 unwrap 패닉" 분기 JZ를
//   형제 resolve의 스킵(clean return) 목적지로 돌림. rel32만 바뀜(6B JZ). 크래시 케이스만 변경.
struct Patch { name: &'static str, rva: usize, orig: &'static [u8], fixed: &'static [u8] }
// 각 항목 = 패닉 JZ의 목적지를 형제 resolve 스킵/함수 clean-return으로 리다이렉트(rel만 변경, 명령길이·스택깊이 불변).
// ghidra 전수조사 확정(스택깊이 일치 검증됨). 크래시 케이스(대상 None)만 스킵, 정상 무영향.
const PATCHES: &[Patch] = &[
    // RushMoveToBack (rush_move_to_back.rs:29) — ✅런타임 검증됨(모델)
    Patch { name: "RushMoveToBack", rva: 0x1ff8f66,
            orig: &[0x0f,0x84,0xa9,0x03,0x00,0x00], fixed: &[0x0f,0x84,0x95,0x03,0x00,0x00] },
    // MoveTo (move_to.rs) — 4개 case(83/24/33/29)
    Patch { name: "MoveTo:83",  rva: 0x1b3c73c,
            orig: &[0x0f,0x84,0xf7,0x01,0x00,0x00], fixed: &[0x0f,0x84,0x9e,0xff,0xff,0xff] },
    Patch { name: "MoveTo:24",  rva: 0x1b1d6d6,
            orig: &[0x0f,0x84,0x07,0x04,0x00,0x00], fixed: &[0x0f,0x84,0xda,0x03,0x00,0x00] },
    Patch { name: "MoveTo:33",  rva: 0x1b1d7b3,
            orig: &[0x0f,0x84,0x0e,0x03,0x00,0x00], fixed: &[0x0f,0x84,0xfd,0x02,0x00,0x00] },
    Patch { name: "MoveTo:29",  rva: 0x1b1d85a,
            orig: &[0x0f,0x84,0x75,0x02,0x00,0x00], fixed: &[0x0f,0x84,0x56,0x02,0x00,0x00] },
    // RushTime (rush_time.rs:64) — harpy #4 궁
    Patch { name: "RushTime",   rva: 0x1cd6c3a,
            orig: &[0x0f,0x84,0xda,0x02,0x00,0x00], fixed: &[0x0f,0x84,0x73,0x02,0x00,0x00] },
    // MoveBack (moveback.rs:20) — SHORT jump(2B)
    Patch { name: "MoveBack",   rva: 0x1950aa6,
            orig: &[0x74,0x27], fixed: &[0x74,0x1b] },
    // Airborne (airborne.rs:34) — knockup/knockback
    Patch { name: "Airborne",   rva: 0x214faa3,
            orig: &[0x0f,0x84,0x93,0x00,0x00,0x00], fixed: &[0x0f,0x84,0x36,0xfe,0xff,0xff] },
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
        p.push("tfm2_rush_guard.txt");
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
    log(&format!("[{}ms] === tfm2_rush_guard INIT ({} patches) ===\n", now_ms(), PATCHES.len()));
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
