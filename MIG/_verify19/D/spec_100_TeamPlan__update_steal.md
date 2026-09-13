---

### `100` TeamPlan::update_steal — [v4] 정글러 막타 스틸 상태기계 — should_steal_now 로 새 액션을 얻고, 진행 중 세션의 종료(성공/사망/대상소멸/대상변경/None)·새 세션 시작(쿨다운 10초)·세션 통계 갱신·Lurk/Commit 진입·틱 카운트·steal_action/prev 저장을 매 틱 수행

| 항목 | 값 |
|---|---|
| id | `team_plan__update_steal` |
| 심볼 | `_RNvMs1_NtNtCshdEBA0ozCnw_7game_ai11plan_legacy9team_planNtB5_8TeamPlan12update_steal` |
| 소스 | `game-ai\src\plan_legacy\team_plan.rs:732` |
| IR | `m09.ll` 24466~25498행 |
| 경로·가시성 | `game_ai::plan_legacy::team_plan::TeamPlan::update_steal` · **pub** |
| 계층 | 기타 |
| exe | `dd90c0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r10` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &game_ai::plan_legacy::types::BigPlan)
```

<details><summary>인자 6개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | self | &mut TeamPlan(1064B) | steal 관련 필드 전부 쓰기 — writes 전수 참조 | 4 |
| 1 | 2 | version | usize | 본문 분기 없음 — should_steal_now 에 전달만 (m09.ll:24534) | 4 |
| 2 | 3 | player | &PlayerState(2528B) | info.position@tag(+0x9c0)·info.team(+0x930); should_steal_now·compute_my_execute_cut 에 전달 | 4 |
| 3 | 4 | data | &OperationData(24B) | cache(+0)·context(+8) | 4 |
| 4 | 5 | goal_data | &GoalData(248B) | 본문 직접 읽기 없음 — should_steal_now 에 전달만 | 4 |
| 5 | 6 | plan | &BigPlan(384B) | 태그(+0, i64)만 읽음: 9 = Battle (m09.ll:24548~24551) | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn update_steal(&mut self, version, player, data, goal_data, plan: &BigPlan)
  // L733: 정글러 전용
  if player.info.position != Jungle(1) { self.steal_action = None(-1); return }   // L906. prev_steal_action 은 안 건드림
  ctx = data.context; game = data.cache.game
  // L734~735
  new_action: StealAction(2B: tag, target) = if epic_exists(ctx) || serpen_exists(ctx) [tutorial ∈ {0,5,7,8}] { should_steal_now(version, player, goal_data, self, data) } else { None(0) }
  // L740~753
  prev_active = self.prev_steal_action.tag > 0   (Lurk/Commit)
  if !prev_active { if plan.tag == 9(Battle) { new_action = None(0) } ; target_changed = false }
  else if new_action.tag == 0 { target_changed = false }
  else { target_changed = (prev.target != new.target) }   // L753 xor
  new_active = new_action.tag != 0                          // L744
  // L755~757
  team = player.info.team
  me_dead = cache.player_champion[team][1].is_none()
  mob = game.get_game_mode().as_moba().unwrap()   (756/757 unwrap)
  ally_epic_buff_ticks = mob.epic_minion_buff_time[team]; ally_serpen_stacks = mob.serpen_count[team]
  // L759~804: 진행 중 세션 종료 판정
  if let Some(sess) = &self.current_steal_session {          // tag(+0x1ba) != 2
    t = sess.target == Serpen
    target_alive = if t { serpen_exists && mob.serpen.live_list.len != 0 } else { epic_exists && mob.epic.live_list.len != 0 }   // L761~762
    success = match self.steal_commit_snapshot { Some((t2, snap)) => if t2 { ally_serpen_stacks > snap } else { ally_epic_buff_ticks != 0 && snap == 0 }, None => false }   // L764~767
    if !(new_active && !me_dead && !success && target_alive) || target_changed {   // L770
      if let Some(mut sess) = self.current_steal_session.take() {    // L773~774, tag←2
        sess.end_tick = game.tick()                                  // L775
        alive_code = if target_alive(재계산 L777~778) { EligibilityLost(3) } else { EnemyKilled(1) }
        success2 = 위 success 와 동일식 (L780~786)
        sess.outcome = if success2 { Success(0) } else if me_dead { SelfDied(2) } else { alive_code }   // L786~788
        if success2 { if sess.target==Serpen { self.serpen_steal_success_count += 1 } else { self.epic_steal_success_count += 1 } }   // L796~798
        self.last_steal_session_end_tick[sess.target] = game.tick()   // L801~802
        self.completed_steal_sessions.push(sess)                        // L803
        self.steal_commit_snapshot = None                                // L804
      }
    }
  }
  // L808~859: 새 세션 시작
  if new_active {
    elapsed = game.tick().saturating_sub(self.last_steal_session_end_tick[new.target])
    if elapsed >= tps*10 {                                              // L810 (IR: < 이면 skip)
      if self.current_steal_session.is_none() || target_changed {        // L812
        t = new.target == Serpen
        target_entity = (t ? mob.serpen : mob.epic).live_list.get(0).and_then(|id| game.get_entity_by_id(*id))   // L815~819
        my_champ = cache.player_champion[team][1]
        (target_hp, my_dmg) = if let (Some(te), Some(c)) { (te.hp, compute_my_burst_damage_to_target(ctx, c, te)) } else { (0,0) }   // L821~823
        execute_cut = compute_my_execute_cut(player, ctx.setting)       // L826
        enemy_jungler_alive = cache.player_champion[1-team][1].is_some()  // L827
        snapshot = if t { ally_serpen_stacks } else { ally_epic_buff_ticks }   // L828
        self.steal_commit_snapshot = Some((new.target, snapshot))         // L832
        self.current_steal_session = Some(StealSession{ start_tick: tick, end_tick: tick, start_target_hp: target_hp, min_target_hp: target_hp, end_target_hp: target_hp, start_my_burst_damage: my_dmg, max_my_burst_damage: my_dmg, execute_cut_at_start: execute_cut, commit_entries:0, lurk_entries:0, commit_ticks:0, lurk_ticks:0, i_attacked_target_ticks:0, outcome: EligibilityLost(3), target: new.target, enemy_jungler_alive_at_start })   // L834~837
        (t ? self.serpen_steal_attempt_count : self.epic_steal_attempt_count) += 1
        if game.tick().saturating_sub(self.last_steal_chat_tick) >= tps*30 {   // L857
          self.last_steal_chat_tick = game.tick()                                // L858
          self.chats.push(if t { Chat::SerpenSteal(new.tag==Commit) } else { Chat::MorgardSteal(new.tag==Commit) })   // L859
        }
      }
    }
  }
  // L871~897: 진행 중 세션 매 틱 갱신
  if let Some(sess) = &mut self.current_steal_session {
    sess.end_tick = game.tick()                                          // L872
    target_entity = (sess.target==Serpen ? serpen : epic).live_list.get(0) → get_entity_by_id   // L873~877
    my_champ = cache.player_champion[team][1]
    match target_entity { None => sess.end_target_hp = 0 (L891),
      Some(te) => { sess.end_target_hp = te.hp (L881); sess.min_target_hp = min(sess.min_target_hp, te.hp) (L882);
                    if let Some(c) = my_champ { burst = compute_my_burst_damage_to_target(ctx, c, te); sess.max_my_burst_damage = max(.., burst) (L884~885);
                                               if te.last_attacked_from == Some(c.id) { sess.i_attacked_target_ticks += 1 } (L886~887) } } }
    match (self.prev_steal_action, new_action.tag) {   // L894~897
      (_, None) => {}
      (Some(Lurk), Lurk) => sess.lurk_ticks += 1,   (그 외, Lurk) => { sess.lurk_entries += 1; sess.lurk_ticks += 1 }
      (Some(Commit), Commit) => sess.commit_ticks += 1,   (그 외, Commit) => { sess.commit_entries += 1; sess.commit_ticks += 1 } }
  }
  self.steal_action = Some(new_action)        // L903 (+0x418/+0x419)
  self.prev_steal_action = Some(new_action)   // L904 (+0x41a/+0x41b)
```

