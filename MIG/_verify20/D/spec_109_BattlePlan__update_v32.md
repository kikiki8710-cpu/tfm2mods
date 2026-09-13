---

### `109` BattlePlan::update_v32 — Battle 플랜 매틱 갱신: 교전 focus 선정→다이브/타워/웨이브 위험 판정→sub_goal(Trace/Kiting/KitingBack/RunAway/End)과 exit_src·dive 상태를 self 에 기록

| 항목 | 값 |
|---|---|
| id | `battle__BattlePlan_update_v32` |
| 심볼 | `_RNvMs0_NtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old6battleNtB5_10BattlePlan10update_v32` |
| 소스 | `game-ai\src\plan_legacy\old\battle.rs:748` |
| IR | `m10.ll` 13739~22810행 |
| 경로·가시성 | `game_ai::plan_legacy::old::BattlePlan::update_v32` · **in:game_ai::plan_legacy::old::battle** |
| 계층 | 레거시 플랜 |
| exe | `df36e0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r11` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&mut game_ai::plan_legacy::old::BattlePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData)
```

<details><summary>인자 8개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | self | &mut BattlePlan(280B) | %0 noalias dereferenceable(280), readonly 없음 → &mut. sub_goal·exit_src·dive_* 등 45필드 중 39곳 기록(writes 참조) | 4 |
| 1 | 2 | version | usize | %1. AI 버전 게이트: `version > 1`(icmp ugt %1,1) 이 v2+ 분기, `version < 2` 가 레거시 분기(1170). 지역 alloca %144 에 저장돼 클로저에 &로 전달 | 4 |
| 2 | 3 | rnd | &mut StdRng(320B) | %2 noalias align16 dereferenceable(320), readonly 없음 → &mut. 본문에선 읽지 않고 check_kill_die_tick·resolve_fight_stake·tower_dive_is_viable·strategy·fight_participants·FightSituation::build 에 그대로 전달 | 4 |
| 3 | 4 | player | &PlayerState(2528B) | %3 noalias readonly dereferenceable(2528) → &. info.team(+0x930)·info.position(+0x9c0)·info.parameter(+0x180, judge_accuracy) 읽음 | 4 |
| 4 | 5 | data | &OperationData(24B) | %4 noalias readonly dereferenceable(24) → &. cache(+0x0 &AbstractGameWithCache), context(+0x8 &GameContext), blackboard(+0x10 &[Blackboard;2]) | 4 |
| 5 | 6 | _positioning_score | &PositioningScoreData | DWARF arg 6. IR 에서 dead-arg 제거(#dbg_value ptr poison !25373) — 미사용 | 3 |
| 6 | 7 | team_plan | &TeamPlan(1064B) | %5 — noalias/readonly/dereferenceable 3속성 전부 없음(추정: UnsafeCell 포함 타입 or 호출측 속성 소실). 본문에서 %5 경유 store 0건 · load 는 ally_battle_stop_tick(+0x0)·last_resolver_bail_tick(+0x228) 뿐 → 읽기 전용(&)으로 판정, DWARF 타입 ref$<TeamPlan> 과 일치 | 3 |
| 7 | 8 | debug | &mut DebugFrameData(224B) | %6 noalias dereferenceable(224), readonly 없음 → &mut. add_log 12회·+0xa0 HashMap entry 2회 (전부 context.debug 게이트 하) | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
// 기호: champ=내 챔피언 &Entity, team=player.info.team, tps=setting.tick_per_second, cache=data.cache, bump=context.pool. 모든 Vec<&Entity> 는 bumpalo(bump) 할당. ★=self 기록. 인라인 헬퍼: engage_pair_range(v,data,a,b)= v>1 ? max(fight_kit_range_cached(data,a,b),fight_kit_range_cached(data,b,a)) : max(max_range_cached(data,a,b),max_range_cached(data,b,a)) ; is_ignored_well_enemy(e)= e.team==Player(1-team) && is_enemy_well_danger(version,player,e.x,e.y) ; is_ignored_battle_enemy(e,with_declared_dive)= (version>1 && with_declared_dive) ? is_ignored_well_enemy(e) : (is_ignored_well_enemy(e) || is_unreasonable_tower_dive_enemy(player,data,e,with_declared_dive)) ; Effect::range(caster)= eff.range(+0x4a0)+caster.stat_buff_cached.range(+0x438)+(caster.level-1)*eff.growth_range(+0x4a8) ; Entity::radius()= radius_mult(+0x470)==0 ? radius(+0x680) : radius*(mult+100)/100 ; distance_sq= dx^2+dy^2 (abs_diff).

// battle.rs:750~772
if self.sub_goal == RunAway(4) { return; }                                   // 750, %3136 = ret void 직행(아무 기록 없음)
let mut committed_dir: i8 = match self.sub_goal { Trace|Kiting|Assassin|AssassinReady => 1, KitingBack => -1, Protect|End => 0 };   // 755~766(주석) 인라인 switch
if version > 1 && cache.tick() == self.start_tick && team_plan.last_resolver_bail_tick != 0 && tick > last_resolver_bail_tick {   // 767~769
    if !(tick > last_resolver_bail_tick + 3*tps) { committed_dir = -1; }      // 771 ([v3 F6] 리졸버패배 이탈 커밋 상속)
}
// 774
let champ = cache.player_champion[team][player.info.position.as_index()].unwrap();   // team>=2 → panic_bounds_check, None → unwrap_failed
// 781~785 (version>1 만)
if version > 1 {
    let incoming = tower_discipline::v3_survival_incoming(player, data, champ);   // 782
    if !(incoming < champ.hp || self.sub_goal == RunAway) { ★exit_src=8; ★sub_goal=RunAway; return; }   // 783~785
}
// 790~794
let near_enemies: Vec<&Entity> = cache.iter_champions(1-team).filter(|e| { let r = engage_pair_range(version,data,champ,e); e.distance_sq(champ) <= (r+30000)^2 && data.blackboard[1-team].is_recent_visible(e) && !is_ignored_battle_enemy(e, self.with_dive) }).collect();
// 797~800
let near_allies: Vec<&Entity> = (0..5).filter(|&c| team_plan.ally_battle_stop_tick[c].is_none() && cache.player_champion[team][c].is_some_and(|a| a.distance_sq(champ) < 120000^2+1)).filter_map(|c| cache.player_champion[team][c]).collect();   // 본인 포함
// 811~812
let can_near_list = team_plan.can_near_enemies_range(version, rnd, player, data, champ.x, champ.y, 150000, debug); let can_near_enemies = can_near_list.len();
// 818~821
let anticipated: (u64,u64) = if sub_goal ∈ {Trace,Assassin,AssassinReady} { cache.get_entity_by_id(sub_goal.focus).map(|e|(e.x,e.y)).unwrap_or((champ.x,champ.y)) } else { (champ.x,champ.y) };
// 824~827
let threat_list = if version>1 && anticipated != (champ.x,champ.y) { can_near_enemies_range(…, anticipated.0, anticipated.1, 150000, debug) } else { can_near_list.clone() };
// 829~845
let threat_enemies: Vec<&Entity> = if version>1 { threat_list.iter().copied().filter(|e| {   // s2_0, 832~839
        if (0..5).any(|i| data.blackboard[team].big_goal[i].1 == Some(BigGoal::Battle{focus: Some(e.id)})) { return false; }   // 이미 아군 big_goal 의 Battle focus 인 적 제외
        if e.ty != Champion { return true; }                                   // 837
        if e.ty.Champion.action_state != Move { return true; }                 // 838
        utils::distance(move.x, move.y, anticipated) <= utils::distance(e.x, e.y, anticipated)   // 839 (내 예상 위치로 접근 중인 적만)
    }).collect() } else { Vec::new_in(bump) };
// 848~853
let nearest_enemy_tower: Option<&Entity> = cache.iter_towers_without_nexus(1-team).min_by_key(|t| t.distance_sq(champ)).filter(|t| t.distance(champ) <= t.attack_effect.unwrap().range(t) + 15000 + champ.radius() + t.radius());
// 856~857
let nearest_enemy_tower_type: Option<TowerType> = nearest_enemy_tower.and_then(|t| (t.ty==Tower).then(|| t.ty.Tower.info.ty));
// 861~870
let die_enemies = { let mut v = near_enemies.clone(); v.extend(threat_enemies.iter().copied()); v };
let die_tick_with_tower = check_kill_die_tick(version, rnd, data, player, champ, die_enemies.clone(), Vec::from_iter(nearest_enemy_tower), debug);   // 866
let mut die_tick = check_kill_die_tick(version, rnd, data, player, champ, die_enemies.clone(), Vec::new_in(bump), debug);          // 869~870
// 879~892
die_tick = match committed_dir { 1 => if die_tick >= tps { die_tick.saturating_add(tps) } else { die_tick }, -1 => die_tick.saturating_sub(tps), _ => die_tick };
let prev_die_eval = self.prev_die_eval; ★self.prev_die_eval = die_tick;         // 887~888
if die_tick < prev_die_eval { ★self.die_fall_evals = die_fall_evals.saturating_add(1) } else { ★self.die_fall_evals = 0 }   // 889~892
// 900~904
if version>1 && committed_dir == -1 && self.sub_goal == KitingBack && die_tick <= tps*2 && self.die_fall_evals >= 2 && self.with_runaway(data) { ★sub_goal=RunAway; return; }
// 908~920
let is_in_tower_focused = nearest_enemy_tower.is_some_and(|t| t.ty==Tower && t.ty.Tower.info.nearest_enemy.is_some_and(|(_,id)| id == champ.id));
let is_in_tower_range   = nearest_enemy_tower.is_some_and(|t| champ.distance_sq(t) <= (t.attack_range(t) + 10000 + champ.radius() + t.radius())^2);
// 923~935
if let Some(st) = self.support_target { let target = cache.get_entity_by_id(st); if target.is_none_or(|x| x.distance_sq(champ) < 60000^2+1 || !blackboard[1-team].is_recent_visible(x) || is_ignored_battle_enemy(x, with_dive)) { ★support_target = None; } }
if let Some(st) = self.support_target.and_then(get_entity_by_id) { if v21_should_defer_support_target(_v, player, data, champ, st, near_allies.len(), near_enemies.len(), can_near_enemies, die_tick, die_tick_with_tower) { ★support_target = None; } }
// 939~964
let chase_range = 250000;
let focused: Option<&Entity> = if near_enemies.is_empty() { cache.iter_champions(1-team).filter(|c| c.distance_sq(champ) <= chase_range^2 && blackboard[1-team].is_recent_visible(c) && !is_ignored_battle_enemy(c, with_dive)).min_by_key(|c| c.distance_sq(champ)) }   // 946~957
              else { near_enemies.iter().min_by_key(|e| check_kill_die_tick(version, rnd, data, player, /*champ=*/e, near_allies.clone(), Vec::new_in(bump), debug)).copied() };   // 942~944: 아군들이 가장 빨리 죽이는 적
let support_target = self.support_target.and_then(get_entity_by_id).filter(|x| !is_ignored_battle_enemy(x, with_dive));   // 960~962
let focused = support_target.or(focused);                                  // 964
// 966~1002
if focused.is_none() {
    ★exit_src = 1;                                                            // 967
    if self.ff_exit1_cls == 255 {                                             // 970~984 [S1a 계측]
        let (mut invis, mut well, mut towerf) = (0,0,0);
        for c in cache.iter_champions(1-team).filter(|c| c.distance_sq(champ) <= 250000^2) { if !blackboard[1-team].is_recent_visible(c) { invis+=1 } else if is_ignored_well_enemy(c) { well+=1 } else { towerf += is_unreasonable_tower_dive_enemy(player,data,c,with_dive) as u32 } }
        ★ff_exit1_cls = match (invis==0, well==0, towerf==0) { (t,t,t)=>0, (f,t,t)=>1, (t,f,t)=>2, (t,t,f)=>3, _=>4 };
    }
    let return_to_objective = main_objective ∈ {Nexus(4),PressTower(10)} || (version>1 && nexus_last_stand(player,data));   // 996~997
    ★sub_goal = if return_to_objective { End } else { RunAway };            // 998 no_valid_battle_target_goal
    return;
}
let mut focused = focused.unwrap();
// 1006~1022
if focused.team == champ.team {                                               // focused 가 아군(=support 대상)이면 그 주변 적으로 교체
    let near_focused_enemies = cache.iter_champions(1-team).filter(|e| e.distance_sq(focused) < 150000^2+1 && blackboard[1-team].is_recent_visible(e) && !is_ignored_battle_enemy(e, with_dive)).collect();   // sh_0
    if near_focused_enemies.is_empty() { ★exit_src = 2; ★sub_goal = End; return; }   // 1012~1014
    focused = *near_focused_enemies.iter().min_by_key(|e| check_kill_die_tick(version,rnd,data,player, e, near_allies.clone(), Vec::new_in(bump), debug)).unwrap();   // 1018~1021
}
// 1031~1062
if near_enemies.is_empty() && committed_dir == -1 && focused.team == Player(1-team) {
    let target_enemies = cache.iter_champions(1-team).filter(sk_0 /*= sh_0 기준 focused 150000*/).collect();                    // 1033~1037
    let target_tower = iter_towers_without_nexus(1-team).filter(|t| t.can_target && t.block_target_tick==0).min_by_key(|t| t.distance_sq(focused)).filter(|t| t.distance(focused) <= t.attack_range(t)+15000+focused.radius()+t.radius());   // 1039~1043
    let rejoin: FightPrediction = if version>1 { let roster = fight_participants(version,rnd,data,player,champ,&near_allies,&target_enemies,team_plan,debug); resolve_fight_stake_roster(version,data,champ,&roster,&target_enemies,committed_dir,target_tower,player.info.parameter.judge_accuracy()) }   // 1050~1053
                 else { resolve_fight_stake(version,rnd,data,player,champ,&near_allies,&target_enemies,committed_dir,target_tower,judge_accuracy,debug) };   // 1054~1055
    if rejoin.line != Commit { ★exit_src = 13; ★sub_goal = End; return; }   // 1057~1059
}
// 1073~1079
let mut model_prediction: Option<FightPrediction> = if version>1 && !near_enemies.is_empty() { let roster = fight_participants(…,&near_allies,&near_enemies,…); Some(resolve_fight_stake_roster(version,data,champ,&roster,&near_enemies,committed_dir,nearest_enemy_tower,judge_accuracy)) }
                                                   else { Some(resolve_fight_stake(version,rnd,data,player,champ,&near_allies,&near_enemies,committed_dir,nearest_enemy_tower,judge_accuracy,debug)) };
// 1085~1094
if let Some(p) = model_prediction.as_mut() { if let Some(ally_id) = p.rescue_ally {
    let ally = cache.get_entity_by_id(ally_id);
    let helping = ally.is_some_and(|a| focused.distance_sq(a) > (engage_pair_range(version,data,focused,a)+30000)^2);   // 1087~1089
    if ally.is_some() && !helping { ★self.ff_rescue_tick = cache.tick(); }   // 1094 (p.line 유지)
    else { p.line = p.line_absolute; }                                        // IR 실측(1091~1093 레지스터 전용, 소스 문장 형태 미확정)
} }
let model_commits = model_prediction.is_some_and(|x| x.line == Commit);    // 1098~1099
// 1110~1125
let model_soaker = model_prediction.and_then(|p| p.soaker);
let soaker_ok = model_soaker.is_none_or(|s| s == champ.id); let i_am_soaker = model_soaker == Some(champ.id);   // 1111 (IR 두 값 %1182/%1181)
let mut my_die_tick_with_tower;
if with_dive && let (Some(dt),Some(nt)) = (self.dive_tower, nearest_enemy_tower_type) && (v50_engine_truth_scope = (dt.is_twin() && nt.is_twin()) || dt==nt) {   // 1115~1118
    my_die_tick_with_tower = if is_in_tower_focused { die_tick_with_tower } else { die_tick };   // 1124 → 1170 으로 점프(1137~1165 생략)
} else {
    my_die_tick_with_tower = if soaker_ok || is_in_tower_focused { die_tick_with_tower } else { die_tick };   // 1125
    // 1137~1165
    let focused_closer_to_tower = version>1 && !with_dive && !is_in_tower_range && nearest_enemy_tower.is_some_and(|t| focused.distance_sq(t) <= champ.distance_sq(t));   // 1137~1138 (%1253)
    let warn_zone = is_in_tower_range || focused_closer_to_tower;              // %1252
    if !with_dive && warn_zone && focused.team == Player(1-team) && !(focused.move_speed*10 < champ.move_speed*9) {   // 1142~1145
        let my_range = max_range_cached(data, champ, focused);                // 1147
        if focused.distance_sq(champ) > my_range^2 {                          // 1148
            if version>1 { if !with_dive { ★dive_abandoned=true; ★dive_abort_src=9; }   // 1150~1152
                           ★exit_src=7;                                        // 1154
                           if my_die_tick_with_tower > tps { let kite_focus = near_enemies.iter().min_by_key(|e| e.distance_sq(champ)).map(|e| e.id).unwrap_or(focused.id); ★sub_goal=KitingBack{kite_focus}; return; }   // 1158~1161
                           else { ★sub_goal=RunAway; return; } }              // 1164
            else { ★exit_src=7; ★sub_goal=RunAway; return; }
        }
    }
}
// 1170~1185 (version < 2 만)
if version < 2 { if let Some(mo)=self.main_objective && mo ∈ {Morgard,Serpen} && mo.phase == Hunt && player.strategy(rnd, game).object_finish == KillPriority {
    match objective_entity_id_for_main_objective(data, mo).and_then(get_entity_by_id) { None => { ★exit_src=4; ★exit_sub=6; ★sub_goal=End; return; }
        Some(obj) => if focused.team == Player(1-team) && !can_enemy_hit_objective(focused, obj, 25000) { ★exit_src=4; ★exit_sub=7; ★sub_goal=End; return; } }
} }
// 1196~1206
if is_enemy_well_danger(version, player, focused.x, focused.y) {            // focused_in_enemy_well
    ★with_dive=false; ★dive_tower=None; ★dive_abandoned=true; ★dive_abort_src=6; ★exit_src=6;   // 1198~1203
    ★sub_goal = if main_objective ∈ {Nexus,PressTower} { End } else if champ.distance_sq(well_anchor(team)) < 300000^2+1 { RunAway } else { End };   // 1204~1206, well_anchor = team==1 ? (0,960000) : (960000,0)
    return;
}
// 1218~1219
let mut dive_viable = with_dive && tower_dive_is_viable(version, rnd, player, data, team_plan, focused, /*team_model=*/true, debug);
// 1230~1278
let dive_tower_matches = self.dive_tower.is_some() && nearest_enemy_tower_type.is_some_and(|nt| (dt.is_twin()&&nt.is_twin()) || dt==nt);
let v50_dive_episode = with_dive && dive_tower.is_some() && focused.team==Player(1-team) && (dive_tower_matches || !is_in_tower_focused);   // 1232~1235 (IR 등가)
if v50_dive_episode && is_in_tower_range { ★dive_entered = true; }        // 1236
if with_dive && (dive_tower.is_none() || v50_dive_episode || !is_in_tower_focused) { ★dive_ctx_break_ticks = 0; }   // 1242~1244
if with_dive && dive_tower.is_some() && !v50_dive_episode {
    if focused.team==Player(1-team) && is_in_tower_focused {                 // 1248
        ★dive_ctx_break_ticks += 1;                                          // 1250
        if dive_entered ? dive_ctx_break_ticks >= tps : dive_ctx_break_ticks >= 2*tps {   // 1253/1257
            if near_allies.len()>1 { ★chats.push(BattleStop(TowerDiveFail)) }   // 1259
            ★with_dive=false; ★dive_tower=None; ★dive_abandoned=true; ★dive_abort_src=2; ★exit_src=10; ★sub_goal=End; return;   // 1260~1265
        }
    }
    if is_unreasonable_tower_dive_enemy(player, data, focused, /*with_declared_dive=*/true) && !dive_viable && !model_commits {   // 1271
        ★with_dive=false; ★dive_tower=None; ★dive_abandoned=true; ★dive_abort_src=13; ★exit_src=10; ★sub_goal=End; return;   // 1272~1278
    }
} else { (1271 도 평가되나 %1486=false 로 전달; 결과는 위 조건과 동일하게 &&) }
// 1282~1334
let focused_die_tick = check_kill_die_tick(version, rnd, data, player, focused, near_allies.clone(), Vec::new_in(bump), debug);
let max_range = max_range_cached(data, champ, focused); let focused_is_in_range = focused.distance_sq(champ) <= max_range^2;   // 1286~1287
let tower_close_danger = nearest_enemy_tower.is_some_and(|t| champ.distance_sq(t) < 40000^2+1) && my_die_tick_with_tower <= 2*tps;   // 1290~1291
let tower_on_other = nearest_enemy_tower.is_some_and(|t| t.ty==Tower && t.nearest_enemy.is_some_and(|(_,id)| get_entity_by_id(id).is_some_and(|e| e.ty==Champion && e.id != champ.id)));   // 1294~1297
let mut warn_tower = tower_close_danger || my_die_tick_with_tower < 11 || (my_die_tick_with_tower <= tps && is_in_tower_focused);   // 1304~1307
if tower_on_other { warn_tower = false } else if let (Some(dt),Some(nt)) = (dive_tower, nearest_enemy_tower_type) && (dt==nt || (dt.is_twin()&&nt.is_twin())) { warn_tower = false }   // 1311~1314
if context.debug { debug.map[+0xa0].entry(champ.id).or_insert(vec![]).push(format!(…)) ×2 }   // 1317~1319
let nearest_enemy_tower = iter_towers_without_nexus(1-team).min_by_key(|t| t.distance_sq(focused));   // 1322 섀도잉(필터 없음)
let attack_range = champ.attack_effect.unwrap().range(champ) + champ.radius() + focused.radius();   // 1324
let focused_is_in_tower = nearest_enemy_tower.is_some_and(|t| t.distance_sq(focused) <= (t.attack_range(t)+focused.radius()+t.radius()).saturating_sub(attack_range) + 10000)^2);   // 1325~1331
let target_hp_ratio = focused.hp*100 / focused.stat_cached.hp;               // 1333 (0 → div_by_zero panic)
let is_dive_current_tower = dive_tower.is_some() && nearest_enemy_tower_type.is_some_and(|nt| (dt,nt 둘 다 Twin) || dt==nt);   // 1334
// 1335~1345
let close_execute_range = max_range + 25000;
let guard = with_dive && is_dive_current_tower;
if tower_close_danger { dive_viable = guard && dive_viable; }
else { let outlast = focused_die_tick.saturating_add(tps/2) < my_die_tick_with_tower;   // 1336
       let close_exec = focused.distance_sq(champ) <= close_execute_range^2 && (target_hp_ratio < 30 || ready_damage_to_target(context, champ, focused) >= focused.hp);   // 1339~1340
       let has_close_execute = guard && close_exec && my_die_tick_with_tower > tps;   // 1341
       dive_viable = guard && (dive_viable || outlast || has_close_execute); }   // 1344~1345 (current_dive_can_continue)
if context.debug && i_am_soaker { debug.add_log(…) }                        // 1348~1349
// 1358~1362
if focused.distance_sq(champ) <= close_execute_range^2 && ready_damage_to_target(context, champ, focused) >= focused.hp {
    let kill_secure = focused_die_tick > my_die_tick_with_tower;               // 1360
    if !kill_secure { ★sub_goal = Trace{focused.id}; return; }                // 1361~1362
}
// 1373
if v50_dive_episode {
    // ===== 1374~1555 (v50 다이브 에피소드 판정) =====
    let holder = is_in_tower_focused;                                         // 1374
    let soaker = dive_entry_soaker(player, focused);   // fight_model.rs:1073~1074 인라인: 아군 (0..5) 중 focused 200000 이내 → max_by(stat_cached.hp, 동률 id)
    let i_am_entry_soaker = soaker.map(|e| e.id) == Some(champ.id);           // 1376
    let aggro_secured = tower_on_other || is_in_tower_focused;                // 1377
    if context.debug { add_log }                                              // 1378~1382
    ★dive_last_model = match model_prediction.map(|p| p.line) { None|Some(CommitAfterJoin)=>0, Some(Commit)=>1, Some(Hold)=>2, Some(Disengage)=>3 };   // 1385~1386
    ★dive_last_race_adv = min(die_tick_with_tower,2000) as i32 - min(focused_die_tick,2000) as i32;   // 1388
    ★dive_last_na = min(near_allies.len(),255); ★dive_last_ne = min(near_enemies.len(),255);   // 1389~1390
    if holder || dive_chase_catchable(champ, focused, tps) { ★dive_catch_break_ticks = 0 } else { ★dive_catch_break_ticks += 1 }   // 1397~1400
    if !holder && !dive_chase_catchable(champ,focused,tps) && self.dive_entered && dive_catch_break_ticks >= tps {   // 1402~1403
        if context.debug { add_log }   // 1408~1411
        if near_allies.len()>1 { ★chats.push(BattleStop(TowerDiveFail)) }   // 1413
        ★with_dive=false; ★dive_tower=None; ★dive_abandoned=true; ★dive_abort_src = if focused_die_tick < die_tick_with_tower {11} else {1}; ★exit_src=10; ★sub_goal=End; return;   // 1414~1419
    }
    if model_prediction.is_some_and(|p| p.line == Disengage) {                // 1427
        ★dive_model_break_ticks += 1;                                        // 1433
        if version < 2 || dive_model_break_ticks >= tps || (version>1 && my_die_tick_with_tower <= tps) {   // 1437~1438
            if context.debug { add_log }   // 1439~1441
            if near_allies.len()>1 { ★chats.push(BattleStop(TowerDiveFail)) }   // 1443
            ★with_dive=false; ★dive_tower=None; ★dive_abandoned=true; ★dive_model_break_ticks=0; ★exit_src=10;   // 1444~1449
            ★sub_goal = if holder { KitingBack{focused.id} } else { End }; return;   // 1450
        }
    } else { ★dive_model_break_ticks = 0; }                                   // 1454
    if holder {                                                               // 1456
        ★tactic = Frontline;                                                  // 1459
        if !(die_tick_with_tower > tps) && focused_die_tick.saturating_add(tps/2) > die_tick_with_tower {   // 1465~1466
            if die_tick_with_tower <= tps/2 || near_allies.len() < 2 { ★with_dive=false; ★dive_tower=None; ★dive_abandoned=true; ★exit_src=10; ★sub_goal=End; return; }   // 1469~1474
            else { ★sub_goal = KitingBack{focused.id}; return; }              // 1477
        }
        let hold_margin = if committed_dir == -1 { tps/3 } else { tps/6 };   // 1480
        if let Some((exit, tdie)) = soaker_tower_hold_components(cache, context, team, champ) {   // 1481
            if context.debug { add_log }   // 1482~1486
            if !(die_tick_with_tower > hold_margin + exit) && focused_die_tick.saturating_add(tps/2) > die_tick_with_tower {   // 1491~1492
                if context.debug { add_log }   // 1494
                if near_allies.len()>1 && die_tick_with_tower > tps/2 { ★sub_goal = KitingBack{focused.id}; return; }   // 1497, 1505
                else { ★with_dive=false; ★dive_tower=None; ★dive_abandoned=true; ★exit_src=10; ★sub_goal=End; return; }   // 1498~1501
            }
            if !(tdie > hold_margin + exit) {                                 // 1508
                if near_allies.len()>1 && die_tick_with_tower > tps/2 { ★sub_goal = KitingBack{focused.id}; return; }   // 1511, 1519
                else { ★with_dive=false; ★dive_tower=None; ★dive_abandoned=true; ★exit_src=10; ★sub_goal=End; return; }   // 1512~1516
            }
        }
    } else {                                                                  // !holder
        if i_am_entry_soaker && !aggro_secured { ★tactic = Frontline; }      // 1524~1526
        if !(die_tick > tps) && (if dive_entered { focused_die_tick.saturating_add(tps/2) } else { focused_die_tick }) > die_tick { ★sub_goal = KitingBack{focused.id}; return; }   // 1529~1532
    }
    // 1542~1555
    if !(version<=1 || with_dive || model_commits) && v3_beyond_enemy_line(team, cache, focused) {
        ★exit_src = 7;                                                        // 1543
        ★sub_goal = if die_tick > tps && self.with_runaway(data) { End } else { RunAway }; return;   // 1547 (IR: with_runaway true → End(7), 아니면 RunAway(4))
    }
    ★sub_goal = Trace{focused.id}; return;                                    // 1554
}
// ===== 1558~1581 (비 v50) =====
if warn_tower {
    let dist = focused.distance_sq(champ);                                    // 1560
    let attack_range = champ.attack_effect.unwrap().range(champ) + champ.radius() + focused.radius();   // 1562
    if dist >= attack_range^2 && (focused_closer_to_tower || is_in_tower_range) && !dive_viable && self.with_runaway(data) && !model_commits {   // 1564
        if with_dive { if focused_is_in_tower && near_allies.len()>1 { ★chats.push(BattleStop(TowerFocused)) } ★dive_abandoned=true; ★exit_src=8; ★sub_goal=End; return; }   // 1565~1572
        else { ★exit_src=8; ★sub_goal=KitingBack{focused.id}; if focused_is_in_tower { if near_allies.len()>1 { ★chats.push(BattleStop(TowerFocused)) } ★sub_goal=RunAway; } return; }   // 1575~1581
    }
}
// 1587~1610
let (mut wave_obs, mut wave_pct, mut wave_danger) = (0u8, 0u8, false);
let can_runaway = self.with_runaway(data); let return_to_objective = main_objective ∈ {Nexus,PressTower}; let open_eval = cache.tick() == self.start_tick;   // 1591~1592
let wave_goal: Option<BattleSubPlanGoal> = v30_wave_danger_chase_guard(version, data, champ, focused, max_range, focused_die_tick, focused_is_in_range, /*my_die_tick=*/die_tick, target_hp_ratio, can_runaway, return_to_objective, open_eval, &mut wave_obs, &mut wave_pct, &mut wave_danger);   // 1590
if self.ff_wave_obs_open == 255 { ★ff_wave_obs_open=wave_obs; ★ff_wave_open_hp=min(champ.hp*100/max(champ.max_hp,1),254); ★ff_wave_open_pct=wave_pct; ★ff_wave_open_danger=wave_danger; }   // 1595~1600
if let Some(goal) = wave_goal { ★exit_src=9; if ff_wave_fire_hp==255 { ★ff_wave_fire_hp=min(hp%,254); ★ff_wave_fire_pct=wave_pct; ★ff_wave_fire_danger=wave_danger; } ★sub_goal = goal; return; }   // 1602~1610
// 1614~1629
if with_dive && is_dive_current_tower && is_in_tower_range && !dive_viable {
    let safe_low_hp = target_hp_ratio < 70 && !(focused_is_in_tower || tower_close_danger || warn_tower || is_in_tower_focused);   // 1618
    if !(safe_low_hp || model_commits) {
        if (focused_is_in_tower || is_in_tower_focused) && near_allies.len()>1 { ★chats.push(BattleStop(TowerDiveFail)) }   // 1623~1624
        ★dive_abandoned=true; ★dive_abort_src=12; ★exit_src=10; ★sub_goal=End; return;   // 1626~1629
    }
}
// 1633~1671
let situation = FightSituation::build(version, rnd, player, data, team_plan, debug, near_allies.len(), near_enemies.len(), can_near_enemies, die_tick_with_tower, die_tick, focused, focused_die_tick, focused_is_in_range, warn_tower, is_in_tower_range, is_in_tower_focused, is_dive_current_tower, focused_is_in_tower, target_hp_ratio);
★tactic = Standard;                                                            // 1641
let die_tick_mult = if near_enemies.len() > 1 {
    let m = if i_am_soaker { ★tactic=Frontline; 50 } else if situation.my_battle_role == Utility { ★tactic=Peel; 100 } else if role ∈ {BaseAttacker,SkillCaster} { ★tactic=BacklineDPS; 150 } else { 100 };   // 1647~1653
    if situation.ally_dps_sum > 0 && situation.enemy_dps_sum > 0 { clamp(enemy_dps_sum*100/ally_dps_sum, 50, 150) } else if near_enemies.len()==2 { 100 } else { m }   // 1662~1664
} else { 100 };
let tps_1 = die_tick_mult*tps/100; let tps_2 = 2*tps*die_tick_mult/100; let tps_3 = 3*tps*die_tick_mult/100;   // 1669~1671
// 1675~1716
let v50_ally_absorbs = tower_on_other && !is_in_tower_focused;
if !v50_ally_absorbs && (warn_tower || target_hp_ratio > 49) && (focused_closer_to_tower || is_in_tower_range) && !is_dive_current_tower && self.with_runaway(data) && !model_commits {   // 1676
    let dive_winnable = tower_dive_is_viable(version, rnd, player, data, team_plan, focused, /*team_model=*/false, debug);   // 1679
    if context.debug && i_am_soaker { add_log }   // 1681~1682
    let legacy = version<=1 || with_dive;
    let v3_promote_ok = if legacy || self.dive_abandoned { false } else { !open_chase_race_hopeless(version, data, player, champ, focused) };   // 1689~1690
    if dive_winnable && (if legacy||dive_abandoned { legacy } else { v3_promote_ok || with_dive }) {   // 1691
        if v3_promote_ok { ★with_dive=true; ★dive_tower = iter_towers_without_nexus(1-team).min_by_key(|t| t.distance_sq(focused)).and_then(|t| (t.ty==Tower).then(|| t.ty.Tower.info.ty)); ★dive_join_tick=cache.tick(); }   // 1708~1716 (legacy 는 아무것도 안 함)
    } else {
        if !legacy { ★dive_abandoned=true; ★dive_abort_src=9; }             // 1695~1697
        ★exit_src=15; ★sub_goal=KitingBack{focused.id};                      // 1699~1700
        if focused_is_in_tower { if near_allies.len()>1 { ★chats.push(BattleStop(TowerDiveFail)) } ★sub_goal=RunAway; }   // 1701~1705
        return;
    }
}
if context.debug { add_log }                                                  // 1721~1725
// 1734~1768
let prediction = model_prediction.unwrap_or_else(|| resolve_fight_full(version, data, champ, &near_allies, &near_enemies, committed_dir, /*tower=*/None, judge_accuracy, baseline));   // 1734~1735 (sG_0)
let resolver_join_pending = prediction.line == CommitAfterJoin;              // 1736
match prediction.line {
    Commit => { if self.v46_brace { if context.debug { add_log } ★v46_brace=false; } ★sub_goal = Trace{prediction.focus_target.unwrap_or(focused.id)}; return; }   // 1743~1751
    Disengage => { ★exit_src=11; if self.with_runaway(data) && committed_dir != 1 { if version>1 && die_tick > tps_1 { ★sub_goal=KitingBack{focused.id} } else { if near_allies.len()>1 { ★chats.push(BattleStop(Outnumbered)) } ★sub_goal=RunAway } } else { ★sub_goal=KitingBack{focused.id} } return; }   // 1756~1768
    CommitAfterJoin | Hold => { /* 1778~1861 사다리 */
        let (A,E,FIR,FDT,DT,CNE) = (near_allies.len(), near_enemies.len(), focused_is_in_range, focused_die_tick, die_tick, can_near_enemies);
        if A < E {
            if FIR && FDT <= DT && with_runaway() { ★sub_goal=KitingBack{focused.id}; return; }   // 1779~1780
            else if CNE != 0 && DT <= tps_3 && with_runaway() { if A>1 { ★chats.push(BattleStop(Outnumbered)) } ★exit_src=12; ★sub_goal=RunAway; return; }   // 1784~1789
            else if DT > tps_3 || !with_runaway() { ★sub_goal=Kiting{focused.id} }   // 1793, 1797
            else { ★exit_src=12; ★sub_goal=RunAway }                          // 1794~1795
        } else if A > E {
            if FIR && FDT <= DT { ★sub_goal=Trace{focused.id}; return; }      // 1800~1801 (1871 후처리 생략)
            else if FDT > DT { if DT < tps_3 { if DT < tps_2 { if DT < tps_1 { if with_runaway() { if A>1 { ★chats.push(BattleStop(LowHp)) } ★exit_src=12; ★sub_goal=RunAway } else { ★sub_goal=Kiting } }   // 1811~1829
                                                            else { if with_runaway() { ★sub_goal=KitingBack } else { ★sub_goal=Kiting } } }   // 1816~1819
                                              else { ★sub_goal=Kiting } }     // 1814
                               else { ★sub_goal=Trace } }                     // 1812
            else { ★sub_goal = if DT > tps_2 { Trace } else { Kiting } }     // 1806~1809
        } else { // A == E
            if FIR && FDT <= DT { ★sub_goal=Kiting{focused.id}; return; }    // 1833~1834
            else if CNE > 1 && DT <= tps_3 && with_runaway() { if A>1 { ★chats.push(BattleStop(BurstRisk)) } ★exit_src=12; ★sub_goal=RunAway; return; }   // 1838~1843
            else if FDT > DT { if DT > tps_2 { ★sub_goal = if (focused_is_in_tower || DT <= tps_3) && with_runaway() { KitingBack } else { Kiting } }   // 1848, 1858~1861
                               else { if with_runaway() { if A>1 { ★chats.push(BattleStop(LowHpMatch)) } ★exit_src=12; ★sub_goal=RunAway } else { ★sub_goal=Kiting } } }   // 1849~1856
            else { ★sub_goal=Kiting }                                          // 1847
        }
        // 1871~1895 후처리 (focus 는 전부 focused.id)
        if model_prediction.is_some_and(|p| p.line == Hold) && committed_dir != 0 {
            let new_dir = match sub_goal { Trace=>1, KitingBack|RunAway=>-1, _=>skip };   // 1873 engage_dir
            if new_dir != committed_dir { if committed_dir==1 && DT > tps_1 { ★sub_goal=Trace{focused.id} }   // 1875
                else if committed_dir==-1 { let oor = !cache.iter_champions(1-team).any(|e| e.distance_sq(champ) <= (engage_pair_range(version,data,champ,e)+30000)^2 && blackboard[1-team].is_recent_visible(e) && !is_ignored_battle_enemy(e,with_dive)); if oor { ★sub_goal=KitingBack{focused.id} } } }   // 1877~1884
        }
        if committed_dir==1 && DT > tps_1 && sub_goal == RunAway { ★sub_goal=KitingBack{focused.id} }   // 1893~1895
        // 1899~1957
        if self.tactic == BacklineDPS && A < 2 && E > 1 { if sub_goal ∈ {Trace,Kiting} { ★sub_goal@tag = KitingBack (focus 유지) } }   // 1899~1902
        else if sub_goal ∈ {Trace,Kiting} {                                   // 1914
            if self.with_runaway(data) {                                      // 1915
                if FDT > DT {                                                 // 1920
                    let refuge = iter_towers_without_nexus(team).min_by_key(|t| t.distance_sq(champ)).or(cache.nexus[team]);   // 1928~1929
                    if let Some(refuge) = refuge {
                        let reaction = champ.attack_duration() + 6; let escape_ticks = champ.distance(refuge) / max(champ.move_speed,1);   // 1931~1932
                        if !self.v46_brace { if !resolver_join_pending && DT <= escape_ticks + reaction { ★v46_brace=true; if debug { add_log } } }   // 1933~1941
                        else if DT > escape_ticks + reaction { if debug { add_log } ★v46_brace=false; }   // 1945~1953
                    }
                    if self.v46_brace { ★sub_goal=KitingBack{focused.id} }   // 1956~1957
                } else { if v46_brace && debug { add_log } ★v46_brace=false; }   // 1922~1926
            } else { ★v46_brace=false; }                                      // 1919
        }
        // 1964~2001
        if self.tactic == Frontline { if let Some(&t) = near_enemies.iter().max_by_key(|e| dps_ratio(context, e, champ)) { if t.id != focused.id { match sub_goal { Trace=>★Trace{t.id}, Kiting=>★Kiting{t.id}, KitingBack=>★KitingBack{t.id}, _=>{} } } } }   // 1965~1977
        if self.tactic == SkillBurst { if !champ.can_skill() && !champ.can_skill2() { let my_attack_dps = champ.attack_effect.map(|e| e.expected_damage_target(context,champ,focused)*100000/max(champ.attack_cooltime(),1)).unwrap_or(0?); let my_total_dps = dps_ratio(context, champ, focused); if my_total_dps != 0 && my_attack_dps*100/my_total_dps < 30 && sub_goal ∈ {Trace,Kiting} { ★sub_goal@tag=KitingBack } } }   // 1987~2000 ※이 함수는 tactic 을 SkillBurst 로 절대 안 놓음 → 함수 내부 기준 사장 코드
        // 2016~2029
        if version>1 && nexus_last_stand(player, data) { match sub_goal { RunAway => ★sub_goal=Trace{focused.id}, End => {}, _ => ★sub_goal=Trace{기존 focus} } }
    }
}
// 2032: die_enemies·threat_enemies·threat_list·can_near_list·near_allies·near_enemies drop(bump) 후 return
```

