---

### `108` TeamPlan::update_objective_after_steal — 스틸 추적 분리 후의 팀 목표(MainObjective) 캐스케이드 — 규율 갱신·사망 해제·갱 전처리·오해 수리·귀환 판정·목표별 핸들러 12종을 순서대로 적용해 self.objective/plan 을 갱신

| 항목 | 값 |
|---|---|
| id | `team_plan__TeamPlan_update_objective_after_steal` |
| 심볼 | `_RNvMs1_NtNtCshdEBA0ozCnw_7game_ai11plan_legacy9team_planNtB5_8TeamPlan28update_objective_after_steal` |
| 소스 | `game-ai\src\plan_legacy\team_plan.rs:910` |
| IR | `m09.ll` 26130~37509행 |
| 경로·가시성 | `game_ai::plan_legacy::team_plan::TeamPlan::update_objective_after_steal` · **in:game_ai::plan_legacy::team_plan** |
| 계층 | 기타 |
| exe | `dda220` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r11` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData)
```

<details><summary>인자 8개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | self | &mut TeamPlan(1064B) | %0 · noalias · readonly 없음 → &mut. objective/objective_discipline/objective_misunderstanding/obj_spawn/mf_obj_clear/chats/comeback_pick_*/press_tower_start_tick 등 다수 필드 쓰기 | 4 |
| 1 | 2 | version | usize | %1 · AI 버전. `version > 1`(L920·L982·PT181) 게이트 3곳 + 콜리 인자로 전달 | 4 |
| 2 | 3 | rnd | &mut StdRng(320B) | %2 · noalias·readonly 없음 → &mut. 본문에서 직접 읽지 않고 콜리(should_recall_to_shop·handle_nexus_attack·need_defense_nexus 등)에 전달만 | 4 |
| 3 | 4 | player | &PlayerState(2528B) | %3 · readonly. info.team(@0x930)·info.position(@0x9c0) 만 직접 읽음 | 4 |
| 4 | 5 | data | &OperationData(24B) | %4 · readonly. cache(@0x0)·context(@0x8)·blackboard(@0x10) 3 포인터 | 4 |
| 5 | 6 | goal_data | &GoalData(248B) | %5 · readonly. epic/serpen 스탠스 틱 6종만 읽음 | 4 |
| 6 | 7 | plan | &mut BigPlan(384B) | %6 · noalias·readonly 없음 → &mut. 태그 읽기 + drop_glue 후 ForcePassive(2)/ActiveRecall(8) 재기록, Battle.sub_goal(@+0x60) 쓰기 | 4 |
| 7 | 8 | debug | &mut DebugFrameData(224B) | %7 · noalias·readonly 없음 → &mut. context.debug 일 때만 infos(@0xa0 HashMap<usize,Vec<String>>) 에 push | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
// ===== team_plan.rs:911 — self.update_v27_objective_discipline(version, player, data, goal_data, debug)  [인라인 · objective_discipline.rs:334~481 · IR 26294~29353]
// 목적: 오브젝트(Morgard/Serpen) 운영 중인 정글러(내 챔프)의 "규율 상태" self.objective_discipline(0x130, Option<ObjectiveDisciplineState>, 니치 태그 @0x149: 0=SafeWait 1=HardDisengage 2=None) 를 매 틱 갱신
// [D336] target = match self.objective(@0x41f 태그) { 0(Morgard) → JungleType 4(Morgard) · %81=true / 1(Serpen) → JungleType 5(Serpen) · %81=false / 그 외 → self.objective_discipline=None(store i8 2 @0x149, IR 26303) 후 종료 }
// [D341~350] phase = self.objective@0x420(enum+1): phase ∉ {1(Setup), 3(Hunt)} → objective_discipline=None(IR 26323) 종료   (Assemble=2·None=0 은 해제)
// [D355] player.info.position(@0x9c0 i32)==1(Jungle) && self.steal_action(@0x418 태그)==2(Commit) → objective_discipline=None(IR 26348) 종료   (스틸 커밋 중엔 규율 없음)
// [D361] team = player.info.team(@0x930); champ = data.cache.player_champion[team][position](@0x1e0+40*team+8*pos); None → objective_discipline=None(IR 26471) 종료
// [D365] objective_entity = v27_objective_entity(인라인): game_mode=cache.game.get_game_mode()(vtable 0x40) → MobaMode 요구(태그 0 아니면 unwrap_failed 패닉); Morgard 면 mode.jungle_runner.epic.live_list[0](@0x1a0 ptr/@0x1a8 len) · Serpen 이면 serpen.live_list[0](@0x1d0/@0x1d8); 비어 있으면 → None(IR 26497) 종료; id 로 cache.game.get_entity_by_id(vtable 0x1f0) → null 이면 None 종료
// [D370~371] tick = cache.game.tick()(vtable 0x28); tps = data.context.setting.tick_per_second(@0x12f8)
// [D372] hp_ratio = champ.hp(@0x670)*100 / champ.stat_cached.hp(@0x628)  (0 이면 div_by_zero 패닉)
// [D373~376] objective_hp_ratio = obj.hp*100/obj.stat_cached.hp; execute_window = objective_hp_ratio < 36 → 참이면 objective_discipline=None(IR 29041) 종료  (막타 창엔 규율 해제)
// [D380] camp_pos = MapDef::camp_pos(map=context.map(@0x20), target, team==0)   [호출]
// [D381] wait_pos = MapDef::camp_pos(map, Morgard?2(Stump):0(Rhino), team==0)   [호출 · v27_objective_wait_pos 인라인]
// [D382~383 closure#0] near_ally_count = count( player_champion[team][*] 중 hp*100/max_hp > 34 && distance_sq(ally, champ) < 19600000001(=140000²+1) )   (5칸 언롤 · 자기 자신 포함)
// [D384~387 closure#1] near_enemy_count = count( player_champion[1-team][*] 중 is_visible_from(champ.team)(=champ.team 이 Player(t) 면 enemy.visible_state[t] 태그==0 Visible 요구, Neutral 이면 통과) && !is_ignored_well_enemy(version, player, enemy)[호출] && Blackboard::is_recent_visible(&data.blackboard[team], game, ctx, player, enemy)[호출] && distance_sq(enemy, champ) < 28900000001(=170000²+1) )
// [D388~389 closure#2] camp_ally_count = count( 아군 중 hp%>34 && distance_sq(ally, camp_pos) < 32400000001(=180000²+1) )
// [D390~393 closure#3] camp_visible_enemy_count = count( 적 중 hp%>34 && is_visible_from && !is_ignored_well_enemy && distance_sq(enemy, camp_pos) < 36100000001(=190000²+1) )   (※closure#1 과 달리 is_recent_visible 없음)
// [D395~406 closure#4/#5] camp_possible_enemy_count = count over p in 0..5 ( enemy=player_champion[1-team][p] 존재 && hp% >= 50(ult 50 → 제외) && !is_recent_visible(enemy) && { d = distance(self.vision.last_visible_pos[p](@0x230+16p), camp_pos)[호출 utils::distance].saturating_sub(180000); elapsed = tick.saturating_sub(self.vision.last_checked_ticks[p](@0x2d0+8p)); elapsed * enemy.stat_cached.move_speed(@0x640) >= d } )   (안 보이는 적이 마지막 확인 이후 캠프 근처까지 걸어올 수 있었는가)
// [D407] camp_enemy_pressure = camp_visible_enemy_count + camp_possible_enemy_count
// [D408] ally_finish_tick = Morgard ? goal_data.epic.epic_ally_tick(@0x98) : goal_data.serpen.epic_ally_tick(@0xd0)
// [D413] ally_finish_soon = (ally_finish_tick - 1) <u tps*4   ⟺ 1 ≤ ally_finish_tick ≤ 4*tps   (`shl %tps,2`)
// [D415] low_contact   = near_enemy_count != 0 && near_ally_count < 2
// [D416] outnumbered   = near_enemy_count > near_ally_count && near_enemy_count > 1
// [D417] camp_out_vis  = camp_visible_enemy_count >= camp_ally_count+2 && camp_visible_enemy_count > 2
// [D418] camp_out_all  = camp_visible_enemy_count > 1 && camp_enemy_pressure >= camp_ally_count+3 && camp_enemy_pressure > 3
// [D419~428] new_kind 결정 (Option<Kind>: 2=None):
//   if hp_ratio < 35:
//     if hp_ratio < 25:                       [D420~422]
//       if near_enemy_count != 0 || ally_finish_soon → new = ally_finish_soon ? None : HardDisengage(1), release=false → [D437]
//       else → (D426 로)
//     else (25≤hp<35):                        [D422~423]
//       if near_enemy_count != 0 && !ally_finish_soon:
//         if near_ally_count < 2 || outnumbered || camp_out_vis → new = HardDisengage(1), release=false → [D437]
//         else → (D426 로)
//       else → [D425]
//   else → [D425]
//   [D425] if ally_finish_soon → new = None, → [D434]   else → (D426 로)
//   (D426) if low_contact && objective_hp_ratio > 54 → new = SafeWait(0)
//   (D427) elif near_ally_count < 2 && objective_hp_ratio > 49 && outnumbered → SafeWait(0)
//   (D428) elif camp_ally_count < 2 && camp_out_all && objective_hp_ratio > 54 → SafeWait(0)
//   else → None
// [D434~436] if hp_ratio < 40 || near_enemy_count > near_ally_count+1 → release=false  else release = (camp_ally_count+2 >= camp_enemy_pressure)
// [D437] current = self.objective_discipline(@0x130..0x14f, 태그 @0x149):
//   current==None(태그 2): if new.is_some → [D453] self.objective_discipline = Some{wait_pos(@0x130,@0x138)=wait_pos, until_tick(@0x140)=tick + (new==HardDisengage ? tps*3 : tps), target(@0x148)=target, kind(@0x149)=new}   (IR 29229~29238)   else 아무것도 안 함
//   current==Some: [D438] if ally_finish_soon || release || current.target != target || current.until_tick <= tick → [D439] self.objective_discipline = None(store i8 2 @0x149, IR 29146)
//     else if new.is_some: [D440~451] kind' = (new==HardDisengage) ? HardDisengage : current.kind; current.wait_pos = wait_pos; current.until_tick = max(tick + (kind'==HardDisengage ? tps*2 : tps/2), current.until_tick); target 재기록; 통째 store(IR 29202~29212, 패딩 6B memcpy 포함)
//     else (new None, 유지조건) → 변경 없음
// [D466~468] if context.debug(@0x3b): debug.infos(@0xa0 HashMap<usize,Vec<String>>).entry(champ.id(@0x5c0)).or_default().push(format!("v27 objective discipline: {:?} {:?} hp:{} obj:{} near_a:{} near_e:{} camp_a:{} camp_p:{}", target, kind, hp_ratio, objective_hp_ratio, near_ally_count, near_enemy_count, camp_ally_count, camp_enemy_pressure))   [호출 rustc_entry · or_insert_with · format_inner · Vec<String>::push_mut · Vec 힙 재할당 지점]
// ===== team_plan.rs:920~939 — 오브젝트 사망(장기 리스폰) 시 목표 해제  [IR 29354~29469]
// if version > 1 {                                                         // L920 (`icmp ugt %1, 1`)
//   if let Some(moba) = data.cache.game.get_game_mode().as_moba() {         // L921 (vtable 0x40 · 태그 0 = MobaMode)
//     now = game.tick(); limit = tps * 15;                                  // L922~923
//     dead = match self.objective(@0x41f) {                                 // L924
//       Some(Morgard{..}) => moba.jungle_runner.epic.live_list.len(@0x1a8)==0 && moba.jungle_runner.epic.next_respawn_tick(@0x1b0).saturating_sub(now) > limit,     // L926~927
//       Some(Serpen{..})  => moba.jungle_runner.serpen.live_list.len(@0x1d8)==0 && serpen.next_respawn_tick(@0x1e0).saturating_sub(now) > limit,                   // L930~931
//       _ => false };
//     if dead {                                                             // L935
//       self.mf_note_obj_clear(36, now) → self.mf_obj_clear = (36 @0x210, now @0x218)   // L936 (인라인)
//       self.objective = None (store i8 -1 @0x41f)                          // L937   ★None 의 메모리값 = 0xFF(IR 정본, 12 가 아님)
//       if matches!(plan.goal(), BigGoal::Epic(2) | BigGoal::Serpen(3)) { drop(*plan); *plan = BigPlan::ForcePassive (store i64 2 @plan+0) }   // L938~939 [호출 BigPlan::goal sret 24B · drop_glue BigPlan]
//     }
//   }
// }
// ===== team_plan.rs:945~946 — if let Some(Gank{line}) = self.objective { self.handle_gank_preprocess(version, player, data, plan, line) }  (태그 8 · IR 30083~30554 · 아래 §3)
// ===== team_plan.rs:949 — self.repair_misunderstood_objective(player, data, plan, debug)  [인라인 · team_plan.rs:603~720 · IR 29476~30081]
// state = self.objective_misunderstanding(@0x90 Option<ObjectiveMisunderstanding>; 태그 = @0x90 i64: -1 → None, 그 외 = opposite_group_start_tick 의 Option 태그 0/1)   // R604
//   None → return (R604)
// target = match self.objective { Morgard → JungleType 4 / Serpen → 5 / 그 외 → self.objective_misunderstanding = None(store i64 -1 @0x90) return }   // R607~609
// if state.target(@0xb8) != target → objective_misunderstanding = None, return   // R611~612
// phase = self.objective.phase(@0x420); opposite = Morgard?5:4                                 // R0
// opposite_available = (Morgard 목표 → serpen_exists: spawn_serpen(ctx.tutorial(@0x38) ∈ {0,5,7,8}) && moba.jungle_runner.serpen.live_list.len(@0x1d8)!=0 / Serpen 목표 → morgard_exists: spawn_epic(tutorial ∈ {0,7,8} — IR `(t-1) ult 6` 이면 false) && epic.live_list.len(@0x1a8)!=0)   // R634~635
// if !opposite_available → state.opposite_group_start_tick = None(@0x90=0), state.solo_hunt_start_tick = None(@0xa0=0); return   // R638~641
// tick = game.tick(); marker_age = tick.saturating_sub(state.accepted_tick(@0xb0)) / max(tps,1)   // R645~647
// target_camp = MapDef::camp_pos(map, target, team==0); opposite_camp = camp_pos(map, opposite, team==0)   // R649~650 [호출]
// target_allies   = v23_healthy_allies_near_point(player, data, target_camp, 180000, 40)     // R651 [호출]
// opposite_allies = v23_healthy_allies_near_point(player, data, opposite_camp, 200000, 40)   // R652 [호출]
// required_group = player_count(ctx.tutorial): {1,3}→2 · {2,4,6}→1 · {0,5,7,8}→3 (그 외 unreachable)   // R653
// opposite_group_start: if opposite_allies >= required_group && !(target_allies > 1 && opposite_allies < target_allies+2) → Some(기존 Some 이면 유지, 아니면 tick) else None   // R655~658
// target_started = objective_is_damaged(data, target)   // R659 [호출]
// my_near_target = player_champion[team][pos] 존재 && distance_sq(champ, target_camp) < 36100000001(=190000²+1)   // R666~667
// solo_hunt_start: if phase==3(Hunt) && target_started && my_near_target && target_allies < 2 → Some(기존 유지 or tick) else None   // R668~670
// opposite_group_elapsed = opposite_group_start.is_some_and(|s| tick.saturating_sub(s) >= tps*4)   // R678 (`shl tps,2`)
// solo_hunt_elapsed      = solo_hunt_start.is_some_and(|s| tick.saturating_sub(s) >= tps*6)         // R680
// if !(opposite_group_elapsed || solo_hunt_elapsed) → self.objective_misunderstanding = 갱신된 state(두 Option 재기록 @0x90..@0xaf); return   // R681~682
// [수리 발동] new_phase = (opposite_allies >= required_group || objective_is_damaged(data, opposite)) ? Hunt(3) : Setup(1); with_battle = true   // R686 [호출]
// self.objective = Some(opposite 오브젝트: Morgard→Serpen(store i8 1) / Serpen→Morgard(store i8 0)) with {phase=new_phase @0x420, with_battle=1 @0x421} (store i16 259|257)   // R691
// self.objective_misunderstanding = None (store i64 -1 @0x90)   // R696
// goal_matches = plan.goal() == (새 목표 Morgard ? Epic(2) : Serpen(3)); if !goal_matches → drop(*plan); *plan = ForcePassive(2)   // R698~704 [호출 BigPlan::goal · drop_glue]
// if champ 존재 && ctx.debug(@0x3b): debug.infos.entry(champ.id).or_insert(vec![]).push(format!("objective misunderstanding repaired: {:?} -> {:?} by {}", target, opposite, opposite_group_elapsed ? "opposite_group" : "solo_hunt")); …push(format!(" objective misunderstanding age: {}s", marker_age))   // R707~715 [호출 rustc_entry·or_insert·format_inner·push_mut ×2]
// ===== team_plan.rs:945~947 — if let Some(Gank{line}) = self.objective(태그 8) { if self.handle_gank_preprocess(version, player, data, plan, line) { return } }   [인라인 · objective_handlers.rs:1133~1225 · IR 30083~30559 · 반환 true 경로(1827) → ret void / false 경로(1826) → L949 로]
// G1134: if !rule_scope::line_exists(ctx, line)[호출] → mf_note_obj_clear(38, tick): mf_obj_clear=(38 @0x210, tick @0x218); self.objective=None(@0x41f=0xFF); if plan 태그(@plan+0 i64)==10(LineGanker) → drop(*plan); *plan=ForcePassive(2); return true   // G1135~1139
// G1143: if tick.saturating_sub(self.gank_start_tick(@0x350)) > tps*20 → mf_note_obj_clear(39, tick); objective=None; if plan==LineGanker → ForcePassive; return true   // G1144~1148
// G1152: target_laner_pos = match line { Top(0)→0, Mid(1)→2, Bottom(2)→3 } (그 외 unreachable)
// G1158~1160: laner = cache.player_champion[team][target_laner_pos]; cancel_reason = laner None → 2 / laner.hp*100/max_hp < 25 → 1
//   취소 시 (G1166~1170): mf_note_obj_clear(40, tick); objective=None; self.chats(@0xc0 Vec<Chat>).push(Chat{tag 17, byte1=cancel_reason}) [인라인 push · 용량 부족 시 grow_one 호출 = 힙 재할당]; if plan==LineGanker → ForcePassive; return true
// G1175: enemy_tower = cache.top/mid/bottom_tower[1-team](@0x180+0x20*line+8*(1-team)).or(tower2(@0x190+0x20*line+8*(1-team)))
// G1177~1180: tower_target_minion = enemy_tower.and_then(|t| t.ty(@0x68)==Tower(2) && t.ty.Tower.info.nearest_enemy(@0x88 Some) → cache.game.get_entity_by_id(nearest_enemy.1(@0x98)) ).filter(|m| m.ty==Minion(1) && m.team==Player(team))   — 없으면 return false(→ L949 로 계속)
// G1193: tower_attack = enemy_tower.attack_effect(@0x490 · Option 태그 @0x4c0 i32 -1=None → return false)
// G1197~1199: ally_minions_in_range = cache.iter_minions(team)[호출 sret 56B].filter(closure#1: tower_attack/enemy_tower 캡처).count()[호출 count 조각 s_0]; tower_damage = Effect::expected_damage_target(tower_attack, ctx, enemy_tower, ?, minion)[호출]
// G1200: if ally_minions_in_range < 2 && !(tower_target_minion.hp(@0x670) > tower_damage) → return false
// G1205: near_allies  = (0..5).filter(closure#2: cache,ctx,player,line 캡처).count()[호출 count 조각 s0_0]
// G1212: near_enemies = iter_champions(1-team).filter(closure#3: game,ctx,blackboard,line,player 캡처).count()[호출 count 조각 s1_0]
// G1218: if near_allies > 1 && near_allies > near_enemies → self.objective = Some(Dive{line}) (store i8 9 @0x41f, line @0x420); self.chats.push(Chat{tag 44, byte1=line, +8=0}) [grow_one 가능]; return true   // G1219~1221
//   else return false
// ===== team_plan.rs:949~950 — if self.repair_misunderstood_objective(player, data, plan, debug) { return }   (위 §R · 수리 발동 경로(1639)만 true → ret)
// ===== team_plan.rs:956~972 — 귀환 판정
// if plan 태그 == 8(ActiveRecall) → L977 로                                           // L956
// else if plan.goal() 태그 == 5(Battle) → L977 로                                        // L957 [호출 BigPlan::goal]
// else if matches!(self.objective, Some(Dive(9)|Defense(2)|DefenseLine(3)|Nexus(4))) → L977 로   // L958
// else if should_recall_to_shop(version, rnd, player, data, goal_data) [호출] {           // L961
//   if plan 태그 == 7(PassiveJungle) {                                                   // L963
//     moba = cache.game.get_game_mode().as_moba().unwrap();                            // L964 (태그≠0 → unwrap_failed 패닉)
//     camp_live: Vec<usize> = JungleRunner::get_jungle_live_list(&moba.jungle_runner(@+0x18), jungle.jungle(@plan+0x68), jungle.team(@plan+0x50)==0) [호출 sret 24B]
//     camp_pos = MapDef::camp_pos(map, jungle.jungle, jungle.team==0)                   // L965 [invoke]
//     skip_for_camp = !camp_live.is_empty() && cache.player_champion[team][pos].is_some_and(|c| distance_sq(c, camp_pos) < 14400000001(=120000²+1))   // L966~967 (closure#0)
//     drop(camp_live)  [Vec<usize> drop_glue 호출 = 힙 해제]                             // L968
//     if skip_for_camp → L977 로                                                         // L971
//   }
//   drop(*plan); *plan = BigPlan::ActiveRecall (store i64 8 @plan+0); return              // L972
// }
// ===== team_plan.rs:977~1022 — match self.objective(@0x41f):
//   None(0xFF) | Some(Gank(8))  → self.handle_none_or_gank_objective(version, rnd, player, data, goal_data, plan, debug)[호출 · 반환 bool 무시]; return   // L1022
//   Some(Repair(7)) → team=player.info.team; if version > 1 { if v3_epicops_repair_need(player→team 승격, data→cache 승격, plan)[호출 · u8] == 0 { mf_note_obj_clear(46, tick); self.objective=None } return }   // L982~985
//                     else { self.handle_repair_objective(self, team, position, cache, context)[호출 · argpromotion] return }   // L988
//   Some(Serpen{phase,..})(1) → §handle_serpen (L991~992)   Some(Morgard{phase,..})(0) → §handle_morgard (L994~995)
//   Some(PressEpic(line))(5) → §press_epic (L997~998)   Some(SplitEpic(line))(6) → §split_epic (L1000~1001)
//   Some(Defense)(2) → §defense (L1003~1004)   Some(Nexus(line))(4) → §nexus (L1006~1007)   Some(DefenseLine(line))(3) → §defense_line (L1009~1010)
//   Some(Dive(line))(9) → §dive (L1012~1013)   Some(PressTower(line))(10) → §press_tower (L1015~1016)   Some(ComebackPick{line,ready})(11) → §comeback_pick (L1018~1019)
//   (모든 arm 은 ret void 로 끝난다 — L1025)
// ===== team_plan.rs:994~995 — Some(Morgard{phase,..}) → self.handle_morgard_objective(version, rnd, player, data, goal_data, plan, debug, phase)  [인라인 · objective_handlers.rs:685~829 · IR 30760~31671]
// 공통 매크로: clear(N) = mf_note_obj_clear(N, tick) → self.mf_obj_clear=(N @0x210, tick @0x218) 후 self.objective=None(0xFF @0x41f) / FP = drop(*plan); *plan=ForcePassive(store i64 2 @plan+0) / chat(T,..) = self.chats(@0xc0).push(Chat{태그 T,..}) [grow_one 재할당 가능]
// M686: if !morgard_exists(ctx, cache) [= spawn_epic: ctx.tutorial(@0x38) ∈ {0,7,8} … 인라인 `(t-7) ult -6`] → clear(35); FP; return   // M687~689
// M697: if version > 1 { moba = get_game_mode().as_moba().unwrap(); if moba.jungle_runner.epic.live_list.len(@0x1a8)==0 && epic.next_respawn_tick(@0x1b0).saturating_sub(tick) > tps*15 → clear(36); if plan.goal()==Epic(2) → FP; return }   // M698~704
// M710: if let Some(line) = self.handle_nexus_attack(version, rnd, player, data, debug)[호출 · i8 -1=None] → self.objective = Nexus(line)(store i8 4 @0x41f, line @0x420); chat(42 AttackNexus{line, +8=0}); return   // M711~712
// M718: if need_defense_nexus(version, rnd, player, data, debug)[호출] { need_count = calculate_nexus_defense_count(version, rnd, player, data, debug)[호출]; if objective_defense_role(version, player, data, JungleType::Morgard(4), need_count)[호출] == GoDefend(1) → self.objective=Defense(store i8 2); FP; chat(43 DefenseNexus{+8=0}); return }   // M719~723
// M728: if phase == Hunt(3) {
//   M729: if (0..5).all(|p| !Blackboard::in_epic(&data.blackboard[team], p)[호출 ×5 언롤]) → self.objective = Morgard{phase=Setup(1) @0x420, with_battle=true @0x421}(store i8 0/1/1); if plan.goal()==Epic → FP; return   // M730~736
//   M741: if min(goal_data.epic.epic_enemy_killed_tick(@0x90), goal_data.epic.epic_ally_killed_tick(@0xa0)) > tps*2 (`shl tps,1`) {   // (dbg 이름 skip_disengage 는 이 비교값 — 참일 때 이탈 판정을 "수행"한다)
//     M744~745: camp_pos = MapDef::camp_pos(map, Morgard(4), team==0); if fight_check::should_disengage_object_hunt(version, player, data, camp_pos, 150000)[호출] → self.objective = Morgard{Setup(1), with_battle=true}; if goal==Epic → FP; chat(34 MorgardSetup{+8=0}); return   // M746~753
//   }
// }
// M759: moba(unwrap); if epic.live_list.len==0 { if epic.next_respawn_tick.saturating_sub(tick) > tps*15 → clear(36); if goal==Epic → FP; return  else return }   // M759~766
// M770: if phase == Setup(1) {
//   M771: if check_epic_giveup(version, rnd, player, data, self, debug)[호출] → [GIVEUP 경로 M772~798]:
//     clear(37); self.obj_spawn.epic_giveup_tick = Some(tick)(@0x50=1, @0x58=tick)   // M772~774
//     if plan 태그==9(Battle) && !BattlePlan::is_end(&plan.battle(@plan+8))[호출] → plan.battle.sub_goal(@plan+0x60) = RunAway(태그 4 · BattleSubPlanGoal)   // M776~778
//     if plan.goal()==Epic → FP   // M782~783
//     moba(unwrap); if epic.live_list.len!=0 → chat(40 MorgardGiveUp)   // M786~787
//     if serpen_exists[spawn_serpen: tutorial ∈ {0,5,7,8} && serpen.live_list.len(@0x1d8)!=0] && self.obj_spawn.serpen_giveup_tick.is_none()(@0x60==0) → return   // M790
//     if let Some(press_line) = check_press_tower_opportunity(version, rnd, player, data, self, Some(Bottom)(i8 2), debug)[호출] → self.objective=PressTower(press_line)(store i8 10, line @0x420); self.press_tower_start_tick(@0x358)=tick; chat(45 PressTower{line,+8=0}); FP; return   // M793~797
//     return
//   M806: v24_setup_ready = self.v24_objective_setup_lane_pressure_ready(version, player, data, goal_data, Morgard(4), debug)[호출]; delay = self.should_delay_morgard_for_wave_priority(player, data, goal_data)[호출]
//   M807~810: if delay && !v24_setup_ready { if goal==Epic → FP; return }
//   M814: if v24_setup_ready || check_epic_hunt(version, rnd, player, data, goal_data, plan, self, debug)[호출 · 단락평가: ready 면 호출 안 함] → self.objective = Morgard{Hunt(3), with_battle=true}(store i8 0/3/1); if goal != Epic → FP; chat(38 MorgardHunt{+8=0}); return   // M815~822
//   return
// } else if phase == Hunt(3) { if check_epic_giveup(...)[호출] → 위 GIVEUP 경로; else return }   // M770~771 (%2136)
// else return   (Assemble/None)
// ===== team_plan.rs:991~992 — Some(Serpen{phase,..}) → self.handle_serpen_objective(version, rnd, player, data, goal_data, plan, debug, phase)  [인라인 · objective_handlers.rs:558~684 · IR 31673~32463 · Morgard 판과 대칭, 차이만 표기]
// S559: if !serpen_exists(ctx) [= spawn_serpen: tutorial ∈ {0,5,7,8} · switch] → clear(32); FP; return   // S560~562
// S566: if let Some(line) = handle_nexus_attack(...)[호출] → objective=Nexus(line)(store i8 4); chat(42 AttackNexus{line,0}); return
// S574: if need_defense_nexus(...)[호출] { need_count = calculate_nexus_defense_count(...)[호출]; if objective_defense_role(version, player, data, JungleType::Serpen(5), need_count)[호출]==GoDefend(1) → objective=Defense(2); FP; chat(43); return }
// S584: moba(unwrap); if moba.jungle_runner.serpen.live_list.len(@0x1d8)==0 { S585: if serpen.next_respawn_tick(@0x1e0).saturating_sub(tick) > tps*15 → clear(33); if goal==Serpen(3) → FP; return  else return }   // ※Morgard 판(M697)과 달리 version 게이트 없음·위치도 다름(nexus/defense 뒤)
// S595: match phase { Hunt(3) → S596.. / Setup(1) → S661.. / 그 외 → return }
// [Hunt] S596: if (0..5).all(|p| !Blackboard::in_serpen(&blackboard[team], p)[호출 ×5]) → objective = Serpen{Setup(1), with_battle=true}(store i8 1/1/1); if goal==Serpen → FP; return   // S597~602
//   S607: if min(goal_data.serpen.epic_enemy_killed_tick(@0xc8), goal_data.serpen.epic_ally_killed_tick(@0xd8)) > tps*2 { S610~611: camp_pos = camp_pos(map, Serpen(5), team==0); if should_disengage_object_hunt(version, player, data, camp_pos, 150000)[호출] → objective=Serpen{Setup,true}; if goal==Serpen → FP; chat(25 SerpenSetup{+8=0}); return }   // S612~619
//   S626: if check_serpen_giveup(version, rnd, player, data, self, debug)[호출] → GIVEUP-S; else return
// [Setup] S626: if check_serpen_giveup(...)[호출] → GIVEUP-S
//   else S661: v24_setup_ready = v24_objective_setup_lane_pressure_ready(self, version, player, data, goal_data, Serpen(5), debug)[호출]; delay = should_delay_serpen_for_wave_priority(self, player, data, goal_data)[호출]
//   S662~665: if delay && !ready { if goal==Serpen → FP; return }
//   S669~677: if ready || check_serpen_hunt(version, rnd, player, data, goal_data, plan, self, debug)[호출] → objective = Serpen{Hunt(3), with_battle=true}; if goal != Serpen → FP; chat(29 SerpenHunt{+8=0}); return   else return
// [GIVEUP-S] S627: clear(34); S629: self.obj_spawn.serpen_giveup_tick = Some(tick)(@0x60=1, @0x68=tick); S631: if goal==Serpen → FP; S635~637: if plan==Battle(9) && !BattlePlan::is_end(plan+8)[호출] → plan.battle.sub_goal(@plan+0x60)=RunAway(4)
//   S641: reason = serpen_giveup_chat_reason(player, data)[호출 · Option<SerpenGiveUpReason> i8, 2=None]; if reason != None → chat(31 SerpenGiveUp{reason})
//   S645: if morgard_exists[spawn_epic(tutorial∈{0,7,8}) && epic.live_list.len(@0x1a8)!=0] && self.obj_spawn.epic_giveup_tick.is_none()(@0x50==0) → return
//   S648~652: if let Some(press_line) = check_press_tower_opportunity(version, rnd, player, data, self, Some(Top)(i8 0), debug)[호출] → objective=PressTower(line)(store i8 10); press_tower_start_tick(@0x358)=tick; chat(45 PressTower{line,0}); FP; return   else return
// ===== team_plan.rs:1003~1004 — Some(Defense) → self.handle_defense_objective(version, rnd, player, data, plan, debug)  [인라인 · objective_handlers.rs:16~27 · IR 32478~32512]
// if need_defense_nexus(version, rnd, player, data, debug)[호출] → return; if plan 태그==9(Battle) → return; else clear(2); FP; return   // D17~23
// ===== team_plan.rs:1006~1007 — Some(Nexus(line)) → self.handle_nexus_objective(version, rnd, player, data, plan, line, debug)  [인라인 · objective_handlers.rs:29~44 · IR 33017~33072]
// if !rule_scope::line_exists(ctx, line)[호출] → clear(3); FP; return   // N30~33
// if !ai::end_check(version, rnd, player, data, line, debug)[호출] → clear(4); FP; return   else return   // N37~40
// ===== team_plan.rs:1009~1010 — Some(DefenseLine(line)) → self.handle_defense_line_objective(version, rnd, player, data, plan, line, debug)  [인라인 · objective_handlers.rs:46~87 · IR 32539~32996]
// DL47~49: if !line_exists(ctx, line)[호출] → clear(5); FP; return
// DL54 (handle_nexus_defense 인라인 1227~1235): if need_defense_nexus(...)[호출] && i_am_chosen_defender(player, data, calculate_nexus_defense_count(...)[호출], &[] (빈 슬라이스))[호출] → DL55~56: self.objective=Defense(store i8 2); chat(43 DefenseNexus{+8=0}); FP; return
// DL61: tower = cache.{top,mid,bottom}_tower[team](@0x180/0x1a0/0x1c0 + 8*team).or({..}_tower2[team](@0x190/0x1b0/0x1d0))
// DL62~67: if tower None → tower = cache.twin_towers[team](@0x130+0x20*team, bumpalo Vec: ptr @+0, len @+24).iter().min_by_key(|t| distance_sq(t, LineType::get_start_position(&line, context.setting(@0x8), team)[호출]))   (closure#0/#1 · map+reduce 인라인, Map::next 호출 1회)
// DL80~81: if tower 여전히 None → clear(7); FP; return
// DL70~71: enemies = cache.champions(1-team, context.pool)[호출 sret 32B bumpalo Vec]; has_near_enemy_champion = enemies.iter().any(|e| Blackboard::is_recent_visible(&blackboard[team], game, ctx, player, e)[invoke] && distance_sq(e, tower) < 90000000001(=300000²+1)); drop(enemies)[bumpalo Vec drop_glue + Entity Drop 호출]   (closure#2)
// DL73~75: if !has_near_enemy_champion → clear(6); FP; return   else return (DL86)
// ===== team_plan.rs:997~998 — Some(PressEpic(line)) → self.handle_press_epic_objective(version, rnd, player, data, goal_data, plan, line, debug)  [인라인 · objective_handlers.rs:253~333 · IR 33075~34112]
// PE254~257: if !morgard_exists(spawn_epic: tutorial∈{0,7,8}) || !line_exists(ctx, line)[호출] → clear(20); FP; return
// PE261~263: if ai::end_check(version, rnd, player, data, line, debug)[호출] → objective=Nexus(line)(store i8 4, line @0x420); chat(42 AttackNexus{line,0}); return
// PE267~269: if need_defense_nexus(...)[호출] && i_am_chosen_defender(player, data, calculate_nexus_defense_count(...)[호출], &[])[호출] → objective=Defense(2); FP; chat(43); return
// PE274~275: goal = plan.goal()[호출]; if goal == Line(0){now_line} && now_line != line → FP   (다른 라인 플랜이면 강제 패시브 후 계속)
// PE280: if goal != Battle(5) {
//   PE281: low_hp_allies = count(player_champion[team][*] 존재 && hp*100/max_hp < 41); live_allies = count(존재); live_enemies = count(player_champion[1-team][*] 존재)   (5칸 언롤 · closure#0)
//   PE285~287: if low_hp_allies > 1 && live_allies <= live_enemies → objective=Repair(store i8 7); chat(23 Repair{+8=0}); return
// }
// PE292~294: if serpen_exists(spawn_serpen: tutorial∈{0,5,7,8}) && check_serpen_setup(version, rnd, player, data, self, debug)[호출] { if should_delay_serpen_for_wave_priority(self, player, data, goal_data)[호출] → return
//   PE297~300: if check_serpen_giveup(version, rnd, player, data, self, debug)[호출] → self.obj_spawn.serpen_giveup_tick = Some(tick)(@0x60=1,@0x68=tick); if goal==Serpen(3) → FP; return
//   PE305~312: objective = Serpen{Setup(1), with_battle=true}(store i8 1/1/1); chat(25 SerpenSetup{+8=0}); if goal != Serpen → FP; return }
// PE317~319: moba(unwrap); if moba.epic_minion_buff_time[team](@0x240+8*team) == 0 → clear(21); return   (remain_epic_time 인라인 · 버프 시간 소진)
// PE321~323: live_ally_count = count(my champs 존재 && hp% > 29); live_enemy_count = count(enemy champs 존재)   (closure#1 · 5칸 언롤)
// PE324~325: if live_ally_count < live_enemy_count → objective=Repair(7); return
// PE329: else self.handle_epic_line_change(version, rnd, player, data, debug, line)[호출]; return
// ===== team_plan.rs:1000~1001 — Some(SplitEpic(line)) → self.handle_split_epic_objective(version, rnd, player, data, goal_data, plan, line, debug)  [인라인 · objective_handlers.rs:334~392 · IR 34114~34483 · PressEpic 의 축소판]
// SE335~338: if !morgard_exists || !line_exists(ctx, line)[호출] → clear(22); FP; return
// SE342~344: if end_check(...)[호출] → objective=Nexus(line); chat(42); return
// SE348~350: if need_defense_nexus && i_am_chosen_defender(…, &[]) → objective=Defense; FP; chat(43); return
// SE355~356: if plan.goal()==Line{now_line} && now_line != line → FP
// SE361~363: if serpen_exists && check_serpen_setup(...)[호출] { if should_delay_serpen_for_wave_priority(...)[호출] → return
//   SE366~369: if check_serpen_giveup(...)[호출] → obj_spawn.serpen_giveup_tick=Some(tick); if goal==Serpen → FP; return
//   SE374~381: objective=Serpen{Setup,true}; chat(25 SerpenSetup); if goal != Serpen → FP; return }
// SE386~388: moba(unwrap); if moba.epic_minion_buff_time[team](@0x240+8*team)==0 → clear(23); return   else return   (※low_hp Repair·handle_epic_line_change 없음)
// ===== team_plan.rs:1012~1013 — Some(Dive(line)) → self.handle_dive_objective(version, rnd, player, data, plan, line, debug)  [인라인 · objective_handlers.rs:88~140 · IR 34486~35389]
// DV89~92: if !line_exists(ctx, line)[호출] → clear(8); FP; return
// DV96~98: if let Some(nexus_line) = handle_nexus_attack(...)[호출] → objective=Nexus(nexus_line); chat(42 AttackNexus{line,0}); return
// DV101~104: if need_defense_nexus && i_am_chosen_defender(…, calculate_nexus_defense_count(…), &[]) → objective=Defense; FP; chat(43); return
// DV108~111: if tick.saturating_sub(self.gank_start_tick(@0x350)) > tps*30 → clear(9); FP; return
// DV116~120: near_allies = Σ_{p∈0..5} [ player_champion[team][p].is_some_and(|c| map_regions::is_near_line(ctx, c.x, c.y, line)[호출] && c.hp*100/max_hp > 39) ]   (closure#0 · 5칸 언롤)
// DV122~126: near_enemies = count( player_champion[1-team][*] 존재 && is_near_line(ctx, e.x, e.y, line)[호출] && Blackboard::is_recent_visible(&blackboard[team], game, ctx, player, e)[호출] )   (closure#1)
// DV128: if near_allies > near_enemies → return (Dive 유지)
// DV129~134: if tick.saturating_sub(gank_start_tick) > tps*20 → clear(10); FP; return   else objective = Gank(line)(store i8 8, line @0x420 — 다이브 → 갱 복귀); return
// ===== team_plan.rs:1015~1016 — Some(PressTower(line)) → self.handle_press_tower_objective(version, rnd, player, data, goal_data, plan, line, debug)  [인라인 · objective_handlers.rs:141~252 · IR 35392~36229]
// PT142~145: if !line_exists(ctx, line)[호출] → clear(11); FP; return
// PT149~151: if let Some(nexus_line) = handle_nexus_attack(...)[호출] → objective=Nexus(nexus_line); chat(42); return   (FP 없음)
// PT154~156: if need_defense_nexus && i_am_chosen_defender(…, &[]) → objective=Defense; FP; chat(43); return
// PT161~163: enemy_team = 1-team; if cache.{top,mid,bottom}_tower[enemy_team].is_none() && {..}_tower2[enemy_team].is_none() → clear(12); FP; return
// PT168~170: if tick.saturating_sub(self.press_tower_start_tick(@0x358)) > tps*30 → clear(13); FP; return
// PT175~176: minion_state = &blackboard[team].{top(@0x0),mid(@0x28),bottom(@0x50)}_minion_state[line]; if minion_state.from_mid(@+0x10 i64) < -1000 {
//   PT181~191: engaged_at_tower = version > 1 && cache.tower(line, enemy_team)[호출 · Option<&Entity>].is_some_and(|t| closure#0(cache, ctx, player)(t.x, t.y))[호출 closure#0 아웃오브라인 조각]; if !engaged_at_tower → clear(14); FP; return }
// PT197~202: for check_line in rule_scope::valid_lines(ctx.tutorial)(인라인: {0,7,8}→3라인 · 5→2라인 · {1,3}/{2}/{4}/{6}→1라인 정적 배열) { if check_line != line { ms = minion_state(check_line); if ms.from_mid < -2999 && ms.minion_count(@+0x20 i32) < -2 → clear(15); FP; return } }
// PT208~209: healthy_allies = iter_champions(team).filter(closure s_0).count()[호출 · 아웃오브라인]; live_enemies = count(player_champion[enemy_team][*] 존재)
// PT211~213: if !(healthy_allies > live_enemies) → clear(16); FP; return
// PT219~227: near_enemies = iter_champions(enemy_team).filter(closure s0_0(cache, game, ctx, blackboard, player, line))[호출 count 조각]; if near_enemies > 1 → clear(17); FP; return
// PT232~234: if serpen_exists(spawn_serpen) && check_serpen_setup(version, rnd, player, data, self, debug)[호출] → clear(18); FP; return
// PT238~240: if morgard_exists(spawn_epic) && check_epic_setup(version, rnd, player, data, self, debug)[호출] → clear(19); FP; return
// PT245~246: if plan.goal()==Line{now_line} && now_line != line → FP;  PT251: return
// ===== team_plan.rs:1018~1019 — Some(ComebackPick{line, ready}) → self.handle_comeback_pick_objective(version, rnd, player, data, goal_data, plan, line, ready, debug)  [인라인 · objective_handlers.rs:393~557 · IR 36231~37462]
// 보조: outcome(V) = self.comeback_pick_outcomes(@0xd8 Vec<(i32,u8)>; len @0xe8, ptr @0xe0).last_mut().map(|l| l.1(@+4 u8) = V)   (1=내 킬 성공 · 2=타임아웃 · 3=적 과다/무성과 · 4=아군 부족 · 5=적 킬 · 6=넥서스로 선점 · 0=진행 중)
// CB394~396: if !line_exists(ctx, line)[호출] → clear(24); FP; return
// CB401~404: if let Some(nexus_line) = handle_nexus_attack(...)[호출] → if last.1==0 { outcome(6) }; objective=Nexus(nexus_line); chat(42); return   (FP 없음)
// CB407~411: if need_defense_nexus && i_am_chosen_defender(…, &[]) → if last.1==0 { outcome(6) }; objective=Defense; FP; chat(43); return
// CB415: pick_start = self.comeback_pick_start_tick(@0x360)
// CB416~420 (closure#0): my_kill_on_line = game.kill_logs()(vtable 0x130 · &[KillLog] 48B stride).iter().rev().any(|log| log.killer_team(@+0x20)==team && log.tick(@+0x18) >= pick_start && tick.saturating_sub(log.tick) <= tps*8(`shl 3`) && (log.killed_position(@+0x2c Position i32)==Jungle(1) || line_of(killed_position: Top(0)→Top, Mid(2)→Mid, Bottom(3)|4→Bottom) == line))
// CB427~431 (closure#1): enemy_kill_on_line = 같은 술어에 killer_team != team
// CB439~452: if my_kill_on_line → self.comeback_pick_success_count(@0x370) += 1; outcome(1); clear(26); FP; → CB454 로 이어짐(return 아님)
// CB440~443: else if enemy_kill_on_line → outcome(5); clear(25); FP; return
// CB479~484: timeout_s = ready ? 20 : 15; if tick.saturating_sub(pick_start) > tps*timeout_s → outcome(2); clear(27); FP; return
// CB488~492: elapsed = tick.saturating_sub(pick_start); grace_period = elapsed < tps*5; in_battle = plan 태그==9(Battle)
// CB493: visible_enemies_in_line = iter_champions(1-team).filter(closure s0_0(game, ctx, context, blackboard, player, line))[호출 count 조각].count()
// CB498: near_allies = (0..5).filter(closure s1_0(cache, ctx, player, line))[호출 count 조각].count()
// CB510~513: if visible_enemies_in_line > 2 → outcome(3); clear(28); FP; return
// CB518: if !(grace_period || in_battle) {
//   CB519~523: if visible_enemies_in_line == 2 && near_allies < 3 → outcome(3); clear(29); FP; return
//   CB527~531: if visible_enemies_in_line == 0 → outcome(3); clear(30); FP; return
//   CB535~539: if near_allies < 2 → outcome(4); clear(31); FP; return }
// CB544: if ready → return
// CB545: allies_on_line = (0..5).filter(closure s2_0(cache, ctx, player, line))[호출 count 조각].count()
// CB551~552: if visible_enemies_in_line != 0 && allies_on_line > 1 → self.objective = ComebackPick{line, ready=true}(store i8 11 @0x41f, line @0x420, 1 @0x421); return   else return
// [분기 CB454~474 — my_kill 성공 후 clear 대신 다음 목표 선택] ※IR 상 4281(FP) 뒤 4289 → 이어짐:
//   CB454: epic_unavailable = moba.epic.live_list.len(@0x1a8)==0 || self.obj_spawn.epic_giveup_tick.is_some()(@0x50 태그)
//   CB456~457: serpen_available = serpen.live_list.len(@0x1d8)!=0 && self.obj_spawn.serpen_giveup_tick.is_none()(@0x60==0)
//   CB459~464: if !epic_unavailable → objective = Morgard{Hunt(3), with_battle=true}(store i8 0/3/1); chat(38 MorgardHunt); return
//   CB465~470: else if serpen_available → objective = Serpen{Hunt(3), with_battle=true}(store i8 1/3/1); chat(29 SerpenHunt); return
//   CB471~474: else if let Some(press_line) = check_press_tower_opportunity(version, rnd, player, data, self, None(i8 -1), debug)[호출] → objective=PressTower(press_line)(store i8 10); press_tower_start_tick(@0x358)=tick; chat(45 PressTower{line,0}); return   else return
// ===== aux — 아웃오브라인 count/술어 조각 9개 (담당 함수에서 호출되는 인라인 핸들러의 클로저 · 전부 m09.ll)
// [gank s_0 · m09 68054~68332] ally_minions_in_range = Filter<Chain<Copied<Iter<&Entity>>×2>(iter_minions 결과), closure#1>::count → 내부 fold 는 m06.ll:7310(Chain::fold) → m11.ll:24927(Copied::fold): 술어 = Effect::is_in_range(tower_attack, enemy_tower, minion)[호출] (obj_handlers.rs:1197)
// [gank s0_0 · m09 68335~68512] near_allies = (0..5).filter(closure#2 @1205~1208: player_champion[team][p].is_some_and(|c| map_regions::is_near_line(ctx, c.x, c.y, line)[호출] && c.hp*100/max_hp > 39)).count()
// [gank s1_0 · m09 66806~66971] near_enemies = iter_champions(1-team).filter(closure#3 @1212~1214: is_near_line(ctx, e.x, e.y, line)[호출] && Blackboard::is_recent_visible(&blackboard[team], game, ctx, player, e)[호출]).count()
// [press_tower closure#0 · m09 3152~3340] engaged_at_tower(tower.x, tower.y) = cache.champions(team, pool)[호출 sret 32B].iter().any(|c| distance_sq(c, tower) < 90000000001(=300000²+1)) || cache.minions(team, pool)[호출].iter().any(|m| distance_sq(m, tower) < 40000000001(=200000²+1))   (obj_handlers.rs:182~187 · bumpalo Vec 2개 drop)
// [press_tower s_0 · m09 67142~67239] healthy_allies = iter_champions(team).filter(closure#1 @208: hp*100/max_hp > 39).count()
// [press_tower s0_0 · m09 66974~67139] near_enemies = iter_champions(enemy_team).filter(closure#2 @219~221: is_recent_visible(...)[호출] && is_near_line(ctx, e.x, e.y, line)[호출]).count()
// [comeback s0_0 · m09 67242~67407] visible_enemies_in_line = iter_champions(1-team).filter(closure#2 @493~495: is_recent_visible[호출] && is_near_line[호출]).count()
// [comeback s1_0 · m09 68515~68732] near_allies = (0..5).filter(closure#3 @498~505: player_champion[team][p].is_some_and(|c| hp%>39 && (is_near_line(ctx,c.x,c.y,line)[호출] || match line { Top(0) → is_top_side: (setting.height(@0x12c0) − c.y) >= c.x · Mid(1) → map_regions::is_near_mid_line(ctx, c.x, c.y)[호출] · Bottom(2) → is_bottom_side: (height − c.y) <= c.x }))).count()
// [comeback s2_0 · m09 68735~68912] allies_on_line = (0..5).filter(closure#4 @545~548: player_champion[team][p].is_some_and(|c| hp%>39 && is_near_line(ctx,c.x,c.y,line)[호출])).count()

