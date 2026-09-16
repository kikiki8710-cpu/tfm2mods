# 배치 A — 변경 99 중 티어 2/3 14 (0.5.8 RVA → 0.6.0 RVA · exe 정규화 정렬 힌트 · 0.5.8 명세 요지)

정규화 정렬(`--norm`)이 이미 흡수한 것: PlayerState +0xd0 · Blackboard 0x2e8→0x5c8 · SmallActionPlay 태그/idx 재번호 · BattleSubPlanGoal 재번호 · Effect vt 슬롯(4 삽입) · LPH 오프셋 6. 아래 「잔여」 가 정렬로 못 닫은 차이.

## `d3d560` → `e759a0` i_am_chosen_defender  (1873→1873B · Δ+0)
- 명령 428→428 · 정렬 428 · exe 판정 ⚠정렬 불가(미확정) · 정규화 5 · 블록이동 16
- 잔여 구조: d3d866 피연산자 형 ; d3d86f 피연산자 형
- 정규화 흡수: 오프셋표 [rcx+930→a00] · 오프셋표 [rcx+9c0→a90] · 오프셋표 2e8→5c8 · 오프셋표 [rax+9c0→a90] · 오프셋표 [rcx+1e0→3e8]
- 0.5.8 명세 #37 `defense_nexus__i_am_chosen_defender` src game-ai\src\plan_legacy\old\defense_nexus.rs:297 · one_line: 아군 5명을 (푸셔 감당 가능, 웨이브 클리어 점수, 넥서스 거리, id) 키로 줄 세워 내가 상위 need_count 안에 드는지
- 0.5.8 logic(앞 1500자):
```
fn i_am_chosen_defender(player, data, need_count, exclude) -> bool

[L298] if need_count == 0 { return false }
[L301] team = player.info.team(+0x930); nexus = data.cache.nexus(+0x170)[team]?   // None → false
[L304] my_champ = data.cache.player_champion(+0x1e0)[team][player.info.position 태그(+0x9c0)]?   // None → false
[L308] if exclude.contains(&my_champ.id(+0x5c0)) { return false }   // 8개 청크 unrolled + 나머지 루프(and !7 / and 7 는 contains 최적화)

// ── 우리 구조물(넥서스/쌍둥이 타워)을 치는 적 미니언 웨이브 ────────────
[L313] rep_minion: Option<&Entity> = None; wave_size = 0
[L315] for m in data.cache.iter_minions(1 - team) {                 // Chain 이터레이터, next() 3구간
[L316]   if m.ty(+0x68) != 1 /*Minion*/ { continue }
[L317]   let Some(id) = m.ty.Minion.info.nearest_enemy(+0x88/+0x90) else { continue }
         let Some(s) = game.get_entity_by_id(id) /*vtable+0x1f0*/ else { continue }
[L318]   if !(s.team(+0x0) == TeamType::Player(team)) { continue }          // 우리 편 구조물
         if !(s.ty == 3 /*Nexus*/ || (s.ty == 2 /*Tower*/ && s.ty.Tower.info.ty(+0x128) ∈ {TwinA,TwinB})) { continue }
[L323]   wave_size += 1
[L324]   if rep_minion.is_none() { [L325] rep_minion = Some(m) }   // 첫 번째 해당 미니언이 대표
       }
[L329] wave_size = max(wave_size, 1)

// ── 푸셔(넥서스 근처 최근접 적 챔프) ────────────────────────────────
[L332] pusher = None; pusher_dist = u64::MAX
[L334] for c in data.cache.iter_champions(1 - team) {               // player_champion[enemy] 의 Some 만
[L335]   if !data.blackboard(+0x10)[1 - team].is_recent
```

## `d2da10` → `e71d40` DefenseNexusPlan::sub_plan  (2104→2112B · Δ+8)
- 명령 524→524 · 정렬 518 · exe 판정 ⚠정렬 불가(미확정) · 정규화 5
- 잔여 구조: 구 6명령(mov|eax, I…) / 신 6명령(mov|rax, qword ptr [rbp + I]…) 짝 없음
- 잔여 분기: d2e036 → d2e1ee/e7251c ; d2e0a1 → d2e023/e7234f ; d2e1da → d2e1ee/e7251c
- 정규화 흡수: 오프셋표 [rax+930→a00] · 오프셋표 [rax+9c0→a90] · 오프셋표 2e8→5c8 · 오프셋표 2e8→5c8 · 오프셋표 2e8→5c8
- 0.5.8 명세 #77 `defense_nexus__DefenseNexusPlan_sub_plan` src game-ai\src\plan_legacy\old\defense_nexus.rs:32 · one_line: 넥서스 방어 플랜의 서브플랜 결정 — DefenseNexus(계속 방어) vs Recall(귀환) 을 넥서스 위기·본진 위험·자기 HP·분수 여부로 판정
- 0.5.8 logic(앞 1500자):
```
L38: team = player.info.team (<2 아니면 bounds panic); champ = data.cache.player_champion[team][player.info.position].unwrap()
L40: hp_ratio = champ.hp*100 / champ.stat_cached.hp  (max=0 이면 div_by_zero panic — max(.,1) 없음)
L41: (lx,ly,rx,ry) = data.context.map.fountains[team]
L42: in_fountain = lx<=champ.x<=rx && ly<=champ.y<=ry   (x 먼저, 실패 시 y 는 안 봄 — 단락)
L44: nexus = data.cache.nexus[team].unwrap()
L45~47: enemy_bb = data.blackboard[1-team]; top/mid/bottom_front_minion = enemy_bb.{top,mid,bottom}_minion_state.front_minion.and_then(|id| cache.game.get_entity_by_id(id))   [vtable 0x1f0]
L48~50: ally_top/mid/bottom_front_minion = data.blackboard[team].* 동일 방식 — ★결과가 이후 어디에도 안 쓰임(호출만 남음, 관측 무영향)
L51~53: top_near = top_front_minion.is_some_and(|m| m.distance_sq(nexus) < 14400000001); mid_near, bottom_near 동일
L54: existing_lines_weak = valid_lines(context.tutorial).iter().all(|line| match line { Top=>top_near, Mid=>mid_near, Bottom=>bottom_near })
     valid_lines 인라인 표(27764~27816): None/Line/Total→[Top,Mid,Bottom] · First/Bottom→[Bottom] · TopSolo→[Top] · MidSolo→[Mid] · MidBottom→[Mid,Bottom] · JungleOnly→[] (빈 슬라이스면 all()=true)
L59: enemies = data.cache.champions(1-team, context.pool)   (bumpalo Vec<&Entity>, L60 뒤 drop)
L60: nexus_near_enemy_champion = enemies.iter().any(|e| e.distance_sq(nexus) < 14400000001)
     real_danger = nexus_near_enemy_champion  (dbg_value 가 같은 %245 의 not — 추가 조건 없음)
L82: if version > 1 {
       if goal_data.heal_commit { return Recall }            //
```

