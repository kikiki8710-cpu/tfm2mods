---

### `70` tower_dive_is_viable — 타워 다이브 실행 가능 판정 — 타워 포함 내 사망틱 vs 대상 사망틱(+탈출여유) 개인 레이스, 실패 시 팀모델(resolve_fight_stake) 로 재판정

| 항목 | 값 |
|---|---|
| id | `fight_model__tower_dive_is_viable` |
| 심볼 | `_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old11fight_model20tower_dive_is_viable` |
| 소스 | `game-ai\src\plan_legacy\old\fight_model.rs:935` |
| IR | `m10.ll` 42838~43820행 |
| 경로·가시성 | `game_ai::plan_legacy::old::tower_dive_is_viable` · **pub** |
| 계층 | 레거시 플랜 |
| exe | `e07430` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r9` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan, &game_core::Entity, bool, &mut game_core::DebugFrameData) -> bool
```

<details><summary>인자 8개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | version | usize | AI 버전. `>1` 이면 우물 ETA 게이트·exit_margin 계산(L989). check_kill_die_tick·resolve_fight_stake·closure#0 에도 전달 | 4 |
| 1 | 2 | rnd | &mut StdRng | check_kill_die_tick·resolve_fight_stake 로 전달만 | 4 |
| 2 | 3 | player | &PlayerState(2528B) | info.team(+0x930)·info.position(+0x9c0)·info.parameter(+0x180) | 4 |
| 3 | 4 | data | &OperationData(24B) | +0x0 cache(&AbstractGameWithCache) / +0x8 context / +0x10 blackboard[2] | 4 |
| 4 | 5 | team_plan | &TeamPlan(1064B) | ally_battle_stop_tick[5]: Option<usize> (+0x0, stride 16B) 만 읽음 — closure s_0(L960) | 4 |
| 5 | 6 | target | &Entity(1728B) | 다이브 대상 적 챔피언 | 4 |
| 6 | 7 | team_model | bool | false 면 개인 레이스 결과만 반환. true 면 개인 레이스 실패 시 팀모델(resolve_fight_stake) 로 재판정(L1015) | 4 |
| 7 | 8 | debug | &mut DebugFrameData | check_kill_die_tick·resolve_fight_stake 로 전달만 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn tower_dive_is_viable(version, rnd, player, data, team_plan, target, team_model, debug) -> bool
// L945
let champ = cache.player_champion[player.team][player.position] else { return false };
let tps = data.context.setting.tick_per_second;          // L948
let enemy_team = 1 - player.team;                          // L951

// L951~957 near_enemies (closure#0, aux m10:56215~): 적 챔피언 e 중
//   r = max(max_range_cached(data,champ,e), max_range_cached(data,e,champ));
//   dist_sq(e,champ) <= (r+30000)^2                                  (L953, ugt 면 탈락)
//   && blackboard[1-player.team].is_recent_visible(game, ctx, player, e)   (L954)
//   && !is_ignored_well_enemy(version, player, data, e, with_declared_dive=true)  (L955, 인라인 899~903)
//        v>=2: ignored = (e.team==Team::Player(enemy_team)) && is_enemy_well_danger(version, player, e.x, e.y)
//        v<=1: ignored = ((e.team==Player(enemy_team)) && is_enemy_well_danger(..)) || is_unreasonable_tower_dive_enemy(_, player, data, e, true)
// L959~962 near_allies (s_0/s0_0, aux m01:25361~): pos in 0..5 중
//   team_plan.ally_battle_stop_tick[pos].is_none() && player_champion[my][pos]=Some(c) && dist_sq(c,champ) < 120000^2+1  → c  (champ 자신 포함 가능)
// L969~972 nearest_enemy_tower =
//   cache.iter_towers_without_nexus(enemy_team).min_by_key(|t| dist_sq(t,target))   // s1_0 인라인
//     .filter(|t| t.distance(target) <= Effect::range(t.attack_effect.unwrap(), caster=t, target))   // L970~972 · ugt 면 None
//   where range = ae.range + 15000 + t.stat_buff.range + (t.level-1)*ae.growth_range + target.radius() + t.radius()

// L976~979
let die_tick_with_tower = check_kill_die_tick(version, rnd, data, player, champ, near_enemies.clone(), nearest_enemy_tower.into_iter().collect(), debug);
let target_die_tick     = check_kill_die_tick(version, rnd, data, player, target, near_allies.clone(), vec![], debug);

