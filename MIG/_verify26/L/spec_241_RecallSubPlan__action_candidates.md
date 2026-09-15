---

### `241` RecallSubPlan::action_candidates — 귀환 서브플랜 후보군: Recall 1개 + battle_action(아군 대상 스킬 제거) + attack_summon_action 을 순서대로 이어붙인다

| 항목 | 값 |
|---|---|
| id | `recall__Recall__action_candidates` |
| 심볼 | `_RNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy8sub_plan6recallNtB2_13RecallSubPlan17action_candidates` |
| 소스 | `game-ai\src\plan_legacy\sub_plan\recall.rs:10` |
| IR | `m02.ll` 40152~40314행 |
| 경로·가시성 | `game_ai::plan_legacy::sub_plan::RecallSubPlan::action_candidates` · **pub** |
| 계층 | 기타 |
| exe | `cc5ca0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r17` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&mut game_ai::plan_legacy::sub_plan::RecallSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay>
```

**TLS 메모 절(`sig.tls` · 정정 경로 `/specs[241]/sig/tls/<키>`)**

없음 — 본 함수 본문·aux 클로저에 `@anon.* = constant ptr @<KEY…call_once>` fn-포인터 상수 참조 0 · LocalKey::with 0

<details><summary>인자 7개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | bumpalo::Vec<SmallActionPlay> (32B) | IR 속성 sret([32 x i8]) writeonly dereferenceable(32). 레이아웃 +0 ptr(빈 벡터면 dangling 8) · +0x8 bump(&Bump) · +0x10 cap · +0x18 len — 4워드 전부 live(m02.ll:40172~40177 초기화 후 40303 memcpy 32B) | 4 |
| 1 | 1 | self | &mut RecallSubPlan (0B ZST) | 소스는 `&mut self`(tcx sig) 지만 RecallSubPlan 은 필드 0 의 ZST 라 IR 속성 `readnone` — 쓰기 표면 없음(writes 비어 있음이 정답) | 3 |
| 2 | 2 | version | usize | 본 함수에서 분기 없음 · battle_action 1번 인자로 그대로 전달(m02.ll:40228) | 4 |
| 3 | 3 | rnd | &mut StdRng (320B) | IR 속성 없음(readonly 아님 = &mut). 본 함수 안 gen_range 호출 사이트 0 · battle_action 2번 인자로 통과만(m02.ll:40228) | 4 |
| 4 | 4 | player | &PlayerState (2528B) | IR readonly | 4 |
| 5 | 5 | data | &OperationData (24B) | IR readonly | 4 |
| 6 | 6 | _parameter | &ScoreParameter (5384B) | IR readonly · DI 이름 `_parameter`(m02.ll:40164) — 본문에서 읽지 않음 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn action_candidates(&mut self, version, rnd, player, data, _parameter) -> Vec<SmallActionPlay>  [recall.rs:10]
  bump = data.context.pool                                                   [L11 · m02.ll:40168~40170]
  res  = Vec::new_in(bump)                                                    [L11 · 40172~40177]
  res.push(SmallActionPlay::Recall(SmallActionRecall::new(data, player, 5)))  [L12 · 40180 new(sret136) → 40189 memcpy136 → 40191 tag=4 → 40206 reserve(0,1) → 40223 memcpy184 → 40225 len+=1]
  battle = fight_check::battle_action(version, rnd, player, data, 5)          [L13 · 40228 · sret 32B 지역 %10]
  champ  = data.cache.player_champion[player.info.team][player.info.position].unwrap()   [L18 · 40232~40261 · team<2 바운드체크 → None 이면 unwrap_failed(40278)]
  battle.retain(|a| {                                                          [L19 · 40266~40274 · 캡처 (game.data, game.vtable, champ)]
      match a.tag { Skill|Skill2|Ult => {                                      [aux m01.ll:13914~13916 · Attack/기타는 항상 keep]
          match game.get_entity_by_id(a.target) {                              [L21 · m01.ll:13920~13921 · vtable+0x1f0]
              None    => true(keep)                                            [13925~13926 → %50]
              Some(t) => t.team != champ.team    // Team 파생 PartialEq: 판별자 같고 (Player 면 payload 도 같음) → false(delete)   [13936~13958]
          } }
          _ => true(keep) }
  })
  res.extend(battle)                                                           [L25 · 40283~40286 · (ptr,len) 를 Extend::extend 에 전달]
  res.extend(fight_check::attack_summon_action(player, data))                 [L26 · 40291~40298]
  return res                                                                   [L27 · 40303 memcpy 32B → sret]

결과 벡터 순서 = [Recall] ++ battle_action(아군/자기 팀 대상 Skill·Skill2·Ult 제거 후) ++ attack_summon_action. self 는 ZST — 상태 변경 없음. rnd 는 battle_action 에만 통과(본문 gen_range 0). 언와인드 시 res(20 블록)·battle(68 블록) drop_glue 만(cleanup).
```

