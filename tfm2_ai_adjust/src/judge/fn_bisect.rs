//! fn_bisect — **함수 단위 이분 탐색**: 게임(exe)의 game_ai 함수와 **내 DLL 에 링크된 사본**의 같은 함수를
//! 같은 인자로 불러 반환값을 비트 대조한다.
//!
//! 왜: 에이전트 단위 트윈 비교(`agent_twin`)에서 `get_input` 결정이 0.025% 갈리는데,
//!   진입 상태는 99% 동일했다 ⟹ 갈라짐은 **호출 안쪽**이다. 어느 함수인지는 에이전트 경계에서는 알 수 없다.
//!   그래서 경계를 함수로 내린다 — 잎(leaf)에서 불일치가 나오면 그 위는 전부 그 결과다.
//!
//! 방법: exe 함수 진입부를 wrap 해서 ① 원본(트램폴린) 실행 → ② **같은 인자로 내 사본** 실행 → ③ 대조 → ④ 원본값 반환.
//!   게임 동작은 바뀌지 않는다(항상 원본값을 돌려준다).
//!   ⚠내 사본 호출은 부작용(TLS 캐시 채움)이 있으므로 이 모듈은 **진단 전용**(cfg `fn_bisect`, 기본 0).
//!   ⚠judge 훅이 이미 걸린 함수에는 `hook.rs` 의 체인 분기가 적용된다(늦게 설치 = 바깥).
//!
//! 대상 선정: 이동/공격 목표가 갈리므로 그 경로의 잎부터.
//!   `possible_risk`(위험도) · `enemy_minion_wave_risk_damage_at`(미니언 웨이브 위험) · `champion_hp_value`(HP 가치, HP_VALUE_MEMO 보유)
//!   · `v47_siege_stance`(공성 태세, SIEGE_STANCE_CACHE 보유) · `tower_dive_is_viable`(다이브 판정).
use crate::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

// ── 내 DLL 에 링크된 사본(SDK game_ai 비트코드)의 같은 함수들. 시그니처는 IR `define` 실측.
extern "Rust" {
    /// `<ChampionScoreParameter>::possible_risk(ptr, ptr, i64) -> i64`
    #[link_name = "_RNvMs0_NtCshdEBA0ozCnw_7game_ai15score_parameterNtB5_22ChampionScoreParameter13possible_risk"]
    fn my_possible_risk(a: *const u8, b: *const u8, c: i64) -> i64;
    /// `enemy_minion_wave_risk_damage_at(i64, ptr, ptr, i64, i64, i64) -> i64`
    #[link_name = "_RNvNtCshdEBA0ozCnw_7game_ai16minion_wave_risk32enemy_minion_wave_risk_damage_at"]
    fn my_mw_risk(a: i64, b: *const u8, c: *const u8, d: i64, e: i64, f: i64) -> i64;
    /// `interaction_score(i64, &mut StdRng(320B), ptr(2528), ptr(24), ptr(5384), ptr(184), &mut DebugFrameData(224B)) -> i64`
    ///   ⚠두 번째 인자가 **가변 StdRng** 이라 사본을 돌리기 전에 320B 를 떠서 돌려놓아야 한다.
    #[link_name = "_RNvNtCshdEBA0ozCnw_7game_ai12action_score17interaction_score"]
    fn my_interaction_score(a: i64, rng: *mut u8, c: *const u8, d: *const u8, e: *const u8, f: *const u8, dbg: *mut u8) -> i64;
    /// `SmallActionPlay::evaluation_position(sret 24B, &self 184B, i64, ptr2528, ptr24)`
    #[link_name = "_RNvMNtCshdEBA0ozCnw_7game_ai12small_actionNtB2_15SmallActionPlay19evaluation_position"]
    fn my_eval_pos(out: *mut u8, s: *const u8, a: i64, b: *const u8, c: *const u8);
    /// `game_core::simulation::effect::Effect::expected_damage_target(&self 56B, ptr64, ptr, ptr88, ptr1728) -> i64`
    ///   ★game_core 쪽 헬퍼 — 내 모드는 game_core 도 자기 사본을 링크하므로 이걸 대조해야 game_core 동일성을 본다.
    #[link_name = "_RNvMNtNtCs97f5S1uJLkH_9game_core10simulation6effectNtB2_6Effect22expected_damage_target"]
    fn my_exp_dmg(s: *const u8, b: *const u8, c: *const u8, d: *const u8, e: *const u8) -> i64;
    /// ★HP 메모 캐시 접근자 — `LocalKey::<RefCell<(u64,u64,HashMap<usize,i64,ahash>)>>::with(&key, &env 40B) -> i64`
    ///   exe 쪽은 `0xc87fe0`(기존 UTIL_C87FE0). `champion_hp_value` 의 래퍼 본체는 exe 에서 인라인돼
    ///   **이 접근자만이 유일한 후킹 경계**다.
    #[link_name = "_RINvMs2_NtNtCs9ec1k27omRZ_3std6thread5localINtB6_8LocalKeyINtNtCsjihNppCmMEE_4core4cell7RefCellTyjjINtNtNtNtBa_11collections4hash3map7HashMapjxNtNtCs9EYcZKFYzm_5ahash12random_state11RandomStateEEEE4withNCNvNtCshdEBA0ozCnw_7game_ai5utils17champion_hp_value0xEB3k_"]
    fn my_hp_with(key: *const u8, env: *const u8) -> i64;
    /// 내 사본의 `champion_hp_value` — 주소만 필요하다(본문을 디코딩해 LocalKey 정적 주소를 찾는다).
    #[link_name = "_RNvNtCshdEBA0ozCnw_7game_ai5utils17champion_hp_value"]
    fn my_hp_value(a: *const u8, b: *const u8, c: *const u8) -> i64;
    /// `champion_hp_value(ptr, ptr, ptr) -> i64`
    #[link_name = "_RNvNtCshdEBA0ozCnw_7game_ai5utils17champion_hp_value"]
    fn my_champion_hp_value(a: *const u8, b: *const u8, c: *const u8) -> i64;
    /// `v47_siege_stance(i64, ptr, ptr, ptr) -> {i64, i64}`
    #[link_name = "_RNvNtCshdEBA0ozCnw_7game_ai16tower_discipline16v47_siege_stance"]
    fn my_siege_stance(a: i64, b: *const u8, c: *const u8, d: *const u8) -> (i64, i64);
    /// `tower_dive_is_viable(i64, ptr, ptr, ptr, ptr, ptr, i1, ptr) -> i1`
    #[link_name = "_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old11fight_model20tower_dive_is_viable"]
    fn my_tower_dive(a: i64, b: *const u8, c: *const u8, d: *const u8, e: *const u8, f: *const u8, g: bool, h: *const u8) -> bool;
    /// `position_eval_at(sret [56 x i8], i64, ptr, ptr, i64, i64, i8)` — exe `0xd84db0`(197명령 : IR 203줄)
    #[link_name = "_RNvNtCshdEBA0ozCnw_7game_ai13position_eval16position_eval_at"]
    fn my_position_eval_at(out: *mut u8, a: i64, b: *const u8, c: *const u8, d: i64, e: i64, f: u8);
    /// `position_risk_all_zero_near(i64, ptr, ptr, ptr, i8) -> i1` — exe `0xd8ca70`(2021명령 : IR 3337줄)
    /// ★dn 캐시(LAST_STAND_MEMO) 접근자 — `(key: ptr8, env: ptr32) -> i24`.
    ///   nexus_final_stand / base_defense_focus 가 이걸 통해 플래그 3개를 읽고, 그 결과가
    ///   interaction_score 의 **최초 거부 게이트**(IR %201 / %212)를 여닫는다.
    #[link_name = "_RINvMs2_NtNtCs9ec1k27omRZ_3std6thread5localINtB6_8LocalKeyINtNtCsjihNppCmMEE_4core4cell7RefCellTyjINtNtNtNtBa_11collections4hash3map7HashMapjTbbbENtNtCs9EYcZKFYzm_5ahash12random_state11RandomStateEEEE4withNCNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old13defense_nexus16last_stand_flags0B2f_EB3r_"]
    fn my_dn_with(key: *const u8, env: *const u8) -> u32;
    /// 주소 스캔 앵커용 — 본문에서 `lea rcx,[rip+X]; call my_dn_with` 를 찾아 **내** LocalKey 를 얻는다.
    #[link_name = "_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old13defense_nexus17nexus_final_stand"]
    fn my_nexus_final_stand(a: *const u8, b: *const u8, c: *const u8, d: *const u8) -> bool;
    /// dn 플래그 [1] 의 정체 — `(ptr2528, ptr24) -> bool`. exe 0xd3fe50.
    #[link_name = "_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old13defense_nexus26nexus_final_stand_uncached"]
    fn my_nfs_uncached(a: *const u8, b: *const u8) -> bool;
    /// `Effect::is_in_range(&self 56B, ptr1728, ptr1728) -> bool` — exe 0x1285320.
    ///   nexus_final_stand_uncached 의 유일한 실제 피호출자(IR 기준 5회 호출이 exe 에선 1곳으로 병합).
    #[link_name = "_RNvMNtNtCs97f5S1uJLkH_9game_core10simulation6effectNtB2_6Effect11is_in_range"]
    fn my_is_in_range(s: *const u8, b: *const u8, c: *const u8) -> bool;
    #[link_name = "_RNvNtCshdEBA0ozCnw_7game_ai13position_eval27position_risk_all_zero_near"]
    fn my_pos_risk_zero(a: i64, b: *const u8, c: *const u8, d: *const u8, e: u8) -> bool;
}

