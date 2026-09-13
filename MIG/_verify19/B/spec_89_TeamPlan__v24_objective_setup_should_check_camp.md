---

### `89` TeamPlan::v24_objective_setup_should_check_camp — [v24] Morgard/Serpen Setup 태세에서 '내가 캠프를 직접 확인하러 가야 하는가' — 라인 압박 완료·건강한 같은-편 아군 충분·내가 checker 로 뽑힘일 때, 적이 치는 중이거나 캠프가 최근에 안 보였거나 숨은 건강 적이 닿을 수 있으면 true

| 항목 | 값 |
|---|---|
| id | `objective_discipline__v24_objective_setup_should_check_camp` |
| 심볼 | `_RNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy9team_plan20objective_disciplineNtB4_8TeamPlan37v24_objective_setup_should_check_camp` |
| 소스 | `game-ai\src\plan_legacy\team_plan\objective_discipline.rs:88` |
| IR | `m09.ll` 19843~21206행 |
| 경로·가시성 | `game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v24_objective_setup_should_check_camp` · **pub** |
| 계층 | 기타 |
| exe | `dd5db0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r10` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, game_core::JungleType) -> bool
```

<details><summary>인자 6개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | self | &TeamPlan(1064B) | 공유 &self. objective 태그/phase·obj_spawn.*_camp_last_visible_tick·vision.last_visible_pos[]·vision.last_checked_ticks[] 읽기. 본문 store 는 전부 alloca(camp_pos·tick·클로저 env) — self 쓰기 0건 | 4 |
| 1 | 2 | _version | usize | 미사용 (dbg 이름 `_version`, m09.ll:19858) | 4 |
| 2 | 3 | player | &PlayerState(2528B) | info.team(+0x930)·info.position@tag(+0x9c0) | 4 |
| 3 | 4 | data | &OperationData(24B) | cache·context·blackboard | 4 |
| 4 | 5 | goal_data | &GoalData(248B) | 본문 직접 읽기 없음 — is_object_being_taken_by_enemy 에 전달만 (m09.ll:21094) | 4 |
| 5 | 6 | target | JungleType(i8) | 지역명 camp. 4=Morgard / 5=Serpen 만 의미(그 외 take_setup_like 가 false → 즉시 false). DILocalVariable !25367 type JungleType | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn v24_objective_setup_should_check_camp(&self, _version, player, data, goal_data, camp: JungleType) -> bool
  team = player.info.team; enemy = 1 - team; ctx = data.context; cache = data.cache
  // L91: 태세 게이트 — take_setup_like(camp) (team_plan.rs:258)
  //   camp==Morgard(4): self.objective 태그==0(Morgard) && phase(+0x420)==1(Setup)
  //   camp==Serpen(5) : self.objective 태그==1(Serpen)  && phase==1(Setup)
  //   그 외 camp: false
  if !take_setup_like { return false }
  // L95: v24_objective_setup_relevant_lanes_ready(player, data, camp) (인라인)
  //   lanes = camp==Morgard ? [Top(0),Mid(1)] : [Bottom(2),Mid(1)]   (anon.190 = \00\01 / anon.191 = \02\01)
  //   ready = v23_objective_setup_pressure_line(player, data, &lanes, 2).is_none()   (반환 i8 == -1)
  if !ready { return false }
  // L99~106: 캠프 쪽 절반에 있는 건강한 아군 수
  side(c) = camp==Morgard ? is_top_side(ctx,c.x,c.y) [= height - y < x] : is_bottom_side [= height - y > x]   (map_regions.rs:24 / :30)
  healthy_side(c) = c.hp*100/c.stat_cached.hp > 39 && side(c) && is_near_mid_line(ctx, c.x, c.y)
  healthy_side_allies = cache.iter_champions(team).filter(healthy_side).count()      // L100~101, [team] 5칸
  N = player_count(ctx) 인라인 → tutorial TopSolo/MidSolo/JungleOnly: 1, 그 외: 2
  if healthy_side_allies < N { return false }     // L106 (samesign ult)
  // L110~111
  camp_pos = ctx.map.camp_pos(camp, team == 0)
  my_idx = player.info.position 태그(i32)
  if cache.player_champion[team][my_idx].is_none() { return false }
  // L123~132: checker 선정 — 같은 술어를 만족하는 아군 중 (캠프 거리 + 역할 가중) 최소
  role_bias(i) = match Position::from_index(i) { Jungle=>0, Support=>20000, Mid=>40000, Top|Bottom=>60000 }
  checker_position = iter_champions(team).enumerate().filter(|(_,c)| healthy_side(c))   // L117~120 closure#2
                      .min_by_key(|(i,c)| distance(c.x,c.y, camp_pos).saturating_add(role_bias(i)))   // L124~130 closure#3
                      .map(|(i,_)| i)
  if checker_position != Some(my_idx) { return false }   // L132 (None 도 false)
  // L137~142
  target_object = (camp != Morgard)     // Morgard→false, Serpen→true 로 5번째 인자
  if is_object_being_taken_by_enemy(player, data, goal_data, self, target_object) { return true }
  // L146~155
  tick = game.tick()
  camp_last_visible_tick = camp==Morgard ? self.obj_spawn.epic_camp_last_visible_tick : self.obj_spawn.serpen_camp_last_visible_tick
  camp_recently_checked = camp_last_visible_tick + tps*2 >= tick
  camp_visible = game.is_visible_cell(team, camp_pos.x/32000, camp_pos.y/32000)
  if !(camp_recently_checked && camp_visible) { return true }
  // L160~173 (aux m09.ll:2974): 숨은 건강한 적이 마지막 관측 이후 캠프까지 이동할 수 있었나
  return (0..5).filter_map(|p| cache.player_champion[enemy][p].map(|c| (p,c))).any(|(p,c)| {
     c.hp*100/c.stat_cached.hp >= 50                                   // L162 (IR: <50 이면 skip)
     && !data.blackboard[enemy].is_recent_visible(game, player, c)       // L165
     && { last_pos = self.vision.last_visible_pos[p];                     // L169
          d = distance(last_pos, camp_pos).saturating_sub(150000);        // L170
          can_move = tick.saturating_sub(self.vision.last_checked_ticks[p]) * c.stat_cached.move_speed;  // L171~172
          can_move >= d }                                                 // L173 (IR: can_move < d 이면 skip)
  })
```