**`mem` 메모리 접근 15건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | OperationData | 0x8 | context | r | &GameContext (m02.ll:40168) | 4 | OK |  |
| 1 | GameContext | 0x0 | pool | r | &Bump — Vec::new_in 의 allocator (m02.ll:40170 · 오프셋 0 이라 gep 없음) | 4 | OK |  |
| 2 | PlayerState | 0x930 | info.team | r | player_champion 1차 인덱스 · `< 2` 바운드체크(m02.ll:40232~40238) | 4 | OK |  |
| 3 | PlayerState | 0x9c0 | info.position@tag | r | i32 → zext · player_champion 2차 인덱스 (m02.ll:40251~40253 · player.rs:581 인라인) | 4 | OK |  |
| 4 | OperationData | 0x0 | cache | r | &AbstractGameWithCache (m02.ll:40254) | 4 | OK |  |
| 5 | AbstractGameWithCache | 0x1e0 | player_champion[team][pos] | r | [2][5] Option<&Entity> (니치 null=None) → unwrap (m02.ll:40255~40261, 40278) | 4 | OK |  |
| 6 | AbstractGameWithCache | 0x0 | game.data_ptr / game.vtable_ptr(+0x8) | r | &dyn AbstractGame 팻포인터 두 워드를 클로저 캡처로 복사(m02.ll:40266~40271) — 클로저에서 vtable+0x1f0 = get_entity_by_id(divtable AbstractGame 0x1f0) | 3 | OK |  |
| 7 | SmallActionPlay(aux 클로저) | 0xb1 | 태그 | r | m01.ll:13905~13916 · `tag-16 <u 3` = Skill16/Skill2 17/Ult18 만 술어 검사 대상 | 4 | OK |  |
| 8 | SmallActionSkill/Skill2/Ult(aux 클로저) | 0x8 | target | r | 엔티티 id(usize) → get_entity_by_id (m01.ll:13903~13904, 13921) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 9 | Entity(aux 클로저) | 0x0 | team@tag | r | 대상 엔티티와 champ 의 Team 판별자 비교 (m01.ll:13936~13941) | 4 | OK |  |
| 10 | Entity(aux 클로저) | 0x8 | team@Player.0 | r | 판별자 0(Player) 이면 페이로드 usize 비교 (m01.ll:13955~13957) | 4 | OK |  |
| 11 | bumpalo::Vec(sret/battle) | 0x18 | len | r | push/extend/retain 이 읽고 쓰는 길이 워드 (m02.ll:40216, 40285, 40297) | 4 | 확인불가(★모호: 동명 def_path 2개 [("bumpalo::collecti) |  |
| 12 | sret Vec<SmallActionPlay> | 0x0..0x20 | ptr/bump/cap/len | w | m02.ll:40172~40177 초기화 · 40225 len=1 · 40286/40298 extend(콜리가 len·ptr 갱신) · 40303 sret 로 memcpy 32B. self 쓰기는 없음(ZST·readnone) | 4 | 확인불가(tcx 사전에 타입 없음) | Vec::new_in(bump) → push(Recall) → extend×2 |
| 13 | Vec 원소[0] | 0xb1 | SmallActionPlay 태그 | w | m02.ll:40191 `store i8 4` @ %12+177 → 40223 memcpy 184B 로 원소 슬롯에 복사 | 4 | 확인불가(tcx 사전에 타입 없음) | 4 (Recall) |
| 14 | battle(지역 Vec, aux 클로저) | 0x18 | len | w | m01.ll:13849(0 으로 선저장) · 14093 최종 저장 · 삭제된 원소는 drop_glue(m01.ll:14081) — 지역 벡터라 외부 부작용 아님 | 4 | 확인불가(★모호: 동명 def_path 19개 [('game_ai::BattleR) | retain 후 new_len = old_len − 삭제수 |

**`consts` 상수 6건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 5 | 12 | 미상 | SmallActionRecall::new(data, player, end_delay=5) 의 end_delay(DI 인자명 m08.ll `end_delay` arg3) — 단위는 new 내부(미독) · 본 배치 범위 밖 | 4 |
| 1 | 5 | 13 | 미상 | battle_action(version, rnd, player, data, _end_delay=5) 의 5번 인자 — DI 이름 `_end_delay`(m15.ll !37147) 로 콜리에서 미사용(밑줄 접두) | 4 |
| 2 | 2 | 18 | 길이 | player_champion 1차 배열 길이(팀 수) 바운드체크 `team < 2` (m02.ll:40234) | 4 |
| 3 | 4 | 12 | 센티널 | SmallActionPlay::Recall 메모리 태그(니치 3+idx1) — m02.ll:40191 store i8 4 @+0xb1 | 4 |
| 4 | 16 | 19 | 미상 | aux 클로저 `tag-16 <u 3`: Skill(16)·Skill2(17)·Ult(18) 만 술어 대상(m01.ll:13914~13915) — Attack(15) 는 대상 아님 | 4 |
| 5 | 3 | 19 | 태그 | 위 `<u 3` 의 폭(세 variant) | 4 |

**`knobs` 조정점 2건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | Recall 액션 end_delay | recall.rs:12 (SmallActionRecall::new 3번 인자) | 5 | SmallActionRecall 내부 end_delay 로 저장(+0x60) — 의미·단위는 new/is_end 쪽(본 배치 범위 밖) · 값 변경 효과 미확인이라 knob 판정 보류 | 4 | 기존 |
| 1 | 귀환 중 제거하는 스킬 후보의 팀 조건 | recall.rs:19~21 (aux m01.ll:13936~13958) | t.team == champ.team 이면 제거 | 조건을 없애면 귀환 중에도 아군/자기 대상 Skill·Skill2·Ult 후보가 남아 score 경쟁에 참여한다(선택 여부는 score 층) | 4 | 기존 |

<details><summary>`callees` 피호출자 10건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | action_candidates | game_ai::plan_legacy::sub_plan::RecallSubPlan::action_candidates | pub | fn(&mut game_ai::plan_legacy::sub_plan::RecallSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\recall.rs:10 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | attack_summon_action | game_ai::attack_summon_action | pub | fn(&game_core::PlayerState, &game_core::OperationData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\fight_check.rs:836 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 2 | battle_action | game_ai::battle_action | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, usize) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\fight_check.rs:620 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 3 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 5개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 4 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 5개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 5 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 5개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 6 | new | game_ai::SmallActionRecall::new | pub | fn(&game_core::OperationData, &game_core::PlayerState, usize) -> game_ai::SmallActionRecall | game-ai\src\small_action\move_actions.rs:659 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 7 | push | <game_core::Staff as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::staff | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\staff.rs:38 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 6개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 8 | push | <game_core::Athlete as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:434 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 6개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 9 | push | <game_core::Contract as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:2498 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 6개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
</details>

⚠**미매칭 5개**: `battle`, `extend`, `reserve`, `reserve_internal_or_panic`, `retain`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m12.ll:35201) · **형제 6개** (RecallSubPlan)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::plan_legacy::sub_plan::RecallSubPlan as std::clone::Clone>::clone | pub | game-ai\src\plan_legacy\sub_plan\recall.rs:5 | True | fn(&game_ai::plan_legacy::sub_plan::RecallSubPlan) -> game_ai::plan_legacy::sub_plan::RecallSubPlan |
| 1 | <game_ai::plan_legacy::sub_plan::RecallSubPlan as std::fmt::Debug>::fmt | pub | game-ai\src\plan_legacy\sub_plan\recall.rs:5 | True | fn(&game_ai::plan_legacy::sub_plan::RecallSubPlan, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 2 | <game_ai::plan_legacy::sub_plan::RecallSubPlan as std::default::Default>::default | pub | game-ai\src\plan_legacy\sub_plan\recall.rs:5 | True | fn() -> game_ai::plan_legacy::sub_plan::RecallSubPlan |
| 3 | game_ai::plan_legacy::sub_plan::RecallSubPlan::action_candidates | pub | game-ai\src\plan_legacy\sub_plan\recall.rs:10 | False | fn(&mut game_ai::plan_legacy::sub_plan::RecallSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 4 | game_ai::plan_legacy::sub_plan::RecallSubPlan::calculate_score_parameter_value | pub | game-ai\src\plan_legacy\sub_plan\recall.rs:30 | True | fn(&game_ai::plan_legacy::sub_plan::RecallSubPlan, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::ScoreParameter) |
| 5 | game_ai::plan_legacy::sub_plan::RecallSubPlan::score | pub | game-ai\src\plan_legacy\sub_plan\recall.rs:42 | False | fn(&game_ai::plan_legacy::sub_plan::RecallSubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 |

**`open` 3건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | SmallActionRecall::new(end_delay=5) 의 5 가 틱 단위인지 초 단위인지 — new 본체(m08.ll:98371~) 는 본 배치 범위 밖(시그니처만 확인: (data,player,end_delay:usize) · initializes((69,70),(72,129))) | 4 |  |
| 1 | 표기 불가 | retain 술어의 `Team` 비교가 소스에서 `t.team != champ.team` 인지 `!(t.team == champ.team)` 인지 — 표기 불가(외연 동일 · IR 은 파생 PartialEq 인라인 + 분기 방향으로만 남음). 동작은 확정: 같은 팀이면 삭제 | 4 |  |
| 2 | 미탐색 | aux 클로저 m01.ll:14064 `tag == -1(0xff) → 루프 탈출` 은 retain 의 BackshiftOnDrop/drop 처리 아티팩트로 보이나 정확한 소스 대응(bumpalo vec.rs:2493 근처) 미확인 — 판정 로직과 무관 | 4 |  |

**`notes` 1건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | battle_action 5번 인자 5 의 원 의미 — 콜리 DI 이름이 `_end_delay`(미사용 인자)라 본 함수 동작엔 영향 없음. 소스 표기만 미확정 | 4 | 사실 서술 |

<details><summary>`closed` 1건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

| # | 물음 | 닫은 근거 |
|---|---|---|
| 0 | recall.rs:19~21 의 클로저 원문이 `matches!` 인지 `match` 인지 — 표기 불가. Attack(15) 가 술어 대상에서 빠진 것은 IR 확정(`tag-16 <u 3`) | 본문에 해소 표기가 있다 |
</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

