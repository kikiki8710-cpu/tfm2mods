# 배치 B — 실변경 6 (0.5.8 RVA → 0.6.0 RVA · exe 정렬 힌트 · 0.5.8 명세 요지)

## `dc2960` → `ed0400` SmallActionAroundBush::get_input
- 명령 372→373 · 정렬 312 · exe 판정 ❌구조 변경 · 블록이동 24 · 스택슬롯 41 · 콜리 주의 2
- 구조 차이: 구 60명령(mov|r15, qword ptr [rbp + I]…) / 신 61명령(mov|r8, qword ptr [rbp + I]…) 짝 없음 ; dc2a53 피연산자 형 ; dc2b4d IAT ('kernel32.dll', 'HeapFree')≠('kernel32.dll', 'GetProcessHeap') ; dc2cca 피연산자 형
- 분기 차이: dc2a6d → dc2be5/ed05a2 ; dc2be3 → dc2bfe/ed06de ; dc2db9 → dc2d86/ed0818 ; dc2dc7 → dc2dd6/ed0868
- 변위: r13+0x930→0xa00 · r13+0x9c0→0xa90 · r12+0x18→0x50 · r12+0x50→0x58 · r12+0x18→0x50 · r12+0x20→0x18
- 콜리 주의: ce2df0→e1f610 불일치(mig060 는 e19550) ; cecdd0→e2d650 불일치(mig060 는 e266e0)
- 0.5.8 명세 #168 `SmallActionAroundBush__get_input` src game-ai\src\small_action\around.rs:1195 · one_line: 수풀 목표 셀까지 check_cell(지역 규칙+타워 회피) 술어로 PathFinder 경로를 만들고/갱신해 이동 입력 생성, 디버그면 경로선·지역 로그 기록
- 0.5.8 logic(앞 1800자):
```
SmallActionAroundBush::get_input(&mut self, version, rnd, player, data, ps, debug) -> Option<Input>   [around.rs:1195]
 entity = data.cache.player_champion[player.team][player.position].unwrap()          (L1196)
 tcx = min(self.target_x/32000, 29) ; tcy = min(self.target_y/32000, 29)             (L1197~1198; 지역 변수, 클로저에 &참조)
 tower_dodge = TowerDodgeContext::new_tower_avoid_v3(version, player, data, self.target_x, self.target_y)   (L1199)
 if self.path_finder.is_none():                                                        (L1200)
     self.path_finder = PathFinder::new_target(rnd, data.context, version, "around_bush", entity.x, entity.y, self.target_x, self.target_y,
         |sx,sy,nx,ny| check_cell(version, player, ps, data, &tower_dodge, self.out_line, sx,sy,nx,ny, tcx, tcy))   (L1201; 클로저 환경 8칸: &version, player, ps, data, &tower_dodge, &self.out_line, &tcx, &tcy)
     if self.path_finder.is_none(): return None                                        (L1206 — new_target 결과 None 가능, 105492)
 pf = self.path_finder.as_mut()  (Some 확정)
 pf.update_path(rnd, data.context, entity.x, entity.y, self.target_x, self.target_y, 같은 클로저(s_0))   (L1206)
 if self.path_finder.is_none(): return None                                            (L1210, 105613 — update_path 뒤 재검사)
 input = pf.get_input(player, data, SafeMoveWithSkill::Safe, false)                    (L1210)
 if data.context.debug && input is Move{x:mx, y:my}:                                    (L1211~1212)
     debug.infos.entry(entity.id).or_insert(Vec::new()).push(format!("region: {}", map.regions[entity.y/32000][entity.x/32000]))   (L1213; ex,ey ≥ 960000 이면 panic)
     debug.add_line(entity.x, entity.y, mx, my, Color(1,0,1,1))                          (L1214)
 return Some(input)                                        
```

