# eaeda0→f6bda0 EpicHuntSubPlan::action_candidates

## logic_060
```
// epic_hunt.rs:0~429 (배치 J) · 0.6.0 f6bda0
// 인자: self=&mut EpicHuntSubPlan{need_recall} · version · rnd · player · data{cache,context} · parameter · team_plan · debug. 반환 sret = bumpalo Vec<SmallActionPlay>(원소 0xb8 · 태그 @+0xb1 = 3+idx · AroundPosition 은 +0xb1 ∈ {0,1,2} 니치 ★0.6.0 AroundHide 제거 재번호).
// 자식 명세 있는 콜리는 계약만 적는다. 이하 `game` = data.cache.game(&dyn AbstractGame), `ctx` = data.context, `bump` = ctx.pool.
// ★v2 사장: 이 함수엔 version 분기 없음(version 은 콜리로 전달만).

// ── L202 ──
let team = player.info.team(@0xa00 ★0.6.0);  assert team < 2;
let champ: &Entity = cache.player_champion[team][player.info.position(@0xa90 ★0.6.0)].unwrap();   // cache+0x1e0+team*0x28+pos*8 · None 이면 unwrap 패닉

// ── L203~221 need_recall 갱신 (&mut self 유일한 쓰기) ──
if !self.need_recall {                                                                   // L203
    // L204: 에픽 엔티티 = 모바 모드의 jungle_runner.epic.live_list 첫 id → get_entity_by_id
    let mode = game.get_game_mode();  Moba 아니면 unwrap 패닉                              // vt+0x40 · (tag,&moba)
    if mode.live_list.len(@0x1a8) != 0 {
        if let Some(epic) = game.get_entity_by_id(*live_list.ptr(@0x1a0)[0]) {             // vt+0x1f0
            // L205: dmg = epic.attack_effect.unwrap()(@0x490, tag@0x4c0==-1 → 패닉).expected_damage_target(ctx, caster=epic as &dyn AbstractEntity, target=champ)   // Effect vt+0x28 expected_damage(불변)
            let dmg = expected_damage_target(...);
            if epic.ty@tag(@0x68) == 5 /*Epic*/ {                                           // L206
                let info = &epic.ty.Epic.info;
                // L207: 극성 = 분기방향. (A && B) || C 순서로 평가된다
                if (info.focused(@0x88/0x90) == Some(champ.id(@0x5c0)) && dmg*3 >= champ.hp(@0x670)) || champ.hp <= dmg {
                    self.need_recall = true;                                                // store i8 1 @self+0
                }
            }
        }
    }
} else {                                                                                  // L213
    let hp_ratio = champ.hp*100 / champ.stat_cached.hp(@0x628);   // 0 이면 div_by_zero 패닉
    if hp_ratio > 29 { self.need_recall = false; }                                          // L214
}

// ── L221~225 귀환 단일 후보 ──
if self.need_recall {                                                                     // L221 (재읽기)
    let mut res = Vec::new_in(bump);                                                       // L222
    res.push(SmallActionPlay::Recall(SmallActionRecall::new(data, player, end_delay=5)));  // L223 (tag 4 @0xb1 · idx 1 — 불변)
    return res;                                                                            // L224 → sret
}

// ── L227~228 목표 규율 액션 ──
if let Some(action) = team_plan.v27_objective_discipline_action(version, rnd, player, data, target=JungleType::Morgard(4)) {   // 184B(0xb8), tag@0xb1 == -1 → None · TeamPlan objective_discipline 블록 +0x278 ★0.6.0(구 +0x130)
    return Vec::from_iter_in([action], bump);                                              // L228 → sret (1개)
}

// ── L231~234 최근접 적 타워 ──
let champ = champ;  // L231 재바인딩
let nearest_enemy_tower: Option<&Entity> = cache.iter_towers_without_nexus(team=1-team)   // L232 (6 고정 타워 cache+0x180+enemy*8+k*0x10 + twin_towers 슬라이스 +0x130/+0x148 Chain)
    .filter(|x| x.can_target())      // L233 closure#2 = x.can_target(@0x6b9) && x.block_target_tick(@0x6a0)==0  (entity.rs:1478)
    .min_by_key(|x| x.distance_sq(champ));   // L234 closure#3 = |x.x-c.x|²+|x.y-c.y|² (@0x660/0x668) · 동률이면 앞 원소

// ── L237 공격 후보 원천 = get_act_action(version, rnd, player, data) 인라인(epic_hunt.rs:670~676) ──
let mut act_actions = Vec::new_in(bump);                                                   // 671
act_actions.extend(fight_check::battle_action(_version, _rnd, player, data, _end_delay));  // 672
act_actions.extend(fight_check::attack_summon_action(player, data));                       // 673
act_actions.extend(self.attack_jungle_action(_rnd, player, data));                          // 674 (fastcc: player→(team,position) 승격)

// ── L238~240 킬 우선 전략 시 에픽 참조 ──
let object_finish_objective: Option<&Entity> =
    if player.strategy(rnd, game).object_finish(@0xf) == KillPriority(0) {                  // L238 · 인라인: game.is_solorank(vt+0xe8) ? player.+0x568(24B ★0.6.0 · 구 +0x4f8) : game.strategy(team)(vt+0x108) · rnd 미소비
        game.get_game_mode() /*Moba 아니면 패닉*/ .live_list.first().and_then(|id| game.get_entity_by_id(*id))   // L239~240
    } else { None };

// ── L244~313 act_actions 필터 (closure#5, 캡처: game·player·&object_finish_objective·champ·&nearest_enemy_tower) → clone 수집 ──
let act_actions: Vec<_> = act_actions.iter().filter(|action| {
    // L246~249: 킬우선 무시 대상이면 제거
    if let Some(id) = action.target_id() /*tag 14~17 ★0.6.0(구 15~18) 의 +8*/ {
        if game.get_entity_by_id(id).is_some_and(|t| fight_model::should_ignore_object_finish_kill_priority_target(player, t, object_finish_objective)) { return false; }
    }
    // L253: 대상이 챔피언이 아니면 무조건 유지
    if let Some(id) = action.target_id() { if let Some(t) = game.get_entity_by_id(id) { if !t.is_champion() /*ty@0x68 != 13*/ { return true; } } }
    // L254: 챔피언 대상 공격류만 세부 판정. 그 외 variant(RunAway~Trace·AroundPosition·Stop) → false
    match action {
      Attack(a) => {                                                                     // L257~261 · tag 14 ★0.6.0
        let Some(target) = game.get_entity_by_id(a.target_id) else { return false };
        let eff = champ.attack_effect.as_ref().unwrap();                                    // L258
        eff.is_in_range(caster=champ, target)                                              // L259 · 165e7a0 ★0.6.0 RVA(구 12a0180)
          && nearest_enemy_tower.is_none_or(|t| {                                          // L259~260 (closure#4)
               let tower_hits_me = t.attack_effect.unwrap().is_in_range(t, champ);
               !(tower_hits_me && target.ty != Tower(2)) || target.team == champ.team })   // 팀 판별 동일: Player 면 인덱스까지 비교, Neutral 끼리면 true
      }
      Skill(a) => {                                                                      // L267~272 · tag 15 ★0.6.0
        let Some(target) = get_entity_by_id(a.target_id) else { return false };
        let eff = champ.skill_effect.as_ref().unwrap();                                     // L268 (@0x4c8, tag@0x4f8)
        eff.is_in_range(champ, target)                                                     // L269
          && nearest_enemy_tower.is_none_or(|t| !(t.atk.is_in_range(t,champ) && target.ty != Tower) || target.team == champ.team)   // L269~270
          && ( !(eff.ty.expected_move_on_hit() /*vt+0x88 ★0.6.0(구 +0x68)*/ || eff.ty.expected_rush_effect() /*vt+0x80 ★0.6.0(구 +0x60)*/)   // L270: 이동/돌진 스킬이 아니면 통과
               || nearest_enemy_tower.is_none_or(|t| !t.attack_effect.unwrap().is_in_range_ex(caster=t, target, cx=t.x, cy=t.y, tx=target.x, ty=target.y, offset=15000)) )   // L271~272 closure#6
      }
      Skill2(a) => { 동일 골격, eff = champ.skill2_effect() /*level(@0x5c8)>2 ? &@0x500 : None*/ .unwrap(); L282~287, closure#8 · tag 16 ★0.6.0 }
      Ult(a)    => { 동일 골격, eff = champ.ult_effect()    /*level>4 ? &@0x538 : None*/ .unwrap(); L297~302, closure#10 · tag 17 ★0.6.0 }
      _ => false
    }
}).map(|a| a.clone()).collect_in(bump);   // L313

// ── L315~423 retain #1 (closure#7, 캡처: data·champ·player·&version) — 아군 대상 힐/실드 정리 ──
act_actions.retain(|action| match action {
    Skill(a) | Skill2(a) | Ult(a) => {                                                    // L316 (Skill 318~336 / Skill2 352~370 / Ult 386~404 동형)
        let Some(target) = game.get_entity_by_id(a.target_id) else { return false /*제거*/ };   // L318/352/386
        if target.team != champ.team { return true /*적 대상은 유지*/ }                     // L319/353/387
        let Some(eff) = champ.skill_effect /*Skill2: skill2_effect() · Ult: ult_effect()*/ else { return false };   // L321/355/389
        let heal   = eff.ty.expected_heal(ctx, champ, @anon) != 0;                         // L322 vt+0x58 ★0.6.0(구 +0x40)
        let shield = eff.ty.expected_shield(ctx, champ, @anon) != 0;                       // L323 vt+0x60 ★0.6.0(구 +0x48)
        let buff   = eff.ty.expected_buff(ctx, champ, @anon).is_some();                   // L324 vt+0x68 ★0.6.0(구 +0x50) → Option<BuffState>(@0x48 != -1)
        let near = |e: &Entity| target.distance_sq(e) < 120000²+1 && (champ.team==Neutral || e.visible_state[champ.team]==Visible);   // visible_state Entity+0x38+team*0x18 tag 0
        let near_enemy = cache.player_champion[1-team].iter().flatten().any(near)          // L325
                      || cache.iter_towers(1-team).any(near)                               // L326
                      || cache.jungles.iter().any(near)                                    // L327
                      || cache.others[1-team].iter().any(|e| target.distance_sq(e) < 120000²+1);   // L328 (가시성 검사 없음)
        let hp_ratio = target.hp*100 / target.stat_cached.hp;                              // L329 (div0 패닉)
        if heal && !buff && hp_ratio > 79 && !buff_value::aoe_heal_covers_low_ally(version, &champ.skill_effect/*arm 별 효과 슬롯*/, data, player, anchor=target) { return false }   // L330~332
        !shield || buff || near_enemy                                                       // L336
    }
    _ => true
});

// ── L424~428 retain #2 (closure#8, 캡처: self·&version·parameter·rnd·player·data·debug) ──
act_actions.retain(|action| self.score(version, parameter, rnd, player, data, action, debug) >= -30);   // L425~426 (i64 signed) · 콜리 EpicHunt::score f860e0 ★0.6.0 RVA(구 ec7c90)

// ── L429 move_actions = get_move_action(version, rnd, player, data, &parameter.positioning_score, team_plan, debug) 인라인(epic_hunt.rs:517~668) ──
//    (★0.6.0 exe: position_score_at_position ff5ec0 ABI = (sret, version, player, data, x, y, purpose) — positioning_score 포인터 인자 없음(TLS 캐시 경유) · parameter.positioning_score 오프셋(구 +0x9f0)은 0.6.0 미확인 · cx/cy 는 parameter+0x14a8/+0x14b0)
let move_actions: Vec<SmallActionPlay> = {
    let mut res = Vec::new_in(bump);                                                       // 518
    let champ = cache.player_champion[team][pos].unwrap();                                 // 520
    // 523~530: 적 챔프가 비대상 스킬 예비동작으로 나를 겨눌 수 있는가
    let has_non_target_action_range = cache.player_champion[1-team].iter().flatten().any(|c|
        utils::nontarget_windup_perceived(version, player, data, caster=c) && c.is_champion()
        && match c.action_state@tag(@0x70) {
             4 /*Skill*/  => { let e = c.skill_effect.unwrap(); matches!(e.casting(@0x4f8 · Effect+0x30 캐스팅 태그), Position(1)|Direction(2)) && e.is_in_range(c, champ) }   // 525
             5 /*Skill2*/ => { let e = c.skill2_effect().unwrap(); 같은 조건 }              // 527
             6 /*Ult*/    => { let e = c.ult_effect().unwrap();    같은 조건 }              // 529
             _ => false });
    // 536~541
    let ps = position_eval::position_score_at_position(version, player, data, x=champ.x, y=champ.y, purpose=Objective(11));   // 56B PositioningScore · ff5ec0 ★0.6.0 RVA(구 d84db0) · 0.6.0 exe 는 호출 전 x,y 를 셀 중심(min(v/32000,29)*32000+16000)으로 스냅(콜리 내부 등가 추정)
    let on_trajectory = ps.on_periodic_trajectory(@0x31);
    if ps.on_trajectory(@0x30) || has_non_target_action_range || on_trajectory {          // 538
        res.push(RunAway(SmallActionRunAway::new_with_skill(data, player, 5, with_skill=true)));   // 540 (tag 3 · idx 0 — 불변)
        return res;                                                                        // 541
    }
    // 544~554 목표 태세
    let posture = team_plan.v25_objective_posture(version, player, data, target=Morgard(4));   // Option<ObjectivePosture>(88B, +0 i64==-1 → None)
    let mut objective_in_attack_range = false;
    if let Some(p) = &posture {                                                              // 545
        match p.kind(@0x50) {
          WaitGroup(3) | SoftDisengage(4) => {                                              // 546
              if p.kind == SoftDisengage && p.near_enemy_count(@0x38) != 0 { res.push(RunAway(new_with_skill(data, player, 5, false))); }   // 547~548
              res.push(AroundPosition(SmallActionAroundPosition::new(rnd, data, target_x=p.wait_pos.0(@0x20), target_y=p.wait_pos.1(@0x28), 5)));   // 550 (untagged variant · ecde50 ★0.6.0 RVA)
              if ctx.debug(@0x3b) { debug.infos(@0xa0).entry(champ.id).or_default().push(format!("v25 morgard hunt posture: {:?}", p.kind)); }   // 551~553
              return res;                                                                    // 554
          }
          Screen(2) if p.focus_enemy(@0x0).is_some() => { res.push(Trace(SmallActionTrace::new_attack_range(data, target=p.focus_enemy.unwrap()(@0x8), 5))); }   // 557~559 (tag 13 ★0.6.0(구 14)) — 계속 진행
          _ => {}
        }
    }
    // 565~574 에픽 사거리 판정
    let epic: Option<&Entity> = game.get_game_mode()/*Moba 아니면 패닉*/ .live_list.first().and_then(|id| game.get_entity_by_id(*id));   // 565
    if let Some(e) = epic {                                                                  // 566
        let mr = objective_attack_range(champ, e);                                           // 567 = epic_hunt.rs:16~17 인라인: champ.attack_effect.unwrap().range(champ, e) + champ.radius() + e.radius()
              // effect.rs:26 range() = range(@0x4a0) + growth_range(@0x4a8)*(level-1) + caster.stat_buff_cached.range(@0x438) + range_adjust(caster, target) /*vt+0x108 ★0.6.0(구 +0xe8)*/
              // entity.rs:1511 radius() = radius_mult(@0x470)==0 ? radius(@0x680) : radius*(100+mult)/100
        if champ.distance_sq(e) > mr*mr {                                                    // 569
            let camp = ctx.map.camp_pos(ty=Morgard(4), side = team!=0);                        // 570 · 157b720 ★0.6.0 RVA(구 131f300)
            if !objective_helpers::v23_should_break_objective_hunt_anchor(player, data, champ, objective=e, camp.0, camp.1)   // 570 · f89c30 ★0.6.0 RVA(구 ecb300)
               && !posture.map_or(false, |p| matches!(p.kind, WaitGroup|SoftDisengage)) {    // 571
                // ★0.6.0 신설(f6d9a4~f6da5d): v6 계약 집결점 우선
                if team_plan.v6_obj_armed /*+0xcc7 · 런타임 상시 1*/ != 0 {
                    let s: (u64,u64) = efb5b0(team_plan, slot=1 /*Epic*/, is_blue = team==0, data);   // 콜리 v3: efb5b0 = 두 캠프 중점 — slot 1: (camp_pos(map, JungleType(2), side=team!=0) + camp_pos(map, Morgard(4), side)) / 2 (x,y 각각 >>1) · slot 0: JungleType(0)·Serpen(5)
                    if champ.distance_sq(e) > champ.distance_sq(s) {                         // 집결점이 에픽보다 가까우면
                        res.push(AroundPosition(SmallActionAroundPosition::new(rnd, data, s.0, s.1, 5)));   // ecde50(rnd, data, sx, sy, 5) · 0xb8 memcpy → push(f5f020)
                        return res;                                                          // 에픽 접근(Trace) 후보 생략 · 573 과 같은 반환점(f6f2c9)
                    }
                }
                res.push(Trace(SmallActionTrace::new_attack_range(data, target=e.id, 5)));   // 572 · tag 13 ★0.6.0
                return res;                                                                  // 573~574
            }
            // (break 이거나 WaitGroup/SoftDisengage 면 epic 은 Some 으로 유지하고 계속)
        } else { objective_in_attack_range = true; }                                          // 569 else
    }
    // 579~633 근접 적 순회 (추격/도주 판정)
    let object_finish_battle = player.strategy(rnd, game).object_finish != KillPriority;     // 579 (BattlePriority)
    let mut runaway = false; let mut force_runaway = false;
    let pa = player.info.parameter(@0x180 · <0x448 구간 불변 추정).positioning_accuracy();    // 594
    let (min_v, max_v) = (pa, 2000 - pa);                                                    // 594~596
    let near_enemies: Vec<&Entity> = cache.iter_champions(1-team)                            // 598 closure#1: (champ.team Neutral || e.visible_state[champ.team]==Visible) && champ.distance_sq(e) < 160000²
        .filter(..).collect_in(bump);
    let me_die_tick = fight_check::check_kill_die_tick(version, data, judger=player, focus=champ, enemy=&near_enemies.clone(), towers=&Vec::new_in(bump), false, false, false, None);   // 599~600 · eda920 ★0.6.0 신 ABI(bool×3=false · &Option None)
    for enemy in near_enemies.iter() {                                                       // 602
        if let Some(e) = epic { if !object_finish_battle && !fight_model::can_enemy_hit_objective(enemy, objective=e, range_margin=25000) { continue; } }   // 603
        let jrng = utils::range_misjudge_rng(version, data, player, enemy_id=enemy.id);       // 607 ({i64,i64} 16B)
        let mr       = battle::max_range_can_use(champ, enemy)        * utils::range_misjudge_roll(rnd, &jrng, min_v, max_v) / 1000;   // 608 (롤1)
        let emr      = battle::max_range_can_use(enemy, champ)        * range_misjudge_roll(rnd, &jrng, min_v, max_v) / 1000;          // 609 (롤2)
        let emr_near = battle::max_range_nearly_can_use(enemy, champ, 40) * range_misjudge_roll(rnd, &jrng, min_v, max_v) / 1000;    // 610 (롤3)
        let dist = champ.distance_sq(enemy);                                                 // 611
        if me_die_tick < ctx.setting.tick_per_second(@0x12f8) {                              // 614 (1초 내 사망 예측)
            let emr = battle::max_range_nearly_can_use(enemy, champ, 60) * range_misjudge_roll(rnd, &jrng, min_v, max_v) / 1000;   // 627 (롤4)
            if dist > mr*mr && emr < mr { res.push(Trace(SmallActionTrace::new(data, target=enemy.id, 5))); }   // 629~631
            else if dist <= emr*emr { runaway = true; force_runaway = true; }                // 632~633
        } else if enemy.remain_action_time() > 10 && mr > 0 {                                // 615
            if dist > mr*mr { res.push(Trace(SmallActionTrace::new(data, enemy.id, 5))); }  // 616~617
        } else if emr < mr && dist > mr*mr {                                                 // 619
            res.push(Trace(SmallActionTrace::new(data, enemy.id, 5)));                       // 621
        } else if dist <= emr_near*emr_near { runaway = true; }                              // 622
    }
    // 640~643 적 others(소환수 등) 사거리
    for e in cache.others[1-team].iter() {                                                   // 640
        if let Some(atk) = &e.attack_effect {                                                // 641
            let range = atk.range(caster=e, target=champ) + e.radius() + champ.radius();      // 642
            if champ.distance_sq(e) <= range*range { runaway = true; force_runaway = true; break; }   // 643
        }
    }
    // 651~667 최종 합성
    let break_objective_anchor = epic.is_some_and(|e| v23_should_break_objective_hunt_anchor(player, data, champ, e, camp_pos(map, Morgard, side)));   // 651 (f89c30 2번째 호출)
    if runaway && (break_objective_anchor || !objective_in_attack_range || force_runaway) {   // 653
        res.push(RunAway(SmallActionRunAway::new_with_skill(data, player, 5, false)));       // 654
    } else if objective_in_attack_range {                                                    // 655
        res.truncate(0);                                                                     // 656
        if let Some(e) = epic {                                                              // 657
            let range = objective_attack_range(champ, e);                                    // 658
            let margin = objective_wiggle_margin(rnd, champ, range);   // epic_hunt.rs:20~30 인라인:
                // let mut inner = clamp(range.saturating_sub(10000), 12000, 45000);  let dist = champ.distance(e);
                // if dist + 15000 < range { inner = if inner + dist > range { rnd.gen_range(12000..=inner) } else { 12000 } }
                // margin = inner
            res.push(Trace(SmallActionTrace::new_attack_range_margin(data, target=e.id, 5, attack_range_margin=margin)));   // 659
        } else { res.push(Stop); }                                                           // 661 (tag 18 ★0.6.0(구 19))
    } else if res.is_empty() {                                                               // 663
        res.push(RunAway(SmallActionRunAway::new_with_skill(data, player, 5, false)));       // 664
    }
    res                                                                                      // 667
};
// → 배치 K(줄 432): nearest_tower / attacking_objective / best_move_action 선별 후 sret 기록.
// 예외 경로: L224·L228 은 J 가 sret 를 직접 채우고 ret. 언와인드 cleanup 은 지역 Vec drop 만.

// epic_hunt.rs:432~515 (배치 K) — 0.6.0 동치(태그·슬롯 재번호만)
// 진입 문맥(배치 J 소관): champ · team · nearest_enemy_tower · act_actions(L244) · act_actions0(L237) · move_actions(L429) · version · bump=context.pool.

// ---- L432~L437 타워 어그로 도주 ----
if let Some(nearest_tower) = nearest_enemy_tower {                      // L432
  if nearest_tower.ty@tag(+0x68) == 2 /*Tower*/ {                       // L433 → info = &Tower(+0x70)
    if info.nearest_enemy.is_some()(+0x88 tag)                          // L434
       && info.nearest_enemy.1(+0x98) == champ.id(+0x5c0) {             //      타워가 나를 조준중
      act_actions.truncate(0);                                          // L435
      move_actions.truncate(0);                                         // L436
      move_actions.push(SmallActionPlay::RunAway(                       // L437 태그 3 @+0xb1
        SmallActionRunAway::new_with_skill(data, player, end_delay=5, with_skill=true)));
    }
  }
}

// ---- L442~L451 에픽 공격 중이면 v27 안전 공격 위치 ----
if !act_actions.is_empty() {                                            // L442
  // L443: let GameMode::Moba(moba) = game.get_game_mode() else unwrap_failed; let Some(epic) = moba.live_list.first(+0x1a8/+0x1a0).and_then(get_entity_by_id) else → L456
  if let Some(epic) = ... {
    // L444: attacking_objective = act_actions.iter().any(|a| (a.tag(+0xb1)-14) <u 4 /*Attack|Skill|Skill2|Ult ★0.6.0(구 -15)*/ && a.target(+0x8) == epic.id)
    if attacking_objective {                                            // L445
      // L446: let action = self.v27_objective_safe_attack_position(version, rnd, player, data, &parameter.positioning_score, epic) — 완전 인라인, 아래 v27 절
      if let Some(action) = action {                                    // @+0xb1 != 0xFF
        if context.debug(+0x3b) {                                       // L447
          debug.infos(+0xa0).entry(champ.id).or_insert(Vec::new())      // L448
               .push("v27 morgard safe attack position".to_string());
        }
        return bumpalo::vec![in bump; action];                          // L450
      }
    }
  }
}

// ---- v27_objective_safe_attack_position (epic_hunt.rs:43~145, L446 인라인) → Option<SmallActionPlay> ----
//  L43  let Some(champ) = data.cache.player_champion[team][pos] else None
//  L44  objective_hp_ratio = epic.hp(+0x670)*100 / max(epic.stat_cached.hp(+0x628),1)
//  L45  if objective_hp_ratio < 36 → None
//  L49  attack_range = objective_attack_range(champ, epic)   // epic_hunt.rs:16~17 인라인(위와 동일 · range_adjust vt+0x108 ★0.6.0)
//  L52  if dist²(champ, epic) > attack_range² → None
//  L56~60 threats: Vec<&Entity> = cache.player_champion[1-team].iter_champions()
//         .filter(e ↦ blackboard[player.info.team /*stride 0x5c8 ★0.6.0*/].is_recent_visible(game, player, e) && (dist²(e,champ) ≤ 260000² || dist²(e,epic) ≤ 220000²))
//  L61  if threats.is_empty() → None
//  L65  hp_ratio = champ.hp*100 / max(champ.max_hp,1)
//  L67~68 current_nearest_enemy = threats.iter().map(|t| distance(champ,t)).min().unwrap()
//  L70~72 threat_contact = threats.iter().any(|t| dist²(champ,t) ≤ (max(max_range_nearly_can_use(t, champ, 50), 100000) + (hp_ratio<50 ? 70000 : 40000))²)
//  L74  if !threat_contact && !(hp_ratio < 45 && current_nearest_enemy < 220001) → None
//  L78  keep_attack_range = attack_range.saturating_sub(30000)
//  L83~86 allies: Vec<&Entity> = cache.player_champion[team].iter_champions().filter(a ↦ a.id != champ.id && dist²(a,epic) ≤ 220000²)
//  L87~91 ally_centroid = allies.is_empty() ? None : Some((Σa.x/n, Σa.y/n))
//  L95  cohesion(x,y) := ally_centroid.map_or(0, |(ax,ay)| (320000).saturating_sub(distance((x,y),(ax,ay))) / 10000)
//  L98~99 xi = parameter.positioning_score.cx(parameter+0x14a8) as i32, yi = .cy(+0x14b0) as i32
//  L100~101 cur_pv = v27_positioning_value(version, player, position_score_at_position(version, player, data, champ.x, champ.y, 11))
//       v27_positioning_value(L34~38): positioning = player.info.parameter.positioning_effective();
//         gain_weight = clamp(100 + (50-positioning)/10, 95, 105); risk_weight = clamp(100 + (positioning-50)/2, 75, 125);
//         trajectory_penalty = (score.on_trajectory(+0x30) || score.on_periodic_trajectory(+0x31)) ? max(positioning-50, 0) : 0;
//         value = score.gain(+0x10)*gain_weight/100 - score.risk(+0x0)*risk_weight/100 - trajectory_penalty
//  L102~103 current_score = cur_pv + min(current_nearest_enemy, 320000)/10000 + 8 + cohesion(champ.x, champ.y)
//  L105 best: Option<(x,y,score,nearest)> = None
//  L106~135 for dx in 0..7 { xb = xi-3+dx; if xb > 29 { continue }
//             for dy in 0..7 { yb = yi-3+dy;
//               L110 if xb<0 || yb<0 || yb>29 || map.walls[yb][xb](+0x78, [30][30]) != 0 → continue
//               L114~115 x = xb*32000+16000, y = yb*32000+16000
//               L116 if dist²((x,y), epic) > keep_attack_range² → continue
//               L120~123 nearest_enemy = threats.iter().map(|t| distance((x,y),t)).min().unwrap_or(u64::MAX)
//               L124~125 pv = v27_positioning_value(version, player, position_score_at_cell(version, player, data, xb, yb, 11))
//               L126 enemy_spacing = min(nearest_enemy, 320000)/10000
//               L127 range_slack = min(keep_attack_range.saturating_sub(distance((x,y), epic)), 80000)/10000
//               L128 score = pv + enemy_spacing + range_slack + cohesion(x,y)
//               L130~131 if best.map_or(true, |b| score > b.score) { best = Some((x,y,score,nearest_enemy)) }
//             } }
//  L136 let Some(best) = best else → None
//  L137 if best.score < current_score && best.nearest < current_nearest_enemy.saturating_add(30000) → None
//  L140 if dist²(champ, best) < 12000²+1 → None
//  L144 Some(SmallActionPlay::AroundPosition(SmallActionAroundPosition::new_with_radius(rnd, data, best.x, best.y, end_delay=3, around_radius=25000)))

// ---- L456~L512 공격 후보 없을 때의 이동 후보 확정 ----
if act_actions.is_empty() {                                             // L456
  positioning_accuracy = player.info.parameter.positioning_accuracy();  // L457
  min_v = positioning_accuracy; max_v = 2000 - positioning_accuracy;    // L458~459
  // L463~473: 아웃레인지 판정 — 적 챔프(cache.player_champion[1-team], None 건너뜀) 를 find:
  //   L464 jrng = range_misjudge_rng(version, data, player, c.id)
  //   L465 emr = max_range_can_use(c, champ) * range_misjudge_roll(rnd, &jrng, min_v, max_v) / 1000 + 10000
  //   L466 mr  = max_range_can_use(champ, c)
  //   L468 if !blackboard[1-team].is_recent_visible(game, player, c) → false
  //   L469 if dist²(c, champ) > emr² → false
  //   L470 if c.is_in_action() (ty==Champion(13) && action_state.tag(+0x70) >= 3) → false
  //   L472 if mr != 0 || c.block_input() → false
  //   → true
  if let Some(_) = ... {                                                 // L463 성립
    return bumpalo::vec![in bump; RunAway(SmallActionRunAway::new_with_skill(data, player, 5, false))];   // L473
  }
  // L476: best_move_action = move_actions.iter().max_by_key(|a| self.score(version, rnd, player, data, parameter, a, debug)).unwrap()   // f860e0 · 동점이면 뒤 원소 · move_actions 비면 unwrap panic
  // L480~482: trajectory_possible = game.iter_projectile()(vt+0x210).any(p ↦ p.team != Player(team) && !p.is_targeting() [move_type(+0x40) ∈ {Target(6), TargetSplash(7)…}] && dist²(champ, (p.x(+0x100), p.y(+0x108))) < 420000²)
  // L483~484: enemy_rushing = cache.player_champion[1-team].iter().flatten().any(c ↦ matches!(c.rush_state(+0x308), Rush | RushPenetrate))   // trajectory_possible 참이면 미평가
  if trajectory_possible || enemy_rushing {                              // L483
    move_action_input = best_move_action.clone().get_input(version, rnd, player, data, &parameter.positioning_score, debug);   // L486 (Option<Input> 32B)
    if let Some(Input::Move{x,y}) = move_action_input {                 // L490 tag 0 · -1=None → L507 · 그 외 → L504
      position_score = position_score_at_position(version, player, data, x, y, 11);   // L494
      on_trajectory = position_score.on_trajectory(+0x30) || position_score.on_periodic_trajectory(+0x31);   // L496
      if on_trajectory {                                                 // L498
        return bumpalo::vec![in bump; RunAway(SmallActionRunAway::new_with_skill(data, player, 5, false))];   // L499
      } else {
        return bumpalo::vec![in bump; best_move_action.clone()];        // L501
      }
    } else if move_action_input.is_some() {
      return bumpalo::vec![in bump; best_move_action.clone()];          // L504
    }
  }
  return bumpalo::vec![in bump; best_move_action.clone()];              // L507
} else {
  return act_actions;                                                    // L512 → sret 32B memcpy (move)
}
// L514~515: 함수 끝 drop — move_actions · act_actions(L512 제외) · act_actions0 순.
```