```

**`mem` 메모리 접근 98건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | TeamPlan | 0x41f | objective | r | Option<MainObjective> 태그(3B 중 0번): 0 Morgard·1 Serpen·2 Defense·3 DefenseLine·4 Nexus·5 PressEpic·6 SplitEpic·7 Repair·8 Gank·9 Dive·10 PressTower·11 ComebackPick·**0xFF None**(IR 정본 `store i8 -1`·`icmp eq -1`) | 3 | OK |  |
| 1 | TeamPlan | 0x420 | objective.phase / .line | r | Morgard/Serpen 은 ObjectPhase(0 None·1 Setup·2 Assemble·3 Hunt), 라인형 variant 는 LineType | 4 | OK |  |
| 2 | TeamPlan | 0x421 | objective.with_battle / ComebackPick.ready | r | bool | 4 | OK |  |
| 3 | TeamPlan | 0x418 | steal_action@tag | r | 2=Commit 이면 규율 해제(D355) | 4 | OK |  |
| 4 | TeamPlan | 0x149 | objective_discipline.kind | r | 니치 태그: 0 SafeWait·1 HardDisengage·2 None (kind 필드와 동일 바이트) | 4 | OK |  |
| 5 | TeamPlan | 0x140 | objective_discipline.until_tick | r |  | 4 | OK |  |
| 6 | TeamPlan | 0x148 | objective_discipline.target | r | JungleType | 4 | OK |  |
| 7 | TeamPlan | 0x90 | objective_misunderstanding@tag / .opposite_group_start_tick@tag | r | i64 -1 = None; 0/1 = 내부 Option 태그 | 4 | OK |  |
| 8 | TeamPlan | 0x98 | objective_misunderstanding.opposite_group_start_tick.0 | r |  | 4 | OK |  |
| 9 | TeamPlan | 0xa0 | objective_misunderstanding.solo_hunt_start_tick@tag | r |  | 4 | OK |  |
| 10 | TeamPlan | 0xa8 | objective_misunderstanding.solo_hunt_start_tick.0 | r |  | 4 | OK |  |
| 11 | TeamPlan | 0xb0 | objective_misunderstanding.accepted_tick | r | marker_age 계산 | 4 | OK |  |
| 12 | TeamPlan | 0xb8 | objective_misunderstanding.target | r | JungleType | 4 | OK |  |
| 13 | TeamPlan | 0x50 | obj_spawn.epic_giveup_tick@tag | r | is_some/is_none 판정(M790·S645·CB454) | 4 | OK |  |
| 14 | TeamPlan | 0x60 | obj_spawn.serpen_giveup_tick | r |  | 4 | OK |  |
| 15 | TeamPlan | 0xc0 | chats.cap | r | push 인라인(용량 검사) | 4 | OK |  |
| 16 | TeamPlan | 0xc8 | chats.ptr | r |  | 4 | OK |  |
| 17 | TeamPlan | 0xd0 | chats.len | r |  | 4 | OK |  |
| 18 | TeamPlan | 0xe0 | comeback_pick_outcomes.ptr | r | last_mut | 4 | OK |  |
| 19 | TeamPlan | 0xe8 | comeback_pick_outcomes.len | r |  | 4 | OK |  |
| 20 | TeamPlan | 0x230 | vision.last_visible_pos[i] | r | @0x230+16p (p=0..4) — D395 closure | 4 | OK |  |
| 21 | TeamPlan | 0x2d0 | vision.last_checked_ticks[i] | r | @0x2d0+8p — D404 elapsed | 4 | OK |  |
| 22 | TeamPlan | 0x350 | gank_start_tick | r | G1143·DV108·DV129 | 4 | OK |  |
| 23 | TeamPlan | 0x358 | press_tower_start_tick | r | PT168 | 4 | OK |  |
| 24 | TeamPlan | 0x360 | comeback_pick_start_tick | r | CB415 | 4 | OK |  |
| 25 | TeamPlan | 0x370 | comeback_pick_success_count | r | +1 갱신용 읽기 | 4 | OK |  |
| 26 | PlayerState | 0x930 | info.team | r | 16회 | 4 | OK |  |
| 27 | PlayerState | 0x9c0 | info.position@tag | r | i32 Position(1=Jungle) | 4 | OK |  |
| 28 | OperationData | 0x0 | cache | r | &AbstractGameWithCache | 4 | OK |  |
| 29 | OperationData | 0x8 | context | r | &GameContext | 4 | OK |  |
| 30 | OperationData | 0x10 | blackboard | r | &[Blackboard;2] — [team] 로 인덱스 | 4 | OK |  |
| 31 | GoalData | 0x98 | epic.epic_ally_tick | r | D408 (select 로 0x98/0xd0 택일 → gep 에 152/208 로 등장) | 4 | OK |  |
| 32 | GoalData | 0xd0 | serpen.epic_ally_tick | r |  | 4 | OK |  |
| 33 | GoalData | 0x90 | epic.epic_enemy_killed_tick | r | M741 | 4 | OK |  |
| 34 | GoalData | 0xa0 | epic.epic_ally_killed_tick | r | M741 | 4 | OK |  |
| 35 | GoalData | 0xc8 | serpen.epic_enemy_killed_tick | r | S607 | 4 | OK |  |
| 36 | GoalData | 0xd8 | serpen.epic_ally_killed_tick | r | S607 | 4 | OK |  |
| 37 | BigPlan | 0x0 | plan@tag(니치 i64) | r | 6=DeathMatchBattle(암묵)·7 PassiveJungle·8 ActiveRecall·9 Battle·10 LineGanker | 4 | OK |  |
| 38 | BigPlan | 0x50 | PassiveJungle.0.team | r | L964 (enum+8+0x48) | 4 | OK |  |
| 39 | BigPlan | 0x68 | PassiveJungle.0.jungle | r | L964 JungleType | 4 | OK |  |
| 40 | AbstractGameWithCache | 0x0 | game.data_ptr / game.vtable_ptr | r | vtable 슬롯 0x28 tick·0x40 get_game_mode·0x130 kill_logs·0x1f0 get_entity_by_id | 4 | OK |  |
| 41 | AbstractGameWithCache | 0x1e0 | player_champion | r | @0x1e0+40*team+8*pos | 4 | OK |  |
| 42 | AbstractGameWithCache | 0x180 | top_tower[]/top_tower2[]/mid_tower[]/mid_tower2[]/bottom_tower[]/bottom_tower2[] | r | 0x180+0x20*line(+0x10 for tower2)+8*team | 4 | 오귀속(사전은 다른 필드를 준다) |  |
| 43 | AbstractGameWithCache | 0x130 | twin_towers[team] | r | bumpalo Vec: ptr @+0, len @+24 (DL62) | 4 | OK |  |
| 44 | GameContext | 0x8 | setting | r | &GameSetting: tick_per_second(@0x12f8)·height(@0x12c0, aux) | 4 | OK |  |
| 45 | GameContext | 0x20 | map | r | &MapDef → camp_pos | 4 | OK |  |
| 46 | GameContext | 0x38 | tutorial | r | TutorialType: spawn_epic={0,7,8} spawn_serpen={0,5,7,8} player_count/valid_lines 인라인 | 4 | OK |  |
| 47 | GameContext | 0x3b | debug | r | 디버그 문자열 push 게이트 | 4 | OK |  |
| 48 | GameContext | 0x0 | pool | r | DL70 champions(…, pool) — context+0x0 을 %2702 로 로드 | 4 | OK |  |
| 49 | MobaMode | 0x1a0 | jungle_runner.epic.live_list.ptr | r | D365 첫 원소 id | 4 | OK |  |
| 50 | MobaMode | 0x1a8 | jungle_runner.epic.live_list.len | r | is_empty 판정 다수 | 4 | OK |  |
| 51 | MobaMode | 0x1b0 | jungle_runner.epic.next_respawn_tick | r | L927·M700·M760 | 4 | OK |  |
| 52 | MobaMode | 0x1d0 | jungle_runner.serpen.live_list.ptr | r |  | 4 | OK |  |
| 53 | MobaMode | 0x1d8 | jungle_runner.serpen.live_list.len | r |  | 4 | OK |  |
| 54 | MobaMode | 0x1e0 | jungle_runner.serpen.next_respawn_tick | r |  | 4 | OK |  |
| 55 | MobaMode | 0x240 | epic_minion_buff_time[team] | r | PE317·SE386 remain_epic_time 인라인 | 4 | OK |  |
| 56 | MobaMode | 0x18 | jungle_runner | r | L964 get_jungle_live_list 수신자 | 4 | OK |  |
| 57 | Entity | 0x670 | hp | r | hp*100/max_hp 비율 다수 | 4 | OK |  |
| 58 | Entity | 0x628 | stat_cached.hp (max_hp) | r | 0 이면 div_by_zero 패닉 | 4 | OK |  |
| 59 | Entity | 0x660 | x | r |  | 4 | OK |  |
| 60 | Entity | 0x668 | y | r |  | 4 | OK |  |
| 61 | Entity | 0x640 | stat_cached.move_speed | r | D403 | 4 | OK |  |
| 62 | Entity | 0x5c0 | id | r | debug HashMap 키 | 4 | OK |  |
| 63 | Entity | 0x0 | team@tag (0 Player/1 Neutral) | r | is_visible_from 인라인 | 4 | OK |  |
| 64 | Entity | 0x8 | team@Player.0 | r | 팀 인덱스 | 4 | OK |  |
| 65 | Entity | 0x38 | visible_state | r | 0=Visible 요구 (stride 24) | 4 | OK |  |
| 66 | Entity | 0x68 | ty@tag | r | 2 Tower / 1 Minion (G1177~1180) | 4 | OK |  |
| 67 | Entity | 0x88 | ty@Tower.info.nearest_enemy@tag | r | G1177 | 4 | OK |  |
| 68 | Entity | 0x98 | ty@Tower.info.nearest_enemy.1 (id) | r | G1178 | 4 | OK |  |
| 69 | Entity | 0x490 | attack_effect (Some 페이로드) | r | tower_attack (G1193) | 4 | OK |  |
| 70 | Entity | 0x4c0 | attack_effect@tag | r | i32 -1 = None | 4 | OK |  |
| 71 | Blackboard | 0x0 | top_minion_state / mid(@0x28) / bottom(@0x50) | r | PT175~200: .from_mid(@+0x10 i64)·.minion_count(@+0x20 i32) | 4 | OK |  |
| 72 | KillLog | 0x18 | tick | r | CB416 (48B stride, rev 순회 · gep -24/-16/-4 로 등장) | 4 | OK |  |
| 73 | KillLog | 0x20 | killer_team | r |  | 4 | OK |  |
| 74 | KillLog | 0x2c | killed_position | r | Position i32 | 4 | OK |  |
| 75 | GameSetting | 0x12f8 | tick_per_second | r | tps | 4 | OK |  |
| 76 | DebugFrameData | 0xa0 | infos | r | HashMap<usize,Vec<String>> entry/or_insert | 4 | OK |  |
| 77 | TeamPlan | 0x41f | objective | w | 76 지점. 이 함수의 주 출력. Morgard/Serpen 은 @0x420 phase(1 Setup/3 Hunt)·@0x421 with_battle=1 을 같이 씀(R691 은 i16 store 259/257). 라인형은 @0x420 에 line | 4 | OK | -1(None)·0 Morgard·1 Serpen·2 Defense·4 Nexus·7 Repair·8 Gank·9 Dive·10 PressTower·11 ComebackPick |
| 78 | TeamPlan | 0x420 | objective.phase / .line | w | 22 지점 | 4 | OK | 1·3 / LineType |
| 79 | TeamPlan | 0x421 | objective.with_battle / ComebackPick.ready | w | 11 지점 | 4 | OK | 1 |
| 80 | TeamPlan | 0x210 | mf_obj_clear.0 (사이트 코드) | w | mf_note_obj_clear 인라인 42 지점 — 목표 해제 사유 코드(관측 전용, simulator MF_OBJCLR_NAMES 1:1) | 4 | OK | 2·3·4·5·6·7·8·9·10·11·12·13·14·15·16·17·18·19·20·21·22·23·24·25·26·27·28·29·30·31·32·33·34·35·36·37·38·39·40·46 |
| 81 | TeamPlan | 0x218 | mf_obj_clear.1 (tick) | w | 0x210 과 쌍 | 4 | OK | game.tick() |
| 82 | TeamPlan | 0x130 | objective_discipline.wait_pos.0 | w | D451·D458 | 4 | OK | wait_pos.x |
| 83 | TeamPlan | 0x138 | objective_discipline.wait_pos.1 | w |  | 4 | OK | wait_pos.y |
| 84 | TeamPlan | 0x140 | objective_discipline.until_tick | w |  | 4 | OK | tick+tps(*3 HardDisengage) 신규 / max(tick+tps*2\|tps/2, 기존) 갱신 |
| 85 | TeamPlan | 0x148 | objective_discipline.target | w |  | 4 | OK | JungleType 4/5 |
| 86 | TeamPlan | 0x149 | objective_discipline.kind(kind) | w | None 저장 9 지점(D337·351·357·362·366·376·439) + Some 2 지점; @0x14a 패딩 6B memcpy 동반 | 4 | OK | 0 SafeWait·1 HardDisengage·2 None |
| 87 | TeamPlan | 0x90 | objective_misunderstanding@tag / .opposite_group_start_tick | w | R608·612·641·682·696 | 4 | OK | -1 None / 0·1(+@0x98 tick) |
| 88 | TeamPlan | 0xa0 | objective_misunderstanding.solo_hunt_start_tick | w | R641·682 | 4 | OK | 0·1(+@0xa8 tick) |
| 89 | TeamPlan | 0x50 | obj_spawn.epic_giveup_tick@tag | w | M774 (에픽 기브업) | 4 | OK | Some(tick) (@0x50=1, @0x58=tick) |
| 90 | TeamPlan | 0x60 | obj_spawn.serpen_giveup_tick | w | S629·PE298·SE367 | 4 | OK | Some(tick) (@0x60=1, @0x68=tick) |
| 91 | TeamPlan | 0xd0 | chats.len (+ chats.ptr[len] 원소 24B: 태그 @+0, 라인/사유 @+1, +8=0) | w | 31 지점 · Chat 태그 17 Cancel{reason}·23 Repair·25 SerpenSetup·29 SerpenHunt·31 SerpenGiveUp{reason}·34 MorgardSetup·38 MorgardHunt·40 MorgardGiveUp·42 AttackNexus{line}·43 DefenseNexus·44 GankDive{line}·45 PressTower{line} — len==cap 이면 Vec<Chat>::grow_one 호출(힙 재할당) | 4 | OK | len+1 |
| 92 | TeamPlan | 0x358 | press_tower_start_tick | w | M795·S650·CB473 | 4 | OK | tick |
| 93 | TeamPlan | 0x370 | comeback_pick_success_count | w | CB448 | 4 | OK | +1 |
| 94 | TeamPlan | 0xe0 -> comeback_pick_outcomes.ptr[len-1]+4 | comeback_pick_outcomes.last().1 | w | 힙 원소 쓰기(u8 outcome 코드). len==0 이면 건너뜀 | 4 | 부분일치(첫 성분(컨테이너)만 맞고 **마지막 성분이 다르다** — 사람이 봐야 ) | 1·2·3·4·5·6 |
| 95 | BigPlan | 0x0 | plan@tag | w | 128 지점(cleanuppad 포함) — 항상 drop_glue(BigPlan) 선행 → 기존 variant 의 Vec/Box 해제(HEAP_SUBST 재료) | 4 | OK | 2 ForcePassive / 8 ActiveRecall |
| 96 | BigPlan | 0x60 | Battle.0.sub_goal@tag | w | M778·S637 — BattlePlan+0x58 sub_goal: BattleSubPlanGoal 태그 4 = RunAway (tcxdict --enum 확정) — 기브업 시 진행 중 교전을 도주로 전환 | 3 | OK | 4 |
| 97 | DebugFrameData | 0xa0 | infos[champ.id].push(String) | w | context.debug 일 때만 · rustc_entry/or_insert/format_inner/push_mut 호출 = String·Vec 힙 할당 | 4 | 부분일치(첫 성분(컨테이너)만 맞고 **마지막 성분이 다르다** — 사람이 봐야 ) | format! 문자열 3종 |

**`consts` 상수 52건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev | folded_from |
|---|---|---|---|---|---|---|
| 0 | 19600000001 | 911 | 임계 | 140000²+1 — near_ally_count: 내 챔프 기준 아군 반경(제곱비교 ult) [objective_discipline.rs] [인라인 콜리 줄 383 → 루트 911 · 20차 G] | 4 |  |
| 1 | 28900000001 | 911 | 임계 | 170000²+1 — near_enemy_count 반경 [인라인 콜리 줄 387 → 루트 911 · 20차 G] | 4 |  |
| 2 | 32400000001 | 911 | 임계 | 180000²+1 — camp_ally_count: 캠프 기준 아군 반경 [인라인 콜리 줄 389 → 루트 911 · 20차 G] | 4 |  |
| 3 | 36100000001 | 393 | 임계 | 190000²+1 — camp_visible_enemy_count 반경 · R667 my_near_target 반경(같은 값) | 4 |  |
| 4 | 180000 | 402 | 계수 | D402 d=distance(last_pos,camp).saturating_sub(180000) 접근 여유 · R651 v23_healthy_allies_near_point(target_camp) 반경 | 4 |  |
| 5 | 200000 | 652 | 미상 | R652 v23_healthy_allies_near_point(opposite_camp) 반경 [team_plan.rs] | 4 |  |
| 6 | 40 | 651 | 임계 | v23_healthy_allies_near_point 의 hp% 하한 인자(651·652) · D434 hp_ratio < 40 이면 release 불가 | 4 |  |
| 7 | 34 | 382 | 임계 | hp*100/max_hp > 34 — 규율 카운트 4종의 '건강' 기준 | 4 |  |
| 8 | 50 | 911 | 임계 | D397 안 보이는 적 후보 hp% < 50 제외 [인라인 콜리 줄 397 → 루트 911 · 20차 G] | 4 |  |
| 9 | 36 | 374 | 임계 | objective_hp_ratio < 36 = execute_window(막타 창) → 규율 해제 | 4 |  |
| 10 | 35 | 419 | 임계 | hp_ratio < 35 위험 분기 진입 | 4 |  |
| 11 | 25 | 420 | 임계 | hp_ratio < 25 최위험 · G1160 갱 대상 라이너 hp% < 25 취소 | 4 |  |
| 12 | 54 | 911 | 임계 | objective_hp_ratio > 54 (SafeWait 조건 D426·D428) [인라인 콜리 줄 426 → 루트 911 · 20차 G] | 4 |  |
| 13 | 49 | 911 | 임계 | objective_hp_ratio > 49 (SafeWait 조건 D427) [인라인 콜리 줄 427 → 루트 911 · 20차 G] | 4 |  |
| 14 | 2 | 413 | 임계 | D413 `shl %tps, 2` = tps*4: ally_finish_tick ∈ [1, 4초] · R678 opposite_group 4초 · (2 자체로도 등장: near_ally<2·camp_ally+2 등) | 4 | 4 |
| 15 | 1 | 741 | 임계 | `shl %tps, 1` = tps*2: M741/S607 킬 틱 min > 2초 · D447 HardDisengage 연장 2초 · `lshr %tps,1` = tps/2 SafeWait 연장 | 4 | 2 |
| 16 | 3 | 419 | 태그 | CB419/431 `shl %tps, 3` = tps*8: 킬 로그 8초 창 · (3 자체: until=tick+tps*3 HardDisengage 신규(D454 `mul 3`), Hunt 태그, near_allies<3) | 4 | 8 |
| 17 | 15 | 923 | 계수 | tps*15: 오브젝트 리스폰 잔여 > 15초면 목표 해제(L923·M700·M760·S585) · CB479 ready=false 타임아웃 15초 | 4 |  |
| 18 | 20 | 1143 | 계수 | tps*20: 갱 목표 20초 초과 해제(G1143) · DV129 다이브→갱 복귀 20초 · CB479 ready=true 타임아웃 20초 | 4 |  |
| 19 | 30 | 108 | 계수 | tps*30: 다이브(DV108)·타워 압박(PT168) 30초 초과 해제 | 4 |  |
| 20 | 5 | 489 | 태그 | tps*5: CB489 grace_period(픽 시작 5초 이내) `mul %tps, 5` — shl 피연산자 아님(range 상한 5 = 플레이어 5칸) | 4 |  |
| 21 | 6 | 680 | 임계 | tps*6: R680 solo_hunt 6초 경과 · outcome 코드 6(넥서스 선점) | 4 |  |
| 22 | 100 | 372 | 계수 | hp 백분율 환산 계수 | 4 |  |
| 23 | 14400000001 | 967 | 임계 | 120000²+1 — L967 skip_for_camp: 정글 캠프 근처면 귀환 보류 [team_plan.rs] | 4 |  |
| 24 | 150000 | 745 | 미상 | should_disengage_object_hunt 반경 인자(M745·S611) [objective_handlers.rs] | 4 |  |
| 25 | 41 | 998 | 임계 | PE281 low_hp_allies: hp% < 41 [인라인 콜리 줄 281 → 루트 998 · 20차 G] | 4 |  |
| 26 | 29 | 321 | 임계 | PE321 live_ally_count: hp% > 29 · Chat 29 SerpenHunt | 4 |  |
| 27 | 39 | 118 | 임계 | hp% > 39 — DV118·aux count 술어 5종(gank near_allies·press_tower healthy_allies·comeback near_allies/allies_on_line) | 4 |  |
| 28 | 90000000001 | 1010 | 임계 | 300000²+1 — DL71 타워 근처 적 챔프 · aux PT184 engaged_at_tower 아군 챔프 [인라인 콜리 줄 71 → 루트 1010 · 20차 G] | 4 |  |
| 29 | 40000000001 | 186 | 미상 | 200000²+1 — aux PT186 engaged_at_tower 아군 미니언 반경 | 4 |  |
| 30 | -1000 | 1016 | 임계 | PT176 minion_state.from_mid < -1000 (해당 라인 미니언 열세) → engaged 아니면 해제 [인라인 콜리 줄 176 → 루트 1016 · 20차 G] | 4 |  |
| 31 | -2999 | 1016 | 임계 | PT200 다른 라인 from_mid < -2999 && minion_count < -2 → 해제(15) [인라인 콜리 줄 200 → 루트 1016 · 20차 G] | 4 |  |
| 32 | -2 | 200 | 임계 | PT200 minion_count < -2 | 4 |  |
| 33 | -1 | 937 | 태그 | ★Option<MainObjective>/Option<LineType>/handle_nexus_attack 반환의 None 메모리값 = 0xFF (IR 정본; 이론값 12/3 아님) · objective_misunderstanding None 태그(i64 -1) · attack_effect None(i32 -1) | 3 |  |
| 34 | 8 | 972 | 태그 | BigPlan 태그 8 = ActiveRecall(L972 재기록·L956 판정) · MainObjective 8 = Gank | 4 |  |
| 35 | 259 | 691 | 산출값 | R691 i16 store 0x0103 = phase Hunt(3)+with_battle(1) | 4 |  |
| 36 | 257 | 691 | 산출값 | R691 i16 store 0x0101 = phase Setup(1)+with_battle(1) | 4 |  |
| 37 | 4 | 778 | 태그 | M778/S637 plan.battle.sub_goal = BattleSubPlanGoal::RunAway(4) · MainObjective 4 Nexus · JungleType 4 Morgard · outcome 4 | 4 |  |
| 38 | 9 | 776 | 태그 | BigPlan 태그 9 Battle(M776·S635·CB490·D17) · MainObjective 9 Dive(G1219) | 4 |  |
| 39 | 10 | 1137 | 태그 | BigPlan 태그 10 LineGanker(G1137 → ForcePassive) · MainObjective 10 PressTower | 4 |  |
| 40 | 7 | 963 | 태그 | BigPlan 태그 7 PassiveJungle(L963) · MainObjective 7 Repair | 4 |  |
| 41 | 11 | 552 | 태그 | MainObjective 11 ComebackPick(CB552) | 4 |  |
| 42 | 44 | 946 | 산출값 | Chat 44 GankDive{line} (G1220) [인라인 콜리 줄 1220 → 루트 946 · 20차 G] | 4 |  |
| 43 | 17 | 1168 | 산출값 | Chat 17 Cancel{reason 1=라이너 저hp/2=라이너 없음} (G1168) | 4 |  |
| 44 | 42 | 712 | 산출값 | Chat 42 AttackNexus{line} (nexus_attack 성공 6곳) | 4 |  |
| 45 | 43 | 723 | 산출값 | Chat 43 DefenseNexus (defense role/chosen 8곳) | 4 |  |
| 46 | 45 | 796 | 산출값 | Chat 45 PressTower{line} (M796·S651·CB474) | 4 |  |
| 47 | 38 | 822 | 산출값 | Chat 38 MorgardHunt (M822·CB464) · mf 코드 38 | 4 |  |
| 48 | 23 | 287 | 산출값 | Chat 23 Repair (PE287) · mf 코드 23 | 4 |  |
| 49 | 31 | 641 | 산출값 | Chat 31 SerpenGiveUp{reason} (S641) · mf 코드 31 | 4 |  |
| 50 | 46 | 984 | 산출값 | mf_obj_clear 코드 46 (L984 v3_epicops_repair_need 없음) | 4 |  |
| 51 | 12 | 162 | 산출값 | mf 코드 12 (PT162 적 타워 없음) — ⚠Option None 값이 아님 | 4 |  |

**`knobs` 조정점 22건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 규율 '건강' hp% 기준 | objective_discipline.rs:382~393 (×4) | 34 | 올리면 저hp 아군/적이 카운트에서 빠져 near/camp 카운트가 줄어 SafeWait/HardDisengage 판정이 덜/더 나옴(아군 감소=HardDisengage↑, 적 감소=규율↓) | 4 | 기존 |
| 1 | 막타 창 objective hp% | objective_discipline.rs:374 | 36 | 올리면 더 이른 시점부터 규율이 해제돼 무조건 오브젝트에 붙는다 | 4 | 기존 |
| 2 | 위험 hp 단계 | objective_discipline.rs:419~420 / 434 | 35 / 25 / 40 | 올리면 HardDisengage(이탈) 가 더 자주, release 가 덜 발생 | 4 | 기존 |
| 3 | SafeWait 오브젝트 hp 하한 | objective_discipline.rs:426~428 | 54 / 49 | 내리면 오브젝트가 낮은 hp 여도 대기(SafeWait)로 빠져 막타 경쟁이 늦어짐 | 4 | 기존 |
| 4 | 규율 유지시간 | objective_discipline.rs:447 / 454 | 신규 tps·3tps / 연장 tps/2·2tps | 올리면 상태가 오래 붙어 오브젝트 복귀가 늦어짐 | 4 | 기존 |
| 5 | 아군 막타 임박 창 | objective_discipline.rs:413 | 4*tps | 올리면 ally_finish_tick 이 더 이른 시점부터 규율을 무효화 | 4 | 기존 |
| 6 | 안 보이는 적 근접 판정 여유 | objective_discipline.rs:402 | 180000 | 올리면 미확인 적을 덜 세어 camp_enemy_pressure↓ → release 쉬움 | 4 | 기존 |
| 7 | 오브젝트 사망 해제 임계 | team_plan.rs:923 · objective_handlers.rs:700/760/585 | tps*15 | 내리면 리스폰까지 15초 미만이어도 목표를 유지하지 않고 해제 | 4 | 기존 |
| 8 | 오해 수리 트리거 | team_plan.rs:678/680 | tps*4 / tps*6 | 내리면 반대 오브젝트로 더 빨리 목표를 갈아탐 | 4 | 기존 |
| 9 | 오해 수리 반경 | team_plan.rs:651~652 / 667 | 180000 / 200000 / 190000² | 반경↑ = 더 넓게 아군을 '모임'으로 인정 | 4 | 기존 |
| 10 | 갱 목표 수명 | objective_handlers.rs:1143 | tps*20 | 올리면 갱 목표를 더 오래 유지 | 4 | 기존 |
| 11 | 갱→다이브 전환 조건 | objective_handlers.rs:1160/1200/1218 | 라이너 hp≥25 · 미니언≥2 또는 hp>타워 1타 · near_allies>1 && >near_enemies | 완화하면 다이브(9) 목표 발행↑ | 4 | 기존 |
| 12 | 다이브 수명/복귀 | objective_handlers.rs:108/129 | tps*30 / tps*20 | 30초 초과 해제 · 열세면 20초 초과 해제, 아니면 Gank 복귀 | 4 | 기존 |
| 13 | 귀환 보류 캠프 반경 | team_plan.rs:967 | 120000² | 올리면 정글 캠프 근처에서 귀환을 더 자주 미룸 | 4 | 기존 |
| 14 | 에픽 압박 저hp 수리 전환 | objective_handlers.rs:281~285 / 321~324 | hp<41 이 2명 이상 && 생존 아군≤적 / hp>29 아군 < 적 | 완화하면 PressEpic 중 Repair(7) 전환↑ | 4 | 기존 |
| 15 | 타워 압박 해제 조건 | objective_handlers.rs:168/176/200/211/225 | 30초 · from_mid<-1000 · 타 라인 from_mid<-2999&&count<-2 · 건강아군≤적 · 근접적>1 | 완화하면 PressTower 유지↑ | 4 | 기존 |
| 16 | engaged_at_tower 반경 | objective_handlers.rs:184/186 | 300000² 챔프 / 200000² 미니언 | 올리면 미니언 열세여도 압박 유지 | 4 | 기존 |
| 17 | 컴백 픽 타임아웃/유예 | objective_handlers.rs:479/489 | ready?20:15 초 / 5초 | 타임아웃↑ 이면 픽 대기 길어짐 | 4 | 기존 |
| 18 | 컴백 픽 킬 판정 창 | objective_handlers.rs:419/431 | tps*8 | 킬 로그를 더 오래 소급 인정 | 4 | 기존 |
| 19 | 컴백 픽 인원 조건 | objective_handlers.rs:510/519/535/551 | 적>2 해제 · 적==2&&아군<3 해제 · 적==0 해제 · 아군<2 해제 · ready 는 적≠0&&아군>1 | 완화하면 ComebackPick 유지/ready 전환↑ | 4 | 기존 |
| 20 | 이탈 판정 반경(오브젝트 사냥) | objective_handlers.rs:745/611 | 150000 | should_disengage_object_hunt 에 넘기는 반경 — 올리면 더 넓은 범위의 적을 이탈 근거로 봄(콜리 의미는 미독) | 4 | 기존 |
| 21 | 기브업 후 압박 타워 라인 | objective_handlers.rs:793 / 648 | Morgard→Some(Bottom) / Serpen→Some(Top) | check_press_tower_opportunity 힌트 라인 | 4 | 기존 |

<details><summary>`callees` 피호출자 111건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | as_moba | game_core::GameMode::<'a>::as_moba | pub | fn(game_core::GameMode<'a/#0>) -> std::option::Option<&'a/#0 game_core::MobaMode> | game-core\src\simulation\game.rs:230 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 1 | attack_effect | game_core::Entity::attack_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1684 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 2 | calculate_nexus_defense_count | game_ai::plan_legacy::handler::calculate_nexus_defense_count | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> usize | game-ai\src\plan_legacy\handler.rs:2328 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 3 | camp_pos | game_core::MapDef::camp_pos | pub | fn(&game_core::MapDef, game_core::JungleType, bool) -> (u64, u64) | game-core\src\simulation\map_def.rs:207 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | champions | game_core::AbstractGameWithCache::<'a, 'b>::champions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize, &'b/#1 bumpalo::Bump) -> bumpalo::collections::vec::Vec< &game_core::Entity> | game-core\src\simulation.rs:1896 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 5 | chats | game_ai::plan_legacy::types::BigPlan::chats | pub | fn(&mut game_ai::plan_legacy::types::BigPlan) -> std::option::Option<&mut std::vec::Vec<game_core::Chat, std::alloc::Global>> | game-ai\src\plan_legacy\types.rs:184 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 6 | check_epic_giveup | game_ai::plan_legacy::old::check_epic_giveup | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bool | game-ai\src\plan_legacy\old\epic.rs:109 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 7 | check_epic_hunt | game_ai::plan_legacy::old::check_epic_hunt | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &game_ai::plan_legacy::types::BigPlan, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bool | game-ai\src\plan_legacy\old\epic.rs:168 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 8 | check_epic_setup | game_ai::plan_legacy::old::check_epic_setup | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bool | game-ai\src\plan_legacy\old\epic.rs:16 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 9 | check_press_tower_opportunity | game_ai::plan_legacy::team_plan::check_press_tower_opportunity | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan, std::option::Option<game_core::LineType>, &mut game_core::DebugFrameData) -> std::option::Option<game_core::LineType> | game-ai\src\plan_legacy\team_plan.rs:1129 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 10 | check_serpen_giveup | game_ai::plan_legacy::old::check_serpen_giveup | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bool | game-ai\src\plan_legacy\old\serpen.rs:234 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 11 | check_serpen_hunt | game_ai::plan_legacy::old::check_serpen_hunt | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &game_ai::plan_legacy::types::BigPlan, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bool | game-ai\src\plan_legacy\old\serpen.rs:297 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 12 | check_serpen_setup | game_ai::plan_legacy::old::check_serpen_setup | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bool | game-ai\src\plan_legacy\old\serpen.rs:68 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 13 | clear | game_core::DataTable::<T>::clear | pub | fn(&mut game_core::DataTable<T/#0>) | game-core\src\data.rs:2259 | True | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 14 | debug | game_core::AbstractBanpickRunner::debug | pub | fn(&Self/#0, &[usize]) | game-core\src\banpick\base_runner.rs:411 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 15 | debug | <game_core::FactoredBanpickAgent as game_core::AbstractBanpickRunner>::debug | pub | fn(&game_core::FactoredBanpickAgent, &[usize]) | game-core\src\banpick\sgd_v2.rs:5891 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 16 | debug | <game_core::LogisticSGDBanpickAgent as game_core::AbstractBanpickRunner>::debug | pub | fn(&game_core::LogisticSGDBanpickAgent, &[usize]) | game-core\src\banpick\sgd.rs:1172 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 17 | distance | game_core::utils::distance | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:12 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 18 | distance_sq | game_core::utils::distance_sq | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:6 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 19 | distance_sq | game_core::Entity::distance_sq | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2157 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 20 | end_check | game_ai::end_check | in:game_ai | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, game_core::LineType, &mut game_core::DebugFrameData) -> bool | game-ai\src\lib.rs:1211 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 21 | expected_damage_target | game_core::Effect::expected_damage_target | pub | fn(&game_core::Effect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + , &game_core::Entity) -> usize | game-core\src\simulation\effect.rs:91 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 22 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 23 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 24 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 25 | get_game_mode | game_core::AbstractGame::get_game_mode | pub | fn(&Self/#0) -> game_core::GameMode | game-core\src\simulation.rs:78 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 26 | get_game_mode | <game_core::Game as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::Game) -> game_core::GameMode | game-core\src\simulation\game.rs:1707 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 27 | get_game_mode | <game_core::SingleLaneGame as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::SingleLaneGame) -> game_core::GameMode | game-core\src\simulation\game.rs:4248 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 28 | get_jungle_live_list | game_core::JungleRunner::get_jungle_live_list | pub | fn(&game_core::JungleRunner, game_core::JungleType, bool) -> std::vec::Vec<usize, std::alloc::Global> | game-core\src\simulation\entity\jungle.rs:743 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 29 | get_start_position | game_core::LineType::get_start_position | pub | fn(&game_core::LineType, &game_core::GameSetting, usize) -> (u64, u64) | game-core\src\simulation\state\player.rs:1013 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 30 | goal | game_ai::plan_legacy::types::BigPlan::goal | pub | fn(&game_ai::plan_legacy::types::BigPlan) -> game_core::BigGoal | game-ai\src\plan_legacy\types.rs:132 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 31 | handle_comeback_pick_objective | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_comeback_pick_objective | in:game_ai | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData, game_core::LineType, bool) -> bool | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:393 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 32 | handle_defense_line_objective | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_defense_line_objective | in:game_ai | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData, game_core::LineType) -> bool | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:46 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 33 | handle_defense_objective | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_defense_objective | in:game_ai | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData) -> bool | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:16 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 34 | handle_dive_objective | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_dive_objective | in:game_ai | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData, game_core::LineType) -> bool | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:88 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 35 | handle_epic_line_change | game_ai::plan_legacy::old::epic::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_epic_line_change | pub | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData, game_core::LineType) | game-ai\src\plan_legacy\old\epic.rs:684 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 36 | handle_gank_preprocess | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_gank_preprocess | in:game_ai | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, game_core::LineType) -> bool | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:1133 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 37 | handle_morgard_objective | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_morgard_objective | in:game_ai | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData, game_ai::plan_legacy::team_plan::ObjectPhase) -> bool | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:685 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 38 | handle_nexus_attack | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_nexus_attack | pub | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> std::option::Option<game_core::LineType> | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:1237 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 39 | handle_nexus_objective | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_nexus_objective | in:game_ai | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData, game_core::LineType) -> bool | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:29 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 40 | handle_none_or_gank_objective | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_none_or_gank_objective | in:game_ai | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData) -> bool | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:830 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 41 | handle_press_epic_objective | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_press_epic_objective | in:game_ai | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData, game_core::LineType) -> bool | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:253 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 42 | handle_press_tower_objective | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_press_tower_objective | in:game_ai | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData, game_core::LineType) -> bool | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:141 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 43 | handle_repair_objective | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_repair_objective | in:game_ai | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PlayerState, &game_core::OperationData) | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:7 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 44 | handle_serpen_objective | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_serpen_objective | in:game_ai | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData, game_ai::plan_legacy::team_plan::ObjectPhase) -> bool | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:558 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 45 | handle_split_epic_objective | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_split_epic_objective | in:game_ai | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData, game_core::LineType) -> bool | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:334 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 46 | i_am_chosen_defender | game_ai::plan_legacy::old::i_am_chosen_defender | pub | fn(&game_core::PlayerState, &game_core::OperationData, usize, &[usize]) -> bool | game-ai\src\plan_legacy\old\defense_nexus.rs:297 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 47 | in_epic | game_core::Blackboard::in_epic | pub | fn(&game_core::Blackboard, usize) -> bool | game-core\src\simulation\game\blackboard.rs:179 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 48 | in_serpen | game_core::Blackboard::in_serpen | pub | fn(&game_core::Blackboard, usize) -> bool | game-core\src\simulation\game\blackboard.rs:183 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 49 | is_empty | game_core::PatchSetting::is_empty | pub | fn(&game_core::PatchSetting) -> bool | game-core\src\patch_engine.rs:853 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 50 | is_empty | game_core::ModAiRegistry::is_empty | pub | fn(&game_core::ModAiRegistry) -> bool | game-core\src\mod_ai.rs:15 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 51 | is_empty | game_core::PublicChampionEvidence::is_empty | pub | fn(&game_core::PublicChampionEvidence) -> bool | game-core\src\banpick\public_evidence.rs:96 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 52 | is_end | game_ai::plan_legacy::old::BattlePlan::is_end | pub | fn(&game_ai::plan_legacy::old::BattlePlan) -> bool | game-ai\src\plan_legacy\old\battle.rs:2039 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 53 | is_ignored_well_enemy | game_ai::plan_legacy::old::is_ignored_well_enemy | pub | fn(usize, &game_core::PlayerState, &game_core::Entity) -> bool | game-ai\src\plan_legacy\old\fight_model.rs:754 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 54 | is_in_range | game_core::Effect::is_in_range | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\effect.rs:63 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 55 | is_near_line | game_core::is_near_line | pub | fn(&game_core::GameContext, u64, u64, game_core::LineType) -> bool | game-core\src\simulation\map_regions.rs:139 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 56 | is_near_mid_line | game_core::is_near_mid_line | pub | fn(&game_core::GameContext, u64, u64) -> bool | game-core\src\simulation\map_regions.rs:127 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 57 | is_recent_visible | game_core::Blackboard::is_recent_visible | pub | fn(&game_core::Blackboard, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\game\blackboard.rs:346 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 58 | is_visible_from | game_core::Entity::is_visible_from | pub | fn(&game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1481 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 59 | iter_champions | game_core::AbstractGameWithCache::<'a, 'b>::iter_champions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5759 ~ game_core[6a30]::simulation::{impl#5}::iter_champions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1904 | True | True | 3 | IR 호출 심볼 일치(토큰 경계) |
| 60 | iter_minions | game_core::AbstractGameWithCache::<'a, 'b>::iter_minions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5753 ~ game_core[6a30]::simulation::{impl#5}::iter_minions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1847 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 61 | jungle_runner | game_core::MobaMode::jungle_runner | pub | fn(&game_core::MobaMode) -> &game_core::JungleRunner | game-core\src\simulation\game.rs:213 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 62 | jungle_runner | game_core::AbstractGame::jungle_runner | pub | fn(&Self/#0) -> &game_core::JungleRunner | game-core\src\simulation.rs:119 | True | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 63 | kill_logs | game_core::AbstractGame::kill_logs | pub | fn(&Self/#0) -> &std::vec::Vec<game_core::KillLog, std::alloc::Global> | game-core\src\simulation.rs:141 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 64 | kill_logs | <game_core::Game as game_core::AbstractGame>::kill_logs | pub | fn(&game_core::Game) -> &std::vec::Vec<game_core::KillLog, std::alloc::Global> | game-core\src\simulation\game.rs:3768 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 65 | kill_logs | <game_core::SingleLaneGame as game_core::AbstractGame>::kill_logs | pub | fn(&game_core::SingleLaneGame) -> &std::vec::Vec<game_core::KillLog, std::alloc::Global> | game-core\src\simulation\game.rs:4027 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 66 | kind | game_view::ui::database_edit_ui::DbEditAppearanceTarget::kind | in:game_view::ui::database_edit_ui | fn(game_view::ui::database_edit_ui::DbEditAppearanceTarget) -> game_view::ui::athlete_appearance_popup::AppearanceTargetKind | game-view\src\ui\database_edit_ui.rs:100 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 67 | line_exists | game_ai::plan_legacy::rule_scope::line_exists | pub | fn(&game_core::GameContext, game_core::LineType) -> bool | game-ai\src\plan_legacy\rule_scope.rs:24 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 68 | mf_note_obj_clear | game_ai::plan_legacy::team_plan::TeamPlan::mf_note_obj_clear | in:game_ai | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, u8, usize) | game-ai\src\plan_legacy\team_plan.rs:196 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 69 | minion_state | game_core::Blackboard::minion_state | pub | fn(&game_core::Blackboard, game_core::LineType) -> &game_core::BrainMinionParameter | game-core\src\simulation\game\blackboard.rs:378 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 70 | minions | game_core::AbstractGameWithCache::<'a, 'b>::minions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize, &'b/#1 bumpalo::Bump) -> bumpalo::collections::vec::Vec< &game_core::Entity> | game-core\src\simulation.rs:1853 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 71 | moba | game_core::MapDef::moba | pub | fn(&game_core::GameSetting) -> game_core::MapDef | game-core\src\simulation\map_def.rs:67 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 72 | morgard_exists | game_ai::plan_legacy::rule_scope::morgard_exists | pub | fn(&game_core::GameContext) -> bool | game-ai\src\plan_legacy\rule_scope.rs:45 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 73 | nearest_enemy | game_core::Entity::nearest_enemy | pub | fn(&game_core::Entity) -> std::option::Option<usize> | game-core\src\simulation\entity.rs:1819 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 74 | need_defense_nexus | game_ai::plan_legacy::old::need_defense_nexus | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> bool | game-ai\src\plan_legacy\old\defense_nexus.rs:481 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 75 | objective_defense_role | game_ai::plan_legacy::old::objective_defense_role | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, game_core::JungleType, usize) -> game_ai::plan_legacy::old::DefenseRole | game-ai\src\plan_legacy\old\defense_nexus.rs:382 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 76 | objective_is_damaged | game_ai::plan_legacy::team_plan::objective_helpers::objective_is_damaged | in:game_ai | fn(&game_core::OperationData, game_core::JungleType) -> bool | game-ai\src\plan_legacy\team_plan\objective_helpers.rs:41 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 77 | player_count | game_core::GameRule::player_count | pub | fn(&game_core::GameRule) -> usize | game-core\src\data\match_info.rs:542 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 78 | player_count | game_core::TutorialType::player_count | pub | fn(&game_core::TutorialType) -> usize | game-core\src\simulation\game\runner.rs:294 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 79 | position | game_core::PlayerAiContext::<'a, 'b, 'r>::position | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> game_core::Position | game-core\src\mod_ai.rs:587 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 80 | position | game_core::transfer::TeamRosterPlanningContext::position | pub | fn(&game_core::transfer::TeamRosterPlanningContext, game_core::Position) -> std::option::Option<&game_core::transfer::PositionPlanningContext> | game-core\src\transfer\roster_context.rs:234 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 81 | push | <game_core::Staff as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::staff | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\staff.rs:38 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 82 | push | <game_core::Athlete as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:434 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 83 | push | <game_core::Contract as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:2498 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 84 | repair_misunderstood_objective | game_ai::plan_legacy::team_plan::TeamPlan::repair_misunderstood_objective | in:game_ai::plan_legacy::team_plan | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData) -> bool | game-ai\src\plan_legacy\team_plan.rs:603 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 85 | serpen_exists | game_ai::plan_legacy::rule_scope::serpen_exists | pub | fn(&game_core::GameContext) -> bool | game-ai\src\plan_legacy\rule_scope.rs:49 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 86 | serpen_giveup_chat_reason | game_ai::plan_legacy::old::serpen_giveup_chat_reason | pub | fn(&game_core::PlayerState, &game_core::OperationData) -> std::option::Option<game_core::SerpenGiveUpReason> | game-ai\src\plan_legacy\old\serpen.rs:217 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 87 | should_delay_morgard_for_wave_priority | game_ai::plan_legacy::team_plan::TeamPlan::should_delay_morgard_for_wave_priority | in:game_ai::plan_legacy::team_plan | fn(&game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData) -> bool | game-ai\src\plan_legacy\team_plan.rs:570 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 88 | should_delay_serpen_for_wave_priority | game_ai::plan_legacy::team_plan::TeamPlan::should_delay_serpen_for_wave_priority | in:game_ai::plan_legacy::team_plan | fn(&game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData) -> bool | game-ai\src\plan_legacy\team_plan.rs:574 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 89 | should_disengage_object_hunt | game_ai::should_disengage_object_hunt | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, (u64, u64), u64) -> bool | game-ai\src\fight_check.rs:1361 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 90 | should_recall_to_shop | game_ai::should_recall_to_shop | in:game_ai | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData) -> bool | game-ai\src\lib.rs:1642 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 91 | spawn_epic | game_core::TutorialType::spawn_epic | pub | fn(&game_core::TutorialType) -> bool | game-core\src\simulation\game\runner.rs:262 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 92 | spawn_serpen | game_core::TutorialType::spawn_serpen | pub | fn(&game_core::TutorialType) -> bool | game-core\src\simulation\game\runner.rs:266 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 93 | sub_goal | game_ai::plan_legacy::old::BattlePlan::sub_goal | pub | fn(&game_ai::plan_legacy::old::BattlePlan) -> game_ai::plan_legacy::old::BattleSubPlanGoal | game-ai\src\plan_legacy\old\battle.rs:178 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 94 | sub_goal | game_ai::plan_legacy::old::SinglePlanBattle::sub_goal | pub | fn(&game_ai::plan_legacy::old::SinglePlanBattle) -> game_ai::plan_legacy::old::BattleSubPlanGoal | game-ai\src\plan_legacy\old\single_battle.rs:27 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 95 | sub_goal | game_ai::plan_legacy::old::DeathMatchBattle::sub_goal | pub | fn(&game_ai::plan_legacy::old::DeathMatchBattle) -> game_ai::plan_legacy::old::BattleSubPlanGoal | game-ai\src\plan_legacy\old\death_battle.rs:114 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 96 | team | game_view::ClientData::team | pub | fn(&game_view::ClientData, usize) -> std::option::Option<std::cell::Ref< game_core::Team>> | game-view\src\logic\client\data.rs:1651 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 97 | team | game_view::ClientDatabase::team | pub | fn(&game_view::ClientDatabase, usize) -> std::option::Option<&game_core::Team> | game-view\src\logic\client\data.rs:968 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 98 | team | game_core::PlayerAiContext::<'a, 'b, 'r>::team | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:583 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 99 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 100 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 101 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 102 | tick_per_second | game_view::view::effect::nightmare::tick_per_second | in:game_view::view::effect::nightmare | fn(&engine_core::assets::Assets) -> f32 | game-view\src\view\effect\nightmare.rs:31 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 103 | tower | game_core::AbstractGameWithCache::<'a, 'b>::tower | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, game_core::LineType, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:1822 | True | True | 3 | IR 호출 심볼 일치(토큰 경계) |
| 104 | tutorial | game_core::Strategy::tutorial | pub | fn() -> game_core::Strategy | game-core\src\simulation\strategy.rs:590 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 105 | update_v27_objective_discipline | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::update_v27_objective_discipline | in:game_ai | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &mut game_core::DebugFrameData) | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:334 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 106 | v23_healthy_allies_near_point | game_ai::plan_legacy::team_plan::v23_healthy_allies_near_point | pub | fn(&game_core::PlayerState, &game_core::OperationData, u64, u64, u64, usize) -> usize | game-ai\src\plan_legacy\team_plan\objective_helpers.rs:25 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 107 | v24_objective_setup_lane_pressure_ready | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v24_objective_setup_lane_pressure_ready | pub | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, game_core::JungleType, &mut game_core::DebugFrameData) -> bool | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:7 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 108 | v27_objective_entity | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v27_objective_entity | in:game_ai | fn(&game_ai::plan_legacy::team_plan::TeamPlan, &game_core::OperationData, game_core::JungleType) -> std::option::Option<&game_core::Entity> | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:324 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 109 | v3_epicops_repair_need | game_ai::plan_legacy::old::epic::v3_epicops_repair_need | in:game_ai | fn(&game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::types::BigPlan) -> u8 | game-ai\src\plan_legacy\old\epic.rs:833 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 110 | valid_lines | game_ai::plan_legacy::rule_scope::valid_lines | pub | fn(&game_core::GameContext) -> &[game_core::LineType] | game-ai\src\plan_legacy\rule_scope.rs:13 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
</details>

⚠**미매칭 64개**: `Vec<&Entity>>`, `accepted_tick`, `battle`, `bottom`, `chat`, `comeback_pick`, `comeback_pick_outcomes`, `comeback_pick_start_tick`, `comeback_pick_success_count`, `defense`, `defense_line`, `dive`, `drop  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `drop_glue<BigPlan>`, `drop_glue<Vec<usize>>`, `else`, `engaged_at_tower`, `entry`, `epic_ally_killed_tick`, `epic_ally_tick`, `epic_enemy_killed_tick`, `format_inner`, `from_mid`, `gank_start_tick`, `grow_one`, `handle_morgard`, `handle_serpen`, `height`, `infos`, `jungle`, `killed_position`, `killer_team`, `last_mut`, `line_of`, `llvm.memcpy.p0.p0.i64`, `llvm.umax.i64`, `llvm.umin.i64`, `llvm.usub.sat.i64`, `minion_count`, `move_speed`, `next_respawn_tick`, `objective`, `objective_discipline`, `objective_misunderstanding`, `or_default`, `or_insert`, `outcome`, `phase`, `press_epic`, `press_tower`, `press_tower_start_tick`, `push_mut`, `rustc_entry`, `s0_0`, `s1_0`, `s2_0`, `setting`, `split_epic`, `state`, `steal_action`, `target`, `tower2`, `until_tick`, `wait_pos`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m09.ll:25667) · **형제 55개** (TeamPlan)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | game_ai::plan_legacy::old::epic::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_press_epic | pub | game-ai\src\plan_legacy\old\epic.rs:502 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 1 | game_ai::plan_legacy::old::epic::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v3_epicops_buff_window | in:game_ai | game-ai\src\plan_legacy\old\epic.rs:634 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &game_ai::plan_legacy::types::BigPlan) -> bool |
| 2 | game_ai::plan_legacy::old::epic::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_epic_line_change | pub | game-ai\src\plan_legacy\old\epic.rs:684 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData, game_core::LineType) |
| 3 | <game_ai::plan_legacy::team_plan::TeamPlan as std::clone::Clone>::clone | pub | game-ai\src\plan_legacy\team_plan.rs:47 | True | fn(&game_ai::plan_legacy::team_plan::TeamPlan) -> game_ai::plan_legacy::team_plan::TeamPlan |
| 4 | <game_ai::plan_legacy::team_plan::TeamPlan as std::fmt::Debug>::fmt | pub | game-ai\src\plan_legacy\team_plan.rs:47 | True | fn(&game_ai::plan_legacy::team_plan::TeamPlan, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 5 | <game_ai::plan_legacy::team_plan::TeamPlan as std::default::Default>::default | pub | game-ai\src\plan_legacy\team_plan.rs:47 | True | fn() -> game_ai::plan_legacy::team_plan::TeamPlan |
| 6 | game_ai::plan_legacy::team_plan::TeamPlan::mf_note_obj_clear | in:game_ai | game-ai\src\plan_legacy\team_plan.rs:196 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, u8, usize) |
| 7 | game_ai::plan_legacy::team_plan::TeamPlan::sanitize_rule_scope | pub | game-ai\src\plan_legacy\team_plan.rs:200 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, &game_core::GameContext) |
| 8 | game_ai::plan_legacy::team_plan::TeamPlan::objective_target | pub | game-ai\src\plan_legacy\team_plan.rs:230 | True | fn(&game_ai::plan_legacy::team_plan::TeamPlan) -> std::option::Option<game_core::JungleType> |
| 9 | game_ai::plan_legacy::team_plan::TeamPlan::take_active | pub | game-ai\src\plan_legacy\team_plan.rs:243 | True | fn(&game_ai::plan_legacy::team_plan::TeamPlan, game_core::JungleType) -> bool |
| 10 | game_ai::plan_legacy::team_plan::TeamPlan::take_hunt_commit | pub | game-ai\src\plan_legacy\team_plan.rs:248 | True | fn(&game_ai::plan_legacy::team_plan::TeamPlan, game_core::JungleType) -> bool |
| 11 | game_ai::plan_legacy::team_plan::TeamPlan::take_setup_like | pub | game-ai\src\plan_legacy\team_plan.rs:257 | True | fn(&game_ai::plan_legacy::team_plan::TeamPlan, game_core::JungleType) -> bool |
| 12 | game_ai::plan_legacy::team_plan::TeamPlan::mark_objective_misunderstanding | pub | game-ai\src\plan_legacy\team_plan.rs:265 | True | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, game_core::JungleType, usize) |
| 13 | game_ai::plan_legacy::team_plan::TeamPlan::clear_objective_misunderstanding | pub | game-ai\src\plan_legacy\team_plan.rs:277 | True | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan) |
| 14 | game_ai::plan_legacy::team_plan::TeamPlan::init | in:game_ai::plan_legacy::team_plan | game-ai\src\plan_legacy\team_plan.rs:281 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 15 | game_ai::plan_legacy::team_plan::TeamPlan::update | pub | game-ai\src\plan_legacy\team_plan.rs:294 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 16 | game_ai::plan_legacy::team_plan::TeamPlan::can_near_enemies | pub | game-ai\src\plan_legacy\team_plan.rs:444 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< &game_core::Entity> |
| 17 | game_ai::plan_legacy::team_plan::TeamPlan::can_near_enemies_range | pub | game-ai\src\plan_legacy\team_plan.rs:483 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, u64, u64, u64, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< &game_core::Entity> |
| 18 | game_ai::plan_legacy::team_plan::TeamPlan::update_wave_priority_clear_line | in:game_ai::plan_legacy::team_plan | game-ai\src\plan_legacy\team_plan.rs:540 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) |
| 19 | game_ai::plan_legacy::team_plan::TeamPlan::should_player_clear_wave_priority_line | pub | game-ai\src\plan_legacy\team_plan.rs:561 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PlayerState, &game_core::OperationData) -> std::option::Option<game_core::LineType> |
| 20 | game_ai::plan_legacy::team_plan::TeamPlan::should_delay_morgard_for_wave_priority | in:game_ai::plan_legacy::team_plan | game-ai\src\plan_legacy\team_plan.rs:570 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData) -> bool |
| 21 | game_ai::plan_legacy::team_plan::TeamPlan::should_delay_serpen_for_wave_priority | in:game_ai::plan_legacy::team_plan | game-ai\src\plan_legacy\team_plan.rs:574 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData) -> bool |
| 22 | game_ai::plan_legacy::team_plan::TeamPlan::should_delay_object_setup_for_wave_priority | pub | game-ai\src\plan_legacy\team_plan.rs:578 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData) -> bool |
| 23 | game_ai::plan_legacy::team_plan::TeamPlan::should_keep_object_for_contested_wave_priority | in:game_ai::plan_legacy::team_plan | game-ai\src\plan_legacy\team_plan.rs:586 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, game_ai::plan_legacy::team_plan::objective_helpers::WavePriorityObject) -> bool |
| 24 | game_ai::plan_legacy::team_plan::TeamPlan::repair_misunderstood_objective | in:game_ai::plan_legacy::team_plan | game-ai\src\plan_legacy\team_plan.rs:603 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData) -> bool |
| 25 | game_ai::plan_legacy::team_plan::TeamPlan::update_objective | pub | game-ai\src\plan_legacy\team_plan.rs:722 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData) |
| 26 | game_ai::plan_legacy::team_plan::TeamPlan::update_steal | pub | game-ai\src\plan_legacy\team_plan.rs:732 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &game_ai::plan_legacy::types::BigPlan) |
| 27 | game_ai::plan_legacy::team_plan::TeamPlan::update_objective_after_steal | in:game_ai::plan_legacy::team_plan | game-ai\src\plan_legacy\team_plan.rs:910 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData) |
| 28 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v24_objective_setup_lane_pressure_ready | pub | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:7 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, game_core::JungleType, &mut game_core::DebugFrameData) -> bool |
| 29 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v24_objective_setup_should_check_camp | pub | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:88 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, game_core::JungleType) -> bool |
| 30 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v24_objective_setup_should_release_to_passive | pub | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:177 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::PlayerState, &game_core::OperationData, game_core::JungleType) -> bool |
| 31 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v24_objective_setup_relevant_lanes_ready | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:199 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PlayerState, &game_core::OperationData, game_core::JungleType) -> bool |
| 32 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v25_objective_posture | pub | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:207 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::PlayerState, &game_core::OperationData, game_core::JungleType) -> std::option::Option<game_ai::plan_legacy::team_plan::ObjectivePosture> |
| 33 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v27_objective_wait_pos | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:316 | False | fn(game_core::JungleType, usize, &game_core::MapDef) -> (u64, u64) |
| 34 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v27_objective_entity | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:324 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, &game_core::OperationData, game_core::JungleType) -> std::option::Option<&game_core::Entity> |
| 35 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::update_v27_objective_discipline | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:334 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &mut game_core::DebugFrameData) |
| 36 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v27_active_objective_discipline | pub | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:483 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::OperationData, game_core::JungleType) -> std::option::Option<game_ai::plan_legacy::team_plan::ObjectiveDisciplineState> |
| 37 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v27_objective_discipline_blocks_battle | pub | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:497 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::OperationData) -> bool |
| 38 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v27_objective_discipline_action | pub | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:505 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, game_core::JungleType) -> std::option::Option<game_ai::SmallActionPlay> |
| 39 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_repair_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:7 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PlayerState, &game_core::OperationData) |
| 40 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_defense_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:16 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData) -> bool |
| 41 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_nexus_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:29 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData, game_core::LineType) -> bool |
| 42 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_defense_line_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:46 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData, game_core::LineType) -> bool |
| 43 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_dive_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:88 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData, game_core::LineType) -> bool |
| 44 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_press_tower_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:141 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData, game_core::LineType) -> bool |
| 45 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_press_epic_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:253 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData, game_core::LineType) -> bool |
| 46 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_split_epic_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:334 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData, game_core::LineType) -> bool |
| 47 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_comeback_pick_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:393 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData, game_core::LineType, bool) -> bool |
| 48 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_serpen_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:558 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData, game_ai::plan_legacy::team_plan::ObjectPhase) -> bool |
| 49 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_morgard_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:685 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData, game_ai::plan_legacy::team_plan::ObjectPhase) -> bool |
| 50 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_none_or_gank_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:830 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData) -> bool |
| 51 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_gank_preprocess | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:1133 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, game_core::LineType) -> bool |
| 52 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_nexus_defense | pub | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:1227 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> bool |
| 53 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_nexus_attack | pub | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:1237 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> std::option::Option<game_core::LineType> |
| 54 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_sub_objective | pub | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:1258 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData) |

