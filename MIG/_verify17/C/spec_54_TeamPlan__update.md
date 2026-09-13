---

### `54` TeamPlan::update — 매 틱 TeamPlan 상태 갱신 — 정글캠프 리스폰 추적·타임아웃 정리·웨이브우선 라인·에픽/세르펜 콜 채팅·적 시야 추적·MIA 채팅

| 항목 | 값 |
|---|---|
| id | `team_plan__update` |
| 심볼 | `_RNvMs1_NtNtCshdEBA0ozCnw_7game_ai11plan_legacy9team_planNtB5_8TeamPlan6update` |
| 소스 | `game-ai\src\plan_legacy\team_plan.rs:294` |
| IR | `m09.ll` 38194~39876행 |
| 경로·가시성 | `game_ai::plan_legacy::team_plan::TeamPlan::update` · **pub** |
| 계층 | 기타 |
| exe | `de0770` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r8` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData)
```

<details><summary>인자 6개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | self | &mut TeamPlan(1064B) | 아래 writes 전수 참조 | 4 |
| 1 | 2 | version | usize | 본문 분기 없음. 인라인된 init 에도 _version 으로 미사용 | 4 |
| 2 | 3 | rnd | &mut StdRng(320B) | PlayerState::strategy(player, rnd, game) 에만 전달(L541) | 4 |
| 3 | 4 | player | &PlayerState(2528B) | info.team(+0x930)·info.position 태그(+0x9c0)·strategy | 4 |
| 4 | 5 | data | &OperationData(24B) | cache(+0x0)·context(+0x8)·blackboard(+0x10) 전부 사용 | 4 |
| 5 | 6 | debug | &mut DebugFrameData(224B) | readnone — 본문에서 전혀 안 씀(인라인 init 에서도 _debug) | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn update(&mut self, version, rnd, player, data, debug)

[L295] context = data.context; self.sanitize_rule_scope(context)          // 외부 호출(내용 안 봄)
[L296] if !self.is_init(+0x41c) {  // ── init 인라인 (team_plan.rs:281~289)
[L282]   self.is_init = true; team = player.info.team
[L285]   if game.get_game_mode() is Moba(m) {           // 태그 0 아니면 건너뜀
[L288]     for (idx, jungle) in [(0,Rhino0),(1,Mushroom1),(2,Bee3),(3,Stump2)] {
             self.next_respawn_tick[team][idx]   = m.jungle_runner(+0x18).get_camp_state(team,   jungle).+0x18
[L289]       self.next_respawn_tick[1-team][idx] = m.jungle_runner.get_camp_state(1-team, jungle).+0x18
           } } }
[L301] if let Some(st) = self.objective_discipline (+0x149 != 2) { if st.until_tick(+0x140) <= tick { [L302] self.objective_discipline = None } }

// ── 정글캠프 리스폰 추적 (L305~320) — team ∈ {0,1}, jungle ∈ [Rhino,Mushroom,Bee,Stump]
[L308] camp_pos = MapDef::camp_pos(map, jungle, team==0); (xi,yi) = (x/32000, y/32000)
[L311] if game.is_visible_cell(my_team, xi, yi) {                    // ★내 팀 시야 기준 (인자 = player.info.team)
[L314]   if tick >= self.next_respawn_tick[team][idx] {
[L317]     if !(mode is Moba && m.jungle_runner.get_camp_state(team, jungle).live_list.len(+0x10) != 0) {
[L320]       info = JungleType::get_info(&jungle, setting)   // 272B
             self.next_respawn_tick[team][idx] = tick + info.respawn_tick(+0x100)
           } } }

// ── 타임아웃 정리 (L325~346)
[L326] for i in 0..5 { if allies[i].is_some() && allies[i].last_tick + tps*10 < tick { allies[i] = None } }   // 각 i 뒤에
[L331]   if let Some(t) = ally_battle_stop_tick[i] { if t + tps*6 < tick { = None } }   (i 별 교대로 실행)
[L338] if let Some(t) = obj_spawn.serpen_giveup_tick(+0x60) { [L339] if t + tps*20 < tick { [L340] = None } }
[L344] if let Some(t) = obj_spawn.epic_giveup_tick(+0x50)   { [L345] if t + tps*20 < tick { [L346] = None } }
       // ⚠IR: L344 검사는 L338 의 Some 분기 안(%109 는 %98·%112 에서만 도달)… 정확히는 %666 에서 serpen None 이면 %109 로 직접 감 → 결국 두 검사 모두 항상 실행됨

// ── 웨이브 우선 클리어 라인 (L350 → update_wave_priority_clear_line 540~559 인라인)
[L541] if player.strategy(rnd, game).minion_wave(+0x10) == WavePriority(0) {
[L546]   if let Some(line) = self.wave_priority_clear_line(+0x41d) {
[L547]     if rule_scope::line_exists(context, line) {
[L551]       if blackboard[team].<line>_minion_state.minion_count(+0x20/+0x48/+0x70) > 0 {
[L552]         self.wave_priority_clear_line = None; [L558] = find_wave_priority_clear_line(player, data) }
             else { 유지 }
           } else { self.wave_priority_clear_line = None }          // 라인 자체가 룰스코프 밖
[L558]   } else { self.wave_priority_clear_line = find_wave_priority_clear_line(player, data) }
       } else { self.wave_priority_clear_line = None }

