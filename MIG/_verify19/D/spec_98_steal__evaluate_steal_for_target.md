---

### `98` steal::evaluate_steal_for_target — [v4/v28] 한 스틸 대상(에픽/세르펜)에 대해 None/Lurk/Commit 을 판정 — 도착틱·적 처치 잔여틱·내 폭딜+처형컷 vs 대상 hp(실측/추정)·위협 적 수·내 hp%·구조적 생존 가능성(적 사거리·폭딜 vs 내구도)·대기 부시 여부를 조합. 반환 (StealAction, enemy_tick)

| 항목 | 값 |
|---|---|
| id | `steal__evaluate_steal_for_target` |
| 심볼 | `_RNvNtNtCshdEBA0ozCnw_7game_ai11plan_legacy5steal25evaluate_steal_for_target` |
| 소스 | `game-ai\src\plan_legacy\steal.rs:392` |
| IR | `m07.ll` 55331~57634행 |
| 경로·가시성 | `game_ai::plan_legacy::steal::evaluate_steal_for_target` · **in:game_ai::plan_legacy::steal** |
| 계층 | 기타 |
| exe | `d9bce0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r10` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan, &game_core::Entity, game_core::StealTarget, std::option::Option<&game_core::Entity>, usize, usize, usize, (u64, u64), bool) -> (game_ai::plan_legacy::steal::StealAction, usize)
```

<details><summary>인자 13개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | &mut (StealAction, usize)(16B) | +0 tag(0 None/1 Lurk/2 Commit) +1 target(=인자 target) +8 usize = 조기 None 이면 usize::MAX(-1), 그 외 enemy_tick 그대로 (m07.ll:57628 phi) | 4 |
| 1 | 1 | _version | usize | 미사용 — IR 인자에서 제거됨(DILocalVariable 만) | 4 |
| 2 | 2 | player | &PlayerState(2528B) | IR %1. info.team(+0x930)·info.parameter(+0x180) last_hit_execute_hp | 4 |
| 3 | 3 | data | &OperationData(24B) | SROA 승격 — role %2 = data.cache(&AbstractGameWithCache), %3 = data.context(&GameContext) | 4 |
| 4 | 4 | _team_plan | &TeamPlan | 미사용 — IR 인자에서 제거됨 | 4 |
| 5 | 5 | my_champ | &Entity(1728B) | IR %4 | 4 |
| 6 | 6 | target | StealTarget(i1) | IR %5. false=Epic / true=Serpen. 대기 부시 후보 선택·반환 target 바이트 | 4 |
| 7 | 7 | obj_entity | Option<&Entity> | IR %6. None 이면 즉시 (None, MAX) | 4 |
| 8 | 8 | last_hp | usize | IR %7. goal_data.*.last_epic_hp — 마지막 관측 hp | 4 |
| 9 | 9 | enemy_tick | usize | IR %8. goal_data.*.epic_enemy_tick — last_seen 시점 '적이 처치까지 걸릴 예상 틱'(0=예측 없음). remaining = enemy_tick - (tick - last_seen) | 4 |
| 10 | 10 | last_seen | usize | IR %9. goal_data.*.last_epic_seen 틱 | 4 |
| 11 | 11 | world_pos | (usize, usize) | SROA 승격 — role %10 = x, %11 = y (캠프 존 좌표 288000 또는 672000, range 속성) | 4 |
| 12 | 12 | in_session | bool | IR %12. 이미 이 대상 스틸 세션 중인가 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn evaluate_steal_for_target(player, data, my_champ, target, obj_entity, last_hp, enemy_tick, last_seen, world_pos, in_session) -> (StealAction, usize)
  let Some(obj) = obj_entity else { return (None, MAX) }                 // L406
  if !in_session {
    if !(1 <= enemy_tick && enemy_tick <= tps*30) { return (None, MAX) }  // L414~415
    allies_at_camp = cache.player_champion[team].iter().flatten().filter(|c| distance_sq(c, world_pos) < 100000^2+1).count()   // L417 (steal.rs:267~268)
    if allies_at_camp > 1 { return (None, MAX) }                          // L418~419 (나 외 아군이 캠프에 있으면 팀 오브젝트 — 스틸 아님)
  } else if !(obj.hp*20 < obj.stat_cached.hp*19) { return (None, MAX) }  // L410~411 (세션 중인데 대상 hp>=95% 면 종료)
  // ===== L423: evaluate_v28_steal_for_target (steal.rs:447~530, 인라인) — 이후 None 도 .1 = enemy_tick =====
  tick = game.tick()
  elapsed = tick.saturating_sub(last_seen)
  current_hp_est = if enemy_tick==0 { last_hp } else if elapsed < enemy_tick { last_hp*(enemy_tick-elapsed)/enemy_tick } else { 0 }   // L449 estimate_current_hp(234~238): 선형 감소 추정
  execute_cut = max(player.parameter.last_hit_execute_hp(), setting.jungle_execute_threshold)   // L450 (227~229)
  move_speed = max(my.stat_cached.move_speed, 1)                                                  // L452
  early_range = steal_damage_and_range_within(ctx, my_champ, obj, tps*2).1 ; entry_range = max(early_range, 16000)   // L453~454
  arrival_ticks = distance(me, obj).saturating_sub(entry_range) / move_speed                     // L455~456
  (my_damage, damage_range) = steal_damage_and_range_within(ctx, my_champ, obj, arrival_ticks + tps)   // L457
  if my_damage==0 || damage_range==0 { return (None, enemy_tick) }                                // L459~460
  remaining_enemy_tick = enemy_tick.saturating_sub(elapsed)                                        // L463~464
  predicted_arrival_hp = if enemy_tick==0 { obj.hp } else if remaining > arrival { current_hp_est*(remaining-arrival)/remaining } else { 0 }   // L465~469
  obj_visible = obj.is_visible_from(my_team)  [visible_state[team]==Visible]; visible_hp = if obj_visible { obj.hp } else { current_hp_est }   // L472~473
  steal_damage = my_damage + execute_cut                                                          // L474
  vision_fresh = elapsed <= tps*5                                                                 // L475
  hp_ratio = my.hp*100/my.stat_cached.hp                                                          // L476
  threat_count = (1-team) 적 중 [visible_state[my_team]==Visible 인 것만] filter(|e| { r = max(steal_damage_and_range_within(ctx, e, my_champ, tps).1, 80000);
                   can_hit_me_soon = dist(e,me) <= r + e.move_speed*tps; controls_objective = dist(e,obj) <= r + 80000; can_hit_me_soon || controls_objective }).count()   // L477 (591~603)
  hp_ok_visible   = obj_visible && visible_hp <= steal_damage                                     // L478
  hp_ok_predicted = vision_fresh && predicted_arrival_hp <= steal_damage                          // L479
  in_wait_bush = { cands = steal_wait_bush(target, team) [Serpen: 1개 / Epic: 2개, 팀별 상수표]; bush = cands.min_by_key(|b| bush_distance_sq(b, me, map).unwrap_or(MAX));
                   map.bushes[cy][cx] == bush || bush_distance_sq(bush, me).is_some_and(|d| d < 36000^2+1) }   // L480 (28~45)
  structurally_viable = {                                                                         // L481 (540~583)
    cur_dur = (def+mr)*10 + my.hp ; full_dur = (def+mr)*10 + my.max_hp ; short_entry = damage_range < 110000 ; very_short = damage_range < 80000
    for e in 적 5슬롯(가시성 무관): (dmg, range) = steal_potential_damage_and_range(ctx, e, my_champ) [attack/skill/skill2/ult 기대피해 합·최대 사거리]
        ranged += range>=135000 ; long_range += range>=180000 ; if range+30000 >= damage_range { top3_entry.push(dmg) } ; top3_total.push(dmg)
    fragile = full_dur < 2300 || cur_dur*100 < full_dur*70 ; many_ranged = ranged>2 || long_range>1
    burst_entry_bad = sum(top3_entry)*100 >= cur_dur*70 ; burst_total_bad = sum(top3_total)*100 >= cur_dur*95
    if very_short && fragile && ranged>2 { false }
    else if short_entry && ((many_ranged && (burst_entry_bad || fragile)) || (burst_total_bad && ranged>1)) { false } else { true } }
  check_window = arrival <= tps*6 && remaining <= arrival + tps*2 && current_hp_est <= steal_damage*2 && in_wait_bush   // L482
  safe_enough = threat<2 || hp_ok_visible || (hp_ratio>69 && threat<3)                            // L486
  final_entry_safe = threat<3 || hp_ok_visible || hp_ratio>54                                     // L487
  visible_final_window = obj_visible && visible_hp <= steal_damage*2 && arrival <= tps*5          // L488
  predicted_final_window = vision_fresh && remaining <= arrival + tps*3 && current_hp_est <= steal_damage*2 && arrival <= tps*6   // L489~490
  final_entry_window = in_session && final_entry_safe && (hp_ok_visible || hp_ok_predicted || visible_final_window)   // L493~495
  if hp_ratio < 35 { return None }                                                                 // L497~498
  if final_entry_window { return Commit }                                                          // L506
  if in_session && final_entry_safe { if !(structurally_viable || predicted_final_window) { return None } ; if predicted_final_window { return Commit } }   // L501~506
  else if !(hp_ok_visible || structurally_viable) { return None }                                  // L501~502
  if !(in_wait_bush || hp_ok_visible) {                                                            // L509
    if in_session && obj_visible { if remaining > tps*2 && visible_hp*100 > obj.max*35 { return Lurk } else { return None } }   // L510~511
    return Lurk }                                                                                  // L513
  if safe_enough && (hp_ok_visible || hp_ok_predicted || check_window) { return Commit }           // L516~517
  finish_window = remaining <= tps*2 || (obj_visible && visible_hp*100 <= obj.max*35)              // L520 (IR 는 분기 접힘)
  if in_session { if finish_window && obj_visible { return None } else { return Lurk } }           // L521~522 / L529
  if enemy_tick > elapsed && remaining <= arrival { return None }                                   // L525~526 (도착 전에 적이 끝냄)
  return Lurk                                                                                        // L529
```