## `e4aec0` → `d56570` v2_obj_restore_safe  (432→462B · Δ+30)
- 명령 100→100 · 정렬 92 · exe 판정 ⚠정렬 불가(미확정) · 정규화 2 · 스택슬롯 26 · 콜리 주의 1
- 잔여 구조: 구 8명령(push|r13…) / 신 8명령(lea|rax, [rsp + I]…) 짝 없음 ; e4afde 피연산자 형
- 잔여 즉치: 0xd0→0x108 · 0xd0→0x108
- 콜리 주의: eb82d0→eda920 변경
- 정규화 흡수: 오프셋표 [r8+930→a00] · 오프셋표 [r8+9c0→a90]
- 0.5.8 명세 #33 `handler__v2_obj_restore_safe` src game-ai\src\plan_legacy\handler.rs:501 · one_line: 오브젝트 후 회복(restore)이 안전한지: 150000 안 '최근 가시' 적 챔프가 없거나, 있어도 2초 안에 안 죽으면 true
- 0.5.8 logic(앞 1500자):
```
fn LegacyPlanHandler::v2_obj_restore_safe(&self, version, rnd, player, data, debug) -> bool
// ⚠internal fastcc 로 인자 승격됨: IR 인자 = (version i64, rnd, player, data, debug). self 는 본문에서 안 읽혀 제거됨

[L504] team = player.info.team (+0x930)                         // >=2 → panic
        champ: &Entity = match data.cache.player_champion(+0x1e0)[team][player.info.position(+0x9c0) as usize] {
            Some(c) => c,
            None    => return true                                 // 내 챔프 없음 → 안전(phi [true, %15])
        }

[L505~509] near_enemies: Vec<&Entity, &Bump> =
        data.cache.iter_champions(1 - team)                       // player_champion[1-team] 5칸, Some 만 (simulation.rs:1905)
          .filter(|e| {                                            // ★aux: m13.ll 58204~58285 (call_mut 심)
[L507]        Entity::distance_sq(e, champ) < 22500000001          // abs_diff(x)^2 + abs_diff(y)^2, x=+0x660 y=+0x668 → 150000 이내
[L508]        && data.blackboard(+0x10)[1 - player.team].is_recent_visible(game, player, e)   // 최근 시야에 잡힌 적
[L509]        && !fight_model::is_ignored_well_enemy(version, player, e)   // 적 샘 위험구역의 적은 무시
          })
          .collect_in(data.context.pool(+0x0))                     // from_iter_in

[L510] if near_enemies.is_empty() { drop; return true }           // 근처 가시 적 없음 → 안전

[L513] my_die = fight_check::check_kill_die_tick(
            version, rnd, data,
            judger = player, focus = champ,
            enemy  = near_enemies,                     
```

## `e8a100` → `f33fa0` unsafe_v19_non_champion_walkup  (3432→3369B · Δ-63)
- 명령 684→670 · 정렬 641 · exe 판정 ⚠정렬 불가(미확정) · 정규화 9 · 블록이동 2 · 스택슬롯 210 · 콜리 주의 4
- 잔여 구조: 구 43명령(mov|r15, r8…) / 신 29명령(mov|r15, rdx…) 짝 없음 ; e8a1f3 피연산자 형 ; e8a22e 피연산자 형 ; e8a321 피연산자 형 ; e8a49a 피연산자 형 ; e8ad02 피연산자 형
- 잔여 분기: e8a409 → e8a9c7/f34859 ; e8a54b → e8a59a/f34415 ; e8a576 → e8a59a/f34415 ; e8a585 → e8a59a/f34415 ; e8a594 → e8ab16/f3499a
- 잔여 즉치: 0x228→0x258 · 0x228→0x258
- 잔여 변위: None+0x1e0→0x0 · rcx+0x670→0x8 · r9+0x10→0x1c8
- 콜리 주의: 12857f0→1643790 변경 ; 6d4d0→6ed50 미지(pdata 밖 thunk) ; d9a220→100b570 변경 ; eb82d0→eda920 변경
- 정규화 흡수: SmallActionPlay 태그 7→6 · SmallActionPlay 태그 c→b · SmallActionPlay idx(부호) -c→-b · 오프셋표 [r14+930→a00] · 오프셋표 [r14+9c0→a90]
- 0.5.8 명세 #216 `line_defense__LineDefense__unsafe_v19_non_champion_walkup` src game-ai\src\plan_legacy\sub_plan\line_defense.rs:72 · one_line: 비챔피언(미니언/타워 등) 대상 공격·스킬을 위해 걸어 들어가는 위치가 위험한가 — 타워 커버 위험 / 궤적·타워 포커스 위험 / 적 챔피언 킬각·리스크·미니언 웨이브 위험이면 true(행동 차단)
- 0.5.8 logic(앞 1500자):
```
fn unsafe_v19_non_champion_walkup(&self, version, rnd, player, data, parameter, action, has_runaway, debug) -> bool
L74: _t = ProfTimer(phase 82)   // prof::ENABLED 일 때만 Instant::now · 종료 시 PHASE_NANOS/CALLS[82] atomic add
L75: small_action = action.get_action(); Attack(6)/Skill(7)/Skill2(8)/Ult(9) 외 → return false
     backoff = (Ult ? 150000 : 15000)   // phi %62 · L61 에서 사용
L79: target = cache.game.get_entity_by_id(small_action.target_id)? ; None → false
L82: champ = cache.player_champion[team][pos]? ; None → false
L86: if target.team == champ.team || target.is_champion() → return false      // 비챔피언 대상 전용
L90: effect = action_effect(champ, small_action)? (L25~29: attack/skill/skill2(level>2)/ult(level>4) as_ref) ; None → false
L93: if !CastingTarget::check(&effect.target, champ, target) → return false
L97: action_kills_target = !(expected_damage_target(effect, context, champ, target) < target.hp)
L98: if !action_kills_target && v22_current_line_non_champion_action_tower_risk(version, context, cache, player, target) {
L99:   if context.debug { debug.infos[champ.id].push("#v22 lane tower cover block: target {target.id}") }
       return true }
L105: let Some((x,y)) = action_walkup_position(champ, target, effect, backoff) else { return false }   // 인라인 L39~66:
   L39/35: if effect(재계산 action_effect).is_some_and(|e| e.is_in_range(champ, target)) → return false  (※Some(walkup) 이 아니라 함수 전체 false: %182→%493 phi false)
   L43: if !(champ.team is Player(t) → target.visible_state[t]
```

