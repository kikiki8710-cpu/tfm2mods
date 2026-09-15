---

### `191` EpicCheckSubPlan::action_candidates — 에픽(모르가드) 확인 서브플랜 후보: 위험이면 도주 단독 / v25 오브젝트 자세(Screen·WaitGroup·SoftDisengage)가 있으면 그 자세 전용 후보 / 없으면 사이드 경유(Stump 캠프)→모르가드 캠프 AroundPosition + 적 근접 시 도주 + 적에게 보이면 교전 + 소환수 공격

| 항목 | 값 |
|---|---|
| id | `epic_check__EpicCheck__action_candidates` |
| 심볼 | `_RNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy8sub_plan10epic_checkNtB2_16EpicCheckSubPlan17action_candidates` |
| 소스 | `game-ai\src\plan_legacy\sub_plan\epic_check.rs:14` |
| IR | `m02.ll` 9581~10950행 |
| 경로·가시성 | `game_ai::plan_legacy::sub_plan::EpicCheckSubPlan::action_candidates` · **pub** |
| 계층 | 기타 |
| exe | `cb03b0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r16` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&mut game_ai::plan_legacy::sub_plan::EpicCheckSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay>
```

**TLS 메모 절(`sig.tls` · 정정 경로 `/specs[191]/sig/tls/<키>`)**

- `direct`: 없음 — 본 범위(m02.ll 9581~10950)에 LocalKey::with / thread_local 참조 0건(grep 실측).
- `indirect_callees_in_order`: ①nontarget_windup_perceived(적 챔프마다, rs:24) ②position_score_at_position(rs:36, purpose=Objective(11)) — 내부 position_eval_at(POS_EVAL_CACHE) 소관 ③v25_objective_posture(rs:45) ④Blackboard::is_recent_visible(rs:90, 적 챔프마다) ⑤battle_action(rs:99, 조건부) — 각 TLS 접점은 그 콜리 명세 참조. 미러 재현 시 이 호출 순서를 지켜야 캐시 채움 순서가 같다.

<details><summary>인자 9개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | bumpalo::Vec<SmallActionPlay> (32B) | IR %0 `sret([32 x i8]) writeonly`. ptr@0 · bump@+8 · cap@+0x10 · len@+0x18 (IR L9645~9651 초기화). 원소 184B, 태그 @+0xb1(177). | 4 |
| 1 | 1 | self | &mut EpicCheckSubPlan (1B: move_check bool @0x0) | IR %1 `captures(none) dereferenceable(1)` — readonly 없음 = 가변. 읽기 L9922(rs:65) · 쓰기 L10363 `store i8 1`(조건부, 아래 writes). | 4 |
| 2 | 2 | version | usize | IR %2. 본 함수 자체 분기 없음 — nontarget_windup_perceived·position_score_at_position·v25_objective_posture·battle_action 에 전달. | 4 |
| 3 | 3 | rnd | &mut StdRng (320B) | IR %3. 본문 직접 사용 없음 — SmallActionAroundPosition::new(4곳)·battle_action 에 전달. | 4 |
| 4 | 4 | player | &PlayerState (2528B) | IR %4 readonly. info.team(+0x930)·info.position(+0x9c0). | 4 |
| 5 | 5 | data | &OperationData (24B) | IR %5 readonly. +0 cache · +8 context · +0x10 blackboard(&[Blackboard;2]). | 4 |
| 6 | 6 | parameter | &ScoreParameter (5384B) | IR %6 readonly. +0x9f0 positioning_score 만 position_score_at_position 에 참조로 전달(L9868). | 4 |
| 7 | 7 | team_plan | &TeamPlan | IR %7 `ptr noundef nonnull align 8`(dereferenceable 없음 — 크기 미확정 타입). 본문 직접 읽기 0회 — v25_objective_posture 의 &self 로만 전달(L9911). ⚠attack_nexus 판에는 없는 인자(8인자 vs 7인자). | 4 |
| 8 | 8 | debug | &mut DebugFrameData (224B) | IR %8 `dereferenceable(224)` readonly 없음 = 가변. context.debug(+0x3b) 참일 때 infos(+0xa0) HashMap 에 문자열 push(rs:52~53). | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn action_candidates(&mut self, version, rnd, player, data, parameter, team_plan, debug) -> Vec<SmallActionPlay> {
  let bump = data.context.pool;  let mut res = Vec::new_in(bump);                        // rs:15
  let team = player.info.team; assert!(team<2);                                             // rs:17
  let champ = data.cache.player_champion[team][player.info.position].unwrap();              // rs:17
  // rs:23~31  (attack_nexus rs:125~131 과 문자 단위 동일 구조 · closure#0 rs:23)
  let has_non_target_action_range = data.cache.player_champion[1-team].iter().flatten().any(|c|
      nontarget_windup_perceived(version, player, data, c) && c.ty==Champion(13) && {
          let eff = match c.action_state.tag { 4=>skill_effect(casting 1|2 아니면 false, -1 panic), 5=>(level>2 ? skill2_effect : None)…, 6=>(level>4 ? ult_effect : None)…, _=>return false };
          Effect::is_in_range(eff, c, champ) });
  // rs:36~38
  let ps = position_score_at_position(version, player, data, &parameter.positioning_score, champ.x, champ.y, PositionEvalPurpose::Objective /*11*/);
  if ps.on_trajectory || (has_non_target_action_range || ps.on_periodic_trajectory) {         // rs:38 (뒤 둘 순서 표기불가)
      res.push(RunAway(SmallActionRunAway::new_with_skill(data, player, 5, true)));           // rs:40  tag 3
      return res;                                                                            // rs:41
  }
  // rs:45~55  v25 오브젝트 자세
  let posture = team_plan.v25_objective_posture(version, player, data, JungleType::Morgard /*4*/);   // rs:45  sret 88B
  if let Some(posture) = posture {                                                           // rs:46  (+0 != -1)
      if posture.kind as u8 > 2 {                                                            // rs:47  WaitGroup(3) | SoftDisengage(4)
          if posture.kind == SoftDisengage(4) && posture.near_enemy_count != 0 {              // rs:48
              res.push(RunAway(SmallActionRunAway::new_with_skill(data, player, 5, false)));  // rs:49  tag 3 · with_skill=false
          }
          res.push(AroundPosition(SmallActionAroundPosition::new(rnd, data, posture.wait_pos.0, posture.wait_pos.1, 5)));   // rs:51
          if data.context.debug {                                                            // rs:52 (+0x3b)
              debug.infos.entry(champ.id).or_default().push(format!("v25 morgard check posture: {:?}", posture.kind));   // rs:53
          }
          return res;                                                                        // rs:55
      } else if posture.kind == Screen(2) && posture.focus_enemy.is_some() {                 // rs:58
          let focus_enemy = posture.focus_enemy.unwrap();                                    // rs:59
          res.push(Trace(SmallActionTrace::new_attack_range(data, focus_enemy, 5)));         // rs:60  tag 14 (margin 15000 · attack_range_only=true · goal=get_entity_by_id(focus_enemy).xy or 0)
          // ⚠return 없음 — 아래 rs:65 로 계속(IR %196→%153→%133)
      }
      // Commit(0)/HoldCamp(1) 또는 Screen+focus None → 아래로
  }
  // rs:65~75  사이드 경유 검사 (self.move_check 갱신)
  let move_check_now: bool = if self.move_check { true }                                     // rs:65
      else if is_enemy_side(data.context, team, champ.x, champ.y) {                          // rs:66  = is_blue_side(ctx,x,y) != (team==0) ; is_blue_side = (x - y + setting.height) > setting.width (map_regions.rs:7~8, wrapping)
          let camp = data.context.map.camp_pos(JungleType::Stump /*2*/, team==0);            // rs:69
          if champ.distance_sq(camp) < 4900000001 /*70000²+1*/ { self.move_check = true; true }   // rs:71  (store L10363)
          else { false }
      } else { self.move_check = true; true };                                               // rs:66 else (IR %291)
  // rs:77~87  모르가드 캠프 접근
  let camp = data.context.map.camp_pos(JungleType::Morgard /*4*/, team==0);                  // rs:77
  if champ.distance_sq(camp) > 22500000000 /*150000²*/ {                                     // rs:78  멀다
      if move_check_now {                                                                    // rs:79
          res.push(AroundPosition(SmallActionAroundPosition::new(rnd, data, camp.0, camp.1, 5)));           // rs:80  모르가드 캠프로
      } else {
          let c2 = data.context.map.camp_pos(JungleType::Stump /*2*/, team==0);              // rs:83
          res.push(AroundPosition(SmallActionAroundPosition::new(rnd, data, c2.0, c2.1, 5)));               // rs:84  Stump 경유지로
      }
  } else {
      res.push(AroundPosition(SmallActionAroundPosition::new(rnd, data, camp.0, camp.1, 5)));               // rs:87  캠프 근처 대기
  }
  // rs:90~95  적 근접 → 도주 후보 추가
  let enemy_near = data.cache.player_champion[1-team].iter().flatten().any(|c|                // rs:90  closure#1
          Blackboard::is_recent_visible(&data.blackboard[1-team], data.cache.game, player, c)  // ⚠blackboard 인덱스 = 1-team (IR %61)
          && c.distance_sq(champ) < 22500000001 /*150000²+1*/)                                // rs:91
      || data.cache.others[1-team].iter().any(|x| x.distance_sq(champ) < 22500000001);      // rs:92  closure#2 (any 첫 절이 참이면 둘째 미평가: IR %366→%458 직행)
  if enemy_near {
      res.push(RunAway(SmallActionRunAway::new(data, player, 5)));                           // rs:95  tag 3
  }
  // rs:98~101
  if data.cache.game.is_visible(1-team, champ.id) {                                          // rs:98  vtable+0xf8 — 적에게 보이면
      res.extend(battle_action(version, rnd, player, data, 5));                              // rs:99
  }
  res.extend(attack_summon_action(player, data));                                            // rs:101
  res                                                                                        // rs:103
}
```