**`mem` 메모리 접근 66건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x9c0 | info.position@tag | r | 1=Jungle 이 아니면 steal_action=None 후 즉시 return (m09.ll:24518~24521) | 4 | OK |  |
| 1 | PlayerState | 0x930 | info.team | r | team; enemy_team = 1-team (L827) | 4 | OK |  |
| 2 | OperationData | 0x0 | cache | r | &AbstractGameWithCache | 4 | OK |  |
| 3 | OperationData | 0x8 | context | r | &GameContext | 4 | OK |  |
| 4 | GameContext | 0x8 | setting | r | &GameSetting — tps · compute_my_execute_cut 인자 | 4 | OK |  |
| 5 | GameContext | 0x38 | tutorial | r | epic_exists(rule_scope.rs:46→runner.rs:263: 태그 1..6 이면 false) · serpen_exists(rule_scope.rs:50→runner.rs:267: {0,5,7,8} 참). L734 switch 는 두 집합의 합집합 {0,5,7,8} | 4 | OK |  |
| 6 | GameSetting | 0x12f8 | tick_per_second | r | tps*10 (L810) · tps*30 (L857) | 4 | OK |  |
| 7 | AbstractGameWithCache | 0x0 | game.data_ptr | r | vtable +0x28 tick / +0x40 get_game_mode / +0x1f0 get_entity_by_id | 4 | OK |  |
| 8 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r |  | 4 | OK |  |
| 9 | AbstractGameWithCache | 0x1e0 | player_champion | r | [team][1] = 내(정글) 챔피언 (me_dead 판정·my_champ) · [1-team][1] = 적 정글러 생존 (L827) | 4 | OK |  |
| 10 | MobaMode | 0x240 | epic_minion_buff_time[team] | r | ally_epic_buff_ticks = [team] (game.rs:210, L756, +576) | 4 | OK |  |
| 11 | MobaMode | 0x260 | serpen_count | r | ally_serpen_stacks = [team] (game.rs:211, L757, +608) | 4 | OK |  |
| 12 | MobaMode | 0x1a0 | jungle_runner.epic.live_list.buf.inner.ptr | r | epic live_list[0] = 대상 id (+416) | 4 | OK |  |
| 13 | MobaMode | 0x1a8 | jungle_runner.epic.live_list.len | r | 0 이면 에픽 없음 (+424) | 4 | OK |  |
| 14 | MobaMode | 0x1d0 | jungle_runner.serpen.live_list.buf.inner.ptr | r | serpen live_list[0] (+464) | 4 | OK |  |
| 15 | MobaMode | 0x1d8 | jungle_runner.serpen.live_list.len | r | (+472) | 4 | OK |  |
| 16 | Entity | 0x670 | hp | r | 대상 hp (L823·L881) | 4 | OK |  |
| 17 | Entity | 0x5c0 | id | r | my_champ.id (L886) | 4 | OK |  |
| 18 | Entity | 0x28 | last_attacked_from@tag | r | 대상.last_attacked_from: Option<usize> (+40 태그, +48 값) == my_champ.id 이면 i_attacked_target_ticks++ (L886) | 4 | OK |  |
| 19 | Entity | 0x30 | last_attacked_from@Some.0 | r |  | 4 | OK |  |
| 20 | BigPlan | 0x0 | @tag | r | 9 = Battle (L741) | 4 | OK |  |
| 21 | TeamPlan | 0x41a | prev_steal_action@tag | r | L740: >0(Lurk1/Commit2) 이면 '진행 중' · L894: -1(None)/1/2 비교 | 4 | OK |  |
| 22 | TeamPlan | 0x41b | prev_steal_action@Some.0@Lurk.0@tag | r | prev.target — new.target 과 xor 로 target_changed (L753) | 4 | OK |  |
| 23 | TeamPlan | 0x1ba | current_steal_session@tag | r | 2 = None (니치: enemy_jungler_alive_at_start bool 자리) | 4 | OK |  |
| 24 | TeamPlan | 0x1b9 | current_steal_session@Some.0.target@tag | r | 0=Epic/1=Serpen | 4 | OK |  |
| 25 | TeamPlan | 0x120 | steal_commit_snapshot@tag | r | 2 = None; 0/1 = Some((StealTarget, usize)) | 4 | OK |  |
| 26 | TeamPlan | 0x128 | steal_commit_snapshot@Some.0.1 | r | snap 값(에픽: 버프 틱 / 세르펜: 스택) | 4 | OK |  |
| 27 | TeamPlan | 0x150 | current_steal_session@Some.0.start_tick | r | take() 시 읽기 | 4 | OK |  |
| 28 | TeamPlan | 0x3e0 | last_steal_session_end_tick[] | r | [new.target] — 쿨다운 10초 (L810) | 4 | OK |  |
| 29 | TeamPlan | 0x3d8 | last_steal_chat_tick | r | 채팅 30초 간격 (L857) | 4 | OK |  |
| 30 | TeamPlan | 0xf0 | completed_steal_sessions.buf.inner.cap.0 | r | push 의 grow 판정 | 4 | OK |  |
| 31 | TeamPlan | 0xc0 | chats.buf.inner.cap.0 | r | push 의 grow 판정 | 4 | OK |  |
| 32 | TeamPlan | 0x418 | steal_action@tag | w | L906: position != Jungle 이면 여기만 쓰고 return. DWARF Option<StealAction> None DISCR_EXACT=255 (m09.ll !4895) | 3 | OK | -1 (Option None) |
| 33 | TeamPlan | 0x418 | steal_action@tag | w | L903 (m09.ll:25298) | 4 | OK | new_action.tag (0 None/1 Lurk/2 Commit) |
| 34 | TeamPlan | 0x419 | steal_action@Some.0@Lurk.0@tag | w | L903 (m09.ll:25300) | 4 | OK | new_action.target |
| 35 | TeamPlan | 0x41a | prev_steal_action@tag | w | L904 (m09.ll:25303) — 다음 틱의 prev | 4 | OK | new_action.tag |
| 36 | TeamPlan | 0x41b | prev_steal_action@Some.0@Lurk.0@tag | w | L904 | 4 | OK | new_action.target |
| 37 | TeamPlan | 0x1ba | current_steal_session@tag | w | L774 take() (m09.ll:24786) — 세션 종료 시 | 4 | OK | 2 (None) |
| 38 | TeamPlan | 0x1ba | current_steal_session@Some.0.enemy_jungler_alive_at_start | w | L834 새 세션 (m09.ll:25205) | 4 | OK | enemy_jungler_alive as u8 (=Some 태그) |
| 39 | TeamPlan | 0x150 | current_steal_session@Some.0.start_tick | w | L834/836 | 4 | OK | game.tick() |
| 40 | TeamPlan | 0x158 | current_steal_session@Some.0.end_tick | w | L834/837 새 세션 · L872 매 틱 갱신 (m09.ll:25266) | 4 | OK | game.tick() |
| 41 | TeamPlan | 0x160 | current_steal_session@Some.0.start_target_hp | w | L834 | 4 | OK | target_hp |
| 42 | TeamPlan | 0x168 | current_steal_session@Some.0.min_target_hp | w | L834 초기 · L882 갱신 (m09.ll:25407) | 4 | OK | target_hp / min(min, hp) |
| 43 | TeamPlan | 0x170 | current_steal_session@Some.0.end_target_hp | w | L834 · L881 매 틱 · L891 대상 없으면 0 | 4 | OK | target_hp / hp / 0 |
| 44 | TeamPlan | 0x178 | current_steal_session@Some.0.start_my_burst_damage | w | L834 | 4 | OK | my_dmg |
| 45 | TeamPlan | 0x180 | current_steal_session@Some.0.max_my_burst_damage | w | L834 · L885 (m09.ll:25429) | 4 | OK | my_dmg / max(max, burst) |
| 46 | TeamPlan | 0x188 | current_steal_session@Some.0.execute_cut_at_start | w | L834 | 4 | OK | compute_my_execute_cut(player, setting) |
| 47 | TeamPlan | 0x190 | current_steal_session@Some.0.commit_entries | w | L834 memset(+400..+440) · L894~897 Commit 진입(prev≠Commit) 시 +1 (m09.ll:25285 phi 400) | 4 | OK | 0 (memset 40B) / +1 |
| 48 | TeamPlan | 0x198 | current_steal_session@Some.0.lurk_entries | w | Lurk 진입(prev≠Lurk) 시 +1 (phi 408) | 4 | OK | 0 / +1 |
| 49 | TeamPlan | 0x1a0 | current_steal_session@Some.0.commit_ticks | w | new==Commit 인 매 틱 +1 (m09.ll:25293 phi 416) | 4 | OK | 0 / +1 |
| 50 | TeamPlan | 0x1a8 | current_steal_session@Some.0.lurk_ticks | w | new==Lurk 인 매 틱 +1 (phi 424) | 4 | OK | 0 / +1 |
| 51 | TeamPlan | 0x1b0 | current_steal_session@Some.0.i_attacked_target_ticks | w | L887 대상.last_attacked_from == 내 id 이면 +1 (m09.ll:25456) | 4 | OK | 0 / +1 |
| 52 | TeamPlan | 0x1b8 | current_steal_session@Some.0.outcome@tag | w | L834 초기값 (m09.ll:25202). 종료 시 outcome 은 take() 한 로컬 sess 에 써서 completed 로 push (self 필드엔 안 씀) | 4 | OK | 3 (EligibilityLost) |
| 53 | TeamPlan | 0x1b9 | current_steal_session@Some.0.target@tag | w | L834 | 4 | OK | new_action.target |
| 54 | TeamPlan | 0x120 | steal_commit_snapshot@tag | w | L832 새 세션 (m09.ll:25178) · L804 세션 종료 (m09.ll:24989) | 4 | OK | new_action.target (=Some) / 2 (None) |
| 55 | TeamPlan | 0x128 | steal_commit_snapshot@Some.0.1 | w | L832: Serpen→ally_serpen_stacks, Epic→ally_epic_buff_ticks · L804: 0 | 4 | OK | snapshot / 0 |
| 56 | TeamPlan | 0x3b8 | epic_steal_attempt_count | w | 새 세션 target Epic (m09.ll:25210 select 952) | 4 | OK | +1 |
| 57 | TeamPlan | 0x3c8 | serpen_steal_attempt_count | w | 새 세션 target Serpen (select 968) | 4 | OK | +1 |
| 58 | TeamPlan | 0x3c0 | epic_steal_success_count | w | L797 세션 종료 success && target Epic (m09.ll:24933) | 4 | OK | +1 |
| 59 | TeamPlan | 0x3d0 | serpen_steal_success_count | w | L798 success && Serpen (m09.ll:24926) | 4 | OK | +1 |
| 60 | TeamPlan | 0x3e0 | last_steal_session_end_tick[] | w | L802 [sess.target] (m09.ll:24948) | 4 | OK | game.tick() |
| 61 | TeamPlan | 0x3d8 | last_steal_chat_tick | w | L858 (m09.ll:25225) | 4 | OK | game.tick() |
| 62 | TeamPlan | 0xf8 | completed_steal_sessions.buf.inner.ptr[len] | w | L803 push (grow_one 가능) (m09.ll:24974~24984) | 4 | OK | 종료된 StealSession(112B: end_tick/outcome 갱신본) |
| 63 | TeamPlan | 0x100 | completed_steal_sessions.len | w | L803 (m09.ll:24988) | 4 | OK | +1 |
| 64 | TeamPlan | 0xc8 | chats.buf.inner.ptr[len] | w | L859 push (m09.ll:25033~25035) | 4 | OK | Chat{tag: Serpen→32 SerpenSteal / Epic→41 MorgardSteal, +1: is_commit(new.tag==2)} |
| 65 | TeamPlan | 0xd0 | chats.len | w | L859 (m09.ll:25037) | 4 | OK | +1 |

