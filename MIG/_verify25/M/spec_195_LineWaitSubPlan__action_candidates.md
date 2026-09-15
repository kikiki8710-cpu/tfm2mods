---

### `195` LineWaitSubPlan::action_candidates — 라인 대기(LineWait) 서브플랜 후보 — 기지 포지셔닝(타워/앞미니언 뒤 180000) + 도주 + 전투/라인미니언/소환/타워공격/구조물스킬 후보를 모은 뒤 v30 타워 어그로 위험 후보를 retain 으로 제거

| 항목 | 값 |
|---|---|
| id | `line_wait__LineWait__action_candidates` |
| 심볼 | `_RNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy8sub_plan9line_waitNtB2_15LineWaitSubPlan17action_candidates` |
| 소스 | `game-ai\src\plan_legacy\sub_plan\line_wait.rs:144` |
| IR | `m15.ll` 20693~22514행 |
| 경로·가시성 | `game_ai::plan_legacy::sub_plan::LineWaitSubPlan::action_candidates` · **pub** |
| 계층 | 기타 |
| exe | `eb43a0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r16` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&mut game_ai::plan_legacy::sub_plan::LineWaitSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay>
```

**TLS 메모 절(`sig.tls` · 정정 경로 `/specs[195]/sig/tls/<키>`)**

- `name`: (직접 접점 없음) — 콜리 경유만
- `role`: 소비자(간접)
- `key`: 본문 LocalKey::with 0건. 콜리 순서: ①LineType::get_start_position(L22<21<147, twin_towers 원소마다) ②MapDef::lane_path(L59<46<147, behind 경로만) ③Game::adjust_position(L83<46<147) ④battle_action(L149, 무조건) → INTER_CTX·HP_VALUE_MEMO 등 ⑤line_minion_action_candidates(L150) ⑥attack_summon_action(L151) ⑦AbstractGameWithCache::iter_towers(L105<152) ⑧Effect::range_adjust(L113<152) ⑨v22_lane_tower_pressure_attack_allowed(L119<152, 사거리 통과 시) ⑩attack_structure_skill_action(L153) ⑪v30_line_champion_action_tower_aggro_risk(L154, 후보 원소마다). ⚠position_score_at_position / position_eval_at 직접 호출 없음(serpen_check·jungle 과 다름) — 후보의 purpose=LaneSafe 는 하류 평가기(evaluation_position)가 소비
- `layout`: 해당 없음
- `invalidation`: 해당 없음
- `call_conditions`: ④⑤⑥⑩⑪은 무조건. ⑦~⑨는 champ Some 일 때. ⑨는 nearest_enemy_tower Some && attack_effect Some && dist² ≤ max_dist² 일 때만

<details><summary>인자 7개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | bumpalo::Vec<SmallActionPlay> (32B) | writeonly sret. +0 ptr · +8 bump · +0x10 cap · +0x18 len. 원소 184B | 4 |
| 1 | 1 | self | &mut LineWaitSubPlan (1B) | DI 타입 ref_mut. IR 속성 noalias 만(readonly·captures(none) 없음 — 클로저 env 에 &self 가 캡처돼 get_start_position 의 self 로 전달). ★store 0건 = &mut 이지만 쓰기 없음(전수 grep: `store …, ptr %1` 0). 유일 필드 line:LineType @+0 | 4 |
| 2 | 2 | version | usize | alloca %30 에 저장(retain 클로저 env 가 &version 캡처). 분기 없음. line_minion_action_candidates · v22 · v30 · Around::new 에 전달. ⚠battle_action 에는 poison 전달(_version) | 4 |
| 3 | 3 | rnd | &mut StdRng (320B) | Around::new(_rnd) · AroundPosition::new 에 전달. ⚠battle_action 에는 poison(_rnd) | 4 |
| 4 | 4 | player | &PlayerState (2528B) | readonly | 4 |
| 5 | 5 | data | &OperationData (24B) | cache@0 · context@8 · blackboard@0x10 모두 읽음 | 4 |
| 6 | 6 | _parameter | &ScoreParameter (5384B) | DI 이름 `_parameter` · captures(none) · 본문 사용 0건(미사용) | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn action_candidates(&mut self, version, rnd, player, data, _parameter) -> Vec<SmallActionPlay>
L145 bump = data.context.pool; res = Vec::new_in(bump)
L147 res.extend(self.base_positioning(version, rnd, player, data)):   // line_wait.rs:17~53 인라인
  L17   sub = Vec::new_in(bump); purpose = PositionEvalPurpose::LaneSafe(10)
  L19   team = player.info.team (<2); optb: Option<&Entity> = match self.line { Top => cache.top_tower[team].or(top_tower2[team]), Mid => mid_tower.or(mid_tower2), Bottom => bottom_tower.or(bottom_tower2) }
  L20~25 optb = optb.or( cache.twin_towers[team].iter().min_by_key(|t| dist²(t, self.line.get_start_position(context.setting, team))) )   // 빈 Vec 이면 None · 동률 첫 원소
  L26   optb = optb.or(cache.nexus[team])
  L27   nearest_tower = optb.unwrap()
  L29   nexus = cache.nexus[team].unwrap()
  L31   ms = data.blackboard[team].{top|mid|bottom}_minion_state (self.line 기준)
  L32   fm = ms.front_minion.and_then(|id| cache.game.get_entity_by_id(id))
  L34   if fm.is_none() || dist²(fm, nexus) < dist²(nearest_tower, nexus) {   // 앞미니언이 타워보다 넥서스에 가까움(=타워 뒤)
  L35     sub.push(Around(SmallActionAround::new(version, rnd, data, player, target=nearest_tower.id, 5).with_position_eval_purpose(LaneSafe)))   // 태그 5
  L41   } else if dist²(fm, nearest_tower) < 32400000001 (180000²+1) {
  L42     sub.push(Around(SmallActionAround::new(…, nearest_tower.id, 5).with_position_eval_purpose(LaneSafe)))
        } else {
  L46     (bx,by) = behind(fm, 180000):   // line_wait.rs:55~95 인라인
    L56     team = fm.player_team() (Neutral → unwrap_failed); L57 fm.ty must be Minion(1) else panic
    L59     path: [(u64,u64);7] = context.map.lane_path(fm.minion.line, team)
    L62~63  moved = Σ_{i=1..<move_index} distance(path[i-1], path[i])   (move_index<7 아니면 panic_bounds_check)
    L66     moved += distance(path[move_index], fm.pos)
    L68     moved = moved.saturating_sub(180000)
    L72~73  (x,y) = path[0]
    L76     for i in 1..7 { seg = distance((x,y), path[i]);
    L77       if seg > moved { L78~82 nx = x + (path[i].x - x)*moved/isqrt(dx²+dy²); ny = y + (path[i].y - y)*moved/isqrt(…); (isqrt==0 → div_by_zero panic) L83 return game::adjust_position(map, setting, nx, ny) }
    L88       moved -= seg; (x,y) = path[i] }
    L94     return path[6]
  L47     sub.push(AroundPosition(SmallActionAroundPosition::new(rnd, data, bx, by, 5).with_position_eval_purpose(LaneSafe)))   // 암묵 variant
        }
  L52   sub )
L148 res.push(RunAway(SmallActionRunAway::new(data, player, 5)))   // 태그 3
L149 res.extend(battle_action(_version=poison, _rnd=poison, player, data, _end_delay=poison))
L150 res.extend(line_minion_action_candidates(version, data, player, self.line))
L151 res.extend(attack_summon_action(player, data))
L152 res.extend(self.attack_tower_action(version, player, data)):   // line_wait.rs:104~121 인라인 · Option<SmallActionPlay>
  L104  champ = cache.player_champion[team][pos]? (None → None)
  L105  enemy_team = 1 - team
  L107  tower = cache.iter_towers(enemy_team)   // [top,top2,mid,mid2,bottom,bottom2][enemy] flatten ⧺ twin_towers[enemy]
              .filter(|t| t.can_target && t.block_target_tick == 0).min_by_key(|t| dist²(t, champ))? (None → None)
  L109  move_speed = champ.stat_cached.move_speed
  L110  atk = champ.attack_effect.as_ref()? (None → None)
  L112  dist = dist²(tower, champ)
  L113  max = atk.range + champ.stat_buff_cached.range + (champ.level-1)*atk.growth_range + atk.range_adjust(champ, tower) + champ.radius()
  L114  max += move_speed*30 + tower.radius()
  L115  if dist > max² → None
  L119  if !v22_lane_tower_pressure_attack_allowed(version, data, player, tower) → None
  L120  Some(Attack(SmallActionAttack::new(data, tower.id))) )   // 태그 15
L153 res.extend(attack_structure_skill_action(player, data))
L154 res.retain(|a| !v30_line_champion_action_tower_aggro_risk(version, data, player, a))   // aux m01.ll:20583/20608 — true 면 제거
L156 return res
```

