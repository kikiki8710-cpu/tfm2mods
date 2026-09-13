---

### `111` LegacyPlanHandler::update — AI 플랜 계층 최상위 틱 진입점 — 게임모드 분기(DeathMatch/SingleLane 전용 경로는 Moba 에서 사장) 뒤 Moba 본류: 룰스코프 정화·팀플랜/GoalData 갱신·채팅 수신/발신·플랜 전환(페이즈게이트 handle_interact_battle·패시브·전투 BattlePlan)·서브플랜 병합·계측 기록

| 항목 | 값 |
|---|---|
| id | `handler__LegacyPlanHandler_update` |
| 심볼 | `_RNvMNtNtCshdEBA0ozCnw_7game_ai11plan_legacy7handlerNtB2_17LegacyPlanHandler6update` |
| 소스 | `game-ai\src\plan_legacy\handler.rs:685` |
| IR | `m13.ll` 14467~28943행 |
| 경로·가시성 | `game_ai::plan_legacy::handler::LegacyPlanHandler::update` · **pub** |
| 계층 | 플랜 핸들러 |
| exe | `e4c5c0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r12` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData, bool)
```

<details><summary>인자 7개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | self | &mut LegacyPlanHandler(6168B) | IR %0 · `noalias align 8 dereferenceable(6168)` · readonly 없음 = 가변. 배치 A 살아있는 범위(L685·688·694)에서는 self 쓰기 0건(사장 689/695 내부 쓰기는 NA 미등재). 근거 m13.ll:14467 \| (배치 B) IR %0. 본 배치의 self 쓰기 전수 = writes \| (배치 C) IR %0 · noalias align 8 · readonly 없음 = &mut \| (배치 D) IR %0. 본 범위 쓰기 = writes 전수 | 4 |
| 1 | 2 | version | usize | IR %1 (i64). L685 에서 스택 스필 `store i64 %1, ptr %210`(m13.ll:14711) — 이후 배치 B~E 가 %210 을 23회 재로드(`load i64, ptr %210`)해 version 으로 씀(주석본 ;; version = i64 %N). ⚠reach.py 의 --version 접기는 br 직접 피연산자만 접으므로 `or` 로 합쳐진 version<2 비교(예 L947 인라인 1724 `%2111 = icmp ult %2078, 2`)는 안 접힘 — 해당 배치가 손으로 사장 처리할 것. 배치 A 살아있는 범위에선 version 분기 없음 \| (배치 B) IR %1(alloca %210 경유 재로드). 본 범위 게이트: 711 `version<2 → update_v2_egowave 생략` · force_plan_update:1724 `version<2 → r2_saved=None` \| (배치 C) IR %1 · 본 범위에선 %210(alloca 8B 복사본) 로드 후 콜리에 전달만 (L988·L1010·L1129·L1240·L1246·L1251/36) — 본 범위 자체 분기 없음 \| (배치 D) IR %1 → alloca %210(14710~14711). 1263 `version > 1` 게이트(reach.py 접힘 @3915: version=2 상수 접기로 항상 true) · 클로저 캡처(&version → is_ignored_well_enemy 인자) | 4 |
| 2 | 3 | rnd | &mut StdRng(320B) | IR %2 · `noalias align 16 dereferenceable(320)` · readonly 없음 = 가변. DI !23527 ref_mut$<rand::rngs::std::StdRng>. 배치 A 살아있는 범위에선 미사용 \| (배치 B) IR %2. 483 gen_range(0..1000) · 콜리(passive_plan/handle_chat/next_plan 등)에 전달 \| (배치 C) IR %2 · 콜리 전달만(v2_apply_assign_commit·BigPlan::update·is_end·BattlePlan::update·passive_plan) \| (배치 D) IR %2. 본 범위에선 직접 소비 없음 — sub_plan/passive_plan/check_kill_die_tick 에 그대로 전달 | 4 |
| 3 | 4 | player | &PlayerState(2528B) | IR %3 · `readonly captures(address, read_provenance)` = 불변. DI !23528. 배치 A 살아있는 범위에선 미사용(사장 689 의 update_deathmatch 가 +0x5f8 info.statistics.death 를 읽음 — NA) \| (배치 B) IR %3 readonly. info.position(0x9c0)·info.team(0x930)·info.parameter(0x180) \| (배치 C) IR %3 · readonly · info.team(0x930)·info.position@tag(0x9c0)·statistics.non_target_hit(0x678) 읽음 \| (배치 D) IR %3 readonly. info.position(0x9c0=%1799)·info.team(0x930=%2867)·statistics.death(0x5f8) | 4 |
| 4 | 5 | data | &OperationData(24B) | IR %4 · `readonly` = 불변. DI !23529. L688/L694 에서 data.cache(+0x0) → cache.game(&dyn AbstractGame, +0x0 data_ptr/+0x8 vtable_ptr) → vtable+0x40 get_game_mode() 호출에 사용 \| (배치 B) IR %4 readonly. cache(+0)=%211 → game 팻포인터(%212 data_ptr/%214 vtable_ptr) · context(+8)=%1639 · blackboard(+0x10)=%1713 \| (배치 C) IR %4 · cache(+0)·context(+8) 읽음 \| (배치 D) IR %4 readonly. +0 cache(=%211 AbstractGameWithCache) · +8 context(=%1639 GameContext). cache+0 = game.data_ptr(%212) · cache+8 = game.vtable_ptr(%214) | 4 |
| 5 | 6 | debug | &mut DebugFrameData(224B) | IR %5 · `noalias align 8 dereferenceable(224)` · readonly 없음 = 가변. DI !23530 ref_mut$<DebugFrameData>. 배치 A 살아있는 범위에선 미사용 \| (배치 B) IR %5. 본 범위에선 콜리에 전달만 \| (배치 C) IR %5 · L1125(626~627) infos(+0xa0) HashMap entry.or_insert().push(String) \| (배치 D) IR %5. 1509~1512 infos(+0xa0) HashMap 삽입 · 1569/1665 add_log | 4 |
| 6 | 7 | lapse | bool | IR %6 (i1 zeroext) · DI !23531 name "lapse" (m13.ll:92088) = 인지 공백(집중/멘탈 lapse, _docs game_ai.txt:398 「v3+에서만 참 · 새로 시작·새로 인지하는 것만 막고 진행 중 플랜의 유지·종료·전환은 항상 돈다」). 본체 사용처 4곳 전부 뒤 배치: `br i1 %6` at L728 · L947(인라인 1714·1727) · L960. 사장 689/695(update_deathmatch/update_single_lane)에는 전달되지 않음(%6 참조 0) \| (배치 B) IR %6 — DI 이름 `lapse`(m13.ll:92088 !23531). _docs: 「인지 공백(v3+에서만 참). 새로 시작·새로 인지만 막는다 — 콜 수신 처리·새 개전·팀 목표 발행/전환·배정 재선정」. 본 범위 게이트: 728(콜 수신 처리 생략) · force_plan_update:1714(handle_interact_battle 생략) · :1727(팀목표 갱신 롤백 모드) · 960(passive 재선정 생략→1005) \| (배치 C) IR %6 · DI 이름 `lapse`(!23531, arg 7) = 인지 공백(_docs game_ai.txt:398 — v3+에서만 참, 새 개전·목표 발행·배정 재선정만 막음). 본 범위 안 직접 분기 없음. ⚠배치 B 의 L960 `br i1 %6` 이 true 면 L960~1002 를 건너뛰고 본 범위 L1005(%2768) 로 진입한다(m13.ll:21510) \| (배치 D) IR %6 zeroext. DI 이름 `lapse`(m13.ll:14718 #dbg_value). 본 범위(1252~1672)엔 분기 없음 — 728/947/960 은 다른 배치 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
// handler.rs:685~695 (배치 A)
// L685: fn update(&mut self, version, rnd, player, data, debug, lapse) — 진입. version 을 스택 %210 에 스필(store, m13.ll:14711) 외 명령 없음(alloca 200여 개는 함수 전체 지역변수).
// L688: mode = data.cache.game.get_game_mode()  // vtable+0x40 간접호출, {tag,ptr}
//   if tag == 2 (GameMode::DeathMatch) → 블록 %220 … (L689)
// L689: self.update_deathmatch(version, rnd, player, data, debug)  // modes.rs 인라인 517줄(주석본 L220~L674)
//   // NA(gamemode≠Moba 전용: DeathMatch) — reach version=2·gamemode=Moba 접힘 @entry %219→0 · 블록 220~674 전부 사장(내 236블록 중 233 사장) · reads/writes/consts 미등재
// L694: else if tag == 1 (GameMode::SingleLane) → 블록 %680 … (L695)   // get_game_mode() 재호출 %676(m13.ll:16483)
// L695: self.update_single_lane(version, rnd, player, data, debug)  // modes.rs 인라인 1,060줄(주석본 L680~L1626)
//   // NA(gamemode≠Moba 전용: SingleLane) — reach 접힘 @675 %678→0 · 블록 680~1626 전부 사장 · reads/writes/consts 미등재
// else → 블록 %1627 = Moba 본류 → 배치 B(줄 698, `_t_head` ProfTimer 시작)
//
// self writes(살아있는 범위): 0건. HEAP 재료(살아있는 범위): 0건. rnd/debug 부작용: 0건.
// 함수 공용 EH: 모든 invoke 의 unwind → 블록 %1640 cleanuppad(m13.ll 주석본 L3939) → phi %1641 참이면 %5376 에서 drop_glue<Option<ProfTimer>>(%209 = L698 `_t_head`) 후 %5375 `cleanupret unwind to caller`(m13.ll:28938, L685 귀속) — 패닉 전파 경로에서 self 쓰기 없음.
// 함수 공용 스필: %210 = version(i64) 스택 사본 · 배치 B~E 의 `load i64, ptr %210` 23곳이 전부 version.

// handler.rs:696~976 (배치 B)
// 진입: 배치 A 블록 %675(695) → %1627. 예외 cleanup 블록(1640·2039·2216·2270 등)은 언와인딩 전용이라 생략.
// 표기: game = data.cache.game(&dyn AbstractGame, vtable+0x28=tick, +0x130=kill_logs) · ctx = data.context · bb = data.blackboard[player.team]

698 _t_head = ProfTimer::start(77)                       // prof::ENABLED 원자 로드; 계측 전용(판정 무관, 이하 ProfTimer 전부 동일)
699 self.sanitize_rule_scope(data.cache, ctx)           // 콜리 계약만(&mut self, &cache, &ctx)
702 { _t = ProfTimer::start(57); 703 self.team_plan.update(version, rnd, player, data, debug); 704 drop(_t) }
705 self.chats.extend(self.team_plan.chats.drain(..))    // ★HEAP chats@0x7c8 grow / team_plan.chats@0x1b8 비움
707 { _t = ProfTimer::start(58); 708 GoalData::update(&mut self.data, version, rnd, player, data, debug); 709 drop(_t) }

711 if version >= 2 { self.update_v2_egowave(player, data.blackboard, ctx, rnd) }   // 인라인 handler.rs:474~488
  474 (version<2 → return)
  477 self.v2_armed = true; 478 self.v3_armed = true; 479 self.v3_epicops_armed = true
  480 match egowave_check(player, bb, ctx) {                // 인라인 handler.rs:2269(정의)~2285 (자유함수 handler::egowave_check)
    2270 if player.info.position == Jungle → None
    2273 roaming = player.info.parameter.roaming_ratio() as i32; 2274 ego = ego_ratio() as i32
    2275 my_line = match position { Top→Top(0), Mid→Mid(1), _→Bottom(2) }
    2280 my_line = rule_scope::fallback_line(ctx, my_line, default=Bottom)   // 인라인 rule_scope.rs:29~34: line_exists(ctx,line)(tutorial.spawn_line_minion(line)) 이면 그대로, 아니면 valid_lines(tutorial).last() (First/Bottom→[Bottom], TopSolo→[Top], MidSolo→[Mid], MidBottom→[Mid,Bottom]), JungleOnly→default Bottom
    2281 ms = &bb[player.team].{top|mid|bottom}_minion_state (my_line 별 +0/+0x28/+0x50; team<2 bounds check)
    2282 if !(ms.from_mid < -2000 && ms.minion_count < -2) → None
    2283 wave_concern = 500 - roaming; 2284 if wave_concern < 1 → None
    2285 Some((my_line, follow_own = wave_concern*ego/500))
  }
  None → 488 self.v2_egowave = 0
  Some((line, follow_own)) → 482 if self.v2_egowave != 0 && self.v2_egowave_line == line { /*유지*/ }
                             else { 483 self.v2_egowave = if rnd.gen_range(0..1000) < follow_own {2} else {1}; 484 self.v2_egowave_line = line }

714 self.sanitize_rule_scope(data.cache, ctx)
716 if !self.counter_jungle_route_init && player.info.position == Jungle && player_count(ctx)==full /*인라인: !(tutorial∈1..=7)*/ {
  717 self.counter_jungle_route_init = true
  718 strategy = player.strategy(rnd, game)               // PlayerState::strategy(sret 24B, &player, &mut rnd, game 팻포인터)
  719 if strategy.early_jungle == CounterJungle(2) {
    720 mf_note_swap(28, game.tick())                    // 인라인 handler.rs:410: mf_swap = (28, tick)
    722 p = PassiveJunglePlan::new_counter_jungle(rnd, player.info.team, strategy.focused)   // sret 104B
    721 self.plan = BigPlan::PassiveJungle(p)             // ★HEAP: drop_glue(&self.plan) → 태그 7 store + 104B memcpy@+0x5f0 (723)
  }
}

728 if !lapse && (!self.received_chats.is_empty() || !self.misunderstood_received_chats.is_empty()) {
  729 handled_chats: Vec<(usize,Position,Chat)> = Vec::new()   // 로컬 힙(741 에서 drop)
  730 for (tick, from, chat) in self.received_chats.iter() { 731 if *tick <= game.tick() { 732 handled_chats.push((tick, from, chat.clone())) } }
  735 self.received_chats.retain(|(tick,..)| *tick > game.tick())      // aux m06.ll:2048
  736 for (tick, from, c) in handled_chats {
    737 if rule_scope::chat_allowed(ctx, &c) {
      738 misunderstood = self.take_misunderstood_received_chat(tick, from, &c)     // fastcc, &mut self
      739 self.handle_chat(version, rnd, player, data, from, &c, misunderstood, debug)   // 자식 명세(계약만)
    }
  }
  742 self.misunderstood_received_chats.retain(|(tick,..)| *tick > game.tick())   // aux m06.ll:2184
}
745 if !self.chats_wait.is_empty() {
  746 for (tick, chat) in self.chats_wait.iter() { 747 if *tick <= game.tick() { 748 if chat_allowed(ctx, chat) { 749 self.chats.push(chat.clone()) /*★HEAP*/ } } }
  754 self.chats_wait.retain(|(tick,_)| *tick > game.tick())   // aux m06.ll:2727
}
758 if let Some(chats) = self.plan.chats() /*인라인 types.rs:185~189, 태그별 Vec 위치*/ { 759 self.chats.extend(chats.drain(..)) }   // ★HEAP
761 self.sanitize_rule_scope(data.cache, ctx)
763 drop(_t_head)
764 _t_np = ProfTimer::start(59)
765 next: Option<BigPlan> = self.plan.next_plan(version, rnd, player, data, &self.data, &self.positioning_score, &mut self.team_plan, debug)   // sret 384B
767 drop(_t_np); 768 _t_tail = ProfTimer::start(78)   // (_t_tail 의 drop 은 배치 C/D)

769 if let Some(plan) = next {                                // next.tag == -1 → None → 947 로
  770 if rule_scope::plan_allowed(ctx, &plan) {               // 인라인 rule_scope.rs:101 → goal_allowed(ctx, plan.goal()) :91~95
        // goal.tag: Line(0)→line_exists(ctx, goal.line) · Jungle(1)→tutorial∈{None,JungleOnly,Total} · Epic(2)→morgard_exists=tutorial∈{None,Line,Total} · Serpen(3)→tutorial∈{None,MidBottom,Line,Total} · Nexus/Battle/Recall(4·5·6)→true
    772 if plan.tag==LineGanker(10) && plan.LineGanker.phase == WaitResponse(6) {
      775 can_transition = game.tick().saturating_sub(self.gank_cancel_tick) > ctx.setting.tick_per_second*10
      783 if !can_transition → 945 drop(plan) → 947   // 전이 거부(갱크 취소 후 10초 쿨다운)
    }
    784 if let Some(chats) = self.plan.chats() { 785 self.chats.extend(chats.drain(..)) }   // ★HEAP (전이 직전 현재 플랜의 chats 회수)
    789 if player.info.position == Jungle {                    // %1800 거짓일 때만; 아니면 938 로
      789 if self.plan.tag == PassiveJungle(7) { 790 self.last_jungle_lead_action_tick = game.tick() }
      797 if self.plan.tag == LineGanker(10) {
        799 is_transition_to_battle = plan.tag == Battle(9)
        801 if is_transition_to_battle { 803 self.active_gank_line = Some(self.plan.LineGanker.line) }
        else {
          806 self.active_gank_line = None
          807 if let Some(last) = self.gank_periods.last_mut() { 808 if last.1 == 0 {
            809 start_tick = last.0; 810 end_tick = game.tick(); 811 last.1 = end_tick
            813 if ctx.trace_level != Off {
              815 duration = end_tick.saturating_sub(start_tick); 816 line = self.plan.LineGanker.line
              818 kills  = game.kill_logs().iter().filter(|k| start<=k.tick<=end && k.killer_team==player.team && (k.killer_position==Jungle || k.assist.contains(&Jungle))).count()   // aux m13.ll:60162
              824 deaths = game.kill_logs().iter().filter(|k| start<=k.tick<=end && k.killer_team!=player.team && k.killed_position==Jungle).count()   // aux m13.ll:60317
              830 success = kills != 0
              833 self.pending_trace_events.push(PendingTraceEvent{ event: GankResult{kills,deaths,duration,line,success}, tick: end_tick })   // ★HEAP
            }
          } }
        }
      }
      850 if self.plan.tag == Battle(9) {
        851 if plan.tag == Battle(9) → 938   // Battle→Battle 은 장부 생략
        853 if self.team_plan.objective.tag == ComebackPick(11) {
          854 pick_start = self.team_plan.comeback_pick_start_tick
          855 got_kill = game.kill_logs().iter().rev().any(|k| k.killer_team==player.team && k.tick >= pick_start)   // 인라인 closure$5(:856)
          if got_kill { 859 self.team_plan.comeback_pick_success_count += 1; 860 if let Some(last)=self.team_plan.comeback_pick_outcomes.last_mut() { if last.1==0 { last.1 = 1 } } }
          else       { 863 if let Some(last)=...last_mut() { if last.1==0 { last.1 = 6 } } }
        }
        866 if matches!(self.team_plan.objective.tag, ComebackPick(11)|Gank(8)|Dive(9)) { 867 self.team_plan.mf_note_obj_clear(41, game.tick()) /*mf_obj_clear=(41,tick)*/; 868 self.team_plan.objective = None }
        870 if let Some(last) = self.gank_periods.last_mut() { 871 if last.1 == 0 {
          872 start_tick = last.0; 873 end_tick = game.tick(); 874 last.1 = end_tick
          876 if ctx.trace_level != Off {
            878 duration = end_tick.saturating_sub(start_tick)
            879 line = rule_scope::fallback_line(ctx, self.active_gank_line.unwrap_or(Mid))   // 아웃오브라인 call(2인자판, default 없음)
            882~899 kills/deaths/success/GankResult push — 818~833 과 동일(aux m13.ll:60462/60617 본문 동일)
          }
        } }
        909 self.active_gank_line = None
      }
      916 if plan.tag == LineGanker(10) {
        918 last_start = self.gank_periods.last().map(|(s,_)| *s).unwrap_or(0)
        919 if game.tick().saturating_sub(last_start) > tps*10 {
          920 self.gank_attempt_count += 1
          922 self.gank_periods.push((game.tick(), 0))   // ★HEAP
          925 actual_score = evaluate_gank_opportunity_with_score(rnd, player, data, plan.LineGanker.line, 0).1   // sret 12B 의 +8 i32
          927 self.gank_score_attempts.push((game.tick(), actual_score, 0))   // ★HEAP
        }
        931 if self.team_plan.objective.is_none() { 932 self.team_plan.objective = Some(Gank{line: plan.LineGanker.line}); 933 self.team_plan.gank_start_tick = game.tick() }
      }
    }
    938 mf_note_swap(20, game.tick()); 939 self.plan = plan   // ★HEAP drop_glue(&self.plan) 후 memcpy 384
  } else {   // plan_allowed 거짓
    942 mf_note_swap(26, game.tick()); 943 self.plan = BigPlan::ForcePassive(태그 2)   // ★HEAP drop_glue 후 store
    945 drop(plan)
  }
}

947 self.force_plan_update(version, rnd, player, data, debug, lapse)   // 인라인 handler.rs:1713~1766
  1713 _t = ProfTimer::start(79)
  1714 if !lapse { 1715 self.handle_interact_battle(version, rnd, player, data, debug) }   // 자식 명세(전환엔진·페이즈게이트 소재 — 본 범위엔 그 식 없음). 1717 drop(_t)
  1719 was_battle = self.plan.tag == Battle(9); entry_src = self.plan.Battle.entry_src(0x6f7, 선읽기)
  1724 r2_saved: Option<BigPlan> = if version < 2 || !was_battle { None } else { Some(self.plan.clone()) }
  1726 _t = ProfTimer::start(80)
  1727 if lapse {
    1737 before_obj = self.team_plan.objective (3B); 1738 before_plan = self.plan.clone(); 1739 before_chats = self.team_plan.chats.len()
    1740 self.team_plan.update_objective(version, rnd, player, data, self, &self.plan, debug)
    1741 cleared = before_obj.is_some() && self.team_plan.objective.is_none()
    if cleared { 1747 drop(before_plan) }   // 목표 해제(종료)만 통과
    else { 1743 self.team_plan.objective = before_obj; 1744 self.plan = before_plan /*★HEAP*/; 1745 self.team_plan.chats.truncate(before_chats) }   // 발행/전환 롤백
  } else { 1748 self.team_plan.update_objective(version, rnd, player, data, self, &self.plan, debug) }
  1750 drop(_t)
  1751 if let Some(saved) = r2_saved { 1752 if self.plan.tag == Battle(9) { 1755 drop(saved) } else { 1753 self.plan = saved /*★HEAP: v2+ 에서 update_objective 가 Battle 을 못 걷어내게 복원*/ } }
  1756 if was_battle && self.plan.tag != Battle(9) {   // v2+ 에선 위 복원 때문에 도달 불가 → 사실상 version<2 전용
    1758 self.ff_battle_exit = (22, entry_src, game.tick(), 0, 255); 1759 self.ff_battle_exit_latch = [0xff;8]
  }
  1763 goal = self.plan.goal()   // sret %31 24B — 소비처는 배치 C(948 이후)

948 if self.plan.tag == ForcePassive(2) {
  949 (mf_p, mf_src) = self.passive_plan(version, rnd, player, data, debug)   // sret 392B = BigPlan + u8
  950 self.v2_apply_assign_commit(version, rnd, player, data, &mf_p, &mf_src, debug)   // 자식 명세(계약만)
  952 if self.mf_swap.1 != game.tick() { 953 mf_note_swap(mf_src, game.tick()) }
  955 self.plan = mf_p   // ★HEAP
}
960 if !lapse && self.plan.is_passive() /*인라인 types.rs:254~257: PassiveLine(3)|SinglePlanLine(4) → true · PassiveJungle(7) → !is_counter_jungle ⇔ p.team == p.player_team · 그 외 false*/ {
  961 (new_plan, mf_src) = self.passive_plan(version, rnd, player, data, debug)
  969 keep_journey = false; if self.v3_armed {
    970 obj_key = self.team_plan.objective
    976 if self.v3_dest_obj == obj_key /*None==None 참, 둘 다 Some 이면 MainObjective::eq*/ && let Some((dx,dy)) = self.v3_dest { → 배치 C(줄 977, 블록 %2683) } else { → 배치 C(줄 982, 블록 %2725) }
  } else { → 배치 C(줄 988, 블록 %2671: v2_apply_assign_commit) }
} else { → 배치 C(줄 1005, 블록 %2768) }
// 다른 배치 경계: 2610(lapse 참) → %2768(1005) · 2639 switch default/2644 거짓 → %2768(1005)

// handler.rs:977~1251 (배치 C)
// 진입 ①: 배치B L976 `v3_armed && v3_dest_obj == team_plan.objective && v3_dest.is_some()` → L977(%2683). ②: L976 거짓/`arrived` 경로 → L982(%2725). ③: L969 v3_armed 거짓 → L988(%2671, 배치B 21622). ④: 배치B L948(plan!=SinglePlanLine)·L960(`lapse`==true 또는 !passive) → L1005(%2768) 직행(새 플랜 계산·커밋 전부 생략).
// vtable: game.tick = vtable+0x28 · get_entity_by_id = +0x1f0 · kill_logs = +0x130 (divtable AbstractGame). champ = cache.player_champion[player.team][player.position] (0x1e0+team*40+pos*8, team<2 bounds check).

// ── L977~1002: v3 목적지 게이트 + 새 플랜 커밋 (new=%172 384B, src=%171 u8: 배치B L961 passive_plan 결과)
L977: let (dx,dy) = self.v3_dest.unwrap()  // 값은 B 에서 로드(0x550/0x558)
L978: arrived = champ.is_some_and(|x| (|x.x-dx|² + |x.y-dy|²) < 64000²+1)   // Entity 0x660/0x668 · 4096000001
L979: if !arrived && closure$9(&self.plan) && closure$9(&new) {  // closure$9(L974, 배치B) = idx ∈ {PassiveLine, SinglePlanLine, PassiveJungle}
L985:   drop(new); → L1002 → L1005   // 목적지로 이동 중 + 구/신 모두 소극 플랜 = 교체 생략(구 플랜 유지). ⚠ champ 가 None 이면 arrived=false 로 취급(21694)
}
L982: self.v3_dest = self.v3_plan_dest(player.team, context, &new)   // sret 24 · 내부 미탐색
L983: self.v3_dest_obj = obj   // %169 (B L970 에서 team_plan.objective 복사)
L988: self.v2_apply_assign_commit(version, rnd, player, data, &mut new, &src, debug)   // 진입③ 합류점 %2671
L991: if self.plan is PassiveLine(3) && new is PassiveLine(3) {
L992:   if old.line(0x706) == new.line(+0x11e) {
L993:     new.0.v46_carry_over(&mut old.0)   // PassiveLinePlan::v46_carry_over(&mut new, &mut old)
}}
L997: if self.mf_swap.1(0x1618) != game.tick() {
L998:   self.mf_note_swap(src, game.tick())   // mf_swap = (src, tick) (handler.rs:410 인라인)
}
L1000: self.plan = new   // HEAP: 구 플랜 drop_glue 후 memcpy 384 (언와인드 시 원복 memcpy)
L1002: (스코프 끝 — 언와인드 경로 %2680: new drop)

// ── L1005~1013: 플랜 update (진입④ 합류점 %2768)
L1005: phase = match self.plan idx { Battle(7)→68, PassiveLine(1)→69, _→70 }; timer = ProfTimer::start(phase)  // prof::ENABLED 원자 로드 0 이면 None
L1010: self.plan.update(version, rnd, player, data, &self.data(0x0), &mut self.team_plan(0xf8), &self.positioning_score(0x990), debug)   // BigPlan::update m02.ll:6887 — 자식 명세 없음, 내부 미탐색
L1013: drop(timer)  // PHASE_NANOS[phase] += elapsed, PHASE_CALLS[phase] += 1 (원자)

// ── L1015~1016: 플랜 채팅 회수
L1015: if let Some(chats) = self.plan.chats() {   // types.rs:185~189: PassiveLine→+0x18(0x608) · SinglePlanLine/PassiveJungle/LineGanker/LineGankCover→+0x0(0x5f0) · SinglePlanBattle/Battle→+0x68(0x658) · DeathMatchBattle→+0xf8(0x6e0) · 그 외(ForcePassive·ActiveRecall·Epic*/Serpen*/AttackNexus/DefenseNexus) None
L1016:   self.chats(0x7c8).extend(chats.drain(..))   // HEAP grow
}

// ── L1023~1053: v48 회피 claim 창 계측
L1023: if let SubPlan::Battle(sb) = &self.sub_plan(0x768 tag 7) && sb.v48_dodge_claim(0x79b) {
L1025:   self.v48_claim_until_tick = max(self.v48_claim_until_tick, sb.v48_claim_hold_until(0x790))
}
L1029: now = game.tick()
L1030: if now > self.v48_claim_until_tick {
L1033:   nth = player.info.statistics.non_target_hit(0x678)
L1034:   if nth > self.v48_last_non_target_hit { L1035: d = nth-last; L1050: self.v48_other_hits += d }
} else {
L1031:   self.v48_claim_window_ticks += 1
L1033:   nth = …
L1034:   if nth > last { L1035: d = nth-last; L1037: self.v48_claim_window_hits += d
L1039:     if let Some(c) = champ { L1040: if c.ty is Champion(13) {
L1041:       match c.action_state@tag(0x70) { Move(2) → L1044 v48_claim_hit_moving += d ; Attack(3)|Skill(4)|Skill2(5)|Ult(6) → L1043 v48_claim_hit_locked += d ; _(Idle/Return) → L1045 v48_claim_hit_idle += d }
}}}}
L1053: self.v48_last_non_target_hit = nth   // 양 분기 공통(phi)

// ── L1059~1092: v48 시전 분포 계측 (자기 논타겟 스킬 시전 시 가장 가까운 적의 상태)
L1059: if let Some(c) = champ { L1060: if c.ty is Champion {
L1061:   cur = match action_state { Skill(4)→1, Skill2(5)→2, Ult(6)→3, _→0 }   // 0 이면 L1092 로
L1067:   if cur != 0 && cur != self.v48_last_cast_state(0x1814) {
L1069~1071: effect = match cur { 1 → c.skill_effect.as_ref()(tag 0x4f8) ; 2 → c.skill2_effect().as_ref() [level(0x5c8)>2 ? &skill2_effect(0x500) : &NONE] ; 3 → c.ult_effect().as_ref() [level>4 ? &ult_effect(0x538) : &NONE] }
L1074:   if let Some(e) = effect && e.casting(+0x30).is_nontarget() [casting ∈ {Position 1, Direction 2}] {
L1075:     nearest = cache.iter_champions(1-team).filter(|x| x.is_visible_from(c)).min_by_key(|x| distance_sq(x, c))   // aux m13.ll:5116 · is_visible_from(entity.rs:1482): c.team 이 Player 아니면 true, 아니면 x.visible_state[c.team]@tag(0x38+16*team)==0
L1077:     if let Some(t) = nearest {
L1078:       windup = e.start_timing(+0x20)
L1079:       if t.block_move() { self.v48_cast_cc += 1 }
L1081:       else if !(t.remain_action_time() < windup) { self.v48_cast_locked += 1 }   // 극성: remain < windup 이면 아래로, 아니면 locked (22332~22333)
L1083:       else { d = t.distance(c); r = max(e.range(c), 1) [range + (level-1)*growth + c.stat_buff_cached.range(0x438)]; if d*2 < r { v48_cast_free_near += 1 } else { v48_cast_free_far += 1 } }
}}}
L1092:   self.v48_last_cast_state = cur   // Champion 이면 항상(cur 가 0 이어도) 저장
}}

// ── L1098~1115: PassiveLine v46 계측 접기 (플랜 → 핸들러 누적)
L1098: if let BigPlan::PassiveLine(p) = &mut self.plan {
L1100:   self.v46_lane_recall_trigger_ticks(0x870).append(&mut p.v46_pending.trigger_ticks(0x650))   // HEAP: reserve+memcpy, 소스 len=0
L1101~1114: self.v46_lane_recall_{danger_ticks,veto_wave,veto_crash,veto_heal,stage2_saves,commit_clears,wave_enemy_half,wave_my_half} / v46_lane_flee_{triggers,hold_ticks,hold_acute_ticks,hold_refuge_ticks,hold_cover_ticks,nohit_episodes} (0x14d0~0x1538) += mem::take(&mut p.v46_pending.<동명>) (0x680~0x6e8 → 0)
L1115:   self.v46_lane_flee_episodes(0x8a0).append(&mut p.v46_pending.flee_episodes(0x668))
}
// ── L1118~1119
L1118: else if let BigPlan::Battle(b) = &mut self.plan { L1119: self.v54_reentry_ticks(0x900).append(&mut b.v54_reentry_ticks(0x670)) }
// ── L1121
L1121: self.sanitize_rule_scope(cache, context)   // internal fastcc m13.ll:11163~11383 · 내부 미탐색

// ── L1124~1126: 라인 백파이트 지원 진입 (enter_line_backfight_support, handler.rs:599~633 인라인)
L1124: timer = ProfTimer::start(76)
L1125: {
  599: if let BigPlan::PassiveLine(p) = &self.plan {
  600:   line = p.line(0x706)
  603:   if let Some((ally_id, focus_id)) = utils::line_backfight_support_focus(version, player, data, line) {   // sret 24 · m04.ll:53979 미탐색
  606:     if let Some(focus) = game.get_entity_by_id(focus_id) {
  609:       if !fight_model::is_ignored_battle_enemy(version, player, data, focus, false) {
  613:         let mut battle = BattlePlan::new(version, &Some(focus_id), data, player)
  614:         battle.entry_src(+0x107) = 6
  615:         battle.support_target(+0x0) = Some(focus_id)
  616:         battle.set_main_objective(self.team_plan.objective(0x517))  // → +0xff
  617:         battle.update(version, rnd, player, data, &self.positioning_score, &mut self.team_plan, debug)
  618:         battle.chats.clear()   // +0x78 len=0
  620:         if battle.sub_goal(+0x58)@tag ∈ {RunAway 4, End 7} { drop(battle); → 633 }   // 진입 취소
  624:         else { if context.debug(0x3b) { 625: if let Some(c) = champ { 626~627: debug.infos(+0xa0).entry(c.id).or_insert(vec![]).push(format!("!v26 line backfight support: line {:?}, ally {}, focus {}", line, ally_id, focus_id)) } }
  631:           self.plan = BigPlan::Battle(battle)   // HEAP: 구 플랜 drop · tag 9 + memcpy 280 → 0x5f0
  }}}}}
}
L1126: drop(timer)

// ── L1128~1131: 종료 판정
L1128: timer = ProfTimer::start(74)
L1129: plan_ended = !self.plan.is_passive() && self.plan.is_end(version, rnd, player, data, &self.data, &mut self.team_plan, debug)
       // is_passive(types.rs:254~257): PassiveLine(idx1)·SinglePlanLine(idx2) → true ; PassiveJungle(idx5) → !is_counter_jungle = (team(0x638) == player_team(0x640)) ; 그 외 false
L1130: drop(timer)
L1131: if !plan_ended → L1251

// ── L1133~1173: (plan_ended) 정글러 갱크 결과 트레이스
L1133: if player.position(0x9c0) == Jungle(1) && self.plan is LineGanker(10) {
L1135:   if let Some(last) = self.gank_periods(0x810).last_mut() {   // (start,end) 16B, len 0x820
L1136:     if last.1 == 0 {
L1137:       start = last.0 ; L1138: now = game.tick() ; L1139: last.1 = now   // 힙 원소 in-place
L1141:       if context.trace_level(0x39) != Off(0) {
L1143:         duration = now.saturating_sub(start)
L1144:         line = gank_plan.line(0x618)
L1147~1151:    kills = game.kill_logs().iter().filter(|k| start <= k.tick(+0x18) <= now && k.killer_team(+0x20) == player.team && (k.killer_position(+0x28) == Jungle || k.assist(+0x0).contains(&Jungle))).count()   // aux m13.ll:60762
L1153~1157:    deaths = kill_logs().iter().filter(|k| start <= k.tick <= now && k.killer_team != player.team && k.killed_position(+0x2c) == Jungle).count()   // aux m13.ll:60917
L1159:         success = kills != 0
L1162:         self.pending_trace_events(0x858).push(PendingTraceEvent{ event: GankResult{kills, deaths, duration, line, success}, tick: now })   // HEAP
}}}}
// ── L1178~1192
L1178: if self.plan.is_cancel() { L1179: if self.plan is LineGanker(10) { L1181: self.gank_cancel_tick(0x1470) = game.tick() } }
L1189: if self.plan tag ∈ {Battle 9, LineGanker 10} {
L1190:   if self.team_plan.objective(0x517) tag ∈ {Gank 8, Dive 9} {
L1191:     self.team_plan.mf_note_obj_clear(42, game.tick())   // (0x308,0x310) = (42, tick)
L1192:     self.team_plan.objective = None (0xff)
}}
// ── L1200~1237: Battle 이탈 계측
L1200: if let BigPlan::Battle(b) = &self.plan {
L1204:   if (b.dive_abandoned(0x6ea) || b.with_dive(0x6e6)) && !b.dive_entered(0x6e7) { L1205: self.last_dive_abandon_tick(0x1480) = tick }   // 극성: !(…) || dive_entered 이면 건너뜀 (24115~24121)
L1211:   if b.main_goal(0x630)@tag == Response(2) && b.exit_src(0x6f8) ∈ 11..=14 { L1212: self.last_response_bail_tick(0x1488) = tick }
L1215:   if b.exit_src == 11 { L1216: self.team_plan.last_resolver_bail_tick(0x320) = tick }
L1225:   if b.exit_src ∈ {11,12} {
L1226:     if b.main_goal@tag < 2 (TryKill 0 | Support 1) { t = main_goal.0(0x638); L1227: self.last_lost_fight(0x1490) = (t, tick) }
L1228:     else if let Some(focus) = b.focus() [sub_goal@tag(0x648) ∈ {0,1,2,3,5,6} → 0x650] { L1231: self.last_lost_fight = (focus, tick) }
  }
L1235:   self.ff_battle_exit(0x15f8) = (b.exit_src, b.entry_src(0x6f7), tick, b.exit_sub(0x701), b.ff_wave_obs_open(0x6f9))
L1236~1237: self.ff_battle_exit_latch(0x1608..) = (b.ff_exit1_cls(0x700), ff_wave_open_hp(0x6fa), ff_wave_open_pct(0x6fb), ff_wave_fire_hp(0x6fc), ff_wave_fire_pct(0x6fd), ff_wave_open_danger(0x6fe), ff_wave_fire_danger(0x6ff), dive_abort_src(0x6f3))
}
// ── L1240~1249: 종료 플랜 교체
L1240: let (new, src) = self.passive_plan(version, rnd, player, data, debug)   // sret 392 = BigPlan 384 + u8
L1241: self.mf_ret25_src(0x1658)[min(src,31)] += 1
L1246: self.v2_apply_assign_commit(version, rnd, player, data, &mut new, &src, debug)
L1247: self.mf_note_swap(25, game.tick())   // mf_swap = (25, tick) — L997 의 tick 동일성 검사 없이 무조건
L1248: self.plan = new   // HEAP: 구 플랜 drop · 언와인드 시 원복
L1249: (언와인드 %3873: new drop)

// ── L1251: self.v50_track_dive_episode(version, player, data)  (dive_episode.rs:36~118 인라인 · 346 IR줄)
  36: tick = game.tick()
  37~59: snap = closure$0() → Ok(active) | Err(reason):
    38: champ = cache.player_champion[team][pos] else Err(8)
    39: b = self.plan as Battle(9) else Err(1)
    40: b.with_dive(0x6e6) else Err(2)
    41: tower_ty = b.dive_tower(0x6ee) (Some) else Err(3)
    42: focus = b.focus() [sub_goal 0x648 tag ∈ {0,1,2,3,5,6} → 0x650] else Err(4)   // ⚠ tag 기본(default) 케이스는 %1736(L711, 배치A 의 unreachable/패닉 블록)로 점프
    43: target = game.get_entity_by_id(focus) else Err(5)
    44: target.team(0x0 tag 0 Player, 0x8 == 1-team) 아니면 Err(6)
    45: tp = cache.player_by_champion_id(target.id(0x5c0)) else Err(6)
    46: target_pos = tp.info.position@tag(0x9c0)
    47~49: tower = cache.iter_towers_without_nexus(1-team).filter(|t| t.tower_type == tower_ty).min_by_key(closure s_0)  else Err(7)   // 필터·키 본체 = dive_episode 소유 별도 define(m13.ll:4340~4531 `v50_track_dive_episode` 클로저) — 본 배치 범위 밖·미탐색
    50: atk = tower.attack_effect.as_ref()(tag 0x4c0 != -1) else Err(7)
    51: reach = atk.range(tower) [range(0x4a0) + (tower.level(0x5c8)-1)*growth(0x4a8) + tower.stat_buff_cached.range(0x438)] + champ.radius() + tower.radius()   // radius(entity.rs:1511~1515): mult=radius_mult(0x470); mult==0 ? radius(0x680) : radius*(mult+100)/100
    52: in_range = tower.distance(champ) <= reach
    53~56: if tower.ty is Tower(2) && let Some((_, id)) = tower.info.nearest_enemy(0x88 tag, 0x98 id) { tgt = get_entity_by_id(id); holder = (id == champ.id); tower_on_champ = tgt.is_some_and(|e| e.ty is Champion(13)); team_holder = tower_on_champ && e.team == Player(team) } else { holder=false, team_holder=false, tower_on_champ=false }
    58: tgt_hp_pct = min(target.hp(0x670)*100 / max(target.stat_cached.hp(0x628),1), 255) as u8
    59: Ok(active{ hp: champ.hp, catch_break: b.dive_catch_break_ticks(0x688), target_pos, in_range, holder, team_holder, tower_on_champ, tower_ty, snap: (race_adv: b.dive_last_race_adv(0x6e0), model 0x6f4, na 0x6f5, ne 0x6f6, tgt_hp_pct) })
  Ok(active):
    65: if let Some(live) = &self.v50_dive_ep_live(0x570) && live.tower(0x5e1) != active.tower_ty { 66: self.v50_fold_dive_episode(false, 0) }   // tps(0x12f8) 로드는 dead
    68: if let Some(live) = self.v50_dive_ep_live.as_mut() {
      70: live.gap_ticks = 0 ; 71: live.max_catch_break = max(catch_break, live.max_catch_break) ; 72: live.uncatch_total += (catch_break != 0)
      73: if live.start_model == 0 && snap.model != 0 { 75: live.start_model = model; live.start_race_adv = race_adv; 76: live.start_na = na; live.start_ne = ne }
      78: live.last_tick = tick ; 79: live.ep_ticks += 1
      80: if in_range { live.in_range_ticks += 1 }
      81: if team_holder { live.team_holder_ticks += 1 }
      82: if in_range && !tower_on_champ { live.minion_cover_ticks += 1 }   // 23755~23777 분기 재구성
      83: if holder { 84: live.holder_ticks += 1 ; 85: if let Some(ph) = live.prev_holder_hp { live.soaked_hp += ph.saturating_sub(hp) } ; 86: live.prev_holder_hp = Some(hp) } else { 88: live.prev_holder_hp = None }
    } else { 91~92: self.v50_dive_ep_live = Some(V50DiveEpLive{…}) (writes 참조) }
  Err(reason):
    108: if let Some(live) = self.v50_dive_ep_live.as_mut() {
      110: if reason ∈ {4,6} && live.gap_ticks < setting.tick_per_second(0x12f8) { 111: live.gap_ticks += 1; 112: live.ep_ticks += 1; 113: live.last_tick = tick }
      else { 115: dive_abandoned = self.plan is Battle && b.dive_abandoned(0x6ea) ; 116: abort_src = Battle ? b.dive_abort_src(0x6f3) : 0 ; 117: self.v50_dive_ep_abort_src(0x1811) = abort_src ; 118: self.v50_fold_dive_episode(dive_abandoned, reason) }
    }
  → 모든 경로 %3897 = L1253 (배치 D)

// ── 다른 배치로 넘어가는 지점
// %2725/%2683 ← 배치B L976 · %2671 ← 배치B L969 · %2768 ← 배치B L948/L960(lapse) · %1736 ← dive_episode.rs:42 focus() default → 배치A L711 · 언와인드 %2270→L769(배치B) · %3303→L1672(배치E) · 정상 종료 %3897 → L1253(배치D)

// handler.rs:1252~1672 (배치 D)
// 진입: 배치 C 의 블록들(%3507/%3573/%3623/%3636/%3649/%3664)이 %3897(24343) 로 합류. 레지스터: %0=self %210=&version %2=rnd %3=player %4=data %211=cache %212=game.data_ptr %214=game.vtable_ptr %1639=context %1930=&self.plan %2824=&self.sub_plan %2913=&cache.player_champion[team][pos] %2915=(내 챔피언 None) %1799=player.info.position %2867=player.info.team %4057=context.debug

// ── [1253~1291] 서브플랜 산출 + v3 귀환→패시브 사다리 ──
1253 let _t = ProfTimer::start(75)                      // prof::ENABLED 원자 로드가 0 이면 타이머 None(%151 tag -1)
1254 let mut sub: SubPlan(72B) = self.plan.sub_plan(version, rnd, player, data, self, &self.team_plan(0xf8), &self.positioning_score(0x990), debug)   // 자식 명세, 계약만
1263 if version > 1 /*항상 true*/ && sub.tag == 5(Recall) && self.v3_repair_done(1264 인라인 handler.rs:1851~1852: cache.player_champion[team][pos].is_some_and(|c| c.hp(0x670) >= c.stat_cached.hp(0x628)))  {
1265     self.v3_home_ladder[0](0x1640) += 1
1266     let plan = self.passive_plan(version, rnd, player, data, debug)          // sret 392B 중 앞 384B 만 BigPlan 으로 memcpy
1267     let mut swapped_ok = false
         if plan_allowed(context, &plan) {   // rule_scope.rs:101 인라인 → goal = plan.goal()(BigGoal 24B) → goal_allowed(rule_scope.rs:90):
                                            //   tag0 Line(line@+1): line_exists = tutorial(ctx+0x38) ∈ Top{0,2,7,8} / Mid{0,4,5,7,8} / Bottom{0,1,3,5,7,8}
                                            //   tag1 Jungle: player_count → tutorial ∈ {0,6,8}
                                            //   tag2 Epic: morgard_exists → tutorial ∉ {1..6}   tag3 Serpen: serpen_exists → tutorial ∈ {0,5,7,8}
                                            //   tag4 Nexus / 5 Battle / 6 Recall → 항상 허용
1268         self.mf_note_swap(29, game.tick())   // self.mf_swap = (29, tick)
1269         drop(self.plan); self.plan = plan                     // ★HEAP plan@0x5e8 교체 #1
1270         sub = self.plan.sub_plan(…같은 인자…)
1272         swapped_ok = (sub.tag != 5)          // 여전히 Recall 이면 아래 사다리 계속
         }
1272     if !swapped_ok {
1273         self.v3_home_ladder[1](0x1648) += 1
1274         let line = fallback_line(context, match player.info.position { Top(0)→Top(0), Bottom(3)|Support(4)→Bottom(2), _→Mid(1) })
1279         let plan = BigPlan::PassiveLine(PassiveLinePlan::new(line))   // tag 3, 280B 페이로드 0-초기화 + 5개 빈 Vec(ptr=8) + line@+0x11e
1280         if plan_allowed(context, &plan) {
1281             drop(self.plan); self.plan = plan                 // ★HEAP plan@0x5e8 교체 #2
1282             sub = self.plan.sub_plan(…)
1284             if sub.tag == 5(Recall) { 1285 self.v3_home_ladder[2](0x1650) += 1; 1286 sub = SubPlan::LineWait(line) /*tag4, line@+8*/ }
         } else { 1285 self.v3_home_ladder[2] += 1; 1286 sub = SubPlan::LineWait(line); drop(plan) }
     }
     // 1289 임시 plan(%150) drop — passive_plan 결과가 self 로 이동되지 않은 경우만(%3982==1)
 }
1290 SubPlan::merge(&mut self.sub_plan(0x768), &sub)        // 자식 명세, 계약만
1291 drop(_t)   // ENABLED 였으면 PHASE_NANOS[75] += elapsed_ns, PHASE_CALLS[75] += 1

// ── [1294~1505] trace (context.trace_level(0x39) != Off) — 계측, 판단 없음 ──
1294 if context.trace_level != 0 {
1295     let name: String = self.plan.get_name()
1296     if name != self.prev_plan_name(0x840)  /*len 비교 후 memcmp*/ {
1298         let reason = determine_transition_reason(&prev, &name)   // 인라인 handler.rs:1675~1707, 순서대로:
             //  1679 name.starts_with("LineGanker") → 3 GankInitiated
             //  1682 prev.starts_with("LineGanker") && !name.starts_with("LineGanker") → 4 GankCompleted
             //  1687 name.starts_with("Battle") → 2 BattleTriggered
             //  1692 name.starts_with("Passive") && !prev.starts_with("Passive") → 0 PlanEnded
             //  1697 name.starts_with("ActiveRecall") → 6 LowHp
             //  1702 name.contains("Epic")||contains("Serpen")||contains("Nexus") → 5 ObjectiveChange
             //  1706 else → 9 Other(format!("{prev} -> {name}"))
1301~1305 my_champ = cache.player_champion[team][pos]; hp_ratio = my_champ.map(|c| c.hp*100/c.stat_cached.hp /*max 0 이면 div_by_zero panic*/).unwrap_or(0)
             ally_count = player_champion[team] 의 Some 개수; enemy_count = player_champion[1-team] 의 Some 개수 (1304/1305 count, 필터 없음)
1307 tick = game.tick()
1310 self.pending_trace_events.push(PendingTraceEvent{ event: PlanTransition{reason, from: prev.clone(), to: name.clone(), context: PlanContext{hp_ratio, ally_count, enemy_count, tick, gold_diff: 0}}, tick: game.tick()(1311) })
1321 if name.starts_with("Battle") && !prev.starts_with("Battle") {
1323     self.battle_start_tick = Some(game.tick())
1325     if self.plan.tag == 9(Battle) {
1326         focus = match plan.Battle.sub_goal.tag(0x648) { 0,1,2,3,5,6 → sub_goal.focus(0x650), 4 RunAway|7 End → 0 }   // BattlePlan::focus 인라인 battle.rs:30
1332         (my_hp%, my_max) = my_champ.map(|c| (c.hp*100/c.max, c.max)).unwrap_or((0,0))
1337         target = game.get_entity_by_id(focus); 1339 (t_hp%, t_max) = target.map(..).unwrap_or((0,0))
1344         engage_reason = match plan.Battle.main_goal.tag(0x630) {
                 0 TryKill → 1347 if prev.starts_with("LineGanker") {"gank_kill"} else 1349 if prev.contains("Hunt")||prev.contains("Epic") {"objective_kill"} else {"try_kill"}
                 1 Support → "support_ally"(1355)   2 Response → "forced_response"(1356)   3 Avoid → "avoid_fight"(1357) }
1361         initial_sub_goal = format!("{:?}", plan.Battle.sub_goal)
1367         ally_count = (0..5).filter(|i| player_champion[team][i].is_some_and(|a| a.distance_sq(my_champ) < 150000²)).count()   // aux sg_0 (my_champ.unwrap() — None 이면 panic 경로 %4390)
1372         enemy_count = player_champion[1-team].iter().flatten().filter(|e| e 가 내 팀에 가시(entity.rs:1482) && !is_ignored_well_enemy(version, player, e) && e.distance_sq(my_champ) < 150000²).count()   // aux sh_0
1375         drop(self.battle_start_state); self.battle_start_state = Some(BattleStartState{engage_reason, initial_sub_goal, tick: game.tick()(1376), my_hp: my_hp%, my_max_hp, target_id: focus, target_hp: t_hp%, target_max_hp, ally_count, enemy_count})
1388         self.pending_trace_events.push({EngageDecision{target_id: focus, my_die_tick:0, enemy_die_tick:0, expected_win:false}, tick: game.tick()(1389)})
1397         drop(engage_reason 원본)
         }
     }
1401 if prev.starts_with("Battle") && !name.starts_with("Battle") {
1402     tick = game.tick(); 1403 start = self.battle_start_tick.unwrap_or(tick); duration = tick.saturating_sub(start)
1407     state = self.battle_start_state.take().unwrap_or_default()   // Default: 빈 String 2개 + 0
1413     (my_hp_end%, my_hp_now) = my_champ.map(|c| (c.hp*100/c.max, c.hp)).unwrap_or((0,0))
1418     target = game.get_entity_by_id(state.target_id(+0x48)); 1420 (t_hp_end%, t_hp_now) = target.map(..).unwrap_or((0,0))
1425     damage_taken = (state.my_hp(+0x38) * state.my_max_hp(+0x40) / 100).saturating_sub(my_hp_now)
1428     damage_dealt = (state.target_hp(+0x50) * state.target_max_hp(+0x58) / 100).saturating_sub(t_hp_now); 1429 target None 이면 = 시작 hp 전부
1437     kills  = game.kill_logs().iter().filter(|k| start<=k.tick<=tick && k.killer_team==team && (k.killer_position==pos || k.assist.contains(&pos))).count()
1443     deaths = kill_logs.filter(|k| start<=k.tick<=tick && k.killer_team!=team && k.killed_position==pos).count()
1450     (disengage_reason, deaths_field, target_escaped) =
             if deaths>0 {("died", deaths, false)} else if kills>0 {("kill_secured",0,false)} else if my_hp_end% < 30 {("low_hp_retreat",0, target.is_none())} else if target.is_none() {("target_escaped",0,true)} else {("target_died",0,false)}
1472     won = kills != 0 && deaths_field == 0
1475     self.pending_trace_events.push({BattleResult{engage_reason: state.engage_reason, disengage_reason, initial_sub_goal: state.initial_sub_goal, kills, deaths: deaths_field, duration, ally_count: state.ally_count, enemy_count: state.enemy_count, my_hp_start: state.my_hp, my_hp_end, target_hp_start: state.target_hp, target_hp_end, damage_dealt, damage_taken, target_id: state.target_id, won, prediction_correct: won, target_escaped}, tick: game.tick()(1476)})
     }
1501 self.battle_start_tick = None      // ⚠ %4436 — 1321/1401 어느 쪽이든 name!=prev 블록 끝에서 무조건
     }
1504 drop(self.prev_plan_name); self.prev_plan_name = name     // name==prev 여도 실행(%4069→%4114)
 }

// ── [1507~1512] 디버그 라벨 (context.debug(0x3b)) ──
1507 if context.debug { champ = my_champ.unwrap() /*None 이면 panic %4730*/;
1509 debug.infos(+0xa0).entry(champ.id(0x5c0)).or_insert(vec![]).push(format!("speed : {}", champ.stat_cached.move_speed(0x640)))
1510 …push(format!("Main Objective: {:?}", self.team_plan.objective(0x517)))   1511 …push(format!("Plan: {:?}", self.plan.get_name()))   1512 …push(format!("SubPlan: {:?}", self.sub_plan)) }

// ── [1520~1622] SIM_STATS 도주 ring 계측 (prof::SIM_STATS != 0 && position != Jungle(1)) ──
1521 if setting.is_line_phase(game.tick())  /*setting.rs:703: tick < epic_jungle.first_spawn_tick(0x8a8).saturating_sub(tick_per_second(0x12f8)*30)*/ {
1522   death_count = player.info.statistics.death(0x5f8)
1523   if death_count > self.flee_prev_death_count(0x15b0) {
1524     self.flee_prev_death_count = death_count
1530     match self.flee_ring.last() {
         None → 1565 (tick=game.tick(), plan_tag=-1(255), gate=-1, label=4)
         Some((t_last, _, ptag_last, _)) → 1531 win = t_last.saturating_sub(tps*10)
1536       for (t, block, ptag, gate) in ring.iter().rev() { 1537 if t < win break;
1540         if block&1 != 0 { any_danger=true; 1542 A |= block&4!=0; B |= (block&6)==6;
1548           if (block&6)==2 && !found { found=true; rec=(ptag, gate) } } }
1553       if found → (t_last, rec.ptag, rec.gate, label 0)
1555       else → (t_last, ptag_last, -1, label = if B {1} else if A {2} else if any_danger {3} else {4}) }
1567     self.flee_death_retrospects.push((tick, label, ptag, gate))
1568     if context.debug { 1569 debug.add_log(data, player, format!("V46DEATH T{team} {pos:?} tick~{tick} label={LABELS[label]} plan_tag={ptag} block={gate}")) }   // LABELS = @anon.85 &str 표(이름 미해독)
1574     self.flee_ring.clear()
1575   } else if game.tick() % 6 == 0 && my_champ.is_some() {
1577     champ = my_champ
1579     enemies: bumpalo Vec = iter_champions(1-team).filter(sm_0: e.distance_sq(champ) < 150000² && (champ.team==Neutral || e.visible_state[champ.team]==Visible) && !is_ignored_well_enemy(version, player, e)).collect_in(context.pool)
1582     if enemies.is_empty() { drop; → 1629 } else {
1583       dis = match player_count(context)/*tutorial*/ { First(1)|Bottom(3) → setting.tower_attack_disable_tick_2v2(0x1400), MidBottom(5) → _3v3(0x1408), _ → tower_attack_disable_tick(0x13f8) }
1588       towers = iter_towers_without_nexus(cache, 1-team).filter(su_0: game.tick() <= dis && t.distance_sq(champ) < 150000²).collect_in(pool)
1593       mdt = check_kill_die_tick(version, rnd, data, player, champ, enemies(복사 32B), &towers, debug)
1594       my_tower = iter_towers_without_nexus(cache, team).min_by_key(so_0 /*미독해*/).or(cache.nexus[team](0x170))
1596       nrst = my_tower.map(|t| champ.distance(t) / max(champ.move_speed, 1)).unwrap_or(0)
1597 a = mdt < 600;  1598 b = mdt > nrst
1599       fleeing = self.sub_plan.tag==5(Recall) || self.plan.battle_engage_dir() == Some(-1) || (1600 self.plan.tag==3 && plan.PassiveLine.v46_flee(0x702))
1601       plan_tag_code = match self.plan.tag { 3 PassiveLine→0, 9 Battle→1, 8 ActiveRecall→2, 12|13|14|15 Epic/Serpen→3, _→4 }
1609       block = (a as u8) | (b?2:0) | (fleeing?4:0)
1612       gate = if plan.tag==3 && !fleeing && a && b { 1613 v46_flee_gate_check(version, player, data, champ).0 /*sret 40B 첫 u8, 내부 bumpalo Vec drop*/ } else { 255 }
1617       self.flee_ring.push((game.tick(), block, plan_tag_code, gate))
1618       if len_before > 127 { 1619 self.flee_ring.remove(0) }
1622       drop(enemies) } } }

// ── [1629~1670] DIEWIN 디버그 로그 (position ∈ {Bottom(3),Support(4)} && context.debug) ──
1630 if game.tick()%6==0 && my_champ.is_some() { 1631 champ;
1633 enemies = iter_champions(1-team).filter(sq_0 ≡ sm_0).collect_in(pool); 1636 if !empty {
1637 ne = enemies.len; 1639 nearest = enemies.min_by_key(|e| e.distance_sq(champ)) (sr_0);
1640 (myrng, erng, nrst, ehp) = nearest.map(|e| (1641 champ.attack_effect.map_or(false,|f| f.is_in_range(champ,e)), 1642 e.attack_effect.map_or(false,|f| f.is_in_range(e,champ)), 1643 e.distance(champ)/1000, e.hp*100/max(e.max,1))).unwrap_or((0,0,0,0))
1645 dis = (1583 과 동일 선택); 1650 towers = 적 타워 filter(sn_0≡su_0); 1654 mdt = check_kill_die_tick(…); 1655 hp = champ.hp*100/max(champ.max,1)
1656 sub = match self.sub_plan.tag { 2→"LineDefense",3→"LineSafe",4→"LineWait",5→"Recall",7→"Battle",9→"Hide",_→"Other" }
1665 debug.add_log(data, player, format!("DIEWIN T{team} {pos:?} hp={hp}% mdt={mdt} ne={ne} nrst={nrst}k myrng={myrng} erng={erng} ehp={ehp}% plan={get_name()} sub={sub}")) } 1670 drop(enemies) }

// ── 꼬리 ──
5069(28058) drop(_t_tail %191 — 배치 A L768 의 ProfTimer, 인덱스 동적·bounds 132) → 5362 → 679(16488) ret void  ;L1672
// unwind 정리 블록(1640/2013/2039/2270/3303/… L1672 cleanuppad)은 본 범위에 귀속되나 논리 없음(BigPlan/ProfTimer/String drop_glue)
```