**`mem` 메모리 접근 24건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | TeamPlan | 0x41f | objective | r | Option<MainObjective> 태그(1B, 니치). 0=Morgard / 1=Serpen 만 통과 (take_setup_like, team_plan.rs:258; m09.ll:19876) | 4 | OK |
| 1 | TeamPlan | 0x420 | objective@Some.0@Morgard.phase | r | Morgard/Serpen variant 페이로드 +1 = ObjectPhase. 1=Setup 이어야 통과 (m09.ll:19878). tcxdict 는 SplitEpic.0@tag 로 라벨하지만 태그 0/1 문맥에선 phase | 3 | OK |
| 2 | TeamPlan | 0x80 | obj_spawn.epic_camp_last_visible_tick | r | camp==Morgard 일 때 (+128, phi m09.ll:21102) | 4 | OK |
| 3 | TeamPlan | 0x88 | obj_spawn.serpen_camp_last_visible_tick | r | camp==Serpen 일 때 (+136) | 4 | OK |
| 4 | TeamPlan | 0x230 | vision.last_visible_pos[i] | r | aux(m09.ll:3111~3114) [(u64,u64);5][p] — 적 p 의 마지막 관측 위치 (+560, stride 16) | 4 | OK |
| 5 | TeamPlan | 0x2d0 | vision.last_checked_ticks[i] | r | aux(m09.ll:3129~3130) [usize;5][p] — 적 p 를 마지막으로 확인한 틱 (+720) | 4 | OK |
| 6 | PlayerState | 0x930 | info.team | r | team (m09.ll:19903); aux 에서도 1-team 산출 | 4 | OK |
| 7 | PlayerState | 0x9c0 | info.position@tag | r | i32 Position 태그 = 내 슬롯 인덱스 (+2496, m09.ll:20652). checker 인덱스와 비교 | 4 | OK |
| 8 | OperationData | 0x0 | cache | r | &AbstractGameWithCache | 4 | OK |
| 9 | OperationData | 0x8 | context | r | &GameContext | 4 | OK |
| 10 | OperationData | 0x10 | blackboard | r | &[Blackboard;2] — aux 에서 [1-team](stride 744) 을 is_recent_visible 의 self 로 전달 (m09.ll:3101) | 4 | OK |
| 11 | AbstractGameWithCache | 0x0 | game.data_ptr | r | vtable +0x28 tick(m09.ll:21110) / +0x100 is_visible_cell(m09.ll:21128) | 4 | OK |
| 12 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r |  | 4 | OK |
| 13 | AbstractGameWithCache | 0x1e0 | player_champion | r | iter_champions(team) = [team] 5칸 (simulation.rs:1905, bounds 2); aux 는 [1-team] (+480) | 4 | OK |
| 14 | GameContext | 0x8 | setting | r | &GameSetting | 4 | OK |
| 15 | GameContext | 0x20 | map | r | &MapDef → camp_pos (m09.ll:20643) | 4 | OK |
| 16 | GameContext | 0x38 | tutorial | r | player_count(runner.rs:295~296) switch: 2/4/6(TopSolo/MidSolo/JungleOnly)→1, 그 외→2 (m09.ll:20612~20623) | 4 | OK |
| 17 | GameSetting | 0x12c0 | height | r | is_top_side/is_bottom_side: ry = height - y (+4800, map_regions.rs:22) | 4 | OK |
| 18 | GameSetting | 0x12f8 | tick_per_second | r | tps*2 (+4856, m09.ll:21118) | 4 | OK |
| 19 | Entity | 0x628 | stat_cached.hp | r | hp% 분모 (본문 L100/L117, aux L162) | 4 | OK |
| 20 | Entity | 0x670 | hp | r |  | 4 | OK |
| 21 | Entity | 0x660 | x | r | +1632 | 4 | OK |
| 22 | Entity | 0x668 | y | r | +1640 | 4 | OK |
| 23 | Entity | 0x640 | stat_cached.move_speed | r | aux L171 (+1600) | 4 | OK |

