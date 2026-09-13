---

### `57` handle_press_epic — 에픽 압박(PressEpic/SplitEpic) 팀 목표 발령 — 21% 확률로, 모르가드 운영 전략(Gather/Split14/Split131)과 챔프 위치·적 타워 잔존·미니언 수로 압박 라인을 고르고 Chat + objective 를 쓴다

| 항목 | 값 |
|---|---|
| id | `epic__handle_press_epic` |
| 심볼 | `_RNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old4epicNtNtB6_9team_plan8TeamPlan17handle_press_epic` |
| 소스 | `game-ai\src\plan_legacy\old\epic.rs:502` |
| IR | `m09.ll` 5284~6876행 |
| 경로·가시성 | `game_ai::plan_legacy::old::epic::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_press_epic` · **pub** |
| 계층 | 레거시 플랜 |
| exe | `dcd930` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r9` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData)
```

<details><summary>인자 6개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | self | &mut TeamPlan(1064B) | chats(+0xc0 Vec<Chat>) push · objective(+0x41f Option<MainObjective>) 쓰기 | 4 |
| 1 | 1 | _version | i64 | 미사용 (DILocalVariable `_version`, m09.ll:83677) | 4 |
| 2 | 2 | rnd | &mut StdRng | gen_range(0..100) 1회 (L507) + PlayerState::strategy 에 전달 (L510) | 4 |
| 3 | 3 | player | &PlayerState(2528B) | info.team(+0x930)·info.position(+0x9c0) | 4 |
| 4 | 4 | data | &OperationData(24B) | cache·context·blackboard 전부 사용 | 4 |
| 5 | 5 | _debug | &mut DebugFrameData | readnone — 미사용 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn handle_press_epic(&mut self, _version, rnd, player, data, _debug)
  if !morgard_exists(ctx)  // = spawn_epic: tutorial ∈ {1..6} 이면 false           L503
    return
  on = rnd.gen_range(0..100) < 21                                                   L507
  if !on: return                                                                     L509
  strategy = player.strategy(rnd, game)                                              L510
  team = player.info.team; enemy = 1 - team

  // 공용 서브루틴 (인라인, 소스 L518~547 / L568~606 에 두 벌 복제)
  //   tower(line) = cache.<line>_tower[enemy].or(<line>_tower2[enemy])
  //   has_<line>_first_tower  = tower(line).is_some_and(|t| !t.is_tower2())    // 적 1차 타워 생존
  //   has_<line>_second_tower = tower(line).is_some_and(|t|  t.is_tower2())    // 1차 죽고 2차만 생존
  //   <line>_state = data.blackboard[team].<line>_minion_state.minion_count  (i32)
  //   pick(X vs Mid) = if (has_X_first, has_X_second, X_state) > (has_mid_first, has_mid_second, mid_state)  // 튜플 사전식 >
  //                    { X } else { Mid }
  //   emit(kind, line) = self.chats.push(Chat::kind(line, 0)); self.objective = Some(MainObjective::{PressEpic|SplitEpic}(line))
  //   side(champ) = IR 식 `(height - champ.y) <u champ.x`  → true 면 '아래쪽(Bottom 라인) 진영', false 면 '위쪽(Top 라인) 진영'  (인라인 is_top_side; 이름 극성은 unknown 참조)

  match strategy.morgard_use {                                                       L512
    Gather =>                                                                        L515
      champ = cache.player_champion[team][player.position].unwrap()
      if side(champ)==위쪽:  emit(Press, pick(Top vs Mid))      // L518~530: Top→Press(Top) / else Press(Mid)
      else:                  emit(Press, pick(Bottom vs Mid))   // L535~547: Bottom→Press(Bottom) / else Press(Mid)
      objective = PressEpic(line)

    Split14{position} =>                                                             L552
      epic_tick   = game.get_game_mode().as_moba().unwrap().jungle_runner.epic.next_respawn_tick     L553 (비 Moba → 패닉)
      serpen_tick = ....serpen.next_respawn_tick                                                     L554
      object_far_line = if epic_tick > serpen_tick { Top } else { Bottom }   // 다음에 뜨는 오브젝트에서 먼 라인   L555
      if player.position == position:                                            L561
        emit(Split, object_far_line); objective = SplitEpic(object_far_line)     L562  // 분할 담당자는 먼 라인으로
      else:                                                                      L565
        champ = cache.player_champion[team][player.position].unwrap()
        if side(champ)==위쪽:                                                    L566~584
          if object_far_line == Top: emit(Press, Mid)              L576~577  // Top 이 먼 라인이면 Mid
          else:                      emit(Press, pick(Top vs Mid)) L580~584
        else:                                                                    L590~606
          if object_far_line == Top: emit(Press, pick(Bottom vs Mid))  L598~606
          else:                      emit(Press, Mid)                  L599
        objective = PressEpic(line)

    Split131{position1, position2} =>                                                L613
      if player.position == position1                                                L614
         || (player.position == position2 && cache.player_champion[team][position1].is_none()):   L615
        emit(Split, Top); objective = SplitEpic(Top)                                 L616
      else if player.position == position2:                                          L618
        emit(Split, Bottom); objective = SplitEpic(Bottom)                           L620
      else:
        emit(Press, Mid); objective = PressEpic(Mid)                                 L623
  }                                                                                  L629 ret

  튜플 비교 의미: 1순위 '적 1차 타워가 아직 서 있는 라인'(true>false) → 2순위 '2차 타워만 남은 라인' → 3순위 내 팀 minion_count 큰 쪽(부호 있는 i32). 즉 덜 밀린(타워가 남은) 사이드 라인을 Mid 보다 우선 압박하고, 동률이면 Mid.