[L351] position = player.info.position(+0x9c0)
       if blackboard[team].big_goal[position].1 == Some(BigGoal::Battle{focus: Some(_)}) { [L352] self.last_battle_tick(+0x220) = tick }

// ── 오브젝트 캠프 가시 틱 (L355~369)
[L355] match tutorial { 0|7|8 => {
[L356]   epic_camp = camp_pos(map, Morgard(4), team==0); [L359] if is_visible_cell(team, x/32000, y/32000) { [L360] epic_camp_last_visible_tick(+0x80) = tick } ; fallthrough to serpen }
         5 => serpen only, _ => skip }
[L365]   serpen_camp = camp_pos(map, Serpen(5), team==0); [L368] if is_visible_cell(..) { [L369] serpen_camp_last_visible_tick(+0x88) = tick }

// ── 스폰 콜 채팅 (L374~392) — 서포터만, 10틱마다
[L374] if position == Support(4) && tick % 10 == 0 {
[L375]   tps1 = max(tps, 1)
[L376]   remain_tick = moba.map_or(0, |m| m.epic.next_respawn_tick(+0x1b0)).saturating_sub(tick)
[L378]   if tutorial ∉ 1..=6 && moba.map_or(false, |m| m.epic.live_list.len(+0x1a8) == 0) {
[L379]     if moba.epic.next_respawn_tick.saturating_sub(self.epic_spawn_call_tick(+0x70)) > tps*20 && remain_tick <= tps*20 {
[L381]       self.epic_spawn_call_tick = tick; [L382] remain_second = remain_tick / tps1
[L383]       self.chats.push(Chat::MorgardPrepare(remain_second, 0)) } }
[L386]   remain_tick = moba.map_or(0, |m| m.serpen.next_respawn_tick(+0x1e0)).saturating_sub(tick)
[L387]   if tutorial ∈ {0,5,7,8} && moba.serpen.live_list.len(+0x1d8) == 0 {
[L388]     if serpen.next_respawn_tick.saturating_sub(self.serpen_spawn_call_tick(+0x78)) > tps*20 && remain_tick <= tps*20 {
[L390]       self.serpen_spawn_call_tick = tick; [L391] remain_second = remain_tick / tps1
[L392]       self.chats.push(Chat::SerpenPrepare(remain_second, 0)) } } }

// ── 적 시야 추적 + MIA 콜 (L397~442)
[L397] my_champ = cache.player_champion[team][position]; [L399] if None → return
[L400] for p in game.iter_player() {                              // &[PlayerState] stride 2528
[L401]   pteam = p.info.team; c = cache.player_champion[pteam][p.position]
[L403]   if pteam != team {                                        // 적만 갱신
[L404]     if let Some(c) = c {
[L406]       if c.is_visible_from(my_champ) {   // my_champ.team==Neutral || c.visible_state[my_champ.team].tag==Visible(0)
[L407~411]     last_visible_ticks[pos]=tick; last_checked_ticks[pos]=tick; last_visible_distance[pos]=distance_sq(c,my_champ);
               last_visible_pos[pos]=(c.x,c.y); last_hp_ratio[pos]=c.hp*100/c.stat_cached.hp }
           } else {                                                // 적 챔프 없음(사망)
[L414~416]   last_visible_ticks[pos]=tick; last_checked_ticks[pos]=tick; last_visible_distance[pos]=10^12
[L418]       home = cache.nexus[pteam](+0x170).map(|n| (n.x,n.y)).unwrap_or(map.nexus_pos[pteam](+0x6d50))
[L420]       last_visible_pos[pos] = home } }
       // ⚠아군(pteam==team)도 아래 MIA 검사는 통과한다 — 단 vision[pos] 는 적 pos 로만 기록되므로 같은 pos 의 적 데이터로 검사됨
[L425]   if self.last_call_tick(+0x3f0) + tps < tick {
[L426]     lv = last_visible_ticks[pos]; if lv != 0 {
[L428]       if lv + tps*4 < tick {
[L429]         if last_hp_ratio[pos] > 59 {
[L430]           if last_visible_distance[pos] < 28900000000 (170000^2) {
[L431]             if mia_call_ticks[pos](+0x280) < lv + tps*4 {
[L433]               mia_call_ticks[pos] = tick; [L435] last_call_tick = tick
[L436]               self.chats.push(Chat::Mia(p.position, 0)) } } } } } }
     }
