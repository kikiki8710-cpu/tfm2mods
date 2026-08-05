//! 훅 지점 — RE 정본 = ANA\discovered-banpick-ai.md §16 / §17k(0.5.3 마이그).
//! ★현행 = **게임 0.5.3 (buildid 24451609)**. 0.5.2 값은 MIGRATION §7.2-C 참조.
//!
//! ⚠**0.5.3 구조 변화**: 0.5.2의 A(`0x1cd9380` MSI phase getter)는 **함수째로 소멸**하고
//!   콜러들에 인라인화됐다. phase 디스패처 인라인 복제본이 11→30개로 늘었고, phase_from(B)의
//!   콜러도 26→3으로 줄었다. 대체 설계:
//!     · A 자리 = **씬 phase leaf `0x1bf3dd0`**(0.5.3 신설, 클라 23콜러) 전체 대체
//!     · 서버 AI턴(`0x1827e00`)의 인라인 phase = **바이트 패치**로 모드 phase 호출
//! A' `0x1bf3dd0` scene_step(&BanpickScene)->u8            : 전체 대체(0=밴 1=픽 2=완료 0xff)
//! B  `0x167c0e0` phase_from(total, rule, ban)->u8          : 전체 대체(완전 재구현)
//!    — B는 진입 7바이트째부터 rip-rel lea(점프테이블 디스패처)라 트램폴린 불가.
//!      전체 대체라 원본 재실행이 없으므로 12B 패치로 문제 없음.
//! G  `0x1828213` AI턴 인라인 phase                         : 바이트 패치 → 모드 phase_from
//! C  `0x1bd8c20` 클라 셀렉트 확정 적용기(void, 7인자)       : 트램폴린 detour + shim
//!    — 밴/픽 분류가 인라인: 픽 ⇔ (t1밴len(+0x148)==bc(+0x3c0)) && (t2밴len(+0x160)==bc).
//!      밴 차례는 무개입으로 올바름(밴 미완이면 반드시 밴 분류, 완료 전이는 마지막
//!      밴에서 자연 발화). "밴 완료 전의 픽 차례"만 +0x3c0(필요시 +0x160)을 일시
//!      동치화→원본→원복. 픽 분기는 밴 vec을 읽지 않아 일시 조작 안전(§16).
//!      부작용: 콜러가 호출 전에 고른 밴/픽 효과음이 어긋날 수 있음(연출 한정).
//!
//! 안전수칙: detour 본문 catch_unwind + Drop 원복 가드, 포인터 range 가드,
//! 훅 1회 설치(재설치 금지), 외부 훅 감지 시 설치 포기(덮어쓰기 금지 — 07-18 교훈),
//! 하나라도 실패하면 커스텀 시퀀스 전체 비활성(바닐라 재현으로 동작 = 무해).

use crate::config::{self, PH_B1, PH_P1, PH_P2};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};

// ── 0.5.4 RVA (패치 시 재핀 — 정본 = MODS\MIGRATION.md §15) ────────────────
const RVA_PHASE_SCENE: usize = 0x1dad900; // A' (0.5.3 0x1bf3dd0)
const RVA_PHASE_SCALAR: usize = 0x11bc7b0; // B  (0.5.3 0x167c0e0)
const RVA_APPLIER: usize = 0x1d92750; // C  (0.5.3 0x1bd8c20)

// 진입부 원본 바이트 (0.5.3 exe 실측 — 본 세션 채록)
/// A' 씬 phase leaf: mov rax,[rcx+0x160]; mov rdx,[rcx+0x178]; ...
const PROLOGUE_SCENE: &[u8] = &[
    0x48, 0x8B, 0x81, 0x60, 0x01, 0x00, 0x00, 0x48, 0x8B, 0x91, 0x78, 0x01,
];
const PROLOGUE_SCALAR: &[u8] = &[
    0x4D, 0x01, 0xC0, 0x0F, 0xB6, 0xC2, 0x48, 0x8D, 0x15, 0x0B, 0x6D, 0x11,
];
const PROLOGUE_APPLIER: &[u8] = &[
    0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x55, 0x53,
];
const ORIG_LEN: usize = 12;

// ── 클라 밴픽 씬 오프셋 (§16) ──────────────────────────────────────────────
const O_SC_T1BAN_LEN: usize = 0x148;
const O_SC_T2BAN_LEN: usize = 0x160;
const O_SC_T1PICK_LEN: usize = 0x178;
const O_SC_T2PICK_LEN: usize = 0x190;
const O_SC_BAN_COUNT: usize = 0x3c0;
const O_SC_RULE: usize = 0xce; // u8 game_rule (0=2v2..3=5v5)

// ── MatchSetInfo 오프셋 (§16) — 0.5.3 불변 확인. 0.5.3에선 MSI phase getter가 소멸해
//    모드가 직접 읽지는 않지만(AI턴 인라인 패치가 스택 사본을 씀) 계약 기록용으로 보존.
#[allow(dead_code)]
const O_MS_LEN_A: usize = 0x40;
#[allow(dead_code)]
const O_MS_LEN_B: usize = 0x58;
#[allow(dead_code)]
const O_MS_LEN_C: usize = 0x70;
#[allow(dead_code)]
const O_MS_LEN_D: usize = 0x88;
#[allow(dead_code)]
const O_MS_BAN_COUNT: usize = 0xf0; // u64
#[allow(dead_code)]
const O_MS_RULE: usize = 0xf9; // u8

/// 바닐라 픽 순서 테이블 (0.5.2 .rdata 0x38397a8 / 0.5.3 0x3277c70 실측 사본 — 완전 재구현
/// 원칙상 게임 데이터를 참조하지 않고 자체 보유. 0.5.3에서도 내용 동일 확인). 0=T1Pick, 1=T2Pick.
const PICK_TABLES: [&[u8]; 4] = [
    &[0, 1, 0, 1],
    &[0, 1, 1, 0, 0, 1],
    &[0, 1, 1, 0, 1, 0, 0, 1],
    &[0, 1, 1, 0, 0, 1, 1, 0, 0, 1],
];

// ── 상태 ──
static BASE: AtomicUsize = AtomicUsize::new(0);
static INSTALLED: AtomicBool = AtomicBool::new(false);
/// 3훅 전부 설치 성공했을 때만 true — 아니면 커스텀 시퀀스 미적용(바닐라 재현).
static CUSTOM_ACTIVE: AtomicBool = AtomicBool::new(false);
/// ★컨텍스트 게이트: 플레이어가 실제 밴픽 화면을 보고 있을 때만 true.
/// 이 함수들(0x1cd9380/0x1d04120)은 밴픽 진행 외에 **경기 진행 후 로스터 재구성·
/// 백그라운드 시뮬·타 경기**에도 공유된다. 화면이 없을 때 커스텀 순서를 적용하면
/// 로스터가 비대칭/미완성으로 매치 초기화에 넘어가 다운스트림 크래시(2026-07-26 실사고).
/// → false면 무조건 바닐라(원본 비트동일) 폴백. 매 프레임 lib.rs post_update가 갱신.
static IN_BANPICK: AtomicBool = AtomicBool::new(false);
/// ★내 매치 한정 게이트(2026-07-29): 화면 존재만으로 게이팅하면 **백그라운드 다른 매치의
/// AI 밴픽·시뮬**에도 커스텀 phase가 먹혀 AI 후보 계산이 깨진다(sgd_v2 빈 후보 → Rust
/// panic → __fastfail). 내 밴픽 셀렉트가 통과한 MatchSetInfo 포인터를 기억해, phase
/// 대체를 **그 포인터일 때만** 적용한다. 0 = 미확정(그땐 IN_BANPICK만으로 판정).
static MY_MSI: AtomicUsize = AtomicUsize::new(0);
/// 내 밴픽 씬 포인터(hook_applier가 캡처) — 매치 식별 상관 기준.
static MY_SCENE: AtomicUsize = AtomicUsize::new(0);
/// ★내 매치의 팀 ID 쌍(씬 +0x3d0/+0x3d8) — 매치 식별의 강한 키.
/// 진행수 상관만으로는 같은 룰·밴수의 백그라운드 매치가 통과해 오염된다(commit=74 실측).
static MY_T1: AtomicU64 = AtomicU64::new(0);
static MY_T2: AtomicU64 = AtomicU64::new(0);
static TRAMP_APPLIER: AtomicUsize = AtomicUsize::new(0);
// 진단 카운터 (post_update에서만 덤프 — detour 안 파일 I/O 금지)
static CNT_INFO: AtomicU64 = AtomicU64::new(0);
static CNT_SCALAR: AtomicU64 = AtomicU64::new(0);
static CNT_APPLIER: AtomicU64 = AtomicU64::new(0);
static CNT_FORCED_PICK: AtomicU64 = AtomicU64::new(0);
static FRAME: AtomicU64 = AtomicU64::new(0);
/// 턴 오라클(D′)이 "행동할 팀 없음"으로 빠진 사유별 카운터 — 진행 정지 진단용.
/// 전체 대체라 여기서 거절하면 그 경기의 밴픽이 영구 정지한다(2026-07-31 사고).
static REJ_RLEN: AtomicU64 = AtomicU64::new(0);
static REJ_STATE: AtomicU64 = AtomicU64::new(0);
/// 관측된 최대 레코드 개수 — 구 상한(64)이 실제로 초과되는지 확인용.
static MAX_RLEN: AtomicU64 = AtomicU64::new(0);
/// AI 밴 스코어러 파리티 훅 호출 수 / 그중 커스텀 팀비트를 돌려준 수.
/// ⚠커스텀 수가 밴픽 화면 밖에서 늘면 게이트 비대칭 = 백그라운드 경기 정지의 원인.
static CNT_AIBAN: AtomicU64 = AtomicU64::new(0);
static CNT_AIBAN_CUSTOM: AtomicU64 = AtomicU64::new(0);
/// 턴 오라클(D′) 총 호출 / "턴 있음(ok=1)" 반환 / 커스텀 팀비트를 쓴 횟수.
/// ★막힌 단계를 가르는 핵심 지표 — turn=0 이면 게임이 그 단계까지 오지도 못한 것,
/// turn 은 느는데 ok 가 안 늘면 오라클이 거절 중, ok 는 느는데 commit 이 0 이면
/// 턴은 나왔는데 커밋 단계로 못 넘어간 것이다.
static CNT_TURN: AtomicU64 = AtomicU64::new(0);
static CNT_TURN_OK: AtomicU64 = AtomicU64::new(0);
static CNT_TURN_CUSTOM: AtomicU64 = AtomicU64::new(0);
/// 커밋 거부(반환 0) 수 · "같은 경기·같은 진행수"에서 커밋이 연속으로 반복된 최대 횟수.
/// 후자가 크게 튀면 그 경기는 제자리를 맴돌고 있다(= 일정 정지의 직접 지문).
static CNT_COMMIT_REJ: AtomicU64 = AtomicU64::new(0);
static LAST_COMMIT_KEY: AtomicU64 = AtomicU64::new(0);
static SAME_COMMIT_RUN: AtomicU64 = AtomicU64::new(0);
static MAX_SAME_COMMIT: AtomicU64 = AtomicU64::new(0);

// ── 재귀 폭주 가드 ──────────────────────────────────────────────────────────
// 가설: 밴↔픽 인터리브에서 AI 시뮬이 phase 함수를 재귀 폭주 호출 → 스택 오버플로
// (VEH가 못 잡는 크래시 = 스택 소진). 스레드별 재진입 깊이를 세서 임계 초과 시
// 0xFF(종료)로 재귀를 끊고 트립 카운트를 기록한다. 임계 = 정상 sim보다 훨씬 깊게.
thread_local! {
    static PHASE_DEPTH: std::cell::Cell<u32> = const { std::cell::Cell::new(0) };
}
static RECURSION_TRIP: AtomicU64 = AtomicU64::new(0);
static MAX_DEPTH_SEEN: AtomicU64 = AtomicU64::new(0);
const DEPTH_LIMIT: u32 = 96;

const MEM_CR: u32 = 0x1000 | 0x2000;
const RWX: u32 = 0x40;
#[link(name = "kernel32")]
extern "system" {
    fn GetModuleHandleW(n: *const u16) -> usize;
    fn VirtualAlloc(addr: usize, size: usize, ty: u32, prot: u32) -> usize;
    fn VirtualProtect(addr: usize, size: usize, prot: u32, old: *mut u32) -> u32;
    fn FlushInstructionCache(h: isize, addr: usize, size: usize) -> u32;
    fn GetCurrentProcess() -> isize;
}

#[inline]
fn addr_ok(a: usize) -> bool {
    (0x10000..1usize << 48).contains(&a)
}

#[inline]
unsafe fn ru64(a: usize) -> u64 {
    core::ptr::read(a as *const u64)
}

#[inline]
unsafe fn ru8(a: usize) -> u8 {
    core::ptr::read(a as *const u8)
}

/// 재귀 진입 — 깊이 반환(1부터). 임계 초과면 None(호출자는 0xFF로 즉시 종료).
#[inline]
fn depth_enter() -> Option<u32> {
    PHASE_DEPTH.with(|c| {
        let d = c.get().wrapping_add(1);
        c.set(d);
        // 최대 관측 깊이 갱신(진단)
        let mut m = MAX_DEPTH_SEEN.load(Ordering::Relaxed);
        while (d as u64) > m {
            match MAX_DEPTH_SEEN.compare_exchange_weak(
                m, d as u64, Ordering::Relaxed, Ordering::Relaxed,
            ) {
                Ok(_) => break,
                Err(x) => m = x,
            }
        }
        if d > DEPTH_LIMIT {
            RECURSION_TRIP.fetch_add(1, Ordering::Relaxed);
            None
        } else {
            Some(d)
        }
    })
}

#[inline]
fn depth_exit() {
    PHASE_DEPTH.with(|c| c.set(c.get().wrapping_sub(1)));
}

// ── 순서 계산 (공용, 패닉·할당 없음) ───────────────────────────────────────
/// 바닐라 완전 재현: 밴 전부 선행(T1부터 교대) → 룰별 픽테이블. game==mine.
#[inline]
fn vanilla_phase(total: u64, rule: u8, ban: u64) -> u8 {
    let r = (rule & 3) as usize;
    let ppt = r as u64 + 2;
    if total >= 2 * ban + 2 * ppt {
        return 0xFF;
    }
    if total < 2 * ban {
        return PH_B1 | (total as u8 & 1);
    }
    PICK_TABLES[r]
        .get((total - 2 * ban) as usize)
        .copied()
        .unwrap_or(0xFF)
}

/// ★진행 축(phase·턴 오라클·커밋·AI 밴 파리티)의 **단일 게이트**.
/// 이 넷은 반드시 같은 판단을 써야 한다 — 하나라도 다른 순서를 기준으로 움직이면 그 경기가
/// 밴 단계를 못 빠져나오고 커밋만 반복해 시즌 일정이 멈춘다(2026-08-01 실사고).
///
/// `apply_all`(기본 ON) = 밴픽 화면 여부와 무관하게 **모든 경기**(백그라운드 AI 경기 포함)에
/// 커스텀 순서 적용. OFF면 구 동작(화면이 있을 때만).
/// ⚠구 IN_BANPICK 게이트의 근거였던 "백그라운드 커스텀 적용 = 로스터 오염 크래시"(2026-07-26)는
/// **오진으로 판명**됐다(진범 = `0x11cedb0` unwrap(None) → 훅 E로 해결, 메모리 §10).
#[inline]
fn custom_ctx() -> bool {
    CUSTOM_ACTIVE.load(Ordering::Relaxed)
        && (config::get().apply_all || IN_BANPICK.load(Ordering::Relaxed))
}

#[inline]
fn phase_of(total: u64, rule: u8, ban: u64) -> u8 {
    if custom_ctx() {
        if let Some(seq) = config::get().seq_for(rule, ban) {
            // 검증 통과 시퀀스는 길이 == 2*ban + 2*(rule+2) 보장 → 종료판정 자동 일치
            return seq.get(total as usize).copied().unwrap_or(0xFF);
        }
    }
    vanilla_phase(total, rule, ban)
}

/// lib.rs 가 UI 흰칸(`in_turn` 노드)을 직접 제어하기 위한 현재 상태 조회.
/// 반환 = (phase, t1밴수, t2밴수, t1픽수, t2픽수). 밴픽 화면·씬 확보 상태에서만 Some.
///
/// ★배경(2026-07-30): 하단 슬롯 렌더러(0.5.3 `0x1bf9560` / 0.5.2 `0x12028b0`)는 **씬을
/// 읽지 않는 순수 렌더러**라 phase 를 아무리 정확히 넘겨도 흰칸에 닿지 않는다. 흰칸은
/// UI 노드 `…/in_turn` 의 visible 로 표현되므로, 순서를 아는 모드가 **SDK 로 직접** 켜고
/// 끄는 게 정공법(게임 코드 패치 불필요·순서 로직과 완전 분리).
pub fn ui_turn_state() -> Option<(u8, u64, u64, u64, u64, u8)> {
    if !IN_BANPICK.load(Ordering::Relaxed) || !CUSTOM_ACTIVE.load(Ordering::Relaxed) {
        return None;
    }
    let sc = MY_SCENE.load(Ordering::Relaxed);
    if sc == 0 || !addr_ok(sc) {
        return None;
    }
    unsafe {
        let (t1b, t2b) = (ru64(sc + O_SC_T1BAN_LEN), ru64(sc + O_SC_T2BAN_LEN));
        let (t1p, t2p) = (ru64(sc + O_SC_T1PICK_LEN), ru64(sc + O_SC_T2PICK_LEN));
        if t1b > 15 || t2b > 15 || t1p > 10 || t2p > 10 {
            return None;
        }
        let total = t1b.wrapping_add(t2b).wrapping_add(t1p).wrapping_add(t2p);
        let ban = ru64(sc + O_SC_BAN_COUNT);
        let rule = ru8(sc + O_SC_RULE);
        // 커스텀 시퀀스가 없으면(바닐라 폴백) 게임 기본 표시를 그대로 두는 게 맞다.
        let seq = config::get().seq_for(rule, ban)?;
        // 다음 스텝의 phase — 같은 팀·같은 타입이 연속이면 그 칸을 "다음 차례(흰색)"로 칠한다.
        let next = seq.get(total.wrapping_add(1) as usize).copied().unwrap_or(0xFF);
        Some((phase_of(total, rule, ban), t1b, t2b, t1p, t2p, next))
    }
}

/// lib.rs post_update가 매 프레임 호출 — 밴픽 컨테이너 노드 존재 여부.
pub fn set_in_banpick(v: bool) {
    IN_BANPICK.store(v, Ordering::Relaxed);
}

/// 내 매치 판정(내용 대조) — 씬(MY_SCENE)의 밴수·룰·진행수가 인자와 일치하는가.
/// 포인터 동일성은 못 쓴다(AI턴이 레코드 스택 clone을 넘김). MY_SCENE 미확정이면
/// 아직 첫 셀렉트 전이므로 통과시킨다(밴픽 화면 게이트가 이미 걸려 있음).
fn scene_matches(ban: u64, rule: u8, total: u64) -> bool {
    let sc = MY_SCENE.load(Ordering::Relaxed);
    if sc == 0 {
        return true;
    }
    if !addr_ok(sc) {
        return false;
    }
    unsafe {
        let s_total = ru64(sc + O_SC_T1BAN_LEN)
            .wrapping_add(ru64(sc + O_SC_T2BAN_LEN))
            .wrapping_add(ru64(sc + O_SC_T1PICK_LEN))
            .wrapping_add(ru64(sc + O_SC_T2PICK_LEN));
        ru64(sc + O_SC_BAN_COUNT) == ban && ru8(sc + O_SC_RULE) == rule && s_total == total
    }
}

// ── 훅 O: 슬롯 색 적용기 param_7 오버라이드 (★칸 채움색의 진짜 스위치) ──────
// ★딥 RE 결과(2026-07-30, Ghidra 0.5.3):
//   `0x1c252c0(slot_node rcx, .., ..., ..., .., .., u8 param_7)` 은 param_7 로 **색 세트**를
//   고른다: `uVar17 = param_7 ? DAT_1430f5c84 : DAT_1432956f0` … 를 노드의
//   +0x84/+0x104/+0x184/+0x204(4상태 색 필드)에 기록 = 칸 채움색.
//   호출부는 정적 상수 두 곳뿐: `0x1bfcbf2 mov byte [rsp+0x30],1`(강조) /
//   `0x1bfd554 …,0`(기본). 게임은 **바닐라 순서**로 어느 경로를 탈지 정하므로 인터리브에서
//   엉뚱한 칸이 칠해진다(유저 실측: 3픽째부터 빨강, 2차 밴 구간에도 4픽 빨강).
// ⟹ 게임의 색 적용 로직은 그대로 두고 **판정(param_7)만** 모드가 덮어쓴다.
//   슬롯 식별 = param_1(슬롯 노드 포인터)를 lib.rs 가 매 프레임 채우는 표와 대조.
const RVA_SLOTUPD: usize = 0x1ddff30; // 0.5.3 0x1c252c0
const PROLOGUE_SLOTUPD: &[u8] = &[
    0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x53,
];
type SlotUpdFn = unsafe extern "win64" fn(usize, usize, usize, usize, usize, usize, u8);
static TRAMP_SLOTUPD: AtomicUsize = AtomicUsize::new(0);
static CNT_SLOTUPD: AtomicU64 = AtomicU64::new(0);
static CNT_SLOTUPD_OV: AtomicU64 = AtomicU64::new(0);

