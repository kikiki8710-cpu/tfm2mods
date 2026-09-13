---

### `45` can_recall — 지금 귀환(recall)해도 안전한가 — 최근 피격·주변 적·은신·샘까지 거리로 판정, 아니면 샘 안인가

| 항목 | 값 |
|---|---|
| id | `utils__can_recall` |
| 심볼 | `_RNvNtCshdEBA0ozCnw_7game_ai5utils10can_recall` |
| 소스 | `game-ai\src\utils.rs:350` |
| IR | `m04.ll` 46695~46881행 |
| 경로·가시성 | `game_ai::can_recall` · **pub** |
| 계층 | 기타 |
| exe | `d36480` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r8` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bool
```

<details><summary>인자 3개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | rnd | &mut StdRng(320B, align16) | 본문에서 직접 안 씀. find_nearest_entity 두 번 호출에 그대로 전달 | 4 |
| 1 | 2 | player | &PlayerState(2528B) | info.id(+0x928)·info.team(+0x930) 만 읽는다 | 4 |
| 2 | 3 | data | &OperationData(24B) | {+0x0 cache:&AbstractGameWithCache, +0x8 context:&GameContext}. cache+0x0/+0x8 = &dyn AbstractGame (data, vtable) | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn can_recall(rnd, player, data) -> bool

[L351] game = data.cache(+0x0) 의 &dyn AbstractGame (data ptr = cache+0x0, vtable = cache+0x8)
[L352] champ = game.get_player_champion(player.info.id(+0x928))   // vtable+0x1c8, Option<&Entity>; None → unwrap_failed(utils.rs:352 패닉)
[L354] nearest_visible_enemy = game.find_nearest_entity(rnd, data.context.pool(+0x0), champ.x(+0x660), champ.y(+0x668), 200000,
         |e| e.team(+0x0) != champ.team && data.can_target(game, player, e)
             && (if e.ty(+0x68)==4 /*Jungle*/ { e.ty.Jungle.info.focused(+0x88 tag ==1 && +0x90) == Some(champ.id(+0x5c0)) }
                 else { data.can_target(game, player, e) /* L360: 같은 심볼을 한 번 더 호출 */ }))
       // {i64,i64} 반환, tag(+0)==1 이면 Some
[L365] nearest_enemy = game.find_nearest_entity(rnd, pool, champ.x, champ.y, 20000,
         |e| e.team != champ.team && data.can_target(game, player, e)
             && (if e.ty==4 /*Jungle*/ { focused == Some(champ.id) } else { true }))
[L375] team = player.info.team(+0x930)
       healp = if team == 0 { (32000, 928000) } else { (928000, 32000) }     // (x, y)
[L381] (lx, ly, rx, ry) = data.context.map(+0x20).fountains(+0x6d70)[team]   // team>=2 → panic_bounds_check
[L383] dist_to_heal_area = game_core::utils::distance(champ.x, champ.y, healp.x, healp.y)
[L385] time = dist_to_heal_area / champ.stat_cached.move_speed(+0x640)     // speed 0 → div_by_zero 패닉
[L388] rec = game_core::simulation::battle_recency(game, vtable, player.info.id)   // 32B = (Option<usize>, Option<usize>)
[L389] recent_attacked = rec.1(+0x10 tag).is_some_and(|t| t + 60 >= game.tick() /*vtable+0x28*/)   // closure#2
       //  IR: %76 = (t+60 < tick) ; br %76 → 계속, 아니면 → [L394]. None 이면 무조건 계속(=안 맞음)
[L391] if !recent_attacked {
[L392]   if champ.invisible_tick(+0x698) == 0 || champ.invisible_tick >= data.context.setting(+0x8).return_tick(+0x1368) {
[L393]     if nearest_visible_enemy.is_none() && nearest_enemy.is_none() && time > 210 { return true }
         }
       }
