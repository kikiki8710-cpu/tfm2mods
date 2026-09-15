---

### `218` AgentVerHamster::update_state — 틱당 AI 상태 갱신 루트: 이벤트→(lapse 게이트)플랜 update→소액션 update_state→(전제상실/종료 ∧ !can_skip_eval)이면 update_small_action, 아니면 extend_action. 반환 Vec 은 항상 빈 것

| 항목 | 값 |
|---|---|
| id | `AgentVerHamster__update_state` |
| 심볼 | `_RNvMs1_CshdEBA0ozCnw_7game_aiNtB5_15AgentVerHamster12update_state` |
| 소스 | `game-ai\src\lib.rs:556` |
| IR | `m14.ll` 34283~35106행 |
| 경로·가시성 | `game_ai::AgentVerHamster::update_state` · **pub** |
| 계층 | 기타 |
| exe | `e8d6f0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r17` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&mut game_ai::AgentVerHamster, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bumpalo::collections::vec::Vec< game_core::TurnEvent>
```

**TLS 메모 절(`sig.tls` · 정정 경로 `/specs[218]/sig/tls/<키>`)**

없음 — 본문 직접 TLS 접점 0(`@anon.* = constant ptr @…call_once` 참조 없음). prof::ENABLED/PHASE_NANOS/PHASE_CALLS 는 `external global`(프로세스 전역 원자, TLS 아님)

<details><summary>인자 5개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | ptr sret([32 x i8]) (%0) | L609 에서 res(%13) 32B 통째 memcpy | 4 |
| 1 | 1 | self | &mut AgentVerHamster(10704B) (%1) | 직접 쓰기 = last_lapse · small_action.start_tick(extend_action). 나머지는 콜리 경유(writes 참조) | 4 |
| 2 | 2 | rnd | &mut StdRng(320B) (%2) | gen_range 호출 사이트 = 본문 0 | 4 |
| 3 | 3 | player | &PlayerState(2528B) (%3) | +0x930 info.team · +0x9c0 info.position 읽음(can_skip_eval) | 4 |
| 4 | 4 | data | &OperationData(24B) (%4) | +0 cache · +8 context · +16 blackboard 전부 읽음 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn update_state(&mut self, rnd, player, data) -> Vec<TurnEvent>   // lib.rs:556~610
  _t_ev = ProfTimer::start(23)                                  // L558 (ENABLED 원자 0 이면 None)
  self.update_event(rnd, player, data.cache, data.context)      // L559 (콜리 · 명세 밖)
  drop(_t_ev)                                                   // L560 → PHASE_NANOS[23]+=ns, PHASE_CALLS[23]+=1
  _t_plan = ProfTimer::start(24)                                // L562
  lapse = utils::player_awareness_lapse(player, data)           // L566
  self.last_lapse = lapse                                       // +0x29ca 쓰기
  res = if self.version > 1 {                                   // L569 (+0x2910)
          self.update_plan_lapse(rnd, player, data, lapse)       // L570 → lib.rs:689 인라인: plan_system.update(version, rnd, player, data, &mut self.debug, lapse); Vec::new_in(pool)
        } else if lapse {                                       // L571 (v≤1 · 인지공백)
          Vec::new_in(data.context.pool)                        // L572 — 플랜 update 생략
        } else {
          self.update_plan(rnd, player, data)                   // L574 → lib.rs:683: plan_system.update(version, rnd, player, data, &mut self.debug, false); Vec::new_in(pool)
        }                                                       // ★세 경로 모두 빈 Vec
  drop(_t_plan)                                                 // L576
  _t_sa = ProfTimer::start(25)                                  // L577
  { _t_sus = ProfTimer::start(28)                               // L579
    self.small_action.update_state(rnd, player, data, &mut self.debug)   // L580 (+0x2858)
    drop(_t_sus) }                                              // L581
  need_new = (self.version > 1 && self.small_action.is_premise_lost(data))     // L589 — 단락: v≤1 이면 is_premise_lost 미호출
             || self.small_action.is_end(rnd, self.version, player, data)        // L590 — premise_lost 가 true 면 is_end 미호출
  if need_new:
    // premise_lost 경로는 can_skip_eval 을 건너뛰고 바로 재평가(%165→%285). is_end 경로만 아래 검사
    if premise_lost || !self.can_skip_eval(player, data):        // L591 (lib.rs:796~824 인라인)
      _t_usa = ProfTimer::start(29)                              // L594
      self.update_small_action(rnd, player, data)                // L595 (별도 명세)
      drop(_t_usa)                                               // L596
    else:
      self.small_action.extend_action(game.tick())               // L592 → small_action.rs:445: 현 variant 의 start_tick = tick (Recall +0x48 · Trace +0x58 · 나머지 이동계 +0x0 · 캐스트/Stop 은 무시)
  drop(_t_sa)                                                   // L598
  if data.context.debug {                                       // L600 (+0x3b)
    self.debug.merge(self.big_debug.clone())                    // L601
    self.debug.merge(self.small_debug.clone())                  // L602
    if let Some(pf) = self.small_action.path_finder() {         // L604 (null 검사)
      pf.draw_debug(player, data, &mut self.debug) } }          // L605
  return res                                                    // L609 (빈 Vec)

---- can_skip_eval(&self, player, data) -> bool  (lib.rs:796~824 · 인라인 · L591 문맥) ----
  tick = game.tick()                                            // L797 (vtable+0x28)
  if game.get_game_mode().tag == 2 (DeathMatch): return false   // L801 (vtable+0x40)
  if !(tick < self.last_eval_tick + 10): return false           // L806 (+0x2948) — 재평가 예산 10틱
  if self.small_action.is_action_complete(): return false       // L811 → small_action.rs:463: tag∈{15,16,17,18}(Attack/Skill/Skill2/Ult) && payload.is_act(+0x10)
  champ = data.cache.player_champion[player.info.team][player.info.position].unwrap()   // L816 (team ult 2 bounds · None→unwrap_failed)
  if champ.hp < self.last_eval_hp: return false                 // L817 (+0x670 vs +0x2950) — 피해 입었으면 재평가
  current_enemies = count_nearby_enemies(champ, player, data)   // L822 (u16 마스크)
  return current_enemies == self.last_eval_nearby_enemies       // L823 (+0x29c8) — 근접 적 구성 불변이면 생략 가능

※ 분기 극성 근거: %165 `br %164 → %285(재평가) / %166`, %171 `br %170 → %195(can_skip 검사) / %172(아무것도 안 함)`, %258 `eq → %259(extend) / %285(재평가)`.
※ 개발자 주석(_docs game_ai.txt:9~11): is_premise_lost 는 '재평가 예산(can_skip_eval)으로 연장해서는 안 되는 유일한 조건' — IR 과 일치(premise_lost 경로는 can_skip_eval 우회).
```

