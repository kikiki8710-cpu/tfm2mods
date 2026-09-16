# r21 티어1 심층 — 웨이브 9 (4) · 0.5.8 명세 logic 전문 + exe 힌트

요구: 0.5.8 명세(아래 logic) 를 기준선으로 0.6.0 디컴을 **줄 단위로 대조**해 「어느 줄이 어떻게 바뀌었나 / 새 분기·조건·상수·필드·콜리」 를 명세 형식(줄번호 참조 · 등가 Rust)으로 낸다. version≥3 분기는 「v3 경로」 로 별도 표기(값 미확정 · v2/v3 둘 다 적는다). 대형 함수는 먼저 콜리 지도·블록 지도(0.5.8 ↔ 0.6.0 매핑)를 만들고 신규 구역을 특정한 뒤 신규 구역만 정독한다.

## `d65620` → `1011d20` serpen_passive_plan  (3528→3833B · Δ+305)
- 힌트: serpen_passive_plan(이관 · 배치 A §9 골격 · CONTEST 블록 헬퍼 ef5f20/ef57f0/ef6da0/efb690/e89b70/10167f0 정체 미확정)
- exe 정렬: 명령 728→797 · 정렬 262 · 잔여 구조 7 · 분기 10 · 콜리 주의: 12a07d0→1016510 콜리 변경?(J0.00·+342B) ; 12a07d0→12920 미지(-341B) ; 12a07d0→196520 미지(-216B) ; 1323a00→1b3950 미지(+310B) ; 1323a00→3821af0 미지(-86B) ; 1323a00→e89b70 미지(+347B) ; 1323a00→ef57f0 미지(+449B) ; 31a37c0→16a7af0 미지(+469B)
- 0.5.8 명세 #96 `serpen__serpen_passive_plan` src game-ai\src\plan_legacy\old\serpen.rs:416 · one_line: 세르펜 목표(phase)에 맞는 개인 BigPlan 을 고른다 — Hunt→SerpenHuntAndPoke, Setup→전략(object_buildup)·적 압박·라인 상태·이동시간으로 PassiveLine/SerpenHuntAndPoke/None
- 0.5.8 params: (sret):Option<BigPlan>(384B)(tag -1(0xFFFF..)=None / 14=SerpenHuntAndPoke / 3=PassiveLine) · version:usize(v25_objective_splitter_should_join_contest 에만 전달(52002 `i64 ) · rnd:&mut StdRng(320B)(is_skip_serpen·PlayerState::strategy 에 전달) · player:&PlayerState(2528B)(info.team(+0x930)·info.position(+0x9c0)) · data:&OperationData(24B)() · team_plan:&TeamPlan(1064B)(vision.last_visible_pos[]·last_checked_ticks[] (Flexible 분기)) · phase:ObjectPhase(i8 0..4)(0 None / 1 Setup / 2 Assemble / 3 Hunt) · depart_anchor:&Option<(u64,u64)>(24B)(출발 기준점. Some 이면 챔피언 위치 대신 사용(L485)) · _debug:&mut DebugFrameData(224B)(미사용(readnone))
- 0.5.8 logic 전문:
```
ctx = data.context; team = player.info.team; my_pos = player.info.position
L417 if !serpen_exists(ctx) → return None (L418)
L421 is_line_phase = tutorial∈{0,5,7,8} ? tick < epic_jungle.first_spawn_tick.saturating_sub(tps*30) : true
L422 champ = cache.player_champion[team][my_pos].unwrap()
L423 skip = is_skip_serpen(version, rnd, player, data);  L425 if skip → return None (L506)
L426 match phase {
  Hunt(3)  → L427 return Some(BigPlan::SerpenHuntAndPoke(SerpenHuntAndPokePlan{v46_flee_threats: vec![], focus_serpen_only:false, vision_only:false, v46_flee:false}))
  Setup(1) → 아래
  _        → return None (L503)
}
L429 strategy = player.strategy(rnd, game)
L431 opposite_object_pressure = v23_enemy_object_pressure(player, data, JungleType::Morgard)
     (objective_helpers.rs:181~207 요지: 해당 오브젝트 생존 && 근처(180000, hp40%) 최근가시 적 != 0 && 적 >= 건강 아군)
L432 if opposite_object_pressure {
L433     if let Some(line) = v23_objective_setup_pressure_line(player, data, &[Bottom, Mid]) {
L434         return Some(PassiveLine(PassiveLinePlan{line, ..default}))
         }
     }
L438 object_buildup = strategy.object_buildup
L439 match object_buildup {
  Split(position) → L440 if !is_line_phase && position == my_pos {
                      L441 if !v25_objective_splitter_should_join_contest(version, player, data, JungleType::Serpen) {
                      L443     return Some(PassiveLine{line: fallback_line(ctx, Top)})
                      } }
                    // 그 외 → L496
  Flexible(6)     → L449 camp = map.camp_pos(Serpen, team==0)
                    L452 near_serpen_enemy = (0..5).filter_map(|p| player_champion[1-team][p]).filter(|e|
                    L454     last_pos = team_plan.vision.last_visible_pos[p];
                    L455     d = distance(last_pos, camp).saturating_sub(150000);
                    L456     move_speed = e.stat_cached.move_speed;
                    L457     can_move = (tick.saturating_sub(team_plan.vision.last_checked_ticks[p])) * move_speed;
                    L459     e.hp*100/e.max_hp > 49 && can_move >= d && !blackboard[1-team].is_recent_visible(game, player, e)
                         ).count()
                    L463 if near_serpen_enemy < 3 && dist_sq(champ, camp) <= 320000^2
                    L464    && is_enemy_side(ctx, team, champ.x, champ.y)   [= (team==0) XOR ((x - y + height) > width)] {
                    L466     bottom_count = blackboard[team].bottom_minion_state.minion_count; L467 mid_count = ...mid...
                    L469     if cache.bottom_lead[team] < 3 {
                                 if cache.mid_lead[team] < 3 { L470 return Some(PassiveLine{line: bottom_count < mid_count ? Bottom(L471) : Mid(L473)}) }
                    L476         else return Some(PassiveLine{line: Bottom})
                             }
                    L477     else if cache.mid_lead[team] < 3 { L478 return Some(PassiveLine{line: Mid}) }
                         }
                    L482 remain_spawn_tick = mode.jungle_runner.serpen.next_respawn_tick.saturating_sub(tick)
                    L483 camp_pos = map.camp_pos(Serpen, team==0)
                    L485 (dpx,dpy) = depart_anchor.unwrap_or((champ.x, champ.y))
                    L486 dist_to_camp = distance(dpx,dpy, camp_pos)
                    L487 move_speed = champ.stat_cached.move_speed;  L488 move_tick = dist_to_camp / move_speed
                    L489 if remain_spawn_tick > move_tick + tps*2 → L490 return None
                    // 아니면 L496
  Gather(5)       → L496
}
L496 return Some(BigPlan::SerpenHuntAndPoke(SerpenHuntAndPokePlan{빈 Vec, 플래그 false}))   // camp_pos 호출 결과는 IR 상 미사용
```

## `de92d0` → `fb5ff0` epic_passive_plan  (4417→5343B · Δ+926)
- 힌트: epic_passive_plan(이관 · 배치 D §9 골격 · 동형)
- exe 정렬: 명령 900→989 · 정렬 498 · 잔여 구조 7 · 분기 10 · 콜리 주의: 12a07d0→16a7af0 콜리 변경?(J0.00·+115B) ; 1323a00→157b720 미지(-31B) ; 1323a00→16a7af0 미지(+383B) ; 1323a00→ef57f0 미지(+449B) ; 1323a00→f89d70 미지(+142B) ; 1323a00→f904b0 미지(+232B) ; 1323a00→fbb470 미지(+110B) ; dd9f30→f10c60 미지(+39B)
- 0.5.8 명세 #82 `epic__epic_passive_plan` src game-ai\src\plan_legacy\old\epic.rs:317 · one_line: 에픽(모가드) 목표의 수동 국면에서 내 BigPlan(PassiveLine 라인 / EpicHuntAndPoke / None)을 고른다
- 0.5.8 params: (sret):Option<BigPlan>(384B)(sret · tag +0x0: -1=None / 3=PassiveLine(payload +0x8 Passiv) · version:usize(본문 분기 없음. v25_objective_splitter_should_join_contest(L340)에만) · rnd:&mut StdRng(PlayerState::strategy(L327)·can_near_enemies_range(L461)에 전달) · player:&PlayerState(2528B)(info.team(+0x930)·info.position@tag(+0x9c0)) · data:&OperationData(24B)(+0x0 cache / +0x8 context / +0x10 blackboard[2]) · phase:ObjectPhase(i8)(L324 switch: 3 Hunt → EpicHuntAndPoke 즉시 / 1 Setup → 본문 / 그 ) · team_plan:&TeamPlan(vision.last_visible_pos[p](+0x230+16p)·vision.last_checked_t) · depart_anchor:Option<(u64,u64)>(24B, by-re(L399 unwrap_or(내 챔프 좌표) — 캠프까지 이동시간 계산의 출발점) · debug:&mut DebugFrameData(readnone — 본문에서 읽지도 쓰지도 않음. can_near_enemies_range 에는 poison)
- 0.5.8 logic 전문:
```
ctx = data.context; if !spawn_epic(ctx.tutorial) (태그∈1..6) → return None                                   [L318-319]
champ = cache.player_champion[team][position].unwrap()                                                  [L322]
match phase { Hunt(3) → return EpicHuntAndPoke(default) [L325]; Setup(1) → 아래; _ → return None [L497] } [L324]
strategy = player.strategy(rnd, game)                                                                     [L327]
opposite_object_pressure = v23_enemy_object_pressure(player, data, Serpen)                                [L329]
if opposite_object_pressure { if let Some(line) = v23_objective_setup_pressure_line(player,data,&[Top,Mid]) → return PassiveLine{line} } [L330-332]
match strategy.object_buildup {                                                                            [L337]
  Split(pos) [L339]: if pos == my position { if !v25_objective_splitter_should_join_contest(version,player,data,Morgard) → return PassiveLine{Bottom} [L340-341] } → Gather 로 진행
  Flexible [L347]: camp = map.camp_pos(Morgard, team==0)
    can_reach = Σ_{p∈0..5, 적 챔프 e=player_champion[1-team][p] 존재} [                                    [L350-359]
        last_pos = team_plan.vision.last_visible_pos[p]; d = distance(last_pos, camp).sat_sub(150000)      [L352-353]
        can_move = (game.tick() − vision.last_checked_ticks[p]).sat_sub * e.move_speed                    [L354-355]
        e.hp*100/e.max_hp > 49 && can_move >= d && !blackboard[1-team].is_recent_visible(game, player, e) ] [L357]
    if can_reach < 2 [L362]: if blackboard[team].bottom_minion_state.minion_count < -3 && count{p≠me && blackboard[team].in_big_line(p,Bottom)}==0 → return PassiveLine{Bottom} [L361-366]
    if can_reach <= 2 (L362 count<2 에서 바텀 조건 실패한 경우 + L370 count==2) && dist²(champ, camp) < 320000² [L370] && is_enemy_side(ctx, team, champ.x, champ.y) [L371 = is_blue_side(x−y+height > width) XOR team==0]:
        top=bb[team].top_minion_state.minion_count; mid=bb[team].mid_minion_state.minion_count             [L373-374]
        if top_lead[team] < 3 { if mid_lead[team] < 3 { return PassiveLine{ if top<mid Top else Mid } [L377-380] } else return PassiveLine{Top} [L383] }
        else if mid_lead[team] < 3 → return PassiveLine{Mid} [L385]  (그 외 → Gather 로 진행) [L376-386]
    그 외 → Gather 로 진행
  Gather (및 위 폴백) [L394]: camp = map.camp_pos(Morgard, team==0)
    moba = game.get_game_mode().as_moba().unwrap(); remain = moba.jungle_runner.epic.next_respawn_tick.sat_sub(tick) [L397]
    pos = depart_anchor.unwrap_or((champ.x,champ.y)); travel = distance(pos,camp) / champ.move_speed (0이면 패닉) [L399-402]
    if remain > tps*2 + travel → return None                                                               [L403-404]
    if tick − team_plan.obj_spawn.epic_camp_last_visible_tick > tps*3 [L407]:
        near = team_plan.can_near_enemies_range(rnd, player, data, camp, 150000)                            [L461]
        if near.len() > 2 → return EpicHuntAndPoke                                                          [L462-463]
        if top_lead>2 { if mid_lead>2 → EpicHuntAndPoke [L470-471] else → PassiveLine{Mid} [L490] }
        else if mid_lead<3 → PassiveLine{ if top<mid Top else Mid } [L478-485] else → PassiveLine{Top} [L488]
    else:
        nearest = player_champion[1-team].filter(|e| bb[1-team].is_recent_visible(game,player,e)).min_by_key(dist²(e,camp)) [L408-409, aux m12]
        if top_lead>2 && mid_lead>2 → return EpicHuntAndPoke                                               [L412-413]
        if let Some(e)=nearest [L420]: if dist²(e,camp) < 150000² → return EpicHuntAndPoke [L421-422]
            top/mid = bb[team] minion_count [L424-425]; if top_lead<3 { if mid_lead<3 → PassiveLine{top<mid?Top:Mid} [L428-431] else PassiveLine{Top} [L434] } else if mid_lead<3 → PassiveLine{Mid} [L436] else → None [L439]
        else (적 시야 없음) [L442-457]: 같은 lead/minion 표 → Top/Mid(L447-454) 또는 None [L457]
}
```

## `df0e90` → `dcbe00` SerpenHuntAndPokePlan::sub_plan  (3471→4064B · Δ+593)
- 힌트: SerpenHuntAndPokePlan::sub_plan(이관 · 배치 D §6 · ef7590/ef6da0)
- exe 정렬: 명령 829→949 · 정렬 716 · 잔여 구조 10 · 분기 10 · 콜리 주의: 31a37c0→3821813 미지(+32B) ; 31a3863→3821770 미지(-32B) ; d3b2a0→eed7f0 변경 ; dd5db0→f0cfe0 변경 ; e1c100→df8d60 미지(pdata 밖 thunk) ; e7a8c0→ef7590 불일치(mig060 는 f27140) ; e7a8c0→f27140 변경
- 0.5.8 명세 #97 `serpen_hunt_and_poke__sub_plan` src game-ai\src\plan_legacy\old\serpen\hunt_and_poke.rs:30 · one_line: 세르펜 사냥/견제 빅플랜의 서브플랜 선택 — Epic 판과 동형(Steal/SerpenCheck/Recall/LineDefense/SerpenHunt/SerpenPoke), 차이 5곳은 unknown/logic 에 명시
- 0.5.8 params: (sret):*mut SubPlan(72B)(%0. 니치 태그(idx+2)) · self:&mut SerpenHuntAndPokePlan(3(%1. v46_flee_threats(+0x0 Vec)·focus_serpen_only(+0x18)·visi) · version:usize(%2. 본문 분기 없음 — upgrade_item·v46_flee_gate_check·v25·v24 에 전달) · rnd:&mut StdRng(320B)(%3. upgrade_item·PlayerState::strategy 에 전달) · player:&PlayerState(2528B)(%4. info.team(0x930)·info.position 태그(0x9c0)) · data:&OperationData(24B)(%5. cache·context·blackboard) · goal_data:&GoalData(248B)(%6. serpen.epic_enemy_tick(0xc0)·serpen.epic_ally_tick(0xd0)) · team_plan:&TeamPlan(1064B)(%7. objective 태그(0x41f)·phase(0x420) + v24 의 self) · debug:&mut DebugFrameData(224B)(%8. ctx.debug 일 때 v46 도주 로그)
- 0.5.8 logic 전문:
```
// serpen/hunt_and_poke.rs:30~92 (+ check_recall :99~159 인라인). Epic 판(epic_hunt_and_poke__sub_plan.json)과 같은 골격 — ★표시가 차이.
t, pos, champ = … (:32)
// A. 스틸 (:35~44)
if self.focus_serpen_only || self.vision_only {
   serpen = as_moba().jungle_runner.serpen.live_list.first().and_then(get_entity_by_id)   // :36~37 (MobaMode+0x1d0/+0x1d8)
   Some → Steal{ last_vision_tick:0, target:Some(Serpen) ★, commit:focus_serpen_only }   // :39
   None → SerpenCheck{false}                                                            // :44
}
// B. 일반
hp_ratio = hp*100/max_hp                                                                 // :47
serpen = live_list.first().and_then(get_entity_by_id); None → SerpenCheck{false}         // :48~50
(x0,y0,x1,y1) = map.fountains[t]; is_in_heal_area = …                                    // :55~56
can_upgrade_item = upgrade_item(version, rnd, player, game, ctx).is_some()               // :57 (★Epic 은 :48 에서 먼저 계산 — 순서만 다르고 값 동일)
if serpen.hp == serpen.max_hp && (hp_ratio<51 || can_upgrade_item || (is_in_heal_area && hp<max_hp)) → Recall   // :58~59
// C. 오브젝트 게이트
if team_plan.objective != Some(Serpen{..}) → Recall                                       // :63 / :90 ★태그 1
if phase == Hunt {                                                                        // :64
   strategy = player.strategy(rnd, game)                                                  // :65
   if object_buildup == Split(pos) && v25_objective_splitter_can_stay(version, player, data, Serpen ★5)   // :66~68
      → LineDefense{ Aggressive, fallback_line(ctx, Top ★0), Push }                       // :70~73
   else → SerpenHunt{ need_recall:false }                                                 // :77
}
// D. check_recall (:99~159)
   heal_area(t).contains(champ) && hp<max → true                                          // :103~104
   hp_ratio = hp*100/max                                                                  // :108
   min(goal_data.serpen.epic_enemy_tick, epic_ally_tick) <= tps*5 → false                 // :111
   v46 도주 블록 — Epic 판과 동일(:119~135, 위협 반경 200000, gate 코드 0, 로그)
   ★serpen = game.jungle_runner()(vtable+0xe0).serpen.live_list.first().and_then(get_entity_by_id)   // :142~143 (JungleRunner+0x1b8/+0x1c0)
   ★if serpen.is_none() → true(귀환)                                                       // :143 (Epic 판엔 이 조회 없음)
   ★enemy_cnt = player_champion[1-t] 중 blackboard[1-t].is_recent_visible && dist_sq(e, serpen) < 22500000001 인 수   // :148~151 closure$2
   ★ally_cnt  = player_champion[t]   중 blackboard[t].is_recent_visible   && dist_sq(a, serpen) < 22500000001 인 수   // :153~156 closure$3  (Epic 은 regions==7 셀)
   return hp_ratio < 21 && ally_cnt < enemy_cnt                                            // :159
if check_recall → Recall                                                                  // :79~80
// E. 꼬리
if team_plan.v24_objective_setup_should_check_camp(version, player, data, goal_data, Serpen ★5) → SerpenCheck{false}   // :82~83
★else → SerpenPoke (거리 게이트 없음 — Epic 판은 dist<150000 일 때만 Poke, 아니면 Check)   // :86
```

## `defcd0` → `fd7f70` EpicHuntAndPokePlan::sub_plan  (3413→5068B · Δ+1655)
- 힌트: EpicHuntAndPokePlan::sub_plan(전담 · 배치 B §14)
- exe 정렬: 명령 800→1171 · 정렬 474 · 잔여 구조 3 · 분기 10 · 콜리 주의: 1323a00→2f380b0 미지(-58B) ; 1323a00→381ebcf 미지(-89B) ; d3b2a0→ef6da0 불일치(mig060 는 eed7f0) ; d7bdf0→3821770 미지(-292B) ; dd5db0→f0cfe0 변경 ; e1c100→fed330 미지(pdata 밖 thunk) ; e7a8c0→f27140 변경
- 0.5.8 명세 #83 `epic_hunt_and_poke__sub_plan` src game-ai\src\plan_legacy\old\epic\hunt_and_poke.rs:30 · one_line: 에픽(Morgard) 사냥/견제 빅플랜의 서브플랜 선택 — Steal/EpicCheck/Recall/LineDefense/EpicHunt/EpicPoke 중 하나를 sret 로 반환
- 0.5.8 params: (sret):*mut SubPlan(72B)(%0. 태그 = enum+0x0 (니치: tag = idx+2). 반환 variant 는 writes 참조) · self:&mut EpicHuntAndPokePlan(32B(%1. v46_flee_threats(Vec, +0x0)·focus_epic_only(+0x18)·visio) · version:usize(%2. 본문 분기 없음 — upgrade_item·v46_flee_gate_check·v25_objectiv) · rnd:&mut StdRng(320B)(%3. upgrade_item·PlayerState::strategy 에 전달) · player:&PlayerState(2528B)(%4. info.team(0x930)·info.position 태그(0x9c0)) · data:&OperationData(24B)(%5. cache(+0)·context(+8)·blackboard(+0x10)) · goal_data:&GoalData(248B)(%6. epic.epic_enemy_tick(0x88)·epic.epic_ally_tick(0x98) — c) · team_plan:&TeamPlan(1064B)(%7. objective(0x41f 태그, 0x420 phase) + v24_objective_setup_s) · debug:&mut DebugFrameData(224B)(%8. ctx.debug 일 때 v46 도주 로그 add_log 에만)
- 0.5.8 logic 전문:
```
// hunt_and_poke.rs:30~96 (+ check_recall :103~160 인라인). 분기 순서대로.
t = player.info.team; pos = player.info.position
champ = cache.player_champion[t][pos].unwrap()                                   // :32

// ── A. 스틸 모드 (:35~44)
if self.focus_epic_only || self.vision_only {
   moba = game.get_game_mode() (vtable+0x40) → tag 0 = Moba 아니면 unwrap 패닉      // :36
   epic = moba.jungle_runner.epic.live_list.first().and_then(|id| game.get_entity_by_id(id))   // :37 closure$0
   if epic.is_some() → return Steal{ last_vision_tick:0, target:Some(Epic), commit:self.focus_epic_only }   // :39
   else → return EpicCheck{ move_check:false }                                        // :44
}

// ── B. 일반 (:47~)
hp_ratio = champ.hp*100 / champ.stat_cached.hp                                    // :47 (max_hp 0 → 패닉)
can_upgrade_item = upgrade_item(version, rnd, player, game, ctx).is_some()          // :48
epic = moba.jungle_runner.epic.live_list.first().and_then(get_entity_by_id)         // :49 closure$1
if epic.is_none() → return EpicCheck{false}                                        // :50~51
(x0,y0,x1,y1) = map.fountains[t]                                                    // :56
is_in_heal_area = x0<=champ.x<=x1 && y0<=champ.y<=y1                                // :57
if epic.hp == epic.stat_cached.hp /*에픽 풀피*/ && (hp_ratio < 51 || can_upgrade_item || (is_in_heal_area && champ.hp < champ.stat_cached.hp))   // :58 (줄 안 순서 표기 불가)
   → return Recall                                                                   // :59

// ── C. 팀 오브젝트 게이트 (:63~64)
if team_plan.objective != Some(Morgard{..}) → return Recall                         // :63 objective_target / :94
if phase == Hunt {                                                                   // :64 take_hunt_commit
   strategy = player.strategy(rnd, game)                                             // :65
   if strategy.object_buildup == Split(position) && position == pos                  // :66
      && v25_objective_splitter_can_stay(version, player, data, Morgard)             // :68
      → return LineDefense{ style:Aggressive, line:fallback_line(ctx, Bottom), minion_action_type:Push }   // :70~73
   else → return EpicHunt{ need_recall:false }                                       // :77
}

// ── D. check_recall(self, version, player, data, goal_data, debug) (:79, 본문 :103~160)
   champ = player_champion[t][pos].unwrap()                                          // :104
   if heal_area(t).contains(champ) && champ.hp < champ.max_hp → true                 // :107~108 (game_core heal_area 인라인: 팀0 x<=64000&&y>=896000&&y<=960000 / 팀1 x>=892000&&x<=960000&&y<=64000)
   hp_ratio = champ.hp*100/champ.max_hp                                              // :112
   if min(goal_data.epic.epic_enemy_tick, epic_ally_tick) <= tps*5 → false           // :115
   if self.v46_flee {                                                                // :124
      if self.v46_flee_threats.iter().any(|id| game.get_entity_by_id(id).is_some_and(|e| dist_sq(e,champ) < 40000000001)) → true   // :125~126
      self.v46_flee=false; self.v46_flee_threats.clear()                              // :131
   } else {
      (code, threats) = v46_flee_gate_check(version, player, data, champ)             // :134
      if code == 0 { self.v46_flee=true; self.v46_flee_threats = threats.to_vec();    // :136~137
                     if ctx.debug { debug.add_log(data, player, format!(.. tick, position, .. threats)) }   // :138~139
                     return true }                                                    // :140
   }
   enemy_cnt = player_champion[1-t].iter().flatten().filter(|e| map.regions[e.y/32000][e.x/32000]==7 && blackboard[1-t].is_recent_visible(game, player, e)).count()   // :147~149
   ally_cnt  = player_champion[t].iter().flatten().filter(|a| regions==7 && blackboard[t].is_recent_visible(game, player, a)).count()                 // :154~155
   return hp_ratio < 21 && ally_cnt < enemy_cnt                                      // :160
if check_recall(..) → return Recall                                                  // :79~80

// ── E. 캠프 확인/견제 (:82~89)
if team_plan.v24_objective_setup_should_check_camp(version, player, data, goal_data, Morgard) → return EpicCheck{false}   // :82~83
if dist_sq(champ, epic) < 22500000001 → return EpicPoke                             // :86 → :87
else → return EpicCheck{false}                                                       // :89
```