## `e23fa0` → `e5e0e0` SmallActionPlay::get_input
- 명령 574→564 · 정렬 564 · exe 판정 ❌구조 변경 · 블록이동 33 · 콜리 주의 14
- 구조 차이: 구 10명령 / 신 0명령 짝 없음
- 분기 차이: e24386 switch case 수 8→7(정렬 실패) ; e243ce → e24660/e5e753 ; e2459f → e2463b/e5e778
- 소형 즉치: 0xe242e4:0x7→0x6 · 0xe242f1:0x10→0xf · 0xe24378:0x7→0x6 · 0xe2472c:0x7→0x6 · 0xe24734:0xb→0xa
- 즉치: 0x7→0x6 · 0x10→0xf · 0x7→0x6 · 0x7→0x6 · 0xb→0xa · 0xfcf→0x7e7
- 변위: rax+0x930→0xa00 · rax+0x9c0→0xa90
- 콜리 주의: cce210→ec3060 불일치(mig060 는 fd98c0) ; d845e0→ed4350 불일치(mig060 는 ddd010) ; d84800→ddd690 불일치(mig060 는 ddd230) ; d84a30→eccaf0 불일치(mig060 는 ddd460) ; d84c60→e95f70 불일치(mig060 는 ddd690) ; db6ff0→ddd460 불일치(mig060 는 ec3060)
- 0.5.8 명세 #141 `SmallActionPlay__get_input` src game-ai\src\small_action.rs:157 · one_line: 소액션 → 엔진 입력(Option<Input>) 디스패처 — 적 우물 위험 시 탈출 Move 선출력, 종류별 get_input 위임, Move 결과의 우물 회피 보정, v2+ 에서 '제자리 Move' 를 목표점 Move 로 승격
- 0.5.8 logic(앞 1800자):
```
L159: champ = data.cache.player_champion[team][position]?  — None → return None(-1)
L160~161: if is_enemy_well_danger(version, player, champ.x, champ.y) || is_recent_enemy_well_damage_danger(version, player, champ) {
  L162: (x,y) = enemy_well_escape_position(ctx.setting, ctx.map, player, champ.x, champ.y)   [ScalarPair (x,y)]
  L168: if version <= 1 || distance_sq(champ,(x,y)) >= 4000001 (>2000) → L169: return Some(Move{x,y})
        (v2+ 이고 탈출점에 이미 도달(≤2000)이면 통과)
}
L174~178: phase = match self { RunAway|AroundRunAway→72, LaneMinionPosition→66, Trace→67, Attack|Skill|Skill2|Ult→47, _→46 } ; _t = ProfTimer::start(phase)  (prof::ENABLED 일 때만 Instant::now · 텔레메트리)
L181~200: input: Option<Input> = match self {
  RunAway(0)→SmallActionRunAway::get_input(self,version,rnd,player,data,positioning_score,debug)   L182
  Recall(1)→SmallActionRecall::get_input(…,debug) L183 · Positioning(6)→SmallActionPositioning::get_input(… 7인자) L184
  Around(2)→SmallActionAround::get_input(…,debug) L185 · AroundHide(3)→(7인자) L186 · AroundRegion(4)→(7인자) L187
  AroundRunAway(5)→SmallActionAround::get_input(…,debug) L188 (Around 와 같은 페이로드 타입) · AroundPosition(7)→(7인자) L189
  AroundPositionBush(8)→(7인자) L190 · AroundBush(9)→(…,debug) L191 · LaneMinionPosition(10)→fastcc get_input(…,debug) L192 · Trace(11)→(…,debug) L193
  Attack(12)/Skill(13)/Skill2(14)/Ult(15)→cast::*::get_input(… 7인자) L194~197
  Stop(16)→ Some(Move{champ.x, champ.y})  L200 (태그 store 는 다음 단계 Move 분기로 접힘)
}
L204: out = avoid_enemy_well_move_input(version, player, data, input)  [인라인 small_action.rs:34~41]:
      if input.tag==0 (Some(Move{x,y})): champ = player_champion[..]? (None→None) ; out = safe_move_avoiding_enemy_well(version, player, data, champ, x, y)  [잎22 · sret 32B Option<Input>]
      else: out = input 그대로(memcpy 32B)

```

