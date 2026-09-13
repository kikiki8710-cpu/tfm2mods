//! sweep20.rs — **자동 생성**(`MIG\gensweep20.py`). 손으로 고치지 말 것.
//! ===========================================================================
//! 2단계 = **sweep**: 게임 원본 함수와 **내 dll 안 링크사본**(`extern crate game_ai;` = SDK rlib
//!   640함수 = 재현 정본)을 **같은 인자로 각각 호출해 반환을 비트동일 대조**한다.
//!   1단계(발화수)는 끝났다 — 판 종료 #1 확정치 = 설치 19/19 · 발화 17 · 미발화 2.
//!
//! ★★**sweep 과 진입부 프로브는 같은 함수에 공존할 수 없다**(둘 다 진입부 12B 를 패치한다).
//!   ⟹ 여기 실린 함수는 sweep 이 **프로브를 대체**한다(`probe::install_all` 이 `is_installed_spec()`
//!      로 건너뛴다). 나머지는 1단계 프로브를 그대로 유지한다 — 표기 = `[sweep]`/`[진입부]`/`[호출부]`.
//!
//! 켜는 법(★**기본 OFF**): `<게임>\mods\tfm2_judge_verify\sweep20_on.txt` 에 비트마스크를
//!   16진(`0x3`) 또는 10진으로 한 줄 적고 게임 재시작. 파일이 없거나 0 이면 **한 곳도 안 건다.**
//!     bit0 = 0x1  v3_epic_group_line               (1단계 발화 미측정(명세 밖 이분 대상))
//!               ⛔이 함수의 1단계 발화수는 **무효**다(그때 잰 주소가 다른 함수였다).
//!                 ⟹ 표본 수·성능 충격·켜는 순서를 이 값으로 판단하지 마라.
//!                 정정된 주소로 **1단계를 다시 돌린 뒤** 판단할 것.
//!     bit1 = 0x2  v3_epicops_repair_need           (1단계 발화 미측정(명세 밖 이분 대상))
//!               ⛔이 함수의 1단계 발화수는 **무효**다(그때 잰 주소가 다른 함수였다).
//!                 ⟹ 표본 수·성능 충격·켜는 순서를 이 값으로 판단하지 마라.
//!                 정정된 주소로 **1단계를 다시 돌린 뒤** 판단할 것.
//!     bit2 = 0x4  is_object_being_taken_by_enemy   (1단계 발화 미측정(명세 밖 이분 대상))
//!               ⛔이 함수의 1단계 발화수는 **무효**다(그때 잰 주소가 다른 함수였다).
//!                 ⟹ 표본 수·성능 충격·켜는 순서를 이 값으로 판단하지 마라.
//!                 정정된 주소로 **1단계를 다시 돌린 뒤** 판단할 것.
//!     bit3 = 0x8  v3_serpen_contest_clear_win      (1단계 발화 미측정(명세 밖 이분 대상))
//!               ⚠caveat: a3: &mut 게임 상태(1064B) — 게임 호출 전 상태로 되돌려 내 사본을 부르고 게임 상태로 복구한다. Vec 은 빈 것으로 바꿔 **게임 버퍼를 건드리지 않는다**
//!               ⛔이 함수의 1단계 발화수는 **무효**다(그때 잰 주소가 다른 함수였다).
//!                 ⟹ 표본 수·성능 충격·켜는 순서를 이 값으로 판단하지 마라.
//!                 정정된 주소로 **1단계를 다시 돌린 뒤** 판단할 것.
//!     bit4 = 0x10  resolve_fight_uncached           (1단계 발화 미측정(명세 밖 이분 대상))
//!               ⚠caveat: a3: 가변이지만 편입 — tcx `GameContext` = 디버그 싱크(인덱스 직접 허용)
//!               ⛔이 함수의 1단계 발화수는 **무효**다(그때 잰 주소가 다른 함수였다).
//!                 ⟹ 표본 수·성능 충격·켜는 순서를 이 값으로 판단하지 마라.
//!                 정정된 주소로 **1단계를 다시 돌린 뒤** 판단할 것.
//!     bit5 = 0x20  objective_is_damaged             (1단계 발화 2회)
//!               ⚠caveat: 호출부 리다이렉트로 설치한다(진입부 12B 불가) — 사이트 2곳은 1단계가 exe 로 검산한 것(전수·간접호출 0). 한 사이트라도 빠지면 표본은 **하한선**이다
//!     bit6 = 0x40  serpen_giveup_chat_reason        (1단계 발화 2,110회)
//!     bit7 = 0x80  v50_fold_dive_episode            (1단계 발화 3,403회)
//!               ⚠caveat: a0: &mut 게임 상태(6168B) — 게임 호출 전 상태로 되돌려 내 사본을 부르고 게임 상태로 복구한다. Vec 은 빈 것으로 바꿔 **게임 버퍼를 건드리지 않는다**
//!     bit8 = 0x100  v3_fall_back_to_passive          (1단계 발화 10,041회)
//!               ⚠caveat: a0: &mut 게임 상태(6168B) — 게임 호출 전 상태로 되돌려 내 사본을 부르고 게임 상태로 복구한다. Vec 은 빈 것으로 바꿔 **게임 버퍼를 건드리지 않는다** / a5: 가변이지만 편입 — tcx `&mut DebugFrameData(224B)` = 디버그 싱크 — IR 실측상 본문이 역참조하지 않고 넘기기만 한다(클로저 내부 쓰기는 미확인 ⟹ 중복 기록 가능 · 반환 대조엔 무관)
//!     bit9 = 0x200  defensive_crisis                 (1단계 발화 29,128회)
//!               ⚠caveat: a5: 가변이지만 편입 — tcx `&mut DebugFrameData(224B)` = 디버그 싱크 — IR 실측상 본문이 역참조하지 않고 넘기기만 한다(클로저 내부 쓰기는 미확인 ⟹ 중복 기록 가능 · 반환 대조엔 무관)
//!     bit10 = 0x400  resolve_join_stake               (1단계 발화 68,945회)
//!               ⚠caveat: a8: 가변이지만 편입 — tcx `DebugFrameData` = 디버그 싱크 — IR 실측상 본문이 역참조하지 않고 넘기기만 한다(클로저 내부 쓰기는 미확인 ⟹ 중복 기록 가능 · 반환 대조엔 무관)
//!     bit11 = 0x800  ult                              (1단계 발화 101,915회)
//!     bit12 = 0x1000  v2_response_retreat_stance       (1단계 발화 107,627회)
//!               ⚠caveat: a4: IR readonly 표기 없음 · tcx `&OperationData(24B)` = 공유참조(쓰기 관측 0) 근거로 편입
//!     bit13 = 0x2000  target_bush_v30                  (1단계 발화 132,642회)
//!               ⚠caveat: 호출부 리다이렉트로 설치한다(진입부 12B 불가) — 사이트 2곳은 1단계가 exe 로 검산한 것(전수·간접호출 0). 한 사이트라도 빠지면 표본은 **하한선**이다
//!     bit14 = 0x4000  update                           (1단계 발화 148,803회)
//!               ⚠caveat: a0: &mut 게임 상태(48B) — 게임 호출 전 상태로 되돌려 내 사본을 부르고 게임 상태로 복구한다. Vec 은 빈 것으로 바꿔 **게임 버퍼를 건드리지 않는다**
//!     bit15 = 0x8000  check_favorable_engage_formation (1단계 발화 428,286회)
//!     bit16 = 0x10000  v2_obj_restore_safe              (1단계 발화 506,982회)
//!               ⚠caveat: a4: IR readonly 표기 없음 · tcx `&OperationData(24B)` = 공유참조(쓰기 관측 0) 근거로 편입
//!     bit17 = 0x20000  i_am_chosen_defender             (1단계 발화 556,740회)
//!     bit18 = 0x40000  calculate_nexus_defense_count    (1단계 발화 560,347회)
//!     bit19 = 0x80000  resolve_fight_stake              (1단계 발화 656,947회)
//!               ⚠caveat: a13: 가변이지만 편입 — tcx `DebugFrameData` = 디버그 싱크 — IR 실측상 본문이 역참조하지 않고 넘기기만 한다(클로저 내부 쓰기는 미확인 ⟹ 중복 기록 가능 · 반환 대조엔 무관)
//!     bit20 = 0x100000  calculate_jungle_action_score    (1단계 발화 676,759회)
//!     bit21 = 0x200000  BattlePlan::with_runaway         (1단계 발화 709,478회)
//!     bit22 = 0x400000  v21_should_defer_support_target  (1단계 발화 866,135회)
//!     bit23 = 0x800000  nexus_under_direct_attack        (1단계 발화 1,081,789회)
//!     bit24 = 0x1000000  v23_objective_setup_pressure_lin (1단계 발화 1,149,517회)
//!     bit25 = 0x2000000  check_epic_giveup                (1단계 발화 1,223,027회)
//!     bit26 = 0x4000000  v23_healthy_allies_near_point    (1단계 발화 1,370,692회)
//!     bit27 = 0x8000000  is_end                           (1단계 발화 1,554,463회)
//!               ⚠caveat: a5: IR readonly 표기 없음 · tcx `&TeamPlan(1064B)` = 공유참조(쓰기 관측 0) 근거로 편입
//!     bit28 = 0x10000000  v3_epicops_buff_window           (1단계 발화 1,672,540회)
//!               ⚠caveat: a0: &mut 게임 상태(1064B) — 게임 호출 전 상태로 되돌려 내 사본을 부르고 게임 상태로 복구한다. Vec 은 빈 것으로 바꿔 **게임 버퍼를 건드리지 않는다**
//!     bit29 = 0x20000000  fight_participants               (1단계 발화 1,785,206회)
//!               ⚠caveat: a11: 가변이지만 편입 — tcx `DebugFrameData` = 디버그 싱크 — IR 실측상 본문이 역참조하지 않고 넘기기만 한다(클로저 내부 쓰기는 미확인 ⟹ 중복 기록 가능 · 반환 대조엔 무관)
//!     bit30 = 0x40000000  handle_chat                      (1단계 발화 1,808,301회)
//!               ⚠caveat: a0: &mut 게임 상태(6168B) — 게임 호출 전 상태로 되돌려 내 사본을 부르고 게임 상태로 복구한다. Vec 은 빈 것으로 바꿔 **게임 버퍼를 건드리지 않는다** / a8: 가변이지만 편입 — tcx `&mut DebugFrameData(224B)` = 디버그 싱크 — IR 실측상 본문이 역참조하지 않고 넘기기만 한다(클로저 내부 쓰기는 미확인 ⟹ 중복 기록 가능 · 반환 대조엔 무관)
//!     bit31 = 0x80000000  v25_objective_far_split_pressure (1단계 발화 2,105,201회)
//!     bit32 = 0x100000000  v25_scoped_battle_objective      (1단계 발화 2,777,647회)
//!     bit33 = 0x200000000  best_jungle_goal                 (1단계 발화 2,806,126회)
//!               ⚠caveat: a4: IR readonly 표기 없음 · tcx `&TeamPlan` = 공유참조(쓰기 관측 0) 근거로 편입 / a6: 가변이지만 편입 — tcx `&mut DebugFrameData(224B)` = 디버그 싱크 — IR 실측상 본문이 역참조하지 않고 넘기기만 한다(클로저 내부 쓰기는 미확인 ⟹ 중복 기록 가능 · 반환 대조엔 무관)
//!     bit34 = 0x400000000  has_line_defense_threat          (1단계 발화 2,818,679회)
//!     bit35 = 0x800000000  v23_recent_visible_enemies_near_ (1단계 발화 3,055,211회)
//!     bit36 = 0x1000000000  v3_epic_formation_role           (1단계 발화 3,144,357회)
//!     bit37 = 0x2000000000  v27_active_objective_discipline  (1단계 발화 3,158,080회)
//!     bit38 = 0x4000000000  v22_visible_enemy_is_runaway_thr (1단계 발화 5,660,210회)
//!     bit39 = 0x8000000000  PassiveLinePlan::sub_plan        (1단계 발화 6,570,780회)
//!               ⚠caveat: a6: 가변이지만 편입 — tcx `&mut DebugFrameData(224B)` = 디버그 싱크 — IR 실측상 본문이 역참조하지 않고 넘기기만 한다(클로저 내부 쓰기는 미확인 ⟹ 중복 기록 가능 · 반환 대조엔 무관) / a7: 가변이지만 편입 — tcx `DebugFrameData` = 디버그 싱크 — IR 실측상 본문이 역참조하지 않고 넘기기만 한다(클로저 내부 쓰기는 미확인 ⟹ 중복 기록 가능 · 반환 대조엔 무관)
//!     bit40 = 0x10000000000  check_press_tower_opportunity    (1단계 발화 6,807,462회)
//!     bit41 = 0x20000000000  max_range_nearly_can_use         (1단계 발화 6,940,762회)
//!     bit42 = 0x40000000000  can_recall                       (1단계 발화 7,564,211회)
//!     bit43 = 0x80000000000  check_epic_setup                 (1단계 발화 8,012,325회)
//!               ⚠caveat: a5: 가변이지만 편입 — tcx `&mut DebugFrameData(224B)` = 디버그 싱크 — IR 실측상 본문이 역참조하지 않고 넘기기만 한다(클로저 내부 쓰기는 미확인 ⟹ 중복 기록 가능 · 반환 대조엔 무관)
//!     bit44 = 0x100000000000  buy_item                         (1단계 발화 10,295,069회)
//!               ⚠호출수 10,295,069 = 대조가 그 함수의 실행을 **2배**로 만든다 → 프레임 지연 각오.
//!     bit45 = 0x200000000000  need_defense_nexus               (1단계 발화 10,990,018회)
//!               ⚠호출수 10,990,018 = 대조가 그 함수의 실행을 **2배**로 만든다 → 프레임 지연 각오.
//!     bit46 = 0x400000000000  upgrade_item                     (1단계 발화 11,686,232회)
//!               ⚠호출수 11,686,232 = 대조가 그 함수의 실행을 **2배**로 만든다 → 프레임 지연 각오.
//!     bit47 = 0x800000000000  EntityPositioningCache::new      (1단계 발화 18,749,665회)
//!               ⚠호출수 18,749,665 = 대조가 그 함수의 실행을 **2배**로 만든다 → 프레임 지연 각오.
//!     bit48 = 0x1000000000000  can_tower_focused_when_battle    (1단계 발화 18,769,014회)
//!               ⚠호출수 18,769,014 = 대조가 그 함수의 실행을 **2배**로 만든다 → 프레임 지연 각오.
//!     bit49 = 0x2000000000000  TeamPlan::update                 (1단계 발화 25,238,388회)
//!               ⚠caveat: a0: &mut 게임 상태(1064B) — 게임 호출 전 상태로 되돌려 내 사본을 부르고 게임 상태로 복구한다. Vec 은 빈 것으로 바꿔 **게임 버퍼를 건드리지 않는다**
//!               ⚠호출수 25,238,388 = 대조가 그 함수의 실행을 **2배**로 만든다 → 프레임 지연 각오.
//!     bit50 = 0x4000000000000  is_wave_priority_start_line      (1단계 발화 33,717,543회)
//!               ⚠호출수 33,717,543 = 대조가 그 함수의 실행을 **2배**로 만든다 → 프레임 지연 각오.
//!     bit51 = 0x8000000000000  handle_line_defense              (1단계 발화 54,660,390회)
//!               ⚠호출수 54,660,390 = 대조가 그 함수의 실행을 **2배**로 만든다 → 프레임 지연 각오.
//!     bit52 = 0x10000000000000  can_tower_focused                (1단계 발화 173,781,504회)
//!               ⚠호출수 173,781,504 = 대조가 그 함수의 실행을 **2배**로 만든다 → 프레임 지연 각오.
//!   ⚠전체를 한 번에 켜지 마라 — 선례(ai_adjust `fn_bisect` 비트2)에 **게임 즉사**가 있다.
//!     권장 순서 = 위에서 아래로(호출수 적은 것부터. 이유 = 사고 노출·성능 충격이 작다).
//!
//! ⚠거짓 DIFF 를 만드는 것들(설계상 처리한 것/못 한 것)
//!   ①`StdRng`(320B) 인자 = 게임 호출이 난수열을 소비하므로 **첫 호출 전에 떠서 두 번째 호출 전에
//!     되돌린다**. 안 되돌리면 내 사본이 다른 난수를 받아 DIFF 가 거짓으로 뜬다.
//!   ②가변 포인터 인자가 있는 함수는 **애초에 여기 안 들어온다**(EXCLUDED 참조) — 두 번 호출이
//!     상태를 두 번 바꾸고, 스냅샷 복원도 내부 Vec/Box 재할당에 무너진다.
//!   ③내 사본의 `thread_local` 메모 캐시는 **비어 있다**(게임 것과 별 인스턴스). 순수 메모면 같은
//!     값이 나오지만, 캐시가 이전 tick 값을 재사용하는 구조라면 DIFF 가 캐시 차이일 수 있다.
//!     ⟹ DIFF≠0 은 결론이 아니라 **시작**이다(첫 DIFF 의 인자·반환·몇 번째 호출을 남긴다).
//! ===========================================================================
#![allow(dead_code)]
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::Mutex;

// ── 내 링크사본 직접 호출(rlib 의 Rust 망글 심볼). llvm-nm 으로 `T`(외부노출) 확인분만 있다 ──
extern "Rust" {
    /// #58 v3_epic_group_line — fn(usize, &game_core::PlayerState, &game_core::OperationData) -> i8
    #[link_name = "_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old4epic18v3_epic_group_line"]
    fn my_58(a0: i64, a1: *const u8, a2: *const u8) -> u8;
    /// #59 v3_epicops_repair_need — fn(usize, *const AbstractGameWithCache /*널 허용*/, &Plan384) -> u8 /*range 0..3*/
    #[link_name = "_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old4epic22v3_epicops_repair_need"]
    fn my_59(a0: i64, a1: *const u8, a2: *const u8) -> u8;
    /// #60 is_object_being_taken_by_enemy — fn(&PlayerState, &OperationData, &GoalData248, &TeamPlan, bool) -> bool
    #[link_name = "_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy9team_plan17objective_helpers30is_object_being_taken_by_enemy"]
    fn my_60(a0: *const u8, a1: *const u8, a2: *const u8, a3: *const u8, a4: u8) -> bool;
    /// #61 v3_serpen_contest_clear_win — fn(usize /*version*/, &PlayerState, &OperationData, &mut TeamPlan) -> bool
    #[link_name = "_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old6serpen27v3_serpen_contest_clear_win"]
    fn my_61(a0: i64, a1: *const u8, a2: *const u8, a3: *const u8) -> bool;
    /// #62 resolve_fight_uncached — fn(usize, &AbstractGameWithCache, &GameContext, &Entity, *const &Entity, usize, *const &Entity, usize, i8, Option<&Entity>, usize, &mut DebugFrameData, usize, u
    #[link_name = "_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old11fight_model22resolve_fight_uncached"]
    fn my_62(a0: *const u8, a1: i64, a2: *const u8, a3: *const u8, a4: *const u8, a5: *const u8, a6: i64, a7: *const u8, a8: i64, a9: u8, a10: *const u8, a11: i64, a12: *const u8, a13: i64, a14: i64);
    /// #30 objective_is_damaged — fn(&game_core::OperationData, game_core::JungleType) -> bool
    #[link_name = "_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy9team_plan17objective_helpers20objective_is_damaged"]
    fn my_30(a0: *const u8, a1: u8) -> bool;
    /// #32 serpen_giveup_chat_reason — fn(&game_core::PlayerState, &game_core::OperationData) -> std::option::Option<game_core::SerpenGiveUpReason>
    #[link_name = "_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old6serpen25serpen_giveup_chat_reason"]
    fn my_32(a0: *const u8, a1: *const u8) -> u8;
    /// #05 v50_fold_dive_episode — fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, usize, bool, u8)
    #[link_name = "_RNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy7handler12dive_episodeNtB4_17LegacyPlanHandler21v50_fold_dive_episode"]
    fn my_5(a0: *const u8, a1: u8, a2: u8);
    /// #11 v3_fall_back_to_passive — fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_cor
    #[link_name = "_RNvMNtNtCshdEBA0ozCnw_7game_ai11plan_legacy7handlerNtB2_17LegacyPlanHandler23v3_fall_back_to_passive"]
    fn my_11(a0: *const u8, a1: i64, a2: *const u8, a3: *const u8, a4: *const u8, a5: *const u8);
    /// #03 defensive_crisis — fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, &mut game_core::DebugFrameData) -> game_ai::Defe
    #[link_name = "_RNvNtCshdEBA0ozCnw_7game_ai10buff_value16defensive_crisis"]
    fn my_3(a0: i64, a1: *const u8, a2: *const u8, a3: *const u8, a4: *const u8, a5: *const u8) -> P8;
    /// #49 resolve_join_stake — fn(usize, &mut rand::rngs::std::StdRng, &game_core::OperationData, &game_core::PlayerState, &game_core::Entity, &game_core::Entity, &game_ai::plan_legacy::team_
    #[link_name = "_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old11fight_model18resolve_join_stake"]
    fn my_49(a0: *const u8, a1: i64, a2: *const u8, a3: *const u8, a4: *const u8, a5: *const u8, a6: *const u8, a7: *const u8, a8: *const u8);
    /// #00 ult — fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_core::Entity) -> std::option
    #[link_name = "_RNvNtCshdEBA0ozCnw_7game_ai14abstract_input3ult"]
    fn my_0(a0: *const u8, a1: i64, a2: *const u8, a3: *const u8, a4: *const u8, a5: *const u8, a6: *const u8);
    /// #06 v2_response_retreat_stance — fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::D
    #[link_name = "_RNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy7handler6engageNtB4_17LegacyPlanHandler26v2_response_retreat_stance"]
    fn my_6(a0: i64, a1: *const u8, a2: *const u8, a3: *const u8, a4: *const u8) -> P64;
    /// #13 target_bush_v30 — fn(&game_ai::plan_legacy::old::LineGankCoverPlan, &game_core::PlayerState, &game_core::OperationData) -> usize
    #[link_name = "_RNvMNtNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old9line_gank5coverNtB2_17LineGankCoverPlan15target_bush_v30"]
    fn my_13(a0: u8, a1: i64, a2: i32, a3: *const u8, a4: *const u8) -> i64;
    /// #14 update — fn(&mut game_ai::plan_legacy::old::LineGankerPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, 
    #[link_name = "_RNvMNtNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old9line_gank6gankerNtB2_14LineGankerPlan6update"]
    fn my_14(a0: *const u8, a1: i64, a2: *const u8, a3: *const u8, a4: *const u8, a5: *const u8, a6: *const u8, a7: *const u8);
    /// #09 check_favorable_engage_formation — fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, u64) -> bool
    #[link_name = "_RNvNtCshdEBA0ozCnw_7game_ai11fight_check32check_favorable_engage_formation"]
    fn my_9(a0: i64, a1: *const u8, a2: *const u8, a3: *const u8, a4: i64) -> bool;
    /// #33 v2_obj_restore_safe — fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::D
    #[link_name = "_RNvMNtNtCshdEBA0ozCnw_7game_ai11plan_legacy7handlerNtB2_17LegacyPlanHandler19v2_obj_restore_safe"]
    fn my_33(a0: i64, a1: *const u8, a2: *const u8, a3: *const u8, a4: *const u8) -> bool;
    /// #37 i_am_chosen_defender — fn(&game_core::PlayerState, &game_core::OperationData, usize, &[usize]) -> bool
    #[link_name = "_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old13defense_nexus20i_am_chosen_defender"]
    fn my_37(a0: *const u8, a1: *const u8, a2: i64, a3: *const u8, a4: i64) -> bool;
    /// #36 calculate_nexus_defense_count — fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> usize
    #[link_name = "_RNvNtNtCshdEBA0ozCnw_7game_ai11plan_legacy7handler29calculate_nexus_defense_count"]
    fn my_36(a0: i64, a1: *const u8, a2: *const u8, a3: *const u8, a4: *const u8) -> i64;
    /// #35 resolve_fight_stake — fn(usize, &mut rand::rngs::std::StdRng, &game_core::OperationData, &game_core::PlayerState, &game_core::Entity, &[&game_core::Entity], &[&game_core::Entity], i8
    #[link_name = "_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old11fight_model19resolve_fight_stake"]
    fn my_35(a0: *const u8, a1: i64, a2: *const u8, a3: *const u8, a4: *const u8, a5: *const u8, a6: *const u8, a7: i64, a8: *const u8, a9: i64, a10: u8, a11: *const u8, a12: i64, a13: *const u8);
    /// #01 calculate_jungle_action_score — fn(&mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &std::boxed::Box<dyn [Binder { value: Trait(game_
    #[link_name = "_RNvNtCshdEBA0ozCnw_7game_ai12action_score29calculate_jungle_action_score"]
    fn my_1(a0: *const u8, a1: *const u8, a2: *const u8, a3: *const u8, a4: *const u8, a5: *const u8, a6: *const u8) -> i64;
    /// #43 BattlePlan::with_runaway — fn(&game_ai::plan_legacy::old::BattlePlan, usize, &game_core::OperationData) -> bool
    #[link_name = "_RNvMs0_NtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old6battleNtB5_10BattlePlan12with_runaway"]
    fn my_43(a0: *const u8, a1: i64, a2: *const u8) -> bool;
    /// #28 v21_should_defer_support_target — fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, &game_core::Entity, usize, usize, usize, usize, usize) -> bool
    #[link_name = "_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old11fight_model31v21_should_defer_support_target"]
    fn my_28(a0: i64, a1: *const u8, a2: *const u8, a3: *const u8, a4: *const u8, a5: i64, a6: i64, a7: i64, a8: i64, a9: i64) -> bool;
    /// #27 nexus_under_direct_attack — fn(&game_core::PlayerState, &game_core::OperationData) -> bool
    #[link_name = "_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old13defense_nexus25nexus_under_direct_attack"]
    fn my_27(a0: *const u8, a1: *const u8) -> bool;
    /// #26 v23_objective_setup_pressure_line — fn(&game_core::PlayerState, &game_core::OperationData, &[game_core::LineType]) -> std::option::Option<game_core::LineType>
    #[link_name = "_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy9team_plan17objective_helpers33v23_objective_setup_pressure_line"]
    fn my_26(a0: *const u8, a1: *const u8, a2: *const u8, a3: i64) -> u8;
    /// #50 check_epic_giveup — fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFra
    #[link_name = "_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old4epic17check_epic_giveup"]
    fn my_50(a0: i64, a1: *const u8, a2: *const u8, a3: *const u8, a4: *const u8, a5: *const u8) -> bool;
    /// #24 v23_healthy_allies_near_point — fn(&game_core::PlayerState, &game_core::OperationData, u64, u64, u64, usize) -> usize
    #[link_name = "_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy9team_plan17objective_helpers29v23_healthy_allies_near_point"]
    fn my_24(a0: *const u8, a1: *const u8, a2: i64, a3: i64, a4: i64, a5: i64) -> i64;
    /// #08 is_end — fn(&game_ai::plan_legacy::old::EpicHuntAndPokePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_lega
    #[link_name = "_RNvMNtNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old4epic13hunt_and_pokeNtB2_19EpicHuntAndPokePlan6is_end"]
    fn my_8(a0: *const u8, a1: i64, a2: *const u8, a3: *const u8, a4: *const u8, a5: *const u8, a6: *const u8) -> bool;
    /// #18 v3_epicops_buff_window — fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, 
    #[link_name = "_RNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old4epicNtNtB6_9team_plan8TeamPlan22v3_epicops_buff_window"]
    fn my_18(a0: *const u8, a1: i64, a2: *const u8, a3: *const u8, a4: *const u8, a5: *const u8, a6: *const u8) -> bool;
    /// #41 fight_participants — fn(usize, &mut rand::rngs::std::StdRng, &game_core::OperationData, &game_core::PlayerState, &game_core::Entity, &[&game_core::Entity], &[&game_core::Entity], &g
    #[link_name = "_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old11fight_model18fight_participants"]
    fn my_41(a0: *const u8, a1: i64, a2: *const u8, a3: *const u8, a4: *const u8, a5: *const u8, a6: *const u8, a7: i64, a8: *const u8, a9: i64, a10: *const u8, a11: *const u8);
    /// #12 handle_chat — fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, game_core::Po
    #[link_name = "_RNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy7handler4chatNtB4_17LegacyPlanHandler11handle_chat"]
    fn my_12(a0: *const u8, a1: i64, a2: *const u8, a3: *const u8, a4: *const u8, a5: i32, a6: *const u8, a7: u8, a8: *const u8);
    /// #52 v25_objective_far_split_pressure — fn(&game_core::PlayerState, &game_core::OperationData, game_core::JungleType) -> bool
    #[link_name = "_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy9team_plan17objective_helpers32v25_objective_far_split_pressure"]
    fn my_52(a0: *const u8, a1: *const u8, a2: u8) -> bool;
    /// #48 v25_scoped_battle_objective — fn(usize, std::option::Option<game_ai::plan_legacy::team_plan::MainObjective>, &game_core::PlayerState, &game_core::OperationData, std::option::Option<usize>) -
    #[link_name = "_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old11fight_model27v25_scoped_battle_objective"]
    fn my_48(a0: i64, a1: u32, a2: *const u8, a3: *const u8, a4: i64, a5: i64) -> u32;
    /// #19 best_jungle_goal — fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan, std::option::Option<game
    #[link_name = "_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old14passive_jungle16best_jungle_goal"]
    fn my_19(a0: i64, a1: *const u8, a2: *const u8, a3: *const u8, a4: *const u8, a5: u8, a6: *const u8) -> u8;
    /// #34 has_line_defense_threat — fn(&game_core::PlayerState, &game_core::OperationData, game_core::LineType, usize) -> bool
    #[link_name = "_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old13defense_nexus23has_line_defense_threat"]
    fn my_34(a0: *const u8, a1: *const u8, a2: u8, a3: i64) -> bool;
    /// #29 v23_recent_visible_enemies_near_point — fn(&game_core::PlayerState, &game_core::OperationData, u64, u64, u64, usize) -> usize
    #[link_name = "_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy9team_plan17objective_helpers37v23_recent_visible_enemies_near_point"]
    fn my_29(a0: *const u8, a1: *const u8, a2: i64, a3: i64, a4: i64, a5: i64) -> i64;
    /// #31 v3_epic_formation_role — fn(game_core::MorgardUseStrategy, game_core::Position, &game_core::PlayerState, &game_core::OperationData) -> std::option::Option<game_ai::plan_legacy::old::V3E
    #[link_name = "_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old4epic22v3_epic_formation_role"]
    fn my_31(a0: i64, a1: i32, a2: *const u8, a3: *const u8) -> P8;
    /// #20 v27_active_objective_discipline — fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::OperationData, game_core::JungleType) -> std::option::Option<game_ai::plan_legacy::team_plan::
    #[link_name = "_RNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy9team_plan20objective_disciplineNtB4_8TeamPlan31v27_active_objective_discipline"]
    fn my_20(a0: *const u8, a1: *const u8, a2: i64, a3: *const u8, a4: u8);
    /// #25 v22_visible_enemy_is_runaway_threat — fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, &game_core::Entity) -> bool
    #[link_name = "_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old11fight_model35v22_visible_enemy_is_runaway_threat"]
    fn my_25(a0: i64, a1: *const u8, a2: *const u8, a3: *const u8, a4: *const u8) -> bool;
    /// #56 PassiveLinePlan::sub_plan — fn(&game_ai::plan_legacy::old::PassiveLinePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::
    #[link_name = "_RNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old12passive_lineNtB2_15PassiveLinePlan8sub_plan"]
    fn my_56(a0: *const u8, a1: *const u8, a2: i64, a3: *const u8, a4: *const u8, a5: *const u8, a6: *const u8, a7: *const u8);
    /// #51 check_press_tower_opportunity — fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan, std::option::Option<game
    #[link_name = "_RNvNtNtCshdEBA0ozCnw_7game_ai11plan_legacy9team_plan29check_press_tower_opportunity"]
    fn my_51(a0: i64, a1: *const u8, a2: *const u8, a3: *const u8, a4: *const u8, a5: u8, a6: *const u8) -> u8;
    /// #16 max_range_nearly_can_use — fn(&game_core::Entity, &game_core::Entity, usize) -> u64
    #[link_name = "_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old6battle24max_range_nearly_can_use"]
    fn my_16(a0: *const u8, a1: *const u8, a2: i64) -> i64;
    /// #45 can_recall — fn(&mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bool
    #[link_name = "_RNvNtCshdEBA0ozCnw_7game_ai5utils10can_recall"]
    fn my_45(a0: *const u8, a1: *const u8, a2: *const u8) -> bool;
    /// #53 check_epic_setup — fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFra
    #[link_name = "_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old4epic16check_epic_setup"]
    fn my_53(a0: i64, a1: *const u8, a2: *const u8, a3: *const u8, a4: *const u8, a5: *const u8) -> bool;
    /// #23 buy_item — fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::GameCon
    #[link_name = "_RNvCshdEBA0ozCnw_7game_ai8buy_item"]
    fn my_23(a0: i64, a1: *const u8, a2: *const u8, a3: *const u8, a4: *const u8, a5: *const u8) -> P64;
    /// #46 need_defense_nexus — fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> bool
    #[link_name = "_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old13defense_nexus18need_defense_nexus"]
    fn my_46(a0: i64, a1: *const u8, a2: *const u8, a3: *const u8, a4: *const u8) -> bool;
    /// #21 upgrade_item — fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::GameCon
    #[link_name = "_RNvCshdEBA0ozCnw_7game_ai12upgrade_item"]
    fn my_21(a0: *const u8, a1: i64, a2: *const u8, a3: *const u8, a4: *const u8, a5: *const u8, a6: *const u8);
    /// #55 EntityPositioningCache::new — fn(usize, &game_core::Entity, &game_core::Entity, &game_core::PlayerState, &game_core::PlayerState, &game_core::ChampionCache, &game_core::ChampionCache, bool) 
    #[link_name = "_RNvMNtCshdEBA0ozCnw_7game_ai15score_parameterNtB2_22EntityPositioningCache3new"]
    fn my_55(a0: *const u8, a1: i64, a2: *const u8, a3: *const u8, a4: *const u8, a5: *const u8, a6: *const u8, a7: *const u8, a8: u8);
    /// #38 can_tower_focused_when_battle — fn(&game_core::GameContext, &game_core::AbstractGameWithCache, &game_core::PlayerState, u64, u64, u64) -> bool
    #[link_name = "_RNvNtCshdEBA0ozCnw_7game_ai16tower_discipline29can_tower_focused_when_battle"]
    fn my_38(a0: *const u8, a1: *const u8, a2: *const u8, a3: i64, a4: i64, a5: i64) -> bool;
    /// #54 TeamPlan::update — fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::Debu
    #[link_name = "_RNvMs1_NtNtCshdEBA0ozCnw_7game_ai11plan_legacy9team_planNtB5_8TeamPlan6update"]
    fn my_54(a0: *const u8, a1: i64, a2: *const u8, a3: *const u8, a4: *const u8, a5: *const u8);
    /// #39 is_wave_priority_start_line — fn(&game_core::PlayerState, &game_core::OperationData, game_core::LineType) -> bool
    #[link_name = "_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy9team_plan17objective_helpers27is_wave_priority_start_line"]
    fn my_39(a0: *const u8, a1: *const u8, a2: u8) -> bool;
    /// #04 handle_line_defense — fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, game_core::LineType, &mut game_core::DebugFrameData) -> bool
    #[link_name = "_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old13defense_nexus19handle_line_defense"]
    fn my_4(a0: i64, a1: *const u8, a2: *const u8, a3: *const u8, a4: u8, a5: *const u8) -> bool;
    /// #22 can_tower_focused — fn(&game_core::GameContext, &game_core::AbstractGameWithCache, &game_core::PlayerState, u64, u64) -> bool
    #[link_name = "_RNvNtCshdEBA0ozCnw_7game_ai16tower_discipline17can_tower_focused"]
    fn my_22(a0: *const u8, a1: *const u8, a2: *const u8, a3: i64, a4: i64) -> bool;
}