## `dd90c0` → `f0e5f0` TeamPlan::update_steal  (2601→2692B · Δ+91)
- 명령 578→595 · 정렬 540 · exe 판정 ⚠정렬 불가(미확정) · 정규화 2 · 블록이동 13 · 스택슬롯 64
- 잔여 구조: 구 38명령(mov|r12, rcx…) / 신 55명령(mov|rsi, rcx…) 짝 없음 ; dd9274 피연산자 형 ; dd9311 피연산자 형 ; dd94b9 피연산자 형
- 잔여 분기: dd90de → dd9159/f0e6bf ; dd9143 → dd91b5/f0e84a ; dd9157 → dd91c1/f0e86f ; dd918f → dd91c1/f0e744 ; dd9292 → dd92c1/f0e82a
- 잔여 소형 즉치: 0xdd919b:0x9→0x2
- 잔여 즉치: 0x9→0x2 · 0x198→0x450 · 0x1a8→0x460 · 0x1a8→0x460 · 0x198→0x450 · 0x1a0→0x458 · 0x1a0→0x458 · 0x190→0x448
- 잔여 변위: r12+0x41a→0xc9c · r12+0x41a→0xc9c · r12+0x41b→0xc9d · r12+0x41a→0xc9c · r12+0x41a→0xc9c · rsi+0x1b9→0x471 · rsi+0x120→0x398 · rsi+0x120→0x398 · rsi+0x128→0x3a0 · rsi+0x1ba→0x471
- 정규화 흡수: 오프셋표 [r8+9c0→a90] · 오프셋표 [rdi+930→a00]
- 0.5.8 명세 #100 `team_plan__update_steal` src game-ai\src\plan_legacy\team_plan.rs:732 · one_line: [v4] 정글러 막타 스틸 상태기계 — should_steal_now 로 새 액션을 얻고, 진행 중 세션의 종료(성공/사망/대상소멸/대상변경/None)·새 세션 시작(쿨다운 10초)·세션 통계 갱신·Lurk/Commit 진입·틱 카운트·steal_action/prev 저장을 매 틱 수행
- 0.5.8 logic(앞 1500자):
```
fn update_steal(&mut self, version, player, data, goal_data, plan: &BigPlan)
  // L733: 정글러 전용
  if player.info.position != Jungle(1) { self.steal_action = None(-1); return }   // L906. prev_steal_action 은 안 건드림
  ctx = data.context; game = data.cache.game
  // L734~735
  new_action: StealAction(2B: tag, target) = if epic_exists(ctx) || serpen_exists(ctx) [tutorial ∈ {0,5,7,8}] { should_steal_now(version, player, goal_data, self, data) } else { None(0) }
  // L740~753
  prev_active = self.prev_steal_action.tag > 0   (Lurk/Commit)
  if !prev_active { if plan.tag == 9(Battle) { new_action = None(0) } ; target_changed = false }
  else if new_action.tag == 0 { target_changed = false }
  else { target_changed = (prev.target != new.target) }   // L753 xor
  new_active = new_action.tag != 0                          // L744
  // L755~757
  team = player.info.team
  me_dead = cache.player_champion[team][1].is_none()
  mob = game.get_game_mode().as_moba().unwrap()   (756/757 unwrap)
  ally_epic_buff_ticks = mob.epic_minion_buff_time[team]; ally_serpen_stacks = mob.serpen_count[team]
  // L759~804: 진행 중 세션 종료 판정
  if let Some(sess) = &self.current_steal_session {          // tag(+0x1ba) != 2
    t = sess.target == Serpen
    target_alive = if t { serpen_exists && mob.serpen.live_list.len != 0 } else { epic_exists && mob.epic.live_list.len != 0 }   // L761~762
    success = match self.steal_commit_snapshot { Some((t2, snap)) => if t2 { ally_serpen_stacks > snap } else { ally_epic_buff_ti
```

