---

### `215` v16_gambler_ult_cc_bonus — 겜블러 궁(GamblerUltAction, 매혹 CC)을 적 챔피언 target 에 쓸 때의 가산점(0~70) — CC 시간·궁 피해·대상 HP가치·유입피해·근접 아군/가시 적 수·포커스/서포트 일치로 합산

| 항목 | 값 |
|---|---|
| id | `battle_common__v16_gambler_ult_cc_bonus` |
| 심볼 | `_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy8sub_plan13battle_common24v16_gambler_ult_cc_bonus` |
| 소스 | `game-ai\src\plan_legacy\sub_plan\battle_common.rs:109` |
| IR | `m05.ll` 54567~55828행 |
| 경로·가시성 | `game_ai::plan_legacy::sub_plan::battle_common::v16_gambler_ult_cc_bonus` · **in:game_ai** |
| 계층 | 기타 |
| exe | `d676c0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r17` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_ai::plan_legacy::old::BattleSubPlanGoal, std::option::Option<usize>, &std::boxed::Box<dyn [Binder { value: Trait(game_core::Action), bound_vars: [] }] + 'static, std::alloc::Global>, &game_core::Effect, &game_core::Entity, &game_core::Entity) -> i64
```

**TLS 메모 절(`sig.tls` · 정정 경로 `/specs[215]/sig/tls/<키>`)**

- `name`: (직접 접점 없음 · 콜리 경유 1건)
- `role`: 소비자(간접)
- `key`: 본체에 `LocalKey`/`call_once` 참조 0건. 콜리 `utils::champion_hp_value`(m04.ll 49583) 가 TLS `utils::HP_VALUE_MEMO`(RefCell<(usize,usize,HashMap<usize,i64>)>) 를 읽고 씀 — L139 에서 1회(target_parameter 를 찾았을 때만)
- `layout`: 콜리 명세 소관
- `invalidation`: 콜리 명세 소관
- `call_conditions`: L138 parameter.near_enemies 에 target.id 가 있을 때만 champion_hp_value 1회 호출(그 외 콜리 expected_damage_target·possible_risk·is_recent_visible 은 TLS 참조 0 확인)

<details><summary>인자 9개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | player | &PlayerState(2528B) | IR %0 · readonly · info.team(+0x930) · is_recent_visible 인자 | 4 |
| 1 | 2 | data | &OperationData(24B) | IR %1 · readonly · cache(+0)·context(+8)·blackboard(+0x10) | 4 |
| 2 | 3 | parameter | &ScoreParameter(5384B) | IR %2 · readonly · near_enemies Vec(+0x14d8 ptr/+0x14f0 len) 에서 target.id 검색 · champion_hp_value/possible_risk 인자 | 4 |
| 3 | 4 | goal | &BattleSubPlanGoal(16B) | IR %3 · readonly captures(none) · L165 goal.focus(): 태그(+0)∈{0,1,2,3,5,6} 이면 Some(+8 focus) · RunAway(4)/End(7) 은 None | 4 |
| 4 | 5 | support_target | Option<usize> | IR 스칼라 2개: %4 태그(range(0,2)) · %5 페이로드. L170 `== Some(target.id)` 비교에만 사용 | 4 |
| 5 | 6 | action | &Box<dyn Action>(16B) | IR %6 · readonly captures(none) · L120 as_any(vtable+0x68)→type_id(Any vtable+0x18) 로 GamblerUltAction 다운캐스트 | 4 |
| 6 | 7 | effect | &Effect(56B) | IR %7 · readonly · ty(Arc<dyn EffectType> +0/+8) 의 expected_cc_time(vtable+0x80) · expected_damage_target 의 self | 4 |
| 7 | 8 | champ | &Entity(1728B) | IR %8 · readonly · 시전자. team(+0/+8)·id(+0x5c0) · expected_damage_target 에 &dyn AbstractEntity(vtable @anon.17)로 전달 | 4 |
| 8 | 9 | target | &Entity(1728B) | IR %9 · readonly · team·ty@tag(+0x68)·cc_immune(+0x468)·cc Vec(+0x2c8/+0x2d0)·id·x·y·hp | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
L120  let Some(gambler_ult) = action.as_any().downcast_ref::<GamblerUltAction>() else { return 0 };   // TypeId i128 비교
L124  if target.team == champ.team { return 0 }                    // TeamType derived PartialEq
L125  if target.ty 태그 != 13(Champion) { return 0 }                // L124·L125 는 IR 에서 한 select 로 접힘
L126  if target.stat_buff_cached.cc_immune { return 0 }
L127  if target.cc.iter().any(|c| c.is_cc()) { return 0 }           // entity.rs:534 is_cc = 태그∈{Airborne,Stun,Bind,ForceMove,Fear,Charm}
L131  let cc_time = effect.ty.expected_cc_time().map_or(gambler_ult.charm_duration, |t| max(t, gambler_ult.charm_duration));   // (외연 동일: max(charm, cc.unwrap_or(0)))
L132  if cc_time == 0 { return 0 }
L136  let mut bonus = ((cc_time as i64) / 3).clamp(12, 30);         // IR: cc_time<36 → 12, else min(cc_time/3, 30)
L138  if let Some(tp) = parameter.near_enemies.iter().find(|p| p.id == target.id) {
L139      let hp_value = min(champion_hp_value(data, parameter, tp), 100);
L140      let ult_damage = effect.expected_damage_target(data.context, champ as &dyn AbstractEntity, target);
L141      let incoming = tp.applyed_damage + tp.risk_damage                                     // L141~L143
L143                   + tp.possible_risk(data, cc_time + 30);
L144      bonus += min(ult_damage * hp_value / max(target.hp,1), 35);       // sdiv(i64)
L145      bonus += hp_value / 6;
L146      if incoming > 0 {                                                    // signed
L147          bonus += min(incoming * hp_value / max(target.hp,1), 22);
      }
      } else {
L150      bonus += 6;
      }
L153  let team = player.info.team;                                            // <2 bounds check
L154  let ally_near  = data.cache.iter_champions(team).filter(|a| a.id != champ.id && dist2(a, target) < 120000²+1).count();     // L153~155, 5칸 언롤
L156  let enemy_team = 1 - team;
L157  let enemy_near = data.cache.iter_champions(enemy_team).filter(|e| e.id != target.id
L158                        && data.blackboard[enemy_team].is_recent_visible(game, player, e)
L159                        && dist2(e, target) < 90000²+1).count();                                                        // L157~160, 5칸 언롤
L162  bonus += min(ally_near, 3) * 5;
L163  bonus += min(enemy_near, 2) * 4;                                          // shl 2
L165  let focus_relevant = goal.focus().is_some_and(|focus_id|                 // old/battle.rs:30 focus(): 태그 4·7 → None
L166        focus_id == target.id
L166        || game.get_entity_by_id(focus_id).is_some_and(|focus|
L167              focus.team == champ.team && dist2(focus, target) < 90000²+1));
L170  if focus_relevant || support_target == Some(target.id) { bonus += 12 }   // L171
L174  return min(bonus, 70);                                                    // L175

※ 언롤된 fold 의 dbg 이름: ally_near(%304)·enemy_near(%468)·focus_relevant(%513)·bonus 단계별. 거리² 는 |dx|²+|dy|²(u64).
```

