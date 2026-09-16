# 배치 D — 변경 99 중 티어 2/3 14 (0.5.8 RVA → 0.6.0 RVA · exe 정규화 정렬 힌트 · 0.5.8 명세 요지)

정규화 정렬(`--norm`)이 이미 흡수한 것: PlayerState +0xd0 · Blackboard 0x2e8→0x5c8 · SmallActionPlay 태그/idx 재번호 · BattleSubPlanGoal 재번호 · Effect vt 슬롯(4 삽입) · LPH 오프셋 6. 아래 「잔여」 가 정렬로 못 닫은 차이.

## `e5d300` → `d5ff20` try_engage_dive  (654→653B · Δ-1)
- 명령 146→144 · 정렬 140 · exe 판정 ⚠정렬 불가(미확정) · 정규화 2 · 스택슬롯 29 · 콜리 주의 3
- 잔여 구조: 구 6명령(mov|rax, qword ptr [rbp + I]…) / 신 4명령(mov|byte ptr [rbp + I], I…) 짝 없음
- 잔여 즉치: 0x188→0x298 · 0x118→0x228 · 0x188→0x298
- 잔여 변위: r14+0x1480→0x1f50 · r14+0x517→0xdcd · r14+0x519→0xdcf · r14+0x990→0x1408
- 콜리 주의: 31a01a3→381e0b3 미지 ; dfb840→dd8e70 변경 ; dfdd50→ddc9f0 불일치(mig060 는 dd8b60)
- 정규화 흡수: 오프셋표 [rax+930→a00] · 오프셋표 [rax+9c0→a90]
- 0.5.8 명세 #69 `engage__try_engage_dive` src game-ai\src\plan_legacy\handler\engage.rs:109 · one_line: 다이브 재합류 쿨다운·(v2+) 추격 레이스 가망 검사 후 TryKill 다이브 BattlePlan 을 생성·1틱 update 해 이탈 태세면 폐기, 아니면 반환
- 0.5.8 logic(앞 1500자):
```
fn try_engage_dive(&self, version, rnd, player, data, target_id, dive_tower, debug) -> Option<BattlePlan>
  tick = game.tick()                                                                  // L112 (vtable+0x28)
  tps  = context.setting.tick_per_second
  // dive_rejoin_cd(self) 인라인 (engage.rs:973) = self.last_dive_abandon_tick + 1 + tps*4
  if !(tick > self.last_dive_abandon_tick + 1 + tps*4) → return None                 // L112~113 (어보트 후 4초 쿨다운)
  if version > 1 {                                                                    // L116
    champ  = cache.player_champion[player.team][player.position]                     // L117
    target = game.get_entity_by_id(target_id)                                          // L118 (vtable+0x1f0)
    if let (Some(champ), Some(target)) = (champ, target) {
      if open_chase_race_hopeless(version, data, player, champ, target) → return None   // L119~120
    }
  }
  goal = BattlePlanGoal::TryKill(target_id, 60)                                       // L124
  plan = BattlePlan::new_dive(version, &goal, data, player)
  plan.entry_src = 2                                                                   // L125
  plan.dive_tower = dive_tower                                                         // L126
  if version > 1 { plan.set_main_objective(self.team_plan.objective) }                // L128 (battle.rs:350 인라인, 0x517 → 0xff 3B)
  plan.update(version, rnd, player, data, &self.positioning_score, &self.team_plan, debug)   // L130
  d
```

## `e7caf0` → `f60480` SerpenPokeSubPlan::action_candidates  (12629→12645B · Δ+16)
- 명령 2535→2537 · 정렬 2533 · exe 판정 ⚠정렬 불가(미확정) · 정규화 14 · 블록이동 2 · 스택슬롯 680 · 콜리 주의 7
- 잔여 구조: 구 2명령(mov|rax, qword ptr [rbp + I]…) / 신 4명령(lea|rax, [rbp + I]…) 짝 없음 ; e7d48e 피연산자 형 ; e7d48e 피연산자 형 ; e7d85f 피연산자 수
- 잔여 즉치: 0x448→0x478 · 0x448→0x478
- 콜리 주의: 12857f0→1643790 변경 ; 31a01a3→381e0b3 미지 ; c8c8f0→df9110 미지(+7B) ; ca9230→e168e0 미지(+0B) ; e77ef0→f5b930 미지(+0B) ; ea43a0→f8cc10 미지(-278B)
- 정규화 흡수: 오프셋표 [r14+930→a00] · 오프셋표 [r14+9c0→a90] · 오프셋표 [rax+928→9f8] · SmallActionPlay 태그 e→d · SmallActionPlay 태그 e→d · SmallActionPlay 태그 e→d · SmallActionPlay 태그 7→6 · SmallActionPlay 태그 7→6
- 0.5.8 명세 #200 `serpen_poke__SerpenPoke__action_candidates` src game-ai\src\plan_legacy\sub_plan\serpen_poke.rs:15 · one_line: 세르펜 포킹 서브플랜의 소액션 후보 생성: v27 규율액션 우선 → 구후보 중 공격형만 3단 필터 → 비면 이동후보(도주/추적/세르펜 배회)에서 최고점 1개
- 0.5.8 logic(앞 1500자):
```
action_candidates(self: ZST, version, rnd, player, data, parameter, team_plan, debug) -> Vec<SmallActionPlay>
# 기호: team = player.info.team(+0x930) · enemy = 1-team · champ = data.cache.player_champion[team][player.position].unwrap() (L23) · dist²(a,b) = |ax-bx|²+|ay-by|² (u64) · E_champs = cache.player_champion[enemy] (Option<&Entity> 5칸 flatten) · visible_to(e, t) = (t is Neutral) || e.visible_state[t.idx]==Visible (entity.rs:1482 인라인)

L16: if let Some(a) = team_plan.v27_objective_discipline_action(version, rnd, player, data, JungleType::Serpen(5)) { L17: return vec_in(bump=data.context.pool)[a] }   # v27 sret 184B, 태그@+177 != 255 이면 Some
L20: old_actions = self.action_candidates_old(version, rnd, player, data, parameter)   # self 는 poison(ZST)
L23: champ = player_champion[team][position].unwrap()
L24~26: nearest_enemy_tower: Option<&Entity> = cache.iter_towers_without_nexus(enemy).filter(|t| t.can_target(+0x6b9) && t.block_target_tick(+0x6a0)==0).min_by_key(|t| dist²(t, champ))
L29~92: act_actions = old_actions.iter().filter(s0_0).map(|a| a.clone()).collect_in(bump)
  s0_0(a) [캡처 game, champ, &nearest_enemy_tower]:
    L32: if let Some(id)=a.target_id() [태그 15..18 만 Some, 페이로드+8] && let Some(t)=game.get_entity_by_id(id) && t.ty != Champion(13) { return true }   # 비챔피언(미니언·세르펜·타워) 대상은 무조건 통과
    L33: match a {
      Attack(15) L36: target=get_entity_by_id(a.target)? else false; L37: eff=champ.attack_effect.unwrap(); L38: if !eff.is_in_range(champ,target) →false; L38~39: if
```