/// 0xd851d0 — gen_fns 의 AS_D851D0 과 같은 RVA 지만 여기서 독립 채록(capstone: push 12B 완결).
pub const PEU: super::FnSpec = super::FnSpec { name: "pe_uncached", sym: r"game_ai::position_eval::position_eval_at_uncached", role: "helper", rva: 0xd851d0, size: 0, prolog: &[0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x53], status: "bisect" };

/// 0xd8ca70 은 gen_fns 에 없어 여기서 직접 채록(capstone 확인: push 12B 로 완결).
pub const PRZ: super::FnSpec = super::FnSpec { name: "pos_risk_zero", sym: r"game_ai::position_eval::position_risk_all_zero_near", role: "helper", rva: 0xd8ca70, size: 0, prolog: &[0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x53], status: "bisect" };

/// `action_score::interaction_score` — exe RVA 는 `MIG\\name2rva.py` 로 해석(action_score.rs 구간 ∩ aimap 패닉줄).
///   seq_trace 의 첫 불일치가 이 함수의 `possible_risk(threshold=9999)` 호출지와 battle_common 의 `90` 호출지 사이에서 갈린다.
pub const ISC: super::FnSpec = super::FnSpec { name: "interaction_score", sym: r"game_ai::action_score::interaction_score", role: "helper", rva: 0xd57540, size: 0, prolog: &[0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x55, 0x53], status: "bisect" };
/// `action_score::calculate_interaction_action_score` = 기존 `COMBAT_SCORE`(exe 0xd5bbf0, 19,586B).
///   `interaction_score` 의 최대 콜리 — 잔차가 본문인지 이 콜리인지 가르는 표적.
#[allow(dead_code)]
pub const CIAS: super::FnSpec = super::FnSpec { name: "calc_interaction", sym: r"game_ai::action_score::calculate_interaction_action_score", role: "helper", rva: 0xd5bbf0, size: 0, prolog: &[0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x53], status: "bisect" };
/// `SmallActionPlay::evaluation_position` — exe `0xe23170`(small_action 모듈 729B, 패닉 246줄).
///   ⚠푸시가 7B 뿐이라 `sub rsp,0x80`(7B)까지 포함해 **14B** 프롬로그를 쓴다(install_wrap_bytes 는 12..=32 허용).
pub const EPOS: super::FnSpec = super::FnSpec { name: "eval_position", sym: r"game_ai::small_action::SmallActionPlay::evaluation_position", role: "helper", rva: 0xe23170, size: 0, prolog: &[0x41, 0x57, 0x41, 0x56, 0x56, 0x57, 0x53, 0x48, 0x81, 0xec, 0x80, 0x00, 0x00, 0x00], status: "bisect" };
/// `Effect::expected_damage_target` — exe `0x12857f0`(= 기존 EST_DAMAGE). 전 AI 공용 헬퍼라 판당 1억 호출 이상.
///   그래서 **`interaction_score` 안에서 불렸을 때만** 대조한다(IN_IS 게이트) — 안 그러면 리플레이가 끝없이 느려진다.
pub const EDMG: super::FnSpec = super::FnSpec { name: "exp_damage", sym: r"game_core::simulation::effect::Effect::expected_damage_target", role: "helper", rva: 0x12857f0, size: 0, prolog: &[0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x53], status: "bisect" };
/// HP 메모 캐시 접근자(exe `0xc87fe0`). 프롬로그 = push ×8 + sub rsp,0x38 (12B 넘게 확보).
pub const HPC: super::FnSpec = super::FnSpec { name: "hp_cache", sym: r"LocalKey::with<champion_hp_value>", role: "helper", rva: 0xc87fe0, size: 0, prolog: &[0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x53], status: "bisect" };
/// `defense_nexus::nexus_final_stand_uncached` — exe 0xd3fe50 (defense_nexus.rs 패닉 줄 241).
///   dn 캐시 플래그 3개 중 **가운데 것**을 만드는 함수. 프롬로그 = push ×8 (12B 정확).
pub const NFSU: super::FnSpec = super::FnSpec { name: "nexus_final_stand", sym: r"game_ai::plan_legacy::old::defense_nexus::nexus_final_stand_uncached", role: "helper", rva: 0xd3fe50, size: 0, prolog: &[0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x55, 0x53], status: "bisect" };
/// `game_core::simulation::effect::Effect::is_in_range` — exe 0x1285320.
///   확인 근거 ②: interaction_score 에서 expected_damage_target(0x12857f0) **직전**에 호출(IR 순서와 일치).
///   확인 근거 ②: nexus_final_stand_uncached(0xd3fe50) 의 유일한 실제 call 대상.
pub const IIR: super::FnSpec = super::FnSpec { name: "is_in_range", sym: r"game_core::simulation::effect::Effect::is_in_range", role: "helper", rva: 0x1285320, size: 0, prolog: &[0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x55, 0x53], status: "bisect" };
pub struct Slot { pub name: &'static str, pub orig: AtomicUsize, pub n: AtomicUsize, pub diff: AtomicUsize }
macro_rules! slot { ($n:expr) => { Slot { name: $n, orig: AtomicUsize::new(0), n: AtomicUsize::new(0), diff: AtomicUsize::new(0) } } }
pub static S: [Slot; 16] = [slot!("possible_risk"), slot!("mw_risk"), slot!("champion_hp_value"), slot!("siege_stance"), slot!("tower_dive"), slot!("position_eval_at"), slot!("pos_risk_zero"), slot!("pe_uncached"), slot!("interaction_score"), slot!("calc_interaction"), slot!("eval_position"), slot!("exp_damage"), slot!("hp_cache"), slot!("dn_cache"), slot!("nexus_final_stand"), slot!("is_in_range")];
const PR: usize = 0; const MW: usize = 1; const HP: usize = 2; const SS: usize = 3; const TD: usize = 4; const PE: usize = 5; const PZ: usize = 6; const PU: usize = 7; const IS: usize = 8; const CI: usize = 9; const EP: usize = 10; const ED: usize = 11; const HC: usize = 12; const DNC: usize = 13; const NFS: usize = 14; const IIRX: usize = 15;
static LOG: Mutex<Vec<String>> = Mutex::new(Vec::new());
/// ★입력 안정성 계측 — 게임 호출 직전과 내 사본 호출 직후에 **인자가 가리키는 게임 상태**를 비교한다.
///   시뮬은 rayon 워커에서 돌아 다른 스레드가 그 사이에 상태를 바꾸면 **두 호출의 입력이 애초에 다르다**.
///   그러면 "같은 인자인데 결과가 다르다"는 진단 자체가 헛된다.
/// ★마지막 인자 `dbg`(DebugFrameData 224B)는 **가변**이다 — 게임 호출이 거기에 쓴다.
///   하네스는 게임→내사본 순서라 **내 사본은 게임이 방금 쓴 dbg 를 본다**. 지금까지 놓친 비대칭.
///   게임 함수를 같은 인자로 연달아 두 번 불러, 2회차가 다르면 그 변형이 결과에 먹힌다는 뜻.
pub static G2_CMP: AtomicUsize = AtomicUsize::new(0);
pub static G2_DIFF: AtomicUsize = AtomicUsize::new(0);
pub static DBG_MOVED: AtomicUsize = AtomicUsize::new(0);
pub static ARG_CMP: AtomicUsize = AtomicUsize::new(0);
pub static ARG_MOVED: AtomicUsize = AtomicUsize::new(0);
pub static ARG_MOVED_DIFF: AtomicUsize = AtomicUsize::new(0);
/// ★재진입 가드 — 게임의 **내부 재귀 호출**도 이 훅을 탄다. 그때마다 사본을 또 돌리면
///   사본은 같은 계산을 두 번 하게 되고(바깥 미러 안에서 한 번, 재귀 미러로 또 한 번)
///   그만큼 **사본의 TLS 캐시가 게임과 다르게 채워진다**. 미러는 최상위 1회만.
///   (실측 2026-09-09: 이 가드 없이 position_eval_at 이 6,839만 콜 중 355건 불일치)
thread_local! { static DEPTH: std::cell::Cell<u32> = std::cell::Cell::new(0); }
#[allow(dead_code)]
thread_local! { static DEPTH_CI: std::cell::Cell<u32> = std::cell::Cell::new(0); }
/// ★`interaction_score` 의 **게임 원본**이 도는 동안만 1 — 공용 헬퍼를 그 구간에서만 대조하려고.
thread_local! { static IN_IS: std::cell::Cell<u32> = std::cell::Cell::new(0); }
thread_local! { static DEPTH_H: std::cell::Cell<u32> = std::cell::Cell::new(0); }
thread_local! { static DEPTH_H2: std::cell::Cell<u32> = std::cell::Cell::new(0); }
#[inline] fn enter() -> bool { DEPTH.with(|d| { let v = d.get(); d.set(v + 1); v == 0 }) }
#[inline] fn leave() { DEPTH.with(|d| d.set(d.get().saturating_sub(1))); }

fn note(i: usize, s: String) {
    S[i].diff.fetch_add(1, Ordering::Relaxed);
    let mut g = LOG.lock().unwrap_or_else(|e| e.into_inner());
    if g.len() < 40 { g.push(s); }
}
/// 인자가 가리키는 읽기전용 구조체 4개를 한 버퍼로 복사(2528 + 24 + 5384 + 184).
const ARG_SNAP: usize = 2528 + 24 + 5384 + 184;
unsafe fn arg_copy(out: *mut u8, c: *const u8, d: *const u8, e: *const u8, f: *const u8) {
    core::ptr::copy_nonoverlapping(c, out, 2528);
    core::ptr::copy_nonoverlapping(d, out.add(2528), 24);
    core::ptr::copy_nonoverlapping(e, out.add(2528 + 24), 5384);
    core::ptr::copy_nonoverlapping(f, out.add(2528 + 24 + 5384), 184);
}
fn hex(b: &[u8]) -> String { b.iter().map(|x| format!("{:02x}", x)).collect() }
#[inline] fn o(i: usize) -> usize { S[i].orig.load(Ordering::Relaxed) }

unsafe fn wrap_pr(a: *const u8, b: *const u8, c: i64) -> i64 {
    let f: unsafe fn(*const u8, *const u8, i64) -> i64 = core::mem::transmute(o(PR));
    let top = enter();
    let g = f(a, b, c);
    if !top { leave(); return g; }
    S[PR].n.fetch_add(1, Ordering::Relaxed);
    if let Ok(m) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| my_possible_risk(a, b, c))) {
        if m != g { note(PR, format!("possible_risk g={} m={} (c={})", g, m, c)); }
    }
    leave();
    g
}
unsafe fn wrap_mw(a: i64, b: *const u8, c: *const u8, d: i64, e: i64, ff: i64) -> i64 {
    let f: unsafe fn(i64, *const u8, *const u8, i64, i64, i64) -> i64 = core::mem::transmute(o(MW));
    let top = enter();
    let g = f(a, b, c, d, e, ff);
    if !top { leave(); return g; }
    S[MW].n.fetch_add(1, Ordering::Relaxed);
    if let Ok(m) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| my_mw_risk(a, b, c, d, e, ff))) {
        if m != g { note(MW, format!("mw_risk g={} m={} (a={} d={} e={} f={})", g, m, a, d, e, ff)); }
    }
    leave();
    g
}
/// ★`interaction_score` — 사본을 돌리면 **게임의 rng 가 또 소모된다**. 320B 를 떠서 돌렸다가 원상복시킨다.
///   (마지막 인자 DebugFrameData 도 가변이라 사본이 같은 버퍼에 또 미는데, 디버그 표시용이라 결정에 안 쓰인다.)
unsafe fn wrap_is(a: i64, rng: *mut u8, c: *const u8, d: *const u8, e: *const u8, f: *const u8, dbg: *mut u8) -> i64 {
    let fp: unsafe fn(i64, *mut u8, *const u8, *const u8, *const u8, *const u8, *mut u8) -> i64 = core::mem::transmute(o(IS));
    let top = enter();
    let mut s0 = [0u8; 320];
    if top { core::ptr::copy_nonoverlapping(rng, s0.as_mut_ptr(), 320); }
    // ★입력 안정성 표본(8회에 1번) — 게임 호출 전 상태를 떠 둔다.
    let sample = top && S[IS].n.load(Ordering::Relaxed) % 8 == 0;
    let mut snap = if sample { Some(Box::new([0u8; ARG_SNAP])) } else { None };
    if let Some(b) = snap.as_mut() { arg_copy(b.as_mut_ptr(), c, d, e, f); }
    let mut dbg0 = [0u8; 224];
    if top { core::ptr::copy_nonoverlapping(dbg, dbg0.as_mut_ptr(), 224); }
    IN_IS.with(|x| x.set(x.get() + 1));
    // ★게임 호출 구간을 seq_trace 에 담는다 — 내부 호출열을 받아 적어둠다.
    if top { super::seq_trace::begin_game(); }
    let g = fp(a, rng, c, d, e, f, dbg);
    if top { super::seq_trace::end(); }
    IN_IS.with(|x| x.set(x.get().saturating_sub(1)));
    if !top { leave(); return g; }
    // ★게임 함수를 바로 한 번 더 — 이번엔 dbg 가 이미 변형된 상태다(내 사본이 보는 것과 같은 조건).
    {
        if core::ptr::read_unaligned(dbg as *const [u8; 224]) != dbg0 { DBG_MOVED.fetch_add(1, Ordering::Relaxed); }
        let mut s1 = [0u8; 320];
        core::ptr::copy_nonoverlapping(rng, s1.as_mut_ptr(), 320);   // 게임 1회차 후 rng
        core::ptr::copy_nonoverlapping(s0.as_ptr(), rng, 320);       // 되돌리고
        IN_IS.with(|x| x.set(x.get() + 1));
        let g2 = fp(a, rng, c, d, e, f, dbg);
        IN_IS.with(|x| x.set(x.get().saturating_sub(1)));
        core::ptr::copy_nonoverlapping(s1.as_ptr(), rng, 320);       // 게임 기준 복원
        G2_CMP.fetch_add(1, Ordering::Relaxed);
        if g2 != g {
            G2_DIFF.fetch_add(1, Ordering::Relaxed);
            note(IS, format!("★게임 재호출이 다름 g1={} g2={} (dbg 변형 후) a={}", g, g2, a));
        }
    }
    let mut after = [0u8; 320];
    core::ptr::copy_nonoverlapping(rng, after.as_mut_ptr(), 320);
    core::ptr::copy_nonoverlapping(s0.as_ptr(), rng, 320);
    S[IS].n.fetch_add(1, Ordering::Relaxed);
    super::seq_trace::begin_mine();
    let m = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| my_interaction_score(a, rng, c, d, e, f, dbg)));
    super::seq_trace::end();
    let mut mine_rng = [0u8; 320];
    core::ptr::copy_nonoverlapping(rng, mine_rng.as_mut_ptr(), 320);
    core::ptr::copy_nonoverlapping(after.as_ptr(), rng, 320);   // 게임 기준으로 복원
    if let Ok(m) = m {
        // rng 비교는 패딩 8B(+0x108..0x110) 제외 — new 의 memcpy 가 스택 쓰레기까지 복사한다.
        let rs = mine_rng[..0x108] == after[..0x108] && mine_rng[0x110..] == after[0x110..];
        // ★IR 패치(`MIG\\patches.json`)로 내 사본의 거부 반환에 **분기별 고유 센티넬**을 심어둔다.
        //   -9999901..-9999907 → 원래 값으로 되돌려 비교하고, 번호는 로그에 남긴다.
        //   1=%448(753줄) 2=%493(774) 3=%285(700) 4=%161(647) 5=%201(661) 6=%212(667) 7=%1113(2158)
        let (mn, br) = if (-9999907..=-9999901).contains(&m) {
            let b = (-9999900 - m) as u8;
            (if b <= 2 { -9999999i64 } else { -99999i64 }, b)
        } else { (m, 0u8) };
        let bad = mn != g || !rs;
        if let Some(b) = snap.as_ref() {
            let mut now = Box::new([0u8; ARG_SNAP]);
            arg_copy(now.as_mut_ptr(), c, d, e, f);
            ARG_CMP.fetch_add(1, Ordering::Relaxed);
            if now.as_ref() != b.as_ref() {
                ARG_MOVED.fetch_add(1, Ordering::Relaxed);
                if bad { ARG_MOVED_DIFF.fetch_add(1, Ordering::Relaxed); }
            }
        }
        if bad {
            note(IS, format!("interaction_score g={} m={} (내거부분기 #{}) rng_same={} (a={})", g, mn, br, rs, a));
            // ★반환이 갈린 그 호출의 **내부 호출열**을 대조한다.
            //   호출열이 완전히 같은데 답만 다르면 원인은 interaction_score 자신의 산술/분기다.
            //   다르면 첫 차이 함수가 곧 갈라진 게이트가 읽는 값이다.
            super::seq_trace::compare_is(g, mn);
        }
    }
    leave();
    g
}
/// ★공용 헬퍼는 `interaction_score` 안에서 불렸을 때만 대조한다 — 전수 대조하면 판당 1억 호출이라 리플레이가 못 끝난다.
#[inline]
fn helper_top() -> bool {
    if IN_IS.with(|d| d.get()) == 0 { return false; }
    DEPTH_H.with(|d| { let v = d.get(); d.set(v + 1); v == 0 })
}
#[inline]
fn helper_leave() { DEPTH_H.with(|d| d.set(d.get().saturating_sub(1))); }

