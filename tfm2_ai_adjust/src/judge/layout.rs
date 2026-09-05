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

// ── 월드 모드(`World<Mode>` 모노모픽 3종) ── [디컴 ghidra-re 2026-09-06: RE\…WorldOps-슬롯-0x40…]
//   vt+0x40(data) = (rax=모드 태그, rdx=&모드 데이터). MOBA = tag 0 → 모드 데이터 = w+0xed00 / tag 1·2 → w+0xecc8.
//   ⚠steal·hunt_and_* 의 "vt+0x40 != 0 → panic" 은 assert 가 아니라 **MOBA 전용 가드**이고, 직후 `[rdx+…]` 는 모드 데이터를 읽는다.
//   (2026-09-06 02:50 첫 검증판 steal DIFF 226건의 원인 = 이걸 홀더/페이로드 기준으로 읽었던 것.)
pub const W_MODE_TAG: usize = 0xecc2;        // u8 (⚠모드 태그로 추정했으나 03:05 검증판에서 기각 — 기록용)
pub const VT_WORLD_MODE: usize = 0x40;       // vt+0x40 구현 RVA 로 모드 판정(모노모픽 상수 반환)
pub const VT40_IMPL_MOBA: usize = 0x1849400; // vt2 0x1434ae890 → tag 0, mode = w+0xed00
pub const VT40_IMPL_TAG1: usize = 0x186b080; // vt1 0x1434ae560 → tag 1, mode = w+0xecc8
pub const VT40_IMPL_TAG2: usize = 0x187f240; // vt0 0x1434ae230 → tag 2, mode = w+0xecc8
pub const W_MODE_DATA_MOBA: usize = 0xed00;
pub const W_MODE_DATA_OTHER: usize = 0xecc8;
pub const STEAL_PHASE: usize = 0x8;
// 모드 데이터 안의 목표 핸들 Vec(모르가드=T0 / 세르펜=T1): +ptr/+len. 핸들러는 len!=0 이면 **첫 핸들 `**ptr`** 을 리졸버에 넘긴다.
pub const T0_LEN: usize = 0x1a8;   // = w+0xeea8 (모르가드 핸들 Vec len)
pub const T0_PTR: usize = 0x1a0;   // = w+0xeea0
pub const T1_LEN: usize = 0x1d8;   // = w+0xeed8 (세르펜)
pub const T1_PTR: usize = 0x1d0;   // = w+0xeed0
// 기타 슬롯 순수재현용 [ghidra-re 2026-09-06]
pub const W_ROSTER_REC_BASE: usize = 0x858;  // vt+0x150 핸들→로스터 레코드: base·count·stride, rec+0x9c8(alive)·+0x9d0(handle)·+0x9c0(role)·+0x930(side)
pub const W_ROSTER_REC_CNT: usize = 0x860;
pub const REC_STRIDE: usize = 0x9e0;
pub const REC_ALIVE: usize = 0x9c8; pub const REC_HANDLE: usize = 0x9d0; pub const REC_ROLE: usize = 0x9c0; pub const REC_SIDE: usize = 0x930;
pub const ENT_VIS_BASE: usize = 0x38;        // vt+0xf8 now-visible: [e+0x38+side*0x18]==0
pub const ENT_VIS_STRIDE: usize = 0x18;
pub const W_CFG_FLAG: usize = 0xecc1;        // vt+0xe8: u8 설정 플래그
pub const W_SIDE_CFG: usize = 0xb3b0;        // vt+0x108: 24B 사이드별 설정 = w+0xb3b0+side*0x18
pub const W_SIDE_CFG_STRIDE: usize = 0x18;
pub const W_JUNGLE_STATE: usize = 0xed18;    // vt+0xe0: &(w+0xed18) (MOBA 전용)
pub const W_KILLS: usize = 0xeca0;           // vt+0x290: u64[2] 사이드 킬

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
// ── passive_line(Plan 3, 0xd2c5d0) 전용 ── [디컴 2026-09-06]
pub const W_TICK: usize = 0xec98;            // vt+0x28 = [w+0xec98] (현재 틱 추정)
pub const PL_F110: usize = 0x110;            // payload 플래그: !=0 → code 5
pub const PL_F112: usize = 0x112;            //   ==1 && F115==0 → code 3/4(F113), out+8=lane
pub const PL_F113: usize = 0x113;
pub const PL_F115: usize = 0x115;
pub const PL_LANE: usize = 0x116;            //   u8 레인 f (0/1/2)
pub const ORDER_PLAN: usize = 0x41f;         // p7: u8 plan 종류(8=SF, 1, 0…)
pub const ORDER_SF: usize = 0x420;           // p7: u8 sf 레인
pub const G_PHASE: usize = 0x38;             // u8 [G+0x38] 국면(비트마스크 0x1a1 = {0,5,7,8} 에서 미드 특수블록)
pub const CFG_8A8: usize = 0x8a8;            // [cfg+0x8a8] 시각(틱·tps*30 과 비교)
pub const HOLDER_LANES: usize = 0x10;        // holder[2] = 레인 표 base
pub const LANE_STRIDE: usize = 0x2e8;        //   + side*0x2e8 (사이드별 레인 데이터)
pub const LANE_SUB_MID: usize = 0x50;        //   레인 f 서브 오프셋: 0→+0 · 2→+0x50 · 그 외(1)→+0x28
pub const LANE_SUB_SIDE: usize = 0x28;
pub const LR_STATE: usize = 0x0;             //   i32 (1 = 활성)
pub const LR_TARGET_H: usize = 0x8;          //   u64 목표 핸들
pub const LR_F10: usize = 0x10;              //   i64 (< 0x7d1 비교)
pub const LR_F18: usize = 0x18;              //   i64 (부호 → cVar15 2/0)
pub const LR_F20: usize = 0x20;              //   i32 (< 3 → code 4 / else 5)
pub const LANE_F60: usize = 0x60;            //   [lanes+side*0x2e8+0x60] i64 (> 999)
pub const LANE_ROSTER: usize = 0x1e0;        //   [lanes+other*0x2e8+0x1e0+role*8] i64 (틱 임계, +0x78 여유)
pub const LANE_ROSTER_MARGIN: u64 = 0x78;
pub const X_MINION_PTR: usize = 0x130;       // X+0x130+side*0x20 : 슬라이스 ptr / +0x148 : len
pub const X_MINION_LEN: usize = 0x148;
pub const X_TOWER_A: usize = 0x180;          // X+0x180+side*8+lane*0x20 : 1차 타워 / +0x190 : 2차
pub const X_TOWER_B: usize = 0x190;
pub const X_NEXUS: usize = 0x170;            // X+0x170+side*8 : 넥서스(0 → panic)
pub const ENT_KIND: usize = 0x68;
pub const ENT_SUBTYPE: usize = 0x70;
pub const ENT_F88: usize = 0x88;
pub const GRID_TBL: usize = 0x1c98;          // [G+0x20]+0x1c98 + (y/32000)*0xf0 + (x/32000)*8 (30×30, clamp 29)
pub const GRID_CELL: u64 = 32000;
pub const GRID_MAX: u64 = 29;
pub const D2_150K_SHR8: u64 = 0x53d1ac1;     // (d²>>8) < 이 값 ⟺ d < 150000
pub const D2_170K_SHR8: u64 = 0x6ba9301;     // d < 170000
pub const D2_200K_PLUS1: u64 = 0x9502f9001;  // d² < 200000²+1
pub const LANE_ANCHOR_A: [u64; 3] = [820000, 817000, 880000];   // .rdata 0x1433d9ea0
pub const LANE_ANCHOR_B: [u64; 3] = [80000, 144000, 144000];    // .rdata 0x1433d9eb8
// MovePriority(out 0x30) 코드 [디컴] — move_name 표: 0 히트맨 1 스킬 2 전진 3 도망 4 귀환 5 주변 6 포지셔닝 7 추적 8 라인 9 라인공격 10 라인대기 11 결사전
pub const MP_CODE_AROUND: u64 = 5;
pub const MP_CODE_LINE_ATTACK: u64 = 9;
pub const MP_CODE_EPIC_HB: u64 = 0xb;
pub const MP_CODE_SERPEN_HB: u64 = 0xe;

