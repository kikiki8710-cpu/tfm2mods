---

### `92` v46_flee_gate_check — v46 라인 도주 게이트 — 피난처(가장 가까운 타워/넥서스) 사거리 밖에 있을 때 근처 적 중 '나를 잡을 수 있는' 위협(committers)을 뽑고, 없으면 차단 사유 코드를 돌려준다

| 항목 | 값 |
|---|---|
| id | `passive_line__v46_flee_gate_check` |
| 심볼 | `_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old12passive_line19v46_flee_gate_check` |
| 소스 | `game-ai\src\plan_legacy\old\passive_line.rs:38` |
| IR | `m04.ll` 55362~57000행 |
| 경로·가시성 | `game_ai::plan_legacy::old::passive_line::v46_flee_gate_check` · **in:game_ai** |
| 계층 | 레거시 플랜 |
| exe | `d3b2a0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r10` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity) -> (u8, bumpalo::collections::vec::Vec< usize>)
```

<details><summary>인자 5개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | (u8, bumpalo Vec<usize>)(40B) | %0. +0 u8 사유코드, +8 Vec<usize>(ptr/bump/cap/len 32B) committers(위협 적 챔피언 id) | 4 |
| 1 | 1 | version | usize | %1. 스택 %21 에 저장 — is_ignored_well_enemy(version,…) 인자로만 전달, 본문 분기 없음 | 4 |
| 2 | 2 | player | &PlayerState(2528B) | %2 | 4 |
| 3 | 3 | data | &OperationData(24B) | %3 | 4 |
| 4 | 4 | champ | &Entity(1728B) | %4. 내 챔피언 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
L40: pool = context.pool; L41: tps = setting.tick_per_second; L42: tick = cache.game.tick()[vtable 0x28]; L43: committers = Vec::new_in(pool)
L46: team = player.info.team; refuge = cache.iter_towers_without_nexus(team).min_by_key(closure$0: |t| t.distance_sq(champ))   (fold 은 별도 define — 키 = 타워↔나 distance_sq, 가장 가까운 타워)
L47: refuge = refuge.or(cache.nexus[team])   (team<2 bounds)
L48: refuge None → L49 return (5, [])
L51: under_refuge = refuge.attack_effect.as_ref().map_or(false, closure$1: |atk| champ.distance_sq(refuge) <= (atk.range(refuge) + champ.radius())^2)
     atk.range(e) 인라인(effect.rs:26) = atk.range + e.stat_buff_cached.range + (e.level-1)*atk.growth_range ; radius() 인라인(entity.rs:1511) = mult==0 ? radius : radius*(mult+100)/100
L55: if under_refuge → L56 return (5, [])   (IR 55766: dist_sq > r² 이면 %172 로 계속, 아니면 %181 코드 5 — 권역 안이면 도주 게이트 차단; attack_effect None 이면 map_or(false) → 계속)
L59~62: near_enemies = cache.iter_champions(1-team).filter(closure$2 = aux 66064: |e| e.distance_sq(champ) < 22500000001 && champ.is_visible_from(e)[champ.team Neutral→true; Player(t)→ e.visible_state[t]==Visible] && !is_ignored_well_enemy(version, player, e)).collect(pool)
L64: near_enemies.is_empty() → L65 return (1, [])
L69: my_pos = player.info.position; L70: reaction = champ.attack_duration() + 6; L71: my_ms = champ.stat_cached.move_speed; L72: escape_ticks = champ.distance(refuge) / max(my_ms,1)
L73~76: tower_disable_tick = match context.player_count()[tutorial→인원] { 2 => setting.tower_attack_disable_tick_2v2, 3 => _3v3, _ => tower_attack_disable_tick }   (switch 55889: First/Bottom→2v2, MidBottom→3v3, 그 외→기본)
L79~81: my_towers = iter_towers_without_nexus(team).filter(closure$3 = aux 66153: |t| !(tick > tower_disable_tick) && t.distance_sq(champ) < 22500000001).collect(pool)   ★타워는 tick ≤ tower_attack_disable_tick 일 때만 억지력에 포함
any_margin_pass = false; any_range_pass = false
L86: for e in near_enemies {
  L87: ep = cache.player_by_champion_id(e.id).unwrap(); L88: e_pos = ep.info.position; L89: aggr = ep.info.parameter.aggressive_ratio()
  L90: diff_bound = (1000-aggr)*80/1000 + 80;  L91: die_tick_bound = aggr*45/1000 + 45
  L96: (kill_dps, kill_nuke) = (0,0); for a in iter_champions(1-team) {   // 적 팀 전원(e 포함)
    L97: if a.distance_sq(e) > 22500000000 || a.hp*100/max(a.max_hp,1) < 40 { continue }
    L100: ap = player_by_champion_id(a.id).unwrap(); L101: c = cache.player_champion_cache[ap.team][ap.position]
    L103: nuke = if a.can_attack() || a.attack_cooldown()[ty별 인라인] <= tps { max(0, c.attack[my_pos]) } else { 0 }
    L104: if a.can_skill() || !(a.ty==Champion && a.skill_cooldown > tps) { nuke = max(nuke, c.skill[my_pos]) }
    L105: if a.can_skill2() || !(a.ty==Champion && a.skill2_cooldown > tps) { nuke = max(nuke, c.skill2[my_pos]) }
    L106: kill_dps += c.attack_per_sec[my_pos] + c.skill_per_sec[my_pos] + c.skill2_per_sec[my_pos];  L107: kill_nuke += nuke }
  L109: my_die = champ.hp.saturating_sub(kill_nuke) * 60 / max(kill_dps,1)
  L114: (det_dps, det_nuke) = (0,0); for a in iter_champions(team) {   // 내 팀 전원(나 포함)
    L115: if a.distance_sq(e) > 22500000000 || !(e.team Neutral || a.visible_state[e.team]==Visible) { continue }   // 적이 볼 수 있는 아군만 억지력
    L116: if is_ignored_well_enemy(version, ep, a) { continue }
    L119~126: ap/c 동일; nuke 계산 동일(인덱스 e_pos); det_dps += c.*_per_sec[e_pos]; det_nuke += nuke }
  L128: for t in my_towers { L129: if let Some(atk) = t.attack_effect { L130: dmg = atk.expected_damage_target(ctx, t, e); L131: det_dps += dmg*tps/max(t.attack_cooltime(),1); L132: det_nuke += dmg } }
  L135: e_die = e.hp.saturating_sub(det_nuke) * 60 / max(det_dps,1)
  L144: if diff_bound + my_die > e_die { continue }        // 내가 마진만큼 더 오래 못 버팀 → 위협 아님(교전 여유)
  any_margin_pass = true
  L150: er = e.attack_effect.map(|a| a.range(e)).unwrap_or(0) + e.radius() + champ.radius()
  L152: if e.distance(champ) > er + e.move_speed * reaction { continue }   // 반응시간 안에 못 닿음
  L155: if e.move_speed <= my_ms && my_die >= die_tick_bound { continue }  // 못 쫓아오고 나는 충분히 버팀
  any_range_pass = true
  L160: if my_die > escape_ticks + reaction { continue }   // 피난처까지 도망칠 시간이 있음
  L163: committers.push(e.id) }
L165: if !committers.is_empty() → (0, committers)
L167: else if any_range_pass → (4, [])
L169: else if any_margin_pass → (3, []) else (2, [])
(IR phi 56486~56487: %461=any_range_pass(true at 56439 L155 통과·56483), %462=any_margin_pass(true at L144 통과 이후 전 경로))
```

