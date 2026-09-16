# r19 — 실변경·미확인 17 디컴 대조 결과(0.5.8→0.6.0 · 09-16 밤 · RE 배치 A/B/C)

| 구 | 신 | 함수 | 판정 |
|---|---|---|---|
| `cc5fc0` | `ea8450` | RecallSubPlan::score | 동치 |
| `ccaa10` | `eacee0` | EpicPokeSubPlan::score | 동치 |
| `e81390` | `f64d30` | SerpenPokeSubPlan::score | 동치 |
| `ccbb10` | `f3a070` | LineSafeSubPlan::score | 동치 |
| `eb5840` | `eae650` | LineWaitSubPlan::score | 동치 |
| `cc3080` | `ea6f40` | BattleSubPlan::score | 동치(goal 4→3·vt +0x20) |
| `dc2960` | `ed0400` | SmallActionAroundBush::get_input | ★로직 변경: path_finder 이름≠around_bush 면 폐기·재생성 |
| `e23fa0` | `e5e0e0` | SmallActionPlay::get_input | 동치(AroundHide arm 소멸) |
| `e23170` | `e5d4e0` | SmallActionPlay::evaluation_position | 동치(vt 0x60/0x68→0x80/0x88) |
| `e26c40` | `e95f70` | SmallActionLaneMinionPosition::get_input | 동치 |
| `dc2330` | `ecfdd0` | SmallActionAroundBush::update_state | 동치 |
| `ccacf0` | `f39240` | LineSafeSubPlan::action_candidates | 동치(vt 0xe8→0x108) |
| `d405d0` | `e78a10` | base_attacking_minion_uncached | 동치 |
| `e0dfc0` | `dea9b0` | v30_wave_danger_chase_guard | 동치(BattleSubPlanGoal 재번호) |
| `d99f60` | `100b2b0` | v30_line_champion_action_tower_aggro_risk | 동치 |
| `e28410` | `e73610` | line_action_economy_adjustment | 동치 |
| `e4a780` | `d55830` | v3_assign_anchor | 동치(LPH 레이아웃) |

동치 16 · 로직 변경 1 ⟹ 0.6.0 명세 유효 = 119 + 16 = **135** · 재명세 = 변경 99 + 1(AroundBush::get_input) = **100**.

공통 갱신 재료: PlayerState +0xd0(0x928/0x930/0x9c0 → 0x9f8/0xa00/0xa90) · SmallActionPlay 태그표(AroundHide 제거) · BattleSubPlanGoal {Trace 0 Kiting 1 KitingBack 2 RunAway 3 End 4} · Effect vt 신규 4(0x30·0x40·0x48·0x70 · 구 0x38~0x50 +0x18 · 구 ≥0x58 +0x20) · Blackboard 0x2e8→0x5c8 · LegacyPlanHandler 레이아웃표(RE 배치 C §5).