**`consts` 상수 23건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 1 | 733 | 태그 | Position 태그 1=Jungle — 정글러만 실행 (그 외 steal_action=None 후 return) | 4 |
| 1 | 0 | 734 | 태그 | TutorialType 태그 0=None (epic/serpen 존재 집합 {0,5,7,8}) | 4 |
| 2 | 5 | 734 | 태그 | TutorialType 5=MidBottom (serpen 만 존재) | 4 |
| 3 | 7 | 734 | 태그 | TutorialType 7=Line | 4 |
| 4 | 8 | 734 | 태그 | TutorialType 8=Total | 4 |
| 5 | 6 | 761 | 태그 | epic_exists(runner.rs:263): (tag-1) < 6 즉 태그 1..6 이면 에픽 없음 | 4 |
| 6 | 9 | 741 | 센티널 | BigPlan 메모리태그 9 = Battle (tcxdict --enum BigPlan, 니치 idx7+2). prev 가 진행 중이 아닐 때 Battle 이면 new_action=None | 3 |
| 7 | 0 | 741 | 태그 | StealAction::None 태그 0 (new_action 무효화 / L744 `!= 0` = 액션 활성) | 4 |
| 8 | 2 | 759 | 센티널 | Option<StealSession> None 니치 = 2 (enemy_jungler_alive_at_start bool 자리, +0x1ba) · Option<(StealTarget,usize)> None = 2 (+0x120) 도 동일 | 4 |
| 9 | 2 | 774 | 태그 | take(): current_steal_session 태그 ← 2 (None) | 4 |
| 10 | 1 | 788 | 태그 | StealOutcome 1=EnemyKilled — 대상이 없으면(live_list.len==0 또는 존재 안 함) | 4 |
| 11 | 3 | 788 | 임계 | StealOutcome 3=EligibilityLost — 대상 살아있는데 종료 / 새 세션 초기 outcome (L834) | 4 |
| 12 | 2 | 788 | 임계 | StealOutcome 2=SelfDied — me_dead 우선 | 4 |
| 13 | 0 | 786 | 임계 | StealOutcome 0=Success — snapshot 성공 판정 시 | 4 |
| 14 | 10 | 810 | 계수 | tps*10 = 10초 — 같은 대상 세션 종료 후 재시작 쿨다운: tick - last_steal_session_end_tick[target] < tps*10 이면 새 세션 안 만듦 | 4 |
| 15 | 30 | 857 | 계수 | tps*30 = 30초 — 스틸 채팅 최소 간격 | 4 |
| 16 | 32 | 859 | 태그 | Chat 태그 32=SerpenSteal (target Serpen) | 4 |
| 17 | 41 | 859 | 태그 | Chat 태그 41=MorgardSteal (target Epic) | 4 |
| 18 | 112 | 803 | 미상 | StealSession 크기(Vec push elem_size) — stride, 임계 아님 | 4 |
| 19 | 24 | 859 | 미상 | Chat 크기(Vec push elem_size) — stride | 4 |
| 20 | 40 | 834 | 길이 | memset 길이: commit_entries~i_attacked_target_ticks 5×8B 를 0 으로 | 4 |
| 21 | -1 | 894 | 센티널 | prev_steal_action == None(니치 -1/255) 이면 Lurk/Commit 을 '진입' 으로 침(entries+1) | 4 |
| 22 | 3 | 904 | 임계 | assume new.tag < 3 (StealAction 유효 범위) — 검사 아님 | 4 |