**`mem` 메모리 접근 108건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | BattlePlan | 0x0 | support_target@tag | r | L923/932/960 Option<usize> 태그 | 4 | OK |  |
| 1 | BattlePlan | 0x8 | support_target@Some.0 | r | L923/932/960 | 4 | OK |  |
| 2 | BattlePlan | 0x40 | main_goal@tag | r | with_runaway 인라인(903,1591): TryKill(0) 판정 | 4 | OK |  |
| 3 | BattlePlan | 0x58 | sub_goal@tag | r | 750/783/818/1894/1970/1998/2021 | 4 | OK |  |
| 4 | BattlePlan | 0x60 | sub_goal.focus | r | 818(Trace/Assassin/AssassinReady focus)·2021 | 4 | OK |  |
| 5 | BattlePlan | 0x68 | chats (Vec<Chat> cap/ptr/len +0x68/+0x70/+0x78) | r | push 12곳에서 len·cap 읽음 | 4 | OK |  |
| 6 | BattlePlan | 0x98 | dive_catch_break_ticks | r | 1400/1403 | 4 | OK |  |
| 7 | BattlePlan | 0xa0 | dive_model_break_ticks | r | 1433 | 4 | OK |  |
| 8 | BattlePlan | 0xa8 | prev_die_eval | r | 887 | 4 | OK |  |
| 9 | BattlePlan | 0xb0 | dive_ctx_break_ticks | r | 1250 | 4 | OK |  |
| 10 | BattlePlan | 0xc0 | start_tick | r | 768/1592/with_runaway(369) | 4 | OK |  |
| 11 | BattlePlan | 0xf6 | with_dive | r | 16곳. 클로저에 &로 전달돼 is_ignored_battle_enemy 의 with_declared_dive | 4 | OK |  |
| 12 | BattlePlan | 0xf7 | dive_entered | r | 1253/1403/1531 | 4 | OK |  |
| 13 | BattlePlan | 0xfa | dive_abandoned | r | 1689 | 4 | OK |  |
| 14 | BattlePlan | 0xfb | v46_brace | r | 1743/1922/1933/1956 | 4 | OK |  |
| 15 | BattlePlan | 0xfd | tactic | r | 1899/1964/1987 | 4 | OK |  |
| 16 | BattlePlan | 0xfe | dive_tower (Option<TowerType>, 255=None) | r | 1116/1230/1313/1334 | 4 | OK |  |
| 17 | BattlePlan | 0xff | main_objective@tag (255=None) | r | 996/1171/1202/with_runaway(374)/should_return_to_objective | 4 | OK |  |
| 18 | BattlePlan | 0x100 | main_objective@Some.0.phase | r | 1172 Morgard/Serpen phase==Hunt(3) | 4 | OK |  |
| 19 | BattlePlan | 0x101 | main_objective@Some.0.with_battle | r | 1171 (objective_entity_id 인자 조립용 로드) | 4 | OK |  |
| 20 | BattlePlan | 0x102 | die_fall_evals | r | 890 | 4 | OK |  |
| 21 | BattlePlan | 0x109 | ff_wave_obs_open | r | 1595 ==255 미기록 판정 | 4 | OK |  |
| 22 | BattlePlan | 0x10c | ff_wave_fire_hp | r | 1605 ==255 미기록 판정 | 4 | OK |  |
| 23 | BattlePlan | 0x110 | ff_exit1_cls | r | 970 ==255 미기록 판정 | 4 | OK |  |
| 24 | PlayerState | 0x930 | info.team | r | 774 및 클로저 | 4 | OK |  |
| 25 | PlayerState | 0x9c0 | info.position@tag | r | 774 as_index | 4 | OK |  |
| 26 | PlayerState | 0x180 | info.parameter (AthleteParameter 744B) | r | judge_accuracy() 인자 1052/1055/1076/1079/1735 | 4 | OK |  |
| 27 | OperationData | 0x0 | cache (&AbstractGameWithCache) | r |  | 4 | OK |  |
| 28 | OperationData | 0x8 | context (&GameContext) | r |  | 4 | OK |  |
| 29 | OperationData | 0x10 | blackboard (&[Blackboard;2]) | r | is_recent_visible 인자, s2_0 big_goal 스캔 | 4 | OK |  |
| 30 | AbstractGameWithCache | 0x0 | game.data_ptr | r | &dyn AbstractGame 팻포인터 | 4 | OK |  |
| 31 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r | vtable+0x28=tick() · vtable+0x1f0=get_entity_by_id() | 4 | OK |  |
| 32 | AbstractGameWithCache | 0x170 | nexus[team] | r | 1929 refuge 폴백 | 4 | OK |  |
| 33 | AbstractGameWithCache | 0x1e0 | player_champion | r | 774 champ, near_allies, dive_entry_soaker | 4 | OK |  |
| 34 | GameContext | 0x0 | pool (&Bump) | r | bumpalo Vec 할당자 | 4 | OK |  |
| 35 | GameContext | 0x8 | setting (&GameSetting) | r |  | 4 | OK |  |
| 36 | GameContext | 0x3b | debug | r | 계측 로그 게이트(1317/1348/1378/1408/1439/1482/1494/1681/1721/1744/1922/1938/1949) | 4 | OK |  |
| 37 | GameSetting | 0x12f8 | tick_per_second | r | 771/879/with_runaway | 4 | OK |  |
| 38 | TeamPlan | 0x0 | ally_battle_stop_tick[] | r | 798 None 인 아군만 near_allies | 4 | OK |  |
| 39 | TeamPlan | 0x228 | last_resolver_bail_tick | r | 769~771 | 4 | OK |  |
| 40 | Entity | 0x0 | team@tag | r | TeamType eq (Player=0) | 4 | OK |  |
| 41 | Entity | 0x8 | team@Player.0 | r |  | 4 | OK |  |
| 42 | Entity | 0x68 | ty@tag | r | Tower(2)/Champion(13) | 4 | OK |  |
| 43 | Entity | 0x70 | ty@Champion.action_state@tag | r | s2_0: Move(2) | 4 | OK |  |
| 44 | Entity | 0x78 | ty@Champion.action_state@Move.x | r | s2_0 | 4 | OK |  |
| 45 | Entity | 0x80 | ty@Champion.action_state@Move.y | r | s2_0 | 4 | OK |  |
| 46 | Entity | 0x88 | ty@Tower.info.nearest_enemy@tag | r | 908/1295 | 4 | OK |  |
| 47 | Entity | 0x98 | ty@Tower.info.nearest_enemy@Some.1 (id) | r | 910/1296 | 4 | OK |  |
| 48 | Entity | 0x128 | ty@Tower.info.ty (TowerType) | r | 857/1715 | 4 | OK |  |
| 49 | Entity | 0x438 | stat_buff_cached.range | r | Effect::range 인라인 | 4 | OK |  |
| 50 | Entity | 0x470 | stat_buff_cached.radius_mult | r | Entity::radius 인라인 | 4 | OK |  |
| 51 | Entity | 0x4a0 | attack_effect@Some.range | r |  | 4 | OK |  |
| 52 | Entity | 0x4a8 | attack_effect@Some.growth_range | r |  | 4 | OK |  |
| 53 | Entity | 0x4c0 | attack_effect@tag (-1=None → unwrap 패닉) | r |  | 4 | OK |  |
| 54 | Entity | 0x5c0 | id | r |  | 4 | OK |  |
| 55 | Entity | 0x5c8 | level | r | Effect::range (level-1)*growth | 4 | OK |  |
| 56 | Entity | 0x628 | stat_cached.hp (최대HP) | r | 1333/1598/1606/dive_entry_soaker | 4 | OK |  |
| 57 | Entity | 0x640 | stat_cached.move_speed | r | 1145/1932 | 4 | OK |  |
| 58 | Entity | 0x660 | x | r |  | 4 | OK |  |
| 59 | Entity | 0x668 | y | r |  | 4 | OK |  |
| 60 | Entity | 0x670 | hp | r |  | 4 | OK |  |
| 61 | Entity | 0x680 | radius | r |  | 4 | OK |  |
| 62 | Entity | 0x6a0 | block_target_tick | r | 1039 타워 필터 | 4 | OK |  |
| 63 | Entity | 0x6b9 | can_target | r | 1039 타워 필터 | 4 | OK |  |
| 64 | Blackboard | 0xf0 | big_goal[i] (stride 32: +0xf8 tag, +0x100 focus tag, +0x108 focus id) | r | s2_0 832~833 | 4 | OK |  |
| 65 | FightPrediction | 0x0 | focus_target | r | 1750 | 4 | OK |  |
| 66 | FightPrediction | 0x10 | soaker | r | 1110 | 4 | OK |  |
| 67 | FightPrediction | 0x20 | rescue_ally@tag | r | 1086 | 4 | OK |  |
| 68 | FightPrediction | 0x38 | line | r | 1057/1099/1385/1427/1736/1871 | 4 | OK |  |
| 69 | FightPrediction | 0x39 | line_absolute | r | 1091~1093 (IR: line ← line_absolute) | 4 | OK |  |
| 70 | FightSituation | 0x60 | ally_dps_sum | r | 1662 | 4 | OK |  |
| 71 | FightSituation | 0x68 | enemy_dps_sum | r | 1662~1663 | 4 | OK |  |
| 72 | FightSituation | 0x79 | my_battle_role | r | 1649/1653 | 4 | OK |  |
| 73 | Strategy | 0xf | object_finish (PlayerState::strategy 반환 24B) | r | 1176 KillPriority(0) | 4 | OK |  |
| 74 | BattlePlan | 0x0 | support_target | w | 928(대상 소실/근접/비가시/무시대상) · 935(v21_should_defer_support_target) | 4 | OK | None(tag 0) |
| 75 | BattlePlan | 0x58 | sub_goal@tag | w | 60개 store. 785,904,998,1014,1059,1161,1164,1204,1265,1278,1362,1419,1450,1474,1477,1516,1519,1532,1547,1554,1572,1576,1581,1610,1629,1700,1705,1751,1762,1766,1768,1780,1789,1795,1797,1801,1807,1809,1812,1814,1817,1819,1827,1829,1834,1843,1847,1854,1856,1859,1861,1875,1884,1895,1902,1957,2000,2029 (+1498/1505 phi) | 4 | OK | 0 Trace/2 Kiting/3 KitingBack/4 RunAway/7 End 또는 wave_goal |
| 76 | BattlePlan | 0x60 | sub_goal.focus | w | 34개 store, 태그 store 와 짝 | 4 | OK | focused.id / kite_focus / prediction.focus_target / target.id / wave_goal.1 |
| 77 | BattlePlan | 0x78 | chats.len (+ chats.ptr[len] 에 Chat 24B write, cap 부족 시 grow_one(+0x68)) | w | Chat::BattleStop(reason) push 12곳: 1259(TowerDiveFail=1) 1413(1) 1443(1) 1568(TowerFocused=0) 1579(0) 1624(1) 1703(1) 1765(Outnumbered=2) 1786(2) 1824(LowHp=3) 1840(BurstRisk=4) 1851(LowHpMatch=5). 전부 near_allies.len()>1 조건 | 4 | OK | len+1 |
| 78 | BattlePlan | 0x98 | dive_catch_break_ticks | w | 1398 리셋 · 1400 증가 | 4 | OK | 0 / +1 |
| 79 | BattlePlan | 0xa0 | dive_model_break_ticks | w | 1433 증가 · 1447/1454 리셋 | 4 | OK | +1 / 0 |
| 80 | BattlePlan | 0xa8 | prev_die_eval | w | 888 무조건 | 4 | OK | die_tick(committed_dir 보정 후) |
| 81 | BattlePlan | 0xb0 | dive_ctx_break_ticks | w | 1244 리셋 · 1250 증가 | 4 | OK | 0 / +1 |
| 82 | BattlePlan | 0xb8 | dive_join_tick | w | 1716 (v3 다이브 승격 시) | 4 | OK | cache.tick() |
| 83 | BattlePlan | 0xc8 | ff_rescue_tick | w | 1094 (rescue_ally 존재 && !helping) | 4 | OK | cache.tick() |
| 84 | BattlePlan | 0xf0 | dive_last_race_adv (i32) | w | 1388 | 4 | OK | min(die_tick_with_tower,2000)-min(focused_die_tick,2000) |
| 85 | BattlePlan | 0xf6 | with_dive | w | 1198,1260,1272,1414,1444,1470,1498,1512 = 다이브 포기 · 1712 = v3 승격 | 4 | OK | false(8곳) / true(1712) |
| 86 | BattlePlan | 0xf7 | dive_entered | w | 1236 (v50_dive_episode && is_in_tower_range) | 4 | OK | true |
| 87 | BattlePlan | 0xfa | dive_abandoned | w | 1151,1200,1262,1274,1416,1446,1472,1500,1514,1570,1626,1696 | 4 | OK | true |
| 88 | BattlePlan | 0xfb | v46_brace | w | 1748,1919,1926,1953 = false | 4 | OK | true(1937) / false |
| 89 | BattlePlan | 0xfd | tactic | w |  | 4 | OK | Standard(0)@1641 → Frontline(1)@1647/1459/1526, Peel(4)@1649, BacklineDPS(2)@1653 |
| 90 | BattlePlan | 0xfe | dive_tower | w | 1199,1261,1273,1415,1445,1471,1499,1513 | 4 | OK | None(255) 8곳 / Some(TowerType)@1713 |
| 91 | BattlePlan | 0x102 | die_fall_evals | w | 890 / 892 | 4 | OK | saturating_add(1) / 0 |
| 92 | BattlePlan | 0x103 | dive_abort_src | w | [D1 계측] exit_src=10 발원 코드표 | 4 | OK | 9@1152/1697 · 6@1201 · 2@1263 · 13@1275 · (focused_die_tick<die_tick_with_tower ? 11 : 1)@1417 · 12@1627 |
| 93 | BattlePlan | 0x104 | dive_last_model | w | 1385~1386 | 4 | OK | None\|CommitAfterJoin→0, Commit→1, Hold→2, Disengage→3 |
| 94 | BattlePlan | 0x105 | dive_last_na | w | 1389 | 4 | OK | min(near_allies.len(),255) |
| 95 | BattlePlan | 0x106 | dive_last_ne | w | 1390 | 4 | OK | min(near_enemies.len(),255) |
| 96 | BattlePlan | 0x108 | exit_src | w | [ff 계측] 이탈 발원 코드표 | 4 | OK | 8@784/1571/1575 · 1@967 · 2@1013 · 13@1058 · 7@1154/1543 · 4@1178~1185 · 6@1203 · 10@1264/1277/1418/1449/1473/1501/1515/1628 · 9@1603 · 15@1699 · 11@1756 · 12@1788/1794/1826/1842/1853 |
| 97 | BattlePlan | 0x109 | ff_wave_obs_open | w | 1596 (255 일 때만 1회) | 4 | OK | wave_obs |
| 98 | BattlePlan | 0x10a | ff_wave_open_hp | w | 1598 | 4 | OK | min(champ.hp*100/max(maxhp,1),254) |
| 99 | BattlePlan | 0x10b | ff_wave_open_pct | w | 1599 | 4 | OK | wave_pct |
| 100 | BattlePlan | 0x10c | ff_wave_fire_hp | w | 1606 (255 일 때만) | 4 | OK | min(hp%,254) |
| 101 | BattlePlan | 0x10d | ff_wave_fire_pct | w | 1607 | 4 | OK | wave_pct |
| 102 | BattlePlan | 0x10e | ff_wave_open_danger | w | 1600 | 4 | OK | wave_danger |
| 103 | BattlePlan | 0x10f | ff_wave_fire_danger | w | 1608 | 4 | OK | wave_danger |
| 104 | BattlePlan | 0x110 | ff_exit1_cls | w | 984 (exit_src=1 && 255 일 때만): 250k 내 적 분류 invis/well/towerf | 4 | OK | 0/1/2/3/4 |
| 105 | BattlePlan | 0x111 | exit_sub | w | 1178~1185, exit_src=4 와 짝 | 4 | OK | 6(objective 엔티티 없음) / 7(can_enemy_hit_objective 거짓) |
| 106 | DebugFrameData | 0xa0 | HashMap<usize,Vec<String>> entry(champ.id).or_insert(vec![]).push(format!) | w | 1318~1319, context.debug 일 때만 (힙 String/Vec 할당) | 4 | 오귀속(사전은 다른 필드를 준다) | 계측 문자열 ×2 |
| 107 | DebugFrameData | 0x0 | add_log(&mut self, data, String) | w | 1349/1379/1409/1440/1483/1494/1682/1723/1745/1923/1939/1950 — context.debug 게이트 | 4 | 오귀속(사전은 다른 필드를 준다) | 계측 문자열 ×12 |

