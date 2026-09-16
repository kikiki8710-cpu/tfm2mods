# r18 편성표 — 지도 밖 호출자 편입에서 드러난 미명세 판단 로직 18 (2026-09-16 · 원장 §3-D)

| # | RVA | 실명 | IR | 줄수 | exe | 계층 | internal | 비고 |
|---|---|---|---|---|---|---|---|---|
| 1 | `0xcd05f0` | `battle::base_battle_action` | m02.ll:66196 | 7901 | 21,149B | act | ✓ | rvaname 확정 · 21KB 거대 → rootcut 분책 |
| 2 | `0xdf1f50` | `LineGankCoverPlan::next_plan` | m10.ll:11810 | 1806 | 5,418B | plan |  | ghidra 확정(BigPlan::next_plan JT arm 9) |
| 3 | `0xeb6100` | `fight_check::battle_action` | m15.ll:23700 | 1913 | 5,068B | act |  | rvaname 확정 |
| 4 | `0xd68090` | `battle_common::v16_knight_ult_zone_bonus` | m05.ll:55831 | 1924 | 4,228B | act |  | rvaname 확정 · 2단 정적(판 8 미계측 → 프로브 필요) |
| 5 | `0xccacf0` | `LineSafeSubPlan::action_candidates` | m02.ll:47101 | 1324 | 3,155B | act |  | rvaname 확정 · 16번째 action_candidates |
| 6 | `0xe03360` | `buff_value::noncombat_steroid_window` | m10.ll:36654 | 1008 | 2,922B | act |  | ghidra 확정 · IR 945 ins ↔ 2922B · d75cb0(1166B·콜리 0)은 그 클로저(642 ins) 추정 |
| 7 | `0xe7b9f0` | `end_check` | m14.ll:8724 | 1494 | 2,859B | plan |  | rvaname 확정 |
| 8 | `0xe49a50` | `LegacyPlanHandler::update_on_dead` | m13.ll:10120 | 552 | 2,734B | plan |  | ghidra 확정 · IR 501 ins |
| 9 | `0xe0dfc0` | `battle::v30_wave_danger_chase_guard` | m10.ll:52377 | 561 | 2,242B | plan |  | 보조 · IR 504 ins ↔ 2242B · 지도 e119e0 라벨과 비교 필요 |
| 10 | `0xeba320` | `fight_check::v48_projectile_profile` | m15.ll:30617 | 426 | 1,478B | act |  | IR 389 ins ↔ 1478B · 콜리 일치 · c875a0(626B·콜리 0)은 with 클로저 z(195 ins) 추정 |
| 11 | `0xe0b030` | `fight_model::resolve_fight_stake_roster` | m10.ll:47870 | 532 | 1,445B | plan |  | 09-13 ghidra 확정 · IR 486 ins ↔ 1445B |
| 12 | `0xcd5ee0` | `battle::kite_reposition_point` | m02.ll:74099 | 429 | 1,193B | act | ✓ | IR 418 ins ↔ 1193B · 콜리 일치 · cadcd0(492B·콜리 0)은 fp 오매칭 |
| 13 | `0xe25030` | `SmallActionLaneMinionPosition::target_score` | m11.ll:43890 | 346 | 1,045B | act |  | ghidra 추정 강 · IR 318 ins ↔ 1045B |
| 14 | `0xca6700` | `get_small_action 점수 합성 클로저` | m01.ll:48954 | 374 | 1,007B | act |  | 클로저(IR 339 ins ↔ 1007B) — 심볼은 아래에서 크기로 고른다 · 후보 32개 중 선택 |
| 15 | `0xe25450` | `SmallActionLaneMinionPosition::choose_goal(후보 클로저)` | m11.ll:43376 | 512 | 952B | act | ✓ | ghidra 추정 · IR choose_goal 475 ins vs 952B → 클로저일 수 있음(편성 시 확인) |
| 16 | `0xe02bc0` | `buff_value::v54_aoe_ally_heal_value` | m10.ll:35859 | 228 | 871B | act |  | ghidra 확정 · IR 212 ins ↔ 871B |
| 17 | `0xdd9f30` | `TeamPlan::can_near_enemies` | m09.ll:25501 | 151 | 745B | plan |  | ghidra 추정 · IR 143 ins ↔ 745B · 콜리 c99fe0(=can_near_enemies_range · 지도 라벨 fmt 오류) |
| 18 | `0xeb5dd0` | `fight_check::expected_dps` | m15.ll:23520 | 178 | 742B | act |  | ghidra 확정 · IR 152 ins ↔ 742B |

합계 IR 줄 21659 · exe 59,253 B