```

**`mem` 메모리 접근 35건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | GameContext | 0x38 | tutorial | r | 인라인 morgard_exists→spawn_epic(runner.rs:263): `(tutorial-1) <u 6` 즉 First(1)..JungleOnly(6) 면 에픽 없음→즉시 return (m09.ll:5347~5351) | 4 | OK |  |
| 1 | GameContext | 0x8 | setting | r | &GameSetting → height (m09.ll:5440~5446, 6080~6086) | 4 | OK |  |
| 2 | GameSetting | 0x12c0 | height | r | 인라인 map_regions::is_top_side(x,y): IR 식 `(height - y) <u x` (m09.ll:5442~5448) | 4 | OK |  |
| 3 | OperationData | 0x0 | cache | r | &AbstractGameWithCache (m09.ll:5361) | 4 | OK |  |
| 4 | OperationData | 0x8 | context | r | &GameContext (m09.ll:5341~5342) | 4 | OK |  |
| 5 | OperationData | 0x10 | blackboard | r | &[Blackboard;2] → `[team]` (내 팀) 인덱스, 744B stride (m09.ll:5598~5600·5837~5841) | 4 | OK |  |
| 6 | AbstractGameWithCache | 0x0 | game.data_ptr | r | &dyn AbstractGame (data +0x0 / vtable +0x8). vtable+0x40 = get_game_mode (divtable) (m09.ll:5361~5364, 5391~5393) | 3 | OK |  |
| 7 | AbstractGameWithCache | 0x1e0 | player_champion[team][pos] | r | 내 챔프(Gather L515 / Split14 L565: None→unwrap 패닉) · Split131 L615 에선 `[team][position1].is_none()` 검사 | 4 | OK |  |
| 8 | AbstractGameWithCache | 0x180 | top_tower[1-team] | r | 인라인 `cache.tower(enemy, Top)` = top_tower.or(top_tower2) (option.rs:1622 `or`, simulation.rs `tower`) (m09.ll:5475~5482, or 폴백 select 5482) | 4 | OK |  |
| 9 | AbstractGameWithCache | 0x190 | top_tower2[1-team] | r | 동상(폴백) | 4 | OK |  |
| 10 | AbstractGameWithCache | 0x1a0 | mid_tower[1-team] | r | `cache.tower(enemy, Mid)` (m09.ll:5503~5510) | 4 | OK |  |
| 11 | AbstractGameWithCache | 0x1b0 | mid_tower2[1-team] | r | 동상(폴백) | 4 | OK |  |
| 12 | AbstractGameWithCache | 0x1c0 | bottom_tower[1-team] | r | `cache.tower(enemy, Bottom)` (m09.ll:5458~5465) | 4 | OK |  |
| 13 | AbstractGameWithCache | 0x1d0 | bottom_tower2[1-team] | r | 동상(폴백) | 4 | OK |  |
| 14 | PlayerState | 0x930 | info.team | r | team(<2 아니면 bounds 패닉)·enemy=1-team (m09.ll:5381~5384·5447) | 4 | OK |  |
| 15 | PlayerState | 0x9c0 | info.position@tag | r | Position 태그. 챔프 조회 인덱스 및 Split 전략의 position 과 비교 (m09.ll:5407~5408·5421~5422·6000~6005) | 4 | OK |  |
| 16 | Strategy | 0x4 | morgard_use / Split131.position1 | r | PlayerState::strategy() 반환(24B 스택). 니치 인코딩: 5=Gather·6=Split14·그 외(0..4)=Split131 이며 그 값이 곧 position1 (tcxdict --enum MorgardUseStrategy, m09.ll:5366~5370) | 3 | OK |  |
| 17 | Strategy | 0x8 | morgard_use@Split14.position / Split131.position2 | r | m09.ll:5387~5388(Split14), 5402~5403(Split131) | 4 | OK |  |
| 18 | Entity | 0x660 | x | r | 내 챔프 좌표 → is_top_side (m09.ll:5434~5437) | 4 | OK |  |
| 19 | Entity | 0x668 | y | r | 동상 | 4 | OK |  |
| 20 | Entity | 0x68 | ty@tag | r | 인라인 Entity::is_tower2(entity.rs:1309): ty 태그==2(Tower) 여야 진행 (m09.ll:5492~5495) | 4 | OK |  |
| 21 | Entity | 0x128 | ty@Tower.info.ty | r | TowerType 태그. 인라인 Tower::is_second_tower(tower.rs:100): `>4`(Top2/Mid2/Bottom2=5..7) = 2차 타워, `<5` = 1차(Top/Mid/Bottom/TwinA/TwinB) (m09.ll:5518~5520) | 4 | OK |  |
| 22 | Blackboard | 0x20 | top_minion_state.minion_count | r | i32(부호). top_state (m09.ll:5837~5838) | 4 | OK |  |
| 23 | Blackboard | 0x48 | mid_minion_state.minion_count | r | mid_state (m09.ll:5840~5841) | 4 | OK |  |
| 24 | Blackboard | 0x70 | bottom_minion_state.minion_count | r | bottom_state (m09.ll:5601~5602) | 4 | OK |  |
| 25 | GameMode | 0x0 | tag | r | get_game_mode() 반환 {i64 tag, ptr}. 인라인 as_moba(game.rs:231): 0=Moba 아니면 unwrap 패닉 (m09.ll:5393~5398) | 4 | OK |  |
| 26 | MobaMode | 0x1b0 | jungle_runner.epic.next_respawn_tick | r | epic_tick (L553, m09.ll:5976~5977) | 4 | OK |  |
| 27 | MobaMode | 0x1e0 | jungle_runner.serpen.next_respawn_tick | r | serpen_tick (L554, m09.ll:5992~5993) | 4 | OK |  |
| 28 | TeamPlan | 0xc0 | chats.cap | r | Vec::push 인라인 — len==cap 이면 RawVec::grow_one (m09.ll:5637·5656~5661) | 4 | OK |  |
| 29 | TeamPlan | 0xc8 | chats.ptr | r | push 대상 버퍼 (m09.ll:5666~5668) | 4 | OK |  |
| 30 | TeamPlan | 0xd0 | chats.len | r | push 후 +1 (m09.ll:5651~5652·5676~5677) | 4 | OK |  |
| 31 | TeamPlan | 0xc8 | chats[len] | w | 24B Chat 원소에 byte0=태그·byte1=LineType·+0x8=usize 0 을 씀. 14개 분기 각각 1회 push (m09.ll:5671~5680 등) | 4 | OK | Chat::Press(line, 0) (태그 21) 또는 Chat::Split(line, 0) (태그 20) |
| 32 | TeamPlan | 0xd0 | chats.len | w | push | 4 | OK | len+1 |
| 33 | TeamPlan | 0x41f | objective | w | Some(MainObjective) — Option 니치(None=255) 라 태그 자체가 Some 표시. 14분기 phi 로 접혀 line 0 에 sunk (m09.ll:5961·5963~5964) | 4 | OK | 5=PressEpic 또는 6=SplitEpic |
| 34 | TeamPlan | 0x420 | objective@Some.0.line | w | chat 에 넣은 line 과 동일 (m09.ll:5962·5965~5966) | 4 | OK | LineType 0=Top/1=Mid/2=Bottom (분기별) |

**`consts` 상수 9건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 6 | 503 | 태그 | ① 인라인 spawn_epic: `(tutorial-1) <u 6` → TutorialType 1..6(First·TopSolo·Bottom·MidSolo·MidBottom·JungleOnly) 면 에픽 미스폰→return (m09.ll:5350). ② MainObjective::SplitEpic 메모리태그 6 (phi m09.ll:5961) | 4 |
| 1 | 100 | 507 | 임계 | rnd.gen_range(0..100) 상한 (m09.ll:5354) | 4 |
| 2 | 21 | 507 | 태그 | ① `on = gen_range(0..100) < 21` — 호출당 21% 확률로만 발령 (m09.ll:5355). ② Chat::Press 메모리태그 21 (store i8 21, m09.ll:5671 등) | 4 |
| 3 | 5 | 512 | 센티널 | ① MorgardUseStrategy 니치 시작: 태그 5=Gather·6=Split14·<5=Split131(암묵, 그 값=position1) — `tag>4 ? tag-5 : 2` switch (m09.ll:5368~5370). ② TowerType `<5` = 1차 타워 판정 (m09.ll:5520 등). ③ MainObjective::PressEpic 태그 5 (phi 5961). ④ Split131 position1 <5 bounds (m09.ll:6758) | 4 |
| 4 | 4 | 520 | 임계 | TowerType `>4` = 2차 타워(Top2=5/Mid2=6/Bottom2=7) — 인라인 is_second_tower (m09.ll:5574 등) | 4 |
| 5 | 2 | 518 | 태그 | ① EntityType::Tower 메모리태그 2 (is_tower2 인라인, m09.ll:5494 등). ② LineType::Bottom=2 (chat/objective line, m09.ll:5717·5996 select). ③ team<2 bounds (m09.ll:5384·5416) | 4 |
| 6 | 20 | 562 | 태그 | Chat::Split 메모리태그 20 — Split14/Split131 의 분할 담당자에게 (m09.ll:6049·6747·6823) | 4 |
| 7 | 0 | 527 | 태그 | LineType::Top=0 (chat line·objective line) 및 Chat 페이로드 .1 = 0 (m09.ll:5675 등, Top 저장 예 5717 아님 — Bottom=2; Top 예는 L527 분기) | 4 |
| 8 | 1 | 530 | 태그 | LineType::Mid=1 (m09.ll:5673 등). enemy = 1 - team (m09.ll:5447) | 4 |

**`knobs` 조정점 3건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 발령 확률 | epic.rs:507 | 21 | gen_range(0..100) < 21. 올리면 handle_press_epic 이 호출될 때 더 자주 PressEpic/SplitEpic 목표를 세움(호출 빈도는 호출자 소관) | 4 | 기존 |
| 1 | 먼 라인 판정 | epic.rs:555 | epic_tick > serpen_tick → Top | 부등호를 바꾸면 Split14 분할 담당자가 가는 라인과 나머지의 Mid 강제 조건이 반전됨 | 4 | 기존 |
| 2 | 라인 선택 우선순위 | epic.rs:526·543·580·602 | (1차타워, 2차타워, minion_count) 사전식 | 튜플 원소 순서를 바꾸면(예: minion_count 우선) 압박 라인 선택 기준이 바뀜. 값 노브가 아니라 구조 노브 | 4 | 기존 |

<details><summary>`callees` 피호출자 14건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | as_moba | game_core::GameMode::<'a>::as_moba | pub | fn(game_core::GameMode<'a/#0>) -> std::option::Option<&'a/#0 game_core::MobaMode> | game-core\src\simulation\game.rs:230 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 1 | get_game_mode | game_core::AbstractGame::get_game_mode | pub | fn(&Self/#0) -> game_core::GameMode | game-core\src\simulation.rs:78 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 2 | get_game_mode | <game_core::Game as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::Game) -> game_core::GameMode | game-core\src\simulation\game.rs:1707 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 3 | get_game_mode | <game_core::SingleLaneGame as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::SingleLaneGame) -> game_core::GameMode | game-core\src\simulation\game.rs:4248 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 4 | handle_press_epic | game_ai::plan_legacy::old::epic::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_press_epic | pub | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) | game-ai\src\plan_legacy\old\epic.rs:502 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 5 | is_tower2 | game_core::EntityType::is_tower2 | pub | fn(&game_core::EntityType) -> bool | game-core\src\simulation\entity.rs:1308 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 6 | kind | game_view::ui::database_edit_ui::DbEditAppearanceTarget::kind | in:game_view::ui::database_edit_ui | fn(game_view::ui::database_edit_ui::DbEditAppearanceTarget) -> game_view::ui::athlete_appearance_popup::AppearanceTargetKind | game-view\src\ui\database_edit_ui.rs:100 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 7 | morgard_exists | game_ai::plan_legacy::rule_scope::morgard_exists | pub | fn(&game_core::GameContext) -> bool | game-ai\src\plan_legacy\rule_scope.rs:45 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 8 | pick | game_view::MatchUIRunner::pick | pub | fn(&mut game_view::MatchUIRunner, &mut std::vec::Vec<engine_core::ui::node::Node, std::alloc::Global>, usize, &str) | game-view\src\ui\match_ui.rs:4833 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 9 | push | <game_core::Staff as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::staff | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\staff.rs:38 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 10 | push | <game_core::Athlete as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:434 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 11 | push | <game_core::Contract as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:2498 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 12 | strategy | game_core::PlayerState::strategy | pub | fn(&game_core::PlayerState, &mut rand::rngs::std::StdRng, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + ) -> game_core::Strategy | game-core\src\simulation\state\player.rs:1576 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 13 | tower | game_core::AbstractGameWithCache::<'a, 'b>::tower | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, game_core::LineType, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:1822 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
</details>

⚠**미매칭 5개**: `emit`, `gen_range`, `grow_one`, `minion_count`, `side`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m09.ll:14832) · **형제 55개** (TeamPlan)

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

**`open` 7건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | `Blackboard.<line>_minion_state.minion_count`(i32) 의 정확한 의미(내 팀 라인 미니언 수인지 차이값인지) — _docs 에 없음, BrainMinionParameter(ai_interface.rs:18) 채우는 쪽 미탐색. 여기선 '클수록 그 라인 우선' 까지만 확정 | 4 |  |
| 1 | 미탐색 | PlayerState::strategy(rnd, game) 내부(어떤 확률로 Gather/Split14/Split131 을 고르는지) — 담당 밖. Strategy+0x4 morgard_use 만 소비 | 4 |  |
| 2 | 미탐색 | `cache.tower(team, line)` (simulation.rs, 인라인 line 0) 이 `.or()` 이외의 필터(생존 여부 등)를 갖는지 — IR 에는 or 만 남음. `*_tower[]` 캐시가 파괴된 타워를 None 으로 두는지는 new_with_prev_cache 소관(미탐색) — 튜플 의미 해석은 그 가정에 의존 | 4 |  |
| 3 | 미탐색 | Chat::Press/Split 의 두 번째 페이로드(usize, 항상 0)의 의미 — 소비처 미탐색 | 4 |  |
| 4 | 표기 불가 | objective 저장이 각 분기 안에 있는지 match 뒤 한 번인지 — phi 로 접혀(line 0) 표기 불가, 동작 동일 | 4 |  |
| 5 | 표기 불가 | L614~615 의 소스 표기(`\|\|` 결합 순서) — column 부재로 표기 불가. 외연 확정 | 4 |  |
| 6 | 미탐색 | 호출자(누가 언제 handle_press_epic 을 부르는지 · 21% 가 틱당인지 이벤트당인지) — 미탐색 = `grep handle_press_epic` 호출 사이트 | 4 |  |

**`notes` 1건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | `is_top_side` 의 이름 극성: game_core 에 define 없이 인라인만(grep ^define 0건). IR 식은 `(height-y) <u x` 이고 그 true 쪽이 Bottom 라인 타워 검사(L535/L590)·false 쪽이 Top 라인 검사(L518/L568)로 간다. 소스 순서(if 본문=L518 top / else=L535 bottom)로 보면 `is_top_side == !((h-y)<x) == (x+y <= height)` 로 읽히나(컴파일러의 비교 반전+라벨 교환) 이는 추정 — 확정은 오라클(is_top_side 가 pub 이면) 필요. 동작(어느 좌표가 어느 분기)은 확정 | 2 | 사실 서술 |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

