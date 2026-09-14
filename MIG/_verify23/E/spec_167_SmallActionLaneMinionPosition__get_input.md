---

### `167` SmallActionLaneMinionPosition::get_input — 라인미니언 포지셔닝 소액션의 매틱 실행: 목표 재선정(choose_goal)→12000 초과 이동 시 경로 재생성→타워 탈출 필요면 RunAway, 아니면 PathFinder 입력

| 항목 | 값 |
|---|---|
| id | `SmallActionLaneMinionPosition__get_input` |
| 심볼 | `_RNvMNtNtCshdEBA0ozCnw_7game_ai12small_action11lane_minionNtB2_29SmallActionLaneMinionPosition9get_input` |
| 소스 | `game-ai\src\small_action\lane_minion.rs:540` |
| IR | `m11.ll` 46067~46558행 |
| 경로·가시성 | `game_ai::SmallActionLaneMinionPosition::get_input` · **in:game_ai** |
| 계층 | 기타 |
| exe | `e26c40` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r14` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&mut game_ai::SmallActionLaneMinionPosition, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &mut game_core::DebugFrameData) -> std::option::Option<game_core::Input>
```

<details><summary>인자 8개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | Option<Input> (32B) | +0 tag i64: None = -1(DWARF variant0 DISCR_EXACT -1 · 46112 등 8곳), Some = Input 태그 0..5(Move/Return/Attack/Skill/Skill2/Ult · tcxdict --enum game_core::Input). None 경로 live 바이트 = [0x0,0x8) 뿐. Some 은 PathFinder::get_input(46537) 또는 SmallActionRunAway::get_input(46546) 이 sret 로 직접 채움 — 어느 바이트가 live 인지는 그 콜리 소관(미확인) | 3 |
| 1 | 1 | &mut self | &mut SmallActionLaneMinionPosition (128B) | IR 속성 `noalias nonnull align 8 dereferenceable(128)` — readonly 없음 = &mut. writes 전수는 writes 항목 | 4 |
| 2 | 2 | version | usize | 분기 2곳: <2(46226) / >1(46279). 콜리 8곳에 전달 | 4 |
| 3 | 3 | rnd | &mut StdRng (320B) | align 16 dereferenceable(320), readonly 없음. 본 함수 직접 접근 0 — new_target·update_path·is_safe_recall·RunAway::get_input 에 전달 | 4 |
| 4 | 4 | player | &PlayerState (2528B) | readonly. info.team(0x930)·info.position(0x9c0) | 4 |
| 5 | 5 | data | &OperationData (24B) | readonly. cache(+0)·context(+8) | 4 |
| 6 | 6 | positioning_score | &PositioningScoreData (2760B) | readonly. choose_goal·클로저 캡처·is_safe_recall·RunAway::get_input 에 전달 | 4 |
| 7 | 7 | debug | &mut DebugFrameData (224B) | align 8 dereferenceable(224), readonly 없음. 본 함수 직접 접근 0 — RunAway::get_input(46546) 에만 전달 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
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
[L585] if dist_sq((self.goal_x,self.goal_y),(gx,gy)) > 144000000 {                 ; 46344 (12000^2)
   [L586~588] self.goal_x=gx; self.goal_y=gy; self.goal_score=gs                    ; 46371~46374
   [L589] self.path_finder = None                                                   ; 옛 PathFinder drop(46408·46430) → 태그 2(46454)
   [L596] tower_dodge = TowerDodgeContext::new_unnecessary_tower_avoid(version, player, data, gx, gy)   ; 46458, 416B
 } else {
   [L591] self.goal_score = gs                                                       ; 46435 (좌표는 옛 것 유지)
   [L596] tower_dodge = TowerDodgeContext::new_unnecessary_tower_avoid(version, player, data, self.goal_x, self.goal_y)   ; 46442
 }
[L597] if self.path_finder.is_none() {                                              ; 46438 / 먼 목표 경로는 항상 진입
   [L598] self.path_finder = PathFinder::new_target(rnd, data.context, version, "lane_minion_position", champ.x, champ.y, goal_x, goal_y, closure$0{version,player,data,positioning_score,&tower_dodge,champ})   ; 46488, sret 72B → self+0x30 memcpy(46491)
 }
