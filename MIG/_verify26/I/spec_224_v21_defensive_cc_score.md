---

### `224` v21_defensive_cc_score — 방어용 CC 스킬(cc_time 있음)을 가시·타겟가능 적 챔피언에 쓸 가치(0~160) — 이동/돌진형 이펙트 제외, 시전 사거리+여유 이내에서 CC시간·사거리 안팎·직접위협·유입피해·수적 열세·내 HP%·즉시위협권으로 합산

| 항목 | 값 |
|---|---|
| id | `battle_common__v21_defensive_cc_score` |
| 심볼 | `_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy8sub_plan13battle_common22v21_defensive_cc_score` |
| 소스 | `game-ai\src\plan_legacy\sub_plan\battle_common.rs:540` |
| IR | `m05.ll` 54048~54564행 |
| 경로·가시성 | `game_ai::plan_legacy::sub_plan::battle_common::v21_defensive_cc_score` · **in:game_ai** |
| 계층 | 기타 |
| exe | `d67060` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r17` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_core::Entity, &game_core::Entity, &game_core::Effect, std::option::Option<usize>) -> i64
```

**TLS 메모 절(`sig.tls` · 정정 경로 `/specs[224]/sig/tls/<키>`)**

- `name`: (직접 접점 없음 · 콜리 경유)
- `role`: 소비자(간접)
- `key`: 본체·aux 클로저 2개 모두 `LocalKey`/`call_once` 참조 0건. 콜리 `battle::max_range_cached`(m10.ll 50689) 가 TLS `old::battle::MAX_RANGE_CACHE`(RefCell<MaxRangeCache>) 를 읽고 씀
- `layout`: 콜리 명세 소관
- `invalidation`: 콜리 명세 소관
- `call_conditions`: max_range_cached 호출 순서: ①aux closure#1 안에서 가시 적마다 (data, enemy, champ) 최대 5회(L590 · 거리 판정 전에 무조건 호출) ②L595 (data, target, champ) 1회 ③L628 (data, target, champ) 1회(L597 게이트 통과 후). 나머지 콜리(is_recent_visible·can_target·is_enemy_well_danger·CastingTarget::check·range_adjust·Entity::distance·possible_risk) 는 TLS 참조 0 확인

<details><summary>인자 8개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | version | usize | IR %0 · L560 path_finder::is_enemy_well_danger 에 그대로 전달(본문 자체의 버전 분기 없음) | 4 |
| 1 | 2 | player | &PlayerState(2528B) | IR %1 · readonly · info.team(+0x930) · is_recent_visible/can_target/is_enemy_well_danger 인자 · 클로저 캡처 | 4 |
| 2 | 3 | data | &OperationData(24B) | IR %2 · readonly · cache(+0)·blackboard(+0x10) · can_target 의 self · max_range_cached/possible_risk 인자 | 4 |
| 3 | 4 | parameter | &ScoreParameter(5384B) | IR %3 · readonly · player 블록(+0x918) applyed_damage/risk_damage/risk_possible_tower · possible_risk self | 4 |
| 4 | 5 | champ | &Entity(1728B) | IR %4 · readonly · 시전자(DI 별칭 caster). team·level(+0x5c8)·stat_buff_cached.range(+0x438)·radius_mult(+0x470)·radius(+0x680)·move_speed(+0x640)·x·y·hp·stat_cached.hp | 4 |
| 5 | 6 | target | &Entity(1728B) | IR %5 · readonly · 적. team·ty@tag·cc_immune(+0x468)·cc Vec·radius_mult·radius·move_speed·x·y | 4 |
| 6 | 7 | effect | &Effect(56B) | IR %6 · readonly · ty(Arc<dyn EffectType>) 의 expected_rush_effect/expected_move_on_hit/expected_move_distance · range(+0x10)·growth_range(+0x18)·target(+0x28 CastingTarget) · range_adjust self | 4 |
| 7 | 8 | cc_time | Option<usize> | IR 스칼라 2개: %7 태그(range(0,2)) · %8 페이로드. None 또는 Some(0) 이면 0 (L550) | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
L550  let cc_time = match cc_time { Some(t) if t != 0 => t, _ => return 0 };   // IR: 태그==1 && 페이로드≠0 (option.rs:1580 인라인 · filter/is_some_and 표기 불명)
L554  if target.team == champ.team { return 0 }
L555  if target.ty 태그 != 13(Champion) { return 0 }                      // L554·555 한 select 로 접힘
L556  if target.stat_buff_cached.cc_immune { return 0 }
L557  if target.cc.iter().any(|c| c.is_cc()) { return 0 }                // 이미 하드 CC 중
L558  let enemy_team = 1 - player.info.team;  if !data.blackboard[enemy_team].is_recent_visible(game, player, target) { return 0 }
L559  if !data.can_target(game, player, target) { return 0 }
L560  if path_finder::is_enemy_well_danger(version, player, target.x, target.y) { return 0 }   // 적 우물 위험 좌표
L561  if effect.ty.expected_rush_effect()  { return 0 }                 // 돌진형 제외
L562  if effect.ty.expected_move_on_hit()  { return 0 }                 // 피격 이동형 제외
L563  if effect.ty.expected_move_distance().is_some() { return 0 }      // 이동거리 있는 이펙트 제외
L564  if !effect.target.check(champ, target) { return 0 }               // CastingTarget 적합성
L568  let cast_range = effect.range(champ)                               // effect.rs:26 = range + (champ.level-1)*growth_range + champ.stat_buff_cached.range
                     + effect.range_adjust(champ, target)
                     + champ.radius() + target.radius();                // entity.rs:1511: radius_mult≠0 ? radius*(mult+100)/100 : radius