/// lib.rs 가 매 프레임 채우는 슬롯 표: (노드 포인터, 강조 여부). 최대 24칸.
pub const SLOT_TBL_N: usize = 24;
pub static SLOT_PTR: [AtomicUsize; SLOT_TBL_N] = [const { AtomicUsize::new(0) }; SLOT_TBL_N];
pub static SLOT_CUR: [AtomicU64; SLOT_TBL_N] = [const { AtomicU64::new(0) }; SLOT_TBL_N];

/// lib.rs 진입점 — 표를 통째로 갱신(프레임마다).
pub fn set_slot_table(entries: &[(usize, bool)]) {
    for i in 0..SLOT_TBL_N {
        match entries.get(i) {
            Some(&(p, cur)) => {
                SLOT_PTR[i].store(p, Ordering::Relaxed);
                SLOT_CUR[i].store(cur as u64, Ordering::Relaxed);
            }
            None => SLOT_PTR[i].store(0, Ordering::Relaxed),
        }
    }
}

unsafe extern "win64" fn hook_slotupd(
    p1: usize,
    p2: usize,
    p3: usize,
    p4: usize,
    p5: usize,
    p6: usize,
    p7: u8,
) {
    CNT_SLOTUPD.fetch_add(1, Ordering::Relaxed);
    let stub = TRAMP_SLOTUPD.load(Ordering::Relaxed);
    if stub == 0 {
        return;
    }
    let orig: SlotUpdFn = core::mem::transmute(stub);
    let mut want = p7;
    let _ = catch_unwind(AssertUnwindSafe(|| {
        if !IN_BANPICK.load(Ordering::Relaxed) || !CUSTOM_ACTIVE.load(Ordering::Relaxed) {
            return;
        }
        for i in 0..SLOT_TBL_N {
            let ptr = SLOT_PTR[i].load(Ordering::Relaxed);
            if ptr == 0 {
                continue;
            }
            if ptr == p1 {
                let cur = SLOT_CUR[i].load(Ordering::Relaxed) != 0;
                want = cur as u8;
                if want != p7 {
                    CNT_SLOTUPD_OV.fetch_add(1, Ordering::Relaxed);
                }
                return;
            }
        }
    }));
    orig(p1, p2, p3, p4, p5, p6, want);
}

// ── 훅 N: 씬 기반 **원시 phase** leaf (0.5.3 신설·A′와 별개) ─────────────────
// ★2026-07-30 규명: 0.5.3 에는 씬→phase leaf 가 **두 개**다.
//   · `0x1bf3dd0` = 단계 enum(0=밴 1=픽 2=완료 0xff) 반환  → 훅 A′
//   · `0x1bce8e0` = **원시 phase(0..3, 0xff)** 반환         → 이 훅 N (그동안 미후킹)
// 소비처(`0x2262ca0`@`0x2262f25`)는 `lea ecx,[rax-2]; cmp ecx,2` 로 "밴 단계인가"를
// 가려 슬롯 표시를 고른다. 0.5.2 에서는 이 계산이 phase_from(훅 B)로 갔기에 커스텀
// 순서를 자동 추종했고(=유저 증언 "0.5.2 에선 문제없었다"), 0.5.3 에서 이 leaf 로
// 분리되면서 훅이 비어 흰칸만 바닐라로 남았다.
const RVA_PHASE_RAW: usize = 0x1d88160; // 0.5.3 0x1bce8e0
const PROLOGUE_PHASE_RAW: &[u8] = &[
    0x48, 0x8B, 0x91, 0x60, 0x01, 0x00, 0x00, 0x48, 0x03, 0x91, 0x48, 0x01,
];

/// 훅 N 본체 — 씬에서 total/rule/ban 을 읽어 **원시 phase** 를 그대로 반환.
/// (A′ 와 입력은 같고 출력 규약만 다르다 — enum 사상 없음.)
unsafe extern "win64" fn hook_phase_raw(scene: usize) -> u8 {
    CNT_INFO.fetch_add(1, Ordering::Relaxed);
    if !addr_ok(scene) {
        return 0xFF;
    }
    if depth_enter().is_none() {
        depth_exit();
        return 0xFF;
    }
    let total = ru64(scene.wrapping_add(O_SC_T1BAN_LEN))
        .wrapping_add(ru64(scene.wrapping_add(O_SC_T2BAN_LEN)))
        .wrapping_add(ru64(scene.wrapping_add(O_SC_T1PICK_LEN)))
        .wrapping_add(ru64(scene.wrapping_add(O_SC_T2PICK_LEN)));
    let ban = ru64(scene.wrapping_add(O_SC_BAN_COUNT));
    let rule = ru8(scene.wrapping_add(O_SC_RULE));
    let r = phase_of(total, rule, ban);
    depth_exit();
    r
}

// ── 훅 A': 씬 단계 판정 scene_step(&BanpickScene) (0.5.3 신설 leaf, 클라 23콜러) ──
// 원본 계약(0.5.3 `0x1bf3dd0` 실측):
//   total = +0x148 + +0x160 + +0x178 + +0x190, rule = +0xce, ban = +0x3c0
//   · 밴 단계(total < 2*ban)            → 0
//   · 픽 단계(픽테이블 범위 내)          → 1
//   · 그 외(진행 종료)                   → t1pick == rule+2 && t2pick == t1pick 이면 2, 아니면 0xff
// 반환은 phase 그 자체가 아니라 **단계 종류 enum**이라 커스텀 phase를 이 enum으로 사상한다.
unsafe extern "win64" fn hook_phase_scene(scene: usize) -> u8 {
    CNT_INFO.fetch_add(1, Ordering::Relaxed);
    if !addr_ok(scene) {
        return 0xFF;
    }
    // 재귀 폭주 가드 — 임계 초과면 종료(0xFF)로 스택 소진 방지.
    if depth_enter().is_none() {
        depth_exit();
        return 0xFF;
    }
    // 원본과 동일한 필드 접근(원본도 같은 주소를 직접 deref) — 추가 위험 없음.
    let t1p = ru64(scene.wrapping_add(O_SC_T1PICK_LEN));
    let t2p = ru64(scene.wrapping_add(O_SC_T2PICK_LEN));
    let total = ru64(scene.wrapping_add(O_SC_T1BAN_LEN))
        .wrapping_add(ru64(scene.wrapping_add(O_SC_T2BAN_LEN)))
        .wrapping_add(t1p)
        .wrapping_add(t2p);
    let ban = ru64(scene.wrapping_add(O_SC_BAN_COUNT));
    let rule = ru8(scene.wrapping_add(O_SC_RULE));
    // ★매치 한정 게이트 없음(2026-07-29 유저 확정 — 모든 경기에 동일 순서 적용이 의도).
    // ⚠이력: 포인터 비교 게이트(MY_MSI)는 AI턴이 레코드 스택 clone을 넘겨 내 매치조차
    // "남의 매치"로 오판 → 순서함수만 바닐라 폴백 → 커밋훅(커스텀 팀)과 어긋나 내 픽↔상대
    // 픽이 꼬였다. 게이트는 seq_for(rule,ban) 유무로 충분(없으면 바닐라 비트동일).
    crate::diag::CTX_LAST_TOTAL.store(total as u32, Ordering::Relaxed);
    let ph = phase_of(total, rule, ban);
    let r = if ph == 0xFF {
        let per = rule.wrapping_add(2) as u64;
        if t1p != per || t2p != t1p {
            0xFF
        } else {
            2
        }
    } else if ph & 2 != 0 {
        0 // 밴 단계
    } else {
        1 // 픽 단계
    };
    if config::get().debug {
        probe_rec(1, total, rule, ban, r);
    }
    depth_exit();
    r
}

// ── 훅 B: phase_from(total, rule(dl), ban) ────────────────────────────────
unsafe extern "win64" fn hook_phase_scalar(total: usize, rdx: usize, ban: usize) -> u8 {
    CNT_SCALAR.fetch_add(1, Ordering::Relaxed);
    if depth_enter().is_none() {
        depth_exit();
        return 0xFF;
    }
    let r = phase_of(total as u64, rdx as u8, ban as u64);
    if config::get().debug {
        probe_rec(0, total as u64, rdx as u8, ban as u64, r);
    }
    depth_exit();
    r
}

// ── 훅 C: 클라 셀렉트 확정 적용기 = 재구성기 방식 재구현 ────────────────────
// 원본 0x11e2140은 type을 ban-fullness(양팀 밴벡터가 다 차야 픽)로 판정해 밴↔픽
// 인터리브 시 밴 미완 픽을 밴 벡터로 오저장 → 로스터 오염 → 전환 크래시.
// 재구성기 0x11dd200이 쓰는 4개 버킷 appender를 직접 호출하면 ban-fullness를 우회,
// 커스텀 seq[position]의 (밴/픽·팀)대로 올바른 벡터에 넣는다(검증된 청사진 = §17 appender절).
type ApplierFn = unsafe extern "win64" fn(usize, usize, usize, usize, usize, usize, u8);
/// 버킷 appender: (scene, ui, champ_ptr, champ_len). champ는 내부에서 복사 소유(빌림 안전).
type AppendFn = unsafe extern "win64" fn(usize, usize, *const u8, usize);
/// 매치 전환: (scene, ui, p3). 로스터 len assert 없음(§17).
type TransitionFn = unsafe extern "win64" fn(usize, usize, usize);

// 0.5.3 재핀(버킷별 씬 오프셋 지문으로 확정: pick_t1 +0x168/0x170/0x178 · pick_t2 +0x180/
// 0x188/0x190 · ban_t1 +0x138/0x140/0x148 · ban_t2 +0x150/0x158/0x160)
const RVA_APP_PICK_T1: usize = 0x1d7e070; // 0.5.3 0x1bc47f0
const RVA_APP_PICK_T2: usize = 0x1d7e200; // 0.5.3 0x1bc4980
const RVA_APP_BAN_T1: usize = 0x1dbc290; // 0.5.3 0x1c028d0
const RVA_APP_BAN_T2: usize = 0x1dbc410; // 0.5.3 0x1c02a50
#[allow(dead_code)]
const RVA_TRANSITION: usize = 0x1d88900; // 0.5.3 0x1bcf010 (직접 호출 안 함 — 참고용)

// ── 단계 배너 (금지/선택 단계 애니메이션) ──────────────────────────────────
// 배너는 씬의 연출 FSM(scene+0x380 state, +0x384 타이머)이 그린다. update 0x1250370의
// switch(+0x380): arm0 = "금지 단계"(ban_step 텍스트·노드표시·재생 전부 수행) → 끝나면
// 스스로 state2/+0x43e=0 복원. arm3 = "선택 단계"(0x11df9f0가 세팅) → 끝나면 state4.
// ★게임에는 "밴→픽" 배너 함수(0x11df9f0)만 있고 **픽→밴 재진입 경로가 없다** → 모드가
//   `+0x380=0`(u64, 타이머 동시 클리어) + `+0x43e=1`(FSM 게이트)만 쓰면 arm0이 재발동.
// ⚠두 write는 반드시 세트(+0x380 먼저): +0x43e=1만 쓰고 state가 2/4면 어느 arm도 래치를
//   못 풀어 영구 소프트락. +0x446은 절대 건드리지 말 것(1이면 즉시 경기 전환).
// 부수 이득: 어드밴스 게이트가 state(2=밴 idle/4=픽 idle)와 phase 일치를 요구하므로,
//   경계마다 올바른 배너를 태우면 UI 리빌드 스테일도 해소된다.
// ── AI 밴 스코어러 인라인 phase 패치 (파리티 = 팀 사이드 셀렉터) ───────────
// 밴 스코어러 2곳이 phase를 **인라인 복제**해 바닐라 순서를 재가정한다. 그 산출물은
// 단순 가중치가 아니라 **"행동 팀이 T1인가"** 불리언이며, 플랜 스코어러 0x1bbc670에서
// 8슬롯 레인 배열의 **어느 팀 절반을 읽을지 고르는 사이드 셀렉터**로 쓰인다.
// ⇒ 인터리브에선 후행 밴(T1 차례)에서 **상대 팀 통계로 자기 밴을 평가**한다.
// 해결: 인라인 계산 구간을 모드가 대체한 phase_from(0x1d04120) 호출로 치환.
//   site1 0x1c04389 (합류 0x1c04475, `cmp cl,2`) / site2 0x1c07938 (합류 0x1c07a09, `cmp al,2`)
//   phase_from은 r8=ban_count를 받아 내부에서 2배(진입 `add r8,r8`)하므로 그대로 전달.
// 0.5.3 재핀: 컨테이너 0x1c041c0→0x10a0320 / 0x1c07880→0x10a3c40 (SIG 바이트 동일·유일 히트)
// ★0.5.4 재핀(2026-08-05): 컨테이너 0x10a0320→0x149e380 / 0x10a3c40→0x14a1e60.
// ⚠**두 사이트 모두 레지스터 배치가 바뀌었다** — site1 out cl→dl·rule cl→dl·total rdx→r9
// (+ 디스패처 직전 스토어 2개 신설), site2 out al→cl·ban rdx→r8. emit 본문도 같이 교체함.
const RVA_AI_SITE1: usize = 0x149e561; // 0.5.3 0x10a04e2
const RVA_AI_JOIN1: usize = 0x149e680; // 0.5.3 0x10a05f0
const RVA_AI_SITE2: usize = 0x14a1f1e; // 0.5.3 0x10a3cf8
const RVA_AI_JOIN2: usize = 0x14a1fef; // 0.5.3 0x10a3dc9
/// 패치 전 원본 프리픽스(실측) — 불일치 시 패치 포기.
const AI_SIG1: &[u8] = &[0x4c, 0x8b, 0x5b, 0x10, 0x4f, 0x8d, 0x0c, 0x33, 0x4d, 0x01, 0xc0, 0x0f];
const AI_SIG2: &[u8] = &[
    0x4c, 0x8b, 0x85, 0x20, 0x01, 0x00, 0x00, 0x4d, 0x8b, 0x64, 0x24, 0x10,
];
static AI_PATCHED: AtomicU64 = AtomicU64::new(0);

// ── 훅 G: 서버 AI턴 인라인 phase 패치 (0.5.3 신설 — 0.5.2 훅 A의 역할 승계) ─────────
// 0.5.2에서는 AI턴 `0xebe530`이 phase getter A(`0x1cd9380`)를 **호출**했기에 A 전체 대체로
// 커스텀 순서가 반영됐다. 0.5.3에서는 그 계산이 AI턴 함수 `0x1827e00`에 **인라인**됐다
// (MSI 스택 사본 사용: total=[rbp+0x5eb0], rule=[rbp+0x5d61], ban=[rbp+0x5d58]).
// ⇒ 인라인 디스패처 구간 [0x1828213, 0x18282fa) 231B를 모드 phase_from 호출로 치환한다.
//   합류점 `0x18282fa`는 `mov [rbp+0x5ebf], al` 이므로 반환 al 이 그대로 소비된다.
//   합류 직후 곧바로 턴 오라클을 call 하므로 volatile 레지스터 생존 없음(클로버 안전).
// 0.5.4 재핀: 컨테이너 0x1827e00→0x211dd40. 창 231B·스택슬롯(0x5eb0/0x5d61/0x5d58/0x5ebf) 전부 불변.
const RVA_AITURN_SITE: usize = 0x211e14f; // 0.5.3 0x1828213
const RVA_AITURN_JOIN: usize = 0x211e236; // 0.5.3 0x18282fa
/// 원본 프리픽스: movzx ecx,[rbp+0x5d61] / mov rax,[rbp+0x5d58]
const AITURN_SIG: &[u8] = &[
    0x0f, 0xb6, 0x8d, 0x61, 0x5d, 0x00, 0x00, 0x48, 0x8b, 0x85, 0x58, 0x5d, 0x00, 0x00,
];
static AITURN_PATCHED: AtomicBool = AtomicBool::new(false);

// ── 셀렉트 효과음 분류 패치 ────────────────────────────────────────────────
// 드레인 0x1250370 안(0x1251303~0x1251352)에서 밴/픽 효과음 문자열을 고르는데,
// 판정이 **ban-fullness**(양 밴벡터 == ban_count)라 인터리브 1차 픽 구간에서
// "밴 소리"가 난다(실측). 이 구간을 모드 함수 호출로 치환해 seq를 따르게 한다.
//   원본: r8 = 문자열 ptr, r9 = 길이(ban 0x1c / pick 0x1d) 세팅 후 0x1251352로 진행.
// 0.5.3 재핀: 드레인 컨테이너 0x1250370→0x1c55300, 창 크기 79B 동일. 씬 스택슬롯 0x12b0→0x12d0.
const RVA_SFX_SITE: usize = 0x1e1a575; // 0.5.3 0x1c56245
const RVA_SFX_END: usize = 0x1e1a5c4; // 0.5.3 0x1c56294
const RVA_STR_BAN: usize = 0x3392398; // "asset/base/sound/sfx/ban_sfx"  (0x1c)
const RVA_STR_PICK: usize = 0x33923b4; // "asset/base/sound/sfx/pick_sfx" (0x1d)
/// 패치 전 원본 프리픽스(실측): mov rcx,[rbp+0x12f0] / mov rax,[rcx+0x3c0]
/// ★0.5.4: 드레인 프레임이 0x1448→0x1468 로 커져 씬 스택슬롯 0x12d0 → **0x12f0**.
const SFX_SIG: &[u8] = &[
    0x48, 0x8b, 0x8d, 0xf0, 0x12, 0x00, 0x00, 0x48, 0x8b, 0x81, 0xc0, 0x03, 0x00, 0x00,
];
static SFX_PATCHED: AtomicBool = AtomicBool::new(false);

/// 이번 셀렉트가 픽인가(1) 밴인가(0) — 효과음 선택용. seq 없으면 바닐라 규칙 폴백.
unsafe extern "win64" fn sfx_is_pick(scene: usize) -> u64 {
    let mut r = 0u64;
    let _ = catch_unwind(AssertUnwindSafe(|| {
        if !addr_ok(scene) {
            return;
        }
        let t1b = ru64(scene + O_SC_T1BAN_LEN);
        let t2b = ru64(scene + O_SC_T2BAN_LEN);
        let bc = ru64(scene + O_SC_BAN_COUNT);
        let rule = ru8(scene + O_SC_RULE);
        // 커스텀 순서가 있으면 그 위치의 타입으로 판단.
        if IN_BANPICK.load(Ordering::Relaxed) && CUSTOM_ACTIVE.load(Ordering::Relaxed) {
            if let Some(seq) = config::get().seq_for(rule, bc) {
                let pos = t1b
                    .wrapping_add(t2b)
                    .wrapping_add(ru64(scene + O_SC_T1PICK_LEN))
                    .wrapping_add(ru64(scene + O_SC_T2PICK_LEN));
                if let Some(&ph) = seq.get(pos as usize) {
                    if ph != 0xFF {
                        r = if ph & 2 == 0 { 1 } else { 0 };
                        return;
                    }
                }
            }
        }
        // 바닐라: 양 밴벡터가 다 차면 픽.
        r = (t1b == bc && t2b == bc) as u64;
    }));
    r
}

const RVA_BANNER: usize = 0x1d8fc90; // 0.5.3 0x1bd63a0 (호출 전용 — 프롤로그 변경 무관)
type BannerFn = unsafe extern "win64" fn(usize, usize, usize, u8);
const O_SC_FSM_STATE: usize = 0x380; // u64 (state + 타이머 0x384 동시 클리어)
const O_SC_FSM_LATCH: usize = 0x43e; // u8
const O_SC_FSM_DEFER: usize = 0x446; // u8 (0=끝나면 픽배너, 1=끝나면 전환, 2=없음)
const O_SC_CARD_ANIM: usize = 0x348; // i64 (-1 = 카드 연출 없음/종료)
static CNT_BANNER: AtomicU64 = AtomicU64::new(0);
/// 픽→밴 배너 예약 — 카드 연출이 끝난 뒤 띄우기 위해 tick()에서 폴링.
/// (게임엔 밴 배너 지연 경로가 없어 모드가 직접 대기해야 한다. 픽 배너는 +0x446=0으로
///  게임 드레인에 위임 가능하지만 밴은 그 경로가 없음.)
static PENDING_BAN_BANNER: AtomicBool = AtomicBool::new(false);
/// 예약 대기 프레임 수 — 연출 종료 신호가 끝내 안 올 때 소프트락 방지용 타임아웃.
static PENDING_FRAMES: AtomicU64 = AtomicU64::new(0);
// 코치 위임 정지 워치독 상태(tick 참조 — 정본 MIGRATION §7.3 §14.6)
static WD_LAST_TOTAL: AtomicU64 = AtomicU64::new(u64::MAX);
static WD_FRAMES: AtomicU64 = AtomicU64::new(0);
static WD_KICKS: AtomicU64 = AtomicU64::new(0);

