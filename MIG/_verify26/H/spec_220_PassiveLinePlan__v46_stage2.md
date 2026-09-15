---

### `220` PassiveLinePlan::v46_stage2 — v46 라인 CS 사망예측 2단계(생존판정) — 가장 빨리 나를 죽이는 커밋터 e* 가 우리측(도착 가능 아군+타워) 화력으로 나보다 먼저(≤) 죽으면 true

| 항목 | 값 |
|---|---|
| id | `passive_line__PassiveLinePlan__v46_stage2` |
| 심볼 | `_RNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old12passive_lineNtB2_15PassiveLinePlan10v46_stage2` |
| 소스 | `game-ai\src\plan_legacy\old\passive_line.rs:783` |
| IR | `m04.ll` 18037~18818행 |
| 경로·가시성 | `game_ai::plan_legacy::old::PassiveLinePlan::v46_stage2` · **in:game_ai::plan_legacy::old::passive_line** |
| 계층 | 레거시 플랜 |
| exe | `d27d50` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r17` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_ai::plan_legacy::old::PassiveLinePlan, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, &game_core::Entity, &bumpalo::collections::vec::Vec< (usize, usize, usize)>) -> bool
```

**TLS 메모 절(`sig.tls` · 정정 경로 `/specs[220]/sig/tls/<키>`)**

없음 — 본문·aux 에 LocalKey/call_once/threadlocal 참조 0건(grep 실측)

<details><summary>인자 7개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | team | usize |  | 4 |
| 1 | 2 | cache | &AbstractGameWithCache(8840B) |  | 4 |
| 2 | 3 | ctx | &GameContext(64B) |  | 4 |
| 3 | 4 | champ | &Entity(1728B) |  | 4 |
| 4 | 5 | front | &Entity(1728B) |  | 4 |
| 5 | 6 | committers.data_ptr | &[(usize,usize,usize)] 의 ptr (원소 24B: +0 e.id, +8 my_die, +16 kill_dps) |  | 4 |
| 6 | 6 | committers.len | usize |  | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn v46_stage2(team, cache, ctx, champ, front, committers: &[(usize e_id, usize my_die, usize kill_dps)]) -> bool

tps = ctx.setting.tick_per_second                                     // L785
let Some(&(best_id, my_die, _)) = committers.iter().min_by_key(|c| c.1) else { return true }   // L786 (len==0 → true; 동률 시 앞 원소)
let Some(e_star) = cache.game.get_entity_by_id(best_id) else { return true }                  // L789 vtable+0x1f0
let Some(ep) = cache.player_by_champion_id(e_star.id) else { return true }                     // L792
e_pos = ep.info.position.as_index()                                   // L795
my_range = champ.attack_effect.map(|f| f.range + f.growth_range*(level-1) + stat_buff_cached.range).unwrap_or(0)   // L797 closure$1
pull = my_range + champ.radius() + front.radius()                     // L798
our_dps = 0; our_nuke = 0

// 우리측 챔피언 (L802~824): 내 팀 5슬롯(나 포함), 가시성·is_ignored_well 검사 없음
for a in cache.player_champion[team].iter().flatten() {
  if a.is_in_return() { continue }                                    // L803 (Champion && action_state==Return)
  if a.hp*100/max(a.stat_cached.hp,1) < 40 { continue }               // L806
  a_range = a.attack_effect.map(range 공식).unwrap_or(0)               // L809 closure$2
  reach = a_range + pull + a.radius() + e_star.radius()               // L809~810
  arrival = a.distance(front).saturating_sub(reach) / max(a.stat_cached.move_speed,1)   // L812~813 (틱)
  if arrival > my_die { continue }                                    // L814 — 내가 죽기 전에 못 닿는 아군 제외
  ap = cache.player_by_champion_id(a.id).unwrap()                     // L817 (None 패닉)
  c = &cache.player_champion_cache[ap.info.team][ap.info.position]    // L818 (bounds<2 패닉 가드)
  nuke = if a.can_attack() || a.attack_cooldown() <= tps { c.attack[e_pos] } else { 0 }               // L820 (ty 별 필드 switch entity.rs:1748; None/Nexus 는 쿨 없음)
  if a.can_skill()  || !(a is Champion) || a.skill_cooldown  <= tps { nuke = max(nuke, c.skill[e_pos]) }   // L821
  if a.can_skill2() || !(a is Champion) || a.skill2_cooldown <= tps { nuke = max(nuke, c.skill2[e_pos]) }  // L822
  our_dps  += c.attack_per_sec[e_pos] + c.skill_per_sec[e_pos] + c.skill2_per_sec[e_pos]   // L823
  our_nuke += nuke                                                     // L824
}