## `e23170` → `e5d4e0` SmallActionPlay::evaluation_position
- 명령 164→164 · 정렬 164 · exe 판정 ❌구조 변경 · 블록이동 28
- 분기 차이: e231dc switch case 수 17→16(정렬 실패) ; e2322e → e2341d/e5d652 ; e23262 → e232a8/e5d770 ; e2339e → e233f4/e5d799
- 소형 즉치: 0xe231c9:0x7→0x6 · 0xe231d1:0x10→0xf · 0xe23211:0x3→0x5 · 0xe233a7:0x5→0x3
- 즉치: 0x7→0x6 · 0x10→0xf · 0x3→0x5 · 0x58→0x20 · 0x50→0x18 · 0x20→0x58 · 0x18→0x50 · 0x5→0x3
- 변위: r9+0x930→0xa00 · r9+0x9c0→0xa90 · r15+0x500→0x538 · r15+0x68→0x88 · r15+0x538→0x500 · r15+0x68→0x88 · r15+0x60→0x80
- 0.5.8 명세 #145 `SmallActionPlay__evaluation_position` src game-ai\src\small_action.rs:245 · one_line: 소액션의 '평가 위치' — 액션 종류별 목표점을 구해, 공격/스킬계는 착지점 그대로, 그 외는 그 목표를 향해 1초(move_speed×tps)만큼 이동한 좌표를 Option<(x,y)> 로
- 0.5.8 logic(앞 1800자):
```
L246: champ = data.cache.player_champion[player.team][player.position]?   — None → return None
L247: target: Option<(u64,u64)> = match self (idx=tag−3) {
  L248 Trace(11):  self.expected_goal_position(player, data).or_else(|| Some((trace.goal_x, trace.goal_y)))   [잎22 · sret 24B Option<(u64,u64)> · None 이면 +0x68/+0x70 폴백]
  L249 Attack(12): SmallActionAttack::movement_landing_position(self, champ, data)      [sret 24B Option<(u64,u64)>]
  L250 Skill(13):  SmallActionSkill::movement_landing_position(self, champ, data)
  L251 Skill2(14): SmallActionSkill2::movement_landing_position(self, champ, data)
  L252 Ult(15):    SmallActionUlt::movement_landing_position(self, champ, data)
  L253 Stop(16):   Some((champ.x, champ.y))   → L257 분기 없이 바로 move_to 경로(42020)
  L254 그 외:      self.target_position()  [인라인 small_action.rs:230~238]
      RunAway(0)/Positioning(6)/AroundPosition(7)/AroundPositionBush(8) → Some((+0x8, +0x10))
      Recall(1)                                                          → Some((+0x50, +0x58))
      Around(2)/AroundHide(3)/LaneMinionPosition(10)                    → Some((+0x10, +0x18))
      AroundBush(9)                                                      → Some((+0x18, +0x20))
      AroundRegion(4)/AroundRunAway(5)                                   → None   (o145 case6 AroundRunAway → game=None 실행 확인)
}?   — None → return None (42048)
L257: if idx ∈ {Attack,Skill,Skill2,Ult} (idx & !3 == 12) → return Some(target)   (L258 · 착지점 그대로)
L260~261: x = champ.x ; y = champ.y
L262~264: Entity::move_to(ctx.setting, ctx.map_setting, ctx.map, champ.id, &mut x, &mut y, champ.stat_cached.move_speed * setting.tick_per_second, target.x, target.y, &mut None(ptr null))
L265: return Some((x, y))   — 1초 뒤 예상 위치
```

## `e26c40` → `e95f70` SmallActionLaneMinionPosition::get_input
- 명령 348→350 · 정렬 348 · exe 판정 ❌구조 변경 · 스택슬롯 10 · 콜리 주의 1
- 구조 차이: 구 0명령 / 신 2명령 짝 없음
- 분기 차이: e27293 → e27102/e96432 ; e272da → e27102/e96432
- 변위: r9+0x930→0xa00 · r9+0x9c0→0xa90
- 콜리 주의: dc3240→fddcd0 변경
- 0.5.8 명세 #167 `SmallActionLaneMinionPosition__get_input` src game-ai\src\small_action\lane_minion.rs:540 · one_line: 라인미니언 포지셔닝 소액션의 매틱 실행: 목표 재선정(choose_goal)→12000 초과 이동 시 경로 재생성→타워 탈출 필요면 RunAway, 아니면 PathFinder 입력
- 0.5.8 logic(앞 1800자):
```
fn get_input(&mut self, version, rnd:&mut StdRng, player, data, positioning_score, debug:&mut DebugFrameData) -> Option<Input>
[L541] target = data.cache.game.get_entity_by_id(self.target)?            ; vtable+0x1f0 · null→None(46112)
[L542] champ = data.cache.player_champion[player.info.team][player.info.position]?   ; 46150
[L543] if target.team == champ.team || !target.is_minion() { [L544] return None }   ; TeamType derive-eq(둘 다 Neutral 도 '같음'), is_minion = ty@tag==1. 46146~46200 → 46165
[L552] unseen = !(champ.team is Player(t) && target.visible_state[t]==Visible)      ; champ Neutral 이면 unseen=false 로 진행(46170→46217)
[L553] if unseen && (version < 2 || self.goal_x == 1987654321 || self.goal_y == 1987654321) { [L554] return None }   ; 46226~46241
[L557] line = match player.info.position { Top(0)=>Top(0), Jungle(1)=>[L562] return None, Mid(2)=>[L559] Mid(1), Bottom(3)|Support(4)=>[L560] Bottom(2) }   ; unseen 경로는 Jungle 검사만(46352→46251)
[L570] chosen: Option<(x,y,score)> = if unseen { Some((self.goal_x, self.goal_y, self.goal_score)) }   ; 46357~46365
[L572]   else { SmallActionLaneMinionPosition::choose_goal(self.goal_score, self.position_eval_purpose, version, player, data, champ, target, positioning_score, line) }   ; 46260, sret 32B(+0 tag 1=Some,+8 x,+16 y,+24 score)
[L574] (gx, gy, gs) = match chosen {
   Some(c) => c,                                                                  ; [L575] 46266~46270
   None => { [L576] if version > 1 {
              [L577] if self.goal_x==SENT || self.goal_y==SENT { [L578] (target.x, target.y, self.goal_score) }   ; 46301~46308
                     else { (self.goal_x, self.goal_y, self.goal_score) } }
            else { [L583] return None } } }                                         ; 46283
[L585] if dist_sq((self.goal_x
```

