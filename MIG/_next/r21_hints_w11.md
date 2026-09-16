# r21 w11 — 미발견 잔여 2 짝 찾기 + 콜리 실측으로 짝이 드러난 9 의 본체 대조

## A. 짝 미확정 2

- `ca6700` get_small_action_score_closure(명세 #263 · get_small_action 안의 score 클로저 · 0.5.8 5.3KB · Location 0) — 0.6.0 에서 인라인됐거나 다른 RVA. get_small_action d6c2b0 의 콜리(w6: score=ed3dd0 · choose=e59c80 · clone=d7c3d0)와 명세 logic 을 대조해 어디로 갔는지 확정.
- `e49a50` LegacyPlanHandler::update_on_dead(0.5.8 6.2KB · Location 0) — LPH 사망 처리. 0.6.0 LPH 콜러(f3adb0 update_state · d5a940 래퍼 · d3d210) 에서 호출부 찾기 또는 Location/문자열(`update_on_dead` · dive episode) 로.

## B. 짝 확정 9(콜리 실측) — 본체 대조 필요(동치/변경 판정 · 등가 Rust · v2/v3)

### `d40b20` → `fafe50` best_jungle_goal (r21 w3/w6: best_jungle_goal v3(ABI 변경 · 골격만 독해))
- 0.5.8 명세 #19 `passive_jungle__best_jungle_goal` src game-ai\src\plan_legacy\old\passive_jungle.rs:806 · one_line: 정글러가 다음에 갈 캠프(JungleType)를 고른다 — 미클리어 캠프 중 내 챔프에서 가장 가까운 것. ★출구는 셋이다: ①본선 = 미클리어 캠프 중 제곱거리 최소 ②전부 클리어(817) = `next_respawn_tick` 이 가장 빠른 캠프(**이미 잡힌 캠프가 나온다**) ③내 챔프 엔티티가 없음(824) = `now_camp` 를 그대로(**Morgard/Serpen 도 가능**) 또는 4개 중 랜덤 — ②③은 미클리어 필터를 안 탄다
- 0.5.8 logic 전문:
```
fn best_jungle_goal(version, rnd, player, data, team_plan, now_camp, debug) -> JungleType

[807] jungle_camps: [JungleType;4] = [Rhino(0), Mushroom(1), Bee(3), Stump(2)]
// ※ 후보 배열에는 Morgard(4)/Serpen(5)가 없다. 다만 **반환값이 일반 캠프로 한정되지는 않는다** — 824~829 폴백이 `now_camp` 를 **그대로 돌려주므로** Morgard/Serpen 도 나올 수 있다 (실측 `champNone_nowcamp Some(Morgard) game=Morgard`). ⚠재구현에서 반환값을 일반 캠프로 필터링하면 **동작이 달라진다**.

[814-815] not_cleared_camps: bumpalo Vec<JungleType> =
 jungle_camps.into_iter()
 .filter(|c| !is_cleared(*c, player.info.team, _version, _rnd, player, data, team_plan, offset=0, _debug))
 .map(identity)
 .collect_in(data.context.pool)
 ※ filter 술어는 담당 범위 밖(call_mut 심 = m04.ll 66311~66341)에 있고,
 본체는 passive_jungle::is_cleared(passive_jungle.rs:841). 결과를 xor true 로 뒤집는다
 = '아직 안 잡힌 캠프만' 남긴다. offset 은 리터럴 0 으로 고정.

[817] if not_cleared_camps.is_empty() { // 전부 클리어된 상태. ★소스 형태는 `is_empty()` 다 — m04.ll:62643 의 !dbg 사슬이 `len<JungleType>`@vec.rs:1617 ← `is_empty<JungleType>`@vec.rs:1636 ← passive_jungle.rs:817 이다. IR 에서는 `load(len @+0x18)` + `icmp eq i64 …, 0` 으로 접힌다
[818] return jungle_camps.into_iter()
[819] .min_by_key(|c| data.cache.game.get_game_mode() // vtable +0x40, indirect call
                        .unwrap() // tag!=0 이면 option::unwrap_failed
                        .jungle_runner // MobaMode +0x18, 480B
                        .get_camp_state(player.info.team, *c).next_respawn_tick)
[821] .unwrap();
 // ★DWARF 정본(11차 배치D): vtable +0x40 간접호출(m04.ll:62706~62708)의 !dbg 는 scope 가
 // `closure$2 @passive_jungle.rs:819` 이고 inlinedAt 루트가 818 이다 ⟹ get_game_mode() 는
 // **키 클로저 안**에 있다(LLVM 이 루프 밖으로 호이스트했을 뿐, 818 의 `let` 이 아니다).
 // 체인 끝 `.unwrap()` 은 **821**(!71892 = option.rs:1011 ← 821).
 // ★즉 '가장 먼저 리스폰될 캠프'로 미리 간다. 동점이면 배열 앞쪽(Rhino→Mushroom→Bee→Stump)이 이긴다
 // (fold 가 cmp<Greater 일 때만 교체 = min_by 의 first-wins).
 }

[823] if player.info.team >= 2 { panic_bounds_check(team, 2) } // 배열 경계
 let champ: Option<&Entity> = data.cache.player_champion[player.info.team][player.info.position];
 // player_champion = AbstractGameWithCache +0x1e0, [[Option<&Entity>;5];2] (team stride 40B, position stride 8B)

[824] if champ.is_none() { // 내 챔프 엔티티가 없다(사망/미스폰 등)
[825] let camp = match now_camp {
 Some(c) => c, // 지금 가던 캠프를 그대로 유지
[829] None => *jungle_camps.choose(rnd).unwrap(), // 4개 중 랜덤(choose 가 None 이면 unwrap_failed)
 };
 return camp; // ※ not_cleared 필터를 전혀 안 탄다
 }

[832] let champ: &Entity = champ.unwrap();
[833] return not_cleared_camps.into_iter()
 .min_by_key(|c| {
[834] let (cx, cy) = data.context.map.camp_pos(*c, player.info.team == 0);
[835] let dx = abs_diff(cx, champ.x); // Entity +0x660
 let dy = abs_diff(cy, champ.y); // Entity +0x668
 dx*dx + dy*dy // 제곱거리 (sqrt 없음)
 })
 .unwrap();
 // ★본선 판정: 아직 안 잡힌 캠프 중 내 챔프에서 제곱거리가 최소인 것.
 // 동점이면 not_cleared_camps 의 앞 원소(=jungle_camps 배열 순서)가 이긴다.

// 부수효과 없음: 구조체 write 는 하나도 없고 지역 alloca(배열/Vec/IntoIter)만 쓴다.
// 예외 경로에서 Vec/RawVec/IntoIter 의 Drop 과 drop_glue 를 호출하는 것이 전부.
```

### `db8e60` → `fa9d60` LineGankerPlan::make_gank_battle (r21 w8: make_gank_battle(site 인자 추가 · 본체 미독))
- 0.5.8 명세 #155 `LineGankerPlan__make_gank_battle` src game-ai\src\plan_legacy\old\line_gank\ganker.rs:76 · one_line: 갱 대상에 대한 TryKill BattlePlan 을 만들어 BigPlan::Battle 로 반환 — v2+ 는 추격 가망 없음(open_chase_race_hopeless) 또는 첫 update 후 sub_goal 이 KitingBack/RunAway/End 면 None
- 0.5.8 logic 전문:
```
LineGankerPlan::make_gank_battle(&self /*IR 에서 제거*/, version, rnd, player, data, ps, team_plan, target, arg8, debug) -> Option<BigPlan>   [ganker.rs:76]
 if version > 1:                                                        (L79, 94368)
     me  = data.cache.player_champion[player.team][player.position]     (L80, 94404~94411)
     tgt = game.get_entity_by_id(target)                                (L81, 94412~94417, vtable+0x1f0)
     if me.is_some() && tgt.is_some() && open_chase_race_hopeless(version, data, player, me, tgt):   (L82, 94418~94427)
         return None                                                    (L83, 94430)   # 추격 경주 가망 없음
 plan = BattlePlan::new(version, BattlePlanGoal::TryKill(target, arg8), data, player)   (L87, 94379/94449)
 plan.entry_src = 7                                                     (L88, 94382/94452)
 if version > 1:                                                        (L89~91 — IR 은 L79 분기 결과로 블록 분리)
     plan.set_main_objective(team_plan.objective)   # battle.rs:350 인라인, Option<MainObjective> 3B 복사   (L90, 94453~94458)
     plan.update(version, rnd, player, data, ps, team_plan, debug)      (L92, 94459)
     if plan.sub_goal is KitingBack(3) | RunAway(4) | End(7):           (L92~93, switch 94465~94469)
         drop(plan) ; return None                                       (L93, 94472~94513: chats Vec<Chat> +0x68 · v54_reentry_ticks Vec<usize> +0x80 drop)
 return Some(BigPlan::Battle(plan))                                     (L96, 94392~94394)

요지: 갱 대상에 TryKill 전투플랜을 세우되, v2+ 에선 (a) 추격 경주가 가망 없으면 세우지 않고 (b) 한 틱 update 를 돌려 플랜이 즉시 후퇴/종료로 판정되면 폐기한다. `me`/`tgt` 가 None 이면 (a) 검사를 건너뛰고 플랜을 세운다(94421 → %49).
```

### `dd9f30` → `f10c60` TeamPlan::can_near_enemies_range (r20/r21 다수: can_near_enemies_range)
- 0.5.8 명세 #266 `TeamPlan__can_near_enemies_range` src game-ai\src\plan_legacy\team_plan.rs:483 · one_line: (x,y) 반경 d 안에 '있을 수 있는' 비가시 적 챔피언 목록 — 안 보인 시간·이속·판단력 추정으로 후보를 걸러 bumpalo Vec<&Entity> 로 반환
- 0.5.8 logic 전문:
```
fn can_near_enemies_range(&self, _version, rnd, player, data, x, y, d, _debug) -> bumpalo Vec<&Entity>
// team_plan.rs:487~489 (m09 25991~26001)
ja = player.info.parameter.judge_accuracy()            // [100,1000]
range_min = 1000 - (1000 - ja)/2                        // [550,1000]  (lshr 1)
range_max = 1000 + (1000 - ja)/2                        // [1000,1450]
// L494 (26003~26013)
is_dm = (data.cache.game.get_game_mode()@tag == 2 /*DeathMatch*/)
// L496~497 (26014~26018 · 75)  ★극성: is_dm 이 참이면 lapse 는 평가조차 안 함(A||B 의 B)
if !is_dm && player_awareness_lapse(player, data) {
    return Vec::new_in(data.context.pool)                // ptr=8 · cap=0 · len=0 (26115~26119)
}
// L499~502 (26022~26041)
judgement_base = player.info.parameter.judgement_base()  // [0,100]
game_seed = game.seed();  tick = game.tick();  tps = data.context.setting.tick_per_second
// L505~537 (26064~26102 → aux m01 24912~25358 from_iter · 캡처 15포인터 = cache,blackboard,player,&is_dm,&tick,self,&tps,&x,&y,&d,&judgement_base,&game_seed,rnd,&range_min,&range_max)
out = Vec::new_in(data.context.pool)
for i in 0..5 {                                          // 적 팀 슬롯 순회 (Range 0..5 · 25983~25992)
  // ── filter 술어 closure0 (L506~535) ──
  et = 1 - player.info.team                              // 25035~25038 · et<2 아니면 panic(506:7)
  c = data.cache.player_champion[et][i]                  // 25053~25057 · i<5 아니면 panic(506:7)
  if c.is_none() { continue }                            // 25092 null → 제외
  e = c.unwrap()
  // L507 (25121~25129)
  if game.is_visible(player.info.team, e.id) { continue } // ★보이는 적은 제외 — 이 목록은 '비가시' 적 전용
  // L510
  move_speed_raw = e.stat_cached.move_speed              // 25132
  // L511 (25135~25137)
  if !is_dm {                                            // ── MOBA/SingleLane ──
    // L512~513 (25140~25151)
    elapsed = tick.saturating_sub(self.vision.last_visible_ticks[i])
    if elapsed > tps*3 {                                 // 3초 넘게 미목격 → 판단력 미시야 추정
      // L519~520 (25154~25161)
      err = unseen_error_radius(judgement_base, elapsed, tps)
      if err > 300000 { continue }                       // 추정 불능 → 제외
      // L523 (25257~25264)
      (ex, ey) = unseen_estimated_pos(game_seed, player.info.id, i, tick, tps, e.x, e.y, err)
      // L525 (25272~25284)
      keep = distance(ex, ey, x, y) <= err + move_speed_raw*3*tps + d   // IR: `dist > err+3·ms·tps+d` 이면 제외
    } else {                                             // ≤3초 → 도달 원반(마지막 목격 + 이속·경과)
      // L515~517 (25164~25185)
      can_move = elapsed * move_speed_raw
      dist_from_last = distance(x, y, self.vision.last_visible_pos[i].0, .1).saturating_sub(d)
      keep = can_move >= dist_from_last                  // IR: `can_move < dist_from_last` 이면 제외
    }
  } else {                                               // ── DeathMatch (gamemode=0 에선 사장 · NA 봉인) ──
    // L528 (25189~25210)  ★gen_range 사이트 1 — 후보 i 마다 1회, tick() 재호출보다 먼저
    move_speed = rnd.gen_range(range_min..=range_max) * move_speed_raw / 1000   // 판단 정확도 노이즈 ±(1000−ja)/2 ‰
    // L531 (25202~25218)
    can_move = game.tick().saturating_sub(self.vision.last_visible_ticks[i]) * move_speed
    // L533 (25220~25234)
    dist_from_last = distance(x, y, self.vision.last_visible_pos[i].0, .1).saturating_sub(d)
    // L534~535 (25235~25254)
    if self.vision.last_visible_ticks[i] > self.vision.mia_call_ticks[i] {
      keep = blackboard[et].is_recent_visible_big_action(game, player, e) && can_move >= dist_from_last   // IR: 호출을 무조건 먼저 수행 후 `and`(25244~25249)
    } else {
      keep = can_move >= dist_from_last                  // 25253
    }
  }
  if !keep { continue }
  // ── filter_map closure_s_0 (L537 · aux m09 66111~66147) ──
  t = data.cache.player_champion[1 - player.info.team][i]   // Option<&Entity> 재조회(바운드 검사 재수행 · 537:23) · None 이면 skip(25295 · 실제론 위에서 Some 확정)
  out.push(t)                                            // 25308~25340 (cap==len 이면 reserve_internal_or_panic)
}
return out                                               // 25355 memcpy 32B

■ 사장 판정(reach.py · version=2 무관(버전 분기 없음) · gamemode=0 → %36=0(본체) · %90=0(클로저)): 본체 5블록 전부 live / 클로저 36블록 중 7 사장 = is_dm 분기(L528~535) → gen_range · distance(L533) · is_recent_visible_big_action 3 호출부 NA. 살아있는 콜리 = panic_bounds_check ×2 · unseen_error_radius · distance ×2(L516·L525) · unseen_estimated_pos · call_mut · reserve_internal_or_panic · Drop::drop(unwind 정리).
■ rnd(StdRng) 소비: gen_range 사이트 1(L528) · is_dm 일 때만 · 순서 = i 오름차순으로 (Some 챔피언 && !is_visible) 인 후보마다 1회(그 뒤 keep 여부와 무관). gamemode=0 → 0회.
■ 호출 순서(부작용 순): judge_accuracy → get_game_mode(vt+0x40) → [!is_dm] player_awareness_lapse → judgement_base → seed(vt+0x20) → tick(vt+0x28) → from_iter 루프.
```

### `de0770` → `f18f90` TeamPlan::update (r21 w4/w7: TeamPlan::update(+0xcc7 세팅 프롤로그 · 본체 미독))
- 0.5.8 명세 #54 `team_plan__update` src game-ai\src\plan_legacy\team_plan.rs:294 · one_line: 매 틱 TeamPlan 상태 갱신 — 정글캠프 리스폰 추적·타임아웃 정리·웨이브우선 라인·에픽/세르펜 콜 채팅·적 시야 추적·MIA 채팅
- 0.5.8 logic 전문:
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

### `e06df0` → `ee5e80` resolve_fight_stake (r20 F/r21 w2: resolve_fight_stake(ee3c80 신규 호출 · 본체 미독))
- 0.5.8 명세 #35 `fight_model__resolve_fight_stake` src game-ai\src\plan_legacy\old\fight_model.rs:571 · one_line: 교전 저울(resolve_fight_full)을 '묶인 아군을 버린 판' 기준선으로 한 번 더 돌려 차분 판정(FightPrediction)을 낸다
- 0.5.8 logic 전문:
```
fn resolve_fight_stake(version, rnd, data, player, champ, near_allies, near_enemies, committed_dir, tower, judge_accuracy, debug) -> FightPrediction

// ── 경로 A: 구버전 ────────────────────────────────────────────
[L574] if version < 2 {
[L575]   return resolve_fight_full(version, data, champ, near_allies, near_enemies,
                                  committed_dir, tower, judge_accuracy, arrivals=&[], baseline=0)
       }

// ── remaining = '교전에 묶인' 아군(나 제외) ─────────────────────
[L582~585] remaining: Vec<&Entity, &Bump> = near_allies.iter().copied()
             .filter(|a| {                      // ★closure#0 = aux m10.ll 56168~56212 (call_mut 심)
[L584]         a.id(+0x5c0) != champ.id(+0x5c0)  // 자기 자신 제외 (같으면 즉시 false, ally_is_bound 호출 안 함)
               && fight_model::ally_is_bound(version, rnd, data, player, a, near_enemies, debug)
             })
             .collect_in(data.context(+0x8).pool(+0x0))   // bumpalo from_iter_in

// ── 절대 판정 ───────────────────────────────────────────────
[L586] absolute = resolve_fight_full(version, data, champ, near_allies, near_enemies,
                                     committed_dir, tower, judge_accuracy, &[], baseline=0)

// ── 경로 B: 묶인 아군이 없으면 절대 판정 그대로 ─────────────────
[L587] if remaining.len(+0x18) == 0 { drop(remaining); return absolute }

// ── 경로 C: 차분 저울 ─────────────────────────────────────────
[L591] abandon = resolve_fight_full(version, data, champ,
                                    allies = remaining(ptr +0x0, len +0x18),   // ★아군을 '묶인 아군'만으로
                                    near_enemies, committed_dir = 0,           // ★히스테리시스 없음
                                    tower, judge_accuracy, &[], baseline=0)
[L592~593] diff = resolve_fight_full(version, data, champ, near_allies, near_enemies,
                                     committed_dir, tower, judge_accuracy, &[],
                                     baseline = abandon.net_value(+0x30))   // ★'버린 판'의 순가치를 기준선으로
[L594] diff.line_absolute(+0x39) = absolute.line(+0x38)
[L595] if diff.line(+0x38) != absolute.line(+0x38) {
[L597]   diff.rescue_ally(+0x20/+0x28) = remaining.iter()
             .min_by_key(|a| ( (a.x(+0x660).abs_diff(champ.x))^2 + (a.y(+0x668).abs_diff(champ.y))^2, a.id(+0x5c0) ))   // ★키 = (dist², id) 튜플 — 동거리면 id 작은 쪽
             .map(|a| a.id(+0x5c0))              // 나와 가장 가까운 묶인 아군 = 구조 대상
       }
       // 같으면 diff.rescue_ally 는 resolve_fight_full 이 준 값 그대로
[L599] drop(remaining); return diff

※ 세 저울(absolute/abandon/diff)은 모두 arrivals=&[] (합류 없음). 부작용은 rnd/debug 가 ally_is_bound 로 &mut 전달되는 것뿐.
※ 순서: absolute 는 remaining 수집 뒤에, abandon → diff 순으로 호출된다(IR 블록 24→42→50→54).
```

### `e0b030` → `eebb40` resolve_fight_stake_roster (r20 F/r21 w2: stake_roster)
- 0.5.8 명세 #260 `fight_model__resolve_fight_stake_roster` src game-ai\src\plan_legacy\old\fight_model.rs:645 · one_line: 합류 편성(roster)으로 교전 저울 3회 — 절대 판정 / 묶인 아군만 남긴 포기 판정 / 포기 net 기준 차분 판정 → 차분을 채택하되 라인이 갈리면 rescue_ally(최근접 묶인 아군) 표시
- 0.5.8 logic 전문:
```
// fight_model.rs:645~647  pub(crate) fn resolve_fight_stake_roster(version, data:&OperationData, champ:&Entity, roster:&[(&Entity /*a*/, i64 /*arrival*/, bool /*bound*/)], near_enemies:&[&Entity], committed_dir:i8, tower:Option<&Entity>, judge_accuracy:usize) -> FightPrediction
// L648~649
bump = data.context.pool
allies:   bumpalo Vec<&Entity> = new_in(bump)
arrivals: bumpalo Vec<i64>     = new_in(bump)
// L650~652  (편성 전원 → 절대 판정 입력)
for (a, arrival, _bound) in roster { allies.push(a); arrivals.push(arrival) }
// L654  ① 절대 판정 — sret 에 직접
absolute = resolve_fight_full(version, data, champ, &allies, near_enemies, committed_dir, tower, judge_accuracy, &arrivals, baseline=0)
// L655~656  묶인 아군(자기 제외)만
remaining: bumpalo Vec<&Entity> = roster.iter().filter(|(a,_,bound)| *bound && a.id != champ.id).map(|(a,_,_)| *a).collect_in(bump)
// L658
if remaining.is_empty() { return absolute }          // 묶인 아군 없음 → 절대 판정 그대로(rescue_ally·line_absolute 는 콜리가 쓴 값)
// L661  ② 포기 판정 — 묶인 아군만, 커밋 없음(dir 0), 도착 없음, baseline 0
abandon = resolve_fight_full(version, data, champ, &remaining, near_enemies, 0, tower, judge_accuracy, &[], 0)
// L662~663  ③ 차분 판정 — ①과 같은 입력에 baseline = abandon.net_value
diff = resolve_fight_full(version, data, champ, &allies, near_enemies, committed_dir, tower, judge_accuracy, &arrivals, abandon.net_value)
// L664
diff.line_absolute = absolute.line
// L665~666  라인이 갈렸으면(차분이 절대를 뒤집음) 근거 아군 = champ 에서 가장 가까운 remaining
if diff.line != absolute.line {
   diff.rescue_ally = remaining.iter().min_by_key(|a| ((a.x-champ.x)²+(a.y-champ.y)² /*u64 절대차 제곱합*/, a.id)).map(|a| a.id)   // remaining.len>0 이므로 항상 Some
}
// L668~669
return diff   // (drops: remaining · arrivals · allies)

// 분기 순서: remaining 비었나(L658) → 라인 갈림(L665). version/tower/judge_accuracy/near_enemies/committed_dir 은 이 함수에서 분기 없음.
// rnd 인자 없음 · gen_range 0회. 세 콜리 호출 순서 = absolute → abandon → diff (모두 같은 data 로 — 콜리가 TLS/캐시를 쓰면 이 순서가 관측 순서).
```

### `e0b730` → `eec620` v25_scoped_battle_objective (r21 w2: v25_scoped_battle_objective(6-arg 동일))
- 0.5.8 명세 #48 `fight_model__v25_scoped_battle_objective` src game-ai\src\plan_legacy\old\fight_model.rs:1138 · one_line: Morgard/Serpen 목표(Setup·Assemble 단계)가 focus 엔티티 근처(240000)의 국지전이 아니면 목표를 None 으로 지운다
- 0.5.8 logic 전문:
```
fn v25_scoped_battle_objective(_version, main_objective: Option<MainObjective>, player, data, focus: Option<usize>) -> Option<MainObjective>

[L1145] match main_objective.tag(byte0) {
  255 (None)            → return main_objective (None)
  0  (Morgard{phase,..}) if phase ∈ {Setup(1), Assemble(2)}:
[L1146~1147]  return if v25_is_objective_local_battle(JungleType::Morgard(4), player, data, focus) { main_objective } else { None }
  1  (Serpen{phase,..})  if phase ∈ {Setup(1), Assemble(2)}:
[L1149~1150]  return if v25_is_objective_local_battle(JungleType::Serpen(5),  player, data, focus) { main_objective } else { None }
  _  (Morgard/Serpen 의 다른 phase, 그 외 태그 2..11)
[L1154]       → return main_objective 그대로
}

// ── 인라인된 헬퍼 v25_is_objective_local_battle (fight_model.rs:1158~1166 추정, dloc 로 이름 확정) ──
[L1162] camp = data.context(+0x8).map(+0x20).camp_pos(jungle_type, player.info.team(+0x930) == 0)   // (x,y)
[L1163] focus_local_range = 240000
[L1164] e = focus.and_then(|id| game.get_entity_by_id(id))          // vtable+0x1f0 · null → false
[L1165] return e.is_some_and(|e| {
          dx = abs_diff(e.x(+0x660), camp.x); dy = abs_diff(e.y(+0x668), camp.y)
          dx*dx + dy*dy <= 240000²                                    // Morgard: !(> 57600000000) / Serpen: < 57600000001
        })

// 반환 조립 [L1154]: (main_objective & 0xFFFF00) | new_tag  — phase·with_battle 바이트는 항상 입력값 유지
// 분기 우선순위 근거: switch(i8 byte0) 한 번 → phase 범위검사 → focus.tag(i1) → get_entity null → 거리. phi %60 의 10개 유입이 전부 태그값.
```

### `e25030` → `e93e00` SmallActionLaneMinionPosition::target_score (r20 C: target_score)
- 0.5.8 명세 #262 `SmallActionLaneMinionPosition__target_score` src game-ai\src\small_action\lane_minion.rs:294 · one_line: 라인 미니언 후보 하나의 공격 매력도 점수(Option<i64>, 항상 Some·≥1): 킬 타이밍·HP%·이 미니언이 노리는 아군(타워/미니언)·사거리 내·거리 페널티·난수 0..=10
- 0.5.8 logic 전문:
```
fn target_score(_version, data, _player, champ, atk, target, wave_snapshot, rnd) -> Option<i64>   // lane_minion.rs:294
// 296
attack_damage = atk.expected_damage_target(data.context, champ as &dyn AbstractEntity, target)   // i64, 계약만
// 297
hp_ratio = target.hp * 100 / max(target.stat_cached.hp, 1)          // usize 산술(udiv)
// 298  attack_range = atk.range(champ) + atk.range_adjust(champ,target) + champ.radius() + target.radius()
//   Effect::range(caster) 인라인(effect.rs:26) = atk.range + caster.stat_buff_cached.range + (caster.level-1)*atk.growth_range
//   Entity::radius() 인라인(entity.rs:1511~1515) = if radius_mult==0 { radius } else { radius*(radius_mult+100)/100 }
// 299~302
walk_dist  = target.distance(champ).saturating_sub(attack_range)       // Entity::distance(target, champ) 계약만
walk_tick  = walk_dist / max(champ.stat_cached.move_speed, 1)
start_tick = atk.start_timing * 100 / max(champ.attack_speed_mult(), 1)
impact_tick = start_tick + walk_tick
// 306~316  타이밍 점수
score = 0; has_timing_reason = false
if let Some(traj) = wave_snapshot.and_then(|s| s.find(target.id)) {       // find: Option<&MinionHpTrajectory>
    predicted_hp = traj.hp_at_tick(impact_tick)                             // i64
    if predicted_hp > 0 && predicted_hp <= attack_damage + 5 {              // 308 (icmp sgt 두 번: >0 참·>dmg+5 거짓)
        score = 140                                                         // 309
    } else if predicted_hp > 0 && predicted_hp <= attack_damage * 2         // 311 (shl 1)
              && traj.expected_death_tick <= impact_tick + tps {            // 312
        score = 70                                                          // 313
    } else {                                                                // 315 (predicted_hp<=0 도 여기)
        score = if traj.expected_death_tick > tps * 2 { 0 } else { 25 }     // shl 1
    }
}   // 스냅샷 None 또는 find None → score 0 유지
// 320~326  현재 HP
if target.hp > attack_damage + 5 {                                          // 320 (usize ugt)
    if hp_ratio < 26 { score += 50 }                                        // 323~324
    else if hp_ratio < 51 { score += 25 }                                   // 325
} else { score += 90; has_timing_reason = true }                            // 321
// 329~338  이 미니언이 노리는 상대(= 내 편)
if target.ty@tag == Minion(1) && target.ty.Minion.info.nearest_enemy is Some(id) {   // 329 (select i1: && 단락)
    if let Some(enemy) = data.cache.game.get_entity_by_id(id) {             // 330 vtable+0x1f0
        if enemy.team == champ.team {                                        // 331 TeamType PartialEq 인라인
            if enemy.ty.is_tower() /* 태그 2 Tower · 3 Nexus */ { score += 80; has_timing_reason = true }   // 332~333
            else if enemy.ty@tag == Minion(1) { score += 25 }                // 335~336 (switch case 1)
        }
    }
}
// 342  이미 사거리 안
if target.distance_sq(champ) <= attack_range * attack_range { score += 20 }   // ugt → 가산 없음(select)
// 346~347
distance_penalty = utils::distance(champ.x, champ.y, target.x, target.y)     // 인자 순서 (x2,y2,x1,y1)=(champ, target)
score = score + distance_penalty / -3000 + rnd.gen_range(0..=10)             // sdiv(0 방향 절삭) · gen_range 1회·무조건
// 351~352
return Some(max(score, 1))                                                   // smax · 유일한 ret

※ has_timing_reason 은 DI 지역변수로만 존재(44013·44080·44228) — 반환·부작용에 소비처 0(사장 변수 또는 상위 인라인 전 잔재).
※ 평가 순서: 위 소스 줄 순서 그대로(IR 블록 %85→%117/%119→%127→%170…→%136). 320줄 hp 비교와 329줄 minion 분기는 독립 가산(else 아님).
```

### `e4b070` → `d57b00` v2_apply_assign_commit (r21 w4: v2_apply_assign_commit)
- 0.5.8 명세 #88 `handler__v2_apply_assign_commit` src game-ai\src\plan_legacy\handler.rs:518 · one_line: v2 이상: 오브젝트 사냥 플랜 참여 래치(v2_obj_part)와 라인 배정 래치(v2_assign)를 갱신하고, 조건 충족 시 plan/src 를 래치된 값으로 되돌려 커밋을 유지한다
- 0.5.8 logic 전문:
```
if version < 2 { return }                                            // L520
s = *src
if s ∈ {11, 12} {                                                       // L524 오브젝트 사냥 소스
  key = mf2_obj_key(self.team_plan.objective)                          // L525 (handler.rs:2314)
        = match objective { Some(Serpen{phase,..}) => phase|0x50, Some(Morgard{phase,..}) => phase|0x60, _ => return }
  alive = if s==11 { serpen_exists(ctx) } else { morgard_exists(ctx) } // L526 (tutorial 기반, 위 constants)
  if plan.tag ∈ {12,13,14,15} (Epic/Serpen HuntAndPoke/Battle) {      // L527
    self.v2_obj_part = Some(key)                                       // L529
    return
  }
  if self.v2_obj_part == Some(key) && alive {                          // L530
    if v2_obj_restore_safe(version, rnd, player, data, debug) {        // L531
      drop(*plan); *plan = if s==11 { SerpenHuntAndPoke(기본) } else { EpicHuntAndPoke(기본) }  // L533
    }
    return
  }
  if !alive && self.v2_obj_part == Some(key) { self.v2_obj_part = None }  // L535~538
  return
}
// s ∉ {11,12}
if self.v2_obj_part.is_some() && self.v2_obj_part != mf2_obj_key(objective) {   // L545 (objective 가 Morgard/Serpen 아니면 키 없음 → 다름)
  self.v2_obj_part = None                                              // L546
}
if s ∉ {17,18,19} { return }                                            // L548
if let Some((csrc, cline)) = self.v2_assign {                          // L551
  ok = match csrc {                                                    // L552
    19 => mf2_cover_need_excluding_me(player, data, cline),            // L553 (2294~2309)
    17 => mf2_epic_line_valid(player, data, cline),                    // L554 (2323~2325)
    _  => false }
  if ok {                                                              // L557
    drop(*plan); *plan = PassiveLine(PassiveLinePlan::기본(line=cline)) // L558
    *src = csrc                                                        // L559
    return
  }
  self.v2_assign = None                                                // L562
}
match s { 17 | 19 =>                                                    // L564
  if let PassiveLine(p) = plan { self.v2_assign = Some((s, p.line)) }  // L565~566
  _ => {} }

--- mf2_cover_need_excluding_me(player,data,line) [handler.rs:2294~2309] ---
line_exists(ctx.tutorial, line) 아니면 false                            // L2294
me = player.info.position as usize                                    // L2297
n  = (0..5).filter(|p| p != me && blackboard[team].in_big_line(p, line)).count()   // L2298
return n == 0 && cache.<line>_lead[team] < 3 && game.get_player_champion_by_position(team, line→Position{Top0,Mid2,Bottom3}).is_none()   // L2309

--- mf2_epic_line_valid(player,data,line) [handler.rs:2323~2325] ---
line_exists(ctx.tutorial, line) 아니면 false                            // L2323
moba = game.get_game_mode().as_moba()  (tag==0 && ptr!=null 이어야 Some) // L2324
moba.remain_epic_time(team) = epic_minion_buff_time[team] != 0 이어야 함
return cache.tower(line, 1-team) (1차 or 2차 타워) .is_some()          // L2325
```