[L612] p = self.path_finder.as_mut()?                                               ; 태그 2 면 None(46495→46527)
[L613] p.update_path(rnd, data.context, champ.x, champ.y, goal_x, goal_y, closure$s_0{같은 캡처})   ; 46520
[L626] if path_needs_tower_escape(version, player, data, p) {                         ; 46523
   [L627] runaway = SmallActionRunAway::new_with_skill(data, player, 5, true)        ; 46544, 136B
   [L628] return runaway.get_input(version, rnd, player, data, positioning_score, debug)   ; 46546 → sret 직접 · [L629] drop(runaway)
 }
[L631] return p.get_input(player, data, SafeMoveWithSkill::Safe(1), is_safe_recall(version, rnd, player, data, positioning_score))   ; 46536~46537

```

**`mem` 메모리 접근 29건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | OperationData | 0x0 | cache | r | 46087 | 4 | OK |  |
| 1 | OperationData | 0x8 | context | r | 46449/46470 → new_target·update_path 의 ctx 인자 | 4 | OK |  |
| 2 | AbstractGameWithCache | 0x0 | game data ptr | r | 46088 | 4 | OK |  |
| 3 | AbstractGameWithCache | 0x8 | game vtable | r | 46092 → vtable+0x1f0 = AbstractGame::get_entity_by_id(divtable) · 46096~46098 간접호출(self.target) | 3 | OK |  |
| 4 | AbstractGameWithCache | 0x1e0 | player_champion[team][pos] | r | 46123~46126 | 4 | OK |  |
| 5 | PlayerState | 0x930 | info.team | r | 46106, bounds<2(46108) | 4 | OK |  |
| 6 | PlayerState | 0x9c0 | info.position@tag | r | 46120. switch(46217): 0 Top→line 0, 1 Jungle→None, 2 Mid→1, 3 Bottom/4 Support→2 · unseen 경로는 ==1 만 검사(46352) | 4 | OK |  |
| 7 | SmallActionLaneMinionPosition | 0x8 | self.target (entity id) | r | 46094 | 4 | OK |  |
| 8 | SmallActionLaneMinionPosition | 0x10 | self.goal_x | r | 46227·46272·46287·46370 | 4 | OK |  |
| 9 | SmallActionLaneMinionPosition | 0x18 | self.goal_y | r | 46231·46274·46290·46369 | 4 | OK |  |
| 10 | SmallActionLaneMinionPosition | 0x20 | self.goal_score | r | 46256(choose_goal 인자)·46357 | 4 | OK |  |
| 11 | SmallActionLaneMinionPosition | 0x78 | self.position_eval_purpose | r | 46258 → choose_goal | 4 | 오귀속(사전은 다른 필드를 준다) |  |
| 12 | SmallActionLaneMinionPosition | 0x75 | self.path_finder@tag (bool 니치, 2=None) | r | 46377·46436·46468(재검사) | 4 | OK |  |
| 13 | SmallActionLaneMinionPosition | 0x58 | self.path_finder.path Box ptr | r | 46384 (drop: dealloc 1120B=70*(u64,u64) · 46408) | 4 | 오귀속(사전은 다른 필드를 준다) |  |
| 14 | SmallActionLaneMinionPosition | 0x60 | self.path_finder.planned_verdict Box ptr | r | 46386 (drop: dealloc 70B · 46430) | 4 | 오귀속(사전은 다른 필드를 준다) |  |
| 15 | Entity | 0x0 | team@tag (target·champ) | r | 46140/46143 TeamType eq(entity.rs:1127 derive) · 46170 champ Neutral 판정 | 4 | OK |  |
| 16 | Entity | 0x8 | team@Player.0 (target·champ) | r | 46141/46144 · 46191~46192 team 번호 비교 · 46174 visible_state 인덱스 | 4 | OK |  |
| 17 | Entity | 0x68 | ty@tag (target) | r | 46155/46198 ==1 Minion (entity.rs:1261 is_minion 인라인) | 4 | OK |  |
| 18 | Entity | 0x38 | visible_state[champ.team]@tag (target) | r | 46205 + team*24 · ==0 Visible → unseen=false(46208~46210, dbg DW_OP_not) | 4 | OK |  |
| 19 | Entity | 0x660 | x (target·champ) | r | 46301 target(폴백 목표) / 46472·46504 champ(경로 시작점) | 4 | OK |  |
| 20 | Entity | 0x668 | y (target·champ) | r | 46306 / 46474·46506 | 4 | OK |  |
| 21 | SmallActionLaneMinionPosition(self) | 0x10 | goal_x | w | 46371 — 옛 목표와 거리² > 144000000 일 때만(L586) | 4 | OK | 새 목표 x (chosen 또는 폴백) |
| 22 | SmallActionLaneMinionPosition(self) | 0x18 | goal_y | w | 46372 — 같은 조건(L587) | 4 | OK | 새 목표 y |
| 23 | SmallActionLaneMinionPosition(self) | 0x20 | goal_score | w | 46374(먼 목표 경로, L588) / 46435(가까운 목표 경로, L591) — 두 경로 모두 항상 갱신 | 4 | OK | 새 score |
| 24 | SmallActionLaneMinionPosition(self) | 0x75 | path_finder@tag | w | 46454 — 먼 목표 경로에서 옛 PathFinder drop(Box 2개 dealloc 46408·46430) 후 None 으로(L589) | 4 | OK | 2 (None) |
| 25 | SmallActionLaneMinionPosition(self) | 0x30..0x78 | path_finder (Option<PathFinder> 72B) | w | 46491 (self+48=0x30) — path_finder 가 None 일 때(L597~598). 이후 update_path(46520)·PathFinder::get_input(46537) 이 &mut 로 내부 갱신(콜리 소관) | 4 | OK | PathFinder::new_target(...) 결과 memcpy 72B |
| 26 | SmallActionLaneMinionPosition(self) | 0x0 / 0x8 / 0x28 | start_tick / target / end_delay | w | 읽기만(target) 또는 미접근 | 4 | OK | 변경 없음 |
| 27 | rnd / debug | - | - | w | &mut 로 콜리에 전달만 | 4 | 확인불가(오프셋 파싱 실패) | 본 함수 직접 쓰기 없음 |
| 28 | sret | 0x0 | Option<Input> tag | w | 46112·46150·46165·46241·46251(L562)·46283·46348·46527. Some 은 콜리 sret | 4 | 확인불가(tcx 사전에 타입 없음) | -1 (None) |

**`consts` 상수 10건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | -1 | 541 | 태그 | Option<Input> None 태그(i64, DWARF DISCR_EXACT -1) · 46112 등 8곳 | 3 |
| 1 | 1 | 543 | 태그 | EntityType 태그 1 = Minion(46157·46200) / L557 Position 태그 1 = Jungle(46352·switch 46217) / L631 SafeMoveWithSkill 태그 1 = Safe(46537 i8 1) | 4 |
| 2 | 0 | 543 | 태그 | TeamType 태그 0 = Player(46161) / VisibleState 0 = Visible(46208) | 4 |
| 3 | 2 | 553 | 센티널 | version < 2 게이트(46226): v1 은 목표 미가시 시 즉시 None. 또한 Option<PathFinder> None 니치 태그 2(46379·46438·46454·46495) | 4 |
| 4 | 1987654321 | 553 | 센티널 | goal_x/goal_y 미설정 센티널(new 에서 넣은 값) · 46229·46233·46289·46292 | 4 |
| 5 | 144000000 | 585 | 임계 | 12000^2 — 옛 목표↔새 목표 거리(제곱)가 이보다 크면 목표 좌표·경로 재생성, 이하면 score 만 갱신하고 옛 목표·경로 유지 · 46344 | 4 |
| 6 | 20 | 598 | 길이 | PathFinder::new_target 의 key="lane_minion_position"(anon.42, 20B) 문자열 길이(팻포인터 len) · 46488 | 4 |
| 7 | 5 | 627 | 미상 | SmallActionRunAway::new_with_skill(data, player, 5, true) 의 3번째 인자(용도는 그 콜리 소관 — 미확인) · 46544 | 4 |
| 8 | 1120 | 589 | 미상 | path Box<[(u64,u64);70]> 크기 = 70*16 (drop 인라인) · 46408 | 4 |
| 9 | 70 | 589 | 길이 | planned_verdict Box<[u8;70]> 크기 / 경로 최대 길이 70 · 46430 | 4 |

**`knobs` 조정점 3건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 목표 재생성 거리 임계(제곱) | lane_minion.rs:585 (IR 46344) | 144000000 | 12000(0.375셀). 올리면 목표가 더 많이 움직여도 옛 경로를 유지(재탐색 빈도↓, 추종 정확도↓). 내리면 미세 이동에도 path_finder 를 새로 만든다(비용↑) | 4 | 기존 |
| 1 | 미가시 목표 허용 버전 게이트 | lane_minion.rs:553 (IR 46226) | 2 | version>=2 이면 목표가 안 보여도 기존 goal 좌표로 계속 이동. 1 이하는 즉시 None(소액션 종료) | 4 | 기존 |
| 2 | PathFinder::get_input 의 스킬 이동 모드 | lane_minion.rs:631 (IR 46537 i8 1) | 1 | SafeMoveWithSkill::Safe. Must(2)/MustWithUlt(3) 로 바꾸면 이동 스킬 사용 강제(의미는 path 계층 소관) | 4 | 기존 |

<details><summary>`callees` 피호출자 14건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | choose_goal | game_ai::SmallActionLaneMinionPosition::choose_goal | in:game_ai | fn(&game_ai::SmallActionLaneMinionPosition, usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, &game_core::Entity, &game_core::PositioningScoreData, game_core::LineType) -> std::option::Option<(u64, u64, i64)> | game-ai\src\small_action\lane_minion.rs:493 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | drop | <game_core::prof::ProfTimer as std::ops::Drop>::drop | pub | fn(&mut game_core::prof::ProfTimer) | game-core\src\simulation\prof.rs:184 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 2 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 3 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 4 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 5 | get_input | game_ai::PathFinder::get_input | pub | fn(&mut game_ai::PathFinder, &game_core::PlayerState, &game_core::OperationData, game_ai::SafeMoveWithSkill, bool) -> game_core::Input | game-ai\src\path_finder.rs:856 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 6 | get_input | game_ai::SmallActionRunAway::get_input | in:game_ai | fn(&mut game_ai::SmallActionRunAway, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &mut game_core::DebugFrameData) -> std::option::Option<game_core::Input> | game-ai\src\small_action\move_actions.rs:95 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 7 | is_minion | game_core::EntityType::is_minion | pub | fn(&game_core::EntityType, game_core::LineType) -> bool | game-core\src\simulation\entity.rs:1256 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 8 | is_safe_recall | game_ai::small_action::cast::is_safe_recall | in:game_ai | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData) -> bool | game-ai\src\small_action\cast.rs:304 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 9 | new_target | game_ai::PathFinder::new_target | pub | fn(&mut rand::rngs::std::StdRng, &game_core::GameContext, usize, &str, u64, u64, u64, u64, impl Fn(usize, usize, usize, usize) -> PathVerdict/#0) -> game_ai::PathFinder | game-ai\src\path_finder.rs:66 | True | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 10 | new_unnecessary_tower_avoid | game_ai::TowerDodgeContext::new_unnecessary_tower_avoid | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, u64, u64) -> game_ai::TowerDodgeContext | game-ai\src\path_finder.rs:1220 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 11 | new_with_skill | game_ai::SmallActionRunAway::new_with_skill | pub | fn(&game_core::OperationData, &game_core::PlayerState, usize, bool) -> game_ai::SmallActionRunAway | game-ai\src\small_action\move_actions.rs:29 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 12 | path_needs_tower_escape | game_ai::small_action::path_needs_tower_escape | in:game_ai::small_action | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_ai::PathFinder) -> bool | game-ai\src\small_action.rs:109 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 13 | update_path | game_ai::PathFinder::update_path | pub | fn(&mut game_ai::PathFinder, &mut rand::rngs::std::StdRng, &game_core::GameContext, u64, u64, u64, u64, impl Fn(usize, usize, usize, usize) -> PathVerdict/#0) | game-ai\src\path_finder.rs:631 | True | False | 3 | IR 호출 심볼 일치(토큰 경계) |
</details>

⚠**미매칭 2개**: `__rust_dealloc`, `dist_sq`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m11.ll:43062) · **형제 20개** (SmallActionLaneMinionPosition)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::SmallActionLaneMinionPosition as std::clone::Clone>::clone | pub | game-ai\src\small_action\lane_minion.rs:8 | True | fn(&game_ai::SmallActionLaneMinionPosition) -> game_ai::SmallActionLaneMinionPosition |
| 1 | <game_ai::SmallActionLaneMinionPosition as std::fmt::Debug>::fmt | pub | game-ai\src\small_action\lane_minion.rs:8 | True | fn(&game_ai::SmallActionLaneMinionPosition, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 2 | game_ai::SmallActionLaneMinionPosition::new | pub | game-ai\src\small_action\lane_minion.rs:139 | False | fn(&game_core::OperationData, usize, usize, i64, game_ai::PositionEvalPurpose) -> game_ai::SmallActionLaneMinionPosition |
| 3 | game_ai::SmallActionLaneMinionPosition::should_suppress_direct_minion_attack | pub | game-ai\src\small_action\lane_minion.rs:152 | False | fn(usize, &game_core::OperationData, &game_core::PlayerState, game_core::LineType, &game_core::Entity) -> bool |
| 4 | game_ai::SmallActionLaneMinionPosition::has_current_explicit_minion_action | in:game_ai | game-ai\src\small_action\lane_minion.rs:187 | False | fn(usize, &game_core::OperationData, &game_core::PlayerState, game_core::LineType, &game_core::Entity) -> bool |
| 5 | game_ai::SmallActionLaneMinionPosition::can_use_minion_action_now | in:game_ai | game-ai\src\small_action\lane_minion.rs:208 | False | fn(bool, std::option::Option<&game_core::Effect>, &game_core::Entity, &game_core::Entity) -> bool |
| 6 | game_ai::SmallActionLaneMinionPosition::direct_attack_stance | in:game_ai | game-ai\src\small_action\lane_minion.rs:212 | False | fn(&game_core::OperationData, &game_core::Entity, &game_core::Entity, u64) -> std::option::Option<(u64, u64)> |
| 7 | game_ai::SmallActionLaneMinionPosition::has_safe_minion_attack_stance | in:game_ai | game-ai\src\small_action\lane_minion.rs:226 | False | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, &game_core::Entity, game_core::LineType, u64) -> bool |
| 8 | game_ai::SmallActionLaneMinionPosition::is_safe_minion_attack_stance | in:game_ai | game-ai\src\small_action\lane_minion.rs:272 | False | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, &game_core::Entity, game_core::LineType, u64, u64, u64) -> bool |
| 9 | game_ai::SmallActionLaneMinionPosition::target_score | in:game_ai | game-ai\src\small_action\lane_minion.rs:294 | False | fn(usize, &game_core::OperationData, &game_core::PlayerState, &game_core::Entity, &game_core::Effect, &game_core::Entity, std::option::Option<&game_ai::MinionWaveSnapshot>, &mut rand::rngs::std::StdRng) -> std::option::Option<i64> |
| 10 | game_ai::SmallActionLaneMinionPosition::attack_range | in:game_ai | game-ai\src\small_action\lane_minion.rs:354 | False | fn(&game_core::Entity, &game_core::Entity) -> std::option::Option<u64> |
| 11 | game_ai::SmallActionLaneMinionPosition::local_position_score | in:game_ai | game-ai\src\small_action\lane_minion.rs:359 | False | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, u64, u64, game_ai::PositionEvalPurpose) -> i64 |
| 12 | game_ai::SmallActionLaneMinionPosition::lane_stance_risk | in:game_ai | game-ai\src\small_action\lane_minion.rs:380 | False | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, u64, u64) -> std::option::Option<i64> |
| 13 | game_ai::SmallActionLaneMinionPosition::push_candidate | in:game_ai | game-ai\src\small_action\lane_minion.rs:439 | False | fn(&mut bumpalo::collections::vec::Vec< (u64, u64, i64)>, usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, &game_core::Entity, &game_core::PositioningScoreData, u64, u64, game_ai::PositionEvalPurpose, u64, u64, i64, game_core::LineType, i64) |
| 14 | game_ai::SmallActionLaneMinionPosition::choose_goal | in:game_ai | game-ai\src\small_action\lane_minion.rs:493 | False | fn(&game_ai::SmallActionLaneMinionPosition, usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, &game_core::Entity, &game_core::PositioningScoreData, game_core::LineType) -> std::option::Option<(u64, u64, i64)> |
| 15 | game_ai::SmallActionLaneMinionPosition::get_input | in:game_ai | game-ai\src\small_action\lane_minion.rs:540 | False | fn(&mut game_ai::SmallActionLaneMinionPosition, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &mut game_core::DebugFrameData) -> std::option::Option<game_core::Input> |
| 16 | game_ai::SmallActionLaneMinionPosition::merge | in:game_ai | game-ai\src\small_action\lane_minion.rs:635 | False | fn(&mut game_ai::SmallActionLaneMinionPosition, usize, game_ai::SmallActionLaneMinionPosition) |
| 17 | game_ai::SmallActionLaneMinionPosition::get_action | in:game_ai | game-ai\src\small_action\lane_minion.rs:652 | True | fn(&game_ai::SmallActionLaneMinionPosition) -> game_core::SmallAction |
| 18 | game_ai::SmallActionLaneMinionPosition::is_end | in:game_ai | game-ai\src\small_action\lane_minion.rs:656 | False | fn(&game_ai::SmallActionLaneMinionPosition, &mut rand::rngs::std::StdRng, usize, &game_core::PlayerState, &game_core::OperationData) -> bool |
| 19 | game_ai::SmallActionLaneMinionPosition::near_move_complete | in:game_ai | game-ai\src\small_action\lane_minion.rs:685 | False | fn(&game_ai::SmallActionLaneMinionPosition, &game_core::Entity) -> bool |

**`open` 10건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | closure$0(new_target 비용/위험 콜백)·closure$s_0(update_path 콜백) 의 본문 — 별도 define 없이 PathFinder 제네릭 인스턴스(m03.ll:112~3631 / 24887~25960, m08.ll 등) 안에 인라인. 캡처 = {&version, player, data, positioning_score, &tower_dodge(416B), champ}(46476~46486/46508~46518). 경로 계층이라 미탐색(지시 범위 밖) | 4 |  |
| 1 | 미탐색 | PathFinder::new_target 계약(m03.ll:112): (sret Option<PathFinder> 72B, rnd:&mut StdRng, ctx:&GameContext, version, key:&str("lane_minion_position"), from_x, from_y, to_x, to_y, closure) → 태그 +0x45(=self+0x75) 2=None. 내부 미탐색 | 4 |  |
| 2 | 미탐색 | PathFinder::update_path 계약: (&mut PathFinder, rnd, ctx, from_x, from_y, to_x, to_y, closure) → void. 내부 미탐색 | 4 |  |
| 3 | 미탐색 | PathFinder::get_input 계약(m03.ll:137276, path_finder.rs:856): (sret Option<Input>, &mut PathFinder, &PlayerState, &OperationData, SafeMoveWithSkill:i8, allow_recall:bool) — 마지막 bool 이 is_safe_recall 결과. 반환 바이트 미확인 | 4 |  |
| 4 | 미탐색 | TowerDodgeContext::new_unnecessary_tower_avoid 계약(m03.ll:1397): (sret 416B, version, player, data, goal_x, goal_y). 내부 미탐색 | 4 |  |
| 5 | 미탐색 | path_needs_tower_escape 계약(m11.ll:52695): (version, player, data, &PathFinder) -> bool(true=RunAway 로 전환). r14 중간 함수 — 내부 안 팜 | 4 |  |
| 6 | 미탐색 | is_safe_recall(잎 22, m07.ll:58406) 계약: (version, rnd:&mut StdRng[readnone], player, data, positioning_score) -> bool → PathFinder::get_input 의 allow_recall 인자 | 4 |  |
| 7 | 미탐색 | SmallActionRunAway::new_with_skill(m08.ll:92086) 계약: (sret 136B initializes (0,56),(125,126),(128,132), data, player, i64 5, bool true) / RunAway::get_input(m08.ll:106076): (sret Option<Input>, &mut self 136B, version, rnd, player, data, positioning_score, debug). 상수 5·true 의 의미 미확인 | 4 |  |
| 8 | 미탐색 | L543 한 줄 안 `team==` 과 `!is_minion` 의 소스 순서(column 부재) — 외연 동일 | 4 |  |
| 9 | 미탐색 | choose_goal 내부(43376~43887, r14 중간) — 계약은 lane_minion_position_action 명세와 동일 | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

