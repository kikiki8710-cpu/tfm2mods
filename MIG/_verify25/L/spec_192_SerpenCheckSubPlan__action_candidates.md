---

### `192` SerpenCheckSubPlan::action_candidates — 서펜 스틸 Lurk(SerpenCheck) 서브플랜의 후보 생성 — 위험(논타겟 윈드업·궤적·근접 적)이면 도주 단일, 아니면 v25 태세→서펜 캠프/경유지 대기 + 전투 후보

| 항목 | 값 |
|---|---|
| id | `serpen_check__SerpenCheck__action_candidates` |
| 심볼 | `_RNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy8sub_plan12serpen_checkNtB2_18SerpenCheckSubPlan17action_candidates` |
| 소스 | `game-ai\src\plan_legacy\sub_plan\serpen_check.rs:14` |
| IR | `m14.ll` 30237~31553행 |
| 경로·가시성 | `game_ai::plan_legacy::sub_plan::SerpenCheckSubPlan::action_candidates` · **pub** |
| 계층 | 기타 |
| exe | `e8b5e0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r16` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&mut game_ai::plan_legacy::sub_plan::SerpenCheckSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay>
```

**TLS 메모 절(`sig.tls` · 정정 경로 `/specs[192]/sig/tls/<키>`)**

- `name`: (직접 접점 없음) — 콜리 경유만
- `role`: 소비자(간접)
- `key`: 본문에 LocalKey::with 0건(grep LocalKey|__getit|CACHE|MEMO = 0). 콜리 순서: ①MapDef::camp_pos(L23, 조건부) → game_core CAMP_POS_MEMO(TLS RefCell<[[Option<(u64,u64)>;2];8]>, g07.ll:152570·351647) ②nontarget_windup_perceived(L36, 적 챔프마다) ③position_score_at_position(L48, purpose=Objective(11)) → position_eval 계열 캐시(POS_EVAL_CACHE 등 — 그 명세 참조) ④v25_objective_posture(L57) ⑤camp_pos(L77·L83) ⑥Blackboard::is_recent_visible(L90) ⑦battle_action(L99, is_visible 참일 때) → interaction_score/INTER_CTX·HP_VALUE_MEMO 등은 이 안 ⑧attack_summon_action(L101)
- `layout`: 해당 없음(본 함수 자체 TLS 없음)
- `invalidation`: 해당 없음
- `call_conditions`: ③은 L20~L35 를 지나면 무조건 1회(champ 좌표) · ⑦⑧은 도주 단일(L52)·태세 WaitGroup/SoftDisengage(L67) 조기 return 을 안 탄 경우에만, 그리고 ⑦은 game.is_visible(enemy_team, champ.id) 참일 때만

<details><summary>인자 9개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | bumpalo::Vec<SmallActionPlay> (32B) | writeonly sret. +0 ptr · +8 bump(&Bump = data.context.pool) · +0x10 cap · +0x18 len. 원소 184B | 4 |
| 1 | 1 | self | &mut SerpenCheckSubPlan (1B) | IR: noalias captures(none), readonly 없음 = &mut. 유일 필드 move_check:bool @+0. 쓰기 1곳(writes 참조) | 4 |
| 2 | 2 | version | usize | 이 함수 본문엔 분기 없음. nontarget_windup_perceived · position_score_at_position · v25_objective_posture · battle_action 에 그대로 전달 | 4 |
| 3 | 3 | rnd | &mut StdRng (320B) | 본문에서 직접 읽지 않음. AroundPosition::new(4곳)·battle_action 에 전달(콜리가 소비) | 4 |
| 4 | 4 | player | &PlayerState (2528B) | readonly | 4 |
| 5 | 5 | data | &OperationData (24B) | +0 cache(&AbstractGameWithCache) · +8 context(&GameContext) · +0x10 blackboard(&[Blackboard;2]) | 4 |
| 6 | 6 | parameter | &ScoreParameter (5384B) | 본문은 +0x9f0(positioning_score: PositioningScoreData) 주소만 취해 position_score_at_position 에 넘김. 다른 필드 읽지 않음 | 4 |
| 7 | 7 | team_plan | &TeamPlan (1064B) | IR 속성 nonnull 만(readonly 없음)이나 본문은 v25_objective_posture(self=team_plan) 호출에만 사용. 콜리 시그니처가 &self(readonly) 라 사실상 읽기 전용(추정 근거: m09.ll:15168 define 의 %0 readonly) | 4 |
| 8 | 8 | debug | &mut DebugFrameData (224B) | noalias, readonly 없음 = &mut. context.debug=true 일 때만 +0xa0 infos 에 문자열 push(writes 참조) | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn action_candidates(&mut self, version, rnd, player, data, parameter, team_plan, debug) -> Vec<SmallActionPlay>
L15  bump = data.context.pool; res = Vec::new_in(bump)
L17  team = player.info.team (bounds<2); champ = data.cache.player_champion[team][player.info.position as usize].unwrap()
L19  if !self.move_check {
L20    if is_enemy_side(team, champ.x, champ.y)   // = (team==0) XOR is_blue_side(x,y), is_blue_side = (x - y + setting.height) > setting.width  (map_regions.rs:58, 7~8)
L23      camp = context.map.camp_pos(JungleType::Rhino, is_blue_side=team==0)
L25      if |champ.x-camp.x|² + |champ.y-camp.y|² < 4900000001 (70000²+1) { self.move_check = true }
       else { self.move_check = true }   // 아군 진영이면 즉시 통과 (store 는 dbg L0 블록 %99 공유)
     }
     mc = self.move_check (phi %101)
L35  enemy_team = 1 - team
     has_non_target_action_range = cache.player_champion[enemy_team].iter().flatten().any(|c|
L36    nontarget_windup_perceived(version, player, data, c) && c.ty==Champion(13) &&
L37    match c.action_state { Skill(4) => e=c.skill_effect.unwrap()(casting -1 → panic) ,
L39                           Skill2(5) => e = if c.level>2 {c.skill2_effect} else {DEFAULT_EFFECT(@anon…22)} ,
L41                           Ult(6)    => e = if c.level>4 {c.ult_effect} else {DEFAULT_EFFECT} , _ => false }
       && matches!(e.casting, Position(1)|Direction(2)) && e.is_in_range(caster=c, target=champ))
L48  ps: PositioningScore = position_score_at_position(version, player, data, &parameter.positioning_score, champ.x, champ.y, PositionEvalPurpose::Objective(11))
L50  if ps.on_trajectory(+0x30) || has_non_target_action_range || ps.on_periodic_trajectory(+0x31) {
L52    res.push(RunAway(SmallActionRunAway::new_with_skill(data, player, end_delay=5, with_skill=true)))   // 태그 3
L53    return res }
L57  posture: Option<ObjectivePosture> = team_plan.v25_objective_posture(version, player, data, target=JungleType::Serpen(5))
L58  if let Some(p) = posture {
L59    if p.kind as u8 > 2 {            // WaitGroup(3) | SoftDisengage(4)
L60      if p.kind==SoftDisengage(4) && p.near_enemy_count != 0 {
L61        res.push(RunAway(new_with_skill(data, player, 5, with_skill=false))) }   // 태그 3
L63      res.push(AroundPosition(SmallActionAroundPosition::new(rnd, data, p.wait_pos.0, p.wait_pos.1, end_delay=5)))   // 암묵 variant, 태그 store 없음
L64      if context.debug { L65 debug.infos.entry(champ.id).or_default().push(format!("{:?}", p.kind)) }
L67      return res
       } else if p.kind==Screen(2) && p.focus_enemy.is_some() {
L71      focus = p.focus_enemy.unwrap()
L72      res.push(Trace(SmallActionTrace::new_attack_range(data, target=focus, end_delay=5)))   // 태그 14
       }   // Commit(0)/HoldCamp(1)/Screen-without-focus → 아무것도 안 함
     }
L77  camp = map.camp_pos(Serpen(5), team==0)
L78  if dist²(champ, camp) > 22500000000 (150000²) {
L79    if mc { L80 res.push(AroundPosition::new(rnd, data, camp.0, camp.1, 5)) }
       else { L83 rhino = map.camp_pos(Rhino(0), team==0); L84 res.push(AroundPosition::new(rnd, data, rhino.0, rhino.1, 5)) }
     } else { L87 res.push(AroundPosition::new(rnd, data, camp.0, camp.1, 5)) }
L90  danger = cache.player_champion[enemy_team].iter().flatten().any(|c| data.blackboard[enemy_team].is_recent_visible(cache.game, player, c) && L91 dist²(c, champ) < 22500000001)
       || L92 cache.others[enemy_team].iter().any(|e| dist²(e, champ) < 22500000001)   // 단락: 앞이 참이면 others 미순회
L94  if danger { L95 res.push(RunAway(SmallActionRunAway::new(data, player, end_delay=5))) }   // 태그 3
L98  if cache.game.is_visible(enemy_team, champ.id) {   // vtable+0xf8
L99    res.extend(battle_action(version, rnd, player, data, 5)) }
L101 res.extend(attack_summon_action(player, data))
L103 return res
```