// ── 레코드 컨테이너 후킹 (D: 진단/정규화) ──────────────────────────────────
// acting_team 헬퍼 0x1d07cf0의 rcx = 밴픽 이력 레코드 컨테이너([rcx+8]=base,[rcx+0x10]
// =count, stride 0x100, rec+0xf8=side, rec+0xf9=step). 진입부가 조건분기(jz)라 특수
// 트램폴린(분기 절대점프 재배치, install_container). 매 밴픽 액션 호출.
// ── 훅 E: 라인업 적용기 (크래시 진범 회피) ─────────────────────────────────
// 0x11cedb0 = banpick_scene__apply_lineup(scene, team_a, team_b, *Vec<Rec>A, *Vec<Rec>B).
// 드래프트 확정 시 서버가 보내는 "팀별 최종 라인업" 패킷을 처리해 레인별 표시를 구성한다.
// 각 Rec(0x28B: position@8, String@0x10)의 챔프명을 **그 팀 픽벡터에서 position()으로 찾아**
// out[slot]=idx 를 쓰는데, **못 찾으면 Option::unwrap() on None → panic → __fastfail**
// (match_ui.rs:4181, 사이트 0x11cf5de = 런타임 훅으로 확정).
// 인터리브에서는 적용기의 팀 배정이 게임 파리티에 종속이라 일부 픽이 반대 팀 벡터로
// 들어가 이름을 못 찾는다 → 이 패닉. 회피: 모든 이름이 해당 팀 픽벡터에 있을 때만 원본
// 실행, 아니면 스킵(레인 표시만 스테일, 밴픽 진행·전환은 별개 이벤트라 그대로 진행).
const RVA_LINEUP: usize = 0x1d7eb30; // 0.5.3 0x1bc52b0
const PROLOGUE_LINEUP: &[u8] = &[
    0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x53,
];
type LineupFn = unsafe extern "win64" fn(usize, usize, usize, usize, usize);
static TRAMP_LINEUP: AtomicUsize = AtomicUsize::new(0);
static CNT_LINEUP_SKIP: AtomicU64 = AtomicU64::new(0);

// ── 훅 F: 권위 밴픽 커밋기 (교차 오염 = 밴한 챔프 출전 해결) ────────────────
// 0x1d075d0 = bool banpick_commit(RMI* rcx, u64 acting_team rdx, String* r8).
// 레코드(RMI+0x00 Vec, stride 0x100)의 마지막 원소에 챔프를 커밋한다. 버킷/요구팀을
// **자체 바닐라 phase**로 재계산하므로, 모드가 미개입이면 내 순서와 어긋나 교차 오염
// (내가 픽한 게 밴 버킷에, 밴한 게 픽 버킷에 = 밴한 챔프가 경기 출전).
// ★해결: 진입 직전 `[last+0xf0]`(ban_count)만 일시 조작해 원하는 (타입, 버킷)을 유도.
//   · 밴 강제: ban' = total/2 + 1  (total < 2*ban' 성립)
//   · 픽 강제(버킷 b): k ≡ total(mod2), k<npicks, pick_table[rule][k]==b → ban' = (total-k)/2
//   요구 acting_team은 버킷에서 자동 결정: b==0 → team[s^1](=T1), b==1 → team[s](=T2).
// 원본의 중복검사·상한·fearless·allocator 경로는 전부 보존된다.
const RVA_COMMIT: usize = 0x11c04a0; // 0.5.3 0x167fdd0
const PROLOGUE_COMMIT: &[u8] = &[
    0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x53,
];
type CommitFn = unsafe extern "win64" fn(usize, usize, usize) -> u8;
static TRAMP_COMMIT: AtomicUsize = AtomicUsize::new(0);
static CNT_COMMIT: AtomicU64 = AtomicU64::new(0);
static CNT_COMMIT_CUSTOM: AtomicU64 = AtomicU64::new(0);

// ── 훅 D': 턴 오라클 전체 대체 (팀 순서 지배) ──────────────────────────────
// 0x1d07cf0 = fn turn(RMI* rcx) -> (rax: 0/1, rdx: acting team_id). "지금 누구 차례냐"의
// 단일 오라클 — AI턴 0xebe530@0xebe8de가 이걸로 행동 팀을 정한다(타입은 hook A).
// 커스텀 seq의 팀비트로 team_id를 반환하면 AI가 내 순서대로 행동 → 씬·레코드 일치.
// 2워드 반환은 Rust로 직접 불가 → raw 스텁(out 파라미터 → rdx 로드)으로 처리.
const RVA_TURN: usize = 0x11c0bd0; // 0.5.3 0x1680500
static TURN_STUB: AtomicUsize = AtomicUsize::new(0);

/// Drop 원복 가드 — shim 도중 패닉해도 unwind 중에 원복 보장.
struct FieldGuard {
    addr: usize,
    old: u64,
}
impl Drop for FieldGuard {
    fn drop(&mut self) {
        unsafe { core::ptr::write(self.addr as *mut u64, self.old) };
    }
}

/// 바이트 필드용 원복 가드(레코드 +0xf8 side 일시 조정).
struct ByteGuard {
    addr: usize,
    old: u8,
}
impl Drop for ByteGuard {
    fn drop(&mut self) {
        unsafe { core::ptr::write(self.addr as *mut u8, self.old) };
    }
}

unsafe extern "win64" fn hook_applier(
    scene: usize,
    p2: usize,
    p3: usize,
    acting_team: usize,
    champ_ptr: usize,
    champ_len: usize,
    animate: u8,
) {
    CNT_APPLIER.fetch_add(1, Ordering::Relaxed);
    let orig_addr = TRAMP_APPLIER.load(Ordering::Relaxed);
    if orig_addr == 0 {
        return; // 설치 불변식상 도달 불가(트램폴린 확보 후 패치)
    }
    let orig: ApplierFn = core::mem::transmute(orig_addr);

    let _ = catch_unwind(AssertUnwindSafe(|| {
        // 밴픽 화면 아님/커스텀 미적용/포인터 이상 → 원본 그대로
        if !IN_BANPICK.load(Ordering::Relaxed)
            || !CUSTOM_ACTIVE.load(Ordering::Relaxed)
            || !addr_ok(scene)
        {
            orig(scene, p2, p3, acting_team, champ_ptr, champ_len, animate);
            return;
        }
        // 내 밴픽 씬·팀ID 기억 — 매치 식별용(백그라운드 매치 배제 게이트).
        MY_SCENE.store(scene, Ordering::Relaxed);
        MY_T1.store(ru64(scene + 0x3d0), Ordering::Relaxed);
        MY_T2.store(ru64(scene + 0x3d8), Ordering::Relaxed);
        let bc = ru64(scene.wrapping_add(O_SC_BAN_COUNT));
        let rule = ru8(scene.wrapping_add(O_SC_RULE));
        let Some(seq) = config::get().seq_for(rule, bc) else {
            orig(scene, p2, p3, acting_team, champ_ptr, champ_len, animate);
            return;
        };
        // position = 진행된 밴+픽 총수(4벡터 len 합) — 원본과 동일 산출.
        let position = ru64(scene.wrapping_add(O_SC_T1BAN_LEN))
            .wrapping_add(ru64(scene.wrapping_add(O_SC_T2BAN_LEN)))
            .wrapping_add(ru64(scene.wrapping_add(O_SC_T1PICK_LEN)))
            .wrapping_add(ru64(scene.wrapping_add(O_SC_T2PICK_LEN)));
        let ph = match seq.get(position as usize) {
            Some(&p) if p != 0xFF => p,
            // 범위 밖/완료 = 커스텀 관할 밖 → 원본(안전 폴백)
            _ => {
                orig(scene, p2, p3, acting_team, champ_ptr, champ_len, animate);
                return;
            }
        };
        let is_ban = (ph & 2) != 0; // bit1 = 타입(0=pick,1=ban)

        // ★★설계 전환(2026-07-28 실측): 4버킷 appender 직접호출 방식을 폐기.
        // 실측(RECS tag=09 vlen=01.00.00.00 고정)으로 **원본 적용기를 거치지 않은 액션은
        // 게임의 밴픽 이력(레코드 컨테이너)에 전혀 기록되지 않음**이 확정됐다. 이력은
        // 경기 sim 초기화의 권위 입력이라, 우회하면 sim이 "밴픽 1개만 진행된" 이력을 보고
        // __fastfail. (연출이 orig 위임한 마지막 픽에서만 나온 것도 같은 이유.)
        // ⟹ 모든 액션을 orig에 통과시키되, 원본의 분류(ban-fullness)만 내 seq대로 유도한다:
        //    원본 판정 = (t1ban_len==bc && t2ban_len==bc) ? 픽 : 밴.
        //    · 밴을 원하면  → bc를 "현재 밴 len보다 큰 값"으로 잠깐 올려 밴 분기 강제
        //    · 픽을 원하면  → bc를 "양팀 밴 len과 같은 값"으로 잠깐 낮춰 픽 분기 강제
        //    (양팀 밴 len이 다르면 픽 강제 시 t2ban_len도 일시 동치화. 픽 분기는 밴 vec을
        //     읽지 않음 = §16 검증) 호출 후 Drop 가드로 원복.
        let t1bn = ru64(scene.wrapping_add(O_SC_T1BAN_LEN));
        let t2bn = ru64(scene.wrapping_add(O_SC_T2BAN_LEN));
        let bans_full = t1bn == bc && t2bn == bc;

        if is_ban == !bans_full {
            // 원본 기본 판정이 이미 내 의도와 일치 → 조작 없이 그대로.
            orig(scene, p2, p3, acting_team, champ_ptr, champ_len, animate);
        } else if is_ban {
            // 밴 원하는데 원본은 "픽"으로 볼 상태(밴 다 참) → bc를 올려 밴 강제.
            let _g = FieldGuard { addr: scene + O_SC_BAN_COUNT, old: bc };
            core::ptr::write((scene + O_SC_BAN_COUNT) as *mut u64, bc + 1);
            orig(scene, p2, p3, acting_team, champ_ptr, champ_len, animate);
        } else {
            // 픽 원하는데 원본은 "밴"으로 볼 상태(밴 미완) → bc/밴len 동치화로 픽 강제.
            CNT_FORCED_PICK.fetch_add(1, Ordering::Relaxed);
            let _g_bc = FieldGuard { addr: scene + O_SC_BAN_COUNT, old: bc };
            core::ptr::write((scene + O_SC_BAN_COUNT) as *mut u64, t1bn);
            let _g_t2 = if t2bn != t1bn {
                core::ptr::write((scene + O_SC_T2BAN_LEN) as *mut u64, t1bn);
                Some(FieldGuard { addr: scene + O_SC_T2BAN_LEN, old: t2bn })
            } else {
                None
            };
            orig(scene, p2, p3, acting_team, champ_ptr, champ_len, animate);
        }

        // ★단계 경계 배너: 이번 커밋 후 다음 액션의 타입이 바뀌면 해당 배너를 태운다.
        // (마지막 픽 orig 위임 경로는 위에서 early-return — 전환 배너는 원본이 처리)
        let pos = (position + 1) as usize; // 방금 커밋 반영된 다음 인덱스
        if pos < seq.len() {
            let next_ban = (seq[pos] & 2) != 0;
            if next_ban != is_ban {
                // ★타이밍: 카드가 슬롯으로 날아가는 연출(+0x348 != -1) 중에는 배너를 띄우지
                // 않는다(게임 바닐라도 연출 종료 후 띄움). 연출 진행중이면 지연 처리.
                let card_busy =
                    core::ptr::read((scene + O_SC_CARD_ANIM) as *const i64) != -1;
                if next_ban {
                    // 픽→밴: ★래치(+0x43e=1)를 **즉시** 세워 턴 진행을 붙잡는다. 그러지 않으면
                    // 카드 연출이 끝나는 프레임에 게임이 먼저 다음 밴을 커밋해 배너가 한 박자
                    // 늦게 뜬다(실측). state=0(배너 시작)은 연출 종료 후 tick()이 세팅 →
                    // arm0가 "금지 단계"를 그리고 arm1이 래치를 자동 해제해 진행 재개.
                    core::ptr::write((scene + O_SC_FSM_LATCH) as *mut u8, 1);
                    if card_busy {
                        PENDING_BAN_BANNER.store(true, Ordering::Relaxed);
                    } else {
                        CNT_BANNER.fetch_add(1, Ordering::Relaxed);
                        core::ptr::write((scene + O_SC_FSM_STATE) as *mut u64, 0);
                    }
                } else {
                    // 밴→픽: ★밴이 이번 커밋으로 다 찼으면 원본 적용기가 자체 배너를 띄운다
                    // (양 밴벡터 == ban_count 엣지) → 중복 방지로 건너뛴다.
                    let bans_full = ru64(scene.wrapping_add(O_SC_T1BAN_LEN)) == bc
                        && ru64(scene.wrapping_add(O_SC_T2BAN_LEN)) == bc;
                    if !bans_full {
                        CNT_BANNER.fetch_add(1, Ordering::Relaxed);
                        if card_busy {
                            // 게임 드레인에 위임: 카드 연출 종료 시 픽 배너 발동(+0x446=0).
                            core::ptr::write((scene + O_SC_FSM_LATCH) as *mut u8, 1);
                            core::ptr::write((scene + O_SC_FSM_DEFER) as *mut u8, 0);
                        } else {
                            let b = BASE.load(Ordering::Relaxed);
                            if b != 0 {
                                let banner: BannerFn = core::mem::transmute(b + RVA_BANNER);
                                banner(scene, p2, p3, 0);
                            }
                        }
                    }
                }
            }
        }

        // 진단: 마지막 액션(seq 완전소진)에서 로스터 상태 덤프.
        if config::get().debug && (position as usize) + 1 >= seq.len() {
            crate::diag::dump_roster_check(scene);
        }
    }));
}

// ── 훅 E: 라인업 적용기 — 이름 조회 실패 시 스킵(패닉 회피) ────────────────
/// Vec<String>(ptr,len)에서 (ptr,len) 이름이 존재하는지. String stride 0x18{cap,ptr,len}.
unsafe fn name_in_vec(vec_ptr: usize, vec_len: usize, np: usize, nl: usize) -> bool {
    if vec_ptr < 0x10000 || vec_len > 64 || np < 0x10000 || nl == 0 || nl > 64 {
        return false;
    }
    for i in 0..vec_len {
        let e = vec_ptr + i * 0x18;
        let ep = core::ptr::read((e + 8) as *const usize);
        let el = core::ptr::read((e + 0x10) as *const usize);
        if el == nl && ep >= 0x10000 {
            let mut same = true;
            for k in 0..nl {
                if core::ptr::read((ep + k) as *const u8) != core::ptr::read((np + k) as *const u8)
                {
                    same = false;
                    break;
                }
            }
            if same {
                return true;
            }
        }
    }
    false
}

/// RMI의 마지막 레코드에서 (last, total, ban, rule, side)를 읽고, 내 매치인지 상관 검증.
/// 내 매치 = 씬(MY_SCENE)의 ban/rule/total과 레코드의 그것이 모두 일치.
/// 반환: Some((last, total, ban, rule, side)) — 내 매치 + 드래프트 진행중(state 9)일 때만.
unsafe fn my_record(rmi: usize) -> Option<(usize, u64, u64, u8, usize)> {
    if !addr_ok(rmi) {
        return None;
    }
    let rlen = ru64(rmi + 0x10) as usize;
    let rptr = ru64(rmi + 8) as usize;
    // ⚠레코드 개수 상한을 두면 안 된다 — 원본 턴 오라클(0x1680500)은 `rlen == 0` 만 거르고
    // 상한이 없다. 구 `rlen > 64` 가드는 레코드가 64개를 넘는 경기(다전제 등 액션이 누적된
    // 매치)를 통째로 탈락시켜, 그 경기의 커스텀 순서가 조용히 죽었다(2026-07-31).
    // 안전은 상한이 아니라 **산출된 주소의 유효성**으로 확보한다.
    if rlen == 0 || !addr_ok(rptr) {
        return None;
    }
    let last = rptr.wrapping_add((rlen - 1).wrapping_mul(0x100));
    if !addr_ok(last) {
        return None;
    }
    if core::ptr::read((last + 0xc0) as *const u32) != 9 {
        return None;
    }
    let total = ru64(last + 0x40)
        .wrapping_add(ru64(last + 0x58))
        .wrapping_add(ru64(last + 0x70))
        .wrapping_add(ru64(last + 0x88));
    let ban = ru64(last + 0xf0);
    let rule = ru8(last + 0xf9);
    let side = (ru8(last + 0xf8) & 1) as usize;
    // ★매치 식별 게이트 없음(2026-07-29 유저 확정): 커스텀 순서는 **모든 경기에 동일 적용**
    // 하는 게 의도된 동작(게임 전체 규칙 변경). 팀 판정은 각 경기 자체의 side 규약으로 한다:
    // T1 = team[side^1] (픽 버킷 +0x60), T2 = team[side] (+0x78) — RE §17 정규 대응.
    Some((last, total, ban, rule, side))
}

/// 레코드의 Vec<String>(ptr@off, len@off+8, 원소 0x18 {cap,ptr,len}) → Vec<String>.
unsafe fn read_name_vec(rec: usize, ptr_off: usize) -> Vec<String> {
    let mut out = Vec::new();
    let p = ru64(rec + ptr_off) as usize;
    let n = ru64(rec + ptr_off + 8) as usize;
    if p < 0x10000 || n > 32 {
        return out;
    }
    for i in 0..n {
        let e = p + i * 0x18;
        let sp = ru64(e + 8) as usize;
        let sl = ru64(e + 0x10) as usize;
        if sp < 0x10000 || sl == 0 || sl > 64 {
            continue;
        }
        let mut b = Vec::with_capacity(sl);
        for k in 0..sl {
            b.push(core::ptr::read((sp + k) as *const u8));
        }
        if let Ok(s) = String::from_utf8(b) {
            out.push(s);
        }
    }
    out
}

/// 우리 밴픽 AI용 상태 스냅샷 — 커밋 직후 호출(액션당 1회).
/// 레코드 오프셋: ban0 ptr+0x38, ban1 +0x50, pick0 +0x68, pick1 +0x80 (버킷0=T1).
unsafe fn snapshot_draft(last: usize, total_after: u64, rule: u8, ban: u64) {
    let acting_t1 = match config::get().seq_for(rule, ban) {
        Some(seq) => match seq.get(total_after as usize) {
            Some(&ph) if ph != 0xFF => ph & 1 == 0,
            _ => return, // 드래프트 종료 → 스냅샷 불필요
        },
        None => (total_after & 1) == 0, // 바닐라 근사
    };
    crate::draft_ai::set_snapshot(crate::draft_ai::Snapshot {
        t1_ban: read_name_vec(last, 0x38),
        t2_ban: read_name_vec(last, 0x50),
        t1_pick: read_name_vec(last, 0x68),
        t2_pick: read_name_vec(last, 0x80),
        acting_t1,
        valid: true,
    });
}

/// 팀 id 조회: RMI+0x140 + 8*i (i = side 인덱스).
#[inline]
/// 진단 한정용: 이 RMI 가 **유저가 보고 있는 경기**인가(백그라운드 리그 경기 배제).
/// MY_T1/MY_T2 는 클라 씬(+0x3d0/+0x3d8)에서 hook_applier 가 학습한다. 미학습이면 false
/// (= 로그를 남기지 않음) — 진단 전용이라 동작에는 영향 없다.
unsafe fn is_my_match(rmi: usize) -> bool {
    let (a, b) = (MY_T1.load(Ordering::Relaxed), MY_T2.load(Ordering::Relaxed));
    if a == 0 && b == 0 {
        return false;
    }
    let (x, y) = (team_id(rmi, 0), team_id(rmi, 1));
    (x == a && y == b) || (x == b && y == a)
}

unsafe fn team_id(rmi: usize, i: usize) -> u64 {

    ru64(rmi + 0x140 + 8 * (i & 1))
}

/// 커밋 결과 관측 — 거부 수와 "제자리 맴돎"을 센다(진행 정지 진단).
/// 같은 (rmi, total) 조합으로 커밋이 연속 호출되면 그 경기는 앞으로 나아가지 못하고 있다.
unsafe fn observe_commit(rmi: usize, r: u8) {
    if r == 0 {
        CNT_COMMIT_REJ.fetch_add(1, Ordering::Relaxed);
    }
    let mut tot = u64::MAX;
    if addr_ok(rmi) {
        let rlen = ru64(rmi + 0x10) as usize;
        let rptr = ru64(rmi + 8) as usize;
        if rlen != 0 && addr_ok(rptr) {
            let last = rptr.wrapping_add((rlen - 1).wrapping_mul(0x100));
            if addr_ok(last) {
                tot = ru64(last + 0x40)
                    .wrapping_add(ru64(last + 0x58))
                    .wrapping_add(ru64(last + 0x70))
                    .wrapping_add(ru64(last + 0x88));
            }
        }
    }
    let key = (rmi as u64).rotate_left(16) ^ tot;
    if LAST_COMMIT_KEY.swap(key, Ordering::Relaxed) == key {
        let n = SAME_COMMIT_RUN.fetch_add(1, Ordering::Relaxed) + 1;
        MAX_SAME_COMMIT.fetch_max(n, Ordering::Relaxed);
    } else {
        SAME_COMMIT_RUN.store(0, Ordering::Relaxed);
    }
}

