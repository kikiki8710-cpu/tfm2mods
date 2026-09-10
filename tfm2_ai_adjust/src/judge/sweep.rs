//! sweep.rs — **자동 생성**(`MIG/gensweep.py`). 판정 계층이 RVA 를 확정해 둔 함수를 전수로
//! 게임 vs 내 링크 사본 대조한다. cfg `fn_sweep` 비트마스크로 하나씩/묶어서 켠다.
//! ⚠ABI 가 틀리면 게임이 즉사한다 — 크래시나면 그 비트를 빼고 그 함수는 RVA/시그니처를 다시 본다.
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use super::tune;

extern "Rust" {
    #[link_name = "_RNvNtCshdEBA0ozCnw_7game_ai10buff_value14v55_mark_value"]
    fn my_v55_mark(a0: i64, a1: *const u8, a2: *const u8, a3: *const u8, a4: *const u8, a5: *const u8, a6: *const u8, a7: i64, a8: *const u8) -> i64;
    #[link_name = "_RNvNtCshdEBA0ozCnw_7game_ai10buff_value14v55_seal_value"]
    fn my_v55_seal(a0: i64, a1: *const u8, a2: *const u8, a3: *const u8, a4: *const u8, a5: i64) -> i64;
    #[link_name = "_RNvNtCshdEBA0ozCnw_7game_ai10buff_value18v55_banish_penalty"]
    fn my_v55_banish(a0: i64, a1: *const u8, a2: *const u8, a3: *const u8, a4: *const u8, a5: *const u8, a6: i64) -> i64;
    #[link_name = "_RNvNtCshdEBA0ozCnw_7game_ai16minion_wave_risk32enemy_minion_wave_risk_damage_at"]
    fn my_mw_risk(a0: i64, a1: *const u8, a2: *const u8, a3: i64, a4: i64, a5: i64) -> i64;
    #[link_name = "_RNvNtCshdEBA0ozCnw_7game_ai10buff_value23v54_aoe_ally_heal_value"]
    fn my_v54_aoe(a0: i64, a1: *const u8, a2: *const u8, a3: *const u8, a4: *const u8, a5: *const u8, a6: i64, a7: *const u8) -> i64;
    #[link_name = "_RNvNtCshdEBA0ozCnw_7game_ai10buff_value24v55_spirit_trigger_value"]
    fn my_v55_spirit(a0: *const u8, a1: *const u8, a2: *const u8, a3: *const u8, a4: *const u8, a5: i64) -> i64;
    #[link_name = "_RNvNtCshdEBA0ozCnw_7game_ai10buff_value21area_buff_multi_scale"]
    fn my_abms(a0: i64, a1: *const u8, a2: *const u8, a3: *const u8, a4: *const u8, a5: i64) -> i64;
    #[link_name = "_RNvNtCshdEBA0ozCnw_7game_ai10buff_value23noncombat_steroid_value"]
    fn my_ncsv(a0: *const u8, a1: *const u8, a2: *const u8, a3: *const u8) -> i64;
    #[link_name = "_RNvNtCshdEBA0ozCnw_7game_ai10buff_value16defensive_crisis"]
    fn my_defc(a0: i64, a1: *const u8, a2: *const u8, a3: *const u8, a4: *const u8, a5: *const u8) -> bool;
}

