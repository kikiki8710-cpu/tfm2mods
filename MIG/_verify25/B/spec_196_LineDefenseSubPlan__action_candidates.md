---

### `196` LineDefenseSubPlan::action_candidates — 라인 수비 서브플랜의 행동 후보 목록 생성 루트(vtable 진입). 구판 후보(action_candidates_old: 위험시 RunAway 조기귀환→기본 포지셔닝→아군타워 곁 AroundRunAway/RunAway→전투·라인미니언·소환수공격·적타워공격·구조물스킬)에 아군지원 전투 후보를 합치고, 뒤 배치에서 최근접 적 타워 커버 판정(L457)·retain 필터(L466)·판단 정확도 무작위 절삭(L736~762)·v46 이동 후보(L766)·v47 시즈 스탠스(L774)·retain/score/get_input 재평가(L788~871)를 거쳐 sret Vec<SmallActionPlay> 로 돌려준다.

| 항목 | 값 |
|---|---|
| id | `line_defense__LineDefense__action_candidates` |
| 심볼 | `_RNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy8sub_plan12line_defenseNtB2_18LineDefenseSubPlan17action_candidates` |
| 소스 | `game-ai\src\plan_legacy\sub_plan\line_defense.rs:440` |
| IR | `m14.ll` 21211~28586행 |
| 경로·가시성 | `game_ai::plan_legacy::sub_plan::LineDefenseSubPlan::action_candidates` · **pub** |
| 계층 | 기타 |
| exe | `e83390` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r16` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&mut game_ai::plan_legacy::sub_plan::LineDefenseSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay>
```

**TLS 메모 절(`sig.tls` · 정정 경로 `/specs[196]/sig/tls/<키>`)**

- {"name": "(직접 접점 없음)", "role": "전 함수 7,376줄 grep: LocalKey/with/thread_local 호출 0건 — 이 함수 본문은 TLS 를 직접 읽거나 쓰지 않는다", "key": "-", "layout": "-", "invalidation": "-", "call_conditions": "TLS 접점은 전부 콜리 내부: 배치 D 범위에서 순서대로 ①nontarget_windup_perceived(L890, 적 챔피언 루프 안 최대 5회, 21616) ②position_score_at_position(L903, 21742; 순수 sret — POS_EVAL_CACHE 는 position_eval_at 계열이며 이 함수는 position_eval_at 를 직접 부르지 않는다) ③base_positioning 안 lane_minion_position_action(L183, 22206) ④battle_action(L928, 22688) ⑤line_minion_action_candidates(L929, 22764) ⑥attack_summon_action(L932, 22792) ⑦v22_lane_tower_pressure_attack_allowed(L389, 23402) ⑧attack_structure_skill_action(L935, 23577) ⑨battle_ally_action(L448, 23702). v47_siege_stance(SIEGE_STANCE_CACHE) 는 **배치 E L774 에서 1회**(27112, 조건: L771 nearest_enemy_tower.id == …) · v48_cast_beams / champion_hp_value / interaction_score / position_eval_at 는 이 함수 IR 에 심볼 0건(콜리 내부 간접). 미러 재현 시 배치 D 구간의 콜리 호출 순서를 위 ①~⑨ 그대로 유지해야 한다"}