**`mem` 메모리 접근 241건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | OperationData | 0x0 | cache | r | L688·L694 `%211 = load ptr, ptr %4`(m13.ll:15200) — &AbstractGameWithCache. tcxdict OperationData 0x0 cache \| (배치 D) %211 = &AbstractGameWithCache | 3 | OK |  |
| 1 | AbstractGameWithCache | 0x0 | game.data_ptr | r | L688 `%212 = load ptr, ptr %211`(m13.ll:15207) — &dyn AbstractGame 팻포인터 데이터 절반. tcxdict AbstractGameWithCache 0x0 game(16B) | 3 | OK |  |
| 2 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r | L688 `%213 = gep %211, 8 ; %214 = load`(m13.ll:15208~15209) — 팻포인터 vtable 절반 | 4 | OK |  |
| 3 | AbstractGame vtable | 0x40 | get_game_mode | r | L688 `%215 = gep %214, 64 ; %216 = load(invariant)`(m13.ll:15210~15211) → `%217 = call {i64,ptr} %216(%212)`(15212) · divtable AbstractGame 0x40 = get_game_mode · 반환 GameMode(16B: +0 태그 i64 Direct, +8 &Mode). L694 는 같은 %216/%212 로 재호출(%676, m13.ll:16483) = 소스에서 get_game_mode() 를 두 번 부름 | 3 | 확인불가(vtable 슬롯) |  |
| 4 | LegacyPlanHandler | 0xf8 | team_plan | r | 703 TeamPlan::update(&mut) · force_plan_update:1740/1748 update_objective(&mut) · 765 next_plan 에 ptr(&mut 추정: 속성 없음) | 5 | OK |  |
| 5 | LegacyPlanHandler | 0x1b8 | team_plan.chats | r | 705 drain(..) 소스 · force_plan_update:1739 len 읽기(0x1c8) · :1745 truncate | 4 | OK |  |
| 6 | LegacyPlanHandler | 0x1c8 | team_plan.chats.len | r | force_plan_update:1739 before_chats | 4 | OK |  |
| 7 | LegacyPlanHandler | 0x517 | team_plan.objective@tag | r | None=-1(0xff, IR 정본). 853/866 태그 비교(11 ComebackPick·8 Gank·9 Dive) · 931 is_none · 970 obj_key 3B 복사 · force_plan_update:1737/1741 before_obj/cleared \| (배치 D) 1510 debug 라벨 `Main Objective: {:?}` Debug fmt 만(판단 없음) (=team_plan 0xf8 + 0x41f) | 3 | OK |  |
| 8 | LegacyPlanHandler | 0x458 | team_plan.comeback_pick_start_tick | r | 854 pick_start | 4 | OK |  |
| 9 | LegacyPlanHandler | 0x1e0 | team_plan.comeback_pick_outcomes.len | r | 860/863 last_mut (ptr 0x1d8) | 4 | OK |  |
| 10 | LegacyPlanHandler | 0x5e8 | plan@tag | r | 758/784 chats() 태그 스위치 · 789 ==7 · 797 ==10 · 850 ==9 · 948 ==2 · 960 is_passive · force_plan_update:1719/1752/1757 ==9. 태그 6 은 assume ne(무효값) \| (배치 C) BigPlan 판별자(니치 tag = idx+2, DeathMatchBattle 암묵). L979·991·1005·1015·1098·1118·1125(599)·1129·1134·1179·1189·1200·1251(39) 에서 switch/eq \| (배치 D) 1325/1599/1601/1656 BigPlan 태그(9=Battle·3=PassiveLine·8=ActiveRecall·12~15=Epic/Serpen) · 1254/1270/1282 sub_plan() 의 &self.plan · 1295/1511/1668 get_name | 4 | OK |  |
| 11 | LegacyPlanHandler | 0x5f0 | plan 페이로드(+8) — chats() 오프셋 1520/1544/1624/1760 | r | BigPlan::chats(types.rs:185~189) 인라인: 태그 4·7·10·11 → +0x5f0 / 3 → +0x608 / 5·9 → +0x658 / DeathMatchBattle(암묵) → +0x6e0 / 그 외(2·8·12~17) → None | 4 | OK |  |
| 12 | LegacyPlanHandler | 0x618 | plan@LineGanker.0.line | r | 803 active_gank_line=Some(line) · 816 GankResult.line | 4 | OK |  |
| 13 | LegacyPlanHandler | 0x638 | plan@PassiveJungle.0.team | r | 960 is_passive→is_counter_jungle(types.rs:257/passive_jungle.rs:104): team == player_team(0x640) 이면 passive 로 취급 | 4 | OK |  |
| 14 | LegacyPlanHandler | 0x640 | plan@PassiveJungle.0.player_team | r | 960 | 4 | OK |  |
| 15 | LegacyPlanHandler | 0x6f7 | plan@Battle.0.entry_src | r | force_plan_update:1719 선읽기 → :1758 ff_battle_exit.1 에 기록 | 4 | OK |  |
| 16 | LegacyPlanHandler | 0x990 | positioning_score | r | 765 next_plan 인자(2760B) | 4 | OK |  |
| 17 | LegacyPlanHandler | 0x7b0 | misunderstood_received_chats | r | 728 is_empty(len 0x7c0) · 742 retain | 4 | OK |  |
| 18 | LegacyPlanHandler | 0x7c0 | misunderstood_received_chats.len | r | 728 | 4 | OK |  |
| 19 | LegacyPlanHandler | 0x7f8 | received_chats | r | 728 is_empty(len 0x808) · 730 iter(ptr 0x800) · 735 retain | 4 | OK |  |
| 20 | LegacyPlanHandler | 0x800 | received_chats.ptr | r | 730 | 4 | OK |  |
| 21 | LegacyPlanHandler | 0x808 | received_chats.len | r | 728/730 | 4 | OK |  |
| 22 | LegacyPlanHandler | 0x7e0 | chats_wait | r | 745 is_empty(len 0x7f0) · 746 iter(ptr 0x7e8) · 754 retain | 4 | OK |  |
| 23 | LegacyPlanHandler | 0x7e8 | chats_wait.ptr | r | 746 | 4 | OK |  |
| 24 | LegacyPlanHandler | 0x7f0 | chats_wait.len | r | 745/746 | 4 | OK |  |
| 25 | LegacyPlanHandler | 0x7c8 | chats | r | 749 push 시 cap(0x7c8)/len(0x7d8)/ptr(0x7d0) 읽기 | 4 | OK |  |
| 26 | LegacyPlanHandler | 0x818 | gank_periods.ptr | r | 807/870/918 last(_mut) | 4 | OK |  |
| 27 | LegacyPlanHandler | 0x820 | gank_periods.len | r | 807/870/918/922 | 4 | OK |  |
| 28 | LegacyPlanHandler | 0x838 | gank_score_attempts.len | r | 927 push | 4 | OK |  |
| 29 | LegacyPlanHandler | 0x1470 | gank_cancel_tick | r | 775 can_transition = tick.saturating_sub(gank_cancel_tick) > tps*10 | 4 | OK |  |
| 30 | LegacyPlanHandler | 0x14a0 | gank_attempt_count | r | 920 +=1 | 4 | OK |  |
| 31 | LegacyPlanHandler | 0x1618 | mf_swap.1 | r | 952 `mf_swap.1 != game.tick()` 이면 953 mf_note_swap(mf_src) \| (배치 C) L997 != tick 이면 mf_note_swap | 4 | OK |  |
| 32 | LegacyPlanHandler | 0x1806 | counter_jungle_route_init | r | 716 게이트 | 4 | OK |  |
| 33 | LegacyPlanHandler | 0x1808 | v3_armed | r | 969 게이트(true → 970~976, false → 배치 C 988) | 4 | OK |  |
| 34 | LegacyPlanHandler | 0x180c | v2_egowave_line | r | update_v2_egowave:482 `v2_egowave!=0 && v2_egowave_line==my_line` 이면 재롤 생략 | 4 | OK |  |
| 35 | LegacyPlanHandler | 0x1815 | v2_egowave | r | update_v2_egowave:482 | 4 | OK |  |
| 36 | LegacyPlanHandler | 0x180d | active_gank_line | r | 879 unwrap_or(Mid=1) → fallback_line | 4 | OK |  |
| 37 | LegacyPlanHandler | 0x180e | v3_dest_obj | r | 976 `== obj_key`(None=-1 / 둘 다 Some 이면 MainObjective::eq 호출) | 4 | OK |  |
| 38 | LegacyPlanHandler | 0x548 | v3_dest@tag | r | 976 Some 판정(trunc i64→i1) · .0(0x550) .1(0x558) 로드=dx,dy(소비는 배치 C 977) \| (배치 C) 배치B L976 에서 Some 확정 후 본 범위 L977 (dx,dy)=0x550/0x558 사용(로드는 B, 21629~21633) | 4 | OK |  |
| 39 | PlayerState | 0x9c0 | info.position@tag | r | ==1(Jungle): egowave 생략(update_v2_egowave:480) · 716 카운터정글 초기화 조건 · 789 정글 장부 게이트(%1800 = position!=Jungle) \| (배치 D) %1799(19417). 1274 match(0 Top→Top,3\|4 Bottom\|Support→Bottom,else Mid) · 1440/1446 킬로그 대조 · 1520 !=1(Jungle) · 1629 in{3,4} | 4 | OK |  |
| 40 | PlayerState | 0x930 | info.team | r | blackboard[team] 인덱스(bounds<2) · 722 new_counter_jungle · 클로저 killer_team 비교 \| (배치 C) L977·1039·1059·1075(1-team)·1251(44·55) — 전부 `<2` bounds check 동반 \| (배치 D) %2867(22078). player_champion[team] · 1-team = 적팀 · 1439/1445 killer_team 대조 | 4 | OK |  |
| 41 | PlayerState | 0x180 | info.parameter (AthleteParameter 744B) | r | egowave_check:2273/2274 roaming_ratio()·ego_ratio() | 4 | OK |  |
| 42 | OperationData | 0x0 | cache → game.data_ptr(+0)/game.vtable_ptr(+8) | r | %211/%212/%214. vtable+0x28=tick() (divtable) · +0x130=kill_logs() | 3 | OK |  |
| 43 | OperationData | 0x8 | context (&GameContext) | r | %1639 | 4 | OK |  |
| 44 | OperationData | 0x10 | blackboard (&[Blackboard;2]) | r | %1713 · egowave_check:2281 blackboard[player.team] | 4 | OK |  |
| 45 | GameContext | 0x8 | setting → tick_per_second(GameSetting+0x12f8) | r | 775/919 tps*10 | 4 | OK |  |
| 46 | GameContext | 0x38 | tutorial(TutorialType) | r | rule_scope 인라인들: line_exists/fallback_line/player_count/morgard_exists/serpen_exists 전부 이 1바이트 스위치 | 4 | OK |  |
| 47 | GameContext | 0x39 | trace_level | r | 813/876 is_enabled = !=Off(0) \| (배치 D) 1294 trace.rs:15 is_enabled = !=0(Off) | 4 | OK |  |
| 48 | Blackboard | 0x10 | {top,mid,bottom}_minion_state.from_mid (line별 +0/+0x28/+0x50) | r | egowave_check:2282 `< -2000` | 4 | OK |  |
| 49 | Blackboard | 0x20 | {top,mid,bottom}_minion_state.minion_count | r | egowave_check:2282 `< -2` | 4 | OK |  |
| 50 | Strategy(local, PlayerState::strategy 반환 24B) | 0x12 | early_jungle | r | 719 ==2(CounterJungle) | 4 | OK |  |
| 51 | Strategy(local) | 0x11 | focused | r | 722 new_counter_jungle 인자 | 4 | OK |  |
| 52 | BigPlan(local next, %190/%193) | 0x31 | @LineGanker.0.phase | r | 772 ==6(WaitResponse 메모리태그; 논리idx0, niche_start 6) | 4 | OK |  |
| 53 | BigPlan(local next) | 0x30 | @LineGanker.0.line | r | 926 evaluate_gank_opportunity_with_score 인자 · 932 objective=Gank{line} | 4 | OK |  |
| 54 | BigGoal(local goal, BigPlan::goal 반환 24B) | 0x1 | @Line.line | r | 770 goal_allowed → line_exists(ctx,line) | 4 | OK |  |
| 55 | KillLog(aux) | 0x18 | tick | r | 클로저: start<=tick<=end | 4 | OK |  |
| 56 | KillLog(aux) | 0x20 | killer_team | r | kills: ==player.team / deaths: !=player.team / 856 any: ==player.team | 4 | OK |  |
| 57 | KillLog(aux) | 0x28 | killer_position | r | kills: ==Jungle(1) 이면 즉시 카운트, 아니면 assist.contains(Jungle) | 4 | OK |  |
| 58 | KillLog(aux) | 0x2c | killed_position | r | deaths: ==Jungle(1) | 4 | OK |  |
| 59 | KillLog(aux) | 0x8 | assist.ptr/len(0x10) | r | kills: Position::Jungle.slice_contains(assist) | 4 | 부분일치(첫 성분(컨테이너)만 맞고 **마지막 성분이 다르다** — 사람이 봐야 ) |  |
| 60 | LegacyPlanHandler | 0x5f0 | plan@PassiveLine.0 / plan@Battle.0 등 페이로드 시작 | r | PassiveLine: +0x18 chats(0x608) · +0x60 v46_pending(0x650 trigger_ticks · 0x668 flee_episodes · 0x680~0x6e8 카운터 14) · +0x116 line(0x706). Battle: +0x68 chats(0x658) · +0x80 v54_reentry_ticks(0x670) · +0x98 dive_catch_break_ticks(0x688) · +0xf0 dive_last_race_adv(0x6e0) · +0xf6 with_dive(0x6e6) · +0xf7 dive_entered(0x6e7) · +0xfa dive_abandoned(0x6ea) · +0xfe dive_tower(0x6ee) · +0x40 main_goal@tag(0x630)·+0x48 payload(0x638) · +0x58 sub_goal@tag(0x648)·+0x60 focus(0x650) · +0x103 dive_abort_src(0x6f3) · +0x104~0x106 dive_last_model/na/ne(0x6f4~0x6f6) · +0x107 entry_src(0x6f7) · +0x108 exit_src(0x6f8) · +0x109~0x111 ff_wave_*·ff_exit1_cls·exit_sub(0x6f9~0x701). PassiveJungle: +0x48 team(0x638)·+0x50 player_team(0x640). LineGanker: +0x28 line(0x618) | 4 | OK |  |
| 61 | LegacyPlanHandler | 0x768 | sub_plan@tag | r | L1023 == 7(Battle) \| (배치 D) 1599/1656 태그 5=Recall 판정 · 1290 merge 대상 · 1512 Debug fmt | 4 | OK |  |
| 62 | LegacyPlanHandler | 0x790 | sub_plan@Battle.0.v48_claim_hold_until | r | L1025 | 4 | OK |  |
| 63 | LegacyPlanHandler | 0x79b | sub_plan@Battle.0.v48_dodge_claim | r | L1023 bool | 4 | OK |  |
| 64 | LegacyPlanHandler | 0x517 | team_plan.objective (Option<MainObjective> 3B) | r | L1125(616) BattlePlan.main_objective 로 복사 · L1190 tag&!1==8 (Gank8\|Dive9) | 4 | OK |  |
| 65 | LegacyPlanHandler | 0x570 | v50_dive_ep_live@tag | r | L1251(65·68·108) -1=None | 4 | OK |  |
| 66 | LegacyPlanHandler | 0x5e1 | v50_dive_ep_live@Some.0.tower | r | L1251(65) live.tower != active.tower_ty → fold | 4 | OK |  |
| 67 | LegacyPlanHandler | 0x5e2 | v50_dive_ep_live@Some.0.start_model | r | L1251(73) ==0 이면 스냅샷 채움 | 4 | OK |  |
| 68 | LegacyPlanHandler | 0x5c0 | v50_dive_ep_live@Some.0.gap_ticks | r | L1251(110) < tps | 4 | OK |  |
| 69 | LegacyPlanHandler | 0x7c8 | chats (Vec<Chat>) | r | L1016 extend 대상(&mut) | 4 | OK |  |
| 70 | LegacyPlanHandler | 0x810 | gank_periods (Vec<(usize,usize)>) ptr 0x818·len 0x820 | r | L1135 last_mut() | 4 | OK |  |
| 71 | LegacyPlanHandler | 0x1550 | v48_claim_until_tick | r | L1025 max · L1030 now > 이면 claim 창 밖 | 4 | OK |  |
| 72 | LegacyPlanHandler | 0x1558 | v48_last_non_target_hit | r | L1034 nth > last | 4 | OK |  |
| 73 | LegacyPlanHandler | 0x1814 | v48_last_cast_state | r | L1067 cur == last 이면 계측 생략 | 4 | OK |  |
| 74 | LegacyPlanHandler | 0x0 | data (GoalData 248B) | r | L1010 BigPlan::update 에 &GoalData(readonly) 로 전달 · L1129 is_end 에도 | 4 | OK |  |
| 75 | LegacyPlanHandler | 0x990 | positioning_score (2760B) | r | L1010·L1125(617) readonly 전달 | 4 | OK |  |
| 76 | PlayerState | 0x9c0 | info.position@tag (i32) | r | as_index → player_champion[team][pos] · L1133 !=1(Jungle) 이면 갱크 결과 트레이스 생략 | 4 | OK |  |
| 77 | PlayerState | 0x678 | info.statistics.non_target_hit | r | L1033 nth | 4 | OK |  |
| 78 | OperationData | 0x0 | cache (&AbstractGameWithCache) | r | %211 | 4 | OK |  |
| 79 | AbstractGameWithCache | 0x0 | game.data_ptr / game.vtable_ptr(+0x8) | r | vtable 슬롯 0x28 tick · 0x130 kill_logs · 0x1f0 get_entity_by_id (divtable AbstractGame) | 3 | OK |  |
| 80 | AbstractGameWithCache | 0x1e0 | player_champion[team][pos] | r | gep 480 + team*40 + pos*8 (Option<&Entity>, null=None) \| (배치 D) %2913 = cache+480+team*40+pos*8 → 내 챔피언 Option<&Entity>(%2915 = null 판정). [1-team] 5슬롯 = 적 | 4 | OK |  |
| 81 | GameContext | 0x8 | setting (&GameSetting) | r | L1251(66·110·118) setting.tick_per_second(0x12f8) | 4 | OK |  |
| 82 | GameContext | 0x39 | trace_level (TraceLevel 1B) | r | L1141 is_enabled = !=0(Off) | 4 | OK |  |
| 83 | GameContext | 0x3b | debug (bool) | r | L1125(624) 디버그 문자열 기록 게이트 | 4 | OK |  |
| 84 | GameSetting | 0x12f8 | tick_per_second | r | L1251(110) gap_ticks < tps \| (배치 D) 1521 tps*30 · 1531 tps*10 | 4 | OK |  |
| 85 | DebugFrameData | 0xa0 | infos (HashMap<usize, Vec<String>>) | r | L1125(626) entry(champ.id).or_insert(Vec::new()).push(format!(..)) | 4 | OK |  |
| 86 | Entity | 0x0 | team@tag (0=Player) | r | L1251(44·55) · aux is_visible_from | 4 | OK |  |
| 87 | Entity | 0x8 | team@Player.0 | r | L1251(44) == 1-team(적) · (55) == team(아군) | 4 | OK |  |
| 88 | Entity | 0x38 | visible_state[team]@tag (stride 16) | r | aux closure$11 is_visible_from: tag==0 이면 가시 | 4 | OK |  |
| 89 | Entity | 0x68 | ty@tag (EntityType) | r | 13=Champion(L1040·1060·1251/55) · 2=Tower(L1251/53) | 4 | OK |  |
| 90 | Entity | 0x70 | ty@Champion.0.action_state@tag | r | L1041 {2 Move,3 Attack,4 Skill,5 Skill2,6 Ult} · L1061 {4,5,6} | 4 | OK |  |
| 91 | Entity | 0x88 | ty@Tower.info.nearest_enemy@tag | r | L1251(54) | 4 | OK |  |
| 92 | Entity | 0x98 | ty@Tower.info.nearest_enemy@Some.0.1 (entity id) | r | L1251(54~56) 타워 표적 id | 4 | OK |  |
| 93 | Entity | 0x438 | stat_buff_cached.range | r | Effect::range 인라인(effect.rs:26) L1083·L1251(51) | 4 | OK |  |
| 94 | Entity | 0x470 | stat_buff_cached.radius_mult (i32) | r | Entity::radius 인라인(entity.rs:1511~1515) L1251(51) | 4 | OK |  |
| 95 | Entity | 0x490 | attack_effect@Some.0 (Effect) — range 0x4a0 · growth_range 0x4a8 · casting@tag 0x4c0(-1=None) | r | L1251(50~51) 타워 공격 이펙트 | 4 | OK |  |
| 96 | Entity | 0x4c8 | skill_effect@Some.0 (Effect) — range 0x4d8 · growth 0x4e0 · start_timing 0x4e8 · casting@tag 0x4f8 | r | L1069 | 4 | OK |  |
| 97 | Entity | 0x500 | skill2_effect@Some.0 — casting@tag 0x530 | r | L1070 skill2_effect(): level>2 이면 &self.skill2_effect 아니면 정적 None(@anon.76 = FF FF FF FF) | 4 | OK |  |
| 98 | Entity | 0x538 | ult_effect@Some.0 — casting@tag 0x568 | r | L1071 ult_effect(): level>4 조건 | 4 | OK |  |
| 99 | Entity | 0x5c0 | id | r | L1125(626) 디버그 키 · L1251(45·56) \| (배치 D) 1509 debug.infos 키 | 4 | OK |  |
| 100 | Entity | 0x5c8 | level | r | L1070/1071 스킬 해금 · Effect::range 의 (level-1)*growth | 4 | OK |  |
| 101 | Entity | 0x628 | stat_cached.hp (max hp) | r | L1251(58) tgt_hp_pct 분모 max(.,1) | 4 | OK |  |
| 102 | Entity | 0x660 | x | r | L978 · aux distance_sq \| (배치 D) closure distance_sq (aux) · 1639 nearest key | 4 | OK |  |
| 103 | Entity | 0x668 | y | r | closure distance_sq (aux) | 4 | OK |  |
| 104 | Entity | 0x670 | hp | r | L1251(58·59) \| (배치 D) 1264 v3_repair_done(hp >= stat_cached.hp) · 1303/1332/1339/1413/1420/1643/1655 hp*100/max | 4 | OK |  |
| 105 | Entity | 0x680 | radius | r | L1251(51) | 4 | OK |  |
| 106 | Effect | 0x10 | range | r | +0x18 growth_range · +0x20 start_timing(L1078 windup) · +0x30 casting(CastingType: 1 Position·2 Direction = is_nontarget) | 4 | OK |  |
| 107 | KillLog | 0x0 | assist(Vec<Position>) · 0x18 tick · 0x20 killer_team · 0x28 killer_position · 0x2c killed_position | r | aux sb_0/sc_0 (stride 48) | 4 | OK |  |
| 108 | LegacyPlanHandler | 0x630 | plan@Battle.main_goal@tag | r | 1344 switch 0 TryKill/1 Support/2 Response/3 Avoid → engage_reason 문자열 (=BattlePlan+0x40) | 4 | OK |  |
| 109 | LegacyPlanHandler | 0x648 | plan@Battle.sub_goal@tag | r | 1326 BattlePlan::focus 인라인(battle.rs:30): 태그 0,1,2,3,5,6 → focus 읽음, 4(RunAway)/7(End) → 0 | 4 | OK |  |
| 110 | LegacyPlanHandler | 0x650 | plan@Battle.sub_goal.focus | r | 1326 focus 엔티티 id | 4 | OK |  |
| 111 | LegacyPlanHandler | 0x702 | plan@PassiveLine.v46_flee | r | 1600 PassiveLinePlan+0x112 bool (passive_line.rs:208 v46_fleeing 인라인) | 4 | OK |  |
| 112 | LegacyPlanHandler | 0x840 | prev_plan_name | r | String cap@0x840 ptr@0x848 len@0x850. 1296 name 비교(len+memcmp) · 1298/1321/1347/1349/1401 starts_with/contains | 4 | OK |  |
| 113 | LegacyPlanHandler | 0x520 | battle_start_tick | r | 1403 unwrap_or(tick). ⚠본 함수 안에서 1323 Some 직후 1501 에서 항상 None 으로 되돌아가므로 다음 호출의 1403 은 항상 None(unknown 참조) | 4 | OK |  |
| 114 | LegacyPlanHandler | 0x918 | battle_start_state | r | 1407 take().unwrap_or_default() (tag -1 = None) | 4 | OK |  |
| 115 | LegacyPlanHandler | 0x15b0 | flee_prev_death_count | r | 1523 death_count > 이전값 | 4 | OK |  |
| 116 | LegacyPlanHandler | 0x8b8 | flee_ring | r | Vec<(usize,u8,u8,u8)> cap@0x8b8 ptr@0x8c0 len@0x8c8. 1530 last() · 1536 역순 순회 · 1618 len>127 | 4 | OK |  |
| 117 | PlayerState | 0x5f8 | info.statistics.death | r | 1522 death_count | 4 | OK |  |
| 118 | OperationData | 0x8 | context | r | %1639 = &GameContext | 4 | OK |  |
| 119 | GameContext | 0x0 | pool | r | 1581/1635 bumpalo Vec 수집 arena | 4 | OK |  |
| 120 | GameContext | 0x8 | setting | r | %4907/%5269 = &GameSetting | 4 | OK |  |
| 121 | GameContext | 0x38 | tutorial | r | rule_scope 인라인(line_exists/morgard_exists/serpen_exists/player_count) · 1583/1645 player_count → 타워공격금지틱 선택 | 4 | OK |  |
| 122 | GameContext | 0x3b | debug | r | 1507/1568/1629 디버그 게이트 (%4057) | 4 | OK |  |
| 123 | GameSetting | 0x8a8 | epic_jungle.first_spawn_tick | r | 1521 is_line_phase(setting.rs:703) | 4 | OK |  |
| 124 | GameSetting | 0x13f8 | tower_attack_disable_tick | r | 1586/1648 player_count 기본(5v5) → 타워 필터 클로저 캡처 | 4 | OK |  |
| 125 | GameSetting | 0x1400 | tower_attack_disable_tick_2v2 | r | tutorial First(1)/Bottom(3) | 4 | OK |  |
| 126 | GameSetting | 0x1408 | tower_attack_disable_tick_3v3 | r | tutorial MidBottom(5) | 4 | OK |  |
| 127 | AbstractGameWithCache | 0x170 | nexus[team] | r | 1595 가장 가까운 아군 타워 없으면 넥서스 대체 (.or) | 4 | OK |  |
| 128 | game.vtable_ptr | 0x28 | AbstractGame::tick | r | %2836. 1268/1307/1311/1323/1376/1389/1402/1476/1521/1565/1575/1617/1630 호출 | 4 | 확인불가(베이스 하위경로 'vtable_ptr' 를 game_core::Game ) |  |
| 129 | game.vtable_ptr | 0x130 | AbstractGame::kill_logs | r | 1437/1443 &Vec<KillLog>(ptr@+8 len@+16, 원소 48B) | 4 | 확인불가(베이스 하위경로 'vtable_ptr' 를 game_core::Game ) |  |
| 130 | game.vtable_ptr | 0x1f0 | AbstractGame::get_entity_by_id | r | 1337 focus · 1418 state.target_id | 4 | 확인불가(베이스 하위경로 'vtable_ptr' 를 game_core::Game ) |  |
| 131 | Entity | 0x628 | stat_cached.hp | r | 최대 HP. 0 이면 1303 udiv panic(div_by_zero) / 1643·1655 은 max(…,1) | 4 | OK |  |
| 132 | Entity | 0x640 | stat_cached.move_speed | r | 1509 `speed : {}` · 1596 distance/max(speed,1) | 4 | OK |  |
| 133 | Entity | 0x0 | team@tag | r | aux sm_0/sh_0 entity.rs:1482: champ.team Neutral(1) 이면 가시 true | 4 | OK |  |
| 134 | Entity | 0x38 | visible_state[team]@tag | r | aux entity.rs:1483: e.visible_state[champ.team] == Visible(0) | 4 | OK |  |
| 135 | Entity | 0x490 | attack_effect@Some.0 | r | 1641/1642 Effect::is_in_range(&effect, self, other) | 4 | OK |  |
| 136 | Entity | 0x4c0 | attack_effect@tag | r | 1641/1642 -1 = None → false | 4 | OK |  |
| 137 | KillLog | 0x18 | tick | r | 1438/1444 start<=tick<=now | 4 | OK |  |
| 138 | KillLog | 0x20 | killer_team | r | 1439 ==내팀(킬) / 1445 !=내팀(데스) | 4 | OK |  |
| 139 | KillLog | 0x28 | killer_position | r | 1440 == 내 포지션 | 4 | OK |  |
| 140 | KillLog | 0x2c | killed_position | r | 1446 == 내 포지션 | 4 | OK |  |
| 141 | KillLog | 0x0 | assist | r | 1440 Vec<Position>.contains(내 포지션) (slice_contains) | 4 | OK |  |
| 142 | DebugFrameData | 0xa0 | infos | r | 1509~1512 HashMap<usize,Vec<String>>.entry(champ.id).or_insert(vec![]).push(..) | 4 | OK |  |
| 143 | LegacyPlanHandler | 0x0 | data (GoalData 248B) | w | 708 — 콜리 부작용(계약만) | 4 | OK | GoalData::update(&mut self.data, version, rnd, player, data, debug) |
| 144 | LegacyPlanHandler | 0xf8 | team_plan | w | 703 — 콜리 부작용(계약만). force_plan_update:1740/1748 update_objective(&mut team_plan, version, rnd, player, data, &self(6168 ptr), &self.plan, debug) 도 동일 | 4 | OK | TeamPlan::update(&mut, version, rnd, player, data, debug) |
| 145 | LegacyPlanHandler | 0x1b8 | team_plan.chats | w | 705 `self.chats.extend(self.team_plan.chats.drain(..))` · force_plan_update:1745 `truncate(before_chats)`(lapse 롤백 시) | 4 | OK | drain(..) 로 비움 |
| 146 | LegacyPlanHandler | 0x7c8 | chats | w | 705(team_plan.chats) · 749(chats_wait 중 tick<=now && chat_allowed 인 것 clone push, grow_one) · 759(plan.chats() 현재 플랜) · 785(plan.chats() 전이 직전 현재 플랜) | 4 | OK | extend(drain) ×3 · push(chat.clone()) ×1 |
| 147 | LegacyPlanHandler | 0x7f8 | received_chats | w | 735 (aux m06.ll:2048) — 처리분(tick<=now) 제거 | 4 | OK | retain(\|(tick,_,_)\| *tick > game.tick()) |
| 148 | LegacyPlanHandler | 0x7b0 | misunderstood_received_chats | w | 742 (aux m06.ll:2184). 738 take_misunderstood_received_chat(&mut self,..) 콜리 부작용도 여기 | 4 | OK | retain(\|(tick,_,_)\| *tick > game.tick()) |
| 149 | LegacyPlanHandler | 0x7e0 | chats_wait | w | 754 (aux m06.ll:2727) | 4 | OK | retain(\|(tick,_)\| *tick > game.tick()) |
| 150 | LegacyPlanHandler | 0x1807 | v2_armed | w | update_v2_egowave:477 (711, version>=2 무조건) | 4 | OK | 1 |
| 151 | LegacyPlanHandler | 0x1808 | v3_armed | w | update_v2_egowave:478 | 4 | OK | 1 |
| 152 | LegacyPlanHandler | 0x180a | v3_epicops_armed | w | update_v2_egowave:479 | 4 | OK | 1 |
| 153 | LegacyPlanHandler | 0x1815 | v2_egowave | w | :488 =0 (Jungle / 라인 미니언 상태 비해당 / wave_concern<1) · :483 = roll<follow_own ? 2 : 1 | 4 | OK | 0 \| 2 \| 1 |
| 154 | LegacyPlanHandler | 0x180c | v2_egowave_line | w | :484 (:483 롤과 같은 경로) | 4 | OK | my_line(fallback_line 적용) |
| 155 | LegacyPlanHandler | 0x1806 | counter_jungle_route_init | w | 717 | 4 | OK | 1 |
| 156 | LegacyPlanHandler | 0x1610 | mf_swap.0 | w | mf_note_swap 인라인(handler.rs:410): 720(=28 카운터정글 초기 플랜) · 942(=26 plan_allowed 거부→ForcePassive) · 938(=20 next 채택) · 953(=mf_src, mf_swap.1!=tick 일 때만) \| (배치 C) L998 mf_note_swap 인라인(handler.rs:410) — mf_swap.1 != tick 일 때만 \| (배치 D) L1268 mf_note_swap(29, tick) 인라인 (24565~24566, handler.rs:410). 관측 전용 필드 | 4 | OK | 28 \| 26 \| 20 \| mf_src(passive_plan 반환 u8) |
| 157 | LegacyPlanHandler | 0x1618 | mf_swap.1 | w | 위 4곳과 짝 \| (배치 C) L998 · L1247 에서는 (25, tick) \| (배치 D) L1268 (24567~24568) | 4 | OK | game.tick() |
| 158 | LegacyPlanHandler | 0x5e8 | plan (BigPlan 384B) — ★HEAP | w | 6곳 전부 `drop_glue BigPlan(&self.plan)` 후 대입(교체 = 구 플랜 힙 drop). cleanuppad 경로도 같은 대입을 함(언와인딩 시에도 self.plan 이 유효 상태 유지) | 4 | OK | ①721 태그7 PassiveJungle{new_counter_jungle(rnd, team, strategy.focused)} 페이로드 104B@+0x5f0 ②943 태그2 ForcePassive ③939 next(Option<BigPlan> 페이로드 memcpy 384) ④955 mf_p(passive_plan 반환) ⑤force_plan_update:1744 before_plan 복원(lapse 롤백) ⑥:1753 r2_saved(Battle) 복원 |
| 159 | LegacyPlanHandler | 0x14b8 | last_jungle_lead_action_tick | w | 790 (정글러 && 현재 플랜 태그7 PassiveJungle) | 4 | OK | game.tick() |
| 160 | LegacyPlanHandler | 0x180d | active_gank_line | w | 803(LineGanker→Battle 전이) · 806(LineGanker→非Battle) · 909(Battle→非Battle 정리 끝) | 4 | OK | Some(plan@LineGanker.line) \| None(-1) |
| 161 | LegacyPlanHandler | 0x810 | gank_periods (Vec<(usize,usize)>) — ★HEAP | w | (start,end) 구간 장부. .1==0 = 열린 구간 | 4 | OK | last.1 = end_tick(811, 874) · push((tick, 0))(922, grow_one) |
| 162 | LegacyPlanHandler | 0x858 | pending_trace_events — ★HEAP | w | 833(LineGanker 종료) · 897(Battle 종료). GankResult 메모리태그 = -9223372036854775805(i64) = 9223372036854775811(u64). trace_level!=Off 일 때만 | 4 | OK | push(PendingTraceEvent{event: GankResult{kills,deaths,duration,line,success}, tick: end_tick}) |
| 163 | LegacyPlanHandler | 0x468 | team_plan.comeback_pick_success_count | w | 859 (objective==ComebackPick && got_kill) | 4 | OK | +=1 |
| 164 | LegacyPlanHandler | 0x1d8 | team_plan.comeback_pick_outcomes.last().1 (u8 @elem+4) | w | 860/863 — last.1==0(미기록)일 때만 | 4 | 부분일치(첫 성분(컨테이너)만 맞고 **마지막 성분이 다르다** — 사람이 봐야 ) | 1(킬) \| 6(무킬) |
| 165 | LegacyPlanHandler | 0x308 | team_plan.mf_obj_clear.0 | w | 867 mf_note_obj_clear 인라인(team_plan.rs:197) — objective∈{ComebackPick,Gank,Dive} 인 Battle 종료 | 4 | OK | 41 |
| 166 | LegacyPlanHandler | 0x310 | team_plan.mf_obj_clear.1 | w | 867 | 4 | OK | game.tick() |
| 167 | LegacyPlanHandler | 0x517 | team_plan.objective | w | L1192 | 4 | OK | None(-1)(868) \| Gank{line=next.line}(932: tag 8 @0x517, line @0x518) \| before_obj 복원 3B(force_plan_update:1743) |
| 168 | LegacyPlanHandler | 0x448 | team_plan.gank_start_tick | w | 933 (objective 를 Gank 로 세운 직후) | 4 | OK | game.tick() |
| 169 | LegacyPlanHandler | 0x14a0 | gank_attempt_count | w | 920 (next==LineGanker && tick-last_start > tps*10) | 4 | OK | +=1 |
| 170 | LegacyPlanHandler | 0x828 | gank_score_attempts (Vec<(i32,usize,usize)>) — ★HEAP | w | 927 grow_one. actual_score = evaluate_gank_opportunity_with_score(rnd,player,data,line,0).1(sret+8 i32) | 4 | OK | push((tick @+0, actual_score @+8, 0 @+16)) |
| 171 | LegacyPlanHandler | 0x15f8 | ff_battle_exit | w | force_plan_update:1758 — 진입 시 Battle 이었는데 update_objective 후 Battle 이 아니면(version<2 에서만 도달 가능: v2+ 는 r2_saved 로 Battle 복원됨). 22 = 「플랜 교체 이탈」 exit_src 코드(_docs:654 ff_battle_exit 발원) | 4 | OK | .0(0x1600)=22 · .1(0x1601)=plan@Battle.entry_src(선읽기) · .2(0x15f8)=game.tick() · .3(0x1602)=0 · .4(0x1603)=255 |
| 172 | LegacyPlanHandler | 0x1608 | ff_battle_exit_latch | w | force_plan_update:1759 | 4 | OK | i64 -1 (8바이트 전부 0xff) |
| 173 | LegacyPlanHandler | 0x548 | v3_dest (Option<(u64,u64)> 24B) | w | L982 (m13.ll:21768) · sret %168 | 4 | OK | v3_plan_dest(player.team, context, &new_plan) 결과 memcpy 24 |
| 174 | LegacyPlanHandler | 0x180e | v3_dest_obj (Option<MainObjective> 3B) | w | L983 (21770) | 4 | OK | obj(=배치B L970 에서 복사한 team_plan.objective %169) |
| 175 | LegacyPlanHandler | 0x5f0 | plan@PassiveLine.0 (구 플랜 페이로드) | w | L993 (21824) · new 쪽은 initializes((273,274)) = +0x111 v46_commit·+0x112 v46_flee | 4 | OK | PassiveLinePlan::v46_carry_over(&mut new.0, &mut old.0) — old 쪽 인자 captures(none)·readonly 없음 = &mut, 쓰기 내용은 콜리(m04.ll:18821) 몫 |
| 176 | LegacyPlanHandler | 0x5e8 | plan (BigPlan 384B) ★교체 ① | w | L1000 (21838~21857) · HEAP: 구 플랜 drop | 4 | OK | new_plan(%172) — memcpy →%167, drop_glue(old), memcpy 384 → self.plan |
| 177 | LegacyPlanHandler | 0x5e8 | plan (&mut) | w | L1010 (21906) · 내부 쓰기는 콜리 몫(자식 명세 없음 — m02.ll:6887 미탐색) | 4 | OK | BigPlan::update(&mut plan, version, rnd, player, data, &self.data, &mut self.team_plan, &self.positioning_score, debug) |
| 178 | LegacyPlanHandler | 0xf8 | team_plan (&mut TeamPlan 1064B) | w | 내부 쓰기 미탐색 | 4 | OK | BigPlan::update(L1010)·BigPlan::is_end(L1129)·BattlePlan::update(L1125/617) 에 &mut 전달 |
| 179 | LegacyPlanHandler | 0x7c8 | chats (Vec<Chat>) ★HEAP grow | w | L1016 (22003~22020) · 소스 = 플랜 페이로드 chats(태그별 0x608/0x5f0/0x658/0x6e0) | 4 | OK | extend(plan.chats().drain(..)) — Vec<Chat>::extend_trusted(Drain) |
| 180 | LegacyPlanHandler | 0x1550 | v48_claim_until_tick | w | L1025 — sub_plan 이 Battle 이고 v48_dodge_claim 일 때 | 4 | OK | max(self, sub_plan.Battle.v48_claim_hold_until) |
| 181 | LegacyPlanHandler | 0x1560 | v48_claim_window_ticks | w | L1031 (now <= claim_until) | 4 | OK | += 1 |
| 182 | LegacyPlanHandler | 0x1568 | v48_claim_window_hits | w | L1037 | 4 | OK | += d (nth - last) |
| 183 | LegacyPlanHandler | 0x1578 | v48_claim_hit_locked | w | L1043 action_state ∈ {Attack,Skill,Skill2,Ult} | 4 | OK | += d |
| 184 | LegacyPlanHandler | 0x1580 | v48_claim_hit_moving | w | L1044 action_state == Move | 4 | OK | += d |
| 185 | LegacyPlanHandler | 0x1588 | v48_claim_hit_idle | w | L1045 그 외(Idle/Return) | 4 | OK | += d |
| 186 | LegacyPlanHandler | 0x1570 | v48_other_hits | w | L1050 (now > claim_until) | 4 | OK | += d |
| 187 | LegacyPlanHandler | 0x1558 | v48_last_non_target_hit | w | L1053 (양 분기 공통, phi %2864/%2865) | 4 | OK | nth |
| 188 | LegacyPlanHandler | 0x1590 | v48_cast_cc | w | L1079 nearest.block_move() | 4 | OK | += 1 |
| 189 | LegacyPlanHandler | 0x1598 | v48_cast_locked | w | L1081 !(remain_action_time < windup) | 4 | OK | += 1 |
| 190 | LegacyPlanHandler | 0x15a0 | v48_cast_free_near | w | L1083 dist*2 < max(effect.range(champ),1) | 4 | OK | += 1 |
| 191 | LegacyPlanHandler | 0x15a8 | v48_cast_free_far | w | L1083 else | 4 | OK | += 1 |
| 192 | LegacyPlanHandler | 0x1814 | v48_last_cast_state | w | L1092 (22253) — Champion 이면 항상 저장 | 4 | OK | cur (0=없음,1 Skill,2 Skill2,3 Ult) |
| 193 | LegacyPlanHandler | 0x870 | v46_lane_recall_trigger_ticks (Vec<(usize,LineType)>) ★HEAP grow | w | L1100 (22380~22416) · 소스 Vec 0x650(ptr 0x658·len 0x660) | 4 | OK | append(&mut plan.PassiveLine.v46_pending.trigger_ticks) — reserve(len)+memcpy(len*16)+len+=; 소스 len=0 |
| 194 | LegacyPlanHandler | 0x14d0 | v46_lane_recall_danger_ticks … 0x1538 v46_lane_flee_nohit_episodes (14개 usize) | w | L1101~1114 순서: danger_ticks·veto_wave·veto_crash·veto_heal·stage2_saves·commit_clears·wave_enemy_half·wave_my_half·flee_triggers·flee_hold_ticks·flee_hold_acute·flee_hold_refuge·flee_hold_cover·flee_nohit_episodes | 4 | OK | += mem::take(plan.PassiveLine.v46_pending.<동명 카운터>) (0x680~0x6e8 → 0 으로) |
| 195 | LegacyPlanHandler | 0x680 | plan@PassiveLine.0.v46_pending 카운터 14 (0x680~0x6e8) | w | L1101~1114 | 4 | OK | 0 (take) |
| 196 | LegacyPlanHandler | 0x8a0 | v46_lane_flee_episodes (Vec<(usize,LineType,u8)>) ★HEAP grow | w | L1115 (22560~22589) | 4 | OK | append(&mut plan.PassiveLine.v46_pending.flee_episodes 0x668) |
| 197 | LegacyPlanHandler | 0x900 | v54_reentry_ticks (Vec<usize>) ★HEAP grow | w | L1119 (22612~22645) · 소스 len 0x680 → 0 | 4 | OK | append(&mut plan.Battle.v54_reentry_ticks 0x670) |
| 198 | LegacyPlanHandler | 0x0 | self 전체 (&mut) | w | L1121 (22638) · 내부 쓰기 미탐색(m13.ll:11163~11383) \| (배치 C) L1240·L1246 (및 L988) · 내부 쓰기는 자식 명세 몫 \| (배치 C) L1251(66: (false,0) · 118: (dive_abandoned, reason)) · 자식 명세 있음 — 계약만 | 4 | OK | sanitize_rule_scope(&mut self, cache, context) |
| 199 | LegacyPlanHandler | 0x5e8 | plan ★교체 ② (enter_line_backfight_support) | w | L1125(631) (22841·22936~22937) · HEAP: 구 플랜 drop · battle.chats 는 이동 전 len=0(618) | 4 | OK | drop_glue(old); tag=9(Battle); memcpy 280 battle → 0x5f0 |
| 200 | LegacyPlanHandler | 0x818 | gank_periods[len-1].1 (힙 원소, in-place) | w | L1139 (23921) · grow 없음 | 4 | OK | now |
| 201 | LegacyPlanHandler | 0x858 | pending_trace_events (Vec<PendingTraceEvent 184B>) ★HEAP grow | w | L1162 (24024~24042) · 태그 -9223372036854775805 = GankResult | 4 | OK | push(PendingTraceEvent{ event: GankResult{kills, deaths, duration, line, success}, tick: now }) |
| 202 | LegacyPlanHandler | 0x1470 | gank_cancel_tick | w | L1181 | 4 | OK | game.tick() |
| 203 | LegacyPlanHandler | 0x308 | team_plan.mf_obj_clear.0 / 0x310 .1 | w | L1191 mf_note_obj_clear 인라인(team_plan.rs:197) | 4 | OK | (42, tick) |
| 204 | LegacyPlanHandler | 0x1480 | last_dive_abandon_tick | w | L1205 (24140~24141 gep 5248) | 4 | OK | tick |
| 205 | LegacyPlanHandler | 0x1488 | last_response_bail_tick | w | L1212 (24162~24163 gep 5256) | 4 | OK | tick |
| 206 | LegacyPlanHandler | 0x320 | team_plan.last_resolver_bail_tick | w | L1216 exit_src==11 | 4 | OK | tick |
| 207 | LegacyPlanHandler | 0x1490 | last_lost_fight (.0 0x1490, .1 0x1498) | w | L1227 / L1231 | 4 | OK | (main_goal.0 또는 focus, tick) |
| 208 | LegacyPlanHandler | 0x15f8 | ff_battle_exit (.2 tick 0x15f8 · .0 exit_src 0x1600 · .1 entry_src 0x1601 · .3 exit_sub 0x1602 · .4 ff_wave_obs_open 0x1603) | w | L1235 (24233~24242) | 4 | OK | (b.exit_src, b.entry_src, tick, b.exit_sub, b.ff_wave_obs_open) |
| 209 | LegacyPlanHandler | 0x1608 | ff_battle_exit_latch (u8×8 0x1608~0x160f) | w | L1236~1237 (24259~24274) | 4 | OK | (ff_exit1_cls, ff_wave_open_hp, ff_wave_open_pct, ff_wave_fire_hp, ff_wave_fire_pct, ff_wave_open_danger, ff_wave_fire_danger, dive_abort_src) |
| 210 | LegacyPlanHandler | 0x1658 | mf_ret25_src[min(src,31)] | w | L1241 (24292~24298) | 4 | 오귀속(사전은 다른 필드를 준다) | += 1 |
| 211 | LegacyPlanHandler | 0x1610 | mf_swap | w | L1247 | 4 | OK | (25, tick) |
| 212 | LegacyPlanHandler | 0x5e8 | plan ★교체 ③ | w | L1248 (24316~24326) · HEAP | 4 | OK | passive_plan 결과(%155) — memcpy→%152, drop_glue(old), memcpy 384 |
| 213 | LegacyPlanHandler | 0x570 | v50_dive_ep_live 신규 Some (0x570~0x5e5 전 필드) | w | L1251(91~92) (23696~23733) | 4 | OK | V50DiveEpLive{prev_holder_hp: holder?Some(hp):None(tag 0x570·값 0x578), start_tick 0x580=tick, last_tick 0x588=tick, ep_ticks 0x590=1, in_range_ticks 0x598=in_range, holder_ticks 0x5a0=holder, team_holder_ticks 0x5a8=team_holder, minion_cover_ticks 0x5b0=in_range&&!tower_on_champ, soaked_hp 0x5b8=0, gap_ticks 0x5c0=0, max_catch_break 0x5c8=catch_break, uncatch_total 0x5d0=(catch_break!=0), target_pos 0x5d8, start_race_adv 0x5dc, start_in_range 0x5e0, tower 0x5e1, start_model 0x5e2, start_na 0x5e3, start_ne 0x5e4, start_tgt_hp 0x5e5} |
| 214 | LegacyPlanHandler | 0x5c0 | v50_dive_ep_live.gap_ticks | w | L1251 | 4 | OK | 0 (L70) / +=1 (L111) |
| 215 | LegacyPlanHandler | 0x5c8 | v50_dive_ep_live.max_catch_break | w | L1251(71) | 4 | OK | max(catch_break, old) |
| 216 | LegacyPlanHandler | 0x5d0 | v50_dive_ep_live.uncatch_total | w | L1251(72) | 4 | OK | += (catch_break != 0) |
| 217 | LegacyPlanHandler | 0x5e2 | v50_dive_ep_live.start_model / 0x5dc start_race_adv / 0x5e3 start_na / 0x5e4 start_ne | w | L1251(75~76) start_model==0 && model!=0 일 때만 | 4 | OK | snap 값 |
| 218 | LegacyPlanHandler | 0x588 | v50_dive_ep_live.last_tick | w | L1251(78·113) | 4 | OK | tick |
| 219 | LegacyPlanHandler | 0x590 | v50_dive_ep_live.ep_ticks | w | L1251(79·112) | 4 | OK | += 1 |
| 220 | LegacyPlanHandler | 0x598 | v50_dive_ep_live.in_range_ticks | w | L1251(80) | 4 | OK | += 1 (in_range) |
| 221 | LegacyPlanHandler | 0x5a8 | v50_dive_ep_live.team_holder_ticks | w | L1251(81) | 4 | OK | += 1 (team_holder) |
| 222 | LegacyPlanHandler | 0x5b0 | v50_dive_ep_live.minion_cover_ticks | w | L1251(82) | 4 | OK | += 1 (in_range && !tower_on_champ) |
| 223 | LegacyPlanHandler | 0x5a0 | v50_dive_ep_live.holder_ticks | w | L1251(84) | 4 | OK | += 1 (holder) |
| 224 | LegacyPlanHandler | 0x5b8 | v50_dive_ep_live.soaked_hp | w | L1251(85) prev_holder_hp 가 Some 일 때 | 4 | OK | += prev_holder_hp.saturating_sub(hp) |
| 225 | LegacyPlanHandler | 0x570 | v50_dive_ep_live.prev_holder_hp | w | L1251(83~88) | 4 | OK | Some(hp) (L86: tag 1 + 0x578=hp) / None (L88: tag 0) |
| 226 | LegacyPlanHandler | 0x1811 | v50_dive_ep_abort_src | w | L1251(117) | 4 | OK | plan 이 Battle 이면 b.dive_abort_src 아니면 0 |
| 227 | DebugFrameData | 0xa0 | infos HashMap ★HEAP(외부) | w | L1125(626~627) context.debug 일 때만 · self 아님 | 4 | OK | entry(champ.id).or_insert(Vec::new()).push(format!("!v26 line backfight support: line {:?}, ally {}, focus {}", line, ally_id, focus_id)) |
| 228 | LegacyPlanHandler | 0x1640 | v3_home_ladder[0] | w | L1265 (24421~24424) — version>1 && sub==Recall && v3_repair_done 통과 시 | 4 | OK | +=1 |
| 229 | LegacyPlanHandler | 0x5e8 | plan | w | L1269 (24571 drop_glue<BigPlan>(&self.plan) → 24580 memcpy 384). plan_allowed 통과 시. ★HEAP: 옛 plan 의 소유 힙(Battle.chats/v54_reentry_ticks 등) drop \| (배치 D) L1281 (24658 drop_glue<BigPlan>(&self.plan) → 24684 memcpy 384). plan_allowed 통과 시. ★HEAP: 옛 plan drop | 4 | OK | passive_plan() 결과(384B) |
| 230 | LegacyPlanHandler | 0x1648 | v3_home_ladder[1] | w | L1273 (24588~24591) — 패시브 플랜 불허 또는 교체 후에도 sub==Recall | 4 | OK | +=1 |
| 231 | LegacyPlanHandler | 0x1650 | v3_home_ladder[2] | w | L1285 (24669~24672 / 24691~24694) — PassiveLine 도 sub==Recall 이거나 불허 | 4 | OK | +=1 |
| 232 | LegacyPlanHandler | 0x768 | sub_plan | w | L1290 (24388) — 자식 명세 계약만: (ptr %2824=self+0x768, ptr %145=sub 72B). 내부 미독해 | 4 | OK | SubPlan::merge(&mut self.sub_plan, &sub) |
| 233 | LegacyPlanHandler | 0x858 | pending_trace_events | w | L1310 (25247~25277, grow_one 25257). trace_level!=Off && name!=prev 일 때. ★HEAP: grow_one · from/to String clone 2개 힙 \| (배치 D) L1388 (26040~26047 push_mut). plan==Battle 신규 진입 시 \| (배치 D) L1475 (26921~26988, grow_one 26969). ★HEAP: grow_one · disengage_reason 리터럴 String alloc(1450~1461) | 4 | OK | push(PendingTraceEvent{event: PlanTransition{reason, from: prev_plan_name.clone(), to: name.clone(), context: PlanContext{hp_ratio, ally_count, enemy_count, tick, gold_diff:0}}, tick}) |
| 234 | LegacyPlanHandler | 0x520 | battle_start_tick | w | L1323 (25325~25328) — name starts "Battle" && !prev starts "Battle" \| (배치 D) L1501 (26083~26084). ⚠블록 %4436 은 preds %4711(전투종료 뒤)·%4438(둘 다 Battle)·%4431(prev 비Battle) 전부 → name!=prev 블록의 모든 경로에서 실행(1323 의 Some 도 같은 호출 안에서 지워짐) | 4 | OK | Some(tick) (tag 1 @0x520, tick @0x528) |
| 235 | LegacyPlanHandler | 0x918 | battle_start_state | w | L1375 (25991 drop_glue<Option<BattleStartState>> 옛값 → 26016~26032 write). plan==Battle 일 때만. ★HEAP: 옛 String 2개 drop, 새 String 2개(engage_reason 1344~1357 리터럴 alloc, initial_sub_goal 1361 format!) \| (배치 D) L1407 take() (26114~26117 store -1). prev starts Battle && !name starts Battle | 4 | OK | Some(BattleStartState{engage_reason, initial_sub_goal, tick, my_hp(%), my_max_hp, target_id=focus, target_hp(%), target_max_hp, ally_count, enemy_count}) |
| 236 | LegacyPlanHandler | 0x840 | prev_plan_name | w | L1504 (24987 memcpy 24B 임시 → 24990/25004 옛 String 힙 drop → 27016 memcpy self). trace 켜진 경우 매 호출. ★HEAP: 옛 이름 버퍼 free | 4 | OK | name (get_name() String 이동) |
| 237 | LegacyPlanHandler | 0x15b0 | flee_prev_death_count | w | L1524 (27682) — SIM_STATS && !Jungle && line_phase && death 증가 | 4 | OK | death_count |
| 238 | LegacyPlanHandler | 0x8d0 | flee_death_retrospects | w | L1567 (28191~28319, grow_one 28201). 원소 16B: usize@+0 u8@+8 u8@+9 u8@+10. ★HEAP grow_one | 4 | OK | push((tick, label, plan_tag, gate)) |
| 239 | LegacyPlanHandler | 0x8c8 | flee_ring.len | w | L1574 (28326 / 28384) — 사망 회고 뒤 ring 비움(dealloc 없음) | 4 | OK | 0 (clear) |
| 240 | LegacyPlanHandler | 0x8b8 | flee_ring | w | L1617 (28004~28032, grow_one 28014). 원소 16B. ★HEAP grow_one \| (배치 D) L1619 (28039) — push 전 len > 127 이면 (ring 128 유지). memmove 만, dealloc 없음 | 4 | OK | push((tick, block, plan_tag_code, gate)) |

