---

### `41` fight_participants — 교전 참여 아군 목록: 근처 아군 전원 + (전투 선언·묶임·6초내 도착) 원거리 아군을 (엔티티, 도착틱, 묶임) 로 모은다

| 항목 | 값 |
|---|---|
| id | `fight_model__fight_participants` |
| 심볼 | `_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old11fight_model18fight_participants` |
| 소스 | `game-ai\src\plan_legacy\old\fight_model.rs:610` |
| IR | `m10.ll` 39411~39912행 |
| 경로·가시성 | `game_ai::plan_legacy::old::fight_model::fight_participants` · **in:game_ai** |
| 계층 | 레거시 플랜 |
| exe | `e04f50` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r8` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &mut rand::rngs::std::StdRng, &game_core::OperationData, &game_core::PlayerState, &game_core::Entity, &[&game_core::Entity], &[&game_core::Entity], &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< (&game_core::Entity, i64, bool)>
```

<details><summary>인자 10개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | bumpalo Vec<(&Entity, i64, bool)>(32B: +0x0 ptr, +0x8 bump, +0x10 cap, +0x18 len; 원소 24B stride) | IR %0 | 4 |
| 1 | 1 | version | usize | IR %1. 본문 분기 없음 — ally_is_bound 로 전달만 | 4 |
| 2 | 2 | rnd | &mut StdRng(320B) | IR %2. ally_is_bound 로 전달만 | 4 |
| 3 | 3 | data | &OperationData(24B) | IR %3. cache(+0x0)·context(+0x8)·blackboard(+0x10) 전부 읽음 | 4 |
| 4 | 4 | player | &PlayerState(2528B) | IR %4. info.team(+0x930) 만 직접 읽음 | 4 |
| 5 | 5 | champ | &Entity(1728B) | IR %5. id(+0x5c0) 만 — 자기 자신 제외용 | 4 |
| 6 | 6 | near_allies | &[&Entity] (팻포인터, IR %6 ptr / %7 len) | 전원 무조건 out 에 들어간다 | 4 |
| 7 | 7 | fight_enemies | &[&Entity] (팻포인터, IR %8 ptr / %9 len) | ally_is_bound 인자 · focus 대조 · 최소거리 계산 대상 | 4 |
| 8 | 8 | team_plan | &TeamPlan(1064B) | IR %10. ally_battle_stop_tick[5](+0x0) 만 읽음 | 4 |
| 9 | 9 | debug | &mut DebugFrameData(224B) | IR %11. ally_is_bound 로 전달만 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn fight_participants(version, rnd, data, player, champ, near_allies, fight_enemies, team_plan, debug) -> Vec<(&Entity, i64, bool), &Bump>   // fight_model.rs:610~640
[L613] horizon = data.context.setting.tick_per_second(+0x12f8) * 6
[L614] out = Vec::new_in(data.context.pool)

// ── 1) 근처 아군은 전원 포함 ──
[L615] for a in near_allies {
[L616]   bound = if a.id == champ.id { false } else { ally_is_bound(version, rnd, data, player, a, fight_enemies, debug) }
[L617]   out.push((a, 0, bound)) }        // ⚠자기 자신이 near_allies 에 있어도 push 된다(bound=false)… 아래 unknown 참조

// ── 2) 원거리 아군(팀 챔프 5명 순회) ──
[L619] for i in 0..5 {
[L620]   if team_plan.ally_battle_stop_tick[i](+0x0, stride16).is_some() { continue }
[L621]   team = player.info.team(+0x930) (<2)   Some(a) = cache.player_champion(+0x1e0)[team][i] else continue
[L622]   if a.id == champ.id { continue }   if out.iter().any(|o| o.0.id == a.id) { continue }   // 이미 near 에 있음
[L623]   if ally_is_bound(version, rnd, data, player, a, fight_enemies, debug) { out.push((a, 0, true)); continue }
[L627]   declared = matches!(data.blackboard[team].big_goal[i].1, Some(BigGoal::Battle{focus: Some(f)}))   // +0xf8==5 && +0x100==1
                    && fight_enemies.iter().any(|e| e.id == f /*+0x108*/)
         if !declared { continue }
[L630]   sp = max(a.stat_cached.move_speed(+0x640), 1)
[L631]   reach = a.attack_effect(+0x4c0 != -1).map(|e| e.range() /*= a.stat_buff_cached.range(+0x438) + e.range(+0x4a0) + e.growth_range(+0x4a8)*(a.level(+0x5c8)-1)*/).unwrap_or(0)
[L632]   gap = fight_enemies.iter().map(|e| a.distance(e)).min()   // declared 로 비어있지 않음이 보장돼 첫 원소 무검사 로드
               .saturating_sub(reach)
[L633]   t = gap / sp
[L634]   if t > horizon { continue }
         out.push((a, t, false)) }
[L639] return out