## `dc2330` → `ecfdd0` around::SmallActionAroundBush::update_state
- 명령 122→123 · 정렬 121 · exe 판정 ❌구조 변경 · 블록이동 2 · 콜리 주의 1
- 구조 차이: 구 1명령(jne|I…) / 신 2명령(je|I…) 짝 없음
- 변위: r8+0x930→0xa00 · r8+0x9c0→0xa90
- 콜리 주의: 31a01a3→381e0b3 미지
- 0.5.8 명세 #239 `around__SmallActionAroundBush__update_state` src game-ai\src\small_action\around.rs:1221 · one_line: 부쉬 주변대기 상태 갱신: 목표점 16000 이내 도달 && change_tick 경과면 같은 부쉬(id) 셀 71 후보 중 무작위 셀로 목표를 옮기고 change_tick = tick + rand(60..=120); debug 면 선 그리기
- 0.5.8 logic(앞 1800자):
```
fn update_state(&mut self, rnd, player, data, debug)   // around.rs:1221~1238
  // L1222
  champ = data.cache.player_champion[player.info.team][player.info.position.as_index()].unwrap()
  // L1224 — 도달 && 교체 시각 경과 (단락: 거리 먼저, tick() 가상호출은 거리 조건 참일 때만)
  if champ.distance_sq((self.target_x, self.target_y)) < 256000001 && game.tick() >= self.change_tick {
      // L1225 — 71 개 고정 셀 후보(@anon…94) 중 self.bush 와 같은 부쉬 id 인 셀만
      const CANDIDATES: [(usize,usize);71] = [(0,0),(1,0),(2,0),(19,0),(20,0),(21,0),(0,1),(0,2),(7,4),(8,4),(9,4),(20,4),(20,5),(12,6),(4,7),(4,8),(29,8),(4,9),(14,9),(29,9),(29,10),(26,11),(6,12),(12,12),(13,12),(26,12),(12,13),(26,13),(9,14),(26,14),(20,15),(21,15),(17,16),(21,16),(16,17),(17,17),(25,18),(0,19),(25,19),(0,20),(4,20),(5,20),(15,20),(25,20),(0,21),(15,21),(16,21),(25,21),(25,22),(29,24),(18,25),(19,25),(20,25),(21,25),(22,25),(29,25),(11,26),(12,26),(13,26),(14,26),(29,26),(28,28),(29,28),(8,29),(9,29),(10,29),(24,29),(25,29),(26,29),(28,29),(29,29)]
      candidates = CANDIDATES.into_iter().filter(|(x, y)| data.context.map.bushes[*y][*x] == self.bush)   // aux call_mut: bushes = MapDef+0x1c98 [[usize;30];30] · 튜플 .0=x(+0) .1=y(+8)
      // L1226
      (cx, cy) = candidates.choose(rnd).unwrap()          // aux choose: reservoir — 매칭 원소 k 번째마다 gen_range(0..k)==0 이면 교체 · 매칭 0개 → None → unwrap_failed 패닉
      // L1227~1228
      target_x = cx*32000 + 16000; target_y = cy*32000 + 16000
      // L1230~1231
      self.target_x = target_x; self.target_y = target_y
      // L1232
      self.change_tick = game.tick() + rnd.gen_range(60..=120)   // RangeInclusive{60,120,exhausted=false} · tick 재호출
  }
  // L1235
  if data.context.debug {
      // L1236
      debug.add_line(champ.x, champ.y, self.target_x, self.target_y, &[1.0,1.0,0.0,1.0])   // 갱
```

