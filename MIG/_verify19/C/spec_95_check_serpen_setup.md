---

### `95` check_serpen_setup — 세르펜 셋업(라인 정리·시야 확보 태세)에 들어갈지 판정 — 스폰 임박·에픽 상태·머릿수·라인 상태·판단 페널티·early_serpen 전략으로 결정

| 항목 | 값 |
|---|---|
| id | `serpen__check_serpen_setup` |
| 심볼 | `_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old6serpen18check_serpen_setup` |
| 소스 | `game-ai\src\plan_legacy\old\serpen.rs:68` |
| IR | `m05.ll` 47634~49220행 |
| 경로·가시성 | `game_ai::plan_legacy::old::check_serpen_setup` · **pub** |
| 계층 | 레거시 플랜 |
| exe | `d62bb0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r10` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bool
```

<details><summary>인자 6개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | version | usize | macro_judgement_penalty 에 그대로 전달만. 본문 분기 없음 | 4 |
| 1 | 1 | rnd | &mut StdRng(320B) | PlayerState::strategy 호출에만 전달(L175) | 4 |
| 2 | 2 | player | &PlayerState(2528B) | info.team(+0x930)·info.position(+0x9c0)·athlete(+0x180 via penalty) | 4 |
| 3 | 3 | data | &OperationData(24B) | +0x0 cache / +0x8 context / +0x10 blackboard[2] | 4 |
| 4 | 4 | team_plan | &TeamPlan(1064B) | obj_spawn.epic_giveup_tick(+0x50)·serpen_giveup_tick(+0x60) 만 읽음 | 4 |
| 5 | 5 | debug | &mut DebugFrameData(224B) | ctx.debug 일 때만 +0xa0 infos(HashMap<usize,Vec<String>>) 에 로그 push | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
ctx = data.context; team = player.info.team; enemy = 1-team
L69  if !serpen_exists(ctx)  [ctx.tutorial ∉ {None,MidBottom,Line,Total}]  → return false
L73  if is_skip_serpen(version, rnd, player, data) → return false
     (is_skip_serpen 요지, serpen.rs:366~414: !serpen_exists→true / position==Top && is_line_phase && line_exists(Top) 일 때 strategy.early_serpen_top: Must→false, Giveup→true, Flexible→적 탑타워·top_lead[team]>2·top minion_count>1 로 결정 / 그 외 false)
L77  mode = game.get_game_mode(); Moba 가 아니면 unwrap 패닉
     if mode.jungle_runner.serpen.live_list.len == 0 {          // 세르펜 미생존
L78      remain_spawn_tick = serpen.next_respawn_tick.saturating_sub(game.tick())
L81      if remain_spawn_tick >= tps*15 → return false             // 15초 이내 스폰만 통과
     }
L87  if epic.live_list.len != 0 && team_plan.obj_spawn.epic_giveup_tick.is_none() → return false   // 에픽 살아있고 포기 안 함
L91  if team_plan.obj_spawn.serpen_giveup_tick.is_some() → return false
L95  if mode.epic_minion_buff_time[enemy] != 0 {               // 적이 에픽 버프
L97      live_ally = count(player_champion[team] Some); live_enemy = count(player_champion[enemy] Some)
L99      if live_ally + 2 < live_enemy → return false
     }
L105 if epic.live_list.len != 0 && epic_giveup_tick.is_some() {   // (L87 통과했으므로 생존이면 반드시 포기 상태)
L107     live_ally = count(...)
L108     if live_ally > 2 {
L111         has_big_wave = valid_lines(ctx.tutorial).iter().any(|line| blackboard[team].line_state(line).minion_count < -2)   // closure$0(aux)
L115         if ctx.debug { debug.infos[my_champ.id].push(format!(.. has_big_wave, top/mid/bottom minion_count ..)) }
L125         return !has_big_wave
         }
     }
L135 near_serpen_ally = player_champion[team].iter_champions().filter(|e| is_bottom_side(ctx,e.x,e.y) && e.hp*100/e.stat_cached.hp > 49).count()
     (is_bottom_side, map_regions.rs:28~30: ry = setting.height - y; (ry <= x) || is_near_mid_line(ctx,x,y))
L138 (in_vision, out_vision) = serpen_reachable_enemy_count(player, data, team_plan)
L140 near_serpen_enemy = in_vision + out_vision
L142 if ctx.debug { debug.infos[my_champ.id].push(format!(near_serpen_ally, near_serpen_enemy)) }
L150 mid_count = blackboard[team].mid_minion_state.minion_count; L151 bottom_count = ...bottom_minion_state.minion_count
L153 if is_line_phase(ctx) {   // tutorial∈{0,5,7,8} && tick < epic_jungle.first_spawn_tick.saturating_sub(tps*30)
L154     if line_exists(ctx, Mid) && mid_count < 2 → line_clear = false
L156     else line_clear = !(line_exists(ctx, Bottom) && bottom_count < 2)
     } else {
L162     line_clear = !valid_lines(ctx.tutorial).iter().any(|line| blackboard[team].line_state(line).minion_count < 2)
     }
L166 live_ally = count(player_champion[team]); L167 live_enemy = count(player_champion[enemy])
L168 judgement_penalty = macro_judgement_penalty(version, player)   // = ceil(max(400 - judge_accuracy, 0)/125) ∈ 0..=4 (objective_helpers.rs:16~18)
L169 score = (near_serpen_ally - near_serpen_enemy)*2 + (live_ally - live_enemy) + (line_clear ? 2 : 0)   (i32)
L172 judgement_ready = judgement_penalty == 0 || score >= judgement_penalty
L174 if is_line_phase(ctx) {
L175     strategy = player.strategy(rnd, game)
L178     match strategy.early_serpen {
           Must(0)     → return true
           Flexible(1) → (아래 L193 으로 계속)
           Giveup(2)   → L187 return line_clear && near_serpen_ally > near_serpen_enemy+1 && live_ally > live_enemy+1 && judgement_ready
         }
     }
L193 if !judgement_ready → return false
L197 if near_serpen_ally >= near_serpen_enemy {
L199     return line_clear || near_serpen_ally > near_serpen_enemy
     } else {
L208     return out_vision > 2 && live_ally > 2      // L208~210
     }
```

