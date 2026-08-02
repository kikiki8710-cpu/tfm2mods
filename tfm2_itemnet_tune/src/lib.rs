// tfm2_itemnet_tune — 아이템 빌드 추천 신경망(LogisticSGDAgent) 학습 계수 조절
// =====================================================================================
// 배경 (RE 2026-07-31 @0.5.3 buildid 24451609)
//   RE 원본 = mods_report\tfm2_item_tactics\RE\2026-07-31_아이템빌드-신경망-SGD학습경로.md
//             mods_report\tfm2_item_tactics\RE\2026-07-31_아이템빌드-beam탐색-아이템수-의존성.md
//
//   게임은 아이템 빌드 추천에 **전역 단일 로지스틱 회귀** 하나를 쓴다.
//     Database+0xda0 LogisticSGDAgent { +0x00 cap / +0x08 f32* w / +0x10 len(16384) / +0x18 flag }
//     피처는 SipHash-1-3 해싱으로 16384 버킷에 뭉개 넣는다(feature hashing).
//     ★+0x18 은 "fdim"이 아니라 스위치다 — ≠0 이면 forward 가 champ_pos_build·lane_counter
//       피처를 추가하고 update 가 prefix 커리큘럼 루프를 돈다. 바닐라 값 = 1.
//
//   학습(`update` 0x105c060) 수식:
//     z    = Σ x[j]*w[j]                        ← ★update 는 bias(w[0])를 안 더한다(forward 는 더함)
//     p    = 1 / (1 + exp(-z))
//     own  = min((T - buy_time[i]) / T, 1.0)     ← 늦게 산 아이템일수록 작아짐
//     e    = (own*0.8 + 0.2) * (p - y)           ← y = 0.9(승) / 0.1(패), label smoothing
//     e    = clamp(e, -2.0, +2.0)
//     g    = e * 0.01                            ← learning rate
//     for j: w[j] *= 0.99999;  if x[j]!=0 { w[j] -= x[j]*g }
//
//   학습 트리거 = `server.rs tick 0x24625c0` → 0x1951b80 / 0x19708a0 → record 0x1093b50 → update.
//     콜러 폐집합이 전부 game-view\src\logic\server*.rs ⟹ **경기 결과가 서버에 반영될 때만** 학습.
//     리플레이 재생·rayon 백그라운드 sim 은 이 경로를 안 탄다(유력).
//     호출 밀도 = 선수 1명당 update 1회, 그 안에서 구매 아이템 1개당 1 gradient step.
//
//   ★왜 이 모드가 필요한가 (유저 문제 3종의 구조적 원인):
//     ① 패치를 못 따라감 — decay 0.99999 는 매 step 전체 벡터에 곱해진다.
//        반감기 = ln(0.5)/ln(0.99999) ≈ 69,315 step. 경기당 step ≈ 30~60 ⟹ **약 1,200~2,300 경기**.
//        게다가 게임 패치 시 weight 리셋이 **없다**(세이브에 직렬화, 세이브 로드 경로에
//        base_item_network 에셋 재시딩 없음 = 커리어 단위 영구 누적).
//     ② 경기 수가 적어 느림 — lr 0.01 + 시간가중(늦게 산 아이템은 최소 0.2배)의 이중 감쇠.
//     ③ 모드 아이템이 너무 많음 — 바닐라 후보는 tier4 **6개**뿐인데(컬렉션 30개 = 6카테고리×5티어),
//        모드템 M개를 얹으면 같은 16384 버킷·같은 lr 로 6+M 개를 서열매겨야 한다.
//        게다가 모드템은 base_item_network 사전학습 가중치가 **0**이라 콜드스타트다.
//
// 방식 A — **상수 리포인트 (기본 ON)**
//   모든 계수가 `.rdata` rip-relative 로드다(즉시값 리터럴 0개). 각 명령은
//   `movss/mulss/movaps xmm, [rip+disp32]` 고정 길이라 **disp32 4바이트만** 우리 f32 풀로 돌리면
//   계수 전부가 cfg 제어 하에 들어온다. 게임 코드가 계산을 그대로 수행하므로 AV 리스크가 없다.
//   ⟹ 풀은 우리 메모리라 **재시작 없이 cfg 핫리로드**가 된다(값만 덮어쓰면 즉시 반영).
//
//   ⚠ 공유 상수의 값을 직접 고치면 안 된다 — 1.0 은 exe 전역 2072곳, 0.01 은 43곳이 쓴다.
//     반드시 disp32 리포인트. (decay 0.99999 만 xref 4개 전부가 update 라 값 패치도 가능하지만,
//     일관성과 되돌리기 편의를 위해 이것도 리포인트한다.)
//
//   ⛔ 노브로 노출하지 않는 상수 2종 (실측으로 걸러냄 — 노출하면 사고):
//     · 1.0 (`0x105c208`/`0x105c58e`) = `min(own,1.0)` 의 상한이 아니라 **시그모이드 분자**다.
//       0x105c380: movaps xmm1,xmm14(1.0) / divss xmm1,xmm0 / subss xmm1,xmm6 → p-y.
//       루프 불변 레지스터라 이중 용도이므로 건드리면 시그모이드가 깨진다.
//     · -1.0 (`0x1058d75`) = `shr esi,9 / or 0x3f800000 / addss -1.0` = **U[0,1) 난수 생성 관용구**.
//       0.5 로 바꾸면 난수 분포가 [0.5,1.5)로 밀린다. 조절 대상은 뒤의 ×range, +offset 뿐.
//
// 방식 B — **update 대체 구현 (기본 OFF, `replace_update=1`)**
//   `0x105c060` 프롤로그 12B(push ×8)를 훔쳐 Rust 재구현으로 보낸다. 바이트패치로는 못 하는 것
//   (bias 비대칭 교정, decay 를 건드린 피처에만 적용 등 **구조 변경**)을 위한 자리다.
//   ⚠ **미검증**이다. 피처벡터 x[] 를 게임 dense 빌더(0x1058350) shadow-CALL 로 얻으므로 AV 위험이
//     있어 cfg 게이트로 격리했다(CLAUDE.md §3 "위험 shadow-call 은 기본 OFF").
//   ⚠ **완전 재구현(§3 원칙)은 아직 불가** — dense 빌더(0x1058350)와 피처 해시(0x1044600)가
//     각 피처에 무엇을 concat 해 SipHash 하는지가 미규명이다. 그 RE 1건이 선행돼야 게임 헬퍼
//     호출 없이 x[] 를 만들 수 있다. 그때까지 B 는 "수식만 우리 것"인 부분 대체다.
//
// 빌드: powershell -File C:\tfm2mods\build_inj.ps1 -Src C:\tfm2mods\tfm2_itemnet_tune\src\lib.rs -ModId tfm2_itemnet_tune
// =====================================================================================

use mod_api::*;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::sync::Mutex;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

const MOD_ID: &str = "tfm2_itemnet_tune";
const CFG_NAME: &str = "tfm2_itemnet_tune.cfg";
const LOG_NAME: &str = "tfm2_itemnet_tune.txt";

// ─────────────────────────────────────────────────────────────────────────────
// 계수 (기본값 = 0.5.3 게임 실측값. 전부 19/19 바이트 대조 통과)
// ─────────────────────────────────────────────────────────────────────────────
#[derive(Clone, Copy, PartialEq)]
struct Coef {
    /// 매 gradient step 마다 **전체 16384 벡터**에 곱해지는 망각 계수. 게임값 0.99999.
    /// 반감기(step) = ln(0.5)/ln(decay). 0.9999 로 낮추면 반감기가 10배 짧아진다(≈120~230 경기).
    decay: f32,
    /// learning rate. 게임값 0.01. 올리면 적은 경기로도 빨리 움직이지만 진동한다.
    lr: f32,
    /// gradient 클리핑 하한/상한. 게임값 -2.0 / +2.0.
    clamp_lo: f32,
    clamp_hi: f32,
    /// 시간가중 `own*a + b`. 게임값 a=0.8, b=0.2.
    /// b=1.0, a=0.0 으로 두면 "언제 샀든 똑같이 학습"이 된다(늦게 사는 3~4번째 아이템 학습 가속).
    time_w_a: f32,
    time_w_b: f32,
    /// 라벨. 게임값 승 0.9 / 패 0.1 (label smoothing).
    /// 1.0 / 0.0 으로 두면 스무딩이 꺼져 신호가 세진다.
    label_win: f32,
    label_lose: f32,
    /// forward(추론) 탐색 노이즈 `U[0,1)*range + offset`. 게임값 range=0.2, offset=-0.1 ⟹ ±0.1.
    /// ★모드 아이템 콜드스타트 대응 노브 — 키우면 탐색이 늘어 새 아이템이 빌드에 더 자주 들어가고,
    ///   그만큼 학습 표본이 생긴다. 대신 빌드 품질 분산이 커진다.
    ///   중앙값을 0 으로 유지하려면 offset = -range/2 로 둘 것.
    noise_range: f32,
    noise_offset: f32,
}

impl Coef {
    /// 게임 0.5.3 실측 기본값
    const fn vanilla() -> Self {
        Coef {
            decay: 0.99999,
            lr: 0.01,
            clamp_lo: -2.0,
            clamp_hi: 2.0,
            time_w_a: 0.8,
            time_w_b: 0.2,
            label_win: 0.9,
            label_lose: 0.1,
            noise_range: 0.2,
            noise_offset: -0.1,
        }
    }
}

/// 방식 B(대체 구현) 전용 구조 옵션. 바이트패치로는 표현할 수 없는 것들.
#[derive(Clone, Copy, PartialEq)]
struct ReplOpt {
    /// update 를 Rust 재구현으로 대체할지. **기본 0(끔)** — 미검증 + shadow-CALL AV 위험.
    enabled: bool,
    /// 게임은 forward 에서만 w[0]을 bias 로 쓰고 update 에선 안 쓴다(코드 비대칭).
    /// 1 로 두면 update 에서도 z 에 w[0]을 더해 학습/추론을 일치시킨다.
    bias_in_update: bool,
    /// 0 = 게임과 동일(매 step 전체 벡터 감쇠).
    /// 1 = **이번 step 에 등장한 피처만** 감쇠 ⟹ 안 쓰인 아이템의 가중치가 0으로 흘러내리지 않는다.
    ///     표본이 희소한 모드 아이템이 decay 에 잡아먹히는 것을 막는 용도.
    decay_touched_only: bool,
    /// 챔피언 id ↔ 이름 수집(읽기 전용 peek). **기본 ON** — 게임 동작에 개입하지 않는다.
    /// 결과 = `champ_ids.txt`. 모드 챔피언을 오프라인 채점에 포함시키려면 이게 필요하다.
    /// ⚠`replace_update=1` 과는 같은 지점을 쓰므로 동시 설치 불가(그때는 자동으로 꺼진다).
    champ_probe: bool,
}

impl ReplOpt {
    const fn off() -> Self {
        ReplOpt {
            enabled: false,
            bias_in_update: false,
            decay_touched_only: false,
            champ_probe: true, // 읽기 전용이라 기본 ON
        }
    }
}

