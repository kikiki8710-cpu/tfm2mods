---

### `213` PassiveLinePlan::v46_stage1 — v46 라인 CS 사망예측 1단계 — near_enemies 중 '나를 먼저 죽일 수 있는' 커밋터(e.id, my_die, kill_dps)를 bumpalo Vec 으로 반환

| 항목 | 값 |
|---|---|
| id | `passive_line__PassiveLinePlan__v46_stage1` |
| 심볼 | `_RNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old12passive_lineNtB2_15PassiveLinePlan10v46_stage1` |
| 소스 | `game-ai\src\plan_legacy\old\passive_line.rs:663` |
| IR | `m04.ll` 16364~18034행 |
| 경로·가시성 | `game_ai::plan_legacy::old::PassiveLinePlan::v46_stage1` · **in:game_ai::plan_legacy::old::passive_line** |
| 계층 | 레거시 플랜 |
| exe | `d26900` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r17` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_ai::plan_legacy::old::PassiveLinePlan, usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, &game_core::Entity, &game_core::Entity, usize, bool, std::option::Option<&[usize]>, &bumpalo::collections::vec::Vec< &game_core::Entity>) -> bumpalo::collections::vec::Vec< (usize, usize, usize)>
```

**TLS 메모 절(`sig.tls` · 정정 경로 `/specs[213]/sig/tls/<키>`)**

없음 — 본문·aux 에 LocalKey/call_once/threadlocal 참조 0건(grep 실측)

<details><summary>인자 15개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | bumpalo::collections::Vec<'bump, (usize, usize, usize)> 32B | 레이아웃(본문 관측 m04.ll:16576~16627·17180~17291): +0x0 ptr(비었을 때 dangling 8) · +0x8 &Bump(=ctx.pool) · +0x10 cap · +0x18 len. 4워드 전부 live. 원소 24B (usize,usize,usize) = (+0 e.id, +8 my_die, +16 kill_dps) 패딩 없음 · 전부 live(17284~17288) | 4 |
| 1 | 1 | version | usize |  | 4 |
| 2 | 2 | team | usize |  | 4 |
| 3 | 3 | my_pos | Position(i32 태그, zext → usize) |  | 4 |
| 4 | 4 | cache | &AbstractGameWithCache(8840B) |  | 4 |
| 5 | 5 | ctx | &GameContext(64B) |  | 4 |
| 6 | 6 | champ | &Entity(1728B) |  | 4 |
| 7 | 7 | front | &Entity(1728B) |  | 4 |
| 8 | 8 | my_tower | &Entity(1728B) |  | 4 |
| 9 | 9 | my_hp | usize |  | 4 |
| 10 | 10 | range_gate | bool |  | 4 |
| 11 | 11 | only.data_ptr | Option<&[usize]> 의 ptr (null=None) |  | 4 |
| 12 | 11 | only.len | usize |  | 4 |
| 13 | 12 | near_enemies.data_ptr | &[&Entity] 의 ptr |  | 4 |
| 14 | 12 | near_enemies.len | usize |  | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn v46_stage1(version, team, my_pos, cache, ctx, champ, front, my_tower, my_hp, range_gate, only: Option<&[usize]>, near_enemies: &[&Entity]) -> bumpalo::Vec<(usize,usize,usize)>

// 준비 (L667~689)
pool = ctx.pool; tps = ctx.setting.tick_per_second; my_ms = champ.stat_cached.move_speed
my_range = champ.attack_effect.map(|f| f.range + f.growth_range*(level-1) + champ.stat_buff_cached.range).unwrap_or(0)   // L671 closure$0 · effect.rs:26 Effect::range
pull = my_range + champ.radius() + front.radius()          // L672 · radius() = mult==0 ? radius : radius*(mult+100)/100
cs_lock = champ.attack_duration()                          // L673
escape_ticks = front.distance(my_tower).saturating_sub(pull) / max(my_ms,1) + cs_lock   // L674
tower_disable_tick = match ctx.tutorial { First|Bottom => setting.tower_attack_disable_tick_2v2, MidBottom => …_3v3, _ => …tower_attack_disable_tick }   // L677~680
my_towers: bumpalo Vec<&Entity> = cache.iter_towers_without_nexus(team).filter(|t| game.tick() <= tower_disable_tick && t.distance(front).saturating_sub(pull) <= 150000).collect_in(pool)   // L684~688 closure$1(aux)
committers = Vec::new_in(pool)                              // L689

