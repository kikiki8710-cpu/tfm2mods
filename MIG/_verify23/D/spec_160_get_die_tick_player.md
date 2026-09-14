---

### `160` get_die_tick_player — 적 챔피언(공/Q/W 도달틱별 dps)·타워·적 기타 엔티티의 dps 를 도달 시각순으로 누적해 내 챔피언이 죽는 틱을 계산

| 항목 | 값 |
|---|---|
| id | `goal_data__get_die_tick_player` |
| 심볼 | `_RNvNtCshdEBA0ozCnw_7game_ai9goal_data19get_die_tick_player` |
| 소스 | `game-ai\src\goal_data.rs:577` |
| IR | `m09.ll` 52061~52663행 |
| 경로·가시성 | `game_ai::get_die_tick_player` · **pub** |
| 계층 | 기타 |
| exe | `de3630` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r14` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_core::OperationData, usize, usize, &bumpalo::collections::vec::Vec< usize>, std::option::Option<&game_core::Entity>, std::option::Option<game_core::SmallAction>) -> u64
```

<details><summary>인자 6개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | data | &OperationData(24B) | +0 cache → player_champion/player_champion_cache/others · +8 context → setting.tick_per_second, pool(bump 할당자) | 4 |
| 1 | 2 | champ_team | usize | 내 팀(0/1). 스택 슬롯 %13 에 저장돼 클로저가 참조로 캡처(52070·52121). 적 팀 = 1-champ_team | 4 |
| 2 | 3 | champ | usize | 내 포지션 인덱스(0..5). %12 에 저장, 캡처 | 4 |
| 3 | 4 | enemies | &bumpalo::Vec<usize>(32B: +0 ptr / +0x18 len) | 적 챔피언 포지션 인덱스 목록. flat_map 의 원 이터레이터(52085~52097) | 4 |
| 4 | 5 | tower | Option<&Entity> | null=None(52168). Some 이면 dps_list 에 (0, 타워dps) 추가 | 4 |
| 5 | 6 | action | Option<SmallAction>(24B, +0 태그 8B) | 클로저가 캡처(52125). 태그==0(Some(RunAway), SmallAction Direct 태그 0=RunAway) 이면 적 이속에서 내 이속을 뺀다(aux m01.ll:51602~51604·51714~51717) | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn get_die_tick_player(data, champ_team, champ, enemies:&Vec<usize>, tower:Option<&Entity>, action:Option<SmallAction>) -> u64
// L578~605: dps_list: bumpalo Vec<(u64 arrive_tick, u64 dps)> = enemies.iter().flat_map(|&e| closure#0).collect_in(ctx.pool)   [aux m01.ll:50863~51883]
//   closure#0(e):
//     enemy = 1 - champ_team (bounds 2), e<5, champ<5 (bounds)
//     cc = cache.player_champion_cache[enemy][e]                         // L579
//     attack_dps = cc.attack_per_sec[champ]; skill_dps = cc.skill_per_sec[champ]; skill2_dps = cc.skill2_per_sec[champ]   // L579~581
//     champ_e = cache.player_champion[champ_team][champ].unwrap()       // L583
//     e_e     = cache.player_champion[enemy][e].unwrap()                // L584
//     range_of(ef) = ef.range(e_e) /*= ef.range + ef.growth_range*(e_e.level-1) + e_e.stat_buff_cached.range*/ + ef.range_adjust(e_e, champ_e) + e_e.radius() + champ_e.radius()
//     attack_range = e_e.attack_effect.map(range_of).unwrap_or(0)       // L585
//     skill_range  = e_e.skill_effect.map(range_of).unwrap_or(0)        // L586
//     skill2_range = e_e.skill2_effect()/*level>2 만*/.map(range_of).unwrap_or(0)   // L587
//     dist = e_e.distance(champ_e)                                       // L589
//     move_speed = e_e.stat_cached.move_speed                            // L590
//     if action == Some(RunAway) { move_speed = move_speed.saturating_sub(champ_e.stat_cached.move_speed) }   // L592~593 (태그 0)
//     move_speed = max(move_speed, 1)                                    // L596
//     cc_tick = e_e.cc.iter().filter(|c| c.block_input()).map(|c| c.tick()).max().unwrap_or(0)   // L598 [aux m12.ll:18848 fold + m09.ll:65639 shim]
//     ceil_div(a,b) = (a + b - 1)/b
//     [(ceil_div(dist.sat_sub(attack_range), move_speed) + cc_tick, attack_dps),   // L602
//      (ceil_div(dist.sat_sub(skill_range),  move_speed) + cc_tick, skill_dps),    // L603
//      (ceil_div(dist.sat_sub(skill2_range), move_speed) + cc_tick, skill2_dps)]   // L604
// L607
champ_e = cache.player_champion[champ_team][champ].unwrap()
// L610~614
if let Some(t) = tower {
    dps = t.attack_effect.unwrap().expected_damage_target(ctx, t as &dyn AbstractEntity, champ_e) * setting.tick_per_second / t.attack_cooltime()   // cooltime 0 → panic(div by zero, 가드 없음)
    dps_list.push((0, dps))
}
// L618~626
for e in cache.others[1-champ_team].iter() {
    if let Some(atk) = e.attack_effect {
        dps = atk.expected_damage_target(ctx, e, champ_e) * tps / max(e.attack_cooltime(), 1)           // L620~621
        attack_range = atk.range(e) + atk.range_adjust(e, champ_e) + e.radius() + champ_e.radius()        // L622
        dist = e.distance(champ_e)                                                                        // L623
        arrive_tick = dist.saturating_sub(attack_range) / max(e.stat_cached.move_speed, 1)               // L624~625 (floor, cc 무시)
        dps_list.push((arrive_tick, dps))                                                                 // L626
    }
}
dps_list.push((9999999999, 0))                 // L630 센티넬
dps_list.sort_by_key(|x| x.0)                  // L632 (도달 틱 오름차순, stable)
hp = champ_e.hp * 100                          // L633
(tick, dps, total) = (0, 0, 0)
for (t, add_dps) in dps_list {                 // L639
    dt = t - tick                              // L640
    add_max = dps * 100 * dt / 60              // L641
    if total + add_max >= hp {                 // L642 (icmp ult total+add_max, hp 의 부정)
        remain_hp = hp - total                 // L643 (add_max 더하기 전)
        return tick + remain_hp / max(dps, 1)  // L644~645
    }
    total += add_max; dps += add_dps; tick = t // L649
}
return if tick == 0 { 9999999999 } else { tick }   // L653/637 — 정상 경로에선 센티넬 9999999999
```

**`mem` 메모리 접근 30건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | OperationData | 0x0 | cache | r | 52102 | 4 | OK |
| 1 | OperationData | 0x8 | context | r | 52126~52127; context+0 = pool(52128, from_iter_in 할당자) · context+8 = setting | 4 | OK |
| 2 | GameContext | 0x8 | setting | r | 52201~52202 · 52289 | 4 | OK |
| 3 | GameSetting | 0x12f8 | tick_per_second | r | dps = dmg*tps/cooltime (52203~52205 · 52364~52366) | 4 | OK |
| 4 | bumpalo Vec<usize>(enemies) | 0x18 | len | r | 52087~52088 (ptr 는 +0) | 4 | 확인불가(tcx 사전에 타입 없음) |
| 5 | AbstractGameWithCache | 0x1e0 | player_champion[team][pos] | r | 52153~52156 (내 챔피언 unwrap, 52172 unwrap_failed) · 클로저 51259~51277 (내/적 챔피언 unwrap) | 4 | OK |
| 6 | AbstractGameWithCache | 0xf0 | others[1-champ_team].buf.ptr | r | 52270~52274 stride 32, len +0x18(52276~52277) | 4 | OK |
| 7 | AbstractGameWithCache | 0x280 | player_champion_cache[1-champ_team][e] | r | 클로저 51227~51228 (ChampionCache 800B, 팀당 5) | 4 | OK |
| 8 | ChampionCache | 0x190 | attack_per_sec[champ] | r | 클로저 51229~51231 attack_dps | 4 | OK |
| 9 | ChampionCache | 0x1b8 | skill_per_sec[champ] | r | 클로저 51233~51235 skill_dps | 4 | OK |
| 10 | ChampionCache | 0x1e0 | skill2_per_sec[champ] | r | 클로저 51237~51239 skill2_dps | 4 | OK |
| 11 | Entity | 0x4c0 | attack_effect@tag | r | i32 -1=None. 타워 52178~52180(None 이면 unwrap_failed 52197) · others 52321~52323 · 클로저 51286~51288 | 4 | OK |
| 12 | Entity | 0x490 | attack_effect@Some.0 (Effect) | r | expected_damage_target self · range_adjust self (52190 · 52320 · 51298) | 4 | OK |
| 13 | Entity | 0x4a0 | attack_effect.range | r | Effect::range 인라인(effect.rs:26): range + growth_range*(level-1) + stat_buff_cached.range (52377~52384) | 4 | OK |
| 14 | Entity | 0x4a8 | attack_effect.growth_range | r | 52379~52380 | 4 | OK |
| 15 | Entity | 0x4f8 | skill_effect@tag | r | 클로저 51391~51394 (-1=None) | 4 | OK |
| 16 | Entity | 0x4c8 | skill_effect@Some.0 (Effect) | r | 클로저 51397; range +0x4d8(1240)·growth +0x4e0(1248) = 51412~51416 | 4 | OK |
| 17 | Entity | 0x500 | skill2_effect@Some.0 (Effect) | r | 클로저 51491: Entity::skill2_effect(entity.rs:1693) 인라인 — level>2 면 &self.skill2_effect 아니면 정적 None(@anon…32, 태그 -1) | 4 | OK |
| 18 | Entity | 0x30 | skill2_effect@tag (선택된 Effect ptr+0x30 · 절대 0x530) | r | 51494~51496 (선택된 ptr+48) | 4 | 오귀속(사전은 다른 필드를 준다) |
| 19 | Entity | 0x5c8 | level | r | growth_range*(level-1) (52381) · skill2_effect 게이트 level>2 (51488~51490) | 4 | OK |
| 20 | Entity | 0x438 | stat_buff_cached.range | r | Effect::range 가산항 (52383) | 4 | OK |
| 21 | Entity | 0x470 | stat_buff_cached.radius_mult | r | Entity::radius 인라인(entity.rs:1511~1515): mult==0 → radius, 아니면 radius*(mult+100)/100 (52391~52409) | 4 | OK |
| 22 | Entity | 0x680 | radius | r | 52398 · 52405 | 4 | OK |
| 23 | Entity | 0x640 | stat_cached.move_speed | r | 적 이속(52449~52450 · 51596~51597) · RunAway 시 내 이속 차감(51714~51715) | 4 | OK |
| 24 | Entity | 0x670 | hp | r | 내 챔피언 hp*100 (52560~52562) | 4 | OK |
| 25 | Entity | 0x2c8 | cc.buf.ptr | r | 클로저 51615~51616 CCState 40B 배열 | 4 | OK |
| 26 | Entity | 0x2d0 | cc.len | r | 51617~51618 | 4 | OK |
| 27 | CCState | 0x0 | tag(i32) | r | block_input 인라인(entity.rs:521): (tag-6) as u32 < 0xFFFFFFFC ⟺ tag ∉ {2 Bind,3 BlockAttack,4 BlockSkill,5 BlockMoveSkill} (m09.ll:65646~65648 · m12.ll:18909~18911) | 4 | OK |
| 28 | CCState | 0x8 | tick(비 Animation) | r | CCState::tick 인라인(entity.rs:546): 태그 10(Animation)이면 +0x20, 아니면 +0x8 (m12.ll:18921~18924 · m01.ll:51701~51705) | 4 | OK |
| 29 | Option<SmallAction>(action) | 0x0 | tag | r | 클로저 51602~51603: ==0 → Some(RunAway) | 4 | 확인불가(tcx 사전에 타입 없음) |

**`consts` 상수 13건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 2 | 607 | 임계 | 팀 인덱스 bounds(2). 클로저 51200 도 동일. 또 skill2_effect 게이트 level > 2 (51490, entity.rs:1693) | 4 |
| 1 | 5 | 607 | 임계 | 포지션 인덱스 bounds(5) | 4 |
| 2 | 100 | 633 | 계수 | hp = champ.hp*100 (백분 스케일) · add_max = dps*100*dt/60 · Entity::radius 의 (mult+100)/100 | 4 |
| 3 | 60 | 641 | 계수 | add_max = dps*100*dt/60 — dt(틱)를 초로 환산하는 하드코딩 tps=60 (setting.tick_per_second 를 안 씀 — dps 계산에서는 tps 를 썼음) | 4 |
| 4 | 9999999999 | 630 | 산출값 | 센티넬 (9999999999, 0) 을 dps_list 끝에 push(52515~52517) — 정렬 후 마지막; 죽지 않으면 이 값이 반환. 또 빈 목록(tick==0)일 때 반환값(52616) | 4 |
| 5 | 0 | 614 | 태그 | 타워 dps 는 도달 틱 0 으로 push(52256). 루프 초기 tick/dps/total 0 · attack_range unwrap_or(0)(51387)·skill 0(51485)·skill2 0(51585)·cc_tick unwrap_or(0)(51778) | 4 |
| 6 | 1 | 621 | 인덱스 | umax(attack_cooltime,1)(52374) · umax(move_speed,1)(52455·51779) · umax(dps,1)(52648) — 0 나눗셈 가드. 타워 경로(52210)는 가드 없이 cooltime==0 → panic_const_div_by_zero | 4 |
| 7 | 21 | 632 | 임계 | sort_by_key 의 삽입정렬/드리프트정렬 분기(len<21 → insertion_sort_shift_left, 표준 라이브러리 내부) — 판정 아님 | 4 |
| 8 | -6 | 598 | 미상 | CCState::block_input 인라인: tag-6 (aux m09.ll:65647 · m12.ll:18910) | 4 |
| 9 | -4 | 598 | 미상 | CCState::block_input: (tag-6) as u32 < (u32)-4 → tag ∉ {2,3,4,5} = Bind/BlockAttack/BlockSkill/BlockMoveSkill 는 입력차단 아님, 그 외(Airborne/Stun/ForceMove/Taunt/Fear/Charm/Animation) 는 차단 | 4 |
| 10 | 10 | 598 | 태그 | CCState 태그 10 = Animation → tick 필드가 +0x20(32), 그 외 +0x8 (aux m12.ll:18921~18923 · m01.ll:51702~51703) | 4 |
| 11 | 3 | 578 | 길이 | flat_map 클로저가 원소당 [(도달틱,dps);3] 배열(공격/스킬/스킬2)을 돌려줌 — 배열 길이 3 (aux m01.ll:50969 size_hint ×3 · 51092·51096) | 4 |
| 12 | -1 | 585 | 센티널 | Option<Effect> 니치 태그 i32 -1 = None (attack/skill/skill2 전부) · level-1 (growth_range 배수) | 4 |

**`knobs` 조정점 5건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 죽음 지평 센티넬 | goal_data.rs:630 (m09.ll:52515) | 9999999999 | 반환 상한. 호출자가 '지평 안에 안 죽음' 판정에 쓰는 값(변경 시 호출자 비교 기준도 같이) | 4 | 기존 |
| 1 | 누적 피해 틱→초 환산 계수 | goal_data.rs:641 (m09.ll:52599) | 60 | 내리면 같은 dt 에 더 많은 피해가 누적돼 죽는 틱이 앞당겨짐(tps≠60 이면 dps 계산과 단위 불일치) | 4 | 기존 |
| 2 | hp 스케일 | goal_data.rs:633 (m09.ll:52562) | 100 | add_max 의 *100 과 짝. 마지막 remain_hp/dps 는 이 스케일이 그대로 남아 잔여 틱이 (hp*100)/dps 단위 — 계수 변경 시 잔여 계산도 같이 바뀜 | 4 | 기존 |
| 3 | 스킬2 사거리 포함 레벨 게이트 | entity.rs:1693 (aux m01.ll:51490) | 2 | level>2 에서만 skill2 도달틱 산정 — game_core 소관 | 4 | 기존 |
| 4 | 입력차단 CC 판정 집합 | entity.rs:521 block_input (aux m09.ll:65647~65648) | -4 | Bind/BlockAttack/BlockSkill/BlockMoveSkill 제외. game_core 소관 — 게임 AI 쪽 노브 아님 | 4 | 기존 |

<details><summary>`callees` 피호출자 16건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | attack_cooltime | game_core::Entity::attack_cooltime | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1766 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | block_input | game_core::Entity::block_input | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1501 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
| 2 | block_input | game_core::CCState::block_input | pub | fn(&game_core::CCState) -> bool | game-core\src\simulation\entity.rs:520 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
| 3 | distance | game_core::Entity::distance | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2161 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | expected_damage_target | game_core::Effect::expected_damage_target | pub | fn(&game_core::Effect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + , &game_core::Entity) -> usize | game-core\src\simulation\effect.rs:91 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 5 | get_die_tick_player | game_ai::get_die_tick_player | pub | fn(&game_core::OperationData, usize, usize, &bumpalo::collections::vec::Vec< usize>, std::option::Option<&game_core::Entity>, std::option::Option<game_core::SmallAction>) -> u64 | game-ai\src\goal_data.rs:577 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 6 | push | <game_core::Staff as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::staff | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\staff.rs:38 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 6개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 7 | push | <game_core::Athlete as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:434 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 6개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 8 | push | <game_core::Contract as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:2498 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 6개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 9 | radius | game_core::Entity::radius | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1509 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 10 | range | game_core::Effect::range | pub | fn(&game_core::Effect, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:25 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 11 | range_adjust | game_core::Effect::range_adjust | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:29 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 12 | skill2_effect | game_core::Entity::skill2_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1692 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 13 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 7개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 14 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 7개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 15 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 7개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
</details>

⚠**미매칭 15개**: `_RINvXs0_NtNtNtCsjihNppCmMEE_4core4iter8adapters3mapINtB6_3MapINtNtB8_6filter6FilterINtNtNtBc_5slice4iter4IterNtNtNtCs97f5S1uJLkH_9game_core10simulation6entity7CCStateENCNCNvNtCshdEBA0ozCnw_7game_ai9goal_data19get_die_tick_player0s1_0ENCB2H_s2_0ENtNtNtBa_6traits8iterator8Iterator4foldyNCINvNvB3V_6max_by4foldyNvYyNtNtBc_3cmp3Ord3cmpE0EB2N_`, `_RNvXs1_NtNtNtCsjihNppCmMEE_4core3ops8function5implsQNCNCNvNtCshdEBA0ozCnw_7game_ai9goal_data19get_die_tick_player0s1_0INtB7_5FnMutTRRNtNtNtCs97f5S1uJLkH_9game_core10simulation6entity7CCStateEE8call_mutBW_`, `ceil_div`, `champ_team`, `driftsort_main`, `drop  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `flat_map`, `insertion_sort_shift_left`, `llvm.umax.i64`, `llvm.usub.sat.i64`, `panic`, `range_of`, `reserve_internal_or_panic`, `sat_sub`, `sort_by_key`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 4곳** (m08.ll:97515, m08.ll:97528, m10.ll:12961, m10.ll:12973) · **형제 0개** 