/// 방식 B 전용 — **경기 지표를 학습에 끌어오는 계수**.
/// 게임 원본에는 없는 입력이라 바이트패치로는 표현할 수 없다.
///
/// 두 축을 섞지 않는 것이 설계 원칙이다:
///   · 개인 기록(MVP·KDA·딜·탱) → **표본 가중치** `quality` (이 표본을 얼마나 신뢰할 것인가)
///   · 팀 결과(점수차·경기시간) → **라벨** `y` (얼마나 확실한 승리였나)
/// 개인 지표를 라벨에 섞으면 팀 승패 신호가 오염된다.
///
/// ★**모든 가중치 기본값 = 0** ⟹ 켜기 전에는 게임 원본과 수치적으로 동일하다(A/B 비교용).
#[derive(Clone, Copy, PartialEq)]
struct Shape {
    // ── 표본 가중치 quality ──
    /// 팀 내 MVP(= `statistics.rating` 최대)면 가산.
    w_mvp: f32,
    /// 킬 지분 편차 `kill/팀킬합 − 0.2` 에 곱한다(5인이므로 평균 지분 = 0.2).
    /// ★절대 킬 수가 아니라 **지분**을 쓰는 이유 = 난타전 경기가 통째로 과대평가되는 것을 막기 위해.
    w_kill: f32,
    /// 딜 지분 편차. ⚠딜은 아이템에 직접 의존하므로 자기충족적 신호가 섞인다 — 작게 쓸 것.
    w_deal: f32,
    /// 탱킹 지분 편차. ⚠같은 이유로 작게.
    w_tank: f32,
    /// 데스 지분 편차에 곱하는 **감점**. 아래 `tank_eff` 로 나눠진다.
    w_death: f32,
    /// **탱킹 효율** = (본인 탱킹/데스) ÷ (팀 평균 탱킹/데스). 1.0 = 팀 평균.
    /// 데스 감점을 이 값으로 나눈다 ⟹ 탱커처럼 "많이 죽었지만 그만큼 받아낸" 경우 감점이 줄고,
    /// "탱킹 없이 죽기만 한" 경우 감점이 커진다. 데스가 0이면 분모를 1로 보아 효율이 커진다.
    /// clamp 범위 — 상한이 없으면 데스 0인 선수가 학습을 지배한다.
    tank_eff_min: f32,
    tank_eff_max: f32,
    /// 최종 quality clamp. 한 경기가 학습을 지배하는 것을 막는다.
    quality_min: f32,
    quality_max: f32,

    // ── 라벨 등급화 ──
    /// 점수차(킬 스코어) 정규화값 `|내점수−상대점수| / 총점` 에 곱한다.
    y_margin_w: f32,
    /// 속도 `clamp(1 − 경기초/y_ref_sec, 0, 1)` 에 곱한다. 빨리 끝날수록 1에 가깝다.
    y_speed_w: f32,
    /// 속도 기준 경기 길이(초). 이 시간이면 speed=0, 절반이면 0.5.
    y_ref_sec: f32,
}