**`knobs` 조정점 4건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 같은 대상 스틸 세션 재시작 쿨다운 | team_plan.rs:810 | tps*10 | 내리면 세션 종료 직후 바로 새 세션(Lurk/Commit)이 열려 재시도가 잦아진다 | 4 | 기존 |
| 1 | 스틸 채팅 간격 | team_plan.rs:857 | tps*30 | 연출 전용(Chat push). 판단 영향 없음 | 4 | 기존 |
| 2 | Battle 플랜 중 스틸 시작 차단 | team_plan.rs:741 | BigPlan 태그 9 | 제거하면 교전 중에도 새 스틸 액션이 시작된다(진행 중이면 원래도 유지) | 4 | 기존 |
| 3 | 정글러 전용 게이트 | team_plan.rs:733 | Position 1 | 다른 포지션은 항상 steal_action=None | 4 | 기존 |

<details><summary>`callees` 피호출자 18건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | as_moba | game_core::GameMode::<'a>::as_moba | pub | fn(game_core::GameMode<'a/#0>) -> std::option::Option<&'a/#0 game_core::MobaMode> | game-core\src\simulation\game.rs:230 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 1 | compute_my_burst_damage_to_target | game_ai::plan_legacy::steal::compute_my_burst_damage_to_target | pub | fn(&game_core::GameContext, &game_core::Entity, &game_core::Entity) -> usize | game-ai\src\plan_legacy\steal.rs:178 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 2 | compute_my_execute_cut | game_ai::plan_legacy::steal::compute_my_execute_cut | pub | fn(&game_core::PlayerState, &game_core::GameSetting) -> usize | game-ai\src\plan_legacy\steal.rs:226 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 3 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 4 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 5 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 6 | get_game_mode | game_core::AbstractGame::get_game_mode | pub | fn(&Self/#0) -> game_core::GameMode | game-core\src\simulation.rs:78 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 7 | get_game_mode | <game_core::Game as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::Game) -> game_core::GameMode | game-core\src\simulation\game.rs:1707 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 8 | get_game_mode | <game_core::SingleLaneGame as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::SingleLaneGame) -> game_core::GameMode | game-core\src\simulation\game.rs:4248 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 9 | push | <game_core::Staff as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::staff | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\staff.rs:38 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 10 | push | <game_core::Athlete as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:434 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 11 | push | <game_core::Contract as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:2498 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 12 | serpen_exists | game_ai::plan_legacy::rule_scope::serpen_exists | pub | fn(&game_core::GameContext) -> bool | game-ai\src\plan_legacy\rule_scope.rs:49 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 13 | should_steal_now | game_ai::plan_legacy::steal::should_steal_now | pub | fn(usize, &game_core::PlayerState, &game_ai::GoalData, &game_ai::plan_legacy::team_plan::TeamPlan, &game_core::OperationData) -> game_ai::plan_legacy::steal::StealAction | game-ai\src\plan_legacy\steal.rs:279 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 14 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 15 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 16 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 17 | update_steal | game_ai::plan_legacy::team_plan::TeamPlan::update_steal | pub | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &game_ai::plan_legacy::types::BigPlan) | game-ai\src\plan_legacy\team_plan.rs:732 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
</details>

