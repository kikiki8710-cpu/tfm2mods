# dispcheck060 — 「오프셋만」 변위 짝의 §A 역참조 검사(09-17)

대상 = mig060_same 147 · 변위 짝 총 658 · 미설명 88(함수 22)

## 미설명 짝 빈도(구→신 · 함수 수)

| 구 | 신 | Δ | 건수 | 등장 함수 |
|---|---|---|---|---|
| `0x490` | `0x500` | +0x70 | 3 | ccbb10(r13 cbda4) e8b200(r9 8b4ad) eb5840(r13 b5ad4) |
| `0x50` | `0x70` | +0x20 | 3 | d62bb0(rax 62c98) d62bb0(rax 62de6) de6ce0(rax e6da0) |
| `0x4c0` | `0x4f8` | +0x38 | 2 | ccf670(r9 cf9e9) ec7c90(r9 c8009) |
| `0x40` | `0x60` | +0x20 | 2 | d51fb0(rcx 51fb9) dfb220(rcx fb229) |
| `0x230` | `0x260` | +0x30 | 2 | d815e0(r8 81648) d815e0(rdx 81e64) |
| `0x280` | `0x2b0` | +0x30 | 2 | d815e0(r8 81661) d815e0(rdx 81e91) |
| `0x258` | `0x288` | +0x30 | 2 | d815e0(r8 8167c) d815e0(r8 81e03) |
| `0x2d0` | `0x300` | +0x30 | 2 | d815e0(r8 816af) d815e0(r8 81e1e) |
| `0x18` | `0x50` | +0x38 | 2 | dc2960(r12 c29f4) dc2960(r12 c2bbd) |
| `0x30` | `0x38` | +0x8 | 2 | ebc140(rcx bc1ed) ebc140(rcx bc26f) |
| `-0xf` | `-0xe` | +0x1 | 1 | cc5fc0(rdx c6091) |
| `0x4d8` | `0x508` | +0x30 | 1 | d39ba0(r11 39d71) |
| `0x528` | `0x558` | +0x30 | 1 | d39ba0(r11 39d79) |
| `0x4b0` | `0x4e0` | +0x30 | 1 | d39ba0(r11 39d81) |
| `0x550` | `0x580` | +0x30 | 1 | d39ba0(r11 39d89) |
| `0x4b8` | `0x4e8` | +0x30 | 1 | d39ba0(r11 39d91) |
| `0x4e0` | `0x510` | +0x30 | 1 | d39ba0(r11 39d99) |
| `0x558` | `0x588` | +0x30 | 1 | d39ba0(r11 39da1) |
| `0x4c0` | `0x4f0` | +0x30 | 1 | d39ba0(r11 39da9) |
| `0x4e8` | `0x518` | +0x30 | 1 | d39ba0(r11 39db1) |
| `0x560` | `0x590` | +0x30 | 1 | d39ba0(r11 39db9) |
| `0x4c8` | `0x4f8` | +0x30 | 1 | d39ba0(r11 39dc1) |
| `0x4f0` | `0x520` | +0x30 | 1 | d39ba0(r11 39dc9) |
| `0x568` | `0x598` | +0x30 | 1 | d39ba0(r11 39dd1) |
| `0x4d0` | `0x500` | +0x30 | 1 | d39ba0(r11 39dd9) |
| `0x4f8` | `0x528` | +0x30 | 1 | d39ba0(r11 39de1) |
| `0x570` | `0x5a0` | +0x30 | 1 | d39ba0(r11 39de9) |
| `0x500` | `0x530` | +0x30 | 1 | d39ba0(r11 39df1) |
| `0x578` | `0x5a8` | +0x30 | 1 | d39ba0(r11 39df9) |
| `0x508` | `0x538` | +0x30 | 1 | d39ba0(r11 39e01) |
| `0x530` | `0x560` | +0x30 | 1 | d39ba0(r11 39e09) |
| `0x88` | `0x68` | -0x20 | 1 | d405d0(r8 40733) |
| `0x660` | `0x668` | +0x8 | 1 | d405d0(r8 4080a) |
| `0x378` | `0x668` | +0x2f0 | 1 | d408e0(rdx 4096a) |
| `0x80` | `0x1c8` | +0x148 | 1 | d51fb0(rsi 51fd5) |
| `0x8d` | `0x208` | +0x17b | 1 | d51fb0(rsi 51ff6) |
| `0x88` | `0x1f8` | +0x170 | 1 | d51fb0(rsi 5200e) |
| `0x118` | `0x320` | +0x208 | 1 | d61330(r12 622cb) |
| `0x120` | `0x328` | +0x208 | 1 | d61330(r12 622d6) |
| `0x138` | `0x340` | +0x208 | 1 | d61330(r12 62328) |
| `0x140` | `0x348` | +0x208 | 1 | d61330(r12 62333) |
| `0x158` | `0x360` | +0x208 | 1 | d61330(r12 62385) |
| `0x160` | `0x368` | +0x208 | 1 | d61330(r12 62390) |
| `0x178` | `0x380` | +0x208 | 1 | d61330(r12 623e2) |
| `0x2a8` | `0x2d8` | +0x30 | 1 | d815e0(r8 81697) |
| `0x2f8` | `0x328` | +0x30 | 1 | d815e0(r8 816c7) |
| `0x1410` | `0x3118` | +0x1d08 | 1 | d84db0(rax 84f70) |
| `0x1428` | `0x3130` | +0x1d08 | 1 | d84db0(rax 84f77) |
| `0x50` | `0x58` | +0x8 | 1 | dc2960(r12 c2b33) |
| `0x20` | `0x18` | -0x8 | 1 | dc2960(r12 c2bc2) |
| `0x149` | `0x3c1` | +0x278 | 1 | dd50e0(rdx d50ef) |
| `0x140` | `0x3b8` | +0x278 | 1 | dd50e0(rdx d5107) |
| `0x148` | `0x3c0` | +0x278 | 1 | dd50e0(rdx d510e) |
| `0x130` | `0x3a8` | +0x278 | 1 | dd50e0(rdx d5115) |
| `0x14a` | `0x3c2` | +0x278 | 1 | dd50e0(rdx d5121) |
| `0x14e` | `0x3c6` | +0x278 | 1 | dd50e0(rdx d512b) |
| `0x238` | `0x528` | +0x2f0 | 1 | de6ce0(rdi e71d9) |
| `0x240` | `0x530` | +0x2f0 | 1 | de6ce0(rdi e7311) |
| `0x248` | `0x538` | +0x2f0 | 1 | de6ce0(rdi e7318) |
| `0x2d8` | `0x5c8` | +0x2f0 | 1 | de6ce0(rcx e735a) |
| `0x250` | `0x540` | +0x2f0 | 1 | de6ce0(rdi e745b) |
| `0x258` | `0x548` | +0x2f0 | 1 | de6ce0(rdi e7462) |
| `0x2e0` | `0x5d0` | +0x2f0 | 1 | de6ce0(rcx e74a4) |
| `0x260` | `0x550` | +0x2f0 | 1 | de6ce0(rdi e759e) |
| `0x268` | `0x558` | +0x2f0 | 1 | de6ce0(rdi e75a5) |
| `0x2e8` | `0x5d8` | +0x2f0 | 1 | de6ce0(rcx e75e7) |
| `0xc0` | `0x1c8` | +0x108 | 1 | dfb220(rsi fb245) |
| `0xff` | `0x208` | +0x109 | 1 | dfb220(rsi fb266) |
| `0xf6` | `0x1f8` | +0x102 | 1 | dfb220(rsi fb27e) |
| `0x500` | `0x538` | +0x38 | 1 | e23170(r15 2320a) |
| `0x1808` | `0x24b5` | +0xcad | 1 | e4a780(rdx 4a7bf) |
| `0x638` | `0xfc0` | +0x988 | 1 | e4a780(rdx 4a83d) |
| `0x650` | `0xfe3` | +0x993 | 1 | e4a780(rdx 4a845) |
| `0x706` | `0x107b` | +0x975 | 1 | e4a780(rdx 4a8a0) |
| `0x618` | `0x100b` | +0x9f3 | 1 | e4a780(rdx 4a8b2) |
| `0x1628` | `0x2260` | +0xc38 | 1 | e4b5d0(rsi 4b7b4) |