**`mem` 메모리 접근 31건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | Box<dyn Action> | 0x0 | data ptr | r | L120 as_any self | 4 | 확인불가(tcx 사전에 타입 없음) |
| 1 | Box<dyn Action> | 0x8 | vtable ptr | r | L120 +0x68 슬롯 = Action::as_any(divtable) → {ptr,ptr}(&dyn Any) | 3 | 확인불가(tcx 사전에 타입 없음) |
| 2 | dyn Any vtable | 0x18 | type_id (sret 16B TypeId) | r | L120 i128 == 129962296932191015333459514643667840978 (GamblerUltAction 의 TypeId · m02/m05/m15 downcast 사이트 6곳 동일) | 4 | 확인불가(tcx 사전에 타입 없음) |
| 3 | GamblerUltAction | 0x10 | charm_duration | r | L131 cc_time 하한(max) | 4 | OK |
| 4 | Entity | 0x0 | team@tag | r | L124 target.team == champ.team / L167 focus.team == champ.team | 4 | OK |
| 5 | Entity | 0x8 | team@Player.0 | r | L124/L167 태그 Player(0)일 때 페이로드 비교 | 4 | OK |
| 6 | Entity | 0x68 | ty@tag | r | L125 target.ty 태그 == 13(Champion) | 4 | OK |
| 7 | Entity | 0x468 | stat_buff_cached.cc_immune | r | L126 true 면 0 | 4 | OK |
| 8 | Entity | 0x2c8 | cc.buf.ptr | r | L127 target.cc 슬라이스(CCState 40B stride · 태그 i32 +0) | 4 | OK |
| 9 | Entity | 0x2d0 | cc.len | r | L127 | 4 | OK |
| 10 | Entity | 0x5c0 | id | r | target.id(L138 검색키·L157 적 제외·L166 focus 비교·L170 support 비교) · champ.id(L154 아군 자기제외) | 4 | OK |
| 11 | Entity | 0x660 | x | r | target 기준 거리²(L154/L159/L167) | 4 | OK |
| 12 | Entity | 0x668 | y | r | 동상 | 4 | OK |
| 13 | Entity | 0x670 | hp | r | L144/L147 target.hp — max(…,1) 분모 | 4 | OK |
| 14 | CCState | 0x0 | tag(i32) | r | L127 is_cc(): {0 Airborne,1 Stun,2 Bind,6 ForceMove,8 Fear,9 Charm} 이면 true | 4 | OK |
| 15 | Effect | 0x0 | ty (Arc<dyn EffectType> data ptr) | r | L131 ArcInner 페이로드 = ptr + ((align-1)&~15) + 16 | 4 | OK |
| 16 | Effect | 0x8 | ty vtable ptr | r | +0x10 align · +0x80 슬롯 = EffectType::expected_cc_time -> Option<usize>(divtable) | 3 | OK |
| 17 | ScoreParameter | 0x14d8 | near_enemies.buf.ptr | r | L138 ChampionScoreParameter 216B stride | 4 | OK |
| 18 | ScoreParameter | 0x14f0 | near_enemies.len | r | L138 | 4 | OK |
| 19 | ChampionScoreParameter | 0x58 | id | r | L138 == target.id 로 find | 4 | OK |
| 20 | ChampionScoreParameter | 0x70 | applyed_damage | r | L141 incoming 항 | 4 | OK |
| 21 | ChampionScoreParameter | 0x80 | risk_damage | r | L142 incoming 항 | 4 | OK |
| 22 | OperationData | 0x0 | cache | r | L153/L157 player_champion · game | 4 | OK |
| 23 | OperationData | 0x8 | context | r | L140 expected_damage_target 의 &GameContext | 4 | OK |
| 24 | OperationData | 0x10 | blackboard | r | L158 [1-team] · 744B stride | 4 | OK |
| 25 | PlayerState | 0x930 | info.team | r | L153 아군 인덱스(<2 bounds check) · L156 1-team | 4 | OK |
| 26 | AbstractGameWithCache | 0x0 | game data ptr | r | L158 is_recent_visible · L166 get_entity_by_id | 4 | OK |
| 27 | AbstractGameWithCache | 0x8 | game vtable ptr | r | +0x1f0 get_entity_by_id | 4 | OK |
| 28 | AbstractGameWithCache | 0x1e0 | player_champion[2][5] | r | L153 [team] · L156 [1-team] 각 5칸 완전 언롤 | 4 | OK |
| 29 | BattleSubPlanGoal | 0x0 | tag | r | L165 switch: 0,1,2,3,5,6 → focus 있음 / 4(RunAway),7(End) → None | 4 | OK |
| 30 | BattleSubPlanGoal | 0x8 | focus | r | L165 focus_id | 4 | OK |

