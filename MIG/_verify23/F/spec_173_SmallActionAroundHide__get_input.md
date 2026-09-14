---

### `173` SmallActionAroundHide::get_input — 대상 엔티티 주변 7×7 셀 중 대상 2.5셀 이내·적 시야 밖 셀을 Around 가치로 골라 숨을 목표를 정하고, 후보 없으면 대상에서 (visible_distance+30000) 떨어진 점으로 PathFinder 이동 입력 생성

| 항목 | 값 |
|---|---|
| id | `SmallActionAroundHide__get_input` |
| 심볼 | `_RNvMs_NtNtCshdEBA0ozCnw_7game_ai12small_action6aroundNtB4_21SmallActionAroundHide9get_input` |
| 소스 | `game-ai\src\small_action\around.rs:324` |
| IR | `m08.ll` 110434~111202행 |
| 경로·가시성 | `game_ai::SmallActionAroundHide::get_input` · **in:game_ai** |
| 계층 | 기타 |
| exe | `dc6ec0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r14` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&mut game_ai::SmallActionAroundHide, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData) -> std::option::Option<game_core::Input>
```

<details><summary>인자 7개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | *mut Option<Input>(32B) | None = i64 -1 @+0x0 | 4 |
| 1 | 1 | self | &mut SmallActionAroundHide(120B) | ★&mut. 읽기 +0x8 target(엔티티 id) · +0x10 goal_x · +0x18 goal_y · +0x75 path_finder 태그. 쓰기 = writes | 4 |
| 2 | 2 | version | usize | 본문 분기 없음(Positioning 과 달리 version>1 게이트 없음). 콜리·클로저 환경[0] 전달 | 4 |
| 3 | 3 | rnd | &mut StdRng(320B) | ★&mut — 본문 직접 store 0. positioning_window_pick · new_target · update_path · is_safe_recall 전달 | 4 |
| 4 | 4 | player | &PlayerState(2528B, readonly) | +0x930 team · +0x9c0 position · +0x180 info.parameter | 4 |
| 5 | 5 | data | &OperationData(24B, readonly) | +0x0 cache(game 팻포인터 · player_champion) · +0x8 context(pool · setting · map) | 4 |
| 6 | 6 | positioning_score | &PositioningScoreData(2760B, readonly) | +0xab8 cx · +0xac0 cy | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
SmallActionAroundHide::get_input(&mut self, version, rnd, player, data, ps) -> Option<Input>   [around.rs:324]
 target = game.get_entity_by_id(self.target)?          → None 이면 return None                  (L325, vtable+0x1f0)
 me = data.cache.player_champion[player.team][player.position]?  → None 이면 return None       (L327)
 (cx, cy) = (ps.cx, ps.cy) as i32 ; enemy = 1 - player.team                                     (L328~329, 347)
 candidates: bumpalo Vec<(i32 i, i32 j, i32 x, i32 y, i64 key)> in ctx.pool                     (L331)
 for i in 0..7: for j in 0..7:                                                                    (L333~334)
     ix = cx-3+i ; iy = cy-3+j ; x = ix*32000+16000 ; y = iy*32000+16000                        (L336)
     if ix<0 || iy<0 || ix>29 || iy>29 || map.walls[iy][ix] != 0: continue                       (L341)
     dist = distance(x, y, target.x, target.y)                                                    (L344)
     if dist > 79999: continue                                # 대상 2.5셀 밖                      (L347)
     if game.is_visible_cell(enemy, ix, iy): continue           # 적에게 보이는 셀                 (L347, vtable+0x100)
     value = positioning_cell_value_at(version, player, data, ps, x, y, Around)                   (L351)
     candidates.push((i, j, x, y, value*10000 + (dist as i32)/1000))                              (L353)
 if candidates.is_empty():                                                                        (L357)
     dx = me.x - target.x ; dy = me.y - target.y ; d = isqrt(dx²+dy²)     (d==0 → 패닉)            (L358~360)
     r = setting.visible_distance + 30000                                                          (L362)
     goal = Game::adjust_position(map, setting, target.x + r*dx/d, target.y + r*dy/d)   # 대상에서 r 만큼 내 쪽으로   (L362~364)
 else:
     candidates.sort_by_key(|c| -c.key)          # key 내림차순                                    (L368, aux)
     (lo‰, hi‰) = positioning_choice_window(version, player, athlete.positioning_min_range(), athlete.positioning_max_range(), false)   (L369~371)
     lo = min(len-1, len*lo‰/1000) ; hi = min(max(len*hi‰/1000, lo), len-1)                       (L372~373)
     window = candidates.into_iter().skip(lo).take(hi-lo+1).collect()                              (L375~377)
     pick = positioning_window_pick(version, rnd, player, data, &window)                          (L379)
     cur = positioning_score_at_position(…, self.goal_x, self.goal_y, Around) ; cur_val = cur.gain - cur.risk    (L381~382)
     new = positioning_score_at_position(…, pick.x, pick.y, Around) ; self.goal_gain = new.gain - new.risk        (L384~385, 무조건 기록)
     if cur_val >= new_val && goal_x < 960000 && goal_y < 960000 && |me - goal|² > 143999999:     (L386)
         goal = (self.goal_x, self.goal_y)      # 아직 12000 이상 남은 기존 목표 유지
     else: goal = (pick.x, pick.y)
 self.goal_x, self.goal_y = goal                                                                   (L393~394)
 tower_dodge = TowerDodgeContext::new_tower_avoid_v3(version, player, data, goal_x, goal_y)        (L395)
 if self.path_finder.is_none():                                                                    (L396)
     self.path_finder = PathFinder::new_target(rnd, ctx, version, "hide_around", me.x, me.y, goal_x, goal_y,
         |sx,sy,nx,ny| SmallActionAroundHide::check_cell(version, player, ps, data, &tower_dodge, sx,sy,nx,ny, self.goal_x, self.goal_y))   (L397; 환경 7칸)
     if self.path_finder.is_none(): drop(candidates); return None                                   (L402)
 pf.update_path(rnd, ctx, me.x, me.y, self.goal_x, self.goal_y, 같은 클로저(s0_0))                  (L404)
 safe = is_safe_recall(version, rnd, player, data, ps)      # 잎22 · 계약만                          (L408)
 input = pf.get_input(player, data, SafeMoveWithSkill::Safe, safe) ; drop(candidates) ; return Some(input)   (L408~409)

클로저 계약(관측, m03.ll 21357~24886 new_target 인스턴스 24682): `SmallActionAroundHide::check_cell`(around.rs:411 — 이 배치 밖 별도 함수, 11인자: version, &PlayerState, &PositioningScoreData, &OperationData, &TowerDodgeContext, sx,sy,nx,ny:usize, goal_x,goal_y:u64) -> PathVerdict. around::check_cell(잎 아님·이 배치 #1) 과는 다른 함수.
```