## `cc6170` → `ea8610` EpicPokeSubPlan::action_candidates  (12777→12728B · Δ-49)
- 명령 2549→2544 · 정렬 2483 · exe 판정 ⚠정렬 불가(미확정) · 정규화 15 · 블록이동 37 · 스택슬롯 660 · 콜리 주의 11
- 잔여 구조: 구 66명령(movsxd|rax, dword ptr [r9 + I]…) / 신 61명령(mov|rdx, qword ptr [rbp + I]…) 짝 없음 ; cc7ad4 피연산자 형 ; cc8245 피연산자 형 ; cc8765 피연산자 수 ; cc87d1 피연산자 수 ; cc8850 피연산자 형
- 잔여 분기: cc6863 → cc68ea/ea8d80 ; cc6885 → cc68ea/ea8d80 ; cc68a6 → cc68ea/ea8d80 ; cc68bd → cc68ea/ea8d80 ; cc69d3 → cc676d/ea8c14
- 잔여 즉치: 0x458→0x478 · 0x458→0x478
- 잔여 변위: rdi+0x668→0x660
- 콜리 주의: 12857f0→1643790 변경 ; 31a01a3→381e0b3 미지 ; c8c8f0→df9110 미지(+7B) ; c8d570→df9db0 불일치(mig060 는 df9780) ; c96830→e03070 불일치(mig060 는 dfc100) ; c98290→e04ad0 불일치(mig060 는 dfdb60)
- 정규화 흡수: 오프셋표 [r14+930→a00] · 오프셋표 [r14+9c0→a90] · 오프셋표 [rax+928→9f8] · SmallActionPlay 태그 e→d · SmallActionPlay 태그 e→d · SmallActionPlay 태그 e→d · SmallActionPlay 태그 7→6 · SmallActionPlay 태그 7→6
- 0.5.8 명세 #199 `epic_poke__EpicPoke__action_candidates` src game-ai\src\plan_legacy\sub_plan\epic_poke.rs:14 · one_line: 에픽(Morgard) 견제 서브플랜의 소액션 후보: v27 규율 액션 우선 → old 후보 중 사거리·타워 안전한 공격/스킬만 남기고, 없으면 견제 이동(Trace/Around/RunAway) 1개
- 0.5.8 logic(앞 1500자):
```
fn action_candidates(&mut self, version, rnd, player, data, parameter, team_plan, debug) -> Vec<SmallActionPlay>:
L15  if let Some(a) = team_plan.v27_objective_discipline_action(version, rnd, player, data, JungleType::Morgard) { return vec![a] }   // Option 태그 +0xb1 != 0xFF
L19  let mut old_actions = self.action_candidates_old(version, rnd, player, data, parameter)
L22  team = player.info.team(<2 아니면 bounds 패닉); champ = data.cache.player_champion[team][player.info.position].unwrap()
L23  nearest_enemy_tower: Option<&Entity> = cache.iter_towers_without_nexus(1-team).filter(|t| t.can_target && t.block_target_tick==0).min_by_key(|t| dist²(t,champ))   // 동점=먼저 나온 것
L28  let mut act_actions = old_actions.iter().filter(closure#2).map(|a| a.clone()).collect_in(pool)
     closure#2(a): [캡처 game(&dyn), champ, &nearest_enemy_tower]
       L31 if a ∈ {Attack,Skill,Skill2,Ult} && let Some(t)=game.get_entity_by_id(a.target) && t.ty != Champion → return TRUE (비챔피언 대상 공격은 무조건 통과)
       L32 match a:
         Attack(L35~38): t = get_entity_by_id(a.target) (None→false); champ.attack_effect.unwrap().is_in_range(champ,t) 아니면 false; match nearest_enemy_tower { None → true, Some(tw) → if tw.attack_effect.unwrap().is_in_range(tw, champ) && t.ty != Tower { return same_team(t, champ) /*적이면 false*/ } else true }
         Skill(L45~49): t 없으면 false; champ.skill_effect.unwrap().is_in_range(champ,t) 아니면 false; if let Some(tw)=tower && tw.atk.is_in_range(tw,champ) && t.ty!=Tower && !same_team(t,champ) → 
```

## `df1f50` → `fa8570` next_plan  (5418→5495B · Δ+77)
- 명령 1051→1068 · 정렬 1002 · exe 판정 ⚠정렬 불가(미확정) · 정규화 6 · 블록이동 15 · 스택슬롯 302 · 콜리 주의 2
- 잔여 구조: 구 49명령(mov|rax, qword ptr [r14 + I]…) / 신 66명령(mov|rcx, qword ptr [rbp + I]…) 짝 없음 ; df25f4 피연산자 형 ; df27ba 피연산자 형 ; df27ba 피연산자 형
- 잔여 분기: df218a → df2241/fa8861 ; df2196 → df2241/fa8861 ; df21b8 → df2241/fa8861 ; df2850 → df2864/fa8ec4 ; df2b6c → df2be0/fa9229
- 잔여 즉치: 0x5b8→0x8d8 · 0x118→0x228 · 0x118→0x228 · 0x5b8→0x8d8
- 잔여 변위: rsi+0x107→0x213
- 콜리 주의: 31a01a3→381e0b3 미지 ; dfb5d0→dd8b60 변경
- 정규화 흡수: 오프셋표 [rcx+930→a00] · 오프셋표 [rcx+9c0→a90] · 오프셋표 2e8→5c8 · 오프셋표 [rax+9c0→a90] · 오프셋표 [rsi+1e0→3e8]
- 0.5.8 명세 #250 `LineGankCoverPlan__next_plan` src game-ai\src\plan_legacy\old\line_gank\cover.rs:34 · one_line: 갱 커버 플랜의 BigPlan 전이: 내 챔프가 목표 부시(target_bush_v30)에 서 있을 때만(champ_bush==bush) 근처(150k) 적·아군 인덱스, 최근접 적 타워, 양측 die_tick, 타워 이동 틱을 재어 Battle(태그 9)로 전이(Some) 또는 None(유지); 부시 미도착이면 즉시 None. 배치 G = 머리(L34~65: target_bush 인라인 사장값·부시 판정·근처 목록·최근접 적 타워) / 배치 H = L68~132 (die_tick·타워 이동·Battle 생성 판정).
- 0.5.8 logic(앞 1500자):
```
// cover.rs:0~65 (배치 G)
// fn next_plan(&mut self, version, _rnd, player, data, _positioning_score, _team_plan, debug) -> Option<BigPlan>   [cover.rs:34~35 · m10.ll:11810]
//
// ── L36: self.target_bush(player, data) 197줄 인라인 (cover.rs:196~222, 반환 (u64,u64)) ── ★반환값 사용처 0 = 사장값. 남은 효과는 unwrap/bounds 패닉 4곳뿐(정상 데이터면 무해)
team = player.info.team (+0x930);  if team >= 2 → panic_bounds_check(cover.rs:197:17)            [11893~11899]
pos  = player.info.position@tag (+0x9c0) as usize
champ = cache.player_champion[team][pos] (+0x1e0);  None → unwrap_failed(197:95)                    [11907~11913, 12062]
line = self.line (+0x20)                                                                                 [11918 L198]
tower  = cache.<line>_tower[team]  (0x180 + line*32 + team*8)                                             [11924~11929]
tower2 = cache.<line>_tower2[team] (0x190 + line*32 + team*8)                                             [11930~11932]
optb = tower.or(tower2)   (select tower==null ? tower2 : tower)                                          [11933~11934 L199]
twins = cache.twin_towers[team] (+0x130: ptr, len)                                                       [11939~11946]
nearest_twin = twins.iter().min_by_key(|t| dist_sq(self.line.get_start_position(context.setting, team), (t.x, t.y)))   [11995~12059 L200~202 · 첫 원소 인라인 12021~12053 · 나머지 fold aux m12.ll:29368 · get_start_position 은 원소마다 호출]
   twins.len == 0 → nearest_twin = None                                  
```

