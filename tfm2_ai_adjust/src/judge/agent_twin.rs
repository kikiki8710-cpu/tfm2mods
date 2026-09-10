//! agent_twin — **mode 3: 같은 프로세스 안에서 게임 에이전트 ↔ 내 사본 에이전트 쌍(twin) 비교**.
//!
//! 왜 필요한가(2026-09-09 실측): 리플레이 재시뮬은 프로세스마다 다르다 — 같은 리플레이를 바닐라로 세 번 돌리면
//!   경기 길이가 25,741 / 25,723 / 19,223 틱으로 갈린다(HashMap RandomState·ahash 등 프로세스별 난수). 그래서
//!   "판 전체 다이제스트를 두 프로세스에서 비교" 는 원리적으로 DIFF=0 이 나올 수 없다.
//!   대신 **한 프로세스에서 같은 입력을 두 에이전트에 주고 출력을 비트 대조**한다 — judge 계층과 같은 철학을 에이전트 경계에 적용.
//!
//! 방법:
//!   · 팩토리: rng(StdRng 320B, 힙 없음)를 스냅샷 → 게임 에이전트 생성 → rng 복원 → 내 에이전트 생성 → 두 rng 결과 상태 비교 → 게임 쪽 상태로 되돌림.
//!   · 게임 vtable(0x33ebba0)의 상태 변경 슬롯을 shim 으로 교체: 0 drop · 11 set_version · 14 get_input · 15 buy_item · 16 upgrade_item ·
//!     17 update_on_dead · 20 push_game_event · 21 push_pending_trace_event. 각 shim 은 게임 에이전트에 원본을 실행한 뒤 **같은 인자**로
//!     트윈에도 실행한다. rnd 를 받는 메서드는 호출 전 rng 를 스냅샷하고, 게임 호출 후 상태를 보관 → 스냅샷 복원 → 내 호출 → 두 결과 상태 비교 → 게임 쪽 상태로 복원.
//!   · get_input/update_on_dead 는 `data.blackboard`(&[Blackboard;2] = 744×2B, 힙 없음)도 같은 방식으로 스냅샷·복원(AI 가 팀 블랙보드에 쓴다).
//!   · 비교 대상: get_input 출력 64B 중 `Option<Input>` 의 결정 워드 [0..24) + 이벤트 Vec 의 len(+0x38) — +24 와 +0x28 은 포인터(실행마다 다름).
//!     buy_item 반환 {i64,i64} · upgrade_item sret 24B · 그리고 모든 rnd 메서드 후의 **rng 상태 320B** · 블랙보드 1488B.
//!   · presim 클론(`__clone_box`)은 트윈이 없으므로 원본만 실행(비교 제외).
//! ⚠ push_* 이벤트 인자는 `dead_on_return`(값 이동)이지만 이동이 memcpy(32B/184B)라 원본 바이트가 남는다 — 힙 필드가 없다는 전제(IR 실측 32B/184B 고정 크기).
use crate::*;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

pub const RNG_SIZE: usize = 320;
pub const BB_SIZE: usize = 744 * 2;
const S_DROP: usize = 0; const S_CLONE: usize = 4; const S_SETVER: usize = 11; const S_GI: usize = 14; const S_BUY: usize = 15; const S_UPG: usize = 16;
const S_DEAD: usize = 17; const S_PGE: usize = 20; const S_PPTE: usize = 21;
// ★AiAgent vtable 40슬롯의 정체 = `_gaibc/m14.ll` 의 vtable 상수
//   `@anon.b0108feec1ab8ff62b7a37c1a95c251f.18` 에서 순서대로 읽은 것(0=drop, 1=size, 2=align, 3부터 메서드).
//   이 중 **상태를 바꾸는** 것은 모두 미러링해야 트윈이 안 샜다.
//   drain 둘은 이름 그대로 에이전트의 Vec 을 **비워서 가져간다** — 게임만 비우면 트윈은 계속 쌓인다.
const S_CHATS: usize = 22;      // plan_chats_drain(sret Vec<Chat> 24B, &mut self)
const S_PTED: usize = 23;       // plan_pending_trace_events_drain(같은 모양)
const OD_BB: usize = 0x10;   // OperationData.blackboard: &[Blackboard;2]

type FactoryFn = unsafe fn(*mut u8, usize, usize, u32) -> (usize, usize);
type F1 = unsafe fn(*mut u8);
type F1r = unsafe fn(*mut u8) -> *mut u8;
type F2i = unsafe fn(*mut u8, i64);
type F2p = unsafe fn(*mut u8, *mut u8);
type F3ip = unsafe fn(*mut u8, i64, *mut u8);
type F4 = unsafe fn(*mut u8, *mut u8, *mut u8, *mut u8);
type F5 = unsafe fn(*mut u8, *mut u8, *mut u8, *mut u8, *mut u8);
type F6r = unsafe fn(*mut u8, *mut u8, *mut u8, *mut u8, *mut u8, *mut u8) -> (usize, usize);
type F7 = unsafe fn(*mut u8, *mut u8, *mut u8, *mut u8, *mut u8, *mut u8, *mut u8);

