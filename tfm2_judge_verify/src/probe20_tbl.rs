//! probe20_tbl.rs — **자동생성**(`MIG\probe20.py`). 손으로 고치지 말 것.
//!   명세 20함수(`MIG\_spec\specs20_v3.json`)의 **발화수 전용** 프로브 표.
//!   ★목적 = `ev1`(런타임 DIFF=0) 측정의 **선행조건** 확인 — 「발화 0 = 검증 표본 불성립」.
//!   스텁은 레지스터·스택·인자를 안 건드리므로 sret·페어반환·5인자+ 도 안전하다.
#![allow(dead_code)]
pub struct P20 { pub idx: u8, pub rva: usize, pub len: u8, pub prolog: &'static [u8],
                 pub name: &'static str, pub module: &'static str, pub ins: u32 }
pub static PROBES20: &[P20] = &[
    P20 { idx: 0, rva: 0xd354c0, len: 12, prolog: &[0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x55, 0x53], name: "ult", module: "abstract_input", ins: 269 },
    P20 { idx: 1, rva: 0xd5ba80, len: 12, prolog: &[0x56, 0x48, 0x83, 0xec, 0x30, 0x48, 0x8b, 0x8a, 0x30, 0x09, 0x00, 0x00], name: "calculate_jungle_action_score", module: "action_score", ins: 93 },
    P20 { idx: 3, rva: 0xe01c40, len: 12, prolog: &[0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x53], name: "defensive_crisis", module: "?", ins: 0 },
    P20 { idx: 4, rva: 0xd3cfa0, len: 12, prolog: &[0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x53], name: "handle_line_defense", module: "defense_nexus", ins: 324 },
    P20 { idx: 5, rva: 0xe59190, len: 12, prolog: &[0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x53], name: "v50_fold_dive_episode", module: "?", ins: 0 },
    P20 { idx: 6, rva: 0xe657a0, len: 12, prolog: &[0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x55, 0x53], name: "v2_response_retreat_stance", module: "engage", ins: 190 },
    P20 { idx: 7, rva: 0xccc010, len: 12, prolog: &[0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x55, 0x53], name: "sub_plan", module: "?", ins: 0 },
    P20 { idx: 8, rva: 0xdefa20, len: 12, prolog: &[0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x55, 0x53], name: "is_end", module: "hunt_and_poke", ins: 165 },
    P20 { idx: 9, rva: 0xebd570, len: 12, prolog: &[0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x55, 0x53], name: "check_favorable_engage_formation", module: "fight_check", ins: 781 },
    P20 { idx: 10, rva: 0xe0c560, len: 12, prolog: &[0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x55, 0x53], name: "should_end_object_finish_kill_priority_battle", module: "fight_model", ins: 232 },
    P20 { idx: 11, rva: 0xe4b5d0, len: 12, prolog: &[0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x53], name: "v3_fall_back_to_passive", module: "handler", ins: 156 },
    P20 { idx: 12, rva: 0xe595b0, len: 12, prolog: &[0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x53], name: "handle_chat", module: "chat", ins: 198 },
    P20 { idx: 14, rva: 0xdb90f0, len: 17, prolog: &[0x56, 0x57, 0x53, 0x48, 0x83, 0xec, 0x20, 0x48, 0x89, 0xce, 0x49, 0x8b, 0x89, 0x30, 0x09, 0x00, 0x00], name: "update", module: "ganker", ins: 200 },
    P20 { idx: 15, rva: 0xe5c1f0, len: 12, prolog: &[0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x53], name: "single_try_engage", module: "modes", ins: 348 },
    P20 { idx: 16, rva: 0xe0daa0, len: 12, prolog: &[0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x55, 0x53], name: "max_range_nearly_can_use", module: "battle", ins: 303 },
    P20 { idx: 18, rva: 0xdce220, len: 12, prolog: &[0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x55, 0x53], name: "v3_epicops_buff_window", module: "epic", ins: 195 },
    P20 { idx: 19, rva: 0xd40b20, len: 12, prolog: &[0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x53], name: "best_jungle_goal", module: "passive_jungle", ins: 228 },
    P20 { idx: 20, rva: 0xcaf9f0, len: 14, prolog: &[0x41, 0x57, 0x41, 0x56, 0x56, 0x57, 0x53, 0x48, 0x83, 0xec, 0x50, 0x48, 0x89, 0xce], name: "BigPlan::sub_plan", module: "plan_legacy/types", ins: 0 },
];