**`mem` 메모리 접근 41건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | OperationData | 0x0 | cache | r | 55413 | 4 | OK |  |
| 1 | OperationData | 0x8 | context | r | 55402 | 4 | OK |  |
| 2 | GameContext | 0x0 | pool | r | &Bump — committers/near_enemies/my_towers Vec::new_in (55404) | 4 | OK |  |
| 3 | GameContext | 0x8 | setting | r | 55408 | 4 | OK |  |
| 4 | GameContext | 0x38 | tutorial | r | player_count() 인라인 switch (55887~55899) | 4 | OK |  |
| 5 | GameSetting | 0x12f8 | tick_per_second | r | tps (55409~55410) | 4 | OK |  |
| 6 | GameSetting | 0x13f8 | tower_attack_disable_tick | r | tutorial None/TopSolo/MidSolo/JungleOnly/Line/Total (55911 phi 5112) | 4 | OK |  |
| 7 | GameSetting | 0x1400 | tower_attack_disable_tick_2v2 | r | tutorial First/Bottom (phi 5120) | 4 | OK |  |
| 8 | GameSetting | 0x1408 | tower_attack_disable_tick_3v3 | r | tutorial MidBottom (phi 5128) | 4 | OK |  |
| 9 | AbstractGameWithCache | 0x0 | game.data_ptr | r | 55417 | 4 | OK |  |
| 10 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r | 슬롯 0x28 = tick (55420~55422) | 4 | OK |  |
| 11 | AbstractGameWithCache | 0x170 | nexus[team] | r | refuge 폴백 (55643~55645) | 4 | OK |  |
| 12 | AbstractGameWithCache | 0x1e0 | player_champion / [team][0..5] | r | iter_champions 인라인 (55776~55777, 55961) | 4 | OK |  |
| 13 | AbstractGameWithCache | 0x280 | player_champion_cache[team][pos] | r | 800B 셀: +0 attack[5] · +0x28 skill[5] · +0x50 skill2[5] · +0x190 attack_per_sec[5] · +0x1b8 skill_per_sec[5] · +0x1e0 skill2_per_sec[5] (56554~56555, 56630, 56660, 56685, 56695~56703; 56754~ 동일) | 4 | OK |  |
| 14 | PlayerState | 0x930 | info.team | r | team (55432); ep/ap 의 team (56536, 56736) | 4 | OK |  |
| 15 | PlayerState | 0x9c0 | info.position@tag | r | my_pos(55841) / e_pos(56040) / ap pos(56551, 56751) | 4 | OK |  |
| 16 | PlayerState | 0x180 | info.parameter | r | AthleteParameter(744B) → aggressive_ratio() (56044~56045) | 4 | OK |  |
| 17 | Entity | 0x0 | team@tag | r | TeamType 0=Player/1=Neutral — is_visible_from 인라인 (56492, aux 66115) | 4 | OK |  |
| 18 | Entity | 0x8 | team@Player.0 | r | 팀 번호 (56498, aux 66122) | 4 | OK |  |
| 19 | Entity | 0x38 | visible_state | r | stride 24, 0=Visible (56505~56508, aux 66129~66132) | 4 | OK |  |
| 20 | Entity | 0x68 | ty@tag | r | attack_cooldown()/skill_cooldown() 인라인 switch (56565~56582, 56640~56642 ==13 Champion) | 4 | OK |  |
| 21 | Entity | 0xb0 | ty@Champion.0.attack_cooldown | r | phi 176 (13 Champion / 8 SmallJiangshi). 다른 타입: 184 Minion, 272 Tower, 232 Jungle/Ghoul, 200 Bear, 240 Eagle, 216 Revenant, 208 Illusion, 496 Epic/Serpen | 4 | OK |  |
| 22 | Entity | 0xb8 | ty@Champion.0.skill_cooldown | r | 56647 (184) | 4 | OK |  |
| 23 | Entity | 0xc0 | ty@Champion.0.skill2_cooldown | r | 56678 (192) | 4 | OK |  |
| 24 | Entity | 0x438 | stat_buff_cached.range | r | Effect::range 인라인 (55694, 56380) | 4 | OK |  |
| 25 | Entity | 0x470 | stat_buff_cached.radius_mult | r | radius() 인라인 i32 (55698, 55966, 56390) | 4 | OK |  |
| 26 | Entity | 0x490 | attack_effect@Some.0 | r | &Effect: refuge(55679) / e(56372) / tower(56312) | 4 | OK |  |
| 27 | Entity | 0x4a0 | attack_effect@Some.0.range | r | 55680, 56373 | 4 | OK |  |
| 28 | Entity | 0x4a8 | attack_effect@Some.0.growth_range | r | 55682, 56375 | 4 | OK |  |
| 29 | Entity | 0x4c0 | attack_effect@tag | r | -1 = None (55665~55667, 56306~56308, 56361~56363) | 4 | OK |  |
| 30 | Entity | 0x5c0 | id | r | e.id(56006) a.id(56527, 56727) | 4 | OK |  |
| 31 | Entity | 0x5c8 | level | r | range = base + buff + (level-1)*growth (55692, 56376) | 4 | OK |  |
| 32 | Entity | 0x628 | stat_cached.hp | r | 적 아군 hp% (56717) | 4 | OK |  |
| 33 | Entity | 0x640 | stat_cached.move_speed | r | my_ms(55873) / e.move_speed(56424) | 4 | OK |  |
| 34 | Entity | 0x660 | x | r | distance_sq 다수 | 4 | OK |  |
| 35 | Entity | 0x668 | y | r |  | 4 | OK |  |
| 36 | Entity | 0x670 | hp | r | champ.hp(55960) / e.hp(56344) / a.hp(56714) | 4 | OK |  |
| 37 | Entity | 0x680 | radius | r | radius() 인라인 (55705, 55970, 56397) | 4 | OK |  |
| 38 | (u8,Vec)(sret) | 0x0 | code | w | 55722(5) 55805(5) 55820(1) 56922(0) 56932(4) 56937(2) 56942(3) | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) | 0\|1\|2\|3\|4\|5 |
| 39 | (u8,Vec)(sret) | 0x8 | committers | w | 모든 반환 경로에서 %19 를 복사 — 0 경로만 비어있지 않음 | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) | Vec<usize> 32B memcpy |
| 40 | stack Vec<usize> committers | 0x0 -> committers.ptr[len]+0 | e.id | w | 56479~56482 (L163) | 4 | 확인불가(tcx 사전에 타입 없음) | push |

