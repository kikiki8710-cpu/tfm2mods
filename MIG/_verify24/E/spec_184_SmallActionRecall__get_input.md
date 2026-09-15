---

### `184` SmallActionRecall::get_input — 귀환 소액션의 틱 입력: 우물 안이면 None, 안전하고 가까우면 Input::Return, 아니면 우물 방향 7x7 셀 후보를 위험·거리로 채점해 goal 을 (재)확정하고 PathFinder(dodge_danger_cell) 로 Move 입력을 만든다.

| 항목 | 값 |
|---|---|
| id | `move_actions__SmallActionRecall_get_input` |
| 심볼 | `_RNvMs1_NtNtCshdEBA0ozCnw_7game_ai12small_action12move_actionsNtB5_17SmallActionRecall9get_input` |
| 소스 | `game-ai\src\small_action\move_actions.rs:681` |
| IR | `m08.ll` 99743~102302행 |
| 경로·가시성 | `game_ai::SmallActionRecall::get_input` · **in:game_ai** |
| 계층 | 기타 |
| exe | `dbd260` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r15` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&mut game_ai::SmallActionRecall, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &mut game_core::DebugFrameData) -> std::option::Option<game_core::Input>
```

<details><summary>인자 8개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | *mut Option<Input> (32B) | sret. Option<Input> 은 니치: tag(+0, i64) = -1(0xFFFFFFFFFFFFFFFF) 이면 None, 0..5 이면 Some(Input) (tcxdict --enum Option<game_core::Input>: niche_start=18446744073709551615). Input: 0=Move{x:+8,y:+16} · 1=Return · 2=Attack · 3=Skill · 4=Skill2 · 5=Ult (페이로드 InputTarget 24B @+8) | 3 |
| 1 | 1 | self | &mut SmallActionRecall (136B) | IR 속성: noalias align 8 dereferenceable(136) — readonly 없음 → 가변. writes 전수는 writes 절. | 4 |
| 2 | 2 | version | usize | AI 버전. 분기: version<2 이면 legacy(직접귀환 판정 안 함·enemy_knows=false·free_dist 미사용·flee_ok 미사용·후보 무작위 choose·closure#10 은 is_enemy_danger_cell 경로·policy.direct_cross=true). 스택 사본 %40 이 closure#10/#11 에 &version 으로 캡처됨. | 4 |
| 3 | 3 | rnd | &mut StdRng (320B) | IR: align 16 dereferenceable(320), readonly 없음. 이 함수 본문은 직접 안 읽고 is_safe_recall · SliceRandom::choose(version<2) · positioning_window_pick · PathFinder::new_target_with_policy/update_path 에 그대로 전달. | 4 |
| 4 | 4 | player | &PlayerState (2528B) | readonly. 읽는 필드: info.team(0x930) · info.position tag(0x9c0, i32) · info.parameter(0x180, &AthleteParameter 로 전달) | 4 |
| 5 | 5 | data | &OperationData (24B) | readonly. +0 cache(&AbstractGameWithCache) · +8 context(&GameContext) · +0x10 blackboard(&[Blackboard;2]) | 4 |
| 6 | 6 | positioning_score | &PositioningScoreData (2760B) | readonly. 직접 읽는 필드: cx(0xab8) · cy(0xac0). 나머지는 positioning_score_at_* 에 전달 | 4 |
| 7 | 7 | _debug | &mut DebugFrameData (224B) | IR readnone — 본문에서 전혀 안 씀(dbg 이름 `_debug`) | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn get_input(&mut self, version, rnd, player, data, ps, _debug) -> Option<Input>   // move_actions.rs:681
// ── 절1 L682~721: 즉시 판정 ──
team = player.info.team;  healp = team==0 ? (32000, 928000) : (928000, 32000)              // L682
champ = data.cache.player_champion[team][player.info.position].unwrap()                  // L688 (None→panic)
(lx,ly,rx,ry) = ctx.map.fountains[team]                                                    // L689
if lx<=champ.x<=rx && ly<=champ.y<=ry { return None }                                       // L691~692 이미 우물 안
direct_heal = version>=2 && distance(champ, healp) < 200001                                  // L700 (version<2: false, distance 호출 안 함)
if direct_heal { self.goal=(healp); self.goal_committed=true }                              // L702~704
(enemy_positions, visible_enemy_count) = collect_visible_enemy_positions(player,data)       // L707
enemy_knows = version>=2 && enemy_knows_my_position(player,data)                             // L710
eff_risk = |s:&PositioningScore| s.risk + (enemy_knows ? s.unseen_champ_threat : 0)          // closure#0 L711
dist_to_heal_area = distance(champ, healp)                                                   // L714
time = dist_to_heal_area / champ.move_speed                                                  // L716 (speed 0 → panic)
if is_safe_recall(version,rnd,player,data,ps) && time < setting.tick_per_second*3 + 60 {    // L719 (is_safe_recall 먼저 평가)
    return Some(Input::Return) }                                                             // L720
if !direct_heal {                                                                            // L721 (direct_heal 이면 절4 로 직행)
  // ── 절2 L723~757: 진전 추적 · 확정 goal 재평가 ──
  now_tick = cache.game.tick()                                                                // L723
  prog_now = match self.path_finder { Some(pf) if pf.path_len>0 =>                            // L726
        idx=min(pf.index, path_len-1); (wx,wy)=pf.path[idx];                                  // L727~728
        isqrt(dist²(champ,(wx,wy))) + (path_len-1-idx)*32000                                 // L729
     _ => isqrt(dist²(champ, self.goal)) }                                                    // L731
  if prog_now < self.prog_best_dist_sq { self.prog_best_dist_sq=prog_now; self.prog_best_tick=now_tick }   // L733~735
  stale = None
  if self.goal_committed {                                                                    // L738
     if now_tick.saturating_sub(self.prog_best_tick) > 119 {                                  // L739 120틱 진전 없음
         stale = Some((min(goal_x/32000,29), min(goal_y/32000,29))); self.goal_committed=false // L741·L746
     } else if dist²(champ, self.goal) > 143999999 {                                          // L751 (goal 까지 >=12000)
         s = positioning_score_at_position(version,player,data,ps, goal_x,goal_y, Recall)     // L752
         risk_now = positioning_risk_value(version,player, eff_risk(s), s.on_trajectory||s.on_periodic_trajectory)  // L753
         if !(risk_now > self.goal_risk) { goto 절4 }   // L755~757: 위험이 goal_risk 이하로 유지되면 옛 goal 그대로 경로 생성 (dbg 이름 keep_committed_goal = (risk_now > goal_risk), true 면 재탐색)
     }  // dist² <= 143999999 (goal 근처) 이면 재탐색으로 진행
  }
  // ── 절3 L760~939: 후보 셀 채점 ──
  champ_cell=(min(x/32000,29), min(y/32000,29)); (hx,hy)=(healp.x/32000, healp.y/32000); (cx,cy)=(ps.cx, ps.cy)   // L760~765
  candidates: bumpalo Vec<(u64,u64,i64)> in ctx.pool                                          // L766
  vis = enemy_positions[..visible_enemy_count]   (count>5 → panic)                            // L769
  nearby = vis.count(|e| dist²(champ,e) < 40000000000)                                        // L770 closure#1 (200000²)
  risk_weight = nearby>2 ? 5 : nearby==2 ? 3 : 1                                              // L772
  if version>=2 {                                                                             // L776
     f2_enemy_anchor = vis.filter(dist²<62500000000).min_by_key(dist²)  → Option<(ex,ey)>      // L778~780 closure#2/#3 (250000²), 최초 최소 유지
     f2_now_dist = anchor.map(|a| distance(champ, a))                                          // L784 closure#4
     b1_free = Some(shared_free_dist(ctx.map))  (Arc<FreeDistMap>)                             // L787
     edge_ticks = max(32000 / max(champ.move_speed,1), 1)                                      // L789
     edge_dmg_milli = 0; for e in cache.player_champion[1-team].iter_champions() {             // L791 (team := 1-team)
         if data.blackboard[1-team].is_recent_visible(cache.game, player, e)                   // L792 ⚠적팀 blackboard
            && dist²(e, champ) <= 40000000000 {                                                // L795
            edge_dmg_milli = edge_dmg_milli.saturating_add(fight_dps(version,ctx,e,champ).saturating_mul(edge_ticks)) } }   // L798~799
     chase_pct_per_cell = edge_dmg_milli / (max(champ.hp,1)*10)                                // L801
     b1_home_w_per_unit = (chase_pct_per_cell*risk_weight)/5 + 2                               // L802 (sdiv)
     c1_chaser = vis.filter(dist²<40000000000).min_by_key(dist²) → Option<(ex,ey)>            // L810~812 closure#6/#7
     c1_home_me = free[champ_cell → home_cell(hx,hy)]  (u16)                                   // L816 closure#8, index = (cy*30+cx)*900 + (hy*30+hx)
  } else { f2=None; b1_free=None; b1_home_w_per_unit=0; c1_chaser=None }                       // L776 else / L808
  flee_ok: bumpalo Vec<(u64,u64,i64)>                                                          // L818
  for dx in 0..7 { for dy in 0..7 {                                                             // L820~821
     xi=cx-3+dx; yi=cy-3+dy (wrapping)                                                          // L822~823
     if xi>29 || yi>29 || map.walls[yi][xi]!=0 || stale==Some((xi,yi)) { continue }             // L824 (stale 셀 제외)
     if let Some((ex,ey))=f2_enemy_anchor { wx=xi*32000+16000; wy=yi*32000+16000;
        if distance((wx,wy),(ex,ey)) + 8000 < f2_now_dist { continue } }                       // L833~834 앵커 적에게 더 가까워지는 셀 제외
     cell = positioning_score_at_cell(version,player,data,ps, xi,yi, Recall)                    // L838
     nxt = map_setting.path.find_path(champ_cell.x, champ_cell.y, xi, yi)                       // L839
     risk = if let Some((nx,ny))=nxt { next=positioning_score_at_cell(nx,ny,Recall);            // L840~841
              (eff_risk(next) + eff_risk(cell)) / 2 }                                            // L842 (sdiv)
            else { eff_risk(cell) }                                                             // L844
     risk = positioning_risk_value(version,player, risk, cell.on_trajectory||cell.on_periodic_trajectory)   // L846~847
     score = if let Some(free)=b1_free { d=free[(xi,yi)→home]; if d==0xFFFF {d=150};            // L850~851
                 -(risk*risk_weight + b1_home_w_per_unit*d) }                                    // L852
             else { for_nexus=|hx-xi|+|hy-yi|; -10*for_nexus - risk*risk_weight }               // L854~855
     candidates.push((wx,wy,score))                                                             // L858~859
     if let (Some(chaser),Some(free)) = (c1_chaser,b1_free) {                                   // L861
        home_cand = free[(xi,yi)→home];                                                         // L863
        c1_worse_both = home_cand > c1_home_me && home_cand != 0xFFFF && dist²((wx,wy),chaser) < dist²(champ,chaser)   // L864 (u16 비교)
        if c1_worse_both { continue } }                                                         // flee_ok 에 넣지 않음
     flee_ok.push((wx,wy,score))                                                                // L871
  }}
  if version>=2 && !flee_ok.is_empty() { candidates = flee_ok }                                 // L877~878 (교체, 구 candidates drop)
  if candidates.is_empty() {                                                                    // L881~882 (분기 방향으로 극성 확정)
     self.goal=(healp); self.goal_committed=false                                               // L883~884 · L934
  } else {
     candidates.sort_by_key(|&(_,_,s)| -s)                                                      // L886 closure#9 (점수 내림차순)
     (min_range,max_range) = positioning_choice_window(version,player, param.positioning_runaway_min_range(), param.positioning_runaway_max_range(), true)   // L887~889
     len=candidates.len(); min_idx=min(len*min_range/1000, len-1); max_idx=clamp(len*max_range/1000, min_idx, len-1)   // L890~891
     candidates = candidates.into_iter().skip(min_idx).take(max_idx-min_idx+1).collect_in(pool)  // L893~895
     p = version<2 ? candidates.choose(rnd).unwrap() : positioning_window_pick(version,rnd,player,data,&candidates)   // L898~901
     pre_xi=min(goal_x/32000,29); pre_yi=min(goal_y/32000,29)                                  // L903~904 (옛 goal)
     pre = positioning_score_at_position(version,player,data,ps, goal_x,goal_y, Recall); pre_risk=eff_risk(pre)   // L906~907
     pre_score = if let Some(free)=b1_free { d=free[(pre_xi,pre_yi)→home]; d==0xFFFF?150:d; -(pre_risk*risk_weight + b1_home_w_per_unit*d) }   // L908~912
                 else { pre_for_nexus=|hx-pre_xi|+|hy-pre_yi|; -10*pre_for_nexus - pre_risk*risk_weight }   // L914~915
     (gx,gy) = if pre_score < p.s || dist²(champ, self.goal) <= 143999999 { (p.x,p.y) } else { (goal_x,goal_y) }   // L917 옛 goal 이 더 좋고 아직 멀면 유지
     self.goal=(gx,gy)                                                                          // L923~924
     s = positioning_score_at_position(...gx,gy,Recall); self.goal_risk = positioning_risk_value(version,player, eff_risk(s), s.on_trajectory||s.on_periodic_trajectory)   // L929~930
     self.goal_committed = true                                                                 // L932
  }
  self.prog_best_dist_sq = u64::MAX; self.prog_best_tick = now_tick                             // L937~938
  drop(flee_ok, b1_free(Arc 감소), candidates)                                                  // L939
}
// ── 절4 L943~1016: 경로 생성 · Move 입력 ──
tower_dodge = TowerDodgeContext::new_tower_avoid_v3(version,player,data, self.goal_x, self.goal_y)   // L943
pve_hazard_recall = PveHazardContext::new(version,player,data)                                 // L944
policy = SolvePolicy{ deadly_cells: v3_deadly_edge_cells(version,player,data), direct_cross: version<2 }   // L947~948
(enemy_positions, visible_enemy_count) = collect_known_enemy_positions(version,player,data)     // L951 (known 판으로 교체)
enemy_hazard = EnemyHazardContext::new(version,player,data); zone_hazard = ZoneHazardContext::new(version,player,data)   // L953~954
verdict = |sx,sy,nx,ny| -> PathVerdict {                                                       // closure#10 L956~982 (== closure#11 L987~1013)
   if tower_dodge.deadly_band_cell(version,nx,ny) { Deadly }                                    // L958
   else if !dodge_tower_cell_with_context(version,player,data,ps,&tower_dodge,sx,sy,nx,ny) { Danger }   // L961
   else if !direct_heal && version>=2 {                                                         // L965~966
        if let Some(v)=enemy_hazard.verdict(goal_x,goal_y,nx,ny) { v }                          // L967
        else if let Some(v)=zone_hazard.verdict(goal_x,goal_y,nx,ny) { v }                      // L970
        else { pve_hazard_recall.verdict(version,player,data,nx,ny).unwrap_or(Allow) } }        // L978
   else if !direct_heal && is_enemy_danger_cell(nx,ny,goal_x,goal_y,&enemy_positions,count,6400000000,1) { Soft }   // L973 (version<2)
   else { pve_hazard_recall.verdict(version,player,data,nx,ny).unwrap_or(Allow) } }             // L978 (direct_heal 이면 곧장 여기)
if self.path_finder.is_none() {                                                                 // L955
   self.path_finder = Some(PathFinder::new_target_with_policy(rnd,ctx,version,"dodge_danger_cell",policy, champ.x,champ.y, goal_x,goal_y, verdict)) }   // L956
let Some(p) = &mut self.path_finder else { return None };                                        // L985 (구조상 존재, 실질 도달 불가)
p.update_path(rnd,ctx, champ.x,champ.y, self.goal_x,self.goal_y, verdict)                        // L987
return Some(p.get_input(player,data, SafeMoveWithSkill::Must, false))                           // L1015 (sret 직접 기록)
```

**`mem` 메모리 접근 49건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | L682 우물 좌표 선택 · L688 player_champion 1차 인덱스 · L791 적팀=1-team | 4 | OK |  |
| 1 | PlayerState | 0x9c0 | info.position@tag | r | i32 → zext, player_champion 2차 인덱스 (L688) | 4 | OK |  |
| 2 | PlayerState | 0x180 | info.parameter | r | &AthleteParameter 로 positioning_runaway_min/max_range 에 전달 (L888~889) | 4 | OK |  |
| 3 | OperationData | 0x0 | cache | r | &AbstractGameWithCache | 4 | OK |  |
| 4 | OperationData | 0x8 | context | r | &GameContext | 4 | OK |  |
| 5 | OperationData | 0x10 | blackboard | r | &[Blackboard;2] — [1-team] 원소(744B stride)를 is_recent_visible self 로 (L791) | 4 | OK |  |
| 6 | AbstractGameWithCache | 0x0 | game.data_ptr | r | &dyn AbstractGame 데이터 포인터 (L723 tick 호출 self · L792) | 4 | OK |  |
| 7 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r | vtable+0x28 = tick (L723) | 4 | OK |  |
| 8 | AbstractGameWithCache | 0x1e0 | player_champion[2][5] | r | Option<&Entity> 8B 니치(null=None). [team][position] → champ (L688, None 이면 unwrap panic) · [1-team][0..5] 적 순회 (L791) | 4 | OK |  |
| 9 | GameContext | 0x0 | pool | r | &Bump — candidates(L766)·flee_ok(L818) bumpalo Vec 할당자 | 4 | OK |  |
| 10 | GameContext | 0x8 | setting | r | &GameSetting | 4 | OK |  |
| 11 | GameContext | 0x18 | map_setting | r | &MapSetting → +0x18 path(PathMap) 를 find_path 에 (L839) | 4 | OK |  |
| 12 | GameContext | 0x20 | map | r | &MapDef | 4 | OK |  |
| 13 | GameSetting | 0x12f8 | tick_per_second | r | L719 tps*3+60 | 4 | OK |  |
| 14 | MapDef | 0x6d70 | fountains[team] | r | [(lx,ly,rx,ry);2] 32B stride: +0 lx, +8 ly, +16 rx, +24 ry (L689~691) | 4 | OK |  |
| 15 | MapDef | 0x78 | walls[30][30] | r | usize. IR 주소 = +0x78 + yi*240 + xi*8 (= walls[yi][xi]) != 0 → 후보 셀 제외 (L824) | 4 | OK |  |
| 16 | MapSetting | 0x18 | path | r | PathMap — find_path self (L839) | 4 | OK |  |
| 17 | Entity | 0x660 | x | r | champ (L691·L700·L714·L731…) · 적 e (L795) | 4 | OK |  |
| 18 | Entity | 0x668 | y | r | 동상 | 4 | OK |  |
| 19 | Entity | 0x640 | stat_cached.move_speed | r | L716 time 분모(0 → div_by_zero panic) · L789 edge_ticks = 32000/max(speed,1) | 4 | OK |  |
| 20 | Entity | 0x670 | hp | r | L801 chase_pct_per_cell 분모 max(hp,1)*10 | 4 | OK |  |
| 21 | PositioningScoreData | 0xab8 | cx | r | L764 후보 창 중심 x셀 | 4 | OK |  |
| 22 | PositioningScoreData | 0xac0 | cy | r | L765 | 4 | OK |  |
| 23 | PositioningScore | 0x0 | risk | r | sret 지역 (L753·L842·L844·L907·L930) | 4 | OK |  |
| 24 | PositioningScore | 0x28 | unseen_champ_threat | r | enemy_knows 일 때만 risk 에 가산 (closure#0 L711) | 4 | OK |  |
| 25 | PositioningScore | 0x30 | on_trajectory | r | positioning_risk_value 4번째 인자 = on_trajectory \|\| on_periodic_trajectory | 4 | OK |  |
| 26 | PositioningScore | 0x31 | on_periodic_trajectory | r |  | 4 | OK |  |
| 27 | SmallActionRecall | 0x45 | path_finder@tag | r | i8 == 2 → None (Option<PathFinder> 니치). L726·L955·L985 | 4 | OK |  |
| 28 | SmallActionRecall | 0x10 | path_finder.path_len | r | L726 (==0 이면 pf 없는 것과 같이 처리) | 4 | OK |  |
| 29 | SmallActionRecall | 0x18 | path_finder.index | r | L727 idx = min(index, path_len-1) | 4 | OK |  |
| 30 | SmallActionRecall | 0x28 | path_finder.path | r | Box<[(u64,u64);70]> — [idx] 웨이포인트 (idx>=70 bounds panic) L728 · L956 구 Some drop 시 dealloc(1120,8) | 4 | OK |  |
| 31 | SmallActionRecall | 0x30 | path_finder.planned_verdict | r | Box<[u8;70]> — L956 구 Some drop 시 dealloc(70,1) (본문 직접 읽기 없음) | 4 | OK |  |
| 32 | SmallActionRecall | 0x50 | goal_x | r |  | 4 | OK |  |
| 33 | SmallActionRecall | 0x58 | goal_y | r |  | 4 | OK |  |
| 34 | SmallActionRecall | 0x68 | goal_risk | r | L755 비교 기준 | 4 | OK |  |
| 35 | SmallActionRecall | 0x70 | prog_best_dist_sq | r | 이름은 _sq 지만 저장값은 prog_now(isqrt 거리 + 잔여 웨이포인트*32000) L733 | 4 | OK |  |
| 36 | SmallActionRecall | 0x78 | prog_best_tick | r | L739 now_tick.saturating_sub(prog_best_tick) > 119 | 4 | OK |  |
| 37 | SmallActionRecall | 0x80 | goal_committed | r | L738 | 4 | OK |  |
| 38 | Arc<FreeDistMap> | 0x18 | d.ptr | r | u16 테이블 (L816·L850·L863·L909) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 39 | Arc<FreeDistMap> | 0x20 | d.len | r | bounds 검사 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 40 | Arc<FreeDistMap> | 0x0 | strong | r | L939 atomicrmw sub 1 (Arc drop) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 41 | SmallActionRecall | 0x50 | goal_x | w | IR 99966 · 101546 · 101559 | 4 | OK | healp.x (L702 direct_heal · L883 후보 0개) / gx (L923 선택 후보 또는 유지된 옛 goal) |
| 42 | SmallActionRecall | 0x58 | goal_y | w | IR 99968 · 101548 · 101560 | 4 | OK | healp.y (L703 · L884) / gy (L924) |
| 43 | SmallActionRecall | 0x80 | goal_committed | w | IR 99970 · 100331 · 101549 · 101594 | 4 | OK | 1 (L704 direct_heal · L932 후보 확정) / 0 (L746 진전 정체 120틱 · L934 후보 0개) |
| 44 | SmallActionRecall | 0x70 | prog_best_dist_sq | w | IR 100293 · 101567 | 4 | OK | prog_now (L734, prog_now < 기존값일 때만) / u64::MAX=-1 (L937 goal 재확정 후 리셋) |
| 45 | SmallActionRecall | 0x78 | prog_best_tick | w | IR 100295 · 101569 | 4 | OK | now_tick (L735 진전 갱신 · L938 리셋) |
| 46 | SmallActionRecall | 0x68 | goal_risk | w | IR 101593 — 후보 재확정 경로에서만. L883 빈 후보 경로는 goal_risk 미갱신 | 4 | OK | positioning_risk_value(version,player, s.risk + (enemy_knows? s.unseen_champ_threat:0), s.on_trajectory\|\|s.on_periodic_trajectory) @ (gx,gy) (L930) |
| 47 | SmallActionRecall | 0x0 | path_finder (72B 전체 = Some(PathFinder)) | w | IR 102142. ★HEAP 재료: 새 PathFinder 는 path Box(1120B, self+0x28) · planned_verdict Box(70B, self+0x30) 를 새로 할당. 대입 직전 구값 drop 코드(IR 102188~102239: 두 __rust_dealloc)는 구조상 존재하나 tag==2(None) 확인 후 도달하므로 **실질 사장**(reach.py 는 live 로 셈) | 4 | OK | PathFinder::new_target_with_policy(...) 결과 memcpy 72B (L956, pf None 일 때만) |
| 48 | SmallActionRecall | 0x0 | path_finder (Some 페이로드, &mut PathFinder 로 전달) | w | IR 102272 · 102274 — 내부 쓰기(index/path_len/path[]/planned_verdict[]/danger_violations/used_fallback…)는 path_finder 명세 소관. heapsurf --depth 4 결과 이 함수 전이 힙 표면 = dealloc(1120/8) self+0x28 · dealloc(70/1) self+0x30 두 건뿐 | 4 | OK | PathFinder::update_path<closure#11>(&mut pf, rnd, ctx, champ.x, champ.y, goal_x, goal_y) (L987) · PathFinder::get_input(&mut pf, player, data, Must, false) (L1015) |

**`consts` 상수 28건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 32000 | 682 | 계수 | 셀 크기. L682 우물 x/y 좌표(1셀) · L729 잔여 웨이포인트 1개당 거리 · L741·L760·L761·L903·L904 좌표→셀 나눗셈 · L789 edge_ticks 분자 · L834·L858 셀→좌표 | 4 |
| 1 | 928000 | 682 | 산출값 | = 29*32000 : 반대편 우물 좌표. healp = team==0 ? (32000, 928000) : (928000, 32000) | 4 |
| 2 | 2 | 700 | 태그 | version < 2 → legacy 분기(direct_heal=false·enemy_knows=false·free_dist 미사용·choose·is_enemy_danger_cell·direct_cross=true). 또 self+0x45 == 2 = Option<PathFinder>::None 태그(L726·L955·L985) · L1015 SafeMoveWithSkill::Must 태그 i8 2 · L842 risk 평균 /2 | 4 |
| 3 | 200001 | 700 | 임계 | distance(champ, healp) < 200001 (≈ 6.25셀) → direct_heal (우물 직행) | 4 |
| 4 | 3 | 719 | 태그 | tps*3 (3초). time < tps*3+60 이면 Input::Return. 또 L772 nearby==2 → risk_weight 3 · L819 창 반폭(cx-3, cy-3) · closure#10 Deadly 태그 | 4 |
| 5 | 60 | 719 | 계수 | +60틱 여유 (tps=60 이면 1초). time < tps*3+60 | 4 |
| 6 | 1 | 720 | 태그 | Input::Return 태그 (sret +0 store i64 1) · L772 risk_weight 기본 1 · closure#10 Soft 태그 | 4 |
| 7 | -1 | 692 | 센티널 | sret store i64 -1 = Option<Input>::None (L692 우물 안 · L985) · L937 prog_best_dist_sq = u64::MAX 리셋 · L851/L911 FreeDist u16 0xFFFF(도달불가) 검사 · closure#10 Option<PathVerdict> None 태그 | 4 |
| 8 | 70 | 728 | 길이 | PathFinder.path 배열 길이 [(u64,u64);70] — idx>=70 bounds panic · planned_verdict Box 70B | 4 |
| 9 | 119 | 739 | 임계 | now_tick.saturating_sub(prog_best_tick) > 119 (즉 >=120틱 = 2초@60tps 진전 없음) → goal_committed 해제 + stale_goal_cell 기록 | 4 |
| 10 | 29 | 741 | 인덱스 | 셀 인덱스 상한 min(v/32000, 29) (30x30 맵) | 4 |
| 11 | 143999999 | 751 | 임계 | = 12000²-1. dist²(champ, goal) > 143999999 (goal 에 12000 이상 남음) 이면 위험 재평가(L752) / L917 옛 goal 유지 조건 | 4 |
| 12 | 4 | 752 | 태그 | PositionEvalPurpose::Recall 메모리태그 4 (논리 idx 2). positioning_score_at_position/at_cell 의 purpose 인자 전부. (본문의 `shl i64 %97, 4` 는 이 상수가 아니라 [(u64,u64)] 슬라이스 원소 16B stride — 판정 아님, folded 등록 대상 아님) | 4 |
| 13 | 40000000000 | 770 | 임계 | = 200000² (6.25셀). L770 nearby 카운트 · L795 적 edge_dmg 합산 범위 · L810 c1_chaser 필터 | 4 |
| 14 | 5 | 772 | 길이 | nearby > 2 → risk_weight 5. 또 [(u64,u64);5] 슬라이스 길이 검사(L769 count>5 → slice_index_fail) · L802 (chase_pct*risk_weight)/5 | 4 |
| 15 | 62500000000 | 778 | 임계 | = 250000² (7.8셀). f2_enemy_anchor 후보 필터 (aux fold m12.ll 20275) | 4 |
| 16 | 10 | 801 | 계수 | chase_pct_per_cell = edge_dmg_milli / (max(hp,1)*10) (밀리→%) · L855/L915 for_nexus 계수 -10 | 4 |
| 17 | -10 | 855 | 미상 | score = -10*for_nexus - risk*risk_weight (free_dist 없을 때·version<2) | 4 |
| 18 | 8000 | 834 | 계수 | distance(cell, anchor) + 8000 < f2_now_dist → 셀 제외 (앵커 적에게 8000 이상 더 가까워지는 셀 금지) | 4 |
| 19 | 16000 | 834 | 미상 | 셀 중심 = cell*32000 + 16000 | 4 |
| 20 | 30 | 816 | 인덱스 | 셀 인덱스 = y*30 + x (FreeDist 900칸 행) | 4 |
| 21 | 900 | 816 | 미상 | FreeDist index = src_cell*900 + dst_cell | 4 |
| 22 | 150 | 851 | 산출값 | FreeDist 값 0xFFFF(도달불가) 대체값 150 | 4 |
| 23 | 21 | 886 | 임계 | sort_by_key: len<21 이면 insertion_sort, 아니면 driftsort (core 라이브러리 임계, 판정 아님) | 4 |
| 24 | 1000 | 890 | 계수 | min_idx = min(len*min_range/1000, len-1), max_idx = clamp(len*max_range/1000, min_idx, len-1) — 창 비율 천분율 | 4 |
| 25 | 17 | 956 | 길이 | "dodge_danger_cell" 키 길이 17 (PathFinder key &str) | 4 |
| 26 | 6400000000 | 973 | 미상 | = 80000² (2.5셀). closure#10/#11 version<2 경로 is_enemy_danger_cell 반경² (aux m00.ll 96134) | 4 |
| 27 | 0 | 978 | 태그 | closure#10: 어느 컨텍스트도 판정 안 주면 PathVerdict::Allow(0) | 4 |

**`knobs` 조정점 13건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 우물 직행 반경 | move_actions.rs:700 | 200001 | 올리면 더 먼 곳에서도 후보 탐색 없이 우물 좌표로 곧장 경로를 만든다(direct_heal). version<2 에선 무효 | 4 | 기존 |
| 1 | 즉시 Return 허용 시간 | move_actions.rs:719 | tps*3+60 틱 | 올리면 우물에서 더 멀어도(이동 시간 기준) 안전 판정만 통과하면 그 자리에서 귀환 시전 | 4 | 기존 |
| 2 | 진전 정체 판정 틱 | move_actions.rs:739 | 119 | 내리면 goal 이 더 빨리 stale 로 버려지고 그 셀이 다음 탐색에서 제외된다 | 4 | 기존 |
| 3 | goal 도달 판정 거리² | move_actions.rs:751/917 | 143999999 | 올리면 goal 근처에서 더 일찍 재탐색으로 들어가고, 옛 goal 유지 조건도 더 멀어야 성립 | 4 | 기존 |
| 4 | 근접 적 카운트 반경² | move_actions.rs:770 | 40000000000 | 올리면 nearby 가 커져 risk_weight(1/3/5) 가 커진다 → 위험 항 가중 | 4 | 기존 |
| 5 | risk_weight 단계 | move_actions.rs:772 | nearby>2→5, ==2→3, else 1 | 위험 점수의 곱셈 계수. 올리면 거리(우물 접근)보다 위험 회피가 우선 | 4 | 기존 |
| 6 | 앵커 적 후보 반경² | move_actions.rs:778 | 62500000000 | f2_enemy_anchor 선택 범위. 올리면 더 먼 적도 앵커가 되어 접근 금지 셀이 생김 | 4 | 기존 |
| 7 | 앵커 접근 금지 여유 | move_actions.rs:834 | 8000 | 내리면(0) 앵커 적에게 조금이라도 가까워지는 셀이 전부 제외. 올리면 약간 가까워지는 셀 허용 | 4 | 기존 |
| 8 | 우물 거리 가중 단위 | move_actions.rs:802 | (chase_pct*risk_weight)/5 + 2 | b1_home_w_per_unit. 상수 2 를 올리면 free 거리(우물 접근)가 점수에서 더 무겁고, 적 DPS(chase_pct) 가 클수록 자동으로 커진다 | 4 | 기존 |
| 9 | 도달불가 셀 거리 대체 | move_actions.rs:851 | 150 | free_dist 0xFFFF 셀의 벌점 거리. 내리면 도달불가 셀이 덜 벌받는다 | 4 | 기존 |
| 10 | free_dist 없을 때 거리 계수 | move_actions.rs:855 | -10 | 맨해튼 셀 거리 1당 -10. version<2 전용 | 4 | 기존 |
| 11 | 후보 창 비율 | move_actions.rs:887~891 | positioning_choice_window(param.positioning_runaway_min/max_range) 천분율 | min/max 가 낮을수록 상위(고득점) 후보만 남는다. 선수 파라미터 유래 | 4 | 기존 |
| 12 | version<2 is_enemy_danger_cell 반경² | move_actions.rs:973 | 6400000000 | legacy 경로에서 Soft 판정 반경(2.5셀) | 4 | 기존 |

<details><summary>`callees` 피호출자 42건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | collect_known_enemy_positions | game_ai::collect_known_enemy_positions | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData) -> ([(u64, u64); 5_usize], usize) | game-ai\src\path_finder.rs:1493 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | collect_visible_enemy_positions | game_ai::collect_visible_enemy_positions | pub | fn(&game_core::PlayerState, &game_core::OperationData) -> ([(u64, u64); 5_usize], usize) | game-ai\src\path_finder.rs:1471 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 2 | deadly_band_cell | game_ai::TowerDodgeContext::deadly_band_cell | pub | fn(&game_ai::TowerDodgeContext, usize, usize, usize) -> bool | game-ai\src\path_finder.rs:1256 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 3 | distance | game_core::utils::distance | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:12 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | dodge_tower_cell_with_context | game_ai::dodge_tower_cell_with_context | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_ai::TowerDodgeContext, usize, usize, usize, usize) -> bool | game-ai\src\path_finder.rs:1322 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 5 | enemy_knows_my_position | game_ai::enemy_knows_my_position | pub | fn(&game_core::PlayerState, &game_core::OperationData) -> bool | game-ai\src\path_finder.rs:1701 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 6 | fight_dps | game_ai::fight_dps | pub | fn(usize, &game_core::GameContext, &game_core::Entity, &game_core::Entity) -> usize | game-ai\src\fight_check.rs:32 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 7 | find_path | game_core::PathMap::find_path | pub | fn(&game_core::PathMap, usize, usize, usize, usize) -> std::option::Option<(usize, usize)> | game-core\src\setting.rs:82 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 8 | get_input | game_ai::PathFinder::get_input | pub | fn(&mut game_ai::PathFinder, &game_core::PlayerState, &game_core::OperationData, game_ai::SafeMoveWithSkill, bool) -> game_core::Input | game-ai\src\path_finder.rs:856 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 9 | get_input | game_ai::SmallActionRecall::get_input | in:game_ai | fn(&mut game_ai::SmallActionRecall, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &mut game_core::DebugFrameData) -> std::option::Option<game_core::Input> | game-ai\src\small_action\move_actions.rs:681 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 10 | is_empty | game_core::PatchSetting::is_empty | pub | fn(&game_core::PatchSetting) -> bool | game-core\src\patch_engine.rs:853 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 11 | is_empty | game_core::ModAiRegistry::is_empty | pub | fn(&game_core::ModAiRegistry) -> bool | game-core\src\mod_ai.rs:15 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 12 | is_empty | game_core::PublicChampionEvidence::is_empty | pub | fn(&game_core::PublicChampionEvidence) -> bool | game-core\src\banpick\public_evidence.rs:96 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 13 | is_enemy_danger_cell | game_ai::is_enemy_danger_cell | pub | fn(usize, usize, u64, u64, &[(u64, u64); 5_usize], usize, u64, u32) -> bool | game-ai\src\path_finder.rs:1811 | True | True | 3 | IR 호출 심볼 일치(토큰 경계) |
| 14 | is_recent_visible | game_core::Blackboard::is_recent_visible | pub | fn(&game_core::Blackboard, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\game\blackboard.rs:346 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 15 | is_safe_recall | game_ai::small_action::cast::is_safe_recall | in:game_ai | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData) -> bool | game-ai\src\small_action\cast.rs:304 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 16 | isqrt | game_core::utils::isqrt | pub | fn(i64) -> i64 | game-core\src\utils.rs:85 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 17 | iter_champions | game_core::AbstractGameWithCache::<'a, 'b>::iter_champions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5759 ~ game_core[6a30]::simulation::{impl#5}::iter_champions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1904 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 18 | new | game_ai::PveHazardContext::new | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData) -> game_ai::PveHazardContext | game-ai\src\path_finder.rs:1120 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 19 | new | game_ai::ZoneHazardContext::new | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData) -> game_ai::ZoneHazardContext | game-ai\src\path_finder.rs:1632 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 20 | new | game_ai::EnemyHazardContext::new | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData) -> game_ai::EnemyHazardContext | game-ai\src\path_finder.rs:1531 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 21 | new_target_with_policy | game_ai::PathFinder::new_target_with_policy | pub | fn(&mut rand::rngs::std::StdRng, &game_core::GameContext, usize, &str, game_ai::path_field::SolvePolicy, u64, u64, u64, u64, impl Fn(usize, usize, usize, usize) -> PathVerdict/#0) -> game_ai::PathFinder | game-ai\src\path_finder.rs:88 | True | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 22 | new_tower_avoid_v3 | game_ai::TowerDodgeContext::new_tower_avoid_v3 | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, u64, u64) -> game_ai::TowerDodgeContext | game-ai\src\path_finder.rs:1205 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 23 | positioning_choice_window | game_ai::small_action::positioning_choice_window | in:game_ai::small_action | fn(usize, &game_core::PlayerState, usize, usize, bool) -> (usize, usize) | game-ai\src\small_action.rs:43 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 24 | positioning_risk_value | game_ai::small_action::positioning_risk_value | in:game_ai::small_action | fn(usize, &game_core::PlayerState, i64, bool) -> i64 | game-ai\src\small_action.rs:79 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 25 | positioning_runaway_max_range | game_core::AthleteParameter::positioning_runaway_max_range | pub | fn(&game_core::AthleteParameter) -> usize | game-core\src\simulation\state\player.rs:324 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 26 | positioning_runaway_min_range | game_core::AthleteParameter::positioning_runaway_min_range | pub | fn(&game_core::AthleteParameter) -> usize | game-core\src\simulation\state\player.rs:318 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 27 | positioning_score_at_cell | game_ai::small_action::positioning_score_at_cell | in:game_ai::small_action | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, i64, i64, game_ai::PositionEvalPurpose) -> game_core::PositioningScore | game-ai\src\small_action.rs:94 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 28 | positioning_score_at_position | game_ai::small_action::positioning_score_at_position | in:game_ai::small_action | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, u64, u64, game_ai::PositionEvalPurpose) -> game_core::PositioningScore | game-ai\src\small_action.rs:99 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 29 | positioning_window_pick | game_ai::small_action::positioning_window_pick | in:game_ai::small_action | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &[T/#0]) -> &T/#0 | game-ai\src\small_action.rs:66 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 30 | push | <game_core::Staff as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::staff | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\staff.rs:38 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 31 | push | <game_core::Athlete as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:434 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 32 | push | <game_core::Contract as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:2498 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 33 | shared_free_dist | game_ai::shared_free_dist | pub | fn(&game_core::MapDef) -> std::sync::Arc<game_ai::FreeDistMap, std::alloc::Global> | game-ai\src\free_dist.rs:138 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 34 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 35 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 36 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 37 | update_path | game_ai::PathFinder::update_path | pub | fn(&mut game_ai::PathFinder, &mut rand::rngs::std::StdRng, &game_core::GameContext, u64, u64, u64, u64, impl Fn(usize, usize, usize, usize) -> PathVerdict/#0) | game-ai\src\path_finder.rs:631 | True | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 38 | v3_deadly_edge_cells | game_ai::v3_deadly_edge_cells | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData) -> u32 | game-ai\src\tower_discipline.rs:304 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 39 | verdict | game_ai::PveHazardContext::verdict | pub | fn(&game_ai::PveHazardContext, usize, &game_core::PlayerState, &game_core::OperationData, usize, usize) -> std::option::Option<game_ai::PathVerdict> | game-ai\src\path_finder.rs:1172 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 40 | verdict | game_ai::ZoneHazardContext::verdict | pub | fn(&game_ai::ZoneHazardContext, u64, u64, usize, usize) -> std::option::Option<game_ai::PathVerdict> | game-ai\src\path_finder.rs:1675 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 41 | verdict | game_ai::EnemyHazardContext::verdict | pub | fn(&game_ai::EnemyHazardContext, u64, u64, usize, usize) -> std::option::Option<game_ai::PathVerdict> | game-ai\src\path_finder.rs:1602 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
</details>

⚠**미매칭 15개**: `__rust_dealloc`, `b1_free`, `clamp`, `driftsort_main`, `drop  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `drop_slow`, `eff_risk`, `home_cell`, `insertion_sort_shift_left`, `reserve_internal_or_panic`, `saturating_mul`, `skip`, `slice_index_fail`, `sort_by_key`, `take`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m11.ll:43017) · **형제 11개** (SmallActionRecall)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::SmallActionRecall as std::clone::Clone>::clone | pub | game-ai\src\small_action\move_actions.rs:645 | True | fn(&game_ai::SmallActionRecall) -> game_ai::SmallActionRecall |
| 1 | <game_ai::SmallActionRecall as std::default::Default>::default | pub | game-ai\src\small_action\move_actions.rs:645 | True | fn() -> game_ai::SmallActionRecall |
| 2 | <game_ai::SmallActionRecall as std::fmt::Debug>::fmt | pub | game-ai\src\small_action\move_actions.rs:645 | True | fn(&game_ai::SmallActionRecall, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 3 | game_ai::SmallActionRecall::new | pub | game-ai\src\small_action\move_actions.rs:659 | False | fn(&game_core::OperationData, &game_core::PlayerState, usize) -> game_ai::SmallActionRecall |
| 4 | game_ai::SmallActionRecall::get_input | in:game_ai | game-ai\src\small_action\move_actions.rs:681 | False | fn(&mut game_ai::SmallActionRecall, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &mut game_core::DebugFrameData) -> std::option::Option<game_core::Input> |
| 5 | game_ai::SmallActionRecall::is_stalled | pub | game-ai\src\small_action\move_actions.rs:1019 | True | fn(&game_ai::SmallActionRecall, usize) -> bool |
| 6 | game_ai::SmallActionRecall::merge | in:game_ai | game-ai\src\small_action\move_actions.rs:1023 | False | fn(&mut game_ai::SmallActionRecall, game_ai::SmallActionRecall) |
| 7 | game_ai::SmallActionRecall::update_state | in:game_ai | game-ai\src\small_action\move_actions.rs:1026 | False | fn(&mut game_ai::SmallActionRecall, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 8 | game_ai::SmallActionRecall::get_action | in:game_ai | game-ai\src\small_action\move_actions.rs:1029 | True | fn(&game_ai::SmallActionRecall) -> game_core::SmallAction |
| 9 | game_ai::SmallActionRecall::is_end | in:game_ai | game-ai\src\small_action\move_actions.rs:1032 | False | fn(&game_ai::SmallActionRecall, &mut rand::rngs::std::StdRng, usize, &game_core::PlayerState, &game_core::OperationData) -> bool |
| 10 | game_ai::SmallActionRecall::near_move_complete | in:game_ai | game-ai\src\small_action\move_actions.rs:1046 | False | fn(&game_ai::SmallActionRecall, &game_core::Entity) -> bool |

**`open` 9건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 표기 불가 | L816/L909/L850/L863 FreeDist 인덱스 방향: IR 은 index = (src_y*30+src_x)*900 + (dst_y*30+dst_x) 로 src=후보/champ 셀, dst=우물 셀. 무향 그래프라 d[s][t]==d[t][s](rmeta 주석) 이므로 결과엔 무관 — 소스 표기가 `free.get(s,t)` 인지 `(t,s)` 인지는 표기 불가. | 4 |  |
| 1 | 미탐색 | L792 `data.blackboard[1-team]` — 적팀 Blackboard 를 self 로 is_recent_visible 을 부른다(IR gep 인덱스 %454 = 1-team 확정). 소스 의도(적이 나를 최근에 봤는가 vs 내가 적을 봤는가)는 is_recent_visible 본문(game_core, 이 명세 범위 밖) 소관. | 4 |  |
| 2 | 표기 불가 | L755 dbg 이름 keep_committed_goal = (risk_now > goal_risk) 인데 true 면 재탐색 블록으로 간다 — 이름과 동작이 반대로 보이지만 IR 이 정본. 소스가 `if !keep_committed_goal { goto 경로 }` 인지 `if keep {재탐색}` 인지 표기 불가. | 3 |  |
| 3 | 미탐색 | closure#10/#11 반환이 i32(PathVerdict 태그만) — DeadArgumentElimination 으로 Priced(u32) 페이로드가 잘렸다. 즉 new_target_with_policy/update_path 의 이 인스턴스는 Priced 값을 안 쓴다(추정 · path_finder 명세에서 확인 필요). | 5 |  |
| 4 | 미탐색 | L956 대입 직전 구 PathFinder drop 코드(IR 102188~102239 dealloc 1120/70) 는 tag==2(None) 확인 뒤라 실질 사장인데 reach.py(version=2·gamemode=0)는 live 로 센다 — 이 함수 안에선 self.path_finder 의 Box 가 실제로 해제되는 경로 없음(추정 근거: L955 %147 분기). | 4 |  |
| 5 | 미탐색 | PathFinder::update_path / get_input 이 self+0x0..0x48 에 쓰는 필드 전수는 path_finder 계층 명세 소관(이 라운드 범위 밖). heapsurf --depth 4 는 이 함수 전이 힙 표면으로 dealloc 2건만 보고 — grow 없음. | 4 |  |
| 6 | 미탐색 | is_safe_recall · v3_deadly_edge_cells 은 r13 잎 명세 참조(계약만 기재). | 4 |  |
| 7 | 미탐색 | positioning_choice_window 의 5번째 인자 true 의 의미(소스 이름) — 콜리 명세 소관. | 4 |  |
| 8 | 미탐색 | constants 의 21(sort 임계)·70·17 은 판정 상수가 아니라 라이브러리/배열/문자열 길이 — C1 대조용으로만 등록. | 4 |  |

<details><summary>`closed` 1건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

| # | 물음 | 닫은 근거 |
|---|---|---|
| 0 | L881 dbg 이름 picked_real_candidate = (candidates.len()==0) 로 찍히나 극성은 분기 방향으로 확정(len==0 → healp 폴백). 소스 표현(`!is_empty()` 등)은 표기 불가. | 본문에 해소 표기가 있다 |
</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

