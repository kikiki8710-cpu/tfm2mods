---

### `176` SmallActionTrace::get_input — 추격(Trace) 소액션의 매 틱 입력 — 대상까지의 최소 사거리 표적점을 목표로 잡고, 타워/우물/적존 회피 정책의 PathFinder 를 (재)생성·갱신한 뒤 타워 escape 판정이면 RunAway 입력, 아니면 경로 입력을 낸다

| 항목 | 값 |
|---|---|
| id | `SmallActionTrace__get_input` |
| 심볼 | `_RNvMs_NtNtCshdEBA0ozCnw_7game_ai12small_action5traceNtB4_16SmallActionTrace9get_input` |
| 소스 | `game-ai\src\small_action\trace.rs:165` |
| IR | `m02.ll` 63325~64494행 |
| 경로·가시성 | `game_ai::SmallActionTrace::get_input` · **in:game_ai** |
| 계층 | 기타 |
| exe | `cce210` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r14` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&mut game_ai::SmallActionTrace, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &mut game_core::DebugFrameData) -> std::option::Option<game_core::Input>
```

<details><summary>인자 8개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | Option<Input>(32B) | None 경로 = `store i64 -1` 만(+8..+32 미기록). Some 경로 = 콜리(RunAway::get_input 64364 / PathFinder::get_input 64415)가 32B 를 씀 | 4 |
| 1 | 1 | self | &mut SmallActionTrace(152B) | writes 전수 = writes 필드 | 4 |
| 2 | 2 | version | usize | version<2 게이트(63725: lethal_chase 계산 생략 · 64320/64349: escape 커밋). 콜리 new_target_with_policy/update_path/path_needs_tower_escape/RunAway::get_input 에 전달, 클로저 캡처(&%26) | 4 |
| 3 | 3 | rnd | &mut StdRng(320B) | 본문 직접 write 없음 — new_target_with_policy/update_path/RunAway::get_input 에 &mut 전달 | 4 |
| 4 | 4 | player | &PlayerState(2528B) | info.team(+0x930)·info.position(+0x9c0) | 4 |
| 5 | 5 | data | &OperationData(24B) | +0 cache(game 팻포인터·player_champion·iter_towers_without_nexus) / +8 context(→ +8 setting.tick_per_second · +0x20 map.bushes) | 4 |
| 6 | 6 | positioning_score | &PositioningScoreData(2760B) | 본문에서 안 읽음 — 클로저 캡처(64067·64168) 및 RunAway::get_input 전달 | 4 |
| 7 | 7 | debug | &mut DebugFrameData(224B) | 본문 직접 write 없음 — RunAway::get_input 에만 전달(64364) | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn get_input(&mut self, version, rnd, player, data, positioning_score, debug) -> Option<Input>
// L166
target = game.get_entity_by_id(self.target)?          // None → return None
// L167
champ = cache.player_champion[player.info.team][player.info.position].unwrap()
// L169~171
radius_sum = champ.radius() + target.radius()
atk = champ.attack_effect.unwrap()
min_range = atk.range(champ) /*range+growth*(level-1)+stat_buff.range*/ + atk.range_adjust(champ, target) + radius_sum
// L174~187
if !self.attack_range_only {
    if let Some(s) = champ.skill_effect  && s.target.check(champ, target) { min_range = min(min_range, s.range(champ)+s.range_adjust(champ,target)+radius_sum) }   // L175~178
    if let Some(s2) = champ.skill2_effect() /*level>2*/ && s2.target.check(champ, target) { min_range = min(min_range, s2.range(champ)+s2.range_adjust(champ,target)+radius_sum) }   // L184~187
}
// L194
if target.is_visible_from(champ) {
    self.goal_x = target.x; self.goal_y = target.y                       // L217~218
} else {
    dx = champ.x - target.x; dy = champ.y - target.y; sz = max(isqrt(dx²+dy²), 1)   // L195~197
    txi = min(target.x/32000, 29); tyi = min(target.y/32000, 29)          // L201~202
    if map.bushes[tyi][txi] == 0 {                                        // L204
        d = min_range.saturating_sub(self.attack_range_margin)            // L199
        (x,y) = Game::adjust_position(map, setting, target.x + d*dx/sz, target.y + d*dy/sz)   // L209~211 (sdiv)
        self.goal_x = x; self.goal_y = y                                  // L212
    } else { self.goal_x = target.x; self.goal_y = target.y }             // L205 (대상이 부시 안)
}
// L222
use_tower_avoid_path = self.avoid_unnecessary_tower || target.ty != Champion(13)
// L227~231
lethal_chase = version >= 2 && v3_lethal_tower_hp(data, player, champ)
chase_policy = if lethal_chase { SolvePolicy{deadly_cells: v3_deadly_edge_cells(version, player, data), direct_cross: false} } else { SolvePolicy{2, true} }
// L238
if use_tower_avoid_path {
    strict = self.avoid_unnecessary_tower                                                          // L239·241
    key = if lethal_chase {"trace_lethal_tower"} else if strict {"trace_avoid_tower"} else {"trace_objective_avoid_tower"}
    tower_dodge = if strict { TowerDodgeContext::new_unnecessary_tower_avoid(version, player, data, goal) } else { new_tower_avoid_v3(version, player, data, goal) }   // L243/245
    // L247~248
    if !(self.path_finder is Some && pf.key == key) {
        self.path_finder = Some(PathFinder::new_target_with_policy(rnd, ctx, version, key, chase_policy, champ.x, champ.y, goal_x, goal_y, closure(&lethal_chase, &tower_dodge, &version, &strict, player, data, positioning_score)))   // 구 Box 해제
    }
    self.path_finder.as_mut()?.update_path(rnd, ctx, champ.x, champ.y, goal_x, goal_y, closure(동일 캡처))   // L261
} else {
    avoid_zone = self.avoid_enemy_zone; enemy_team = 1-team; champ_radius = champ.radius()               // L275~277
    zone_ok = (&avoid_zone, data, &enemy_team, &champ_radius)                                            // L278 캡처
    lethal_dodge = if lethal_chase { Some(new_tower_avoid_v3(version, player, data, goal)) } else { None }   // L292~295
    well_key = if lethal_chase {"trace_avoid_well_lethal"} else {"trace_avoid_well"}                       // L297
    if !(self.path_finder is Some && pf.key == well_key) {                                                // L298
        self.path_finder = Some(PathFinder::new_target_with_policy(rnd, ctx, version, well_key, chase_policy, champ.x, champ.y, goal_x, goal_y, closure(&lethal_dodge, &version, player, &zone_ok)))   // L299
    }
    self.path_finder.as_mut()?.update_path(rnd, ctx, champ.x, champ.y, goal_x, goal_y, closure(동일 캡처))   // L315
}
// L334~342
raw_escape = if !self.dive_ignore_tower_escape || lethal_chase { path_needs_tower_escape(version, player, data, self.path_finder.as_ref()?) }
             else { self.escape_commit_until = 0; self.last_escape = None; false }   // L335~336
