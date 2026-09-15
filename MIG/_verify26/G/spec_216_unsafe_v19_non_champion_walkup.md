---

### `216` unsafe_v19_non_champion_walkup — 비챔피언(미니언/타워 등) 대상 공격·스킬을 위해 걸어 들어가는 위치가 위험한가 — 타워 커버 위험 / 궤적·타워 포커스 위험 / 적 챔피언 킬각·리스크·미니언 웨이브 위험이면 true(행동 차단)

| 항목 | 값 |
|---|---|
| id | `line_defense__LineDefense__unsafe_v19_non_champion_walkup` |
| 심볼 | `_RNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy8sub_plan12line_defenseNtB2_18LineDefenseSubPlan30unsafe_v19_non_champion_walkup` |
| 소스 | `game-ai\src\plan_legacy\sub_plan\line_defense.rs:72` |
| IR | `m14.ll` 28602~29733행 |
| 경로·가시성 | `game_ai::plan_legacy::sub_plan::LineDefenseSubPlan::unsafe_v19_non_champion_walkup` · **in:game_ai::plan_legacy::sub_plan::line_defense** |
| 계층 | 기타 |
| exe | `e8a100` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r17` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_ai::plan_legacy::sub_plan::LineDefenseSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_ai::SmallActionPlay, bool, &mut game_core::DebugFrameData) -> bool
```

**TLS 메모 절(`sig.tls` · 정정 경로 `/specs[216]/sig/tls/<키>`)**

- `name`: (직접 접점 없음)
- `role`: 본문에 LocalKey/@anon fn-포인터 상수 참조 0건. 간접: v47_tower_focus_position_dangerous → v47_siege_stance(SIEGE_STANCE_CACHE 읽기/쓰기) · check_kill_die_tick(DieTickCache, m00.ll:73353/74500 LocalKey::with) — 둘 다 콜리 소관
- `key`: —
- `layout`: —
- `invalidation`: —
- `call_conditions`: v47 는 L112 (walkup 위치 확정 후 항상) · check_kill_die_tick 은 L137 (attackers 비어있지 않을 때만)

<details><summary>인자 9개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | self | &LineDefenseSubPlan(3B) | %0. style/line/minion_action_type 어느 것도 읽지 않음(positioning_score_at 에도 self 를 넘기지 않음) | 4 |
| 1 | 2 | version | usize | %1. v22_current_line_non_champion_action_tower_risk · positioning_score_at · v47_tower_focus_position_dangerous · enemy_minion_line_action_danger_damage_at · check_kill_die_tick 에 전달 | 4 |
| 2 | 3 | rnd | &mut StdRng(320B, align 16) | %2. 쓰기 표면 없음 | 4 |
| 3 | 4 | player | &PlayerState(2528B) | %3. info.team(+0x930) · info.position(+0x9c0). 콜리 5곳에 전달 · 클로저 캡처 | 4 |
| 4 | 5 | data | &OperationData(24B) | %4. +0 cache · +8 context · +16 blackboard(클로저) | 4 |
| 5 | 6 | parameter | &ScoreParameter(5384B) | %5. +0x9f0 positioning_score(&→positioning_score_at) · +0x918 player(ChampionScoreParameter → possible_risk) · +0x998 player.risk_damage | 4 |
| 6 | 7 | action | &SmallActionPlay(184B) | %6. 니치 태그 +0xb1 → Attack/Skill/Skill2/Ult 만 · 페이로드 +0x8 target id | 4 |
| 7 | 8 | has_runaway | bool | %7. L143/L144 should_block 합성에만 | 4 |
| 8 | 9 | debug | &mut DebugFrameData(224B) | %8 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn unsafe_v19_non_champion_walkup(&self, version, rnd, player, data, parameter, action, has_runaway, debug) -> bool
L74: _t = ProfTimer(phase 82)   // prof::ENABLED 일 때만 Instant::now · 종료 시 PHASE_NANOS/CALLS[82] atomic add
L75: small_action = action.get_action(); Attack(6)/Skill(7)/Skill2(8)/Ult(9) 외 → return false
     backoff = (Ult ? 150000 : 15000)   // phi %62 · L61 에서 사용
L79: target = cache.game.get_entity_by_id(small_action.target_id)? ; None → false
L82: champ = cache.player_champion[team][pos]? ; None → false
L86: if target.team == champ.team || target.is_champion() → return false      // 비챔피언 대상 전용
L90: effect = action_effect(champ, small_action)? (L25~29: attack/skill/skill2(level>2)/ult(level>4) as_ref) ; None → false
L93: if !CastingTarget::check(&effect.target, champ, target) → return false
L97: action_kills_target = !(expected_damage_target(effect, context, champ, target) < target.hp)
L98: if !action_kills_target && v22_current_line_non_champion_action_tower_risk(version, context, cache, player, target) {
L99:   if context.debug { debug.infos[champ.id].push("#v22 lane tower cover block: target {target.id}") }
       return true }
