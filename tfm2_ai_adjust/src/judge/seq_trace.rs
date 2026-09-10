//! seq_trace — **호출 순서 대조**: 한 번의 `get_input` 동안 게임과 내 사본이 각각
//! 어떤 함수를 어떤 인자로 몇 번 불렀는지 **합쳐진 타임라인**으로 기록해 첫 갈라짐 지점을 찾는다.
//!
//! 왜 필요한가: 함수 단위 대조는 3.2억 콜 전부 DIFF=0 인데 에이전트 단위는 0.02% 갈린다.
//!   즉 "같은 질문엔 같은 답"인데 **질문 자체가 달라지는** 지점이 있다.
//!   1차 결과(position_eval_at 단독 추적, 2026-09-09): 872만 건 중 1,881건에서 시퀀스가 다르고
//!   그 대부분이 `len g=0 m=1` — **게임은 위치 평가를 아예 안 했는데 사본은 했다** ⟹ 갈라짐은 그 위 관문.
//!   ⟹ 여러 함수를 한 타임라인에 섞어 기록하면, 첫 불일치의 **함수 태그**가 관문을 가리킨다.
//!
//! 내 사본의 내부 호출은 exe 훅에 안 걸리므로 **내 DLL 안 코드에도 같은 훅**을 건다
//! (`extern` 심볼로 주소를 얻어 진입부 12B 패치 — 사본 함수는 아무도 안 잡았으므로 push 프롤로그 그대로).
//! 진단 전용(cfg `seq_trace`, 기본 0).
use crate::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

/// 한 호출의 기록. `t` = 함수 태그.
#[derive(Clone, Copy)]
pub struct Rec { pub t: u8, pub a: i64, pub b: i64, pub c: i64, pub r: i64 }
/// ★반환값이 **값이 아니라 주소**일 수 있다 — 두 사본은 서로 다른 힙을 쓰므로
///   주소를 값으로 비교하면 **항상 다르다**. `small_action` 첫 8B 와 같은 함정.
///   실측(2026-09-10): siege_stance 반환 g=0x119803f2000 / m=0x119803f2cc0 — 3264B 떨어진 따로 할당된 것.
///   원인 = IR 은 `define { i64, i64 }`(RAX:RDX) 인데 Rust extern 은 `-> (i64,i64)` 로 선언돼 ABI 가 어깼다.
#[inline] fn ptrish(v: i64) -> bool { (v as u64) >= 0x10000 && (v as u64) < (1u64 << 48) }
#[inline] fn ret_same(a: i64, b: i64) -> bool { a == b || (ptrish(a) && ptrish(b)) }
impl Rec {
    /// 인자까지만 같은가 — 다르면 원인은 **이 함수 위(호출자)** 에 있다.
    #[inline] pub fn args_same(&self, o: &Rec) -> bool { self.t == o.t && self.a == o.a && self.b == o.b && self.c == o.c }
    /// 인자도 반환도 같은가.
    #[inline] pub fn same(&self, o: &Rec) -> bool { self.args_same(o) && ret_same(self.r, o.r) }
}

pub const NAMES: [&str; 7] = ["position_eval_at", "possible_risk", "mw_risk", "siege_stance", "pos_risk_zero", "interaction_score", "eval_position"];

thread_local! {
    static SIDE: std::cell::Cell<u8> = std::cell::Cell::new(0);      // 0=끔 1=게임 2=사본
    static SEQ_G: std::cell::RefCell<Vec<Rec>> = std::cell::RefCell::new(Vec::new());
    static SEQ_M: std::cell::RefCell<Vec<Rec>> = std::cell::RefCell::new(Vec::new());
    static DEPTH: std::cell::Cell<u32> = std::cell::Cell::new(0);
}

pub static CMP: AtomicUsize = AtomicUsize::new(0);
pub static OK: AtomicUsize = AtomicUsize::new(0);
pub static LEN_DIFF: AtomicUsize = AtomicUsize::new(0);
pub static ARG_DIFF: AtomicUsize = AtomicUsize::new(0);
pub static FIRST_TAG: [AtomicUsize; 8] = [const { AtomicUsize::new(0) }; 8];
/// 첫 불일치가 **인자**에서 난 것 — 원인은 그 함수 위에 있다.
pub static FIRST_ARG: [AtomicUsize; 8] = [const { AtomicUsize::new(0) }; 8];
/// 첫 불일치가 **반환값**에서만 난 것 — 같은 질문에 다른 답 = 그 함수 안이 원인.
pub static FIRST_RET: [AtomicUsize; 8] = [const { AtomicUsize::new(0) }; 8];