impl Shape {
    /// 전부 0 = 게임 원본과 동일한 학습
    const fn off() -> Self {
        Shape {
            w_mvp: 0.0,
            w_kill: 0.0,
            w_deal: 0.0,
            w_tank: 0.0,
            w_death: 0.0,
            tank_eff_min: 0.25,
            tank_eff_max: 4.0,
            quality_min: 0.3,
            quality_max: 2.5,
            y_margin_w: 0.0,
            y_speed_w: 0.0,
            y_ref_sec: 1800.0,
        }
    }
    /// 가중치가 전부 0이면 quality 계산 자체를 건너뛴다(= World 접근 불필요).
    fn quality_active(&self) -> bool {
        self.w_mvp != 0.0
            || self.w_kill != 0.0
            || self.w_deal != 0.0
            || self.w_tank != 0.0
            || self.w_death != 0.0
    }
    fn grade_active(&self) -> bool {
        self.y_margin_w != 0.0 || self.y_speed_w != 0.0
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 상수 참조 사이트 (0.5.3 buildid 24451609 — 2026-07-31 전건 바이트 실측)
//   L = 명령 전체 길이. disp32 는 항상 명령의 마지막 4바이트(rip-rel 규칙).
//   PRE = disp32 앞의 오피코드 바이트(= L-4 바이트). 이게 어긋나면 RVA stale 이므로 건드리지 않는다.
// ─────────────────────────────────────────────────────────────────────────────
struct Site {
    label: &'static str,
    rva: usize,
    len: u8,
    pre: &'static [u8],
    slot: u8, // POOL 슬롯 인덱스
}

// 풀 슬롯 인덱스
const S_DECAY: u8 = 0; // +0x00, 16B packed (movaps 용 16바이트 정렬 필수). scalar 도 lane0 을 공유.
const S_LR: u8 = 1;
const S_CLAMP_LO: u8 = 2;
const S_CLAMP_HI: u8 = 3;
const S_TW_A: u8 = 4;
const S_TW_B: u8 = 5;
const S_LABEL_WIN: u8 = 6;
const S_LABEL_LOSE: u8 = 7;
const S_NOISE_RANGE: u8 = 8;
const S_NOISE_OFFSET: u8 = 9;
const SLOT_COUNT: usize = 10;

/// 슬롯 → 풀 내부 바이트 오프셋. 0번(decay)만 16B(packed ×4), 나머지는 4B.
fn slot_off(slot: u8) -> usize {
    if slot == S_DECAY { 0 } else { 0x10 + (slot as usize - 1) * 4 }
}
const POOL_SIZE: usize = 0x10 + (SLOT_COUNT - 1) * 4;

const SITES: &[Site] = &[
    // ── update 0x105c060 : 커리큘럼(flag≠0) 경로 ──
    Site { label: "y_lose",       rva: 0x105c14f, len: 8, pre: &[0xf3,0x0f,0x10,0x35],           slot: S_LABEL_LOSE },
    Site { label: "y_win",        rva: 0x105c178, len: 8, pre: &[0xf3,0x0f,0x10,0x35],           slot: S_LABEL_WIN },
    Site { label: "tw_a",         rva: 0x105c211, len: 9, pre: &[0xf3,0x44,0x0f,0x10,0x0d],      slot: S_TW_A },
    Site { label: "tw_b",         rva: 0x105c21a, len: 9, pre: &[0xf3,0x44,0x0f,0x10,0x15],      slot: S_TW_B },
    Site { label: "decay_s",      rva: 0x105c227, len: 9, pre: &[0xf3,0x44,0x0f,0x10,0x25],      slot: S_DECAY },
    Site { label: "decay_p",      rva: 0x105c230, len: 8, pre: &[0x44,0x0f,0x28,0x3d],           slot: S_DECAY },
    Site { label: "clamp_lo",     rva: 0x105c391, len: 8, pre: &[0xf3,0x0f,0x10,0x0d],           slot: S_CLAMP_LO },
    Site { label: "clamp_hi",     rva: 0x105c39e, len: 8, pre: &[0xf3,0x0f,0x10,0x05],           slot: S_CLAMP_HI },
    Site { label: "lr",           rva: 0x105c3aa, len: 8, pre: &[0xf3,0x0f,0x59,0x05],           slot: S_LR },
    // ── update 0x105c060 : 비커리큘럼(flag==0) 대체 경로. 바닐라 에셋은 flag=1 이라 평소엔 안 돌지만
    //    에셋/세이브 값에 따라 돌 수 있으므로 양쪽 다 패치해야 계수가 일관된다. ──
    Site { label: "clamp_lo_b",   rva: 0x105c5a2, len: 8, pre: &[0xf3,0x0f,0x10,0x15],           slot: S_CLAMP_LO },
    Site { label: "clamp_hi_b",   rva: 0x105c5ae, len: 8, pre: &[0xf3,0x0f,0x10,0x05],           slot: S_CLAMP_HI },
    Site { label: "lr_b",         rva: 0x105c5ba, len: 8, pre: &[0xf3,0x0f,0x59,0x05],           slot: S_LR },
    Site { label: "decay_p_b",    rva: 0x105c5d8, len: 7, pre: &[0x0f,0x28,0x0d],                slot: S_DECAY },
    Site { label: "decay_s_b",    rva: 0x105c6e6, len: 8, pre: &[0xf3,0x0f,0x10,0x0d],           slot: S_DECAY },
    // ── forward 0x10587e0 : 탐색 노이즈 ──
    Site { label: "noise_range",  rva: 0x1058d7e, len: 9, pre: &[0xf3,0x44,0x0f,0x59,0x05],      slot: S_NOISE_RANGE },
    Site { label: "noise_offset", rva: 0x1058d87, len: 9, pre: &[0xf3,0x44,0x0f,0x58,0x05],      slot: S_NOISE_OFFSET },
];

/// 기대 원본 목표 .rdata RVA (stale 판정 보강용 — pre 만으로는 다른 상수를 읽는 쌍둥이 명령을
/// 구별 못 하므로, "지금 이 명령이 실제로 가리키는 값"까지 대조한다).
fn expect_orig_value(slot: u8) -> f32 {
    let v = Coef::vanilla();
    match slot {
        S_DECAY => v.decay,
        S_LR => v.lr,
        S_CLAMP_LO => v.clamp_lo,
        S_CLAMP_HI => v.clamp_hi,
        S_TW_A => v.time_w_a,
        S_TW_B => v.time_w_b,
        S_LABEL_WIN => v.label_win,
        S_LABEL_LOSE => v.label_lose,
        S_NOISE_RANGE => v.noise_range,
        S_NOISE_OFFSET => v.noise_offset,
        _ => f32::NAN,
    }
}

// ── update 대체 구현용 (방식 B) ──
const RVA_UPDATE: usize = 0x105c060;
/// push r15/r14/r13/r12/rsi/rdi/rbp/rbx = 정확히 12바이트. `movabs rax,imm64; jmp rax` 와 같은 길이다.
/// 진입 시 rax 는 인자가 아니므로(win64) 파괴해도 안전하다.
const UPDATE_PROLOGUE: [u8; 12] = [
    0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x55, 0x53,
];
/// dense 피처벡터 빌더. 0x105c1a8 콜사이트에서 인자 규약 확정:
///   lea rcx,[rsp+0x38] / mov rdx,r14(=agent+0x10) / xor r8d,r8d / mov r9,ctx
///   [rsp+0x20]=items / [rsp+0x28]=n_items
const RVA_DENSE: usize = 0x1058350;

// ── 경기 지표 수집 (방식 B 전용) ──
// RE 원본 = REPORT\tfm2_itemnet_tune\RE\2026-07-31_경기지표-World-GamePlayer-레이아웃.md
// ★핵심 = `record` 의 **2번째 인자가 곧 `&World`** 다. 학습 시점에 sim 상태가 살아 있으므로
//   세이브/DB 조회 없이 선수별 KDA·딜·탱·레이팅과 팀 스코어·경기틱을 전부 직접 읽을 수 있다.
const RVA_RECORD: usize = 0x1093b50;
/// push rbp/r15/r14/r13/r12/rsi/rdi/rbx = 정확히 12바이트.
const RECORD_PROLOGUE: [u8; 12] = [
    0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x53,
];

// World (size 0xEEC8)
const W_PLAYERS_PTR: usize = 0x840;
const W_PLAYERS_LEN: usize = 0x848;
const W_TICK: usize = 0xeb00;
const W_SCORES: usize = 0xeb08; // [u64;2]
const PLAYER_STRIDE: usize = 0x8d0;

// GamePlayer (stride 0x8d0). statistics(PlayerStatistics, 0x170) 는 +0x530 에 인라인돼 있다.
/// champion_name: String { cap@+0x418, ptr@+0x420, len@+0x428 }
const P_CHAMP_NAME_PTR: usize = 0x420;
const P_CHAMP_NAME_LEN: usize = 0x428;
const P_ATHLETE_ID: usize = 0x810;
const P_TEAM: usize = 0x820;
const P_POSITION: usize = 0x8b0;
const P_RATING: usize = 0x578; // statistics.rating — MVP 판정 기준
const P_KILL: usize = 0x5a0;
const P_DEATH: usize = 0x5a8;
const P_ASSIST: usize = 0x5b0;
const P_DEAL: usize = 0x5c0;
const P_TANK: usize = 0x5c8;
#[allow(dead_code)]
const P_HEAL: usize = 0x5d0;

// MatchResult = [TeamMatchInfo(0x670); 2].  won = MR + 0x668 + team*0x670
const MR_TEAM_STRIDE: usize = 0x670;
const MR_WON: usize = 0x668;

/// ctx(0x58B) — [0..5] 아군 챔프 id / [5..10] 적군 / [10] 본인 레인(0~4)
const CTX_LANE: usize = 0x50;

/// 경기시간 단위. ★`World.tick` 은 **60틱 = 1초** (match_result UI 가 `tick/60`=초, `tick/3600`=분).
/// ⚠리플레이 `MatchReplayData.game_tick` 은 **30틱/초**로 다른 축이다 — 섞으면 2배 틀어진다.
const TICKS_PER_SEC: f32 = 60.0;

// ─────────────────────────────────────────────────────────────────────────────
// Win32
// ─────────────────────────────────────────────────────────────────────────────
type BOOL = i32;
type DWORD = u32;
type HMODULE = usize;
#[link(name = "kernel32")]
extern "system" {
    fn GetModuleHandleW(name: *const u16) -> usize;
    fn GetModuleHandleExW(flags: u32, addr: *const u16, h: *mut HMODULE) -> BOOL;
    fn GetModuleFileNameW(h: HMODULE, buf: *mut u16, sz: DWORD) -> DWORD;
    fn VirtualProtect(addr: usize, size: usize, new_protect: u32, old: *mut u32) -> BOOL;
    fn VirtualQuery(addr: *const core::ffi::c_void, buf: *mut MemBasicInfo, len: usize) -> usize;
    fn VirtualAlloc(addr: usize, size: usize, alloc_type: u32, protect: u32) -> usize;
    fn FlushInstructionCache(proc: usize, addr: usize, sz: usize) -> BOOL;
    fn GetCurrentProcess() -> usize;
    fn GetProcessHeap() -> usize;
    fn HeapFree(heap: usize, flags: u32, mem: usize) -> BOOL;
}
#[repr(C)]
#[derive(Default)]
struct MemBasicInfo {
    base: usize,
    alloc_base: usize,
    alloc_protect: u32,
    _p0: u32,
    region_size: usize,
    state: u32,
    protect: u32,
    typ: u32,
    _p1: u32,
}

#[inline]
unsafe fn readable(addr: usize, len: usize) -> bool {
    if addr < 0x10000 || len == 0 {
        return false;
    }
    let mut mbi = MemBasicInfo::default();
    if VirtualQuery(addr as *const _, &mut mbi, core::mem::size_of::<MemBasicInfo>()) == 0 {
        return false;
    }
    const COMMIT: u32 = 0x1000;
    const RD: u32 = 0x02 | 0x04 | 0x20 | 0x40;
    const GUARD: u32 = 0x01 | 0x100;
    if mbi.state != COMMIT || mbi.protect & GUARD != 0 || mbi.protect & RD == 0 {
        return false;
    }
    addr + len <= mbi.base + mbi.region_size
}

// ─────────────────────────────────────────────────────────────────────────────
// 로그 / 경로
// ─────────────────────────────────────────────────────────────────────────────
fn now_ms() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0)
}
fn dir() -> Option<PathBuf> {
    unsafe {
        let mut h: HMODULE = 0;
        if GetModuleHandleExW(0x4 | 0x2, dir as *const () as *const u16, &mut h) == 0 || h == 0 {
            return None;
        }
        let mut b = [0u16; 4096];
        let n = GetModuleFileNameW(h, b.as_mut_ptr(), b.len() as DWORD);
        if n == 0 {
            return None;
        }
        let mut p = PathBuf::from(String::from_utf16_lossy(&b[..n as usize]));
        p.pop();
        Some(p)
    }
}
fn log(s: &str) {
    if let Some(mut p) = dir() {
        p.push(LOG_NAME);
        if let Ok(mut f) = fs::OpenOptions::new().create(true).append(true).open(&p) {
            let _ = write!(f, "{}", s);
            let _ = f.flush();
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// cfg
// ─────────────────────────────────────────────────────────────────────────────
const CFG_TEMPLATE: &str = "\
# tfm2_itemnet_tune — 아이템 빌드 추천 신경망(LogisticSGD) 학습 계수
#
# 게임의 학습 수식 (0.5.3 실측):
#   p    = 1 / (1 + exp(-Σ x[j]*w[j]))
#   own  = min((경기길이 - 구매시각) / 경기길이, 1.0)      # 늦게 산 아이템일수록 작다
#   e    = (own*time_w_a + time_w_b) * (p - y)            # y = label_win / label_lose
#   e    = clamp(e, clamp_lo, clamp_hi)
#   g    = e * lr
#   모든 j:  w[j] *= decay ;  x[j]!=0 이면  w[j] -= x[j]*g
#
# 학습은 '경기 결과가 반영될 때'만 돈다(리플레이 재생은 학습 안 함).
# 호출 밀도 = 선수 1명당 1회 → 그 안에서 구매 아이템 1개당 1 step.
#
# ★ 값을 비우거나 줄을 지우면 게임 기본값을 쓴다. 파일을 저장하면 재시작 없이 즉시 반영된다.
# ────────────────────────────────────────────────────────────────────────────

# [망각 속도] 매 step 마다 전체 가중치에 곱한다. 게임 기본 0.99999.
#   반감기(경기) ≈ ln(0.5)/ln(decay) / 45      (경기당 약 45 step 가정)
#     0.99999  → 약 1,500 경기   (기본. 게임 패치를 사실상 못 따라간다)
#     0.9999   → 약 150 경기
#     0.999    → 약 15 경기      (너무 빠르면 메타가 안 굳는다)
#   ※ 패치 주기에 맞추고 싶다면 '한 시즌 경기 수' 근처를 노려볼 것.
decay = 0.99999

# [학습률] 게임 기본 0.01. 올리면 적은 경기로도 빨리 움직이지만 진동한다.
lr = 0.01

# [gradient 클리핑] 게임 기본 -2.0 / 2.0. lr 을 크게 올릴 때 같이 좁히면 폭주를 막는다.
clamp_lo = -2.0
clamp_hi = 2.0

# [구매 시점 가중] 가중치 = own*time_w_a + time_w_b   (own = 소유 시간 비율 0~1)
#   기본 0.8 / 0.2 → 마지막에 산 아이템은 학습량이 1/5 로 줄어든다.
#   0.0 / 1.0 로 두면 '언제 샀든 동등' → 3~4번째 아이템 학습이 5배 빨라진다.
time_w_a = 0.8
time_w_b = 0.2

# [라벨] 게임 기본 0.9 / 0.1 (label smoothing).
#   1.0 / 0.0 으로 두면 스무딩이 꺼져 승패 신호가 세진다(수렴은 빠르고 과적합은 늘어난다).
label_win = 0.9
label_lose = 0.1

# [탐색 노이즈] 빌드 생성 시 점수에 U[0,1)*noise_range + noise_offset 을 더한다.
#   기본 0.2 / -0.1 → ±0.1.  중앙값을 0 으로 두려면 offset = -range/2.
#   ★모드 아이템은 사전학습 가중치가 0이라 콜드스타트다. 노이즈를 키우면 새 아이템이
#     빌드에 더 자주 들어가 학습 표본이 생긴다. 대신 빌드 품질 분산이 커진다.
noise_range = 0.2
noise_offset = -0.1

# ────────────────────────────────────────────────────────────────────────────
# [대체 구현] update 함수를 Rust 재구현으로 통째 교체한다.
#   ⚠ 미검증 + 게임 함수 shadow-CALL 을 쓴다. 위 계수만 바꿀 거면 0 으로 둘 것.
#   1 로 켜면 아래 두 옵션이 추가로 쓸 수 있게 된다(바이트패치로는 불가능한 구조 변경).
replace_update = 0

#   게임은 forward 에서만 w[0]을 bias 로 쓰고 update 에선 안 쓴다(게임 코드의 비대칭).
#   1 = update 에서도 bias 를 반영해 학습과 추론을 일치시킨다.
bias_in_update = 0

#   0 = 게임과 동일(매 step 전체 벡터 감쇠)
#   1 = 이번 step 에 등장한 피처만 감쇠 → 안 쓰인 아이템 가중치가 0으로 흘러내리지 않는다.
#       (표본이 희소한 모드 아이템이 decay 에 잡아먹히는 것을 막는 용도)
decay_touched_only = 0

# ────────────────────────────────────────────────────────────────────────────
# [경기 지표 반영]  ⚠ replace_update = 1 일 때만 동작한다.
#
# 게임 원본은 '이겼나/졌나' 딱 하나만 보고 학습한다. 아래 값을 켜면 경기 내용을 반영한다.
#   e = (시간가중) × quality × (p − y)
#         quality = 개인 기록 → 이 표본을 얼마나 신뢰할 것인가
#         y       = 팀 결과   → 얼마나 확실한 승리였나
#   개인 기록을 y 에 섞지 않는 이유 = 팀 승패 신호가 오염되기 때문.
#
# ★전부 0 = 게임 원본과 동일하게 학습한다. 하나씩 켜서 비교해 보는 것을 권장.
#
#   quality = 1 + w_mvp·MVP + w_kill·Δ킬지분 + w_deal·Δ딜지분 + w_tank·Δ탱지분
#               − w_death·Δ데스지분 ÷ 탱킹효율
#     Δ지분 = (본인 / 팀합계) − 0.2      # 5인 팀이므로 평균 지분 0.2
#     탱킹효율 = (내 데스당 탱킹) ÷ (팀 평균 데스당 탱킹)     # 1.0 = 팀 평균

# MVP(팀 내 rating 최고)면 가산. 0.3 이면 MVP 경기의 학습량이 30% 늘어난다.
w_mvp = 0

# 킬 지분이 평균보다 높으면 가산. 1.0 이면 '혼자 팀 킬 절반(0.5)' 일 때 +0.3.
w_kill = 0

# ⚠딜·탱은 아이템에 직접 의존한다(공격템 사면 딜이 오른다). 자기충족적 신호가 섞이므로
#   킬/데스보다 작게 두는 것을 권장한다. 0.2~0.5 정도부터 시험해 볼 것.
w_deal = 0
w_tank = 0

# 데스 지분이 평균보다 높으면 감점. 단 아래 탱킹효율로 나눠진다 —
#   탱커처럼 '많이 죽었지만 그만큼 받아낸' 경우 감점이 줄고,
#   '탱킹 없이 죽기만 한' 경우 감점이 커진다.
w_death = 0

# 탱킹효율 clamp. 상한이 없으면 데스 0인 선수가 학습을 지배한다.
tank_eff_min = 0.25
tank_eff_max = 4.0

# 최종 quality clamp. 한 경기가 학습을 지배하는 것을 막는다.
quality_min = 0.3
quality_max = 2.5

# [승리의 질 → 라벨]  y = 0.5 ± (0.4 + y_margin_w·점수차비율 + y_speed_w·속도)
#   점수차비율 = |내킬 − 상대킬| / 총킬        (0~1)
#   속도       = clamp(1 − 경기초/y_ref_sec, 0, 1)   빠를수록 1
#   둘 다 0 이면 위의 label_win/label_lose 를 그대로 쓴다(= 게임 원본).
#   합쳐서 0.1 을 넘기면 압승 라벨이 1.0 에 붙는다 — 각 0.05 정도부터 권장.
y_margin_w = 0
y_speed_w = 0
# 경기 길이 기준(초). 이 시간이면 속도 0, 절반이면 0.5.
# ★World.tick 은 60틱 = 1초다(리플레이 game_tick 의 30틱/초와 다른 축).
y_ref_sec = 1800
";

fn parse_cfg(text: &str) -> (Coef, ReplOpt, Shape, Vec<String>) {
    let mut c = Coef::vanilla();
    let mut r = ReplOpt::off();
    let mut sh = Shape::off();
    let mut notes: Vec<String> = Vec::new();

    for raw in text.trim_start_matches('\u{feff}').lines() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') || line.starts_with("//") {
            continue;
        }
        let (k, v) = match line.split_once('=') {
            Some((k, v)) => (k.trim(), v.trim()),
            None => continue,
        };
        if v.is_empty() {
            continue; // 빈 값 = 기본값 유지
        }
        let fv = v.parse::<f32>();
        let bv = || matches!(v, "1" | "true" | "TRUE" | "on" | "ON" | "yes");

        /// 대상 구조체를 명시하는 일반형
        macro_rules! sets {
            ($t:ident, $field:ident, $lo:expr, $hi:expr) => {{
                match fv {
                    Ok(x) if x.is_finite() && x >= $lo && x <= $hi => $t.$field = x,
                    Ok(x) => notes.push(format!("{} = {} 은 허용 범위({}~{}) 밖 → 기본값 유지", k, x, $lo, $hi)),
                    Err(_) => notes.push(format!("{} 파싱 실패('{}') → 기본값 유지", k, v)),
                }
            }};
        }
        macro_rules! setf {
            ($field:ident, $lo:expr, $hi:expr) => {{ sets!(c, $field, $lo, $hi) }};
        }

        match k {
            // decay 는 1.0 을 허용한다(= 망각 없음). 1.0 초과는 발산이라 금지.
            "decay" => setf!(decay, 0.0, 1.0),
            "lr" => setf!(lr, 0.0, 10.0),
            "clamp_lo" => setf!(clamp_lo, -1.0e6, 0.0),
            "clamp_hi" => setf!(clamp_hi, 0.0, 1.0e6),
            "time_w_a" => setf!(time_w_a, -10.0, 10.0),
            "time_w_b" => setf!(time_w_b, -10.0, 10.0),
            "label_win" => setf!(label_win, -1.0, 2.0),
            "label_lose" => setf!(label_lose, -1.0, 2.0),
            "noise_range" => setf!(noise_range, 0.0, 100.0),
            "noise_offset" => setf!(noise_offset, -100.0, 100.0),
            "replace_update" => r.enabled = bv(),
            "bias_in_update" => r.bias_in_update = bv(),
            "decay_touched_only" => r.decay_touched_only = bv(),
            "champ_probe" => r.champ_probe = bv(),
            // ── 경기 지표 (방식 B 전용) ──
            "w_mvp" => sets!(sh, w_mvp, -5.0, 5.0),
            "w_kill" => sets!(sh, w_kill, -10.0, 10.0),
            "w_deal" => sets!(sh, w_deal, -10.0, 10.0),
            "w_tank" => sets!(sh, w_tank, -10.0, 10.0),
            "w_death" => sets!(sh, w_death, -10.0, 10.0),
            "tank_eff_min" => sets!(sh, tank_eff_min, 0.01, 1.0),
            "tank_eff_max" => sets!(sh, tank_eff_max, 1.0, 100.0),
            "quality_min" => sets!(sh, quality_min, 0.0, 1.0),
            "quality_max" => sets!(sh, quality_max, 1.0, 100.0),
            "y_margin_w" => sets!(sh, y_margin_w, 0.0, 0.5),
            "y_speed_w" => sets!(sh, y_speed_w, 0.0, 0.5),
            "y_ref_sec" => sets!(sh, y_ref_sec, 10.0, 100000.0),
            _ => notes.push(format!("알 수 없는 항목 '{}' 무시", k)),
        }
    }

    // sanity: clamp 가 뒤집히면 maxss/minss 조합이 항상 상수를 뱉는다.
    if c.clamp_lo > c.clamp_hi {
        notes.push(format!(
            "clamp_lo({}) > clamp_hi({}) — 뒤집혀 있어 기본값으로 되돌림",
            c.clamp_lo, c.clamp_hi
        ));
        c.clamp_lo = Coef::vanilla().clamp_lo;
        c.clamp_hi = Coef::vanilla().clamp_hi;
    }
    if sh.tank_eff_min > sh.tank_eff_max {
        notes.push("tank_eff_min > tank_eff_max — 뒤집혀 있어 기본값으로 되돌림".into());
        sh.tank_eff_min = Shape::off().tank_eff_min;
        sh.tank_eff_max = Shape::off().tank_eff_max;
    }
    if sh.quality_min > sh.quality_max {
        notes.push("quality_min > quality_max — 뒤집혀 있어 기본값으로 되돌림".into());
        sh.quality_min = Shape::off().quality_min;
        sh.quality_max = Shape::off().quality_max;
    }
    // 지표 가중치는 대체 구현 경로에서만 의미가 있다 — 켜두고 replace_update=0 이면 조용히 무시된다.
    if !r.enabled && (sh.quality_active() || sh.grade_active()) {
        notes.push("경기 지표 가중치가 설정됐지만 replace_update=0 이라 적용되지 않습니다".into());
    }
    (c, r, sh, notes)
}

fn cfg_path() -> Option<PathBuf> {
    dir().map(|mut p| {
        p.push(CFG_NAME);
        p
    })
}

/// 반환 = (계수, 대체옵션, 지표계수, 사람이 읽을 메모, 파일 mtime)
fn load_cfg() -> (Coef, ReplOpt, Shape, Vec<String>, Option<SystemTime>) {
    let d = |m: String| (Coef::vanilla(), ReplOpt::off(), Shape::off(), vec![m], None);
    let path = match cfg_path() {
        Some(p) => p,
        None => return d("cfg 경로 확인 실패 → 기본값".into()),
    };
    if !path.exists() {
        // ★게임이 읽는 파일은 아니지만 우리 파서도 BOM 을 싫어하므로 BOM 없이 쓴다.
        let _ = fs::write(&path, CFG_TEMPLATE);
        return (
            Coef::vanilla(),
            ReplOpt::off(),
            Shape::off(),
            vec![format!("{} 없음 → 기본값으로 생성", CFG_NAME)],
            fs::metadata(&path).ok().and_then(|m| m.modified().ok()),
        );
    }
    let text = match fs::read_to_string(&path) {
        Ok(t) => t,
        Err(e) => return d(format!("cfg 읽기 실패({}) → 기본값", e)),
    };
    let (c, r, sh, notes) = parse_cfg(&text);
    let mt = fs::metadata(&path).ok().and_then(|m| m.modified().ok());
    (c, r, sh, notes, mt)
}

// ─────────────────────────────────────────────────────────────────────────────
// 상수 풀
// ─────────────────────────────────────────────────────────────────────────────
static POOL_ADDR: AtomicU64 = AtomicU64::new(0);
static PATCHED_SITES: AtomicU64 = AtomicU64::new(0);
static RELOAD_COUNT: AtomicU64 = AtomicU64::new(0);
static REPL_INSTALLED: AtomicBool = AtomicBool::new(false);

/// `[rip+disp32]` 사거리(±2GB) 안에 상수 풀을 잡는다. movaps 가 16B 정렬을 요구하므로
/// VirtualAlloc 의 64KB 정렬 반환값을 그대로 쓴다.
unsafe fn alloc_pool_near(target: usize) -> usize {
    const MEM_CR: u32 = 0x1000 | 0x2000;
    const RW: u32 = 0x04;
    let base = target & !0xFFFF;
    for step in 1..0x7F00usize {
        for d in [1i64, -1] {
            let addr = (base as i64 + d * (step as i64) * 0x10000) as usize;
            if addr < 0x10000 {
                continue;
            }
            let p = VirtualAlloc(addr, POOL_SIZE.max(0x1000), MEM_CR, RW);
            if p != 0 {
                let delta = (p as i64) - (target as i64);
                if delta > i32::MIN as i64 + 0x10000 && delta < i32::MAX as i64 - 0x10000 {
                    return p;
                }
            }
        }
    }
    0
}

/// 풀에 계수를 써넣는다. **코드 재패치 불필요** — 사이트들이 이미 풀을 가리키고 있으므로
/// 값만 덮어쓰면 다음 계산부터 즉시 반영된다(핫리로드의 근거).
unsafe fn write_pool(pool: usize, c: &Coef) {
    let w = |off: usize, v: f32| {
        core::ptr::write_unaligned((pool + off) as *mut f32, v);
    };
    // decay 는 packed(movaps ×4 레인) + scalar 가 lane0 을 공유한다 → 4레인 모두 같은 값.
    for lane in 0..4 {
        w(lane * 4, c.decay);
    }
    w(slot_off(S_LR), c.lr);
    w(slot_off(S_CLAMP_LO), c.clamp_lo);
    w(slot_off(S_CLAMP_HI), c.clamp_hi);
    w(slot_off(S_TW_A), c.time_w_a);
    w(slot_off(S_TW_B), c.time_w_b);
    w(slot_off(S_LABEL_WIN), c.label_win);
    w(slot_off(S_LABEL_LOSE), c.label_lose);
    w(slot_off(S_NOISE_RANGE), c.noise_range);
    w(slot_off(S_NOISE_OFFSET), c.noise_offset);
}

/// 사이트 하나의 disp32 를 풀 슬롯으로 돌린다.
/// 검증 3단: ①오피코드 프리픽스 일치 ②현재 목표가 읽히는지 ③현재 목표의 f32 값이 게임 기본값인지.
/// 하나라도 어긋나면 **건드리지 않고** 사유를 반환한다(패치로 RVA 가 밀린 상태 = stale).
unsafe fn patch_site(base: usize, pool: usize, s: &Site) -> Result<String, String> {
    let site = base.wrapping_add(s.rva);
    let l = s.len as usize;
    if !readable(site, l) {
        return Err(format!("{}: site unreadable @0x{:x}", s.label, site));
    }
    let mut cur = [0u8; 16];
    core::ptr::copy_nonoverlapping(site as *const u8, cur.as_mut_ptr(), l);
    if &cur[..s.pre.len()] != s.pre {
        return Err(format!(
            "{}: 오피코드 불일치 @0x{:x} cur={:02x?} want={:02x?} (RVA stale?)",
            s.label,
            site,
            &cur[..s.pre.len()],
            s.pre
        ));
    }
    let disp = i32::from_le_bytes([cur[l - 4], cur[l - 3], cur[l - 2], cur[l - 1]]);
    let cur_tgt = (site as i64 + l as i64 + disp as i64) as usize;

    let want_slot = pool + slot_off(s.slot);
    if cur_tgt == want_slot {
        return Ok(format!("{}: 이미 적용됨", s.label)); // 멱등
    }
    // 아직 게임 .rdata 를 가리키는 상태여야 한다. 값까지 대조해 쌍둥이 명령 오식별을 막는다.
    if !readable(cur_tgt, 4) {
        return Err(format!("{}: 현재 목표 0x{:x} 를 읽을 수 없음", s.label, cur_tgt));
    }
    let cur_val = core::ptr::read_unaligned(cur_tgt as *const f32);
    let want_val = expect_orig_value(s.slot);
    if (cur_val - want_val).abs() > 1e-6 * want_val.abs().max(1.0) {
        return Err(format!(
            "{}: 원본 상수값 불일치 @0x{:x} → {} (기대 {}) — RVA stale, 건너뜀",
            s.label, cur_tgt, cur_val, want_val
        ));
    }

    let new_disp = (want_slot as i64) - (site as i64 + l as i64);
    if new_disp < i32::MIN as i64 || new_disp > i32::MAX as i64 {
        return Err(format!("{}: rip-rel 사거리 초과({})", s.label, new_disp));
    }

    const RWX: u32 = 0x40;
    let at = site + l - 4;
    let mut old: u32 = 0;
    if VirtualProtect(at, 4, RWX, &mut old) == 0 {
        return Err(format!("{}: VirtualProtect 실패", s.label));
    }
    core::ptr::copy_nonoverlapping((new_disp as i32).to_le_bytes().as_ptr(), at as *mut u8, 4);
    VirtualProtect(at, 4, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), site, l);

    let back = core::ptr::read_unaligned(at as *const i32);
    if back != new_disp as i32 {
        return Err(format!("{}: write 미반영", s.label));
    }
    Ok(format!("{}: 0x{:x} → pool+0x{:x}", s.label, cur_tgt, slot_off(s.slot)))
}