**`consts` 상수 39건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev | folded_from |
|---|---|---|---|---|---|---|
| 0 | 1 | 767 | 임계 | `version > 1`(icmp ugt %1,1) = v2+ 게이트. 같은 값 1 이 `shl i64 tps,1`(=tps*2: 902/1253/1290/1438/1670) 와 `lshr tps,1`(=tps/2: 1336/1466/1492/1511/1531) 의 시프트량으로도 쓰임 | 4 | 2 |
| 1 | 3 | 771 | 태그 | last_resolver_bail_tick + tps*3 (`mul %177,3`) · 1671 tps_3 = 3*tps*mult/100 · 1480 hold_margin = tps/3(committed_dir=-1) · TowerType Twin 판정 `add -3`. ※본문의 `shl … , 3` 은 &Entity 슬라이스 stride(×8) 산술이지 임계 아님(시프트량 아님) | 4 |  |
| 2 | 150000 | 811 | 미상 | can_near_enemies_range 반경(can_near_list · threat_list) | 4 |  |
| 3 | 30000 | 792 | 계수 | engage_pair_range + 30000 (제곱비교) — near_enemies 페어링 여유(closure0), 1089 rescue helping 판정, 1880 oor 판정 | 4 |  |
| 4 | 14400000001 | 799 | 미상 | 120000^2 + 1 — near_allies 반경(aux m01, `icmp ult`) | 4 |  |
| 5 | 15000 | 850 | 계수 | nearest_enemy_tower 필터: dist <= 타워사거리+15000+반경합 (853) · 1043 target_tower 도 동일 | 4 |  |
| 6 | 10000 | 917 | 계수 | is_in_tower_range: dist_sq <= (타워사거리+10000+반경합)^2 · 1330 focused_is_in_tower 여유 | 4 |  |
| 7 | 100 | 850 | 계수 | radius_mult 백분율(radius*(mult+100)/100) · hp% 계산 · die_tick_mult 기본값 100 · tps_n = n*tps*mult/100 | 4 |  |
| 8 | 3600000001 | 925 | 임계 | 60000^2 + 1 — support_target 이 champ 60000 이내면 해제 | 4 |  |
| 9 | 250000 | 939 | 산출값 | chase_range: near_enemies 비었을 때 focused 탐색 반경(제곱비교) | 4 |  |
| 10 | 62500000000 | 973 | 임계 | 250000^2 — ff_exit1_cls 계측용 주변 적 반경 | 4 |  |
| 11 | 90000000001 | 1206 | 임계 | 300000^2 + 1 — 적 우물 앵커 근접 시 RunAway(아니면 End) | 4 |  |
| 12 | 960000 | 1206 | 산출값 | 우물 앵커 좌표(team 0 → (960000,0), team 1 → (0,960000)) well_damage_anchor 인라인 | 4 |  |
| 13 | 22500000001 | 1008 | 미상 | 150000^2 + 1 — near_focused_enemies / target_enemies 반경(aux sh_0/sk_0) | 4 |  |
| 14 | 40000000001 | 1375 | 임계 | 200000^2 + 1 — dive_entry_soaker(fight_model.rs:1073) 아군 후보 반경 [인라인 콜리 줄 1073 → 루트 1375 · 20차 G] | 4 |  |
| 15 | 1600000001 | 1291 | 임계 | 40000^2 + 1 — tower_close_danger: champ 가 적 타워 40000 이내 | 4 |  |
| 16 | 11 | 1305 | 임계 | my_die_tick_with_tower < 11 → warn_tower | 4 |  |
| 17 | 25000 | 1337 | 계수 | close_execute_range = max_range + 25000 · 1185 can_enemy_hit_objective range_margin | 4 |  |
| 18 | 30 | 1340 | 임계 | target_hp_ratio < 30 = 처형권 · 1997 my_attack_dps*100/my_total_dps < 30 | 4 |  |
| 19 | 49 | 1676 | 임계 | target_hp_ratio > 49 (=>=50) 조건 | 4 |  |
| 20 | 70 | 1618 | 임계 | target_hp_ratio < 70 이면 다이브 중단 면제(안전 조건과 AND) | 4 |  |
| 21 | 2000 | 1388 | 임계 | dive_last_race_adv 계산 시 die_tick 상한 캡 | 4 |  |
| 22 | 254 | 1598 | 인덱스 | ff_wave_*_hp 캡(255 = 미기록 표식과 구분) | 4 |  |
| 23 | 255 | 1389 | 인덱스 | dive_last_na/ne u8 캡 · 0xff/None 표식 판정은 -1 로 비교 | 4 |  |
| 24 | 50 | 1647 | 임계 | die_tick_mult: i_am_soaker → 50 · clamp 하한 50 | 4 |  |
| 25 | 150 | 1653 | 임계 | die_tick_mult: BaseAttacker/SkillCaster → 150 · clamp 상한 150 | 4 |  |
| 26 | 6 | 1931 | 임계 | reaction = champ.attack_duration()+6 · 1480 hold_margin = tps/6(committed_dir != -1) | 4 |  |
| 27 | 10 | 1145 | 태그 | focused.move_speed*10 < champ.move_speed*9 (focused 가 champ 의 90% 미만 속도) 판정 · MainObjective PressTower(10) 태그 | 4 |  |
| 28 | 9 | 1145 | 계수 | 위 속도비 90% · exit_src=9(wave) · dive_abort_src=9 | 4 |  |
| 29 | 100000 | 1990 | 미상 | my_attack_dps = expected_damage*100000/max(attack_cooltime,1) (aux sL_0) | 4 |  |
| 30 | 768 | 1177 | 미상 | objective_entity_id_for_main_objective 인자 i24 조립: phase 바이트 := Hunt(3)<<8 (1172 에서 확정된 값의 상수접힘) | 4 |  |
| 31 | 13 | 837 | 태그 | EntityType::Champion 태그 · exit_src 13(rejoin.line != Commit) · dive_abort_src 13 | 4 |  |
| 32 | 2 | 856 | 태그 | EntityType::Tower 태그 · BattleSubPlanGoal Kiting · FightLine Disengage · exit_src 2 · dive_abort_src 2 · `version < 2` 레거시 게이트(1170) | 4 |  |
| 33 | 4 | 750 | 태그 | BattleSubPlanGoal::RunAway 태그(750 조기 return · 다수 store) · BattleTactic Peel · BattleRole Utility · MainObjective Nexus · exit_src 4 | 4 |  |
| 34 | 7 | 1014 | 태그 | BattleSubPlanGoal::End 태그 · exit_src 7 · Chat::BattleStop 태그 7 | 4 |  |
| 35 | 5 | 832 | 태그 | BigGoal::Battle 태그(aux s2_0) · MainObjective PressEpic(with_runaway) · StopReason LowHpMatch | 4 |  |
| 36 | 8 | 784 | 산출값 | exit_src 8 (survival_incoming / 사거리 밖 타워권 이탈) | 4 |  |
| 37 | 12 | 1627 | 산출값 | dive_abort_src 12 · exit_src 12(사다리 RunAway) | 4 |  |
| 38 | 15 | 1699 | 산출값 | exit_src 15 (v3 승격 실패 KitingBack) | 4 |  |