**`open` 8건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | Option<SmallAction> 의 None 태그 — tcxdict 에 Option 인스턴스 없음. SmallAction 이 Direct 8B 태그 0..10 이라 니치 규칙상 None=11 이고, 클로저의 `tag==0` 비교는 Some(RunAway) 로 읽었다(추정 · 검증법: 호출자 m08.ll:97515/m10.ll:12961 의 action 인자 구성 추적) | 3 |  |
| 1 | 표기 불가 | L602~604 의 ceil 나눗셈이 소스에서 `div_ceil` 인지 `(a+b-1)/b` 인지 — 외연 동일(표기 불가). 본체 L625 는 floor 나눗셈(52456)으로 다름 | 4 |  |
| 2 | 미탐색 | L637/653 의 반환 표기 — `if tick==0 {9999999999} else {tick}` 로 접힘(52615~52616). 센티넬이 항상 push 되므로 정상 경로에선 tick==0 이 될 수 없음(빈 목록 불가) — 죽은 분기 가능성 | 4 |  |
| 3 | 미탐색 | L644 remain_hp/dps 단위: remain_hp 는 hp*100 스케일, dps 는 초당(틱 환산 없음) — 의도된 근사인지 버그인지 소스 주석 없음(rmetadocs 0건). IR 그대로 기록 | 4 |  |
| 4 | 미탐색 | 타워 경로(L611) attack_cooltime()==0 이면 panic_const_div_by_zero(52266) — others 경로는 max(..,1) 가드. 비대칭은 IR 사실 | 4 |  |
| 5 | 미탐색 | expected_damage_target(&Effect,&GameContext,&dyn AbstractEntity,&Entity)->usize / range_adjust(&Effect,&Entity,&Entity)->u64 / attack_cooltime(&Entity)->usize / distance(&Entity,&Entity)->u64 는 game_core 경계 — 시그니처만(tcx) | 3 |  |
| 6 | 미탐색 | calls 마지막 두 항목은 aux 조각 자체의 심볼(call_mut 심·max_by fold) — qcspec 매칭용 망글 표기 | 4 |  |
| 7 | 미탐색 | 정렬 알고리즘 조각(m03/m04/m05/m10/m12 의 quicksort/drift/merge/median3 등 9개 인스턴스)은 표준 라이브러리 sort_by_key 본체 — 키가 `.0`(도달틱) 임만 insertion_sort_shift_left(m03.ll:118684~118690, 원소+0 비교)로 확인 | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

