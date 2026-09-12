공통 전제(오프셋은 전부 `tcxdict` 확인값): `OperationData`(24B) = `{cache:&AbstractGameWithCache@0, context:&GameContext@8, blackboard@0x10}` · `GameContext.tutorial@0x38`(TutorialType 0=None 1=First 2=TopSolo 3=Bottom 4=MidSolo 5=MidBottom 6=JungleOnly 7=Line 8=Total) · `GameContext.setting→GameSetting.tick_per_second@0x12f8` · `PlayerState.info.team@0x930(2352)` · `info.position@0x9c0(2496)` · `info.parameter@0x180`(AthleteParameter) · `AbstractGameWithCache.player_champion@0x1e0(480) = [[Option<&Entity>;5];2]` · `Entity.id@0x5c0(1472)`, `stat_cached.hp@0x628(1576)`=최대HP, `hp@0x670(1648)` · `MobaMode.jungle_runner.epic{live_list.ptr@0x1a0,len@0x1a8,next_respawn_tick@0x1b0}` / `.serpen{@0x1d0,@0x1d8,@0x1e0}` · dyn `AbstractGame` vtable 슬롯(divtable): `0x28 tick` / `0x40 get_game_mode`(→`GameMode` 태그0=Moba, 페이로드 `&MobaMode`@+8) / `0xf8 is_visible(team, entity_id)` / `0x1f0 get_entity_by_id(id)`.

---

### #21 `v3_epic_group_line` — 모르가드 운용 전략(Gather/Split14/Split131)에 따라 "4~5명 그룹이 압박할 라인"을 고른다
- **시그니처**(tcx): `fn(strategy: MorgardUseStrategy, player: &PlayerState, op: &OperationData) -> Option<LineType>` · `pub` · `epic.rs:814`(본문 815~822) · IR `m09.ll:64337~64420` · exe `0xdea4a0` · 계층 = 레거시 플랜 헬퍼(`old::epic`)
- **입력**: `%0` = `MorgardUseStrategy`(8B 통째, 하위 u32가 니치 태그: **5=Gather / 6=Split14(상위 u32=position) / 그 외=Split131(0x0 position1, 0x4 position2)**) · `%1` player(team만 사용) · `%2` op
- **읽는 상태**: `player.info.team@0x930` · `op.cache`/`op.context` · (Split14 인라인 `v3_split14_far_line`) `game.get_game_mode()→MobaMode.jungle_runner.epic.next_respawn_tick@0x1b0`, `.serpen.next_respawn_tick@0x1e0`
- **쓰는 상태**: 없음(순수)
- **판정 흐름**:
```
tag = strategy.low_u32; k = tag > 4 ? tag-5 : 2        // 815 — 0=Gather 1=Split14 2=Split131
match k {
  0 /*Gather*/  => return v3_group_press_line(team, cache, ctx, skip=None)          // 816
  1 /*Split14*/ => far = v3_split14_far_line(op):                                    // 817 (854~857 인라인)
                     if get_game_mode() != Moba → Top
                     else if epic.next_respawn_tick > serpen.next_respawn_tick → Top(0) else Bottom(2)
                   return v3_group_press_line(team, cache, ctx, skip=Some(far))
  2 /*Split131*/=> if line_exists(ctx, Mid) → return Some(Mid)                        // 819
                   else return v3_group_press_line(team, cache, ctx, skip=None)     // 822
}
반환 i8: -1=None / 0=Top / 1=Mid / 2=Bottom
```
- **상수/노브**: `4`/`5`(니치 태그 경계, 구조상 상수) · `Mid(1)`(Split131의 고정 선호 라인) · far_line의 Top/Bottom 선택(추정: 에픽이 세르펜보다 늦게 리스폰 = "다음 오브젝트=세르펜(바텀 측)" → 반대편 Top이 먼 라인). ⚠참고: 피호출자 `v3_group_press_line` 내부의 정렬 보조키 `obj_line`은 같은 비교의 **반대값**(`m09.ll:64472` epic>serpen ? Bottom : Top)이다 — far_line ≠ obj_line. 기존 `_spec` 서술의 "Split14 skip=obj_line" 표기는 이름만 다를 뿐 값은 far_line(`select …, i8 0, i8 2`)이니 문서화 시 혼동 주의.
- **호출하는 함수**: `old::epic::v3_group_press_line`(m09.ll:64423 — 후보 라인을 tutorial별 정적 슬라이스에서 뽑아 `skip` 제외 후 적 타워 상태 점수·Mid·obj_line 사전식 최대) · `old::epic::v3_split14_far_line`(인라인) · `rule_scope::line_exists(ctx, LineType)` · `AbstractGame::get_game_mode`(vtable)
- **미확정**: far_line의 "먼"의 지리적 근거(에픽=탑측/세르펜=바텀측)는 맵 데이터 미확인 → 추정. `v3_group_press_line` 내부 점수표는 본 범위 밖(기존 spec 확정분 참조).