pub struct Slot { pub name: &'static str, pub rva: usize, pub orig: AtomicUsize, pub n: AtomicUsize, pub diff: AtomicUsize }
macro_rules! sl { ($n:expr, $r:expr) => { Slot { name: $n, rva: $r, orig: AtomicUsize::new(0), n: AtomicUsize::new(0), diff: AtomicUsize::new(0) } } }
pub static S: [Slot; 9] = [
    sl!("v55_mark", 0xe01450),
    sl!("v55_seal", 0xe019d0),
    sl!("v55_banish", 0xe02020),
    sl!("mw_risk", 0xd95d00),
    sl!("v54_aoe", 0xe02bc0),
    sl!("v55_spirit", 0xe03ed0),
    sl!("abms", 0xe022d0),
    sl!("ncsv", 0xe02540),
    sl!("defc", 0xe01c40),
];
static LOG: Mutex<Vec<String>> = Mutex::new(Vec::new());
thread_local! { static D: [std::cell::Cell<u32>; 9] = [const { std::cell::Cell::new(0) }; 9]; }
#[inline] fn top(i: usize) -> bool { D.with(|d| { let v = d[i].get(); d[i].set(v + 1); v == 0 }) }
#[inline] fn pop(i: usize) { D.with(|d| d[i].set(d[i].get().saturating_sub(1))); }
fn note(i: usize, s: String) { S[i].diff.fetch_add(1, Ordering::Relaxed); let mut g = LOG.lock().unwrap_or_else(|e| e.into_inner()); if g.len() < 30 { g.push(s); } }

unsafe fn w_v55_mark(a0: i64, a1: *const u8, a2: *const u8, a3: *const u8, a4: *const u8, a5: *const u8, a6: *const u8, a7: i64, a8: *const u8) -> i64 {
    let f: unsafe fn(i64, *const u8, *const u8, *const u8, *const u8, *const u8, *const u8, i64, *const u8) -> i64 = core::mem::transmute(S[0].orig.load(Ordering::Relaxed));
    let t = top(0);
    let g = f(a0, a1, a2, a3, a4, a5, a6, a7, a8);
    if !t { pop(0); return g; }
    S[0].n.fetch_add(1, Ordering::Relaxed);
    let m = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| my_v55_mark(a0, a1, a2, a3, a4, a5, a6, a7, a8)));
    if let Ok(m) = m { if m != g { note(0, format!("v55_mark g={:?} m={:?}", g, m)); } }
    pop(0);
    g
}
unsafe fn w_v55_seal(a0: i64, a1: *const u8, a2: *const u8, a3: *const u8, a4: *const u8, a5: i64) -> i64 {
    let f: unsafe fn(i64, *const u8, *const u8, *const u8, *const u8, i64) -> i64 = core::mem::transmute(S[1].orig.load(Ordering::Relaxed));
    let t = top(1);
    let g = f(a0, a1, a2, a3, a4, a5);
    if !t { pop(1); return g; }
    S[1].n.fetch_add(1, Ordering::Relaxed);
    let m = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| my_v55_seal(a0, a1, a2, a3, a4, a5)));
    if let Ok(m) = m { if m != g { note(1, format!("v55_seal g={:?} m={:?}", g, m)); } }
    pop(1);
    g
}
unsafe fn w_v55_banish(a0: i64, a1: *const u8, a2: *const u8, a3: *const u8, a4: *const u8, a5: *const u8, a6: i64) -> i64 {
    let f: unsafe fn(i64, *const u8, *const u8, *const u8, *const u8, *const u8, i64) -> i64 = core::mem::transmute(S[2].orig.load(Ordering::Relaxed));
    let t = top(2);
    let g = f(a0, a1, a2, a3, a4, a5, a6);
    if !t { pop(2); return g; }
    S[2].n.fetch_add(1, Ordering::Relaxed);
    let m = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| my_v55_banish(a0, a1, a2, a3, a4, a5, a6)));
    if let Ok(m) = m { if m != g { note(2, format!("v55_banish g={:?} m={:?}", g, m)); } }
    pop(2);
    g
}
unsafe fn w_mw_risk(a0: i64, a1: *const u8, a2: *const u8, a3: i64, a4: i64, a5: i64) -> i64 {
    let f: unsafe fn(i64, *const u8, *const u8, i64, i64, i64) -> i64 = core::mem::transmute(S[3].orig.load(Ordering::Relaxed));
    let t = top(3);
    let g = f(a0, a1, a2, a3, a4, a5);
    if !t { pop(3); return g; }
    S[3].n.fetch_add(1, Ordering::Relaxed);
    let m = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| my_mw_risk(a0, a1, a2, a3, a4, a5)));
    if let Ok(m) = m { if m != g { note(3, format!("mw_risk g={:?} m={:?}", g, m)); } }
    pop(3);
    g
}
unsafe fn w_v54_aoe(a0: i64, a1: *const u8, a2: *const u8, a3: *const u8, a4: *const u8, a5: *const u8, a6: i64, a7: *const u8) -> i64 {
    let f: unsafe fn(i64, *const u8, *const u8, *const u8, *const u8, *const u8, i64, *const u8) -> i64 = core::mem::transmute(S[4].orig.load(Ordering::Relaxed));
    let t = top(4);
    let g = f(a0, a1, a2, a3, a4, a5, a6, a7);
    if !t { pop(4); return g; }
    S[4].n.fetch_add(1, Ordering::Relaxed);
    let m = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| my_v54_aoe(a0, a1, a2, a3, a4, a5, a6, a7)));
    if let Ok(m) = m { if m != g { note(4, format!("v54_aoe g={:?} m={:?}", g, m)); } }
    pop(4);
    g
}
unsafe fn w_v55_spirit(a0: *const u8, a1: *const u8, a2: *const u8, a3: *const u8, a4: *const u8, a5: i64) -> i64 {
    let f: unsafe fn(*const u8, *const u8, *const u8, *const u8, *const u8, i64) -> i64 = core::mem::transmute(S[5].orig.load(Ordering::Relaxed));
    let t = top(5);
    let g = f(a0, a1, a2, a3, a4, a5);
    if !t { pop(5); return g; }
    S[5].n.fetch_add(1, Ordering::Relaxed);
    let m = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| my_v55_spirit(a0, a1, a2, a3, a4, a5)));
    if let Ok(m) = m { if m != g { note(5, format!("v55_spirit g={:?} m={:?}", g, m)); } }
    pop(5);
    g
}
unsafe fn w_abms(a0: i64, a1: *const u8, a2: *const u8, a3: *const u8, a4: *const u8, a5: i64) -> i64 {
    let f: unsafe fn(i64, *const u8, *const u8, *const u8, *const u8, i64) -> i64 = core::mem::transmute(S[6].orig.load(Ordering::Relaxed));
    let t = top(6);
    let g = f(a0, a1, a2, a3, a4, a5);
    if !t { pop(6); return g; }
    S[6].n.fetch_add(1, Ordering::Relaxed);
    let m = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| my_abms(a0, a1, a2, a3, a4, a5)));
    if let Ok(m) = m { if m != g { note(6, format!("abms g={:?} m={:?}", g, m)); } }
    pop(6);
    g
}
unsafe fn w_ncsv(a0: *const u8, a1: *const u8, a2: *const u8, a3: *const u8) -> i64 {
    let f: unsafe fn(*const u8, *const u8, *const u8, *const u8) -> i64 = core::mem::transmute(S[7].orig.load(Ordering::Relaxed));
    let t = top(7);
    let g = f(a0, a1, a2, a3);
    if !t { pop(7); return g; }
    S[7].n.fetch_add(1, Ordering::Relaxed);
    let m = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| my_ncsv(a0, a1, a2, a3)));
    if let Ok(m) = m { if m != g { note(7, format!("ncsv g={:?} m={:?}", g, m)); } }
    pop(7);
    g
}
unsafe fn w_defc(a0: i64, a1: *const u8, a2: *const u8, a3: *const u8, a4: *const u8, a5: *const u8) -> bool {
    let f: unsafe fn(i64, *const u8, *const u8, *const u8, *const u8, *const u8) -> bool = core::mem::transmute(S[8].orig.load(Ordering::Relaxed));
    let t = top(8);
    let mut s0 = [0u8; 320]; if t { core::ptr::copy_nonoverlapping(a1, s0.as_mut_ptr(), 320); }
    let g = f(a0, a1, a2, a3, a4, a5);
    if !t { pop(8); return g; }
    S[8].n.fetch_add(1, Ordering::Relaxed);
    let mut af = [0u8; 320]; core::ptr::copy_nonoverlapping(a1, af.as_mut_ptr(), 320);
    core::ptr::copy_nonoverlapping(s0.as_ptr(), a1 as *mut u8, 320);
    let m = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| my_defc(a0, a1, a2, a3, a4, a5)));
    core::ptr::copy_nonoverlapping(af.as_ptr(), a1 as *mut u8, 320);
    if let Ok(m) = m { if m != g { note(8, format!("defc g={:?} m={:?}", g, m)); } }
    pop(8);
    g
}

