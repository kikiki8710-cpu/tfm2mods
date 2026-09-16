# r21 티어1 심층 — 웨이브 10 (4) · 0.5.8 명세 logic 전문 + exe 힌트

요구: 0.5.8 명세(아래 logic) 를 기준선으로 0.6.0 디컴을 **줄 단위로 대조**해 「어느 줄이 어떻게 바뀌었나 / 새 분기·조건·상수·필드·콜리」 를 명세 형식(줄번호 참조 · 등가 Rust)으로 낸다. version≥3 분기는 「v3 경로」 로 별도 표기(값 미확정 · v2/v3 둘 다 적는다). 대형 함수는 먼저 콜리 지도·블록 지도(0.5.8 ↔ 0.6.0 매핑)를 만들고 신규 구역을 특정한 뒤 신규 구역만 정독한다.

## `d59940` → `f749a0` calculate_action_score  (8506→9588B · Δ+1082)
- 힌트: calculate_action_score(이관 · 배치 A #13: parameter 장부(+0x14d8/+0x14f0) 조기 반환 · 특성 · return 25 arm · 신규 콜리 ff1fc0/db0400/1526cb0/f714f0/de3df0/1514a50/165e4d0/1514140)
- exe 정렬: 명령 1844→2066 · 정렬 1614 · 잔여 구조 10 · 분기 10 · 콜리 주의: 12857f0→1643790 변경 ; 12857f0→ff1fc0 불일치(mig060 는 1643790) ; d31bb0→d72be0 미지
- 0.5.8 명세 #186 `action_score__calculate_action_score` src game-ai\src\action_score.rs:8 · one_line: 액션(평타/스킬)을 대상 t 에 쓸 때의 점수 — 미니언은 막타 타이밍·라인 스타일, 챔피언/타워/넥서스는 계수×기대피해/HP + 보너스
- 0.5.8 params: version:usize(분기 없음 · range_misjudge_rng 에 전달만) · rnd:&mut StdRng(320B)(gen_range 11지점 + range_misjudge_roll_i64 8지점(jrng None 이면 rn) · player:&PlayerState(2528B)(readonly · 0x930 team · 0x9c0 position · 0x180 parameter(las) · data:&OperationData(24B)(readonly · +0 cache · +8 context · +16 blackboard[2]) · parameter:&ScoreParameter(5384B)(readonly · +0 wave_snapshot Option 태그(1=Some) · +8 minions[1) · action:&Box<dyn Action>(16B)(readonly · 팻포인터 +0 data +8 vtable · vtable+0x90 cooltime(&se) · effect:&Effect(56B)(readonly · +0x20 start_timing · expected_damage_target(self)) · speed_mult:usize(0 이면 L22 udiv 패닉(m05.ll:37826)) · t:&Entity(1728B)(readonly · 평가 대상 엔티티) · ty:MinionActionType i8 range(0,(0 Pull / 1 Normal / 2 Push (tcxdict --enum, Direct))
- 0.5.8 logic 전문:
```
// L10  team=player.team(<2); champ = cache.player_champion[team][position].unwrap()
// L11  acc = parameter.last_hit_accuracy(); base = 1000-acc; min_v=acc; max_v=2000-acc
// L18  jrng = range_misjudge_rng(version, data, player, t.id)
// L19  value = effect.expected_damage_target(ctx, champ, t) * roll/1000        // roll = range_misjudge_roll_i64(rnd,&jrng,acc,2000-acc), 매번 새 롤
// L20  hp = t.hp*roll/1000;  L21 total = t.stat_cached.hp*roll/1000
// L22  start_timing = (effect.start_timing*100/speed_mult)*roll/1000   (speed_mult==0 → 패닉)
// L23  cooltime = (action.cooltime(champ)*100/speed_mult)*roll/1000    // %88 = roll*raw (÷1000 전) 이 뒤의 29999 비교에 쓰임
// L26  match t.ty {
//  Minion(1) =>
//   L29 if let Some(target)=t.Minion.nearest_enemy.and_then(get_entity_by_id) { match target.ty {
//     Tower(2) => { L32 if Moba && epic_minion_buff_time[1-team]!=0 → return 30;  L37 if target.Tower.ty ∈{TwinA,TwinB} { L38 if Moba && buff[1-team]!=0 → return 70 else → return 50 } }
//     Nexus(3) => { L46 if Moba && buff[1-team]!=0 → return 100 else → return 70 }
//     _ => {} } }
//   L55 if Moba && epic_minion_buff_time[1-team]!=0 → return 20        // 적 에픽버프 미니언
//   L62 if let Some(snap)=parameter.wave_snapshot { L63 if let Some(traj)=snap.minions[..count].find(id==t.id) {   // count>12 → 패닉
//     L68 error_prob = min(base*base/1000, 1000)
//     L72 if gen_range(0..=1000) < error_prob → return -9999               // 오판
//     L76 lane_phase = !(tutorial∈{None,MidBottom,Line,Total} && tick < first_spawn_tick - 30*tps ? false : true)  — 정확히: early=(tutorial∈집합 && tick<spawn-30tps); flag = !early && position!=Jungle   ⚠주의: tutorial∈집합 && tick≥경계 → flag=false 가 아니라 … IR: tutorial∈집합 이면 tick<경계 일 때만 position!=Jungle, 아니면 false; tutorial∉집합 이면 position!=Jungle
//     L89 predicted_hp = hp_at_tick(traj, start_timing)*roll/1000 ; L90 death_tick = traj.expected_death_tick
//     L93 if roll*hp_at_tick > 999 (생존) {
//        can_last_hit(DI명) = predicted_hp > value+5   // = 이번 타격으로 못 죽임
//        will_die_soon = death_tick ≤ cooltime+start_timing
//        L97 if !can_last_hit (지금 죽일 수 있음) {
//          L100 urgency = death_tick > start+5 ? (will_die_soon ? 25 : 15) : 30
//          L113 concurrent = #{other in snap: id≠t.id && 0 < hp_at_tick(other, cooltime+start) ≤ value+5}
//          L126 multi_bonus = concurrent>0 ? (L129 earlier=#{other: id≠t.id && other.death_tick < death_tick}; earlier==0 ? 5 : -5) : 0
//          L147 return flag ? urgency+multi_bonus : urgency+3+multi_bonus
//        } else { L152 if will_die_soon && predicted_hp ≤ 2*value → return ty==Pull ? -9999 : 5 }
//     }
//     L159 if predicted_hp > 3*value { L161 match ty { Pull→-9999, Push→10, Normal→ L168 if roll*cooltime_raw>29999 → (gen<err?10:-9999) else if flag → (gen<err?10:-9999) else → (gen<err?-9999:10) } }
//     else { L178 match ty { Pull→-9999, Push→10, Normal→ L184 gen<err ? 10 : -9999 } }
//   } }
//   // 스냅샷 없음/미등재 → 레거시(L195~)
//   L199 flag(%420) 위와 동일 계산
//   L211 for e in iter_entity(): match e.ty { Minion|Tower|Ghoul|SmallJiangshi|Bear|Eagle }: if e 가 t 를 조준(nearest_enemy/target_enemy == t.id) { dmg = e.attack_effect.expected_damage_target(ctx,e,t) (None→패닉); acd=attack_cooltime(e); ast = attack_effect.start_timing*100/attack_speed_mult(e); if e.state==Attack && !(Minion && is_range) && time<ast && ast(+15 if Tower)-time < start_timing && Attack.target_id==t.id → applyed += dmg; expected += max((cooltime+start_timing)/acd,1)*dmg (acd==0→패닉) }
//   L297 for p in iter_projectile(): if p.move_type==Target(6) { if target_id==t.id { caster=get_entity_by_id(p.caster_id)?; dist=distance(p,t); dmg=p.expected_damage_target(ctx,caster,t); if start_timing ≥ dist/speed+5 → applyed+=dmg; expected+=dmg } } else { caster?; if p.applyed_target.check_projectile(p,t) && p.is_in_orbit(t.x,t.y, radius*(100+radius_mult)/100) { dmg=…; applyed+=dmg; expected+=dmg } }
//   L319 applyed*=roll/1000; L320 expected*=roll/1000; L321 error_prob = 1000 - gen_range(acc..=1000)
//   L325 if value-5+applyed < hp (못 죽임) {
//      L365 if value + total*3/10 + expected < hp { L368 Pull→-9999 · Push→10 · Normal→ L378 cooltime>29999 ? (gen<err?10:-9999) : flag ? (gen<err?10:-9999) : (gen<err?-9999:10) }
//      else { L393 Pull→-9999 · Push→10 · Normal→ L402 gen<err?10:-9999 }
//   } else { L329 if gen_range(0..=1000) < error_prob → return -9999
//      L333 if expected + total*3/10 < hp { L335 Pull→-9999 · Push→10 · Normal→ L345 cooltime>29999 ? (gen<err?10:-9999) : flag ? (gen<err?10:-9999) : (gen<err?-9999:10) }
//      else { L359 gen; flag ? (gen<err ? -9999 : 15) : (gen<err ? -9999 : 20) } }
//  Champion(13) =>
//   L414 bonus=0; if t.team != champ.team { L416 if let Some(ep)=player_by_champion_id(t.id) { L417 a = blackboard[1-team].small_actions[ep.position]; if a.tag∈{Attack,Skill,Skill2} { L419 if let Some(tt)=get_entity_by_id(a.target_id) { L420 match tt.ty { Tower → bonus = twin ? 80 : 10 (L424); Nexus → bonus = 100 (L428) } } } } }
//   → L438 공통
//  _ => bonus = 0 → L438
// }
// L438 value = effect.expected_damage_target(ctx, champ, t) (롤 없음)
// L452 has_epic_buff(DI명) = !(Moba && ptr) || epic_minion_buff_time[team]==0     ⚠이름과 반대 극성처럼 보임 — IR 그대로: '아군 에픽버프 없음' 이 true
// L454 coef = match t.ty {
//   Nexus(3) → 200
//   Tower(2) → L457 in_range_minion = #{e: e.team==champ.team && e.ty==Minion && t.attack_effect.is_in_range(t, e)}   // ★t(타워) 자신의 attack_effect(사거리) — champ 것이 아니다. t.attack_effect None 이면 같은 팀 미니언을 만나는 순간 unwrap 패닉(L459)
//              L460 cond = match t.Tower.nearest_enemy { Some(id) if id!=champ.id → match get_entity_by_id(id) { Some(e) → e.hp ≥ t.attack_effect.expected_damage_target(ctx, t, e) || in_range_minion>1, None → in_range_minion>1 }, _ → false }   // ★타워가 조준 대상 e 에 주는 기대피해(t.attack_effect · None 이면 unwrap 패닉 L461) — 「타워 한 방에 안 죽는 대상을 조준 중이거나 사거리 안 아군 미니언 2+」
//              L462 if cond { ty==Push ? (has_epic_buff ? 160 : 240) : (has_epic_buff ? 80 : 160) } else { L476 t.hp > effect.expected_damage_target(ctx,champ,t) ? 0 : 30 }
//   _ → 0 }
// L516 return min(coef, coef*value / t.hp) + bonus     // t.hp==0 → 패닉
```

## `cb03b0` → `e96780` EpicCheckSubPlan::action_candidates  (5540→7098B · Δ+1558)
- 힌트: EpicCheckSubPlan::action_candidates(이관 · 배치 A #14: +0xcc0 게이트 · 캠프 주변 아군/적 세기 · 165e110 표 · db8970)
- exe 정렬: 명령 1071→1453 · 정렬 1014 · 잔여 구조 10 · 분기 10 · 콜리 주의: 31a01a3→381e0b3 미지 ; cada80→16a7af0 미지(+84B) ; cada80→e920f0 미지(+2B) ; ccc7a0→3821770 미지(-503B) ; cdac20→ebde60 미지(pdata 밖 thunk) ; dd26e0→f09860 변경 ; eb6100→16a7af0 불일치(mig060 는 ed6300) ; ffa3e0→ed6300 콜리 변경?(J0.00·+4567B)
- 0.5.8 명세 #191 `epic_check__EpicCheck__action_candidates` src game-ai\src\plan_legacy\sub_plan\epic_check.rs:14 · one_line: 에픽(모르가드) 확인 서브플랜 후보: 위험이면 도주 단독 / v25 오브젝트 자세(Screen·WaitGroup·SoftDisengage)가 있으면 그 자세 전용 후보 / 없으면 (자기 진영일 때 Stump 캠프 경유 확인 후) 모르가드 캠프 AroundPosition + 적 근접 시 도주 + 적에게 보이면 교전 + 소환수 공격
- 0.5.8 params: (sret):bumpalo::Vec<SmallActionPlay(IR %0 `sret([32 x i8]) writeonly`. ptr@0 · bump@+8 · cap@+0x) · self:&mut EpicCheckSubPlan (1B: m(IR %1 `captures(none) dereferenceable(1)` — readonly 없음 = 가변) · version:usize(IR %2. 본 함수 자체 분기 없음 — nontarget_windup_perceived·position_s) · rnd:&mut StdRng (320B)(IR %3. 본문 직접 사용 없음 — SmallActionAroundPosition::new(4곳)·batt) · player:&PlayerState (2528B)(IR %4 readonly. info.team(+0x930)·info.position(+0x9c0).) · data:&OperationData (24B)(IR %5 readonly. +0 cache · +8 context · +0x10 blackboard(&[B) · parameter:&ScoreParameter (5384B)(IR %6 readonly. +0x9f0 positioning_score 만 position_score_at) · team_plan:&TeamPlan(IR %7 `ptr noundef nonnull align 8`(dereferenceable·noalias·) · debug:&mut DebugFrameData (224B)(IR %8 `dereferenceable(224)` readonly 없음 = 가변. context.debug)
- 0.5.8 logic 전문:
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
      else if !is_enemy_side(data.context, team, champ.x, champ.y) {                         // rs:66 ★자기 진영이면 Stump 경유 검사. IR %290 = xor(team==0, (x−y+height) >u width)(L10349~10359) 참 → %298 Stump 검사 · 거짓 → %291 move_check=true. (x−y+height)>u width 는 !is_blue_side(레드 진영, 오라클 pub is_blue_side 대조) · is_enemy_side = is_blue_side != (team==0). 소스가 `if is_enemy_side {..true} else {Stump}` 인지 `if !is_enemy_side {Stump} else {..}` 인지는 표기 불가(분기 의미는 확정)
          let camp = data.context.map.camp_pos(JungleType::Stump /*2*/, team==0);            // rs:69
          if champ.distance_sq(camp) < 4900000001 /*70000²+1*/ { self.move_check = true; true }   // rs:71  (store L10363)
          else { false }
      } else { self.move_check = true; true };                                               // rs:66 else = 적 진영 (IR %291 store i8 1)
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

## `d5bbf0` → `f77820` calculate_interaction_action_score  (19586→20138B · Δ+552)
- 힌트: calculate_interaction_action_score(전담 · 배치 B §11: v3 분기 5지점 주소 확보)
- exe 정렬: 명령 3893→4054 · 정렬 3204 · 잔여 구조 10 · 분기 10 · 콜리 주의: 12857f0→1643790 변경 ; 128cc90→164b370 미지 ; 12a07d0→e80960 콜리 변경?(J0.00·+6444B) ; 16047b0→17d68b0 미지(+40B) ; 16fc160→de3d60 미지(-131B) ; 31a01a3→381e0b3 미지 ; 340e0→340e0 미지(pdata 밖 thunk) ; 6d4d0→6ed50 미지(pdata 밖 thunk)
- 0.5.8 명세 #181 `action_score__calculate_interaction_action_score` src game-ai\src\action_score.rs:980 · one_line: 액션 후보 1개(effect, 대상 t)의 상호작용 점수 i64 — 적/아군/자기 대상별 피해·힐·버프·CC 후속·타워·이동 항을 합산
- 0.5.8 params: version:usize (i64 %0)(AI 버전. 본문 분기 3곳: L986/L987 `version>1`(넥서스 최종방어·기지방어 판정 활성),) · _rnd:&mut StdRng (320B) %1(IR %1 = `ptr noalias noundef nonnull align 16 dereferenceabl) · player:&PlayerState (2528B) %2(info.team(+0x930)·info.position 태그(+0x9c0) 읽음. 콜리 다수에 전달.) · data:&OperationData (24B) %3(cache(+0x0 &AbstractGameWithCache 8840B) · context(+0x8 &Gam) · parameter:&ScoreParameter (5384B) %4 ((DI 이름 `parameter`. player(+0x918 ChampionScoreParameter)·pos) · action:&Box<dyn Action> (16B 팻포인터를 (DI 이름 `action`. vtable 슬롯: +0x78 action_name(sret String 24B) · effect:&Effect (56B) %6 (readonly)(DI 이름 `effect`. ty(+0x0 Arc<dyn EffectType> · ArcInner 데이터 =) · t:&Entity (1728B) %7 (readonly(DI 이름 `t` = 대상 엔티티. 적 챔피언/아군 챔피언/자기 자신/그 밖(타워·미니언 등) 으로 분기.) · _debug:&mut DebugFrameData (224B) %(IR %8 = `ptr noalias noundef nonnull align 8 dereferenceable)
- 0.5.8 logic 전문:
```
// 표기: L = action_score.rs 루트 줄(inlinedAt 루트). 오프셋은 reads 참조. 판정 순서 = IR 블록 순서.
// ---- §A 도입·v3 도주 차단 (L980~1034) ----
team = player.info.team; team>=2 → panic_bounds_check. my_position = player.info.position 태그
champ = data.cache.player_champion[team][my_position].expect(..)  (None → unwrap_failed, 메시지 anon.159 · L1013 인라인 위치)
L985 no_self_risk = if champ.stat_buff_cached.undying { true }
L986   else if version>1 && nexus_final_stand(player,data) { true }
L987   else if version>1 && champ.hp*100 > max(champ.stat_cached.hp,1)*35 { L988 base_defense_focus(player,data) }
       else { false }                                  // version<=1 이면 undying 아니면 false
L991 if let Some(v) = v57_summon_command_score(data,player,champ,action,t) { return v }   // Option<i64> {tag,v}
L1000 if version>1 && t.ty==Champion(13) && t.team != champ.team   // TeamType::eq 인라인(태그 → Player 페이로드)
L1001    && game.get_game_mode() 태그 != DeathMatch(2) {          // reach --gamemode 0: 항상 통과
L1013   if ctx.debug && parameter.v3_turnback_hold {
L1014     _debug.add_log(format!("TBHOLD-CAST T{team} {pos:?} tgt={t.id} d={champ.distance(t)} reach={line_effect_range_with_radii(effect,champ,t)} held={d>reach}")) }   // anon.152
L1019   if champ.distance(t) > line_effect_range_with_radii(effect,champ,t) {          // 사거리 밖
L1020     if parameter.v3_turnback_hold { return -9999999 }
L1023     if !no_self_risk {
L1024       if let Some((_,_,walk_tick)) = line_action_stance(data,champ,t,effect) {   // Option<(u64,u64,usize)>
L1025         if walk_tick != 0 {
L1026~1030      die_enemies: bump Vec<&Entity> = cache.player_champion[1-team].iter_champions()
                    .filter(|e| e.distance_sq(champ) < 22500000001 && data.blackboard[1-player.info.team].is_recent_visible(game,player,e)).collect_in(ctx.pool)   // closure$0 (aux)
L1031           if !die_enemies.is_empty() {
L1032             my_die = check_kill_die_tick(version,_rnd,data,player,champ,&die_enemies,&Vec::new_in(pool),_debug)
L1034             if my_die <= action.duration() + walk_tick { return -9999999 }   // IR: my_die > dur+walk 이면 계속
                } } } } } }
// ---- §B 기본 위험 · 적 타워 집중 (L1050~1082) ----
L1050 applyed_damage = parameter.player.applyed_damage ; L1051 base_damage = parameter.player.risk_damage
L1052 possible_damage = parameter.player.possible_risk(data, action.duration()+30)
L1053 is_visible = game.is_visible(1-team, champ.id)          // 적이 나를 보는가
L1054 if !is_visible { possible_damage /= 3 }   (sdiv)
L1060~1061 near_enemy_tower = cache.iter_towers_without_nexus(1-team).filter(|tw| tw.distance_sq(champ) < 22500000000).min_by_key(|tw| tw.distance_sq(champ))
L1064 can_focused_tower = false
      if let Some(tower) = near_enemy_tower {
L1065   if tower.ty == Tower(2) && L1066 tower.ty@Tower.info.nearest_enemy.is_none() {
L1067     range = effect.range + (champ.level-1)*effect.growth_range + champ.stat_buff_cached.range + effect.range_adjust(champ,tower) + champ.radius() + t.radius()
            // radius() 인라인: if stat_buff_cached.radius_mult==0 {radius} else {radius*(mult+100)/100 (udiv)}
L1068     d = t.distance(champ) ; L1069 d = d.saturating_sub(range)
L1071     tower_range = tower.attack_effect.unwrap().range + (tower.level-1)*growth_range + tower.stat_buff_cached.range   // None → unwrap_failed anon.160
L1072     dist = utils::distance(champ.x,champ.y,tower.x,tower.y)
L1074     can_focused_tower = dist <= d + 15000 + tower_range + tower.radius() + champ.radius()   (icmp ule)
        } }
L1081~1082 if can_focused_tower || t.is_champion() { possible_damage += parameter.player.risk_possible_tower }
// ---- §C 미니언 웨이브 투사 피해 (L1086~1094) ----
L1086 tick = game.tick(); spawn_epic = ctx.tutorial ∈ {None,MidBottom,Line,Total} (인라인 switch);
      line_phase_ok = !(tick < setting.epic_jungle.first_spawn_tick.saturating_sub(tps*30))   // 인라인 scope 이름 is_line_phase — IR 극성: tick >= first_spawn-30초 일 때 진행
L1087 projected_minion_damage = 0; if spawn_epic && line_phase_ok && t.team != champ.team && t.is_champion() {
L1088   if let Some((sx,sy,walk_tick)) = line_action_stance(data,champ,t,effect) {
L1089     a = action.duration() + walk_tick
L1090     window = max(tps/2, a) ; L1091 window = min(tps*2, window)      // lshr/shl 접힘
L1092     current_damage = enemy_minion_wave_risk_damage_at(version,data,champ,champ.x,champ.y,window)
L1093     stance_damage  = enemy_minion_wave_danger_damage_at(version,data,champ,sx,sy,window)
L1094     projected_minion_damage = stance_damage.saturating_sub(current_damage/2) } }
// ---- §D 기본 점수 (L1102~1108) ----
L1102 hp_value = champion_hp_value(data, parameter, &parameter.player)
L1105 base_score = if no_self_risk { -1 }
L1108              else { !( (possible_damage + base_damage + projected_minion_damage) * hp_value / champ.hp ) }   // xor -1 = 비트 NOT = -x-1 · sdiv · champ.hp==0 → div_by_zero panic · 표기(`!x` vs `-1-x`)는 외연 동일
// ---- §E 아군 타워 · 근접 적 · 이동 점수 (L1111~1149) ----
L1111~1112 near_ally_tower = cache.iter_towers_without_nexus(team).filter(|tw| tw.distance_sq(champ) < 22500000000).min_by_key(distance_sq)
L1115 has_near_enemy_champion = cache.player_champion[1-team].iter().flatten().any(|c| c.distance_sq(champ) < 15000000000 && data.blackboard[1-team].is_recent_visible(game,player,c))   // 5슬롯 언롤
L1119 tower_score = 0
      if let Some(tower) = near_ally_tower {
L1121   tower_atk = tower.attack_effect.as_ref().unwrap()   // None → unwrap_failed anon.162
        if has_near_enemy_champion && tower_atk.is_in_range(tower, t)
L1123      && tick < setting.tower_attack_disable_tick {
L1127     tower_score = min(tower_atk.expected_damage_target(ctx, tower as &dyn, t) * 100 / t.hp, 100)   (udiv · t.hp==0 → panic anon.163)
        } }
L1131 move_score = 0
      if effect.ty.expected_move_on_hit() || effect.ty.expected_rush_effect() {        // vtable +0x68 || +0x60 (이 순서)
L1134   if champ.distance_sq(t) < 1225000001 { move_score = 0 }   // ≤35000
L1137   else { ps = position_score_at_position(version, player, data, &parameter.positioning_score, t.x, t.y, PositionEvalPurpose::AttackStance)
L1140          move_score = -(hp_value * ps.risk) / 100 }   (sdiv)
      }
L1149 in_lapse = player_awareness_lapse(player, data)
      lapse_proximity_score = |e| (150000 - min(e.distance(champ),150000)) / 1500     // closure$6 (smin·sdiv)
// ---- §F 이동 중 사용 가능 · 사거리 내 (L1155~1172) ----
L1155 if action.can_use_with_move() && effect.is_in_range(champ, t) {                 // vtable +0xb8, 이 순서
L1156   if parameter.near_enemies.iter().any(|p| p.id == t.id) && in_lapse {
L1157     return 100 + lapse_proximity_score(t) }
L1159   if let Some(tp) = parameter.near_enemies.iter().find(|p| p.id == t.id) {
L1160     base_damage = tp.risk_damage
L1161     expected_damage = effect.expected_damage_target(ctx, champ as &dyn, t)
L1162     possible_damage = tp.risk_possible_tower + tp.possible_risk(data, action.duration()+30)
L1163     hp_value = champion_hp_value(data, parameter, tp)
L1166     total_damage = expected_damage + base_damage/2 + possible_damage/4   (sdiv)
L1167     if t.stat_buff_cached.undying { L1168 total_damage = min(max(t.hp-1,0), total_damage) }
L1172     return total_damage * hp_value / t.hp + 100   (sdiv · t.hp==0 → panic anon.164)
        } }   // near_enemies 에 없으면 아래로 낙하
// ---- §G 적 챔피언 대상 (L1177~1300) ----
sum3 = tower_score + base_score + move_score
L1177 if in_lapse && near_enemies.any(|p| p.id==t.id) { total = lapse_proximity_score(t) (L1178) ; goto RET }
L1179 if let Some(tp) = near_enemies.find(|p| p.id==t.id) {
L1180   base_damage = tp.risk_damage ; L1181 expected_damage = effect.expected_damage_target(ctx,champ,t)
L1182   possible_damage = tp.risk_possible_tower + tp.possible_risk(data, action.duration()+30)
L1183   hp_value = champion_hp_value(data,parameter,tp)
L1186   total_damage = expected_damage + base_damage/2 + possible_damage/4
L1187   if t.undying { L1188 total_damage = min(max(t.hp-1,0), total_damage) }
L1192   target_hp = t.hp ; score = total_damage * hp_value / target_hp   (sdiv · 0 → panic anon.165)
L1195   if t.is_champion() {
L1198     if expected_damage >= target_hp               { L1200 score += min(hp_value,100) }           // icmp slt 반전
L1201     else if expected_damage + base_damage/2 >= target_hp { L1203 score += min(hp_value,100)*3/4 }
L1206     else { auto_damage = champ.attack_effect.as_ref().map(|a| a.expected_damage_target(ctx,champ,t)).unwrap_or(0)   (L1206~1207)
L1210            if auto_damage + expected_damage >= target_hp { L1211 score += min(hp_value,100)/2 } } }
L1218   if let Some(cc_duration) = effective_action_cc_time(version, action, effect) {      // {tag,val}
L1219     if let Some(target_player) = cache.player_by_champion_id(t.id) {                   // L1220 null 검사
L1222       target_pos = target_player.info.position 태그 ; L1223 tps = max(setting.tick_per_second,1)
L1226       my_attack_dps = cache.player_champion_cache[team][my_position].attack_per_sec[target_pos]
L1231       ally_dps = 0; for ap in 0..5 { L1232 if ap == my_position {continue}
L1233         if let Some(ally) = cache.player_champion[team][ap] {
L1234           if ally.distance_sq(champ) < 22500000001 { c = cache.player_champion_cache[team][ap];
L1235~1241        ally_dps += c.attack_per_sec[target_pos] + c.skill_per_sec[target_pos] + c.skill2_per_sec[target_pos] + c.ult_per_sec[target_pos] } } }
L1248       follow_up_damage = (ally_dps + my_attack_dps) * cc_duration / tps   (udiv)
L1249       cc_score = follow_up_damage * hp_value / target_hp   (sdiv · overflow panic anon.166)
L1251       score += min(cc_score, 80)
L1258       nuke_during_cc = 0
            if champ.can_attack() || champ.attack_cooldown() <= cc_duration {      // attack_cooldown 인라인: ty 태그 switch → info.attack_cooldown
L1259~1260    if let Some(atk) = champ.attack_effect.as_ref() { nuke_during_cc = atk.expected_damage_target(ctx,champ,t) } }
L1264       if champ.can_skill() || champ.skill_cooldown() <= cc_duration {         // Champion 이 아니면 쿨다운 0 취급(IR: 태그≠13 → 통과)
L1265~1267    if let Some(s) = champ.skill_effect.as_ref() { if s.target.check(champ,t) { nuke_during_cc += s.expected_damage_target(ctx,champ,t) } } }
L1271       if champ.can_skill2() || champ.skill2_cooldown() <= cc_duration {
L1272~1274    s = if champ.level > 2 { champ.skill2_effect.as_ref() } else { None }; if let Some(s) { if s.target.check(champ,t) { nuke += s.expected_damage_target(..) } } }
L1278       if champ.can_ult() || champ.ult_cooldown() <= cc_duration {
L1279~1281    s = if champ.level > 4 { champ.ult_effect.as_ref() } else { None }; (동일) }
L1286       combined_with_cc_skill = nuke_during_cc + expected_damage
L1287       if combined >= target_hp { L1289 score += min(hp_value,80) }
L1290       else if target_hp > 0 && combined*100/target_hp > 59 { L1292 score += min(hp_value,80)/3 }
          } }
L1298   total = score + v55_mark_value(version,effect,data,player,parameter,champ,t,hp_value,_debug)
L1299         + v55_seal_value(version,effect,data,champ,t,hp_value)
L1300         - v55_banish_penalty(version,effect,data,player,champ,t,hp_value)
        goto RET }
// ---- §H 아군 챔피언 대상 (L1302~1423) ----
L1302 else if let Some(tp) = parameter.near_allies.iter().find(|p| p.id==t.id) {
L1303   expected_heal = effect.expected_heal_target(ctx, champ as &dyn, t as &dyn)
L1304   expected_shield = effect.expected_shield_target(ctx, champ, t)
L1306   v55_tbuff: Option<BuffState> = v55_target_dependent_buff(effect,data,t)
L1307   has_buff = v55_tbuff.is_some() || effect_buff_target(version,effect,ctx,champ,t).is_some()   // 단락: v55 Some 이면 두 번째 미호출
L1309   v55_on_attack = effect.ty.expected_on_attack_damage(ctx, champ)          // vtable +0xb0
L1310   etc_buff = if has_buff || v55_on_attack > 0 || !effect.ty.etc_buff() { 0 }    // IR 평가 순서: on_attack>0 · !etc_buff() · has_buff 를 or 로 합침(단락 없음)
L1312             else if effect.ty.expected_mark(ctx, champ as &dyn).is_none() { 5 } else { 0 }   // vtable +0xa8 sret 48B, 태그 0=None
L1320   hp_value = champion_hp_value(data,parameter,tp)
L1322   possible_damage = tp.risk_possible_tower + tp.possible_risk(data, action.duration()+30)
L1323   applyed_damage = tp.applyed_damage ; L1325 epic_tank = tp.risk_epic_damage
L1328   expected_shield = if possible_damage>0 || epic_tank!=0 || applyed_damage>0 { expected_shield } else { 0 }
L1338   incoming = applyed_damage + possible_damage + epic_tank ; epic_incoming = epic_tank
L1339   missing = max(t.stat_cached.hp - t.hp, 0)
L1340   expected_heal = min(missing + incoming*2, expected_heal) ; expected_shield = min(incoming*3, expected_shield)
L1347   if has_buff || v55_on_attack > 0 {
L1348     bs = v55_tbuff.clone().or_else(|| effect_buff_target(version,effect,ctx,champ,t))   // closure$12
          if let Some(bs) = bs {
L1350       crisis_needed = bs.cc_immune || bs.undying || bs.toughness == 0
L1351~1354  crisis: Option<(bool,bool)> = if crisis_needed { Some(defensive_crisis(version,_rnd,player,data,t,_debug)) } else { None }
L1360       v = buff_value_v54(&bs, t, data, player, parameter, crisis.as_ref(), incoming, epic_incoming, v55_on_attack, hp_value)
L1363       cast_range = effect.range + (champ.level-1)*growth + champ.stat_buff_cached.range + effect.range_adjust(champ,t) + champ.radius() + t.radius()
L1364       walk_ticks = champ.distance(t).saturating_sub(cast_range) / max(champ.stat_cached.move_speed,1)   (udiv)
L1365       delay_sec = walk_ticks / tps   (sdiv · tps==0 → panic anon.167)
L1366       v = clamp(6 - delay_sec, 0, 6) * v / 6
L1368       v_currency = area_buff_multi_scale(version,effect,data,player,t,v)
          } else if v55_on_attack > 0 {     // L1369
L1373       v = buff_value_v54(&BuffState::default(), t, data, player, parameter, None, incoming, epic_incoming, v55_on_attack, hp_value)
L1375       v_currency = area_buff_multi_scale(version,effect,data,player,t,v)
          } else { v_currency = 0 }
L1378     if has_buff && v_currency == 0 {
L1380       bs2 = v55_tbuff.clone().or_else(|| effect_buff_target(..)) ; w = noncombat_steroid_window(player,data,champ,t)   // Option<(bool,bool)>
            if bs2.is_none() || w.is_none() { L1392 expected_buff_score = 0 }
            else { L1383 v = noncombat_steroid_value(data, &bs2, t, &w) ; L1384 v = area_buff_multi_scale(version,effect,data,player,t,v)
L1386              if v > 0 && ctx.debug && tick % 6 == 0 { L1387 add_log("STEROID_WIN T{team} {pos:?} ally_target v={v} fights_back={w.fights_back}(=.0)") }   // anon.153 · w = NoncombatSteroidWindow{fights_back@0, minion_wave@1}(tcxdict 2B)
L1390              expected_buff_score = v }
          } else { L1395 expected_buff_score = v_currency }
        } else { L1398 expected_buff_score = 0 }
L1401   score = (expected_heal + expected_shield) * hp_value / t.hp   (sdiv · 0 → panic anon.168)
L1403   if expected_heal + expected_shield > 0 && score == 0 {
L1404     score = if applyed_damage > 0 || possible_damage > 0 || tp.risk_damage != 0 || t.hp < t.stat_cached.hp { 1 } else { 0 } }
L1411   aoe_ally_value = v54_aoe_ally_heal_value(version,effect,data,parameter,champ,t,t.id,action)
L1414   if has_buff && ctx.debug { L1415~1416 add_log("BUFFSCORE nm={action.action_name()} T{team} {pos:?} ally t={t.id} buff={expected_buff_score} hs={score} etc={etc_buff} aoe={aoe_ally_value}") }   // anon.154
L1419   total = expected_buff_score + score + etc_buff + aoe_ally_value
L1423   if total == 0 { total = if has_buff || expected_heal+expected_shield > 0 { -10 } else { 0 } }
        goto RET }
// ---- §I 자기 자신 대상 (L1429~1550) ----
L1429 else if t.id == champ.id {           // tp = parameter.player
L1430~1431 expected_heal/shield = effect.expected_heal_target/expected_shield_target(ctx,champ,t)
L1434~1435 v55_tbuff = v55_target_dependent_buff(effect,data,t) ; has_buff = v55_tbuff.is_some() || effect_buff_target(version,effect,ctx,champ,t).is_some()
L1437   v55_on_attack = effect.ty.expected_on_attack_damage(ctx,champ)
L1438~1440 etc_buff = (§H L1310~1312 와 동일)
L1450   epic_tank = parameter.player.risk_epic_damage
L1453   expected_shield = if possible_damage(§B 값) < 1 && (epic_tank | parameter.player.applyed_damage) == 0 { 0 } else { expected_shield }
L1462   incoming = possible_damage + applyed_damage + epic_tank ; epic_incoming = epic_tank
L1463   missing = max(champ.stat_cached.hp - champ.hp, 0)
L1464   expected_heal = min(missing + incoming*2, expected_heal) ; expected_shield = min(incoming*3, expected_shield)
L1469~1511 expected_buff_score = §H L1347~1395 와 같은 구조(대상 t=champ · hp_value=§D 의 hp_value) — 단 L1363~1366 의 사거리/도달 감쇠가 없다: v_currency = area_buff_multi_scale(version,effect,data,player,champ, buff_value_v54(&bs,champ,data,player,parameter,crisis.as_ref(),incoming,epic_incoming,v55_on_attack,hp_value)) (L1482~1484) ; bs None && on_attack>0 → default BuffState (L1489~1491) ; L1494 has_buff && v_currency==0 → L1496 bs2·w=noncombat_steroid_window(player,data,champ,champ) → L1499 noncombat_steroid_value → L1500 area_buff_multi_scale → L1502~1503 로그 "STEROID_WIN ... self v= fights_back="(anon.155) → L1506 expected_buff_score=v / L1508 0 ; L1511 else v_currency
L1514   (has_buff 아니고 on_attack<=0) expected_buff_score = 0
L1517   score = (expected_heal + expected_shield) * hp_value / champ.hp   (sdiv · 0 → panic anon.169)
L1523   ally_aura_protect = 0; for ally in cache.player_champion[team].iter().flatten() {      // 자기 자신 포함 5슬롯
L1524     if effect.ty.expected_ally_aura_heal_at(ctx, champ, ally) != 0 {                       // vtable +0x98
L1527       if let Some(atp) = near_allies.find(|p| p.id == ally.id) {
L1528         if defensive_crisis(version,_rnd,player,data,ally,_debug).0 {
L1529           ally_aura_protect += min(champion_hp_value(data,parameter,atp), 80) } } } }
L1537   v55_trigger = v55_spirit_trigger_value(effect,data,player,parameter,champ,hp_value)
L1540   aoe_ally_value = v54_aoe_ally_heal_value(version,effect,data,parameter,champ,champ,champ.id,action)
L1543   if has_buff && ctx.debug { L1544~1545 add_log("BUFFSCORE nm=.. T.. self buff= hs= etc= aura= trig= aoe=") }   // anon.156
L1548   total = score + expected_buff_score + etc_buff + ally_aura_protect + v55_trigger + aoe_ally_value
L1550   if total == 0 { total = if has_buff || expected_heal+expected_shield > 0 { -10 } else { 0 } }
        goto RET }
// ---- §J 그 밖(타워·미니언·근접목록 밖 챔피언) ----
L1558 else { if t.is_champion() && ctx.debug && t.team == champ.team { L1559 add_log("BUFFTGT_MISS T{team} {pos:?} t={t.id} near_allies") }   // anon.157
        total = 0 }
// ---- RET (L1564) ----
return sum3 + total          // sum3 = tower_score + base_score + move_score
// 반환 phi 전체: [v57 Some(v)] [§F 100+x] [sum3+total] [-9999999 L1020] [-9999999 L1034]
```

## `cce210` → `fd98c0` SmallActionTrace::get_input  (4742→5984B · Δ+1242)
- 힌트: SmallActionTrace::get_input(전담 · 배치 B §13: 우물 회피/lethal 정책)
- exe 정렬: 명령 961→1225 · 정렬 737 · 잔여 구조 10 · 분기 10 · 콜리 주의: 12857f0→1643790 변경 ; 31a01a3→381e0cb 미지 ; 31a01bb→13f0790 미지 ; 31a01bb→381e0cb 미지 ; cefd50→381ebcf 불일치(mig060 는 e29770) ; d11bc0→e48880 불일치(mig060 는 e1b5b0) ; d13b30→e4abd0 불일치(mig060 는 e23740) ; d1d9c0→1643790 불일치(mig060 는 e609e0)
- 0.5.8 명세 #176 `SmallActionTrace__get_input` src game-ai\src\small_action\trace.rs:165 · one_line: 추격(Trace) 소액션의 매 틱 입력 — 대상까지의 최소 사거리 표적점을 목표로 잡고, 타워/우물/적존 회피 정책의 PathFinder 를 (재)생성·갱신한 뒤 타워 escape 판정이면 RunAway 입력, 아니면 경로 입력을 낸다
- 0.5.8 params: (sret):Option<Input>(32B)(None 경로 = `store i64 -1` 만(+8..+32 미기록). Some 경로 = 콜리(RunAwa) · self:&mut SmallActionTrace(152B)(writes 전수 = writes 필드) · version:usize(version<2 게이트(63725: lethal_chase 계산 생략 · 64320/64349: escap) · rnd:&mut StdRng(320B)(본문 직접 write 없음 — new_target_with_policy/update_path/RunAway:) · player:&PlayerState(2528B)(info.team(+0x930)·info.position(+0x9c0)) · data:&OperationData(24B)(+0 cache(game 팻포인터·player_champion·iter_towers_without_nexus) · positioning_score:&PositioningScoreData(2760B)(본문에서 안 읽음 — 클로저 캡처(64067·64168) 및 RunAway::get_input 전달) · debug:&mut DebugFrameData(224B)(본문 직접 write 없음 — RunAway::get_input 에만 전달(64364). 그 콜리의 %7 도)
- 0.5.8 logic 전문:
```
fn get_input(&mut self, version, rnd, player, data, positioning_score, debug) -> Option<Input>
// L166
target = game.get_entity_by_id(self.target)?          // None → return None
// L167
champ = cache.player_champion[player.info.team][player.info.position].unwrap()
// L169~171
radius_sum = champ.radius() + target.radius()
atk = champ.attack_effect.unwrap()
min_range = atk.range(champ) /*range+growth*(level-1)+stat_buff.range*/ + atk.range_adjust(champ, target) + radius_sum
// L174~187
if !self.attack_range_only {
    if let Some(s) = champ.skill_effect  && s.target.check(champ, target) { min_range = min(min_range, s.range(champ)+s.range_adjust(champ,target)+radius_sum) }   // L175~178
    if let Some(s2) = champ.skill2_effect() /*level>2*/ && s2.target.check(champ, target) { min_range = min(min_range, s2.range(champ)+s2.range_adjust(champ,target)+radius_sum) }   // L184~187
}
// L194
if target.is_visible_from(champ) {
    self.goal_x = target.x; self.goal_y = target.y                       // L217~218
} else {
    dx = champ.x - target.x; dy = champ.y - target.y; sz = max(isqrt(dx²+dy²), 1)   // L195~197
    txi = min(target.x/32000, 29); tyi = min(target.y/32000, 29)          // L201~202
    if map.bushes[tyi][txi] == 0 {                                        // L204
        d = min_range.saturating_sub(self.attack_range_margin)            // L199
        (x,y) = Game::adjust_position(map, setting, target.x + d*dx/sz, target.y + d*dy/sz)   // L209~211 (sdiv)
        self.goal_x = x; self.goal_y = y                                  // L212
    } else { self.goal_x = target.x; self.goal_y = target.y }             // L205 (대상이 부시 안)
}
// L222
use_tower_avoid_path = self.avoid_unnecessary_tower || target.ty != Champion(13)
// L227~231
lethal_chase = version >= 2 && v3_lethal_tower_hp(data, player, champ)
chase_policy = if lethal_chase { SolvePolicy{deadly_cells: v3_deadly_edge_cells(version, player, data), direct_cross: false} } else { SolvePolicy{2, true} }
// L238
if use_tower_avoid_path {
    strict = self.avoid_unnecessary_tower                                                          // L239·241
    key = if lethal_chase {"trace_lethal_tower"} else if strict {"trace_avoid_tower"} else {"trace_objective_avoid_tower"}
    tower_dodge = if strict { TowerDodgeContext::new_unnecessary_tower_avoid(version, player, data, goal) } else { new_tower_avoid_v3(version, player, data, goal) }   // L243/245
    // L247~248
    if !(self.path_finder is Some && pf.key == key) {
        self.path_finder = Some(PathFinder::new_target_with_policy(rnd, ctx, version, key, chase_policy, champ.x, champ.y, goal_x, goal_y, closure(&lethal_chase, &tower_dodge, &version, &strict, player, data, positioning_score)))   // 구 Box 해제
    }
    self.path_finder.as_mut()?.update_path(rnd, ctx, champ.x, champ.y, goal_x, goal_y, closure(동일 캡처))   // L261
} else {
    avoid_zone = self.avoid_enemy_zone; enemy_team = 1-team; champ_radius = champ.radius()               // L275~277
    zone_ok = (&avoid_zone, data, &enemy_team, &champ_radius)                                            // L278 캡처
    lethal_dodge = if lethal_chase { Some(new_tower_avoid_v3(version, player, data, goal)) } else { None }   // L292~295
    well_key = if lethal_chase {"trace_avoid_well_lethal"} else {"trace_avoid_well"}                       // L297
    if !(self.path_finder is Some && pf.key == well_key) {                                                // L298
        self.path_finder = Some(PathFinder::new_target_with_policy(rnd, ctx, version, well_key, chase_policy, champ.x, champ.y, goal_x, goal_y, closure(&lethal_dodge, &version, player, &zone_ok)))   // L299
    }
    self.path_finder.as_mut()?.update_path(rnd, ctx, champ.x, champ.y, goal_x, goal_y, closure(동일 캡처))   // L315
}
// L334~342
raw_escape = if !self.dive_ignore_tower_escape || lethal_chase { path_needs_tower_escape(version, player, data, self.path_finder.as_ref()?) }
             else { self.escape_commit_until = 0; self.last_escape = None; false }   // L335~336
// L354~361
tower_shooting_me = cache.iter_towers_without_nexus(1-team).any(|t| t.ty==Tower && t.Tower.info.nearest_enemy.is_some_and(|(_,id)| id == champ.id))   // closure$5 [aux m06.ll:34359]
    || (champ.last_attacked_from.is_some_and(|id| game.get_entity_by_id(id).is_some_and(|e| e.team == Player(1-team) && e.ty == Tower && champ.hp*100/max(champ.stat_cached.hp,1) < 66)))   // L360~361 closure$9 (any 이 true 면 단락)
tick = game.tick()   // L363
// L364~379
if raw_escape {
    if version < 2 || tower_shooting_me { self.escape_commit_until = tick + tps*3/2; escape = true }   // L370
    else { self.avoid_unnecessary_tower = true; self.escape_commit_until = 0; escape = false }         // L366~367
} else { escape = tick < self.escape_commit_until }                                                   // L374
self.last_escape = Some(escape)                                                                       // L379
// L381~383
if escape { return SmallActionRunAway::new_with_skill(data, player, 5, true).get_input(version, rnd, player, data, positioning_score, debug) }
// L386~390
pf = self.path_finder.as_mut()?
safe = if champ.stat_cached.move_speed > target.stat_cached.move_speed || dist²(champ,target) < 14400000001 { SafeMoveWithSkill::Safe(1) } else { Must(2) }
return pf.get_input(player, data, safe, false)   // L389 (Input → Some 로 그대로 sret)
```