**`consts` 상수 105건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev | folded_from |
|---|---|---|---|---|---|---|
| 0 | 2 | 688 | 태그 | GameMode::DeathMatch 메모리태그(tcxdict --enum GameMode: idx2·discr2·태그2 · 인코딩 Direct). `%219 = icmp eq i64 %218, 2`(m13.ll:15214) 참 → L689 update_deathmatch 인라인. ★reach(version=2·gamemode=Moba) 접힘 @entry %219→0 = 사장 | 3 |  |
| 1 | 1 | 694 | 태그 | GameMode::SingleLane 메모리태그(tcxdict --enum GameMode: 태그1). `%678 = icmp eq i64 %677, 1`(m13.ll:16485) 참 → L695 update_single_lane 인라인. ★reach 접힘 @675 %678→0 = 사장. (qcspec 경고 사유: 이 1 은 `icmp eq` 의 비교값=열거형 태그이지 shl 시프트량이 아님 — folded_from 해당 없음) | 3 |  |
| 2 | 2 | 711 | 태그 | version < 2 → update_v2_egowave 생략(handler.rs:474). force_plan_update:1724 도 `version<2 → r2_saved None`. 또 BigPlan 태그 2=ForcePassive(943 store), 태그 산술 `tag-2`(chats/is_passive 스위치). shl 접힘 아님 | 4 |  |
| 3 | 1 | 711 | 태그 | Position::Jungle 메모리태그(player.info.position@0x9c0 ==1). 716/789/클로저 killer_position·killed_position 도 동일. update_v2_egowave:477~479 store i8 1(armed 3종). shl 접힘 아님 | 4 |  |
| 4 | 500 | 711 | 계수 | egowave_check(handler.rs:2269 정의, :2283/2285): wave_concern = 500 - roaming_ratio ; follow_own = wave_concern*ego_ratio/500 (sdiv) | 4 |  |
| 5 | -2000 | 711 | 임계 | egowave_check:2282 minion_state.from_mid < -2000 (내 라인 미니언 전선이 미드 기준 -2000 보다 밀림) | 4 |  |
| 6 | -2 | 711 | 임계 | egowave_check:2282 minion_state.minion_count < -2 (미니언 수 열세 3 이상) | 4 |  |
| 7 | 1000 | 711 | 계수 | update_v2_egowave:483 rnd.gen_range(0..1000) < follow_own → v2_egowave=2 else 1 | 4 |  |
| 8 | 0 | 711 | 태그 | gen_range 하한 · v2_egowave=0(:488) · ff_battle_exit.3=0 · gank_periods push (tick,0) · gank_score_attempts (.,.,0) · evaluate_gank_opportunity_with_score 마지막 인자 0 | 4 |  |
| 9 | 7 | 716 | 태그 | rule_scope player_count(context) 인라인: `(tutorial-1) <u 7` ⇔ tutorial∈{First..Line}(1..=7) → 인원 부족 → 카운터정글 초기화 생략. 또 BigPlan 태그 7 = PassiveJungle(721 store / 789 비교). shl 접힘 아님 | 4 |  |
| 10 | 28 | 720 | 산출값 | mf_swap src 코드 — 카운터정글 초기 플랜 배치 | 4 |  |
| 11 | 26 | 942 | 산출값 | mf_swap src 코드 — next 가 plan_allowed 거부 → ForcePassive | 4 |  |
| 12 | 20 | 938 | 산출값 | mf_swap src 코드 — next 채택 | 4 |  |
| 13 | 41 | 867 | 산출값 | team_plan.mf_obj_clear src 코드 — Battle 종료로 ComebackPick/Gank/Dive 목표 해제 | 4 |  |
| 14 | 22 | 947 | 산출값 | force_plan_update:1758 ff_battle_exit.0 = 22 (플랜 교체 이탈 exit_src) | 4 |  |
| 15 | -1 | 769 | 센티널 | 니치 None: Option<BigPlan>(769 next) · Option<MainObjective>@0x517/0x180e(None=0xff) · Option<LineType> active_gank_line · ProfTimer 없음(i32 -1) · ff_battle_exit_latch 전비트 | 4 |  |
| 16 | 10 | 775 | 태그 | tps*10 = 10초. 775 `tick-gank_cancel_tick > tps*10` 이어야 LineGanker(WaitResponse) 전이 허용 · 919 `tick-last_gank_start > tps*10` 이어야 새 갱크 시도로 계수. 또 BigPlan 태그 10=LineGanker(772/797/916) | 4 |  |
| 17 | 6 | 772 | 태그 | LineGankerPhase::WaitResponse 메모리태그(niche_start 6, idx 0). 또 comeback_pick_outcomes.last().1 = 6(863 무킬 결과코드) · BigPlan 태그 6 은 무효(assume ne) · TutorialType JungleOnly=6. shl 접힘 아님 | 4 |  |
| 18 | 9 | 799 | 태그 | BigPlan 태그 9 = Battle (799/850/851/force_plan_update:1719·1752·1757) · MainObjective 태그 9 = Dive(866) | 4 |  |
| 19 | 11 | 853 | 태그 | MainObjective 태그 11 = ComebackPick | 4 |  |
| 20 | 8 | 866 | 태그 | MainObjective 태그 8 = Gank(866 매치 / 932 store) · TutorialType Total=8 | 4 |  |
| 21 | -9223372036854775805 | 833 | 태그 | TraceEventType::GankResult 메모리태그(u64 9223372036854775811 = niche_start 2^63 + idx 3) | 4 |  |
| 22 | 132 | 763 | 임계 | prof PHASE_NANOS/PHASE_CALLS 배열 길이(bounds check) — 계측 전용 | 4 |  |
| 23 | 77 | 698 | 산출값 | ProfTimer phase id _t_head (계측 전용, 판정 무관) | 4 |  |
| 24 | 57 | 702 | 산출값 | ProfTimer phase id (team_plan.update) | 4 |  |
| 25 | 58 | 707 | 산출값 | ProfTimer phase id (goal_data.update) | 4 |  |
| 26 | 59 | 764 | 산출값 | ProfTimer phase id _t_np (next_plan) | 4 |  |
| 27 | 78 | 768 | 산출값 | ProfTimer phase id _t_tail | 4 |  |
| 28 | 79 | 947 | 산출값 | ProfTimer phase id (force_plan_update: handle_interact_battle) | 4 |  |
| 29 | 80 | 947 | 산출값 | ProfTimer phase id (force_plan_update: update_objective) | 4 |  |
| 30 | 1000000000 | 698 | 임계 | Duration::as_nanos 변환(계측 전용) | 4 |  |
| 31 | -7 | 770 | 임계 | rule_scope player_count 인라인(goal_allowed:95, Jungle 목표): `(tutorial-8) <u -7 \|\| tutorial==6` ⇔ tutorial∈{None,JungleOnly,Total} → 정글 목표 허용 | 4 |  |
| 32 | -6 | 770 | 임계 | rule_scope morgard_exists→TutorialType::spawn_epic 인라인: `(tutorial-7) <u -6` ⇔ tutorial∈{None,Line,Total} | 4 |  |
| 33 | -8 | 770 | 미상 | player_count 인라인 산술(위 -7 항목의 감산값) | 4 |  |
| 34 | 4096000001 | 978 | 임계 | 64000²+1 — v3 목적지 도착 판정: dist_sq < 64000²+1 ⟺ 거리 ≤ 64000(=2셀). arrived (21741) | 4 |  |
| 35 | 2 | 977 | 임계 | player.info.team < 2 bounds check(panic_bounds_check) — L977·1039·1059 및 aux | 4 |  |
| 36 | 1 | 979 | 태그 | closure$9(BigPlan 논리 idx) = {1 PassiveLine, 2 SinglePlanLine, 5 PassiveJungle} — switch case (21706~21708·21755~21757). 메모리 태그 3·4·7 (shl 시프트량 아님 — 태그/열거/카운트 값) | 4 |  |
| 37 | 5 | 979 | 태그 | 위 closure$9 의 idx 5 = PassiveJungle | 4 |  |
| 38 | 3 | 991 | 태그 | BigPlan 메모리 태그 3 = PassiveLine (idx 1). L991·1098·1125(599)·1129 에서 eq (shl 시프트량 아님 — 태그/열거/카운트 값) | 4 |  |
| 39 | 6 | 1005 | 센티널 | llvm.assume(tag != 6) — BigPlan 니치 인코딩상 6 은 불가 값(untagged DeathMatchBattle 자리). 판정 아님 | 4 |  |
| 40 | 7 | 1005 | 임계 | BigPlan 논리 idx 7 = Battle → prof 페이즈 68 / idx 1(PassiveLine) → 69 / 그 외 70 (21882~21885) | 4 |  |
| 41 | 68 | 1005 | 산출값 | ProfTimer 페이즈 코드(Battle 플랜 update). 이름 미확인(PHASE_NANOS[132] 인덱스) | 4 |  |
| 42 | 69 | 1005 | 산출값 | ProfTimer 페이즈 코드(PassiveLine 플랜 update) | 4 |  |
| 43 | 70 | 1005 | 산출값 | ProfTimer 페이즈 코드(그 외 플랜 update) | 4 |  |
| 44 | 76 | 1124 | 산출값 | ProfTimer 페이즈 코드(enter_line_backfight_support 구간) | 4 |  |
| 45 | 74 | 1128 | 산출값 | ProfTimer 페이즈 코드(is_passive/is_end 구간) | 4 |  |
| 46 | 132 | 1013 | 임계 | PHASE_NANOS/PHASE_CALLS 배열 길이(bounds check) — 판정 아님 | 4 |  |
| 47 | 1000000000 | 1013 | 임계 | Duration::as_nanos (초×1e9) — 프로파일링 전용 | 4 |  |
| 48 | 8 | 1023 | 센티널 | llvm.assume(sub_plan tag != 8) — SubPlan 니치 불가 값. 판정 아님 | 4 |  |
| 49 | 7 | 1023 | 태그 | SubPlan 메모리 태그 7 = Battle (idx 5) — v48 claim 창 연장 조건 | 4 |  |
| 50 | 13 | 1040 | 태그 | EntityType 태그 13 = Champion (L1040·1060·1251/55) | 4 |  |
| 51 | 2 | 1041 | 태그 | ChampionActionState 태그 2 = Move → v48_claim_hit_moving | 4 |  |
| 52 | 3 | 1041 | 태그 | ChampionActionState 3 Attack·4 Skill·5 Skill2·6 Ult → v48_claim_hit_locked (shl 시프트량 아님 — 태그/열거/카운트 값) | 4 |  |
| 53 | 4 | 1061 | 태그 | ChampionActionState 4 Skill→cur=1 · 5 Skill2→cur=2 · 6 Ult→cur=3 (L1061~1067) (shl 시프트량 아님 — 태그/열거/카운트 값) | 4 |  |
| 54 | -1 | 1069 | 센티널 | Option<Effect> 니치 None (casting@tag == -1) — L1069~1071·L1251(50)·v50_dive_ep_live tag | 4 |  |
| 55 | 1 | 1074 | 태그 | CastingType is_nontarget: (casting-1) <u 2 ⟺ casting ∈ {1 Position, 2 Direction} (22282~22283) (shl 시프트량 아님 — 태그/열거/카운트 값) | 4 |  |
| 56 | 1 | 1083 | 임계 | `shl i64 %dist, 1` = dist*2 < max(range,1) → v48_cast_free_near (근접 판정 = 사거리의 절반 이내). shl 로 접힘 (22355) | 4 | 2 |
| 57 | 1 | 1083 | 태그 | max(effect.range(champ), 1) 하한 (umax) (shl 시프트량 아님 — 태그/열거/카운트 값) | 4 |  |
| 58 | 9 | 1118 | 태그 | BigPlan 메모리 태그 9 = Battle (idx 7) — L1118·1200·1251(39·115)·L631 store | 4 |  |
| 59 | 576460752303423488 | 1100 | 임계 | Vec<(usize,LineType)> len 상한 assume(isize::MAX/16) — 판정 아님 | 4 |  |
| 60 | 1152921504606846976 | 1119 | 임계 | Vec<usize> len 상한 assume — 판정 아님 | 4 |  |
| 61 | 6 | 1125 | 태그 | handler.rs:614 battle.entry_src = 6 (라인 백파이트 지원 진입 사유 코드) (22751) | 4 |  |
| 62 | 4 | 1125 | 태그 | handler.rs:620 battle.sub_goal 태그 4 RunAway · 7 End → 진입 취소(battle drop) (22773~22775) (shl 시프트량 아님 — 태그/열거/카운트 값) | 4 |  |
| 63 | 10 | 1134 | 태그 | BigPlan 메모리 태그 10 = LineGanker (idx 8) — L1134·1179 | 4 |  |
| 64 | 1 | 1133 | 태그 | Position 태그 1 = Jungle (player.info.position != 1 이면 갱크 결과 트레이스 생략) · aux 킬로그 술어의 killer/killed_position==1 (shl 시프트량 아님 — 태그/열거/카운트 값) | 4 |  |
| 65 | -9223372036854775805 | 1162 | 센티널 | TraceEventType 니치 태그 9223372036854775811 = GankResult (idx 3) (24031) | 4 |  |
| 66 | -9 | 1189 | 미상 | (tag-9) <u 2 ⟺ tag ∈ {9 Battle, 10 LineGanker} | 4 |  |
| 67 | -2 | 1190 | 임계 | team_plan.objective tag & !1 == 8 ⟺ tag ∈ {8 Gank, 9 Dive} (MainObjective) | 4 |  |
| 68 | 8 | 1190 | 태그 | MainObjective 태그 8 Gank / 9 Dive — 플랜 종료 시 팀 목표 해제 대상 | 4 |  |
| 69 | 42 | 1191 | 산출값 | team_plan.mf_note_obj_clear(src=42, tick) — 목표 해제 사유 코드 | 4 |  |
| 70 | 2 | 1211 | 태그 | BattlePlanGoal 태그 2 = Response (main_goal) — last_response_bail_tick 조건 | 4 |  |
| 71 | -11 | 1211 | 계수 | (exit_src-11) <u 4 ⟺ exit_src ∈ {11,12,13,14} (battle.rs [ff 계측] 코드표 — 본 범위 밖) | 4 |  |
| 72 | 11 | 1215 | 임계 | exit_src == 11 → team_plan.last_resolver_bail_tick = tick | 4 |  |
| 73 | 2 | 1225 | 임계 | (exit_src-11) <u 2 ⟺ exit_src ∈ {11,12} → last_lost_fight 갱신 | 4 |  |
| 74 | 25 | 1247 | 산출값 | mf_note_swap(src=25) — _docs: 25=교체불명(플랜 종료 후 passive_plan 교체 경로) | 4 |  |
| 75 | 31 | 1241 | 임계 | mf_ret25_src[min(src,31)] — 32칸 배열 상한 클램프 | 4 |  |
| 76 | 100 | 1251 | 계수 | dive_episode.rs:58 tgt_hp_pct = hp*100/max(max_hp,1) · entity.rs:1515 radius*(mult+100)/100 | 4 |  |
| 77 | 255 | 1251 | 인덱스 | dive_episode.rs:58 tgt_hp_pct 를 u8 로 클램프 (umin 255) | 4 |  |
| 78 | 2 | 1251 | 태그 | dive_episode.rs:53 EntityType 태그 2 = Tower (타워 표적 nearest_enemy 읽기 조건) | 4 |  |
| 79 | 4 | 1251 | 태그 | dive_episode.rs:110 reason ∈ {4 (focus None), 6 (target 이 적 챔피언 아님/플레이어 없음)} 이면 gap_ticks < tps 동안 에피소드 유지 (shl 시프트량 아님 — 태그/열거/카운트 값) | 4 |  |
| 80 | 8 | 1251 | 임계 | closure$0 reason 코드: 8 = 내 챔피언 없음 · 1 = 플랜이 Battle 아님 · 2 = !with_dive · 3 = dive_tower None · 4 = focus None · 5 = 표적 엔티티 없음 · 6 = 표적이 적 Player 팀 아님/player_by_champion_id None · 7 = 같은 종류 타워 없음/타워 attack_effect None (23531·23538) | 4 |  |
| 81 | 75 | 1253 | 산출값 | ProfTimer::start 페이즈 인덱스(prof.rs:179 store i64 75). 1291 drop 에서 PHASE_NANOS[75]/PHASE_CALLS[75](=+600) 누적. 계측 전용 | 4 |  |
| 82 | 1 | 1263 | 임계 | `version > 1`(icmp ugt %3916, 1). reach.py version=2 접기로 항상 true — 비교 피연산자(icmp ugt), shl 시프트량 아님 | 4 |  |
| 83 | 5 | 1263 | 태그 | SubPlan 메모리태그 5 = Recall(tcxdict --enum SubPlan). 1263/1272/1284/1599/1656 동일 | 3 |  |
| 84 | 8 | 1263 | 센티널 | `llvm.assume(tag != 8)` — SubPlan 니치 빈칸(DeathBattle 암묵 태그 자리). 판정 아님 | 4 |  |
| 85 | 29 | 1268 | 산출값 | mf_note_swap src 코드 29 (self.mf_swap.0 = 29). 관측 전용 — 코드표 이름은 찾지 않음(지시) | 4 |  |
| 86 | 2 | 1274 | 태그 | LineType 태그 2 = Bottom (position Bottom(3)\|Support(4) → Bottom). 0=Top, 1=Mid 도 같은 switch | 4 |  |
| 87 | 3 | 1279 | 태그 | BigPlan 메모리태그 3 = PassiveLine(store i64 3 @%147). PassiveLinePlan::new(line): line@+0x116 → plan+0x11e — 태그 리터럴(store), shl 시프트량 아님(본문의 shl 3 은 슬라이스 stride 8B 계산) | 4 |  |
| 88 | 4 | 1286 | 태그 | SubPlan 메모리태그 4 = LineWait(line) (store i64 4 @%145, line u8 @+8) — 태그 리터럴(store), shl 시프트량 아님(본문의 shl 4 는 ring 원소 stride 16B 계산) | 4 |  |
| 89 | 9 | 1325 | 태그 | BigPlan 메모리태그 9 = Battle. 1325/1601 동일. 1298 의 9 는 PlanTransitionReason::Other | 4 |  |
| 90 | 6 | 1325 | 센티널 | `assume(plan tag != 6)` BigPlan 니치 빈칸(DeathMatchBattle 자리). 1298 의 6 = PlanTransitionReason::LowHp, 1575/1630 의 6 = tick%6 주기 | 4 |  |
| 91 | -9223372036854775808 | 1310 | 센티널 | TraceEventType 니치 태그 i64::MIN = PlanTransition(tcxdict --enum TraceEventType idx0) | 3 |  |
| 92 | 100 | 1303 | 계수 | hp*100/max_hp (%). 1303/1332/1339/1413/1420/1643/1655 · 1425/1428 역산 hp_start*max/100 | 4 |  |
| 93 | 7812735363465179764 | 1349 | 산출값 | 8바이트 리터럴 "try_kill"(LE i64 store) — TryKill 기본 engage_reason | 4 |  |
| 94 | -9223372036854775804 | 1388 | 태그 | TraceEventType 태그 i64::MIN+4 = EngageDecision | 4 |  |
| 95 | 30 | 1457 | 임계 | 내 hp% < 30 → disengage_reason "low_hp_retreat". (1521 의 30 은 tps*30 = 30초) | 4 |  |
| 96 | 1684367716 | 1453 | 산출값 | 4바이트 리터럴 "died"(LE i32 store) — deaths>0 일 때 disengage_reason | 4 |  |
| 97 | 132 | 1672 | 임계 | ProfTimer drop 의 PHASE_NANOS 배열 길이 bounds check(_t_tail %191, 배치 A 의 L768 타이머 drop 이 여기) | 4 |  |
| 98 | 6 | 1575 | 태그 | game.tick() % 6 == 0 — SIM_STATS ring 기록 주기 / 1630 DIEWIN 로그 주기 | 4 |  |
| 99 | 600 | 1597 | 임계 | mdt(check_kill_die_tick) < 600 틱 → block bit0(위험) | 4 |  |
| 100 | 10 | 1531 | 임계 | tps*10 = 사망 회고 창(마지막 ring 틱 − 10초) | 4 |  |
| 101 | 127 | 1618 | 임계 | flee_ring push 전 len > 127 → remove(0): ring 최대 128 | 4 |  |
| 102 | 1000 | 1643 | 계수 | DIEWIN 로그 nrst = distance/1000 (k 단위 표기). 디버그 전용 | 4 |  |
| 103 | 22500000001 | 1580 | 미상 | 150000^2+1 — aux 클로저(1366/1371/1580/1652)의 distance_sq < 150000² (150k 반경). 본문엔 없고 aux 에 있음 | 4 |  |
| 104 | 12 | 1601 | 태그 | BigPlan 태그 12~15 = EpicHuntAndPoke/EpicHuntAndBattle/SerpenHuntAndPoke/SerpenHuntAndBattle → plan_tag_code 3. 1298 의 12 는 "ActiveRecall" 길이 | 4 |  |

