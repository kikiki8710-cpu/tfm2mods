---

### `189` DefenseNexusSubPlan::action_candidates — 넥서스 방어 서브플랜의 행동 후보 루트(vtable 진입): 결사 대상 커밋(Trace)→위험시 도주 조기귀환(스킬)→미니언 웨이브 피해 도주→본진 포지셔닝(Around)→RunAway·전투·근접 미니언 공격(Attack/Skill/Skill2)·소환수·최근접 적 타워 공격·구조물 스킬 후보를 sret Vec<SmallActionPlay> 로 돌려준다

| 항목 | 값 |
|---|---|
| id | `defense_nexus__DefenseNexus__action_candidates` |
| 심볼 | `_RNvMs_NtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy8sub_plan13defense_nexusNtB4_19DefenseNexusSubPlan17action_candidates` |
| 소스 | `game-ai\src\plan_legacy\sub_plan\defense_nexus.rs:245` |
| IR | `m14.ll` 45282~48978행 |
| 경로·가시성 | `game_ai::plan_legacy::sub_plan::DefenseNexusSubPlan::action_candidates` · **pub** |
| 계층 | 기타 |
| exe | `e928f0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r16` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&mut game_ai::plan_legacy::sub_plan::DefenseNexusSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay>
```

**TLS 메모 절(`sig.tls` · 정정 경로 `/specs[189]/sig/tls/<키>`)**

- {"name": "(직접 접점 없음)", "role": "본체 45282~48978 grep: LocalKey/with/thread_local 0건 — 이 함수 본문은 TLS 를 직접 읽거나 쓰지 않는다", "key": "-", "layout": "-", "invalidation": "-", "call_conditions": "TLS 접점은 전부 콜리 내부. 1단 콜리 중 TLS 를 가진 것과 이 함수 안의 호출 순서(미러 재현 시 유지): ①base_defense_focus(L36, 45402; 조건 version≥2) — old::defense_nexus LAST_STAND_MEMO(LocalKey<RefCell<{borrow, seed, tick, HashMap<player.info.id(+0x928), (bool,bool,bool)>}>>, m00.ll:90215 last_stand_flags::closure$0; (seed,tick) 바뀌면 전체 무효화) 소비자 ②base_attacking_minion(L45, 45449; 조건 champ Some) — 같은 LAST_STAND_MEMO 소비자(3번째 bool 이 set 일 때만 base_attacking_minion_uncached 호출, m04.ll:59498~59507) ③check_kill_die_tick(L68, 45835; minion None 이고 threats 비어있지 않을 때 threats 원소마다 — 첫 원소 본체, 나머지 m12.ll:25039 fold 안) — fight_check DieTickCache(LocalKey<RefCell<DieTickCache>>, m15.ll:27257·27414) 작성/소비 ④base_defense_focus(L253, 46067; version>1) 재호출 ⑤nontarget_windup_perceived(L260, 46163; 적 챔피언마다 최대 5회) — TLS 직접 접점 0(m04.ll:53108~53231) ⑥position_score_at_position(L272, 46285; champ Some 이면 무조건 1회, purpose=General(2)) — 내부에서 position_eval_at 호출(m07.ll:34232~34265) = POS_EVAL_CACHE 작성/소비는 그 안 ⑦enemy_minion_wave_danger_damage_at(L284, 46314; !(도주 조기귀환) 일 때) — TLS 직접 접점 0 ⑧nexus_is_critical(L290, 46393) — TLS 직접 접점 0(nexus_under_direct_attack 경유) ⑨battle_action(L302, 47484) — 직접 접점 0(내부 2단 콜리는 미확인) ⑩attack_summon_action(L304) · can_tower_focused_when_attack(L222, 48787) · attack_structure_skill_action(L306) — 직접 접점 0. v47_siege_stance/v48_cast_beams/champion_hp_value/interaction_score 심볼은 이 함수 IR·1단 콜리 본문에 0건(2단 이하는 미확인)"}

<details><summary>인자 8개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | bumpalo Vec<SmallActionPlay> 32B | ptr@0 · bump@+8 · cap@+0x10 · len@+0x18. 본체는 로컬 %66(res) 를 new_in(bump)(45365~45371: ptr=8 dangling·bump=context.pool·cap=len=0) 으로 만들고 정상 종료 3곳에서 32B memcpy 로 sret 채움(46377 조기귀환 L279/L294 · 48976 L308) | 4 |
| 1 | 1 | self | &mut DefenseNexusSubPlan(24B: +0 focus@tag(Option<usize>: 0 None/1 Some) · +8 focus@Some.0 · +0x10 last_gate u8) | IR 속성 initializes((16,17)) = 무조건 쓰는 표면은 +0x10 last_gate 1B 뿐(L247 45372). focus(+0/+8) 는 조건부 쓰기(commit_chase L37·L90 — 아래 writes 전수). 읽기: +0 태그 45517(L76)·46411(L300, L117/L141 에 재사용) · +8 45519(L76)·47497(L141) | 4 |
| 2 | 2 | version | usize | 분기 2곳: L36 `version < 2`(45397, commit_chase 면제) · L253 `version > 1`(46063). reach.txt(version=2·gamemode=0) 는 두 분기를 접음(%77→0 · %284→1) = 현행 버전에선 항상 focus 경로·last_stand 평가. 그 외엔 콜리 인자(check_kill_die_tick·nontarget_windup_perceived·position_score_at_position·enemy_minion_wave_danger_damage_at·Around::new·battle_action) — %54 alloca 에 spill(45377) 해 closure$1 env 로 전달 | 4 |
| 3 | 3 | rnd | &mut StdRng(320B, align 16) | 직접 소비 없음. closure$1 env(45729)→check_kill_die_tick(45835) · Around::new(46974·47131·47197·47205) · battle_action(47484) 에 전달 | 4 |
| 4 | 4 | player | &PlayerState(2528B) | +0x930 info.team(45421~45423 L40 · 46073~46075 L256, bounds<2) · +0x9c0 info.position@tag(as_index 45435~45437 · 46087~46089). 콜리 인자 다수 | 4 |
| 5 | 5 | data | &OperationData(24B) | +0 cache(&AbstractGameWithCache) · +8 context(&GameContext: +0 pool=&Bump 45361~45363 · +8 setting 46309~46310) · +0x10 blackboard(&[Blackboard;2] 46684~46686, 적 팀 인덱스) | 4 |
| 6 | 6 | parameter | &ScoreParameter(5384B) | 단 1곳: +0x9f0 positioning_score(&PositioningScoreData 2760B) 를 L272 position_score_at_position 인자로(46280). 그 외 읽지 않음 | 4 |
| 7 | 7 | debug | &mut DebugFrameData(224B) (DI ref_mut$<DebugFrameData>) | readonly 없음 = &mut. 이 함수는 직접 읽고 쓰지 않는다 — closure$1 env(45731)→check_kill_die_tick 8번째 인자(45835) 로만 전달 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn action_candidates(&mut self, version, rnd, player, data, parameter, debug) -> Vec<SmallActionPlay>   [defense_nexus.rs:245~309]
L246 let mut res = Vec::new_in(data.context.pool)                                        // 45361~45371
L247 self.last_gate = 0                                                                 // 45372 (initializes 표면)
L250 res.extend(self.commit_chase(version, rnd, player, data, debug))                    // 인라인 45392~46059, 소스 L35~95:
  L35  let mut r = Vec::new_in(bump)
  L36  if version < 2 || !base_defense_focus(player, data) { L37 self.focus = None; return r }     // 45397~45406 (version<2 는 reach 접힘)
  L40  let Some(champ) = data.cache.player_champion[team][pos] else { return r }          // 45440~45445
  L45  let minion: Option<usize> = base_attacking_minion(player, data)                    // 45449 (LAST_STAND_MEMO 게이트 → uncached)
  L51  let champ_target: Option<usize> = if minion.is_some() { None } else {              // 45459
  L52    let nexus = data.cache.nexus[team]; L53 let twins = &data.cache.twin_towers[team];
  L54~56 let threats: Vec<&Entity> = iter_champions(1−team).filter(|e| e.attack_effect.map_or(false, |atk| nexus.map_or(false,|n| atk.is_in_range(e,n)) || twins.iter().any(|t| atk.is_in_range(e,t)))).collect_in(bump)   // aux closure$0
  L59~60 threats.iter().min_by_key(|e| {
  L61~63   let attackers = iter_champions(team).filter(|a| dist_sq(a,e) <= 120000²).collect_in(bump)      // aux closure$1::closure$0
  L64~67   let towers = iter_towers(team).filter(|t| t.can_target && t.block_target_tick==0 && t.attack_effect.map_or(false,|atk| atk.is_in_range(t,e))).collect_in(bump)   // aux closure$1::closure$1
  L68      let die = check_kill_die_tick(version, rnd, data, player, e, attackers, towers, debug)   // 45835
  L69      (die, dist_sq(e, champ)) })                                                    // 튜플 사전순: 빨리 죽는 위협 → 가까운 위협
  L71    .map(|c| c.id) }                                                                 // 45907~45909
  L74  let want = minion.or(champ_target)                                                 // 45509~45516
  L76  let kept = self.focus.filter(|id| {                                                // 45517~45529
  L77    let Some(e) = data.cache.get_entity_by_id(id) else { return false }              // vtable+0x1f0, 45536~45552
  L78    if minion.is_some() { L80 is_base_attacking_minion(player, data, e) }            // 45677
  L82    else { e.team == Player(1−team) && e.ty == Champion && e.attack_effect.is_some()  // 45566~45591
  L83~85        && (nexus.map_or(false,|n| atk.is_in_range(e,n)) || twins.iter().any(|t| atk.is_in_range(e,t))) } })   // 45610~45674
  L90  self.focus = kept.or(want)                                                         // 45968~45969 / 45984~45985
  L91  if let Some(id) = self.focus { L92 r.push(Trace(SmallActionTrace::new(data, id, 5))) }   // 45994~46045
  L95  r
L253 let last_stand = version > 1 && base_defense_focus(player, data)                      // 46063~46071
L256 let champ = data.cache.player_champion[team][pos].unwrap()                            // 46073~46098 (None → 패닉)
L259 let has_non_target_action_range = iter_champions(1−team).any(|c|                     // 46103~46277
  L260   nontarget_windup_perceived(version, player, data, c) && c.ty == Champion &&        // 46163~46170
  L261   { let eff = match c.action_state { Skill(4) => c.skill_effect().unwrap(), L263 Skill2(5) => c.skill2_effect().unwrap()  /* level>2 ? Some : None(→패닉) */, L265 Ult(6) => c.ult_effect().unwrap() /* level>4 */, _ => return false };
         matches!(eff.casting, Position(1)|Direction(2)) && eff.is_in_range(c, champ) })    // 46196~46262 switch · 46175
L272 let position_score = position_score_at_position(version, player, data, &parameter.positioning_score, champ.x, champ.y, PositionEvalPurpose::General)   // 46285, sret 56B PositioningScore
L274 let on_trajectory = position_score.on_trajectory(+0x30) || position_score.on_periodic_trajectory(+0x31)   // 46289~46297
L277 if !last_stand && (on_trajectory || has_non_target_action_range) {                    // 46299~46306 (IR: +0x30 참 → last_stand 만 판정 / 거짓 → last_stand || !(windup || +0x31))
L278   self.last_gate = 1                                                                  // 46318
L279   res.push(RunAway(SmallActionRunAway::new_with_skill(data, player, 5, true))); return res }   // 46321~46377
L284 let minion_wave_damage = enemy_minion_wave_danger_damage_at(version, data, champ, champ.x, champ.y, L285 setting.tick_per_second*2)   // 46309~46314
L286 let hp_ratio = champ.hp*100 / max(champ.stat_cached.hp, 1)                            // 46383~46391
L290 let critical = nexus_is_critical(player, data)                                        // 46393
L291 if minion_wave_damage != 0 && (minion_wave_damage >= champ.hp || hp_ratio < 46) {     // 46398 · 47340~47343
L292   if !last_stand && !(critical && minion_wave_damage < champ.hp) {                    // 47346~47354 (critical: last_stand||dmg<hp 면 통과 / 비critical: last_stand 면 통과)
L293     self.last_gate = 2                                                                // 47357
L294     res.push(RunAway(SmallActionRunAway::new(data, player, 5))); return res } }        // 47360~47413
L300 res.extend(self.base_positioning(version, rnd, player, data))                          // 인라인 46402~47424, 소스 L98~135:
  L98  let mut r = Vec::new_in(bump); L100 champ = player_champion[team][pos].unwrap()
  L101 let enemy_minions = data.cache.minions(1−team, bump)                                  // 46443 sret Vec<&Entity>
  L102 let twins = &data.cache.twin_towers[team]
  L103~104 let attacked_twin_tower = twins.iter().filter(|t| enemy_minions.iter().any(|m| m.ty==Minion && m.Minion.info.nearest_enemy == Some(t.id))).min_by_key(|t| dist_sq(t, champ))   // 46510~46659 + aux m12:19614
  L105 let nexus = data.cache.nexus[team].unwrap()                                          // 46671~46679
  L108~113 let bb = &data.blackboard[1−team]; top/mid/bottom_front_minion = bb.{top,mid,bottom}_minion_state.front_minion.and_then(|id| get_entity_by_id(id))   // 46684~46759
  L114~115 let front_minions: Vec<&Entity> = vec![top,mid,bottom].into_iter().filter_map(|m| m).collect_in(bump)   // 46778~46818
  L117 if self.focus.is_none() {                                                            // 46825 (Some 이면 r 빈 채로 L133 반환)
  L118   let nearest_front_minion = front_minions.iter().min_by_key(|m| dist_sq(m, nexus))  // 46857~46961
  L119   if let Some(m) = nearest_front_minion { L120 r.push(Around(SmallActionAround::new(version, rnd, data, player, m.id, 5))) }   // 46971~47025
  L123   let twin_tower = twins.iter().min_by_key(|t| dist_sq(t, champ))                    // 47043~47112
  L124   if let Some(t) = attacked_twin_tower { L125 r.push(Around(new(.., t.id, 5))) }      // 47115~47131
  L126   else if let Some(t) = twin_tower { L127 r.push(Around(new(.., t.id, 5))) }          // 47135~47197
  L128   else { L129 r.push(Around(new(.., nexus.id, 5))) } }                                // 47203~47205
  L133 r
L301 res.push(RunAway(SmallActionRunAway::new(data, player, 5)))                            // 47430~47481
L302 res.extend(battle_action(version, rnd, player, data, 5))                              // 47484~47491 (sret Vec 32B)
L303 res.extend(self.attack_minion_actions(player, data))                                   // 인라인 47496~48253, 소스 L137~195:
  L137 champ = player_champion[team][pos].unwrap()
  L141~142 let focus_near = self.focus.filter(|id| get_entity_by_id(id).map_or(false, |e| dist_sq(e, champ) < 80000²))   // 47529~47594
  L143 let minions = data.cache.iter_minions(1−team)                                        // 47599 sret 56B Chain 이터레이터
  L147 let mut r = Vec::new_in(bump); L148 let speed = champ.stat_cached.move_speed
  L150 for m in minions.filter(|m| focus_near.map_or(true, |f| m.id == f) && dist_sq(m, champ) < 80000²) {   // aux m11:27026 · m14:60826
  L151   if !m.is_visible_from(champ) { continue }                                          // 47688~47705
  L155   if champ.can_attack() {                                                            // 47715
  L156     let atk = champ.attack_effect.as_ref().unwrap()                                  // 47729~47746
  L158     let range = atk.range(champ) + atk.range_adjust(champ, m) + champ.radius() + speed*30   // 47737~47829
  L162     let range = range + m.radius()                                                   // 47827
  L163     if dist_sq(m, champ) <= range² { L164 r.push(Attack(SmallActionAttack::new(data, m.id))) } }   // 47831~47845
  L168   if let Some(sk) = champ.skill_effect.as_ref() {                                    // 47723~47725
  L169     if champ.can_skill() && sk.target.check(champ, m) {                              // 47897 · 47914
  L170       let range = sk.range(champ) + sk.range_adjust(champ, m) + champ.radius() + speed*30; L174 + m.radius()   // 47921~48008
  L175       if dist_sq(m, champ) <= range² { L176 r.push(Skill(SmallActionSkill::new(data, m.id))) } } }   // 48010~48024
  L181   if let Some(sk2) = champ.skill2_effect() /* level>2 */ {                           // 47901~47908
  L182     if champ.can_skill2() && sk2.target.check(champ, m) {                            // 48076 · 48087
  L183       let range = sk2.range(champ) + sk2.range_adjust(champ, m) + champ.radius() + speed*30; L187 + m.radius()
  L188       if dist_sq(m, champ) <= range² { L189 r.push(Skill2(SmallActionSkill2::new(data, m.id))) } } } }   // 48184~48198
  L195 r
L304 res.extend(attack_summon_action(player, data))                                        // 48258~48265
L305 res.extend(self.attack_tower_action(player, data))                                     // 인라인 48279~48945, 소스 L199~223, 반환 Option<SmallActionPlay>:
  L199 let champ = player_champion[team][pos]?                                              // 48279~48282
  L200~202 let tower = iter_towers(1−team).filter(|t| t.can_target && t.block_target_tick==0).min_by_key(|t| dist_sq(t, champ))?   // 48290~48635 + aux
  L204 let speed = champ.stat_cached.move_speed; L205 let atk = champ.attack_effect.as_ref()?   // 48641~48651
  L206 let ally_minion_in_my_range = data.cache.iter_minions(team).any(|m| atk.is_in_range(champ, m))   // 48655~48662 + aux m11:30573
  L209 let d2 = dist_sq(tower, champ)                                                       // 48668~48697
  L210 let range = atk.range(champ) + atk.range_adjust(champ, tower) + champ.radius() + L211 speed*30 + tower.radius()   // 48698~48766
  L212 if d2 > range² { return None }                                                       // 48767~48769
  L216 let tower_dmg = tower.attack_effect.map_or(0, |e| e.expected_damage_target(data.context, tower as &dyn, champ))   // 48773~48785
  L222 if !can_tower_focused_when_attack(data.context, data.cache, player, tower) && ally_minion_in_my_range && champ.can_attack() && champ.hp > tower_dmg {   // 48787~48806 (IR: focused||!any → None 먼저)
  L223   return Some(Attack(SmallActionAttack::new(data, tower.id))) }                       // 48811~48816
  else None
L306 res.extend(attack_structure_skill_action(player, data))                               // 48964~48971
L308 res   (sret memcpy 48976)
```