**`mem` 메모리 접근 30건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | OperationData | 0x8 | context | r | &GameContext | 4 | OK |  |
| 1 | OperationData | 0x10 | blackboard | r | &[Blackboard;2], [team] 인덱스 | 4 | OK |  |
| 2 | GameContext | 0x38 | tutorial | r | TutorialType 태그. serpen_exists/is_line_phase/valid_lines 인라인 switch 의 키 | 4 | OK |  |
| 3 | GameContext | 0x8 | setting | r | &GameSetting | 4 | OK |  |
| 4 | GameContext | 0x3b | debug | r | bool. true 면 디버그 문자열 push(판정엔 무영향) | 4 | OK |  |
| 5 | GameSetting | 0x12f8 | tick_per_second | r | tps*15(L81), tps*30(is_line_phase) | 4 | OK |  |
| 6 | GameSetting | 0x8a8 | epic_jungle.first_spawn_tick | r | is_line_phase(setting.rs:703) 의 기준 | 4 | OK |  |
| 7 | GameSetting | 0x12c0 | height | r | is_bottom_side(map_regions.rs:28): ry = height - y | 4 | OK |  |
| 8 | AbstractGameWithCache | 0x0 | game.data_ptr(dyn AbstractGame) | r | vtable +0x40 get_game_mode / +0x28 tick | 4 | OK |  |
| 9 | AbstractGameWithCache | 0x1e0 | player_champion | r | Option<&Entity> 5칸 — 생존 수·근처 아군 필터 | 4 | OK |  |
| 10 | GameMode | 0x0 | @tag | r | 0=Moba 만 허용, 아니면 unwrap_failed 패닉(L1013) | 4 | OK |  |
| 11 | MobaMode | 0x1d8 | jungle_runner.serpen.live_list.len | r | 0 = 세르펜 미생존 | 4 | OK |  |
| 12 | MobaMode | 0x1e0 | jungle_runner.serpen.next_respawn_tick | r | remain_spawn_tick = saturating_sub(tick) | 4 | OK |  |
| 13 | MobaMode | 0x1a8 | jungle_runner.epic.live_list.len | r | 0 = 에픽 미생존 | 4 | OK |  |
| 14 | MobaMode | 0x240 | epic_minion_buff_time[team] | r | 적팀 에픽 버프 잔여 (≠0 = 적이 에픽 버프 보유) | 4 | OK |  |
| 15 | TeamPlan | 0x50 | obj_spawn.epic_giveup_tick@tag | r | Option<usize> 태그 0=None | 4 | OK |  |
| 16 | TeamPlan | 0x60 | obj_spawn.serpen_giveup_tick | r | Option<usize> 태그 0=None | 4 | OK |  |
| 17 | PlayerState | 0x930 | info.team | r | team / 1-team | 4 | OK |  |
| 18 | PlayerState | 0x9c0 | info.position@tag | r | 디버그 로그에서 내 챔피언 인덱스로만 사용 | 4 | OK |  |
| 19 | Blackboard | 0x20 | top_minion_state.minion_count | r | i32. Top 라인 (closure$0·L163 에서 line 별 선택) | 4 | OK |  |
| 20 | Blackboard | 0x48 | mid_minion_state.minion_count | r | i32. mid_count(L150) | 4 | OK |  |
| 21 | Blackboard | 0x70 | bottom_minion_state.minion_count | r | i32. bottom_count(L151) | 4 | OK |  |
| 22 | Entity | 0x660 | x | r | is_bottom_side | 4 | OK |  |
| 23 | Entity | 0x668 | y | r | is_bottom_side | 4 | OK |  |
| 24 | Entity | 0x628 | stat_cached.hp | r | 최대 HP (0 이면 div_by_zero 패닉) | 4 | OK |  |
| 25 | Entity | 0x670 | hp | r | hp*100/max_hp > 49 | 4 | OK |  |
| 26 | Entity | 0x5c0 | id | r | 디버그 HashMap 키만 | 4 | OK |  |
| 27 | Strategy | 0x13 | early_serpen@tag | r | EarlySerpenStrategy 0=Must 1=Flexible 2=Giveup | 4 | OK |  |
| 28 | DebugFrameData | 0xa0 | infos | r | HashMap<usize,Vec<String>> — 디버그 전용 쓰기 | 4 | OK |  |
| 29 | DebugFrameData | 0xa0 | infos | w | ctx.debug==true 일 때만. 판정에 영향 없음 | 4 | OK | format 문자열 push |

