---

### `46` need_defense_nexus — 우리 넥서스 방어가 필요한가 — (A)포탑 다 깨진 라인으로 적 챔프가 안쪽 포탑 자리보다 넥서스에 가까이 왔거나 (B)적 미니언이 넥서스를 노리거나 (C)포탑 다 깨진 라인의 적 미니언 6마리 이상이 쌍둥이보다 넥서스에 가까우면 true

| 항목 | 값 |
|---|---|
| id | `defense_nexus__need_defense_nexus` |
| 심볼 | `_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old13defense_nexus18need_defense_nexus` |
| 소스 | `game-ai\src\plan_legacy\old\defense_nexus.rs:481` |
| IR | `m04.ll` 57179~58172행 |
| 경로·가시성 | `game_ai::plan_legacy::old::need_defense_nexus` · **pub** |
| 계층 | 레거시 플랜 |
| exe | `d3c700` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r8` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> bool
```

<details><summary>인자 5개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | _version | usize | 미사용(dbg 이름 _version). 본문 분기 없음 | 4 |
| 1 | 2 | _rnd | &mut StdRng(320B) | 미사용(readnone 속성) | 4 |
| 2 | 3 | player | &PlayerState(2528B) | info.team(+0x930) 만 읽는다. is_recent_visible 에 그대로 전달 | 4 |
| 3 | 4 | data | &OperationData(24B) | +0x0 cache, +0x8 context, +0x10 blackboard(&[Blackboard;2]) | 4 |
| 4 | 5 | _debug | &mut DebugFrameData(224B) | 미사용(readnone) | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn need_defense_nexus(_version, _rnd, player, data, _debug) -> bool

[L484] team = player.info.team(+0x930)            // >=2 → panic_bounds_check
       nexus = data.cache.nexus(+0x170)[team].unwrap()   // None → unwrap_failed
[L485] lines = rule_scope::valid_lines(context.tutorial(+0x38))   // 인라인. 6(JungleOnly) 이면 빈 슬라이스

// ── (A) 뚫린 라인으로 적 챔프가 넥서스 코앞 ──
[L485] for line in lines {
[L487]   if cache.tower(line,team)(+0x180+line*32+team*8) != None || cache.tower2(line,team)(+0x190+…) != None { continue }   // 둘 다 파괴돼야
[L492]   tower_pos = match line { Top    => team0 ? (48000,528000)  : (528000,48000),
[L494]                            Mid    => team0 ? (240000,720000) : (720000,240000),
[L495]                            Bottom => team0 ? (432000,912000) : (912000,432000) }   // 안쪽 포탑 자리 하드코딩
[L498]   enemies = cache.champions(1-team, context.pool)          // bumpalo Vec<&Entity>
[L499]   has_near_enemy_champion = enemies.iter().any(|e|
[L499]       data.blackboard(+0x10)[1-team].is_recent_visible(game, player, e)
[L500]       && dist_sq(e, nexus) <= dist_sq(tower_pos, nexus))      // IR: `ugt` 면 continue
         drop(enemies)
[L502]   if has_near_enemy_champion { return true }
       }

// ── (B) 적 미니언이 넥서스를 직접 노림 ──
[L509] if nexus.can_target(+0x6b9) {
         minions = cache.minions(1-team, pool)
[L510]   if minions.iter().any(|m| m.ty(+0x68)==1 /*Minion*/ && m.ty.Minion.info.nearest_enemy(+0x88 tag==1, +0x90) == Some(nexus.id(+0x5c0))) { return true }
       }

// ── (C) 뚫린 라인의 적 미니언 떼가 쌍둥이 포탑보다 안쪽 ──
[L516] twins = cache.twin_towers(+0x130)[team]; if twins.len(+0x18) == 0 { return false }
[L517] twin_dist_sq = dist_sq(twins[0], nexus)
[L519~521] per_line = [(Top, top_minions(+0x10)[1-team]), (Mid, mid_minions(+0x50)[1-team]), (Bottom, bottom_minions(+0x90)[1-team])]
[L523] for (line, enemy_minions) in per_line {            // ⚠여기는 valid_lines 가 아니라 line_exists 로 거른다
[L523]   if !rule_scope::line_exists(context, line) { continue }
[L526]   if tower(line,team) != None || tower2(line,team) != None { continue }
[L529~531] near_twin_count = enemy_minions.iter().filter(|m| dist_sq(m, nexus) <= twin_dist_sq).count()
[L532]   if near_twin_count > 5 { return true }
       }
[L541] return false