static ORIG_G: [AtomicUsize; 7] = [const { AtomicUsize::new(0) }; 7];
static ORIG_M: [AtomicUsize; 7] = [const { AtomicUsize::new(0) }; 7];
static LOG: Mutex<Vec<String>> = Mutex::new(Vec::new());
static LOG_AC: Mutex<Vec<String>> = Mutex::new(Vec::new());
pub static AC_CMP: AtomicUsize = AtomicUsize::new(0);
pub static AC_FIRST: [AtomicUsize; 8] = [const { AtomicUsize::new(0) }; 8];

extern "Rust" {
    #[link_name = "_RNvNtCshdEBA0ozCnw_7game_ai13position_eval16position_eval_at"]
    fn m_pe(out: *mut u8, a: i64, b: *const u8, c: *const u8, d: i64, e: i64, f: u8);
    #[link_name = "_RNvMs0_NtCshdEBA0ozCnw_7game_ai15score_parameterNtB5_22ChampionScoreParameter13possible_risk"]
    fn m_pr(a: *const u8, b: *const u8, c: i64) -> i64;
    #[link_name = "_RNvNtCshdEBA0ozCnw_7game_ai16minion_wave_risk32enemy_minion_wave_risk_damage_at"]
    fn m_mw(a: i64, b: *const u8, c: *const u8, d: i64, e: i64, f: i64) -> i64;
    #[link_name = "_RNvNtCshdEBA0ozCnw_7game_ai16tower_discipline16v47_siege_stance"]
    fn m_ss(a: i64, b: *const u8, c: *const u8, d: *const u8) -> (i64, i64);
    #[link_name = "_RNvNtCshdEBA0ozCnw_7game_ai13position_eval27position_risk_all_zero_near"]
    fn m_pz(a: i64, b: *const u8, c: *const u8, d: *const u8, e: u8) -> bool;
    /// ★position_eval_at 의 **직접 호출자** — 첫 불일치가 position_eval_at 의 인자에서 난다면
    ///   원인은 이 함수 안이거나 그 위다. 추가해야 둘을 가른다. ABI = fn_bisect bit 8 검증본.
    #[link_name = "_RNvNtCshdEBA0ozCnw_7game_ai12action_score17interaction_score"]
    fn m_is(a: i64, rng: *mut u8, c: *const u8, d: *const u8, e: *const u8, f: *const u8, dbg: *mut u8) -> i64;
    /// small_action 평가 진입. sret 24B. ABI = fn_bisect bit 10 검증본.
    #[link_name = "_RNvMNtCshdEBA0ozCnw_7game_ai12small_actionNtB2_15SmallActionPlay19evaluation_position"]
    fn m_ep(out: *mut u8, s: *const u8, a: i64, b: *const u8, c: *const u8);
}

#[inline] pub fn begin_game() { SIDE.with(|s| s.set(1)); SEQ_G.with(|v| v.borrow_mut().clear()); }
#[inline] pub fn begin_mine() { SIDE.with(|s| s.set(2)); SEQ_M.with(|v| v.borrow_mut().clear()); }
#[inline] pub fn end() { SIDE.with(|s| s.set(0)); }

#[inline] fn push(r: Rec) {
    match SIDE.with(|s| s.get()) {
        1 => SEQ_G.with(|v| { let mut v = v.borrow_mut(); if v.len() < 8192 { v.push(r); } }),
        2 => SEQ_M.with(|v| { let mut v = v.borrow_mut(); if v.len() < 8192 { v.push(r); } }),
        _ => {}
    }
}
/// ★방금 push 한 항목의 반환값 칸을 채운다 — "같은 함수·같은 인자인데 다른 값"을 잡기 위해.
#[inline] fn set_ret(v: i64) {
    match SIDE.with(|s| s.get()) {
        1 => SEQ_G.with(|q| { if let Some(l) = q.borrow_mut().last_mut() { l.r = v; } }),
        2 => SEQ_M.with(|q| { if let Some(l) = q.borrow_mut().last_mut() { l.r = v; } }),
        _ => {}
    }
}
#[inline] fn top() -> bool { DEPTH.with(|d| { let v = d.get(); d.set(v + 1); v == 0 }) }
#[inline] fn pop() { DEPTH.with(|d| d.set(d.get().saturating_sub(1))); }

