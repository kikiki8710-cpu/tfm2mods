---

### `43` BattlePlan::with_runaway — 이 전투 플랜에서 완전 도주(RunAway)를 허용하나 — TryKill 개전 1초 이내·오브젝트 전투면 불허

| 항목 | 값 |
|---|---|
| id | `battle__with_runaway` |
| 심볼 | `_RNvMs0_NtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old6battleNtB5_10BattlePlan12with_runaway` |
| 소스 | `game-ai\src\plan_legacy\old\battle.rs:367` |
| IR | `m10.ll` 22813~22868행 |
| 경로·가시성 | `game_ai::plan_legacy::old::BattlePlan::with_runaway` · **pub** |
| 계층 | 레거시 플랜 |
| exe | `dfb220` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r8` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_ai::plan_legacy::old::BattlePlan, usize, &game_core::OperationData) -> bool
```

<details><summary>인자 3개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | self | &BattlePlan(280B) | main_goal(+0x40)·start_tick(+0xc0)·main_objective(+0xff)·with_dive(+0xf6) 를 읽는다 | 4 |
| 1 | 2 | _version | usize | 이름부터 언더스코어 — 본문에서 완전히 안 쓰임(dbg_value poison) | 4 |
| 2 | 3 | data | &OperationData(24B) | cache.game(dyn).tick() 과 context.setting.tick_per_second 만 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn with_runaway(&self, _version, data) -> bool   // battle.rs:367~383
[L368] if self.main_goal(+0x40) == TryKill(0) {
[L369]   elapsed = game.tick()(vtable+0x28).saturating_sub(self.start_tick(+0xc0))
         if elapsed <= tps(+0x12f8) { return false }      // 개전 후 1초 이내엔 도주 금지 (IR: elapsed > tps 여야 아래로)
       }
[L374] match self.main_objective(+0xff) {
[L380]   Some(PressEpic(5))                                   => return !self.with_dive(+0xf6),
         Some(Morgard 0 | Serpen 1 | Defense 2 | Nexus 4)     => return false,
         None | Some(DefenseLine 3 | SplitEpic 6 | Repair 7 | Gank 8 | Dive 9 | PressTower 10 | ComebackPick 11) => return true }