static TWIN: Mutex<Option<HashMap<usize, usize>>> = Mutex::new(None);
static MY_VT: AtomicUsize = AtomicUsize::new(0);
static G_VT: AtomicUsize = AtomicUsize::new(0);
static ORIG: [AtomicUsize; 40] = [const { AtomicUsize::new(0) }; 40];
pub static F_CALLS: AtomicUsize = AtomicUsize::new(0);
pub static F_RNG_DIFF: AtomicUsize = AtomicUsize::new(0);
pub static F_FAIL: AtomicUsize = AtomicUsize::new(0);
pub static GI_CMP: AtomicUsize = AtomicUsize::new(0);
pub static GI_DIFF: AtomicUsize = AtomicUsize::new(0);
pub static GI_RNG_DIFF: AtomicUsize = AtomicUsize::new(0);
pub static GI_BB_DIFF: AtomicUsize = AtomicUsize::new(0);
pub static GI_DEC_ONLY: AtomicUsize = AtomicUsize::new(0);
pub static GI_RNG_ONLY: AtomicUsize = AtomicUsize::new(0);
pub static GI_DIFF_PRESIM: AtomicUsize = AtomicUsize::new(0);
pub static GI_DIFF_WORKER: AtomicUsize = AtomicUsize::new(0);
pub static GI_CMP_PRESIM: AtomicUsize = AtomicUsize::new(0);
pub static GI_CMP_WORKER: AtomicUsize = AtomicUsize::new(0);
static SIM_TID: AtomicUsize = AtomicUsize::new(0);
// ★자가검증: **게임 자신의 get_input 을 같은 상태로 두 번** 돌려본다(둘째는 게임 에이전트의 복제본에).
//   같은 입력·같은 rng 인데 소비량이 다르면, 그 함수는 (에이전트, rng) 만의 함수가 아니다
//   = 숨은 가변상태(TLS 메모 캐시 등)가 개입한다는 뜻 → 사본이 아무리 같아도 100% 는 불가능하다.
pub static M_CMP: AtomicUsize = AtomicUsize::new(0);
pub static M_AB: AtomicUsize = AtomicUsize::new(0);
pub static M_AC: AtomicUsize = AtomicUsize::new(0);
pub static M_AD: AtomicUsize = AtomicUsize::new(0);
pub static M_CC: AtomicUsize = AtomicUsize::new(0);
pub static M_TLS: AtomicUsize = AtomicUsize::new(0);
pub static G2_CMP: AtomicUsize = AtomicUsize::new(0);
pub static G2_IDX: AtomicUsize = AtomicUsize::new(0);
pub static G2_DEC: AtomicUsize = AtomicUsize::new(0);
pub static G2_RES: AtomicUsize = AtomicUsize::new(0);
/// ★**호출 후** 상태 비교 — 결정이 갈라지기 훨씬 전에 상태가 먼저 갈라진다.
///   지금까지는 결정 불일치(0.03%)만 트리거로 썼는데, 그건 **마지막 증상**이다.
///   진입 상태가 같았는데 퇴장 상태가 다른 첫 호출 = **발생 지점**이다.
pub static PS_CMP: AtomicUsize = AtomicUsize::new(0);
pub static PS_DIFF: AtomicUsize = AtomicUsize::new(0);
pub static PS_FIRST: AtomicUsize = AtomicUsize::new(0);
pub static PS_DEC_OK: AtomicUsize = AtomicUsize::new(0);
static SA_DUMPED: AtomicUsize = AtomicUsize::new(0);
pub static PS_FIELD: [AtomicUsize; 6] = [const { AtomicUsize::new(0) }; 6];
pub static ST_CMP: AtomicUsize = AtomicUsize::new(0);
pub static ST_RNG: AtomicUsize = AtomicUsize::new(0);     // static_rnd(+0x2d0, 320B) 가 호출 전부터 다름
pub static ST_SCAL: AtomicUsize = AtomicUsize::new(0);    // +0x2910..0x29d0 스칼라 필드가 다름
pub static ST_SA: AtomicUsize = AtomicUsize::new(0);
pub static ST_PLAN: AtomicUsize = AtomicUsize::new(0);   // plan_system(+0x530, 6168B) 이 호출 전부터 다름      // small_action(+0x2858, 184B) 이 다름
/// 게임 에이전트/트윈의 **POD 상태 영역**을 호출 전에 비교 — 갈라짐이 get_input 안에서 시작되는지, 그 전(미러 누락)부터인지 가른다.
///   AgentVerHamster 레이아웃(DI): static_rnd +0x2d0(320B) · small_action +0x2858(184B) · version +0x2910 … freeze_fired +0x29cb (힙 없음)
const K_RAW: u8 = 0; const K_VEC: u8 = 1; const K_OPT: u8 = 2; const K_PTR: u8 = 3;
/// LegacyPlanHandler(6168B) 필드 전개 — 출처 = `_gaibc/m07.ll` 의 DICompositeType "LegacyPlanHandler".
///   종류: K_RAW=그대로 / K_VEC·K_PTR=양쪽 다 주소인 워드를 빼고 비교 / K_OPT=태그 항상, 페이로드는 Some 때만.
pub static PLAN_F: [(usize, usize, u8, &str); 46] = [
    (0x0000,  248, K_PTR, "data"),
    (0x00f8, 1064, K_PTR, "team_plan"),
    (0x0520,   16, K_OPT, "battle_start_tick"),
    (0x0530,   24, K_OPT, "pending_global_ult_target"),
    (0x0548,   24, K_OPT, "v3_dest"),
    (0x0560,   16, K_OPT, "judge_noise_plan"),
    (0x0570,  120, K_PTR, "v50_dive_ep_live"),
    (0x05e8,  384, K_PTR, "plan"),
    (0x0768,   72, K_PTR, "sub_plan"),
    (0x07b0,   24, K_VEC, "misunderstood_chats"),
    (0x07c8,   24, K_VEC, "chats"),
    (0x07e0,   24, K_VEC, "chats_wait"),
    (0x07f8,   24, K_VEC, "received_chats"),
    (0x0810,   24, K_VEC, "gank_periods"),
    (0x0828,   24, K_VEC, "gank_score_attempts"),
    (0x0840,   24, K_VEC, "prev_plan_name"),
    (0x0858,   24, K_VEC, "pending_trace_events"),
    (0x0870,   24, K_VEC, "v46_recall_trigger_ticks"),
    (0x0888,   24, K_VEC, "v50_dive_episodes"),
    (0x08a0,   24, K_VEC, "v46_flee_episodes"),
    (0x08b8,   24, K_VEC, "flee_ring"),
    (0x08d0,   24, K_VEC, "flee_death_retrospects"),
    (0x08e8,   24, K_VEC, "v54_cj_call_ticks"),
    (0x0900,   24, K_VEC, "v54_reentry_ticks"),
    (0x0918,  112, K_OPT, "battle_start_state"),
    (0x0988,    8, K_RAW, "dm_death_latch"),
    (0x0990, 2760, K_PTR, "positioning_score"),
    (0x1458,   16, K_OPT, "requested_gank_line"),
    (0x1468,  400, K_RAW, "counters50"),
    (0x15f8,   16, K_PTR, "ff_battle_exit"),
    (0x1608,    8, K_RAW, "ff_exit_latch"),
    (0x1610,   16, K_PTR, "mf_swap"),
    (0x1620,   32, K_RAW, "v3_lapse4"),
    (0x1640,   24, K_PTR, "v3_home_ladder"),
    (0x1658,  256, K_PTR, "mf_ret25_src"),
    (0x1758,   32, K_PTR, "ff_line_counts"),
    (0x1778,   40, K_RAW, "ff_retreat5"),
    (0x17a0,   88, K_PTR, "judge_noise_ratio"),
    (0x17f8,    8, K_RAW, "dm_seen_deaths"),
    (0x1800,    2, K_OPT, "v2_obj_part"),
    (0x1802,    2, K_OPT, "v2_assign"),
    (0x1804,    8, K_RAW, "bools8"),
    (0x180c,    1, K_RAW, "v2_egowave_line"),
    (0x180d,    1, K_OPT, "active_gank_line"),
    (0x180e,    3, K_OPT, "v3_dest_obj"),
    (0x1811,    5, K_RAW, "tail_u8"),
];
pub static PLAN_D: [AtomicUsize; 46] = [const { AtomicUsize::new(0) }; 46];
#[inline]
fn ptrish(v: u64) -> bool { v >= 0x10000 && v < (1u64 << 48) }
/// 8B 단위 비교 — 양쪽 다 유효 주소 범위면 힙 포인터로 보고 건너뛴다(주소는 사본과 달라도 정상).
unsafe fn words_same(a: *const u8, b: *const u8, sz: usize) -> bool {
    let n = sz / 8;
    for k in 0..n {
        let (x, y) = (core::ptr::read_unaligned((a as *const u64).add(k)), core::ptr::read_unaligned((b as *const u64).add(k)));
        if x == y { continue; }
        if ptrish(x) && ptrish(y) { continue; }
        return false;
    }
    let rem = sz - n * 8;
    if rem != 0 && std::slice::from_raw_parts(a.add(n * 8), rem) != std::slice::from_raw_parts(b.add(n * 8), rem) { return false; }
    true
}
/// 한 필드가 같은가.
unsafe fn field_same(g: *mut u8, m: *mut u8, base: usize, o: usize, sz: usize, kind: u8) -> bool {
    let (a, b) = (g.add(base + o) as *const u8, m.add(base + o) as *const u8);
    match kind {
        K_RAW => std::slice::from_raw_parts(a, sz) == std::slice::from_raw_parts(b, sz),
        K_OPT => {
            if sz >= 16 {
                let (tg, tm) = (core::ptr::read_unaligned(a as *const u64), core::ptr::read_unaligned(b as *const u64));
                if tg != tm { return false; }
                if tg == 0 { return true; }
                words_same(a.add(8), b.add(8), sz - 8)
            } else {
                let (tg, tm) = (*a, *b);
                if tg != tm { return false; }
                if sz == 1 || tg == 0 { return true; }
                std::slice::from_raw_parts(a.add(1), sz - 1) == std::slice::from_raw_parts(b.add(1), sz - 1)
            }
        }
        _ => words_same(a, b, sz),
    }
}
/// 플랜 핸들러를 필드별로 비교해 **어느 필드가** 다른지 집계한다.
unsafe fn plan_field_check(g: *mut u8, m: *mut u8) {
    let mut bits = 0u64;
    for (i, &(o, sz, k, _)) in PLAN_F.iter().enumerate() {
        if !field_same(g, m, 0x530, o, sz, k) { PLAN_D[i].fetch_add(1, Ordering::Relaxed); bits |= 1 << i; }
    }
    if bits != 0 { ST_PLAN.fetch_add(1, Ordering::Relaxed); }
    PLAN_BITS.with(|c| c.set(bits));
}
// ★전체 집계는 "한 번 갈라진 쌍은 계속 다르다"라 인과를 못 가린다.
//   **첫 갈라짐(first) 시점의 진입 플랜 상태**만 따로 세야 원인 후보가 된다.
thread_local! { static PLAN_BITS: std::cell::Cell<u64> = std::cell::Cell::new(0); }
pub static PLAN_FD: [AtomicUsize; 46] = [const { AtomicUsize::new(0) }; 46];
pub static PLAN_MB: [AtomicUsize; 46] = [const { AtomicUsize::new(0) }; 46];
pub static PLAN_FCLEAN: AtomicUsize = AtomicUsize::new(0);
fn plan_first_record() {
    let b = PLAN_BITS.with(|c| c.get());
    if b == 0 { PLAN_FCLEAN.fetch_add(1, Ordering::Relaxed); return; }
    for i in 0..PLAN_F.len() { if b & (1 << i) != 0 { PLAN_FD[i].fetch_add(1, Ordering::Relaxed); } }
}
/// 첫 갈라짐 몇 건에 한해 **어느 워드가 어떤 값으로** 달랐는지 원본을 찍는다.
///   필드 단위로 "100% 다름" 까지는 알았으나, 그게 패딩·미초기화 잎음인지 진짜 상태차인지는 값을 봐야 가린다.
static PLAN_DUMPED: AtomicUsize = AtomicUsize::new(0);
unsafe fn plan_dump(g: *mut u8, m: *mut u8) {
    if PLAN_DUMPED.fetch_add(1, Ordering::Relaxed) >= 3 { return; }
    let bits = PLAN_BITS.with(|c| c.get());
    let mut s = String::from("plan-dump");
    for (i, &(o, sz, _k, nm)) in PLAN_F.iter().enumerate() {
        if bits & (1 << i) == 0 { continue; }
        s.push_str(&format!("\n  {} +0x{:x}({}B):", nm, o, sz));
        let mut shown = 0;
        for w in 0..(sz / 8) {
            let (x, y) = (core::ptr::read_unaligned((g.add(0x530 + o) as *const u64).add(w)),
                          core::ptr::read_unaligned((m.add(0x530 + o) as *const u64).add(w)));
            if x == y { continue; }
            s.push_str(&format!(" [{}]{:x}/{:x}", w, x, y));
            shown += 1;
            if shown >= 5 { s.push_str(" ..."); break; }
        }
        if shown == 0 { s.push_str(" (꺼리 바이트만)"); }
    }
    log(s);
}
pub fn plan_mb_report() -> String {
    let f = |a: &[AtomicUsize], names: &dyn Fn(usize) -> &'static str, n: usize| {
        let mut v: Vec<(usize, &str)> = (0..n).map(|i| (a[i].load(Ordering::Relaxed), names(i))).filter(|x| x.0 > 0).collect();
        v.sort_by(|x, y| y.0.cmp(&x.0));
        if v.is_empty() { "없음".to_string() } else { v.iter().map(|(c, nm)| format!("{}={}", nm, c)).collect::<Vec<_>>().join(" ") }
    };
    format!("plan[{}] ag[{}]", f(&PLAN_MB, &|i| PLAN_F[i].3, PLAN_F.len()), f(&AG_MB, &|i| AG_F[i].3, AG_F.len()))
}
pub fn plan_first_report() -> String {
    let mut v: Vec<(usize, &str)> = PLAN_F.iter().enumerate().map(|(i, f)| (PLAN_FD[i].load(Ordering::Relaxed), f.3)).filter(|x| x.0 > 0).collect();
    v.sort_by(|a, b| b.0.cmp(&a.0));
    format!("플랜 전원일치={} | {}", PLAN_FCLEAN.load(Ordering::Relaxed),
        v.iter().take(14).map(|(c, n)| format!("{}={}", n, c)).collect::<Vec<_>>().join(" "))
}
/// 리포트용 — 다른 필드를 많은 순으로.
pub fn plan_field_report() -> String {
    let mut v: Vec<(usize, &str)> = PLAN_F.iter().enumerate().map(|(i, f)| (PLAN_D[i].load(Ordering::Relaxed), f.3)).filter(|x| x.0 > 0).collect();
    v.sort_by(|a, b| b.0.cmp(&a.0));
    if v.is_empty() { return "전원 일치".into(); }
    v.iter().take(14).map(|(c, n)| format!("{}={}", n, c)).collect::<Vec<_>>().join(" ")
}
/// ★에이전트 본체 필드 — 출처 = `_gaibc/m07.ll` DICompositeType "AgentVerHamster"(40필드).
///   그동안 static_rnd·명명 스칼라·small_action 태그·plan_system 만 봤다 — 나머지 15개는 미검사였다.
///   (plan_system 은 PLAN_F 로 따로 전개, static_rnd 는 패딩 제외 비교라 제외)
pub static AG_F: [(usize, usize, u8, &str); 15] = [
    (0x0000,  224, K_PTR, "debug"),
    (0x00e0,  224, K_PTR, "big_debug"),
    (0x01c0,  224, K_PTR, "small_debug"),
    (0x02a0,   48, K_PTR, "sthfn_stack"),
    (0x0410,   32, K_PTR, "noinput_bucket"),
    (0x0430,   16, K_RAW, "freeze_anchor"),
    (0x0440,   48, K_PTR, "freeze_eps"),
    (0x0470,   48, K_PTR, "freeze_plan"),
    (0x04a0,   48, K_PTR, "freeze_field_action"),
    (0x04d0,   48, K_PTR, "sthfn_plan"),
    (0x0500,   48, K_PTR, "noinput_lapse_action"),
    (0x1d48,   24, K_VEC, "failed_action"),
    (0x1d60,   24, K_VEC, "events"),
    (0x1d78,   24, K_VEC, "stay_events"),
    (0x1d90, 2760, K_PTR, "ag_positioning_score"),
];
pub static AG_D: [AtomicUsize; 15] = [const { AtomicUsize::new(0) }; 15];
pub static AG_FD: [AtomicUsize; 15] = [const { AtomicUsize::new(0) }; 15];
pub static AG_MB: [AtomicUsize; 15] = [const { AtomicUsize::new(0) }; 15];
thread_local! { static AG_BITS: std::cell::Cell<u64> = std::cell::Cell::new(0); }
unsafe fn ag_field_check(g: *mut u8, m: *mut u8) {
    let mut bits = 0u64;
    for (i, &(o, sz, k, _)) in AG_F.iter().enumerate() {
        if !field_same(g, m, 0, o, sz, k) { AG_D[i].fetch_add(1, Ordering::Relaxed); bits |= 1 << i; }
    }
    AG_BITS.with(|c| c.set(bits));
}
fn ag_first_record() {
    let b = AG_BITS.with(|c| c.get());
    for i in 0..AG_F.len() { if b & (1 << i) != 0 { AG_FD[i].fetch_add(1, Ordering::Relaxed); } }
}
pub fn ag_field_report() -> String {
    let f = |a: &[AtomicUsize]| {
        let mut v: Vec<(usize, &str)> = AG_F.iter().enumerate().map(|(i, x)| (a[i].load(Ordering::Relaxed), x.3)).filter(|x| x.0 > 0).collect();
        v.sort_by(|x, y| y.0.cmp(&x.0));
        if v.is_empty() { "전원 일치".to_string() } else { v.iter().map(|(c, n)| format!("{}={}", n, c)).collect::<Vec<_>>().join(" ") }
    };
    format!("전체[{}] | 첫갈라짐[{}]", f(&AG_D), f(&AG_FD))
}
/// 스택 잔재를 고정 패턴으로 덮는다 — 같은 코드·같은 상태인데 **스택 초기값만 다르게** 두 번 돌려
///   결과가 갈리면 그 코드는 **초기화 안 된 스택을 읽고 있다**(= 어떤 사본도 비트동일 재현 불가).
#[inline(never)]
unsafe fn scrub_stack(v: u8) {
    let mut buf = [0u8; 49152];
    core::ptr::write_bytes(buf.as_mut_ptr(), v, buf.len());
    std::hint::black_box(buf.as_mut_ptr());
}
/// ★**트윈이 태어나는 순간**의 전체 이미지 비교(10,704B).
///   지금까지는 `get_input` 진입 시점만 봤고, 거기서 "항상 다른 필드"를 잡음으로 치부했다.
///   그러나 실측된 값이 게임=0 / 내사본=`fffffffffffffffe`(SEH 마커)·스택 주소 잔재라
///   **내 사본은 처음부터 초기화 안 된 바이트를 가지고 태어날** 가능성이 있다.
///   드물게 그걸 읽는 경로가 있다면 정확히 0.02% 짜리 증상이 된다.
pub static BIRTH_N: AtomicUsize = AtomicUsize::new(0);
pub static BIRTH_BYTES: AtomicUsize = AtomicUsize::new(0);
pub static BIRTH_ZERO_G: AtomicUsize = AtomicUsize::new(0);
static BIRTH_LOG: Mutex<Vec<String>> = Mutex::new(Vec::new());
unsafe fn birth_diff(g: *mut u8, m: *mut u8) {
    let n = BIRTH_N.fetch_add(1, Ordering::Relaxed);
    let mut nd = 0usize;
    let mut zg = 0usize;
    let mut first: Vec<String> = Vec::new();
    for k in (0..super::agent_link::AGENT_SIZE).step_by(8) {
        let (a, b) = (core::ptr::read_unaligned(g.add(k) as *const u64), core::ptr::read_unaligned(m.add(k) as *const u64));
        if a == b { continue; }
        // 양쪽 다 유효 주소면 힙 포인터로 보고 제외(사본마다 달라도 정상)
        let ptrish = |v: u64| v >= 0x10000 && v < (1u64 << 48);
        if ptrish(a) && ptrish(b) { continue; }
        nd += 1;
        if a == 0 { zg += 1; }
        if first.len() < 24 { first.push(format!("+0x{:04x} g={:016x} m={:016x}", k, a, b)); }
    }
    BIRTH_BYTES.fetch_add(nd, Ordering::Relaxed);
    BIRTH_ZERO_G.fetch_add(zg, Ordering::Relaxed);
    if n < 3 {
        let mut lg = BIRTH_LOG.lock().unwrap_or_else(|e| e.into_inner());
        lg.push(format!("[birth #{}] 다른 워드 {}개(그중 게임이 0 인 것 {}개)\n   {}", n, nd, zg, first.join("\n   ")));
    }
}
/// ★태어날 때 트윈의 **해시 키를 게임 것으로 맞춴 넣는다**.
///   에이전트 안 `HashMap` 들은 `ahash::RandomState`(구조체 끝 32B) 를 가지며 인스턴스마다 난수다.
///   그러면 **반복 순서**가 게임과 달라져 드물게 판단이 갈린다.
///   태어날 시점엔 맵이 모두 비어 있으므로 키를 바꿔도 안전하다.
///   ⚠양쪽 다 유효주소인 워드(= 힙 포인터)는 건드리지 않는다 — 그건 각자의 버퍼다.
pub static ALIGN_WORDS: AtomicUsize = AtomicUsize::new(0);
unsafe fn birth_align(g: *mut u8, m: *mut u8) {
    let mut n = 0usize;
    for k in (0..super::agent_link::AGENT_SIZE).step_by(8) {
        let (a, b) = (core::ptr::read_unaligned(g.add(k) as *const u64), core::ptr::read_unaligned(m.add(k) as *const u64));
        if a == b { continue; }
        let ptrish = |v: u64| v >= 0x10000 && v < (1u64 << 48);
        if ptrish(a) || ptrish(b) { continue; }          // 한쪽이라도 포인터면 손대지 않는다
        core::ptr::write_unaligned(m.add(k) as *mut u64, a);
        n += 1;
    }
    ALIGN_WORDS.fetch_add(n, Ordering::Relaxed);
}
pub fn birth_report() -> String {
    let n = BIRTH_N.load(Ordering::Relaxed).max(1);
    let mut s = format!("[twin-birth] 팬어리 쌍 {}개 · 태어날 때 다른 워드 평균 {:.1}개(포인터 제외) · 그중 게임이 0 인 비율 {:.0}%\n",
        BIRTH_N.load(Ordering::Relaxed), BIRTH_BYTES.load(Ordering::Relaxed) as f64 / n as f64,
        100.0 * BIRTH_ZERO_G.load(Ordering::Relaxed) as f64 / BIRTH_BYTES.load(Ordering::Relaxed).max(1) as f64);
    s.push_str(&format!("   └ 해시키 정렬해 넣은 워드 {}개(cfg twin_align_hash)\n", ALIGN_WORDS.load(Ordering::Relaxed)));
    let lg = BIRTH_LOG.lock().unwrap_or_else(|e| e.into_inner());
    for l in lg.iter() { s.push_str(l); s.push('\n'); }
    s
}
/// 호출 후 상태가 같은가 — 잡음 필드(해시키·패딩)를 뺀 **진짜 상태**만 본다.
///   0 명명스칼라 17 / 1 small_action 태그 / 2 plan counters50 / 3 static_rnd / 4 failed_action len / 5 events len
/// ★plan +0x1468‥+0x15f8 = usize 50개. **포인터도 패딩도 없는 유일하게 완전히 믿을 수 있는 상태**다.
///   출처 = `_gaibc/m07.ll` DICompositeType "LegacyPlanHandler" DI 멤버 전개(2026-09-10 확인).
///   어느 카운터가 **진입이 같았는데 그 호출에서 벌어졌는가** — 그게 곳 발생 지점이다.
pub static CTR_NAMES: [&str; 50] = [
    "version", "gank_cancel_tick", "dive_cancel_tick", "last_dive_abandon_tick", "last_response_bail_tick", "last_lost_fight.0", "last_lost_fight.1", "gank_attempt_count", "gank_request_count", "last_gank_request_tick", "last_jungle_lead_action_tick", "comeback_pick_attempt_count", "comeback_pick_success_count", "v46_lane_recall_danger_ticks", "v46_lane_recall_veto_wave", "v46_lane_recall_veto_crash", "v46_lane_recall_veto_heal", "v46_lane_recall_stage2_saves", "v46_lane_recall_commit_clears", "v46_lane_recall_wave_enemy_half", "v46_lane_recall_wave_my_half", "v46_lane_flee_triggers", "v46_lane_flee_hold_ticks", "v46_lane_flee_hold_acute_ticks", "v46_lane_flee_hold_refuge_ticks", "v46_lane_flee_hold_cover_ticks", "v46_lane_flee_nohit_episodes", "v48_dodge_step_picks", "v48_dodge_flee_picks", "v48_claim_until_tick", "v48_last_non_target_hit", "v48_claim_window_ticks", "v48_claim_window_hits", "v48_other_hits", "v48_claim_hit_locked", "v48_claim_hit_moving", "v48_claim_hit_idle", "v48_cast_cc", "v48_cast_locked", "v48_cast_free_near", "v48_cast_free_far", "flee_prev_death_count", "ff_call_recv", "ff_call_ignored", "ff_call_in_battle", "ff_call_no_help", "ff_call_too_far", "ff_call_low_hp", "ff_call_bail", "ff_call_join",
];
/// 진입 counters 가 **같았는데** 퇴장에서 벌어진 횟수(= 신규 발생). 잔차 이월과 가른다.
pub static CTR_FRESH: [AtomicUsize; 50] = [const { AtomicUsize::new(0) }; 50];
/// 진입이 이미 달랐던 경우 포함 — 잔차까지 모두 센 횟수(참고값).
pub static CTR_ANY: [AtomicUsize; 50] = [const { AtomicUsize::new(0) }; 50];
pub static CTR_GATE: AtomicUsize = AtomicUsize::new(0);
pub static CTR_FRESH_N: AtomicUsize = AtomicUsize::new(0);
/// 진입 시점 counters50 일치 여부(state_check 가 쓰고 퇴장 비교가 읽는다).
thread_local! { static ENTRY_CTR: std::cell::Cell<bool> = std::cell::Cell::new(false); }
#[inline] unsafe fn ctr_diff_mask(g: *mut u8, m: *mut u8, out: &mut [bool; 50]) -> bool {
    let mut any = false;
    for k in 0..50usize {
        let a = core::ptr::read_unaligned((g.add(0x530 + 0x1468) as *const u64).add(k));
        let b = core::ptr::read_unaligned((m.add(0x530 + 0x1468) as *const u64).add(k));
        out[k] = a != b;
        any |= a != b;
    }
    any
}
unsafe fn post_state_same(g: *mut u8, m: *mut u8) -> (bool, [bool; 6]) {
    let mut f = [true; 6];
    const SCAL: [usize; 17] = [0x2910, 0x2918, 0x2920, 0x2928, 0x2930, 0x2938, 0x2940, 0x2948, 0x2950, 0x2958, 0x2960, 0x2968, 0x2970, 0x2978, 0x2998, 0x29a0, 0x29a8];
    f[0] = !SCAL.iter().any(|&o| core::ptr::read_unaligned(g.add(o) as *const u64) != core::ptr::read_unaligned(m.add(o) as *const u64));
    // ⚠small_action(SmallActionPlay 184B) 은 **enum 이라 바이트 비교 자체가 성립하지 않는다**.
    //   실측(2026-09-10): 태그와 사용 페이로드는 같은데 미사용 변형 자리에 서로 다른 쓰레기가 남는다.
    //     g=6402..1d00..0000  vs  m=6402..1d00..1027(=10000)  — 같은 변형인데 뒷부분만 다름.
    //   첫 8B 도 변형에 따라선 자기 이미지를 가리키는 포인터다(g=exe 범위 / m=내 DLL 범위).
    //   → 양방향으로 믿을 수 없으므로 **진입 게이트와 동일하게 태그 8B 만** 본다(포인터면 면제).
    {
        let ptrish = |v: u64| v >= 0x10000 && v < (1u64 << 48);
        let (a0, b0) = (core::ptr::read_unaligned(g.add(0x2858) as *const u64), core::ptr::read_unaligned(m.add(0x2858) as *const u64));
        f[1] = a0 == b0 || (ptrish(a0) && ptrish(b0));
    }
    f[2] = std::slice::from_raw_parts(g.add(0x530 + 0x1468), 400) == std::slice::from_raw_parts(m.add(0x530 + 0x1468), 400);
    f[3] = std::slice::from_raw_parts(g.add(0x2d0), 0x108) == std::slice::from_raw_parts(m.add(0x2d0), 0x108)
        && std::slice::from_raw_parts(g.add(0x2d0 + 0x110), 48) == std::slice::from_raw_parts(m.add(0x2d0 + 0x110), 48);
    // Vec 은 포인터 뺀 두 워드(길이·용량)만
    let veclen = |o: usize| -> bool {
        let mut ok = true;
        for w in 0..3 {
            let (a, b) = (core::ptr::read_unaligned((g.add(o) as *const u64).add(w)), core::ptr::read_unaligned((m.add(o) as *const u64).add(w)));
            if a == b { continue; }
            if a >= 0x10000 && a < (1u64 << 48) && b >= 0x10000 && b < (1u64 << 48) { continue; }
            ok = false;
        }
        ok
    };
    f[4] = veclen(0x1d48);
    f[5] = veclen(0x1d60);
    (f.iter().all(|x| *x), f)
}
unsafe fn state_check(g: *mut u8, m: *mut u8, tag: &str) {
    ST_CMP.fetch_add(1, Ordering::Relaxed);
    // ★StdRng 320B = BlockRng{ results 256B @+0 · index 8B @+0x100 · **공백 8B @+0x108** · core 48B @+0x110 }.
    //   공백은 new 의 memcpy 가 스택 쓰레기째 복사하므로 양쪽이 다르다 → 실측 90% 가 이 8바이트였다(2026-09-09). 제외하고 비교.
    let rq = |p: *mut u8| (std::slice::from_raw_parts(p.add(0x2d0), 0x108), std::slice::from_raw_parts(p.add(0x2d0 + 0x110), 48));
    // ★스칼라는 **이름 붙은 정수 필드만** 본다. +0x2980 freeze_pos / +0x29b0 noinput_lapse_pos 는 Option<Position>(24B)라
    //   미사용 페이로드가 쓰레기다(실측: 통째 비교 시 1.5% 가 전부 이것). u64 16개 + u16/bool 3개만 대조.
    const SCAL: [usize; 17] = [0x2910, 0x2918, 0x2920, 0x2928, 0x2930, 0x2938, 0x2940, 0x2948, 0x2950, 0x2958, 0x2960, 0x2968, 0x2970, 0x2978, 0x2998, 0x29a0, 0x29a8];
    // ★freeze_pos(+0x2980) · noinput_lapse_pos(+0x29b0) 는 `Option<Position>`(24B) = 태그 8B + 값 16B.
    //   앞서 "미초기화 페이로드" 라며 통째로 뺐는데, **Some 일 때 값은 진짜다**. 태그는 항상, 값은 Some 일 때만 본다.
    //   (이 둘은 "어디서 멈춰 있었나" 를 담아 다음 틱의 평가 위치를 좌우한다 — 진입 상태 판정에서 빠지면 안 된다.)
    let opt_pos_same = |o: usize| {
        let (tg, tm) = (core::ptr::read_unaligned(g.add(o) as *const u64), core::ptr::read_unaligned(m.add(o) as *const u64));
        tg == tm && (tg == 0 || std::slice::from_raw_parts(g.add(o + 8), 16) == std::slice::from_raw_parts(m.add(o + 8), 16))
    };
    let pos_same = opt_pos_same(0x2980) && opt_pos_same(0x29b0);
    let scal_diff = SCAL.iter().any(|&o| core::ptr::read_unaligned(g.add(o) as *const u64) != core::ptr::read_unaligned(m.add(o) as *const u64))
        || core::ptr::read_unaligned(g.add(0x29c8) as *const u16) != core::ptr::read_unaligned(m.add(0x29c8) as *const u16)
        || *g.add(0x29ca) != *m.add(0x29ca) || *g.add(0x29cb) != *m.add(0x29cb)
        || !pos_same;
    // small_action 은 enum(184B) — 미사용 변형 페이로드가 쓰레기라 **태그 8B 만** 본다.
    let (ga, ma) = (core::ptr::read_unaligned(g.add(0x2858) as *const u64), core::ptr::read_unaligned(m.add(0x2858) as *const u64));
    // ★plan_system: LegacyPlanHandler(+0x530, 6168B) = 에이전트의 **가장 큰 상태**(플랜 계층 전체).
    //   통꺼 비교는 힙 포인터·패딩·니치 때문에 항상 100% 불일치로 나와 무의미했다.
    //   디버그정보(_gaibc/m07.ll DICompositeType "LegacyPlanHandler")로 필드를 전개해 종류별로 비교한다.
    plan_field_check(g, m);
    ag_field_check(g, m);
    let (dr, ds, da) = (rq(g) != rq(m), scal_diff, ga != ma);
    if dr { ST_RNG.fetch_add(1, Ordering::Relaxed); }
    if ds { ST_SCAL.fetch_add(1, Ordering::Relaxed); }
    if da { ST_SA.fetch_add(1, Ordering::Relaxed); }
    ENTRY_SAME.with(|c| c.set((!dr, !ds, !da)));
    {   // ★진입 counters50 도 같았는지 — 같았을 때만 퇴장 차이가 "이 호출에서 발생" 이다.
        let mut d = [false; 50];
        ENTRY_CTR.with(|c| c.set(!ctr_diff_mask(g, m, &mut d)));
    }
    if (dr || ds || da) && ST_RNG.load(Ordering::Relaxed) + ST_SCAL.load(Ordering::Relaxed) + ST_SA.load(Ordering::Relaxed) <= 12 {
        let mut fields = String::new();
        for (o, n) in [(0x2910usize, "version"), (0x2918, "small_action_score"), (0x2920, "last_battle_tick"), (0x2928, "trace_escape_target"), (0x2930, "trace_escape_ticks"), (0x2938, "trace_escape_last_tick"), (0x2940, "next_input_tick"), (0x2948, "last_eval_tick"), (0x2950, "last_eval_hp"), (0x2958, "input_chances"), (0x2960, "freeze_since"), (0x2968, "freeze_eps_lapse"), (0x2970, "freeze_eps_dead_target"), (0x2978, "freeze_ticks"), (0x2998, "lapse_chances"), (0x29a0, "last_combat_tick"), (0x29a8, "stay_home_full_nobuy")] {
            let (a, b) = (core::ptr::read_unaligned(g.add(o) as *const u64), core::ptr::read_unaligned(m.add(o) as *const u64));
            if a != b { fields.push_str(&format!(" {}={}/{}", n, a, b)); }
        }
        let ri = (core::ptr::read_unaligned(g.add(0x2d0 + 0x100) as *const u64), core::ptr::read_unaligned(m.add(0x2d0 + 0x100) as *const u64));
        log(format!("state[{}] #{} rnd_diff={} (index {}/{}) scalar_diff={}{} sa_tag={}/{}", tag, ST_CMP.load(Ordering::Relaxed), dr, ri.0, ri.1, ds, fields, ga, ma));
    }
}
extern "system" { fn VirtualAlloc(a: usize, sz: usize, ty: u32, pr: u32) -> usize; }
extern "system" { fn GetCurrentThreadId() -> u32; }
pub static GI_NOTWIN: AtomicUsize = AtomicUsize::new(0);
pub static BUY_CMP: AtomicUsize = AtomicUsize::new(0);
pub static BUY_DIFF: AtomicUsize = AtomicUsize::new(0);
pub static UPG_CMP: AtomicUsize = AtomicUsize::new(0);
pub static UPG_DIFF: AtomicUsize = AtomicUsize::new(0);
pub static DEAD_CMP: AtomicUsize = AtomicUsize::new(0);
pub static DEAD_DIFF: AtomicUsize = AtomicUsize::new(0);
pub static MIRROR: AtomicUsize = AtomicUsize::new(0);
pub static DROPS: AtomicUsize = AtomicUsize::new(0);
pub static CLONES: AtomicUsize = AtomicUsize::new(0);
pub static CLONE_FAIL: AtomicUsize = AtomicUsize::new(0);
static LOG: Mutex<Vec<String>> = Mutex::new(Vec::new());
/// ★재진입 가드 — 게임이 에이전트 메서드를 **자기 안에서 다시 부르면**(예: get_input 안에서 update_on_dead)
///   그때마다 트윈을 또 돌리게 되고, 트윈은 같은 계산을 두 번 하면서 **자기 TLS 캐시를 게임과 다르게 채운다**.
///   함수 단위 이분 탐색에서 이것이 `position_eval_at` 불일치 355건의 원인이었고, 가드를 넣자 6,588만 콜 DIFF 0 이 됐다
///   (2026-09-09). 미러는 **최상위 1회만**.
thread_local! { static DEPTH: std::cell::Cell<u32> = std::cell::Cell::new(0); }
#[inline] fn enter() -> bool { DEPTH.with(|d| { let v = d.get(); d.set(v + 1); v == 0 }) }
#[inline] fn leave() { DEPTH.with(|d| d.set(d.get().saturating_sub(1))); }
/// ★호출 **직전** 상태가 같았는가 — 최초 갈라짐이 "코드 동작 차이"인지 "이미 어긋난 상태" 인지 가르는 결정적 표시.
static PS_SEEN: Mutex<Option<std::collections::HashSet<usize>>> = Mutex::new(None);
fn mark_ps(k: usize) -> bool { let mut g = PS_SEEN.lock().unwrap_or_else(|e| e.into_inner()); g.get_or_insert_with(std::collections::HashSet::new).insert(k) }
thread_local! { static ENTRY_SAME: std::cell::Cell<(bool, bool, bool)> = std::cell::Cell::new((true, true, true)); }
pub static FIRST_CLEAN_ENTRY: AtomicUsize = AtomicUsize::new(0);
/// ★한 번 갈라진 트윈 쌍은 내부 상태가 어긋나 이후 계속 다르다(눈덩이). 그래서 **에이전트별 최초 1회**만 세야
///   "독립적인 오류가 몇 건인가" 를 알 수 있다. 실측 2026-09-09: 총 DIFF 2,831 / 3.75M.
static DIVERGED: Mutex<Option<std::collections::HashSet<usize>>> = Mutex::new(None);
pub static FIRST_DIFF: AtomicUsize = AtomicUsize::new(0);
pub static PAIRS_SEEN: AtomicUsize = AtomicUsize::new(0);
fn mark_diverged(s: usize) -> bool {
    let mut g = DIVERGED.lock().unwrap_or_else(|e| e.into_inner());
    g.get_or_insert_with(std::collections::HashSet::new).insert(s)
}

