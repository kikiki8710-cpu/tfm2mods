---

### `182` SmallActionRunAway::get_input — 도주 소액션 입력: 홈(분수/타워) 방향 7x7 셀 후보를 위험·귀환거리로 점수화해 목표를 고르고(히스테리시스·커밋) PathFinder 로 경로 입력을 낸다

| 항목 | 값 |
|---|---|
| id | `move_actions__SmallActionRunAway_get_input` |
| 심볼 | `_RNvMs_NtNtCshdEBA0ozCnw_7game_ai12small_action12move_actionsNtB4_18SmallActionRunAway9get_input` |
| 소스 | `game-ai\src\small_action\move_actions.rs:95` |
| IR | `m08.ll` 106076~110004행 |
| 경로·가시성 | `game_ai::SmallActionRunAway::get_input` · **in:game_ai** |
| 계층 | 기타 |
| exe | `dc3240` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r15` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&mut game_ai::SmallActionRunAway, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &mut game_core::DebugFrameData) -> std::option::Option<game_core::Input>
```

<details><summary>인자 8개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | Option<Input> 32B | 반환 슬롯. 아래 return 참조 | 4 |
| 1 | 1 | self | &mut SmallActionRunAway(136B) | IR %1 · dereferenceable(136) · readonly 아님 → writes 전수 필수 | 4 |
| 2 | 2 | version | usize | IR %2 · alloca %51 에 저장해 콜리에 &version 으로도 넘김. 게이트 = `version > 1`(L116/L184/L232/L255/L295/L400/L427/L536) · 클로저 안 `version < 2`(L494 밴드 검사 생략) · SolvePolicy.direct_cross = version < 2 (L488) | 4 |
| 3 | 3 | rnd | &mut StdRng(320B) | IR %3 · 본문이 직접 읽지 않음. 전달만: new_target_with_policy(L492/L538)·update_path(L555)·SliceRandom::choose(L430, version<=1)·positioning_window_pick(L428, version>1) | 4 |
| 4 | 4 | player | &PlayerState(2528B) | team(+0x930)·position 태그(+0x9c0)·info.parameter(+0x180) 읽음, 콜리 다수에 전달 | 4 |
| 5 | 5 | data | &OperationData(24B) | readonly. cache(+0)·context(+8)·blackboard(+0x10) | 4 |
| 6 | 6 | positioning_score | &PositioningScoreData(2760B) | readonly. cx(+0xab8)/cy(+0xac0)=7x7 창 중심 셀. 나머지는 positioning_score_at_cell/at_position 콜리에 전달 | 4 |
| 7 | 7 | _debug | &mut DebugFrameData | IR %7 readnone — 본문에서 전혀 안 씀(DWARF 타입 ref_mut$<DebugFrameData>) | 3 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
== 준비 (L96~L128) ==
team = player.info.team (<2 아니면 panic); champ = data.cache.player_champion[team][player.info.position]? → None 이면 return None(-1)
nearest_ally_tower = cache.iter_towers(team).min_by_key(|t| t.distance(champ) + (t.can_target && t.block_target_tick==0 ? 0 : 100000))   // L100, 동률 첫 원소
nexus = cache.nexus[team]; enemy_team = 1-team
nearest_enemy_champion = player_champion[enemy_team].iter().flatten().filter(|e| blackboard[team].is_recent_visible(game, player, e)).min_by_key(|e| dist_sq(e,champ))   // L105~106
enemy_champion_to_champ = nearest_enemy.map(|e| e.distance(champ)).unwrap_or(u64::MAX)   // L108 (-1)
(enemy_positions[5], visible_enemy_count) = collect_visible_enemy_positions(player, data)   // L111
enemy_knows = version>1 ? enemy_knows_my_position(player,data) : false   // L116
eff_risk(s) := s.risk + (enemy_knows ? s.unseen_champ_threat : 0)   // L117 지역 클로저
now_tick = game.tick()   // L123

== 진행도 추적 (L125~L135) ==
if self.path_finder.is_none() || pf.path_len==0 { prog_now = isqrt(dist_sq(champ, (goal_x,goal_y))) }   // L131
else { idx = min(pf.path_len-1, pf.index); (wx,wy) = pf.path[idx] (idx<70); prog_now = isqrt(dist_sq(champ,(wx,wy))) + (pf.path_len-1-idx)*32000 }   // L127~129 남은 경로 길이
if prog_now < self.prog_best_dist_sq { self.prog_best_dist_sq = prog_now; self.prog_best_tick = now_tick }   // L133~135

== 커밋 목표 재평가 (L138~L161) ==
stale_goal_cell = None
if self.goal_committed {
  if now_tick.sat_sub(self.prog_best_tick) > 119 { stale_goal_cell = Some((min(goal_x/32000,29), min(goal_y/32000,29))); self.goal_committed = false }   // L139~146 정체 2초 → 커밋 해제 + 그 셀 금지
  else if dist_sq(champ, goal) > 143999999 && !is_enemy_well_danger(version, player, goal_x, goal_y) {   // L152~153 아직 12000 이상 남음
    s = positioning_score_at_position(version, player, data, ps, goal_x, goal_y, RunAway)
    on_traj = s.on_trajectory || s.on_periodic_trajectory
    risk_now = positioning_risk_value(version, player, eff_risk(s), on_traj)   // L156
    keep = risk_now <= self.goal_risk && (!self.dodge_trajectory || !on_traj)   // L158~160 (IR: risk_now > goal_risk → 해제 / dodge_trajectory && on_traj → 해제)
    if keep { dodge_traj = self.dodge_trajectory; goto PATH(goal_x, goal_y) }   // L161 (self 목표 그대로, goal_committed 등 안 건드림)
  }
}

== 귀환 지점 (L162~L201) ==
(flx,fly,frx,fry) = map.fountains[team]; (hx,hy) = ((flx+frx)/2, (fly+fry)/2)   // L162~163 분수 중심 기본값
if let (Some(tower), Some(nexus)) = (nearest_ally_tower, nexus) {   // L169
  tower_to_nexus = tower.distance(nexus); nexus_to_champ = champ.distance(nexus); tower_to_champ = tower.distance(champ)
  hp_ratio = champ.hp*100 / max(champ.max_hp,1)   // L175
  near_base = dist_sq(champ,nexus) < 67600000001 || cache.twin_towers[team].iter().any(|t| dist_sq(champ,t) < 32400000001)   // L176~178
  go_tower = true
  if near_base && (hp_ratio < 36 || (hp_ratio < 46 && enemy_champion_to_champ <= 160000)) {   // L180
     go_tower = nexus_is_critical(player,data) || (version>1 && nexus_final_stand(player,data))   // L183~186 (아니면 분수 중심으로 풀회복)
  }
  if go_tower {
    if tower_to_nexus > nexus_to_champ.sat_sub(30000) && tower_to_champ > enemy_champion_to_champ.sat_sub(30000) { /* 분수 유지 */ }   // L188
    else if visible_enemy_count <= 1 { (hx,hy) = tower.pos }   // L191/L201
    else { enemies_near_tower = enemy_positions[..count].iter().filter(|p| dist_sq(tower,p) < 14400000000).count();   // L192~194
           (hx,hy) = enemies_near_tower > 1 ? 분수중심 : tower.pos }   // L195
  }
}