**`open` 9건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | handle_gank_preprocess closure#1(ally_minions_in_range 술어)은 m11.ll:24927 Copied::fold 안에 있고 Effect::is_in_range(tower_attack, enemy_tower, minion) 호출만 확인 — 그 세부(사거리 계산)는 game_core 본체(_gcbc) 미독 | 4 |  |
| 1 | 미탐색 | Effect::expected_damage_target 의 4번째 인자 @anon...138(ptr 상수)의 정체 미확인 — 관측: tower_attack·ctx·enemy_tower·<상수>·minion 순 | 4 |  |
| 2 | 미탐색 | M741/S607 의 `min(epic_enemy_killed_tick, epic_ally_killed_tick) > tps*2` — 필드가 절대 틱인지 경과 틱인지 이 함수만으론 불명(GoalData 갱신 코드 미독). dbg 이름 skip_disengage 가 이 비교값이라 극성은 IR 분기 방향으로만 기록 | 4 |  |
| 3 | 미탐색 | R653 player_count(tutorial) 매핑({1,3}→2·{2,4,6}→1·{0,5,7,8}→3)은 switch 인라인 결과 — 함수 이름과 값의 의미(라인 플레이어 수)는 추정 | 5 |  |
| 4 | 미탐색 | PT197 valid_lines 정적 배열(@anon .291 외) 의 원소 순서는 읽지 않음 — 길이(3/2/1)만 확인 | 4 |  |
| 5 | 미탐색 | Option<MainObjective>/Option<LineType> None = 0xFF 로 IR 이 일관되나, tcx layout 은 tag_enc 를 표시하지 않아 니치 시작값 문서 대조는 못 함(IR 정본 원칙으로 기록) | 3 |  |
| 6 | 미탐색 | handle_none_or_gank_objective(bool 반환)·handle_repair_objective·handle_nexus_attack 등 아웃오브라인 콜리 내부는 범위 밖(별도 명세 대상) | 4 |  |
| 7 | 미탐색 | aux 9개 조각 중 count 4종(gank s_0 제외)의 5칸 언롤 반복은 첫 iteration 만 정독하고 나머지는 구조 동일로 간주(runs2 맵으로 동일 chain 확인) | 4 |  |
| 8 | 미탐색 | exe 0xdda220 과의 대조는 하지 않음(지시상 재탐색 금지) — 24,674B 크기와 IR 11,379줄(실명령 ≈4,800줄 + aux) 규모는 정합 범위로 보임(추정) | 4 |  |

**`notes` 1건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | objective_discipline.rs:415~416 dbg 이름(very_low_contact 등)이 IR phi/and 와 1:1 대응되지 않아 소스 변수명은 추정; 판정식 자체는 IR 기준으로 확정 | 4 | 사실 서술 |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

