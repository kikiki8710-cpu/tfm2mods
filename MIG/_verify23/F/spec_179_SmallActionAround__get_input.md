---

### `179` SmallActionAround::get_input — 엔티티(target, 보통 타워/오브젝트) 주변 range 안에서 가치 높은 셀을 goal 로 잡고 PathFinder 로 이동; escape_mode 면 적 반대편이면 분수로 goal 을 바꾸고, 타워 탈출이 필요하면 RunAway 로 위임

| 항목 | 값 |
|---|---|
| id | `SmallActionAround__get_input` |
| 심볼 | `_RNvMNtNtCshdEBA0ozCnw_7game_ai12small_action6aroundNtB2_17SmallActionAround9get_input` |
| 소스 | `game-ai\src\small_action\around.rs:61` |
| IR | `m08.ll` 92628~94133행 |
| 경로·가시성 | `game_ai::SmallActionAround::get_input` · **in:game_ai** |
| 계층 | 기타 |
| exe | `db6ff0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r14` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&mut game_ai::SmallActionAround, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &mut game_core::DebugFrameData) -> std::option::Option<game_core::Input>
```

<details><summary>인자 8개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | *mut Option<game_core::Input> (32B) | None = `store i64 -1` +0 만(92726·92758·93972). Some = (a) 94048 스택 %12 에서 32B memcpy (b) 94044 SmallActionRunAway::get_input 이 sret %0 에 직접 기록 | 4 |
| 1 | 1 | self | &mut SmallActionAround (136B) | noalias·dereferenceable(136)·readonly 없음 → &mut. writes 전수는 writes 항목 | 4 |
| 2 | 2 | version | usize | 스택 %35(92668)에 저장돼 클로저 캡처. 분기: version>1(93775) · policy.direct_cross = version<2(93234) | 4 |
| 3 | 3 | rnd | &mut StdRng (320B) | 직접 쓰기 0 — positioning_window_pick·new_target_with_policy·update_path·is_safe_recall·SmallActionRunAway::get_input 에 전달 | 4 |
| 4 | 4 | player | &PlayerState (2528B) | readonly | 4 |
| 5 | 5 | data | &OperationData (24B) | readonly. cache(+0)/context(+8)/blackboard(+0x10) 모두 사용 | 4 |
| 6 | 6 | positioning_score | &PositioningScoreData (2760B) | readonly | 4 |
| 7 | 7 | debug | &mut DebugFrameData (224B) | readnone captures(none) — 미사용(죽은 인자). 콜리 SmallActionRunAway::get_input 에도 poison 으로 넘김 = 그 콜리도 안 읽는다는 LLVM 판정 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn get_input(&mut self, version, rnd, player, data, ps, debug /*미사용*/) -> Option<Input>   [around.rs:61~264]
// 63: target = data.cache.game.get_entity_by_id(self.target)?          (vtable +0x1f0 · null → None 92726)
// 65: champ = data.cache.player_champion[player.team][player.position.as_index()]?   (None → 92758)
// 67: skip_around = false
// 69: if self.escape_mode {
//   70-72: nearest_enemy = data.cache.iter_champions(1 - team)                                 (적 팀 5칸, 92884-93004 언롤)
//            .filter(|e| data.blackboard[1 - team].is_recent_visible(game, player, e))          (closure#0, 71)
//            .min_by_key(|e| e.distance_sq(champ))                                              (closure#1, 72 · 첫 원소 인라인 후 Map::fold 93083)
//   74: if let Some(enemy) = nearest_enemy {
//     75-79: to_tower = target.pos - champ.pos; away_enemy = champ.pos - enemy.pos; dot = to_tower.x*away_enemy.x + to_tower.y*away_enemy.y   (i64)
//     81: if dot <= 0 {                                                                        (icmp slt 1 — 적이 타워 반대편/직각)
//       82: f = map.fountains[team]  (lx,ly,rx,ry)
//       83-84: self.goal_x = (lx+rx)/2; self.goal_y = (ly+ry)/2;  skip_around = true          (lshr 1)
//     }
//   }
// }
// 90: dist = distance_sq(champ.pos, self.goal);  91: goal_to_target = distance_sq(self.goal, target.pos)
// 93: pve_hazard = PveHazardContext::new(version, player, data)
// 94: if !skip_around && (goal_to_target > 9215999999 /*≥96000²*/ || dist <= 25000000 /*5000²*/ || dist >= 9216000000 /*96000²*/) {   (93180-93185, 범위검사 접힘)
//   95-96: xi = ps.cx as i32; yi = ps.cy as i32;   98: candidates: bumpalo Vec<(i32,i32,i32,i32,i64)> = new_in(pool)
//   100-129: for dx in 0..7 { for dy in 0..7 {
//       xb=xi-3+dx; yb=yi-3+dy; x=xb*32000+16000; y=yb*32000+16000
//       108: if xb<0||yb<0||xb>29||yb>29 → continue; if map.walls[yb][xb]!=0 → continue
//       111: if is_enemy_well_danger(version, player, x, y) → continue
//       115: if v3_lethal_tower_position(version, player, data, x, y) → continue
//       116: if v3_lethal_wave_position(version, player, data, x, y) → continue
//       117: if pve_hazard.lethal_zone_goal(x, y) → continue
//       120: d = utils::distance(x, y, target.x, target.y)
//       122: if d >= self.range → continue                                                    (ult 라 `<` 만 통과)
//       126: score = positioning_cell_value_at(version, player, data, ps, x, y, self.position_eval_purpose)
//       128: candidates.push((dx, dy, x, y, score*10000 + d/1000))
//   } }
//   132: let (gx, gy) = if candidates.is_empty() { (target.x, target.y) } else {
//       135: candidates.sort_by_key(|c| -c.4)                                                   (키 내림차순)
//       136-140: (min_range,max_range) = positioning_choice_window(version, player, param.positioning_min_range(), param.positioning_max_range(), false);
//                min_idx = min(len-1, len*min_range/1000); max_idx = clamp(len*max_range/1000, min_idx, len-1)
//       142-144: candidates = into_iter().skip(min_idx).take(max_idx-min_idx+1).collect()
//       146: pick = positioning_window_pick(version, rnd, player, data, &candidates);  (x, y) = (pick.2, pick.3)
//       148-149: pre = positioning_score_at_position(version, player, data, ps, self.goal_x, self.goal_y, purpose); pre_score = pre.gain - pre.risk
//       151-153: g = positioning_score_at_cell(version, player, data, ps, x/32000, y/32000, purpose); self.goal_gain = g.gain - g.risk      (★새 후보 기준)
//       154: if pre_score >= self.goal_gain && self.goal_x < 960000 && self.goal_y < 960000 && dist > 143999999 (champ↔이전 goal ≥12000) { (self.goal_x, self.goal_y) } else { (x, y) }
//   };
//   161-162: self.goal_x = gx; self.goal_y = gy;   163: drop(candidates)
// }
// 165: tower_dodge = TowerDodgeContext::new_unnecessary_tower_avoid(version, player, data, goal_x, goal_y)      (★Region/Position 의 new_tower_avoid_v3 와 다름)
// 170: (enemy_positions, visible_enemy_count) = collect_visible_enemy_positions(player, data)
// 171: (unseen_positions, unseen_count) = collect_unseen_enemy_estimates(version, player, data)
// 172: cross = SafeCrossRoute::new(version, player, data, champ.x, champ.y, goal_x, goal_y)
// 173: is_far_travel = distance_sq(champ.pos, goal) > 102400000000 (320000²)
// 176-177: policy = SolvePolicy { deadly_cells: v3_deadly_edge_cells(version, player, data), direct_cross: version < 2 }
// 179: if self.path_finder.is_none() {
//   180: self.path_finder = Some(PathFinder::new_target_with_policy(rnd, ctx, version, "dodge_unnecessary_tower_cell", policy, champ.x, champ.y, goal_x, goal_y,
//        closure#3 |sx,sy,nx,ny| -> PathVerdict {                                               (aux m03.ll:130787)
//           182: if tower_dodge.deadly_band_cell(version, nx, ny) → Deadly(3)
//           185: if !dodge_unnecessary_tower_cell_with_context(version, player, data, ps, &tower_dodge, sx,sy,nx,ny) → Danger(2)
//           189: if let Some(v) = pve_hazard.verdict(version, player, data, nx, ny) → return v     (인라인 path_finder.rs:1172-1178: lethal_zone_goal(셀중심) → Deadly(3); 아니면 wave 플래그(+0x1e8) && v3_lethal_wave_position(version,player,data,셀중심) → Soft(1); 아니면 None)
//           192: if is_far_travel && is_enemy_danger_cell(nx,ny, goal, &enemy_positions, visible_enemy_count, 6400000000, 1) → Soft(1)
//           195: if is_enemy_danger_cell(nx,ny, goal, &unseen_positions, unseen_count, 6400000000, 1) → Soft(1)
//           198: if cross.blocks(data, nx, ny) → Soft(1) else Allow(0)
//        }))
// }
// 206: if self.path_finder.as_ref().is_some_and(|pf| pf.unconstrained_fallback) {
//   207: lethal_tower = version > 1 && v3_lethal_tower_hp(data, player, champ)
//   208: retry = PathFinder::new_target_with_policy(rnd, ctx, version, "cross_only", policy, champ.x, champ.y, self.goal_x, self.goal_y,
//        closure#5 |sx,sy,nx,ny| {                                                              (aux m03.ll:131268)
//           210: if tower_dodge.deadly_band_cell(version, nx, ny) → Deadly(3)
//           214: if lethal_tower && !dodge_tower_cell_with_context(version, player, data, ps, &tower_dodge, sx,sy,nx,ny) → Danger(2)      (★여기는 unnecessary 판이 아닌 일반 dodge_tower_cell_with_context)
//           217: if cross.blocks(data, nx, ny) → Soft(1) else Allow(0)
//        })
//   219: if !retry.unconstrained_fallback { 220: self.path_finder = Some(retry) } else { 222: drop(retry) }
// }
// 224: let p = self.path_finder.as_mut()?          (None → 93972, 실질 도달 불가)
// 226: p.update_path(rnd, ctx, champ.x, champ.y, self.goal_x, self.goal_y, closure#6 /*closure#3 과 동일, aux 131590*/)
// 250: if path_needs_tower_escape(version, player, data, p) {
//   251: let mut runaway = SmallActionRunAway::new_with_skill(data, player, 5, true)    (인라인: healp = team==0 ? (32000,928000) : (928000,32000); start_tick=game.tick(); path_finder=None; with_skill=true)
//   252: return runaway.get_input(version, rnd, player, data, ps, debug /*poison*/)     (sret 직접 · 253: drop(runaway))
// }
// 255: input = p.get_input(player, data, SafeMoveWithSkill::Safe(1), is_safe_recall(version, rnd, player, data, ps))
// 257: if input == Input::Move{x:0,y:0} { 258: println!("Error move!"); 259: p.debug_print() }
// 262: return Some(input)
```