// ── 훅 F 본체: 커밋 직전 ban_count 유도 ────────────────────────────────────
unsafe extern "win64" fn hook_commit(rmi: usize, acting_team: usize, champ: usize) -> u8 {
    CNT_COMMIT.fetch_add(1, Ordering::Relaxed);
    let stub = TRAMP_COMMIT.load(Ordering::Relaxed);
    if stub == 0 {
        return 0;
    }
    let orig: CommitFn = core::mem::transmute(stub);

    // (ban_field_addr, new_ban, required_team, side_override(addr,val)) — None이면 원본 그대로.
    let mut plan: Option<(usize, u64, usize, Option<(usize, u8)>)> = None;
    let _ = catch_unwind(AssertUnwindSafe(|| {
        if !custom_ctx() {
            return;
        }
        let Some((last, total, ban, rule, side)) = my_record(rmi) else { return };
        let Some(seq) = config::get().seq_for(rule, ban) else { return };
        let Some(&ph) = seq.get(total as usize) else { return };
        if ph == 0xFF {
            return;
        }
        let r = (rule & 3) as usize;
        let npicks = 2 * (r as u64 + 2);
        // 원하는 팀 = seq 팀비트. 각 경기 자체 규약: 0=T1=team[side^1], 1=T2=team[side].
        let d = if ph & 1 == 0 { side ^ 1 } else { side }; // 원하는 팀의 RMI 인덱스
        let want = team_id(rmi, d);
        if (ph & 2) != 0 {
            // 밴: 버킷은 total 홀짝 고정(로스터 무관)이나 **요구 acting_team**은 side로 결정.
            // 원하는 팀이 행동하도록 side 일시 조정: s' = d ^ ((total&1)^1).
            let s_new = d ^ (((total & 1) as usize) ^ 1);
            let nb = (total >> 1) + 1;
            plan = Some((last + 0xf0, nb, want as usize, Some((last + 0xf8, s_new as u8))));
        } else {
            // 픽: 원하는 팀의 버킷 b(0=+0x60=T1, 1=+0x78=T2)가 나오는 k 선택 → ban'=(total-k)/2.
            let b: u8 = ph & 1;
            let tbl = PICK_TABLES[r];
            let mut k = (total & 1) as usize; // k ≡ total (mod 2)
            let mut nb: Option<u64> = None;
            while (k as u64) <= total && (k as u64) < npicks {
                if tbl.get(k).copied() == Some(b) {
                    nb = Some((total - k as u64) / 2);
                    break;
                }
                k += 2;
            }
            let Some(nb) = nb else { return };
            plan = Some((last + 0xf0, nb, want as usize, None));
        }
    }));

    match plan {
        Some((addr, nb, req, side_ov)) => {
            CNT_COMMIT_CUSTOM.fetch_add(1, Ordering::Relaxed);
            let old = ru64(addr);
            let _g = FieldGuard { addr, old };
            core::ptr::write(addr as *mut u64, nb);
            // 밴일 때만 side 일시 조정(요구 acting_team을 원하는 팀으로) — Drop 가드로 원복.
            let _gs = side_ov.map(|(sa, sv)| {
                let ob = core::ptr::read(sa as *const u8);
                core::ptr::write(sa as *mut u8, sv);
                ByteGuard { addr: sa, old: ob }
            });
            let r = orig(rmi, req, champ);
            // ★우리 밴픽 AI용: 커밋 직후 상태를 스냅샷(다음 판단의 입력).
            if config::get().ai_ban_context {
                let _ = catch_unwind(AssertUnwindSafe(|| {
                    if let Some((last, tot, bn, rl, _)) = my_record(rmi) {
                        snapshot_draft(last, tot, rl, bn);
                    }
                }));
            }
            // CMT: total(=plan 산출 시점), ban', req팀, 반환값(1=성공/0=거부)
            if config::get().debug {
                let _ = catch_unwind(AssertUnwindSafe(|| {
                    if let Some((_, tot, _, _, _)) = my_record(rmi) {
                        crate::diag::dump_step(b"CMT", tot, nb, req as u64, r as u64);
                    }
                }));
            }
            observe_commit(rmi, r);
            r
        }
        None => {
            let r = orig(rmi, acting_team, champ);
            observe_commit(rmi, r);
            if config::get().debug {
                let _ = catch_unwind(AssertUnwindSafe(|| {
                    if addr_ok(rmi) {
                        let rlen = ru64(rmi + 0x10) as usize;
                        let rptr = ru64(rmi + 8) as usize;
                        if rlen > 0 && addr_ok(rptr) {
                            let last = rptr + (rlen - 1) * 0x100;
                            let tot = ru64(last + 0x40)
                                .wrapping_add(ru64(last + 0x58))
                                .wrapping_add(ru64(last + 0x70))
                                .wrapping_add(ru64(last + 0x88));
                            if true {  // ⚠전 경기 로깅(팀ID로 식별) — is_my_match 는 RMI/씬 팀ID 체계가 달라 신뢰 불가
                                // VAN: 커스텀 미적용(게이트 실패)으로 원본 통과한 경우
                                crate::diag::dump_step(
                                    b"VAN", tot, acting_team as u64, 0, r as u64,
                                );
                            }
                        }
                    }
                }));
            }
            r
        }
    }
}

// ── 훅 D' 본체: 턴 오라클 (전체 대체 — 원본 미실행, 바닐라 로직 자체 재현) ──
/// out_team에 team_id를 쓰고 rax(0/1)를 반환. raw 스텁이 rdx로 옮긴다.
unsafe extern "win64" fn turn_impl(rmi: usize, out_team: *mut u64) -> u64 {
    CNT_TURN.fetch_add(1, Ordering::Relaxed);
    let mut ok = 0u64;
    let mut team = 0u64;
    let _ = catch_unwind(AssertUnwindSafe(|| {
        if !addr_ok(rmi) {
            return;
        }
        let rlen = ru64(rmi + 0x10) as usize;
        let rptr = ru64(rmi + 8) as usize;
        // ★★이 함수는 원본 미실행 **전체 대체**라, 여기서 ok=0 을 반환하면 그 경기는
        // "행동할 팀 없음" = 밴픽이 영원히 진행되지 않는다(내 경기뿐 아니라 **모든 경기**).
        // 원본(0x1680500)은 `rlen == 0` 만 거르고 개수 상한이 없으므로 상한을 두면 안 된다 —
        // 구 `rlen > 64` 가드가 레코드 64개 초과 경기를 정지시켜 시즌 일정이 안 넘어갔다
        // (2026-07-31 유저 보고 · 원본 디스어셈블 대조로 확정).
        MAX_RLEN.fetch_max(rlen as u64, Ordering::Relaxed);
        if rlen == 0 || !addr_ok(rptr) {
            REJ_RLEN.fetch_add(1, Ordering::Relaxed);
            return;
        }
        let last = rptr.wrapping_add((rlen - 1).wrapping_mul(0x100));
        if !addr_ok(last) {
            REJ_RLEN.fetch_add(1, Ordering::Relaxed);
            return;
        }
        if core::ptr::read((last + 0xc0) as *const u32) != 9 {
            REJ_STATE.fetch_add(1, Ordering::Relaxed);
            return;
        }
        let total = ru64(last + 0x40)
            .wrapping_add(ru64(last + 0x58))
            .wrapping_add(ru64(last + 0x70))
            .wrapping_add(ru64(last + 0x88));
        let ban = ru64(last + 0xf0);
        let rule = ru8(last + 0xf9);
        let side = (ru8(last + 0xf8) & 1) as usize;
        let r = (rule & 3) as usize;
        let npicks = 2 * (r as u64 + 2);

        // 커스텀: seq 범위 내면 seq의 팀비트로 결정(apply_all=1 이면 백그라운드 경기도 포함).
        if custom_ctx() {
            if my_record(rmi).is_some() {
                if let Some(seq) = config::get().seq_for(rule, ban) {
                    match seq.get(total as usize) {
                        Some(&ph) if ph != 0xFF => {
                            // 각 경기 자체 규약: 0=T1=team[side^1], 1=T2=team[side].
                            CNT_TURN_CUSTOM.fetch_add(1, Ordering::Relaxed);
                            ok = 1;
                            team = if ph & 1 == 0 {
                                team_id(rmi, side ^ 1)
                            } else {
                                team_id(rmi, side)
                            };
                            return;
                        }
                        _ => {
                            return; // seq 소진 = 드래프트 종료
                        }
                    }
                }
            }
        }
        // 바닐라 재현(비트동일): 완료면 0, 밴구간은 total 홀짝, 픽구간은 pick_table.
        if total >= 2 * ban + npicks {
            return;
        }
        ok = 1;
        if total < 2 * ban {
            team = if total & 1 == 0 {
                team_id(rmi, side ^ 1)
            } else {
                team_id(rmi, side)
            };
        } else {
            let k = (total - 2 * ban) as usize;
            let b = PICK_TABLES[r].get(k).copied().unwrap_or(0);
            team = if b == 0 {
                team_id(rmi, side ^ 1)
            } else {
                team_id(rmi, side)
            };
        }
    }));
    if !out_team.is_null() {
        *out_team = team;
    }
    if ok != 0 {
        CNT_TURN_OK.fetch_add(1, Ordering::Relaxed);
    }
    // 진단: 후반부(total>=14)만 로깅 — 마지막 픽 정지 원인 추적.
    if config::get().debug && ok != 0 {
        let _ = catch_unwind(AssertUnwindSafe(|| {
            if addr_ok(rmi) {
                let rlen = ru64(rmi + 0x10) as usize;
                let rptr = ru64(rmi + 8) as usize;
                if rlen > 0 && addr_ok(rptr) {
                    let last = rptr + (rlen - 1) * 0x100;
                    let tot = ru64(last + 0x40)
                        .wrapping_add(ru64(last + 0x58))
                        .wrapping_add(ru64(last + 0x70))
                        .wrapping_add(ru64(last + 0x88));
                    if true {  // ⚠전 경기 로깅(팀ID로 식별) — is_my_match 는 RMI/씬 팀ID 체계가 달라 신뢰 불가
                        // TRN total, team(하위32), t1, t2
                        crate::diag::dump_step(
                            b"TRN", tot, team, team_id(rmi, 0), team_id(rmi, 1),
                        );
                    }
                }
            }
        }));
    }
    ok
}

/// 라인업 Vec<Rec>(Rec 0x28B: position@8, String@0x10{cap,ptr,len})의 모든 이름이
/// 대상 픽벡터에 존재하는지 검증.
unsafe fn lineup_ok(list: usize, pick_ptr: usize, pick_len: usize) -> bool {
    if list < 0x10000 {
        return false;
    }
    let lp = core::ptr::read((list + 8) as *const usize);
    let ll = core::ptr::read((list + 0x10) as *const usize);
    if lp < 0x10000 || ll == 0 || ll > 16 {
        return false;
    }
    for i in 0..ll {
        let rec = lp + i * 0x28;
        let np = core::ptr::read((rec + 0x18) as *const usize); // String.ptr
        let nl = core::ptr::read((rec + 0x20) as *const usize); // String.len
        if !name_in_vec(pick_ptr, pick_len, np, nl) {
            return false;
        }
    }
    true
}

unsafe extern "win64" fn hook_lineup(
    scene: usize,
    team_a: usize,
    team_b: usize,
    list_a: usize,
    list_b: usize,
) {
    let stub = TRAMP_LINEUP.load(Ordering::Relaxed);
    if stub == 0 {
        return;
    }
    let orig: LineupFn = core::mem::transmute(stub);
    let mut skip = false;
    let _ = catch_unwind(AssertUnwindSafe(|| {
        if !CUSTOM_ACTIVE.load(Ordering::Relaxed) || !addr_ok(scene) {
            return;
        }
        let t1 = ru64(scene + 0x3d0) as usize;
        // team_a/team_b 중 어느 쪽이 T1인지에 따라 대상 픽벡터 매칭
        let (la, lb) = if t1 == team_a {
            (list_a, list_b)
        } else if t1 == team_b {
            (list_b, list_a)
        } else {
            return; // 원본도 두 Vec drop 후 return — 개입 불필요
        };
        let p1 = ru64(scene + 0x170) as usize; // T1 pick vec ptr
        let l1 = ru64(scene + 0x178) as usize; // len
        let p2 = ru64(scene + 0x188) as usize; // T2
        let l2 = ru64(scene + 0x190) as usize;
        if !lineup_ok(la, p1, l1) || !lineup_ok(lb, p2, l2) {
            skip = true;
        }
    }));
    if skip {
        // 원본 실행 시 match_ui.rs:4181 unwrap(None) 패닉 → __fastfail. 스킵한다.
        // (입력 Vec은 원본이 소유·해제하지만, 스킵 시 누수만 발생하고 크래시는 없음 —
        //  드래프트 확정 1회성이라 무시 가능. 레인별 표시만 스테일.)
        CNT_LINEUP_SKIP.fetch_add(1, Ordering::Relaxed);
        return;
    }
    orig(scene, team_a, team_b, list_a, list_b);
}

// (구 훅 D = 0x1d07cf0 진단 트램폴린은 D'(turn_impl 전체 대체)로 교체됨 — 2026-07-29)

// ── 설치 ──────────────────────────────────────────────────────────────────
unsafe fn write_entry_patch(fn_addr: usize, target: usize) -> Result<(), &'static str> {
    let mut patch = [0x90u8; ORIG_LEN];
    patch[0] = 0x48;
    patch[1] = 0xb8; // movabs rax, target
    patch[2..10].copy_from_slice(&target.to_le_bytes());
    patch[10] = 0xff;
    patch[11] = 0xe0; // jmp rax
    let mut old: u32 = 0;
    if VirtualProtect(fn_addr, ORIG_LEN, RWX, &mut old) == 0 {
        return Err("VirtualProtect");
    }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), fn_addr as *mut u8, ORIG_LEN);
    VirtualProtect(fn_addr, ORIG_LEN, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), fn_addr, ORIG_LEN);
    Ok(())
}

unsafe fn check_site(fn_addr: usize, prologue: &[u8]) -> Result<(), &'static str> {
    // 외부 훅(다른 모드의 movabs+jmp)이면 덮지 않고 포기 — 현재 3지점 모두 타 모드
    // 미후킹 확인(§16). 공존 필요가 생기면 체인 후킹으로 재설계할 것.
    if ru8(fn_addr) == 0x48 && ru8(fn_addr + 1) == 0xb8 {
        return Err("foreign hook");
    }
    for (i, b) in prologue.iter().enumerate() {
        if ru8(fn_addr + i) != *b {
            return Err("prologue mismatch");
        }
    }
    Ok(())
}

/// 전체 대체 설치 (트램폴린 없음 — 원본 재실행 없음).
unsafe fn install_full(rva: usize, prologue: &[u8], repl: usize) -> Result<(), &'static str> {
    let fn_addr = BASE.load(Ordering::Relaxed) + rva;
    check_site(fn_addr, prologue)?;
    write_entry_patch(fn_addr, repl)
}

/// 트램폴린 detour 설치 (원본 12B + 복귀 점프 스텁).
unsafe fn install_detour(
    rva: usize,
    prologue: &[u8],
    repl: usize,
) -> Result<usize, &'static str> {
    let fn_addr = BASE.load(Ordering::Relaxed) + rva;
    check_site(fn_addr, prologue)?;
    let stub = VirtualAlloc(0, 64, MEM_CR, RWX);
    if stub == 0 {
        return Err("VirtualAlloc");
    }
    let mut s: Vec<u8> = Vec::with_capacity(32);
    let mut orig = [0u8; ORIG_LEN];
    core::ptr::copy_nonoverlapping(fn_addr as *const u8, orig.as_mut_ptr(), ORIG_LEN);
    s.extend_from_slice(&orig);
    s.extend_from_slice(&[0x49, 0xbb]); // movabs r11, ret (진입부 push열은 r11 미사용)
    s.extend_from_slice(&(fn_addr + ORIG_LEN).to_le_bytes());
    s.extend_from_slice(&[0x41, 0xff, 0xe3]); // jmp r11
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    write_entry_patch(fn_addr, repl)?;
    Ok(stub)
}

/// 0x1d07cf0 특수 트램폴린 — 진입부 `mov rax,[rcx+0x10]; test rax,rax; jz rel32`(13B).
/// 12B 경계에서 jz가 잘리므로 13B를 스텁에 복사하되 jz를 절대점프로 변환(rel32 범위 무관).
/// 진입 패치 = movabs rax,repl(10) + jmp rax(2) + nop(1) = 13B.
unsafe fn install_container(rva: usize, repl: usize) -> Result<usize, &'static str> {
    let fn_addr = BASE.load(Ordering::Relaxed) + rva;
    if ru8(fn_addr) == 0x48 && ru8(fn_addr + 1) == 0xb8 {
        return Err("foreign hook");
    }
    // 프롤로그 검증: 48 8b 41 10  48 85 c0  0f 84 <rel32>
    const EXPECT: [u8; 9] = [0x48, 0x8b, 0x41, 0x10, 0x48, 0x85, 0xc0, 0x0f, 0x84];
    for (i, b) in EXPECT.iter().enumerate() {
        if ru8(fn_addr + i) != *b {
            return Err("prologue mismatch");
        }
    }
    let rel32 = core::ptr::read_unaligned((fn_addr + 9) as *const i32) as isize;
    let target_a = (fn_addr as isize + 13 + rel32) as usize; // jz taken (count==0)
    let orig_cont = fn_addr + 13; // jz not-taken (원본 나머지)

    let stub = VirtualAlloc(0, 64, MEM_CR, RWX);
    if stub == 0 {
        return Err("VirtualAlloc");
    }
    let mut s: Vec<u8> = Vec::with_capacity(48);
    s.extend_from_slice(&[0x48, 0x8b, 0x41, 0x10]); // mov rax,[rcx+0x10]
    s.extend_from_slice(&[0x48, 0x85, 0xc0]); // test rax,rax
    s.extend_from_slice(&[0x0f, 0x85, 0x0d, 0x00, 0x00, 0x00]); // jnz +0x0d → orig_cont 블록
    s.extend_from_slice(&[0x49, 0xbb]); // movabs r11, target_a   (count==0)
    s.extend_from_slice(&target_a.to_le_bytes());
    s.extend_from_slice(&[0x41, 0xff, 0xe3]); // jmp r11
    s.extend_from_slice(&[0x49, 0xbb]); // movabs r11, orig_cont  (count!=0)
    s.extend_from_slice(&orig_cont.to_le_bytes());
    s.extend_from_slice(&[0x41, 0xff, 0xe3]); // jmp r11
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());

    let mut patch = [0x90u8; 13]; // nop 패딩(마지막 바이트)
    patch[0] = 0x48;
    patch[1] = 0xb8; // movabs rax, repl
    patch[2..10].copy_from_slice(&repl.to_le_bytes());
    patch[10] = 0xff;
    patch[11] = 0xe0; // jmp rax
    let mut old: u32 = 0;
    if VirtualProtect(fn_addr, 13, RWX, &mut old) == 0 {
        return Err("VirtualProtect");
    }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), fn_addr as *mut u8, 13);
    VirtualProtect(fn_addr, 13, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), fn_addr, 13);
    Ok(stub)
}

/// 0x1d07cf0 전체 대체 — 2워드 반환(rax=0/1, rdx=team_id) ABI를 raw 스텁으로 처리.
/// 스텁: sub rsp,0x38 / lea rdx,[rsp+0x20] / movabs rax,turn_impl / call rax /
///       mov rdx,[rsp+0x20] / add rsp,0x38 / ret
unsafe fn install_turn(base: usize) -> Result<(), &'static str> {
    let fn_addr = base + RVA_TURN;
    if ru8(fn_addr) == 0x48 && ru8(fn_addr + 1) == 0xb8 {
        return Err("foreign hook");
    }
    // 진입부 검증(원본 미실행 = 전체 대체지만 프롤로그로 대상 확인)
    const EXPECT: [u8; 9] = [0x48, 0x8b, 0x41, 0x10, 0x48, 0x85, 0xc0, 0x0f, 0x84];
    for (i, b) in EXPECT.iter().enumerate() {
        if ru8(fn_addr + i) != *b {
            return Err("prologue mismatch");
        }
    }
    let stub = VirtualAlloc(0, 64, MEM_CR, RWX);
    if stub == 0 {
        return Err("VirtualAlloc");
    }
    let mut s: Vec<u8> = Vec::with_capacity(40);
    s.extend_from_slice(&[0x48, 0x83, 0xEC, 0x38]); // sub rsp,0x38
    s.extend_from_slice(&[0x48, 0x8D, 0x54, 0x24, 0x20]); // lea rdx,[rsp+0x20]
    s.extend_from_slice(&[0x48, 0xB8]); // movabs rax, turn_impl
    s.extend_from_slice(&(turn_impl as usize).to_le_bytes());
    s.extend_from_slice(&[0xFF, 0xD0]); // call rax
    s.extend_from_slice(&[0x48, 0x8B, 0x54, 0x24, 0x20]); // mov rdx,[rsp+0x20]
    s.extend_from_slice(&[0x48, 0x83, 0xC4, 0x38]); // add rsp,0x38
    s.push(0xC3); // ret
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    TURN_STUB.store(stub, Ordering::Relaxed);

    // 진입 13B를 movabs rax,stub; jmp rax 로 대체(원본 미실행).
    let mut patch = [0x90u8; 13];
    patch[0] = 0x48;
    patch[1] = 0xb8;
    patch[2..10].copy_from_slice(&stub.to_le_bytes());
    patch[10] = 0xff;
    patch[11] = 0xe0;
    let mut old: u32 = 0;
    if VirtualProtect(fn_addr, 13, RWX, &mut old) == 0 {
        return Err("VirtualProtect");
    }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), fn_addr as *mut u8, 13);
    VirtualProtect(fn_addr, 13, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), fn_addr, 13);
    Ok(())
}