// ── ★이 모드의 바이트패치 사이트(즉치 RVA) — 포팅 범위 안에 있는 것 ── [orig_table.rs 교차검사 2026-09-06, 0.5.8]
//   재현 대상은 정적 exe 가 아니라 **바이트패치가 적용된 실행 이미지**다. 포팅은 이 즉치를 라이브로 읽는다(judge::live_imm8/16).
//   목록 갱신 = `python MIGiport.py sites` (포팅 함수·콜리 범위 ∩ orig_table.rs). 라이브 승격 시 그 노브를 포팅 안으로 옮기고 사이트 패치는 은퇴.
pub const SITE_VW_CHECK_IMM: usize = 0x1323a5b;      // recently_seen `add rbx, imm8` (노브 vw_check, 원본 0x78) — 5판을 태운 그 자리
pub const SITE_VW_LANE_IMM: usize = 0xd2cd11;        // passive_line MAIN `add rdi, imm8` ×5 중 첫 사이트(노브 vw_lane, 5곳 동일값, 원본 0x78)
pub const SITE_HD_PHASE_EPIC_IMM: usize = 0xccc1f2;  // epic hunt_and_battle `mov word [rax+0x10], imm16`(노브 hd_phase, 원본 1)
pub const SITE_HD_PHASE_SERPEN_IMM: usize = 0xccc5a2; // serpen 동일

