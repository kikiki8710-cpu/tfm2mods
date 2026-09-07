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
pub const EFF_E8_BUFF_FLAG: usize = 0x16a8c10;
pub const EFF48_OPT_PTR: usize = 0x16b8820;          // 효과 리스트 vt+0x48 impl: rax = ([data+0x10] != 0), rdx = [data+8] (capstone 2026-09-06 19:52)       // ×1    [self+0x62]==1 && ent+0x2f8 버프 vt+0x48==1 → 9,999,999 (버프 층 미재현 → NA)

// ── passive_jungle(Plan 7, 0xd2e500) · dyn_eff 전용 ── [capstone + ghidra-re 2026-09-06, 0.5.8]
pub const PLAN_PJ_SIDE: usize = 0x48;            // 플랜 인자 +0x48 u64 side(0/≠0)
pub const PLAN_PJ_CAMP: usize = 0x60;            // 플랜 인자 +0x60 u8 camp(0..5)
pub const PJ_WP_D2: u64 = 14_400_000_000;        // 캠프 위치 게이트 120000² (사이트 없음 · 노브 d7_wp_dist2)
pub const SITE_PJ_HP_SELFHEAL_IMM: usize = 0xd2ed5a; // cmp rax, 21 (노브 d7_hp_selfheal, 사이트 0xd2ed57)
pub const SITE_PJ_HP_NORMAL_IMM: usize = 0xd2ed70;   // cmp rax, 41 (노브 d7_hp_normal, 사이트 0xd2ed6d)
// ★★combat_score 계열이 **자기 모드의 바이트패치 대상**인 즉치들(detour.rs:1425-1445, 1980-2027).
//   재현이 exe 원본 상수를 쓰면 노브를 건드린 순간 전부 DIFF 된다 — 반드시 **실행중 바이트**를 읽을 것.
//   값은 imm 위치(명령 시작 + prefix 길이)이다. RE 2026-09-07 `S12잔차-정체는-자기모드-바이트패치`.
// position_eval(0xd851d0 본체) 의 패치 즉치 — RE 2026-09-07 `position_eval-패치즉치18곳-재현매핑`.
//   현재 cfg 기준 **원본과 다른 것만** 상수화했다(같은 값이면 읽어도 무해하나 노브를 바꾸면 즉시 깨진다).
pub const SITE_PE_TOWER_MARGIN_A_IMM: usize = 0xd87ffa; // add rax,18000 — S7 타워 게이트(사이트 0xd87ff8) · 노브 pe_tower_margin
pub const SITE_PE_TOWER_MARGIN_B_IMM: usize = 0xd89280; // add r9,18000  — S11 액션 장판 cover (사이트 0xd8927d)
pub const SITE_PE_COUNT_D2_INC_IMM: usize = 0xd8b66d;   // movabs 120000²+1 (사이트 0xd8b66b) · 노브 pe_count_radius
pub const SITE_PE_COUNT_D2_IMM: usize = 0xd8beec;       // movabs 120000²   (사이트 0xd8beea)
pub const SITE_PE_KIND_MASK_IMM: usize = 0xd86a08;      // mov ecx,0x503 (사이트 0xd86a07) · 노브 pe_kind_mask
pub const SITE_PE_FILTER_D2_SHR8_IMM: usize = 0xd86f21; // cmp …,87890625 (사이트 0xd86f1e 외 5곳 동값)
pub const SITE_PE_NEAR_D2_SHR8_IMM: usize = 0xd8715d;   // cmp …,19140625 (사이트 0xd8715a · 0xd87b4a)
pub const SITE_PE_FIELD_D2_SHR8_IMM: usize = 0xd890f3;  // cmp …,244140624 (사이트 0xd890f0)
pub const SITE_SC_FOCUS_CAP_IMM: usize = 0xd5f5cd;   // cmp rax,80  (노브 sc_focus_cap)  사이트 0xd5f5ca
pub const SITE_SC_KILL_CAP_IMM: usize = 0xd5fd8f;    // cmp rcx,80  (노브 sc_kill_cap)   사이트 0xd5fd8c · 쌍 0xd5fe81
pub const SITE_SC_KILL_PCT_IMM: usize = 0xd5fe69;    // cmp rax,60  (노브 sc_kill_pct)   사이트 0xd5fe66
pub const SITE_SC_DIVE_MARGIN_IMM: usize = 0xd5e92f; // add rax,15000 imm32 (노브 sc_dive_margin) 사이트 0xd5e92d
pub const SITE_BV_CAP_MAIN_IMM: usize = 0xe01880;    // cmp rcx,160 imm32 (노브 bv_cap_main) — e01450 소유분
pub const SITE_BV_CAP_HALF_E019D0_IMM: usize = 0xe01bfe; // cmp rcx,80 (노브 bv_cap_half) — e019d0 소유분
pub const SITE_BV_CAP_HALF_E02020_IMM: usize = 0xe0228e; // cmp rcx,80 (노브 bv_cap_half) — e02020 소유분
pub const ENT_F3F0: usize = 0x3f0;               // i32 흡혈 누산(>0 이면 지속회복 있음)
pub const ENT_F3FC: usize = 0x3fc;               // i32 공속 보정 %(+100)
pub const ENT_SLOT1: usize = 0x4c8;              // 스킬1 슬롯(Arc data·+8 vt) · flag +0x4f8
pub const ENT_SLOT1_FLAG: usize = 0x4f8;
pub const ENT_SLOT2: usize = 0x500;              // 스킬2 슬롯 · flag +0x530 (level≥3 에서만 검사)
pub const ENT_SLOT2_FLAG: usize = 0x530;
pub const ENT_PROV0_DATA: usize = 0x570;         // 평타 프로바이더 Box<dyn>(data) / +0x578 vt
pub const ENT_PROV0_VT: usize = 0x578;
pub const ENT_BUFF_BLOCK: usize = 0x370;         // 버프 누산 블록(ENT_VT+0x40): +0xa8/+0xb0 관통 · +0xd0/+0xd8/+0xe0/+0xf0 피해 보정
pub const ENT_STATS: usize = 0x618;              // 스탯 스냅샷(ENT_VT+0x38): +0x10 사용
pub const ENT_DEF_P: usize = 0x630;              // 물리 방어 / +0x638 마법 방어
pub const ENT_DEF_M: usize = 0x638;
pub const MAPDEF_CAMPS_PTR: usize = 0x68;        // map_def(=G+0x20 객체) 캠프 표 ptr / +0x70 len · stride 0x28 {pos0, pos1, kind u8 @0x20}
pub const MAPDEF_CAMPS_LEN: usize = 0x70;
// effect vt 구현체 RVA(0.5.8 정적 열거) — dyn_eff 디스패치 키
pub const EFF28_ZERO: usize = 0x109bad0;         // (0,0) ×152
pub const EFF28_PAIR_RAW: usize = 0x146a90;      // ([s],[s+8]) ×1
pub const EFF28_PAIR_BYKIND: usize = 0x29b84a0;  // [s+8]==1 ? (0,[s]) : ([s],0) ×1
pub const EFF38_ZERO: usize = 0x9db70;           // 0 ×194
pub const EFF38_GET28: usize = 0x2f840;          // [s+0x28] ×10
pub const EFF40_ZERO: usize = 0x9db70;           // 0 ×197
pub const EFFA0_NONE: usize = 0x109baa0;         // type=−1 ×171

