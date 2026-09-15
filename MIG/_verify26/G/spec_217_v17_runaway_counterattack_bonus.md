---

### `217` v17_runaway_counterattack_bonus — RunAway 골 중 가시 적 챔피언 target 에 대한 '역공(counterattack)' 가산점(0~95) — 즉사/콤보즉사/무거운 교환 중 하나일 때만, 사거리·유입피해·대상HP·아군/적 수로 가감

| 항목 | 값 |
|---|---|
| id | `battle_common__v17_runaway_counterattack_bonus` |
| 심볼 | `_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy8sub_plan13battle_common31v17_runaway_counterattack_bonus` |
| 소스 | `game-ai\src\plan_legacy\sub_plan\battle_common.rs:270` |
| IR | `m05.ll` 59244~60293행 |
| 경로·가시성 | `game_ai::plan_legacy::sub_plan::battle_common::v17_runaway_counterattack_bonus` · **in:game_ai** |
| 계층 | 기타 |
| exe | `d69f80` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r17` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_ai::plan_legacy::old::BattleSubPlanGoal, &game_core::Entity, &game_core::Entity, &game_core::Effect) -> i64
```

**TLS 메모 절(`sig.tls` · 정정 경로 `/specs[217]/sig/tls/<키>`)**

- `name`: (직접 접점 없음 · 콜리 경유)
- `role`: 소비자(간접)
- `key`: 본체에 `LocalKey`/`call_once` 참조 0건. 콜리 `battle::max_range_cached`(m10.ll 50689) 가 TLS `old::battle::MAX_RANGE_CACHE`(RefCell<MaxRangeCache>) 를 읽고 씀 — L314 에서 정확히 1회(champ,target)
- `layout`: 콜리 명세 소관
- `invalidation`: 콜리 명세 소관
- `call_conditions`: L314 는 L297 게이트 통과 후 무조건 1회. 직접 콜리 v17_ready_counterattack_damage/v17_escape_is_costly 본체는 TLS 참조 0(그 안의 2차 콜리는 미확인) · max_range(비캐시)·is_recent_visible·expected_damage_target·possible_risk·Entity::distance 는 TLS 참조 0 확인