unsafe fn wrap_ep(out: *mut u8, s: *const u8, a: i64, b: *const u8, c: *const u8) {
    let fp: unsafe fn(*mut u8, *const u8, i64, *const u8, *const u8) = core::mem::transmute(o(EP));
    let top = helper_top();
    fp(out, s, a, b, c);
    if !top { if IN_IS.with(|d| d.get()) != 0 { helper_leave(); } return; }
    let mut og = [0u8; 24];
    core::ptr::copy_nonoverlapping(out, og.as_mut_ptr(), 24);
    S[EP].n.fetch_add(1, Ordering::Relaxed);
    let mut om = [0u8; 24];
    core::ptr::copy_nonoverlapping(out, om.as_mut_ptr(), 24);   // 호출 전 내용 대신 게임 결과로 채워 미초기화 차이 제거
    if std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| my_eval_pos(om.as_mut_ptr(), s, a, b, c))).is_ok() {
        if om != og { note(EP, format!("eval_position g={} m={} (a={})", hex(&og), hex(&om), a)); }
    }
    core::ptr::copy_nonoverlapping(og.as_ptr(), out, 24);        // 게임 결과 복원
    helper_leave();
}
unsafe fn wrap_ed(s: *const u8, b: *const u8, c: *const u8, d: *const u8, e: *const u8) -> i64 {
    let fp: unsafe fn(*const u8, *const u8, *const u8, *const u8, *const u8) -> i64 = core::mem::transmute(o(ED));
    let top = helper_top();
    let g = fp(s, b, c, d, e);
    if !top { if IN_IS.with(|x| x.get()) != 0 { helper_leave(); } return g; }
    S[ED].n.fetch_add(1, Ordering::Relaxed);
    if let Ok(m) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| my_exp_dmg(s, b, c, d, e))) {
        if m != g { note(ED, format!("exp_damage g={} m={}", g, m)); }
    }
    helper_leave();
    g
}
/// ★내 사본의 HP 캐시 **LocalKey 정적 주소**를 런타임에 찾는다.
///   그 정적은 IR 에서 `@anon....` 익명 상수라 링크할 심볼이 없다.
///   `champion_hp_value` 본문을 돌며 `48 8d 0d <disp32>`(lea rcx,[rip+d]) 중 **마지막 call 직전**의 것을 잡는다.
static MY_HP_KEY: AtomicUsize = AtomicUsize::new(0);
unsafe fn find_my_hp_key() -> usize {
    let cur = MY_HP_KEY.load(Ordering::Relaxed);
    if cur != 0 { return cur; }
    let f = my_hp_value as usize;
    let mut last_lea = 0usize;
    let mut i = 0usize;
    while i < 400 {
        let p = (f + i) as *const u8;
        if *p == 0x48 && *p.add(1) == 0x8d && *p.add(2) == 0x0d {
            let d = core::ptr::read_unaligned(p.add(3) as *const i32) as i64;
            last_lea = (f + i + 7).wrapping_add(d as usize);
            i += 7;
            continue;
        }
        if *p == 0xe8 && last_lea != 0 {              // call rel32 — 직전 lea 가 LocalKey
            MY_HP_KEY.store(last_lea, Ordering::Relaxed);
            return last_lea;
        }
        i += 1;
    }
    0
}
/// exe 의 HP 캐시 접근자를 미러링 — 게임이 캐시를 만질 때마다 **내 캐시도 같은 시점·같은 키로** 채운다.
///   ⚠게임의 LocalKey 를 그대로 넘기면 내 값이 게임 캐시를 오염시킨다 — 반드시 **내 LocalKey** 로 부른다.
unsafe fn wrap_hpc(key: *const u8, env: *const u8) -> i64 {
    let fp: unsafe fn(*const u8, *const u8) -> i64 = core::mem::transmute(o(HC));
    let top = helper_top2();
    let g = fp(key, env);
    if !top { helper_leave2(); return g; }
    let mk = find_my_hp_key();
    if mk != 0 {
        S[HC].n.fetch_add(1, Ordering::Relaxed);
        if let Ok(m) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| my_hp_with(mk as *const u8, env))) {
            if m != g { note(HC, format!("hp_cache g={} m={}", g, m)); }
        }
    }
    helper_leave2();
    g
}
/// 내 DLL 의 `LAST_STAND_MEMO` LocalKey static 주소 — nexus_final_stand 본문에서
/// **my_dn_with 로 가는 call 을 찾아 그 직전 `lea rcx,[rip+X]`** 를 읽는다.
/// (hp 쪽처럼 "첫 call 앞 lea" 로 잡으면 다른 static 을 짚을 수 있어 대상을 명시한다.)
/// dn 캐시 불일치의 **방향별** 카운터 — 비트별로 게임만 1 / 내 사본만 1 을 각각 센다.
/// 지금까지 "항상 게임=true" 는 로그 40줄 기준이라 전체가 그런지 확인해야 한다.
pub static DN_G1: [AtomicUsize; 3] = [const { AtomicUsize::new(0) }; 3];
pub static DN_M1: [AtomicUsize; 3] = [const { AtomicUsize::new(0) }; 3];
/// ★nexus_final_stand 의 **양쪽 true 비율** — 두 함수가 "같은 모양"인지 판별한다.
///   불리언은 대부분 false 면 **서로 다른 함수끼리도 99.8% 일치**한다 — DIFF 율만으로는 판별이 안 된다.
///   모양이 같으면 true 비율이 비슷해야 하고, exe 에만 추가 조건이 있으면 게임 쪽이 일관되게 높다.
static NFS_DUMP: AtomicUsize = AtomicUsize::new(0);
/// ★게임 **재호출** 통제 — 게임을 연속 두 번 부른다.
///   시뮬은 rayon 워커에서 병렬로 돌고 이 함수는 **가변 공유 상태**(미니언 좌표)를 읽는다.
///   하네스는 게임→내사본 순서라, 그 사이 다른 스레드가 상태를 바꾸면
///   "재현이 틀린 것"이 아니라 **시점이 다른 것**이다. g1!=g2 가 나오면 그게 증거다.
/// ★exe 기준 대체 구현 vs 게임 - 이게 0 이 되어야 재현이 맞은 것이다.
/// 내 구현이 그 답을 낸 이유. 0=nexus null 1=게이트 2=미니언·타워 모두 없음 3=미니언 4=타워
pub static NFS_WHY: [AtomicUsize; 5] = [const { AtomicUsize::new(0) }; 5];
pub static NFS_WHY_ALL: [AtomicUsize; 5] = [const { AtomicUsize::new(0) }; 5];
/// ★게이트 무시본 - 디스어셈 해석과 실측이 어긋날 때 어느 쪽이 맞는지 가른다.
/// 타워 판정 두 방식 대조: rlib is_in_range vs exe 인라인 공식 직접구현
pub static TWR_CMP: AtomicUsize = AtomicUsize::new(0);
pub static TWR_DIFF: AtomicUsize = AtomicUsize::new(0);
/// 잔차 분해: [0]=게이트가 켜져 있었나(=게임이 게이트로 막았을 것) [1]=아니었나
pub static NG_GATED: [AtomicUsize; 2] = [const { AtomicUsize::new(0) }; 2];
/// 잔차를 낸 타워 인덱스 0..4
pub static NG_TOWER: [AtomicUsize; 5] = [const { AtomicUsize::new(0) }; 5];
pub static NFS_NG_WHY: [AtomicUsize; 5] = [const { AtomicUsize::new(0) }; 5];
pub static NFS_NOGATE_CMP: AtomicUsize = AtomicUsize::new(0);
pub static NFS_NOGATE_DIFF: AtomicUsize = AtomicUsize::new(0);
pub static NFS_NOGATE_GT: AtomicUsize = AtomicUsize::new(0);
pub static NFS_NOGATE_MT: AtomicUsize = AtomicUsize::new(0);
pub static NFS_EXE_CMP: AtomicUsize = AtomicUsize::new(0);
pub static NFS_EXE_DIFF: AtomicUsize = AtomicUsize::new(0);
pub static NFS_EXE_GT: AtomicUsize = AtomicUsize::new(0);   // 게임true/내false
pub static NFS_EXE_MT: AtomicUsize = AtomicUsize::new(0);   // 내true/게임false
pub static NFS_G2_CMP: AtomicUsize = AtomicUsize::new(0);
pub static NFS_G2_DIFF: AtomicUsize = AtomicUsize::new(0);