**`consts` 상수 19건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev | folded_from |
|---|---|---|---|---|---|---|
| 0 | 4 | 91 | 태그 | JungleType 태그 4=Morgard (target switch, m09.ll:19876; L137 `camp == Morgard` 도 동일 값) | 4 |  |
| 1 | 5 | 91 | 태그 | JungleType 태그 5=Serpen | 4 |  |
| 2 | 0 | 258 | 태그 | team_plan.rs:258 take_setup_like: MainObjective 태그 0=Morgard (camp Morgard 일 때 요구) | 4 |  |
| 3 | 1 | 258 | 태그 | MainObjective 태그 1=Serpen (camp Serpen 일 때) · 그리고 ObjectPhase 태그 1=Setup (+0x420 == 1, 두 camp 공통). 본문 `shl 1` 은 별개(tps*2 항목) | 4 |  |
| 4 | -1 | 95 | 센티널 | v23_objective_setup_pressure_line 반환 Option<LineType> 의 None 니치(range(i8 -1,3)). None 이어야(=압박할 라인 없음) 통과 | 4 |  |
| 5 | 2 | 95 | 임계 | v23_objective_setup_pressure_line 의 4번째 인자 리터럴 2 (의미는 그 함수 소유 — 미탐색) | 4 |  |
| 6 | 100 | 100 | 계수 | hp*100/stat_cached.hp 백분율 (L100·L117·aux L162 공통) | 4 |  |
| 7 | 39 | 100 | 임계 | hp% > 39 (≥40%) 인 아군만 (L100 count 술어 · L117 checker 후보 술어). `>=40` 표기 여부는 표기 불가 | 4 |  |
| 8 | 2 | 106 | 태그 | healthy_side_allies < N 이면 false. N = tutorial 태그 2/4/6 → 1, 그 외 → 2 (player_count 인라인 phi, runner.rs:295~296). 소스가 `player_count(ctx).min(2)` 류인지는 미탐색 | 4 |  |
| 9 | 1 | 106 | 태그 | 위 N 의 1인 튜토리얼(TopSolo/MidSolo/JungleOnly) 값. `shl 1`(tps*2) 과 무관 | 4 |  |
| 10 | 0 | 125 | 태그 | role_bias: Position 태그 1=Jungle → 0 | 4 |  |
| 11 | 40000 | 127 | 태그 | role_bias: Position 태그 2=Mid → 40000 (1.25셀) | 4 |  |
| 12 | 20000 | 126 | 태그 | role_bias: Position 태그 4=Support → 20000 | 4 |  |
| 13 | 60000 | 124 | 태그 | role_bias: Position 태그 0=Top / 3=Bottom → 60000 (match 기본 arm, phi 기본값) | 4 |  |
| 14 | 1 | 152 | 태그 | tps*2 (= 2초) — `shl i64 %480, 1` 로 접힘 (m09.ll:21118). camp_last_visible_tick + tps*2 >= tick | 4 | 2 |
| 15 | 32000 | 153 | 계수 | 셀 크기 — camp_pos.x/32000, y/32000 → is_visible_cell(team, cx, cy) 인자 (좌표 변환, 임계 아님) | 4 |  |
| 16 | 50 | 162 | 미상 | aux: 적 hp% < 50 이면 후보 제외 (즉 ≥50% 만) (m09.ll:3089) | 4 |  |
| 17 | 150000 | 170 | 미상 | aux: d = distance(last_pos, camp_pos).saturating_sub(150000) — 캠프 반경 150000(≈4.7셀) 을 '도달' 로 간주 (m09.ll:3122) | 4 |  |
| 18 | 5 | 160 | 태그 | aux: 0..5 슬롯 상한·player_champion bounds (배열 길이) | 4 |  |