**`mem` 메모리 접근 46건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | DefenseNexusSubPlan(self) | 0x0 | focus@tag (Option<usize>: 0 None / 1 Some) | r | L76 (45517) commit_chase 의 self.focus.filter · L300 (46411) base_positioning L117 `focus.is_none()` 분기(46825) · L141 (47529) attack_minion_actions | 4 | OK |  |
| 1 | DefenseNexusSubPlan(self) | 0x8 | focus@Some.0 (엔티티 id) | r | L76 (45519) get_entity_by_id 인자 · L141 (47497) | 4 | OK |  |
| 2 | PlayerState | 0x930 | info.team | r | L40 (45421~45423) · L256 (46073~46075) bounds<2 아니면 panic_bounds_check. 1−team = 적 팀(45472·46103) | 4 | OK |  |
| 3 | PlayerState | 0x9c0 | info.position@tag (as_index) | r | L40 (45435~45437) · L256 (46087~46089) | 4 | OK |  |
| 4 | OperationData | 0x0 | cache (&AbstractGameWithCache) | r | 45438 · 46090 | 4 | OK |  |
| 5 | OperationData | 0x8 | context (&GameContext) | r | 45361~45362 pool · 46309 setting | 4 | OK |  |
| 6 | OperationData | 0x10 | blackboard (&[Blackboard;2]) | r | L108 (46684~46686) 적 팀(1−team) 원소 | 4 | OK |  |
| 7 | GameContext | 0x0 | pool (&Bump) | r | 45363 — 모든 로컬 bumpalo Vec 의 할당자(res %66·%53·%38·%22·threats·attackers·towers·enemy_minions·front_minions) | 4 | OK |  |
| 8 | GameContext | 0x8 | setting (&GameSetting) | r | L285 (46309~46310) | 4 | OK |  |
| 9 | GameSetting | 0x12f8 | tick_per_second | r | L285 (46311~46313) ×2 (shl 1) = enemy_minion_wave_danger_damage_at 의 window 인자 | 4 | OK |  |
| 10 | AbstractGameWithCache | 0x0 | game (&dyn AbstractGame 팻포인터: +0 data_ptr · +8 vtable_ptr) | r | vtable+0x1f0 = get_entity_by_id(id) → Option<&Entity>(null=None): L77 (45536~45541) · L109/L111/L113 (46691~46693·46709~46711·46731~46733·46753~46755) · L141 (47533~47542) | 4 | OK |  |
| 11 | AbstractGameWithCache | 0x1e0 | player_champion[team][pos] (Option<&Entity>, null=None) | r | L40 (45440~45445, None → 빈 res 반환) · L256 (46092~46098, None → unwrap_failed 패닉 46273 — `?` 아님) · L100 (46435~46447 unwrap) · L137 (47506~47509 unwrap) · L199 (48279~48282, None → attack_tower_action None) | 4 | OK |  |
| 12 | AbstractGameWithCache | 0x170 | nexus[team] (Option<&Entity>, stride 8) | r | L52 (45464~45467) 위협 필터 캡처 · L84 (45610~45612) · L105 (46671~46675 unwrap → nexus) | 4 | OK |  |
| 13 | AbstractGameWithCache | 0x130 | twin_towers[team] (bumpalo Vec<&Entity>: ptr@+0 · len@+0x18, stride 32) | r | L53 (45474~45476) 캡처 · L85 (45629~45636 순회) · L102 (46451~46458) · L123 (47043~47046) | 4 | OK |  |
| 14 | Blackboard(적 팀) | 0x0 | top_minion_state.front_minion@tag / +0x8 값 | r | L108~109 (46686~46713) and_then(get_entity_by_id) → top_front_minion | 4 | OK |  |
| 15 | Blackboard(적 팀) | 0x28 | mid_minion_state.front_minion@tag / +0x30 값 | r | L110~111 (46718~46734, blackboard.rs:381 minion_state(Mid) 인라인) → mid_front_minion | 4 | OK |  |
| 16 | Blackboard(적 팀) | 0x50 | bottom_minion_state.front_minion@tag / +0x58 값 | r | L112~113 (46740~46756, blackboard.rs:382) → bottom_front_minion | 4 | OK |  |
| 17 | ScoreParameter | 0x9f0 | positioning_score (&PositioningScoreData 2760B) | r | L272 (46280) position_score_at_position 인자 | 4 | OK |  |
| 18 | Entity(e: 적 챔피언/미니언/타워) | 0x0 | team@tag (TeamType 0 Player/1 Neutral) | r | L82 (45566~45568 player_team) · L151 (47688~47690 is_visible_from(champ) — champ.team) 등 | 4 | OK |  |
| 19 | Entity | 0x8 | team@Player.0 | r | L82 (45571~45585) `e.team == Player(1−team)` · L151 (47694~47697) is_visible_from 팀 인덱스 | 4 | OK |  |
| 20 | Entity(m: 적 미니언) | 0x38 | visible_state[team]@tag (0 Visible) | r | L151 (47701~47705) 비가시면 continue | 4 | OK |  |
| 21 | Entity | 0x68 | ty@tag (EntityType) | r | L82 (45582~45584) ==13 Champion · L260 (46167~46169) ==13 · L103 (46557~46559) ==1 Minion (aux m12.ll:19614 동형) | 4 | OK |  |
| 22 | Entity(c: 적 챔피언) | 0x70 | ty@Champion.info.action_state@tag (ChampionActionState) | r | L261 (46194~46200, entity.rs:1572 is_in_skill) switch 4 Skill / 5 Skill2 / 6 Ult, 그 외 false | 4 | OK |  |
| 23 | Entity(m: 적 미니언) | 0x88 | ty@Minion.info.nearest_enemy@tag / +0x90 값 (Option<usize>) | r | L103 (46566~46578) == Some(t.id) — 어느 쌍둥이 타워를 노리는 미니언이 있는가 | 4 | OK |  |
| 24 | Entity | 0x4c0 | attack_effect@tag (niche, −1=None) | r | L82 (45587~45589) · L156 (47729~47731 unwrap) · L205 (48645~48648 `?`) · L216 (48773~48776 tower.attack_effect map_or 0) · aux 클로저 | 4 | OK |  |
| 25 | Entity | 0x490 | attack_effect@Some.0 (&Effect 56B) | r | L83 (45594) · L156 (47628) · L205 (48651) — is_in_range/range_adjust/expected_damage_target 인자 | 4 | OK |  |
| 26 | Entity(champ) | 0x4a0 | attack_effect.range / +0x4a8 growth_range | r | L158 (47737~47738) · L210 (48698~48701) Effect::range 인라인 | 4 | OK |  |
| 27 | Entity(champ) | 0x4f8 | skill_effect@tag (niche −1=None) | r | L168 (47723~47725) · L261 (46204~46210: switch −1 unwrap 패닉 / 1\|2 통과 / 그 외 false) | 4 | OK |  |
| 28 | Entity(champ) | 0x4c8 | skill_effect@Some.0 (&Effect) · +0x4d8 range · +0x4e0 growth_range · +0x4f0 target(CastingTarget) | r | L169~170 (47914 CastingTarget::check(&+0x4f0) · 47921~47925 range/growth/range_adjust) | 4 | OK |  |
| 29 | Entity(champ) | 0x5c8 | level | r | Effect::range 의 (level−1)×growth (47739·48702) · L181/L263 skill2_effect() = level>2 ? Some(+0x500) : None (47901~47903·46227~46231) · L265 ult_effect() = level>4 ? Some(+0x538) : None (46250~46254) | 4 | OK |  |
| 30 | Entity(champ) | 0x500 | skill2_effect@Some.0 (&Effect) · +0x528 target · +0x530 tag(niche) | r | L181~183 (47905~47908 tag · 48086~48087 target.check · 48099 range_adjust) | 4 | OK |  |
| 31 | Entity(champ) | 0x538 | ult_effect@Some.0 · +0x568 tag | r | L265 (46253~46258) 적 챔피언 c 의 궁 이펙트 casting 검사 | 4 | OK |  |
| 32 | Entity | 0x5c0 | id | r | L71 (45908~45909 champ_target) · L103 (46545~46546) · L120/L125/L127/L129 Around target · L144 (aux) · L164/L176/L189/L223 Attack/Skill 타겟 | 4 | OK |  |
| 33 | Entity(champ) | 0x628 | stat_cached.hp (최대 HP) | r | L286 (46386~46391) hp_ratio 분모 max(·,1) | 4 | OK |  |
| 34 | Entity(champ) | 0x640 | stat_cached.move_speed | r | L148 (47614~47616) · L204 (48641~48642) ×30 을 사거리에 가산 | 4 | OK |  |
| 35 | Entity | 0x660 | x / +0x668 y | r | dist_sq 전부(45850~45863 · 46622~46642 · 46921~46934 · 47077~47090 · 47559~47572 · 47796~47810 · 48592~48605 · 48668~48681) · L273 champ 좌표(46281~46284) · L285 | 4 | OK |  |
| 36 | Entity(champ) | 0x670 | hp | r | L286 (46383~46385) ×100 · L291 (47341·47352 minion_wave_damage 와 비교) · L222 (48803~48805 > tower 기대피해) | 4 | OK |  |
| 37 | Entity | 0x680 | radius / +0x470 stat_buff_cached.radius_mult / +0x438 stat_buff_cached.range | r | Entity::radius() 인라인(47752~47771 등: mult==0 ? radius : radius*(mult+100)/100) · Effect::range 의 stat range 항(47740·48706~48707) | 4 | OK |  |
| 38 | Entity(타워) | 0x6b9 | can_target / +0x6a0 block_target_tick | r | L66 (aux 58047~58055) · L201 (48442~48448 · 48554~48560 · aux 61066~61074) | 4 | OK |  |
| 39 | DefenseNexusSubPlan(self) | 0x10 | last_gate (u8) | w | 값 의미는 게이트 번호(0 통과 · 1 L277 게이트 · 2 L292 게이트). 소비처는 이 함수 밖(미확인) | 4 | OK | 0 (L247, 45372 무조건 — initializes((16,17)) 표면) → 1 (L278, 46318: 위험 도주 조기귀환 직전) / 2 (L293, 47357: 미니언 웨이브 도주 조기귀환 직전) |
| 40 | DefenseNexusSubPlan(self) | 0x0 | focus@tag | w | L40 champ None 경로(45445→%280)는 focus 를 건드리지 않는다 — 조건부 쓰기라 initializes 에 없음 | 4 | OK | 0 (L37, 45406: version<2 \|\| !base_defense_focus — 현행 접힘으로 !base_defense_focus 때만) / 1 (L90, 45968: 기존 focus 가 아직 유효) / select(minion.is_some, 1, champ_target.tag) (L90, 45984) |
| 41 | DefenseNexusSubPlan(self) | 0x8 | focus@Some.0 | w | None 경로(45984 에 tag 0 저장)에서도 +8 은 undef 페이로드를 그대로 쓴다(niche 없는 Option<usize> — sweep 비교 시 tag 0 이면 +8 은 죽은 슬롯) | 4 | OK | 기존 focus id(45969, L90 kept) / minion id 또는 champ_target id (45985, L90 = `kept.or(want)` 의 want 페이로드 %123 = select(minion.is_some, minion.id, champ_target.id)) |
| 42 | (sret) | 0x0 | Vec 헤더 32B | w |  | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) | memcpy ← %66 res (46377 조기귀환 · 48976 정상) |
| 43 | res (%66 로컬 Vec<SmallActionPlay>) | 0x0 | 헤더+원소 | w | push 경로 전부 cap==len 이면 reserve_internal_or_panic(…,1,true) 후 gep 원소 → memcpy 184 → len+1 | 4 | 확인불가(★모호: 동명 def_path 40개 [('game_ai::path_fi) | new_in(bump) 45365~45371 → extend(commit_chase res 46059) → [push RunAway 46349~46372 → return] → [push RunAway 47388~47411 → return] → extend(base_positioning 47424) → push RunAway(47458~47481) → extend(battle_action 47491) → extend(attack_minion 48253) → extend(summon 48265) → push Attack(tower) 48918~48945 → extend(structure_skill 48971) |
| 44 | %53 (commit_chase 로컬 res) / %38 (base_positioning res) / %22 (attack_minion_actions res) | 0x0 | 로컬 Vec 헤더+원소 | w |  | 4 | 확인불가(tcx 사전에 타입 없음) | 각각 new_in(bump) 후 push(Trace 46043~46045 / Around 47023~47025·47184~47186·… / Attack·Skill·Skill2 47876 근방·48055·48229) → extend 로 %66 에 편입 후 drop_glue |
| 45 | closure env / 로컬 alloca | - | %54 version spill · %52 nexus · %51 threats · %50/%43/%41/%46/%45/%26/%25/%24/%35/%21 클로저 env · %65 position_score 56B · %37 enemy_minions · %36 front_minions · %44/%42 attackers/towers | w | 부작용 없음 | 4 | 확인불가(오프셋 파싱 실패) | 콜리·fold 인자용 임시 |

**`consts` 상수 20건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev | folded_from |
|---|---|---|---|---|---|---|
| 0 | 2 | 36 | 태그 | `version < 2` 면 commit_chase 면제(45397; reach 접힘 %77→0 = 현행 항상 거짓) · bounds 검사 길이 2(팀) · L272 purpose=General 태그 2(46285 i8 2, tcxdict --enum PositionEvalPurpose) · L181/L263 `level > 2` skill2 해금(46229·47902) · L82/L261 casting 1\|2 (Position/Direction) | 3 |  |
| 1 | 1 | 253 | 태그 | `version > 1`(46063, reach 접힘 %284→1) · Option<usize> Some 태그 1(45459·45968·45977·46559 EntityType::Minion 태그 1) · tps `shl 1` = ×2(46313, folded_from 2) · casting Position 태그 1 · umax(max_hp,1) 분모 하한(46390) | 4 | 2 |
| 2 | 5 | 92 | 길이 | end_delay=5 — Trace::new(45994)·RunAway::new_with_skill(46321)·RunAway::new(47360·47430)·Around::new(46974·47131·47197·47205)·battle_action(47484) 공통 마지막 인자 · L261 switch case 5 = ChampionActionState::Skill2 · player_champion 배열 길이 5(stride 40) | 4 |  |
| 3 | 14 | 92 | 태그 | SmallActionPlay::Trace 메모리태그 14 (46000, tcxdict --enum SmallActionPlay) | 3 |  |
| 4 | 3 | 279 | 태그 | SmallActionPlay::RunAway 메모리태그 3 (46327·47366·47436) · L115 front_minions 임시 Vec 길이 3(46815) · &Entity 슬라이스 바이트 변환 `shl nuw nsw i64 %len, 3`(45696·46472·46518·46869 = len×8, 시프트량 3 — 판정 아님) | 4 | 8 |
| 5 | 15 | 164 | 태그 | SmallActionPlay::Attack 메모리태그 15 (47845 미니언 · 48816/48896 타워) | 4 |  |
| 6 | 16 | 176 | 태그 | SmallActionPlay::Skill 메모리태그 16 (48024) | 4 |  |
| 7 | 17 | 189 | 태그 | SmallActionPlay::Skill2 메모리태그 17 (48198) | 4 |  |
| 8 | 13 | 82 | 태그 | EntityType::Champion 메모리태그 13 (45584 focus 유효성 · 46169 L260 적 챔피언 술어) | 4 |  |
| 9 | 4 | 261 | 태그 | ChampionActionState::Skill 태그 4(46197) → skill_effect 사용 · L265 `level > 4` ult 해금(46252) | 4 |  |
| 10 | 6 | 261 | 태그 | ChampionActionState::Ult 태그 6(46199) → ult_effect 사용 · iter_towers Flatten 배열 길이 6(48414 assume) | 4 |  |
| 11 | -1 | 82 | 센티널 | Option<Effect> 니치 None(i32 −1: 45589·46207·46236·46259·47724·47730·48647·48775 등) · Effect::range 의 level−1(45813 대응 47750·48704) · 접힌 iterator 상태 −1/−2(48461·48502 Flatten 상태 센티널, 판정 아님) | 4 |  |
| 12 | 46 | 291 | 임계 | hp_ratio < 46(%) — 미니언 웨이브 피해가 있을 때 HP 46% 미만이면(또는 웨이브 피해 ≥ 현재 HP) 도주 게이트 진입 (47340) | 4 |  |
| 13 | 100 | 286 | 계수 | hp_ratio = hp*100/max(max_hp,1) (46385·46391) · Entity::radius 의 (radius_mult+100)/100 (47765~47767 등) | 4 |  |
| 14 | 30 | 158 | 계수 | move_speed × 30 을 평타/스킬/스킬2/타워 공격 사거리에 가산(47637 L148 값 · 48759 L211) — 30틱(0.5초@60tps) 동안 이동할 거리만큼 여유 | 4 |  |
| 15 | 6400000000 | 142 | 임계 | 80000² — L141~142 focus 엔티티가 champ 로부터 80k(2.5셀) 미만이면 focus_near 유지(47589) · L145 미니언 후보 거리 상한(aux 60865) | 4 |  |
| 16 | 14400000001 | 63 | 미상 | 120000²+1 — attackers 필터 dist_sq(a, e) ≤ 120000²(3.75셀) (aux m14.ll:58033) | 4 |  |
| 17 | 8 | 246 | 미상 | 빈 bumpalo Vec 의 dangling ptr 값 8(45365·45392·46429·47609) · &Entity 배열 stride(45696 shl 3) — 판정 아님 | 4 |  |
| 18 | 24 | 115 | 미상 | __rust_alloc(24, 8) = [Option<&Entity>;3] 임시 배열(46779~46784) — 판정 아님 | 4 |  |
| 19 | 40 | 40 | 태그 | player_champion[team] 행 stride 40(45485·45793) · Flatten 순회 상한 40(46189 = 5×8) — 판정 아님 | 4 |  |

**`knobs` 조정점 7건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 행동 end_delay | defense_nexus.rs:92·279·294·301·120·125·127·129·302 (리터럴 5, 9곳) | 5 | 올리면 넥서스 방어 후보(Trace/RunAway/Around/battle 계열)의 지속 틱이 늘어 재평가가 드물어지고, 내리면 매 틱에 가깝게 후보를 다시 고른다 | 4 | 기존 |
| 1 | 미니언 웨이브 도주 HP% 임계 | defense_nexus.rs:291 (m14.ll:47340) | 46 | 올리면 웨이브 피해가 있을 때 더 높은 HP 에서도 도주 게이트(L292)에 들어가 RunAway 조기귀환이 잦아지고, 내리면 더 낮은 HP 까지 버틴다(last_stand·critical 예외는 그대로) | 4 | 기존 |
| 2 | 미니언 웨이브 피해 예측 창 | defense_nexus.rs:285 (m14.ll:46313 shl 1) | 2 | tps×N 초 창. 올리면 더 긴 시간의 웨이브 누적 피해를 보므로 도주 게이트가 잦아지고, 내리면 눈앞 피해만 본다 | 4 | 기존 |
| 3 | focus/미니언 후보 거리 상한 | defense_nexus.rs:142·145 (m14.ll:47589 · aux 60865) | 6400000000 | 80000²(2.5셀). 올리면 더 먼 적 미니언도 Attack/Skill/Skill2 후보에 오르고 focus 가 더 멀어도 유지된다, 내리면 근접 미니언만 | 4 | 기존 |
| 4 | 결사 위협 처치시간 계산의 아군 반경 | defense_nexus.rs:63 (aux m14.ll:58033) | 14400000001 | 120000²+1. 올리면 더 먼 아군까지 attackers 에 넣어 check_kill_die_tick 이 위협을 더 빨리 죽는 것으로 보고, 내리면 가까운 아군만 계산에 넣는다 | 4 | 기존 |
| 5 | 사거리 이동 여유 계수 | defense_nexus.rs:158·170·183·211 (m14.ll:47637·48759 mul 30) | 30 | move_speed×30 을 사거리에 더한다. 올리면 더 먼 미니언/타워도 공격 후보가 되고(접근 중 도달 가정), 내리면 지금 사거리 안만 | 4 | 기존 |
| 6 | 포지션 평가 purpose | defense_nexus.rs:272 (m14.ll:46285 i8 2) | 2 | PositionEvalPurpose::General. 바꾸면 position_score_at_position 의 평가 문맥이 바뀌어 on_trajectory 판정(도주 게이트 L277)이 달라진다 | 4 | 기존 |

<details><summary>`callees` 피호출자 49건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | action_candidates | game_ai::plan_legacy::sub_plan::SubPlan::action_candidates | pub | fn(&mut game_ai::plan_legacy::sub_plan::SubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\mod.rs:118 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 18개 중 상위 3개 |
| 1 | action_candidates | game_ai::plan_legacy::sub_plan::HideSubPlan::action_candidates | pub | fn(&mut game_ai::plan_legacy::sub_plan::HideSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\hide.rs:19 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 18개 중 상위 3개 |
| 2 | action_candidates | game_ai::plan_legacy::sub_plan::StealSubPlan::action_candidates | pub | fn(&mut game_ai::plan_legacy::sub_plan::StealSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\steal.rs:33 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 18개 중 상위 3개 |
| 3 | attack_structure_skill_action | game_ai::attack_structure_skill_action | pub | fn(&game_core::PlayerState, &game_core::OperationData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\fight_check.rs:794 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | attack_summon_action | game_ai::attack_summon_action | pub | fn(&game_core::PlayerState, &game_core::OperationData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\fight_check.rs:836 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 5 | attack_tower_action | game_ai::plan_legacy::sub_plan::LineSafeSubPlan::attack_tower_action | in:game_ai::plan_legacy::sub_plan::line_safe | fn(&mut game_ai::plan_legacy::sub_plan::LineSafeSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> std::option::Option<game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\line_safe.rs:48 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 6 | attack_tower_action | game_ai::plan_legacy::sub_plan::LineWaitSubPlan::attack_tower_action | in:game_ai::plan_legacy::sub_plan::line_wait | fn(&mut game_ai::plan_legacy::sub_plan::LineWaitSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> std::option::Option<game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\line_wait.rs:103 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 7 | attack_tower_action | game_ai::plan_legacy::sub_plan::LineDefenseSubPlan::attack_tower_action | in:game_ai::plan_legacy::sub_plan::line_defense | fn(&mut game_ai::plan_legacy::sub_plan::LineDefenseSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> std::option::Option<game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\line_defense.rs:372 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 8 | base_attacking_minion | game_ai::plan_legacy::old::base_attacking_minion | pub | fn(&game_core::PlayerState, &game_core::OperationData) -> std::option::Option<usize> | game-ai\src\plan_legacy\old\defense_nexus.rs:271 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 9 | base_defense_focus | game_ai::plan_legacy::old::base_defense_focus | pub | fn(&game_core::PlayerState, &game_core::OperationData) -> bool | game-ai\src\plan_legacy\old\defense_nexus.rs:200 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 10 | base_positioning | game_ai::plan_legacy::sub_plan::DefenseNexusSubPlan::base_positioning | in:game_ai::plan_legacy::sub_plan::defense_nexus | fn(&mut game_ai::plan_legacy::sub_plan::DefenseNexusSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\defense_nexus.rs:97 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 11 | battle_action | game_ai::battle_action | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, usize) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\fight_check.rs:620 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 12 | can_attack | game_core::Entity::can_attack | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1527 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 13 | can_skill | game_core::Entity::can_skill | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1708 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 14 | can_skill2 | game_core::Entity::can_skill2 | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1721 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 15 | can_tower_focused_when_attack | game_ai::can_tower_focused_when_attack | pub | fn(&game_core::GameContext, &game_core::AbstractGameWithCache, &game_core::PlayerState, &game_core::Entity) -> bool | game-ai\src\tower_discipline.rs:83 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 16 | check | game_core::CastingTarget::check | pub | fn(&game_core::CastingTarget, &game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\effect\type.rs:227 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 17 | check_kill_die_tick | game_ai::check_kill_die_tick | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::OperationData, &game_core::PlayerState, &game_core::Entity, bumpalo::collections::vec::Vec< &game_core::Entity>, bumpalo::collections::vec::Vec< &game_core::Entity>, &mut game_core::DebugFrameData) -> usize | game-ai\src\fight_check.rs:917 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 18 | commit_chase | game_ai::plan_legacy::sub_plan::DefenseNexusSubPlan::commit_chase | in:game_ai::plan_legacy::sub_plan::defense_nexus | fn(&mut game_ai::plan_legacy::sub_plan::DefenseNexusSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\defense_nexus.rs:33 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 19 | enemy_minion_wave_danger_damage_at | game_ai::enemy_minion_wave_danger_damage_at | pub | fn(usize, &game_core::OperationData, &game_core::Entity, u64, u64, usize) -> usize | game-ai\src\minion_wave_risk.rs:99 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 20 | expected_damage_target | game_core::Effect::expected_damage_target | pub | fn(&game_core::Effect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + , &game_core::Entity) -> usize | game-core\src\simulation\effect.rs:91 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 21 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 22 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 23 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 24 | is_base_attacking_minion | game_ai::plan_legacy::old::is_base_attacking_minion | pub | fn(&game_core::PlayerState, &game_core::OperationData, &game_core::Entity) -> bool | game-ai\src\plan_legacy\old\defense_nexus.rs:259 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 25 | is_in_range | game_core::Effect::is_in_range | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\effect.rs:63 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 26 | is_visible_from | game_core::Entity::is_visible_from | pub | fn(&game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1481 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 27 | iter_champions | game_core::AbstractGameWithCache::<'a, 'b>::iter_champions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5759 ~ game_core[6a30]::simulation::{impl#5}::iter_champions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1904 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 28 | iter_minions | game_core::AbstractGameWithCache::<'a, 'b>::iter_minions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5753 ~ game_core[6a30]::simulation::{impl#5}::iter_minions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1847 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 29 | iter_towers | game_core::AbstractGameWithCache::<'a, 'b>::iter_towers | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5762 ~ game_core[6a30]::simulation::{impl#5}::iter_towers::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1908 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 30 | minions | game_core::AbstractGameWithCache::<'a, 'b>::minions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize, &'b/#1 bumpalo::Bump) -> bumpalo::collections::vec::Vec< &game_core::Entity> | game-core\src\simulation.rs:1853 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 31 | new | game_ai::SmallActionTrace::new | pub | fn(&game_core::OperationData, usize, usize) -> game_ai::SmallActionTrace | game-ai\src\small_action\trace.rs:42 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 32 | new | game_ai::SmallActionSkill::new | pub | fn(&game_core::OperationData, usize) -> game_ai::SmallActionSkill | game-ai\src\small_action\cast.rs:119 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 33 | new | game_ai::SmallActionAround::new | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::OperationData, &game_core::PlayerState, usize, usize) -> game_ai::SmallActionAround | game-ai\src\small_action\around.rs:23 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 34 | new | game_ai::SmallActionAttack::new | pub | fn(&game_core::OperationData, usize) -> game_ai::SmallActionAttack | game-ai\src\small_action\cast.rs:23 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 35 | new | game_ai::SmallActionRunAway::new | pub | fn(&game_core::OperationData, &game_core::PlayerState, usize) -> game_ai::SmallActionRunAway | game-ai\src\small_action\move_actions.rs:25 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 36 | new_with_skill | game_ai::SmallActionRunAway::new_with_skill | pub | fn(&game_core::OperationData, &game_core::PlayerState, usize, bool) -> game_ai::SmallActionRunAway | game-ai\src\small_action\move_actions.rs:29 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 37 | nexus_is_critical | game_ai::plan_legacy::old::nexus_is_critical | pub | fn(&game_core::PlayerState, &game_core::OperationData) -> bool | game-ai\src\plan_legacy\old\defense_nexus.rs:139 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 38 | nontarget_windup_perceived | game_ai::nontarget_windup_perceived | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity) -> bool | game-ai\src\utils.rs:553 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 39 | position_score_at_position | game_ai::position_score_at_position | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, u64, u64, game_ai::PositionEvalPurpose) -> game_core::PositioningScore | game-ai\src\position_eval.rs:1192 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 40 | push | <game_core::Staff as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::staff | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\staff.rs:38 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 41 | push | <game_core::Athlete as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:434 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 42 | push | <game_core::Contract as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:2498 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 43 | radius | game_core::Entity::radius | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1509 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 44 | range | game_core::Effect::range | pub | fn(&game_core::Effect, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:25 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 45 | range_adjust | game_core::Effect::range_adjust | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:29 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 46 | skill2_effect | game_core::Entity::skill2_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1692 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 47 | skill_effect | game_core::Entity::skill_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1688 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 48 | ult_effect | game_core::Entity::ult_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1700 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
</details>

⚠**미매칭 12개**: `__rust_alloc`, `attack_minion_actions`, `dist_sq`, `extend`, `handle_alloc_error`, `llvm.memcpy.p0.p0.i64`, `llvm.memset.p0.i64`, `llvm.umax.i64`, `map_or`, `on_periodic_trajectory`, `on_trajectory`, `reserve_internal_or_panic`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m12.ll:35270) · **형제 11개** (DefenseNexusSubPlan)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::plan_legacy::sub_plan::DefenseNexusSubPlan as std::clone::Clone>::clone | pub | game-ai\src\plan_legacy\sub_plan\defense_nexus.rs:6 | True | fn(&game_ai::plan_legacy::sub_plan::DefenseNexusSubPlan) -> game_ai::plan_legacy::sub_plan::DefenseNexusSubPlan |
| 1 | <game_ai::plan_legacy::sub_plan::DefenseNexusSubPlan as std::default::Default>::default | pub | game-ai\src\plan_legacy\sub_plan\defense_nexus.rs:6 | True | fn() -> game_ai::plan_legacy::sub_plan::DefenseNexusSubPlan |
| 2 | <game_ai::plan_legacy::sub_plan::DefenseNexusSubPlan as std::fmt::Debug>::fmt | pub | game-ai\src\plan_legacy\sub_plan\defense_nexus.rs:6 | True | fn(&game_ai::plan_legacy::sub_plan::DefenseNexusSubPlan, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 3 | game_ai::plan_legacy::sub_plan::DefenseNexusSubPlan::merge | pub | game-ai\src\plan_legacy\sub_plan\defense_nexus.rs:20 | True | fn(&mut game_ai::plan_legacy::sub_plan::DefenseNexusSubPlan, game_ai::plan_legacy::sub_plan::DefenseNexusSubPlan) |
| 4 | game_ai::plan_legacy::sub_plan::DefenseNexusSubPlan::commit_chase | in:game_ai::plan_legacy::sub_plan::defense_nexus | game-ai\src\plan_legacy\sub_plan\defense_nexus.rs:33 | False | fn(&mut game_ai::plan_legacy::sub_plan::DefenseNexusSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 5 | game_ai::plan_legacy::sub_plan::DefenseNexusSubPlan::base_positioning | in:game_ai::plan_legacy::sub_plan::defense_nexus | game-ai\src\plan_legacy\sub_plan\defense_nexus.rs:97 | False | fn(&mut game_ai::plan_legacy::sub_plan::DefenseNexusSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 6 | game_ai::plan_legacy::sub_plan::DefenseNexusSubPlan::attack_minion_action | in:game_ai::plan_legacy::sub_plan::defense_nexus | game-ai\src\plan_legacy\sub_plan\defense_nexus.rs:136 | False | fn(&mut game_ai::plan_legacy::sub_plan::DefenseNexusSubPlan, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 7 | game_ai::plan_legacy::sub_plan::DefenseNexusSubPlan::attack_tower_action | in:game_ai::plan_legacy::sub_plan::defense_nexus | game-ai\src\plan_legacy\sub_plan\defense_nexus.rs:198 | False | fn(&mut game_ai::plan_legacy::sub_plan::DefenseNexusSubPlan, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> std::option::Option<game_ai::SmallActionPlay> |
| 8 | game_ai::plan_legacy::sub_plan::DefenseNexusSubPlan::calculate_score_parameter_value | pub | game-ai\src\plan_legacy\sub_plan\defense_nexus.rs:229 | True | fn(&game_ai::plan_legacy::sub_plan::DefenseNexusSubPlan, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::ScoreParameter) |
| 9 | game_ai::plan_legacy::sub_plan::DefenseNexusSubPlan::action_candidates | pub | game-ai\src\plan_legacy\sub_plan\defense_nexus.rs:245 | False | fn(&mut game_ai::plan_legacy::sub_plan::DefenseNexusSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 10 | game_ai::plan_legacy::sub_plan::DefenseNexusSubPlan::score | pub | game-ai\src\plan_legacy\sub_plan\defense_nexus.rs:311 | False | fn(&game_ai::plan_legacy::sub_plan::DefenseNexusSubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 |

**`open` 9건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | L274 `on_trajectory` 의 소스 표기(`a \|\| b` 순서·let 유무) — column 부재. IR 은 +0x30 이 참이면 +0x31 을 읽지 않고, DI 는 `on_trajectory = %371(+0x31 로드)` 를 단다. 동작(둘 중 하나면 참)은 확정, 표기만 불가(외연 동일) | 4 |  |
| 1 | 표기 불가 | L277 `!last_stand && (on_trajectory \|\| has_non_target_action_range)` 의 항 순서 — IR 은 +0x30 참일 때 last_stand 만 보고, 거짓일 때 (windup\|\|+0x31) 를 먼저 or 한다. 외연 동일 = 표기 불가 | 4 |  |
| 2 | 표기 불가 | L292 의 소스 표기 — IR 분기(47346~47354)는 critical ? (last_stand \|\| dmg<hp) : last_stand 로 통과. `!last_stand && !(critical && dmg < hp)` 와 외연 동일. 표기 불가 | 4 |  |
| 3 | 표기 불가 | L253 `version > 1` vs `version >= 2` — 외연 동일, 표기 불가. reach(version=2)에선 상수 참 | 4 |  |
| 4 | 미탐색 | L261~265 skill2_effect()/ult_effect() 가 None(level 미달)인 적 챔피언이 Skill2/Ult 상태일 때 unwrap 패닉(46243·46266) — 게임 규칙상 도달 불가로 보이나 이 함수 IR 만으로는 보증 못 함(action_state 가 레벨 해금 후에만 세팅되는지는 game_core 소관) | 4 |  |
| 5 | 미탐색 | self.last_gate(+0x10) 의 소비처 — 이 함수 안에는 읽기 0건. 다른 메서드(score/merge 등, 미열람) 소관 | 4 |  |
| 6 | 미탐색 | L216 expected_damage_target(effect, context, caster=tower(&dyn, vtable @anon.41), target=champ) 의 caster 트레이트 정체 — g06.ll:52355 시그니처만 확인(ptr nonnull, dereferenceable 없음 = 팻포인터 데이터 절반). 값 의미 = 타워가 champ 에게 줄 기대 피해 | 4 |  |
| 7 | 미탐색 | 콜리 내부(base_defense_focus·base_attacking_minion(_uncached)·is_base_attacking_minion·nexus_is_critical·check_kill_die_tick·battle_action·attack_summon_action·attack_structure_skill_action·can_tower_focused_when_attack·enemy_minion_wave_danger_damage_at·nontarget_windup_perceived·position_score_at_position·SmallAction*::new·AbstractGameWithCache::minions/iter_minions/iter_towers) 는 계약(인자·반환)만 — 본문 미열람(r13/r14/r15 명세 또는 별도 소관) | 4 |  |
| 8 | 미탐색 | 2단 이하 콜리의 TLS(v47_siege_stance·v48_cast_beams·champion_hp_value·interaction_score) 접점 여부 — battle_action(m15.ll:23700~25612) 등의 내부는 grep 하지 않음(1단 콜리 본문에는 심볼 0건) | 4 |  |

<details><summary>`closed` 1건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

| # | 물음 | 닫은 근거 |
|---|---|---|
| 0 | LAST_STAND_MEMO 의 정확한 키 튜플·3개 bool 의 의미(last_stand/final_stand/?)는 old::defense_nexus 소관 — 여기선 base_defense_focus·base_attacking_minion 이 같은 메모를 순서대로 읽는다는 사실만 확정(m04.ll:57166·59498) | 본문에 해소 표기가 있다 |
</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