/// 모델 검증 집계 — [0]=예측일치 [1]=예측true·실제false(★모델오류) [2]=예측false·실제true [3]=판독불가
pub static NFS_PRED: [AtomicUsize; 4] = [const { AtomicUsize::new(0) }; 4];
thread_local! { static NFS_PRED_V: std::cell::Cell<Option<bool>> = std::cell::Cell::new(None); }
/// ★예측 vs **내 사본** — 모델이 내 사본과 같은지. 같다면 "게임만 다르다" 가 확정된다.
pub static NFS_PM: [AtomicUsize; 2] = [const { AtomicUsize::new(0) }; 2];   // [0]=같음 [1]=다름


pub static NFS_G_TRUE: AtomicUsize = AtomicUsize::new(0);
pub static NFS_M_TRUE: AtomicUsize = AtomicUsize::new(0);
static MY_DN_KEY: AtomicUsize = AtomicUsize::new(0);
unsafe fn find_my_dn_key() -> usize {
    let cur = MY_DN_KEY.load(Ordering::Relaxed);
    if cur != 0 { return cur; }
    let f = my_nexus_final_stand as usize;
    let tgt = my_dn_with as usize;
    let mut last_lea = 0usize;
    let mut i = 0usize;
    while i < 2048 {
        let p = (f + i) as *const u8;
        if *p == 0x48 && *p.add(1) == 0x8d && *p.add(2) == 0x0d {
            let d = core::ptr::read_unaligned(p.add(3) as *const i32) as i64;
            last_lea = (f + i + 7).wrapping_add(d as usize);
            i += 7; continue;
        }
        if *p == 0xe8 {
            let d = core::ptr::read_unaligned(p.add(1) as *const i32) as i64;
            let dst = (f + i + 5).wrapping_add(d as usize);
            if dst == tgt && last_lea != 0 { MY_DN_KEY.store(last_lea, Ordering::Relaxed); return last_lea; }
            i += 5; continue;
        }
        i += 1;
    }
    0
}
/// dn 캐시 접근자 대조 — 게임 캐시 vs 내 캐시. 반환은 i24 이므로 하위 24비트만 본다.
unsafe fn wrap_dnc(key: *const u8, env: *const u8) -> u32 {
    let fp: unsafe fn(*const u8, *const u8) -> u32 = core::mem::transmute(o(DNC));
    let top = helper_top2();
    let g = fp(key, env);
    if !top { helper_leave2(); return g; }
    let mk = find_my_dn_key();
    if mk != 0 {
        S[DNC].n.fetch_add(1, Ordering::Relaxed);
        if let Ok(m) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| my_dn_with(mk as *const u8, env))) {
            if (m & 0xff_ffff) != (g & 0xff_ffff) {
                for k in 0..3usize {
                    let (gb, mb) = ((g >> (k * 8)) & 1, (m >> (k * 8)) & 1);
                    if gb == 1 && mb == 0 { DN_G1[k].fetch_add(1, Ordering::Relaxed); }
                    if gb == 0 && mb == 1 { DN_M1[k].fetch_add(1, Ordering::Relaxed); }
                }
                note(DNC, format!("dn_cache g={:#08x} m={:#08x} (flags g={:?} m={:?})", g & 0xff_ffff, m & 0xff_ffff,
                    [g & 1, (g >> 8) & 1, (g >> 16) & 1], [m & 1, (m >> 8) & 1, (m >> 16) & 1]));
            }
        }
    }
    helper_leave2();
    g
}
/// dn 플래그 [1] 을 만드는 함수 자체 대조 — 캐시 위가 아니라 **계산 자체**가 갈리는지 본다.
thread_local! { static IN_NFS: std::cell::Cell<u32> = std::cell::Cell::new(0); }
/// ⚠is_in_range 는 전 AI 공용이라 판당 수억 호출이다.
///   **nexus_final_stand_uncached 안에서 불렸을 때만** 대조한다(IN_NFS 게이트).
unsafe fn wrap_iir(s: *const u8, b: *const u8, c: *const u8) -> bool {
    let fp: unsafe fn(*const u8, *const u8, *const u8) -> bool = core::mem::transmute(o(IIRX));
    let g = fp(s, b, c);
    if IN_NFS.with(|x| x.get()) == 0 { return g; }
    S[IIRX].n.fetch_add(1, Ordering::Relaxed);
    if let Ok(m) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| my_is_in_range(s, b, c))) {
        if m != g { note(IIRX, format!("is_in_range g={} m={}", g, m)); }
    }
    g
}