/// AI 밴 스코어러 전용 phase 산출 — ★인자가 "전체 진행 수"가 아니라 **지금까지 밴 개수**다
/// (밴 경로는 밴 리스트 2개만 받으므로 total = my_bans+opp_bans). 따라서 seq를 그대로
/// 인덱싱하면 안 되고, **seq에서 (bans_done+1)번째 밴 토큰**을 찾아 그 팀비트를 돌려준다.
/// 바닐라 폴백 = `2 | (bans_done & 1)`(밴 T1부터 교대) — 원본과 비트동일.
unsafe extern "win64" fn ai_ban_phase(bans_done: usize, rule: usize, ban: usize) -> u8 {
    CNT_AIBAN.fetch_add(1, Ordering::Relaxed);
    let mut out = 2u8 | ((bans_done as u8) & 1);
    let _ = catch_unwind(AssertUnwindSafe(|| {
        // ★★게이트는 `phase_of` 와 **반드시 같아야** 한다(2026-08-01 일정 정지 진범).
        // 여기만 게이트가 없어서, 백그라운드 경기가 **진행은 바닐라 순서 · AI 밴 판정만
        // 커스텀**이 되어 서로 다른 팀을 기준으로 움직였다 → 밴 단계를 못 빠져나오고 커밋만
        // 반복 → 일정 정지. (현행 seq 는 7번째 밴부터 바닐라와 팀비트가 다르다.)
        if !custom_ctx() {
            return;
        }
        if let Some(seq) = config::get().seq_for(rule as u8, ban as u64) {
            let mut n = 0usize;
            for &ph in seq.iter() {
                if ph & 2 != 0 {
                    if n == bans_done {
                        CNT_AIBAN_CUSTOM.fetch_add(1, Ordering::Relaxed);
                        out = ph;
                        return;
                    }
                    n += 1;
                }
            }
        }
    }));
    out
}

/// AI 밴 스코어러 인라인 phase 2사이트를 모드 함수 호출로 치환(바이트 패치).
/// 각 사이트: [보존 명령] + 인자 세팅 + call phase_from + jmp 합류점. rel32는 설치 시 계산.
unsafe fn install_ai_parity(base: usize) {
    // 모드 함수는 게임 모듈에서 ±2GB 밖일 수 있어 rel32 call 불가 → movabs rax + call rax.
    let fnaddr = ai_ban_phase as usize;
    let mut done = 0u64;

    // ── site1 (0.5.4 = 64B): 합류점이 **dl**(phase)·al·cl 을 전부 읽는다 ──
    //   0.5.3 대비 변경: rule cl→dl · total rdx→r9 · out cl→**dl** ·
    //   디스패처 직전에 `mov [rbp+0x70],r14` / `mov [rbp+0x88],r11` 스토어 2개가 신설됐다
    //   (스텁이 재현). 합류점의 al = 프롤로그가 저장한 인자 바이트 [rbp+0x97](0.5.3 [rbp+0x6f]),
    //   cl = [rsi+0xe13] 플래그. 둘 다 call 로 깨지므로 복원한다.
    let a1 = base + RVA_AI_SITE1;
    if (0..AI_SIG1.len()).all(|i| ru8(a1 + i) == AI_SIG1[i]) {
        let mut p = [0u8; 64];
        p[0..4].copy_from_slice(&[0x4c, 0x8b, 0x5b, 0x10]); // mov r11,[rbx+0x10] (보존)
        p[4..8].copy_from_slice(&[0x4c, 0x89, 0x75, 0x70]); // mov [rbp+0x70],r14 (원본 부작용)
        p[8..15].copy_from_slice(&[0x4c, 0x89, 0x9d, 0x88, 0x00, 0x00, 0x00]); // mov [rbp+0x88],r11
        p[15..19].copy_from_slice(&[0x4b, 0x8d, 0x0c, 0x33]); // lea rcx,[r11+r14] (밴 개수 합)
        p[19..22].copy_from_slice(&[0x0f, 0xb6, 0xd2]); // movzx edx,dl (rule)
        p[22..24].copy_from_slice(&[0x48, 0xb8]); // movabs rax, ai_ban_phase
        p[24..32].copy_from_slice(&fnaddr.to_le_bytes());
        p[32..34].copy_from_slice(&[0xff, 0xd0]); // call rax  (r8 = ban_count 그대로)
        p[34..37].copy_from_slice(&[0x0f, 0xb6, 0xd0]); // movzx edx,al  → out = dl
        p[37..41].copy_from_slice(&[0x4c, 0x8b, 0x5b, 0x10]); // mov r11,[rbx+0x10] (복원)
        p[41..45].copy_from_slice(&[0x4f, 0x8d, 0x0c, 0x33]); // lea r9,[r11+r14] (total 복원)
        p[45..52].copy_from_slice(&[0x0f, 0xb6, 0x8e, 0x13, 0x0e, 0x00, 0x00]); // movzx ecx,[rsi+0xe13]
        p[52..59].copy_from_slice(&[0x0f, 0xb6, 0x85, 0x97, 0x00, 0x00, 0x00]); // movzx eax,[rbp+0x97]
        p[59] = 0xe9; // jmp 합류점
        let rel2 = ((base + RVA_AI_JOIN1) as i64 - (a1 + 64) as i64) as i32;
        p[60..64].copy_from_slice(&rel2.to_le_bytes());
        let mut old: u32 = 0;
        if VirtualProtect(a1, p.len(), RWX, &mut old) != 0 {
            core::ptr::copy_nonoverlapping(p.as_ptr(), a1 as *mut u8, p.len());
            VirtualProtect(a1, p.len(), old, &mut old);
            FlushInstructionCache(GetCurrentProcess(), a1, p.len());
            done += 1;
        }
    }

    // ── site2 (0.5.4 = 54B): 합류점이 **cl**(phase)·al 을 읽는다 ──
    //   0.5.3 대비 변경: out al→**cl** · ban_count 슬롯 [rbp+0x110]→[rbp+0x120] ·
    //   rule 슬롯 [rbp+0x108]→[rbp+0x118] · total 은 rdx(=r12+r15), al 은 [rdi+0xe13] 플래그.
    let a2 = base + RVA_AI_SITE2;
    if (0..AI_SIG2.len()).all(|i| ru8(a2 + i) == AI_SIG2[i]) {
        let mut p = [0u8; 54];
        p[0..7].copy_from_slice(&[0x4c, 0x8b, 0x85, 0x20, 0x01, 0x00, 0x00]); // mov r8,[rbp+0x120]
        p[7..12].copy_from_slice(&[0x4d, 0x8b, 0x64, 0x24, 0x10]); // mov r12,[r12+0x10] (보존)
        p[12..16].copy_from_slice(&[0x4b, 0x8d, 0x0c, 0x3c]); // lea rcx,[r12+r15] (밴 개수 합)
        p[16..23].copy_from_slice(&[0x0f, 0xb6, 0x95, 0x18, 0x01, 0x00, 0x00]); // movzx edx,[rbp+0x118]
        p[23..25].copy_from_slice(&[0x48, 0xb8]); // movabs rax, ai_ban_phase
        p[25..33].copy_from_slice(&fnaddr.to_le_bytes());
        p[33..35].copy_from_slice(&[0xff, 0xd0]); // call rax → al = phase
        p[35..38].copy_from_slice(&[0x0f, 0xb6, 0xc8]); // movzx ecx,al → out = cl
        p[38..42].copy_from_slice(&[0x4b, 0x8d, 0x14, 0x3c]); // lea rdx,[r12+r15] (total 복원)
        p[42..49].copy_from_slice(&[0x0f, 0xb6, 0x87, 0x13, 0x0e, 0x00, 0x00]); // movzx eax,[rdi+0xe13]
        p[49] = 0xe9;
        let rel2 = ((base + RVA_AI_JOIN2) as i64 - (a2 + 54) as i64) as i32;
        p[50..54].copy_from_slice(&rel2.to_le_bytes());
        let mut old: u32 = 0;
        if VirtualProtect(a2, p.len(), RWX, &mut old) != 0 {
            core::ptr::copy_nonoverlapping(p.as_ptr(), a2 as *mut u8, p.len());
            VirtualProtect(a2, p.len(), old, &mut old);
            FlushInstructionCache(GetCurrentProcess(), a2, p.len());
            done += 1;
        }
    }
    AI_PATCHED.store(done, Ordering::Relaxed);
    config::dlog(&format!("AI parity patch: {done}/2"));
}


// ── 훅 H: AI 조합/추천 함수의 인라인 phase 6사이트 (★코치 위임 경로) ─────────
// 0.5.2에서는 이 6곳이 phase_from(B)를 **호출**해서 B 전체대체로 커스텀 순서를 따랐다
// (0xefef00 x2 / 0xefff70 x1 / 0xf00bb0 x2 / 0xf014d0 x1 = AI 추천·조합 평가).
// 0.5.3에서는 전부 **인라인**(0x188dd30 x2 / 0x188f360 / 0x1890450 x2 / 0x1890fd0)이라
// 미보정 시 AI가 바닐라 순서로 단계를 판단 → 인터리브에서 "코치에게 맡기기"가 중간에 멈춘다.
//
// 패치 방식: 사이트에 `jmp [rip+0]`(14B, **레지스터 무클로버**) → 스텁.
//   스텁 = volatile 6종 저장 → (필요 시 밀려난 원본 명령 재현) → 인자 마샬 →
//          모드 phase_from 호출 → 결과를 out 레지스터로 → 복원 → `jmp [rip+0]` 합류.
//   rsp는 0x70(16배수)만 조정하므로 정렬·shadow space 충족. 스텁 바이트는 오프라인
//   생성기(C:	fm2modso_gen_ai6.py)가 만들고 여기 상수로 박는다.
// 튜플 = (site_rva, join_rva, sig8, stub, fn_addr_offset, join_addr_offset)
const AI6: [(usize, usize, &[u8], &[u8], usize, usize); 6] = [
    // ai_reco1
    (0x215e526, 0x215e9d6, &[0xff, 0xe0, 0x4d, 0x8d, 0x42, 0x04, 0xb0, 0xff], &[0x48, 0x83, 0xec, 0x70, 0x48, 0x89, 0x4c, 0x24, 0x28, 0x48, 0x89, 0x54, 0x24, 0x30, 0x4c, 0x89, 0x44, 0x24, 0x38, 0x4c, 0x89, 0x4c, 0x24, 0x40, 0x4c, 0x89, 0x54, 0x24, 0x48, 0x4c, 0x89, 0x5c, 0x24, 0x50, 0x45, 0x0f, 0xb6, 0xdc, 0x4d, 0x89, 0xd2, 0x49, 0x89, 0xc9, 0x4c, 0x89, 0xc9, 0x4c, 0x89, 0xda, 0x4d, 0x89, 0xd0, 0x49, 0xd1, 0xe8, 0x48, 0xb8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xd0, 0x88, 0x44, 0x24, 0x58, 0x48, 0x8b, 0x4c, 0x24, 0x28, 0x48, 0x8b, 0x54, 0x24, 0x30, 0x4c, 0x8b, 0x44, 0x24, 0x38, 0x4c, 0x8b, 0x4c, 0x24, 0x40, 0x4c, 0x8b, 0x54, 0x24, 0x48, 0x4c, 0x8b, 0x5c, 0x24, 0x50, 0x0f, 0xb6, 0x44, 0x24, 0x58, 0x48, 0x83, 0xc4, 0x70, 0xff, 0x25, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], 58, 117),
    // ai_reco2
    (0x215e723, 0x215e9d6, &[0xff, 0xe0, 0x49, 0x8d, 0x52, 0x04, 0xb0, 0xff], &[0x48, 0x83, 0xec, 0x70, 0x48, 0x89, 0x4c, 0x24, 0x28, 0x48, 0x89, 0x54, 0x24, 0x30, 0x4c, 0x89, 0x44, 0x24, 0x38, 0x4c, 0x89, 0x4c, 0x24, 0x40, 0x4c, 0x89, 0x54, 0x24, 0x48, 0x4c, 0x89, 0x5c, 0x24, 0x50, 0x45, 0x0f, 0xb6, 0xdc, 0x4d, 0x89, 0xd2, 0x49, 0x89, 0xc9, 0x4c, 0x89, 0xc9, 0x4c, 0x89, 0xda, 0x4d, 0x89, 0xd0, 0x49, 0xd1, 0xe8, 0x48, 0xb8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xd0, 0x88, 0x44, 0x24, 0x58, 0x48, 0x8b, 0x4c, 0x24, 0x28, 0x48, 0x8b, 0x54, 0x24, 0x30, 0x4c, 0x8b, 0x44, 0x24, 0x38, 0x4c, 0x8b, 0x4c, 0x24, 0x40, 0x4c, 0x8b, 0x54, 0x24, 0x48, 0x4c, 0x8b, 0x5c, 0x24, 0x50, 0x0f, 0xb6, 0x44, 0x24, 0x58, 0x48, 0x83, 0xc4, 0x70, 0xff, 0x25, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], 58, 117),
    // ai_comp
    (0x215fac8, 0x215fc10, &[0xff, 0xe0, 0x4c, 0x8d, 0x42, 0x04, 0xb0, 0xff], &[0x48, 0x83, 0xec, 0x70, 0x48, 0x89, 0x4c, 0x24, 0x28, 0x48, 0x89, 0x54, 0x24, 0x30, 0x4c, 0x89, 0x44, 0x24, 0x38, 0x4c, 0x89, 0x4c, 0x24, 0x40, 0x4c, 0x89, 0x54, 0x24, 0x48, 0x4c, 0x89, 0x5c, 0x24, 0x50, 0x45, 0x0f, 0xb6, 0xd9, 0x49, 0x89, 0xd2, 0x49, 0x89, 0xc9, 0x4c, 0x89, 0xc9, 0x4c, 0x89, 0xda, 0x4d, 0x89, 0xd0, 0x49, 0xd1, 0xe8, 0x48, 0xb8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xd0, 0x88, 0x44, 0x24, 0x58, 0x48, 0x8b, 0x4c, 0x24, 0x28, 0x48, 0x8b, 0x54, 0x24, 0x30, 0x4c, 0x8b, 0x44, 0x24, 0x38, 0x4c, 0x8b, 0x4c, 0x24, 0x40, 0x4c, 0x8b, 0x54, 0x24, 0x48, 0x4c, 0x8b, 0x5c, 0x24, 0x50, 0x0f, 0xb6, 0x44, 0x24, 0x58, 0x48, 0x83, 0xc4, 0x70, 0xff, 0x25, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], 58, 117),
    // ai_bb1
    (0x216082c, 0x2160a6f, &[0xff, 0xe2, 0x48, 0x8d, 0x51, 0x04, 0x41, 0xb0], &[0x48, 0x83, 0xec, 0x70, 0x48, 0x89, 0x4c, 0x24, 0x28, 0x48, 0x89, 0x54, 0x24, 0x30, 0x4c, 0x89, 0x44, 0x24, 0x38, 0x4c, 0x89, 0x4c, 0x24, 0x40, 0x4c, 0x89, 0x54, 0x24, 0x48, 0x4c, 0x89, 0x5c, 0x24, 0x50, 0x45, 0x0f, 0xb6, 0xda, 0x49, 0x89, 0xca, 0x49, 0x89, 0xc1, 0x4c, 0x89, 0xc9, 0x4c, 0x89, 0xda, 0x4d, 0x89, 0xd0, 0x49, 0xd1, 0xe8, 0x48, 0xb8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xd0, 0x88, 0x44, 0x24, 0x58, 0x48, 0x8b, 0x4c, 0x24, 0x28, 0x48, 0x8b, 0x54, 0x24, 0x30, 0x4c, 0x8b, 0x44, 0x24, 0x38, 0x4c, 0x8b, 0x4c, 0x24, 0x40, 0x4c, 0x8b, 0x54, 0x24, 0x48, 0x4c, 0x8b, 0x5c, 0x24, 0x50, 0x0f, 0xb6, 0x44, 0x24, 0x58, 0x41, 0x88, 0xc0, 0x48, 0x83, 0xc4, 0x70, 0xff, 0x25, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], 58, 120),
    // ai_bb2
    (0x2160918, 0x2160a6f, &[0xff, 0xe0, 0x48, 0x8d, 0x56, 0x04, 0xb0, 0xff], &[0x48, 0x83, 0xec, 0x70, 0x48, 0x89, 0x4c, 0x24, 0x28, 0x48, 0x89, 0x54, 0x24, 0x30, 0x4c, 0x89, 0x44, 0x24, 0x38, 0x4c, 0x89, 0x4c, 0x24, 0x40, 0x4c, 0x89, 0x54, 0x24, 0x48, 0x4c, 0x89, 0x5c, 0x24, 0x50, 0x44, 0x0f, 0xb6, 0x9d, 0x98, 0x01, 0x00, 0x00, 0x49, 0x89, 0xf2, 0x49, 0x89, 0xc9, 0x4c, 0x89, 0xc9, 0x4c, 0x89, 0xda, 0x4d, 0x89, 0xd0, 0x49, 0xd1, 0xe8, 0x48, 0xb8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xd0, 0x88, 0x44, 0x24, 0x58, 0x48, 0x8b, 0x4c, 0x24, 0x28, 0x48, 0x8b, 0x54, 0x24, 0x30, 0x4c, 0x8b, 0x44, 0x24, 0x38, 0x4c, 0x8b, 0x4c, 0x24, 0x40, 0x4c, 0x8b, 0x54, 0x24, 0x48, 0x4c, 0x8b, 0x5c, 0x24, 0x50, 0x0f, 0xb6, 0x44, 0x24, 0x58, 0x48, 0x83, 0xc4, 0x70, 0xff, 0x25, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], 62, 121),
    // ai_bb3
    // ★0.5.4 재핀: 밀려난 원본 스토어 3개가 disp8→disp32 로 커져 스텁을 재생성했다
    //   (bo_ai6_6_54.py). 합류점도 정정 — 0.5.3 은 arm 수렴점 다음 명령을 잡아
    //   `movzx edx,[rbp+0x218]` 를 건너뛰고 있었다(0.5.4 = 수렴점 0x21615b3 그대로 사용).
    (0x21612c3, 0x21615b3, &[0x0f, 0xb6, 0xc2, 0x48, 0x8d, 0x15, 0xdf, 0x20], &[0x48, 0x83, 0xec, 0x70, 0x48, 0x89, 0x4c, 0x24, 0x28, 0x48, 0x89, 0x54, 0x24, 0x30, 0x4c, 0x89, 0x44, 0x24, 0x38, 0x4c, 0x89, 0x4c, 0x24, 0x40, 0x4c, 0x89, 0x54, 0x24, 0x48, 0x4c, 0x89, 0x5c, 0x24, 0x50, 0x44, 0x0f, 0xb6, 0xda, 0x4d, 0x89, 0xea, 0x49, 0x89, 0xc9, 0x4a, 0x8d, 0x14, 0x6d, 0x00, 0x00, 0x00, 0x00, 0x4c, 0x89, 0x9d, 0x90, 0x00, 0x00, 0x00, 0x48, 0x89, 0xb5, 0xc0, 0x00, 0x00, 0x00, 0x48, 0x89, 0x9d, 0xc8, 0x00, 0x00, 0x00, 0x4c, 0x89, 0xc9, 0x4c, 0x89, 0xda, 0x4d, 0x89, 0xd0, 0x48, 0xb8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xd0, 0x88, 0x44, 0x24, 0x58, 0x48, 0x8b, 0x4c, 0x24, 0x28, 0x48, 0x8b, 0x54, 0x24, 0x30, 0x4c, 0x8b, 0x44, 0x24, 0x38, 0x4c, 0x8b, 0x4c, 0x24, 0x40, 0x4c, 0x8b, 0x54, 0x24, 0x48, 0x4c, 0x8b, 0x5c, 0x24, 0x50, 0x0f, 0xb6, 0x44, 0x24, 0x58, 0x48, 0x83, 0xc4, 0x70, 0xff, 0x25, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], 84, 143),
];
static AI6_PATCHED: AtomicU64 = AtomicU64::new(0);

