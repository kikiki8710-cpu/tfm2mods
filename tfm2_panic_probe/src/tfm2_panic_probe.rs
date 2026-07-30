// tfm2_panic_probe.rs — sim 패닉 지점 런타임 확인 프로브
// =====================================================================================
// 목적: 백그라운드 경기 sim 중 Rust 패닉("panic occurred!")이 터지는 정확한 지점을 로그로 확증.
//   Rust 패닉 헬퍼(unwrap-None / bounds-check / slice-OOB)를 진입 훅 →
//   패닉 순간의 (1) 호출자 return address(=진짜 패닉난 게임함수 RVA)
//              (2) &Location 에서 file:line (게임 소스 경로·줄)
//              (3) 스택의 in-exe 복귀주소 미니 백트레이스
//   를 파일에 기록 후 원본 헬퍼로 복귀(게임은 그대로 패닉/abort — 우리 로그는 이미 flush됨).
//
// ★유력 가설(정적 확증): harpy 궁 RushTime 매틱 핸들러 FUN_141cd08e0 의 unwrap.
//   패닉 콜사이트 0x1cd6f1a: `lea rcx,[rip+..]; call 0x2957be0(unwrap-None); ud2`.
//   → return address = 0x1cd6f26. 로그에 caller_rva=0x1cd6f26 찍히면 RushTime 확정.
//
// 안전: log-first(원본 실행 전 로깅) → 원본 재배치가 어긋나도 로그는 남음. 헬퍼 프롤로그는
//   전부 rsp/rbp-relative(재배치 안전). 재진입 가드 + catch_unwind + VirtualQuery safe-read.
//
// 빌드: powershell -File C:\tfm2mods\build_inj.ps1 -Src C:\tfm2mods\tfm2_panic_probe\src\tfm2_panic_probe.rs -ModId tfm2_panic_probe
// =====================================================================================

use mod_api::*;
use std::path::PathBuf;
use std::fs;
use std::io::Write;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

const MOD_ID: &str = "tfm2_panic_probe";

// ── 패닉 헬퍼(★0.5.0 buildid 24102827, image_base 0x140000000, RVA=abs-base) ──
// unwrap-None 헬퍼 FUN_142bee240: 프롤로그 0.4.14와 동일(`push rbp; sub rsp,0x20; lea rbp,[rsp+0x20]; mov r8,rcx`), rcx=&Location(saved[0]).
// 모든 unwrap/expect/Option-None/panic!(&str) 수렴 = sim 스톨(unwrap-None) 포착에 이것 하나면 충분.
const RVA_UNWRAP: usize = 0x2bee240;
const UNWRAP_LEN: usize = 13;
const UNWRAP_EXPECT: [u8; 13] = [0x55,0x48,0x83,0xec,0x20,0x48,0x8d,0x6c,0x24,0x20,0x49,0x89,0xc8];
// panic_bounds_check: `push rbp; sub rsp,0x50; lea rbp,[rsp+0x50]; lea rax,[rbp-8]` (rcx=index,rdx=len,r8=&Location)
const RVA_BOUNDS: usize = 0x2957c83;
const BOUNDS_LEN: usize = 14;
const BOUNDS_EXPECT: [u8; 14] = [0x55,0x48,0x83,0xec,0x50,0x48,0x8d,0x6c,0x24,0x50,0x48,0x8d,0x45,0xf8];
// panic_fmt(만능 깔때기, 모든 패닉 종류 수렴): `push rbp; sub rsp,0x50; lea rbp,[rsp+0x50]; mov [rbp-0x10],rcx` (r8=&Location)
const RVA_SLICE: usize = 0x2957e60;
const SLICE_LEN: usize = 14;
const SLICE_EXPECT: [u8; 14] = [0x55,0x48,0x83,0xec,0x50,0x48,0x8d,0x6c,0x24,0x50,0x48,0x89,0x4d,0xf0];

// RushTime 매틱 핸들러 범위(caller_rva 분류용). FUN_141cd08e0 ~ 대략.
const RUSHTIME_LO: usize = 0x1cd08e0;
const RUSHTIME_HI: usize = 0x1cd7400;
const RUSHTIME_UNWRAP_RET: usize = 0x1cd6f26;   // 0x1cd6f1a의 call 다음(ud2) = 확정 시그니처

static IN_HOOK: AtomicBool = AtomicBool::new(false);
static PANIC_N: AtomicU64 = AtomicU64::new(0);
const PANIC_LOG_LIMIT: u64 = 200;

// ── Win32 ──
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
#[inline] unsafe fn rd_u64(a: usize) -> Option<u64> { if readable(a, 8) { Some(core::ptr::read_unaligned(a as *const u64)) } else { None } }
#[inline] unsafe fn rd_u32(a: usize) -> Option<u32> { if readable(a, 4) { Some(core::ptr::read_unaligned(a as *const u32)) } else { None } }
#[inline] unsafe fn rd_u8(a: usize) -> Option<u8> { if readable(a, 1) { Some(core::ptr::read_unaligned(a as *const u8)) } else { None } }
#[inline] fn ptr_ok(a: usize) -> bool { a >= 0x10000 && a < (1usize << 48) }