/// ★`nexus_final_stand_uncached` 의 **exe 기준** 대체 구현 (SDK rlib 은 술어가 다르다).
///   exe 0xd3fe50 디스어셈 그대로. 미니언 판정만 rlib 과 다르고 타워 경로는 동일하다.
///   - rlib : dx*dx+dy*dy < 14_400_000_001            ("반경 120,000 안인가")
///   - exe  : e[+0x68]==1 && e[+0x88]==1 && e[+0x90]==nexus[+0x5c0]   ("내 넥서스를 공격 대상으로 잡았는가")

/// ★타워 사거리 판정 - exe 인라인본. **Ghidra 디컴파일 C 기준**(손 필사로 두 번 틀린 뒤 교체).
///   ⚠`/100` 은 부호 있는 나눗셈 **하나**다. 어셈의 `shr rax,2`+`mul magic`+`shr rdx,2` 를
///     "4로 나누고 100으로 나눔" 으로 읽은 것이 오류였다(잔차 38 -> 199 로 악화).
///   ⚠타워 5개 중 **0~3 만** 이 공식이고 4번째는 독립 `is_in_range` 호출이다.
unsafe fn tower_in_range(tw: usize, nexus: usize) -> Option<bool> {
    #[inline(always)]
    fn ptrish(v: usize) -> bool { v >= 0x10000 && v < (1usize << 48) }
    #[inline(always)]
    unsafe fn r64(p: usize) -> i64 { core::ptr::read_unaligned(p as *const i64) }
    #[inline(always)]
    unsafe fn r32(p: usize) -> i32 { core::ptr::read_unaligned(p as *const i32) }
    /// `*(p+0x470)` 가 0 이면 사거리 원본, 아니면 (a+100)*사거리/100  (전부 i64 부호 연산)
    #[inline(always)]
    unsafe fn scale(p: usize) -> i64 {
        let a = r32(p + 0x470) as i64;
        let base = r64(p + 0x680);
        if a == 0 { base } else { (a + 100).wrapping_mul(base) / 100 }
    }

    let vt = r64(tw + 0x498) as usize;
    if !ptrish(vt) { return None; }
    let a1 = ((r64(vt + 0x10) as usize).wrapping_sub(1) & !0xfusize)
        .wrapping_add(r64(tw + 0x490) as usize)
        .wrapping_add(0x10);
    let fp = r64(vt + 0xe8) as usize;
    if !ptrish(fp) || !ptrish(a1) { return None; }
    // ⚠Ghidra 는 이 가상호출의 **3번째 인자를 놓쳤다**(2개로 보여준다). 어셈이 정본이다:
    //     mov rcx,a1 ; mov rdx,rdi(tower) ; mov r8,rsi(nexus) ; call qword [r9+0xe8]
    //   2개로 부르면 r8 이 쓰레기라 콜리가 그걸 역참조해 AV 로 죽는다(2026-09-10 실사고).
    //   ★디컴파일러는 **로직**에 정확하고, **ABI/레지스터**는 디스어셈이 정본이다.
    let f: unsafe extern "system" fn(usize, usize, usize) -> i64 = core::mem::transmute(fp);
    let v = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(a1, tw, nexus))) {
        Ok(x) => x,
        Err(_) => return None,
    };
    let flag = r32(tw + 0x4c0);
    let s_t = if flag == 0 { scale(tw) } else { 0 };
    let s_n = scale(nexus);
    let rng = r64(tw + 0x438)
        .wrapping_add(r64(tw + 0x4a0))
        .wrapping_add(r64(tw + 0x5c8).wrapping_sub(1).wrapping_mul(r64(tw + 0x4a8)))
        .wrapping_add(s_t)
        .wrapping_add(s_n)
        .wrapping_add(v);
    // 좌표차 절댓값은 **부호 없는** 비교로 만든다(디컴파일 그대로).
    let (tx, ty) = (r64(tw + 0x660) as u64, r64(tw + 0x668) as u64);
    let (nx, ny) = (r64(nexus + 0x660) as u64, r64(nexus + 0x668) as u64);
    let dx = (if nx < tx { tx - nx } else { nx - tx }) as i64;
    let dy = (if ny < ty { ty - ny } else { ny - ty }) as i64;
    let d2 = dy.wrapping_mul(dy).wrapping_add(dx.wrapping_mul(dx)) as u64;
    Some(d2 <= rng.wrapping_mul(rng) as u64)
}
unsafe fn nfs_exe(ctx: *const u8, arg2: *const u8) -> Option<(bool, bool, u8)> {
    #[inline(always)]
    fn ptrish(v: usize) -> bool { v >= 0x10000 && v < (1usize << 48) }
    #[inline(always)]
    unsafe fn r64(p: usize) -> u64 { core::ptr::read_unaligned(p as *const u64) }
    #[inline(always)]
    unsafe fn r32(p: usize) -> i32 { core::ptr::read_unaligned(p as *const i32) }

    let c = ctx as usize;
    if !ptrish(c) || !ptrish(arg2 as usize) { return None; }
    let team = r64(c + 0x930) as usize;
    if team > 1 { return None; }                       // 게임은 여기서 panic - 판정 제외
    let gm = r64(arg2 as usize) as usize;
    if !ptrish(gm) { return None; }
    let nexus = r64(gm + 0x170 + team * 8) as usize;
    if !ptrish(nexus) { return Some((false, false, 0)); }   // nexus null -> false
    // ★★게이트의 정체 (2026-09-10 확정 — 디스어셈과 실측이 어긋난 진짜 이유)
    //   `gm + side*0x20 + 0x148` = **살아있는 쌍둥이 타워 수**(nexus_emg O_TWIN_LEN 과 같은 필드).
    //   원본 게이트는 `== 0` 이어야 진행(= "쌍둥이가 하나도 안 남았을 때만 최후방어").
    //   ⚠그런데 **이 모드의 nexus_emg 디투어가 바로 이 사이트(NXE_RVA = 0xd3fe88)를 패치**해
    //     실행 이미지의 게이트를 `nxe_level(reg, side) > 0` 으로 바꿔 둔다.
    //     ⟹ 정적 디스어셈(원본)과 인게임 실측(패치본)이 영원히 어긋난 것 —
    //        "서로 다른 바이너리를 본 것"(DONE.md 교훈, S7 게이트와 같은 함정 재발).
    //   근거: cfg 에 `nxe_twin1=80`·`nxe_t2_3=60`(기본 0) 이 들어 있어 `need`=true →
    //         `nxe.txt` 가 "설치① 비상 판정 : OK" 를 찍는다(2026-09-10 09:38 실측 판).
    //   ⟹ 실행 이미지와 같게 판정하려면 `nxe_gate()` 를 써야 한다(dn_reach.rs 는 09-06 부터 이미 그랬다).
    let twin_len = r64(gm + team * 32 + 0x148);
    let gated = !crate::nxe_gate(gm, team as u64, twin_len);

    let foe = 1 - team;
    let base = gm + foe * 32;
    let nid = r64(nexus + 0x5c0);
    for (po, lo) in [(0x10usize, 0x28usize), (0x50, 0x68), (0x90, 0xa8)] {
        let p = r64(base + po) as usize;
        let n = r64(base + lo) as usize;
        if !ptrish(p) || n > 4096 { continue; }
        for k in 0..n {
            let e = r64(p + k * 8) as usize;
            if !ptrish(e) { continue; }
            if r32(e + 0x68) != 1 { continue; }
            if r32(e + 0x88) != 1 { continue; }
            if r64(e + 0x90) != nid { continue; }
            return Some((true, gated, 3));
        }
    }
    // ★세 변형의 실측치(2026-09-10) — 위에서 밝힌 "패치된 게이트" 로 전부 설명된다:
    //     원본 게이트 적용(twin==0) DIFF 2,096~2,194 (게임true/내false)
    //        = 게임은 nxe_twin1/nxe_t2_3 로 **추가 발동**하는데 나는 원본 조건만 봐서 false 를 냈다.
    //     적용 안 함                DIFF    74~143   (내true/게임false)
    //        = nxe_level==0 인 상황(쌍둥이 2기 생존 && 2차 3개 미만 파괴)에서 게임은 false 인데 내가 true.
    //   ⟹ 옳은 모델은 둘 다 아니고 **nxe_gate()**(위) 이다.
    //   ⚠"잔차 74건이 100% 게이트 켜짐" 을 인과로 읽었던 것은 여전히 오판이다 —
    //     게이트는 **전체 호출의 97.8%** 에서 켜져 있어 어떤 부분집합도 거의 전부 켜짐이다(기저율 효과).
    // ★★"타워 경로" 가 아니다 — **적 팀 챔피언 로스터 5명**이다(2026-09-10 정정).
    //   `gm + 0x1e0 + side*40 + role*8` = layout.rs `X_ROSTER`(0x1e0) · ROSTER_SIDE_STRIDE 40 · role 0~4(포지션).
    //   `+0x4c0` = layout.rs `ENT_SLOT0_FLAG`(= ENT_SLOT0 + 0x30) = **슬롯0(평타/스킬) flag, -1 = 슬롯 없음**.
    //   `+0x490` = `ENT_SLOT0` 자체 → `is_in_range(slot0, champ, nexus)` = "그 챔피언의 슬롯0 사거리가 넥서스에 닿는가".
    //   ⟹ 이전 기록의 "타워 5개 / 타워 인덱스 0·2 에 몰림" 은 전부 **로스터 role 인덱스**로 읽어야 한다.
    // ★사거리는 rlib `is_in_range` 를 쓴다(실측으로 그게 가장 정확했다). 인라인 공식을 디컴파일 기준으로
    //   직접 구현했더니 오히려 나빠졌다(잔차 0~143 -> 876) — 항이 많아(0x438/0x4a0/0x4a8/0x5c8/0x470/0x680
    //   + 이펙트 vt+0xe8 가상호출) 필사 오류를 못 잡았다.
    //   ⟹ 완전 재구현본이 필요하면 **이미 있다**: `port/dn_reach.rs::in_reach`(eff_e8 구현체 4종까지 재현).
    for k in 0..5usize {
        let tw = r64(gm + 0x1e0 + foe * 40 + k * 8) as usize;
        if !ptrish(tw) { continue; }
        if r32(tw + 0x4c0) == -1 { continue; }
        if my_is_in_range((tw + 0x490) as *const u8, tw as *const u8, nexus as *const u8) {
            // 어느 role 이 true 를 냈는지 실어 보낸다(10+role).
            return Some((true, gated, (10 + k) as u8));
        }
    }
    Some((false, gated, 2))
}
unsafe fn wrap_nfs(a: *const u8, b: *const u8) -> bool {
    let fp: unsafe fn(*const u8, *const u8) -> bool = core::mem::transmute(o(NFS));
    IN_NFS.with(|x| x.set(x.get() + 1));
    // ★게이트를 **게임 호출 직전**에 떠 둔다 — 덤프는 두 호출이 끝난 뒤라
    //   그 사이 다른 rayon 워커가 상태를 바꾸면 사후 판독이 호출 시점과 다르다.
    let pre = {
        let rd = |p: usize| -> i64 { if p >= 0x10000 && readable(p, 8) { core::ptr::read_unaligned(p as *const i64) } else { i64::MIN } };
        let tm = rd(a as usize + 2352);
        let gmp = rd(b as usize);
        if (0..2).contains(&tm) && gmp >= 0x10000 { (tm, rd(gmp as usize + tm as usize * 32 + 328)) } else { (tm, i64::MIN) }
    };
    let g = fp(a, b);
    // ★게임을 곧바로 한 번 더 — 내 사본이 보는 것과 같은 "나중 시점" 조건이다.
    let g2 = fp(a, b);
    NFS_G2_CMP.fetch_add(1, Ordering::Relaxed);
    if g2 != g { NFS_G2_DIFF.fetch_add(1, Ordering::Relaxed); }
    S[NFS].n.fetch_add(1, Ordering::Relaxed);
    {   // ★exe 기준 대체 구현 대조 - 이게 0 이면 재현 성공이다.
        if let Ok(Some((raw, gated, why))) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| nfs_exe(a, b))) {
            let e = raw && !gated;      // 게이트 적용본(디스어셈 해석 그대로)
            let e2 = raw;               // 게이트 무시본
            NFS_NOGATE_CMP.fetch_add(1, Ordering::Relaxed);
            if e2 != g {
                NFS_NOGATE_DIFF.fetch_add(1, Ordering::Relaxed);
                if g { NFS_NOGATE_GT.fetch_add(1, Ordering::Relaxed); } else { NFS_NOGATE_MT.fetch_add(1, Ordering::Relaxed); }
                // ★게이트무시본이 그 답을 낸 경로 - 미니언(3)인지 타워(4)인지 갈린다.
                NFS_NG_WHY[if why >= 10 { 4 } else { (why as usize).min(4) }].fetch_add(1, Ordering::Relaxed);
                if why >= 10 {
                    NG_TOWER[(why as usize - 10).min(4)].fetch_add(1, Ordering::Relaxed);
                    NG_GATED[(!gated) as usize].fetch_add(1, Ordering::Relaxed);
                }
            }
            NFS_EXE_CMP.fetch_add(1, Ordering::Relaxed);
            NFS_WHY_ALL[(why as usize).min(4)].fetch_add(1, Ordering::Relaxed);
            if e != g {
                NFS_EXE_DIFF.fetch_add(1, Ordering::Relaxed);
                if g { NFS_EXE_GT.fetch_add(1, Ordering::Relaxed); } else { NFS_EXE_MT.fetch_add(1, Ordering::Relaxed); }
                // ★내가 그 답을 낸 **이유**를 센다 - 어느 분기가 게임과 갈리는지 바로 나온다.
                NFS_WHY[(why as usize).min(4)].fetch_add(1, Ordering::Relaxed);
            }
        }
    }
    {   // ★모델 검증 — exe 로직을 그대로 재현한 예측과 실제 g 를 대조한다.
        let rd = |p: usize| -> Option<i64> {
            if p >= 0x10000 && readable(p, 8) { Some(core::ptr::read_unaligned(p as *const i64)) } else { None }
        };
        let mut pred: Option<bool> = None;
        if let (Some(tm), Some(gmi)) = (rd(a as usize + 2352), rd(b as usize)) {
            let gm = gmi as usize;
            if (0..2).contains(&tm) && gm >= 0x10000 && readable(gm, 8840) {
                let nexus = rd(gm + 368 + tm as usize * 8).unwrap_or(0) as usize;
                if nexus < 0x10000 { pred = Some(false); }
                else if rd(gm + tm as usize * 32 + 328).unwrap_or(1) != 0 { pred = Some(false); }
                else if readable(nexus + 1632, 16) {
                    let nx = core::ptr::read_unaligned((nexus + 1632) as *const i64);
                    let ny = core::ptr::read_unaligned((nexus + 1640) as *const i64);
                    let foe = 1 - tm as usize;
                    let mut hit = false;
                    for base in [16usize, 80, 144] {
                        let s = gm + base + foe * 32;
                        let p = rd(s).unwrap_or(0) as usize;
                        let n = rd(s + 24).unwrap_or(-1);
                        if p < 0x10000 || !(0..=2048).contains(&n) { continue; }
                        let n = n as usize;
                        if !readable(p, n * 8) { continue; }
                        for k in 0..n {
                            let e = core::ptr::read_unaligned((p + k * 8) as *const usize);
                            if e < 0x10000 || !readable(e + 1632, 16) { continue; }
                            let mx = core::ptr::read_unaligned((e + 1632) as *const i64);
                            let my = core::ptr::read_unaligned((e + 1640) as *const i64);
                            let dx = (mx - nx).unsigned_abs();
                            let dy = (my - ny).unsigned_abs();
                            if dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy)) < 14_400_000_001u64 { hit = true; break; }
                        }
                        if hit { break; }
                    }
                    // ⚠타워 5개 is_in_range 경로는 재현 못 한다 → 예측 true 만 강한 주장이다.
                    pred = Some(hit);
                }
            }
        }
        NFS_PRED_V.with(|c| c.set(pred));   // ★내 사본(m)과도 대조하려고 남긴다
        match pred {
            None => { NFS_PRED[3].fetch_add(1, Ordering::Relaxed); }
            Some(p) if p == g => { NFS_PRED[0].fetch_add(1, Ordering::Relaxed); }
            Some(true) => { NFS_PRED[1].fetch_add(1, Ordering::Relaxed); }
            Some(false) => { NFS_PRED[2].fetch_add(1, Ordering::Relaxed); }
        }
    }
    if g { NFS_G_TRUE.fetch_add(1, Ordering::Relaxed); }
    if let Ok(m) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| my_nfs_uncached(a, b))) {
        if m { NFS_M_TRUE.fetch_add(1, Ordering::Relaxed); }
        if let Some(p) = NFS_PRED_V.with(|c| c.get()) {
            NFS_PM[(p != m) as usize].fetch_add(1, Ordering::Relaxed);
        }
        if m != g {
            S[NFS].diff.fetch_add(1, Ordering::Relaxed);
            // ★갈린 순간의 중간값을 래퍼에서 **직접 재현**해 찍는다(IR m04.ll 구조 그대로).
            //   %5=*(ctx+2352) 팀 · %8=*(arg2) 게임(8840B) · 넥서스=*(%8+368+%5*8)
            //   게이트=*(%8+%5*40+328) 은 0 이어야 진행 · 적 미니언 = 슬라이스 3개(%8+16/+80/+144, stride 40, len@+24)
            //   술어 = dx*dx+dy*dy < 14_400_000_001 (좌표 entity +1632/+1640)
            // ⚠원시 포인터는 **전부 readable() 로 가드**한다 — 가드 없이 읽었다가 AV 로 게임이 죽었다(2026-09-10).
            if NFS_DUMP.fetch_add(1, Ordering::Relaxed) < 40 {   // 상세 덤프만 제한(카운터는 위에서 이미 셌다)
                let rd64 = |p: usize| -> Option<i64> {
                    if p >= 0x10000 && readable(p, 8) { Some(core::ptr::read_unaligned(p as *const i64)) } else { None }
                };
                let team = rd64(a as usize + 2352).unwrap_or(-1);
                let gm = rd64(b as usize).unwrap_or(0) as usize;
                let mut gate = -1i64;
                let mut nexus_ok = false;
                let mut best = u64::MAX;
                let mut cnt = 0usize;
                if (0..2).contains(&team) && gm >= 0x10000 && readable(gm, 8840) {
                    // ⚠stride 는 32 다 — `{ { ptr, ptr, i64 }, i64 }` = 8+8+8+8.
                    //   40 으로 읽었더니 team=1 에서 gm+368(넥서스 포인터 배열)을 짚어 값이 포인터로 나왔다.
                    gate = rd64(gm + team as usize * 32 + 328).unwrap_or(-1);
                    let nexus = rd64(gm + 368 + team as usize * 8).unwrap_or(0) as usize;
                    if nexus >= 0x10000 && readable(nexus, 1648) {
                        nexus_ok = true;
                        let nx = core::ptr::read_unaligned((nexus + 1632) as *const i64);
                        let ny = core::ptr::read_unaligned((nexus + 1640) as *const i64);
                        let foe = 1 - team as usize;
                        for base in [16usize, 80, 144] {
                            let s = gm + base + foe * 32;
                            let p = rd64(s).unwrap_or(0) as usize;
                            let n = rd64(s + 24).unwrap_or(0);
                            if p < 0x10000 || !(0..=2048).contains(&n) { continue; }
                            let n = n as usize;
                            if !readable(p, n * 8) { continue; }
                            for k in 0..n {
                                let e = core::ptr::read_unaligned((p + k * 8) as *const usize);
                                if e < 0x10000 || !readable(e + 1632, 16) { continue; }
                                cnt += 1;
                                let mx = core::ptr::read_unaligned((e + 1632) as *const i64);
                                let my = core::ptr::read_unaligned((e + 1640) as *const i64);
                                let dx = (mx - nx).unsigned_abs();
                                let dy = (my - ny).unsigned_abs();
                                let d2 = dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy));
                                if d2 < best { best = d2; }
                            }
                        }
                    }
                }
                note(NFS, format!("nfs g={} m={} | pre(team={} gate={}) post(team={} gate={}) nexus={} 적미니언수={} 최근거리제곱={} 미니언판정={}",
                    g, m, pre.0, pre.1,
                    team, gate, nexus_ok, cnt,
                    if best == u64::MAX { -1i64 } else { best as i64 }, best < 14_400_000_001u64));
            }
        }
    }
    IN_NFS.with(|x| x.set(x.get().saturating_sub(1)));
    g
}
#[inline]
fn helper_top2() -> bool { DEPTH_H2.with(|d| { let v = d.get(); d.set(v + 1); v == 0 }) }
#[inline]
fn helper_leave2() { DEPTH_H2.with(|d| d.set(d.get().saturating_sub(1))); }