== 위험 가중·거리장 (L213~L305) ==
champ_cell=(champ.x/32000, champ.y/32000); home_cell=(hx/32000, hy/32000); (cx,cy) = ps.cx, ps.cy   // L213~218
candidates: Vec<(u64,u64,i64)> = new_in(bump)   // L219
pve_hazard = PveHazardContext::new(version, player, data)   // L220
nearby = enemy_positions[..count].filter(|p| dist_sq(champ,p) < 40000000000).count()   // L225
risk_weight = nearby>2 ? 5 : nearby==2 ? 3 : 1   // L227
f2_enemy_anchor = version>1 ? enemy_positions[..count].filter(dist_sq(champ,p) < 62500000000).min_by_key(dist_sq) : None; f2_now_dist = anchor.map(|a| distance(champ,a))   // L232~240
no_structures = tower.is_none() && nexus.is_none(); for_nexus_weight = no_structures ? 0 : 10   // L244
b1_free: Option<Arc<FreeDistMap>> = (version>1 && !no_structures) ? Some(shared_free_dist(map)) : None   // L255
b1_home_w_per_unit = 0
if version>1 && !no_structures {   // L256~270
  edge_ticks = max(32000 / max(champ.move_speed,1), 1)   // L257 한 셀 통과 틱
  edge_dmg_milli = Σ_{e in enemy champions, is_recent_visible(bb[team],e), dist_sq(e,champ) <= 40000000000} fight_dps(version, ctx, e, champ).sat_mul(edge_ticks) (sat_add)   // L259~267
  chase_pct_per_cell = edge_dmg_milli / (max(champ.hp,1)*10)   // L269 (분모 0 불가)
  b1_home_w_per_unit = chase_pct_per_cell * risk_weight / 5 + 2   // L270 (sdiv)
}
field_right = (team == 0)   // L277
structure_cells = towers(team).map(|t| (t.x/32000, t.y/32000)) ++ nexus.map(cell)   // L281~286
c1_chaser = version>1 ? enemy_positions[..count].filter(dist_sq(champ,p) < 40000000000).min_by_key(dist_sq) : None   // L295~299
home_idx = home_cell.y*30 + home_cell.x; c1_home_me = b1_free.map(|f| f.d[(champ_cell.y*30+champ_cell.x)*900 + home_idx])   // L303~305 (u16, 0xFFFF=도달불가)
cell_scores: Vec<(u64,u64,i64)> = new_in(bump)   // L306

== 7x7 후보 (L308~L394) ==
for dx in 0..7 { xi = cx-3+dx; for dy in 0..7 { yi = cy-3+dy;
  if xi>29 || yi>29 || map.walls[xi][yi] != 0 { continue }   // L312
  wx = xi*32000+16000; wy = yi*32000+16000
  if no_structures && ((xi==cx && yi==cy) || (field_right ? wx <= frx : wx >= flx)) { continue }   // L315~317 구조물 전멸 시: 현재 셀·자기 분수쪽 셀 제외
  if is_enemy_well_danger(version, player, wx, wy) { continue }   // L327
  if v3_lethal_tower_position(version, player, data, wx, wy) { continue }   // L332
  if v3_lethal_wave_position(version, player, data, wx, wy) { continue }   // L333
  if pve_hazard.lethal_zone_goal(wx, wy) || stale_goal_cell == Some((xi,yi)) { continue }   // L334
  if let Some((ex,ey)) = f2_enemy_anchor { if distance(wx,wy,ex,ey) + 8000 < f2_now_dist { continue } }   // L343~344 적에게 8000 넘게 다가가는 셀 제외
  if structure_cells.contains(&(xi,yi)) { continue }   // L351
  cell_score = positioning_score_at_cell(version, player, data, ps, xi, yi, RunAway)   // L357
  if self.dodge_trajectory && (cell_score.on_trajectory || cell_score.on_periodic_trajectory) { continue }   // L358
  risk = match PathMap::find_path(&map_setting.path, min(champ_cell.x,29), min(champ_cell.y,29), xi, yi) {   // L361 다음 셀
     Some((nx,ny)) => { next = positioning_score_at_cell(..., nx, ny, RunAway); (eff_risk(next) + eff_risk(cell_score)) / 2 }   // L362~364 (sdiv)
     None => eff_risk(cell_score) }   // L366
  risk = positioning_risk_value(version, player, risk, cell_score.on_trajectory || cell_score.on_periodic_trajectory)   // L368~369
  score = match b1_free { None => -(risk*risk_weight + (|home_cell.x-xi| + |home_cell.y-yi|)*for_nexus_weight),   // L377~378
                          Some(f) => { d = f.d[(yi*30+xi)*900 + home_idx]; if d==0xFFFF { d=150 }; -(b1_home_w_per_unit*d + risk*risk_weight) } }   // L373~375
  candidates.push((wx, wy, score))   // L382
  // c1 필터 (L384~394): 추격자에게 가까워지면서 귀환거리도 나빠지는 셀은 cell_scores 에서 제외
  if let (Some(chaser), Some(f)) = (c1_chaser, b1_free) {
     home_cand = f.d[(yi*30+xi)*900 + home_idx]
     c1_worse_both = dist_sq((wx,wy), chaser) < dist_sq(champ, chaser) && home_cand != 0xFFFF && home_cand > c1_home_me
     if c1_worse_both { continue } }
  cell_scores.push((wx, wy, score))   // L394
}}
if version>1 && !cell_scores.is_empty() { candidates = cell_scores }   // L400~401