**`consts` 상수 17건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 22500000001 | 61 | 미상 | 150000^2+1 — near_enemies: 적↔나 distance_sq < 이 값(≤150000, 4.7셀) (aux 66108); my_towers: 타워↔나 (aux 66207) | 4 |
| 1 | 22500000000 | 97 | 임계 | 150000^2 — 적의 아군(L97)·내 아군(L115) 이 대상 적 e 로부터 distance_sq > 이 값이면 제외(> 150000 초과) | 4 |
| 2 | 40 | 97 | 임계 | 적 아군 a 의 hp% < 40 이면 kill 전력에서 제외 | 4 |
| 3 | 100 | 97 | 계수 | hp*100/max_hp 백분율 · radius*(mult+100)/100 | 4 |
| 4 | 6 | 70 | 임계 | reaction = champ.attack_duration() + 6 틱 | 4 |
| 5 | 1000 | 90 | 계수 | aggressive_ratio 스케일(0..1000) | 4 |
| 6 | 80 | 90 | 오프셋가감 | diff_bound = (1000-aggr)*80/1000 + 80 → 80(공격적)..160(소극적) 틱 — 마진 | 4 |
| 7 | 45 | 91 | 계수 | die_tick_bound = aggr*45/1000 + 45 → 45..90 틱 — 추격불가 시 생존 하한 | 4 |
| 8 | 60 | 109 | 계수 | my_die = (hp − nuke)*60 / dps · e_die 도 동일(L135) — dps 가 초당 단위(*_per_sec)라 60 틱/초 환산(리터럴, tps 아님) | 4 |
| 9 | 13 | 104 | 태그 | EntityType 태그 13 = Champion — skill/skill2 cooldown 인라인 게이트 | 4 |
| 10 | -1 | 51 | 센티널 | Option 니치 None(attack_effect 태그) · 타워 반복자 종료 표식(55464, 55547) | 4 |
| 11 | 5 | 49 | 길이 | 반환 코드 5(피난처 없음 또는 피난처 사거리 권역 안) · 포지션 배열 길이 | 4 |
| 12 | 4 | 168 | 태그 | 반환 코드 4(사거리 통과·도주가능) | 4 |
| 13 | 3 | 170 | 태그 | 반환 코드 3(마진 통과·사거리 미통과) | 4 |
| 14 | 2 | 172 | 임계 | 반환 코드 2(전원 마진 미통과) · team bounds | 4 |
| 15 | 1 | 65 | 태그 | 반환 코드 1(근처 적 없음) · max(x,1) 가드 · level-1 | 4 |
| 16 | 0 | 166 | 태그 | 반환 코드 0(committers 있음) · unwrap_or(0) | 4 |