fn log(s: String) { let mut g = LOG.lock().unwrap_or_else(|e| e.into_inner()); if g.len() < 80 { g.push(s); } }
fn hex(b: &[u8]) -> String { b.iter().map(|x| format!("{:02x}", x)).collect() }
#[inline] unsafe fn my(slot: usize) -> usize { core::ptr::read_unaligned((MY_VT.load(Ordering::Relaxed) + slot * 8) as *const usize) }
#[inline] fn twin(s: *mut u8) -> Option<usize> {
    let g = TWIN.lock().unwrap_or_else(|e| e.into_inner());
    g.as_ref().and_then(|m| m.get(&(s as usize)).copied())
}
#[inline] unsafe fn bb_of(data: *mut u8) -> usize { if ptr_ok(data as usize) { rd_u64(data as usize + OD_BB).unwrap_or(0) as usize } else { 0 } }


/// `Option<Input>`(32B) 의 **의미 있는 바이트만** 비교 — 레이아웃(_gaibc/m01.ll !5533 · m00.ll !792 DI):
///   Input tag u64 @+0 : 0 Move{x@+8,y@+16} · 1 Return · 2 Attack · 3 Skill · 4 Skill2 · 5 Ult (2~5 = InputTarget @+8) · None = 0xffff…(niche)
///   InputTarget tag u32 @+8(=Input+8) : 0 Target{target_id @Input+16} · 1 Dir{dir_x @+16, dir_y @+24} · 2 Pos{x @+16, y @+24} · 3 None
///   ⟹ Input+12..16 은 항상 패딩, Target/None 변형에선 +24..32 도 미사용 → 게임/내 코드가 서로 다른 스택 쓰레기를 남긴다(실측 2026-09-09: 모든 "DIFF" 가 이 바이트였다).
pub fn input_eq(g: &[u8], m: &[u8]) -> bool {
    let tag = u64::from_le_bytes(g[0..8].try_into().unwrap());
    if tag != u64::from_le_bytes(m[0..8].try_into().unwrap()) { return false; }
    match tag {
        0 => g[8..24] == m[8..24],
        1 => true,
        2 | 3 | 4 | 5 => {
            let tt = u32::from_le_bytes(g[8..12].try_into().unwrap());
            if tt != u32::from_le_bytes(m[8..12].try_into().unwrap()) { return false; }
            match tt { 0 => g[16..24] == m[16..24], 1 | 2 => g[16..32] == m[16..32], _ => true }
        }
        _ => true,
    }
}
pub fn input_desc(g: &[u8]) -> String {
    let tag = u64::from_le_bytes(g[0..8].try_into().unwrap());
    let u = |o: usize| u64::from_le_bytes(g[o..o + 8].try_into().unwrap());
    match tag {
        0 => format!("Move({},{})", u(8), u(16)), 1 => "Return".into(),
        2 | 3 | 4 | 5 => {
            let nm = ["Attack", "Skill", "Skill2", "Ult"][(tag - 2) as usize];
            let tt = u32::from_le_bytes(g[8..12].try_into().unwrap());
            match tt { 0 => format!("{}(id {})", nm, u(16)), 1 => format!("{}(dir {},{})", nm, u(16) as i64, u(24) as i64), 2 => format!("{}(pos {},{})", nm, u(16), u(24)), _ => format!("{}(None)", nm) }
        }
        _ => "None".into(),
    }
}