/// 훅 G: AI턴 인라인 phase(0x1828213~0x18282fa, 231B 창에 38B) → 모드 phase_from 호출.
/// 실패해도 비치명적이지만 **AI 단계판정이 바닐라로 남아** 인터리브에서 AI가 엉뚱한 단계로
/// 행동할 수 있다(0.5.2 훅 A 미설치와 동등한 상태).
unsafe fn install_aiturn(base: usize) {
    let a = base + RVA_AITURN_SITE;
    if (0..AITURN_SIG.len()).any(|i| ru8(a + i) != AITURN_SIG[i]) {
        config::dlog("hook G(ai turn inline): sig mismatch — 스킵(AI 단계판정 바닐라)");
        return;
    }
    let mut p = [0u8; 38];
    // mov rcx,[rbp+0x5eb0]  (total)
    p[0..7].copy_from_slice(&[0x48, 0x8b, 0x8d, 0xb0, 0x5e, 0x00, 0x00]);
    // movzx edx,byte [rbp+0x5d61]  (rule)
    p[7..14].copy_from_slice(&[0x0f, 0xb6, 0x95, 0x61, 0x5d, 0x00, 0x00]);
    // mov r8,[rbp+0x5d58]  (ban_count)
    p[14..21].copy_from_slice(&[0x4c, 0x8b, 0x85, 0x58, 0x5d, 0x00, 0x00]);
    p[21..23].copy_from_slice(&[0x48, 0xb8]); // movabs rax, hook_phase_scalar
    p[23..31].copy_from_slice(&(hook_phase_scalar as usize).to_le_bytes());
    p[31..33].copy_from_slice(&[0xff, 0xd0]); // call rax → al = phase
    p[33] = 0xe9; // jmp 합류점
    let rel = ((base + RVA_AITURN_JOIN) as i64 - (a + 38) as i64) as i32;
    p[34..38].copy_from_slice(&rel.to_le_bytes());
    let mut old: u32 = 0;
    if VirtualProtect(a, p.len(), RWX, &mut old) != 0 {
        core::ptr::copy_nonoverlapping(p.as_ptr(), a as *mut u8, p.len());
        VirtualProtect(a, p.len(), old, &mut old);
        FlushInstructionCache(GetCurrentProcess(), a, p.len());
        AITURN_PATCHED.store(true, Ordering::Relaxed);
        config::dlog("hook G(ai turn inline): OK (38B/231B)");
    }
}

// ── 훅 I: UI 턴 하이라이트(흰칸) 인라인 phase 1사이트 ────────────────────────
// 0.5.2에서는 match_ui `0x11e2980` 이 phase_from(B)를 **호출**해서 흰칸 개수·위치가
// 모드 순서를 자동 추종했다(§17 갈래C 정정). 0.5.3에서는 그 함수(→`0x193a940`)가
// phase 계산을 **인라인**으로 품어서, 미보정 시 바닐라 스네이크 기준으로 흰칸이 2개
// 켜진다(유저 실측: 12/20 지점에서 레드 1픽 차례인데 하이라이트 2개).
//
// ★사이트 전량 수동 확정(2026-07-30): 디스패처 `jmp rdx` = 0x193b434 /
//   합류 = arm 들의 공통 `jae` 타깃 0x193b570 / 출력 = **sil** / total=rax·2*ban=rcx·
//   rule=[rbp+0xa0f9] / ⚠arm 마다 `lea r15,[rbp+0xade8]` 부작용이 있어 스텁이 재현한다.
//   합류 시점 live = esi 뿐(`mov eax,esi` 로 rax 는 즉시 덮임).
const HL: (usize, usize, &[u8], &[u8], usize, usize) = (
    // 0.5.4 재핀: 컨테이너 0x193a940→0x237c030. ⚠**출력 레지스터가 sil → bl 로 바뀌었다**
    // (게임 arm 이 `mov sil,0xff` → `mov bl,0xff`). 스텁 꼬리의 결과 반환도 ebx 로 교체.
    0x237cb14, 0x237cc50,
    &[0xff, 0xe2, 0x48, 0x8d, 0x51, 0x04, 0xb3, 0xff],
    &[0x48, 0x83, 0xec, 0x70, 0x48, 0x89, 0x44, 0x24, 0x20, 0x48, 0x89, 0x4c, 0x24, 0x28, 0x48, 0x89, 0x54, 0x24, 0x30, 0x4c, 0x89, 0x44, 0x24, 0x38, 0x4c, 0x89, 0x4c, 0x24, 0x40, 0x4c, 0x89, 0x54, 0x24, 0x48, 0x4c, 0x89, 0x5c, 0x24, 0x50, 0x44, 0x0f, 0xb6, 0x9d, 0xf9, 0xa0, 0x00, 0x00, 0x49, 0x89, 0xca, 0x49, 0x89, 0xc1, 0x4c, 0x8d, 0xbd, 0xe8, 0xad, 0x00, 0x00, 0x4c, 0x89, 0xc9, 0x4c, 0x89, 0xda, 0x4d, 0x89, 0xd0, 0x49, 0xd1, 0xe8, 0x48, 0xb8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xd0, 0x88, 0x44, 0x24, 0x60, 0x48, 0x8b, 0x4c, 0x24, 0x28, 0x48, 0x8b, 0x54, 0x24, 0x30, 0x4c, 0x8b, 0x44, 0x24, 0x38, 0x4c, 0x8b, 0x4c, 0x24, 0x40, 0x4c, 0x8b, 0x54, 0x24, 0x48, 0x4c, 0x8b, 0x5c, 0x24, 0x50, 0x48, 0x8b, 0x44, 0x24, 0x20, 0x0f, 0xb6, 0x5c, 0x24, 0x60, 0x90, 0x48, 0x83, 0xc4, 0x70, 0xff, 0x25, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    74, 139,
);
static HL_PATCHED: AtomicBool = AtomicBool::new(false);

/// 훅 H: AI 조합/추천 인라인 phase 6사이트 → 스텁 경유 모드 phase_from 호출.
/// 사이트별 시그 불일치는 개별 스킵(fail-safe) — 설치 수를 로그·카운터로 남긴다.
unsafe fn install_ai6(base: usize) {
    let mut done = 0u64;
    for (site, join, sig, stub, fn_off, join_off) in AI6.iter() {
        let a = base + *site;
        if (0..sig.len()).any(|i| ru8(a + i) != sig[i]) {
            config::dlog(&format!("hook H site {site:#x}: sig mismatch — 스킵"));
            continue;
        }
        // 외부 훅이 이미 있으면 건드리지 않는다(덮어쓰기 금지).
        if ru8(a) == 0x48 && ru8(a + 1) == 0xb8 {
            config::dlog(&format!("hook H site {site:#x}: foreign hook — 스킵"));
            continue;
        }
        let mem = VirtualAlloc(0, 256, MEM_CR, RWX);
        if mem == 0 {
            continue;
        }
        let mut s = stub.to_vec();
        s[*fn_off..*fn_off + 8].copy_from_slice(&(hook_phase_scalar as usize).to_le_bytes());
        s[*join_off..*join_off + 8].copy_from_slice(&(base + *join).to_le_bytes());
        core::ptr::copy_nonoverlapping(s.as_ptr(), mem as *mut u8, s.len());

        let mut p = [0u8; 14];
        p[0..6].copy_from_slice(&[0xff, 0x25, 0x00, 0x00, 0x00, 0x00]); // jmp [rip+0]
        p[6..14].copy_from_slice(&mem.to_le_bytes());
        let mut old: u32 = 0;
        if VirtualProtect(a, p.len(), RWX, &mut old) != 0 {
            core::ptr::copy_nonoverlapping(p.as_ptr(), a as *mut u8, p.len());
            VirtualProtect(a, p.len(), old, &mut old);
            FlushInstructionCache(GetCurrentProcess(), a, p.len());
            done += 1;
        }
    }
    AI6_PATCHED.store(done, Ordering::Relaxed);
    config::dlog(&format!("hook H(ai inline phase): {done}/6"));
}

// ── 훅 J: 드레인(update `0x1c55300`) 안의 phase 인라인 복제본 2곳 ────────────
// ★유저 실측(2026-07-30): 인터리브 3밴+2픽 구간에서 **픽 차례인데 흰칸이 아예 안 뜬다**
//   = 클라가 아직 밴 단계로 계산 중. 하이라이트를 만드는 코드는 훅 I(`0x193a940`)가
//   아니라 **드레인 안**에 있었다(`player_turn_highlight` 노드 참조 `0x1c66497`).
//   드레인은 연속된 두 계산을 쓴다:
//     · cur  `0x1c6605d` : phase(total)   → r14b   (현재 단계)
//     · next `0x1c66374` : phase(total+1) → r15b   (다음 차례 = 흰칸 대상)
// ★0.5.2 대조로 규약 확정(2026-07-30): 0.5.2 드레인은 같은 자리(콜사이트 0x1251900 +
//   0x1251968 루프 = "같은 phase 연속 개수" 공식)에서 **plain phase_from 을 그냥 호출**했다.
//   0.5.3 next 복제본의 밴 경로 `(total&1)^3` 도 `((total+1)&1)|2` 와 수학적으로 동일한
//   컴파일러 재작성일 뿐 ⟹ **둘 다 변환 없이 hook_phase_scalar 직결이 정답**
//   (최초 시도의 패리티 반전 래퍼 hl_next_phase 는 오판 → 제거).
// ⚠arm 부작용 `mov rdx,[rbp+0x1308]`(next)은 스텁이 **volatile 복원 뒤** 재현한다.
// 튜플 = (site, join, sig8, stub, fn_off, join_off, fnkind[0=phase,1=next])
const DRAIN_HL: [(usize, usize, &[u8], &[u8], usize, usize, u8); 2] = [
    // drain_cur
    (0x1e2a37d, 0x1e2a643, &[0xff, 0xe1, 0x48, 0x8d, 0x43, 0x04, 0x41, 0xb6], &[0x48, 0x83, 0xec, 0x70, 0x48, 0x89, 0x44, 0x24, 0x20, 0x48, 0x89, 0x4c, 0x24, 0x28, 0x48, 0x89, 0x54, 0x24, 0x30, 0x4c, 0x89, 0x44, 0x24, 0x38, 0x4c, 0x89, 0x4c, 0x24, 0x40, 0x4c, 0x89, 0x54, 0x24, 0x48, 0x4c, 0x89, 0x5c, 0x24, 0x50, 0x44, 0x0f, 0xb6, 0xde, 0x49, 0x89, 0xda, 0x4c, 0x8b, 0x8d, 0x18, 0x11, 0x00, 0x00, 0x4c, 0x89, 0xc9, 0x4c, 0x89, 0xda, 0x4d, 0x89, 0xd0, 0x49, 0xd1, 0xe8, 0x48, 0xb8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xd0, 0x88, 0x44, 0x24, 0x60, 0x48, 0x8b, 0x4c, 0x24, 0x28, 0x48, 0x8b, 0x54, 0x24, 0x30, 0x4c, 0x8b, 0x44, 0x24, 0x38, 0x4c, 0x8b, 0x4c, 0x24, 0x40, 0x4c, 0x8b, 0x54, 0x24, 0x48, 0x4c, 0x8b, 0x5c, 0x24, 0x50, 0x48, 0x8b, 0x44, 0x24, 0x20, 0x44, 0x8a, 0x74, 0x24, 0x60, 0x48, 0x83, 0xc4, 0x70, 0xff, 0x25, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], 67, 131, 0),
    // drain_next
    (0x1e2a694, 0x1e2a754, &[0xff, 0xe1, 0x48, 0x8d, 0x4b, 0x04, 0x41, 0xb7], &[0x48, 0x83, 0xec, 0x70, 0x48, 0x89, 0x44, 0x24, 0x20, 0x48, 0x89, 0x4c, 0x24, 0x28, 0x48, 0x89, 0x54, 0x24, 0x30, 0x4c, 0x89, 0x44, 0x24, 0x38, 0x4c, 0x89, 0x4c, 0x24, 0x40, 0x4c, 0x89, 0x54, 0x24, 0x48, 0x4c, 0x89, 0x5c, 0x24, 0x50, 0x44, 0x0f, 0xb6, 0xde, 0x49, 0x89, 0xda, 0x49, 0x89, 0xc1, 0x4c, 0x89, 0xc9, 0x4c, 0x89, 0xda, 0x4d, 0x89, 0xd0, 0x49, 0xd1, 0xe8, 0x48, 0xb8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xd0, 0x88, 0x44, 0x24, 0x60, 0x48, 0x8b, 0x4c, 0x24, 0x28, 0x48, 0x8b, 0x54, 0x24, 0x30, 0x4c, 0x8b, 0x44, 0x24, 0x38, 0x4c, 0x8b, 0x4c, 0x24, 0x40, 0x4c, 0x8b, 0x54, 0x24, 0x48, 0x4c, 0x8b, 0x5c, 0x24, 0x50, 0x48, 0x8b, 0x44, 0x24, 0x20, 0x48, 0x8b, 0x95, 0x28, 0x13, 0x00, 0x00, 0x44, 0x8a, 0x7c, 0x24, 0x60, 0x48, 0x83, 0xc4, 0x70, 0xff, 0x25, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], 63, 134, 1),
];
static DRAIN_PATCHED: AtomicU64 = AtomicU64::new(0);


// ── 훅 K: 드레인 잔여 phase 복제본 3곳 — ★drainA 는 코치 위임 펌프 게이트 ────
// ★역할 정정(2026-07-30 ghidra-re): drainA(0x1c5a0b2)는 하이라이트가 아니라 **코치 위임
//   자동행동 펌프의 게이트**다(0x1c5a28d `cmp al,dl` 일치 시에만 AI턴 요청 큐잉) ⟹ 상시 설치.
// 유저 실측(2026-07-30): 훅 J(cur/next) 만으로는 부족 — "내 2번째 픽인데 상대 4번째 밴
// 슬롯 하이라이트가 켜짐". 드레인엔 복제본이 총 7개이고 규약이 제각각이라 전부 수동 확정:
//   A 0x1c5a0b2 → out=dl **불리언 규약**(픽=1/밴=0), 완료는 별도 경로 0x1c5a274 로 분기
//   B 0x1c5a5b9 → out=al 표준 phase(0xff 도 같은 합류) = 1-way
//   C 0x1c5a9b1 → out=r8b 표준 phase, 완료는 0x1c5ab4e 로 분기
//   ⛔D 0x1c5aa99 = 루프 본문(인덱스가 회전마다 변함) · ⛔E 0x1c6fb16 = phase 를 **다시
//     점프테이블 인덱스로 재분기**하는 구조 ⟹ 둘 다 스텁 모델 부적합 = 패치 제외.
// A/C 는 결과가 0xff 면 원본 완료 경로로, 아니면 out 세팅 후 정상 합류로 가는 **2-way 스텁**.
// 튜플 = (site, join, ff_join(0=1-way), sig8, stub, fn_off, join_off, ff_off)
const DRAIN_HL2: [(usize, usize, usize, &[u8], &[u8], usize, usize, usize); 3] = [
    // drainA_step
    (0x1e1e3b2, 0x1e1e588, 0x1e1e574, &[0xff, 0xe6, 0x4d, 0x8d, 0x5a, 0x04, 0x4d, 0x39], &[0x48, 0x83, 0xec, 0x70, 0x48, 0x89, 0x44, 0x24, 0x20, 0x48, 0x89, 0x4c, 0x24, 0x28, 0x48, 0x89, 0x54, 0x24, 0x30, 0x4c, 0x89, 0x44, 0x24, 0x38, 0x4c, 0x89, 0x4c, 0x24, 0x40, 0x4c, 0x89, 0x54, 0x24, 0x48, 0x4c, 0x89, 0x5c, 0x24, 0x50, 0x45, 0x0f, 0xb6, 0xd8, 0x4d, 0x89, 0xd2, 0x4d, 0x89, 0xc9, 0x4c, 0x89, 0xc9, 0x4c, 0x89, 0xda, 0x4d, 0x89, 0xd0, 0x49, 0xd1, 0xe8, 0x48, 0xb8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xd0, 0x88, 0x44, 0x24, 0x60, 0x48, 0x8b, 0x4c, 0x24, 0x28, 0x48, 0x8b, 0x54, 0x24, 0x30, 0x4c, 0x8b, 0x44, 0x24, 0x38, 0x4c, 0x8b, 0x4c, 0x24, 0x40, 0x4c, 0x8b, 0x54, 0x24, 0x48, 0x4c, 0x8b, 0x5c, 0x24, 0x50, 0x48, 0x8b, 0x44, 0x24, 0x20, 0x80, 0x7c, 0x24, 0x60, 0xff, 0x75, 0x12, 0x48, 0x83, 0xc4, 0x70, 0xff, 0x25, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf6, 0x44, 0x24, 0x60, 0x02, 0x0f, 0x94, 0xc2, 0x48, 0x83, 0xc4, 0x70, 0xff, 0x25, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], 63, 155, 129),
    // drainC_phase
    (0x1e1ecb8, 0x1e1ed6e, 0x1e1ee55, &[0x41, 0xff, 0xe1, 0x4c, 0x8d, 0x41, 0x04, 0x4c], &[0x48, 0x83, 0xec, 0x70, 0x48, 0x89, 0x44, 0x24, 0x20, 0x48, 0x89, 0x4c, 0x24, 0x28, 0x48, 0x89, 0x54, 0x24, 0x30, 0x4c, 0x89, 0x44, 0x24, 0x38, 0x4c, 0x89, 0x4c, 0x24, 0x40, 0x4c, 0x89, 0x54, 0x24, 0x48, 0x4c, 0x89, 0x5c, 0x24, 0x50, 0x45, 0x0f, 0xb6, 0xda, 0x49, 0x89, 0xca, 0x49, 0x89, 0xf1, 0x4c, 0x89, 0xc9, 0x4c, 0x89, 0xda, 0x4d, 0x89, 0xd0, 0x49, 0xd1, 0xe8, 0x48, 0xb8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xd0, 0x88, 0x44, 0x24, 0x60, 0x48, 0x8b, 0x4c, 0x24, 0x28, 0x48, 0x8b, 0x54, 0x24, 0x30, 0x4c, 0x8b, 0x44, 0x24, 0x38, 0x4c, 0x8b, 0x4c, 0x24, 0x40, 0x4c, 0x8b, 0x54, 0x24, 0x48, 0x4c, 0x8b, 0x5c, 0x24, 0x50, 0x48, 0x8b, 0x44, 0x24, 0x20, 0x80, 0x7c, 0x24, 0x60, 0xff, 0x75, 0x12, 0x48, 0x83, 0xc4, 0x70, 0xff, 0x25, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x44, 0x8a, 0x44, 0x24, 0x60, 0x48, 0x83, 0xc4, 0x70, 0xff, 0x25, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], 63, 152, 129),
    // drainB_phase
    (0x1e1e8c0, 0x1e1ec12, 0x0, &[0xff, 0xe0, 0x4c, 0x8d, 0x42, 0x04, 0xb0, 0xff], &[0x48, 0x83, 0xec, 0x70, 0x48, 0x89, 0x44, 0x24, 0x20, 0x48, 0x89, 0x4c, 0x24, 0x28, 0x48, 0x89, 0x54, 0x24, 0x30, 0x4c, 0x89, 0x44, 0x24, 0x38, 0x4c, 0x89, 0x4c, 0x24, 0x40, 0x4c, 0x89, 0x54, 0x24, 0x48, 0x4c, 0x89, 0x5c, 0x24, 0x50, 0x44, 0x0f, 0xb6, 0xd8, 0x49, 0x89, 0xd2, 0x49, 0x89, 0xc9, 0x4c, 0x89, 0xc9, 0x4c, 0x89, 0xda, 0x4d, 0x89, 0xd0, 0x49, 0xd1, 0xe8, 0x48, 0xb8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xd0, 0x88, 0x44, 0x24, 0x60, 0x48, 0x8b, 0x4c, 0x24, 0x28, 0x48, 0x8b, 0x54, 0x24, 0x30, 0x4c, 0x8b, 0x44, 0x24, 0x38, 0x4c, 0x8b, 0x4c, 0x24, 0x40, 0x4c, 0x8b, 0x54, 0x24, 0x48, 0x4c, 0x8b, 0x5c, 0x24, 0x50, 0x48, 0x8b, 0x44, 0x24, 0x20, 0x40, 0x8a, 0x44, 0x24, 0x60, 0x48, 0x83, 0xc4, 0x70, 0xff, 0x25, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], 63, 127, 0),
];
static DRAIN2_PATCHED: AtomicU64 = AtomicU64::new(0);

// ── 훅 L: 드레인 슬롯 하이라이트 선택기 (구 "복제본 E") ──────────────────────
// ★0.5.2 한줄대조(2026-07-30, 유저 요청)로 확정한 진짜 범인. 0.5.2에서는 같은 로직이
// 드레인 콜사이트 `0x12514a8`(call phase_from → sil=0/1/2 재분류 → 분기)였는데 0.5.3은
// 인라인 + 픽테이블값을 **2차 점프테이블 인덱스로 재분기**하는 형태로 바뀌었다.
// 이 코드가 "지금 누구(팀) 차례냐"로 **깜빡일 밴/픽 슬롯**을 정한다(범위밖이면
// `[scene+0x434]=0` = 하이라이트 없음). JT를 다 풀어보면 최종 목적지는 3개뿐:
//   완료(0xff) → 0x1c70508 · T1 행동(phase&1==0) → 0x1c6fc40 · T2 행동 → 0x1c6fc3a
// 패치 지점 = 디스패처보다 앞인 0x1c6fb05(add r10,r10 직전 — r11d=rule·r10=ban 원값·
// rax=total 이 전부 살아있는 마지막 지점). ban 은 2배 전 원값이라 스텁에 shr 없음.
// arm 부작용 `mov rsi,[rbp+0x1308]` 재현. T2 경로의 dl/r8/r9 는 volatile 복원으로 보존.
const SLOTSEL: (usize, usize, usize, usize, &[u8], &[u8], usize, usize, usize, usize) = (
    // 0.5.4 재핀(합류 3종 전부 arm 분기 타깃으로 실측). 씬 베이스 레지스터가 r15→r14 로
    // 바뀌었으나 스텁이 쓰는 rax(total)/r10(ban)/r11(rule) 은 불변.
    0x1e33aeb, 0x1e33c26, 0x1e3453a, 0x1e33c20,
    &[0x4d, 0x01, 0xd2, 0x48, 0x8d, 0x35, 0xcf, 0xb2],
    &[0x48, 0x83, 0xec, 0x70, 0x48, 0x89, 0x44, 0x24, 0x20, 0x48, 0x89, 0x4c, 0x24, 0x28, 0x48, 0x89, 0x54, 0x24, 0x30, 0x4c, 0x89, 0x44, 0x24, 0x38, 0x4c, 0x89, 0x4c, 0x24, 0x40, 0x4c, 0x89, 0x54, 0x24, 0x48, 0x4c, 0x89, 0x5c, 0x24, 0x50, 0x48, 0x89, 0xc1, 0x4c, 0x89, 0xda, 0x4d, 0x89, 0xd0, 0x48, 0xb8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xd0, 0x88, 0x44, 0x24, 0x60, 0x48, 0x8b, 0x4c, 0x24, 0x28, 0x48, 0x8b, 0x54, 0x24, 0x30, 0x4c, 0x8b, 0x44, 0x24, 0x38, 0x4c, 0x8b, 0x4c, 0x24, 0x40, 0x4c, 0x8b, 0x54, 0x24, 0x48, 0x4c, 0x8b, 0x5c, 0x24, 0x50, 0x48, 0x8b, 0x44, 0x24, 0x20, 0x48, 0x8b, 0xb5, 0x28, 0x13, 0x00, 0x00, 0x80, 0x7c, 0x24, 0x60, 0xff, 0x74, 0x19, 0xf6, 0x44, 0x24, 0x60, 0x01, 0x75, 0x24, 0x48, 0x83, 0xc4, 0x70, 0xff, 0x25, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x48, 0x83, 0xc4, 0x70, 0xff, 0x25, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x48, 0x83, 0xc4, 0x70, 0xff, 0x25, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    50, 130, 148, 166,
);
static SLOTSEL_PATCHED: AtomicBool = AtomicBool::new(false);

// ── 진단: 트리거(자동행동 요청기) 진입 카운터 (0.5.3 · 정본 §14.6 후속) ──────
// 코치 위임 정지의 "죽은 링크" 특정용 — 펌프가 트리거를 부르는지(진입수)를 bp: 로그의
// trig= 로 노출. 큐 len·dedup 필드와 조합하면 펌프/트리거/큐/서버 중 어디서 끊기는지
// 갈라진다. 프롤로그 15B = ghidra-re 실측(⚠12B는 `sub rsp,0x838` 중간 절단 = 15 필수,
// 15B 내 rip-rel 없음). 스텁 = lock inc [TRIG_N] → 원본 15B → 복귀(rax만 클로버 —
// 프롤로그는 rax 미사용·win64 인자 아님).
const RVA_TRIGGER: usize = 0x1db11a0; // 0.5.3 0x1bf77d0
const PROLOGUE_TRIGGER: [u8; 15] = [
    0x55, 0x41, 0x57, 0x41, 0x56, 0x56, 0x57, 0x53, 0x48, 0x81, 0xec, 0x38, 0x08, 0x00, 0x00,
];
pub static TRIG_N: AtomicU64 = AtomicU64::new(0);

unsafe fn install_trigger_probe(base: usize) {
    let fn_addr = base + RVA_TRIGGER;
    if check_site(fn_addr, &PROLOGUE_TRIGGER).is_err() {
        config::dlog("trig probe: prologue mismatch — 스킵");
        return;
    }
    let stub = VirtualAlloc(0, 64, MEM_CR, RWX);
    if stub == 0 {
        return;
    }
    let mut s: Vec<u8> = Vec::with_capacity(48);
    s.extend_from_slice(&[0x48, 0xb8]); // movabs rax, &TRIG_N
    s.extend_from_slice(&(TRIG_N.as_ptr() as usize).to_le_bytes());
    s.extend_from_slice(&[0xf0, 0x48, 0xff, 0x00]); // lock inc qword [rax]
    s.extend_from_slice(&PROLOGUE_TRIGGER); // 원본 15B
    s.extend_from_slice(&[0x48, 0xb8]); // movabs rax, 복귀
    s.extend_from_slice(&(fn_addr + PROLOGUE_TRIGGER.len()).to_le_bytes());
    s.extend_from_slice(&[0xff, 0xe0]); // jmp rax
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    if write_entry_patch(fn_addr, stub).is_ok() {
        config::dlog("trig probe: OK");
    }
}

// ── 진단: 사이트별 phase 계측 (0.5.3 하이라이트 추적용, debug=1 전용) ────────
// 모든 스텁이 hook_phase_scalar 하나를 부르면 "어느 사이트가 무엇을 받았나"를 알 수 없다.
// 사이트마다 별도 래퍼를 두고 마지막 (total, rule, ban, 결과)를 원자적으로 기록한다.
// tick() 이 1초마다 덤프 → 어느 지점이 호출조차 안 되는지/값이 무엇인지 즉시 판정.
pub const PROBE_N: usize = 6;
pub static PROBE: [AtomicU64; PROBE_N] = [
    AtomicU64::new(u64::MAX), AtomicU64::new(u64::MAX), AtomicU64::new(u64::MAX),
    AtomicU64::new(u64::MAX), AtomicU64::new(u64::MAX), AtomicU64::new(u64::MAX),
];
pub static PROBE_CNT: [AtomicU64; PROBE_N] = [
    AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0),
    AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0),
];
const PROBE_NAME: [&str; PROBE_N] = [
    "builderB", "scene_step", "drain_cur", "drain_next", "drain_K", "slotsel",
];