## `dffa10` → `e80960` buff_value_v54  (6720→6830B · Δ+110)
- 명령 1586→1616 · 정렬 1428 · exe 판정 ⚠정렬 불가(미확정) · 정규화 5 · 블록이동 101 · 스택슬롯 73 · 패닉스텁 재배열 2 · 콜리 주의 2
- 잔여 구조: 구 158명령(mov|rbp, r9…) / 신 188명령(mov|r13, r9…) 짝 없음 ; dffaae 피연산자 형 ; dffaae 피연산자 형 ; dfffba 피연산자 형 ; e0005e 피연산자 형 ; e00095 피연산자 형
- 잔여 분기: dffad8 → dffbd2/e80b10 ; dffbd0 → dffbe1/e80b1e ; dffd6f → dffdaa/e80cea ; dffd7b → dffdaa/e80cea ; dffda0 → dffda7/e80ce7
- 잔여 즉치: 0xfa0→0x1090 · 0x320→0x350 · 0xfa0→0x1090 · 0x3f→0x20 · 0x20→0x3f
- 잔여 변위: rax+0x12f8→0xa00 · r14+0x4f0→0x520 · r14+0x4b0→0x4e0 · r14+0x4c0→0x4f0 · r14+0x4d0→0x500 · r14+0x4e0→0x510 · r14+0x7f0→0x850 · r14+0x810→0x870 · r14+0x7d0→0x830 · r14+0x800→0x860
- 콜리 주의: 31a37c0→3821813 미지(+32B) ; d31bb0→d72be0 미지
- 정규화 흡수: 오프셋표 [rax+930→a00] · 오프셋표 [rax+9c0→a90] · 오프셋표 [rax+930→a00] · 오프셋표 2e8→5c8 · 오프셋표 [rax+9c0→a90]
- 0.5.8 명세 #133 `buff_value__buff_value_v54` src game-ai\src\buff_value.rs:85 · one_line: BuffState 한 개를 받는 recv 에게 얼마나 가치 있나 — 공격(DPS 증가→앵커 적 HP가치 환산)·쿨감·힐·피해경감·기동·위기(undying/cc_immune/toughness) 항을 합산해 0..160
- 0.5.8 logic(앞 1500자):
```
fn buff_value_v54(buff, recv, data, player, parameter, crisis, incoming, epic_incoming, on_attack_damage, recv_hp_value) -> i64  [buff_value.rs:85]

§1 준비 (L88~101)
L88:  tps = data.context.setting.tick_per_second as i64
L90:  dur_sec = if buff.duration is Time(tick) { max(tick as i64 / tps, 1) } else { 6 }   // tps==0 → div_by_zero 패닉
L94:  window = min(dur_sec, 6)    // dur_sec 은 이후 미사용
L97:  (a_ps, s_ps, u_ps) = if let Some(p) = data.cache.player_by_champion_id(recv.id) {
        c = &cache.player_champion_cache[p.info.team][p.info.position]   // team bounds<2
L99~101: (sum(c.attack_per_sec[0..5])/5, (sum(c.skill_per_sec)+sum(c.skill2_per_sec))/5, sum(c.ult_per_sec)/5)
      } else { (0,0,0) }
L105: stat = recv.get_stat()   // = stat_cached 복사 (attack, magic_power, hp, defence, magic_resistance 사용)
L108: (e_hp,e_def,e_mr,e_n) = (0,0,0,0)
L109: for e in data.cache.iter_champions(1 - player.info.team) { e_hp += e.stat_cached.hp; e_def += …defence; e_mr += …magic_resistance; e_n += 1 }   // L110~114 적팀 5명
L116: if e_n > 0 { e_hp /= e_n; e_def /= e_n; e_mr /= e_n }   // L117~119 적 평균

§2 delta_dps — 버프가 늘리는 초당 피해 (L123~167)
L123: delta_dps = 0
L124: if buff.attack_mult > 0        { L125: delta_dps += a_ps * attack_mult / 100 }
L127: if buff.attack > 0             { L128: delta_dps += a_ps * attack / max(stat.attack,1) }
L130: if buff.attack_speed_mult > 0  { L132: delta_dps += a_ps * attack_speed_mult / 100 }
L134: if buff.crit_chance > 0        { L136: delta_dps += a_ps * cri
```