## `e03360` → `e84340` noncombat_steroid_window  (2922→3044B · Δ+122)
- 명령 669→714 · 정렬 604 · exe 판정 ⚠정렬 불가(미확정) · 정규화 12 · 블록이동 17 · 스택슬롯 84 · 콜리 주의 3
- 잔여 구조: 구 65명령(mov|r13, qword ptr [rax + I]…) / 신 110명령(mov|rdx, qword ptr [rax + I]…) 짝 없음 ; e03c05 피연산자 수
- 잔여 분기: e03620 → e03627/e84ad1 ; e03622 → e039dc/e84e69 ; e0365c → e039dc/e84abb ; e036fc → e0373b/e84713 ; e03761 → e039d9/e84abb
- 잔여 즉치: 0x108→0x118 · 0x108→0x118
- 잔여 변위: rsi+0x28→0x50 · rsi+0x50→0x68 · rsi+0x68→0x90 · rsi+0x90→0xa8
- 콜리 주의: d717e0→e6f6b0 미지(-189B) ; d75cb0→e7ff60 불일치(mig060 는 e71450) ; def0a0→e7ff60 불일치(mig060 는 e71450)
- 정규화 흡수: 오프셋표 [rcx+930→a00] · 오프셋표 2e8→5c8 · 오프셋표 [rax+9c0→a90] · 오프셋표 [rcx+1e0→3e8] · 오프셋표 [rax+9c0→a90] · 오프셋표 [rcx+1e0→3e8] · 오프셋표 [rax+9c0→a90] · 오프셋표 [rcx+1e0→3e8]
- 0.5.8 명세 #254 `buff_value__noncombat_steroid_window` src game-ai\src\buff_value.rs:378 · one_line: 비교전 스테로이드 창: 적 챔프가 교전권 밖(최근 비가시)일 때 아군 소액션이 노리는 정글/타워/넥서스/미니언이 수혜자 공격범위+0.5초 안이면 창 종류(반격형/미니언웨이브) 반환
- 0.5.8 logic(앞 1500자):
```
fn noncombat_steroid_window(player, data, caster, beneficiary) -> Option<NoncombatSteroidWindow>  [buff_value.rs:378~452]

384~387: let engage = |e: &Entity| -> u64 {                    // 클로저#0 · 전부 인라인
           e.attack_effect.as_ref().map(|ef| ef.range(e)).unwrap_or(0)   // Effect::range(effect.rs:26) = e.stat_buff_cached.range(+0x438) + ef.range(+0x4a0) + (e.level(+0x5c8) - 1) * ef.growth_range(+0x4a8) · None(+0x4c0 == -1) 이면 0
           + e.radius()                                          // entity.rs:1511~1515: mult=+0x470; mult==0 ? +0x680 : +0x680*(mult+100)/100
           + e.stat_cached.move_speed(+0x640) * 120 };
388: let bene_engage   = engage(beneficiary);
389: let caster_engage = engage(caster);
390: let enemy_team = 1 - player.info.team;                         // (36815) · team>=2 면 panic_bounds_check(37514)
390: let enemy_near = data.cache.iter_champions(enemy_team).any(|c| {   // player_champion[enemy_team][0..5] 5칸 언롤 · null 칸 skip
391:     let rb = bene_engage   + c.radius();
392:     let rc = caster_engage + c.radius();
393:     (c.distance_sq(beneficiary) <= rb*rb || c.distance_sq(caster) <= rc*rc)   // IR: dist_b > rb² 일 때만 dist_c 검사(단락) · `icmp ugt … → skip`
394:     && data.blackboard[enemy_team].is_recent_visible(data.cache.game, player, c)   // 5회 call(36988·37114·37240·37366·37492)
     });
395~404(추정): if enemy_near { return None; }                        // IR: is_recent_visible true → %440 {2, undef}
405~416: let team_action_hits = |target_i
```

## `d40f10` → `fb1510` evaluate_gank_opportunity_with_score  (1615→1459B · Δ-156)
- 명령 412→368 · 정렬 329 · exe 판정 ⚠정렬 불가(미확정) · 정규화 2 · 블록이동 39 · 스택슬롯 31 · 콜리 주의 1
- 잔여 구조: 구 83명령(lea|rbx, [rbp + I]…) / 신 39명령(lea|rsi, [rbp + I]…) 짝 없음
- 잔여 분기: d4100a → d4106a/fb1837 ; d41013 → d4106a/fb1837 ; d4103f → d4106a/fb1837 ; d41085 → d410b7/fb1723 ; d4130b → d41334/fb1940
- 잔여 즉치: 0xd8→0xe8 · 0xd8→0xe8
- 콜리 주의: 2a29480→de2fa0 미지(-2454B)
- 정규화 흡수: 오프셋표 [r8+930→a00] · 오프셋표 [rdx+9c0→a90]
- 0.5.8 명세 #66 `passive_jungle__evaluate_gank_opportunity_with_score` src game-ai\src\plan_legacy\old\passive_jungle.rs:693 · one_line: 정글러 관점 특정 라인의 갱 성공 가능성을 점수화하고 판단력 노이즈를 곱해 required_score 와 비교
- 0.5.8 logic(앞 1500자):
```
fn evaluate_gank_opportunity_with_score(rnd, player, data, line, required_score) -> (ok, evaluated, actual)
  self = data.cache; team = player.info.team; enemy_team = 1 - team  (L702, bounds<2)
  // L701~705 적 라이너 수집
  line_enemies = self.iter_champions(enemy_team)            // player_champion[enemy_team][0..5] 의 Some 만
      .filter(|e| is_near_line(data.context, e.x, e.y, line)          // L703 (aux m04.ll:66359)
               && data.blackboard[enemy_team].is_recent_visible(game, player, e))  // L704 (aux 66376) — 인덱스가 1-team 임에 주의
      .collect()
  if line_enemies.is_empty() → return (false, 0, 0)          // L708~709
  // L713~716 아군 라이너 수집
  line_allies = (0..5)
      .filter(|pos| data.blackboard[team].in_big_line(pos, line))   // L714: big_goal[pos] == Line{line}
      .filter_map(|pos| self.player_champion[team][pos])           // L715
      .filter(|a| a.hp*100 / a.stat_cached.hp > 40)                // L716 (aux m04.ll:66416)  ※ 자기 자신 제외 없음
      .collect()
  if line_allies.is_empty() → return (false, 0, 0)           // L720
  if line_enemies.len() > line_allies.len() + 1 → return (false, 0, 0)   // L724 (정글러 합류 감안)
  jungler_champ = self.player_champion[team][player.info.position] else return (false,0,0)  // L728
  target_enemy = line_enemies.iter().min_by_key(|e| (|e.x-j.x|)² + (|e.y-j.y|)²)  else return  // L733 (정글러에게 가장 가까운 적)
  nearest_ally = line_allies.iter().min_by_key(|a| dist²(a, target_enemy)) else return           // L738
  // L747~755 적 체력
  ehp = 
```