pub fn compare(seed: u64, tick: u32, pl: u16) {
    CMP.fetch_add(1, Ordering::Relaxed);
    SEQ_G.with(|g| SEQ_M.with(|m| {
        let (g, m) = (g.borrow(), m.borrow());
        let n = g.len().min(m.len());
        let first = (0..n).find(|&i| !g[i].same(&m[i]));
        if first.is_none() && g.len() == m.len() { OK.fetch_add(1, Ordering::Relaxed); return; }
        let i = first.unwrap_or(n);
        // 첫 불일치가 어느 함수에서 났나 — 관문을 가리키는 지표
        let tag = g.get(i).map(|r| r.t).or_else(|| m.get(i).map(|r| r.t)).unwrap_or(7);
        let ti = (tag as usize).min(7);
        FIRST_TAG[ti].fetch_add(1, Ordering::Relaxed);
        if first.is_some() { ARG_DIFF.fetch_add(1, Ordering::Relaxed); } else { LEN_DIFF.fetch_add(1, Ordering::Relaxed); }
        // ★인자가 다른가, 답만 다른가 — 원인이 이 함수 위인지 안인지를 가른다.
        if let (Some(a), Some(b)) = (g.get(i), m.get(i)) {
            if a.args_same(b) { FIRST_RET[ti].fetch_add(1, Ordering::Relaxed); }
            else { FIRST_ARG[ti].fetch_add(1, Ordering::Relaxed); }
        }
        let mut lg = LOG.lock().unwrap_or_else(|e| e.into_inner());
        if lg.len() < 24 {
            let f = |r: Option<&Rec>| r.map(|r| format!("{} a={} b={} c={} -> {}", NAMES[(r.t as usize).min(6)], r.a, r.b, r.c, r.r)).unwrap_or_else(|| "(없음)".into());
            let ctx: Vec<String> = (i.saturating_sub(2)..i).map(|k| f(g.get(k))).collect();
            lg.push(format!("seed={:016x} t={} pl={:04x} len g={} m={} · 첫 불일치 #{}\n      직전: {}\n      g: {}\n      m: {}",
                seed, tick, pl, g.len(), m.len(), i, ctx.join(" | "), f(g.get(i)), f(m.get(i))));
        }
    }));
}

/// ★A≠C(게임코드×게임상태 vs 내코드×게임상태) 순간의 호출순서 대조.
///   상태가 바이트까지 같은 두 실행이니, 여기서 갈라지는 지점이 **순수 코드 차이**의 발생지다.
pub fn compare_ac(seed: u64, tick: u32, pl: u16) {
    AC_CMP.fetch_add(1, Ordering::Relaxed);
    SEQ_G.with(|g| SEQ_M.with(|m| {
        let (g, m) = (g.borrow(), m.borrow());
        let n = g.len().min(m.len());
        let first = (0..n).find(|&i| !g[i].same(&m[i]));
        let i = first.unwrap_or(n);
        let tag = g.get(i).map(|r| r.t).or_else(|| m.get(i).map(|r| r.t)).unwrap_or(7);
        AC_FIRST[(tag as usize).min(7)].fetch_add(1, Ordering::Relaxed);
        let mut lg = LOG_AC.lock().unwrap_or_else(|e| e.into_inner());
        if lg.len() < 16 {
            let f = |r: Option<&Rec>| r.map(|r| format!("{} a={} b={} c={} -> {}", NAMES[(r.t as usize).min(6)], r.a, r.b, r.c, r.r)).unwrap_or_else(|| "(없음)".into());
            let ctx: Vec<String> = (i.saturating_sub(3)..i).map(|k| f(g.get(k))).collect();
            lg.push(format!("seed={:016x} t={} pl={:04x} len A={} C={} · 첫 불일치 #{}\n      직전: {}\n      A(게임코드): {}\n      C(내코드): {}",
                seed, tick, pl, g.len(), m.len(), i, ctx.join(" | "), f(g.get(i)), f(m.get(i))));
        }
    }));
}