**`mem` 메모리 접근 49건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | LineWaitSubPlan(self) | 0x0 | line | r | LineType 0=Top 1=Mid 2=Bottom. switch 로 타워/미니언상태 선택 · line_minion_action_candidates 인자 · get_start_position self | 4 | OK |  |
| 1 | OperationData | 0x0 | cache | r | &AbstractGameWithCache | 4 | OK |  |
| 2 | OperationData | 0x8 | context | r | &GameContext | 4 | OK |  |
| 3 | OperationData | 0x10 | blackboard | r | &[Blackboard;2] → [team] (L31<147) | 4 | OK |  |
| 4 | GameContext | 0x0 | pool | r | &Bump (L145 · L17<147) | 4 | OK |  |
| 5 | GameContext | 0x8 | setting | r | &GameSetting → get_start_position · adjust_position 인자 | 4 | OK |  |
| 6 | GameContext | 0x20 | map | r | &MapDef → lane_path · adjust_position (gep 32) | 4 | OK |  |
| 7 | PlayerState | 0x930 | info.team | r | gep 2352 (<2 아니면 panic_bounds_check, 라인별 3곳 + L104) | 4 | OK |  |
| 8 | PlayerState | 0x9c0 | info.position@tag | r | gep 2496 (L104<152) | 4 | OK |  |
| 9 | AbstractGameWithCache | 0x180 | top_tower | r | gep 384 [team] (line==Top) | 4 | OK |  |
| 10 | AbstractGameWithCache | 0x190 | top_tower2 | r | gep 400 [team] | 4 | OK |  |
| 11 | AbstractGameWithCache | 0x1a0 | mid_tower | r | gep 416 [team] (line==Mid) | 4 | OK |  |
| 12 | AbstractGameWithCache | 0x1b0 | mid_tower2 | r | gep 432 [team] | 4 | OK |  |
| 13 | AbstractGameWithCache | 0x1c0 | bottom_tower | r | gep 448 [team] (line==Bottom) | 4 | OK |  |
| 14 | AbstractGameWithCache | 0x1d0 | bottom_tower2 | r | gep 464 [team] | 4 | OK |  |
| 15 | AbstractGameWithCache | 0x130 | twin_towers | r | gep 304 [team] bumpalo Vec<&Entity>(ptr@0 len@0x18) → min_by_key (L20~25<147) | 4 | OK |  |
| 16 | AbstractGameWithCache | 0x170 | nexus | r | gep 368 [team] (L26·L29<147) | 4 | OK |  |
| 17 | AbstractGameWithCache | 0x1e0 | player_champion | r | gep 480 [team][pos] (L104<152, None → 타워공격 없음) | 4 | OK |  |
| 18 | AbstractGameWithCache | 0x0 | game | r | &dyn AbstractGame → vtable+0x1f0 get_entity_by_id(front_minion id) (L32<147) | 4 | OK |  |
| 19 | AbstractGame vtable | 0x1f0 | get_entity_by_id | r | gep 496 | 4 | 확인불가(vtable 슬롯) |  |
| 20 | Blackboard | 0x0 | top_minion_state.front_minion | r | Option<usize>: tag@+0 값@+8. line==Mid → +0x28(gep 40) mid_minion_state · Bottom → +0x50(gep 80) bottom_minion_state (L31<147) | 4 | OK |  |
| 21 | Entity | 0x660 | x | r | gep 1632 | 4 | OK |  |
| 22 | Entity | 0x668 | y | r | gep 1640 | 4 | OK |  |
| 23 | Entity | 0x5c0 | id | r | gep 1472 (Around target = nearest_tower.id · Attack target = enemy tower id) | 4 | OK |  |
| 24 | Entity | 0x0 | team@tag | r | TeamType: 0=Player 1=Neutral. behind(): Neutral 이면 unwrap_failed (entity.rs:1136 player_team) | 4 | OK |  |
| 25 | Entity | 0x8 | team@Player.0 | r | 팀 번호 → lane_path 인자 | 4 | OK |  |
| 26 | Entity | 0x68 | ty@tag | r | gep 104 ==1(Minion) 아니면 panic(line_wait.rs:95) | 4 | OK |  |
| 27 | Entity | 0x108 | ty@Minion.info.move_index | r | gep 264. 경로 인덱스(<7 아니면 panic_bounds_check) | 4 | OK |  |
| 28 | Entity | 0x11a | ty@Minion.info.line@tag | r | gep 282 → lane_path 인자 | 4 | OK |  |
| 29 | Entity | 0x6b9 | can_target | r | gep 1721 (적 타워 필터) | 4 | OK |  |
| 30 | Entity | 0x6a0 | block_target_tick | r | gep 1696 ==0 (적 타워 필터) | 4 | OK |  |
| 31 | Entity | 0x640 | stat_cached.move_speed | r | gep 1600 ×30 | 4 | OK |  |
| 32 | Entity | 0x4c0 | attack_effect@tag | r | gep 1216. -1=None → 타워공격 없음 (L110<152) | 4 | OK |  |
| 33 | Entity | 0x490 | attack_effect@Some.0 | r | gep 1168 &Effect → range_adjust | 4 | OK |  |
| 34 | Entity | 0x4a0 | attack_effect@Some.0.range | r | gep 1184 | 4 | OK |  |
| 35 | Entity | 0x4a8 | attack_effect@Some.0.growth_range | r | gep 1192 | 4 | OK |  |
| 36 | Entity | 0x5c8 | level | r | gep 1480 (level-1)*growth_range | 4 | OK |  |
| 37 | Entity | 0x438 | stat_buff_cached.range | r | gep 1080 | 4 | OK |  |
| 38 | Entity | 0x470 | stat_buff_cached.radius_mult | r | gep 1136 (챔프·타워) | 4 | OK |  |
| 39 | Entity | 0x680 | radius | r | gep 1664 (챔프·타워) | 4 | OK |  |
| 40 | SmallActionAround | 0x80 | position_eval_purpose | r | gep 128 store 10(LaneSafe) — with_position_eval_purpose 인라인 (L36·L43<147) | 4 | OK |  |
| 41 | SmallActionAroundPosition | 0xb0 | position_eval_purpose | r | gep 176 store 10 (L48<147) | 4 | OK |  |
| 42 | lane_path sret [(u64,u64);7] | 0x0 | path[i] | r | stride 16, i∈0..7 (gep 0/8/16/…/104) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 43 | LineWaitSubPlan(self) | (없음) |  | w | ★&mut self 이지만 본문·aux 전 범위에 self 로의 store 0건(m15.ll 20693~22514 `store …, ptr %1` grep 0). initializes 속성 없음 | 4 | 확인불가(오프셋 파싱 실패) | - |
| 44 | (sret) Vec<SmallActionPlay> | 0x0 -> ptr/bump/cap/len |  | w | m15.ll 20730~20738 초기화 · 21714/21781/21800/21813/22495 extend · 22467 원소 memcpy · 22511 sret memcpy | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) | new_in(bump) · push(RunAway L148 · Attack L152) · extend×5 · retain 이 len 재기록(m01.ll:20524 len=0 후 20672 new_len) · 32B memcpy |
| 45 | (지역) base_positioning res Vec (%20) | (alloca) |  | w | 언와인드 시 drop_glue(m15.ll:20813) | 4 | 확인불가(오프셋 파싱 실패) | Around/AroundPosition 1개 push 후 L147 extend |
| 46 | SmallActionAround(지역 %16/%18) | 0x80 |  | w | m15.ll:21601 · 21652 (with_position_eval_purpose 인라인) | 4 | OK | i8 10 (LaneSafe) |
| 47 | SmallActionAroundPosition(지역 %15) | 0xb0 |  | w | m15.ll:21554 | 4 | OK | i8 10 (LaneSafe) |
| 48 | StdRng(rnd) | (콜리) |  | w | 본문 직접 쓰기 없음. battle_action 에는 poison 전달(안 씀) | 4 | 확인불가(오프셋 파싱 실패) | Around::new · AroundPosition::new 이 소비 |