pub unsafe fn install(log: &mut String) {
    let mask = tune("fn_sweep", 0);
    if mask == 0 { return; }
    let w: [usize; 9] = [w_v55_mark as usize, w_v55_seal as usize, w_v55_banish as usize, w_mw_risk as usize, w_v54_aoe as usize, w_v55_spirit as usize, w_abms as usize, w_ncsv as usize, w_defc as usize];
    let pro: [&[u8]; 9] = [&[0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x55, 0x53], &[0x41, 0x57, 0x41, 0x56, 0x56, 0x57, 0x55, 0x53, 0x48, 0x83, 0xec, 0x38], &[0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x55, 0x53], &[0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x55, 0x53], &[0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x55, 0x53], &[0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x55, 0x53], &[0x56, 0x57, 0x53, 0x48, 0x83, 0xec, 0x20, 0x48, 0x8b, 0x74, 0x24, 0x68], &[0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x55, 0x53], &[0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x53]];
    for i in 0..S.len() {
        if mask & (1i64 << i) == 0 { continue; }
        match super::hook::install_wrap_bytes(S[i].rva, pro[i], w[i]) {
            Ok(o) => { S[i].orig.store(o, Ordering::Relaxed); log.push_str(&format!("[sweep] {} OK @{:#x}\n", S[i].name, S[i].rva)); }
            Err(e) => log.push_str(&format!("[sweep] {} 실패: {} @{:#x}\n", S[i].name, e, S[i].rva)),
        }
    }
}
pub fn report() -> String {
    let mut s = String::from("[sweep] 판정계층 RVA 확정 함수 전수 대조 (게임 vs 링크 사본)\n");
    for (i, x) in S.iter().enumerate() {
        let (n, dd) = (x.n.load(Ordering::Relaxed), x.diff.load(Ordering::Relaxed));
        if n == 0 && dd == 0 { continue; }
        s.push_str(&format!("  bit{:<2} {:<22} n={:>10} DIFF={:>7}\n", i, x.name, n, dd));
    }
    let g = LOG.lock().unwrap_or_else(|e| e.into_inner());
    for l in g.iter() { s.push_str("   "); s.push_str(l); s.push('\n'); }
    s
}