**`mem` 메모리 접근 38건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | EpicCheckSubPlan | 0x0 | move_check (bool) | r | IR L9922~9924 (rs:65) — true 면 사이드 검사 생략 | 4 | OK |  |
| 1 | PlayerState | 0x930 | info.team | r | IR L9650~9654. <2 바운즈체크 · 적팀=1-team(L9694) · is_blue = team==0(L10349/10370) | 4 | OK |  |
| 2 | PlayerState | 0x9c0 | info.position@tag | r | IR L9671~9673 | 4 | OK |  |
| 3 | OperationData | 0x0 | cache (&AbstractGameWithCache) | r | IR L9674. cache+0 = &dyn AbstractGame data ptr · cache+8 = vtable (L9973~9977·10495~10497) | 4 | OK |  |
| 4 | OperationData | 0x8 | context (&GameContext) | r | IR L9638~9639 | 4 | OK |  |
| 5 | OperationData | 0x10 | blackboard (&[Blackboard;2]) | r | IR L10498~10499·10527 — blackboard[1-team](적 팀 인덱스 %61) 을 is_recent_visible 1번째 인자로 | 4 | OK |  |
| 6 | GameContext | 0x0 | pool (&Bump) | r | IR L9643 | 4 | OK |  |
| 7 | GameContext | 0x8 | setting (&GameSetting) | r | IR L10350~10351 (is_blue_side 인라인) | 4 | OK |  |
| 8 | GameContext | 0x20 | map (&MapDef) | r | IR L10368~10369·10375~10376 — camp_pos 의 &self | 4 | OK |  |
| 9 | GameContext | 0x3b | debug (bool) | r | IR L10191~10194 (rs:52) — 디버그 문자열 기록 게이트 | 4 | OK |  |
| 10 | GameSetting | 0x12b8 | width | r | IR L10356~10358 (map_regions.rs:8 is_blue_side) | 4 | OK |  |
| 11 | GameSetting | 0x12c0 | height | r | IR L10352~10355 (map_regions.rs:7) | 4 | OK |  |
| 12 | AbstractGameWithCache | 0x1e0 | player_champion[2][5] | r | IR L9677~9680(내 챔프) · L9697~9737·10528~10542(적 5칸 순회 2회: rs:23·rs:90) | 4 | OK |  |
| 13 | AbstractGameWithCache | 0xf0 | others[team] (Vec<&Entity>, stride 32: buf ptr@+0 · len@+24) | r | IR L10726~10733 (rs:92) others[1-team] 순회 | 4 | OK |  |
| 14 | AbstractGame vtable | 0x28 | tick() | r | IR L9984~9986 (Trace 생성자 인라인 start_tick) | 4 | 확인불가(vtable 슬롯) |  |
| 15 | AbstractGame vtable | 0x1f0 | get_entity_by_id(id) -> Option<&Entity> | r | IR L9976~9978 (Trace 생성자: focus_enemy 좌표) | 4 | 확인불가(vtable 슬롯) |  |
| 16 | AbstractGame vtable | 0xf8 | is_visible(team, id) -> bool | r | IR L10801~10803 (rs:98) is_visible(1-team, champ.id) = 적 팀에게 내 챔프가 보이는가 | 4 | 확인불가(vtable 슬롯) |  |
| 17 | ScoreParameter | 0x9f0 | positioning_score | r | IR L9868 | 4 | OK |  |
| 18 | PositioningScore(sret 56B) | 0x30 | on_trajectory | r | IR L9891~9893 (+48) | 4 | OK |  |
| 19 | PositioningScore(sret 56B) | 0x31 | on_periodic_trajectory | r | IR L9894~9896 (+49). DI 지역변수명은 on_trajectory(rs:38 블록) — tcx 우선 | 3 | OK |  |
| 20 | ObjectivePosture(sret 88B, Option 니치 +0 == -1 → None) | 0x0 | focus_enemy@tag (0=None,1=Some; 외부 Option None = -1) | r | IR L9915~9917 (rs:46) · L9946~9948 (rs:58 trunc → Some 여부) | 4 | OK |  |
| 21 | ObjectivePosture | 0x8 | focus_enemy@Some.0 (usize) | r | IR L9958~9959 (rs:59) | 4 | OK |  |
| 22 | ObjectivePosture | 0x20 | wait_pos.0 | r | IR L10086~10087 (rs:51) | 4 | OK |  |
| 23 | ObjectivePosture | 0x28 | wait_pos.1 | r | IR L10088~10089 (rs:51) | 4 | OK |  |
| 24 | ObjectivePosture | 0x38 | near_enemy_count | r | IR L9937~9939 (rs:48) != 0 | 4 | OK |  |
| 25 | ObjectivePosture | 0x50 | kind@tag (ObjectivePostureKind) | r | IR L9929~9932 (rs:47 `>2`) · L9936 (==4) · L9945 (==2) · Debug fmt 인자(L10295) | 4 | OK |  |
| 26 | Entity | 0x68 | ty@tag | r | IR L9759~9761 ==13 Champion (적 챔프) | 4 | OK |  |
| 27 | Entity | 0x70 | ty@Champion.0.action_state@tag | r | IR L9786~9791 4/5/6 | 4 | OK |  |
| 28 | Entity | 0x4c8 | skill_effect (Option<Effect>) | r | IR L9813 (적 챔프 Skill 상태) · +0x4f8 casting@tag -1/1/2 | 4 | OK |  |
| 29 | Entity | 0x500 | skill2_effect (level>2 조건) | r | IR L9819~9823 · +0x530 casting@tag | 4 | OK |  |
| 30 | Entity | 0x538 | ult_effect (level>4 조건) | r | IR L9842~9846 · +0x568 casting@tag | 4 | OK |  |
| 31 | Entity | 0x5c0 | id | r | IR L10204~10205 (rs:53 디버그 키) · L10799~10800 (rs:98 is_visible 인자) | 4 | OK |  |
| 32 | Entity | 0x5c8 | level | r | IR L9819~9821 (>2) · L9842~9844 (>4) | 4 | OK |  |
| 33 | Entity | 0x660 | x | r | IR L9869~9870 (내 챔프, 이후 전 거리계산에 재사용) · 적 챔프/others | 4 | OK |  |
| 34 | Entity | 0x668 | y | r | IR L9878~9879 | 4 | OK |  |
| 35 | EpicCheckSubPlan (&mut self) | 0x0 | move_check | w | ★&mut self 쓰기 전수 = 이 1건. IR L10363 `store i8 1, ptr %1`(;L0 — 줄번호 소실, rs:66~75 블록 내부). 조건: move_check==false 일 때 (a) !is_enemy_side(context, team, champ.x, champ.y) 이거나 (b) is_enemy_side 이고 dist_sq(champ, camp_pos(Stump, team==0)) < 4900000001. 한 번 true 가 되면 이후 호출에서 rs:65 가 바로 통과. initializes 속성 없음(조건부 쓰기라 당연). | 4 | OK | true (i8 1) |
| 36 | DebugFrameData (&mut debug) | 0xa0 | infos: HashMap<usize, Vec<String>> | w | IR L10208~10345 (rs:53). 게이트 = context.debug(+0x3b) && posture 경로(kind∈{WaitGroup,SoftDisengage}) 만. 힙 부작용(hashbrown rustc_entry · insert_no_grow · RawVec<String>::grow_one · format_inner). 판정 무관 텔레메트리. | 4 | OK | infos.entry(champ.id).or_default().push(format!("v25 morgard check posture: {:?}", posture.kind)) |
| 37 | sret Vec<SmallActionPlay> | 0x0..0x20 | res | w | IR L9645~9651 초기화 · push 마다 len store · 반환 memcpy L10198/10898/10947 | 4 | 확인불가(tcx 사전에 타입 없음) | ptr/bump/cap/len |