**`mem` 메모리 접근 41건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | OperationData | 0x8 | context | r | &GameContext (L15) | 4 | OK |  |
| 1 | OperationData | 0x0 | cache | r | &AbstractGameWithCache (L17·L90) | 4 | OK |  |
| 2 | OperationData | 0x10 | blackboard | r | &[Blackboard;2] → [enemy_team] 을 is_recent_visible 의 self 로 (L90) | 4 | OK |  |
| 3 | GameContext | 0x0 | pool | r | &Bump → Vec::new_in (L15) | 4 | OK |  |
| 4 | GameContext | 0x8 | setting | r | &GameSetting → is_blue_side 인라인이 width/height 읽음 (L20) | 4 | OK |  |
| 5 | GameContext | 0x20 | map | r | &MapDef → camp_pos (L23·L77·L83) | 4 | OK |  |
| 6 | GameContext | 0x3b | debug | r | bool. true 면 debug.infos 기록 (L64). gep 59 | 4 | OK |  |
| 7 | GameSetting | 0x12b8 | width | r | is_blue_side: (x - y + height) > width (map_regions.rs:7~8, gep 4792) | 4 | OK |  |
| 8 | GameSetting | 0x12c0 | height | r | gep 4800 | 4 | OK |  |
| 9 | PlayerState | 0x930 | info.team | r | gep 2352. <2 아니면 panic_bounds_check | 4 | OK |  |
| 10 | PlayerState | 0x9c0 | info.position@tag | r | i32 zext → player_champion[team][pos] 인덱스 (gep 2496) | 4 | OK |  |
| 11 | AbstractGameWithCache | 0x1e0 | player_champion | r | [[Option<&Entity>;5];2] (gep 480). [team][pos] None 이면 unwrap_failed. L35/L90 은 [enemy_team] 5칸을 순회(stride 8, 종료 40) | 4 | OK |  |
| 12 | AbstractGameWithCache | 0xf0 | others | r | [bumpalo Vec<&Entity>;2] (gep 240) → [enemy_team].ptr(+0)/len(+0x18) 순회 (L92) | 4 | OK |  |
| 13 | AbstractGameWithCache | 0x0 | game | r | &dyn AbstractGame 팻포인터(data@+0, vtable@+8) → is_recent_visible 인자, vtable+0xf8(is_visible) 간접호출 (L90·L98) | 4 | OK |  |
| 14 | Entity | 0x660 | x | r | gep 1632 (champ·적 챔프·others) | 4 | OK |  |
| 15 | Entity | 0x668 | y | r | gep 1640 | 4 | OK |  |
| 16 | Entity | 0x5c0 | id | r | gep 1472. debug.infos 키(L65) · is_visible 인자(L98) | 4 | OK |  |
| 17 | Entity | 0x68 | ty@tag | r | gep 104. ==13(Champion) 확인 (L36 closure, Entity::is_in_skill 인라인) | 4 | OK |  |
| 18 | Entity | 0x70 | ty@Champion.0.action_state@tag | r | gep 112. switch 4=Skill/5=Skill2/6=Ult (entity.rs:1572 is_in_skill 인라인) | 4 | OK |  |
| 19 | Entity | 0x4f8 | skill_effect@Some.0.casting@tag | r | gep 1272. -1=None→unwrap_failed · 1(Position)\|2(Direction)→is_in_range · 그외(0 Targeting/3 None) skip | 4 | OK |  |
| 20 | Entity | 0x4c8 | skill_effect@Some.0 | r | gep 1224. &Effect(56B) → is_in_range self | 4 | OK |  |
| 21 | Entity | 0x5c8 | level | r | gep 1480. >2 면 skill2_effect, >4 면 ult_effect 사용(아니면 정적 기본 Effect @anon…22) — entity.rs:1693/1701 skill2_effect/ult_effect 인라인 | 4 | OK |  |
| 22 | Entity | 0x500 | skill2_effect@Some.0 | r | gep 1280. casting@+0x30(=0x530) | 4 | OK |  |
| 23 | Entity | 0x538 | ult_effect@Some.0 | r | gep 1336. casting@+0x30(=0x568) | 4 | OK |  |
| 24 | Effect | 0x30 | casting | r | gep 48 (skill2/ult 경로에서 선택된 Effect 기준) | 4 | OK |  |
| 25 | ScoreParameter | 0x9f0 | positioning_score | r | gep 2544. 주소만 position_score_at_position 4번째 인자로 | 4 | OK |  |
| 26 | PositioningScore(sret 56B) | 0x30 | on_trajectory | r | gep 48 (L50) | 4 | OK |  |
| 27 | PositioningScore(sret 56B) | 0x31 | on_periodic_trajectory | r | gep 49 (L50). ⚠DI 는 이 값을 `on_trajectory` 로 이름붙임 — tcx 오프셋이 정본 | 3 | OK |  |
| 28 | Option<ObjectivePosture>(sret 88B) | 0x0 | focus_enemy@tag | r | i64 -1 = None(Option<ObjectivePosture> 니치) · 1 = focus_enemy Some (L58·L70) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 29 | ObjectivePosture | 0x8 | focus_enemy.0 | r | Trace 대상 (L71) | 4 | OK |  |
| 30 | ObjectivePosture | 0x20 | wait_pos.0 | r | gep 32 (L63) | 4 | OK |  |
| 31 | ObjectivePosture | 0x28 | wait_pos.1 | r | gep 40 (L63) | 4 | OK |  |
| 32 | ObjectivePosture | 0x38 | near_enemy_count | r | gep 56. !=0 && kind==SoftDisengage → RunAway (L60) | 4 | OK |  |
| 33 | ObjectivePosture | 0x50 | kind | r | gep 80. u8 >2 → WaitGroup(3)/SoftDisengage(4) 분기 · ==2 Screen (L59·L70) | 4 | OK |  |
| 34 | DebugFrameData | 0xa0 | infos | r | gep 160. HashMap<usize,Vec<String>> entry(champ.id) (L65) | 4 | OK |  |
| 35 | AbstractGame vtable | 0xf8 | is_visible | r | gep 248. divtable AbstractGame 0xf8 → is_visible(game, team=enemy_team, entity_id=champ.id) -> bool (L98) | 3 | 확인불가(vtable 슬롯) |  |
| 36 | bumpalo Vec<&Entity> | 0x18 | len | r | gep 24 (others[enemy_team]·battle_action/attack_summon_action 반환 Vec) | 4 | OK |  |
| 37 | SerpenCheckSubPlan(self) | 0x0 | move_check | w | ★&mut self 쓰기 전수 = 이 1곳. 조건: !move_check && (!is_enemy_side(team,champ.x,champ.y) \|\| dist²(champ, camp_pos(Rhino, team==0)) < 70000²+1). m14.ll:30414 (store i8 1, ptr %1 · dbg 루트 L0 — 소스 줄 소실, L25 블록 %99 에서 실행) | 4 | OK | i8 1 |
| 38 | (sret) Vec<SmallActionPlay> | 0x0 -> ptr/bump/cap/len |  | w | m14.ll:30302~30308(초기화 ptr=8(dangling) bump cap=0 len=0) · 30726/30794/30844/31083/31264/31314/31453/31546 (원소 memcpy 184B) · 31476/31487 extend · 30855/31492/31550 (sret memcpy) | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) | new_in(bump) 후 push 로 len 증가, memcpy 32B 로 sret 에 복사 |
| 39 | DebugFrameData(debug) | 0xa0 -> infos[champ.id].push(String) |  | w | context.debug 일 때만 (L64~65). rustc_entry/insert_no_grow/format_inner/grow_one. 행동 무영향 텔레메트리 | 4 | OK | format!("{:?}", posture.kind) |
| 40 | StdRng(rnd) | (콜리) |  | w | 본문 직접 쓰기 없음 | 4 | 확인불가(오프셋 파싱 실패) | AroundPosition::new(4곳)·battle_action 이 &mut 로 소비 |