#[inline]
unsafe fn probe_rec(i: usize, total: u64, rule: u8, ban: u64, r: u8) {
    PROBE[i].store(
        (total & 0xffff) | ((ban & 0xff) << 16) | ((rule as u64 & 0xf) << 24) | ((r as u64) << 32),
        Ordering::Relaxed,
    );
    PROBE_CNT[i].fetch_add(1, Ordering::Relaxed);
}

macro_rules! probe_fn {
    ($name:ident, $idx:expr) => {
        unsafe extern "win64" fn $name(total: usize, rdx: usize, ban: usize) -> u8 {
            let r = hook_phase_scalar(total, rdx, ban);
            if config::get().debug {
                probe_rec($idx, total as u64, rdx as u8, ban as u64, r);
            }
            r
        }
    };
}
probe_fn!(ph_drain_cur, 2);
probe_fn!(ph_drain_next, 3);
probe_fn!(ph_drain_k, 4);
probe_fn!(ph_slotsel, 5);

/// 훅 L: 슬롯 하이라이트 선택기(3-way) 설치.
unsafe fn install_slotsel(base: usize) {
    let (site, jt1, jff, jt2, sig, stub, fn_off, t1_off, ff_off, t2_off) = SLOTSEL;
    let a = base + site;
    if (0..sig.len()).any(|i| ru8(a + i) != sig[i]) {
        config::dlog("hook L(slot select): sig mismatch — 스킵(슬롯 깜빡임 바닐라)");
        return;
    }
    let mem = VirtualAlloc(0, 256, MEM_CR, RWX);
    if mem == 0 {
        return;
    }
    let mut s = stub.to_vec();
    s[fn_off..fn_off + 8].copy_from_slice(&(ph_slotsel as usize).to_le_bytes());
    s[t1_off..t1_off + 8].copy_from_slice(&(base + jt1).to_le_bytes());
    s[ff_off..ff_off + 8].copy_from_slice(&(base + jff).to_le_bytes());
    s[t2_off..t2_off + 8].copy_from_slice(&(base + jt2).to_le_bytes());
    core::ptr::copy_nonoverlapping(s.as_ptr(), mem as *mut u8, s.len());
    let mut p = [0u8; 14];
    p[0..6].copy_from_slice(&[0xff, 0x25, 0x00, 0x00, 0x00, 0x00]);
    p[6..14].copy_from_slice(&mem.to_le_bytes());
    let mut old: u32 = 0;
    if VirtualProtect(a, p.len(), RWX, &mut old) != 0 {
        core::ptr::copy_nonoverlapping(p.as_ptr(), a as *mut u8, p.len());
        VirtualProtect(a, p.len(), old, &mut old);
        FlushInstructionCache(GetCurrentProcess(), a, p.len());
        SLOTSEL_PATCHED.store(true, Ordering::Relaxed);
        config::dlog("hook L(slot select): OK");
    }
}