**`consts` 상수 15건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 2 | 17 | 태그 | team 바운즈 상한(2팀). IR L9654. ⚠같은 리터럴 2 = ObjectivePostureKind 태그 2 Screen(rs:47 `>2`·rs:58 `==2`) · JungleType 태그 2 Stump(camp_pos 인자 rs:69·83) · CastingType Direction · level>2(rs:27) | 4 |
| 1 | 13 | 24 | 태그 | EntityType 태그 13 Champion (적 챔프 필터) | 4 |
| 2 | 4 | 25 | 임계 | ChampionActionState 4 Skill. ⚠같은 리터럴 4 = JungleType 4 Morgard(v25_objective_posture 마지막 인자 i8 4 · camp_pos rs:77) · ObjectivePostureKind 4 SoftDisengage(rs:48) · 적 ult 레벨 게이트 >4(rs:29) | 4 |
| 3 | 5 | 27 | 태그 | ChampionActionState 5 Skill2. ⚠같은 리터럴 5 = end_delay=5(RunAway/AroundPosition/Trace 생성자 · rs:40·49·51·60·80·84·87·95) · battle_action 5번째 인자(rs:99) | 4 |
| 4 | 6 | 29 | 태그 | ChampionActionState 6 Ult | 4 |
| 5 | 1 | 25 | 태그 | CastingType 1 Position (switch case) · trunc 비교 · store i8 1(move_check=true · Trace.attack_range_only) | 4 |
| 6 | -1 | 25 | 센티널 | Option 니치: Effect casting@tag -1 = None(적 챔프 unwrap_failed) · Option<ObjectivePosture> +0 == -1 = None(rs:46 IR L9916) | 4 |
| 7 | 11 | 36 | 태그 | PositionEvalPurpose 메모리태그 11 = Objective (position_score_at_position 마지막 인자 i8 11 · IR L9887). ⚠attack_nexus 는 General(2) | 4 |
| 8 | 3 | 40 | 태그 | SmallActionPlay 태그 3 RunAway (store i8 3 · IR L10909/10102/10815) | 4 |
| 9 | 14 | 60 | 태그 | SmallActionPlay 태그 14 Trace (store i8 14 @+177 · IR L10042) | 4 |
| 10 | 15000 | 60 | 산출값 | SmallActionTrace.attack_range_margin = 15000 (new_attack_range → new_attack_range_margin 인라인, trace.rs:79). IR L10024 | 4 |
| 11 | 0 | 60 | 태그 | Trace 필드 0 초기화(explicit_min_range None · escape_commit_until 0 · goal 0/0 when target None) · ObjectivePosture.near_enemy_count != 0(rs:48) · team==0=블루(rs:66·69·77) | 4 |
| 12 | 4900000001 | 71 | 임계 | 70000²+1 — Stump 캠프까지 dist_sq < 70000²+1 (= ≤70000, 약 2.2셀) 이면 move_check=true. IR L10402 | 4 |
| 13 | 22500000000 | 78 | 임계 | 150000² — 모르가드 캠프까지 dist_sq > 150000² 이면 '멀다' → move_check 여부로 모르가드/Stump 캠프 선택. IR L10427 | 4 |
| 14 | 22500000001 | 91 | 임계 | 150000²+1 — 적 챔프(rs:91)/others(rs:92) 와의 dist_sq < 150000²+1 (≤150000, 약 4.7셀) 이면 RunAway 후보 추가. IR L10589/10795 | 4 |