// ─────────────────────────────────────────────────────────────────────────────
// 방식 B — update 대체 구현 (기본 OFF)
//   ⚠ 미검증. dense 빌더 shadow-CALL 을 쓰므로 cfg 게이트로 격리한다.
// ─────────────────────────────────────────────────────────────────────────────
static REPL_COEF: parking::Cell = parking::Cell::new();
static REPL_CALLS: AtomicU64 = AtomicU64::new(0);
static REPL_STEPS: AtomicU64 = AtomicU64::new(0);
static REPL_FAULTS: AtomicU64 = AtomicU64::new(0);

/// 계수 스냅샷을 lock 없이 읽기 위한 최소 셀. detour 안에서 뮤텍스를 잡으면 poison·데드락
/// 위험이 있어 원자적 스왑만 쓴다(f32 12개 = 48B 라 Box 포인터 하나로 교체한다).
mod parking {
    use super::{Coef, ReplOpt, Shape};
    use std::sync::atomic::{AtomicPtr, Ordering};

    pub struct Snap {
        pub c: Coef,
        pub r: ReplOpt,
        pub s: Shape,
    }
    pub struct Cell(AtomicPtr<Snap>);
    impl Cell {
        pub const fn new() -> Self {
            Cell(AtomicPtr::new(core::ptr::null_mut()))
        }
        /// 이전 스냅샷은 의도적으로 leak 한다 — detour 가 그 포인터를 읽고 있을 수 있어
        /// 해제하면 use-after-free 가 된다(교체는 드물고 100B 미만이라 무시 가능).
        pub fn store(&self, c: Coef, r: ReplOpt, s: Shape) {
            let b = Box::into_raw(Box::new(Snap { c, r, s }));
            self.0.store(b, Ordering::Release);
        }
        pub fn load(&self) -> Option<&'static Snap> {
            let p = self.0.load(Ordering::Acquire);
            if p.is_null() {
                None
            } else {
                Some(unsafe { &*p })
            }
        }
    }
    unsafe impl Sync for Cell {}
}