## `e7b640` → `f27ea0` buy_item  (856→1069B · Δ+213)
- 명령 234→293 · 정렬 189 · exe 판정 ⚠정렬 불가(미확정) · 정규화 1 · 블록이동 17 · 스택슬롯 26
- 잔여 구조: 구 45명령(mov|rbx, r8…) / 신 104명령(mov|qword ptr [rbp + I], rdx…) 짝 없음 ; e7b754 피연산자 형 ; e7b8f5 피연산자 수 ; e7b908 피연산자 형
- 잔여 분기: e7b6ae → e7b690/f27ef0 ; e7b723 → e7b870/f27fa0 ; e7b73b → e7b720/f2827c ; e7b74f → e7b720/f27fe0 ; e7b75e → e7b720/f28080
- 잔여 즉치: 0xc8→0xe8 · 0xc8→0xe8
- 잔여 변위: r8+0x4a0→0x510 · rax+0x8→0xa68
- 정규화 흡수: SmallActionPlay 태그 2→0
- 0.5.8 명세 #23 `lib__buy_item` src game-ai\src\lib.rs:1477 · one_line: 저티어(<4) 보유템이 없고 보유 ≤2개일 때, 활성·구매가능·tier0 아이템 중 챔피언 카테고리(근접/원거리/마법/유틸/암살)와 기본 HP 에 따라 허용 카테고리를 골라 후보를 만들고 무작위 1개 item_list 인덱스를 돌려준다
- 0.5.8 logic(앞 1500자):
```
fn buy_item(_version, rnd, player, _game, context) -> Option<usize>
// L1480
let has_low_tier_item = player.info.items.iter().any(|x| x.tier() < 4);   // vtable+0x70 — inlinedAt 사슬이 slice::iter::any(closure$0 L1480)를 직접 보인다(find().is_some() 아님)
if has_low_tier_item { return None; }
// L1487~1491
let pool = context.pool; let item_list = context.item_list;
let mut candidate: bumpalo Vec<usize> = Vec::new_in(pool);
// L1493
let tag = player.info.champion.category();                      // ChampionCategory (vtable+0x20)
// L1495
for (i, item) in item_list.iter().enumerate() {
  if player.info.items.len() > 2 { continue; }                   // L1496 (루프 불변)
  if !item.is_active() { continue; }                             // L1500 vtable+0x50
  if item.price() > player.info.gold { continue; }               // L1504 vtable+0x68 vs +0x998
  if item.tier() != 0 { continue; }                              // L1508
  let category = item.category();                                // L1512 vtable+0xa0 (ItemCategory)
  let is_defensive = matches!(category, Defense(2) | MagicResistance(3) | Hp(5));   // L1514
  let is_magic = category == Magic(4);                           // L1515
  let n = player.info.items.len();
  let ok = match tag {                                           // L1518
    Melee(0) => {                                                // L1523
      let hp = player.info.champion.stat().hp;                   // vtable+0x30, EntityStat+0x10
      if hp < 1550 { match n { 0 |
```

## `d65620` → `1011d20` serpen_passive_plan  (3528→3833B · Δ+305)
- 명령 728→797 · 정렬 262 · exe 판정 ⚠정렬 불가(미확정) · 정규화 1 · 블록이동 117 · 스택슬롯 17 · 패닉스텁 재배열 3 · 콜리 주의 11
- 잔여 구조: 구 466명령(mov|rsi, rcx…) / 신 535명령(lea|rbp, [rsp + I]…) 짝 없음 ; d658a2 피연산자 형 ; d658a7 피연산자 형 ; d6618b 피연산자 형 ; d6618b 피연산자 형 ; d6619c 피연산자 형
- 잔여 분기: d6564a → d65729/1011e2e ; d65658 → d65729/1011e2e ; d65687 → d656c0/1011ddb ; d65691 → d656c0/1011ddb ; d6570f → d65729/1011eb6
- 잔여 즉치: 0x1e0→0x10 · 0x31→0x3 · 0x32→0x3 · 0x32→0x3 · 0x32→0x2
- 잔여 변위: rdi+0x628→0x640 · r13+0x20→0x58
- 콜리 주의: 12a07d0→1016510 콜리 변경?(J0.00·+342B) ; 12a07d0→12920 미지(-341B) ; 12a07d0→196520 미지(-216B) ; 1323a00→1b3950 미지(+310B) ; 1323a00→3821af0 미지(-86B) ; 1323a00→e89b70 미지(+347B)
- 정규화 흡수: BattleSubPlanGoal 3→2
- 0.5.8 명세 #96 `serpen__serpen_passive_plan` src game-ai\src\plan_legacy\old\serpen.rs:416 · one_line: 세르펜 목표(phase)에 맞는 개인 BigPlan 을 고른다 — Hunt→SerpenHuntAndPoke, Setup→전략(object_buildup)·적 압박·라인 상태·이동시간으로 PassiveLine/SerpenHuntAndPoke/None
- 0.5.8 logic(앞 1500자):
```
ctx = data.context; team = player.info.team; my_pos = player.info.position
L417 if !serpen_exists(ctx) → return None (L418)
L421 is_line_phase = tutorial∈{0,5,7,8} ? tick < epic_jungle.first_spawn_tick.saturating_sub(tps*30) : true
L422 champ = cache.player_champion[team][my_pos].unwrap()
L423 skip = is_skip_serpen(version, rnd, player, data);  L425 if skip → return None (L506)
L426 match phase {
  Hunt(3)  → L427 return Some(BigPlan::SerpenHuntAndPoke(SerpenHuntAndPokePlan{v46_flee_threats: vec![], focus_serpen_only:false, vision_only:false, v46_flee:false}))
  Setup(1) → 아래
  _        → return None (L503)
}
L429 strategy = player.strategy(rnd, game)
L431 opposite_object_pressure = v23_enemy_object_pressure(player, data, JungleType::Morgard)
     (objective_helpers.rs:181~207 요지: 해당 오브젝트 생존 && 근처(180000, hp40%) 최근가시 적 != 0 && 적 >= 건강 아군)
L432 if opposite_object_pressure {
L433     if let Some(line) = v23_objective_setup_pressure_line(player, data, &[Bottom, Mid]) {
L434         return Some(PassiveLine(PassiveLinePlan{line, ..default}))
         }
     }
L438 object_buildup = strategy.object_buildup
L439 match object_buildup {
  Split(position) → L440 if !is_line_phase && position == my_pos {
                      L441 if !v25_objective_splitter_should_join_contest(version, player, data, JungleType::Serpen) {
                      L443     return Some(PassiveLine{line: fallback_line(ctx, Top)})
                      } }
                    // 그 외 → L496
  Flexible(6)     → L4
```