// ── 로그 ──
fn now_ms() -> u64 { SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0) }
fn dir() -> Option<PathBuf> { unsafe {
    let mut h: HMODULE = 0;
    if GetModuleHandleExW(0x4|0x2, dir as *const () as *const u16, &mut h) == 0 || h == 0 { return None; }
    let mut b = [0u16; 4096];
    let n = GetModuleFileNameW(h, b.as_mut_ptr(), b.len() as DWORD);
    if n == 0 { return None; }
    let mut p = PathBuf::from(String::from_utf16_lossy(&b[..n as usize])); p.pop(); Some(p)
}}
fn pth(name: &str) -> Option<PathBuf> { dir().map(|mut p| { p.push(name); p }) }
fn append_log(s: &str) {
    if let Some(p) = pth("tfm2_panic_probe.txt") {
        if let Ok(mut f) = fs::OpenOptions::new().create(true).append(true).open(&p) { let _ = write!(f, "{}", s); let _ = f.flush(); }
    }
}

// &Location{ file:&str(ptr@0,len@8), line:u32@0x10, col:u32@0x14 } 후보 디코드
unsafe fn decode_location(loc: usize) -> Option<(String, u32, u32)> {
    if !ptr_ok(loc) { return None; }
    let fptr = rd_u64(loc)? as usize;
    let flen = rd_u64(loc + 8)? as usize;
    let line = rd_u32(loc + 0x10)?;
    let col  = rd_u32(loc + 0x14).unwrap_or(0);
    if !ptr_ok(fptr) || flen == 0 || flen > 512 || line == 0 || line > 10_000_000 { return None; }
    let mut b = Vec::with_capacity(flen);
    for i in 0..flen { match rd_u8(fptr + i) { Some(c) if c != 0 => b.push(c), _ => break } }
    // 소스경로처럼 보이는지(ascii 다수) 최소 검증
    if b.len() < 3 { return None; }
    let ascii = b.iter().filter(|&&c| c >= 0x20 && c < 0x7f).count();
    if ascii * 10 < b.len() * 9 { return None; }
    Some((String::from_utf8_lossy(&b).into_owned(), line, col))
}

// ── 공통 패닉 cap ──
fn panic_cap_common(kind: &str, saved: *mut u64, entry_rsp: usize) {
    if IN_HOOK.swap(true, Ordering::Relaxed) { return; }   // 재진입 차단
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe {
        let n = PANIC_N.fetch_add(1, Ordering::Relaxed);
        if n >= PANIC_LOG_LIMIT { return; }
        let base = exe_base();
        // (1) 즉시 호출자 return address
        let caller_abs = if entry_rsp != 0 { rd_u64(entry_rsp).unwrap_or(0) as usize } else { 0 };
        let caller_rva = caller_abs.wrapping_sub(base);
        // (2) &Location: saved[0..4] 중 유효 디코드
        let mut locstr = String::from("(none)");
        if !saved.is_null() {
            for i in 0..4usize {
                let r = *saved.add(i) as usize;
                if let Some((f, line, col)) = decode_location(r) {
                    locstr = format!("{}:{}:{}", f, line, col);
                    break;
                }
            }
        }
        // (3) 미니 백트레이스: entry_rsp 위 스택에서 in-exe 복귀주소 최대 12개
        let mut bt = String::new();
        let mut o = 0usize; let mut cnt = 0;
        while o < 0x400 && cnt < 12 {
            if let Some(v) = rd_u64(entry_rsp + o) {
                let v = v as usize;
                if base != 0 && v >= base && v < base + 0x4000000 {
                    bt.push_str(&format!(" 0x{:x}", v - base));
                    cnt += 1;
                }
            }
            o += 8;
        }
        let hit = if caller_rva == RUSHTIME_UNWRAP_RET { "  <<< RushTime unwrap 확정(0x1cd6f26)" }
                  else if caller_rva >= RUSHTIME_LO && caller_rva < RUSHTIME_HI { "  <<< RushTime 핸들러 범위" }
                  else { "" };
        append_log(&format!(
            "[PANIC #{} {}ms] kind={} caller_rva=0x{:x} loc={}{}\n    bt(rva):{}\n",
            n, now_ms(), kind, caller_rva, locstr, hit, bt));
    }));
    IN_HOOK.store(false, Ordering::Relaxed);
}

#[no_mangle] pub extern "C" fn cap_unwrap(saved: *mut u64, entry_rsp: usize) { panic_cap_common("unwrap-None", saved, entry_rsp); }
#[no_mangle] pub extern "C" fn cap_bounds(saved: *mut u64, entry_rsp: usize) { panic_cap_common("bounds-check", saved, entry_rsp); }
#[no_mangle] pub extern "C" fn cap_slice(saved: *mut u64, entry_rsp: usize) { panic_cap_common("panic_fmt", saved, entry_rsp); }

