---

### `68` base_sub_goal — BattlePlanGoal(TryKill/Support/Response/Avoid) → 기본 BattleSubPlanGoal 결정: 대상 추적(Trace) / 우물 위험이면 End / 가까운 적 있으면 KitingBack 없으면 RunAway

| 항목 | 값 |
|---|---|
| id | `battle__base_sub_goal` |
| 심볼 | `_RNvMs_NtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old6battleNtB4_14BattlePlanGoal13base_sub_goal` |
| 소스 | `game-ai\src\plan_legacy\old\battle.rs:69` |
| IR | `m10.ll` 29294~29997행 |
| 경로·가시성 | `game_ai::plan_legacy::old::BattlePlanGoal::base_sub_goal` · **pub** |
| 계층 | 레거시 플랜 |
| exe | `dff080` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r9` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_ai::plan_legacy::old::BattlePlanGoal, usize, &game_core::PlayerState, &game_core::OperationData) -> game_ai::plan_legacy::old::BattleSubPlanGoal
```

<details><summary>인자 4개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | self | &BattlePlanGoal(24B: tag i64@0, payload@8) | tag 0=TryKill(2필드) 1=Support(1필드) 2=Response 3=Avoid. payload+8 = 대상 엔티티 id(TryKill.0 / Support.0) | 4 |
| 1 | 1 | version | usize | is_enemy_well_danger 에 전달만. 이 본문에 버전 분기 없음 | 4 |
| 2 | 2 | player | &PlayerState(2528B) | info.team(0x930)·info.position@tag(0x9c0) | 4 |
| 3 | 3 | data | &OperationData(24B) | cache(+0)·blackboard(+0x10). context 미사용 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn base_sub_goal(&self, version, player, data) -> BattleSubPlanGoal
  team = player.info.team; enemy_team = 1 - team
  if self.tag < 2 {                                   // L70: TryKill{0: focus, ..} | Support{0: focus}
    focus = self.payload+8                            // L71
    target = game.get_entity_by_id(focus)             // L72 (vtable+0x1f0)
    // L73: if let Some(t) = target { if is_ignored_well_enemy(version, player, t) { return End } }   (fight_model.rs:754~756 인라인)
    //   is_ignored_well_enemy(t) = t.team == TeamType::Player(enemy_team) && is_enemy_well_danger(version, player, t.x, t.y)
    if target.is_some() && target.team@tag == 0 && target.team.0 == enemy_team
       && is_enemy_well_danger(version, player, target.x, target.y) → return End(7)
    return Trace{focus}(0)                            // L72 (target None 이거나 우물 위험 아님)
  }
  // Response | Avoid
  me = cache.player_champion[team][player.info.position].unwrap()     // L80 (None → unwrap_failed 패닉)
  nearest = cache.iter_champions(enemy_team)                            // L81 player_champion[1-team] 의 Some
      .filter(|e| data.blackboard[enemy_team].is_recent_visible(game, player, e)     // L82 (⚠ 적 팀 판 blackboard)
               && dist²(e, me) < 200000² + 1                                        // L83
               && !is_ignored_well_enemy(version, player, e))                       // L84 (= 적 챔피언이 우물 위험 안이면 제외)
      .min_by_key(|e| dist²(e, me))                                                  // L85 (첫 원소 인라인 + Map::fold)
  match nearest { Some(e) => KitingBack{focus: e.id}(3),   // L87
                  None    => RunAway(4) }                   // L86
