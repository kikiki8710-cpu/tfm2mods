---

### `49` resolve_join_stake — anchor(적) 주변 교전에 내가 6초 내 합류할 때의 '저울' — 아군만/아군+나 두 판을 resolve_fight_full 로 돌려 차분 FightPrediction 을 낸다

| 항목 | 값 |
|---|---|
| id | `fight_model__resolve_join_stake` |
| 심볼 | `_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old11fight_model18resolve_join_stake` |
| 소스 | `game-ai\src\plan_legacy\old\fight_model.rs:680` |
| IR | `m10.ll` 41001~42354행 |
| 경로·가시성 | `game_ai::plan_legacy::old::fight_model::resolve_join_stake` · **in:game_ai** |
| 계층 | 레거시 플랜 |
| exe | `e05e70` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r8` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &mut rand::rngs::std::StdRng, &game_core::OperationData, &game_core::PlayerState, &game_core::Entity, &game_core::Entity, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> std::option::Option<game_ai::plan_legacy::old::FightPrediction>
```

<details><summary>인자 9개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | &mut Option<FightPrediction>(64B) | 반환 슬롯. None = +0x0(focus_target 니치) 에 i64 -1 저장(41074·41118·41163) | 4 |
| 1 | 1 | version | usize | AI 버전. <2 면 즉시 None(41062~41063, L682). 이후 resolve_fight_full·ally_is_bound·is_enemy_well_danger(클로저) 에 전달 | 4 |
| 2 | 2 | rnd | &mut StdRng(320B, align16) | 본문에서 직접 안 씀. ally_is_bound 에만 전달(42085) | 4 |
| 3 | 3 | data | &OperationData(24B) | +0 cache(&AbstractGameWithCache) / +8 context(&GameContext) / +0x10 blackboard(&[Blackboard;2]) | 4 |
| 4 | 4 | player | &PlayerState(2528B) | 판단 주체. info.team(+0x930)·info.position 태그(+0x9c0)·info.parameter(+0x180, judge_accuracy) 를 읽는다 | 4 |
| 5 | 5 | champ | &Entity(1728B) | 판단 주체의 챔피언(나). dbg 이름 champ/caster | 4 |
| 6 | 6 | anchor | &Entity(1728B) | 합류 기준점이 되는 적 엔티티. 거리·사거리·near 판정의 중심 | 4 |
| 7 | 7 | team_plan | &TeamPlan(1064B) | ally_battle_stop_tick[i](+0x0, Option<usize> stride16) 만 읽는다 | 4 |
| 8 | 8 | debug | &mut DebugFrameData(224B) | 본문에서 직접 안 씀. ally_is_bound 에만 전달 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn resolve_join_stake(version, rnd, data, player, champ, anchor, team_plan, debug) -> Option<FightPrediction>

[L682] if version < 2 { return None }                      // 41062
[L685] horizon = data.context.setting.tick_per_second * 6    // 6초
[L688] cache = data.cache ; my_team = player.info.team(+0x930) ; enemy_team = 1 - my_team (≥2 면 panic_bounds_check)
[L687~691] enemies: Vec<&Entity,&Bump> = cache.player_champion[enemy_team] (iter_champions, Some 만)
      .filter(|e| {                                   // ★클로저0 = aux m10.ll 56030~56141 (Entity::call_mut 심)
         [L689] e.id == anchor.id                      // anchor 자신은 무조건 포함
              || ( distance_sq(e, anchor) < 22500000001   // ≤150000
         [L690]   && data.blackboard[enemy_team].is_recent_visible(cache.game, player, e)
         [L691]   && !is_ignored_well_enemy(version, player, e) )
                  //  is_ignored_well_enemy 인라인 = (e.team(+0x0) == TeamType::Player(enemy_team)) && path_finder::is_enemy_well_danger(version, player, e.x, e.y)
      }).collect_in(pool)
[L692~693] if enemies.is_empty() { return None }

[L696] allies: Vec<&Entity> = new_in(pool) ; [L697] arrivals: Vec<usize> = new_in(pool)
      nearest_bound: Option<&Entity> = None
      my_pos = player.info.position 태그(+0x9c0)