**`knobs` 조정점 6건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | Stump 경유지 도착 판정 반경 | epic_check.rs:71 (m02.ll L10402) | 4900000001 | 70000²+1. 올리면 경유지에 덜 가까워도 move_check 완료로 보고 바로 모르가드 캠프로 향함(경유 생략 경향), 내리면 경유지에 더 붙어야 함 | 4 | 기존 |
| 1 | 모르가드 캠프 '멀다' 임계 | epic_check.rs:78 (m02.ll L10427) | 22500000000 | 150000². 이보다 멀면(and move_check 미완) Stump 경유지로 우회. 올리면 우회 구간이 줄고 직행, 내리면 더 자주 우회 | 4 | 기존 |
| 2 | 적 근접 도주 후보 반경 | epic_check.rs:91·92 (m02.ll L10589·10795) | 22500000001 | 150000²+1. 최근 시야에 잡힌 적 챔프/others 가 이 안이면 RunAway 후보 추가. 올리면 더 일찍 도주 후보가 섞임(소극적), 내리면 더 버팀 | 4 | 기존 |
| 3 | Trace 사거리 마진 | epic_check.rs:60 → trace.rs:79 (m02.ll L10024) | 15000 | Screen 자세의 focus_enemy 추적 시 attack_range_margin. 올리면 더 먼 거리에서 멈춤(덜 붙음) | 4 | 기존 |
| 4 | 위치평가 목적 | epic_check.rs:36 (m02.ll L9887 i8 11) | 11 | Objective 목적으로 위치 점수 산출. General(2)로 바꾸면 attack_nexus 와 같은 평가 축이 됨 — 궤도 판정 결과가 달라질 수 있음(효과 크기는 position_eval 명세 소관) | 4 | 기존 |
| 5 | SoftDisengage 도주 게이트 | epic_check.rs:48 (m02.ll L9936~9940) | 0 | near_enemy_count != 0 일 때만 with_skill=false 도주 후보 추가. 임계를 올리면(예: >1) 소수 적에는 도주 안 함 | 4 | 기존 |

