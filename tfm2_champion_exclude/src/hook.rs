//! hook — 패치데이 신챔프 추가 후보 Vec<String> 생성 함수 진입 트램폴린 + 사후 필터 (클래식 0.5.x 그대로, 0.6.0 재핀).
//! 원본 계약(0.5.5 RE·0.6.0 동일): rcx=out(*mut Vec<String>), rdx=iter_ctx, 반환 rax=out.
//! Vec{cap@0, ptr@8, len@0x10} / 요소 String{cap@0, ptr@8, len@0x10} stride 0x18.
//! 제외 목록은 `crate::effective_exclusion()`(post_update 가 세이브에서 캐시) — detour 엔 SDK 컨텍스트가 없다.
use std::collections::HashSet;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::atomic::{AtomicUsize, Ordering};

/// 신챔프 추가 후보 Vec<String> 생성 함수 — ★0.6.0 재핀(MIG/manifest/tfm2_champion_exclude.json HOOK_RVA, FN_UNIQUE size 331 동일; 0.5.6 0x1894610).
pub const HOOK_RVA: usize = 0x1b5c490;
/// 프롤로그: push rbp; push r15; push r14; push r12; push rsi; push rdi; push rbx; sub rsp,0xA0 (17B, rip-rel 없음)
const HOOK_ORIG: [u8; 17] = [0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x54, 0x56, 0x57, 0x53, 0x48, 0x81, 0xEC, 0xA0, 0x00, 0x00, 0x00];
const ORIG_LEN: usize = 17;
const FREE_REMOVED: bool = false; // 제거한 String 힙 버퍼는 의도적 leak(패치데이당 몇 개·교차 힙 free 위험 회피)
const MAX_CANDIDATES: usize = 4096;
const MAX_NAME_LEN: usize = 256;

type BOOL = i32;
extern "system" {
    fn GetModuleHandleW(name: *const u16) -> usize;
    fn VirtualProtect(addr: usize, size: usize, new_protect: u32, old: *mut u32) -> BOOL;
    fn VirtualAlloc(addr: usize, size: usize, alloc_type: u32, protect: u32) -> usize;
    fn VirtualQuery(addr: *const core::ffi::c_void, buf: *mut MemBasicInfo, len: usize) -> usize;
    fn FlushInstructionCache(proc: usize, addr: usize, sz: usize) -> BOOL;
    fn GetCurrentProcess() -> usize;
    fn GetProcessHeap() -> usize;
    fn HeapFree(heap: usize, flags: u32, ptr: usize) -> BOOL;
}
#[repr(C)]
#[derive(Default)]
pub struct MemBasicInfo { base: usize, alloc_base: usize, alloc_protect: u32, _p0: u32, region_size: usize, state: u32, protect: u32, typ: u32, _p1: u32 }

#[inline]
pub fn ptr_sane(addr: usize) -> bool { addr >= 0x10000 && addr < (1usize << 48) }

pub unsafe fn readable(addr: usize, len: usize) -> bool {
    if !ptr_sane(addr) || len == 0 { return false; }
    let mut mbi = MemBasicInfo::default();
    if VirtualQuery(addr as *const _, &mut mbi, core::mem::size_of::<MemBasicInfo>()) == 0 { return false; }
    const COMMIT: u32 = 0x1000; const RD: u32 = 0x02 | 0x04 | 0x20 | 0x40; const GUARD: u32 = 0x01 | 0x100;
    if mbi.state != COMMIT || mbi.protect & GUARD != 0 || mbi.protect & RD == 0 { return false; }
    addr.wrapping_add(len) <= mbi.base + mbi.region_size
}

type HookFn = extern "C" fn(usize, usize, usize, usize) -> usize;
static TRAMPOLINE: AtomicUsize = AtomicUsize::new(0);
pub static FIRE_COUNT: AtomicUsize = AtomicUsize::new(0);
pub static MOD_BASE: AtomicUsize = AtomicUsize::new(0);

extern "C" fn detour_candidates(rcx: usize, rdx: usize, r8: usize, r9: usize) -> usize {
    let tramp = TRAMPOLINE.load(Ordering::Acquire);
    if tramp == 0 { return rcx; }
    let orig: HookFn = unsafe { core::mem::transmute(tramp) };
    let ret = orig(rcx, rdx, r8, r9);
    let _ = catch_unwind(AssertUnwindSafe(|| filter_candidates(ret)));
    ret
}