[L699] for i in 0..5 {
[L700]   if i == my_pos { continue }                          // 나 자신 슬롯 제외
[L701]   if team_plan.ally_battle_stop_tick[i].is_some() { continue }   // 태그(+0x0+16i) != 0
[L702]   a = cache.player_champion[my_team][i] ; if a.is_none() { continue }   // my_team≥2 면 panic
[L703]   bg = data.blackboard[my_team].big_goal[i].1
         focus_match = (bg 태그 == 5 /*Battle*/) && bg.focus.is_some()
[L704]                 && enemies.iter().any(|e| e.id == bg.focus.unwrap())
[L705]   bound = ally_is_bound(version, rnd, data, player, a, &enemies, debug)
[L706]   if !focus_match && !bound { continue }               // 둘 다 아니면 후보 아님
[L707]   if bound {                                           // (focus_match 만 참이면 nearest 갱신 없이 통과)
            nearest_bound = min_by_key((distance_sq(a,anchor), a.id)) — 기존 nb 와 비교:
              dist 같으면 a.id < nb.id, 아니면 dist_a < dist_nb 일 때 a 로 교체 (None 이면 a)
         }
[L710]   sp = max(a.move_speed(+0x640), 1)
[L711]   reach = a.attack_effect.map(|ef| ef.range(a)).unwrap_or(0)
              // Effect::range 인라인 = stat_buff_cached.range(+0x438) + range(+0x4a0) + (level(+0x5c8)-1)*growth_range(+0x4a8)
[L712]   allies.push(a)
[L713]   arrivals.push( min(horizon, sat_sub(Entity::distance(a, anchor), reach) / sp) )
      }
[L715] if allies.is_empty() { return None }

[L719] my_sp = max(champ.move_speed, 1)
[L720] my_reach = champ.attack_effect.map(|ef| ef.range(champ)).unwrap_or(0)   // 같은 식
[L721] my_arrival = sat_sub(Entity::distance(champ, anchor), my_reach) / my_sp
[L722] if my_arrival > horizon { return None }                 // ⚠아군은 umin 으로 잘리지만 나는 컷

[L726] tower = cache.iter_towers_without_nexus(enemy_team)      // Chain<Flatten<[Option<&Entity>;6]>, Copied<slice>> 120B
         .filter(|t| t.can_target(+0x6b9) && t.block_target_tick(+0x6a0) == 0)   // 클로저 s2_0 = aux 56144~56165 (배열부는 본문 인라인 41272~41330, 슬라이스부는 try_fold 심)
[L727]   .min_by_key(|t| distance_sq(t, anchor))               // 첫 원소 키는 본문 인라인(41341~41360), 나머지는 Map::fold 심(m06.ll 30568~30820)
[L728]   .filter(|t| {
[L729]      r = t.attack_effect.unwrap()/*None 이면 unwrap_failed*/ .range(t) + 15000 + anchor.radius() + t.radius()
                // Entity::radius = radius_mult(+0x470)==0 ? radius(+0x680) : radius*(100+mult)/100
[L730]      Entity::distance(t, anchor) <= r
         })                                                    // 아니면 tower = None
[L732] judge = AthleteParameter::judge_accuracy(&player.info.parameter(+0x180))

[L733] without  = resolve_fight_full(version, data, champ, &allies, &enemies, committed_dir=0, tower, judge, &arrivals, baseline=0)
[L734] with_me  = allies.clone() ; [L735] with_arr = arrivals.clone()
[L736] with_me.push(champ) ; [L737] with_arr.push(my_arrival)
[L738] absolute = resolve_fight_full(version, data, champ, &with_me, &enemies, 0, tower, judge, &with_arr, baseline=0)
[L739] diff     = resolve_fight_full(version, data, champ, &with_me, &enemies, 0, tower, judge, &with_arr, baseline=without.net_value(+0x30))
[L740] diff.line_absolute(+0x39) = absolute.line(+0x38)
[L741] diff.rescue_ally(+0x20) = nearest_bound.map(|a| a.id)
[L742] return Some(diff)