**`consts` 상수 23건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev | folded_from |
|---|---|---|---|---|---|---|
| 0 | 129962296932191015333459514643667840978 | 120 | 태그 | TypeId(GamblerUltAction) i128 — downcast_ref 판별 | 4 |  |
| 1 | 13 | 125 | 태그 | EntityType 메모리태그 13 = Champion (target.ty) | 4 |  |
| 2 | 0 | 127 | 태그 | CCState 태그 Airborne (is_cc 참) / L124 TeamType Player 태그 / L132 cc_time==0 / L146 incoming>0 / 반환 0 | 4 |  |
| 3 | 1 | 127 | 태그 | CCState 태그 Stun (is_cc 참) / L144·L147 max(hp,1) 가드 | 4 |  |
| 4 | 2 | 127 | 태그 | CCState 태그 Bind (is_cc 참) / L153 team<2 bounds / L163 min(enemy_near,2) (L163 의 `shl …, 2` 는 아래 folded_from 항목) | 4 |  |
| 5 | 2 | 163 | 인덱스 | min(enemy_near,2)*4 — `shl nuw nsw i64 %472, 2` 로 접힘(리터럴 4 는 본문에 없고 dbg_value 의 루프 인덱스뿐) | 4 | 4 |
| 6 | 6 | 127 | 태그 | CCState 태그 ForceMove (is_cc 참) / L145 hp_value/6 / L150 target_parameter 없을 때 +6 | 4 |  |
| 7 | 8 | 127 | 태그 | CCState 태그 Fear (is_cc 참) | 4 |  |
| 8 | 9 | 127 | 태그 | CCState 태그 Charm (is_cc 참) | 4 |  |
| 9 | 3 | 136 | 태그 | cc_time/3 (sdiv) · L162 min(ally_near,3) | 4 |  |
| 10 | 36 | 136 | 임계 | clamp 하한 접힘: cc_time<36 ⟺ cc_time/3<12 → 12 (icmp slt %cc_time, 36) | 4 |  |
| 11 | 12 | 136 | 계수 | clamp(12,30) 하한 / L171 focus_relevant\|\|support 일치 가산 +12 | 4 |  |
| 12 | 30 | 136 | 임계 | clamp(12,30) 상한 / L143 possible_risk(…, cc_time+30) 의 +30 틱 | 4 |  |
| 13 | 100 | 139 | 인덱스 | hp_value = min(champion_hp_value, 100) | 4 |  |
| 14 | 35 | 144 | 인덱스 | min(ult_damage*hp_value/max(target.hp,1), 35) | 4 |  |
| 15 | 22 | 147 | 인덱스 | min(incoming*hp_value/max(target.hp,1), 22) | 4 |  |
| 16 | 14400000001 | 154 | 임계 | 120000²+1 — 아군↔target 거리² 상한(ally_near) | 4 |  |
| 17 | 8100000001 | 159 | 임계 | 90000²+1 — 가시 적↔target 거리² 상한(enemy_near) / L167 focus↔target | 4 |  |
| 18 | 5 | 162 | 태그 | min(ally_near,3)*5 | 4 |  |
| 19 | 70 | 174 | 임계 | 최종 bonus 상한 min(bonus,70) | 4 |  |
| 20 | -9223372036854775808 | 144 | 태그 | i64::MIN — sdiv 오버플로 패닉 가드(product==MIN && divisor==-1) · 판정값 아님 | 4 |  |
| 21 | -1 | 131 | 태그 | ArcInner align-1 (Arc<dyn> 페이로드 오프셋 계산) / L144·L147 divisor==-1 오버플로 가드 · 판정값 아님 | 4 |  |
| 22 | -16 | 131 | 계수 | (align-1)&~15 — Arc 헤더(16B) 정렬 · 판정값 아님 | 4 |  |

