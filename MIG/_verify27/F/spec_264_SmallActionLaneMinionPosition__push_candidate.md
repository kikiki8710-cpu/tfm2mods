---

### `264` SmallActionLaneMinionPosition::push_candidate — 라인 미니언 공격 위치 후보 (x,y) 하나를 벽·우물·라인·불필요 타워·사거리·스탠스 위험으로 거르고 점수를 매겨 candidates 에 push

| 항목 | 값 |
|---|---|
| id | `SmallActionLaneMinionPosition__push_candidate` |
| 심볼 | `_RNvMNtNtCshdEBA0ozCnw_7game_ai12small_action11lane_minionNtB2_29SmallActionLaneMinionPosition14push_candidate` |
| 소스 | `game-ai\src\small_action\lane_minion.rs:439` |
| IR | `m11.ll` 44238~44465행 |
| 경로·가시성 | `game_ai::SmallActionLaneMinionPosition::push_candidate` · **in:game_ai** |
| 계층 | 기타 |
| exe | `e25450` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r18` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&mut bumpalo::collections::vec::Vec< (u64, u64, i64)>, usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, &game_core::Entity, &game_core::PositioningScoreData, u64, u64, game_ai::PositionEvalPurpose, u64, u64, i64, game_core::LineType, i64)
```

**TLS 메모 절(`sig.tls` · 정정 경로 `/specs[264]/sig/tls/<키>`)**

- `name`: 없음
- `role`: 없음 — 본문(44238~44465)에 `LocalKey`/`call_once`/`llvm.threadlocal.address`/`@anon.* = constant ptr` 참조 0
- `key`: 해당 없음
- `layout`: 해당 없음
- `invalidation`: 해당 없음
- `call_conditions`: 콜리 position_score_at_position(→ position_eval_at, 계약만)·lane_stance_risk 내부의 TLS 메모 여부는 범위 밖(미열람)