/// Vec<String> 을 읽어 소문자 id 목록으로. (out = &Vec)
pub unsafe fn read_string_vec(out: usize, max: usize) -> Option<Vec<String>> {
    if !readable(out, 0x18) { return None; }
    let vec_ptr = *((out + 8) as *const usize);
    let len = *((out + 0x10) as *const usize);
    if len == 0 { return Some(Vec::new()); }
    if len > max || !ptr_sane(vec_ptr) || !readable(vec_ptr, len * 0x18) { return None; }
    let mut v = Vec::with_capacity(len);
    for i in 0..len {
        let elem = vec_ptr + i * 0x18;
        let sptr = *((elem + 8) as *const usize);
        let slen = *((elem + 0x10) as *const usize);
        if slen == 0 || slen > MAX_NAME_LEN || !ptr_sane(sptr) || !readable(sptr, slen) { return None; }
        let bytes = core::slice::from_raw_parts(sptr as *const u8, slen);
        v.push(std::str::from_utf8(bytes).ok()?.to_string());
    }
    Some(v)
}

fn filter_candidates(out: usize) {
    let n = FIRE_COUNT.fetch_add(1, Ordering::Relaxed) + 1;
    if !ptr_sane(out) || unsafe { !readable(out, 0x18) } { crate::log(&format!("fire#{}: bad out ptr (0x{:x}) - filter skipped", n, out)); return; }
    let vec_ptr = unsafe { *((out + 8) as *const usize) };
    let mut len = unsafe { *((out + 0x10) as *const usize) };
    if len == 0 { crate::log(&format!("fire#{}: 0 candidates (champion_add off or all released) - nothing to do", n)); return; }
    if len > MAX_CANDIDATES || !ptr_sane(vec_ptr) || unsafe { !readable(vec_ptr, len * 0x18) } {
        crate::log(&format!("fire#{}: bad candidate vec (ptr=0x{:x} len={}) - filter skipped (misread guard)", n, vec_ptr, len)); return;
    }
    let read_name = |idx: usize| -> Option<String> {
        let elem = vec_ptr + idx * 0x18;
        let sptr = unsafe { *((elem + 8) as *const usize) };
        let slen = unsafe { *((elem + 0x10) as *const usize) };
        if slen == 0 || slen > MAX_NAME_LEN || !ptr_sane(sptr) || unsafe { !readable(sptr, slen) } { return None; }
        let bytes = unsafe { core::slice::from_raw_parts(sptr as *const u8, slen) };
        std::str::from_utf8(bytes).ok().map(|s| s.to_string())
    };
    let names: Vec<String> = (0..len).map(|i| read_name(i).unwrap_or_else(|| "<unreadable>".into())).collect();
    let (exclude, block_all, src) = crate::effective_exclusion();
    crate::log(&format!("fire#{}: {} candidates = [{}] / exclude list {} (source={}){}", n, len, names.join(", "), exclude.len(), src, if block_all { " + block-all(*)" } else { "" }));
    save_seen(&names);
    if exclude.is_empty() && !block_all { return; }
    let mut removed: Vec<String> = Vec::new();
    let mut i = 0usize;
    while i < len {
        let name = read_name(i);
        let hit = match (&name, block_all) { (_, true) => true, (Some(nm), false) => exclude.iter().any(|e| e == &nm.to_ascii_lowercase()), (None, false) => false };
        if hit {
            unsafe {
                let elem = vec_ptr + i * 0x18;
                if FREE_REMOVED { let scap = *(elem as *const usize); let sptr = *((elem + 8) as *const usize); if scap > 0 && ptr_sane(sptr) { HeapFree(GetProcessHeap(), 0, sptr); } }
                let last = vec_ptr + (len - 1) * 0x18;
                if last != elem { core::ptr::copy_nonoverlapping(last as *const u8, elem as *mut u8, 0x18); }
                len -= 1;
                *((out + 0x10) as *mut usize) = len;
            }
            removed.push(name.unwrap_or_else(|| "<unreadable>".into()));
        } else { i += 1; }
    }
    if removed.is_empty() { crate::log(&format!("fire#{}: no match - {} candidates kept", n, len)); }
    else { crate::log(&format!("fire#{}: removed {} = [{}] -> {} candidates left", n, removed.len(), removed.join(", "), len)); }
}