```

**`mem` 메모리 접근 13건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | BattlePlanGoal | 0x0 | tag | r | L70 `tag < 2` (samesign ult, m10.ll:29307) → TryKill/Support 경로, 아니면 Response/Avoid 경로 | 4 | OK |
| 1 | BattlePlanGoal | 0x8 | TryKill.0 / Support.0 (대상 엔티티 id) | r | gep +8 (m10.ll:29313). get_entity_by_id 인자이자 Trace{focus} | 4 | OK |
| 2 | PlayerState | 0x930 | info.team | r | enemy_team = 1 - team (L73 인라인 fight_model.rs:755 / L81) | 4 | OK |
| 3 | PlayerState | 0x9c0 | info.position@tag | r | L80 my_champ = player_champion[team][position].unwrap() | 4 | OK |
| 4 | OperationData | 0x0 | cache | r | game(+0/+8)·player_champion | 4 | OK |
| 5 | OperationData | 0x10 | blackboard(&[Blackboard;2]) | r | L82: blackboard[1 - team] (적 팀 판) 을 is_recent_visible 의 self 로 (m10.ll:29466~29467) | 4 | OK |
| 6 | AbstractGameWithCache | 0x0 | game.data_ptr data/vtable | r | L72 vtable+0x1f0 = get_entity_by_id(focus) (m10.ll:29324, divtable 확인) | 3 | OK |
| 7 | AbstractGameWithCache | 0x1e0 | player_champion[2][5] | r | gep +480 (m10.ll:29394). [team][position]=나, [1-team][0..5]=적 후보 | 4 | OK |
| 8 | Entity | 0x0 | team@tag (TeamType: 0=Player 1=Neutral) | r | is_ignored_well_enemy 인라인(fight_model.rs:755): tag==0 | 4 | OK |
| 9 | Entity | 0x8 | team@Player.0 (팀 번호) | r | == 1 - player.team 일 때만 우물 위험 검사 | 4 | OK |
| 10 | Entity | 0x660 | x | r | is_enemy_well_danger(version, player, x, y)·거리² | 4 | OK |
| 11 | Entity | 0x668 | y | r | 동상 | 4 | OK |
| 12 | Entity | 0x5c0 | id | r | L87 KitingBack{focus: nearest.id} (m10.ll:29994) | 4 | OK |

**`consts` 상수 7건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 2 | 70 | 임계 | self.tag < 2 ⇔ TryKill(0)\|Support(1). (팀 bounds check 의 2 도 동일 리터럴) | 4 |
| 1 | 0 | 73 | 태그 | Entity.team@tag == 0 (TeamType::Player) — is_ignored_well_enemy 인라인 조건. 반환 tag 0 = Trace | 4 |
| 2 | 1 | 73 | 계수 | enemy_team = 1 - player.team (L73·L81) | 4 |
| 3 | 7 | 73 | 태그 | BattleSubPlanGoal::End 메모리태그 — 대상이 우물 위험 안의 적 챔피언이면 | 4 |
| 4 | 40000000001 | 83 | 임계 | 200000² + 1 — 적↔나 거리²(\|dx\|²+\|dy\|²) < 이 값 (200k 이내) 인 적만 후보 | 4 |
| 5 | 3 | 87 | 태그 | BattleSubPlanGoal::KitingBack 메모리태그 — 200k 내 가시 적 중 최근접 대상 | 4 |
| 6 | 4 | 86 | 태그 | BattleSubPlanGoal::RunAway 메모리태그 — 후보 적 없음 | 4 |

**`knobs` 조정점 2건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | KitingBack 대상 탐색 반경 | battle.rs:83 | 40000000001 (=200000²+1) | 키우면 더 먼 적을 KitingBack 대상으로 잡아 RunAway 대신 거리조절 이탈이 늘어남 | 4 | 기존 |
| 1 | TryKill/Support 의 End 전환 조건 | battle.rs:73 (is_ignored_well_enemy) | 대상이 적 우물 위험 지역 | is_enemy_well_danger 를 완화하면 우물 근처 적도 계속 Trace | 4 | 기존 |

<details><summary>`callees` 피호출자 8건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | base_sub_goal | game_ai::plan_legacy::old::BattlePlanGoal::base_sub_goal | pub | fn(&game_ai::plan_legacy::old::BattlePlanGoal, usize, &game_core::PlayerState, &game_core::OperationData) -> game_ai::plan_legacy::old::BattleSubPlanGoal | game-ai\src\plan_legacy\old\battle.rs:69 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 1 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 2 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 3 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 4 | is_enemy_well_danger | game_ai::is_enemy_well_danger | pub | fn(usize, &game_core::PlayerState, u64, u64) -> bool | game-ai\src\path_finder.rs:1032 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 5 | is_ignored_well_enemy | game_ai::plan_legacy::old::is_ignored_well_enemy | pub | fn(usize, &game_core::PlayerState, &game_core::Entity) -> bool | game-ai\src\plan_legacy\old\fight_model.rs:754 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 6 | is_recent_visible | game_core::Blackboard::is_recent_visible | pub | fn(&game_core::Blackboard, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\game\blackboard.rs:346 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 7 | iter_champions | game_core::AbstractGameWithCache::<'a, 'b>::iter_champions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5759 ~ game_core[6a30]::simulation::{impl#5}::iter_champions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1904 | True | True | 3 | IR 호출 심볼 일치(토큰 경계) |
</details>

**호출처 9곳** (m05.ll:10581, m05.ll:17733, m05.ll:26685, m05.ll:26788, m05.ll:26935, m05.ll:34428, m10.ll:13627, m10.ll:23253, m10.ll:27501) · **형제 5개** (BattlePlanGoal)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::plan_legacy::old::BattlePlanGoal as std::clone::Clone>::clone | pub | game-ai\src\plan_legacy\old\battle.rs:60 | True | fn(&game_ai::plan_legacy::old::BattlePlanGoal) -> game_ai::plan_legacy::old::BattlePlanGoal |
| 1 | <game_ai::plan_legacy::old::BattlePlanGoal as std::fmt::Debug>::fmt | pub | game-ai\src\plan_legacy\old\battle.rs:60 | True | fn(&game_ai::plan_legacy::old::BattlePlanGoal, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 2 | <game_ai::plan_legacy::old::BattlePlanGoal as std::cmp::PartialEq>::eq | pub | game-ai\src\plan_legacy\old\battle.rs:60 | True | fn(&game_ai::plan_legacy::old::BattlePlanGoal, &game_ai::plan_legacy::old::BattlePlanGoal) -> bool |
| 3 | <game_ai::plan_legacy::old::BattlePlanGoal as std::cmp::Eq>::assert_fields_are_eq | pub | game-ai\src\plan_legacy\old\battle.rs:60 | True | fn(&game_ai::plan_legacy::old::BattlePlanGoal) |
| 4 | game_ai::plan_legacy::old::BattlePlanGoal::base_sub_goal | pub | game-ai\src\plan_legacy\old\battle.rs:69 | False | fn(&game_ai::plan_legacy::old::BattlePlanGoal, usize, &game_core::PlayerState, &game_core::OperationData) -> game_ai::plan_legacy::old::BattleSubPlanGoal |

**`open` 5건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | is_enemy_well_danger(path_finder.rs:1032) 내부 미탐색 — '우물 위험' 의 구체 반경/조건은 이 명세 범위 밖 | 4 |  |
| 1 | 미탐색 | blackboard 인덱스가 1-team(적 팀 판)인 이유 — evaluate_gank_opportunity_with_score 에서도 같은 패턴. Blackboard 갱신 코드 미탐색이라 의미(적 팀 판에 '적이 우리에게 보였던 시각' 이 기록되는지) 미확정 | 4 |  |
| 2 | 미탐색 | Support 경로에서 payload.0 이 아군 id 라면 is_ignored_well_enemy 는 team 불일치로 항상 false → Trace{ally}. Support.0 이 아군인지 적인지는 생성 지점 미탐색 | 4 |  |
| 3 | 표기 불가 | L86/L87 의 소스 형태(match vs map_or)는 표기 불가(외연 동일) | 4 |  |
| 4 | 미탐색 | get_entity_by_id 가 vtable+0x1f0 임은 divtable 로 확인(정적 vtable 전역 기준). 런타임 Arc<dyn> 이면 다를 수 있음 — 이 함수는 cache.game(&dyn AbstractGame) 정적 참조라 해당 없음으로 봄 | 3 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

