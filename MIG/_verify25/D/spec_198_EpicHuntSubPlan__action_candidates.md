---

### `198` EpicHuntSubPlan::action_candidates — EpicHunt(모가드 사냥) 서브플랜의 후보 행동 생성: need_recall 갱신→귀환/규율 단일후보 조기반환→전투·소환수·정글 공격후보 필터/정리→get_move_action 이동후보 합성(J: 200~429) → 이후 타워·에픽 위치·최적 이동 선별(K: 432~515)

| 항목 | 값 |
|---|---|
| id | `epic_hunt__EpicHunt__action_candidates` |
| 심볼 | `_RNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy8sub_plan9epic_huntNtB2_15EpicHuntSubPlan17action_candidates` |
| 소스 | `game-ai\src\plan_legacy\sub_plan\epic_hunt.rs:200` |
| IR | `m15.ll` 12793~18168행 |
| 경로·가시성 | `game_ai::plan_legacy::sub_plan::EpicHuntSubPlan::action_candidates` · **pub** |
| 계층 | 기타 |
| exe | `eaeda0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r16` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&mut game_ai::plan_legacy::sub_plan::EpicHuntSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay>
```

**TLS 메모 절(`sig.tls` · 정정 경로 `/specs[198]/sig/tls/<키>`)**

- `name`: (직접 접점 없음)
- `role`: 간접 소비자만 — 이 함수 본체·J 범위 aux 어디에도 LocalKey::with / thread_local 참조가 없다(주석본 eaeda0.ll 전체 grep 0건).
- `key`: 해당 없음
- `layout`: 해당 없음
- `invalidation`: 해당 없음
- `call_conditions`: TLS 를 쓰는 콜리를 J 범위가 부르는 순서(미러 설계용): ①L205 Effect::expected_damage_target(need_recall==false 경로) ②L227 v27_objective_discipline_action(need_recall==false) ③L237 battle_action→attack_summon_action→attack_jungle_action ④L244 closure#5(should_ignore_object_finish_kill_priority_target·is_in_range) ⑤L315 retain#7(expected_heal/shield/buff vtable·aoe_heal_covers_low_ally) ⑥L424 retain#8→EpicHuntSubPlan::score(각 액션마다) ⑦L429 get_move_action: L523 nontarget_windup_perceived(적 챔프마다) → L536 position_score_at_position(purpose=Objective 11) → L544 v25_objective_posture → L570 MapDef::camp_pos(★game_core 내부 LocalKey<RefCell<(usize,[[Option<(u64,u64)>;2];8])>> 메모, g02.ll:6914 — 순수 좌표 캐시라 순서 무관)+v23_should_break_objective_hunt_anchor → L599 check_kill_die_tick(TLS 메모 함수, 키에 엔티티 id) → L607~627 range_misjudge_rng/roll·max_range_can_use/nearly → L651 camp_pos+v23_should_break… . 자식 명세(r13/r14/r15)의 tls 절이 정본.

<details><summary>인자 9개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | bumpalo::collections::vec::Vec<SmallActionPlay> (32B) | 반환 슬롯. ptr@0 · bump@+8 · cap@+0x10 · len@+0x18. 빈 Vec 은 ptr=8(dangling) cap=len=0. 원소 184B · 태그 @+0xb1(1B) · +0xb2..+0xb8 패딩 6B(미기록). J 범위에서 직접 기록: L224(Recall 1개) · L228(v27 액션 1개, from_iter_in([action])). 그 외 경로는 지역 res/act_actions/move_actions 를 만들고 K(432+)가 sret 를 채운다. \| (배치 K) ptr@0 · bump@+8 · cap@+0x10 · len@+0x18 · 원소 184B(태그 @+0xb1). 배치 K 의 반환 경로는 전부 len 1 의 from_iter_in([x]) 이거나 act_actions(%97, L244 벡터)를 32B memcpy 로 그대로 넘김(L512) | 4 |
| 1 | 1 | self | &mut EpicHuntSubPlan (1B: need_recall bool @0x0) | &mut. IR 속성 noalias·dereferenceable(1)(readonly 아님). writes = need_recall 갱신 L207(→true)·L214(→false). initializes 속성 없음(부분 갱신). \| (배치 K) 배치 K 범위에서 self 에 대한 store 0건. L476 max_by_key 클로저 env 에 포인터로 캡처돼 EpicHuntSubPlan::score(self,..) 인자로만 전달됨(aux m12 23228). 쓰기는 없음 | 4 |
| 2 | 2 | version | usize (i64 %2) | AI 버전. J 범위에서 자체 분기 없음 — 콜리(v27/v25/score/nontarget_windup_perceived/range_misjudge_rng/check_kill_die_tick/aoe_heal_covers_low_ally/position_score_at_position)에 그대로 전달. 스택 슬롯 %108 에 저장돼 클로저#7(retain) 환경에 &version 으로 캡처. \| (배치 K) %108 alloca 에 저장돼 있음(배치 J). K 범위에선 L446(v27 인라인) · L463 range_misjudge_rng · L476 epic_action_score · L486 get_input · L494 position_score_at_position 에 그대로 전달만 함. K 범위 안 version 분기 0건 | 4 |
| 3 | 3 | rnd | &mut rand::rngs::StdRng (320B, align 16) | &mut. J 범위 소비처(순서): L227 v27_objective_discipline_action · L238/L579 PlayerState::strategy(정의부 readnone=미사용) · L424 score · L550 SmallActionAroundPosition::new · L608~610(적마다 3회)·L627(die_tick<tps 시 4회째) range_misjudge_roll · L658→29 objective_wiggle_margin 의 gen_range(12000..=inner)(조건부). \| (배치 K) ★RNG 소비 순서(K 범위): ①L463 루프(적 챔프 null 아닌 순, 최대 5회) range_misjudge_roll(rnd,&jrng,min_v,max_v) ②L476 epic_action_score(..rnd..)/score 를 move_actions 원소 순으로 ③L486 get_input(..rnd..) ④(L446 경로) SmallActionAroundPosition::new_with_radius(rnd,..). 단 L446 은 L456 이전에 오므로 실제 시간순은 L446 → L463 → L476 → L486 | 4 |
| 4 | 4 | player | &PlayerState (2528B, readonly) | info.team@0x930 · info.position@tag 0x9c0 · info.parameter@0x180(AthleteParameter, L594) 직접 읽음. 나머지는 콜리 전달. \| (배치 K) info.parameter(+0x180) 로 positioning_effective/positioning_accuracy 호출 · 클로저 aux 에서 info.team(+0x930) 읽음 | 4 |
| 5 | 5 | data | &OperationData (24B, readonly) | cache@0x0(&AbstractGameWithCache) · context@0x8(&GameContext). blackboard@0x10 은 J 범위에서 안 읽음. \| (배치 K) cache(+0) · context(+8) · blackboard(+0x10) | 4 |
| 6 | 6 | parameter | &ScoreParameter (5384B, readonly) | J 범위 직접 읽기 없음. &parameter.positioning_score(+0x9f0=2544) 를 L536 position_score_at_position 에, parameter 전체를 L424 score 에 전달. \| (배치 K) positioning_score(+0x9f0) 를 position_score_at_* 와 get_input 에 전달 · positioning_score.cx/cy(+0x14a8/+0x14b0) 를 L446 후보 셀 중심으로 읽음 | 4 |
| 7 | 7 | team_plan | &TeamPlan (ptr %7, nonnull, 크기 속성 없음) | DI 이름 team_plan(!26456). J 범위: L227 v27_objective_discipline_action · L544 v25_objective_posture 의 self 로만 전달(직접 읽기 없음). \| (배치 K) 배치 K 범위에서 미사용(load 0건) | 4 |
| 8 | 8 | debug | &mut DebugFrameData (224B) | DI 이름 debug(!26457) · 타입 ref_mut$<game_core::simulation::game::frame::DebugFrameData>(!18578). &mut. J 범위 쓰기: L552 `debug.infos(+0xa0 HashMap<usize,Vec<String>>).entry(champ.id).or_default().push(format!("v25 morgard hunt posture: {:?}", posture.kind))` — context.debug(+0x3b) 가 true 일 때만. 그 외 L424 score 에 전달. \| (배치 K) ★쓰기: L448 debug.infos(+0xa0).entry(champ.id).or_insert(vec![]).push("v27 morgard safe attack position".to_string()) — context.debug(+0x3b) 가 true 일 때만. 그 밖엔 epic_action_score/score/get_input 에 &mut 로 전달(콜리 쓰기) | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
// epic_hunt.rs:0~429 (배치 J)
// 인자: self=&mut EpicHuntSubPlan{need_recall} · version · rnd · player · data{cache,context} · parameter · team_plan · debug. 반환 sret = bumpalo Vec<SmallActionPlay>.
// 자식 명세 있는 콜리는 계약만 적는다. 이하 `game` = data.cache.game(&dyn AbstractGame), `ctx` = data.context, `bump` = ctx.pool.

// ── L202 ──
let team = player.info.team(@0x930);  assert team < 2;
let champ: &Entity = cache.player_champion[team][player.info.position(@0x9c0)].unwrap();   // None 이면 unwrap 패닉(m15.ll:13017)

// ── L203~221 need_recall 갱신 (&mut self 유일한 쓰기) ──
if !self.need_recall {                                                                   // L203 (%125 false → 127)
    // L204: 에픽 엔티티 = 모바 모드의 jungle_runner.epic.live_list 첫 id → get_entity_by_id
    let mode = game.get_game_mode();  Moba 아니면 unwrap 패닉(m15.ll:13145)
    if mode.live_list.len(@0x1a8) != 0 {
        if let Some(epic) = game.get_entity_by_id(*live_list.ptr(@0x1a0)[0]) {
            // L205: dmg = epic.attack_effect.unwrap()(@0x490, tag@0x4c0==-1 → 패닉).expected_damage_target(ctx, caster=epic as &dyn AbstractEntity(@anon.56 vtable), target=champ)
            let dmg = expected_damage_target(...);
            if epic.ty@tag(@0x68) == 5 /*Epic*/ {                                           // L206
                let info = &epic.ty.Epic.info;
                // L207: 극성 = 분기방향. (A && B) || C 순서로 평가된다
                if (info.focused(@0x88/0x90) == Some(champ.id(@0x5c0)) && dmg*3 >= champ.hp(@0x670)) || champ.hp <= dmg {
                    self.need_recall = true;                                                // store i8 1 @self+0 (m15.ll:13136)
                }
            }
        }
    }
} else {                                                                                  // L213
    let hp_ratio = champ.hp*100 / champ.stat_cached.hp(@0x628);   // 0 이면 div_by_zero 패닉
    if hp_ratio > 29 { self.need_recall = false; }                                          // L214 (store i8 0)
}

// ── L221~225 귀환 단일 후보 ──
if self.need_recall {                                                                     // L221 (재읽기)
    let mut res = Vec::new_in(bump);                                                       // L222
    res.push(SmallActionPlay::Recall(SmallActionRecall::new(data, player, end_delay=5)));  // L223 (tag 4 @0xb1)
    return res;                                                                            // L224 → sret
}

// ── L227~228 목표 규율 액션 ──
if let Some(action) = team_plan.v27_objective_discipline_action(version, rnd, player, data, target=JungleType::Morgard(4)) {   // 184B, tag@0xb1 == -1 → None
    return Vec::from_iter_in([action], bump);                                              // L228 → sret (1개)
}

// ── L231~234 최근접 적 타워 ──
let champ = champ;  // L231 재바인딩(동일 값 %121)
let nearest_enemy_tower: Option<&Entity> = cache.iter_towers_without_nexus(team=1-team)   // L232 (6 고정 타워 Option 배열 + twin_towers 슬라이스 Chain)
    .filter(|x| x.can_target())      // L233 closure#2 = x.can_target(@0x6b9) && x.block_target_tick(@0x6a0)==0  (entity.rs:1478)
    .min_by_key(|x| x.distance_sq(champ));   // L234 closure#3 = |x.x-c.x|²+|x.y-c.y|² (@0x660/0x668) · 동률이면 앞 원소

// ── L237 공격 후보 원천 = get_act_action(version, rnd, player, data) 인라인(epic_hunt.rs:670~676) ──
let mut act_actions = Vec::new_in(bump);                                                   // 671
act_actions.extend(fight_check::battle_action(_version, _rnd, player, data, _end_delay));  // 672 (미사용 인자는 poison 으로 전달)
act_actions.extend(fight_check::attack_summon_action(player, data));                       // 673
act_actions.extend(self.attack_jungle_action(_rnd, player, data));                          // 674 (fastcc: player→(team,position) 승격, self·rnd 소거)

// ── L238~240 킬 우선 전략 시 에픽 참조 ──
let object_finish_objective: Option<&Entity> =
    if player.strategy(rnd, game).object_finish(@0xf) == KillPriority(0) {                  // L238 (strategy 정의부에서 rnd 미사용)
        game.get_game_mode() /*Moba 아니면 패닉*/ .live_list.first().and_then(|id| game.get_entity_by_id(*id))   // L239~240
    } else { None };

// ── L244~313 act_actions 필터 (closure#5, 캡처: game·player·&object_finish_objective·champ·&nearest_enemy_tower) → clone 수집 ──
let act_actions: Vec<_> = act_actions.iter().filter(|action| {
    // L246~249: 킬우선 무시 대상이면 제거
    if let Some(id) = action.target_id() /*tag 15~18 의 +8*/ {
        if game.get_entity_by_id(id).is_some_and(|t| fight_model::should_ignore_object_finish_kill_priority_target(player, t, object_finish_objective)) { return false; }
    }
    // L253: 대상이 챔피언이 아니면 무조건 유지
    if let Some(id) = action.target_id() { if let Some(t) = game.get_entity_by_id(id) { if !t.is_champion() /*ty@0x68 != 13*/ { return true; } } }
    // L254: 챔피언 대상 공격류만 세부 판정. 그 외 variant(RunAway~Trace·AroundPosition·Stop) → false
    match action {
      Attack(a) => {                                                                     // L257~261
        let Some(target) = game.get_entity_by_id(a.target_id) else { return false };
        let eff = champ.attack_effect.as_ref().unwrap();                                    // L258
        eff.is_in_range(caster=champ, target)                                              // L259
          && nearest_enemy_tower.is_none_or(|t| {                                          // L259~260 (closure#4)
               let tower_hits_me = t.attack_effect.unwrap().is_in_range(t, champ);
               !(tower_hits_me && target.ty != Tower(2)) || target.team == champ.team })   // 팀 판별 동일: Player 면 인덱스까지 비교, Neutral 끼리면 true
      }
      Skill(a) => {                                                                      // L267~272
        let Some(target) = get_entity_by_id(a.target_id) else { return false };
        let eff = champ.skill_effect.as_ref().unwrap();                                     // L268 (@0x4c8, tag@0x4f8)
        eff.is_in_range(champ, target)                                                     // L269
          && nearest_enemy_tower.is_none_or(|t| !(t.atk.is_in_range(t,champ) && target.ty != Tower) || target.team == champ.team)   // L269~270
          && ( !(eff.ty.expected_move_on_hit() /*vt+0x68*/ || eff.ty.expected_rush_effect() /*vt+0x60*/)   // L270: 이동/돌진 스킬이 아니면 통과
               || nearest_enemy_tower.is_none_or(|t| !t.attack_effect.unwrap().is_in_range_ex(caster=t, target, cx=t.x, cy=t.y, tx=target.x, ty=target.y, offset=15000)) )   // L271~272 closure#6
      }
      Skill2(a) => { 동일 골격, eff = champ.skill2_effect() /*level(@0x5c8)>2 ? &@0x500 : None*/ .unwrap(); L282~287, closure#8 }
      Ult(a)    => { 동일 골격, eff = champ.ult_effect()    /*level>4 ? &@0x538 : None*/ .unwrap(); L297~302, closure#10 }
      _ => false
    }
}).map(|a| a.clone()).collect_in(bump);   // L313 (from_iter_in @L244)

// ── L315~423 retain #1 (closure#7, 캡처: data·champ·player·&version) — 아군 대상 힐/실드 정리 ──
act_actions.retain(|action| match action {
    Skill(a) | Skill2(a) | Ult(a) => {                                                    // L316 (Skill 318~336 / Skill2 352~370 / Ult 386~404 동형)
        let Some(target) = game.get_entity_by_id(a.target_id) else { return false /*제거*/ };   // L318/352/386
        if target.team != champ.team { return true /*적 대상은 유지*/ }                     // L319/353/387
        let Some(eff) = champ.skill_effect /*Skill2: skill2_effect() · Ult: ult_effect()*/ else { return false };   // L321/355/389
        let heal   = eff.ty.expected_heal(ctx, champ, @anon.11) != 0;                      // L322 vt+0x40
        let shield = eff.ty.expected_shield(ctx, champ, @anon.11) != 0;                    // L323 vt+0x48
        let buff   = eff.ty.expected_buff(ctx, champ, @anon.11).is_some();                // L324 vt+0x50 → Option<BuffState>(@0x48 != -1)
        let near = |e: &Entity| target.distance_sq(e) < 120000²+1 && (champ.team==Neutral || e.visible_state[champ.team]==Visible);
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
act_actions.retain(|action| self.score(version, parameter, rnd, player, data, action, debug) >= -30);   // L425~426 (i64 signed)

// ── L429 move_actions = get_move_action(version, rnd, player, data, &parameter.positioning_score(+0x9f0), team_plan, debug) 인라인(epic_hunt.rs:517~668) ──
let move_actions: Vec<SmallActionPlay> = {
    let mut res = Vec::new_in(bump);                                                       // 518
    let champ = cache.player_champion[team][pos].unwrap();                                 // 520
    // 523~530: 적 챔프가 비대상 스킬 예비동작으로 나를 겨눌 수 있는가
    let has_non_target_action_range = cache.player_champion[1-team].iter().flatten().any(|c|
        utils::nontarget_windup_perceived(version, player, data, caster=c) && c.is_champion()
        && match c.action_state@tag(@0x70) {
             4 /*Skill*/  => { let e = c.skill_effect.unwrap(); matches!(e.casting(@0x4f8), Position(1)|Direction(2)) && e.is_in_range(c, champ) }   // 525
             5 /*Skill2*/ => { let e = c.skill2_effect().unwrap(); 같은 조건 }              // 527
             6 /*Ult*/    => { let e = c.ult_effect().unwrap();    같은 조건 }              // 529
             _ => false });
    // 536~541
    let ps = position_eval::position_score_at_position(version, player, data, positioning_score, x=champ.x, y=champ.y, purpose=Objective(11));   // 56B PositioningScore
    let on_trajectory = ps.on_periodic_trajectory(@0x31);
    if ps.on_trajectory(@0x30) || has_non_target_action_range || on_trajectory {          // 538
        res.push(RunAway(SmallActionRunAway::new_with_skill(data, player, 5, with_skill=true)));   // 540 (tag 3)
        return res;                                                                        // 541
    }
    // 544~554 목표 태세
    let posture = team_plan.v25_objective_posture(version, player, data, target=Morgard(4));   // Option<ObjectivePosture>(88B, +0 i64==-1 → None)
    let mut objective_in_attack_range = false;
    if let Some(p) = &posture {                                                              // 545
        match p.kind(@0x50) {
          WaitGroup(3) | SoftDisengage(4) => {                                              // 546
              if p.kind == SoftDisengage && p.near_enemy_count(@0x38) != 0 { res.push(RunAway(new_with_skill(data, player, 5, false))); }   // 547~548
              res.push(AroundPosition(SmallActionAroundPosition::new(rnd, data, target_x=p.wait_pos.0(@0x20), target_y=p.wait_pos.1(@0x28), 5)));   // 550 (untagged variant)
              if ctx.debug(@0x3b) { debug.infos(@0xa0).entry(champ.id).or_default().push(format!("v25 morgard hunt posture: {:?}", p.kind)); }   // 551~553
              return res;                                                                    // 554
          }
          Screen(2) if p.focus_enemy(@0x0).is_some() => { res.push(Trace(SmallActionTrace::new_attack_range(data, target=p.focus_enemy.unwrap()(@0x8), 5))); }   // 557~559 (tag 14) — 계속 진행
          _ => {}
        }
    }
    // 565~574 에픽 사거리 판정
    let epic: Option<&Entity> = game.get_game_mode()/*Moba 아니면 패닉*/ .live_list.first().and_then(|id| game.get_entity_by_id(*id));   // 565
    if let Some(e) = epic {                                                                  // 566
        let mr = objective_attack_range(champ, e);                                           // 567 = epic_hunt.rs:16~17 인라인: champ.attack_effect.unwrap().range(champ, e) + champ.radius() + e.radius()
              // effect.rs:26 range() = range(@0x4a0) + growth_range(@0x4a8)*(level-1) + caster.stat_buff_cached.range(@0x438) + range_adjust(caster, target)
              // entity.rs:1511 radius() = radius_mult(@0x470)==0 ? radius(@0x680) : radius*(100+mult)/100
        if champ.distance_sq(e) > mr*mr {                                                    // 569
            let camp = ctx.map.camp_pos(ty=Morgard(4), is_blue_side = team==0);                // 570
            if !objective_helpers::v23_should_break_objective_hunt_anchor(player, data, champ, objective=e, camp.0, camp.1)   // 570
               && !posture.map_or(false, |p| matches!(p.kind, WaitGroup|SoftDisengage)) {    // 571
                res.push(Trace(SmallActionTrace::new_attack_range(data, target=e.id, 5)));   // 572
                return res;                                                                  // 573~574
            }
            // (break 이거나 WaitGroup/SoftDisengage 면 epic 은 Some 으로 유지하고 계속)
        } else { objective_in_attack_range = true; }                                          // 569 else
    }
    // 579~633 근접 적 순회 (추격/도주 판정)
    let object_finish_battle = player.strategy(rnd, game).object_finish != KillPriority;     // 579 (BattlePriority)
    let mut runaway = false; let mut force_runaway = false;
    let pa = player.info.parameter(@0x180).positioning_accuracy();                            // 594
    let (min_v, max_v) = (pa, 2000 - pa);                                                    // 594~596
    let near_enemies: Vec<&Entity> = cache.iter_champions(1-team)                            // 598 closure#1: (champ.team Neutral || e.visible_state[champ.team]==Visible) && champ.distance_sq(e) < 160000²
        .filter(..).collect_in(bump);
    let me_die_tick = fight_check::check_kill_die_tick(version, _rnd, data, judger=player, focus=champ, enemy=&near_enemies.clone(), towers=&Vec::new_in(bump), _debug);   // 599~600
    for enemy in near_enemies.iter() {                                                       // 602
        if let Some(e) = epic { if !object_finish_battle && !fight_model::can_enemy_hit_objective(enemy, objective=e, range_margin=25000) { continue; } }   // 603 (킬우선이면 에픽을 못 때리는 적은 무시)
        let jrng = utils::range_misjudge_rng(version, data, player, enemy_id=enemy.id);       // 607 ({i64,i64} 16B, 이후 &jrng)
        let mr       = battle::max_range_can_use(champ, enemy)        * utils::range_misjudge_roll(rnd, &jrng, min_v, max_v) / 1000;   // 608 (롤1)
        let emr      = battle::max_range_can_use(enemy, champ)        * range_misjudge_roll(rnd, &jrng, min_v, max_v) / 1000;          // 609 (롤2)
        let emr_near = battle::max_range_nearly_can_use(enemy, champ, 40) * range_misjudge_roll(rnd, &jrng, min_v, max_v) / 1000;    // 610 (롤3)
        let dist = champ.distance_sq(enemy);                                                 // 611
        if me_die_tick < ctx.setting.tick_per_second(@0x12f8) {                              // 614 (1초 내 사망 예측)
            let emr = battle::max_range_nearly_can_use(enemy, champ, 60) * range_misjudge_roll(rnd, &jrng, min_v, max_v) / 1000;   // 627 (롤4, 그늘 변수)
            if dist > mr*mr && emr < mr { res.push(Trace(SmallActionTrace::new(data, target=enemy.id, 5))); }   // 629~631
            else if dist <= emr*emr { runaway = true; force_runaway = true; }                // 632~633
        } else if enemy.remain_action_time() > 10 && mr > 0 /*IR: roll*range > 999*/ {      // 615
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
    let break_objective_anchor = epic.is_some_and(|e| v23_should_break_objective_hunt_anchor(player, data, champ, e, camp_pos(map, Morgard, team==0)));   // 651
    if runaway && (break_objective_anchor || !objective_in_attack_range || force_runaway) {   // 653
        res.push(RunAway(SmallActionRunAway::new_with_skill(data, player, 5, false)));       // 654
    } else if objective_in_attack_range {                                                    // 655 (runaway 이고 위 조건 거짓이면 반드시 in_range)
        res.truncate(0);                                                                     // 656 (res.clear())
        if let Some(e) = epic {                                                              // 657
            let range = objective_attack_range(champ, e);                                    // 658
            let margin = objective_wiggle_margin(rnd, champ, range);   // epic_hunt.rs:20~30 인라인:
                // let mut inner = clamp(range.saturating_sub(10000), 12000, 45000);  let dist = champ.distance(e);
                // if dist + 15000 < range { inner = if inner + dist > range { rnd.gen_range(12000..=inner) } else { 12000 } }
                // margin = inner
            res.push(Trace(SmallActionTrace::new_attack_range_margin(data, target=e.id, 5, attack_range_margin=margin)));   // 659
        } else { res.push(Stop); }                                                           // 661 (tag 19)
    } else if res.is_empty() {                                                               // 663
        res.push(RunAway(SmallActionRunAway::new_with_skill(data, player, 5, false)));       // 664
    }
    res                                                                                      // 667 (near_enemies drop)
};
// → 배치 K(줄 432, 블록 %1128): nearest_tower / attacking_objective / best_move_action 선별 후 sret 기록.
// 예외 경로: L224·L228 은 J 가 sret 를 직접 채우고 ret(%310). 언와인드 cleanup(%357/%367/%703/%1987) 은 지역 Vec drop 만.

// epic_hunt.rs:432~515 (배치 K)
// 진입 문맥(배치 J 소관, 계약만): champ=%121=cache.player_champion[team][pos](L202) · team=%110=player.info.team · nearest_enemy_tower=%102(Option<&Entity>, L232/L234) · act_actions=%97(L244 벡터) · act_actions0=%100(L237 벡터) · move_actions=%93(L429 벡터) · version=%108 · bump=%350=context.pool. 배치 J 의 L429 블록에서 %1128 로 진입(m15.ll 15685).

// ---- L432~L437 타워 어그로 도주 (m15.ll 15689~15765) ----
if let Some(nearest_tower) = nearest_enemy_tower {                      // L432 null 검사
  if nearest_tower.ty@tag(+0x68) == 2 /*Tower*/ {                       // L433 → info = &Tower(+0x70)
    if info.nearest_enemy.is_some()(+0x88 tag)                          // L434
       && info.nearest_enemy.1(+0x98) == champ.id(+0x5c0) {             //      타워가 나를 조준중
      act_actions.truncate(0);                                          // L435 (%97)
      move_actions.truncate(0);                                         // L436 (%93)
      move_actions.push(SmallActionPlay::RunAway(                       // L437 태그 3 @+0xb1
        SmallActionRunAway::new_with_skill(data, player, end_delay=5, with_skill=true)));
    }
  }
}

// ---- L442~L451 에픽 공격 중이면 v27 안전 공격 위치 ----
if !act_actions.is_empty() {                                            // L442 len(+0x18)!=0, 비면 → L456
  // L443: let GameMode::Moba(moba) = game.get_game_mode()(vtable+0x40) else unwrap_failed(443:71);
  //       let Some(epic) = moba.jungle_runner.epic.live_list.first()(+0x1a8 len, +0x1a0 ptr)
  //                        .and_then(|id| game.get_entity_by_id(*id)(vtable+0x1f0)) else → L456
  if let Some(epic) = ... {
    // L444: attacking_objective = act_actions.iter().any(|a| (a.tag(+0xb1)-15) <u 4 /*Attack|Skill|Skill2|Ult*/ && a.target(+0x8) == epic.id)
    //       (루프 진입 전 len==0 재검사 → L457, 컴파일러 잔재)
    if attacking_objective {                                            // L445 (거짓이면 → L456)
      // L446 (m15.ll 15898~17058): let action = self.v27_objective_safe_attack_position(version, rnd, player, data, &parameter.positioning_score, epic) — 완전 인라인, 아래 v27 절
      if let Some(action) = action {                                    // @+0xb1 != 0xFF
        if context.debug(+0x3b) {                                       // L447
          debug.infos(+0xa0).entry(champ.id).or_insert(Vec::new())      // L448
               .push("v27 morgard safe attack position".to_string());
        }
        return bumpalo::vec![in bump; action];                          // L450 from_iter_in([action]) · L451 = 언와인드 drop
      }
    }
  }
}

// ---- v27_objective_safe_attack_position (epic_hunt.rs:43~145, L446 인라인) → Option<SmallActionPlay> ----
//  L43  let Some(champ) = data.cache.player_champion[team][pos](%120 재로드) else None
//  L44  objective_hp_ratio = epic.hp(+0x670)*100 / max(epic.stat_cached.hp(+0x628),1)
//  L45  if objective_hp_ratio < 36 → None
//  L49  attack_range = objective_attack_range(champ, epic)   // epic_hunt.rs:16~17 인라인:
//         L16 atk = champ.attack_effect(+0x4c0 tag; -1=None → unwrap panic 16:46).as_ref().unwrap()  (&Effect = champ+0x490)
//         L17 = [effect.rs:26 Effect::range] champ.stat_buff_cached.range(+0x438) + atk.range(+0x4a0) + (champ.level(+0x5c8)-1)*atk.growth_range(+0x4a8)
//              + Effect::range_adjust(atk, champ, epic)
//              + radius(champ) + radius(epic)   // entity.rs:1511~1515: mult=stat_buff_cached.radius_mult(+0x470, i32); mult==0 ? radius(+0x680) : radius*(mult+100)/100
//  L52  if dist²(champ, epic) > attack_range² → None            // 사거리 밖이면 안전위치 불필요
//  L56~60 threats: bumpalo Vec<&Entity> = cache.player_champion[1-team](%381..%382).iter_champions() (None 제거)
//         .filter(closure$0: e ↦ blackboard[player.info.team].is_recent_visible(game, player, e)
//                              && (dist²(e,champ) ≤ 260000² || dist²(e,epic) ≤ 220000²))  [aux m15 57952~58059]
//  L61  if threats.is_empty() → drop, None
//  L65  hp_ratio = champ.hp*100 / max(champ.max_hp,1)
//  L67~68 current_nearest_enemy = threats.iter().map(|t| distance(champ,t)).min().unwrap()   [aux m12 31526 fold, first 원소는 본체 inline]
//  L70~72 threat_contact = threats.iter().any(|t| dist²(champ,t) ≤ (max(max_range_nearly_can_use(t, champ, 50), 100000) + (hp_ratio<50 ? 70000 : 40000))²)
//  L74  if !threat_contact && !(hp_ratio < 45 && current_nearest_enemy < 220001) → drop, None
//  L78  keep_attack_range = attack_range.saturating_sub(30000)
//  L83~86 allies: Vec<&Entity> = cache.player_champion[team](%119..+40).iter_champions().filter(closure$1: a ↦ a.id != champ.id && dist²(a,epic) ≤ 220000²)  [aux m15 58062~58118]
//  L87~91 ally_centroid = allies.is_empty() ? None : Some((Σa.x/n, Σa.y/n))   (u64 정수 나눗셈) ; allies drop
//  L95  cohesion(x,y) := ally_centroid.map_or(0, |(ax,ay)| (320000).saturating_sub(distance((x,y),(ax,ay))) / 10000)   (closure$6, 0..32)
//  L98~99 xi = parameter.positioning_score.cx(+0x14a8) as i32, yi = .cy(+0x14b0) as i32
//  L100~101 cur_pv = v27_positioning_value(version, player, position_score_at_position(version, player, data, &positioning_score, champ.x, champ.y, 11))
//       v27_positioning_value(L34~38): positioning = player.info.parameter.positioning_effective();
//         gain_weight = clamp(100 + (50-positioning)/10, 95, 105); risk_weight = clamp(100 + (positioning-50)/2, 75, 125);   (sdiv, 0 방향 절삭)
//         trajectory_penalty = (score.on_trajectory(+0x30) || score.on_periodic_trajectory(+0x31)) ? max(positioning-50, 0) : 0;
//         value = score.gain(+0x10)*gain_weight/100 - score.risk(+0x0)*risk_weight/100 - trajectory_penalty   (i64 sdiv)
//  L102~103 current_score = cur_pv + min(current_nearest_enemy, 320000)/10000 + 8 + cohesion(champ.x, champ.y)   // ★8 = 현 위치 range_slack 만점 가정(추정, 상수 표 참조)
//  L105 best: Option<(x,y,score,nearest)> = None
//  L106~135 for dx in 0..7 { xb = xi-3+dx; if xb > 29 { continue }        // 내부 루프 진입 조건
//             for dy in 0..7 { yb = yi-3+dy;
//               L110 if xb<0 || yb<0 || yb>29 || map.walls[yb][xb](+0x78, [30][30]) != 0 → continue
//               L114~115 x = xb*32000+16000, y = yb*32000+16000
//               L116 if dist²((x,y), epic) > keep_attack_range² → continue          // 에픽을 계속 때릴 수 있는 셀만
//               L120~123 nearest_enemy = threats.iter().map(|t| distance((x,y),t)).min().unwrap_or(u64::MAX)   [aux m12 31422 fold]
//               L124~125 pv = v27_positioning_value(version, player, position_score_at_cell(version, player, data, &positioning_score, xb, yb, 11))
//               L126 enemy_spacing = min(nearest_enemy, 320000)/10000
//               L127 range_slack = min(keep_attack_range.saturating_sub(distance((x,y), epic)), 80000)/10000
//               L128 score = pv + enemy_spacing + range_slack + cohesion(x,y)
//               L130~131 if best.map_or(true, |b| score > b.score) { best = Some((x,y,score,nearest_enemy)) }   // 동점 유지(먼저 것)
//             } }
//  L136 let Some(best) = best else → drop threats, None
//  L137 if best.score < current_score && best.nearest < current_nearest_enemy.saturating_add(30000) → None   // 점수도 낮고 적과의 간격도 충분히 안 벌어지면 이동 안 함
//  L140 if dist²(champ, best) < 12000²+1 (=≤12000) → None                    // 사실상 제자리
//  L144 Some(SmallActionPlay::AroundPosition(SmallActionAroundPosition::new_with_radius(rnd, data, best.x, best.y, end_delay=3, around_radius=25000)))   // 태그 없음(untagged), @+0xb1=outline_type
//  L145 threats drop

// ---- L456~L512 공격 후보 없을 때의 이동 후보 확정 ----
if act_actions.is_empty() {                                             // L456 (L442 거짓·L443 None·L445 거짓·L446 None 전부 여기로)
  positioning_accuracy = player.info.parameter.positioning_accuracy();  // L457
  min_v = positioning_accuracy; max_v = 2000 - positioning_accuracy;    // L458~459
  // L463~473: 아웃레인지 판정 — 적 챔프(cache.player_champion[1-team], 5칸, None 건너뜀) 를 find(closure$12):
  //   L464 jrng = range_misjudge_rng(version, data, player, c.id)               (16B 시드, rnd 미소비)
  //   L465 emr = max_range_can_use(c, champ) * range_misjudge_roll(rnd, &jrng, min_v, max_v) / 1000 + 10000
  //   L466 mr  = max_range_can_use(champ, c)
  //   L468 if !blackboard[1-team].is_recent_visible(game, player, c) → false
  //   L469 if dist²(c, champ) > emr² → false                                    (적이 나를 사거리(오판 포함) 안에 둠)
  //   L470 if c.is_in_action() (ty==Champion(13) && action_state.tag(+0x70) >= 3) → false
  //   L472 if mr != 0 || c.block_input() → false                                (내가 c 를 칠 수 있거나 c 가 행동불능이면 제외)
  //   → true
  if let Some(_) = ... {                                                 // L463 성립
    return bumpalo::vec![in bump; RunAway(SmallActionRunAway::new_with_skill(data, player, 5, false))];   // L473
  }
  // L476: best_move_action = move_actions.iter().max_by_key(|a| self.score(version, rnd, player, data, parameter, a, debug)).unwrap()
  //        (첫 원소는 epic_action_score(version,rnd,player,data,parameter,a,debug) 로 인라인 · 나머지는 aux m12 23134 fold 에서 EpicHuntSubPlan::score 호출 · 동점이면 뒤 원소 · move_actions 비면 unwrap panic 476:128)
  // L480~482: trajectory_possible = game.iter_projectile()(vtable+0x210).any(closure$14: p ↦
  //        p.team != TeamType::Player(team)   (+0x0 tag==0 && +0x8==team 이면 제외)
  //        && !p.is_targeting()  [projectile.rs:134: move_type(+0x40) ∈ {Target(6), TargetSplash(7), BouncingTarget(암묵)이고 +0x40==1}]
  //        && dist²(champ, (p.x(+0x100), p.y(+0x108))) < 420000²)
  // L483~484: enemy_rushing = cache.player_champion[1-team].iter().flatten().any(closure$15: c ↦ matches!(c.rush_state(+0x308), Rush(0x8000000000000003) | RushPenetrate(암묵: signed>=0)))
  //   (trajectory_possible 가 true 면 L484 는 평가 안 함 — %1921→%1922)
  if trajectory_possible || enemy_rushing {                              // L483 → L485/486
    move_action_input = best_move_action.clone().get_input(version, rnd, player, data, &parameter.positioning_score, debug);   // L486 (Option<Input> 32B) · L487 clone drop
    if let Some(Input::Move{x,y}) = move_action_input {                 // L490 tag 0 · -1=None → L507 · 그 외 → L504
      position_score = position_score_at_position(version, player, data, &positioning_score, x, y, 11);   // L494
      on_trajectory = position_score.on_trajectory(+0x30) || position_score.on_periodic_trajectory(+0x31);   // L496
      if on_trajectory {                                                 // L498
        return bumpalo::vec![in bump; RunAway(SmallActionRunAway::new_with_skill(data, player, 5, false))];   // L499 (목적지가 투사체 궤도 위면 도주로 대체)
      } else {
        return bumpalo::vec![in bump; best_move_action.clone()];        // L501
      }
    } else if move_action_input.is_some() {                              // Move 아닌 Input
      return bumpalo::vec![in bump; best_move_action.clone()];          // L504
    }
  }
  return bumpalo::vec![in bump; best_move_action.clone()];              // L507 (조건 불성립·None 모두)
} else {
  return act_actions;                                                    // L512 %97 → sret 32B memcpy (move)
}
// L514~515: 함수 끝 drop — move_actions(%93) · act_actions %97(L512 제외) · act_actions0 %100 순. 언와인드 정리 블록 %1143/%357/%311 도 같은 순서.
// 다른 배치로 넘어가는 지점: 배치 K 진입은 배치 J 의 %575/%1109/%1121 → %1128 뿐. K 에서 J 로 되돌아가는 br 없음(반환·언와인드 패드 %357/%311/%352 는 공용 정리 블록)
```