<details><summary>인자 16개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | candidates | &mut bumpalo::Vec<(u64,u64,i64)>(32B) %0 | define 줄 속성: noalias noundef nonnull align 8 dereferenceable(32) — readonly 없음(가변) · initializes 속성 없음. 쓰기는 bumpalo Vec::push(m11.ll:44459) 한 곳뿐(콜리 안에서 ptr/cap/len 갱신·원소 24B 기록). 이 함수 본문의 직접 store 는 지역 %18(24B 튜플) 뿐 | 4 |
| 1 | 2 | version | usize (i64 %1) | 속성 noundef. 본문 분기 없음 — is_enemy_well_danger·is_unnecessary_enemy_tower_position·position_score_at_position 1번째 인자로 전달. lane_stance_risk 에는 `i64 poison` 을 넘김(44433 · 콜리가 안 읽는다는 뜻) | 4 |
| 2 | 3 | player | &PlayerState(2528B) %2 | 속성 noalias noundef nonnull readonly captures(address, read_provenance) dereferenceable(2528). +0x180 info.parameter(AthleteParameter 744B) 를 positioning_effective(×2 경로)·skill_avoid_effective 의 self 로. 그 밖엔 콜리 전달 | 4 |
| 3 | 4 | data | &OperationData(24B) %3 | 속성 noalias noundef nonnull readonly captures(address, read_provenance) dereferenceable(24). +8 context → GameContext(+0x20 map → MapDef+0x78 walls) 읽음 · is_near_line 에 &GameContext 전달 | 4 |
| 4 | 5 | champ | &Entity(1728B) %4 | 속성 noalias noundef nonnull readonly captures(address, read_provenance) dereferenceable(1728). +0x660 x · +0x668 y 만 읽음(482 move_penalty) · lane_stance_risk 4번째 인자 | 4 |
| 5 | 6 | target.x | i64 %5 (ArgumentPromotion 조각 1/2 · DI `target` = ptr poison) | IR 속성 없음. 468줄 utils::distance 3번째 인자로만. 호출자 choose_goal 이 target+0x660 을 load 해 넘김(m11.ll:43534~43535) | 4 |
| 6 | 6 | target.y | i64 %6 (ArgumentPromotion 조각 2/2) | IR 속성 없음. 468줄 utils::distance 4번째 인자로만(target+0x668, 43536~43537) | 4 |
| 7 | 7 | positioning_score | &PositioningScoreData(2760B) %7 | 속성 noalias noundef nonnull readonly captures(address, read_provenance) dereferenceable(2760). +0xab8 cx · +0xac0 cy 읽음(363~364 격자 안 판정) · position_score_at_position 4번째 인자 | 4 |
| 8 | 8 | x | u64 (i64 %8, noundef) | 후보 좌표 x. 456 `x/32000` clamp(0,29) = xi | 4 |
| 9 | 9 | y | u64 (i64 %9, noundef) | 후보 좌표 y. 457 yi | 4 |
| 10 | 10 | position_eval_purpose | PositionEvalPurpose (i8 %10, noundef range(i8 0,13)) | 본문 분기 없음 — position_score_at_position 7번째 인자로 전달만 | 4 |
| 11 | 11 | attack_range | u64 (i64 %11, noundef) | 469 `dist_to_target > attack_range` 거절 · 473 `> 69999` 장거리 분기 · 473 min_spacing = attack_range*55/100 | 4 |
| 12 | 12 | preferred_range | u64 (i64 %12, noundef range(i64 0, -14000)) | 480 band_penalty = \|dist_to_target - preferred_range\| | 4 |
| 13 | 13 | target_score | i64 %13 (noundef) | 489 점수 기저(= 호출자의 self.goal_score) | 4 |
| 14 | 14 | line | LineType (i8 %14, noundef range(i8 0,3)) | is_near_line 4번째 인자로만 | 4 |
| 15 | 15 | source_bonus | i64 %15 (noundef range(i64 0,36)) | 489 가산(호출자: 현재위치 35 · 링 0 · 격자 10) | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn push_candidate(candidates, version, player, data, champ, target{x,y}, positioning_score, x, y, purpose, attack_range, preferred_range, target_score, line, source_bonus)   // lane_minion.rs:439~454
// 456~458  격자 셀·벽
xi = (x/32000).clamp(0,29); yi = (y/32000).clamp(0,29);
if data.context.map.walls[yi][xi] != 0 { return }                                   // 458 (벽/막힌 셀)
if is_enemy_well_danger(version, player, x, y) { return }                            // 458 (같은 줄 `||` · 좌우 순서 표기 불가·단락 순서는 IR 대로 walls 먼저)
// 461
if !is_near_line(data.context, x, y, line) { return }                                // 라인 근처가 아니면 거절
// 464
if is_unnecessary_enemy_tower_position(version, player, data, x, y) { return }
// 468~469
dist_to_target = distance(x, y, target.x, target.y);
if dist_to_target > attack_range { return }                                          // 사거리 밖 후보 거절
// 473~476  장거리 챔프 근접 페널티
close_penalty = if attack_range > 69999 { min_spacing = attack_range*55/100;
                    if dist_to_target < min_spacing { (min_spacing - dist_to_target)/1000 } else { 0 } } else { 0 };