**`consts` 상수 17건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev | folded_from |
|---|---|---|---|---|---|---|
| 0 | 0 | 69 | 태그 | TutorialType 태그 집합 {0 None,5 MidBottom,7 Line,8 Total} = spawn_serpen(runner.rs:267) → serpen_exists(rule_scope.rs:50). 밖이면 즉시 false | 4 |  |
| 1 | 15 | 81 | 계수 | tps*15 — 세르펜 미생존 시 리스폰까지 남은 틱이 15초 이상이면 false | 4 |  |
| 2 | 2 | 99 | 임계 | 적이 에픽 버프 보유 시 live_ally+2 < live_enemy(3명 이상 열세)면 false | 4 |  |
| 3 | 2 | 108 | 임계 | 에픽 생존+포기 상태에서 live_ally > 2 이면 has_big_wave 만으로 판정 | 4 |  |
| 4 | -2 | 112 | 미상 | has_big_wave: 유효 라인 중 blackboard[team].<line>_minion_state.minion_count < -2 인 라인이 하나라도 있으면 true (closure$0, aux) | 4 |  |
| 5 | 49 | 136 | 임계 | hp*100/max_hp > 49 = HP 50% 이상 아군만 near_serpen_ally 로 셈 | 4 |  |
| 6 | 100 | 136 | 계수 | HP 백분율 산출 | 4 |  |
| 7 | 30 | 153 | 계수 | is_line_phase(setting.rs:704): tick < epic first_spawn_tick.saturating_sub(tps*30) | 4 |  |
| 8 | 1 | 154 | 태그 | LineType::Mid 태그(line_exists 인자) | 4 |  |
| 9 | 2 | 154 | 임계 | mid_count < 2 / bottom_count < 2 / valid_lines any(minion_count < 2) → line_clear=false | 4 |  |
| 10 | 2 | 156 | 태그 | LineType::Bottom 태그(line_exists 인자) | 4 |  |
| 11 | 1 | 169 | 태그 | shl 1 = (near_serpen_ally - near_serpen_enemy)*2 로 접힘 | 4 | 2 |
| 12 | 2 | 171 | 임계 | line_clear 이면 점수 +2 | 4 |  |
| 13 | 0 | 172 | 태그 | judgement_penalty == 0 이면 무조건 judgement_ready | 4 |  |
| 14 | 1 | 187 | 태그 | Giveup 전략: near_ally > near_enemy+1 && live_ally > live_enemy+1 필요 | 4 |  |
| 15 | 2 | 208 | 임계 | 근처 적이 더 많을 때: 비가시 도달가능 적(out_vision) > 2 | 4 |  |
| 16 | 2 | 210 | 임계 | …이고 live_ally_count > 2 이면 true | 4 |  |

