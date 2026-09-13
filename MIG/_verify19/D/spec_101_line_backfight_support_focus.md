---

### `101` line_backfight_support_focus — 라인 후방 교전 지원 — 내 뒤쪽(넥서스 쪽)에서 적과 얽힌 아군이 있으면 (아군id, 적id) 포커스 쌍을 고른다

| 항목 | 값 |
|---|---|
| id | `utils__line_backfight_support_focus` |
| 심볼 | `_RNvNtCshdEBA0ozCnw_7game_ai5utils28line_backfight_support_focus` |
| 소스 | `game-ai\src\utils.rs:269` |
| IR | `m04.ll` 53979~54933행 |
| 경로·가시성 | `game_ai::line_backfight_support_focus` · **pub** |
| 계층 | 기타 |
| exe | `d3a3a0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r10` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &game_core::PlayerState, &game_core::OperationData, game_core::LineType) -> std::option::Option<(usize, usize)>
```

<details><summary>인자 5개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | Option<(usize,usize)>(24B) | %0. +0 tag(0=None/1=Some), +8 ally.id, +16 enemy.id | 4 |
| 1 | 1 | version | usize | %1. 본문·클로저 어디서도 안 읽음(dbg_value 만) | 4 |
| 2 | 2 | player | &PlayerState(2528B) | %2 | 4 |
| 3 | 3 | data | &OperationData(24B) | %3 | 4 |
| 4 | 4 | line | LineType(i8) | %4. 스택 %23 에 저장해 클로저가 참조. 0 Top/1 Mid/2 Bottom | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
L271: team = player.info.team; champ = cache.player_champion[team][player.info.position]?   (None → return None)
L272: if !is_near_line(context, champ.x, champ.y, line) { L273: return None }
L276: nexus = cache.nexus[team]?
L277: front_id = data.blackboard[team].minion_state(line).front_minion?   (Top→top/Mid→mid/Bottom→bottom _minion_state)
L278: front = cache.game.get_entity_by_id(front_id)?
L279: if !is_near_line(context, front.x, front.y, line) { L280: return None }
L283: front_progress = distance(nexus.xy, front.xy)
L284: champ_progress = distance(nexus.xy, champ.xy)
L285: enemy_team = 1 - team
L286: support_range_sq = 115600000000
L288~346: result = cache.player_champion[team].iter().enumerate()
  .filter_map(|(i,a)| a.map(|a| (i,a)))
  .filter(closure$2 L291~294: |(_, ally)| ally.id != champ.id && ally.can_target && ally.block_target_tick == 0 && !ally.is_in_return()[ty==Champion && action_state==Return] && is_near_line(ctx, ally.x, ally.y, line))
  .filter_map(closure$3 L296~346: |(ally_pos, ally)| {
     L297: ally_progress = distance(nexus.xy, ally.xy)
     L298: if !(front_progress > ally_progress + 100000) { return None }    // 아군이 전선보다 충분히 후방
     L299: if !(champ_progress > ally_progress + 70000) { return None }     // 아군이 나보다 충분히 후방
     L300: if ally.distance_sq(champ) > support_range_sq { return None }    // 340000 이내
     L304~305: (ally_focus, ally_in_battle) = match blackboard[team].big_goal[ally_pos].1 { Some(Battle{focus}) => (focus, focus.is_some()), _ => (None, false) }   [in_battle 인라인 blackboard.rs:167]
     L310: enemy_team 인덱스 검사(<2)
     L312~: cache.player_champion[enemy_team].iter().enumerate().filter_map(|(j,e)| e.map(|e| (j,e)))
        .filter_map(closure$1 = aux call_mut m04.ll:63860 L314~337: |(enemy_pos, enemy)| {
           can_target(enemy)[can_target && block_target_tick==0] 아니면 None
           L314: if !data.blackboard[enemy_team].is_recent_visible(cache.game, player, enemy) { None }
           L315: if !is_near_line(ctx, enemy.x, enemy.y, line) { None }
           L319: enemy_progress = distance(nexus.xy, enemy.xy)
           L320: if !(front_progress > enemy_progress + 70000) { None }    // 적이 전선 미니언보다 70000 이상 우리 넥서스 쪽으로 파고듦
           L324: close_to_ally = ally.distance_sq(enemy) < 22500000001
           L325: focused_enemy = ally_focus == Some(enemy.id)
           L326~328: enemy_focus_ally = matches!(blackboard[enemy_team].big_goal[enemy_pos].1, Some(Battle{focus: Some(f)}) if f == ally.id)
           L330~331: enemy_targeting_ally = blackboard[enemy_team].small_actions[enemy_pos] ∈ {Attack,Skill,Skill2,Ult}(태그 6..=9) && target_id == ally.id
           L332: ally_targeting_enemy = blackboard[player.info.team].small_actions[ally_pos] ∈ {6..=9} && target_id == enemy.id
           L335: if focused_enemy || enemy_focus_ally || enemy_targeting_ally || ally_targeting_enemy   → Some
           L336: else if close_to_ally && ally_in_battle → Some
           else None
           L337: Some((ally.id, enemy.id, ally.distance_sq(champ)))   // ★키 = 아군↔나 거리 (적↔나 아님)
        })
        .min_by_key(|t| t.2)   [aux m12.ll:4403 fold — 키가 아군마다 상수라 같은 아군의 적 중 첫 번째(enemy_pos 최소)가 뽑힘]
  })
  .min_by_key(|t| t.2)   [aux m12.ll:4886 fold — 나에게 가장 가까운 아군; 동률이면 ally_pos 작은 쪽(min_by 는 Less|Equal 이면 앞 값 유지)]
L347: result.map(|(a, e, _)| (a, e))

※ 본체는 첫 아군 매치(find_map)로 초기값을 만들고 나머지를 aux fold 에 넘기는 reduce 구조 — 본체 54300~54846 은 첫 아군 탐색, 54857~54923 은 fold 호출·결과 조립.
```