L105: let Some((x,y)) = action_walkup_position(champ, target, effect, backoff) else { return false }   // 인라인 L39~66:
   L39/35: if effect(재계산 action_effect).is_some_and(|e| e.is_in_range(champ, target)) → return false  (※Some(walkup) 이 아니라 함수 전체 false: %182→%493 phi false)
   L43: if !(champ.team is Player(t) → target.visible_state[t]==Visible ; Neutral → true) → Some((target.x, target.y))   // 안 보이면 대상 좌표 그대로
   L47: if effect.ty.can_move() || effect.ty.expected_move_on_hit() || effect.ty.expected_rush_effect() → Some((target.x, target.y))
   L51~53: dx = champ.x - target.x ; dy = champ.y - target.y ; sz = isqrt(dx²+dy²)
   L54: if sz < 1 → None (→ return false)
   L58: caster_radius = (effect.casting == Targeting) ? champ.radius() : 0
   L59: range = effect.range(champ, caster_radius) + range_adjust(effect, champ, target) + target.radius()
        // Effect::range(effect.rs:26) 인라인 = effect.range + caster_radius + champ.stat_buff_cached.range + growth_range×(level-1)  (add 재결합 가능 · dbg L26 귀속)
   L61: from_distance = range.saturating_sub(backoff)
   L62~63: x = target.x + from_distance*dx / sz ; y = target.y + from_distance*dy / sz   (sdiv · 대상에서 champ 쪽으로 from_distance 만큼)
   L65: (x,y) = Game::adjust_position(context.map, context.setting, x, y)
L109: score = self.positioning_score_at(version, player, data, &parameter.positioning_score, x, y)
L110: trajectory_danger = score.on_trajectory || score.on_periodic_trajectory
L112: tower_focus_danger = v47_tower_focus_position_dangerous(version, data, player, x, y)
L113: minion_wave_danger = enemy_minion_line_action_danger_damage_at(version, data, champ, x, y, tps*2, false, false) != 0
L115: if trajectory_danger || tower_focus_danger {
L116:   if context.debug { debug.infos[champ.id].push("v19 lane danger block: target .., trajectory=.., tower_focus=..") }
       return true }
L122: enemy_team = 1 - team
L123: attackers = cache.iter_champions(enemy_team).filter(|e| {          // aux closure$0
   L125:   (champ.team Neutral || e.visible_state[champ.team]==Visible) || data.blackboard[enemy_team].is_recent_visible(game, player, e)   // 둘 다 아니면 false
   L129:   range = max_range_nearly_can_use(e, champ, tps) + 20000
   L130:   range != 0 && distance_sq((e.x,e.y),(x,y)) <= range²
      }).collect_in(bump)
L133: if attackers.is_empty() → should_block = false → return false
L137: die_tick = check_kill_die_tick(version, rnd, data, player, champ, attackers, Vec::new_in(bump), debug)
L139~140: risk_damage = parameter.player.risk_damage + max(parameter.player.possible_risk(data, tps*2), 0)
L141: risk_is_high = risk_damage*100 >= max(champ.hp,1)*35     (부호 있는 비교)
L142: if die_tick > tps*2 {
L144:   forced_risk_walkup = score.risk > 24 ; should_block = (has_runaway && forced_risk_walkup) || minion_wave_danger
     } else {
L143:   should_block = risk_is_high || has_runaway || score.risk > 9 || minion_wave_danger   (줄 안 순서는 column 부재로 미확정)
     }