[L442] return
```

**`mem` 메모리 접근 75건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | OperationData | 0x8 | context | r | &GameContext | 4 | OK |  |
| 1 | OperationData | 0x0 | cache | r | &AbstractGameWithCache. +0x0/+0x8 = &dyn AbstractGame(data, vtable) | 4 | OK |  |
| 2 | OperationData | 0x10 | blackboard | r | &[Blackboard;2]. [team] 만 읽음 | 4 | OK |  |
| 3 | GameContext | 0x38 | tutorial (TutorialType 태그) | r | IR 56. L355·L364·L378·L387 게이트 | 4 | OK |  |
| 4 | GameContext | 0x20 | map | r | IR 32. &MapDef — camp_pos·nexus_pos | 4 | OK |  |
| 5 | GameContext | 0x8 | setting | r | &GameSetting | 4 | OK |  |
| 6 | GameSetting | 0x12f8 | tick_per_second | r | IR 4856. 모든 시간 임계의 단위 | 4 | OK |  |
| 7 | dyn AbstractGame vtable | 0x28 | tick | r | divtable 슬롯 0x28 | 3 | 확인불가(vtable 슬롯(구조체 아님) — divtable.py 소관) |  |
| 8 | dyn AbstractGame vtable | 0x40 | get_game_mode | r | divtable 슬롯 0x40 → GameMode(16B). 태그 0=Moba, +0x8=&MobaMode | 3 | 확인불가(vtable 슬롯(구조체 아님) — divtable.py 소관) |  |
| 9 | dyn AbstractGame vtable | 0x100 | is_visible_cell(team, xi, yi) | r | divtable 슬롯 0x100. 캠프 셀(좌표/32000) 가시성 | 3 | 확인불가(vtable 슬롯(구조체 아님) — divtable.py 소관) |  |
| 10 | dyn AbstractGame vtable | 0x208 | iter_player | r | divtable 슬롯 0x208 → &[PlayerState] (ptr,end), stride 2528 | 3 | 확인불가(vtable 슬롯(구조체 아님) — divtable.py 소관) |  |
| 11 | MobaMode | 0x18 | jungle_runner | r | IR 24. JungleRunner::get_camp_state(&jungle_runner, team, JungleType) 의 self | 4 | OK |  |
| 12 | MobaMode | 0x1a8 | jungle_runner.epic.live_list.len | r | IR 424. 0 이면 에픽 미생존 | 4 | OK |  |
| 13 | MobaMode | 0x1b0 | jungle_runner.epic.next_respawn_tick | r | IR 432 | 4 | OK |  |
| 14 | MobaMode | 0x1d8 | jungle_runner.serpen.live_list.len | r | IR 472 | 4 | OK |  |
| 15 | MobaMode | 0x1e0 | jungle_runner.serpen.next_respawn_tick | r | IR 480 | 4 | OK |  |
| 16 | JungleCampState | 0x10 | live_list.len | r | IR +16. get_camp_state 반환 캠프의 생존 수 (L317) | 4 | OK |  |
| 17 | JungleCampState | 0x18 | (init) +24 필드 | r | IR +24. init(L288~289) 이 next_respawn_tick[team][idx] 초기값으로 복사 — tcxdict JungleCampState 48B 의 +0x18 이름은 조회 안 함(unknown) | 3 | OK |  |
| 18 | JungleInfo | 0x100 | respawn_tick | r | IR +256. JungleType::get_info(&jungle, setting) 272B 반환 중. next_respawn_tick = tick + 이 값 (L320) | 4 | OK |  |
| 19 | PlayerState | 0x930 | info.team | r | IR 2352. >=2 면 panic_bounds_check | 4 | OK |  |
| 20 | PlayerState | 0x9c0 | info.position@tag (i32) | r | IR 2496. Position: 0 Top/1 Jungle/2 Mid/3 Bottom/4 Support | 4 | OK |  |
| 21 | TeamStrategy | 0x10 | minion_wave@tag | r | PlayerState::strategy 반환(24B) +16. 0=WavePriority 일 때만 wave_priority_clear_line 유지/탐색 | 4 | OK |  |
| 22 | Blackboard | 0x20 | top_minion_state.minion_count (i32) | r | Top +0x20 / Mid +0x48 / Bottom +0x70. > 0 이면 wave_priority_clear_line 해제(L551) | 4 | OK |  |
| 23 | Blackboard | 0xf0 | big_goal[position].1 (Option<BigGoal>) | r | IR 240+32p+8 태그==5(Battle), +16 = Battle.focus Option<usize> 태그 != 0(Some) → last_battle_tick 갱신(L351) | 4 | OK |  |
| 24 | AbstractGameWithCache | 0x1e0 | player_champion | r | IR 480. 내 챔프·적 챔프 포인터 | 4 | OK |  |
| 25 | AbstractGameWithCache | 0x170 | nexus[] | r | IR 368. 죽은 적의 홈 좌표(넥서스 x,y). None 이면 map.nexus_pos[team] | 4 | OK |  |
| 26 | MapDef | 0x6d50 | nexus_pos[team] (u64,u64) | r | IR 27984 + 16*team | 4 | OK |  |
| 27 | Entity | 0x0 | team@tag (TeamType) | r | 내 챔프. 1=Neutral 이면 is_visible_from 무조건 true (entity.rs:1482 인라인) | 4 | OK |  |
| 28 | Entity | 0x8 | team@Player.0 | r | 내 챔프의 팀 인덱스 → 적 챔프 visible_state[team] | 4 | OK |  |
| 29 | Entity | 0x38 | visible_state | r | IR 56 + 24*team. 0=Visible 이면 보임 (data.rs:122 인라인) | 4 | OK |  |
| 30 | Entity | 0x660 | x | r | IR 1632 | 4 | OK |  |
| 31 | Entity | 0x668 | y | r | IR 1640 | 4 | OK |  |
| 32 | Entity | 0x670 | hp | r | IR 1648 | 4 | OK |  |
| 33 | Entity | 0x628 | stat_cached.hp | r | IR 1576. 0 이면 div_by_zero 패닉 | 4 | OK |  |
| 34 | TeamPlan | 0x41c | is_init | r | IR 1052. false 면 init 인라인 실행 | 4 | OK |  |
| 35 | TeamPlan | 0x149 | objective_discipline.kind (kind 니치) | r | IR 329. 2=None | 4 | OK |  |
| 36 | TeamPlan | 0x140 | objective_discipline.until_tick | r | IR 320 | 4 | OK |  |
| 37 | TeamPlan | 0x378 | next_respawn_tick[team][idx] | r | IR 888 + 32*team + 8*idx. idx 0 Rhino/1 Mushroom/2 Bee/3 Stump | 4 | OK |  |
| 38 | TeamPlan | 0x1c8 | allies[i]@tag (region 니치, -1=None) | r | IR 456+16i | 4 | OK |  |
| 39 | TeamPlan | 0x1c0 | allies[i].last_tick | r | IR 448+16i | 4 | OK |  |
| 40 | TeamPlan | 0x0 | ally_battle_stop_tick[] | r | IR 0+16i, 값 +8 | 4 | OK |  |
| 41 | TeamPlan | 0x60 | obj_spawn.serpen_giveup_tick (tag +0x60, val +0x68) | r | IR 96/104 | 4 | OK |  |
| 42 | TeamPlan | 0x50 | obj_spawn.epic_giveup_tick@tag (tag +0x50, val +0x58) | r | IR 80/88 | 4 | OK |  |
| 43 | TeamPlan | 0x41d | wave_priority_clear_line (Option<LineType>, -1=None) | r | IR 1053 | 4 | OK |  |
| 44 | TeamPlan | 0x70 | obj_spawn.epic_spawn_call_tick | r | IR 112 | 4 | OK |  |
| 45 | TeamPlan | 0x78 | obj_spawn.serpen_spawn_call_tick | r | IR 120 | 4 | OK |  |
| 46 | TeamPlan | 0x3f0 | last_call_tick | r | IR 1008 | 4 | OK |  |
| 47 | TeamPlan | 0x2a8 | vision.last_visible_ticks[pos] | r | IR 680+8p | 4 | OK |  |
| 48 | TeamPlan | 0x320 | vision.last_hp_ratio[pos] | r | IR 800+8p | 4 | OK |  |
| 49 | TeamPlan | 0x2f8 | vision.last_visible_distance[pos] | r | IR 760+8p | 4 | OK |  |
| 50 | TeamPlan | 0x280 | vision.mia_call_ticks[pos] | r | IR 640+8p | 4 | OK |  |
| 51 | TeamPlan | 0xc0 | chats.cap | r | IR 192/200/208. push 시 len==cap 이면 grow_one | 4 | OK |  |
| 52 | TeamPlan | 0x41c | is_init | w | L282(init 인라인). !is_init 일 때 1회 | 4 | OK | true |
| 53 | TeamPlan | 0x378 | next_respawn_tick[team][idx] · [1-team][idx] | w | IR 888+32t+8idx. idx 순서 = jungle [Rhino0, Mushroom1, Bee3, Stump2]. L320 은 캠프 셀이 보이고 tick>=기록값이고 캠프 생존수 0 일 때 | 4 | OK | init: get_camp_state(jr, t, jungle).+0x18 / L320: tick + JungleType::get_info(jungle, setting).respawn_tick(+0x100) |
| 54 | TeamPlan | 0x149 | objective_discipline.kind | w | L302: Some 이고 until_tick(+0x140) <= tick | 4 | OK | None (store i8 2) |
| 55 | TeamPlan | 0x1c8 | allies[i] (i=0..4) | w | L328: Some 이고 last_tick + tps*10 < tick | 4 | OK | None (store i8 -1 at +0x1c8+16i) |
| 56 | TeamPlan | 0x0 | ally_battle_stop_tick[] (i=0..4) | w | L333: Some(t) 이고 t + tps*6 < tick | 4 | OK | None (store i64 0 at +16i) |
| 57 | TeamPlan | 0x60 | obj_spawn.serpen_giveup_tick | w | L340: Some(t) 이고 t + tps*20 < tick | 4 | OK | None (store i64 0) |
| 58 | TeamPlan | 0x50 | obj_spawn.epic_giveup_tick@tag | w | L346: serpen_giveup 이 Some 이던 경우에만 검사됨(IR 분기: L344 는 %109 에서 +0x60 태그 재로드). Some(t) 이고 t + tps*20 < tick | 4 | OK | None (store i64 0) |
| 59 | TeamPlan | 0x41d | wave_priority_clear_line | w | L541~559 update_wave_priority_clear_line 인라인. 아래 logic | 4 | OK | -1(None) / find_wave_priority_clear_line(player, data) 결과 / 유지 |
| 60 | TeamPlan | 0x220 | last_battle_tick | w | L352: blackboard[team].big_goal[position].1 == Some(Battle{focus:Some}) | 4 | OK | tick |
| 61 | TeamPlan | 0x80 | obj_spawn.epic_camp_last_visible_tick | w | L360: tutorial ∈{0,7,8} 이고 is_visible_cell(team, epic_camp/32000) | 4 | OK | tick |
| 62 | TeamPlan | 0x88 | obj_spawn.serpen_camp_last_visible_tick | w | L369: tutorial ∈{0,5,7,8} 이고 is_visible_cell(team, serpen_camp/32000) | 4 | OK | tick |
| 63 | TeamPlan | 0x70 | obj_spawn.epic_spawn_call_tick | w | L381: 에픽 콜 발행 시 | 4 | OK | tick |
| 64 | TeamPlan | 0xc0 | chats.push(Chat::MorgardPrepare(remain_second, 0)) | w | L383 | 4 | 부분일치(첫 성분(컨테이너)만 맞고 **마지막 성분이 다르다** — 사람이 봐야 ) | tag 33, +0x8 remain_second, +0x10 0 |
| 65 | TeamPlan | 0x78 | obj_spawn.serpen_spawn_call_tick | w | L390 | 4 | OK | tick |
| 66 | TeamPlan | 0xc0 | chats.push(Chat::SerpenPrepare(remain_second, 0)) | w | L392 | 4 | 부분일치(첫 성분(컨테이너)만 맞고 **마지막 성분이 다르다** — 사람이 봐야 ) | tag 24, +0x8 remain_second, +0x10 0 |
| 67 | TeamPlan | 0x2a8 | vision.last_visible_ticks[p.position] | w | L407(보이는 적) · L414(죽은 적) | 4 | OK | tick |
| 68 | TeamPlan | 0x2d0 | vision.last_checked_ticks[p.position] | w | L408 · L415 | 4 | OK | tick |
| 69 | TeamPlan | 0x2f8 | vision.last_visible_distance[p.position] | w | IR 760+8p | 4 | OK | distance_sq(c, my_champ) (L409) / 1000000000000 (L416, 죽은 적) |
| 70 | TeamPlan | 0x230 | vision.last_visible_pos[p.position] | w | IR 560+16p | 4 | OK | (c.x, c.y) (L410) / home = nexus[team].xy 또는 map.nexus_pos[team] (L420) |
| 71 | TeamPlan | 0x320 | vision.last_hp_ratio[p.position] | w | L411. 죽은 적은 갱신 안 함 | 4 | OK | c.hp*100 / c.stat_cached.hp |
| 72 | TeamPlan | 0x280 | vision.mia_call_ticks[p.position] | w | L433: MIA 콜 발행 시 | 4 | OK | tick |
| 73 | TeamPlan | 0x3f0 | last_call_tick | w | L435 | 4 | OK | tick |
| 74 | TeamPlan | 0xc0 | chats.push(Chat::Mia(p.position, 0)) | w | L436 | 4 | 부분일치(첫 성분(컨테이너)만 맞고 **마지막 성분이 다르다** — 사람이 봐야 ) | tag 1, +0x4 position(i32), +0x8 0 |

**`consts` 상수 30건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev | folded_from |
|---|---|---|---|---|---|---|
| 0 | 0 | 285 | 태그 | GameMode 태그 0 = Moba. init 은 Moba 가 아니면 리스폰 초기화를 건너뜀(패닉 없음) | 4 |  |
| 1 | 2 | 301 | 센티널 | objective_discipline 니치 태그 2 = None (kind 0 SafeWait/1 HardDisengage) | 4 |  |
| 2 | 32000 | 309 | 인덱스 | 셀 크기 — 캠프 좌표 → 셀 인덱스(xi, yi) 변환(L309·310·357·358·366·367) | 4 |  |
| 3 | 0 | 317 | 임계 | get_camp_state(..).live_list.len == 0 (캠프 비어 있음) 일 때만 next_respawn_tick 재계산 | 4 |  |
| 4 | 10 | 327 | 계수 | allies[i].last_tick + tps*10 < tick 이면 allies[i]=None (10초 타임아웃) | 4 |  |
| 5 | -1 | 326 | 센티널 | allies[i] 의 AllyRegion 니치 태그 -1(0xff) = None | 4 |  |
| 6 | 6 | 332 | 임계 | ally_battle_stop_tick[i] + tps*6 < tick 이면 None (6초 타임아웃) | 4 |  |
| 7 | 20 | 339 | 계수 | serpen_giveup_tick + tps*20 < tick 이면 None (L345 epic 도 동일 20초) | 4 |  |
| 8 | 0 | 541 | 태그 | MinionWaveStrategy 태그 0 = WavePriority. 이때만 wave_priority_clear_line 을 유지·탐색, 아니면 None | 4 |  |
| 9 | 0 | 551 | 임계 | blackboard[team].<line>_minion_state.minion_count > 0 이면 현재 clear_line 해제 후 재탐색 | 4 |  |
| 10 | 5 | 351 | 태그 | BigGoal 태그 5 = Battle. big_goal[position].1 이 Battle{focus: Some} 이면 last_battle_tick=tick | 4 |  |
| 11 | 4 | 356 | 태그 | JungleType 4 = Morgard(에픽) camp_pos | 4 |  |
| 12 | 5 | 365 | 태그 | JungleType 5 = Serpen camp_pos | 4 |  |
| 13 | 4 | 374 | 태그 | Position 태그 4 = Support. 서포터만 에픽/세르펜 스폰 콜을 낸다 | 4 |  |
| 14 | 10 | 374 | 계수 | tick % 10 == 0 일 때만 스폰 콜 검사 | 4 |  |
| 15 | 1 | 375 | 태그 | tick_per_second = max(setting.tps, 1) — 0 나눗셈 방지 | 4 |  |
| 16 | 6 | 378 | 임계 | tutorial (tag-1) < 6 ⟺ 1..=6 이면 에픽 콜 안 함 (None/Line/Total 만) | 4 |  |
| 17 | 20 | 379 | 계수 | 에픽: next_respawn_tick - epic_spawn_call_tick > tps*20 (마지막 콜로부터 20초) && remain_tick <= tps*20 (20초 이내 스폰) 이면 콜 | 4 |  |
| 18 | 33 | 383 | 태그 | Chat 태그 33 = MorgardPrepare(remain_second, 0) | 4 |  |
| 19 | 20 | 388 | 계수 | 세르펜 콜 동일 조건 (tps*20) | 4 |  |
| 20 | 24 | 392 | 태그 | Chat 태그 24 = SerpenPrepare(remain_second, 0) | 4 |  |
| 21 | 1 | 406 | 태그 | 내 챔프 TeamType 태그 1 = Neutral 이면 is_visible_from 무조건 true | 4 |  |
| 22 | 0 | 406 | 태그 | 적 챔프 visible_state[my_team] 태그 0 = Visible | 4 |  |
| 23 | 100 | 411 | 계수 | last_hp_ratio = hp*100/max_hp | 4 |  |
| 24 | 1000000000000 | 416 | 산출값 | 죽은(None) 적의 last_visible_distance = 10^12 (사실상 무한) | 4 |  |
| 25 | 1 | 436 | 태그 | Chat 태그 1 = Mia(position, 0) | 4 |  |
| 26 | 2 | 428 | 임계 | tps*4 가 `shl i64 %396, 2` 로 접힘. last_visible_ticks[p] + tps*4 < tick (4초 이상 미관측) 및 mia_call_ticks[p] < last_visible_ticks[p] + tps*4 | 4 | 4 |
| 27 | 59 | 429 | 임계 | last_hp_ratio[p] > 59 (60% 이상) 인 적만 MIA 콜 | 4 |  |
| 28 | 28900000000 | 430 | 임계 | 170000^2 — 마지막 관측 거리(제곱) < 170000 인 적만 MIA 콜 | 4 |  |
| 29 | 0 | 426 | 임계 | last_visible_ticks[p] == 0 (한 번도 관측 안 됨) 이면 MIA 콜 안 함 | 4 |  |

**`knobs` 조정점 11건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | allies[i] 지역정보 유효 시간 | team_plan.rs:327 (m09.ll:39282 `mul i64 %517, 10`) | 10 | tps*10 초 지나면 아군 지역 기록 폐기. 올리면 오래된 위치 정보를 더 오래 신뢰 | 4 | 기존 |
| 1 | ally_battle_stop_tick 유효 시간 | team_plan.rs:332 (m09.ll:39305 `mul i64 %532, 6`) | 6 | 6초 지나면 '전투 중지' 표식 해제 | 4 | 기존 |
| 2 | 오브젝트 포기(giveup) 유효 시간 | team_plan.rs:339·345 (m09.ll:38403·38426 `mul .. 20`) | 20 | serpen/epic giveup 표식이 20초 뒤 자동 해제 → 다시 오브젝트 시도 가능. 올리면 포기 후 더 오래 재시도 안 함 | 4 | 기존 |
| 3 | 스폰 콜 재발행 간격 · 사전 예고 시간 | team_plan.rs:379·388 (m09.ll:38725·38868 `mul i64 %220, 20`) | 20 | 다음 스폰 20초 이내 & 마지막 콜로부터 20초 경과 시 MorgardPrepare/SerpenPrepare 채팅. 내리면 스폰 직전에만 콜 | 4 | 기존 |
| 4 | 스폰 콜 담당 포지션 | team_plan.rs:374 (m09.ll:38627 `icmp eq i32 %162, 4`) | 4 | Support(4) 플레이어의 TeamPlan 만 콜 발행. 바꾸면 다른 포지션이 콜 | 4 | 기존 |
| 5 | 스폰 콜 검사 주기 | team_plan.rs:374 (m09.ll:38625 `urem i64 %212, 10`) | 10 | 10틱마다 검사 | 4 | 기존 |
| 6 | MIA 콜 미관측 시간 | team_plan.rs:428·431 (m09.ll:39198 `shl i64 %396, 2` = tps*4) | 4 | 마지막 관측 후 4초 지나야 MIA 채팅. 내리면 잠깐 사라져도 MIA | 4 | 기존 |
| 7 | MIA 콜 HP 하한(%) | team_plan.rs:429 (m09.ll:39208 `icmp ugt i64 %486, 59`) | 59 | 마지막 관측 HP 60% 이상인 적만 MIA (빈사 적은 리콜로 간주). 내리면 더 자주 MIA | 4 | 기존 |
| 8 | MIA 콜 거리 상한 | team_plan.rs:430 (m09.ll:39215 `28900000000`) | 28900000000 | 170000(≈5.3셀) 안에서 보였던 적만 MIA. 올리면 먼 적도 MIA 대상 | 4 | 기존 |
| 9 | 팀 콜 최소 간격 | team_plan.rs:425 (m09.ll `%397 = tps + last_call_tick; ult %397, tick`) | 1 | last_call_tick + tps*1 < tick — 1초에 1회만 MIA 콜. 접힌 계수(×1)라 리터럴 없음 | 4 | 기존 |
| 10 | 죽은 적의 관측 거리 초기값 | team_plan.rs:416 (m09.ll:39041) | 1000000000000 | 10^12 로 세팅 → 거리 상한(170000^2) 을 넘어 MIA 콜 대상에서 제외. 내리면 리스폰 직후 MIA 오발 | 4 | 기존 |

<details><summary>`callees` 피호출자 29건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | camp_pos | game_core::MapDef::camp_pos | pub | fn(&game_core::MapDef, game_core::JungleType, bool) -> (u64, u64) | game-core\src\simulation\map_def.rs:207 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | distance_sq | game_core::utils::distance_sq | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:6 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 2 | distance_sq | game_core::Entity::distance_sq | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2157 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 3 | find_wave_priority_clear_line | game_ai::plan_legacy::team_plan::objective_helpers::find_wave_priority_clear_line | in:game_ai | fn(&game_core::PlayerState, &game_core::OperationData) -> std::option::Option<game_core::LineType> | game-ai\src\plan_legacy\team_plan\objective_helpers.rs:209 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | get_camp_state | game_core::JungleRunner::get_camp_state | pub | fn(&game_core::JungleRunner, usize, game_core::JungleType) -> &game_core::JungleCampState | game-core\src\simulation\entity\jungle.rs:702 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 5 | get_game_mode | game_core::AbstractGame::get_game_mode | pub | fn(&Self/#0) -> game_core::GameMode | game-core\src\simulation.rs:78 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 6 | get_game_mode | <game_core::Game as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::Game) -> game_core::GameMode | game-core\src\simulation\game.rs:1707 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 7 | get_game_mode | <game_core::SingleLaneGame as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::SingleLaneGame) -> game_core::GameMode | game-core\src\simulation\game.rs:4248 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 8 | get_info | game_core::JungleType::get_info | pub | fn(&game_core::JungleType, &game_core::GameSetting) -> game_core::JungleInfo | game-core\src\simulation\entity\jungle.rs:405 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 9 | is_visible_cell | game_core::AbstractGame::is_visible_cell | pub | fn(&Self/#0, usize, usize, usize) -> bool | game-core\src\simulation.rs:126 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 10 | is_visible_cell | <game_core::Game as game_core::AbstractGame>::is_visible_cell | pub | fn(&game_core::Game, usize, usize, usize) -> bool | game-core\src\simulation\game.rs:1804 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 11 | is_visible_cell | <game_core::SingleLaneGame as game_core::AbstractGame>::is_visible_cell | pub | fn(&game_core::SingleLaneGame, usize, usize, usize) -> bool | game-core\src\simulation\game.rs:3869 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 12 | is_visible_from | game_core::Entity::is_visible_from | pub | fn(&game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1481 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 13 | iter_player | game_core::AbstractGame::iter_player | pub | fn(&Self/#0) -> game_core::PlayerIter | game-core\src\simulation.rs:181 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 14 | iter_player | <game_core::Game as game_core::AbstractGame>::iter_player | pub | fn(&game_core::Game) -> game_core::PlayerIter | game-core\src\simulation\game.rs:3756 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 15 | iter_player | <game_core::SingleLaneGame as game_core::AbstractGame>::iter_player | pub | fn(&game_core::SingleLaneGame) -> game_core::PlayerIter | game-core\src\simulation\game.rs:4015 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 16 | jungle_runner | game_core::MobaMode::jungle_runner | pub | fn(&game_core::MobaMode) -> &game_core::JungleRunner | game-core\src\simulation\game.rs:213 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 17 | jungle_runner | game_core::AbstractGame::jungle_runner | pub | fn(&Self/#0) -> &game_core::JungleRunner | game-core\src\simulation.rs:119 | True | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 18 | line_exists | game_ai::plan_legacy::rule_scope::line_exists | pub | fn(&game_core::GameContext, game_core::LineType) -> bool | game-ai\src\plan_legacy\rule_scope.rs:24 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 19 | position | game_core::PlayerAiContext::<'a, 'b, 'r>::position | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> game_core::Position | game-core\src\mod_ai.rs:587 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 20 | position | game_core::transfer::TeamRosterPlanningContext::position | pub | fn(&game_core::transfer::TeamRosterPlanningContext, game_core::Position) -> std::option::Option<&game_core::transfer::PositionPlanningContext> | game-core\src\transfer\roster_context.rs:234 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 21 | push | <game_core::Staff as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::staff | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\staff.rs:38 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 22 | push | <game_core::Athlete as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:434 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 23 | push | <game_core::Contract as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:2498 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 24 | sanitize_rule_scope | game_ai::plan_legacy::team_plan::TeamPlan::sanitize_rule_scope | pub | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, &game_core::GameContext) | game-ai\src\plan_legacy\team_plan.rs:200 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 25 | strategy | game_core::PlayerState::strategy | pub | fn(&game_core::PlayerState, &mut rand::rngs::std::StdRng, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + ) -> game_core::Strategy | game-core\src\simulation\state\player.rs:1576 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 26 | update | game_ai::GoalData::update | pub | fn(&mut game_ai::GoalData, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) | game-ai\src\goal_data.rs:82 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 257개 중 상위 3개 |
| 27 | update | game_ai::EpicStanceData::update | pub | fn(&mut game_ai::EpicStanceData, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &[std::option::Option<game_ai::EnemyRegionInfo>; 5_usize], &mut game_core::DebugFrameData) | game-ai\src\goal_data.rs:158 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 257개 중 상위 3개 |
| 28 | update | game_ai::plan_legacy::types::BigPlan::update | pub | fn(&mut game_ai::plan_legacy::types::BigPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PositioningScoreData, &mut game_core::DebugFrameData) | game-ai\src\plan_legacy\types.rs:198 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 257개 중 상위 3개 |
</details>

⚠**미매칭 19개**: `drop  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `epic_camp_last_visible_tick`, `epic_giveup_tick`, `epic_spawn_call_tick`, `grow_one`, `is_init`, `last_battle_tick`, `last_call_tick`, `map_or`, `minion_count`, `minion_wave`, `next_respawn_tick`, `objective_discipline`, `respawn_tick`, `serpen_camp_last_visible_tick`, `serpen_giveup_tick`, `serpen_spawn_call_tick`, `until_tick`, `wave_priority_clear_line`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 4곳** (m13.ll:10505, m13.ll:15484, m13.ll:16507, m13.ll:19024) · **형제 55개** (TeamPlan)

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