## changes
- ★신설(get_move_action L570~572 사이 · f6d9a4~f6da5d): `if tp.+0xcc7 != 0 { s = efb5b0(tp, 1, team==0, data); if dist²(champ,epic) > dist²(champ,s) { res.push(AroundPosition::new(rnd,data,s.0,s.1,5)); return res } }` — 에픽 접근 Trace 대신 계약 집결점 후보. 조건 위치 = `!v23_should_break_objective_hunt_anchor && posture ∉ {WaitGroup,SoftDisengage}` 통과 직후.
- 태그 재번호(SmallActionPlay 0xb1): Trace 14→13 · Attack/Skill/Skill2/Ult 15~18→14~17(L444 `tag-15`→`tag-14`) · Stop 19→18 · RunAway 3/Recall 4/AroundPosition(0/1/2) 불변.
- Effect vt: expected_heal/shield/buff 0x40/0x48/0x50→0x58/0x60/0x68 · expected_rush_effect 0x60→0x80 · expected_move_on_hit 0x68→0x88 · range_adjust 0xe8→0x108.
- player.team/pos +0x930/+0x9c0→+0xa00/+0xa90 · Strategy +0x4f8→+0x568 · TeamPlan objective_discipline +0x130→+0x278 · bb stride 0x5c8.
- check_kill_die_tick 신 ABI eda920(bool×3 · &None) · position_score_at_position ff5ec0 exe ABI 에 positioning_score 인자 없음(TLS) · 호출 전 셀 중심 스냅.
- 콜리: 128cc90→164b370 · c8d360→df9ba0 · c94a00→e01240 · ecb300→f89c30 · ca1240→e0d130 · d84db0→ff5ec0 · 131f300→157b720 · score ec7c90→f860e0 · 신규 efb5b0/ecde50/f5f020(push).
- v2 사장 게이트: 없음.