<details><summary>인자 9개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | bumpalo Vec<SmallActionPlay> 32B | ptr@0 · bump@+8 · cap@+0x10 · len@+0x18. 배치 D 는 sret 를 직접 쓰지 않는다 — L442 결과를 %129(old_actions) 에 memcpy 32B(m14.ll:23589 / 23642) → L447 %128(res) 로 복사(23699) 하고 그 위에 extend 한다. 최종 sret 채움은 배치 F(L878~879) 소관 \| (배치 E) ptr@0 · bump@+8 · cap@+0x10 · len@+0x18 · 원소 184B · 태그@+0xb1. ⚠내 범위는 sret 을 직접 채우지 않는다 — act_actions(%122)·move_actions(%110) 두 로컬 Vec 를 만들어 배치 F(769~)로 넘긴다 \| (배치 F) ptr@0 · bump@+8 · cap@+0x10 · len@+0x18. 배치 F 가 채우는 지점 6곳: L876 ret(%122) memcpy 32B(m14.ll:27310) / L833·L863 from_iter_in([RunAway]) (28392, 28286) / L865·L868·L871 from_iter_in([best_move_action.clone()]) (28270, 28296, 28327) — from_iter_in 3번째 인자 %1079 = data.context.pool(bump) | 4 |
| 1 | 1 | self | &mut LineDefenseSubPlan(3B: +0 style LineStyle · +1 line LineType · +2 minion_action_type MinionActionType) | IR 속성 = noalias dereferenceable(3), readonly 없음 ⇒ &mut. 단 **initializes 속성 없음** 이고 전 함수(7,376줄) grep 결과 `store … ptr %1` / `+1`/`+2` 쓰기 0건 — 이 함수는 self 를 읽기만 한다(+1 line: m14.ll:21875(L160<912) · 22445(L913) · 22758(L929) · 24848(L230<766 배치 E)) · L837 score 에 ptr 로 전달(27726, 배치 F). 쓰기 표면 = 없음(전 배치 공통 관측) \| (배치 E) IR 속성에 readonly 없음(=&mut 가능). 내 범위: 읽기 line(+1, 230행) · self 포인터를 score()(closure#7 742행)에 전달. 내 범위 안 self 쓰기 0건 \| (배치 F) 배치 F 범위에선 **쓰기 0**. 읽기도 직접은 없고 콜리에 &self 로 전달만 — L789 unsafe_v19_non_champion_walkup(정의 m14.ll:28602, %0 readonly) · L837 score(m14.ll:29906, %0 readonly) | 4 |
| 2 | 2 | version | usize | L440 에서 %131 alloca(8B) 에 spill(21365) 되고 뒤 배치가 재로드(L448 23701 · L774 27111). 배치 D 안에서 version 분기 없음 — 콜리 인자로만 전달(nontarget_windup_perceived · position_score_at_position · Around::new · lane_minion_position_action · battle_action · line_minion_action_candidates · v22_lane_tower_pressure_attack_allowed · battle_ally_action) \| (배치 E) %131 alloca 로 spill 되어 클로저에 &version 으로 캡처. 내 범위 안에서 version 자체 분기 없음(콜리 인자로만 전달) \| (배치 F) 스택 슬롯 %131 로 재로드. 배치 F 에선 분기 없음 — L774 v47_siege_stance · L789 · L809 battle_action · L810 v30 · L824 range_misjudge_rng · L837 score · L853 get_input · L860 position_score_at_position 인자로만 | 4 |
| 3 | 3 | rnd | &mut StdRng(320B, ChaCha12) | 배치 D 에선 Around::new / lane_minion_position_action / battle_action / battle_ally_action 에 전달만. 직접 소비(choose 등)는 L759 배치 E \| (배치 E) gen_range(757·743) · SliceRandom::choose(759) · range_misjudge_roll(271/272/273/278/313) · check_kill_die_tick · lane_minion_position_action · score() 가 소비 → 상태 전진(부작용) · ⚠Around::new 의 rnd 인자는 `_rnd`(미사용, DILocalVariable) — rnd 소비 없음 \| (배치 F) L788(s8_0 경유 unsafe_v19) · L809 battle_action · L825 range_misjudge_roll(m14.ll:27523, 첫 인자) · L837 score · L853 get_input 에 &mut 전달 — 부작용은 콜리 소유 | 4 |
| 4 | 4 | player | &PlayerState(2528B) | +0x930 info.team(21511) · +0x9c0 info.position@tag(as_index, 21541) 직접 읽음. 클로저 캡처(base_positioning/action_candidates_old 최근접 키의 get_start_position team 인자) \| (배치 E) 읽기 +0x930 team · +0x180 parameter(AthleteParameter: judge_line_accuracy · positioning_accuracy) \| (배치 F) +0x930 info.team(%160, 배치 D 산출 재사용) · +0x9c0 info.position(%175) · +0x180 info.parameter(%1174, 배치 E 산출 · L817 positioning_accuracy) | 4 |
| 5 | 5 | data | &OperationData(24B) | +0 cache(&AbstractGameWithCache 8840B) · +8 context(&GameContext 64B; +0 pool=&Bump · +8 setting=&GameSetting) · +0x10 blackboard(&[Blackboard;2] 744B 원소) \| (배치 E) +0 cache(&AbstractGameWithCache) · +8 context(&GameContext: +0 pool(bump) · +8 setting · +0x3b debug) · +0x10 blackboard(&[Blackboard;2]) \| (배치 F) +0 cache(&AbstractGameWithCache · %176) · +8 context(&GameContext · %143 · +0 pool=bump %1079 · +0x3b debug L779) · +0x10 blackboard(&[Blackboard;2] · %2296 · L828/L804 는 [1-team] 인덱스) | 4 |
| 6 | 6 | parameter | &ScoreParameter(5384B) | +0 wave_snapshot@tag(i64, bit0 = Some) → +8 페이로드 &MinionWaveSnapshot (L912 base_positioning 인자, 21804~21807) · +0x9f0 positioning_score(&PositioningScoreData 2760B; L903 position_score_at_position · L183 lane_minion_position_action 인자, 21733) \| (배치 E) 읽기 +0 wave_snapshot Option tag(i64 bool) · +8 payload(&MinionWaveSnapshot) · +0x9f0 positioning_score. 클로저#7 에서 score() 로 전달 \| (배치 F) +0x9f0 positioning_score(PositioningScoreData 2760B · %237 배치 D 산출) → L853 get_input 6번째 · L860 position_score_at_position 4번째 인자. 그 외 L788/L837 에 통째 전달 | 4 |
| 7 | 7 | team_plan | &TeamPlan (ptr readnone) | 전 함수에서 **미사용**(정의 줄 외 %7 참조 0건). DI 이름 team_plan \| (배치 E) 내 범위 미사용 \| (배치 F) 배치 F 범위에서 미사용(%7 참조 0) | 4 |
| 8 | 8 | debug | &mut DebugFrameData(224B) | DI 이름 debug. 배치 D 범위에서는 읽기·쓰기·전달 모두 없음. 다른 배치: +0xa0 infos(HashMap<usize,Vec<String>>) entry 삽입 L347<766(26407, 배치 E)·L780(27209, 배치 E) · check_kill_die_tick(L766 25128)·score(L837 27726)·get_input(L853 28157) 8번째 인자. initializes 속성 없음 \| (배치 E) +0xa0 HashMap<usize,Vec<String>> — ctx.debug 일 때 champ.id 키로 format!("{}", has_enemy_minion_in_range) push(347행). closure#7 score() 에도 전달 \| (배치 F) +0xa0 infos(HashMap<usize,Vec<String>>) L780 쓰기(context.debug 일 때만) · L788/L837/L853 콜리에 &mut 전달 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
// line_defense.rs:0~454 (배치 D)
// 사장 코드: reach.py(version=2 · gamemode=0) 결과 이 함수는 사장 블록 0 · 사장 호출부 0 — NA 봉인 대상 없음.
// 소스 줄 0 루트 블록(%171 %291 %320 %345 %672 %908 %944 %956 및 %1268~%2699 의 cleanupret/unreachable)은 unwind 잡음이라 판정 없음.
//
// ── L440 진입. version(%2)을 %131 에 spill. team_plan(%7)은 전 함수 미사용.
// ── L441 let _t_ld = ProfTimer::start(92)  // prof::ENABLED(원자 로드)==0 이면 None(+16=-1). 계측 전용.
//
// ── L442 let old_actions = self.action_candidates_old(version, rnd, player, data, parameter)   // 통째 인라인(line_defense.rs:881~938, 936 IR줄)
//   [881~882] let bump = data.context.pool; let mut res = Vec::new_in(bump)   // %78: ptr=8 · bump · cap=len=0
//   [885] let _t = ProfTimer::start(87)
//   [886] let team = player.info.team(+0x930); assert team<2 (panic_bounds_check 2);
//         let champ = data.cache.player_champion[team][player.info.position.as_index()(+0x9c0)].unwrap()   // None → unwrap_failed
//   [889~895] let has_non_target_action_range = data.cache.iter_champions(1-team)   // player_champion[1-team] 5슬롯, None 건너뜀
//        .any(|c| nontarget_windup_perceived(version, player, data, c)      // ★먼저 호출됨(invoke 21616)
//                 && c.ty@0x68 == Champion(13)
//                 && c.is_in_skill()   // entity.rs:1572 인라인: match c.action_state@0x70 {
//                        //   4 Skill  → c.skill_effect(+0x4c8).as_ref().unwrap()   (casting@0x4f8 == -1 → unwrap 패닉)
//                        //   5 Skill2 → c.skill2_effect() = if c.level(+0x5c8) > 2 { &c.skill2_effect(+0x500) } else { &DEFAULT(@anon.22) }
//                        //   6 Ult    → c.ult_effect()    = if c.level > 4 { &c.ult_effect(+0x538) } else { &DEFAULT }
//                        //   _ → None }
//                        // 그리고 effect.casting(+0x30) ∈ {Position(1), Direction(2)} 일 때만 Some(effect) (0 Targeting / 3 None → None)
//                    .is_some_and(|e| Effect::is_in_range(e, caster=c, target=champ)))
//        // ⚠ `nontarget_windup_perceived && ty==Champion` 의 소스 순서는 column 부재로 확정 불가 — IR 은 호출을 먼저 하고 ty 비교를 select 로 합친다(21616~21623)
//   [903~904] let position_score: PositioningScore(56B) = position_score_at_position(version, player, data, &parameter.positioning_score(+0x9f0), champ.x(+0x660), champ.y(+0x668), line_phase_position_eval_purpose(player, data))
//   [905] if position_score.on_trajectory(+0x30) || has_non_target_action_range || position_score.on_periodic_trajectory(+0x31) {   // IR: select(+0x30, true, has_non_target | +0x31) — 세 항의 소스 순서는 미확정(column 0)
//   [907]     res.push(SmallActionPlay::RunAway(SmallActionRunAway::new_with_skill(data, player, 5, true)))   // 태그 3 @+0xb1
//   [908]     return res   // → old_actions(%129); prof 87 drop(L910) 후 L447 로 (블록 %916→%926→%943→%948)
//   }
//   [912] res.extend(self.base_positioning(version, rnd, player, data, &parameter.positioning_score, parameter.wave_snapshot.as_ref()))   // 통째 인라인(line_defense.rs:156~198)
//      [156] let _t = ProfTimer::start(64)
//      [157] let mut res2 = Vec::new_in(bump)   // %62
//      [158] let purpose = line_phase_position_eval_purpose(player, data)
//      [160] let nearest_tower = cache.tower(self.line(+1), team)   // simulation.rs:1823 인라인 = [top_tower(+0x180)|mid(+0x1a0)|bottom(+0x1c0)][team].or([top_tower2(+0x190)|mid2|bottom2][team])
//      [161~166]     .or(cache.twin_towers[team](+0x130+team*0x20).iter().min_by_key(|t| dist²(t.pos, self.line.get_start_position(context.setting, team))).copied())   // aux m12.ll:28442 fold · 빈 슬라이스면 None(21948~21957)
//      [167]     .or(cache.nexus[team](+0x170))
//      [168]     .unwrap();   // 셋 다 None → unwrap_failed
//      [170] let nexus = cache.nexus[team].unwrap();
//      [172] let champ = cache.player_champion[team][pos].unwrap();   // (L886 과 같은 슬롯 재로드)
//      [173] let front_minion = data.blackboard[team].minion_state(self.line)   // blackboard.rs:379: line 0→+0 top_minion_state · 1→+0x28 mid · 2→+0x50 bottom (그 외 unreachable)
//                .front_minion   // Option<usize>: tag@+0 bit0 · 값@+8
//      [174]     .and_then(|id| cache.game.get_entity_by_id(id))   // dyn AbstractGame vtable+0x1f0 간접 호출(22120)
//      [179] if front_minion.is_none_or(|m| dist²(m.pos, nexus.pos) < dist²(nearest_tower.pos, nexus.pos)) {   // 선두 미니언이 없거나, 있어도 아군 타워보다 넥서스에 가까우면(=타워 뒤)
//      [180~181]   res2.push(Around(SmallActionAround::new(version, rnd, data, player, nearest_tower.id(+0x5c0), 5).with_purpose(purpose)))   // 태그 5 · +0x80=purpose
//            } else {
//      [183]   let action: Option<SmallActionPlay> = lane_minion_position_action(version, rnd, data, player, self.line, &parameter.positioning_score, wave_snapshot, 5, purpose)
//      [185]   let has_precise = action.is_some()   // 태그@+0xb1 != 255 (IR 은 ==-1 로 뒤집어 분기, DW_OP_not 아티팩트)
//      [186~187] if has_precise { res2.push(action.unwrap()) }   // 184B memcpy 그대로(variant 는 lane_minion_position_action 계약)
//      [192~193] else { res2.push(Around(SmallActionAround::new(version, rnd, data, player, front_minion.id, 5).with_purpose(purpose))) }   // 태그 5
//            }
//      [197~198] prof 64 drop; return res2   // → extend(%78, ptr, len)
//   [913] if let Some(nearest_tower) = cache.tower(self.line, team).or(twin_towers[team] 최근접(L914~917, aux m12.ll:28752)).or(cache.nexus[team](L920)) {   // L160~167 과 동일 계산, unwrap 대신 if let
//   [921~924]   let mut around = SmallActionAround::new(version, rnd, data, player, nearest_tower.id, 5).with_purpose(line_phase_position_eval_purpose(player, data));
//              around.escape_mode(+0x81) = true;
//              res.push(SmallActionPlay::AroundRunAway(around))   // 태그 8
//   [926] } else { res.push(SmallActionPlay::RunAway(SmallActionRunAway::new(data, player, 5))) }   // 태그 3
//   [928] res.extend(battle_action(version, rnd, player, data, 5))
//   [929] res.extend(line_minion_action_candidates(version, data, player, self.line))
//   [931~933] { let _t = ProfTimer::start(95); res.extend(attack_summon_action(player, data)); }
//   [934] res.extend(self.attack_tower_action(version, player, data))   // 통째 인라인(line_defense.rs:373~394) → Option<SmallActionPlay>, extend 는 0/1개
//      [373] let _t = ProfTimer::start(89)
//      [374] let champ = cache.player_champion[team][pos]?;   // None → None
//      [375~377] let nearest_enemy_tower = cache.iter_towers(1-team)   // [top,mid,bottom,top2,mid2,bottom2][1-team] + twin_towers[1-team] + nexus[1-team]
//                    .filter(|t| t.can_target())   // +0x6b9 && +0x6a0==0 (aux m14.ll:61031 / m06.ll:9947)
//                    .min_by_key(|t| dist²(t.pos, champ.pos))?;   // 없으면 None (23271~23279)
//      [379] let move_speed = champ.stat_cached.move_speed(+0x640)
//      [380] let atk = champ.attack_effect(+0x490).as_ref()?;   // 니치 +0x4c0 == -1 → None
//      [382] let dist = dist²(tower.pos, champ.pos)
//      [383] let max_dist = atk.range(champ, tower)   // effect.rs:26 = atk.range(+0x4a0) + champ.stat_buff_cached.range(+0x438) + (champ.level-1)*atk.growth_range(+0x4a8) + Effect::range_adjust(atk, champ, tower)
//                         + champ.radius() + tower.radius()   // entity.rs:1511~1515: mult=radius_mult(+0x470); mult==0 ? radius(+0x680) : radius*(mult+100)/100
//      [384]               + move_speed*30
//      [385] if dist > max_dist*max_dist { return None }   // icmp ugt (23398) — 사거리+이속 여유 밖
//      [389] if v22_lane_tower_pressure_attack_allowed(version, data, player, tower) {
//      [390]     Some(SmallActionPlay::Attack(SmallActionAttack::new(data, tower.id)))   // 태그 15, live 24B
//            } else { None }
//      [394] prof 89 drop
//   [935] res.extend(attack_structure_skill_action(player, data))
//   [937~938] prof 87 drop; return res   // → old_actions %129 (23589)
//
// ── L447 let mut res = old_actions;   // %128 memcpy 32B
// ── L448 res.extend(battle_ally_action(version(%131 재로드), rnd, player, data, 5))
// ── L451 let champ = cache.player_champion[team][pos].unwrap();   // None → unwrap_failed(23739)
// ── L452~454 let nearest_enemy_tower = cache.iter_towers_without_nexus(1-team)   // 6 라인 타워 + twin_towers, 넥서스 제외
//                 .filter(|t| t.can_target())   // aux m14.ll:59793 / m11.ll:32089 / m06.ll:27567
//                 .min_by_key(|t| dist²(t.pos, champ.pos));   // Flatten 상태 -2/-1 은 이터레이터 내부 표지
//     · Some 경로: fold 호출(23982) → %1030 → 배치 E(줄 457: is_none_or 클로저 · Effect::is_in_range 로 enemy_tower_not_covering_champ 판정)
//     · None 경로(6슬롯 전부 None/can_target 불가 + 슬라이스 빈 경우): %1029 store null → %126(nearest_enemy_tower) → %1045 → 배치 E(줄 461)
// ── unwind: 내 범위 invoke 의 unwind 는 %169(ProfTimer 87 drop)→%150(res drop 조건부)→%945/%944→%946(ProfTimer 92 drop, 배치 F 줄 879 소유)→ 호출자.

// line_defense.rs:456~766 (배치 E)
// 입력(배치 D 산출): f=%958 = cache.player_champion[team][pos].unwrap() (내 챔피언, 451행) · nearest_enemy_tower=%126 (Option<&Entity>, 452~454행) · old_actions=%128 (Vec<SmallActionPlay>, 이전 틱 행동) · positioning_score=%237=parameter+0x9f0 · team=%160=player+0x930 · enemy=%183=1-team

// ── 457: 적 타워가 나를 못 덮는가
enemy_tower_not_covering_champ: bool = match nearest_enemy_tower { None => true, Some(t) => !t.attack_effect().as_ref().unwrap().is_in_range(t, f) }   // t+0x490 · None 이면 unwrap 패닉(anon.38)
// 459~462
f_atk    = &f.attack_effect (f+0x490, &Option<Effect>)
f_skill  = &f.skill_effect  (f+0x4c8)
f_skill2 = if f.level > 2 { &f+0x500 } else { &EMPTY(anon.22 = None) }
f_ult    = if f.level > 4 { &f+0x538 } else { &EMPTY }
_t_af = ProfTimer::start(93)   // 465, ENABLED 일 때만

// ── 466~552: act_actions = old_actions.iter().filter(closure#3).map(|a| a.clone()).collect_in(bump)
//   closure#3(a) — m14.ll 59817~ (캡처: data,f_atk,&enemy_tower_not_covering_champ,f,f_skill,player,&nearest_enemy_tower,f_skill2,f_ult):
  468: if a.tag == 13 (LaneMinionPosition) { return false }
  472: if a.tag ∈ {15,16,17,18} { let t = game.get_entity_by_id(a.target_id(+8));            // closure#1 = is_some_and(|t| !t.is_champion())
         if let Some(t) = t && t.ty != Champion(13) { return true } }                          // 비챔피언 대상(미니언·구조물·정글) 공격/스킬은 무조건 유지
  473: match a.tag (논리인덱스 tag-3):
    Attack(15) → 476: target = get_entity_by_id(a.target_id) else return false
                 477: effect = f_atk.unwrap()   // f.attack_effect None 이면 패닉(anon.39)
                 478: if enemy_tower_not_covering_champ || target.is_tower() || target.team == f.team { return effect.is_in_range(f, target) } else { return false }   // 적 타워 아래서 적 챔피언 평타 후보 제거
    Skill(16)  → 485: target = get_entity_by_id(...) else false
                 486: effect = f_skill.unwrap()   // None → 패닉(anon.40)
                 487: if !(enemy_tower_not_covering_champ || target.is_tower() || target.team == f.team) { return false }; if !effect.is_in_range(f, target) { return false }
                 488~489: if !effect.ty.can_move()(vtable+0x120) { return false }             // 이동 중 시전 불가 스킬 제거
                 490~492: if target.team != f.team && target.is_champion() { dmg = effect.expected_damage_target(data.context, f, target, anon.41); return is_dash_worth(data, player, f, target, dmg) }
                 494~495: return nearest_enemy_tower.is_none_or(|t| !t.attack_effect.unwrap().is_in_range_ex(t, target, t.x, t.y, target.x, target.y, 15000))   // 중첩 closure(m14.ll 6150) · 타워 attack_effect None 이면 패닉(anon.10)
    Skill2(17) → 507~517: 위 Skill 과 동일하되 f_skill2 사용(anon.42 · 중첩 closure m14.ll 6181)
    Ult(18)    → 529~539: 동일하되 f_ult (anon.43 · m14.ll 6212)
    그 외(RunAway3·Recall4·Around5~Trace14·Stop19·AroundPosition) → false
553: drop _t_af → PHASE_NANOS[93]+=elapsed · PHASE_CALLS[93]+=1
555: _t_ar = ProfTimer::start(94)

// ── 556~731: act_actions.retain(closure#5)  — m01.ll 8382~13039 (캡처: data, f, &version, player) · Skill(557~599)/Skill2(616~656)/Ult(673~713) 3분기 문자단위 동일(IR 비교로 확인, effect 선택만 다름)
  557: match a.tag: Skill(16)→559 / Skill2(17)→616 / Ult(18)→673 / 그 외 → keep(true)
  559: target = get_entity_by_id(a.target_id) else → 삭제(false)
  560: if target.team != f.team { keep }                                     // 아군 대상 스킬만 검사
  562: let Some(effect) = f.skill_effect (Skill2: level>2 ? +0x500 : EMPTY / Ult: level>4 ? +0x538) else → 삭제   // casting@+0x30 == -1 → None
  563: has_heal   = effect.ty.expected_heal  (vtable+0x40)(ctx=data.context, f, anon.11) != 0     // DW_OP_not 확인: dbg has_heal = !(ret==0)
  564: has_shield = effect.ty.expected_shield(vtable+0x48)(ctx, f, anon.11) != 0
  565: has_buff   = effect.ty.expected_buff  (vtable+0x50)(sret 288B, ctx, f, anon.11).is_some()   // 니치: ret+0x48 i32 == -1 → None
  567~572: if has_heal && !has_buff {
             hp_ratio = target.hp(+0x670)*100 / target.max_hp(+0x628)   // max_hp==0 → div_by_zero 패닉
             if hp_ratio > 79 && !aoe_heal_covers_low_ally(version, effect, data, player, target) { return false } }   // 체력 80%+ 아군에게 힐 낭비 금지
  576~580: if has_shield && !has_buff {   // 실드는 target 근처(<=120000) 에 적이 있어야 유지
             if enemy_champs(cache.player_champion[enemy]).flatten().any(|c| dist²(target,c) <= 120000² && c.is_visible_from(f)) { keep }   // 577
             if cache.iter_towers(enemy).any(|t| dist²(target,t) <= 120000² && t.is_visible_from(f)) { keep }              // 578 (m06.ll 46907)
             if cache.jungles.iter().any(|j| dist²(target,j) <= 120000² && j.is_visible_from(f)) { keep }                  // 579
             if cache.others[enemy].iter().any(|o| dist²(target,o) <= 120000²) { keep }                                    // 580 (가시성 검사 없음)
             return false }
  590~599: if !has_heal && !has_shield {   // 순수 버프/딜 스킬
             let Some(_) = fight_check::effect_buff_target(version, effect, data.context, f, anon.11) (sret 288B, +0x48==-1 → None) else { keep }   // 591: 버프 대상 정의 없음 → 유지
             has_near_combat = enemy_champs.any(|c| dist²(target,c)<=120000² && c.is_visible_from(f))   // 592
                            || iter_towers(enemy).any(같은 술어)                                          // 593
                            || others[enemy].any(|o| dist²<=120000²)                                     // 594
                            || iter_minions(enemy).any(|m| dist²(target,m) <= 120000²)                  // 597~598 (m06.ll 46776)
             return has_near_combat }                                                                    // 599
  그 외 조합(has_buff 이거나 heal/shield 검사 통과) → keep
  // is_visible_from(viewer): viewer.team Neutral(discr 1) → true / Player(t) → self.visible_state[t](+0x38+24t).tag == 0(Visible)
732: drop _t_ar(94)
735: _t_sr = ProfTimer::start(83)

// ── 736: act_actions.retain(|a| !v30_line_champion_action_tower_aggro_risk(version, data, player, a))   // m01.ll 13042 · 타워 어그로 위험 행동 제거
738: judgement_line_accuracy = player.parameter(+0x180).judge_line_accuracy()

// ── 742~755: act_actions.retain(closure#7)  — m01.ll 13214 (캡처: rnd, &judgement_line_accuracy, data, f, self, &version, parameter, player, debug)
  743: gated = rnd.gen_range(0..1000) > judgement_line_accuracy          // ★원소마다 롤(rnd 소비)
  744~746: target_is_ally = a.tag ∈ {16,17,18} && get_entity_by_id(a.target_id).is_some_and(|t| t.team == f.team)
  749: if gated { if !target_is_ally { keep } else { score = self.score(version, parameter, rnd, player, data, a, debug); keep iff score >= 0 } }
  752~753: else { score = self.score(...);  keep iff (target_is_ally ? score >= 0 : score >= -30) }   // IR: (score > -31 && ally) ? score > -1 : score > -31

// ── 757~762: 판단 오차 — 후보가 2개 이상이고 롤이 정확도를 넘으면 무작위 1개만 남김
757: if rnd.gen_range(0..1000) > judgement_line_accuracy && act_actions.len() > 1 {
  759: choosed = act_actions.choose(rnd).unwrap().clone()
  760: act_actions.truncate(0)
  761: act_actions.push(choosed) }
765: drop _t_sr(83)

// ── 766: move_actions = self.get_move_action_v46(version, rnd, player, data, positioning_score, parameter.wave_snapshot.as_ref()(+0 tag/+8), debug)   — line_defense.rs:202~366 완전 인라인
  202: _t = ProfTimer::start(81); 203: res = Vec::new_in(bump)
  204: position_eval_purpose = line_phase_position_eval_purpose(player, data)
  206: champ = cache.player_champion[team][pos].unwrap()   // None → 패닉(anon.122)
  209~216: has_non_target_action_range = cache.player_champion[enemy].iter().flatten().any(|c|
             nontarget_windup_perceived(version, player, data, c) && c.ty == Champion(13) &&
             match c.action_state(+0x70) { Skill(4) => c.skill_effect.unwrap(), Skill2(5) => c.skill2_effect().unwrap(), Ult(6) => c.ult_effect().unwrap(), _ => return false }   // unwrap: casting@+0x30 == -1 → 패닉(anon.44/45/46)
             .casting ∈ {Position(1), Direction(2)} && effect.is_in_range(c, champ))          // 적이 비타겟 스킬 시전 중이고 내가 그 사거리 안
  222~223: position_score = position_score_at_position(version, player, data, positioning_score, champ.x, champ.y, line_phase_position_eval_purpose(player, data))   // 56B sret · purpose 재호출
  224~227: if position_score.+48(bool) || has_non_target_action_range || position_score.on_trajectory(+49) {
             res.push(RunAway(tag3) = SmallActionRunAway::new_with_skill(data, player, end_delay=5, with_skill=true)); return res }   // ★조기 반환 → move_actions
  230: line = self.line(+1)
       tower = cache.{top,mid,bottom}_tower[line][team] (0x180+line*32) .or( *_tower2[line][team] (0x190+line*32) )
  231~236: .or( cache.twin_towers[team].iter().min_by_key(|t| dist²(t, LineType::get_start_position(&line, ctx.setting, team))).copied() )   // m12.ll 28597 fold · 동률이면 앞 원소
  237: .or( cache.nexus[team] )
  238: nearest_tower = ….unwrap()   // 전부 None → 패닉(anon.123)
  240: nexus = cache.nexus[team].unwrap()   // 패닉(anon.124)
  243~244: front_minion: Option<&Entity> = blackboard[team].{top,mid,bottom}_minion_state[line].front_minion(Option<usize>).map(|id| game.get_entity_by_id(id))  (None 가능)
  253: runaway = false
  254~256: positioning_accuracy = player.parameter.positioning_accuracy(); min_v = positioning_accuracy; max_v = 2000 - positioning_accuracy
  258: near_enemies = cache.player_champion[enemy].iter_champions().filter(|e| e.is_visible_from(champ) && dist²(e, champ) < 160000²).collect_in(bump)   // m14.ll 60488
  259: me_die_tick = check_kill_die_tick(version, rnd, data, player, champ, near_enemies.clone(), Vec::new_in(bump), debug)
  263: near_ally_count = cache.player_champion[team].iter_champions().filter(|x| x.id != champ.id && dist²(x, champ) < 160000²).count()
  264: local_outnumbered = near_enemies.len() <= near_ally_count + 1      // ⚠IR 극성: `icmp ule len, cnt+1` (이름과 반대로 읽히지만 IR 정본)
  265~267: has_moving_skill(이름 미상 %1676) = (champ.can_skill() && champ.skill_effect.unwrap().ty.can_move()) || (can_skill2() && skill2_effect.unwrap().ty.can_move()) || (can_ult() && ult_effect.unwrap().ty.can_move())   // effect None(casting -1) 이면 그 항은 false(unwrap 패닉 아님 · `let Some` 패턴)
  269: for enemy in near_enemies.iter() {
    270: jrng = range_misjudge_rng(version, data, player, enemy.id)   // {i64,i64} 16B
    271: mr       = max_range_can_use(champ, enemy)        * range_misjudge_roll(rnd, &jrng, min_v, max_v) / 1000
    272: emr      = max_range_can_use(enemy, champ)        * roll / 1000
    273: emr_near = max_range_nearly_can_use(enemy, champ, 40) * roll / 1000
    274: dist = dist²(champ, enemy)
    278: poke_range = max_poke_range(champ, enemy, tps) * roll / 1000   // tps = setting+0x12f8
    // trace_action(emr_x) (281~284 헬퍼): if poke_range > emr_x { Trace::new_keep_range(data, enemy.id, 5, poke_range) } else { Trace::new(data, enemy.id, 5) }  → tag 14
    290~292: bruiser_engage = local_outnumbered && has_moving_skill && emr >= mr && me_die_tick >= tps*2 && expected_trade_net_hp(data, champ, enemy) > 0
    295: if me_die_tick < tps {
      313: emr60 = max_range_nearly_can_use(enemy, champ, 60) * roll / 1000
      315~317: if dist > mr² && emr60 < mr { res.push(trace_action(emr60)) }
      318: else { runaway = runaway || dist <= emr60² } }
    296: else if enemy.remain_action_time() > 10 && mr >= 1 {
      297~298: if dist > mr² { res.push(trace_action(emr)) } }
    300~302: else if dist > mr² && emr < mr { res.push(trace_action(emr)) }
    303~306: else if bruiser_engage { if dist > mr² { res.push(Trace::new(data, enemy.id, 5)) } }
    308: else if dist <= emr_near² { runaway = true }
  }
  325~328: for e in cache.others[enemy].iter(): if let Some(atk)=e.attack_effect (+0x4c0 != -1) { range = e.stat_buff_cached.range(+0x438) + atk.range(+0x4a0) + (e.level-1)*atk.growth_range(+0x4a8) + atk.range_adjust(e, champ) + e.radius() + champ.radius();   // radius() = radius_mult==0 ? radius : radius*(mult+100)/100
             if dist²(champ, e) <= range² { → 336 } }
  335~336: if runaway || (위 any 참) { res.push(RunAway(tag3) = SmallActionRunAway::new_with_skill(data, player, 5, false)); runaway_pushed = true }
  340: champ_atk = champ.attack_effect.unwrap()   // 패닉(anon.125)
  341~344: has_enemy_minion_in_range = cache.iter_minions(enemy).any(|m| champ_atk.is_in_range_ex(champ, m, champ.x, champ.y, m.x, m.y, 30000)) || cache.iter_towers(enemy).any(|t| 같은 술어 30000)
  346~347: if ctx.debug(+0x3b) { debug.map(+0xa0).entry(champ.id).or_insert(Vec::new()).push(format!(anon.121, has_enemy_minion_in_range)) }
  351: if has_enemy_minion_in_range && runaway_pushed { return res }   // 도주 + 적 미니언 사정권 → 대기 후보 없음
  352: if front_minion.is_none_or(|m| dist²(m, nexus) < dist²(nearest_tower, nexus)) {   // 웨이브가 우리 타워보다 뒤(넥서스 쪽)이거나 없음
    353~354: res.push(Around(tag5) = SmallActionAround::new(version, rnd, data, player, nearest_tower.id, 5).with(position_eval_purpose @+0x80)) }
  else {
    356~358: if let Some(action) = lane_minion_position_action(version, rnd, data, player, line, positioning_score, wave_snapshot, 5, position_eval_purpose) (tag@0xb1 != -1) { res.push(action) }
    360~361: res.push(Around(tag5) = SmallActionAround::new(version, rnd, data, player, front_minion.id, 5).with(purpose)) }
  365~366: drop near_enemies; drop _t(81); return res }
