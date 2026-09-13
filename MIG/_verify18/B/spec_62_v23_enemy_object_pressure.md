---

### `62` v23_enemy_object_pressure — 에픽(Morgard/Serpen)이 살아있고 보이고 피가 깎였으며, 반경 180000 내 적(가시) 수가 건강한 아군 수 이상이면 '적이 오브젝트 압박 중'

| 항목 | 값 |
|---|---|
| id | `objective_helpers__v23_enemy_object_pressure` |
| 심볼 | `_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy9team_plan17objective_helpers25v23_enemy_object_pressure` |
| 소스 | `game-ai\src\plan_legacy\team_plan\objective_helpers.rs:178` |
| IR | `m15.ll` 52547~52848행 |
| 경로·가시성 | `game_ai::plan_legacy::team_plan::v23_enemy_object_pressure` · **pub** |
| 계층 | 기타 |
| exe | `ec9190` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r9` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_core::PlayerState, &game_core::OperationData, game_core::JungleType) -> bool
```

<details><summary>인자 3개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | player | &PlayerState(2528B) | info.team(+0x930) 만 읽음 | 4 |
| 1 | 2 | data | &OperationData(24B) | +0 cache(&AbstractGameWithCache → +0 &dyn AbstractGame) · +8 context(&GameContext → +0x20 map, +0x38 tutorial) | 4 |
| 2 | 3 | target | JungleType(i8, range 0..6) | switch: 4=Morgard / 5=Serpen 만 처리, 그 외(Rhino0·Mushroom1·Stump2·Bee3) 즉시 false (m15.ll:52549~52552) | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn v23_enemy_object_pressure(player, data, target) -> bool {
  let ctx = data.context; let game = data.cache.game;
  // 179~192: 대상별 (에픽 id, 위치)
  let (id_opt, pos) = match target {
    Morgard(4) => {                                                    // 181
      if !morgard_exists(ctx)  { return false }   // 인라인: ctx.tutorial ∈ 1..=6 → false  (m15.ll:52560~52562, 극성: (t-1)<u6 → %106 false)
      let moba = game.get_game_mode()[vtable+0x40].as_moba().unwrap(); // 184 — game.rs:231 as_moba; 태그≠0 이면 unwrap_failed(패닉)
      let first = moba.jungle_runner.epic.live_list.first();           // 184 — len(+0x1a8)==0 → None
      (first, ctx.map.camp_pos(Morgard, player.info.team == 0))         // 185 — camp_pos 는 first 가 None 이어도 호출됨(m15.ll:52766~52772 %92)
    }
    Serpen(5) => {                                                     // 188
      if !serpen_exists(ctx)   { return false }   // 인라인: ctx.tutorial ∉ {0,5,7,8} → false (m15.ll:52571~52576 switch default → %106)
      let moba = … as_moba().unwrap();                                  // 191
      let first = moba.jungle_runner.serpen.live_list.first();          // 191 — len(+0x1d8)==0 → None
      (first, ctx.map.camp_pos(Serpen, player.info.team == 0))          // 192
    }
    _ => return false                                                  // 179 switch default
  };
  // 197: id → 엔티티
  let Some(ent) = id_opt.and_then(|&id| game.get_entity_by_id(id)[vtable+0x1f0]) else { return false };  // 197 closure$0 = get_entity_by_id 호출부; None(null) → false
  // 200: 보이고 + 피 깎임
  if !game.is_visible(player.info.team, ent.id)[vtable+0xf8] { return false }   // 200 — 먼저 평가(m15.ll:52716~52720)
  if !(ent.hp < ent.stat_cached.hp)               { return false }   // 200 — hp(+0x670) <u stat_cached.hp(+0x628); 풀피면 false (m15.ll:52723~52728)
  // 204~206: 인원 비교
  let allies  = v23_healthy_allies_near_point(player, data, pos.x, pos.y, 180000, 40);        // 204
  let enemies = v23_recent_visible_enemies_near_point(player, data, pos.x, pos.y, 180000, 40); // 205
  enemies != 0 && enemies >= allies                                    // 206 (m15.ll:52741~52743: `%88 ne 0` && `%88 uge %87`)
}