// ─────────────────────────────────────────────────────────────────────────────
// record 진입 캡처 — `&World` 와 `&MatchResult` 를 훔쳐 둔다
//
// `update` 는 World 를 인자로 받지 않으므로, 바로 위 프레임인 `record` 진입부에서 잡아 둬야 한다.
// record → update 는 **같은 스레드의 중첩 호출**이라 전역 3칸으로 충분하다. 다만 서버 tick 이
// 여러 스레드에서 돌 가능성을 배제할 수 없으므로 **스레드 id 를 함께 저장하고 update 에서 대조**한다.
// 불일치면 지표를 쓰지 않고 quality=1.0 으로 폴백한다(= 게임 원본 동작).
// ─────────────────────────────────────────────────────────────────────────────
static G_WORLD: AtomicU64 = AtomicU64::new(0);
static G_MATCHRESULT: AtomicU64 = AtomicU64::new(0);
static G_TID: AtomicU64 = AtomicU64::new(0);
static RECORD_CALLS: AtomicU64 = AtomicU64::new(0);
static QUALITY_MISS: AtomicU64 = AtomicU64::new(0); // 선수 매칭 실패 = 폴백 횟수

/// 현재 스레드 id. x64 Windows 는 `gs:[0x48]` = TEB.ClientId.UniqueThread.
/// (스텁이 쓰는 것과 같은 소스여야 대조가 성립한다 — GetCurrentThreadId 호출로 대체하지 말 것.)
#[inline(always)]
unsafe fn cur_tid() -> u64 {
    let t: u64;
    core::arch::asm!("mov {}, gs:[0x48]", out(reg) t, options(nostack, preserves_flags));
    t
}

/// record 진입 캡처 스텁을 조립한다.
/// 사이트 12B 를 `call rel32 + nop*7` 로 바꾸므로, 스텁이 **원본 8개 push 를 대신 수행**한 뒤
/// 복귀해야 한다. 순서가 중요하다 — 먼저 복귀주소를 걷어내고, push 들을 쌓고, 복귀주소를 다시 올린다.
/// rax·r11 은 win64 volatile 이고 진입 시점엔 인자가 아니므로 파괴해도 안전하다.
unsafe fn build_record_stub(stub: usize) {
    let mut s: Vec<u8> = Vec::new();
    // 스텁 진입 시 [rsp]=복귀주소 ⟹ 원본 arg6([rsp+0x30])은 [rsp+0x38] 에 있다.
    // mov r11, &G_WORLD ; mov [r11], rdx
    s.extend_from_slice(&[0x49, 0xbb]);
    s.extend_from_slice(&((&G_WORLD as *const AtomicU64) as u64).to_le_bytes());
    s.extend_from_slice(&[0x49, 0x89, 0x13]);
    // mov rax, [rsp+0x38] ; mov r11, &G_MATCHRESULT ; mov [r11], rax
    s.extend_from_slice(&[0x48, 0x8b, 0x44, 0x24, 0x38]);
    s.extend_from_slice(&[0x49, 0xbb]);
    s.extend_from_slice(&((&G_MATCHRESULT as *const AtomicU64) as u64).to_le_bytes());
    s.extend_from_slice(&[0x49, 0x89, 0x03]);
    // mov rax, gs:[0x48] ; mov r11, &G_TID ; mov [r11], rax
    s.extend_from_slice(&[0x65, 0x48, 0x8b, 0x04, 0x25, 0x48, 0x00, 0x00, 0x00]);
    s.extend_from_slice(&[0x49, 0xbb]);
    s.extend_from_slice(&((&G_TID as *const AtomicU64) as u64).to_le_bytes());
    s.extend_from_slice(&[0x49, 0x89, 0x03]);
    // inc qword [&RECORD_CALLS]
    s.extend_from_slice(&[0x49, 0xbb]);
    s.extend_from_slice(&((&RECORD_CALLS as *const AtomicU64) as u64).to_le_bytes());
    s.extend_from_slice(&[0x49, 0xff, 0x03]);
    // pop r11(복귀주소) → 원본 push 8개 → push r11 → ret
    s.push(0x41);
    s.push(0x5b); // pop r11
    s.extend_from_slice(&RECORD_PROLOGUE);
    s.extend_from_slice(&[0x41, 0x53]); // push r11
    s.push(0xc3); // ret

    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    FlushInstructionCache(GetCurrentProcess(), stub, s.len());
}

#[repr(C)]
struct RawVec {
    cap: usize,
    ptr: *mut f32,
    len: usize,
}

type DenseFn = unsafe extern "win64" fn(
    out: *mut RawVec,
    cap_hint: usize,
    mode: u32,
    ctx: *const u8,
    items: *const u64,
    n_items: usize,
);

#[repr(C)]
struct Agent {
    cap: usize,
    w: *mut f32,
    len: usize,
    flag: u32,
    _pad: u32,
}

static DENSE_ADDR: AtomicU64 = AtomicU64::new(0);

// ─────────────────────────────────────────────────────────────────────────────
// 챔피언 id ↔ 이름 수집 (읽기 전용 peek · 게임 동작 무개입)
//
// 왜: 점수 계산의 모든 피처가 `me`(챔피언 id 정수)를 첫 인자로 쓴다. 바닐라 챔프는 시트 순서로
//   알 수 있지만 **모드(워크샵) 챔피언은 id 를 알 방법이 없어** 오프라인 채점에서 통째로 빠진다.
//   그런데 `update` 가 받는 ctx 에 그 id 가 들어 있고, 같은 시점에 `record` 가 캡처해 둔 World 에서
//   그 선수의 champion_name 을 읽을 수 있다 ⟹ **둘을 짝지으면 이름 → id 매핑이 나온다.**
//
// ★부수 효과 = 바닐라 챔프도 함께 찍히므로, 그 id 들이 알려진 시트 순서와 맞는지 대조하면
//   "이 매핑이 세이브 무관한 설치 고정 속성인가"를 데이터가 스스로 검증해준다.
//
// 방식 = `update` 진입부 12B 를 `call rel32 + nop*7` 로 가로채 인자만 엿보고 원본 프롤로그를
//   대신 수행한 뒤 복귀한다. **원본 로직에 일절 개입하지 않는다**(replace_update 와 무관).
//   ⚠ update 는 인자 4개가 레지스터로 오므로 스텁이 rcx/rdx/r8/r9 를 반드시 보존해야 한다.
// ─────────────────────────────────────────────────────────────────────────────
static CHAMP_MAP: Mutex<Option<HashMap<String, u64>>> = Mutex::new(None);
static CHAMP_SEEN: AtomicU64 = AtomicU64::new(0);
static CHAMP_PROBE_ON: AtomicBool = AtomicBool::new(false);

/// update 진입에서 (ctx, won) 만 엿본다. 게임 상태를 **읽기만** 한다.
unsafe extern "win64" fn champ_probe(ctx: *const u64, won: u8) {
    let _ = catch_unwind(AssertUnwindSafe(|| champ_probe_inner(ctx, won)));
}

unsafe fn champ_probe_inner(ctx: *const u64, won: u8) {
    // record 캡처가 같은 스레드의 것이어야 World 가 이 경기의 것이다.
    if G_TID.load(Ordering::Relaxed) != cur_tid() {
        return;
    }
    let world = G_WORLD.load(Ordering::Relaxed) as usize;
    let mr = G_MATCHRESULT.load(Ordering::Relaxed) as usize;
    if world == 0 || mr == 0 {
        return;
    }
    if !readable(ctx as usize, 0x58) || !readable(mr + MR_WON + MR_TEAM_STRIDE, 1) {
        return;
    }
    let lane = rd_u64(ctx as usize + CTX_LANE);
    if lane > 4 {
        return;
    }
    let cid = rd_u64(ctx as usize + (lane as usize) * 8); // ctx[lane] = 본인 챔프 id

    // 팀 = MatchResult 의 팀별 won 과 인자 won 을 대조(무승부면 구별 불가 → 포기)
    let won0 = core::ptr::read_unaligned((mr + MR_WON) as *const u8);
    let won1 = core::ptr::read_unaligned((mr + MR_WON + MR_TEAM_STRIDE) as *const u8);
    if won0 == won1 {
        return;
    }
    let team: u64 = if won == won0 { 0 } else { 1 };

    if !readable(world + W_PLAYERS_PTR, 16) {
        return;
    }
    let pptr = rd_u64(world + W_PLAYERS_PTR) as usize;
    let plen = rd_u64(world + W_PLAYERS_LEN) as usize;
    if pptr == 0 || plen == 0 || plen > 64 || !readable(pptr, plen * PLAYER_STRIDE) {
        return;
    }
    // (team, position) 은 10명 중 유일하다
    let mut me = usize::MAX;
    for i in 0..plen {
        let p = pptr + i * PLAYER_STRIDE;
        if rd_u64(p + P_TEAM) == team && rd_u64(p + P_POSITION) == lane {
            me = i;
            break;
        }
    }
    if me == usize::MAX {
        return;
    }
    // champion_name: String { cap@+0x418, ptr@+0x420, len@+0x428 }
    let p = pptr + me * PLAYER_STRIDE;
    let nptr = rd_u64(p + P_CHAMP_NAME_PTR) as usize;
    let nlen = rd_u64(p + P_CHAMP_NAME_LEN) as usize;
    if nptr < 0x10000 || nlen == 0 || nlen > 64 || !readable(nptr, nlen) {
        return;
    }
    let mut buf = vec![0u8; nlen];
    core::ptr::copy_nonoverlapping(nptr as *const u8, buf.as_mut_ptr(), nlen);
    let name = match String::from_utf8(buf) {
        Ok(s) => s,
        Err(_) => return,
    };

    let mut g = CHAMP_MAP.lock().unwrap_or_else(|e| e.into_inner());
    let m = g.get_or_insert_with(HashMap::new);
    // ⚠같은 이름에 다른 id 가 관측되면 매핑이 커리어마다 다르다는 뜻 = 소급 적용 불가 신호.
    //   그 사실을 놓치지 않도록 충돌을 별도 키로 남긴다.
    match m.get(&name) {
        Some(&old) if old != cid => {
            // ⚠여기서도 카운터를 올려야 진단 스레드가 파일을 다시 쓴다.
            //   (안 올리면 "충돌 없음"과 "충돌났지만 기록 안 됨"이 구별되지 않는다 — 실제로 겪은 결함)
            let key = format!("{}#CONFLICT#{}", name, cid);
            if !m.contains_key(&key) {
                m.insert(key, cid);
                CHAMP_SEEN.fetch_add(1, Ordering::Relaxed);
            }
        }
        Some(_) => {}
        None => {
            m.insert(name, cid);
            CHAMP_SEEN.fetch_add(1, Ordering::Relaxed);
        }
    }
}