struct Snap { rng: [u8; RNG_SIZE], bb: [u8; BB_SIZE], bbp: usize, rp: usize }
impl Snap {
    #[inline] unsafe fn take(rnd: *mut u8, bbp: usize) -> Snap {
        let mut s = Snap { rng: [0; RNG_SIZE], bb: [0; BB_SIZE], bbp, rp: rnd as usize };
        if ptr_ok(rnd as usize) { core::ptr::copy_nonoverlapping(rnd, s.rng.as_mut_ptr(), RNG_SIZE); }
        if ptr_ok(bbp) { core::ptr::copy_nonoverlapping(bbp as *const u8, s.bb.as_mut_ptr(), BB_SIZE); }
        s
    }
    #[inline] unsafe fn restore(&self) {
        if ptr_ok(self.rp) { core::ptr::copy_nonoverlapping(self.rng.as_ptr(), self.rp as *mut u8, RNG_SIZE); }
        if ptr_ok(self.bbp) { core::ptr::copy_nonoverlapping(self.bb.as_ptr(), self.bbp as *mut u8, BB_SIZE); }
    }
    /// ★외부 rnd 비교도 **패딩 제외**: StdRng = BlockRng{ results 256B @+0 · index 8B @+0x100 · 공백 8B @+0x108 · core 48B @+0x110 }.
    ///   게임 코드가 로컬 사본을 320B 통째로 되쓰면 그 공백에 각자의 스택 쓰레기가 들어간다 —
    ///   실측 2026-09-09: "결정은 같은데 rng 다름" 1,061건이 전부 이 8바이트였다(내부 static_rnd 는 패딩 제외 후 차이 0).
    #[inline] unsafe fn same_as(&self, o: &Snap) -> (bool, bool) {
        let r = self.rng[..0x108] == o.rng[..0x108] && self.rng[0x110..] == o.rng[0x110..];
        (r, self.bb == o.bb)
    }
}