L569  let dist = target.distance(champ);
L570  if dist > cast_range + 10000 + champ.stat_cached.move_speed*8 { return 0 }     // shl 3
L574  let incoming = parameter.player.applyed_damage + parameter.player.risk_damage      // L574~575
L576               + parameter.player.possible_risk(data, 90)
L577               + parameter.player.risk_possible_tower / 2;
L578  let hp = max(champ.hp, 1);
L579  let hp_ratio = champ.hp*100 / max(champ.stat_cached.hp, 1);
L581  let near_allies  = data.cache.player_champion[team].iter().flatten().filter(|a| a.id != champ.id && dist2(a,champ) < 120000²+1).count() + 1;   // aux closure#0 (L582~583)
L584  let near_enemies = data.cache.player_champion[enemy_team].iter().flatten().filter(|e|                                                          // aux closure#1 (L585~593)
L586        data.blackboard[1 - player.info.team].is_recent_visible(game, player, e)
L590        && { let threat = max_range_cached(data, e, champ) + 40000;                                          // 거리 판정 전에 호출
L591              dist2(e,champ) < 120000²+1 || dist2(e,champ) <= threat² }).count();
L595  let catch_range = max_range_cached(data, target, champ) + 25000 + target.stat_cached.move_speed*18;
L596  let direct_threat = dist2(target, champ) <= catch_range²;        // dbg DW_OP_not 확인 · 분기 방향 일치
L597  if !direct_threat && incoming*100 < hp*20 && near_enemies <= near_allies { return 0 }
L601  let mut bonus = ((cc_time as i64)/3).clamp(10, 45);
L602  bonus += if dist > cast_range { 16 } else { 35 };
L609  if direct_threat { bonus += 22 }
L612  if incoming*100 >= hp*35 { bonus += 24 }                          // L613
L614  else if incoming*100 >= hp*15 { bonus += 10 }
L618  if near_enemies > near_allies {
L619      bonus += min((near_enemies - near_allies)*8, 24); }           // shl 3
L622  bonus += if hp_ratio < 45 { 14 } else if hp_ratio < 65 { 6 } else { 0 };
L628  let immediate_threat_range = max_range_cached(data, target, champ) + 30000;
L629  if dist2(target, champ) <= immediate_threat_range² { bonus += 12 }
L633  return min(bonus, 160);                                           // L634

