---

### `178` SmallActionAroundPosition::get_input — 특정 좌표(target) 반경 around_radius 안에서 가치(positioning_cell_value_at) 높은 셀을 골라 goal 을 잡고, 타워/적/횡단 위험을 피하는 PathFinder 로 이동 Input 을 만든다

| 항목 | 값 |
|---|---|
| id | `SmallActionAroundPosition__get_input` |
| 심볼 | `_RNvMs3_NtNtCshdEBA0ozCnw_7game_ai12small_action6aroundNtB5_25SmallActionAroundPosition9get_input` |
| 소스 | `game-ai\src\small_action\around.rs:864` |
| IR | `m08.ll` 103480~104548행 |
| 경로·가시성 | `game_ai::SmallActionAroundPosition::get_input` · **pub** |
| 계층 | 기타 |
| exe | `dc0800` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r14` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&mut game_ai::SmallActionAroundPosition, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData) -> std::option::Option<game_core::Input>
```

<details><summary>인자 7개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | *mut Option<game_core::Input> (32B) | None = `store i64 -1` +0 만(103611·104380). Some = PathFinder::get_input 이 sret %0 에 직접 기록(104369 — memcpy 없이 콜리가 32B 를 씀) | 4 |
| 1 | 1 | self | &mut SmallActionAroundPosition (184B) | noalias·dereferenceable(184)·readonly 없음 → &mut. writes 전수는 writes 항목 | 4 |
| 2 | 2 | version | usize | 스택 %30(103516)에 저장돼 클로저에 &version 으로 캡처. 분기: version<2 && DeathMatch(103873-103875) / version>1(103923·104170) / policy.direct_cross = version<2(103663) | 4 |
| 3 | 3 | rnd | &mut StdRng (320B) | 직접 쓰기 0 — positioning_window_pick·new_target_with_policy·update_path·is_safe_recall 에 전달 | 4 |
| 4 | 4 | player | &PlayerState (2528B) | readonly | 4 |
| 5 | 5 | data | &OperationData (24B) | readonly | 4 |
| 6 | 6 | positioning_score | &PositioningScoreData (2760B) | readonly. cx/cy 직접 읽고 나머지는 콜리·클로저 캡처 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn get_input(&mut self, version, rnd, player, data, ps) -> Option<Input>   [around.rs:864~1032]
// 866: champ = data.cache.player_champion[player.team][player.position.as_index()]?   (None → 103611)
// 868: d = distance_sq(champ.pos, (self.goal_x, self.goal_y))
// 870: if d <= 25000000 (5000²) || d >= 921600000000 (960000²)  {      ← 접힘 상수 -921600000000 / -921574999999 (103605-103608). 참 = goal 도달/무효 → 재선택
//   871-872: xi = ps.cx as i32; yi = ps.cy as i32
//   873: pve_hazard_ap = PveHazardContext::new(version, player, data)            (496B)
//   874: now_score = positioning_cell_value_at(version, player, data, ps, champ.x, champ.y, self.position_eval_purpose)
//   876: candidates: bumpalo Vec<(i32 x, i32 y, i64 score)> = new_in(data.context.pool)   (원소 16B)
//   878-901: for dx in 0..7 { for dy in 0..7 {
//       xb=xi-3+dx; yb=yi-3+dy; x=xb*32000+16000; y=yb*32000+16000
//       886: if xb<0||yb<0||xb>29||yb>29 → continue; if map.walls[yb][xb]!=0 → continue
//       890: if distance_sq((x,y), (self.target_x, self.target_y)) >= self.around_radius² → continue     (104438-104460, ult 라 `<` 만 통과)
//       894: if v3_lethal_tower_position(version, player, data, x, y) → continue
//       895: if v3_lethal_wave_position(version, player, data, x, y) → continue
//       896: if pve_hazard_ap.lethal_zone_goal(x, y) → continue
//       900: score = positioning_cell_value_at(version, player, data, ps, x, y, self.position_eval_purpose)
//       901: candidates.push((x, y, score))
//   } }
//   905: candidates.sort_by_key(|c| -c.2)                 (score 내림차순 — 가치 높은 셀이 앞)
//   907: let (gx, gy) = if candidates.is_empty() {
//       908: self.goal_score_diff = 0;  909: (self.target_x, self.target_y)
//   } else {
//       911: candidates.sort_by_key(|c| -c.2)             (같은 키로 한 번 더 — 결과 불변)
//       916: let c = if matches!(data.cache.game.get_game_mode() /*vtable +0x40*/, GameMode::DeathMatch /*tag 2*/) && version < 2 {
//           918: &candidates[0]                           (bounds: 비어있으면 panic — 여기선 비어있지 않음)
//       } else {
//           920-922: (min_range, max_range) = positioning_choice_window(version, player, param.positioning_min_range(), param.positioning_max_range(), false)
//           923: min_idx = min(len-1, len*min_range/1000);  924: max_idx = clamp(len*max_range/1000, min_idx, len-1)
//           926-928: candidates = candidates.into_iter().skip(min_idx).take(max_idx-min_idx+1).collect()
//           929: positioning_window_pick(version, rnd, player, data, &candidates)
//       };  (x, y, score) = (c.0, c.1, c.2)
//       933: if version > 1 && self.goal_x < 960000 && self.goal_y < 960000
//            && 934: distance_sq(self.goal, self.target) < self.around_radius²
//            && 935: !(positioning_cell_value_at(version,player,data,ps,self.goal_x,self.goal_y,purpose) < score)   /*keep_prev = prev >= score*/ {
//           937: self.goal_score_diff = positioning_cell_value_at(…, self.goal_x, self.goal_y, purpose) /*★같은 호출을 한 번 더*/ - now_score;  (self.goal_x, self.goal_y)
//       } else {
//           940: self.goal_score_diff = score - now_score;  941: (x as u64, y as u64)
//       }
//   };
//   945-946: self.goal_x = gx; self.goal_y = gy;   947: drop(candidates)
// }   (거짓이면 goal 그대로, 이하 공통)
// 949: tower_dodge = TowerDodgeContext::new_tower_avoid_v3(version, player, data, goal_x, goal_y)
// 952-953: policy = SolvePolicy { deadly_cells: v3_deadly_edge_cells(version, player, data) /*u32*/, direct_cross: version < 2 }
// 957: (enemy_positions, known_enemy_count) = collect_visible_enemy_positions(player, data)
// 958: (unseen_positions, unseen_count) = collect_unseen_enemy_estimates(version, player, data)
// 959: cross = SafeCrossRoute::new(version, player, data, champ.x, champ.y, goal_x, goal_y)
// 960: is_far_travel = distance_sq(champ.pos, goal) > 102400000000 (320000²)
// 961: if self.path_finder.is_none() {
//   962: self.path_finder = Some(PathFinder::new_target_with_policy(rnd, data.context, version, "dodge_tower_cell", policy, champ.x, champ.y, goal_x, goal_y,
//        closure#2 |sx,sy,nx,ny| -> PathVerdict {                       (aux m03.ll:133791)
//           964: if tower_dodge.deadly_band_cell(version, nx, ny) → Deadly(3)     (인라인: version>=2 && towers[..len] 중 flag && in_range_sq(tower, 셀중심, r) 인 것 존재)
//           967: base = check_cell(version, player, ps, data, &tower_dodge, self.outline_type, sx, sy, nx, ny, clamp(goal_x/32000,0,29), clamp(goal_y/32000,0,29))
//           968: if base != Allow → return base                                     (Priced 페이로드 포함 그대로)
//           971: if is_far_travel && is_enemy_danger_cell(nx,ny, goal, &enemy_positions, known_enemy_count, 6400000000, 1) → Soft(1)
//           974: if is_enemy_danger_cell(nx,ny, goal, &unseen_positions, unseen_count, 6400000000, 1) → Soft(1)
//           977: if cross.blocks(data, nx, ny) → Soft(1) else Allow(0)             (blocks 인라인: active && region∉{champ_region,goal_region} && 진영별 지역 집합 — path_finder 소관)
//        }))
// }
// 985: if self.path_finder.as_ref().is_some_and(|pf| pf.unconstrained_fallback) {
//   986: lethal_tower = version > 1 && v3_lethal_tower_hp(data, player, champ)
//   987: retry = PathFinder::new_target_with_policy(rnd, ctx, version, "cross_only", policy, champ.x, champ.y, self.goal_x, self.goal_y,
//        closure#4 |sx,sy,nx,ny| {                                              (aux m03.ll:134175)
//           989: if tower_dodge.deadly_band_cell(version, nx, ny) → Deadly(3)
//           993: if lethal_tower && !dodge_tower_cell_with_context(version, player, data, ps, &tower_dodge, sx,sy,nx,ny) → Danger(2)
//           996: if cross.blocks(data, nx, ny) → Soft(1) else Allow(0)
//        })
//   998: if !retry.unconstrained_fallback { 999: self.path_finder = Some(retry) } else { 1001: drop(retry) }
// }
// 1003: let p = self.path_finder.as_mut()?          (None → 104380, 실질 도달 불가)
// 1005: p.update_path(rnd, data.context, champ.x, champ.y, self.goal_x, self.goal_y, closure#5 /*closure#2 와 동일, aux 134497*/)
// 1029: safe = if self.goal_score_diff >= 10 (IR: > 9) { SafeMoveWithSkill::Must(2) } else { Safe(1) }
// 1030: recall = is_safe_recall(version, rnd, player, data, ps)
// 1028: return Some(p.get_input(player, data, safe, recall))      (sret 직접 기록)
```

**`mem` 메모리 접근 42건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | 103547 (bounds 2) | 4 | OK |  |
| 1 | PlayerState | 0x9c0 | info.position@tag → as_index | r | 103558-103560 | 4 | OK |  |
| 2 | PlayerState | 0x180 | info.parameter (AthleteParameter) | r | 103879 · positioning_min/max_range | 4 | OK |  |
| 3 | OperationData | 0x0 | cache | r | 103561·103863 | 4 | OK |  |
| 4 | OperationData | 0x8 | context (&GameContext) | r | 103633·104069·104190·104328 | 4 | OK |  |
| 5 | AbstractGameWithCache | 0x1e0 | player_champion[team][pos] | r | 103562-103567 | 4 | OK |  |
| 6 | AbstractGameWithCache | 0x0 | game (&dyn AbstractGame data ptr) | r | 103863 | 4 | OK |  |
| 7 | AbstractGameWithCache | 0x8 | game vtable ptr | r | 103864-103867 · 슬롯 +0x40 = AbstractGame::get_game_mode (divtable) → {i64 tag, ptr} GameMode, tag 2 = DeathMatch | 3 | OK |  |
| 8 | GameContext | 0x0 | pool (&Bump) | r | 103635 | 4 | OK |  |
| 9 | GameContext | 0x20 | map (&MapDef) | r | 103646·104424 | 4 | OK |  |
| 10 | PositioningScoreData | 0xab8 | cx | r | 103618 | 4 | OK |  |
| 11 | PositioningScoreData | 0xac0 | cy | r | 103622 | 4 | OK |  |
| 12 | MapDef | 0x78 | walls[yb][xb] | r | 104425-104430 · 0 이어야 후보 | 4 | OK |  |
| 13 | Entity | 0x660 | x (champ) | r | 103572 | 4 | OK |  |
| 14 | Entity | 0x668 | y (champ) | r | 103578 | 4 | OK |  |
| 15 | SmallActionAroundPosition | 0x8 | goal_x | r | 103584·103924·104192·104330 | 4 | OK |  |
| 16 | SmallActionAroundPosition | 0x10 | goal_y | r | 103588·103926·104193·104331 | 4 | OK |  |
| 17 | SmallActionAroundPosition | 0x18 | goal_score_diff | r | 104364 · > 9 이면 SafeMoveWithSkill::Must | 4 | OK |  |
| 18 | SmallActionAroundPosition | 0x20 | target_x | r | 103647(%88)·103794·104004·104438 — 후보 반경 중심·빈 후보 시 goal | 4 | OK |  |
| 19 | SmallActionAroundPosition | 0x28 | target_y | r | 103648(%89)·103796·104007·104441 | 4 | OK |  |
| 20 | SmallActionAroundPosition | 0x58 | around_radius | r | 103649(%90)·104023·104457 · 제곱해서 distance_sq 와 비교 | 4 | OK |  |
| 21 | SmallActionAroundPosition | 0xad | path_finder@tag (= unconstrained_fallback 바이트, 2=None) | r | 103706-103708 is_none · 104159-104164 is_some_and(closure#3 = \|pf\| pf.unconstrained_fallback) | 4 | OK |  |
| 22 | SmallActionAroundPosition | 0x90 | path_finder.path (Box 1120B) | r | 104109·104223 (gep 144) · 교체 시 dealloc | 4 | OK |  |
| 23 | SmallActionAroundPosition | 0x98 | path_finder.planned_verdict (Box 70B) | r | 104111·104225 (gep 152) | 4 | OK |  |
| 24 | SmallActionAroundPosition | 0xb0 | position_eval_purpose (PositionEvalPurpose 1B, i8 %77) | r | 103628-103630 · positioning_cell_value_at 7번째 인자(4곳) | 4 | OK |  |
| 25 | SmallActionAroundPosition | 0xb1 | outline_type (AroundBushOutlineType: 0 None/1 Outline/2 Inline) | r | 104072·104333 — &outline_type 을 클로저#2/#5 에 캡처(+56) → check_cell 6번째 인자(aux 133972·133987) | 4 | OK |  |
| 26 | PathFinder(retry, 스택 %13) | 0x45 | unconstrained_fallback | r | 104210-104212 (gep 69) | 4 | OK |  |
| 27 | PathFinder(retry) | 0x28 | path (Box) | r | 104273 (gep 40) · retry 폐기 시 dealloc | 4 | OK |  |
| 28 | PathFinder(retry) | 0x30 | planned_verdict (Box) | r | 104275 (gep 48) | 4 | OK |  |
| 29 | 후보 튜플 (i32 x,i32 y,i64 score) 16B | 0x0 | .0 = x | r | 103919 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 30 | 후보 튜플 | 0x4 | .1 = y | r | 103917 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 31 | 후보 튜플 | 0x8 | .2 = score (정렬 키, 부호 반전) | r | 103915 · m03.ll:118275-118293 insertion_sort 인스턴스: `sub 0, 원소+8` 후 slt → closure#0(905)·closure#1(911) 모두 `\|c\| -c.2` = score 내림차순 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 32 | TowerDodgeContext (aux, deadly_band_cell 인라인) | 0x0 | towers.len (≤16, bounds 17) | r | aux 133848·133863 · +0x8 반경 r(133929) · +0x18 부터 stride 24 항목(x@+0,y@+8,flag@+18)(133867-133910) — TowerDodgeContext 내부는 path_finder 소관, 관측만 | 4 | 오귀속(사전은 다른 필드를 준다) |  |
| 33 | SafeCrossRoute (aux, blocks 인라인) | 0x10 | active | r | aux 134027-134030 · +0x0 champ_region(134048) · +0x8 goal_region(134051) · +0x11 my_blue(134064) | 4 | OK |  |
| 34 | SmallActionAroundPosition | 0x8 | goal_x | w | 103838 (around.rs:945) — 재선택 경로(870 조건 참)에서만 | 4 | OK | gx (target_x \| 후보 x \| 이전 goal_x 유지) |
| 35 | SmallActionAroundPosition | 0x10 | goal_y | w | 103839 (around.rs:946) | 4 | OK | gy |
| 36 | SmallActionAroundPosition | 0x18 | goal_score_diff | w | 재선택 경로에서만 갱신 | 4 | OK | 0 (후보 없음, 103793/908) · score - now_score (104035/940) · positioning_cell_value_at(goal) - now_score (104054/937, 이전 goal 유지 시) |
| 37 | SmallActionAroundPosition | 0x68 | path_finder (Option<PathFinder> 72B, +0x68..+0xb0) | w | 104060 memcpy 72B (around.rs:962). 직전 drop_glue(104104-104156)는 tag==2 라 dealloc 미실행 | 4 | OK | Some(PathFinder::new_target_with_policy(rnd,ctx,version,"dodge_tower_cell",policy,champ.x,champ.y,gx,gy,closure#2)) — is_none 이었을 때 |
| 38 | SmallActionAroundPosition | 0x68 | path_finder | w | 104323 (around.rs:999). 기존 Box 2개 dealloc 104222-104270 | 4 | OK | Some(retry) = new_target_with_policy(...,"cross_only",policy,champ.x,champ.y,goal_x,goal_y,closure#4) — 기존 pf.unconstrained_fallback && !retry.unconstrained_fallback |
| 39 | SmallActionAroundPosition | 0x68 | path_finder 내부 (update_path / get_input 의 &mut PathFinder) | w | 104361·104369 | 4 | OK | 콜리 소관 |
| 40 | StdRng(rnd) | 0x0 | (콜리 경유) | w | 직접 store 없음 | 4 | OK | positioning_window_pick·new_target_with_policy·update_path·is_safe_recall |
| 41 | sret Option<Input> | 0x0 | 전체 | w |  | 4 | 확인불가(tcx 사전에 타입 없음) | -1(None, 103611·104380) 또는 PathFinder::get_input 이 직접 기록(104369) |

**`consts` 상수 19건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev | folded_from |
|---|---|---|---|---|---|---|
| 0 | 2 | 866 | 태그 | 팀 bounds(103549) · path_finder None 태그 `icmp eq i8, 2`(103708·104064·104105·104176·104219) · GameMode::DeathMatch 태그 2(103873) · `version < 2`(103663·103874) · SafeMoveWithSkill::Must 태그 i8 2(104366) · aux Danger 태그 2 | 4 |  |
| 1 | -921600000000 | 868 | 계수 | -(960000²). 870 의 범위검사 접힘: `(d - 960000²) ult (25000001 - 960000²)` ⟺ d < 25000001 (≤5000²) \|\| d ≥ 960000² — 챔피언이 goal 에 5000 이내로 도달했거나 goal 이 무효(≥960000 센티널 거리)이면 goal 재선택 | 4 |  |
| 2 | -921574999999 | 870 | 임계 | = 25000001 - 921600000000 (위 접힘의 비교 상수). 소스 수준 임계는 5000²=25000000(이하) 과 960000² | 4 |  |
| 3 | 7 | 878 | 임계 | 후보 창 한 변 7 셀(dx,dy ∈ 0..7) | 4 |  |
| 4 | -3 | 879 | 계수 | 창 반폭 3: xb = cx-3+dx, yb = cy-3+dy | 4 |  |
| 5 | 32000 | 881 | 인덱스 | 셀 크기 — x = xb*32000+16000; aux 967 에서 goal/32000 → 셀 인덱스; aux deadly_band_cell 도 셀 중심 환산 | 4 |  |
| 6 | 16000 | 881 | 계수 | 셀 중심 오프셋 | 4 |  |
| 7 | 29 | 886 | 임계 | 격자 최대 인덱스 — xb>29\|\|yb>29 제외; aux 967 clamp(goal/32000, 0, 29) | 4 |  |
| 8 | 0 | 886 | 태그 | walls==0 만 후보(104429) · goal_score_diff=0(103793) · aux Allow 태그 0 | 4 |  |
| 9 | 1000 | 923 | 계수 | 창 per-mille 분모: min_idx=min(len-1,len*min/1000), max_idx=clamp(len*max/1000,min_idx,len-1) | 4 |  |
| 10 | 1 | 928 | 태그 | take(max_idx-min_idx+1)(103975) · version>1(103923·104170 ugt 1) · SafeMoveWithSkill::Safe i8 1(104366) · aux Soft 태그 1·is_enemy_danger_cell 마지막 인자 1 | 4 |  |
| 11 | 960000 | 933 | 임계 | goal 좌표 유효 상한(30*32000) — goal_x/y < 960000 일 때만 이전 goal 유지 판정 | 4 |  |
| 12 | 102400000000 | 960 | 임계 | 320000² — distance_sq(champ, goal) > 이 값이면 is_far_travel=true (클로저#2/#5 에서 보이는 적 위험 검사를 켬) | 4 |  |
| 13 | 9 | 1029 | 임계 | `goal_score_diff > 9` (icmp sgt 9) = 소스 `>= diff_cut` 에서 diff_cut=10 (dbg_value `diff_cut = i64 10` 103536) → Must(2), 아니면 Safe(1) | 4 | 10 |
| 14 | 16 | 962 | 길이 | PathFinder key 길이 — @anon...87 = "dodge_tower_cell"(16B) | 4 |  |
| 15 | 10 | 987 | 길이 | key 길이 — @anon...41 = "cross_only"(10B); 103536 의 diff_cut 은 dbg 만(본문 리터럴은 9) | 4 |  |
| 16 | 6400000000 | 971 | 미상 | (aux closure#2/#5) 80000² — is_enemy_danger_cell 반경 제곱. 보이는 적(971, is_far_travel 일 때만)·미관측 적(974) 둘 다 이 값·마지막 인자 1 | 4 |  |
| 17 | 3 | 964 | 태그 | (aux) PathVerdict::Deadly 태그 3 — tower_dodge.deadly_band_cell(version,nx,ny) 참이면 반환(133946→134165 `[3, %37]`) | 4 |  |
| 18 | 17 | 964 | 미상 | (aux, deadly_band_cell 인라인) towers.len bounds: `icmp ult %22, 17` → len ≤ 16 아니면 slice_index_fail (TowerDodgeContext 고정 배열 16) | 4 |  |

**`knobs` 조정점 8건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | goal 도달 판정 반경 5000 (25000000=5000²) | around.rs:870 (접힘 -921574999999) | -921574999999 | 반경을 키우면 goal 에 덜 가까워도 다음 셀을 재선택(더 자주 재선택·잦은 방향 변경) | 4 | 기존 |
| 1 | 후보 창 7×7 | around.rs:878-879 | 7 | 키우면 around_radius 안의 더 많은 셀을 평가(비용 증가) | 4 | 기존 |
| 2 | 후보 반경 = self.around_radius (필드) | around.rs:890·934 (+0x58) | 필드 | 생성자가 정하는 값 — 키우면 target 에서 더 먼 셀도 후보 | 4 | 기존 |
| 3 | is_far_travel 임계 320000 (102400000000) | around.rs:960 | 102400000000 | 내리면 더 짧은 이동에서도 보이는 적 회피(Soft)가 켜진다 | 4 | 기존 |
| 4 | Must 이동 전환 diff_cut = 10 | around.rs:1029 (IR 9) | 9 | goal_score_diff ≥ 10 이면 스킬 강제(Must); 올리면 Safe 이동이 늘어난다 | 4 | 기존 |
| 5 | 적 위험 반경 80000 (6400000000) | around.rs:971·974 (aux) | 6400000000 | 올리면 적 주변을 더 넓게 우회 | 4 | 기존 |
| 6 | DeathMatch 레거시 최고점 고정 선택 | around.rs:916-918 | 2 | version<2 && DeathMatch 에서만 창 없이 1등 후보 — version≥2 는 항상 창 선택 | 4 | 기존 |
| 7 | policy.direct_cross = version<2 | around.rs:953 | 2 | version≥2 부터 SolvePolicy.direct_cross=false(경로 계층 의미는 콜리 소관) | 4 | 기존 |

<details><summary>`callees` 피호출자 38건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | as_index | game_core::BigGoal::as_index | pub | fn(&game_core::BigGoal) -> usize | game-core\src\simulation\ai_interface.rs:338 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 1 | as_index | game_core::Position::as_index | pub | fn(&game_core::Position) -> usize | game-core\src\simulation\entity.rs:580 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 2 | blocks | game_ai::SafeCrossRoute::blocks | pub | fn(&game_ai::SafeCrossRoute, &game_core::OperationData, usize, usize) -> bool | game-ai\src\path_finder.rs:1788 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 3 | check_cell | game_ai::small_action::around::check_cell | in:game_ai::small_action::around | fn(usize, &game_core::PlayerState, &game_core::PositioningScoreData, &game_core::OperationData, &game_ai::TowerDodgeContext, game_ai::AroundBushOutlineType, usize, usize, usize, usize, usize, usize) -> game_ai::PathVerdict | game-ai\src\small_action\around.rs:1256 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | collect_unseen_enemy_estimates | game_ai::collect_unseen_enemy_estimates | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData) -> ([(u64, u64); 5_usize], usize) | game-ai\src\path_finder.rs:1722 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 5 | collect_visible_enemy_positions | game_ai::collect_visible_enemy_positions | pub | fn(&game_core::PlayerState, &game_core::OperationData) -> ([(u64, u64); 5_usize], usize) | game-ai\src\path_finder.rs:1471 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 6 | deadly_band_cell | game_ai::TowerDodgeContext::deadly_band_cell | pub | fn(&game_ai::TowerDodgeContext, usize, usize, usize) -> bool | game-ai\src\path_finder.rs:1256 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 7 | distance_sq | game_core::utils::distance_sq | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:6 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 8 | distance_sq | game_core::Entity::distance_sq | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2157 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 9 | dodge_tower_cell_with_context | game_ai::dodge_tower_cell_with_context | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_ai::TowerDodgeContext, usize, usize, usize, usize) -> bool | game-ai\src\path_finder.rs:1322 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 10 | get_game_mode | game_core::AbstractGame::get_game_mode | pub | fn(&Self/#0) -> game_core::GameMode | game-core\src\simulation.rs:78 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 11 | get_game_mode | <game_core::Game as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::Game) -> game_core::GameMode | game-core\src\simulation\game.rs:1707 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 12 | get_game_mode | <game_core::SingleLaneGame as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::SingleLaneGame) -> game_core::GameMode | game-core\src\simulation\game.rs:4248 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 13 | get_input | game_ai::PathFinder::get_input | pub | fn(&mut game_ai::PathFinder, &game_core::PlayerState, &game_core::OperationData, game_ai::SafeMoveWithSkill, bool) -> game_core::Input | game-ai\src\path_finder.rs:856 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 14 | in_range_sq | game_ai::path_finder::in_range_sq | in:game_ai::path_finder | fn(u64, u64, u64, u64, u64) -> bool | game-ai\src\path_finder.rs:1008 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 15 | is_empty | game_core::PatchSetting::is_empty | pub | fn(&game_core::PatchSetting) -> bool | game-core\src\patch_engine.rs:853 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 16 | is_empty | game_core::ModAiRegistry::is_empty | pub | fn(&game_core::ModAiRegistry) -> bool | game-core\src\mod_ai.rs:15 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 17 | is_empty | game_core::PublicChampionEvidence::is_empty | pub | fn(&game_core::PublicChampionEvidence) -> bool | game-core\src\banpick\public_evidence.rs:96 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 18 | is_enemy_danger_cell | game_ai::is_enemy_danger_cell | pub | fn(usize, usize, u64, u64, &[(u64, u64); 5_usize], usize, u64, u32) -> bool | game-ai\src\path_finder.rs:1811 | True | True | 3 | IR 호출 심볼 일치(토큰 경계) |
| 19 | is_safe_recall | game_ai::small_action::cast::is_safe_recall | in:game_ai | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData) -> bool | game-ai\src\small_action\cast.rs:304 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 20 | lethal_zone_goal | game_ai::PveHazardContext::lethal_zone_goal | pub | fn(&game_ai::PveHazardContext, u64, u64) -> bool | game-ai\src\path_finder.rs:1167 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 21 | new | game_ai::SafeCrossRoute::new | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, u64, u64, u64, u64) -> game_ai::SafeCrossRoute | game-ai\src\path_finder.rs:1774 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 22 | new | game_ai::PveHazardContext::new | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData) -> game_ai::PveHazardContext | game-ai\src\path_finder.rs:1120 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 23 | new_target_with_policy | game_ai::PathFinder::new_target_with_policy | pub | fn(&mut rand::rngs::std::StdRng, &game_core::GameContext, usize, &str, game_ai::path_field::SolvePolicy, u64, u64, u64, u64, impl Fn(usize, usize, usize, usize) -> PathVerdict/#0) -> game_ai::PathFinder | game-ai\src\path_finder.rs:88 | True | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 24 | new_tower_avoid_v3 | game_ai::TowerDodgeContext::new_tower_avoid_v3 | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, u64, u64) -> game_ai::TowerDodgeContext | game-ai\src\path_finder.rs:1205 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 25 | positioning_cell_value_at | game_ai::small_action::positioning_cell_value_at | in:game_ai::small_action | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, u64, u64, game_ai::PositionEvalPurpose) -> i64 | game-ai\src\small_action.rs:104 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 26 | positioning_choice_window | game_ai::small_action::positioning_choice_window | in:game_ai::small_action | fn(usize, &game_core::PlayerState, usize, usize, bool) -> (usize, usize) | game-ai\src\small_action.rs:43 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 27 | positioning_max_range | game_core::AthleteParameter::positioning_max_range | pub | fn(&game_core::AthleteParameter) -> usize | game-core\src\simulation\state\player.rs:312 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 28 | positioning_min_range | game_core::AthleteParameter::positioning_min_range | pub | fn(&game_core::AthleteParameter) -> usize | game-core\src\simulation\state\player.rs:306 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 29 | positioning_window_pick | game_ai::small_action::positioning_window_pick | in:game_ai::small_action | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &[T/#0]) -> &T/#0 | game-ai\src\small_action.rs:66 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 30 | push | <game_core::Staff as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::staff | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\staff.rs:38 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 31 | push | <game_core::Athlete as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:434 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 32 | push | <game_core::Contract as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:2498 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 33 | update_path | game_ai::PathFinder::update_path | pub | fn(&mut game_ai::PathFinder, &mut rand::rngs::std::StdRng, &game_core::GameContext, u64, u64, u64, u64, impl Fn(usize, usize, usize, usize) -> PathVerdict/#0) | game-ai\src\path_finder.rs:631 | True | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 34 | v3_deadly_edge_cells | game_ai::v3_deadly_edge_cells | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData) -> u32 | game-ai\src\tower_discipline.rs:304 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 35 | v3_lethal_tower_hp | game_ai::v3_lethal_tower_hp | pub | fn(&game_core::OperationData, &game_core::PlayerState, &game_core::Entity) -> bool | game-ai\src\tower_discipline.rs:272 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 36 | v3_lethal_tower_position | game_ai::v3_lethal_tower_position | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, u64, u64) -> bool | game-ai\src\tower_discipline.rs:282 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 37 | v3_lethal_wave_position | game_ai::v3_lethal_wave_position | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, u64, u64) -> bool | game-ai\src\minion_wave_risk.rs:251 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
</details>

⚠**미매칭 13개**: `__rust_dealloc`, `base`, `clamp`, `collect`, `continue`, `driftsort_main`, `drop  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `insertion_sort_shift_left`, `reserve_internal_or_panic`, `skip`, `slice_index_fail`, `sort_by_key`, `take`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m11.ll:43047) · **형제 12개** (SmallActionAroundPosition)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::SmallActionAroundPosition as std::clone::Clone>::clone | pub | game-ai\src\small_action\around.rs:814 | True | fn(&game_ai::SmallActionAroundPosition) -> game_ai::SmallActionAroundPosition |
| 1 | <game_ai::SmallActionAroundPosition as std::fmt::Debug>::fmt | pub | game-ai\src\small_action\around.rs:814 | True | fn(&game_ai::SmallActionAroundPosition, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 2 | game_ai::SmallActionAroundPosition::new | pub | game-ai\src\small_action\around.rs:831 | False | fn(&mut rand::rngs::std::StdRng, &game_core::OperationData, u64, u64, usize) -> game_ai::SmallActionAroundPosition |
| 3 | game_ai::SmallActionAroundPosition::new_with_out_line | pub | game-ai\src\small_action\around.rs:834 | False | fn(&mut rand::rngs::std::StdRng, &game_core::OperationData, u64, u64, usize, game_ai::AroundBushOutlineType) -> game_ai::SmallActionAroundPosition |
| 4 | game_ai::SmallActionAroundPosition::new_with_radius | pub | game-ai\src\small_action\around.rs:837 | False | fn(&mut rand::rngs::std::StdRng, &game_core::OperationData, u64, u64, usize, u64) -> game_ai::SmallActionAroundPosition |
| 5 | game_ai::SmallActionAroundPosition::new_with_out_line_and_radius | in:game_ai | game-ai\src\small_action\around.rs:840 | False | fn(&mut rand::rngs::std::StdRng, &game_core::OperationData, u64, u64, usize, game_ai::AroundBushOutlineType, u64) -> game_ai::SmallActionAroundPosition |
| 6 | game_ai::SmallActionAroundPosition::with_position_eval_purpose | pub | game-ai\src\small_action\around.rs:857 | True | fn(game_ai::SmallActionAroundPosition, game_ai::PositionEvalPurpose) -> game_ai::SmallActionAroundPosition |
| 7 | game_ai::SmallActionAroundPosition::get_input | pub | game-ai\src\small_action\around.rs:864 | False | fn(&mut game_ai::SmallActionAroundPosition, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData) -> std::option::Option<game_core::Input> |
| 8 | game_ai::SmallActionAroundPosition::get_action | in:game_ai | game-ai\src\small_action\around.rs:1034 | True | fn(&game_ai::SmallActionAroundPosition) -> game_core::SmallAction |
| 9 | game_ai::SmallActionAroundPosition::is_end | in:game_ai | game-ai\src\small_action\around.rs:1037 | False | fn(&game_ai::SmallActionAroundPosition, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bool |
| 10 | game_ai::SmallActionAroundPosition::merge | in:game_ai | game-ai\src\small_action\around.rs:1042 | False | fn(&mut game_ai::SmallActionAroundPosition, usize, game_ai::SmallActionAroundPosition) |
| 11 | game_ai::SmallActionAroundPosition::near_move_complete | in:game_ai | game-ai\src\small_action\around.rs:1060 | False | fn(&game_ai::SmallActionAroundPosition, &game_core::Entity) -> bool |

**`open` 8건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 표기 불가 | 870 의 소스 표기 — IR 은 범위검사 접힘(`d - 960000² ult 25000001 - 960000²`)만 남아 `d <= 25000000 \|\| d >= 921600000000` 인지 `d < 25000001 \|\| d > 921599999999` 인지 표기 불가(외연 동일). 960000² 항이 goal 무효 센티널을 뜻하는지도 소스 없이는 추정 | 4 |  |
| 1 | 미탐색 | get_game_mode 는 vtable 간접호출(103868)이라 calls 에 못 넣음 — divtable AbstractGame 0x40 = get_game_mode 로 확정, 실제 구현체는 런타임 소관 | 3 |  |
| 2 | 미탐색 | 935·937 에서 positioning_cell_value_at(goal) 을 두 번 호출 — 소스가 두 번 쓴 것인지(중복 계산) 확정. 캐시 유무는 콜리 소관 | 4 |  |
| 3 | 미탐색 | check_cell(around.rs:1256) 내부 — 같은 r14 중간 콜리(시그니처: (version,&PlayerState,&PositioningScoreData,&OperationData,&TowerDodgeContext,AroundBushOutlineType,sx,sy,nx,ny,gcx,gcy) -> PathVerdict). 여기서는 계약만 | 4 |  |
| 4 | 미탐색 | deadly_band_cell / SafeCrossRoute::blocks 는 aux 클로저 안에 인라인돼 관측됐지만 path_finder 계층이라 내부 규칙(지역 id 집합 {2,6,7}·진영별 집합 등)은 명세 범위 밖 — 관측 사실만 logic 에 한 줄 | 4 |  |
| 5 | 미탐색 | closure#4(987) 인라인 사이트(134485)에서 dodge_tower_cell_with_context 의 version/data/ps 인자가 `poison` — 콜리가 그 인자를 안 읽는다는 LLVM 판정 흔적(추정), 콜리 내부 미확인 | 5 |  |
| 6 | 미탐색 | SmallActionAroundPosition.around_input(+0x30..+0x58)·start_tick·end_delay 는 이 함수가 읽지 않음(다른 메서드 소관) | 4 |  |
| 7 | 미탐색 | closure#0(905)·closure#1(911) 의 키가 같은 `-score` 인 이유(두 번 정렬) — 소스 의도 불명, 동작은 멱등 | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