**`knobs` 조정점 19건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 에고웨이브 판정의 미니언 전선 임계 | egowave_check:2282 (m13.ll:19359) | -2000 | 값을 올리면(예 -1000) 전선이 덜 밀려도 에고웨이브 후보가 되어 v2_egowave 롤이 더 자주 일어난다 | 4 | 기존 |
| 1 | 에고웨이브 판정의 미니언 수 열세 임계 | egowave_check:2282 (m13.ll:19365) | -2 | 올리면(예 0) 열세가 작아도 후보 | 4 | 기존 |
| 2 | wave_concern 기준값·follow_own 분모 | egowave_check:2283/2285 (m13.ll:19369, 19400) | 500 | follow_own = (500-roaming)*ego/500 : 기준값을 올리면 로밍 성향이 높은 선수도 라인 집착(2) 확률↑ | 4 | 기존 |
| 3 | 에고웨이브 롤 범위 | update_v2_egowave:483 (m13.ll:19394) | 1000 | gen_range(0..1000)<follow_own — 범위를 줄이면 2(라인 집착) 확률↑. ⚠rnd 스트림 소비 1회라 다른 롤에 연쇄 | 4 | 기존 |
| 4 | 갱크 취소 후 재전이 쿨다운·갱크 시도 계수 간격 | handler.rs:775·919 (m13.ll:20641, 21335) | 10 | tps*10(10초). 줄이면 LineGanker(WaitResponse) 재진입이 빨라지고 gank_attempt_count 가 더 잦게 계수된다 | 4 | 기존 |
| 5 | mf_swap 발원 코드 | handler.rs:720/938/942 (m13.ll:19461, 21468, 20606) | 28/20/26 | 관측 전용(텔레메트리) — 판정에 영향 없음 | 4 | 기존 |
| 6 | comeback_pick 결과 코드 | handler.rs:860/863 (m13.ll:21100, 21080) | 1/6 | 관측 전용 | 4 | 기존 |
| 7 | v3 목적지 도착 반경 | handler.rs:978 (m13.ll:21741) | 4096000001 | 64000²+1. 올리면 더 멀리서도 '도착'으로 보고 새 소극 플랜으로 교체를 허용, 내리면 목적지까지 더 가까이 가야 교체됨(그 전엔 구 소극 플랜 유지) | 4 | 기존 |
| 8 | v48 근접/원거리 시전 구분 | handler.rs:1083 (m13.ll:22355~22357) | dist*2 < max(range,1) | 계측 전용(v48_cast_free_near/far 카운터). 판단에 영향 없음 | 4 | 기존 |
| 9 | 라인 백파이트 진입 취소 조건 | handler.rs:620 (m13.ll:22773) | sub_goal ∈ {RunAway, End} | 여기 케이스를 줄이면 update 직후 이미 도망/종료 상태인 BattlePlan 으로도 교체됨 | 4 | 기존 |
| 10 | 백파이트 진입 entry_src 코드 | handler.rs:614 | 6 | 계측 코드(ff_battle_exit.1 로 나중에 기록). 판단 영향 없음 | 4 | 기존 |
| 11 | 다이브 에피소드 공백 허용 | dive_episode.rs:110 (m13.ll:23834) | gap_ticks < tick_per_second (=1초) | 올리면 focus 부재(4)/표적 비적(6) 상태가 더 오래 이어져도 에피소드가 접히지 않음. 계측 전용 | 4 | 기존 |
| 12 | 플랜 종료 시 팀 목표 해제 대상 | handler.rs:1190 | objective ∈ {Gank, Dive} | Battle/LineGanker 종료 시 이 목표만 None 으로 해제. 대상을 넓히면 다른 팀 목표도 종료마다 초기화됨(판단 영향 있음) | 4 | 기존 |
| 13 | v3 귀환→패시브 사다리 진입 조건: 서브플랜 Recall + 챔피언 만피 | handler.rs:1263~1264 (m13.ll 24383~24418) | hp >= stat_cached.hp | 완충(hp>=max) 이어야만 passive_plan 재선정. 비교를 완화하면 귀환 직후 더 일찍 라인 복귀 | 4 | 기존 |
| 14 | 패시브 플랜 불허/재귀환 시 폴백 라인 | handler.rs:1274 (24592~24611) | Top→Top, Bottom\|Support→Bottom, 그 외→Mid 후 fallback_line | 정글/미드가 미드로 떨어짐. 매핑을 바꾸면 v3 사다리 2단의 라인 배정이 바뀜 | 4 | 기존 |
| 15 | SIM_STATS ring 위험 임계 | handler.rs:1597 (27864) | 600 | 계측 라벨(block bit0)만 바뀜 — 판단에 영향 없음(관측 전용) | 4 | 기존 |
| 16 | flee_ring 크기 | handler.rs:1618 (28034) | 127 | 계측 전용 | 4 | 기존 |
| 17 | 사망 회고 창 | handler.rs:1531 (28132) | tps*10 | 계측 전용 | 4 | 기존 |
| 18 | 라인 페이즈 정의(계측 게이트) | setting.rs:703 인라인 @ handler.rs:1521 (27657~27666) | tick < first_spawn_tick − tps*30 | 계측 전용 | 4 | 기존 |