**`knobs` 조정점 20건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 리졸버패배 이탈 커밋 상속 창 | battle.rs:771 | 3 | tps*3 — 올리면 리졸버패배 후 더 오래 committed_dir=-1(이탈) 상속 | 4 | 기존 |
| 1 | near_enemies 페어링 여유 | battle.rs:792 (closure0) | 30000 | 올리면 더 먼 적까지 교전 참여자로 봄(die_tick·focused 후보 확대) | 4 | 기존 |
| 2 | near_allies 반경 | battle.rs:799 | 14400000001 | 120000^2+1. 올리면 더 먼 아군도 동행으로 계산(A 증가 → 사다리 분기 이동) | 4 | 기존 |
| 3 | can_near/threat 반경 | battle.rs:811/825 | 150000 | can_near_enemies_range 반경 | 4 | 기존 |
| 4 | nearest_enemy_tower 인정 여유 | battle.rs:850 | 15000 | 타워 사거리+15000+반경합 안이어야 타워 판정에 포함 | 4 | 기존 |
| 5 | is_in_tower_range 여유 | battle.rs:917 | 10000 | 올리면 타워권 판정이 관대해져 이탈/KitingBack 빈발 | 4 | 기존 |
| 6 | support_target 해제 근접거리 | battle.rs:925 | 3600000001 | 60000^2+1. 대상이 이 안이면 support_target 해제 | 4 | 기존 |
| 7 | chase_range | battle.rs:939 | 250000 | near_enemies 비었을 때 focused 탐색 반경 | 4 | 기존 |
| 8 | die_tick 하락 재평가 임계 | battle.rs:902 | 2 | die_tick <= tps*2 && die_fall_evals >= 2 → KitingBack→RunAway 강등 | 4 | 기존 |
| 9 | tower_close_danger 거리 | battle.rs:1291 | 1600000001 | 40000^2+1 | 4 | 기존 |
| 10 | warn_tower 절대 임계 | battle.rs:1305 | 11 | my_die_tick_with_tower < 11 틱이면 무조건 warn_tower | 4 | 기존 |
| 11 | close_execute_range 여유 | battle.rs:1337 | 25000 | max_range+25000 안에서 처형 판정 | 4 | 기존 |
| 12 | 처형 HP% | battle.rs:1340 | 30 | target_hp_ratio < 30 이면 ready_damage 무관 처형권 | 4 | 기존 |
| 13 | 다이브 중단 면제 HP% | battle.rs:1618 | 70 | target_hp_ratio < 70 && 안전조건 → dive 중단(abort_src 12) 면제 | 4 | 기존 |
| 14 | 다이브 승격 HP 조건 | battle.rs:1676 | 49 | target_hp_ratio > 49 (또는 warn_tower) 일 때만 1676 블록 진입 | 4 | 기존 |
| 15 | die_tick_mult 소커/역할 | battle.rs:1647~1653 | 50/100/150 | tps_1~3 스케일: 소커 50 → 더 빨리 RunAway/Kiting, BacklineDPS 150 → 더 오래 버팀 | 4 | 기존 |
| 16 | dps 비 clamp | battle.rs:1663 | 50..150 | enemy_dps/ally_dps 비율 범위 | 4 | 기존 |
| 17 | v46 brace 반응 여유 | battle.rs:1931 | 6 | reaction = attack_duration+6 틱 | 4 | 기존 |
| 18 | SkillBurst 평타 비중 | battle.rs:1997 | 30 | my_attack_dps/total < 30% → KitingBack (단 tactic==SkillBurst 도달 불가 → 현재 무효) | 4 | 기존 |
| 19 | dive_last_race_adv 캡 | battle.rs:1388 | 2000 | 계측 전용 | 4 | 기존 |