⚠**미매칭 4개**: `epic_exists`, `grow_one`, `take`, `target_alive`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m09.ll:25666) · **형제 55개** (TeamPlan)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | game_ai::plan_legacy::old::epic::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_press_epic | pub | game-ai\src\plan_legacy\old\epic.rs:502 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 1 | game_ai::plan_legacy::old::epic::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v3_epicops_buff_window | in:game_ai | game-ai\src\plan_legacy\old\epic.rs:634 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &game_ai::plan_legacy::types::BigPlan) -> bool |
| 2 | game_ai::plan_legacy::old::epic::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_epic_line_change | pub | game-ai\src\plan_legacy\old\epic.rs:684 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData, game_core::LineType) |
| 3 | <game_ai::plan_legacy::team_plan::TeamPlan as std::clone::Clone>::clone | pub | game-ai\src\plan_legacy\team_plan.rs:47 | True | fn(&game_ai::plan_legacy::team_plan::TeamPlan) -> game_ai::plan_legacy::team_plan::TeamPlan |
| 4 | <game_ai::plan_legacy::team_plan::TeamPlan as std::fmt::Debug>::fmt | pub | game-ai\src\plan_legacy\team_plan.rs:47 | True | fn(&game_ai::plan_legacy::team_plan::TeamPlan, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 5 | <game_ai::plan_legacy::team_plan::TeamPlan as std::default::Default>::default | pub | game-ai\src\plan_legacy\team_plan.rs:47 | True | fn() -> game_ai::plan_legacy::team_plan::TeamPlan |
| 6 | game_ai::plan_legacy::team_plan::TeamPlan::mf_note_obj_clear | in:game_ai | game-ai\src\plan_legacy\team_plan.rs:196 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, u8, usize) |
| 7 | game_ai::plan_legacy::team_plan::TeamPlan::sanitize_rule_scope | pub | game-ai\src\plan_legacy\team_plan.rs:200 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, &game_core::GameContext) |
| 8 | game_ai::plan_legacy::team_plan::TeamPlan::objective_target | pub | game-ai\src\plan_legacy\team_plan.rs:230 | True | fn(&game_ai::plan_legacy::team_plan::TeamPlan) -> std::option::Option<game_core::JungleType> |
| 9 | game_ai::plan_legacy::team_plan::TeamPlan::take_active | pub | game-ai\src\plan_legacy\team_plan.rs:243 | True | fn(&game_ai::plan_legacy::team_plan::TeamPlan, game_core::JungleType) -> bool |
| 10 | game_ai::plan_legacy::team_plan::TeamPlan::take_hunt_commit | pub | game-ai\src\plan_legacy\team_plan.rs:248 | True | fn(&game_ai::plan_legacy::team_plan::TeamPlan, game_core::JungleType) -> bool |
| 11 | game_ai::plan_legacy::team_plan::TeamPlan::take_setup_like | pub | game-ai\src\plan_legacy\team_plan.rs:257 | True | fn(&game_ai::plan_legacy::team_plan::TeamPlan, game_core::JungleType) -> bool |
| 12 | game_ai::plan_legacy::team_plan::TeamPlan::mark_objective_misunderstanding | pub | game-ai\src\plan_legacy\team_plan.rs:265 | True | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, game_core::JungleType, usize) |
| 13 | game_ai::plan_legacy::team_plan::TeamPlan::clear_objective_misunderstanding | pub | game-ai\src\plan_legacy\team_plan.rs:277 | True | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan) |
| 14 | game_ai::plan_legacy::team_plan::TeamPlan::init | in:game_ai::plan_legacy::team_plan | game-ai\src\plan_legacy\team_plan.rs:281 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 15 | game_ai::plan_legacy::team_plan::TeamPlan::update | pub | game-ai\src\plan_legacy\team_plan.rs:294 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 16 | game_ai::plan_legacy::team_plan::TeamPlan::can_near_enemies | pub | game-ai\src\plan_legacy\team_plan.rs:444 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< &game_core::Entity> |
| 17 | game_ai::plan_legacy::team_plan::TeamPlan::can_near_enemies_range | pub | game-ai\src\plan_legacy\team_plan.rs:483 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, u64, u64, u64, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< &game_core::Entity> |
| 18 | game_ai::plan_legacy::team_plan::TeamPlan::update_wave_priority_clear_line | in:game_ai::plan_legacy::team_plan | game-ai\src\plan_legacy\team_plan.rs:540 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) |
| 19 | game_ai::plan_legacy::team_plan::TeamPlan::should_player_clear_wave_priority_line | pub | game-ai\src\plan_legacy\team_plan.rs:561 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PlayerState, &game_core::OperationData) -> std::option::Option<game_core::LineType> |
| 20 | game_ai::plan_legacy::team_plan::TeamPlan::should_delay_morgard_for_wave_priority | in:game_ai::plan_legacy::team_plan | game-ai\src\plan_legacy\team_plan.rs:570 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData) -> bool |
| 21 | game_ai::plan_legacy::team_plan::TeamPlan::should_delay_serpen_for_wave_priority | in:game_ai::plan_legacy::team_plan | game-ai\src\plan_legacy\team_plan.rs:574 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData) -> bool |
| 22 | game_ai::plan_legacy::team_plan::TeamPlan::should_delay_object_setup_for_wave_priority | pub | game-ai\src\plan_legacy\team_plan.rs:578 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData) -> bool |
| 23 | game_ai::plan_legacy::team_plan::TeamPlan::should_keep_object_for_contested_wave_priority | in:game_ai::plan_legacy::team_plan | game-ai\src\plan_legacy\team_plan.rs:586 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, game_ai::plan_legacy::team_plan::objective_helpers::WavePriorityObject) -> bool |
| 24 | game_ai::plan_legacy::team_plan::TeamPlan::repair_misunderstood_objective | in:game_ai::plan_legacy::team_plan | game-ai\src\plan_legacy\team_plan.rs:603 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData) -> bool |
| 25 | game_ai::plan_legacy::team_plan::TeamPlan::update_objective | pub | game-ai\src\plan_legacy\team_plan.rs:722 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData) |
| 26 | game_ai::plan_legacy::team_plan::TeamPlan::update_steal | pub | game-ai\src\plan_legacy\team_plan.rs:732 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &game_ai::plan_legacy::types::BigPlan) |
| 27 | game_ai::plan_legacy::team_plan::TeamPlan::update_objective_after_steal | in:game_ai::plan_legacy::team_plan | game-ai\src\plan_legacy\team_plan.rs:910 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData) |
| 28 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v24_objective_setup_lane_pressure_ready | pub | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:7 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, game_core::JungleType, &mut game_core::DebugFrameData) -> bool |
| 29 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v24_objective_setup_should_check_camp | pub | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:88 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, game_core::JungleType) -> bool |
| 30 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v24_objective_setup_should_release_to_passive | pub | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:177 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::PlayerState, &game_core::OperationData, game_core::JungleType) -> bool |
| 31 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v24_objective_setup_relevant_lanes_ready | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:199 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PlayerState, &game_core::OperationData, game_core::JungleType) -> bool |
| 32 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v25_objective_posture | pub | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:207 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::PlayerState, &game_core::OperationData, game_core::JungleType) -> std::option::Option<game_ai::plan_legacy::team_plan::ObjectivePosture> |
| 33 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v27_objective_wait_pos | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:316 | False | fn(game_core::JungleType, usize, &game_core::MapDef) -> (u64, u64) |
| 34 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v27_objective_entity | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:324 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, &game_core::OperationData, game_core::JungleType) -> std::option::Option<&game_core::Entity> |
| 35 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::update_v27_objective_discipline | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:334 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &mut game_core::DebugFrameData) |
| 36 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v27_active_objective_discipline | pub | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:483 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::OperationData, game_core::JungleType) -> std::option::Option<game_ai::plan_legacy::team_plan::ObjectiveDisciplineState> |
| 37 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v27_objective_discipline_blocks_battle | pub | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:497 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::OperationData) -> bool |
| 38 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v27_objective_discipline_action | pub | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:505 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, game_core::JungleType) -> std::option::Option<game_ai::SmallActionPlay> |
| 39 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_repair_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:7 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PlayerState, &game_core::OperationData) |
| 40 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_defense_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:16 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData) -> bool |
| 41 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_nexus_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:29 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData, game_core::LineType) -> bool |
| 42 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_defense_line_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:46 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData, game_core::LineType) -> bool |
| 43 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_dive_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:88 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData, game_core::LineType) -> bool |
| 44 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_press_tower_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:141 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData, game_core::LineType) -> bool |
| 45 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_press_epic_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:253 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData, game_core::LineType) -> bool |
| 46 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_split_epic_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:334 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData, game_core::LineType) -> bool |
| 47 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_comeback_pick_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:393 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData, game_core::LineType, bool) -> bool |
| 48 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_serpen_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:558 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData, game_ai::plan_legacy::team_plan::ObjectPhase) -> bool |
| 49 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_morgard_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:685 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData, game_ai::plan_legacy::team_plan::ObjectPhase) -> bool |
| 50 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_none_or_gank_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:830 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData) -> bool |
| 51 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_gank_preprocess | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:1133 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, game_core::LineType) -> bool |
| 52 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_nexus_defense | pub | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:1227 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> bool |
| 53 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_nexus_attack | pub | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:1237 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> std::option::Option<game_core::LineType> |
| 54 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_sub_objective | pub | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:1258 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData) |

