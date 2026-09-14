---

### `169` calculate_serpen_action_score — 세르펜 사냥 중 공격/스킬 대상 t 의 가치 = min(coef, coef*기대피해/t.hp) + (t 가 세르펜 ∧ 캠프 근처 아군>적 ? 5 : 0), coef 는 대상 종류(넥서스 200·타워 80·세르펜 0/20/200·미니언 -10·기타 0)

| 항목 | 값 |
|---|---|
| id | `serpen_hunt__calculate_serpen_action_score` |
| 심볼 | `_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy8sub_plan11serpen_hunt29calculate_serpen_action_score` |
| 소스 | `game-ai\src\plan_legacy\sub_plan\serpen_hunt.rs:914` |
| IR | `m02.ll` 65168~66027행 |
| 경로·가시성 | `game_ai::plan_legacy::sub_plan::serpen_hunt::calculate_serpen_action_score` · **in:game_ai::plan_legacy::sub_plan::serpen_hunt** |
| 계층 | 기타 |
| exe | `ccfbc0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r14` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &std::boxed::Box<dyn [Binder { value: Trait(game_core::Action), bound_vars: [] }] + 'static, std::alloc::Global>, &game_core::Effect, &game_core::Entity) -> i64
```

<details><summary>인자 7개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 2 | player | &PlayerState(2528B) | IR %0. info.team(+0x930)·info.position 태그(+0x9c0) → 내 챔피언 ; is_recent_visible_big_action 인자 | 4 |
| 1 | 3 | data | &OperationData(24B) | IR %1. cache(+0)→player_champion·player_state·game ; context(+8)→map(+0x20), expected_damage_target 인자 ; blackboard(+0x10) | 4 |
| 2 | 6 | effect | &Effect(56B) | IR %2. expected_damage_target 의 self 로만 전달(필드 직접 읽기 없음) | 4 |
| 3 | 7 | t | &Entity(1728B) | IR %3. 대상. hp(+0x670)·ty 태그(+0x68) 읽음 ; expected_damage_target 의 target | 4 |
| 4 | 1 | rnd | &mut StdRng | 소스 인자 — IR 에서 제거됨(미사용). write 0건 | 4 |
| 5 | 4 | parameter | &ScoreParameter | 소스 인자 — IR 에서 제거됨(미사용) | 4 |
| 6 | 5 | action | &Box<dyn Action> | 소스 인자 — IR 에서 제거됨(미사용) | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn calculate_serpen_action_score(_rnd, player, data, _parameter, _action, effect, t) -> i64   // serpen_hunt.rs:914 (IR 는 player,data,effect,t 4인자)
  let champ = data.cache.player_champion[player.info.team][player.info.position].unwrap();   // L916
  let value = effect.expected_damage_target(data.context, champ as &dyn AbstractEntity /*vtable @anon.54*/, t);   // L917 (65219) 기대 피해
  let hp = t.hp;                                                                                // L918
  let camp_pos = data.context.map.camp_pos(JungleType::Serpen /*5*/, true);                     // L920 (65226)
  // L924~926: near_camp_allys = cache.iter_champions(team).filter(|x| x.distance_sq(camp_pos) < 22500000000 /*closure$0 L923*/)
  //             .filter(|x| { let p = cache.player_by_champion_id(x.id).unwrap(); data.blackboard[p.info.team].in_serpen(p.info.position) } /*closure$1 L926*/).count()
  //   (65329~65559: 5슬롯 루프 · player_by_champion_id 는 10슬롯 id 대조로 인라인 · 못 찾으면 unwrap_failed)
  // L930~931: near_camp_enemies = cache.iter_champions(1-team).filter(|x| x.distance_sq(camp_pos) < 22500000000)
  //             .filter(|x| data.blackboard[team].is_recent_visible_big_action(cache.game, player, x) /*closure$2 L931*/).count()   (65562~65953, 5회 언롤)
  let coef = if t.ty.is_nexus() { 200 }                          // L933 (switch 65957: 태그 3)
    else if t.ty.is_tower() { 80 }                               // L935 (태그 2)
    else if t.ty.is_serpen() {                                   // L938 (태그 6)
      if near_camp_allys > near_camp_enemies { if hp > value { 20 } else { 200 } }   // L939 (65981, 65990)
      else { if hp > value { 0 } else { 200 } }                                       // L945 (65986)
    }
    else if t.ty.is_minion() { -10 }                             // L951 (태그 1)
    else { 0 };
  let base_score = if t.ty.is_serpen() && near_camp_allys > near_camp_enemies { 5 } else { 0 };   // L957 (66012~66015)
  min(coef, coef * value / hp) + base_score                      // L963~964 (66020~66022; hp==0 → div_by_zero 패닉)
```