[L383]
※ 부작용 없음. _version 미사용. SinglePlanBattle::with_runaway(single_battle.rs:103) 와 오프셋(start_tick 0x80→0xc0, main_objective 0x8d→0xff, with_dive 0x88→0xf6)만 다른 문자 단위 복제본(m05.ll 26839~26894).
```

**`mem` 메모리 접근 7건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | BattlePlan | 0x40 | main_goal@tag (BattlePlanGoal i64 0..4) | r | == 0 TryKill 일 때만 경과시간 검사 | 4 | OK |
| 1 | BattlePlan | 0xc0 | start_tick | r | 플랜 시작 틱 | 4 | OK |
| 2 | BattlePlan | 0xff | main_objective@tag (Option<MainObjective> 니치 i8, -1=None) | r | 5 PressEpic / 0 Morgard / 1 Serpen / 2 Defense / 4 Nexus 만 특별 취급 | 4 | OK |
| 3 | BattlePlan | 0xf6 | with_dive | r | bool. PressEpic 일 때 반환 = !with_dive | 4 | OK |
| 4 | OperationData | 0x0 | cache | r | cache+0x0/+0x8 = &dyn AbstractGame (data, vtable). vtable+0x28 = tick (divtable) | 3 | OK |
| 5 | OperationData | 0x8 | context | r | context+0x8 = &GameSetting | 4 | OK |
| 6 | GameSetting | 0x12f8 | tick_per_second | r | 경과 임계 = 1초 | 4 | OK |

**`consts` 상수 6건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 0 | 368 | 태그 | BattlePlanGoal 태그 0 = TryKill — 이 목표일 때만 '개전 직후 도주 금지' 규칙 적용 | 4 |
| 1 | 5 | 374 | 태그 | MainObjective 태그 5 = PressEpic → 반환 !with_dive | 4 |
| 2 | 0 | 374 | 태그 | MainObjective 태그 0 = Morgard → false | 4 |
| 3 | 1 | 374 | 태그 | MainObjective 태그 1 = Serpen → false | 4 |
| 4 | 2 | 374 | 태그 | MainObjective 태그 2 = Defense → false | 4 |
| 5 | 4 | 374 | 태그 | MainObjective 태그 4 = Nexus → false | 4 |

**`knobs` 조정점 3건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | TryKill 개전 직후 도주 금지 시간 | battle.rs:369 (IR m10.ll:22841 `icmp ugt i64 %17, %23` — 임계 = tps 그대로, 배수 리터럴 없음) | 1 | tps×1 = 1초. 배수를 올리면 킬 시도 후 더 오래 도주가 막혀 끈질기게 붙고, 0 으로 만들면 개전 즉시도 도주 가능(콜 개전 즉이탈 증가) | 4 | 기존 |
| 1 | 오브젝트 전투 중 도주 금지 집합 | battle.rs:374 (IR m10.ll:22847~22852 switch) | 0 | Morgard/Serpen/Defense/Nexus 일 때 false. 여기서 태그를 빼면 그 오브젝트 전투 중에도 완전 도주가 가능해진다 | 4 | 기존 |
| 2 | PressEpic 은 다이브가 아닐 때만 도주 허용 | battle.rs:380 (IR m10.ll:22866 `xor i1 %34, true`) | 1 | with_dive 면 도주 금지. 반전하면 다이브 중 도주가 열려 타워 다이브가 쉽게 중단된다 | 4 | 기존 |

<details><summary>`callees` 피호출자 6건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 1 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 2 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 3 | with_runaway | game_ai::plan_legacy::old::BattlePlan::with_runaway | pub | fn(&game_ai::plan_legacy::old::BattlePlan, usize, &game_core::OperationData) -> bool | game-ai\src\plan_legacy\old\battle.rs:367 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 4 | with_runaway | game_ai::plan_legacy::old::SinglePlanBattle::with_runaway | pub | fn(&game_ai::plan_legacy::old::SinglePlanBattle, usize, &game_core::OperationData) -> bool | game-ai\src\plan_legacy\old\single_battle.rs:103 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 5 | with_runaway | game_ai::plan_legacy::old::DeathMatchBattle::with_runaway | pub | fn(&game_ai::plan_legacy::old::DeathMatchBattle, usize, &game_core::OperationData) -> bool | game-ai\src\plan_legacy\old\death_battle.rs:885 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
</details>

⚠**미매칭 4개**: `main_goal`, `main_objective`, `start_tick`, `with_dive`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 13곳** (m10.ll:19400, m10.ll:19861, m10.ll:20300, m10.ll:20492, m10.ll:20526, m10.ll:20538, m10.ll:20702, m10.ll:20706, m10.ll:20809, m10.ll:20833, m10.ll:20851, m10.ll:21038, m10.ll:22271) · **형제 20개** (BattlePlan)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::plan_legacy::old::BattlePlan as std::clone::Clone>::clone | pub | game-ai\src\plan_legacy\old\battle.rs:96 | True | fn(&game_ai::plan_legacy::old::BattlePlan) -> game_ai::plan_legacy::old::BattlePlan |
| 1 | <game_ai::plan_legacy::old::BattlePlan as std::fmt::Debug>::fmt | pub | game-ai\src\plan_legacy\old\battle.rs:96 | True | fn(&game_ai::plan_legacy::old::BattlePlan, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 2 | game_ai::plan_legacy::old::BattlePlan::sub_goal | pub | game-ai\src\plan_legacy\old\battle.rs:178 | True | fn(&game_ai::plan_legacy::old::BattlePlan) -> game_ai::plan_legacy::old::BattleSubPlanGoal |
| 3 | game_ai::plan_legacy::old::BattlePlan::ff_birth_src | in:game_ai::plan_legacy::old::battle | game-ai\src\plan_legacy\old\battle.rs:184 | False | fn(&game_ai::plan_legacy::old::BattleSubPlanGoal) -> u8 |
| 4 | game_ai::plan_legacy::old::BattlePlan::in_active_fight | pub | game-ai\src\plan_legacy\old\battle.rs:190 | True | fn(&game_ai::plan_legacy::old::BattlePlan) -> bool |
| 5 | game_ai::plan_legacy::old::BattlePlan::new | pub | game-ai\src\plan_legacy\old\battle.rs:196 | False | fn(usize, game_ai::plan_legacy::old::BattlePlanGoal, &game_core::OperationData, &game_core::PlayerState) -> game_ai::plan_legacy::old::BattlePlan |
| 6 | game_ai::plan_legacy::old::BattlePlan::new_dive | pub | game-ai\src\plan_legacy\old\battle.rs:246 | False | fn(usize, game_ai::plan_legacy::old::BattlePlanGoal, &game_core::OperationData, &game_core::PlayerState) -> game_ai::plan_legacy::old::BattlePlan |
| 7 | game_ai::plan_legacy::old::BattlePlan::new_region | pub | game-ai\src\plan_legacy\old\battle.rs:296 | False | fn(usize, game_ai::plan_legacy::old::BattlePlanGoal, &game_core::OperationData, &game_core::PlayerState, u64, u64, u64) -> game_ai::plan_legacy::old::BattlePlan |
| 8 | game_ai::plan_legacy::old::BattlePlan::goal | pub | game-ai\src\plan_legacy\old\battle.rs:341 | False | fn(&game_ai::plan_legacy::old::BattlePlan) -> game_core::BigGoal |
| 9 | game_ai::plan_legacy::old::BattlePlan::set_main_objective | pub | game-ai\src\plan_legacy\old\battle.rs:349 | True | fn(&mut game_ai::plan_legacy::old::BattlePlan, std::option::Option<game_ai::plan_legacy::team_plan::MainObjective>) |
| 10 | game_ai::plan_legacy::old::BattlePlan::return_to_objective | in:game_ai::plan_legacy::old::battle | game-ai\src\plan_legacy\old\battle.rs:354 | False | fn(&game_ai::plan_legacy::old::BattlePlan) -> bool |
| 11 | game_ai::plan_legacy::old::BattlePlan::current_focus | pub | game-ai\src\plan_legacy\old\battle.rs:358 | True | fn(&game_ai::plan_legacy::old::BattlePlan) -> std::option::Option<usize> |
| 12 | game_ai::plan_legacy::old::BattlePlan::with_runaway | pub | game-ai\src\plan_legacy\old\battle.rs:367 | False | fn(&game_ai::plan_legacy::old::BattlePlan, usize, &game_core::OperationData) -> bool |
| 13 | game_ai::plan_legacy::old::BattlePlan::v3_beyond_enemy_line | in:game_ai::plan_legacy::old::battle | game-ai\src\plan_legacy\old\battle.rs:387 | False | fn(&game_ai::plan_legacy::old::BattlePlan, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity) -> bool |
| 14 | game_ai::plan_legacy::old::BattlePlan::no_enemy_idle_watchdog | in:game_ai::plan_legacy::old::battle | game-ai\src\plan_legacy\old\battle.rs:406 | False | fn(&mut game_ai::plan_legacy::old::BattlePlan, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity) -> bool |
| 15 | game_ai::plan_legacy::old::BattlePlan::update | pub | game-ai\src\plan_legacy\old\battle.rs:427 | False | fn(&mut game_ai::plan_legacy::old::BattlePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) |
| 16 | game_ai::plan_legacy::old::BattlePlan::update_v32 | in:game_ai::plan_legacy::old::battle | game-ai\src\plan_legacy\old\battle.rs:748 | False | fn(&mut game_ai::plan_legacy::old::BattlePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) |
| 17 | game_ai::plan_legacy::old::BattlePlan::next_plan | pub | game-ai\src\plan_legacy\old\battle.rs:2034 | True | fn(&game_ai::plan_legacy::old::BattlePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &mut game_core::DebugFrameData) -> std::option::Option<game_ai::plan_legacy::types::BigPlan> |
| 18 | game_ai::plan_legacy::old::BattlePlan::is_end | pub | game-ai\src\plan_legacy\old\battle.rs:2039 | False | fn(&game_ai::plan_legacy::old::BattlePlan) -> bool |
| 19 | game_ai::plan_legacy::old::BattlePlan::sub_plan | pub | game-ai\src\plan_legacy\old\battle.rs:2043 | False | fn(&game_ai::plan_legacy::old::BattlePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> game_ai::plan_legacy::sub_plan::SubPlan |

**`open` 5건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 표기 불가 | main_objective 의 나머지 태그(None·3·6·7·8·9·10·11)가 default 로 true 인 것은 IR 로 확정. 소스가 `_ => true` 인지 열거인지는 표기 불가 | 4 |  |
| 1 | 표기 불가 | TryKill 가지에서 elapsed <= tps 의 등호 방향: IR 은 `elapsed > tps → 계속` 이므로 정확히 `elapsed <= tps → false`. 소스가 `< tps+1` 류인지는 표기 불가(외연 동일) | 4 |  |
| 2 | 미탐색 | game.tick() 은 vtable+0x28 (divtable 정적 vtable 기준). 런타임 구현체는 미확정 | 3 |  |
| 3 | 미탐색 | 호출자(어느 판정이 이 bool 을 소비하는지 — _docs 262·410행의 'RunAway 계측' 과의 관계)는 범위 밖·미탐색 | 4 |  |
| 4 | 미탐색 | PressEpic 가지의 `xor i1 %34, true`(= !with_dive, src 380) 는 bool 리터럴이라 constants 에 못 싣는다(qcspec C1 이 True 를 상수로 안 잡음). logic/knobs 에만 실었다 | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