// L354~361
tower_shooting_me = cache.iter_towers_without_nexus(1-team).any(|t| t.ty==Tower && t.Tower.info.nearest_enemy.is_some_and(|(_,id)| id == champ.id))   // closure$5 [aux m06.ll:34359]
    || (champ.last_attacked_from.is_some_and(|id| game.get_entity_by_id(id).is_some_and(|e| e.team == Player(1-team) && e.ty == Tower && champ.hp*100/max(champ.stat_cached.hp,1) < 66)))   // L360~361 closure$9 (any 이 true 면 단락)
tick = game.tick()   // L363
// L364~379
if raw_escape {
    if version < 2 || tower_shooting_me { self.escape_commit_until = tick + tps*3/2; escape = true }   // L370
    else { self.avoid_unnecessary_tower = true; self.escape_commit_until = 0; escape = false }         // L366~367
} else { escape = tick < self.escape_commit_until }                                                   // L374
self.last_escape = Some(escape)                                                                       // L379
// L381~383
if escape { return SmallActionRunAway::new_with_skill(data, player, 5, true).get_input(version, rnd, player, data, positioning_score, debug) }
// L386~390
pf = self.path_finder.as_mut()?
safe = if champ.stat_cached.move_speed > target.stat_cached.move_speed || dist²(champ,target) < 14400000001 { SafeMoveWithSkill::Safe(1) } else { Must(2) }
return pf.get_input(player, data, safe, false)   // L389 (Input → Some 로 그대로 sret)
```

**`mem` 메모리 접근 52건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | SmallActionTrace(self) | 0x60 | target | r | 63371~63372 → game.get_entity_by_id(target)(vtable+0x1f0, 63373~63375) | 4 | OK |  |
| 1 | SmallActionTrace(self) | 0x91 | attack_range_only | r | 63497~63500 (L174): false 면 스킬/스킬2 사거리로 min_range 축소 | 4 | OK |  |
| 2 | SmallActionTrace(self) | 0x78 | attack_range_margin | r | 63685~63688 (L199) min_range.saturating_sub(margin) | 4 | OK |  |
| 3 | SmallActionTrace(self) | 0x90 | avoid_unnecessary_tower | r | 63674~63677 (L222) · 63766~63768 (L239·241 strict_tower_avoid) | 4 | OK |  |
| 4 | SmallActionTrace(self) | 0x93 | avoid_enemy_zone | r | 63754~63756 (L275) → zone_ok 캡처 | 4 | OK |  |
| 5 | SmallActionTrace(self) | 0x55 | path_finder@tag | r | i8 2 = None (63825~63827 · 64012~64014 · 63951 · 64144 · 64196 · 64356) | 4 | OK |  |
| 6 | SmallActionTrace(self) | 0x10 | path_finder (PathFinder 72B: +0 key.ptr/+8 key.len/+0x28 path Box/+0x30 verdict Box) | r | key 비교 memcmp(63935~63944 · 64128~64137) · 재생성 시 구 Box 해제(63909·63931: 1120B/70B) · update_path/get_input/path_needs_tower_escape 인자 | 4 | OK |  |
| 7 | SmallActionTrace(self) | 0x94 | dive_ignore_tower_escape | r | 63991~63994 (L334) | 4 | OK |  |
| 8 | SmallActionTrace(self) | 0x88 | escape_commit_until | r | 64310~64312 (L374) tick < until | 4 | OK |  |
| 9 | PlayerState | 0x930 | info.team | r | 63384~63386 bounds 2; 적 팀 1-team(63758 · 64204) | 4 | OK |  |
| 10 | PlayerState | 0x9c0 | info.position@tag | r | 63399~63401 | 4 | OK |  |
| 11 | OperationData | 0x0 | cache | r | 63367 | 4 | OK |  |
| 12 | OperationData | 0x8 | context | r | 63654~63655 · 63861 등 | 4 | OK |  |
| 13 | AbstractGameWithCache | 0x0 | game(dyn AbstractGame 팻포인터) | r | 63368~63370; vtable+0x1f0 get_entity_by_id(63373) · vtable+0x28 tick(64236~64238) | 4 | OK |  |
| 14 | AbstractGameWithCache | 0x1e0 | player_champion[team][position] | r | 63402~63405 unwrap(63431) | 4 | OK |  |
| 15 | GameContext | 0x20 | map(&MapDef) | r | 63656~63657 | 4 | OK |  |
| 16 | GameContext | 0x8 | setting | r | 63700~63701 adjust_position 인자 · 64325~64328 tick_per_second | 4 | OK |  |
| 17 | MapDef | 0x1c98 | bushes[tyi][txi] | r | [30][30] usize (63658~63662, L204): 0 이면 표적점 계산, 아니면 대상 좌표 그대로 | 4 | OK |  |
| 18 | GameSetting | 0x12f8 | tick_per_second | r | 64327~64328 escape 커밋 = tick + tps*3/2 | 4 | OK |  |
| 19 | Entity(champ / target) | 0x470 | stat_buff_cached.radius_mult | r | Entity::radius 인라인 63420~63437 · 63442~63460 · 63761~63782 | 4 | OK |  |
| 20 | Entity(champ / target) | 0x680 | radius | r | 63426~63427 · 63449~63450 | 4 | OK |  |
| 21 | Entity(champ) | 0x4c0 | attack_effect@tag | r | 63467~63470 None → unwrap_failed(63504, L170) | 4 | OK |  |
| 22 | Entity(champ) | 0x490 | attack_effect@Some.0 (Effect) | r | 63475 range_adjust self; range +0x4a0(63478)·growth +0x4a8(63480)·stat_buff.range +0x438(63486)·level +0x5c8(63482) | 4 | OK |  |
| 23 | Entity(champ) | 0x4f8 | skill_effect@tag | r | 63510~63513 (L175) | 4 | OK |  |
| 24 | Entity(champ) | 0x4c8 | skill_effect@Some.0 (Effect) | r | 63509; target +0x4f0(63527) check · range_adjust(63550) | 4 | OK |  |
| 25 | Entity(champ) | 0x500 | skill2_effect@Some.0 (Effect) | r | Entity::skill2_effect 인라인 level>2 (63535~63537), 태그 = 선택ptr+0x30(63539~63541) · target 선택ptr+0x28(63563) · range_adjust(63573) | 4 | OK |  |
| 26 | Entity(champ) | 0x5c8 | level | r | 63482 · 63535 | 4 | OK |  |
| 27 | Entity(champ) | 0x0 | team@tag | r | is_visible_from 인라인(63520~63522, L194): Neutral 이면 가시 검사 생략 | 4 | OK |  |
| 28 | Entity(champ) | 0x8 | team@Player.0 | r | 63584~63586 → target.visible_state[team] | 4 | OK |  |
| 29 | Entity(target) | 0x38 | visible_state[team]@tag | r | 63593~63596 (data.rs:122 is_visible): 0=Visible | 4 | OK |  |
| 30 | Entity(champ / target) | 0x660 | x | r | 63604 · 63617~63620 · 63863 · 64381~64390 등 | 4 | OK |  |
| 31 | Entity(champ / target) | 0x668 | y | r | 63610 · 63625~63628 등 | 4 | OK |  |
| 32 | Entity(target) | 0x68 | ty@tag | r | 63715~63717 (L222) != 13(Champion) → use_tower_avoid_path | 4 | OK |  |
| 33 | Entity(champ) | 0x28 | last_attacked_from@tag | r | 64225~64231 (L360) Some 이면 +0x30 id 로 get_entity_by_id | 4 | OK |  |
| 34 | Entity(champ) | 0x30 | last_attacked_from@Some.0 | r | 64243~64248 | 4 | OK |  |
| 35 | Entity(attacker e) | 0x0 | team@tag / +0x8 idx | r | 64272~64278 (L361 closure$9): e.team == Player(1-team) | 4 | OK |  |
| 36 | Entity(attacker e) | 0x68 | ty@tag | r | 64279~64281 == 2(Tower) | 4 | OK |  |
| 37 | Entity(champ) | 0x670 | hp | r | 64286~64287 hp*100/max(stat_cached.hp,1) < 66 | 4 | OK |  |
| 38 | Entity(champ) | 0x628 | stat_cached.hp | r | 64288~64292 | 4 | OK |  |
| 39 | Entity(champ / target) | 0x640 | stat_cached.move_speed | r | 64369~64373 (L390) champ > target | 4 | OK |  |
| 40 | Entity(tower) | 0x68 | ty@tag == 2 | r | aux m06.ll:34464~34466 (closure$5, L355) | 4 | OK |  |
| 41 | Entity(tower) | 0x88 | ty@Tower.info.nearest_enemy@tag | r | aux 34468~34476 Some(1) | 4 | OK |  |
| 42 | Entity(tower) | 0x98 | ty@Tower.info.nearest_enemy@Some.0.1 | r | aux 34481~34487 == champ.id(+0x5c0, 34421~34422) (L356) | 4 | OK |  |
| 43 | Entity(champ) | 0x5c0 | id | r | aux 34421 closure$5 비교값 | 4 | OK |  |
| 44 | SmallActionTrace(self) | 0x68 |  | w | L217 target.x(63608~63609) / L205 target.x(63680~63681, 부시 안) / L212 adjust_position 결과 x(63709~63710) | 4 | OK | goal_x |
| 45 | SmallActionTrace(self) | 0x70 |  | w | 63668~63669 phi(L218 target.y / L205 target.y / L212 adjust y) | 4 | OK | goal_y |
| 46 | SmallActionTrace(self) | 0x10 |  | w | L299(63949) 또는 L248(64142) — 키 불일치/None 일 때만 재생성; 직전 PathFinder 의 Box 2개 해제(63909·63931 / 64102·64124). 이어서 update_path(&mut, L315 63971 / L261 64170) 가 내부(+0x10 path_len·+0x18 index·+0x28 path 등)를 갱신 — 콜리 소관 | 4 | OK | PathFinder 72B (new_target_with_policy 결과 memcpy) |
| 47 | SmallActionTrace(self) | 0x88 |  | w | L335(64187~64188, dive_ignore&&!lethal) · L367(64341~64342, raw_escape 이나 version>=2 && !tower_shooting_me) | 4 | OK | 0 |
| 48 | SmallActionTrace(self) | 0x88 |  | w | L370 (64329~64333) escape 커밋 1.5초 | 4 | OK | tick + tps*3/2 |
| 49 | SmallActionTrace(self) | 0x90 |  | w | L366 (64340) avoid_unnecessary_tower = true — raw_escape 인데 타워가 나를 쏘지 않으면 이후 불필요 타워 회피 경로로 전환 | 4 | OK | 1(true) |
| 50 | SmallActionTrace(self) | 0x95 |  | w | last_escape: L336 None(64189~64190) · L379 Some(escape)(64315~64316 · 64335~64336 · 64344~64345) | 4 | OK | 2(None) / 0(Some(false)) / 1(Some(true)) |
| 51 | sret Option<Input> | 0x0 |  | w | None 5곳(63390 · 63981 · 64177 · 64221 · 64377) | 4 | 확인불가(tcx 사전에 타입 없음) | -1 |

**`consts` 상수 19건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | -1 | 397 | 센티널 | Option<Input> None 니치(i64, +0). 또 Option<Effect> None(i32 -1) · level-1 | 4 |
| 1 | 2 | 227 | 태그 | version < 2 → lethal_chase=false·escape 무조건 커밋. 또 팀 bounds 2 · Option<PathFinder>/Option<TowerDodgeContext>/Option<bool> None 태그 2 · skill2 level>2 · SafeMoveWithSkill::Must=2 · chase_policy 기본 deadly_cells=2 (63747) | 4 |
| 2 | 32000 | 201 | 인덱스 | 셀 크기 — target.x/32000 = 그리드 인덱스 | 4 |
| 3 | 29 | 201 | 인덱스 | 그리드 인덱스 상한 min(idx,29) (30x30 bushes) | 4 |
| 4 | 13 | 222 | 태그 | EntityType::Champion 태그 13 — 대상이 챔피언이 아니면 use_tower_avoid_path=true (avoid_unnecessary_tower=false 일 때) | 4 |
| 5 | 16 | 297 | 산출값 | path key 길이: "trace_avoid_well"(16, @anon.251) · "trace_avoid_well_lethal"(23, .252) · "trace_objective_avoid_tower"(27, .253) · "trace_avoid_tower"(17, .254) · "trace_lethal_tower"(18, .255) — 키가 바뀌면 PathFinder 재생성 | 4 |
| 6 | 23 | 297 | 산출값 | "trace_avoid_well_lethal" 길이 | 4 |
| 7 | 27 | 239 | 산출값 | "trace_objective_avoid_tower" 길이 | 4 |
| 8 | 17 | 239 | 산출값 | "trace_avoid_tower" 길이 | 4 |
| 9 | 18 | 239 | 산출값 | "trace_lethal_tower" 길이 | 4 |
| 10 | 3 | 370 | 계수 | escape_commit_until = tick + tps*3/2 (mul 3, lshr 1 = 1.5초) | 4 |
| 11 | 1 | 370 | 인덱스 | lshr 1 = /2 (접힘) · smax(isqrt,1) · umax(max_hp,1) · SafeMoveWithSkill::Safe=1 · new_with_skill bool true · avoid_unnecessary_tower=true | 4 |
| 12 | 66 | 361 | 임계 | 마지막 피격원이 적 타워일 때 hp% < 66 이면 tower_shooting_me=true (closure$9) | 4 |
| 13 | 100 | 361 | 계수 | hp*100/max_hp 백분율 · Entity::radius (mult+100)/100 | 4 |
| 14 | 14400000001 | 390 | 임계 | dist² < 120000²+1 ⟺ dist<=120000(3.75셀): 이 안이거나 내 이속>대상 이속이면 SafeMoveWithSkill::Safe(1), 아니면 Must(2) | 4 |
| 15 | 5 | 382 | 미상 | SmallActionRunAway::new_with_skill(data, player, 5, true) usize 인자(v27 objective_discipline 과 동일 값) | 4 |
| 16 | 1120 | 299 | 미상 | 구 PathFinder.path Box<[(u64,u64);70]> 해제 크기(계측 아님, 메모리 관리) | 4 |
| 17 | 70 | 299 | 미상 | 구 PathFinder.planned_verdict Box<[u8;70]> 해제 크기 | 4 |
| 18 | 0 | 204 | 태그 | bushes[tyi][txi]==0(부시 아님) · VisibleState::Visible=0 · escape_commit_until=0 리셋 · SafeMove get_input 마지막 bool false | 4 |

**`knobs` 조정점 6건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | escape 커밋 지속(초) | trace.rs:370 (m02.ll:64329~64330 mul 3 lshr 1) | 3 | tps*3/2 = 1.5초. 올리면 타워 escape 후 RunAway 를 더 오래 유지 | 4 | 기존 |
| 1 | 피격원 타워 escape HP% 임계 | trace.rs:361 (m02.ll:64295) | 66 | 적 타워에게 마지막으로 맞았고 hp<66% 면 tower_shooting_me — 올리면 더 건강할 때도 escape 커밋 | 4 | 기존 |
| 2 | Safe/Must 이동 판정 거리(제곱) | trace.rs:390 (m02.ll:64410) | 14400000001 | 120000². 대상이 이 안이거나 내가 더 빠르면 Safe(스킬 이동 자제), 아니면 Must — 올리면 Must 로 스킬 이동을 덜 씀 | 4 | 기존 |
| 3 | chase_policy 기본 deadly_cells | trace.rs:229 (m02.ll:63747) | 2 | 비치명 추격의 SolvePolicy.deadly_cells=2·direct_cross=true. 치명이면 v3_deadly_edge_cells 계산값·direct_cross=false | 4 | 기존 |
| 4 | 표적점 후퇴 마진 | self.attack_range_margin(+0x78) — trace.rs:199 | 0 | 필드값(생성자 소관). 크면 대상에 더 가까운 점을 목표로 잡음(min_range - margin) | 4 | 기존 |
| 5 | RunAway 파라미터 | trace.rs:382 (m02.ll:64362) | 5 | new_with_skill(…,5,true) — 의미는 콜리 소관 | 4 | 기존 |

<details><summary>`callees` 피호출자 25건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | adjust_position | game_core::Game::adjust_position | pub | fn(&game_core::MapDef, &game_core::GameSetting, i64, i64) -> (u64, u64) | game-core\src\simulation\game.rs:818 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | check | game_core::CastingTarget::check | pub | fn(&game_core::CastingTarget, &game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\effect\type.rs:227 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 2 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 3 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 4 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 5 | get_input | game_ai::PathFinder::get_input | pub | fn(&mut game_ai::PathFinder, &game_core::PlayerState, &game_core::OperationData, game_ai::SafeMoveWithSkill, bool) -> game_core::Input | game-ai\src\path_finder.rs:856 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 6 | get_input | game_ai::SmallActionRunAway::get_input | in:game_ai | fn(&mut game_ai::SmallActionRunAway, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &mut game_core::DebugFrameData) -> std::option::Option<game_core::Input> | game-ai\src\small_action\move_actions.rs:95 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 7 | is_visible_from | game_core::Entity::is_visible_from | pub | fn(&game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1481 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 8 | isqrt | game_core::utils::isqrt | pub | fn(i64) -> i64 | game-core\src\utils.rs:85 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 9 | iter_towers_without_nexus | game_core::AbstractGameWithCache::<'a, 'b>::iter_towers_without_nexus | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5750 ~ game_core[6a30]::simulation::{impl#5}::iter_towers_without_nexus::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1830 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 10 | new_target_with_policy | game_ai::PathFinder::new_target_with_policy | pub | fn(&mut rand::rngs::std::StdRng, &game_core::GameContext, usize, &str, game_ai::path_field::SolvePolicy, u64, u64, u64, u64, impl Fn(usize, usize, usize, usize) -> PathVerdict/#0) -> game_ai::PathFinder | game-ai\src\path_finder.rs:88 | True | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 11 | new_tower_avoid_v3 | game_ai::TowerDodgeContext::new_tower_avoid_v3 | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, u64, u64) -> game_ai::TowerDodgeContext | game-ai\src\path_finder.rs:1205 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 12 | new_unnecessary_tower_avoid | game_ai::TowerDodgeContext::new_unnecessary_tower_avoid | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, u64, u64) -> game_ai::TowerDodgeContext | game-ai\src\path_finder.rs:1220 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 13 | new_with_skill | game_ai::SmallActionRunAway::new_with_skill | pub | fn(&game_core::OperationData, &game_core::PlayerState, usize, bool) -> game_ai::SmallActionRunAway | game-ai\src\small_action\move_actions.rs:29 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 14 | path_needs_tower_escape | game_ai::small_action::path_needs_tower_escape | in:game_ai::small_action | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_ai::PathFinder) -> bool | game-ai\src\small_action.rs:109 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 15 | radius | game_core::Entity::radius | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1509 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 16 | range | game_core::Effect::range | pub | fn(&game_core::Effect, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:25 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 17 | range_adjust | game_core::Effect::range_adjust | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:29 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 18 | skill2_effect | game_core::Entity::skill2_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1692 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 19 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 20 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 21 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 22 | update_path | game_ai::PathFinder::update_path | pub | fn(&mut game_ai::PathFinder, &mut rand::rngs::std::StdRng, &game_core::GameContext, u64, u64, u64, u64, impl Fn(usize, usize, usize, usize) -> PathVerdict/#0) | game-ai\src\path_finder.rs:631 | True | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 23 | v3_deadly_edge_cells | game_ai::v3_deadly_edge_cells | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData) -> u32 | game-ai\src\tower_discipline.rs:304 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 24 | v3_lethal_tower_hp | game_ai::v3_lethal_tower_hp | pub | fn(&game_core::OperationData, &game_core::PlayerState, &game_core::Entity) -> bool | game-ai\src\tower_discipline.rs:272 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
</details>

⚠**미매칭 8개**: `_RINvXs_NtNtNtCsjihNppCmMEE_4core4iter8adapters5chainINtB5_5ChainINtNtB7_7flatten7FlattenINtNtNtBb_5array4iter8IntoIterINtNtBb_6option6OptionRNtNtNtCs97f5S1uJLkH_9game_core10simulation6entity6EntityEKj6_EEINtNtB7_6copied6CopiedINtNtNtBb_5slice4iter4IterB2e_EEENtNtNtB9_6traits8iterator8Iterator8try_folduNCINvNvB49_3any5checkB2e_NCNvMs_NtNtCshdEBA0ozCnw_7game_ai12small_action5traceNtB5n_16SmallActionTrace9get_inputs5_0E0INtNtNtBb_3ops12control_flow11ControlFlowuEEB5r_`, `__rust_dealloc`, `closure`, `llvm.smax.i64`, `llvm.umax.i64`, `llvm.umin.i64`, `llvm.usub.sat.i64`, `memcmp`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m11.ll:43067) · **형제 20개** (SmallActionTrace)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::SmallActionTrace as std::clone::Clone>::clone | pub | game-ai\src\small_action\trace.rs:8 | True | fn(&game_ai::SmallActionTrace) -> game_ai::SmallActionTrace |
| 1 | <game_ai::SmallActionTrace as std::fmt::Debug>::fmt | pub | game-ai\src\small_action\trace.rs:8 | True | fn(&game_ai::SmallActionTrace, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 2 | game_ai::SmallActionTrace::new | pub | game-ai\src\small_action\trace.rs:42 | False | fn(&game_core::OperationData, usize, usize) -> game_ai::SmallActionTrace |
| 3 | game_ai::SmallActionTrace::with_avoid_enemy_zone | pub | game-ai\src\small_action\trace.rs:64 | True | fn(game_ai::SmallActionTrace) -> game_ai::SmallActionTrace |
| 4 | game_ai::SmallActionTrace::new_keep_range | pub | game-ai\src\small_action\trace.rs:69 | False | fn(&game_core::OperationData, usize, usize, u64) -> game_ai::SmallActionTrace |
| 5 | game_ai::SmallActionTrace::new_attack_range | pub | game-ai\src\small_action\trace.rs:75 | False | fn(&game_core::OperationData, usize, usize) -> game_ai::SmallActionTrace |
| 6 | game_ai::SmallActionTrace::new_attack_range_margin | pub | game-ai\src\small_action\trace.rs:79 | False | fn(&game_core::OperationData, usize, usize, u64) -> game_ai::SmallActionTrace |
| 7 | game_ai::SmallActionTrace::new_avoid_tower | pub | game-ai\src\small_action\trace.rs:100 | False | fn(&game_core::OperationData, usize, usize) -> game_ai::SmallActionTrace |
| 8 | game_ai::SmallActionTrace::new_attack_range_avoid_tower | pub | game-ai\src\small_action\trace.rs:106 | False | fn(&game_core::OperationData, usize, usize) -> game_ai::SmallActionTrace |
| 9 | game_ai::SmallActionTrace::avoid_unnecessary_tower | pub | game-ai\src\small_action\trace.rs:112 | True | fn(&game_ai::SmallActionTrace) -> bool |
| 10 | game_ai::SmallActionTrace::expected_goal_position | in:game_ai | game-ai\src\small_action\trace.rs:116 | False | fn(&game_ai::SmallActionTrace, &game_core::PlayerState, &game_core::OperationData) -> std::option::Option<(u64, u64)> |
| 11 | game_ai::SmallActionTrace::get_input | in:game_ai | game-ai\src\small_action\trace.rs:165 | False | fn(&mut game_ai::SmallActionTrace, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &mut game_core::DebugFrameData) -> std::option::Option<game_core::Input> |
| 12 | game_ai::SmallActionTrace::update_state | in:game_ai | game-ai\src\small_action\trace.rs:400 | False | fn(&mut game_ai::SmallActionTrace, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 13 | game_ai::SmallActionTrace::get_action | in:game_ai | game-ai\src\small_action\trace.rs:403 | True | fn(&game_ai::SmallActionTrace) -> game_core::SmallAction |
| 14 | game_ai::SmallActionTrace::is_end | in:game_ai | game-ai\src\small_action\trace.rs:406 | False | fn(&game_ai::SmallActionTrace, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bool |
| 15 | game_ai::SmallActionTrace::near_move_complete | in:game_ai | game-ai\src\small_action\trace.rs:413 | False | fn(&game_ai::SmallActionTrace, &game_core::Entity) -> bool |
| 16 | game_ai::SmallActionTrace::is_abandoned | pub | game-ai\src\small_action\trace.rs:418 | True | fn(&game_ai::SmallActionTrace) -> bool |
| 17 | game_ai::SmallActionTrace::applied_escape | pub | game-ai\src\small_action\trace.rs:423 | True | fn(&game_ai::SmallActionTrace) -> bool |
| 18 | game_ai::SmallActionTrace::target_id | pub | game-ai\src\small_action\trace.rs:427 | True | fn(&game_ai::SmallActionTrace) -> usize |
| 19 | game_ai::SmallActionTrace::mark_abandoned | pub | game-ai\src\small_action\trace.rs:432 | True | fn(&mut game_ai::SmallActionTrace) |

**`open` 8건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | 잎 22 콜리 계약: v3_deadly_edge_cells(usize,&PlayerState,&OperationData)->u32 (63739, 치명 추격일 때만) — 내부 미독. v3_lethal_tower_hp(&OperationData,&PlayerState,&Entity)->bool 은 잎 목록에 없음(tower_discipline, 같은 계층) — 시그니처만 | 4 |  |
| 1 | 미탐색 | path_finder/path_field 계층(시그니처만): TowerDodgeContext::new_tower_avoid_v3 / new_unnecessary_tower_avoid(usize,&PlayerState,&OperationData,u64,u64)->TowerDodgeContext(416B, Option 니치 = +0x19a 태그 2) · PathFinder::new_target_with_policy(rnd,&GameContext,version,key:&str,SolvePolicy{u32,bool},x,y,gx,gy,closure)->PathFinder(72B) · update_path(&mut PathFinder,rnd,&GameContext,x,y,gx,gy,closure) · PathFinder::get_input(&mut,&PlayerState,&OperationData,SafeMoveWithSkill,bool)->Input · path_needs_tower_escape(usize,&PlayerState,&OperationData,&PathFinder)->bool | 4 |  |
| 2 | 미탐색 | closure#3/#4(우물 경로 verdict: 캡처 &lethal_dodge,&version,player,&zone_ok) 와 closure#_/#0(타워 경로 verdict: 캡처 &lethal_chase,&tower_dodge,&version,&strict,player,data,positioning_score) 본체는 PathFinder 모노모픽 인스턴스(m03.ll 68257~74984 · 103055~110018, 각 3.3~3.5k줄)에 인라인 — 경로 계층이라 미탐색(aux 미등록). PathVerdict 반환 규칙은 그 인스턴스에서 읽어야 함 | 4 |  |
| 3 | 미탐색 | SmallActionRunAway::get_input(&mut,usize,&mut StdRng,&PlayerState,&OperationData,&PositioningScoreData,&mut DebugFrameData)->Option<Input> — 같은 계층, 내부 미독. debug/rnd 의 실제 쓰기는 여기서 발생 | 4 |  |
| 4 | 미탐색 | L204 bushes 판정: MapDef+0x1c98 = bushes[30][30] usize(tcxdict) — 0 이 '부시 아님'이라는 의미는 필드명 기반 추정(값 코드표 미확인) | 3 |  |
| 5 | 미탐색 | L211 Game::adjust_position(&MapDef,&GameSetting,i64,i64)->(i64,i64) 의 인자 순서는 IR(63702: map=%189, %216=context+8 setting) 기준 — game_core 경계 | 4 |  |
| 6 | 미탐색 | L297 %266 분기(lethal 인데 lethal_chase 바이트 false)는 도달 불가(죽은 분기) — freeze 아티팩트 | 4 |  |
| 7 | 미탐색 | path_finder 가 None 인 채 update_path 직전에 도달하는 경로(63980 · 64176) 는 new 직후라 이론상 도달 불가 | 4 |  |

**`notes` 1건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | Option<Input> None = i64 -1 은 IR 실측(63390 등) — tcxdict 에 Option<Input> 인스턴스 없어 니치 값은 IR 로만 확정 | 3 | 사실 서술 |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

