---

### `212` AgentVerHamster::update_small_action — 소액션 재평가: failed_action 쿨다운 정리·실패 기록(포기 Trace/무진전 도주·귀환/미발동 캐스트 교체/Trace 이탈) → plan_system.get_small_action 경매 → small_action.merge → last_eval_* 스냅샷 갱신

| 항목 | 값 |
|---|---|
| id | `AgentVerHamster__update_small_action` |
| 심볼 | `_RNvMs1_CshdEBA0ozCnw_7game_aiNtB5_15AgentVerHamster19update_small_action` |
| 소스 | `game-ai\src\lib.rs:693` |
| IR | `m14.ll` 35955~37839행 |
| 경로·가시성 | `game_ai::AgentVerHamster::update_small_action` · **in:game_ai** |
| 계층 | 기타 |
| exe | `e8e560` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r17` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&mut game_ai::AgentVerHamster, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData)
```

**TLS 메모 절(`sig.tls` · 정정 경로 `/specs[212]/sig/tls/<키>`)**

없음 — 본문 TLS 접점 0(`@anon.*` 참조 3개는 전부 panic Location(146~151)·format 문자열(148) · call_once/LocalKey/threadlocal 0건). CNT_DM_IDLE_EVAL/CNT_DM_IDLE_PICK 는 external global 원자(프로세스 전역)

<details><summary>인자 4개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | self | &mut AgentVerHamster(10704B) (%0) | 쓰기 표면 = writes 전수 | 4 |
| 1 | 2 | rnd | &mut StdRng(320B) (%1) |  | 4 |
| 2 | 3 | player | &PlayerState(2528B) (%2) | +0x930 team · +0x9c0 position | 4 |
| 3 | 4 | data | &OperationData(24B) (%3) | +0 cache · +8 context · +16 blackboard | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn update_small_action(&mut self, rnd, player, data)   // lib.rs:693~794
  game = data.cache.game (dyn AbstractGame · tick()=vtable+0x28 · get_game_mode()=vtable+0x40)
  // ── 1. 실패 기록 정리 (L695 · aux m06.ll retain)
  self.failed_action.retain(|(t, _)| !(t + 60 < game.tick()))      // 60틱(1초) 지난 실패는 제거
  // ── 2. 실패 기록 A: 포기된 추격 (L699~705)
  if let Trace(t) = &self.small_action && t.abandoned (+0x92):
    act = SmallAction::Trace{ target_id: t.target (+0x60) }         // discr 4
    if let Some(x) = failed_action.iter_mut().find(|(_,a)| a == act): x.0 = game.tick()   // L703
    else: failed_action.push((game.tick(), act))                     // L705
  // ── 3. 실패 기록 B: 무진전 도주/귀환 (L713~723)
  stalled = match &self.small_action {
    RunAway(a) => a.prog_best_tick != 0 && game.tick().saturating_sub(a.prog_best_tick) > 119   // L714 (+0x30)
    Recall(a)  => a.prog_best_tick != 0 && game.tick().saturating_sub(a.prog_best_tick) > 119   // L715 (+0x78)
    _ => false }
  if stalled:
    act = self.small_action.to_small_action()    // L719 · 이 문맥에선 항상 SmallAction::RunAway(discr 0 · 페이로드 없음)
    find(a == act) ? x.0 = tick (L721) : push((tick, act)) (L723)
  // ── 4. 경매 (L728~730)
  self.small_debug = DebugFrameData::default()
  (score_parameter, next_score, next_action) = self.plan_system.get_small_action(self.version, rnd, player, data, &self.small_action, &self.failed_action, &mut self.small_debug)
  // ── 5. DM 계측 (L734~752 · 관측 전용)
  if game.get_game_mode() is DeathMatch(tag 2) && plan_system.plan is DeathMatchBattle(tag<2) && plan.idle_spec_tick == game.tick():
    CNT_DM_IDLE_EVAL += 1;  CNT_DM_IDLE_PICK[next_action.to_small_action().discr] += 1
  // ── 6. 실패 기록 C: 미발동 캐스트가 다른 액션으로 교체됨 (L757~761)
  if self.small_action ∈ {Attack,Skill,Skill2,Ult} && !is_act (+0x10) && self.small_action.to_small_action() != next_action.to_small_action():
    act = self.small_action.to_small_action()    // {discr 6..9, target}
    find(a == act) ? x.0 = tick (L759) : push((tick, act)) (L761)
  // ── 7. 실패 기록 D: 추격이 비-추격·비-캐스트로 교체됨 (L765~770)
  if self.small_action is Trace && !(next_action.tag ∈ 14..=18):
    act = SmallAction::Trace{target}
    find(a == act) ? x.0 = tick (L768) : push((tick, act)) (L770)
  // ── 8. 적용 (L774~777)
  champ = data.cache.player_champion[player.info.team][player.info.position].unwrap()
  self.small_action.merge(self.version, champ, next_action)       // L774 (콜리)
  self.small_action_score = next_score                              // L776
  self.positioning_score = score_parameter.positioning_score        // L777 (2760B 복사)
  // ── 9. 디버그 (L779~786 · context.debug 일 때만)
  if data.context.debug:
    self.small_debug.infos.entry(champ.id).or_default().push(format!("applyed_damage: {}, risk_damage: {}, possible damage: {}, tower: {}, risk_epic_damage: {}", sp.player.applyed_damage, sp.player.risk_damage, sp.player.possible_risk(data, 30), sp.player.risk_possible_tower, sp.player.risk_epic_damage))
    self.small_debug.positiong_score.insert(champ.id, score_parameter.positioning_score.clone())
  // ── 10. 재평가 스냅샷 (L791~793 · can_skip_eval 의 기준값)
  self.last_eval_tick = game.tick()
  self.last_eval_hp = champ.hp
  self.last_eval_nearby_enemies = count_nearby_enemies(champ, player, data)
  drop(score_parameter)                                             // L794

