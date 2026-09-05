//! layout — 구조체 오프셋·vtable 슬롯 상수(**버전 태그 필수**). 포팅 코드의 매직 넘버는 전부 여기로.
//! 근거 표기 규칙: `[디컴]` = 0.5.8 디컴 실측(착수서 §6) / `[가설 0.5.0]` = 구 순수재현 값을 가설로 둔 것 = ⬜0.5.8 미검증.
//! 패치 대응: aidiff 가 EDITED 로 찍은 함수의 `offs` 히스토그램과 이 표를 대조해 밀린 값을 고친다(MIG\offsets.py 축과 같은 원리).
#![allow(dead_code)]
pub const VER: &str = "0.5.8";

// ── SmallAction ── [디컴] stride 0xb8 · tag +0xb1
pub const SA_STRIDE: usize = 0xb8;
pub const SA_TAG: usize = 0xb1;

// ── 엔티티 ── [디컴]
pub const ENT_HANDLE: usize = 0x5c0;
pub const ENT_LEVEL: usize = 0x5c8;
pub const ENT_MAXHP: usize = 0x628;
pub const ENT_X: usize = 0x660;
pub const ENT_Y: usize = 0x668;
pub const ENT_HP: usize = 0x670;
pub const ENT_RANGE_BASE: usize = 0x680;
pub const ENT_RANGE_PCT: usize = 0x470;
pub const ENT_HITBOX: usize = 0x438;
pub const ENT_ABIL0: usize = 0x490;
pub const ENT_ABIL_STRIDE: usize = 0x38;

// ── 선수 sim 상태(p5) ── [디컴]
pub const P5_SIDE: usize = 0x930;
pub const P5_ROLE: usize = 0x9c0;
pub const P5_TICKS_PER_SEC: usize = 0x12f8;

// ── World-ish X(*p6) ── [디컴 recall 0xcc5fc0] 로스터 = X+0x1e0 + side*0x28 + role*8
pub const X_ROSTER: usize = 0x1e0;
pub const ROSTER_SIDE_STRIDE: usize = 0x28;

// ── WorldOps vtable 슬롯 ── [디컴] 0.5.8 은 +0x30 이상 Δ+0x10(0.5.7 +0x1e0 → +0x1f0)
pub const VT_WORLD_ENTITY: usize = 0x1f0;

// ── 핸들 → 엔티티 순수 재현(WorldOps vt+0x1f0 = RVA 0x184ad60 의 재현) ── [디컴 ghidra-re 2026-09-06, 0.5.8 실측]
//   정본 = REPORT\tfm2_ai_adjust\RE\2026-09-06_WorldOps-vt0x1f0-핸들엔티티리졸버-0.5.8.md. 0.5.7 과 동일 로직(슬롯만 Δ+0x10).
//   ⚠구 재현(geom_resolve150)의 싱글턴 핸들 필드 0x618 은 0.5.0_3 값 — 0.5.5+ 는 **0x630**. 이웃 슬롯: +0x1e8 = 원시 엔트리, +0x1f8 = &mut 변형.
pub const W_L3_CNT: usize = 0x758;      // 슬롯맵 엔트리 수
pub const W_L3_TBL: usize = 0x750;      // 슬롯맵 테이블(엔트리 0x10B: [+0] i32 tag(1=alive) · [+8] u64 index)
pub const W_SLOT_STRIDE: usize = 0x10;
pub const W_ENT_CNT: usize = 0x740;     // 엔티티 수
pub const W_ENT_BASE: usize = 0x738;    // 엔티티 배열 base
pub const ENT_STRIDE: usize = 0x6c0;    // 엔티티 stride
pub const W_SINGLETON_H: usize = 0x630; // 인라인 싱글턴 폴백: 핸들(cmp [rcx+0x630], rdx)
pub const W_SINGLETON_TAG: usize = 0x70; //   i32 tag(-1 = 없음), 엔티티 = data+0x70

// ── 목표 슬롯 쌍(모르가드=T0 / 세르펜=T1) ── [디컴 0xcbbca0(steal: p6 기준)·0xccc010/0xccc3c0(hunt_and_battle: p2=Plan payload 기준)]
//   같은 오프셋이 두 구조체에서 관측됨 — steal 의 arg6 "ctx" 와 Plan payload 가 같은 객체일 가능성(⬜미확정). 사용처가 base 를 명시한다.
pub const STEAL_PHASE: usize = 0x8;
pub const T0_SET: usize = 0x1a8;   // 모르가드 목표 지정 여부(≠0)
pub const T0_SLOT: usize = 0x1a0;  //   → *slot = 목표 핸들
pub const T1_SET: usize = 0x1d8;   // 세르펜 목표
pub const T1_SLOT: usize = 0x1d0;
pub const HOLDER_T0_SET: usize = T0_SET; pub const HOLDER_T0_SLOT: usize = T0_SLOT; pub const HOLDER_T1_SET: usize = T1_SET; pub const HOLDER_T1_SLOT: usize = T1_SLOT;

// ── Plan 디스패처 핸들러 공통(p6 = &Holder) ── [디컴 0xccc010 · 0xcaf9f0 인라인 아암 16]
pub const HOLDER_X: usize = 0x0;      // *p6 = X(월드 data/vt/로스터)
pub const HOLDER_G: usize = 0x8;      // *(p6+8) = G(게임 컨텍스트)
pub const G_CFG: usize = 0x8;         //   *(G+8) = cfg(+0x12f8 tick/sec)
pub const G_BOXES: usize = 0x20;      //   *(G+0x20) = 홈존 박스 표 소유 객체
pub const BOX_BASE: usize = 0x6d70;   //     박스 = obj+0x6d70+side*0x20: [xlo, ylo, xhi, yhi] u64 4개
pub const BOX_STRIDE: usize = 0x20;
pub const CFG_TPS: usize = 0x12f8;
// hunt_and_battle 의 p7(타이머 쌍): epic +0x98/+0xa0 · serpen +0xd0/+0xd8 (`p7[b] < tps + p7[a]` 이면 최근 창)
pub const P7_EPIC_TA: usize = 0x98;  pub const P7_EPIC_TB: usize = 0xa0;
pub const P7_SERPEN_TA: usize = 0xd0; pub const P7_SERPEN_TB: usize = 0xd8;
// MovePriority(out 0x30) 코드 [디컴] — move_name 표: 0 히트맨 1 스킬 2 전진 3 도망 4 귀환 5 주변 6 포지셔닝 7 추적 8 라인 9 라인공격 10 라인대기 11 결사전
pub const MP_CODE_AROUND: u64 = 5;
pub const MP_CODE_LINE_ATTACK: u64 = 9;
pub const MP_CODE_EPIC_HB: u64 = 0xb;
pub const MP_CODE_SERPEN_HB: u64 = 0xe;