// 내 타워 (L826~841)
tower_disable_tick = match ctx.tutorial { First|Bottom => …_2v2, MidBottom => …_3v3, _ => tower_attack_disable_tick }   // L826~829
for t in cache.iter_towers_without_nexus(team) {                      // L831 (인라인 순회)
  if cache.game.tick() > tower_disable_tick { continue }              // L832
  if t.distance(front).saturating_sub(pull) > 150000 { continue }     // L835
  if let Some(atk) = t.attack_effect.as_ref() {                       // L838
    dmg = atk.expected_damage_target(ctx, t as &dyn AbstractEntity, e_star)   // L839
    our_dps  += dmg*tps / max(t.attack_cooltime(),1)                  // L840
    our_nuke += dmg                                                    // L841
  }
}
e_die = e_star.hp.saturating_sub(our_nuke) * 60 / max(our_dps,1)      // L844
return e_die <= my_die                                                // L845~846 (icmp ule %180,%31)

극성 근거: %18(len==0)/%27/%39/%44 → %182 phi true. L803 %208 eq 1 → %209(continue). L806 %218 ult 40 → %209. L814 %274 ugt → %209. L832 %173 ugt → %204(continue). L835 %187 ugt 150000 → %204. 반환 %181 = icmp ule e_die, my_die.
```

**`mem` 메모리 접근 44건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | GameContext | 0x8 | setting | r | &GameSetting | 4 | OK |
| 1 | GameContext | 0x38 | tutorial | r | TutorialType 태그 switch(L826~829) | 4 | OK |
| 2 | GameSetting | 0x12f8 | tick_per_second | r | tps — 쿨다운>tps 비교(L820~822)·타워 DPS 환산(L840) | 4 | OK |
| 3 | GameSetting | 0x13f8 | tower_attack_disable_tick | r | tutorial 0/2/4/6/7/8 → L829 | 4 | OK |
| 4 | GameSetting | 0x1400 | tower_attack_disable_tick_2v2 | r | tutorial 1/3 (phi 기본 5120) | 4 | OK |
| 5 | GameSetting | 0x1408 | tower_attack_disable_tick_3v3 | r | tutorial 5 → L828 | 4 | OK |
| 6 | AbstractGameWithCache | 0x0 | game.data_ptr | r | &dyn AbstractGame | 4 | OK |
| 7 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r | vtable+0x1f0 get_entity_by_id(L789) · vtable+0x28 tick(L832) — divtable AbstractGame 실측 | 3 | OK |
| 8 | AbstractGameWithCache | 0x1e0 | player_champion[2][5] | r | [team] 내 팀 5슬롯 순회(L802, 나 자신 포함) | 4 | OK |
| 9 | AbstractGameWithCache | 0x280 | player_champion_cache[2][5] | r | ChampionCache 800B [ap.info.team][ap.info.position] (L818) | 4 | OK |
| 10 | ChampionCache | 0x0 | attack[e_pos] | r |  | 4 | OK |
| 11 | ChampionCache | 0x28 | skill[e_pos] | r |  | 4 | OK |
| 12 | ChampionCache | 0x50 | skill2[e_pos] | r |  | 4 | OK |
| 13 | ChampionCache | 0x190 | attack_per_sec[e_pos] | r |  | 4 | OK |
| 14 | ChampionCache | 0x1b8 | skill_per_sec[e_pos] | r |  | 4 | OK |
| 15 | ChampionCache | 0x1e0 | skill2_per_sec[e_pos] | r |  | 4 | OK |
| 16 | PlayerState | 0x930 | info.team | r | ChampionCache 1차 인덱스(bounds<2 패닉 가드 L818) | 4 | OK |
| 17 | PlayerState | 0x9c0 | info.position@tag | r | as_index → e_pos(L795) / 아군 캐시 열(L818) | 4 | OK |
| 18 | Entity | 0x5c0 | id | r | e_star.id(L792) · a.id(L817) | 4 | OK |
| 19 | Entity | 0x4c0 | attack_effect@tag | r | -1 None. champ(L797)·a(L809)·t(L838) | 4 | OK |
| 20 | Entity | 0x490 | attack_effect@Some.0 | r | &Effect — 타워 t 의 expected_damage_target 수신자(L839) | 4 | OK |
| 21 | Entity | 0x4a0 | attack_effect@Some.0.range | r | Effect::range 공식(effect.rs:26) | 4 | OK |
| 22 | Entity | 0x4a8 | attack_effect@Some.0.growth_range | r |  | 4 | OK |
| 23 | Entity | 0x5c8 | level | r |  | 4 | OK |
| 24 | Entity | 0x438 | stat_buff_cached.range | r |  | 4 | OK |
| 25 | Entity | 0x470 | stat_buff_cached.radius_mult | r | Entity::radius(entity.rs:1511~1515). champ·front·a·e_star | 4 | OK |
| 26 | Entity | 0x680 | radius | r |  | 4 | OK |
| 27 | Entity | 0x640 | stat_cached.move_speed | r | a.ms(L813) arrival 분모 max(…,1) | 4 | OK |
| 28 | Entity | 0x670 | hp | r | a.hp(L806) · e_star.hp(L844) | 4 | OK |
| 29 | Entity | 0x628 | stat_cached.hp | r | a 최대 HP(L806 분모) | 4 | OK |
| 30 | Entity | 0x68 | ty@tag | r | EntityType — ==13 Champion(L803 is_in_return · L821~822 스킬쿨 검사 대상) · attack_cooldown switch(L820) | 4 | OK |
| 31 | Entity | 0x70 | ty@Champion.0.action_state@tag | r | ==1 Return → is_in_return(entity.rs:1649) → 아군 제외(L803) | 4 | OK |
| 32 | Entity | 0xb0 | ty@Champion.0.attack_cooldown / SmallJiangshi | r | switch case 13·8 | 4 | OK |
| 33 | Entity | 0xb8 | ty@Champion.0.skill_cooldown (L821) / Minion.attack_cooldown (case 1) | r |  | 4 | OK |
| 34 | Entity | 0xc0 | ty@Champion.0.skill2_cooldown | r | L822 | 4 | OK |
| 35 | Entity | 0xc8 | ty@Bear.info.attack_cooldown | r | case 9 | 4 | OK |
| 36 | Entity | 0xd0 | ty@Illusion.info.attack_cooldown | r | case 12 | 4 | OK |
| 37 | Entity | 0xd8 | ty@Revenant.info.attack_cooldown | r | case 11 | 4 | OK |
| 38 | Entity | 0xe8 | ty@Jungle/Ghoul.info.attack_cooldown | r | case 4·7 | 4 | OK |
| 39 | Entity | 0xf0 | ty@Eagle.info.attack_cooldown | r | case 10 | 4 | OK |
| 40 | Entity | 0x110 | ty@Tower.info.attack_cooldown | r | case 2 | 4 | OK |
| 41 | Entity | 0x1f0 | ty@Epic/Serpen.info.attack_cooldown | r | case 5·6 | 4 | OK |
| 42 | committers 원소(24B) | 0x0 / 0x8 | e.id / my_die | r | min_by_key 키 = +8(my_die). +16 kill_dps 미사용 | 4 | 확인불가(tcx 사전에 타입 없음) |
| 43 | iter_towers_without_nexus sret(120B, 지역 %9) | 0x0 / 0x8 / 0x10 / 0x18 / 0x68 | front 배열 IntoIter 상태(태그 -1=소진 · 인덱스 +8/+16 · [6 x Option<&Entity>] +24) / 뒤 Copied<slice::Iter> (+104) | r | Chain<Flatten<array::IntoIter<Option<&Entity>,6>>, Copied<slice::Iter<&Entity>>> 을 인라인 순회(L831) | 4 | 확인불가(tcx 사전에 타입 없음) |

**`consts` 상수 12건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 24 | 786 | 미상 | committers 원소 stride 24B(len*24 = end). 판정값 아님 | 4 |
| 1 | -1 | 797 | 센티널 | Option<Effect> None 니치 태그(i32, Entity+0x4c0). L797·L809·L838. 또한 타워 이터레이터 front 배열 소진 태그(i64 -1, L831) | 4 |
| 2 | 100 | 798 | 계수 | radius*(mult+100)/100 (entity.rs:1515) · L806 hp*100/max_hp | 4 |
| 3 | 13 | 803 | 태그 | EntityType 태그 13 = Champion. L803(is_in_return 전제)·L821/L822(스킬쿨 검사는 챔피언만) | 4 |
| 4 | 1 | 803 | 태그 | Champion.action_state 태그 1 = Return (tcxdict --enum: 0 Idle/1 Return/2 Move/3 Attack/4 Skill/5 Skill2/6 Ult) → is_in_return 이면 아군 제외 | 3 |
| 5 | 40 | 806 | 임계 | 아군 hp% < 40 이면 우리측 화력에서 제외 | 4 |
| 6 | 5 | 826 | 태그 | TutorialType 태그 5 MidBottom → 3v3 disable tick(태그 1/3 → 2v2, 나머지 기본) | 4 |
| 7 | 5120 | 826 | 산출값 | phi 에 접힌 GameSetting 오프셋 0x1400(2v2) — 5112=0x13f8 기본 · 5128=0x1408 3v3. 오프셋이지 판정값 아님 | 4 |
| 8 | 150000 | 835 | 임계 | t.distance(front).saturating_sub(pull) > 150000 이면 타워 제외(stage1 closure$1 의 <150001 과 동치) | 4 |
| 9 | 60 | 844 | 계수 | 초→틱 환산 하드코딩: e_die = (e*.hp - our_nuke)*60/max(our_dps,1). stage1 의 my_die 와 같은 단위 | 4 |
| 10 | 2 | 802 | 길이 | 팀 배열 길이 bounds check 인자. 판정값 아님 | 4 |
| 11 | 6 | 831 | 길이 | 타워 front 배열 길이 6(Option<&Entity> ×6: top/top2/mid/mid2/bottom/bottom2 — assume ult 6). 판정값 아님 | 4 |

**`knobs` 조정점 6건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 아군 hp% 컷 | passive_line.rs:806 | 40 | 내리면 저체력 아군도 화력에 포함 → e_die 감소 → true(생존) 판정 증가 | 4 | 기존 |
| 1 | 아군 도착 조건 | passive_line.rs:814 | arrival <= my_die | my_die 는 stage1 산출. 마진을 더하면(예: arrival <= my_die+k) 멀리 있는 아군도 산입 → 생존 판정 관대 | 4 | 기존 |
| 2 | 타워 산입 반경 | passive_line.rs:835 | 150000 | 내리면 타워 화력 제외 증가 → e_die 증가 → false(귀환) 증가 | 4 | 기존 |
| 3 | 초→틱 환산 | passive_line.rs:844 | 60 | stage1 L720/L752 와 같은 상수. 한쪽만 바꾸면 my_die 와 단위가 어긋남 | 4 | 기존 |
| 4 | 생존 판정 비교 | passive_line.rs:845 | e_die <= my_die | ule → ult 로 바꾸면 동률(같은 틱 사망)이 패배로 바뀜 | 4 | 기존 |
| 5 | 귀환 중 아군 제외 | passive_line.rs:803 | action_state==Return | 제거하면 귀환 중 아군도 화력 산입(도착 조건은 여전히 적용) | 4 | 기존 |

<details><summary>`callees` 피호출자 20건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | as_index | game_core::BigGoal::as_index | pub | fn(&game_core::BigGoal) -> usize | game-core\src\simulation\ai_interface.rs:338 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 1 | as_index | game_core::Position::as_index | pub | fn(&game_core::Position) -> usize | game-core\src\simulation\entity.rs:580 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 2 | attack_cooldown | game_core::Entity::attack_cooldown | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1747 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 3 | attack_cooltime | game_core::Entity::attack_cooltime | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1766 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | can_attack | game_core::Entity::can_attack | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1527 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 5 | can_skill | game_core::Entity::can_skill | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1708 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 6 | can_skill2 | game_core::Entity::can_skill2 | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1721 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 7 | distance | game_core::Entity::distance | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2161 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 8 | expected_damage_target | game_core::Effect::expected_damage_target | pub | fn(&game_core::Effect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + , &game_core::Entity) -> usize | game-core\src\simulation\effect.rs:91 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 9 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 10 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 11 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 12 | is_in_return | game_core::Entity::is_in_return | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1647 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 13 | iter_towers_without_nexus | game_core::AbstractGameWithCache::<'a, 'b>::iter_towers_without_nexus | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5750 ~ game_core[6a30]::simulation::{impl#5}::iter_towers_without_nexus::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1830 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 14 | player_by_champion_id | game_core::AbstractGameWithCache::<'a, 'b>::player_by_champion_id | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> std::option::Option<&'a/#0 game_core::PlayerState> | game-core\src\simulation.rs:1915 | True | True | 3 | IR 호출 심볼 일치(토큰 경계) |
| 15 | radius | game_core::Entity::radius | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1509 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 16 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 17 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 18 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 19 | v46_stage2 | game_ai::plan_legacy::old::PassiveLinePlan::v46_stage2 | in:game_ai::plan_legacy::old::passive_line | fn(&game_ai::plan_legacy::old::PassiveLinePlan, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, &game_core::Entity, &bumpalo::collections::vec::Vec< (usize, usize, usize)>) -> bool | game-ai\src\plan_legacy\old\passive_line.rs:783 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
</details>

⚠**미매칭 4개**: `llvm.memcpy.p0.p0.i64`, `llvm.umax.i64`, `llvm.usub.sat.i64`, `next  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 9개는 **전부 다른 함수**라 싣지 않는다`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 2곳** (m04.ll:21682, m04.ll:21903) · **형제 18개** (PassiveLinePlan)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::plan_legacy::old::PassiveLinePlan as std::clone::Clone>::clone | pub | game-ai\src\plan_legacy\old\passive_line.rs:176 | True | fn(&game_ai::plan_legacy::old::PassiveLinePlan) -> game_ai::plan_legacy::old::PassiveLinePlan |
| 1 | <game_ai::plan_legacy::old::PassiveLinePlan as std::fmt::Debug>::fmt | pub | game-ai\src\plan_legacy\old\passive_line.rs:176 | True | fn(&game_ai::plan_legacy::old::PassiveLinePlan, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 2 | game_ai::plan_legacy::old::PassiveLinePlan::new | pub | game-ai\src\plan_legacy\old\passive_line.rs:197 | True | fn(game_core::LineType) -> game_ai::plan_legacy::old::PassiveLinePlan |
| 3 | game_ai::plan_legacy::old::PassiveLinePlan::v46_fleeing | pub | game-ai\src\plan_legacy\old\passive_line.rs:207 | True | fn(&game_ai::plan_legacy::old::PassiveLinePlan) -> bool |
| 4 | game_ai::plan_legacy::old::PassiveLinePlan::goal | pub | game-ai\src\plan_legacy\old\passive_line.rs:211 | True | fn(&game_ai::plan_legacy::old::PassiveLinePlan) -> game_core::BigGoal |
| 5 | game_ai::plan_legacy::old::PassiveLinePlan::update | pub | game-ai\src\plan_legacy\old\passive_line.rs:219 | False | fn(&mut game_ai::plan_legacy::old::PassiveLinePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PositioningScoreData, &mut game_core::DebugFrameData) |
| 6 | game_ai::plan_legacy::old::PassiveLinePlan::v46_carry_over | pub | game-ai\src\plan_legacy\old\passive_line.rs:307 | False | fn(&mut game_ai::plan_legacy::old::PassiveLinePlan, &mut game_ai::plan_legacy::old::PassiveLinePlan) |
| 7 | game_ai::plan_legacy::old::PassiveLinePlan::v46_flee_end | in:game_ai::plan_legacy::old::passive_line | game-ai\src\plan_legacy\old\passive_line.rs:322 | False | fn(&mut game_ai::plan_legacy::old::PassiveLinePlan) |
| 8 | game_ai::plan_legacy::old::PassiveLinePlan::update_v46_flee | in:game_ai::plan_legacy::old::passive_line | game-ai\src\plan_legacy\old\passive_line.rs:343 | False | fn(&mut game_ai::plan_legacy::old::PassiveLinePlan, usize, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 9 | game_ai::plan_legacy::old::PassiveLinePlan::v46_clear | in:game_ai::plan_legacy::old::passive_line | game-ai\src\plan_legacy\old\passive_line.rs:476 | False | fn(&mut game_ai::plan_legacy::old::PassiveLinePlan, bool) |
| 10 | game_ai::plan_legacy::old::PassiveLinePlan::update_v46_lane_recall | in:game_ai::plan_legacy::old::passive_line | game-ai\src\plan_legacy\old\passive_line.rs:488 | False | fn(&mut game_ai::plan_legacy::old::PassiveLinePlan, usize, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 11 | game_ai::plan_legacy::old::PassiveLinePlan::v46_stage1 | in:game_ai::plan_legacy::old::passive_line | game-ai\src\plan_legacy\old\passive_line.rs:663 | False | fn(&game_ai::plan_legacy::old::PassiveLinePlan, usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, &game_core::Entity, &game_core::Entity, usize, bool, std::option::Option<&[usize]>, &bumpalo::collections::vec::Vec< &game_core::Entity>) -> bumpalo::collections::vec::Vec< (usize, usize, usize)> |
| 12 | game_ai::plan_legacy::old::PassiveLinePlan::v46_stage2 | in:game_ai::plan_legacy::old::passive_line | game-ai\src\plan_legacy\old\passive_line.rs:783 | False | fn(&game_ai::plan_legacy::old::PassiveLinePlan, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, &game_core::Entity, &bumpalo::collections::vec::Vec< (usize, usize, usize)>) -> bool |
| 13 | game_ai::plan_legacy::old::PassiveLinePlan::sub_plan | pub | game-ai\src\plan_legacy\old\passive_line.rs:848 | False | fn(&game_ai::plan_legacy::old::PassiveLinePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> game_ai::plan_legacy::sub_plan::SubPlan |
| 14 | game_ai::plan_legacy::old::PassiveLinePlan::check_bot_lane_2v1 | in:game_ai::plan_legacy::old::passive_line | game-ai\src\plan_legacy\old\passive_line.rs:1018 | False | fn(&game_ai::plan_legacy::old::PassiveLinePlan, &game_core::PlayerState, &game_core::OperationData) -> std::option::Option<game_ai::plan_legacy::sub_plan::SubPlan> |
| 15 | game_ai::plan_legacy::old::PassiveLinePlan::has_lead | pub | game-ai\src\plan_legacy\old\passive_line.rs:1048 | False | fn(&game_ai::plan_legacy::old::PassiveLinePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> bool |
| 16 | game_ai::plan_legacy::old::PassiveLinePlan::check_recall | in:game_ai::plan_legacy::old::passive_line | game-ai\src\plan_legacy\old\passive_line.rs:1068 | False | fn(&game_ai::plan_legacy::old::PassiveLinePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> bool |
| 17 | game_ai::plan_legacy::old::PassiveLinePlan::next_plan | pub | game-ai\src\plan_legacy\old\passive_line.rs:1451 | True | fn(&mut game_ai::plan_legacy::old::PassiveLinePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &game_core::PositioningScoreData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> std::option::Option<game_ai::plan_legacy::types::BigPlan> |

**`open` 5건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 표기 불가 | committers 가 비었을 때 true 를 반환하는 것이 소스에서 `let Some(..) = .. else { return true }` 인지 `unwrap_or(true)` 류인지 — 표기 불가(동작 확정) | 4 |  |
| 1 | 재료 부재 | L821/L822 한 줄 안의 `can_skill() \|\| !is_champion \|\| cooldown<=tps` 세 항의 소스 표기 순서 — IR 은 `or i1 %126, %309` 로 접혀 있어 순서 복원 불가(외연 동일) | 4 |  |
| 2 | 미탐색 | get_entity_by_id(vtable+0x1f0)·tick(vtable+0x28) 의 슬롯 이름은 divtable 정적 vtable 대조 결과(런타임 vtable 미검증) | 3 |  |
| 3 | 미탐색 | expected_damage_target·attack_cooltime·iter_towers_without_nexus·Iterator::next 의 내부 미열람(계약만 — 지시 범위) | 4 |  |
| 4 | 미탐색 | 타워 이터레이터의 6슬롯 순서(top/top2/mid/mid2/bottom/bottom2)는 AbstractGameWithCache 필드 순서로 추정 — iter_towers_without_nexus 본문(_gcbc g15.ll:108971) 미열람이라 순서는 추정 | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