// 480
band_penalty = (dist_to_target as i64 - preferred_range as i64).abs();               // 나중에 /-3000
// 481  local_score = Self::local_position_score(version, player, data, positioning_score, x, y, purpose) (lane_minion.rs:360~378 인라인)
//   361~365: xi,yi(위와 동일) · cx,cy = positioning_score.cx/cy · if |xi-cx|>3 || |yi-cy|>3 { local_score = 0 } else {
//   369: score: PositioningScore(56B) = positioning_score_at_position(version, player, data, positioning_score, x, y, purpose)  (small_action.rs:101 래퍼 → position_eval::position_score_at_position · 계약만)
//   370: on_trajectory = score.on_trajectory || score.on_periodic_trajectory
//   371: positioning = player.info.parameter.positioning_effective()
//   372~373: trajectory_penalty = if on_trajectory { clamp(220 - 3*max(50 - positioning, 0), 70, 220) } else { 0 }
//   377: local_score = score.gain - (trajectory_penalty + positioning_risk_value(version, player, score.risk, on_trajectory=false))   // = risk*2 · skill_avoid_effective 호출 결과 미사용
//   }
// 482
move_penalty = distance(champ.x, champ.y, x, y);                                     // 나중에 /-5000
// 483
let stance_risk = Self::lane_stance_risk(version(poison), player, data, champ, x, y)?;   // Option<i64> · None 이면 거절(push 없이 ret)
// 489~490
score = target_score + 80 + source_bonus + band_penalty/-3000 - close_penalty + local_score + move_penalty/-5000 - stance_risk;   // i64 sdiv(0 방향 절삭)
candidates.push((x, y, score));

