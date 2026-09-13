---

### `106` TeamPlan::v25_objective_posture — 에픽/세르펜 오브젝트 태세 판정 — 근접/캠프 아군·적 수, 적 압력, 체력창을 재서 Commit/HoldCamp/Screen/WaitGroup/SoftDisengage 중 하나(또는 None)를 낸다

| 항목 | 값 |
|---|---|
| id | `objective_discipline__TeamPlan_v25_objective_posture` |
| 심볼 | `_RNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy9team_plan20objective_disciplineNtB4_8TeamPlan21v25_objective_posture` |
| 소스 | `game-ai\src\plan_legacy\team_plan\objective_discipline.rs:207` |
| IR | `m09.ll` 15168~19005행 |
| 경로·가시성 | `game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v25_objective_posture` · **pub** |
| 계층 | 기타 |
| exe | `dd26e0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r11` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::PlayerState, &game_core::OperationData, game_core::JungleType) -> std::option::Option<game_ai::plan_legacy::team_plan::ObjectivePosture>
```

<details><summary>인자 6개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | *mut Option<ObjectivePosture>(88B) | sret · None 은 +0x0 에 i64 -1 저장(니치, IR L15209/15273/18999) · Some 은 9필드 전부 채움(L303) | 4 |
| 1 | 1 | self | &TeamPlan(1064B) | IR 속성 nonnull readonly — 쓰기 없음. 읽는 필드 = objective 태그·phase, vision.last_visible_pos[p]·last_checked_ticks[p] | 4 |
| 2 | 2 | version | usize | 본문 분기 없음. 클로저 캡처용 alloca %12 에 저장(L15179) 후 is_ignored_well_enemy(version, player, x) 에 그대로 전달 | 4 |
| 3 | 3 | player | &PlayerState(2528B) | noalias readonly. info.team(0x930)·info.position(0x9c0) 읽고 is_ignored_well_enemy/is_recent_visible/far_split_pressure 에 전달 | 4 |
| 4 | 4 | data | &OperationData(24B) | noalias readonly. cache(+0)·context(+8)·blackboard(+0x10) | 4 |
| 5 | 5 | target | JungleType(i8, range 0..6) | Morgard=4 / Serpen=5 만 유효(L210). camp_pos·wait_pos 선택, side 판정(is_top_side/is_bottom_side), 결과 target 필드에 그대로 저장 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
// objective_discipline.rs:207~214 — 진입 게이트
// self.objective(+0x41f 태그, +0x420 phase) 를 target 과 대조
match (target, self.objective) {
  (Morgard(4), Some(Morgard{phase,..}(tag0))) | (Serpen(5), Some(Serpen{phase,..}(tag1))) => phase = *(self+0x420)   // L210~212
  _ => return None                                                                                      // L213 (sret+0 = -1)
}

// L216 — 내 챔프
team = player.info.team (bounds <2, 아니면 panic_bounds_check)
champ = data.cache.player_champion[team][player.info.position as usize]?   // null 이면 return None (from_residual)

// L217~220 — 좌표
camp_pos = map.camp_pos(target, team == 0)                       // MapDef::camp_pos(&self, JungleType, bool) — bool 은 (team==0)
wait_pos = if target == Morgard(4) { map.camp_pos(Stump(2), team==0) }   // L219
           else                    { map.camp_pos(Rhino(0), team==0) }   // L220

// L224~228 — 오브젝트 엔티티 (GameMode::as_moba 인라인 game.rs:231, Moba 가 아니면 unwrap_failed 패닉)
moba = game.get_game_mode()[vtable+0x40].as_moba().unwrap()
objective_entity = if target == Morgard { moba.jungle_runner.epic.live_list.get(0) }     // L225  (+0x1a8 len, +0x1a0 ptr)
                   else                 { moba.jungle_runner.serpen.live_list.get(0) }   // L227  (+0x1d8 len, +0x1d0 ptr)
                   .and_then(|id| game.get_entity_by_id(*id)[vtable+0x1f0])            // closure#0 L226 / closure#1 L228 — null=None

// L231~232, L283
object_hp_ratio = objective_entity.map_or(100, |t| t.hp*100 / t.stat_cached.hp)   // closure#2 (분모 0 → div_by_zero 패닉)
execute_window      = object_hp_ratio < 26    // L232
objective_untouched = object_hp_ratio > 84    // L283 (엔티티 없으면 100 → true)

// L234~235 — near_ally_count  (allies = cache.iter_champions(team): player_champion[team][0..5] 중 Some, ★자기 자신 포함)
near_ally_count = allies.filter(|x| x.hp*100/x.stat_cached.hp > 39            // L234
                               && distance_sq(x, champ) < 140000²+1).count()   // L235

// L236~238 — near_enemy_count (enemies = iter_champions(1-team))  ※hp 필터 없음
near_enemy_count = enemies.filter(|x| x.is_visible_from(champ)                 // L236: champ.team 이 Neutral 이면 true, Player(t) 면 x.visible_state[t]==Visible(0)
                                   && !is_ignored_well_enemy(version, player, x)  // L237 (call)
                                   && distance_sq(x, champ) < 170000²+1).count() // L238

// L240~241 — camp_ally_count
camp_ally_count = allies.filter(|x| hp% > 39                                   // L240
                               && distance_sq(x, camp_pos) < 180000²+1).count() // L241

// L242~245 — camp_visible_enemy_count
camp_visible_enemy_count = enemies.filter(|x| hp% > 39                          // L242
                                           && x.is_visible_from(champ)          // L243
                                           && !is_ignored_well_enemy(version, player, x)  // L244
                                           && distance_sq(x, camp_pos) < 190000²+1).count()  // L245

// L246~258 — camp_possible_enemy_count (closure#7 247:19 / closure#8 248:15 = |(p, x)| {...}, 합산 루트 L258)
camp_possible_enemy_count = enemies.enumerate().filter(|(p, x)| {
    x.hp*100/x.stat_cached.hp >= 50                                             // L249 (`ult 50` 탈락)
    && !data.blackboard[1 - team].is_recent_visible(game, player, x)            // ★적 팀 보드. is_recent_visible = 지금 보임 || last_visible[pos]+120 >= tick (blackboard.rs:348~350)
    && { last_pos = self.vision.last_visible_pos[p];                            // TeamPlan+0x230+16p
         d = utils::distance(last_pos, camp_pos).saturating_sub(180000);
         elapsed = game.tick()[vtable+0x28].saturating_sub(self.vision.last_checked_ticks[p]);   // +0x2d0+8p
         elapsed * x.stat_cached.move_speed >= d }                              // 마지막 목격 위치에서 지금까지 캠프 180000 반경에 닿을 수 있었나
}).count()

// L259
camp_enemy_pressure = camp_visible_enemy_count + camp_possible_enemy_count

// L261~267 — side_ally_count (closure#9 261:78)
side_ally_count = allies.filter(|x| hp% > 39                                    // L262
    && match target { Morgard => is_top_side(ctx,x.x,x.y)    || is_near_mid_line(ctx,x.x,x.y),   // L263 (is_top_side 인라인: height-y >= x → true 면 근처 판정 생략)
                      _(Serpen) => is_bottom_side(ctx,x.x,x.y) || is_near_mid_line(ctx,x.x,x.y) } // L264 (is_bottom_side: height-y <= x)
).count()

// L269~274 — focus_enemy (closure#10 269:78 필터 · closure#11 273:19 키 · closure#12 274:12 map)
focus_enemy = enemies.filter(|x| x.is_visible_from(champ)                       // L269
                              && !is_ignored_well_enemy(version, player, x)      // L270
                              && distance_sq(x, champ) < 180000²+1             // L271
                              && distance_sq(x, camp_pos) < 230000²+1)         // L272
              .min_by_key(|x| distance_sq(x, champ).saturating_add(distance_sq(x, camp_pos) >> 2))   // L273 (/4)
              .map(|e| e.id)                                                    // L274 (Entity+0x5c0)
// 구현: 첫 통과 원소를 call_mut 심(aux 42086)으로 찾고(최대 5회), 키를 계산한 뒤 나머지를 fold(aux 13515)로 min_by

// L276~283 — 판정 재료
far_split_pressure = v25_objective_far_split_pressure(player, data, target)     // L276 (call, bool)
split_screen_ready = camp_ally_count >= 2 && far_split_pressure && camp_enemy_pressure <= camp_ally_count + 2   // L277
champ_camp_dist_sq = distance_sq(champ, camp_pos)                               // L278
approach_camp      = champ_camp_dist_sq > 150000²                               // L279
isolated_danger    = near_enemy_count >= 2 && approach_camp && near_ally_count < 2   // L280
camp_overloaded    = !(camp_visible_enemy_count < camp_ally_count + 2 || camp_visible_enemy_count < 3)   // L281~282 (IR 는 부정형 %1497 로 보유; 이름 극성은 사용처(L285)에서 역산)
low_hp_contact     = approach_camp && champ.hp*100/champ.stat_cached.hp < 35 && near_enemy_count != 0   // L282 (approach_camp 거짓이면 hp 계산 안 함; dbg DW_OP_not 로 %1514 부정)

// L285~300 — kind 결정 (else-if 사슬, 분기 순서 = IR 블록 순서)
kind =
  if !execute_window && (low_hp_contact || (isolated_danger && camp_overloaded))        // L285  → SoftDisengage(4)
     [IR: %1513 경로 = !%108 && low_hp_contact / %1498 경로 = !%108 && !%1497 && %1494]
  else if !execute_window && champ_camp_dist_sq > 180000² && camp_ally_count < 2 && near_ally_count < 2 && near_enemy_count != 0   // L287 → WaitGroup(3)
  else if objective_untouched && approach_camp && camp_enemy_pressure <= camp_ally_count + 1 && focus_enemy.is_some() && !isolated_danger   // L290
          && (camp_ally_count >= 2 || near_ally_count >= 2 || side_ally_count >= 4 || split_screen_ready)   // L292 → Screen(2)
  else if execute_window && camp_ally_count >= 2 && camp_enemy_pressure <= camp_ally_count + 1              // L294 → Commit(0)  (L290 참·L292 거짓이면 objective_untouched 라 여기 자동 거짓 → L296 으로)
  else match phase {                                                                                        // L296 (self+0x420)
         Setup(1) | Hunt(3) if camp_ally_count >= 2 && camp_enemy_pressure <= camp_ally_count + 1 => HoldCamp(1)   // L297
         _ => return None                                                                                   // L300 (sret+0 = -1)
       };

// L303 — Some(ObjectivePosture{ focus_enemy, camp_pos, wait_pos, near_ally_count, near_enemy_count, camp_ally_count, camp_enemy_pressure, kind, target })
// L314 ret
```