## 함수별

| 구 | 신 | 함수 | 판정(v060) | strict | 변위 짝 | 미설명 | 미설명 상세 |
|---|---|---|---|---|---|---|---|
| `d39ba0` | `de6280` | precompute_champion_powers | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0x30·0x20·0xf0) | 20 | 20 | r11:0x4d8→0x508 r11:0x528→0x558 r11:0x4b0→0x4e0 r11:0x550→0x580 r11:0x4b8→0x4e8 r11:0x4e0→0x510 |
| `de6ce0` | `fb34e0` | check_epic_setup | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0x2f0·0xd0·0x208· | 20 | 11 | rax:0x50→0x70 rdi:0x238→0x528 rdi:0x240→0x530 rdi:0x248→0x538 rcx:0x2d8→0x5c8 rdi:0x250→0x540 |
| `d815e0` | `feefc0` | EntityPositioningCache::new | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0x20·0x30·0xd0) | 20 | 10 | r8:0x230→0x260 r8:0x280→0x2b0 r8:0x258→0x288 r8:0x2a8→0x2d8 r8:0x2d0→0x300 r8:0x2f8→0x328 |
| `d61330` | `100d990` | check_serpen_hunt | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0x208·0x2f0·0xd0· | 20 | 7 | r12:0x118→0x320 r12:0x120→0x328 r12:0x138→0x340 r12:0x140→0x348 r12:0x158→0x360 r12:0x160→0x368 |
| `dd50e0` | `f0c310` | v27_active_objective_discipline | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0x278) | 6 | 6 | rdx:0x149→0x3c1 rdx:0x140→0x3b8 rdx:0x148→0x3c0 rdx:0x130→0x3a8 rdx:0x14a→0x3c2 rdx:0x14e→0x3c6 |
| `e4a780` | `d55830` | v3_assign_anchor | 동치(r19: 동치(LPH 레이아웃)) | ⚠소형 즉치 변경(4→7) | 6 | 5 | rdx:0x1808→0x24b5 rdx:0x638→0xfc0 rdx:0x650→0xfe3 rdx:0x706→0x107b rdx:0x618→0x100b |
| `d51fb0` | `dd87b0` | SinglePlanBattle::with_runaway | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0x20·0x148·0x17b· | 4 | 4 | rcx:0x40→0x60 rsi:0x80→0x1c8 rsi:0x8d→0x208 rsi:0x88→0x1f8 |
| `dc2960` | `ed0400` | SmallActionAroundBush::get_input | 변경(r19: ★로직 변경: path_fin | ⚠정렬 불가(미확정) · 블록이동 24 ·  | 6 | 4 | r12:0x18→0x50 r12:0x50→0x58 r12:0x18→0x50 r12:0x20→0x18 |
| `dfb220` | `dd87b0` | BattlePlan::with_runaway | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0x20·0x108·0x109· | 4 | 4 | rcx:0x40→0x60 rsi:0xc0→0x1c8 rsi:0xff→0x208 rsi:0xf6→0x1f8 |
| `d405d0` | `e78a10` | base_attacking_minion_uncached | 동치(r19: 동치) | ⚠정렬 불가(미확정) · 블록이동 27 ·  | 4 | 2 | r8:0x88→0x68 r8:0x660→0x668 |
| `d62bb0` | `100f210` | check_serpen_setup | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0x20·0xd0·0x2e0·0 | 9 | 2 | rax:0x50→0x70 rax:0x50→0x70 |
| `d84db0` | `ff5ec0` | position_eval_at | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0x1d08·0xd0) · 콜리 | 3 | 2 | rax:0x1410→0x3118 rax:0x1428→0x3130 |
| `ebc140` | `ede720` | attack_structure_skill_action | 동치(mig060_same 확정: 동일) | ⚠소형 즉치 변경(10→f·11→10) ·  | 6 | 2 | rcx:0x30→0x38 rcx:0x30→0x38 |
| `cc5fc0` | `ea8450` | RecallSubPlan::score | 동치(r19: 동치) | ⚠정렬 불가(미확정) · 블록이동 9 · 콜 | 5 | 1 | rdx:-0xf→-0xe |
| `ccbb10` | `f3a070` | LineSafeSubPlan::score | 동치(r19: 동치) | ⚠정렬 불가(미확정) · 블록이동 16 ·  | 10 | 1 | r13:0x490→0x500 |
| `ccf670` | `eb4320` | serpen_action_score | 동치(mig060_same 확정: 오프셋 이 | ⚠소형 즉치 변경(7→6·f4→f5·f4→f | 4 | 1 | r9:0x4c0→0x4f8 |
| `d408e0` | `faf060` | is_cleared | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0xd0·0x2f0) | 3 | 1 | rdx:0x378→0x668 |
| `e23170` | `e5d4e0` | SmallActionPlay::evaluation_position | 동치(r19: 동치(vt 0x60/0x68→ | ⚠정렬 불가(미확정) · 블록이동 28 | 7 | 1 | r15:0x500→0x538 |
| `e4b5d0` | `d582a0` | v3_fall_back_to_passive | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0x150·0xa8·0xc38· | 5 | 1 | rsi:0x1628→0x2260 |
| `e8b200` | `f352d0` | LineDefenseSubPlan::score | 동치(mig060_same 확정: 이동만) | ⚠소형 즉치 변경(7→6) · 블록이동 5  | 10 | 1 | r9:0x490→0x500 |
| `eb5840` | `eae650` | LineWaitSubPlan::score | 동치(r19: 동치) | ⚠정렬 불가(미확정) · 블록이동 16 ·  | 10 | 1 | r13:0x490→0x500 |
| `ec7c90` | `f860e0` | epic_action_score | 동치(mig060_same 확정: 오프셋 이 | ⚠소형 즉치 변경(7→6·f4→f5·f4→f | 4 | 1 | r9:0x4c0→0x4f8 |
| `caff00` | `fe16c0` | SmallActionTrace::expected_goal_position | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0x20·0xd0) | 5 | 0 |  |
| `cbbca0` | `e601b0` | StealSubPlan::score | 동치(mig060_same 확정: 이동만) | ⚠소형 즉치 변경(7→6) | 0 | 0 |  |
| `cc3080` | `ea6f40` | BattleSubPlan::score | 동치(r19: 동치(goal 4→3·vt + | ⚠소형 즉치 변경(7→6·4→3·4→3·4→ | 11 | 0 |  |
| `cc5ca0` | `ea8130` | RecallSubPlan::action_candidates | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0xd0) · 콜리 주의 2 | 2 | 0 |  |
| `ccaa10` | `eacee0` | EpicPokeSubPlan::score | 동치(r19: 동치) | ⚠정렬 불가(미확정) · 블록이동 5 · 콜 | 2 | 0 |  |
| `ccacf0` | `f39240` | LineSafeSubPlan::action_candidates | 동치(r19: 동치(vt 0xe8→0x108 | ⚠정렬 불가(미확정) · 블록이동 45 ·  | 3 | 0 |  |
| `cce0c0` | `fd9770` | trace::SmallActionTrace::is_end | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0xd0) | 2 | 0 |  |
| `ccfbc0` | `eb4880` | calculate_serpen_action_score | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0xd0·0x208·0x2e0) | 13 | 0 |  |
| `cd0480` | `eb5140` | v48_on_cast_line | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0xd0) · 콜리 주의 1 | 1 | 0 |  |
| `cd5ee0` | `ebab70` | kite_reposition_point | 동치(mig060_same 확정: 동일) | ✅동일 · 콜리 주의 1 | 0 | 0 |  |
| `d27d50` | `ec62f0` | PassiveLinePlan::v46_stage2 | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0xd0·0xf0·0x30) · | 3 | 0 |  |
| `d31f20` | `dde530` | get_input_target | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0x20·0xd0) · 콜리 주 | 6 | 0 |  |
| `d34470` | `de0a80` | convert_to_move_action_target | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0x20) | 1 | 0 |  |
| `d34750` | `de0d60` | safe_move_avoiding_enemy_well | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0xd0) · 콜리 주의 1 | 1 | 0 |  |
| `d354c0` | `de1ad0` | ult | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0x20·0xd0) | 5 | 0 |  |
| `d35930` | `de1f40` | abstract_input::skill | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0xd0·0x20) | 3 | 0 |  |
| `d35d10` | `de2320` | abstract_input::attack | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0xd0·0x20) · 콜리 주 | 3 | 0 |  |
| `d360a0` | `de26b0` | abstract_input::skill2 | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0xd0·0x20) | 3 | 0 |  |
| `d36480` | `de2a90` | can_recall | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0xd0) · 스택슬롯 1 | 1 | 0 |  |
| `d377a0` | `de3e80` | build_minion_wave_snapshot | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0xd0) · 콜리 주의 3 | 2 | 0 |  |
| `d390a0` | `de5780` | champion_hp_value_uncached | 동치(mig060_same 확정: 동일) | ✅동일 | 0 | 0 |  |
| `d399a0` | `de6080` | nontarget_windup_perceived | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0xd0·0x2e0·0x208) | 4 | 0 |  |
| `d3a3a0` | `de6a80` | line_backfight_support_focus | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0xd0·0x2e0·0x208) | 2 | 0 |  |
| `d3ab40` | `de7220` | utils::can1v1win | 동치(mig060_same 확정: 동일) | ✅동일 · 콜리 주의 1 | 0 | 0 |  |
| `d3c700` | `e74b40` | need_defense_nexus | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0xd0·0x2e0·0x208) | 3 | 0 |  |
| `d3cfa0` | `e753e0` | handle_line_defense | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0xd0·0x70·0x2e0·0 | 5 | 0 |  |
| `d3dcc0` | `e76100` | objective_defense_role | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0xd0) · 콜리 주의 2 | 2 | 0 |  |
| `d3e4b0` | `e768f0` | has_line_defense_threat | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0x2e0·0xd0) | 1 | 0 |  |
| `d3e660` | `e76aa0` | nexus_last_stand_uncached | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0x20·0xd0) | 14 | 0 |  |
| `d3fa80` | `e77ec0` | nexus_under_direct_attack | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0xd0·0x2e0·0x208) | 3 | 0 |  |
| `d3fe50` | `e78290` | nexus_final_stand_uncached | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0x20·0xd0) | 5 | 0 |  |
| `d5ba80` | `f776b0` | calculate_jungle_action_score | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0xd0) · 콜리 주의 1 | 2 | 0 |  |
| `d665e0` | `1012e50` | serpen_giveup_chat_reason | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0xd0) | 1 | 0 |  |
| `d67060` | `fe3970` | v21_defensive_cc_score | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0x20·0x2e0·0xd0)  | 5 | 0 |  |
| `d68090` | `fe4950` | v16_knight_ult_zone_bonus | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0xd0·0x208·0x2e0) | 20 | 0 |  |
| `d69120` | `fe59e0` | v15_can_keep_support_pressure | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0xd0·0x2e0) | 2 | 0 |  |
| `d845e0` | `ddd010` | SmallActionSkill::get_input | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0xd0) | 2 | 0 |  |
| `d84800` | `ddd230` | SmallActionSkill2::get_input | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0xd0) | 2 | 0 |  |
| `d84a30` | `ddd460` | SmallActionUlt::get_input | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0xd0) | 2 | 0 |  |
| `d8ca70` | `ffdda0` | position_risk_all_zero_near | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0x20·0xd0·0xf0·0x | 13 | 0 |  |
| `d97300` | `1008640` | can_tower_focused | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0xd0) · 콜리 주의 1 | 2 | 0 |  |
| `d97700` | `1008a40` | v3_deadly_edge_cells | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0xd0) | 2 | 0 |  |
| `d98210` | `1009560` | can_trace_without_tower | 동치(mig060_same 확정: 동일) | ✅동일 | 0 | 0 |  |
| `d98420` | `1009770` | v3_lethal_tower_position | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0xd0) · 콜리 주의 1 | 2 | 0 |  |
| `d98740` | `1009a90` | can_tower_focused_when_attack | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0xd0) · 콜리 주의 1 | 2 | 0 |  |
| `d988d0` | `1009c20` | can_tower_focused_when_battle | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0xd0) | 2 | 0 |  |
| `d98ac0` | `1009e10` | v47_tower_focus_position_dangerous | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0xd0) · 콜리 주의 2 | 2 | 0 |  |
| `d99030` | `100a380` | v22_lane_tower_pressure_attack_allowed | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0xd0) · 콜리 주의 3 | 2 | 0 |  |
| `d99f60` | `100b2b0` | v30_line_champion_action_tower_aggro_ris | 동치(r19: 동치) | ⚠소형 즉치 변경(7→6·c→b·-c→-b) | 2 | 0 |  |
| `d9aa80` | `e85d70` | bush_distance_sq | 동치(mig060_same 확정: 동일) | ✅동일 · 콜리 주의 1 | 0 | 0 |  |
| `d9bce0` | `e87300` | steal::evaluate_steal_for_target | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0x20) · 콜리 주의 1 | 4 | 0 |  |
| `d9ddc0` | `de7980` | is_safe_recall | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0x20·0xd0·0x2e0·0 | 9 | 0 |  |
| `d9f3c0` | `de8f80` | check_nontarget | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0xd0) · 스택슬롯 2 | 2 | 0 |  |
| `dbbd60` | `ed4350` | SmallActionAroundRegion::get_input | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0xd0) · 콜리 주의 1 | 3 | 0 |  |
| `dbd130` | `fdb0e0` | move_actions::SmallActionRecall::is_end | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0xd0) | 2 | 0 |  |
| `dbf680` | `eccaf0` | SmallActionPositioning::get_input | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0xd0) | 3 | 0 |  |
| `dc0800` | `ece1d0` | SmallActionAroundPosition::get_input | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0xd0) · 콜리 주의 1 | 3 | 0 |  |
| `dc2330` | `ecfdd0` | around::SmallActionAroundBush::update_st | 동치(r19: 동치) | ⚠정렬 불가(미확정) · 블록이동 2 · 콜 | 2 | 0 |  |
| `dc2550` | `ecfff0` | SmallActionAroundBush::new_with_target | 동치(mig060_same 확정: 동일) | ✅동일 · 콜리 주의 1 | 0 | 0 |  |
| `dc27a0` | `ed0240` | SmallActionAroundBush::new_with_out_line | 동치(mig060_same 확정: 동일) | ✅동일 · 콜리 주의 2 | 0 | 0 |  |
| `dc8550` | `ee0960` | around::check_cell | 동치(mig060_same 확정: 동일) | ✅동일 | 0 | 0 |  |
| `dcd930` | `ef3da0` | handle_press_epic | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0x228·0xd0·0x2e0· | 20 | 0 |  |
| `dce520` | `ef4ab0` | TeamPlan::handle_epic_line_change | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0x228·0xd0·0x70·0 | 20 | 0 |  |
| `dd5270` | `f0c4a0` | v27_objective_discipline_action | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0xd0·0x208·0x2e0) | 12 | 0 |  |
| `de03d0` | `f18bf0` | should_keep_object_for_contested_wave_pr | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0xd0) | 1 | 0 |  |
| `de3630` | `100bb90` | get_die_tick_player | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0x20) · 콜리 주의 1 | 1 | 0 |  |
| `de40c0` | `f1fce0` | check_press_tower_opportunity | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0xd0·0x208·0x2e0) | 10 | 0 |  |
| `de5300` | `f20f20` | camp_idx | 동치(mig060_same 확정: 동일) | ✅동일 | 0 | 0 |  |
| `dea800` | `fb78b0` | v3_epic_formation_role | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0xd0) | 3 | 0 |  |
| `df1c80` | `fa82a0` | target_bush_v30 | 동치(mig060_same 확정: 동일) | ✅동일 | 0 | 0 |  |
| `e01450` | `e82410` | v55_mark_value | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0xd0·0xf0·0x30·0x | 7 | 0 |  |
| `e019d0` | `e82990` | v55_seal_value | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0xd0·0xf0·0x30·0x | 3 | 0 |  |
| `e02020` | `e83000` | v55_banish_penalty | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0xd0·0x20·0xf0·0x | 6 | 0 |  |
| `e02540` | `e83520` | noncombat_steroid_value | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0xd0·0xf0·0x30) · | 2 | 0 |  |
| `e02bc0` | `e83ba0` | v54_aoe_ally_heal_value | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0x18·0x20) · 콜리 주 | 3 | 0 |  |
| `e04400` | `e85460` | v57_summon_command_score | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0xd0) · TypeId 4  | 1 | 0 |  |
| `e0bd60` | `eecc90` | v21_should_defer_support_target | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0xd0·0x2e0·0x208) | 3 | 0 |  |
| `e0bf70` | `eecea0` | is_unreasonable_tower_dive_enemy | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0xd0) · 스택슬롯 1 | 2 | 0 |  |
| `e0c310` | `eed240` | v22_visible_enemy_is_runaway_threat | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0xd0·0x2e0) · 스택슬 | 2 | 0 |  |
| `e0c560` | `eed490` | should_end_object_finish_kill_priority_b | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0x70·0xd0·0x2e0)  | 4 | 0 |  |
| `e0daa0` | `dea490` | max_range_nearly_can_use | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0x20) | 4 | 0 |  |
| `e0dfc0` | `dea9b0` | v30_wave_danger_chase_guard | 동치(r19: 동치(BattleSubPlan | ⚠정렬 불가(미확정) | 0 | 0 |  |
| `e23fa0` | `e5e0e0` | SmallActionPlay::get_input | 동치(r19: 동치(AroundHide ar | ⚠정렬 불가(미확정) · 블록이동 33 ·  | 2 | 0 |  |
| `e248f0` | `e936c0` | SmallActionLaneMinionPosition::choose_go | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0x20) | 1 | 0 |  |
| `e25450` | `e944e0` | SmallActionLaneMinionPosition::push_cand | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0xd0) | 1 | 0 |  |
| `e266e0` | `e95770` | SmallActionLaneMinionPosition::has_curre | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0xd0) · 콜리 주의 1 | 2 | 0 |  |
| `e269a0` | `e95a30` | SmallActionLaneMinionPosition::should_su | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0xd0·0x20) | 3 | 0 |  |
| `e26c40` | `e95f70` | SmallActionLaneMinionPosition::get_input | 동치(r19: 동치) | ⚠정렬 불가(미확정) · 스택슬롯 10 ·  | 2 | 0 |  |
| `e28060` | `e73260` | line_recall_pressure_penalty | 동치(mig060_same 확정: 동일) | ✅동일 | 0 | 0 |  |
| `e281f0` | `e733f0` | line_champion_trade_allowance | 동치(mig060_same 확정: 동일) | ⚠lea 데이터 차이(검토) · 콜리 주의  | 0 | 0 |  |
| `e28410` | `e73610` | line_action_economy_adjustment | 동치(r19: 동치) | ⚠정렬 불가(미확정) · 블록이동 1 · 콜 | 2 | 0 |  |
| `e288c0` | `e73ac0` | line_projected_punish_damage_at | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0xd0·0x2e0·0x208) | 3 | 0 |  |
| `e29520` | `e64920` | path_needs_tower_escape | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0xd0) | 2 | 0 |  |
| `e2ac00` | `eb3980` | line_minion_action_candidates | 동치(mig060_same 확정: 동일) | ⚠소형 즉치 변경(f→e·10→f·11→10 | 5 | 0 |  |
| `e35a40` | `dc0040` | TeamPlan::handle_nexus_attack | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0xd0·0x2e0) | 2 | 0 |  |
| `e38c90` | `dc1c10` | engage::can_battle_triggered_filtered | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0xd0·0x208·0x2e0· | 8 | 0 |  |
| `e4b8c0` | `d59280` | LegacyPlanHandler::take_misunderstood_re | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0x9d0) | 3 | 0 |  |
| `e595b0` | `d5b410` | handle_chat | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0xa00·0x970·0x8b6 | 10 | 0 |  |
| `e6d000` | `d75ad0` | calculate_nexus_defense_count | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0x1e0·0xd0) | 5 | 0 |  |
| `e7b0b0` | `f27910` | build_game_finish_check_state | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0xd0·0x2e0) | 2 | 0 |  |
| `e80070` | `f63a10` | SerpenPokeSubPlan::action_candidates_old | 동치(mig060_same 확정: 이동만) | ⚠소형 즉치 변경(7→6·7→6) · 콜리  | 3 | 0 |  |
| `e81390` | `f64d30` | SerpenPokeSubPlan::score | 동치(r19: 동치) | ⚠정렬 불가(미확정) · 블록이동 8 · 콜 | 2 | 0 |  |
| `e81680` | `f65030` | AttackNexusSubPlan::action_candidates | 동치(mig060_same 확정: 동일) | ⚠소형 즉치 변경(f→e·10→f·11→10 | 7 | 0 |  |
| `e8e160` | `f3b9e0` | AgentVerHamster::upgrade_item | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0x70) · 콜리 주의 2 | 1 | 0 |  |
| `e8f850` | `f3d140` | AgentVerHamster::count_nearby_enemies | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0xd0·0x208·0x2e0) | 10 | 0 |  |
| `e8fc80` | `f3d570` | AgentVerHamster::buy_item | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0x70) · 콜리 주의 2 | 1 | 0 |  |
| `eb5dd0` | `ed5fd0` | expected_dps | 동치(mig060_same 확정: 동일) | ✅동일 · 콜리 주의 1 | 0 | 0 |  |
| `eb6100` | `ed6300` | battle_action | 동치(mig060_same 확정: 동일) | ⚠소형 즉치 변경(f→e·10→f·10→f· | 14 | 0 |  |
| `eb77a0` | `ed79a0` | battle_ally_action | 동치(mig060_same 확정: 동일) | ⚠소형 즉치 변경(10→f·10→f·11→1 | 9 | 0 |  |
| `eb8b80` | `ed8550` | attack_summon_action | 동치(mig060_same 확정: 동일) | ⚠소형 즉치 변경(f→e·10→f·11→10 | 7 | 0 |  |
| `eb9570` | `ed8f40` | battle_check_with_list | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0xf0·0x30·0xd0) · | 1 | 0 |  |
| `eba320` | `eda1a0` | v48_projectile_profile | 동치(mig060_same 확정: 동일) | ⚠lea 데이터 차이(검토) | 0 | 0 |  |
| `ebbb80` | `ede160` | should_disengage_object_hunt | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0xd0) | 2 | 0 |  |
| `ebcbd0` | `edf1b0` | should_add_self_etc_buff_action | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0xd0·0x2e0·0x208) | 13 | 0 |  |
| `ebd570` | `edfb50` | check_favorable_engage_formation | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0xd0) | 2 | 0 |  |
| `ec81e0` | `f86640` | calculate_epic_action_score | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0xd0·0x208·0x2e0) | 20 | 0 |  |
| `ec8af0` | `f86f50` | objective_is_damaged | 동치(mig060_same 확정: 동일) | ✅동일 | 0 | 0 |  |
| `ec9190` | `f87c30` | v23_enemy_object_pressure | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0xd0) | 3 | 0 |  |
| `ec9400` | `f87ea0` | is_wave_priority_start_line | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0xd0) | 1 | 0 |  |
| `ec9840` | `f882e0` | v23_healthy_allies_near_point | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0xd0) | 1 | 0 |  |
| `eca200` | `f88b30` | v25_objective_splitter_can_stay | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0xd0) | 2 | 0 |  |
| `eca430` | `f88d60` | v25_objective_far_split_pressure | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0x2e0·0xd0·0x1e0) | 2 | 0 |  |
| `eca9a0` | `f892d0` | v23_objective_setup_pressure_line | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0x1e0·0x2e0·0xd0) | 4 | 0 |  |
| `ecacc0` | `f895f0` | v23_recent_visible_enemies_near_point | 동치(mig060_same 확정: 동일) | ✅오프셋만(Δ0xd0·0x208·0x2e0) | 11 | 0 |  |
| `ecb300` | `f89c30` | v23_should_break_objective_hunt_anchor | 동치(mig060_same 확정: 동일) | ✅동일 | 0 | 0 |  |