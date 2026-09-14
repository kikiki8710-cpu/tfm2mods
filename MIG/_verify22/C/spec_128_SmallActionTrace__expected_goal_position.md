---

### `128` SmallActionTrace::expected_goal_position — 추격(Trace) 소액션이 향할 목표 좌표 — 대상이 안 보이거나 부시 안이면 대상 좌표 그대로, 보이면 대상에서 (최소사거리−여유) 만큼 내 쪽으로 물러선 지점을 adjust_position 으로 보정해 Some((x,y))

| 항목 | 값 |
|---|---|
| id | `trace__SmallActionTrace_expected_goal_position` |
| 심볼 | `_RNvMNtNtCshdEBA0ozCnw_7game_ai12small_action5traceNtB2_16SmallActionTrace22expected_goal_position` |
| 소스 | `game-ai\src\small_action\trace.rs:116` |
| IR | `m02.ll` 8995~9340행 |
| 경로·가시성 | `game_ai::SmallActionTrace::expected_goal_position` · **in:game_ai** |
| 계층 | 기타 |
| exe | `caff00` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r13` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_ai::SmallActionTrace, &game_core::PlayerState, &game_core::OperationData) -> std::option::Option<(u64, u64)>
```

<details><summary>인자 4개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | *mut Option<(u64,u64)>(24B) | +0x0 태그 i64(0=None/1=Some) · +0x8 x · +0x10 y | 4 |
| 1 | 1 | self | &SmallActionTrace(152B) | explicit_min_range(0x0/0x8) · target(0x60) · attack_range_margin(0x78) · attack_range_only(0x91) 만 읽음 | 4 |
| 2 | 2 | player | &PlayerState(2528B) | info.team / info.position 으로 내 챔피언 조회 | 4 |
| 3 | 3 | data | &OperationData(24B) | cache(+0) → game dyn AbstractGame(vtable 0x1f0 get_entity_by_id)·player_champion / context(+8) → map(+0x20)·setting(+0x8) | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn expected_goal_position(&self, player:&PlayerState, data:&OperationData) -> Option<(u64,u64)>
  let target = data.cache.game.get_entity_by_id(self.target)?                       // L117 vtable+0x1f0, null → None (9010~9013)
  let champ  = data.cache.player_champion[player.info.team][player.info.position.as_index()]?   // L118 (9047~9053)
  let radius_sum = radius(champ) + radius(target)                                     // L119 (9063~9109) radius(e)= mult==0 ? e.radius : e.radius*(mult+100)/100
  let base = champ.stat_buff_cached.range + radius_sum                                // %94 (9157) — 세 사거리 공통
  let min_range = if let Some(r) = self.explicit_min_range { r }                      // L120 (9111~9122)
    else {
      let atk = champ.attack_effect.as_ref()?                                          // L124 (-1 → 반환 None, 9126~9129)
      let mut m = base + atk.range + (champ.level-1)*atk.growth_range + Effect::range_adjust(atk, champ, target)   // L124~125 (9145~9160)
      if !self.attack_range_only {                                                     // L127 (9165~9168; true 면 아래 전부 건너뜀)
        if let Some(sk) = champ.skill_effect.as_ref() {                                // L128 (9173~9176)
          if sk.target.check(champ, target) {                                          // L129 (9182)
            let sr = base + sk.range + (level-1)*sk.growth_range + range_adjust(sk, champ, target)   // L130 (9201~9209)
            m = min(sr, m) } }                                                          // L131 (umin 9213)
        let sk2 = if champ.level > 2 { &champ.skill2_effect } else { &NONE }           // L134 (entity.rs:1693, 9191~9193)
        if let Some(sk2) = sk2 {                                                       // (9195~9198)
          if sk2.target.check(champ, target) {                                         // L135 (9224)
            let sr2 = base + sk2.range + (level-1)*sk2.growth_range + range_adjust(sk2, champ, target)   // L136 (9228~9236)
            m = min(sr2, m) } }                                                         // L137 (umin 9240)
      }
      m }
  // L144: 가시성 (champ Neutral 이면 검사 생략 → 보이는 것으로 취급, 9138~9140)
  if !target.is_visible_from(champ.player_team()) {                                   // visible_state[t]!=Visible → (9260~9261 false 분기 %152)
    return Some((target.x, target.y)) }                                                // L145 (9268~9272)
  let txi = min(target.x/32000, 29); let tyi = min(target.y/32000, 29)                // L147~148 (9277~9289; clamp(0,29) — 부호없음이라 하한은 umin 만)
  if data.context.map.bushes[tyi][txi] != 0 {                                          // L149 (9295~9300)
    return Some((target.x, target.y)) }
  let dx = champ.x - target.x; let dy = champ.y - target.y                             // L153~154 (9303~9309; wrapping i64)
  let sz = max(isqrt(dx*dx + dy*dy), 1)                                                // L155 (9311~9317)
  let from_distance = min_range.saturating_sub(self.attack_range_margin)              // L156 (9319~9322)
  let x = target.x + from_distance*dx/sz; let y = target.y + from_distance*dy/sz       // L157~158 (9324~9330; sdiv)
  Some(Game::adjust_position(data.context.map, data.context.setting, x, y))            // L159 (9332~9336) → sret +8/+16, 태그 1