**`knobs` 조정점 9건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 스폰 임박 창 | serpen.rs:81 | 15 | 올리면 세르펜 리스폰 더 이르게(더 오래 전부터) 셋업 시작. 내리면 스폰 직전에만 | 4 | 기존 |
| 1 | 적 에픽버프 시 허용 열세 | serpen.rs:99 | 2 | 올리면 더 큰 열세에서도 셋업 진입 | 4 | 기존 |
| 2 | big wave 임계 | serpen.rs:112 (closure$0) | -2 | 더 음수로 내리면 큰 적 웨이브를 덜 인정 → 에픽포기 상황에서 셋업 진입 잦아짐 | 4 | 기존 |
| 3 | 근처 아군 HP 컷 | serpen.rs:136 | 49 | 올리면 더 건강한 아군만 세어 near_serpen_ally 감소 → 셋업 보수적 | 4 | 기존 |
| 4 | 라인 정리 기준 minion_count | serpen.rs:154/156/163 | 2 | 올리면 line_clear 조건 엄격 → 점수 +2 와 L199 통과가 어려워짐 | 4 | 기존 |
| 5 | line_clear 가산점 | serpen.rs:171 | 2 | 올리면 라인 정리만으로 판단 페널티 상쇄 쉬움 | 4 | 기존 |
| 6 | Giveup 전략 우위 마진 | serpen.rs:187 | 1 | 올리면 Giveup 전략에서 셋업이 더 어려움 | 4 | 기존 |
| 7 | 비가시 적 허용 수 | serpen.rs:208 | 2 | 내리면 근처 적 우세여도 비가시 다수 상황에서 진입 쉬워짐 | 4 | 기존 |
| 8 | 판단 페널티 분모/기준 | objective_helpers.rs:17 (별도 함수) | 125 | judge_accuracy 400 미만이면 페널티 1~4. 분모를 키우면 페널티 감소 | 4 | 기존 |