**`knobs` 조정점 7건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 근처 적/타워 반경 | passive_line.rs:61,81 | 22500000001 | 올리면 더 먼 적을 위협 후보로, 더 먼 타워를 억지력으로 셈 | 4 | 기존 |
| 1 | 전력 합산 반경(적 아군·내 아군 ↔ e) | passive_line.rs:97,115 | 22500000000 | 올리면 kill/deterrent 전력에 더 먼 챔피언까지 합산 | 4 | 기존 |
| 2 | 적 아군 참여 HP% | passive_line.rs:97 | 40 | 내리면 저체력 적도 kill 전력에 포함 → 위협 판정↑ | 4 | 기존 |
| 3 | 반응 틱 | passive_line.rs:70 | 6 | 올리면 적 도달 허용 거리(er + ms*reaction)와 탈출 허용 시간이 늘어 위협 판정↑·도주가능 판정↓ | 4 | 기존 |
| 4 | 마진 계수/기저 | passive_line.rs:90 | 80 | diff_bound=80~160. 올리면 '내가 훨씬 오래 버텨야' 비위협 → 위협 판정↑(더 자주 도주) | 4 | 기존 |
| 5 | 생존 하한 계수/기저 | passive_line.rs:91 | 45 | die_tick_bound=45~90. 올리면 느린 적에게도 '충분히 버틴다' 판정이 어려워져 위협↑ | 4 | 기존 |
| 6 | die 틱 환산 | passive_line.rs:109,135 | 60 | 초당 DPS→틱 환산 리터럴(60fps 가정). tps 와 불일치 시 my_die/e_die 스케일 왜곡 | 4 | 기존 |