**`open` 6건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | L734 switch {0,5,7,8} 의 소스 표현 — dbg 는 rule_scope.rs:46(→runner.rs:263, epic_exists) 하나만 가리키나 그 함수의 참 집합은 L761 에서 {0,7,8} 로 실측됐다. {0,5,7,8} = epic_exists ∪ serpen_exists 이므로 `epic_exists(ctx) \|\| serpen_exists(ctx)` 병합으로 추정(LLVM switch 병합). 확정 방법 = runner.rs:263/267 원문 또는 rmeta 줄 길이 | 3 |  |
| 1 | 표기 불가 | L770 종료 조건의 소스 표현 — IR 은 두 판(스냅샷 유/무)으로 갈라져 있고 `!(new_active && !(me_dead\|\|success) && target_alive) \|\| target_changed` 로 읽었다. 항 순서는 표기 불가 | 4 |  |
| 2 | 미탐색 | should_steal_now / compute_my_burst_damage_to_target / compute_my_execute_cut 내부는 이 명세 범위 밖(should_steal_now 는 같은 배치 별도 명세) | 4 |  |
| 3 | 미탐색 | epic_exists 의 정확한 참 집합: IR 은 (tag-1) ult 6 → 거짓 집합 {1..6}, 즉 참 = {0,7,8} (None/Line/Total). 다른 태그(9+)는 TutorialType 에 없음 | 4 |  |
| 4 | 미탐색 | exe 에서 compute_my_burst_damage_to_target 직접 call 이 1개(0xd9dbe0)뿐인 이유(IR 2회) — 한쪽 인라인 또는 블록 공유 추정, Ghidra 미확인 | 4 |  |
| 5 | 미탐색 | _docs: 'v4: 정글러 막타 스틸 — 세션 단위 추적. update_objective 에서 분리 … v100 에서도 이 함수만 직접 호출' (game_ai.txt:388~391) — 호출자 2곳(exe 0xe4c5c0) 과 부합 | 4 |  |

**`notes` 1건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | L786~788 outcome 의 소스 형태 — IR 은 `select(len==0,1,3)` 로 접혀 있어 target_alive→EligibilityLost / !alive→EnemyKilled 는 값으로만 확정 | 4 | 사실 서술 |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