## verified
- capstone 0.6.0 f6d960~f6da80: f89c30 → posture(+0x5f0 != -1 && kind(+0x340-3 <2) → skip) → `[tp+0xcc7]==0 → f6efcd(Trace 경로)` / efb5b0(tp, 1, sete(team==0), data) → |Δx|²+|Δy|² vs r15(dist² champ↔epic) `jbe → f6efcd` / ecde50(rnd, data, sx, sy, 5) → memcpy 0xb8 → f5f020 push → jmp f6f2c9(res → move_actions 이동 · 573 의 반환점과 동일).
- efb5b0 디컴: camp_pos(map=ctx+0x20, ty=(slot!=0)*2, !is_blue) 와 camp_pos(map, ty=(slot==0)+4, !is_blue) 의 합 >>1.
- ff5ec0 프롤로그: 7인자(sret, version, player, data, [0x20] x, [0x28] y, [0x30] purpose) 확인 · 호출부 3곳 모두 `min(v/32000,29)*32000+16000` 스냅.
- 미확인: 삽입 블록 외 전 구간은 RE 「명령 다중집합 diff 잔여 = 태그 재번호·스텁 재배열」 판정 인용(본 세션 미대조) · parameter.positioning_score 의 0.6.0 오프셋 · ecde50 이 0.5.8 L550 생성자와 동일 RVA 짝인지(RE 는 필드 레이아웃만 기술) · 셀 스냅이 0.5.8 콜리 내부에 있었는지.

## confidence
A(신설 분기) / B(나머지 = RE 동치 판정 + §A 규칙 기계 치환 · 미확인 오프셋 2종).