**`mem` 메모리 접근 32건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | AgentVerHamster | 0x2910 | version | r | L569 `> 1` (버전 게이트 v2+) · L589 `> 1` 재로드 · is_end 인자로 전달 | 4 | OK |  |
| 1 | AgentVerHamster | 0x2948 | last_eval_tick | r | can_skip_eval L806: tick < last_eval_tick + 10 | 4 | OK |  |
| 2 | AgentVerHamster | 0x2909 | small_action@tag (small_action+0xb1) | r | is_action_complete(L811→small_action.rs:463): (tag-15) < 4 ⟹ Attack15/Skill16/Skill2 17/Ult18 · extend_action(L592→:445) switch | 4 | OK |  |
| 3 | AgentVerHamster | 0x2868 | small_action.<Attack\|Skill\|Skill2\|Ult>.is_act (payload+0x10) | r | is_action_complete: 캐스트 계열이고 is_act 이면 '완료' → can_skip_eval false | 4 | 부분일치(첫 성분(컨테이너)만 맞고 **마지막 성분이 다르다** — 사람이 봐야 ) |  |
| 4 | AgentVerHamster | 0x2950 | last_eval_hp | r | can_skip_eval L817: champ.hp < last_eval_hp 이면 false | 4 | OK |  |
| 5 | AgentVerHamster | 0x29c8 | last_eval_nearby_enemies | r | can_skip_eval L823: count_nearby_enemies(...) != 이 값이면 false | 4 | OK |  |
| 6 | AgentVerHamster | 0x530 | plan_system | r | &mut 로 LegacyPlanHandler::update 에 전달(L570/L574) | 4 | OK |  |
| 7 | AgentVerHamster | 0x2858 | small_action | r | &mut 로 SmallActionPlay::update_state/is_premise_lost/is_end/path_finder/extend_action 에 전달 | 4 | OK |  |
| 8 | AgentVerHamster | 0xe0 | big_debug | r | L601 clone → self.debug.merge (context.debug 일 때만) | 4 | OK |  |
| 9 | AgentVerHamster | 0x1c0 | small_debug | r | L602 clone → self.debug.merge | 4 | OK |  |
| 10 | AgentVerHamster | 0x0 | debug (DebugFrameData 224B) | r | &mut 로 plan_system.update · small_action.update_state · merge · draw_debug 의 224B 인자(dereferenceable(224)) | 4 | OK |  |
| 11 | OperationData | 0x0 | cache | r | update_event 인자 · can_skip_eval 의 game/player_champion | 4 | OK |  |
| 12 | OperationData | 0x8 | context | r | update_event 인자 · pool · debug 플래그 | 4 | OK |  |
| 13 | OperationData | 0x10 | blackboard | r | count_nearby_enemies 5번째 인자 | 4 | OK |  |
| 14 | GameContext | 0x0 | pool (&Bump) | r | Vec::new_in 의 할당자(반환 Vec +0x8 에 저장) | 4 | OK |  |
| 15 | GameContext | 0x3b | debug | r | L600 true 이면 debug 병합 + path_finder draw_debug | 4 | OK |  |
| 16 | AbstractGameWithCache | 0x0 | game.data_ptr | r | vtable 호출 self | 4 | OK |  |
| 17 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r | +0x28 tick() · +0x40 get_game_mode() (divtable AbstractGame) | 3 | OK |  |
| 18 | AbstractGameWithCache | 0x1e0 | player_champion[team][position] | r | can_skip_eval L816 · None 이면 unwrap_failed(lib.rs:1013 경유 헬퍼) | 4 | OK |  |
| 19 | PlayerState | 0x930 | info.team | r | bounds ult 2 (panic_bounds_check) | 4 | OK |  |
| 20 | PlayerState | 0x9c0 | info.position@tag (i32) | r | player_champion 2차 인덱스(player.rs:581 인라인) | 4 | OK |  |
| 21 | Entity | 0x670 | hp (자기 챔피언) | r | L817 | 4 | OK |  |
| 22 | Entity | 0x660 | x (자기 챔피언) | r | L822 count_nearby_enemies 인자(ArgumentPromotion) | 4 | OK |  |
| 23 | Entity | 0x668 | y (자기 챔피언) | r | L822 | 4 | OK |  |
| 24 | AgentVerHamster | 0x29ca | last_lapse | w | L566 · 무조건 매 호출 기록(m14.ll:34434) | 4 | OK | player_awareness_lapse(player, data) 결과(bool→i8 zext) |
| 25 | AgentVerHamster | 0x2858 | small_action.<variant>.start_tick | w | L592 extend_action(small_action.rs:445~457) — can_skip_eval true 일 때만. 태그별 목적지: RunAway3/Around5/AroundHide6/AroundRegion7/AroundRunAway8/Positioning9/AroundPosition(암묵)/AroundPositionBush11/AroundBush12/LaneMinionPosition13 → +0x2858(payload+0x0) · Recall4 → +0x28a0(payload+0x48) · Trace14 → +0x28b0(payload+0x58) · Attack/Skill/Skill2/Ult/Stop(15~19) → 쓰기 없음(switch default) | 4 | OK | game.tick() |
| 26 | AgentVerHamster | 0x28a0 | small_action.Recall.start_tick | w | extend_action case 1 (m14.ll:34893) | 4 | OK | game.tick() |
| 27 | AgentVerHamster | 0x28b0 | small_action.Trace.start_tick | w | extend_action case 11 (m14.ll:34944) | 4 | OK | game.tick() |
| 28 | AgentVerHamster | 0x0 | debug (DebugFrameData) | w | L601/602/605 · context.debug 일 때만. 콜리 경유(DebugFrameData::merge · PathFinder::draw_debug) | 4 | OK | merge(big_debug.clone()) · merge(small_debug.clone()) · draw_debug |
| 29 | AgentVerHamster | * | (콜리 경유 전체) | w | 이 함수의 &mut self 표면은 콜리 4개의 합집합. 본 명세 직접 쓰기는 위 5행 | 4 | 확인불가(오프셋 파싱 실패) | update_event(self 전체) · plan_system.update(+0x530 · 6168B) · small_action.update_state(+0x2858 · 184B) · update_small_action(self 전체 — 그 명세 참조) |
| 30 | StdRng | * | rnd | w | 본문 gen_range 0 | 4 | 확인불가(오프셋 파싱 실패) | 콜리 경유만 |
| 31 | prof::PHASE_NANOS/PHASE_CALLS (전역) | phase*8 | 프로파일 카운터 | w | ENABLED != 0 일 때만 · phase 23(_t_ev) · 24(_t_plan) · 25(_t_sa) · 28(_t_sus) · 29(_t_usa) · 게임 판정과 무관 | 4 | 확인불가(tcx 사전에 타입 없음) | atomicrmw add |