/// ★이분용 스위치 — 1 이면 내 사본이 push 한 요소의 String 을 해제하지 않는다(누수 감수).
const BISECT_NO_STRFREE: u8 = 0;
pub struct Slot {
    pub bit: u8, pub idx: u8, pub name: &'static str, pub src: &'static str,
    pub rva: usize, pub prolog: &'static [u8], pub stage1: u64, pub rng: &'static [u8],
    pub caveat: &'static str,
    /// 비어 있으면 **진입부 훅**, 차 있으면 **호출부 리다이렉트**(진입부 12B 를 못 빼는 함수).
    pub sites: &'static [usize],
    pub calls: AtomicU64, pub cmp: AtomicU64, pub diff: AtomicU64, pub pan: AtomicU64,
    /// ★내 Vec 사본 버퍼에 안 들어가 **표본에서 뺀** 호출 수. 조용한 누락을 막으려 센다.
    pub skip: AtomicU64,
    pub base_cmp: AtomicU64, pub base_diff: AtomicU64, pub orig: AtomicUsize,
}
macro_rules! sl { ($b:expr, $i:expr, $n:expr, $s:expr, $r:expr, $p:expr, $c:expr, $g:expr, $v:expr, $st:expr) => {
    Slot { bit: $b, idx: $i, name: $n, src: $s, rva: $r, prolog: $p, stage1: $c, rng: $g, caveat: $v, sites: $st,
           calls: AtomicU64::new(0), cmp: AtomicU64::new(0), diff: AtomicU64::new(0),
           pan: AtomicU64::new(0), skip: AtomicU64::new(0), base_cmp: AtomicU64::new(0), base_diff: AtomicU64::new(0),
           orig: AtomicUsize::new(0) } } }
pub static S: [Slot; 53] = [
    sl!(0, 58, "v3_epic_group_line", "game-ai\\src\\plan_legacy\\old\\epic.rs", 0xdea4a0, &[0x56, 0x57, 0x53, 0x48, 0x83, 0xec, 0x20, 0x83, 0xe9, 0x05, 0xb8, 0x02, 0x00, 0x00, 0x00], u64::MAX, &[], "", &[]),
    sl!(1, 59, "v3_epicops_repair_need", "game-ai\\src\\plan_legacy\\old\\epic.rs", 0xdeaa70, &[0x56, 0x57, 0x53, 0x48, 0x83, 0xec, 0x50, 0x0f, 0x29, 0x74, 0x24, 0x40], u64::MAX, &[], "", &[]),
    sl!(2, 60, "is_object_being_taken_by_enemy", "game-ai\\src\\plan_legacy\\team_plan\\objective_helpers.rs", 0xec9bf0, &[0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x55, 0x53], u64::MAX, &[], "", &[]),
    sl!(3, 61, "v3_serpen_contest_clear_win", "game-ai\\src\\plan_legacy\\old\\serpen.rs", 0xd666c0, &[0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x53], u64::MAX, &[], "a3: &mut 게임 상태(1064B) — 게임 호출 전 상태로 되돌려 내 사본을 부르고 게임 상태로 복구한다. Vec 은 빈 것으로 바꿔 **게임 버퍼를 건드리지 않는다**", &[]),
    sl!(4, 62, "resolve_fight_uncached", "game-ai\\src\\plan_legacy\\old\\fight_model.rs", 0xe083c0, &[0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x55, 0x53], u64::MAX, &[], "a3: 가변이지만 편입 — tcx `GameContext` = 디버그 싱크(인덱스 직접 허용)", &[]),
    sl!(5, 30, "objective_is_damaged", "game-ai\\src\\plan_legacy\\team_plan\\objective_helpers.rs", 0xec8af0, &[], 2, &[], "호출부 리다이렉트로 설치한다(진입부 12B 불가) — 사이트 2곳은 1단계가 exe 로 검산한 것(전수·간접호출 0). 한 사이트라도 빠지면 표본은 **하한선**이다", &[0xddbd6d, 0xdddf0d]),
    sl!(6, 32, "serpen_giveup_chat_reason", "game-ai\\src\\plan_legacy\\old\\serpen.rs", 0xd665e0, &[0x41, 0x56, 0x56, 0x57, 0x53, 0x48, 0x83, 0xec, 0x28, 0x48, 0x8b, 0x42, 0x08], 2110, &[], "", &[]),
    sl!(7, 5, "v50_fold_dive_episode", "game-ai\\src\\plan_legacy\\handler\\dive_episode.rs", 0xe59190, &[0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x53], 3403, &[], "a0: &mut 게임 상태(6168B) — 게임 호출 전 상태로 되돌려 내 사본을 부르고 게임 상태로 복구한다. Vec 은 빈 것으로 바꿔 **게임 버퍼를 건드리지 않는다**", &[]),
    sl!(8, 11, "v3_fall_back_to_passive", "game-ai\\src\\plan_legacy\\handler.rs", 0xe4b5d0, &[0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x53], 10041, &[2], "a0: &mut 게임 상태(6168B) — 게임 호출 전 상태로 되돌려 내 사본을 부르고 게임 상태로 복구한다. Vec 은 빈 것으로 바꿔 **게임 버퍼를 건드리지 않는다** / a5: 가변이지만 편입 — tcx `&mut DebugFrameData(224B)` = 디버그 싱크 — IR 실측상 본문이 역참조하지 않고 넘기기만 한다(클로저 내부 쓰기는 미확인 ⟹ 중복 기록 가능 · 반환 대조엔 무관)", &[]),
    sl!(9, 3, "defensive_crisis", "game-ai\\src\\buff_value.rs", 0xe01c40, &[0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x53], 29128, &[1], "a5: 가변이지만 편입 — tcx `&mut DebugFrameData(224B)` = 디버그 싱크 — IR 실측상 본문이 역참조하지 않고 넘기기만 한다(클로저 내부 쓰기는 미확인 ⟹ 중복 기록 가능 · 반환 대조엔 무관)", &[]),
    sl!(10, 49, "resolve_join_stake", "game-ai\\src\\plan_legacy\\old\\fight_model.rs", 0xe05e70, &[0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x53], 68945, &[2], "a8: 가변이지만 편입 — tcx `DebugFrameData` = 디버그 싱크 — IR 실측상 본문이 역참조하지 않고 넘기기만 한다(클로저 내부 쓰기는 미확인 ⟹ 중복 기록 가능 · 반환 대조엔 무관)", &[]),
    sl!(11, 0, "ult", "game-ai\\src\\abstract_input.rs", 0xd354c0, &[0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x55, 0x53], 101915, &[2], "", &[]),
    sl!(12, 6, "v2_response_retreat_stance", "game-ai\\src\\plan_legacy\\handler\\engage.rs", 0xe657a0, &[0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x55, 0x53], 107627, &[1], "a4: IR readonly 표기 없음 · tcx `&OperationData(24B)` = 공유참조(쓰기 관측 0) 근거로 편입", &[]),
    sl!(13, 13, "target_bush_v30", "game-ai\\src\\plan_legacy\\old\\line_gank\\cover.rs", 0xdf1c80, &[], 132642, &[], "호출부 리다이렉트로 설치한다(진입부 12B 불가) — 사이트 2곳은 1단계가 exe 로 검산한 것(전수·간접호출 0). 한 사이트라도 빠지면 표본은 **하한선**이다", &[0xcafc27, 0xdf230e]),
    sl!(14, 14, "update", "game-ai\\src\\plan_legacy\\old\\line_gank\\ganker.rs", 0xdb90f0, &[0x56, 0x57, 0x53, 0x48, 0x83, 0xec, 0x20, 0x48, 0x89, 0xce, 0x49, 0x8b, 0x89, 0x30, 0x09, 0x00, 0x00], 148803, &[2], "a0: &mut 게임 상태(48B) — 게임 호출 전 상태로 되돌려 내 사본을 부르고 게임 상태로 복구한다. Vec 은 빈 것으로 바꿔 **게임 버퍼를 건드리지 않는다**", &[]),
    sl!(15, 9, "check_favorable_engage_formation", "game-ai\\src\\fight_check.rs", 0xebd570, &[0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x55, 0x53], 428286, &[], "", &[]),
    sl!(16, 33, "v2_obj_restore_safe", "game-ai\\src\\plan_legacy\\handler.rs", 0xe4aec0, &[0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x53, 0x48, 0x81, 0xec, 0xd0, 0x00, 0x00, 0x00], 506982, &[1], "a4: IR readonly 표기 없음 · tcx `&OperationData(24B)` = 공유참조(쓰기 관측 0) 근거로 편입", &[]),
    sl!(17, 37, "i_am_chosen_defender", "game-ai\\src\\plan_legacy\\old\\defense_nexus.rs", 0xd3d560, &[0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x55, 0x53], 556740, &[], "", &[]),
    sl!(18, 36, "calculate_nexus_defense_count", "game-ai\\src\\plan_legacy\\handler.rs", 0xe6d000, &[0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x53], 560347, &[], "", &[]),
    sl!(19, 35, "resolve_fight_stake", "game-ai\\src\\plan_legacy\\old\\fight_model.rs", 0xe06df0, &[0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x53], 656947, &[2], "a13: 가변이지만 편입 — tcx `DebugFrameData` = 디버그 싱크 — IR 실측상 본문이 역참조하지 않고 넘기기만 한다(클로저 내부 쓰기는 미확인 ⟹ 중복 기록 가능 · 반환 대조엔 무관)", &[]),
    sl!(20, 1, "calculate_jungle_action_score", "game-ai\\src\\action_score.rs", 0xd5ba80, &[0x56, 0x48, 0x83, 0xec, 0x30, 0x48, 0x8b, 0x8a, 0x30, 0x09, 0x00, 0x00], 676759, &[0], "", &[]),
    sl!(21, 43, "BattlePlan::with_runaway", "game-ai\\src\\plan_legacy\\old\\battle.rs", 0xdfb220, &[0x56, 0x57, 0x48, 0x83, 0xec, 0x28, 0x48, 0x89, 0xce, 0x48, 0x83, 0x79, 0x40, 0x00], 709478, &[], "", &[]),
    sl!(22, 28, "v21_should_defer_support_target", "game-ai\\src\\plan_legacy\\old\\fight_model.rs", 0xe0bd60, &[0x41, 0x57, 0x41, 0x56, 0x41, 0x54, 0x56, 0x57, 0x53, 0x48, 0x83, 0xec, 0x28], 866135, &[], "", &[]),
    sl!(23, 27, "nexus_under_direct_attack", "game-ai\\src\\plan_legacy\\old\\defense_nexus.rs", 0xd3fa80, &[0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x53], 1081789, &[], "", &[]),
    sl!(24, 26, "v23_objective_setup_pressure_line", "game-ai\\src\\plan_legacy\\team_plan\\objective_helpers.rs", 0xeca9a0, &[0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x55, 0x53], 1149517, &[], "", &[]),
    sl!(25, 50, "check_epic_giveup", "game-ai\\src\\plan_legacy\\old\\epic.rs", 0xde81b0, &[0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x53], 1223027, &[], "", &[]),
    sl!(26, 24, "v23_healthy_allies_near_point", "game-ai\\src\\plan_legacy\\team_plan\\objective_helpers.rs", 0xec9840, &[0x56, 0x57, 0x53, 0x48, 0x83, 0xec, 0x20, 0x48, 0x8b, 0x89, 0x30, 0x09, 0x00, 0x00], 1370692, &[], "", &[]),
    sl!(27, 8, "is_end", "game-ai\\src\\plan_legacy\\old\\epic\\hunt_and_poke.rs", 0xdefa20, &[0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x55, 0x53], 1554463, &[2], "a5: IR readonly 표기 없음 · tcx `&TeamPlan(1064B)` = 공유참조(쓰기 관측 0) 근거로 편입", &[]),
    sl!(28, 18, "v3_epicops_buff_window", "game-ai\\src\\plan_legacy\\old\\epic.rs", 0xdce220, &[0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x55, 0x53], 1672540, &[], "a0: &mut 게임 상태(1064B) — 게임 호출 전 상태로 되돌려 내 사본을 부르고 게임 상태로 복구한다. Vec 은 빈 것으로 바꿔 **게임 버퍼를 건드리지 않는다**", &[]),
    sl!(29, 41, "fight_participants", "game-ai\\src\\plan_legacy\\old\\fight_model.rs", 0xe04f50, &[0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x53], 1785206, &[2], "a11: 가변이지만 편입 — tcx `DebugFrameData` = 디버그 싱크 — IR 실측상 본문이 역참조하지 않고 넘기기만 한다(클로저 내부 쓰기는 미확인 ⟹ 중복 기록 가능 · 반환 대조엔 무관)", &[]),
    sl!(30, 12, "handle_chat", "game-ai\\src\\plan_legacy\\handler\\chat.rs", 0xe595b0, &[0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x53], 1808301, &[2], "a0: &mut 게임 상태(6168B) — 게임 호출 전 상태로 되돌려 내 사본을 부르고 게임 상태로 복구한다. Vec 은 빈 것으로 바꿔 **게임 버퍼를 건드리지 않는다** / a8: 가변이지만 편입 — tcx `&mut DebugFrameData(224B)` = 디버그 싱크 — IR 실측상 본문이 역참조하지 않고 넘기기만 한다(클로저 내부 쓰기는 미확인 ⟹ 중복 기록 가능 · 반환 대조엔 무관)", &[]),
    sl!(31, 52, "v25_objective_far_split_pressure", "game-ai\\src\\plan_legacy\\team_plan\\objective_helpers.rs", 0xeca430, &[0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x55, 0x53], 2105201, &[], "", &[]),
    sl!(32, 48, "v25_scoped_battle_objective", "game-ai\\src\\plan_legacy\\old\\fight_model.rs", 0xe0b730, &[0x41, 0x57, 0x41, 0x56, 0x41, 0x54, 0x56, 0x57, 0x55, 0x53, 0x48, 0x83, 0xec, 0x60], 2777647, &[], "", &[]),
    sl!(33, 19, "best_jungle_goal", "game-ai\\src\\plan_legacy\\old\\passive_jungle.rs", 0xd40b20, &[0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x53], 2806126, &[1], "a4: IR readonly 표기 없음 · tcx `&TeamPlan` = 공유참조(쓰기 관측 0) 근거로 편입 / a6: 가변이지만 편입 — tcx `&mut DebugFrameData(224B)` = 디버그 싱크 — IR 실측상 본문이 역참조하지 않고 넘기기만 한다(클로저 내부 쓰기는 미확인 ⟹ 중복 기록 가능 · 반환 대조엔 무관)", &[]),
    sl!(34, 34, "has_line_defense_threat", "game-ai\\src\\plan_legacy\\old\\defense_nexus.rs", 0xd3e4b0, &[0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x54, 0x56, 0x57, 0x53, 0x48, 0x83, 0xec, 0x50], 2818679, &[], "", &[]),
    sl!(35, 29, "v23_recent_visible_enemies_near_point", "game-ai\\src\\plan_legacy\\team_plan\\objective_helpers.rs", 0xecacc0, &[0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x55, 0x53], 3055211, &[], "", &[]),
    sl!(36, 31, "v3_epic_formation_role", "game-ai\\src\\plan_legacy\\old\\epic.rs", 0xdea800, &[0x41, 0x56, 0x56, 0x57, 0x55, 0x53, 0x48, 0x83, 0xec, 0x20, 0x4c, 0x89, 0xcb], 3144357, &[], "", &[]),
    sl!(37, 20, "v27_active_objective_discipline", "game-ai\\src\\plan_legacy\\team_plan\\objective_discipline.rs", 0xdd50e0, &[0x41, 0x57, 0x41, 0x56, 0x56, 0x57, 0x55, 0x53, 0x48, 0x83, 0xec, 0x48], 3158080, &[], "", &[]),
    sl!(38, 25, "v22_visible_enemy_is_runaway_threat", "game-ai\\src\\plan_legacy\\old\\fight_model.rs", 0xe0c310, &[0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x55, 0x53], 5660210, &[], "", &[]),
    sl!(39, 56, "PassiveLinePlan::sub_plan", "game-ai\\src\\plan_legacy\\old\\passive_line.rs", 0xd2c5d0, &[0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x55, 0x53], 6570780, &[3], "a6: 가변이지만 편입 — tcx `&mut DebugFrameData(224B)` = 디버그 싱크 — IR 실측상 본문이 역참조하지 않고 넘기기만 한다(클로저 내부 쓰기는 미확인 ⟹ 중복 기록 가능 · 반환 대조엔 무관) / a7: 가변이지만 편입 — tcx `DebugFrameData` = 디버그 싱크 — IR 실측상 본문이 역참조하지 않고 넘기기만 한다(클로저 내부 쓰기는 미확인 ⟹ 중복 기록 가능 · 반환 대조엔 무관)", &[]),
    sl!(40, 51, "check_press_tower_opportunity", "game-ai\\src\\plan_legacy\\team_plan.rs", 0xde40c0, &[0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x55, 0x53], 6807462, &[], "", &[]),
    sl!(41, 16, "max_range_nearly_can_use", "game-ai\\src\\plan_legacy\\old\\battle.rs", 0xe0daa0, &[0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x55, 0x53], 6940762, &[], "", &[]),
    sl!(42, 45, "can_recall", "game-ai\\src\\utils.rs", 0xd36480, &[0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x55, 0x53], 7564211, &[0], "", &[]),
    sl!(43, 53, "check_epic_setup", "game-ai\\src\\plan_legacy\\old\\epic.rs", 0xde6ce0, &[0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x53], 8012325, &[], "a5: 가변이지만 편입 — tcx `&mut DebugFrameData(224B)` = 디버그 싱크 — IR 실측상 본문이 역참조하지 않고 넘기기만 한다(클로저 내부 쓰기는 미확인 ⟹ 중복 기록 가능 · 반환 대조엔 무관)", &[]),
    sl!(44, 23, "buy_item", "game-ai\\src\\lib.rs", 0xe7b640, &[0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x53], 10295069, &[1], "", &[]),
    sl!(45, 46, "need_defense_nexus", "game-ai\\src\\plan_legacy\\old\\defense_nexus.rs", 0xd3c700, &[0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x53], 10990018, &[1], "", &[]),
    sl!(46, 21, "upgrade_item", "game-ai\\src\\lib.rs", 0xe7a8c0, &[0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x53], 11686232, &[2], "", &[]),
    sl!(47, 55, "EntityPositioningCache::new", "game-ai\\src\\score_parameter.rs", 0xd815e0, &[0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x55, 0x53], 18749665, &[], "", &[]),
    sl!(48, 38, "can_tower_focused_when_battle", "game-ai\\src\\tower_discipline.rs", 0xd988d0, &[0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x55, 0x53], 18769014, &[], "", &[]),
    sl!(49, 54, "TeamPlan::update", "game-ai\\src\\plan_legacy\\team_plan.rs", 0xde0770, &[0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x55, 0x53], 25238388, &[2], "a0: &mut 게임 상태(1064B) — 게임 호출 전 상태로 되돌려 내 사본을 부르고 게임 상태로 복구한다. Vec 은 빈 것으로 바꿔 **게임 버퍼를 건드리지 않는다**", &[]),
    sl!(50, 39, "is_wave_priority_start_line", "game-ai\\src\\plan_legacy\\team_plan\\objective_helpers.rs", 0xec9400, &[0x41, 0x57, 0x41, 0x56, 0x41, 0x54, 0x56, 0x57, 0x55, 0x53, 0x48, 0x83, 0xec, 0x20], 33717543, &[], "", &[]),
    sl!(51, 4, "handle_line_defense", "game-ai\\src\\plan_legacy\\old\\defense_nexus.rs", 0xd3cfa0, &[0x55, 0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x53], 54660390, &[1], "", &[]),
    sl!(52, 22, "can_tower_focused", "game-ai\\src\\tower_discipline.rs", 0xd97300, &[0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x56, 0x57, 0x55, 0x53], 173781504, &[], "", &[]),
];

/// ★대조에서 **빠진** 명세 함수와 그 사유. 「빠진 것을 모르는 상태」를 만들지 않는다.
///   (idx, name, 1단계 발화수, 사유)
pub static EXCLUDED: &[(u8, &str, &str, &str)] = &[
    (2, "sub_plan", "?", "**인라인 — 독립 진입부가 없다**(진입부 detour·호출부 리다이렉트 방식 한정 불가): `BigPlan::sub_plan`(0xcaf9f0)에 인라인 — 독립 진입부가 없다 ⟹ pin02.rs midpin(**bit62** = 0x4000000000000000 · 09-13 bit19→62 이동)으로 대조(2026-09-13 DIFF 0)"),
    (7, "sub_plan", "0", "미발화(재측정 확정치 0회) — ★프로브는 유지(뜰 때까지 계속 본다)"),
    (10, "should_end_object_finish_kill_priority_battle", "0", "미발화(재측정 확정치 0회 — 정정된 주소 0xe0c560 에서) — ★프로브는 유지(뜰 때까지 계속 본다)"),
    (15, "single_try_engage", "0", "미발화(재측정 확정치 0회 — 정정된 주소 0xe5c1f0 에서) — ★프로브는 유지(뜰 때까지 계속 본다)"),
    (17, "new", "?", "probe20_tbl.rs 에 RVA 가 없다(= 1단계에서도 측정 안 됨. MISSING20 참조) — `SPEC_RVA_OVERRIDE` 에 주소를 보충하면 열린다"),
    (40, "v3_assign_anchor", "248,491", "rlib 심볼 internal('t') = 링크 불가(llvm-nm 실측 · IR internal) · fastcc = 호출규약 비호환 · 반환 void = **비교할 값이 없다** — `SELF_DIFF` 에 비교 대상을 등록하면 열린다"),
    (42, "SinglePlanBattle::with_runaway", "0", "미발화(SinglePlanBattle = SingleLane 전용 · MOBA NA · 09-13 r8 판1 0회) — ★프로브는 유지(뜰 때까지 계속 본다)"),
    (44, "LegacyPlanHandler::take_misunderstood_received_chat", "966,291", "rlib 심볼 internal('t') = 링크 불가(llvm-nm 실측 · IR internal) · fastcc = 호출규약 비호환 · 가변 포인터 인자 a0(6168B · tcx `&mut LegacyPlanHandler(6168B)`) = 두 번 호출하면 상태가 두 번 변한다(스냅샷 복원도 불가 — 내부 Vec/Box 재할당)"),
    (47, "TeamPlan::handle_epic_line_change", "0", "미발화(handle_epic_line_change · 09-13 r8 판1 0회 — 이 리플레이 한정) — ★프로브는 유지(뜰 때까지 계속 본다)"),
];

/// 첫 DIFF 덤프(슬롯당 1건 + 전체 상한). detour 문맥에서 잡으므로 poison-safe 하게 연다.
static FIRST: Mutex<Vec<(usize, String)>> = Mutex::new(Vec::new());
const FIRST_MAX: usize = 64;
thread_local! { static D: [std::cell::Cell<u32>; 53] = [const { std::cell::Cell::new(0) }; 53]; }
/// 재진입 깊이. 최상위 호출에서만 대조한다(내 사본이 같은 함수를 재귀 호출해도 2중 대조 안 함).
#[inline] fn top(i: usize) -> bool { D.with(|d| { let v = d[i].get(); d[i].set(v + 1); v == 0 }) }
#[inline] fn pop(i: usize) { D.with(|d| d[i].set(d[i].get().saturating_sub(1))); }
/// `{ i8, i8 }`(rustc **ScalarPair** · al:dl 두 레지스터) 반환용.
/// ⚠`repr(Rust)` 그대로 둔다 — `extern "Rust"` 로 부르므로 rlib 의 원본 구조체와 같은
///   레이아웃·같은 ABI 여야 한다. `repr(C)` 를 붙이면 C ABI(ax 한 칸)로 바뀌어 어긋난다.
#[derive(PartialEq, Debug, Clone, Copy)] pub struct P8 { pub a: u8, pub b: u8 }

/// `{ i64, i64 }`(ScalarPair · rax:rdx) 반환용. `repr(Rust)` 유지(위 `P8` 와 같은 이유).
#[derive(PartialEq, Debug, Clone, Copy)] pub struct P64 { pub a: i64, pub b: i64 }

/// 살아있는 구간 하나. `c` 의 조건이 **전부** 성립할 때만 비교 대상이다.
#[derive(Clone, Copy)] pub struct Cond { pub off: u16, pub len: u8, pub mask: u64 }
#[derive(Clone, Copy)] pub struct Span { pub off: u16, pub len: u8, pub c: [Cond; 2] }
const NOC: Cond = Cond { off: 0, len: 0, mask: 0 };
/// 리틀엔디언 태그 읽기(1~8B).
#[inline] unsafe fn rdtag(p: *const u8, off: u16, len: u8) -> u64 {
    let mut v = 0u64;
    for i in 0..len as usize { v |= (*p.add(off as usize + i) as u64) << (8 * i); }
    v
}
/// 게임 버퍼 `g` 의 판별자로 **살아있는 구간만** 골라 `m` 과 비교한다.
/// 반환 = 갈린 첫 구간의 오프셋(없으면 None) — 「어디가」를 남겨야 다음이 짧다.
unsafe fn live_eq(g: *const u8, m: *const u8, sp: &[Span]) -> Option<usize> {
    for s in sp {
        let mut on = true;
        for c in s.c.iter() {
            if c.len == 0 { continue; }
            let t = rdtag(g, c.off, c.len);
            if t >= 64 || (c.mask >> t) & 1 == 0 { on = false; break; }
        }
        if !on { continue; }
        let (o, l) = (s.off as usize, s.len as usize);
        if core::slice::from_raw_parts(g.add(o), l) != core::slice::from_raw_parts(m.add(o), l) {
            return Some(o);
        }
    }
    None
}
/// `#62 resolve_fight_uncached` 의 살아있는 구간(생성기 `SRET_LIVE` 에서 자동 생성).
static LIVE_62: &[Span] = &[
    Span { off: 0x0, len: 8, c: [NOC, NOC] },
    Span { off: 0x8, len: 8, c: [Cond { off: 0x0, len: 8, mask: 0x2 }, NOC] },
    Span { off: 0x10, len: 8, c: [NOC, NOC] },
    Span { off: 0x18, len: 8, c: [Cond { off: 0x10, len: 8, mask: 0x2 }, NOC] },
    Span { off: 0x20, len: 8, c: [NOC, NOC] },
    Span { off: 0x28, len: 8, c: [Cond { off: 0x20, len: 8, mask: 0x2 }, NOC] },
    Span { off: 0x30, len: 8, c: [NOC, NOC] },
    Span { off: 0x38, len: 1, c: [NOC, NOC] },
    Span { off: 0x39, len: 1, c: [NOC, NOC] },
];
/// `#49 resolve_join_stake` 의 살아있는 구간(생성기 `SRET_LIVE` 에서 자동 생성).
static LIVE_49: &[Span] = &[
    Span { off: 0x0, len: 8, c: [NOC, NOC] },
    Span { off: 0x8, len: 8, c: [Cond { off: 0x0, len: 8, mask: 0x2 }, NOC] },
    Span { off: 0x10, len: 8, c: [NOC, NOC] },
    Span { off: 0x18, len: 8, c: [Cond { off: 0x10, len: 8, mask: 0x2 }, NOC] },
    Span { off: 0x20, len: 8, c: [NOC, NOC] },
    Span { off: 0x28, len: 8, c: [Cond { off: 0x20, len: 8, mask: 0x2 }, NOC] },
    Span { off: 0x30, len: 8, c: [NOC, NOC] },
    Span { off: 0x38, len: 1, c: [NOC, NOC] },
    Span { off: 0x39, len: 1, c: [NOC, NOC] },
];
/// `#00 ult` 의 살아있는 구간(생성기 `SRET_LIVE` 에서 자동 생성).
static LIVE_0: &[Span] = &[
    Span { off: 0x0, len: 8, c: [NOC, NOC] },
    Span { off: 0x8, len: 8, c: [Cond { off: 0x0, len: 8, mask: 0x1 }, NOC] },
    Span { off: 0x10, len: 8, c: [Cond { off: 0x0, len: 8, mask: 0x1 }, NOC] },
    Span { off: 0x8, len: 4, c: [Cond { off: 0x0, len: 8, mask: 0x3c }, NOC] },
    Span { off: 0x10, len: 8, c: [Cond { off: 0x0, len: 8, mask: 0x3c }, Cond { off: 0x8, len: 4, mask: 0x7 }] },
    Span { off: 0x18, len: 8, c: [Cond { off: 0x0, len: 8, mask: 0x3c }, Cond { off: 0x8, len: 4, mask: 0x6 }] },
];
/// `#35 resolve_fight_stake` 의 살아있는 구간(생성기 `SRET_LIVE` 에서 자동 생성).
static LIVE_35: &[Span] = &[
    Span { off: 0x0, len: 8, c: [NOC, NOC] },
    Span { off: 0x8, len: 8, c: [Cond { off: 0x0, len: 8, mask: 0x2 }, NOC] },
    Span { off: 0x10, len: 8, c: [NOC, NOC] },
    Span { off: 0x18, len: 8, c: [Cond { off: 0x10, len: 8, mask: 0x2 }, NOC] },
    Span { off: 0x20, len: 8, c: [NOC, NOC] },
    Span { off: 0x28, len: 8, c: [Cond { off: 0x20, len: 8, mask: 0x2 }, NOC] },
    Span { off: 0x30, len: 8, c: [NOC, NOC] },
    Span { off: 0x38, len: 1, c: [NOC, NOC] },
    Span { off: 0x39, len: 1, c: [NOC, NOC] },
];
/// `#20 v27_active_objective_discipline` 의 살아있는 구간(생성기 `SRET_LIVE` 에서 자동 생성).
static LIVE_20: &[Span] = &[
    Span { off: 0x19, len: 1, c: [NOC, NOC] },
    Span { off: 0x0, len: 25, c: [Cond { off: 0x19, len: 1, mask: 0x3 }, NOC] },
];
/// `#21 upgrade_item` 의 살아있는 구간(생성기 `SRET_LIVE` 에서 자동 생성).
static LIVE_21: &[Span] = &[
    Span { off: 0x0, len: 8, c: [NOC, NOC] },
    Span { off: 0x8, len: 16, c: [Cond { off: 0x0, len: 8, mask: 0x2 }, NOC] },
];
/// `#55 EntityPositioningCache::new` 의 살아있는 구간(생성기 `SRET_LIVE` 에서 자동 생성).
static LIVE_55: &[Span] = &[
    Span { off: 0x0, len: 8, c: [NOC, NOC] },
    Span { off: 0x8, len: 8, c: [NOC, NOC] },
    Span { off: 0x10, len: 8, c: [NOC, NOC] },
    Span { off: 0x18, len: 8, c: [NOC, NOC] },
    Span { off: 0x20, len: 8, c: [NOC, NOC] },
    Span { off: 0x28, len: 8, c: [NOC, NOC] },
    Span { off: 0x30, len: 8, c: [NOC, NOC] },
    Span { off: 0x38, len: 8, c: [NOC, NOC] },
    Span { off: 0x40, len: 8, c: [NOC, NOC] },
    Span { off: 0x48, len: 8, c: [NOC, NOC] },
    Span { off: 0x50, len: 8, c: [NOC, NOC] },
    Span { off: 0x58, len: 8, c: [NOC, NOC] },
    Span { off: 0x60, len: 8, c: [NOC, NOC] },
    Span { off: 0x68, len: 8, c: [NOC, NOC] },
    Span { off: 0x70, len: 8, c: [NOC, NOC] },
    Span { off: 0x78, len: 8, c: [NOC, NOC] },
    Span { off: 0x80, len: 8, c: [NOC, NOC] },
    Span { off: 0x88, len: 8, c: [NOC, NOC] },
    Span { off: 0x90, len: 8, c: [NOC, NOC] },
    Span { off: 0x98, len: 8, c: [NOC, NOC] },
    Span { off: 0xa0, len: 8, c: [NOC, NOC] },
    Span { off: 0xa8, len: 8, c: [NOC, NOC] },
    Span { off: 0xb0, len: 8, c: [NOC, NOC] },
    Span { off: 0xb8, len: 8, c: [NOC, NOC] },
    Span { off: 0xc0, len: 8, c: [NOC, NOC] },
    Span { off: 0xc8, len: 8, c: [NOC, NOC] },
    Span { off: 0xd0, len: 8, c: [NOC, NOC] },
    Span { off: 0xd8, len: 8, c: [NOC, NOC] },
    Span { off: 0xe0, len: 8, c: [NOC, NOC] },
    Span { off: 0xe8, len: 8, c: [NOC, NOC] },
    Span { off: 0xf0, len: 8, c: [NOC, NOC] },
    Span { off: 0xf8, len: 8, c: [NOC, NOC] },
    Span { off: 0x100, len: 8, c: [NOC, NOC] },
    Span { off: 0x108, len: 8, c: [NOC, NOC] },
    Span { off: 0x110, len: 8, c: [NOC, NOC] },
    Span { off: 0x118, len: 8, c: [NOC, NOC] },
    Span { off: 0x120, len: 8, c: [NOC, NOC] },
    Span { off: 0x128, len: 8, c: [NOC, NOC] },
    Span { off: 0x130, len: 8, c: [NOC, NOC] },
    Span { off: 0x138, len: 8, c: [NOC, NOC] },
    Span { off: 0x140, len: 8, c: [NOC, NOC] },
    Span { off: 0x148, len: 8, c: [NOC, NOC] },
    Span { off: 0x150, len: 8, c: [NOC, NOC] },
    Span { off: 0x158, len: 8, c: [NOC, NOC] },
    Span { off: 0x160, len: 8, c: [NOC, NOC] },
    Span { off: 0x168, len: 8, c: [NOC, NOC] },
    Span { off: 0x170, len: 8, c: [NOC, NOC] },
    Span { off: 0x178, len: 8, c: [NOC, NOC] },
    Span { off: 0x180, len: 8, c: [NOC, NOC] },
    Span { off: 0x188, len: 8, c: [NOC, NOC] },
    Span { off: 0x190, len: 8, c: [NOC, NOC] },
    Span { off: 0x198, len: 8, c: [NOC, NOC] },
    Span { off: 0x1a0, len: 1, c: [NOC, NOC] },
    Span { off: 0x1a1, len: 1, c: [NOC, NOC] },
    Span { off: 0x1a2, len: 1, c: [NOC, NOC] },
];

fn note(i: usize, s: String) {
    S[i].diff.fetch_add(1, Ordering::Relaxed);
    let mut g = FIRST.lock().unwrap_or_else(|e| e.into_inner());
    if g.len() < FIRST_MAX && !g.iter().any(|x| x.0 == i) { g.push((i, s)); }
}

/// IR 의 `align 16` 계약을 지키기 위한 래퍼 — 지금은 `readnone` 이라 무해하지만
/// rlib 가 바뀌어 `movaps` 로 읽게 되면 정렬 위반은 곰바로 0xc0000005 다.
#[repr(align(16))] struct A16([u8; 320]);

