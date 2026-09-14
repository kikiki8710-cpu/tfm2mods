---

### `172` SmallActionPositioning::get_input — 포지셔닝 목표: 도착/이탈 시 7×7 셀 후보를 가치 평가·정렬·창 안에서 rng 선택해 goal 을 갱신하고, 타워 회피 PathFinder 로 이동 입력 생성

| 항목 | 값 |
|---|---|
| id | `SmallActionPositioning__get_input` |
| 심볼 | `_RNvMs1_NtNtCshdEBA0ozCnw_7game_ai12small_action6aroundNtB5_22SmallActionPositioning9get_input` |
| 소스 | `game-ai\src\small_action\around.rs:701` |
| IR | `m08.ll` 102521~103100행 |
| 경로·가시성 | `game_ai::SmallActionPositioning::get_input` · **in:game_ai** |
| 계층 | 기타 |
| exe | `dbf680` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r14` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&mut game_ai::SmallActionPositioning, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData) -> std::option::Option<game_core::Input>
```

<details><summary>인자 7개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | *mut Option<Input>(32B) | None = i64 -1 @+0x0 | 4 |
| 1 | 1 | self | &mut SmallActionPositioning(112B) | ★&mut. 읽기 +0x8 goal_x · +0x10 goal_y · +0x6d path_finder 태그. 쓰기 = writes(goal_x·goal_y·goal_score·path_finder) | 4 |
| 2 | 2 | version | usize | 분기 1곳: L754 `version > 1` (현 목표 유지 비교 게이트). 나머지는 콜리 전달 | 4 |
| 3 | 3 | rnd | &mut StdRng(320B) | ★&mut — 본문 직접 store 0. positioning_window_pick · new_target · update_path · is_safe_recall 에 전달 | 4 |
| 4 | 4 | player | &PlayerState(2528B, readonly) | +0x930 team · +0x9c0 position · +0x180 info.parameter(&AthleteParameter → positioning_min/max_range) | 4 |
| 5 | 5 | data | &OperationData(24B, readonly) | +0x0 cache(player_champion) · +0x8 context(+0x0 pool · +0x20 map.walls) | 4 |
| 6 | 6 | positioning_score | &PositioningScoreData(2760B, readonly) | +0xab8 cx · +0xac0 cy (후보 격자 중심). 콜리 전달 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
SmallActionPositioning::get_input(&mut self, version, rnd, player, data, ps) -> Option<Input>   [around.rs:701]
 entity = data.cache.player_champion[player.team][player.position]?   → None 이면 return None      (L702)
 (ex,ey) = entity 위치 ; d2 = |ex-goal_x|² + |ey-goal_y|²                                          (L704)
 if d2 ≤ 5000² || d2 ≥ 96000²:            # 도착(≤5000) 또는 3셀 이상 이탈 → 목표 재선정        (L706, 접힌 범위비교)
     (cx, cy) = (ps.cx, ps.cy) as i32                                                              (L707~708)
     hazard = PveHazardContext::new(version, player, data)                                          (L709)
     candidates: bumpalo Vec<(i32 i, i32 j, i32 x, i32 y, i64 value)> in ctx.pool                  (L711)
     for i in 0..7: for j in 0..7:                                                                  (L713~714)
         ix = cx-3+i ; iy = cy-3+j ; x = ix*32000+16000 ; y = iy*32000+16000   (i32)               (L716)
         if ix<0 || iy<0 || ix>29 || iy>29 || map.walls[iy][ix] != 0: continue                     (L721)
         if v3_lethal_tower_position(version, player, data, x, y): continue                        (L725)
         if v3_lethal_wave_position(version, player, data, x, y): continue                         (L726)
         if hazard.lethal_zone_goal(x, y): continue                                                 (L727)
         value = positioning_cell_value_at(version, player, data, ps, x, y, Positioning)            (L731)
         candidates.push((i, j, x, y, value))                                                       (L732)
     if candidates.is_empty():                                                                      (L736)
         goal = (ex, ey)              # 제자리                                                       (L767~768 경유)
     else:
         candidates.sort_by_key(|c| -c.value)          # value 내림차순, 안정정렬                   (L739, aux)
         (lo‰, hi‰) = positioning_choice_window(version, player, athlete.positioning_min_range(), athlete.positioning_max_range(), false)   (L740~742)
         len = candidates.len(); lo = min(len-1, len*lo‰/1000); hi = min(max(len*hi‰/1000, lo), len-1)   (L743~744)
         window = candidates.into_iter().skip(lo).take(hi-lo+1).collect()   (bumpalo)              (L746~748)
         pick = positioning_window_pick(version, rnd, player, data, &window)    # &elem              (L750)
         if version > 1 && goal_x < 960000 && goal_y < 960000                                        (L754)
            && positioning_cell_value_at(version, player, data, ps, goal_x, goal_y, Positioning) >= pick.value:   (L755; IR `slt` 의 부정)
             s = positioning_score_at_position(…, goal_x, goal_y, Positioning); goal_score = s.gain - s.risk ; goal 유지   (L757~758)
         else:
             s = positioning_score_at_position(…, pick.x, pick.y, Positioning); goal_score = s.gain - s.risk ; goal = (pick.x, pick.y)   (L761~762)
     self.goal_x, self.goal_y = goal ; drop(candidates)                                             (L767~769)
 tower_dodge = TowerDodgeContext::new_tower_avoid_v3(version, player, data, goal_x, goal_y)          (L771)
 if self.path_finder.is_none():                                                                     (L772)
     self.path_finder = PathFinder::new_target(rnd, ctx, version, "dodge_tower_cell", ex, ey, goal_x, goal_y, |sx,sy,nx,ny| …{version,player,data,ps,&tower_dodge})   (L773)
     if self.path_finder.is_none(): return None                                                     (L782)
 pf.update_path(rnd, ctx, ex, ey, goal_x, goal_y, 같은 클로저(s0_0))                                 (L784)
 safe = is_safe_recall(version, rnd, player, data, ps)        # 잎22 · 계약만                        (L792)
 return Some(pf.get_input(player, data, SafeMoveWithSkill::Safe, safe))                            (L792)