## `d639f0` → `1010060` check_serpen_giveup  (7210→7353B · Δ+143)
- 명령 1749→1773 · 정렬 1639 · exe 판정 ⚠정렬 불가(미확정) · 정규화 19 · 블록이동 153 · 스택슬롯 35
- 잔여 구조: 구 110명령(mov|rcx, rdi…) / 신 134명령(mov|qword ptr [rsp + I], r12…) 짝 없음 ; d63ba9 피연산자 형 ; d64479 피연산자 형 ; d64497 피연산자 형 ; d646cb 피연산자 형 ; d646e8 피연산자 형
- 잔여 분기: d63a6d → d63bd3/1010111 ; d63c3b → d63cb5/101035a ; d63da3 → d643a5/1010a43 ; d6419c → d642de/1010986 ; d64241 → d643a5/1010a43
- 잔여 변위: rax+0x41f→0x3e4 · rax+0x420→0x404 · rax+0xf8→0x28 · r15+0x10→0x8 · r15+0x10→0x18 · rax+0x28→0xf8 · rax+0xf8→0x28 · rcx+0x450→0x480 · r15+0x10→0x8 · r11+0x450→0x480
- 정규화 흡수: SmallActionPlay 태그 1→2 · BattleSubPlanGoal 3→2 · 오프셋표 [r15+930→a00] · 오프셋표 [rax+930→a00] · 오프셋표 2e8→5c8 · 오프셋표 [rax+9c0→a90] · 오프셋표 [rax+9c0→a90] · 오프셋표 [rax+9c0→a90]
- 0.5.8 명세 #93 `serpen__check_serpen_giveup` src game-ai\src\plan_legacy\old\serpen.rs:234 · one_line: 세르펜 목표를 포기할지 판정 — 세르펜 부재/과부하/근처·건강 머릿수 열세(판단 페널티 가산)로 결정
- 0.5.8 logic(앞 1500자):
```
ctx = data.context; team = player.info.team; enemy = 1-team; camp = JungleType::Serpen(5)
L235 if !serpen_exists(ctx) → return true                       // tutorial ∉ {None,MidBottom,Line,Total}
L241 mode = game.get_game_mode() (Moba 아니면 unwrap 패닉); if mode.jungle_runner.serpen.live_list.len == 0 → return true   // 세르펜 없음 = 포기
L246 if team_plan.take_hunt_commit(camp)  [objective == Some(Serpen{phase: Hunt, ..})] {
L247     if serpen.live_list.len != 0 && let Some(serpen) = game.get_entity_by_id(live_list[0]) {
L249         hp_ratio = serpen.hp*100 / serpen.stat_cached.hp
L250         if hp_ratio <= 20 → return false                     // 다 잡은 세르펜은 포기 금지
L251         if v23_visible_objective_overload(player, data, map.camp_pos(Serpen, team==0)) → return true
             // (objective_helpers.rs:54~57: 근처(반경 180000·hp40%) 최근가시 적 > 2 && 적 >= 건강 아군 + 2)
         }
     }
L260 my_serpen_count = mode.serpen_count[team]; L261 enemy_serpen_count = mode.serpen_count[enemy]
L263 if my_serpen_count > enemy_serpen_count {
L265     near_ally  = player_champion[team].iter_champions().filter(|e| is_bottom_side(ctx,e.x,e.y) && e.hp*100/e.max_hp > 49).count()
L268     near_enemy = player_champion[enemy].iter_champions().filter(|e| is_bottom_side && hp% > 49 && blackboard[1-team].is_recent_visible(game, player, e)).count()
L271     penalty = macro_judgement_penalty(version, player)
L272     return near_ally + penalty < near_enemy
     }
L274 near_ally  = (L265 와 동일 필터).count()
L277 near_ene
```

## `e8fd70` → `f3d660` AgentVerHamster::item_v26  (818→1007B · Δ+189)
- 명령 208→247 · 정렬 156 · exe 판정 ⚠정렬 불가(미확정) · 블록이동 8 · 스택슬롯 21 · 콜리 주의 2
- 잔여 구조: 구 52명령(mov|rsi, r9…) / 신 91명령(movaps|xmmword ptr [rsp + I], …) 짝 없음 ; e8fdbe 피연산자 수
- 잔여 분기: e8ff0c → e8ff8b/f3d8d7 ; e8ff67 → e8ff7d/f3d8ac ; e8ffc4 → e9002e/f3d9ec ; e8ffcc → e8ffe7/f3d981 ; e90006 → e9004f/f3d9fb
- 잔여 즉치: 0x78→0xb8 · 0x78→0xb8
- 잔여 변위: rdx+0x4e8→0x558 · rdx+0x4f0→0x560 · rax+0x8→0x58 · rbx+0x998→0xa68
- 콜리 주의: 105fae0→16a8100 콜리 변경?(J0.50·+278B) ; d7a360→381e0cb 미지
- 0.5.8 명세 #225 `AgentVerHamster__item_v26` src game-ai\src\lib.rs:1149 · one_line: v26 빌드 경로: 활성 아이템빌드의 build_slot 번째 목표 아이템을 잡고, 인벤 현재템→목표로 가는 다음 아이템(또는 빌드경로 첫 아이템)을 골 조건까지 검사해 반환
- 0.5.8 logic(앞 1500자):
```
fn item_v26(&mut self, rnd, player, context, build_slot, inventory_index) -> Option<usize>   // lib.rs:1149~1170 (self 미사용)
  item_list = &context.item_list                                   // IR 는 승격된 %2(&Vec<Box<dyn ItemInfo>>) — GameContext 필드 오프셋은 호출자(buy_item/upgrade_item · exe e8fca4 `mov r15,[rax+0x30]`) 소관이라 이 함수 mem 표엔 없음
  // L1151 — 목표 아이템: 활성 빌드 항목 중 build_slot 번째
  target: usize = *player.info.item_builds.iter()
        .filter(|&&i| i < item_list.len() && item_list[i].is_active())   // i>=len 이면 패닉 없이 술어 거짓(get 의미) · is_active = vtable+0x50
        .nth(build_slot)?                                            // 블록 %22~%40 = advance_by(build_slot: 술어 참일 때만 accum-=1), %43~%59 = next(). 소진 → None
  target_item = &item_list[target]
  // L1153
  next_item: usize = if let Some(inv) = inventory_index {
      // L1154
      current = player.info.items.get(inv)?                          // inv >= items.len → None(패닉 아님 · %64→%167)
      // L1155
      random_item_next_toward_target(item_list, current.key(), target_item.key(), rnd)?   // key = vtable+0x58 → &String → &str. game_core 경계(item.rs:353)
  } else {
      // L1157
      path: Vec<usize> = random_item_build_path(item_list, target, rnd)?   // game_core 경계(item.rs:320) · None 니치 cap==-1
      // L1158
      first = *path.first()?                                         // len==0 → drop(path), None
      // L1159
      drop(path); first
  }
  // L1161
  inv_active: Vec<usize> = active_inventory_item_indices(player,
```