unsafe fn wrap_hp(a: *const u8, b: *const u8, c: *const u8) -> i64 {
    let f: unsafe fn(*const u8, *const u8, *const u8) -> i64 = core::mem::transmute(o(HP));
    let g = f(a, b, c);
    S[HP].n.fetch_add(1, Ordering::Relaxed);
    if let Ok(m) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| my_champion_hp_value(a, b, c))) {
        if m != g { note(HP, format!("champion_hp_value g={} m={}", g, m)); }
    }
    g
}
unsafe fn wrap_ss(a: i64, b: *const u8, c: *const u8, d: *const u8) -> (i64, i64) {
    let f: unsafe fn(i64, *const u8, *const u8, *const u8) -> (i64, i64) = core::mem::transmute(o(SS));
    let top = enter();
    let g = f(a, b, c, d);
    if !top { leave(); return g; }
    S[SS].n.fetch_add(1, Ordering::Relaxed);
    if let Ok(m) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| my_siege_stance(a, b, c, d))) {
        // ★반환은 `Option<usize>`(16B) = (태그, 값). **None 이면 값은 미초기화 쓰레기**라 양쪽이 다르다
        //   (실측 2026-09-09: 97% 가 이것). 태그가 같고 Some 일 때만 값을 비교한다.
        let bad = m.0 != g.0 || (g.0 != 0 && m.1 != g.1);
        if bad { note(SS, format!("siege_stance g={:?} m={:?} (a={})", g, m, a)); }
    }
    leave();
    g
}
unsafe fn wrap_td(a: i64, b: *const u8, c: *const u8, d: *const u8, e: *const u8, ff: *const u8, gg: bool, h: *const u8) -> bool {
    let f: unsafe fn(i64, *const u8, *const u8, *const u8, *const u8, *const u8, bool, *const u8) -> bool = core::mem::transmute(o(TD));
    let g = f(a, b, c, d, e, ff, gg, h);
    S[TD].n.fetch_add(1, Ordering::Relaxed);
    if let Ok(m) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| my_tower_dive(a, b, c, d, e, ff, gg, h))) {
        // ⚠bool 반환은 rax 상위 바이트에 쓰레기가 남는다(CLAUDE.md 교훈) — 비트 0 만 본다.
        if (m as u8 & 1) != (g as u8 & 1) { note(TD, format!("tower_dive g={} m={} (a={} bound={})", g, m, a, gg)); }
    }
    g
}