/// 기존 `champ_ids.txt` 를 맵에 미리 적재한다.
/// ★이게 없으면 게임을 재시작할 때마다 맵이 비어서, **다른 커리어의 매핑과 대조가 불가능**하다
///   (이전 세션 값이 사라지므로 충돌이 영원히 안 잡힌다). 세션을 넘겨 검증하려면 필수.
fn preload_champ_ids() -> usize {
    let path = match dir() {
        Some(mut p) => {
            p.push("champ_ids.txt");
            p
        }
        None => return 0,
    };
    let text = match fs::read_to_string(&path) {
        Ok(t) => t,
        Err(_) => return 0,
    };
    let mut g = CHAMP_MAP.lock().unwrap_or_else(|e| e.into_inner());
    let m = g.get_or_insert_with(HashMap::new);
    let mut n = 0;
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut it = line.splitn(2, '\t');
        let (id, name) = match (it.next(), it.next()) {
            (Some(a), Some(b)) => (a, b),
            _ => continue,
        };
        // 지난 충돌 기록은 그대로 보존(재검증 대상이 아니라 이력이다)
        if let Ok(v) = id.parse::<u64>() {
            m.insert(name.to_string(), v);
            n += 1;
        }
    }
    CHAMP_SEEN.store(n as u64, Ordering::Relaxed);
    n
}

/// champ_probe 스텁. 인자 레지스터(rcx/rdx/r8/r9)를 보존하고 win64 호출 규약을 갖춘다.
/// 스택 정렬: 스텁 진입 rsp ≡ 0 (mod 16) → push 4개(32) → sub 0x20(shadow) → call 시점 ≡ 0.
unsafe fn build_champ_probe_stub(stub: usize) {
    let mut s: Vec<u8> = Vec::new();
    s.push(0x51); // push rcx
    s.push(0x52); // push rdx
    s.extend_from_slice(&[0x41, 0x50]); // push r8
    s.extend_from_slice(&[0x41, 0x51]); // push r9
    s.extend_from_slice(&[0x48, 0x83, 0xec, 0x20]); // sub rsp,0x20  (shadow space)
    // arg2 = won : 스텁 진입 [rsp+0x48] → 지금은 +0x40 만큼 밀렸다
    s.extend_from_slice(&[0x0f, 0xb6, 0x94, 0x24, 0x88, 0x00, 0x00, 0x00]); // movzx edx,[rsp+0x88]
    // arg1 = ctx : 저장해 둔 rdx
    s.extend_from_slice(&[0x48, 0x8b, 0x4c, 0x24, 0x30]); // mov rcx,[rsp+0x30]
    s.extend_from_slice(&[0x48, 0xb8]);
    s.extend_from_slice(&(champ_probe as usize as u64).to_le_bytes()); // movabs rax, fn
    s.extend_from_slice(&[0xff, 0xd0]); // call rax
    s.extend_from_slice(&[0x48, 0x83, 0xc4, 0x20]); // add rsp,0x20
    s.extend_from_slice(&[0x41, 0x59]); // pop r9
    s.extend_from_slice(&[0x41, 0x58]); // pop r8
    s.push(0x5a); // pop rdx
    s.push(0x59); // pop rcx
    // 복귀주소를 먼저 걷어내고 원본 프롤로그를 쌓은 뒤 다시 올린다(순서 중요)
    s.extend_from_slice(&[0x41, 0x5b]); // pop r11
    s.extend_from_slice(&UPDATE_PROLOGUE);
    s.extend_from_slice(&[0x41, 0x53]); // push r11
    s.push(0xc3); // ret
    core::ptr::copy_nonoverlapping(s.as_ptr(), stub as *mut u8, s.len());
    FlushInstructionCache(GetCurrentProcess(), stub, s.len());
}

unsafe fn install_champ_probe(base: usize) -> Result<String, String> {
    let site = base.wrapping_add(RVA_UPDATE);
    if !readable(site, 12) {
        return Err(format!("update site unreadable @0x{:x}", site));
    }
    let mut cur = [0u8; 12];
    core::ptr::copy_nonoverlapping(site as *const u8, cur.as_mut_ptr(), 12);
    if cur[0] == 0xe8 {
        return Ok("이미 설치됨".into());
    }
    if cur != UPDATE_PROLOGUE {
        return Err(format!("update 프롤로그 불일치 cur={:02x?} (RVA stale?)", cur));
    }
    let stub = alloc_exec_near(site);
    if stub == 0 {
        return Err("스텁 할당 실패".into());
    }
    build_champ_probe_stub(stub);
    let rel = (stub as i64) - (site as i64 + 5);
    if rel < i32::MIN as i64 || rel > i32::MAX as i64 {
        return Err("rel32 범위 초과".into());
    }
    let mut patch = [0x90u8; 12];
    patch[0] = 0xe8;
    patch[1..5].copy_from_slice(&(rel as i32).to_le_bytes());
    const RWX: u32 = 0x40;
    let mut old: u32 = 0;
    if VirtualProtect(site, 12, RWX, &mut old) == 0 {
        return Err("VirtualProtect 실패".into());
    }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), site as *mut u8, 12);
    VirtualProtect(site, 12, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), site, 12);
    CHAMP_PROBE_ON.store(true, Ordering::Relaxed);
    Ok(format!("update @0x{:x} → probe stub 0x{:x}", site, stub))
}

/// 수집된 매핑을 파일로. 진단 스레드(메인 아님)가 부른다 — detour 안에서 IO 금지.
fn flush_champ_ids() {
    let g = CHAMP_MAP.lock().unwrap_or_else(|e| e.into_inner());
    let m = match g.as_ref() {
        Some(m) if !m.is_empty() => m,
        _ => return,
    };
    let mut v: Vec<(&String, &u64)> = m.iter().collect();
    v.sort_by_key(|(_, &id)| id);
    let mut s = String::from(
        "# 챔피언 id ↔ 이름 (update 의 ctx 와 World 의 champion_name 을 짝지어 실측)\n\
         # id 는 게임 챔피언 레지스트리의 순회 인덱스다. 모드 구성이 같으면 세이브와 무관하게 동일.\n\
         # ⚠ '#CONFLICT#' 줄이 있으면 같은 이름에 다른 id 가 관측된 것 = 커리어마다 다르다는 뜻이므로\n\
         #    다른 세이브에 소급 적용하면 안 된다.\n\
         # id\tname\n",
    );
    for (name, id) in v {
        s.push_str(&format!("{}\t{}\n", id, name));
    }
    if let Some(mut p) = dir() {
        p.push("champ_ids.txt");
        let _ = fs::write(p, s);
    }
}

/// 경기 지표에서 뽑아낸 이 표본의 (가중치, 라벨).
struct Metrics {
    quality: f32,
    y: f32,
}

#[inline]
unsafe fn rd_u64(p: usize) -> u64 {
    core::ptr::read_unaligned(p as *const u64)
}

/// `record` 가 캡처해 둔 `&World` / `&MatchResult` 로부터 이 선수의 표본 가중치와 라벨을 계산한다.
/// 어느 단계든 확신이 없으면 **None** 을 반환하고, 호출측은 게임 원본 동작(quality=1, y=고정라벨)으로
/// 폴백한다 — 잘못된 지표로 학습을 오염시키는 것보다 안 쓰는 게 낫다.
unsafe fn compute_metrics(ctx: *const u8, won: u8, c: &Coef, sh: &Shape) -> Option<Metrics> {
    // ① 캡처가 같은 스레드의 것인지
    if G_TID.load(Ordering::Relaxed) != cur_tid() {
        return None;
    }
    let world = G_WORLD.load(Ordering::Relaxed) as usize;
    let mr = G_MATCHRESULT.load(Ordering::Relaxed) as usize;
    if world == 0 || mr == 0 {
        return None;
    }
    if !readable(world + W_SCORES, 16) || !readable(mr + MR_WON + MR_TEAM_STRIDE, 1) {
        return None;
    }

    // ② 이 update 호출이 어느 팀 선수의 것인가 — MatchResult 의 팀별 won 과 인자 won 을 대조.
    //    무승부 등으로 양 팀 won 이 같으면 구별 불가 ⟹ 폴백.
    let won0 = core::ptr::read_unaligned((mr + MR_WON) as *const u8);
    let won1 = core::ptr::read_unaligned((mr + MR_WON + MR_TEAM_STRIDE) as *const u8);
    if won0 == won1 {
        return None;
    }
    let team: u64 = if won == won0 { 0 } else { 1 };

    // ③ 레인(포지션) — ctx+0x50. 팀+포지션이면 10명 중 유일하다.
    if !readable(ctx as usize + CTX_LANE, 8) {
        return None;
    }
    let lane = rd_u64(ctx as usize + CTX_LANE);
    if lane > 4 {
        return None;
    }

    // ④ 선수 배열
    if !readable(world + W_PLAYERS_PTR, 16) {
        return None;
    }
    let pptr = rd_u64(world + W_PLAYERS_PTR) as usize;
    let plen = rd_u64(world + W_PLAYERS_LEN) as usize;
    if pptr == 0 || plen == 0 || plen > 64 || !readable(pptr, plen * PLAYER_STRIDE) {
        return None;
    }

    // ⑤ 본인 찾기 + 같은 팀 합계 + 팀 내 최고 rating(=MVP) 를 한 번에 순회
    let mut me: usize = usize::MAX;
    let (mut t_kill, mut t_death, mut t_deal, mut t_tank) = (0u64, 0u64, 0u64, 0u64);
    let mut best_rating = 0u64;
    let mut best_idx = usize::MAX;
    for i in 0..plen {
        let p = pptr + i * PLAYER_STRIDE;
        if rd_u64(p + P_TEAM) != team {
            continue;
        }
        t_kill += rd_u64(p + P_KILL);
        t_death += rd_u64(p + P_DEATH);
        t_deal += rd_u64(p + P_DEAL);
        t_tank += rd_u64(p + P_TANK);
        let r = rd_u64(p + P_RATING);
        // 게임의 MVP 선정과 동일: argmax(rating), 동률이면 뒤쪽(index 큰 쪽) 우선
        if r >= best_rating {
            best_rating = r;
            best_idx = i;
        }
        if rd_u64(p + P_POSITION) == lane {
            me = i;
        }
    }
    if me == usize::MAX {
        return None;
    }
    let p = pptr + me * PLAYER_STRIDE;

    // ── 라벨 y ──
    let y = if !sh.grade_active() {
        if won != 0 { c.label_win } else { c.label_lose }
    } else {
        let s0 = rd_u64(world + W_SCORES) as f32;
        let s1 = rd_u64(world + W_SCORES + 8) as f32;
        let total = (s0 + s1).max(1.0);
        let (mine, theirs) = if team == 0 { (s0, s1) } else { (s1, s0) };
        let margin = ((mine - theirs) / total).abs(); // 0~1
        let sec = rd_u64(world + W_TICK) as f32 / TICKS_PER_SEC;
        let speed = (1.0 - sec / sh.y_ref_sec.max(1.0)).clamp(0.0, 1.0);
        // base = 바닐라 라벨과의 연속성 (label_win 0.9 → base 0.4 → grade 0 이면 정확히 0.9/0.1)
        let base = c.label_win - 0.5;
        let amp = (base + sh.y_margin_w * margin + sh.y_speed_w * speed).clamp(0.0, 0.5);
        if won != 0 { 0.5 + amp } else { 0.5 - amp }
    };

    // ── 표본 가중치 quality ──
    let quality = if !sh.quality_active() {
        1.0
    } else {
        const EVEN: f32 = 0.2; // 5인 팀에서의 평균 지분
        let share = |v: u64, tot: u64| -> f32 {
            if tot == 0 { EVEN } else { v as f32 / tot as f32 }
        };
        let kill = rd_u64(p + P_KILL);
        let death = rd_u64(p + P_DEATH);
        let deal = rd_u64(p + P_DEAL);
        let tank = rd_u64(p + P_TANK);

        // 탱킹 효율 = (내 데스당 탱킹) ÷ (팀 평균 데스당 탱킹). 1.0 = 팀 평균.
        // ★데스 감점을 이 값으로 나눈다 = "많이 죽었어도 그만큼 받아냈으면 감점을 깎아준다".
        //   데스 0이면 분모를 1로 보아 효율이 커지고, clamp 상한이 폭주를 막는다.
        let mine_tpd = tank as f32 / (death.max(1)) as f32;
        let team_tpd = t_tank as f32 / (t_death.max(1)) as f32;
        let tank_eff = if team_tpd > 0.0 {
            (mine_tpd / team_tpd).clamp(sh.tank_eff_min, sh.tank_eff_max)
        } else {
            1.0
        };

        let is_mvp = if me == best_idx { 1.0f32 } else { 0.0 };
        let q = 1.0
            + sh.w_mvp * is_mvp
            + sh.w_kill * (share(kill, t_kill) - EVEN)
            + sh.w_deal * (share(deal, t_deal) - EVEN)
            + sh.w_tank * (share(tank, t_tank) - EVEN)
            - sh.w_death * (share(death, t_death) - EVEN) / tank_eff.max(1e-3);
        q.clamp(sh.quality_min, sh.quality_max)
    };

    Some(Metrics { quality, y })
}

