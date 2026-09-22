//! ai_swap — 서버 워커의 AI 스왑 배정 `compute_rule_swap_order(sret Vec<u64>, scorer, ally:&[u64 모델 인덱스], enemy:&[u64], rule, is_explore)` 진입 detour(0.6.0 RVA 0x27d8660 · push×8).
//! ⚠ally/enemy 는 0.5.8 IR 앵커와 달리 **챔피언 모델 인덱스 슬라이스**(콜러가 champion_model_indices 로 변환) → 이름은 hookA 와 같은 모델 이름표(`legacy_assign::model_names`, champion_briefs 순서)로 역매핑.
//! 원본을 그대로 부른 뒤 **반환 order(포지션 p → 픽 인덱스)를 포지션 제한 규칙에 맞는 최적 순열로 고쳐 쓴다**(`assign::best_order`: 맞게 앉는 인원 최대 · 원본과 일치 최대).
//! 클래식 v0.5.0 의 `cprod_swap_order`(DecisionRecord tag 7 rewrite) 를 0.6.0 stable 로 이식한 것 — 레코드 레이아웃 대신 함수 반환값을 후처리하므로 구조체 의존이 없다.
//! 근거 RE: `mods_report/tfm2_champ_pos_lock/RE/2026-09-18_0.6.0-compute_rule_swap_order-AI스왑배정.md`.
//! 안전: 워커 스레드에서 돈다 — detour 본문은 catch_unwind · 읽기는 전부 `readable`(VirtualQuery) 경유 · Vec 길이/순열 검증 실패 = 무개입 · 설정 `swap_force=0` 이면 무개입.
use crate::assign;
use crate::config;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

/// 0.6.0 — RE 09-18. (값은 RE 결과로 확정)
pub const RVA_COMPUTE_RULE_SWAP_ORDER: usize = 0x2a789d0;
const PROL: [u8; 12] = [0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x53];

#[link(name = "kernel32")]
extern "system" {
    fn GetModuleHandleW(n: *const u16) -> usize;
    fn VirtualAlloc(addr: usize, size: usize, ty: u32, prot: u32) -> usize;
    fn VirtualProtect(addr: usize, size: usize, prot: u32, old: *mut u32) -> i32;
    fn FlushInstructionCache(h: usize, addr: usize, size: usize) -> i32;
    fn GetCurrentProcess() -> usize;
    fn VirtualQuery(addr: *const core::ffi::c_void, buf: *mut MemBasicInfo, len: usize) -> usize;
}
#[repr(C)] #[derive(Default)]
struct MemBasicInfo { base: usize, alloc_base: usize, alloc_protect: u32, _p0: u32, region_size: usize, state: u32, protect: u32, typ: u32, _p1: u32 }
unsafe fn readable(addr: usize, len: usize) -> bool {
    if addr < 0x10000 || addr >= (1usize << 48) || len == 0 { return false; }
    let mut mbi = MemBasicInfo::default();
    if VirtualQuery(addr as *const _, &mut mbi, core::mem::size_of::<MemBasicInfo>()) == 0 { return false; }
    const COMMIT: u32 = 0x1000; const RD: u32 = 0x02 | 0x04 | 0x20 | 0x40; const GUARD: u32 = 0x01 | 0x100;
    if mbi.state != COMMIT || mbi.protect & GUARD != 0 || mbi.protect & RD == 0 { return false; }
    addr.wrapping_add(len) <= mbi.base.wrapping_add(mbi.region_size)
}

static TRAMP: AtomicUsize = AtomicUsize::new(0);
pub static INSTALL_STATE: AtomicUsize = AtomicUsize::new(0); // 0 미설치 1 설치 2 영구실패
pub static CNT_FIRE: AtomicU64 = AtomicU64::new(0);
pub static CNT_REWRITE: AtomicU64 = AtomicU64::new(0);
pub static CNT_SKIP: AtomicU64 = AtomicU64::new(0);
/// 마지막 개입 기록(로그용, 메인 스레드가 폴링): (매치 수·순열 전/후)는 lineups 로그에 남긴다.
pub static LAST_LOG: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None);

/// &[u64] 모델 인덱스 슬라이스 → 소문자 id 목록(모델 이름표 역매핑). 형태 이상·이름표 없음·인덱스 범위 밖(usize::MAX = 미등록) 이면 None.
unsafe fn read_names(ptr: usize, len: usize) -> Option<Vec<String>> {
    if len == 0 || len > 8 || !readable(ptr, len * 8) { return None; }
    let names = crate::legacy_assign::model_names()?;
    let mut v = Vec::with_capacity(len);
    for i in 0..len {
        let idx = core::ptr::read_unaligned((ptr + i * 8) as *const u64) as usize;
        v.push(names.get(idx)?.clone());
    }
    Some(v)
}