// ── 훅 M: 흰칸 "동시 점등 개수" 루프 (★정본이 ⬜로 남긴 그 공식) ─────────────
// 드레인 `0x1c55300` 안의 연속 same-phase 카운트 루프. 인덱스를 1씩 올리며 phase 를
// 구해 기준 phase(r8b)와 같은 동안 세고, 그 개수를 `[rbp+0x520]` 에 넣는다 = 흰칸 개수.
//   0x1c5aa95 lea rdi,[rsi+rdx]  (rdi=인덱스) / 0x1c5aa99 jmp r10 ← 패치
//   → 결과 dil, 합류 0x1c5ab31 `cmp dil, r8b`
// 유저 실측 "블루 4·5번째 두 칸이 동시에 켜짐"이 바로 이 루프의 바닐라 산출이었다
// (스네이크에서 연속 2픽). 커스텀 순서에선 연속 구간이 달라지므로 여기가 핵심.
// 인자: rcx=인덱스(rdi), r8=2*ban(rcx). rule 은 모드가 씬에서 읽는다(rdx 미사용).
const HL_COUNT: (usize, usize, &[u8], &[u8], usize, usize) = (
    0x1e1eda0, 0x1e1ee38,
    &[0x41, 0xff, 0xe2, 0x4c, 0x39, 0xdf, 0x0f, 0x83],
    &[0x48, 0x83, 0xec, 0x70, 0x48, 0x89, 0x44, 0x24, 0x20, 0x48, 0x89, 0x4c, 0x24, 0x28, 0x48, 0x89, 0x54, 0x24, 0x30, 0x4c, 0x89, 0x44, 0x24, 0x38, 0x4c, 0x89, 0x4c, 0x24, 0x40, 0x4c, 0x89, 0x54, 0x24, 0x48, 0x4c, 0x89, 0x5c, 0x24, 0x50, 0x49, 0x89, 0xca, 0x48, 0x89, 0xf9, 0x31, 0xd2, 0x4d, 0x89, 0xd0, 0x48, 0xb8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xd0, 0x88, 0x44, 0x24, 0x60, 0x48, 0x8b, 0x4c, 0x24, 0x28, 0x48, 0x8b, 0x54, 0x24, 0x30, 0x4c, 0x8b, 0x44, 0x24, 0x38, 0x4c, 0x8b, 0x4c, 0x24, 0x40, 0x4c, 0x8b, 0x54, 0x24, 0x48, 0x4c, 0x8b, 0x5c, 0x24, 0x50, 0x48, 0x8b, 0x44, 0x24, 0x20, 0x40, 0x0f, 0xb6, 0x7c, 0x24, 0x60, 0x48, 0x83, 0xc4, 0x70, 0xff, 0x25, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    52, 117,
);
static HLCNT_PATCHED: AtomicBool = AtomicBool::new(false);

/// 훅 M 전용 phase — 인덱스와 2*ban 만 받고 rule 은 씬에서 읽는다(루프 안이라 rule
/// 레지스터가 이미 소모됨). 씬 미확보 시에도 phase_of 가 바닐라로 폴백하므로 안전.
unsafe extern "win64" fn hl_count_phase(idx: usize, _rdx: usize, ban2: usize) -> u8 {
    let mut r = 0xFFu8;
    let _ = catch_unwind(AssertUnwindSafe(|| {
        let sc = MY_SCENE.load(Ordering::Relaxed);
        let rule = if sc != 0 && addr_ok(sc) { ru8(sc + O_SC_RULE) } else { 3 };
        r = phase_of(idx as u64, rule, (ban2 as u64) >> 1);
    }));
    r
}

/// 훅 M 설치.
unsafe fn install_hl_count(base: usize) {
    let (site, join, sig, stub, fn_off, join_off) = HL_COUNT;
    let a = base + site;
    if (0..sig.len()).any(|i| ru8(a + i) != sig[i]) {
        config::dlog("hook M(hl count): sig mismatch — 스킵");
        return;
    }
    let mem = VirtualAlloc(0, 256, MEM_CR, RWX);
    if mem == 0 {
        return;
    }
    let mut s = stub.to_vec();
    s[fn_off..fn_off + 8].copy_from_slice(&(hl_count_phase as usize).to_le_bytes());
    s[join_off..join_off + 8].copy_from_slice(&(base + join).to_le_bytes());
    core::ptr::copy_nonoverlapping(s.as_ptr(), mem as *mut u8, s.len());
    let mut p = [0u8; 14];
    p[0..6].copy_from_slice(&[0xff, 0x25, 0x00, 0x00, 0x00, 0x00]);
    p[6..14].copy_from_slice(&mem.to_le_bytes());
    let mut old: u32 = 0;
    if VirtualProtect(a, p.len(), RWX, &mut old) != 0 {
        core::ptr::copy_nonoverlapping(p.as_ptr(), a as *mut u8, p.len());
        VirtualProtect(a, p.len(), old, &mut old);
        FlushInstructionCache(GetCurrentProcess(), a, p.len());
        HLCNT_PATCHED.store(true, Ordering::Relaxed);
        config::dlog("hook M(hl count): OK");
    }
}

/// 훅 K: 드레인 잔여 복제본 3곳 설치(2-way 지원).
unsafe fn install_drain_hl2(base: usize) {
    let mut done = 0u64;
    for (site, join, ffjoin, sig, stub, fn_off, join_off, ff_off) in DRAIN_HL2.iter() {
        let a = base + *site;
        if (0..sig.len()).any(|i| ru8(a + i) != sig[i]) {
            config::dlog(&format!("hook K site {site:#x}: sig mismatch — 스킵"));
            continue;
        }
        let mem = VirtualAlloc(0, 256, MEM_CR, RWX);
        if mem == 0 {
            continue;
        }
        let mut s = stub.to_vec();
        s[*fn_off..*fn_off + 8].copy_from_slice(&(ph_drain_k as usize).to_le_bytes());
        s[*join_off..*join_off + 8].copy_from_slice(&(base + *join).to_le_bytes());
        if *ffjoin != 0 {
            s[*ff_off..*ff_off + 8].copy_from_slice(&(base + *ffjoin).to_le_bytes());
        }
        core::ptr::copy_nonoverlapping(s.as_ptr(), mem as *mut u8, s.len());
        let mut p = [0u8; 14];
        p[0..6].copy_from_slice(&[0xff, 0x25, 0x00, 0x00, 0x00, 0x00]);
        p[6..14].copy_from_slice(&mem.to_le_bytes());
        let mut old: u32 = 0;
        if VirtualProtect(a, p.len(), RWX, &mut old) != 0 {
            core::ptr::copy_nonoverlapping(p.as_ptr(), a as *mut u8, p.len());
            VirtualProtect(a, p.len(), old, &mut old);
            FlushInstructionCache(GetCurrentProcess(), a, p.len());
            done += 1;
        }
    }
    DRAIN2_PATCHED.store(done, Ordering::Relaxed);
    config::dlog(&format!("hook K(drain rest): {done}/3"));
}

/// 훅 J: 드레인 하이라이트 2사이트 설치.
unsafe fn install_drain_hl(base: usize) {
    let mut done = 0u64;
    for (site, join, sig, stub, fn_off, join_off, kind) in DRAIN_HL.iter() {
        let a = base + *site;
        if (0..sig.len()).any(|i| ru8(a + i) != sig[i]) {
            config::dlog(&format!("hook J site {site:#x}: sig mismatch — 스킵"));
            continue;
        }
        let mem = VirtualAlloc(0, 256, MEM_CR, RWX);
        if mem == 0 {
            continue;
        }
        // 진단 계측용 래퍼로 연결(값·호출수 기록). 동작은 phase_from 직결과 동일.
        let f = if *kind == 0 { ph_drain_cur as usize } else { ph_drain_next as usize };
        let mut s = stub.to_vec();
        s[*fn_off..*fn_off + 8].copy_from_slice(&f.to_le_bytes());
        s[*join_off..*join_off + 8].copy_from_slice(&(base + *join).to_le_bytes());
        core::ptr::copy_nonoverlapping(s.as_ptr(), mem as *mut u8, s.len());
        let mut p = [0u8; 14];
        p[0..6].copy_from_slice(&[0xff, 0x25, 0x00, 0x00, 0x00, 0x00]);
        p[6..14].copy_from_slice(&mem.to_le_bytes());
        let mut old: u32 = 0;
        if VirtualProtect(a, p.len(), RWX, &mut old) != 0 {
            core::ptr::copy_nonoverlapping(p.as_ptr(), a as *mut u8, p.len());
            VirtualProtect(a, p.len(), old, &mut old);
            FlushInstructionCache(GetCurrentProcess(), a, p.len());
            done += 1;
        }
    }
    DRAIN_PATCHED.store(done, Ordering::Relaxed);
    config::dlog(&format!("hook J(drain highlight): {done}/2"));
}

/// 훅 I: UI 하이라이트 인라인 phase → 스텁 경유 모드 phase_from 호출.
unsafe fn install_hl(base: usize) {
    let (site, join, sig, stub, fn_off, join_off) = HL;
    let a = base + site;
    if (0..sig.len()).any(|i| ru8(a + i) != sig[i]) {
        config::dlog("hook I(ui highlight): sig mismatch — 스킵(흰칸 바닐라)");
        return;
    }
    let mem = VirtualAlloc(0, 256, MEM_CR, RWX);
    if mem == 0 {
        return;
    }
    let mut s = stub.to_vec();
    s[fn_off..fn_off + 8].copy_from_slice(&(hook_phase_scalar as usize).to_le_bytes());
    s[join_off..join_off + 8].copy_from_slice(&(base + join).to_le_bytes());
    core::ptr::copy_nonoverlapping(s.as_ptr(), mem as *mut u8, s.len());
    let mut p = [0u8; 14];
    p[0..6].copy_from_slice(&[0xff, 0x25, 0x00, 0x00, 0x00, 0x00]); // jmp [rip+0]
    p[6..14].copy_from_slice(&mem.to_le_bytes());
    let mut old: u32 = 0;
    if VirtualProtect(a, p.len(), RWX, &mut old) != 0 {
        core::ptr::copy_nonoverlapping(p.as_ptr(), a as *mut u8, p.len());
        VirtualProtect(a, p.len(), old, &mut old);
        FlushInstructionCache(GetCurrentProcess(), a, p.len());
        HL_PATCHED.store(true, Ordering::Relaxed);
        config::dlog("hook I(ui highlight): OK");
    }
}

/// 셀렉트 효과음 분류를 seq 기반으로 치환(0x1c56245~0x1c56294, 79B 창에 56B).
/// 원본 계약 유지: r8 = 문자열 ptr, r9 = 길이. 이후 0x1251352로 자연 진행.
unsafe fn install_sfx(base: usize) {
    let a = base + RVA_SFX_SITE;
    if (0..SFX_SIG.len()).any(|i| ru8(a + i) != SFX_SIG[i]) {
        config::dlog("sfx patch: prologue mismatch — 스킵");
        return;
    }
    let win = RVA_SFX_END - RVA_SFX_SITE; // 0x4f = 79
    let mut p = vec![0x90u8; win]; // 남는 자리는 nop
    let mut i = 0usize;
    macro_rules! emit {
        ($b:expr) => {{
            let b = $b;
            p[i..i + b.len()].copy_from_slice(&b);
            i += b.len();
        }};
    }
    emit!([0x48u8, 0xb8]); // movabs rax, sfx_is_pick
    emit!((sfx_is_pick as usize).to_le_bytes());
    // ⚠씬 스택슬롯은 버전마다 바뀐다(0.5.2 0x12b0 → 0.5.3 0x12d0 → **0.5.4 0x12f0**).
    //   SFX_SIG 와 반드시 동일해야 한다. 2026-07-31 회귀: SFX_SIG 만 고치고 여기가 구값으로 남아
    //   엉뚱한 슬롯을 씬으로 읽음 → sfx_is_pick 이 addr_ok 실패로 0(밴) 반환 →
    //   **픽 때 밴 소리**. 시그가 통과하므로 로그상 "sfx patch: OK" 로 보였다.
    emit!([0x48u8, 0x8b, 0x8d, 0xf0, 0x12, 0x00, 0x00]); // mov rcx,[rbp+0x12f0] (scene)
    emit!([0xffu8, 0xd0]); // call rax → rax = 1(픽)/0(밴)
    emit!([0x49u8, 0xb8]); // movabs r8, ban_str
    emit!((base + RVA_STR_BAN).to_le_bytes());
    emit!([0x41u8, 0xb9, 0x1c, 0x00, 0x00, 0x00]); // mov r9d, 0x1c
    emit!([0x48u8, 0x85, 0xc0]); // test rax,rax
    emit!([0x74u8, 0x10]); // jz +16 (픽 아니면 그대로)
    emit!([0x49u8, 0xb8]); // movabs r8, pick_str
    emit!((base + RVA_STR_PICK).to_le_bytes());
    emit!([0x41u8, 0xb9, 0x1d, 0x00, 0x00, 0x00]); // mov r9d, 0x1d
    let mut old: u32 = 0;
    if VirtualProtect(a, win, RWX, &mut old) != 0 {
        core::ptr::copy_nonoverlapping(p.as_ptr(), a as *mut u8, win);
        VirtualProtect(a, win, old, &mut old);
        FlushInstructionCache(GetCurrentProcess(), a, win);
        SFX_PATCHED.store(true, Ordering::Relaxed);
        config::dlog(&format!("sfx patch: OK ({i}B / {win}B)"));
    }
}

/// 매 프레임 post_update에서 호출 — 1회 설치(멱등 게이트, 재설치 금지).
pub fn tick() {
    if !INSTALLED.swap(true, Ordering::Relaxed) {
        unsafe { install() };
    }
    // 예약된 픽→밴 배너: 카드 연출이 끝나고(+0x348==-1) 이전 배너 래치가 풀렸을 때(+0x43e==0)
    // 발동. 게임엔 밴 배너 지연 경로가 없어 모드가 프레임 폴링으로 대신한다.
    if PENDING_BAN_BANNER.load(Ordering::Relaxed) && IN_BANPICK.load(Ordering::Relaxed) {
        let sc = MY_SCENE.load(Ordering::Relaxed);
        if sc != 0 && addr_ok(sc) {
            unsafe {
                // 래치는 경계에서 이미 1(진행 정지). 카드 연출만 끝나면 배너 시작.
                if core::ptr::read((sc + O_SC_CARD_ANIM) as *const i64) == -1 {
                    PENDING_BAN_BANNER.store(false, Ordering::Relaxed);
                    PENDING_FRAMES.store(0, Ordering::Relaxed);
                    CNT_BANNER.fetch_add(1, Ordering::Relaxed);
                    core::ptr::write((sc + O_SC_FSM_STATE) as *mut u64, 0);
                    core::ptr::write((sc + O_SC_FSM_LATCH) as *mut u8, 1);
                } else {
                    // 안전장치: 연출 종료 신호가 끝내 안 오면 소프트락(래치 1 고정)이 되므로
                    // 일정 프레임 후 강제로 배너를 띄워 래치를 풀리게 한다.
                    let f = PENDING_FRAMES.fetch_add(1, Ordering::Relaxed);
                    if f > 600 {
                        PENDING_BAN_BANNER.store(false, Ordering::Relaxed);
                        PENDING_FRAMES.store(0, Ordering::Relaxed);
                        core::ptr::write((sc + O_SC_FSM_STATE) as *mut u64, 0);
                        core::ptr::write((sc + O_SC_FSM_LATCH) as *mut u8, 1);
                    }
                }
            }
        }
    }
    // ── 코치 위임 정지 워치독 (0.5.3 · ghidra-re 확정, 정본 MIGRATION §7.3 §14.6) ──
    // 클라 트리거(0x1bf77d0)의 L1 지문 String 캐시(scene+0x288/0x290/0x298)는 "AI턴 요청
    // 발사" 시점에 갱신된다. 서버 AI턴이 프레임 레이스로 팀 불일치를 내면(0x182813f:
    // out disc=-1) 요청만 소비되고 커밋·응답이 없다 → 같은 국면의 지문이 영원히 동일 →
    // 재요청 영구 차단(해제 조건이 "커밋"뿐이라 교착 = 코치 위임 랜덤 정지의 진범).
    // ⟹ total 이 8초(480프레임) 정체하면 캐시를 무효화해 펌프(훅 K drainA = 커스텀 정합)가
    // 자동 재발사하게 한다. 재무산돼도 다음 주기에 재킥(재시도 루프).
    //   +0x298(len) = 0  : L1 지문 강제 불일치 (String 해제는 cap/ptr 기준이라 len=0 안전)
    //   +0xe0 = -1(u64)  : L0 "씽킹중" 레코드 무효 (게임 자체 sentinel 규약)
    // 수동 턴에 발사돼도 서버가 -1 로 무시할 뿐 상태 오염 없음(턴 소유 판단 불요).
    {
        const O_SC_THINK_REC: usize = 0xe0; // AI턴 in-flight 레코드 match id (sentinel -1)
        const O_SC_FPRINT_LEN: usize = 0x298; // 지문 String len
        let mut stalled = false;
        if IN_BANPICK.load(Ordering::Relaxed) && CUSTOM_ACTIVE.load(Ordering::Relaxed) {
            let sc = MY_SCENE.load(Ordering::Relaxed);
            if sc != 0 && addr_ok(sc) {
                unsafe {
                    let (t1b, t2b) = (ru64(sc + O_SC_T1BAN_LEN), ru64(sc + O_SC_T2BAN_LEN));
                    let (t1p, t2p) = (ru64(sc + O_SC_T1PICK_LEN), ru64(sc + O_SC_T2PICK_LEN));
                    let total = t1b.wrapping_add(t2b).wrapping_add(t1p).wrapping_add(t2p);
                    let (ban, rule) = (ru64(sc + O_SC_BAN_COUNT), ru8(sc + O_SC_RULE));
                    if total <= 40 && phase_of(total, rule, ban) != 0xFF {
                        stalled = WD_LAST_TOTAL.swap(total, Ordering::Relaxed) == total;
                        if stalled {
                            let n = WD_FRAMES.fetch_add(1, Ordering::Relaxed) + 1;
                            if n % 480 == 0 {
                                core::ptr::write((sc + O_SC_FPRINT_LEN) as *mut u64, 0);
                                core::ptr::write((sc + O_SC_THINK_REC) as *mut u64, u64::MAX);
                                let k = WD_KICKS.fetch_add(1, Ordering::Relaxed) + 1;
                                config::dlog(&format!(
                                    "watchdog: kick#{k} total={total} phase={:#x} (지문·씽킹 캐시 무효화)",
                                    phase_of(total, rule, ban)
                                ));
                            }
                        }
                    }
                }
            }
        }
        if !stalled {
            WD_FRAMES.store(0, Ordering::Relaxed);
        }
    }
    // 진단 덤프 (~5초마다, 메인 스레드)
    if config::get().debug {
        let f = FRAME.fetch_add(1, Ordering::Relaxed);
        // ★밴픽 진행 스냅샷 — "어디서 멈췄나"를 특정하기 위해 1초마다(멈춤 진단용).
        if f % 60 == 0 && IN_BANPICK.load(Ordering::Relaxed) {
            let sc = MY_SCENE.load(Ordering::Relaxed);
            if sc != 0 && addr_ok(sc) {
                unsafe {
                    let (t1b, t2b) = (ru64(sc + O_SC_T1BAN_LEN), ru64(sc + O_SC_T2BAN_LEN));
                    let (t1p, t2p) = (ru64(sc + O_SC_T1PICK_LEN), ru64(sc + O_SC_T2PICK_LEN));
                    let total = t1b.wrapping_add(t2b).wrapping_add(t1p).wrapping_add(t2p);
                    let (ban, rule) = (ru64(sc + O_SC_BAN_COUNT), ru8(sc + O_SC_RULE));
                    config::dlog(&format!(
                        "bp: vec={t1b}.{t2b}.{t1p}.{t2p} total={total} rule={rule} ban={ban} \
                         phase={:#x} state={} timer={:.2} latch={} defer={} anim={} banner={} seq={} myT={:#x}/{:#x} sel={:#x} \
                         trig={} q={} l0m={:#x} l0t={} l0s={} l1={} cd={:.2} g220={:#x}",
                        phase_of(total, rule, ban),
                        core::ptr::read((sc + O_SC_FSM_STATE) as *const u32),
                        f32::from_bits(core::ptr::read((sc + O_SC_FSM_STATE + 4) as *const u32)),
                        ru8(sc + O_SC_FSM_LATCH),
                        ru8(sc + O_SC_FSM_DEFER),
                        core::ptr::read((sc + O_SC_CARD_ANIM) as *const i64),
                        CNT_BANNER.load(Ordering::Relaxed),
                        config::get().seq_for(rule, ban).map(|s| s.len()).unwrap_or(0),
                        MY_T1.load(Ordering::Relaxed),
                        MY_T2.load(Ordering::Relaxed),
                        ru64(sc + 0x3d0),
                        // 코치 위임 죽은 링크 특정(§14.6 후속): 트리거 진입수 / 클라 패킷큐 len /
                        // dedup L0(match·total·step) / L1 지문 len / 펌프 쿨다운 / 가드 +0x220
                        TRIG_N.load(Ordering::Relaxed),
                        ru64(sc + 0x208),
                        ru64(sc + 0xe0),
                        ru64(sc + 0x128),
                        ru8(sc + 0x130),
                        ru64(sc + 0x298),
                        f32::from_bits(core::ptr::read((sc + 0x428) as *const u32)),
                        core::ptr::read((sc + 0x220) as *const u32),
                    ));
                }
            }
        }
        if f % 60 == 0 && IN_BANPICK.load(Ordering::Relaxed) {
            let mut o = String::new();
            for i in 0..PROBE_N {
                let v = PROBE[i].load(Ordering::Relaxed);
                let c = PROBE_CNT[i].load(Ordering::Relaxed);
                if v == u64::MAX {
                    o.push_str(&format!("{}=미호출 ", PROBE_NAME[i]));
                } else {
                    o.push_str(&format!(
                        "{}=t{} r{} b{} →{:#x} (n={}) ",
                        PROBE_NAME[i], v & 0xffff, (v >> 24) & 0xf, (v >> 16) & 0xff,
                        (v >> 32) as u8, c
                    ));
                }
            }
            config::dlog(&format!("probe: {o}"));
        }
        // 정지 진단 해상도: 60프레임(≈1초) 주기. 정지 직전 마지막 상태를 놓치지 않기 위함.
        if f % 60 == 0 {
            config::dlog(&format!(
                "patched: aiturn={} ai6={} hl={} drain={}+{} slot={} cnt={} ai_parity={} sfx={}",
                AITURN_PATCHED.load(Ordering::Relaxed),
                AI6_PATCHED.load(Ordering::Relaxed),
                HL_PATCHED.load(Ordering::Relaxed),
                DRAIN_PATCHED.load(Ordering::Relaxed),
                DRAIN2_PATCHED.load(Ordering::Relaxed),
                SLOTSEL_PATCHED.load(Ordering::Relaxed),
                HLCNT_PATCHED.load(Ordering::Relaxed),
                AI_PATCHED.load(Ordering::Relaxed),
                SFX_PATCHED.load(Ordering::Relaxed),
            ));
            config::dlog(&format!(
                "slotupd={}/{} counters: info={} scalar={} applier={} forced_pick={} custom={} recursion_trip={} max_depth={} lineup_skip={} commit={}/{}",
                CNT_SLOTUPD_OV.load(Ordering::Relaxed),
                CNT_SLOTUPD.load(Ordering::Relaxed),
                CNT_INFO.load(Ordering::Relaxed),
                CNT_SCALAR.load(Ordering::Relaxed),
                CNT_APPLIER.load(Ordering::Relaxed),
                CNT_FORCED_PICK.load(Ordering::Relaxed),
                CUSTOM_ACTIVE.load(Ordering::Relaxed),
                RECURSION_TRIP.load(Ordering::Relaxed),
                MAX_DEPTH_SEEN.load(Ordering::Relaxed),
                CNT_LINEUP_SKIP.load(Ordering::Relaxed),
                CNT_COMMIT_CUSTOM.load(Ordering::Relaxed),
                CNT_COMMIT.load(Ordering::Relaxed),
            ));
            // 턴 오라클 거절 사유 — rej_rlen/rej_state 가 늘면 그 경기는 진행이 멈춘다.
            config::dlog(&format!(
                "turn: n={}/ok={}/cust={} in_bp={} apply_all={} | rej_rlen={} rej_state={} max_rlen={} | aiban={}/{} commit_rej={} same_run={}",
                CNT_TURN.load(Ordering::Relaxed),
                CNT_TURN_OK.load(Ordering::Relaxed),
                CNT_TURN_CUSTOM.load(Ordering::Relaxed),
                IN_BANPICK.load(Ordering::Relaxed) as u8,
                config::get().apply_all as u8,
                REJ_RLEN.load(Ordering::Relaxed),
                REJ_STATE.load(Ordering::Relaxed),
                MAX_RLEN.load(Ordering::Relaxed),
                CNT_AIBAN_CUSTOM.load(Ordering::Relaxed),
                CNT_AIBAN.load(Ordering::Relaxed),
                CNT_COMMIT_REJ.load(Ordering::Relaxed),
                MAX_SAME_COMMIT.load(Ordering::Relaxed),
            ));
        }
    }
}

unsafe fn install() {
    let base = GetModuleHandleW(core::ptr::null());
    if base == 0 {
        return;
    }
    BASE.store(base, Ordering::Relaxed);
    for l in &config::get().load_log {
        config::dlog(&format!("cfg: {l}"));
    }
    // C 먼저(실패 시 커스텀 전체 포기 — A/B는 바닐라 재현이라 설치돼도 무해하지만
    // 순서상 C 성공을 확인하고 진행한다)
    let c = install_detour(RVA_APPLIER, PROLOGUE_APPLIER, hook_applier as usize);
    match c {
        Ok(stub) => TRAMP_APPLIER.store(stub, Ordering::Relaxed),
        Err(e) => {
            config::dlog(&format!("hook C(applier) FAIL: {e} — 커스텀 순서 비활성"));
            return;
        }
    }
    if let Err(e) = install_full(RVA_PHASE_SCENE, PROLOGUE_SCENE, hook_phase_scene as usize) {
        config::dlog(&format!("hook A'(phase_scene) FAIL: {e} — 커스텀 순서 비활성"));
        return;
    }
    // N: 씬 기반 원시 phase leaf(0.5.3 신설) — 흰칸/슬롯 표시의 실제 원천.
    if let Err(e) = install_full(RVA_PHASE_RAW, PROLOGUE_PHASE_RAW, hook_phase_raw as usize) {
        config::dlog(&format!("hook N(phase_raw) FAIL: {e} — 흰칸 바닐라"));
    } else {
        config::dlog("hook N(phase_raw): OK");
    }
    if let Err(e) = install_full(RVA_PHASE_SCALAR, PROLOGUE_SCALAR, hook_phase_scalar as usize) {
        // A'는 이미 바닐라 재현으로 대체됨(무해). B 실패 시 커스텀만 포기.
        config::dlog(&format!("hook B(phase_scalar) FAIL: {e} — 커스텀 순서 비활성"));
        return;
    }
    // G: 서버 AI턴 인라인 phase 패치(0.5.3 — 0.5.2 훅 A의 역할 승계). 실패해도 진행.
    install_aiturn(base);
    // 진단: 트리거 진입 카운터(§14.6 후속 — 코치 위임 죽은 링크 특정). debug 전용.
    if config::get().debug {
        install_trigger_probe(base);
    }
    // H: AI 조합/추천 인라인 phase 6사이트(코치 위임 경로).
    // ⚠2026-07-30: 사이트 3곳(ai_bb1/2/3)의 합류주소·arm 부작용 명령 추출 오류로 크래시 실증
    //   ⟹ 재검증 전까지 cfg 기본 OFF. 켜려면 tfm2_banpick_order.cfg 에 ai_inline_phase=1.
    // I: UI 턴 하이라이트(흰칸, 드레인 밖 사이트 0x193b434) — 표시 전용. cfg 게이트 유지.
    if config::get().ui_highlight {
        install_hl(base);
    }
    // ★드레인(0.5.4 0x1e19640 / 0.5.3 0x1c55300) 내부 인라인 phase 패치 일가(J·K·L·카운트) = **상시 설치**
    // (2026-07-30 확정). "하이라이트에 무효" 판정 자체는 유효하나, 이 드레인 흐름은
    // **코치 위임 자동행동 펌프 + 생각 타이머 진행의 본체**라 load-bearing:
    //   ①K drainA(0.5.4 0x1e1e3b2 / 0.5.3 0x1c5a0b2) = 펌프 게이트(cmp al,dl 일치 시에만
    //     트리거 RVA_TRIGGER → AI턴 요청 disc 0x93 큐잉) — ghidra-re 바이트레벨 확정.
    //   ②J/L/카운트 중 최소 1곳 = state-4 arm 의 진행(생각 타이머 틱→추천 적용) 선행 경로
    //     — K만 상시로는 정지 재현(타이머 동결). ⟹ 네 개를 **세트로** 상시 설치한다.
    // ⚠2026-07-31 회귀 사고: 이 주석만 남고 아래 호출 4줄이 통째로 삭제된 채 v1.2.0 이
    //   릴리스돼 코치 위임이 2번째 픽에서 영구 정지했다(로그 지문 = `drain=0+0
    //   slot=false cnt=false` + `trig` 고정 + state4 `timer` 동결). 지우지 말 것.
    install_drain_hl(base); // J: 드레인 cur/next 2사이트
    install_drain_hl2(base); // K: 드레인 잔여 3사이트(0.5.4 0x1e1e3b2 = 펌프 게이트)
    install_slotsel(base); // L: 슬롯 하이라이트 선택기
    install_hl_count(base); // M: 흰칸 개수 루프
    if config::get().ai_inline_phase {
        install_ai6(base);
    } else {
        config::dlog("hook H(ai inline phase): cfg OFF — 스킵(AI 단계판정 바닐라)");
    }
    // F: 권위 커밋기 — 교차 오염(밴한 챔프 출전) 해결. ★핵심.
    match install_detour(RVA_COMMIT, PROLOGUE_COMMIT, hook_commit as usize) {
        Ok(stub) => TRAMP_COMMIT.store(stub, Ordering::Relaxed),
        Err(e) => config::dlog(&format!("hook F(commit) FAIL: {e} — 교차오염 남음")),
    }
    // D': 턴 오라클 전체 대체 — 팀 순서 지배(AI가 seq대로 행동).
    if let Err(e) = install_turn(base) {
        config::dlog(&format!("hook D'(turn) FAIL: {e} — 팀 순서 바닐라"));
    }
    // O: 슬롯 색 적용기 — ⚠**이 화면에선 호출 0회로 실측**(slotupd=0/0, 2026-07-30).
    //    다른 화면(관전/리플레이) 경로로 판단 ⟹ cfg 게이트 뒤로(기본 OFF).
    if config::get().ui_highlight {
    match install_detour(RVA_SLOTUPD, PROLOGUE_SLOTUPD, hook_slotupd as usize) {
        Ok(stub) => {
            TRAMP_SLOTUPD.store(stub, Ordering::Relaxed);
            config::dlog("hook O(slot color): OK");
        }
        Err(e) => config::dlog(&format!("hook O(slot color) FAIL: {e} — 칸 채움 바닐라")),
    }
    }
    // E: 라인업 적용기 — 크래시 진범(match_ui.rs:4181 unwrap None) 회피. ★핵심.
    match install_detour(RVA_LINEUP, PROLOGUE_LINEUP, hook_lineup as usize) {
        Ok(stub) => TRAMP_LINEUP.store(stub, Ordering::Relaxed),
        Err(e) => config::dlog(&format!("hook E(lineup) FAIL: {e} — 크래시 회피 비활성")),
    }
    // AI 밴 스코어러 파리티 패치(사이드 셀렉터 교정). 실패해도 비치명적(AI 품질만).
    install_ai_parity(base);
    // 셀렉트 효과음 분류 패치(밴/픽 소리 정합). 실패해도 비치명적(연출만).
    install_sfx(base);
    CUSTOM_ACTIVE.store(true, Ordering::Relaxed);
    config::dlog("hooks installed (A'/B phase + G aiturn + C applier + F commit + D' turn + E lineup) — 커스텀 순서 활성");
}