※ SmallAction PartialEq(인라인 blackboard.rs:81~86): discr 같고, Positioning(1)/AroundPosition(3)은 x·y(+16,+24) 둘 다, Around(2)/Trace(4)/Attack(6)/Skill(7)/Skill2(8)/Ult(9)는 target(+16), RunAway(0)/Dodge(5)/Stop(10)은 discr 만.
※ to_small_action(small_action.rs:309~326) 태그→SmallAction: RunAway3/Recall4/AroundRunAway8→RunAway(0) · Around5/AroundHide6→Around{+0x8}(2) · AroundRegion7→AroundPosition{+0x10,+0x18}(3) · Positioning9→Positioning{+0x8,+0x10}(1) · AroundPosition(암묵)→AroundPosition{+0x30,+0x38}(3) · AroundPositionBush11→AroundPosition{+0x8,+0x10}(3) · AroundBush12→AroundPosition{+0x18,+0x20}(3) · LaneMinionPosition13→Around{+0x8}(2) · Trace14→Trace{+0x60}(4) · Attack15→Attack{+0x8}(6) · Skill16→Skill(7) · Skill2 17→Skill2(8) · Ult18→Ult(9) · Stop19→Stop(10).
※ 극성 근거: L699 `select %26(tag==14), abandoned, false` → true 면 %37(기록) · L713 stalled `and` 참이면 %99 · L757 `%268 = !cast || is_act` 참이면 %337(생략) · L765 `(next.tag-14) ult 5` 참이면 %487(생략).
```

**`mem` 메모리 접근 43건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | AgentVerHamster | 0x1d48 | failed_action (Vec<(usize,SmallAction)> cap/ptr/len) | r | +0x1d48 cap · +0x1d50 ptr · +0x1d58 len. 원소 32B = {tick@0, SmallAction@8(discr@8 · payload0@16 · payload1@24)} | 4 | OK |  |
| 1 | AgentVerHamster | 0x2909 | small_action@tag (+0xb1) | r | L699 ==14 Trace · L713 switch(RunAway3/Recall4) · L757 cast 계열(15~18) · L765 ==14 · to_small_action 스위치 여러 곳 | 4 | OK |  |
| 2 | AgentVerHamster | 0x28ea | small_action.Trace.abandoned (+0x92) | r | L699: Trace 이고 abandoned 면 실패 기록 | 4 | OK |  |
| 3 | AgentVerHamster | 0x28b8 | small_action.Trace.target (+0x60) | r | L701/L767 SmallAction::Trace{target_id} | 4 | OK |  |
| 4 | AgentVerHamster | 0x2888 | small_action.RunAway.prog_best_tick (+0x30) / AroundPosition.around_input.target_x | r | L714 stalled 판정(RunAway) · to_small_action AroundPosition x | 4 | OK |  |
| 5 | AgentVerHamster | 0x28d0 | small_action.Recall.prog_best_tick (+0x78) | r | L715 stalled 판정(Recall) | 4 | OK |  |
| 6 | AgentVerHamster | 0x2868 | small_action.<cast>.is_act (+0x10) | r | L757: 캐스트 계열이고 !is_act 일 때만 교체-실패 기록 | 4 | 부분일치(첫 성분(컨테이너)만 맞고 **마지막 성분이 다르다** — 사람이 봐야 ) |  |
| 7 | AgentVerHamster | 0x2860 | small_action payload +0x8 (target / goal_x) | r | to_small_action 인라인(small_action.rs:309~326): Around/AroundHide/LaneMinion target · Positioning goal_x · AroundPositionBush target_x · Attack/Skill/Skill2/Ult target | 4 | OK |  |
| 8 | AgentVerHamster | 0x2870 | small_action payload +0x18 (AroundRegion goal_x / AroundBush target_x) | r | to_small_action | 4 | OK |  |
| 9 | AgentVerHamster | 0x2878 | small_action payload +0x20 (AroundBush target_y) | r | to_small_action | 4 | OK |  |
| 10 | AgentVerHamster | 0x2890 | small_action payload +0x38 (AroundPosition around_input.target_y) | r | to_small_action | 4 | OK |  |
| 11 | AgentVerHamster | 0xb18 | plan_system.plan@tag (LegacyPlanHandler+0x5e8) | r | L735: tag<2 ⟺ BigPlan::DeathMatchBattle(암묵 니치) — DM 계측 게이트 | 4 | OK |  |
| 12 | AgentVerHamster | 0xc58 | plan_system.plan@DeathMatchBattle.idle_spec_tick (+0x728) | r | L736 == game.tick() 이면 CNT_DM_IDLE_* 계측 | 4 | OK |  |
| 13 | AgentVerHamster | 0x2910 | version | r | L730 get_small_action · L774 merge 인자 | 4 | OK |  |
| 14 | AgentVerHamster | 0x530 | plan_system | r | &mut 로 get_small_action 에 전달 | 4 | OK |  |
| 15 | AgentVerHamster | 0x1c0 | small_debug | r | L728 drop 후 Default · &mut 로 get_small_action · L782/786 infos/positiong_score 쓰기 | 4 | OK |  |
| 16 | OperationData | 0x0 | cache | r |  | 4 | OK |  |
| 17 | OperationData | 0x8 | context | r | +0x3b debug 플래그(L779) | 4 | OK |  |
| 18 | OperationData | 0x10 | blackboard | r | count_nearby_enemies 인자(L793) | 4 | OK |  |
| 19 | AbstractGameWithCache | 0x0 | game.data_ptr | r |  | 4 | OK |  |
| 20 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r | +0x28 tick() 다수 · +0x40 get_game_mode()(L734) | 4 | OK |  |
| 21 | AbstractGameWithCache | 0x1e0 | player_champion[team][position] | r | L774 champ = .unwrap() (None→unwrap_failed) | 4 | OK |  |
| 22 | GameContext | 0x3b | debug | r | L779 | 4 | OK |  |
| 23 | PlayerState | 0x930 | info.team | r | L774 bounds ult 2 | 4 | OK |  |
| 24 | PlayerState | 0x9c0 | info.position@tag | r | L774 | 4 | OK |  |
| 25 | Entity | 0x5c0 | id (자기 챔피언) | r | L782/786 디버그 맵 키 | 4 | OK |  |
| 26 | Entity | 0x670 | hp | r | L792 → last_eval_hp | 4 | OK |  |
| 27 | Entity | 0x660 | x | r | L793 count_nearby_enemies | 4 | OK |  |
| 28 | Entity | 0x668 | y | r | L793 | 4 | OK |  |
| 29 | ScoreParameter(get_small_action sret) | 0x9f0 | positioning_score (2760B) | r | L777 self.positioning_score 로 복사 · L786 디버그 insert | 4 | OK |  |
| 30 | ScoreParameter | 0x918 | player (ChampionScoreParameter 216B) | r | L783/784 디버그 문자열: +0x988 applyed_damage · +0x998 risk_damage · possible_risk(&player,data,30) · +0x9b0 risk_possible_tower · +0x9a0 risk_epic_damage | 4 | OK |  |
| 31 | get_small_action sret(5576B) | 0x1508 | next_score (i64 @+5384) | r | L729 분해 · L776 → small_action_score | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 32 | get_small_action sret(5576B) | 0x1510 | next_action (SmallActionPlay 184B @+5392) | r | L729 분해 · +0xb1 태그로 L739/757/765 검사 · L774 merge 인자(값 전달) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 33 | AgentVerHamster | 0x1d48 | failed_action | w | L695 retain(aux m06.ll · 60틱=1초 쿨다운) / L702~705(포기 Trace) / L720~723(무진전 RunAway·Recall) / L758~761(미발동 캐스트가 next 와 다를 때) / L767~770(Trace 가 비-Trace·비-캐스트 next 로 교체될 때). push 는 grow_one 경유 → ptr/cap/len 전부 변동 가능 | 4 | OK | retain(\|(t,_)\| t+60 >= now) · 기존 원소.0 = tick 갱신 · push((tick, act)) |
| 34 | AgentVerHamster | 0x1c0 | small_debug (DebugFrameData 224B) | w | L728 (구값 drop_glue → memcpy 224B). 디버그 모드면 L782 infos[champ.id].push(String) · L786 positiong_score.insert(champ.id, ps) | 4 | OK | DebugFrameData::default() 로 교체 후 get_small_action 이 채움 |
| 35 | AgentVerHamster | 0x530 | plan_system (LegacyPlanHandler 6168B) | w | L730 · 이 함수 직접 쓰기 없음 | 4 | OK | 콜리 get_small_action 경유 |
| 36 | AgentVerHamster | 0x2858 | small_action (SmallActionPlay 184B) | w | L774 · 콜리 SmallActionPlay::merge(m11.ll:42100) 경유 — 병합 규칙은 그 명세 | 4 | OK | merge(version, champ, next_action) |
| 37 | AgentVerHamster | 0x2918 | small_action_score | w | L776 (m14.ll:37595) | 4 | OK | next_score (get_small_action 반환 i64) |
| 38 | AgentVerHamster | 0x1d90 | positioning_score (PositioningScoreData 2760B) | w | L777 (m14.ll:37599) — score_parameter.rs:383 getter 인라인 | 4 | OK | score_parameter.positioning_score 통째 memcpy |
| 39 | AgentVerHamster | 0x2948 | last_eval_tick | w | L791 (m14.ll:37811) — can_skip_eval 의 10틱 예산 기준점 | 4 | OK | game.tick() |
| 40 | AgentVerHamster | 0x2950 | last_eval_hp | w | L792 (m14.ll:37815) | 4 | OK | champ.hp (+0x670) |
| 41 | AgentVerHamster | 0x29c8 | last_eval_nearby_enemies (u16) | w | L793 (m14.ll:37827) — 5비트 마스크 | 4 | OK | count_nearby_enemies(champ, player, data) |
| 42 | CNT_DM_IDLE_EVAL / CNT_DM_IDLE_PICK[ki] (전역 원자) | 0x0 | DM 계측 카운터 | w | L738/752 · DeathMatch && plan=DeathMatchBattle && idle_spec_tick==tick 일 때만 · 판정 영향 없음(관측 전용) | 4 | 확인불가(tcx 사전에 타입 없음) | atomicrmw add 1 |

**`consts` 상수 16건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev | folded_from |
|---|---|---|---|---|---|---|
| 0 | 14 | 699 | 태그 | SmallActionPlay 메모리태그 14 = Trace (tcxdict --enum: idx11 → 태그14). L765 도 동일 | 3 |  |
| 1 | 4 | 701 | 태그 | SmallAction::Trace 판별자 4 (Direct) — failed_action 원소 discr 비교 · 또한 `(tag-15) ult 4` 캐스트 계열 폭 | 4 |  |
| 2 | 119 | 714 | 임계 | 무진전 워치독: tick.saturating_sub(prog_best_tick) > 119 ⟹ ≥120틱(=2초@60tps) 진전 없음 (move_actions.rs:90 RunAway / :1020 Recall 인라인). prog_best_tick != 0 조건과 AND | 4 |  |
| 3 | 60 | 695 | 미상 | failed_action 보존 창: 기록 tick + 60 ≥ now 이면 유지(1초 쿨다운). aux m06.ll:2364/2428 (retain 클로저 인라인) | 4 |  |
| 4 | 2 | 734 | 센티널 | GameMode 태그 2 = DeathMatch (DM 계측 게이트) · 또한 L774 팀 bounds(ult 2) · L735 `ult 2` = BigPlan 니치 태그 0..1 = DeathMatchBattle | 4 |  |
| 5 | 6 | 735 | 태그 | BigPlan 태그 6 은 미사용 구멍(assume ne) — 판정 아님 | 4 |  |
| 6 | 3 | 713 | 태그 | SmallActionPlay 태그-3 스위치 접힘(RunAway=3 → case0 · Recall=4 → case1). small_action.rs:309 to_small_action 도 동일 접힘 | 4 |  |
| 7 | 7 | 713 | 태그 | 암묵(untagged) AroundPosition 의 스위치 인덱스(tag≤2 → 7) | 4 |  |
| 8 | 10 | 699 | 태그 | 태그 10 미사용 구멍(assume ne) — 판정 아님 | 4 |  |
| 9 | -19 | 757 | 미상 | small_action.rs:387: `(tag-19) as u8 < 252` ⟺ tag ∉ {15,16,17,18} — 캐스트 계열(Attack/Skill/Skill2/Ult) 여부의 보수 판정 | 4 | 15 |
| 10 | -4 | 757 | 임계 | 위 비교의 상한(i8 -4 = u8 252) | 4 | 252 |
| 11 | -15 | 758 | 미상 | small_action.rs:309: `(tag-15) ult 4` = matches!(Attack\|Skill\|Skill2\|Ult) 재판정(L757 경로에선 항상 참) | 4 | 15 |
| 12 | -14 | 765 | 태그 | next_action 태그 (tag-14) ult 5 ⟺ next ∈ {Trace14, Attack15, Skill16, Skill2 17, Ult18} 이면 Trace 실패 기록 생략 | 4 | 14 |
| 13 | 5 | 765 | 임계 | 위 range 폭(14..18) — `icmp ult i8 %485, 5`(m14.ll:37284). ⚠본문의 `shl … 5` 는 failed_action 원소 stride 32B(len×32) 의 접힘이지 이 상수의 용법이 아님(stride 라 등록 안 함) | 4 |  |
| 14 | 30 | 784 | 미상 | possible_risk(&score_parameter.player, data, 30) 의 3번째 인자 — 디버그 문자열 전용(판정 아님) | 4 |  |
| 15 | 8 | 782 | 태그 | Vec<String>::new() 댕글링 ptr(inttoptr 8) — or_default 인라인, 디버그 전용 | 4 |  |

**`knobs` 조정점 2건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | failed_action 쿨다운(틱) | lib.rs:695 · aux m06.ll:2364/2428 | 60 | 올리면 실패한 액션(포기 추격·무진전 도주·미발동 캐스트)이 경매에서 더 오래 배제됨(get_small_action 이 failed_action 을 참조) · 내리면 같은 실패를 빨리 재시도 | 4 | 기존 |
| 1 | 무진전 워치독 임계(틱) | lib.rs:714/715 · m14.ll:36168/36184 | 119 | 올리면 도주/귀환 경로가 더 오래 막혀 있어야 '실패'로 기록 · 내리면 잠깐의 정체도 실패로 기록해 경로 폴백이 빨라짐 | 4 | 기존 |

<details><summary>`callees` 피호출자 16건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | count_nearby_enemies | game_ai::AgentVerHamster::count_nearby_enemies | in:game_ai | fn(&game_core::Entity, &game_core::PlayerState, &game_core::OperationData) -> u16 | game-ai\src\lib.rs:830 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | find | game_ai::MinionWaveSnapshot::find | pub | fn(&game_ai::MinionWaveSnapshot, usize) -> std::option::Option<&game_ai::MinionHpTrajectory> | game-ai\src\utils.rs:84 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 2 | get_game_mode | game_core::AbstractGame::get_game_mode | pub | fn(&Self/#0) -> game_core::GameMode | game-core\src\simulation.rs:78 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 3 | get_game_mode | <game_core::Game as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::Game) -> game_core::GameMode | game-core\src\simulation\game.rs:1707 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 4 | get_game_mode | <game_core::SingleLaneGame as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::SingleLaneGame) -> game_core::GameMode | game-core\src\simulation\game.rs:4248 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 5 | get_small_action | game_ai::plan_legacy::handler::auction::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::get_small_action | pub | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &std::vec::Vec<(usize, game_core::SmallAction), std::alloc::Global>, &mut game_core::DebugFrameData) -> (game_ai::ScoreParameter, i64, game_ai::SmallActionPlay) | game-ai\src\plan_legacy\handler\auction.rs:8 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 6 | is_act | game_core::Input::is_act | pub | fn(&game_core::Input) -> bool | game-core\src\simulation\state\player.rs:1446 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 7 | merge | game_ai::SmallActionPlay::merge | pub | fn(&mut game_ai::SmallActionPlay, usize, &game_core::Entity, game_ai::SmallActionPlay) | game-ai\src\small_action.rs:397 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 8 | possible_risk | game_ai::ChampionScoreParameter::<'_>::possible_risk | pub | fn(&game_ai::ChampionScoreParameter</#0>, &game_core::OperationData, usize) -> i64 | game-ai\src\score_parameter.rs:1407 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 9 | push | <game_core::Staff as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::staff | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\staff.rs:38 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 10 | push | <game_core::Athlete as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:434 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 11 | push | <game_core::Contract as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:2498 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 12 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 13 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 14 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 15 | update_small_action | game_ai::AgentVerHamster::update_small_action | in:game_ai | fn(&mut game_ai::AgentVerHamster, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) | game-ai\src\lib.rs:693 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
</details>

⚠**미매칭 13개**: `abandoned`, `default  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 988개는 **전부 다른 함수**라 싣지 않는다`, `drop  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `entry`, `format_inner`, `grow_one`, `insert`, `insert_no_grow`, `or_default`, `retain`, `rustc_entry`, `target`, `to_small_action`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 3곳** (m14.ll:34966, m14.ll:39045, m14.ll:39079) · **형제 90개** (AgentVerHamster)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::AgentVerHamster as std::clone::Clone>::clone | pub | game-ai\src\lib.rs:121 | True | fn(&game_ai::AgentVerHamster) -> game_ai::AgentVerHamster |
| 1 | <game_ai::AgentVerHamster as std::fmt::Debug>::fmt | pub | game-ai\src\lib.rs:121 | True | fn(&game_ai::AgentVerHamster, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 2 | game_ai::AgentVerHamster::plan_chats_drain | pub | game-ai\src\lib.rs:197 | False | fn(&mut game_ai::AgentVerHamster) -> std::vec::Vec<game_core::Chat, std::alloc::Global> |
| 3 | game_ai::AgentVerHamster::push_pending_trace_event | pub | game-ai\src\lib.rs:208 | False | fn(&mut game_ai::AgentVerHamster, usize, game_core::TraceEventType) |
| 4 | game_ai::AgentVerHamster::plan_goal | pub | game-ai\src\lib.rs:211 | False | fn(&game_ai::AgentVerHamster) -> std::option::Option<game_core::BigGoal> |
| 5 | game_ai::AgentVerHamster::plan_name | pub | game-ai\src\lib.rs:214 | False | fn(&game_ai::AgentVerHamster) -> std::string::String |
| 6 | game_ai::AgentVerHamster::battle_sub_goal_dir | pub | game-ai\src\lib.rs:218 | False | fn(&game_ai::AgentVerHamster) -> std::option::Option<i8> |
| 7 | game_ai::AgentVerHamster::battle_sub_goal_is_full_runaway | pub | game-ai\src\lib.rs:222 | True | fn(&game_ai::AgentVerHamster) -> bool |
| 8 | game_ai::AgentVerHamster::plan_v3_cand_src | pub | game-ai\src\lib.rs:227 | True | fn(&game_ai::AgentVerHamster) -> u8 |
| 9 | game_ai::AgentVerHamster::plan_v3_last_stand | pub | game-ai\src\lib.rs:231 | True | fn(&game_ai::AgentVerHamster) -> bool |
| 10 | game_ai::AgentVerHamster::plan_v3_final_stand | pub | game-ai\src\lib.rs:235 | True | fn(&game_ai::AgentVerHamster) -> bool |
| 11 | game_ai::AgentVerHamster::plan_v3_bail_goal | pub | game-ai\src\lib.rs:239 | True | fn(&game_ai::AgentVerHamster) -> u8 |
| 12 | game_ai::AgentVerHamster::plan_ff_line_counts | pub | game-ai\src\lib.rs:243 | True | fn(&game_ai::AgentVerHamster) -> [usize; 4_usize] |
| 13 | game_ai::AgentVerHamster::plan_ff_retreat_stats | pub | game-ai\src\lib.rs:247 | True | fn(&game_ai::AgentVerHamster) -> (usize, usize, usize, usize, usize) |
| 14 | game_ai::AgentVerHamster::team_objective_code | pub | game-ai\src\lib.rs:254 | True | fn(&game_ai::AgentVerHamster) -> u8 |
| 15 | game_ai::AgentVerHamster::eo_cover_picks | pub | game-ai\src\lib.rs:258 | False | fn(&game_ai::AgentVerHamster) -> usize |
| 16 | game_ai::AgentVerHamster::eo_serpen_punish_issues | pub | game-ai\src\lib.rs:262 | True | fn(&game_ai::AgentVerHamster) -> usize |
| 17 | game_ai::AgentVerHamster::subplan_is_recall | pub | game-ai\src\lib.rs:266 | True | fn(&game_ai::AgentVerHamster) -> bool |
| 18 | game_ai::AgentVerHamster::plan_ff_call_stats | pub | game-ai\src\lib.rs:270 | True | fn(&game_ai::AgentVerHamster) -> (usize, usize, usize, usize, usize, usize, usize, usize) |
| 19 | game_ai::AgentVerHamster::plan_gank_attempt_count | pub | game-ai\src\lib.rs:275 | True | fn(&game_ai::AgentVerHamster) -> usize |
| 20 | game_ai::AgentVerHamster::plan_gank_periods | pub | game-ai\src\lib.rs:278 | True | fn(&game_ai::AgentVerHamster) -> &std::vec::Vec<(usize, usize), std::alloc::Global> |
| 21 | game_ai::AgentVerHamster::plan_gank_score_attempts | pub | game-ai\src\lib.rs:281 | True | fn(&game_ai::AgentVerHamster) -> &std::vec::Vec<(i32, usize, usize), std::alloc::Global> |
| 22 | game_ai::AgentVerHamster::plan_gank_request_count | pub | game-ai\src\lib.rs:284 | True | fn(&game_ai::AgentVerHamster) -> usize |
| 23 | game_ai::AgentVerHamster::plan_v46_lane_recall_stats | pub | game-ai\src\lib.rs:289 | True | fn(&game_ai::AgentVerHamster) -> (&std::vec::Vec<(usize, game_core::LineType), std::alloc::Global>, usize, usize, usize, usize, usize, usize, usize, usize) |
| 24 | game_ai::AgentVerHamster::plan_flee_death_retrospects | pub | game-ai\src\lib.rs:297 | True | fn(&game_ai::AgentVerHamster) -> &std::vec::Vec<(usize, u8, u8, u8), std::alloc::Global> |
| 25 | game_ai::AgentVerHamster::plan_v48_dodge_stats | pub | game-ai\src\lib.rs:302 | True | fn(&game_ai::AgentVerHamster) -> (usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) |
| 26 | game_ai::AgentVerHamster::plan_v46_flee_stats | pub | game-ai\src\lib.rs:310 | True | fn(&game_ai::AgentVerHamster) -> (usize, usize, usize, usize, usize, usize) |
| 27 | game_ai::AgentVerHamster::plan_v46_flee_episodes | pub | game-ai\src\lib.rs:317 | True | fn(&game_ai::AgentVerHamster) -> &std::vec::Vec<(usize, game_core::LineType, u8), std::alloc::Global> |
| 28 | game_ai::AgentVerHamster::plan_v50_dive_episodes | pub | game-ai\src\lib.rs:321 | True | fn(&game_ai::AgentVerHamster) -> &std::vec::Vec<game_core::V50DiveEpisode, std::alloc::Global> |
| 29 | game_ai::AgentVerHamster::plan_epic_steal_attempt_count | pub | game-ai\src\lib.rs:324 | True | fn(&game_ai::AgentVerHamster) -> usize |
| 30 | game_ai::AgentVerHamster::plan_epic_steal_success_count | pub | game-ai\src\lib.rs:327 | True | fn(&game_ai::AgentVerHamster) -> usize |
| 31 | game_ai::AgentVerHamster::plan_serpen_steal_attempt_count | pub | game-ai\src\lib.rs:330 | True | fn(&game_ai::AgentVerHamster) -> usize |
| 32 | game_ai::AgentVerHamster::plan_serpen_steal_success_count | pub | game-ai\src\lib.rs:333 | True | fn(&game_ai::AgentVerHamster) -> usize |
| 33 | game_ai::AgentVerHamster::plan_steal_sessions | pub | game-ai\src\lib.rs:336 | False | fn(&game_ai::AgentVerHamster) -> std::vec::Vec<game_core::StealSession, std::alloc::Global> |
| 34 | game_ai::AgentVerHamster::plan_pending_trace_events_drain | pub | game-ai\src\lib.rs:339 | False | fn(&mut game_ai::AgentVerHamster) -> std::vec::Vec<game_core::PendingTraceEvent, std::alloc::Global> |
| 35 | game_ai::AgentVerHamster::plan_comeback_pick_stats | pub | game-ai\src\lib.rs:342 | False | fn(&game_ai::AgentVerHamster) -> (usize, usize, std::vec::Vec<(i32, u8), std::alloc::Global>) |
| 36 | game_ai::AgentVerHamster::plan_v54_cj_call_ticks | pub | game-ai\src\lib.rs:346 | True | fn(&game_ai::AgentVerHamster) -> &std::vec::Vec<usize, std::alloc::Global> |
| 37 | game_ai::AgentVerHamster::plan_is_counter_jungle | pub | game-ai\src\lib.rs:350 | True | fn(&game_ai::AgentVerHamster) -> bool |
| 38 | game_ai::AgentVerHamster::plan_v54_fs_fog_stats | pub | game-ai\src\lib.rs:357 | False | fn(&game_ai::AgentVerHamster) -> (usize, usize) |
| 39 | game_ai::AgentVerHamster::plan_v54_reentry_ticks | pub | game-ai\src\lib.rs:362 | True | fn(&game_ai::AgentVerHamster) -> &std::vec::Vec<usize, std::alloc::Global> |
| 40 | game_ai::AgentVerHamster::new | pub | game-ai\src\lib.rs:368 | False | fn(&mut rand::rngs::std::StdRng, usize, usize, game_core::Position) -> game_ai::AgentVerHamster |
| 41 | game_ai::AgentVerHamster::init | pub | game-ai\src\lib.rs:413 | True | fn(&mut game_ai::AgentVerHamster) |
| 42 | <game_ai::AgentVerHamster as game_core::AiAgent>::as_any | pub | game-ai\src\lib.rs:425 | True | fn(&game_ai::AgentVerHamster) -> &dyn [Binder { value: Trait(std::any::Any), bound_vars: [] }] + 'static |
| 43 | <game_ai::AgentVerHamster as game_core::AiAgent>::as_any_mut | pub | game-ai\src\lib.rs:426 | True | fn(&mut game_ai::AgentVerHamster) -> &mut dyn [Binder { value: Trait(std::any::Any), bound_vars: [] }] + 'static |
| 44 | <game_ai::AgentVerHamster as game_core::AiAgent>::version | pub | game-ai\src\lib.rs:428 | True | fn(&game_ai::AgentVerHamster) -> usize |
| 45 | <game_ai::AgentVerHamster as game_core::AiAgent>::set_version | pub | game-ai\src\lib.rs:429 | True | fn(&mut game_ai::AgentVerHamster, usize) |
| 46 | <game_ai::AgentVerHamster as game_core::AiAgent>::debug_mut | pub | game-ai\src\lib.rs:430 | True | fn(&mut game_ai::AgentVerHamster) -> &mut game_core::DebugFrameData |
| 47 | <game_ai::AgentVerHamster as game_core::AiAgent>::small_action_current | pub | game-ai\src\lib.rs:431 | False | fn(&game_ai::AgentVerHamster) -> game_core::SmallAction |
| 48 | <game_ai::AgentVerHamster as game_core::AiAgent>::get_input | pub | game-ai\src\lib.rs:433 | False | fn(&mut game_ai::AgentVerHamster, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> (std::option::Option<game_core::Input>, bumpalo::collections::vec::Vec< game_core::TurnEvent>) |
| 49 | <game_ai::AgentVerHamster as game_core::AiAgent>::buy_item | pub | game-ai\src\lib.rs:437 | False | fn(&mut game_ai::AgentVerHamster, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::GameContext) -> std::option::Option<usize> |
| 50 | <game_ai::AgentVerHamster as game_core::AiAgent>::upgrade_item | pub | game-ai\src\lib.rs:440 | False | fn(&mut game_ai::AgentVerHamster, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::GameContext) -> std::option::Option<(usize, usize)> |
| 51 | <game_ai::AgentVerHamster as game_core::AiAgent>::update_on_dead | pub | game-ai\src\lib.rs:443 | False | fn(&mut game_ai::AgentVerHamster, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) |
| 52 | <game_ai::AgentVerHamster as game_core::AiAgent>::plan_goal | pub | game-ai\src\lib.rs:447 | False | fn(&game_ai::AgentVerHamster) -> std::option::Option<game_core::BigGoal> |
| 53 | <game_ai::AgentVerHamster as game_core::AiAgent>::plan_debug_label | pub | game-ai\src\lib.rs:448 | False | fn(&game_ai::AgentVerHamster) -> std::string::String |
| 54 | <game_ai::AgentVerHamster as game_core::AiAgent>::push_game_event | pub | game-ai\src\lib.rs:449 | False | fn(&mut game_ai::AgentVerHamster, game_core::TurnEvent) |
| 55 | <game_ai::AgentVerHamster as game_core::AiAgent>::push_pending_trace_event | pub | game-ai\src\lib.rs:450 | False | fn(&mut game_ai::AgentVerHamster, usize, game_core::TraceEventType) |
| 56 | <game_ai::AgentVerHamster as game_core::AiAgent>::plan_chats_drain | pub | game-ai\src\lib.rs:453 | False | fn(&mut game_ai::AgentVerHamster) -> std::vec::Vec<game_core::Chat, std::alloc::Global> |
| 57 | <game_ai::AgentVerHamster as game_core::AiAgent>::plan_pending_trace_events_drain | pub | game-ai\src\lib.rs:454 | False | fn(&mut game_ai::AgentVerHamster) -> std::vec::Vec<game_core::PendingTraceEvent, std::alloc::Global> |
| 58 | <game_ai::AgentVerHamster as game_core::AiAgent>::plan_epic_steal_attempt_count | pub | game-ai\src\lib.rs:458 | True | fn(&game_ai::AgentVerHamster) -> usize |
| 59 | <game_ai::AgentVerHamster as game_core::AiAgent>::plan_epic_steal_success_count | pub | game-ai\src\lib.rs:459 | True | fn(&game_ai::AgentVerHamster) -> usize |
| 60 | <game_ai::AgentVerHamster as game_core::AiAgent>::plan_serpen_steal_attempt_count | pub | game-ai\src\lib.rs:460 | True | fn(&game_ai::AgentVerHamster) -> usize |
| 61 | <game_ai::AgentVerHamster as game_core::AiAgent>::plan_serpen_steal_success_count | pub | game-ai\src\lib.rs:461 | True | fn(&game_ai::AgentVerHamster) -> usize |
| 62 | <game_ai::AgentVerHamster as game_core::AiAgent>::plan_steal_sessions | pub | game-ai\src\lib.rs:462 | False | fn(&game_ai::AgentVerHamster) -> std::vec::Vec<game_core::StealSession, std::alloc::Global> |
| 63 | <game_ai::AgentVerHamster as game_core::AiAgent>::plan_gank_attempt_count | pub | game-ai\src\lib.rs:463 | True | fn(&game_ai::AgentVerHamster) -> usize |
| 64 | <game_ai::AgentVerHamster as game_core::AiAgent>::plan_gank_request_count | pub | game-ai\src\lib.rs:464 | True | fn(&game_ai::AgentVerHamster) -> usize |
| 65 | <game_ai::AgentVerHamster as game_core::AiAgent>::plan_gank_periods | pub | game-ai\src\lib.rs:465 | True | fn(&game_ai::AgentVerHamster) -> &std::vec::Vec<(usize, usize), std::alloc::Global> |
| 66 | <game_ai::AgentVerHamster as game_core::AiAgent>::plan_gank_score_attempts | pub | game-ai\src\lib.rs:466 | True | fn(&game_ai::AgentVerHamster) -> &std::vec::Vec<(i32, usize, usize), std::alloc::Global> |
| 67 | <game_ai::AgentVerHamster as game_core::AiAgent>::plan_comeback_pick_stats | pub | game-ai\src\lib.rs:467 | False | fn(&game_ai::AgentVerHamster) -> (usize, usize, std::vec::Vec<(i32, u8), std::alloc::Global>) |
| 68 | <game_ai::AgentVerHamster as game_core::AiAgent>::plan_flee_death_retrospects | pub | game-ai\src\lib.rs:468 | True | fn(&game_ai::AgentVerHamster) -> &std::vec::Vec<(usize, u8, u8, u8), std::alloc::Global> |
| 69 | <game_ai::AgentVerHamster as game_core::AiAgent>::plan_v46_flee_stats | pub | game-ai\src\lib.rs:469 | True | fn(&game_ai::AgentVerHamster) -> (usize, usize, usize, usize, usize, usize) |
| 70 | <game_ai::AgentVerHamster as game_core::AiAgent>::plan_v46_flee_episodes | pub | game-ai\src\lib.rs:470 | True | fn(&game_ai::AgentVerHamster) -> &std::vec::Vec<(usize, game_core::LineType, u8), std::alloc::Global> |
| 71 | <game_ai::AgentVerHamster as game_core::AiAgent>::plan_v46_lane_recall_stats | pub | game-ai\src\lib.rs:471 | True | fn(&game_ai::AgentVerHamster) -> (&std::vec::Vec<(usize, game_core::LineType), std::alloc::Global>, usize, usize, usize, usize, usize, usize, usize, usize) |
| 72 | <game_ai::AgentVerHamster as game_core::AiAgent>::plan_v48_dodge_stats | pub | game-ai\src\lib.rs:474 | True | fn(&game_ai::AgentVerHamster) -> (usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) |
| 73 | <game_ai::AgentVerHamster as game_core::AiAgent>::plan_v50_dive_episodes | pub | game-ai\src\lib.rs:477 | True | fn(&game_ai::AgentVerHamster) -> &std::vec::Vec<game_core::V50DiveEpisode, std::alloc::Global> |
| 74 | game_ai::AgentVerHamster::get_play_type | pub | game-ai\src\lib.rs:552 | True | fn(&game_ai::AgentVerHamster) -> game_core::PlayType |
| 75 | game_ai::AgentVerHamster::update_state | pub | game-ai\src\lib.rs:556 | False | fn(&mut game_ai::AgentVerHamster, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bumpalo::collections::vec::Vec< game_core::TurnEvent> |
| 76 | game_ai::AgentVerHamster::update_event | in:game_ai | game-ai\src\lib.rs:612 | False | fn(&mut game_ai::AgentVerHamster, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) |
| 77 | game_ai::AgentVerHamster::update_plan | in:game_ai | game-ai\src\lib.rs:682 | False | fn(&mut game_ai::AgentVerHamster, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bumpalo::collections::vec::Vec< game_core::TurnEvent> |
| 78 | game_ai::AgentVerHamster::update_plan_lapse | in:game_ai | game-ai\src\lib.rs:688 | False | fn(&mut game_ai::AgentVerHamster, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, bool) -> bumpalo::collections::vec::Vec< game_core::TurnEvent> |
| 79 | game_ai::AgentVerHamster::update_small_action | in:game_ai | game-ai\src\lib.rs:693 | False | fn(&mut game_ai::AgentVerHamster, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) |
| 80 | game_ai::AgentVerHamster::can_skip_eval | in:game_ai | game-ai\src\lib.rs:796 | False | fn(&game_ai::AgentVerHamster, &game_core::PlayerState, &game_core::OperationData) -> bool |
| 81 | game_ai::AgentVerHamster::count_nearby_enemies | in:game_ai | game-ai\src\lib.rs:830 | False | fn(&game_core::Entity, &game_core::PlayerState, &game_core::OperationData) -> u16 |
| 82 | game_ai::AgentVerHamster::get_input | pub | game-ai\src\lib.rs:843 | False | fn(&mut game_ai::AgentVerHamster, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> (std::option::Option<game_core::Input>, bumpalo::collections::vec::Vec< game_core::TurnEvent>) |
| 83 | game_ai::AgentVerHamster::update_trace_escape_abandon | in:game_ai | game-ai\src\lib.rs:1088 | False | fn(&mut game_ai::AgentVerHamster, &game_core::OperationData) |
| 84 | game_ai::AgentVerHamster::update_on_dead | pub | game-ai\src\lib.rs:1113 | False | fn(&mut game_ai::AgentVerHamster, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) |
| 85 | game_ai::AgentVerHamster::push_game_event | pub | game-ai\src\lib.rs:1120 | False | fn(&mut game_ai::AgentVerHamster, game_core::TurnEvent) |
| 86 | game_ai::AgentVerHamster::item_v26_slot | in:game_ai | game-ai\src\lib.rs:1125 | False | fn(&game_core::PlayerState, &[std::boxed::Box<dyn [Binder { value: Trait(game_core::ItemInfo), bound_vars: [] }] + 'static, std::alloc::Global>]) -> std::option::Option<(usize, std::option::Option<usize>)> |
| 87 | game_ai::AgentVerHamster::item_v26 | in:game_ai | game-ai\src\lib.rs:1149 | False | fn(&mut game_ai::AgentVerHamster, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::GameContext, usize, std::option::Option<usize>) -> std::option::Option<usize> |
| 88 | game_ai::AgentVerHamster::buy_item | pub | game-ai\src\lib.rs:1173 | False | fn(&mut game_ai::AgentVerHamster, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::GameContext) -> std::option::Option<usize> |
| 89 | game_ai::AgentVerHamster::upgrade_item | pub | game-ai\src\lib.rs:1193 | False | fn(&mut game_ai::AgentVerHamster, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::GameContext) -> std::option::Option<(usize, usize)> |

**`open` 7건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | LegacyPlanHandler::get_small_action(m13.ll:45629) 계약만: (sret 5576B, &mut LegacyPlanHandler, version, &mut StdRng, &PlayerState, &OperationData, &SmallActionPlay 현재, &Vec<(usize,SmallAction)> failed_action, &mut DebugFrameData small_debug) → sret = {ScoreParameter 5384B @0, next_score i64 @5384, SmallActionPlay 184B @5392}. 내부(경매·후보 생성)는 r16 루트 *SubPlan::action_candidates 계열 명세 | 4 |  |
| 1 | 미탐색 | SmallActionPlay::merge(m11.ll:42100) 계약만: (&mut SmallActionPlay, version: usize, champ: &Entity, next: SmallActionPlay by-value(dead_on_return ptr)) → (). 어떤 필드를 이어받는지(start_tick 유지 등)는 그 명세 | 4 |  |
| 2 | 미탐색 | ChampionScoreParameter::possible_risk(m07.ll:7315): (&ChampionScoreParameter 216B, &OperationData, i64=30) → i64 — 디버그 문자열에만 쓰여 판정 무관. 30 의 의미 미독해 | 4 |  |
| 3 | 표기 불가 | L699 조건이 소스에서 `if let Trace(t) = … && t.abandoned` 한 식인지 중첩 if 인지 — 외연 동일(표기 불가) | 4 |  |
| 4 | 표기 불가 | L713 stalled 헬퍼 이름(move_actions.rs:90 / :1020 인라인) — DI 변수명 `stalled` 만 관측. `> 119` 가 소스에서 `>= 120` 인지 `> 119` 인지는 표기 불가 | 4 |  |
| 5 | 미탐색 | stalled 경로(L719~723)에서 to_small_action 의 17-way 스위치가 전부 살아있다고 reach 는 보고하지만, 이 문맥 도달 태그는 3·4 뿐이라 실제 기록되는 act 는 항상 SmallAction::RunAway — 경로 민감 분석 미수행(reach.py 는 태그 상수를 접지 않음) | 4 |  |
| 6 | 미탐색 | SmallActionPlay 태그 10 과 BigPlan 태그 6 은 `llvm.assume(ne)` 로만 등장 — 미사용 구멍으로 판단, 판정 상수 아님 | 4 |  |

**`notes` 1건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | L757 `is_act` 필드의 정확한 의미(캐스트 입력이 이미 발행됐는가)는 SmallActionAttack 등 cast.rs 소관 — 여기선 '!is_act 인 캐스트를 다른 액션으로 갈아치우면 실패로 기록' 동작만 확정 | 4 | 사실 서술 |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