**`consts` 상수 16건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev | folded_from |
|---|---|---|---|---|---|---|
| 0 | 1 | 569 | 임계 | version > 1 ⟹ v2+ 에서만 lapse 를 플랜 update 에 전달(update_plan_lapse). L589 동일 조건으로 is_premise_lost 검사 | 4 |  |
| 1 | 10 | 806 | 태그 | can_skip_eval: 마지막 평가 후 10틱 미만이어야 재평가 생략 가능(재평가 예산 창) | 4 |  |
| 2 | 2 | 801 | 태그 | GameMode 태그 2 = DeathMatch ⟹ can_skip_eval 항상 false. 또한 L816 팀 bounds(ult 2) | 4 |  |
| 3 | -15 | 463 | 태그 | small_action.rs:463 is_action_complete: 태그 15(Attack) 기준 오프셋 — `add nsw i8 %tag, -15; icmp ult 4` 로 접힌 matches!(Attack\|Skill\|Skill2\|Ult). 본문 리터럴은 -15 | 4 | 15 |
| 4 | 4 | 463 | 태그 | 위 range 폭(15..19 = 4 variant) | 4 |  |
| 5 | 3 | 445 | 태그 | extend_action switch: 태그-3 (RunAway=3 이 case 0) · tag≤2(암묵 AroundPosition) 는 7 로 매핑 | 4 |  |
| 6 | 7 | 445 | 태그 | 암묵(untagged) AroundPosition 의 switch 인덱스 | 4 |  |
| 7 | 8 | 547 | 태그 | bumpalo Vec::new_in 의 댕글링 ptr(inttoptr 8 · align 8) — 반환 Vec +0x0 | 4 |  |
| 8 | 23 | 558 | 산출값 | ProfTimer phase id (_t_ev · update_event 구간) | 4 |  |
| 9 | 24 | 562 | 산출값 | ProfTimer phase id (_t_plan) | 4 |  |
| 10 | 25 | 577 | 산출값 | ProfTimer phase id (_t_sa · 소액션 전체) | 4 |  |
| 11 | 28 | 579 | 산출값 | ProfTimer phase id (_t_sus · small_action.update_state) | 4 |  |
| 12 | 29 | 594 | 산출값 | ProfTimer phase id (_t_usa · update_small_action) | 4 |  |
| 13 | 132 | 825 | 길이 | PHASE_NANOS/PHASE_CALLS 배열 길이(bounds) — prof.rs:185 인라인 | 4 |  |
| 14 | 1000000000 | 825 | 임계 | Duration → ns (secs*1e9 + nanos) — prof 계측 | 4 |  |
| 15 | -1 | 825 | 센티널 | Option<ProfTimer> None 니치(Instant.nanos = -1 · +0x10 i32) | 4 |  |