L147: if should_block && context.debug { debug.infos[champ.id].push("v19 lane danger block: target .., die_tick=.., score_risk=.., high_risk=.., minion_wave=..") }
L152: drop(_t) ; return should_block
```

**`mem` 메모리 접근 59건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | SmallActionPlay | 0xb1 | @tag(niche) | r | L75 small_action.rs:309 get_action → 태그 15/16/17/18 = Attack/Skill/Skill2/Ult, 그 외 return false | 4 | OK |  |
| 1 | SmallActionPlay | 0x8 | @Attack\|Skill\|Skill2\|Ult.0.target | r | L75~79 target_id (blackboard.rs:104~106 get_action_target 경유) | 4 | OK |  |
| 2 | OperationData | 0x0 | cache | r |  | 4 | OK |  |
| 3 | OperationData | 0x8 | context | r | L97 expected_damage_target · L99/116/147 debug · L65 map/setting · L114 setting | 4 | OK |  |
| 4 | OperationData | 0x10 | blackboard | r | &[Blackboard;2] → 클로저 캡처, [enemy_team] 인덱스(stride 744B) | 4 | OK |  |
| 5 | GameContext | 0x8 | setting | r | L65 adjust_position · L114 tick_per_second | 4 | OK |  |
| 6 | GameContext | 0x20 | map | r | L65 Game::adjust_position | 4 | OK |  |
| 7 | GameContext | 0x3b | debug | r | L99 · L116 · L147 디버그 로그 게이트 | 4 | OK |  |
| 8 | GameSetting | 0x12f8 | tick_per_second | r | L114: ×2 (shl 1) = 2초 창 | 4 | OK |  |
| 9 | AbstractGameWithCache | 0x0 | game.data_ptr | r |  | 4 | OK |  |
| 10 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r | 슬롯 +0x1f0 get_entity_by_id (L79) | 4 | OK |  |
| 11 | AbstractGameWithCache | 0x1e0 | player_champion[team][pos] | r | L82 내 챔피언 · L123 iter_champions(enemy_team) 도 같은 배열의 [enemy_team] 행(5칸)을 순회 | 4 | OK |  |
| 12 | PlayerState | 0x930 | info.team | r | bounds <2 | 4 | OK |  |
| 13 | PlayerState | 0x9c0 | info.position@tag | r | as_index | 4 | OK |  |
| 14 | ScoreParameter | 0x9f0 | positioning_score | r | L109 positioning_score_at 4번째 인자(&, 2760B) | 4 | OK |  |
| 15 | ScoreParameter | 0x918 | player | r | L140 ChampionScoreParameter::possible_risk 의 &self(216B) | 4 | OK |  |
| 16 | ScoreParameter | 0x998 | player.risk_damage | r | L139 risk_damage 기본값 | 4 | OK |  |
| 17 | Entity(target) | 0x0 | team@tag | r | L86 TeamType PartialEq | 4 | OK |  |
| 18 | Entity(target) | 0x8 | team@Player.0 | r | == champ 팀이면 return false | 4 | OK |  |
| 19 | Entity(target) | 0x68 | ty@tag | r | L86 is_champion(13) 이면 return false(비챔피언 전용) | 4 | OK |  |
| 20 | Entity(target) | 0x670 | hp | r | L97 action_kills_target = !(dmg < hp) | 4 | OK |  |
| 21 | Entity(target) | 0x38 | visible_state[champ.team] | r | L43 is_visible_from: gepS stride 24B · 태그 0=Visible | 4 | OK |  |
| 22 | Entity(target) | 0x660 | x | r | L44/L48/L51/L62 | 4 | OK |  |
| 23 | Entity(target) | 0x668 | y | r | L44/L48/L52/L63 | 4 | OK |  |
| 24 | Entity(target) | 0x470 | stat_buff_cached.radius_mult | r | L59 target.radius() | 4 | OK |  |
| 25 | Entity(target) | 0x680 | radius | r | L59 | 4 | OK |  |
| 26 | Entity(target) | 0x5c0 | id | r | 디버그 문자열 인자(L100/117/148) | 4 | OK |  |
| 27 | Entity(champ) | 0x0 | team@tag | r | L86 · L43 player_team | 4 | OK |  |
| 28 | Entity(champ) | 0x8 | team@Player.0 | r |  | 4 | OK |  |
| 29 | Entity(champ) | 0x4c0 | attack_effect@tag(niche i32) | r | L90/L35 action_effect(Attack) | 4 | OK |  |
| 30 | Entity(champ) | 0x490 | attack_effect@Some.0 | r |  | 4 | OK |  |
| 31 | Entity(champ) | 0x4f8 | skill_effect@tag(niche i32) | r | Skill | 4 | OK |  |
| 32 | Entity(champ) | 0x4c8 | skill_effect@Some.0 | r |  | 4 | OK |  |
| 33 | Entity(champ) | 0x5c8 | level | r | skill2_effect(): level>2 / ult_effect(): level>4 아니면 정적 NONE(@anon.22) · L59 growth_range×(level-1) | 4 | OK |  |
| 34 | Entity(champ) | 0x500 | skill2_effect | r | 태그 +0x530 | 4 | OK |  |
| 35 | Entity(champ) | 0x538 | ult_effect | r | 태그 +0x568 | 4 | OK |  |
| 36 | Entity(champ) | 0x660 | x | r | L51 dx = champ.x - target.x | 4 | OK |  |
| 37 | Entity(champ) | 0x668 | y | r | L52 | 4 | OK |  |
| 38 | Entity(champ) | 0x470 | stat_buff_cached.radius_mult | r | L58 caster_radius = champ.radius() (casting==Targeting 일 때) | 4 | OK |  |
| 39 | Entity(champ) | 0x680 | radius | r | L58 | 4 | OK |  |
| 40 | Entity(champ) | 0x438 | stat_buff_cached.range | r | L59 Effect::range 인라인 가산 | 4 | OK |  |
| 41 | Entity(champ) | 0x670 | hp | r | L141 max(hp,1)×35 | 4 | OK |  |
| 42 | Entity(champ) | 0x5c0 | id | r | 디버그 로그 키(L100/117/148) · 클로저 캡처 | 4 | OK |  |
| 43 | Effect | 0x0 | ty(Arc<dyn EffectType>) | r | L47 ArcInner 데이터(+16, align 마스크) · vtable 슬롯 0x120 can_move / 0x68 expected_move_on_hit / 0x60 expected_rush_effect (divtable 94% 일치) | 3 | OK |  |
| 44 | Effect | 0x10 | range | r | L59 | 4 | OK |  |
| 45 | Effect | 0x18 | growth_range | r | L59 | 4 | OK |  |
| 46 | Effect | 0x28 | target(CastingTarget) | r | L93 CastingTarget::check | 4 | OK |  |
| 47 | Effect | 0x30 | casting(CastingType) | r | L58 == 0 Targeting | 4 | OK |  |
| 48 | PositioningScore | 0x0 | risk | r | L143 >9 / L144 >24 (sret %31 로컬) | 4 | OK |  |
| 49 | PositioningScore | 0x30 | on_trajectory | r | L110 | 4 | OK |  |
| 50 | PositioningScore | 0x31 | on_periodic_trajectory | r | L110 | 4 | OK |  |
| 51 | Entity(enemy e) | 0x38 | visible_state[champ.team] | r | aux L125 | 4 | OK |  |
| 52 | Entity(enemy e) | 0x660 | x | r | aux L130 distance_sq | 4 | OK |  |
| 53 | Entity(enemy e) | 0x668 | y | r | aux L130 | 4 | OK |  |
| 54 | DebugFrameData | 0xa0 | infos | w | L99~100 · 조건: !action_kills_target && v22 tower risk true && context.debug. HashMap 재해시/Vec grow 포함(hashbrown rustc_entry · or_insert · push_mut) | 4 | OK | entry(champ.id).or_insert(Vec::new()).push(format!("#v22 lane tower cover block: target {}", target.id)) |
| 55 | DebugFrameData | 0xa0 | infos | w | L116~117 · 조건: (trajectory_danger \|\| tower_focus_danger) && context.debug | 4 | OK | push(format!("v19 lane danger block: target {}, trajectory={}, tower_focus={}", target.id, trajectory_danger, tower_focus_danger)) |
| 56 | DebugFrameData | 0xa0 | infos | w | L147~148 · 조건: should_block && context.debug | 4 | OK | push(format!("v19 lane danger block: target {}, die_tick={}, score_risk={}, high_risk={}, minion_wave={}", target.id, die_tick, score.risk, risk_is_high, minion_wave_danger)) |
| 57 | global game_core::prof::PHASE_NANOS | [82] | PHASE_NANOS[82] | w | prof::ENABLED(전역 atomic i8)!=0 일 때만 · ProfTimer phase 82 · 텔레메트리(판정 무관) | 4 | 확인불가(tcx 사전에 타입 없음) | atomicrmw add elapsed_ns |
| 58 | global game_core::prof::PHASE_CALLS | [82] | PHASE_CALLS[82] | w | 동일 조건 | 4 | 확인불가(tcx 사전에 타입 없음) | atomicrmw add 1 |

**`consts` 상수 21건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev | folded_from |
|---|---|---|---|---|---|---|
| 0 | 82 | 74 | 산출값 | prof 위상 id(ProfTimer phase) — 텔레메트리, 판정 아님 | 4 |  |
| 1 | 10 | 75 | 센티널 | SmallActionPlay 니치 불가값(llvm.assume ne 10) — 판정 아님 | 4 |  |
| 2 | -3 | 75 | 센티널 | 니치 태그→논리 idx (niche_start=3) | 4 |  |
| 3 | 12 | 75 | 태그 | 논리 idx 12=Attack → SmallAction 태그 6 | 4 |  |
| 4 | 13 | 75 | 태그 | 논리 idx 13=Skill → 7. 또한 L86 EntityType 태그 13=Champion(is_champion → return false) | 4 |  |
| 5 | 14 | 75 | 태그 | 논리 idx 14=Skill2 → 8 | 4 |  |
| 6 | 15 | 75 | 태그 | 논리 idx 15=Ult → 9 | 4 |  |
| 7 | 150000 | 61 | 산출값 | backoff(Ult 일 때) — 걸어갈 위치를 사거리에서 이만큼 안쪽으로(약 4.7셀). phi 로 호이스팅돼 소스 줄 미확정(get_action 분기 %61 phi) | 4 |  |
| 8 | 15000 | 61 | 산출값 | backoff(Attack/Skill/Skill2) — 사거리 - 15000 지점까지 접근 | 4 |  |
| 9 | 2 | 82 | 임계 | team 경계검사 상한. 또한 skill2_effect level>2 · L114 tps×2 접힘(shl 1)·L142 die_tick > tps*2 창 | 4 |  |
| 10 | 1 | 114 | 임계 | tick_per_second << 1 = tps*2 (2초 창) — enemy_minion_line_action_danger_damage_at 창 · possible_risk 창 · L142 die_tick 비교 | 4 | 2 |
| 11 | 0 | 86 | 태그 | TeamType 태그 0=Player. 또한 L58 CastingType 태그 0=Targeting · L133 attackers.len()==0 · L140 max(possible_risk,0) | 4 |  |
| 12 | -1 | 90 | 센티널 | Option<Effect> 니치 None (4종 as_ref) | 4 |  |
| 13 | 4 | 90 | 임계 | ult_effect(): level > 4 | 4 |  |
| 14 | 100 | 58 | 계수 | radius(): radius×(mult+100)/100. 또한 L141 risk_damage×100 (퍼센트 비교) | 4 |  |
| 15 | 35 | 141 | 계수 | risk_is_high = risk_damage×100 >= max(hp,1)×35 — 위험 피해가 HP 의 35% 이상 | 4 |  |
| 16 | 9 | 143 | 임계 | die_tick ≤ 2초일 때 score.risk > 9 이면 차단 | 4 |  |
| 17 | 24 | 144 | 임계 | die_tick > 2초일 때 has_runaway && score.risk > 24 이면 차단(forced_risk_walkup) | 4 |  |
| 18 | 20000 | 129 | 미상 | aux: attackers 판정 사거리 = max_range_nearly_can_use(e, champ, tps) + 20000 | 4 |  |
| 19 | 132 | 152 | 길이 | prof PHASE 배열 길이(경계검사) — 판정 아님 | 4 |  |
| 20 | 1000000000 | 152 | 임계 | 초→나노초 (prof 텔레메트리) | 4 |  |

**`knobs` 조정점 7건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | Ult 걸어들기 backoff | line_defense.rs:61 (phi %62, 150000) | 150000 | 올리면 궁 사거리에서 더 멀찍이 서서 시전 위치를 잡는다(위험 판정이 관대해질 수 있음) | 4 | 기존 |
| 1 | 일반 걸어들기 backoff | line_defense.rs:61 (15000) | 15000 | 올리면 사거리 끝에서 더 안쪽까지 걸어가지 않는다 | 4 | 기존 |
| 2 | attackers 인정 사거리 여유 | line_defense.rs:129 (aux) | 20000 | 올리면 더 먼 적 챔피언도 공격자로 세어 킬각 판정이 늘어난다 | 4 | 기존 |
| 3 | 고위험 비율 | line_defense.rs:141 | 35 | 내리면 더 작은 위험 피해에도 risk_is_high → 차단 증가 | 4 | 기존 |
| 4 | score.risk 차단 임계(즉사 창 안) | line_defense.rs:143 | 9 | 올리면 위험 점수가 높아도 걸어들기 허용 | 4 | 기존 |
| 5 | score.risk 강제 차단 임계(즉사 창 밖·has_runaway) | line_defense.rs:144 | 24 | 올리면 도주 상태에서도 더 관대 | 4 | 기존 |
| 6 | 판정 창(초) | line_defense.rs:114 (tps<<1) | 2 | die_tick·미니언 웨이브·possible_risk 창 — 올리면 더 먼 미래의 죽음까지 본다 | 4 | 기존 |

<details><summary>`callees` 피호출자 47건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | action_effect | game_ai::plan_legacy::sub_plan::LineDefenseSubPlan::action_effect | in:game_ai::plan_legacy::sub_plan::line_defense | fn(&game_core::Entity, game_core::SmallAction) -> std::option::Option<&game_core::Effect> | game-ai\src\plan_legacy\sub_plan\line_defense.rs:24 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 1 | action_walkup_position | game_ai::plan_legacy::sub_plan::LineDefenseSubPlan::action_walkup_position | in:game_ai::plan_legacy::sub_plan::line_defense | fn(&game_ai::plan_legacy::sub_plan::LineDefenseSubPlan, &game_core::Entity, &game_core::Entity, game_core::SmallAction, &game_core::Effect, &game_core::OperationData) -> std::option::Option<(u64, u64)> | game-ai\src\plan_legacy\sub_plan\line_defense.rs:38 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 2 | adjust_position | game_core::Game::adjust_position | pub | fn(&game_core::MapDef, &game_core::GameSetting, i64, i64) -> (u64, u64) | game-core\src\simulation\game.rs:818 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 3 | can_move | game_core::Entity::can_move | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1489 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 4개 중 상위 3개 |
| 4 | can_move | game_core::Champion::can_move | pub | fn(&game_core::Champion) -> bool | game-core\src\simulation\entity\champion.rs:48 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 4개 중 상위 3개 |
| 5 | can_move | game_core::EffectType::can_move | pub | fn(&Self/#0) -> bool | game-core\src\simulation\effect\type.rs:358 | True | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 4개 중 상위 3개 |
| 6 | check | game_core::CastingTarget::check | pub | fn(&game_core::CastingTarget, &game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\effect\type.rs:227 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 7 | check_kill_die_tick | game_ai::check_kill_die_tick | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::OperationData, &game_core::PlayerState, &game_core::Entity, bumpalo::collections::vec::Vec< &game_core::Entity>, bumpalo::collections::vec::Vec< &game_core::Entity>, &mut game_core::DebugFrameData) -> usize | game-ai\src\fight_check.rs:917 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 8 | distance_sq | game_core::utils::distance_sq | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:6 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 9 | distance_sq | game_core::Entity::distance_sq | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2157 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 10 | enemy_minion_line_action_danger_damage_at | game_ai::enemy_minion_line_action_danger_damage_at | pub | fn(usize, &game_core::OperationData, &game_core::Entity, u64, u64, usize, bool, bool) -> usize | game-ai\src\minion_wave_risk.rs:233 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 11 | expected_damage_target | game_core::Effect::expected_damage_target | pub | fn(&game_core::Effect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + , &game_core::Entity) -> usize | game-core\src\simulation\effect.rs:91 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 12 | expected_move_on_hit | game_core::EffectType::expected_move_on_hit | pub | fn(&Self/#0) -> bool | game-core\src\simulation\effect\type.rs:293 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 20개 중 상위 3개 |
| 13 | expected_move_on_hit | <game_core::CombineEffect as game_core::EffectType>::expected_move_on_hit | pub | fn(&game_core::CombineEffect) -> bool | game-core\src\simulation\effect\type\combine.rs:96 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 20개 중 상위 3개 |
| 14 | expected_move_on_hit | <game_core::DelayedEffect as game_core::EffectType>::expected_move_on_hit | pub | fn(&game_core::DelayedEffect) -> bool | game-core\src\simulation\effect\type\delayed.rs:45 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 20개 중 상위 3개 |
| 15 | expected_rush_effect | game_core::EffectType::expected_rush_effect | pub | fn(&Self/#0) -> bool | game-core\src\simulation\effect\type.rs:291 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 9개 중 상위 3개 |
| 16 | expected_rush_effect | <game_core::RushEffect as game_core::EffectType>::expected_rush_effect | pub | fn(&game_core::RushEffect) -> bool | game-core\src\simulation\effect\type\rush.rs:53 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 9개 중 상위 3개 |
| 17 | expected_rush_effect | <game_core::CombineEffect as game_core::EffectType>::expected_rush_effect | pub | fn(&game_core::CombineEffect) -> bool | game-core\src\simulation\effect\type\combine.rs:42 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 9개 중 상위 3개 |
| 18 | get_action | game_ai::SmallActionUlt::get_action | in:game_ai | fn(&game_ai::SmallActionUlt) -> game_core::SmallAction | game-ai\src\small_action\cast.rs:286 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 16개 중 상위 3개 |
| 19 | get_action | game_ai::SmallActionPlay::get_action | pub | fn(&game_ai::SmallActionPlay) -> game_core::SmallAction | game-ai\src\small_action.rs:308 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 16개 중 상위 3개 |
| 20 | get_action | game_ai::SmallActionTrace::get_action | in:game_ai | fn(&game_ai::SmallActionTrace) -> game_core::SmallAction | game-ai\src\small_action\trace.rs:403 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 16개 중 상위 3개 |
| 21 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 22 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 23 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 24 | is_champion | game_core::EntityType::is_champion | pub | fn(&game_core::EntityType) -> bool | game-core\src\simulation\entity.rs:1403 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 25 | is_empty | game_core::PatchSetting::is_empty | pub | fn(&game_core::PatchSetting) -> bool | game-core\src\patch_engine.rs:853 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 26 | is_empty | game_core::ModAiRegistry::is_empty | pub | fn(&game_core::ModAiRegistry) -> bool | game-core\src\mod_ai.rs:15 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 27 | is_empty | game_core::PublicChampionEvidence::is_empty | pub | fn(&game_core::PublicChampionEvidence) -> bool | game-core\src\banpick\public_evidence.rs:96 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 28 | is_in_range | game_core::Effect::is_in_range | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\effect.rs:63 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 29 | is_recent_visible | game_core::Blackboard::is_recent_visible | pub | fn(&game_core::Blackboard, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\game\blackboard.rs:346 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 30 | isqrt | game_core::utils::isqrt | pub | fn(i64) -> i64 | game-core\src\utils.rs:85 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 31 | iter_champions | game_core::AbstractGameWithCache::<'a, 'b>::iter_champions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5759 ~ game_core[6a30]::simulation::{impl#5}::iter_champions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1904 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 32 | max_range_nearly_can_use | game_ai::plan_legacy::old::max_range_nearly_can_use | pub | fn(&game_core::Entity, &game_core::Entity, usize) -> u64 | game-ai\src\plan_legacy\old\battle.rs:2397 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 33 | positioning_score_at | game_ai::plan_legacy::sub_plan::LineDefenseSubPlan::positioning_score_at | in:game_ai::plan_legacy::sub_plan::line_defense | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, u64, u64) -> game_core::PositioningScore | game-ai\src\plan_legacy\sub_plan\line_defense.rs:68 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 34 | possible_risk | game_ai::ChampionScoreParameter::<'_>::possible_risk | pub | fn(&game_ai::ChampionScoreParameter</#0>, &game_core::OperationData, usize) -> i64 | game-ai\src\score_parameter.rs:1407 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 35 | push | <game_core::Staff as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::staff | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\staff.rs:38 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 36 | push | <game_core::Athlete as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:434 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 37 | push | <game_core::Contract as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:2498 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 38 | radius | game_core::Entity::radius | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1509 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 39 | range | game_core::Effect::range | pub | fn(&game_core::Effect, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:25 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 40 | range_adjust | game_core::Effect::range_adjust | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:29 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 41 | skill2 | game_ai::skill2 | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_core::Entity) -> std::option::Option<game_core::Input> | game-ai\src\abstract_input.rs:239 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 65개 중 상위 3개 |
| 42 | skill2 | game_core::Entity::skill2 | pub | fn(&game_core::Entity) -> &std::boxed::Box<dyn [Binder { value: Trait(game_core::Action), bound_vars: [] }] + 'static, std::alloc::Global> | game-core\src\simulation\entity.rs:1668 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 65개 중 상위 3개 |
| 43 | skill2 | game_core::ChampionInfo::skill2 | pub | fn(&Self/#0) -> std::boxed::Box<dyn [Binder { value: Trait(game_core::Action), bound_vars: [] }] + 'static, std::alloc::Global> | game-core\src\setting.rs:1086 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 65개 중 상위 3개 |
| 44 | unsafe_v19_non_champion_walkup | game_ai::plan_legacy::sub_plan::LineDefenseSubPlan::unsafe_v19_non_champion_walkup | in:game_ai::plan_legacy::sub_plan::line_defense | fn(&game_ai::plan_legacy::sub_plan::LineDefenseSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_ai::SmallActionPlay, bool, &mut game_core::DebugFrameData) -> bool | game-ai\src\plan_legacy\sub_plan\line_defense.rs:72 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 45 | v22_current_line_non_champion_action_tower_risk | game_ai::v22_current_line_non_champion_action_tower_risk | pub | fn(usize, &game_core::GameContext, &game_core::AbstractGameWithCache, &game_core::PlayerState, &game_core::Entity) -> bool | game-ai\src\tower_discipline.rs:223 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 46 | v47_tower_focus_position_dangerous | game_ai::v47_tower_focus_position_dangerous | pub | fn(usize, &game_core::OperationData, &game_core::PlayerState, u64, u64) -> bool | game-ai\src\tower_discipline.rs:619 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
</details>

⚠**미매칭 8개**: `drop  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `effect  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 193개는 **전부 다른 함수**라 싣지 않는다`, `elapsed`, `format_inner`, `minion_wave_danger`, `or_insert`, `push_mut`, `rustc_entry`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 2곳** (m01.ll:13557, m01.ll:13589) · **형제 17개** (LineDefenseSubPlan)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::plan_legacy::sub_plan::LineDefenseSubPlan as std::clone::Clone>::clone | pub | game-ai\src\plan_legacy\sub_plan\line_defense.rs:7 | True | fn(&game_ai::plan_legacy::sub_plan::LineDefenseSubPlan) -> game_ai::plan_legacy::sub_plan::LineDefenseSubPlan |
| 1 | <game_ai::plan_legacy::sub_plan::LineDefenseSubPlan as std::fmt::Debug>::fmt | pub | game-ai\src\plan_legacy\sub_plan\line_defense.rs:7 | True | fn(&game_ai::plan_legacy::sub_plan::LineDefenseSubPlan, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 2 | game_ai::plan_legacy::sub_plan::LineDefenseSubPlan::new | pub | game-ai\src\plan_legacy\sub_plan\line_defense.rs:15 | True | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, game_core::LineType, game_ai::MinionActionType, game_core::LineStyle) -> game_ai::plan_legacy::sub_plan::LineDefenseSubPlan |
| 3 | game_ai::plan_legacy::sub_plan::LineDefenseSubPlan::new_for_gank | pub | game-ai\src\plan_legacy\sub_plan\line_defense.rs:20 | True | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, game_core::LineType, game_ai::MinionActionType, game_core::LineStyle) -> game_ai::plan_legacy::sub_plan::LineDefenseSubPlan |
| 4 | game_ai::plan_legacy::sub_plan::LineDefenseSubPlan::action_effect | in:game_ai::plan_legacy::sub_plan::line_defense | game-ai\src\plan_legacy\sub_plan\line_defense.rs:24 | False | fn(&game_core::Entity, game_core::SmallAction) -> std::option::Option<&game_core::Effect> |
| 5 | game_ai::plan_legacy::sub_plan::LineDefenseSubPlan::action_is_currently_in_range | in:game_ai::plan_legacy::sub_plan::line_defense | game-ai\src\plan_legacy\sub_plan\line_defense.rs:34 | False | fn(&game_core::Entity, game_core::SmallAction, &game_core::Entity) -> bool |
| 6 | game_ai::plan_legacy::sub_plan::LineDefenseSubPlan::action_walkup_position | in:game_ai::plan_legacy::sub_plan::line_defense | game-ai\src\plan_legacy\sub_plan\line_defense.rs:38 | False | fn(&game_ai::plan_legacy::sub_plan::LineDefenseSubPlan, &game_core::Entity, &game_core::Entity, game_core::SmallAction, &game_core::Effect, &game_core::OperationData) -> std::option::Option<(u64, u64)> |
| 7 | game_ai::plan_legacy::sub_plan::LineDefenseSubPlan::positioning_score_at | in:game_ai::plan_legacy::sub_plan::line_defense | game-ai\src\plan_legacy\sub_plan\line_defense.rs:68 | False | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, u64, u64) -> game_core::PositioningScore |
| 8 | game_ai::plan_legacy::sub_plan::LineDefenseSubPlan::unsafe_v19_non_champion_walkup | in:game_ai::plan_legacy::sub_plan::line_defense | game-ai\src\plan_legacy\sub_plan\line_defense.rs:72 | False | fn(&game_ai::plan_legacy::sub_plan::LineDefenseSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_ai::SmallActionPlay, bool, &mut game_core::DebugFrameData) -> bool |
| 9 | game_ai::plan_legacy::sub_plan::LineDefenseSubPlan::base_positioning | in:game_ai::plan_legacy::sub_plan::line_defense | game-ai\src\plan_legacy\sub_plan\line_defense.rs:154 | False | fn(&game_ai::plan_legacy::sub_plan::LineDefenseSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, std::option::Option<&game_ai::MinionWaveSnapshot>) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 10 | game_ai::plan_legacy::sub_plan::LineDefenseSubPlan::get_move_action_v46 | in:game_ai::plan_legacy::sub_plan::line_defense | game-ai\src\plan_legacy\sub_plan\line_defense.rs:200 | False | fn(&game_ai::plan_legacy::sub_plan::LineDefenseSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, std::option::Option<&game_ai::MinionWaveSnapshot>, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 11 | game_ai::plan_legacy::sub_plan::LineDefenseSubPlan::attack_minion_action | in:game_ai::plan_legacy::sub_plan::line_defense | game-ai\src\plan_legacy\sub_plan\line_defense.rs:368 | False | fn(&mut game_ai::plan_legacy::sub_plan::LineDefenseSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 12 | game_ai::plan_legacy::sub_plan::LineDefenseSubPlan::attack_tower_action | in:game_ai::plan_legacy::sub_plan::line_defense | game-ai\src\plan_legacy\sub_plan\line_defense.rs:372 | False | fn(&mut game_ai::plan_legacy::sub_plan::LineDefenseSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> std::option::Option<game_ai::SmallActionPlay> |
| 13 | game_ai::plan_legacy::sub_plan::LineDefenseSubPlan::calculate_score_parameter_value | pub | game-ai\src\plan_legacy\sub_plan\line_defense.rs:396 | False | fn(&game_ai::plan_legacy::sub_plan::LineDefenseSubPlan, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::ScoreParameter) |
| 14 | game_ai::plan_legacy::sub_plan::LineDefenseSubPlan::action_candidates | pub | game-ai\src\plan_legacy\sub_plan\line_defense.rs:440 | False | fn(&mut game_ai::plan_legacy::sub_plan::LineDefenseSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 15 | game_ai::plan_legacy::sub_plan::LineDefenseSubPlan::action_candidates_old | in:game_ai::plan_legacy::sub_plan::line_defense | game-ai\src\plan_legacy\sub_plan\line_defense.rs:881 | False | fn(&mut game_ai::plan_legacy::sub_plan::LineDefenseSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 16 | game_ai::plan_legacy::sub_plan::LineDefenseSubPlan::score | pub | game-ai\src\plan_legacy\sub_plan\line_defense.rs:940 | False | fn(&game_ai::plan_legacy::sub_plan::LineDefenseSubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 |

**`open` 6건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 재료 부재 | Effect::range(effect.rs:26) 의 정확한 시그니처 — 본 함수에선 caster_radius 가산이, v47 에선 15000 가산이 L26 로 귀속돼 `range(&self, caster, extra)` 형태로 추정. add 가 nsw 없이 재결합 가능해 dbg 귀속만으로는 확정 불가 | 4 |  |
| 1 | 표기 불가 | L143 `risk_is_high \|\| has_runaway \|\| score.risk > 9 \|\| minion_wave_danger` 의 줄 안 피연산자 순서 — column 부재(표기 불가) | 4 |  |
| 2 | 미탐색 | L47 vtable 슬롯 이름은 divtable 94% 일치 vtable 기준(can_move/expected_move_on_hit/expected_rush_effect) — Arc<dyn EffectType> 런타임 구현체별 의미는 미확인 | 3 |  |
| 3 | 미탐색 | is_visible_from(entity.rs:1482) 의 self/other 역할명 — IR 상 self=champ(team 공급), other=target(visible_state 공급)으로 읽음 | 4 |  |
| 4 | 미탐색 | positioning_score_at · check_kill_die_tick · enemy_minion_line_action_danger_damage_at · possible_risk · max_range_nearly_can_use · is_recent_visible 본문은 계약만(담당 밖) | 4 |  |
| 5 | 미탐색 | iter_champions(enemy_team) 의 FilterMap 클로저(simulation.rs:1905)는 Option flatten 으로 읽었으나 game_core 본문 미독 | 4 |  |

**`notes` 1건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | backoff(150000/15000) 의 소스 줄 — phi %62 에 dbg 없음. action_walkup_position 시그니처에 backoff 인자가 있는지, 아니면 L60~61 안에서 match 하는지 미확정 | 4 | 사실 서술 |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