**`knobs` 조정점 6건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 건강 아군 hp% 임계(count·checker 후보 공통) | objective_discipline.rs:100 / :117 | 39 | 올리면 healthy_side_allies 가 줄어 L106 에서 false 가 늘고 checker 후보도 줄어든다 | 4 | 기존 |
| 1 | 같은-편 건강 아군 최소 인원 | objective_discipline.rs:106 | 2 | N(일반 2). 올리면 확인 자체가 드물어진다 | 4 | 기존 |
| 2 | checker 역할 가중 | objective_discipline.rs:124~127 | Jungle 0 / Support 20000 / Mid 40000 / Top·Bottom 60000 | 낮은 쪽이 checker 로 뽑힌다. Jungle 가중을 올리면 정글러가 아닌 아군이 확인 담당이 된다 | 4 | 기존 |
| 3 | 캠프 최근 확인 창 | objective_discipline.rs:152 | tps*2 | 올리면 더 오래된 확인도 '최근'으로 쳐 재확인(true)이 줄어든다 | 4 | 기존 |
| 4 | 숨은 적 hp% 임계 | objective_discipline.rs:162 | 50 | 내리면 저체력 적도 위협으로 세어 true 가 늘어난다 | 4 | 기존 |
| 5 | 캠프 도달 반경 | objective_discipline.rs:170 | 150000 | 올리면 더 먼 적도 '닿을 수 있음' 으로 판정돼 true 가 늘어난다 | 4 | 기존 |