## `df0a90` → `dcb890` SerpenHuntAndPokePlan::is_end  (1016→1388B · Δ+372)
- 명령 245→310 · 정렬 214 · exe 판정 ⚠정렬 불가(미확정) · 정규화 5 · 블록이동 34 · 스택슬롯 32 · 콜리 주의 1
- 잔여 구조: 구 31명령(mov|r12, rdx…) / 신 96명령(mov|r15, rdx…) 짝 없음 ; df0c7a 피연산자 형 ; df0c7a 피연산자 형
- 잔여 분기: df0b3c → df0dce/dcb9a2 ; df0c48 → df0c71/dcbb3d ; df0c6b → df0d2d/dcbb83 ; df0c78 → df0ca1/dcbb3d ; df0c9b → df0d2d/dcbb88
- 잔여 소형 즉치: 0xdf0c94:0x10→0x8 · 0xdf0cf0:0x20→0x18
- 잔여 즉치: 0xa8→0xc8 · 0x10→0x8 · 0x18→0x28 · 0x20→0x18 · 0x28→0x10 · 0xa8→0xc8
- 잔여 변위: r15+0x41f→0x404 · r15+0x41f→0x3e4 · r15+0x420→0x404 · r12+0x100→0x150
- 콜리 주의: dd7250→f0e390 변경
- 정규화 흡수: SmallActionPlay 태그 1→2 · 오프셋표 [r9+930→a00] · SmallActionPlay 태그 1→2 · SmallActionPlay 태그 1→2 · 오프셋표 2e8→5c8
- 0.5.8 명세 #72 `serpen_hunt_and_poke__is_end` src game-ai\src\plan_legacy\old\serpen\hunt_and_poke.rs:162 · one_line: 세르펜 헌트/포크 플랜 종료 판정 — 목표가 Serpen 이 아니거나, Setup 국면에서 해제/적 부재/적 원거리, 또는 세르펜이 죽고 리스폰이 15초 넘게 남으면 종료
- 0.5.8 logic(앞 1500자):
```
fn is_end(&self, version, _rnd, player, data, team_plan, _debug) -> bool
// L163  take_active(JungleType::Serpen) 인라인(team_plan.rs:244→231 objective_target)
if team_plan.objective.tag != MainObjective::Serpen(1) { return true; }
// L168
let camp_pos = ctx.map.camp_pos(JungleType::Serpen, is_blue = player.team == 0);
// L169  take_setup_like 인라인(team_plan.rs:258): objective == Serpen && phase == ObjectPhase::Setup(1)
if team_plan.objective.tag == 1 && team_plan.objective.phase(+0x420) == 1 {
    // L170
    if team_plan.v24_objective_setup_should_release_to_passive(version, player, data, JungleType::Serpen) { return true; }
    // L174  vtable+0x100
    if game.is_visible_cell(player.team, camp_pos.x / 32000, camp_pos.y / 32000) {
        // L175~176  enemy_team = 1 - player.team
        let nearest_to_camp_enemy = cache.iter_champions(enemy_team)
            .filter(|e| blackboard[enemy_team].is_recent_visible(game, ctx, player, e))   // closure#0
            .min_by_key(|e| dist_sq(e, camp_pos));                                          // closure s_0 (aux m12)
        // L178
        let Some(e) = nearest_to_camp_enemy else { return true; };
        // L179
        if dist_sq(e, camp_pos) > 22500000000 { return true; }
    }
}
// L189  vtable+0x40 get_game_mode → Moba 아니면 unwrap 패닉
let m = game.get_game_mode().as_moba().unwrap();
let serpen = m.jungle_runner.serpen.live_list.first().and_then(|id| game.get_entity_by_id(*id));   // vtable+0x1f0
// L190
if serpen.is_some() { r
```