extern "Rust" {
    #[link_name = "_RNvNtNtCs97f5S1uJLkH_9game_core10simulation12ai_interface15create_ai_agent"]
    fn my_create_ai_agent(rng: *mut u8, a: usize, b: usize, pos: u32) -> (usize, usize);
}

/// 팩토리(mode 3): 게임 에이전트 + 트윈 생성. 반환은 게임 에이전트(게임은 원본 AI 로 진행 — 트윈은 그림자).
pub unsafe fn on_factory(orig: FactoryFn, rng: *mut u8, a: usize, b: usize, pos: u32) -> (usize, usize) {
    F_CALLS.fetch_add(1, Ordering::Relaxed);
    let _ = SIM_TID.compare_exchange(0, GetCurrentThreadId() as usize, Ordering::SeqCst, Ordering::Relaxed);
    let s0 = Snap::take(rng, 0);
    let (dg, vg) = orig(rng, a, b, pos);
    let after_g = Snap::take(rng, 0);
    s0.restore();
    // mode 4 = 대조군: 트윈도 **게임 팩토리**로 만든다(게임 vs 게임). 여기서 나오는 DIFF 율이 게임 자체의 인스턴스별 비결정성(해시 시드 등)의 바닥이다.
    let control = crate::judge::tune_pub("agent_link", 0) == 4;
    let mine = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| if control { orig(rng, a, b, pos) } else { my_create_ai_agent(rng, a, b, pos) }));
    let after_m = Snap::take(rng, 0);
    after_g.restore();
    match mine {
        Ok((dm, vm)) => {
            if !after_m.same_as(&after_g).0 { F_RNG_DIFF.fetch_add(1, Ordering::Relaxed); }
            MY_VT.store(vm, Ordering::SeqCst);
            { let mut g = TWIN.lock().unwrap_or_else(|e| e.into_inner()); g.get_or_insert_with(HashMap::new).insert(dg, dm); }
            birth_diff(dg as *mut u8, dm as *mut u8);
            if crate::judge::tune_pub("twin_align_hash", 0) != 0 { birth_align(dg as *mut u8, dm as *mut u8); }
            PAIRS_SEEN.fetch_add(1, Ordering::Relaxed);
            install_shims(vg);
        }
        Err(_) => { F_FAIL.fetch_add(1, Ordering::Relaxed); }
    }
    (dg, vg)
}

unsafe fn patch_slot(vt: usize, slot: usize, f: usize) {
    let p = vt + slot * 8;
    ORIG[slot].store(core::ptr::read_unaligned(p as *const usize), Ordering::SeqCst);
    let mut old: u32 = 0;
    if VirtualProtect(p, 8, 0x04, &mut old) != 0 { core::ptr::write_unaligned(p as *mut usize, f); VirtualProtect(p, 8, old, &mut old); }
}
unsafe fn install_shims(vg: usize) {
    if G_VT.compare_exchange(0, vg, Ordering::SeqCst, Ordering::Relaxed).is_err() { return; }
    patch_slot(vg, S_DROP, drop_shim as usize);
    patch_slot(vg, S_CLONE, clone_box_shim as usize);
    patch_slot(vg, S_SETVER, set_version_shim as usize);
    patch_slot(vg, S_GI, get_input_shim as usize);
    patch_slot(vg, S_BUY, buy_item_shim as usize);
    patch_slot(vg, S_UPG, upgrade_item_shim as usize);
    patch_slot(vg, S_DEAD, update_on_dead_shim as usize);
    patch_slot(vg, S_PGE, push_game_event_shim as usize);
    patch_slot(vg, S_PPTE, push_pending_trace_event_shim as usize);
    patch_slot(vg, S_CHATS, plan_chats_drain_shim as usize);
    patch_slot(vg, S_PTED, plan_pted_drain_shim as usize);
    install_slot_probes(vg);
}
/// ★미미러링 슬롯 찾기 — AiAgent vtable 은 40슬롯(DI: vtable_type\$ size 2560bit)인데
///   현재 9개만 미러링한다. 게임이 나머지 중 상태를 바꾸는 슬롯을 부르면 트윈이 그만큼 샬다.
///   ABI 를 몰라도 안전하게 세려고 **레지스터를 안 건드리는** 스팝을 쓴다:
///     +0 카운터(u64) / +8 `lock inc qword [rip-16]` / +16 `jmp [rip+0]` / +22 원본 주소
pub static SLOT_HITS: [AtomicUsize; 40] = [const { AtomicUsize::new(0) }; 40];
static PROBE_BASE: AtomicUsize = AtomicUsize::new(0);
const MIRRORED: [usize; 11] = [S_DROP, S_CLONE, S_SETVER, S_GI, S_BUY, S_UPG, S_DEAD, S_PGE, S_PPTE, S_CHATS, S_PTED];
unsafe fn install_slot_probes(vg: usize) {
    let base = VirtualAlloc(0, 40 * 32, 0x3000, 0x40);
    if base == 0 { return; }
    PROBE_BASE.store(base, Ordering::SeqCst);
    for slot in 0..40usize {
        if MIRRORED.contains(&slot) { continue; }
        let p = vg + slot * 8;
        let o = core::ptr::read_unaligned(p as *const usize);
        if o < 0x10000 { continue; }
        let st = base + slot * 32;
        core::ptr::write_bytes(st as *mut u8, 0, 32);
        let c = st as *mut u8;
        // +8: lock inc qword [rip-16]  (rip = +16 이므로 -16 이면 +0 = 카운터)
        core::ptr::copy_nonoverlapping([0xF0u8, 0x48, 0xFF, 0x05, 0xF0, 0xFF, 0xFF, 0xFF].as_ptr(), c.add(8), 8);
        // +16: jmp qword [rip+0] -> +22 의 원본 주소
        core::ptr::copy_nonoverlapping([0xFFu8, 0x25, 0x00, 0x00, 0x00, 0x00].as_ptr(), c.add(16), 6);
        core::ptr::write_unaligned(c.add(22) as *mut usize, o);
        let mut old: u32 = 0;
        if VirtualProtect(p, 8, 0x04, &mut old) != 0 {
            ORIG[slot].store(o, Ordering::SeqCst);
            core::ptr::write_unaligned(p as *mut usize, st + 8);
            VirtualProtect(p, 8, old, &mut old);
        }
    }
}
pub fn slot_report() -> String {
    let base = PROBE_BASE.load(Ordering::Relaxed);
    if base == 0 { return "probe 미설치".into(); }
    let mut v = Vec::new();
    for slot in 0..40usize {
        if MIRRORED.contains(&slot) { continue; }
        let n = unsafe { core::ptr::read_unaligned((base + slot * 32) as *const u64) };
        if n > 0 { v.push((n, slot)); }
    }
    v.sort_by(|a, b| b.0.cmp(&a.0));
    if v.is_empty() { return "미미러링 슬롯 호출 0 — 상태 유입구는 vtable 밖에 있다".into(); }
    v.iter().map(|(n, s)| format!("[{}]={}", s, n)).collect::<Vec<_>>().join(" ")
}
#[inline] unsafe fn orig(slot: usize) -> usize { ORIG[slot].load(Ordering::Relaxed) }