※ dbg 이름: cc_time·cast_range·dist·incoming·hp·hp_ratio·near_allies·near_enemies·catch_range·direct_threat·immediate_threat_range·bonus.
※ L597 의 세 항 결합 순서·L622 의 else-if 는 IR select 체인(hp_ratio<45 우선, 다음 <65)으로 복원.
```

**`mem` 메모리 접근 32건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | Entity | 0x0 | team@tag | r | L554 target.team == champ.team → 0 | 4 | OK |
| 1 | Entity | 0x8 | team@Player.0 | r | L554 페이로드 비교 | 4 | OK |
| 2 | Entity | 0x68 | ty@tag | r | L555 target.ty == 13(Champion) | 4 | OK |
| 3 | Entity | 0x468 | stat_buff_cached.cc_immune | r | L556 target | 4 | OK |
| 4 | Entity | 0x2c8 | cc.buf.ptr | r | L557 target.cc 슬라이스(CCState 40B) | 4 | OK |
| 5 | Entity | 0x2d0 | cc.len | r | L557 | 4 | OK |
| 6 | Entity | 0x660 | x | r | target(L560 is_enemy_well_danger 인자 · L596/L629 거리²) · champ · aux 아군/적 | 4 | OK |
| 7 | Entity | 0x668 | y | r | 동상 | 4 | OK |
| 8 | Entity | 0x5c8 | level | r | L568 champ.level — effect.range(): (level-1)*growth_range | 4 | OK |
| 9 | Entity | 0x438 | stat_buff_cached.range | r | L568 champ — effect.range() 가산항(effect.rs:26) | 4 | OK |
| 10 | Entity | 0x470 | stat_buff_cached.radius_mult | r | L568 champ·target — entity.rs:1511 radius(): mult≠0 이면 radius*(mult+100)/100 | 4 | OK |
| 11 | Entity | 0x680 | radius | r | L568 champ·target | 4 | OK |
| 12 | Entity | 0x640 | stat_cached.move_speed | r | L570 champ*8 · L595 target*18 | 4 | OK |
| 13 | Entity | 0x670 | hp | r | L578 champ hp=max(…,1) · L579 hp_ratio 분자 | 4 | OK |
| 14 | Entity | 0x628 | stat_cached.hp | r | L579 champ 최대HP | 4 | OK |
| 15 | Entity | 0x5c0 | id | r | aux closure#0: 아군 id ≠ champ.id | 4 | OK |
| 16 | CCState | 0x0 | tag(i32) | r | L557 is_cc(): {0,1,2,6,8,9} | 4 | OK |
| 17 | Effect | 0x0 | ty (Arc data ptr) | r | L561~563 ArcInner 페이로드 = ptr + ((align-1)&~15) + 16 | 4 | OK |
| 18 | Effect | 0x8 | ty vtable ptr | r | +0x10 align · +0x60 expected_rush_effect · +0x68 expected_move_on_hit · +0x58 expected_move_distance (divtable) | 3 | OK |
| 19 | Effect | 0x10 | range | r | L568 effect.range() | 4 | OK |
| 20 | Effect | 0x18 | growth_range | r | L568 (level-1)*growth_range | 4 | OK |
| 21 | Effect | 0x28 | target (CastingTarget 4B) | r | L564 CastingTarget::check(self, champ, target) | 4 | OK |
| 22 | PlayerState | 0x930 | info.team | r | L558 1-team(<2 bounds) · L581 team(<2 bounds) · L584 1-team · aux closure#1 1-team | 4 | OK |
| 23 | OperationData | 0x0 | cache | r | game · player_champion | 4 | OK |
| 24 | OperationData | 0x10 | blackboard | r | L558 · aux closure#1 [1-team] · 744B stride | 4 | OK |
| 25 | AbstractGameWithCache | 0x0 | game data ptr | r | is_recent_visible/can_target 인자 | 4 | OK |
| 26 | AbstractGameWithCache | 0x8 | game vtable ptr | r | 동상 | 4 | OK |
| 27 | AbstractGameWithCache | 0x1e0 | player_champion[2][5] | r | L581 [team] 슬라이스 · L584 [1-team] 슬라이스 → 클로저에 begin/end 전달 | 4 | OK |
| 28 | ScoreParameter | 0x918 | player (ChampionScoreParameter 시작) | r | L576 possible_risk self | 4 | OK |
| 29 | ScoreParameter | 0x988 | player.applyed_damage | r | L574 | 4 | OK |
| 30 | ScoreParameter | 0x998 | player.risk_damage | r | L575 | 4 | OK |
| 31 | ScoreParameter | 0x9b0 | player.risk_possible_tower | r | L577 /2 | 4 | OK |

**`consts` 상수 31건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev | folded_from |
|---|---|---|---|---|---|---|
| 0 | 0 | 550 | 태그 | Some(0) 배제(cc_time≠0) / L563 Option<(usize,u64)> 태그 0 = None 이어야 계속 / TeamType Player 태그 / 반환 0 | 4 |  |
| 1 | 13 | 555 | 태그 | EntityType 메모리태그 13 = Champion | 4 |  |
| 2 | 1 | 557 | 태그 | CCState Stun 태그 / 1-team / max(hp,1) 가드 / L581 near_allies = count+1(자기 포함) | 4 |  |
| 3 | 2 | 557 | 태그 | CCState Bind 태그 / 배열 길이 bounds / L577 risk_possible_tower/2 (sdiv) | 4 |  |
| 4 | 6 | 557 | 태그 | CCState ForceMove 태그 / L622 hp_ratio<65 가산 +6 | 4 |  |
| 5 | 8 | 557 | 태그 | CCState Fear 태그(리터럴) — L570·L619 의 ×8 은 `shl 3` 접힘(아래 value 3 항목) | 4 |  |
| 6 | 9 | 557 | 태그 | CCState Charm 태그 | 4 |  |
| 7 | 100 | 568 | 계수 | radius*(mult+100)/100 (entity.rs:1511~1515) / L579 hp*100 / L597·L612 incoming*100 | 4 |  |
| 8 | 3 | 570 | 계수 | `shl i64 %150, 3` = champ.move_speed*8 (L570 catch 여유) · `shl i64 %269, 3` = (near_enemies-near_allies)*8 (L619) — 리터럴 8 은 CC 태그뿐 | 4 | 8 |
| 9 | 3 | 601 | 계수 | bonus 기본 = (cc_time/3).clamp(10,45) (sdiv 3 · 리터럴) — 같은 값의 `shl 3` 접힘은 위 folded_from 항목 | 4 |  |
| 10 | 10000 | 570 | 계수 | dist > cast_range + 10000 + champ.move_speed*8 이면 0 | 4 |  |
| 11 | 90 | 576 | 미상 | possible_risk(parameter.player, data, 90) — 90틱 창 | 4 |  |
| 12 | 14400000001 | 582 | 임계 | 120000²+1 — aux closure#0 아군↔champ / closure#1 적↔champ 거리² 상한 | 4 |  |
| 13 | 40000 | 590 | 미상 | aux closure#1: 적의 max_range_cached(data,enemy,champ)+40000 이내면 '근접 적' 으로 셈(120000 초과여도) | 4 |  |
| 14 | 25000 | 595 | 계수 | catch_range = max_range_cached(data,target,champ) + 25000 + target.move_speed*18 | 4 |  |
| 15 | 18 | 595 | 계수 | catch_range 의 target.move_speed*18 | 4 |  |
| 16 | 20 | 597 | 계수 | !direct_threat 이고 incoming*100 < hp*20 이고 near_enemies<=near_allies 이면 0 | 4 |  |
| 17 | 10 | 601 | 임계 | clamp 하한 10 / L614 incoming>=hp 15% 가산 +10 | 4 |  |
| 18 | 45 | 601 | 임계 | clamp 상한 45 / L622 hp_ratio<45 → +14 | 4 |  |
| 19 | 16 | 602 | 산출값 | dist > cast_range 이면 +16 (사거리 밖·접근 필요) | 4 |  |
| 20 | 35 | 602 | 계수 | dist <= cast_range 이면 +35 / L612 incoming*100 >= hp*35 → +24 | 4 |  |
| 21 | 22 | 609 | 미상 | direct_threat(dist2 <= catch_range²) 이면 +22 | 4 |  |
| 22 | 24 | 613 | 인덱스 | incoming >= hp 35% → +24 / L619 min((ne-na)*8, 24) | 4 |  |
| 23 | 15 | 614 | 계수 | incoming*100 >= hp*15 (35% 미만일 때) → +10 | 4 |  |
| 24 | 14 | 622 | 미상 | hp_ratio < 45 → +14 | 4 |  |
| 25 | 65 | 622 | 임계 | hp_ratio < 65 (45 이상일 때) → +6 | 4 |  |
| 26 | 30000 | 628 | 계수 | immediate_threat_range = max_range_cached(data,target,champ) + 30000; dist2 <= 그 제곱이면 +12 (L629) | 4 |  |
| 27 | 12 | 629 | 미상 | 즉시위협권 가산 | 4 |  |
| 28 | 160 | 633 | 임계 | 최종 상한 min(bonus,160) | 4 |  |
| 29 | -1 | 568 | 계수 | (level-1) / Arc align-1 · 판정값 아님 | 4 |  |
| 30 | -16 | 561 | 계수 | (align-1)&~15 Arc 헤더 정렬 · 판정값 아님 | 4 |  |

**`knobs` 조정점 11건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | CC 시간 기본 clamp | battle_common.rs:601 | 10..45 (cc_time/3) | 긴 CC 일수록 방어 CC 가치↑ — 상한 45 를 올리면 장시간 CC 가 더 반영 | 4 | 기존 |
| 1 | 사거리 안/밖 가산 | battle_common.rs:602 | 35 / 16 | 밖(16)을 올리면 접근이 필요한 대상도 선호 | 4 | 기존 |
| 2 | 시전 허용 여유 | battle_common.rs:570 | 10000 + move_speed*8 | 올리면 더 먼 적에게도 방어 CC 후보 성립 | 4 | 기존 |
| 3 | 직접 위협 판정 여유 | battle_common.rs:595 | 25000 + target.move_speed*18 | 올리면 direct_threat 가 자주 참 → +22 및 L597 게이트 우회 | 4 | 기존 |
| 4 | 약한 상황 컷 | battle_common.rs:597 | incoming<20% && 수적 열세 아님 | 20 을 올리면 더 많은 상황이 0 으로 컷 | 4 | 기존 |
| 5 | 유입피해 가산 | battle_common.rs:612~614 | 35%→+24 / 15%→+10 | 내리면 적은 피해 예상에도 방어 CC 선호 | 4 | 기존 |
| 6 | 수적 열세 가산 | battle_common.rs:619 | min(차이*8, 24) | 열세일수록 CC 로 버티기 — 24 상한 | 4 | 기존 |
| 7 | 내 HP% 가산 | battle_common.rs:622 | <45→+14 / <65→+6 | 저체력일수록 방어 CC 선호 | 4 | 기존 |
| 8 | 즉시위협권 | battle_common.rs:628 | max_range_cached+30000 → +12 | 올리면 더 먼 적도 즉시 위협 | 4 | 기존 |
| 9 | 적 근접 판정 확장 | battle_common.rs:590 | 40000 | 적 사거리+40000 — 올리면 원거리 적도 near_enemies 에 포함 | 4 | 기존 |
| 10 | 최종 상한 | battle_common.rs:633 | 160 | 방어 CC 점수 최대 | 4 | 기존 |

<details><summary>`callees` 피호출자 20건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | can_target | game_core::OperationData::<'a, 'b>::can_target | pub | fn(&game_core::OperationData<'a/#0, 'b/#1>, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\state\player.rs:23 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | check | game_core::CastingTarget::check | pub | fn(&game_core::CastingTarget, &game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\effect\type.rs:227 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 2 | distance | game_core::Entity::distance | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2161 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 3 | expected_move_distance | game_core::EffectType::expected_move_distance | pub | fn(&Self/#0) -> std::option::Option<(usize, u64)> | game-core\src\simulation\effect\type.rs:289 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 14개 중 상위 3개 |
| 4 | expected_move_distance | <game_core::RushEffect as game_core::EffectType>::expected_move_distance | pub | fn(&game_core::RushEffect) -> std::option::Option<(usize, u64)> | game-core\src\simulation\effect\type\rush.rs:45 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 14개 중 상위 3개 |
| 5 | expected_move_distance | <game_core::MoveToEffect as game_core::EffectType>::expected_move_distance | pub | fn(&game_core::MoveToEffect) -> std::option::Option<(usize, u64)> | game-core\src\simulation\effect\type\move_to.rs:54 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 14개 중 상위 3개 |
| 6 | expected_move_on_hit | game_core::EffectType::expected_move_on_hit | pub | fn(&Self/#0) -> bool | game-core\src\simulation\effect\type.rs:293 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 20개 중 상위 3개 |
| 7 | expected_move_on_hit | <game_core::CombineEffect as game_core::EffectType>::expected_move_on_hit | pub | fn(&game_core::CombineEffect) -> bool | game-core\src\simulation\effect\type\combine.rs:96 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 20개 중 상위 3개 |
| 8 | expected_move_on_hit | <game_core::DelayedEffect as game_core::EffectType>::expected_move_on_hit | pub | fn(&game_core::DelayedEffect) -> bool | game-core\src\simulation\effect\type\delayed.rs:45 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 20개 중 상위 3개 |
| 9 | expected_rush_effect | game_core::EffectType::expected_rush_effect | pub | fn(&Self/#0) -> bool | game-core\src\simulation\effect\type.rs:291 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 9개 중 상위 3개 |
| 10 | expected_rush_effect | <game_core::RushEffect as game_core::EffectType>::expected_rush_effect | pub | fn(&game_core::RushEffect) -> bool | game-core\src\simulation\effect\type\rush.rs:53 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 9개 중 상위 3개 |
| 11 | expected_rush_effect | <game_core::CombineEffect as game_core::EffectType>::expected_rush_effect | pub | fn(&game_core::CombineEffect) -> bool | game-core\src\simulation\effect\type\combine.rs:42 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 9개 중 상위 3개 |
| 12 | is_cc | game_core::CCState::is_cc | pub | fn(&game_core::CCState) -> bool | game-core\src\simulation\entity.rs:533 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 13 | is_enemy_well_danger | game_ai::is_enemy_well_danger | pub | fn(usize, &game_core::PlayerState, u64, u64) -> bool | game-ai\src\path_finder.rs:1032 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 14 | is_recent_visible | game_core::Blackboard::is_recent_visible | pub | fn(&game_core::Blackboard, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\game\blackboard.rs:346 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 15 | max_range_cached | game_ai::plan_legacy::old::max_range_cached | pub | fn(&game_core::OperationData, &game_core::Entity, &game_core::Entity) -> u64 | game-ai\src\plan_legacy\old\battle.rs:2332 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 16 | possible_risk | game_ai::ChampionScoreParameter::<'_>::possible_risk | pub | fn(&game_ai::ChampionScoreParameter</#0>, &game_core::OperationData, usize) -> i64 | game-ai\src\score_parameter.rs:1407 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 17 | radius | game_core::Entity::radius | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1509 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 18 | range | game_core::Effect::range | pub | fn(&game_core::Effect, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:25 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 19 | range_adjust | game_core::Effect::range_adjust | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:29 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
</details>

⚠**미매칭 2개**: `clamp`, `dist2`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 4곳** (m05.ll:58210, m05.ll:60680, m05.ll:60714, m05.ll:60795) · **형제 0개** 

**`open` 7건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 표기 불가 | L550 소스 표기(`filter(\|t\| *t>0)` vs `match … Some(t) if t!=0`) — option.rs:1580 인라인만 남아 표기 불가(외연: Some 이고 ≠0) | 4 |  |
| 1 | 표기 불가 | L597 세 항의 소스 순서 — 값 결합(and)이라 IR 에 순서 흔적 없음(표기 불가) | 4 |  |
| 2 | 표기 불가 | L554/L555 결합 순서 — 한 select 로 접힘(표기 불가) | 4 |  |
| 3 | 미탐색 | aux closure#1 의 `dist<120000 \|\| dist<=threat` 순서: IR 은 120000 비교를 먼저 하고 실패 시 threat 비교(관측) · max_range_cached 는 둘 다보다 앞서 무조건 호출됨(TLS 캐시 부작용 순서에 영향) | 4 |  |
| 4 | 재료 부재 | expected_rush_effect/expected_move_on_hit/expected_move_distance 의 런타임 구현체별 값 — Arc<dyn EffectType> 이라 정적 확정 불가 | 4 |  |
| 5 | 미탐색 | 콜리 내부(범위 밖): can_target(&OperationData, game:&dyn AbstractGame, &PlayerState, &Entity)->bool · is_enemy_well_danger(usize version, &PlayerState, x:u64, y:u64)->bool (path_finder — r18) · CastingTarget::check(&self,&Entity caster,&Entity target)->bool · range_adjust(&Effect,&Entity,&Entity)->u64 · max_range_cached(&OperationData,&Entity attacker,&Entity target)->u64 | 4 |  |
| 6 | 미탐색 | 반환 range 하한 -2^63+26 이 실제로 도달 가능한지(bonus 항이 전부 비음수라 실제 하한은 26 이라고 봄) — smin 형식 범위 | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