※ 부작용: 게임 구조체 store 0. out 은 bump pool 할당. rnd/debug 는 ally_is_bound 에서 변할 수 있음.
```

**`mem` 메모리 접근 19건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | OperationData | 0x0 | cache | r | &AbstractGameWithCache | 4 | OK |
| 1 | OperationData | 0x8 | context | r | &GameContext. +0x0 pool(bump 할당자), +0x8 setting | 4 | OK |
| 2 | OperationData | 0x10 | blackboard | r | &[Blackboard;2]. [player.info.team] (744B stride, ≥2 면 panic_bounds_check) | 4 | OK |
| 3 | GameContext | 0x0 | pool: &Bump | r | out Vec 의 할당자 | 4 | OK |
| 4 | GameContext | 0x8 | setting: &GameSetting | r |  | 4 | OK |
| 5 | GameSetting | 0x12f8 | tick_per_second | r | horizon = tps*6 | 4 | OK |
| 6 | PlayerState | 0x930 | info.team | r | player_champion / blackboard 인덱스 | 4 | OK |
| 7 | AbstractGameWithCache | 0x1e0 | player_champion | r | [[Option<&Entity>;5];2] stride 40. null → skip | 4 | OK |
| 8 | TeamPlan | 0x0 | ally_battle_stop_tick[]: Option<usize>(16B, +0x0 tag) | r | tag != 0(Some) 이면 그 아군은 원거리 후보에서 제외 (fight_model.rs:620) | 4 | OK |
| 9 | Blackboard | 0xf8 | big_goal[i].1@tag (Option<BigGoal>, stride 32) | r | == 5 (Battle) 여야 함. IR gep 248+32*i | 4 | OK |
| 10 | Blackboard | 0x100 | big_goal[i].1@Battle.focus@tag | r | Option<usize> Some=1. IR 256+32*i | 4 | OK |
| 11 | Blackboard | 0x108 | big_goal[i].1@Battle.focus@Some.0 | r | focus 엔티티 id. fight_enemies 에 같은 id 가 있어야 declared. IR 264+32*i | 4 | OK |
| 12 | Entity | 0x5c0 | id | r | champ·아군·적 전부 id 비교 | 4 | OK |
| 13 | Entity(아군) | 0x640 | stat_cached.move_speed | r | sp = max(1, ·) | 4 | OK |
| 14 | Entity(아군) | 0x4c0 | attack_effect@tag (Option<Effect> 니치 i32) | r | == -1 None → reach 0 | 4 | OK |
| 15 | Entity(아군) | 0x4a0 | attack_effect.range | r | Effect::range 인라인 (effect.rs:26) | 4 | OK |
| 16 | Entity(아군) | 0x4a8 | attack_effect.growth_range | r | × (level-1) | 4 | OK |
| 17 | Entity(아군) | 0x5c8 | level | r |  | 4 | OK |
| 18 | Entity(아군) | 0x438 | stat_buff_cached.range | r | reach 에 가산 | 4 | OK |

**`consts` 상수 11건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 6 | 613 | 계수 | horizon = tick_per_second * 6 — 원거리 아군이 교전에 합류하는 데 허용하는 최대 시간 = 6초 | 4 |
| 1 | 0 | 617 | 태그 | near_allies 원소의 도착틱 = 0 (이미 근처) | 4 |
| 2 | 2 | 621 | 임계 | player_champion / blackboard 의 팀 인덱스 상한(panic_bounds_check len=2) | 4 |
| 3 | 5 | 619 | 임계 | 포지션 루프 0..5 | 4 |
| 4 | 0 | 620 | 태그 | team_plan.ally_battle_stop_tick[i] 의 None 태그 — None 일 때만 후보 | 4 |
| 5 | 5 | 627 | 태그 | BigGoal 태그 5 = Battle — 블랙보드에서 그 아군이 '전투'를 선언했어야 함 | 4 |
| 6 | 1 | 630 | 인덱스 | sp = max(move_speed, 1) — 0 나눗셈 방지 | 4 |
| 7 | -1 | 631 | 센티널 | attack_effect Option 니치 None(-1). None 이면 reach=0 | 4 |
| 8 | -1 | 26 | 태그 | Effect::range 인라인: growth_range * (level - 1) | 4 |
| 9 | 0 | 631 | 태그 | reach 의 unwrap_or(0) | 4 |
| 10 | 1 | 623 | 인덱스 | bound=true 로 push 되는 원거리 아군의 tuple.2 (도착틱 0) | 4 |

**`knobs` 조정점 4건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 원거리 아군 합류 허용 시간 (horizon) | fight_model.rs:613 (IR m10.ll:39461 `mul i64 %20, 6`) | 6 | tps*6 = 6초. 올리면 더 멀리 있는(전투 선언한) 아군도 참여자로 세어 교전 판단이 낙관적이 되고, 내리면 근처·묶인 아군만 센다 | 4 | 기존 |
| 1 | 원거리 아군 인정 조건 = 블랙보드 Battle 선언 + focus 가 fight_enemies 안 | fight_model.rs:627 (IR m10.ll:39732 `icmp eq i8 %123, 5`) | 5 | BigGoal::Battle(5) 외 태그를 허용하면 라인/정글 중인 아군까지 도착시간만으로 참여자가 된다 | 4 | 기존 |
| 2 | 도착 시간 계산의 사거리 공제 | fight_model.rs:631~633 (IR m10.ll:39855 usub.sat, 39857 udiv) | 0 | t = (최소 적거리 − 아군 사거리) / 이속. 사거리 공제를 없애면 원거리 딜러의 도착이 늦게 계산돼 참여자에서 더 자주 빠진다 | 4 | 기존 |
| 3 | 묶인(bound) 아군은 거리 무관 참여 | fight_model.rs:623 (IR m10.ll:39690 ally_is_bound → push (a,0,true)) | 1 | ally_is_bound 가 true 면 도착시간 검사 없이 참여자로 넣는다. 이 분기를 빼면 묶인 아군도 6초 규칙을 받는다 | 4 | 기존 |

<details><summary>`callees` 피호출자 15건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | ally_is_bound | game_ai::plan_legacy::old::fight_model::ally_is_bound | in:game_ai | fn(usize, &mut rand::rngs::std::StdRng, &game_core::OperationData, &game_core::PlayerState, &game_core::Entity, &[&game_core::Entity], &mut game_core::DebugFrameData) -> bool | game-ai\src\plan_legacy\old\fight_model.rs:531 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | attack_effect | game_core::Entity::attack_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1684 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 2 | distance | game_core::Entity::distance | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2161 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 3 | fight_participants | game_ai::plan_legacy::old::fight_model::fight_participants | in:game_ai | fn(usize, &mut rand::rngs::std::StdRng, &game_core::OperationData, &game_core::PlayerState, &game_core::Entity, &[&game_core::Entity], &[&game_core::Entity], &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< (&game_core::Entity, i64, bool)> | game-ai\src\plan_legacy\old\fight_model.rs:610 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | level | <game_ai::WindowStatView<'_> as game_core::AbstractEntity>::level | pub | fn(&game_ai::WindowStatView</#0>) -> usize | game-ai\src\fight_check.rs:96 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 5 | level | game_core::AbstractEntity::level | pub | fn(&Self/#0) -> usize | game-core\src\simulation\entity.rs:718 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 6 | level | <game_core::Entity as game_core::AbstractEntity>::level | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:772 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 7 | push | <game_core::Staff as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::staff | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\staff.rs:38 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 6개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 8 | push | <game_core::Athlete as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:434 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 6개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 9 | push | <game_core::Contract as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:2498 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 6개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 10 | range | game_core::Effect::range | pub | fn(&game_core::Effect, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:25 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 11 | team | game_view::ClientData::team | pub | fn(&game_view::ClientData, usize) -> std::option::Option<std::cell::Ref< game_core::Team>> | game-view\src\logic\client\data.rs:1651 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 12 | team | game_view::ClientDatabase::team | pub | fn(&game_view::ClientDatabase, usize) -> std::option::Option<&game_core::Team> | game-view\src\logic\client\data.rs:968 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 13 | team | game_core::PlayerAiContext::<'a, 'b, 'r>::team | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:583 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 14 | tick_per_second | game_view::view::effect::nightmare::tick_per_second | in:game_view::view::effect::nightmare | fn(&engine_core::assets::Assets) -> f32 | game-view\src\view\effect\nightmare.rs:31 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
</details>

⚠**미매칭 3개**: `move_speed`, `player_champion`, `reserve_internal_or_panic`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 3곳** (m02.ll:33259, m10.ll:16354, m10.ll:16459) · **형제 0개** 

**`open` 6건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | ally_is_bound(m10.ll 별도 define, 여기선 안 읽음) 의 판정 내용 — '아군이 적에게 묶여 있다' 정도만 이름·호출 위치로 추정. 안에서 version 분기가 있을 수 있다 | 5 |  |
| 1 | 미탐색 | Entity::distance 는 _gcbc 에 본문(안 읽음). 제곱거리인지 실거리인지 미확인 — reach(사거리 단위)와 뺄셈하므로 실거리로 추정 | 5 |  |
| 2 | 미탐색 | line 616: near_allies 안에 champ 자신이 있으면 bound=false 로 push 된다(제외되지 않는다). 호출측이 near_allies 에서 자신을 이미 뺐는지는 안 봤다(범위 밖). 반면 원거리 루프(L622)는 명시적으로 제외 | 4 |  |
| 3 | 미탐색 | Effect::range(effect.rs:25~26) 의 인라인 결과가 엔티티 필드(level·stat_buff_cached.range)까지 읽는다 — 시그니처가 range(&self, &Entity) 인지 클로저가 더 더하는지는 dbg 로 구분 못 함. 합계식만 확정 | 4 |  |
| 4 | 표기 불가 | fight_enemies 첫 원소 무검사 로드(m10.ll:39838) — declared 조건(any 가 true)이 비어있지 않음을 함의하므로 안전. 소스가 min().unwrap() 인지 unwrap_or 인지는 표기 불가(unwrap_or<u64> DISubprogram 이 보이나 값이 접힘) | 4 |  |
| 5 | 미탐색 | min 의 fold 는 별도 define(`…fight_participants…s0_0…min_by…fold`, calls 에 fold 로 등록)에 있고 본체는 안 읽음 — 이름으로 min_by(Ord::cmp) 임만 확정 | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