**`mem` 메모리 접근 15건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | m02.ll:65189 (내) · 65525 (아군 player, 블랙보드 인덱스) | 4 | OK |
| 1 | PlayerState | 0x9c0 | info.position@tag | r | m02.ll:65201 (내) · 65541 (아군 player → in_serpen(position)) | 4 | OK |
| 2 | OperationData | 0x0 | cache | r | m02.ll:65204 | 4 | OK |
| 3 | OperationData | 0x8 | context | r | m02.ll:65217 → expected_damage_target 인자·map | 4 | OK |
| 4 | OperationData | 0x10 | blackboard (&[Blackboard;2], 원소 744B) | r | m02.ll:65231 → [player.team](65539, 65626) | 4 | OK |
| 5 | GameContext | 0x20 | map | r | m02.ll:65224 → camp_pos | 4 | OK |
| 6 | AbstractGameWithCache | 0x1e0 | player_champion[team][pos] / [team][0..5] / [1-team][0..5] | r | m02.ll:65207~65210 (내 챔프, None→unwrap_failed 65965) · 65287~65326 (player_by_champion_id 인라인: 10슬롯 전부) · 65566 (적 iter_champions) | 4 | OK |
| 7 | AbstractGameWithCache | 0x230 | player_state[t][p] | r | m02.ll:65296(+560)·65516~65518 — player_by_champion_id 결과, None→unwrap_failed(65531) | 4 | OK |
| 8 | AbstractGameWithCache | 0x0 | game.data_ptr | r | m02.ll:65567 → is_recent_visible_big_action | 4 | OK |
| 9 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r | m02.ll:65569 | 4 | OK |
| 10 | Entity | 0x670 | hp | r | t (m02.ll:65221) | 4 | OK |
| 11 | Entity | 0x68 | ty@tag (EntityType) | r | t (m02.ll:65955) 3=Nexus 2=Tower 6=Serpen 1=Minion | 4 | OK |
| 12 | Entity | 0x660 | x | r | 아군·적 챔피언 (m02.ll:65355, 65648 등) | 4 | OK |
| 13 | Entity | 0x668 | y | r | m02.ll:65359, 65652 등 | 4 | OK |
| 14 | Entity | 0x5c0 | id | r | 아군 챔피언 id(65384) ↔ player_champion[t][p].id(65299~65326) 대조 (player_by_champion_id 인라인 simulation.rs:1918~1920) | 4 | OK |

**`consts` 상수 13건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 5 | 920 | 태그 | JungleType::Serpen 태그(camp_pos 인자, m02.ll:65226) · L957 base_score 5 (66015) · 슬롯 수 5(65558 루프 상한, 판정 아님) | 4 |
| 1 | 22500000000 | 923 | 임계 | 150000² — 캠프 중심과의 distance_sq < 이것 = 150k 이내 챔피언만 근처로 셈 (아군 65380 · 적 65673 등 5회) | 4 |
| 2 | 200 | 933 | 산출값 | coef: 넥서스(65969 phi ←%303) · 세르펜인데 value>=hp(처치 가능)(65986, 65990) | 4 |
| 3 | 80 | 935 | 산출값 | coef: 타워 (65977) | 4 |
| 4 | 20 | 939 | 산출값 | coef: 세르펜 ∧ 근처 아군 > 적 ∧ hp > value (65990) | 4 |
| 5 | 0 | 945 | 태그 | coef: 세르펜 ∧ 아군 <= 적 ∧ hp > value (65986) · 기타 타입 0 (65998) · base_score 0 | 4 |
| 6 | -10 | 951 | 인덱스 | coef: 미니언 (65994) | 4 |
| 7 | 3 | 933 | 태그 | EntityType::Nexus 태그 (switch 65958) | 4 |
| 8 | 2 | 935 | 태그 | EntityType::Tower 태그 (switch 65959). ⚠65192·65527 의 2 는 팀 경계검사 | 4 |
| 9 | 6 | 938 | 태그 | EntityType::Serpen 태그 (switch 65960, 66012) | 4 |
| 10 | 1 | 951 | 태그 | EntityType::Minion 태그 (switch 65961) · 1-team (65564) · 루프 증분 | 4 |
| 11 | -1 | 963 | 태그 | sdiv 오버플로 검사 hp==-1 (66002) — 컴파일러 삽입 | 4 |
| 12 | -9223372036854775808 | 963 | 태그 | sdiv 오버플로 검사 i64::MIN (66003) — 컴파일러 삽입 | 4 |