**`knobs` 조정점 8건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | CC 시간 기본 가산 clamp 범위 | battle_common.rs:136 | 12..30 (cc_time/3) | 하한 12 를 올리면 짧은 매혹도 더 높게, 상한 30 을 올리면 긴 CC 가 더 크게 반영 | 4 | 기존 |
| 1 | 궁 피해 기여 상한 | battle_common.rs:144 | 35 | 올리면 고피해 궁일수록 가산이 커짐(ult_damage·hp_value/target.hp 비례) | 4 | 기존 |
| 2 | 유입 피해 기여 상한 | battle_common.rs:147 | 22 | 올리면 팀 화력 집중 대상에 더 높은 점수 | 4 | 기존 |
| 3 | 아군 근접 반경(↔target) | battle_common.rs:154 | 14400000001 | 120000² — 올리면 더 먼 아군도 세어 최대 +15(3×5)에 빨리 도달 | 4 | 기존 |
| 4 | 가시 적 근접 반경(↔target) | battle_common.rs:159 | 8100000001 | 90000² — 올리면 주변 적이 더 많이 세여 최대 +8(2×4) | 4 | 기존 |
| 5 | 포커스/서포트 일치 가산 | battle_common.rs:171 | 12 | 올리면 팀 포커스 대상에 궁을 더 우선 | 4 | 기존 |
| 6 | 최종 상한 | battle_common.rs:174 | 70 | 이 함수가 액션 점수에 더할 수 있는 최대치 | 4 | 기존 |
| 7 | target_parameter 부재 시 고정 가산 | battle_common.rs:150 | 6 | near_enemies 에 없는(정보 없는) 대상의 기본 가치 | 4 | 기존 |