/// ★`interaction_score` 의 반환이 갈렸을 때, 그 안에서 **어느 내부 호출부터 달랐는지** 를 찍는다.
///   fn_bisect::wrap_is 가 게임 호출을 begin_game, 내 호출을 begin_mine 으로 감싸 둔 다음 부른다.
pub static IS_CMP: AtomicUsize = AtomicUsize::new(0);
pub static IS_SAME_SEQ: AtomicUsize = AtomicUsize::new(0);
pub static IS_FIRST: [AtomicUsize; 8] = [const { AtomicUsize::new(0) }; 8];
static LOG_IS: Mutex<Vec<String>> = Mutex::new(Vec::new());
pub fn compare_is(gr: i64, mr: i64) {
    IS_CMP.fetch_add(1, Ordering::Relaxed);
    SEQ_G.with(|g| SEQ_M.with(|m| {
        let (g, m) = (g.borrow(), m.borrow());
        let n = g.len().min(m.len());
        let first = (0..n).find(|&i| !g[i].same(&m[i]));
        if first.is_none() && g.len() == m.len() {
            // 내부 호출은 하나도 안 달랐는데 답만 다르다 = 이 함수 자기 산술/분기가 원인이다.
            IS_SAME_SEQ.fetch_add(1, Ordering::Relaxed);
        }
        let i = first.unwrap_or(n);
        let tag = g.get(i).map(|r| r.t).or_else(|| m.get(i).map(|r| r.t)).unwrap_or(7);
        IS_FIRST[(tag as usize).min(7)].fetch_add(1, Ordering::Relaxed);
        let mut lg = LOG_IS.lock().unwrap_or_else(|e| e.into_inner());
        if lg.len() < 20 {
            let f = |r: Option<&Rec>| r.map(|r| format!("{} a={} b={} c={} -> {}", NAMES[(r.t as usize).min(6)], r.a, r.b, r.c, r.r)).unwrap_or_else(|| "(없음)".into());
            let ctx: Vec<String> = (i.saturating_sub(2)..i).map(|k| f(g.get(k))).collect();
            lg.push(format!("g={} m={} · 내부호출 g={} m={} · 첫 차이 #{}\n      직전: {}\n      게임: {}\n      사본: {}",
                gr, mr, g.len(), m.len(), i, ctx.join(" | "), f(g.get(i)), f(m.get(i))));
        }
    }));
}
macro_rules! pair {
    ($wg:ident, $wm:ident, $idx:expr, $mine:ident, ($($an:ident: $at:ty),*), $ret:ty, $rec:expr, $ret_bind:ident, $retexpr:expr) => {
        unsafe fn $wg($($an: $at),*) -> $ret {
            let f: unsafe fn($($at),*) -> $ret = core::mem::transmute(ORIG_G[$idx].load(Ordering::Relaxed));
            let t = top(); if t { push($rec); }
            let r = f($($an),*); pop();
            if t { let rv: i64 = { let $ret_bind = &r; $retexpr }; set_ret(rv); }
            r
        }
        unsafe fn $wm($($an: $at),*) -> $ret {
            let f: unsafe fn($($at),*) -> $ret = core::mem::transmute(ORIG_M[$idx].load(Ordering::Relaxed));
            let t = top(); if t { push($rec); }
            let r = f($($an),*); pop();
            if t { let rv: i64 = { let $ret_bind = &r; $retexpr }; set_ret(rv); }
            r
        }
    };
}
pair!(g_pe, m2_pe, 0, m_pe, (out: *mut u8, a: i64, b: *const u8, c: *const u8, d: i64, e: i64, ff: u8), (), Rec { t: 0, a, b: d, c: e, r: 0 }, _z, core::ptr::read_unaligned(out as *const i64));
pair!(g_pr, m2_pr, 1, m_pr, (a: *const u8, b: *const u8, c: i64), i64, Rec { t: 1, a: c, b: 0, c: 0, r: 0 }, z, *z);
// ★mw_risk 의 **첫 인자(%0)는 죽은 파라미터**다 — IR 호출부가 `i64 poison` 을 넘기고
//   본문 362줄에서 %0 사용 0회(2026-09-10 확인). 레지스터 잔재가 그대로 들어오므로
//   이걸 기록하면 두 사본이 항상 다르게 보인다(실측: g=2829497437384 m=2829497437424 — 40 차이,
//   b·c·반환값은 전부 동일). 이것만으로 가짜 불일치 25,348건이 나왔다.
//   대신 실제로 쓰이는 6번째 인자(%5)를 기록한다.
pair!(g_mw, m2_mw, 2, m_mw, (a: i64, b: *const u8, c: *const u8, d: i64, e: i64, ff: i64), i64, Rec { t: 2, a: ff, b: d, c: e, r: 0 }, z, *z);
pair!(g_ss, m2_ss, 3, m_ss, (a: i64, b: *const u8, c: *const u8, d: *const u8), (i64, i64), Rec { t: 3, a, b: 0, c: 0, r: 0 }, z, z.0 ^ (z.1 << 1));
pair!(g_pz, m2_pz, 4, m_pz, (a: i64, b: *const u8, c: *const u8, d: *const u8, e: u8), bool, Rec { t: 4, a, b: e as i64, c: 0, r: 0 }, z, *z as i64);
pair!(g_is, m2_is, 5, m_is, (a: i64, rng: *mut u8, c: *const u8, d: *const u8, e: *const u8, ff: *const u8, dbg: *mut u8), i64, Rec { t: 5, a, b: 0, c: 0, r: 0 }, z, *z);
// ⚠eval_position 은 sret — 반환값 칸에는 out 의 첫 i64 를 넣는다(포인터면 ret_same 가 면제한다).
pair!(g_ep, m2_ep, 6, m_ep, (out: *mut u8, s: *const u8, a: i64, b: *const u8, c: *const u8), (), Rec { t: 6, a, b: 0, c: 0, r: 0 }, _z, core::ptr::read_unaligned(out as *const i64));