<details><summary>`callees` 피호출자 33건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | action_candidates | game_ai::plan_legacy::sub_plan::SubPlan::action_candidates | pub | fn(&mut game_ai::plan_legacy::sub_plan::SubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\mod.rs:118 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 18개 중 상위 3개 |
| 1 | action_candidates | game_ai::plan_legacy::sub_plan::HideSubPlan::action_candidates | pub | fn(&mut game_ai::plan_legacy::sub_plan::HideSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\hide.rs:19 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 18개 중 상위 3개 |
| 2 | action_candidates | game_ai::plan_legacy::sub_plan::StealSubPlan::action_candidates | pub | fn(&mut game_ai::plan_legacy::sub_plan::StealSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\steal.rs:33 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 18개 중 상위 3개 |
| 3 | attack_summon_action | game_ai::attack_summon_action | pub | fn(&game_core::PlayerState, &game_core::OperationData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\fight_check.rs:836 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | battle_action | game_ai::battle_action | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, usize) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\fight_check.rs:620 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 5 | camp_pos | game_core::MapDef::camp_pos | pub | fn(&game_core::MapDef, game_core::JungleType, bool) -> (u64, u64) | game-core\src\simulation\map_def.rs:207 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 6 | distance_sq | game_core::utils::distance_sq | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:6 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 7 | distance_sq | game_core::Entity::distance_sq | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2157 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 8 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 9 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 10 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 11 | is_blue_side | game_core::is_blue_side | pub | fn(&game_core::GameContext, u64, u64) -> bool | game-core\src\simulation\map_regions.rs:6 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 12 | is_enemy_side | game_core::is_enemy_side | pub | fn(&game_core::GameContext, usize, u64, u64) -> bool | game-core\src\simulation\map_regions.rs:57 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 13 | is_in_range | game_core::Effect::is_in_range | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\effect.rs:63 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 14 | is_recent_visible | game_core::Blackboard::is_recent_visible | pub | fn(&game_core::Blackboard, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\game\blackboard.rs:346 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 15 | is_visible | game_core::World::is_visible | pub | fn(&game_core::World, usize, usize) -> bool | game-core\src\simulation\game\data.rs:384 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 16 | is_visible | game_core::VisibleState::is_visible | pub | fn(&game_core::VisibleState) -> bool | game-core\src\simulation\game\data.rs:121 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 17 | is_visible | game_core::AbstractGame::is_visible | pub | fn(&Self/#0, usize, usize) -> bool | game-core\src\simulation.rs:125 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 18 | new | game_ai::SmallActionRunAway::new | pub | fn(&game_core::OperationData, &game_core::PlayerState, usize) -> game_ai::SmallActionRunAway | game-ai\src\small_action\move_actions.rs:25 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 19 | new | game_ai::SmallActionAroundPosition::new | pub | fn(&mut rand::rngs::std::StdRng, &game_core::OperationData, u64, u64, usize) -> game_ai::SmallActionAroundPosition | game-ai\src\small_action\around.rs:831 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 20 | new_attack_range | game_ai::SmallActionTrace::new_attack_range | pub | fn(&game_core::OperationData, usize, usize) -> game_ai::SmallActionTrace | game-ai\src\small_action\trace.rs:75 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 21 | new_with_skill | game_ai::SmallActionRunAway::new_with_skill | pub | fn(&game_core::OperationData, &game_core::PlayerState, usize, bool) -> game_ai::SmallActionRunAway | game-ai\src\small_action\move_actions.rs:29 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 22 | nontarget_windup_perceived | game_ai::nontarget_windup_perceived | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity) -> bool | game-ai\src\utils.rs:553 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 23 | position_score_at_position | game_ai::position_score_at_position | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, u64, u64, game_ai::PositionEvalPurpose) -> game_core::PositioningScore | game-ai\src\position_eval.rs:1192 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 24 | push | <game_core::Staff as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::staff | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\staff.rs:38 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 25 | push | <game_core::Athlete as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:434 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 26 | push | <game_core::Contract as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:2498 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 27 | skill_effect | game_core::Entity::skill_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1688 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 28 | team | game_view::ClientData::team | pub | fn(&game_view::ClientData, usize) -> std::option::Option<std::cell::Ref< game_core::Team>> | game-view\src\logic\client\data.rs:1651 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 29 | team | game_view::ClientDatabase::team | pub | fn(&game_view::ClientDatabase, usize) -> std::option::Option<&game_core::Team> | game-view\src\logic\client\data.rs:968 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 30 | team | game_core::PlayerAiContext::<'a, 'b, 'r>::team | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:583 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 31 | v25_objective_posture | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v25_objective_posture | pub | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::PlayerState, &game_core::OperationData, game_core::JungleType) -> std::option::Option<game_ai::plan_legacy::team_plan::ObjectivePosture> | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:207 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 32 | width | game_view::ui::team_history_common::view::Measure::<'a>::width | pub | fn(&game_view::ui::team_history_common::view::Measure<'a/#0>, &str, &str, f32) -> f32 | game-view\src\ui\team_history_common\view.rs:149 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
</details>

⚠**미매칭 9개**: `else`, `entry`, `extend`, `format_inner`, `grow_one`, `insert_no_grow`, `or_default`, `reserve_internal_or_panic`, `rustc_entry`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m12.ll:35231) · **형제 7개** (EpicCheckSubPlan)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::plan_legacy::sub_plan::EpicCheckSubPlan as std::clone::Clone>::clone | pub | game-ai\src\plan_legacy\sub_plan\epic_check.rs:8 | True | fn(&game_ai::plan_legacy::sub_plan::EpicCheckSubPlan) -> game_ai::plan_legacy::sub_plan::EpicCheckSubPlan |
| 1 | <game_ai::plan_legacy::sub_plan::EpicCheckSubPlan as std::default::Default>::default | pub | game-ai\src\plan_legacy\sub_plan\epic_check.rs:8 | True | fn() -> game_ai::plan_legacy::sub_plan::EpicCheckSubPlan |
| 2 | <game_ai::plan_legacy::sub_plan::EpicCheckSubPlan as std::fmt::Debug>::fmt | pub | game-ai\src\plan_legacy\sub_plan\epic_check.rs:8 | True | fn(&game_ai::plan_legacy::sub_plan::EpicCheckSubPlan, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 3 | game_ai::plan_legacy::sub_plan::EpicCheckSubPlan::action_candidates | pub | game-ai\src\plan_legacy\sub_plan\epic_check.rs:14 | False | fn(&mut game_ai::plan_legacy::sub_plan::EpicCheckSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 4 | game_ai::plan_legacy::sub_plan::EpicCheckSubPlan::calculate_score_parameter_value | pub | game-ai\src\plan_legacy\sub_plan\epic_check.rs:106 | True | fn(&game_ai::plan_legacy::sub_plan::EpicCheckSubPlan, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::ScoreParameter) |
| 5 | game_ai::plan_legacy::sub_plan::EpicCheckSubPlan::score | pub | game-ai\src\plan_legacy\sub_plan\epic_check.rs:123 | False | fn(&game_ai::plan_legacy::sub_plan::EpicCheckSubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 |
| 6 | game_ai::plan_legacy::sub_plan::EpicCheckSubPlan::merge | pub | game-ai\src\plan_legacy\sub_plan\epic_check.rs:143 | True | fn(&mut game_ai::plan_legacy::sub_plan::EpicCheckSubPlan, game_ai::plan_legacy::sub_plan::EpicCheckSubPlan) |

**`open` 8건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 표기 불가 | rs:38 `A \|\| B \|\| C` 의 B·C 상대 순서 — IR `or` 접힘, 외연 동일 → 표기 불가(attack_nexus rs:140 과 동일 사정). | 4 |  |
| 1 | 표기 불가 | self.move_check = true 의 store 가 ;L0(줄번호 소실). rs:66 else 절과 rs:71 참 절 두 경로가 하나의 store 로 합쳐져(IR %291 preds %302·%279) 소스가 두 줄인지 한 줄인지 표기 불가 — 동작은 확정. | 4 |  |
| 2 | 미탐색 | rs:90 is_recent_visible 의 첫 인자 blackboard 인덱스가 1-team(적 팀)인 것이 의도(적 팀 블랙보드 = '적이 나를 최근에 봤나')인지 자기 팀 오기인지 — IR 은 %61(=1-team) 로 확정, 의미는 콜리 명세 소관. | 4 |  |
| 3 | 미탐색 | AroundPosition 원소의 path_finder 72B(+104..+176) 가 memcpy 로 채워지는데 원본(%8)이 wait_around 결과의 일부라 어느 바이트가 초기화돼 있는지 미확인(태그 0xad=2 None 만 확정). ELEM_LIVE 등록 시 그 구간은 런타임 갈림 오프셋으로 확인 필요. | 4 |  |
| 4 | 미탐색 | Option<ObjectivePosture> None 니치가 focus_enemy@tag == -1 인 것은 IR(L9916) 관측. tcxdict --enum 으로 Option<ObjectivePosture> 는 직접 조회 불가(제네릭 인스턴스). | 3 |  |
| 5 | 미탐색 | team_plan(%7) 의 실제 크기 — IR 에 dereferenceable 없음. TeamPlan 구조는 v25_objective_posture 명세 소관. | 4 |  |
| 6 | 미탐색 | rs:99 battle_action 5번째 인자 5 · 각 생성자 end_delay 5 의 게임적 의미 — 자식 명세 소관. | 4 |  |
| 7 | 미탐색 | SmallActionAroundPosition::new 가 around_radius(+88)=80000 을 고정 저장(m08.ll L103238 관측) — 본 범위 밖 리터럴이라 constants 제외(C1). AroundPosition 후보의 배회 반경 노브는 그 생성자 명세 소관. | 4 |  |

**`notes` 1건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | rs:66 is_enemy_side 인자 순서 — tcx sig fn(&GameContext, usize, u64, u64) 로 (ctx, team, x, y) 추정. 인라인이라 IR 로는 (team==0) xor is_blue_side(x,y) 만 확정. | 3 | 사실 서술 |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

