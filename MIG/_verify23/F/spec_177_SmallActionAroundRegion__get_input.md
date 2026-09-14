---

### `177` SmallActionAroundRegion::get_input — 지역(target_region) 배회: 7×7 셀 후보를 위험도로 정렬→창(window)에서 하나 골라 goal 을 갱신하고 PathFinder 로 이동 Input 을 만든다

| 항목 | 값 |
|---|---|
| id | `SmallActionAroundRegion__get_input` |
| 심볼 | `_RNvMs0_NtNtCshdEBA0ozCnw_7game_ai12small_action6aroundNtB5_23SmallActionAroundRegion9get_input` |
| 소스 | `game-ai\src\small_action\around.rs:529` |
| IR | `m08.ll` 98610~99536행 |
| 경로·가시성 | `game_ai::SmallActionAroundRegion::get_input` · **in:game_ai** |
| 계층 | 기타 |
| exe | `dbbd60` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r14` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&mut game_ai::SmallActionAroundRegion, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData) -> std::option::Option<game_core::Input>
```

<details><summary>인자 7개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | *mut Option<game_core::Input> (32B) | dead_on_unwind writeonly. None = `store i64 -1` 을 +0 에만(98729·99338 — tcx tg: Option<Input> Niche, None 태그 = 0xFFFFFFFFFFFFFFFF, +8..+32 는 미기록). Some = PathFinder::get_input 결과 32B memcpy(99373) | 3 |
| 1 | 1 | self | &mut SmallActionAroundRegion (120B) | IR 속성 noalias·align 8·dereferenceable(120), readonly 없음 → &mut. writes 전수는 writes 항목 | 4 |
| 2 | 2 | version | usize | AI 버전. 스택 슬롯 %32(98648)에 저장돼 클로저 캡처(&version)로 전달. 본문 분기: `version > 1`(99146) 일 때만 v3_lethal_tower_hp 평가 | 4 |
| 3 | 3 | rnd | &mut StdRng (320B) | readonly 없음(&mut). 본문 직접 쓰기 0 — positioning_window_pick·PathFinder::new_target·update_path·is_safe_recall 에 그대로 전달(콜리가 소비) | 4 |
| 4 | 4 | player | &PlayerState (2528B) | readonly | 4 |
| 5 | 5 | data | &OperationData (24B) | readonly. cache(+0)·context(+8) 를 통해 map/bump/player_champion 접근 | 4 |
| 6 | 6 | positioning_score | &PositioningScoreData (2760B) | readonly. cx(+0xab8)/cy(+0xac0) 만 직접 읽고 나머지는 콜리에 전달 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn get_input(&mut self, version, rnd, player, data, ps) -> Option<Input>   [around.rs:529~647]
// 530: champ = data.cache.player_champion[player.info.team][player.info.position.as_index()]?   (None → return None: 98729)
// 532-533: xi = ps.cx as i32; yi = ps.cy as i32
// 535: candidates: bumpalo Vec<(i32 dx,i32 dy,i32 x,i32 y,i64 score)> = Vec::new_in(data.context.pool)   (원소 24B)
// 537-557: for dx in 0..7 { for dy in 0..7 {
//     xb = xi-3+dx; yb = yi-3+dy; x = xb*32000+16000; y = yb*32000+16000            (98761-98763·99418-99421)
//     545: if xb<0 || yb<0 || xb>29 || yb>29 → continue;  if map.walls[yb][xb] != 0 → continue   (99423-99438)
//     549: if map.regions[yb][xb] != self.target_region → continue                         (99441-99447)
//     553: sd = positioning_score_at_position(version, player, data, ps, x, y, Positioning(6))
//     554-555: score = positioning_risk_value(version, player, sd.risk, sd.on_trajectory || sd.on_periodic_trajectory)
//     556: candidates.push((dx, dy, x, y, score))
// } }
// 560: let (gx, gy) = if candidates.is_empty() {
//     561: map.region_centers[self.target_region]            (bounds 27 · 98773-98833)
// } else {
//     563: candidates.sort_by_key(|c| c.4)                  (score 오름차순 = 위험 낮은 순 · m03.ll:117866 icmp slt 원소+16)
//     564-566: (min_range, max_range) = positioning_choice_window(version, player, param.positioning_min_range(), param.positioning_max_range(), false)
//     567: min_idx = min(len-1, len*min_range/1000)
//     568: max_idx = clamp(len*max_range/1000, min_idx, len-1)
//     570-572: candidates = candidates.into_iter().skip(min_idx).take(max_idx-min_idx+1).collect()   (from_iter_in 98909)
//     574: pick = positioning_window_pick(version, rnd, player, data, &candidates)   → &(dx,dy,x,y,score); (x,y) = (pick.2, pick.3)
//     576: pre_score = positioning_score_at_position(version, player, data, ps, self.goal_x, self.goal_y, Positioning).risk
//     578: self.goal_risk = positioning_score_at_position(version, player, data, ps, x, y, Positioning).risk    (★새 후보 기준으로 기록)
//     579: if pre_score >= self.goal_risk && self.goal_x < 960000 && self.goal_y < 960000 && distance_sq(champ.pos, self.goal) > 143999999 (≥12000)
//            { (self.goal_x, self.goal_y) /*이전 goal 유지*/ } else { (x as u64, y as u64) }        (98956-98991)
// };
// 586-587: self.goal_x = gx; self.goal_y = gy
// 590: (enemy_positions, visible_enemy_count) = collect_visible_enemy_positions(player, data)      ([(u64,u64);5], usize → 88B sret, 배열 80B + count)
// 591: (unseen_positions, unseen_count) = collect_unseen_enemy_estimates(version, player, data)
// 592: cross = SafeCrossRoute::new(version, player, data, champ.x, champ.y, gx, gy)
// 593: tower_dodge = TowerDodgeContext::new_tower_avoid_v3(version, player, data, gx, gy)        (416B)
// 595: if self.path_finder.is_none() {                       (tag 바이트 +0x75 == 2)
//     596: self.path_finder = Some(PathFinder::new_target(rnd, data.context, version, "dodge_danger_cell", champ.x, champ.y, gx, gy,
//              closure#1 |sx,sy,nx,ny| -> PathVerdict {                                              (aux m00.ll:95979)
//                 597: if !dodge_tower_cell_with_context(version, player, data, ps, &tower_dodge, sx,sy,nx,ny) → Danger(2)
//                 601: if is_enemy_danger_cell(nx,ny, self.goal_x, self.goal_y, &enemy_positions, visible_enemy_count, 4096000000 /*64000²*/, 2) → Soft(1)
//                 604: if is_enemy_danger_cell(nx,ny, goal, &unseen_positions, unseen_count, 6400000000 /*80000²*/, 1) → Soft(1)
//                 607: if cross.blocks(data, nx, ny) → Soft(1) else Allow(0)                          (zext i1 → 0/1)
//              }))
// }
// 614: if self.path_finder.as_ref().is_some_and(|pf| pf.unconstrained_fallback) {      (closure#2 인라인 — +0x75 바이트 그대로 bool)
//     615: lethal_tower = version > 1 && v3_lethal_tower_hp(data, player, champ)           (version<=1 이면 false, 호출 안 함)
//     616: retry = PathFinder::new_target(rnd, data.context, version, "cross_only", champ.x, champ.y, self.goal_x, self.goal_y,
//              closure#3 |sx,sy,nx,ny| {                       (별도 define 없음 — m03.ll:3634~7402 new_target<closure#3> 인스턴스에 인라인, 7242-7376)
//                 618: if lethal_tower && !dodge_tower_cell_with_context(version, player, data, ps, &tower_dodge, sx,sy,nx,ny) → Danger(2)
//                 621: if cross.blocks(data, nx, ny) → Soft(1) else Allow(0)
//              })
//     623: if !retry.unconstrained_fallback { 624: self.path_finder = Some(retry) /*기존 Box 2개 dealloc*/ } else { 626: drop(retry) }
// }
// 628: let p = self.path_finder.as_mut()?          (여기서 None 이면 return None — 99338; 실제로는 도달 불가)
// 630: p.update_path(rnd, data.context, champ.x, champ.y, self.goal_x, self.goal_y, closure#4 /*closure#1 과 동일 본체, aux m00.ll:95909, 631-640*/)
// 646: return Some(p.get_input(player, data, SafeMoveWithSkill::Safe(1), is_safe_recall(version, rnd, player, data, ps)))   (32B memcpy → sret)
// 647: drop(candidates)
```

**`mem` 메모리 접근 38건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | 98674 · [2] 인덱스 — bounds 2 (98676) | 4 | OK |  |
| 1 | PlayerState | 0x9c0 | info.position@tag (Position::as_index) | r | 98685-98687 · i32 → usize | 4 | OK |  |
| 2 | PlayerState | 0x180 | info.parameter (AthleteParameter) | r | 98851 · positioning_min_range/max_range 의 self | 4 | OK |  |
| 3 | OperationData | 0x0 | cache (&AbstractGameWithCache) | r | 98688 | 4 | OK |  |
| 4 | OperationData | 0x8 | context (&GameContext) | r | 98708 | 4 | OK |  |
| 5 | AbstractGameWithCache | 0x1e0 | player_champion[team][pos] (Option<&Entity>, [2][5], null=None) | r | 98689-98694 | 4 | OK |  |
| 6 | GameContext | 0x0 | pool (&Bump) | r | 98710 · candidates Vec 의 할당자 | 4 | OK |  |
| 7 | GameContext | 0x20 | map (&MapDef) | r | 98721·98825·99432 · PathFinder 에는 &GameContext(%55) 자체를 넘김 | 4 | OK |  |
| 8 | PositioningScoreData | 0xab8 | cx | r | 98699 · i32 로 trunc → 7×7 창 중심 셀 x | 4 | OK |  |
| 9 | PositioningScoreData | 0xac0 | cy | r | 98703 | 4 | OK |  |
| 10 | MapDef | 0x78 | walls[yb][xb] (usize [30][30]) | r | 99433-99436 · 0 이어야 후보 | 4 | OK |  |
| 11 | MapDef | 0x38b8 | regions[yb][xb] (usize [30][30]) | r | 99441-99446 · == self.target_region 이어야 후보 | 4 | OK |  |
| 12 | MapDef | 0x6ba0 | region_centers[region] ((u64,u64)[27], stride 16) | r | 98827-98832 · 후보 0개일 때 goal (map_def.rs:262 region_center 인라인, bounds 27) | 4 | OK |  |
| 13 | SmallActionAroundRegion | 0x8 | target_region | r | 98771(region_center 인덱스)·99445(regions 비교) | 4 | OK |  |
| 14 | SmallActionAroundRegion | 0x10 | goal_x | r | 98929·99164·99308 — 이전 goal(히스테리시스·경로 재계산 입력) | 4 | OK |  |
| 15 | SmallActionAroundRegion | 0x18 | goal_y | r | 98931·99165·99309 | 4 | OK |  |
| 16 | SmallActionAroundRegion | 0x75 | path_finder@tag (== path_finder.unconstrained_fallback 바이트, 2=None) | r | 99031-99033 is_none · 99135-99140 is_some_and(\|pf\| pf.unconstrained_fallback) — Some 이면 이 바이트가 그대로 bool | 4 | OK |  |
| 17 | SmallActionAroundRegion | 0x58 | path_finder.path (Box<[(u64,u64);70]>, 1120B) | r | 99085·99203 · 교체 시 drop_glue 가 dealloc | 4 | OK |  |
| 18 | SmallActionAroundRegion | 0x60 | path_finder.planned_verdict (Box<[u8;70]>) | r | 99087·99205 | 4 | OK |  |
| 19 | Entity | 0x660 | x (champ) | r | 98964·99015 | 4 | OK |  |
| 20 | Entity | 0x668 | y (champ) | r | 98968·99017 | 4 | OK |  |
| 21 | PositioningScore | 0x0 | risk | r | 98936(pre_score)·98952(goal_risk)·99456(후보 점수) — positioning_score_at_position 의 sret .0 | 4 | OK |  |
| 22 | PositioningScore | 0x30 | on_trajectory | r | 99457 (%64 = score_data+48) | 4 | OK |  |
| 23 | PositioningScore | 0x31 | on_periodic_trajectory | r | 99459 (%65 = +49) · `on_trajectory \|\| on_periodic_trajectory` 가 positioning_risk_value 4번째 인자 | 4 | OK |  |
| 24 | PathFinder(retry, 스택 %13) | 0x45 | unconstrained_fallback | r | 99190-99192 (gep 69) | 4 | OK |  |
| 25 | PathFinder(retry) | 0x28 | path (Box) | r | 99253 (gep 40) · retry 폐기 시 dealloc | 4 | OK |  |
| 26 | PathFinder(retry) | 0x30 | planned_verdict (Box) | r | 99255 (gep 48) | 4 | OK |  |
| 27 | 후보 튜플 (i32 dx,i32 dy,i32 x,i32 y,i64 score) 24B | 0x8 | .2 = x (셀 중심 좌표) | r | 98940 — positioning_window_pick 반환 &T 의 +8 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 28 | 후보 튜플 | 0xc | .3 = y | r | 98944 (+12) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 29 | 후보 튜플 | 0x10 | .4 = score (정렬 키) | r | aux 아님·m03.ll:117858-117866 insertion_sort 인스턴스에서 확인: 원소+16 i64 를 `icmp slt` 오름차순 — closure#0(563) = `\|c\| c.4` 상당 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 30 | SmallActionAroundRegion | 0x10 | goal_x | w | 98842 (around.rs:586) | 4 | OK | gx (region_center.0 \| 창에서 고른 후보 x \| 이전 goal_x 유지) |
| 31 | SmallActionAroundRegion | 0x18 | goal_y | w | 98844 (around.rs:587) | 4 | OK | gy |
| 32 | SmallActionAroundRegion | 0x20 | goal_risk | w | 98954 (around.rs:578). 후보 0개(region_center) 경로에서는 기록 안 됨 | 4 | OK | positioning_score_at_position(version,player,data,ps,gx_new,gy_new,Positioning).risk (새 후보 기준 — 이전 goal 을 유지해도 새 후보의 risk 가 기록됨) |
| 33 | SmallActionAroundRegion | 0x30 | path_finder (Option<PathFinder> 72B, +0x30..+0x78) | w | 99037 memcpy 72B (around.rs:596). 직전 drop_glue(99080-99132)는 tag==2 라 dealloc 미실행 | 4 | OK | Some(PathFinder::new_target(rnd,ctx,version,"dodge_danger_cell",champ.x,champ.y,gx,gy,closure#1)) — is_none 이었을 때 |
| 34 | SmallActionAroundRegion | 0x30 | path_finder | w | 99303 memcpy 72B (around.rs:624). 기존 Some 의 path(1120B)·planned_verdict(70B) Box 는 99227·99249 에서 __rust_dealloc | 4 | OK | Some(retry) = PathFinder::new_target(rnd,ctx,version,"cross_only",champ.x,champ.y,goal_x,goal_y,closure#3) — 기존 pf.unconstrained_fallback 이고 retry.unconstrained_fallback 이 false 일 때 |
| 35 | SmallActionAroundRegion | 0x30 | path_finder 내부 (PathFinder::update_path 경유) | w | 99334 (around.rs:630) · 99369 get_input 도 &mut self(PathFinder) — 내부 쓰기는 콜리 명세 소관 | 4 | OK | 콜리가 &mut PathFinder 로 갱신(path/index/verdict 등) |
| 36 | StdRng(rnd) | 0x0 | (콜리 경유) | w | 본문 직접 store 없음 | 4 | OK | positioning_window_pick·new_target·update_path·is_safe_recall 이 소비 |
| 37 | sret Option<Input> | 0x0 | tag / 전체 | w |  | 4 | 확인불가(tcx 사전에 타입 없음) | -1 (None, 98729·99338) 또는 PathFinder::get_input 결과 32B memcpy (99373) |

**`consts` 상수 18건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 2 | 530 | 태그 | player_champion 1차원 bounds(팀 수 2) — 98676; 같은 리터럴 2 가 path_finder None 태그(99033·99041·99152·99199 `icmp eq i8 %, 2`)·`is_enemy_danger_cell(...,2)` 4번째 마지막 인자(aux 95954)·closure 반환 PathVerdict::Danger 태그(aux 95958) | 4 |
| 1 | 7 | 537 | 임계 | 후보 창 한 변 = 7 셀 (dx,dy ∈ 0..7 → 중심 ±3) | 4 |
| 2 | -3 | 538 | 계수 | xb = cx-3+dx, yb = cy-3+dy — 창 반폭 3 셀 | 4 |
| 3 | 32000 | 540 | 계수 | 셀 크기(좌표 변환) x = xb*32000+16000 | 4 |
| 4 | 16000 | 540 | 계수 | 셀 중심 오프셋 | 4 |
| 5 | 29 | 545 | 임계 | 격자 최대 인덱스(30×30) — xb>29 \|\| yb>29 → 후보 제외 | 4 |
| 6 | 0 | 545 | 임계 | walls[yb][xb] == 0 이어야 후보(벽 아님) | 4 |
| 7 | 27 | 561 | 임계 | region_centers 배열 길이(bounds) — target_region ≥ 27 이면 panic | 4 |
| 8 | 1000 | 567 | 인덱스 | 창 인덱스 per-mille: min_idx = min(len-1, len*min_range/1000), max_idx = clamp(len*max_range/1000, min_idx, len-1) | 4 |
| 9 | 1 | 572 | 태그 | take(max_idx-min_idx+1) 의 +1(98900) · `version > 1`(99146 icmp ugt 1) · SafeMoveWithSkill::Safe 태그 i8 1 (99369) · aux: Soft 태그 1·is_enemy_danger_cell 마지막 인자 1 | 4 |
| 10 | 6 | 553 | 센티널 | PositionEvalPurpose 메모리태그 6 = Positioning (tcxdict --enum: 니치, idx 4) — positioning_score_at_position 7번째 인자(98932·98948·99452) | 3 |
| 11 | 960000 | 579 | 임계 | = 30*32000 — goal 좌표 유효 상한(≥960000 이면 미설정 센티널로 보고 히스테리시스 불가) | 4 |
| 12 | 143999999 | 579 | 임계 | 12000²-1 — distance_sq(champ, 이전 goal) > 143999999 (즉 ≥ 12000) 이고 pre_score ≥ new_risk 이면 이전 goal 유지 | 4 |
| 13 | -1 | 530 | 센티널 | Option<Input>::None 니치 태그(98729·99338). 98871·99396 의 -1 은 len-1 | 4 |
| 14 | 17 | 596 | 길이 | PathFinder key &str 길이 — @anon...72 = "dodge_danger_cell"(17B) | 4 |
| 15 | 10 | 616 | 길이 | PathFinder key 길이 — @anon...41 = "cross_only"(10B) | 4 |
| 16 | 4096000000 | 601 | 미상 | (aux closure#1/#4) 64000² — 보이는 적 위치 대비 위험 셀 반경 제곱. is_enemy_danger_cell(nx,ny,goal,enemy_positions,visible_count, 4096000000, 2) | 4 |
| 17 | 6400000000 | 604 | 미상 | (aux) 80000² — 미관측 적 추정 위치 대비 위험 반경 제곱. is_enemy_danger_cell(..., unseen_positions, unseen_count, 6400000000, 1) | 5 |

**`knobs` 조정점 7건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 후보 창 크기(7×7, 반폭 3) | around.rs:537-538 (7 · -3) | 7 | 키우면 더 먼 셀까지 배회 후보로 본다(positioning_score 호출 수 = 창 셀 수만큼 증가) | 4 | 기존 |
| 1 | 이전 goal 유지 거리 임계 12000(143999999=12000²-1) | around.rs:579 | 143999999 | 올리면 goal 근처(12000 이내)까지 와야 새 후보로 바꾼다 → 더 자주 재선택; 내리면 goal 에 거의 닿기 전엔 유지 | 4 | 기존 |
| 2 | 창 위치 per-mille 분모 | around.rs:567-568 | 1000 | positioning_min/max_range(‰)를 후보 인덱스로 바꾸는 분모 — 값 자체보다 AthleteParameter 의 두 값이 실제 노브 | 4 | 기존 |
| 3 | 보이는 적 위험 반경 64000 (4096000000) | around.rs:601 (aux closure#1) / 634 (closure#4) | 4096000000 | 올리면 보이는 적에서 더 멀리 떨어진 셀도 Soft(우회 비용) 처리 → 더 우회 | 4 | 기존 |
| 4 | 미관측 적 추정 위험 반경 80000 (6400000000) | around.rs:604 / 637 | 6400000000 | 올리면 추정 적 위치 주변을 더 넓게 피한다 | 5 | 기존 |
| 5 | PositionEvalPurpose = Positioning(6) 고정 | around.rs:553·576·578 | 6 | 다른 목적(Around=5 등)으로 바꾸면 positioning_score_at_position 의 가중이 바뀐다(콜리 내부) | 4 | 기존 |
| 6 | v3_lethal_tower_hp 게이트 version>1 | around.rs:615 | 1 | version 1 이하에서는 retry 클로저가 타워 위험을 전혀 안 본다 | 4 | 기존 |

<details><summary>`callees` 피호출자 28건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | as_index | game_core::BigGoal::as_index | pub | fn(&game_core::BigGoal) -> usize | game-core\src\simulation\ai_interface.rs:338 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
| 1 | as_index | game_core::Position::as_index | pub | fn(&game_core::Position) -> usize | game-core\src\simulation\entity.rs:580 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
| 2 | blocks | game_ai::SafeCrossRoute::blocks | pub | fn(&game_ai::SafeCrossRoute, &game_core::OperationData, usize, usize) -> bool | game-ai\src\path_finder.rs:1788 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 3 | collect_unseen_enemy_estimates | game_ai::collect_unseen_enemy_estimates | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData) -> ([(u64, u64); 5_usize], usize) | game-ai\src\path_finder.rs:1722 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | collect_visible_enemy_positions | game_ai::collect_visible_enemy_positions | pub | fn(&game_core::PlayerState, &game_core::OperationData) -> ([(u64, u64); 5_usize], usize) | game-ai\src\path_finder.rs:1471 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 5 | distance_sq | game_core::utils::distance_sq | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:6 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
| 6 | distance_sq | game_core::Entity::distance_sq | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2157 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
| 7 | dodge_tower_cell_with_context | game_ai::dodge_tower_cell_with_context | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_ai::TowerDodgeContext, usize, usize, usize, usize) -> bool | game-ai\src\path_finder.rs:1322 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 8 | get_input | game_ai::PathFinder::get_input | pub | fn(&mut game_ai::PathFinder, &game_core::PlayerState, &game_core::OperationData, game_ai::SafeMoveWithSkill, bool) -> game_core::Input | game-ai\src\path_finder.rs:856 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 9 | is_empty | game_core::PatchSetting::is_empty | pub | fn(&game_core::PatchSetting) -> bool | game-core\src\patch_engine.rs:853 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 10 | is_empty | game_core::ModAiRegistry::is_empty | pub | fn(&game_core::ModAiRegistry) -> bool | game-core\src\mod_ai.rs:15 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 11 | is_empty | game_core::PublicChampionEvidence::is_empty | pub | fn(&game_core::PublicChampionEvidence) -> bool | game-core\src\banpick\public_evidence.rs:96 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 12 | is_enemy_danger_cell | game_ai::is_enemy_danger_cell | pub | fn(usize, usize, u64, u64, &[(u64, u64); 5_usize], usize, u64, u32) -> bool | game-ai\src\path_finder.rs:1811 | True | True | 3 | IR 호출 심볼 일치(토큰 경계) |
| 13 | is_safe_recall | game_ai::small_action::cast::is_safe_recall | in:game_ai | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData) -> bool | game-ai\src\small_action\cast.rs:304 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 14 | new | game_ai::SafeCrossRoute::new | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, u64, u64, u64, u64) -> game_ai::SafeCrossRoute | game-ai\src\path_finder.rs:1774 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 15 | new_target | game_ai::PathFinder::new_target | pub | fn(&mut rand::rngs::std::StdRng, &game_core::GameContext, usize, &str, u64, u64, u64, u64, impl Fn(usize, usize, usize, usize) -> PathVerdict/#0) -> game_ai::PathFinder | game-ai\src\path_finder.rs:66 | True | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 16 | new_tower_avoid_v3 | game_ai::TowerDodgeContext::new_tower_avoid_v3 | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, u64, u64) -> game_ai::TowerDodgeContext | game-ai\src\path_finder.rs:1205 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 17 | positioning_choice_window | game_ai::small_action::positioning_choice_window | in:game_ai::small_action | fn(usize, &game_core::PlayerState, usize, usize, bool) -> (usize, usize) | game-ai\src\small_action.rs:43 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 18 | positioning_max_range | game_core::AthleteParameter::positioning_max_range | pub | fn(&game_core::AthleteParameter) -> usize | game-core\src\simulation\state\player.rs:312 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 19 | positioning_min_range | game_core::AthleteParameter::positioning_min_range | pub | fn(&game_core::AthleteParameter) -> usize | game-core\src\simulation\state\player.rs:306 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 20 | positioning_risk_value | game_ai::small_action::positioning_risk_value | in:game_ai::small_action | fn(usize, &game_core::PlayerState, i64, bool) -> i64 | game-ai\src\small_action.rs:79 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 21 | positioning_score_at_position | game_ai::small_action::positioning_score_at_position | in:game_ai::small_action | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, u64, u64, game_ai::PositionEvalPurpose) -> game_core::PositioningScore | game-ai\src\small_action.rs:99 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 22 | positioning_window_pick | game_ai::small_action::positioning_window_pick | in:game_ai::small_action | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &[T/#0]) -> &T/#0 | game-ai\src\small_action.rs:66 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 23 | push | <game_core::Staff as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::staff | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\staff.rs:38 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 6개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 24 | push | <game_core::Athlete as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:434 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 6개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 25 | push | <game_core::Contract as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:2498 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 6개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 26 | update_path | game_ai::PathFinder::update_path | pub | fn(&mut game_ai::PathFinder, &mut rand::rngs::std::StdRng, &game_core::GameContext, u64, u64, u64, u64, impl Fn(usize, usize, usize, usize) -> PathVerdict/#0) | game-ai\src\path_finder.rs:631 | True | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 27 | v3_lethal_tower_hp | game_ai::v3_lethal_tower_hp | pub | fn(&game_core::OperationData, &game_core::PlayerState, &game_core::Entity) -> bool | game-ai\src\tower_discipline.rs:272 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
</details>

⚠**미매칭 12개**: `__rust_dealloc`, `clamp`, `collect`, `continue`, `driftsort_main`, `drop  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `insertion_sort_shift_left`, `reserve_internal_or_panic`, `risk`, `skip`, `sort_by_key`, `take`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m11.ll:43032) · **형제 8개** (SmallActionAroundRegion)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::SmallActionAroundRegion as std::clone::Clone>::clone | pub | game-ai\src\small_action\around.rs:505 | True | fn(&game_ai::SmallActionAroundRegion) -> game_ai::SmallActionAroundRegion |
| 1 | <game_ai::SmallActionAroundRegion as std::fmt::Debug>::fmt | pub | game-ai\src\small_action\around.rs:505 | True | fn(&game_ai::SmallActionAroundRegion, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 2 | game_ai::SmallActionAroundRegion::new | pub | game-ai\src\small_action\around.rs:517 | False | fn(usize, &mut rand::rngs::std::StdRng, &game_core::OperationData, &game_core::PlayerState, usize, usize) -> game_ai::SmallActionAroundRegion |
| 3 | game_ai::SmallActionAroundRegion::get_input | in:game_ai | game-ai\src\small_action\around.rs:529 | False | fn(&mut game_ai::SmallActionAroundRegion, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData) -> std::option::Option<game_core::Input> |
| 4 | game_ai::SmallActionAroundRegion::merge | in:game_ai | game-ai\src\small_action\around.rs:649 | False | fn(&mut game_ai::SmallActionAroundRegion, usize, game_ai::SmallActionAroundRegion) |
| 5 | game_ai::SmallActionAroundRegion::get_action | in:game_ai | game-ai\src\small_action\around.rs:664 | True | fn(&game_ai::SmallActionAroundRegion) -> game_core::SmallAction |
| 6 | game_ai::SmallActionAroundRegion::is_end | in:game_ai | game-ai\src\small_action\around.rs:667 | False | fn(&game_ai::SmallActionAroundRegion, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bool |
| 7 | game_ai::SmallActionAroundRegion::near_move_complete | in:game_ai | game-ai\src\small_action\around.rs:673 | False | fn(&game_ai::SmallActionAroundRegion, &game_core::Entity) -> bool |

**`open` 7건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 표기 불가 | closure#0(563:30..51) 정렬 키의 정확한 소스 표기 — insertion_sort 인스턴스(m03.ll:117858-117866)가 원소+16(i64 score)을 slt 비교하므로 `c.4` 오름차순임은 확정, 튜플 패턴 표기만 불명(표기 불가) | 4 |  |
| 1 | 미탐색 | positioning_choice_window 5번째 bool 인자의 의미 — 이 함수는 항상 false 를 넘김(98860). 콜리 내부는 범위 밖 | 4 |  |
| 2 | 미탐색 | closure#3(616) 인라인 인스턴스(m03.ll:3634~)에서 dodge_tower_cell_with_context 에 version/data/ps 가 `poison` 으로 넘어감(7372) — LLVM 이 콜리에서 그 인자가 죽었다고 판정한 흔적(추정). dodge_tower_cell_with_context 가 실제로 version/data/ps 를 안 읽는다는 뜻이지만 콜리 내부는 범위 밖이라 미확인 | 4 |  |
| 3 | 미탐색 | 578 에서 goal_risk 를 '새 후보' 기준으로 기록하면서 579 에서 이전 goal 을 유지할 수 있다 — 유지 시 self.goal_risk 는 goal 과 다른 좌표의 risk 다(소스 의도인지 버그인지는 소스 없이 판정 불가, IR 동작만 확정) | 4 |  |
| 4 | 표기 불가 | walls/regions 인덱스 순서 [yb][xb](99434-99435: [30 x i64] 를 yb 로, 원소를 xb 로) — MapDef 필드 선언이 [y][x] 인지 [row][col] 명명인지는 tcx 가 배열로만 보여 미확인. 동작은 확정 | 3 |  |
| 5 | 미탐색 | `&mut rnd`·PathFinder 내부(update_path/get_input) 의 힙·상태 쓰기 표면은 콜리 소관(path_finder 계층 = 시그니처만) | 4 |  |
| 6 | 미탐색 | positioning_window_pick 가 제네릭(<T>)이라 후보 원소 타입에 대한 어떤 필드를 읽는지는 콜리 소관 — 이 함수는 반환 &T 의 +8/+12 만 읽음 | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

