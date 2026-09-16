# r21 티어1 심층 — 웨이브 4 (1) · 0.5.8 명세 logic 전문 + exe 힌트

요구: 0.5.8 명세(아래 logic) 를 기준선으로 0.6.0 디컴을 **줄 단위로 대조**해 「어느 줄이 어떻게 바뀌었나 / 새 분기·조건·상수·필드·콜리」 를 명세 형식(줄번호 참조 · 등가 Rust)으로 낸다. version≥3 분기는 「v3 경로」 로 별도 표기(값 미확정 · v2/v3 둘 다 적는다). 대형 함수는 먼저 콜리 지도·블록 지도(0.5.8 ↔ 0.6.0 매핑)를 만들고 신규 구역을 특정한 뒤 신규 구역만 정독한다.

## `e4c5c0` → `d3d210` LegacyPlanHandler::update  (42054→77103B · Δ+35049)
- 힌트: LegacyPlanHandler::update(42→77KB · d3d210 · 근 8407→15063 명령 · 호출 d38180 13곳·d72e70·d52c00 · 래퍼 d5a940 이 `if 2<version` · abort_src +0x24ec byte store)
- exe 정렬: 명령 8407→15063 · 정렬 6973 · 잔여 구조 10 · 분기 10 · 콜리 주의: 2a300→2a300 미지(pdata 밖 thunk) ; 2b072a0→3821813 미지(-420B) ; 2b074c0→381ebcf 미지(-334B) ; 2b074c0→ff4380 미지(-206B) ; 2b1b410→16432c0 미지(+304B) ; 2b1b410→381e0b3 미지 ; 31a01a3→16432c0 미지 ; 31a01a3→381e0b3 미지
- 0.5.8 명세 #111 `handler__LegacyPlanHandler_update` src game-ai\src\plan_legacy\handler.rs:685 · one_line: AI 플랜 계층 최상위 틱 진입점 — 게임모드 분기(DeathMatch/SingleLane 전용 경로는 Moba 에서 사장) 뒤 Moba 본류: 룰스코프 정화·팀플랜/GoalData 갱신·채팅 수신/발신·플랜 전환(페이즈게이트 handle_interact_battle·패시브·전투 BattlePlan)·v3 귀환→패시브 사다리(1263~1289: Recall+만피면 passive_plan/PassiveLine 으로 플랜 교체)·서브플랜 병합·계측 기록(트레이스/SIM_STATS/DIEWIN)
- 0.5.8 params: self:&mut LegacyPlanHandler(6168B(IR %0 · `noalias align 8 dereferenceable(6168)` · readonly 없) · version:usize(IR %1 (i64). L685 에서 스택 스필 `store i64 %1, ptr %210`(m13.ll:1) · rnd:&mut StdRng(320B)(IR %2 · `noalias align 16 dereferenceable(320)` · readonly 없) · player:&PlayerState(2528B)(IR %3 · `readonly captures(address, read_provenance)` = 불변. ) · data:&OperationData(24B)(IR %4 · `readonly` = 불변. DI !23529. L688/L694 에서 data.cache() · debug:&mut DebugFrameData(224B)(IR %5 · `noalias align 8 dereferenceable(224)` · readonly 없음) · lapse:bool(IR %6 (i1 zeroext) · DI !23531 name "lapse" (m13.ll:92088) =)
- 0.5.8 logic 전문:
```
// handler.rs:685~695 (배치 A)
// L685: fn update(&mut self, version, rnd, player, data, debug, lapse) — 진입. version 을 스택 %210 에 스필(store, m13.ll:14711) 외 명령 없음(alloca 200여 개는 함수 전체 지역변수).
// L688: mode = data.cache.game.get_game_mode()  // vtable+0x40 간접호출, {tag,ptr}
//   if tag == 2 (GameMode::DeathMatch) → 블록 %220 … (L689)
// L689: self.update_deathmatch(version, rnd, player, data, debug)  // modes.rs 인라인 517줄(주석본 L220~L674)
//   // NA(gamemode≠Moba 전용: DeathMatch) — reach version=2·gamemode=Moba 접힘 @entry %219→0 · 블록 220~674 전부 사장(배치 A 236블록(220~674 85·675·679·680~1626 149) 중 234 사장 = 85+149) · reads/writes/consts 미등재
// L690: return;  → 블록 %674 `br label %679`(m13.ll:16480) → %679 `ret void`(m13.ll:16489) = 조기 반환 · L691: `}`  (rmeta_srcmap handler.rs L690=13자 `      return;` · L691=5자 `    }` · 파일은 2칸 들여쓰기)
// L694: if tag == 1 (GameMode::SingleLane) → 블록 %680 … (L695)   // ★L688 과 별개의 `if`(else-if 아님 — L691 이 `    }` 단독 줄) · get_game_mode() 재호출 %676(m13.ll:16483) · L696 `return;`(블록 %1626 `br label %679`, m13.ll:18969) · L697 `}`
// L695: self.update_single_lane(version, rnd, player, data, debug)  // modes.rs 인라인 1,060줄(주석본 L680~L1626)
//   // NA(gamemode≠Moba 전용: SingleLane) — reach 접힘 @675 %678→0 · 블록 680~1626 전부 사장 · reads/writes/consts 미등재
// else → 블록 %1627 = Moba 본류 → 배치 B(줄 698, `_t_head` ProfTimer 시작)
//
// self writes(살아있는 범위): 0건. HEAP 재료(살아있는 범위): 0건. rnd/debug 부작용: 0건.
// 함수 공용 EH: 살아있는 invoke 318개의 unwind(목표 64종: %3303 62·%2270 28·%4122 27·%2216 26·%1640 24 …)는 직접 또는 중간 cleanuppad 의 `cleanupret … unwind label` 사슬을 거쳐 **전부** 블록 %1640 cleanuppad 로 합류(살아있는 `unwind to caller` 는 %5375 하나뿐 — 사장 영역의 %571·%771·%946·%1570·%1589·%1609 은 DeathMatch/SingleLane 인라인 내부)(m13.ll:18998~19001 — `%1641 = phi i1 [...]` · `%1642 = cleanuppad within none []` · `br i1 %1641, label %5376, label %5375`) → phi %1641 참이면 %5376 에서 drop_glue<Option<ProfTimer>>(%209 = L698 `_t_head`) 후 %5375 `cleanupret unwind to caller`(m13.ll:28938, L685 귀속) — 패닉 전파 경로에서 self 쓰기 없음.
// 함수 공용 스필: %210 = version(i64) 스택 사본 · 배치 B~E 의 `load i64, ptr %210` 23곳이 전부 version.

// handler.rs:696~976 (배치 B)
// 진입: 배치 A 블록 %675(695) → %1627. 예외 cleanup 블록(1640·2039·2216·2270 등)은 언와인딩 전용이라 생략.
// 표기: game = data.cache.game(&dyn AbstractGame, vtable+0x28=tick, +0x130=kill_logs) · ctx = data.context · bb = data.blackboard[player.team]

698 _t_head = ProfTimer::start(77)                       // prof::ENABLED 원자 로드; 계측 전용(판정 무관, 이하 ProfTimer 전부 동일)
699 self.sanitize_rule_scope(data)           // 콜리 계약(tcx callees[86]) = (&mut self, &OperationData). IR 의 3인자 (self, cache=%211, ctx=%1639) 는 internal fastcc 함수의 &OperationData 인자가 필드로 갈라진 것(ArgPromotion, 추정 — define m13.ll:11163 인자 %1 `readonly captures(address_is_null)` %2 `captures(address, read_provenance)`). 714·761 도 동일 · &mut self = 본 범위 self 쓰기 재료(내부 미탐색)
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
    2280 my_line = rule_scope::fallback_line(ctx, my_line)   // 2인자(tcx callees[20]) — 「default Bottom」은 인자가 아니라 본체 rule_scope.rs:34(JungleOnly → %1749 → Bottom=2, m13.ll:19315/19318). 879 의 아웃오브라인 call 과 같은 함수 · 인라인 rule_scope.rs:29~34: line_exists(ctx,line)(tutorial.spawn_line_minion(line)) 이면 그대로, 아니면 valid_lines(tutorial).last() (First/Bottom→[Bottom], TopSolo→[Top], MidSolo→[Mid], MidBottom→[Mid,Bottom]), JungleOnly→default Bottom
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

728 if !lapse && (!self.received_chats.is_empty() || !self.misunderstood_received_chats.is_empty()) {   // ★lapse 게이트는 745 블록까지 덮는다: lapse=true 면 %1802 `br i1 %6, %1929, %1832`(m13.ll:19423) 로 758(%1929) 직행 — 745 의 chats_wait 검사(%1923)는 preds %1910(742 뒤)·%1838(수신 둘 다 빔)뿐
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
745 if !self.chats_wait.is_empty() {   // ← 728 과 같은 `!lapse` 블록 안(위 주석). lapse=true 면 이 블록 통째 생략(chats_wait 발행·retain 둘 다)
  746 for (tick, chat) in self.chats_wait.iter() { 747 if *tick <= game.tick() { 748 if chat_allowed(ctx, chat) { 749 self.chats.push(chat.clone()) /*★HEAP*/ } } }
  754 self.chats_wait.retain(|(tick,_)| *tick > game.tick())   // aux m06.ll:2727
}
758 if let Some(chats) = self.plan.chats() /*인라인 types.rs:185~189, 태그별 Vec 위치*/ { 759 self.chats.extend(chats.drain(..)) }   // ★HEAP
761 self.sanitize_rule_scope(data.cache, ctx)
763 drop(_t_head)
764 _t_np = ProfTimer::start(59)
765 next: Option<BigPlan> = self.plan.next_plan(version, rnd, player, data, &self.data, &self.positioning_score, &self.team_plan, debug)   // sret 384B · &mut 는 self.plan(%1930) 뿐, team_plan 은 `&TeamPlan`(tcx callees[69]). ⚠IR 인자 %1652 에 noalias/readonly/dereferenceable 이 없는 사유는 미탐색(정의 m02.ll:8528 %8 도 동일 — 본체는 %8 을 PassiveJungle/LineGanker/LineGankCover ::next_plan 에 넘길 뿐)
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
  1724 r2_saved: Option<BigPlan> = if version < 2 || !was_battle { None } else { Some(self.plan.clone()) }   // ★HEAP: BigPlan::clone(m13.ll:20310) = 페이로드 소유 Vec 복제(alloc) · 1753 이동 또는 1755 drop
  1726 _t = ProfTimer::start(80)
  1727 if lapse {
    1737 before_obj = self.team_plan.objective (3B); 1738 before_plan = self.plan.clone() /*★HEAP clone(m13.ll:20364) · 1744 이동 또는 1747 drop*/; 1739 before_chats = self.team_plan.chats.len()
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
  1763 goal = self.plan.goal()   // sret %31 24B — ★소비처 없음: %31 은 20510 호출 직후 21494 lifetime.end 뿐(로드 0건). 결과 미사용 호출(BigPlan::goal 은 &self 순수 accessor)

948 if self.plan.tag == ForcePassive(2) {
  949 (mf_p, mf_src) = self.passive_plan(version, rnd, player, data, debug)   // sret 392B = BigPlan + u8
  950 self.v2_apply_assign_commit(version, rnd, player, data, &mut mf_p, &mut mf_src, debug)   // 자식 명세(계약만) · ★둘 다 &mut(tcx): 952/953 의 mf_src 와 955 의 mf_p 는 커밋 뒤 값(IR %175 재로드 21542 · %176 → %173 memcpy 21548)
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
L982: self.v3_dest = self.v3_plan_dest(player, data, &new)   // tcx (&self, &PlayerState, &OperationData, &BigPlan) → fastcc 판(m13.ll:9990)은 self 제거·player→info.team(i64)·data→context 로 인자승격되어 (sret 24, team, &GameContext, &BigPlan) · 내부 미탐색
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
L1010: self.plan.update(version, rnd, player, data, &self.data(0x0), &self.team_plan(0xf8) /*tcx `&TeamPlan` — Atomic 3개 외 불변*/, &self.positioning_score(0x990), debug)   // BigPlan::update m02.ll:6887 — 자식 명세 없음, 내부 미탐색
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
L1118: if let BigPlan::Battle(b) /*별도 if — PassiveLine 블록 끝 %3125 가 %3032 로 가서 태그를 다시 읽는다(22590·22406). 태그가 배타라 else-if 와 외연 동일*/ = &mut self.plan { L1119: self.v54_reentry_ticks(0x900).append(&mut b.v54_reentry_ticks(0x670)) }
// ── L1121
L1121: self.sanitize_rule_scope(data)   // tcx (&mut self, &OperationData)(handler.rs:571) → fastcc 판(m13.ll:11163)은 data→(cache, context) 인자승격 · 내부 미탐색

// ── L1124~1126: 라인 백파이트 지원 진입 (enter_line_backfight_support, handler.rs:599~633 인라인)
L1124: timer = ProfTimer::start(76)
L1125: {
  599: if let BigPlan::PassiveLine(p) = &self.plan {
  600:   line = p.line(0x706)
  603:   if let Some((ally_id, focus_id)) = utils::line_backfight_support_focus(version, player, data, line) {   // sret 24 · m04.ll:53979 미탐색
  606:     if let Some(focus) = game.get_entity_by_id(focus_id) {
  609:       if !fight_model::is_ignored_battle_enemy(version, player, data, focus, false) {
  613:         let mut battle = BattlePlan::new(version, BattlePlanGoal::Support(focus_id) /*24B by-ptr %25: store i64 1(+0)=Support 태그, +8=focus_id(22743~22744) — tcxdict --enum BattlePlanGoal: 0 TryKill·1 Support·2 Response·3 Avoid*/, data, player)
  614:         battle.entry_src(+0x107) = 6
  615:         battle.support_target(+0x0) = Some(focus_id)
  616:         battle.set_main_objective(self.team_plan.objective(0x517))  // → +0xff
  617:         battle.update(version, rnd, player, data, &self.positioning_score, &self.team_plan /*tcx `&`*/, debug)
  618:         battle.chats.clear()   // +0x78 len=0
  620:         if battle.sub_goal(+0x58)@tag ∈ {RunAway 4, End 7} { drop(battle); → 633 }   // 진입 취소
  624:         else { if context.debug(0x3b) { 625: if let Some(c) = champ { 626~627: debug.infos(+0xa0).entry(c.id).or_insert(vec![]).push(format!("!v26 line backfight support: line {:?}, ally {}, focus {}", line, ally_id, focus_id)) } }
  631:           self.plan = BigPlan::Battle(battle)   // HEAP: 구 플랜 drop · tag 9 + memcpy 280 → 0x5f0
  }}}}}
}
L1126: drop(timer)

// ── L1128~1131: 종료 판정
L1128: timer = ProfTimer::start(74)
L1129: plan_ended = !self.plan.is_passive() && self.plan.is_end(version, rnd, player, data, &self.data, &self.team_plan /*tcx `&`*/, debug)
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
L1240: let (new, src) = self.passive_plan(version, rnd, player, data, debug)   // sret 392 = BigPlan 384 + u8 · tcx `&self`(self 쓰기 없음)
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
1254 let mut sub: SubPlan(72B) = BigPlan::sub_plan(&mut self.plan(0x5e8, %1930 readonly 없음), version, rnd, player, data, &self.data(GoalData 0x0 = %0 readonly), &self.team_plan(0xf8), &self.positioning_score(0x990), debug)   // 자식 명세, 계약만 — self.plan 은 &mut 로 넘어가 내부 쓰기 가능(1270/1282 도 동일)
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
1279         let plan = BigPlan::PassiveLine(PassiveLinePlan::new(line))   // tag 3(store i64 3 @+0) · 페이로드: v46_flee_entry tag@+8=0(None; 페이로드 +0x10~+0x20 미기록) · 5개 빈 Vec(cap 0·ptr=8) · +0x48~+0x120 memset 0 · line@+0x11e. ⚠+0x10~+0x20 과 +0x120~+0x180(enum 꼬리) 은 스택 잔재 그대로 self.plan 으로 memcpy 됨 → 0x5f8~0x608·0x708~0x768 은 비결정
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
1298         let reason = self.determine_transition_reason(&name)   // &self 메서드(prev = self.prev_plan_name 0x840 을 내부에서 읽음) · 인라인 handler.rs:1675~1707, 순서대로:
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
1332         (my_hp%, my_max) = my_champ.map(|c| (if c.max==0 {0} else {c.hp*100/c.max}, c.max)).unwrap_or((0,0))   // 1303 과 달리 max 0 이어도 panic 없음(icmp eq 0 → 0)
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
1413     (my_hp_end%, my_hp_now) = my_champ.map(|c| (if c.max==0 {0} else {c.hp*100/c.max}, c.hp)).unwrap_or((0,0))   // max 0 → (0, c.hp), panic 없음. 1339/1420 의 target 쪽도 같은 checked 형
1418     target = game.get_entity_by_id(state.target_id(+0x48)); 1420 (t_hp_end%, t_hp_now) = target.map(..).unwrap_or((0,0))
1425     damage_taken = (state.my_hp(+0x38) * state.my_max_hp(+0x40) / 100).saturating_sub(my_hp_now)
1428     damage_dealt = (state.target_hp(+0x50) * state.target_max_hp(+0x58) / 100).saturating_sub(t_hp_now); 1429 target None(=표적 소멸) 이면 = 시작 hp 전부(select %4486)
1437     kills  = game.kill_logs().iter().filter(|k| start<=k.tick<=tick && k.killer_team==team && (k.killer_position==pos || k.assist.contains(&pos))).count()
1443     deaths = kill_logs.filter(|k| start<=k.tick<=tick && k.killer_team!=team && k.killed_position==pos).count()
1450     (disengage_reason, deaths_field, target_escaped) =
             if deaths>0 {("died", deaths, false)} else if kills>0 {("kill_secured",0,false)} else if my_hp_end% < 30 {("low_hp_retreat",0, target.is_some())} else if target.is_some() /*get_entity_by_id Some = 표적 생존*/ {("target_escaped",0,true)} else /*None = 표적 소멸*/ {("target_died",0,false)}
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