unsafe fn drop_shim(s: *mut u8) {
    { let mut g = TWIN.lock().unwrap_or_else(|e| e.into_inner()); if let Some(m) = g.as_mut() { if m.remove(&(s as usize)).is_some() { DROPS.fetch_add(1, Ordering::Relaxed); } } }
    let f: F1 = std::mem::transmute(orig(S_DROP)); f(s)
}
/// `__clone_box`: 게임이 에이전트를 복제할 때(선수 배치·presim) 트윈도 함께 복제해 짝을 유지한다.
///   실측(2026-09-09): 팩토리 산출물은 곧 drop 되고 실제 선수 에이전트는 전부 클론이라, 이게 없으면 비교 대상이 0 이다.
unsafe fn clone_box_shim(s: *mut u8) -> *mut u8 {
    let f: F1r = std::mem::transmute(orig(S_CLONE));
    let g = f(s);
    if let Some(m) = twin(s) {
        let c: F1r = std::mem::transmute(my(S_CLONE));
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| c(m as *mut u8))) {
            Ok(m2) if !m2.is_null() => { let mut w = TWIN.lock().unwrap_or_else(|e| e.into_inner()); w.get_or_insert_with(HashMap::new).insert(g as usize, m2 as usize); CLONES.fetch_add(1, Ordering::Relaxed); PAIRS_SEEN.fetch_add(1, Ordering::Relaxed); }
            _ => { CLONE_FAIL.fetch_add(1, Ordering::Relaxed); }
        }
    }
    g
}
/// ★`plan_chats_drain` — 반환 = 가져간 Vec(24B, sret). 게임이 매 틱 부르며 에이전트의 chats 를 비운다.
///   미러링하지 않으면 트윈의 chats 만 계속 쌓여 **상태가 갈라진다**(2026-09-09 실측: chats 필드 99.3% 불일치).
///   트윈 쪽 반환 Vec 은 드롭 글루가 없어 놓아둔다 — 대부분 빈 Vec 이라 실제 힙 누수는 채팅 건수만큼이다.
unsafe fn plan_chats_drain_shim(out: *mut u8, s: *mut u8) {
    let f: F2p = std::mem::transmute(orig(S_CHATS));
    f(out, s);
    if let Some(m) = twin(s) {
        let g: F2p = std::mem::transmute(my(S_CHATS));
        let mut o2 = [0u8; 24];
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| g(o2.as_mut_ptr(), m as *mut u8)));
        MIRROR.fetch_add(1, Ordering::Relaxed);
    }
}
unsafe fn plan_pted_drain_shim(out: *mut u8, s: *mut u8) {
    let f: F2p = std::mem::transmute(orig(S_PTED));
    f(out, s);
    if let Some(m) = twin(s) {
        let g: F2p = std::mem::transmute(my(S_PTED));
        let mut o2 = [0u8; 24];
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| g(o2.as_mut_ptr(), m as *mut u8)));
        MIRROR.fetch_add(1, Ordering::Relaxed);
    }
}
unsafe fn set_version_shim(s: *mut u8, v: i64) {
    let f: F2i = std::mem::transmute(orig(S_SETVER)); f(s, v);
    if let Some(m) = twin(s) { let g: F2i = std::mem::transmute(my(S_SETVER)); g(m as *mut u8, v); MIRROR.fetch_add(1, Ordering::Relaxed); }
}
unsafe fn push_game_event_shim(s: *mut u8, ev: *mut u8) {
    let f: F2p = std::mem::transmute(orig(S_PGE)); f(s, ev);
    if let Some(m) = twin(s) { let g: F2p = std::mem::transmute(my(S_PGE)); g(m as *mut u8, ev); MIRROR.fetch_add(1, Ordering::Relaxed); }
}
unsafe fn push_pending_trace_event_shim(s: *mut u8, i: i64, ev: *mut u8) {
    let f: F3ip = std::mem::transmute(orig(S_PPTE)); f(s, i, ev);
    if let Some(m) = twin(s) { let g: F3ip = std::mem::transmute(my(S_PPTE)); g(m as *mut u8, i, ev); MIRROR.fetch_add(1, Ordering::Relaxed); }
}