## `e23750` → `e5da70` cast::SmallActionUlt::is_end  (2118→1639B · Δ-479)
- 명령 484→379 · 정렬 378 · exe 판정 ⚠정렬 불가(미확정) · 정규화 20 · 블록이동 128 · 스택슬롯 2 · 패닉스텁 재배열 13 · 콜리 주의 1
- 잔여 구조: 구 106명령(push|rbp…) / 신 1명령(mov|rcx, rdi…) 짝 없음
- 잔여 분기: e23981 → e23d45/e5dfef ; e23a4b → e23e0d/e5df57 ; e23a60 → e23d47/e5dd8e ; e23ae7 → e23c32/fd9770 ; e23ba9 → e23ced/e5df89
- 잔여 소형 즉치: 0xe2375c:0x38→0x30 · 0xe23a6b:0x38→0x30 · 0xe23d47:0x38→0x30
- 잔여 즉치: 0x38→0x30 · 0x38→0x30 · 0x38→0x30
- 잔여 변위: rdi+0x8→0x10 · rdi+0x20→0x60 · rdi+0x60→0x20
- 콜리 주의: e266e0→e95dc0 불일치(mig060 는 e95770)
- 정규화 흡수: SmallActionPlay 태그 7→6 · 오프셋표 [r15+930→a00] · 오프셋표 [r15+9c0→a90] · 오프셋표 [r15+930→a00] · 오프셋표 [r15+9c0→a90] · 오프셋표 [rax+930→a00] · 오프셋표 [rax+9c0→a90] · 오프셋표 [rax+930→a00]
- 0.5.8 명세 #248 `cast__SmallActionUlt__is_end` src game-ai\src\small_action\cast.rs:289 · one_line: 궁 소액션 종료 판정: 시작 후 5틱 경과 · 이미 시전(is_act) · 대상 엔티티 소멸 · 대상이 내 챔피언 팀 시야에 없음 — 넷 중 하나면 true
- 0.5.8 logic(앞 1500자):
```
fn is_end(&self, _rnd, _version, player, data) -> bool   // cast.rs:289~296
  game = data.cache.game                                           // OperationData+0x0 → cache+0x0/+0x8 팻포인터
  // L291 — tick() 가상호출이 무조건 먼저 실행되므로 소스 순서 = (틱 조건) || is_act
  if self.start_tick + 5 <= game.tick() || self.is_act { return true }   // tick = vtable+0x28 · select(%15, true, is_act)
  // L292
  game.get_entity_by_id(self.target)                               // vtable+0x1f0 → Option<&Entity>(null=None)
      .is_none_or(|t| {                                            // None → true(대상 소멸 = 종료)
          // L293
          champ = data.cache.player_champion[player.info.team][player.info.position.as_index()].unwrap()   // +0x1e0 · team ult 2 아니면 panic_bounds_check · None 이면 unwrap_failed
          // L294
          !t.is_visible_from(champ)                                // entity.rs:1482: match champ.player_team() { None(Neutral) => true(보임 취급) → 클로저 false, Some(team) => t.visible_state[team].is_visible() }
      })
  // L296

★해석: 클로저 결과 phi = [Neutral → false] [Player → visible_state[team].tag != 0]. 즉 대상이 내 팀 시야에 없으면(Invisible/Unknown) 종료. is_visible_from 이 Neutral 관측자에 대해 true 를 돌려주는 것은 IR 분기 방향에서 확정(태그 1 → 클로저 false).
★rnd/version 미사용 · 쓰기 0.
★exe 0xe23750(2,118B) = SmallActionPlay::is_end JT 디스패처(jump-table lea @e2378c)이지 이 함수 단독 본체가 아니다 — Ult::is_end 본체는 그 안에 **인라인**(fnprobe: 패닉 Location cast.rs:293:99 @e238b0 · 293:21 @e23f34 · IND call [r12+0x28]=tick @e237b3 → [r12+0x1f0]=get_enti
```

## `d26900` → `ec4c20` PassiveLinePlan::v46_stage1  (5047→5686B · Δ+639)
- 명령 1067→1186 · 정렬 858 · exe 판정 ⚠정렬 불가(미확정) · 정규화 4 · 블록이동 74 · 스택슬롯 150 · 패닉스텁 재배열 1 · 콜리 주의 6
- 잔여 구조: 구 209명령(mov|rax, qword ptr [r13 + I]…) / 신 328명령(mov|rax, qword ptr [rbp + I]…) 짝 없음 ; d26934 피연산자 형 ; d26ddf 피연산자 수 ; d2736f 피연산자 수
- 잔여 분기: d2699b → d269b3/ec4cd6 ; d26a5f → d26a65/ec4d9b ; d26ae2 → d26ae9/ec4e0b ; d26c38 → d26c65/ec4fec ; d26d02 → d26d3f/ec5280
- 잔여 소형 즉치: 0xd279e9:0x1→0x9
- 잔여 즉치: 0x1f8→0x2f8 · 0x64→0x43 · 0xf→0x13880 · 0xfa0→0x1090 · 0x320→0x350 · 0xb0→0xc8 · 0xf0→0xb0 · 0xd8→0x1f0 · 0xb8→0xf0 · 0x110→0xd8
- 잔여 변위: rdx+0x668→0x10 · rsi+0x230→0x640
- 콜리 주의: 12857f0→1643790 변경 ; 128cc90→164b370 미지 ; 31a37c0→3821813 미지(+32B) ; 31a3863→3821770 미지(-32B) ; ccc9c0→2f31f90 미지(+220B) ; d31bb0→d72be0 미지
- 정규화 흡수: 오프셋표 [rax+9c0→a90] · 오프셋표 [rax+9c0→a90] · 오프셋표 [rsi+930→a00] · 오프셋표 [rax+9c0→a90]
- 0.5.8 명세 #213 `passive_line__PassiveLinePlan__v46_stage1` src game-ai\src\plan_legacy\old\passive_line.rs:663 · one_line: v46 라인 CS 사망예측 1단계 — near_enemies 중 '나를 먼저 죽일 수 있는' 커밋터(e.id, my_die, kill_dps)를 bumpalo Vec 으로 반환
- 0.5.8 logic(앞 1500자):
```
fn v46_stage1(version, team, my_pos, cache, ctx, champ, front, my_tower, my_hp, range_gate, only: Option<&[usize]>, near_enemies: &[&Entity]) -> bumpalo::Vec<(usize,usize,usize)>

// 준비 (L667~689)
pool = ctx.pool; tps = ctx.setting.tick_per_second; my_ms = champ.stat_cached.move_speed
my_range = champ.attack_effect.map(|f| f.range + f.growth_range*(level-1) + champ.stat_buff_cached.range).unwrap_or(0)   // L671 closure$0 · effect.rs:26 Effect::range
pull = my_range + champ.radius() + front.radius()          // L672 · radius() = mult==0 ? radius : radius*(mult+100)/100
cs_lock = champ.attack_duration()                          // L673
escape_ticks = front.distance(my_tower).saturating_sub(pull) / max(my_ms,1) + cs_lock   // L674
tower_disable_tick = match ctx.tutorial { First|Bottom => setting.tower_attack_disable_tick_2v2, MidBottom => …_3v3, _ => …tower_attack_disable_tick }   // L677~680
my_towers: bumpalo Vec<&Entity> = cache.iter_towers_without_nexus(team).filter(|t| game.tick() <= tower_disable_tick && t.distance(front).saturating_sub(pull) <= 150000).collect_in(pool)   // L684~688 closure$1(aux)
committers = Vec::new_in(pool)                              // L689

for e in near_enemies {                                     // L690
  if only.is_some_and(|ids| !ids.contains(&e.id)) { continue }   // L691 (IR: only==null → 통과, contains → 통과, 아니면 skip)
  ep = cache.player_by_champion_id(e.id).unwrap()          // L694 (None 이면 패닉)
  e_pos = ep.info.position.as_index()       
```