**`mem` 메모리 접근 30건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | team (L417 아군 슬롯 / L477 적 = 1-team) | 4 | OK |  |
| 1 | PlayerState | 0x180 | info.parameter | r | &AthleteParameter → last_hit_execute_hp() (L228 compute_my_execute_cut 인라인, +384) | 4 | OK |  |
| 2 | GameContext | 0x8 | setting | r |  | 4 | OK |  |
| 3 | GameContext | 0x20 | map | r | &MapDef → bushes 그리드·bush_distance_sq (+32, L480) | 4 | OK |  |
| 4 | GameSetting | 0x12f8 | tick_per_second | r | tps (+4856) | 4 | OK |  |
| 5 | GameSetting | 0x14a8 | jungle_execute_threshold | r | jungle_cut (+5288, L227) | 4 | OK |  |
| 6 | AbstractGameWithCache | 0x0 | game.data_ptr | r | vtable +0x28 tick (L448) | 4 | OK |  |
| 7 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r |  | 4 | OK |  |
| 8 | AbstractGameWithCache | 0x1e0 | player_champion | r | [team] 아군 5칸(L417 캠프 근처 아군 수) · [1-team] 적 5칸(L477 threat_count · L481 구조적 생존) | 4 | OK |  |
| 9 | MapDef | 0x1c98 | bushes | r | [30][30] usize — bushes[cy][cx] == bush 이면 in_wait_bush (+7320, steal.rs:44) | 4 | OK |  |
| 10 | Entity | 0x0 | team@tag | r | TeamType 0=Player/1=Neutral — my_champ.player_team() (entity.rs:1136). Player 면 obj/적 가시성 검사 경로 | 4 | OK |  |
| 11 | Entity | 0x8 | team@Player.0 | r | my_champ 의 팀 인덱스 (is_visible_from 인덱스, bounds 2) | 4 | OK |  |
| 12 | Entity | 0x38 | visible_state | r | [team] VisibleState 태그 0=Visible — obj.is_visible_from(team) (L473) · 적 가시 필터 (L591~) | 4 | OK |  |
| 13 | Entity | 0x670 | hp | r | obj.hp (L410·L469·L473) · my hp (L476·L540) | 4 | OK |  |
| 14 | Entity | 0x628 | stat_cached.hp | r | obj max (L410·L510·L520) · my max (L476·L541) | 4 | OK |  |
| 15 | Entity | 0x630 | stat_cached.defence | r | 내구도 = (defence+magic_resistance)*10 + hp (L540~541, +1584) | 4 | OK |  |
| 16 | Entity | 0x638 | stat_cached.magic_resistance | r | (+1592) | 4 | OK |  |
| 17 | Entity | 0x640 | stat_cached.move_speed | r | my max(move_speed,1) (L452) · 적 move_speed*tps (L598) | 4 | OK |  |
| 18 | Entity | 0x660 | x | r | (+1632) | 4 | OK |  |
| 19 | Entity | 0x668 | y | r | (+1640) | 4 | OK |  |
| 20 | Entity | 0x4c0 | attack_effect@tag | r | 적 잠재 피해 산정(steal.rs:106~127 steal_potential_damage_and_range): attack(+0x490)/skill(+0x4c8)/skill2/ult 이펙트 존재·CastingTarget::check 후 expected_damage_target 합·effect_total_range 최대 | 4 | OK |  |
| 21 | Entity | 0x4a0 | attack_effect@Some.0.range | r | effect_total_range = range + growth_range*(level-1) + stat_buff_cached.range + range_adjust + radius(radius_mult) (effect.rs:26·steal.rs:49·entity.rs:1512~1515) | 4 | OK |  |
| 22 | Entity | 0x4a8 | attack_effect@Some.0.growth_range | r |  | 4 | OK |  |
| 23 | Entity | 0x5c8 | level | r | growth_range*(level-1) | 4 | OK |  |
| 24 | Entity | 0x438 | stat_buff_cached.range | r |  | 4 | OK |  |
| 25 | Entity | 0x470 | stat_buff_cached.radius_mult | r | radius() = radius*(100+mult)/100 (entity.rs:1515) | 4 | OK |  |
| 26 | Entity | 0x680 | radius | r |  | 4 | OK |  |
| 27 | (sret) | 0x0 | StealAction.tag | w | None 저장 지점: L406·L411·L415·L419·L460·L498·L502·L511·L522·L526 / Lurk: L513·L529 / Commit: L506·L517 | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) | 0 None / 1 Lurk / 2 Commit |
| 28 | (sret) | 0x1 | StealAction.target | w | Lurk/Commit 일 때만 기록 | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) | target as u8 |
| 29 | (sret) | 0x8 | .1 (usize) | w | L406·L411·L415·L419 조기 None 은 MAX, 그 외(L460 이후 None 포함) enemy_tick (m07.ll:57628) | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) | -1 (usize::MAX) / enemy_tick |