**`consts` 상수 11건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 32400000001 | 41 | 임계 | 180000² + 1 — 앞미니언↔가장 가까운 아군 타워 제곱거리 ult: 미니언이 타워 180000 안이면 타워 주변 Around, 밖이면 미니언 뒤 180000 지점 AroundPosition | 4 |
| 1 | 180000 | 46 | 계수 | behind(front_minion, d=180000): 미니언의 레인 경로 진행거리에서 180000 을 뺀(usub.sat, L68) 지점 좌표 | 4 |
| 2 | 30 | 114 | 계수 | move_speed × 30 — 타워 공격 사거리 합의 이동 여유(dbg L114) | 4 |
| 3 | 100 | 113 | 계수 | Entity::radius(): radius*(100+radius_mult)/100 (entity.rs:1515) | 4 |
| 4 | 10 | 36 | 태그 | PositionEvalPurpose::LaneSafe 메모리태그(논리 idx 8) — Around(+0x80)/AroundPosition(+0xb0) 에 store (L36·L43·L48<147) | 4 |
| 5 | 5 | 35 | 태그 | end_delay=5 (Around::new L35·L42<147 · AroundPosition::new L47<147 · RunAway::new L148) / SmallActionPlay::Around 메모리태그(store i8 5 @+177, L35·L42<147) | 4 |
| 6 | 3 | 148 | 태그 | SmallActionPlay::RunAway 메모리태그 (store @+177). ⚠본문의 `shl nuw nsw i64 %73, 3`(m15.ll:20848) 은 twin_towers len×8 포인터 stride 이지 이 값과 무관 | 4 |
| 7 | 15 | 120 | 태그 | SmallActionPlay::Attack 메모리태그 (L120<152 store, extend 시 22420 재기록) | 4 |
| 8 | 1 | 57 | 태그 | EntityType::Minion 태그(behind 는 미니언 전제, 아니면 panic L95) / (level-1) / 경로 루프 시작 i=1 | 4 |
| 9 | 0 | 32 | 태그 | block_target_tick==0(타워 필터) / TeamType::Player 태그(behind, trunc→0 이어야 통과) / radius_mult==0 / moved_distance 초기값 | 4 |
| 10 | -1 | 110 | 센티널 | Option<Effect> None(attack_effect casting 니치) → 타워공격 후보 없음 / iter_towers chain 상태 태그(-1·-2 는 이터레이터 내부) | 4 |