thread_local! {
    /// `#61` 의 **게임 호출 전** self 스냅샷(1064B). 스레드당 1개 — 재진입은 `top()` 이 막는다.
    static SV3: core::cell::UnsafeCell<[u8; 1064]> = core::cell::UnsafeCell::new([0u8; 1064]);
    /// `#61` 의 **게임 호출 후** self 스냅샷(1064B).
    static SP3: core::cell::UnsafeCell<[u8; 1064]> = core::cell::UnsafeCell::new([0u8; 1064]);
    /// `#61` 의 `Vec` 사본 버퍼(용량 512개) + 게임과 같은 len.
    /// `.2` = 이번 호출이 내 버퍼에 **들어갔나**. 안 들어갔으면 비교를 건너뛴다(거짓 DIFF 방지).
    static VB3: core::cell::UnsafeCell<([u8; 12288], usize, bool)> = core::cell::UnsafeCell::new(([0u8; 12288], 0, false));
}
thread_local! {
    /// `#05` 의 **게임 호출 전** self 스냅샷(6168B). 스레드당 1개 — 재진입은 `top()` 이 막는다.
    static SV7: core::cell::UnsafeCell<[u8; 6168]> = core::cell::UnsafeCell::new([0u8; 6168]);
    /// `#05` 의 **게임 호출 후** self 스냅샷(6168B).
    static SP7: core::cell::UnsafeCell<[u8; 6168]> = core::cell::UnsafeCell::new([0u8; 6168]);
    /// `#05` 의 **내 사본 호출 후** self 스냅샷(6168B) — 반환값이 죽은 슬롯의 판정 재료.
    static SQ7: core::cell::UnsafeCell<[u8; 6168]> = core::cell::UnsafeCell::new([0u8; 6168]);
    /// `#05` 의 `Vec` 사본 버퍼(용량 118개) + 게임과 같은 len.
    /// `.2` = 이번 호출이 내 버퍼에 **들어갔나**. 안 들어갔으면 비교를 건너뛴다(거짓 DIFF 방지).
    static VB7: core::cell::UnsafeCell<([u8; 12272], usize, bool)> = core::cell::UnsafeCell::new(([0u8; 12272], 0, false));
}
thread_local! {
    /// `#11` 의 **게임 호출 전** self 스냅샷(6168B). 스레드당 1개 — 재진입은 `top()` 이 막는다.
    static SV8: core::cell::UnsafeCell<[u8; 6168]> = core::cell::UnsafeCell::new([0u8; 6168]);
    /// `#11` 의 **게임 호출 후** self 스냅샷(6168B).
    static SP8: core::cell::UnsafeCell<[u8; 6168]> = core::cell::UnsafeCell::new([0u8; 6168]);
    /// `#11` 의 **내 사본 호출 후** self 스냅샷(6168B) — 반환값이 죽은 슬롯의 판정 재료.
    static SQ8: core::cell::UnsafeCell<[u8; 6168]> = core::cell::UnsafeCell::new([0u8; 6168]);
    /// `#11` 명세 0 소유 Vec 의 **게임 호출 전** 내용(32KB) + (cap,len,esz)×4 + 개수 + 수용 여부.
    ///   ⚠게임 호출이 그 버퍼를 **해제/재할당**했을 수 있어 호출 뒤에 읽으면 freelist 잔재다.
    static PV8_0: core::cell::UnsafeCell<([u8; 32768], [(usize, usize, usize); 8], usize, bool)> = core::cell::UnsafeCell::new(([0u8; 32768], [(0, 0, 0); 8], 0, false));
    /// `#11` 의 `Vec` 사본 버퍼(용량 1536개) + 게임과 같은 len.
    /// `.2` = 이번 호출이 내 버퍼에 **들어갔나**. 안 들어갔으면 비교를 건너뛴다(거짓 DIFF 방지).
    static VB8: core::cell::UnsafeCell<([u8; 12288], usize, bool)> = core::cell::UnsafeCell::new(([0u8; 12288], 0, false));
}
thread_local! {
    /// `#14` 의 **게임 호출 전** self 스냅샷(48B). 스레드당 1개 — 재진입은 `top()` 이 막는다.
    static SV14: core::cell::UnsafeCell<[u8; 48]> = core::cell::UnsafeCell::new([0u8; 48]);
    /// `#14` 의 **게임 호출 후** self 스냅샷(48B).
    static SP14: core::cell::UnsafeCell<[u8; 48]> = core::cell::UnsafeCell::new([0u8; 48]);
    /// `#14` 의 **내 사본 호출 후** self 스냅샷(48B) — 반환값이 죽은 슬롯의 판정 재료.
    static SQ14: core::cell::UnsafeCell<[u8; 48]> = core::cell::UnsafeCell::new([0u8; 48]);
    /// `#14` 의 `Vec` 사본 버퍼(용량 512개) + 게임과 같은 len.
    /// `.2` = 이번 호출이 내 버퍼에 **들어갔나**. 안 들어갔으면 비교를 건너뛴다(거짓 DIFF 방지).
    static VB14: core::cell::UnsafeCell<([u8; 12288], usize, bool)> = core::cell::UnsafeCell::new(([0u8; 12288], 0, false));
}
thread_local! {
    /// `#18` — exe 가 **버린** `&mut StdRng` 자리에 넘길 더미(320B).
    /// 게임은 이 인자를 안 쓴다(그래서 dead-arg 로 제거됐다) — 내 사본도 안 써야 정상이고,
    /// 쓴다면 DIFF 로 드러난다(= 그 자체가 새 사실).
    static RNG28: core::cell::UnsafeCell<A16> = core::cell::UnsafeCell::new(A16([0u8; 320]));
}
thread_local! {
    /// `#18` 의 **게임 호출 전** self 스냅샷(1064B). 스레드당 1개 — 재진입은 `top()` 이 막는다.
    static SV28: core::cell::UnsafeCell<[u8; 1064]> = core::cell::UnsafeCell::new([0u8; 1064]);
    /// `#18` 의 **게임 호출 후** self 스냅샷(1064B).
    static SP28: core::cell::UnsafeCell<[u8; 1064]> = core::cell::UnsafeCell::new([0u8; 1064]);
    /// `#18` 의 **내 사본 호출 후** self 스냅샷(1064B) — 반환값이 죽은 슬롯의 판정 재료.
    static SQ28: core::cell::UnsafeCell<[u8; 1064]> = core::cell::UnsafeCell::new([0u8; 1064]);
    /// `#18` 의 `Vec` 사본 버퍼(용량 512개) + 게임과 같은 len.
    /// `.2` = 이번 호출이 내 버퍼에 **들어갔나**. 안 들어갔으면 비교를 건너뛴다(거짓 DIFF 방지).
    static VB28: core::cell::UnsafeCell<([u8; 12288], usize, bool)> = core::cell::UnsafeCell::new(([0u8; 12288], 0, false));
}
thread_local! {
    /// `#12` 의 **게임 호출 전** self 스냅샷(6168B). 스레드당 1개 — 재진입은 `top()` 이 막는다.
    static SV30: core::cell::UnsafeCell<[u8; 6168]> = core::cell::UnsafeCell::new([0u8; 6168]);
    /// `#12` 의 **게임 호출 후** self 스냅샷(6168B).
    static SP30: core::cell::UnsafeCell<[u8; 6168]> = core::cell::UnsafeCell::new([0u8; 6168]);
    /// `#12` 의 **내 사본 호출 후** self 스냅샷(6168B) — 반환값이 죽은 슬롯의 판정 재료.
    static SQ30: core::cell::UnsafeCell<[u8; 6168]> = core::cell::UnsafeCell::new([0u8; 6168]);
    /// `#12` 명세 0 소유 Vec 의 **게임 호출 전** 내용(32KB) + (cap,len,esz)×4 + 개수 + 수용 여부.
    ///   ⚠게임 호출이 그 버퍼를 **해제/재할당**했을 수 있어 호출 뒤에 읽으면 freelist 잔재다.
    static PV30_0: core::cell::UnsafeCell<([u8; 32768], [(usize, usize, usize); 8], usize, bool)> = core::cell::UnsafeCell::new(([0u8; 32768], [(0, 0, 0); 8], 0, false));
    /// `#12` 명세 1 소유 Vec 의 **게임 호출 전** 내용(32KB) + (cap,len,esz)×4 + 개수 + 수용 여부.
    ///   ⚠게임 호출이 그 버퍼를 **해제/재할당**했을 수 있어 호출 뒤에 읽으면 freelist 잔재다.
    static PV30_1: core::cell::UnsafeCell<([u8; 32768], [(usize, usize, usize); 8], usize, bool)> = core::cell::UnsafeCell::new(([0u8; 32768], [(0, 0, 0); 8], 0, false));
    /// `#12` 의 `Vec` 사본 버퍼(용량 1536개) + 게임과 같은 len.
    /// `.2` = 이번 호출이 내 버퍼에 **들어갔나**. 안 들어갔으면 비교를 건너뛴다(거짓 DIFF 방지).
    static VB30: core::cell::UnsafeCell<([u8; 12288], usize, bool)> = core::cell::UnsafeCell::new(([0u8; 12288], 0, false));
}
thread_local! {
    /// `#54` 의 **게임 호출 전** self 스냅샷(1064B). 스레드당 1개 — 재진입은 `top()` 이 막는다.
    static SV49: core::cell::UnsafeCell<[u8; 1064]> = core::cell::UnsafeCell::new([0u8; 1064]);
    /// `#54` 의 **게임 호출 후** self 스냅샷(1064B).
    static SP49: core::cell::UnsafeCell<[u8; 1064]> = core::cell::UnsafeCell::new([0u8; 1064]);
    /// `#54` 의 **내 사본 호출 후** self 스냅샷(1064B) — 반환값이 죽은 슬롯의 판정 재료.
    static SQ49: core::cell::UnsafeCell<[u8; 1064]> = core::cell::UnsafeCell::new([0u8; 1064]);
    /// `#54` 의 `Vec` 사본 버퍼(용량 512개) + 게임과 같은 len.
    /// `.2` = 이번 호출이 내 버퍼에 **들어갔나**. 안 들어갔으면 비교를 건너뛴다(거짓 DIFF 방지).
    static VB49: core::cell::UnsafeCell<([u8; 12288], usize, bool)> = core::cell::UnsafeCell::new(([0u8; 12288], 0, false));
}
/// Vec 요소 비교 — `lid` 가 가리키는 ELEM_LIVE 명세로 **살아있는 바이트**만 본다(0 = 전 바이트).
unsafe fn elem_cmp(lid: u8, esz: usize, gb: usize, mb: usize) -> Option<String> {
    let tag = *(gb as *const u8);
    match lid {
        1 => {
            if true { for j in 0..1 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8));
                if gv != mv { return Some(format!("(tag {:#x})+{}: g={:02x} m={:02x}", tag, j, gv, mv)); } } }
            if tag == 0x02 || tag == 0x05 || tag == 0x07 || tag == 0x08 || tag == 0x09 || tag == 0x0a || tag == 0x0b || tag == 0x0c || tag == 0x0d || tag == 0x0e || tag == 0x11 || tag == 0x12 || tag == 0x14 || tag == 0x15 || tag == 0x16 || tag == 0x1f || tag == 0x20 || tag == 0x29 || tag == 0x2a || tag == 0x2c || tag == 0x2d || tag == 0x2e || tag == 0x2f || tag == 0x30 || tag == 0x31 || tag == 0x32 || tag == 0x33 || tag == 0x34 || tag == 0x35 || tag == 0x36 || tag == 0x37 || tag == 0x38 { for j in 1..2 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8));
                if gv != mv { return Some(format!("(tag {:#x})+{}: g={:02x} m={:02x}", tag, j, gv, mv)); } } }
            if tag == 0x2f || tag == 0x30 || tag == 0x31 || tag == 0x32 || tag == 0x37 { for j in 2..3 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8));
                if gv != mv { return Some(format!("(tag {:#x})+{}: g={:02x} m={:02x}", tag, j, gv, mv)); } } }
            if tag == 0x31 { for j in 3..4 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8));
                if gv != mv { return Some(format!("(tag {:#x})+{}: g={:02x} m={:02x}", tag, j, gv, mv)); } } }
            if tag == 0x01 { for j in 4..8 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8));
                if gv != mv { return Some(format!("(tag {:#x})+{}: g={:02x} m={:02x}", tag, j, gv, mv)); } } }
            if tag == 0x00 || tag == 0x01 || tag == 0x02 || tag == 0x03 || tag == 0x04 || tag == 0x05 || tag == 0x06 || tag == 0x08 || tag == 0x09 || tag == 0x0a || tag == 0x0b || tag == 0x0c || tag == 0x0d || tag == 0x0e || tag == 0x0f || tag == 0x10 || tag == 0x12 || tag == 0x13 || tag == 0x14 || tag == 0x15 || tag == 0x16 || tag == 0x17 || tag == 0x18 || tag == 0x19 || tag == 0x1a || tag == 0x1b || tag == 0x1c || tag == 0x1d || tag == 0x1e || tag == 0x21 || tag == 0x22 || tag == 0x23 || tag == 0x24 || tag == 0x25 || tag == 0x26 || tag == 0x27 || tag == 0x2a || tag == 0x2b || tag == 0x2c || tag == 0x2d || tag == 0x2e || tag == 0x2f || tag == 0x30 || tag == 0x31 || tag == 0x32 || tag == 0x33 || tag == 0x34 || tag == 0x35 || tag == 0x36 || tag == 0x37 || tag == 0x38 { for j in 8..16 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8));
                if gv != mv { return Some(format!("(tag {:#x})+{}: g={:02x} m={:02x}", tag, j, gv, mv)); } } }
            if tag == 0x03 || tag == 0x04 || tag == 0x06 || tag == 0x18 || tag == 0x21 { for j in 16..24 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8));
                if gv != mv { return Some(format!("(tag {:#x})+{}: g={:02x} m={:02x}", tag, j, gv, mv)); } } }
            None
        }
        2 => {
            if true { for j in 0..8 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8));
                if gv != mv { return Some(format!("(tag {:#x})+{}: g={:02x} m={:02x}", tag, j, gv, mv)); } } }
            if tag == 0x0f { for j in 128..133 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8));
                if gv != mv { return Some(format!("(tag {:#x})+{}: g={:02x} m={:02x}", tag, j, gv, mv)); } } }
            if true { for j in 176..184 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8));
                if gv != mv { return Some(format!("(tag {:#x})+{}: g={:02x} m={:02x}", tag, j, gv, mv)); } } }
            if tag == 0x0f {
                let (gp, gl) = (*((gb + 0x8 + 8) as *const usize), *((gb + 0x8 + 16) as *const usize));
                let (mp, ml) = (*((mb + 0x8 + 8) as *const usize), *((mb + 0x8 + 16) as *const usize));
                if gl != ml { return Some(format!("+{:#x}.len: g={} m={}", 0x8, gl, ml)); }
                if gl > 0 && gl < 65536 && gp > 0x1000 && mp > 0x1000 && gp != mp {
                    for j in 0..gl { let (gv, mv) = (*((gp + j) as *const u8), *((mp + j) as *const u8));
                        if gv != mv { return Some(format!("+{:#x}.str[{}]: g={:02x} m={:02x}", 0x8, j, gv, mv)); } }
                }
            }
            if tag == 0x0f {
                let (gp, gl) = (*((gb + 0x20 + 8) as *const usize), *((gb + 0x20 + 16) as *const usize));
                let (mp, ml) = (*((mb + 0x20 + 8) as *const usize), *((mb + 0x20 + 16) as *const usize));
                if gl != ml { return Some(format!("+{:#x}.len: g={} m={}", 0x20, gl, ml)); }
                if gl > 0 && gl < 65536 && gp > 0x1000 && mp > 0x1000 && gp != mp {
                    for j in 0..gl { let (gv, mv) = (*((gp + j) as *const u8), *((mp + j) as *const u8));
                        if gv != mv { return Some(format!("+{:#x}.str[{}]: g={:02x} m={:02x}", 0x20, j, gv, mv)); } }
                }
            }
            if tag == 0x0f {
                let (gp, gl) = (*((gb + 0x38 + 8) as *const usize), *((gb + 0x38 + 16) as *const usize));
                let (mp, ml) = (*((mb + 0x38 + 8) as *const usize), *((mb + 0x38 + 16) as *const usize));
                if gl != ml { return Some(format!("+{:#x}.len: g={} m={}", 0x38, gl, ml)); }
                if gl > 0 && gl < 65536 && gp > 0x1000 && mp > 0x1000 && gp != mp {
                    for j in 0..gl { let (gv, mv) = (*((gp + j) as *const u8), *((mp + j) as *const u8));
                        if gv != mv { return Some(format!("+{:#x}.str[{}]: g={:02x} m={:02x}", 0x38, j, gv, mv)); } }
                }
            }
            if tag == 0x0f {
                let (gp, gl) = (*((gb + 0x50 + 8) as *const usize), *((gb + 0x50 + 16) as *const usize));
                let (mp, ml) = (*((mb + 0x50 + 8) as *const usize), *((mb + 0x50 + 16) as *const usize));
                if gl != ml { return Some(format!("+{:#x}.len: g={} m={}", 0x50, gl, ml)); }
                if gl > 0 && gl < 65536 && gp > 0x1000 && mp > 0x1000 && gp != mp {
                    for j in 0..gl { let (gv, mv) = (*((gp + j) as *const u8), *((mp + j) as *const u8));
                        if gv != mv { return Some(format!("+{:#x}.str[{}]: g={:02x} m={:02x}", 0x50, j, gv, mv)); } }
                }
            }
            if tag == 0x0f {
                let (gp, gl) = (*((gb + 0x68 + 8) as *const usize), *((gb + 0x68 + 16) as *const usize));
                let (mp, ml) = (*((mb + 0x68 + 8) as *const usize), *((mb + 0x68 + 16) as *const usize));
                if gl != ml { return Some(format!("+{:#x}.len: g={} m={}", 0x68, gl, ml)); }
                if gl > 0 && gl < 65536 && gp > 0x1000 && mp > 0x1000 && gp != mp {
                    for j in 0..gl { let (gv, mv) = (*((gp + j) as *const u8), *((mp + j) as *const u8));
                        if gv != mv { return Some(format!("+{:#x}.str[{}]: g={:02x} m={:02x}", 0x68, j, gv, mv)); } }
                }
            }
            None
        }
        3 => {
            if true { for j in 0..10 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8));
                if gv != mv { return Some(format!("(tag {:#x})+{}: g={:02x} m={:02x}", tag, j, gv, mv)); } } }
            None
        }
        _ => { for j in 0..esz { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8));
            if gv != mv { return Some(format!("+{}: g={:02x} m={:02x}", j, gv, mv)); } } None }
    }
}
/// 요소 안 String(ELEM_LIVE.str) 해제 — **내 사본이 새로 push 한 요소**에만 쓸 것.
unsafe fn elem_free_str(lid: u8, mb: usize) {
    let tag = *(mb as *const u8);
    match lid {
        2 => {
            if tag == 0x0f { let (c, p) = (*((mb + 0x8) as *const usize), *((mb + 0x8 + 8) as *const usize));
                if c > 0 && c < (1 << 24) && p > 0x1000 { if let Ok(l) = std::alloc::Layout::from_size_align(c, 1) { std::alloc::dealloc(p as *mut u8, l); } } }
            if tag == 0x0f { let (c, p) = (*((mb + 0x20) as *const usize), *((mb + 0x20 + 8) as *const usize));
                if c > 0 && c < (1 << 24) && p > 0x1000 { if let Ok(l) = std::alloc::Layout::from_size_align(c, 1) { std::alloc::dealloc(p as *mut u8, l); } } }
            if tag == 0x0f { let (c, p) = (*((mb + 0x38) as *const usize), *((mb + 0x38 + 8) as *const usize));
                if c > 0 && c < (1 << 24) && p > 0x1000 { if let Ok(l) = std::alloc::Layout::from_size_align(c, 1) { std::alloc::dealloc(p as *mut u8, l); } } }
            if tag == 0x0f { let (c, p) = (*((mb + 0x50) as *const usize), *((mb + 0x50 + 8) as *const usize));
                if c > 0 && c < (1 << 24) && p > 0x1000 { if let Ok(l) = std::alloc::Layout::from_size_align(c, 1) { std::alloc::dealloc(p as *mut u8, l); } } }
            if tag == 0x0f { let (c, p) = (*((mb + 0x68) as *const usize), *((mb + 0x68 + 8) as *const usize));
                if c > 0 && c < (1 << 24) && p > 0x1000 { if let Ok(l) = std::alloc::Layout::from_size_align(c, 1) { std::alloc::dealloc(p as *mut u8, l); } } }
        }
        _ => {}
    }
}
/// `#11` 명세 0 — self+0x5e8 의 메모리태그 → 소유 Vec 목록(off, 요소 크기, live id).
fn hs_vecs_11_0(tag: u64) -> &'static [(usize, usize, u8)] {
    match tag {
        3 => &[(0x20, 24, 1), (0x38, 8, 0), (0x50, 8, 0), (0x68, 16, 3), (0x80, 16, 3)],
        4 => &[(0x8, 24, 1)],
        5 => &[(0x70, 24, 1)],
        7 => &[(0x8, 24, 1), (0x20, 16, 3)],
        9 => &[(0x70, 24, 1), (0x88, 8, 0)],
        10 => &[(0x8, 24, 1)],
        11 => &[(0x8, 24, 1)],
        12 => &[(0x8, 8, 0)],
        14 => &[(0x8, 8, 0)],
        t if !(2..=17).contains(&t) => &[(0xf8, 24, 1)],
        _ => &[],   // 단위 variant 등 소유 없음
    }
}
/// `#12` 명세 0 — self+0x5e8 의 메모리태그 → 소유 Vec 목록(off, 요소 크기, live id).
fn hs_vecs_12_0(tag: u64) -> &'static [(usize, usize, u8)] {
    match tag {
        3 => &[(0x20, 24, 1), (0x38, 8, 0), (0x50, 8, 0), (0x68, 16, 3), (0x80, 16, 3)],
        4 => &[(0x8, 24, 1)],
        5 => &[(0x70, 24, 1)],
        7 => &[(0x8, 24, 1), (0x20, 16, 3)],
        9 => &[(0x70, 24, 1), (0x88, 8, 0)],
        10 => &[(0x8, 24, 1)],
        11 => &[(0x8, 24, 1)],
        12 => &[(0x8, 8, 0)],
        14 => &[(0x8, 8, 0)],
        t if !(2..=17).contains(&t) => &[(0xf8, 24, 1)],
        _ => &[],   // 단위 variant 등 소유 없음
    }
}
/// `#12` 명세 1 — self+0x0 의 평면 필드(태그 무관) → 소유 Vec 목록(off, 요소 크기, live id).
fn hs_vecs_12_1(tag: u64) -> &'static [(usize, usize, u8)] {
    match tag {
        _ => &[(0x7c8, 24, 1), (0x858, 184, 2)],
    }
}
#[inline(always)] unsafe fn rd_le(p: usize, n: usize) -> u64 { let mut v = 0u64; for j in 0..n { v |= (*((p + j) as *const u8) as u64) << (8 * j); } v }
/// `#11` self+0x5e8 `BigPlan` — variant 별 페이로드 live 비교(structlive.py 자동 생성 · variant 14개 · 태그 2..=17 · 밖 = untagged).
/// variant 히스토그램 — [0] = untagged(범위 밖) · [t] = 메모리태그 t (대조 1건당 1증가 · 단위 variant 포함).
pub static EH_11_0: [AtomicU64; 32] = [const { AtomicU64::new(0) }; 32];
pub static EH_11_0_NAMES: &[(u64, &str)] = &[(2, "ForcePassive"), (3, "PassiveLine"), (4, "SinglePlanLine"), (5, "SinglePlanBattle"), (7, "PassiveJungle"), (8, "ActiveRecall"), (9, "Battle"), (10, "LineGanker"), (11, "LineGankCover"), (12, "EpicHuntAndPoke"), (13, "EpicHuntAndBattle"), (14, "SerpenHuntAndPoke"), (15, "SerpenHuntAndBattle"), (16, "AttackNexus"), (17, "DefenseNexus"), (0, "DeathMatchBattle")];
pub unsafe fn enumlive_cmp_11_0(tag: u64, gb: usize, mb: usize) -> Option<String> {
    EH_11_0[if (2..=17).contains(&tag) { tag as usize } else { 0 }].fetch_add(1, Ordering::Relaxed);
    match tag {
        3 => {   // PassiveLinePlan · 잎 26
            if true { for j in 0x8..0x10 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x8, 8) == 1 { for j in 0x10..0x18 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x8, 8) == 1 { for j in 0x18..0x19 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x98..0xa0 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0xa0..0xa8 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0xa8..0xb0 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0xb0..0xb8 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0xb8..0xc0 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0xc0..0xc8 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0xc8..0xd0 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0xd0..0xd8 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0xd8..0xe0 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0xe0..0xe8 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0xe8..0xf0 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0xf0..0xf8 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0xf8..0x100 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x100..0x108 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x108..0x110 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x110..0x118 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x118..0x119 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x119..0x11a { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x11a..0x11b { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x11b..0x11c { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x11c..0x11d { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x11d..0x11e { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x11e..0x11f { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            None
        }
        4 => {   // SinglePlanLine · 잎 2
            if true { for j in 0x20..0x21 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x21..0x22 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            None
        }
        5 => {   // SinglePlanBattle · 잎 41
            if true { for j in 0x8..0x10 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x8, 8) == 1 { for j in 0x10..0x18 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x18..0x20 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x18, 8) == 1 { for j in 0x20..0x28 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x18, 8) == 1 { for j in 0x28..0x30 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x18, 8) == 1 { for j in 0x30..0x38 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x38..0x40 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x38, 8) == 1 { for j in 0x40..0x48 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x48..0x50 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x48, 8) == 1 { for j in 0x50..0x58 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x48, 8) == 0 { for j in 0x50..0x58 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x48, 8) == 0 { for j in 0x58..0x60 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x60..0x68 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x60, 8) == 5 { for j in 0x68..0x70 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x60, 8) == 6 { for j in 0x68..0x70 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x60, 8) == 2 { for j in 0x68..0x70 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x60, 8) == 3 { for j in 0x68..0x70 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x60, 8) == 1 { for j in 0x68..0x70 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x60, 8) == 0 { for j in 0x68..0x70 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x88..0x90 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x90..0x91 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x91..0x92 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x92..0x93 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x93..0x94 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x94, 1), 255) { for j in 0x94..0x95 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x94..0x95 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x95, 1), 255) { for j in 0x95..0x96 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x95..0x96 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x95, 1), 255) && rd_le(gb + 0x95, 1) == 11 { for j in 0x96..0x97 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x95, 1), 255) && rd_le(gb + 0x95, 1) == 3 { for j in 0x96..0x97 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x95, 1), 255) && rd_le(gb + 0x95, 1) == 9 { for j in 0x96..0x97 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x95, 1), 255) && rd_le(gb + 0x95, 1) == 8 { for j in 0x96..0x97 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x95, 1), 255) && rd_le(gb + 0x95, 1) == 0 { for j in 0x96..0x97 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x95, 1), 255) && rd_le(gb + 0x95, 1) == 4 { for j in 0x96..0x97 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x95, 1), 255) && rd_le(gb + 0x95, 1) == 5 { for j in 0x96..0x97 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x95, 1), 255) && rd_le(gb + 0x95, 1) == 10 { for j in 0x96..0x97 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x95, 1), 255) && rd_le(gb + 0x95, 1) == 1 { for j in 0x96..0x97 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x95, 1), 255) && rd_le(gb + 0x95, 1) == 6 { for j in 0x96..0x97 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x95, 1), 255) && rd_le(gb + 0x95, 1) == 11 { for j in 0x97..0x98 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x95, 1), 255) && rd_le(gb + 0x95, 1) == 0 { for j in 0x97..0x98 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x95, 1), 255) && rd_le(gb + 0x95, 1) == 1 { for j in 0x97..0x98 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            None
        }
        7 => {   // PassiveJunglePlan · 잎 8
            if true { if (rd_le(gb + 0x20, 8) >> 63) != (rd_le(mb + 0x20, 8) >> 63) { return Some(format!("+{:#x}.opt g={} m={}", 0x20, if rd_le(gb + 0x20, 8) >> 63 != 0 { "None" } else { "Some" }, if rd_le(mb + 0x20, 8) >> 63 != 0 { "None" } else { "Some" })); } }
            if ((rd_le(gb + 0x20, 8) >> 63) == 0) == true { for j in 0x38..0x40 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if ((rd_le(gb + 0x20, 8) >> 63) == 0) == true { for j in 0x40..0x48 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if ((rd_le(gb + 0x20, 8) >> 63) == 0) == true { for j in 0x48..0x49 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x50..0x58 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x58..0x60 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x60..0x68 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x68..0x69 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            None
        }
        9 => {   // BattlePlan · 잎 75
            if true { for j in 0x8..0x10 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x8, 8) == 1 { for j in 0x10..0x18 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x18..0x20 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x18, 8) == 1 { for j in 0x20..0x28 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x18, 8) == 1 { for j in 0x28..0x30 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x18, 8) == 1 { for j in 0x30..0x38 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x38..0x40 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x38, 8) == 1 { for j in 0x40..0x48 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x48..0x50 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x48, 8) == 1 { for j in 0x50..0x58 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x48, 8) == 0 { for j in 0x50..0x58 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x48, 8) == 0 { for j in 0x58..0x60 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x60..0x68 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x60, 8) == 5 { for j in 0x68..0x70 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x60, 8) == 6 { for j in 0x68..0x70 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x60, 8) == 2 { for j in 0x68..0x70 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x60, 8) == 3 { for j in 0x68..0x70 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x60, 8) == 1 { for j in 0x68..0x70 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x60, 8) == 0 { for j in 0x68..0x70 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0xa0..0xa8 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0xa8..0xb0 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0xb0..0xb8 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0xb8..0xc0 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0xc0..0xc8 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0xc8..0xd0 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0xd0..0xd8 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0xd8..0xe0 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0xe0..0xe8 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0xe8..0xf0 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0xf0..0xf8 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0xf8..0xfc { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0xfc, 1), 2) { for j in 0xfc..0xfd { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0xfc..0xfd { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0xfc, 1), 2) { for j in 0xfd..0xfe { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0xfe..0xff { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0xff..0x100 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x100..0x101 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x101..0x102 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x102..0x103 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x103..0x104 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x104..0x105 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x105..0x106 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x106, 1), 255) { for j in 0x106..0x107 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x106..0x107 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x107, 1), 255) { for j in 0x107..0x108 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x107..0x108 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x107, 1), 255) && rd_le(gb + 0x107, 1) == 11 { for j in 0x108..0x109 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x107, 1), 255) && rd_le(gb + 0x107, 1) == 3 { for j in 0x108..0x109 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x107, 1), 255) && rd_le(gb + 0x107, 1) == 9 { for j in 0x108..0x109 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x107, 1), 255) && rd_le(gb + 0x107, 1) == 8 { for j in 0x108..0x109 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x107, 1), 255) && rd_le(gb + 0x107, 1) == 0 { for j in 0x108..0x109 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x107, 1), 255) && rd_le(gb + 0x107, 1) == 4 { for j in 0x108..0x109 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x107, 1), 255) && rd_le(gb + 0x107, 1) == 5 { for j in 0x108..0x109 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x107, 1), 255) && rd_le(gb + 0x107, 1) == 10 { for j in 0x108..0x109 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x107, 1), 255) && rd_le(gb + 0x107, 1) == 1 { for j in 0x108..0x109 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x107, 1), 255) && rd_le(gb + 0x107, 1) == 6 { for j in 0x108..0x109 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x107, 1), 255) && rd_le(gb + 0x107, 1) == 11 { for j in 0x109..0x10a { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x107, 1), 255) && rd_le(gb + 0x107, 1) == 0 { for j in 0x109..0x10a { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x107, 1), 255) && rd_le(gb + 0x107, 1) == 1 { for j in 0x109..0x10a { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x10a..0x10b { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x10b..0x10c { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x10c..0x10d { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x10d..0x10e { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x10e..0x10f { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x10f..0x110 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x110..0x111 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x111..0x112 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x112..0x113 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x113..0x114 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x114..0x115 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x115..0x116 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x116..0x117 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x117..0x118 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x118..0x119 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x119..0x11a { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            None
        }
        10 => {   // LineGankerPlan · 잎 5
            if true { for j in 0x20..0x28 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x28..0x30 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x30..0x31 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x31, 1), 6 | 7 | 8) { for j in 0x31..0x32 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x31..0x32 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            None
        }
        11 => {   // LineGankCoverPlan · 잎 2
            if true { for j in 0x20..0x28 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x28..0x29 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            None
        }
        12 => {   // EpicHuntAndPokePlan · 잎 3
            if true { for j in 0x20..0x21 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x21..0x22 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x22..0x23 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            None
        }
        13 => {   // EpicHuntAndBattlePlan · 잎 2
            if true { for j in 0x8..0x10 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x8, 8) == 1 { for j in 0x10..0x18 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            None
        }
        14 => {   // SerpenHuntAndPokePlan · 잎 3
            if true { for j in 0x20..0x21 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x21..0x22 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x22..0x23 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            None
        }
        15 => {   // SerpenHuntAndBattlePlan · 잎 2
            if true { for j in 0x8..0x10 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x8, 8) == 1 { for j in 0x10..0x18 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            None
        }
        16 => {   // AttackNexusPlan · 잎 2
            if true { for j in 0x8..0x10 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x10..0x11 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            None
        }
        17 => {   // DefenseNexusPlan · 잎 1
            if true { for j in 0x8..0x10 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            None
        }
        t if !(2..=17).contains(&t) => {   // DeathMatchBattle · 잎 77 · untagged(페이로드 base 0)
            if true { for j in 0x0..0x8 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x0, 8) == 1 { for j in 0x8..0x10 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x10..0x18 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x10, 8) == 1 { for j in 0x18..0x20 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x10, 8) == 1 { for j in 0x20..0x28 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x10, 8) == 1 { for j in 0x28..0x30 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x30..0x38 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x30, 8) == 1 { for j in 0x38..0x40 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x40..0x48 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x40, 8) == 1 { for j in 0x48..0x50 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x50..0x58 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x50, 8) == 1 { for j in 0x58..0x60 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x50, 8) == 1 { for j in 0x60..0x68 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x50, 8) == 1 { for j in 0x68..0x70 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x70..0x78 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x70, 8) == 1 { for j in 0x78..0x80 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x70, 8) == 1 { for j in 0x80..0x88 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x70, 8) == 1 { for j in 0x88..0x90 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x90..0x98 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x90, 8) == 1 { for j in 0x98..0xa0 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x90, 8) == 1 { for j in 0xa0..0xa8 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0xa8..0xb0 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0xa8, 8) == 1 { for j in 0xb0..0xb8 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0xa8, 8) == 1 { for j in 0xb8..0xc0 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0xc0..0xc8 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0xc0, 8) == 1 { for j in 0xc8..0xd0 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0xd0..0xd8 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0xd0, 8) == 1 { for j in 0xd8..0xe0 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0xd0, 8) == 0 { for j in 0xd8..0xe0 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0xd0, 8) == 0 { for j in 0xe0..0xe8 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0xe8..0xf0 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0xe8, 8) == 5 { for j in 0xf0..0xf8 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0xe8, 8) == 6 { for j in 0xf0..0xf8 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0xe8, 8) == 2 { for j in 0xf0..0xf8 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0xe8, 8) == 3 { for j in 0xf0..0xf8 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0xe8, 8) == 1 { for j in 0xf0..0xf8 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0xe8, 8) == 0 { for j in 0xf0..0xf8 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x110..0x118 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x118..0x120 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x120..0x128 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x128..0x130 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x130..0x138 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x138..0x140 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x140..0x148 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x148..0x150 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x150..0x158 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x158..0x160 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x160..0x168 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x168..0x170 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x170..0x171 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x171..0x172 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x172..0x173 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x173..0x174 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x174..0x175 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x175..0x176 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x176..0x177 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x177..0x178 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x178..0x179 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x179..0x17a { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x17a, 1), 255) { for j in 0x17a..0x17b { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x17a..0x17b { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x17b, 1), 255) { for j in 0x17b..0x17c { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x17b..0x17c { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x17b, 1), 255) && rd_le(gb + 0x17b, 1) == 11 { for j in 0x17c..0x17d { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x17b, 1), 255) && rd_le(gb + 0x17b, 1) == 3 { for j in 0x17c..0x17d { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x17b, 1), 255) && rd_le(gb + 0x17b, 1) == 9 { for j in 0x17c..0x17d { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x17b, 1), 255) && rd_le(gb + 0x17b, 1) == 8 { for j in 0x17c..0x17d { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x17b, 1), 255) && rd_le(gb + 0x17b, 1) == 0 { for j in 0x17c..0x17d { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x17b, 1), 255) && rd_le(gb + 0x17b, 1) == 4 { for j in 0x17c..0x17d { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x17b, 1), 255) && rd_le(gb + 0x17b, 1) == 5 { for j in 0x17c..0x17d { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x17b, 1), 255) && rd_le(gb + 0x17b, 1) == 10 { for j in 0x17c..0x17d { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x17b, 1), 255) && rd_le(gb + 0x17b, 1) == 1 { for j in 0x17c..0x17d { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x17b, 1), 255) && rd_le(gb + 0x17b, 1) == 6 { for j in 0x17c..0x17d { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x17b, 1), 255) && rd_le(gb + 0x17b, 1) == 11 { for j in 0x17d..0x17e { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x17b, 1), 255) && rd_le(gb + 0x17b, 1) == 0 { for j in 0x17d..0x17e { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x17b, 1), 255) && rd_le(gb + 0x17b, 1) == 1 { for j in 0x17d..0x17e { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x17e..0x17f { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            None
        }
        _ => None,   // 단위 variant(페이로드 0B)
    }
}
/// `#11` self+0x768 `SubPlan` — variant 별 페이로드 live 비교(structlive.py 자동 생성 · variant 13개 · 태그 2..=18 · 밖 = untagged).
/// variant 히스토그램 — [0] = untagged(범위 밖) · [t] = 메모리태그 t (대조 1건당 1증가 · 단위 variant 포함).
pub static EH_11_1: [AtomicU64; 32] = [const { AtomicU64::new(0) }; 32];
pub static EH_11_1_NAMES: &[(u64, &str)] = &[(2, "LineDefense"), (3, "LineSafe"), (4, "LineWait"), (5, "Recall"), (6, "Jungle"), (7, "Battle"), (9, "Hide"), (10, "EpicCheck"), (11, "EpicHunt"), (12, "EpicPoke"), (13, "SerpenCheck"), (14, "SerpenHunt"), (15, "SerpenPoke"), (16, "AttackNexus"), (17, "DefenseNexus"), (18, "Steal"), (0, "DeathBattle")];
pub unsafe fn enumlive_cmp_11_1(tag: u64, gb: usize, mb: usize) -> Option<String> {
    EH_11_1[if (2..=18).contains(&tag) { tag as usize } else { 0 }].fetch_add(1, Ordering::Relaxed);
    match tag {
        2 => {   // LineDefenseSubPlan · 잎 3
            if true { for j in 0x8..0x9 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x9..0xa { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0xa..0xb { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            None
        }
        3 => {   // LineSafeSubPlan · 잎 1
            if true { for j in 0x8..0x9 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            None
        }
        4 => {   // LineWaitSubPlan · 잎 1
            if true { for j in 0x8..0x9 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            None
        }
        6 => {   // JungleSubPlan · 잎 3
            if true { for j in 0x8..0x10 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x10..0x11 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x11..0x12 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            None
        }
        7 => {   // BattleSubPlan · 잎 16
            if true { for j in 0x8..0x10 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x8, 8) == 1 { for j in 0x10..0x18 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x18..0x20 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x18, 8) == 5 { for j in 0x20..0x28 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x18, 8) == 6 { for j in 0x20..0x28 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x18, 8) == 2 { for j in 0x20..0x28 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x18, 8) == 3 { for j in 0x20..0x28 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x18, 8) == 1 { for j in 0x20..0x28 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x18, 8) == 0 { for j in 0x20..0x28 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x28..0x30 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x30..0x31 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x31..0x32 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x32..0x33 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x33..0x34 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x34..0x35 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x35..0x36 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            None
        }
        9 => {   // HideSubPlan · 잎 4
            if true { for j in 0x8..0x10 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x10..0x11 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x11..0x12 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x12..0x13 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            None
        }
        10 => {   // EpicCheckSubPlan · 잎 1
            if true { for j in 0x8..0x9 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            None
        }
        11 => {   // EpicHuntSubPlan · 잎 1
            if true { for j in 0x8..0x9 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            None
        }
        13 => {   // SerpenCheckSubPlan · 잎 1
            if true { for j in 0x8..0x9 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            None
        }
        14 => {   // SerpenHuntSubPlan · 잎 1
            if true { for j in 0x8..0x9 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            None
        }
        17 => {   // DefenseNexusSubPlan · 잎 3
            if true { for j in 0x8..0x10 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x8, 8) == 1 { for j in 0x10..0x18 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x18..0x19 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            None
        }
        18 => {   // StealSubPlan · 잎 4
            if true { for j in 0x8..0x10 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x10, 1), 2) { for j in 0x10..0x11 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x10..0x11 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x11..0x12 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            None
        }
        t if !(2..=18).contains(&t) => {   // DeathBattleSubPlan · 잎 19 · untagged(페이로드 base 0)
            if true { for j in 0x0..0x8 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x0, 8) == 1 { for j in 0x8..0x10 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x10..0x18 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x10, 8) == 1 { for j in 0x18..0x20 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x10, 8) == 1 { for j in 0x20..0x28 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x28..0x30 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x28, 8) == 5 { for j in 0x30..0x38 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x28, 8) == 6 { for j in 0x30..0x38 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x28, 8) == 2 { for j in 0x30..0x38 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x28, 8) == 3 { for j in 0x30..0x38 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x28, 8) == 1 { for j in 0x30..0x38 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x28, 8) == 0 { for j in 0x30..0x38 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x38..0x40 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x40..0x41 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x41..0x42 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x42..0x43 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x43..0x44 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x44..0x45 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x45..0x46 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            None
        }
        _ => None,   // 단위 variant(페이로드 0B)
    }
}
/// `#12` self+0x5e8 `BigPlan` — variant 별 페이로드 live 비교(structlive.py 자동 생성 · variant 14개 · 태그 2..=17 · 밖 = untagged).
/// variant 히스토그램 — [0] = untagged(범위 밖) · [t] = 메모리태그 t (대조 1건당 1증가 · 단위 variant 포함).
pub static EH_12_0: [AtomicU64; 32] = [const { AtomicU64::new(0) }; 32];
pub static EH_12_0_NAMES: &[(u64, &str)] = &[(2, "ForcePassive"), (3, "PassiveLine"), (4, "SinglePlanLine"), (5, "SinglePlanBattle"), (7, "PassiveJungle"), (8, "ActiveRecall"), (9, "Battle"), (10, "LineGanker"), (11, "LineGankCover"), (12, "EpicHuntAndPoke"), (13, "EpicHuntAndBattle"), (14, "SerpenHuntAndPoke"), (15, "SerpenHuntAndBattle"), (16, "AttackNexus"), (17, "DefenseNexus"), (0, "DeathMatchBattle")];
pub unsafe fn enumlive_cmp_12_0(tag: u64, gb: usize, mb: usize) -> Option<String> {
    EH_12_0[if (2..=17).contains(&tag) { tag as usize } else { 0 }].fetch_add(1, Ordering::Relaxed);
    match tag {
        3 => {   // PassiveLinePlan · 잎 26
            if true { for j in 0x8..0x10 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x8, 8) == 1 { for j in 0x10..0x18 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x8, 8) == 1 { for j in 0x18..0x19 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x98..0xa0 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0xa0..0xa8 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0xa8..0xb0 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0xb0..0xb8 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0xb8..0xc0 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0xc0..0xc8 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0xc8..0xd0 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0xd0..0xd8 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0xd8..0xe0 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0xe0..0xe8 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0xe8..0xf0 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0xf0..0xf8 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0xf8..0x100 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x100..0x108 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x108..0x110 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x110..0x118 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x118..0x119 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x119..0x11a { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x11a..0x11b { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x11b..0x11c { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x11c..0x11d { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x11d..0x11e { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x11e..0x11f { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            None
        }
        4 => {   // SinglePlanLine · 잎 2
            if true { for j in 0x20..0x21 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x21..0x22 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            None
        }
        5 => {   // SinglePlanBattle · 잎 41
            if true { for j in 0x8..0x10 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x8, 8) == 1 { for j in 0x10..0x18 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x18..0x20 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x18, 8) == 1 { for j in 0x20..0x28 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x18, 8) == 1 { for j in 0x28..0x30 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x18, 8) == 1 { for j in 0x30..0x38 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x38..0x40 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x38, 8) == 1 { for j in 0x40..0x48 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x48..0x50 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x48, 8) == 1 { for j in 0x50..0x58 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x48, 8) == 0 { for j in 0x50..0x58 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x48, 8) == 0 { for j in 0x58..0x60 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x60..0x68 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x60, 8) == 5 { for j in 0x68..0x70 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x60, 8) == 6 { for j in 0x68..0x70 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x60, 8) == 2 { for j in 0x68..0x70 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x60, 8) == 3 { for j in 0x68..0x70 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x60, 8) == 1 { for j in 0x68..0x70 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x60, 8) == 0 { for j in 0x68..0x70 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x88..0x90 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x90..0x91 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x91..0x92 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x92..0x93 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x93..0x94 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x94, 1), 255) { for j in 0x94..0x95 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x94..0x95 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x95, 1), 255) { for j in 0x95..0x96 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x95..0x96 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x95, 1), 255) && rd_le(gb + 0x95, 1) == 11 { for j in 0x96..0x97 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x95, 1), 255) && rd_le(gb + 0x95, 1) == 3 { for j in 0x96..0x97 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x95, 1), 255) && rd_le(gb + 0x95, 1) == 9 { for j in 0x96..0x97 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x95, 1), 255) && rd_le(gb + 0x95, 1) == 8 { for j in 0x96..0x97 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x95, 1), 255) && rd_le(gb + 0x95, 1) == 0 { for j in 0x96..0x97 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x95, 1), 255) && rd_le(gb + 0x95, 1) == 4 { for j in 0x96..0x97 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x95, 1), 255) && rd_le(gb + 0x95, 1) == 5 { for j in 0x96..0x97 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x95, 1), 255) && rd_le(gb + 0x95, 1) == 10 { for j in 0x96..0x97 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x95, 1), 255) && rd_le(gb + 0x95, 1) == 1 { for j in 0x96..0x97 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x95, 1), 255) && rd_le(gb + 0x95, 1) == 6 { for j in 0x96..0x97 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x95, 1), 255) && rd_le(gb + 0x95, 1) == 11 { for j in 0x97..0x98 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x95, 1), 255) && rd_le(gb + 0x95, 1) == 0 { for j in 0x97..0x98 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x95, 1), 255) && rd_le(gb + 0x95, 1) == 1 { for j in 0x97..0x98 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            None
        }
        7 => {   // PassiveJunglePlan · 잎 8
            if true { if (rd_le(gb + 0x20, 8) >> 63) != (rd_le(mb + 0x20, 8) >> 63) { return Some(format!("+{:#x}.opt g={} m={}", 0x20, if rd_le(gb + 0x20, 8) >> 63 != 0 { "None" } else { "Some" }, if rd_le(mb + 0x20, 8) >> 63 != 0 { "None" } else { "Some" })); } }
            if ((rd_le(gb + 0x20, 8) >> 63) == 0) == true { for j in 0x38..0x40 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if ((rd_le(gb + 0x20, 8) >> 63) == 0) == true { for j in 0x40..0x48 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if ((rd_le(gb + 0x20, 8) >> 63) == 0) == true { for j in 0x48..0x49 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x50..0x58 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x58..0x60 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x60..0x68 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x68..0x69 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            None
        }
        9 => {   // BattlePlan · 잎 75
            if true { for j in 0x8..0x10 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x8, 8) == 1 { for j in 0x10..0x18 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x18..0x20 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x18, 8) == 1 { for j in 0x20..0x28 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x18, 8) == 1 { for j in 0x28..0x30 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x18, 8) == 1 { for j in 0x30..0x38 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x38..0x40 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x38, 8) == 1 { for j in 0x40..0x48 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x48..0x50 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x48, 8) == 1 { for j in 0x50..0x58 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x48, 8) == 0 { for j in 0x50..0x58 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x48, 8) == 0 { for j in 0x58..0x60 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x60..0x68 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x60, 8) == 5 { for j in 0x68..0x70 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x60, 8) == 6 { for j in 0x68..0x70 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x60, 8) == 2 { for j in 0x68..0x70 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x60, 8) == 3 { for j in 0x68..0x70 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x60, 8) == 1 { for j in 0x68..0x70 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x60, 8) == 0 { for j in 0x68..0x70 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0xa0..0xa8 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0xa8..0xb0 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0xb0..0xb8 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0xb8..0xc0 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0xc0..0xc8 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0xc8..0xd0 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0xd0..0xd8 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0xd8..0xe0 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0xe0..0xe8 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0xe8..0xf0 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0xf0..0xf8 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0xf8..0xfc { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0xfc, 1), 2) { for j in 0xfc..0xfd { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0xfc..0xfd { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0xfc, 1), 2) { for j in 0xfd..0xfe { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0xfe..0xff { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0xff..0x100 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x100..0x101 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x101..0x102 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x102..0x103 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x103..0x104 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x104..0x105 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x105..0x106 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x106, 1), 255) { for j in 0x106..0x107 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x106..0x107 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x107, 1), 255) { for j in 0x107..0x108 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x107..0x108 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x107, 1), 255) && rd_le(gb + 0x107, 1) == 11 { for j in 0x108..0x109 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x107, 1), 255) && rd_le(gb + 0x107, 1) == 3 { for j in 0x108..0x109 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x107, 1), 255) && rd_le(gb + 0x107, 1) == 9 { for j in 0x108..0x109 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x107, 1), 255) && rd_le(gb + 0x107, 1) == 8 { for j in 0x108..0x109 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x107, 1), 255) && rd_le(gb + 0x107, 1) == 0 { for j in 0x108..0x109 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x107, 1), 255) && rd_le(gb + 0x107, 1) == 4 { for j in 0x108..0x109 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x107, 1), 255) && rd_le(gb + 0x107, 1) == 5 { for j in 0x108..0x109 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x107, 1), 255) && rd_le(gb + 0x107, 1) == 10 { for j in 0x108..0x109 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x107, 1), 255) && rd_le(gb + 0x107, 1) == 1 { for j in 0x108..0x109 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x107, 1), 255) && rd_le(gb + 0x107, 1) == 6 { for j in 0x108..0x109 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x107, 1), 255) && rd_le(gb + 0x107, 1) == 11 { for j in 0x109..0x10a { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x107, 1), 255) && rd_le(gb + 0x107, 1) == 0 { for j in 0x109..0x10a { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x107, 1), 255) && rd_le(gb + 0x107, 1) == 1 { for j in 0x109..0x10a { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x10a..0x10b { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x10b..0x10c { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x10c..0x10d { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x10d..0x10e { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x10e..0x10f { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x10f..0x110 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x110..0x111 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x111..0x112 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x112..0x113 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x113..0x114 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x114..0x115 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x115..0x116 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x116..0x117 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x117..0x118 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x118..0x119 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x119..0x11a { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            None
        }
        10 => {   // LineGankerPlan · 잎 5
            if true { for j in 0x20..0x28 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x28..0x30 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x30..0x31 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x31, 1), 6 | 7 | 8) { for j in 0x31..0x32 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x31..0x32 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            None
        }
        11 => {   // LineGankCoverPlan · 잎 2
            if true { for j in 0x20..0x28 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x28..0x29 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            None
        }
        12 => {   // EpicHuntAndPokePlan · 잎 3
            if true { for j in 0x20..0x21 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x21..0x22 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x22..0x23 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            None
        }
        13 => {   // EpicHuntAndBattlePlan · 잎 2
            if true { for j in 0x8..0x10 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x8, 8) == 1 { for j in 0x10..0x18 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            None
        }
        14 => {   // SerpenHuntAndPokePlan · 잎 3
            if true { for j in 0x20..0x21 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x21..0x22 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x22..0x23 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            None
        }
        15 => {   // SerpenHuntAndBattlePlan · 잎 2
            if true { for j in 0x8..0x10 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x8, 8) == 1 { for j in 0x10..0x18 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            None
        }
        16 => {   // AttackNexusPlan · 잎 2
            if true { for j in 0x8..0x10 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x10..0x11 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            None
        }
        17 => {   // DefenseNexusPlan · 잎 1
            if true { for j in 0x8..0x10 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            None
        }
        t if !(2..=17).contains(&t) => {   // DeathMatchBattle · 잎 77 · untagged(페이로드 base 0)
            if true { for j in 0x0..0x8 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x0, 8) == 1 { for j in 0x8..0x10 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x10..0x18 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x10, 8) == 1 { for j in 0x18..0x20 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x10, 8) == 1 { for j in 0x20..0x28 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x10, 8) == 1 { for j in 0x28..0x30 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x30..0x38 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x30, 8) == 1 { for j in 0x38..0x40 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x40..0x48 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x40, 8) == 1 { for j in 0x48..0x50 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x50..0x58 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x50, 8) == 1 { for j in 0x58..0x60 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x50, 8) == 1 { for j in 0x60..0x68 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x50, 8) == 1 { for j in 0x68..0x70 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x70..0x78 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x70, 8) == 1 { for j in 0x78..0x80 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x70, 8) == 1 { for j in 0x80..0x88 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x70, 8) == 1 { for j in 0x88..0x90 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x90..0x98 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x90, 8) == 1 { for j in 0x98..0xa0 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x90, 8) == 1 { for j in 0xa0..0xa8 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0xa8..0xb0 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0xa8, 8) == 1 { for j in 0xb0..0xb8 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0xa8, 8) == 1 { for j in 0xb8..0xc0 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0xc0..0xc8 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0xc0, 8) == 1 { for j in 0xc8..0xd0 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0xd0..0xd8 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0xd0, 8) == 1 { for j in 0xd8..0xe0 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0xd0, 8) == 0 { for j in 0xd8..0xe0 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0xd0, 8) == 0 { for j in 0xe0..0xe8 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0xe8..0xf0 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0xe8, 8) == 5 { for j in 0xf0..0xf8 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0xe8, 8) == 6 { for j in 0xf0..0xf8 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0xe8, 8) == 2 { for j in 0xf0..0xf8 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0xe8, 8) == 3 { for j in 0xf0..0xf8 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0xe8, 8) == 1 { for j in 0xf0..0xf8 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0xe8, 8) == 0 { for j in 0xf0..0xf8 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x110..0x118 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x118..0x120 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x120..0x128 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x128..0x130 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x130..0x138 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x138..0x140 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x140..0x148 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x148..0x150 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x150..0x158 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x158..0x160 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x160..0x168 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x168..0x170 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x170..0x171 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x171..0x172 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x172..0x173 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x173..0x174 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x174..0x175 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x175..0x176 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x176..0x177 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x177..0x178 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x178..0x179 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x179..0x17a { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x17a, 1), 255) { for j in 0x17a..0x17b { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x17a..0x17b { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x17b, 1), 255) { for j in 0x17b..0x17c { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x17b..0x17c { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x17b, 1), 255) && rd_le(gb + 0x17b, 1) == 11 { for j in 0x17c..0x17d { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x17b, 1), 255) && rd_le(gb + 0x17b, 1) == 3 { for j in 0x17c..0x17d { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x17b, 1), 255) && rd_le(gb + 0x17b, 1) == 9 { for j in 0x17c..0x17d { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x17b, 1), 255) && rd_le(gb + 0x17b, 1) == 8 { for j in 0x17c..0x17d { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x17b, 1), 255) && rd_le(gb + 0x17b, 1) == 0 { for j in 0x17c..0x17d { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x17b, 1), 255) && rd_le(gb + 0x17b, 1) == 4 { for j in 0x17c..0x17d { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x17b, 1), 255) && rd_le(gb + 0x17b, 1) == 5 { for j in 0x17c..0x17d { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x17b, 1), 255) && rd_le(gb + 0x17b, 1) == 10 { for j in 0x17c..0x17d { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x17b, 1), 255) && rd_le(gb + 0x17b, 1) == 1 { for j in 0x17c..0x17d { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x17b, 1), 255) && rd_le(gb + 0x17b, 1) == 6 { for j in 0x17c..0x17d { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x17b, 1), 255) && rd_le(gb + 0x17b, 1) == 11 { for j in 0x17d..0x17e { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x17b, 1), 255) && rd_le(gb + 0x17b, 1) == 0 { for j in 0x17d..0x17e { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x17b, 1), 255) && rd_le(gb + 0x17b, 1) == 1 { for j in 0x17d..0x17e { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x17e..0x17f { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            None
        }
        _ => None,   // 단위 variant(페이로드 0B)
    }
}
/// `#02` self+0x0 `SubPlan` — variant 별 페이로드 live 비교(structlive.py 자동 생성 · variant 13개 · 태그 2..=18 · 밖 = untagged).
/// variant 히스토그램 — [0] = untagged(범위 밖) · [t] = 메모리태그 t (대조 1건당 1증가 · 단위 variant 포함).
pub static EH_2_0: [AtomicU64; 32] = [const { AtomicU64::new(0) }; 32];
pub static EH_2_0_NAMES: &[(u64, &str)] = &[(2, "LineDefense"), (3, "LineSafe"), (4, "LineWait"), (5, "Recall"), (6, "Jungle"), (7, "Battle"), (9, "Hide"), (10, "EpicCheck"), (11, "EpicHunt"), (12, "EpicPoke"), (13, "SerpenCheck"), (14, "SerpenHunt"), (15, "SerpenPoke"), (16, "AttackNexus"), (17, "DefenseNexus"), (18, "Steal"), (0, "DeathBattle")];
pub unsafe fn enumlive_cmp_2_0(tag: u64, gb: usize, mb: usize) -> Option<String> {
    EH_2_0[if (2..=18).contains(&tag) { tag as usize } else { 0 }].fetch_add(1, Ordering::Relaxed);
    match tag {
        2 => {   // LineDefenseSubPlan · 잎 3
            if true { for j in 0x8..0x9 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x9..0xa { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0xa..0xb { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            None
        }
        3 => {   // LineSafeSubPlan · 잎 1
            if true { for j in 0x8..0x9 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            None
        }
        4 => {   // LineWaitSubPlan · 잎 1
            if true { for j in 0x8..0x9 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            None
        }
        6 => {   // JungleSubPlan · 잎 3
            if true { for j in 0x8..0x10 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x10..0x11 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x11..0x12 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            None
        }
        7 => {   // BattleSubPlan · 잎 16
            if true { for j in 0x8..0x10 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x8, 8) == 1 { for j in 0x10..0x18 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x18..0x20 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x18, 8) == 5 { for j in 0x20..0x28 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x18, 8) == 6 { for j in 0x20..0x28 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x18, 8) == 2 { for j in 0x20..0x28 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x18, 8) == 3 { for j in 0x20..0x28 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x18, 8) == 1 { for j in 0x20..0x28 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x18, 8) == 0 { for j in 0x20..0x28 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x28..0x30 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x30..0x31 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x31..0x32 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x32..0x33 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x33..0x34 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x34..0x35 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x35..0x36 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            None
        }
        9 => {   // HideSubPlan · 잎 4
            if true { for j in 0x8..0x10 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x10..0x11 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x11..0x12 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x12..0x13 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            None
        }
        10 => {   // EpicCheckSubPlan · 잎 1
            if true { for j in 0x8..0x9 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            None
        }
        11 => {   // EpicHuntSubPlan · 잎 1
            if true { for j in 0x8..0x9 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            None
        }
        13 => {   // SerpenCheckSubPlan · 잎 1
            if true { for j in 0x8..0x9 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            None
        }
        14 => {   // SerpenHuntSubPlan · 잎 1
            if true { for j in 0x8..0x9 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            None
        }
        17 => {   // DefenseNexusSubPlan · 잎 3
            if true { for j in 0x8..0x10 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x8, 8) == 1 { for j in 0x10..0x18 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x18..0x19 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            None
        }
        18 => {   // StealSubPlan · 잎 4
            if true { for j in 0x8..0x10 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x10, 1), 2) { for j in 0x10..0x11 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x10..0x11 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x11..0x12 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            None
        }
        t if !(2..=18).contains(&t) => {   // DeathBattleSubPlan · 잎 19 · untagged(페이로드 base 0)
            if true { for j in 0x0..0x8 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x0, 8) == 1 { for j in 0x8..0x10 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x10..0x18 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x10, 8) == 1 { for j in 0x18..0x20 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x10, 8) == 1 { for j in 0x20..0x28 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x28..0x30 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x28, 8) == 5 { for j in 0x30..0x38 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x28, 8) == 6 { for j in 0x30..0x38 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x28, 8) == 2 { for j in 0x30..0x38 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x28, 8) == 3 { for j in 0x30..0x38 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x28, 8) == 1 { for j in 0x30..0x38 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x28, 8) == 0 { for j in 0x30..0x38 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x38..0x40 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x40..0x41 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x41..0x42 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x42..0x43 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x43..0x44 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x44..0x45 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x45..0x46 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            None
        }
        _ => None,   // 단위 variant(페이로드 0B)
    }
}
/// `#56` self+0x0 `SubPlan` — variant 별 페이로드 live 비교(structlive.py 자동 생성 · variant 13개 · 태그 2..=18 · 밖 = untagged).
/// variant 히스토그램 — [0] = untagged(범위 밖) · [t] = 메모리태그 t (대조 1건당 1증가 · 단위 variant 포함).
pub static EH_56_0: [AtomicU64; 32] = [const { AtomicU64::new(0) }; 32];
pub static EH_56_0_NAMES: &[(u64, &str)] = &[(2, "LineDefense"), (3, "LineSafe"), (4, "LineWait"), (5, "Recall"), (6, "Jungle"), (7, "Battle"), (9, "Hide"), (10, "EpicCheck"), (11, "EpicHunt"), (12, "EpicPoke"), (13, "SerpenCheck"), (14, "SerpenHunt"), (15, "SerpenPoke"), (16, "AttackNexus"), (17, "DefenseNexus"), (18, "Steal"), (0, "DeathBattle")];
pub unsafe fn enumlive_cmp_56_0(tag: u64, gb: usize, mb: usize) -> Option<String> {
    EH_56_0[if (2..=18).contains(&tag) { tag as usize } else { 0 }].fetch_add(1, Ordering::Relaxed);
    match tag {
        2 => {   // LineDefenseSubPlan · 잎 3
            if true { for j in 0x8..0x9 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x9..0xa { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0xa..0xb { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            None
        }
        3 => {   // LineSafeSubPlan · 잎 1
            if true { for j in 0x8..0x9 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            None
        }
        4 => {   // LineWaitSubPlan · 잎 1
            if true { for j in 0x8..0x9 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            None
        }
        6 => {   // JungleSubPlan · 잎 3
            if true { for j in 0x8..0x10 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x10..0x11 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x11..0x12 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            None
        }
        7 => {   // BattleSubPlan · 잎 16
            if true { for j in 0x8..0x10 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x8, 8) == 1 { for j in 0x10..0x18 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x18..0x20 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x18, 8) == 5 { for j in 0x20..0x28 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x18, 8) == 6 { for j in 0x20..0x28 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x18, 8) == 2 { for j in 0x20..0x28 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x18, 8) == 3 { for j in 0x20..0x28 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x18, 8) == 1 { for j in 0x20..0x28 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x18, 8) == 0 { for j in 0x20..0x28 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x28..0x30 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x30..0x31 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x31..0x32 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x32..0x33 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x33..0x34 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x34..0x35 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x35..0x36 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            None
        }
        9 => {   // HideSubPlan · 잎 4
            if true { for j in 0x8..0x10 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x10..0x11 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x11..0x12 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x12..0x13 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            None
        }
        10 => {   // EpicCheckSubPlan · 잎 1
            if true { for j in 0x8..0x9 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            None
        }
        11 => {   // EpicHuntSubPlan · 잎 1
            if true { for j in 0x8..0x9 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            None
        }
        13 => {   // SerpenCheckSubPlan · 잎 1
            if true { for j in 0x8..0x9 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            None
        }
        14 => {   // SerpenHuntSubPlan · 잎 1
            if true { for j in 0x8..0x9 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            None
        }
        17 => {   // DefenseNexusSubPlan · 잎 3
            if true { for j in 0x8..0x10 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x8, 8) == 1 { for j in 0x10..0x18 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x18..0x19 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            None
        }
        18 => {   // StealSubPlan · 잎 4
            if true { for j in 0x8..0x10 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if !matches!(rd_le(gb + 0x10, 1), 2) { for j in 0x10..0x11 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x10..0x11 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x11..0x12 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            None
        }
        t if !(2..=18).contains(&t) => {   // DeathBattleSubPlan · 잎 19 · untagged(페이로드 base 0)
            if true { for j in 0x0..0x8 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x0, 8) == 1 { for j in 0x8..0x10 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x10..0x18 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x10, 8) == 1 { for j in 0x18..0x20 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x10, 8) == 1 { for j in 0x20..0x28 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x28..0x30 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x28, 8) == 5 { for j in 0x30..0x38 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x28, 8) == 6 { for j in 0x30..0x38 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x28, 8) == 2 { for j in 0x30..0x38 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x28, 8) == 3 { for j in 0x30..0x38 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x28, 8) == 1 { for j in 0x30..0x38 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if rd_le(gb + 0x28, 8) == 0 { for j in 0x30..0x38 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x38..0x40 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x40..0x41 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x41..0x42 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x42..0x43 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x43..0x44 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x44..0x45 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            if true { for j in 0x45..0x46 { let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8)); if gv != mv { return Some(format!("+{:#x}: g={:02x} m={:02x}", j, gv, mv)); } } }
            None
        }
        _ => None,   // 단위 variant(페이로드 0B)
    }
}

unsafe fn w_58(a0: i64, a1: *const u8, a2: *const u8) -> u8 {
    S[0].calls.fetch_add(1, Ordering::Relaxed);
    let f: unsafe fn(i64, *const u8, *const u8) -> u8 = core::mem::transmute(S[0].orig.load(Ordering::Relaxed));
    let t = top(0);
    let g = f(a0, a1, a2);
    if !t { pop(0); return g; }
    let n = S[0].cmp.fetch_add(1, Ordering::Relaxed) + 1;
    let m = catch_unwind(AssertUnwindSafe(|| my_58(a0, a1, a2)));
    match m {
        Ok(m) => { if m != g { note(0, format!("#58 v3_epic_group_line 대조#{} 갈림: g={:?} m={:?} | a0={} a1={:#x} a2={:#x}", n, g, m, a0, a1 as usize, a2 as usize)); } }
        Err(_) => { S[0].pan.fetch_add(1, Ordering::Relaxed); }
    }
    pop(0);
    g
}
unsafe fn w_59(a0: i64, a1: *const u8, a2: *const u8) -> u8 {
    S[1].calls.fetch_add(1, Ordering::Relaxed);
    let f: unsafe fn(i64, *const u8, *const u8) -> u8 = core::mem::transmute(S[1].orig.load(Ordering::Relaxed));
    let t = top(1);
    let g = f(a0, a1, a2);
    if !t { pop(1); return g; }
    let n = S[1].cmp.fetch_add(1, Ordering::Relaxed) + 1;
    let m = catch_unwind(AssertUnwindSafe(|| my_59(a0, a1, a2)));
    match m {
        Ok(m) => { if m != g { note(1, format!("#59 v3_epicops_repair_need 대조#{} 갈림: g={:?} m={:?} | a0={} a1={:#x} a2={:#x}", n, g, m, a0, a1 as usize, a2 as usize)); } }
        Err(_) => { S[1].pan.fetch_add(1, Ordering::Relaxed); }
    }
    pop(1);
    g
}
unsafe fn w_60(a0: *const u8, a1: *const u8, a2: *const u8, a3: *const u8, a4: u8) -> bool {
    S[2].calls.fetch_add(1, Ordering::Relaxed);
    let f: unsafe fn(*const u8, *const u8, *const u8, *const u8, u8) -> bool = core::mem::transmute(S[2].orig.load(Ordering::Relaxed));
    let t = top(2);
    let g = f(a0, a1, a2, a3, a4);
    if !t { pop(2); return g; }
    let n = S[2].cmp.fetch_add(1, Ordering::Relaxed) + 1;
    let m = catch_unwind(AssertUnwindSafe(|| my_60(a0, a1, a2, a3, a4)));
    match m {
        Ok(m) => { if m != g { note(2, format!("#60 is_object_being_taken_by_enemy 대조#{} 갈림: g={:?} m={:?} | a0={:#x} a1={:#x} a2={:#x} a3={:#x} a4={}", n, g, m, a0 as usize, a1 as usize, a2 as usize, a3 as usize, a4)); } }
        Err(_) => { S[2].pan.fetch_add(1, Ordering::Relaxed); }
    }
    pop(2);
    g
}
unsafe fn w_61(a0: i64, a1: *const u8, a2: *const u8, a3: *const u8) -> bool {
    S[3].calls.fetch_add(1, Ordering::Relaxed);
    let f: unsafe fn(i64, *const u8, *const u8, *const u8) -> bool = core::mem::transmute(S[3].orig.load(Ordering::Relaxed));
    let t = top(3);
    // ★★&mut 게임 상태(1064B) — **게임 호출 전** 상태를 떠 두지 않으면 내 사본은
    //   게임이 바꿔놓은 입력을 보게 돼 **거짓 DIFF** 가 난다(이 함수는 self 를 읽고도 쓴다).
    // ★★**스택이 아니라 thread_local 힙**에 둔다 — 1064B × 2 를 스택에 잡으면
    //   rayon 워커의 **소스택 + 게임 sim 재귀**에서 STATUS_STACK_OVERFLOW 로 죽는다
    //   (2026-09-12 실사고: 스택 배열로 두던 판이 배경 sim 시작 직후 패닉로그 없이 즉사).
    SV3.with(|c| { let sv = &mut *c.get();
        if t { core::ptr::copy_nonoverlapping(a3, sv.as_mut_ptr(), 1064); } });
    VB3.with(|c| { let vb = &mut *c.get(); if t {
        let ln = core::ptr::read_unaligned((a3 as usize + 0xd0) as *const usize);
        let pz = core::ptr::read_unaligned((a3 as usize + 0xc8) as *const usize);
        vb.1 = ln; vb.2 = ln + 2 <= 512;
        if ln > 0 && ln + 2 <= 512 && pz > 0x1000 {
            core::ptr::copy_nonoverlapping(pz as *const u8, vb.0.as_mut_ptr(), ln * 24);
        } else { vb.1 = ln; vb.2 = ln + 2 <= 512; }
    } });
    let g = f(a0, a1, a2, a3);
    if !t { pop(3); return g; }
    // ★내 Vec 사본 버퍼에 안 들어간 호출은 **표본에서 뺀다** — 예전엔 `len=0` 으로
    //   떨어뜨려 **입력이 어긋난 채 비교**했고 그게 거짓 DIFF 였다(2026-09-12 차단).
    if !VB3.with(|c| (&*c.get()).2) { S[3].skip.fetch_add(1, Ordering::Relaxed); pop(3); return g; }
    let n = S[3].cmp.fetch_add(1, Ordering::Relaxed) + 1;
    SP3.with(|c| { let sp = &mut *c.get();
        core::ptr::copy_nonoverlapping(a3, sp.as_mut_ptr(), 1064); });   // 게임 호출 후 상태
    SV3.with(|c| { let sv = &*c.get();
        core::ptr::copy_nonoverlapping(sv.as_ptr(), a3 as *mut u8, 1064); }); // 내 사본 호출 전 = 호출 전 상태로
    // ★Vec 을 **빈 것**으로 — cap=0 이면 push 가 realloc 이 아니라 alloc 을 하므로
    //   **게임의 버퍼를 절대 해제하지 않는다**(순서는 IR 실측: cap@0xc0 · ptr@0xc8 · len@0xd0).
    VB3.with(|c| { let vb = &*c.get();
        core::ptr::write_unaligned((a3 as usize + 0xc0) as *mut usize, 512);  // cap = 내 버퍼 용량
        core::ptr::write_unaligned((a3 as usize + 0xc8) as *mut usize, vb.0.as_ptr() as usize); // ptr = 내 버퍼
        core::ptr::write_unaligned((a3 as usize + 0xd0) as *mut usize, vb.1);  // len = 게임과 **같은** 개수
    });
    let m = catch_unwind(AssertUnwindSafe(|| my_61(a0, a1, a2, a3)));
    // ★내 사본이 할당한 것을 해제한다(안 하면 호출당 누수). rlib 은 내 DLL 안에 링크돼 있어 알로케이터가 같다.
    { let c = core::ptr::read_unaligned((a3 as usize + 0xc0) as *const usize);
      let pz = core::ptr::read_unaligned((a3 as usize + 0xc8) as *const usize);
      let mine = VB3.with(|c2| (&*c2.get()).0.as_ptr() as usize);
      // ★내 버퍼면 해제하면 안 된다(thread_local 정적) — 재할당된 경우에만 해제.
      if c > 0 && pz > 0x1000 && pz != mine { if let Ok(l) = std::alloc::Layout::from_size_align(c * 24, 8) {
          std::alloc::dealloc(pz as *mut u8, l); } } }
    SP3.with(|c| { let sp = &*c.get();
        core::ptr::copy_nonoverlapping(sp.as_ptr(), a3 as *mut u8, 1064); }); // 게임 호출 후 상태로 복구
    match m {
        Ok(m) => { if m != g { note(3, format!("#61 v3_serpen_contest_clear_win 대조#{} 갈림: g={:?} m={:?} | a0={} a1={:#x} a2={:#x} a3={:#x}", n, g, m, a0, a1 as usize, a2 as usize, a3 as usize)); } }
        Err(_) => { S[3].pan.fetch_add(1, Ordering::Relaxed); }
    }
    pop(3);
    g
}
unsafe fn w_62(a0: *const u8, a1: i64, a2: *const u8, a3: *const u8, a4: *const u8, a5: *const u8, a6: i64, a7: *const u8, a8: i64, a9: u8, a10: *const u8, a11: i64, a12: *const u8, a13: i64, a14: i64) {
    S[4].calls.fetch_add(1, Ordering::Relaxed);
    let f: unsafe fn(*const u8, i64, *const u8, *const u8, *const u8, *const u8, i64, *const u8, i64, u8, *const u8, i64, *const u8, i64, i64) = core::mem::transmute(S[4].orig.load(Ordering::Relaxed));
    let t = top(4);
    let g = f(a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12, a13, a14);
    if !t { pop(4); return g; }
    let n = S[4].cmp.fetch_add(1, Ordering::Relaxed) + 1;
    let mut gb = [0u64; 8]; core::ptr::copy_nonoverlapping(a0, gb.as_mut_ptr() as *mut u8, 64); // 게임 출력 사본
    let mut mb = [0u64; 8];                                                   // 내 사본 전용 출력 버퍼
    let m = catch_unwind(AssertUnwindSafe(|| my_62(mb.as_mut_ptr() as *const u8, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12, a13, a14)));
    match m {
        Ok(_) => { if let Some(off) = live_eq(a0, mb.as_ptr() as *const u8, LIVE_62) {
            note(4, format!("#62 resolve_fight_uncached 대조#{} 갈림(sret 64B · +{:#x}): g={:02x?} m={:02x?} | a1={} a2={:#x} a3={:#x} a4={:#x} a5={:#x} a6={} a7={:#x} a8={} a9={} a10={:#x} a11={} a12={:#x} a13={} a14={}", n, off, &gb[..8], &mb[..8], a1, a2 as usize, a3 as usize, a4 as usize, a5 as usize, a6, a7 as usize, a8, a9, a10 as usize, a11, a12 as usize, a13, a14)); } }
        Err(_) => { S[4].pan.fetch_add(1, Ordering::Relaxed); }
    }
    pop(4);
    g
}
unsafe fn w_30(a0: *const u8, a1: u8) -> bool {
    S[5].calls.fetch_add(1, Ordering::Relaxed);
    let f: unsafe fn(*const u8, u8) -> bool = core::mem::transmute(S[5].orig.load(Ordering::Relaxed));
    let t = top(5);
    let g = f(a0, a1);
    if !t { pop(5); return g; }
    let n = S[5].cmp.fetch_add(1, Ordering::Relaxed) + 1;
    let m = catch_unwind(AssertUnwindSafe(|| my_30(a0, a1)));
    match m {
        Ok(m) => { if m != g { note(5, format!("#30 objective_is_damaged 대조#{} 갈림: g={:?} m={:?} | a0={:#x} a1={}", n, g, m, a0 as usize, a1)); } }
        Err(_) => { S[5].pan.fetch_add(1, Ordering::Relaxed); }
    }
    pop(5);
    g
}
unsafe fn w_32(a0: *const u8, a1: *const u8) -> u8 {
    S[6].calls.fetch_add(1, Ordering::Relaxed);
    let f: unsafe fn(*const u8, *const u8) -> u8 = core::mem::transmute(S[6].orig.load(Ordering::Relaxed));
    let t = top(6);
    let g = f(a0, a1);
    if !t { pop(6); return g; }
    let n = S[6].cmp.fetch_add(1, Ordering::Relaxed) + 1;
    let m = catch_unwind(AssertUnwindSafe(|| my_32(a0, a1)));
    match m {
        Ok(m) => { if m != g { note(6, format!("#32 serpen_giveup_chat_reason 대조#{} 갈림: g={:?} m={:?} | a0={:#x} a1={:#x}", n, g, m, a0 as usize, a1 as usize)); } }
        Err(_) => { S[6].pan.fetch_add(1, Ordering::Relaxed); }
    }
    pop(6);
    g
}
unsafe fn w_5(a0: *const u8, a1: u8, a2: u8) {
    S[7].calls.fetch_add(1, Ordering::Relaxed);
    let f: unsafe fn(*const u8, u8, u8) = core::mem::transmute(S[7].orig.load(Ordering::Relaxed));
    let t = top(7);
    // ★★&mut 게임 상태(6168B) — **게임 호출 전** 상태를 떠 두지 않으면 내 사본은
    //   게임이 바꿔놓은 입력을 보게 돼 **거짓 DIFF** 가 난다(이 함수는 self 를 읽고도 쓴다).
    // ★★**스택이 아니라 thread_local 힙**에 둔다 — 6168B × 2 를 스택에 잡으면
    //   rayon 워커의 **소스택 + 게임 sim 재귀**에서 STATUS_STACK_OVERFLOW 로 죽는다
    //   (2026-09-12 실사고: 스택 배열로 두던 판이 배경 sim 시작 직후 패닉로그 없이 즉사).
    SV7.with(|c| { let sv = &mut *c.get();
        if t { core::ptr::copy_nonoverlapping(a0, sv.as_mut_ptr(), 6168); } });
    VB7.with(|c| { let vb = &mut *c.get(); if t {
        let ln = core::ptr::read_unaligned((a0 as usize + 0x898) as *const usize);
        let pz = core::ptr::read_unaligned((a0 as usize + 0x890) as *const usize);
        vb.1 = ln; vb.2 = ln + 2 <= 118;
        if ln > 0 && ln + 2 <= 118 && pz > 0x1000 {
            core::ptr::copy_nonoverlapping(pz as *const u8, vb.0.as_mut_ptr(), ln * 104);
        } else { vb.1 = ln; vb.2 = ln + 2 <= 118; }
    } });
    let g = f(a0, a1, a2);
    if !t { pop(7); return g; }
    // ★내 Vec 사본 버퍼에 안 들어간 호출은 **표본에서 뺀다** — 예전엔 `len=0` 으로
    //   떨어뜨려 **입력이 어긋난 채 비교**했고 그게 거짓 DIFF 였다(2026-09-12 차단).
    if !VB7.with(|c| (&*c.get()).2) { S[7].skip.fetch_add(1, Ordering::Relaxed); pop(7); return g; }
    let n = S[7].cmp.fetch_add(1, Ordering::Relaxed) + 1;
    SP7.with(|c| { let sp = &mut *c.get();
        core::ptr::copy_nonoverlapping(a0, sp.as_mut_ptr(), 6168); });   // 게임 호출 후 상태
    SV7.with(|c| { let sv = &*c.get();
        core::ptr::copy_nonoverlapping(sv.as_ptr(), a0 as *mut u8, 6168); }); // 내 사본 호출 전 = 호출 전 상태로
    // ★Vec 을 **빈 것**으로 — cap=0 이면 push 가 realloc 이 아니라 alloc 을 하므로
    //   **게임의 버퍼를 절대 해제하지 않는다**(순서는 IR 실측: cap@0x888 · ptr@0x890 · len@0x898).
    VB7.with(|c| { let vb = &*c.get();
        core::ptr::write_unaligned((a0 as usize + 0x888) as *mut usize, 118);  // cap = 내 버퍼 용량
        core::ptr::write_unaligned((a0 as usize + 0x890) as *mut usize, vb.0.as_ptr() as usize); // ptr = 내 버퍼
        core::ptr::write_unaligned((a0 as usize + 0x898) as *mut usize, vb.1);  // len = 게임과 **같은** 개수
    });
    let m = catch_unwind(AssertUnwindSafe(|| my_5(a0, a1, a2)));
    // ★★★상태 diff — 이 함수는 **반환이 void** 다. ABI 상 반환이 없는 것이지
    //   **출력이 없는 게 아니다** — 출력은 `&mut self` 에 있다 ⟹ 그걸 비교한다.
    //   ⚠되돌리기·해제 **전에** 떠서 비교한다 — 내 사본이 재할당했으면 해제 후엔 못 읽는다.
    let (m_ptr, m_len) = (core::ptr::read_unaligned((a0 as usize + 0x890) as *const usize),
                          core::ptr::read_unaligned((a0 as usize + 0x898) as *const usize));
    SQ7.with(|c| { let sq = &mut *c.get();
        core::ptr::copy_nonoverlapping(a0, sq.as_mut_ptr(), 6168); });
    let sd: Option<String> = SP7.with(|c| { let sp = &*c.get(); SQ7.with(|c2| { let sq = &*c2.get();
        // ① 본체 바이트(설계상 다른 구간은 제외)
        for off in 0..6168usize {
            if off >= 0x888 && off < 0x8a0 { continue; }
            if sp[off] != sq[off] {
                return Some(format!("self+{:#x}: g={:02x} m={:02x}", off, sp[off], sq[off]));
            }
        }
        // ② Vec 의 len (cap/ptr 은 버퍼가 달라 비교 대상이 아니다)
        let g_ptr = core::ptr::read_unaligned(sp.as_ptr().add(0x890) as *const usize);
        let g_len = core::ptr::read_unaligned(sp.as_ptr().add(0x898) as *const usize);
        if g_len != m_len {
            return Some(format!("vec.len: g={} m={}", g_len, m_len));
        }
        // ③ Vec 의 **내용** (요소 104B)
        //   ★요소는 열거형이라 **variant 별로 쓰는 칸이 다르다** — 안 쓰는 칸은
        //   **재사용된 버퍼의 잔재**라 양쪽이 다른 게 정상이다(IR 실측 기반 live 범위만 본다).
        //   (off, len, 이 tag 들에서만 live · 빈 것 = 항상)
        const EL: &[(usize, usize, &[u8])] = &[(0, 98, &[])];
        if g_len > 0 && g_len < 4096 && g_ptr > 0x1000 && m_ptr > 0x1000 {
            for e in 0..g_len {
                let (gb, mb) = (g_ptr + e * 104, m_ptr + e * 104);
                let tag = *(gb as *const u8);
                for &(o, l, tg) in EL {
                    if !tg.is_empty() && !tg.contains(&tag) { continue; }
                    for j in o..o + l {
                        let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8));
                        if gv != mv {
                            return Some(format!("vec[{}](tag {:#x})+{}: g={:02x} m={:02x}", e, tag, j, gv, mv));
                        }
                    }
                }
            }
        }
        None
    }) });
    // ★내 사본이 할당한 것을 해제한다(안 하면 호출당 누수). rlib 은 내 DLL 안에 링크돼 있어 알로케이터가 같다.
    { let c = core::ptr::read_unaligned((a0 as usize + 0x888) as *const usize);
      let pz = core::ptr::read_unaligned((a0 as usize + 0x890) as *const usize);
      let mine = VB7.with(|c2| (&*c2.get()).0.as_ptr() as usize);
      // ★내 버퍼면 해제하면 안 된다(thread_local 정적) — 재할당된 경우에만 해제.
      if c > 0 && pz > 0x1000 && pz != mine { if let Ok(l) = std::alloc::Layout::from_size_align(c * 104, 8) {
          std::alloc::dealloc(pz as *mut u8, l); } } }
    SP7.with(|c| { let sp = &*c.get();
        core::ptr::copy_nonoverlapping(sp.as_ptr(), a0 as *mut u8, 6168); }); // 게임 호출 후 상태로 복구
    match m {
        Ok(_) => { if let Some(d) = sd {
            note(7, format!("#05 v50_fold_dive_episode 대조#{} **상태갈림**: {} | (반환 void — 출력은 &mut self 다) | a0={:#x} a1={} a2={}", n, d, a0 as usize, a1, a2)); } }
        Err(_) => { S[7].pan.fetch_add(1, Ordering::Relaxed); }
    }
    pop(7);
    g
}
unsafe fn w_11(a0: *const u8, a1: i64, a2: *const u8, a3: *const u8, a4: *const u8, a5: *const u8) {
    S[8].calls.fetch_add(1, Ordering::Relaxed);
    let f: unsafe fn(*const u8, i64, *const u8, *const u8, *const u8, *const u8) = core::mem::transmute(S[8].orig.load(Ordering::Relaxed));
    let t = top(8);
    // ★★&mut 게임 상태(6168B) — **게임 호출 전** 상태를 떠 두지 않으면 내 사본은
    //   게임이 바꿔놓은 입력을 보게 돼 **거짓 DIFF** 가 난다(이 함수는 self 를 읽고도 쓴다).
    // ★★**스택이 아니라 thread_local 힙**에 둔다 — 6168B × 2 를 스택에 잡으면
    //   rayon 워커의 **소스택 + 게임 sim 재귀**에서 STATUS_STACK_OVERFLOW 로 죽는다
    //   (2026-09-12 실사고: 스택 배열로 두던 판이 배경 sim 시작 직후 패닉로그 없이 즉사).
    SV8.with(|c| { let sv = &mut *c.get();
        if t { core::ptr::copy_nonoverlapping(a0, sv.as_mut_ptr(), 6168); } });
    VB8.with(|c| { (&mut *c.get()).2 = true; });   // 치환할 Vec 이 없다 = 항상 표본
    // ★명세 0: self+0x5e8 의 소유 Vec **내용을 게임 호출 전에** 떠 둔다 — 게임이 해제/재할당할 수 있다.
    PV8_0.with(|c| { let pv = &mut *c.get(); pv.2 = 0; pv.3 = true; if t {
        let tg = core::ptr::read_unaligned((a0 as usize + 0x5e8) as *const u64);
        let mut used = 0usize;
        for (i, &(off, esz, _)) in hs_vecs_11_0(tg).iter().enumerate() {
            if i >= 8 { pv.3 = false; break; }
            let b = a0 as usize + 0x5e8 + off;
            let (cap, ptr, len) = (core::ptr::read_unaligned(b as *const usize),
                                   core::ptr::read_unaligned((b + 8) as *const usize),
                                   core::ptr::read_unaligned((b + 16) as *const usize));
            pv.2 = i + 1;
            if ptr > 0x1000 && len <= cap && cap < (1 << 20) {
                if used + len * esz > 32768 { pv.3 = false; break; }
                if len > 0 { core::ptr::copy_nonoverlapping(ptr as *const u8, pv.0.as_mut_ptr().add(used), len * esz); }
                pv.1[i] = (cap, len, esz); used += len * esz;
            } else { pv.1[i] = (0, 0, esz); }   // 비정상 삼중항 = 치환 안 함
        }
    } });
    // ★StdRng(320B) — 게임 호출이 난수열을 소비하므로 **먼저 떠 둔다**(안 그러면 거짓 DIFF).
    let mut r2 = [0u8; 320]; if t { core::ptr::copy_nonoverlapping(a2, r2.as_mut_ptr(), 320); }
    let g = f(a0, a1, a2, a3, a4, a5);
    if !t { pop(8); return g; }
    // ★내 Vec 사본 버퍼에 안 들어간 호출은 **표본에서 뺀다** — 예전엔 `len=0` 으로
    //   떨어뜨려 **입력이 어긋난 채 비교**했고 그게 거짓 DIFF 였다(2026-09-12 차단).
    if !VB8.with(|c| (&*c.get()).2) { S[8].skip.fetch_add(1, Ordering::Relaxed); pop(8); return g; }
    let n = S[8].cmp.fetch_add(1, Ordering::Relaxed) + 1;
    let mut p2 = [0u8; 320]; core::ptr::copy_nonoverlapping(a2, p2.as_mut_ptr(), 320); // 게임 호출 후 상태
    core::ptr::copy_nonoverlapping(r2.as_ptr(), a2 as *mut u8, 320);                    // 내 사본 호출 전 = 호출 전 상태로
    SP8.with(|c| { let sp = &mut *c.get();
        core::ptr::copy_nonoverlapping(a0, sp.as_mut_ptr(), 6168); });   // 게임 호출 후 상태
    SV8.with(|c| { let sv = &*c.get();
        core::ptr::copy_nonoverlapping(sv.as_ptr(), a0 as *mut u8, 6168); }); // 내 사본 호출 전 = 호출 전 상태로
    // ★★★힙 인식 스냅샷 — self 소유 Vec 들을 **내 힙 할당**으로 바꿔치기(내 사본의 drop/realloc 이 내 것에만 닿게).
    let ptag_0: u64 = core::ptr::read_unaligned((a0 as usize + 0x5e8) as *const u64);
    if !PV8_0.with(|c| (&*c.get()).3) {   // 스크래치에 안 들어갔다 = 표본 제외
        S[8].skip.fetch_add(1, Ordering::Relaxed);
        SP8.with(|c| { let sp = &*c.get(); core::ptr::copy_nonoverlapping(sp.as_ptr(), a0 as *mut u8, 6168); });
        core::ptr::copy_nonoverlapping(p2.as_ptr(), a2 as *mut u8, 320);   // RNG = 게임 호출 후
        pop(8); return g;
    }
    PV8_0.with(|c| { let pv = &*c.get(); let mut used = 0usize;
        for (i, &(off, esz, _)) in hs_vecs_11_0(ptag_0).iter().enumerate() {
            if i >= pv.2 { break; }
            let (cap, len, _) = pv.1[i];
            let b = a0 as usize + 0x5e8 + off;
            if cap > 0 && len <= cap {
                // 여유를 둔다 — 내 사본이 push 해도 realloc 없이 들어가게(realloc 도 합법이지만 덜 흔들리게)
                let ncap = cap.max(len + 64);
                if let Ok(l) = std::alloc::Layout::from_size_align(ncap * esz, 8) {
                    let blk = std::alloc::alloc(l);
                    if !blk.is_null() {
                        if len > 0 { core::ptr::copy_nonoverlapping(pv.0.as_ptr().add(used), blk, len * esz); }   // ★게임 호출 전 내용
                        core::ptr::write_unaligned(b as *mut usize, ncap);
                        core::ptr::write_unaligned((b + 8) as *mut usize, blk as usize);
                        core::ptr::write_unaligned((b + 16) as *mut usize, len);
                    }
                }
                used += len * esz;
            } else if cap == 0 {
                // 빈 Vec(cap 0) — 게임 것도 댕글링이라 그대로 둬도 free 는 안 나지만, push 가 alloc 을 부르면
                // 그 결과는 내 것이다(아래 해제가 처리). 그대로 둔다.
            }
        }
    });
    let m = catch_unwind(AssertUnwindSafe(|| my_11(a0, a1, a2, a3, a4, a5)));
    // ★★★상태 diff — 이 함수는 **반환이 void** 다. ABI 상 반환이 없는 것이지
    //   **출력이 없는 게 아니다** — 출력은 `&mut self` 에 있다 ⟹ 그걸 비교한다.
    //   ⚠되돌리기·해제 **전에** 떠서 비교한다 — 내 사본이 재할당했으면 해제 후엔 못 읽는다.
    SQ8.with(|c| { let sq = &mut *c.get();
        core::ptr::copy_nonoverlapping(a0, sq.as_mut_ptr(), 6168); });
    let sd: Option<String> = SP8.with(|c| { let sp = &*c.get(); SQ8.with(|c2| { let sq = &*c2.get();
        // ① 본체 바이트(설계상 다른 구간은 제외)
        // ★소유 Vec 삼중항(cap/ptr/len 24B)은 **동적 skip**(게임 것 vs 내 할당) — len·내용은 ②′에서 비교
        let gtag_0: u64 = core::ptr::read_unaligned(sp.as_ptr().add(0x5e8) as *const u64);
        let dyn_0: &[(usize, usize, u8)] = hs_vecs_11_0(gtag_0);
        for off in 0..6168usize {
            if off >= 0x5f0 && off < 0x768 { continue; }
            if off >= 0x770 && off < 0x7b0 { continue; }
            if dyn_0.iter().any(|&(o, _, _)| off >= 0x5e8 + o && off < 0x5e8 + o + 24) { continue; }
            if sp[off] != sq[off] {
                return Some(format!("self+{:#x}: g={:02x} m={:02x}", off, sp[off], sq[off]));
            }
        }
        // ①′ 열거형 필드 self+0x5e8 `BigPlan` 페이로드 — 타입 기반 live 맵(structlive)으로 variant 조건부 비교
        { let t = core::ptr::read_unaligned(sp.as_ptr().add(0x5e8) as *const u64);
          if let Some(d) = enumlive_cmp_11_0(t, sp.as_ptr() as usize + 0x5e8, sq.as_ptr() as usize + 0x5e8) {
              return Some(format!("self+{:#x}(tag {}){}", 0x5e8, t, d)); } }
        // ①′ 열거형 필드 self+0x768 `SubPlan` 페이로드 — 타입 기반 live 맵(structlive)으로 variant 조건부 비교
        { let t = core::ptr::read_unaligned(sp.as_ptr().add(0x768) as *const u64);
          if let Some(d) = enumlive_cmp_11_1(t, sp.as_ptr() as usize + 0x768, sq.as_ptr() as usize + 0x768) {
              return Some(format!("self+{:#x}(tag {}){}", 0x768, t, d)); } }
        // ②′ 명세 0 소유 Vec 의 len·내용 — 요소는 ELEM_LIVE(live id)로 살아있는 바이트만
        for &(o, esz, lid) in dyn_0 {
            let (gb, mb) = (sp.as_ptr().add(0x5e8 + o), sq.as_ptr().add(0x5e8 + o));
            // ★`Option<Vec>` 은 cap 을 니치로 쓴다(상위비트 = None). 그 경우 len/ptr 은 미초기화 — 읽지 않는다.
            let (gc, mc) = (core::ptr::read_unaligned(gb as *const usize), core::ptr::read_unaligned(mb as *const usize));
            let (gn, mn) = (gc >> 63 != 0, mc >> 63 != 0);
            if gn != mn { return Some(format!("self+{:#x}.opt: g={} m={}", 0x5e8 + o, if gn { "None" } else { "Some" }, if mn { "None" } else { "Some" })); }
            if gn { continue; }
            let (gp, gl) = (core::ptr::read_unaligned(gb.add(8) as *const usize), core::ptr::read_unaligned(gb.add(16) as *const usize));
            let (mp, ml) = (core::ptr::read_unaligned(mb.add(8) as *const usize), core::ptr::read_unaligned(mb.add(16) as *const usize));
            if gl != ml { return Some(format!("self+{:#x}.len: g={} m={}", 0x5e8 + o, gl, ml)); }
            if gl > 0 && gl < 4096 && gp > 0x1000 && mp > 0x1000 && gp != mp {
                for e in 0..gl {
                    if let Some(d) = elem_cmp(lid, esz, gp + e * esz, mp + e * esz) {
                        return Some(format!("self+{:#x}[{}]{}", 0x5e8 + o, e, d));
                    }
                }
            }
        }
        None
    }) });
    // ★내 사본이 남긴 소유 Vec 을 해제한다 — 이 시점에 그 포인터는 **전부 내 것**이다
    //   (게임 것은 위에서 내 할당으로 바꿔치기됐고, 새로 만든 것은 내 사본이 할당했다).
    //   ⚠먼저 **내 사본이 새로 push 한 요소**의 String(ELEM_LIVE.str)을 해제한다 — pre-call len 미만은 게임 버퍼의 복사본.
    { let t2: u64 = core::ptr::read_unaligned((a0 as usize + 0x5e8) as *const u64);
      PV8_0.with(|c| { let pv = &*c.get();
      for (i, &(off, esz, lid)) in hs_vecs_11_0(t2).iter().enumerate() {
          let b = a0 as usize + 0x5e8 + off;
          let (cap, ptr, len) = (core::ptr::read_unaligned(b as *const usize), core::ptr::read_unaligned((b + 8) as *const usize),
                                 core::ptr::read_unaligned((b + 16) as *const usize));
          if cap > 0 && cap < (1 << 20) && ptr > 0x1000 {
              let pre = if i < pv.2 { pv.1[i].1 } else { 0 };
              if BISECT_NO_STRFREE == 0 && lid != 0 && len > pre && len < 4096 { for e in pre..len { elem_free_str(lid, ptr + e * esz); } }
              if let Ok(l) = std::alloc::Layout::from_size_align(cap * esz, 8) { std::alloc::dealloc(ptr as *mut u8, l); }
          }
      } }); }
    SP8.with(|c| { let sp = &*c.get();
        core::ptr::copy_nonoverlapping(sp.as_ptr(), a0 as *mut u8, 6168); }); // 게임 호출 후 상태로 복구
    core::ptr::copy_nonoverlapping(p2.as_ptr(), a2 as *mut u8, 320);                    // 게임 호출 후 상태로 복구(게임 진행은 게임 값으로)
    match m {
        Ok(_) => { if let Some(d) = sd {
            note(8, format!("#11 v3_fall_back_to_passive 대조#{} **상태갈림**: {} | (반환 void — 출력은 &mut self 다) | a0={:#x} a1={} a2={:#x} a3={:#x} a4={:#x} a5={:#x}", n, d, a0 as usize, a1, a2 as usize, a3 as usize, a4 as usize, a5 as usize)); } }
        Err(_) => { S[8].pan.fetch_add(1, Ordering::Relaxed); }
    }
    pop(8);
    g
}
unsafe fn w_3(a0: i64, a1: *const u8, a2: *const u8, a3: *const u8, a4: *const u8, a5: *const u8) -> P8 {
    S[9].calls.fetch_add(1, Ordering::Relaxed);
    let f: unsafe fn(i64, *const u8, *const u8, *const u8, *const u8, *const u8) -> P8 = core::mem::transmute(S[9].orig.load(Ordering::Relaxed));
    let t = top(9);
    // ★StdRng(320B) — 게임 호출이 난수열을 소비하므로 **먼저 떠 둔다**(안 그러면 거짓 DIFF).
    let mut r1 = [0u8; 320]; if t { core::ptr::copy_nonoverlapping(a1, r1.as_mut_ptr(), 320); }
    let g = f(a0, a1, a2, a3, a4, a5);
    if !t { pop(9); return g; }
    let n = S[9].cmp.fetch_add(1, Ordering::Relaxed) + 1;
    let mut p1 = [0u8; 320]; core::ptr::copy_nonoverlapping(a1, p1.as_mut_ptr(), 320); // 게임 호출 후 상태
    core::ptr::copy_nonoverlapping(r1.as_ptr(), a1 as *mut u8, 320);                    // 내 사본 호출 전 = 호출 전 상태로
    let m = catch_unwind(AssertUnwindSafe(|| my_3(a0, a1, a2, a3, a4, a5)));
    core::ptr::copy_nonoverlapping(p1.as_ptr(), a1 as *mut u8, 320);                    // 게임 호출 후 상태로 복구(게임 진행은 게임 값으로)
    match m {
        Ok(m) => { if m != g { note(9, format!("#03 defensive_crisis 대조#{} 갈림: g={:?} m={:?} | a0={} a1={:#x} a2={:#x} a3={:#x} a4={:#x} a5={:#x}", n, g, m, a0, a1 as usize, a2 as usize, a3 as usize, a4 as usize, a5 as usize)); } }
        Err(_) => { S[9].pan.fetch_add(1, Ordering::Relaxed); }
    }
    pop(9);
    g
}
unsafe fn w_49(a0: *const u8, a1: i64, a2: *const u8, a3: *const u8, a4: *const u8, a5: *const u8, a6: *const u8, a7: *const u8, a8: *const u8) {
    S[10].calls.fetch_add(1, Ordering::Relaxed);
    let f: unsafe fn(*const u8, i64, *const u8, *const u8, *const u8, *const u8, *const u8, *const u8, *const u8) = core::mem::transmute(S[10].orig.load(Ordering::Relaxed));
    let t = top(10);
    // ★StdRng(320B) — 게임 호출이 난수열을 소비하므로 **먼저 떠 둔다**(안 그러면 거짓 DIFF).
    let mut r2 = [0u8; 320]; if t { core::ptr::copy_nonoverlapping(a2, r2.as_mut_ptr(), 320); }
    let g = f(a0, a1, a2, a3, a4, a5, a6, a7, a8);
    if !t { pop(10); return g; }
    let n = S[10].cmp.fetch_add(1, Ordering::Relaxed) + 1;
    let mut p2 = [0u8; 320]; core::ptr::copy_nonoverlapping(a2, p2.as_mut_ptr(), 320); // 게임 호출 후 상태
    core::ptr::copy_nonoverlapping(r2.as_ptr(), a2 as *mut u8, 320);                    // 내 사본 호출 전 = 호출 전 상태로
    let mut gb = [0u64; 8]; core::ptr::copy_nonoverlapping(a0, gb.as_mut_ptr() as *mut u8, 64); // 게임 출력 사본
    let mut mb = [0u64; 8];                                                   // 내 사본 전용 출력 버퍼
    let m = catch_unwind(AssertUnwindSafe(|| my_49(mb.as_mut_ptr() as *const u8, a1, a2, a3, a4, a5, a6, a7, a8)));
    core::ptr::copy_nonoverlapping(p2.as_ptr(), a2 as *mut u8, 320);                    // 게임 호출 후 상태로 복구(게임 진행은 게임 값으로)
    match m {
        Ok(_) => { if let Some(off) = live_eq(a0, mb.as_ptr() as *const u8, LIVE_49) {
            note(10, format!("#49 resolve_join_stake 대조#{} 갈림(sret 64B · +{:#x}): g={:02x?} m={:02x?} | a1={} a2={:#x} a3={:#x} a4={:#x} a5={:#x} a6={:#x} a7={:#x} a8={:#x}", n, off, &gb[..8], &mb[..8], a1, a2 as usize, a3 as usize, a4 as usize, a5 as usize, a6 as usize, a7 as usize, a8 as usize)); } }
        Err(_) => { S[10].pan.fetch_add(1, Ordering::Relaxed); }
    }
    pop(10);
    g
}
unsafe fn w_0(a0: *const u8, a1: i64, a2: *const u8, a3: *const u8, a4: *const u8, a5: *const u8, a6: *const u8) {
    S[11].calls.fetch_add(1, Ordering::Relaxed);
    let f: unsafe fn(*const u8, i64, *const u8, *const u8, *const u8, *const u8, *const u8) = core::mem::transmute(S[11].orig.load(Ordering::Relaxed));
    let t = top(11);
    // ★StdRng(320B) — 게임 호출이 난수열을 소비하므로 **먼저 떠 둔다**(안 그러면 거짓 DIFF).
    let mut r2 = [0u8; 320]; if t { core::ptr::copy_nonoverlapping(a2, r2.as_mut_ptr(), 320); }
    let g = f(a0, a1, a2, a3, a4, a5, a6);
    if !t { pop(11); return g; }
    let n = S[11].cmp.fetch_add(1, Ordering::Relaxed) + 1;
    let mut p2 = [0u8; 320]; core::ptr::copy_nonoverlapping(a2, p2.as_mut_ptr(), 320); // 게임 호출 후 상태
    core::ptr::copy_nonoverlapping(r2.as_ptr(), a2 as *mut u8, 320);                    // 내 사본 호출 전 = 호출 전 상태로
    let mut gb = [0u64; 4]; core::ptr::copy_nonoverlapping(a0, gb.as_mut_ptr() as *mut u8, 32); // 게임 출력 사본
    let mut mb = [0u64; 4];                                                   // 내 사본 전용 출력 버퍼
    let m = catch_unwind(AssertUnwindSafe(|| my_0(mb.as_mut_ptr() as *const u8, a1, a2, a3, a4, a5, a6)));
    core::ptr::copy_nonoverlapping(p2.as_ptr(), a2 as *mut u8, 320);                    // 게임 호출 후 상태로 복구(게임 진행은 게임 값으로)
    match m {
        Ok(_) => { if let Some(off) = live_eq(a0, mb.as_ptr() as *const u8, LIVE_0) {
            note(11, format!("#00 ult 대조#{} 갈림(sret 32B · +{:#x}): g={:02x?} m={:02x?} | a1={} a2={:#x} a3={:#x} a4={:#x} a5={:#x} a6={:#x}", n, off, &gb[..4], &mb[..4], a1, a2 as usize, a3 as usize, a4 as usize, a5 as usize, a6 as usize)); } }
        Err(_) => { S[11].pan.fetch_add(1, Ordering::Relaxed); }
    }
    pop(11);
    g
}
unsafe fn w_6(a0: i64, a1: *const u8, a2: *const u8, a3: *const u8, a4: *const u8) -> P64 {
    S[12].calls.fetch_add(1, Ordering::Relaxed);
    let f: unsafe fn(i64, *const u8, *const u8, *const u8, *const u8) -> P64 = core::mem::transmute(S[12].orig.load(Ordering::Relaxed));
    let t = top(12);
    // ★StdRng(320B) — 게임 호출이 난수열을 소비하므로 **먼저 떠 둔다**(안 그러면 거짓 DIFF).
    let mut r1 = [0u8; 320]; if t { core::ptr::copy_nonoverlapping(a1, r1.as_mut_ptr(), 320); }
    let g = f(a0, a1, a2, a3, a4);
    if !t { pop(12); return g; }
    let n = S[12].cmp.fetch_add(1, Ordering::Relaxed) + 1;
    let mut p1 = [0u8; 320]; core::ptr::copy_nonoverlapping(a1, p1.as_mut_ptr(), 320); // 게임 호출 후 상태
    core::ptr::copy_nonoverlapping(r1.as_ptr(), a1 as *mut u8, 320);                    // 내 사본 호출 전 = 호출 전 상태로
    let m = catch_unwind(AssertUnwindSafe(|| my_6(a0, a1, a2, a3, a4)));
    core::ptr::copy_nonoverlapping(p1.as_ptr(), a1 as *mut u8, 320);                    // 게임 호출 후 상태로 복구(게임 진행은 게임 값으로)
    match m {
        Ok(m) => { let bad = m.a != g.a || (matches!(g.a, 0 | 1 | 2 | 3 | 5 | 6) && m.b != g.b);
            if bad { note(12, format!("#06 v2_response_retreat_stance 대조#{} 갈림: g={:?} m={:?} | a0={} a1={:#x} a2={:#x} a3={:#x} a4={:#x}", n, g, m, a0, a1 as usize, a2 as usize, a3 as usize, a4 as usize)); } }
        Err(_) => { S[12].pan.fetch_add(1, Ordering::Relaxed); }
    }
    pop(12);
    g
}
unsafe fn w_13(a0: u8, a1: i64, a2: i32, a3: *const u8, a4: *const u8) -> i64 {
    S[13].calls.fetch_add(1, Ordering::Relaxed);
    let f: unsafe fn(u8, i64, i32, *const u8, *const u8) -> i64 = core::mem::transmute(S[13].orig.load(Ordering::Relaxed));
    let t = top(13);
    let g = f(a0, a1, a2, a3, a4);
    if !t { pop(13); return g; }
    let n = S[13].cmp.fetch_add(1, Ordering::Relaxed) + 1;
    let m = catch_unwind(AssertUnwindSafe(|| my_13(a0, a1, a2, a3, a4)));
    match m {
        Ok(m) => { if m != g { note(13, format!("#13 target_bush_v30 대조#{} 갈림: g={:?} m={:?} | a0={} a1={} a2={} a3={:#x} a4={:#x}", n, g, m, a0, a1, a2, a3 as usize, a4 as usize)); } }
        Err(_) => { S[13].pan.fetch_add(1, Ordering::Relaxed); }
    }
    pop(13);
    g
}
unsafe fn w_14(a0: *const u8, a1: i64, a2: *const u8, a3: *const u8, a4: *const u8, a5: *const u8, a6: *const u8, a7: *const u8) {
    S[14].calls.fetch_add(1, Ordering::Relaxed);
    let f: unsafe fn(*const u8, i64, *const u8, *const u8, *const u8, *const u8, *const u8, *const u8) = core::mem::transmute(S[14].orig.load(Ordering::Relaxed));
    let t = top(14);
    // ★★&mut 게임 상태(48B) — **게임 호출 전** 상태를 떠 두지 않으면 내 사본은
    //   게임이 바꿔놓은 입력을 보게 돼 **거짓 DIFF** 가 난다(이 함수는 self 를 읽고도 쓴다).
    // ★★**스택이 아니라 thread_local 힙**에 둔다 — 48B × 2 를 스택에 잡으면
    //   rayon 워커의 **소스택 + 게임 sim 재귀**에서 STATUS_STACK_OVERFLOW 로 죽는다
    //   (2026-09-12 실사고: 스택 배열로 두던 판이 배경 sim 시작 직후 패닉로그 없이 즉사).
    SV14.with(|c| { let sv = &mut *c.get();
        if t { core::ptr::copy_nonoverlapping(a0, sv.as_mut_ptr(), 48); } });
    VB14.with(|c| { let vb = &mut *c.get(); if t {
        let ln = core::ptr::read_unaligned((a0 as usize + 0x10) as *const usize);
        let pz = core::ptr::read_unaligned((a0 as usize + 0x8) as *const usize);
        vb.1 = ln; vb.2 = ln + 2 <= 512;
        if ln > 0 && ln + 2 <= 512 && pz > 0x1000 {
            core::ptr::copy_nonoverlapping(pz as *const u8, vb.0.as_mut_ptr(), ln * 24);
        } else { vb.1 = ln; vb.2 = ln + 2 <= 512; }
    } });
    // ★StdRng(320B) — 게임 호출이 난수열을 소비하므로 **먼저 떠 둔다**(안 그러면 거짓 DIFF).
    let mut r2 = [0u8; 320]; if t { core::ptr::copy_nonoverlapping(a2, r2.as_mut_ptr(), 320); }
    let g = f(a0, a1, a2, a3, a4, a5, a6, a7);
    if !t { pop(14); return g; }
    // ★내 Vec 사본 버퍼에 안 들어간 호출은 **표본에서 뺀다** — 예전엔 `len=0` 으로
    //   떨어뜨려 **입력이 어긋난 채 비교**했고 그게 거짓 DIFF 였다(2026-09-12 차단).
    if !VB14.with(|c| (&*c.get()).2) { S[14].skip.fetch_add(1, Ordering::Relaxed); pop(14); return g; }
    let n = S[14].cmp.fetch_add(1, Ordering::Relaxed) + 1;
    let mut p2 = [0u8; 320]; core::ptr::copy_nonoverlapping(a2, p2.as_mut_ptr(), 320); // 게임 호출 후 상태
    core::ptr::copy_nonoverlapping(r2.as_ptr(), a2 as *mut u8, 320);                    // 내 사본 호출 전 = 호출 전 상태로
    SP14.with(|c| { let sp = &mut *c.get();
        core::ptr::copy_nonoverlapping(a0, sp.as_mut_ptr(), 48); });   // 게임 호출 후 상태
    SV14.with(|c| { let sv = &*c.get();
        core::ptr::copy_nonoverlapping(sv.as_ptr(), a0 as *mut u8, 48); }); // 내 사본 호출 전 = 호출 전 상태로
    // ★Vec 을 **빈 것**으로 — cap=0 이면 push 가 realloc 이 아니라 alloc 을 하므로
    //   **게임의 버퍼를 절대 해제하지 않는다**(순서는 IR 실측: cap@0x0 · ptr@0x8 · len@0x10).
    VB14.with(|c| { let vb = &*c.get();
        core::ptr::write_unaligned((a0 as usize + 0x0) as *mut usize, 512);  // cap = 내 버퍼 용량
        core::ptr::write_unaligned((a0 as usize + 0x8) as *mut usize, vb.0.as_ptr() as usize); // ptr = 내 버퍼
        core::ptr::write_unaligned((a0 as usize + 0x10) as *mut usize, vb.1);  // len = 게임과 **같은** 개수
    });
    let m = catch_unwind(AssertUnwindSafe(|| my_14(a0, a1, a2, a3, a4, a5, a6, a7)));
    // ★★★상태 diff — 이 함수는 **반환이 void** 다. ABI 상 반환이 없는 것이지
    //   **출력이 없는 게 아니다** — 출력은 `&mut self` 에 있다 ⟹ 그걸 비교한다.
    //   ⚠되돌리기·해제 **전에** 떠서 비교한다 — 내 사본이 재할당했으면 해제 후엔 못 읽는다.
    let (m_ptr, m_len) = (core::ptr::read_unaligned((a0 as usize + 0x8) as *const usize),
                          core::ptr::read_unaligned((a0 as usize + 0x10) as *const usize));
    SQ14.with(|c| { let sq = &mut *c.get();
        core::ptr::copy_nonoverlapping(a0, sq.as_mut_ptr(), 48); });
    let sd: Option<String> = SP14.with(|c| { let sp = &*c.get(); SQ14.with(|c2| { let sq = &*c2.get();
        // ① 본체 바이트(설계상 다른 구간은 제외)
        for off in 0..48usize {
            if off >= 0x0 && off < 0x18 { continue; }
            if sp[off] != sq[off] {
                return Some(format!("self+{:#x}: g={:02x} m={:02x}", off, sp[off], sq[off]));
            }
        }
        // ② Vec 의 len (cap/ptr 은 버퍼가 달라 비교 대상이 아니다)
        let g_ptr = core::ptr::read_unaligned(sp.as_ptr().add(0x8) as *const usize);
        let g_len = core::ptr::read_unaligned(sp.as_ptr().add(0x10) as *const usize);
        if g_len != m_len {
            return Some(format!("vec.len: g={} m={}", g_len, m_len));
        }
        // ③ Vec 의 **내용** (요소 24B)
        //   ★요소는 열거형이라 **variant 별로 쓰는 칸이 다르다** — 안 쓰는 칸은
        //   **재사용된 버퍼의 잔재**라 양쪽이 다른 게 정상이다(IR 실측 기반 live 범위만 본다).
        //   (off, len, 이 tag 들에서만 live · 빈 것 = 항상)
        const EL: &[(usize, usize, &[u8])] = &[(0, 1, &[]), (1, 1, &[0x11])];
        if g_len > 0 && g_len < 4096 && g_ptr > 0x1000 && m_ptr > 0x1000 {
            for e in 0..g_len {
                let (gb, mb) = (g_ptr + e * 24, m_ptr + e * 24);
                let tag = *(gb as *const u8);
                for &(o, l, tg) in EL {
                    if !tg.is_empty() && !tg.contains(&tag) { continue; }
                    for j in o..o + l {
                        let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8));
                        if gv != mv {
                            return Some(format!("vec[{}](tag {:#x})+{}: g={:02x} m={:02x}", e, tag, j, gv, mv));
                        }
                    }
                }
            }
        }
        None
    }) });
    // ★내 사본이 할당한 것을 해제한다(안 하면 호출당 누수). rlib 은 내 DLL 안에 링크돼 있어 알로케이터가 같다.
    { let c = core::ptr::read_unaligned((a0 as usize + 0x0) as *const usize);
      let pz = core::ptr::read_unaligned((a0 as usize + 0x8) as *const usize);
      let mine = VB14.with(|c2| (&*c2.get()).0.as_ptr() as usize);
      // ★내 버퍼면 해제하면 안 된다(thread_local 정적) — 재할당된 경우에만 해제.
      if c > 0 && pz > 0x1000 && pz != mine { if let Ok(l) = std::alloc::Layout::from_size_align(c * 24, 8) {
          std::alloc::dealloc(pz as *mut u8, l); } } }
    SP14.with(|c| { let sp = &*c.get();
        core::ptr::copy_nonoverlapping(sp.as_ptr(), a0 as *mut u8, 48); }); // 게임 호출 후 상태로 복구
    core::ptr::copy_nonoverlapping(p2.as_ptr(), a2 as *mut u8, 320);                    // 게임 호출 후 상태로 복구(게임 진행은 게임 값으로)
    match m {
        Ok(_) => { if let Some(d) = sd {
            note(14, format!("#14 update 대조#{} **상태갈림**: {} | (반환 void — 출력은 &mut self 다) | a0={:#x} a1={} a2={:#x} a3={:#x} a4={:#x} a5={:#x} a6={:#x} a7={:#x}", n, d, a0 as usize, a1, a2 as usize, a3 as usize, a4 as usize, a5 as usize, a6 as usize, a7 as usize)); } }
        Err(_) => { S[14].pan.fetch_add(1, Ordering::Relaxed); }
    }
    pop(14);
    g
}
unsafe fn w_9(a0: i64, a1: *const u8, a2: *const u8, a3: *const u8, a4: i64) -> bool {
    S[15].calls.fetch_add(1, Ordering::Relaxed);
    let f: unsafe fn(i64, *const u8, *const u8, *const u8, i64) -> bool = core::mem::transmute(S[15].orig.load(Ordering::Relaxed));
    let t = top(15);
    let g = f(a0, a1, a2, a3, a4);
    if !t { pop(15); return g; }
    let n = S[15].cmp.fetch_add(1, Ordering::Relaxed) + 1;
    let m = catch_unwind(AssertUnwindSafe(|| my_9(a0, a1, a2, a3, a4)));
    match m {
        Ok(m) => { if m != g { note(15, format!("#09 check_favorable_engage_formation 대조#{} 갈림: g={:?} m={:?} | a0={} a1={:#x} a2={:#x} a3={:#x} a4={}", n, g, m, a0, a1 as usize, a2 as usize, a3 as usize, a4)); } }
        Err(_) => { S[15].pan.fetch_add(1, Ordering::Relaxed); }
    }
    pop(15);
    g
}
unsafe fn w_33(a0: i64, a1: *const u8, a2: *const u8, a3: *const u8, a4: *const u8) -> bool {
    S[16].calls.fetch_add(1, Ordering::Relaxed);
    let f: unsafe fn(i64, *const u8, *const u8, *const u8, *const u8) -> bool = core::mem::transmute(S[16].orig.load(Ordering::Relaxed));
    let t = top(16);
    // ★StdRng(320B) — 게임 호출이 난수열을 소비하므로 **먼저 떠 둔다**(안 그러면 거짓 DIFF).
    let mut r1 = [0u8; 320]; if t { core::ptr::copy_nonoverlapping(a1, r1.as_mut_ptr(), 320); }
    let g = f(a0, a1, a2, a3, a4);
    if !t { pop(16); return g; }
    let n = S[16].cmp.fetch_add(1, Ordering::Relaxed) + 1;
    let mut p1 = [0u8; 320]; core::ptr::copy_nonoverlapping(a1, p1.as_mut_ptr(), 320); // 게임 호출 후 상태
    core::ptr::copy_nonoverlapping(r1.as_ptr(), a1 as *mut u8, 320);                    // 내 사본 호출 전 = 호출 전 상태로
    let m = catch_unwind(AssertUnwindSafe(|| my_33(a0, a1, a2, a3, a4)));
    core::ptr::copy_nonoverlapping(p1.as_ptr(), a1 as *mut u8, 320);                    // 게임 호출 후 상태로 복구(게임 진행은 게임 값으로)
    match m {
        Ok(m) => { if m != g { note(16, format!("#33 v2_obj_restore_safe 대조#{} 갈림: g={:?} m={:?} | a0={} a1={:#x} a2={:#x} a3={:#x} a4={:#x}", n, g, m, a0, a1 as usize, a2 as usize, a3 as usize, a4 as usize)); } }
        Err(_) => { S[16].pan.fetch_add(1, Ordering::Relaxed); }
    }
    pop(16);
    g
}
unsafe fn w_37(a0: *const u8, a1: *const u8, a2: i64, a3: *const u8, a4: i64) -> bool {
    S[17].calls.fetch_add(1, Ordering::Relaxed);
    let f: unsafe fn(*const u8, *const u8, i64, *const u8, i64) -> bool = core::mem::transmute(S[17].orig.load(Ordering::Relaxed));
    let t = top(17);
    let g = f(a0, a1, a2, a3, a4);
    if !t { pop(17); return g; }
    let n = S[17].cmp.fetch_add(1, Ordering::Relaxed) + 1;
    let m = catch_unwind(AssertUnwindSafe(|| my_37(a0, a1, a2, a3, a4)));
    match m {
        Ok(m) => { if m != g { note(17, format!("#37 i_am_chosen_defender 대조#{} 갈림: g={:?} m={:?} | a0={:#x} a1={:#x} a2={} a3={:#x} a4={}", n, g, m, a0 as usize, a1 as usize, a2, a3 as usize, a4)); } }
        Err(_) => { S[17].pan.fetch_add(1, Ordering::Relaxed); }
    }
    pop(17);
    g
}
unsafe fn w_36(a0: i64, a1: *const u8, a2: *const u8, a3: *const u8, a4: *const u8) -> i64 {
    S[18].calls.fetch_add(1, Ordering::Relaxed);
    let f: unsafe fn(i64, *const u8, *const u8, *const u8, *const u8) -> i64 = core::mem::transmute(S[18].orig.load(Ordering::Relaxed));
    let t = top(18);
    let g = f(a0, a1, a2, a3, a4);
    if !t { pop(18); return g; }
    let n = S[18].cmp.fetch_add(1, Ordering::Relaxed) + 1;
    let m = catch_unwind(AssertUnwindSafe(|| my_36(a0, a1, a2, a3, a4)));
    match m {
        Ok(m) => { if m != g { note(18, format!("#36 calculate_nexus_defense_count 대조#{} 갈림: g={:?} m={:?} | a0={} a1={:#x} a2={:#x} a3={:#x} a4={:#x}", n, g, m, a0, a1 as usize, a2 as usize, a3 as usize, a4 as usize)); } }
        Err(_) => { S[18].pan.fetch_add(1, Ordering::Relaxed); }
    }
    pop(18);
    g
}
unsafe fn w_35(a0: *const u8, a1: i64, a2: *const u8, a3: *const u8, a4: *const u8, a5: *const u8, a6: *const u8, a7: i64, a8: *const u8, a9: i64, a10: u8, a11: *const u8, a12: i64, a13: *const u8) {
    S[19].calls.fetch_add(1, Ordering::Relaxed);
    let f: unsafe fn(*const u8, i64, *const u8, *const u8, *const u8, *const u8, *const u8, i64, *const u8, i64, u8, *const u8, i64, *const u8) = core::mem::transmute(S[19].orig.load(Ordering::Relaxed));
    let t = top(19);
    // ★StdRng(320B) — 게임 호출이 난수열을 소비하므로 **먼저 떠 둔다**(안 그러면 거짓 DIFF).
    let mut r2 = [0u8; 320]; if t { core::ptr::copy_nonoverlapping(a2, r2.as_mut_ptr(), 320); }
    let g = f(a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12, a13);
    if !t { pop(19); return g; }
    let n = S[19].cmp.fetch_add(1, Ordering::Relaxed) + 1;
    let mut p2 = [0u8; 320]; core::ptr::copy_nonoverlapping(a2, p2.as_mut_ptr(), 320); // 게임 호출 후 상태
    core::ptr::copy_nonoverlapping(r2.as_ptr(), a2 as *mut u8, 320);                    // 내 사본 호출 전 = 호출 전 상태로
    let mut gb = [0u64; 8]; core::ptr::copy_nonoverlapping(a0, gb.as_mut_ptr() as *mut u8, 64); // 게임 출력 사본
    let mut mb = [0u64; 8];                                                   // 내 사본 전용 출력 버퍼
    let m = catch_unwind(AssertUnwindSafe(|| my_35(mb.as_mut_ptr() as *const u8, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12, a13)));
    core::ptr::copy_nonoverlapping(p2.as_ptr(), a2 as *mut u8, 320);                    // 게임 호출 후 상태로 복구(게임 진행은 게임 값으로)
    match m {
        Ok(_) => { if let Some(off) = live_eq(a0, mb.as_ptr() as *const u8, LIVE_35) {
            note(19, format!("#35 resolve_fight_stake 대조#{} 갈림(sret 64B · +{:#x}): g={:02x?} m={:02x?} | a1={} a2={:#x} a3={:#x} a4={:#x} a5={:#x} a6={:#x} a7={} a8={:#x} a9={} a10={} a11={:#x} a12={} a13={:#x}", n, off, &gb[..8], &mb[..8], a1, a2 as usize, a3 as usize, a4 as usize, a5 as usize, a6 as usize, a7, a8 as usize, a9, a10, a11 as usize, a12, a13 as usize)); } }
        Err(_) => { S[19].pan.fetch_add(1, Ordering::Relaxed); }
    }
    pop(19);
    g
}
unsafe fn w_1(a0: *const u8, a1: *const u8, a2: *const u8, a3: *const u8, a4: *const u8, a5: *const u8, a6: *const u8) -> i64 {
    S[20].calls.fetch_add(1, Ordering::Relaxed);
    let f: unsafe fn(*const u8, *const u8, *const u8, *const u8, *const u8, *const u8, *const u8) -> i64 = core::mem::transmute(S[20].orig.load(Ordering::Relaxed));
    let t = top(20);
    // ★StdRng(320B) — 게임 호출이 난수열을 소비하므로 **먼저 떠 둔다**(안 그러면 거짓 DIFF).
    let mut r0 = [0u8; 320]; if t { core::ptr::copy_nonoverlapping(a0, r0.as_mut_ptr(), 320); }
    let g = f(a0, a1, a2, a3, a4, a5, a6);
    if !t { pop(20); return g; }
    let n = S[20].cmp.fetch_add(1, Ordering::Relaxed) + 1;
    let mut p0 = [0u8; 320]; core::ptr::copy_nonoverlapping(a0, p0.as_mut_ptr(), 320); // 게임 호출 후 상태
    core::ptr::copy_nonoverlapping(r0.as_ptr(), a0 as *mut u8, 320);                    // 내 사본 호출 전 = 호출 전 상태로
    let m = catch_unwind(AssertUnwindSafe(|| my_1(a0, a1, a2, a3, a4, a5, a6)));
    core::ptr::copy_nonoverlapping(p0.as_ptr(), a0 as *mut u8, 320);                    // 게임 호출 후 상태로 복구(게임 진행은 게임 값으로)
    match m {
        Ok(m) => { if m != g { note(20, format!("#01 calculate_jungle_action_score 대조#{} 갈림: g={:?} m={:?} | a0={:#x} a1={:#x} a2={:#x} a3={:#x} a4={:#x} a5={:#x} a6={:#x}", n, g, m, a0 as usize, a1 as usize, a2 as usize, a3 as usize, a4 as usize, a5 as usize, a6 as usize)); } }
        Err(_) => { S[20].pan.fetch_add(1, Ordering::Relaxed); }
    }
    pop(20);
    g
}
unsafe fn w_43(a0: *const u8, a1: i64, a2: *const u8) -> bool {
    S[21].calls.fetch_add(1, Ordering::Relaxed);
    let f: unsafe fn(*const u8, i64, *const u8) -> bool = core::mem::transmute(S[21].orig.load(Ordering::Relaxed));
    let t = top(21);
    let g = f(a0, a1, a2);
    if !t { pop(21); return g; }
    let n = S[21].cmp.fetch_add(1, Ordering::Relaxed) + 1;
    let m = catch_unwind(AssertUnwindSafe(|| my_43(a0, a1, a2)));
    match m {
        Ok(m) => { if m != g { note(21, format!("#43 BattlePlan::with_runaway 대조#{} 갈림: g={:?} m={:?} | a0={:#x} a1={} a2={:#x}", n, g, m, a0 as usize, a1, a2 as usize)); } }
        Err(_) => { S[21].pan.fetch_add(1, Ordering::Relaxed); }
    }
    pop(21);
    g
}
unsafe fn w_28(a0: i64, a1: *const u8, a2: *const u8, a3: *const u8, a4: *const u8, a5: i64, a6: i64, a7: i64, a8: i64, a9: i64) -> bool {
    S[22].calls.fetch_add(1, Ordering::Relaxed);
    let f: unsafe fn(i64, *const u8, *const u8, *const u8, *const u8, i64, i64, i64, i64, i64) -> bool = core::mem::transmute(S[22].orig.load(Ordering::Relaxed));
    let t = top(22);
    let g = f(a0, a1, a2, a3, a4, a5, a6, a7, a8, a9);
    if !t { pop(22); return g; }
    let n = S[22].cmp.fetch_add(1, Ordering::Relaxed) + 1;
    let m = catch_unwind(AssertUnwindSafe(|| my_28(a0, a1, a2, a3, a4, a5, a6, a7, a8, a9)));
    match m {
        Ok(m) => { if m != g { note(22, format!("#28 v21_should_defer_support_target 대조#{} 갈림: g={:?} m={:?} | a0={} a1={:#x} a2={:#x} a3={:#x} a4={:#x} a5={} a6={} a7={} a8={} a9={}", n, g, m, a0, a1 as usize, a2 as usize, a3 as usize, a4 as usize, a5, a6, a7, a8, a9)); } }
        Err(_) => { S[22].pan.fetch_add(1, Ordering::Relaxed); }
    }
    pop(22);
    g
}
unsafe fn w_27(a0: *const u8, a1: *const u8) -> bool {
    S[23].calls.fetch_add(1, Ordering::Relaxed);
    let f: unsafe fn(*const u8, *const u8) -> bool = core::mem::transmute(S[23].orig.load(Ordering::Relaxed));
    let t = top(23);
    let g = f(a0, a1);
    if !t { pop(23); return g; }
    let n = S[23].cmp.fetch_add(1, Ordering::Relaxed) + 1;
    let m = catch_unwind(AssertUnwindSafe(|| my_27(a0, a1)));
    match m {
        Ok(m) => { if m != g { note(23, format!("#27 nexus_under_direct_attack 대조#{} 갈림: g={:?} m={:?} | a0={:#x} a1={:#x}", n, g, m, a0 as usize, a1 as usize)); } }
        Err(_) => { S[23].pan.fetch_add(1, Ordering::Relaxed); }
    }
    pop(23);
    g
}
unsafe fn w_26(a0: *const u8, a1: *const u8, a2: *const u8, a3: i64) -> u8 {
    S[24].calls.fetch_add(1, Ordering::Relaxed);
    let f: unsafe fn(*const u8, *const u8, *const u8, i64) -> u8 = core::mem::transmute(S[24].orig.load(Ordering::Relaxed));
    let t = top(24);
    let g = f(a0, a1, a2, a3);
    if !t { pop(24); return g; }
    let n = S[24].cmp.fetch_add(1, Ordering::Relaxed) + 1;
    let m = catch_unwind(AssertUnwindSafe(|| my_26(a0, a1, a2, a3)));
    match m {
        Ok(m) => { if m != g { note(24, format!("#26 v23_objective_setup_pressure_line 대조#{} 갈림: g={:?} m={:?} | a0={:#x} a1={:#x} a2={:#x} a3={}", n, g, m, a0 as usize, a1 as usize, a2 as usize, a3)); } }
        Err(_) => { S[24].pan.fetch_add(1, Ordering::Relaxed); }
    }
    pop(24);
    g
}
unsafe fn w_50(a0: i64, a1: *const u8, a2: *const u8, a3: *const u8, a4: *const u8, a5: *const u8) -> bool {
    S[25].calls.fetch_add(1, Ordering::Relaxed);
    let f: unsafe fn(i64, *const u8, *const u8, *const u8, *const u8, *const u8) -> bool = core::mem::transmute(S[25].orig.load(Ordering::Relaxed));
    let t = top(25);
    let g = f(a0, a1, a2, a3, a4, a5);
    if !t { pop(25); return g; }
    let n = S[25].cmp.fetch_add(1, Ordering::Relaxed) + 1;
    let m = catch_unwind(AssertUnwindSafe(|| my_50(a0, a1, a2, a3, a4, a5)));
    match m {
        Ok(m) => { if m != g { note(25, format!("#50 check_epic_giveup 대조#{} 갈림: g={:?} m={:?} | a0={} a1={:#x} a2={:#x} a3={:#x} a4={:#x} a5={:#x}", n, g, m, a0, a1 as usize, a2 as usize, a3 as usize, a4 as usize, a5 as usize)); } }
        Err(_) => { S[25].pan.fetch_add(1, Ordering::Relaxed); }
    }
    pop(25);
    g
}
unsafe fn w_24(a0: *const u8, a1: *const u8, a2: i64, a3: i64, a4: i64, a5: i64) -> i64 {
    S[26].calls.fetch_add(1, Ordering::Relaxed);
    let f: unsafe fn(*const u8, *const u8, i64, i64, i64, i64) -> i64 = core::mem::transmute(S[26].orig.load(Ordering::Relaxed));
    let t = top(26);
    let g = f(a0, a1, a2, a3, a4, a5);
    if !t { pop(26); return g; }
    let n = S[26].cmp.fetch_add(1, Ordering::Relaxed) + 1;
    let m = catch_unwind(AssertUnwindSafe(|| my_24(a0, a1, a2, a3, a4, a5)));
    match m {
        Ok(m) => { if m != g { note(26, format!("#24 v23_healthy_allies_near_point 대조#{} 갈림: g={:?} m={:?} | a0={:#x} a1={:#x} a2={} a3={} a4={} a5={}", n, g, m, a0 as usize, a1 as usize, a2, a3, a4, a5)); } }
        Err(_) => { S[26].pan.fetch_add(1, Ordering::Relaxed); }
    }
    pop(26);
    g
}
unsafe fn w_8(a0: *const u8, a1: i64, a2: *const u8, a3: *const u8, a4: *const u8, a5: *const u8, a6: *const u8) -> bool {
    S[27].calls.fetch_add(1, Ordering::Relaxed);
    let f: unsafe fn(*const u8, i64, *const u8, *const u8, *const u8, *const u8, *const u8) -> bool = core::mem::transmute(S[27].orig.load(Ordering::Relaxed));
    let t = top(27);
    // ★StdRng(320B) — 게임 호출이 난수열을 소비하므로 **먼저 떠 둔다**(안 그러면 거짓 DIFF).
    let mut r2 = [0u8; 320]; if t { core::ptr::copy_nonoverlapping(a2, r2.as_mut_ptr(), 320); }
    let g = f(a0, a1, a2, a3, a4, a5, a6);
    if !t { pop(27); return g; }
    let n = S[27].cmp.fetch_add(1, Ordering::Relaxed) + 1;
    let mut p2 = [0u8; 320]; core::ptr::copy_nonoverlapping(a2, p2.as_mut_ptr(), 320); // 게임 호출 후 상태
    core::ptr::copy_nonoverlapping(r2.as_ptr(), a2 as *mut u8, 320);                    // 내 사본 호출 전 = 호출 전 상태로
    let m = catch_unwind(AssertUnwindSafe(|| my_8(a0, a1, a2, a3, a4, a5, a6)));
    core::ptr::copy_nonoverlapping(p2.as_ptr(), a2 as *mut u8, 320);                    // 게임 호출 후 상태로 복구(게임 진행은 게임 값으로)
    match m {
        Ok(m) => { if m != g { note(27, format!("#08 is_end 대조#{} 갈림: g={:?} m={:?} | a0={:#x} a1={} a2={:#x} a3={:#x} a4={:#x} a5={:#x} a6={:#x}", n, g, m, a0 as usize, a1, a2 as usize, a3 as usize, a4 as usize, a5 as usize, a6 as usize)); } }
        Err(_) => { S[27].pan.fetch_add(1, Ordering::Relaxed); }
    }
    pop(27);
    g
}
unsafe fn w_18(a0: *const u8, a1: i64, a2: *const u8, a3: *const u8, a4: *const u8, a5: *const u8) -> bool {
    S[28].calls.fetch_add(1, Ordering::Relaxed);
    let f: unsafe fn(*const u8, i64, *const u8, *const u8, *const u8, *const u8) -> bool = core::mem::transmute(S[28].orig.load(Ordering::Relaxed));
    let t = top(28);
    // ★★&mut 게임 상태(1064B) — **게임 호출 전** 상태를 떠 두지 않으면 내 사본은
    //   게임이 바꿔놓은 입력을 보게 돼 **거짓 DIFF** 가 난다(이 함수는 self 를 읽고도 쓴다).
    // ★★**스택이 아니라 thread_local 힙**에 둔다 — 1064B × 2 를 스택에 잡으면
    //   rayon 워커의 **소스택 + 게임 sim 재귀**에서 STATUS_STACK_OVERFLOW 로 죽는다
    //   (2026-09-12 실사고: 스택 배열로 두던 판이 배경 sim 시작 직후 패닉로그 없이 즉사).
    SV28.with(|c| { let sv = &mut *c.get();
        if t { core::ptr::copy_nonoverlapping(a0, sv.as_mut_ptr(), 1064); } });
    VB28.with(|c| { let vb = &mut *c.get(); if t {
        let ln = core::ptr::read_unaligned((a0 as usize + 0xd0) as *const usize);
        let pz = core::ptr::read_unaligned((a0 as usize + 0xc8) as *const usize);
        vb.1 = ln; vb.2 = ln + 2 <= 512;
        if ln > 0 && ln + 2 <= 512 && pz > 0x1000 {
            core::ptr::copy_nonoverlapping(pz as *const u8, vb.0.as_mut_ptr(), ln * 24);
        } else { vb.1 = ln; vb.2 = ln + 2 <= 512; }
    } });
    let g = f(a0, a1, a2, a3, a4, a5);
    if !t { pop(28); return g; }
    // ★내 Vec 사본 버퍼에 안 들어간 호출은 **표본에서 뺀다** — 예전엔 `len=0` 으로
    //   떨어뜨려 **입력이 어긋난 채 비교**했고 그게 거짓 DIFF 였다(2026-09-12 차단).
    if !VB28.with(|c| (&*c.get()).2) { S[28].skip.fetch_add(1, Ordering::Relaxed); pop(28); return g; }
    let n = S[28].cmp.fetch_add(1, Ordering::Relaxed) + 1;
    SP28.with(|c| { let sp = &mut *c.get();
        core::ptr::copy_nonoverlapping(a0, sp.as_mut_ptr(), 1064); });   // 게임 호출 후 상태
    SV28.with(|c| { let sv = &*c.get();
        core::ptr::copy_nonoverlapping(sv.as_ptr(), a0 as *mut u8, 1064); }); // 내 사본 호출 전 = 호출 전 상태로
    // ★Vec 을 **빈 것**으로 — cap=0 이면 push 가 realloc 이 아니라 alloc 을 하므로
    //   **게임의 버퍼를 절대 해제하지 않는다**(순서는 IR 실측: cap@0xc0 · ptr@0xc8 · len@0xd0).
    VB28.with(|c| { let vb = &*c.get();
        core::ptr::write_unaligned((a0 as usize + 0xc0) as *mut usize, 512);  // cap = 내 버퍼 용량
        core::ptr::write_unaligned((a0 as usize + 0xc8) as *mut usize, vb.0.as_ptr() as usize); // ptr = 내 버퍼
        core::ptr::write_unaligned((a0 as usize + 0xd0) as *mut usize, vb.1);  // len = 게임과 **같은** 개수
    });
    let m = catch_unwind(AssertUnwindSafe(|| my_18(a0, a1, RNG28.with(|c| c.get() as *const u8), a2, a3, a4, a5)));
    // ★★★상태 diff — 이 함수의 **반환값은 exe 에 실재하지 않는다**(호출부가 결과를
    //   안 만져서 `AL` 이 미정규화 상태로 남는다) ⟹ 판정은 **self 부작용**으로 한다.
    //   ⚠되돌리기·해제 **전에** 떠서 비교한다 — 내 사본이 재할당했으면 해제 후엔 못 읽는다.
    let (m_ptr, m_len) = (core::ptr::read_unaligned((a0 as usize + 0xc8) as *const usize),
                          core::ptr::read_unaligned((a0 as usize + 0xd0) as *const usize));
    SQ28.with(|c| { let sq = &mut *c.get();
        core::ptr::copy_nonoverlapping(a0, sq.as_mut_ptr(), 1064); });
    let sd: Option<String> = SP28.with(|c| { let sp = &*c.get(); SQ28.with(|c2| { let sq = &*c2.get();
        // ① 본체 바이트(설계상 다른 구간은 제외)
        for off in 0..1064usize {
            if off >= 0xc0 && off < 0xd8 { continue; }
            if sp[off] != sq[off] {
                return Some(format!("self+{:#x}: g={:02x} m={:02x}", off, sp[off], sq[off]));
            }
        }
        // ② Vec 의 len (cap/ptr 은 버퍼가 달라 비교 대상이 아니다)
        let g_ptr = core::ptr::read_unaligned(sp.as_ptr().add(0xc8) as *const usize);
        let g_len = core::ptr::read_unaligned(sp.as_ptr().add(0xd0) as *const usize);
        if g_len != m_len {
            return Some(format!("vec.len: g={} m={}", g_len, m_len));
        }
        // ③ Vec 의 **내용** (요소 24B)
        //   ★요소는 열거형이라 **variant 별로 쓰는 칸이 다르다** — 안 쓰는 칸은
        //   **재사용된 버퍼의 잔재**라 양쪽이 다른 게 정상이다(IR 실측 기반 live 범위만 본다).
        //   (off, len, 이 tag 들에서만 live · 빈 것 = 항상)
        const EL: &[(usize, usize, &[u8])] = &[(0, 1, &[]), (1, 1, &[0x15, 0x16]), (8, 8, &[])];
        if g_len > 0 && g_len < 4096 && g_ptr > 0x1000 && m_ptr > 0x1000 {
            for e in 0..g_len {
                let (gb, mb) = (g_ptr + e * 24, m_ptr + e * 24);
                let tag = *(gb as *const u8);
                for &(o, l, tg) in EL {
                    if !tg.is_empty() && !tg.contains(&tag) { continue; }
                    for j in o..o + l {
                        let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8));
                        if gv != mv {
                            return Some(format!("vec[{}](tag {:#x})+{}: g={:02x} m={:02x}", e, tag, j, gv, mv));
                        }
                    }
                }
            }
        }
        None
    }) });
    // ★내 사본이 할당한 것을 해제한다(안 하면 호출당 누수). rlib 은 내 DLL 안에 링크돼 있어 알로케이터가 같다.
    { let c = core::ptr::read_unaligned((a0 as usize + 0xc0) as *const usize);
      let pz = core::ptr::read_unaligned((a0 as usize + 0xc8) as *const usize);
      let mine = VB28.with(|c2| (&*c2.get()).0.as_ptr() as usize);
      // ★내 버퍼면 해제하면 안 된다(thread_local 정적) — 재할당된 경우에만 해제.
      if c > 0 && pz > 0x1000 && pz != mine { if let Ok(l) = std::alloc::Layout::from_size_align(c * 24, 8) {
          std::alloc::dealloc(pz as *mut u8, l); } } }
    SP28.with(|c| { let sp = &*c.get();
        core::ptr::copy_nonoverlapping(sp.as_ptr(), a0 as *mut u8, 1064); }); // 게임 호출 후 상태로 복구
    match m {
        Ok(_) => { if let Some(d) = sd {
            note(28, format!("#18 v3_epicops_buff_window 대조#{} **상태갈림**: {} | (반환은 exe 에 미실재 — 참고 g={:?}) | a0={:#x} a1={} a2={:#x} a3={:#x} a4={:#x} a5={:#x}", n, d, g, a0 as usize, a1, a2 as usize, a3 as usize, a4 as usize, a5 as usize)); } }
        Err(_) => { S[28].pan.fetch_add(1, Ordering::Relaxed); }
    }
    pop(28);
    g
}
unsafe fn w_41(a0: *const u8, a1: i64, a2: *const u8, a3: *const u8, a4: *const u8, a5: *const u8, a6: *const u8, a7: i64, a8: *const u8, a9: i64, a10: *const u8, a11: *const u8) {
    S[29].calls.fetch_add(1, Ordering::Relaxed);
    let f: unsafe fn(*const u8, i64, *const u8, *const u8, *const u8, *const u8, *const u8, i64, *const u8, i64, *const u8, *const u8) = core::mem::transmute(S[29].orig.load(Ordering::Relaxed));
    let t = top(29);
    // ★StdRng(320B) — 게임 호출이 난수열을 소비하므로 **먼저 떠 둔다**(안 그러면 거짓 DIFF).
    let mut r2 = [0u8; 320]; if t { core::ptr::copy_nonoverlapping(a2, r2.as_mut_ptr(), 320); }
    let g = f(a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11);
    if !t { pop(29); return g; }
    let n = S[29].cmp.fetch_add(1, Ordering::Relaxed) + 1;
    let mut p2 = [0u8; 320]; core::ptr::copy_nonoverlapping(a2, p2.as_mut_ptr(), 320); // 게임 호출 후 상태
    core::ptr::copy_nonoverlapping(r2.as_ptr(), a2 as *mut u8, 320);                    // 내 사본 호출 전 = 호출 전 상태로
    let mut gb = [0u64; 4]; core::ptr::copy_nonoverlapping(a0, gb.as_mut_ptr() as *mut u8, 32); // 게임 출력 사본
    let mut mb = [0u64; 4];                                                   // 내 사본 전용 출력 버퍼
    let m = catch_unwind(AssertUnwindSafe(|| my_41(mb.as_mut_ptr() as *const u8, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11)));
    core::ptr::copy_nonoverlapping(p2.as_ptr(), a2 as *mut u8, 320);                    // 게임 호출 후 상태로 복구(게임 진행은 게임 값으로)
    match m {
        Ok(_) => { let (gl, ml) = (gb[3] as usize, mb[3] as usize); let (gp, mp) = (gb[0] as usize, mb[0] as usize);
            const EL: &[(usize, usize, &[u8])] = &[(0, 8, &[]), (8, 8, &[]), (16, 1, &[])];
            let d: Option<String> = if gl != ml { Some(format!("len g={} m={}", gl, ml)) }
                else if gl > 0 && gl < 4096 && gp > 0x1000 && mp > 0x1000 { (|| { for e in 0..gl { let (eb, fb) = (gp + e * 24, mp + e * 24); let tag = *(eb as *const u8);
                    for &(o, l, tg) in EL { if !tg.is_empty() && !tg.contains(&tag) { continue; } for j in o..o + l { let (gv, mv) = (*((eb + j) as *const u8), *((fb + j) as *const u8)); if gv != mv { return Some(format!("vec[{}]+{}: g={:02x} m={:02x}", e, j, gv, mv)); } } } } None })() } else { None };
            if let Some(d) = d { note(29, format!("#41 fight_participants 대조#{} 갈림(sret bump Vec len={}): {} | a1={} a2={:#x} a3={:#x} a4={:#x} a5={:#x} a6={:#x} a7={} a8={:#x} a9={} a10={:#x} a11={:#x}", n, gl, d, a1, a2 as usize, a3 as usize, a4 as usize, a5 as usize, a6 as usize, a7, a8 as usize, a9, a10 as usize, a11 as usize)); } }
        Err(_) => { S[29].pan.fetch_add(1, Ordering::Relaxed); }
    }
    pop(29);
    g
}
unsafe fn w_12(a0: *const u8, a1: i64, a2: *const u8, a3: *const u8, a4: *const u8, a5: i32, a6: *const u8, a7: u8, a8: *const u8) {
    S[30].calls.fetch_add(1, Ordering::Relaxed);
    let f: unsafe fn(*const u8, i64, *const u8, *const u8, *const u8, i32, *const u8, u8, *const u8) = core::mem::transmute(S[30].orig.load(Ordering::Relaxed));
    let t = top(30);
    // ★★&mut 게임 상태(6168B) — **게임 호출 전** 상태를 떠 두지 않으면 내 사본은
    //   게임이 바꿔놓은 입력을 보게 돼 **거짓 DIFF** 가 난다(이 함수는 self 를 읽고도 쓴다).
    // ★★**스택이 아니라 thread_local 힙**에 둔다 — 6168B × 2 를 스택에 잡으면
    //   rayon 워커의 **소스택 + 게임 sim 재귀**에서 STATUS_STACK_OVERFLOW 로 죽는다
    //   (2026-09-12 실사고: 스택 배열로 두던 판이 배경 sim 시작 직후 패닉로그 없이 즉사).
    SV30.with(|c| { let sv = &mut *c.get();
        if t { core::ptr::copy_nonoverlapping(a0, sv.as_mut_ptr(), 6168); } });
    VB30.with(|c| { (&mut *c.get()).2 = true; });   // 치환할 Vec 이 없다 = 항상 표본
    // ★명세 0: self+0x5e8 의 소유 Vec **내용을 게임 호출 전에** 떠 둔다 — 게임이 해제/재할당할 수 있다.
    PV30_0.with(|c| { let pv = &mut *c.get(); pv.2 = 0; pv.3 = true; if t {
        let tg = core::ptr::read_unaligned((a0 as usize + 0x5e8) as *const u64);
        let mut used = 0usize;
        for (i, &(off, esz, _)) in hs_vecs_12_0(tg).iter().enumerate() {
            if i >= 8 { pv.3 = false; break; }
            let b = a0 as usize + 0x5e8 + off;
            let (cap, ptr, len) = (core::ptr::read_unaligned(b as *const usize),
                                   core::ptr::read_unaligned((b + 8) as *const usize),
                                   core::ptr::read_unaligned((b + 16) as *const usize));
            pv.2 = i + 1;
            if ptr > 0x1000 && len <= cap && cap < (1 << 20) {
                if used + len * esz > 32768 { pv.3 = false; break; }
                if len > 0 { core::ptr::copy_nonoverlapping(ptr as *const u8, pv.0.as_mut_ptr().add(used), len * esz); }
                pv.1[i] = (cap, len, esz); used += len * esz;
            } else { pv.1[i] = (0, 0, esz); }   // 비정상 삼중항 = 치환 안 함
        }
    } });
    // ★명세 1: self+0x0 의 소유 Vec **내용을 게임 호출 전에** 떠 둔다 — 게임이 해제/재할당할 수 있다.
    PV30_1.with(|c| { let pv = &mut *c.get(); pv.2 = 0; pv.3 = true; if t {
        let tg = 0u64;
        let mut used = 0usize;
        for (i, &(off, esz, _)) in hs_vecs_12_1(tg).iter().enumerate() {
            if i >= 8 { pv.3 = false; break; }
            let b = a0 as usize + 0x0 + off;
            let (cap, ptr, len) = (core::ptr::read_unaligned(b as *const usize),
                                   core::ptr::read_unaligned((b + 8) as *const usize),
                                   core::ptr::read_unaligned((b + 16) as *const usize));
            pv.2 = i + 1;
            if ptr > 0x1000 && len <= cap && cap < (1 << 20) {
                if used + len * esz > 32768 { pv.3 = false; break; }
                if len > 0 { core::ptr::copy_nonoverlapping(ptr as *const u8, pv.0.as_mut_ptr().add(used), len * esz); }
                pv.1[i] = (cap, len, esz); used += len * esz;
            } else { pv.1[i] = (0, 0, esz); }   // 비정상 삼중항 = 치환 안 함
        }
    } });
    // ★StdRng(320B) — 게임 호출이 난수열을 소비하므로 **먼저 떠 둔다**(안 그러면 거짓 DIFF).
    let mut r2 = [0u8; 320]; if t { core::ptr::copy_nonoverlapping(a2, r2.as_mut_ptr(), 320); }
    // ★간접 전달 인자 a6(24B) — IR 계약이 `dead_on_return` 이라 **피호출이 훼손해도 된다**.
    //   게임 호출이 먼저 훼손하면 내 사본이 다른 입력을 받는다 ⟹ StdRng 과 같은 이유로 되돌린다.
    let mut q6 = [0u8; 24]; if t { core::ptr::copy_nonoverlapping(a6, q6.as_mut_ptr(), 24); }
    let g = f(a0, a1, a2, a3, a4, a5, a6, a7, a8);
    if !t { pop(30); return g; }
    // ★내 Vec 사본 버퍼에 안 들어간 호출은 **표본에서 뺀다** — 예전엔 `len=0` 으로
    //   떨어뜨려 **입력이 어긋난 채 비교**했고 그게 거짓 DIFF 였다(2026-09-12 차단).
    if !VB30.with(|c| (&*c.get()).2) { S[30].skip.fetch_add(1, Ordering::Relaxed); pop(30); return g; }
    let n = S[30].cmp.fetch_add(1, Ordering::Relaxed) + 1;
    let mut p2 = [0u8; 320]; core::ptr::copy_nonoverlapping(a2, p2.as_mut_ptr(), 320); // 게임 호출 후 상태
    core::ptr::copy_nonoverlapping(r2.as_ptr(), a2 as *mut u8, 320);                    // 내 사본 호출 전 = 호출 전 상태로
    let mut s6 = [0u8; 24]; core::ptr::copy_nonoverlapping(a6, s6.as_mut_ptr(), 24); // 게임 호출 후
    core::ptr::copy_nonoverlapping(q6.as_ptr(), a6 as *mut u8, 24);                    // 내 사본 호출 전 = 호출 전 상태로
    SP30.with(|c| { let sp = &mut *c.get();
        core::ptr::copy_nonoverlapping(a0, sp.as_mut_ptr(), 6168); });   // 게임 호출 후 상태
    SV30.with(|c| { let sv = &*c.get();
        core::ptr::copy_nonoverlapping(sv.as_ptr(), a0 as *mut u8, 6168); }); // 내 사본 호출 전 = 호출 전 상태로
    // ★★★힙 인식 스냅샷 — self 소유 Vec 들을 **내 힙 할당**으로 바꿔치기(내 사본의 drop/realloc 이 내 것에만 닿게).
    let ptag_0: u64 = core::ptr::read_unaligned((a0 as usize + 0x5e8) as *const u64);
    if !PV30_0.with(|c| (&*c.get()).3) {   // 스크래치에 안 들어갔다 = 표본 제외
        S[30].skip.fetch_add(1, Ordering::Relaxed);
        SP30.with(|c| { let sp = &*c.get(); core::ptr::copy_nonoverlapping(sp.as_ptr(), a0 as *mut u8, 6168); });
        core::ptr::copy_nonoverlapping(p2.as_ptr(), a2 as *mut u8, 320);   // RNG = 게임 호출 후
        core::ptr::copy_nonoverlapping(s6.as_ptr(), a6 as *mut u8, 24);
        pop(30); return g;
    }
    PV30_0.with(|c| { let pv = &*c.get(); let mut used = 0usize;
        for (i, &(off, esz, _)) in hs_vecs_12_0(ptag_0).iter().enumerate() {
            if i >= pv.2 { break; }
            let (cap, len, _) = pv.1[i];
            let b = a0 as usize + 0x5e8 + off;
            if cap > 0 && len <= cap {
                // 여유를 둔다 — 내 사본이 push 해도 realloc 없이 들어가게(realloc 도 합법이지만 덜 흔들리게)
                let ncap = cap.max(len + 64);
                if let Ok(l) = std::alloc::Layout::from_size_align(ncap * esz, 8) {
                    let blk = std::alloc::alloc(l);
                    if !blk.is_null() {
                        if len > 0 { core::ptr::copy_nonoverlapping(pv.0.as_ptr().add(used), blk, len * esz); }   // ★게임 호출 전 내용
                        core::ptr::write_unaligned(b as *mut usize, ncap);
                        core::ptr::write_unaligned((b + 8) as *mut usize, blk as usize);
                        core::ptr::write_unaligned((b + 16) as *mut usize, len);
                    }
                }
                used += len * esz;
            } else if cap == 0 {
                // 빈 Vec(cap 0) — 게임 것도 댕글링이라 그대로 둬도 free 는 안 나지만, push 가 alloc 을 부르면
                // 그 결과는 내 것이다(아래 해제가 처리). 그대로 둔다.
            }
        }
    });
    let ptag_1: u64 = 0;
    if !PV30_1.with(|c| (&*c.get()).3) {   // 스크래치에 안 들어갔다 = 표본 제외
        S[30].skip.fetch_add(1, Ordering::Relaxed);
        SP30.with(|c| { let sp = &*c.get(); core::ptr::copy_nonoverlapping(sp.as_ptr(), a0 as *mut u8, 6168); });
        core::ptr::copy_nonoverlapping(p2.as_ptr(), a2 as *mut u8, 320);   // RNG = 게임 호출 후
        core::ptr::copy_nonoverlapping(s6.as_ptr(), a6 as *mut u8, 24);
        pop(30); return g;
    }
    PV30_1.with(|c| { let pv = &*c.get(); let mut used = 0usize;
        for (i, &(off, esz, _)) in hs_vecs_12_1(ptag_1).iter().enumerate() {
            if i >= pv.2 { break; }
            let (cap, len, _) = pv.1[i];
            let b = a0 as usize + 0x0 + off;
            if cap > 0 && len <= cap {
                // 여유를 둔다 — 내 사본이 push 해도 realloc 없이 들어가게(realloc 도 합법이지만 덜 흔들리게)
                let ncap = cap.max(len + 64);
                if let Ok(l) = std::alloc::Layout::from_size_align(ncap * esz, 8) {
                    let blk = std::alloc::alloc(l);
                    if !blk.is_null() {
                        if len > 0 { core::ptr::copy_nonoverlapping(pv.0.as_ptr().add(used), blk, len * esz); }   // ★게임 호출 전 내용
                        core::ptr::write_unaligned(b as *mut usize, ncap);
                        core::ptr::write_unaligned((b + 8) as *mut usize, blk as usize);
                        core::ptr::write_unaligned((b + 16) as *mut usize, len);
                    }
                }
                used += len * esz;
            } else if cap == 0 {
                // 빈 Vec(cap 0) — 게임 것도 댕글링이라 그대로 둬도 free 는 안 나지만, push 가 alloc 을 부르면
                // 그 결과는 내 것이다(아래 해제가 처리). 그대로 둔다.
            }
        }
    });
    let m = catch_unwind(AssertUnwindSafe(|| my_12(a0, a1, a2, a3, a4, a5, a6, a7, a8)));
    // ★★★상태 diff — 이 함수는 **반환이 void** 다. ABI 상 반환이 없는 것이지
    //   **출력이 없는 게 아니다** — 출력은 `&mut self` 에 있다 ⟹ 그걸 비교한다.
    //   ⚠되돌리기·해제 **전에** 떠서 비교한다 — 내 사본이 재할당했으면 해제 후엔 못 읽는다.
    SQ30.with(|c| { let sq = &mut *c.get();
        core::ptr::copy_nonoverlapping(a0, sq.as_mut_ptr(), 6168); });
    let sd: Option<String> = SP30.with(|c| { let sp = &*c.get(); SQ30.with(|c2| { let sq = &*c2.get();
        // ① 본체 바이트(설계상 다른 구간은 제외)
        // ★소유 Vec 삼중항(cap/ptr/len 24B)은 **동적 skip**(게임 것 vs 내 할당) — len·내용은 ②′에서 비교
        let gtag_0: u64 = core::ptr::read_unaligned(sp.as_ptr().add(0x5e8) as *const u64);
        let dyn_0: &[(usize, usize, u8)] = hs_vecs_12_0(gtag_0);
        let gtag_1: u64 = 0;
        let dyn_1: &[(usize, usize, u8)] = hs_vecs_12_1(gtag_1);
        for off in 0..6168usize {
            if off >= 0x5f0 && off < 0x768 { continue; }
            if dyn_0.iter().any(|&(o, _, _)| off >= 0x5e8 + o && off < 0x5e8 + o + 24) { continue; }
            if dyn_1.iter().any(|&(o, _, _)| off >= 0x0 + o && off < 0x0 + o + 24) { continue; }
            if sp[off] != sq[off] {
                return Some(format!("self+{:#x}: g={:02x} m={:02x}", off, sp[off], sq[off]));
            }
        }
        // ①′ 열거형 필드 self+0x5e8 `BigPlan` 페이로드 — 타입 기반 live 맵(structlive)으로 variant 조건부 비교
        { let t = core::ptr::read_unaligned(sp.as_ptr().add(0x5e8) as *const u64);
          if let Some(d) = enumlive_cmp_12_0(t, sp.as_ptr() as usize + 0x5e8, sq.as_ptr() as usize + 0x5e8) {
              return Some(format!("self+{:#x}(tag {}){}", 0x5e8, t, d)); } }
        // ②′ 명세 0 소유 Vec 의 len·내용 — 요소는 ELEM_LIVE(live id)로 살아있는 바이트만
        for &(o, esz, lid) in dyn_0 {
            let (gb, mb) = (sp.as_ptr().add(0x5e8 + o), sq.as_ptr().add(0x5e8 + o));
            // ★`Option<Vec>` 은 cap 을 니치로 쓴다(상위비트 = None). 그 경우 len/ptr 은 미초기화 — 읽지 않는다.
            let (gc, mc) = (core::ptr::read_unaligned(gb as *const usize), core::ptr::read_unaligned(mb as *const usize));
            let (gn, mn) = (gc >> 63 != 0, mc >> 63 != 0);
            if gn != mn { return Some(format!("self+{:#x}.opt: g={} m={}", 0x5e8 + o, if gn { "None" } else { "Some" }, if mn { "None" } else { "Some" })); }
            if gn { continue; }
            let (gp, gl) = (core::ptr::read_unaligned(gb.add(8) as *const usize), core::ptr::read_unaligned(gb.add(16) as *const usize));
            let (mp, ml) = (core::ptr::read_unaligned(mb.add(8) as *const usize), core::ptr::read_unaligned(mb.add(16) as *const usize));
            if gl != ml { return Some(format!("self+{:#x}.len: g={} m={}", 0x5e8 + o, gl, ml)); }
            if gl > 0 && gl < 4096 && gp > 0x1000 && mp > 0x1000 && gp != mp {
                for e in 0..gl {
                    if let Some(d) = elem_cmp(lid, esz, gp + e * esz, mp + e * esz) {
                        return Some(format!("self+{:#x}[{}]{}", 0x5e8 + o, e, d));
                    }
                }
            }
        }
        // ②′ 명세 1 소유 Vec 의 len·내용 — 요소는 ELEM_LIVE(live id)로 살아있는 바이트만
        for &(o, esz, lid) in dyn_1 {
            let (gb, mb) = (sp.as_ptr().add(0x0 + o), sq.as_ptr().add(0x0 + o));
            // ★`Option<Vec>` 은 cap 을 니치로 쓴다(상위비트 = None). 그 경우 len/ptr 은 미초기화 — 읽지 않는다.
            let (gc, mc) = (core::ptr::read_unaligned(gb as *const usize), core::ptr::read_unaligned(mb as *const usize));
            let (gn, mn) = (gc >> 63 != 0, mc >> 63 != 0);
            if gn != mn { return Some(format!("self+{:#x}.opt: g={} m={}", 0x0 + o, if gn { "None" } else { "Some" }, if mn { "None" } else { "Some" })); }
            if gn { continue; }
            let (gp, gl) = (core::ptr::read_unaligned(gb.add(8) as *const usize), core::ptr::read_unaligned(gb.add(16) as *const usize));
            let (mp, ml) = (core::ptr::read_unaligned(mb.add(8) as *const usize), core::ptr::read_unaligned(mb.add(16) as *const usize));
            if gl != ml { return Some(format!("self+{:#x}.len: g={} m={}", 0x0 + o, gl, ml)); }
            if gl > 0 && gl < 4096 && gp > 0x1000 && mp > 0x1000 && gp != mp {
                for e in 0..gl {
                    if let Some(d) = elem_cmp(lid, esz, gp + e * esz, mp + e * esz) {
                        return Some(format!("self+{:#x}[{}]{}", 0x0 + o, e, d));
                    }
                }
            }
        }
        None
    }) });
    // ★내 사본이 남긴 소유 Vec 을 해제한다 — 이 시점에 그 포인터는 **전부 내 것**이다
    //   (게임 것은 위에서 내 할당으로 바꿔치기됐고, 새로 만든 것은 내 사본이 할당했다).
    //   ⚠먼저 **내 사본이 새로 push 한 요소**의 String(ELEM_LIVE.str)을 해제한다 — pre-call len 미만은 게임 버퍼의 복사본.
    { let t2: u64 = core::ptr::read_unaligned((a0 as usize + 0x5e8) as *const u64);
      PV30_0.with(|c| { let pv = &*c.get();
      for (i, &(off, esz, lid)) in hs_vecs_12_0(t2).iter().enumerate() {
          let b = a0 as usize + 0x5e8 + off;
          let (cap, ptr, len) = (core::ptr::read_unaligned(b as *const usize), core::ptr::read_unaligned((b + 8) as *const usize),
                                 core::ptr::read_unaligned((b + 16) as *const usize));
          if cap > 0 && cap < (1 << 20) && ptr > 0x1000 {
              let pre = if i < pv.2 { pv.1[i].1 } else { 0 };
              if BISECT_NO_STRFREE == 0 && lid != 0 && len > pre && len < 4096 { for e in pre..len { elem_free_str(lid, ptr + e * esz); } }
              if let Ok(l) = std::alloc::Layout::from_size_align(cap * esz, 8) { std::alloc::dealloc(ptr as *mut u8, l); }
          }
      } }); }
    { let t2: u64 = 0;
      PV30_1.with(|c| { let pv = &*c.get();
      for (i, &(off, esz, lid)) in hs_vecs_12_1(t2).iter().enumerate() {
          let b = a0 as usize + 0x0 + off;
          let (cap, ptr, len) = (core::ptr::read_unaligned(b as *const usize), core::ptr::read_unaligned((b + 8) as *const usize),
                                 core::ptr::read_unaligned((b + 16) as *const usize));
          if cap > 0 && cap < (1 << 20) && ptr > 0x1000 {
              let pre = if i < pv.2 { pv.1[i].1 } else { 0 };
              if BISECT_NO_STRFREE == 0 && lid != 0 && len > pre && len < 4096 { for e in pre..len { elem_free_str(lid, ptr + e * esz); } }
              if let Ok(l) = std::alloc::Layout::from_size_align(cap * esz, 8) { std::alloc::dealloc(ptr as *mut u8, l); }
          }
      } }); }
    SP30.with(|c| { let sp = &*c.get();
        core::ptr::copy_nonoverlapping(sp.as_ptr(), a0 as *mut u8, 6168); }); // 게임 호출 후 상태로 복구
    core::ptr::copy_nonoverlapping(p2.as_ptr(), a2 as *mut u8, 320);                    // 게임 호출 후 상태로 복구(게임 진행은 게임 값으로)
    core::ptr::copy_nonoverlapping(s6.as_ptr(), a6 as *mut u8, 24);                    // 게임 호출 후 상태로 복구
    match m {
        Ok(_) => { if let Some(d) = sd {
            note(30, format!("#12 handle_chat 대조#{} **상태갈림**: {} | (반환 void — 출력은 &mut self 다) | a0={:#x} a1={} a2={:#x} a3={:#x} a4={:#x} a5={} a6={:#x} a7={} a8={:#x}", n, d, a0 as usize, a1, a2 as usize, a3 as usize, a4 as usize, a5, a6 as usize, a7, a8 as usize)); } }
        Err(_) => { S[30].pan.fetch_add(1, Ordering::Relaxed); }
    }
    pop(30);
    g
}
unsafe fn w_52(a0: *const u8, a1: *const u8, a2: u8) -> bool {
    S[31].calls.fetch_add(1, Ordering::Relaxed);
    let f: unsafe fn(*const u8, *const u8, u8) -> bool = core::mem::transmute(S[31].orig.load(Ordering::Relaxed));
    let t = top(31);
    let g = f(a0, a1, a2);
    if !t { pop(31); return g; }
    let n = S[31].cmp.fetch_add(1, Ordering::Relaxed) + 1;
    let m = catch_unwind(AssertUnwindSafe(|| my_52(a0, a1, a2)));
    match m {
        Ok(m) => { if m != g { note(31, format!("#52 v25_objective_far_split_pressure 대조#{} 갈림: g={:?} m={:?} | a0={:#x} a1={:#x} a2={}", n, g, m, a0 as usize, a1 as usize, a2)); } }
        Err(_) => { S[31].pan.fetch_add(1, Ordering::Relaxed); }
    }
    pop(31);
    g
}
unsafe fn w_48(a0: i64, a1: u32, a2: *const u8, a3: *const u8, a4: i64, a5: i64) -> u32 {
    S[32].calls.fetch_add(1, Ordering::Relaxed);
    let f: unsafe fn(i64, u32, *const u8, *const u8, i64, i64) -> u32 = core::mem::transmute(S[32].orig.load(Ordering::Relaxed));
    let t = top(32);
    let g = f(a0, a1, a2, a3, a4, a5);
    if !t { pop(32); return g; }
    let n = S[32].cmp.fetch_add(1, Ordering::Relaxed) + 1;
    let m = catch_unwind(AssertUnwindSafe(|| my_48(a0, a1, a2, a3, a4, a5)));
    match m {
        Ok(m) => { if (m & 0xffffff) != (g & 0xffffff) { note(32, format!("#48 v25_scoped_battle_objective 대조#{} 갈림(i24): g={:#x} m={:#x} | a0={} a1={} a2={:#x} a3={:#x} a4={} a5={}", n, g & 0xffffff, m & 0xffffff, a0, a1, a2 as usize, a3 as usize, a4, a5)); } }
        Err(_) => { S[32].pan.fetch_add(1, Ordering::Relaxed); }
    }
    pop(32);
    g
}
unsafe fn w_19(a0: i64, a1: *const u8, a2: *const u8, a3: *const u8, a4: *const u8, a5: u8, a6: *const u8) -> u8 {
    S[33].calls.fetch_add(1, Ordering::Relaxed);
    let f: unsafe fn(i64, *const u8, *const u8, *const u8, *const u8, u8, *const u8) -> u8 = core::mem::transmute(S[33].orig.load(Ordering::Relaxed));
    let t = top(33);
    // ★StdRng(320B) — 게임 호출이 난수열을 소비하므로 **먼저 떠 둔다**(안 그러면 거짓 DIFF).
    let mut r1 = [0u8; 320]; if t { core::ptr::copy_nonoverlapping(a1, r1.as_mut_ptr(), 320); }
    let g = f(a0, a1, a2, a3, a4, a5, a6);
    if !t { pop(33); return g; }
    let n = S[33].cmp.fetch_add(1, Ordering::Relaxed) + 1;
    let mut p1 = [0u8; 320]; core::ptr::copy_nonoverlapping(a1, p1.as_mut_ptr(), 320); // 게임 호출 후 상태
    core::ptr::copy_nonoverlapping(r1.as_ptr(), a1 as *mut u8, 320);                    // 내 사본 호출 전 = 호출 전 상태로
    let m = catch_unwind(AssertUnwindSafe(|| my_19(a0, a1, a2, a3, a4, a5, a6)));
    core::ptr::copy_nonoverlapping(p1.as_ptr(), a1 as *mut u8, 320);                    // 게임 호출 후 상태로 복구(게임 진행은 게임 값으로)
    match m {
        Ok(m) => { if m != g { note(33, format!("#19 best_jungle_goal 대조#{} 갈림: g={:?} m={:?} | a0={} a1={:#x} a2={:#x} a3={:#x} a4={:#x} a5={} a6={:#x}", n, g, m, a0, a1 as usize, a2 as usize, a3 as usize, a4 as usize, a5, a6 as usize)); } }
        Err(_) => { S[33].pan.fetch_add(1, Ordering::Relaxed); }
    }
    pop(33);
    g
}
unsafe fn w_34(a0: *const u8, a1: *const u8, a2: u8, a3: i64) -> bool {
    S[34].calls.fetch_add(1, Ordering::Relaxed);
    let f: unsafe fn(*const u8, *const u8, u8, i64) -> bool = core::mem::transmute(S[34].orig.load(Ordering::Relaxed));
    let t = top(34);
    let g = f(a0, a1, a2, a3);
    if !t { pop(34); return g; }
    let n = S[34].cmp.fetch_add(1, Ordering::Relaxed) + 1;
    let m = catch_unwind(AssertUnwindSafe(|| my_34(a0, a1, a2, a3)));
    match m {
        Ok(m) => { if m != g { note(34, format!("#34 has_line_defense_threat 대조#{} 갈림: g={:?} m={:?} | a0={:#x} a1={:#x} a2={} a3={}", n, g, m, a0 as usize, a1 as usize, a2, a3)); } }
        Err(_) => { S[34].pan.fetch_add(1, Ordering::Relaxed); }
    }
    pop(34);
    g
}
unsafe fn w_29(a0: *const u8, a1: *const u8, a2: i64, a3: i64, a4: i64, a5: i64) -> i64 {
    S[35].calls.fetch_add(1, Ordering::Relaxed);
    let f: unsafe fn(*const u8, *const u8, i64, i64, i64, i64) -> i64 = core::mem::transmute(S[35].orig.load(Ordering::Relaxed));
    let t = top(35);
    let g = f(a0, a1, a2, a3, a4, a5);
    if !t { pop(35); return g; }
    let n = S[35].cmp.fetch_add(1, Ordering::Relaxed) + 1;
    let m = catch_unwind(AssertUnwindSafe(|| my_29(a0, a1, a2, a3, a4, a5)));
    match m {
        Ok(m) => { if m != g { note(35, format!("#29 v23_recent_visible_enemies_near_point 대조#{} 갈림: g={:?} m={:?} | a0={:#x} a1={:#x} a2={} a3={} a4={} a5={}", n, g, m, a0 as usize, a1 as usize, a2, a3, a4, a5)); } }
        Err(_) => { S[35].pan.fetch_add(1, Ordering::Relaxed); }
    }
    pop(35);
    g
}
unsafe fn w_31(a0: i64, a1: i32, a2: *const u8, a3: *const u8) -> P8 {
    S[36].calls.fetch_add(1, Ordering::Relaxed);
    let f: unsafe fn(i64, i32, *const u8, *const u8) -> P8 = core::mem::transmute(S[36].orig.load(Ordering::Relaxed));
    let t = top(36);
    let g = f(a0, a1, a2, a3);
    if !t { pop(36); return g; }
    let n = S[36].cmp.fetch_add(1, Ordering::Relaxed) + 1;
    let m = catch_unwind(AssertUnwindSafe(|| my_31(a0, a1, a2, a3)));
    match m {
        Ok(m) => { if m != g { note(36, format!("#31 v3_epic_formation_role 대조#{} 갈림: g={:?} m={:?} | a0={} a1={} a2={:#x} a3={:#x}", n, g, m, a0, a1, a2 as usize, a3 as usize)); } }
        Err(_) => { S[36].pan.fetch_add(1, Ordering::Relaxed); }
    }
    pop(36);
    g
}
unsafe fn w_20(a0: *const u8, a1: *const u8, a2: i64, a3: *const u8, a4: u8) {
    S[37].calls.fetch_add(1, Ordering::Relaxed);
    let f: unsafe fn(*const u8, *const u8, i64, *const u8, u8) = core::mem::transmute(S[37].orig.load(Ordering::Relaxed));
    let t = top(37);
    let g = f(a0, a1, a2, a3, a4);
    if !t { pop(37); return g; }
    let n = S[37].cmp.fetch_add(1, Ordering::Relaxed) + 1;
    let mut gb = [0u64; 4]; core::ptr::copy_nonoverlapping(a0, gb.as_mut_ptr() as *mut u8, 32); // 게임 출력 사본
    let mut mb = [0u64; 4];                                                   // 내 사본 전용 출력 버퍼
    let m = catch_unwind(AssertUnwindSafe(|| my_20(mb.as_mut_ptr() as *const u8, a1, a2, a3, a4)));
    match m {
        Ok(_) => { if let Some(off) = live_eq(a0, mb.as_ptr() as *const u8, LIVE_20) {
            note(37, format!("#20 v27_active_objective_discipline 대조#{} 갈림(sret 32B · +{:#x}): g={:02x?} m={:02x?} | a1={:#x} a2={} a3={:#x} a4={}", n, off, &gb[..4], &mb[..4], a1 as usize, a2, a3 as usize, a4)); } }
        Err(_) => { S[37].pan.fetch_add(1, Ordering::Relaxed); }
    }
    pop(37);
    g
}
unsafe fn w_25(a0: i64, a1: *const u8, a2: *const u8, a3: *const u8, a4: *const u8) -> bool {
    S[38].calls.fetch_add(1, Ordering::Relaxed);
    let f: unsafe fn(i64, *const u8, *const u8, *const u8, *const u8) -> bool = core::mem::transmute(S[38].orig.load(Ordering::Relaxed));
    let t = top(38);
    let g = f(a0, a1, a2, a3, a4);
    if !t { pop(38); return g; }
    let n = S[38].cmp.fetch_add(1, Ordering::Relaxed) + 1;
    let m = catch_unwind(AssertUnwindSafe(|| my_25(a0, a1, a2, a3, a4)));
    match m {
        Ok(m) => { if m != g { note(38, format!("#25 v22_visible_enemy_is_runaway_threat 대조#{} 갈림: g={:?} m={:?} | a0={} a1={:#x} a2={:#x} a3={:#x} a4={:#x}", n, g, m, a0, a1 as usize, a2 as usize, a3 as usize, a4 as usize)); } }
        Err(_) => { S[38].pan.fetch_add(1, Ordering::Relaxed); }
    }
    pop(38);
    g
}
unsafe fn w_56(a0: *const u8, a1: *const u8, a2: i64, a3: *const u8, a4: *const u8, a5: *const u8, a6: *const u8, a7: *const u8) {
    S[39].calls.fetch_add(1, Ordering::Relaxed);
    let f: unsafe fn(*const u8, *const u8, i64, *const u8, *const u8, *const u8, *const u8, *const u8) = core::mem::transmute(S[39].orig.load(Ordering::Relaxed));
    let t = top(39);
    // ★StdRng(320B) — 게임 호출이 난수열을 소비하므로 **먼저 떠 둔다**(안 그러면 거짓 DIFF).
    let mut r3 = [0u8; 320]; if t { core::ptr::copy_nonoverlapping(a3, r3.as_mut_ptr(), 320); }
    let g = f(a0, a1, a2, a3, a4, a5, a6, a7);
    if !t { pop(39); return g; }
    let n = S[39].cmp.fetch_add(1, Ordering::Relaxed) + 1;
    let mut p3 = [0u8; 320]; core::ptr::copy_nonoverlapping(a3, p3.as_mut_ptr(), 320); // 게임 호출 후 상태
    core::ptr::copy_nonoverlapping(r3.as_ptr(), a3 as *mut u8, 320);                    // 내 사본 호출 전 = 호출 전 상태로
    let mut gb = [0u64; 9]; core::ptr::copy_nonoverlapping(a0, gb.as_mut_ptr() as *mut u8, 72); // 게임 출력 사본
    let mut mb = [0u64; 9];                                                   // 내 사본 전용 출력 버퍼
    let m = catch_unwind(AssertUnwindSafe(|| my_56(mb.as_mut_ptr() as *const u8, a1, a2, a3, a4, a5, a6, a7)));
    core::ptr::copy_nonoverlapping(p3.as_ptr(), a3 as *mut u8, 320);                    // 게임 호출 후 상태로 복구(게임 진행은 게임 값으로)
    match m {
        Ok(_) => { let (gt, mt) = (gb[0], mb[0]);
            let d = if gt != mt { Some(format!("tag g={} m={}", gt, mt)) } else { enumlive_cmp_56_0(gt, gb.as_ptr() as usize, mb.as_ptr() as usize).map(|x| format!("(tag {}){}", gt, x)) };
            if let Some(d) = d { note(39, format!("#56 PassiveLinePlan::sub_plan 대조#{} 갈림(sret 열거형 72B): {} | g={:02x?} m={:02x?} | a1={:#x} a2={} a3={:#x} a4={:#x} a5={:#x} a6={:#x} a7={:#x}", n, d, &gb[..9], &mb[..9], a1 as usize, a2, a3 as usize, a4 as usize, a5 as usize, a6 as usize, a7 as usize)); } }
        Err(_) => { S[39].pan.fetch_add(1, Ordering::Relaxed); }
    }
    pop(39);
    g
}
unsafe fn w_51(a0: i64, a1: *const u8, a2: *const u8, a3: *const u8, a4: *const u8, a5: u8, a6: *const u8) -> u8 {
    S[40].calls.fetch_add(1, Ordering::Relaxed);
    let f: unsafe fn(i64, *const u8, *const u8, *const u8, *const u8, u8, *const u8) -> u8 = core::mem::transmute(S[40].orig.load(Ordering::Relaxed));
    let t = top(40);
    let g = f(a0, a1, a2, a3, a4, a5, a6);
    if !t { pop(40); return g; }
    let n = S[40].cmp.fetch_add(1, Ordering::Relaxed) + 1;
    let m = catch_unwind(AssertUnwindSafe(|| my_51(a0, a1, a2, a3, a4, a5, a6)));
    match m {
        Ok(m) => { if m != g { note(40, format!("#51 check_press_tower_opportunity 대조#{} 갈림: g={:?} m={:?} | a0={} a1={:#x} a2={:#x} a3={:#x} a4={:#x} a5={} a6={:#x}", n, g, m, a0, a1 as usize, a2 as usize, a3 as usize, a4 as usize, a5, a6 as usize)); } }
        Err(_) => { S[40].pan.fetch_add(1, Ordering::Relaxed); }
    }
    pop(40);
    g
}
unsafe fn w_16(a0: *const u8, a1: *const u8, a2: i64) -> i64 {
    S[41].calls.fetch_add(1, Ordering::Relaxed);
    let f: unsafe fn(*const u8, *const u8, i64) -> i64 = core::mem::transmute(S[41].orig.load(Ordering::Relaxed));
    let t = top(41);
    let g = f(a0, a1, a2);
    if !t { pop(41); return g; }
    let n = S[41].cmp.fetch_add(1, Ordering::Relaxed) + 1;
    let m = catch_unwind(AssertUnwindSafe(|| my_16(a0, a1, a2)));
    match m {
        Ok(m) => { if m != g { note(41, format!("#16 max_range_nearly_can_use 대조#{} 갈림: g={:?} m={:?} | a0={:#x} a1={:#x} a2={}", n, g, m, a0 as usize, a1 as usize, a2)); } }
        Err(_) => { S[41].pan.fetch_add(1, Ordering::Relaxed); }
    }
    pop(41);
    g
}
unsafe fn w_45(a0: *const u8, a1: *const u8, a2: *const u8) -> bool {
    S[42].calls.fetch_add(1, Ordering::Relaxed);
    let f: unsafe fn(*const u8, *const u8, *const u8) -> bool = core::mem::transmute(S[42].orig.load(Ordering::Relaxed));
    let t = top(42);
    // ★StdRng(320B) — 게임 호출이 난수열을 소비하므로 **먼저 떠 둔다**(안 그러면 거짓 DIFF).
    let mut r0 = [0u8; 320]; if t { core::ptr::copy_nonoverlapping(a0, r0.as_mut_ptr(), 320); }
    let g = f(a0, a1, a2);
    if !t { pop(42); return g; }
    let n = S[42].cmp.fetch_add(1, Ordering::Relaxed) + 1;
    let mut p0 = [0u8; 320]; core::ptr::copy_nonoverlapping(a0, p0.as_mut_ptr(), 320); // 게임 호출 후 상태
    core::ptr::copy_nonoverlapping(r0.as_ptr(), a0 as *mut u8, 320);                    // 내 사본 호출 전 = 호출 전 상태로
    let m = catch_unwind(AssertUnwindSafe(|| my_45(a0, a1, a2)));
    core::ptr::copy_nonoverlapping(p0.as_ptr(), a0 as *mut u8, 320);                    // 게임 호출 후 상태로 복구(게임 진행은 게임 값으로)
    match m {
        Ok(m) => { if m != g { note(42, format!("#45 can_recall 대조#{} 갈림: g={:?} m={:?} | a0={:#x} a1={:#x} a2={:#x}", n, g, m, a0 as usize, a1 as usize, a2 as usize)); } }
        Err(_) => { S[42].pan.fetch_add(1, Ordering::Relaxed); }
    }
    pop(42);
    g
}
unsafe fn w_53(a0: i64, a1: *const u8, a2: *const u8, a3: *const u8, a4: *const u8, a5: *const u8) -> bool {
    S[43].calls.fetch_add(1, Ordering::Relaxed);
    let f: unsafe fn(i64, *const u8, *const u8, *const u8, *const u8, *const u8) -> bool = core::mem::transmute(S[43].orig.load(Ordering::Relaxed));
    let t = top(43);
    let g = f(a0, a1, a2, a3, a4, a5);
    if !t { pop(43); return g; }
    let n = S[43].cmp.fetch_add(1, Ordering::Relaxed) + 1;
    let m = catch_unwind(AssertUnwindSafe(|| my_53(a0, a1, a2, a3, a4, a5)));
    match m {
        Ok(m) => { if m != g { note(43, format!("#53 check_epic_setup 대조#{} 갈림: g={:?} m={:?} | a0={} a1={:#x} a2={:#x} a3={:#x} a4={:#x} a5={:#x}", n, g, m, a0, a1 as usize, a2 as usize, a3 as usize, a4 as usize, a5 as usize)); } }
        Err(_) => { S[43].pan.fetch_add(1, Ordering::Relaxed); }
    }
    pop(43);
    g
}
unsafe fn w_23(a0: i64, a1: *const u8, a2: *const u8, a3: *const u8, a4: *const u8, a5: *const u8) -> P64 {
    S[44].calls.fetch_add(1, Ordering::Relaxed);
    let f: unsafe fn(i64, *const u8, *const u8, *const u8, *const u8, *const u8) -> P64 = core::mem::transmute(S[44].orig.load(Ordering::Relaxed));
    let t = top(44);
    // ★StdRng(320B) — 게임 호출이 난수열을 소비하므로 **먼저 떠 둔다**(안 그러면 거짓 DIFF).
    let mut r1 = [0u8; 320]; if t { core::ptr::copy_nonoverlapping(a1, r1.as_mut_ptr(), 320); }
    let g = f(a0, a1, a2, a3, a4, a5);
    if !t { pop(44); return g; }
    let n = S[44].cmp.fetch_add(1, Ordering::Relaxed) + 1;
    let mut p1 = [0u8; 320]; core::ptr::copy_nonoverlapping(a1, p1.as_mut_ptr(), 320); // 게임 호출 후 상태
    core::ptr::copy_nonoverlapping(r1.as_ptr(), a1 as *mut u8, 320);                    // 내 사본 호출 전 = 호출 전 상태로
    let m = catch_unwind(AssertUnwindSafe(|| my_23(a0, a1, a2, a3, a4, a5)));
    core::ptr::copy_nonoverlapping(p1.as_ptr(), a1 as *mut u8, 320);                    // 게임 호출 후 상태로 복구(게임 진행은 게임 값으로)
    match m {
        Ok(m) => { if m != g { note(44, format!("#23 buy_item 대조#{} 갈림: g={:?} m={:?} | a0={} a1={:#x} a2={:#x} a3={:#x} a4={:#x} a5={:#x}", n, g, m, a0, a1 as usize, a2 as usize, a3 as usize, a4 as usize, a5 as usize)); } }
        Err(_) => { S[44].pan.fetch_add(1, Ordering::Relaxed); }
    }
    pop(44);
    g
}
unsafe fn w_46(a0: i64, a1: *const u8, a2: *const u8, a3: *const u8, a4: *const u8) -> bool {
    S[45].calls.fetch_add(1, Ordering::Relaxed);
    let f: unsafe fn(i64, *const u8, *const u8, *const u8, *const u8) -> bool = core::mem::transmute(S[45].orig.load(Ordering::Relaxed));
    let t = top(45);
    // ★StdRng(320B) — 게임 호출이 난수열을 소비하므로 **먼저 떠 둔다**(안 그러면 거짓 DIFF).
    let mut r1 = [0u8; 320]; if t { core::ptr::copy_nonoverlapping(a1, r1.as_mut_ptr(), 320); }
    let g = f(a0, a1, a2, a3, a4);
    if !t { pop(45); return g; }
    let n = S[45].cmp.fetch_add(1, Ordering::Relaxed) + 1;
    let mut p1 = [0u8; 320]; core::ptr::copy_nonoverlapping(a1, p1.as_mut_ptr(), 320); // 게임 호출 후 상태
    core::ptr::copy_nonoverlapping(r1.as_ptr(), a1 as *mut u8, 320);                    // 내 사본 호출 전 = 호출 전 상태로
    let m = catch_unwind(AssertUnwindSafe(|| my_46(a0, a1, a2, a3, a4)));
    core::ptr::copy_nonoverlapping(p1.as_ptr(), a1 as *mut u8, 320);                    // 게임 호출 후 상태로 복구(게임 진행은 게임 값으로)
    match m {
        Ok(m) => { if m != g { note(45, format!("#46 need_defense_nexus 대조#{} 갈림: g={:?} m={:?} | a0={} a1={:#x} a2={:#x} a3={:#x} a4={:#x}", n, g, m, a0, a1 as usize, a2 as usize, a3 as usize, a4 as usize)); } }
        Err(_) => { S[45].pan.fetch_add(1, Ordering::Relaxed); }
    }
    pop(45);
    g
}
unsafe fn w_21(a0: *const u8, a1: i64, a2: *const u8, a3: *const u8, a4: *const u8, a5: *const u8, a6: *const u8) {
    S[46].calls.fetch_add(1, Ordering::Relaxed);
    let f: unsafe fn(*const u8, i64, *const u8, *const u8, *const u8, *const u8, *const u8) = core::mem::transmute(S[46].orig.load(Ordering::Relaxed));
    let t = top(46);
    // ★StdRng(320B) — 게임 호출이 난수열을 소비하므로 **먼저 떠 둔다**(안 그러면 거짓 DIFF).
    let mut r2 = [0u8; 320]; if t { core::ptr::copy_nonoverlapping(a2, r2.as_mut_ptr(), 320); }
    let g = f(a0, a1, a2, a3, a4, a5, a6);
    if !t { pop(46); return g; }
    let n = S[46].cmp.fetch_add(1, Ordering::Relaxed) + 1;
    let mut p2 = [0u8; 320]; core::ptr::copy_nonoverlapping(a2, p2.as_mut_ptr(), 320); // 게임 호출 후 상태
    core::ptr::copy_nonoverlapping(r2.as_ptr(), a2 as *mut u8, 320);                    // 내 사본 호출 전 = 호출 전 상태로
    let mut gb = [0u64; 3]; core::ptr::copy_nonoverlapping(a0, gb.as_mut_ptr() as *mut u8, 24); // 게임 출력 사본
    let mut mb = [0u64; 3];                                                   // 내 사본 전용 출력 버퍼
    let m = catch_unwind(AssertUnwindSafe(|| my_21(mb.as_mut_ptr() as *const u8, a1, a2, a3, a4, a5, a6)));
    core::ptr::copy_nonoverlapping(p2.as_ptr(), a2 as *mut u8, 320);                    // 게임 호출 후 상태로 복구(게임 진행은 게임 값으로)
    match m {
        Ok(_) => { if let Some(off) = live_eq(a0, mb.as_ptr() as *const u8, LIVE_21) {
            note(46, format!("#21 upgrade_item 대조#{} 갈림(sret 24B · +{:#x}): g={:02x?} m={:02x?} | a1={} a2={:#x} a3={:#x} a4={:#x} a5={:#x} a6={:#x}", n, off, &gb[..3], &mb[..3], a1, a2 as usize, a3 as usize, a4 as usize, a5 as usize, a6 as usize)); } }
        Err(_) => { S[46].pan.fetch_add(1, Ordering::Relaxed); }
    }
    pop(46);
    g
}
unsafe fn w_55(a0: *const u8, a1: i64, a2: *const u8, a3: *const u8, a4: *const u8, a5: *const u8, a6: *const u8, a7: *const u8, a8: u8) {
    S[47].calls.fetch_add(1, Ordering::Relaxed);
    let f: unsafe fn(*const u8, i64, *const u8, *const u8, *const u8, *const u8, *const u8, *const u8, u8) = core::mem::transmute(S[47].orig.load(Ordering::Relaxed));
    let t = top(47);
    let g = f(a0, a1, a2, a3, a4, a5, a6, a7, a8);
    if !t { pop(47); return g; }
    let n = S[47].cmp.fetch_add(1, Ordering::Relaxed) + 1;
    let mut gb = [0u64; 53]; core::ptr::copy_nonoverlapping(a0, gb.as_mut_ptr() as *mut u8, 424); // 게임 출력 사본
    let mut mb = [0u64; 53];                                                   // 내 사본 전용 출력 버퍼
    let m = catch_unwind(AssertUnwindSafe(|| my_55(mb.as_mut_ptr() as *const u8, a1, a2, a3, a4, a5, a6, a7, a8)));
    match m {
        Ok(_) => { if let Some(off) = live_eq(a0, mb.as_ptr() as *const u8, LIVE_55) {
            note(47, format!("#55 EntityPositioningCache::new 대조#{} 갈림(sret 424B · +{:#x}): g={:02x?} m={:02x?} | a1={} a2={:#x} a3={:#x} a4={:#x} a5={:#x} a6={:#x} a7={:#x} a8={}", n, off, &gb[..53], &mb[..53], a1, a2 as usize, a3 as usize, a4 as usize, a5 as usize, a6 as usize, a7 as usize, a8)); } }
        Err(_) => { S[47].pan.fetch_add(1, Ordering::Relaxed); }
    }
    pop(47);
    g
}
unsafe fn w_38(a0: *const u8, a1: *const u8, a2: *const u8, a3: i64, a4: i64, a5: i64) -> bool {
    S[48].calls.fetch_add(1, Ordering::Relaxed);
    let f: unsafe fn(*const u8, *const u8, *const u8, i64, i64, i64) -> bool = core::mem::transmute(S[48].orig.load(Ordering::Relaxed));
    let t = top(48);
    let g = f(a0, a1, a2, a3, a4, a5);
    if !t { pop(48); return g; }
    let n = S[48].cmp.fetch_add(1, Ordering::Relaxed) + 1;
    let m = catch_unwind(AssertUnwindSafe(|| my_38(a0, a1, a2, a3, a4, a5)));
    match m {
        Ok(m) => { if m != g { note(48, format!("#38 can_tower_focused_when_battle 대조#{} 갈림: g={:?} m={:?} | a0={:#x} a1={:#x} a2={:#x} a3={} a4={} a5={}", n, g, m, a0 as usize, a1 as usize, a2 as usize, a3, a4, a5)); } }
        Err(_) => { S[48].pan.fetch_add(1, Ordering::Relaxed); }
    }
    pop(48);
    g
}
unsafe fn w_54(a0: *const u8, a1: i64, a2: *const u8, a3: *const u8, a4: *const u8, a5: *const u8) {
    S[49].calls.fetch_add(1, Ordering::Relaxed);
    let f: unsafe fn(*const u8, i64, *const u8, *const u8, *const u8, *const u8) = core::mem::transmute(S[49].orig.load(Ordering::Relaxed));
    let t = top(49);
    // ★★&mut 게임 상태(1064B) — **게임 호출 전** 상태를 떠 두지 않으면 내 사본은
    //   게임이 바꿔놓은 입력을 보게 돼 **거짓 DIFF** 가 난다(이 함수는 self 를 읽고도 쓴다).
    // ★★**스택이 아니라 thread_local 힙**에 둔다 — 1064B × 2 를 스택에 잡으면
    //   rayon 워커의 **소스택 + 게임 sim 재귀**에서 STATUS_STACK_OVERFLOW 로 죽는다
    //   (2026-09-12 실사고: 스택 배열로 두던 판이 배경 sim 시작 직후 패닉로그 없이 즉사).
    SV49.with(|c| { let sv = &mut *c.get();
        if t { core::ptr::copy_nonoverlapping(a0, sv.as_mut_ptr(), 1064); } });
    VB49.with(|c| { let vb = &mut *c.get(); if t {
        let ln = core::ptr::read_unaligned((a0 as usize + 0xd0) as *const usize);
        let pz = core::ptr::read_unaligned((a0 as usize + 0xc8) as *const usize);
        vb.1 = ln; vb.2 = ln + 2 <= 512;
        if ln > 0 && ln + 2 <= 512 && pz > 0x1000 {
            core::ptr::copy_nonoverlapping(pz as *const u8, vb.0.as_mut_ptr(), ln * 24);
        } else { vb.1 = ln; vb.2 = ln + 2 <= 512; }
    } });
    // ★StdRng(320B) — 게임 호출이 난수열을 소비하므로 **먼저 떠 둔다**(안 그러면 거짓 DIFF).
    let mut r2 = [0u8; 320]; if t { core::ptr::copy_nonoverlapping(a2, r2.as_mut_ptr(), 320); }
    let g = f(a0, a1, a2, a3, a4, a5);
    if !t { pop(49); return g; }
    // ★내 Vec 사본 버퍼에 안 들어간 호출은 **표본에서 뺀다** — 예전엔 `len=0` 으로
    //   떨어뜨려 **입력이 어긋난 채 비교**했고 그게 거짓 DIFF 였다(2026-09-12 차단).
    if !VB49.with(|c| (&*c.get()).2) { S[49].skip.fetch_add(1, Ordering::Relaxed); pop(49); return g; }
    let n = S[49].cmp.fetch_add(1, Ordering::Relaxed) + 1;
    let mut p2 = [0u8; 320]; core::ptr::copy_nonoverlapping(a2, p2.as_mut_ptr(), 320); // 게임 호출 후 상태
    core::ptr::copy_nonoverlapping(r2.as_ptr(), a2 as *mut u8, 320);                    // 내 사본 호출 전 = 호출 전 상태로
    SP49.with(|c| { let sp = &mut *c.get();
        core::ptr::copy_nonoverlapping(a0, sp.as_mut_ptr(), 1064); });   // 게임 호출 후 상태
    SV49.with(|c| { let sv = &*c.get();
        core::ptr::copy_nonoverlapping(sv.as_ptr(), a0 as *mut u8, 1064); }); // 내 사본 호출 전 = 호출 전 상태로
    // ★Vec 을 **빈 것**으로 — cap=0 이면 push 가 realloc 이 아니라 alloc 을 하므로
    //   **게임의 버퍼를 절대 해제하지 않는다**(순서는 IR 실측: cap@0xc0 · ptr@0xc8 · len@0xd0).
    VB49.with(|c| { let vb = &*c.get();
        core::ptr::write_unaligned((a0 as usize + 0xc0) as *mut usize, 512);  // cap = 내 버퍼 용량
        core::ptr::write_unaligned((a0 as usize + 0xc8) as *mut usize, vb.0.as_ptr() as usize); // ptr = 내 버퍼
        core::ptr::write_unaligned((a0 as usize + 0xd0) as *mut usize, vb.1);  // len = 게임과 **같은** 개수
    });
    let m = catch_unwind(AssertUnwindSafe(|| my_54(a0, a1, a2, a3, a4, a5)));
    // ★★★상태 diff — 이 함수는 **반환이 void** 다. ABI 상 반환이 없는 것이지
    //   **출력이 없는 게 아니다** — 출력은 `&mut self` 에 있다 ⟹ 그걸 비교한다.
    //   ⚠되돌리기·해제 **전에** 떠서 비교한다 — 내 사본이 재할당했으면 해제 후엔 못 읽는다.
    let (m_ptr, m_len) = (core::ptr::read_unaligned((a0 as usize + 0xc8) as *const usize),
                          core::ptr::read_unaligned((a0 as usize + 0xd0) as *const usize));
    SQ49.with(|c| { let sq = &mut *c.get();
        core::ptr::copy_nonoverlapping(a0, sq.as_mut_ptr(), 1064); });
    let sd: Option<String> = SP49.with(|c| { let sp = &*c.get(); SQ49.with(|c2| { let sq = &*c2.get();
        // ① 본체 바이트(설계상 다른 구간은 제외)
        for off in 0..1064usize {
            if off >= 0xc0 && off < 0xd8 { continue; }
            if sp[off] != sq[off] {
                return Some(format!("self+{:#x}: g={:02x} m={:02x}", off, sp[off], sq[off]));
            }
        }
        // ② Vec 의 len (cap/ptr 은 버퍼가 달라 비교 대상이 아니다)
        let g_ptr = core::ptr::read_unaligned(sp.as_ptr().add(0xc8) as *const usize);
        let g_len = core::ptr::read_unaligned(sp.as_ptr().add(0xd0) as *const usize);
        if g_len != m_len {
            return Some(format!("vec.len: g={} m={}", g_len, m_len));
        }
        // ③ Vec 의 **내용** (요소 24B)
        //   ★요소는 열거형이라 **variant 별로 쓰는 칸이 다르다** — 안 쓰는 칸은
        //   **재사용된 버퍼의 잔재**라 양쪽이 다른 게 정상이다(IR 실측 기반 live 범위만 본다).
        //   (off, len, 이 tag 들에서만 live · 빈 것 = 항상)
        const EL: &[(usize, usize, &[u8])] = &[(0, 1, &[]), (4, 4, &[0x01]), (8, 8, &[])];
        if g_len > 0 && g_len < 4096 && g_ptr > 0x1000 && m_ptr > 0x1000 {
            for e in 0..g_len {
                let (gb, mb) = (g_ptr + e * 24, m_ptr + e * 24);
                let tag = *(gb as *const u8);
                for &(o, l, tg) in EL {
                    if !tg.is_empty() && !tg.contains(&tag) { continue; }
                    for j in o..o + l {
                        let (gv, mv) = (*((gb + j) as *const u8), *((mb + j) as *const u8));
                        if gv != mv {
                            return Some(format!("vec[{}](tag {:#x})+{}: g={:02x} m={:02x}", e, tag, j, gv, mv));
                        }
                    }
                }
            }
        }
        None
    }) });
    // ★내 사본이 할당한 것을 해제한다(안 하면 호출당 누수). rlib 은 내 DLL 안에 링크돼 있어 알로케이터가 같다.
    { let c = core::ptr::read_unaligned((a0 as usize + 0xc0) as *const usize);
      let pz = core::ptr::read_unaligned((a0 as usize + 0xc8) as *const usize);
      let mine = VB49.with(|c2| (&*c2.get()).0.as_ptr() as usize);
      // ★내 버퍼면 해제하면 안 된다(thread_local 정적) — 재할당된 경우에만 해제.
      if c > 0 && pz > 0x1000 && pz != mine { if let Ok(l) = std::alloc::Layout::from_size_align(c * 24, 8) {
          std::alloc::dealloc(pz as *mut u8, l); } } }
    SP49.with(|c| { let sp = &*c.get();
        core::ptr::copy_nonoverlapping(sp.as_ptr(), a0 as *mut u8, 1064); }); // 게임 호출 후 상태로 복구
    core::ptr::copy_nonoverlapping(p2.as_ptr(), a2 as *mut u8, 320);                    // 게임 호출 후 상태로 복구(게임 진행은 게임 값으로)
    match m {
        Ok(_) => { if let Some(d) = sd {
            note(49, format!("#54 TeamPlan::update 대조#{} **상태갈림**: {} | (반환 void — 출력은 &mut self 다) | a0={:#x} a1={} a2={:#x} a3={:#x} a4={:#x} a5={:#x}", n, d, a0 as usize, a1, a2 as usize, a3 as usize, a4 as usize, a5 as usize)); } }
        Err(_) => { S[49].pan.fetch_add(1, Ordering::Relaxed); }
    }
    pop(49);
    g
}
unsafe fn w_39(a0: *const u8, a1: *const u8, a2: u8) -> bool {
    S[50].calls.fetch_add(1, Ordering::Relaxed);
    let f: unsafe fn(*const u8, *const u8, u8) -> bool = core::mem::transmute(S[50].orig.load(Ordering::Relaxed));
    let t = top(50);
    let g = f(a0, a1, a2);
    if !t { pop(50); return g; }
    let n = S[50].cmp.fetch_add(1, Ordering::Relaxed) + 1;
    let m = catch_unwind(AssertUnwindSafe(|| my_39(a0, a1, a2)));
    match m {
        Ok(m) => { if m != g { note(50, format!("#39 is_wave_priority_start_line 대조#{} 갈림: g={:?} m={:?} | a0={:#x} a1={:#x} a2={}", n, g, m, a0 as usize, a1 as usize, a2)); } }
        Err(_) => { S[50].pan.fetch_add(1, Ordering::Relaxed); }
    }
    pop(50);
    g
}
unsafe fn w_4(a0: i64, a1: *const u8, a2: *const u8, a3: *const u8, a4: u8, a5: *const u8) -> bool {
    S[51].calls.fetch_add(1, Ordering::Relaxed);
    let f: unsafe fn(i64, *const u8, *const u8, *const u8, u8, *const u8) -> bool = core::mem::transmute(S[51].orig.load(Ordering::Relaxed));
    let t = top(51);
    // ★StdRng(320B) — 게임 호출이 난수열을 소비하므로 **먼저 떠 둔다**(안 그러면 거짓 DIFF).
    let mut r1 = [0u8; 320]; if t { core::ptr::copy_nonoverlapping(a1, r1.as_mut_ptr(), 320); }
    let g = f(a0, a1, a2, a3, a4, a5);
    if !t { pop(51); return g; }
    let n = S[51].cmp.fetch_add(1, Ordering::Relaxed) + 1;
    let mut p1 = [0u8; 320]; core::ptr::copy_nonoverlapping(a1, p1.as_mut_ptr(), 320); // 게임 호출 후 상태
    core::ptr::copy_nonoverlapping(r1.as_ptr(), a1 as *mut u8, 320);                    // 내 사본 호출 전 = 호출 전 상태로
    let m = catch_unwind(AssertUnwindSafe(|| my_4(a0, a1, a2, a3, a4, a5)));
    core::ptr::copy_nonoverlapping(p1.as_ptr(), a1 as *mut u8, 320);                    // 게임 호출 후 상태로 복구(게임 진행은 게임 값으로)
    match m {
        Ok(m) => { if m != g { note(51, format!("#04 handle_line_defense 대조#{} 갈림: g={:?} m={:?} | a0={} a1={:#x} a2={:#x} a3={:#x} a4={} a5={:#x}", n, g, m, a0, a1 as usize, a2 as usize, a3 as usize, a4, a5 as usize)); } }
        Err(_) => { S[51].pan.fetch_add(1, Ordering::Relaxed); }
    }
    pop(51);
    g
}
unsafe fn w_22(a0: *const u8, a1: *const u8, a2: *const u8, a3: i64, a4: i64) -> bool {
    S[52].calls.fetch_add(1, Ordering::Relaxed);
    let f: unsafe fn(*const u8, *const u8, *const u8, i64, i64) -> bool = core::mem::transmute(S[52].orig.load(Ordering::Relaxed));
    let t = top(52);
    let g = f(a0, a1, a2, a3, a4);
    if !t { pop(52); return g; }
    let n = S[52].cmp.fetch_add(1, Ordering::Relaxed) + 1;
    let m = catch_unwind(AssertUnwindSafe(|| my_22(a0, a1, a2, a3, a4)));
    match m {
        Ok(m) => { if m != g { note(52, format!("#22 can_tower_focused 대조#{} 갈림: g={:?} m={:?} | a0={:#x} a1={:#x} a2={:#x} a3={} a4={}", n, g, m, a0 as usize, a1 as usize, a2 as usize, a3, a4)); } }
        Err(_) => { S[52].pan.fetch_add(1, Ordering::Relaxed); }
    }
    pop(52);
    g
}

/// 이 명세 idx 가 sweep 으로 **설치돼 있나**(= 진입부 프로브를 걸면 안 되나).
pub fn is_installed_spec(idx: u8) -> bool {
    S.iter().any(|s| s.idx == idx && s.orig.load(Ordering::Relaxed) != 0)
}
pub fn any_installed() -> bool { S.iter().any(|s| s.orig.load(Ordering::Relaxed) != 0) }

/// sweep 설치. `mask` 비트 k = `S[k]`. 반환 = (성공, 시도).
/// ⚠`orig` 는 **진입부 패치 전에** 저장된다(`hookw` 가 그 순서를 보장) — 패치 직후 다른 스레드가
///   들어와 `orig==0` 을 transmute 하면 널 호출이다(배경 sim 워커가 있으니 실재하는 경합).
pub unsafe fn install(mask: u64, log: &mut String) -> (usize, usize) {
    if mask == 0 {
        log.push_str("[sweep] 게이트 OFF (sweep20_on.txt 없음/0) — 한 곳도 안 걸었다\n");
        return (0, 0);
    }
    let unknown = mask & !((1u64 << S.len()) - 1);
    if unknown != 0 {
        // 「빠진 것을 모르는 상태」를 만들지 않는다 — 슬롯이 없는 비트를 켜면 조용히 무시되는 게 아니라 말한다.
        log.push_str(&format!("[sweep] ⚠mask 의 미지 비트 {:#x} 는 슬롯이 없어 무시했다(슬롯 {}개)\n", unknown, S.len()));
    }
    let w: [usize; 53] = [w_58 as usize, w_59 as usize, w_60 as usize, w_61 as usize, w_62 as usize, w_30 as usize, w_32 as usize, w_5 as usize, w_11 as usize, w_3 as usize, w_49 as usize, w_0 as usize, w_6 as usize, w_13 as usize, w_14 as usize, w_9 as usize, w_33 as usize, w_37 as usize, w_36 as usize, w_35 as usize, w_1 as usize, w_43 as usize, w_28 as usize, w_27 as usize, w_26 as usize, w_50 as usize, w_24 as usize, w_8 as usize, w_18 as usize, w_41 as usize, w_12 as usize, w_52 as usize, w_48 as usize, w_19 as usize, w_34 as usize, w_29 as usize, w_31 as usize, w_20 as usize, w_25 as usize, w_56 as usize, w_51 as usize, w_16 as usize, w_45 as usize, w_53 as usize, w_23 as usize, w_46 as usize, w_21 as usize, w_55 as usize, w_38 as usize, w_54 as usize, w_39 as usize, w_4 as usize, w_22 as usize];
    let (mut ok, mut tried) = (0usize, 0usize);
    for i in 0..S.len() {
        if mask & (1u64 << i) == 0 { continue; }
        tried += 1;
        let r = if S[i].sites.is_empty() {
            crate::hookw::install_wrap(S[i].rva, S[i].prolog, w[i], &S[i].orig)
        } else {
            crate::probe::install_cs_wrap(S[i].rva, S[i].sites, w[i], &S[i].orig)
        };
        match r {
            Ok(o) => { ok += 1; log.push_str(&format!("[sweep] bit{} #{:02} {} OK @{:#x} {} {:#x}{}\n", i, S[i].idx, S[i].name, S[i].rva,
                if S[i].sites.is_empty() { "트램폴린" } else { "호출부스텁" }, o,
                if S[i].sites.is_empty() { String::new() } else { format!(" (사이트 {}곳 — 원 함수 명령 무손상)", S[i].sites.len()) })); }
            Err(e) => log.push_str(&format!("[sweep] bit{} #{:02} {} 실패: {} @{:#x}\n", i, S[i].idx, S[i].name, e, S[i].rva)),
        }
    }
    (ok, tried)
}

/// 「판 종료」 확정 직후 호출 — 다음 판의 델타 기준선.
pub fn mark_base() {
    for s in S.iter() {
        s.base_cmp.store(s.cmp.load(Ordering::Relaxed), Ordering::Relaxed);
        s.base_diff.store(s.diff.load(Ordering::Relaxed), Ordering::Relaxed);
    }
}
pub fn total_cmp() -> u64 { S.iter().map(|s| s.cmp.load(Ordering::Relaxed)).sum() }

pub fn report(header: &str, gate: &str, inst: &str) -> String {
    let mut s = String::new();
    s.push_str("=== tfm2_judge_verify 2단계 · sweep (게임 원본 vs 내 링크사본 비트동일 대조) ===\n");
    s.push_str(header); s.push_str("\n\n");
    s.push_str(&format!("--- 게이트: {}\n--- 설치: {}\n\n", gate, inst));
    s.push_str("--- 대조표 (DIFF=0 은 **표본 수와 함께** 읽어라 — 대조수 0 이면 판정 불성립)\n");
    s.push_str("    bit  idx  종류      호출수         대조수        DIFF     panic     skip   이번판델타(대조/DIFF)  name\n");
    for x in S.iter() {
        let kind = if x.orig.load(Ordering::Relaxed) != 0 { "[sweep]" } else { "[미설치]" };
        let (c, n, dd, pa, sk) = (x.calls.load(Ordering::Relaxed), x.cmp.load(Ordering::Relaxed),
                              x.diff.load(Ordering::Relaxed), x.pan.load(Ordering::Relaxed), x.skip.load(Ordering::Relaxed));
        s.push_str(&format!("   {:>3} {:>4}  {:<9} {:>12} {:>12} {:>10} {:>8} {:>8}   {:>10}/{:<8} {}\n",
            x.bit, x.idx, kind, c, n, dd, pa, sk,
            n.saturating_sub(x.base_cmp.load(Ordering::Relaxed)),
            dd.saturating_sub(x.base_diff.load(Ordering::Relaxed)), x.name));
    }
    s.push_str("    (호출수 = 진입 전부 · 대조수 = 최상위 진입만 = 실제 표본 수 · panic = 내 사본이 패닉한 횟수 · skip = 버퍼/변종 사유로 **표본에서 뺀** 횟수)\n");
    for x in S.iter() { if !x.caveat.is_empty() { s.push_str(&format!("    ⚠#{:02} {} caveat: {}\n", x.idx, x.name, x.caveat)); } }
    s.push_str("    1단계 발화수 대조: ");
    for x in S.iter() { if x.stage1 == u64::MAX { s.push_str(&format!("#{:02} 무효(옛주소) ", x.idx)); } else { s.push_str(&format!("#{:02} {} ", x.idx, x.stage1)); } }
    s.push_str("\n    ⛔「무효(옛주소)」= 1단계를 **다른 함수**에서 쟀다 — 정정된 주소로 1단계를 다시 돌려야 값이 생긴다.");
    s.push_str("\n\n");
    let g = FIRST.lock().unwrap_or_else(|e| e.into_inner());
    s.push_str(&format!("--- 첫 DIFF 덤프 {}건 (슬롯당 1건 · 상한 {})\n", g.len(), FIRST_MAX));
    if g.is_empty() { s.push_str("    (없음)\n"); }
    for (_, l) in g.iter() { s.push_str("    "); s.push_str(l); s.push('\n'); }
    s.push_str("\n--- ENUM_LIVE variant 히스토그램 (페이로드 정밀 비교가 실제로 어느 variant 에 적용됐나 · 대조 1건 = 1)\n");
    { let tot: u64 = EH_11_0.iter().map(|a| a.load(Ordering::Relaxed)).sum();
      s.push_str(&format!("    #11 self+0x5e8 BigPlan: 총 {} — ", tot));
      for &(t, nm) in EH_11_0_NAMES.iter() { let c = EH_11_0[t as usize].load(Ordering::Relaxed); if c > 0 { s.push_str(&format!("{}({})={} ", nm, if t == 0 { "untagged".to_string() } else { t.to_string() }, c)); } }
      let seen: Vec<&str> = EH_11_0_NAMES.iter().filter(|(t, _)| EH_11_0[*t as usize].load(Ordering::Relaxed) == 0).map(|(_, n)| *n).collect();
      s.push_str(&format!("\n        미출현 {}: {}\n", seen.len(), seen.join(" "))); }
    { let tot: u64 = EH_11_1.iter().map(|a| a.load(Ordering::Relaxed)).sum();
      s.push_str(&format!("    #11 self+0x768 SubPlan: 총 {} — ", tot));
      for &(t, nm) in EH_11_1_NAMES.iter() { let c = EH_11_1[t as usize].load(Ordering::Relaxed); if c > 0 { s.push_str(&format!("{}({})={} ", nm, if t == 0 { "untagged".to_string() } else { t.to_string() }, c)); } }
      let seen: Vec<&str> = EH_11_1_NAMES.iter().filter(|(t, _)| EH_11_1[*t as usize].load(Ordering::Relaxed) == 0).map(|(_, n)| *n).collect();
      s.push_str(&format!("\n        미출현 {}: {}\n", seen.len(), seen.join(" "))); }
    { let tot: u64 = EH_12_0.iter().map(|a| a.load(Ordering::Relaxed)).sum();
      s.push_str(&format!("    #12 self+0x5e8 BigPlan: 총 {} — ", tot));
      for &(t, nm) in EH_12_0_NAMES.iter() { let c = EH_12_0[t as usize].load(Ordering::Relaxed); if c > 0 { s.push_str(&format!("{}({})={} ", nm, if t == 0 { "untagged".to_string() } else { t.to_string() }, c)); } }
      let seen: Vec<&str> = EH_12_0_NAMES.iter().filter(|(t, _)| EH_12_0[*t as usize].load(Ordering::Relaxed) == 0).map(|(_, n)| *n).collect();
      s.push_str(&format!("\n        미출현 {}: {}\n", seen.len(), seen.join(" "))); }
    { let tot: u64 = EH_2_0.iter().map(|a| a.load(Ordering::Relaxed)).sum();
      s.push_str(&format!("    #02 self+0x0 SubPlan: 총 {} — ", tot));
      for &(t, nm) in EH_2_0_NAMES.iter() { let c = EH_2_0[t as usize].load(Ordering::Relaxed); if c > 0 { s.push_str(&format!("{}({})={} ", nm, if t == 0 { "untagged".to_string() } else { t.to_string() }, c)); } }
      let seen: Vec<&str> = EH_2_0_NAMES.iter().filter(|(t, _)| EH_2_0[*t as usize].load(Ordering::Relaxed) == 0).map(|(_, n)| *n).collect();
      s.push_str(&format!("\n        미출현 {}: {}\n", seen.len(), seen.join(" "))); }
    { let tot: u64 = EH_56_0.iter().map(|a| a.load(Ordering::Relaxed)).sum();
      s.push_str(&format!("    #56 self+0x0 SubPlan: 총 {} — ", tot));
      for &(t, nm) in EH_56_0_NAMES.iter() { let c = EH_56_0[t as usize].load(Ordering::Relaxed); if c > 0 { s.push_str(&format!("{}({})={} ", nm, if t == 0 { "untagged".to_string() } else { t.to_string() }, c)); } }
      let seen: Vec<&str> = EH_56_0_NAMES.iter().filter(|(t, _)| EH_56_0[*t as usize].load(Ordering::Relaxed) == 0).map(|(_, n)| *n).collect();
      s.push_str(&format!("\n        미출현 {}: {}\n", seen.len(), seen.join(" "))); }
    s.push_str("\n--- ★대조에서 빠진 명세 함수와 사유 (1단계 프로브는 그대로 유지된다)\n");
    for (i, nm, c, why) in EXCLUDED.iter() {
        s.push_str(&format!("    #{:02} {:<34} 발화 {:>12}  ← {}\n", i, nm, c, why));
    }
    s.push_str("\n1단계 발화수는 probe20.txt(별도 파일)에 있다 — 섞어 읽지 말 것.\n");
    s
}