<details><summary>`callees` 피호출자 17건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | as_any | <game_ai::AgentVerHamster as game_core::AiAgent>::as_any | pub | fn(&game_ai::AgentVerHamster) -> &dyn [Binder { value: Trait(std::any::Any), bound_vars: [] }] + 'static | game-ai\src\lib.rs:425 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 321개 중 상위 3개 |
| 1 | as_any | game_core::Action::as_any | pub | fn(&Self/#0) -> &dyn [Binder { value: Trait(std::any::Any), bound_vars: [] }] + 'static | game-core\src\setting\action.rs:12 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 321개 중 상위 3개 |
| 2 | as_any | game_core::AiAgent::as_any | pub | fn(&Self/#0) -> &dyn [Binder { value: Trait(std::any::Any), bound_vars: [] }] + 'static | game-core\src\simulation\ai_interface.rs:499 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 321개 중 상위 3개 |
| 3 | champion_hp_value | game_ai::champion_hp_value | pub | fn(&game_core::OperationData, &game_ai::ScoreParameter, &game_ai::ChampionScoreParameter) -> i64 | game-ai\src\utils.rs:909 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | expected_cc_time | game_core::EffectType::expected_cc_time | pub | fn(&Self/#0) -> std::option::Option<usize> | game-core\src\simulation\effect\type.rs:299 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 37개 중 상위 3개 |
| 5 | expected_cc_time | game_core::EffectBuff::expected_cc_time | pub | fn(&Self/#0) -> std::option::Option<usize> | game-core\src\simulation\effect.rs:170 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 37개 중 상위 3개 |
| 6 | expected_cc_time | <game_core::GrabEffect as game_core::EffectType>::expected_cc_time | pub | fn(&game_core::GrabEffect) -> std::option::Option<usize> | game-core\src\simulation\effect\type\grab.rs:31 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 37개 중 상위 3개 |
| 7 | expected_damage_target | game_core::Effect::expected_damage_target | pub | fn(&game_core::Effect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + , &game_core::Entity) -> usize | game-core\src\simulation\effect.rs:91 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 8 | find | game_ai::MinionWaveSnapshot::find | pub | fn(&game_ai::MinionWaveSnapshot, usize) -> std::option::Option<&game_ai::MinionHpTrajectory> | game-ai\src\utils.rs:84 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 9 | focus | game_ai::plan_legacy::old::BattleSubPlanGoal::focus | pub | fn(&game_ai::plan_legacy::old::BattleSubPlanGoal) -> std::option::Option<usize> | game-ai\src\plan_legacy\old\battle.rs:29 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 10 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 11 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 12 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 13 | is_cc | game_core::CCState::is_cc | pub | fn(&game_core::CCState) -> bool | game-core\src\simulation\entity.rs:533 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 14 | is_recent_visible | game_core::Blackboard::is_recent_visible | pub | fn(&game_core::Blackboard, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\game\blackboard.rs:346 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 15 | iter_champions | game_core::AbstractGameWithCache::<'a, 'b>::iter_champions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5759 ~ game_core[6a30]::simulation::{impl#5}::iter_champions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1904 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 16 | possible_risk | game_ai::ChampionScoreParameter::<'_>::possible_risk | pub | fn(&game_ai::ChampionScoreParameter</#0>, &game_core::OperationData, usize) -> i64 | game-ai\src\score_parameter.rs:1407 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
</details>

⚠**미매칭 6개**: `ally_near`, `clamp`, `dist2`, `enemy_near`, `focus_relevant`, `map_or`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 2곳** (m02.ll:38211, m15.ll:12651) · **형제 0개** 

**`open` 5건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 표기 불가 | L131 소스 표기: `map_or(charm, \|t\| max(t,charm))` 인지 `max(charm, expected_cc_time().unwrap_or(0))` 인지 — 외연 동일(표기 불가). 같은 파일의 `effective_ult_cc_time`(L99) 은 여기서 호출되지 않았다(콜리 요약·dloc 사슬에 L99~107 없음) | 4 |  |
| 1 | 미탐색 | L124/L125 의 `\|\|`/`&&` 소스 순서 — 한 select 로 접혀 판별 불가(외연 동일) | 4 |  |
| 2 | 재료 부재 | expected_cc_time 의 런타임 구현체(CombineEffect 등) 별 값 — Arc<dyn EffectType> 이라 정적 확정 불가(divtable 은 슬롯 이름만) | 3 |  |
| 3 | 미탐색 | champion_hp_value 의 TLS HP_VALUE_MEMO 레이아웃·무효화 규칙 — 콜리(utils) 명세 소관 | 4 |  |
| 4 | 미탐색 | hp_value·ult_damage 가 음수가 되는 경우가 있는지(반환 range 하한 -2^63 의 근거) — champion_hp_value/expected_damage_target 내부 미독해 | 4 |  |

**`notes` 1건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | L166 클로저 안 `focus_id == target.id \|\| get_entity_by_id(...)` 의 소스 순서 — IR 은 id 비교 후 vtable 호출(short-circuit 관측) · 소스 순서로 확정 | 4 | 사실 서술 |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