```

**`mem` 메모리 접근 41건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | OperationData | 0x0 | cache | r | &AbstractGameWithCache(9002) | 4 | OK |  |
| 1 | OperationData | 0x8 | context | r | &GameContext(9292) | 4 | OK |  |
| 2 | AbstractGameWithCache | 0x0 | game(dyn AbstractGame data_ptr) | r | 9003 — vtable 호출 인자 | 4 | OK |  |
| 3 | AbstractGameWithCache | 0x8 | game(dyn AbstractGame vtable_ptr) | r | 9005; vtable+0x1f0(496) = get_entity_by_id (divtable AbstractGame 0x1f0, 9008~9010) | 3 | OK |  |
| 4 | AbstractGameWithCache | 0x1e0 | player_champion[team][pos] | r | [2][5] Option<&Entity> 니치 null=None(9047~9053, L118) | 4 | OK |  |
| 5 | SmallActionTrace | 0x0 | explicit_min_range@tag | r | i64 trunc i1: 1=Some → +0x8 값을 min_range 로(9111~9113, L120) | 4 | OK |  |
| 6 | SmallActionTrace | 0x8 | explicit_min_range@Some.0 | r | u64 명시 최소사거리(9116~9117) | 4 | OK |  |
| 7 | SmallActionTrace | 0x60 | target | r | usize 엔티티 id → get_entity_by_id(9006~9010, L117) | 4 | OK |  |
| 8 | SmallActionTrace | 0x78 | attack_range_margin | r | u64. min_range.saturating_sub(margin) = from_distance(9319~9322, L156) | 4 | OK |  |
| 9 | SmallActionTrace | 0x91 | attack_range_only | r | bool. true 면 스킬 사거리 min 반영 생략(9165~9168, L127) | 4 | OK |  |
| 10 | PlayerState | 0x930 | info.team | r | <2 아니면 panic_bounds_check(9019~9022, 9039) | 4 | OK |  |
| 11 | PlayerState | 0x9c0 | info.position@tag | r | as_index()(player.rs:581 인라인, 9044~9046) | 4 | OK |  |
| 12 | Entity | 0x0 | team@tag | r | champ. 1=Neutral 이면 가시성 검사 생략(9138~9140, entity.rs:1136 player_team 인라인) | 4 | OK |  |
| 13 | Entity | 0x8 | team@Player.0 | r | champ 팀번호 → target.visible_state 인덱스(9248~9253) | 4 | OK |  |
| 14 | Entity | 0x38 | visible_state[team]@tag | r | target. stride 24B(gepS 9258). 0=Visible(9259~9261, L144) | 4 | OK |  |
| 15 | Entity | 0x470 | stat_buff_cached.radius_mult | r | champ(9063)·target(9086). 0 이면 radius 그대로, 아니면 radius*(mult+100)/100 (entity.rs:1511~1515) | 4 | OK |  |
| 16 | Entity | 0x680 | radius | r | champ·target → radius_sum(9070·9077·9093·9100·9109, L119) | 4 | OK |  |
| 17 | Entity | 0x4c0 | attack_effect@tag(니치) | r | champ. i32 -1=None → 반환 None(9126~9129, L124) | 4 | OK |  |
| 18 | Entity | 0x490 | attack_effect@Some.0 | r | &Effect(56B) → range_adjust 인자(9143, 9156) | 4 | OK |  |
| 19 | Entity | 0x4a0 | attack_effect@Some.0.range | r | 9145~9146 (effect.rs:26 range() 인라인) | 4 | OK |  |
| 20 | Entity | 0x4a8 | attack_effect@Some.0.growth_range | r | ×(level-1) (9147~9152) | 4 | OK |  |
| 21 | Entity | 0x5c8 | level | r | champ. (level-1)*growth 공용(9149~9151) · level>2 가 skill2 존재 조건(9191) | 4 | OK |  |
| 22 | Entity | 0x438 | stat_buff_cached.range | r | champ 기본 사거리(9153~9154). +radius_sum 이 세 사거리 공통 베이스 %94(9157) | 4 | OK |  |
| 23 | Entity | 0x4f8 | skill_effect@tag(니치) | r | champ. -1=None 이면 스킬 사거리 반영 생략(9173~9176, L128) | 4 | OK |  |
| 24 | Entity | 0x4c8 | skill_effect@Some.0 | r | &Effect → range_adjust(9172, 9206) | 4 | OK |  |
| 25 | Entity | 0x4f0 | skill_effect@Some.0.target | r | CastingTarget → check(champ,target)(9181~9182, L129) | 4 | OK |  |
| 26 | Entity | 0x4d8 | skill_effect@Some.0.range | r | 9201~9202 (L130) | 4 | OK |  |
| 27 | Entity | 0x4e0 | skill_effect@Some.0.growth_range | r | 9203~9205 | 4 | OK |  |
| 28 | Entity | 0x500 | skill2_effect@Some.0 | r | level>2 일 때만 이 주소, 아니면 정적 None(@anon…19) (entity.rs:1693 skill2_effect 인라인, 9191~9193). 이후 +0x30 태그·+0x28 target·+0x10 range·+0x18 growth 는 이 base 상대(9195·9223·9228·9230) | 4 | OK |  |
| 29 | Entity | 0x530 | skill2_effect@tag(니치) | r | = skill2 base+0x30(48). -1=None 이면 생략(9195~9198, L134) | 4 | OK |  |
| 30 | Entity | 0x528 | skill2_effect@Some.0.target | r | = base+0x28(40) → check(9223~9224, L135) | 4 | OK |  |
| 31 | Entity | 0x510 | skill2_effect@Some.0.range | r | = base+0x10(16) (9228~9229, L136) | 4 | OK |  |
| 32 | Entity | 0x518 | skill2_effect@Some.0.growth_range | r | = base+0x18(24) (9230~9232) | 4 | OK |  |
| 33 | Entity | 0x660 | x | r | target(9268·9275) / champ(9303) | 4 | OK |  |
| 34 | Entity | 0x668 | y | r | target(9270·9283) / champ(9307) | 4 | OK |  |
| 35 | GameContext | 0x20 | map | r | &MapDef(28112B) (9293~9294, L149) | 4 | OK |  |
| 36 | GameContext | 0x8 | setting | r | &GameSetting(5432B) → adjust_position 인자(9332~9334, L159) | 4 | OK |  |
| 37 | MapDef | 0x1c98 | bushes[tyi][txi] | r | [30][30] usize. 0 이 아니면 대상이 부시 안 → 대상 좌표 그대로(9295~9300, L149) | 4 | OK |  |
| 38 | (sret) | 0x0 | Option tag | w | m02.ll:9034~9035 phi: 0 ← %4(target 없음)/%27(champ 없음)/%73(atk 없음), 1 ← %19 | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) | 0(None) \| 1(Some) |
| 39 | (sret) | 0x8 | x | w | Some 경로만(9027~9028). phi %20: %154(target.x, L145) / %159(target.x, L149 부시) / %199(adjust 결과, L159) | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) | target.x \| adjust_position().0 |
| 40 | (sret) | 0x10 | y | w | Some 경로만(9029~9030). phi %21: %156 / %163 / %200 | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) | target.y \| adjust_position().1 |

**`consts` 상수 7건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 100 | 119 | 계수 | 반경 배율 퍼센트 radius*(mult+100)/100 (entity.rs:1511~1515 인라인, 9079~9081·9102~9104). 임계 아님 | 4 |
| 1 | -1 | 124 | 센티널 | Option<Effect> 니치 None(i32). attack_effect(9128) 없으면 반환 None / skill(9175)·skill2(9197) 없으면 해당 사거리 생략 | 4 |
| 2 | 2 | 134 | 임계 | skill2 존재 조건 level > 2 (entity.rs:1693 skill2_effect 인라인, 9191). 3레벨부터 skill2 사거리를 min 후보에 포함. 같은 값 2 는 9021·9252 에선 팀 배열 상한(bounds) | 4 |
| 3 | 0 | 144 | 태그 | VisibleState 태그 0=Visible (9260). Visible 이면 부시/오프셋 계산, 아니면 대상 좌표 그대로. 9299 의 0 은 bushes 셀 '부시 아님' | 4 |
| 4 | 32000 | 147 | 인덱스 | 셀 크기 — 좌표/32000 = 셀 인덱스(9277, 9285). 좌표 변환 | 4 |
| 5 | 29 | 147 | 인덱스 | clamp 상한 = 30x30 격자 마지막 인덱스(umin, 9281·9289) | 4 |
| 6 | 1 | 155 | 태그 | sz = max(isqrt(dx²+dy²), 1) — 0 나눗셈 방지 바닥(9317). 9034 의 1 은 Some 태그, 9112 는 explicit_min_range Some 태그 | 4 |

**`knobs` 조정점 1건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | skill2 사거리 반영 시작 레벨 | entity.rs:1693 (Entity::skill2_effect 인라인, m02.ll:9191) | 2 | level>2 조건. 올리면 더 늦은 레벨부터 skill2 사거리가 min 후보에 들어감 — 단 game_core 인라인이라 이 함수 단독 조정 불가 | 4 | 기존 |

<details><summary>`callees` 피호출자 15건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | adjust_position | game_core::Game::adjust_position | pub | fn(&game_core::MapDef, &game_core::GameSetting, i64, i64) -> (u64, u64) | game-core\src\simulation\game.rs:818 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | as_index | game_core::BigGoal::as_index | pub | fn(&game_core::BigGoal) -> usize | game-core\src\simulation\ai_interface.rs:338 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 2 | as_index | game_core::Position::as_index | pub | fn(&game_core::Position) -> usize | game-core\src\simulation\entity.rs:580 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 3 | check | game_core::CastingTarget::check | pub | fn(&game_core::CastingTarget, &game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\effect\type.rs:227 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | expected_goal_position | game_ai::SmallActionTrace::expected_goal_position | in:game_ai | fn(&game_ai::SmallActionTrace, &game_core::PlayerState, &game_core::OperationData) -> std::option::Option<(u64, u64)> | game-ai\src\small_action\trace.rs:116 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 5 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 6 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 7 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 8 | is_visible_from | game_core::Entity::is_visible_from | pub | fn(&game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1481 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 9 | isqrt | game_core::utils::isqrt | pub | fn(i64) -> i64 | game-core\src\utils.rs:85 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 10 | player_team | game_core::TeamType::player_team | pub | fn(&game_core::TeamType) -> std::option::Option<usize> | game-core\src\simulation\entity.rs:1135 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 11 | player_team | game_view::ClientData::player_team | pub | fn(&game_view::ClientData) -> std::option::Option<std::cell::Ref< game_core::Team>> | game-view\src\logic\client\data.rs:1579 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 12 | player_team | game_view::ClientDatabase::player_team | pub | fn(&game_view::ClientDatabase) -> &game_core::Team | game-view\src\logic\client\data.rs:1163 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 13 | radius | game_core::Entity::radius | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1509 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 14 | range_adjust | game_core::Effect::range_adjust | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:29 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
</details>

⚠**미매칭 4개**: `clamp`, `llvm.smax.i64`, `llvm.umin.i64`, `llvm.usub.sat.i64`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m11.ll:41956) · **형제 20개** (SmallActionTrace)

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

**`open` 5건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 표기 불가 | L144 의 소스 표기: IR 은 visible_state[t]==Visible(0) 이면 부시/오프셋 경로, 아니면 대상 좌표 직반환. 소스가 `if !is_visible_from(..) { return Some(target.pos) }` 형태인지 `is_visible_from` 의 극성이 반대인지는 표기 불가(동작은 확정) | 4 |  |
| 1 | 미탐색 | champ.team==Neutral 경로(9139~9140 → %157)는 player_champion 이 항상 Player 팀이라 실전 도달 불가(추정) | 5 |  |
| 2 | 미탐색 | attack_range_margin 이 min_range 보다 크면 from_distance=0 → 목표 = target 좌표(adjust 후). 의도 여부는 소스 없이 판단 불가 | 4 |  |
| 3 | 미탐색 | @anon.94acafa22d01e083ca1cc62f01598c8f.19(정적 None Option<Effect>) 은 컴파일러가 만든 상수 — 값 미확인(태그 -1 로 추정, 9193→9197 경로) | 5 |  |
| 4 | 미탐색 | self 의 나머지 필드(goal_x/goal_y 등)는 본 함수에서 안 읽음 — 반환값을 self.goal 에 쓰는 것은 호출자 몫(미독) | 4 |  |

**`notes` 1건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | adjust_position(&MapDef,&GameSetting,x,y)->(i64,i64) 의 내부(벽/맵 밖 보정으로 추정) 및 isqrt·range_adjust·CastingTarget::check 내부는 game_core(_gcbc g15.ll:72245 / g06.ll:87607) — 본 범위 밖, 시그니처만 확정 | 4 | 사실 서술 |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