## `ccacf0` → `f39240` LineSafeSubPlan::action_candidates
- 명령 675→685 · 정렬 670 · exe 판정 ❌구조 변경 · 블록이동 45 · 스택슬롯 33 · 콜리 주의 2
- 구조 차이: 구 5명령(xor|edx, edx…) / 신 15명령(mov|rsi, qword ptr [rbp - I]…) 짝 없음 ; ccb421 피연산자 수 ; ccb740 피연산자 형 ; ccb746 피연산자 형
- 분기 차이: ccb3b0 → ccb46a/f396b5 ; ccb45f → ccb527/f39614 ; ccb4ed → ccb067/f39a82 ; ccb6ee → ccb6f8/f395cd
- 소형 즉치: 0xccb7a3:0xf→0xe · 0xccb7fb:0xf→0xe
- 즉치: 0x2e8→0x5c8 · 0xf→0xe · 0xf→0xe
- 변위: r10+0x930→0xa00 · rax+0x9c0→0xa90 · r9+0xe8→0x108
- 콜리 주의: 31a01a3→381e0b3 미지 ; cada80→f26680 미지(+2B)
- 0.5.8 명세 #253 `LineSafe__action_candidates` src game-ai\src\plan_legacy\sub_plan\line_safe.rs:89 · one_line: 라인 안전(LineSafe) 서브플랜의 소행동 후보 생성: 아군 최전방(타워/최전방 미니언) 주변 대기 + 도주 + 전투 + 미니언 + 소환수 + 사거리 안 적 타워 평타 + 구조물 스킬을 모은 뒤 v30 타워 어그로 위험 액션을 제거
- 0.5.8 logic(앞 1800자):
```
fn action_candidates(&mut self, version, rnd, player, data, _parameter) -> Vec<SmallActionPlay>  [line_safe.rs:89]
  L90  bump = data.context.pool; res = Vec::new_in(bump)   // sret 32B: ptr=dangling8, a=bump, cap=len=0

  L92  res.extend(self.base_positioning(version, rnd, player, data))   — 인라인(line_safe.rs:15~42), 항상 원소 1개:
    L16  vec 는 같은 bump 에 new_in
    L18  team = player.info.team;  tower = cache.tower(self.line, team)   // simulation.rs 인라인: team<2 bounds check(line 별 panic 3곳) → match line { Top: top_tower[team].or(top_tower2[team]) · Mid: mid_tower.or(mid_tower2) · Bottom: bottom_tower.or(bottom_tower2) } (Option::or, 첫 Some)
    L19~24  twin = cache.twin_towers[team].iter()
              .min_by_key(|t| { L21 (x,y) = self.line.get_start_position(context.setting, team); L22 utils::distance_sq(t.x, t.y, x, y) })   // 첫 원소는 본문 인라인·2번째부터 aux m12 fold(원소마다 get_start_position 재호출) · 동률이면 앞 원소 유지 · len==0 → None
              .map(|t| *t)  (L24 closure#1)
         nearest_tower = tower.or(twin)                     // ★eager: twin 의 min_by_key 는 tower 가 Some 이어도 계산된다(IR 이 %66 select 전에 %104 fold 호출)
    L25  nearest_tower = nearest_tower.or(cache.nexus[team])
    L26  nearest_tower = nearest_tower.unwrap()             // 셋 다 None 이면 unwrap_failed 패닉
    L28  nexus = cache.nexus[team].unwrap()
    L30  ms = data.blackboard[team].minion_state(self.line)   // Top→+0x0 · Mid→+0x28 · Bottom→+0x50 (BrainMinionParameter 40B)
    L31  front_minion: Option<&Entity> = ms.front_minion.and_then(|id| cache.game.get_entity_by_id(id))   // vtable +0x1f0
    L33  if front_minion.is_none_or(|m| utils::distance_sq(m, nexus) < utils::distance_sq(nearest_tower, nexus)) {   // None 이거나, 최전방 미니언이 우리 넥서스에 타워보다 더 가까우면(웨이브가 타워 안쪽까지 밀림)
    L34~35   [ SmallActionPlay::Around( SmallActionAr
```