/// update 대체 구현.
/// 계약(0x105c060 콜사이트 실측):
///   rcx=agent, rdx=ctx, r8=items, r9=n_items,
///   [rsp+0x28]=buy_times, [rsp+0x30]=n_times, [rsp+0x38]=duration, [rsp+0x40]=won
/// ★게임 원본과 동일하게 `n_items != n_times || n_items == 0` 이면 즉시 반환한다.
unsafe extern "win64" fn repl_update(
    agent: *mut Agent,
    ctx: *const u8,
    items: *const u64,
    n_items: usize,
    buy_times: *const u64,
    n_times: usize,
    duration: u64,
    won: u8,
) {
    // detour 패닉은 게임 콜스택 unwind = UB. 반드시 잡는다.
    let r = catch_unwind(AssertUnwindSafe(|| {
        repl_update_inner(agent, ctx, items, n_items, buy_times, n_times, duration, won)
    }));
    if r.is_err() {
        REPL_FAULTS.fetch_add(1, Ordering::Relaxed);
    }
}

unsafe fn repl_update_inner(
    agent: *mut Agent,
    ctx: *const u8,
    items: *const u64,
    n_items: usize,
    buy_times: *const u64,
    n_times: usize,
    duration: u64,
    won: u8,
) {
    if n_items != n_times || n_items == 0 {
        return;
    }
    let snap = match REPL_COEF.load() {
        Some(s) => s,
        None => return,
    };
    let (c, opt, sh) = (snap.c, snap.r, snap.s);

    if !readable(agent as usize, core::mem::size_of::<Agent>()) {
        return;
    }
    let a = &*agent;
    let n = a.len;
    if n == 0 || n > 1 << 22 || !readable(a.w as usize, n * 4) {
        return;
    }
    let dense_addr = DENSE_ADDR.load(Ordering::Relaxed);
    if dense_addr == 0 {
        return;
    }
    let dense: DenseFn = core::mem::transmute(dense_addr as usize);

    REPL_CALLS.fetch_add(1, Ordering::Relaxed);

    // 경기 지표는 이 update 호출 전체에 대해 한 번만 구한다(prefix 마다 변하지 않는다).
    // 실패하면 게임 원본 동작으로 폴백 — 잘못된 지표로 학습을 오염시키지 않는다.
    let (quality, y) = match compute_metrics(ctx, won, &c, &sh) {
        Some(m) => (m.quality, m.y),
        None => {
            if sh.quality_active() || sh.grade_active() {
                QUALITY_MISS.fetch_add(1, Ordering::Relaxed);
            }
            (1.0, if won != 0 { c.label_win } else { c.label_lose })
        }
    };
    // flag≠0 = prefix 커리큘럼(1개빌드 → 2개빌드 → … → 최종빌드).
    // flag==0 = 게임도 커리큘럼을 돌지 않고 전체 빌드 1샘플만 학습한다.
    // detour 안에서는 힙 할당을 피하려고 Box<dyn Iterator> 대신 시작 인덱스만 바꾼다.
    let first = if a.flag != 0 { 0 } else { n_items - 1 };

    for i in first..n_items {
        let mut out = RawVec { cap: 0, ptr: core::ptr::null_mut(), len: 0 };
        // 게임 dense 빌더 shadow-CALL. items[0..=i] 접두사를 넘긴다.
        dense(&mut out, n, 0, ctx, items, i + 1);
        if out.ptr.is_null() || out.len == 0 || !readable(out.ptr as usize, out.len * 4) {
            free_rawvec(&out);
            continue;
        }
        let m = out.len.min(n);
        let x = core::slice::from_raw_parts(out.ptr, m);
        let w = core::slice::from_raw_parts_mut(a.w, n);

        // z = Σ x[j]*w[j]   (게임 update 는 bias 를 안 더한다 — 옵션으로 교정 가능)
        let mut z: f32 = if opt.bias_in_update { w[0] } else { 0.0 };
        for j in 0..m {
            if x[j] != 0.0 {
                z += x[j] * w[j];
            }
        }
        let p = 1.0f32 / (1.0 + (-z).exp());

        // own = min((T - buy_time) / T, 1.0), buy_time > T 면 0
        let t = duration as f32;
        let own = if t > 0.0 && readable(buy_times as usize, (i + 1) * 8) {
            let bt = *buy_times.add(i) as f32;
            if bt > t {
                0.0
            } else {
                ((t - bt) / t).min(1.0)
            }
        } else {
            1.0
        };

        // e = 시간가중 × 표본가중(경기 지표) × (p − y)
        // ★quality 는 (p−y) 에 **곱해지는 가중치**라 부호는 여전히 승패가 정한다.
        //   딜·탱이 아이템에 의존해도 "졌는데 딜 많이 넣음"이 양의 학습으로 뒤집히지는 않는다.
        let e = ((own * c.time_w_a + c.time_w_b) * quality * (p - y)).clamp(c.clamp_lo, c.clamp_hi);
        let g = e * c.lr;

        if opt.decay_touched_only {
            // 이번 step 에 등장한 피처만 감쇠 → 희소한 모드 아이템이 0으로 흘러내리지 않는다.
            for j in 0..m {
                if x[j] != 0.0 {
                    w[j] = w[j] * c.decay - x[j] * g;
                }
            }
        } else {
            // 게임 원본과 동일: 전체 벡터 감쇠 + 등장분만 gradient
            for j in 0..n {
                w[j] *= c.decay;
            }
            for j in 0..m {
                if x[j] != 0.0 {
                    w[j] -= x[j] * g;
                }
            }
        }
        REPL_STEPS.fetch_add(1, Ordering::Relaxed);
        free_rawvec(&out);
    }
}

/// dense 빌더가 준 Vec<f32> 해제. 0.5.3 은 `__rust_dealloc` 이 인라인화로 사라져
/// `HeapFree(GetProcessHeap(), 0, ptr)` 가 정본이다(DONE.md, 0.5.3 07-30).
/// cap==0 이면 미할당이므로 건너뛴다.
unsafe fn free_rawvec(v: &RawVec) {
    if v.cap != 0 && !v.ptr.is_null() {
        HeapFree(GetProcessHeap(), 0, v.ptr as usize);
    }
}

/// `call rel32` 사거리 안에 **실행 가능한** 스텁을 잡는다(상수 풀과 달리 RWX 가 필요하다).
unsafe fn alloc_exec_near(target: usize) -> usize {
    const MEM_CR: u32 = 0x1000 | 0x2000;
    const RWX: u32 = 0x40;
    let base = target & !0xFFFF;
    for step in 1..0x7F00usize {
        for d in [1i64, -1] {
            let addr = (base as i64 + d * (step as i64) * 0x10000) as usize;
            if addr < 0x10000 {
                continue;
            }
            let p = VirtualAlloc(addr, 0x1000, MEM_CR, RWX);
            if p != 0 {
                let delta = (p as i64) - (target as i64);
                if delta > i32::MIN as i64 + 0x10000 && delta < i32::MAX as i64 - 0x10000 {
                    return p;
                }
            }
        }
    }
    0
}

/// `record` 진입부 12B 를 `call rel32 + nop*7` 로 덮어 `&World`/`&MatchResult` 를 캡처한다.
/// 원본을 대체하는 게 아니라 **엿보기만** 하므로 스텁이 원본 push 8개를 대신 수행하고 복귀한다.
unsafe fn install_record_hook(base: usize) -> Result<String, String> {
    let site = base.wrapping_add(RVA_RECORD);
    if !readable(site, 12) {
        return Err(format!("record site unreadable @0x{:x}", site));
    }
    let mut cur = [0u8; 12];
    core::ptr::copy_nonoverlapping(site as *const u8, cur.as_mut_ptr(), 12);
    if cur[0] == 0xe8 {
        return Ok("이미 후킹됨".into()); // 멱등
    }
    if cur != RECORD_PROLOGUE {
        return Err(format!(
            "record 프롤로그 불일치 @0x{:x} cur={:02x?} (RVA stale?)",
            site, cur
        ));
    }
    let stub = alloc_exec_near(site);
    if stub == 0 {
        return Err("스텁 할당 실패(call rel32 사거리 내 여유 없음)".into());
    }
    build_record_stub(stub);

    let rel = (stub as i64) - (site as i64 + 5);
    if rel < i32::MIN as i64 || rel > i32::MAX as i64 {
        return Err("rel32 범위 초과".into());
    }
    let mut patch = [0x90u8; 12];
    patch[0] = 0xe8;
    patch[1..5].copy_from_slice(&(rel as i32).to_le_bytes());

    const RWX: u32 = 0x40;
    let mut old: u32 = 0;
    if VirtualProtect(site, 12, RWX, &mut old) == 0 {
        return Err("VirtualProtect 실패".into());
    }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), site as *mut u8, 12);
    VirtualProtect(site, 12, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), site, 12);
    Ok(format!("record @0x{:x} → stub 0x{:x}", site, stub))
}