**`knobs` 조정점 5건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 앞미니언↔타워 근접 판정(타워 대기 vs 미니언 뒤) | line_wait.rs:41 | 32400000001 | 올리면 미니언이 타워에서 더 멀어도 타워 주변(Around tower) 대기를 택함 → 라인 전진 대기 감소 | 4 | 기존 |
| 1 | 미니언 뒤 대기 거리 | line_wait.rs:46 (behind d) | 180000 | 올리면 앞미니언 경로상 더 뒤쪽(아군 쪽)에서 대기 · 내리면 미니언에 붙어 대기 | 4 | 기존 |
| 2 | 타워 공격 사거리 이동 여유 | line_wait.rs:114 | 30 | move_speed×30. 올리면 더 먼 적 타워에도 Attack 후보 생성(v22 게이트는 별도) | 4 | 기존 |
| 3 | 후보 purpose | line_wait.rs:36,43,48 | 10 | LaneSafe. 바꾸면 하류 위치 평가 프로파일이 바뀜 | 4 | 기존 |
| 4 | end_delay | line_wait.rs:35,42,47,148 | 5 | 생성 후보의 end_delay | 4 | 기존 |

<details><summary>`callees` 피호출자 40건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | action_candidates | game_ai::plan_legacy::sub_plan::SubPlan::action_candidates | pub | fn(&mut game_ai::plan_legacy::sub_plan::SubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\mod.rs:118 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 18개 중 상위 3개 |
| 1 | action_candidates | game_ai::plan_legacy::sub_plan::HideSubPlan::action_candidates | pub | fn(&mut game_ai::plan_legacy::sub_plan::HideSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\hide.rs:19 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 18개 중 상위 3개 |
| 2 | action_candidates | game_ai::plan_legacy::sub_plan::StealSubPlan::action_candidates | pub | fn(&mut game_ai::plan_legacy::sub_plan::StealSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\steal.rs:33 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 18개 중 상위 3개 |
| 3 | adjust_position | game_core::Game::adjust_position | pub | fn(&game_core::MapDef, &game_core::GameSetting, i64, i64) -> (u64, u64) | game-core\src\simulation\game.rs:818 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | attack_structure_skill_action | game_ai::attack_structure_skill_action | pub | fn(&game_core::PlayerState, &game_core::OperationData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\fight_check.rs:794 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 5 | attack_summon_action | game_ai::attack_summon_action | pub | fn(&game_core::PlayerState, &game_core::OperationData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\fight_check.rs:836 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 6 | attack_tower_action | game_ai::plan_legacy::sub_plan::LineWaitSubPlan::attack_tower_action | in:game_ai::plan_legacy::sub_plan::line_wait | fn(&mut game_ai::plan_legacy::sub_plan::LineWaitSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> std::option::Option<game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\line_wait.rs:103 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 7 | base_positioning | game_ai::plan_legacy::sub_plan::JungleSubPlan::base_positioning | in:game_ai::plan_legacy::sub_plan::jungle | fn(&mut game_ai::plan_legacy::sub_plan::JungleSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> game_ai::SmallActionPlay | game-ai\src\plan_legacy\sub_plan\jungle.rs:19 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 8 | base_positioning | game_ai::plan_legacy::sub_plan::BattleSubPlan::base_positioning | in:game_ai::plan_legacy::sub_plan::battle | fn(&mut game_ai::plan_legacy::sub_plan::BattleSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\battle.rs:72 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 9 | base_positioning | game_ai::plan_legacy::sub_plan::LineSafeSubPlan::base_positioning | in:game_ai::plan_legacy::sub_plan::line_safe | fn(&game_ai::plan_legacy::sub_plan::LineSafeSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\line_safe.rs:15 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 10 | battle_action | game_ai::battle_action | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, usize) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\fight_check.rs:620 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 11 | behind | game_ai::plan_legacy::sub_plan::LineWaitSubPlan::behind | in:game_ai::plan_legacy::sub_plan::line_wait | fn(&game_core::OperationData, &game_core::Entity, u64) -> (u64, u64) | game-ai\src\plan_legacy\sub_plan\line_wait.rs:55 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 12 | distance | game_core::utils::distance | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:12 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 13 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 14 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 15 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 16 | get_start_position | game_core::LineType::get_start_position | pub | fn(&game_core::LineType, &game_core::GameSetting, usize) -> (u64, u64) | game-core\src\simulation\state\player.rs:1013 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 17 | isqrt | game_core::utils::isqrt | pub | fn(i64) -> i64 | game-core\src\utils.rs:85 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 18 | iter_towers | game_core::AbstractGameWithCache::<'a, 'b>::iter_towers | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5762 ~ game_core[6a30]::simulation::{impl#5}::iter_towers::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1908 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 19 | lane_path | game_core::MapDef::lane_path | pub | fn(&game_core::MapDef, game_core::LineType, usize) -> [(u64, u64); 7_usize] | game-core\src\simulation\map_def.rs:174 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 20 | line_minion_action_candidates | game_ai::line_minion_action_candidates | pub | fn(usize, &game_core::OperationData, &game_core::PlayerState, game_core::LineType) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\small_action\lane_minion.rs:72 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 21 | new | game_ai::SmallActionAround::new | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::OperationData, &game_core::PlayerState, usize, usize) -> game_ai::SmallActionAround | game-ai\src\small_action\around.rs:23 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 22 | new | game_ai::SmallActionAttack::new | pub | fn(&game_core::OperationData, usize) -> game_ai::SmallActionAttack | game-ai\src\small_action\cast.rs:23 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 23 | new | game_ai::SmallActionRunAway::new | pub | fn(&game_core::OperationData, &game_core::PlayerState, usize) -> game_ai::SmallActionRunAway | game-ai\src\small_action\move_actions.rs:25 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 24 | new | game_ai::SmallActionAroundPosition::new | pub | fn(&mut rand::rngs::std::StdRng, &game_core::OperationData, u64, u64, usize) -> game_ai::SmallActionAroundPosition | game-ai\src\small_action\around.rs:831 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 25 | player_team | game_core::TeamType::player_team | pub | fn(&game_core::TeamType) -> std::option::Option<usize> | game-core\src\simulation\entity.rs:1135 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 26 | player_team | game_view::ClientData::player_team | pub | fn(&game_view::ClientData) -> std::option::Option<std::cell::Ref< game_core::Team>> | game-view\src\logic\client\data.rs:1579 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 27 | player_team | game_view::ClientDatabase::player_team | pub | fn(&game_view::ClientDatabase) -> &game_core::Team | game-view\src\logic\client\data.rs:1163 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 28 | push | <game_core::Staff as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::staff | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\staff.rs:38 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 29 | push | <game_core::Athlete as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:434 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 30 | push | <game_core::Contract as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:2498 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 31 | radius | game_core::Entity::radius | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1509 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 32 | range_adjust | game_core::Effect::range_adjust | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:29 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 33 | team | game_view::ClientData::team | pub | fn(&game_view::ClientData, usize) -> std::option::Option<std::cell::Ref< game_core::Team>> | game-view\src\logic\client\data.rs:1651 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 34 | team | game_view::ClientDatabase::team | pub | fn(&game_view::ClientDatabase, usize) -> std::option::Option<&game_core::Team> | game-view\src\logic\client\data.rs:968 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 35 | team | game_core::PlayerAiContext::<'a, 'b, 'r>::team | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:583 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 36 | v22_lane_tower_pressure_attack_allowed | game_ai::v22_lane_tower_pressure_attack_allowed | pub | fn(usize, &game_core::OperationData, &game_core::PlayerState, &game_core::Entity) -> bool | game-ai\src\tower_discipline.rs:394 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 37 | v30_line_champion_action_tower_aggro_risk | game_ai::v30_line_champion_action_tower_aggro_risk | pub | fn(usize, &game_core::OperationData, &game_core::PlayerState, &game_ai::SmallActionPlay) -> bool | game-ai\src\tower_discipline.rs:158 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 38 | with_position_eval_purpose | game_ai::SmallActionAround::with_position_eval_purpose | pub | fn(game_ai::SmallActionAround, game_ai::PositionEvalPurpose) -> game_ai::SmallActionAround | game-ai\src\small_action\around.rs:56 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 39 | with_position_eval_purpose | game_ai::SmallActionAroundPosition::with_position_eval_purpose | pub | fn(game_ai::SmallActionAroundPosition, game_ai::PositionEvalPurpose) -> game_ai::SmallActionAroundPosition | game-ai\src\small_action\around.rs:857 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
</details>

⚠**미매칭 4개**: `extend`, `panic`, `reserve_internal_or_panic`, `retain`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m12.ll:35196) · **형제 10개** (LineWaitSubPlan)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::plan_legacy::sub_plan::LineWaitSubPlan as std::clone::Clone>::clone | pub | game-ai\src\plan_legacy\sub_plan\line_wait.rs:6 | True | fn(&game_ai::plan_legacy::sub_plan::LineWaitSubPlan) -> game_ai::plan_legacy::sub_plan::LineWaitSubPlan |
| 1 | <game_ai::plan_legacy::sub_plan::LineWaitSubPlan as std::fmt::Debug>::fmt | pub | game-ai\src\plan_legacy\sub_plan\line_wait.rs:6 | True | fn(&game_ai::plan_legacy::sub_plan::LineWaitSubPlan, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 2 | game_ai::plan_legacy::sub_plan::LineWaitSubPlan::new | pub | game-ai\src\plan_legacy\sub_plan\line_wait.rs:12 | True | fn(game_core::LineType) -> game_ai::plan_legacy::sub_plan::LineWaitSubPlan |
| 3 | game_ai::plan_legacy::sub_plan::LineWaitSubPlan::base_positioning | in:game_ai::plan_legacy::sub_plan::line_wait | game-ai\src\plan_legacy\sub_plan\line_wait.rs:16 | False | fn(&game_ai::plan_legacy::sub_plan::LineWaitSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 4 | game_ai::plan_legacy::sub_plan::LineWaitSubPlan::behind | in:game_ai::plan_legacy::sub_plan::line_wait | game-ai\src\plan_legacy\sub_plan\line_wait.rs:55 | False | fn(&game_core::OperationData, &game_core::Entity, u64) -> (u64, u64) |
| 5 | game_ai::plan_legacy::sub_plan::LineWaitSubPlan::attack_minion_action | in:game_ai::plan_legacy::sub_plan::line_wait | game-ai\src\plan_legacy\sub_plan\line_wait.rs:99 | False | fn(&mut game_ai::plan_legacy::sub_plan::LineWaitSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 6 | game_ai::plan_legacy::sub_plan::LineWaitSubPlan::attack_tower_action | in:game_ai::plan_legacy::sub_plan::line_wait | game-ai\src\plan_legacy\sub_plan\line_wait.rs:103 | False | fn(&mut game_ai::plan_legacy::sub_plan::LineWaitSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> std::option::Option<game_ai::SmallActionPlay> |
| 7 | game_ai::plan_legacy::sub_plan::LineWaitSubPlan::calculate_score_parameter_value | pub | game-ai\src\plan_legacy\sub_plan\line_wait.rs:126 | True | fn(&game_ai::plan_legacy::sub_plan::LineWaitSubPlan, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::ScoreParameter) |
| 8 | game_ai::plan_legacy::sub_plan::LineWaitSubPlan::action_candidates | pub | game-ai\src\plan_legacy\sub_plan\line_wait.rs:144 | False | fn(&mut game_ai::plan_legacy::sub_plan::LineWaitSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 9 | game_ai::plan_legacy::sub_plan::LineWaitSubPlan::score | pub | game-ai\src\plan_legacy\sub_plan\line_wait.rs:159 | False | fn(&game_ai::plan_legacy::sub_plan::LineWaitSubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 |

**`open` 5건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | L34 비교 `dist²(fm,nexus) < dist²(tower,nexus)` 의 방향(ult %184(fm) , %187(tower))은 IR 로 확정. 소스에서 `\|\|` 좌우 순서(fm None 검사가 먼저)는 IR 분기 순서 기준 | 4 |  |
| 1 | 미탐색 | L113/L114 사거리 합에서 move_speed*30 이 어느 줄 항인지: dbg 는 %625(mul 30)=L114, %626(add)=L26<113 — Effect::range(caster) 인라인(effect.rs:26)에 range+stat_buff.range+(level-1)*growth 가 있고 move_speed*30·tower.radius 는 L114 로 추정. 합 자체는 확정 | 4 |  |
| 2 | 미탐색 | battle_action 에 version/rnd/end_delay 가 poison 으로 전달됨(콜리의 _version/_rnd/_end_delay 미사용 확정). 재현 시 아무 값이나 넘겨도 되나 정본 콜리 명세(r14)에서 미사용 재확인 권장 | 3 |  |
| 3 | 미탐색 | behind() L81~82 의 sdiv 오버플로 검사(-9223372036854775808 / -1)는 rustc 자동 검사 — 판정 상수 아님(constants 에서 제외) | 4 |  |
| 4 | 미탐색 | get_start_position(&LineType, &GameSetting, team) 의 반환 의미(라인 시작 좌표)는 이름·인자로 추정, g15.ll:115823 본문은 안 봄 | 4 |  |

**`notes` 1건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | iter_towers 이터레이터(136B) 내부 상태 태그 -1/-2 · chain/flatten 인라인 분기(m15.ll 21885~22136)는 표준 라이브러리 어댑터라 의미 해석 생략 — 결과는 filter+min_by_key 로 확정 | 4 | 사실 서술 |

<details><summary>`closed` 1건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

| # | 물음 | 닫은 근거 |
|---|---|---|
| 0 | v30_line_champion_action_tower_aggro_risk(version, data, player, &SmallActionPlay 184B) -> bool 의 판정 내용은 범위 밖(m07.ll:52737) — 여기선 true=제거 극성만 확정(m01.ll:20584 br %27 → %47 삭제 경로) | 본문에 해소 표기가 있다 |
</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