/// 원본 호출 후 order 재배치. 반환값 = 원본 rax(sret 포인터).
extern "C" fn detour(sret: usize, scorer: usize, ally_ptr: usize, ally_len: usize, enemy_ptr: usize, enemy_len: usize, rule: usize, is_explore: usize) -> usize {
    let tramp = TRAMP.load(Ordering::Relaxed);
    if tramp == 0 { return 0; }
    let orig: extern "C" fn(usize, usize, usize, usize, usize, usize, usize, usize) -> usize = unsafe { core::mem::transmute(tramp) };
    let ret = orig(sret, scorer, ally_ptr, ally_len, enemy_ptr, enemy_len, rule, is_explore);
    CNT_FIRE.fetch_add(1, Ordering::Relaxed);
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe {
        let cfg = config::get();
        if !cfg.enabled || cfg.swap_force == 0 || !config::any_restricted() { return; }
        if !readable(sret, 24) { CNT_SKIP.fetch_add(1, Ordering::Relaxed); return; }
        let vptr = core::ptr::read_unaligned((sret + 8) as *const usize);
        let vlen = core::ptr::read_unaligned((sret + 16) as *const usize);
        if vlen < 2 || vlen > 8 || vlen != ally_len || !readable(vptr, vlen * 8) { CNT_SKIP.fetch_add(1, Ordering::Relaxed); return; }
        let cur: Vec<u64> = (0..vlen).map(|i| core::ptr::read_unaligned((vptr + i * 8) as *const u64)).collect();
        // 순열 검증(오식별 가드)
        let mut seen = vec![false; vlen];
        for &x in &cur { if x as usize >= vlen || seen[x as usize] { CNT_SKIP.fetch_add(1, Ordering::Relaxed); return; } seen[x as usize] = true; }
        let Some(names) = read_names(ally_ptr, ally_len) else { CNT_SKIP.fetch_add(1, Ordering::Relaxed); return; };
        let masks: Vec<u8> = names.iter().map(|n| crate::mask_of(n)).collect();
        let (want, best_n) = assign::best_order(&masks, &cur);
        let cur_n = assign::order_matched(&masks, &cur);
        if best_n <= cur_n || want == cur { if cfg.debug { if let Ok(mut g) = LAST_LOG.try_lock() { *g = Some(format!("aiswap(무개입): {:?} {:?} 맞춤 {}/{} (최적 {})", names, cur, cur_n, vlen, best_n)); } } return; }
        // 쓰기 가능한지(VirtualQuery RD 만 봤으므로 WRITE 플래그도 확인)
        let mut mbi = MemBasicInfo::default();
        if VirtualQuery(vptr as *const _, &mut mbi, core::mem::size_of::<MemBasicInfo>()) == 0 || mbi.protect & (0x04 | 0x40) == 0 { CNT_SKIP.fetch_add(1, Ordering::Relaxed); return; }
        for (i, &x) in want.iter().enumerate() { core::ptr::write_unaligned((vptr + i * 8) as *mut u64, x); }
        CNT_REWRITE.fetch_add(1, Ordering::Relaxed);
        if let Ok(mut g) = LAST_LOG.try_lock() { *g = Some(format!("aiswap: {:?} {:?} -> {:?} (맞춤 {}→{}/{} rule={} explore={})", names, cur, want, cur_n, best_n, vlen, rule & 0xff, is_explore & 1)); }
    }));
    ret
}

unsafe fn install(fn_addr: usize) -> Result<bool, &'static str> {
    if !readable(fn_addr, 12) { return Err("entry unreadable"); }
    let mut cur = [0u8; 12];
    core::ptr::copy_nonoverlapping(fn_addr as *const u8, cur.as_mut_ptr(), 12);
    let chained = cur[0] == 0x48 && cur[1] == 0xb8 && cur[10] == 0xff && cur[11] == 0xe0;
    if !chained && cur != PROL { return Err("prologue mismatch"); }
    let stub = VirtualAlloc(0, 64, 0x1000 | 0x2000, 0x40);
    if stub == 0 { return Err("VirtualAlloc"); }
    let mut s: Vec<u8> = Vec::with_capacity(32);
    s.extend_from_slice(&cur);
    if !chained { s.extend_from_slice(&[0xff, 0x25, 0, 0, 0, 0]); s.extend_from_slice(&(fn_addr + 12).to_le_bytes()); }
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    TRAMP.store(stub, Ordering::SeqCst);
    let repl = detour as extern "C" fn(usize, usize, usize, usize, usize, usize, usize, usize) -> usize as *const () as usize;
    let mut patch = [0u8; 12];
    patch[0] = 0x48; patch[1] = 0xb8; patch[2..10].copy_from_slice(&repl.to_le_bytes()); patch[10] = 0xff; patch[11] = 0xe0;
    let mut old: u32 = 0;
    if VirtualProtect(fn_addr, 12, 0x40, &mut old) == 0 { return Err("VirtualProtect"); }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), fn_addr as *mut u8, 12);
    VirtualProtect(fn_addr, 12, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), fn_addr, 12);
    Ok(chained)
}

/// 1회 설치(swap_force=0 이면 설치 보류 아님·영구 실패 아님 — 설정은 detour 안에서 매번 본다).
pub fn install_once() -> Option<String> {
    if INSTALL_STATE.load(Ordering::Relaxed) != 0 { return None; }
    if RVA_COMPUTE_RULE_SWAP_ORDER == 0 { INSTALL_STATE.store(2, Ordering::Relaxed); return Some("ai_swap: RVA 미확정 — 미설치".into()); }
    let base = unsafe { GetModuleHandleW(core::ptr::null()) };
    if base == 0 { return None; }
    let fn_addr = base + RVA_COMPUTE_RULE_SWAP_ORDER;
    Some(match unsafe { install(fn_addr) } {
        Ok(chained) => { INSTALL_STATE.store(1, Ordering::Relaxed); format!("ai_swap(compute_rule_swap_order) 훅 설치 OK @{:#x} chained={}", fn_addr, chained) }
        Err(e) => { INSTALL_STATE.store(2, Ordering::Relaxed); format!("ai_swap 훅 설치 실패: {e} (RVA stale?) — AI 스왑 배정 강제 없이 진행") }
    })
}

/// 메인 스레드 폴링: 마지막 개입 로그를 꺼내 lineups 로그로.
pub fn drain_log() -> Option<String> { LAST_LOG.try_lock().ok().and_then(|mut g| g.take()) }
