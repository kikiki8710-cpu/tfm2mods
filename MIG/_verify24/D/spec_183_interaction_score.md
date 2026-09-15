---

### `183` interaction_score — SmallActionPlay 하나의 상호작용(전투) 점수 i64 — 행동 종류별(RunAway/Around/Trace/Attack/Skill/Skill2/Ult) 피해·위험·탑·근접수 차이로 계산, 조기 거부는 -99999/-9999999

| 항목 | 값 |
|---|---|
| id | `action_score__interaction_score` |
| 심볼 | `_RNvNtCshdEBA0ozCnw_7game_ai12action_score17interaction_score` |
| 소스 | `game-ai\src\action_score.rs:616` |
| IR | `m05.ll` 34861~37632행 |
| 경로·가시성 | `game_ai::interaction_score` · **pub** |
| 계층 | 점수화·술어 |
| exe | `d57540` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r15` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64
```

<details><summary>인자 7개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | version | usize | AI 버전. `version > 1` 게이트 3곳(L632·L660·L743 — reach version=2 접기로 전부 true). 콜리(position_eval_at·evaluation_position·check_kill_die_tick·calculate_interaction_action_score·get_battle_role)에 전달 | 4 |
| 1 | 2 | rnd | &mut StdRng(320B) | 이 함수는 직접 읽고 쓰지 않음 — check_kill_die_tick(35954)·calculate_interaction_action_score(4곳)에만 전달. writes 는 그 콜리 소관 | 4 |
| 2 | 3 | player | &PlayerState(2528B, readonly) | +0x930 info.team · +0x928 info.id(ictx pid) · +0x9c0 info.position(champ 슬롯 인덱스, i32) | 4 |
| 3 | 4 | data | &OperationData(24B, readonly) | +0x0 cache(&AbstractGameWithCache) · +0x8 context(&GameContext) · +0x10 blackboard(&[Blackboard;2]) | 4 |
| 4 | 5 | parameter | &ScoreParameter(5384B, readonly) | +0x918 player(ChampionScoreParameter 216B) · +0x14d8/+0x14f0 near_enemies Vec · +0x1500 v3_turnback_hold | 4 |
| 5 | 6 | action | &SmallActionPlay(184B, readonly) | 니치 태그 +0xb1. get_action() 으로 game_core::SmallAction 으로 변환해 분기(L618) · Around/AroundHide +0x20 goal_gain · LaneMinionPosition +0x20 goal_score · Trace +0x94 dive_ignore_tower_escape | 4 |
| 6 | 7 | debug | &mut DebugFrameData(224B) | 이 함수는 직접 쓰지 않음 — check_kill_die_tick·calculate_interaction_action_score 에 전달만(35954·37422·37443·37469·37495) | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn interaction_score(version, rnd, player, data, parameter, action, debug) -> i64 {
  let game = data.cache.game;  // &dyn AbstractGame
  let team = player.info.team;  let enemy = 1 - team;
  // L617
  let champ = data.cache.player_champion[team][player.position()].unwrap();
  // L618 (get_action 인라인 · small_action.rs:308)
  let ty: SmallAction = action.get_action();   // RunAway/Recall/AroundRunAway→RunAway(0) · Around/AroundHide/LaneMinionPosition→Around(2,target) · Positioning(1) · AroundRegion/AroundPosition/AroundPositionBush/AroundBush→AroundPosition(3) · Trace→Trace(4,target) · Attack(6)/Skill(7)/Skill2(8)/Ult(9,target) · Stop(10)
  // L619 = interaction_ctx(player, data, champ) 인라인: INTER_CTX TLS (키 seed,tick,pid) → ictx: InterActionCtx
  let ictx = INTER_CTX.with(...);
  // L631~634
  let no_self_risk = if champ.stat_buff_cached.undying { true }
     else if version > 1 { if nexus_final_stand(player,data) { true } else if champ.hp*100 > max(champ.stat_cached.hp,1)*35 { base_defense_focus(player,data) } else { false } }
     else { false };
  // L636~638 (Option<id>.and_then(get_entity_by_id))
  let nearest_enemy_champion = ictx.nearest_vis_enemy → entity;  let near_ally_tower = ictx.near_ally_tower → entity;  let near_enemy_tower = ictx.near_enemy_tower → entity;
  let mut act_score = -99999;  let mut positioning_score = 0;   // L938 (phi 기본)
  match ty {
   RunAway => {                                                         // L647~731
     if champ.undying { return -99999; }
     if version > 1 { if nexus_final_stand { return -99999; }
                      let hp_ratio = champ.hp*100 / max(maxhp,1);  if hp_ratio > 35 && base_defense_focus { return -99999; } }   // L660~667
     let p = &parameter.player;  let applyed_damage = p.applyed_damage;   // L674
     let dest_score = action.evaluation_position(version,player,data).map(|(x,y)| position_eval_at(version,player,data,x,y,PositionEvalPurpose::RunAway));   // L678~679
     let (base_damage, tower_damage) = match dest_score {
        Some(s) => (max(p.risk_damage + max(maxhp,1)*max(s.risk,0)/-100, 0), max(p.risk_possible_tower + max(maxhp,1)*max(s.tower_risk,0)/-100, 0)),   // L681·685
        None => (p.risk_damage, p.risk_possible_tower) };                 // L682·686
     let tower_damage = if game.is_visible(enemy, champ.id) || near_enemy_tower.is_some_and(|t| t.attack_effect.unwrap().is_in_range(t, champ)) { tower_damage } else { 0 };   // L689 (closure#2)
     let possible_damage = tower_damage + p.possible_risk(data, 9999);    // L688
     let hp_value = champion_hp_value(data, parameter, p);                // L695
     let f = map.fountains[team];  if champ.x∈[f.0,f.2] && champ.y∈[f.1,f.3] { return -99999; }   // L697~700 자기 분수 안
     if applyed_damage < champ.hp {                                       // L704
        let base_score = hp_value*base_damage / champ.hp;                // L709 (hp==0 → panic)
        let possible_score = possible_damage*hp_value / champ.hp;         // L710
        let near_enemies = popcnt(ictx.vis150_mask);  let near_allies = ictx.ally100_count;  let diff = near_enemies - near_allies;   // L713~716
        let diff_coef = if diff > 1 {300} else { match diff { 1=>200, 0=>100, -1=>75, _=>40 } };   // L719~725
        act_score = base_score/4 - 2 + diff_coef*possible_score/800;      // L731
     } // else act_score = -99999
     // L947~954 positioning_score (RunAway 만)
     let near = ictx.vis150_nearest → entity;
     let role = get_battle_role(version, data.context, data.cache, player);
     if matches!(role, SkillCaster|Utility) && champ.skill_effect.is_some() && champ.ty==Champion && champ.skill_cooldown > 30
        && champ.skill2_effect_at_level().is_some() /*level>2 이면 skill2_effect 아니면 빈 Effect*/ && champ.skill2_cooldown >= 31
        && near.is_some_and(|e| dist²(champ,e) < 100000²+1) { positioning_score = 10; }
   }
   Around(target_id) => {                                                 // L892~925
     let Some(target) = game.get_entity_by_id(target_id) else { act_score=-99999; goto L965 };
     let p = &parameter.player;  let base_damage = p.risk_damage;  let hp_value = champion_hp_value(data,parameter,p);   // L893·895
     let base_score = hp_value*base_damage / champ.hp;                   // L898
     let mut tower_score = 0;                                             // L900
     if let Some(t) = near_ally_tower { if let Some(target) = game.get_entity_by_id(target_id) {   // L902~903
        if target.team != champ.team && t.attack_effect.unwrap().is_in_range(t, target) && game.tick() < setting.tower_attack_disable_tick { if let Some(e) = nearest_enemy_champion {   // L904
           tower_score = min(t.attack_effect.expected_damage_target(ctx, t, e)*100 / e.hp, 100); } } } }   // L907~909
     let bonus = match action { Around(a) => a.goal_gain*hp_value/200, LaneMinionPosition(l) => l.goal_score, _ => 0 };   // L915~921
     act_score = tower_score - base_score + bonus;                        // L925
   }
   Trace(target_id) => {                                                  // L735~865
     let Some(t) = game.get_entity_by_id(target_id) else { act_score=-99999; goto L965 };
     let dive_ignore = matches!(action, Trace(x) if x.dive_ignore_tower_escape);   // L739
     if version > 1 && t.ty==Champion && t.team != champ.team && game.get_game_mode()!=DeathMatch {   // L743~744
        if parameter.v3_turnback_hold { return -9999999; }              // L753
        if !(no_self_risk || dive_ignore) {                               // L756
           let reach = champ.attack_effect.map(|ef| line_effect_range_with_radii(ef, champ, t)).unwrap_or(0);   // L757~758
           let d = champ.distance(t);                                     // L759
           if d > reach {                                                 // L760
              let die_enemies = iter_champions(enemy).filter(|e| dist²(champ,e)<150000²+1 && blackboard[enemy].is_recent_visible(game,player,e)).collect_in(bump);   // L763~766 (closure#4)
              if !die_enemies.is_empty() {                                 // L767
                 let walk_tick = (d - reach) / max(champ.move_speed,1);    // L761
                 let my_die = check_kill_die_tick(version, rnd, data, player, champ, die_enemies, Vec::new_in(bump), debug);   // L768
                 if my_die <= walk_tick { return -9999999; } } } } }      // L770
     let pts = action.evaluation_position(version,player,data).map(|(x,y)| position_eval_at(version,player,data,x,y,PositionEvalPurpose::Trace));   // L778~779
     let p = &parameter.player;
     let base_damage = pts.map(|s| max(maxhp,1)*max(s.risk,0)/100).unwrap_or(p.risk_damage);   // L781~782 (⚠RunAway 와 달리 risk_damage 에 더하지 않고 대체)
     let tower_possible = if dive_ignore { 0 } else { pts.map(|s| max(maxhp,1)*max(s.tower_risk,0)/100).unwrap_or(p.risk_possible_tower) / 3 };   // L783~786
     let possible_damage = p.possible_risk(data,9999) + tower_possible;   // L788
     let hp_value = champion_hp_value(data,parameter,p);                  // L789
     let (base_score, possible_score) = if no_self_risk { (0,0) } else { (hp_value*base_damage/champ.hp, possible_damage*hp_value/champ.hp) };   // L793~797
     let mut tower_score = 0;
     if let Some(t2) = near_ally_tower { if let Some(target) = game.get_entity_by_id(target_id) {   // L801~802
        let ef = t2.attack_effect.unwrap();                               // L803
        let attack_range = (ef.range + t2.stat_buff.range + (t2.level-1)*ef.growth_range) + ef.range_adjust(t2,target) + t2.radius_adj + target.radius_adj;   // L804 (radius_adj = radius*(100+radius_mult)/100, mult==0 이면 radius)
        let attack_range = attack_range.saturating_sub(30000);            // L805
        if dist²(target,t2) <= attack_range² && game.tick() < tower_attack_disable_tick { if let Some(e)=nearest_enemy_champion {   // L806
           tower_score = min(ef.expected_damage_target(ctx,t2,e)*100 / e.hp, 100); } } } }   // L809~811
     if !dive_ignore { if let Some(t3) = near_enemy_tower { if let Some(target) = game.get_entity_by_id(target_id) {   // L819~821
        let ef = t3.attack_effect.unwrap();  let attack_range = (같은 식, t3·target).saturating_sub(30000);   // L822~824
        if dist²(target,t3) <= attack_range² && game.tick() < tower_attack_disable_tick {   // L825
           tower_score -= min(ef.expected_damage_target(ctx,t3,champ)*100 / champ.hp, 100); } } } }   // L829 (적 탑이 나를 때리는 기대피해)
     let mut enemy_possible_score = 0;                                    // L836
     if let Some(ep) = parameter.near_enemies.iter().find(|p| p.id == t.id) {
        let possible_enemy_damage = ep.possible_risk(data,9999) + ep.risk_damage;   // L837
        enemy_possible_score = possible_enemy_damage*hp_value / champ.hp;   // L838
        let ep_player = data.cache.player_by_champion_id(ep.id).unwrap();   // L839
        let in_recall = blackboard[enemy].in_recall(ep_player.position());   // L840
        let mr = max_range(champ, t);                                      // L841
        if in_recall && t.move_speed + 100 > champ.move_speed && dist²(champ,t) > mr² { return -99999; } }   // L842
     let near_enemies = count(enemy champs e: dist²(champ,e)<150000²+1 && blackboard[enemy].is_recent_visible(game,player,e));   // L847~848 (5슬롯 언롤)
     let near_allies = count(ally champs a: dist²(champ,a)<100000²+1);     // L850 (자기 자신 포함 · 가시성 검사 없음)
     let diff = near_enemies - near_allies;                                // L852
     let coef = if diff > 1 {200} else { match diff { 1=>150, 0=>80, -1=>60, _=>30 } };   // L853~859
     act_score = tower_score - (base_score + possible_score) + coef*enemy_possible_score/100;   // L865
   }
   Attack(id) => act_score = calculate_interaction_action_score(version,rnd,player,data,parameter, &champ.attack, champ.attack_effect.unwrap(), target, debug),   // L871~872
   Skill(id) => … (&champ.skill, champ.skill_effect.unwrap(), target),      // L878~879
   Skill2(id) => … (level>2 ? (&champ.skill2, skill2_effect.unwrap()) : (&champ.empty, 빈Effect.unwrap()→None 이면 panic), target),   // L885~886
   Ult(id) => … (level>4 ? (&champ.ult, ult_effect) : (&champ.empty, 빈Effect), target),   // L931~932 (대상 None 이면 act_score=-99999, L965 검사 없음)
   Positioning | AroundPosition | Dodge | Stop => return 0,                // L644 default → 0+0
  }
  // L965~966 (Around/Trace/Attack/Skill/Skill2 만): if game.get_entity_by_id(target_id).is_none() { return 0; }
  act_score + positioning_score                                            // L977~978
}
```

**`mem` 메모리 접근 67건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | 34901 · 팀 인덱스(<2 bounds), 적팀 = 1-team | 4 | OK |  |
| 1 | PlayerState | 0x9c0 | info.position(tag i32) | r | 34914 · player_champion 슬롯 인덱스(PlayerState::position L581 인라인) | 4 | OK |  |
| 2 | PlayerState | 0x928 | info.id | r | 35112 · INTER_CTX 키 pid | 4 | OK |  |
| 3 | OperationData | 0x0 | cache | r | 34917 | 4 | OK |  |
| 4 | OperationData | 0x8 | context(&GameContext) | r | 35469·35703·35924·36279·36495 | 4 | OK |  |
| 5 | OperationData | 0x10 | blackboard(&[Blackboard;2]) | r | 35089 · ictx 캡처·L840 in_recall·L848 is_recent_visible ([1-team] 인덱스) | 4 | OK |  |
| 6 | AbstractGameWithCache | 0x0 | game.data_ptr | r | 35099 | 4 | OK |  |
| 7 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r | 35100 · 슬롯 +0x20 seed · +0x28 tick · +0x40 get_game_mode · +0xf8 is_visible · +0x1f0 get_entity_by_id (divtable) | 3 | OK |  |
| 8 | AbstractGameWithCache | 0x1e0 | player_champion[team][pos] (Option<&Entity> [2][5]) | r | 34921~34924 · unwrap(null→unwrap_failed) | 4 | OK |  |
| 9 | GameContext | 0x8 | setting(&GameSetting) | r | 35705 · →+0x13f8 tower_attack_disable_tick | 4 | OK |  |
| 10 | GameSetting | 0x13f8 | tower_attack_disable_tick | r | 35707 · tick >= 이면 탑 피해 계산 생략(L904·L806·L825) | 4 | OK |  |
| 11 | GameContext | 0x20 | map(&MapDef) | r | 35471 | 4 | OK |  |
| 12 | GameContext | 0x0 | pool(&Bump) | r | 35925 · die_enemies Vec 할당자 | 4 | OK |  |
| 13 | MapDef | 0x6d70 | fountains[team] (x0,y0,x1,y1) stride 32 | r | 35474~35501 · L700 자기 분수 사각형 | 4 | OK |  |
| 14 | ScoreParameter | 0x918 | player (ChampionScoreParameter) | r | 35315 · possible_risk/champion_hp_value 의 self | 4 | OK |  |
| 15 | ScoreParameter | 0x988 | player.applyed_damage | r | 35317 · L704 | 4 | OK |  |
| 16 | ScoreParameter | 0x998 | player.risk_damage | r | 35392·35427·35605·36014 · base_damage 기본값 | 4 | OK |  |
| 17 | ScoreParameter | 0x9b0 | player.risk_possible_tower | r | 35410·35430·36068 · tower_damage 기본값 | 4 | OK |  |
| 18 | ScoreParameter | 0x14d8 | near_enemies.ptr | r | 36321 · Vec<ChampionScoreParameter>(216B stride) | 4 | OK |  |
| 19 | ScoreParameter | 0x14f0 | near_enemies.len | r | 36324 | 4 | OK |  |
| 20 | ScoreParameter | 0x1500 | v3_turnback_hold | r | 35846 · true 면 Trace 는 -9999999 | 4 | OK |  |
| 21 | ChampionScoreParameter(near_enemies[i]) | 0x58 | id | r | 36358 · find 키 == 대상 id | 4 | OK |  |
| 22 | ChampionScoreParameter(near_enemies[i]) | 0x80 | risk_damage | r | 36533 · possible_enemy_damage 가산 | 4 | OK |  |
| 23 | SmallActionPlay | 0xb1 | 니치 태그 | r | 34941 · get_action 분기 + L739(==14 Trace) + L915(==5 Around / ==13 LaneMinionPosition) | 4 | OK |  |
| 24 | SmallActionPlay | 0x8 | variant 페이로드 target id | r | Around/AroundHide/Positioning/AroundPositionBush/LaneMinionPosition/Attack/Skill/Skill2/Ult 의 +0x8 · AroundRegion +0x10 · AroundBush +0x18 · AroundPosition +0x30 · Trace +0x60 (34974~35076) | 4 | 오귀속(사전은 다른 필드를 준다) |  |
| 25 | SmallActionAround / AroundHide | 0x20 | goal_gain | r | 35741 · L917 bonus | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 26 | SmallActionLaneMinionPosition | 0x20 | goal_score | r | 35750 · L920 bonus | 4 | OK |  |
| 27 | SmallActionTrace | 0x94 | dive_ignore_tower_escape | r | 35767 · L739 | 4 | OK |  |
| 28 | Entity | 0x0 | team(TeamType tag) | r | 35659·35811 · Player(0)/Neutral(1) | 4 | OK |  |
| 29 | Entity | 0x8 | team@Player.0 | r | 35685·35840 | 4 | OK |  |
| 30 | Entity | 0x68 | ty tag | r | 35802·37544 · ==13 Champion | 4 | OK |  |
| 31 | Entity | 0xb8 | ty@Champion.skill_cooldown | r | 37551 · L953 > 30 | 4 | OK |  |
| 32 | Entity | 0xc0 | ty@Champion.skill2_cooldown | r | 37570 · L954 >= 31 | 4 | OK |  |
| 33 | Entity | 0x438 | stat_buff_cached.range | r | 36180·36396 · 탑 사거리 합산 | 4 | OK |  |
| 34 | Entity | 0x470 | stat_buff_cached.radius_mult(i32) | r | 36183·36211·36399·36427 | 4 | OK |  |
| 35 | Entity | 0x488 | stat_buff_cached.undying | r | 35137 · true 면 no_self_risk 및 RunAway -99999 | 4 | OK |  |
| 36 | Entity | 0x490 | attack_effect@Some (Effect) | r | 35671·35862·36161·36377·37419 | 4 | OK |  |
| 37 | Entity | 0x4a0 | attack_effect.range | r | 36172·36388 | 4 | OK |  |
| 38 | Entity | 0x4a8 | attack_effect.growth_range | r | 36174·36390 | 4 | OK |  |
| 39 | Entity | 0x4c0 | attack_effect 니치 태그(i32, -1=None) | r | 35673·35857·36163·36379·37414 | 4 | OK |  |
| 40 | Entity | 0x4c8 | skill_effect@Some | r | 37440 | 4 | OK |  |
| 41 | Entity | 0x4f8 | skill_effect 태그(-1=None) | r | 37435·37538 | 4 | OK |  |
| 42 | Entity | 0x500 | skill2_effect@Some (+0x30 태그) | r | 37457~37461·37559~37563 · level>2 일 때만, 아니면 @anon.121 빈 Effect | 4 | OK |  |
| 43 | Entity | 0x538 | ult_effect@Some (+0x30 태그) | r | 37483~37487 · level>4 일 때만 | 4 | OK |  |
| 44 | Entity | 0x570 | attack(Box<dyn Action>) | r | 37420 · Attack 분기 인자 | 4 | OK |  |
| 45 | Entity | 0x580 | skill(Box<dyn Action>) | r | 37441 | 4 | OK |  |
| 46 | Entity | 0x590 | skill2(Box<dyn Action>) | r | 37466 (level>2) | 4 | OK |  |
| 47 | Entity | 0x5a0 | ult(Box<dyn Action>) | r | 37492 (level>4) | 4 | OK |  |
| 48 | Entity | 0x5b0 | empty(Box<dyn Action>) | r | 37466·37492 · 레벨 미달 시 대체 | 4 | OK |  |
| 49 | Entity | 0x5c0 | id | r | 35440·36339·36656 | 4 | OK |  |
| 50 | Entity | 0x5c8 | level | r | 36176·36392 (growth) · 37455·37481·37557 (>2 / >4) | 4 | OK |  |
| 51 | Entity | 0x628 | stat_cached.hp(최대 HP) | r | 35158·35343·35397 | 4 | OK |  |
| 52 | Entity | 0x640 | stat_cached.move_speed | r | 35887·37339·37342 | 4 | OK |  |
| 53 | Entity | 0x660 | x | r | 35484 외 다수 | 4 | OK |  |
| 54 | Entity | 0x668 | y | r | 35504 외 다수 | 4 | OK |  |
| 55 | Entity | 0x670 | hp | r | 35155·35492 외 — 나눗셈 분모(0 이면 panic) | 4 | OK |  |
| 56 | Entity | 0x680 | radius | r | 36195·36202 외 · 사거리 합산 | 4 | OK |  |
| 57 | InterActionCtx(ictx, 스택 %20) | 0x0 | nearest_vis_enemy | r | 35174~35186 | 4 | OK |  |
| 58 | InterActionCtx | 0x10 | near_ally_tower | r | 35199~35210 | 4 | OK |  |
| 59 | InterActionCtx | 0x20 | near_enemy_tower | r | 35223~35234 | 4 | OK |  |
| 60 | InterActionCtx | 0x30 | vis150_nearest | r | 35593~35594·37513~37514 | 4 | OK |  |
| 61 | InterActionCtx | 0x40 | vis150_mask | r | 35541 · popcnt | 4 | OK |  |
| 62 | InterActionCtx | 0x41 | ally100_count | r | 35546 | 4 | OK |  |
| 63 | TLS INTER_CTX RefCell | 0x0 | borrow | w | aux with 인스턴스 91294·92567·92711 | 4 | 확인불가(tcx 사전에 타입 없음) | -1 → 0(+1 복원) |
| 64 | TLS INTER_CTX | 0x8 | (seed, tick, pid, InterActionCtx) 98B | w | aux 92526~92550 (L611) — 단일 엔트리, 무효화 없음(키 불일치 = 재계산) | 4 | 확인불가(tcx 사전에 타입 없음) | miss 시 새 키+계산값 통째 덮어씀 |
| 65 | bumpalo Bump(GameContext.pool) | - | die_enemies Vec<&Entity> 할당 | w | arena 할당이라 힙 상태 변화는 Bump 커서뿐 | 4 | 확인불가(오프셋 파싱 실패) | L766 collect · 비면 drop_glue(35938) · 아니면 check_kill_die_tick 에 move |
| 66 | (참고) rnd / debug | - | 직접 쓰기 없음 | w | check_kill_die_tick·calculate_interaction_action_score 에 &mut 전달만 — 그 콜리 명세의 writes 참조 | 4 | 확인불가(오프셋 파싱 실패) | - |

**`consts` 상수 35건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 2 | 617 | 임계 | team 인덱스 bounds(<2) (34905) | 4 |
| 1 | 10 | 309 | 센티널 | SmallActionPlay 니치 무효값 assume(ne 10) — get_action 인라인 (34943) | 4 |
| 2 | -3 | 309 | 태그 | SmallActionPlay 태그→논리 idx (`tag-3`) (34945) | 4 |
| 3 | 7 | 309 | 태그 | untagged variant AroundPosition 의 논리 idx (34947) — 또 L779 position_eval 목적 Trace 태그(35983) | 4 |
| 4 | 100 | 633 | 인덱스 | hp*100 (퍼센트) — 633·664·681(-100 sdiv)·685·804 radius(100+mult)/100·909/811/829 dmg*100·865 coef/100 등 | 4 |
| 5 | 35 | 633 | 임계 | HP% 임계 — hp*100 > maxhp*35 이면 base_defense_focus 판정 (35162·35351) | 4 |
| 6 | 1 | 633 | 임계 | max(maxhp,1)·max(speed,1) 0 나눗셈 가드 · diff>1 · diff==1 분기 — aux 92643 `shl i8 1, slot` 은 vis150_mask 의 1<<slot 비트(배수 접힘 아님) | 4 |
| 7 | 3 | 679 | 태그 | PositionEvalPurpose 태그 3 = RunAway (35372) — 또 L784 tower_possible/3 (36076) · L952 BattleRole-3 | 4 |
| 8 | -100 | 681 | 계수 | base_damage = risk_damage + maxhp*max(risk,0) / -100 (= 빼기) (35402·35417) | 4 |
| 9 | 0 | 681 | 임계 | smax(.,0) 하한 · 반환 0 · tower_damage 0 등 | 4 |
| 10 | 9999 | 688 | 임계 | ChampionScoreParameter::possible_risk 의 3번째 인자(틱 상한 추정 — 콜리 계약) (35438·36085·36531) | 5 |
| 11 | -99999 | 704 | 산출값 | act_score 기본/거부값 (35590·37505·37617·37629) | 4 |
| 12 | 4 | 731 | 임계 | base_score/4 (35581) — 또 L932 level>4 (37482) | 4 |
| 13 | -2 | 731 | 미상 | act_score 상수항 -2 (35584) | 4 |
| 14 | 800 | 731 | 계수 | diff_coef*possible_score/800 (35583) | 4 |
| 15 | 300 | 719 | 산출값 | RunAway diff_coef: near_enemies-near_allies > 1 (35579) | 4 |
| 16 | 200 | 721 | 계수 | RunAway diff_coef: diff == 1 (35579) · Around bonus = goal_gain*hp_value/200 (35743) · Trace coef diff>1 (37400) | 4 |
| 17 | 75 | 725 | 산출값 | RunAway diff_coef: diff == -1 | 4 |
| 18 | 40 | 725 | 산출값 | RunAway diff_coef: diff <= -2 | 4 |
| 19 | -1 | 721 | 센티널 | diff == -1 분기 · Option<Effect> 니치 None(-1 i32) · level-1 | 4 |
| 20 | 5 | 907 | 태그 | action 태그 5 = Around (35646) — Around 분기 bonus | 4 |
| 21 | 13 | 907 | 태그 | action 태그 13 = LaneMinionPosition (35647) · EntityType 태그 13 = Champion (35803·37545) | 4 |
| 22 | 14 | 739 | 태그 | SmallActionPlay 태그 14 = Trace (35765) | 4 |
| 23 | 22500000001 | 764 | 임계 | 150000²+1 — 적 챔프 근접 판정(dist² <) (aux 61821 · 본체 36659 L848) | 4 |
| 24 | 10000000001 | 850 | 임계 | 100000²+1 — 아군 근접(L850) · positioning_score 근접(L954) (37047·37611) | 4 |
| 25 | 30000 | 805 | 계수 | 탑 사거리 여유 — attack_range.saturating_sub(30000) (36240·36456) | 4 |
| 26 | -9999999 | 753 | 산출값 | 강한 거부값: v3_turnback_hold / 다이브 사망 (37629) | 4 |
| 27 | 150 | 855 | 산출값 | Trace coef: diff == 1 (37400) | 4 |
| 28 | 80 | 857 | 산출값 | Trace coef: diff == 0 | 4 |
| 29 | 60 | 859 | 산출값 | Trace coef: diff == -1 | 4 |
| 30 | 30 | 859 | 임계 | Trace coef: diff <= -2 (37400) · L953 skill_cooldown > 30 (37552) | 4 |
| 31 | 31 | 954 | 임계 | skill2_cooldown < 31 이면 positioning_score 0 (37571) | 4 |
| 32 | 10 | 954 | 태그 | positioning_score 가산값 (37612) | 4 |
| 33 | 2 | 744 | 센티널 | GameMode 태그 2 = DeathMatch 이면 다이브 검사 생략 (35829) · L886 level>2 · L952 BattleRole-3 <u 2 (SkillCaster/Utility) · L680 Option 니치 2 | 4 |
| 34 | 5 | 763 | 태그 | 팀당 챔프 슬롯 5 (iter 상한) | 4 |

**`knobs` 조정점 10건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | RunAway 근접수 차 계수 | action_score.rs:719~725 | 300/200/100/75/40 | 올리면 적이 많을 때 가능피해(possible_score) 항이 커져 후퇴 점수가 오른다(도망 선호) | 4 | 기존 |
| 1 | RunAway 점수식 분모 | action_score.rs:731 | 4 · 800 · -2 | base_score/4 를 키우면 현재 위험이, /800 을 줄이면 가능 위험이 더 반영 | 4 | 기존 |
| 2 | Trace 근접수 차 계수 | action_score.rs:853~859 | 200/150/80/60/30 | 올리면 상대가 받을 피해 기대(enemy_possible_score)가 커져 추격 점수가 오른다 | 4 | 기존 |
| 3 | 탑 사거리 여유 | action_score.rs:805·824 | 30000 | 줄이면 탑 사거리 끝(약 1셀 안쪽)까지 탑 피해를 계산 → 탑 근처 추격이 더 보수적/공격적 | 4 | 기존 |
| 4 | 적 근접 반경 | action_score.rs:764·848 | 22500000001 | 150000(4.7셀). 늘리면 die_enemies·near_enemies 에 더 먼 적이 들어와 다이브 거부·계수 하락이 잦아진다 | 4 | 기존 |
| 5 | 아군 근접 반경 | action_score.rs:850·954 | 10000000001 | 100000(3.1셀). 늘리면 near_allies 가 늘어 diff 가 작아진다 | 4 | 기존 |
| 6 | HP% 임계(base_defense_focus 게이트) | action_score.rs:633·667 | 35 | 낮추면 넥서스 방어 집중 판정이 더 낮은 체력에서만 발동 | 4 | 기존 |
| 7 | Trace tower_possible 분모 | action_score.rs:784 | 3 | 키우면 추격 시 탑 위험을 덜 본다 | 4 | 기존 |
| 8 | positioning_score 스킬 쿨 임계 | action_score.rs:953~954 | 30 / 31 / +10 | SkillCaster·Utility 가 스킬 둘 다 쿨 중이고 적이 3셀 내면 후퇴 +10 | 4 | 기존 |
| 9 | Around goal_gain 분모 | action_score.rs:917 | 200 | 줄이면 Around 의 목표 이득 보너스 증가 | 4 | 기존 |

<details><summary>`callees` 피호출자 41건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | base_defense_focus | game_ai::plan_legacy::old::base_defense_focus | pub | fn(&game_core::PlayerState, &game_core::OperationData) -> bool | game-ai\src\plan_legacy\old\defense_nexus.rs:200 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | calculate_interaction_action_score | game_ai::action_score::calculate_interaction_action_score | in:game_ai | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &std::boxed::Box<dyn [Binder { value: Trait(game_core::Action), bound_vars: [] }] + 'static, std::alloc::Global>, &game_core::Effect, &game_core::Entity, &mut game_core::DebugFrameData) -> i64 | game-ai\src\action_score.rs:980 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 2 | champion_hp_value | game_ai::champion_hp_value | pub | fn(&game_core::OperationData, &game_ai::ScoreParameter, &game_ai::ChampionScoreParameter) -> i64 | game-ai\src\utils.rs:909 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 3 | check_kill_die_tick | game_ai::check_kill_die_tick | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::OperationData, &game_core::PlayerState, &game_core::Entity, bumpalo::collections::vec::Vec< &game_core::Entity>, bumpalo::collections::vec::Vec< &game_core::Entity>, &mut game_core::DebugFrameData) -> usize | game-ai\src\fight_check.rs:917 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | distance | game_core::Entity::distance | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2161 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 5 | evaluation_position | game_ai::SmallActionPlay::evaluation_position | pub | fn(&game_ai::SmallActionPlay, usize, &game_core::PlayerState, &game_core::OperationData) -> std::option::Option<(u64, u64)> | game-ai\src\small_action.rs:245 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 6 | expected_damage_target | game_core::Effect::expected_damage_target | pub | fn(&game_core::Effect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + , &game_core::Entity) -> usize | game-core\src\simulation\effect.rs:91 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 7 | get_action | game_ai::SmallActionUlt::get_action | in:game_ai | fn(&game_ai::SmallActionUlt) -> game_core::SmallAction | game-ai\src\small_action\cast.rs:286 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 16개 중 상위 3개 |
| 8 | get_action | game_ai::SmallActionPlay::get_action | pub | fn(&game_ai::SmallActionPlay) -> game_core::SmallAction | game-ai\src\small_action.rs:308 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 16개 중 상위 3개 |
| 9 | get_action | game_ai::SmallActionTrace::get_action | in:game_ai | fn(&game_ai::SmallActionTrace) -> game_core::SmallAction | game-ai\src\small_action\trace.rs:403 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 16개 중 상위 3개 |
| 10 | get_battle_role | game_ai::get_battle_role | pub | fn(usize, &game_core::GameContext, &game_core::AbstractGameWithCache, &game_core::PlayerState) -> game_ai::BattleRole | game-ai\src\utils.rs:446 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 11 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 12 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 13 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 14 | get_game_mode | game_core::AbstractGame::get_game_mode | pub | fn(&Self/#0) -> game_core::GameMode | game-core\src\simulation.rs:78 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 15 | get_game_mode | <game_core::Game as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::Game) -> game_core::GameMode | game-core\src\simulation\game.rs:1707 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 16 | get_game_mode | <game_core::SingleLaneGame as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::SingleLaneGame) -> game_core::GameMode | game-core\src\simulation\game.rs:4248 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 17 | in_recall | game_core::Blackboard::in_recall | pub | fn(&game_core::Blackboard, usize) -> bool | game-core\src\simulation\game\blackboard.rs:175 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 18 | interaction_ctx | game_ai::action_score::interaction_ctx | in:game_ai::action_score | fn(&game_core::PlayerState, &game_core::OperationData, &game_core::Entity) -> game_ai::action_score::InterActionCtx | game-ai\src\action_score.rs:572 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 19 | interaction_score | game_ai::interaction_score | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 | game-ai\src\action_score.rs:616 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 20 | is_empty | game_core::PatchSetting::is_empty | pub | fn(&game_core::PatchSetting) -> bool | game-core\src\patch_engine.rs:853 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 21 | is_empty | game_core::ModAiRegistry::is_empty | pub | fn(&game_core::ModAiRegistry) -> bool | game-core\src\mod_ai.rs:15 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 22 | is_empty | game_core::PublicChampionEvidence::is_empty | pub | fn(&game_core::PublicChampionEvidence) -> bool | game-core\src\banpick\public_evidence.rs:96 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 23 | is_in_range | game_core::Effect::is_in_range | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\effect.rs:63 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 24 | is_recent_visible | game_core::Blackboard::is_recent_visible | pub | fn(&game_core::Blackboard, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\game\blackboard.rs:346 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 25 | is_visible | game_core::World::is_visible | pub | fn(&game_core::World, usize, usize) -> bool | game-core\src\simulation\game\data.rs:384 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 26 | is_visible | game_core::VisibleState::is_visible | pub | fn(&game_core::VisibleState) -> bool | game-core\src\simulation\game\data.rs:121 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 27 | is_visible | game_core::AbstractGame::is_visible | pub | fn(&Self/#0, usize, usize) -> bool | game-core\src\simulation.rs:125 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 28 | iter_champions | game_core::AbstractGameWithCache::<'a, 'b>::iter_champions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5759 ~ game_core[6a30]::simulation::{impl#5}::iter_champions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1904 | True | True | 3 | IR 호출 심볼 일치(토큰 경계) |
| 29 | line_effect_range_with_radii | game_ai::lane_economy::line_effect_range_with_radii | in:game_ai | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity) -> u64 | game-ai\src\lane_economy.rs:72 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 30 | max_range | game_ai::max_range | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-ai\src\plan_legacy\old\battle.rs:2234 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 31 | nexus_final_stand | game_ai::plan_legacy::old::nexus_final_stand | pub | fn(&game_core::PlayerState, &game_core::OperationData) -> bool | game-ai\src\plan_legacy\old\defense_nexus.rs:190 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 32 | player_by_champion_id | game_core::AbstractGameWithCache::<'a, 'b>::player_by_champion_id | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> std::option::Option<&'a/#0 game_core::PlayerState> | game-core\src\simulation.rs:1915 | True | True | 3 | IR 호출 심볼 일치(토큰 경계) |
| 33 | position | game_core::PlayerAiContext::<'a, 'b, 'r>::position | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> game_core::Position | game-core\src\mod_ai.rs:587 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 34 | position | game_core::transfer::TeamRosterPlanningContext::position | pub | fn(&game_core::transfer::TeamRosterPlanningContext, game_core::Position) -> std::option::Option<&game_core::transfer::PositionPlanningContext> | game-core\src\transfer\roster_context.rs:234 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 35 | position_eval_at | game_ai::position_eval_at | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, u64, u64, game_ai::PositionEvalPurpose) -> game_core::PositioningScore | game-ai\src\position_eval.rs:291 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 36 | possible_risk | game_ai::ChampionScoreParameter::<'_>::possible_risk | pub | fn(&game_ai::ChampionScoreParameter</#0>, &game_core::OperationData, usize) -> i64 | game-ai\src\score_parameter.rs:1407 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 37 | range_adjust | game_core::Effect::range_adjust | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:29 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 38 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 39 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 40 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
</details>

⚠**미매칭 6개**: `find  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `popcnt`, `positioning_score`, `skill2_effect_at_level`, `with  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 2개는 **전부 다른 함수**라 싣지 않는다`, `{closure#0}>`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 16곳** (m02.ll:10964, m02.ll:25355, m02.ll:37190, m02.ll:39943, m02.ll:40326, m02.ll:46781, m02.ll:48493, m02.ll:64636, m14.ll:18636, m14.ll:21023, m14.ll:29972, m14.ll:31567, m14.ll:49029, m15.ll:11539, m15.ll:22583, m15.ll:50160) · **형제 0개** 