**`open` 11건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | sanitize_rule_scope(self, context) 내부는 안 봄 — self 를 &mut 로 받으므로 여기서도 TeamPlan 이 바뀔 수 있다(writes 미포함) | 4 |  |
| 1 | 미탐색 | JungleCampState +0x18 (init 이 next_respawn_tick 초기값으로 복사하는 필드) 의 이름은 tcxdict 로 조회 안 함. IR 은 get_camp_state 반환 ptr+24 의 i64 로드 | 3 |  |
| 2 | 미탐색 | JungleType::get_info 반환 272B 구조체의 이름을 'JungleInfo' 로 적었으나 tcxdict 조회는 +0x100=respawn_tick 만 확인 — 구조체 이름은 추정 | 3 |  |
| 3 | 미탐색 | is_visible_cell 의 vtable 로드(%682)가 L311 블록에 있어 L359·L368 에서 재사용된다 — L305 루프가 항상 먼저 실행되므로 정의는 도달 보장. 시그니처 (game, team, xi, yi) → bool 은 호출 형태로만 확정 | 4 |  |
| 4 | 미탐색 | L344(epic_giveup) 검사 도달: IR 상 %666 → serpen Some 이면 %98 → (%112) → %109, None 이면 %109 직행. 즉 두 giveup 검사는 독립적으로 항상 실행된다고 읽었음(L338~346 이 두 개의 독립 if let) | 4 |  |
| 5 | 미탐색 | MIA 검사(L425~436)가 아군(pteam==team)에서도 실행되는 것은 IR 분기(%383: team==my → %392 직행)로 확정. 소스에서 의도된 것인지(if 블록 밖에 둔 것)는 알 수 없음. vision 배열은 적 pos 로만 기록되므로 아군 pos 와 같은 pos 의 적 데이터로 검사됨 — 효과: 같은 틱에 같은 pos 를 두 번 검사(둘째는 mia_call_ticks 갱신으로 차단) | 4 |  |
| 6 | 미탐색 | camp_pos 의 bool 인자 = (team==0). L305 루프에서는 순회 중인 team(0→true, 1→false), L356/365 에서는 내 team. 의미는 미확인 | 4 |  |
| 7 | 미탐색 | Chat::MorgardPrepare/SerpenPrepare 의 두 번째 필드(0 고정)의 의미 미확인. Mia 의 +0x8 필드(0) 도 동일 | 4 |  |
| 8 | 미탐색 | PlayerState::strategy(player, rnd, game) 내부는 안 봄(_gcbc g15.ll:130480). 반환 TeamStrategy 24B 의 +0x10 minion_wave 만 읽음 | 4 |  |
| 9 | 미탐색 | find_wave_priority_clear_line / rule_scope::line_exists 내부는 안 봄 | 4 |  |
| 10 | 미탐색 | version(p2)·debug(p6) 은 본문에서 안 쓰임(debug 는 readnone 속성) | 4 |  |

<details><summary>`closed` 1건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

| # | 물음 | 닫은 근거 |
|---|---|---|
| 0 | L314 비교 `tick < next_respawn_tick → skip` 은 `tick >= next_respawn_tick` 로 표기. == 경계는 IR 로 확정(ult) | 본문에 해소 표기가 있다 |
</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