**`consts` 상수 28건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev | folded_from |
|---|---|---|---|---|---|---|
| 0 | 30 | 414 | 계수 | tps*30: 세션 전엔 1 <= enemy_tick <= tps*30 이어야 함 (`enemy_tick-1 < tps*30` unsigned). 0(예측없음)·30초 초과 → None(L415) | 4 |  |
| 1 | 20 | 410 | 계수 | 세션 중: obj.hp*20 < obj.stat_cached.hp*19 (hp<95%) 아니면 None(L411) | 4 |  |
| 2 | 19 | 410 | 계수 | 95% 를 20:19 로 표현 | 4 |  |
| 3 | 10000000001 | 417 | 임계 | 100000^2+1: 캠프 존과 거리제곱 < 이면 '캠프에 있는 아군' (steal.rs:267~268 헬퍼 인라인) | 4 |  |
| 4 | 1 | 418 | 임계 | 캠프 근처 아군(나 포함) 수 > 1 이면 None(L419) — 우리 팀이 치는 중이면 스틸 아님. (본문 다른 1 = max(move_speed,1)·shl 1 = tps*2·steal_damage*2) | 4 |  |
| 5 | 16000 | 454 | 인덱스 | entry_range = max(early_range, 16000) (반셀) | 4 |  |
| 6 | 1 | 453 | 임계 | tps*2 — `shl i64 %149, 1`: steal_damage_and_range_within(…, tps*2) 의 early_range · L482/L490/L510/L520 remaining_enemy_tick vs tps*2 · L482 steal_damage*2(`shl %235, 1`) | 4 | 2 |
| 7 | 5 | 475 | 계수 | vision_fresh = elapsed(tick-last_seen) <= tps*5 · L488 arrival_ticks <= tps*5 | 4 |  |
| 8 | 100 | 476 | 계수 | hp_ratio = my hp*100/max (max 0 이면 div0 패닉 476:18) · L565~568 백분율 비교 | 4 |  |
| 9 | 80000 | 596 | 임계 | 적 위협(threat) 술어: range_to_me = max(range,80000); controls_objective = dist(적,obj) <= range+80000 (2.5셀) · L543 very_short_entry = damage_range < 80000 | 4 |  |
| 10 | 110000 | 542 | 임계 | short_entry = 내 damage_range < 110000 | 4 |  |
| 11 | 2300 | 565 | 임계 | full_durability < 2300 이면 fragile | 4 |  |
| 12 | 70 | 565 | 계수 | fragile = current_durability*100 < full_durability*70 · L567 burst_entry_bad = top3_entry*100 >= current*70 | 4 |  |
| 13 | 95 | 568 | 계수 | burst_total_bad = top3_total*100 >= current_durability*95 | 4 |  |
| 14 | 134999 | 551 | 임계 | 적 range > 134999 (>=135000) 이면 ranged_enemies++ | 4 |  |
| 15 | 179999 | 554 | 임계 | 적 range > 179999 (>=180000) 이면 long_range_enemies++ | 4 |  |
| 16 | 30000 | 557 | 계수 | 적 range + 30000 >= 내 damage_range 이면 entry 피해 top3 후보(진입 시 맞는 적) | 4 |  |
| 17 | 2 | 566 | 임계 | many_ranged = ranged_enemies > 2 \|\| long_range_enemies > 1 · L486 threat_count < 2 · L570 ranged>2 | 4 |  |
| 18 | 6 | 482 | 계수 | tps*6: check_window·predicted_final_window 의 arrival_ticks 상한 | 4 |  |
| 19 | 3 | 486 | 임계 | threat_count < 3 (L486 safe_enough 두 번째 항·L487 final_entry_safe) · L490 tps*3 | 4 |  |
| 20 | 69 | 486 | 임계 | safe_enough = threat<2 \|\| hp_ok_visible \|\| (hp_ratio > 69 && threat<3) | 4 |  |
| 21 | 54 | 487 | 임계 | final_entry_safe = threat<3 \|\| hp_ok_visible \|\| hp_ratio > 54 | 4 |  |
| 22 | 35 | 497 | 임계 | hp_ratio < 35 → None(L498) · L510/L520 visible_hp*100 > obj.max*35 (대상 35% 초과 남음) | 4 |  |
| 23 | 32000 | 42 | 계수 | in_steal_wait_bush: cx = (x/32000).clamp(0,29), cy 동일 — 셀 변환 | 4 |  |
| 24 | 29 | 42 | 임계 | clamp 상한 29 (30x30 그리드) | 4 |  |
| 25 | 1296000001 | 45 | 임계 | 36000^2+1: bush_distance_sq(bush, me) < 이면 대기 부시 안(약 1.1셀) | 4 |  |
| 26 | -1 | 36 | 센티널 | bush_distance_sq None → usize::MAX (min_by_key unwrap_or) · sret .1 조기 None 값 | 4 |  |
| 27 | 10 | 540 | 계수 | durability = (defence + magic_resistance)*10 + hp | 4 |  |