**`mem` 메모리 접근 40건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | TeamPlan | 0x41f | objective | r | L210 Option<MainObjective> 태그. target=4(Morgard)면 tag==0(Morgard), target=5(Serpen)면 tag==1(Serpen) 이어야 진행, 아니면 None(L213) | 4 | OK |  |
| 1 | TeamPlan | 0x420 | objective@Some.0@Morgard.phase / Serpen.phase | r | L210 ObjectPhase(1B). L296 switch: 1=Setup / 3=Hunt 만 HoldCamp 후보, 그 외(None=0/Assemble=2)는 return None | 4 | OK |  |
| 2 | TeamPlan | 0x230 | vision.last_visible_pos[p].0/.1 | r | L258(closure#8 본문) 적 슬롯 p(0..4) 별 (u64,u64), stride 16 — 본문에 560,568,576,584,…,632 로 전개 | 4 | OK |  |
| 3 | TeamPlan | 0x2d0 | vision.last_checked_ticks[i] | r | L258 적 슬롯 p 별 usize, stride 8 — 본문 720,728,…,752 | 4 | OK |  |
| 4 | PlayerState | 0x930 | info.team | r | L216. bounds check <2 (panic_bounds_check). 1-team = 적 팀 인덱스(L236/242/258/269) | 4 | OK |  |
| 5 | PlayerState | 0x9c0 | info.position@tag | r | L216 i32 로드→zext = as_index(player.rs:581 인라인) → player_champion[team][pos] | 4 | OK |  |
| 6 | OperationData | 0x0 | cache | r | &AbstractGameWithCache | 4 | OK |  |
| 7 | OperationData | 0x8 | context | r | &GameContext (L217 map, L263/264 setting.height) | 4 | OK |  |
| 8 | OperationData | 0x10 | blackboard | r | &[Blackboard;2] — L258 에서 blackboard[1-team](적 팀 보드, stride 744) 을 is_recent_visible 의 self 로 전달 | 4 | OK |  |
| 9 | AbstractGameWithCache | 0x0 | game.data_ptr | r | &dyn AbstractGame 데이터 포인터 | 4 | OK |  |
| 10 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r | vtable 슬롯 0x28 tick / 0x40 get_game_mode / 0x1f0 get_entity_by_id 를 간접호출(divtable AbstractGame 98%) | 3 | OK |  |
| 11 | AbstractGameWithCache | 0x1e0 | player_champion | r | L216 champ(Option<&Entity>, null=None → return None). [team] 5칸 슬라이스가 L234/240/261 아군 iter_champions, [1-team] 이 L236/242/258/269 적 iter_champions | 4 | OK |  |
| 12 | GameContext | 0x8 | setting | r | &GameSetting | 4 | OK |  |
| 13 | GameContext | 0x20 | map | r | &MapDef → camp_pos 호출 self | 4 | OK |  |
| 14 | GameSetting | 0x12c0 | height | r | L263/264 is_top_side/is_bottom_side 인라인(map_regions.rs:22/28): ry = height - y | 4 | OK |  |
| 15 | MobaMode | 0x1a8 | jungle_runner.epic.live_list.len | r | L225 (target=Morgard) get(0): len==0 → objective_entity=None | 4 | OK |  |
| 16 | MobaMode | 0x1a0 | jungle_runner.epic.live_list.buf.inner.ptr | r | L225 live_list[0] = 엔티티 id(usize) → get_entity_by_id | 4 | OK |  |
| 17 | MobaMode | 0x1d8 | jungle_runner.serpen.live_list.len | r | L227 (target=Serpen) 동일 패턴 | 4 | OK |  |
| 18 | MobaMode | 0x1d0 | jungle_runner.serpen.live_list.buf.inner.ptr | r | L227 | 4 | OK |  |
| 19 | Entity | 0x0 | team@tag | r | champ 의 TeamType 태그(0=Player/1=Neutral) — is_visible_from(champ) 인라인(entity.rs:1482 player_team). Neutral 이면 가시성 검사 생략(=보임) | 4 | OK |  |
| 20 | Entity | 0x8 | team@Player.0 | r | champ 팀 usize → x.visible_state[champ_team] 인덱스(bounds <2) | 4 | OK |  |
| 21 | Entity | 0x38 | visible_state | r | stride 24({i64,[2 x i64]}). ==0(Visible) 만 통과 — L236/243/269 is_visible_from | 4 | OK |  |
| 22 | Entity | 0x5c0 | id | r | L274 focus_enemy = Some(enemy.id) | 4 | OK |  |
| 23 | Entity | 0x628 | stat_cached.hp | r | hp% 분모(==0 이면 panic_const_div_by_zero) — L231/234/240/242/249/262/282 | 4 | OK |  |
| 24 | Entity | 0x640 | stat_cached.move_speed | r | L258 closure#8: elapsed * move_speed >= d | 4 | OK |  |
| 25 | Entity | 0x660 | x | r | distance_sq(entity.rs:2158 인라인) / is_top_side | 4 | OK |  |
| 26 | Entity | 0x668 | y | r |  | 4 | OK |  |
| 27 | Entity | 0x670 | hp | r | hp*100/stat_cached.hp | 4 | OK |  |
| 28 | Option<ObjectivePosture>(sret) | 0x0 | focus_enemy@tag / None 니치 | w | ★self(TeamPlan) 쓰기 0건 — &self readonly. 힙 할당/드롭 0건(alloca 만) | 4 | 확인불가(tcx 사전에 타입 없음) | None 경로: i64 -1 (L213/L216/L300) · Some 경로: focus_enemy 태그(1=Some, 0=None) |
| 29 | Option<ObjectivePosture>(sret) | 0x8 | focus_enemy@Some.0 | w |  | 4 | 확인불가(tcx 사전에 타입 없음) | enemy.id (L274) / Some 아니면 undef |
| 30 | Option<ObjectivePosture>(sret) | 0x10 | camp_pos.0 | w | L303 | 4 | 확인불가(tcx 사전에 타입 없음) | MapDef::camp_pos(map, target, team==0).0 |
| 31 | Option<ObjectivePosture>(sret) | 0x18 | camp_pos.1 | w |  | 4 | 확인불가(tcx 사전에 타입 없음) | …​.1 |
| 32 | Option<ObjectivePosture>(sret) | 0x20 | wait_pos.0 | w | L218~220 | 4 | 확인불가(tcx 사전에 타입 없음) | target==Morgard ? camp_pos(map, Stump(2), team==0) : camp_pos(map, Rhino(0), team==0) |
| 33 | Option<ObjectivePosture>(sret) | 0x28 | wait_pos.1 | w |  | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 34 | Option<ObjectivePosture>(sret) | 0x30 | near_ally_count | w |  | 4 | 확인불가(tcx 사전에 타입 없음) | %289 (L234~235 count) |
| 35 | Option<ObjectivePosture>(sret) | 0x38 | near_enemy_count | w |  | 4 | 확인불가(tcx 사전에 타입 없음) | %496 (L236~238 count) |
| 36 | Option<ObjectivePosture>(sret) | 0x40 | camp_ally_count | w |  | 4 | 확인불가(tcx 사전에 타입 없음) | %669 (L240~241 count) |
| 37 | Option<ObjectivePosture>(sret) | 0x48 | camp_enemy_pressure | w |  | 4 | 확인불가(tcx 사전에 타입 없음) | camp_visible_enemy_count + camp_possible_enemy_count (L259) |
| 38 | Option<ObjectivePosture>(sret) | 0x50 | kind | w | tcxdict --enum ObjectivePostureKind: 메모리태그==idx | 3 | 확인불가(tcx 사전에 타입 없음) | ObjectivePostureKind 태그 0=Commit 1=HoldCamp 2=Screen 3=WaitGroup 4=SoftDisengage (phi %1526) |
| 39 | Option<ObjectivePosture>(sret) | 0x51 | target | w |  | 4 | 확인불가(tcx 사전에 타입 없음) | 인자 target 그대로(i8 %5) |

**`consts` 상수 22건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev | folded_from |
|---|---|---|---|---|---|---|
| 0 | 4 | 210 | 태그 | JungleType::Morgard 메모리태그(tcxdict --enum JungleType: idx==tag). switch i8 %5 case 4 → self.objective tag 0(Morgard) 요구. L218/L267 에서도 `target==4` 로 Morgard 분기 | 3 |  |
| 1 | 5 | 210 | 태그 | JungleType::Serpen 태그. case 5 → self.objective tag 1(Serpen) 요구. 그 외 target → None | 4 |  |
| 2 | -1 | 213 | 센티널 | Option<ObjectivePosture>::None 니치 값(sret+0 i64). L213(objective 불일치)·L216(champ 없음, `?`)·L300(태세 결정 실패) 세 곳 | 4 |  |
| 3 | 2 | 219 | 태그 | JungleType::Stump 태그 — Morgard 일 때 wait_pos = camp_pos(map, Stump, team==0). (같은 리터럴 2 가 `>=2`/`<2` 인원 비교(L277/280/287/292/297)·bounds check(팀<2)·blackboard 인덱스에도 쓰임) | 4 |  |
| 4 | 0 | 220 | 태그 | JungleType::Rhino 태그 — Serpen 일 때 wait_pos = camp_pos(map, Rhino, team==0). (MainObjective::Morgard 태그 0 · VisibleState::Visible 태그 0 · TeamType::Player 태그 0 도 리터럴 0) | 4 |  |
| 5 | 100 | 231 | 계수 | hp 백분율 계수: hp*100/stat_cached.hp (L231/234/240/242/249/262/282 전부 동일식). map_or 기본값 100 도 같은 리터럴(objective_entity 없으면 object_hp_ratio=100) | 4 |  |
| 6 | 26 | 232 | 임계 | execute_window = object_hp_ratio < 26 (`icmp ult %101, 26`) — 오브젝트 체력 26% 미만 = 막타(처형) 창 | 4 |  |
| 7 | 84 | 283 | 임계 | objective_untouched = object_hp_ratio > 84 (`icmp ugt %101, 84`; 엔티티 없으면 true). L290 Screen 전제 | 4 |  |
| 8 | 39 | 234 | 임계 | '건강한' 챔프 기준 hp% > 39 (`ugt`, 즉 ≥40%). L234 near_ally / L240 camp_ally / L242 camp_visible_enemy / L262 side_ally 네 필터 공통 | 4 |  |
| 9 | 19600000001 | 235 | 임계 | 140000²+1 — near_ally_count: distance_sq(x, champ) < 140000²+1 (= ≤140000, 4.375셀). 자기 자신 포함 | 4 |  |
| 10 | 28900000001 | 238 | 임계 | 170000²+1 — near_enemy_count: distance_sq(x, champ) ≤ 170000² | 4 |  |
| 11 | 32400000001 | 241 | 임계 | 180000²+1 — camp_ally_count: distance_sq(x, camp_pos) ≤ 180000². aux call_mut/fold(L271)에서는 focus 후보의 distance_sq(x, champ) ≤ 180000² | 4 |  |
| 12 | 36100000001 | 245 | 임계 | 190000²+1 — camp_visible_enemy_count: distance_sq(x, camp_pos) ≤ 190000² | 4 |  |
| 13 | 50 | 249 | 임계 | camp_possible_enemy 후보 조건 hp% ≥ 50 (`icmp ult %929, 50` 이면 탈락). exe 패닉 Location 249:12 로 줄 확정(IR 태그는 fold 루트 258) | 4 |  |
| 14 | 180000 | 258 | 계수 | d = distance(last_visible_pos[p], camp_pos).saturating_sub(180000) — 캠프 반경 180000 안쪽은 이동거리 0 취급. 조건 elapsed*move_speed >= d | 4 |  |
| 15 | 52900000001 | 272 | 미상 | 230000²+1 — focus 후보 필터: distance_sq(x, camp_pos) ≤ 230000² (aux call_mut L42198 / fold L13726) | 4 |  |
| 16 | 2 | 273 | 임계 | min_by_key 키 = distance_sq(x, champ).saturating_add(distance_sq(x, camp_pos) / 4). `/4` 가 `lshr i64 %1446, 2` 로 접힘(L18774 / fold L13800) | 4 | 4 |
| 17 | 22500000000 | 279 | 임계 | 150000² — approach_camp = champ_camp_dist_sq > 150000² (`ugt`, 즉 거리 > 150000) | 4 |  |
| 18 | 35 | 282 | 임계 | low_hp_contact 의 내 챔프 hp% < 35 (`icmp ult %1510, 35`). approach_camp 일 때만 평가 | 4 |  |
| 19 | 3 | 281 | 태그 | camp_overloaded 의 두 번째 절 camp_visible_enemy_count < 3 (`ult %909, 3`) 및 L292 side_ally_count > 3 (`ugt %1372, 3`, 즉 ≥4). 같은 리터럴 3 = ObjectPhase::Hunt 태그(L296 switch case 3) | 4 |  |
| 20 | 1 | 277 | 태그 | `ugt %669, 1` = camp_ally_count ≥ 2 (L277/290/294/297) · `ugt %1520, 1` = camp_ally\|near_ally ≥2 (L292) · camp_ally_count+1 (L290/294/297 압력 상한) · ObjectPhase::Setup 태그(L296 case 1) · focus_enemy Some 태그 | 4 |  |
| 21 | 32400000000 | 287 | 임계 | 180000² — WaitGroup 조건 champ_camp_dist_sq > 180000² (`ugt`) | 4 |  |

**`knobs` 조정점 16건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 막타(처형) 창 — 오브젝트 체력% | objective_discipline.rs:232 | 26 | 올리면 더 높은 체력에서부터 Commit(L294) 후보가 되고 SoftDisengage/WaitGroup(L285/287) 이 더 일찍 막힌다; 내리면 Commit 이 더 늦게 열린다 | 4 | 기존 |
| 1 | 오브젝트 '손 안 댐' 기준 체력% | objective_discipline.rs:283 | 84 | 내리면 Screen(L290) 이 오브젝트가 어느 정도 깎인 뒤에도 유지된다; 올리면 Screen 은 거의 만피일 때만 | 4 | 기존 |
| 2 | 건강한 챔프 hp% 하한(4개 카운트 공통) | objective_discipline.rs:234/240/242/262 | 39 | 올리면 near_ally/camp_ally/camp_visible_enemy/side_ally 가 모두 줄어 Screen·HoldCamp·Commit 조건이 어려워지고 WaitGroup 이 잦아진다 | 4 | 기존 |
| 3 | 근접 아군 반경 | objective_discipline.rs:235 | 19600000001 | 140000²+1. 올리면 near_ally_count 증가 → isolated_danger(L280)·WaitGroup(L287) 감소, Screen 의 near_ally>=2 절 충족 쉬움 | 4 | 기존 |
| 4 | 근접 적 반경 | objective_discipline.rs:238 | 28900000001 | 170000²+1. 올리면 near_enemy_count 증가 → isolated_danger·low_hp_contact·WaitGroup 조건이 더 자주 참 | 4 | 기존 |
| 5 | 캠프 아군 반경 | objective_discipline.rs:241 | 32400000001 | 180000²+1. 올리면 camp_ally_count 증가 → Commit/HoldCamp/Screen/split_screen_ready 문턱(>=2, 압력≤ally+1/+2) 완화 | 4 | 기존 |
| 6 | 캠프 가시 적 반경 | objective_discipline.rs:245 | 36100000001 | 190000²+1. 올리면 camp_visible_enemy_count·camp_enemy_pressure 증가 → camp_overloaded 참 쉬움, Commit/HoldCamp/Screen 어려움 | 4 | 기존 |
| 7 | '올 수 있는' 적 후보 hp% | objective_discipline.rs:249 | 50 | 내리면 저체력 적도 camp_possible_enemy 로 세어 압력이 커진다 | 4 | 기존 |
| 8 | '올 수 있는' 적 캠프 도달 여유 반경 | objective_discipline.rs:258 | 180000 | 올리면 마지막 목격 지점이 멀어도 도달 가능으로 세어 camp_possible_enemy_count(압력) 증가 | 4 | 기존 |
| 9 | focus 후보 캠프 반경 | objective_discipline.rs:272 | 52900000001 | 230000²+1. 내리면 focus_enemy 가 None 이 되기 쉬워 Screen(L290, is_some 요구) 이 막힌다 | 4 | 기존 |
| 10 | focus 키의 캠프거리 가중(1/4) | objective_discipline.rs:273 | 2 | lshr 2 = /4. 시프트를 줄이면(=/2) 캠프에 가까운 적을 더 우선해 고른다; 늘리면 내 챔프와의 거리 위주 | 4 | 기존 |
| 11 | 캠프 접근중 판정 거리 | objective_discipline.rs:279 | 22500000000 | 150000². 올리면 approach_camp 가 늦게 꺼져(더 가까이 가야 '도착') isolated_danger/low_hp_contact/Screen 의 approach 절이 오래 참 | 4 | 기존 |
| 12 | 저체력 접촉 hp% | objective_discipline.rs:282 | 35 | 올리면 low_hp_contact → SoftDisengage 가 더 높은 체력에서도 발동 | 4 | 기존 |
| 13 | WaitGroup 원거리 문턱 | objective_discipline.rs:287 | 32400000000 | 180000². 내리면 더 가까운 거리에서도 혼자(camp/near ally<2)+적 접촉 시 WaitGroup | 4 | 기존 |
| 14 | camp_overloaded 절대 하한(적 3명) | objective_discipline.rs:281 | 3 | 올리면 적이 더 많아야 overloaded → SoftDisengage(L285) 가 덜 발동 | 4 | 기존 |
| 15 | Screen 의 side_ally 절(>3) | objective_discipline.rs:292 | 3 | 내리면 같은 편 지역 아군이 적어도 Screen 허용 | 4 | 기존 |

<details><summary>`callees` 피호출자 25건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | as_moba | game_core::GameMode::<'a>::as_moba | pub | fn(game_core::GameMode<'a/#0>) -> std::option::Option<&'a/#0 game_core::MobaMode> | game-core\src\simulation\game.rs:230 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 1 | camp_pos | game_core::MapDef::camp_pos | pub | fn(&game_core::MapDef, game_core::JungleType, bool) -> (u64, u64) | game-core\src\simulation\map_def.rs:207 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 2 | distance | game_core::utils::distance | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:12 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 3 | distance_sq | game_core::utils::distance_sq | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:6 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 4 | distance_sq | game_core::Entity::distance_sq | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2157 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 5 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 6 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 7 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 8 | get_game_mode | game_core::AbstractGame::get_game_mode | pub | fn(&Self/#0) -> game_core::GameMode | game-core\src\simulation.rs:78 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 9 | get_game_mode | <game_core::Game as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::Game) -> game_core::GameMode | game-core\src\simulation\game.rs:1707 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 10 | get_game_mode | <game_core::SingleLaneGame as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::SingleLaneGame) -> game_core::GameMode | game-core\src\simulation\game.rs:4248 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 11 | is_bottom_side | game_core::is_bottom_side | pub | fn(&game_core::GameContext, u64, u64) -> bool | game-core\src\simulation\map_regions.rs:27 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 12 | is_ignored_well_enemy | game_ai::plan_legacy::old::is_ignored_well_enemy | pub | fn(usize, &game_core::PlayerState, &game_core::Entity) -> bool | game-ai\src\plan_legacy\old\fight_model.rs:754 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 13 | is_near_mid_line | game_core::is_near_mid_line | pub | fn(&game_core::GameContext, u64, u64) -> bool | game-core\src\simulation\map_regions.rs:127 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 14 | is_recent_visible | game_core::Blackboard::is_recent_visible | pub | fn(&game_core::Blackboard, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\game\blackboard.rs:346 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 15 | is_top_side | game_core::is_top_side | pub | fn(&game_core::GameContext, u64, u64) -> bool | game-core\src\simulation\map_regions.rs:21 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 16 | is_visible_from | game_core::Entity::is_visible_from | pub | fn(&game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1481 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 17 | iter_champions | game_core::AbstractGameWithCache::<'a, 'b>::iter_champions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5759 ~ game_core[6a30]::simulation::{impl#5}::iter_champions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1904 | True | True | 3 | IR 호출 심볼 일치(토큰 경계) |
| 18 | team | game_view::ClientData::team | pub | fn(&game_view::ClientData, usize) -> std::option::Option<std::cell::Ref< game_core::Team>> | game-view\src\logic\client\data.rs:1651 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 19 | team | game_view::ClientDatabase::team | pub | fn(&game_view::ClientDatabase, usize) -> std::option::Option<&game_core::Team> | game-view\src\logic\client\data.rs:968 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 20 | team | game_core::PlayerAiContext::<'a, 'b, 'r>::team | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:583 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 21 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 22 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 23 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 24 | v25_objective_far_split_pressure | game_ai::plan_legacy::team_plan::v25_objective_far_split_pressure | pub | fn(&game_core::PlayerState, &game_core::OperationData, game_core::JungleType) -> bool | game-ai\src\plan_legacy\team_plan\objective_helpers.rs:103 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
</details>

⚠**미매칭 11개**: `camp_possible_enemy_count`, `enumerate`, `focus_enemy`, `llvm.memcpy.p0.p0.i64`, `llvm.uadd.sat.i64`, `llvm.usub.sat.i64`, `map_or`, `near_ally_count`, `near_enemy_count`, `objective`, `side_ally_count`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 6곳** (m02.ll:9911, m02.ll:12214, m13.ll:37881, m13.ll:37886, m14.ll:30627, m15.ll:13972) · **형제 55개** (TeamPlan)

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

**`open` 8건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 재료 부재 | 한 줄 안 `&&`/`\|\|` 피연산자 순서(L277·280·281·285·287·290·292·297) — column 0 이라 재료 부재(IR 3종). 진리표는 IR 로 확정했고 logic 은 IR 블록 평가 순서로 적었다. | 4 |  |
| 1 | 표기 불가 | camp_overloaded 의 소스 표기 — IR 은 부정형 `%1497 = (visible < ally+2) \|\| (visible < 3)` 만 보유(dbg 는 poison). 이름이 'overloaded' 이고 L285 에서 `!%1497 && isolated_danger` 가 SoftDisengage 로 가므로 camp_overloaded = !%1497 로 읽었다(이름 극성 추정, 동작은 확정). `<ally+2` 가 `<=ally+1` 로 적혔을 수도(표기 불가). | 4 |  |
| 2 | 표기 불가 | L285 소스의 실제 괄호 구조 — IR 은 kind=4 로 가는 경로가 두 블록(%1513: !execute_window && low_hp_contact / %1498: !execute_window && !%1497 && isolated_danger)이라 `!ew && (lhc \|\| (id && co))` 로 합성했다. 두 개의 별도 else-if 일 가능성도 동일 외연(표기 불가). | 4 |  |
| 3 | 표기 불가 | L296~300 이 `match phase { Setup\|Hunt if .. => HoldCamp, _ => return None }` 인지 `else if matches!(phase, Setup\|Hunt) && ..` 인지 — L300 이 두 경로(%1554 default, %1556 조건 거짓) 공통 target 이라 외연 동일(표기 불가). | 4 |  |
| 4 | 미탐색 | is_visible_from(champ) 인라인의 정확한 소스(entity.rs:1481~1483) — IR: champ.team Neutral → true, Player(t) → visible_state[t] 태그 == 0(Visible). Invisible/Unknown 은 모두 false. 원문은 _gcbc 미탐색. | 4 |  |
| 5 | 표기 불가 | is_top_side/is_bottom_side 경계 포함 여부 — Morgard 경로 `!(ry < x)` → ry>=x 를 top, Serpen 경로 `!(ry > x)` → ry<=x 를 bottom 으로 읽음(대각선 ry==x 는 양쪽 다 참). 소스가 `>=`/`<=` 인지 컴파일러가 뒤집은 `!(<)` 인지는 표기 불가(동작 확정). | 4 |  |
| 6 | 재료 부재 | camp_possible_enemy 클로저(closure#8, L248~257) 내부 줄 배정 — IR 태그가 전부 fold 루트 258 이고 exe 패닉 Location 249:12 로 hp 식만 L249 확정. is_recent_visible/거리/elapsed 식의 정확한 줄(250~257)은 재료 부재(inlinedAt 도 258). | 4 |  |
| 7 | 미탐색 | exe 대조: 0xdd26e0 본문의 movabs 상수(140000²+1 ×5·170000²+1 ×5·180000²+1 ×6·190000²+1 ×5·150000²) 와 패닉 Location(216:17·225:72·227:71·231:65·234:85·240:85·242:99·249:12·262:9·282:43) 은 IR 과 일치. 차이는 인라인 경계뿐 — exe 는 is_ignored_well_enemy·is_near_mid_line·is_recent_visible 을 인라인(본문에 0xfa01/0x27101 상수, vtable +0xf8/+0x150 호출 출현)하고, 대신 fold(0xe31cd0 714B)·call_mut 심(0xe3b3c0 427B)·far_split_pressure(0xeca430)·utils::distance(0x12a07d0)·camp_pos(0xffa3e0) 은 별도 호출. 어긋남 0건. | 4 |  |

**`notes` 1건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | v25_objective_far_split_pressure / is_ignored_well_enemy / is_near_mid_line / Blackboard::is_recent_visible 내부는 이 명세 범위 밖(호출 계약만). is_recent_visible 만 g07.ll:157005~157049 를 읽어 계약(지금 보임 \|\| last_visible[pos]+120 >= tick)을 적었다. | 4 | 사실 서술 |

<details><summary>`closed` 2건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

| # | 물음 | 닫은 근거 |
|---|---|---|
| 0 | MapDef::camp_pos 의 bool 인자 이름/의미 — 값은 (team == 0) 으로 확정(L15261), 이름은 map_def.rs:207 시그니처에 없음(tcx sig 는 `bool` 만). 미탐색 = _gcbc MapDef::camp_pos 본문. | 본문에 해소 표기가 있다 |
| 1 | blackboard[1-team] 의 의도 — IR 확정(%918 = &data.blackboard[1-team]). is_recent_visible 본문(g07.ll:157005)이 target 소유 플레이어의 position 으로 self.last_visible[pos] 를 읽으므로 '관측 대상 팀의 보드'로 읽히지만 설계 의도 주석은 _docs 에 없음. | 본문에 해소 표기가 있다 |
</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