== 목표 확정 (L404~L470) ==
if candidates.is_empty() {   // L405
  if no_structures { self.goal_x = field_right ? frx+32000 : flx.sat_sub(32000); self.goal_y = champ.y }   // L406~409 분수 바로 바깥
  else { self.goal_x = home_cell.x*32000+16000; self.goal_y = home_cell.y*32000+16000 }   // L411~412
  self.goal_committed = false   // L465
} else {
  candidates.sort_by_key(|c| -c.2)   // L415 점수 내림차순(안정)
  (min_pm, max_pm) = positioning_choice_window(version, player, param.positioning_runaway_min_range(), param.positioning_runaway_max_range(), true)   // L416~418
  min_idx = min(len*min_pm/1000, len-1); max_idx = clamp(len*max_pm/1000, min_idx, len-1)   // L419~420
  candidates = candidates[min_idx ..= max_idx] (skip/take 로 새 Vec)   // L422~424
  picked = version>1 ? positioning_window_pick(version, rnd, player, data, &candidates) : candidates.choose(rnd).unwrap()   // L427~430
  // 히스테리시스: 이전 목표 점수와 비교 (L432~447)
  pre = positioning_score_at_position(version, player, data, ps, self.goal_x, self.goal_y, RunAway); pre_risk = eff_risk(pre)   // L435~436
  pre_score = match b1_free { None => -(pre_risk*risk_weight + (|home_cell.x-pre_xi| + |home_cell.y-pre_yi|)*for_nexus_weight),   // L444~445 (pre_xi/yi = min(goal/32000,29))
                              Some(f) => { d = f.d[(pre_yi*30+pre_xi)*900 + home_idx]; if d==0xFFFF {d=150}; -(b1_home_w_per_unit*d + pre_risk*risk_weight) } }   // L439~442
  (gx,gy) = if picked.score > pre_score { picked.xy } else if dist_sq(champ, self.goal) > 143999999 { self.goal (유지) } else { picked.xy }   // L447
  self.goal_x = gx; self.goal_y = gy   // L453~454
  s = positioning_score_at_position(..., gx, gy, RunAway); self.goal_risk = positioning_risk_value(version, player, eff_risk(s), s.on_trajectory||s.on_periodic_trajectory)   // L460~461
  self.goal_committed = true   // L463
}
self.prog_best_dist_sq = u64::MAX; self.prog_best_tick = now_tick   // L468~469
drop(structure_cells, cell_scores(이동 안 됐으면), b1_free Arc(강참조 -1), candidates)   // L470
dodge_traj = self.dodge_trajectory; goto PATH(self.goal_x, self.goal_y)

== PATH (L475~L603) ==
restrict_field = tower.is_none() && nexus.is_none()   // L475
tower_dodge = TowerDodgeContext::new_tower_avoid_v3(version, player, data, goal_x, goal_y); pve_hazard = PveHazardContext::new(...); enemy_hazard = EnemyHazardContext::new(...); zone_hazard = ZoneHazardContext::new(...)   // L479~483 (여기서 다시 생성)
policy = SolvePolicy{ deadly_cells: v3_deadly_edge_cells(version, player, data), direct_cross: version < 2 }   // L487~488
if self.path_finder.is_none() { self.path_finder = Some(PathFinder::new_target_with_policy(rnd, ctx, version, "dodge_danger_cell", policy, champ.x, champ.y, goal_x, goal_y, &sf_0)) }   // L491~492
if version>1 && pf.is_some() && pf.unconstrained_fallback && v3_lethal_tower_hp(data, player, champ) {   // L536~537
  retry = PathFinder::new_target_with_policy(rnd, ctx, version, "lethal_tower_only", policy, champ.xy, goal.xy, &sh_0)   // L538
  if retry.unconstrained_fallback { drop(retry) } else { self.path_finder = Some(retry) /* 기존 Box 2개 dealloc */ }   // L548~551
}
p = self.path_finder.as_mut()? → None 이면 return None(-1)   // L553
p.update_path(rnd, ctx, champ.x, champ.y, goal_x, goal_y, &si_0)   // L555
skill_mode = self.with_ult ? MustWithUlt(3) : (self.with_skill ? Must(2) : Safe(1))   // L595
return p.get_input(player, data, skill_mode, with_recall=false)   // L602