클로저(경로 셀 술어)는 PositionBush 와 동일 계열(dodge_tower_cell_with_context 경유, 인스턴스 m03.ll 10867~ / 27054~ 추정 — 내부 미탐색). 잎22 호출 지점 = is_safe_recall 1곳(102953): (version, &mut rnd, &PlayerState, &OperationData, &PositioningScoreData) -> bool, 결과는 get_input 의 5번째 bool 로만 쓰임.
```

**`mem` 메모리 접근 27건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | 102568 | 4 | OK |  |
| 1 | PlayerState | 0x9c0 | info.position@tag | r | 102579 | 4 | OK |  |
| 2 | PlayerState | 0x180 | info.parameter | r | &AthleteParameter — positioning_min_range/max_range 의 self (102779) | 4 | OK |  |
| 3 | OperationData | 0x0 | cache | r | 102581 | 4 | OK |  |
| 4 | AbstractGameWithCache | 0x1e0 | player_champion[2][5] | r | null → return None (102582~102627, `?`) | 4 | OK |  |
| 5 | Entity | 0x660 | x | r | ex (102593) | 4 | OK |  |
| 6 | Entity | 0x668 | y | r | ey (102597) | 4 | OK |  |
| 7 | SmallActionPositioning | 0x8 | goal_x | r | 102601 | 4 | OK |  |
| 8 | SmallActionPositioning | 0x10 | goal_y | r | 102605 | 4 | OK |  |
| 9 | SmallActionPositioning | 0x6d | path_finder@tag | r | +0x28+0x45, 2=None (102672·102934) | 4 | OK |  |
| 10 | SmallActionPositioning | 0x28 | path_finder 페이로드 | r | PathFinder 72B — update_path/get_input 의 &mut (102669 `gep %1, 40`) | 4 | OK |  |
| 11 | PositioningScoreData | 0xab8 | cx | r | 후보 격자 중심 셀 x (102635, L707) | 4 | OK |  |
| 12 | PositioningScoreData | 0xac0 | cy | r | 102639 (L708) | 4 | OK |  |
| 13 | OperationData | 0x8 | context | r | 102646 등 | 4 | OK |  |
| 14 | GameContext | 0x0 | pool | r | &Bump — bumpalo Vec 후보 목록 할당자 (102647) | 4 | OK |  |
| 15 | GameContext | 0x20 | map | r | 102658 → walls | 4 | OK |  |
| 16 | MapDef | 0x78 | walls[30][30] | r | walls[iy][ix] != 0 이면 후보 제외 (102997~103002, L721) | 4 | OK |  |
| 17 | candidate(i32,i32,i32,i32,i64)(24B 원소) | 0x8 | x / +0xc y / +0x10 value | r | positioning_window_pick 반환 원소 (102869~102874, 102882) | 4 | 확인불가(★모호: 동명 def_path 9개 [('game_core::Databa) |  |
| 18 | PositioningScore(56B 지역) | 0x0 | risk / +0x10 gain | r | goal_score = gain - risk (102894~102897, 102904~102907) | 4 | OK |  |
| 19 | SmallActionPositioning(self) | 0x8 | goal_x | w | 102753 (L767). 목표 재선정 블록에서만 | 4 | OK | 후보 없음 → entity.x / 현 목표 유지 → 기존 goal_x / 선택 → pick.x(i32→i64) |
| 20 | SmallActionPositioning(self) | 0x10 | goal_y | w | 102754 (L768) | 4 | OK | 위와 짝 |
| 21 | SmallActionPositioning(self) | 0x18 | goal_score | w | 102898·102908. 후보 없음 경로에선 미기록 | 4 | OK | positioning_score_at_position(...).gain - .risk (선택 셀 L762 / 유지 셀 L758) |
| 22 | SmallActionPositioning(self) | 0x28 | path_finder | w | None 일 때만 | 4 | OK | PathFinder::new_target(rnd, ctx, version, "dodge_tower_cell"(16B), ex, ey, goal_x, goal_y, closure s_0{version,player,data,ps,&tower_dodge}) 72B memcpy (102930, L773) |
| 23 | SmallActionPositioning(self) | 0x28 | path_finder(콜리 경유) | w | path_finder 계층 미탐색 | 4 | OK | update_path(&mut)(102950) · get_input(&mut)(102954) |
| 24 | bumpalo Vec(지역 %18, pool) | 0x0 | candidates ptr/+0x8 bump/+0x10 cap/+0x18 len | w | 지역 변수(스택 32B, 데이터는 pool). drop 102756 — 부작용은 pool 소비 | 4 | 확인불가(tcx 사전에 타입 없음) | push (i,j,x,y,value) 24B 원소 — +0 i(i32) +4 j +8 x +12 y +16 value (103077~103088) |
| 25 | Option<Input>(sret) | 0x0 | tag/Input | w |  | 4 | 확인불가(tcx 사전에 타입 없음) | -1 (102627·102959) / get_input sret (102954) |
| 26 | StdRng(rnd) | 0x0 | rng 상태 | w | 본문 직접 store 없음 | 4 | OK | positioning_window_pick · new_target · update_path · is_safe_recall 내부 |

**`consts` 상수 16건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | -9216000000 | 704 | 계수 | -(96000²) — d² = \|ex-goal_x\|²+\|ey-goal_y\|² 에서 96000² 을 뺀 뒤 범위비교(102621). 96000 = 3셀 | 4 |
| 1 | -9190999999 | 706 | 임계 | 2^64-9190999999 과의 unsigned 비교(102623 `icmp ult`) = 접힌 범위검사. 풀면: **d² ≤ 25000000(=5000²) 또는 d² ≥ 9216000000(=96000²)** 이면 목표 재선정(도착 or 3셀 이상 이탈). 소스 원식 표기(<,<=)는 접힘으로 표기 불가, 동작은 확정 | 4 |
| 2 | 7 | 713 | 임계 | 후보 격자 7×7 (i,j in 0..7) — cx-3..cx+3 (102687·102970) | 4 |
| 3 | -3 | 713 | 계수 | ix = cx-3+i, iy = cy-3+j (102656·102657) | 4 |
| 4 | 32000 | 716 | 계수 | 셀→월드 x = ix*32000+16000 (i32 산술, 102708·102985) | 4 |
| 5 | 16000 | 716 | 계수 | 셀 중심 | 4 |
| 6 | 29 | 721 | 임계 | ix>29 \|\| iy>29 → 후보 제외(격자 밖) (102710·102991) | 4 |
| 7 | 0 | 721 | 임계 | walls[iy][ix] == 0 만 후보 (103002). 또한 ix\|iy < 0 검사(102988 `or` 후 slt 0) | 4 |
| 8 | 6 | 731 | 센티널 | PositionEvalPurpose::Positioning 의 메모리 태그 6(니치, idx 4) — positioning_cell_value_at / positioning_score_at_position 마지막 인자 (103028·102864·102877·102889) | 4 |
| 9 | 21 | 739 | 임계 | sort_by_key: len<21 이면 insertion_sort, 아니면 driftsort (std 정렬 임계, 102736). 판정값 아님 | 4 |
| 10 | 1 | 739 | 태그 | len==1 이면 정렬 생략(102731). 또한 L754 `version > 1` 게이트(102856), L748 take 길이 hi-lo+1(102828), get_input SafeMoveWithSkill::Safe 태그(102954) | 4 |
| 11 | 1000 | 743 | 계수 | choice_window 이 돌려주는 (lo,hi) 는 천분율: lo_idx = min(len-1, len*lo/1000), hi_idx = min(max(len*hi/1000, lo_idx), len-1) (102798·102808) | 4 |
| 12 | -1 | 743 | 태그 | len-1 상한(102799). 또한 Option<Input>::None 태그(102627·102959) | 4 |
| 13 | 960000 | 754 | 임계 | 30×32000 — goal_x/goal_y < 960000 (격자 안) 일 때만 현 목표 유지 비교 수행 (102857·102859) | 4 |
| 14 | 16 | 773 | 길이 | "dodge_tower_cell" key 길이 (102928) | 4 |
| 15 | 2 | 772 | 센티널 | Option<PathFinder>::None 니치 태그(102672·102934). player_champion 팀 상한 2 도 같은 리터럴 | 4 |

**`knobs` 조정점 5건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 목표 재선정 트리거(도착 반경 / 이탈 반경) | around.rs:704~706 | 5000 / 96000 | 도착 반경을 올리면 목표 근처에서 더 자주 재선정(요동), 이탈 반경을 내리면 밀려났을 때 더 빨리 새 목표 | 4 | 기존 |
| 1 | 후보 격자 반경 | around.rs:713~716 | 7×7 (±3셀) | 넓히면 더 먼 셀도 포지셔닝 후보 — 비용 49→(2r+1)² 회 cell_value 평가 | 4 | 기존 |
| 2 | 선택 창(천분율) | around.rs:740~744 (positioning_choice_window 반환) | athlete positioning_min/max_range 기반 | 창을 좁히면(상위만) 최적 셀 고정, 넓히면 무작위성↑ — 선수 능력치 반영 지점 | 4 | 기존 |
| 3 | 현 목표 유지 조건 | around.rs:754~755 | version>1 && cur_value >= pick.value | 끄면(version≤1) 매 재선정마다 새 pick 로 갈아타 목표가 요동 | 4 | 기존 |
| 4 | 평가 목적 태그 | around.rs:731/755/757/761 | PositionEvalPurpose::Positioning(6) | 다른 목적(Around 5 등)으로 바꾸면 score_parameter 가중치 세트가 바뀜 | 4 | 기존 |

<details><summary>`callees` 피호출자 21건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | get_input | game_ai::PathFinder::get_input | pub | fn(&mut game_ai::PathFinder, &game_core::PlayerState, &game_core::OperationData, game_ai::SafeMoveWithSkill, bool) -> game_core::Input | game-ai\src\path_finder.rs:856 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | is_empty | game_core::PatchSetting::is_empty | pub | fn(&game_core::PatchSetting) -> bool | game-core\src\patch_engine.rs:853 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 2 | is_empty | game_core::ModAiRegistry::is_empty | pub | fn(&game_core::ModAiRegistry) -> bool | game-core\src\mod_ai.rs:15 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 3 | is_empty | game_core::PublicChampionEvidence::is_empty | pub | fn(&game_core::PublicChampionEvidence) -> bool | game-core\src\banpick\public_evidence.rs:96 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 4 | is_safe_recall | game_ai::small_action::cast::is_safe_recall | in:game_ai | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData) -> bool | game-ai\src\small_action\cast.rs:304 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 5 | lethal_zone_goal | game_ai::PveHazardContext::lethal_zone_goal | pub | fn(&game_ai::PveHazardContext, u64, u64) -> bool | game-ai\src\path_finder.rs:1167 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 6 | new | game_ai::PveHazardContext::new | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData) -> game_ai::PveHazardContext | game-ai\src\path_finder.rs:1120 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 7 | new_target | game_ai::PathFinder::new_target | pub | fn(&mut rand::rngs::std::StdRng, &game_core::GameContext, usize, &str, u64, u64, u64, u64, impl Fn(usize, usize, usize, usize) -> PathVerdict/#0) -> game_ai::PathFinder | game-ai\src\path_finder.rs:66 | True | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 8 | new_tower_avoid_v3 | game_ai::TowerDodgeContext::new_tower_avoid_v3 | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, u64, u64) -> game_ai::TowerDodgeContext | game-ai\src\path_finder.rs:1205 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 9 | positioning_cell_value_at | game_ai::small_action::positioning_cell_value_at | in:game_ai::small_action | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, u64, u64, game_ai::PositionEvalPurpose) -> i64 | game-ai\src\small_action.rs:104 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 10 | positioning_choice_window | game_ai::small_action::positioning_choice_window | in:game_ai::small_action | fn(usize, &game_core::PlayerState, usize, usize, bool) -> (usize, usize) | game-ai\src\small_action.rs:43 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 11 | positioning_max_range | game_core::AthleteParameter::positioning_max_range | pub | fn(&game_core::AthleteParameter) -> usize | game-core\src\simulation\state\player.rs:312 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 12 | positioning_min_range | game_core::AthleteParameter::positioning_min_range | pub | fn(&game_core::AthleteParameter) -> usize | game-core\src\simulation\state\player.rs:306 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 13 | positioning_score_at_position | game_ai::small_action::positioning_score_at_position | in:game_ai::small_action | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, u64, u64, game_ai::PositionEvalPurpose) -> game_core::PositioningScore | game-ai\src\small_action.rs:99 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 14 | positioning_window_pick | game_ai::small_action::positioning_window_pick | in:game_ai::small_action | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &[T/#0]) -> &T/#0 | game-ai\src\small_action.rs:66 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 15 | push | <game_core::Staff as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::staff | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\staff.rs:38 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 6개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 16 | push | <game_core::Athlete as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:434 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 6개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 17 | push | <game_core::Contract as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:2498 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 6개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 18 | update_path | game_ai::PathFinder::update_path | pub | fn(&mut game_ai::PathFinder, &mut rand::rngs::std::StdRng, &game_core::GameContext, u64, u64, u64, u64, impl Fn(usize, usize, usize, usize) -> PathVerdict/#0) | game-ai\src\path_finder.rs:631 | True | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 19 | v3_lethal_tower_position | game_ai::v3_lethal_tower_position | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, u64, u64) -> bool | game-ai\src\tower_discipline.rs:282 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 20 | v3_lethal_wave_position | game_ai::v3_lethal_wave_position | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, u64, u64) -> bool | game-ai\src\minion_wave_risk.rs:251 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
</details>

⚠**미매칭 12개**: `Take`, `collect`, `continue`, `driftsort_main`, `drop  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `elem`, `insertion_sort_shift_left`, `pool`, `reserve_internal_or_panic`, `skip`, `sort_by_key`, `take`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m11.ll:43042) · **형제 8개** (SmallActionPositioning)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::SmallActionPositioning as std::clone::Clone>::clone | pub | game-ai\src\small_action\around.rs:678 | True | fn(&game_ai::SmallActionPositioning) -> game_ai::SmallActionPositioning |
| 1 | <game_ai::SmallActionPositioning as std::fmt::Debug>::fmt | pub | game-ai\src\small_action\around.rs:678 | True | fn(&game_ai::SmallActionPositioning, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 2 | game_ai::SmallActionPositioning::new | pub | game-ai\src\small_action\around.rs:689 | False | fn(usize, &mut rand::rngs::std::StdRng, &game_core::OperationData, &game_core::PlayerState, usize) -> game_ai::SmallActionPositioning |
| 3 | game_ai::SmallActionPositioning::get_input | in:game_ai | game-ai\src\small_action\around.rs:701 | False | fn(&mut game_ai::SmallActionPositioning, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData) -> std::option::Option<game_core::Input> |
| 4 | game_ai::SmallActionPositioning::merge | in:game_ai | game-ai\src\small_action\around.rs:795 | False | fn(&mut game_ai::SmallActionPositioning, game_ai::SmallActionPositioning) |
| 5 | game_ai::SmallActionPositioning::get_action | in:game_ai | game-ai\src\small_action\around.rs:799 | True | fn(&game_ai::SmallActionPositioning) -> game_core::SmallAction |
| 6 | game_ai::SmallActionPositioning::is_end | in:game_ai | game-ai\src\small_action\around.rs:802 | False | fn(&game_ai::SmallActionPositioning, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bool |
| 7 | game_ai::SmallActionPositioning::near_move_complete | in:game_ai | game-ai\src\small_action\around.rs:808 | False | fn(&game_ai::SmallActionPositioning, &game_core::Entity) -> bool |

**`open` 6건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 표기 불가 | L706 소스 원식 — IR 이 `d2-96000²` 의 unsigned 범위비교로 접어 `<`/`<=` 표기와 두 조건의 순서는 표기 불가(동작은 d2≤25000000 ∨ d2≥9216000000 로 확정) | 4 |  |
| 1 | 미탐색 | candidates 튜플의 i,j(격자 인덱스 0..7) 가 positioning_window_pick 안에서 쓰이는지 — 본문은 x,y,value 만 읽음. small_action 계층 미탐색 | 4 |  |
| 2 | 미탐색 | positioning_window_pick 의 선택 규칙(rng 사용 여부·분포) — &mut rnd 를 받으므로 소비 가능성 있음. 미탐색 | 4 |  |
| 3 | 미탐색 | PathFinder::new_target 의 None 반환 가능성(L782 재검사) — path_finder 계층(Bush 계열과 같은 의문) | 4 |  |
| 4 | 미탐색 | 후보 없음(empty) 경로에서 goal_score(+0x18) 가 갱신되지 않는 것이 의도인지 — IR 사실만 기록 | 4 |  |
| 5 | 미탐색 | AthleteParameter::positioning_min_range/max_range 내부(game_core 경계) — fn(&AthleteParameter)->usize 계약만 | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