<details><summary>`callees` 피호출자 133건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | add_log | game_core::DebugFrameData::add_log | pub | fn(&mut game_core::DebugFrameData, &game_core::OperationData, &game_core::PlayerState, std::string::String) | game-core\src\simulation\game\frame.rs:54 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | battle_engage_dir | game_ai::plan_legacy::types::BigPlan::battle_engage_dir | pub | fn(&game_ai::plan_legacy::types::BigPlan) -> std::option::Option<i8> | game-ai\src\plan_legacy\types.rs:154 | True | True | 3 | IR 호출 심볼 일치(토큰 경계) |
| 2 | block_move | game_core::Entity::block_move | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1497 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 3 | chat_allowed | game_ai::plan_legacy::rule_scope::chat_allowed | pub | fn(&game_core::GameContext, &game_core::Chat) -> bool | game-ai\src\plan_legacy\rule_scope.rs:128 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | chats | game_ai::plan_legacy::types::BigPlan::chats | pub | fn(&mut game_ai::plan_legacy::types::BigPlan) -> std::option::Option<&mut std::vec::Vec<game_core::Chat, std::alloc::Global>> | game-ai\src\plan_legacy\types.rs:184 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 5 | check_kill_die_tick | game_ai::check_kill_die_tick | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::OperationData, &game_core::PlayerState, &game_core::Entity, bumpalo::collections::vec::Vec< &game_core::Entity>, bumpalo::collections::vec::Vec< &game_core::Entity>, &mut game_core::DebugFrameData) -> usize | game-ai\src\fight_check.rs:917 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 6 | clear | game_core::DataTable::<T>::clear | pub | fn(&mut game_core::DataTable<T/#0>) | game-core\src\data.rs:2259 | True | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 7 | contains | game_core::RectU64::contains | pub | fn(&game_core::RectU64, u64, u64) -> bool | game-core\src\setting.rs:40 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 8 | contains | game_core::RectI64::contains | pub | fn(&game_core::RectI64, i64, i64) -> bool | game-core\src\setting.rs:55 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 9 | contains | game_core::setting::champion::nightmare::NightmareWellRect::contains | in:game_core::setting::champion::nightmare | fn(game_core::setting::champion::nightmare::NightmareWellRect, u64, u64) -> bool | game-core\src\setting\champion\nightmare.rs:28 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 10 | debug | game_core::AbstractBanpickRunner::debug | pub | fn(&Self/#0, &[usize]) | game-core\src\banpick\base_runner.rs:411 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 11 | debug | <game_core::FactoredBanpickAgent as game_core::AbstractBanpickRunner>::debug | pub | fn(&game_core::FactoredBanpickAgent, &[usize]) | game-core\src\banpick\sgd_v2.rs:5891 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 12 | debug | <game_core::LogisticSGDBanpickAgent as game_core::AbstractBanpickRunner>::debug | pub | fn(&game_core::LogisticSGDBanpickAgent, &[usize]) | game-core\src\banpick\sgd.rs:1172 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 13 | determine_transition_reason | game_ai::plan_legacy::handler::LegacyPlanHandler::determine_transition_reason | in:game_ai::plan_legacy::handler | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, &str) -> game_core::PlanTransitionReason | game-ai\src\plan_legacy\handler.rs:1675 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 14 | distance | game_core::Entity::distance | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2161 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 15 | distance_sq | game_core::utils::distance_sq | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:6 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 16 | distance_sq | game_core::Entity::distance_sq | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2157 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 17 | ego_ratio | game_core::AthleteParameter::ego_ratio | pub | fn(&game_core::AthleteParameter) -> usize | game-core\src\simulation\state\player.rs:439 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 18 | egowave_check | game_ai::plan_legacy::handler::egowave_check | in:game_ai::plan_legacy::handler | fn(&game_core::PlayerState, &game_core::OperationData) -> std::option::Option<(game_core::LineType, i32)> | game-ai\src\plan_legacy\handler.rs:2269 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 19 | evaluate_gank_opportunity_with_score | game_ai::plan_legacy::old::evaluate_gank_opportunity_with_score | pub | fn(&mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, game_core::LineType, i32) -> (bool, i32, i32) | game-ai\src\plan_legacy\old\passive_jungle.rs:693 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 20 | fallback_line | game_ai::plan_legacy::rule_scope::fallback_line | pub | fn(&game_core::GameContext, game_core::LineType) -> game_core::LineType | game-ai\src\plan_legacy\rule_scope.rs:28 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 21 | focus | game_ai::plan_legacy::old::BattleSubPlanGoal::focus | pub | fn(&game_ai::plan_legacy::old::BattleSubPlanGoal) -> std::option::Option<usize> | game-ai\src\plan_legacy\old\battle.rs:29 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 22 | force_plan_update | game_ai::plan_legacy::handler::LegacyPlanHandler::force_plan_update | in:game_ai::plan_legacy::handler | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData, bool) | game-ai\src\plan_legacy\handler.rs:1709 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 23 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 24 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 25 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 26 | get_game_mode | game_core::AbstractGame::get_game_mode | pub | fn(&Self/#0) -> game_core::GameMode | game-core\src\simulation.rs:78 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 27 | get_game_mode | <game_core::Game as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::Game) -> game_core::GameMode | game-core\src\simulation\game.rs:1707 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 28 | get_game_mode | <game_core::SingleLaneGame as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::SingleLaneGame) -> game_core::GameMode | game-core\src\simulation\game.rs:4248 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 29 | get_name | game_ai::plan_legacy::types::BigPlan::get_name | pub | fn(&game_ai::plan_legacy::types::BigPlan) -> std::string::String | game-ai\src\plan_legacy\types.rs:44 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 30 | goal | game_ai::plan_legacy::types::BigPlan::goal | pub | fn(&game_ai::plan_legacy::types::BigPlan) -> game_core::BigGoal | game-ai\src\plan_legacy\types.rs:132 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 31 | goal_allowed | game_ai::plan_legacy::rule_scope::goal_allowed | pub | fn(&game_core::GameContext, game_core::BigGoal) -> bool | game-ai\src\plan_legacy\rule_scope.rs:90 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 32 | growth | game_core::ChampionInfo::growth | pub | fn(&Self/#0) -> game_core::EntityStat | game-core\src\setting.rs:1083 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 63개 중 상위 3개 |
| 33 | growth | <game_core::DataChampionInfo as game_core::ChampionInfo>::growth | pub | fn(&game_core::DataChampionInfo) -> game_core::EntityStat | game-core\src\setting\champion\data_driven.rs:2573 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 63개 중 상위 3개 |
| 34 | growth | <game_core::MonkChampionInfo as game_core::ChampionInfo>::growth | pub | fn(&game_core::MonkChampionInfo) -> game_core::EntityStat | game-core\src\setting\champion\monk.rs:62 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 63개 중 상위 3개 |
| 35 | handle_chat | game_ai::plan_legacy::handler::chat::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::handle_chat | pub | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, game_core::Position, game_core::Chat, bool, &mut game_core::DebugFrameData) | game-ai\src\plan_legacy\handler\chat.rs:8 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 36 | handle_interact_battle | game_ai::plan_legacy::handler::engage::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::handle_interact_battle | in:game_ai | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) | game-ai\src\plan_legacy\handler\engage.rs:233 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 37 | is_cancel | game_ai::plan_legacy::types::BigPlan::is_cancel | pub | fn(&game_ai::plan_legacy::types::BigPlan) -> bool | game-ai\src\plan_legacy\types.rs:37 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 38 | is_empty | game_core::PatchSetting::is_empty | pub | fn(&game_core::PatchSetting) -> bool | game-core\src\patch_engine.rs:853 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 39 | is_empty | game_core::ModAiRegistry::is_empty | pub | fn(&game_core::ModAiRegistry) -> bool | game-core\src\mod_ai.rs:15 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 40 | is_empty | game_core::PublicChampionEvidence::is_empty | pub | fn(&game_core::PublicChampionEvidence) -> bool | game-core\src\banpick\public_evidence.rs:96 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 41 | is_end | game_ai::plan_legacy::types::BigPlan::is_end | pub | fn(&game_ai::plan_legacy::types::BigPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bool | game-ai\src\plan_legacy\types.rs:276 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 42 | is_ignored_battle_enemy | game_ai::plan_legacy::old::is_ignored_battle_enemy | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, bool) -> bool | game-ai\src\plan_legacy\old\fight_model.rs:887 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 43 | is_ignored_well_enemy | game_ai::plan_legacy::old::is_ignored_well_enemy | pub | fn(usize, &game_core::PlayerState, &game_core::Entity) -> bool | game-ai\src\plan_legacy\old\fight_model.rs:754 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 44 | is_in_range | game_core::Effect::is_in_range | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\effect.rs:63 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 45 | is_line_phase | game_core::GameSetting::is_line_phase | pub | fn(&game_core::GameSetting, usize) -> bool | game-core\src\setting.rs:702 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 46 | is_line_phase | game_core::GameContext::<'a, 'b>::is_line_phase | pub | fn(&game_core::GameContext<'a/#0, 'b/#1>, usize) -> bool | game-core\src\simulation\game\runner.rs:397 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 47 | is_nontarget | game_core::CastingType::is_nontarget | pub | fn(&game_core::CastingType) -> bool | game-core\src\simulation\effect\type.rs:148 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 48 | is_passive | game_ai::plan_legacy::types::BigPlan::is_passive | pub | fn(&game_ai::plan_legacy::types::BigPlan) -> bool | game-ai\src\plan_legacy\types.rs:253 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 49 | is_visible_from | game_core::Entity::is_visible_from | pub | fn(&game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1481 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 50 | iter_champions | game_core::AbstractGameWithCache::<'a, 'b>::iter_champions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5759 ~ game_core[6a30]::simulation::{impl#5}::iter_champions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1904 | True | True | 3 | IR 호출 심볼 일치(토큰 경계) |
| 51 | iter_towers_without_nexus | game_core::AbstractGameWithCache::<'a, 'b>::iter_towers_without_nexus | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5750 ~ game_core[6a30]::simulation::{impl#5}::iter_towers_without_nexus::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1830 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 52 | kill_logs | game_core::AbstractGame::kill_logs | pub | fn(&Self/#0) -> &std::vec::Vec<game_core::KillLog, std::alloc::Global> | game-core\src\simulation.rs:141 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 53 | kill_logs | <game_core::Game as game_core::AbstractGame>::kill_logs | pub | fn(&game_core::Game) -> &std::vec::Vec<game_core::KillLog, std::alloc::Global> | game-core\src\simulation\game.rs:3768 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 54 | kill_logs | <game_core::SingleLaneGame as game_core::AbstractGame>::kill_logs | pub | fn(&game_core::SingleLaneGame) -> &std::vec::Vec<game_core::KillLog, std::alloc::Global> | game-core\src\simulation\game.rs:4027 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 55 | level | <game_ai::WindowStatView<'_> as game_core::AbstractEntity>::level | pub | fn(&game_ai::WindowStatView</#0>) -> usize | game-ai\src\fight_check.rs:96 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 56 | level | game_core::AbstractEntity::level | pub | fn(&Self/#0) -> usize | game-core\src\simulation\entity.rs:718 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 57 | level | <game_core::Entity as game_core::AbstractEntity>::level | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:772 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 58 | line | game_ai::plan_legacy::types::BigPlan::debug_label::line | in:game_ai::plan_legacy::types | fn(game_core::LineType) -> &str | game-ai\src\plan_legacy\types.rs:83 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 59 | line | game_core::TowerType::line | pub | fn(&game_core::TowerType) -> std::option::Option<game_core::LineType> | game-core\src\simulation\entity\tower.rs:90 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 60 | line_backfight_support_focus | game_ai::line_backfight_support_focus | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, game_core::LineType) -> std::option::Option<(usize, usize)> | game-ai\src\utils.rs:269 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 61 | line_exists | game_ai::plan_legacy::rule_scope::line_exists | pub | fn(&game_core::GameContext, game_core::LineType) -> bool | game-ai\src\plan_legacy\rule_scope.rs:24 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 62 | merge | game_ai::plan_legacy::sub_plan::SubPlan::merge | pub | fn(&mut game_ai::plan_legacy::sub_plan::SubPlan, game_ai::plan_legacy::sub_plan::SubPlan) | game-ai\src\plan_legacy\sub_plan\mod.rs:186 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 63 | mf_note_obj_clear | game_ai::plan_legacy::team_plan::TeamPlan::mf_note_obj_clear | in:game_ai | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, u8, usize) | game-ai\src\plan_legacy\team_plan.rs:196 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 64 | mf_note_swap | game_ai::plan_legacy::handler::LegacyPlanHandler::mf_note_swap | in:game_ai | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, u8, usize) | game-ai\src\plan_legacy\handler.rs:409 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 65 | nearest_enemy | game_core::Entity::nearest_enemy | pub | fn(&game_core::Entity) -> std::option::Option<usize> | game-core\src\simulation\entity.rs:1819 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 66 | new | game_ai::plan_legacy::old::BattlePlan::new | pub | fn(usize, game_ai::plan_legacy::old::BattlePlanGoal, &game_core::OperationData, &game_core::PlayerState) -> game_ai::plan_legacy::old::BattlePlan | game-ai\src\plan_legacy\old\battle.rs:196 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 67 | new | game_ai::plan_legacy::old::DeathMatchBattle::new | pub | fn(usize, game_ai::plan_legacy::old::BattlePlanGoal, &game_core::OperationData, &game_core::PlayerState) -> game_ai::plan_legacy::old::DeathMatchBattle | game-ai\src\plan_legacy\old\death_battle.rs:746 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 68 | new_counter_jungle | game_ai::plan_legacy::old::PassiveJunglePlan::new_counter_jungle | pub | fn(&mut rand::rngs::std::StdRng, usize, game_core::FocusedAreaStrategy) -> game_ai::plan_legacy::old::PassiveJunglePlan | game-ai\src\plan_legacy\old\passive_jungle.rs:63 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 69 | next_plan | game_ai::plan_legacy::types::BigPlan::next_plan | pub | fn(&mut game_ai::plan_legacy::types::BigPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &game_core::PositioningScoreData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> std::option::Option<game_ai::plan_legacy::types::BigPlan> | game-ai\src\plan_legacy\types.rs:297 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 70 | passive_plan | game_ai::plan_legacy::handler::LegacyPlanHandler::passive_plan | in:game_ai::plan_legacy::handler | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> (game_ai::plan_legacy::types::BigPlan, u8) | game-ai\src\plan_legacy\handler.rs:1855 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 71 | plan_allowed | game_ai::plan_legacy::rule_scope::plan_allowed | pub | fn(&game_core::GameContext, &game_ai::plan_legacy::types::BigPlan) -> bool | game-ai\src\plan_legacy\rule_scope.rs:100 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 72 | player_by_champion_id | game_core::AbstractGameWithCache::<'a, 'b>::player_by_champion_id | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> std::option::Option<&'a/#0 game_core::PlayerState> | game-core\src\simulation.rs:1915 | True | True | 3 | IR 호출 심볼 일치(토큰 경계) |
| 73 | player_count | game_core::GameRule::player_count | pub | fn(&game_core::GameRule) -> usize | game-core\src\data\match_info.rs:542 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 74 | player_count | game_core::TutorialType::player_count | pub | fn(&game_core::TutorialType) -> usize | game-core\src\simulation\game\runner.rs:294 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 75 | player_team | game_core::TeamType::player_team | pub | fn(&game_core::TeamType) -> std::option::Option<usize> | game-core\src\simulation\entity.rs:1135 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 76 | player_team | game_view::ClientData::player_team | pub | fn(&game_view::ClientData) -> std::option::Option<std::cell::Ref< game_core::Team>> | game-view\src\logic\client\data.rs:1579 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 77 | player_team | game_view::ClientDatabase::player_team | pub | fn(&game_view::ClientDatabase) -> &game_core::Team | game-view\src\logic\client\data.rs:1163 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 78 | position | game_core::PlayerAiContext::<'a, 'b, 'r>::position | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> game_core::Position | game-core\src\mod_ai.rs:587 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 79 | position | game_core::transfer::TeamRosterPlanningContext::position | pub | fn(&game_core::transfer::TeamRosterPlanningContext, game_core::Position) -> std::option::Option<&game_core::transfer::PositionPlanningContext> | game-core\src\transfer\roster_context.rs:234 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 80 | push | <game_core::Staff as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::staff | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\staff.rs:38 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 81 | push | <game_core::Athlete as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:434 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 82 | push | <game_core::Contract as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:2498 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 83 | radius | game_core::Entity::radius | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1509 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 84 | remain_action_time | game_core::Entity::remain_action_time | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1616 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 85 | roaming_ratio | game_core::AthleteParameter::roaming_ratio | pub | fn(&game_core::AthleteParameter) -> usize | game-core\src\simulation\state\player.rs:425 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 86 | sanitize_rule_scope | game_ai::plan_legacy::handler::LegacyPlanHandler::sanitize_rule_scope | in:game_ai::plan_legacy::handler | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, &game_core::OperationData) | game-ai\src\plan_legacy\handler.rs:571 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 87 | set_main_objective | game_ai::plan_legacy::old::BattlePlan::set_main_objective | pub | fn(&mut game_ai::plan_legacy::old::BattlePlan, std::option::Option<game_ai::plan_legacy::team_plan::MainObjective>) | game-ai\src\plan_legacy\old\battle.rs:349 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 88 | set_main_objective | game_ai::plan_legacy::old::SinglePlanBattle::set_main_objective | pub | fn(&mut game_ai::plan_legacy::old::SinglePlanBattle, std::option::Option<game_ai::plan_legacy::team_plan::MainObjective>) | game-ai\src\plan_legacy\old\single_battle.rs:85 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 89 | set_main_objective | game_ai::plan_legacy::old::DeathMatchBattle::set_main_objective | pub | fn(&mut game_ai::plan_legacy::old::DeathMatchBattle, std::option::Option<game_ai::plan_legacy::team_plan::MainObjective>) | game-ai\src\plan_legacy\old\death_battle.rs:867 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 90 | skill2_effect | game_core::Entity::skill2_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1692 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 91 | spawn_line_minion | game_core::TutorialType::spawn_line_minion | pub | fn(&game_core::TutorialType, game_core::LineType) -> bool | game-core\src\simulation\game\runner.rs:274 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 92 | start | game_core::prof::start | pub | fn(usize) -> std::option::Option<game_core::prof::ProfTimer> | game-core\src\simulation\prof.rs:175 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 93 | start | game_view::UIPhaseEffect::start | pub | fn(&mut game_view::UIPhaseEffect) | game-view\src\ui\match_ui\phase_effect.rs:30 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 94 | strategy | game_core::PlayerState::strategy | pub | fn(&game_core::PlayerState, &mut rand::rngs::std::StdRng, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + ) -> game_core::Strategy | game-core\src\simulation\state\player.rs:1576 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 95 | sub_goal | game_ai::plan_legacy::old::BattlePlan::sub_goal | pub | fn(&game_ai::plan_legacy::old::BattlePlan) -> game_ai::plan_legacy::old::BattleSubPlanGoal | game-ai\src\plan_legacy\old\battle.rs:178 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 96 | sub_goal | game_ai::plan_legacy::old::SinglePlanBattle::sub_goal | pub | fn(&game_ai::plan_legacy::old::SinglePlanBattle) -> game_ai::plan_legacy::old::BattleSubPlanGoal | game-ai\src\plan_legacy\old\single_battle.rs:27 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 97 | sub_goal | game_ai::plan_legacy::old::DeathMatchBattle::sub_goal | pub | fn(&game_ai::plan_legacy::old::DeathMatchBattle) -> game_ai::plan_legacy::old::BattleSubPlanGoal | game-ai\src\plan_legacy\old\death_battle.rs:114 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 98 | sub_plan | game_ai::plan_legacy::types::BigPlan::sub_plan | pub | fn(&mut game_ai::plan_legacy::types::BigPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PositioningScoreData, &mut game_core::DebugFrameData) -> game_ai::plan_legacy::sub_plan::SubPlan | game-ai\src\plan_legacy\types.rs:230 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 99 | take_misunderstood_received_chat | game_ai::plan_legacy::handler::LegacyPlanHandler::take_misunderstood_received_chat | in:game_ai::plan_legacy::handler | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, game_core::Position, game_core::Chat) -> bool | game-ai\src\plan_legacy\handler.rs:588 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 100 | target_id | game_ai::SmallActionTrace::target_id | pub | fn(&game_ai::SmallActionTrace) -> usize | game-ai\src\small_action\trace.rs:427 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 101 | target_id | game_view::view::effect::alchemist::target_id | in:game_view::view::effect::alchemist | fn(game_core::InputTarget) -> std::option::Option<usize> | game-view\src\view\effect\alchemist.rs:126 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 102 | target_id | game_view::view::projectile::crossbowman::target_id | in:game_view::view::projectile::crossbowman | fn(game_core::InputTarget) -> std::option::Option<usize> | game-view\src\view\projectile\crossbowman.rs:1878 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 103 | team | game_view::ClientData::team | pub | fn(&game_view::ClientData, usize) -> std::option::Option<std::cell::Ref< game_core::Team>> | game-view\src\logic\client\data.rs:1651 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 104 | team | game_view::ClientDatabase::team | pub | fn(&game_view::ClientDatabase, usize) -> std::option::Option<&game_core::Team> | game-view\src\logic\client\data.rs:968 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 105 | team | game_core::PlayerAiContext::<'a, 'b, 'r>::team | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:583 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 106 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 107 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 108 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 109 | tick_per_second | game_view::view::effect::nightmare::tick_per_second | in:game_view::view::effect::nightmare | fn(&engine_core::assets::Assets) -> f32 | game-view\src\view\effect\nightmare.rs:31 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 110 | tower | game_core::AbstractGameWithCache::<'a, 'b>::tower | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, game_core::LineType, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:1822 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 111 | tutorial | game_core::Strategy::tutorial | pub | fn() -> game_core::Strategy | game-core\src\simulation\strategy.rs:590 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 112 | ult_effect | game_core::Entity::ult_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1700 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 113 | update | game_ai::GoalData::update | pub | fn(&mut game_ai::GoalData, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) | game-ai\src\goal_data.rs:82 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 114 | update | game_ai::plan_legacy::types::BigPlan::update | pub | fn(&mut game_ai::plan_legacy::types::BigPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PositioningScoreData, &mut game_core::DebugFrameData) | game-ai\src\plan_legacy\types.rs:198 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 115 | update | game_ai::plan_legacy::old::BattlePlan::update | pub | fn(&mut game_ai::plan_legacy::old::BattlePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) | game-ai\src\plan_legacy\old\battle.rs:427 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 116 | update | game_ai::plan_legacy::team_plan::TeamPlan::update | pub | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) | game-ai\src\plan_legacy\team_plan.rs:294 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 117 | update | game_ai::plan_legacy::handler::LegacyPlanHandler::update | pub | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData, bool) | game-ai\src\plan_legacy\handler.rs:685 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 118 | update_deathmatch | game_ai::plan_legacy::handler::modes::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::update_deathmatch | in:game_ai | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) | game-ai\src\plan_legacy\handler\modes.rs:56 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 119 | update_objective | game_ai::plan_legacy::team_plan::TeamPlan::update_objective | pub | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData) | game-ai\src\plan_legacy\team_plan.rs:722 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 120 | update_single_lane | game_ai::plan_legacy::handler::modes::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::update_single_lane | in:game_ai | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) | game-ai\src\plan_legacy\handler\modes.rs:13 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 121 | update_v2_egowave | game_ai::plan_legacy::handler::LegacyPlanHandler::update_v2_egowave | in:game_ai::plan_legacy::handler | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) | game-ai\src\plan_legacy\handler.rs:473 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 122 | v2_apply_assign_commit | game_ai::plan_legacy::handler::LegacyPlanHandler::v2_apply_assign_commit | in:game_ai::plan_legacy::handler | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, &mut u8, &mut game_core::DebugFrameData) | game-ai\src\plan_legacy\handler.rs:518 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 123 | v3_plan_dest | game_ai::plan_legacy::handler::LegacyPlanHandler::v3_plan_dest | in:game_ai::plan_legacy::handler | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::types::BigPlan) -> std::option::Option<(u64, u64)> | game-ai\src\plan_legacy\handler.rs:1824 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 124 | v3_repair_done | game_ai::plan_legacy::handler::LegacyPlanHandler::v3_repair_done | in:game_ai | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bool | game-ai\src\plan_legacy\handler.rs:1847 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 125 | v46_carry_over | game_ai::plan_legacy::old::PassiveLinePlan::v46_carry_over | pub | fn(&mut game_ai::plan_legacy::old::PassiveLinePlan, &mut game_ai::plan_legacy::old::PassiveLinePlan) | game-ai\src\plan_legacy\old\passive_line.rs:307 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 126 | v46_flee_gate_check | game_ai::plan_legacy::old::passive_line::v46_flee_gate_check | in:game_ai | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity) -> (u8, bumpalo::collections::vec::Vec< usize>) | game-ai\src\plan_legacy\old\passive_line.rs:38 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 127 | v50_fold_dive_episode | game_ai::plan_legacy::handler::dive_episode::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::v50_fold_dive_episode | in:game_ai | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, usize, bool, u8) | game-ai\src\plan_legacy\handler\dive_episode.rs:125 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 128 | v50_track_dive_episode | game_ai::plan_legacy::handler::dive_episode::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::v50_track_dive_episode | in:game_ai | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &game_core::PlayerState, &game_core::OperationData) | game-ai\src\plan_legacy\handler\dive_episode.rs:35 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 129 | valid_lines | game_ai::plan_legacy::rule_scope::valid_lines | pub | fn(&game_core::GameContext) -> &[game_core::LineType] | game-ai\src\plan_legacy\rule_scope.rs:13 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 130 | version | <game_ai::AgentVerHamster as game_core::AiAgent>::version | pub | fn(&game_ai::AgentVerHamster) -> usize | game-ai\src\lib.rs:428 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 592개 중 상위 3개 |
| 131 | version | game_core::AiAgent::version | pub | fn(&Self/#0) -> usize | game-core\src\simulation\ai_interface.rs:503 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 592개 중 상위 3개 |
| 132 | version | <game_core::Team as engine_core::spitz::SpitzDatable>::version | pub | fn() -> usize | game-core\src\data\team.rs:1696 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 592개 중 상위 3개 |
</details>

⚠**미매칭 108개**: `append`, `assist`, `casting`, `cleanuppad`, `data`, `death`, `define`, `dive_abandoned`, `dive_abort_src`, `dive_catch_break_ticks`, `dive_entered`, `dive_last_race_adv`, `dive_tower`, `drain`, `drop  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `drop_glue<BigPlan>`, `drop_glue<Option<ProfTimer>>`, `drop_glue<Vec<`, `elapsed`, `enemies`, `entry`, `entry_src`, `exit_src`, `exit_sub`, `extend`, `extend_trusted`, `ff_battle_exit`, `ff_battle_exit_latch`, `ff_exit1_cls`, `ff_wave_fire_danger`, `ff_wave_fire_hp`, `ff_wave_fire_pct`, `ff_wave_obs_open`, `ff_wave_open_danger`, `ff_wave_open_hp`, `ff_wave_open_pct`, `first_spawn_tick`, `flee_episodes`, `flee_prev_death_count`, `format_inner`, `gank_cancel_tick`, `gank_periods`, `gen_range`, `grow_one`, `handle_error`, `infos`, `insert_no_grow`, `is_contained_in`, `killed_position`, `killer_position`, `killer_team`, `last`, `last_dive_abandon_tick`, `last_lost_fight`, `last_mut`, `last_resolver_bail_tick`, `last_response_bail_tick`, `llvm.usub.sat.i64`, `locked`, `main_goal`, `map_or`, `memcmp`, `mf_ret25_src`, `move_speed`, `my_hp`, `my_max_hp`, `non_target_hit`, `now)`, `objective`, `or_insert`, `pending_trace_events`, `plan`, `positioning_score`, `prev_plan_name`, `push_mut`, `range  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `remove  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `reserve`, `retain`, `rustc_entry`, `sg_0`, `slice_contains`, `start_timing`, `starts_with`, `support_target`, `take`, `target_hp`, `target_max_hp`, `team_plan`, `tower_attack_disable_tick`, `tower_attack_disable_tick_2v2`, `trace`, `trace_level`, `trigger_ticks`, `truncate`, `try_allocate_in`, `unwrap_or_default`, `v46_flee`, `v46_lane_flee_episodes`, `v46_lane_recall_trigger_ticks`, `v48_claim_hold_until`, `v48_dodge_claim`, `v48_last_cast_state`, `v50_dive_ep_abort_src`, `v50_dive_ep_live`, `v54_reentry_ticks`, `with_dive`, `writes`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 2곳** (m14.ll:34452, m14.ll:34471) · **형제 41개** (LegacyPlanHandler)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::plan_legacy::handler::LegacyPlanHandler as std::clone::Clone>::clone | pub | game-ai\src\plan_legacy\handler.rs:13 | True | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler) -> game_ai::plan_legacy::handler::LegacyPlanHandler |
| 1 | <game_ai::plan_legacy::handler::LegacyPlanHandler as std::fmt::Debug>::fmt | pub | game-ai\src\plan_legacy\handler.rs:13 | True | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 2 | game_ai::plan_legacy::handler::LegacyPlanHandler::new | pub | game-ai\src\plan_legacy\handler.rs:228 | False | fn(usize, &mut rand::rngs::std::StdRng, usize, game_core::Position) -> game_ai::plan_legacy::handler::LegacyPlanHandler |
| 3 | game_ai::plan_legacy::handler::LegacyPlanHandler::r2_fight_protected | in:game_ai | game-ai\src\plan_legacy\handler.rs:362 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &game_core::PlayerState, &game_core::OperationData) -> bool |
| 4 | game_ai::plan_legacy::handler::LegacyPlanHandler::ff_note_battle_swap | in:game_ai | game-ai\src\plan_legacy\handler.rs:400 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, u8, usize) |
| 5 | game_ai::plan_legacy::handler::LegacyPlanHandler::mf_note_swap | in:game_ai | game-ai\src\plan_legacy\handler.rs:409 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, u8, usize) |
| 6 | game_ai::plan_legacy::handler::LegacyPlanHandler::v3_fall_back_to_passive | pub | game-ai\src\plan_legacy\handler.rs:416 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 7 | game_ai::plan_legacy::handler::LegacyPlanHandler::eo_cover_picks | pub | game-ai\src\plan_legacy\handler.rs:440 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler) -> usize |
| 8 | game_ai::plan_legacy::handler::LegacyPlanHandler::eo_serpen_punish_issues | pub | game-ai\src\plan_legacy\handler.rs:445 | True | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler) -> usize |
| 9 | game_ai::plan_legacy::handler::LegacyPlanHandler::subplan_is_recall | pub | game-ai\src\plan_legacy\handler.rs:450 | True | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler) -> bool |
| 10 | game_ai::plan_legacy::handler::LegacyPlanHandler::team_objective_code | pub | game-ai\src\plan_legacy\handler.rs:456 | True | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler) -> u8 |
| 11 | game_ai::plan_legacy::handler::LegacyPlanHandler::update_v2_egowave | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:473 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) |
| 12 | game_ai::plan_legacy::handler::LegacyPlanHandler::v2_obj_restore_safe | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:501 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> bool |
| 13 | game_ai::plan_legacy::handler::LegacyPlanHandler::v2_apply_assign_commit | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:518 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, &mut u8, &mut game_core::DebugFrameData) |
| 14 | game_ai::plan_legacy::handler::LegacyPlanHandler::sanitize_rule_scope | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:571 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, &game_core::OperationData) |
| 15 | game_ai::plan_legacy::handler::LegacyPlanHandler::take_misunderstood_received_chat | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:588 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, game_core::Position, game_core::Chat) -> bool |
| 16 | game_ai::plan_legacy::handler::LegacyPlanHandler::enter_line_backfight_support | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:598 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> bool |
| 17 | game_ai::plan_legacy::handler::LegacyPlanHandler::update_on_dead | pub | game-ai\src\plan_legacy\handler.rs:635 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 18 | game_ai::plan_legacy::handler::LegacyPlanHandler::update | pub | game-ai\src\plan_legacy\handler.rs:685 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData, bool) |
| 19 | game_ai::plan_legacy::handler::LegacyPlanHandler::determine_transition_reason | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:1675 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, &str) -> game_core::PlanTransitionReason |
| 20 | game_ai::plan_legacy::handler::LegacyPlanHandler::force_plan_update | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:1709 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData, bool) |
| 21 | game_ai::plan_legacy::handler::LegacyPlanHandler::v3_assign_anchor | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:1773 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, &game_core::PlayerState, &game_core::OperationData) -> std::option::Option<(u64, u64)> |
| 22 | game_ai::plan_legacy::handler::LegacyPlanHandler::v3_depart_anchor | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:1806 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, &game_core::PlayerState, &game_core::OperationData) -> std::option::Option<(u64, u64)> |
| 23 | game_ai::plan_legacy::handler::LegacyPlanHandler::v3_plan_dest | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:1824 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::types::BigPlan) -> std::option::Option<(u64, u64)> |
| 24 | game_ai::plan_legacy::handler::LegacyPlanHandler::v3_repair_done | in:game_ai | game-ai\src\plan_legacy\handler.rs:1847 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bool |
| 25 | game_ai::plan_legacy::handler::LegacyPlanHandler::passive_plan | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:1855 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> (game_ai::plan_legacy::types::BigPlan, u8) |
| 26 | game_ai::plan_legacy::handler::auction::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::get_small_action | pub | game-ai\src\plan_legacy\handler\auction.rs:8 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &std::vec::Vec<(usize, game_core::SmallAction), std::alloc::Global>, &mut game_core::DebugFrameData) -> (game_ai::ScoreParameter, i64, game_ai::SmallActionPlay) |
| 27 | game_ai::plan_legacy::handler::chat::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::handle_chat | pub | game-ai\src\plan_legacy\handler\chat.rs:8 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, game_core::Position, game_core::Chat, bool, &mut game_core::DebugFrameData) |
| 28 | game_ai::plan_legacy::handler::chat::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::handle_chat_inner | in:game_ai | game-ai\src\plan_legacy\handler\chat.rs:41 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, game_core::Position, game_core::Chat, bool, &mut game_core::DebugFrameData) |
| 29 | game_ai::plan_legacy::handler::dive_episode::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::v50_track_dive_episode | in:game_ai | game-ai\src\plan_legacy\handler\dive_episode.rs:35 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &game_core::PlayerState, &game_core::OperationData) |
| 30 | game_ai::plan_legacy::handler::dive_episode::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::v50_fold_dive_episode | in:game_ai | game-ai\src\plan_legacy\handler\dive_episode.rs:125 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, usize, bool, u8) |
| 31 | game_ai::plan_legacy::handler::engage::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::v2_response_retreat_stance | in:game_ai | game-ai\src\plan_legacy\handler\engage.rs:13 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> game_ai::plan_legacy::old::BattleSubPlanGoal |
| 32 | game_ai::plan_legacy::handler::engage::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::try_engage | in:game_ai | game-ai\src\plan_legacy\handler\engage.rs:40 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, usize, &mut game_core::DebugFrameData) -> std::option::Option<game_ai::plan_legacy::old::BattlePlan> |
| 33 | game_ai::plan_legacy::handler::engage::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::try_engage_dive | in:game_ai | game-ai\src\plan_legacy\handler\engage.rs:109 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, usize, std::option::Option<game_core::TowerType>, &mut game_core::DebugFrameData) -> std::option::Option<game_ai::plan_legacy::old::BattlePlan> |
| 34 | game_ai::plan_legacy::handler::engage::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::handle_solokill | in:game_ai | game-ai\src\plan_legacy\handler\engage.rs:136 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 35 | game_ai::plan_legacy::handler::engage::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::handle_interact_battle | in:game_ai | game-ai\src\plan_legacy\handler\engage.rs:233 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 36 | game_ai::plan_legacy::handler::modes::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::update_single_lane | in:game_ai | game-ai\src\plan_legacy\handler\modes.rs:13 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 37 | game_ai::plan_legacy::handler::modes::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::update_deathmatch | in:game_ai | game-ai\src\plan_legacy\handler\modes.rs:56 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 38 | game_ai::plan_legacy::handler::modes::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::single_lane_initiate | in:game_ai | game-ai\src\plan_legacy\handler\modes.rs:196 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 39 | game_ai::plan_legacy::handler::modes::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::single_try_engage | in:game_ai | game-ai\src\plan_legacy\handler\modes.rs:241 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, usize, &mut game_core::DebugFrameData) -> std::option::Option<game_ai::plan_legacy::old::SinglePlanBattle> |
| 40 | game_ai::plan_legacy::handler::modes::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::single_handle_solokill | in:game_ai | game-ai\src\plan_legacy\handler\modes.rs:261 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |

**`open` 34건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 재료 부재 | (배치 A) L689 update_deathmatch·L695 update_single_lane 내부(합 1,577 IR 줄)는 NA 봉인으로 **미독해**(적용 범위: gamemode=Moba 상수 접기 기준 사장. DeathMatch/SingleLane 모드에서는 살아있으므로 그 모드를 다룰 땐 별도 독해 필요 — 「원리적 불가」가 아니라 미탐색). 골격만: 689 = sanitize_rule_scope(L57)→…→TeamPlan::update/GoalData::update(L130~132)→dm_bounded_reach/dm_ally_engaged→BigPlan::update(L159)→DeathMatchBattle::new(L151/154)→decide_deathmatch(L165)→sub_plan/SubPlan::merge(L184)→sanitize_rule_scope(L189); 695 = sanitize_rule_scope(L14)→TeamPlan::update/GoalData::update(L16~18)→(L26 인라인: check_kill·max_range_cached·champions·can_near_enemies_range·single_try_engage)→(L29 인라인: is_ignored_well_enemy·aggressive_ratio·check_favorable_engage_formation)→BigPlan::update(L32)→is_end(L37)→sub_plan/merge(L43)→sanitize_rule_scope(L49). | 4 |  |
| 1 | 미탐색 | (배치 A) 페이즈게이트(objective=min(C*A/1000,100)·threshold=objective*9+min(B,100)*2+100·roll<threshold→handle_interact_battle)는 배치 A 의 살아있는 범위에 없음 — handle_interact_battle 호출은 L947(배치 B) 에 있고 reach 상 live=1 dead=0. 재확인은 배치 B 몫. | 4 |  |
| 2 | 미탐색 | (배치 A) L685 소스 줄 길이 148자(rmeta_srcmap) vs 표준 서식 추정 149자 — 1자 차이의 원인(타입 표기 등) 미확정. 인자 이름·타입은 DI(!23525~!23531)로 확정이므로 명세에는 영향 없음. | 3 |  |
| 3 | 미탐색 | (배치 A) calls 가 빈 이유: 살아있는 범위의 유일한 호출 get_game_mode 는 vtable 간접호출(`call {i64,ptr} %216`)이라 심볼 호출이 아님. 사장 689/695 의 직접 호출 46건은 규칙대로 미등재(reach 사장 호출부 절 그대로). | 4 |  |
| 4 | 미탐색 | (배치 A) aux 31개 조각은 전부 update 소유(심볼에 LegacyPlanHandler::update 클로저 성분)이나 호출 지점은 모두 배치 B~E(handler.rs 735~1650) — 배치 A 범위에서 쓰는 조각은 없음. note 의 종류 표기는 심볼 성분에서 읽은 것이고 내부는 미독해. | 4 |  |
| 5 | 미탐색 | (배치 B) PlayerState::strategy 의 3·4번째 인자(game 팻포인터)가 `&dyn AbstractGame` 인지 — vtable 슬롯 +0x28 을 tick 으로 호출하는 같은 (%212,%214) 쌍이라 그렇게 읽었다(추정: 형식 시그니처는 _gcbc 미확인) | 5 |  |
| 6 | 미탐색 | (배치 B) Option<BigPlan> None 의 니치 값 -1 — BigPlan 은 태그 2..17 니치인데 IR 이 -1 과 비교(769/force_plan_update:1724·1751). 값의 근거는 IR 뿐(tcxdict 는 Option<BigPlan> 을 안 줌) | 3 |  |
| 7 | 미탐색 | (배치 B) force_plan_update:1763 goal(sret %31) 의 소비처 — 배치 C 범위(948 이후 어딘가). 본 범위에선 생성만 | 4 |  |
| 8 | 미탐색 | (배치 B) 976 의 dx,dy(v3_dest .0/.1) 소비 — 배치 C(977) 로 넘어감 | 4 |  |
| 9 | 미탐색 | (배치 B) _t_tail(768 시작) 의 drop 위치 — 배치 C/D | 4 |  |
| 10 | 미탐색 | (배치 B) GankResult 트레이스의 `line`(816: LineGanker.line / 879: fallback_line(active_gank_line.unwrap_or(Mid))) — 879 쪽 fallback_line 은 2인자 아웃오브라인 call 이라 default 인자가 없다(2280 의 인라인판은 3인자). 두 판의 관계는 미확인 | 4 |  |
| 11 | 미탐색 | (배치 B) NA(사장) 코드: reach 의 사장 호출부 100건 중 루트가 696~976 인 것은 0건(전부 689/695 루트 또는 배치 C/D 절대줄) — 본 범위에 NA 봉인 대상 없음. na_e4c5c0.json 의 10 함수도 본 범위에서 호출 없음 | 4 |  |
| 12 | 미탐색 | (배치 B) 1866~1873(730 루프)의 chat.clone() 은 24B memcpy 로 접혀 Chat 변형별 힙 소유 여부(String 페이로드?)를 판별 못 함 — Chat 57 variants 미조사 | 4 |  |
| 13 | 미탐색 | (배치 C) v3_plan_dest(m13.ll:9990~10109, internal fastcc, sret 24 Option<(u64,u64)>, 인자 (team, &GameContext, &BigPlan)) 내부 — 미탐색(자식 명세 목록에 없음). 본 범위는 결과를 self.v3_dest 에 저장할 뿐 | 4 |  |
| 14 | 미탐색 | (배치 C) sanitize_rule_scope(m13.ll:11163~11383, (&mut self, &cache, &context)) 내부 — 미탐색. self 쓰기 가능(&mut) | 4 |  |
| 15 | 미탐색 | (배치 C) utils::line_backfight_support_focus(m04.ll:53979, sret 24 Option<(ally_id, focus_id)>, (version, &player, &data, line:u8 range 0..=3)) 내부 — 미탐색 | 4 |  |
| 16 | 미탐색 | (배치 C) BigPlan::update(m02.ll:6887) / BigPlan::is_end(m02.ll:6663) / BigPlan::is_cancel(m02.ll:8498) / BattlePlan::new(m10.ll:23214) / BattlePlan::update(m10.ll:23429) / is_ignored_battle_enemy(m10.ll:47629) 내부 — 본 배치 범위 밖(계약만). is_end·BattlePlan::update 는 &mut team_plan 을 받아 쓸 수 있음 | 4 |  |
| 17 | 표기 불가 | (배치 C) v50_fold_dive_episode(m13.ll:28946~, (&mut self, dive_abandoned: bool, reason: u8)) — 자식 명세 있음(계약만). L66 호출 직전 setting.tick_per_second 로드(%3552)가 사용처 없이 남아 있음 — 소스에서 인자였다가 접혔는지 불명(표기 불가) | 4 |  |
| 18 | 미탐색 | (배치 C) dive_episode.rs:47~49 타워 필터/키 클로저 본체 = m13.ll:4340~4531 (Chain<Flatten<...>>::min_by_key, `v50_track_dive_episode` 소유 closure$0/s_0) — 본 범위 밖이라 aux 미선언·미탐색. 필터가 tower_ty(%18) 를 캡처하는 것만 확인(23343) | 4 |  |
| 19 | 재료 부재 | (배치 C) ProfTimer 페이즈 코드 68/69/70/74/76 의 이름 — game_core::simulation::prof PHASE_NANOS[132] 는 external global 이고 이름표를 _gaibc/_gcbc 에서 못 찾음(재료 부재: 탐색 범위 = _gaibc m13 · _gcbc g06/g08/g09 grep). 판단에 무관한 프로파일링 | 4 |  |
| 20 | 미탐색 | (배치 C) exit_src 코드 11~14 / entry_src 6 / mf_swap src 25 / mf_obj_clear src 42 / v50 reason 1~8 의 의미 이름표 — battle.rs [ff 계측] 코드표·시뮬레이터 FF_BAIL_NAMES(_docs game_ai.txt:650~656 언급)는 본 범위 밖. 25=교체불명 만 _docs 확인 | 4 |  |
| 21 | 미탐색 | (배치 C) L993 v46_carry_over 가 구 플랜(old) 쪽에 무엇을 쓰는지 — 콜리 m04.ll:18821 미탐색 (new 쪽은 initializes((273,274)) = +0x111/+0x112 확정) | 4 |  |
| 22 | 미탐색 | (배치 C) L979 closure$9 의 정의 줄은 974(배치 B) — 본 범위에는 인라인 사본(21697~21709·21745~21758)만 있어 idx {1,2,5} 집합만 확정, 소스 이름 불명 | 4 |  |
| 23 | 표기 불가 | (배치 C) L1005 phase 선택의 소스 표기(match vs if-chain) — select 2단으로 접혀 표기 불가(동작은 확정) | 4 |  |
| 24 | 표기 불가 | (배치 C) L1081 `t.remain_action_time() < windup` 의 소스 표기(<= 대칭) — 표기 불가, 동작(ult → free 경로 / 아니면 locked)은 확정 | 4 |  |
| 25 | 미탐색 | (배치 C) L1251 dive_episode.rs:42 b.focus() 의 sub_goal 태그 default 케이스가 %1736(L711 루트, 배치 A)로 점프 — 그 블록 정체(unreachable/패닉)는 배치 A 몫 | 4 |  |
| 26 | 미탐색 | (배치 D) ★알려진 결과 「페이즈게이트 objective=min(C*A/1000,100)…handle_interact_battle」는 본 범위(1252~1672)에 없다 — handle_interact_battle 호출은 reach 표 L4618 @1911(;L738) = 배치 A/B 범위. 재확인 대상 아님(범위 밖). | 4 |  |
| 27 | 표기 불가 | (배치 D) battle_start_tick(0x520): 본 함수 안의 쓰기는 1323 Some(tick) 과 1501 None 뿐이고 1501 은 name!=prev 블록의 세 경로(%4711/%4438/%4431) 전부에서 실행된다 → 1323 의 Some 은 같은 호출 안에서 지워지고, 다른 쓰기는 LegacyPlanHandler::new(m13.ll:14329 store 0)뿐(IR 전 모듈 gep +1312 grep: m08 TLS·m14 Debug fmt/clone 제외) ⟹ 1403 unwrap_or(tick) 은 항상 tick, BattleResult.duration 은 항상 0 으로 보인다. 소스 원문이 없어 「if 안의 문장을 LLVM 이 끌어올린 것」인지 「소스가 원래 무조건」인지는 IR 로 구분 불가(표기 불가) — 동작(항상 None)은 확정. | 4 |  |
| 28 | 미탐색 | (배치 D) 1266 passive_plan sret 392B 중 뒤 8B 의 정체 — 본 범위에선 384B 만 memcpy(24431) 하고 나머지 미사용. 자식 명세 소관. | 4 |  |
| 29 | 미탐색 | (배치 D) @anon.85 (V46DEATH label 이름표, &str 배열)·@anon.71/72 포맷 문자열의 자리표시 이름은 텔레메트리라 해독하지 않음(지시). | 4 |  |
| 30 | 미탐색 | (배치 D) closure$27(1596) 의 `distance / max(speed,1)` 단위 = 틱 추정(거리/이동속도) — DI 이름 없음. | 5 |  |
| 31 | 미탐색 | (배치 D) PlanContext.gold_diff 는 1310 에서 리터럴 0 (store i32 0 @+120) — 계산 없음. | 4 |  |
| 32 | 미탐색 | (배치 D) su_0(1652) 의 타워 필터가 `game.tick() <= tower_attack_disable_tick` 인 점(공격 금지 틱 이전의 타워만 포함)은 IR 그대로 적었고 의미 해석은 하지 않음. | 4 |  |
| 33 | 미탐색 | (배치 D) 1305 enemy_count(PlanContext) 는 필터 없는 Some 개수, 1372 enemy_count(BattleStartState) 는 가시·비무시·150k 필터 — 같은 이름 다른 정의(관측). | 4 |  |

**`notes` 3건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | (배치 B) BigPlan::is_passive 의 PassiveJungle 분기: IR 은 `team == player_team → passive` 만 보이고 소스명 is_counter_jungle 은 스코프명으로만 확인 — `is_counter_jungle = team != player_team` 는 이름 기반 추정, 동작(eq→passive)은 확정 | 4 | 사실 서술 |
| 1 | (배치 B) Option<MainObjective> None = -1(0xff) — tcxdict 는 Niche 라고만 하고 값은 안 줌. IR store i8 -1(868)·icmp eq -1(1741/931/976) 로 확정 | 3 | 사실 서술 |
| 2 | (배치 D) 1594 so_0 (아군 타워 min_by_key 키) · 1639 sr_0 의 fold 본체(m12.ll 26623~26763) · Chain min_by_key(m06.ll 21079~21272) — 미독해(계측 전용이라 내려놓음). 1639 의 키는 본문 28516~28547 에 distance_sq(e, champ) 로 인라인돼 있어 그 부분만 확정. | 4 | 사실 서술 |

<details><summary>`closed` 1건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

| # | 물음 | 닫은 근거 |
|---|---|---|
| 0 | (배치 A) L688/L694 가 `if let … else if let …`(get_game_mode 2회 호출과 정합)인지 `match`(1회 호출이 보통)인지 — IR 은 두 번 호출(%217·%676)하므로 전자로 읽히나 소스 표기는 미확정(column 없음·MIR 없음). 동작 차이 없음. | 본문에 해소 표기가 있다 |
</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