for e in near_enemies {                                     // L690
  if only.is_some_and(|ids| !ids.contains(&e.id)) { continue }   // L691 (IR: only==null → 통과, contains → 통과, 아니면 skip)
  ep = cache.player_by_champion_id(e.id).unwrap()          // L694 (None 이면 패닉)
  e_pos = ep.info.position.as_index()                      // L695
  aggr = ep.info.parameter.aggressive_ratio()              // L696
  diff_bound      = 80 + (1000-aggr)*80/1000               // L697  (80..160)
  die_tick_bound  = 45 + aggr*45/1000                      // L698  (45..90)
  tower_tick_bound= 15 + aggr*35/1000                      // L699  (15..50)

  // 적측 킬 능력 (L704~718): 적 팀(1-team) 챔피언 전원(e 자신 포함, 가시성 무관)
  kill_dps = 0; kill_nuke = 0
  for a in cache.player_champion[1-team].iter().flatten() {
    if distance_sq(a, e) > 150000² { continue }            // L705
    if a.hp*100/max(a.stat_cached.hp,1) < 40 { continue }  // L708
    c = &cache.player_champion_cache[ap.info.team][ap.info.position]  where ap = player_state at a.id 의 (t,p)  // L711~712 (unwrap·bounds 패닉 가드)
    nuke = if a.can_attack() || a.attack_cooldown() <= tps { c.attack[my_pos] } else { 0 }            // L714 (attack_cooldown = ty 별 필드 switch entity.rs:1748)
    if a.can_skill()  || !(a is Champion && a.skill_cooldown  > tps) { nuke = max(nuke, c.skill[my_pos]) }   // L715
    if a.can_skill2() || !(a is Champion && a.skill2_cooldown > tps) { nuke = max(nuke, c.skill2[my_pos]) }  // L716
    kill_dps  += c.attack_per_sec[my_pos] + c.skill_per_sec[my_pos] + c.skill2_per_sec[my_pos]   // L717
    kill_nuke += nuke                                       // L718
  }
  my_die = my_hp.saturating_sub(kill_nuke) * 60 / max(kill_dps,1)   // L720 (틱)

  // 우리측 억지력 (L726~749): 내 팀 챔피언 + 내 타워
  det_dps = 0; det_nuke = 0
  for a in cache.player_champion[team].iter().flatten() {
    if distance_sq(a, e) > 150000² { continue }            // L727
    if !a.is_visible_from(e) { continue }                  // L730 entity.rs:1482 — e.team Neutral 이면 가시, 아니면 a.visible_state[e.team]==Visible
    if is_ignored_well_enemy(version, ep, a) { continue }  // L733
    c = ChampionCache of a (L736~737, unwrap·bounds 패닉 가드)
    nuke = (L739~741: 위와 같은 즉발 규칙, 대상 열 = e_pos)
    det_dps  += c.attack_per_sec[e_pos] + c.skill_per_sec[e_pos] + c.skill2_per_sec[e_pos]   // L742
    det_nuke += nuke                                        // L743
  }
  for t in my_towers {                                      // L745
    if let Some(atk) = t.attack_effect.as_ref() {           // L746
      dmg = atk.expected_damage_target(ctx, t as &dyn AbstractEntity, e)   // L747
      det_dps  += dmg*tps / max(t.attack_cooltime(),1)     // L748
      det_nuke += dmg                                       // L749
    }
  }
  e_die = e.hp.saturating_sub(det_nuke) * 60 / max(det_dps,1)   // L752

  // 게이트 (L755): 둘 중 하나라도 참이면 커밋터 아님
  if my_die + diff_bound > e_die || my_die >= escape_ticks.saturating_sub(tower_tick_bound) { continue }

  // 커밋 판정 (L762~770)
  if e.stat_cached.move_speed > my_ms {                     // L762 — 나보다 빠르면 무조건 커밋터
    push
  } else if my_die < die_tick_bound {                       // L764
    if range_gate {                                         // L767
      e_range = e.attack_effect.map(range 공식).unwrap_or(0)           // L768 closure$3
      // L770: e 가 CS 중인 나에게 닿는가
      if pull + e.distance(front) > e_range + e.move_speed*cs_lock + e.radius() + champ.radius() { continue }
    }
    push
  } else { continue }
  push: committers.push((e.id, my_die, kill_dps))          // L775
}
return committers                                          // L777 (sret memcpy 32B)