## `d59940` → `f749a0` calculate_action_score  (8506→9588B · Δ+1082)
- 명령 1844→2066 · 정렬 1614 · exe 판정 ⚠정렬 불가(미확정) · 정규화 5 · 블록이동 133 · 스택슬롯 204 · 패닉스텁 재배열 6 · 콜리 주의 3
- 잔여 구조: 구 230명령(mov|r14, r8…) / 신 452명령(mov|esi, dword ptr [r8 + I]…) 짝 없음 ; d59c5f 피연산자 형 ; d59d5e 피연산자 형 ; d59df7 피연산자 형 ; d59e00 피연산자 형 ; d59e00 피연산자 형
- 잔여 분기: d59d9a → d59da1/f74e0c ; d59ef0 → d5a0bd/f7511e ; d59f0e → d59f2c/f7532a ; d59f13 → d5a0cf/f7533d ; d59f26 → d5a0cf/f7533d
- 잔여 소형 즉치: 0xd5a86f:0x3→0x5
- 잔여 즉치: 0x1f8→0x218 · 0x78→0x1d0 · 0x1e→0x1 · 0x46→0x1 · 0x32→0x1 · 0x14→0x5 · 0x3e8→0x3e7 · 0x64→0x1 · 0x3→0x5 · 0xf→0x19
- 잔여 변위: r15+0x28→0x20 · r15+0x20→0x28 · r8+0x8→0x1e0 · rax+0x78→0x1d0 · rbx+0x40→0x20
- 콜리 주의: 12857f0→1643790 변경 ; 12857f0→ff1fc0 불일치(mig060 는 1643790) ; d31bb0→d72be0 미지
- 정규화 흡수: 오프셋표 [r8+930→a00] · 오프셋표 [rax+9c0→a90] · 오프셋표 2e8→5c8
- 0.5.8 명세 #186 `action_score__calculate_action_score` src game-ai\src\action_score.rs:8 · one_line: 액션(평타/스킬)을 대상 t 에 쓸 때의 점수 — 미니언은 막타 타이밍·라인 스타일, 챔피언/타워/넥서스는 계수×기대피해/HP + 보너스
- 0.5.8 logic(앞 1500자):
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
//     L76 lane_phase = !(tutorial∈{None,MidBottom,Line,Tota
```

## `cb03b0` → `e96780` EpicCheckSubPlan::action_candidates  (5540→7098B · Δ+1558)
- 명령 1071→1453 · 정렬 1014 · exe 판정 ⚠정렬 불가(미확정) · 정규화 6 · 블록이동 412 · 스택슬롯 350 · 패닉스텁 재배열 3 · 콜리 주의 8
- 잔여 구조: 구 57명령(mov|eax, dword ptr [rdx + I]…) / 신 439명령(ja|I…) 짝 없음 ; cb051f 피연산자 수 ; cb095f 피연산자 형 ; cb0c43 피연산자 형 ; cb10cd 피연산자 형 ; cb11fa 피연산자 형
- 잔여 분기: cb06bb → cb0716/e96879 ; cb06f8 → cb0734/e968b8 ; cb0711 → cb18bb/e97504 ; cb072e → cb04a3/e9814a ; cb073b → cb04c4/e98338
- 잔여 즉치: 0x2c8→0x2d8 · 0xe→0x4 · 0x124101101→0x124101100 · 0x3→0xd · 0x2c8→0x2d8
- 잔여 변위: r8+0x8→0x228
- 콜리 주의: 31a01a3→381e0b3 미지 ; cada80→16a7af0 미지(+84B) ; cada80→e920f0 미지(+2B) ; ccc7a0→3821770 미지(-503B) ; cdac20→ebde60 미지(pdata 밖 thunk) ; dd26e0→f09860 변경
- 정규화 흡수: 오프셋표 [rdx+930→a00] · SmallActionPlay 태그 2→1 · 오프셋표 2e8→5c8 · 오프셋표 [rax+9c0→a90] · 오프셋표 [rdi+1e0→3e8]
- 0.5.8 명세 #191 `epic_check__EpicCheck__action_candidates` src game-ai\src\plan_legacy\sub_plan\epic_check.rs:14 · one_line: 에픽(모르가드) 확인 서브플랜 후보: 위험이면 도주 단독 / v25 오브젝트 자세(Screen·WaitGroup·SoftDisengage)가 있으면 그 자세 전용 후보 / 없으면 (자기 진영일 때 Stump 캠프 경유 확인 후) 모르가드 캠프 AroundPosition + 적 근접 시 도주 + 적에게 보이면 교전 + 소환수 공격
- 0.5.8 logic(앞 1500자):
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
  let posture = team_plan.v25_objective_posture(version, player, data
```