**`consts` 상수 15건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 4900000001 | 25 | 임계 | 70000² + 1 — Rhino 캠프 접근 판정(제곱거리 ult). 이보다 작으면 move_check=true | 4 |
| 1 | 22500000000 | 78 | 임계 | 150000² — 서펜 캠프와의 제곱거리 ugt: 멀면 경유(rhino/serpen 분기), 아니면 서펜 캠프 주변 대기 | 4 |
| 2 | 22500000001 | 91 | 임계 | 150000² + 1 — 최근 시야 적 챔피언(L91) / 적 others(L92) 와의 제곱거리 ult → RunAway 추가 | 4 |
| 3 | 0 | 23 | 태그 | JungleType::Rhino(태그 0) — camp_pos 첫 인자 (L23·L83) | 4 |
| 4 | 5 | 77 | 태그 | JungleType::Serpen(태그 5) — camp_pos(L77) · v25_objective_posture target(L57). ⚠같은 리터럴 5 가 end_delay(RunAway/AroundPosition/Trace/battle_action 인자)로도 쓰임 — 별 항목 | 4 |
| 5 | 11 | 48 | 태그 | PositionEvalPurpose::Objective(메모리태그 11 · 논리 idx 9) — position_score_at_position purpose | 4 |
| 6 | 13 | 36 | 태그 | EntityType::Champion 태그 — 적 엔티티가 챔피언인지 (is_in_skill 인라인) | 4 |
| 7 | 4 | 37 | 태그 | ChampionActionState::Skill 태그(switch) / ObjectivePostureKind::SoftDisengage(L60 icmp eq i8 4) / Entity.level > 4 → ult_effect(entity.rs:1701) | 4 |
| 8 | 6 | 37 | 태그 | ChampionActionState::Ult 태그(switch) | 4 |
| 9 | -1 | 37 | 센티널 | Option<Effect> None(casting 니치 -1) → unwrap_failed / Option<ObjectivePosture> None(focus_enemy 니치 -1, L58) | 4 |
| 10 | 1 | 37 | 태그 | CastingType::Position(1) — 논타겟 판정(1\|2 → is_in_range). 또 focus_enemy Some 태그(L70) · 저장값 move_check=1 | 4 |
| 11 | 2 | 37 | 임계 | CastingType::Direction(2) — 논타겟 / Entity.level > 2 → skill2_effect(entity.rs:1693) / ObjectivePostureKind: >2(WaitGroup·SoftDisengage) · ==2(Screen) (L59·L70) | 4 |
| 12 | 3 | 52 | 태그 | SmallActionPlay::RunAway 메모리태그(store i8 3 @+177) — L52·L61·L95 | 4 |
| 13 | 14 | 72 | 태그 | SmallActionPlay::Trace 메모리태그(store i8 14 @+177) — L72 | 4 |
| 14 | 5 | 52 | 태그 | end_delay=5 — RunAway::new_with_skill(L52·L61)·RunAway::new(L95)·AroundPosition::new(L63·L80·L84·L87)·Trace::new_attack_range(L72)·battle_action(_end_delay, L99) | 4 |