**`mem` 메모리 접근 109건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | EpicHuntSubPlan | 0x0 | need_recall | r | L203(분기)·L221(재읽기, 갱신 후) | 4 | OK |  |
| 1 | PlayerState | 0x930 | info.team | r | L202 bounds<2 · 1-team 으로 적 진영 인덱스 \| (배치 K) closure$0(aux m15 57967) blackboard[team] 인덱스(bounds check <2). 본체의 team(%110)은 배치 J 가 L202 에서 읽은 값 | 4 | OK |  |
| 2 | PlayerState | 0x9c0 | info.position@tag (u32) | r | L202 player_champion[team][position] | 4 | OK |  |
| 3 | PlayerState | 0x180 | info.parameter (AthleteParameter) | r | L594 positioning_accuracy(&self) | 4 | OK |  |
| 4 | OperationData | 0x0 | cache | r | &AbstractGameWithCache | 4 | OK |  |
| 5 | OperationData | 0x8 | context | r | &GameContext | 4 | OK |  |
| 6 | AbstractGameWithCache | 0x0 | game (&dyn AbstractGame: data@0, vtable@+8) | r | vtable 슬롯 +0x40 get_game_mode · +0x1f0 get_entity_by_id (divtable) | 3 | OK |  |
| 7 | AbstractGameWithCache | 0x1e0 | player_champion[2][5] (Option<&Entity>) | r | L202/L520 [team][pos] · L325/L523/L598 [1-team] 순회 | 4 | OK |  |
| 8 | AbstractGameWithCache | 0xd0 | jungles.ptr | r | L327 순회(closure#7) | 4 | OK |  |
| 9 | AbstractGameWithCache | 0xe8 | jungles.len | r | L327 | 4 | OK |  |
| 10 | AbstractGameWithCache | 0xf0 | others[2] (stride 32: ptr@0 · len@+0x18) | r | L328(closure#7)·L640 [1-team] 순회 | 4 | OK |  |
| 11 | GameContext | 0x0 | pool (&Bump) | r | Vec::new_in 의 bump (L222/L237/L429/L598) | 4 | OK |  |
| 12 | GameContext | 0x8 | setting (&GameSetting) | r | L614 | 4 | OK |  |
| 13 | GameContext | 0x20 | map (&MapDef) | r | L570/L651 camp_pos | 4 | OK |  |
| 14 | GameContext | 0x3b | debug (bool) | r | L551 디버그 로그 게이트 | 4 | OK |  |
| 15 | GameSetting | 0x12f8 | tick_per_second | r | L614 me_die_tick < tps | 4 | OK |  |
| 16 | GameMode | 0x0 | @tag (0=Moba) | r | get_game_mode 반환 {i64,ptr}; Moba 아니면 unwrap 패닉(L204/L239/L565) | 4 | OK |  |
| 17 | MobaMode | 0x1a0 | jungle_runner.epic.live_list.ptr | r | first() = 에픽 엔티티 id \| (배치 K) L443 live_list[0] = 에픽 엔티티 id | 4 | OK |  |
| 18 | MobaMode | 0x1a8 | jungle_runner.epic.live_list.len | r | 0 이면 에픽 없음 \| (배치 K) L443 ==0 → 에픽 없음 | 4 | OK |  |
| 19 | Entity | 0x0 | team@tag (TeamType: 0 Player / 1 Neutral) | r | closure#5 L260/270/285/300 target.team==champ.team · closure#7 L319 · 가시성 인덱스(Neutral 이면 무조건 통과) | 4 | OK |  |
| 20 | Entity | 0x8 | team@Player.0 (usize) | r | 팀 인덱스(visible_state 첨자, bounds<2) | 4 | OK |  |
| 21 | Entity | 0x38 | visible_state[2] (stride 24 · tag@0: 0=Visible) | r | closure#7 L325~327 · closure#1 L598: e.visible_state[champ.team]==Visible | 4 | OK |  |
| 22 | Entity | 0x68 | ty@tag (EntityType) | r | 5=Epic(L206) · 13=Champion(L253·L523 is_champion) · 2=Tower(closure#5) | 4 | OK |  |
| 23 | Entity | 0x70 | ty@Champion.0.action_state@tag | r | L525 is_in_skill: 4=Skill 5=Skill2 6=Ult \| (배치 K) L470 is_in_action = Champion && tag>=3(Attack/Skill/Skill2/Ult) | 4 | OK |  |
| 24 | Entity | 0x88 | ty@Epic.info.focused@tag (Option<usize>) | r | L207 | 4 | OK |  |
| 25 | Entity | 0x90 | ty@Epic.info.focused@Some.0 | r | L207 == champ.id | 4 | OK |  |
| 26 | Entity | 0x438 | stat_buff_cached.range | r | effect.rs:26 range() 인라인(L567/L642/L658) \| (배치 K) Effect::range 합산항 | 4 | OK |  |
| 27 | Entity | 0x470 | stat_buff_cached.radius_mult (i32) | r | entity.rs:1511 radius() 인라인 | 4 | OK |  |
| 28 | Entity | 0x490 | attack_effect (Option<Effect> 56B) | r | L205/L258/L567/L641/L658 · closure#2·#6·#7 | 4 | OK |  |
| 29 | Entity | 0x4a0 | attack_effect.range | r | range() 인라인 | 4 | OK |  |
| 30 | Entity | 0x4a8 | attack_effect.growth_range | r | range() 인라인: growth*(level-1) | 4 | OK |  |
| 31 | Entity | 0x4c0 | attack_effect@tag (casting, -1=None) | r | unwrap 패닉 게이트 | 4 | OK |  |
| 32 | Entity | 0x4c8 | skill_effect (Option<Effect>) | r | L268/L321/L525(casting@0x4f8: 1 Position·2 Direction 만) | 4 | OK |  |
| 33 | Entity | 0x4f8 | skill_effect@tag/casting | r | -1=None | 4 | OK |  |
| 34 | Entity | 0x500 | skill2_effect (Option<Effect>) | r | entity.rs:1693 skill2_effect(): level>2 일 때만, 아니면 정적 None(@anon.58/32) | 4 | OK |  |
| 35 | Entity | 0x538 | ult_effect (Option<Effect>) | r | entity.rs:1701 ult_effect(): level>4 일 때만 | 4 | OK |  |
| 36 | Entity | 0x5c0 | id | r | L207 · L552 · Trace 대상 id · range_misjudge_rng enemy_id \| (배치 K) L434 champ.id · L444 epic.id(공격 후보 target 비교) · L448 debug 키 · L463 c.id(range_misjudge_rng) · closure$1 ally.id!=champ.id | 4 | OK |  |
| 37 | Entity | 0x5c8 | level | r | skill2/ult_effect 게이트 · range() 성장 \| (배치 K) growth_range 계수 | 4 | OK |  |
| 38 | Entity | 0x628 | stat_cached.hp (최대 HP) | r | L213 champ · L329 target | 4 | OK |  |
| 39 | Entity | 0x660 | x | r | distance_sq 인라인(utils.rs:7 abs_diff²합) \| (배치 K) 거리 계산 전반(champ/epic/threat/ally/enemy) | 4 | OK |  |
| 40 | Entity | 0x668 | y | r | 거리 계산 전반 | 4 | OK |  |
| 41 | Entity | 0x670 | hp | r | L207 dmg 비교 · L213/L329 hp_ratio \| (배치 K) v27 L44 epic.hp · L65 champ.hp | 4 | OK |  |
| 42 | Entity | 0x680 | radius | r | radius() 인라인 \| (배치 K) objective_attack_range 에 champ.radius()+target.radius() 가산 | 4 | OK |  |
| 43 | Entity | 0x6a0 | block_target_tick | r | closure#2 can_target(): ==0 | 4 | OK |  |
| 44 | Entity | 0x6b9 | can_target (bool) | r | closure#2 can_target() | 4 | OK |  |
| 45 | Effect | 0x0 | ty (Arc<dyn EffectType>: ArcInner ptr@0 · vtable@+8) | r | 데이터 = ptr + ((vtable.align-1)&~15) + 16. vtable 슬롯 +0x40 expected_heal · +0x48 expected_shield · +0x50 expected_buff · +0x60 expected_rush_effect · +0x68 expected_move_on_hit (divtable 94% 일치 @anon…1131 g02.ll) | 3 | OK |  |
| 46 | Effect | 0x30 | casting (CastingType) | r | L525: 1 Position / 2 Direction | 4 | OK |  |
| 47 | Strategy | 0xf | object_finish (ObjectFinishStrategy: 0 KillPriority / 1 BattlePriority) | r | L238 · L579 | 4 | OK |  |
| 48 | ObjectivePosture | 0x0 | focus_enemy@tag (Option<usize>; i64 -1 = Option<ObjectivePosture>::None 니치) | r | L545/L557/L571 | 4 | OK |  |
| 49 | ObjectivePosture | 0x8 | focus_enemy@Some.0 | r | L558 Trace 대상 | 4 | OK |  |
| 50 | ObjectivePosture | 0x20 | wait_pos.0 | r | L550 | 4 | OK |  |
| 51 | ObjectivePosture | 0x28 | wait_pos.1 | r | L550 | 4 | OK |  |
| 52 | ObjectivePosture | 0x38 | near_enemy_count | r | L547 !=0 | 4 | OK |  |
| 53 | ObjectivePosture | 0x50 | kind (0 Commit 1 HoldCamp 2 Screen 3 WaitGroup 4 SoftDisengage) | r | L546/L547/L557/L571 | 4 | OK |  |
| 54 | PositioningScore | 0x30 | on_trajectory | r | L538 | 4 | OK |  |
| 55 | PositioningScore | 0x31 | on_periodic_trajectory | r | L538 (DI 지역명 on_trajectory 로 바인딩) | 4 | OK |  |
| 56 | BuffState(Option) | 0x48 | duration@tag (i32 -1 = None) | r | closure#7 L324 expected_buff(...).is_some() | 4 | OK |  |
| 57 | SmallActionPlay | 0xb1 | @tag | r | 3 RunAway…19 Stop · 니치 -1 = Option None(L227) | 4 | OK |  |
| 58 | SmallActionPlay | 0x8 | Attack/Skill/Skill2/Ult.target_id | r | small_action.rs:309 get_action().target_id (태그 15~18 만) | 4 | 부분일치(첫 성분(컨테이너)만 맞고 **마지막 성분이 다르다** — 사람이 봐야 ) |  |
| 59 | DebugFrameData | 0xa0 | infos (HashMap<usize,Vec<String>>) | r | L552 entry | 4 | OK |  |
| 60 | Option<&Entity>(nearest_enemy_tower, %102 alloca) | 0x0 | ptr(null=None) | r | L432 if let Some(nearest_tower) — 배치 J 가 L232/L234 에서 채운 값 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 61 | Entity | 0x68 | ty@tag | r | L433 ==2 (EntityType::Tower) · L470(closure$12) !=13 (Champion) | 4 | OK |  |
| 62 | Entity | 0x88 | ty@Tower.info.nearest_enemy@tag (=Tower+0x18, Option<(usize,usize)>) | r | L434 is_some(trunc i1) | 4 | OK |  |
| 63 | Entity | 0x98 | ty@Tower.info.nearest_enemy.Some.0.1 (=Tower+0x28) | r | L434 == champ.id → 타워가 나를 조준중 | 4 | OK |  |
| 64 | Entity | 0x308 | rush_state@tag | r | L484 (signed)>=0 → 암묵 RushPenetrate · ==-9223372036854775805 → Rush | 4 | OK |  |
| 65 | Entity | 0x628 | stat_cached.hp(max) | r | v27 L44/L65 분모 max(.,1) | 4 | OK |  |
| 66 | Entity | 0x4c0 | attack_effect@tag(i32) | r | objective_attack_range L16: ==-1(None) → unwrap panic | 4 | OK |  |
| 67 | Entity | 0x490 | attack_effect@Some.0 (Effect 선두) | r | objective_attack_range L17: &Effect 를 range_adjust(atk,champ,target) 에 전달 | 4 | OK |  |
| 68 | Entity | 0x4a0 | attack_effect@Some.0.range | r | effect.rs:26 Effect::range 인라인 | 4 | OK |  |
| 69 | Entity | 0x4a8 | attack_effect@Some.0.growth_range | r | ×(level-1) | 4 | OK |  |
| 70 | Entity | 0x470 | stat_buff_cached.radius_mult(i32) | r | entity.rs:1511 radius(): 0 이면 radius 그대로, 아니면 radius*(mult+100)/100 — champ·epic 양쪽 | 4 | OK |  |
| 71 | PlayerState | 0x180 | info.parameter(AthleteParameter) | r | L100/L124 positioning_effective(&param) · L457 positioning_accuracy(&param) | 4 | OK |  |
| 72 | OperationData | 0x0 | cache(&AbstractGameWithCache) | r | L443/L480 game 팻포인터 · L446 player_champion | 4 | OK |  |
| 73 | OperationData | 0x8 | context(&GameContext) | r | L447 debug 플래그 · L446 map.walls · bump(from_iter_in 인자 %350) | 4 | OK |  |
| 74 | OperationData | 0x10 | blackboard(&[Blackboard;2]) | r | L468 blackboard[1-team].is_recent_visible · closure$0 blackboard[team].is_recent_visible | 4 | OK |  |
| 75 | AbstractGameWithCache | 0x0 | game(&dyn AbstractGame: data@0, vtable@+8) | r | vtable+0x40 get_game_mode(L443) · +0x1f0 get_entity_by_id(L443) · +0x210 iter_projectile(L480) — divtable AbstractGame | 3 | OK |  |
| 76 | AbstractGameWithCache | 0x1e0 | player_champion[[Option<&Entity>;5];2] | r | [team] 슬라이스(%119 · allies/L444 champ 재조회 %120) · [1-team] 슬라이스(%381~%382 · threats/L463/L483) | 4 | 오귀속(사전은 다른 필드를 준다) |  |
| 77 | GameContext | 0x20 | map(&MapDef) | r | v27 L110 walls | 4 | OK |  |
| 78 | GameContext | 0x3b | debug(bool) | r | L447 true 면 L448 디버그 문자열 기록 | 4 | OK |  |
| 79 | MapDef | 0x78 | walls[30][30] usize | r | v27 L110 walls[yb][xb] != 0 → 후보 셀 제외 | 4 | OK |  |
| 80 | ScoreParameter | 0x9f0 | positioning_score(PositioningScoreData) | r | %372 — position_score_at_*·get_input 인자 | 4 | OK |  |
| 81 | ScoreParameter | 0x14a8 | positioning_score.cx | r | v27 L98 xi(후보 격자 중심 셀) | 4 | OK |  |
| 82 | ScoreParameter | 0x14b0 | positioning_score.cy | r | v27 L99 yi | 4 | OK |  |
| 83 | PositioningScore(sret 56B) | 0x0 | risk | r | v27_positioning_value L38 −risk×risk_weight/100 | 4 | OK |  |
| 84 | PositioningScore(sret 56B) | 0x10 | gain | r | v27_positioning_value L38 +gain×gain_weight/100 | 4 | OK |  |
| 85 | PositioningScore(sret 56B) | 0x30 | on_trajectory | r | v27 L37 궤도 페널티 · L496 on_trajectory | 4 | OK |  |
| 86 | PositioningScore(sret 56B) | 0x31 | on_periodic_trajectory | r | v27 L37 · L496 | 4 | OK |  |
| 87 | SmallActionPlay | 0xb1 | tag(니치) | r | L444 tag-15 <u 4 → 15 Attack/16 Skill/17 Skill2/18 Ult · L446 v27 반환 Option<SmallActionPlay>: 0xFF=None | 4 | OK |  |
| 88 | SmallActionPlay | 0x8 | Attack/Skill/Skill2/Ult.target | r | L444 == epic.id | 4 | OK |  |
| 89 | bumpalo Vec<SmallActionPlay>(act_actions %97) | 0x18 | len | r | L442/L456 is_empty · L444 iter | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 90 | bumpalo Vec<SmallActionPlay>(move_actions %93) | 0x18 | len | r | L476 iter(비면 unwrap panic) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 91 | bumpalo Vec<&Entity>(threats %22 / allies %19) | 0x18 | len | r | v27 L61/L87 is_empty · L70/L91/L120 iter | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 92 | Projectile | 0x0 | team@tag | r | L481 ==0(Player) 이면 +8 비교 | 4 | OK |  |
| 93 | Projectile | 0x8 | team@Player.0 | r | L481 == my team → 제외 | 4 | OK |  |
| 94 | Projectile | 0x40 | move_type@tag | r | projectile.rs:134 is_targeting 인라인: 6 Target·7 TargetSplash·(암묵 BouncingTarget 이고 +0x40==1) → targeting | 4 | OK |  |
| 95 | Projectile | 0x100 | x | r | L482 dist²(champ,p) | 4 | OK |  |
| 96 | Projectile | 0x108 | y | r | L482 | 4 | OK |  |
| 97 | Option<Input>(sret 32B, get_input) | 0x0 | tag(i64) | r | L490 -1=None · 0=Move · 그 외=다른 Input | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 98 | Option<Input>(sret 32B, get_input) | 0x8 | Move.x | r | L491 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 99 | Option<Input>(sret 32B, get_input) | 0x10 | Move.y | r | L491 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 100 | EpicHuntSubPlan | 0x0 | need_recall | w | L207 (need_recall==false 경로): (epic.focused==Some(champ.id) && dmg*3 >= champ.hp) \|\| champ.hp <= dmg 일 때. store 는 %186 블록에 합쳐짐(m15.ll:13136) \| (배치 J) L214 (need_recall==true 경로): champ.hp*100/champ.stat_cached.hp > 29 일 때(같은 store m15.ll:13136, phi 0) | 4 | OK | 1(true) |
| 101 | DebugFrameData | 0xa0 | infos[champ.id].push(String) | w | L552, context.debug==true && posture.kind ∈ {WaitGroup,SoftDisengage} 경로에서만(m15.ll:14212~14348). 힙 쓰기(HashMap entry + Vec<String> push) | 4 | 부분일치(첫 성분(컨테이너)만 맞고 **마지막 성분이 다르다** — 사람이 봐야 ) | format!("v25 morgard hunt posture: {:?}", posture.kind) |
| 102 | StdRng | 0x0 | rnd 상태 전진(콜리 경유) | w | 직접 write 없음. 순서는 signature.params[3] | 4 | 오귀속(사전은 다른 필드를 준다) | range_misjudge_roll ×3~4/적 · gen_range(12000..=inner) · AroundPosition::new · score · v27 |
| 103 | (sret) | 0x0 | Vec 헤더 32B | w | J 범위의 직접 sret 기록 2곳. 나머지는 K | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) | L224 [Recall] / L228 [v27 action] |
| 104 | DebugFrameData(debug, &mut) | 0xa0 | infos: HashMap<usize, Vec<String>> | w | L448 · 조건 context.debug(+0x3b)==true 이고 v27 이 Some 을 준 경우만(m15.ll 17086~17203) | 4 | OK | entry(champ.id).or_insert(Vec::new()).push(String::from("v27 morgard safe attack position")) |
| 105 | bumpalo Vec<SmallActionPlay>(act_actions %97, 지역) | 0x18 | len | w | L435 타워 어그로 시 clear (m15.ll 15741) | 4 | 확인불가(tcx 사전에 타입 없음) | 0 (truncate(0)) |
| 106 | bumpalo Vec<SmallActionPlay>(move_actions %93, 지역) | 0x18 | len | w | L436 clear · L437 push(RunAway(new_with_skill(data,player,5,true))) (m15.ll 15746~15760) | 4 | 확인불가(tcx 사전에 타입 없음) | 0 → 1 (truncate(0) 후 push) |
| 107 | (sret) bumpalo Vec<SmallActionPlay> | 0x0 | ptr/bump/cap/len | w | 반환점 L450/L473/L499/L501/L504/L507/L512 | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) | from_iter_in([1개], context.pool) 또는 act_actions 32B memcpy |
| 108 | EpicHuntSubPlan(self, &mut) | 0x0 | - | w | 배치 K 범위 store 0건 — sweep 부작용 비교 대상 아님 | 4 | 오귀속(사전은 다른 필드를 준다) | (쓰기 없음) |

**`consts` 상수 74건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev | folded_from |
|---|---|---|---|---|---|---|
| 0 | 2 | 202 | 태그 | player_champion 1차원(팀 2) bounds · Tower 태그(closure#5 target.ty != 2) · visible_state 첨자 bounds | 4 |  |
| 1 | 5 | 206 | 태그 | EntityType::Epic 메모리태그(L206) · 모든 생성자의 end_delay=5 · player_champion 2차원 5 | 4 |  |
| 2 | 3 | 207 | 태그 | dmg*3 >= champ.hp 위험 배수(`mul i64 %162, 3`, m15.ll:13122) · SmallActionPlay::RunAway 메모리태그(store i8 3). ⚠aux 의 `shl … 3` 은 jungles len×8 포인터 stride(접힘 상수 아님·이 항목과 무관) | 4 |  |
| 3 | 100 | 213 | 계수 | hp_ratio = hp*100/max_hp (L213·L329) · radius()*(100+mult)/100 | 4 |  |
| 4 | 29 | 214 | 임계 | hp_ratio > 29 (즉 ≥30%) 이면 need_recall 해제 | 4 |  |
| 5 | 4 | 227 | 태그 | JungleType::Morgard(4) — v27/v25/camp_pos 의 target · SmallActionPlay::Recall 메모리태그(L223 store i8 4) · ChampionActionState::Skill(4, L525) | 4 |  |
| 6 | -1 | 227 | 센티널 | 니치 None: Option<SmallActionPlay>(@0xb1) · Option<Effect>(casting i32) · Option<ObjectivePosture>(i64) · Option<BuffState>(@0x48 i32) | 4 |  |
| 7 | 6 | 232 | 길이 | iter_towers_without_nexus 의 고정 타워 배열 길이 6(top/top2/mid/mid2/bottom/bottom2) · ChampionActionState::Ult(6) | 4 |  |
| 8 | 13 | 253 | 태그 | EntityType::Champion 메모리태그 — closure#5 L253 is_champion · L523 | 4 |  |
| 9 | 15 | 254 | 미상 | closure#5/#7 switch: tag-15 < 4 ⇔ Attack(15)/Skill(16)/Skill2(17)/Ult(18) 만 target_id 보유 | 4 |  |
| 10 | 10 | 254 | 센티널 | llvm.assume(tag != 10) — SmallActionPlay 니치 아티팩트(판정 아님) · L615 enemy.remain_action_time() > 10 | 4 |  |
| 11 | 15000 | 272 | 계수 | closure#5 중첩클로저: 타워.attack_effect.is_in_range_ex(…, offset=15000) — 이동/돌진 스킬 대상이 타워 사거리+15000 안이면 제외 · L658 objective_wiggle_margin: dist+15000 < range | 4 |  |
| 12 | 14400000001 | 325 | 미상 | 120000^2+1 — closure#7: 대상(아군) 주변 120000(3.75셀) 이내 가시 적 챔프/타워/정글/others 존재 판정(dist_sq < 이 값 ⇔ ≤120000^2) | 4 |  |
| 13 | 79 | 330 | 미상 | closure#7: 힐 대상 hp_ratio > 79(즉 ≥80%) 이면 aoe_heal_covers_low_ally 필요 | 4 |  |
| 14 | -30 | 426 | 미상 | closure#8: score < -30 이면 액션 제거(≥ -30 유지) | 4 |  |
| 15 | 11 | 536 | 센티널 | PositionEvalPurpose::Objective 메모리태그(니치 idx9→11) | 4 |  |
| 16 | 14 | 559 | 태그 | SmallActionPlay::Trace 메모리태그(store i8 14) | 4 |  |
| 17 | 25000 | 603 | 미상 | can_enemy_hit_objective(enemy, epic, range_margin=25000) | 4 |  |
| 18 | 2000 | 596 | 계수 | max_v = 2000 - positioning_accuracy (오판 롤 상한, per-mille) | 4 |  |
| 19 | 1000 | 608 | 계수 | mr = max_range * roll / 1000 (per-mille 스케일) | 4 |  |
| 20 | 40 | 610 | 태그 | max_range_nearly_can_use(enemy, champ, 40) — emr_near 여유 | 4 |  |
| 21 | 60 | 627 | 미상 | max_range_nearly_can_use(enemy, champ, 60) — 1초 내 사망 시 재계산 emr 여유 | 4 |  |
| 22 | 999 | 615 | 임계 | `mr > 0` 이 `roll*max_range > 999`(나누기 1000 전 값) 로 접힘 — ≥1000 ⇔ mr ≥ 1 | 4 | 0 |
| 23 | 25600000000 | 598 | 미상 | 160000^2 — near_enemies: champ.distance_sq(e) < 160000^2(5셀 미만) | 4 |  |
| 24 | 12000 | 22 | 임계 | objective_wiggle_margin: inner 하한·gen_range 하한·폴백 12000 | 4 |  |
| 25 | 10000 | 22 | 계수 | objective_wiggle_margin: inner = range.saturating_sub(10000) | 4 |  |
| 26 | 45000 | 22 | 임계 | objective_wiggle_margin: inner 상한 45000 | 4 |  |
| 27 | 19 | 661 | 태그 | SmallActionPlay::Stop 메모리태그(store i8 19) | 4 |  |
| 28 | 1 | 232 | 인덱스 | 1 - team = 적 진영 인덱스 · ObjectFinishStrategy BattlePriority(1) | 4 |  |
| 29 | 2 | 433 | 태그 | EntityType::Tower 메모리태그(tcxdict --enum EntityType idx2=tag2). `icmp eq i64 %1133, 2` | 3 |  |
| 30 | 5 | 437 | 태그 | SmallActionRunAway::new_with_skill(data,player,end_delay=5,with_skill) 의 end_delay — L437(with_skill=true) · L473(false) · L499(false) | 4 |  |
| 31 | 3 | 437 | 태그 | SmallActionPlay::RunAway 메모리태그(store i8 3 @+0xb1) — L437/L473/L499 · 또 L144 new_with_radius end_delay=3. (본문의 `shl .., 3` 은 &Entity 슬라이스 stride ×8 접힘으로 이 상수와 무관) | 4 |  |
| 32 | -15 | 444 | 태그 | tag-15 <u 4 → 태그 15..18 = Attack/Skill/Skill2/Ult (공격 계열 후보 판별) | 4 |  |
| 33 | 4 | 444 | 태그 | 위 범위 폭(4 variant) | 4 |  |
| 34 | 100 | 446 | 계수 | hp% 계산·가중치 % 분모(v27 L44/L65/L38, radius() mult) | 4 |  |
| 35 | 36 | 446 | 임계 | v27 L45: 에픽 hp% < 36 이면 None(막타 구간엔 안전 위치 이동 안 함) | 4 |  |
| 36 | 67600000001 | 446 | 미상 | 260000²+1 — closure$0 L59: 적 챔프가 나로부터 260000(8.1셀) 이내면 threat | 4 |  |
| 37 | 48400000001 | 446 | 미상 | 220000²+1 — closure$0 L59 적이 에픽 220000 이내면 threat · closure$1 L85 아군이 에픽 220000 이내면 ally | 4 |  |
| 38 | 50 | 446 | 임계 | v27 L70: champ hp% < 50 이면 접촉 여유 70000 아니면 40000 · L71 max_range_nearly_can_use(t,champ,50) 인자 · v27_positioning_value 의 positioning 기준점 50 | 4 |  |
| 39 | 70000 | 446 | 산출값 | v27 L71 hp<50% 일 때 접촉 판정 여유 | 4 |  |
| 40 | 40000 | 446 | 산출값 | v27 L71 hp>=50% 일 때 접촉 판정 여유 | 4 |  |
| 41 | 100000 | 446 | 임계 | v27 L71 max(max_range_nearly_can_use(t,champ,50), 100000) — 위협 사거리 하한 | 4 |  |
| 42 | 45 | 446 | 임계 | v27 L74: 접촉 없어도 hp%<45 이고 최근접 위협 ≤220000 이면 계속 | 4 |  |
| 43 | 220001 | 446 | 임계 | v27 L74 nearest < 220001 (=≤220000, 직선거리) | 4 |  |
| 44 | 30000 | 446 | 계수 | v27 L78 keep_attack_range = attack_range.saturating_sub(30000) · L137 best.nearest < cur_nearest.saturating_add(30000) | 4 |  |
| 45 | 10 | 446 | 임계 | v27_positioning_value L35 gain_weight = 100+(50-positioning)/10 | 4 |  |
| 46 | 95 | 446 | 산출값 | gain_weight clamp 하한(-59 비교로 접힘) | 4 |  |
| 47 | 105 | 446 | 임계 | gain_weight clamp 상한 | 4 |  |
| 48 | -59 | 446 | 임계 | clamp 하한 접힘: (50-positioning) < -59 → 95 | 4 |  |
| 49 | 75 | 446 | 산출값 | risk_weight clamp 하한 (L36: 100+(positioning-50)/2) | 4 |  |
| 50 | 125 | 446 | 임계 | risk_weight clamp 상한 | 4 |  |
| 51 | -51 | 446 | 임계 | clamp 하한 접힘: (positioning-50) < -51 → 75 | 4 |  |
| 52 | -100 | 446 | 계수 | L38 risk×risk_weight / -100 (감점) | 4 |  |
| 53 | 320000 | 446 | 인덱스 | v27 L102/L126 적 간격 항 min(nearest,320000)/10000 (0..32) · L95 응집 항 (320000-dist(centroid)).sat/10000 | 4 |  |
| 54 | 10000 | 446 | 계수 | 점수 스케일(거리/10000) · L465 emr = mr×roll/1000 + 10000 | 4 |  |
| 55 | 8 | 446 | 미상 | ★현 위치 점수에만 더해지는 상수(L102 경로 %1512=nearest/10000+8). 후보 셀의 range_slack(최대 80000/10000=8)에 대응하는 '현 위치 = 여유 만점' 가정으로 읽힘. dbg 는 v27_positioning_value L38 에 귀속돼 있으나 후보 경로 L38 계산엔 8 이 없어 소속 줄은 미확정(unknown 참조) | 4 |  |
| 56 | 7 | 446 | 임계 | v27 L106/L107 후보 격자 dx,dy ∈ 0..7 (중심 cx,cy 기준 −3..+3) | 4 |  |
| 57 | -3 | 446 | 계수 | xb = cx-3+dx, yb = cy-3+dy | 4 |  |
| 58 | 29 | 446 | 임계 | v27 L110 격자 상한(30×30) xb>29 \|\| yb>29 제외 | 4 |  |
| 59 | 32000 | 446 | 미상 | 셀 크기(좌표 변환) L114/L115 | 4 |  |
| 60 | 16000 | 446 | 미상 | 셀 중심(좌표 변환) | 4 |  |
| 61 | -1 | 446 | 센티널 | L123 threats 비었을 때 nearest_enemy = u64::MAX(unwrap_or) · L446 Option<SmallActionPlay> None 니치(@0xb1 == 0xFF) · L490 Option<Input> None 태그 · objective_attack_range L16 attack_effect None(i32 -1) | 4 |  |
| 62 | 80000 | 446 | 인덱스 | v27 L127 range_slack = min(keep_attack_range.sat_sub(dist(cand,epic)), 80000)/10000 (0..8) | 4 |  |
| 63 | 144000001 | 446 | 임계 | 12000²+1 — v27 L140: best 가 현 위치에서 12000 이내면 None(제자리) | 4 |  |
| 64 | 25000 | 446 | 미상 | v27 L144 SmallActionAroundPosition::new_with_radius(rnd,data,x,y,end_delay=3,around_radius=25000) | 4 |  |
| 65 | 2000 | 459 | 계수 | max_v = 2000 - positioning_accuracy (min_v = positioning_accuracy) — range_misjudge_roll 범위 | 4 |  |
| 66 | 1000 | 465 | 계수 | emr = max_range_can_use(c,champ) × roll / 1000 + 10000 | 4 |  |
| 67 | 13 | 470 | 태그 | EntityType::Champion 태그 — is_in_action 인라인 | 4 |  |
| 68 | 176400000000 | 482 | 임계 | 420000² — 비조준 적 투사체가 나로부터 420000(13.1셀) 이내면 trajectory_possible | 4 |  |
| 69 | 9 | 481 | 센티널 | ProjectileMoveType 태그 assume(!=9) — 컴파일러 아티팩트(니치 범위) | 4 |  |
| 70 | 6 | 481 | 태그 | ProjectileMoveType::Target 메모리태그(switch 4 = 6-2) → is_targeting | 4 |  |
| 71 | -9223372036854775805 | 484 | 태그 | RushState::Rush 메모리태그(0x8000000000000003) — 적 챔프 돌진 중 | 4 |  |
| 72 | 11 | 494 | 미상 | PositionEvalPurpose 인자(i8 11) — L101/L125/L494 position_score_at_* 공통 | 4 |  |
| 73 | 1 | 446 | 태그 | umax(max_hp,1) 0 나눗셈 가드 · Option<(usize,usize)> Some 태그 · BouncingTarget target_id 태그 1 → targeting | 4 |  |

**`knobs` 조정점 32건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | need_recall 진입: 에픽 예상피해 배수 | epic_hunt.rs:207 | 3 | 올리면(dmg*N ≥ hp) 에픽이 나를 노릴 때 더 일찍 귀환 판단 | 4 | 기존 |
| 1 | need_recall 해제 HP% | epic_hunt.rs:214 | 29 | hp_ratio > 29 에서 해제. 올리면 더 회복해야 사냥 복귀 | 4 | 기존 |
| 2 | 이동/돌진 스킬 대상 타워 여유 | epic_hunt.rs:272 (closure#5 중첩) | 15000 | 올리면 타워 근처 챔피언에게 대시 스킬을 더 안 씀 | 4 | 기존 |
| 3 | 아군 힐/실드 정리: 근접 적 반경 | epic_hunt.rs:325~328 (closure#7) | 14400000001 | 120000²+1. 올리면 더 먼 적도 '근접'으로 봐 실드류를 더 유지 | 4 | 기존 |
| 4 | 힐 대상 HP% 상한 | epic_hunt.rs:330 | 79 | 대상 HP > 79% 이면 aoe_heal_covers_low_ally 통과 필요. 내리면 힐 남발 억제 | 4 | 기존 |
| 5 | 행동 점수 하한 | epic_hunt.rs:426 (closure#8) | -30 | score < -30 제거. 올리면 후보가 줄어 보수적 | 4 | 기존 |
| 6 | 킬우선 시 적 무시 여유 | epic_hunt.rs:603 | 25000 | can_enemy_hit_objective range_margin. 올리면 더 많은 적을 위협으로 인정 | 4 | 기존 |
| 7 | 오판 롤 상한 | epic_hunt.rs:596 | 2000 | max_v = 2000 - positioning_accuracy. per-mille | 4 | 기존 |
| 8 | emr_near 여유 | epic_hunt.rs:610 | 40 | max_range_nearly_can_use 인자. 올리면 도주(runaway) 판정이 넓어짐 | 4 | 기존 |
| 9 | 1초 내 사망 시 emr 여유 | epic_hunt.rs:627 | 60 | 올리면 사망 임박 시 도주 판정이 넓어짐 | 4 | 기존 |
| 10 | 적 행동 잔여시간 문턱 | epic_hunt.rs:615 | 10 | remain_action_time > 10 이면 '사거리 밖이면 추격' 분기 | 4 | 기존 |
| 11 | 근접 적 반경 | epic_hunt.rs:598 (get_move_action closure#1) | 25600000000 | 160000². 올리면 더 먼 적까지 near_enemies 에 포함 | 4 | 기존 |
| 12 | 에픽 주변 흔들림 여유 | epic_hunt.rs:22~29 (objective_wiggle_margin) | 12000 | inner 하한/폴백. 10000(range 차감)·45000(상한)·15000(dist+15000<range 조건)과 함께 | 4 | 기존 |
| 13 | 사망 임박 기준 | epic_hunt.rs:614 | tick_per_second(설정) | me_die_tick < tps(1초). 설정값이라 상수 아님 | 4 | 기존 |
| 14 | v27 발동 에픽 hp% 하한 | epic_hunt.rs:45 (m15.ll 15964) | 36 | 올리면 에픽 체력이 더 높을 때부터 안전위치 이동을 포기(막타 구간 확대) · 내리면 저체력에도 위치 조정 | 4 | 기존 |
| 15 | threat 판정 반경(나 기준) | epic_hunt.rs:59 closure$0 (m15.ll 58021) | 67600000001 | 260000² — 올리면 더 먼 적도 위협으로 잡아 v27 이 더 자주 발동 | 4 | 기존 |
| 16 | threat/ally 판정 반경(에픽 기준) | epic_hunt.rs:59/85 (m15.ll 58053/58112) | 48400000001 | 220000² — 에픽 주변 적/아군 인식 범위 | 4 | 기존 |
| 17 | 접촉 여유(저체력/고체력) | epic_hunt.rs:70~71 (m15.ll 16273~16274) | 70000 / 40000 | 올리면 적이 더 멀어도 '접촉'으로 봐 위치 조정 발동 | 4 | 기존 |
| 18 | 위협 사거리 하한 | epic_hunt.rs:71 (m15.ll 16302) | 100000 | max_range_nearly_can_use 가 작아도 최소 100000 으로 봄 | 4 | 기존 |
| 19 | 비접촉 발동 hp%·거리 | epic_hunt.rs:74 (m15.ll 16339~16340) | 45 / 220001 | 접촉 없어도 hp<45% 이고 220000 이내 위협이면 발동 | 4 | 기존 |
| 20 | keep_attack_range 여유 | epic_hunt.rs:78 (m15.ll 16345) | 30000 | 올리면 후보 셀이 에픽에 더 가까운 곳으로 제한 | 4 | 기존 |
| 21 | 포지셔닝 가중치 clamp | epic_hunt.rs:35~36 (m15.ll 16528~16540) | gain 95..105 / risk 75..125 | 포지셔닝 스탯이 점수에 미치는 폭 | 4 | 기존 |
| 22 | 적 간격 상한 | epic_hunt.rs:102/126 (m15.ll 16563/16899) | 320000 | min(nearest,320000)/10000 — 적과 32셀 이상 떨어져도 추가 점수 없음 | 4 | 기존 |
| 23 | range_slack 상한 | epic_hunt.rs:127 (m15.ll 16914) | 80000 | 에픽 사거리 여유 점수 최대 8 | 4 | 기존 |
| 24 | 현 위치 고정 가산 | epic_hunt.rs:102(추정) (m15.ll 16681) | 8 | 내리면 현 위치 점수가 낮아져 이동 후보가 더 자주 채택 | 5 | 기존 |
| 25 | best 채택 간격 조건 | epic_hunt.rs:137 (m15.ll 16688) | 30000 | best 점수가 현 위치보다 낮아도 적과 30000 이상 더 벌어지면 채택 | 4 | 기존 |
| 26 | 제자리 판정 거리 | epic_hunt.rs:140 (m15.ll 16707) | 144000001 | 12000² — 올리면 짧은 이동은 버림 | 4 | 기존 |
| 27 | AroundPosition 파라미터 | epic_hunt.rs:144 (m15.ll 16712) | end_delay 3 / around_radius 25000 | 이동 행동의 종료 지연·배회 반경 | 4 | 기존 |
| 28 | RunAway end_delay | epic_hunt.rs:437/473/499 | 5 | 도주 행동 유지 틱 여유 | 4 | 기존 |
| 29 | range_misjudge 범위 | epic_hunt.rs:458~459 (m15.ll 17271) | min=positioning_accuracy, max=2000-positioning_accuracy | 적 사거리 오판 폭 — 2000 을 올리면 저능력 선수의 오판이 커짐 | 4 | 기존 |
| 30 | emr 보정 | epic_hunt.rs:465 (m15.ll 17369~17370) | /1000 +10000 | 적 사거리 인식에 +10000 여유 | 4 | 기존 |
| 31 | 투사체 위험 반경 | epic_hunt.rs:482 (m15.ll 17686) | 176400000000 | 420000² — 비조준 투사체가 이 안에 있으면 목적지 궤도 검사 후 도주 대체 | 4 | 기존 |

<details><summary>`callees` 피호출자 113건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | aoe_heal_covers_low_ally | game_ai::aoe_heal_covers_low_ally | pub | fn(usize, &game_core::Effect, &game_core::OperationData, &game_core::PlayerState, &game_core::Entity) -> bool | game-ai\src\buff_value.rs:543 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | attack_effect | game_core::Entity::attack_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1684 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 2 | attack_jungle_action | game_ai::plan_legacy::sub_plan::EpicHuntSubPlan::attack_jungle_action | in:game_ai::plan_legacy::sub_plan::epic_hunt | fn(&mut game_ai::plan_legacy::sub_plan::EpicHuntSubPlan, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\epic_hunt.rs:147 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 3 | attack_summon_action | game_ai::attack_summon_action | pub | fn(&game_core::PlayerState, &game_core::OperationData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\fight_check.rs:836 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | battle_action | game_ai::battle_action | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, usize) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\fight_check.rs:620 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 5 | block_input | game_core::Entity::block_input | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1501 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 6 | camp_pos | game_core::MapDef::camp_pos | pub | fn(&game_core::MapDef, game_core::JungleType, bool) -> (u64, u64) | game-core\src\simulation\map_def.rs:207 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 7 | can_enemy_hit_objective | game_ai::plan_legacy::old::can_enemy_hit_objective | pub | fn(&game_core::Entity, &game_core::Entity, u64) -> bool | game-ai\src\plan_legacy\old\fight_model.rs:1188 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 8 | can_target | game_core::Entity::can_target | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1477 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 9 | can_target | game_core::OperationData::<'a, 'b>::can_target | pub | fn(&game_core::OperationData<'a/#0, 'b/#1>, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\state\player.rs:23 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 10 | check_kill_die_tick | game_ai::check_kill_die_tick | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::OperationData, &game_core::PlayerState, &game_core::Entity, bumpalo::collections::vec::Vec< &game_core::Entity>, bumpalo::collections::vec::Vec< &game_core::Entity>, &mut game_core::DebugFrameData) -> usize | game-ai\src\fight_check.rs:917 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 11 | clear | game_core::DataTable::<T>::clear | pub | fn(&mut game_core::DataTable<T/#0>) | game-core\src\data.rs:2259 | True | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 12 | debug | game_core::AbstractBanpickRunner::debug | pub | fn(&Self/#0, &[usize]) | game-core\src\banpick\base_runner.rs:411 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 13 | debug | <game_core::FactoredBanpickAgent as game_core::AbstractBanpickRunner>::debug | pub | fn(&game_core::FactoredBanpickAgent, &[usize]) | game-core\src\banpick\sgd_v2.rs:5891 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 14 | debug | <game_core::LogisticSGDBanpickAgent as game_core::AbstractBanpickRunner>::debug | pub | fn(&game_core::LogisticSGDBanpickAgent, &[usize]) | game-core\src\banpick\sgd.rs:1172 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 15 | distance | game_core::utils::distance | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:12 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 16 | distance | game_core::Entity::distance | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2161 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 17 | distance_sq | game_core::utils::distance_sq | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:6 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 18 | distance_sq | game_core::Entity::distance_sq | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2157 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 19 | epic_action_score | game_ai::plan_legacy::sub_plan::epic_action_score | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 | game-ai\src\plan_legacy\sub_plan\epic_hunt.rs:848 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 20 | expected_buff | game_core::EffectType::expected_buff | pub | fn(&Self/#0, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + ) -> std::option::Option<game_core::BuffState> | game-core\src\simulation\effect\type.rs:287 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 18개 중 상위 3개 |
| 21 | expected_buff | <game_core::RangeEffect as game_core::EffectType>::expected_buff | pub | fn(&game_core::RangeEffect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + ) -> std::option::Option<game_core::BuffState> | game-core\src\simulation\effect\type\range_effect.rs:94 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 18개 중 상위 3개 |
| 22 | expected_buff | <game_core::AddBuffEffect as game_core::EffectType>::expected_buff | pub | fn(&game_core::AddBuffEffect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + ) -> std::option::Option<game_core::BuffState> | game-core\src\simulation\effect\type\add_buff.rs:28 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 18개 중 상위 3개 |
| 23 | expected_damage_target | game_core::Effect::expected_damage_target | pub | fn(&game_core::Effect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + , &game_core::Entity) -> usize | game-core\src\simulation\effect.rs:91 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 24 | expected_heal | game_ai::expected_heal | pub | fn(&game_core::GameContext, &game_core::Entity, &game_core::Entity) -> usize | game-ai\src\fight_check.rs:266 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 31개 중 상위 3개 |
| 25 | expected_heal | game_core::EffectType::expected_heal | pub | fn(&Self/#0, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + ) -> usize | game-core\src\simulation\effect\type.rs:283 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 31개 중 상위 3개 |
| 26 | expected_heal | <game_core::HealEffect as game_core::EffectType>::expected_heal | pub | fn(&game_core::HealEffect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + ) -> usize | game-core\src\simulation\effect\type\heal.rs:186 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 31개 중 상위 3개 |
| 27 | expected_move_on_hit | game_core::EffectType::expected_move_on_hit | pub | fn(&Self/#0) -> bool | game-core\src\simulation\effect\type.rs:293 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 20개 중 상위 3개 |
| 28 | expected_move_on_hit | <game_core::CombineEffect as game_core::EffectType>::expected_move_on_hit | pub | fn(&game_core::CombineEffect) -> bool | game-core\src\simulation\effect\type\combine.rs:96 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 20개 중 상위 3개 |
| 29 | expected_move_on_hit | <game_core::DelayedEffect as game_core::EffectType>::expected_move_on_hit | pub | fn(&game_core::DelayedEffect) -> bool | game-core\src\simulation\effect\type\delayed.rs:45 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 20개 중 상위 3개 |
| 30 | expected_rush_effect | game_core::EffectType::expected_rush_effect | pub | fn(&Self/#0) -> bool | game-core\src\simulation\effect\type.rs:291 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 9개 중 상위 3개 |
| 31 | expected_rush_effect | <game_core::RushEffect as game_core::EffectType>::expected_rush_effect | pub | fn(&game_core::RushEffect) -> bool | game-core\src\simulation\effect\type\rush.rs:53 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 9개 중 상위 3개 |
| 32 | expected_rush_effect | <game_core::CombineEffect as game_core::EffectType>::expected_rush_effect | pub | fn(&game_core::CombineEffect) -> bool | game-core\src\simulation\effect\type\combine.rs:42 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 9개 중 상위 3개 |
| 33 | expected_shield | game_ai::expected_shield | pub | fn(&game_core::GameContext, &game_core::Entity, &game_core::Entity) -> usize | game-ai\src\fight_check.rs:284 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 22개 중 상위 3개 |
| 34 | expected_shield | game_core::EffectType::expected_shield | pub | fn(&Self/#0, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + ) -> usize | game-core\src\simulation\effect\type.rs:285 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 22개 중 상위 3개 |
| 35 | expected_shield | <game_core::RangeEffect as game_core::EffectType>::expected_shield | pub | fn(&game_core::RangeEffect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + ) -> usize | game-core\src\simulation\effect\type\range_effect.rs:90 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 22개 중 상위 3개 |
| 36 | get_act_action | game_ai::plan_legacy::sub_plan::EpicHuntSubPlan::get_act_action | in:game_ai::plan_legacy::sub_plan::epic_hunt | fn(&mut game_ai::plan_legacy::sub_plan::EpicHuntSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\epic_hunt.rs:670 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 37 | get_act_action | game_ai::plan_legacy::sub_plan::SerpenHuntSubPlan::get_act_action | in:game_ai::plan_legacy::sub_plan::serpen_hunt | fn(&mut game_ai::plan_legacy::sub_plan::SerpenHuntSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\serpen_hunt.rs:668 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 38 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 39 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 40 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 41 | get_game_mode | game_core::AbstractGame::get_game_mode | pub | fn(&Self/#0) -> game_core::GameMode | game-core\src\simulation.rs:78 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 42 | get_game_mode | <game_core::Game as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::Game) -> game_core::GameMode | game-core\src\simulation\game.rs:1707 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 43 | get_game_mode | <game_core::SingleLaneGame as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::SingleLaneGame) -> game_core::GameMode | game-core\src\simulation\game.rs:4248 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 44 | get_input | game_ai::SmallActionPlay::get_input | pub | fn(&mut game_ai::SmallActionPlay, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &mut game_core::DebugFrameData) -> std::option::Option<game_core::Input> | game-ai\src\small_action.rs:157 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 45 | get_move_action | game_ai::plan_legacy::sub_plan::EpicHuntSubPlan::get_move_action | in:game_ai::plan_legacy::sub_plan::epic_hunt | fn(&mut game_ai::plan_legacy::sub_plan::EpicHuntSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\epic_hunt.rs:517 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 4개 중 상위 3개 |
| 46 | get_move_action | game_ai::plan_legacy::sub_plan::EpicPokeSubPlan::get_move_action | in:game_ai::plan_legacy::sub_plan::epic_poke | fn(&mut game_ai::plan_legacy::sub_plan::EpicPokeSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\epic_poke.rs:278 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 4개 중 상위 3개 |
| 47 | get_move_action | game_ai::plan_legacy::sub_plan::SerpenHuntSubPlan::get_move_action | in:game_ai::plan_legacy::sub_plan::serpen_hunt | fn(&mut game_ai::plan_legacy::sub_plan::SerpenHuntSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\serpen_hunt.rs:516 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 4개 중 상위 3개 |
| 48 | is_champion | game_core::EntityType::is_champion | pub | fn(&game_core::EntityType) -> bool | game-core\src\simulation\entity.rs:1403 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 49 | is_empty | game_core::PatchSetting::is_empty | pub | fn(&game_core::PatchSetting) -> bool | game-core\src\patch_engine.rs:853 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 50 | is_empty | game_core::ModAiRegistry::is_empty | pub | fn(&game_core::ModAiRegistry) -> bool | game-core\src\mod_ai.rs:15 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 51 | is_empty | game_core::PublicChampionEvidence::is_empty | pub | fn(&game_core::PublicChampionEvidence) -> bool | game-core\src\banpick\public_evidence.rs:96 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 52 | is_in_action | game_core::Entity::is_in_action | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1547 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 53 | is_in_range | game_core::Effect::is_in_range | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\effect.rs:63 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 54 | is_in_range_ex | game_core::Effect::is_in_range_ex | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity, u64, u64, u64, u64, u64) -> bool | game-core\src\simulation\effect.rs:78 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 55 | is_recent_visible | game_core::Blackboard::is_recent_visible | pub | fn(&game_core::Blackboard, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\game\blackboard.rs:346 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 56 | is_targeting | game_core::ProjectileMoveType::is_targeting | pub | fn(&game_core::ProjectileMoveType) -> bool | game-core\src\simulation\projectile.rs:133 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 57 | iter_champions | game_core::AbstractGameWithCache::<'a, 'b>::iter_champions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5759 ~ game_core[6a30]::simulation::{impl#5}::iter_champions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1904 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 58 | iter_projectile | game_core::AbstractGame::iter_projectile | pub | fn(&Self/#0) -> game_core::ProjectileIter | game-core\src\simulation.rs:182 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 59 | iter_projectile | <game_core::Game as game_core::AbstractGame>::iter_projectile | pub | fn(&game_core::Game) -> game_core::ProjectileIter | game-core\src\simulation\game.rs:3760 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 60 | iter_projectile | <game_core::SingleLaneGame as game_core::AbstractGame>::iter_projectile | pub | fn(&game_core::SingleLaneGame) -> game_core::ProjectileIter | game-core\src\simulation\game.rs:4019 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 61 | iter_towers | game_core::AbstractGameWithCache::<'a, 'b>::iter_towers | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5762 ~ game_core[6a30]::simulation::{impl#5}::iter_towers::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1908 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 62 | iter_towers_without_nexus | game_core::AbstractGameWithCache::<'a, 'b>::iter_towers_without_nexus | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5750 ~ game_core[6a30]::simulation::{impl#5}::iter_towers_without_nexus::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1830 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 63 | kind | game_view::ui::database_edit_ui::DbEditAppearanceTarget::kind | in:game_view::ui::database_edit_ui | fn(game_view::ui::database_edit_ui::DbEditAppearanceTarget) -> game_view::ui::athlete_appearance_popup::AppearanceTargetKind | game-view\src\ui\database_edit_ui.rs:100 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 64 | level | <game_ai::WindowStatView<'_> as game_core::AbstractEntity>::level | pub | fn(&game_ai::WindowStatView</#0>) -> usize | game-ai\src\fight_check.rs:96 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 65 | level | game_core::AbstractEntity::level | pub | fn(&Self/#0) -> usize | game-core\src\simulation\entity.rs:718 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 66 | level | <game_core::Entity as game_core::AbstractEntity>::level | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:772 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 67 | max_range_can_use | game_ai::plan_legacy::old::max_range_can_use | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-ai\src\plan_legacy\old\battle.rs:2431 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 68 | max_range_nearly_can_use | game_ai::plan_legacy::old::max_range_nearly_can_use | pub | fn(&game_core::Entity, &game_core::Entity, usize) -> u64 | game-ai\src\plan_legacy\old\battle.rs:2397 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 69 | new | game_ai::SmallActionTrace::new | pub | fn(&game_core::OperationData, usize, usize) -> game_ai::SmallActionTrace | game-ai\src\small_action\trace.rs:42 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 70 | new | game_ai::SmallActionRecall::new | pub | fn(&game_core::OperationData, &game_core::PlayerState, usize) -> game_ai::SmallActionRecall | game-ai\src\small_action\move_actions.rs:659 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 71 | new | game_ai::SmallActionAroundPosition::new | pub | fn(&mut rand::rngs::std::StdRng, &game_core::OperationData, u64, u64, usize) -> game_ai::SmallActionAroundPosition | game-ai\src\small_action\around.rs:831 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 72 | new_attack_range | game_ai::SmallActionTrace::new_attack_range | pub | fn(&game_core::OperationData, usize, usize) -> game_ai::SmallActionTrace | game-ai\src\small_action\trace.rs:75 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 73 | new_attack_range_margin | game_ai::SmallActionTrace::new_attack_range_margin | pub | fn(&game_core::OperationData, usize, usize, u64) -> game_ai::SmallActionTrace | game-ai\src\small_action\trace.rs:79 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 74 | new_with_radius | game_ai::SmallActionAroundPosition::new_with_radius | pub | fn(&mut rand::rngs::std::StdRng, &game_core::OperationData, u64, u64, usize, u64) -> game_ai::SmallActionAroundPosition | game-ai\src\small_action\around.rs:837 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 75 | new_with_skill | game_ai::SmallActionRunAway::new_with_skill | pub | fn(&game_core::OperationData, &game_core::PlayerState, usize, bool) -> game_ai::SmallActionRunAway | game-ai\src\small_action\move_actions.rs:29 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 76 | nontarget_windup_perceived | game_ai::nontarget_windup_perceived | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity) -> bool | game-ai\src\utils.rs:553 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 77 | objective_attack_range | game_ai::plan_legacy::sub_plan::EpicHuntSubPlan::objective_attack_range | in:game_ai::plan_legacy::sub_plan::epic_hunt | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-ai\src\plan_legacy\sub_plan\epic_hunt.rs:15 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 78 | objective_attack_range | game_ai::plan_legacy::sub_plan::SerpenHuntSubPlan::objective_attack_range | in:game_ai::plan_legacy::sub_plan::serpen_hunt | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-ai\src\plan_legacy\sub_plan\serpen_hunt.rs:15 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 79 | objective_wiggle_margin | game_ai::plan_legacy::sub_plan::EpicHuntSubPlan::objective_wiggle_margin | in:game_ai::plan_legacy::sub_plan::epic_hunt | fn(&mut rand::rngs::std::StdRng, &game_core::Entity, &game_core::Entity, u64) -> u64 | game-ai\src\plan_legacy\sub_plan\epic_hunt.rs:20 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 80 | objective_wiggle_margin | game_ai::plan_legacy::sub_plan::SerpenHuntSubPlan::objective_wiggle_margin | in:game_ai::plan_legacy::sub_plan::serpen_hunt | fn(&mut rand::rngs::std::StdRng, &game_core::Entity, &game_core::Entity, u64) -> u64 | game-ai\src\plan_legacy\sub_plan\serpen_hunt.rs:20 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 81 | position | game_core::PlayerAiContext::<'a, 'b, 'r>::position | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> game_core::Position | game-core\src\mod_ai.rs:587 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 82 | position | game_core::transfer::TeamRosterPlanningContext::position | pub | fn(&game_core::transfer::TeamRosterPlanningContext, game_core::Position) -> std::option::Option<&game_core::transfer::PositionPlanningContext> | game-core\src\transfer\roster_context.rs:234 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 83 | position_score_at_cell | game_ai::position_score_at_cell | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, i64, i64, game_ai::PositionEvalPurpose) -> game_core::PositioningScore | game-ai\src\position_eval.rs:1183 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 84 | position_score_at_position | game_ai::position_score_at_position | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, u64, u64, game_ai::PositionEvalPurpose) -> game_core::PositioningScore | game-ai\src\position_eval.rs:1192 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 85 | positioning_accuracy | game_core::AthleteParameter::positioning_accuracy | pub | fn(&game_core::AthleteParameter) -> usize | game-core\src\simulation\state\player.rs:285 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 86 | positioning_effective | game_core::AthleteParameter::positioning_effective | pub | fn(&game_core::AthleteParameter) -> usize | game-core\src\simulation\state\player.rs:291 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 87 | radius | game_core::Entity::radius | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1509 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 88 | range_adjust | game_core::Effect::range_adjust | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:29 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 89 | range_misjudge_rng | game_ai::range_misjudge_rng | pub | fn(usize, &game_core::OperationData, &game_core::PlayerState, usize) -> std::option::Option<game_core::NoiseRng> | game-ai\src\utils.rs:502 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 90 | range_misjudge_roll | game_ai::range_misjudge_roll | pub | fn(&mut rand::rngs::std::StdRng, &mut std::option::Option<game_core::NoiseRng>, u64, u64) -> u64 | game-ai\src\utils.rs:516 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 91 | remain_action_time | game_core::Entity::remain_action_time | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1616 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 92 | score | game_ai::plan_legacy::sub_plan::EpicHuntSubPlan::score | pub | fn(&game_ai::plan_legacy::sub_plan::EpicHuntSubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 | game-ai\src\plan_legacy\sub_plan\epic_hunt.rs:840 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 93 | should_ignore_object_finish_kill_priority_target | game_ai::plan_legacy::old::should_ignore_object_finish_kill_priority_target | pub | fn(&game_core::PlayerState, &game_core::Entity, std::option::Option<&game_core::Entity>) -> bool | game-ai\src\plan_legacy\old\fight_model.rs:1208 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 94 | skill2_effect | game_core::Entity::skill2_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1692 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 95 | strategy | game_core::PlayerState::strategy | pub | fn(&game_core::PlayerState, &mut rand::rngs::std::StdRng, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + ) -> game_core::Strategy | game-core\src\simulation\state\player.rs:1576 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 96 | target_id | game_ai::SmallActionTrace::target_id | pub | fn(&game_ai::SmallActionTrace) -> usize | game-ai\src\small_action\trace.rs:427 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 97 | target_id | game_view::view::effect::alchemist::target_id | in:game_view::view::effect::alchemist | fn(game_core::InputTarget) -> std::option::Option<usize> | game-view\src\view\effect\alchemist.rs:126 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 98 | target_id | game_view::view::projectile::crossbowman::target_id | in:game_view::view::projectile::crossbowman | fn(game_core::InputTarget) -> std::option::Option<usize> | game-view\src\view\projectile\crossbowman.rs:1878 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 99 | team | game_view::ClientData::team | pub | fn(&game_view::ClientData, usize) -> std::option::Option<std::cell::Ref< game_core::Team>> | game-view\src\logic\client\data.rs:1651 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 100 | team | game_view::ClientDatabase::team | pub | fn(&game_view::ClientDatabase, usize) -> std::option::Option<&game_core::Team> | game-view\src\logic\client\data.rs:968 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 101 | team | game_core::PlayerAiContext::<'a, 'b, 'r>::team | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:583 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 102 | tick_per_second | game_view::view::effect::nightmare::tick_per_second | in:game_view::view::effect::nightmare | fn(&engine_core::assets::Assets) -> f32 | game-view\src\view\effect\nightmare.rs:31 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 103 | to_string | game_core::Position::to_string | pub | fn(&game_core::Position) -> std::string::String | game-core\src\simulation\entity.rs:665 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 4개 중 상위 3개 |
| 104 | to_string | game_core::LineType::to_string | pub | fn(&game_core::LineType) -> std::string::String | game-core\src\simulation\state\player.rs:997 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 4개 중 상위 3개 |
| 105 | to_string | game_core::JungleType::to_string | pub | fn(&game_core::JungleType) -> std::string::String | game-core\src\simulation\entity\jungle.rs:394 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 4개 중 상위 3개 |
| 106 | ult_effect | game_core::Entity::ult_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1700 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 107 | v23_should_break_objective_hunt_anchor | game_ai::plan_legacy::team_plan::v23_should_break_objective_hunt_anchor | pub | fn(&game_core::PlayerState, &game_core::OperationData, &game_core::Entity, &game_core::Entity, (u64, u64)) -> bool | game-ai\src\plan_legacy\team_plan\objective_helpers.rs:59 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 108 | v25_objective_posture | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v25_objective_posture | pub | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::PlayerState, &game_core::OperationData, game_core::JungleType) -> std::option::Option<game_ai::plan_legacy::team_plan::ObjectivePosture> | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:207 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 109 | v27_objective_discipline_action | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v27_objective_discipline_action | pub | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, game_core::JungleType) -> std::option::Option<game_ai::SmallActionPlay> | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:505 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 110 | v27_objective_safe_attack_position | game_ai::plan_legacy::sub_plan::EpicHuntSubPlan::v27_objective_safe_attack_position | in:game_ai::plan_legacy::sub_plan::epic_hunt | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_core::Entity) -> std::option::Option<game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\epic_hunt.rs:41 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 111 | v27_positioning_value | game_ai::plan_legacy::sub_plan::EpicHuntSubPlan::v27_positioning_value | in:game_ai::plan_legacy::sub_plan::epic_hunt | fn(usize, &game_core::PlayerState, game_core::PositioningScore) -> i64 | game-ai\src\plan_legacy\sub_plan\epic_hunt.rs:33 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 112 | v27_positioning_value | game_ai::plan_legacy::sub_plan::SerpenHuntSubPlan::v27_positioning_value | in:game_ai::plan_legacy::sub_plan::serpen_hunt | fn(usize, &game_core::PlayerState, game_core::PositioningScore) -> i64 | game-ai\src\plan_legacy\sub_plan\serpen_hunt.rs:33 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
</details>

⚠**미매칭 48개**: `block_target_tick`, `casting`, `clamp`, `cleanup`, `cohesion`, `drop  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `entry`, `extend`, `find  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `first`, `focus_enemy`, `focused`, `format_inner`, `gain`, `gen_range`, `grow_one`, `handle_error`, `infos`, `insert_no_grow`, `is_none_or`, `map_or`, `move_actions`, `move_type`, `near_enemy_count`, `next  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 9개는 **전부 다른 함수**라 싣지 않는다`, `object_finish`, `on_periodic_trajectory`, `on_trajectory`, `or_default`, `or_insert`, `others`, `parameter`, `positioning_score`, `push  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 6개는 **전부 다른 함수**라 싣지 않는다`, `push_mut`, `range  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `reserve_internal_or_panic`, `retain`, `risk`, `rush_state`, `rustc_entry`, `sret`, `target`, `trajectory_penalty`, `truncate`, `try_allocate_in`, `try_fold`, `variant`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m12.ll:35237) · **형제 15개** (EpicHuntSubPlan)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::plan_legacy::sub_plan::EpicHuntSubPlan as std::clone::Clone>::clone | pub | game-ai\src\plan_legacy\sub_plan\epic_hunt.rs:9 | True | fn(&game_ai::plan_legacy::sub_plan::EpicHuntSubPlan) -> game_ai::plan_legacy::sub_plan::EpicHuntSubPlan |
| 1 | <game_ai::plan_legacy::sub_plan::EpicHuntSubPlan as std::fmt::Debug>::fmt | pub | game-ai\src\plan_legacy\sub_plan\epic_hunt.rs:9 | True | fn(&game_ai::plan_legacy::sub_plan::EpicHuntSubPlan, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 2 | <game_ai::plan_legacy::sub_plan::EpicHuntSubPlan as std::default::Default>::default | pub | game-ai\src\plan_legacy\sub_plan\epic_hunt.rs:9 | True | fn() -> game_ai::plan_legacy::sub_plan::EpicHuntSubPlan |
| 3 | game_ai::plan_legacy::sub_plan::EpicHuntSubPlan::objective_attack_range | in:game_ai::plan_legacy::sub_plan::epic_hunt | game-ai\src\plan_legacy\sub_plan\epic_hunt.rs:15 | False | fn(&game_core::Entity, &game_core::Entity) -> u64 |
| 4 | game_ai::plan_legacy::sub_plan::EpicHuntSubPlan::objective_wiggle_margin | in:game_ai::plan_legacy::sub_plan::epic_hunt | game-ai\src\plan_legacy\sub_plan\epic_hunt.rs:20 | False | fn(&mut rand::rngs::std::StdRng, &game_core::Entity, &game_core::Entity, u64) -> u64 |
| 5 | game_ai::plan_legacy::sub_plan::EpicHuntSubPlan::v27_positioning_value | in:game_ai::plan_legacy::sub_plan::epic_hunt | game-ai\src\plan_legacy\sub_plan\epic_hunt.rs:33 | False | fn(usize, &game_core::PlayerState, game_core::PositioningScore) -> i64 |
| 6 | game_ai::plan_legacy::sub_plan::EpicHuntSubPlan::v27_objective_safe_attack_position | in:game_ai::plan_legacy::sub_plan::epic_hunt | game-ai\src\plan_legacy\sub_plan\epic_hunt.rs:41 | False | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_core::Entity) -> std::option::Option<game_ai::SmallActionPlay> |
| 7 | game_ai::plan_legacy::sub_plan::EpicHuntSubPlan::attack_jungle_action | in:game_ai::plan_legacy::sub_plan::epic_hunt | game-ai\src\plan_legacy\sub_plan\epic_hunt.rs:147 | False | fn(&mut game_ai::plan_legacy::sub_plan::EpicHuntSubPlan, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 8 | game_ai::plan_legacy::sub_plan::EpicHuntSubPlan::action_candidates | pub | game-ai\src\plan_legacy\sub_plan\epic_hunt.rs:200 | False | fn(&mut game_ai::plan_legacy::sub_plan::EpicHuntSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 9 | game_ai::plan_legacy::sub_plan::EpicHuntSubPlan::get_move_action | in:game_ai::plan_legacy::sub_plan::epic_hunt | game-ai\src\plan_legacy\sub_plan\epic_hunt.rs:517 | False | fn(&mut game_ai::plan_legacy::sub_plan::EpicHuntSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 10 | game_ai::plan_legacy::sub_plan::EpicHuntSubPlan::get_act_action | in:game_ai::plan_legacy::sub_plan::epic_hunt | game-ai\src\plan_legacy\sub_plan\epic_hunt.rs:670 | False | fn(&mut game_ai::plan_legacy::sub_plan::EpicHuntSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 11 | game_ai::plan_legacy::sub_plan::EpicHuntSubPlan::action_candidates_old | pub | game-ai\src\plan_legacy\sub_plan\epic_hunt.rs:678 | False | fn(&mut game_ai::plan_legacy::sub_plan::EpicHuntSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 12 | game_ai::plan_legacy::sub_plan::EpicHuntSubPlan::calculate_score_parameter_value | pub | game-ai\src\plan_legacy\sub_plan\epic_hunt.rs:823 | True | fn(&game_ai::plan_legacy::sub_plan::EpicHuntSubPlan, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::ScoreParameter) |
| 13 | game_ai::plan_legacy::sub_plan::EpicHuntSubPlan::score | pub | game-ai\src\plan_legacy\sub_plan\epic_hunt.rs:840 | False | fn(&game_ai::plan_legacy::sub_plan::EpicHuntSubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 |
| 14 | game_ai::plan_legacy::sub_plan::EpicHuntSubPlan::merge | pub | game-ai\src\plan_legacy\sub_plan\epic_hunt.rs:844 | True | fn(&mut game_ai::plan_legacy::sub_plan::EpicHuntSubPlan, game_ai::plan_legacy::sub_plan::EpicHuntSubPlan) |

**`open` 15건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 표기 불가 | (배치 J) L244/L313 의 `.filter(closure#5)`/`.map(closure#6)` 호출 자체가 소스 313줄에, from_iter_in 이 244줄에 찍힌다 — 클로저#5 본문은 246~301(칸 정보 0 이라 `Vec::from_iter_in(iter.filter(..).map(..), bump)` 형태인지 let-바인딩 클로저인지 표기 불가 · 동작은 확정). | 4 |  |
| 1 | 표기 불가 | (배치 J) L615 `mr > 0` 은 IR 에서 `roll*max_range > 999`(1000 나누기 전) 로 접혀 있어 `>0` 인지 `>=1` 인지 표기 불가(외연 동일). | 4 |  |
| 2 | 표기 불가 | (배치 J) L538 DI 지역명 `on_trajectory` 가 PositioningScore.on_periodic_trajectory(+0x31)에 바인딩되고 +0x30 on_trajectory 는 직접 읽힌다 — 소스가 `ps.on_trajectory \|\| … \|\| on_trajectory` 인지 필드명이 바뀐 것인지 표기 불가(읽는 바이트는 확정). | 4 |  |
| 3 | 표기 불가 | (배치 J) closure#5 Attack/Skill arm 의 `target.team == champ.team` 항이 `!(A && B) \|\| C` 인지 `!(A && B && !C)` 인지 표기 불가(외연 동일 · 분기 순서: A(타워가 나를 사거리) → B(대상≠타워) → C(팀 동일)). | 4 |  |
| 4 | 미탐색 | (배치 J) L227 v27_objective_discipline_action · L544 v25_objective_posture · L425 score · L599 check_kill_die_tick · L536 position_score_at_position · L608~ max_range_can_use/nearly · range_misjudge_rng/roll · aoe_heal_covers_low_ally · should_ignore_object_finish_kill_priority_target · v23_should_break_objective_hunt_anchor · nontarget_windup_perceived · battle_action/attack_summon_action/attack_jungle_action 내부 = 자식 명세(r13~r15) 소관, 여기선 계약만. | 4 |  |
| 5 | 미탐색 | (배치 J) EffectType vtable 슬롯 이름(+0x40 heal/+0x48 shield/+0x50 buff/+0x60 rush/+0x68 move_on_hit)은 divtable 의 정적 vtable(@anon…1131, 일치율 94%) 기준 — Arc<dyn> 런타임 구현체별 확인은 못 함. | 3 |  |
| 6 | 미탐색 | (배치 J) expected_heal/shield/buff 의 4번째 인자 @anon.11(24B 제로 Vec 헤더 + Debug fmt 포인터 상수)의 의미(빈 슬라이스/기본 파라미터로 추정) — expected_* 시그니처는 game_core 자식 소관. | 5 |  |
| 7 | 미탐색 | (배치 J) L604~606, L624~626, L634~639, L644~650 등 IR 이 없는 소스 줄은 주석/빈 줄/닫는 괄호로 추정(rmeta_srcmap 미대조). | 3 |  |
| 8 | 미탐색 | (배치 J) MapDef::camp_pos 내부 LocalKey 메모(g02.ll:6914)의 키·무효화 규칙은 game_core 소관(순수 좌표 캐시로 추정 — 미러 순서 무관 판정은 추정). | 4 |  |
| 9 | 표기 불가 | (배치 K) L444 `attacking_objective` 판정에 사용되는 target(+0x8) 은 Attack/Skill/Skill2/Ult 4 variant 모두 +0x8 (tcxdict) — Ult 의 target 이 같은 위치인지는 tcxdict SmallActionUlt 로 확인함(0x8 target). 다만 L444 이 4 variant 를 한 덩어리로 검사하므로 소스가 matches! 인지 개별 arm 인지는 표기 불가(외연 동일) | 3 |  |
| 10 | 미탐색 | (배치 K) blackboard 인덱스가 두 곳에서 다르다: closure$0(L58) 은 blackboard[player.info.team], L468 은 blackboard[1-team](%215). 둘 다 IR 그대로 기록했고 의도(_docs: Blackboard[team]=team 자체 정보, 관측은 1-team)는 별도 검증 안 함 | 4 |  |
| 11 | 미탐색 | (배치 K) epic_hunt.rs L451 · L453~455 · L460~462 · L474~475 · L477~479 · L488~489 · L495 · L500 · L502~503 · L505~506 · L508~511 · L513 은 IR 에 루트 줄이 없음(주석·빈 줄·닫는 괄호로 추정, rmeta_srcmap 미대조) | 3 |  |
| 12 | 미탐색 | (배치 K) team_plan(%7) 은 K 범위에서 load 0건. J 범위 사용 여부는 배치 J 소관 | 4 |  |
| 13 | 미탐색 | (배치 K) range_misjudge_rng/roll · max_range_can_use/nearly · position_score_at_position/at_cell · epic_action_score · EpicHuntSubPlan::score · get_input · is_recent_visible · range_adjust · block_input 내부는 계약만(자식 명세/game_core 경계) — TLS 메모 여부는 그 명세 참조 | 4 |  |
| 14 | 미탐색 | (배치 K) L476 max_by_key 의 첫 원소 키는 본체에서 epic_action_score 직접 호출(m15.ll 17555), fold 에서는 EpicHuntSubPlan::score 호출(aux m12 23228) — score 가 epic_action_score 의 단순 래퍼라는 가정(r14 계약)으로 같은 키로 봄. 래퍼가 self 상태를 읽는다면 첫 원소만 다를 수 있음(미검증) | 4 |  |

**`notes` 2건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | (배치 J) get_act_action(670~676)·objective_attack_range(16~17)·objective_wiggle_margin(20~30)·get_move_action(517~668) 은 같은 파일의 비공개 헬퍼가 전부 인라인된 것(별도 define 없음 — fnparts 로 확인). 배치 K 범위(432~515)와 이 헬퍼 줄 범위(517~668)는 겹치지 않는다. | 4 | 사실 서술 |
| 1 | (배치 K) 상수 8(m15.ll 16681, `add nuw nsw i32 %1476, 8`, dbg=v27_positioning_value L38)의 소속 소스 줄: 후보 셀 경로의 L38 계산(m15.ll 16958~16962)에는 8 이 없고 range_slack(≤8) 이 그 자리에 있어 L102 현 위치 점수식의 상수로 판단(추정). 합산식 자체는 IR 로 확정 | 4 | 사실 서술 |

<details><summary>`closed` 2건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

| # | 물음 | 닫은 근거 |
|---|---|---|
| 0 | (배치 K) Tower.nearest_enemy: Option<(usize,usize)> 의 .1 이 엔티티 id 인 것은 champ.id 와의 비교로 확정(m15.ll 15736). .0 의 의미(거리? 틱?)는 K 범위에서 안 읽어 미확인 — game_core Tower 갱신 코드(_gcbc) 미탐색 | 본문에 해소 표기가 있다 |
| 1 | (배치 K) v27 L130 best 갱신은 `score > best.score`(엄격) 로 확정(m15.ll 16972 icmp sle → 유지). L106/107 루프 순서(dx 외측·dy 내측)는 phi 구조로 확정. 후보 셀 순회 순서는 재현 시 동점 처리에 영향 | 본문에 해소 표기가 있다 |
</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