※ 부작용: 게임 구조체 쓰기 없음. rnd/debug 는 ally_is_bound 로 &mut 전달. resolve_fight_full 은 TLS ResolveFightCache 메모 래퍼(범위 밖)라 캐시 상태가 바뀔 수 있다.
```

**`mem` 메모리 접근 39건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | OperationData | 0x0 | cache | r | &AbstractGameWithCache — iter_champions·player_champion·iter_towers_without_nexus 출처(41064·41083~41085) | 4 | OK |  |
| 1 | OperationData | 0x8 | context | r | &GameContext — setting·pool | 4 | OK |  |
| 2 | OperationData | 0x10 | blackboard | r | &[Blackboard;2]. 클로저 캡처(적팀 판) + 본문 big_goal(아군팀 판) 인덱싱(41157) | 4 | OK |  |
| 3 | GameContext | 0x0 | pool | r | &bumpalo::Bump — enemies/allies/arrivals 세 Vec 의 할당자(41100) | 4 | OK |  |
| 4 | GameContext | 0x8 | setting | r | &GameSetting | 4 | OK |  |
| 5 | GameSetting | 0x12f8 | tick_per_second | r | IR 4856. horizon = tps*6 | 4 | OK |  |
| 6 | PlayerState | 0x930 | info.team | r | usize. enemy_team = 1 - team (범위밖이면 panic_bounds_check len=2). 클로저0 에서도 같은 필드로 적팀 블랙보드·TeamType 을 만든다 | 4 | OK |  |
| 7 | PlayerState | 0x9c0 | info.position@tag | r | i32 태그(Position: 0 Top/1 Jungle/2 Mid/3 Bottom/4 Support). 루프 i == 이 값이면 '나 자신' 슬롯이라 건너뛴다(41149·42010) | 4 | OK |  |
| 8 | PlayerState | 0x180 | info.parameter | r | &AthleteParameter(744B) 를 AthleteParameter::judge_accuracy 에 넘긴다(41636) | 4 | OK |  |
| 9 | AbstractGameWithCache | 0x0 | game.data_ptr (&dyn AbstractGame data+vtable) | r | 클로저0 캡처용(41089~41091) — is_recent_visible 인자 | 4 | OK |  |
| 10 | AbstractGameWithCache | 0x1e0 | player_champion | r | [[Option<&Entity>;5];2]. [enemy_team] 은 iter_champions(적 후보), [my_team][i] 는 아군 a(41154·42033) | 4 | OK |  |
| 11 | TeamPlan | 0x0 | ally_battle_stop_tick[] | r | stride 16, i64 태그 0=None/1=Some. Some 이면 그 아군은 후보에서 제외(42020~42022) | 4 | OK |  |
| 12 | Blackboard | 0xf0 | big_goal[i] | r | [(usize, Option<BigGoal>);5] stride 32. 본문은 원소 i 의 +0x8(=Blackboard+0xf8+32i) 태그 i8, +0x10 focus 태그, +0x18 focus 값을 읽는다(42046~42058) | 4 | OK |  |
| 13 | Blackboard | 0xf8 | big_goal[i].1@tag | r | BigGoal 태그 1B. ==5 → Battle | 4 | OK |  |
| 14 | Blackboard | 0x100 | big_goal[i].1@Battle.focus@tag | r | Option<usize> 태그(i64 trunc→i1). Some 일 때만 아래 focus 값을 enemies 와 대조 | 4 | OK |  |
| 15 | Blackboard | 0x108 | big_goal[i].1@Battle.focus@Some.0 | r | usize entity id. enemies.iter().any(\|e\| e.id == 이 값) | 4 | OK |  |
| 16 | Entity | 0x5c0 | id | r | usize. 적 id 대조(42080)·nearest_bound 동률 타이브레이크(42140~42144)·rescue_ally 출력(41929). 클로저0 에서는 e.id==anchor.id 단락 | 4 | OK |  |
| 17 | Entity | 0x640 | stat_cached.move_speed | r | usize. max(_,1) 로 0 나눗셈 방지 후 도착시간 분모(41228·42148) | 4 | OK |  |
| 18 | Entity | 0x4c0 | attack_effect@tag (casting 니치 i32) | r | == -1 이면 attack_effect None → reach 0 (41232~41234·42152·41508). 타워 경로에선 None 이면 unwrap_failed 패닉(41525) | 4 | OK |  |
| 19 | Entity | 0x4a0 | attack_effect.range | r | u64. reach/사거리 식의 기본항 | 4 | OK |  |
| 20 | Entity | 0x4a8 | attack_effect.growth_range | r | u64. (level-1)*growth_range | 4 | OK |  |
| 21 | Entity | 0x5c8 | level | r | usize. (level-1)*growth_range — level 0 이면 -1 곱(랩어라운드, 가드 없음) | 4 | OK |  |
| 22 | Entity | 0x438 | stat_buff_cached.range | r | usize. reach 가산항 | 4 | OK |  |
| 23 | Entity | 0x660 | x | r | u64. distance_sq(인라인 abs_diff²합)·Entity::distance 의 재료 | 4 | OK |  |
| 24 | Entity | 0x668 | y | r | u64 | 4 | OK |  |
| 25 | Entity | 0x470 | stat_buff_cached.radius_mult | r | i32. 0 이면 radius 그대로, 아니면 radius*(100+mult)/100 (entity.rs Entity::radius 인라인, 41532~41556) | 4 | OK |  |
| 26 | Entity | 0x680 | radius | r | usize. 타워 사거리 판정에 anchor.radius()+tower.radius() 가산 | 4 | OK |  |
| 27 | Entity | 0x6b9 | can_target | r | bool. 타워 후보 필터 1(41290~41293, 클로저 s2_0 56149) | 4 | OK |  |
| 28 | Entity | 0x6a0 | block_target_tick | r | usize. ==0 이어야 타워 후보(41294~41297) | 4 | OK |  |
| 29 | Vec<&Entity>(bumpalo,32B) | 0x0 | ptr | r | enemies/allies 버퍼 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 30 | Vec<&Entity>(bumpalo,32B) | 0x10 | cap | r | push 시 len==cap 이면 reserve_internal_or_panic | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 31 | Vec<&Entity>(bumpalo,32B) | 0x18 | len | r | enemies.len()==0 → None(41111~41114), allies.len()==0 → None(41197~41199) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 32 | FightPrediction(without) | 0x30 | net_value | r | i64. 세 번째 resolve_fight_full 의 baseline 인자(41904~41905) | 4 | OK |  |
| 33 | FightPrediction(absolute) | 0x38 | line | r | FightLine 1B(0 Commit/1 CommitAfterJoin/2 Disengage/3 Hold). diff.line_absolute 로 복사 | 4 | OK |  |
| 34 | Option<FightPrediction>(sret) | 0x0 | focus_target@tag | w | 4개 None 반환 지점(version<2 / enemies 비었음 / allies 비었음 / my_arrival>horizon) | 4 | 확인불가(tcx 사전에 타입 없음) | -1 (None) |
| 35 | FightPrediction(diff→sret) | 0x39 | line_absolute | w | L740 (41911~41914) | 4 | OK | absolute.line |
| 36 | FightPrediction(diff→sret) | 0x20 | rescue_ally@tag | w | L741 (41925~41928) | 4 | OK | 1 if nearest_bound.is_some() else 0 |
| 37 | FightPrediction(diff→sret) | 0x28 | rescue_ally@Some.0 | w | None 이면 undef 채움 | 4 | OK | nearest_bound.id(+0x5c0) |
| 38 | Option<FightPrediction>(sret) | 0x0 | (전체 64B) | w | L742 (41930). ⚠게임 구조체(Entity/PlayerState/TeamPlan/Blackboard) 에 쓰는 곳은 없다 — 스택·bump Vec 뿐 | 4 | 확인불가(tcx 사전에 타입 없음) | memcpy diff |

**`consts` 상수 11건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 2 | 682 | 임계 | version < 2 면 None. v2 이상 전용 판정 | 4 |
| 1 | 6 | 685 | 임계 | horizon = tick_per_second * 6 = 6초. 아군 도착시간 상한(umin)·내 도착시간 컷 | 4 |
| 2 | 1 | 688 | 인덱스 | enemy_team = 1 - player.info.team (아군팀 인덱스는 info.team 그대로) | 4 |
| 3 | 5 | 703 | 태그 | BigGoal 메모리태그 5 = Battle. 아군 i 의 big_goal 이 Battle 이고 focus 가 Some 일 때만 '같은 적을 보고 있나' 대조 | 4 |
| 4 | 1 | 710 | 인덱스 | sp = max(move_speed, 1) — 0 나눗셈 방지(llvm.umax). L719 의 my_sp 도 동일 | 4 |
| 5 | -1 | 711 | 센티널 | (a) attack_effect 니치 None 표식(Entity+0x4c0 == -1 → reach 0) (b) (level-1) 의 -1 (c) 반환 None 의 focus_target 니치 -1 | 4 |
| 6 | 15000 | 729 | 오프셋가감 | 타워 사거리 판정 여유분: distance(tower, anchor) <= tower.attack_effect.range + 15000 + stat_buff.range + (level-1)*growth + anchor.radius() + tower.radius(). 셀(32000)의 절반 ≈ 0.47칸. ⚠IR !dbg 는 effect.rs:26(Effect::range) 을 가리키나 챔피언 reach(L711/720) 경로엔 이 항이 없다 — 귀속 줄은 unknown 참조 | 4 |
| 7 | 100 | 729 | 계수 | Entity::radius 인라인(entity.rs:1511~1515): radius_mult != 0 이면 radius*(100+mult)/100. anchor 와 tower 양쪽에 적용 | 4 |
| 8 | 0 | 733 | 태그 | resolve_fight_full 의 committed_dir(i8)=0 (세 호출 모두) 과 baseline=0 (without·absolute 두 호출). diff 호출만 baseline=without.net_value | 4 |
| 9 | 1 | 741 | 태그 | rescue_ally Option<usize> Some 태그(=1). nearest_bound 가 있으면 1, 없으면 0 | 4 |
| 10 | 22500000001 | 689 | 미상 | ★aux(클로저0, m10.ll 56086): 150000² + 1 — 적 e 가 anchor 로부터 dist² < 이 값(즉 ≤150000, 약 4.7칸) 이어야 enemies 후보. e.id==anchor.id 면 거리 무관 포함 | 4 |

**`knobs` 조정점 5건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 합류 시간 지평(horizon) | fight_model.rs:685 (IR m10.ll:41058 `mul i64 %30, 6`) | 6 | tps*6 = 6초. 올리면 더 먼 아군도 도착 목록에 들고 나도 더 멀리서 합류 판정을 받는다(아군 arrival 은 이 값으로 umin 캡). 내리면 먼 아군/먼 나는 None 으로 잘려 '저울' 자체가 안 선다 | 4 | 기존 |
| 1 | version 게이트 | fight_model.rs:682 (IR 41062 `icmp ult i64 %1, 2`) | 2 | v<2 는 항상 None. 이 함수를 v1 에서도 켜려면 여기를 내린다 | 4 | 기존 |
| 2 | 적 후보 반경 | fight_model.rs:689 — aux 클로저0 (IR m10.ll:56086 `icmp ult i64 %35, 22500000001`) | 22500000001 | anchor 반경 150000(≈4.7칸) 안의 '최근 시야에 잡힌' 적만 enemies. 올리면 더 넓은 범위의 적이 저울에 들어 교전이 더 불리하게(또는 유리하게) 계산된다 | 4 | 기존 |
| 3 | 타워 사거리 여유분 | fight_model.rs:729 (IR m10.ll:41645 `add i64 %207, 15000`) | 15000 | anchor 가 가장 가까운 적 타워 사거리+15000+반지름 합 안이면 그 타워가 교전 cascade 행위자로 resolve_fight_full 에 들어간다. 올리면 타워를 더 자주 계산에 넣어 합류 판정이 보수적으로, 0 이면 실제 사거리 경계에서만 | 4 | 기존 |
| 4 | 아군 후보 조건(교전 중 판정) | fight_model.rs:703~706 (IR 42050 `icmp eq i8 %381, 5`) | 5 | big_goal 이 Battle(태그5)이고 focus 가 enemies 안에 있거나, ally_is_bound 가 참인 아군만 allies 에 든다. Battle 조건을 지우면 '묶인' 아군만 남아 with_me 판이 작아진다 | 4 | 기존 |

<details><summary>`callees` 피호출자 29건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | ally_is_bound | game_ai::plan_legacy::old::fight_model::ally_is_bound | in:game_ai | fn(usize, &mut rand::rngs::std::StdRng, &game_core::OperationData, &game_core::PlayerState, &game_core::Entity, &[&game_core::Entity], &mut game_core::DebugFrameData) -> bool | game-ai\src\plan_legacy\old\fight_model.rs:531 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | can_target | game_core::Entity::can_target | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1477 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
| 2 | can_target | game_core::OperationData::<'a, 'b>::can_target | pub | fn(&game_core::OperationData<'a/#0, 'b/#1>, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\state\player.rs:23 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
| 3 | distance | game_core::Entity::distance | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2161 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | distance_sq | game_core::utils::distance_sq | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:6 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
| 5 | distance_sq | game_core::Entity::distance_sq | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2157 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
| 6 | is_empty | game_core::PatchSetting::is_empty | pub | fn(&game_core::PatchSetting) -> bool | game-core\src\patch_engine.rs:853 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 7 | is_empty | game_core::ModAiRegistry::is_empty | pub | fn(&game_core::ModAiRegistry) -> bool | game-core\src\mod_ai.rs:15 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 8 | is_empty | game_core::PublicChampionEvidence::is_empty | pub | fn(&game_core::PublicChampionEvidence) -> bool | game-core\src\banpick\public_evidence.rs:96 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 9 | is_enemy_well_danger | game_ai::is_enemy_well_danger | pub | fn(usize, &game_core::PlayerState, u64, u64) -> bool | game-ai\src\path_finder.rs:1032 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 10 | is_ignored_well_enemy | game_ai::plan_legacy::old::is_ignored_well_enemy | pub | fn(usize, &game_core::PlayerState, &game_core::Entity) -> bool | game-ai\src\plan_legacy\old\fight_model.rs:754 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 11 | is_recent_visible | game_core::Blackboard::is_recent_visible | pub | fn(&game_core::Blackboard, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\game\blackboard.rs:346 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 12 | iter_towers_without_nexus | game_core::AbstractGameWithCache::<'a, 'b>::iter_towers_without_nexus | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5750 ~ game_core[6a30]::simulation::{impl#5}::iter_towers_without_nexus::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1830 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 13 | judge_accuracy | game_core::AthleteParameter::judge_accuracy | pub | fn(&game_core::AthleteParameter) -> usize | game-core\src\simulation\state\player.rs:337 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 14 | level | <game_ai::WindowStatView<'_> as game_core::AbstractEntity>::level | pub | fn(&game_ai::WindowStatView</#0>) -> usize | game-ai\src\fight_check.rs:96 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 15 | level | game_core::AbstractEntity::level | pub | fn(&Self/#0) -> usize | game-core\src\simulation\entity.rs:718 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 16 | level | <game_core::Entity as game_core::AbstractEntity>::level | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:772 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 17 | line | game_ai::plan_legacy::types::BigPlan::debug_label::line | in:game_ai::plan_legacy::types | fn(game_core::LineType) -> &str | game-ai\src\plan_legacy\types.rs:83 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
| 18 | line | game_core::TowerType::line | pub | fn(&game_core::TowerType) -> std::option::Option<game_core::LineType> | game-core\src\simulation\entity\tower.rs:90 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
| 19 | push | <game_core::Staff as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::staff | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\staff.rs:38 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 6개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 20 | push | <game_core::Athlete as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:434 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 6개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 21 | push | <game_core::Contract as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:2498 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 6개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 22 | radius | game_core::Entity::radius | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1509 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 23 | range | game_core::Effect::range | pub | fn(&game_core::Effect, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:25 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 24 | resolve_fight_full | game_ai::plan_legacy::old::fight_model::resolve_fight_full | in:game_ai::plan_legacy::old::fight_model | fn(usize, &game_core::OperationData, &game_core::Entity, &[&game_core::Entity], &[&game_core::Entity], i8, std::option::Option<&game_core::Entity>, usize, &[i64], i64) -> game_ai::plan_legacy::old::FightPrediction | game-ai\src\plan_legacy\old\fight_model.rs:324 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 25 | resolve_join_stake | game_ai::plan_legacy::old::fight_model::resolve_join_stake | in:game_ai | fn(usize, &mut rand::rngs::std::StdRng, &game_core::OperationData, &game_core::PlayerState, &game_core::Entity, &game_core::Entity, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> std::option::Option<game_ai::plan_legacy::old::FightPrediction> | game-ai\src\plan_legacy\old\fight_model.rs:680 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 26 | team | game_view::ClientData::team | pub | fn(&game_view::ClientData, usize) -> std::option::Option<std::cell::Ref< game_core::Team>> | game-view\src\logic\client\data.rs:1651 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 27 | team | game_view::ClientDatabase::team | pub | fn(&game_view::ClientDatabase, usize) -> std::option::Option<&game_core::Team> | game-view\src\logic\client\data.rs:968 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 28 | team | game_core::PlayerAiContext::<'a, 'b, 'r>::team | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:583 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
</details>

⚠**미매칭 11개**: `block_target_tick`, `drop  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `line_absolute`, `move_speed`, `my_team`, `net_value`, `parameter`, `rescue_ally`, `reserve_internal_or_panic`, `sat_sub`, `try_fold`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 2곳** (m13.ll:30345, m13.ll:37717) · **형제 0개** 