※ dist_sq = abs_diff(x)² + abs_diff(y)² (Entity 좌표 +0x660/+0x668, entity.rs:2158 인라인)
※ 순서 근거: !dbg 줄 485→502 → 509 → 516~532 → 541. ret phi %133 유입 8개 중 true 3개 = (A)%131 (B)%170 (C)%349
```

**`mem` 메모리 접근 23건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | team. >=2 면 panic_bounds_check(len 2). enemy = 1 - team | 4 | OK |
| 1 | OperationData | 0x0 | cache | r | &AbstractGameWithCache(8840B) | 4 | OK |
| 2 | OperationData | 0x8 | context | r | &GameContext — tutorial·pool·line_exists 인자 | 4 | OK |
| 3 | OperationData | 0x10 | blackboard | r | &[Blackboard;2]. [1-team](적팀 판) 의 is_recent_visible 로 적 챔프 최근 가시성 판정 | 4 | OK |
| 4 | GameContext | 0x0 | pool | r | &Bump — champions()/minions() 결과 Vec 할당자 | 4 | OK |
| 5 | GameContext | 0x38 | tutorial | r | TutorialType(i8). rule_scope::valid_lines 인라인: 0/7/8→[Top,Mid,Bottom], 1/3→[Bottom], 2→[Top], 4→[Mid], 5→[Mid,Bottom], 6→[] (자체 상수 @anon.97/.69/.79/.23/.96) | 4 | OK |
| 6 | AbstractGameWithCache | 0x0 | game.data_ptr / game.vtable(+0x8) | r | &dyn AbstractGame — is_recent_visible 인자 | 4 | OK |
| 7 | AbstractGameWithCache | 0x170 | nexus[] | r | Option<&Entity>(니치). None → unwrap_failed(defense_nexus.rs:484) | 4 | OK |
| 8 | AbstractGameWithCache | 0x180 | top_tower[] | r | 라인 line 의 1차 포탑 = cache + 0x180 + line*32 + team*8 (0x180 top / 0x1a0 mid / 0x1c0 bottom). null 이면 파괴 | 4 | OK |
| 9 | AbstractGameWithCache | 0x190 | top_tower2[] | r | 라인 line 의 2차 포탑 = cache + 0x190 + line*32 + team*8 (0x190/0x1b0/0x1d0). 둘 다 null 이어야 그 라인을 '뚫린 라인'으로 본다 | 4 | OK |
| 10 | AbstractGameWithCache | 0x130 | twin_towers[team] | r | bumpalo Vec<&Entity>(32B stride) — +0x18 len. len==0 이면 (C) 단계 없이 false. [0] 만 사용 | 4 | OK |
| 11 | AbstractGameWithCache | 0x10 | top_minions[1-team] | r | bumpalo Vec<&Entity>(32B stride): +0x0 ptr, +0x18 len — 적 탑 미니언 | 4 | OK |
| 12 | AbstractGameWithCache | 0x50 | mid_minions[1-team] | r | 적 미드 미니언 | 4 | OK |
| 13 | AbstractGameWithCache | 0x90 | bottom_minions[1-team] | r | 적 바텀 미니언 | 4 | OK |
| 14 | Entity(nexus) | 0x660 | x | r | 모든 거리제곱의 기준점 | 4 | OK |
| 15 | Entity(nexus) | 0x668 | y | r |  | 4 | OK |
| 16 | Entity(nexus) | 0x6b9 | can_target | r | false 면 (B) 미니언 표적 검사 자체를 건너뛴다 | 4 | OK |
| 17 | Entity(nexus) | 0x5c0 | id | r | (B) 미니언 nearest_enemy 와 비교 | 4 | OK |
| 18 | Entity(적 챔프/미니언/쌍둥이) | 0x660 | x | r | dist_sq 계산(abs_diff² 합) | 4 | OK |
| 19 | Entity(적 챔프/미니언/쌍둥이) | 0x668 | y | r |  | 4 | OK |
| 20 | Entity(적 미니언) | 0x68 | ty@tag | r | ==1 → EntityType::Minion 일 때만 nearest_enemy 를 본다 | 4 | OK |
| 21 | Entity(적 미니언) | 0x88 | ty@Minion.info.nearest_enemy@tag | r | Option<usize> 태그(Minion 페이로드 +0x8 → Minion.nearest_enemy +0x18) | 4 | OK |
| 22 | Entity(적 미니언) | 0x90 | ty@Minion.info.nearest_enemy@Some.0 | r | == nexus.id 면 '넥서스를 노리는 미니언' | 4 | OK |

**`consts` 상수 15건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 2 | 484 | 길이 | nexus[team] 배열 길이 2 — team>=2 면 panic_bounds_check | 4 |
| 1 | 1 | 498 | 인덱스 | enemy_team = 1 - team (champions/minions/blackboard/미니언 Vec 인덱스) | 4 |
| 2 | 48000 | 492 | 산출값 | Top 라인 안쪽 포탑 자리 x(team0) / y(team1) = 1.5셀. 하드코딩 좌표(실제 포탑 엔티티가 아니라 상수) | 4 |
| 3 | 528000 | 492 | 산출값 | Top 라인 안쪽 포탑 자리 y(team0) / x(team1) = 16.5셀 | 4 |
| 4 | 240000 | 494 | 산출값 | Mid 라인 안쪽 포탑 자리 x(team0) / y(team1) = 7.5셀 | 4 |
| 5 | 720000 | 494 | 산출값 | Mid 라인 안쪽 포탑 자리 y(team0) / x(team1) = 22.5셀 | 4 |
| 6 | 432000 | 495 | 산출값 | Bottom 라인 안쪽 포탑 자리 x(team0) / y(team1) = 13.5셀 | 4 |
| 7 | 912000 | 495 | 산출값 | Bottom 라인 안쪽 포탑 자리 y(team0) / x(team1) = 28.5셀 | 4 |
| 8 | 0 | 492 | 태그 | team==0 판정(select) 으로 위 좌표를 (x,y)↔(y,x) 대칭 선택 | 4 |
| 9 | 1 | 510 | 태그 | EntityType 메모리태그 1 = Minion | 4 |
| 10 | 5 | 532 | 임계 | near_twin_count > 5 (즉 6마리 이상) 이면 방어 필요 (`icmp ugt i64 %249, 5`). ⚠본문의 `shl i8 %53, 5` 는 별개 — line*32 포탑 배열 stride 접힘이라 상수 목록 대상 아님 | 4 |
| 11 | 0 | 516 | 태그 | twin_towers[team].len == 0 이면 (C) 건너뛰고 false | 4 |
| 12 | 0 | 523 | 태그 | LineType 태그 0 = Top (line_exists 인자·enemy_minions 짝) | 4 |
| 13 | 1 | 523 | 태그 | LineType 태그 1 = Mid | 4 |
| 14 | 2 | 523 | 태그 | LineType 태그 2 = Bottom | 4 |

**`knobs` 조정점 4건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | (C) 넥서스 근접 적 미니언 수 임계 | defense_nexus.rs:532 (m04.ll:57881 `icmp ugt i64 %249, 5` · Mid/Bottom 복제 57979·58135) | 5 | > 5 (6마리 이상) 이면 방어. 내리면 소수 미니언 푸시에도 방어 플랜이 뜨고, 올리면 큰 웨이브만 반응. 라인별로 3곳 복제돼 있어 전부 고쳐야 한다 | 4 | 기존 |
| 1 | (A) 적 챔프 '코앞' 기준 = 안쪽 포탑 자리 좌표 6종 | defense_nexus.rs:492~495 (m04.ll:57321~57326 select 상수) | 48000/528000, 240000/720000, 432000/912000 | 적 챔프가 이 자리보다 넥서스에 가까워야 (A) 발동. 좌표를 넥서스 쪽으로 당기면 더 늦게, 바깥으로 밀면 더 일찍 방어를 부른다. 실제 포탑 엔티티 좌표가 아니라 상수라 맵이 바뀌면 어긋난다 | 4 | 기존 |
| 2 | (A)/(C) '뚫린 라인' 조건 = 1·2차 포탑 둘 다 파괴 | defense_nexus.rs:487·526 (m04.ll:57361 `and i1 %64, %65`) | tower==None && tower2==None | 포탑 하나라도 남은 라인은 검사 대상에서 빠진다. `\|\|` 로 바꾸면 1차 포탑만 깨져도 넥서스 방어를 검토한다 | 4 | 기존 |
| 3 | (B) 게이트 nexus.can_target | defense_nexus.rs:509 (m04.ll:57378 `gep …, i64 1721`) | bool | 넥서스가 표적 가능 상태(쌍둥이 생존 중엔 보통 false 로 추정)여야 미니언 표적 검사를 한다 | 4 | 기존 |

<details><summary>`callees` 피호출자 14건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | can_target | game_core::Entity::can_target | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1477 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
| 1 | can_target | game_core::OperationData::<'a, 'b>::can_target | pub | fn(&game_core::OperationData<'a/#0, 'b/#1>, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\state\player.rs:23 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
| 2 | champions | game_core::AbstractGameWithCache::<'a, 'b>::champions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize, &'b/#1 bumpalo::Bump) -> bumpalo::collections::vec::Vec< &game_core::Entity> | game-core\src\simulation.rs:1896 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 3 | is_recent_visible | game_core::Blackboard::is_recent_visible | pub | fn(&game_core::Blackboard, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\game\blackboard.rs:346 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | line_exists | game_ai::plan_legacy::rule_scope::line_exists | pub | fn(&game_core::GameContext, game_core::LineType) -> bool | game-ai\src\plan_legacy\rule_scope.rs:24 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 5 | minions | game_core::AbstractGameWithCache::<'a, 'b>::minions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize, &'b/#1 bumpalo::Bump) -> bumpalo::collections::vec::Vec< &game_core::Entity> | game-core\src\simulation.rs:1853 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 6 | nearest_enemy | game_core::Entity::nearest_enemy | pub | fn(&game_core::Entity) -> std::option::Option<usize> | game-core\src\simulation\entity.rs:1819 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 7 | need_defense_nexus | game_ai::plan_legacy::old::need_defense_nexus | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> bool | game-ai\src\plan_legacy\old\defense_nexus.rs:481 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 8 | team | game_view::ClientData::team | pub | fn(&game_view::ClientData, usize) -> std::option::Option<std::cell::Ref< game_core::Team>> | game-view\src\logic\client\data.rs:1651 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 9 | team | game_view::ClientDatabase::team | pub | fn(&game_view::ClientDatabase, usize) -> std::option::Option<&game_core::Team> | game-view\src\logic\client\data.rs:968 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 10 | team | game_core::PlayerAiContext::<'a, 'b, 'r>::team | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:583 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 11 | tower | game_core::AbstractGameWithCache::<'a, 'b>::tower | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, game_core::LineType, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:1822 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 12 | tutorial | game_core::Strategy::tutorial | pub | fn() -> game_core::Strategy | game-core\src\simulation\strategy.rs:590 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 13 | valid_lines | game_ai::plan_legacy::rule_scope::valid_lines | pub | fn(&game_core::GameContext) -> &[game_core::LineType] | game-ai\src\plan_legacy\rule_scope.rs:13 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
</details>

⚠**미매칭 7개**: `bottom_minions`, `dist_sq`, `drop  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `mid_minions`, `top_minions`, `tower2`, `twin_towers`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 11곳** (m09.ll:8588, m09.ll:9175, m09.ll:30949, m09.ll:31785, m09.ll:32478, m09.ll:32568, m09.ll:33150, m09.ll:34184, m09.ll:34614, m09.ll:35511, m09.ll:36319) · **형제 0개** 