**`knobs` 조정점 10건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 적 예상 처치 예측 유효 상한 | steal.rs:414 | tps*30 | 올리면 오래 걸릴 처치도 스틸 후보 | 4 | 기존 |
| 1 | 캠프 아군 인원 상한 | steal.rs:418 | 1 | 올리면 아군이 같이 있어도 스틸 판정 진행 | 4 | 기존 |
| 2 | 내 hp 하한 | steal.rs:497 | 35 | hp% < 35 무조건 None | 4 | 기존 |
| 3 | 안전 판정 hp% | steal.rs:486 / :487 | 69 / 54 | 내리면 위협 적 2명 상황에서도 Commit(safe_enough)·최종 진입(final_entry_safe) 허용 | 4 | 기존 |
| 4 | 위협 인원 | steal.rs:486 / :487 | 2 / 3 | threat_count 상한. 올리면 더 많은 적 앞에서도 Commit | 4 | 기존 |
| 5 | Commit 창(hp 여유) | steal.rs:482 / :488 / :490 | steal_damage*2 | 현재/예측 hp 가 내 스틸딜의 2배 이내면 창 열림 — 배수를 키우면 더 이른 Commit | 4 | 기존 |
| 6 | 도착 시간 창 | steal.rs:482 / :488 / :490 | tps*6 / tps*5 / tps*3 | arrival_ticks 상한·remaining 여유. 줄이면 근접해서만 Commit | 4 | 기존 |
| 7 | 구조적 생존 임계 | steal.rs:542~568 | 110000 / 80000 / 2300 / 70% / 95% / 135000 / 180000 | 짧은 사거리·물몸·원거리 다수·top3 폭딜 대비 내구도 — 완화하면 물몸 정글러도 진입 | 4 | 기존 |
| 8 | 대기 부시 반경 | steal.rs:45 | 36000^2+1 | 부시 근접 판정 — Lurk→Commit 전환(check_window)에 관여 | 4 | 기존 |
| 9 | 대상 잔여 hp 포기선 | steal.rs:510 / :520 | 35% | 세션 중 대상 hp 가 35% 이하로 떨어졌는데 내가 못 끝내면 None(포기) | 4 | 기존 |