**`mem` 메모리 접근 26건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | team (54014); 클로저 안에서도 재로드(64079) | 4 | OK |  |
| 1 | PlayerState | 0x9c0 | info.position@tag | r | as_index (54025) | 4 | OK |  |
| 2 | OperationData | 0x0 | cache | r | 54028 | 4 | OK |  |
| 3 | OperationData | 0x8 | context | r | 54040 | 4 | OK |  |
| 4 | OperationData | 0x10 | blackboard | r | &[Blackboard;2] (54074) | 4 | OK |  |
| 5 | AbstractGameWithCache | 0x1e0 | player_champion / [team][0..5] / [enemy_team][0..5] | r | [2][5] Option<&Entity>: 자기(54029~54032), 아군 순회(54161, 54303), 적 순회(54480) | 4 | OK |  |
| 6 | AbstractGameWithCache | 0x170 | nexus[team] | r | None 이면 None 반환 (54060~54068) | 4 | OK |  |
| 7 | AbstractGameWithCache | 0x0 | game.data_ptr | r | 54108 | 4 | OK |  |
| 8 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r | 슬롯 0x1f0=get_entity_by_id (54115); is_recent_visible 에 &dyn AbstractGame 으로 전달(63924) | 4 | OK |  |
| 9 | Blackboard | 0x0 | top_minion_state.front_minion | r | minion_state(line) 인라인: Top→+0 / Mid→+0x28 / Bottom→+0x50 (54077~54096); Option<usize> tag +0 값 +8 | 4 | OK |  |
| 10 | Blackboard | 0xf8 | big_goal[pos].1@tag | r | stride 32: 아군(54446~54449 blackboard[team].big_goal[ally_pos]) / 적(64022~64026 blackboard[enemy_team].big_goal[enemy_pos]) 태그 5=Battle | 4 | OK |  |
| 11 | Blackboard | 0x100 | big_goal[pos].1@Battle.focus@tag | r | 54458~54459, 64034 | 4 | OK |  |
| 12 | Blackboard | 0x108 | big_goal[pos].1@Battle.focus@Some.0 | r | 54460~54461, 64040 | 4 | OK |  |
| 13 | Blackboard | 0x78 | small_actions[pos]@tag | r | stride 24: 적(64050~64052 blackboard[enemy_team].small_actions[enemy_pos]) / 아군(64094~64098 blackboard[team].small_actions[ally_pos]); 태그-6 <4 ⟹ Attack/Skill/Skill2/Ult | 4 | OK |  |
| 14 | Blackboard | 0x80 | small_actions[pos]@Some.0.target_id | r | 64061~64062, 64119~64120 | 4 | OK |  |
| 15 | Entity | 0x660 | x | r | champ/front/ally/enemy/nexus | 4 | OK |  |
| 16 | Entity | 0x668 | y | r |  | 4 | OK |  |
| 17 | Entity | 0x5c0 | id | r | champ.id(54297) ally.id(54336) enemy.id(63988) | 4 | OK |  |
| 18 | Entity | 0x6b9 | can_target | r | bool — 아군(54340~54342) 적(63902~63904) | 4 | OK |  |
| 19 | Entity | 0x6a0 | block_target_tick | r | == 0 이어야 함 (54344~54346, 63905~63907) | 4 | OK |  |
| 20 | Entity | 0x68 | ty@tag | r | is_in_return 인라인: == 13(Champion) (54351~54353) | 4 | OK |  |
| 21 | Entity | 0x70 | ty@Champion.0.action_state@tag | r | == 1(Return) 이면 귀환 중 → 아군 제외 (54354~54357) | 4 | OK |  |
| 22 | Option<(usize,usize)>(sret) | 0x0 | tag | w | 54049/54056/54068/54123/54136(None) · 54927(phi 0/1) | 4 | 확인불가(tcx 사전에 타입 없음) | 0 \| 1 |
| 23 | Option<(usize,usize)>(sret) | 0x8 | Some.0 = ally.id | w | 54920 | 4 | 확인불가(tcx 사전에 타입 없음) | min_by_key 결과 .0 |
| 24 | Option<(usize,usize)>(sret) | 0x10 | Some.1 = enemy.id | w | 54922 | 4 | 확인불가(tcx 사전에 타입 없음) | min_by_key 결과 .1 |
| 25 | stack | 0x0 | front_progress/champ_progress/enemy_team/support_range_sq/ally_pos/ally_focus/ally_in_battle | w | 54159, 54161, 54163, 54154, 54397, 54473~54474 — 외부 관측 불가 | 4 | 확인불가(★모호: 동명 def_path 3개 [('game_core::Hunter) | 클로저 캡처용 지역 alloca |

**`consts` 상수 10건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 115600000000 | 286 | 산출값 | support_range_sq = 340000^2 — 아군↔나 distance_sq 가 이보다 크면 지원 후보 제외 (L300, 54440~54442). 10.6셀 | 4 |
| 1 | 100000 | 298 | 계수 | front_progress > ally_progress + 100000 — 아군이 전선 미니언보다 100000(3.1셀) 이상 넥서스 쪽(후방)에 있어야 함 | 4 |
| 2 | 70000 | 299 | 계수 | champ_progress > ally_progress + 70000 — 아군이 나보다 70000(2.2셀) 이상 후방. 같은 값이 L320 에서 front_progress > enemy_progress + 70000(적이 전선보다 70000 이상 안쪽으로 들어옴) 에도 쓰임 | 4 |
| 3 | 22500000001 | 324 | 미상 | 150000^2 + 1 — close_to_ally = ally.distance_sq(enemy) < 이 값 (≤150000, 4.7셀). aux m04.ll:63984 | 4 |
| 4 | 13 | 291 | 태그 | EntityType 태그 13 = Champion (is_in_return 인라인 entity.rs:1648) | 4 |
| 5 | 1 | 291 | 태그 | ChampionActionState 태그 1 = Return (is_in_return) · sret Some 태그 | 4 |
| 6 | 5 | 304 | 태그 | BigGoal 태그 5 = Battle{focus} (L304 아군, L326 적) · 포지션 배열 길이 5 | 4 |
| 7 | -6 | 330 | 태그 | SmallAction 태그 t 에 대해 (t-6) <u 4 ⟹ t∈{6 Attack,7 Skill,8 Skill2,9 Ult} — 표적 있는 액션만 (aux 64056~64057, 64102~64103) | 4 |
| 8 | 4 | 330 | 태그 | 위 태그 범위 폭(6..=9 의 4개) | 4 |
| 9 | 40 | 288 | 태그 | 아군 순회 종료 오프셋 = 5 슬롯 × 8B (54845, 루프 상한 접힘) | 4 |

**`knobs` 조정점 4건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 지원 반경(아군↔나) | utils.rs:286 | 115600000000 | 올리면 더 먼 후방 아군까지 지원 대상이 돼 라인에서 더 자주 뒤로 빠진다 | 4 | 기존 |
| 1 | 아군 후방 판정 여유(전선 기준) | utils.rs:298 | 100000 | 내리면 전선 바로 뒤 아군도 '후방'으로 봐서 발동 잦아짐 | 4 | 기존 |
| 2 | 아군 후방 판정 여유(나 기준) / 적 침투 깊이 | utils.rs:299, 320 | 70000 | 내리면 나와 비슷한 위치의 아군도 대상, 전선 살짝 넘은 적도 위협으로 봄 → 발동↑ | 4 | 기존 |
| 3 | 얽힘 근접 반경(아군↔적) | utils.rs:324 | 22500000001 | 올리면 교전 중 아군 근처 더 먼 적도 포커스 후보 (L336 경로) | 4 | 기존 |

<details><summary>`callees` 피호출자 12건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | can_target | game_core::Entity::can_target | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1477 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 1 | can_target | game_core::OperationData::<'a, 'b>::can_target | pub | fn(&game_core::OperationData<'a/#0, 'b/#1>, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\state\player.rs:23 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 2 | distance | game_core::utils::distance | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:12 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 3 | distance_sq | game_core::utils::distance_sq | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:6 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 4 | distance_sq | game_core::Entity::distance_sq | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2157 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 5 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 6 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 7 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 8 | is_in_return | game_core::Entity::is_in_return | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1647 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 9 | is_near_line | game_core::is_near_line | pub | fn(&game_core::GameContext, u64, u64, game_core::LineType) -> bool | game-core\src\simulation\map_regions.rs:139 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 10 | is_recent_visible | game_core::Blackboard::is_recent_visible | pub | fn(&game_core::Blackboard, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\game\blackboard.rs:346 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 11 | minion_state | game_core::Blackboard::minion_state | pub | fn(&game_core::Blackboard, game_core::LineType) -> &game_core::BrainMinionParameter | game-core\src\simulation\game\blackboard.rs:378 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
</details>

⚠**미매칭 2개**: `enumerate`, `llvm.memcpy.p0.p0.i64`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m13.ll:22693) · **형제 0개** 

**`open` 5건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 표기 불가 | L335 의 4항 `\|\|` 결합 순서와 L336 `close_to_ally && ally_in_battle` 의 소스 표기 — IR select 체인(64114~64116, 64130~64132, 64178)으로 동작은 확정, 줄 안 순서는 column 부재로 표기 불가 | 4 |  |
| 1 | 미탐색 | L337 키가 ally.distance_sq(champ) 라는 것은 aux 63963~63983(x1,y1=ally, x2,y2=champ) 로 확정. 소스 의도(적 거리를 의도했을 가능성)는 판단 불가 — IR 이 정본 | 3 |  |
| 2 | 미탐색 | is_recent_visible(blackboard.rs:346)·is_near_line(map_regions.rs:139)·distance(utils.rs:12) 본문 미독 — 계약만. is_recent_visible 은 blackboard[enemy_team] 을 self 로 호출(63919~63924: env+32 = &enemy_team) | 4 |  |
| 3 | 미탐색 | aux 두 fold(m12.ll 4403/4886)는 closure$1~$3 본문을 다시 인라인한 복제본이라 상수(100000/70000/22500000001)가 중복 출현 — 판정은 동일 | 4 |  |
| 4 | 미탐색 | constants 의 5(포지션 수)·40(루프 종료 오프셋)은 배열 길이/stride 성격 — 판정 노브 아님 | 4 |  |

<details><summary>`closed` 1건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

| # | 물음 | 닫은 근거 |
|---|---|---|
| 0 | version 인자는 dbg_value 외 사용처 0 — 미사용 확정(본체·aux 4조각 모두) | 본문에 해소 표기가 있다 |
</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