/// 내 DLL 안 함수에 wrap 설치 — 진입부를 런타임에 읽어 push 계열 12B 인지 확인한 뒤 옮긴다.
unsafe fn install_at(addr: usize, cap: usize) -> Result<usize, &'static str> {
    if !readable(addr, 20) { return Err("unreadable"); }
    // 재배치 안전한 프롤로그 형태만 옮긴다(rip-상대·분기 없음):
    //   push r64(1~2B) · sub rsp,imm8(4B) · sub rsp,imm32(7B) · lea rbp,[rsp+imm8](5B) · lea rbp,[rsp+imm32](8B)
    //   ⚠push 만 받으면 `possible_risk`(push×6 + sub rsp + lea rbp)를 못 잡는다(2026-09-09 실측 — 그 탓에
    //     사본 시퀀스에서 possible_risk 가 통째로 빠져 40만 건이 가짜 불일치로 잡혔다).
    let mut n = 0usize;
    while n < 24 {
        let b = |k: usize| *((addr + n + k) as *const u8);
        let b0 = b(0);
        if b0 == 0x41 && (b(1) & 0xf8) == 0x50 { n += 2; }
        else if (b0 & 0xf8) == 0x50 { n += 1; }
        else if b0 == 0x48 && b(1) == 0x83 && b(2) == 0xec { n += 4; }
        else if b0 == 0x48 && b(1) == 0x81 && b(2) == 0xec { n += 7; }
        else if b0 == 0x48 && b(1) == 0x8d && b(2) == 0x6c && b(3) == 0x24 { n += 5; }
        else if b0 == 0x48 && b(1) == 0x8d && b(2) == 0xac && b(3) == 0x24 { n += 8; }
        else { break; }
        if n >= 12 { break; }
    }
    if n < 12 || n > 24 { return Err("프롤로그를 12B 이상 안전하게 못 옮김"); }
    const MEM_CR: u32 = 0x1000 | 0x2000; const RWX: u32 = 0x40;
    let stub = VirtualAlloc(0, 128, MEM_CR, RWX);
    if stub == 0 { return Err("VirtualAlloc"); }
    let mut s: Vec<u8> = (0..n).map(|i| *((addr + i) as *const u8)).collect();
    s.extend_from_slice(&[0xff, 0x25, 0, 0, 0, 0]);
    s.extend_from_slice(&(addr + n).to_le_bytes());
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    let mut patch = vec![0x90u8; n];
    patch[0] = 0x48; patch[1] = 0xb8; patch[2..10].copy_from_slice(&cap.to_le_bytes()); patch[10] = 0xff; patch[11] = 0xe0;
    let mut old: u32 = 0;
    if VirtualProtect(addr, n, RWX, &mut old) == 0 { return Err("VirtualProtect"); }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), addr as *mut u8, n);
    VirtualProtect(addr, n, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), addr, n);
    Ok(stub)
}