※ 거절 6경로(벽·우물위험·라인 밖·불필요 타워·사거리 밖·stance_risk None) 는 전부 push 없이 ret — 호출자 choose_goal 은 후보 0개면 None.
※ 481 줄 local_position_score 안에서 position_score_at_position 은 격자 안(|Δ|<=3)일 때만 호출된다 — 격자 밖 후보는 local_score=0 이지만 거절되진 않는다.
```

**`mem` 메모리 접근 13건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | OperationData | 0x8 | context (&GameContext) | r | m11.ll:44268~44269 (%23/%24). is_near_line 1번째 인자(dereferenceable(64) = GameContext) | 4 | OK |  |
| 1 | GameContext | 0x20 | map (&MapDef) | r | m11.ll:44270~44271 (%25/%26) | 4 | OK |  |
| 2 | MapDef | 0x78 | walls[yi][xi] ([[usize;30];30] · gep [30 x i64] %22(yi) 행 → %20(xi) 열) | r | m11.ll:44272~44275 (%27~%30). != 0 이면 거절(458). tcxdict MapDef 0x78 = walls[0][0] | 3 | OK |  |
| 3 | PlayerState | 0x180 | info.parameter (AthleteParameter 744B 시작) | r | m11.ll:44377·44384·44416 (%77/%80/%95). positioning_effective(371)·skill_avoid_effective(377 positioning_risk_value 인라인) 의 &self | 4 | OK |  |
| 4 | PositioningScoreData | 0xab8 | cx | r | m11.ll:44340~44342 (%56~%58 · trunc i32). 363줄 local_position_score 인라인 | 4 | OK |  |
| 5 | PositioningScoreData | 0xac0 | cy | r | m11.ll:44344~44346 (%59~%61). 364줄 | 4 | OK |  |
| 6 | PositioningScore (지역 sret %17 56B · position_score_at_position 결과) | 0x0 | risk | r | m11.ll:44411 (%94). 377줄 positioning_risk_value(version, player, risk, on_trajectory=false) 인라인 → risk*2(shl 1, 44418) | 4 | OK |  |
| 7 | PositioningScore (지역 %17) | 0x10 | gain | r | m11.ll:44409~44410 (%92/%93). local_score = gain - (trajectory_penalty + risk*2)(377) | 4 | OK |  |
| 8 | PositioningScore (지역 %17) | 0x30 | on_trajectory (bool) | r | m11.ll:44367~44369 (%70~%72). 370줄 `on_trajectory \|\| on_periodic_trajectory`(단락: 참이면 +0x31 안 읽음 · 블록 %79) | 4 | OK |  |
| 9 | PositioningScore (지역 %17) | 0x31 | on_periodic_trajectory (bool) | r | m11.ll:44373~44375 (%74~%76). +0x30 이 거짓일 때만 읽음(블록 %73) | 4 | OK |  |
| 10 | Entity | 0x660 | x (champ) | r | m11.ll:44427~44428 (%102/%103). 482 move_penalty = distance(champ.x, champ.y, x, y) | 4 | OK |  |
| 11 | Entity | 0x668 | y (champ) | r | m11.ll:44429~44430 (%104/%105) | 4 | OK |  |
| 12 | &mut bumpalo::Vec<(u64,u64,i64)> (candidates %0) | 0x0..0x20 | push 1회 — 원소 24B (+0 x u64 · +8 y u64 · +0x10 score i64) · Vec ptr/cap/len 은 콜리 push 가 갱신 | w | 쓰기 표면 전수 = 이 push 1회(모든 거절 경로는 쓰기 0). initializes 속성 없음. 원소 안 live 바이트 24B 전부(패딩 없음 · u64,u64,i64) | 4 | 확인불가(tcx 사전에 타입 없음) | (x, y, score) — 44454~44458 지역 %18 에 store 후 44459 `bumpalo::Vec::push(candidates, &%18)` |

**`consts` 상수 19건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev | folded_from |
|---|---|---|---|---|---|---|
| 0 | 32000 | 456 | 계수 | 셀 크기 — xi = x/32000 · yi = y/32000 (udiv 44256·44262) | 4 |  |
| 1 | 29 | 456 | 인덱스 | clamp(0, 29) 상한(cmp.rs:2027 인라인 · umin 44260·44266) — 30×30 격자 인덱스. 하한 0 은 unsigned 라 접힘 | 4 |  |
| 2 | 0 | 458 | 태그 | walls[yi][xi] == 0 이어야 통과(44276) · 481 격자 밖이면 local_score = 0(44425 phi) · 473~474 close_penalty 기본 0(44315 phi) · 372 on_trajectory 거짓이면 trajectory_penalty 0(44407 phi) · 373 smax(·,0) | 4 |  |
| 3 | 69999 | 473 | 임계 | `attack_range > 69999`(44298) 장거리 챔프만 근접 페널티(close_penalty) 적용 — choose_goal 495 와 같은 임계 | 4 |  |
| 4 | 55 | 473 | 계수 | min_spacing = attack_range*55/100(44302~44303) — 장거리 챔프가 목표에 이보다 가까이 서면 페널티 | 4 |  |
| 5 | 100 | 473 | 계수 | 백분율 분모(44303 udiv) | 4 |  |
| 6 | 1000 | 475 | 계수 | close_penalty = (min_spacing - dist_to_target)/1000(44310 udiv) — 부족 거리 1000 당 1점 감점 | 4 |  |
| 7 | -4 | 363 | 미상 | local_position_score 인라인: gx = xi - 4 - cx(44348) · gy = yi - 4 - cy(44351). `(gx as u32) < 0xFFFFFFF9`(44350 icmp ult -7) 가 참이면 격자 밖 ⟺ \|xi-cx\|>3 또는 \|yi-cy\|>3 → local_score 0 | 4 |  |
| 8 | -7 | 365 | 임계 | 위 격자 밖 판정의 unsigned 비교값(0xFFFFFFF9 · 44350·44353). 유효 = gx ∈ [-7,-1] ⟺ xi-cx ∈ [-3,3](7×7 격자) | 4 |  |
| 9 | 48 | 370 | 미상 | PositioningScore+0x30 on_trajectory 오프셋(44367) — 필드 오프셋이지만 gep 상수라 등록 | 4 |  |
| 10 | 49 | 370 | 미상 | PositioningScore+0x31 on_periodic_trajectory 오프셋(44373) | 4 |  |
| 11 | 50 | 373 | 계수 | trajectory_penalty = clamp(220 - 3*max(50 - positioning_effective, 0), 70, 220) — 포지셔닝 능력치 50 기준(44392) | 4 |  |
| 12 | -3 | 373 | 계수 | 능력치 부족분 1당 페널티 3 감소(`mul -3` 44396 · 220 에서 뺌) | 4 |  |
| 13 | 220 | 373 | 임계 | 궤적 위 페널티 최대(능력치<=50 근처)·clamp 상한(44397·44402 umin) | 4 |  |
| 14 | 70 | 373 | 임계 | 궤적 위 페널티 최소·clamp 하한(44401 smax) — 능력치 100 이면 220-150=70 | 4 |  |
| 15 | 1 | 377 | 계수 | positioning_risk_value(…, on_trajectory=false) 인라인(small_action.rs:81) = risk*2 가 `shl i64 %94, 1`(44418)로 접힘 — value 1 은 시프트량·소스값은 2(IR 에 리터럴 2 없음). 같은 인라인이 skill_avoid_effective 를 호출하지만 결과 %96 미사용(on_trajectory=false 경로라 사장 · 콜리가 readnone 이 아니라 호출만 남음) | 4 | 2 |
| 16 | -5000 | 482 | 계수 | move_penalty = distance(champ, 후보)/-5000(sdiv 44440) — 현재 위치에서 5000 멀어질 때마다 -1 | 4 |  |
| 17 | -3000 | 480 | 계수 | band_penalty = \|dist_to_target - preferred_range\|/-3000(sdiv 44442 · abs 44319) — 선호 사거리에서 3000 벗어날 때마다 -1 | 4 |  |
| 18 | 80 | 489 | 계수 | 점수 기저 가산: score = target_score + 80 + source_bonus + band_penalty/-3000 - close_penalty + local_score + move_penalty/-5000 - stance_risk(44445~44451) | 4 |  |

**`knobs` 조정점 7건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 점수 기저 가산 | lane_minion.rs:489 (m11.ll:44445) | 80 | 모든 후보에 균등 가산이라 후보 간 순위엔 영향 없음 — 호출자 밖(다른 small_action 과의 비교)에서만 의미. 올리면 lane_minion 목표 점수 전반↑ | 4 | 기존 |
| 1 | 장거리 근접 페널티 비율(%) | lane_minion.rs:473 (m11.ll:44302) | 55 | 올리면 장거리 챔프가 목표에서 더 멀리 서려 함(사거리 55% 안 후보 감점). 1000 분모(475)를 내리면 감점 기울기↑ | 4 | 기존 |
| 2 | 선호 사거리 이탈 페널티 분모 | lane_minion.rs:480·489 (m11.ll:44442 sdiv -3000) | -3000 | 절댓값을 내리면 preferred_range 밴드를 더 엄격히 지킨다(밴드 밖 후보 감점↑) | 4 | 기존 |
| 3 | 이동 거리 페널티 분모 | lane_minion.rs:482 (m11.ll:44440 sdiv -5000) | -5000 | 절댓값을 내리면 현재 위치 근처 후보 선호↑(이동 감소). 호출자의 현재위치 bonus 35 와 합쳐 '제자리' 성향을 결정 | 4 | 기존 |
| 4 | 궤적 위 페널티 clamp(70~220)·능력치 기준 50·기울기 3 | lane_minion.rs:373 (m11.ll:44392~44402) | 220 - 3*max(50-positioning,0), clamp 70..220 | 포지셔닝 능력치가 낮을수록 스킬 궤적 위 셀을 크게 감점(최대 220). 220/70 을 올리면 궤적 회피 성향↑, 기울기 3 을 올리면 능력치 차이가 더 벌어짐 | 4 | 기존 |
| 5 | 포지셔닝 risk 배수 | small_action.rs:81 positioning_risk_value 인라인 (m11.ll:44418 shl 1) | 2 | 올리면 position_score.risk 가 큰 셀(적 위협)을 더 피함 | 4 | 기존 |
| 6 | 격자 안 판정 반경 ±3 | lane_minion.rs:363~365 (m11.ll:44348~44353 -4/-7) | 3 | positioning_score 7×7 격자 크기와 묶여 있어 단독 변경 불가(격자 밖은 local_score 0) | 4 | 기존 |

<details><summary>`callees` 피호출자 15건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | distance | game_core::utils::distance | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:12 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | is_enemy_well_danger | game_ai::is_enemy_well_danger | pub | fn(usize, &game_core::PlayerState, u64, u64) -> bool | game-ai\src\path_finder.rs:1032 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 2 | is_near_line | game_core::is_near_line | pub | fn(&game_core::GameContext, u64, u64, game_core::LineType) -> bool | game-core\src\simulation\map_regions.rs:139 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 3 | is_unnecessary_enemy_tower_position | game_ai::is_unnecessary_enemy_tower_position | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, u64, u64) -> bool | game-ai\src\path_finder.rs:1399 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | lane_stance_risk | game_ai::SmallActionLaneMinionPosition::lane_stance_risk | in:game_ai | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, u64, u64) -> std::option::Option<i64> | game-ai\src\small_action\lane_minion.rs:380 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 5 | local_position_score | game_ai::SmallActionLaneMinionPosition::local_position_score | in:game_ai | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, u64, u64, game_ai::PositionEvalPurpose) -> i64 | game-ai\src\small_action\lane_minion.rs:359 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 6 | position_score_at_position | game_ai::position_score_at_position | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, u64, u64, game_ai::PositionEvalPurpose) -> game_core::PositioningScore | game-ai\src\position_eval.rs:1192 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 7 | positioning_effective | game_core::AthleteParameter::positioning_effective | pub | fn(&game_core::AthleteParameter) -> usize | game-core\src\simulation\state\player.rs:291 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 8 | positioning_risk_value | game_ai::small_action::positioning_risk_value | in:game_ai::small_action | fn(usize, &game_core::PlayerState, i64, bool) -> i64 | game-ai\src\small_action.rs:79 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 9 | positioning_score_at_position | game_ai::small_action::positioning_score_at_position | in:game_ai::small_action | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, u64, u64, game_ai::PositionEvalPurpose) -> game_core::PositioningScore | game-ai\src\small_action.rs:99 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 10 | push_candidate | game_ai::SmallActionLaneMinionPosition::push_candidate | in:game_ai | fn(&mut bumpalo::collections::vec::Vec< (u64, u64, i64)>, usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, &game_core::Entity, &game_core::PositioningScoreData, u64, u64, game_ai::PositionEvalPurpose, u64, u64, i64, game_core::LineType, i64) | game-ai\src\small_action\lane_minion.rs:439 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 11 | skill_avoid_effective | game_core::AthleteParameter::skill_avoid_effective | pub | fn(&game_core::AthleteParameter) -> usize | game-core\src\simulation\state\player.rs:296 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 12 | version | <game_ai::AgentVerHamster as game_core::AiAgent>::version | pub | fn(&game_ai::AgentVerHamster) -> usize | game-ai\src\lib.rs:428 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 592개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 13 | version | game_core::AiAgent::version | pub | fn(&Self/#0) -> usize | game-core\src\simulation\ai_interface.rs:503 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 592개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 14 | version | <game_core::Team as engine_core::spitz::SpitzDatable>::version | pub | fn() -> usize | game-core\src\data\team.rs:1696 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 592개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
</details>

⚠**미매칭 5개**: `clamp`, `llvm.abs.i64`, `llvm.smax.i64`, `llvm.umin.i64`, `push  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 6개는 **전부 다른 함수**라 싣지 않는다`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 3곳** (m11.ll:43538, m11.ll:43869, m11.ll:43878) · **형제 20개** (SmallActionLaneMinionPosition)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::SmallActionLaneMinionPosition as std::clone::Clone>::clone | pub | game-ai\src\small_action\lane_minion.rs:8 | True | fn(&game_ai::SmallActionLaneMinionPosition) -> game_ai::SmallActionLaneMinionPosition |
| 1 | <game_ai::SmallActionLaneMinionPosition as std::fmt::Debug>::fmt | pub | game-ai\src\small_action\lane_minion.rs:8 | True | fn(&game_ai::SmallActionLaneMinionPosition, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 2 | game_ai::SmallActionLaneMinionPosition::new | pub | game-ai\src\small_action\lane_minion.rs:139 | False | fn(&game_core::OperationData, usize, usize, i64, game_ai::PositionEvalPurpose) -> game_ai::SmallActionLaneMinionPosition |
| 3 | game_ai::SmallActionLaneMinionPosition::should_suppress_direct_minion_attack | pub | game-ai\src\small_action\lane_minion.rs:152 | False | fn(usize, &game_core::OperationData, &game_core::PlayerState, game_core::LineType, &game_core::Entity) -> bool |
| 4 | game_ai::SmallActionLaneMinionPosition::has_current_explicit_minion_action | in:game_ai | game-ai\src\small_action\lane_minion.rs:187 | False | fn(usize, &game_core::OperationData, &game_core::PlayerState, game_core::LineType, &game_core::Entity) -> bool |
| 5 | game_ai::SmallActionLaneMinionPosition::can_use_minion_action_now | in:game_ai | game-ai\src\small_action\lane_minion.rs:208 | False | fn(bool, std::option::Option<&game_core::Effect>, &game_core::Entity, &game_core::Entity) -> bool |
| 6 | game_ai::SmallActionLaneMinionPosition::direct_attack_stance | in:game_ai | game-ai\src\small_action\lane_minion.rs:212 | False | fn(&game_core::OperationData, &game_core::Entity, &game_core::Entity, u64) -> std::option::Option<(u64, u64)> |
| 7 | game_ai::SmallActionLaneMinionPosition::has_safe_minion_attack_stance | in:game_ai | game-ai\src\small_action\lane_minion.rs:226 | False | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, &game_core::Entity, game_core::LineType, u64) -> bool |
| 8 | game_ai::SmallActionLaneMinionPosition::is_safe_minion_attack_stance | in:game_ai | game-ai\src\small_action\lane_minion.rs:272 | False | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, &game_core::Entity, game_core::LineType, u64, u64, u64) -> bool |
| 9 | game_ai::SmallActionLaneMinionPosition::target_score | in:game_ai | game-ai\src\small_action\lane_minion.rs:294 | False | fn(usize, &game_core::OperationData, &game_core::PlayerState, &game_core::Entity, &game_core::Effect, &game_core::Entity, std::option::Option<&game_ai::MinionWaveSnapshot>, &mut rand::rngs::std::StdRng) -> std::option::Option<i64> |
| 10 | game_ai::SmallActionLaneMinionPosition::attack_range | in:game_ai | game-ai\src\small_action\lane_minion.rs:354 | False | fn(&game_core::Entity, &game_core::Entity) -> std::option::Option<u64> |
| 11 | game_ai::SmallActionLaneMinionPosition::local_position_score | in:game_ai | game-ai\src\small_action\lane_minion.rs:359 | False | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, u64, u64, game_ai::PositionEvalPurpose) -> i64 |
| 12 | game_ai::SmallActionLaneMinionPosition::lane_stance_risk | in:game_ai | game-ai\src\small_action\lane_minion.rs:380 | False | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, u64, u64) -> std::option::Option<i64> |
| 13 | game_ai::SmallActionLaneMinionPosition::push_candidate | in:game_ai | game-ai\src\small_action\lane_minion.rs:439 | False | fn(&mut bumpalo::collections::vec::Vec< (u64, u64, i64)>, usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, &game_core::Entity, &game_core::PositioningScoreData, u64, u64, game_ai::PositionEvalPurpose, u64, u64, i64, game_core::LineType, i64) |
| 14 | game_ai::SmallActionLaneMinionPosition::choose_goal | in:game_ai | game-ai\src\small_action\lane_minion.rs:493 | False | fn(&game_ai::SmallActionLaneMinionPosition, usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, &game_core::Entity, &game_core::PositioningScoreData, game_core::LineType) -> std::option::Option<(u64, u64, i64)> |
| 15 | game_ai::SmallActionLaneMinionPosition::get_input | in:game_ai | game-ai\src\small_action\lane_minion.rs:540 | False | fn(&mut game_ai::SmallActionLaneMinionPosition, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &mut game_core::DebugFrameData) -> std::option::Option<game_core::Input> |
| 16 | game_ai::SmallActionLaneMinionPosition::merge | in:game_ai | game-ai\src\small_action\lane_minion.rs:635 | False | fn(&mut game_ai::SmallActionLaneMinionPosition, usize, game_ai::SmallActionLaneMinionPosition) |
| 17 | game_ai::SmallActionLaneMinionPosition::get_action | in:game_ai | game-ai\src\small_action\lane_minion.rs:652 | True | fn(&game_ai::SmallActionLaneMinionPosition) -> game_core::SmallAction |
| 18 | game_ai::SmallActionLaneMinionPosition::is_end | in:game_ai | game-ai\src\small_action\lane_minion.rs:656 | False | fn(&game_ai::SmallActionLaneMinionPosition, &mut rand::rngs::std::StdRng, usize, &game_core::PlayerState, &game_core::OperationData) -> bool |
| 19 | game_ai::SmallActionLaneMinionPosition::near_move_complete | in:game_ai | game-ai\src\small_action\lane_minion.rs:685 | False | fn(&game_ai::SmallActionLaneMinionPosition, &game_core::Entity) -> bool |

**`open` 8건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | 이 명세는 지시 밖 추가분 — 지시의 exe 0xe25450 이 이 함수라는 판정(choose_goal 명세 unknown[0])에 따라 붙였다. 검증은 런타임 프로브(0xe25450 리턴 주소가 0xe248f0 안 3곳인지)로 | 4 |  |
| 1 | 미탐색 | reach 판정 파일(_next\reach\<rva>.reach.txt)이 이 함수엔 없다(rc_e254a2fe48e5 는 다른 해시) — 사장 판정은 손 독해: version 분기 0 · 조기 ret 6 경로 전부 입력 의존이라 살아있음 | 4 |  |
| 2 | 표기 불가 | 458 줄 `walls != 0 \|\| is_enemy_well_danger(...)` 의 한 줄 안 좌우 순서는 column 부재로 표기 불가 — IR 단락 순서(walls 먼저, 참이면 콜리 안 부름)는 확정 | 4 |  |
| 3 | 미탐색 | 377 줄 positioning_risk_value(small_action.rs:81) 인라인이 skill_avoid_effective 를 호출하고 결과(%96)를 버린다 — on_trajectory=false 상수 전달로 그 값이 쓰이는 분기가 접힌 것으로 추정(원본 정의는 범위 밖·미열람). 콜리가 readonly 라 부작용은 없으나 exe 에서 호출 자체는 남을 수 있음 | 5 |  |
| 4 | 표기 불가 | 365 줄 격자 안 판정을 `-4`/`-7` unsigned 비교로 접었다 — 소스가 `(xi+3-cx) as usize >= 7` 인지 `!(cx-3..=cx+3).contains(&xi)` 인지 표기 불가(동작 동일: \|xi-cx\|<=3 && \|yi-cy\|<=3) | 4 |  |
| 5 | 미탐색 | 콜리 계약만: is_enemy_well_danger(version, &PlayerState, x, y)->bool · is_near_line(&GameContext(64B), x, y, LineType)->bool · is_unnecessary_enemy_tower_position(version, &PlayerState, &OperationData, x, y)->bool · position_score_at_position(sret PositioningScore 56B, version, &PlayerState, &OperationData, &PositioningScoreData, x, y, purpose:i8) · AthleteParameter::positioning_effective/skill_avoid_effective(&self 744B)->i64 · lane_stance_risk(version, &PlayerState, &OperationData, &Entity, x:u64, y:u64)->Option<i64>(m11.ll:44468 · DI !66711) · bumpalo Vec::push(&mut Vec, &(u64,u64,i64)) | 4 |  |
| 6 | 미탐색 | walls 의 값 의미(0=통과 · 비0=?)는 MapDef 쪽 정본 미열람 — 이 함수는 `!= 0` 만 본다 | 3 |  |
| 7 | 미탐색 | `_docs\game_ai.txt` 에 push_candidate 관련 개발자 주석 0건 | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