주: camp_pos 반환 {i64,i64} 중 extractvalue 0 → x(%66) 가 3번째 인자, extractvalue 1 → y(%65) 가 4번째 인자로 전달됨(m15.ll:52694~52695, 52736). 둘 다 반환값 range(i64 0,6) 인 i64 카운트.
```

**`mem` 메모리 접근 12건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | OperationData | 0x0 | cache | r | &AbstractGameWithCache; 그 +0 = &dyn AbstractGame(fat ptr: data, vtable+8) | 4 | OK |
| 1 | OperationData | 0x8 | context | r | &GameContext | 4 | OK |
| 2 | GameContext | 0x20 | map | r | &MapDef → camp_pos 인자 | 4 | OK |
| 3 | GameContext | 0x38 | tutorial | r | TutorialType(i8). morgard_exists/serpen_exists(rule_scope.rs:46/50 → runner.rs spawn_epic:263 / spawn_serpen:267) 인라인 판정에 사용 | 4 | OK |
| 4 | PlayerState | 0x930 | info.team | r | ==0 → camp_pos 의 is_blue_side; is_visible 의 team 인자 | 4 | OK |
| 5 | MobaMode | 0x1a0 | jungle_runner.epic.live_list.ptr | r | Morgard 경로: 살아있는 에픽 엔티티 id 목록의 [0] | 4 | OK |
| 6 | MobaMode | 0x1a8 | jungle_runner.epic.live_list.len | r | 0 이면 false | 4 | OK |
| 7 | MobaMode | 0x1d0 | jungle_runner.serpen.live_list.ptr | r | Serpen 경로: [0] | 4 | OK |
| 8 | MobaMode | 0x1d8 | jungle_runner.serpen.live_list.len | r | 0 이면 false | 4 | OK |
| 9 | Entity | 0x5c0 | id | r | 에픽 엔티티 id → is_visible(team, id) | 4 | OK |
| 10 | Entity | 0x670 | hp | r | 현재 HP | 4 | OK |
| 11 | Entity | 0x628 | stat_cached.hp | r | 최대 HP(스탯). hp < stat_cached.hp 여야 통과(=피가 깎였는가) | 4 | OK |

**`consts` 상수 8건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 4 | 179 | 태그 | JungleType::Morgard 메모리태그(tcxdict --enum JungleType: idx4=discr4=tag4, Direct). switch case 및 camp_pos 인자 | 3 |
| 1 | 5 | 179 | 태그 | JungleType::Serpen 태그 5. switch case 및 camp_pos 인자 | 4 |
| 2 | 6 | 181 | 임계 | morgard_exists 인라인: `(tutorial-1) <u 6` 즉 tutorial ∈ {First1,TopSolo2,Bottom3,MidSolo4,MidBottom5,JungleOnly6} 이면 에픽 미스폰 → false. None0·Line7·Total8 만 통과 (spawn_epic runner.rs:263) | 4 |
| 3 | 0 | 188 | 태그 | serpen_exists 인라인 switch case: tutorial ∈ {None0, MidBottom5, Line7, Total8} 이면 스폰(spawn_serpen runner.rs:267). 그 외 false. (0 은 또한 live_list.len==0 / enemies!=0 비교값) | 4 |
| 4 | 7 | 188 | 태그 | TutorialType::Line 태그 — serpen 스폰 허용 case | 4 |
| 5 | 8 | 188 | 태그 | TutorialType::Total 태그 — serpen 스폰 허용 case | 4 |
| 6 | 180000 | 204 | 미상 | 근접 반경(range 인자, 월드 단위 = 5.625셀). 아군·적 셈 둘 다 같은 값 | 4 |
| 7 | 40 | 204 | 임계 | min_hp_ratio 인자(HP% 하한 40) — 아군·적 셈 둘 다 같은 값. 실제 비교식은 피호출 함수 내부(미독해) | 4 |

**`knobs` 조정점 3건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 인원 셈 반경 | objective_helpers.rs:204~205 | 180000 | 올리면 더 먼 아군·적까지 세어 판정이 둔해지고(양쪽 다 늘어남), 내리면 오브젝트 바로 옆 인원만 본다 | 4 | 기존 |
| 1 | 건강 판정 HP% 하한(min_hp_ratio) | objective_helpers.rs:204~205 | 40 | 올리면 저체력 인원이 셈에서 빠져(아군·적 둘 다) 결과가 바뀐다 — 정확한 비교 방향은 피호출 함수 명세 참조 | 4 | 기존 |
| 2 | 적≥아군 비교(동수 포함) | objective_helpers.rs:206 | >= | `>` 로 바꾸면 동수일 때 압박으로 안 본다 | 4 | 기존 |

<details><summary>`callees` 피호출자 17건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | as_moba | game_core::GameMode::<'a>::as_moba | pub | fn(game_core::GameMode<'a/#0>) -> std::option::Option<&'a/#0 game_core::MobaMode> | game-core\src\simulation\game.rs:230 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 1 | camp_pos | game_core::MapDef::camp_pos | pub | fn(&game_core::MapDef, game_core::JungleType, bool) -> (u64, u64) | game-core\src\simulation\map_def.rs:207 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 2 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 3 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 4 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 5 | get_game_mode | game_core::AbstractGame::get_game_mode | pub | fn(&Self/#0) -> game_core::GameMode | game-core\src\simulation.rs:78 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 6 | get_game_mode | <game_core::Game as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::Game) -> game_core::GameMode | game-core\src\simulation\game.rs:1707 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 7 | get_game_mode | <game_core::SingleLaneGame as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::SingleLaneGame) -> game_core::GameMode | game-core\src\simulation\game.rs:4248 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 8 | is_visible | game_core::World::is_visible | pub | fn(&game_core::World, usize, usize) -> bool | game-core\src\simulation\game\data.rs:384 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 9 | is_visible | game_core::VisibleState::is_visible | pub | fn(&game_core::VisibleState) -> bool | game-core\src\simulation\game\data.rs:121 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 10 | is_visible | game_core::AbstractGame::is_visible | pub | fn(&Self/#0, usize, usize) -> bool | game-core\src\simulation.rs:125 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 11 | morgard_exists | game_ai::plan_legacy::rule_scope::morgard_exists | pub | fn(&game_core::GameContext) -> bool | game-ai\src\plan_legacy\rule_scope.rs:45 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 12 | range | game_core::Effect::range | pub | fn(&game_core::Effect, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:25 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 13 | serpen_exists | game_ai::plan_legacy::rule_scope::serpen_exists | pub | fn(&game_core::GameContext) -> bool | game-ai\src\plan_legacy\rule_scope.rs:49 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 14 | v23_enemy_object_pressure | game_ai::plan_legacy::team_plan::v23_enemy_object_pressure | pub | fn(&game_core::PlayerState, &game_core::OperationData, game_core::JungleType) -> bool | game-ai\src\plan_legacy\team_plan\objective_helpers.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 15 | v23_healthy_allies_near_point | game_ai::plan_legacy::team_plan::v23_healthy_allies_near_point | pub | fn(&game_core::PlayerState, &game_core::OperationData, u64, u64, u64, usize) -> usize | game-ai\src\plan_legacy\team_plan\objective_helpers.rs:25 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 16 | v23_recent_visible_enemies_near_point | game_ai::plan_legacy::team_plan::v23_recent_visible_enemies_near_point | pub | fn(&game_core::PlayerState, &game_core::OperationData, u64, u64, u64, usize) -> usize | game-ai\src\plan_legacy\team_plan\objective_helpers.rs:31 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
</details>

⚠**미매칭 1개**: `first`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 3곳** (m05.ll:51931, m09.ll:62521, m15.ll:56746) · **형제 0개** 

**`open` 4건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 표기 불가 | line 184/191 의 `live_list.first()` 표기 — IR 은 len==0 검사 후 buf[0] 로드(line 0 = 컴파일러 생성)라 `.first()`/`.get(0)`/`[0]` 어느 표기인지는 표기 불가(외연 동일: 비어있으면 false·아니면 [0]) | 4 |  |
| 1 | 미탐색 | get_game_mode 반환 {i64,ptr} 의 첫 필드가 as_moba 에서 0 이어야 Some 인 이유(GameMode 열거형 태그 0=Moba 추정) — tcxdict 로 미확인, 이 함수에선 태그≠0 이면 unwrap 패닉이라 동작엔 영향 없음 | 3 |  |
| 2 | 미탐색 | v23_healthy_allies_near_point / v23_recent_visible_enemies_near_point 내부(min_hp_ratio 40 의 실제 비교식·가시성 정의) — 담당 범위 밖(m15.ll:53341 / 55548 에 define 있음, 미독해) | 4 |  |
| 3 | 미탐색 | camp_pos(Morgard, is_blue_side) 가 돌려주는 좌표가 진영별로 다른지(is_blue_side 의 의미) — _gcbc g07.ll 본문 미독해 | 4 |  |

**`notes` 1건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | line 197 closure$0 의 정확한 콤비네이터(`and_then`/`map`+`?`) — 동작(None→false) 은 확정 | 4 | 사실 서술 |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