/// update 진입부 12B 를 `movabs rax, repl; jmp rax` 로 덮는다.
/// 원본을 호출하지 않는 **전체 대체**라 트램폴린·orig 보존이 필요 없다.
unsafe fn install_replacement(base: usize) -> Result<String, String> {
    let site = base.wrapping_add(RVA_UPDATE);
    if !readable(site, 12) {
        return Err(format!("update site unreadable @0x{:x}", site));
    }
    let mut cur = [0u8; 12];
    core::ptr::copy_nonoverlapping(site as *const u8, cur.as_mut_ptr(), 12);
    if cur[0] == 0x48 && cur[1] == 0xb8 {
        return Ok("이미 후킹됨(다른 모드 또는 재진입)".into()); // 자기 체인 방지
    }
    if cur != UPDATE_PROLOGUE {
        return Err(format!(
            "update 프롤로그 불일치 @0x{:x} cur={:02x?} (RVA stale?)",
            site, cur
        ));
    }
    let dense = base.wrapping_add(RVA_DENSE);
    if !readable(dense, 16) {
        return Err("dense 빌더 주소를 읽을 수 없음".into());
    }
    DENSE_ADDR.store(dense as u64, Ordering::Relaxed);

    let tgt = repl_update as usize as u64;
    let mut patch = [0u8; 12];
    patch[0] = 0x48;
    patch[1] = 0xb8;
    patch[2..10].copy_from_slice(&tgt.to_le_bytes());
    patch[10] = 0xff;
    patch[11] = 0xe0;

    const RWX: u32 = 0x40;
    let mut old: u32 = 0;
    if VirtualProtect(site, 12, RWX, &mut old) == 0 {
        return Err("VirtualProtect 실패".into());
    }
    core::ptr::copy_nonoverlapping(patch.as_ptr(), site as *mut u8, 12);
    VirtualProtect(site, 12, old, &mut old);
    FlushInstructionCache(GetCurrentProcess(), site, 12);
    REPL_INSTALLED.store(true, Ordering::Relaxed);
    Ok(format!("update @0x{:x} → repl 0x{:x} (dense=0x{:x})", site, tgt, dense))
}

// ─────────────────────────────────────────────────────────────────────────────
// init
// ─────────────────────────────────────────────────────────────────────────────
fn init(_ctx: &GameCtx) -> ModRegistration {
    let (coef, repl, shape, notes, mtime) = load_cfg();

    log(&format!(
        "\n[{}ms] === {} INIT (0.5.3 buildid 24451609) ===\n",
        now_ms(),
        MOD_ID
    ));
    for n in &notes {
        log(&format!("[cfg] {}\n", n));
    }
    log(&format!(
        "[cfg] decay={} lr={} clamp=[{}, {}] time_w={}·own+{} label={}/{} noise={}·U+{}\n",
        coef.decay,
        coef.lr,
        coef.clamp_lo,
        coef.clamp_hi,
        coef.time_w_a,
        coef.time_w_b,
        coef.label_win,
        coef.label_lose,
        coef.noise_range,
        coef.noise_offset
    ));
    // 반감기 환산 — 계수를 얼마나 세게 건드렸는지 감을 주는 게 목적이다.
    if coef.decay > 0.0 && coef.decay < 1.0 {
        let half = (0.5f64).ln() / (coef.decay as f64).ln();
        log(&format!(
            "[cfg] 망각 반감기 ≈ {:.0} step ≈ {:.0} 경기 (경기당 45 step 가정)\n",
            half,
            half / 45.0
        ));
    } else if coef.decay >= 1.0 {
        log("[cfg] decay=1.0 → 망각 없음(옛 학습이 영구 누적)\n");
    }

    let base = unsafe { GetModuleHandleW(core::ptr::null()) };
    if base == 0 {
        log("[fatal] GetModuleHandleW 실패 — 아무것도 하지 않음\n");
        return ModRegistration::new(MOD_ID);
    }

    unsafe {
        // 첫 사이트 근처에 풀을 잡는다(모든 사이트가 0x1058xxx~0x105cxxx 로 32KB 안에 몰려 있다).
        let anchor = base.wrapping_add(SITES[0].rva);
        let pool = alloc_pool_near(anchor);
        if pool == 0 {
            log("[fatal] 상수 풀 할당 실패(rip-rel 사거리 내 여유 없음) — 패치 생략\n");
            return ModRegistration::new(MOD_ID);
        }
        POOL_ADDR.store(pool as u64, Ordering::Relaxed);
        write_pool(pool, &coef);
        log(&format!("[pool] 0x{:x} ({}B)\n", pool, POOL_SIZE));

        let mut ok = 0u64;
        for s in SITES {
            match patch_site(base, pool, s) {
                Ok(m) => {
                    ok += 1;
                    log(&format!("[patch] {}\n", m));
                }
                Err(e) => log(&format!("[patch] 실패 {}\n", e)),
            }
        }
        PATCHED_SITES.store(ok, Ordering::Relaxed);
        log(&format!("[patch] {}/{} 적용\n", ok, SITES.len()));

        // ── 방식 B (기본 OFF) ──
        REPL_COEF.store(coef, repl, shape);
        if repl.enabled {
            log("[repl] ⚠ replace_update=1 — 미검증 경로입니다(shadow-CALL 사용)\n");
            // 지표를 쓸 때만 record 를 후킹한다. 쓰지 않으면 건드리지 않는 게 안전하다.
            if shape.quality_active() || shape.grade_active() {
                log(&format!(
                    "[shape] quality: mvp={} kill={} deal={} tank={} death={} (eff {}~{}, clamp {}~{})\n",
                    shape.w_mvp, shape.w_kill, shape.w_deal, shape.w_tank, shape.w_death,
                    shape.tank_eff_min, shape.tank_eff_max, shape.quality_min, shape.quality_max
                ));
                log(&format!(
                    "[shape] label: margin_w={} speed_w={} ref={}초\n",
                    shape.y_margin_w, shape.y_speed_w, shape.y_ref_sec
                ));
                match install_record_hook(base) {
                    Ok(m) => log(&format!("[shape] {}\n", m)),
                    Err(e) => log(&format!("[shape] record 훅 실패: {} — 지표 미사용으로 폴백\n", e)),
                }
            } else {
                log("[shape] 경기 지표 가중치 전부 0 → record 훅 생략(게임 원본과 동일한 학습)\n");
            }
            match install_replacement(base) {
                Ok(m) => log(&format!("[repl] {}\n", m)),
                Err(e) => log(&format!("[repl] 실패: {}\n", e)),
            }
        } else {
            log("[repl] replace_update=0 (대체 구현 비활성 — 바이트패치만 사용)\n");
            // ★champ_probe 는 update 진입부를 쓰므로 replace_update 와 배타적이다.
            //   읽기 전용 peek 이라 게임 동작·학습에 개입하지 않는다.
            if repl.champ_probe {
                // 이전 세션 매핑을 적재해야 커리어를 넘나드는 대조가 가능하다
                let pre = preload_champ_ids();
                if pre > 0 {
                    log(&format!("[champ] 기존 매핑 {}종 적재(다른 커리어와 대조 가능)\n", pre));
                }
                match install_champ_probe(base) {
                    Ok(m) => log(&format!("[champ] {}\n", m)),
                    Err(e) => log(&format!("[champ] 실패: {}\n", e)),
                }
                // record 훅이 있어야 World 를 잡을 수 있다(지표를 안 쓸 때도 필요)
                if G_TID.load(Ordering::Relaxed) == 0 && !(shape.quality_active() || shape.grade_active()) {
                    match install_record_hook(base) {
                        Ok(m) => log(&format!("[champ] record 캡처 {}\n", m)),
                        Err(e) => log(&format!("[champ] record 훅 실패: {} — 매핑 수집 불가\n", e)),
                    }
                }
            } else {
                log("[champ] champ_probe=0 (챔피언 id 수집 안 함)\n");
            }
        }
    }

    // cfg 핫리로드 + 진단. 풀 값만 덮어쓰면 되므로 코드 재패치가 필요 없다.
    // ★replace_update 토글은 재시작이 필요하다(코드 패치라 되돌리기가 위험하다).
    std::thread::spawn(move || {
        let mut last_mtime = mtime;
        let mut last_coef = coef;
        let mut last_report = (0u64, 0u64, 0u64);
        let mut last_champ = 0u64;
        loop {
            std::thread::sleep(std::time::Duration::from_secs(3));

            let cur_mtime = cfg_path()
                .and_then(|p| fs::metadata(p).ok())
                .and_then(|m| m.modified().ok());
            if cur_mtime != last_mtime {
                last_mtime = cur_mtime;
                let (c, r, sh2, notes, _) = load_cfg();
                if c != last_coef {
                    let pool = POOL_ADDR.load(Ordering::Relaxed) as usize;
                    if pool != 0 {
                        unsafe { write_pool(pool, &c) };
                        last_coef = c;
                        let n = RELOAD_COUNT.fetch_add(1, Ordering::Relaxed) + 1;
                        log(&format!(
                            "[{}ms] [reload #{}] decay={} lr={} clamp=[{}, {}] time_w={}·own+{} label={}/{} noise={}·U+{}\n",
                            now_ms(), n, c.decay, c.lr, c.clamp_lo, c.clamp_hi,
                            c.time_w_a, c.time_w_b, c.label_win, c.label_lose,
                            c.noise_range, c.noise_offset
                        ));
                        for m in &notes {
                            log(&format!("[reload] {}\n", m));
                        }
                    }
                }
                // ★지표 가중치는 코드 패치가 아니라 스냅샷이라 **핫리로드된다**
                //   (단 record 훅이 이미 설치돼 있어야 한다 = 최초 기동 시 하나라도 0이 아니어야 함).
                REPL_COEF.store(c, r, sh2);
                if r.enabled != REPL_INSTALLED.load(Ordering::Relaxed) {
                    log("[reload] ⚠ replace_update 변경은 게임 재시작 후 적용됩니다\n");
                }
                if (sh2.quality_active() || sh2.grade_active())
                    && G_TID.load(Ordering::Relaxed) == 0
                    && RECORD_CALLS.load(Ordering::Relaxed) == 0
                    && REPL_INSTALLED.load(Ordering::Relaxed)
                {
                    log("[reload] ⚠ 지표 가중치를 켰지만 record 훅이 미설치입니다 — 재시작 필요\n");
                }
            }

            // 챔피언 id 매핑 — 새로 관측된 게 있으면 파일 갱신(detour 안이 아니라 여기서 IO)
            if CHAMP_PROBE_ON.load(Ordering::Relaxed) {
                let seen = CHAMP_SEEN.load(Ordering::Relaxed);
                if seen != last_champ {
                    last_champ = seen;
                    flush_champ_ids();
                    log(&format!("[{}ms] [champ] {}종 수집 → champ_ids.txt\n", now_ms(), seen));
                }
            }

            // 대체 구현이 켜져 있을 때만 발화 카운터를 남긴다.
            if REPL_INSTALLED.load(Ordering::Relaxed) {
                let cur = (
                    REPL_CALLS.load(Ordering::Relaxed),
                    REPL_STEPS.load(Ordering::Relaxed),
                    REPL_FAULTS.load(Ordering::Relaxed),
                );
                if cur != last_report {
                    last_report = cur;
                    // record=학습 진입 횟수 / qmiss=선수 매칭 실패로 지표 폴백한 횟수
                    // ★qmiss 가 calls 와 비슷하면 지표가 사실상 안 먹고 있는 것이다.
                    log(&format!(
                        "[{}ms] [repl] calls={} steps={} faults={} record={} qmiss={}\n",
                        now_ms(),
                        cur.0,
                        cur.1,
                        cur.2,
                        RECORD_CALLS.load(Ordering::Relaxed),
                        QUALITY_MISS.load(Ordering::Relaxed)
                    ));
                }
            }
        }
    });

    ModRegistration::new(MOD_ID)
}
declare_mod!(init);