**`knobs` 조정점 2건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 재평가 생략 예산(틱) | lib.rs:806 · m14.ll:34778 | 10 | 올리면 소액션이 is_end 된 뒤에도 더 오래 같은 액션을 연장(extend_action)해 판단 빈도↓·반응↓ · 내리면 매번 update_small_action 재평가(비용↑·반응↑). DeathMatch 에선 무효(항상 재평가) | 4 | 기존 |
| 1 | v2 lapse 플랜 게이트 | lib.rs:569 · m14.ll:34438 | 1 | version>1 이면 lapse 여부와 무관하게 plan_system.update 호출(lapse 인자로 전달) · ≤1 이면 lapse 중엔 플랜 update 자체를 건너뜀 | 4 | 기존 |

<details><summary>`callees` 피호출자 28건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | can_skip_eval | game_ai::AgentVerHamster::can_skip_eval | in:game_ai | fn(&game_ai::AgentVerHamster, &game_core::PlayerState, &game_core::OperationData) -> bool | game-ai\src\lib.rs:796 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 1 | count_nearby_enemies | game_ai::AgentVerHamster::count_nearby_enemies | in:game_ai | fn(&game_core::Entity, &game_core::PlayerState, &game_core::OperationData) -> u16 | game-ai\src\lib.rs:830 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 2 | draw_debug | game_ai::PathFinder::draw_debug | pub | fn(&game_ai::PathFinder, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) | game-ai\src\path_finder.rs:622 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 3 | drop | <game_core::prof::ProfTimer as std::ops::Drop>::drop | pub | fn(&mut game_core::prof::ProfTimer) | game-core\src\simulation\prof.rs:184 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 4 | extend_action | game_ai::SmallActionPlay::extend_action | pub | fn(&mut game_ai::SmallActionPlay, usize) | game-ai\src\small_action.rs:444 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 5 | get_game_mode | game_core::AbstractGame::get_game_mode | pub | fn(&Self/#0) -> game_core::GameMode | game-core\src\simulation.rs:78 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 6 | get_game_mode | <game_core::Game as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::Game) -> game_core::GameMode | game-core\src\simulation\game.rs:1707 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 7 | get_game_mode | <game_core::SingleLaneGame as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::SingleLaneGame) -> game_core::GameMode | game-core\src\simulation\game.rs:4248 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 8 | is_act | game_core::Input::is_act | pub | fn(&game_core::Input) -> bool | game-core\src\simulation\state\player.rs:1446 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 9 | is_action_complete | game_ai::SmallActionPlay::is_action_complete | pub | fn(&game_ai::SmallActionPlay) -> bool | game-ai\src\small_action.rs:462 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 10 | is_end | game_ai::SmallActionPlay::is_end | pub | fn(&game_ai::SmallActionPlay, &mut rand::rngs::std::StdRng, usize, &game_core::PlayerState, &game_core::OperationData) -> bool | game-ai\src\small_action.rs:348 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 11 | is_premise_lost | game_ai::SmallActionPlay::is_premise_lost | pub | fn(&game_ai::SmallActionPlay, &game_core::OperationData) -> bool | game-ai\src\small_action.rs:376 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 12 | merge | game_core::DebugFrameData::merge | pub | fn(&mut game_core::DebugFrameData, &mut game_core::DebugFrameData) | game-core\src\simulation\game\frame.rs:38 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 13 | path_finder | game_ai::SmallActionPlay::path_finder | pub | fn(&game_ai::SmallActionPlay) -> std::option::Option<&game_ai::PathFinder> | game-ai\src\small_action.rs:330 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 14 | player_awareness_lapse | game_ai::player_awareness_lapse | pub | fn(&game_core::PlayerState, &game_core::OperationData) -> bool | game-ai\src\utils.rs:538 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 15 | start | game_core::prof::start | pub | fn(usize) -> std::option::Option<game_core::prof::ProfTimer> | game-core\src\simulation\prof.rs:175 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 16 | start | game_view::UIPhaseEffect::start | pub | fn(&mut game_view::UIPhaseEffect) | game-view\src\ui\match_ui\phase_effect.rs:30 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 17 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 18 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 19 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 20 | update | game_ai::plan_legacy::handler::LegacyPlanHandler::update | pub | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData, bool) | game-ai\src\plan_legacy\handler.rs:685 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 21 | update_event | game_ai::AgentVerHamster::update_event | in:game_ai | fn(&mut game_ai::AgentVerHamster, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) | game-ai\src\lib.rs:612 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 22 | update_plan | game_ai::EpicStanceData::update_plan | pub | fn(&mut game_ai::EpicStanceData, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &[std::option::Option<game_ai::EnemyRegionInfo>; 5_usize], &mut game_core::DebugFrameData) | game-ai\src\goal_data.rs:266 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 23 | update_plan | game_ai::AgentVerHamster::update_plan | in:game_ai | fn(&mut game_ai::AgentVerHamster, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bumpalo::collections::vec::Vec< game_core::TurnEvent> | game-ai\src\lib.rs:682 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 24 | update_plan | game_ai::SerpenStanceData::update_plan | pub | fn(&mut game_ai::SerpenStanceData, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &[std::option::Option<game_ai::EnemyRegionInfo>; 5_usize], &mut game_core::DebugFrameData) | game-ai\src\goal_data.rs:399 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 25 | update_plan_lapse | game_ai::AgentVerHamster::update_plan_lapse | in:game_ai | fn(&mut game_ai::AgentVerHamster, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, bool) -> bumpalo::collections::vec::Vec< game_core::TurnEvent> | game-ai\src\lib.rs:688 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 26 | update_small_action | game_ai::AgentVerHamster::update_small_action | in:game_ai | fn(&mut game_ai::AgentVerHamster, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) | game-ai\src\lib.rs:693 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 27 | update_state | game_ai::SmallActionPlay::update_state | pub | fn(&mut game_ai::SmallActionPlay, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) | game-ai\src\small_action.rs:291 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
</details>

⚠**미매칭 2개**: `bool`, `elapsed`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m14.ll:38938) · **형제 90개** (AgentVerHamster)

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