/// ★**호출부 프로브** — 진입부를 못 건드리는 함수용(#13). `call rel32`(5B)의 rel32 만
///   우리 스텁으로 돌린다 ⟹ **원 함수의 명령을 하나도 옮기지 않는다**(진입부 방식보다 안전).
///   `sites` = 그 함수를 부르는 `E8 rel32` 사이트 **전수**(생성기가 .text 전역 스캔으로 검산.
///   절대주소 8B 리터럴 0곳 = vtable·함수포인터 경유 간접호출 없음도 같이 검산했다).
///   ⚠`E8 rel32` 는 ±2GB ⟹ 스텁을 exe 근처에 할당해야 한다(`probe.rs::alloc_near`,
///   근접 검산 실패 시 **설치하지 않는다**).
pub struct C20 { pub idx: u8, pub target_rva: usize, pub sites: &'static [usize],
                 pub labels: &'static [&'static str], pub name: &'static str,
                 pub module: &'static str, pub ins: u32, pub why: &'static str }
pub static CALLSITES20: &[C20] = &[
    C20 { idx: 13, target_rva: 0xdf1c80, sites: &[0x00cafc27, 0x00df230e], labels: &["fn 0xcaf9f0+0x237 (#20 plan_legacy/types/BigPlan::sub_plan)", "fn 0xdf1f50+0x3be"], name: "target_bush_v30", module: "line_gank/cover", ins: 170, why: "진입부 12B 스틸 불가(je@+9 가 잘린다 · 명령 경계 0/4/7/9/15/21 · 21B 안에 분기 둘) — 호출부 리다이렉트로 전환. 호출부 2곳이 전수(간접호출 0곳 검산)" },
];

/// ★표에 **없는** 함수와 그 이유. 「빠진 것을 모르는 상태」를 만들지 않는다.
pub static MISSING20: &[(u8, &str, &str)] = &[
    (2, "sub_plan", "exe 에 독립 함수가 없다 — `BigPlan::sub_plan`(0xcaf9f0) 안 **점프테이블 idx14 arm**(`0xcafa57` = +0x67)으로 LTO 인라인(확정: attack_nexus.rs 의 panic::Location static 이 이미지 전역에 정확히 2개이고 각 .text 참조가 1개씩, 둘 다 0xcaf9f0 내부 · IR 호출 사이트 1곳 · .pdata 에 0xcafa57 엔트리 없음). 진입부가 없어 카운트 프로브 불가. ★호스트는 `AUX[20]` 으로 따로 계측한다 — ~~「#02 = 2,048만 회」~~ 는 **디스패처 호출수**였다. 개입/실측이 필요하면 = +0x67 에 midpin(첫 명령 7B 라 5B jmp 수용 · rip-상대/분기 없음 · 진입은 점프테이블 유일 · 직후 cmp 가 flags 재설정 ⟹ r11/r10/rdx/rsi/rbx/rax 보존 필요)"),
    (17, "new", "exe 에 독립 함수가 없다 — `update`(0xe4c5c0) 안으로 LTO 인라인(A 0xe4d901 · B 0xe4d964 · tail 0xe4da09, CSE 병합). 진입부가 없어 카운트 프로브 불가. ★단 **데스매치 전용**이라 MOBA 모드 검증에는 쓰이지 않는다(유저 확인 2026-09-12) ⟹ 20 중 19 로 측정 성립. 개입이 필요해지면 = update 상위 훅 또는 3구간 mid-function 핀(midpin.py/sitepin.py)"),
];