→ move_actions = res(%110) · 이후 %2184(769행) = 배치 F
// ── 배치 경계: 진입 = %1029/%1030(452~454행 nearest_enemy_tower 저장 꼬리, 배치 D)→%1045(457행). 탈출 = %2130/%2164 → %2184(769행, 배치 F). unwind 경로 = %950(→배치 F 879행 cleanup) · %1084/%1169(→배치 D 정리, L0) · %1110/%1128(→%2697/%2698 878행 cleanup, 배치 F) — 판정 무관.

// line_defense.rs:769~879 (배치 F)
// 진입: 배치 E(L766 거대 인라인 문, 블록 %2164/%2151/%2130) → %2184 (m14.ll:26988). 이 시점의 값:
//   ret=%122 (bumpalo Vec<SmallActionPlay>, 배치 D/E 가 채움) · move_actions=%110 (L766 산출 Vec) · %128 = L447 Vec(배치 D, 여기선 드롭만)
//   champ=%958 = cache.player_champion[team][pos] (L451) · team=%160=player.info.team · %183 = 1-team · %184 = &cache.player_champion[1-team] ([5 x ptr], L889 헬퍼 인라인, 배치 D)
//   nearest_tower=%126 (Option<&Entity>, L452) · %1046 = nearest_tower.is_none() (L457 phi: %1029 null store→true · %1030 null 검사→true · %1042→false)
//   %237 = &parameter.positioning_score(+0x9f0) · %1174 = &player.info.parameter(+0x180) · %143 = data.context · %1079 = context.pool(bump) · %131 = version 스택슬롯

// ── L769~780 타워 어그로 소커 판정 ──
if let Some(nearest_tower) = nearest_tower {                       // br %1046 (26992): None → L787
  if let EntityType::Tower(info) = &nearest_tower.ty {            // L770 +0x68 == 2 (26999)
    if info.nearest_enemy.map(|e| e.1) == Some(champ.id) {        // L771: +0x88 tag is_some (27086) && +0x98 == champ+0x5c0 (27107, Option<usize>::eq)
      let siege = v47_siege_stance(version, data, player, nearest_tower);   // L774 (27112) → {i64,i64} Option<usize>(소커 id) ★TLS SIEGE_STANCE_CACHE(콜리 내부)
      let v47_soak_hold = siege.is_some() && siege.unwrap() == info.nearest_enemy.1;   // 27119~27125 (Option<usize>::eq 2439)
      if !v47_soak_hold {                                          // 27126 false → %2230
        ret.truncate(0);                                           // L776 (27130)
        move_actions.truncate(0);                                  // L777 (27141)
        move_actions.push(SmallActionPlay::RunAway(SmallActionRunAway::new_with_skill(data, player, 5, true)));   // L778 (27147 · 태그 3 @+177 27153 · push 27163~27200)
        // L775 → L787 (27202)
      } else if data.context.debug {                               // L779 +0x3b (27134~27137); false → L787 (27137)
        debug.infos.entry(champ.id).or_insert(vec![]).push(format!("v47 siege soak hold: tower {}", nearest_tower.id));   // L780 (27209~27270) 텔레메트리 → L787
      }
      // (v47_soak_hold && !debug) → L787
    }   // nearest_enemy 불일치 → L787 (27108)
  }     // Tower 아님 → L787 (27000)
}

// ── L787~792 후보 정리 ──
let has_runaway = move_actions.iter().any(|a| matches!(a, RunAway(_) | Recall(_) | AroundRunAway(_)));   // L787 (27002~27077): 원소 +0xb1 태그, tag<=2→7(AroundPosition)·else tag-3, switch {0,1,5}→true; 빈 Vec→false. %103 에 i8 저장(27274)
ret.retain(|a| !self.unsafe_v19_non_champion_walkup(version, rnd, player, data, parameter, a, has_runaway, debug));   // L788~789 (27276~27292, aux m01.ll:13557 — 콜리 true 면 삭제)
if !ret.is_empty() {                                               // L792 (%122+24 ==0 ? 27299~27301)
  return ret;                                                      // L876 memcpy %0←%122 (27310) → L878 drop move_actions(27312~) → L879 drop %128 · phase95 타이머 → ret
}

// ── L795~814 아군 타워 아래 전투 후보 ──
let towers = data.cache.iter_towers_without_nexus(player.team);   // L795 (27306) = [top,top2,mid,mid2,bottom,bottom2][team](Option ×6) chain twin_towers[team]
let tower_with_enemy_in_range = towers
  .filter(|t| t.can_target() && dist²(t.pos, champ.pos) < 2500000001)   // L796 closure$11(s9_0): +0x6b9 && +0x6a0==0 (entity.rs:1478 인라인) && ≤50000 (aux m06.ll:32280~32323)
  .any(|t| {                                                       // L797 closure$12(sa_0) 캡처 (cache, blackboard, player)
    let tower_attack_range = t.attack_effect.as_ref()              // L798 +0x4c0 tag==-1 → None → 0 (m06.ll:32345~32348)
        .map(|e| e.range(t) + t.radius()).unwrap_or(0);            // L799: Effect::range = stat_buff_cached.range(+0x438) + e.range(+0x4a0) + (level(+0x5c8)-1)*growth_range(+0x4a8) ; Entity::radius = mult(+0x470)==0 ? radius(+0x680) : radius*(mult+100)/100 (32354~32395)
    let team = 1 - player.team;                                    // L801 (32403~32404)
    cache.iter_champions(team).any(|c| {                           // L802 player_champion[team][0..5] 5인 언롤 (32411~)
      let range_with_enemy = c.radius() + tower_attack_range;      // L803 (32467~32501)
      data.blackboard[team].is_recent_visible(cache.game, player, c)   // L804 (32492) — blackboard[적팀]: 「Blackboard[team]=team 자체 정보, 관측은 1-team」(_docs game_core:21) 이므로 적 c 의 최근 시야
        && dist²(t.pos, c.pos) <= range_with_enemy²                // L805 (32508~32531 `ugt` → skip)
    })
  });                                                              // 슬라이스부(twin_towers)는 m11.ll:28250 동일 술어
if tower_with_enemy_in_range {                                     // L808 (27364)
  let mut battle_actions = fight_check::battle_action(version, rnd, player, data, 5);   // L809 (27373) sret 32B Vec
  battle_actions.retain(|a| !v30_line_champion_action_tower_aggro_risk(version, data, player, a));   // L810 (27383, aux m01.ll:13736 — true 면 삭제)
  if !battle_actions.is_empty() {                                  // L811 (27395~27398)
    move_actions.extend(battle_actions);                           // L812 (27421)
  } else { drop(battle_actions); }                                 // L814 (27402)
}

// ── L817~833 적 사거리 침범 도주 ──
let positioning_accuracy = player.info.parameter.positioning_accuracy();   // L817 (27367) ∈[10,1000]
let min_v = positioning_accuracy;                                  // L818
let max_v = 2000 - positioning_accuracy;                           // L819 (27431)
if cache.iter_champions(1 - team).any(|c| {                        // L823 closure$14 인라인 (27473~27610): %184 [5 x ptr] stride 8, null skip
    let jrng = range_misjudge_rng(version, data, player, c.id);    // L824 (27511) → 16B NoiseRng(%11)
    let emr = max_range_can_use(c, champ) * range_misjudge_roll(rnd, &jrng, min_v, max_v) / 1000 + 10000;   // L825 (27519, 27523, 27527~27529) — 적의 사거리를 오판 롤로 스케일
    let mr = max_range_can_use(champ, c);                          // L826 (27531)
    data.blackboard[1 - team].is_recent_visible(cache.game, player, c)   // L828 (27538, %2329 = blackboard[%183])
      && dist²(c.pos, champ.pos) <= emr*emr                        // L829 (27546~27575 `ugt` → skip)
      && !c.is_in_action()                                         // L830 entity.rs:1548 인라인: ty==Champion(13) && action_state.tag>=3 → 행동중 → skip (27578~27585)
      && !c.block_input()                                          // L830 (27588) ※ readonly 라 mr 검사보다 먼저 계산됨(27592~27594 `or`) — 소스상 순서는 표기 불가(column 부재), 외연 동일
      && mr == 0                                                   // L831~832 (27592: mr!=0 → skip)
}) {
  return vec![SmallActionPlay::RunAway(SmallActionRunAway::new_with_skill(data, player, 5, false))];   // L833 (27615 · 태그3 28391 · from_iter_in 28392 · bump=%1079) → L878/879 드롭·타이머 → ret
}

// ── L836~838 최고점 이동행동 ──
let _t_mb = ProfTimer::start(97);                                  // L836 (27622~27633, 27738) prof::ENABLED 일 때만 Instant::now
let best_move_action = move_actions.iter()
  .max_by_key(|a| self.score(version, parameter, rnd, player, data, a, debug))   // L837 (27636~27733, aux m12.ll:23095 score → i64 · compare>0 이면 앞 원소 유지 = 동점 시 뒤 원소)
  .unwrap();                                                       // L837: move_actions 비어 있으면 unwrap_failed 패닉 (27704~27705 → 27816)
drop(_t_mb);                                                       // L838 PHASE_NANOS[97]+=elapsed · PHASE_CALLS[97]+=1 (27758~27813)

// ── L845~849 궤적 위험 ──
let trajectory_possible = cache.game.iter_projectile()             // L845 vtable+0x210 (27822~27826) → ProjectileIter(40B), next() 루프 (27844)
  .any(|p| p.team != TeamType::Player(player.team)                 // L846 +0 tag==0 && +8==team → skip (27864~27879)
         && !p.move_type.is_targeting()                            // L846 projectile.rs:134 인라인: Target(6)|TargetSplash(7)|BouncingTarget{target_id:Some} → skip (27883~27898)
         && dist²(champ.pos, (p.x,p.y)) < 176400000000)            // L847 (27905~27927) 420000²
  || cache.iter_champions(1 - team).any(|c| matches!(c.rush_state, Rush{..} | RushPenetrate{..}));   // L848~849 closure$17 5인 언롤 (27948~28090): +0x308 >=0 || ==-2^63+3
// (projectile 발견 → 28106 trajectory_possible=1 (L851) → %2555 / rush 발견 → %2555 / 둘 다 없음 → 28100 false (L857) → L871)

// ── L851~871 이동 입력 궤적 검증 ──
if trajectory_possible {
  let _t_mp = ProfTimer::start(96);                                // L852 (28120~28148)
  let move_action_input = best_move_action.clone().get_input(version, rnd, player, data, &parameter.positioning_score, debug);   // L853 (28136 clone → 28157 get_input sret 32B Option<Input>) ; 클론 드롭 28209
  drop(_t_mp);                                                     // L854 PHASE[96] (28179~28199)
  if let Some(input) = move_action_input {                         // L857 tag != -1 (28212~28213); None → L871
    if let Input::Move(x, y) = input {                             // L858 tag==0 (28224) x=+8 y=+0x10
      let purpose = line_phase_position_eval_purpose(player, data);   // L861 (28231) i8
      let position_score = position_score_at_position(version, player, data, &parameter.positioning_score, x, y, purpose);   // L860 (28242) sret 56B PositioningScore ★TLS POS_EVAL_CACHE(콜리 내부 position_eval_at)
      if position_score.on_trajectory || position_score.on_periodic_trajectory {   // L862 +0x30 || +0x31 (28246~28253)
        return vec![SmallActionPlay::RunAway(SmallActionRunAway::new_with_skill(data, player, 5, false))];   // L863 (28258 · 태그3 28285 · from_iter_in 28286)
      } else {
        return vec![best_move_action.clone()];                     // L865 (28264 · 28270)
      }
    } else {
      return vec![best_move_action.clone()];                       // L868 (28237 · 28296)
    }
  }
}
return vec![best_move_action.clone()];                             // L871 (28112 · 28327) — trajectory_possible=false 또는 get_input None