극성 근거: L755 %345=or(%344,%342) → br → %411(continue). L762 %349 ugt → %350(push). L764 %358 ult → %359. L770 %402 ugt → %411(skip). 순서는 !dbg 줄번호(755<762<764<767<770).
```

**`mem` 메모리 접근 52건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | GameContext | 0x0 | pool | r | bumpalo Bump — committers Vec::new_in 및 from_iter_in 의 할당자 | 4 | OK |  |
| 1 | GameContext | 0x8 | setting | r | &GameSetting | 4 | OK |  |
| 2 | GameContext | 0x38 | tutorial | r | TutorialType 태그(1B) switch — L677~680 tower_disable_tick 선택 | 4 | OK |  |
| 3 | GameSetting | 0x12f8 | tick_per_second | r | tps. 타워 DPS 환산(L748)·쿨다운>tps 비교(L739~741) | 4 | OK |  |
| 4 | GameSetting | 0x13f8 | tower_attack_disable_tick | r | tutorial 태그 0/2/4/6/7/8(None·TopSolo·MidSolo·JungleOnly·Line·Total) → L680 | 4 | OK |  |
| 5 | GameSetting | 0x1400 | tower_attack_disable_tick_2v2 | r | tutorial 태그 1/3(First·Bottom) → L678(switch 기본 phi 5120) | 4 | OK |  |
| 6 | GameSetting | 0x1408 | tower_attack_disable_tick_3v3 | r | tutorial 태그 5(MidBottom) → L679 | 4 | OK |  |
| 7 | AbstractGameWithCache | 0x0 | game.data_ptr | r | &dyn AbstractGame 팻포인터 — closure$1 캡처(aux 에서 vtable+0x28 tick 호출) | 4 | OK |  |
| 8 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r |  | 4 | OK |  |
| 9 | AbstractGameWithCache | 0x1e0 | player_champion[2][5] | r | [1-team] 적 5슬롯 순회(L704) · [team] 아군 5슬롯 순회(L726) · id 검색(L711/L736) | 4 | OK |  |
| 10 | AbstractGameWithCache | 0x230 | player_state[2][5] | r | id 검색으로 찾은 (t,p) 의 PlayerState.unwrap()(L711/L736) | 4 | OK |  |
| 11 | AbstractGameWithCache | 0x280 | player_champion_cache[2][5] | r | ChampionCache 800B = 20×[usize;5]. [ap.info.team][ap.info.position] | 4 | OK |  |
| 12 | ChampionCache | 0x0 | attack[pos] | r | 즉발 평타 피해(대상 포지션별) | 4 | OK |  |
| 13 | ChampionCache | 0x28 | skill[pos] | r |  | 4 | OK |  |
| 14 | ChampionCache | 0x50 | skill2[pos] | r |  | 4 | OK |  |
| 15 | ChampionCache | 0x190 | attack_per_sec[pos] | r | 초당 DPS | 4 | OK |  |
| 16 | ChampionCache | 0x1b8 | skill_per_sec[pos] | r |  | 4 | OK |  |
| 17 | ChampionCache | 0x1e0 | skill2_per_sec[pos] | r |  | 4 | OK |  |
| 18 | PlayerState | 0x930 | info.team | r | ChampionCache 1차 인덱스(bounds<2 패닉 가드) | 4 | OK |  |
| 19 | PlayerState | 0x9c0 | info.position@tag | r | as_index(entity.rs:581) → ChampionCache 2차 인덱스 / e_pos | 4 | OK |  |
| 20 | PlayerState | 0x180 | info.parameter | r | &AthleteParameter(744B) → aggressive_ratio(ep)(L696) | 4 | OK |  |
| 21 | Entity | 0x4c0 | attack_effect@tag | r | i32 -1 = None. champ(L671)·e(L768)·tower t(L746) 셋 다 | 4 | OK |  |
| 22 | Entity | 0x490 | attack_effect@Some.0 | r | &Effect(56B) — 타워 t 의 expected_damage_target 수신자(L747) | 4 | OK |  |
| 23 | Entity | 0x4a0 | attack_effect@Some.0.range | r | Effect::range(effect.rs:26) = range + growth_range*(level-1) + stat_buff_cached.range | 4 | OK |  |
| 24 | Entity | 0x4a8 | attack_effect@Some.0.growth_range | r |  | 4 | OK |  |
| 25 | Entity | 0x5c8 | level | r | level-1 (wrapping add -1) | 4 | OK |  |
| 26 | Entity | 0x438 | stat_buff_cached.range | r |  | 4 | OK |  |
| 27 | Entity | 0x470 | stat_buff_cached.radius_mult | r | i32. Entity::radius(entity.rs:1511~1515): mult==0 ? radius : radius*(mult+100)/100 | 4 | OK |  |
| 28 | Entity | 0x680 | radius | r |  | 4 | OK |  |
| 29 | Entity | 0x640 | stat_cached.move_speed | r | my_ms(L670) · e.ms(L762·L770) | 4 | OK |  |
| 30 | Entity | 0x5c0 | id | r | e.id(only 필터·player_by_champion_id·push) · a.id(캐시 검색) | 4 | OK |  |
| 31 | Entity | 0x660 | x | r | distance_sq(entity.rs:2158 → utils.rs:7) abs_diff² 합 | 4 | OK |  |
| 32 | Entity | 0x668 | y | r |  | 4 | OK |  |
| 33 | Entity | 0x670 | hp | r | e.hp(L752 e_die) · a.hp(L708 hp% 게이트) | 4 | OK |  |
| 34 | Entity | 0x628 | stat_cached.hp | r | 최대 HP(L708 hp% 분모, max(…,1)) | 4 | OK |  |
| 35 | Entity | 0x0 | team@tag | r | TeamType: 0 Player / 1 Neutral. is_visible_from(entity.rs:1482) — e 가 Neutral 이면 무조건 가시 | 4 | OK |  |
| 36 | Entity | 0x8 | team@Player.0 | r | e 의 팀 → a.visible_state[team] 인덱스(bounds<2 패닉 가드) | 4 | OK |  |
| 37 | Entity | 0x38 | visible_state[i]@tag | r | stride 8·2슬롯. VisibleState 0 Visible 만 통과(data.rs:122 is_visible) | 4 | OK |  |
| 38 | Entity | 0x68 | ty@tag | r | EntityType 태그 — attack_cooldown 인라인 switch(entity.rs:1748) · ==13 Champion 판정(skill/skill2 쿨) | 4 | OK |  |
| 39 | Entity | 0xb0 | ty@Champion.0.attack_cooldown / ty@SmallJiangshi.info.attack_cooldown | r | switch case 13·8 | 4 | OK |  |
| 40 | Entity | 0xb8 | ty@Champion.0.skill_cooldown (L740·L715) / ty@Minion.info.attack_cooldown (case 1) | r |  | 4 | OK |  |
| 41 | Entity | 0xc0 | ty@Champion.0.skill2_cooldown | r | L741·L716 (entity.rs:1791) | 4 | OK |  |
| 42 | Entity | 0xc8 | ty@Bear.info.attack_cooldown | r | case 9 | 4 | OK |  |
| 43 | Entity | 0xd0 | ty@Illusion.info.attack_cooldown | r | case 12 | 4 | OK |  |
| 44 | Entity | 0xd8 | ty@Revenant.info.attack_cooldown | r | case 11 | 4 | OK |  |
| 45 | Entity | 0xe8 | ty@Jungle/Ghoul.info.attack_cooldown | r | case 4·7 | 4 | OK |  |
| 46 | Entity | 0xf0 | ty@Eagle.info.attack_cooldown | r | case 10 | 4 | OK |  |
| 47 | Entity | 0x110 | ty@Tower.info.attack_cooldown | r | case 2 | 4 | OK |  |
| 48 | Entity | 0x1f0 | ty@Epic/Serpen.info.attack_cooldown | r | case 5·6. case 0 None·3 Nexus 는 쿨 없음(바로 즉발 인정) | 4 | OK |  |
| 49 | bumpalo Vec<&Entity>(my_towers, 지역 %23) | 0x0 / 0x18 | ptr / len | r | from_iter_in 결과 순회(L745) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 50 | (sret) bumpalo Vec<(usize,usize,usize)> | 0x0..0x20 | committers 전체 | w | 이 함수의 유일한 외부 쓰기 표면. &mut self 없음(연관함수). 인자 어느 것도 쓰지 않음 | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) | 지역 %20 을 memcpy 32B (m04.ll:16661, L777) |
| 51 | committers 원소(bump 힙) | ptr[len]+0 / +8 / +16 | e.id / my_die / kill_dps | w | cap==len 이면 reserve_internal_or_panic(used_cap=len, 1, exact=true)(17192) | 4 | 확인불가(tcx 사전에 타입 없음) | m04.ll:17284~17288 (L775) · len+=1 (17291) |

**`consts` 상수 17건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | -1 | 671 | 센티널 | Option<Effect> attack_effect 의 None 니치 태그(i32, Entity+0x4c0). L671·L746·L768 | 4 |
| 1 | 100 | 672 | 계수 | radius*(radius_mult+100)/100 — Entity::radius 백분율(entity.rs:1515). L708 hp*100/max_hp 의 100 도 동일 리터럴 | 4 |
| 2 | 5120 | 677 | 산출값 | phi 에 접힌 GameSetting 오프셋(0x1400 tower_attack_disable_tick_2v2). 5112=0x13f8 기본 · 5128=0x1408 3v3 — 판정값이 아니라 오프셋(reads 참조) | 4 |
| 3 | 5 | 677 | 태그 | TutorialType 태그 5 = MidBottom → 3v3 disable tick. 태그 1 First / 3 Bottom → 2v2. 나머지 → 기본 | 4 |
| 4 | 22500000000 | 705 | 임계 | 150000² — 적/아군 후보를 e 로부터 150000 이내로 제한(제곱거리 ugt 이면 제외). L705(적)·L727(아군) | 4 |
| 5 | 40 | 708 | 임계 | 적 a 의 hp% (hp*100/max(max_hp,1)) < 40 이면 kill 집계 제외 | 4 |
| 6 | 1000 | 697 | 계수 | aggressive_ratio 의 분모(0..1000 스케일). L697~699 | 4 |
| 7 | 80 | 697 | 오프셋가감 | diff_bound = 80 + (1000-aggr)*80/1000 — 내가 e 보다 '얼마나 먼저' 죽어야 커밋터로 보는 마진(틱). 공격성 0 → 160, 1000 → 80 | 4 |
| 8 | 45 | 698 | 계수 | die_tick_bound = 45 + aggr*45/1000 — 내가 이 틱 안에 죽을 때만 커밋터(느린 적 한정). 45~90 | 4 |
| 9 | 35 | 699 | 오프셋가감 | tower_tick_bound = 15 + aggr*35/1000 — 타워 도피 시간에서 빼는 마진(15~50) | 4 |
| 10 | 15 | 699 | 미상 | tower_tick_bound 기본 15 | 4 |
| 11 | 60 | 720 | 계수 | 초→틱 환산 하드코딩(tps 미사용): my_die = (my_hp - kill_nuke)*60/max(kill_dps,1) · e_die 도 동일(L752). *_per_sec 가 초당이라 60틱=1초 | 4 |
| 12 | 13 | 740 | 태그 | EntityType 태그 13 = Champion — skill/skill2 쿨다운은 챔피언만 검사(entity.rs:1775/1790) | 4 |
| 13 | 2 | 704 | 길이 | 팀 배열 길이 2 bounds check(panic_bounds_check 인자) — 판정값 아님 | 4 |
| 14 | 8 | 691 | 태그 | only.contains 의 slice 청크 언롤 폭(8개 단위 or-접기 후 잔여 선형). 판정값 아님 | 4 |
| 15 | 150001 | 686 | 미상 | (aux closure$1) t.distance(front).saturating_sub(pull) < 150001 ⇔ ≤150000 — 내 타워가 front 기준 150000 이내면 억지력 타워로 산입 | 4 |
| 16 | 40 | 685 | 임계 | (aux closure$1) vtable+0x28 = AbstractGame::tick 슬롯(divtable 실측) — game.tick() > tower_disable_tick 이면 타워 제외. 오프셋이지 판정값 아님 | 3 |

**`knobs` 조정점 8건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 후보 반경 | passive_line.rs:705 / 727 | 22500000000 | 150000². 올리면 더 먼 적/아군까지 킬·억지 DPS 에 산입 → 커밋터가 늘고(적) 동시에 억지력도 늘어(아군) 방향은 상황 의존 | 4 | 기존 |
| 1 | 적 hp% 컷 | passive_line.rs:708 | 40 | 내리면 저체력 적도 킬 능력에 포함 → my_die 감소 → 커밋터 증가(더 자주 귀환 트리거) | 4 | 기존 |
| 2 | diff_bound 기본/공격성 계수 | passive_line.rs:697 | 80 | 올리면 '내가 훨씬 먼저 죽어야' 커밋터 → 커밋터 감소(덜 소극적). 공격성 낮을수록 마진이 커져 더 보수적으로 커밋터 인정이 줄어듦 | 4 | 기존 |
| 3 | die_tick_bound | passive_line.rs:698 | 45 | 올리면 느린 적도 커밋터로 더 자주 인정(my_die 상한 완화) | 4 | 기존 |
| 4 | tower_tick_bound | passive_line.rs:699 | 15 | 올리면 escape 여유를 더 깎아 my_die>=escape 로 게이트 탈락이 늘어 커밋터 감소 | 4 | 기존 |
| 5 | 타워 산입 반경 | passive_line.rs:686 (closure$1) | 150001 | 내리면 억지 타워가 줄어 e_die 증가 → 커밋터 증가 | 4 | 기존 |
| 6 | 초→틱 환산 | passive_line.rs:720 / 752 | 60 | tps 가 60 이 아닌 환경에선 my_die/e_die 가 실제 틱과 어긋남. 두 곳 같이 바꿔야 비율 유지 | 4 | 기존 |
| 7 | range_gate 인자 | passive_line.rs:767 (호출자 update) | true/false | false 면 느린 적도 사거리 도달 여부 없이 커밋터 인정(v46_committers 재검증 사이트가 이렇게 부름) | 4 | 기존 |

<details><summary>`callees` 피호출자 28건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | aggressive_ratio | game_core::AthleteParameter::aggressive_ratio | pub | fn(&game_core::AthleteParameter) -> usize | game-core\src\simulation\state\player.rs:432 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | as_index | game_core::BigGoal::as_index | pub | fn(&game_core::BigGoal) -> usize | game-core\src\simulation\ai_interface.rs:338 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
| 2 | as_index | game_core::Position::as_index | pub | fn(&game_core::Position) -> usize | game-core\src\simulation\entity.rs:580 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
| 3 | attack_cooldown | game_core::Entity::attack_cooldown | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1747 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 4 | attack_cooltime | game_core::Entity::attack_cooltime | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1766 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 5 | attack_duration | game_core::Entity::attack_duration | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1770 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 6 | can_attack | game_core::Entity::can_attack | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1527 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 7 | can_skill | game_core::Entity::can_skill | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1708 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 8 | can_skill2 | game_core::Entity::can_skill2 | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1721 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 9 | contains | game_core::RectU64::contains | pub | fn(&game_core::RectU64, u64, u64) -> bool | game-core\src\setting.rs:40 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 10 | contains | game_core::RectI64::contains | pub | fn(&game_core::RectI64, i64, i64) -> bool | game-core\src\setting.rs:55 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 11 | contains | game_core::setting::champion::nightmare::NightmareWellRect::contains | in:game_core::setting::champion::nightmare | fn(game_core::setting::champion::nightmare::NightmareWellRect, u64, u64) -> bool | game-core\src\setting\champion\nightmare.rs:28 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 12 | distance | game_core::Entity::distance | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2161 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 13 | distance_sq | game_core::utils::distance_sq | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:6 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
| 14 | distance_sq | game_core::Entity::distance_sq | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2157 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
| 15 | expected_damage_target | game_core::Effect::expected_damage_target | pub | fn(&game_core::Effect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + , &game_core::Entity) -> usize | game-core\src\simulation\effect.rs:91 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 16 | is_ignored_well_enemy | game_ai::plan_legacy::old::is_ignored_well_enemy | pub | fn(usize, &game_core::PlayerState, &game_core::Entity) -> bool | game-ai\src\plan_legacy\old\fight_model.rs:754 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 17 | is_visible_from | game_core::Entity::is_visible_from | pub | fn(&game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1481 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 18 | iter_towers_without_nexus | game_core::AbstractGameWithCache::<'a, 'b>::iter_towers_without_nexus | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5750 ~ game_core[6a30]::simulation::{impl#5}::iter_towers_without_nexus::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1830 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 19 | player_by_champion_id | game_core::AbstractGameWithCache::<'a, 'b>::player_by_champion_id | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> std::option::Option<&'a/#0 game_core::PlayerState> | game-core\src\simulation.rs:1915 | True | True | 3 | IR 호출 심볼 일치(토큰 경계) |
| 20 | push | <game_core::Staff as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::staff | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\staff.rs:38 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 6개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 21 | push | <game_core::Athlete as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:434 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 6개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 22 | push | <game_core::Contract as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:2498 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 6개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 23 | radius | game_core::Entity::radius | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1509 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 24 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 7개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 25 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 7개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 26 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 7개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 27 | v46_stage1 | game_ai::plan_legacy::old::PassiveLinePlan::v46_stage1 | in:game_ai::plan_legacy::old::passive_line | fn(&game_ai::plan_legacy::old::PassiveLinePlan, usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, &game_core::Entity, &game_core::Entity, usize, bool, std::option::Option<&[usize]>, &bumpalo::collections::vec::Vec< &game_core::Entity>) -> bumpalo::collections::vec::Vec< (usize, usize, usize)> | game-ai\src\plan_legacy\old\passive_line.rs:663 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
</details>

⚠**미매칭 6개**: `drop  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `llvm.memcpy.p0.p0.i64`, `llvm.memset.p0.i64`, `llvm.umax.i64`, `llvm.usub.sat.i64`, `reserve_internal_or_panic`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 3곳** (m04.ll:21664, m04.ll:21883, m04.ll:22374) · **형제 18개** (PassiveLinePlan)

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
| 0 | 표기 불가 | L755 한 줄 안의 두 조건(my_die+diff_bound>e_die / my_die>=escape-tower_tick_bound)의 소스 표기 순서 — column 정보 부재. 둘 다 skip 이라 동작은 동일(표기 불가) | 4 |  |
| 1 | 미탐색 | L691 closure$2 의 소스 표기가 `only.is_some_and(\|ids\| !ids.contains(id))` 인지 `!only.is_none_or(..)` 인지 — 동작(only None 통과·포함 통과·미포함 skip)은 확정, 표기만 불가 | 4 |  |
| 2 | 표기 불가 | L697~699 산술의 소스 표기(예: `80 + (1000-aggr)*80/1000` vs `(80*(2000-aggr))/1000`) — IR 은 sub/mul/udiv/add 순으로 접혀 있어 그 순서로 적었다. 외연 동일(표기 불가) | 4 |  |
| 3 | 미탐색 | expected_damage_target·attack_duration·attack_cooltime·is_ignored_well_enemy·iter_towers_without_nexus 의 내부는 미열람(계약만 — 지시 범위) | 4 |  |
| 4 | 미탐색 | `only` 가 Some 일 때 %12(len) 의 undef 여부는 호출자 IR(null,undef)에서만 확인 — 함수 내부는 %111(ptr null) 로만 None 을 판정하므로 len 은 None 일 때 미사용(안전) | 4 |  |

**`notes` 1건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | attack_cooldown()/skill_cooldown()/skill2_cooldown() 헬퍼(entity.rs:1748/1776/1791)의 정확한 이름·반환 의미는 dloc 스코프명으로만 확인(인라인). None/Nexus 타입이 쿨 0 으로 처리되는 것은 switch 대상 블록(%540 직행)으로 확정 | 4 | 사실 서술 |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