<details><summary>`callees` 피호출자 21건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | get_game_mode | game_core::AbstractGame::get_game_mode | pub | fn(&Self/#0) -> game_core::GameMode | game-core\src\simulation.rs:78 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 1 | get_game_mode | <game_core::Game as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::Game) -> game_core::GameMode | game-core\src\simulation\game.rs:1707 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 2 | get_game_mode | <game_core::SingleLaneGame as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::SingleLaneGame) -> game_core::GameMode | game-core\src\simulation\game.rs:4248 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 3 | is_bottom_side | game_core::is_bottom_side | pub | fn(&game_core::GameContext, u64, u64) -> bool | game-core\src\simulation\map_regions.rs:27 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 4 | is_line_phase | game_core::GameSetting::is_line_phase | pub | fn(&game_core::GameSetting, usize) -> bool | game-core\src\setting.rs:702 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 5 | is_line_phase | game_core::GameContext::<'a, 'b>::is_line_phase | pub | fn(&game_core::GameContext<'a/#0, 'b/#1>, usize) -> bool | game-core\src\simulation\game\runner.rs:397 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 6 | is_near_mid_line | game_core::is_near_mid_line | pub | fn(&game_core::GameContext, u64, u64) -> bool | game-core\src\simulation\map_regions.rs:127 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 7 | is_skip_serpen | game_ai::plan_legacy::old::is_skip_serpen | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bool | game-ai\src\plan_legacy\old\serpen.rs:365 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 8 | iter_champions | game_core::AbstractGameWithCache::<'a, 'b>::iter_champions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5759 ~ game_core[6a30]::simulation::{impl#5}::iter_champions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1904 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 9 | line_exists | game_ai::plan_legacy::rule_scope::line_exists | pub | fn(&game_core::GameContext, game_core::LineType) -> bool | game-ai\src\plan_legacy\rule_scope.rs:24 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 10 | macro_judgement_penalty | game_ai::plan_legacy::team_plan::macro_judgement_penalty | pub | fn(usize, &game_core::PlayerState) -> i32 | game-ai\src\plan_legacy\team_plan\objective_helpers.rs:15 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 11 | push | <game_core::Staff as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::staff | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\staff.rs:38 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 12 | push | <game_core::Athlete as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:434 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 13 | push | <game_core::Contract as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:2498 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 14 | serpen_exists | game_ai::plan_legacy::rule_scope::serpen_exists | pub | fn(&game_core::GameContext) -> bool | game-ai\src\plan_legacy\rule_scope.rs:49 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 15 | serpen_reachable_enemy_count | game_ai::plan_legacy::old::serpen_reachable_enemy_count | pub | fn(&game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan) -> (usize, usize) | game-ai\src\plan_legacy\old\serpen.rs:43 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 16 | strategy | game_core::PlayerState::strategy | pub | fn(&game_core::PlayerState, &mut rand::rngs::std::StdRng, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + ) -> game_core::Strategy | game-core\src\simulation\state\player.rs:1576 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 17 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 18 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 19 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 20 | valid_lines | game_ai::plan_legacy::rule_scope::valid_lines | pub | fn(&game_core::GameContext) -> &[game_core::LineType] | game-ai\src\plan_legacy\rule_scope.rs:13 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
</details>

⚠**미매칭 6개**: `ceil`, `format_inner`, `line_state`, `or_insert`, `push_mut`, `rustc_entry`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 4곳** (m09.ll:14237, m09.ll:33635, m09.ll:34296, m09.ll:36060) · **형제 0개** 

**`open` 7건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 재료 부재 | minion_count(i32) 의 부호 의미(아군 우세 +/적 우세 −)는 IR 에서 확정 불가 — has_big_wave 가 < -2 이고 line_clear 가 >= 2 라 '양수=아군 웨이브 우세'로 추정(game_core BrainMinionParameter 주석 미확인) | 4 |  |
| 1 | 표기 불가 | L136 `> 49` 는 소스가 `>= 50` 일 수도 있음(외연 동일, 표기 불가) | 4 |  |
| 2 | 표기 불가 | L154/L156 `line_exists && count<2` 의 한 줄 내 평가 순서(A&&B)는 column 부재로 표기 불가 — IR 은 line_exists 호출을 먼저 수행 | 4 |  |
| 3 | 미탐색 | serpen_reachable_enemy_count 내부(serpen_reachable_enemies 의 가시/비가시 집합 정의)는 담당 밖 — (visible.len, invisible.len) 반환만 확인(m05.ll 53411~53476) | 4 |  |
| 4 | 미탐색 | is_skip_serpen 의 Flexible 분기(적 탑타워 엔티티 상태 +0x68/+0x88 등)는 요지만 적음 — 별도 함수(m05.ll 44823~45032) | 4 |  |
| 5 | 표기 불가 | L95 조건의 소스 표기가 `epic_minion_buff_time[enemy] > 0` 인지 `!= 0` 인지 표기 불가(usize 라 외연 동일) | 4 |  |
| 6 | 미탐색 | calls 에 vtable 간접호출(AbstractGame::get_game_mode +0x40 / tick +0x28, divtable 확정)은 C2 대조 불가라 logic 에만 적음 | 3 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