<details><summary>`callees` 피호출자 29건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | aggressive_ratio | game_core::AthleteParameter::aggressive_ratio | pub | fn(&game_core::AthleteParameter) -> usize | game-core\src\simulation\state\player.rs:432 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | attack_cooldown | game_core::Entity::attack_cooldown | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1747 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 2 | attack_cooltime | game_core::Entity::attack_cooltime | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1766 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 3 | attack_duration | game_core::Entity::attack_duration | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1770 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | can_attack | game_core::Entity::can_attack | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1527 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 5 | can_skill | game_core::Entity::can_skill | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1708 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 6 | can_skill2 | game_core::Entity::can_skill2 | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1721 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 7 | distance | game_core::Entity::distance | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2161 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 8 | distance_sq | game_core::utils::distance_sq | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:6 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 9 | distance_sq | game_core::Entity::distance_sq | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2157 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 10 | expected_damage_target | game_core::Effect::expected_damage_target | pub | fn(&game_core::Effect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + , &game_core::Entity) -> usize | game-core\src\simulation\effect.rs:91 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 11 | is_empty | game_core::PatchSetting::is_empty | pub | fn(&game_core::PatchSetting) -> bool | game-core\src\patch_engine.rs:853 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 12 | is_empty | game_core::ModAiRegistry::is_empty | pub | fn(&game_core::ModAiRegistry) -> bool | game-core\src\mod_ai.rs:15 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 13 | is_empty | game_core::PublicChampionEvidence::is_empty | pub | fn(&game_core::PublicChampionEvidence) -> bool | game-core\src\banpick\public_evidence.rs:96 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 14 | is_ignored_well_enemy | game_ai::plan_legacy::old::is_ignored_well_enemy | pub | fn(usize, &game_core::PlayerState, &game_core::Entity) -> bool | game-ai\src\plan_legacy\old\fight_model.rs:754 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 15 | is_visible_from | game_core::Entity::is_visible_from | pub | fn(&game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1481 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 16 | iter_champions | game_core::AbstractGameWithCache::<'a, 'b>::iter_champions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5759 ~ game_core[6a30]::simulation::{impl#5}::iter_champions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1904 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 17 | iter_towers_without_nexus | game_core::AbstractGameWithCache::<'a, 'b>::iter_towers_without_nexus | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5750 ~ game_core[6a30]::simulation::{impl#5}::iter_towers_without_nexus::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1830 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 18 | player_by_champion_id | game_core::AbstractGameWithCache::<'a, 'b>::player_by_champion_id | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> std::option::Option<&'a/#0 game_core::PlayerState> | game-core\src\simulation.rs:1915 | True | True | 3 | IR 호출 심볼 일치(토큰 경계) |
| 19 | player_count | game_core::GameRule::player_count | pub | fn(&game_core::GameRule) -> usize | game-core\src\data\match_info.rs:542 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 20 | player_count | game_core::TutorialType::player_count | pub | fn(&game_core::TutorialType) -> usize | game-core\src\simulation\game\runner.rs:294 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 21 | push | <game_core::Staff as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::staff | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\staff.rs:38 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 22 | push | <game_core::Athlete as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:434 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 23 | push | <game_core::Contract as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:2498 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 24 | radius | game_core::Entity::radius | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1509 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 25 | range | game_core::Effect::range | pub | fn(&game_core::Effect, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:25 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 26 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 27 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 28 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
</details>

⚠**미매칭 13개**: `any_margin_pass`, `any_range_pass`, `collect`, `drop  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `else`, `llvm.assume`, `llvm.memcpy.p0.p0.i64`, `llvm.memset.p0.i64`, `llvm.umax.i64`, `llvm.usub.sat.i64`, `map_or`, `next  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 9개는 **전부 다른 함수**라 싣지 않는다`, `reserve_internal_or_panic`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 4곳** (m04.ll:23364, m10.ll:8258, m10.ll:10249, m13.ll:27952) · **형제 0개** 

**`open` 5건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | 반환 u8 코드 0~5 의 소스 이름(열거형/상수명) — IR 에 리터럴만 남음. 의미는 분기 구조에서 역산(0 committers·1 무적·2 마진실패·3 사거리실패·4 도주가능·5 피난처없음/권역안). _docs '도주가능 시점 게이트 차단사유' 와 부합하나 명칭은 미확인 | 4 |  |
| 1 | 표기 불가 | L103 의 nuke 초기값: `%516 phi [0,%510]` 는 attack_cooldown > tps 일 때 0 유지, 아니면 max(0, c.attack[..]) — 소스가 `if can_attack \|\| cooldown<=tps { nuke = c.attack }` 인지 `max` 인지는 외연 동일·표기 불가 | 4 |  |
| 2 | 미탐색 | v46_stage1(m04.ll:16364)/v46_stage2(18037) 는 이 함수 본문에서 호출되지 않음(call 0건) — 인라인도 아님(별도 define 존재, 다른 호출자 소유). 계약: stage1(&self,version,&PlayerState,&OperationData,&Entity×3,usize,bool,Option<&[usize]>,&Vec<&Entity>)->Vec<(usize,usize,usize)>, stage2(&self,&PlayerState,&OperationData,&Entity,&Entity,&Vec<(usize,usize,usize)>)->bool | 4 |  |
| 3 | 미탐색 | L73 player_count() 인라인이 tutorial 값을 인원수로 어떻게 매핑하는지(First/Bottom→2v2, MidBottom→3v3 로 관측)는 runner.rs:295 switch 결과이며 반환값 자체(2/3)는 IR 에 남지 않음(오프셋으로 접힘) | 4 |  |
| 4 | 미탐색 | attack_cooldown() 인라인 switch 의 타입별 오프셋(176~496)은 entity.rs:1748~1760 각 variant 필드 — Champion(176=0xb0) 외 타입은 near_enemies 가 챔피언뿐이라 실제로 도달하지 않음(사장 분기) | 4 |  |

**`notes` 2건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | any_margin_pass/any_range_pass ↔ phi 대응: dbg_value(56359 margin=1 at L144 통과 직후 / 56438 range=1 at L155 통과 직후)와 phi 56486~56487(%462=L144 통과 시 true, %461=L155 통과 시 true)이 일치 — %461=any_range_pass, %462=any_margin_pass 로 확정 | 4 | 사실 서술 |
| 1 | min_by_key fold(refuge 선택, 별도 define `…v46_flee_gate_check0E0EB4x_4fold…`)·iter_towers_without_nexus(1830)·is_ignored_well_enemy(fight_model.rs:754)·aggressive_ratio(player.rs:432)·expected_damage_target 본문 미독 — 계약만. refuge 키가 distance_sq(t,champ) 라는 것은 본체 55593~55621(첫 원소) 로 확정 | 4 | 사실 서술 |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