unsafe fn wrap_pe(out: *mut u8, a: i64, b: *const u8, c: *const u8, d: i64, e: i64, ff: u8) {
    let f: unsafe fn(*mut u8, i64, *const u8, *const u8, i64, i64, u8) = core::mem::transmute(o(PE));
    // ★sret 버퍼는 호출 전 내용이 남아 있다(미사용 바이트가 쓰레기) → 내 쪽 버퍼를 게임 버퍼의 **호출 전 내용**으로 채운다.
    let mut om = [0u8; 56];
    core::ptr::copy_nonoverlapping(out, om.as_mut_ptr(), 56);
    let top = enter();
    f(out, a, b, c, d, e, ff);
    if !top { leave(); return; }
    S[PE].n.fetch_add(1, Ordering::Relaxed);
    if std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| my_position_eval_at(om.as_mut_ptr(), a, b, c, d, e, ff))).is_ok() {
        // ★`PositioningScore` 56B = risk/tower_risk/gain/gain_me/adjust/unseen_champ_threat(i64×6, +0x00~0x30)
        //   + on_trajectory/on_periodic_trajectory(bool×2, +0x30/+0x31) + **패딩 6B(+0x32~0x38)**.
        //   패딩은 양쪽 스택 쓰레기라 항상 다르다(실측 2026-09-09: 100% DIFF 가 전부 이 6바이트) → 0x32 까지만 본다.
        let og = std::slice::from_raw_parts(out, 56);
        if og[..0x32] != om[..0x32] { note(PE, format!("position_eval_at
      g={}
      m={}", hex(&og[..0x32]), hex(&om[..0x32]))); }
    }
    leave();
}
unsafe fn wrap_pz(a: i64, b: *const u8, c: *const u8, d: *const u8, e: u8) -> bool {
    let f: unsafe fn(i64, *const u8, *const u8, *const u8, u8) -> bool = core::mem::transmute(o(PZ));
    let top = enter();
    let g = f(a, b, c, d, e);
    if !top { leave(); return g; }
    S[PZ].n.fetch_add(1, Ordering::Relaxed);
    if let Ok(m) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| my_pos_risk_zero(a, b, c, d, e))) {
        if (m as u8 & 1) != (g as u8 & 1) { note(PZ, format!("pos_risk_zero g={} m={} (a={} e={})", g, m, a, e)); }
    }
    leave();
    g
}

unsafe fn wrap_pu(out: *mut u8, a: i64, b: *const u8, c: *const u8, d: i64, e: i64, ff: u8) {
    let f: unsafe fn(*mut u8, i64, *const u8, *const u8, i64, i64, u8) = core::mem::transmute(o(PU));
    let mut om = [0u8; 56];
    core::ptr::copy_nonoverlapping(out, om.as_mut_ptr(), 56);
    f(out, a, b, c, d, e, ff);
    S[PU].n.fetch_add(1, Ordering::Relaxed);
    let _ = (om, d, e, ff);   // pe_uncached 는 exe LTO 로 내부화돼 링크 불가 — 슬롯만 남긴다
}