pub unsafe fn install(log: &mut String) {
    if tune("seq_trace", 0) == 0 { return; }
    // (태그, exe RVA, exe 프롤로그, exe wrap, 내 사본 주소, 내 사본 wrap)
    // ★interaction_score(태그 5) 는 일부러 안 건다 — 그걸 걸면 DEPTH 가드가 **그 안의 호출을 전부 억제**해
    //   정작 알고 싶은 내부 호출열이 빈 채로 남는다. 대신 fn_bisect::wrap_is 가 begin_game/begin_mine 으로 감싸준다.
    let t: [(usize, usize, &[u8], usize, usize, usize); 6] = [
        (0, 0xd84db0, super::AS_D84DB0.prolog, g_pe as usize, m_pe as usize, m2_pe as usize),
        (1, 0xd83230, super::AS_D83230.prolog, g_pr as usize, m_pr as usize, m2_pr as usize),
        (2, 0xd95d00, super::MW_RISK.prolog, g_mw as usize, m_mw as usize, m2_mw as usize),
        (3, 0xd96d00, super::AS_D96D00.prolog, g_ss as usize, m_ss as usize, m2_ss as usize),
        (4, 0xd8ca70, super::fn_bisect::PRZ.prolog, g_pz as usize, m_pz as usize, m2_pz as usize),
        (6, 0xe23170, super::fn_bisect::EPOS.prolog, g_ep as usize, m_ep as usize, m2_ep as usize),
    ];
    for (i, rva, prolog, wg, mine, wm) in t {
        // exe 쪽은 judge 훅이 이미 잡았을 수 있으니 체인 지원 설치기
        match super::hook::install_wrap_bytes(rva, prolog, wg) {
            Ok(o) => { ORIG_G[i].store(o, Ordering::Relaxed); }
            Err(e) => { log.push_str(&format!("[seq] exe {} 실패: {}\n", NAMES[i], e)); continue; }
        }
        match install_at(mine, wm) {
            Ok(o) => { ORIG_M[i].store(o, Ordering::Relaxed); log.push_str(&format!("[seq] {} 양쪽 훅 OK\n", NAMES[i])); }
            Err(e) => log.push_str(&format!("[seq] 사본 {} 실패: {}\n", NAMES[i], e)),
        }
    }
}

pub fn report() -> String {
    let mut s = format!("[seq] get_input 1회 단위 호출순서 대조: 비교 {} · 일치 {} · 길이불일치 {} · 인자불일치 {}\n",
        CMP.load(Ordering::Relaxed), OK.load(Ordering::Relaxed), LEN_DIFF.load(Ordering::Relaxed), ARG_DIFF.load(Ordering::Relaxed));
    s.push_str("  첫 불일치가 난 함수: ");
    for (i, n) in NAMES.iter().enumerate() { s.push_str(&format!("{}={} ", n, FIRST_TAG[i].load(Ordering::Relaxed))); }
    {
        let c = IS_CMP.load(Ordering::Relaxed);
        if c > 0 {
            s.push_str(&format!("\n[seq-IS] interaction_score 반환이 갈린 {}건 · 그중 내부 호출열은 완전히 같았던 것 {}건\n  첫 차이 함수: ",
                c, IS_SAME_SEQ.load(Ordering::Relaxed)));
            for (i, n) in NAMES.iter().enumerate() { let v = IS_FIRST[i].load(Ordering::Relaxed); if v > 0 { s.push_str(&format!("{}={} ", n, v)); } }
            s.push_str(&format!("끝길이차={}\n", IS_FIRST[7].load(Ordering::Relaxed)));
            let lg = LOG_IS.lock().unwrap_or_else(|e| e.into_inner());
            for l in lg.iter() { s.push_str("   "); s.push_str(l); s.push('\n'); }
        }
    }
    s.push_str("\n  └ 그중 인자가 다름(원인은 그 함수 위) / 답만 다름(원인은 그 함수 안): ");
    for (i, n) in NAMES.iter().enumerate() {
        let (fa, fr) = (FIRST_ARG[i].load(Ordering::Relaxed), FIRST_RET[i].load(Ordering::Relaxed));
        if fa + fr > 0 { s.push_str(&format!("{}={}⇑/{}★ ", n, fa, fr)); }
    }
    s.push('\n');
    let g = LOG.lock().unwrap_or_else(|e| e.into_inner());
    for l in g.iter() { s.push_str("   "); s.push_str(l); s.push('\n'); }
    s.push_str(&format!("[seq-AC] 상태가 같은데 갈라진 건(A≠C) {}건 · 첫 불일치 함수: ", AC_CMP.load(Ordering::Relaxed)));
    for (i, n) in NAMES.iter().enumerate() { s.push_str(&format!("{}={} ", n, AC_FIRST[i].load(Ordering::Relaxed))); }
    s.push_str(&format!("끝길이차={}\n", AC_FIRST[5].load(Ordering::Relaxed)));
    let a = LOG_AC.lock().unwrap_or_else(|e| e.into_inner());
    for l in a.iter() { s.push_str("   "); s.push_str(l); s.push('\n'); }
    s
}