**`open` 6건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | Entity.can_target(+0x6b9) 이 넥서스에 대해 언제 true 가 되는지(쌍둥이 전멸 후로 추정) — game_ai IR 밖. _gcbc 의 store 지점 미탐색 | 4 |  |
| 1 | 미탐색 | Blackboard::is_recent_visible(self=blackboard[1-team], game, vtable, player, e) 내부(_gcbc) 안 봄 — '적팀 판'을 self 로 넘기는 것만 확인 | 4 |  |
| 2 | 미탐색 | (A) 의 tower_pos 6개 좌표가 실제 맵의 안쪽(1차) 포탑 좌표와 일치하는지 — MapDef 와 대조 안 함. 이름 tower_pos 와 값의 대칭성(team0↔team1 x/y 교환)만 근거 | 4 |  |
| 3 | 미탐색 | (A) 루프는 valid_lines(tutorial) 로 라인을 고르고 (C) 루프는 line_exists(context, line) 로 고른다 — 두 필터의 관계(같은 집합인지) 는 rule_scope 본체를 안 봐서 미확정. line_exists 는 별도 define(3회 호출) | 4 |  |
| 4 | 미탐색 | AbstractGameWithCache::champions / minions (team, pool) 의 반환이 '살아있는 챔프/미니언 전부'인지 필터가 있는지 — _gcbc 본체 안 봄 | 4 |  |
| 5 | 미탐색 | twin_towers[team][0] 만 쓰고 [1] 은 안 본다(첫 원소 = first/twin dbg). 두 쌍둥이 중 어느 쪽이 [0]인지 미확정 | 4 |  |

**`notes` 2건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | writes 빈 배열 = 확인된 사실(본문 store 는 bumpalo Vec 결과 스택 %6/%7 뿐) | 4 | 사실 서술 |
| 1 | _version/_rnd/_debug 세 인자는 IR 에서 한 번도 안 읽힌다 — 확인된 사실(readnone 속성) | 4 | 사실 서술 |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