pub const EFF28_GENERIC: usize = 0x1708310;        // q400([s+0x18]*AD)+q400([s+0x20]*stats[0x10])+[s+0x10] (몬스터 평타, 리플레이 13만회)
pub const EFF40_SUM_08_10: usize = 0x12a5660;      // 자식 Vec [s+8]/[s+0x10] stride 0x10 합
pub const EFF40_SUM_20_18: usize = 0x12a5ae0;      // 자식 Vec [s+0x20]/[s+0x28] stride 0x18 합
pub const EFF40_SUM_50_18: usize = 0x1248040;      // 자식 Vec [s+0x50]/[s+0x58] stride 0x18 합
pub const EFF40_SUM_50_18_68_10: usize = 0x122e360;// 두 Vec 합([s+0x50]/0x58 s0x18 + [s+0x68]/0x70 s0x10)
pub const EFF40_LEAF_ADAP: usize = 0x117ca10;      // q400([s+0x18]*AD)+q400([s+0x20]*AP)+[s+0x28]
pub const EFF40_LEAF_STACK: usize = 0x12b2e90;     // [s+0x10]+[s+0x18]*(stats[0x38]+1)
pub const EFF40_LEAF_RATIO: usize = 0x12b1550;     // ([s]+[s+8]*AP)*([s+0x20]/[s+0x28])
pub const EFFA0_WIND_SPEED: usize = 0x12266f0;     // BuffState 상수 생성(type 1, vamp 0)
pub const EFF40_SUM_08_18: usize = 0x13bfae0;      // 자식 Vec [s+8]/[s+0x10] stride 0x18 합
pub const EFF40_SUM_48_10: usize = 0x12a4f30;      // 자식 Vec [s+0x48]/[s+0x50] stride 0x10 합
pub const EFF40_SUM_RATIO_68_50: usize = 0x16a3190;// Σ[0x68/0x70 s0x18] + ([s+0x78]/max(1,[s+0x80])) * Σ[0x50/0x58 s0x18]
pub const EFF40_SWITCH_BY_BUFF: usize = 0x16063d0; // 버프 이름([s+8],[s+0x10]) 보유 여부로 자식 (0x18/0x20 | 0x28/0x30) 선택 → 그 +0x40
pub const EFFA0_MERGE_08_10: usize = 0x12a5770;    // 자식 [s+8]/[s+0x10] stride 0x10 병합(type=첫 Some, vamp=Σ)
pub const EFFA0_MERGE_50_18: usize = 0x1248150;    // [s+0x50]/[s+0x58] stride 0x18
pub const EFFA0_MERGE_20_18: usize = 0x1455d70;    // [s+0x20]/[s+0x28] stride 0x18
pub const EFFA0_MERGE_50_18_68_10: usize = 0x153b540; // 두 범위
pub const ENT_BUFFS_PTR: usize = 0x2e0;            // 엔티티 버프 목록 ptr / +0x2e8 len · stride 0x120 · [0]=name len u32, [4..]=name
pub const ENT_BUFFS_LEN: usize = 0x2e8;
pub const EFF40_SWITCH_BY_LEVEL3: usize = 0x164ea30; // level≥3 ? 자식1([s+0x10],[s+0x18]) : 자식0([s],[s+8]) → 그 +0x40
pub const EFFA0_MERGE_50_18_68_18: usize = 0x16a33f0; // 두 범위 모두 stride 0x18 (0x126f800)
pub const EFFA0_MERGE_08_10_B: usize = 0x13be350;     // [s+8]/[s+0x10] stride 0x10 (0x126e6c0)
pub const EFFA0_DELEGATE_RAW: usize = 0x1153860;      // 자식 (payload=[s] 그대로, vt=[s+8]) 의 +0xa0 로 tail
pub const EFFA0_SWITCH_BY_BUFF: usize = 0x16065e0;    // 버프 이름([s+8],[s+0x10]) 보유 ? 자식1(0x28/0x30) : 자식0(0x18/0x20) → +0xa0
pub const EFFA0_INLINE_STATE: usize = 0x11507c0;      // [s+0x120]!=0 → None ; 아니면 payload 안의 BuffState 복사(type=[s+0x48], vamp=[s+0x80])
pub const EFFA0_MERGE_08_18: usize = 0x13bfbf0;       // [s+8]/[s+0x10] stride 0x18 (0x126ec80)
pub const EFFA0_MERGE_48_10: usize = 0x12a50c0;       // [s+0x48]/[s+0x50] stride 0x10 (0x126e6c0)