**`knobs` 조정점 5건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | Rhino 캠프 도달 판정 반경 | serpen_check.rs:25 | 4900000001 | 올리면(반경↑) 적 진영에서 더 멀리서도 move_check 가 켜져 바로 서펜 캠프로 향한다. 내리면 Rhino 캠프에 더 붙어야 통과 | 4 | 기존 |
| 1 | 서펜 캠프 원거리 임계 | serpen_check.rs:78 | 22500000000 | 올리면 더 먼 거리까지 '근거리'로 보아 바로 서펜 캠프 주변 대기(AroundPosition camp) — move_check 미충족 시 Rhino 경유 분기가 줄어든다 | 4 | 기존 |
| 2 | 근접 적 도주 반경 | serpen_check.rs:91~92 | 22500000001 | 올리면 더 먼 적(최근 시야 챔피언·others)에도 RunAway 후보를 추가 → 대기 자리에서 자주 물러남 | 4 | 기존 |
| 3 | position_eval purpose | serpen_check.rs:48 | 11 | Objective 프로파일. 바꾸면 on_trajectory 판정 컨텍스트가 바뀜 | 4 | 기존 |
| 4 | end_delay | serpen_check.rs:52,61,63,72,80,84,87,95,99 | 5 | 생성되는 모든 후보의 end_delay 인자. 의미는 각 SmallAction 생성자 명세 | 4 | 기존 |