<details><summary>`callees` 피호출자 61건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | add_log | game_core::DebugFrameData::add_log | pub | fn(&mut game_core::DebugFrameData, &game_core::OperationData, &game_core::PlayerState, std::string::String) | game-core\src\simulation\game\frame.rs:54 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | as_index | game_core::BigGoal::as_index | pub | fn(&game_core::BigGoal) -> usize | game-core\src\simulation\ai_interface.rs:338 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 2 | as_index | game_core::Position::as_index | pub | fn(&game_core::Position) -> usize | game-core\src\simulation\entity.rs:580 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 3 | attack_cooltime | game_core::Entity::attack_cooltime | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1766 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | attack_duration | game_core::Entity::attack_duration | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1770 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 5 | attack_range | game_ai::SmallActionLaneMinionPosition::attack_range | in:game_ai | fn(&game_core::Entity, &game_core::Entity) -> std::option::Option<u64> | game-ai\src\small_action\lane_minion.rs:354 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 6 | build | game_ai::plan_legacy::old::FightSituation::build | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData, usize, usize, usize, usize, usize, &game_core::Entity, usize, bool, bool, bool, bool, bool, bool, usize) -> game_ai::plan_legacy::old::FightSituation | game-ai\src\plan_legacy\old\fight_model.rs:129 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 7 | can_enemy_hit_objective | game_ai::plan_legacy::old::can_enemy_hit_objective | pub | fn(&game_core::Entity, &game_core::Entity, u64) -> bool | game-ai\src\plan_legacy\old\fight_model.rs:1188 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 8 | can_near_enemies_range | game_ai::plan_legacy::team_plan::TeamPlan::can_near_enemies_range | pub | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, u64, u64, u64, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< &game_core::Entity> | game-ai\src\plan_legacy\team_plan.rs:483 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 9 | can_skill | game_core::Entity::can_skill | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1708 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 10 | can_skill2 | game_core::Entity::can_skill2 | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1721 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 11 | check_kill_die_tick | game_ai::check_kill_die_tick | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::OperationData, &game_core::PlayerState, &game_core::Entity, bumpalo::collections::vec::Vec< &game_core::Entity>, bumpalo::collections::vec::Vec< &game_core::Entity>, &mut game_core::DebugFrameData) -> usize | game-ai\src\fight_check.rs:917 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 12 | distance | game_core::utils::distance | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:12 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 13 | distance | game_core::Entity::distance | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2161 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 14 | distance_sq | game_core::utils::distance_sq | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:6 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 15 | distance_sq | game_core::Entity::distance_sq | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2157 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 16 | dive_chase_catchable | game_ai::plan_legacy::old::dive_chase_catchable | pub | fn(&game_core::Entity, &game_core::Entity, usize) -> bool | game-ai\src\plan_legacy\old\fight_model.rs:919 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 17 | dive_entry_soaker | game_ai::plan_legacy::old::fight_model::dive_entry_soaker | in:game_ai | fn(&game_core::OperationData, &game_core::PlayerState, &game_core::Entity) -> std::option::Option<usize> | game-ai\src\plan_legacy\old\fight_model.rs:1069 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 18 | dps_ratio | game_ai::dps_ratio | pub | fn(&game_core::GameContext, &game_core::Entity, &game_core::Entity) -> usize | game-ai\src\utils.rs:430 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 19 | engage_pair_range | game_ai::plan_legacy::old::engage_pair_range | pub | fn(usize, &game_core::OperationData, &game_core::Entity, &game_core::Entity) -> u64 | game-ai\src\plan_legacy\old\battle.rs:2389 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 20 | expected_damage_target | game_core::Effect::expected_damage_target | pub | fn(&game_core::Effect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + , &game_core::Entity) -> usize | game-core\src\simulation\effect.rs:91 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 21 | fight_kit_range_cached | game_ai::plan_legacy::old::fight_kit_range_cached | pub | fn(&game_core::OperationData, &game_core::Entity, &game_core::Entity) -> u64 | game-ai\src\plan_legacy\old\battle.rs:2364 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 22 | fight_participants | game_ai::plan_legacy::old::fight_model::fight_participants | in:game_ai | fn(usize, &mut rand::rngs::std::StdRng, &game_core::OperationData, &game_core::PlayerState, &game_core::Entity, &[&game_core::Entity], &[&game_core::Entity], &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< (&game_core::Entity, i64, bool)> | game-ai\src\plan_legacy\old\fight_model.rs:610 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 23 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 24 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 25 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 26 | is_empty | game_core::PatchSetting::is_empty | pub | fn(&game_core::PatchSetting) -> bool | game-core\src\patch_engine.rs:853 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 27 | is_empty | game_core::ModAiRegistry::is_empty | pub | fn(&game_core::ModAiRegistry) -> bool | game-core\src\mod_ai.rs:15 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 28 | is_empty | game_core::PublicChampionEvidence::is_empty | pub | fn(&game_core::PublicChampionEvidence) -> bool | game-core\src\banpick\public_evidence.rs:96 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 29 | is_enemy_well_danger | game_ai::is_enemy_well_danger | pub | fn(usize, &game_core::PlayerState, u64, u64) -> bool | game-ai\src\path_finder.rs:1032 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 30 | is_ignored_battle_enemy | game_ai::plan_legacy::old::is_ignored_battle_enemy | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, bool) -> bool | game-ai\src\plan_legacy\old\fight_model.rs:887 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 31 | is_ignored_well_enemy | game_ai::plan_legacy::old::is_ignored_well_enemy | pub | fn(usize, &game_core::PlayerState, &game_core::Entity) -> bool | game-ai\src\plan_legacy\old\fight_model.rs:754 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 32 | is_recent_visible | game_core::Blackboard::is_recent_visible | pub | fn(&game_core::Blackboard, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\game\blackboard.rs:346 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 33 | is_twin | game_core::TowerType::is_twin | pub | fn(&game_core::TowerType) -> bool | game-core\src\simulation\entity\tower.rs:86 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 34 | is_unreasonable_tower_dive_enemy | game_ai::plan_legacy::old::is_unreasonable_tower_dive_enemy | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, bool) -> bool | game-ai\src\plan_legacy\old\fight_model.rs:792 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 35 | iter_champions | game_core::AbstractGameWithCache::<'a, 'b>::iter_champions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5759 ~ game_core[6a30]::simulation::{impl#5}::iter_champions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1904 | True | True | 3 | IR 호출 심볼 일치(토큰 경계) |
| 36 | iter_towers_without_nexus | game_core::AbstractGameWithCache::<'a, 'b>::iter_towers_without_nexus | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5750 ~ game_core[6a30]::simulation::{impl#5}::iter_towers_without_nexus::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1830 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 37 | judge_accuracy | game_core::AthleteParameter::judge_accuracy | pub | fn(&game_core::AthleteParameter) -> usize | game-core\src\simulation\state\player.rs:337 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 38 | max_range_cached | game_ai::plan_legacy::old::max_range_cached | pub | fn(&game_core::OperationData, &game_core::Entity, &game_core::Entity) -> u64 | game-ai\src\plan_legacy\old\battle.rs:2332 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 39 | nexus_last_stand | game_ai::plan_legacy::old::nexus_last_stand | pub | fn(&game_core::PlayerState, &game_core::OperationData) -> bool | game-ai\src\plan_legacy\old\defense_nexus.rs:186 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 40 | objective_entity_id_for_main_objective | game_ai::plan_legacy::old::objective_entity_id_for_main_objective | pub | fn(&game_core::OperationData, game_ai::plan_legacy::team_plan::MainObjective) -> std::option::Option<usize> | game-ai\src\plan_legacy\old\fight_model.rs:1178 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 41 | open_chase_race_hopeless | game_ai::plan_legacy::old::fight_model::open_chase_race_hopeless | in:game_ai | fn(usize, &game_core::OperationData, &game_core::PlayerState, &game_core::Entity, &game_core::Entity) -> bool | game-ai\src\plan_legacy\old\fight_model.rs:847 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 42 | push | <game_core::Staff as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::staff | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\staff.rs:38 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 43 | push | <game_core::Athlete as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:434 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 44 | push | <game_core::Contract as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:2498 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 45 | radius | game_core::Entity::radius | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1509 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 46 | ready_damage_to_target | game_ai::plan_legacy::old::fight_model::ready_damage_to_target | in:game_ai | fn(&game_core::GameContext, &game_core::Entity, &game_core::Entity) -> usize | game-ai\src\plan_legacy\old\fight_model.rs:759 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 47 | resolve_fight_full | game_ai::plan_legacy::old::fight_model::resolve_fight_full | in:game_ai::plan_legacy::old::fight_model | fn(usize, &game_core::OperationData, &game_core::Entity, &[&game_core::Entity], &[&game_core::Entity], i8, std::option::Option<&game_core::Entity>, usize, &[i64], i64) -> game_ai::plan_legacy::old::FightPrediction | game-ai\src\plan_legacy\old\fight_model.rs:324 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 48 | resolve_fight_stake | game_ai::plan_legacy::old::fight_model::resolve_fight_stake | in:game_ai | fn(usize, &mut rand::rngs::std::StdRng, &game_core::OperationData, &game_core::PlayerState, &game_core::Entity, &[&game_core::Entity], &[&game_core::Entity], i8, std::option::Option<&game_core::Entity>, usize, &mut game_core::DebugFrameData) -> game_ai::plan_legacy::old::FightPrediction | game-ai\src\plan_legacy\old\fight_model.rs:571 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 49 | resolve_fight_stake_roster | game_ai::plan_legacy::old::fight_model::resolve_fight_stake_roster | in:game_ai | fn(usize, &game_core::OperationData, &game_core::Entity, &[(&game_core::Entity, i64, bool)], &[&game_core::Entity], i8, std::option::Option<&game_core::Entity>, usize) -> game_ai::plan_legacy::old::FightPrediction | game-ai\src\plan_legacy\old\fight_model.rs:645 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 50 | soaker_tower_hold_components | game_ai::plan_legacy::old::fight_model::soaker_tower_hold_components | in:game_ai | fn(&game_core::OperationData, &game_core::PlayerState, &game_core::Entity) -> std::option::Option<(usize, usize)> | game-ai\src\plan_legacy\old\fight_model.rs:1052 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 51 | strategy | game_core::PlayerState::strategy | pub | fn(&game_core::PlayerState, &mut rand::rngs::std::StdRng, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + ) -> game_core::Strategy | game-core\src\simulation\state\player.rs:1576 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 52 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 53 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 54 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 55 | tower_dive_is_viable | game_ai::plan_legacy::old::tower_dive_is_viable | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan, &game_core::Entity, bool, &mut game_core::DebugFrameData) -> bool | game-ai\src\plan_legacy\old\fight_model.rs:935 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 56 | v21_should_defer_support_target | game_ai::plan_legacy::old::fight_model::v21_should_defer_support_target | in:game_ai | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, &game_core::Entity, usize, usize, usize, usize, usize) -> bool | game-ai\src\plan_legacy\old\fight_model.rs:1077 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 57 | v30_wave_danger_chase_guard | game_ai::plan_legacy::old::battle::v30_wave_danger_chase_guard | in:game_ai | fn(usize, &game_core::OperationData, &game_core::Entity, &game_core::Entity, u64, usize, bool, usize, usize, bool, bool, bool, &mut u8, &mut u8, &mut bool) -> std::option::Option<game_ai::plan_legacy::old::BattleSubPlanGoal> | game-ai\src\plan_legacy\old\battle.rs:2079 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 58 | v3_beyond_enemy_line | game_ai::plan_legacy::old::BattlePlan::v3_beyond_enemy_line | in:game_ai::plan_legacy::old::battle | fn(&game_ai::plan_legacy::old::BattlePlan, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity) -> bool | game-ai\src\plan_legacy\old\battle.rs:387 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 59 | v3_survival_incoming | game_ai::v3_survival_incoming | pub | fn(&game_core::PlayerState, &game_core::OperationData, &game_core::Entity) -> usize | game-ai\src\tower_discipline.rs:367 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 60 | with_runaway | game_ai::plan_legacy::old::BattlePlan::with_runaway | pub | fn(&game_ai::plan_legacy::old::BattlePlan, usize, &game_core::OperationData) -> bool | game-ai\src\plan_legacy\old\battle.rs:367 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
</details>

⚠**미매칭 19개**: `bumpalo`, `clamp`, `collect`, `drop  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `entry`, `extend`, `format_inner`, `from_iter`, `grow_one`, `insert_no_grow`, `is_none_or`, `max_by`, `or_insert`, `range  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `reserve_internal_or_panic`, `rustc_entry`, `then`, `unwrap_or_else`, `well_anchor`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m10.ll:26665) · **형제 20개** (BattlePlan)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::plan_legacy::old::BattlePlan as std::clone::Clone>::clone | pub | game-ai\src\plan_legacy\old\battle.rs:96 | True | fn(&game_ai::plan_legacy::old::BattlePlan) -> game_ai::plan_legacy::old::BattlePlan |
| 1 | <game_ai::plan_legacy::old::BattlePlan as std::fmt::Debug>::fmt | pub | game-ai\src\plan_legacy\old\battle.rs:96 | True | fn(&game_ai::plan_legacy::old::BattlePlan, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 2 | game_ai::plan_legacy::old::BattlePlan::sub_goal | pub | game-ai\src\plan_legacy\old\battle.rs:178 | True | fn(&game_ai::plan_legacy::old::BattlePlan) -> game_ai::plan_legacy::old::BattleSubPlanGoal |
| 3 | game_ai::plan_legacy::old::BattlePlan::ff_birth_src | in:game_ai::plan_legacy::old::battle | game-ai\src\plan_legacy\old\battle.rs:184 | False | fn(&game_ai::plan_legacy::old::BattleSubPlanGoal) -> u8 |
| 4 | game_ai::plan_legacy::old::BattlePlan::in_active_fight | pub | game-ai\src\plan_legacy\old\battle.rs:190 | True | fn(&game_ai::plan_legacy::old::BattlePlan) -> bool |
| 5 | game_ai::plan_legacy::old::BattlePlan::new | pub | game-ai\src\plan_legacy\old\battle.rs:196 | False | fn(usize, game_ai::plan_legacy::old::BattlePlanGoal, &game_core::OperationData, &game_core::PlayerState) -> game_ai::plan_legacy::old::BattlePlan |
| 6 | game_ai::plan_legacy::old::BattlePlan::new_dive | pub | game-ai\src\plan_legacy\old\battle.rs:246 | False | fn(usize, game_ai::plan_legacy::old::BattlePlanGoal, &game_core::OperationData, &game_core::PlayerState) -> game_ai::plan_legacy::old::BattlePlan |
| 7 | game_ai::plan_legacy::old::BattlePlan::new_region | pub | game-ai\src\plan_legacy\old\battle.rs:296 | False | fn(usize, game_ai::plan_legacy::old::BattlePlanGoal, &game_core::OperationData, &game_core::PlayerState, u64, u64, u64) -> game_ai::plan_legacy::old::BattlePlan |
| 8 | game_ai::plan_legacy::old::BattlePlan::goal | pub | game-ai\src\plan_legacy\old\battle.rs:341 | False | fn(&game_ai::plan_legacy::old::BattlePlan) -> game_core::BigGoal |
| 9 | game_ai::plan_legacy::old::BattlePlan::set_main_objective | pub | game-ai\src\plan_legacy\old\battle.rs:349 | True | fn(&mut game_ai::plan_legacy::old::BattlePlan, std::option::Option<game_ai::plan_legacy::team_plan::MainObjective>) |
| 10 | game_ai::plan_legacy::old::BattlePlan::return_to_objective | in:game_ai::plan_legacy::old::battle | game-ai\src\plan_legacy\old\battle.rs:354 | False | fn(&game_ai::plan_legacy::old::BattlePlan) -> bool |
| 11 | game_ai::plan_legacy::old::BattlePlan::current_focus | pub | game-ai\src\plan_legacy\old\battle.rs:358 | True | fn(&game_ai::plan_legacy::old::BattlePlan) -> std::option::Option<usize> |
| 12 | game_ai::plan_legacy::old::BattlePlan::with_runaway | pub | game-ai\src\plan_legacy\old\battle.rs:367 | False | fn(&game_ai::plan_legacy::old::BattlePlan, usize, &game_core::OperationData) -> bool |
| 13 | game_ai::plan_legacy::old::BattlePlan::v3_beyond_enemy_line | in:game_ai::plan_legacy::old::battle | game-ai\src\plan_legacy\old\battle.rs:387 | False | fn(&game_ai::plan_legacy::old::BattlePlan, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity) -> bool |
| 14 | game_ai::plan_legacy::old::BattlePlan::no_enemy_idle_watchdog | in:game_ai::plan_legacy::old::battle | game-ai\src\plan_legacy\old\battle.rs:406 | False | fn(&mut game_ai::plan_legacy::old::BattlePlan, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity) -> bool |
| 15 | game_ai::plan_legacy::old::BattlePlan::update | pub | game-ai\src\plan_legacy\old\battle.rs:427 | False | fn(&mut game_ai::plan_legacy::old::BattlePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) |
| 16 | game_ai::plan_legacy::old::BattlePlan::update_v32 | in:game_ai::plan_legacy::old::battle | game-ai\src\plan_legacy\old\battle.rs:748 | False | fn(&mut game_ai::plan_legacy::old::BattlePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) |
| 17 | game_ai::plan_legacy::old::BattlePlan::next_plan | pub | game-ai\src\plan_legacy\old\battle.rs:2034 | True | fn(&game_ai::plan_legacy::old::BattlePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &mut game_core::DebugFrameData) -> std::option::Option<game_ai::plan_legacy::types::BigPlan> |
| 18 | game_ai::plan_legacy::old::BattlePlan::is_end | pub | game-ai\src\plan_legacy\old\battle.rs:2039 | False | fn(&game_ai::plan_legacy::old::BattlePlan) -> bool |
| 19 | game_ai::plan_legacy::old::BattlePlan::sub_plan | pub | game-ai\src\plan_legacy\old\battle.rs:2043 | False | fn(&game_ai::plan_legacy::old::BattlePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> game_ai::plan_legacy::sub_plan::SubPlan |

**`open` 9건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | %5 team_plan 에 noalias/readonly/dereferenceable 이 전부 없는 원인 미확인(추정: 호출측/LTO 속성 소실 또는 UnsafeCell). 본문 store 0건이라 & 로 판정 | 5 |  |
| 1 | 미탐색 | wave_goal(Option<BattleSubPlanGoal>) None 태그가 IR 에서 -1 (v30_wave_danger_chase_guard ret phi 실측). tcxdict 니치 예상값(8)과 다름 — 반환 규약만 관측, 이유 미조사 | 3 |  |
| 2 | 미탐색 | resolve_fight_full 의 마지막 인자 baseline(ptr) 정체 미확인(sG_0 캡처 %1+48 = &committed_dir? 아님, 별도 ptr) — 명세 대상 밖 | 4 |  |
| 3 | 미탐색 | 1987 SkillBurst 분기: 이 함수 안에서 tactic=3 을 쓰는 곳이 없어 도달 불가로 보이나, 다른 함수가 self.tactic 을 SkillBurst 로 두고 진입하는지는 이 범위 밖(미탐색). 1641 에서 매 호출 Standard 로 리셋되므로 사실상 사장(이 함수 범위 한정 판정) | 4 |  |
| 4 | 미탐색 | sL_0 unwrap_or 기본값(attack_effect None 시 my_attack_dps) 미확인 — 본문 %3104 블록의 as_ref None 경로 값 안 읽음(SkillBurst 사장 분기라 생략) | 4 |  |
| 5 | 미탐색 | exit_src=4 경로의 exit_sub 6/7 의미: _docs 'exit_sub는 exit_src=4일 때만 유효' 만 확인, 코드표(FF_BAIL_NAMES)는 시뮬레이터 측이라 미조회 | 4 |  |
| 6 | 미탐색 | PlayerState::strategy(rnd, game) 반환 24B 구조체는 tcxdict Strategy(24B) 로 매칭했으나 반환 타입 DWARF 를 직접 확인하진 않음(+0xf object_finish 오프셋 일치로 판정) | 3 |  |
| 7 | 미탐색 | dive_entry_soaker(fight_model.rs:1073) 의 max_by 비교 순서: IR (hp, id) 튜플 lexicographic 로 읽음(stat_cached.hp 같으면 id 큰 쪽) — 소스 튜플 순서는 IR 기준 | 4 |  |
| 8 | 미탐색 | exe 0xdf36e0 과의 대조는 수행하지 않음(Ghidra 금지·주소는 메인 확정). 어긋남 발견 없음 = 검증 안 함 | 4 |  |

**`notes` 1건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | 1085~1094 소스 문장 구조: IR 은 (rescue ally None \|\| helping) → p.line=p.line_absolute / (ally Some && !helping) → ff_rescue_tick 기록. dbg 'helping'=%1156 은 dist>range(멀다) 인데 None 경로도 line_absolute 로 가서 단순 if/else 로는 안 맞음 — 줄 1091~1093(22/36/17자) 은 레지스터 전용이라 원문 불명. 동작(IR)은 logic 대로 확정 | 4 | 사실 서술 |

<details><summary>`closed` 1건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

| # | 물음 | 닫은 근거 |
|---|---|---|
| 0 | 1232~1235 v50_dive_episode 소스 표기: IR 등가식 with_dive && DT.is_some() && E && (M \|\| !F) 로 기재. 줄 단위 문장 분할 미확정(column 없음) | 본문에 해소 표기가 있다 |
</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