// ── defense_nexus(Plan 17, 0xd2da10) 전용 ── [capstone 디스어셈 2026-09-06, 0.5.8]
pub const ORDER_F0: usize = 0xf0;                 // p7(오더)+0xf0 u8 — !=0 이면 즉시 code 5 (p3>1 경로)
pub const ENT_TARGET_H: usize = 0x90;             // 유닛 +0x90 u64 = 공격 목표 핸들(+0x88==1 일 때)
pub const X_LIST3_PTR: [usize; 3] = [0x10, 0x50, 0x90];   // X + off + side*0x20 : 사이드별 유닛 리스트 3개(ptr)
pub const X_LIST3_LEN: [usize; 3] = [0x28, 0x68, 0xa8];   //                                          (len)
// nx_dn_* 바이트패치 사이트 즉치(aiport sites 실측 11곳) — 검증판은 live_imm 로 읽는다
pub const SITE_DN_NEAR_P1_IMM: usize = 0xd2dc06;  // movabs rax, 120000²+1 (0xd2dc04·0xd2dd30 두 곳 동일값 — 첫 곳)
pub const SITE_DN_NEAR_IMM: usize = 0xd2df9a;     // movabs r10, 120000²
pub const SITE_DN_NEXUS_HP_IMM: usize = 0xd2e084; // cmp rax, 50 (4곳 동일값 — 첫 곳)
pub const SITE_DN_HP_LOW_IMM: usize = 0xd2e1c0;   // cmp [rbp-0x18], 31
pub const SITE_DN_HP_CRIT_IMM: usize = 0xd2e1ce;  // cmp [rbp-0x18], 21
pub const SITE_DN_PRED_P1_IMM: usize = 0xd3fc07;  // 0xd3fa80 안 movabs 240000²+1
pub const SITE_DN_VISION_IMM: usize = 0xd3fc76;   // 0xd3fa80 안 add, 0x78

// ── dn_reach(0xd3fe50) 전용 ── [capstone 2026-09-06, 0.5.8]
pub const ENT_F438: usize = 0x438;       // 기본 사거리
pub const ENT_F470: usize = 0x470;       // i32 사거리 보정 %(0 이면 미적용)
pub const ENT_F680: usize = 0x680;       // u64 반경(체구) — (0x470+100)*0x680/100
pub const ENT_SLOT0: usize = 0x490;      // 스킬/평타 슬롯0: [+0] Arc data · [+8] vt · [+0x10] 기본 · [+0x18] 레벨당 · [+0x30] i32 flag(−1 = 없음)
pub const ENT_SLOT0_FLAG: usize = 0x4c0; // = ENT_SLOT0 + 0x30
pub const EFF_SLOT_E8: usize = 0xe8;     // 이펙트 vt 슬롯: 사거리 보너스 fn(&self, &ent, &nexus) -> u64
pub const EFF_E8_ZERO: usize = 0x9db70;              // ×280  xor eax,eax
pub const EFF_E8_MAX_CHILDREN: usize = 0x12a67a0;    // ×9    자식 (data,vt)[n] (self+8/+0x10) 의 max
pub const EFF_E8_LVL3_NEXUS_TABLE: usize = 0x12b9e60;// ×1    level≥3 && nexus+0x298 표(stride 0x38, [+0x34]==1) → [self+0x40]
pub const EFF_E8_BUFF_FLAG: usize = 0x16a8c10;       // ×1    [self+0x62]==1 && ent+0x2f8 버프 vt+0x48==1 → 9,999,999 (버프 층 미재현 → NA)