<details><summary>`callees` 피호출자 27건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | action_candidates | game_ai::plan_legacy::sub_plan::SubPlan::action_candidates | pub | fn(&mut game_ai::plan_legacy::sub_plan::SubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\mod.rs:118 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 18개 중 상위 3개 |
| 1 | action_candidates | game_ai::plan_legacy::sub_plan::HideSubPlan::action_candidates | pub | fn(&mut game_ai::plan_legacy::sub_plan::HideSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\hide.rs:19 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 18개 중 상위 3개 |
| 2 | action_candidates | game_ai::plan_legacy::sub_plan::StealSubPlan::action_candidates | pub | fn(&mut game_ai::plan_legacy::sub_plan::StealSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\steal.rs:33 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 18개 중 상위 3개 |
| 3 | attack_summon_action | game_ai::attack_summon_action | pub | fn(&game_core::PlayerState, &game_core::OperationData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\fight_check.rs:836 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | battle_action | game_ai::battle_action | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, usize) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\fight_check.rs:620 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 5 | camp_pos | game_core::MapDef::camp_pos | pub | fn(&game_core::MapDef, game_core::JungleType, bool) -> (u64, u64) | game-core\src\simulation\map_def.rs:207 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 6 | is_blue_side | game_core::is_blue_side | pub | fn(&game_core::GameContext, u64, u64) -> bool | game-core\src\simulation\map_regions.rs:6 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 7 | is_enemy_side | game_core::is_enemy_side | pub | fn(&game_core::GameContext, usize, u64, u64) -> bool | game-core\src\simulation\map_regions.rs:57 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 8 | is_in_range | game_core::Effect::is_in_range | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\effect.rs:63 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 9 | is_recent_visible | game_core::Blackboard::is_recent_visible | pub | fn(&game_core::Blackboard, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\game\blackboard.rs:346 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 10 | is_visible | game_core::World::is_visible | pub | fn(&game_core::World, usize, usize) -> bool | game-core\src\simulation\game\data.rs:384 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 11 | is_visible | game_core::VisibleState::is_visible | pub | fn(&game_core::VisibleState) -> bool | game-core\src\simulation\game\data.rs:121 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 12 | is_visible | game_core::AbstractGame::is_visible | pub | fn(&Self/#0, usize, usize) -> bool | game-core\src\simulation.rs:125 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 13 | new | game_ai::SmallActionRunAway::new | pub | fn(&game_core::OperationData, &game_core::PlayerState, usize) -> game_ai::SmallActionRunAway | game-ai\src\small_action\move_actions.rs:25 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 14 | new | game_ai::SmallActionAroundPosition::new | pub | fn(&mut rand::rngs::std::StdRng, &game_core::OperationData, u64, u64, usize) -> game_ai::SmallActionAroundPosition | game-ai\src\small_action\around.rs:831 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 15 | new_attack_range | game_ai::SmallActionTrace::new_attack_range | pub | fn(&game_core::OperationData, usize, usize) -> game_ai::SmallActionTrace | game-ai\src\small_action\trace.rs:75 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 16 | new_with_skill | game_ai::SmallActionRunAway::new_with_skill | pub | fn(&game_core::OperationData, &game_core::PlayerState, usize, bool) -> game_ai::SmallActionRunAway | game-ai\src\small_action\move_actions.rs:29 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 17 | nontarget_windup_perceived | game_ai::nontarget_windup_perceived | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity) -> bool | game-ai\src\utils.rs:553 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 18 | position_score_at_position | game_ai::position_score_at_position | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, u64, u64, game_ai::PositionEvalPurpose) -> game_core::PositioningScore | game-ai\src\position_eval.rs:1192 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 19 | push | <game_core::Staff as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::staff | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\staff.rs:38 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 20 | push | <game_core::Athlete as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:434 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 21 | push | <game_core::Contract as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:2498 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 22 | team | game_view::ClientData::team | pub | fn(&game_view::ClientData, usize) -> std::option::Option<std::cell::Ref< game_core::Team>> | game-view\src\logic\client\data.rs:1651 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 23 | team | game_view::ClientDatabase::team | pub | fn(&game_view::ClientDatabase, usize) -> std::option::Option<&game_core::Team> | game-view\src\logic\client\data.rs:968 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 24 | team | game_core::PlayerAiContext::<'a, 'b, 'r>::team | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:583 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 25 | v25_objective_posture | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v25_objective_posture | pub | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::PlayerState, &game_core::OperationData, game_core::JungleType) -> std::option::Option<game_ai::plan_legacy::team_plan::ObjectivePosture> | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:207 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 26 | width | game_view::ui::team_history_common::view::Measure::<'a>::width | pub | fn(&game_view::ui::team_history_common::view::Measure<'a/#0>, &str, &str, f32) -> f32 | game-view\src\ui\team_history_common\view.rs:149 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
</details>

⚠**미매칭 11개**: `entry`, `extend`, `format_inner`, `grow_one`, `insert_no_grow`, `move_check`, `on_periodic_trajectory`, `on_trajectory`, `or_default`, `reserve_internal_or_panic`, `rustc_entry`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m12.ll:35248) · **형제 7개** (SerpenCheckSubPlan)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::plan_legacy::sub_plan::SerpenCheckSubPlan as std::clone::Clone>::clone | pub | game-ai\src\plan_legacy\sub_plan\serpen_check.rs:8 | True | fn(&game_ai::plan_legacy::sub_plan::SerpenCheckSubPlan) -> game_ai::plan_legacy::sub_plan::SerpenCheckSubPlan |
| 1 | <game_ai::plan_legacy::sub_plan::SerpenCheckSubPlan as std::default::Default>::default | pub | game-ai\src\plan_legacy\sub_plan\serpen_check.rs:8 | True | fn() -> game_ai::plan_legacy::sub_plan::SerpenCheckSubPlan |
| 2 | <game_ai::plan_legacy::sub_plan::SerpenCheckSubPlan as std::fmt::Debug>::fmt | pub | game-ai\src\plan_legacy\sub_plan\serpen_check.rs:8 | True | fn(&game_ai::plan_legacy::sub_plan::SerpenCheckSubPlan, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 3 | game_ai::plan_legacy::sub_plan::SerpenCheckSubPlan::action_candidates | pub | game-ai\src\plan_legacy\sub_plan\serpen_check.rs:14 | False | fn(&mut game_ai::plan_legacy::sub_plan::SerpenCheckSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 4 | game_ai::plan_legacy::sub_plan::SerpenCheckSubPlan::calculate_score_parameter_value | pub | game-ai\src\plan_legacy\sub_plan\serpen_check.rs:106 | True | fn(&game_ai::plan_legacy::sub_plan::SerpenCheckSubPlan, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::ScoreParameter) |
| 5 | game_ai::plan_legacy::sub_plan::SerpenCheckSubPlan::score | pub | game-ai\src\plan_legacy\sub_plan\serpen_check.rs:123 | False | fn(&game_ai::plan_legacy::sub_plan::SerpenCheckSubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 |
| 6 | game_ai::plan_legacy::sub_plan::SerpenCheckSubPlan::merge | pub | game-ai\src\plan_legacy\sub_plan\serpen_check.rs:143 | True | fn(&mut game_ai::plan_legacy::sub_plan::SerpenCheckSubPlan, game_ai::plan_legacy::sub_plan::SerpenCheckSubPlan) |

**`open` 5건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 표기 불가 | L25 `self.move_check = true` store 의 dbg 루트가 line 0 이라 'else { move_check=true }' 의 정확한 소스 줄(줄 안 순서) 은 표기 불가 — 동작(아군 진영이면 무조건 true, 적 진영이면 Rhino 70000 이내일 때 true) 은 IR 분기(m14.ll:30380 br %79)로 확정 | 4 |  |
| 1 | 미탐색 | @anon.b0108feec1ab8ff62b7a37c1a95c251f.22 (level 미달 시 쓰는 정적 값, m14.ll:28) = casting 바이트 FF FF FF FF = Option<Effect>::None ⟹ 적이 level≤2 로 Skill2 상태 / level≤4 로 Ult 상태면 switch -1 → unwrap_failed(패닉). 실전 도달 가능 여부(그 레벨에서 그 액션 상태가 가능한가)는 game_core 규칙이라 미확인 | 4 |  |
| 2 | 미탐색 | PositioningScore +0x31 의 DI 이름이 on_trajectory 인 이유(소스에 `let on_trajectory = ps.on_periodic_trajectory`류 지역변수?)는 확정 못 함. tcx: +0x30 on_trajectory · +0x31 on_periodic_trajectory 를 정본으로 적음 | 3 |  |
| 3 | 미탐색 | camp_pos 는 game_core TLS CAMP_POS_MEMO 를 쓰는 함수(g07.ll:152570) — 그 캐시 키/무효화는 이 명세 범위 밖 | 4 |  |
| 4 | 미탐색 | battle_action / attack_summon_action 의 extend 원소 variant 집합은 각 명세(r14/r13) 참조 — 여기서는 sret 32B 헤더(ptr@0,len@0x18)만 읽어 memcpy | 4 |  |

<details><summary>`closed` 1건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

| # | 물음 | 닫은 근거 |
|---|---|---|
| 0 | L36 `c.ty==Champion(13)` 비교(m14.ll:30485~30487)에 dbg 가 없어 nontarget_windup_perceived 와의 소스 순서(A && B 의 B 가 is_in_skill 내부인지 별도 술어인지)는 표기 불가 — 평가 순서는 IR 로 확정(nontarget 호출 후 tag 비교) | 본문에 해소 표기가 있다 |
</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