pub const EFFA0_CONST0: usize = 0x16a2ee0;         // 상수 생성(type 0, vamp 0)
pub const EFFA0_SWITCH_BY_LEVEL3: usize = 0x164eba0; // level>=3 ? 자식1 : 자식0 → +0xa0
pub const EFFA0_STATSCALED: usize = 0x114b3f0;     // payload 안 BuffState 복사 후 스탯 비례 필드 갱신(+0x48/+0x80 은 불변)
pub const EFFA0_MERGE_20_18_INLINE: usize = 0x12a5be0; // 병합기 인라인판([s+0x20]/[s+0x28] stride 0x18)
pub const EFFA0_COPY_STATE: usize = 0x106a440;     // memcpy(out, payload, 0x120) → (type [s+0x48], vamp [s+0x80])
pub const EFFA0_CONST1_STAT: usize = 0x17c9e90;    // 상수 생성(type 1, vamp 0; +0x8c 만 스탯 비례)
pub const EFFA0_CONST1_ENCH: usize = 0x12aa290;    // 상수 생성(type 1, vamp 0) enchanter_skill1
// ── camp_pos(0xffa3e0) 스레드로컬 메모 캐시 (접근자 0x1395a70: eax=[TLS_IDX]; rcx=gs:[0x58]; rcx=[rcx+rax*8]; cache=rcx+0x17d50; init=[rcx+0x17ee0])
pub const CAMP_MEMO_TLS_IDX: usize = 0x4856da0;  // .data u32 = 이 exe 의 TLS 슬롯 인덱스(런타임 이미지에서 읽는다)
pub const CAMP_MEMO_OFF: usize = 0x17d50;        // TLS 블록 안 캐시(0x190B): [0]borrow [1]key=map_def ptr · 항목 i=camp*6+side*3+2: [i]valid u8 [i+1]x [i+2]y
pub const CAMP_MEMO_INIT: usize = 0x17ee0;       // 지연초기화 플래그 바이트(0 = 아직 없음 → 재계산)
pub const EFF40_SUM_68_18_80_10: usize = 0x13409d0; // 두 Vec 합([s+0x68]/0x70 stride 0x18 + [s+0x80]/0x88 stride 0x10) — SET2 리플레이 신규(×1398)
// ── battle(disc 9, 0xdfdfc0) — 2026-09-06 15:20
pub const BT_TOWER_MARGIN: u64 = 120_000;        // 타워 사거리 여유(0x1d4c0) · slot0.flag==-1 이면 120000² 고정
pub const BT_PL_KIND: usize = 0x58;              // payload+0x58 kind(u64) — (0x6f>>kind)&1 인 kind 만 계산
pub const BT_PL_HANDLE: usize = 0x60;            // payload+0x60 target 핸들
pub const BT_PL_FLAG: usize = 0xf6;              // payload+0xf6 flag(0 이면 계산 없음)
pub const BT_PL_FD: usize = 0xfd;                // payload+0xfd (1 이면 chase=0)
pub const BT_PL_LANE: usize = 0xfe;              // payload+0xfe lane(0xff = 없음)
pub const ENT_LANE: usize = 0x128;               // 유닛 lane 바이트(타워 kind 2 매칭)
pub const X_FIXED6: [usize; 6] = [0x180, 0x1a0, 0x1c0, 0x190, 0x1b0, 0x1d0]; // 0x1820ee0 고정 6슬롯(X+off+side*8) 순회 순서
pub const EFFA0_MERGE_68_18_80_10: usize = 0x1340e80; // 두 범위([s+0x68]/0x70 stride 0x18 + [s+0x80]/0x88 stride 0x10, 병합기 0x126f240) — 15:22 리플레이 신규 ×7124
pub const EFFA0_CONST1_ENCH2: usize = 0x12bc970; // 상수 생성(type 1, vamp 0) enchanter_skill2 (.pdata 없는 leaf) — 15:22 리플레이 신규 ×1052
// ── obj_helpers(hunt_and_poke 콜리 계층) — 2026-09-06 16:00 (디컴 0xeca200/0xeca430/0xec9840/0xecacc0/0xdd5db0/0xdcc100/0xec9bf0/0xeca9a0)
pub const X_OBJ_CNT: usize = 0x21c0;             // X+0x21c0 + kind*0x10 + side*8 : 오브젝티브 확보 카운터(u64)
pub const LANE_OBJ_T: usize = 0x10;              // lanes+side*0x2e8 (+0/+0x28/+0x50 = kind 0/1/2) +0x10 i64 시각 · +0x20 i32 횟수
pub const LANE_OBJ_N: usize = 0x20;
pub const ORDER_TS4: usize = 0x80;               // order(p8) +0x80 에픽 / +0x88 세르펜 타임스탬프
pub const ORDER_TS5: usize = 0x88;
pub const ORDER_LASTPOS: usize = 0x230;          // order +0x230 + i*0x10 : 적 챔프 i 마지막 관측 (x,y)
pub const ORDER_LASTTICK: usize = 0x2d0;         // order +0x2d0 + i*8   : 적 챔프 i 마지막 관측 틱
pub const ENT_SPEED: usize = 0x640;              // 이동속도(틱당) — 0xdcc100 도달 가능성
pub const W_GRID: usize = 0xb450;                // WorldOps vt+0x100(0x184acb0): [w + 0xb450 + side*0xe10 + gy*0x78 + gx*4] i32 > 0
pub const W_GRID_SIDE: usize = 0xe10;
pub const W_GRID_ROW: usize = 0x78;
// ── hunt_poke(disc 12 epic 0xdefcd0) — 2026-09-06 16:20
pub const HP_PL_F18: usize = 0x18;               // payload+0x18/+0x19 인터럽트 플래그 → code 0x12/10
pub const HP_PL_F19: usize = 0x19;
pub const HP_PL_HASPATH: usize = 0x1a;           // payload+0x1a 경로 보유(1) · +8 경로 핸들 ptr · +0x10 len (게임이 쓴다)
pub const HP_PL_PATH_PTR: usize = 0x8;
pub const HP_PL_PATH_LEN: usize = 0x10;
pub const P5_CFG_SELF: usize = 0x4f8;            // sim+0x4f8 (cfg_flag≠0 일 때 side_cfg 대신 쓰는 챔피언별 24B)
pub const HP_HOME_LO: u64 = 64000;               // 홈 띠: side0 x<=64000 && 896000<=y<=960000 / side1 892000<=x<=960000 && y<=64000
pub const HP_HOME_HI: u64 = 960000;
pub const HP_HOME_X1: u64 = 0xd9c5f;
pub const HP_HOME_Y1: u64 = 0xdabff;
pub const HP_P7_A: usize = 0x88;                 // p7 타이머 min(+0x88, +0x98) > tps*5 면 경로/그리드 블록
pub const HP_P7_B: usize = 0x98;
pub const HP_PATH_NEAR_D2: u64 = 0x9502f9001;    // 200000²+1 = 40,000,000,001 (경로 핸들 근접 → 5) ~~0x9502f901(자릿수 누락)~~ 16:25 정정
pub const HP_ENGAGE_D2: u64 = 0x53d1ac101;       // 150000²+1 (engage_gate 0 && 근접 → code 0xc)
pub const G_GRID: usize = 0x38b8;                // G+0x20 obj +0x38b8 : 30×30 u64 영향력 타일(값 7 = 우세) · 행 0xf0
pub const G_GRID_ROW: usize = 0xf0;
pub const HP_S_P7_A: usize = 0xc0;               // 세르펜 p7 타이머 min(+0xc0, +0xd0)
pub const HP_S_P7_B: usize = 0xd0;
pub const HP_S_SUBTYPE: [u64; 9] = [0, 2, 0, 2, 1, 2, 2, 0, 0]; // 세르펜 code 2 서브타입(맵종류 JT 0x33e580c: 0/2/7/8→0 · 1/3→[0x33e8d81]=2 · 4→[0x33e8e60]=1 · 5→[0x33e9909]=2 · 6→2)
// ── action_score 기저 스코어러(0xd57540) — RE 2026-09-06 17:10 (REPORT RE\2026-09-06_action_score-기저스코어러-…)
pub const P5_MEMO_KEY: usize = 0x928;            // sim+0x928 메모 키(틱 캐시 엔트리)
pub const P5_CHAMP_DATA: usize = 0x510;          // sim+0x510/0x518 = Box<dyn Champion>(data, vt): vt+0x20 kind(u32 getter) · vt+0x28 &Vec<i32> tags · vt+0x30 (out,obj)
pub const P5_CHAMP_VT: usize = 0x518;
pub const AS_BB_REC: usize = 0x918;              // bb+0x918 내 Record R
pub const AS_BB_988: usize = 0x988;              // bb 위험 관련 i64 3종
pub const AS_BB_998: usize = 0x998;
pub const AS_BB_9B0: usize = 0x9b0;
pub const AS_BB_RECB_PTR: usize = 0x14d8;        // bb+0x14d8 적 Record Vec ptr / +0x14f0 len · stride 0xd8
pub const AS_BB_RECB_LEN: usize = 0x14f0;
pub const AS_BB_1500: usize = 0x1500;            // u8 하드리젝트 스위치
pub const AS_REC_STRIDE: usize = 0xd8;
pub const AS_REC_THR_PTR: usize = 0x18;          // Record 위협 리스트 ptr / +0x30 len · stride 0x18 {src, delay, amount}
pub const AS_REC_THR_LEN: usize = 0x30;
pub const AS_REC_AGENT_H: usize = 0x58;          // Record +0x58 에이전트 핸들(0xd31bb0 키)
pub const AS_REC_80: usize = 0x80;               // Record +0x80 가치 가산
pub const AS_REC_ENT_H: usize = 0x130;           // Record +0x130 엔티티 핸들(타깃 매칭)
pub const SA_PAYLOAD8: usize = 0x8;              // SmallAction 페이로드 핸들(cat2/6~9)
pub const SA_PAYLOAD60: usize = 0x60;            // cat4 Trace 핸들
pub const SA_GOAL20: usize = 0x20;               // cat2 goal i64
pub const SA_FLAG94: usize = 0x94;               // cat4 플래그 u8
pub const CFG_TICK_CAP: usize = 0x13f8;          // cfg+0x13f8 틱 상한(피해 추정 게이트 tick < cap)
pub const ENT_IMMOBILE: usize = 0x488;           // u8 이동불가(0.5.3 +0x470)
pub const ENT_B8: usize = 0xb8;                  // 슬롯0 잔여쿨(추정) · +0xc0 슬롯1
pub const ENT_C0: usize = 0xc0;
pub const ENT_SLOT3: usize = 0x538;              // 궁 슬롯(lv≥5) · ZERO desc 0x33daf38
pub const X_SIM_TABLE: usize = 0x230;            // X+0x230+side*0x28+i*8 sim 포인터표(0xd31bb0)
pub const LANE_ROLE_BYTE: usize = 0xf8;          // lanes+side*0x2e8 + role*32 + 0xf8 역할 바이트
pub const AS_MULT0: [i64; 5] = [40, 75, 100, 200, 300];  // cat0 수적우세 배율(diff ≤−2 … ≥+2) — 표 0x33dbea0 + imm
pub const AS_MULT4: [i64; 5] = [30, 60, 80, 150, 200];   // cat4 배율 — 표 0x33dbeb8 + imm
pub const AS_ZERO_DESC: usize = 0x33daf38;       // ZERO 스킬 desc(0x38B, +0x30 = −1)
// ── base_score(0xd57540) 바이트패치 사이트 24곳(aiport sites 실측 2026-09-06 18:05) — 포팅은 live_imm* 로 읽는다(즉치 RVA = 사이트+off)
pub const SITE_AS_BIT100: usize = 0xd57a2e + 1;      // 4B 256 = 0x100 (dn_reach 비트 마스크)
pub const SITE_AS_IDX7: usize = 0xd57fe9 + 3;        // 1B 7
pub const SITE_AS_THR_A: usize = 0xd580a3 + 2;       // 4B 9999 (cat4 D 위협 임계)
pub const SITE_AS_MARGIN_A: usize = 0xd583c6 + 3;    // 4B 30000 (cat4 S 사거리 여유)
pub const SITE_AS_THR_B: usize = 0xd585f6 + 2;       // 4B 9999 (cat0 D)
pub const SITE_AS_M0_HI: usize = 0xd58813 + 2;       // 4B 300 (cat0 diff>1)
pub const SITE_AS_M0_LO: usize = 0xd58823 + 2;       // 4B 40  (cat0 diff<-1)
pub const SITE_AS_SHR4: usize = 0xd5885a + 3;        // 1B 2   (t1 /4 = sar 2)
pub const SITE_AS_SHR800: usize = 0xd58868 + 3;      // 1B 9   (mult*t2 /800 = 매직 a3d7 + sar 9)
pub const SITE_AS_MINUS2: usize = 0xd58873 + 3;      // 1B 254 (= -2)
pub const SITE_AS_PCT_C2A: usize = 0xd589b1 + 3;     // 1B 100 (cat2 gain: dmg*100)
pub const SITE_AS_PCT_C2B: usize = 0xd589b5 + 2;     // 4B 100 (cat2 gain 상한)
pub const SITE_AS_THR_C: usize = 0xd58aff + 2;       // 4B 9999 (cat4 record V)
pub const SITE_AS_MARGIN_B: usize = 0xd58dc0 + 3;    // 4B 30000 (cat4 E2 사거리 여유)
pub const SITE_AS_AL_D2: usize = 0xd592a1 + 2;       // 8B 100000^2+1 (cat4 아군 반경)
pub const SITE_AS_M4_HI: usize = 0xd59427 + 1;       // 4B 200 (cat4 diff>1)
pub const SITE_AS_M4_LO: usize = 0xd59436 + 1;       // 4B 30  (cat4 diff<-1)
pub const SITE_AS_PCT_XA: usize = 0xd594da + 3;      // 1B 100 (cat4 X: dmg*100)
pub const SITE_AS_PCT_XB: usize = 0xd594de + 1;      // 4B 100 (cat4 X 상한)
pub const SITE_AS_PCT_XC: usize = 0xd59509 + 3;      // 1B 100 (cat4 X 2번째)
pub const SITE_AS_PCT_XD: usize = 0xd5950d + 1;      // 4B 100
pub const SITE_AS_950: usize = 0xd59529 + 8;         // 4B 950 (kind3 게이트)
pub const SITE_AS_NEAR_D2: usize = 0xd595e5 + 2;     // 8B 100000^2+1 (cat0 보너스 근접)
pub const SITE_AS_BONUS: usize = 0xd595f2 + 1;       // 4B 10 (cat0 보너스)
pub const EFF28_SUM_20_18: usize = 0x1146bb0;   // 자식 Vec [s+0x20]/[s+0x28] stride 0x18 의 (p,m) 합 — base_score 리플레이 신규(x101,428)
pub const EFFA0_CONST1_BARD2: usize = 0x17cd2e0;  // 상수 생성(type 1, vamp 0) bard_skill2 — x852
pub const EFF28_BASE_AD: usize = 0x16a61d0;      // p = [s] + [s+8]*stats[0]/100 ; m = 0 — base_score 리플레이 신규(x105,458)
pub const EFF38_SUM_20_18: usize = 0x1146ca0;   // 자식 Vec [s+0x20]/[s+0x28] stride 0x18 의 +0x38 합(0x1146bb0 형제) — x107,186
pub const ENT_EFFS_PTR: usize = 0x2f8;            // 엔티티 dyn 효과 리스트 (data,vt) ptr / +0x300 len · stride 0x10 (0x16a8c10·0xeba9b0 vt+0x48/+0x80/+0x88/+0x90)
pub const ENT_EFFS_LEN: usize = 0x300;
// ── 스킬 dyn Effect vt+0x60/+0x68 (타깃 술어) 구현체 — fight_check RE §3-d + 디컴 2026-09-06 20:02
pub const EFF60_PAIR_NONZERO: usize = 0x12a7320;     // +0x60 잎: [p+0x18]!=0 && [p+0x10]!=0
pub const EFF60_ANY_CHILD: usize = 0x12a6d30;        // +0x60 합성: 자식 (d,vt)[len@p+0x10] @p+8 의 vt+0x60 any
pub const EFF68_NONZERO18: usize = 0x12a5ad0;        // +0x68 잎: [p+0x18]!=0
pub const EFF68_ANY_CHILD: usize = 0x12a6cd0;        // +0x68 합성: 자식 vt+0x68 any
pub const EFF_TRUE: usize = 0xadec0;                 // 항상 1 (mov al,1; ret)
pub const WMAP_GRID2: usize = 0x1c98;                // wmap 두 번째 30×30 표(+0x78 + 30*0xf0) — 0xcaff00 접근점 게이트

/// ★게임 리스트를 도는 모든 루프의 공통 상한. 게임에는 이런 상한이 **없다** — 재현이 폭주/무한루프를
///   피하려고 임의로 둔 것이고, 그 임의값이 세 번이나 조용한 오답을 만들었다:
///   ①`eff_bool` depth 8 → 판당 21,642 NA ②`slot_88` depth 6 → 28,456 NA
///   ③`buff_lookup` 순회 128 → Utsuho 가 평타마다 Permanent 버프를 dedupe 없이 append 해
///     버프 Vec 이 수백 개가 되고 찾는 이름이 128 뒤에 놓였다(2026-09-07 RE).
///   ⟹ 상한은 **한 곳에서** 관리하고, 실제 게임 데이터가 닿을 수 없을 만큼 크게 잡는다.
pub const CAP_ITER: u64 = 65536;