**`open` 9건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | INTER_CTX miss 시 계산부(interaction_ctx L581~608, m00.ll 91340~92520 ≈1,180줄)의 정확한 판정(어떤 타워를 near 로 잡는지·nearest 의 거리 기준) — 계약(필드 의미)만 적음. 미탐색 | 4 |  |
| 1 | 미탐색 | interaction_ctx 를 인라인한 다른 작성자(m11.ll 31594·31649 의 closure 본체 2벌)가 어느 함수인지 — 담당 밖. TLS 미러 설계 시 그 호출자도 같은 키로 덮어쓸 수 있음(미탐색) | 4 |  |
| 2 | 미탐색 | blackboard[1-team] 인덱스의 의미(적 팀 칸이 왜 is_recent_visible 의 self 인지) — IR 대로 적음. Blackboard 계약은 미탐색 | 4 |  |
| 3 | 미탐색 | L697 fountain 접근 L235 헬퍼 이름(MapDef::fountain? · MapDef+0x6d70 fountains[team]) — tcxdict 로 필드는 확정, 함수명 미확인 | 3 |  |
| 4 | 미탐색 | L742·L1404·L1494·L1665·L1669·L1677·L1693·L1775·L1776·L1791 의 Entity 접근자 이름(attack_effect/skill2_effect_at_level 등) — 오프셋·조건(level>2, level>4)은 확정, 소스 함수명 추정 | 5 |  |
| 5 | 미탐색 | possible_risk 의 3번째 인자 9999 의 의미(틱 상한 추정) — 콜리 계약, 미탐색 | 5 |  |
| 6 | 미탐색 | Skill2/Ult 분기에서 레벨 미달 시 @anon.121 (빈 Effect 상수)의 +0x30 태그가 -1 이면 unwrap 패닉 경로 — 상수 내용 미확인(실제로 None 이라면 레벨 미달 Skill2/Ult 호출 = 패닉, 호출자가 걸러줄 것으로 추정) | 5 |  |
| 7 | 표기 불가 | L731 의 `-2` 가 소스에서 어느 항인지(base_score/4 - 2 인지 (…)-2 인지) — 표기 불가(줄 안 순서), 값은 확정 | 4 |  |
| 8 | 미탐색 | reach 접힘 4건(version=2 → L632/660/743 true · gamemode=0 → L744 진행)은 판정 상수로만 반영, DeathMatch 경로는 살아있는 코드지만 현행 게임모드에선 NA | 4 |  |

<details><summary>`closed` 1건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

| # | 물음 | 닫은 근거 |
|---|---|---|
| 0 | L965 재조회로 0 을 반환하는 이유(대상 소멸 시 -99999 대신 0) — 동작은 확정(37625~37629), 의도 미상 | 본문에 해소 표기가 있다 |
</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