## `dce220` → `ef4690` v3_epicops_buff_window  (764→1046B · Δ+282)
- 명령 195→243 · 정렬 182 · exe 판정 ⚠정렬 불가(미확정) · 정규화 4 · 블록이동 7 · 콜리 주의 2
- 잔여 구조: 구 13명령(push|r13…) / 신 61명령(mov|r9, qword ptr [rsp + I]…) 짝 없음
- 잔여 분기: dce2e1 → dce33f/ef4829 ; dce2f6 → dce33f/ef4829 ; dce33a → dce2a0/ef4a2a ; dce354 → dce370/ef485c
- 잔여 소형 즉치: 0xdce22c:0x48→0x50 · 0xdce2b3:0x48→0x50
- 잔여 즉치: 0x48→0x50 · 0x48→0x50
- 잔여 변위: rsi+0x41f→0xcd5 · rsi+0x41f→0xcd5 · rsi+0xd0→0x2f8 · rsi+0xc0→0x2e8 · rsi+0xc0→0x2e8 · rsi+0xc8→0x2f0 · rsi+0xd0→0x2f8 · rsi+0x410→0x728 · rsi+0x41f→0xcd5 · rsi+0x421→0x3e4
- 콜리 주의: d666c0→1012f30 변경 ; deaa70→fb7b20 변경
- 정규화 흡수: SmallActionPlay 태그 1→0 · 오프셋표 [rdi+508→578] · 오프셋표 [rdi+4f8→568] · 오프셋표 [rdi+9c0→a90]
- 0.5.8 명세 #18 `old_epic__v3_epicops_buff_window` src game-ai\src\plan_legacy\old\epic.rs:634 · one_line: 에픽(오브젝트) 국면에서 수리·세르펜징벌 목표를 선점하고, 아니면 압박 라인 변경 채팅을 대표 1명이 발화
- 0.5.8 logic(앞 1500자):
```
fn v3_epicops_buff_window(&mut self, version, rnd, player, data, goal_data, plan) -> bool {

// [1] 수리(Repair) 선점 — epic.rs:635
let team = player.info.team; // PlayerState+0x930
match v3_epicops_repair_need(player, data, plan) { // i8 반환. IR 인자 `(team, data.cache, plan)` 은 **ArgumentPromotion 아티팩트**라 소스 인자와 다르다. tcx sig = fn(&PlayerState, &OperationData, &BigPlan). promotion 이 일어났다는 것 자체가 'player 에서 info.team 만, data 에서 cache 만 읽는다'의 증명
 1 => { // 637~638
 self.objective = Some(MainObjective::Repair); // TeamPlan+0x41f = 7
 self.chats.push(Chat::Repair(0)); // 태그 23, usize 필드 0
 return true;
 }
 2 => { // 642
 self.objective = Some(MainObjective::Repair); // 태그 7 만, 채팅 없음
 return true;
 }
 _ => {}
}

// [2] 적이 오브젝트를 먹는 중 + 세르펜 교전에서 확실히 이긴다 → 세르펜 징벌 — 651~658
if is_object_being_taken_by_enemy(player, data, goal_data, self, WavePriorityObject::Serpen /* ★IR 의 5번째 인자 i1 은 1B 열거형의 ABI 표현이다(불리언 아님). tcx sig = fn(&PlayerState,&OperationData,&GoalData,&TeamPlan,WavePriorityObject) -> bool, 0=Morgard/1=Serpen */) // 651
 && v3_serpen_contest_clear_win(version, player, data, self) { // 652
 self.eo_serpen_punish_issues += 1; // 653, +0x410
 self.objective = Some(MainObjective::Serpen{ phase: ObjectPhase::Setup, with_battle: true });
 // 654, +0x41f=1 +0x420=1 +0x421=1
 self.chats.push(Chat::SerpenSetup(0)); // 658, 태그 25
 return true;
}
// ★분기 순서 주의: is_object_being_taken_by_enemy 가 false 면 곧장 [3] 으로 간다.
// true 인데 v3_serpen_contest_clear_win 이 false 여도 [3] 으로 간다(단락 아님, 같은 합류 블록).

//
```

## `eaeda0` → `f6bda0` EpicHuntSubPlan::action_candidates  (20037→20373B · Δ+336)
- 명령 3960→4024 · 정렬 3908 · exe 판정 ⚠정렬 불가(미확정) · 정규화 29 · 블록이동 75 · 스택슬롯 1121 · 패닉스텁 재배열 2 · 콜리 주의 14
- 잔여 구조: 구 52명령(mov|r15, r9…) / 신 116명령(mov|qword ptr [rbp + I], r9…) 짝 없음 ; eaf2ae 피연산자 수 ; eafa86 피연산자 형 ; eafe3f 피연산자 형 ; eafe43 피연산자 형 ; eb07e8 피연산자 형
- 잔여 분기: eaee3c → eaee7d/f6be80 ; eaf0d0 → eaf0f1/f6c0f3 ; eaf1cc → eaf1fa/f6c1f5 ; eaf1d5 → eaf1fa/f6c1f5 ; eaf1df → eaf1fa/f6c1f5
- 잔여 소형 즉치: 0xeb1e49:0xf1→0xf2
- 잔여 즉치: 0x668→0x688 · 0xf1→0xf2 · 0x668→0x688
- 잔여 변위: rdx+0x4a0→0x4a8 · rdx+0x4a8→0x5c8 · rdx+0x5c8→0x438
- 데이터: eaf219 movdqa 16B 01000000000000000400000000000000→01000000000000000500000000000000 ; eaf248 movdqa 16B 01000000000000000500000000000000→01000000000000000400000000000000
- 콜리 주의: 12857f0→1643790 변경 ; 128cc90→164b370 미지 ; 31a01a3→381e0b3 미지 ; c8c8f0→df9110 미지(+7B) ; c8d360→df9ba0 불일치(mig060 는 df9570) ; c94a00→e01240 불일치(mig060 는 dfa2d0)
- 정규화 흡수: 오프셋표 [rax+930→a00] · 오프셋표 [rax+9c0→a90] · SmallActionPlay 태그 f→e · SmallActionPlay 태그 10→f · SmallActionPlay 태그 11→10 · 오프셋표 [rcx+508→578] · 오프셋표 [rcx+4f8→568] · SmallActionPlay 태그 e→d
- 0.5.8 명세 #198 `epic_hunt__EpicHunt__action_candidates` src game-ai\src\plan_legacy\sub_plan\epic_hunt.rs:200 · one_line: EpicHunt(모가드 사냥) 서브플랜의 후보 행동 생성: need_recall 갱신→귀환/규율 단일후보 조기반환→전투·소환수·정글 공격후보 필터/정리→get_move_action 이동후보 합성(J: 200~429) → 이후 타워·에픽 위치·최적 이동 선별(K: 432~515)
- 0.5.8 logic(앞 1500자):
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
                if (info.focused(@0x88/0x90) == Some(champ.id(@0x5c0)) && dmg*3 >= champ.hp(@0x670)) || champ.hp <= dm
