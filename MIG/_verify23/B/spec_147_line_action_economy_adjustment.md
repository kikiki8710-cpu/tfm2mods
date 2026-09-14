---

### `147` line_action_economy_adjustment — 라인 공격/스킬 액션의 경제 보정치 — 적 챔피언/미니언 대상 공격이 받을 응징피해(punish) 대비 허용치(allowance)로 감점(음수)·챔피언 교환은 이득이면 가점

| 항목 | 값 |
|---|---|
| id | `lane_economy__line_action_economy_adjustment` |
| 심볼 | `_RNvNtCshdEBA0ozCnw_7game_ai12lane_economy30line_action_economy_adjustment` |
| 소스 | `game-ai\src\lane_economy.rs:6` |
| IR | `m11.ll` 51340~51725행 |
| 경로·가시성 | `game_ai::line_action_economy_adjustment` · **pub** |
| 계층 | 기타 |
| exe | `e28410` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r14` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_ai::SmallActionPlay, game_ai::MinionActionType) -> i64
```

<details><summary>인자 6개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | version | usize | 본문 분기 없음 — line_projected_punish_damage_at 첫 인자로만 전달(51657) | 4 |
| 1 | 2 | player | &PlayerState(2528B) | +0x930 info.team(배열 인덱스, bounds 2) · +0x9c0 info.position 태그(i32→usize) · punish_damage_at 인자 | 4 |
| 2 | 3 | data | &OperationData(24B) | +0x0 cache(player_champion 조회 · dyn AbstractGame get_entity_by_id) · +0x8 context(minion_reward_allowance 첫 인자) | 4 |
| 3 | 4 | parameter | &ScoreParameter(5384B) | +0x918 player(ChampionScoreParameter) 를 champion_hp_value 에 · trade/minion allowance 에 통째 전달 | 4 |
| 4 | 5 | action | &SmallActionPlay(184B) | captures(none) readonly. +0xb1 태그 · +0x8 target_id(Attack/Skill/Skill2/Ult 페이로드 공통) | 4 |
| 5 | 6 | ty | MinionActionType(1B i8 range 0..3) | Pull0/Normal1/Push2 — 본문 분기 없음, line_minion_reward_allowance 인자(51704)로만 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
L8 : champ = data.cache.player_champion[player.info.team][player.info.position]  — None(null) → return 0
L12: match action.tag (+0xb1):
  Attack(15): effect=&champ.attack_effect (None→0) ; target_id=action+0x8 ; speed_mult=champ.attack_speed_mult() ; action_duration=champ.attack_duration()          (L14~16)
  Skill (16): effect=&champ.skill_effect  (None→0) ; speed_mult=champ.cooldown_reduce(false) ; action_duration=champ.skill_duration()                                     (L18~20)
  Skill2(17): effect=champ.skill2_effect()[level>2 ? &skill2_effect : &NONE] (None→0) ; cooldown_reduce(false) ; skill2_duration()                                        (L22~24)
  Ult   (18): effect=champ.ult_effect()[level>4 ? &ult_effect : &NONE] (None→0) ; cooldown_reduce(true) ; ult_duration()                                                  (L26~28)
  그 외 13종 → return 0                                                                                                                                                       (L309<12 switch)