**`mem` 메모리 접근 51건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | OperationData | 0x0 | cache | r | 92703 | 4 | OK |  |
| 1 | OperationData | 0x8 | context (&GameContext) | r | 93119·93255·93675·93795·93933 | 4 | OK |  |
| 2 | OperationData | 0x10 | blackboard (&[Blackboard;2]) | r | 92824-92825 · [1-team] 로 인덱스(92882 gep 구조체 stride · %82 = 1-team) → Blackboard::is_recent_visible 의 self | 4 | OK |  |
| 3 | AbstractGameWithCache | 0x0 | game (&dyn AbstractGame data ptr) | r | 92705·94015 | 4 | OK |  |
| 4 | AbstractGameWithCache | 0x8 | game vtable | r | 92707 · 슬롯 +0x1f0(496) = get_entity_by_id(92710-92712, divtable) · 슬롯 +0x28(40) = tick(94017-94019, divtable) | 3 | OK |  |
| 5 | AbstractGameWithCache | 0x1e0 | player_champion[team][pos] / iter_champions(1-team) = 같은 배열의 적 팀 행 5개 | r | 92741-92744 champ · 92816-92823·92884-93004 적 5칸 언롤 | 4 | OK |  |
| 6 | PlayerState | 0x930 | info.team | r | 92719 (bounds 2) · 1-team(92814) · fountains 인덱스(93125) · RunAway healp 선택(94010) | 4 | OK |  |
| 7 | PlayerState | 0x9c0 | info.position@tag → as_index | r | 92738-92740 | 4 | OK |  |
| 8 | PlayerState | 0x180 | info.parameter | r | 93376 · positioning_min/max_range | 4 | OK |  |
| 9 | GameContext | 0x0 | pool (&Bump) | r | 93257 | 4 | OK |  |
| 10 | GameContext | 0x20 | map (&MapDef) | r | 93121·93268·93540 | 4 | OK |  |
| 11 | MapDef | 0x6d70 | fountains[team] ((u64 lx,u64 ly,u64 rx,u64 ry) stride 32) | r | 93124-93135 (map_def.rs:235 fountain 인라인) | 4 | OK |  |
| 12 | MapDef | 0x78 | walls[yb][xb] | r | 93541-93546 | 4 | OK |  |
| 13 | PositioningScoreData | 0xab8 | cx | r | 93246 | 4 | OK |  |
| 14 | PositioningScoreData | 0xac0 | cy | r | 93250 | 4 | OK |  |
| 15 | Entity | 0x660 | x (champ 92764 · target 92795·93092 · nearest_enemy 93102 · 적 후보 93052) | r |  | 4 | OK |  |
| 16 | Entity | 0x668 | y | r | 92772·92800·93097·93106·93056 | 4 | OK |  |
| 17 | SmallActionAround | 0x8 | target (엔티티 id) | r | 92708-92709 → get_entity_by_id | 4 | OK |  |
| 18 | SmallActionAround | 0x10 | goal_x | r | 92781·93797·93935·93464(비교) | 4 | OK |  |
| 19 | SmallActionAround | 0x18 | goal_y | r | 92785·93798·93936 | 4 | OK |  |
| 20 | SmallActionAround | 0x28 | range | r | 93583-93584 · distance(cell, target) < range 인 셀만 후보 | 4 | OK |  |
| 21 | SmallActionAround | 0x7d | path_finder@tag (= unconstrained_fallback 바이트, 2=None) | r | 93240-93242 is_none · 93764-93769 is_some_and(closure#4 = \|pf\| pf.unconstrained_fallback) · 93780 as_mut | 4 | OK |  |
| 22 | SmallActionAround | 0x60 | path_finder.path (Box 1120B) | r | 93714·93828 (gep 96) · 교체 시 dealloc | 4 | OK |  |
| 23 | SmallActionAround | 0x68 | path_finder.planned_verdict (Box 70B) | r | 93716·93830 (gep 104) | 4 | OK |  |
| 24 | SmallActionAround | 0x80 | position_eval_purpose (i8) | r | 93453·93588 · positioning_cell_value_at/score_at_position/score_at_cell 7번째 인자 | 4 | OK |  |
| 25 | SmallActionAround | 0x81 | escape_mode (bool) | r | 92752-92754 · 참이면 69-88 분수 전환 판정 | 4 | OK |  |
| 26 | PositioningScore | 0x0 | risk | r | 93460(pre)·93481(goal) | 4 | OK |  |
| 27 | PositioningScore | 0x10 | gain | r | 93458(pre)·93479(goal) · score = gain - risk | 4 | OK |  |
| 28 | PathFinder(retry, 스택 %16) | 0x45 | unconstrained_fallback | r | 93815-93817 (gep 69) | 4 | OK |  |
| 29 | PathFinder(retry) | 0x28 | path (Box) | r | 93878 (gep 40) | 4 | OK |  |
| 30 | PathFinder(retry) | 0x30 | planned_verdict | r | 93880 (gep 48) | 4 | OK |  |
| 31 | Input (스택 %12, PathFinder::get_input 결과) | 0x0 | tag | r | 93991-93992 · ==0(Move) | 4 | OK |  |
| 32 | Input (스택 %12) | 0x8 | Move.x | r | 93993-93995 · ==0 | 4 | OK |  |
| 33 | Input (스택 %12) | 0x10 | Move.y | r | 93997-93999 · ==0 → "Error move!" 출력 | 4 | OK |  |
| 34 | 후보 튜플 (i32 dx,i32 dy,i32 x,i32 y,i64 key) 24B | 0x8 | .2 = x | r | 93463 (positioning_window_pick 반환 &T) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 35 | 후보 튜플 | 0xc | .3 = y | r | 93467 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 36 | 후보 튜플 | 0x10 | .4 = key = score*10000 + d/1000 (정렬 키, 부호 반전) | r | m03.ll:117712-117730 insertion_sort 인스턴스: `sub 0, 원소+16` 후 slt → closure#2(135) = `\|c\| -c.4` = 키 내림차순 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 37 | TowerDodgeContext (aux, deadly_band_cell 인라인) | 0x0 | towers.len(≤16)·+0x8 r·+0x18.. stride 24(x,y,flag@+18) | r | aux 130829-130947 — path_finder 소관, 관측만 | 4 | 오귀속(사전은 다른 필드를 준다) |  |
| 38 | PveHazardContext (aux, verdict 인라인) | 0x1e0 | zones.len(≤20, bounds 21) / +0x0.. stride 24 (x,y,r) / +0x1e8 wave 플래그 | r | aux 130989-131084 (path_finder.rs:1168·1178) — 관측만 | 4 | 오귀속(사전은 다른 필드를 준다) |  |
| 39 | SmallActionAround | 0x10 | goal_x | w | 93140 (around.rs:83) | 4 | OK | (fountain.lx + fountain.rx)/2 — escape_mode && 가장 가까운 보이는 적이 타워 방향과 반대(dot<=0)일 때 |
| 40 | SmallActionAround | 0x18 | goal_y | w | 93144 (around.rs:84) | 4 | OK | (fountain.ly + fountain.ry)/2 |
| 41 | SmallActionAround | 0x10 | goal_x | w | 93351 (around.rs:161) — 재선택 경로(94 조건 참)에서만 | 4 | OK | gx (target.x \| 창에서 고른 후보 x \| 이전 goal 유지) |
| 42 | SmallActionAround | 0x18 | goal_y | w | 93352 (around.rs:162) | 4 | OK | gy |
| 43 | SmallActionAround | 0x20 | goal_gain | w | 93484 (around.rs:153). 후보 0개 경로에서는 갱신 안 됨 | 4 | OK | positioning_score_at_cell(version,player,data,ps,pick.x/32000,pick.y/32000,purpose).gain - .risk (새 후보 기준 — 이전 goal 유지 시에도 새 후보 값이 남음) |
| 44 | SmallActionAround | 0x38 | path_finder (Option<PathFinder> 72B, +0x38..+0x80) | w | 93666 memcpy 72B (around.rs:180). 직전 drop_glue(93709-93761)는 tag==2 라 dealloc 미실행 | 4 | OK | Some(PathFinder::new_target_with_policy(rnd,ctx,version,"dodge_unnecessary_tower_cell",policy,champ.x,champ.y,gx,gy,closure#3)) — is_none 이었을 때 |
| 45 | SmallActionAround | 0x38 | path_finder | w | 93928 (around.rs:220). 기존 Box 2개 dealloc 93823-93875 | 4 | OK | Some(retry) = new_target_with_policy(...,"cross_only",policy,champ.x,champ.y,goal_x,goal_y,closure#5) — 기존 pf.unconstrained_fallback && !retry.unconstrained_fallback |
| 46 | SmallActionAround | 0x38 | path_finder 내부 (update_path / get_input / path_needs_tower_escape 의 &PathFinder) | w | 93965·93990·93968 | 4 | OK | 콜리 소관 |
| 47 | SmallActionRunAway (스택 %13, 136B, 지역) | 0x0 | new_with_skill 인라인 초기화 — start_tick=game.tick() · goal=(team==0 ? (32000,928000) : (928000,32000)) · end_delay(+0x18)=5 · goal_risk(+0x20)=0 · prog_best_dist_sq(+0x28)=u64::MAX(-1) · prog_best_tick(+0x30)=0 · path_finder(+0x38) None(태그 바이트 +0x7d=2) · with_skill(+0x80)=1 · with_ult/dodge_trajectory/goal_committed(+0x81..+0x83)=0 | w | 94010-94042 (move_actions.rs:30-37 인라인) — self 가 아니라 스택. 부작용은 RunAway::get_input 내부 | 4 | 오귀속(사전은 다른 필드를 준다) | 지역 객체(호출 후 drop 94075-94128) |
| 48 | StdRng(rnd) | 0x0 | (콜리 경유) | w | 직접 store 없음 | 4 | OK | positioning_window_pick·new_target_with_policy·update_path·is_safe_recall·SmallActionRunAway::get_input |
| 49 | sret Option<Input> | 0x0 | 전체 | w |  | 4 | 확인불가(tcx 사전에 타입 없음) | -1(None 92726·92758·93972) / RunAway::get_input 직접 기록(94044) / 스택 %12 memcpy 32B(94048) |
| 50 | stdout | 0x0 | std::io::_print("Error move!\n") + PathFinder::debug_print | w | 94061-94062 (around.rs:258-259) — PathFinder::get_input 이 Move{0,0} 을 돌려줄 때만 | 4 | 확인불가(tcx 사전에 타입 없음) | 부작용 |

**`consts` 상수 23건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 2 | 65 | 태그 | 팀 bounds(92722) · path_finder None 태그 `icmp eq i8, 2`(93242·93670·93710·93781·93824) · RunAway.path_finder None 초기화 `store i8 2`(94026) · `version < 2`(93234) · aux Danger 태그 2 | 4 |
| 1 | 1 | 70 | 태그 | enemy_team = 1 - team(92814) · `dot < 1`(93114) = dot<=0 (적이 타워 반대편) · version>1(93775) · take(+1)(93425) · SafeMoveWithSkill::Safe i8 1(93990) · with_skill=1(94030) · aux Soft 태그 1·is_enemy_danger_cell 마지막 인자 1 | 4 |
| 2 | 7 | 100 | 임계 | 후보 창 한 변 7 셀 | 4 |
| 3 | -3 | 101 | 계수 | 창 반폭 3: xb=cx-3+dx, yb=cy-3+dy | 4 |
| 4 | 32000 | 103 | 인덱스 | 셀 크기 — x=xb*32000+16000 · pick.x/32000 → 셀 인덱스(93465·93468) · RunAway healp 좌표 32000(94011-94012) · aux 셀 중심 환산 | 4 |
| 5 | 16000 | 103 | 계수 | 셀 중심 오프셋 | 4 |
| 6 | 29 | 108 | 임계 | 격자 최대 인덱스 — xb>29\|\|yb>29 제외 | 4 |
| 7 | 0 | 108 | 태그 | walls==0 만 후보(93545) · Input::Move 태그 0 / x==0 / y==0 판정(93992-94000) · RunAway 필드 0 초기화 · aux Allow 태그 0 | 4 |
| 8 | 10000 | 128 | 계수 | 후보 키 = score*10000 + d/1000 — 가치가 만 단위로 우선, 같은 가치면 target 에서 먼 셀(d/1000 큼)이 앞(내림차순 정렬) | 4 |
| 9 | 1000 | 128 | 계수 | d/1000 (키의 거리 항, sdiv 93595) · 창 per-mille 분모(93395·93405) | 4 |
| 10 | 9215999999 | 94 | 임계 | 96000²-1 — goal_to_target(distance_sq(goal, target)) > 이 값(즉 ≥96000) 이면 goal 재선택 조건 ① | 4 |
| 11 | -9216000000 | 90 | 계수 | -(96000²). 94 의 범위검사 접힘: `(dist - 96000²) ult (25000001 - 96000²)` ⟺ dist < 25000001 (≤5000²) \|\| dist ≥ 96000² — 챔피언이 goal 에 5000 이내 도달 또는 goal 에서 96000 이상 멀면 재선택 조건 ② | 4 |
| 12 | -9190999999 | 94 | 임계 | = 25000001 - 9216000000 (위 접힘의 비교 상수). 소스 수준 임계 5000²·96000² | 4 |
| 13 | 960000 | 154 | 임계 | goal 좌표 유효 상한(30*32000) — goal_x/y < 960000 일 때만 이전 goal 유지 판정 | 4 |
| 14 | 143999999 | 154 | 임계 | 12000²-1 — distance_sq(champ, 이전 goal) > 이 값(≥12000) && pre_score >= goal_gain 이면 이전 goal 유지 | 4 |
| 15 | 102400000000 | 173 | 임계 | 320000² — distance_sq(champ, goal) > 이 값이면 is_far_travel (클로저에서 보이는 적 회피 검사 활성) | 4 |
| 16 | 5 | 251 | 길이 | SmallActionRunAway::new_with_skill 의 end_delay = 5 (94028) · 92694 len=5 는 iter_champions 배열 길이 | 4 |
| 17 | 928000 | 251 | 산출값 | = 29*32000 — RunAway 대피 목표(healp): team0 (32000, 928000) / team1 (928000, 32000) (move_actions.rs:30 인라인 94011-94012) | 4 |
| 18 | -1 | 63 | 센티널 | Option<Input>::None 니치 태그(92726·92758·93972) · RunAway.prog_best_dist_sq 초기값 u64::MAX(94040) · 93396 의 -1 은 len-1 | 4 |
| 19 | 28 | 180 | 길이 | PathFinder key 길이 — @anon...40 = "dodge_unnecessary_tower_cell"(28B) | 4 |
| 20 | 10 | 208 | 길이 | key 길이 — @anon...41 = "cross_only"(10B) | 4 |
| 21 | 6400000000 | 192 | 미상 | (aux closure#3/#6) 80000² — is_enemy_danger_cell 반경 제곱. 보이는 적(192, is_far_travel 일 때만)·미관측 적(195) 둘 다 | 4 |
| 22 | 3 | 182 | 태그 | (aux) PathVerdict::Deadly 태그 3 — tower_dodge.deadly_band_cell(182) 또는 pve_hazard.verdict==Some(Deadly)(189, lethal_zone_goal) 일 때 반환 | 4 |

**`knobs` 조정점 9건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 후보 키 가중 score*10000 + d/1000 | around.rs:128 | 10000 | 10000 을 줄이면 거리 항(d/1000)이 가치와 경쟁 — target 에서 먼 셀 선호가 강해진다; 현재는 가치가 사실상 절대 우선 | 4 | 기존 |
| 1 | 후보 반경 = self.range (필드, +0x28) | around.rs:122 | 필드 | 키우면 target 에서 더 먼 셀도 후보 | 4 | 기존 |
| 2 | goal 재선택 조건: goal↔target 96000 / champ↔goal 5000 도달·96000 이탈 | around.rs:94 (9215999999 · -9190999999) | 9215999999 | 96000 을 키우면 goal 이 target 에서 더 멀어져도 유지; 5000 을 키우면 더 일찍 다음 셀 재선택 | 4 | 기존 |
| 3 | 이전 goal 유지 거리 12000 (143999999) | around.rs:154 | 143999999 | Region 과 동일한 히스테리시스 | 4 | 기존 |
| 4 | is_far_travel 320000 (102400000000) | around.rs:173 | 102400000000 | 내리면 짧은 이동에서도 보이는 적 회피(Soft) | 4 | 기존 |
| 5 | 적 위험 반경 80000 (6400000000) | around.rs:192·195 (aux) | 6400000000 | 올리면 적 주변을 더 넓게 우회 | 4 | 기존 |
| 6 | escape_mode 분수 전환 조건 dot<=0 | around.rs:81 | 1 | `dot < 1` 을 다른 값으로 바꾸면 적이 타워 쪽에 있어도(양의 내적) 분수로 도망칠 수 있다 | 4 | 기존 |
| 7 | RunAway 위임 end_delay=5·with_skill=true | around.rs:251 | 5 | path_needs_tower_escape 참일 때 만드는 RunAway 의 종료 지연 틱 | 4 | 기존 |
| 8 | 후보 창 7×7 | around.rs:100-101 | 7 | 키우면 range 안의 더 많은 셀을 평가 | 4 | 기존 |

<details><summary>`callees` 피호출자 53건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | as_index | game_core::BigGoal::as_index | pub | fn(&game_core::BigGoal) -> usize | game-core\src\simulation\ai_interface.rs:338 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 1 | as_index | game_core::Position::as_index | pub | fn(&game_core::Position) -> usize | game-core\src\simulation\entity.rs:580 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 2 | blocks | game_ai::SafeCrossRoute::blocks | pub | fn(&game_ai::SafeCrossRoute, &game_core::OperationData, usize, usize) -> bool | game-ai\src\path_finder.rs:1788 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 3 | collect_unseen_enemy_estimates | game_ai::collect_unseen_enemy_estimates | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData) -> ([(u64, u64); 5_usize], usize) | game-ai\src\path_finder.rs:1722 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | collect_visible_enemy_positions | game_ai::collect_visible_enemy_positions | pub | fn(&game_core::PlayerState, &game_core::OperationData) -> ([(u64, u64); 5_usize], usize) | game-ai\src\path_finder.rs:1471 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 5 | deadly_band_cell | game_ai::TowerDodgeContext::deadly_band_cell | pub | fn(&game_ai::TowerDodgeContext, usize, usize, usize) -> bool | game-ai\src\path_finder.rs:1256 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 6 | debug_print | game_ai::PathFinder::debug_print | pub | fn(&game_ai::PathFinder) | game-ai\src\path_finder.rs:128 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 7 | distance | game_core::utils::distance | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:12 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 8 | distance_sq | game_core::utils::distance_sq | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:6 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 9 | distance_sq | game_core::Entity::distance_sq | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2157 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 10 | dodge_tower_cell_with_context | game_ai::dodge_tower_cell_with_context | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_ai::TowerDodgeContext, usize, usize, usize, usize) -> bool | game-ai\src\path_finder.rs:1322 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 11 | dodge_unnecessary_tower_cell_with_context | game_ai::dodge_unnecessary_tower_cell_with_context | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_ai::TowerDodgeContext, usize, usize, usize, usize) -> bool | game-ai\src\path_finder.rs:1418 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 12 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 13 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 14 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 15 | get_input | game_ai::PathFinder::get_input | pub | fn(&mut game_ai::PathFinder, &game_core::PlayerState, &game_core::OperationData, game_ai::SafeMoveWithSkill, bool) -> game_core::Input | game-ai\src\path_finder.rs:856 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 16 | get_input | game_ai::SmallActionRunAway::get_input | in:game_ai | fn(&mut game_ai::SmallActionRunAway, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &mut game_core::DebugFrameData) -> std::option::Option<game_core::Input> | game-ai\src\small_action\move_actions.rs:95 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 17 | is_empty | game_core::PatchSetting::is_empty | pub | fn(&game_core::PatchSetting) -> bool | game-core\src\patch_engine.rs:853 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 18 | is_empty | game_core::ModAiRegistry::is_empty | pub | fn(&game_core::ModAiRegistry) -> bool | game-core\src\mod_ai.rs:15 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 19 | is_empty | game_core::PublicChampionEvidence::is_empty | pub | fn(&game_core::PublicChampionEvidence) -> bool | game-core\src\banpick\public_evidence.rs:96 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 20 | is_enemy_danger_cell | game_ai::is_enemy_danger_cell | pub | fn(usize, usize, u64, u64, &[(u64, u64); 5_usize], usize, u64, u32) -> bool | game-ai\src\path_finder.rs:1811 | True | True | 3 | IR 호출 심볼 일치(토큰 경계) |
| 21 | is_enemy_well_danger | game_ai::is_enemy_well_danger | pub | fn(usize, &game_core::PlayerState, u64, u64) -> bool | game-ai\src\path_finder.rs:1032 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 22 | is_recent_visible | game_core::Blackboard::is_recent_visible | pub | fn(&game_core::Blackboard, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\game\blackboard.rs:346 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 23 | is_safe_recall | game_ai::small_action::cast::is_safe_recall | in:game_ai | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData) -> bool | game-ai\src\small_action\cast.rs:304 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 24 | iter_champions | game_core::AbstractGameWithCache::<'a, 'b>::iter_champions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5759 ~ game_core[6a30]::simulation::{impl#5}::iter_champions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1904 | True | True | 3 | IR 호출 심볼 일치(토큰 경계) |
| 25 | lethal_zone_goal | game_ai::PveHazardContext::lethal_zone_goal | pub | fn(&game_ai::PveHazardContext, u64, u64) -> bool | game-ai\src\path_finder.rs:1167 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 26 | new | game_ai::SafeCrossRoute::new | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, u64, u64, u64, u64) -> game_ai::SafeCrossRoute | game-ai\src\path_finder.rs:1774 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 27 | new | game_ai::PveHazardContext::new | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData) -> game_ai::PveHazardContext | game-ai\src\path_finder.rs:1120 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 28 | new_target_with_policy | game_ai::PathFinder::new_target_with_policy | pub | fn(&mut rand::rngs::std::StdRng, &game_core::GameContext, usize, &str, game_ai::path_field::SolvePolicy, u64, u64, u64, u64, impl Fn(usize, usize, usize, usize) -> PathVerdict/#0) -> game_ai::PathFinder | game-ai\src\path_finder.rs:88 | True | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 29 | new_unnecessary_tower_avoid | game_ai::TowerDodgeContext::new_unnecessary_tower_avoid | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, u64, u64) -> game_ai::TowerDodgeContext | game-ai\src\path_finder.rs:1220 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 30 | new_with_skill | game_ai::SmallActionRunAway::new_with_skill | pub | fn(&game_core::OperationData, &game_core::PlayerState, usize, bool) -> game_ai::SmallActionRunAway | game-ai\src\small_action\move_actions.rs:29 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 31 | path_needs_tower_escape | game_ai::small_action::path_needs_tower_escape | in:game_ai::small_action | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_ai::PathFinder) -> bool | game-ai\src\small_action.rs:109 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 32 | positioning_cell_value_at | game_ai::small_action::positioning_cell_value_at | in:game_ai::small_action | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, u64, u64, game_ai::PositionEvalPurpose) -> i64 | game-ai\src\small_action.rs:104 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 33 | positioning_choice_window | game_ai::small_action::positioning_choice_window | in:game_ai::small_action | fn(usize, &game_core::PlayerState, usize, usize, bool) -> (usize, usize) | game-ai\src\small_action.rs:43 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 34 | positioning_max_range | game_core::AthleteParameter::positioning_max_range | pub | fn(&game_core::AthleteParameter) -> usize | game-core\src\simulation\state\player.rs:312 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 35 | positioning_min_range | game_core::AthleteParameter::positioning_min_range | pub | fn(&game_core::AthleteParameter) -> usize | game-core\src\simulation\state\player.rs:306 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 36 | positioning_score_at_cell | game_ai::small_action::positioning_score_at_cell | in:game_ai::small_action | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, i64, i64, game_ai::PositionEvalPurpose) -> game_core::PositioningScore | game-ai\src\small_action.rs:94 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 37 | positioning_score_at_position | game_ai::small_action::positioning_score_at_position | in:game_ai::small_action | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, u64, u64, game_ai::PositionEvalPurpose) -> game_core::PositioningScore | game-ai\src\small_action.rs:99 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 38 | positioning_window_pick | game_ai::small_action::positioning_window_pick | in:game_ai::small_action | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &[T/#0]) -> &T/#0 | game-ai\src\small_action.rs:66 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 39 | push | <game_core::Staff as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::staff | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\staff.rs:38 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 40 | push | <game_core::Athlete as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:434 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 41 | push | <game_core::Contract as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:2498 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 42 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 43 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 44 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 45 | update_path | game_ai::PathFinder::update_path | pub | fn(&mut game_ai::PathFinder, &mut rand::rngs::std::StdRng, &game_core::GameContext, u64, u64, u64, u64, impl Fn(usize, usize, usize, usize) -> PathVerdict/#0) | game-ai\src\path_finder.rs:631 | True | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 46 | v3_deadly_edge_cells | game_ai::v3_deadly_edge_cells | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData) -> u32 | game-ai\src\tower_discipline.rs:304 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 47 | v3_lethal_tower_hp | game_ai::v3_lethal_tower_hp | pub | fn(&game_core::OperationData, &game_core::PlayerState, &game_core::Entity) -> bool | game-ai\src\tower_discipline.rs:272 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 48 | v3_lethal_tower_position | game_ai::v3_lethal_tower_position | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, u64, u64) -> bool | game-ai\src\tower_discipline.rs:282 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 49 | v3_lethal_wave_position | game_ai::v3_lethal_wave_position | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, u64, u64) -> bool | game-ai\src\minion_wave_risk.rs:251 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 50 | verdict | game_ai::PveHazardContext::verdict | pub | fn(&game_ai::PveHazardContext, usize, &game_core::PlayerState, &game_core::OperationData, usize, usize) -> std::option::Option<game_ai::PathVerdict> | game-ai\src\path_finder.rs:1172 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 51 | verdict | game_ai::ZoneHazardContext::verdict | pub | fn(&game_ai::ZoneHazardContext, u64, u64, usize, usize) -> std::option::Option<game_ai::PathVerdict> | game-ai\src\path_finder.rs:1675 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 52 | verdict | game_ai::EnemyHazardContext::verdict | pub | fn(&game_ai::EnemyHazardContext, u64, u64, usize, usize) -> std::option::Option<game_ai::PathVerdict> | game-ai\src\path_finder.rs:1602 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
</details>

⚠**미매칭 14개**: `_RNvNtNtCs9ec1k27omRZ_3std2io5stdio6__print`, `__rust_dealloc`, `clamp`, `collect`, `continue`, `driftsort_main`, `drop  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `insertion_sort_shift_left`, `reserve_internal_or_panic`, `risk`, `skip`, `slice_index_fail`, `sort_by_key`, `take`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 2곳** (m11.ll:43022, m11.ll:43037) · **형제 10개** (SmallActionAround)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::SmallActionAround as std::clone::Clone>::clone | pub | game-ai\src\small_action\around.rs:8 | True | fn(&game_ai::SmallActionAround) -> game_ai::SmallActionAround |
| 1 | <game_ai::SmallActionAround as std::fmt::Debug>::fmt | pub | game-ai\src\small_action\around.rs:8 | True | fn(&game_ai::SmallActionAround, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 2 | game_ai::SmallActionAround::new | pub | game-ai\src\small_action\around.rs:23 | False | fn(usize, &mut rand::rngs::std::StdRng, &game_core::OperationData, &game_core::PlayerState, usize, usize) -> game_ai::SmallActionAround |
| 3 | game_ai::SmallActionAround::new_in_attack | pub | game-ai\src\small_action\around.rs:38 | False | fn(usize, &mut rand::rngs::std::StdRng, &game_core::OperationData, &game_core::PlayerState, usize, usize) -> game_ai::SmallActionAround |
| 4 | game_ai::SmallActionAround::with_position_eval_purpose | pub | game-ai\src\small_action\around.rs:56 | True | fn(game_ai::SmallActionAround, game_ai::PositionEvalPurpose) -> game_ai::SmallActionAround |
| 5 | game_ai::SmallActionAround::get_input | in:game_ai | game-ai\src\small_action\around.rs:61 | False | fn(&mut game_ai::SmallActionAround, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &mut game_core::DebugFrameData) -> std::option::Option<game_core::Input> |
| 6 | game_ai::SmallActionAround::merge | in:game_ai | game-ai\src\small_action\around.rs:266 | False | fn(&mut game_ai::SmallActionAround, usize, game_ai::SmallActionAround) |
| 7 | game_ai::SmallActionAround::get_action | in:game_ai | game-ai\src\small_action\around.rs:284 | True | fn(&game_ai::SmallActionAround) -> game_core::SmallAction |
| 8 | game_ai::SmallActionAround::is_end | in:game_ai | game-ai\src\small_action\around.rs:287 | False | fn(&game_ai::SmallActionAround, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bool |
| 9 | game_ai::SmallActionAround::near_move_complete | in:game_ai | game-ai\src\small_action\around.rs:294 | False | fn(&game_ai::SmallActionAround, &game_core::Entity) -> bool |

**`open` 7건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 표기 불가 | 94 의 소스 표기 — 접힘된 범위검사라 `dist <= 25000000 \|\| dist >= 9216000000` vs `< 25000001 \|\| > 9215999999` 표기 불가(외연 동일). 세 조건(goal_to_target·dist 하한·dist 상한)의 `\|\|` 순서도 column 부재로 표기 불가 | 4 |  |
| 1 | 미탐색 | closure#0(71)·closure#1(72) 은 별도 define 없이 인라인(fnparts: 클로저 5개 중 #3/#5/#6 만 define). 판정식은 본문에서 복원됨(위 logic) | 4 |  |
| 2 | 미탐색 | PveHazardContext::verdict / TowerDodgeContext::deadly_band_cell / SafeCrossRoute::blocks 는 aux 클로저 안에 인라인돼 관측됐지만 path_finder 계층 — 내부 규칙은 명세 범위 밖(관측 요지만 logic 에) | 4 |  |
| 3 | 미탐색 | closure#3/#6 의 dodge_unnecessary_tower_cell_with_context 4번째 인자(ps)·closure#5 의 dodge_tower_cell_with_context 1·3·4번째 인자가 `poison` — 콜리가 그 인자를 안 읽는다는 LLVM 판정 흔적(추정), 콜리 내부 미확인 | 5 |  |
| 4 | 미탐색 | 128 키의 d/1000 항 의미(같은 가치일 때 target 에서 먼 셀 우선) — 소스 의도 불명, IR 동작만 확정. `sdiv` 라 d 는 부호 있는 나눗셈(항상 양수라 무관) | 4 |  |
| 5 | 미탐색 | calls 의 `_RNvNtNtCs9ec1k27omRZ_3std2io5stdio6__print` = std::io::_print (println! 94061). 읽기 좋은 이름으로 못 적은 이유: v0 망글은 `_` 로 시작하는 식별자에 `_` 를 하나 더 붙여 `6__print`(길이 6 = `_print`) 로 내는데 qcspec.mangled_parts 가 그 규칙을 몰라 `__prin` 으로 자르고, `_print` 는 8자 미만이라 부분일치 폴백도 못 탄다 → 망글 원문으로 등록(도구 결함 보고 대상) | 4 |  |
| 6 | 미탐색 | self.start_tick(+0x0)·end_delay(+0x30)·SmallActionAround 의 target 엔티티 종류(타워/오브젝트/미니언)는 이 함수가 안 읽음/안 검사함 — 생성자 소관 | 4 |  |

**`notes` 3건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | get_entity_by_id(92712)·tick(94019) 은 vtable 간접호출이라 calls 에 못 넣음 — divtable AbstractGame 0x1f0/0x28 로 슬롯 이름 확정 | 3 | 사실 서술 |
| 1 | 153 에서 goal_gain 을 새 후보 기준으로 쓰고 154 에서 이전 goal 을 유지할 수 있어 goal_gain 이 goal 좌표와 어긋날 수 있음(Region 의 goal_risk 와 같은 패턴) — 동작만 확정 | 4 | 사실 서술 |
| 2 | SmallActionRunAway::get_input(경로 b) 의 반환·부작용은 그 함수 명세 소관. 8번째 인자에 poison 을 넘기므로 그 함수도 debug 를 안 읽는다는 것만 확정 | 4 | 사실 서술 |

<details><summary>`closed` 1건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

| # | 물음 | 닫은 근거 |
|---|---|---|
| 0 | 70-72 의 blackboard 인덱스가 `1 - team`(적 팀)인 이유 — IR 확정(92814→92882). 소스 의도(적 팀 관점 가시성?)는 소스 없이 판정 불가. game_core Blackboard::is_recent_visible 내부는 범위 밖 | 본문에 해소 표기가 있다 |
</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