**`mem` 메모리 접근 33건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | OperationData | 0x0 | cache | r | 110481 | 4 | OK |  |
| 1 | AbstractGameWithCache | 0x0 | game.data_ptr / +0x8 vtable_ptr | r | 110482~110484 | 4 | OK |  |
| 2 | AbstractGame vtable | 0x1f0 | get_entity_by_id | r | divtable(정적 ExpectedGame vtable 기준). (game, self.target) → Option<&Entity> (110487~110489, L325) | 3 | 확인불가(vtable 슬롯) |  |
| 3 | AbstractGame vtable | 0x100 | is_visible_cell | r | (game, 1-team, ix, iy) -> bool (110553 `gep %25, 256`, 호출 111118, L347) | 4 | 확인불가(vtable 슬롯) |  |
| 4 | SmallActionAroundHide | 0x8 | target | r | 엔티티 id (110486) | 4 | OK |  |
| 5 | SmallActionAroundHide | 0x10 | goal_x | r | 110802(L381)·110899(L404) | 4 | OK |  |
| 6 | SmallActionAroundHide | 0x18 | goal_y | r | 110804 | 4 | OK |  |
| 7 | SmallActionAroundHide | 0x75 | path_finder@tag | r | +0x30+0x45 Option<PathFinder> 니치 2=None (110887·110934·110895) | 4 | OK |  |
| 8 | SmallActionAroundHide | 0x30 | path_finder 페이로드 | r | 110884 `gep %1, 48` — update_path/get_input &mut | 4 | OK |  |
| 9 | SmallActionAroundHide | 0x58 | path_finder.path Box / +0x60 planned_verdict Box | r | old 값 drop 경로(110938~110984) 에서만 로드 | 4 | OK |  |
| 10 | PlayerState | 0x930 | info.team | r | 110497; 적 팀 = 1-team (110552) | 4 | OK |  |
| 11 | PlayerState | 0x9c0 | info.position@tag | r | 110515 | 4 | OK |  |
| 12 | PlayerState | 0x180 | info.parameter | r | positioning_min/max_range self (110724) | 4 | OK |  |
| 13 | AbstractGameWithCache | 0x1e0 | player_champion[2][5] | r | null → return None (110517~110558) | 4 | OK |  |
| 14 | Entity(target) | 0x660 | x / +0x668 y | r | 대상 위치 (110550~110551 주소, 111106·110599 로드) | 4 | OK |  |
| 15 | Entity(me) | 0x660 | x / +0x668 y | r | 내 위치 (110598·110603, 110843, 110907, 110899) | 4 | OK |  |
| 16 | PositioningScoreData | 0xab8 | cx / +0xac0 cy | r | 후보 격자 중심 (110528·110532) | 4 | OK |  |
| 17 | OperationData | 0x8 | context | r | 110537 | 4 | OK |  |
| 18 | GameContext | 0x0 | pool | r | bumpalo Vec 할당자 (110538) | 4 | OK |  |
| 19 | GameContext | 0x8 | setting | r | 110654 (L362) | 4 | OK |  |
| 20 | GameSetting | 0x12f0 | visible_distance | r | 폴백 목표 반경 = visible_distance + 30000 (110655~110657) | 4 | OK |  |
| 21 | GameContext | 0x20 | map | r | walls(111096) · adjust_position 1번째 인자(110694) | 4 | OK |  |
| 22 | MapDef | 0x78 | walls[30][30] | r | walls[iy][ix] != 0 → 후보 제외 (111097~111101, L341) | 4 | OK |  |
| 23 | candidate(i32,i32,i32,i32,i64) | 0x8 | x / +0xc y | r | pick 원소 (110814~110819) | 4 | 확인불가(★모호: 동명 def_path 9개 [('game_core::Databa) |  |
| 24 | PositioningScore(56B 지역) | 0x0 | risk / +0x10 gain | r | gain - risk (110809~110811, 110828~110830) | 4 | OK |  |
| 25 | SmallActionAroundHide(self) | 0x10 | goal_x | w | 110715 (L393). 대상·챔피언 존재 시 항상 기록 | 4 | OK | 후보 없음 → adjust_position(폴백점).0 / 유지 → 기존 / 선택 → pick.x |
| 26 | SmallActionAroundHide(self) | 0x18 | goal_y | w | 110717 (L394) | 4 | OK | 위와 짝 |
| 27 | SmallActionAroundHide(self) | 0x20 | goal_gain | w | 110833 (L385) — ★후보가 있으면 유지/선택과 무관하게 pick 의 값을 기록. 후보 없음 경로에선 미기록 | 4 | OK | positioning_score_at_position(pick.x, pick.y, Around).gain - .risk |
| 28 | SmallActionAroundHide(self) | 0x30 | path_finder | w | None 일 때만. 직전 old Some 이면 Box 2개 dealloc(110938~110984, 진입 조건상 사장) | 4 | OK | PathFinder::new_target(rnd, ctx, version, "hide_around"(11B), me.x, me.y, goal_x, goal_y, closure s_0{version,player,ps,data,&tower_dodge,&self.goal_x,&self.goal_y}) 72B memcpy (110891, L397) |
| 29 | SmallActionAroundHide(self) | 0x30 | path_finder(콜리 경유) | w | path_finder 계층 미탐색 | 4 | OK | update_path(&mut)(111009) · get_input(&mut)(111039) |
| 30 | bumpalo Vec(지역 %20, pool) | 0x0 | candidates(ptr/+0x8 bump/+0x10 cap/+0x18 len) | w | pool 소비. drop 111016·111047 | 4 | 확인불가(tcx 사전에 타입 없음) | push (i, j, x, y, key) 24B (111179~111190) |
| 31 | Option<Input>(sret) | 0x0 | tag/Input | w |  | 4 | 확인불가(tcx 사전에 타입 없음) | -1 (110502·110558·111013) / 32B memcpy (111043) |
| 32 | StdRng(rnd) | 0x0 | rng 상태 | w | 본문 직접 store 없음 | 4 | OK | positioning_window_pick·new_target·update_path·is_safe_recall 내부 |

**`consts` 상수 21건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 7 | 333 | 임계 | 후보 격자 7×7 (i,j in 0..7) (110568·111069) | 4 |
| 1 | -3 | 333 | 계수 | ix = cx-3+i, iy = cy-3+j (110547·110548) | 4 |
| 2 | 32000 | 336 | 계수 | 셀→월드 x = ix*32000+16000 (i32 산술) | 4 |
| 3 | 16000 | 336 | 계수 | 셀 중심 | 4 |
| 4 | 29 | 341 | 임계 | ix>29 \|\| iy>29 → 제외 (110591·111090) | 4 |
| 5 | 0 | 341 | 임계 | walls[iy][ix]==0 만 후보(111101); (ix\|iy)<0 제외(111088). 또한 is_empty(110575)·isqrt 결과 0 → div_by_zero 패닉(110669) | 4 |
| 6 | 79999 | 347 | 임계 | distance(cell, target) > 79999 → 제외 = **대상과 80000(2.5셀) 이내**만 후보 (111113) | 4 |
| 7 | 1 | 347 | 길이 | 적 팀 = 1 - my_team (110552 `sub nuw nsw i64 1, %34`) → is_visible_cell(enemy, ix, iy) true 면 제외. 또한 sort len==1 생략(110629)·take 길이 hi-lo+1(110773)·SafeMoveWithSkill::Safe(111039) | 4 |
| 8 | 5 | 351 | 센티널 | PositionEvalPurpose::Around 메모리 태그 5(니치, idx 3) — positioning_cell_value_at(111125) · positioning_score_at_position(110805·110822) | 4 |
| 9 | 10000 | 353 | 계수 | 정렬 키 = value*10000 + (dist as i32)/1000 (111130~111134) — 가치 우선, 동가치면 대상에서 먼 셀 우선(dist/1000 ∈ 0..79) | 4 |
| 10 | 1000 | 353 | 계수 | dist/1000 타이브레이커(111132). 또한 choice_window 천분율(110743·110753) | 4 |
| 11 | 30000 | 362 | 계수 | 폴백 목표 반경 r = setting.visible_distance + 30000 (110657) | 4 |
| 12 | -1 | 362 | 태그 | sdiv 오버플로 검사(isqrt == -1 && 분자 == i64::MIN → div_overflow 패닉, 110663~110664·110678). 또한 Option<Input>::None 태그·len-1 상한(110744) | 4 |
| 13 | -9223372036854775808 | 362 | 태그 | i64::MIN — sdiv 오버플로 검사 상수(110664·110678). 판정값 아님 | 4 |
| 14 | 21 | 368 | 임계 | std 정렬 임계(len<21 insertion, 110634). 판정값 아님 | 4 |
| 15 | 960000 | 386 | 임계 | goal_x/goal_y < 960000(격자 안) 일 때만 현 목표 유지 후보 (110835·110837) | 4 |
| 16 | 143999999 | 386 | 임계 | 12000²-1 — \|me-goal\|² > 이 값(= 거리 ≥ 12000) 이고 cur_val ≥ new_val 이면 현 목표 유지, 아니면 pick 로 교체 (110867 `icmp ugt`) | 4 |
| 17 | 11 | 397 | 길이 | "hide_around" key 길이 (@anon…121, 110924) | 4 |
| 18 | 2 | 396 | 센티널 | Option<PathFinder>::None 니치 태그(110887 등). player_champion 팀 상한 2 도 같은 리터럴 | 4 |
| 19 | 1120 | 397 | 미상 | old path Box dealloc 크기 (110962). 판정값 아님 | 4 |
| 20 | 70 | 397 | 미상 | old planned_verdict Box dealloc (110984). 판정값 아님 | 4 |

**`knobs` 조정점 6건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 숨을 셀의 대상 거리 상한 | around.rs:347 | 79999 (2.5셀) | 올리면 대상에서 더 먼 수풀/시야 밖 셀도 후보 — 매복 위치가 멀어짐 | 4 | 기존 |
| 1 | 적 시야 제외 | around.rs:347 (is_visible_cell(1-team)) | 적 팀 가시 셀 제외 | 끄면 보이는 셀에도 숨으려 함(은신 의미 상실) | 4 | 기존 |
| 2 | 정렬 키 타이브레이커 | around.rs:353 | value*10000 + dist/1000 | 10000 을 키우면 거리 항이 사실상 무시, 줄이면 거리가 가치를 뒤집을 수 있음(현재 dist 항 최대 79 < 10000 이라 순수 타이브레이커) | 4 | 기존 |
| 3 | 폴백 목표 반경 | around.rs:362 | visible_distance + 30000 | 후보 셀이 없을 때 대상에서 유지할 거리 — 줄이면 시야 안으로 들어감 | 4 | 기존 |
| 4 | 목표 유지 최소 잔여거리 | around.rs:386 | 143999999 (12000²-1) | 올리면 목표 근처에서 더 일찍 새 pick 로 갈아탐 | 4 | 기존 |
| 5 | 후보 격자·선택 창 | around.rs:333~336 / 369~373 | 7×7 / 선수 min·max range 천분율 | Positioning 과 동일 기구 | 4 | 기존 |

<details><summary>`callees` 피호출자 34건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | adjust_position | game_core::Game::adjust_position | pub | fn(&game_core::MapDef, &game_core::GameSetting, i64, i64) -> (u64, u64) | game-core\src\simulation\game.rs:818 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | check_cell | game_ai::small_action::around::check_cell | in:game_ai::small_action::around | fn(usize, &game_core::PlayerState, &game_core::PositioningScoreData, &game_core::OperationData, &game_ai::TowerDodgeContext, game_ai::AroundBushOutlineType, usize, usize, usize, usize, usize, usize) -> game_ai::PathVerdict | game-ai\src\small_action\around.rs:1256 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 2 | check_cell | game_ai::SmallActionAroundHide::check_cell | in:game_ai | fn(usize, &game_core::PlayerState, &game_core::PositioningScoreData, &game_core::OperationData, &game_ai::TowerDodgeContext, usize, usize, usize, usize, u64, u64) -> game_ai::PathVerdict | game-ai\src\small_action\around.rs:411 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 3 | distance | game_core::utils::distance | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:12 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 5 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 6 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 7 | get_input | game_ai::PathFinder::get_input | pub | fn(&mut game_ai::PathFinder, &game_core::PlayerState, &game_core::OperationData, game_ai::SafeMoveWithSkill, bool) -> game_core::Input | game-ai\src\path_finder.rs:856 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 8 | goal | game_ai::plan_legacy::types::BigPlan::goal | pub | fn(&game_ai::plan_legacy::types::BigPlan) -> game_core::BigGoal | game-ai\src\plan_legacy\types.rs:132 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 16개 중 상위 3개 |
| 9 | goal | game_ai::plan_legacy::old::BattlePlan::goal | pub | fn(&game_ai::plan_legacy::old::BattlePlan) -> game_core::BigGoal | game-ai\src\plan_legacy\old\battle.rs:341 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 16개 중 상위 3개 |
| 10 | goal | game_ai::plan_legacy::old::SinglePlanLine::goal | pub | fn(&game_ai::plan_legacy::old::SinglePlanLine) -> game_core::BigGoal | game-ai\src\plan_legacy\old\single_line.rs:19 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 16개 중 상위 3개 |
| 11 | is_empty | game_core::PatchSetting::is_empty | pub | fn(&game_core::PatchSetting) -> bool | game-core\src\patch_engine.rs:853 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 12 | is_empty | game_core::ModAiRegistry::is_empty | pub | fn(&game_core::ModAiRegistry) -> bool | game-core\src\mod_ai.rs:15 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 13 | is_empty | game_core::PublicChampionEvidence::is_empty | pub | fn(&game_core::PublicChampionEvidence) -> bool | game-core\src\banpick\public_evidence.rs:96 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 14 | is_safe_recall | game_ai::small_action::cast::is_safe_recall | in:game_ai | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData) -> bool | game-ai\src\small_action\cast.rs:304 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 15 | is_visible_cell | game_core::AbstractGame::is_visible_cell | pub | fn(&Self/#0, usize, usize, usize) -> bool | game-core\src\simulation.rs:126 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 16 | is_visible_cell | <game_core::Game as game_core::AbstractGame>::is_visible_cell | pub | fn(&game_core::Game, usize, usize, usize) -> bool | game-core\src\simulation\game.rs:1804 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 17 | is_visible_cell | <game_core::SingleLaneGame as game_core::AbstractGame>::is_visible_cell | pub | fn(&game_core::SingleLaneGame, usize, usize, usize) -> bool | game-core\src\simulation\game.rs:3869 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 18 | isqrt | game_core::utils::isqrt | pub | fn(i64) -> i64 | game-core\src\utils.rs:85 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 19 | new_target | game_ai::PathFinder::new_target | pub | fn(&mut rand::rngs::std::StdRng, &game_core::GameContext, usize, &str, u64, u64, u64, u64, impl Fn(usize, usize, usize, usize) -> PathVerdict/#0) -> game_ai::PathFinder | game-ai\src\path_finder.rs:66 | True | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 20 | new_tower_avoid_v3 | game_ai::TowerDodgeContext::new_tower_avoid_v3 | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, u64, u64) -> game_ai::TowerDodgeContext | game-ai\src\path_finder.rs:1205 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 21 | positioning_cell_value_at | game_ai::small_action::positioning_cell_value_at | in:game_ai::small_action | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, u64, u64, game_ai::PositionEvalPurpose) -> i64 | game-ai\src\small_action.rs:104 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 22 | positioning_choice_window | game_ai::small_action::positioning_choice_window | in:game_ai::small_action | fn(usize, &game_core::PlayerState, usize, usize, bool) -> (usize, usize) | game-ai\src\small_action.rs:43 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 23 | positioning_max_range | game_core::AthleteParameter::positioning_max_range | pub | fn(&game_core::AthleteParameter) -> usize | game-core\src\simulation\state\player.rs:312 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 24 | positioning_min_range | game_core::AthleteParameter::positioning_min_range | pub | fn(&game_core::AthleteParameter) -> usize | game-core\src\simulation\state\player.rs:306 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 25 | positioning_score_at_position | game_ai::small_action::positioning_score_at_position | in:game_ai::small_action | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, u64, u64, game_ai::PositionEvalPurpose) -> game_core::PositioningScore | game-ai\src\small_action.rs:99 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 26 | positioning_window_pick | game_ai::small_action::positioning_window_pick | in:game_ai::small_action | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &[T/#0]) -> &T/#0 | game-ai\src\small_action.rs:66 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 27 | push | <game_core::Staff as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::staff | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\staff.rs:38 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 28 | push | <game_core::Athlete as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:434 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 29 | push | <game_core::Contract as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:2498 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 30 | team | game_view::ClientData::team | pub | fn(&game_view::ClientData, usize) -> std::option::Option<std::cell::Ref< game_core::Team>> | game-view\src\logic\client\data.rs:1651 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 31 | team | game_view::ClientDatabase::team | pub | fn(&game_view::ClientDatabase, usize) -> std::option::Option<&game_core::Team> | game-view\src\logic\client\data.rs:968 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 32 | team | game_core::PlayerAiContext::<'a, 'b, 'r>::team | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:583 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 33 | update_path | game_ai::PathFinder::update_path | pub | fn(&mut game_ai::PathFinder, &mut rand::rngs::std::StdRng, &game_core::GameContext, u64, u64, u64, u64, impl Fn(usize, usize, usize, usize) -> PathVerdict/#0) | game-ai\src\path_finder.rs:631 | True | False | 3 | IR 호출 심볼 일치(토큰 경계) |
</details>

⚠**미매칭 13개**: `Take`, `__rust_dealloc`, `collect`, `continue`, `driftsort_main`, `drop  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `insertion_sort_shift_left`, `pool`, `reserve_internal_or_panic`, `risk`, `skip`, `sort_by_key`, `take`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m11.ll:43027) · **형제 10개** (SmallActionAroundHide)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | game_ai::SmallActionAroundHide::check_cell::HIDE_NEAREST::__rust_std_internal_init_fn | in:game_ai::small_action::around | /rustc/23a3312d92a1c4ba0373f1e25277be20ba8bb28c/library\std\src\sys\thread_local\native\mod.rs:85 | False | fn() -> std::cell::RefCell<(u64, usize, usize, std::collections::HashMap<(u8, u8), std::option::Option<usize>, ahash::random_state::RandomState, std::alloc::Global>)> |
| 1 | <game_ai::SmallActionAroundHide as std::clone::Clone>::clone | pub | game-ai\src\small_action\around.rs:299 | True | fn(&game_ai::SmallActionAroundHide) -> game_ai::SmallActionAroundHide |
| 2 | <game_ai::SmallActionAroundHide as std::fmt::Debug>::fmt | pub | game-ai\src\small_action\around.rs:299 | True | fn(&game_ai::SmallActionAroundHide, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 3 | game_ai::SmallActionAroundHide::new | pub | game-ai\src\small_action\around.rs:312 | False | fn(usize, &mut rand::rngs::std::StdRng, &game_core::OperationData, &game_core::PlayerState, usize, usize) -> game_ai::SmallActionAroundHide |
| 4 | game_ai::SmallActionAroundHide::get_input | in:game_ai | game-ai\src\small_action\around.rs:324 | False | fn(&mut game_ai::SmallActionAroundHide, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData) -> std::option::Option<game_core::Input> |
| 5 | game_ai::SmallActionAroundHide::check_cell | in:game_ai | game-ai\src\small_action\around.rs:411 | False | fn(usize, &game_core::PlayerState, &game_core::PositioningScoreData, &game_core::OperationData, &game_ai::TowerDodgeContext, usize, usize, usize, usize, u64, u64) -> game_ai::PathVerdict |
| 6 | game_ai::SmallActionAroundHide::merge | in:game_ai | game-ai\src\small_action\around.rs:475 | False | fn(&mut game_ai::SmallActionAroundHide, usize, game_ai::SmallActionAroundHide) |
| 7 | game_ai::SmallActionAroundHide::get_action | in:game_ai | game-ai\src\small_action\around.rs:490 | True | fn(&game_ai::SmallActionAroundHide) -> game_core::SmallAction |
| 8 | game_ai::SmallActionAroundHide::is_end | in:game_ai | game-ai\src\small_action\around.rs:493 | False | fn(&game_ai::SmallActionAroundHide, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bool |
| 9 | game_ai::SmallActionAroundHide::near_move_complete | in:game_ai | game-ai\src\small_action\around.rs:500 | False | fn(&game_ai::SmallActionAroundHide, &game_core::Entity) -> bool |

**`open` 6건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 재료 부재 | 정렬 키에 dist/1000 을 **더하는** 것이 의도(동가치면 먼 셀 우선)인지 실수인지 — IR 사실만 기록(111130~111134). 소스 주석은 재료 부재(_docs 에 없음) | 4 |  |
| 1 | 미탐색 | self.goal_gain(+0x20) 이 '유지' 분기에서도 pick 의 값으로 덮이는 것이 의도인지 — IR 사실(110833 은 분기 전 무조건 store) | 4 |  |
| 2 | 미탐색 | Game::adjust_position(&MapDef, &GameSetting, x, y)->(u64,u64) 내부(game_core 경계) — 맵 경계/벽 보정으로 추정, 미탐색 | 5 |  |
| 3 | 미탐색 | SmallActionAroundHide::check_cell(around.rs:411, m08.ll 110007~) 내부 — 이 배치 밖. HIDE_NEAREST TLS 캐시(m08 114410·m11 41134)를 쓴다는 심볼 관측만 | 4 |  |
| 4 | 미탐색 | positioning_window_pick 의 rng 사용 여부·선택 규칙 — small_action 계층 미탐색 | 4 |  |
| 5 | 미탐색 | vtable 슬롯 +0x1f0/+0x100 이름은 정적 ExpectedGame vtable(divtable) 기준 — 런타임 dyn 객체가 다른 구현이어도 트레이트 메서드 순서는 같음 | 3 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