unsafe fn get_input_shim(out: *mut u8, s: *mut u8, rnd: *mut u8, player: *mut u8, data: *mut u8) {
    let f: F5 = std::mem::transmute(orig(S_GI));
    let m = match twin(s) { Some(m) => m, None => { GI_NOTWIN.fetch_add(1, Ordering::Relaxed); f(out, s, rnd, player, data); return; } };
    state_check(s, m as *mut u8, "gi");
    let top = enter();
    if !top { f(out, s, rnd, player, data); super::agent_link::digest_record(out, player, data); leave(); return; }
    let bbp = bb_of(data);
    super::seq_trace::begin_game();     // ★게임 실행 구간: 내부 position_eval_at 호출을 G 시퀀스에
    let s0 = Snap::take(rnd, bbp);
    // ★내 출력 버퍼를 게임 버퍼의 **호출 전 내용**으로 채운다 — 안 쓰인 바이트(None 의 payload·패딩)가 양쪽에서 같아야 비교가 된다.
    let mut om = [0u8; 64];
    core::ptr::copy_nonoverlapping(out, om.as_mut_ptr(), 64);
    // ★자가검증 표본: **호출 전**에 게임 에이전트를 복제해 둔다.
    //   (호출 후에 복제하면 next_input_tick 게이트가 닫혀 2회차가 무조건 None 이 된다 — 2026-09-09 실패)
    // ★차이의 정체를 **코드차 vs 상태차** 로 가르기 위해 2×2 행렬을 돌린다.
    //   A=게임코드×게임상태(기준) B=게임코드×내상태 C=내코드×게임상태 D=내코드×내상태
    //   A≠B 면 트윈의 **상태**가 다른 것이고, A≠C 면 내 **코드**가 다른 것이다.
    // ☠☠⚠누수 경고 — 이 2×2 행렬은 **호출당 에이전트 6벌을 복제하고 해제하지 않는다.**
    //   vtable 슬롯 0 은 `drop_in_place` 라 **소멸자만 돌리고 박스 10,704B 는 해제되지 않는다**
    //   (Box<dyn T> 의 해제는 vtable size/align 으로 호출자가 따로 해야 한다). 이 파일에 dealloc 은 없다.
    //   ★실측(2026-09-10): 한 판에 get_input 5.1M → 샘플 1.28M × 6벌 = 770만 박스 ≈ **82GB**.
   //   Windows 이벤트 2004 가 `TeamfightManager2.exe … 115,699,220,480 bytes` 를 6회 기록했고,
    //   커밋 한계 73GB 를 넘겨 03:29:57 에 PC 가 비정상 종료됐다(이벤트 41/6008).
    //   ⇒ 기본 OFF. 이 행렬은 이미 역할을 다했다 — 순서효과·상태차·TLS 이력 가설을 전부 배제했고
    //     잔차의 발생 함수(interaction_score)는 fn_bisect 로 분리됐다. 다시 켜려면 **먼저 해제부터 고칠 것**.
    let sample = crate::judge::tune_pub("twin_matrix", 0) != 0 && GI_CMP.load(Ordering::Relaxed) % 4 == 0;
    let (mut cg1, mut cg2, mut cg2b, mut cg3, mut cm1, mut cm2) = (core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut());
    if sample {
        let cl: F1r = std::mem::transmute(orig(S_CLONE));
        let cm: F1r = std::mem::transmute(my(S_CLONE));
        cg1 = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| cl(s))).unwrap_or(core::ptr::null_mut());
        cg2 = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| cl(s))).unwrap_or(core::ptr::null_mut());
        cg2b = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| cl(s))).unwrap_or(core::ptr::null_mut());
        cg3 = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| cl(s))).unwrap_or(core::ptr::null_mut());
        cm1 = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| cm(m as *mut u8))).unwrap_or(core::ptr::null_mut());
        cm2 = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| cm(m as *mut u8))).unwrap_or(core::ptr::null_mut());
    }
    f(out, s, rnd, player, data);
    let after_g = Snap::take(rnd, bbp);
    s0.restore();
    super::seq_trace::begin_mine();     // ★사본 실행 구간: M 시퀀스에
    let g: F5 = std::mem::transmute(my(S_GI));
    let ok = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| g(om.as_mut_ptr(), m as *mut u8, rnd, player, data))).is_ok();
    let after_m = Snap::take(rnd, bbp);
    super::seq_trace::end();
    after_g.restore();
    GI_CMP.fetch_add(1, Ordering::Relaxed);
    let og = std::slice::from_raw_parts(out, 64);
    // 결정 워드 [0..24) + 이벤트 len [0x38..0x40). [24..56) 은 포인터(Move 의 +24 · bumpalo Vec ptr/cap/bump).
    let dec_same = ok && input_eq(og, &om) && og[0x38..0x40] == om[0x38..0x40];
    let (rs, bs) = after_m.same_as(&after_g);
    // ★"결정은 같은데 rng 다름" 의 정체를 가른다: 소비량(index) 이 다른가, 내용(results/core)만 다른가.
    let idx = |s: &Snap| u64::from_le_bytes(s.rng[0x100..0x108].try_into().unwrap());
    let (i0, ig, im) = (idx(&s0), idx(&after_g), idx(&after_m));
    let res_same = after_m.rng[..0x100] == after_g.rng[..0x100];
    let core_same = after_m.rng[0x110..] == after_g.rng[0x110..];
    if !dec_same { GI_DIFF.fetch_add(1, Ordering::Relaxed); }
    if !rs { GI_RNG_DIFF.fetch_add(1, Ordering::Relaxed); }
    if !bs { GI_BB_DIFF.fetch_add(1, Ordering::Relaxed); }
    if !dec_same && rs { GI_DEC_ONLY.fetch_add(1, Ordering::Relaxed); }
    if dec_same && !rs { GI_RNG_ONLY.fetch_add(1, Ordering::Relaxed); }
    let (seed, tick, pl, exp) = super::agent_link::ctx(player, data);
    super::seq_trace::compare(seed, tick, pl);
    let tid = GetCurrentThreadId() as usize; let worker = tid != SIM_TID.load(Ordering::Relaxed);
    if exp != 0 { GI_CMP_PRESIM.fetch_add(1, Ordering::Relaxed); }
    if worker { GI_CMP_WORKER.fetch_add(1, Ordering::Relaxed); }
    if !dec_same || !rs {
        if exp != 0 { GI_DIFF_PRESIM.fetch_add(1, Ordering::Relaxed); }
        if worker { GI_DIFF_WORKER.fetch_add(1, Ordering::Relaxed); }
    }
    if !cg1.is_null() && !cg2.is_null() && !cg2b.is_null() && !cg3.is_null() && !cm1.is_null() && !cm2.is_null() {
        let cur = Snap::take(rnd, bbp);
        let gi_my: F5 = std::mem::transmute(my(S_GI));
        let mut run = |fp: usize, ag: *mut u8, mine: bool| -> (u64, [u8; 128]) {
            s0.restore();
            let mut o = [0u8; 128];
            core::ptr::copy_nonoverlapping(om.as_ptr(), o.as_mut_ptr(), 64);
            let ff: F5 = std::mem::transmute(fp);
            let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| ff(o.as_mut_ptr(), ag, rnd, player, data)));
            let _ = mine;
            (idx(&Snap::take(rnd, bbp)), o)
        };
        let fg = orig(S_GI);
        let fm = gi_my as usize;
        super::seq_trace::begin_game();
        let (ia, oa) = run(fg, cg1, false);
        super::seq_trace::end();
        let (ib, ob) = run(fg, cm1, false);
        super::seq_trace::begin_mine();
        let (ic, oc) = run(fm, cg2, true);
        super::seq_trace::end();
        let (id, od) = run(fm, cm2, true);
        // ★통제 셀 A2 = **게임 코드 × 게임 상태**를 C 와 같은 순번에 한 번 더.
        //   A≠A2 가 A≠C 만큼 나오면 그건 코드차가 아니라 **실험장치의 순서 효과**다.
        let (ic2, oc2) = run(fg, cg2b, false);
        let (ie, oe) = run(fm, cg3, true);   // C′ = 내 코드 × 게임 상태, 두 번째 — 내 코드의 자기일관성
        let dr: F1 = std::mem::transmute(orig(S_DROP));
        let dm: F1 = std::mem::transmute(my(S_DROP));
        for (fpz, a) in [(dr, cg1), (dr, cg2), (dr, cg2b), (dr, cg3)] { let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| fpz(a))); }
        for (fpz, a) in [(dm, cm1), (dm, cm2)] { let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| fpz(a))); }
        cur.restore();
        M_CMP.fetch_add(1, Ordering::Relaxed);
        let dab = ia != ib || !input_eq(&oa[..64], &ob[..64]);
        let dac = ia != ic || !input_eq(&oa[..64], &oc[..64]);
        let dad = ia != id || !input_eq(&oa[..64], &od[..64]);
        if dab {
            M_AB.fetch_add(1, Ordering::Relaxed);
            // ★A≠B = "게임 코드인데 트윈 상태로 돌리니 달라졌다" = 그 순간의 진입 상태 차이가 결정에 실제로 먹혔다는 뜻.
            //   이때의 필드 비트맵만 따로 세면 잡음(항상 다른 필드)과 진짜 누출 필드가 갈린다.
            let pb = PLAN_BITS.with(|c| c.get());
            for i in 0..PLAN_F.len() { if pb & (1 << i) != 0 { PLAN_MB[i].fetch_add(1, Ordering::Relaxed); } }
            let ab = AG_BITS.with(|c| c.get());
            for i in 0..AG_F.len() { if ab & (1 << i) != 0 { AG_MB[i].fetch_add(1, Ordering::Relaxed); } }
        }
        if dac {
            M_AC.fetch_add(1, Ordering::Relaxed);
            // ★A≠C = 상태가 바이트까지 같은데 코드가 갈렸다. 이게 모든 잔차의 뿌리다(첫 갈라짐 102건이 전부 진입상태 동일).
            //   게이트 성격 필드를 같이 찍어 어느 분기에서 갈렸는지 좁힌다.
            if M_AC.load(Ordering::Relaxed) <= 12 {
                let sc = |o: usize| core::ptr::read_unaligned(cg1.add(o) as *const u64);
                log(format!("ac #{} seed={:016x} t={} pl={:04x} | idx A={} C={} | A={} C={}
     next_input_tick={} last_eval_tick={} last_eval_hp={} input_chances={} freeze_since={} freeze_ticks={} lapse_chances={} last_combat_tick={} sa_tag={} ver={}",
                    M_AC.load(Ordering::Relaxed), seed, tick, pl, ia, ic,
                    input_desc(&oa[..64]), input_desc(&oc[..64]),
                    sc(0x2940), sc(0x2948), sc(0x2950), sc(0x2958), sc(0x2960), sc(0x2978), sc(0x2998), sc(0x29a0),
                    core::ptr::read_unaligned(cg1.add(0x2858) as *const u64), sc(0x2910)));
            }
            // ★상태는 바이트까지 같았는데 갈라졌다 — 그 순간의 내부 호출순서를 남긴다.
            let (sd, tk, plr, _) = super::agent_link::ctx(player, data);
            super::seq_trace::compare_ac(sd, tk, plr);
        }
        if dad { M_AD.fetch_add(1, Ordering::Relaxed); }
        if ia != ic2 || !input_eq(&oa[..64], &oc2[..64]) { M_CC.fetch_add(1, Ordering::Relaxed); }
        // ★C4 = 내 코드 × 게임 상태를 **D(내 코드 × 트윈) 뒤에** 한 번 더.
        //   C 와 다르면 내 사본의 결과가 **자기 TLS 캐시·스크래치의 이력**에 좌우된다는 뜻(= 잔차의 뿌리).
        if ic != ie || !input_eq(&oc[..64], &oe[..64]) { M_TLS.fetch_add(1, Ordering::Relaxed); }
        if (dab || dac) && M_AB.load(Ordering::Relaxed) + M_AC.load(Ordering::Relaxed) <= 10 {
            log(format!("mx #{} idx A={} B={} C={} D={} | A={} B={} C={} D={}",
                M_CMP.load(Ordering::Relaxed), ia, ib, ic, id,
                input_desc(&oa[..64]), input_desc(&ob[..64]), input_desc(&oc[..64]), input_desc(&od[..64])));
        }
    }
    let (e_rnd0, e_scal0, e_sa0) = ENTRY_SAME.with(|c| c.get());
    // ★호출 후 상태 비교 — 진입이 같았는데 퇴장이 다르면 **이 호출이 발생 지점**이다.
    if e_rnd0 && e_scal0 && e_sa0 {
        let (ok, f) = post_state_same(s, m as *mut u8);
        PS_CMP.fetch_add(1, Ordering::Relaxed);
        if !ok {
            PS_DIFF.fetch_add(1, Ordering::Relaxed);
            for i in 0..6 { if !f[i] { PS_FIELD[i].fetch_add(1, Ordering::Relaxed); } }
            if !f[2] {
                let mut d = [false; 50];
                ctr_diff_mask(s, m as *mut u8, &mut d);
                // 진입도 같았으면 신규 발생 — 이것만이 발생 지점의 증거다.
                let fresh = ENTRY_CTR.with(|c| c.get());
                if fresh { CTR_FRESH_N.fetch_add(1, Ordering::Relaxed); }
                for k in 0..50usize {
                    if !d[k] { continue; }
                    CTR_ANY[k].fetch_add(1, Ordering::Relaxed);
                    if fresh { CTR_FRESH[k].fetch_add(1, Ordering::Relaxed); }
                }
            }
            if dec_same && rs { PS_DEC_OK.fetch_add(1, Ordering::Relaxed); }
            if mark_ps(s as usize) {
                PS_FIRST.fetch_add(1, Ordering::Relaxed);
                // ★small_action 이 갈렸으면 **실물 184B 를 찍는다** — 어느 변형을 골랐는지가 곳 갈라진 분기다.
                //   태그(첫 u64) + 페이로드 앞 40B 만. 미사용 변형 페이로드는 쓰레기라 더 보면 오독한다.
                if !f[1] && SA_DUMPED.fetch_add(1, Ordering::Relaxed) < 8 {
                    let hx = |p: *mut u8| -> String { (0..40).map(|k| format!("{:02x}", *p.add(0x2858 + k))).collect() };
                    log(format!("sa-dump t={} pl={:04x}\n   g={}\n   m={}", tick, pl, hx(s), hx(m as *mut u8)));
                }
                if PS_FIRST.load(Ordering::Relaxed) <= 10 {
                    log(format!("ps #{} 진입동일인데 퇴장다름 seed={:016x} t={} pl={:04x} | 스칼={} sa={} counters={} rnd={} failed={} events={} | dec_same={} rng_same={} g={} m={}",
                        PS_FIRST.load(Ordering::Relaxed), seed, tick, pl, f[0], f[1], f[2], f[3], f[4], f[5], dec_same, rs, input_desc(og), input_desc(&om)));
                }
            }
        }
    }
    let first = (!dec_same || !rs) && mark_diverged(s as usize);
    let (e_rnd, e_scal, e_sa) = ENTRY_SAME.with(|c| c.get());
    if first {
        FIRST_DIFF.fetch_add(1, Ordering::Relaxed);
        // 진입 상태(rng·명명 스칼라)가 같았는데 갈라졌다 = 순수 코드 동작 차이
        if e_rnd && e_scal { FIRST_CLEAN_ENTRY.fetch_add(1, Ordering::Relaxed); }
        plan_first_record();
        ag_first_record();
        plan_dump(s as *mut u8, m as *mut u8);
    }
    if first || ((!dec_same || !rs || !bs) && GI_DIFF.load(Ordering::Relaxed) < 8) {
        log(format!("gi #{} first={} entry(rng={} scal={} sa={}) seed={:016x} t={} pl={:04x} exp={} tid={} worker={} dec={} rng={}(idx {}->{}/{} res_same={} core_same={}) bb={} ok={} | g={} m={} | evlen g={} m={}\n   g={}\n   m={}", GI_CMP.load(Ordering::Relaxed), first, e_rnd, e_scal, e_sa, seed, tick, pl, exp, tid, worker, dec_same, rs, i0, ig, im, res_same, core_same, bs, ok, input_desc(og), input_desc(&om), og[0x38], om[0x38], hex(og), hex(&om)));
    }
    // 게임 쪽 출력(out)이 그대로 게임에 전달된다. 다이제스트도 게임 쪽 값으로 기록.
    super::agent_link::digest_record(out, player, data);
    leave();
}