<details><summary>인자 8개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | _version | usize | IR %0 · 본문에서 안 읽음(DI `_version`) | 4 |
| 1 | 2 | player | &PlayerState(2528B) | IR %1 · readonly · info.team(+0x930) · is_recent_visible/v17_escape_is_costly 인자 | 4 |
| 2 | 3 | data | &OperationData(24B) | IR %2 · readonly · cache(+0)·context(+8)·blackboard(+0x10) · max_range_cached/possible_risk 인자 | 4 |
| 3 | 4 | parameter | &ScoreParameter(5384B) | IR %3 · readonly · player(+0x918) 블록의 applyed_damage/risk_damage/risk_possible_tower · possible_risk 의 self | 4 |
| 4 | 5 | goal | &BattleSubPlanGoal(16B) | IR %4 · readonly captures(none) · L280 태그(+0)==4(RunAway) 만 통과 | 4 |
| 5 | 6 | champ | &Entity(1728B) | IR %5 · readonly · 나. team·id(+0x5c0)·x·y·hp(+0x670)·stat_cached.move_speed(+0x640) · expected_damage_target 에 &dyn AbstractEntity(vtable @anon.17) | 4 |
| 6 | 7 | target | &Entity(1728B) | IR %6 · readonly · 적. team·ty@tag(+0x68)·stat_buff_cached.undying(+0x488)·x·y·hp·stat_cached.hp(+0x628) | 4 |
| 7 | 8 | effect | &Effect(56B) | IR %7 · readonly · expected_damage_target 의 self(현재 액션 이펙트) | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
L280  if !matches!(goal, BattleSubPlanGoal::RunAway) { return 0 }          // 태그 4 (old/battle.rs:264 인라인 비교)
L281  if target.team == champ.team { return 0 }
L282  if target.ty 태그 != 13(Champion) { return 0 }                         // L281·L282 한 select 로 접힘
L283  if target.stat_buff_cached.undying { return 0 }
L284  let enemy_team = 1 - player.info.team;   if !data.blackboard[enemy_team].is_recent_visible(game, player, target) { return 0 }
L288  let current_damage = effect.expected_damage_target(data.context, champ as &dyn AbstractEntity, target);
L289  let ready_damage   = v17_ready_counterattack_damage(data, champ, target);     // IR 은 (cache, context, champ, target) 로 승격
L290  let target_hp = max(target.hp, 1);
L291  let lethal_now   = current_damage >= target_hp;      // signed
L292  let combo_lethal = ready_damage   >= target_hp;      // signed
L293  let escape_is_costly = v17_escape_is_costly(player, data, parameter, champ);
L294  let close_counter = v17_close_counterattack_target(champ, target);   // 인라인 L471~473: reach = max_range(champ,target) + 20000 + champ.stat_cached.move_speed*12; dist2(target,champ) <= reach²
L296  let heavy_trade = escape_is_costly && close_counter && ready_damage*100 >= target_hp*70;   // dbg: %94=(ready*100 < hp*70) 에 DW_OP_not · 분기 방향도 일치
L297  if !lethal_now && !combo_lethal && !heavy_trade { return 0 }
L301  let mut bonus = if lethal_now { 75 } else if combo_lethal { 48 } else { 14 };
L309  if escape_is_costly { bonus += 10 }
L313  let dist  = target.distance(champ);                  // Entity::distance(self=target, champ)
L314  let range = max_range_cached(data, champ, target);   // TLS MAX_RANGE_CACHE 경유
L315  if dist <= range + 15000 { bonus += 10 }             // L316
L317  else if dist <= range + champ.move_speed*12 { bonus += 4 }
L321  let incoming = parameter.player.applyed_damage + parameter.player.risk_damage        // L321~L322
L323               + parameter.player.possible_risk(data, 90)
L324               + parameter.player.risk_possible_tower / 2;                                // sdiv
L325  let my_hp = max(champ.hp, 1);
L326  if incoming*100 >= my_hp*60 { bonus += 10 }          // L327 (IR: slt 면 L328 분기)
L328  else if incoming*100 >= my_hp*30 { bonus += 5 }
L332  if target.hp*100 <= max(target.stat_cached.hp,1)*35 { bonus += 8 }
L336  let near_allies  = data.cache.iter_champions(player.info.team).filter(|a| a.id != champ.id && dist2(a, champ) < 120000²+1).count();   // L336~338 · 5칸 언롤
L339  let near_enemies = data.cache.iter_champions(enemy_team).filter(|e| data.blackboard[enemy_team].is_recent_visible(game, player, e) && dist2(e, champ) < 120000²+1).count();   // L339~342 · id 제외 없음
L344  if near_enemies > near_allies {
L346      if !(lethal_now || combo_lethal || near_enemies < near_allies + 2) { bonus -= 15 }
      } else {
L345      bonus += 5;
      }
L350  return bonus.clamp(0, 95);      // smax 0 → umin 95 · L351 ret