**`open` 9건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | 15000 의 귀속 소스 줄: IR !dbg 는 effect.rs:26(Effect::range, inlinedAt fight_model.rs:729) 을 가리키지만, 같은 Effect::range 가 인라인된 L711/L720(챔피언 reach) 경로엔 +15000 항이 없다. LLVM 재결합(reassociation)으로 L729 의 리터럴이 L26 위치를 물려받았을 가능성이 크나 IR 만으로는 'Effect::range 내부의 조건항'인지 'L729 의 명시 가산'인지 확정 못 함(Effect::range 는 _gaibc/_gcbc 어디에도 define 없음 — 전부 인라인). 동작(타워 경로에만 +15000)은 확정 | 4 |  |
| 1 | 미탐색 | resolve_fight_full 내부(m10.ll 39915~, TLS ResolveFightCache 메모 래퍼)는 범위 밖 — 인자 계약만: (sret 64B, version, data, champ, near_allies ptr/len, near_enemies ptr/len, committed_dir i8, tower Option<&Entity>, judge_accuracy usize, arrivals ptr/len, baseline i64). arg 4·5·9 의 DWARF 이름은 DISubprogram 에 없고 호출측 dbg_value(near_allies/near_enemies/arrivals) 로 잡음 | 3 |  |
| 2 | 미탐색 | ally_is_bound(version, rnd, data, player, a, enemies slice, debug) 내부는 안 봄. 반환 bool 이 '아군 a 가 교전에 묶여 있다'는 뜻이라는 것은 변수명(bound/nearest_bound/rescue_ally) 기반 추정 | 5 |  |
| 3 | 미탐색 | Blackboard::is_recent_visible(&Blackboard[enemy_team], game data, game vtable, player, e) 의 의미('적팀 판'이 무엇을 기록하는지)는 game_core 본문 미열람. 인덱스가 적팀(1-team) 인 것만 확정 | 4 |  |
| 4 | 미탐색 | min_by_key 의 나머지 원소 처리(Map::fold 심, m06.ll 30568~30820)는 본체가 아니라 aux 로 넣지 않았다. 안을 훑어 x/y(1632/1640)·can_target·block_target_tick 만 읽는 것은 확인(키 = distance_sq to anchor, 동률이면 min_by 규칙상 앞 원소 유지) | 4 |  |
| 5 | 미탐색 | iter_towers_without_nexus 의 반환 120B 이터레이터 내부 레이아웃(+0 배열 IntoIter 상태 i64 -1 = 배열부 소진, +8/+16 alive 범위, +24 [Option<&Entity>;6], +104 슬라이스 ptr) 은 본문 사용 패턴으로 읽은 것 — 타워 6개 배열 + 추가 슬라이스라는 구조 자체는 IR 타입(`IntoIter<Option<&Entity>,6>` + `Copied<slice::Iter>`) 으로 확정 | 4 |  |
| 6 | 미탐색 | Position 태그(+0x9c0) 를 슬롯 인덱스 i 와 직접 비교하는 것은 'player_champion[team][i] 의 i = Position 순서(Top0 Jungle1 Mid2 Bottom3 Support4)' 라는 게임 관례에 의존 — 본문에서 i 가 그 순서라는 별도 근거는 없음(구조 관례 추정) | 5 |  |
| 7 | 미탐색 | level==0 인 엔티티의 (level-1)*growth_range 는 랩어라운드(u64) — 게임에서 level 이 0 이 되는지는 미확인 | 4 |  |
| 8 | 미탐색 | constants 의 `1`(L688/L710/L741) 과 `0`(L733) 은 여러 자리에서 겹치는 값이라 대표 줄만 적었다. 배열 상한 5·타워 6·stride 16/32/40 은 규칙대로 제외 | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