<details><summary>`callees` 피호출자 22건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | bush_distance_sq | game_ai::plan_legacy::steal::bush_distance_sq | in:game_ai::plan_legacy::steal | fn(usize, u64, u64, &game_core::MapDef) -> std::option::Option<u64> | game-ai\src\plan_legacy\steal.rs:20 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | check | game_core::CastingTarget::check | pub | fn(&game_core::CastingTarget, &game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\effect\type.rs:227 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 2 | dist | game_view::view::projectile::crossbowman::dist | in:game_view::view::projectile::crossbowman | fn(f32, f32, f32, f32) -> f32 | game-view\src\view\projectile\crossbowman.rs:1923 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 3 | distance | game_core::utils::distance | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:12 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | distance_sq | game_core::utils::distance_sq | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:6 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 5 | distance_sq | game_core::Entity::distance_sq | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2157 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 6 | estimate_current_hp | game_ai::plan_legacy::steal::estimate_current_hp | in:game_ai::plan_legacy::steal | fn(usize, usize, usize, usize) -> usize | game-ai\src\plan_legacy\steal.rs:233 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 7 | evaluate_steal_for_target | game_ai::plan_legacy::steal::evaluate_steal_for_target | in:game_ai::plan_legacy::steal | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan, &game_core::Entity, game_core::StealTarget, std::option::Option<&game_core::Entity>, usize, usize, usize, (u64, u64), bool) -> (game_ai::plan_legacy::steal::StealAction, usize) | game-ai\src\plan_legacy\steal.rs:392 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 8 | evaluate_v28_steal_for_target | game_ai::plan_legacy::steal::evaluate_v28_steal_for_target | in:game_ai::plan_legacy::steal | fn(&game_core::PlayerState, &game_core::OperationData, &game_core::Entity, game_core::StealTarget, &game_core::Entity, usize, usize, usize, bool) -> (game_ai::plan_legacy::steal::StealAction, usize) | game-ai\src\plan_legacy\steal.rs:436 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 9 | expected_damage_target | game_core::Effect::expected_damage_target | pub | fn(&game_core::Effect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + , &game_core::Entity) -> usize | game-core\src\simulation\effect.rs:91 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 10 | is_visible_from | game_core::Entity::is_visible_from | pub | fn(&game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1481 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 11 | last_hit_execute_hp | game_core::AthleteParameter::last_hit_execute_hp | pub | fn(&game_core::AthleteParameter) -> usize | game-core\src\simulation\state\player.rs:255 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 12 | push | <game_core::Staff as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::staff | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\staff.rs:38 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 13 | push | <game_core::Athlete as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:434 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 14 | push | <game_core::Contract as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:2498 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 15 | range_adjust | game_core::Effect::range_adjust | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:29 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 16 | steal_damage_and_range_within | game_ai::plan_legacy::steal::steal_damage_and_range_within | in:game_ai::plan_legacy::steal | fn(&game_core::GameContext, &game_core::Entity, &game_core::Entity, usize) -> (usize, u64) | game-ai\src\plan_legacy\steal.rs:52 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 17 | steal_potential_damage_and_range | game_ai::plan_legacy::steal::steal_potential_damage_and_range | in:game_ai::plan_legacy::steal | fn(&game_core::GameContext, &game_core::Entity, &game_core::Entity) -> (usize, u64) | game-ai\src\plan_legacy\steal.rs:98 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 18 | steal_wait_bush | game_ai::plan_legacy::steal::steal_wait_bush | pub | fn(game_core::StealTarget, usize, &game_core::Entity, &game_core::MapDef) -> usize | game-ai\src\plan_legacy\steal.rs:27 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 19 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 20 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 21 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
</details>

⚠**미매칭 1개**: `next  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 9개는 **전부 다른 함수**라 싣지 않는다`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 2곳** (m07.ll:54867, m07.ll:55072) · **형제 0개** 

**`open` 8건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | steal_damage_and_range_within(ctx, caster, target, within_ticks) 내부 미독해(별도 define, exe 0xd9d4d0 추정) — 이름·_docs('쿨 0인 공격/스킬/스킬2/궁 각각 expected_damage_target 1회씩 합산') 로 '주어진 틱 안에 쓸 수 있는 피해 합과 사거리' 로 읽음 | 5 |  |
| 1 | 미탐색 | bush_distance_sq(bush_id, x, y, map) 내부 미독해 — 반환 {i64 is_some, i64 dist_sq}. steal_wait_bush 후보 상수표(anon.184~187)의 실제 부시 id 값 미확인 | 4 |  |
| 2 | 미탐색 | steal_potential_damage_and_range(steal.rs:106~127)의 skill2/ult 분기 상세(entity.rs:1693 skill2_effect / 1701 ult_effect 의 slot>2 / slot>4 검사)는 요지만 적음 — 4 이펙트 각각 CastingTarget::check 후 expected_damage_target 합, range 는 max 로 읽었으나 L119/L126 의 합/최대 여부는 phi 만 봤음(미확정: 합산 대상이 damage, range 는 max 인지 sum 인지 — IR %730/%731 phi 로는 둘 다 누적형) | 4 |  |
| 3 | 표기 불가 | L520 finish_window 의 소스 표현 — IR 은 세 분기(%945/%946/%952)로 접혀 있어 `remaining<=tps*2 \|\| (visible && hp<=35%)` 로 복원했으나 항 순서·정확한 결합은 표기 불가 | 4 |  |
| 4 | 표기 불가 | L501/L505 의 소스 형태 — IR 은 final_entry_window 유무에 따라 두 경로(%907 / %915)로 갈라져 있어 `!(hp_ok_visible \|\| structurally_viable \|\| predicted_final_window)` 한 식인지 두 if 인지 표기 불가. 동작은 logic 대로 확정 | 4 |  |
| 5 | 미탐색 | threat_count 의 가시 필터: IR 은 my_champ.team 태그가 Player 인 경로(%361~)에서만 visible_state[team]==Visible 검사 — my_champ 는 항상 Player 이므로 실효 술어는 '가시 적만'. Neutral 경로(%259~)는 사장 코드 | 4 |  |
| 6 | 미탐색 | 구조적 생존 루프의 적 5슬롯은 가시성 필터 없음(Some 이면 전부) — threat_count 와 다름 | 4 |  |
| 7 | 미탐색 | exe 0x12857f0(1111B)·0x12a0180(757B)·vtable +0xe8 의 IR 짝(expected_damage_target / range_adjust / CastingTarget::check / last_hit_execute_hp) 배정은 추정 | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