// ── 진입 detour: mov r10,rsp → push rcx rdx r8 r9 r10 r11 → cap(rcx=&saved, rdx=entry_rsp) → 원본 프롤로그 → jmp fn+len ──
unsafe fn install_entry_detour(rva: usize, orig_len: usize, expect: &[u8], cap: usize) -> Result<(), &'static str> {
    let base = exe_base(); if base == 0 { return Err("module 0"); }
    let fn_addr = base + rva;
    if !readable(fn_addr, orig_len + 4) { return Err("unreadable"); }
    let mut cur = vec![0u8; orig_len];
    core::ptr::copy_nonoverlapping(fn_addr as *const u8, cur.as_mut_ptr(), orig_len);
    if cur.as_slice() != expect { return Err("byte mismatch (RVA stale?)"); }
    const MEM_CR: u32 = 0x1000|0x2000; const RWX: u32 = 0x40;
    let stub = VirtualAlloc(0, 256, MEM_CR, RWX); if stub == 0 { return Err("VirtualAlloc"); }
    let ret_addr = fn_addr + orig_len;
    let mut s: Vec<u8> = Vec::new();
    s.extend_from_slice(&[0x49,0x89,0xe2]);                                              // mov r10, rsp (entry rsp)
    s.extend_from_slice(&[0x51,0x52,0x41,0x50,0x41,0x51,0x41,0x52,0x41,0x53]);           // push rcx rdx r8 r9 r10 r11
    s.extend_from_slice(&[0x48,0x89,0xe1]);                                              // mov rcx, rsp  (&saved)
    s.extend_from_slice(&[0x4c,0x89,0xd2]);                                              // mov rdx, r10  (entry_rsp)
    s.extend_from_slice(&[0x48,0x83,0xec,0x28]);                                         // sub rsp,0x28
    s.extend_from_slice(&[0x48,0xb8]); s.extend_from_slice(&cap.to_le_bytes());          // movabs rax, cap
    s.extend_from_slice(&[0xff,0xd0]);                                                   // call rax
    s.extend_from_slice(&[0x48,0x83,0xc4,0x28]);                                         // add rsp,0x28
    s.extend_from_slice(&[0x41,0x5b,0x41,0x5a,0x41,0x59,0x41,0x58,0x5a,0x59]);           // pop r11 r10 r9 r8 rdx rcx
    let mut orig = vec![0u8; orig_len];
    core::ptr::copy_nonoverlapping(fn_addr as *const u8, orig.as_mut_ptr(), orig_len);
    s.extend_from_slice(&orig);                                                          // 원본 프롤로그(재배치)
    s.extend_from_slice(&[0x48,0xb8]); s.extend_from_slice(&ret_addr.to_le_bytes());     // movabs rax, fn+len
    s.extend_from_slice(&[0xff,0xe0]);                                                   // jmp rax
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    let mut patch = vec![0x90u8; orig_len];
    patch[0]=0x48; patch[1]=0xb8; patch[2..10].copy_from_slice(&stub.to_le_bytes()); patch[10]=0xff; patch[11]=0xe0;
    let mut old: u32 = 0;
    if VirtualProtect(fn_addr, orig_len, RWX, &mut old) == 0 { return Err("VirtualProtect"); }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), fn_addr as *mut u8, orig_len);
    VirtualProtect(fn_addr, orig_len, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), fn_addr, orig_len);
    Ok(())
}

fn init(_ctx: &GameCtx) -> ModRegistration {
    if let Some(p) = pth("tfm2_panic_probe.txt") {
        let _ = fs::write(&p, format!("[{}ms] === tfm2_panic_probe INIT (패닉 지점 확인) ===\n", now_ms()));
    }
    unsafe {
        // ★0.5.0: unwrap-None 헬퍼(0x2bee240) 하나만 훅 = 모든 unwrap/Option-None panic 수렴 → sim 스톨 포착.
        //   (0.4.14 bounds/panic_fmt 주소는 0.5.0서 무효라 제거. Location=saved[0]=rcx.)
        let hooks: [(&str, usize, usize, &[u8], usize); 1] = [
            ("unwrap-None", RVA_UNWRAP, UNWRAP_LEN, &UNWRAP_EXPECT, cap_unwrap as *const () as usize),
        ];
        for (name, rva, len, expect, cap) in hooks {
            match install_entry_detour(rva, len, expect, cap) {
                Ok(())  => append_log(&format!("[hook] {} @0x{:x} ({}B) OK\n", name, rva, len)),
                Err(e)  => append_log(&format!("[hook] {} @0x{:x} 실패: {}\n", name, rva, e)),
            }
        }
    }
    append_log("[ready] 백그라운드 경기(일정넘김)로 패닉 재현하면 이 파일에 [PANIC ...] 기록됨\n");
    ModRegistration::new(MOD_ID)
}
declare_mod!(init);