<details><summary>`callees` 피호출자 20건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | camp_pos | game_core::MapDef::camp_pos | pub | fn(&game_core::MapDef, game_core::JungleType, bool) -> (u64, u64) | game-core\src\simulation\map_def.rs:207 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | distance | game_core::utils::distance | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:12 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 2 | from_index | game_core::Position::from_index | pub | fn(usize) -> game_core::Position | game-core\src\simulation\entity.rs:622 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 3 | is_near_mid_line | game_core::is_near_mid_line | pub | fn(&game_core::GameContext, u64, u64) -> bool | game-core\src\simulation\map_regions.rs:127 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | is_object_being_taken_by_enemy | game_ai::plan_legacy::team_plan::objective_helpers::is_object_being_taken_by_enemy | in:game_ai | fn(&game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &game_ai::plan_legacy::team_plan::TeamPlan, game_ai::plan_legacy::team_plan::objective_helpers::WavePriorityObject) -> bool | game-ai\src\plan_legacy\team_plan\objective_helpers.rs:313 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 5 | is_recent_visible | game_core::Blackboard::is_recent_visible | pub | fn(&game_core::Blackboard, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\game\blackboard.rs:346 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 6 | is_top_side | game_core::is_top_side | pub | fn(&game_core::GameContext, u64, u64) -> bool | game-core\src\simulation\map_regions.rs:21 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 7 | is_visible_cell | game_core::AbstractGame::is_visible_cell | pub | fn(&Self/#0, usize, usize, usize) -> bool | game-core\src\simulation.rs:126 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 8 | is_visible_cell | <game_core::Game as game_core::AbstractGame>::is_visible_cell | pub | fn(&game_core::Game, usize, usize, usize) -> bool | game-core\src\simulation\game.rs:1804 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 9 | is_visible_cell | <game_core::SingleLaneGame as game_core::AbstractGame>::is_visible_cell | pub | fn(&game_core::SingleLaneGame, usize, usize, usize) -> bool | game-core\src\simulation\game.rs:3869 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 10 | iter_champions | game_core::AbstractGameWithCache::<'a, 'b>::iter_champions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5759 ~ game_core[6a30]::simulation::{impl#5}::iter_champions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1904 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 11 | player_count | game_core::GameRule::player_count | pub | fn(&game_core::GameRule) -> usize | game-core\src\data\match_info.rs:542 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 12 | player_count | game_core::TutorialType::player_count | pub | fn(&game_core::TutorialType) -> usize | game-core\src\simulation\game\runner.rs:294 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 13 | take_setup_like | game_ai::plan_legacy::team_plan::TeamPlan::take_setup_like | pub | fn(&game_ai::plan_legacy::team_plan::TeamPlan, game_core::JungleType) -> bool | game-ai\src\plan_legacy\team_plan.rs:257 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 14 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 15 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 16 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 17 | v23_objective_setup_pressure_line | game_ai::plan_legacy::team_plan::v23_objective_setup_pressure_line | pub | fn(&game_core::PlayerState, &game_core::OperationData, &[game_core::LineType]) -> std::option::Option<game_core::LineType> | game-ai\src\plan_legacy\team_plan\objective_helpers.rs:73 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 18 | v24_objective_setup_relevant_lanes_ready | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v24_objective_setup_relevant_lanes_ready | in:game_ai | fn(&game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PlayerState, &game_core::OperationData, game_core::JungleType) -> bool | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:199 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 19 | v24_objective_setup_should_check_camp | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v24_objective_setup_should_check_camp | pub | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, game_core::JungleType) -> bool | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:88 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
</details>

⚠**미매칭 6개**: `enumerate`, `healthy_side`, `phase`, `role_bias`, `side`, `try_fold`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 2곳** (m10.ll:9262, m10.ll:11397) · **형제 55개** (TeamPlan)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | game_ai::plan_legacy::old::epic::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_press_epic | pub | game-ai\src\plan_legacy\old\epic.rs:502 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 1 | game_ai::plan_legacy::old::epic::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v3_epicops_buff_window | in:game_ai | game-ai\src\plan_legacy\old\epic.rs:634 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &game_ai::plan_legacy::types::BigPlan) -> bool |
| 2 | game_ai::plan_legacy::old::epic::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_epic_line_change | pub | game-ai\src\plan_legacy\old\epic.rs:684 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData, game_core::LineType) |
| 3 | <game_ai::plan_legacy::team_plan::TeamPlan as std::clone::Clone>::clone | pub | game-ai\src\plan_legacy\team_plan.rs:47 | True | fn(&game_ai::plan_legacy::team_plan::TeamPlan) -> game_ai::plan_legacy::team_plan::TeamPlan |
| 4 | <game_ai::plan_legacy::team_plan::TeamPlan as std::fmt::Debug>::fmt | pub | game-ai\src\plan_legacy\team_plan.rs:47 | True | fn(&game_ai::plan_legacy::team_plan::TeamPlan, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 5 | <game_ai::plan_legacy::team_plan::TeamPlan as std::default::Default>::default | pub | game-ai\src\plan_legacy\team_plan.rs:47 | True | fn() -> game_ai::plan_legacy::team_plan::TeamPlan |
| 6 | game_ai::plan_legacy::team_plan::TeamPlan::mf_note_obj_clear | in:game_ai | game-ai\src\plan_legacy\team_plan.rs:196 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, u8, usize) |
| 7 | game_ai::plan_legacy::team_plan::TeamPlan::sanitize_rule_scope | pub | game-ai\src\plan_legacy\team_plan.rs:200 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, &game_core::GameContext) |
| 8 | game_ai::plan_legacy::team_plan::TeamPlan::objective_target | pub | game-ai\src\plan_legacy\team_plan.rs:230 | True | fn(&game_ai::plan_legacy::team_plan::TeamPlan) -> std::option::Option<game_core::JungleType> |
| 9 | game_ai::plan_legacy::team_plan::TeamPlan::take_active | pub | game-ai\src\plan_legacy\team_plan.rs:243 | True | fn(&game_ai::plan_legacy::team_plan::TeamPlan, game_core::JungleType) -> bool |
| 10 | game_ai::plan_legacy::team_plan::TeamPlan::take_hunt_commit | pub | game-ai\src\plan_legacy\team_plan.rs:248 | True | fn(&game_ai::plan_legacy::team_plan::TeamPlan, game_core::JungleType) -> bool |
| 11 | game_ai::plan_legacy::team_plan::TeamPlan::take_setup_like | pub | game-ai\src\plan_legacy\team_plan.rs:257 | True | fn(&game_ai::plan_legacy::team_plan::TeamPlan, game_core::JungleType) -> bool |
| 12 | game_ai::plan_legacy::team_plan::TeamPlan::mark_objective_misunderstanding | pub | game-ai\src\plan_legacy\team_plan.rs:265 | True | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, game_core::JungleType, usize) |
| 13 | game_ai::plan_legacy::team_plan::TeamPlan::clear_objective_misunderstanding | pub | game-ai\src\plan_legacy\team_plan.rs:277 | True | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan) |
| 14 | game_ai::plan_legacy::team_plan::TeamPlan::init | in:game_ai::plan_legacy::team_plan | game-ai\src\plan_legacy\team_plan.rs:281 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 15 | game_ai::plan_legacy::team_plan::TeamPlan::update | pub | game-ai\src\plan_legacy\team_plan.rs:294 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 16 | game_ai::plan_legacy::team_plan::TeamPlan::can_near_enemies | pub | game-ai\src\plan_legacy\team_plan.rs:444 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< &game_core::Entity> |
| 17 | game_ai::plan_legacy::team_plan::TeamPlan::can_near_enemies_range | pub | game-ai\src\plan_legacy\team_plan.rs:483 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, u64, u64, u64, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< &game_core::Entity> |
| 18 | game_ai::plan_legacy::team_plan::TeamPlan::update_wave_priority_clear_line | in:game_ai::plan_legacy::team_plan | game-ai\src\plan_legacy\team_plan.rs:540 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) |
| 19 | game_ai::plan_legacy::team_plan::TeamPlan::should_player_clear_wave_priority_line | pub | game-ai\src\plan_legacy\team_plan.rs:561 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PlayerState, &game_core::OperationData) -> std::option::Option<game_core::LineType> |
| 20 | game_ai::plan_legacy::team_plan::TeamPlan::should_delay_morgard_for_wave_priority | in:game_ai::plan_legacy::team_plan | game-ai\src\plan_legacy\team_plan.rs:570 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData) -> bool |
| 21 | game_ai::plan_legacy::team_plan::TeamPlan::should_delay_serpen_for_wave_priority | in:game_ai::plan_legacy::team_plan | game-ai\src\plan_legacy\team_plan.rs:574 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData) -> bool |
| 22 | game_ai::plan_legacy::team_plan::TeamPlan::should_delay_object_setup_for_wave_priority | pub | game-ai\src\plan_legacy\team_plan.rs:578 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData) -> bool |
| 23 | game_ai::plan_legacy::team_plan::TeamPlan::should_keep_object_for_contested_wave_priority | in:game_ai::plan_legacy::team_plan | game-ai\src\plan_legacy\team_plan.rs:586 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, game_ai::plan_legacy::team_plan::objective_helpers::WavePriorityObject) -> bool |
| 24 | game_ai::plan_legacy::team_plan::TeamPlan::repair_misunderstood_objective | in:game_ai::plan_legacy::team_plan | game-ai\src\plan_legacy\team_plan.rs:603 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData) -> bool |
| 25 | game_ai::plan_legacy::team_plan::TeamPlan::update_objective | pub | game-ai\src\plan_legacy\team_plan.rs:722 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData) |
| 26 | game_ai::plan_legacy::team_plan::TeamPlan::update_steal | pub | game-ai\src\plan_legacy\team_plan.rs:732 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &game_ai::plan_legacy::types::BigPlan) |
| 27 | game_ai::plan_legacy::team_plan::TeamPlan::update_objective_after_steal | in:game_ai::plan_legacy::team_plan | game-ai\src\plan_legacy\team_plan.rs:910 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData) |
| 28 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v24_objective_setup_lane_pressure_ready | pub | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:7 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, game_core::JungleType, &mut game_core::DebugFrameData) -> bool |
| 29 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v24_objective_setup_should_check_camp | pub | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:88 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, game_core::JungleType) -> bool |
| 30 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v24_objective_setup_should_release_to_passive | pub | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:177 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::PlayerState, &game_core::OperationData, game_core::JungleType) -> bool |
| 31 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v24_objective_setup_relevant_lanes_ready | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:199 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PlayerState, &game_core::OperationData, game_core::JungleType) -> bool |
| 32 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v25_objective_posture | pub | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:207 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::PlayerState, &game_core::OperationData, game_core::JungleType) -> std::option::Option<game_ai::plan_legacy::team_plan::ObjectivePosture> |
| 33 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v27_objective_wait_pos | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:316 | False | fn(game_core::JungleType, usize, &game_core::MapDef) -> (u64, u64) |
| 34 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v27_objective_entity | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:324 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, &game_core::OperationData, game_core::JungleType) -> std::option::Option<&game_core::Entity> |
| 35 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::update_v27_objective_discipline | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:334 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &mut game_core::DebugFrameData) |
| 36 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v27_active_objective_discipline | pub | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:483 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::OperationData, game_core::JungleType) -> std::option::Option<game_ai::plan_legacy::team_plan::ObjectiveDisciplineState> |
| 37 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v27_objective_discipline_blocks_battle | pub | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:497 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::OperationData) -> bool |
| 38 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v27_objective_discipline_action | pub | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:505 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, game_core::JungleType) -> std::option::Option<game_ai::SmallActionPlay> |
| 39 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_repair_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:7 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PlayerState, &game_core::OperationData) |
| 40 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_defense_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:16 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData) -> bool |
| 41 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_nexus_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:29 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData, game_core::LineType) -> bool |
| 42 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_defense_line_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:46 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData, game_core::LineType) -> bool |
| 43 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_dive_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:88 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData, game_core::LineType) -> bool |
| 44 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_press_tower_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:141 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData, game_core::LineType) -> bool |
| 45 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_press_epic_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:253 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData, game_core::LineType) -> bool |
| 46 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_split_epic_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:334 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData, game_core::LineType) -> bool |
| 47 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_comeback_pick_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:393 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData, game_core::LineType, bool) -> bool |
| 48 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_serpen_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:558 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData, game_ai::plan_legacy::team_plan::ObjectPhase) -> bool |
| 49 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_morgard_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:685 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData, game_ai::plan_legacy::team_plan::ObjectPhase) -> bool |
| 50 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_none_or_gank_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:830 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData) -> bool |
| 51 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_gank_preprocess | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:1133 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, game_core::LineType) -> bool |
| 52 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_nexus_defense | pub | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:1227 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> bool |
| 53 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_nexus_attack | pub | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:1237 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> std::option::Option<game_core::LineType> |
| 54 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_sub_objective | pub | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:1258 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData) |