※ dbg 이름: current_damage·ready_damage·target_hp·lethal_now·combo_lethal·escape_is_costly·reach·close_counter·heavy_trade·dist·range·incoming·my_hp·near_allies·near_enemies.
```

**`mem` 메모리 접근 22건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | BattleSubPlanGoal | 0x0 | tag | r | L280 == 4 (RunAway) 아니면 0 | 4 | OK |
| 1 | Entity | 0x0 | team@tag | r | L281 target.team == champ.team → 0 | 4 | OK |
| 2 | Entity | 0x8 | team@Player.0 | r | L281 페이로드 비교 | 4 | OK |
| 3 | Entity | 0x68 | ty@tag | r | L282 target.ty == 13(Champion) 필요 | 4 | OK |
| 4 | Entity | 0x488 | stat_buff_cached.undying | r | L283 target 이 undying 이면 0 | 4 | OK |
| 5 | Entity | 0x670 | hp | r | L290 target_hp=max(target.hp,1) · L325 my_hp=max(champ.hp,1) · L332 target.hp*100 | 4 | OK |
| 6 | Entity | 0x628 | stat_cached.hp | r | L332 target 최대HP(max(…,1)) | 4 | OK |
| 7 | Entity | 0x640 | stat_cached.move_speed | r | champ · L294 reach 의 speed*12 · L317 range+speed*12 | 4 | OK |
| 8 | Entity | 0x660 | x | r | L294 target↔champ 거리² · L337/L341 champ 기준 거리² | 4 | OK |
| 9 | Entity | 0x668 | y | r | 동상 | 4 | OK |
| 10 | Entity | 0x5c0 | id | r | champ.id — L337 아군 자기 제외 | 4 | OK |
| 11 | PlayerState | 0x930 | info.team | r | L284 1-team(<2 bounds check) · L336 team(<2 bounds check) · L339 1-team | 4 | OK |
| 12 | OperationData | 0x0 | cache | r | L284 game · L336/L339 player_champion · L289 v17_ready_counterattack_damage 승격 인자 | 4 | OK |
| 13 | OperationData | 0x8 | context | r | L288 expected_damage_target 의 &GameContext · L289 승격 인자 | 4 | OK |
| 14 | OperationData | 0x10 | blackboard | r | L284/L340 [1-team] · 744B stride | 4 | OK |
| 15 | AbstractGameWithCache | 0x0 | game data ptr | r | is_recent_visible 인자 | 4 | OK |
| 16 | AbstractGameWithCache | 0x8 | game vtable ptr | r | is_recent_visible 인자 | 4 | OK |
| 17 | AbstractGameWithCache | 0x1e0 | player_champion[2][5] | r | L336 [team] · L339 [1-team] 각 5칸 완전 언롤 | 4 | OK |
| 18 | ScoreParameter | 0x918 | player (ChampionScoreParameter 216B 시작) | r | L323 possible_risk 의 self | 4 | OK |
| 19 | ScoreParameter | 0x988 | player.applyed_damage | r | L321 incoming 항 | 4 | OK |
| 20 | ScoreParameter | 0x998 | player.risk_damage | r | L322 incoming 항 | 4 | OK |
| 21 | ScoreParameter | 0x9b0 | player.risk_possible_tower | r | L324 incoming 항 /2 (sdiv) | 4 | OK |

**`consts` 상수 23건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 4 | 280 | 태그 | BattleSubPlanGoal 메모리태그 4 = RunAway / L317 range+speed*12 이내 가산 +4 | 4 |
| 1 | 13 | 282 | 태그 | EntityType 메모리태그 13 = Champion (target.ty) | 4 |
| 2 | 0 | 281 | 태그 | TeamType Player 태그 / 반환 0 / L350 clamp 하한 max(bonus,0) | 4 |
| 3 | 1 | 284 | 인덱스 | 1 - team (적팀 인덱스) / max(hp,1) 가드(L290·L325·L332) | 4 |
| 4 | 2 | 284 | 길이 | blackboard/player_champion 배열 길이 bounds check / L346 near_enemies < near_allies+2 | 4 |
| 5 | 20000 | 472 | 계수 | close_counterattack_target(인라인, battle_common.rs:471~473): reach = max_range(champ,target) + 20000 + champ.move_speed*12 | 4 |
| 6 | 12 | 472 | 계수 | reach 의 move_speed*12 (12틱분 이동) · L317 재사용(%80) | 4 |
| 7 | 100 | 296 | 계수 | % 환산: ready_damage*100 vs target_hp*70 / incoming*100 vs my_hp*60·30 / target.hp*100 vs max_hp*35 | 4 |
| 8 | 70 | 296 | 계수 | heavy_trade = escape_is_costly && close_counter && ready_damage*100 >= target_hp*70 (대상 HP 70% 이상을 준비된 콤보로 깎을 수 있음) | 4 |
| 9 | 75 | 301 | 산출값 | 기본 bonus: lethal_now(현재 이펙트만으로 즉사) | 4 |
| 10 | 48 | 301 | 산출값 | 기본 bonus: combo_lethal(준비된 콤보로 즉사) | 4 |
| 11 | 14 | 301 | 산출값 | 기본 bonus: heavy_trade 만 참 | 4 |
| 12 | 10 | 309 | 미상 | escape_is_costly 면 +10 / L316 dist<=range+15000 이면 +10 / L327 incoming>=my_hp 60% 면 +10 | 4 |
| 13 | 15000 | 315 | 계수 | dist <= max_range_cached+15000 이면 +10(L316) — 사거리 여유 | 4 |
| 14 | 90 | 323 | 미상 | possible_risk(parameter.player, data, 90) — 90틱 창의 위험 피해 | 4 |
| 15 | 60 | 326 | 계수 | incoming*100 >= my_hp*60 → +10(L327) | 4 |
| 16 | 30 | 328 | 계수 | incoming*100 >= my_hp*30 (60% 미만일 때) → +5 | 4 |
| 17 | 5 | 328 | 미상 | L328 +5 / L345 near_enemies<=near_allies → +5 | 4 |
| 18 | 35 | 332 | 계수 | target.hp*100 <= max(target.stat_cached.hp,1)*35 → +8 (대상 HP 35% 이하) | 4 |
| 19 | 8 | 332 | 미상 | 대상 저체력 가산 | 4 |
| 20 | 14400000001 | 337 | 임계 | 120000²+1 — 아군↔champ(L337)·가시 적↔champ(L341) 거리² 상한 | 4 |
| 21 | -15 | 346 | 미상 | near_enemies > near_allies 이고 !lethal_now && !combo_lethal && near_enemies >= near_allies+2 이면 -15 | 4 |
| 22 | 95 | 350 | 임계 | 최종 상한 clamp(0,95) | 4 |

**`knobs` 조정점 10건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | heavy_trade 대상HP 비율 | battle_common.rs:296 | 70 | 내리면 즉사가 아니어도 역공(14점 기본)이 더 자주 열림 | 4 | 기존 |
| 1 | 기본 bonus 3단 | battle_common.rs:301 | 75 / 48 / 14 | lethal_now / combo_lethal / heavy_trade 의 기본 점수 — 도주 중 역공 전환의 문턱 | 4 | 기존 |
| 2 | escape_is_costly 가산 | battle_common.rs:309 | 10 | 도주 비용이 클수록 역공 선호 | 4 | 기존 |
| 3 | 사거리 여유 가산(+10 / +4) | battle_common.rs:315~317 | 15000 / move_speed*12 | 15000 을 올리면 더 먼 대상에도 +10 | 4 | 기존 |
| 4 | 유입피해 비율 가산(+10 / +5) | battle_common.rs:326~328 | 60% / 30% | 내리면 적은 피해 예상에도 역공 가산 | 4 | 기존 |
| 5 | 대상 저체력 가산 | battle_common.rs:332 | 35% → +8 | 올리면 더 건강한 대상도 저체력 취급 | 4 | 기존 |
| 6 | 수적 열세 감점 | battle_common.rs:346 | -15 (near_enemies >= near_allies+2 · 즉사류 아님) | 절댓값을 올리면 수적 열세에서 역공 억제 | 4 | 기존 |
| 7 | 수적 동등/우세 가산 | battle_common.rs:345 | 5 | 올리면 아군 곁에서 역공 선호 | 4 | 기존 |
| 8 | 근접 반경 | battle_common.rs:337 / :341 | 14400000001 | 120000² — 아군/적 수 세는 반경 | 4 | 기존 |
| 9 | 최종 상한 | battle_common.rs:350 | 95 | 역공 가산 최대치 | 4 | 기존 |

<details><summary>`callees` 피호출자 10건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | distance | game_core::Entity::distance | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2161 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | expected_damage_target | game_core::Effect::expected_damage_target | pub | fn(&game_core::Effect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + , &game_core::Entity) -> usize | game-core\src\simulation\effect.rs:91 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 2 | is_recent_visible | game_core::Blackboard::is_recent_visible | pub | fn(&game_core::Blackboard, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\game\blackboard.rs:346 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 3 | iter_champions | game_core::AbstractGameWithCache::<'a, 'b>::iter_champions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5759 ~ game_core[6a30]::simulation::{impl#5}::iter_champions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1904 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 4 | max_range | game_ai::max_range | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-ai\src\plan_legacy\old\battle.rs:2234 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 5 | max_range_cached | game_ai::plan_legacy::old::max_range_cached | pub | fn(&game_core::OperationData, &game_core::Entity, &game_core::Entity) -> u64 | game-ai\src\plan_legacy\old\battle.rs:2332 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 6 | possible_risk | game_ai::ChampionScoreParameter::<'_>::possible_risk | pub | fn(&game_ai::ChampionScoreParameter</#0>, &game_core::OperationData, usize) -> i64 | game-ai\src\score_parameter.rs:1407 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 7 | v17_close_counterattack_target | game_ai::plan_legacy::sub_plan::battle_common::v17_close_counterattack_target | in:game_ai | fn(&game_core::Entity, &game_core::Entity) -> bool | game-ai\src\plan_legacy\sub_plan\battle_common.rs:471 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 8 | v17_escape_is_costly | game_ai::plan_legacy::sub_plan::battle_common::v17_escape_is_costly | in:game_ai | fn(&game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_core::Entity) -> bool | game-ai\src\plan_legacy\sub_plan\battle_common.rs:437 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 9 | v17_ready_counterattack_damage | game_ai::plan_legacy::sub_plan::battle_common::v17_ready_counterattack_damage | in:game_ai | fn(&game_core::OperationData, &game_core::Entity, &game_core::Entity) -> i64 | game-ai\src\plan_legacy\sub_plan\battle_common.rs:386 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
</details>

⚠**미매칭 2개**: `clamp`, `dist2`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 8곳** (m02.ll:37480, m02.ll:37678, m02.ll:37889, m02.ll:38221, m15.ll:11846, m15.ll:12075, m15.ll:12327, m15.ll:12661) · **형제 0개** 

**`open` 5건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 표기 불가 | L346 `lethal_now \|\| combo_lethal \|\| near_enemies < near_allies+2` 의 소스 순서 — 세 항 모두 값이라 IR 에 순서 흔적 없음(표기 불가·외연 동일) | 4 |  |
| 1 | 표기 불가 | L281/L282 결합 순서 — 한 select 로 접힘(표기 불가) | 4 |  |
| 2 | 미탐색 | `v17_ready_counterattack_damage`(m05.ll 57982~58185, internal fastcc · IR 인자 승격 (cache,context,champ,target))·`v17_escape_is_costly`(53479~53991)·`max_range`(m10.ll 52940)·`max_range_cached`(m10.ll 50689) 내부는 범위 밖 — 계약만: ready(&OperationData,&Entity,&Entity)->i64 / escape(&PlayerState,&OperationData,&ScoreParameter,&Entity)->bool / max_range(&Entity,&Entity)->u64 / max_range_cached(&OperationData,&Entity,&Entity)->u64 | 4 |  |
| 3 | 미탐색 | `_version`(i64 %0) 미사용 — 버전 분기 없음 | 4 |  |
| 4 | 미탐색 | blackboard 인덱스 `1-team` 의 의미(팀별 보드 vs 상대팀 보드) — game_core 소관 | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