**`knobs` 조정점 7건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 넥서스 대상 계수 | serpen_hunt.rs:933 (m02.ll:65969) | 200 | 올리면 세르펜 사냥 중에도 넥서스 공격을 더 선호 | 4 | 기존 |
| 1 | 타워 대상 계수 | serpen_hunt.rs:935 (m02.ll:65977) | 80 | 올리면 타워 공격 선호 | 4 | 기존 |
| 2 | 세르펜 처치 가능(value>=hp) 계수 | serpen_hunt.rs:939/945 (m02.ll:65986, 65990) | 200 | 막타 가능 시 세르펜 타격 가치. 내리면 스틸/막타 적극성 저하 | 4 | 기존 |
| 3 | 세르펜 우세(아군>적) 계수 | serpen_hunt.rs:939 (m02.ll:65990) | 20 | 캠프 근처 아군이 적보다 많을 때 세르펜을 때리는 가치. 0 이면 열세와 동일하게 안 때림 | 4 | 기존 |
| 4 | 미니언 대상 계수 | serpen_hunt.rs:951 (m02.ll:65994) | -10 | 더 음수로 하면 세르펜 사냥 중 미니언 공격을 더 억제 | 4 | 기존 |
| 5 | 세르펜 우세 기본 가산 | serpen_hunt.rs:957 (m02.ll:66015) | 5 | 올리면 우세 시 세르펜 타격이 다른 대상보다 우선 | 4 | 기존 |
| 6 | 캠프 근처 판정 반경 | serpen_hunt.rs:923/930 (m02.ll:65380, 65673) | 22500000000 | 150000². 올리면 더 먼 챔피언까지 아군/적 머릿수에 포함 | 4 | 기존 |

<details><summary>`callees` 피호출자 13건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | calculate_serpen_action_score | game_ai::plan_legacy::sub_plan::serpen_hunt::calculate_serpen_action_score | in:game_ai::plan_legacy::sub_plan::serpen_hunt | fn(&mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &std::boxed::Box<dyn [Binder { value: Trait(game_core::Action), bound_vars: [] }] + 'static, std::alloc::Global>, &game_core::Effect, &game_core::Entity) -> i64 | game-ai\src\plan_legacy\sub_plan\serpen_hunt.rs:914 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 1 | camp_pos | game_core::MapDef::camp_pos | pub | fn(&game_core::MapDef, game_core::JungleType, bool) -> (u64, u64) | game-core\src\simulation\map_def.rs:207 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 2 | distance_sq | game_core::utils::distance_sq | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:6 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
| 3 | distance_sq | game_core::Entity::distance_sq | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2157 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
| 4 | expected_damage_target | game_core::Effect::expected_damage_target | pub | fn(&game_core::Effect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + , &game_core::Entity) -> usize | game-core\src\simulation\effect.rs:91 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 5 | in_serpen | game_core::Blackboard::in_serpen | pub | fn(&game_core::Blackboard, usize) -> bool | game-core\src\simulation\game\blackboard.rs:183 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 6 | is_minion | game_core::EntityType::is_minion | pub | fn(&game_core::EntityType, game_core::LineType) -> bool | game-core\src\simulation\entity.rs:1256 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 7 | is_nexus | game_core::EntityType::is_nexus | pub | fn(&game_core::EntityType) -> bool | game-core\src\simulation\entity.rs:1399 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 8 | is_recent_visible_big_action | game_core::Blackboard::is_recent_visible_big_action | pub | fn(&game_core::Blackboard, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\game\blackboard.rs:367 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 9 | is_serpen | game_core::EntityType::is_serpen | pub | fn(&game_core::EntityType) -> bool | game-core\src\simulation\entity.rs:1373 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 10 | is_tower | game_core::EntityType::is_tower | pub | fn(&game_core::EntityType) -> bool | game-core\src\simulation\entity.rs:1385 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 11 | iter_champions | game_core::AbstractGameWithCache::<'a, 'b>::iter_champions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5759 ~ game_core[6a30]::simulation::{impl#5}::iter_champions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1904 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 12 | player_by_champion_id | game_core::AbstractGameWithCache::<'a, 'b>::player_by_champion_id | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> std::option::Option<&'a/#0 game_core::PlayerState> | game-core\src\simulation.rs:1915 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
</details>

⚠**미매칭 1개**: `llvm.smin.i64`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 3곳** (m02.ll:65084, m02.ll:65103, m02.ll:65126) · **형제 0개** 

**`open` 3건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 표기 불가 | 소스 인자 rnd·parameter·action 이 정말 본문 미사용인지는 IR(DeadArgElim 후)만으로 확정 — 소스에 `_` 접두인지 여부는 표기 불가. exe 0xccfbc0 의 실제 인자 수는 argscan 미실시 | 4 |  |
| 1 | 표기 불가 | value>=hp 판정이 `hp > value` 의 부정(icmp sgt, 65982)으로 컴파일됨 — 소스가 `value >= hp` 인지 `!(hp > value)` 인지 표기 불가(외연 동일) | 4 |  |
| 2 | 미탐색 | in_serpen·is_recent_visible_big_action 의 의미(블랙보드 세르펜 참여 플래그 / 최근 가시 대행동)는 이름 기반 추정 — 콜리 내부 안 팜 | 5 |  |

**`notes` 1건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | is_nexus/is_tower/is_serpen/is_minion 체인이 switch 로 접혀 L933/935/938/951 의 정확한 술어 이름은 dbg(1400 is_nexus 만 남음)로 부분 확인 — 태그값 기준 동작은 확정 | 4 | 사실 서술 |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