/// 패치데이에 관측한 실후보를 seen 파일에 병합(소문자·정렬·멱등) — UI 목록 보강용.
pub fn save_seen(names: &[String]) {
    let Some(d) = crate::mod_dir() else { return };
    let p = format!("{}\\champion_exclude_seen.txt", d);
    let mut set = load_seen();
    let before = set.len();
    for n in names { if n != "<unreadable>" { set.insert(n.to_ascii_lowercase()); } }
    if set.len() == before && std::path::Path::new(&p).exists() { return; }
    let mut v: Vec<String> = set.into_iter().collect();
    v.sort();
    let body = format!("# candidates observed on patch days (auto-merged; lowercase ids)\n{}\n", v.join("\n"));
    let _ = std::fs::write(&p, body);
}
pub fn load_seen() -> HashSet<String> {
    let mut set = HashSet::new();
    let Some(d) = crate::mod_dir() else { return set };
    if let Ok(t) = std::fs::read_to_string(format!("{}\\champion_exclude_seen.txt", d)) {
        for line in t.lines() { let l = line.split('#').next().unwrap_or("").trim(); if !l.is_empty() { set.insert(l.to_ascii_lowercase()); } }
    }
    set
}

fn jmp_abs(target: usize) -> [u8; 12] {
    let mut b = [0u8; 12];
    b[0] = 0x48; b[1] = 0xB8; b[2..10].copy_from_slice(&target.to_le_bytes()); b[10] = 0xFF; b[11] = 0xE0;
    b
}

/// 진입 트램폴린 설치(프롤로그 17B 실측 검증·외부 훅이면 체인). 1회.
pub unsafe fn install_hook() -> Result<String, String> {
    let base = GetModuleHandleW(core::ptr::null());
    if base == 0 { return Err("GetModuleHandleW(null)=0".into()); }
    MOD_BASE.store(base, Ordering::Relaxed);
    let addr = base + HOOK_RVA;
    if !readable(addr, ORIG_LEN) { return Err(format!("hook site unreadable @abs=0x{:x} (base=0x{:x} rva=0x{:x})", addr, base, HOOK_RVA)); }
    let mut cur = [0u8; ORIG_LEN];
    core::ptr::copy_nonoverlapping(addr as *const u8, cur.as_mut_ptr(), ORIG_LEN);
    let tramp = VirtualAlloc(0, 0x1000, 0x1000 | 0x2000, 0x40);
    if tramp == 0 { return Err("VirtualAlloc trampoline failed".into()); }
    let mode: &str;
    if cur == HOOK_ORIG {
        core::ptr::copy_nonoverlapping(cur.as_ptr(), tramp as *mut u8, ORIG_LEN);
        let jmp = jmp_abs(addr + ORIG_LEN);
        core::ptr::copy_nonoverlapping(jmp.as_ptr(), (tramp + ORIG_LEN) as *mut u8, 12);
        mode = "clean prologue -> trampoline";
    } else if cur[0] == 0x48 && cur[1] == 0xB8 && cur[10] == 0xFF && cur[11] == 0xE0 {
        core::ptr::copy_nonoverlapping(cur.as_ptr(), tramp as *mut u8, 12);
        let ext = usize::from_le_bytes(cur[2..10].try_into().unwrap());
        crate::log(&format!("chain: external hook target=0x{:x}", ext));
        mode = "external hook detected -> chained";
    } else {
        return Err(format!("prologue mismatch -> install SKIPPED @abs=0x{:x} found={:02x?} expected={:02x?} (check game patch version)", addr, cur, HOOK_ORIG));
    }
    TRAMPOLINE.store(tramp, Ordering::Release);
    let mut patch = [0x90u8; ORIG_LEN];
    patch[..12].copy_from_slice(&jmp_abs(detour_candidates as usize));
    let mut old: u32 = 0;
    if VirtualProtect(addr, ORIG_LEN, 0x40, &mut old) == 0 { return Err("VirtualProtect failed".into()); }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), addr as *mut u8, ORIG_LEN);
    let mut old2: u32 = 0;
    VirtualProtect(addr, ORIG_LEN, old, &mut old2);
    FlushInstructionCache(GetCurrentProcess(), addr, ORIG_LEN);
    let mut landed = [0u8; ORIG_LEN];
    core::ptr::copy_nonoverlapping(addr as *const u8, landed.as_mut_ptr(), ORIG_LEN);
    if landed == patch { Ok(format!("installed+VERIFIED @abs=0x{:x} (rva=0x{:x}) mode={} tramp=0x{:x}", addr, HOOK_RVA, mode, tramp)) }
    else { Err(format!("write not applied @abs=0x{:x} landed={:02x?}", addr, landed)) }
}