// L989~1003
let exit_margin = if version > 1 {
    let well = if player.team==1 {(0,960000)} else {(960000,0)};        // 적 우물 (L990)
    let ts = max(target.move_speed, 1);                                // L991
    let well_eta = distance(target.x,target.y, well).saturating_sub(160000) / ts;   // L992
    if target_die_tick > well_eta { return false; }                    // L993 대상이 죽기 전에 우물 도착
    match nearest_enemy_tower {                                        // L1000~1003
      Some(t) => { let reach = Effect::range(t.attack_effect.unwrap(), t, champ);   // L1001 (같은 공식, target→champ)
                   let exit_dist = reach.saturating_sub(t.distance(target));       // L1002
                   exit_dist / max(champ.move_speed, 1) }                          // L1003
      None => 0 }
} else { 0 };

// L1008
let individual_race = target_die_tick.saturating_add(exit_margin + tps/2) < die_tick_with_tower;
// L1015
if !team_model || individual_race { return individual_race; }

// 팀모델 (team_model && !individual_race)
// L1016~1018 dive_allies (s5_0/s6_0): pos in 0..5 → player_champion[my][pos] 중 dist_sq(a,target) < 200000^2+1
// L1019~1022 dive_enemies (s7_0, aux m10:56457~): 적 챔피언 e 중 e.id==target.id || (dist_sq(e,target) < 150000^2+1 && blackboard[1-my].is_recent_visible(game,ctx,player,e))
// L1024~1025
let pred: FightPrediction = resolve_fight_stake(version, rnd, data, player, champ, &dive_allies, &dive_enemies, 0u8, nearest_enemy_tower, judge_accuracy(&player.info.parameter), debug);
let mut line = pred.line;                                              // L1028 (+0x38)
if let Some(aid) = pred.rescue_ally {                                  // L1029 (+0x20/+0x28)
    let far = match game.get_entity_by_id(aid) {                       // L1030 vtable+0x1f0
        None => true,
        Some(a) => { let r = max(max_range_cached(data,target,a), max_range_cached(data,a,target));   // L1031
                     dist_sq(target,a) > (r+30000)^2 }                                              // L1032 (dbg 이름 'helping' 이 이 값에 붙어 있음)
    };
    if far { line = pred.line_absolute; }                              // L1035 (+0x39)
}
return line != FightLine::Disengage;                                   // L1041 (tag 2)
```

**`mem` 메모리 접근 29건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | L945 · 배열 인덱스(<2 bounds check) · enemy_team = 1 - team (L951) | 4 | OK |
| 1 | PlayerState | 0x9c0 | info.position@tag | r | L945 · i32 → player_champion[team][position] | 4 | OK |
| 2 | PlayerState | 0x180 | info.parameter | r | L1025 · AthleteParameter::judge_accuracy(&player.info.parameter) 인자 | 4 | OK |
| 3 | OperationData | 0x0 | cache | r | &AbstractGameWithCache | 4 | OK |
| 4 | OperationData | 0x8 | context | r | L948 · context.setting.tick_per_second | 4 | OK |
| 5 | OperationData | 0x10 | blackboard | r | closure#0/s7_0 · blackboard[1 - player.team] 로 is_recent_visible 호출 | 4 | OK |
| 6 | AbstractGameWithCache | 0x0 | game.data_ptr (dyn AbstractGame data ptr) | r | L1030 · get_entity_by_id 호출 self / s7_0 is_recent_visible 인자 | 4 | OK |
| 7 | AbstractGameWithCache | 0x8 | game.vtable_ptr vtable | r | L1030 · vtable+0x1f0 = get_entity_by_id (divtable 확인) | 3 | OK |
| 8 | AbstractGameWithCache | 0x1e0 | player_champion[2][5]: Option<&Entity> | r | L945 champ=[team][pos] · L951/1019 iter_champions(enemy_team) 슬라이스 · s0_0/s_0/s5_0 [my_team][pos] | 4 | OK |
| 9 | GameContext | 0x8 | setting | r | L948 | 4 | OK |
| 10 | GameSetting | 0x12f8 | tick_per_second | r | L948 tps · L1008 tps/2 여유 | 4 | OK |
| 11 | TeamPlan | 0x0 | ally_battle_stop_tick[] | r | closure s_0 L960 · is_none() (tag==0) 인 포지션만 near_allies 후보 | 4 | OK |
| 12 | Entity | 0x0 | team@tag | r | closure#0 (is_ignored_well_enemy 인라인 L755) · tag 0 = Team::Player | 4 | OK |
| 13 | Entity | 0x8 | team@Player.0 | r | closure#0 · == enemy_team(1-player.team) 확인 | 4 | OK |
| 14 | Entity | 0x660 | x | r | 거리 제곱 계산 전반(L953/961/969/1018/1021/1032/992) | 4 | OK |
| 15 | Entity | 0x668 | y | r | 동상 | 4 | OK |
| 16 | Entity | 0x5c0 | id | r | closure s7_0 L1021 · e.id == target.id 면 무조건 포함 | 4 | OK |
| 17 | Entity | 0x640 | stat_cached.move_speed | r | L991 target 속도(well_eta) · L1003 champ 속도(exit_margin) · max(·,1) | 4 | OK |
| 18 | Entity | 0x4c0 | attack_effect@tag(Niche) | r | L971/L1001 인라인 Effect::range · -1 이면 None → unwrap 패닉(L1013) | 4 | OK |
| 19 | Entity | 0x4a0 | attack_effect.range | r | L971/L1001 타워 사거리 기본값 (effect.rs:26 인라인) | 4 | OK |
| 20 | Entity | 0x4a8 | attack_effect.growth_range | r | × (level-1) | 4 | OK |
| 21 | Entity | 0x5c8 | level | r | (level-1)*growth_range | 4 | OK |
| 22 | Entity | 0x438 | stat_buff_cached.range | r | 사거리 가산 | 4 | OK |
| 23 | Entity | 0x470 | stat_buff_cached.radius_mult | r | Entity::radius() 인라인(entity.rs:1511~1515) · 0 이면 radius 그대로, 아니면 radius*(100+mult)/100 | 4 | OK |
| 24 | Entity | 0x680 | radius | r | Effect::range 에 caster.radius()+target.radius() 가산 | 4 | OK |
| 25 | FightPrediction | 0x20 | rescue_ally@tag | r | L1029 · Some 이면 구조 아군 실재/거리 검사 | 4 | OK |
| 26 | FightPrediction | 0x28 | rescue_ally@Some.0 (aid) | r | L1029~1030 · get_entity_by_id(aid) | 4 | OK |
| 27 | FightPrediction | 0x38 | line (FightLine) | r | L1028 · 기본 판정선 | 4 | OK |
| 28 | FightPrediction | 0x39 | line_absolute (FightLine) | r | L1035 · 구조 아군이 없거나 멀면 이 값으로 대체 | 4 | OK |

**`consts` 상수 14건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev | folded_from |
|---|---|---|---|---|---|---|
| 0 | 2 | 945 | 태그 | team 인덱스 bounds(팀 2개) — 판정 아님. 또한 L1041 FightLine::Disengage 메모리태그 2 (tcxdict --enum FightLine) | 3 |  |
| 1 | 5 | 959 | 산출값 | 포지션 수(0..5 range 상한 / player_champion 슬라이스 len) — 판정 아님 | 4 |  |
| 2 | 1 | 951 | 임계 | enemy_team = 1 - team (L951) · version > 1 게이트(L989) · max(speed,1) 0나눗셈 방지(L991/1003) · level-1(L971) · tps>>1 | 4 |  |
| 3 | 1 | 1008 | 임계 | tps/2 (=0.5초 여유). `lshr i64 %tps, 1` 로 접힘 — individual_race 우변 가산 | 4 | 2 |
| 4 | 160000 | 992 | 계수 | 우물 도달 판정 여유 거리(5셀). well_eta = (dist(target, 적우물) -sat 160000) / max(target.speed,1) | 4 |  |
| 5 | 960000 | 990 | 산출값 | 적 우물 좌표(30셀): my_team==1 이면 (0,960000), my_team==0 이면 (960000,0) — 팀별 우물 위치 헬퍼 인라인(L1016 of 다른 파일) | 4 |  |
| 6 | 15000 | 971 | 계수 | Effect::range 인라인(effect.rs:26) 고정 가산 — 타워 사거리 = range + 15000 + stat_buff.range + (level-1)*growth_range + target.radius() + caster.radius() | 4 |  |
| 7 | 100 | 971 | 계수 | Entity::radius() 인라인(entity.rs:1515) radius*(100+radius_mult)/100 | 4 |  |
| 8 | -1 | 971 | 센티널 | attack_effect Option 니치 None 태그(i32) — None 이면 unwrap 패닉(L1013) | 4 |  |
| 9 | 30000 | 1032 | 계수 | 페어링 반경 여유(≈0.94셀): dist_sq(target, rescue_ally) > (max_range+30000)^2 이면 구조 아군 '멀다'. closure#0 L953 에서도 같은 값으로 near_enemies 반경 | 4 |  |
| 10 | 22500000001 | 1021 | 미상 | 150000^2+1 — closure s7_0: target 반경 150000(4.69셀) 이내 적 = dive_enemies (aux m10:56457~) | 4 |  |
| 11 | 40000000001 | 1018 | 미상 | 200000^2+1 — closure s6_0: target 반경 200000(6.25셀) 이내 아군 = dive_allies (aux m10:56413~) | 4 |  |
| 12 | 14400000001 | 961 | 미상 | 120000^2+1 — closure s_0: champ(나) 반경 120000(3.75셀) 이내 & ally_battle_stop_tick 없는 아군 = near_allies (aux m01:25361~) | 4 |  |
| 13 | 6 | 969 | 임계 | iter_towers_without_nexus 결과 배열 [Option<&Entity>;6] 원소 수 — 판정 아님 | 4 |  |

**`knobs` 조정점 8건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 개인 레이스 시간 여유 | fight_model.rs:1008 (tps>>1) | 1 | shift 1 = tps/2. 줄이면(0=tps 그대로) 다이브 판정이 더 관대해짐(대상 사망틱에 더 큰 여유를 더하지 않음) — 실제로는 `tps/2` 의 2 를 바꾸는 것 | 4 | 기존 |
| 1 | 우물 도달 여유 거리 | fight_model.rs:992 | 160000 | 올리면 well_eta 가 작아져 '대상이 먼저 우물 도착' 으로 false 반환이 늘어남(다이브 덜 함) | 4 | 기존 |
| 2 | near_enemies 페어링 반경 여유 | fight_model.rs:953 (aux) | 30000 | 올리면 더 먼 적까지 내 사망틱 계산에 포함 → die_tick_with_tower 감소 → 다이브 덜 함 | 4 | 기존 |
| 3 | rescue_ally 실효 거리 여유 | fight_model.rs:1032 | 30000 | 올리면 더 먼 구조 아군도 '돕는 중' 으로 인정 → line(구조 포함) 유지 → 팀모델 다이브 승인 늘어남 | 4 | 기존 |
| 4 | near_allies 반경(제곱) | fight_model.rs:961 (aux m01) | 14400000001 | 올리면 더 먼 아군이 target 사망틱 계산에 합류 → target_die_tick 감소 → 다이브 승인 늘어남 | 4 | 기존 |
| 5 | dive_allies 반경(제곱) | fight_model.rs:1018 (aux) | 40000000001 | 올리면 팀모델 아군 수 증가 → resolve_fight_stake 유리 | 4 | 기존 |
| 6 | dive_enemies 반경(제곱) | fight_model.rs:1021 (aux) | 22500000001 | 올리면 팀모델 적 수 증가 → resolve_fight_stake 불리 | 4 | 기존 |
| 7 | version 게이트 | fight_model.rs:989 | 1 | version<=1 이면 우물 ETA 게이트·exit_margin 이 전부 생략(exit_margin=0) | 4 | 기존 |

<details><summary>`callees` 피호출자 16건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | check_kill_die_tick | game_ai::check_kill_die_tick | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::OperationData, &game_core::PlayerState, &game_core::Entity, bumpalo::collections::vec::Vec< &game_core::Entity>, bumpalo::collections::vec::Vec< &game_core::Entity>, &mut game_core::DebugFrameData) -> usize | game-ai\src\fight_check.rs:917 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | distance | game_core::utils::distance | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:12 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 2 | distance | game_core::Entity::distance | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2161 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 3 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 4 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 5 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 6 | is_enemy_well_danger | game_ai::is_enemy_well_danger | pub | fn(usize, &game_core::PlayerState, u64, u64) -> bool | game-ai\src\path_finder.rs:1032 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 7 | is_ignored_well_enemy | game_ai::plan_legacy::old::is_ignored_well_enemy | pub | fn(usize, &game_core::PlayerState, &game_core::Entity) -> bool | game-ai\src\plan_legacy\old\fight_model.rs:754 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 8 | is_recent_visible | game_core::Blackboard::is_recent_visible | pub | fn(&game_core::Blackboard, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\game\blackboard.rs:346 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 9 | is_unreasonable_tower_dive_enemy | game_ai::plan_legacy::old::is_unreasonable_tower_dive_enemy | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, bool) -> bool | game-ai\src\plan_legacy\old\fight_model.rs:792 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 10 | iter_towers_without_nexus | game_core::AbstractGameWithCache::<'a, 'b>::iter_towers_without_nexus | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5750 ~ game_core[6a30]::simulation::{impl#5}::iter_towers_without_nexus::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1830 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 11 | judge_accuracy | game_core::AthleteParameter::judge_accuracy | pub | fn(&game_core::AthleteParameter) -> usize | game-core\src\simulation\state\player.rs:337 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 12 | max_range_cached | game_ai::plan_legacy::old::max_range_cached | pub | fn(&game_core::OperationData, &game_core::Entity, &game_core::Entity) -> u64 | game-ai\src\plan_legacy\old\battle.rs:2332 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 13 | radius | game_core::Entity::radius | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1509 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 14 | resolve_fight_stake | game_ai::plan_legacy::old::fight_model::resolve_fight_stake | in:game_ai | fn(usize, &mut rand::rngs::std::StdRng, &game_core::OperationData, &game_core::PlayerState, &game_core::Entity, &[&game_core::Entity], &[&game_core::Entity], i8, std::option::Option<&game_core::Entity>, usize, &mut game_core::DebugFrameData) -> game_ai::plan_legacy::old::FightPrediction | game-ai\src\plan_legacy\old\fight_model.rs:571 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 15 | tower_dive_is_viable | game_ai::plan_legacy::old::tower_dive_is_viable | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan, &game_core::Entity, bool, &mut game_core::DebugFrameData) -> bool | game-ai\src\plan_legacy\old\fight_model.rs:935 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
</details>

⚠**미매칭 9개**: `collect`, `dist_sq`, `dive_allies`, `dive_enemies`, `drop  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `near_allies`, `near_enemies`, `next  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 9개는 **전부 다른 함수**라 싣지 않는다`, `range  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 9곳** (m10.ll:17264, m10.ll:19874, m13.ll:30126, m13.ll:30676, m13.ll:33949, m13.ll:39952, m13.ll:40930, m13.ll:42840, m13.ll:44957) · **형제 0개** 

**`open` 7건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | L990 적 우물 좌표 헬퍼의 원 함수명 — 다른 파일 L1016 인라인(`;L1016<990`)으로 (0,960000)/(960000,0) 선택만 남음. 소스 파일 미확인 | 4 |  |
| 1 | 미탐색 | check_kill_die_tick 내부(m?)·resolve_fight_stake 내부·max_range_cached·is_recent_visible·is_enemy_well_danger·is_unreasonable_tower_dive_enemy 본문은 안 봄(범위 밖) | 4 |  |
| 2 | 재료 부재 | L1032 dbg 이름 'helping' 이 `dist_sq > (r+30000)^2`(=멀다) 값에 붙어 있어 소스 변수명 극성은 확정 불가 — 명세는 분기 방향(멀면 line_absolute) 기준 | 4 |  |
| 3 | 미탐색 | closure#0 의 blackboard 인덱스가 `1 - player.team`(적 팀 보드)인 이유 — IR 그대로 기록. Blackboard 의미는 미조사 | 4 |  |
| 4 | 미탐색 | is_unreasonable_tower_dive_enemy 의 version 인자가 `poison` 으로 넘어감 — 그 함수가 version 을 안 읽는다는 뜻으로 보이나 본문 미확인 | 4 |  |
| 5 | 미탐색 | resolve_fight_stake 의 9번째 인자 i8 0 의 의미(열거형/플래그) 미확인 | 4 |  |
| 6 | 미탐색 | 동명 복제본 `death_battle::single_tower_dive_is_viable`(m05:44249?) / `single_battle::single_tower_dive_is_viable`(m05:44249) 은 담당 밖 — 본 명세는 fight_model 판만 | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