L32: target = data.cache.game.get_entity_by_id(target_id) (vtable +0x1f0) — None → 0
L35: if target.team == champ.team → 0 ; if target.ty ∉ {Minion(1), Champion(13)} → 0
L38: if !CastingTarget::check(&effect.target, champ, target) → 0
L42: (stance_x, stance_y, walk_tick) = line_action_stance(data, champ, target, effect)?  — sret 32B Option<(u64,u64,u64)> (+0 태그 i64 trunc→i1, +8/+16/+24) · None → 0 (L43)
L45: window = max( max(effect.start_timing*100 / speed_mult.max(1), action_duration) + walk_tick, 30 ) + 45
L46: punish_damage = line_projected_punish_damage_at(version, player, data, champ, target.ty@tag, stance_x, stance_y, window)
L47: if punish_damage < 1 → 0
L51: hp_value = champion_hp_value(data, parameter, &parameter.player).max(1)
L52: risk_score = hp_value * punish_damage / champ.hp.max(1)   (sdiv)
L53: recall_penalty = line_recall_pressure_penalty(champ, punish_damage, hp_value)
L54: if target.ty == Champion(13):
  L55: allowance = line_champion_trade_allowance(data, parameter, champ, target, effect)
  L60: trade_loss = recall_penalty + risk_score − allowance
  L61: if trade_loss > 0 → return −trade_loss (L62)
  L63~66: (챔피언) → return −trade_loss   (= 0 또는 양수 가점 · 51713)
else (Minion):
  L57: allowance = line_minion_reward_allowance(data.context, parameter, champ, target, effect, speed_mult, ty, walk_tick)
  L60: trade_loss = recall_penalty + risk_score − allowance
  L61: if trade_loss > 0 → return −trade_loss ; else → return 0 (L70)