unsafe fn update_on_dead_shim(s: *mut u8, rnd: *mut u8, player: *mut u8, data: *mut u8) {
    let f: F4 = std::mem::transmute(orig(S_DEAD));
    let m = match twin(s) { Some(m) => m, None => { f(s, rnd, player, data); return; } };
    let top = enter();
    if !top { f(s, rnd, player, data); leave(); return; }
    let bbp = bb_of(data);
    let s0 = Snap::take(rnd, bbp);
    f(s, rnd, player, data);
    let after_g = Snap::take(rnd, bbp);
    s0.restore();
    let g: F4 = std::mem::transmute(my(S_DEAD));
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| g(m as *mut u8, rnd, player, data)));
    let after_m = Snap::take(rnd, bbp);
    after_g.restore();
    DEAD_CMP.fetch_add(1, Ordering::Relaxed);
    let (rs, bs) = after_m.same_as(&after_g);
    if !rs || !bs { DEAD_DIFF.fetch_add(1, Ordering::Relaxed); log(format!("dead #{} rng={} bb={}", DEAD_CMP.load(Ordering::Relaxed), rs, bs)); }
    leave();
}

unsafe fn buy_item_shim(s: *mut u8, rnd: *mut u8, player: *mut u8, p3: *mut u8, p4: *mut u8, ctx: *mut u8) -> (usize, usize) {
    let f: F6r = std::mem::transmute(orig(S_BUY));
    let m = match twin(s) { Some(m) => m, None => return f(s, rnd, player, p3, p4, ctx) };
    let top = enter();
    if !top { let r = f(s, rnd, player, p3, p4, ctx); leave(); return r; }
    let s0 = Snap::take(rnd, 0);
    let rg = f(s, rnd, player, p3, p4, ctx);
    let after_g = Snap::take(rnd, 0);
    s0.restore();
    let g: F6r = std::mem::transmute(my(S_BUY));
    let rm = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| g(m as *mut u8, rnd, player, p3, p4, ctx))).unwrap_or((usize::MAX, usize::MAX));
    let after_m = Snap::take(rnd, 0);
    after_g.restore();
    BUY_CMP.fetch_add(1, Ordering::Relaxed);
    let rs = after_m.same_as(&after_g).0;
    // ★buy_item 반환도 `Option<usize>`(16B) — **None 이면 값은 미초기화 쓰레기**(일곱 번째 같은 함정).
    let ret_same = rg.0 == rm.0 && (rg.0 == 0 || rg.1 == rm.1);
    if !ret_same || !rs { BUY_DIFF.fetch_add(1, Ordering::Relaxed); log(format!("buy #{} g={:?} m={:?} rng_same={}", BUY_CMP.load(Ordering::Relaxed), rg, rm, rs)); }
    leave();
    rg
}

unsafe fn upgrade_item_shim(out: *mut u8, s: *mut u8, rnd: *mut u8, player: *mut u8, p4: *mut u8, p5: *mut u8, ctx: *mut u8) {
    let f: F7 = std::mem::transmute(orig(S_UPG));
    let m = match twin(s) { Some(m) => m, None => { f(out, s, rnd, player, p4, p5, ctx); return; } };
    let top = enter();
    if !top { f(out, s, rnd, player, p4, p5, ctx); leave(); return; }
    let s0 = Snap::take(rnd, 0);
    let mut om = [0u8; 24];
    core::ptr::copy_nonoverlapping(out, om.as_mut_ptr(), 24);   // 호출 전 내용으로 채움(None 의 payload 쓰레기 동일화)
    f(out, s, rnd, player, p4, p5, ctx);
    let after_g = Snap::take(rnd, 0);
    s0.restore();
    let g: F7 = std::mem::transmute(my(S_UPG));
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| g(om.as_mut_ptr(), m as *mut u8, rnd, player, p4, p5, ctx)));
    let after_m = Snap::take(rnd, 0);
    after_g.restore();
    UPG_CMP.fetch_add(1, Ordering::Relaxed);
    let og = std::slice::from_raw_parts(out, 24);
    let rs = after_m.same_as(&after_g).0;
    if og != &om[..] || !rs { UPG_DIFF.fetch_add(1, Ordering::Relaxed); log(format!("upg #{} g={} m={} rng_same={}", UPG_CMP.load(Ordering::Relaxed), hex(og), hex(&om), rs)); }
    leave();
}

pub fn report() -> String {
    let g = LOG.lock().unwrap_or_else(|e| e.into_inner());
    let live = TWIN.lock().unwrap_or_else(|e| e.into_inner()).as_ref().map(|m| m.len()).unwrap_or(0);
    let mut s = format!("[twin] factory={} rng_diff={} fail={} live={} drops={} clones={} clone_fail={} mirror={} | pairs={} **first_diff={} (진입상태 동일 {})** | get_input cmp={} DIFF={} (dec_only={} rng_only={}) rng_diff={} bb_diff={} notwin={} | presim cmp={} diff={} · worker cmp={} diff={} | state cmp={} rnd_diff={} scalar_diff={} small_action_diff={} plan_diff={} | buy cmp={} DIFF={} | upgrade cmp={} DIFF={} | dead cmp={} DIFF={}\n",
        F_CALLS.load(Ordering::Relaxed), F_RNG_DIFF.load(Ordering::Relaxed), F_FAIL.load(Ordering::Relaxed), live, DROPS.load(Ordering::Relaxed), CLONES.load(Ordering::Relaxed), CLONE_FAIL.load(Ordering::Relaxed), MIRROR.load(Ordering::Relaxed),
        PAIRS_SEEN.load(Ordering::Relaxed), FIRST_DIFF.load(Ordering::Relaxed), FIRST_CLEAN_ENTRY.load(Ordering::Relaxed), GI_CMP.load(Ordering::Relaxed), GI_DIFF.load(Ordering::Relaxed), GI_DEC_ONLY.load(Ordering::Relaxed), GI_RNG_ONLY.load(Ordering::Relaxed), GI_RNG_DIFF.load(Ordering::Relaxed), GI_BB_DIFF.load(Ordering::Relaxed), GI_NOTWIN.load(Ordering::Relaxed), GI_CMP_PRESIM.load(Ordering::Relaxed), GI_DIFF_PRESIM.load(Ordering::Relaxed), GI_CMP_WORKER.load(Ordering::Relaxed), GI_DIFF_WORKER.load(Ordering::Relaxed), ST_CMP.load(Ordering::Relaxed), ST_RNG.load(Ordering::Relaxed), ST_SCAL.load(Ordering::Relaxed), ST_SA.load(Ordering::Relaxed), ST_PLAN.load(Ordering::Relaxed),
        BUY_CMP.load(Ordering::Relaxed), BUY_DIFF.load(Ordering::Relaxed), UPG_CMP.load(Ordering::Relaxed), UPG_DIFF.load(Ordering::Relaxed), DEAD_CMP.load(Ordering::Relaxed), DEAD_DIFF.load(Ordering::Relaxed));
    s.push_str(&format!("[twin-mb] A!=B 순간의 다른 필드: {}
", plan_mb_report()));
    s.push_str(&format!("[twin-post] 진입동일 호출 {} 중 퇴장상태 다름 {} ({:.4}%) · 그중 결정은 같았던 것 {} · 쌍별 첫 발생 {} | 스칼={} sa={} counters={} rnd={} failed={} events={}\n",
        PS_CMP.load(Ordering::Relaxed), PS_DIFF.load(Ordering::Relaxed),
        100.0 * PS_DIFF.load(Ordering::Relaxed) as f64 / PS_CMP.load(Ordering::Relaxed).max(1) as f64,
        PS_DEC_OK.load(Ordering::Relaxed), PS_FIRST.load(Ordering::Relaxed),
        PS_FIELD[0].load(Ordering::Relaxed), PS_FIELD[1].load(Ordering::Relaxed), PS_FIELD[2].load(Ordering::Relaxed),
        PS_FIELD[3].load(Ordering::Relaxed), PS_FIELD[4].load(Ordering::Relaxed), PS_FIELD[5].load(Ordering::Relaxed)));
    {
        let fr = CTR_FRESH_N.load(Ordering::Relaxed);
        let mut v: Vec<(usize, usize, &str)> = (0..50)
            .map(|i| (CTR_FRESH[i].load(Ordering::Relaxed), CTR_ANY[i].load(Ordering::Relaxed), CTR_NAMES[i]))
            .filter(|x| x.1 > 0).collect();
        v.sort_by(|a, b| b.0.cmp(&a.0).then(b.1.cmp(&a.1)));
        s.push_str(&format!("[twin-ctr] 진입 counters 같았는데 퇴장에서 벌어진 호출 {} — 카운터별(신규 / 전체):\n", fr));
        for (fq, any, nm) in v.iter().take(16) {
            s.push_str(&format!("   {:<34} 신규 {:>8} · 전체 {:>9}\n", nm, fq, any));
        }
    }
    s.push_str(&birth_report());
    s.push_str(&format!("[twin-ag] {}\n", ag_field_report()));
    s.push_str(&format!("[twin-slot] 미미러링 슬롯 호출: {}\n", slot_report()));
    s.push_str(&format!("[twin-mx] 2x2 cmp={} | A≠B(내 상태가 다름)={} A≠C(내 코드가 다름)={} A≠D(둘 다)={} A!=A2(순서효과 통제)={} C!=C4(내 TLS 이력 의존)={}\n", M_CMP.load(Ordering::Relaxed), M_AB.load(Ordering::Relaxed), M_AC.load(Ordering::Relaxed), M_AD.load(Ordering::Relaxed), M_CC.load(Ordering::Relaxed), M_TLS.load(Ordering::Relaxed)));
    s.push_str(&format!("[twin-plan] {}\n[twin-plan-first] {}\n", plan_field_report(), plan_first_report()));
    for l in g.iter() { s.push_str(l); s.push('\n'); }
    s
}