// ── L878~879 종료(전 반환 경로 공통) ──
// drop move_actions(%110) (27312 / 28307 / 28398) → drop ret(%122) (28340 / 28421 — L876 경로는 이미 이동돼 %122 드롭 없음 27310→27312→%2683→%2635) → drop %128(L447 Vec, 28363 / 28444) → 함수 타이머(%130, phase 95, 배치 D L441) 드롭 (28466~28504) → ret void (28508)
// 배치 F 안의 정상 흐름은 다른 배치로 돌아가지 않는다(모든 br 목적지가 범위 내 또는 cleanup pad %2213/%2427/%2309/%1084/%1110/%946/%950). 예외 경로 unwind 만 %1169/%1128 (배치 E 의 cleanup) 으로 이어진다.
```

**`mem` 메모리 접근 146건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | LineDefenseSubPlan(self) | 0x1 | line (LineType: 0 Top/1 Mid/2 Bottom) | r | L160(21875 m14) cache.tower(line,team) 인덱스(line<<5) · L173 blackboard.minion_state(line) switch(22083) · L913(22445) · L183/L929 콜리 인자 · get_start_position self 인자(&self.line) | 4 | OK |  |
| 1 | PlayerState | 0x930 | info.team | r | L886(21511) — bounds<2 검사 후 player_champion[team] · 1-team 으로 적 팀(21556) · 클로저 캡처 get_start_position team \| (배치 E) team(%160) · enemy=1-team | 4 | OK |  |
| 2 | PlayerState | 0x9c0 | info.position@tag (as_index) | r | L886(21541) player_champion[team][pos] | 4 | OK |  |
| 3 | OperationData | 0x0 | cache (&AbstractGameWithCache) | r | 21543 | 4 | OK |  |
| 4 | OperationData | 0x8 | context (&GameContext) | r | 21477 → +0 pool(bump 21478/21850) · +8 setting(21913, get_start_position 인자) | 4 | OK |  |
| 5 | OperationData | 0x10 | blackboard (&[Blackboard;2]) | r | L173(22080) blackboard[team] | 4 | OK |  |
| 6 | GameContext | 0x0 | pool (&Bump) | r | Vec::new_in(bump) L882/L157 | 4 | OK |  |
| 7 | GameContext | 0x8 | setting (&GameSetting 5432B) | r | LineType::get_start_position(&line, setting, team) 2번째 인자(21974/22535 + aux fold) | 4 | OK |  |
| 8 | AbstractGameWithCache | 0x0 | game (&dyn AbstractGame: data ptr +0 · vtable +8) | r | L174(22109~22120) vtable+0x1f0 = get_entity_by_id(game, id) 간접 호출(divtable) | 3 | OK |  |
| 9 | AbstractGameWithCache | 0x130 | twin_towers[team] (bumpalo Vec<&Entity>; 32B stride · ptr@+0 · len@+0x18) | r | L161/L914(21898~21901, 22464~22466) 최근접 후보 | 4 | OK |  |
| 10 | AbstractGameWithCache | 0x170 | nexus[team] (Option<&Entity>) | r | L167/L170(22042, 22059) .or(nexus) 및 nexus.unwrap() · L920(22596) | 4 | OK |  |
| 11 | AbstractGameWithCache | 0x180 | top_tower[team] (+line*0x20: 0x1a0 mid · 0x1c0 bottom) | r | cache.tower(line, team) 1순위 (L160 21884~21886, L913 22453~22454) | 4 | OK |  |
| 12 | AbstractGameWithCache | 0x190 | top_tower2[team] (+line*0x20: 0x1b0 mid2 · 0x1d0 bottom2) | r | cache.tower(line, team) 2순위 .or (21887~21891, 22455~22459) | 4 | OK |  |
| 13 | AbstractGameWithCache | 0x1e0 | player_champion[team][pos] (Option<&Entity>, 8B·[2][5]) | r | L886(21545~21548) champ.unwrap() · L889 iter_champions(1-team) = [1-team] 5슬롯 순회(21558, 21587~21599) · L172(22068) · L374(22918) · L451(23722) | 4 | OK |  |
| 14 | Blackboard | 0x0 | top_minion_state / +0x28 mid / +0x50 bottom (BrainMinionParameter 40B, line 으로 switch) | r | L173(22083~22099) minion_state(line).front_minion: Option<usize> = tag@+0(bit0) · 값@+8(22114) | 4 | OK |  |
| 15 | ScoreParameter | 0x0 | wave_snapshot@tag (i64, bit0=Some) | r | L912(21804~21807) as_ref → +8 페이로드 ptr 또는 null | 4 | OK |  |
| 16 | ScoreParameter | 0x9f0 | positioning_score (PositioningScoreData 2760B) | r | L903(21733) position_score_at_position 4번째 인자 · L183 lane_minion_position_action 6번째 인자 | 4 | OK |  |
| 17 | PositioningScore(56B sret, L903) | 0x30 | on_trajectory (bool) | r | L905(21746~21748) 조기 RunAway 조건 | 4 | OK |  |
| 18 | PositioningScore(56B sret, L903) | 0x31 | on_periodic_trajectory (bool) | r | L905(21749~21751) 조기 RunAway 조건 | 4 | OK |  |
| 19 | Entity | 0x68 | ty@tag (EntityType; 13=Champion) | r | L890(21620~21622) 적 챔피언 루프 술어 | 4 | OK |  |
| 20 | Entity | 0x70 | ty@Champion.action_state@tag (4 Skill/5 Skill2/6 Ult) | r | L891 is_in_skill switch(21647~21653) | 4 | OK |  |
| 21 | Entity | 0x4c8 | skill_effect (Option<Effect> 56B, 니치 = +0x4f8 casting@tag) | r | action_state==4: +0x4f8 switch {-1 → unwrap 패닉(None), 1\|2 → is_in_range, 그 외 → false}(21657~21663) | 4 | OK |  |
| 22 | Entity | 0x500 | skill2_effect | r | action_state==5: level>2 이면 &skill2_effect 아니면 정적 DEFAULT(@anon…22)(21680~21684) → +0x30 casting@tag 검사(21686~21692) | 4 | OK |  |
| 23 | Entity | 0x538 | ult_effect | r | action_state==6: level>4 이면 &ult_effect 아니면 DEFAULT(21703~21707) → casting@tag 검사 | 4 | OK |  |
| 24 | Entity | 0x5c8 | level | r | L893/L895 skill2_effect()/ult_effect() 게이트(>2 / >4) · L383 (level-1)*growth_range(23332, 23340~23341) \| (배치 E) skill2/ult 존재 판정(>2 / >4) · 327 growth | 4 | OK |  |
| 25 | Entity | 0x5c0 | id | r | Around::new target(L180 22135 · L192 22265 · L921 22607) · SmallActionAttack::new target(L390 23410) \| (배치 E) 263 아군 카운트 제외 · 270 range_misjudge_rng 키 · Trace/Around 생성자 target · 347 debug 키 | 4 | OK |  |
| 26 | Entity | 0x660 | x | r | dist² 계산 전역(21735, 21987, 22142~22150, 22548, 23231~23239, 23298~23306, 23949~23957) · L904 position_score_at_position 좌표 | 4 | OK |  |
| 27 | Entity | 0x668 | y | r | 위와 짝 | 4 | OK |  |
| 28 | Entity | 0x6b9 | can_target (bool) | r | Entity::can_target() L376/L453 (23088, 23200, 23872 + aux) | 4 | OK |  |
| 29 | Entity | 0x6a0 | block_target_tick (usize) | r | can_target(): ==0 이어야 대상 가능(23091~23093, 23875~23877 + aux) | 4 | OK |  |
| 30 | Entity | 0x640 | stat_cached.move_speed | r | L379(23285) max_dist += move_speed*30 | 4 | OK |  |
| 31 | Entity | 0x4c0 | attack_effect@tag (니치 -1 = None) | r | L380(23289~23291) champ.attack_effect.as_ref()? — None 이면 타워 공격 후보 None | 4 | OK |  |
| 32 | Entity | 0x490 | attack_effect (Effect 56B: +0x10 range=+0x4a0 · +0x18 growth_range=+0x4a8) | r | L383 Effect::range(champ, tower) 인라인(effect.rs:26; 23328~23331) + Effect::range_adjust 호출(23336) | 4 | OK |  |
| 33 | Entity | 0x438 | stat_buff_cached.range | r | effect.rs:26 range 합산항(23334~23335) \| (배치 E) 327행 Effect::range(entity) 인라인 | 4 | OK |  |
| 34 | Entity | 0x680 | radius | r | Entity::radius()(entity.rs:1511~1515) champ·tower 양쪽(23349, 23356, 23372, 23379) \| (배치 E) 327 | 4 | OK |  |
| 35 | Entity | 0x470 | stat_buff_cached.radius_mult (i32) | r | radius(): mult==0 이면 radius 그대로, 아니면 radius*(mult+100)/100 (23343~23360, 23366~23383) | 4 | OK |  |
| 36 | LineDefenseSubPlan | 0x1 | line | r | 230행 · LineType 태그(Top0/Mid1/Bottom2) → 타워·블랙보드 라인 선택 | 4 | OK |  |
| 37 | PlayerState | 0x180 | parameter | r | AthleteParameter::judge_line_accuracy(738) · positioning_accuracy(254) | 4 | OK |  |
| 38 | OperationData | 0x0 | cache | r |  | 4 | OK |  |
| 39 | OperationData | 0x8 | context | r | context+0 = pool(bump) · context+8 = setting · context+0x3b = debug(bool) | 4 | OK |  |
| 40 | OperationData | 0x10 | blackboard | r | [Blackboard;2] 744B stride, team 인덱스 | 4 | OK |  |
| 41 | GameContext | 0x8 | setting | r | get_start_position 인자 · setting+0x12f8 tick_per_second | 4 | OK |  |
| 42 | GameContext | 0x3b | debug | r | 346행 debug 문자열 기록 게이트 | 4 | OK |  |
| 43 | GameSetting | 0x12f8 | tick_per_second | r | 278 max_poke_range 인자 · 290 me_die_tick ≥ tps*2 · 295 me_die_tick < tps | 4 | OK |  |
| 44 | AbstractGameWithCache | 0x0 | game (&dyn AbstractGame 팻포인터) | r | vtable+0x1f0(496) = get_entity_by_id(id) → Option<&Entity> | 4 | OK |  |
| 45 | AbstractGameWithCache | 0xd0 | jungles (Vec ptr@0xd0 len@0xe8) | r | closure#5 579/636/693 — 실드 스킬 유지 판정 | 4 | OK |  |
| 46 | AbstractGameWithCache | 0xf0 | others[team] (Vec 32B stride) | r | 325행 적 others 사거리 체크 · closure#5 580/594 등 | 4 | OK |  |
| 47 | AbstractGameWithCache | 0x130 | twin_towers[team] | r | 231행 min_by_key 후보 | 4 | OK |  |
| 48 | AbstractGameWithCache | 0x170 | nexus[team] | r | 237·240행 | 4 | OK |  |
| 49 | AbstractGameWithCache | 0x180 | top_tower / mid_tower / bottom_tower [team] (line*32 stride) | r | 230행 line 별 1차 타워 | 4 | OK |  |
| 50 | AbstractGameWithCache | 0x190 | top_tower2 / mid_tower2 / bottom_tower2 [team] | r | 230행 .or() 폴백 | 4 | OK |  |
| 51 | AbstractGameWithCache | 0x1e0 | player_champion[team][pos] (80B, 5×8B per team) | r | champ(206) · 아군 5슬롯(263) · 적 5슬롯(209·258 · closure#5 577/592) | 4 | OK |  |
| 52 | Blackboard | 0x0 | top/mid/bottom_minion_state.front_minion (Option<usize>: tag@+0 payload@+8, line*40 stride) | r | 243~244행 front_minion → get_entity_by_id | 4 | OK |  |
| 53 | Entity | 0x0 | team (TeamType: discr@0 Player=0/Neutral=1 · Player idx@+8) | r | TeamType::eq(478/487/509/531/560/746) · is_visible_from(1136~1137) | 4 | OK |  |
| 54 | Entity | 0x38 | visible_state[team] (24B stride, tag 0=Visible) | r | is_visible_from → visible_state[viewer.team]==Visible | 4 | OK |  |
| 55 | Entity | 0x68 | ty@tag (EntityType: 2 Tower · 13 Champion) | r |  | 4 | OK |  |
| 56 | Entity | 0x70 | ty@Champion.0.action_state@tag (4 Skill · 5 Skill2 · 6 Ult) | r | 211~215행 비타겟 스킬 시전 중 판정 | 4 | OK |  |
| 57 | Entity | 0x470 | stat_buff_cached.radius_mult | r | 327행 Entity::radius() 인라인 | 4 | OK |  |
| 58 | Entity | 0x490 | attack_effect (Option<Effect>, casting 니치@0x4c0 == -1 → None) | r | f_atk(459) · 326행 others.attack_effect · 340행 champ_atk · 457행 타워 attack_effect | 4 | OK |  |
| 59 | Entity | 0x4a0 | attack_effect.range | r | 327 | 4 | OK |  |
| 60 | Entity | 0x4a8 | attack_effect.growth_range | r | 327 (level-1)*growth_range | 4 | OK |  |
| 61 | Entity | 0x4c8 | skill_effect (Option<Effect>, 니치@0x4f8) | r | f_skill(460) · 211 · 265 · closure#5 562 | 4 | OK |  |
| 62 | Entity | 0x500 | skill2_effect (level>2 일 때만 유효, 아니면 EMPTY 정적) | r | 461 · 213 · 266 · closure#5 619 | 4 | OK |  |
| 63 | Entity | 0x538 | ult_effect (level>4) | r | 462 · 215 · 267 · closure#5 676 | 4 | OK |  |
| 64 | Entity | 0x628 | stat_cached.hp (max_hp) | r | closure#5 568 hp_ratio 분모(0 이면 div_by_zero 패닉) | 4 | OK |  |
| 65 | Entity | 0x670 | hp | r | closure#5 568 | 4 | OK |  |
| 66 | Effect | 0x0 | ty (Arc<dyn EffectType>: +0 ptr · +8 vtable) | r | vtable+0x40 expected_heal · +0x48 expected_shield · +0x50 expected_buff(sret 288B) · +0x120 can_move (divtable 일치율 94%) | 3 | OK |  |
| 67 | Effect | 0x30 | casting (CastingType 0 Targeting/1 Position/2 Direction/3 None · Option<Effect> 니치 -1) | r | 211~215 비타겟(1\|2) 판정 | 4 | OK |  |
| 68 | SmallActionPlay | 0x8 | target id (Attack/Skill/Skill2/Ult 페이로드 +8) | r | closure#3 472~529 · closure#5 559 · closure#7 744 | 4 | OK |  |
| 69 | SmallActionPlay | 0xb1 | tag | r | -1(0xFF) = Option<SmallActionPlay>::None (356행 lane_minion_position_action 결과 · retain 내부) | 4 | OK |  |
| 70 | SmallActionAround | 0x80 | position_eval_purpose | r | 354·361행 .purpose 세팅 | 4 | OK |  |
| 71 | position_score(56B sret) | 0x30 | bool@+48 (이름 미상) / on_trajectory@+49 | r | 224행 도주 조건 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 72 | debug | 0xa0 | HashMap<usize,Vec<String>> | r | 347행 (ctx.debug 일 때만) | 4 | 확인불가(★모호: 동명 def_path 4개 [('game_core::DebugE) |  |
| 73 | Entity(nearest_tower %126→%2186) | 0x68 | ty@tag | r | L770 ==2(Tower) (m14.ll:26997~27000). tcxdict --enum EntityType: Tower 태그 2 | 3 | OK |  |
| 74 | Entity(nearest_tower) | 0x88 | ty@Tower.info.nearest_enemy@tag (Option<(usize,usize)>) | r | L771 trunc i1 → is_some (m14.ll:27081~27087). Tower 독립구조체 +0x18 = Entity+0x70+0x18 | 4 | OK |  |
| 75 | Entity(nearest_tower) | 0x98 | ty@Tower.info.nearest_enemy@Some.0.1 (usize = 대상 entity id) | r | L771 == champ.id (27095~27107, Option<usize>::eq) · L774 == v47 반환값(27123) · L780 debug 맵 키(27210) | 4 | OK |  |
| 76 | Entity(nearest_tower) | 0x5c0 | id | r | L780 format!("v47 siege soak hold: tower {}") 인자(27251) | 4 | OK |  |
| 77 | Entity(champ %958 = cache.player_champion[team][pos]) | 0x5c0 | id | r | L771 (27100~27101) · L780 infos 키 | 4 | OK |  |
| 78 | Entity(champ) | 0x660 | x | r | L829 (%2327→27554) · L847 (27839) · aux L796 (m06.ll:32301) | 4 | OK |  |
| 79 | Entity(champ) | 0x668 | y | r | L829 · L847 · aux L796 | 4 | OK |  |
| 80 | Entity(적 챔피언 c, cache.player_champion[1-team][i]) | 0x5c0 | id | r | L824 range_misjudge_rng 4번째 인자(27509~27510) | 4 | OK |  |
| 81 | Entity(c) | 0x660 | x | r | L829 dist² (27546~27547) | 4 | OK |  |
| 82 | Entity(c) | 0x668 | y | r | L829 | 4 | OK |  |
| 83 | Entity(c) | 0x68 | ty@tag | r | L830 is_in_action 인라인(entity.rs:1548): !=13(Champion) → 행동중 아님 (27578~27584) | 4 | OK |  |
| 84 | Entity(c) | 0x70 | ty@Champion.0.action_state@tag | r | L830 is_in_action: Champion 이고 tag>=3(Attack3/Skill4/Skill2 5/Ult6) 이면 행동중 (27581~27584) | 4 | OK |  |
| 85 | Entity(c) | 0x308 | rush_state@tag (니치 8B) | r | L849 closure$17: >=0(untagged RushPenetrate) \|\| ==-9223372036854775805(Rush 태그 niche_start+3) (27957~27965, 5인 언롤 27988~28090) | 4 | OK |  |
| 86 | Entity(t, 아군 타워 · aux) | 0x6b9 | can_target | r | aux L796 closure$11 = Entity::can_target() 인라인(entity.rs:1478): can_target && block_target_tick==0 (m06.ll:32280~32287) | 4 | OK |  |
| 87 | Entity(t) | 0x6a0 | block_target_tick | r | aux L796 ==0 (m06.ll:32283~32285) | 4 | OK |  |
| 88 | Entity(t) | 0x4c0 | attack_effect@tag (Option<Effect> 니치 4B) | r | aux L798 ==-1(None) → tower_attack_range=0 (m06.ll:32345~32348) | 4 | OK |  |
| 89 | Entity(t) | 0x4a0 | attack_effect@Some.0.range | r | aux L799 Effect::range(effect.rs:26) 인라인 (m06.ll:32354~32355) | 4 | OK |  |
| 90 | Entity(t) | 0x4a8 | attack_effect@Some.0.growth_range | r | aux L799 (level-1)*growth_range (32356~32357, 32390~32392) | 4 | OK |  |
| 91 | Entity(t) | 0x438 | stat_buff_cached.range | r | aux L799 (32365~32366) | 4 | OK |  |
| 92 | Entity(t) | 0x5c8 | level | r | aux L799 level-1 (32363~32364, 32390) | 4 | OK |  |
| 93 | Entity(t · c) | 0x470 | stat_buff_cached.radius_mult(i32) | r | aux L799/L803 Entity::radius() 인라인(entity.rs:1511~1515): mult==0 → radius, else radius*(mult+100)/100 (32367~32385, 32467~32485) | 4 | OK |  |
| 94 | Entity(t · c) | 0x680 | radius | r | aux L799/L803 (32374~32375, 32381~32382) | 4 | OK |  |
| 95 | Entity(t · c) | 0x660 | x | r | aux L796 dist(t,champ) · L805 dist(t,c) (32293~32294, 32508~32509) | 4 | OK |  |
| 96 | Entity(t · c) | 0x668 | y | r | aux L796 · L805 | 4 | OK |  |
| 97 | PlayerState(player) | 0x930 | info.team | r | L795 iter_towers_without_nexus 인자(%160 재사용, 27306) · L846 p.team 비교(27878) · aux L801 team=1-team (m06.ll:32403~32404) | 4 | OK |  |
| 98 | PlayerState(player) | 0x180 | info.parameter (AthleteParameter) | r | L817 positioning_accuracy(&parameter) (%1174 = %4+384, 27367) | 4 | OK |  |
| 99 | OperationData(data) | 0x0 | cache (&AbstractGameWithCache) | r | %176 · L795 · L823/L848 player_champion · L828/L845 game 팻포인터 | 4 | OK |  |
| 100 | OperationData(data) | 0x8 | context (&GameContext) | r | %143 · L779 +0x3b debug(27134~27136) · +0x0 pool → from_iter_in bump(%1079) | 4 | OK |  |
| 101 | OperationData(data) | 0x10 | blackboard (&[Blackboard;2]) | r | %2296(27337~27338) · L828 blackboard[1-team](%183 · 27471 gep) · aux L804 blackboard[1-team](m06.ll:32491) | 4 | OK |  |
| 102 | GameContext(context) | 0x3b | debug | r | L779 (27134~27137) | 4 | OK |  |
| 103 | GameContext(context) | 0x0 | pool (&Bump) | r | %1079 = load %143 (24113, 배치 D 산출) → L833/863/865/868/871 from_iter_in 할당자 | 4 | OK |  |
| 104 | AbstractGameWithCache(cache) | 0x0 | game.data_ptr | r | L828 is_recent_visible 2번째 인자(27536) · L845 iter_projectile self(27822) | 4 | OK |  |
| 105 | AbstractGameWithCache(cache) | 0x8 | game.vtable_ptr | r | L828(27537) · L845 vtable+0x210 = AbstractGame::iter_projectile (divtable, 27823~27826) | 3 | OK |  |
| 106 | AbstractGameWithCache(cache) | 0x1e0 | player_champion[1-team][0..5] (Option<&Entity>) | r | %184 = +480 + (1-team)*40 (배치 D 산출 21558) · L823 closure$14 루프(27476~27487, stride 8 · 40 종료) · L848 closure$17 5인 언롤(27948, 27979, 28010, 28041, 28072) | 4 | OK |  |
| 107 | Projectile(p, iter_projectile 원소) | 0x0 | team@tag (TeamType) | r | L846 ==0(Player) (27864~27867) | 4 | OK |  |
| 108 | Projectile(p) | 0x8 | team@Player.0 (usize) | r | L846 == player.team → 아군 투사체 skip (27877~27879) | 4 | OK |  |
| 109 | Projectile(p) | 0x40 | move_type@tag (ProjectileMoveType 니치 8B) | r | L846 ProjectileMoveType::is_targeting(projectile.rs:134) 인라인: 태그 6(Target)·7(TargetSplash) 또는 untagged BouncingTarget 의 target_id@+0x40 tag==1(Some) 이면 targeting → skip (27883~27898) | 4 | OK |  |
| 110 | Projectile(p) | 0x100 | x | r | L847 dist²(champ, p) (27905~27906) | 4 | OK |  |
| 111 | Projectile(p) | 0x108 | y | r | L847 | 4 | OK |  |
| 112 | PositioningScore(%89, L860 sret 56B) | 0x30 | on_trajectory | r | L862 (28246~28248) | 4 | OK |  |
| 113 | PositioningScore(%89) | 0x31 | on_periodic_trajectory | r | L862 (28249~28251) | 4 | OK |  |
| 114 | Option<Input>(%92, L853 get_input sret 32B) | 0x0 | tag (None=-1 · Input::Move=0) | r | L857 ==-1 → None (28211~28213, None 인코딩 근거 get_input m11.ll:42866 `store i64 -1` from_residual) · L858 ==0 → Move (28224) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 115 | Option<Input>(%92) | 0x8 | Move.x | r | L858 (28217~28218) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 116 | Option<Input>(%92) | 0x10 | Move.y | r | L858 (28220~28221) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 117 | ScoreParameter(parameter) | 0x9f0 | positioning_score (PositioningScoreData 2760B) | r | %237(배치 D 21733) → L853 get_input · L860 position_score_at_position | 4 | OK |  |
| 118 | DebugFrameData(debug) | 0xa0 | infos (HashMap<usize,Vec<String>>) | r | L780 rustc_entry(%8+160) (27209~27210) | 4 | OK |  |
| 119 | SmallActionPlay(move_actions 원소) | 0xb1 | tag | r | L787 has_runaway: tag-3 <=u … switch {0,1,5}=RunAway3/Recall4/AroundRunAway8 (27039~27064) | 4 | OK |  |
| 120 | bumpalo Vec(move_actions %110 · ret %122 · battle_actions %99) | 0x18 | len | r | L787(27009) · L792 ret.len(27299~27300, %1190=%122+24) · L811 battle_actions.len(27395~27397) · L837(27638) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 121 | bumpalo Vec(move_actions %110) | 0x10 | cap | r | L778 push 시 len==cap → reserve_internal_or_panic (27166~27169) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 122 | bumpalo Vec(move_actions %110 · battle_actions %99) | 0x0 | ptr | r | L787(27006) · L812 extend 원본 ptr(27420) · L837(27636) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 123 | res (%78, 로컬 bumpalo Vec<SmallActionPlay> 32B, action_candidates_old L882) | 0x0 | Vec 헤더 4슬롯 + 원소 | w | push 경로 전부 reserve_internal_or_panic(cap==len 일 때) → gep 원소 → memcpy 184 → len+1 store(+0x18: 22681~22682, 22743~22744, 23557~23558, 23639~23640) | 4 | 확인불가(★모호: 동명 def_path 40개 [('game_ai::path_fi) | new_in(bump): ptr=8·bump·cap=0·len=0 (21480~21485) → push/extend 로 원소 추가 → L937/L908 에서 %129(old_actions) 로 memcpy 32B(23589, 23642) |
| 124 | res2 (%62, 로컬 Vec, base_positioning L157) | 0x0 | Vec 헤더 + 원소 | w | len 증가는 %470=%468+1 로 extend 인자에만 반영(22327~22328) | 4 | 확인불가(tcx 사전에 타입 없음) | Around(L180)/LaneMinionPosition 등(L187)/Around(L192) 중 정확히 1개 push 후 L912 extend(ptr,len) 로 %78 에 편입(22441) |
| 125 | old_actions (%129 → %128, 로컬 32B) | 0x0 | Vec 헤더 복사 | w | 이후 배치 E/F 가 %128 을 res 로 계속 사용 | 4 | 확인불가(tcx 사전에 타입 없음) | L442 결과(23589/23642) → L447 %128 memcpy(23699) → L448 battle_ally_action extend(23714) |
| 126 | SmallActionAround(스택 %57/%60/%73 → 원소) | 0x80 | position_eval_purpose | w | with_purpose(L181/L193/L922) | 4 | OK | line_phase_position_eval_purpose(player,data) 반환 i8 (22274, 22388, 22626) |
| 127 | SmallActionAround(스택 %72 → 원소) | 0x81 | escape_mode | w | L924 AroundRunAway 전용 — Around(L180/L192)는 new 가 넣은 값 그대로 | 4 | OK | 1 (22636) |
| 128 | SmallActionPlay 원소 | 0xb1 | tag | w | Option<SmallActionPlay> None=255 는 attack_tower_action 반환(%65+177)용 — extend 시 len 0/1 로 환산(23466~23467) | 4 | 확인불가(tcx 사전에 타입 없음) | 3 RunAway(22278? no → 23595, 22699) · 5 Around(22278, 22392) · 8 AroundRunAway(22638) · 15 Attack(23416 phi→23418) · -1 None(22934, 23418) |
| 129 | prof 전역 | PHASE_NANOS/PHASE_CALLS[phase] | 계측 카운터(atomicrmw add) | w | gc::simulation::prof::ENABLED 가 0 이 아닐 때만(Instant::now/elapsed). 판정 무관 텔레메트리 | 4 | 확인불가(오프셋 파싱 실패) | phase 87(+696, L910) · 64(L198) · 95(L933) · 89(+712, L394) · 92(L441 배치 F 에서 drop) |
| 130 | act_actions(%122 로컬 Vec) | 0x0..0x20 |  | w | 배치 F 로 전달 | 4 | 확인불가(tcx 사전에 타입 없음) | 552행 collect · 556/736/742 retain(len 감소·원소 memmove) · 760 truncate(0) · 761 push(choose clone) |
| 131 | move_actions(%110 로컬 Vec) | 0x0..0x20 |  | w | 배치 F 로 전달 | 4 | 확인불가(tcx 사전에 타입 없음) | get_move_action_v46 res(%44) memcpy 32B (227행 조기반환 / 365행) |
| 132 | rnd(&mut StdRng) | - |  | w | 호출 횟수는 분기·near_enemies 수·act_actions 길이에 의존 → 재현 시 호출 순서 동일 필수 | 4 | 확인불가(오프셋 파싱 실패) | gen_range·choose·range_misjudge_roll·Around::new·check_kill_die_tick·lane_minion_position_action·score 호출을 통한 상태 전진 |
| 133 | debug | 0xa0 |  | w | ctx.debug(+0x3b) 참일 때만 · 347행 | 4 | 확인불가(★모호: 동명 def_path 4개 [('game_core::DebugE) | entry(champ.id).or_insert(Vec::new()).push(format!(anon.121, has_enemy_minion_in_range)) |
| 134 | static prof::PHASE_NANOS / PHASE_CALLS | phase*8 |  | w | prof::ENABLED 가 0 이 아닐 때만 · 판정 무관 | 4 | 확인불가(tcx 사전에 타입 없음) | atomicrmw add (phase 93·94·83·81) |
| 135 | self | - |  | w | 내 범위에서 self 에 store 0건 | 4 | 확인불가(오프셋 파싱 실패) | (없음) |
| 136 | DebugFrameData(debug %8) | 0xa0 | infos[champ.id] (Vec<String>) | w | L779~780, context.debug==true && v47_soak_hold 일 때만 (m14.ll:27209~27270 · entry→or_insert(vec![])→push_mut). 순수 텔레메트리 | 4 | OK | push(format!("v47 siege soak hold: tower {}", nearest_tower.id)) |
| 137 | ret(%122, 지역 Vec) | 0x18 | len | w | L776 truncate(0) (27130) — nearest_tower 가 나를 물고 있고 v47_soak_hold 아닐 때. 배치 D/E 가 채운 후보 전량 폐기 | 4 | 확인불가(★모호: 동명 def_path 9개 [('game_core::Retire) | 0 |
| 138 | ret(%122) | 0x18 | len | w | L788 retain<s8_0> (27291, aux m01.ll:13653) — unsafe_v19_non_champion_walkup==true 인 원소 제거 | 4 | 확인불가(★모호: 동명 def_path 9개 [('game_core::Retire) | retain 결과 |
| 139 | move_actions(%110, 지역 Vec · 배치 E L766 산출) | 0x18 | len | w | L777 truncate(0) (27141) | 4 | 확인불가(tcx 사전에 타입 없음) | 0 |
| 140 | move_actions(%110) | ptr[len] | push(SmallActionPlay::RunAway) | w | L778 (27147~27200): 136B memcpy → +177 store i8 3 → reserve → 184B memcpy → len+1 | 4 | 확인불가(오프셋 파싱 실패) | SmallActionRunAway::new_with_skill(data, player, 5, true) · 태그 3 @+0xb1 |
| 141 | move_actions(%110) | ptr[len..] | extend(battle_actions) | w | L812 (27420~27422) | 4 | 확인불가(오프셋 파싱 실패) | fight_check::battle_action(version,rnd,player,data,5) 결과(v30 retain 후, 비어 있지 않을 때) |
| 142 | sret %0 | 0x0 | Vec<SmallActionPlay> 32B | w | L876 memcpy(27310) · L833(28392) · L863(28286) · L865(28270) · L868(28296) · L871(28327) | 4 | 확인불가(tcx 사전에 타입 없음) | 6 경로 (signature.returns 참조) |
| 143 | rnd(%3, &mut StdRng) | - | RNG 상태 | w | L825 range_misjudge_roll(27523) · L809 battle_action · L788/L837/L853 콜리 — 배치 F 직접 store 없음 | 4 | 확인불가(오프셋 파싱 실패) | 콜리 소유 |
| 144 | @PHASE_NANOS / @PHASE_CALLS (game_core::simulation::prof 전역) | [95]·[96]·[97] | 프로파일 누적 | w | prof::ENABLED 일 때만. 97 = L836 _t_mb(27738) 드롭 L838(27808~27812) · 96 = L852 _t_mp(28142) 드롭 L854(28196~28199) · 95 = 함수 전체 타이머(배치 D L441) 드롭 L879(28501~28503). 텔레메트리 | 4 | 확인불가(tcx 사전에 타입 없음) | atomicrmw add |
| 145 | self(%1) | - | - | w | ★배치 F 범위에서 self 쓰기 0건 (store to %1 없음 · 콜리 unsafe_v19/score 는 %0 readonly) | 4 | 확인불가(오프셋 파싱 실패) | - |

**`consts` 상수 73건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev | folded_from |
|---|---|---|---|---|---|---|
| 0 | 5 | 907 | 태그 | end_delay=5 — RunAway::new_with_skill(L907)·RunAway::new(L926)·Around::new(L180/L192/L921)·lane_minion_position_action(L183)·battle_action(L928)·battle_ally_action(L448) 공통 5번째/6번째 인자(DI 이름 end_delay / _end_delay) ⚠QC shl 경고 사유: 본문의 `shl i8 %line, 5`(21881/22449)는 line*32(top_tower..bottom_tower2 배열 stride)이며 이 상수와 무관 — 시프트량 아님 | 4 |  |
| 1 | 13 | 890 | 태그 | EntityType::Champion 메모리태그 13(tcxdict --enum EntityType) — 적 챔피언 루프 술어 `c.ty is Champion` | 3 |  |
| 2 | 4 | 891 | 태그 | ChampionActionState::Skill 태그 4 — is_in_skill switch case → skill_effect 사용 | 4 |  |
| 3 | 6 | 895 | 태그 | ChampionActionState::Ult 태그 6 — is_in_skill switch case → ult_effect() | 4 |  |
| 4 | -1 | 891 | 센티널 | Option<Effect> 니치 None(casting@tag=-1) — skill_effect.as_ref().unwrap() 실패 경로(21660, 21689, 21712 는 case -1 → unwrap_failed). 또 L380 attack_effect None(23291) · Option<SmallActionPlay> None 태그(22934/23416/23466) | 4 |  |
| 5 | 1 | 891 | 태그 | CastingType::Position 태그 1 — is_in_skill 이 Some 을 주는 casting (1\|2). 또 L924 escape_mode=true(22636) ⚠QC shl 경고 사유: 본문 `shl … 1` 은 다른 배치/aux 의 *2 접힘이며 이 상수(태그·bool)와 무관 | 4 |  |
| 6 | 2 | 893 | 태그 | CastingType::Direction 태그 2(21662/21691/21714) · L893 skill2_effect(): level>2 게이트(21682) · L886 team 바운드 2(21512) | 4 |  |
| 7 | 4 | 895 | 태그 | L895 ult_effect(): level>4 게이트(21705). (Skill 태그 4 와 별개 관측) | 4 |  |
| 8 | 0 | 376 | 임계 | Entity::can_target(): block_target_tick==0 (23093, 23205, 23877 + aux) · L383 radius_mult==0 이면 보정 생략(23345, 23368) | 4 |  |
| 9 | 30 | 384 | 계수 | max_dist += move_speed*30 (23389) — 타워 공격 후보 사거리에 이속 30틱분(60tps 가정 0.5초)을 더한다. 리터럴 30 그대로(접힘 아님) | 4 |  |
| 10 | 100 | 383 | 계수 | Entity::radius(): radius*(radius_mult+100)/100 (23358~23360, 23381~23383) — 퍼센트 보정 | 4 |  |
| 11 | 3 | 907 | 태그 | SmallActionPlay::RunAway 메모리태그 3 (23595 L907 · 22699 L926) ⚠QC shl 경고 사유: 본문 `shl nuw nsw i64 %len, 3`(21910/22475)은 슬라이스 바이트 stride(*8)이며 이 태그값과 무관 | 4 |  |
| 12 | 5 | 180 | 태그 | SmallActionPlay::Around 메모리태그 5 (22392 L180 · 22278 L192) — end_delay 5 와 값이 같아 본문 리터럴이 겹친다 ⚠QC shl 경고 사유: 본문의 `shl i8 %line, 5`(21881/22449)는 line*32(top_tower..bottom_tower2 배열 stride)이며 이 상수와 무관 — 시프트량 아님 | 4 |  |
| 13 | 8 | 924 | 태그 | SmallActionPlay::AroundRunAway 메모리태그 8 (22638) | 4 |  |
| 14 | 15 | 390 | 태그 | SmallActionPlay::Attack 메모리태그 15 (23416 phi) | 4 |  |
| 15 | 87 | 885 | 산출값 | prof phase id 87(ProfTimer::start, 21518) — 계측 전용, 판정 무관. PHASE_NANOS+696 = 87*8 | 4 |  |
| 16 | 64 | 156 | 산출값 | prof phase id 64 (base_positioning, 21840) — 계측 전용 | 4 |  |
| 17 | 95 | 931 | 산출값 | prof phase id 95 (attack_summon_action 구간, 22798) — 계측 전용 | 4 |  |
| 18 | 89 | 373 | 산출값 | prof phase id 89 (attack_tower_action, 22901; PHASE_NANOS+712=89*8) — 계측 전용 | 4 |  |
| 19 | 92 | 441 | 산출값 | prof phase id 92 (action_candidates 전체 _t_ld, 21439) — 계측 전용 | 4 |  |
| 20 | 2 | 461 | 태그 | level > 2 → skill2_effect 존재(아니면 EMPTY 정적 Option=None). 213·266·closure#5 619 동일. 또 EntityType::Tower 태그(478/487/509/531 target.is_tower) · CastingType::Direction(211~215) | 4 |  |
| 21 | 4 | 462 | 태그 | level > 4 → ult_effect 존재. 215·267·closure#5 676 동일. 또 ChampionActionState::Skill 태그(211) | 4 |  |
| 22 | 13 | 468 | 태그 | SmallActionPlay 메모리태그 13 = LaneMinionPosition → closure#3 에서 즉시 false(제외). 또 EntityType::Champion 태그(472 is_champion · 490/512/534 · gmav46 210) | 4 |  |
| 23 | 5 | 226 | 태그 | 생성자 end_delay=5 (RunAway::new_with_skill(data,player,5,…)/Trace::new(data,id,5)/Trace::new_keep_range(data,id,5,r)/Around::new(version,rnd,data,player,id,5) · DILocalVariable 확인: Trace::new_keep_range(data,target,end_delay,keep_range) · RunAway::new_with_skill(data,player,end_delay,with_skill) · Around::new(_version,_rnd,data,_player,target,end_delay) — Around 는 version/rnd/player 를 안 쓴다(밑줄 접두)). 또 ChampionActionState::Skill2 태그(213) · SmallActionPlay Around 태그(353·360) · ⚠본문의 `shl nuw nsw i8 %line, 5`(230행 line*32 = 타워 배열 stride 접힘) 는 이 상수의 용법이 아님 | 4 |  |
| 24 | 6 | 215 | 태그 | ChampionActionState::Ult 태그 | 4 |  |
| 25 | 1 | 290 | 임계 | me_die_tick >= tps*2 — `shl i64 %tps, 1` 로 접힘(2초). 또 264행 near_ally_count+1 · CastingType::Position(211~215) · TeamType::Neutral discr | 4 | 2 |
| 26 | 3 | 226 | 태그 | SmallActionPlay 메모리태그 3 = RunAway (226·336 push). 또 closure#3/5/7 태그→논리인덱스 변환 `tag-3`(idx = tag>2 ? tag-3 : 7) · ⚠본문의 `shl i64 %len, 3`(231/579행 포인터 배열 stride 접힘) 는 이 상수의 용법이 아님 | 4 |  |
| 27 | 14 | 298 | 태그 | SmallActionPlay 메모리태그 14 = Trace (298·302·306·317 push) | 4 |  |
| 28 | 15000 | 495 | 미상 | closure#3 중첩: 적 타워 attack_effect.is_in_range_ex(tower,target,tower.xy,target.xy, 15000) 여유값 — 타워가 target 을 (15000 여유로) 못 덮을 때만 스킬 유지 | 4 |  |
| 29 | 30000 | 342 | 미상 | gmav46 341~344: champ_atk.is_in_range_ex(champ, m\|t, …, 30000) — 적 미니언/타워가 평타 사거리+30000 안 → has_enemy_minion_in_range | 4 |  |
| 30 | 1000 | 757 | 계수 | rnd.gen_range(0..1000) > judgement_line_accuracy (757 · closure#7 743). 또 271/272/273/278/313 `범위*roll/1000`(오판 롤 천분율) | 4 |  |
| 31 | 2000 | 256 | 계수 | max_v = 2000 - positioning_accuracy (range_misjudge_roll 의 상한; min_v = positioning_accuracy) | 4 |  |
| 32 | 40 | 273 | 태그 | emr_near = max_range_nearly_can_use(enemy, champ, 40)*roll/1000 | 4 |  |
| 33 | 60 | 313 | 미상 | me_die_tick < tps 분기의 emr = max_range_nearly_can_use(enemy, champ, 60)*roll/1000 | 4 |  |
| 34 | 10 | 296 | 임계 | enemy.remain_action_time() > 10 (틱) | 4 |  |
| 35 | 999 | 296 | 임계 | `mr_raw(=roll*max_range_can_use) > 999` ⟺ mr(= mr_raw/1000) >= 1 ⟺ mr != 0 — udiv 비교가 접힘 | 4 |  |
| 36 | 25600000000 | 258 | 임계 | 160000² — near_enemies(258: dist² < 160000², 가시 적 챔피언) · near_ally_count(263: id≠champ.id && dist² < 160000²) | 4 |  |
| 37 | 14400000001 | 577 | 미상 | 120000²+1 → dist² <= 120000² — closure#5 의 「target 근처 적 존재」 판정(577/578/579/580 · 592/593/594/597~598 · Skill2/Ult 동일) | 4 |  |
| 38 | 79 | 569 | 미상 | closure#5: target hp_ratio(hp*100/max_hp) > 79 (=80% 이상) 이면 힐 스킬은 aoe_heal_covers_low_ally 가 참일 때만 유지 | 4 |  |
| 39 | 100 | 568 | 계수 | hp_ratio = hp*100/max_hp. 또 327 radius*(mult+100)/100 | 4 |  |
| 40 | -31 | 753 | 임계 | closure#7: score > -31 ⟺ score >= -30 (비아군 대상 · 비게이트 유지 임계) | 4 |  |
| 41 | -1 | 753 | 센티널 | closure#7: score > -1 ⟺ score >= 0 (아군 대상 스킬 유지 임계). 또 Option 니치 None(-1) 검사 다수(casting·tag@0xb1·expected_buff+0x48) | 4 |  |
| 42 | 0 | 292 | 태그 | bruiser_engage = expected_trade_net_hp(data,champ,enemy) > 0. 또 VisibleState::Visible 태그 0 · 760 truncate(0) | 4 |  |
| 43 | 12 | 473 | 태그 | closure#3 switch 논리인덱스 12/13/14/15 = Attack/Skill/Skill2/Ult(태그 15~18) 만 통과, 그 외 인덱스(0~11·16) → false | 4 |  |
| 44 | -15 | 472 | 태그 | `tag-15 <u 4` ⟺ tag∈{15,16,17,18}(Attack/Skill/Skill2/Ult) — get_action().target_id 존재 variant | 4 |  |
| 45 | -16 | 744 | 인덱스 | closure#7 `tag-16 <u 3` ⟺ tag∈{16,17,18}(Skill/Skill2/Ult) — 아군 대상 여부 검사 대상. closure#5 557 은 switch 13/14/15(논리인덱스) | 4 |  |
| 46 | 93 | 465 | 산출값 | ProfTimer phase 93(_t_af, 465~553) · 94(_t_ar, 555~732) · 83(_t_sr, 735~765) · 81(get_move_action_v46 202~366) — prof::ENABLED 일 때만, 판정 무관 | 4 |  |
| 47 | 132 | 553 | 길이 | PHASE_NANOS/PHASE_CALLS 배열 길이 bounds check(프로파일링) — 판정 무관 | 4 |  |
| 48 | 1000000000 | 553 | 임계 | Duration → ns 환산(프로파일링) — 판정 무관 | 4 |  |
| 49 | 2 | 770 | 태그 | EntityType::Tower 메모리태그(tcxdict --enum EntityType) — nearest_tower.ty 판별 (m14.ll:26999) | 3 |  |
| 50 | 5 | 778 | 태그 | SmallActionRunAway::new_with_skill 3번째 인자 end_delay=5 (L778 with_skill=true · L833 · L863 with_skill=false 모두 5) · L809 battle_action 5번째 인자도 5(의미는 battle_action 계약 — 미탐색) ※shl 피연산자 아님(호출 인자 리터럴 27147 `i64 5`) — QC 경고는 함수 다른 범위의 shl 5 와 값이 겹친 것 | 4 |  |
| 51 | 3 | 778 | 태그 | SmallActionPlay::RunAway 메모리태그 (store i8 3 @+177: 27153 · 28391 · 28285) ※shl 아님(store i8 3) | 4 |  |
| 52 | -3 | 787 | 태그 | has_runaway: 태그-3 로 정규화 (27043). 접힌 matches!(a, RunAway\|Recall\|AroundRunAway) | 4 |  |
| 53 | 0 | 787 | 태그 | switch case 0 = 태그 3 RunAway → has_runaway=true (27047). tag<=2 는 select 7(AroundPosition 암묵) 로 보내 false | 4 | 3 |
| 54 | 1 | 787 | 태그 | switch case 1 = 태그 4 Recall → true (27048) | 4 | 4 |
| 55 | 5 | 787 | 태그 | switch case 5 = 태그 8 AroundRunAway → true (27052) | 4 | 8 |
| 56 | 59 | 779 | 미상 | GameContext+0x3b debug bool 오프셋(10진) — 판정값 아님, gep 상수 (27134) | 4 |  |
| 57 | 2000 | 819 | 계수 | max_v = 2000 - positioning_accuracy (27431). positioning_accuracy ∈[10,1000](g15.ll:126073 range) → 롤 구간 [acc, 2000-acc] 을 1000 기준 대칭으로 | 4 |  |
| 58 | 1000 | 825 | 계수 | emr = max_range_can_use(c,champ) * roll / 1000 + 10000 — 롤(퍼밀) 스케일 (27528) | 4 |  |
| 59 | 10000 | 825 | 미상 | emr 에 더하는 절대 여유 10000 (≈0.31셀) (27529) | 4 |  |
| 60 | 13 | 830 | 태그 | EntityType::Champion 태그 — is_in_action 인라인 (27580) | 4 |  |
| 61 | 3 | 830 | 태그 | ChampionActionState 태그 3=Attack 이상(Attack/Skill/Skill2/Ult)이면 행동중 (`ult 3` 27583) ※shl 아님(icmp samesign ult %x, 3) | 4 |  |
| 62 | 0 | 832 | 임계 | mr(max_range_can_use(champ,c)) != 0 이면 도주 안 함 — 내가 닿을 수 있는 적이면 skip (27592) | 4 |  |
| 63 | 97 | 836 | 산출값 | prof phase id (_t_mb, max_by_key 구간) (27738) | 4 |  |
| 64 | 96 | 852 | 산출값 | prof phase id (_t_mp, get_input 구간) (28142) | 4 |  |
| 65 | 528 | 845 | 미상 | AbstractGame vtable+0x210 = iter_projectile 슬롯 (divtable) (27824) | 3 |  |
| 66 | 176400000000 | 847 | 임계 | 420000² — 비타겟 적 투사체가 내 위치 420000(≈13.1셀) 안이면 trajectory_possible (27926 `ult`) | 4 |  |
| 67 | -9223372036854775805 | 849 | 태그 | RushState::Rush 메모리태그(niche_start 2^63 + 3). `sgt -1`(=untagged RushPenetrate) 와 or → 적 챔피언이 돌진 중 (27963~27965) | 4 |  |
| 68 | -1 | 857 | 태그 | Option<Input>::None 태그(get_input m11.ll:42866 from_residual) → 입력 없으면 L871 (28212) | 4 |  |
| 69 | 0 | 858 | 태그 | Input::Move 태그 0 (tcxdict --enum Input) (28224) | 3 |  |
| 70 | 2500000001 | 796 | 미상 | aux closure$11: dist²(t,champ) < 50000²+1 = 50000 이내 아군 타워만 (m06.ll:32322) | 4 |  |
| 71 | -1 | 798 | 임계 | aux closure$12: attack_effect@tag(4B) == -1 → None → tower_attack_range=0 (m06.ll:32347) | 4 |  |
| 72 | 100 | 799 | 계수 | aux Entity::radius() 인라인: radius*(radius_mult+100)/100 (m06.ll:32383~32385, 32483~32485) | 4 |  |

**`knobs` 조정점 24건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 행동 end_delay | line_defense.rs:907·926·180·192·921·183·928·448 (리터럴 5, 8곳) | 5 | 올리면 라인수비 후보(RunAway/Around/AroundRunAway/LaneMinionPosition/battle 계열)의 지속 틱이 늘어 재평가가 드물어지고, 내리면 매 틱에 가깝게 후보를 다시 고른다(콜리가 end_delay 를 어떻게 소비하는지는 각 생성자 계약 참조) | 4 | 기존 |
| 1 | 적 타워 공격 후보 이속 여유 | line_defense.rs:384 (m14.ll:23389) | 30 | 올리면 더 먼(이속×틱 만큼) 적 타워도 공격 후보(Attack)에 오르고 v22 게이트로 넘어간다 · 내리면 사거리 안 타워만 후보 | 4 | 기존 |
| 2 | 스킬2/궁 존재 레벨 게이트(적 논타겟 스킬 사거리 판정) | entity.rs:1693(>2) · 1701(>4), L893/L895 인라인 | 2 | 게임코어 규칙(레벨 3 부터 skill2, 5 부터 ult). 여기서 바꾸면 적 챔피언의 논타겟 스킬 인지가 어긋난다 — 사실상 코어 상수, 개조 대상 아님 | 4 | 기존 |
| 3 | 조기 RunAway 트리거 집합 | line_defense.rs:905 | on_trajectory \|\| has_non_target_action_range \|\| on_periodic_trajectory | 항을 빼면(예: on_periodic_trajectory) 주기 투사체 궤적 위에서도 RunAway 로 조기 귀환하지 않고 정상 후보 생성으로 내려간다. 항을 더하면 후보 생성 자체를 건너뛰는 틱이 늘어난다(RunAway 단독 후보) | 4 | 기존 |
| 4 | 기본 포지셔닝 분기 기준 | line_defense.rs:179 | dist²(front_minion, nexus) < dist²(nearest_tower, nexus) | 부등호를 <= 로 바꾸면 동거리 때 Around(타워) 대신 lane_minion_position_action 이 선택되지 않는 방향으로 바뀐다(현재는 동거리면 미니언 라인 포지셔닝). 비교 대상을 바꾸면 '타워 뒤 미니언' 판정이 통째로 바뀐다 | 4 | 기존 |
| 5 | 실드/버프 스킬 유지용 「target 근처 적」 반경 | line_defense.rs:577~598 (closure#5, m01.ll) | 14400000001 | 120000 을 올리면 더 먼 적이 있어도 아군 대상 실드/버프 스킬이 이동 중 후보로 남는다(과잉 시전↑) · 내리면 접촉 직전에만 남는다 | 4 | 기존 |
| 6 | 힐 스킬 유지 HP 비율 임계 | line_defense.rs:569 | 79 | 올리면(예 89) 체력 90% 아군에게도 힐 후보가 남는다(힐 낭비↑) · 내리면 저체력에만 힐 | 4 | 기존 |
| 7 | near_enemies / near_ally 반경 | line_defense.rs:258·263 | 25600000000 | 160000 을 올리면 더 먼 적/아군이 교전 인원 계산에 들어가 local_outnumbered·me_die_tick 이 바뀐다 | 4 | 기존 |
| 8 | 판단 정확도 게이트 롤 상한 | line_defense.rs:743·757 | 1000 | 천분율. judgement_line_accuracy(선수 능력치) 가 1000 이면 게이트 절대 안 걸림 · 상한을 올리면 능력치 대비 오판(무작위 1개만 남김·아군스킬 점수 검사 생략) 빈도↑ | 4 | 기존 |
| 9 | 오판 롤 범위 상한 기준 | line_defense.rs:256 | 2000 | max_v = 2000 - positioning_accuracy. 올리면 사거리 오판 진폭↑(mr/emr 왜곡↑) | 4 | 기존 |
| 10 | 적 타워 커버 여유(스킬 유지) | line_defense.rs:495·517·539 | 15000 | 올리면 타워 사거리 밖이어도 여유 안이면 스킬을 버린다(타워 근처 스킬 사용↓) | 4 | 기존 |
| 11 | 적 미니언/타워 사정권 여유(대기 후보 억제) | line_defense.rs:342·344 | 30000 | 올리면 더 먼 미니언/타워도 「사정권」으로 봐 도주 중 Around 후보가 더 자주 생략된다 | 4 | 기존 |
| 12 | 브루저 교전 조건 생존 시간 | line_defense.rs:290 | 1 | shl 1 = tps*2 (2초). 올리면 더 오래 버틸 수 있을 때만 bruiser_engage(추격 Trace) · 내리면 공격적 | 4 | 기존 |
| 13 | me_die_tick < tps 위기 분기 사거리 여유 | line_defense.rs:313 | 60 | 위기 시 적 사거리 nearly 여유. 올리면 위기 도주(runaway) 판정 범위↑ | 4 | 기존 |
| 14 | 평시 emr_near 여유 | line_defense.rs:273 | 40 | dist <= emr_near² 이면 runaway. 올리면 더 멀리서도 도주 | 4 | 기존 |
| 15 | 적 행동 잔여시간 임계 | line_defense.rs:296 | 10 | 적이 10틱 넘게 행동에 묶여 있으면 사거리 밖일 때 추격(Trace) 허용 | 4 | 기존 |
| 16 | 아군 대상 스킬 유지 점수 임계 / 비아군 임계 | line_defense.rs:753 | -31 | score >= -30 유지(비아군) · 아군 대상은 >= 0. -31 을 내리면(예 -51) 낮은 점수 행동도 남아 후보 다양성↑ | 4 | 기존 |
| 17 | 생성자 end_delay | line_defense.rs:226·298~317·336·353·360 | 5 | RunAway/Trace/Around 의 end_delay. 의미(틱 지연 vs 유지)는 자식 명세 소관 — 올리면 행동 유지/지연 길어짐(추정) | 5 | 기존 |
| 18 | RunAway 액션 end_delay | line_defense.rs:778 · 833 · 863 (m14.ll:27147, 27615, 28258) | 5 | SmallActionRunAway.end_delay(+0x18). 올리면 도주 액션의 종료 지연이 길어진다(정확한 소비처는 SmallActionRunAway 계약 — 미탐색) | 4 | 기존 |
| 19 | 적 사거리 오판 롤 구간 중심 | line_defense.rs:819 (27431) | 2000 | roll ∈ [acc, 2000-acc]/1000 배. 올리면 적 사거리를 평균적으로 더 크게 오판(더 멀리서 도주) · 내리면 과소평가(덜 도주) | 4 | 기존 |
| 20 | 적 사거리 절대 여유 | line_defense.rs:825 (27529) | 10000 | emr 에 더하는 상수. 올리면 적이 더 멀리 있어도 '사거리 안' 으로 보고 RunAway(with_skill=false) 를 반환(L833) | 4 | 기존 |
| 21 | 비타겟 투사체 궤적 위험 반경(제곱) | line_defense.rs:847 (27926) | 176400000000 | 420000². 올리면 더 먼 적 투사체도 trajectory_possible 로 보고 get_input→position_score 검증(L853~863)을 더 자주 수행 · 내리면 검증 생략(L871 바로 반환) | 4 | 기존 |
| 22 | 타워 아래 전투 후보 — 아군 타워 탐색 반경(제곱) | line_defense.rs:796 (aux m06.ll:32322) | 2500000001 | 50000²+1. 올리면 더 먼 아군 타워도 후보가 돼 그 타워 사거리 안 적이 있으면 battle_action 후보가 추가된다 | 4 | 기존 |
| 23 | has_runaway 판정 variant 집합 | line_defense.rs:787 (27046~27064) | 태그 {3,4,8} | RunAway/Recall/AroundRunAway 가 move_actions 에 있으면 unsafe_v19_non_champion_walkup 에 has_runaway=true 로 전달(그 안의 의미는 콜리 계약 — 미탐색) | 4 | 기존 |

<details><summary>`callees` 피호출자 122건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | action_candidates_old | game_ai::plan_legacy::sub_plan::EpicHuntSubPlan::action_candidates_old | pub | fn(&mut game_ai::plan_legacy::sub_plan::EpicHuntSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\epic_hunt.rs:678 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 1 | action_candidates_old | game_ai::plan_legacy::sub_plan::EpicPokeSubPlan::action_candidates_old | pub | fn(&mut game_ai::plan_legacy::sub_plan::EpicPokeSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\epic_poke.rs:426 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 2 | action_candidates_old | game_ai::plan_legacy::sub_plan::SerpenHuntSubPlan::action_candidates_old | pub | fn(&mut game_ai::plan_legacy::sub_plan::SerpenHuntSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\serpen_hunt.rs:677 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 3 | aoe_heal_covers_low_ally | game_ai::aoe_heal_covers_low_ally | pub | fn(usize, &game_core::Effect, &game_core::OperationData, &game_core::PlayerState, &game_core::Entity) -> bool | game-ai\src\buff_value.rs:543 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | as_index | game_core::BigGoal::as_index | pub | fn(&game_core::BigGoal) -> usize | game-core\src\simulation\ai_interface.rs:338 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 5 | as_index | game_core::Position::as_index | pub | fn(&game_core::Position) -> usize | game-core\src\simulation\entity.rs:580 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 6 | attack_effect | game_core::Entity::attack_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1684 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 7 | attack_structure_skill_action | game_ai::attack_structure_skill_action | pub | fn(&game_core::PlayerState, &game_core::OperationData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\fight_check.rs:794 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 8 | attack_summon_action | game_ai::attack_summon_action | pub | fn(&game_core::PlayerState, &game_core::OperationData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\fight_check.rs:836 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 9 | attack_tower_action | game_ai::plan_legacy::sub_plan::LineDefenseSubPlan::attack_tower_action | in:game_ai::plan_legacy::sub_plan::line_defense | fn(&mut game_ai::plan_legacy::sub_plan::LineDefenseSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> std::option::Option<game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\line_defense.rs:372 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 10 | base_positioning | game_ai::plan_legacy::sub_plan::JungleSubPlan::base_positioning | in:game_ai::plan_legacy::sub_plan::jungle | fn(&mut game_ai::plan_legacy::sub_plan::JungleSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> game_ai::SmallActionPlay | game-ai\src\plan_legacy\sub_plan\jungle.rs:19 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 11 | base_positioning | game_ai::plan_legacy::sub_plan::BattleSubPlan::base_positioning | in:game_ai::plan_legacy::sub_plan::battle | fn(&mut game_ai::plan_legacy::sub_plan::BattleSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\battle.rs:72 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 12 | base_positioning | game_ai::plan_legacy::sub_plan::LineSafeSubPlan::base_positioning | in:game_ai::plan_legacy::sub_plan::line_safe | fn(&game_ai::plan_legacy::sub_plan::LineSafeSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\line_safe.rs:15 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 13 | battle_action | game_ai::battle_action | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, usize) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\fight_check.rs:620 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 14 | battle_ally_action | game_ai::battle_ally_action | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, usize) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\fight_check.rs:1112 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 15 | block_input | game_core::Entity::block_input | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1501 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 16 | can_move | game_core::Entity::can_move | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1489 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 4개 중 상위 3개 |
| 17 | can_move | game_core::Champion::can_move | pub | fn(&game_core::Champion) -> bool | game-core\src\simulation\entity\champion.rs:48 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 4개 중 상위 3개 |
| 18 | can_move | game_core::EffectType::can_move | pub | fn(&Self/#0) -> bool | game-core\src\simulation\effect\type.rs:358 | True | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 4개 중 상위 3개 |
| 19 | can_skill | game_core::Entity::can_skill | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1708 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 20 | can_skill2 | game_core::Entity::can_skill2 | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1721 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 21 | can_target | game_core::Entity::can_target | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1477 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 22 | can_target | game_core::OperationData::<'a, 'b>::can_target | pub | fn(&game_core::OperationData<'a/#0, 'b/#1>, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\state\player.rs:23 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 23 | can_ult | game_core::Entity::can_ult | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1734 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 24 | check_kill_die_tick | game_ai::check_kill_die_tick | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::OperationData, &game_core::PlayerState, &game_core::Entity, bumpalo::collections::vec::Vec< &game_core::Entity>, bumpalo::collections::vec::Vec< &game_core::Entity>, &mut game_core::DebugFrameData) -> usize | game-ai\src\fight_check.rs:917 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 25 | debug | game_core::AbstractBanpickRunner::debug | pub | fn(&Self/#0, &[usize]) | game-core\src\banpick\base_runner.rs:411 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 26 | debug | <game_core::FactoredBanpickAgent as game_core::AbstractBanpickRunner>::debug | pub | fn(&game_core::FactoredBanpickAgent, &[usize]) | game-core\src\banpick\sgd_v2.rs:5891 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 27 | debug | <game_core::LogisticSGDBanpickAgent as game_core::AbstractBanpickRunner>::debug | pub | fn(&game_core::LogisticSGDBanpickAgent, &[usize]) | game-core\src\banpick\sgd.rs:1172 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 28 | effect_buff_target | game_ai::effect_buff_target | pub | fn(usize, &game_core::Effect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + , &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + ) -> std::option::Option<game_core::BuffState> | game-ai\src\fight_check.rs:390 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 29 | expected_buff | game_core::EffectType::expected_buff | pub | fn(&Self/#0, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + ) -> std::option::Option<game_core::BuffState> | game-core\src\simulation\effect\type.rs:287 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 18개 중 상위 3개 |
| 30 | expected_buff | <game_core::RangeEffect as game_core::EffectType>::expected_buff | pub | fn(&game_core::RangeEffect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + ) -> std::option::Option<game_core::BuffState> | game-core\src\simulation\effect\type\range_effect.rs:94 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 18개 중 상위 3개 |
| 31 | expected_buff | <game_core::AddBuffEffect as game_core::EffectType>::expected_buff | pub | fn(&game_core::AddBuffEffect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + ) -> std::option::Option<game_core::BuffState> | game-core\src\simulation\effect\type\add_buff.rs:28 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 18개 중 상위 3개 |
| 32 | expected_damage_target | game_core::Effect::expected_damage_target | pub | fn(&game_core::Effect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + , &game_core::Entity) -> usize | game-core\src\simulation\effect.rs:91 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 33 | expected_heal | game_ai::expected_heal | pub | fn(&game_core::GameContext, &game_core::Entity, &game_core::Entity) -> usize | game-ai\src\fight_check.rs:266 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 31개 중 상위 3개 |
| 34 | expected_heal | game_core::EffectType::expected_heal | pub | fn(&Self/#0, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + ) -> usize | game-core\src\simulation\effect\type.rs:283 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 31개 중 상위 3개 |
| 35 | expected_heal | <game_core::HealEffect as game_core::EffectType>::expected_heal | pub | fn(&game_core::HealEffect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + ) -> usize | game-core\src\simulation\effect\type\heal.rs:186 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 31개 중 상위 3개 |
| 36 | expected_shield | game_ai::expected_shield | pub | fn(&game_core::GameContext, &game_core::Entity, &game_core::Entity) -> usize | game-ai\src\fight_check.rs:284 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 22개 중 상위 3개 |
| 37 | expected_shield | game_core::EffectType::expected_shield | pub | fn(&Self/#0, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + ) -> usize | game-core\src\simulation\effect\type.rs:285 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 22개 중 상위 3개 |
| 38 | expected_shield | <game_core::RangeEffect as game_core::EffectType>::expected_shield | pub | fn(&game_core::RangeEffect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + ) -> usize | game-core\src\simulation\effect\type\range_effect.rs:90 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 22개 중 상위 3개 |
| 39 | expected_trade_net_hp | game_ai::plan_legacy::old::expected_trade_net_hp | pub | fn(&game_core::OperationData, &game_core::Entity, &game_core::Entity) -> i64 | game-ai\src\plan_legacy\old\battle.rs:2493 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 40 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 41 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 42 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 43 | get_input | game_ai::SmallActionPlay::get_input | pub | fn(&mut game_ai::SmallActionPlay, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &mut game_core::DebugFrameData) -> std::option::Option<game_core::Input> | game-ai\src\small_action.rs:157 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 44 | get_move_action_v46 | game_ai::plan_legacy::sub_plan::LineDefenseSubPlan::get_move_action_v46 | in:game_ai::plan_legacy::sub_plan::line_defense | fn(&game_ai::plan_legacy::sub_plan::LineDefenseSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, std::option::Option<&game_ai::MinionWaveSnapshot>, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\line_defense.rs:200 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 45 | get_start_position | game_core::LineType::get_start_position | pub | fn(&game_core::LineType, &game_core::GameSetting, usize) -> (u64, u64) | game-core\src\simulation\state\player.rs:1013 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 46 | is_champion | game_core::EntityType::is_champion | pub | fn(&game_core::EntityType) -> bool | game-core\src\simulation\entity.rs:1403 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 47 | is_dash_worth | game_ai::is_dash_worth | pub | fn(&game_core::OperationData, &game_core::PlayerState, &game_core::Entity, &game_core::Entity, usize) -> bool | game-ai\src\utils.rs:98 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 48 | is_empty | game_core::PatchSetting::is_empty | pub | fn(&game_core::PatchSetting) -> bool | game-core\src\patch_engine.rs:853 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 49 | is_empty | game_core::ModAiRegistry::is_empty | pub | fn(&game_core::ModAiRegistry) -> bool | game-core\src\mod_ai.rs:15 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 50 | is_empty | game_core::PublicChampionEvidence::is_empty | pub | fn(&game_core::PublicChampionEvidence) -> bool | game-core\src\banpick\public_evidence.rs:96 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 51 | is_in_action | game_core::Entity::is_in_action | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1547 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 52 | is_in_range | game_core::Effect::is_in_range | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\effect.rs:63 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 53 | is_in_range_ex | game_core::Effect::is_in_range_ex | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity, u64, u64, u64, u64, u64) -> bool | game-core\src\simulation\effect.rs:78 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 54 | is_in_skill | game_core::Entity::is_in_skill | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1570 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 55 | is_recent_visible | game_core::Blackboard::is_recent_visible | pub | fn(&game_core::Blackboard, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\game\blackboard.rs:346 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 56 | is_targeting | game_core::ProjectileMoveType::is_targeting | pub | fn(&game_core::ProjectileMoveType) -> bool | game-core\src\simulation\projectile.rs:133 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 57 | is_tower | game_core::EntityType::is_tower | pub | fn(&game_core::EntityType) -> bool | game-core\src\simulation\entity.rs:1385 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 58 | is_visible_from | game_core::Entity::is_visible_from | pub | fn(&game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1481 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 59 | iter_champions | game_core::AbstractGameWithCache::<'a, 'b>::iter_champions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5759 ~ game_core[6a30]::simulation::{impl#5}::iter_champions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1904 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 60 | iter_minions | game_core::AbstractGameWithCache::<'a, 'b>::iter_minions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5753 ~ game_core[6a30]::simulation::{impl#5}::iter_minions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1847 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 61 | iter_projectile | game_core::AbstractGame::iter_projectile | pub | fn(&Self/#0) -> game_core::ProjectileIter | game-core\src\simulation.rs:182 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 62 | iter_projectile | <game_core::Game as game_core::AbstractGame>::iter_projectile | pub | fn(&game_core::Game) -> game_core::ProjectileIter | game-core\src\simulation\game.rs:3760 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 63 | iter_projectile | <game_core::SingleLaneGame as game_core::AbstractGame>::iter_projectile | pub | fn(&game_core::SingleLaneGame) -> game_core::ProjectileIter | game-core\src\simulation\game.rs:4019 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 64 | iter_towers | game_core::AbstractGameWithCache::<'a, 'b>::iter_towers | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5762 ~ game_core[6a30]::simulation::{impl#5}::iter_towers::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1908 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 65 | iter_towers_without_nexus | game_core::AbstractGameWithCache::<'a, 'b>::iter_towers_without_nexus | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5750 ~ game_core[6a30]::simulation::{impl#5}::iter_towers_without_nexus::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1830 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 66 | judge_line_accuracy | game_core::AthleteParameter::judge_line_accuracy | pub | fn(&game_core::AthleteParameter) -> usize | game-core\src\simulation\state\player.rs:344 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 67 | keep | game_view::pixel_fx::Dither::keep | in:game_view::pixel_fx | fn(game_view::pixel_fx::Dither, i32, i32) -> bool | game-view\src\view\pixel_fx.rs:49 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 68 | keep | game_view::view::effect::alchemist::Dither::keep | in:game_view::view::effect::alchemist | fn(game_view::view::effect::alchemist::Dither, i32, i32) -> bool | game-view\src\view\effect\alchemist.rs:372 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 69 | keep | game_view::view::projectile::sand_mage::Dither::keep | in:game_view::view::projectile::sand_mage | fn(game_view::view::projectile::sand_mage::Dither, i32, i32) -> bool | game-view\src\view\projectile\sand_mage.rs:240 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 70 | lane_minion_position_action | game_ai::lane_minion_position_action | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::OperationData, &game_core::PlayerState, game_core::LineType, &game_core::PositioningScoreData, std::option::Option<&game_ai::MinionWaveSnapshot>, usize, game_ai::PositionEvalPurpose) -> std::option::Option<game_ai::SmallActionPlay> | game-ai\src\small_action\lane_minion.rs:20 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 71 | level | <game_ai::WindowStatView<'_> as game_core::AbstractEntity>::level | pub | fn(&game_ai::WindowStatView</#0>) -> usize | game-ai\src\fight_check.rs:96 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 72 | level | game_core::AbstractEntity::level | pub | fn(&Self/#0) -> usize | game-core\src\simulation\entity.rs:718 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 73 | level | <game_core::Entity as game_core::AbstractEntity>::level | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:772 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 74 | line | game_ai::plan_legacy::types::BigPlan::debug_label::line | in:game_ai::plan_legacy::types | fn(game_core::LineType) -> &str | game-ai\src\plan_legacy\types.rs:83 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 75 | line | game_core::TowerType::line | pub | fn(&game_core::TowerType) -> std::option::Option<game_core::LineType> | game-core\src\simulation\entity\tower.rs:90 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 76 | line_minion_action_candidates | game_ai::line_minion_action_candidates | pub | fn(usize, &game_core::OperationData, &game_core::PlayerState, game_core::LineType) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\small_action\lane_minion.rs:72 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 77 | line_phase_position_eval_purpose | game_ai::line_phase_position_eval_purpose | pub | fn(&game_core::PlayerState, &game_core::OperationData) -> game_ai::PositionEvalPurpose | game-ai\src\position_eval.rs:47 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 78 | max_hp | game_core::PlayerAiContext::<'a, 'b, 'r>::max_hp | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> std::option::Option<usize> | game-core\src\mod_ai.rs:608 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 79 | max_poke_range | game_ai::plan_legacy::old::max_poke_range | pub | fn(&game_core::Entity, &game_core::Entity, usize) -> u64 | game-ai\src\plan_legacy\old\battle.rs:2468 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 80 | max_range_can_use | game_ai::plan_legacy::old::max_range_can_use | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-ai\src\plan_legacy\old\battle.rs:2431 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 81 | max_range_nearly_can_use | game_ai::plan_legacy::old::max_range_nearly_can_use | pub | fn(&game_core::Entity, &game_core::Entity, usize) -> u64 | game-ai\src\plan_legacy\old\battle.rs:2397 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 82 | minion_state | game_core::Blackboard::minion_state | pub | fn(&game_core::Blackboard, game_core::LineType) -> &game_core::BrainMinionParameter | game-core\src\simulation\game\blackboard.rs:378 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 83 | new | game_ai::SmallActionTrace::new | pub | fn(&game_core::OperationData, usize, usize) -> game_ai::SmallActionTrace | game-ai\src\small_action\trace.rs:42 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 84 | new | game_ai::SmallActionAround::new | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::OperationData, &game_core::PlayerState, usize, usize) -> game_ai::SmallActionAround | game-ai\src\small_action\around.rs:23 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 85 | new | game_ai::SmallActionAttack::new | pub | fn(&game_core::OperationData, usize) -> game_ai::SmallActionAttack | game-ai\src\small_action\cast.rs:23 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 86 | new | game_ai::SmallActionRunAway::new | pub | fn(&game_core::OperationData, &game_core::PlayerState, usize) -> game_ai::SmallActionRunAway | game-ai\src\small_action\move_actions.rs:25 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 87 | new_keep_range | game_ai::SmallActionTrace::new_keep_range | pub | fn(&game_core::OperationData, usize, usize, u64) -> game_ai::SmallActionTrace | game-ai\src\small_action\trace.rs:69 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 88 | new_with_skill | game_ai::SmallActionRunAway::new_with_skill | pub | fn(&game_core::OperationData, &game_core::PlayerState, usize, bool) -> game_ai::SmallActionRunAway | game-ai\src\small_action\move_actions.rs:29 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 89 | nontarget_windup_perceived | game_ai::nontarget_windup_perceived | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity) -> bool | game-ai\src\utils.rs:553 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 90 | position_score_at_position | game_ai::position_score_at_position | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, u64, u64, game_ai::PositionEvalPurpose) -> game_core::PositioningScore | game-ai\src\position_eval.rs:1192 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 91 | positioning_accuracy | game_core::AthleteParameter::positioning_accuracy | pub | fn(&game_core::AthleteParameter) -> usize | game-core\src\simulation\state\player.rs:285 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 92 | push | <game_core::Staff as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::staff | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\staff.rs:38 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 93 | push | <game_core::Athlete as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:434 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 94 | push | <game_core::Contract as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:2498 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 95 | radius | game_core::Entity::radius | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1509 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 96 | range_adjust | game_core::Effect::range_adjust | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:29 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 97 | range_misjudge_rng | game_ai::range_misjudge_rng | pub | fn(usize, &game_core::OperationData, &game_core::PlayerState, usize) -> std::option::Option<game_core::NoiseRng> | game-ai\src\utils.rs:502 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 98 | range_misjudge_roll | game_ai::range_misjudge_roll | pub | fn(&mut rand::rngs::std::StdRng, &mut std::option::Option<game_core::NoiseRng>, u64, u64) -> u64 | game-ai\src\utils.rs:516 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 99 | remain_action_time | game_core::Entity::remain_action_time | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1616 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 100 | score | game_ai::plan_legacy::sub_plan::LineDefenseSubPlan::score | pub | fn(&game_ai::plan_legacy::sub_plan::LineDefenseSubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 | game-ai\src\plan_legacy\sub_plan\line_defense.rs:940 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 101 | skill2_effect | game_core::Entity::skill2_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1692 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 102 | skill_effect | game_core::Entity::skill_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1688 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 103 | start | game_core::prof::start | pub | fn(usize) -> std::option::Option<game_core::prof::ProfTimer> | game-core\src\simulation\prof.rs:175 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 104 | start | game_view::UIPhaseEffect::start | pub | fn(&mut game_view::UIPhaseEffect) | game-view\src\ui\match_ui\phase_effect.rs:30 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 105 | target_id | game_ai::SmallActionTrace::target_id | pub | fn(&game_ai::SmallActionTrace) -> usize | game-ai\src\small_action\trace.rs:427 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 106 | target_id | game_view::view::effect::alchemist::target_id | in:game_view::view::effect::alchemist | fn(game_core::InputTarget) -> std::option::Option<usize> | game-view\src\view\effect\alchemist.rs:126 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 107 | target_id | game_view::view::projectile::crossbowman::target_id | in:game_view::view::projectile::crossbowman | fn(game_core::InputTarget) -> std::option::Option<usize> | game-view\src\view\projectile\crossbowman.rs:1878 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 108 | team | game_view::ClientData::team | pub | fn(&game_view::ClientData, usize) -> std::option::Option<std::cell::Ref< game_core::Team>> | game-view\src\logic\client\data.rs:1651 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 109 | team | game_view::ClientDatabase::team | pub | fn(&game_view::ClientDatabase, usize) -> std::option::Option<&game_core::Team> | game-view\src\logic\client\data.rs:968 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 110 | team | game_core::PlayerAiContext::<'a, 'b, 'r>::team | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:583 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 111 | tower | game_core::AbstractGameWithCache::<'a, 'b>::tower | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, game_core::LineType, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:1822 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 112 | ult_effect | game_core::Entity::ult_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1700 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 113 | unsafe_v19_non_champion_walkup | game_ai::plan_legacy::sub_plan::LineDefenseSubPlan::unsafe_v19_non_champion_walkup | in:game_ai::plan_legacy::sub_plan::line_defense | fn(&game_ai::plan_legacy::sub_plan::LineDefenseSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_ai::SmallActionPlay, bool, &mut game_core::DebugFrameData) -> bool | game-ai\src\plan_legacy\sub_plan\line_defense.rs:72 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 114 | v22_lane_tower_pressure_attack_allowed | game_ai::v22_lane_tower_pressure_attack_allowed | pub | fn(usize, &game_core::OperationData, &game_core::PlayerState, &game_core::Entity) -> bool | game-ai\src\tower_discipline.rs:394 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 115 | v30_line_champion_action_tower_aggro_risk | game_ai::v30_line_champion_action_tower_aggro_risk | pub | fn(usize, &game_core::OperationData, &game_core::PlayerState, &game_ai::SmallActionPlay) -> bool | game-ai\src\tower_discipline.rs:158 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 116 | v47_siege_stance | game_ai::v47_siege_stance | pub | fn(usize, &game_core::OperationData, &game_core::PlayerState, &game_core::Entity) -> std::option::Option<usize> | game-ai\src\tower_discipline.rs:508 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 117 | version | <game_ai::AgentVerHamster as game_core::AiAgent>::version | pub | fn(&game_ai::AgentVerHamster) -> usize | game-ai\src\lib.rs:428 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 592개 중 상위 3개 |
| 118 | version | game_core::AiAgent::version | pub | fn(&Self/#0) -> usize | game-core\src\simulation\ai_interface.rs:503 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 592개 중 상위 3개 |
| 119 | version | <game_core::Team as engine_core::spitz::SpitzDatable>::version | pub | fn() -> usize | game-core\src\data\team.rs:1696 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 592개 중 상위 3개 |
| 120 | with | game_core::Athlete::with | pub | fn(&game_core::Athlete, usize) -> bool | game-core\src\data\athlete.rs:2299 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 121 | with | game_core::Contract::with | pub | fn(&game_core::Contract, usize) -> bool | game-core\src\data\athlete.rs:2813 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
</details>

⚠**미매칭 43개**: `action_state`, `bottom`, `casting`, `closure`, `drop  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `elapsed`, `enemy_champs`, `entry`, `escape_mode`, `extend`, `f_ult`, `format_inner`, `front_minion`, `gen_range`, `grow_one`, `has_moving_skill`, `insert_no_grow`, `is_none_or`, `move_actions`, `move_speed`, `mult`, `next  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 9개는 **전부 다른 함수**라 싣지 않는다`, `old_actions`, `on_periodic_trajectory`, `on_trajectory`, `or_insert`, `parameter`, `pool`, `positioning_score`, `push_mut`, `range  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `reserve_internal_or_panic`, `retain`, `rustc_entry`, `skip`, `team_plan`, `top_tower`, `top_tower2`, `trace_action`, `truncate`, `try_fold`, `void`, `with_purpose`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m12.ll:35184) · **형제 17개** (LineDefenseSubPlan)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::plan_legacy::sub_plan::LineDefenseSubPlan as std::clone::Clone>::clone | pub | game-ai\src\plan_legacy\sub_plan\line_defense.rs:7 | True | fn(&game_ai::plan_legacy::sub_plan::LineDefenseSubPlan) -> game_ai::plan_legacy::sub_plan::LineDefenseSubPlan |
| 1 | <game_ai::plan_legacy::sub_plan::LineDefenseSubPlan as std::fmt::Debug>::fmt | pub | game-ai\src\plan_legacy\sub_plan\line_defense.rs:7 | True | fn(&game_ai::plan_legacy::sub_plan::LineDefenseSubPlan, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 2 | game_ai::plan_legacy::sub_plan::LineDefenseSubPlan::new | pub | game-ai\src\plan_legacy\sub_plan\line_defense.rs:15 | True | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, game_core::LineType, game_ai::MinionActionType, game_core::LineStyle) -> game_ai::plan_legacy::sub_plan::LineDefenseSubPlan |
| 3 | game_ai::plan_legacy::sub_plan::LineDefenseSubPlan::new_for_gank | pub | game-ai\src\plan_legacy\sub_plan\line_defense.rs:20 | True | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, game_core::LineType, game_ai::MinionActionType, game_core::LineStyle) -> game_ai::plan_legacy::sub_plan::LineDefenseSubPlan |
| 4 | game_ai::plan_legacy::sub_plan::LineDefenseSubPlan::action_effect | in:game_ai::plan_legacy::sub_plan::line_defense | game-ai\src\plan_legacy\sub_plan\line_defense.rs:24 | False | fn(&game_core::Entity, game_core::SmallAction) -> std::option::Option<&game_core::Effect> |
| 5 | game_ai::plan_legacy::sub_plan::LineDefenseSubPlan::action_is_currently_in_range | in:game_ai::plan_legacy::sub_plan::line_defense | game-ai\src\plan_legacy\sub_plan\line_defense.rs:34 | False | fn(&game_core::Entity, game_core::SmallAction, &game_core::Entity) -> bool |
| 6 | game_ai::plan_legacy::sub_plan::LineDefenseSubPlan::action_walkup_position | in:game_ai::plan_legacy::sub_plan::line_defense | game-ai\src\plan_legacy\sub_plan\line_defense.rs:38 | False | fn(&game_ai::plan_legacy::sub_plan::LineDefenseSubPlan, &game_core::Entity, &game_core::Entity, game_core::SmallAction, &game_core::Effect, &game_core::OperationData) -> std::option::Option<(u64, u64)> |
| 7 | game_ai::plan_legacy::sub_plan::LineDefenseSubPlan::positioning_score_at | in:game_ai::plan_legacy::sub_plan::line_defense | game-ai\src\plan_legacy\sub_plan\line_defense.rs:68 | False | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, u64, u64) -> game_core::PositioningScore |
| 8 | game_ai::plan_legacy::sub_plan::LineDefenseSubPlan::unsafe_v19_non_champion_walkup | in:game_ai::plan_legacy::sub_plan::line_defense | game-ai\src\plan_legacy\sub_plan\line_defense.rs:72 | False | fn(&game_ai::plan_legacy::sub_plan::LineDefenseSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_ai::SmallActionPlay, bool, &mut game_core::DebugFrameData) -> bool |
| 9 | game_ai::plan_legacy::sub_plan::LineDefenseSubPlan::base_positioning | in:game_ai::plan_legacy::sub_plan::line_defense | game-ai\src\plan_legacy\sub_plan\line_defense.rs:154 | False | fn(&game_ai::plan_legacy::sub_plan::LineDefenseSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, std::option::Option<&game_ai::MinionWaveSnapshot>) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 10 | game_ai::plan_legacy::sub_plan::LineDefenseSubPlan::get_move_action_v46 | in:game_ai::plan_legacy::sub_plan::line_defense | game-ai\src\plan_legacy\sub_plan\line_defense.rs:200 | False | fn(&game_ai::plan_legacy::sub_plan::LineDefenseSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, std::option::Option<&game_ai::MinionWaveSnapshot>, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 11 | game_ai::plan_legacy::sub_plan::LineDefenseSubPlan::attack_minion_action | in:game_ai::plan_legacy::sub_plan::line_defense | game-ai\src\plan_legacy\sub_plan\line_defense.rs:368 | False | fn(&mut game_ai::plan_legacy::sub_plan::LineDefenseSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 12 | game_ai::plan_legacy::sub_plan::LineDefenseSubPlan::attack_tower_action | in:game_ai::plan_legacy::sub_plan::line_defense | game-ai\src\plan_legacy\sub_plan\line_defense.rs:372 | False | fn(&mut game_ai::plan_legacy::sub_plan::LineDefenseSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> std::option::Option<game_ai::SmallActionPlay> |
| 13 | game_ai::plan_legacy::sub_plan::LineDefenseSubPlan::calculate_score_parameter_value | pub | game-ai\src\plan_legacy\sub_plan\line_defense.rs:396 | False | fn(&game_ai::plan_legacy::sub_plan::LineDefenseSubPlan, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::ScoreParameter) |
| 14 | game_ai::plan_legacy::sub_plan::LineDefenseSubPlan::action_candidates | pub | game-ai\src\plan_legacy\sub_plan\line_defense.rs:440 | False | fn(&mut game_ai::plan_legacy::sub_plan::LineDefenseSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 15 | game_ai::plan_legacy::sub_plan::LineDefenseSubPlan::action_candidates_old | in:game_ai::plan_legacy::sub_plan::line_defense | game-ai\src\plan_legacy\sub_plan\line_defense.rs:881 | False | fn(&mut game_ai::plan_legacy::sub_plan::LineDefenseSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 16 | game_ai::plan_legacy::sub_plan::LineDefenseSubPlan::score | pub | game-ai\src\plan_legacy\sub_plan\line_defense.rs:940 | False | fn(&game_ai::plan_legacy::sub_plan::LineDefenseSubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 |

**`open` 24건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 표기 불가 | (배치 D) L905 세 조건(on_trajectory / has_non_target_action_range / on_periodic_trajectory)의 소스 표기 순서 — column 정보 부재. IR(21753~21754)은 +0x31\|has_non_target 을 먼저 or 하고 +0x30 으로 select. DI 는 `on_trajectory = %250(+0x31 로드)` 를 달아 두는데(21752) 필드명(tcxdict +0x31=on_periodic_trajectory)과 어긋난다 — 지역변수 이름이 `on_trajectory` 인 `let` 이 +0x31 을 담는지, dbg_value 잔재인지 IR 만으로는 판정 불가(동작은 or 라 외연 동일 = 표기 불가) | 3 |  |
| 1 | 표기 불가 | (배치 D) L890 `nontarget_windup_perceived(..) && ty==Champion` 의 소스 순서 — IR 은 호출 후 select 로 합쳐 두 항이 모두 평가된다(ty 로드는 부작용 없어 호이스트 가능). 표기 불가(외연 동일) | 4 |  |
| 2 | 미탐색 | (배치 D) L891 is_in_skill(entity.rs:1572) 의 정확한 반환 형식(Option<&Effect> 인지 bool 을 낀 형태인지) — 인라인만 있고 _gcbc define 미확인(미탐색: `grep define.*is_in_skill /c/tfm2mods/_gcbc`). 동작(casting∈{1,2} 일 때만 is_in_range 검사)은 IR 로 확정 | 4 |  |
| 3 | 미탐색 | (배치 D) L174 vtable+0x1f0 = get_entity_by_id 는 divtable(AbstractGame 0x1f0) 표 기준 — 이 함수 IR 에는 슬롯 오프셋 496 만 있다 | 3 |  |
| 4 | 미탐색 | (배치 D) L183 lane_minion_position_action 이 돌려주는 variant(LaneMinionPosition 태그 13 추정)·L928/L929/L932/L935/L448 콜리 Vec 의 variant 집합 — 각 자식 명세(r13/r14) 계약, 여기서는 통째 memcpy/extend 만 관측 | 5 |  |
| 5 | 미탐색 | (배치 D) constants 의 5 는 end_delay(8곳)와 Around 태그(2곳)가 같은 리터럴이라 본문 값 매칭이 겹친다 — 각각 src_line 으로 구분해 두었다 | 4 |  |
| 6 | 미탐색 | (배치 D) 미탐색(범위 밖): L457 이후(배치 E) — nearest_enemy_tower.is_none_or(\|t\| !Effect::is_in_range(..)) 로 enemy_tower_not_covering_champ 를 만드는 부분과 %1029(None) 합류점 %1045(L461)의 의미는 배치 E 가 판정 | 4 |  |
| 7 | 표기 불가 | (배치 E) 264 local_outnumbered 의 IR 극성이 `near_enemies.len() <= near_ally_count+1` — 이름과 반대(열세가 아니라 「우세/동수」). 소스 표기(>= 뒤집기 등) 는 표기 불가 · 동작은 확정 | 4 |  |
| 8 | 미탐색 | (배치 E) 222 position_score(56B) 의 +48 bool 필드 이름 — dbg_value 없음(+49 는 on_trajectory). 자식 명세(position_score_at_position) 참조 | 4 |  |
| 9 | 미탐색 | (배치 E) 565·591 의 expected_buff/effect_buff_target 반환(288B) 의 None 판정이 +0x48 i32 == -1 인 근거 = IR(option.rs:633 is_some 인라인) · 타입은 Option<DataEffectDef>(288B) 로 추정(tcxdict DataEffectDef 288B · 니치 위치는 tcxdict --enum 이 Option 까지는 안 준다) | 3 |  |
| 10 | 표기 불가 | (배치 E) get_move_action_v46 296~308 의 else-if 사슬은 IR 블록 구조(1790/1796/1797/1798/1802/1804/1807/1808/1813/1814)로 복원 — 소스의 정확한 중첩 형태(`else if` vs 중첩 if)는 외연 동일이라 표기 불가 | 4 |  |
| 11 | 미탐색 | (배치 E) SmallActionPlay 원소의 미초기화 바이트: RunAway(136B)/Around(136B)/Trace(152B) 페이로드 뒤 ~0xb0 과 0xb2~0xb7 은 alloca 잔여(memcpy 184B) — 런타임 대조 시 무시 필요(§4-B 패딩) | 4 |  |
| 12 | 미탐색 | (배치 E) old_actions(%128)·nearest_enemy_tower(%126)·f(%958) 의 생성은 배치 D(≤454행) — 계약만 사용 | 4 |  |
| 13 | 미탐색 | (배치 E) 1029/1030 블록(root 452·454 혼합)은 배치 D 의 nearest_enemy_tower 저장 꼬리 — 내 범위엔 457행 br 만 해당 | 4 |  |
| 14 | 미탐색 | (배치 E) closure#5 의 578/593 iter_towers.any 술어에서 f.team 이 Neutral 인 경우 is_visible_from 이 true(챔피언이 Neutral 일 일은 없음 → 실질 무관) | 4 |  |
| 15 | 미탐색 | (배치 E) range_misjudge_rng / range_misjudge_roll / check_kill_die_tick / score / lane_minion_position_action / v30_… / aoe_heal_covers_low_ally / is_dash_worth / effect_buff_target 내부는 자식 명세(r13/r14) 계약만 — 이 중 TLS 를 쓰는 것(check_kill_die_tick 실측)의 호출 순서만 signature.tls 에 기록 | 4 |  |
| 16 | 표기 불가 | (배치 F) L830~832 `!c.block_input()` 과 `mr == 0` 의 소스상 순서 — column 부재. IR 은 block_input(readonly) 을 먼저 호출한 뒤 `or i1 (mr!=0), block_input` 로 접음(27588~27594). 외연 동일(표기 불가) | 4 |  |
| 17 | 미탐색 | (배치 F) L771 `info.nearest_enemy` (Option<(usize,usize)>) 의 .0 의미 — .1 이 entity id 임은 champ.id 비교로 확정, .0 은 배치 F 에서 안 읽음(미탐색 — Tower AI 갱신 지점은 _gcbc) | 4 |  |
| 18 | 미탐색 | (배치 F) L809 battle_action 5번째 인자 5 · L778 new_with_skill end_delay 5 의 단위/소비처 — 콜리 계약(r14/r13 잎이 아님) 미탐색 | 4 |  |
| 19 | 미탐색 | (배치 F) L837 score(...) 의 판정 내용 · L789 unsafe_v19_non_champion_walkup(line_defense.rs:72, m14.ll:28602~29733) · L810 v30_line_champion_action_tower_aggro_risk(m07.ll:52737) 의 본체 — 배치 F 범위 밖(시그니처만 기록) | 4 |  |
| 20 | 미탐색 | (배치 F) SmallActionRunAway::new_with_skill 이 채우는 필드 값(goal_x/y 등) — 콜리(m08.ll:92086) 미탐색. live 바이트는 struct 레이아웃(tcxdict 136B) 기준 | 3 |  |
| 21 | 미탐색 | (배치 F) L857 Option<Input>::None == -1 은 get_input 본체의 from_residual store(m11.ll:42866) 로 확인. tcxdict 에 Option<Input> 레이아웃 없음(제네릭) — 니치값 -1 의 이유(6 이 아닌) 는 미탐색 | 3 |  |
| 22 | 미탐색 | (배치 F) L845 iter_projectile 이 ExpectedGame 이 아닌 다른 AbstractGame 구현체일 때의 순서 — divtable 은 ExpectedGame 정적 vtable 기준(런타임 구현체 미확정) | 3 |  |
| 23 | 미탐색 | (배치 F) constants 의 59(GameContext.debug gep) 는 판정값이 아니라 오프셋이나 C1 대조를 위해 등재 — reads 에 정본 | 3 |  |

**`notes` 2건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | (배치 D) prof phase id(92/87/64/95/89)와 PHASE_NANOS/PHASE_CALLS 인덱스(132 상한, ns 환산 1000000000)는 계측 전용이라 knobs 에 넣지 않았다 | 4 | 사실 서술 |
| 1 | (배치 F) TLS 절의 key/layout/invalidation — v47_siege_stance·position_eval_at·interaction_score 내부는 각 잎 명세(r13/r15) 소관. 배치 F 는 호출 지점·조건·순서만 확정 | 4 | 사실 서술 |

<details><summary>`closed` 1건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

| # | 물음 | 닫은 근거 |
|---|---|---|
| 0 | (배치 E) 265~267 의 bool %1676 의 소스 변수명 — dbg_value 없음. IR 로 동작은 확정(can_skill&&skill.ty.can_move \|\| can_skill2&&… \|\| can_ult&&…). vtable+0x120 = can_move 는 divtable 일치율 94% 로 확정 근거가 정적 vtable 하나뿐(Arc<dyn> 런타임 구현체별 의미는 동일 슬롯) | 본문에 해소 표기가 있다 |
</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