/// cfg `fn_bisect` = **비트마스크**(bit0 possible_risk · bit1 mw_risk · bit2 champion_hp_value · bit3 siege_stance · bit4 tower_dive · bit5 position_eval_at · bit6 pos_risk_zero · bit7 pe_uncached).
/// ⚠하나씩 켤 것 — exe 는 LTO 로 내부 함수의 인자 규약을 바꿀 수 있어, IR 시그니처를 그대로 믿고 5개를 한꺼번에 걸었더니
///   즉시 접근위반으로 죽었다(2026-09-09 실측 c0000005 @ MOD+0x23cd9c).
///   `mw_risk`(6인자)만 judge 손포팅이 같은 인자 배치로 DIFF=0 을 냈으므로 규약이 확인된 상태다.
///   `champion_hp_value` 는 exe `0xc87fe0` 이 **래퍼(pct_c, 2인자)** 라 3인자 원본과 규약이 다르다 — 기본 제외.
/// ★진단: HP 메모 캐시를 **게임 쪽에서** 끔다.
///   접근자 `0xc87fe0` 의 에포크 비교 분기(`0xc88035: 75 18 jne`)를 `EB 18 jmp` 로 바꾸면
///   매 호출 에포크가 바뀐 것으로 취급되어 맵이 비워지고 → **항상 미스**.
///   내 사본은 `MIG/patches.json` 의 IR 패치(`br i1 false`)로 같이 끔다.
///   ⚠게임 AI 가 바뀜다(바닐라 아니다). 다만 트윈은 같은 판 안에서 대조하므로 비교는 유효하다.
unsafe fn game_cache_off(log: &mut String) {
    let base = crate::exe_base();
    if base == 0 { return; }
    let p = base + 0xc88035;
    let mut old: u32 = 0;
    if VP(p, 2, 0x40, &mut old) == 0 { log.push_str("[cache_off] VirtualProtect 실패\n"); return; }
    let cur = *(p as *const u8);
    if cur == 0x75 {
        *(p as *mut u8) = 0xEB;
        log.push_str("[cache_off] 게임 HP 캐시 OFF (0xc88035: 75 -> EB)\n");
    } else if cur == 0xEB {
        log.push_str("[cache_off] 이미 OFF\n");
    } else {
        log.push_str(&format!("[cache_off] 예상 바이트 아님(0x{:02x}) — 건드리지 않음\n", cur));
    }
    VP(p, 2, old, &mut old);
}
extern "system" { #[link_name = "VirtualProtect"] fn VP(a: usize, s: usize, p: u32, o: *mut u32) -> i32; }

pub unsafe fn install(log: &mut String) {
    if tune("judge_cache_off", 0) != 0 { game_cache_off(log); }
    let mask = tune("fn_bisect", 0);
    if mask == 0 { return; }
    let t: [(usize, &super::FnSpec, usize); 15] = [
        (PR, &super::AS_D83230, wrap_pr as usize),
        (MW, &super::MW_RISK, wrap_mw as usize),
        (HP, &super::UTIL_C87FE0, wrap_hp as usize),
        (SS, &super::AS_D96D00, wrap_ss as usize),
        (TD, &super::TOWER_DIVE, wrap_td as usize),
        (PE, &super::AS_D84DB0, wrap_pe as usize),
        (PZ, &PRZ, wrap_pz as usize),
        (PU, &PEU, wrap_pu as usize),
        (IS, &ISC, wrap_is as usize),
        (EP, &EPOS, wrap_ep as usize),
        (ED, &EDMG, wrap_ed as usize),
        (HC, &HPC, wrap_hpc as usize),
        (DNC, &super::DN_CACHE, wrap_dnc as usize),
        (NFS, &NFSU, wrap_nfs as usize),
        (IIRX, &IIR, wrap_iir as usize),
    ];
    for (i, spec, w) in t {
        if mask & (1 << i) == 0 { continue; }
        match super::hook::install_wrap_bytes(spec.rva, spec.prolog, w) {
            Ok(orig) => { S[i].orig.store(orig, Ordering::Relaxed); log.push_str(&format!("[bisect] {} OK @rva {:#x}\n", S[i].name, spec.rva)); }
            Err(e) => log.push_str(&format!("[bisect] {} 실패: {} @rva {:#x}\n", S[i].name, e, spec.rva)),
        }
    }
}

pub fn report() -> String {
    let mut s = String::from("[bisect] 게임 함수 vs 내 링크 사본 (같은 인자·같은 시점)\n");
    s.push_str(&format!("  [게임재호출] 표본={} · 1회↔2회 다름={} · dbg 가 변형됨={}\n",
        G2_CMP.load(Ordering::Relaxed), G2_DIFF.load(Ordering::Relaxed), DBG_MOVED.load(Ordering::Relaxed)));
    {
        let (n, gt, mt) = (S[NFS].n.load(Ordering::Relaxed), NFS_G_TRUE.load(Ordering::Relaxed), NFS_M_TRUE.load(Ordering::Relaxed));
        {
            let (c, dd) = (NFS_EXE_CMP.load(Ordering::Relaxed), NFS_EXE_DIFF.load(Ordering::Relaxed));
            if c > 0 {
                {
                    let (cc, dd) = (NFS_NOGATE_CMP.load(Ordering::Relaxed), NFS_NOGATE_DIFF.load(Ordering::Relaxed));
                    {
                        let (tc, td) = (TWR_CMP.load(Ordering::Relaxed), TWR_DIFF.load(Ordering::Relaxed));
                        if tc > 0 { s.push_str(&format!("  [타워 두방식] 표본 {} · 서로 다름 {} ({:.4}%)
", tc, td, 100.0 * td as f64 / tc as f64)); }
                    }
                    {
                        let gt: Vec<usize> = (0..2).map(|k| NG_GATED[k].load(Ordering::Relaxed)).collect();
                        let tw: Vec<usize> = (0..5).map(|k| NG_TOWER[k].load(Ordering::Relaxed)).collect();
                        if gt[0] + gt[1] > 0 { s.push_str(&format!("  [잔차 분해] 게이트켜짐 {} / 꺼짐 {} · 타워인덱스 {:?}
", gt[0], gt[1], tw)); }
                    }
                    let ng: Vec<usize> = (0..5).map(|k| NFS_NG_WHY[k].load(Ordering::Relaxed)).collect();
                    if cc > 0 { s.push_str(&format!("  [nfs 무시본 경로] {:?}  (2=없음 3=미니언 4=타워)
", ng)); }
                    if cc > 0 { s.push_str(&format!("  [nfs 게이트무시] 표본 {} · DIFF {} ({:.5}%) · 게임true/내false {} · 내true/게임false {}
",
                        cc, dd, 100.0 * dd as f64 / cc as f64, NFS_NOGATE_GT.load(Ordering::Relaxed), NFS_NOGATE_MT.load(Ordering::Relaxed))); }
                }
                let w: Vec<usize> = (0..5).map(|k| NFS_WHY[k].load(Ordering::Relaxed)).collect();
                let wa: Vec<usize> = (0..5).map(|k| NFS_WHY_ALL[k].load(Ordering::Relaxed)).collect();
                s.push_str(&format!("  [nfs 이유] DIFF때 {:?} · 전체 {:?}  (0=nexus_null 1=gate 2=없음 3=미니언 4=타워)
", w, wa));
                s.push_str(&format!("  [nfs exe재현] 표본 {} · DIFF {} ({:.5}%) · 게임true/내false {} · 내true/게임false {}\n",
                    c, dd, 100.0 * dd as f64 / c as f64,
                    NFS_EXE_GT.load(Ordering::Relaxed), NFS_EXE_MT.load(Ordering::Relaxed)));
            }
        }
        {
            let (c, dd) = (NFS_G2_CMP.load(Ordering::Relaxed), NFS_G2_DIFF.load(Ordering::Relaxed));
            if c > 0 { s.push_str(&format!("  [nfs 게임재호출] 표본 {} · 1회<>2회 다름 {} ({:.5}%)
", c, dd, 100.0 * dd as f64 / c as f64)); }
        }
        let pv: Vec<usize> = (0..4).map(|k| NFS_PRED[k].load(Ordering::Relaxed)).collect();
        {
            let (sm, df) = (NFS_PM[0].load(Ordering::Relaxed), NFS_PM[1].load(Ordering::Relaxed));
            if sm + df > 0 { s.push_str(&format!("  [nfs 예측vs내사본] 같음 {} ({:.3}%) · 다름 {}
", sm, 100.0 * sm as f64 / (sm + df) as f64, df)); }
        }
        if pv.iter().any(|v| *v > 0) {
            let tot = pv[0] + pv[1] + pv[2];
            s.push_str(&format!("  [nfs 모델검증] 예측일치 {} ({:.2}%) · 예측true·실제false {} ★ · 예측false·실제true {} · 판독불가 {}\n",
                pv[0], 100.0 * pv[0] as f64 / tot.max(1) as f64, pv[1], pv[2], pv[3]));
        }
        if n > 0 {
            s.push_str(&format!("  [nfs true비율] 게임 {}/{} ({:.4}%) · 내사본 {}/{} ({:.4}%)
",
                gt, n, 100.0 * gt as f64 / n as f64, mt, n, 100.0 * mt as f64 / n as f64));
        }
    }
    {
        let (a, b) = ((0..3).map(|k| DN_G1[k].load(Ordering::Relaxed)).collect::<Vec<_>>(),
                      (0..3).map(|k| DN_M1[k].load(Ordering::Relaxed)).collect::<Vec<_>>());
        if a.iter().chain(b.iter()).any(|v| *v > 0) {
            s.push_str(&format!("  [dn플래그] 게임만1 {:?} · 내사본만1 {:?}  (순서: last_stand / final_stand / base_attacking)\n", a, b));
        }
    }
    s.push_str(&format!("  [입력안정성] 표본={} · 두 호출 사이에 인자가 바뀜={} · 그중 DIFF 난 것={}\n",
        ARG_CMP.load(Ordering::Relaxed), ARG_MOVED.load(Ordering::Relaxed), ARG_MOVED_DIFF.load(Ordering::Relaxed)));
    for x in S.iter() {
        let (n, d) = (x.n.load(Ordering::Relaxed), x.diff.load(Ordering::Relaxed));
        s.push_str(&format!("  {:<20} n={:>12} DIFF={:>8} ({:.5}%)\n", x.name, n, d, 100.0 * d as f64 / n.max(1) as f64));
    }
    let g = LOG.lock().unwrap_or_else(|e| e.into_inner());
    for l in g.iter() { s.push_str("   "); s.push_str(l); s.push('\n'); }
    s
}