**`open` 7건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | L106 의 N: IR 은 player_count(ctx) 인라인 결과가 phi {1,2} 뿐이다 — 튜토리얼 None 에서 player_count 가 5 라면 소스는 `.min(2)` 류로 접힌 것. 소스 표현 미탐색(runner.rs:295~296 원문). 동작은 확정: 일반 게임 N=2 | 4 |  |
| 1 | 미탐색 | v23_objective_setup_pressure_line(player,data,&lanes,2) 의 4번째 인자 2 의 의미 — 그 함수(m15.ll:55350) 미탐색 | 4 |  |
| 2 | 재료 부재 | L95 relevant_lanes_ready 의 소스 줄 — !DILocation line 0 (m09.ll:19896) 이라 함수 시작줄 확정 불가 | 4 |  |
| 3 | 미탐색 | is_object_being_taken_by_enemy 5번째 bool 인자 이름 — dbg 이름 target_object(i8 0/1). 함수1 명세(defer_serpen)에 인라인된 판과 달리 여기는 아웃오브라인 호출이고 bool 이 추가돼 있어 Morgard/Serpen 분기용으로 추정(내부 미탐색) | 4 |  |
| 4 | 미탐색 | exe 0xe3b570(273B)·0xe2fa70(733B)·0xec9bf0(364B) 의 IR 짝 배정(camp_pos / fold / is_object_being_taken_by_enemy)은 크기·순서 추정 — Ghidra 미사용 | 4 |  |
| 5 | 미탐색 | Blackboard::is_recent_visible(&blackboard[1-team], game, player, c) 의 내부(어느 last_visible 배열을 보는지)는 _gcbc g07.ll:157005 미독해 — aux exe 의 vtable +0xf8/+0x150/+0x28 호출은 그 인라인 흔적 | 4 |  |
| 6 | 미탐색 | min_by_key 의 비교 클로저(m12.ll 8000 조각이 부르는 call_mut 심)는 표준 Ord::cmp — 별도 aux 로 안 넣음 | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