---

### #22 `v3_epicops_repair_need` — 아군 생존/체력 분포를 적과 비교해 "지금 수리(Repair) 가야 하나"를 3단(0/1/2)으로 판정
- **시그니처**(tcx): `fn(player: &PlayerState, op: &OperationData, plan: &BigPlan) -> u8` · `in:game_ai` · `epic.rs:833`(본문 834~849) · IR `m09.ll:64975~65551`(argpromote: `(team: i64, cache: *AbstractGameWithCache, plan: &BigPlan[384B])`) · exe `0xdeaa70`(internal fastcc → rlib 패치) · 계층 = 레거시 EPIC-OPS 헬퍼
- **입력**: `%0` team(=player.info.team, 호출부 #18에서 로드) · `%1` cache · `%2` plan(`BigPlan::goal()` 태그만 사용)
- **읽는 상태**: `cache.player_champion[team][0..5]@0x1e0+team*40` · `[1-team]` · 각 `Entity.hp@0x670`, `stat_cached.hp@0x628` · `plan.goal()→BigGoal` 태그(5=Battle)
- **쓰는 상태**: 없음
- **판정 흐름**:
```
assert team < 2                                              // 834 (bounds panic)
mine   = player_champion[team]     ; n_mine  = non-null 수     // 834
enemy  = player_champion[1-team]   ; n_enemy = non-null 수     // 835
in_battle = (plan.goal() tag == 5 /*BigGoal::Battle*/)        // 837
if !in_battle {
  hurt = count(mine, e => e.hp*100 / e.max_hp < 41)          // 838  (max_hp==0 → div-by-zero panic)
  if n_mine <= n_enemy && hurt > 1 { return 1 }              // 839
}
healthy = count(mine, e => e.hp*100 / e.max_hp > 29)         // 844
if healthy < n_enemy && healthy < n_mine { return 2 }        // 845
return 0                                                     // 849
```
- **상수/노브**: `41`(hurt 임계: HP < 41% = "40% 이하") · `29`(healthy 임계: HP > 29% = "30% 이상") · `hurt > 1`(2명 이상) · `BigGoal::Battle(5)` — 교전 중이면 1등급 판정 자체를 건너뜀. 임계 상향 = 수리 잦아짐(추정: #18에서 1/2 모두 objective=Repair로 이어져 버프창 진행을 끊는다).
- **반환 의미(호출부 #18 기준)**: `1` → `objective=MainObjective::Repair(7)` + `chats.push(Chat::Repair(0))`(태그 23) → true / `2` → objective만 Repair → true / `0` → 다음 단계
- **호출하는 함수**: `plan_legacy::types::BigPlan::goal` · `AbstractGameWithCache::iter_champions`(인라인)
- **미확정**: 없음(전 경로 독해). 1과 2의 "채팅 유무" 차이의 설계 의도는 소스 주석 부재.

---

### #23 `is_object_being_taken_by_enemy` — 오브젝트(모르가드/세르펜)가 "지금 적에게 먹히는 중"인지: 캠프 최근 시야 + 몹 피해·가시 + 적 예상 처치시간 20초 이내
- **시그니처**(tcx): `fn(player: &PlayerState, op: &OperationData, goal: &GoalData, team_plan: &TeamPlan, target: WavePriorityObject) -> bool` · `in:game_ai` · `objective_helpers.rs:313`(본문 314~332) · IR `m15.ll:53833~54066` · exe `0xec9bf0` · 계층 = team_plan 공용 헬퍼(호출자 6곳, #18은 `target=Serpen(i1 true)`)
- **입력**: `%4` = `WavePriorityObject`(1B enum, 0=Morgard/1=Serpen — i1로 전달)
- **읽는 상태**: `ctx.setting.tick_per_second@0x12f8` · `ctx.tutorial@0x38` · `team_plan.obj_spawn.epic_camp_last_visible_tick@0x80` / `serpen_camp_last_visible_tick@0x88` · `goal.epic.epic_enemy_tick@0x88` / `goal.serpen.epic_enemy_tick@0xc0`(= **적팀이 그 몹을 죽이는 데 걸릴 예상 틱(duration)** — 기존 spec 확정) · `game.tick()` · `MobaMode.jungle_runner.{epic|serpen}.live_list[0]` · `Entity.id/hp/stat_cached.hp` · `player.info.team@0x930`
- **쓰는 상태**: 없음
- **판정 흐름**:
```
tps = ctx.setting.tick_per_second                                   // 314
match target {
  Morgard => if (tutorial-1) <u 6 { return false }   // 317 morgard_exists: tutorial ∈ {1..=6} 이면 에픽 미스폰
             seen  = obj_spawn.epic_camp_last_visible_tick + tps >= game.tick()     // 318
             kill  = goal.epic.epic_enemy_tick                                       // 319
             camp  = as_moba().unwrap().jungle_runner.epic                           // 320
  Serpen  => if tutorial ∉ {0,5,7,8} { return false }        // 326 serpen_exists
             seen  = obj_spawn.serpen_camp_last_visible_tick + tps >= game.tick()   // 327
             kill  = goal.serpen.epic_enemy_tick                                     // 328
             camp  = as_moba().unwrap().jungle_runner.serpen                         // 329
}
id  = camp.live_list.get(0)?           else false            // 320/329
e   = game.get_entity_by_id(*id)?      else false            // 321/330 closure
dmg = game.is_visible(team, e.id) && e.hp < e.stat_cached.hp  // 322/331 closure (피해 입은 채 보임)
return seen && dmg && kill <= tps*20                          // 323/332
```
- **상수/노브**: `tps*1`(캠프를 최근 1초 내 봤어야 함) · `tps*20`(적이 20초 안에 처치 가능해야 "먹히는 중") · tutorial 집합(에픽 {0,7,8} / 세르펜 {0,5,7,8}). 20 상향 = 더 이르게 "빼앗김" 경보(추정).
- **호출하는 함수**: `rule_scope::morgard_exists`/`serpen_exists`(인라인, 내부 `runner::spawn_epic`/`spawn_serpen`) · `Game::as_moba`(인라인, 비Moba면 `unwrap_failed` 패닉) · vtable `tick`/`get_game_mode`/`get_entity_by_id`/`is_visible`
- **미확정**: `is_visible`의 2번째 인자가 entity id인 것은 시그니처 `(usize, usize)`+IR(`e.id@0x5c0` 전달)로 확인, 셀 기반이 아님. 내부 시야 규칙은 game_core 소관.

---

### #24 `v3_serpen_contest_clear_win` — 세르펜 근처 도달 가능한 적 전원 vs 건강한 아군으로 교전 예측을 돌려 "확실히 이기는 싸움(Commit)"인지
- **시그니처**(tcx): `fn(version: usize, player: &PlayerState, op: &OperationData, team_plan: &TeamPlan) -> bool` · `pub` · `serpen.rs:52`(본문 53~65) · IR `m05.ll:53103~53408` · exe `0xd666c0` · 계층 = 레거시 EPIC-OPS 세르펜 응징 헬퍼
- **입력**: `%0` version(본문에서 안 쓰고 `resolve_fight` 1번 인자로 통과 — #18의 2번 인자 그대로) · `%1` player · `%2` op · `%3` team_plan(→`serpen_reachable_enemies`에 전달)
- **읽는 상태**: `player.info.team@0x930`(bounds<2) · `info.position@0x9c0`(→as_index) · `cache.player_champion[team][pos]` · `info.parameter@0x180`(judge_accuracy) · `ctx.pool@0`(bumpalo) · `iter_champions(team)`의 각 `Entity.hp/stat_cached.hp`
- **쓰는 상태**: 없음(bump 할당만)
- **판정 흐름**:
```
me = cache.player_champion[team][position.as_index()]?  else return false        // 53
(vis, hidden) = serpen_reachable_enemies(player, op, team_plan)                 // 54
enemies = vis; enemies.extend(hidden)                                            // 55
if enemies.is_empty() { return true }                                            // 57  (적 없음 = 무혈승)
allies = iter_champions(team).filter(|e| e.hp*100 / e.max_hp > 39).collect_in(pool) // 60~62 (closure#0 m05.ll:62463)
acc  = player.info.parameter.judge_accuracy()                                    // 63
pred = fight_model::resolve_fight(version, op, me, &allies, &enemies, 0i8, None, acc) // 63
return pred.line /*FightPrediction@0x38*/ == FightLine::Commit(0)               // 64
```
- **상수/노브**: `39`(아군 전력 산입 임계: HP > 39% = 40% 이상) · `0i8`/`None`(resolve_fight 6·7번 인자 고정값 — 의미 미확정) · 피호출자 상수(기존 spec 확정): 도달거리 여유 `150000`, 적 건강 하한 `hp*100/max > 49`. `39` 하향 = 더 많은 저체력 아군을 전력에 넣어 Commit 잦아짐(추정).
- **호출하는 함수**: `old::serpen::serpen_reachable_enemies`(m05.ll:52849 — 반환 (가시 적, 비가시-도달가능 적); `MapDef::camp_pos(map, JungleType 5, is_blue)` 사용) · `old::fight_model::resolve_fight` · `AthleteParameter::judge_accuracy` · `Position::as_index`(인라인) · `AbstractGameWithCache::iter_champions`(인라인)
- **미확정**: `resolve_fight`의 `i8=0`·`Option<&Entity>=None` 인자 의미(fight_model.rs:310 소관) · `FightLine` 결정 규칙(같은 파일 소관, Commit/CommitAfterJoin/Disengage/Hold 중 0만 통과).

---

### 4함수 관계 — #18 `TeamPlan::v3_epicops_buff_window`(`m09.ll:6879~`)의 호출 순서
1. `m09.ll:6895`(epic.rs:635) **#22** `v3_epicops_repair_need(team, cache, plan)` 최우선 → `1`이면 `objective=Repair(@0x41f=7)`+`Chat::Repair` push 후 **return true**(6905~6944) / `2`면 objective만 Repair 후 **return true**(6946) / `0`이면 계속.
2. `m09.ll:6902`(:651) **#23** `is_object_being_taken_by_enemy(player, op, goal, self, Serpen)` — false면 4로.
3. `m09.ll:6956`(:652) true일 때만 **#24** `v3_serpen_contest_clear_win(version, player, op, self)` → true면 `eo_serpen_punish_issues@0x410++`, `objective=Serpen{phase:Setup, with_battle:true}`(@0x41f..0x421 = 1,1,1), `Chat::SerpenSetup(0)`(태그 25) push → **return true**(6971~7018); false면 4로.
4. `m09.ll:6964~6967`(:664~665) `strategy = player.strategy(...)` → **#21** `v3_epic_group_line(strategy.morgard_use(@+4), player, op)` → `None(-1)`이면 **return false**(7052); `Some(line)`이면 5명 슬롯에 `v3_epic_formation_role` 반복(7065~7186)으로 발표자 선정 후 `Chat::Press/PressChange`·`v3_press_chat_line` 갱신 경로.
5. 즉 우선순위 = **수리 필요(#22) > 세르펜 빼앗김 응징(#23∧#24) > 그룹 압박 라인 발표(#21)**; #23·#24는 AND 관계(둘 다 true여야 세르펜 셋업), #21은 그 둘이 실패했을 때의 기본 경로다.

판정 상수(41/29/39/20 · tutorial 집합 · 태그값)는 `MIG\_spec\resolved_2026-09-10.json` 의 동일 함수 확정 서술과 전부 일치(2026-09-13 IR 독해 · ev4 · 1회 독해 · 반증검증 라운드 미경유).