**`open` 8건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | update_event(lib.rs:612 · m14.ll:33589 internal fastcc · 인자 (self, rnd, player, cache, context)) 내부 — 이 배치 담당 아님. self 쓰기 표면은 그 함수 소관 | 4 |  |
| 1 | 미탐색 | LegacyPlanHandler::update(m13.ll:14467) 계약만: (&mut LegacyPlanHandler 6168B, version: usize, &mut StdRng, &PlayerState, &OperationData, &mut DebugFrameData(224B = self.debug), lapse: bool) -> () — 서브트리 90 은 r7~r12 명세 | 4 |  |
| 2 | 미탐색 | SmallActionPlay::update_state(m11.ll:41295): (&mut SmallActionPlay 184B, &mut StdRng, &PlayerState, &OperationData, &mut DebugFrameData 224B) -> () / is_premise_lost(m11.ll:41318): (&SmallActionPlay, &OperationData) -> bool / is_end(m11.ll:42503): (&SmallActionPlay, &mut StdRng, version: usize, &PlayerState, &OperationData) -> bool / path_finder(m11.ll:41146): (&SmallActionPlay) -> Option<&PathFinder>(null=None) — 계약만(r13/r14 잎) | 4 |  |
| 3 | 미탐색 | player_awareness_lapse(utils.rs:538 · m04.ll:49803): (&PlayerState, &OperationData) -> bool — 계약만. 내부 미독해(이 배치 범위 밖) | 4 |  |
| 4 | 미탐색 | PathFinder::draw_debug(m03.ll:136957): (&PathFinder 72B, &PlayerState, &OperationData, &mut DebugFrameData 224B) — 디버그 전용, context.debug 일 때만 도달 | 4 |  |
| 5 | 표기 불가 | L589~591 한 줄 안 순서: `version>1 && is_premise_lost` 가 `is_end` 앞인 것은 CFG(%160→%163→%168)로 확정. 단 소스가 `if A { … } else if B { … }` 두 문장인지 `if A \|\| B` 한 식인지는 표기 불가(외연 동일 — premise_lost 참이면 is_end 미호출은 양쪽 다 성립) | 4 |  |
| 6 | 미탐색 | can_skip_eval 의 L801 get_game_mode 반환 {i64,ptr} 태그 2 = DeathMatch 는 tcxdict --enum GameMode(Direct · 0 Moba/1 SingleLane/2 DeathMatch) 근거. 왜 DM 에서 항상 재평가하는지 의도는 주석 없음 | 3 |  |
| 7 | 미탐색 | self.events(+0x1d60)·stay_events 등 TurnEvent 실제 배출 경로는 이 함수가 아님(반환 Vec 은 항상 빈 값) — 어디서 소비되는지는 update_event/plan 쪽 소관 | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