```

## `e8d6f0` → `f3adb0` AgentVerHamster::update_state  (2159→2597B · Δ+438)
- 명령 482→569 · 정렬 453 · exe 판정 ⚠정렬 불가(미확정) · 정규화 9 · 블록이동 9 · 스택슬롯 103 · 콜리 주의 7
- 잔여 구조: 구 29명령(mov|r10, qword ptr [rbp + I]…) / 신 116명령(mov|r11, qword ptr [rbp + I]…) 짝 없음 ; e8deb8 피연산자 형
- 잔여 분기: e8d807 → e8d80f/f3aecf ; e8d8ef → e8d94e/f3b0cb ; e8d8f8 → e8d917/f3b127 ; e8d950 → e8d98a/f3b723 ; e8dacb → e8db27/f3b2da
- 잔여 소형 즉치: 0xe8dc29:0xf1→0xf2
- 잔여 즉치: 0x198→0x1b8 · 0xf1→0xf2 · 0x198→0x1b8
- 잔여 변위: r15+0x464→0x49c · rdi+0x2910→0x3550 · rdi+0x2910→0x3608 · rdi+0x29ca→0x36c2 · rdi+0x2910→0x3550 · rdi+0x2910→0x3608 · rdi+0x2909→0x3601 · rdi+0x2909→0x3601 · rdi+0x2909→0x3601 · rdi+0x2948→0x3640
- 콜리 주의: 16047b0→17d68b0 미지(+40B) ; d1bb20→e5b900 변경 ; e22ff0→d5a940 미지 ; e23750→e5da70 변경 ; e4c5c0→d52c00 불일치(mig060 는 d3d210) ; e8cef0→f3a420 미지(+344B)
- 정규화 흡수: SmallActionPlay 태그 c→b · SmallActionPlay 태그 c→b · SmallActionPlay 태그 7→6 · SmallActionPlay 마스크 103f3→81fb · SmallActionPlay 마스크 f40c→7a04 · 오프셋표 [rax+930→a00] · 오프셋표 [rax+9c0→a90] · SmallActionPlay 태그 7→6
- 0.5.8 명세 #218 `AgentVerHamster__update_state` src game-ai\src\lib.rs:556 · one_line: 틱당 AI 상태 갱신 루트: 이벤트→(lapse 게이트)플랜 update→소액션 update_state→(전제상실/종료 ∧ !can_skip_eval)이면 update_small_action, 아니면 extend_action. 반환 Vec 은 항상 빈 것
- 0.5.8 logic(앞 1500자):
```
fn update_state(&mut self, rnd, player, data) -> Vec<TurnEvent>   // lib.rs:556~610
  _t_ev = ProfTimer::start(23)                                  // L558 (ENABLED 원자 0 이면 None)
  self.update_event(rnd, player, data.cache, data.context)      // L559 (콜리 · 명세 밖)
  drop(_t_ev)                                                   // L560 → PHASE_NANOS[23]+=ns, PHASE_CALLS[23]+=1
  _t_plan = ProfTimer::start(24)                                // L562
  lapse = utils::player_awareness_lapse(player, data)           // L566
  self.last_lapse = lapse                                       // +0x29ca 쓰기
  res = if self.version > 1 {                                   // L569 (+0x2910)
          self.update_plan_lapse(rnd, player, data, lapse)       // L570 → lib.rs:689 인라인: plan_system.update(version, rnd, player, data, &mut self.debug, lapse); Vec::new_in(pool)
        } else if lapse {                                       // L571 (v≤1 · 인지공백)
          Vec::new_in(data.context.pool)                        // L572 — 플랜 update 생략
        } else {
          self.update_plan(rnd, player, data)                   // L574 → lib.rs:683: plan_system.update(version, rnd, player, data, &mut self.debug, false); Vec::new_in(pool)
        }                                                       // ★세 경로 모두 빈 Vec
  drop(_t_plan)                                                 // L576
  _t_sa = ProfTimer::start(25)                                  // L577
  { _t_sus = ProfTimer::start(28)              
```

## `df0e90` → `dcbe00` SerpenHuntAndPokePlan::sub_plan  (3471→4064B · Δ+593)
- 명령 829→949 · 정렬 716 · exe 판정 ⚠정렬 불가(미확정) · 정규화 7 · 블록이동 180 · 스택슬롯 59 · 패닉스텁 재배열 5 · 콜리 주의 7
- 잔여 구조: 구 113명령(mov|r10, qword ptr [rbp + I]…) / 신 233명령(mov|r14, r9…) 짝 없음 ; df1668 피연산자 형 ; df1670 피연산자 형 ; df16eb 피연산자 형 ; df16f3 피연산자 형 ; df176e 피연산자 형
- 잔여 분기: df0f55 → df0fcd/dcbf2a ; df0f75 → df0fb1/dcc045 ; df0f8d → df0fb1/dcc045 ; df0fac → df162f/dcc055 ; df0fc0 → df162f/dcc055
- 잔여 소형 즉치: 0xdf113f:0x3→0x0
- 잔여 즉치: 0x33→0x2 · 0x0→0x33 · 0x3→0x0 · 0x53d1ac101→-0x7ffffffffffffff3
- 잔여 변위: r12+0x8→0x10 · rsi+0x8→0x670 · rax+0x41f→0x3b · rax+0x420→0xcc2 · rdx+0x508→0x8 · r12+0x8→0x10 · rax+0x8→0x1f0 · rsi+0x8→0x660 · rdi+0x10→0x18 · rdi+0x18→0x20
- 콜리 주의: 31a37c0→3821813 미지(+32B) ; 31a3863→3821770 미지(-32B) ; d3b2a0→eed7f0 변경 ; dd5db0→f0cfe0 변경 ; e1c100→df8d60 미지(pdata 밖 thunk) ; e7a8c0→ef7590 불일치(mig060 는 f27140)
- 정규화 흡수: 오프셋표 [rax+930→a00] · SmallActionPlay 태그 1→0 · SmallActionPlay 태그 0→1 · 오프셋표 2e8→5c8 · 오프셋표 [r14+930→a00] · 오프셋표 [r14+9c0→a90] · 오프셋표 2e8→5c8
- 0.5.8 명세 #97 `serpen_hunt_and_poke__sub_plan` src game-ai\src\plan_legacy\old\serpen\hunt_and_poke.rs:30 · one_line: 세르펜 사냥/견제 빅플랜의 서브플랜 선택 — Epic 판과 동형(Steal/SerpenCheck/Recall/LineDefense/SerpenHunt/SerpenPoke), 차이 5곳은 unknown/logic 에 명시
- 0.5.8 logic(앞 1500자):
```
// serpen/hunt_and_poke.rs:30~92 (+ check_recall :99~159 인라인). Epic 판(epic_hunt_and_poke__sub_plan.json)과 같은 골격 — ★표시가 차이.
t, pos, champ = … (:32)
// A. 스틸 (:35~44)
if self.focus_serpen_only || self.vision_only {
   serpen = as_moba().jungle_runner.serpen.live_list.first().and_then(get_entity_by_id)   // :36~37 (MobaMode+0x1d0/+0x1d8)
   Some → Steal{ last_vision_tick:0, target:Some(Serpen) ★, commit:focus_serpen_only }   // :39
   None → SerpenCheck{false}                                                            // :44
}
// B. 일반
hp_ratio = hp*100/max_hp                                                                 // :47
serpen = live_list.first().and_then(get_entity_by_id); None → SerpenCheck{false}         // :48~50
(x0,y0,x1,y1) = map.fountains[t]; is_in_heal_area = …                                    // :55~56
can_upgrade_item = upgrade_item(version, rnd, player, game, ctx).is_some()               // :57 (★Epic 은 :48 에서 먼저 계산 — 순서만 다르고 값 동일)
if serpen.hp == serpen.max_hp && (hp_ratio<51 || can_upgrade_item || (is_in_heal_area && hp<max_hp)) → Recall   // :58~59
// C. 오브젝트 게이트
if team_plan.objective != Some(Serpen{..}) → Recall                                       // :63 / :90 ★태그 1
if phase == Hunt {                                                                        // :64
   strategy = player.strategy(rnd, game)                                                  // :65
   if object_buildup == Split(pos) && v25_objective_splitter_can_stay(version, player, data, 
```

## `de92d0` → `fb5ff0` epic_passive_plan  (4417→5343B · Δ+926)
- 명령 900→989 · 정렬 498 · exe 판정 ⚠정렬 불가(미확정) · 정규화 6 · 블록이동 154 · 스택슬롯 101 · 패닉스텁 재배열 2 · 콜리 주의 10
- 잔여 구조: 구 402명령(mov|rdi, r9…) / 신 491명령(mov|rbx, qword ptr [rsp + I]…) 짝 없음 ; de93e1 피연산자 형 ; de940c 피연산자 형 ; de9604 피연산자 형 ; de9604 피연산자 형 ; de9cc0 피연산자 형
- 잔여 분기: de92fa → de9e24/fb60b8 ; de9340 → de9379/fb612b ; de9348 → de9e24/fb60cf ; de9374 → de9e2b/fb6328 ; de93ab → de93cd/fb62a8
- 잔여 소형 즉치: 0xde9e24:-0x1→0x3 · 0xdea176:0x2→0x3
- 잔여 즉치: 0x31→0x2 · 0x32→0x2 · 0x32→0x3 · 0x2→0xff · -0x1→0x3 · 0x2→0x3 · 0x1→0xff
- 잔여 변위: rdi+0x508→0x20 · rcx+0x10e→0x110 · rcx+0x11e→0x123 · rcx+0x10e→0x110 · rcx+0x11e→0x125 · r15+0x21d0→0x23b0 · r15+0x21c0→0x23a0 · r9+0x21d0→0x23b0 · r9+0x21c0→0x23a0 · r15+0x660→0x28
- 콜리 주의: 12a07d0→16a7af0 콜리 변경?(J0.00·+115B) ; 1323a00→157b720 미지(-31B) ; 1323a00→16a7af0 미지(+383B) ; 1323a00→ef57f0 미지(+449B) ; 1323a00→f89d70 미지(+142B) ; 1323a00→f904b0 미지(+232B)
- 정규화 흡수: 오프셋표 [rdi+9c0→a90] · BattleSubPlanGoal 3→2 · SmallActionPlay 태그 2→1 · 오프셋표 2e8→5c8 · SmallActionPlay 태그 1→0 · 오프셋표 2e8→5c8
- 0.5.8 명세 #82 `epic__epic_passive_plan` src game-ai\src\plan_legacy\old\epic.rs:317 · one_line: 에픽(모가드) 목표의 수동 국면에서 내 BigPlan(PassiveLine 라인 / EpicHuntAndPoke / None)을 고른다
- 0.5.8 logic(앞 1500자):
```
ctx = data.context; if !spawn_epic(ctx.tutorial) (태그∈1..6) → return None                                   [L318-319]
champ = cache.player_champion[team][position].unwrap()                                                  [L322]
match phase { Hunt(3) → return EpicHuntAndPoke(default) [L325]; Setup(1) → 아래; _ → return None [L497] } [L324]
strategy = player.strategy(rnd, game)                                                                     [L327]
opposite_object_pressure = v23_enemy_object_pressure(player, data, Serpen)                                [L329]
if opposite_object_pressure { if let Some(line) = v23_objective_setup_pressure_line(player,data,&[Top,Mid]) → return PassiveLine{line} } [L330-332]
match strategy.object_buildup {                                                                            [L337]
  Split(pos) [L339]: if pos == my position { if !v25_objective_splitter_should_join_contest(version,player,data,Morgard) → return PassiveLine{Bottom} [L340-341] } → Gather 로 진행
  Flexible [L347]: camp = map.camp_pos(Morgard, team==0)
    can_reach = Σ_{p∈0..5, 적 챔프 e=player_champion[1-team][p] 존재} [                                    [L350-359]
        last_pos = team_plan.vision.last_visible_pos[p]; d = distance(last_pos, camp).sat_sub(150000)      [L352-353]
        can_move = (game.tick() − vision.last_checked_ticks[p]).sat_sub * e.move_speed                    [L354-355]
        e.hp*100/e.max_hp > 49 && can_move >= d && !blackboard[1-team].is_recent_visible(g
```

## `e07430` → `ee6d40` tower_dive_is_viable  (2848→4164B · Δ+1316)
- 명령 596→835 · 정렬 501 · exe 판정 ⚠정렬 불가(미확정) · 정규화 2 · 블록이동 88 · 스택슬롯 162 · 콜리 주의 5
- 잔여 구조: 구 95명령(mov|r12, qword ptr [r9]…) / 신 334명령(mov|r14, r9…) 짝 없음 ; e07998 피연산자 형 ; e079d6 피연산자 형 ; e07a13 피연산자 형 ; e07ee4 피연산자 형 ; e07ef9 피연산자 형
- 잔여 분기: e07498 → e07617/ee6ea0 ; e075fd → e0761e/ee714c ; e07619 → e07e1a/ee7200 ; e07641 → e07658/ee71a6 ; e07656 → e076c0/ee6f8f
- 잔여 소형 즉치: 0xe07e79:0x2→0x3
- 잔여 즉치: 0x1f8→0x238 · 0x1f8→0x238 · 0x2→0x3
- 잔여 변위: r9+0x660→0x668 · rax+0x640→0x668 · rax+0x660→0x8 · r15+0x680→0x4a0 · r15+0x4a0→0x680 · r15+0x5c8→0x438
- 데이터: e0762c movaps 16B 01000000000000000200000000000000→01000000000000000300000000000000 ; e07649 movaps 16B 01000000000000000300000000000000→01000000000000000400000000000000 ; e07660 movaps 16B 01000000000000000400000000000000→01000000000000000200000000000000
- 콜리 주의: c9a790→e06ec0 변경 ; ca2ad0→e0ff10 불일치(mig060 는 e0bd00) ; ca2c30→e0fda0 불일치(mig060 는 e0ba20) ; e06df0→ee5e80 미지(+1222B) ; eb82d0→eda920 변경
- 정규화 흡수: 오프셋표 [r8+930→a00] · 오프셋표 [r8+9c0→a90]
- 0.5.8 명세 #70 `fight_model__tower_dive_is_viable` src game-ai\src\plan_legacy\old\fight_model.rs:935 · one_line: 타워 다이브 실행 가능 판정 — 타워 포함 내 사망틱 vs 대상 사망틱(+탈출여유) 개인 레이스, 실패 시 팀모델(resolve_fight_stake) 로 재판정
- 0.5.8 logic(앞 1500자):
```
fn tower_dive_is_viable(version, rnd, player, data, team_plan, target, team_model, debug) -> bool
// L945
let champ = cache.player_champion[player.team][player.position] else { return false };
let tps = data.context.setting.tick_per_second;          // L948
let enemy_team = 1 - player.team;                          // L951

// L951~957 near_enemies (closure#0, aux m10:56215~): 적 챔피언 e 중
//   r = max(max_range_cached(data,champ,e), max_range_cached(data,e,champ));
//   dist_sq(e,champ) <= (r+30000)^2                                  (L953, ugt 면 탈락)
//   && blackboard[1-player.team].is_recent_visible(game, ctx, player, e)   (L954)
//   && !is_ignored_well_enemy(version, player, data, e, with_declared_dive=true)  (L955, 인라인 899~903)
//        v>=2: ignored = (e.team==Team::Player(enemy_team)) && is_enemy_well_danger(version, player, e.x, e.y)
//        v<=1: ignored = ((e.team==Player(enemy_team)) && is_enemy_well_danger(..)) || is_unreasonable_tower_dive_enemy(_, player, data, e, true)
// L959~962 near_allies (s_0/s0_0, aux m01:25361~): pos in 0..5 중
//   team_plan.ally_battle_stop_tick[pos].is_none() && player_champion[my][pos]=Some(c) && dist_sq(c,champ) < 120000^2+1  → c  (champ 자신 포함 가능)
// L969~972 nearest_enemy_tower =
//   cache.iter_towers_without_nexus(enemy_team).min_by_key(|t| dist_sq(t,target))   // s1_0 인라인
//     .filter(|t| t.distance(target) <= Effect::range(t.attack_effect.unwrap(), caster=t, target))   // L970~972 · ugt 면 None
//   where range = ae.range + 15
```

## `e83390` → `f2caf0` LineDefenseSubPlan::action_candidates  (24820→26678B · Δ+1858)
- 명령 4985→5383 · 정렬 4939 · exe 판정 ⚠정렬 불가(미확정) · 정규화 25 · 블록이동 82 · 스택슬롯 1365 · 패닉스텁 재배열 4 · 콜리 주의 13
- 잔여 구조: 구 46명령(mov|rax, r11…) / 신 444명령(mov|r12, qword ptr [r15 + I]…) 짝 없음 ; e83580 피연산자 형 ; e83584 피연산자 형 ; e84b39 피연산자 형 ; e84b7e 피연산자 형 ; e84fbe 피연산자 형
- 잔여 분기: e8372f → e8378d/f2cf37 ; e83761 → e83786/f2cf37 ; e838e4 → e839b4/f2d139 ; e838f1 → e839b4/f2d139 ; e84b1f → e84b28/f2e2e8
- 잔여 소형 즉치: 0xe87912:0x0→-0x1
- 잔여 즉치: 0x5e8→0x628 · 0x0→-0x1 · 0x5e8→0x628
- 잔여 변위: rdx+0x190→0x1c0 · rdx+0x1c0→0x1a0 · r13+0x660→0x568 · rcx+0x450→0x480 · rsi+0x668→0x660 · r11+-0x6→-0x5 · rax+0x10→0x1f0
- 데이터: e84e9a movdqa 16B 01000000000000000200000000000000→01000000000000000400000000000000 ; e84ec9 movdqa 16B 01000000000000000300000000000000→01000000000000000200000000000000 ; e84ef7 movdqa 16B 01000000000000000400000000000000→01000000000000000300000000000000
- 콜리 주의: 31a01a3→381e0b3 미지 ; 31a37c0→e706b0 미지(+1699B) ; 6d4d0→6ed50 미지(pdata 밖 thunk) ; c8c8f0→df9110 미지(+7B) ; c8d140→df9980 미지(+0B) ; c91770→dfddb0 변경
- 정규화 흡수: 오프셋표 [rax+930→a00] · 오프셋표 [rax+9c0→a90] · 오프셋표 2e8→5c8 · SmallActionPlay 태그 8→7 · 오프셋표 2e8→5c8 · 오프셋표 [rax+928→9f8] · SmallActionPlay 태그 e→d · SmallActionPlay 태그 e→d
- 0.5.8 명세 #196 `line_defense__LineDefense__action_candidates` src game-ai\src\plan_legacy\sub_plan\line_defense.rs:440 · one_line: 라인 수비 서브플랜의 행동 후보 목록 생성 루트(vtable 진입). 구판 후보(action_candidates_old: 위험시 RunAway 조기귀환→기본 포지셔닝→아군타워 곁 AroundRunAway/RunAway→전투·라인미니언·소환수공격·적타워공격·구조물스킬)에 아군지원 전투 후보를 합치고, 뒤 배치에서 최근접 적 타워 커버 판정(L457)·retain 필터(L466)·판단 정확도 무작위 절삭(L736~762)·v46 이동 후보(L766)·v47 시즈 스탠스(L774)·retain/score/get_input 재평가(L788~871)를 거쳐 sret Vec<SmallActionPlay> 로 돌려준다.
- 0.5.8 logic(앞 1500자):
```
// line_defense.rs:0~454 (배치 D)
// 사장 코드: reach.py(version=2 · gamemode=0) 결과 이 함수는 사장 블록 0 · 사장 호출부 0 — NA 봉인 대상 없음.
// 소스 줄 0 루트 블록(%171 %291 %320 %345 %672 %908 %944 %956 및 %1268~%2699 의 cleanupret/unreachable)은 unwind 잡음이라 판정 없음.
//
// ── L440 진입. version(%2)을 %131 에 spill. team_plan(%7)은 전 함수 미사용.
// ── L441 let _t_ld = ProfTimer::start(92)  // prof::ENABLED(원자 로드)==0 이면 None(+16=-1). 계측 전용.
//
// ── L442 let old_actions = self.action_candidates_old(version, rnd, player, data, parameter)   // 통째 인라인(line_defense.rs:881~938, 936 IR줄)
//   [881~882] let bump = data.context.pool; let mut res = Vec::new_in(bump)   // %78: ptr=8 · bump · cap=len=0
//   [885] let _t = ProfTimer::start(87)
//   [886] let team = player.info.team(+0x930); assert team<2 (panic_bounds_check 2);
//         let champ = data.cache.player_champion[team][player.info.position.as_index()(+0x9c0)].unwrap()   // None → unwrap_failed
//   [889~895] let has_non_target_action_range = data.cache.iter_champions(1-team)   // player_champion[1-team] 5슬롯, None 건너뜀
//        .any(|c| nontarget_windup_perceived(version, player, data, c)      // ★먼저 호출됨(invoke 21616)
//                 && c.ty@0x68 == Champion(13)
//                 && c.is_in_skill()   // entity.rs:1572 인라인: match c.action_state@0x70 {
//                        //   4 Skill  → c.skill_effect(+0x4c8).as_ref().unwrap()   (casting@0x4f8 == -1 → unwrap 패닉)
//                        //   5 Skill2 → c.skill2_effect() = if c.level(+0x5c8) > 2 { &c.skill2_ef
```