[L394] return lx <= champ.x && champ.x <= rx && ly <= champ.y && champ.y <= ry     // 샘 사각형 안이면 true

※ 분기 순서 근거: %69/%77 → %78(invisible) → %90(적·time) → 실패 시 전부 %82(사각형). !dbg 줄 389→392→393→394.
※ 부작용 없음(게임 구조체 store 0). rnd 는 find_nearest_entity 에 &mut 로 넘어간다.
```

**`mem` 메모리 접근 23건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | OperationData | 0x0 | cache | r | &AbstractGameWithCache. 그 선두 16B 가 &dyn AbstractGame (data ptr, vtable ptr) | 4 | OK |
| 1 | OperationData | 0x8 | context | r | &GameContext | 4 | OK |
| 2 | AbstractGameWithCache | 0x0 | game.data_ptr | r | dyn AbstractGame 데이터 포인터 | 4 | OK |
| 3 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r | dyn AbstractGame vtable(816B). 슬롯 0x1c8=get_player_champion, 0x110=find_nearest_entity, 0x28=tick (divtable 일치율 98%) | 3 | OK |
| 4 | GameContext | 0x0 | pool | r | &bumpalo::Bump — find_nearest_entity 3번째 인자 | 4 | OK |
| 5 | GameContext | 0x8 | setting | r | &GameSetting | 4 | OK |
| 6 | GameContext | 0x20 | map | r | &MapDef — fountains 사각형 출처 | 4 | OK |
| 7 | GameSetting | 0x1368 | return_tick | r | 귀환 채널 틱 수. invisible_tick 과 비교 | 4 | OK |
| 8 | MapDef | 0x6d70 | fountains[team] | r | [(lx,ly,rx,ry);2] u64 4개 = 32B stride. gepS 인덱스 = player.info.team (>=2 면 panic_bounds_check, map_def.rs:235 인라인) | 4 | OK |
| 9 | PlayerState | 0x928 | info.id | r | get_player_champion(id) 인자 | 4 | OK |
| 10 | PlayerState | 0x930 | info.team | r | 0 이면 샘 좌표 (32000,928000), 아니면 (928000,32000). fountains 인덱스 | 4 | OK |
| 11 | Entity(champ) | 0x640 | stat_cached.move_speed | r | 0 이면 panic_const_div_by_zero (utils.rs:385). time = dist / move_speed | 4 | OK |
| 12 | Entity(champ) | 0x660 | x | r | find_nearest_entity 중심·distance·샘 사각형 판정 | 4 | OK |
| 13 | Entity(champ) | 0x668 | y | r |  | 4 | OK |
| 14 | Entity(champ) | 0x698 | invisible_tick | r | !=0 && < return_tick 이면 '안전' 경로를 건너뛰고 샘 사각형 판정만 한다 | 4 | OK |
| 15 | Entity(champ) | 0x5c0 | id | r | aux 클로저에서 (other+1472) 정글 focused 와 비교 | 4 | OK |
| 16 | Entity(e, 클로저 인자) | 0x0 | team@tag | r | aux: TeamType 16B enum {tag +0x0, Player 페이로드 +0x8}. e.team != champ.team 판정(PartialEq 인라인) | 4 | OK |
| 17 | Entity(e, 클로저 인자) | 0x8 | team@Player.0 | r | aux: 둘 다 Player 이면 팀 번호 비교 | 4 | OK |
| 18 | Entity(e, 클로저 인자) | 0x68 | ty@tag | r | aux: ==4 → EntityType::Jungle | 4 | OK |
| 19 | Entity(e, 클로저 인자) | 0x88 | ty@Jungle.info.focused@tag | r | aux: Option<usize> 태그(1=Some) | 4 | OK |
| 20 | Entity(e, 클로저 인자) | 0x90 | ty@Jungle.info.focused@Some.0 | r | aux: 정글몹이 노리는 엔티티 id. champ.id 와 같아야 '적'으로 센다 | 4 | OK |
| 21 | battle_recency 반환(32B, 스택) | 0x10 | .1 tag (Option<usize>, 피격 최신틱) | r | _docs: '(관여 최신틱, 피격 최신틱). 기록 없으면 None'. +0x10 태그, +0x18 값 | 4 | 확인불가(tcx 사전에 타입 없음) |
| 22 | battle_recency 반환(32B, 스택) | 0x18 | .1 value (last_attacked_tick) | r |  | 4 | 확인불가(tcx 사전에 타입 없음) |

**`consts` 상수 12건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 200000 | 354 | 미상 | find_nearest_entity 반경(제곱 아님, 좌표단위 ≈6.25셀) — '보이는/노리는 적' 탐색 반경 (nearest_visible_enemy) | 4 |
| 1 | 20000 | 365 | 미상 | find_nearest_entity 반경(≈0.6셀) — 정글몹 포함 '어떤 적이든' 근접 탐색 반경 (nearest_enemy) | 4 |
| 2 | 928000 | 375 | 산출값 | 샘(heal area) 좌표 성분. team0 → (x=32000,y=928000), 그 외 → (x=928000,y=32000). 29셀 = 맵 반대편 구석 | 4 |
| 3 | 32000 | 375 | 산출값 | 샘 좌표 성분(=1셀). 위와 짝 | 4 |
| 4 | 2 | 381 | 임계 | MapDef.fountains 배열 길이 2 — team>=2 면 panic_bounds_check(map_def.rs:235) | 4 |
| 5 | 0 | 375 | 태그 | team == 0 판정(select) 으로 샘 좌표 고른다 | 4 |
| 6 | 60 | 389 | 계수 | recent_attacked = last_attacked_tick + 60 >= now_tick — 최근 60틱(1초@60tps, 리터럴이라 tps 무관) 안에 맞았는가 | 4 |
| 7 | 0 | 392 | 태그 | invisible_tick == 0 이면 은신 게이트 통과 | 4 |
| 8 | 1 | 393 | 태그 | Option<..> Some 태그 — nearest_visible_enemy / nearest_enemy 의 is_none() 을 tag != 1 로 판정 | 4 |
| 9 | 210 | 393 | 임계 | 샘까지 걸어가는 데 걸리는 틱(dist/move_speed) > 210(3.5초@60tps) 이어야 귀환이 이득 → true. 이하면 걸어가라(샘 안 여부만 반환) | 4 |
| 10 | 4 | 356 | 태그 | aux closure: EntityType 메모리태그 4 = Jungle. 정글몹은 focused==champ.id 일 때만 적으로 센다 | 4 |
| 11 | 0 | 355 | 태그 | aux closure: TeamType 태그 0 = Player(usize) — 둘 다 Player 면 페이로드(팀번호) 비교 | 4 |

**`knobs` 조정점 6건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | '보이는 적' 탐색 반경 | utils.rs:354 (m04.ll:46736 `i64 200000`) | 200000 | 올리면 더 먼 적이 있어도 귀환을 안 한다(보수적). 내리면 가까운 적만 위협으로 본다 | 4 | 기존 |
| 1 | '어떤 적이든' 근접 탐색 반경 | utils.rs:365 (m04.ll:46752 `i64 20000`) | 20000 | 정글몹 포함 근접 적 체크 반경. 올리면 더 넓게 훑어 귀환을 더 자주 막는다 | 4 | 기존 |
| 2 | 최근 피격 창 | utils.rs:389 (m04.ll:46828 `add i64 %71, 60`) | 60 | 피격 후 이 틱 안이면 귀환 불가. 올리면 맞은 뒤 더 오래 귀환을 참는다. tps 와 무관한 리터럴 | 4 | 기존 |
| 3 | 귀환 이득 거리 임계(틱) | utils.rs:393 (m04.ll:46865 `icmp ugt i64 %64, 210`) | 210 | 샘까지 걸어가는 시간이 이보다 길어야 귀환 true. 내리면 샘 근처에서도 귀환한다. 올리면 웬만하면 걸어간다 | 4 | 기존 |
| 4 | 은신 게이트 기준 | utils.rs:392 (m04.ll:46875 `icmp ult i64 %80, %100`) — GameSetting.return_tick(+0x1368) | return_tick | invisible_tick 이 0 이 아니면서 return_tick 미만이면 안전 경로 차단. return_tick 을 내리면 이 차단이 빨리 풀린다 | 4 | 기존 |
| 5 | 샘 좌표 하드코딩 | utils.rs:375 (m04.ll:46760~46762 select 928000/32000) | (32000,928000)\|(928000,32000) | MapDef.fountains 와 별개로 거리 계산은 이 고정 좌표를 쓴다. 맵을 바꾸면 여기가 어긋난다 | 4 | 기존 |

<details><summary>`callees` 피호출자 17건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | battle_recency | game_core::battle_recency | pub | fn(&dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , usize) -> (std::option::Option<usize>, std::option::Option<usize>) | game-core\src\simulation.rs:937 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | can_recall | game_ai::can_recall | pub | fn(&mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bool | game-ai\src\utils.rs:350 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 2 | can_target | game_core::Entity::can_target | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1477 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 3 | can_target | game_core::OperationData::<'a, 'b>::can_target | pub | fn(&game_core::OperationData<'a/#0, 'b/#1>, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\state\player.rs:23 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 4 | distance | game_core::utils::distance | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:12 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 5 | find_nearest_entity | game_core::World::find_nearest_entity | pub | fn(&game_core::World, &mut rand::rngs::std::StdRng, &bumpalo::Bump, u64, u64, u64, F/#0) -> std::option::Option<usize> | game-core\src\simulation\game\data.rs:544 | True | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 6 | find_nearest_entity | game_core::AbstractGame::find_nearest_entity | pub | fn(&Self/#0, &mut rand::rngs::std::StdRng, &bumpalo::Bump, u64, u64, u64, &dyn [Binder { value: Trait(std::ops::Fn<(&game_core::Entity,)>), bound_vars: [Region(BrNamed(DefId(14:109499 ~ game_core[6a30]::simulation::AbstractGame::find_nearest_entity::)))] }, Binder { value: Projection(ExistentialProjection { def_id: DefId(2:4448 ~ core[e0bd]::ops::function::FnOnce::Output), args: [(&game_core::Entity,)], term: Term::Ty(bool), .. }), bound_vars: [Region(BrNamed(DefId(14:109499 ~ game_core[6a30]::simulation::AbstractGame::find_nearest_entity::)))] }] + ) -> std::option::Option<usize> | game-core\src\simulation.rs:128 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 7 | find_nearest_entity | <game_core::Game as game_core::AbstractGame>::find_nearest_entity | pub | fn(&game_core::Game, &mut rand::rngs::std::StdRng, &bumpalo::Bump, u64, u64, u64, &dyn [Binder { value: Trait(std::ops::Fn<(&game_core::Entity,)>), bound_vars: [Region(BrNamed(DefId(14:105944 ~ game_core[6a30]::simulation::game::{impl#4}::find_nearest_entity::)))] }, Binder { value: Projection(ExistentialProjection { def_id: DefId(2:4448 ~ core[e0bd]::ops::function::FnOnce::Output), args: [(&game_core::Entity,)], term: Term::Ty(bool), .. }), bound_vars: [Region(BrNamed(DefId(14:105944 ~ game_core[6a30]::simulation::game::{impl#4}::find_nearest_entity::)))] }] + ) -> std::option::Option<usize> | game-core\src\simulation\game.rs:1812 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 8 | get_player_champion | game_core::AbstractGame::get_player_champion | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:169 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 9 | get_player_champion | <game_core::Game as game_core::AbstractGame>::get_player_champion | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3621 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 10 | get_player_champion | <game_core::SingleLaneGame as game_core::AbstractGame>::get_player_champion | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3957 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 11 | team | game_view::ClientData::team | pub | fn(&game_view::ClientData, usize) -> std::option::Option<std::cell::Ref< game_core::Team>> | game-view\src\logic\client\data.rs:1651 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 12 | team | game_view::ClientDatabase::team | pub | fn(&game_view::ClientDatabase, usize) -> std::option::Option<&game_core::Team> | game-view\src\logic\client\data.rs:968 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 13 | team | game_core::PlayerAiContext::<'a, 'b, 'r>::team | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:583 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 14 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 15 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 16 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
</details>

⚠**미매칭 8개**: `cache`, `focused`, `fountains`, `invisible_tick`, `move_speed`, `pool`, `return_tick`, `setting`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m14.ll:7171) · **형제 0개** 

**`open` 7건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | closure#0(L354~362) 에서 정글이 아닌 엔티티에 대해 L355 와 L360 이 같은 심볼 OperationData::can_target 을 두 번 호출한다 — 소스에서 다른 함수(예: 가시성 판정)가 can_target 으로 인라인/별칭된 것인지, 진짜 중복 호출인지 IR 만으로 구분 못 함. 동작상 결과는 같다(순수 함수 가정) | 4 |  |
| 1 | 미탐색 | invisible_tick(+0x698) 의 의미(은신 시작 틱인지 은신 지속 틱인지) — game_ai IR·_docs 에 없음. 관측 사실: `!=0 && < return_tick` 이면 안전 경로 차단. 미탐색 = _gcbc 의 store 지점 | 4 |  |
| 2 | 미탐색 | battle_recency 반환 32B 의 필드 이름 — tcx 에 타입 없음(튜플). _docs game_core 292행 '(관여 최신틱, 피격 최신틱) 없으면 None' 을 근거로 .1(+0x10/+0x18) = 피격 최신틱으로 읽음. 본문(_gcbc/g15.ll:131139) 은 안 봄 | 3 |  |
| 3 | 미탐색 | find_nearest_entity 의 반환 {i64,i64} 가 Option<usize>(id) 인지 Option<&Entity> 팻 튜플인지 — tag(+0)==1 을 Some 으로 쓰는 것만 확인. 본체(_gcbc/g15.ll:180061) 안 봄 | 4 |  |
| 4 | 미탐색 | OperationData::can_target 내부(_gcbc/g15.ll:66255) 안 봄 — 시야/타겟가능 판정으로 추정 | 4 |  |
| 5 | 미탐색 | can_recall 은 tcx 상 p=game_ai::can_recall v=pub(루트 재수출) — 오라클 실행 가능. 이번엔 안 돌림 | 2 |  |
| 6 | 미탐색 | closure#0/#1 의 call_mut 심(m04.ll 15886~15975 / 15978~16063) 은 클로저 본체의 복제라 aux 에서 뺐다 | 4 |  |

<details><summary>`closed` 1건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

| # | 물음 | 닫은 근거 |
|---|---|---|
| 0 | ★exe 3개 주소 대응(fnprobe 기준, Ghidra 미사용): 0xd36480 = can_recall 본체 확정(패닉 Location utils.rs:352:56·385:14, 상수 0x30d40=200000·0x4e20=20000·0xe2900=928000·0x7d00=32000, vtable+0x1c8 호출). 0xd41600·0xe2e560 은 Location 이 utils.rs:314/327/332 ⟹ tcx 상 game_ai::line_backfight_support_focus::{closure#3}(utils.rs:311~342) 의 두 복제본이지 can_recall 이 아니다(오라벨 추정). can_recall0/can_recalls_0 의 exe 짝은 미확정 — fnprobe 는 lea 로 넘기는 fn 포인터를 안 찍으므로 미탐색 = 0xd36480 안의 lea 대상 2개 | 본문에 해소 표기가 있다 |
</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