== 클로저 sf_0/si_0/sh_0 = signature.closures 참조 ==
```

**`mem` 메모리 접근 58건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | L96 team(<2 bounds check) · 1-team = 적 팀(L104) | 4 | OK |  |
| 1 | PlayerState | 0x9c0 | info.position@tag(u32) | r | L96 player_champion[team][position] 인덱스 | 4 | OK |  |
| 2 | PlayerState | 0x180 | info.parameter(AthleteParameter) | r | L417/418 positioning_runaway_min/max_range(&param) | 4 | OK |  |
| 3 | OperationData | 0x0 | cache | r | &AbstractGameWithCache | 4 | OK |  |
| 4 | OperationData | 0x8 | context | r | &GameContext (L162/L219/L361/L492/L538/L555) | 4 | OK |  |
| 5 | OperationData | 0x10 | blackboard | r | &[Blackboard;2] → [team] 을 is_recent_visible 에 (L105/L260) | 4 | OK |  |
| 6 | AbstractGameWithCache | 0x0 | game.data_ptr | r | dyn AbstractGame 팻포인터 앞 절반 | 4 | OK |  |
| 7 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r | vtable+0x28 = AbstractGame::tick → now_tick (L123 · divtable 확인) | 3 | OK |  |
| 8 | AbstractGameWithCache | 0x130 | twin_towers[team] | r | L177~178 bumpalo Vec<&Entity>(ptr@+0, len@+24) 순회 | 4 | OK |  |
| 9 | AbstractGameWithCache | 0x170 | nexus[team] | r | L102 Option<&Entity> | 4 | OK |  |
| 10 | AbstractGameWithCache | 0x1e0 | player_champion[team][pos] / [enemy][0..5] | r | L96 champ · L104~106/L259 적 챔피언 5칸 | 4 | OK |  |
| 11 | GameContext | 0x18 | map_setting | r | L361 &map_setting.path(+0x18) 를 PathMap::find_path 에 | 4 | OK |  |
| 12 | GameContext | 0x20 | map | r | &MapDef (L162 fountains · L255 shared_free_dist · L312 walls) | 4 | OK |  |
| 13 | MapSetting | 0x18 | path(PathMap 64B) | r | L361 | 4 | OK |  |
| 14 | MapDef | 0x78 | walls[30][30] usize | r | L312 walls[xi][yi] != 0 → 후보 제외 | 4 | OK |  |
| 15 | MapDef | 0x6d70 | fountains[team] (x0,y0,x1,y1) 32B stride | r | L162 flx/fly/frx/fry · fountain_center=((flx+frx)/2,(fly+fry)/2) | 4 | OK |  |
| 16 | Entity | 0x660 | x | r | champ/적/타워/넥서스 좌표(dist_sq 인라인 다수) | 4 | OK |  |
| 17 | Entity | 0x668 | y | r |  | 4 | OK |  |
| 18 | Entity | 0x670 | hp | r | L175 hp_ratio · L269 chase_pct_per_cell 분모 | 4 | OK |  |
| 19 | Entity | 0x628 | stat_cached.hp(최대HP) | r | L175 hp_ratio = hp*100/max(max_hp,1) | 4 | OK |  |
| 20 | Entity | 0x640 | stat_cached.move_speed | r | L257 edge_ticks = max(32000/max(speed,1),1) | 4 | OK |  |
| 21 | Entity | 0x6b9 | can_target | r | L100 타워 키 (aux m06) | 4 | OK |  |
| 22 | Entity | 0x6a0 | block_target_tick | r | L100 타워 키 (==0 이어야 페널티 0) | 4 | OK |  |
| 23 | PositioningScoreData | 0xab8 | cx | r | L217 7x7 창 중심 x셀 | 4 | OK |  |
| 24 | PositioningScoreData | 0xac0 | cy | r | L218 | 4 | OK |  |
| 25 | PositioningScore(콜리 sret 56B) | 0x0 | risk | r | eff_risk(L117) = risk + (enemy_knows ? unseen_champ_threat : 0) | 4 | OK |  |
| 26 | PositioningScore(콜리 sret 56B) | 0x28 | unseen_champ_threat | r | L117 | 4 | OK |  |
| 27 | PositioningScore(콜리 sret 56B) | 0x30 | on_trajectory | r | L155/L358/L369/L462 (on_traj \|\| on_periodic) → positioning_risk_value 4번째 인자 / dodge 제외 | 4 | OK |  |
| 28 | PositioningScore(콜리 sret 56B) | 0x31 | on_periodic_trajectory | r |  | 4 | OK |  |
| 29 | SmallActionRunAway(self) | 0x8 | goal_x | r | L131/L141/L152/L432/L477 읽음 | 4 | OK |  |
| 30 | SmallActionRunAway(self) | 0x10 | goal_y | r |  | 4 | OK |  |
| 31 | SmallActionRunAway(self) | 0x20 | goal_risk | r | L158 risk_now > goal_risk 면 커밋 해제 | 4 | OK |  |
| 32 | SmallActionRunAway(self) | 0x28 | prog_best_dist_sq | r | L133 (이름과 달리 sqrt 된 거리·경로잔량 단위) | 4 | OK |  |
| 33 | SmallActionRunAway(self) | 0x30 | prog_best_tick | r | L139 정체 틱 = now - prog_best_tick | 4 | OK |  |
| 34 | SmallActionRunAway(self) | 0x7d | path_finder@tag (==2 → None) | r | L125/L491/L536/L553 · Some 이면 이 바이트 = pf.unconstrained_fallback(0/1) | 4 | OK |  |
| 35 | SmallActionRunAway(self) | 0x48 | path_finder.path_len | r | L126 ==0 이면 목표까지 직선거리로 prog 계산 | 4 | OK |  |
| 36 | SmallActionRunAway(self) | 0x50 | path_finder.index | r | L127 idx = min(path_len-1, index) | 4 | OK |  |
| 37 | SmallActionRunAway(self) | 0x60 | path_finder.path (Box<[(u64,u64);70]>) | r | L128 path[idx] (idx<70 bounds check) · L549 drop | 4 | OK |  |
| 38 | SmallActionRunAway(self) | 0x68 | path_finder.planned_verdict (Box<[u8;70]>) | r | L549 drop | 4 | OK |  |
| 39 | SmallActionRunAway(self) | 0x80 | with_skill | r | L595 skill_mode | 4 | OK |  |
| 40 | SmallActionRunAway(self) | 0x81 | with_ult | r | L595 | 4 | OK |  |
| 41 | SmallActionRunAway(self) | 0x82 | dodge_trajectory | r | L158/L358/L476 · 클로저 캡처 dodge_traj | 4 | OK |  |
| 42 | SmallActionRunAway(self) | 0x83 | goal_committed | r | L138 | 4 | OK |  |
| 43 | PathFinder(retry 지역 72B) | 0x45 | unconstrained_fallback | r | L548 true 면 retry 폐기 | 4 | OK |  |
| 44 | PathFinder(retry 지역 72B) | 0x28 | path Box | r | L551 폐기 시 dealloc(1120,8) | 4 | OK |  |
| 45 | PathFinder(retry 지역 72B) | 0x30 | planned_verdict Box | r | L551 폐기 시 dealloc(70,1) | 4 | OK |  |
| 46 | ArcInner<FreeDistMap> | 0x18 | d.ptr (Vec<u16>) | r | L303/L373/L386/L439 free[(sy*30+sx)*900 + (ty*30+tx)] · 0xFFFF=도달불가 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 47 | ArcInner<FreeDistMap> | 0x20 | d.len | r | bounds check | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 48 | ArcInner<FreeDistMap> | 0x0 | strong | r | L470 atomicrmw sub 1 release → 1 이었으면 drop_slow | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 49 | SmallActionRunAway(self) | 0x28 | prog_best_dist_sq | w | L134 는 prog_now < prog_best 일 때만 | 4 | OK | prog_now (L134, m08.ll:107035) / -1=u64::MAX (L468, 109023 새 목표 확정 시 리셋) |
| 50 | SmallActionRunAway(self) | 0x30 | prog_best_tick | w |  | 4 | OK | now_tick (L135, 107037 / L469, 109025) |
| 51 | SmallActionRunAway(self) | 0x83 | goal_committed | w | 커밋 유지 경로(L161)에서는 안 씀 | 4 | OK | 0 (L146 정체≥120틱, 107073) · 1 (L463 후보 선택 성공, 109050) · 0 (L465 구조물 없음/후보 없음 폴백, 109005) |
| 52 | SmallActionRunAway(self) | 0x8 | goal_x | w |  | 4 | OK | field_right ? frx+32000 : flx.sat_sub(32000) (L408, 108752) · hx*32000+16000 (L411, 108731) · gx(선택 후보 또는 이전 목표) (L453, 109015) |
| 53 | SmallActionRunAway(self) | 0x10 | goal_y | w |  | 4 | OK | champ.y (L409, 108754) · hy*32000+16000 (L412, 108735) · gy (L454, 109016) |
| 54 | SmallActionRunAway(self) | 0x20 | goal_risk | w | L463 과 한 세트 | 4 | OK | positioning_risk_value(version, player, eff_risk(score_at_position(gx,gy,RunAway)), on_traj) (L461, 109049) |
| 55 | SmallActionRunAway(self) | 0x38 | path_finder (Option<PathFinder> 72B 통째) | w | ★HEAP: 기존 path Box@self+0x60 → __rust_dealloc(1120,8)(109319) · planned_verdict Box@self+0x68 → __rust_dealloc(70,1)(109341). 새 Box 할당은 콜리 new_target_with_policy 내부. L551 retry 폐기 시엔 retry 의 Box 2개만 dealloc(109267/109289) 하고 self 는 불변 | 4 | OK | Some(PathFinder::new_target_with_policy(rnd, ctx, version, "dodge_danger_cell", policy, champ.xy, goal.xy, &sf_0)) (L492 memcpy 72B, 109192 — None 일 때만) · Some(retry 'lethal_tower_only') (L549 memcpy, 109342 — 기존 Box 2개 dealloc 후) |
| 56 | SmallActionRunAway(self) | 0x38 | path_finder (콜리 경유 &mut) | w | ★HEAP: 두 콜리가 self 소유 Box(path 1120B / planned_verdict 70B)의 내용을 제자리 갱신할 수 있음(내부 미독해 — path_finder 계층). sweep 은 self+0x60/+0x68 이 가리키는 힙을 HEAP_SUBST 에 넣어야 함(heapsurf 결과 dealloc(1120/8)@+0x60 · (70/1)@+0x68) | 4 | OK | PathFinder::update_path(&mut pf, rnd, ctx, champ.xy, goal.xy, &si_0) (L555, 109384) · PathFinder::get_input(sret, &mut pf, player, data, skill_mode, false) (L602, 109394) |
| 57 | rnd(&mut StdRng) | 0x0 | StdRng 상태 | w | 본문 직접 read/write 0건 | 4 | 오귀속(사전은 다른 필드를 준다) | 콜리 경유만(new_target_with_policy/update_path/choose/positioning_window_pick) |

**`consts` 상수 36건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 2 | 96 | 태그 | team < 2 bounds check / L125 path_finder None 태그 == 2 / L488 direct_cross = version < 2 / L536 태그 != 2 | 4 |
| 1 | 100000 | 100 | 산출값 | 가장 가까운 아군 타워 선정 키 페널티: can_target && block_target_tick==0 이 아니면 distance+100000 (aux m06 fold 에도 동일) | 4 |
| 2 | 1 | 116 | 임계 | version > 1 게이트(=v2 이상 경로) 전반 · L191 visible_enemy_count > 1 · L195 enemies_near_tower > 1 | 4 |
| 3 | 40 | 123 | 산출값 | AbstractGame vtable 슬롯 +0x28 = tick() → now_tick | 4 |
| 4 | 70 | 128 | 길이 | path[idx] 배열 길이(bounds check) · Box<[(u64,u64);70]> / Box<[u8;70]> 레이아웃 크기(L549/L551 dealloc size 70) | 4 |
| 5 | 32000 | 129 | 계수 | 셀 크기. L129 prog_now += (path_len-1-idx)*32000 · L141/L213~216/L283/L286/L304/L327/L408/L411/L432 셀↔좌표 변환 | 4 |
| 6 | 119 | 139 | 임계 | 커밋 목표 정체 판정: now - prog_best_tick > 119 틱(=120틱, 60tps 기준 2초) 이면 커밋 해제 + 그 목표 셀을 이번 후보에서 제외(stale_goal_cell) | 4 |
| 7 | 29 | 141 | 인덱스 | 셀 인덱스 clamp 상한(30x30 그리드) | 4 |
| 8 | 143999999 | 152 | 임계 | 12000^2 - 1: dist_sq(champ, goal) > 143999999 ⇔ 목표까지 12000 이상 남음(L152 커밋 재평가 조건 · L447 이전 목표 유지 조건) | 4 |
| 9 | 3 | 154 | 태그 | PositionEvalPurpose::RunAway 메모리 태그 3(tcxdict --enum: idx1 → 태그 3) — positioning_score_at_position/at_cell 마지막 인자 · L227 risk_weight=3(nearby==2) · L503/504 창 반경 3 · L595 SafeMoveWithSkill::MustWithUlt=3 | 3 |
| 10 | 100 | 175 | 계수 | hp_ratio = hp*100 / max(max_hp,1) | 4 |
| 11 | 67600000001 | 176 | 임계 | 260000^2 + 1: dist_sq(champ, nexus) < 이면 '본진 근처' | 4 |
| 12 | 32400000001 | 178 | 임계 | 180000^2 + 1: twin_towers[team] 중 dist_sq(champ, t) < 이면 '본진 근처' | 4 |
| 13 | 36 | 180 | 임계 | hp_ratio < 36 → 저HP(분수 귀환 검토) | 4 |
| 14 | 46 | 180 | 임계 | hp_ratio < 46 && 적 챔피언 거리 <= 160000 → 저HP 취급 | 4 |
| 15 | 160001 | 180 | 임계 | enemy_champion_to_champ < 160001 ⇔ <= 160000 (적 없으면 u64::MAX 라 거짓) | 4 |
| 16 | 30000 | 188 | 계수 | tower_to_nexus > nexus_to_champ.sat_sub(30000) && tower_to_champ > enemy_to_champ.sat_sub(30000) → 타워 대신 분수(타워가 나보다 넥서스에서 멀고 적보다 나한테서 멀면) | 4 |
| 17 | 14400000000 | 193 | 임계 | 120000^2: 타워 반경 120000 안 적 위치 수 = enemies_near_tower | 4 |
| 18 | 40000000000 | 225 | 임계 | 200000^2: L225 nearby(적 위치 수) · L263 fight_dps 합산 대상 적 챔피언 반경 · L297 c1_chaser 후보 반경(aux m12 20747) | 4 |
| 19 | 5 | 227 | 길이 | risk_weight = nearby>2 ? 5 : nearby==2 ? 3 : 1 · L270 b1_home_w_per_unit = chase_pct*risk_weight/5 + 2 · L191/L224 enemy_positions 배열 길이 5. (aux 클로저의 `shl i64 %164, 5` 는 EnemyHazardContext.bands 32B stride 이지 이 상수가 아님 — 접힘 없음) | 4 |
| 20 | 62500000000 | 234 | 임계 | 250000^2: f2_enemy_anchor(가장 가까운 적 위치) 후보 반경(aux m12 20553 fold 에도 동일) | 4 |
| 21 | 10 | 244 | 계수 | for_nexus_weight = (타워 None && 넥서스 None) ? 0 : 10 · L269 분모 max(hp,1)*10 | 4 |
| 22 | 0 | 244 | 태그 | for_nexus_weight 0 (구조물 없음) · L312 walls==0 통과 · Allow 태그 | 4 |
| 23 | 8000 | 344 | 계수 | 후보 셀이 최근접 적(f2_enemy_anchor)에 지금보다 8000 넘게 가까워지면(distance(cell,anchor)+8000 < f2_now_dist) 제외 | 4 |
| 24 | 150 | 374 | 산출값 | free_dist 값 0xFFFF(-1, 도달불가) → 150 셀로 치환 (L441 pre_score 도 동일) | 4 |
| 25 | -1 | 374 | 센티널 | free_dist u16 sentinel(도달불가) · L468 prog_best_dist_sq 리셋값 u64::MAX · Option<Input> None 태그 · 이터레이터 상태 -1/-2(무시) | 4 |
| 26 | 1000 | 419 | 계수 | positioning_choice_window 반환 (min,max) 는 천분율: min_idx = min(len*min/1000, len-1), max_idx = clamp(len*max/1000, min_idx, len-1) | 4 |
| 27 | 16000 | 327 | 계수 | 셀 중심 = idx*32000+16000 | 4 |
| 28 | 7 | 308 | 임계 | 7x7 후보 창(dx,dy in 0..7 · xi=cx-3+dx) | 4 |
| 29 | 6 | 100 | 임계 | iter_towers 결과의 고정 배열 길이 6 (타워 Option 6칸 순회 인라인) — 임계 아님 | 4 |
| 30 | 1120 | 549 | 미상 | Box<[(u64,u64);70]> 할당 크기 1120B(align 8) dealloc — HEAP 재료 | 4 |
| 31 | 30 | 303 | 인덱스 | free_dist 인덱스 stride: (y*30+x) · *900(=30*30) 은 출발 셀 stride | 4 |
| 32 | 900 | 303 | 미상 | free_dist 출발셀 stride 30*30 | 4 |
| 33 | 6400000000 | 526 | 미상 | 80000^2: aux 클로저(version<=1) is_enemy_danger_cell 반경 인자 | 4 |
| 34 | 16 | 494 | 산출값 | aux 클로저 tower_dodge.towers 배열 길이 16(bounds) — 임계 아님 | 4 |
| 35 | 20 | 513 | 미상 | aux 클로저 pve_hazard.zones 배열 길이 20(bounds) — 임계 아님 | 4 |

**`knobs` 조정점 15건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 커밋 목표 정체 허용 틱 | move_actions.rs:139 | 119 | 올리면 진전 없는 도주 목표를 더 오래 붙들고(2초→), 내리면 더 빨리 목표를 갈아치움(정체 셀은 다음 후보에서 제외됨) | 4 | 기존 |
| 1 | 커밋 재평가 최소 잔여거리 | move_actions.rs:152 / 447 | 143999999 | 12000^2-1. 올리면 목표에 더 멀리서부터 '도착' 취급해 매 평가 재선정(흔들림↑), 내리면 목표 근처까지 커밋 유지 | 4 | 기존 |
| 2 | 본진 근처 판정 반경(넥서스/쌍둥이타워) | move_actions.rs:176 / 178 | 67600000001 | 260000^2+1 / 180000^2+1. 올리면 더 먼 곳에서도 저HP 분수귀환 로직이 켜짐 | 4 | 기존 |
| 3 | 저HP 분수귀환 임계 | move_actions.rs:180 | 36 | hp% < 36 (또는 <46 이고 적 챔피언 160000 안) 이면 타워 대신 분수 중심으로 도주. 올리면 더 일찍 풀회복 귀환 | 4 | 기존 |
| 4 | 타워 대신 분수 선택 여유 | move_actions.rs:188 | 30000 | 타워가 나보다 넥서스에서 30000 이상 멀고 적보다 30000 이상 나한테서 멀면 분수로. 올리면 타워 선호↑ | 4 | 기존 |
| 5 | 타워 주변 적 카운트 반경 | move_actions.rs:193 | 14400000000 | 120000^2. 타워 반경 안 적이 2명 이상이면 타워 포기하고 분수. 내리면 타워를 더 자주 택함 | 4 | 기존 |
| 6 | risk_weight 단계 | move_actions.rs:225~227 | 1/3/5 (nearby 0~1/2/3+, 반경 200000) | 근처 적 수에 따라 위험 점수 가중. 올리면 귀환거리보다 위험 회피 우선 | 4 | 기존 |
| 7 | for_nexus_weight | move_actions.rs:244 | 10 | free_dist 없을 때(version<=1 또는 구조물 전멸) 홈 셀 맨해튼 거리 가중. 올리면 귀환 방향성↑ | 4 | 기존 |
| 8 | b1_home_w_per_unit 기본/계수 | move_actions.rs:270 | chase_pct*risk_weight/5 + 2 | free_dist 한 셀당 가중. 기본 2, 추격 피해율(HP%/셀)이 클수록 귀환거리 가중↑ | 4 | 기존 |
| 9 | 적 접근 금지 여유 | move_actions.rs:344 | 8000 | 최근접 적에게 지금보다 8000 넘게 가까워지는 셀 제외. 내리면(0) 옆으로 새는 후보가 줄고, 올리면 적 방향 셀도 허용 | 4 | 기존 |
| 10 | 도달불가 셀 거리 치환 | move_actions.rs:374 / 441 | 150 | free_dist 0xFFFF 셀의 귀환거리. 낮추면 막힌 셀이 덜 불리 | 4 | 기존 |
| 11 | 후보 창(선수 파라미터 천분율) | move_actions.rs:416~420 | positioning_runaway_min_range/max_range → positioning_choice_window | 정렬된 후보에서 뽑는 순위 구간. 좁히면 최고점 셀에 수렴, 넓히면 무작위성↑ | 4 | 기존 |
| 12 | 후보 점수 부호 | move_actions.rs:375/378 | score = -(risk*rw + dist*w) | risk 와 거리 가중을 바꾸면 도주 방향(안전 vs 귀환)이 바뀜 | 4 | 기존 |
| 13 | SolvePolicy.direct_cross | move_actions.rs:488 | version < 2 | v2 이상에선 false 고정 — 경로계층 정책(내부 미독해) | 4 | 기존 |
| 14 | 클로저 v<=1 적 위험 반경 | move_actions.rs:526 | 6400000000 | 80000^2. is_enemy_danger_cell 인자. 올리면 더 넓게 Soft 판정 | 4 | 기존 |

<details><summary>`callees` 피호출자 54건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | collect_visible_enemy_positions | game_ai::collect_visible_enemy_positions | pub | fn(&game_core::PlayerState, &game_core::OperationData) -> ([(u64, u64); 5_usize], usize) | game-ai\src\path_finder.rs:1471 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | contains | game_core::RectU64::contains | pub | fn(&game_core::RectU64, u64, u64) -> bool | game-core\src\setting.rs:40 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 2 | contains | game_core::RectI64::contains | pub | fn(&game_core::RectI64, i64, i64) -> bool | game-core\src\setting.rs:55 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 3 | contains | game_core::setting::champion::nightmare::NightmareWellRect::contains | in:game_core::setting::champion::nightmare | fn(game_core::setting::champion::nightmare::NightmareWellRect, u64, u64) -> bool | game-core\src\setting\champion\nightmare.rs:28 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 4 | distance | game_core::utils::distance | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:12 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 5 | distance | game_core::Entity::distance | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2161 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 6 | dodge_tower_cell_with_context | game_ai::dodge_tower_cell_with_context | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_ai::TowerDodgeContext, usize, usize, usize, usize) -> bool | game-ai\src\path_finder.rs:1322 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 7 | enemy_knows_my_position | game_ai::enemy_knows_my_position | pub | fn(&game_core::PlayerState, &game_core::OperationData) -> bool | game-ai\src\path_finder.rs:1701 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 8 | fight_dps | game_ai::fight_dps | pub | fn(usize, &game_core::GameContext, &game_core::Entity, &game_core::Entity) -> usize | game-ai\src\fight_check.rs:32 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 9 | find_path | game_core::PathMap::find_path | pub | fn(&game_core::PathMap, usize, usize, usize, usize) -> std::option::Option<(usize, usize)> | game-core\src\setting.rs:82 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 10 | get_input | game_ai::PathFinder::get_input | pub | fn(&mut game_ai::PathFinder, &game_core::PlayerState, &game_core::OperationData, game_ai::SafeMoveWithSkill, bool) -> game_core::Input | game-ai\src\path_finder.rs:856 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 11 | goal | game_ai::plan_legacy::types::BigPlan::goal | pub | fn(&game_ai::plan_legacy::types::BigPlan) -> game_core::BigGoal | game-ai\src\plan_legacy\types.rs:132 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 16개 중 상위 3개 |
| 12 | goal | game_ai::plan_legacy::old::BattlePlan::goal | pub | fn(&game_ai::plan_legacy::old::BattlePlan) -> game_core::BigGoal | game-ai\src\plan_legacy\old\battle.rs:341 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 16개 중 상위 3개 |
| 13 | goal | game_ai::plan_legacy::old::SinglePlanLine::goal | pub | fn(&game_ai::plan_legacy::old::SinglePlanLine) -> game_core::BigGoal | game-ai\src\plan_legacy\old\single_line.rs:19 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 16개 중 상위 3개 |
| 14 | is_empty | game_core::PatchSetting::is_empty | pub | fn(&game_core::PatchSetting) -> bool | game-core\src\patch_engine.rs:853 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 15 | is_empty | game_core::ModAiRegistry::is_empty | pub | fn(&game_core::ModAiRegistry) -> bool | game-core\src\mod_ai.rs:15 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 16 | is_empty | game_core::PublicChampionEvidence::is_empty | pub | fn(&game_core::PublicChampionEvidence) -> bool | game-core\src\banpick\public_evidence.rs:96 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 17 | is_enemy_danger_cell | game_ai::is_enemy_danger_cell | pub | fn(usize, usize, u64, u64, &[(u64, u64); 5_usize], usize, u64, u32) -> bool | game-ai\src\path_finder.rs:1811 | True | True | 3 | IR 호출 심볼 일치(토큰 경계) |
| 18 | is_enemy_well_danger | game_ai::is_enemy_well_danger | pub | fn(usize, &game_core::PlayerState, u64, u64) -> bool | game-ai\src\path_finder.rs:1032 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 19 | is_recent_visible | game_core::Blackboard::is_recent_visible | pub | fn(&game_core::Blackboard, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\game\blackboard.rs:346 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 20 | isqrt | game_core::utils::isqrt | pub | fn(i64) -> i64 | game-core\src\utils.rs:85 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 21 | iter_towers | game_core::AbstractGameWithCache::<'a, 'b>::iter_towers | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5762 ~ game_core[6a30]::simulation::{impl#5}::iter_towers::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1908 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 22 | lethal_zone_goal | game_ai::PveHazardContext::lethal_zone_goal | pub | fn(&game_ai::PveHazardContext, u64, u64) -> bool | game-ai\src\path_finder.rs:1167 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 23 | new | game_ai::PveHazardContext::new | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData) -> game_ai::PveHazardContext | game-ai\src\path_finder.rs:1120 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 24 | new | game_ai::ZoneHazardContext::new | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData) -> game_ai::ZoneHazardContext | game-ai\src\path_finder.rs:1632 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 25 | new | game_ai::EnemyHazardContext::new | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData) -> game_ai::EnemyHazardContext | game-ai\src\path_finder.rs:1531 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 26 | new_target_with_policy | game_ai::PathFinder::new_target_with_policy | pub | fn(&mut rand::rngs::std::StdRng, &game_core::GameContext, usize, &str, game_ai::path_field::SolvePolicy, u64, u64, u64, u64, impl Fn(usize, usize, usize, usize) -> PathVerdict/#0) -> game_ai::PathFinder | game-ai\src\path_finder.rs:88 | True | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 27 | new_tower_avoid_v3 | game_ai::TowerDodgeContext::new_tower_avoid_v3 | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, u64, u64) -> game_ai::TowerDodgeContext | game-ai\src\path_finder.rs:1205 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 28 | nexus_final_stand | game_ai::plan_legacy::old::nexus_final_stand | pub | fn(&game_core::PlayerState, &game_core::OperationData) -> bool | game-ai\src\plan_legacy\old\defense_nexus.rs:190 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 29 | nexus_is_critical | game_ai::plan_legacy::old::nexus_is_critical | pub | fn(&game_core::PlayerState, &game_core::OperationData) -> bool | game-ai\src\plan_legacy\old\defense_nexus.rs:139 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 30 | positioning_choice_window | game_ai::small_action::positioning_choice_window | in:game_ai::small_action | fn(usize, &game_core::PlayerState, usize, usize, bool) -> (usize, usize) | game-ai\src\small_action.rs:43 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 31 | positioning_risk_value | game_ai::small_action::positioning_risk_value | in:game_ai::small_action | fn(usize, &game_core::PlayerState, i64, bool) -> i64 | game-ai\src\small_action.rs:79 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 32 | positioning_runaway_max_range | game_core::AthleteParameter::positioning_runaway_max_range | pub | fn(&game_core::AthleteParameter) -> usize | game-core\src\simulation\state\player.rs:324 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 33 | positioning_runaway_min_range | game_core::AthleteParameter::positioning_runaway_min_range | pub | fn(&game_core::AthleteParameter) -> usize | game-core\src\simulation\state\player.rs:318 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 34 | positioning_score_at_cell | game_ai::small_action::positioning_score_at_cell | in:game_ai::small_action | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, i64, i64, game_ai::PositionEvalPurpose) -> game_core::PositioningScore | game-ai\src\small_action.rs:94 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 35 | positioning_score_at_position | game_ai::small_action::positioning_score_at_position | in:game_ai::small_action | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, u64, u64, game_ai::PositionEvalPurpose) -> game_core::PositioningScore | game-ai\src\small_action.rs:99 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 36 | positioning_window_pick | game_ai::small_action::positioning_window_pick | in:game_ai::small_action | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &[T/#0]) -> &T/#0 | game-ai\src\small_action.rs:66 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 37 | push | <game_core::Staff as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::staff | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\staff.rs:38 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 38 | push | <game_core::Athlete as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:434 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 39 | push | <game_core::Contract as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:2498 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 40 | shared_free_dist | game_ai::shared_free_dist | pub | fn(&game_core::MapDef) -> std::sync::Arc<game_ai::FreeDistMap, std::alloc::Global> | game-ai\src\free_dist.rs:138 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 41 | team | game_view::ClientData::team | pub | fn(&game_view::ClientData, usize) -> std::option::Option<std::cell::Ref< game_core::Team>> | game-view\src\logic\client\data.rs:1651 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 42 | team | game_view::ClientDatabase::team | pub | fn(&game_view::ClientDatabase, usize) -> std::option::Option<&game_core::Team> | game-view\src\logic\client\data.rs:968 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 43 | team | game_core::PlayerAiContext::<'a, 'b, 'r>::team | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:583 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 44 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 45 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 46 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 47 | towers | game_core::AbstractGameWithCache::<'a, 'b>::towers | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize, &'b/#1 bumpalo::Bump) -> bumpalo::collections::vec::Vec< &game_core::Entity> | game-core\src\simulation.rs:1859 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 48 | update_path | game_ai::PathFinder::update_path | pub | fn(&mut game_ai::PathFinder, &mut rand::rngs::std::StdRng, &game_core::GameContext, u64, u64, u64, u64, impl Fn(usize, usize, usize, usize) -> PathVerdict/#0) | game-ai\src\path_finder.rs:631 | True | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 49 | v3_deadly_edge_cells | game_ai::v3_deadly_edge_cells | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData) -> u32 | game-ai\src\tower_discipline.rs:304 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 50 | v3_lethal_tower_hp | game_ai::v3_lethal_tower_hp | pub | fn(&game_core::OperationData, &game_core::PlayerState, &game_core::Entity) -> bool | game-ai\src\tower_discipline.rs:272 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 51 | v3_lethal_tower_position | game_ai::v3_lethal_tower_position | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, u64, u64) -> bool | game-ai\src\tower_discipline.rs:282 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 52 | v3_lethal_wave_position | game_ai::v3_lethal_wave_position | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, u64, u64) -> bool | game-ai\src\minion_wave_risk.rs:251 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 53 | verdict | game_ai::ZoneHazardContext::verdict | pub | fn(&game_ai::ZoneHazardContext, u64, u64, usize, usize) -> std::option::Option<game_ai::PathVerdict> | game-ai\src\path_finder.rs:1675 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
</details>

⚠**미매칭 9개**: `__rust_dealloc`, `cell_scores`, `clamp`, `dist_sq`, `drop  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `eff_risk`, `sat_mul`, `sat_sub`, `sort_by_key`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 4곳** (m02.ll:64364, m08.ll:94044, m11.ll:43012, m11.ll:46546) · **형제 14개** (SmallActionRunAway)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::SmallActionRunAway as std::clone::Clone>::clone | pub | game-ai\src\small_action\move_actions.rs:8 | True | fn(&game_ai::SmallActionRunAway) -> game_ai::SmallActionRunAway |
| 1 | <game_ai::SmallActionRunAway as std::fmt::Debug>::fmt | pub | game-ai\src\small_action\move_actions.rs:8 | True | fn(&game_ai::SmallActionRunAway, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 2 | game_ai::SmallActionRunAway::new | pub | game-ai\src\small_action\move_actions.rs:25 | False | fn(&game_core::OperationData, &game_core::PlayerState, usize) -> game_ai::SmallActionRunAway |
| 3 | game_ai::SmallActionRunAway::new_with_skill | pub | game-ai\src\small_action\move_actions.rs:29 | False | fn(&game_core::OperationData, &game_core::PlayerState, usize, bool) -> game_ai::SmallActionRunAway |
| 4 | game_ai::SmallActionRunAway::new_with_ult | pub | game-ai\src\small_action\move_actions.rs:53 | False | fn(&game_core::OperationData, &game_core::PlayerState, usize) -> game_ai::SmallActionRunAway |
| 5 | game_ai::SmallActionRunAway::new_dodge | pub | game-ai\src\small_action\move_actions.rs:77 | False | fn(&game_core::OperationData, &game_core::PlayerState, usize) -> game_ai::SmallActionRunAway |
| 6 | game_ai::SmallActionRunAway::is_with_ult | pub | game-ai\src\small_action\move_actions.rs:84 | True | fn(&game_ai::SmallActionRunAway) -> bool |
| 7 | game_ai::SmallActionRunAway::is_stalled | pub | game-ai\src\small_action\move_actions.rs:89 | True | fn(&game_ai::SmallActionRunAway, usize) -> bool |
| 8 | game_ai::SmallActionRunAway::get_input | in:game_ai | game-ai\src\small_action\move_actions.rs:95 | False | fn(&mut game_ai::SmallActionRunAway, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &mut game_core::DebugFrameData) -> std::option::Option<game_core::Input> |
| 9 | game_ai::SmallActionRunAway::merge | in:game_ai | game-ai\src\small_action\move_actions.rs:605 | False | fn(&mut game_ai::SmallActionRunAway, game_ai::SmallActionRunAway) |
| 10 | game_ai::SmallActionRunAway::update_state | in:game_ai | game-ai\src\small_action\move_actions.rs:626 | False | fn(&mut game_ai::SmallActionRunAway, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 11 | game_ai::SmallActionRunAway::get_action | in:game_ai | game-ai\src\small_action\move_actions.rs:629 | True | fn(&game_ai::SmallActionRunAway) -> game_core::SmallAction |
| 12 | game_ai::SmallActionRunAway::is_end | in:game_ai | game-ai\src\small_action\move_actions.rs:633 | False | fn(&game_ai::SmallActionRunAway, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bool |
| 13 | game_ai::SmallActionRunAway::near_move_complete | in:game_ai | game-ai\src\small_action\move_actions.rs:640 | False | fn(&game_ai::SmallActionRunAway, &game_core::Entity) -> bool |

**`open` 9건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | prog_best_dist_sq(+0x28) 이름은 dist_sq 인데 실제 저장값은 isqrt 된 거리(+경로 잔량*32000) — 소스 표기 문제, 동작은 IR 정본대로 기록 | 3 |  |
| 1 | 미탐색 | L404 `picked_real_candidate` 라벨이 IR 에선 `len==0` 값(%962)에 붙어 있음(dbg_value 어긋남). 논리는 분기 방향으로 기록했고 변수명은 신뢰하지 않음. L184 `force_full_heal` 도 같은 종류(nexus_final_stand 결과에 붙음) | 4 |  |
| 2 | 미탐색 | sf_0 클로저 m03 판이 i32(태그)만 반환하도록 DeadArgElim 된 이유(호출자 new_target_with_policy<sf_0> 가 Priced 페이로드를 안 읽는지)는 path_finder 내부라 안 봄 — 미탐색(경로계층 명세 몫) | 4 |  |
| 3 | 미탐색 | PathFinder::new_target_with_policy / update_path / get_input 내부(경로 계층·FieldScratch/PathScratch TLS) — 지시대로 미독해. self 소유 Box 의 제자리 쓰기 범위(path 1120B / planned_verdict 70B 중 어디까지) 는 그 명세에서 확정해야 함 | 4 |  |
| 4 | 미탐색 | AbstractGame::tick 슬롯(+0x28) 은 divtable 로 이름 확인했으나 ExpectedGame 구현체 기준 — 실행 시 dyn 타입이 다른 구현이어도 슬롯 순서는 동일(트레이트 vtable) | 3 |  |
| 5 | 미탐색 | L317 분수측 제외 조건의 의도(왜 구조물 전멸 시 자기 분수쪽 셀을 빼는지)는 주석 없음 — 동작만 기록 | 4 |  |
| 6 | 미탐색 | fountain_center 등 hx/hy 가 '홈' 좌표라는 해석은 dbg 이름(fountain_center) + 흐름에서 유도. L201 타워 좌표도 같은 변수에 들어감 | 4 |  |
| 7 | 미탐색 | _docs 에 이 함수 전용 개발자 주석 없음(RunAway 는 계측/분류 주석만 존재) — rmetadocs grep 결과 | 4 |  |
| 8 | 미탐색 | m00.ll 의 sf_0/si_0 사본(164줄, {i32,i32} 반환)은 같은 클로저의 다른 CGU 인스턴스 — 본문 축약 원인(상수 전파 여부) 미확인, 명세는 m03 판(537줄) 기준 | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