```

**`mem` 메모리 접근 23건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | 51348 (gep 2352) · player_champion[team] 인덱스, bounds_check 2 (51354) | 4 | OK |
| 1 | PlayerState | 0x9c0 | info.position@tag | r | 51359 (gep 2496, i32 zext) · player_champion[team][position] | 4 | OK |
| 2 | OperationData | 0x0 | cache | r | 51362 → AbstractGameWithCache | 4 | OK |
| 3 | OperationData | 0x8 | context | r | 51702 → line_minion_reward_allowance 첫 인자 | 4 | OK |
| 4 | AbstractGameWithCache | 0x1e0 | player_champion[team][position] | r | 51363~51366 (gep 480 + [5 x ptr]*team + position) · Option<&Entity> 니치(null=None → return 0) | 4 | OK |
| 5 | AbstractGameWithCache | 0x0 | game.data_ptr | r | 51508 — &dyn AbstractGame 데이터 포인터 | 4 | OK |
| 6 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r | 51509~51513 — vtable+0x1f0(496) = get_entity_by_id(divtable) | 3 | OK |
| 7 | SmallActionPlay | 0xb1 | @tag | r | 51375 (gep 177) · 니치 태그: 15 Attack/16 Skill/17 Skill2/18 Ult 만 진행(switch 51382 는 idx=tag−3 기준 12~15) | 4 | OK |
| 8 | SmallActionPlay | 0x8 | Attack/Skill/Skill2/Ult.0.target_id | r | 51472/51518/51538/51557 (cast.rs:94/160/222/287 접근자) | 4 | 부분일치(첫 성분(컨테이너)만 맞고 **마지막 성분이 다르다** — 사람이 봐야 ) |
| 9 | Entity | 0x4c0 | attack_effect@tag | r | 51414 i32 == -1 → None → return 0 | 4 | OK |
| 10 | Entity | 0x490 | attack_effect@Some.0 | r | 51474 effect 포인터(Attack) | 4 | OK |
| 11 | Entity | 0x4f8 | skill_effect@tag | r | 51428 == -1 → return 0 | 4 | OK |
| 12 | Entity | 0x4c8 | skill_effect@Some.0 | r | 51520 effect 포인터(Skill) | 4 | OK |
| 13 | Entity | 0x5c8 | level | r | 51441/51460 — level>2 면 &skill2_effect 아니면 정적 NONE(@anon.38) · level>4 면 &ult_effect (entity.rs:1693/1701 접근자 인라인) | 4 | OK |
| 14 | Entity | 0x500 | skill2_effect | r | 51444 (+0x530 태그 == -1 → return 0 · 51447 gep 48) | 4 | OK |
| 15 | Entity | 0x538 | ult_effect | r | 51463 (+0x568 태그 == -1 → return 0 · 51466 gep 48) | 4 | OK |
| 16 | Entity | 0x0 | team@tag | r | 51579/51582 target.team vs champ.team (TeamType PartialEq 인라인) | 4 | OK |
| 17 | Entity | 0x8 | team@Player.0 | r | 51608/51609 — 둘 다 Player(태그0) 면 팀 번호 비교 | 4 | OK |
| 18 | Entity | 0x68 | ty@tag | r | 51594 target.ty 태그: 1 Minion / 13 Champion 만 진행 · 13 이면 챔피언 교환 경로 | 4 | OK |
| 19 | Entity | 0x670 | hp | r | 51670 champ.hp — risk_score 분모 | 4 | OK |
| 20 | Effect | 0x28 | target | r | 51614 (gep 40) → CastingTarget::check(&effect.target, champ, target) | 4 | OK |
| 21 | Effect | 0x20 | start_timing | r | 51640 (gep 32) — window 계산 | 4 | OK |
| 22 | ScoreParameter | 0x918 | player | r | 51663 (gep 2328) → champion_hp_value(data, parameter, &parameter.player) | 4 | OK |

**`consts` 상수 8건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 2 | 8 | 임계 | player_champion 1차원 bounds(team<2, 51350) · 및 level>2 (skill2 사용가능, entity.rs:1693 · 51443) | 4 |
| 1 | 4 | 26 | 임계 | level>4 → ult_effect 사용가능 (entity.rs:1701 · 51462) | 4 |
| 2 | -1 | 14 | 센티널 | Option<Effect> 니치 태그 None (i32 -1 · 51416/51430/51449/51468) | 4 |
| 3 | 1 | 35 | 태그 | target.ty 태그 Minion (switch 51597) · 및 max(…,1) 분모 가드(51644/51667/51680) | 4 |
| 4 | 13 | 35 | 태그 | target.ty 태그 Champion (switch 51598 · 51685 챔피언 교환 분기) | 4 |
| 5 | 100 | 45 | 계수 | effect.start_timing*100/speed_mult — 시전 선딜을 배율(%)로 나눔 (51645) | 4 |
| 6 | 30 | 45 | 임계 | window 하한 틱: max(…+walk_tick, 30) (51653) | 4 |
| 7 | 45 | 45 | 계수 | window 가산 틱: +45 (51654) — 응징피해 투영 시간창 | 4 |

**`knobs` 조정점 4건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | window 하한 | lane_economy.rs:45 | 30 | 응징피해 투영 시간창 최소 틱. 올리면 짧은 액션도 긴 시간창으로 punish 를 계산해 감점 증가 | 4 | 기존 |
| 1 | window 가산 | lane_economy.rs:45 | 45 | 모든 액션의 투영 시간창에 더하는 틱. 올리면 punish_damage↑ → risk_score/recall_penalty↑ → 공격 억제 | 4 | 기존 |
| 2 | punish 무시 하한 | lane_economy.rs:47 | 1 | punish_damage<1 이면 보정 0. 사실상 '응징 없음=무보정' 게이트 | 4 | 기존 |
| 3 | skill2/ult 가용 레벨 | entity.rs:1693/1701 (인라인) | level>2 / level>4 | game_core 접근자 — 이 함수에선 그 레벨 미만이면 효과 None 으로 즉시 0 | 4 | 기존 |

<details><summary>`callees` 피호출자 20건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | attack_duration | game_core::Entity::attack_duration | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1770 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | attack_effect | game_core::Entity::attack_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1684 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 2 | attack_speed_mult | game_core::Entity::attack_speed_mult | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:2433 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 3 | champion_hp_value | game_ai::champion_hp_value | pub | fn(&game_core::OperationData, &game_ai::ScoreParameter, &game_ai::ChampionScoreParameter) -> i64 | game-ai\src\utils.rs:909 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | check | game_core::CastingTarget::check | pub | fn(&game_core::CastingTarget, &game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\effect\type.rs:227 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 5 | cooldown_reduce | game_core::Entity::cooldown_reduce | pub | fn(&game_core::Entity, bool) -> usize | game-core\src\simulation\entity.rs:2437 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 6 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 7 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 8 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 9 | line_action_stance | game_ai::lane_economy::line_action_stance | in:game_ai | fn(&game_core::OperationData, &game_core::Entity, &game_core::Entity, &game_core::Effect) -> std::option::Option<(u64, u64, usize)> | game-ai\src\lane_economy.rs:77 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 10 | line_champion_trade_allowance | game_ai::lane_economy::line_champion_trade_allowance | in:game_ai | fn(&game_core::OperationData, &game_ai::ScoreParameter, &game_core::Entity, &game_core::Entity, &game_core::Effect) -> i64 | game-ai\src\lane_economy.rs:221 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 11 | line_minion_reward_allowance | game_ai::lane_economy::line_minion_reward_allowance | in:game_ai | fn(&game_core::OperationData, &game_ai::ScoreParameter, &game_core::Entity, &game_core::Entity, &game_core::Effect, usize, game_ai::MinionActionType, usize) -> i64 | game-ai\src\lane_economy.rs:185 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 12 | line_projected_punish_damage_at | game_ai::lane_economy::line_projected_punish_damage_at | in:game_ai | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, &game_core::Entity, u64, u64, usize) -> i64 | game-ai\src\lane_economy.rs:130 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 13 | line_recall_pressure_penalty | game_ai::lane_economy::line_recall_pressure_penalty | in:game_ai | fn(&game_core::Entity, i64, i64) -> i64 | game-ai\src\lane_economy.rs:238 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 14 | skill2_duration | game_core::Entity::skill2_duration | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1800 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 15 | skill2_effect | game_core::Entity::skill2_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1692 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 16 | skill_duration | game_core::Entity::skill_duration | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1785 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 17 | skill_effect | game_core::Entity::skill_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1688 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 18 | ult_duration | game_core::Entity::ult_duration | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1815 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 19 | ult_effect | game_core::Entity::ult_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1700 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
</details>

⚠**미매칭 2개**: `else`, `trade_loss`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 3곳** (m02.ll:48495, m14.ll:29977, m15.ll:22585) · **형제 0개** 

**`open` 4건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | line_action_stance sret 32B 의 정확한 타입명(Option<(u64,u64,u64)> 추정 — +0 i64 를 trunc nuw i1 로 태그 판정, +8 stance_x/+16 stance_y/+24 walk_tick 은 dbg 이름으로 확정) — 내부는 r14 타 배치/미탐색 | 4 |  |
| 1 | 미탐색 | line_projected_punish_damage_at(internal fastcc · m11.ll:51728)·line_minion_reward_allowance(internal fastcc · m11.ll:51018 · 반환 range 0..41) 내부 미탐색(시그니처만) | 4 |  |
| 2 | 미탐색 | @anon.dfa1f8a0e3a880d1d1859ec099fe0b0f.38 = 정적 None Option<Effect>(skill2/ult 접근자의 레벨 미달 폴백)로 읽음 — 상수 내용 미확인 | 4 |  |
| 3 | 표기 불가 | speed_mult 의 udiv(51646, unsigned) vs risk_score 의 sdiv(51681, signed) — 타입(usize vs i64) 혼재는 IR 관측 그대로이며 소스 캐스팅 위치는 표기 불가 | 4 |  |

**`notes` 1건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | L63~66 챔피언 분기의 정확한 소스 형태: IR 은 trade_loss<=0 인 챔피언 경로에서 L66 `0 − trade_loss` 를 반환 — `if target is champion { return −trade_loss }` 로 읽었으나 L63~65 의 조건식(다른 조건이 상수접힘됐는지)은 미확정 | 4 | 사실 서술 |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

