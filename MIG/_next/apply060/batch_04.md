# apply060 batch_04.md — 반영(logic_060) 브리핑 · 함수 9

★**version = 3 런타임 확정(09-17)** — v<3/v2 경로는 명세에서 제외(사장). 태그/오프셋/vt 슬롯 = `spec_patch_060.md` §A(dispcheck 추가 행 포함). 출력 형식·절차 = `mkapply060.py` 도크스트링. 출력 = `_next/apply060/out/<old>.md`.

RE 정본 파일: 2026-09-17_r20_변경99_배치B_14_디컴대조_원문.md · 2026-09-17_r20_변경99_배치E_14_디컴대조_원문.md

### `d2c5d0` → `ecb2b0` PassiveLinePlan::sub_plan (i=56 · ⚠변경·다건(r20))
- 0.5.8 src: `game-ai\src\plan_legacy\old\passive_line.rs:848` · one_line: 수동 라인전 서브플랜 선택 — Recall / LineSafe / LineWait / LineDefense{style,line,action_type} 중 하나를 낸다
- 0.6.0 판정: **다건** · 패치 요지: 6건: +0x117→LineWait · 갱크 게이트 bb Vec · +0x119 action_type 0 · 라인 매칭 이중모드 · Recall 앞 f1f8d0/ec6dc0 · style = `!aggr && (pos==1 || def)`
- RE 정본: `2026-09-17_r20_변경99_배치E_14_디컴대조_원문.md` · 절: `2026-09-17_r20_변경99_배치E_14_디컴대조_원문.md` §10
- sig: `fn(&game_ai::plan_legacy::old::PassiveLinePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> game_ai::plan_legacy::sub_plan::SubPlan`
- consts: [{"value": 5, "src_line": 850, "meaning": "SubPlan 태그 5 = Recall (L850·L1004·L994 세 곳) · 오라클 실행 확증(17차 배치D: 오라클 실행 확인(_verify17/D/oracle/D17_o1.rs · 케이스당 프로세스 1개 · D17_o1_case{0..10}.out · setting_ok=true · 11/11 예측 일치): C0 in_recall(+0x110)=1 → tag 5 Recall(RecallSubPlan))", "kind": "태그", "ev": 2}, {"value": 3, "src_line": 858, "meaning": "SubPlan 태그 3 = LineSafe · 오라클 실행 확증(17차 배치D: 오라클 실행 확인(_verify17/D/oracle/D17_o1.rs · 케이스당 프로세스 1개 · D17_o1_case{0..10}.out · setting_ok=true · 11/11 예측 일치): C2 v46_flee=1·cover=0·acute=0 → tag 3 LineSafe{line: Bottom}(+8=2))", "kind": "태그", "ev": 2}, {"val
- 0.5.8 logic 전문:
```
fn sub_plan(&self, version, rnd, player, data, team_plan, debug) -> SubPlan

[L849] if self.in_recall(+0x110) { [L850] return Recall }
[L851] if self.v46_flee(+0x112) && !self.v46_flee_cover(+0x115) {
[L852]   if self.v46_flee_acute(+0x113) { [L854] return LineWait{line: self.line} } else { [L858] return LineSafe{line: self.line} } }

[L863] gank_or_dive_here = team_plan.objective(+0x41f) ∈ {Gank(8), Dive(9)} && objective.line(+0x420) == self.line
[L864] line = self.line
[L867] if !gank_or_dive_here {
[L868]   if let Some(p) = self.check_bot_lane_2v1(player, data) { return p } }   // ── 인라인 (passive_line.rs:1018~1041)
       //  [L1019] if line != Bottom(2) → None
       //  [L1020] tick = game.tick()/*vtable+0x28*/; if !context.is_line_phase(tick) → None   // 초반(첫 에픽 스폰 30초 전)에만
       //   ★인라인 GameContext::is_line_phase(&self, tick)[runner.rs:397~399, pub] = !self.tutorial(+0x38).spawn_epic() || self.setting(+0x8).is_line_phase(tick)
       //     TutorialType::spawn_epic()[runner.rs:262~263] 자리의 switch = 태그 ∈{0 None,5 MidBottom,7 Line,8 Total} → 시간 게이트, 그 외(1,2,3,4,6) → 게이트 없이 통과
       //     GameSetting::is_line_phase(&self, tick)[setting.rs:702~704, pub] = tick < epic_jungle.first_spawn_tick(+0x8a8).saturating_sub(tick_per_second(+0x12f8)*30)
       //  [L1021] if position(+0x9c0) <= 2 → None      (Bottom/Support 만)
       //  [L1023] partner_pos = position==3 ? 4 : 3
       //  [L1024] (lx,ly,rx,ry) = map.fountains[team](+0x6d70)
       //  [L1025] partner = cache.player_champion[team][partner_pos]
       //  [L1027] partner_absent = partner.is_none() || partner.is_in_return()(ty==13 && action_state==1) || (lx<=p.x<=rx && ly<=p.y<=ry)
       //          if !partner_absent → None
       //  [L1031] enemies_on_lane = enemy champs .filter(|c| is_near_line(context, c.x, c.y, Bottom) && bb[1-team].is_recent_visible(game, player, c)).count()
       //  [L1035] if enemies_on_lane < 2 → None
       //  [L1037] champ = cache.player_champion[team][position].unwrap(); [L1038] hp_ratio = champ.hp*100/champ.stat_cached.hp
       //  [L1039] wave_pushed = bb[team].bottom_minion_state.from_mid(+0x60) < 1000
       //  [L1041] Some(if hp_ratio > 50 || wave_pushed { LineSafe{line: Bottom} } else { Recall })

[L884] is_gank_target_line = objective == Gank(8) && objective.line == line
[L888] if is_gank_target_line {
[L889]   champ = player_champion[team][position].unwrap(); [L890] jungler = player_champion[team][Jungle(1)]
[L891]   jungler_ready = jungler.map_or(false, |j| {
[L892]     nearby  = distance_sq(j, champ) < 40000000001            // < 200000^2+1
[L893]     in_bush = map.bushes(+0x1c98)[min(j.y/32000,29)][min(j.x/32000,29)] != 0
[L894]     hidden  = !game.is_visible(1-team, j.id)                 // vtable 0xf8
[L895]     nearby && in_bush && hidden })
[L899]   line_style = position==Jungle(1) ? Defensive(1) : Aggressive(0)
[L906]   action_type = if jungler_ready { Normal(1) } else {
[L911]     from_mid = bb[team].<line>_minion_state.from_mid(+0x10)
[L912]     from_mid < 2001 ? Normal(1) : Pull(0) }
       } else {
[L899]   line_style = position==Jungle(1) ? Defensive(1) : Aggressive(0)
[L918]   champ = player_champion[team][position].unwrap()
[L919]   has_near_enemy_champion = enemy champs .any(|c| distance_sq(c, champ) < 22500000000 && bb[1-team].is_recent_visible(game, player, c))
[L922]   action_type = if has_near_enemy_champion { bb[team].<line>.minion_power(+0x18) < 0 ? Push(2) : Pull(0) } else { Push(2) }
       }

[L935] front_minion = bb[team].<line>_minion_state.front_minion (Option<usize>).and_then(|id| game.get_entity_by_id(id))   // vtable 0x1f0
[L938] if front_minion.is_none() → [L1013] return LineDefense{style: line_style, line, minion_action_type: action_type}
[L939] champ = player_champion[team][position].unwrap()
[L940] can_near_enemy = team_plan.can_near_enemies_range(version, rnd, player, data, fm.x, fm.y, 150000, debug).len()   // bumpalo Vec, +0x18
[L942] near_allies = ally champs .filter(|c| distance_sq(c, champ) < 22500000000 || distance_sq(c, fm) < 22500000000).count()   // 자기 자신 포함
[L945] nearest_tower = <line>_tower[team].or(<line>_tower2[team])
[L946]   .or(cache.twin_towers[team].iter().min_by_key(|t| distance_sq(t, LineType::get_start_position(&self.line, setting, team))))   // ★eager: min_by_key 는 항상 계산(aux m12.ll)
[L952]   .unwrap_or(cache.nexus[team].unwrap())
[L958] nexus = cache.nexus[team].unwrap()
[L959] if distance_sq(fm, nexus) < distance_sq(nearest_tower, nexus) → return LineDefense{..}     // 전방 미니언이 타워보다 안쪽
[L960] if !(near_allies < can_near_enemy && distance_sq(fm, nearest_tower) > 28899999999) → return LineDefense{..}
       // 이하: 열세 && 전방 미니언이 타워에서 170000 이상 떨어짐
[L962] minion_diff = bb[team].<line>.minion_count(+0x20)
[L968] is_object_far_line = match team_plan.objective {
[L969]   Some(Morgard(0)) => line == Bottom, [L971] Some(Serpen(1)) => line == Top,
[L972]   _ => if moba.map_or(false, |m| m.epic.live_list.len(+0x1a8) != 0) { [L973] line == Bottom }
[L974]        else { moba.map_or(false, |m| m.serpen.live_list.len(+0x1d8) != 0) && line == Top } }
[L980] giveup_object = match line {
[L981]   Top    => moba.and_then(|m| m.serpen.live_list[0] → get_entity_by_id).map_or(false, |s| s.is_visible_from(champ)),   // champ.team Neutral ⇒ true / visible_state[champ.team]==Visible
[L984]   Bottom => moba.and_then(|m| m.epic.live_list[0] → get_entity_by_id).map_or(false, |mg| mg.is_visible_from(champ)),
         Mid    => false }
[L990] if is_object_far_line {
[L993]   if can_near_enemy > 3 || giveup_object {
[L994]     if minion_diff > 2 { return Recall } else { [L997] return LineWait{line} } }
         return LineDefense{..}
       } else {
[L1003]  if let Tower(info) = nearest_tower.ty (+0x68 == 2) { [L1004] if info.nearest_enemy(+0x88).is_some() { return Recall } }
[L1013]  return LineDefense{style: line_style, line, minion_action_type: action_type}
       }
```
- logic_note: ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)

### `dfe2a0` → `ed5720` FightSituation::build (i=86 · ⚠변경·다건(r20))
- 0.5.8 src: `game-ai\src\plan_legacy\old\fight_model.rs:129` · one_line: 교전 상황 요약 구조체(FightSituation 128B) 조립 — 내 HP%/스킬·궁 준비, 전위 아군 유무, 위기 캐리, 아군/적 DPS 합과 우위를 계산
- 0.6.0 판정: **다건** · 패치 요지: FightSituation 96B: endangered_carry·my_skills_ready·my_ult_ready·has_frontline_ally·net_dps 삭제
- RE 정본: `2026-09-17_r20_변경99_배치E_14_디컴대조_원문.md` · 절: `2026-09-17_r20_변경99_배치E_14_디컴대조_원문.md` §11
- sig: `fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData, usize, usize, usize, usize, usize, &game_core::Entity, usize, bool, bool, bool, bool, bool, bool, usize) -> game_ai::plan_legacy::ol`
- consts: [{"value": 13, "src_line": 145, "meaning": "EntityType 메모리태그 13 = Champion (tcxdict --enum EntityType). skill_cooldown() 인라인의 match 판별", "kind": "태그", "ev": 3}, {"value": 31, "src_line": 145, "meaning": "스킬/스킬2 쿨다운 임계(틱). `icmp ult cd, 31` — 소스가 `< 31` 인지 `<= 30` 인지는 표기 불가(외연 동일). 60tps 기준 약 0.5초", "kind": "임계", "ev": 4}, {"value": 100, "src_line": 144, "meaning": "my_hp_ratio 백분율 계수(hp*100/max_hp)", "kind": "계수", "ev": 4}, {"value": 2, "src_line": 157, "meaning": "has_frontline_ally: BattleRole 태그 < 2 = Tanker(0)|Initiator(1) (`icmp samesign ult i8 role, 2`)", "kind": "태그", "ev": 4}, {"value"
- 0.5.8 logic 전문:
```
// fight_model.rs:129~238. 판정 없이 FightSituation 을 '조립'하는 함수. prof 타이머(phase 49) 는 prof::ENABLED 일 때만.
t = player.info.team (0x930); pos = player.info.position 태그 (0x9c0)
champ = cache.player_champion[t][pos].unwrap()                         // :140 (None → unwrap_failed 패닉)
my_battle_role = get_battle_role(version, ctx, cache, player)           // :143
my_hp_ratio = champ.hp*100 / max(champ.stat_cached.hp, 1)             // :144
my_skills_ready = champ.skill_cooldown() < 31 || champ.skill2_cooldown() < 31   // :145 (비챔프 엔티티면 cooldown()=0 → true)
my_ult_ready = champ.can_ult()                                         // :146
my_dist_to_focused = dist_sq(champ, focused)                           // :149

// :150~163 has_frontline_ally
has_frontline_ally = (0..5).filter(p != pos)
   .filter_map(p → (player_champion[t][p]?, player_state[t][p]?))
   .any(|(p, ally)| get_battle_role(version, ctx, cache, ally_state) < 2 /*Tanker|Initiator*/ && dist_sq(ally, focused) < my_dist_to_focused)

// :165~191 endangered_carry — 첫 매치에서 중단(find_map)
endangered_carry = None
for p in 0..5 { if p == pos continue;
   ally = player_champion[t][p]?; ally_state = player_state[t][p]?;
   role = get_battle_role(version, ctx, cache, ally_state); if (role & 6) != 2 continue;   // BaseAttacker|SkillCaster 만
   enemies: bumpalo Vec<&Entity> = player_champion[1-t].iter().flatten().filter(|e|          // closure$0 (aux m10:54399)
        dist_sq(e, ally) <= (max_range_cached(data, e, ally) + 30000)^2                       // :175
        && !is_ignored_well_enemy(version, player, e)   /* 인라인: e.team==Player(1-t) && is_enemy_well_danger(version, player, e.x, e.y) */  // :176
        && blackboard[1-t].is_recent_visible(game, player, e))                                   // :177
   die_tick = check_kill_die_tick(version, rnd, data, judger=ally_state, focus=ally, enemy=&enemies, towers=&[] /*빈 Vec*/, debug)   // :180
   if die_tick < tps*2 { endangered_carry = Some(ally.id); break }                              // :182~183
}

// :194~216 ally_dps_sum — ★자기 자신 제외 없음(p==pos 도 포함, dist 0)
ally_dps_sum = 0
for p in 0..5 { ally = player_champion[t][p]?;
   if dist_sq(ally, champ) >= 14400000001 continue;                    // :200  (≤120000 만)
   nearest_enemy_champ = player_champion[1-t].iter().flatten()          // :203~206 (첫 후보는 본체 인라인, 나머지는 aux m12:15597 폴드)
        .filter(|e| !is_ignored_well_enemy(version, player, e) && blackboard[1-t].is_recent_visible(game, player, e))   // closure$5 :204~205
        .min_by_key(|e| dist_sq(e, ally))                              // closure$6 :206 (동점이면 앞쪽 유지)
   if let Some(ne) = nearest_enemy_champ {
      team_plan.v54_fs_pairings += 1 (atomic)                          // :207 계측
      if !blackboard[1-t].is_recent_visible(game, player, ne) { team_plan.v54_fs_unseen_picks += 1 }   // :209 계측(필터를 통과했으므로 사실상 0)
      ally_dps_sum += fight_dps(version, ctx, attacker=ally, enemy=ne)   // :211
   }
}

// :218~223 enemy_dps_sum
enemy_dps_sum = 0
for enemy in cache.iter_champions(player_champion[1-t]) {
   if dist_sq(enemy, champ) >= 22500000001 continue;                  // :219 (≤150000 만)
   if is_ignored_well_enemy(version, player, enemy) continue;          // :220 인라인(TeamType 0 + is_enemy_well_danger)
   if !blackboard[1-t].is_recent_visible(game, player, enemy) continue; // :221
   enemy_dps_sum += fight_dps(version, ctx, attacker=enemy, enemy=champ)   // :222
}
team_dps_advantage = ally_dps_sum - enemy_dps_sum                     // :226 (i64)

write FightSituation { endangered_carry, 패스스루 인자 11개, focused_id=focused.id, my_hp_ratio, team_dps_advantage, ally_dps_sum, enemy_dps_sum, my_skills_ready, my_ult_ready, has_frontline_ally, my_battle_role }   // :228
// prof 타이머 drop(:238): PHASE_NANOS[49] += 경과ns, PHASE_CALLS[49] += 1 (ENABLED 일 때만)
```
- logic_note: ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)

### `dd5db0` → `f0cfe0` TeamPlan::v24_objective_setup_should_check_camp (i=89 · ⚠변경·한 줄(r20))
- 0.5.8 src: `game-ai\src\plan_legacy\team_plan\objective_discipline.rs:88` · one_line: [v24] Morgard/Serpen Setup 태세에서 '내가 캠프를 직접 확인하러 가야 하는가' — 라인 압박 완료·건강한 같은-편 아군 충분·내가 checker 로 뽑힘일 때, 적이 치는 중이거나 캠프가 최근에 안 보였거나 숨은 건강 적이 닿을 수 있으면 true
- 0.6.0 판정: **한 줄** · 패치 요지: L91 게이트: 3e4!=2 → camp 5 && 3e0==1 → [2,1] · 404!=2 → camp 4 && 400==1 → [0,1] · 둘 다 2 → cd5/cd6
- RE 정본: `2026-09-17_r20_변경99_배치E_14_디컴대조_원문.md` · 절: `2026-09-17_r20_변경99_배치E_14_디컴대조_원문.md` §9
- sig: `fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, game_core::JungleType) -> bool`
- consts: [{"value": 4, "src_line": 91, "meaning": "JungleType 태그 4=Morgard (target switch, m09.ll:19876; L137 `camp == Morgard` 도 동일 값)", "kind": "태그", "ev": 4}, {"value": 5, "src_line": 91, "meaning": "JungleType 태그 5=Serpen", "kind": "태그", "ev": 4}, {"value": 0, "src_line": 91, "meaning": "team_plan.rs:258 take_setup_like: MainObjective 태그 0=Morgard (camp Morgard 일 때 요구)", "kind": "태그", "ev": 4}, {"value": 1, "src_line": 91, "meaning": "MainObjective 태그 1=Serpen (camp Serpen 일 때) · 그리고 ObjectPhase 태그 1=Setup (+0x420 == 1, 두 camp 공통). 본문 `shl 1` 은 별개(tps*2 항목)", "kind": "태그", "ev": 4}, {"value": -1, "
- 0.5.8 logic 전문:
```
fn v24_objective_setup_should_check_camp(&self, _version, player, data, goal_data, camp: JungleType) -> bool
  team = player.info.team; enemy = 1 - team; ctx = data.context; cache = data.cache
  // L91: 태세 게이트 — take_setup_like(camp) (team_plan.rs:258)
  //   camp==Morgard(4): self.objective 태그==0(Morgard) && phase(+0x420)==1(Setup)
  //   camp==Serpen(5) : self.objective 태그==1(Serpen)  && phase==1(Setup)
  //   그 외 camp: false
  if !take_setup_like { return false }
  // L95: v24_objective_setup_relevant_lanes_ready(player, data, camp) (인라인)
  //   lanes = camp==Morgard ? [Top(0),Mid(1)] : [Bottom(2),Mid(1)]   (anon.190 = \00\01 / anon.191 = \02\01)
  //   ready = v23_objective_setup_pressure_line(player, data, &lanes, 2).is_none()   (반환 i8 == -1)
  if !ready { return false }
  // L99~106: 캠프 쪽 절반에 있는 건강한 아군 수
  side(c) = camp==Morgard ? is_top_side(ctx,c.x,c.y) [= height - y < x] : is_bottom_side [= height - y > x]   (map_regions.rs:24 / :30)
  healthy_side(c) = c.hp*100/c.stat_cached.hp > 39 && side(c) && is_near_mid_line(ctx, c.x, c.y)
  healthy_side_allies = cache.iter_champions(team).filter(healthy_side).count()      // L100~101, [team] 5칸
  N = player_count(ctx) 인라인 → tutorial TopSolo/MidSolo/JungleOnly: 1, 그 외: 2
  if healthy_side_allies < N { return false }     // L106 (samesign ult)
  // L110~111
  camp_pos = ctx.map.camp_pos(camp, team == 0)
  my_idx = player.info.position 태그(i32)
  if cache.player_champion[team][my_idx].is_none() { return false }
  // L123~132: checker 선정 — 같은 술어를 만족하는 아군 중 (캠프 거리 + 역할 가중) 최소
  role_bias(i) = match Position::from_index(i) { Jungle=>0, Support=>20000, Mid=>40000, Top|Bottom=>60000 }
  checker_position = iter_champions(team).enumerate().filter(|(_,c)| healthy_side(c))   // L117~120 closure#2
                      .min_by_key(|(i,c)| distance(c.x,c.y, camp_pos).saturating_add(role_bias(i)))   // L124~130 closure#3
                      .map(|(i,_)| i)
  if checker_position != Some(my_idx) { return false }   // L132 (None 도 false)
  // L137~142
  target_object = (camp != Morgard)     // Morgard→false, Serpen→true 로 5번째 인자
  if is_object_being_taken_by_enemy(player, data, goal_data, self, target_object) { return true }
  // L146~155
  tick = game.tick()
  camp_last_visible_tick = camp==Morgard ? self.obj_spawn.epic_camp_last_visible_tick : self.obj_spawn.serpen_camp_last_visible_tick
  camp_recently_checked = camp_last_visible_tick + tps*2 >= tick
  camp_visible = game.is_visible_cell(team, camp_pos.x/32000, camp_pos.y/32000)
  if !(camp_recently_checked && camp_visible) { return true }
  // L160~173 (aux m09.ll:2974): 숨은 건강한 적이 마지막 관측 이후 캠프까지 이동할 수 있었나
  return (0..5).filter_map(|p| cache.player_champion[enemy][p].map(|c| (p,c))).any(|(p,c)| {
     c.hp*100/c.stat_cached.hp >= 50                                   // L162 (IR: <50 이면 skip)
     && !data.blackboard[enemy].is_recent_visible(game, player, c)       // L165
     && { last_pos = self.vision.last_visible_pos[p];                     // L169
          d = distance(last_pos, camp_pos).saturating_sub(150000);        // L170
          can_move = tick.saturating_sub(self.vision.last_checked_ticks[p]) * c.stat_cached.move_speed;  // L171~172
          can_move >= d }                                                 // L173 (IR: can_move < d 이면 skip)
  })
```
- logic_note: ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)

### `e083c0` → `ee8450` resolve_fight_uncached (i=207 · ⚠변경·한 줄(r20))
- 0.5.8 src: `game-ai\src\plan_legacy\old\fight_model.rs:378` · one_line: 교전 예측 순수 코어: 아군·적 각 ≤5명의 EHP·DPS·CC·도착틱을 만들어(오판 노이즈 포함) 6초 창을 틱 단위로 전개하고 승패 라인(Commit/CommitAfterJoin/Disengage/Hold)·net_value·focus/soaker/rescue 를 FightPrediction(64B) 으로 낸다
- 0.6.0 판정: **한 줄** · 패치 요지: bias:i8 인자(judge_acc 뒤) · committed==-1: thr=unit*(1-bias) · 그 외 thr=unit*(1-min(bias,0)) · bias 0 이면 동치
- RE 정본: `2026-09-17_r20_변경99_배치E_14_디컴대조_원문.md` · 절: `2026-09-17_r20_변경99_배치E_14_디컴대조_원문.md` §12
- sig: `fn(usize, &game_core::OperationData, &game_core::Entity, &[&game_core::Entity], &[&game_core::Entity], i8, std::option::Option<&game_core::Entity>, usize, &[i64], i64) -> game_ai::plan_legacy::old::FightPrediction`
- consts: [{"value": 1, "src_line": 392, "meaning": "version > 1 (icmp ugt) — 시드 생성 분기. ≤1 = 틱 버킷 시드(NA · version<2 전용) / >1 = 집합 해시 시드. 표기 불가: `>1` 과 `>=2` 외연 동일 (qcspec shl 경고 = `shl i64 %74, 1`/`shl nuw nsw i64 %8, 3` 과 문자 겹침 · 이 값은 비교 상수 — 접힌 `tps*2` 는 별도 항목 folded_from=2) · 오라클 실행 확증(26차 배치D: 오라클 실행 확인(26차 D · o207.exe 85/85 MATCH · resolve_fight(hidden) 진입 → resolve_fight_full(len>8 우회/캐시 미스) → uncached · 독립 재구현 대조) — version=2 vs 1/0 케이스(V1~V1e·V0) 에서 시드 분기가 갈리고 각각 재구현과 일치)", "kind": "임계", "ev": 2}, {"value": 3, "src_line": 381, "meaning": "FightLine::Hold 메모리 태그(tcxdict --enum FightLine: idx3 di
- 0.5.8 logic 전문:
```
// fight_model.rs:0~464 (배치 K)
// 인자: version, data(&OperationData → %2=data.cache · %3=data.context), champ, near_allies(&[&Entity]), near_enemies(&[&Entity]), committed_dir(i8), tower(Option<&Entity>), judge_accuracy, arrivals(&[i64]), baseline(i64) → sret FightPrediction(64B)
// 콜리 계약(자식 명세 별도): self_sustain_in_window(version,&GameContext,&Entity,horizon)->i64(창 내 자기 힐+실드 1회분) · available_cc_in_window(version,&Entity,horizon)->i64(창 내 CC 틱 합) · fight_dps(version,&GameContext,att,tgt)->i64 · expected_dps(&GameContext,tower,tgt)->i64 · error_ratio_noise(&mut NoiseRng,j)->i64 (m04.ll:49631 본문 확인: e=(1000-j)/20(udiv) · state+=0x9E3779B97F4A7C15 · splitmix64 믹스 · mulhi(mix, 2e+1) - e + 100 ⇒ [100-e, 100+e] 균등 정수 백분율)

L379: let context = data.context;                                             // %3 (승격 슬롯)
L380: if near_allies.is_empty() || near_enemies.is_empty() {                // %40 = len_a==0, %41 = len_e==0, or
L381:   return FightPrediction { focus_target: None, soaker: None, rescue_ally: None, net_value: 0, line: Hold(3), line_absolute: Hold(3) };  // 0x0/0x10/0x20 ← 0, 0x30 ← 0, 0x38/0x39 ← 3 · 페이로드/패딩 미기록 → 배치 L(줄 521) ret
}

// ── 시드 (L392~403) ──
L392: let seed: u64 = if version > 1 {                                       // %58 = icmp ugt %1, 1 (reach: 항상 true 로 접힘)
L394:   let mut set_h: u64 = 0;
L394:   for a in near_allies.iter().take(5) {                                // %97 루프(카운트 5↓ · end 포인터)
L395:     set_h ^= a.id.wrapping_mul(0x9E3779B97F4A7C15);                    // Entity+0x5c0
        }
L397:   for e in near_enemies.iter().take(5) {                               // %128 루프
L398:     set_h ^= e.id.wrapping_mul(0x517CC1B727220A95);
        }
L400:   game.seed() ^ set_h.rotate_left(17) ^ champ.id                        // vtable+0x20 seed() · llvm.fshl 17 · champ+0x5c0 (xor 순서: (seed ^ rotl) ^ id)
      } else {                                                               // NA(version<2 전용) — reach version=2 접기로 사장
L402:   let bucket = game.tick() / max(tps * 2, 1);                          // vtable+0x28 tick() · GameSetting+0x12f8 · `shl 1` · umax
L403:   champ.id ^ (bucket << 40)
      };
L392: let mut jrng = NoiseRng(seed);                                          // %38 ← seed 그대로(믹스 없음 · ai_interface.rs:177 인라인)

// ── 오판·DPS 클로저 (L405~412) ──
L405: let misjudge = |v: i64| -> i64 {
L406:   if judge_accuracy > 999 { v } else { v * error_ratio_noise(&mut jrng, judge_accuracy) / 100 }   // %93 호이스트 · sdiv 100 · 호출마다 jrng 전진
      };
L411: let dps_of = |att: &Entity, tgt: &Entity| -> i64 {
L412:   fight_dps(version, context, att, tgt)
      };

// ── 창·아군 표 (L415~425) ──
L415: let horizon_ticks: i64 = tps * 6;                                       // GameSetting+0x12f8 (version≤1 경로는 L402 에서 읽은 값 재사용)
L416: let mut our_hp = [0i64; 5]; let mut our_dps = [0i64; 5]; let mut our_alive = [false; 5]; let mut our_n = 0usize; let mut our_cc_sum = 0i64;
L418: let mut our_arrive = [0i64; 5];
L419: for (ai, a) in near_allies.iter().copied().take(5).enumerate() {         // %33 = {ptr,end,take=5,idx} · Copied::next 아웃오브라인
L420:   our_arrive[ai] = arrivals.get(ai).copied().unwrap_or(0);              // %152 = ai < arrivals.len · 5칸 경계검사(%909, take(5) 로 실질 불발)
L421:   let tgt = near_enemies.iter().copied().min_by_key(|e| { let dx=|a.x-e.x|; let dy=|a.y-e.y|; dx*dx+dy*dy });   // aux s0_0 · 첫 원소 next 뒤 fold · 동률=먼저 것
L422:   let dps = tgt.map(|t| dps_of(a, t)).unwrap_or(0);                     // near_enemies 비면 0(L380 으로 실질 불발) · fight_dps(version, context, a, t)
L423:   let ehp = a.hp + self_sustain_in_window(version, context, a, horizon_ticks);   // Entity+0x670
L424:   our_cc_sum += available_cc_in_window(version, a, horizon_ticks);
L425:   our_hp[our_n] = misjudge(ehp * 1000); our_dps[our_n] = misjudge(dps); our_alive[our_n] = true; our_n += 1;   // 노이즈 호출 순서: hp → dps · 5칸 경계검사(%953/%955)
      }                                                                       // our_n == min(len_a, 5) · ai == our_n 항상

// ── 적 표 (L427~435) ──
L427: let mut their_hp = [0i64; 5]; let mut their_dps = [0i64; 5]; let mut their_alive = [false; 5]; let mut their_n = 0usize;
L428: let mut their_id = [0usize; 5]; let mut their_cc_sum = 0i64;
L430: for e in near_enemies.iter().take(5) {                                  // %158 루프(end 포인터 · 카운트 5↓)
L431:   let nearest_a = near_allies.iter().copied().min_by_key(|a| dist2(a, e)).unwrap_or(champ);   // aux s2_0 · (near_allies 비면 champ — L380 으로 실질 불발)
L432:   let ehp = e.hp + self_sustain_in_window(version, context, e, horizon_ticks);
L433:   their_cc_sum += available_cc_in_window(version, e, horizon_ticks);
L434:   their_hp[their_n] = misjudge(ehp * 1000); their_dps[their_n] = misjudge(dps_of(e, nearest_a));   // 노이즈 순서: hp → dps · fight_dps(version, context, e, nearest_a)
L435:   their_alive[their_n] = true; their_id[their_n] = e.id; their_n += 1;
      }                                                                       // their_n == min(len_e, 5) ≥ 1
// ⇒ jrng 소비 순서(judge_accuracy ≤ 999 일 때만): 아군 i=0..our_n (hp, dps) → 적 j=0..their_n (hp, dps) · 최대 20회

// ── CC 로 유효시간 보정 (L437~443) ──
L437: if horizon_ticks != 0 {                                                 // %168 = icmp eq %85, 0 → 건너뜀
L440:   let their_eff = max(h - min(h / 2, our_cc_sum), 1);                   // h = horizon_ticks · smin/smax(i64) · ashr 1
L441:   let our_eff   = max(h - min(h / 2, their_cc_sum), 1);
L442:   for j in 0..their_n { their_dps[j] = their_dps[j] * their_eff / h; }    // sdiv · 5칸 언롤(%303,%879~%897)
L443:   for i in 0..our_n   { our_dps[i]   = our_dps[i]   * our_eff   / h; }    // 5칸 언롤(%317,%854~%872) · our_n==0 이면 통째 생략(%316)
      }

// ── 타워·소커 (L449~455) ──
L449: let tower_dps: i64 = tower.map(|t| {
L450:     (0..our_n).max_by_key(|&i| near_allies[i].stat_cached.hp)            // aux s3_00 · Entity+0x628 · 동률=뒤의 것 · our_n==0 → None
L451:       .map(|i| expected_dps(context, t, near_allies[i]))                 // near_allies[i] 경계검사(%246)
          }).flatten().unwrap_or(0);                                          // tower None(null) 또는 our_n==0 → 0
L454: let soaker: Option<usize> = (0..our_n).filter(|&i| our_alive[i]).max_by_key(|&i| near_allies[i].stat_cached.hp);   // aux s4_0/s5_0 · 첫 alive 원소는 인라인 언롤(%263~%295), 나머지는 fold 호출
L455: let soaker_id: Option<usize> = soaker.map(|i| near_allies[i].id);         // Entity+0x5c0 · 경계검사(%329)

// ── 틱 전개 준비 (L459~464) ──
L459: let mut t: i64 = 0; let mut our_dead_v: i64 = 0; let mut their_dead_v: i64 = 0; let mut first_focus: Option<usize> = None;   // t=%28(스택 · 클로저가 &t 캡처)
L461: let horizon = horizon_ticks;
L461: loop {                                                                  // 헤더 %765 — 루프 캐리: our_dead_v(%766) · their_dead_v(%767) · 첫 반복 플래그(%768) · first_focus(%769/%770) · soaker(%771/%772). 종료 조건·역방향 엣지(%761 ← L499/L500/L501)는 → 배치 L(줄 465~521)
L463:   let arrived = |i: usize| -> bool { our_arrive[i] <= t };              // %27 = {&our_arrive, &t}
L464:   let our_dps_sum: i64 = (0..our_n).map(|i| if our_alive[i] && arrived(i) { our_dps[i] } else { 0 }).sum();   // 5칸 언롤(%398~%431) · our_dps[i] 는 루프 진입 전 호이스트 로드(%378~%382 — 루프 안에서 our_dps 불변) · our_n==0 이면 0(%331)
        // → 배치 L(줄 465): their_dps_sum 등 이어짐 (%439)
      }
// L465~521 = 배치 L (틱 전개 · 사망 처리 · focus/rescue · line 판정 · sret 기록)

// fight_model.rs:465~521 (배치 L)
// ── 전제(배치 K 산출 · 이름은 DI 변수명) ──
// 로컬 배열(스택, 길이 5): our_alive[%35: 5×bool] our_arrive[%34: 5×i64] our_dps[%36] our_hp[%37] / their_alive[%30] their_dps[%31] their_hp[%32] their_id[%29: 엔티티 id]
// our_n=%154 · their_n=%167 · horizon=%85(horizon_ticks) · t=*%28(현재 시뮬 틱) · tower_dps=%332 · 루프 소커 soaker=(tag %771, idx %772) · 초기 소커 출력용 soaker_id=(%333,%335)
// arrived(i) := !(our_arrive[i] > t)  — 463 줄 클로저(캡처 %27 = {&our_arrive, &t}), 468·480 에서 인라인
// %331 = K 플래그: `(0..our_n)` 폴드를 통째로 건너뛰는 경로(our_n == 0 로 추정 — unknown 참조). 루프 머리 phi(m10.ll:46791~46798): our_dead_v=%766 their_dead_v=%767 first_focus=(%769 tag,%770) soaker=(%771,%772) · 첫 진입 시 0/0/None/K값
// our_total(%440, 464 줄 K) = Σ our_dps[i] (i<our_n, our_alive[i] && arrived(i))

// 465: their_total = (0..their_n).filter(|i| their_alive[i]).map(|i| their_dps[i]).sum()
//   (m10.ll:45397~45451: %355 = their_n ∉ 1..=5 이면 panic_bounds_check(5,5) — 컴파일러가 their_n≥1 을 알고 do-while 로 편 것 · 실전 도달 불가)
//   %454 = their_total

// 466: te = (0..their_n).filter(|i| their_alive[i]).min_by_key(|i| their_hp[i])   ← closure#14/#15, aux m12.ll:17437 (동률이면 앞 인덱스)
//   첫 생존 원소를 본문에서 언롤(m10.ll:45500~45624: their_alive[0..4] 순차 검사, %480=첫 idx, %482=their_hp[첫])
//   → 생존 적 없음(their_n 개 전부 false) → @534 (508 줄로 break: 전멸 = 승리 정산)
//   → 있음 → %483 = fold(env %21={&their_alive, their_n, &their_hp, cur}, first_key, first_idx) · te = %483.1 (m10.ll:45663 / 46011)

// 468: ta = (0..our_n).filter(|i| our_alive[i] && arrived(i)).min_by_key(|i| our_hp[i])   ← closure#16/#17, aux m12.ll:17583
//   %331 이면 폴드 생략 → @600: net = 0 - baseline, our_unit = 0 으로 곧장 515 로(m10.ll:46264·47063)
//   언롤(m10.ll:45723~45908): i=0..4 에 대해 our_alive[i] 확인 → our_arrive[i] > t 이면 미도착으로 skip(%493 등 `icmp sgt arrive, t`)
//   → 도착한 생존 아군 없음 → 471 로
//   → 있음 → %557 = fold(env %20={&our_alive, &arrived캡처, our_n, &our_hp}, …) · ta = %557.1 (m10.ll:46027~46028, DI 이름 `ta`)

// 471 (ta 없음 분기): next = (0..our_n).filter(|i| our_alive[i] && our_arrive[i] > t).map(|i| our_arrive[i]).min()   ← closure#18/#19, aux m12.ll:17750
//   언롤(m10.ll:46086~46211: our_alive[i] && our_arrive[i] > t 인 첫 i 의 arrive 가 first, 이후 fold(env %26) 가 min)
//   → 후보 없음 → @597: 508 줄로 break(net 정산)
//   → Some(next): if next < horizon (m10.ll:46296~46297)
//        472: t = next; continue  (m10.ll:46300 store %28 · 46302 br %397 — dead/first_focus/soaker 는 그대로 유지)
//      else → 508 로 break

// 476: if t >= horizon || (our_total | their_total) == 0   (m10.ll:46031~46036 · `or i64` = our_total==0 && their_total==0 접힘 · IR 은 t>=horizon 을 먼저 평가)
//        → break → 508

// 477: if first_focus.is_none() { first_focus = Some(their_id[te]) }   (m10.ll:46305 %768 · 46378~46379 their_id[te] · bounds te<5)

// 480: soak: Option<usize> = soaker.filter(|&s| our_alive[s] && arrived(s))   (m10.ll:46318~46359: %613=soaker.is_some · our_alive[s] · our_arrive[s] > t 면 None) → (%630 is_some, %631 s)

// 481: ta_incoming = their_total + (if soak == Some(ta) { tower_dps } else { 0 })   (m10.ll:46365~46368 · %633 = soak.is_some && s==ta)

// 484: t_te = if our_total > 0 { (their_hp[te] + our_total - 1) / our_total } else { INF }   (INF = 2305843009213693951 · m10.ll:46370·46403~46407 · ceil 나눗셈, sdiv)
// 485: t_ta = if ta_incoming > 0 { (our_hp[ta] + ta_incoming - 1) / ta_incoming } else { INF }   (m10.ll:46395·46426~46430)
// 486: t_soak = if let Some(s) = soak && s != ta && tower_dps > 0 { (our_hp[s] + tower_dps - 1) / tower_dps } else { INF }   (m10.ll:46420~46442 · %368 = tower_dps>0, %369 = tower_dps-1 은 루프 밖에서 선계산 · `s != ta` 와 `tower_dps > 0` 의 소스 순서는 표기 불가)
// 487: dt = max(1, min(horizon - t, min(t_soak, min(t_ta, t_te))))   (m10.ll:46451~46461 · smin 사슬 순서 = IR 기준, 소스 괄호 순서는 표기 불가)

// 489: their_hp[te] -= dt * our_total          (m10.ll:46467~46471)
// 490: our_hp[ta]   -= dt * ta_incoming        (m10.ll:46480~46484)
// 491: if let Some(s) = soak && s != ta { our_hp[s] -= dt * tower_dps }   (m10.ll:46485~46488 %695 = !(is_some && s!=ta) → skip · 46501~46505 · ★tower_dps>0 검사 없음 — 0 이면 0 차감)
// 492: t += dt                                  (m10.ll:46495~46496 store %28)

// 493: if their_hp[te] < 1 { their_alive[te] = false; their_dead_v += their_dps[te] }   (m10.ll:46497~46514 · `<1` == `<=0` 표기 불가 · dead_v 는 죽은 유닛의 **dps** 합)
// 494: if our_hp[ta] < 1 {                                                          (m10.ll:46520~46522)
// 495:   our_alive[ta] = false; our_dead_v += our_dps[ta]                             (m10.ll:46535~46539)
// 496:   if soaker == Some(ta) { soaker = (0..our_n).filter(|i| our_alive[i]).max_by_key(|i| near_allies[i].stat_cached.hp) }   ← closure#21/#22 aux m12.ll:17885 (m10.ll:46543~46545 루프 소커(%613/%772) 와 비교 · 언롤 46595~46718 · 첫 원소 key = near_allies[i]+0x628 · 후보 없으면 soaker = None(%760=0) · 동률이면 **뒤** 인덱스)
//      }
// 498: if let Some(s) = soak && s != ta {                                              (m10.ll:46532 %695 재사용)
// 499:   if our_hp[s] < 1 {                                                            (m10.ll:46802~46805)
// 500:     our_alive[s] = false; our_dead_v += our_dps[s]                             (m10.ll:46808~46812)
// 501:     if soaker == Some(s) { soaker = 위 496 과 동일 재선정 }   ← closure#23/#24 aux m12.ll:18047 (m10.ll:46816~46819 · 496 에서 갱신된 soaker(%716/%717) 기준 · 언롤 46821~47045)
//      } }
// → 루프 머리(@761→@765, m10.ll:46781~46799)로 back-edge · 464 로 (배치 K 줄)

// ── 루프 탈출 후 ──
// 508: net = their_dead_v - (our_dead_v + baseline)   (m10.ll:45912~45913 / 45938~45939 / 46218~46219 · %331 경로는 0 - baseline 으로 접힘 46264)
// 510: our_unit = if our_n > 0 { (Σ_{i<our_n} our_dps[i]) / our_n } else { 0 }   ← closure#25 (m10.ll:45964 our_n>5 → panic · 45972~46001 합(생존/도착 무관 전원) · 47056 sdiv · %331 경로 0 = 47063 phi · 0 처리의 소스 형태(명시 if / checked_div / max(1)) 는 표기 불가)
// 515: match committed_dir {                       (m10.ll:47065 switch i8 %9)
// 516:   1  => line = if net < -our_unit { Disengage } else if net > -1 /*net>=0*/ { Commit } else { Hold }   (47075~47085)
// 517:   -1 => line = if net > our_unit { Commit } else if net < 1 /*net<=0*/ { Disengage } else { Hold }        (47080~47081 · 47119~47120)
// 518:   _  => line = if net > our_unit { Commit } else if net < -our_unit { Disengage } else { Hold }             (47071~47072 · 47124~47126)
//      }
// 520: *sret = FightPrediction { focus_target: first_focus, soaker: soaker_id(K 초기값 %333/%335 — 루프 중 교체분 아님), rescue_ally: None, net_value: net, line, line_absolute: line }   (m10.ll:47091~47105)
// 521: ret   (m10.ll:44135 @59 · 381 조기반환도 같은 ret 블록으로 합류)
```
- logic_note: ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)

### `e8aeb0` → `f34d10` LineDefenseSubPlan::calculate_score_parameter_value (i=240 · ⚠변경·한 줄(r20))
- 0.5.8 src: `game-ai\src\plan_legacy\sub_plan\line_defense.rs:396` · one_line: 라인수비 서브플랜의 ScoreParameter 가중치 세팅: 자기 attack/util_value 를 라인 스타일(공격/수비)과 HP% 로 30/50/70 중 택일, 아군 전원 50, 적 전원 50(수비)/100(공격)
- 0.6.0 판정: **한 줄** · 패치 요지: my = aggr?value : def?90 : value · param.+0x1501=true · ev = aggr?150 : def?35 : line_style?50:100
- RE 정본: `2026-09-17_r20_변경99_배치E_14_디컴대조_원문.md` · 절: `2026-09-17_r20_변경99_배치E_14_디컴대조_원문.md` §3
- sig: `fn(&game_ai::plan_legacy::sub_plan::LineDefenseSubPlan, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::ScoreParameter)`
- consts: [{"value": 2, "src_line": 399, "meaning": "player_champion 1차 길이 바운드체크 team<2 (m14.ll:29751)", "kind": "길이", "ev": 4}, {"value": 100, "src_line": 400, "meaning": "hp_ratio = hp*100/max_hp — 백분율 환산 (m14.ll:29786)", "kind": "계수", "ev": 4}, {"value": 50, "src_line": 401, "meaning": "HP% 임계 — Defensive 에선 `hp_ratio > 50`(29789), Aggressive 에선 `hp_ratio < 50`(29791) · 동시에 결과값 50 (29790/29792 select 의 참 가지) · 오라클 실행 확증(26차 배치K: 오라클 실행 확인(6케이스 전부 일치): Defensive hp 49→70 · 50→70 · 51→50 (경계 `>50`) / Aggressive 49→50 · 50→30 · 51→30 (경계 `<50`))", "kind": "임계", "ev": 2}, {"value": 70, "src_line": 401, "
- 0.5.8 logic 전문:
```
fn calculate_score_parameter_value(&self, _rnd, player, data, parameter: &mut ScoreParameter)   [line_defense.rs:396]
  line_style: bool = (self.style == LineStyle::Defensive)        // 태그 1 → true          [L397 · m14.ll:29746~29747]
  champ = data.cache.player_champion[player.info.team][player.info.position].unwrap()      [L399 · 29749~29770 · team<2 체크]
  hp_ratio = champ.hp * 100 / champ.stat_cached.hp                 // usize udiv · max_hp==0 이면 패닉   [L400 · 29774~29787]
  value = if line_style /*Defensive*/ { if hp_ratio > 50 { 50 } else { 70 } }
          else          /*Aggressive*/ { if hp_ratio < 50 { 50 } else { 30 } }             [L401 · 29789~29793]
  parameter.player.attack_value = value                                                    [L418 · 29795~29796]
  parameter.player.util_value   = value                                                    [L419 · 29797~29798]
  for p in parameter.near_allies.iter_mut()  { p.attack_value = 50; p.util_value = 50; }   [L421~423 · 29801~29847]
  for p in parameter.near_enemies.iter_mut() { let v = if line_style { 50 } else { 100 }; p.attack_value = v; p.util_value = v; }   [L426, 435~436 · 29852~29899]
  return                                                                                    [L438 · 29902]

해석: 수비 스타일은 HP 가 절반 이하로 떨어졌을 때 자기 가치를 70 으로 올리고(더 조심/보호) 적 가치는 50 으로 낮춰 본다 · 공격 스타일은 HP 가 절반 이상이면 자기 가치 30 (희생 허용) 이고 적 가치는 100 으로 본다. L402~417 · 424~425 · 427~434 는 IR 에 흔적 없음(빈 줄/주석/닫는 괄호 추정 — 미확인). gen_range 사이트 0.
```
- logic_note: ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)

### `e388c0` → `ed3dd0` SerpenCheckSubPlan::score (i=246 · ⚠변경·다건(r20))
- 0.5.8 src: `game-ai\src\plan_legacy\sub_plan\serpen_check.rs:123` · one_line: SerpenCheck(정글러 스틸 Lurk) 서브플랜의 후보 액션 점수: interaction_score + (Around 계열 대상이 살아있는 첫 세르펜이고 내 팀 시야에 안 보이면 +10), 그 외 0
- 0.6.0 판정: **다건** · 패치 요지: 디스패처 후처리: `if param.1501 && def && !aggr { if let Some((a,b))=f71fc0 { if a>0 && score>=-9998 && b<2a { score=-9999 } } }` · idx 17 ObjContest arm = f72620
- RE 정본: `2026-09-17_r20_변경99_배치E_14_디컴대조_원문.md` · 절: `2026-09-17_r20_변경99_배치E_14_디컴대조_원문.md` §4
- sig: `fn(&game_ai::plan_legacy::sub_plan::SerpenCheckSubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64`
- consts: [{"value": 10, "src_line": 125, "meaning": "SmallActionPlay 니치 구멍 assume(tag≠10). 판정 아님. ★같은 값 10 이 L128 가산으로도 쓰임(별 항목)", "kind": "센티널", "ev": 4}, {"value": 3, "src_line": 125, "meaning": "태그→idx 환산(tag-3). 판정 아님", "kind": "태그", "ev": 4}, {"value": 7, "src_line": 125, "meaning": "tag≤2 → idx 7(AroundPosition). 판정 아님", "kind": "태그", "ev": 4}, {"value": 0, "src_line": 127, "meaning": "GameMode::Moba 태그(0) — as_moba().unwrap() 검사. 그리고 live_list.len == 0 검사(get(0) None) · 오라클 실행 확증(26차 배치L: 오라클 실행 확인(o246 · 9케이스 · 케이스당 프로세스 1개 · base 는 별도 프로세스의 interaction_score · 9/9 IR 독해와 일치): serpen.live_list=
- 0.5.8 logic 전문:
```
fn score(&self, version, parameter, rnd, player, data, action, debug) -> i64   // serpen_check.rs:123 · self(move_check) 미사용
let base = interaction_score(version, rnd, player, data, parameter, action, debug);   // L124
let add = match action.get_action() {   // L125 · SmallActionPlay 태그 switch
    Around{target_id} /*Play idx 2 Around / 3 AroundHide / 10 LaneMinionPosition · payload+0x8*/ => {
        // L127: 살아있는 첫 세르펜 엔티티
        let game = data.cache.game;   // &dyn AbstractGame (+0x0 data, +0x8 vtable)
        let serpen: Option<&Entity> = game.get_game_mode() /*vtable+0x40 → GameMode{tag,ptr}*/
            .as_moba() /*game.rs:231 · tag≠0(Moba) 이면 unwrap_failed — reach: 접힌 분기(gamemode=0), 사장 호출부 NA*/ .unwrap()
            .jungle_runner.serpen.live_list /*MobaMode+0x1d0 ptr / +0x1d8 len · Vec<usize>*/
            .get(0) /*len==0 → None*/
            .and_then(|id| game.get_entity_by_id(*id) /*vtable+0x1f0*/);
        // L128
        if let Some(serpen) = serpen {
            if serpen.id /*+0x5c0*/ == target_id && !game.is_visible(player.info.team /*+0x930*/, target_id) /*vtable+0xf8*/ { 10 } else { 0 }
        } else { 0 }
    }
    _ => 0,   // Attack/Skill/Skill2 포함 전부 0 — 형제 플랜과 달리 공격 액션 가산이 전혀 없음
};
return base + add;   // L124 합산 · L141

★형제 대조(복붙 의심): 형제 4개(LineSafe/LineWait/Jungle/AttackNexus)는 champ 조회+Attack/Skill/Skill2 arm+calculate_(jungle_)action_score 골격인데 SerpenCheck 는 그 골격이 통째로 없고 Around arm 만 있다(_docs: 「commit=false(Lurk): 시야 확인 주기 기반 대기 … stale 이면 잠깐 캠프로 접근해 시야 갱신」과 부합 — 공격은 SerpenHunt 몫). is_visible 인자 순서 (team, id) 는 _gcbc g08.ll:200059 DI(self, team, id) 로 확인. `serpen.id == target_id` 뒤 `is_visible(team, target_id)` 는 `&&` 단락 — IR 상 id 비교가 먼저(블록 43→47).
rnd gen_range 사이트: 0(interaction_score 1회 전달).
version 분기 0 · debug 직접 쓰기 0 · self 읽기 0.
★exe 0xe388c0(966B) = SubPlan::score JT 디스패처(jump-table lea @e388fc)이지 이 함수 단독 본체가 아니다 — SerpenCheck::score 본체는 그 안에 **인라인**(fnprobe: 패닉 Location serpen_check.rs:127:73 @e38c6a · IND call [r15+0x40]=get_game_mode @e389c4 → [r15+0x1f0]=get_entity_by_id @e389ec → [r15+0xf8]=is_visible @e38c40 · call 0xd57540(9212B)=interaction_score @e3897f). 형제 EpicCheck::score 도 같은 디스패처에 인라인(epic_check.rs:127:71 @e38c78 — 문자 복제 형제) · Steal::score 는 TAIL-JMP 0xcbbca0(269B). ev1 프로브는 디스패처 단위로 잡아야 한다. 오라클(o246 9케이스) 은 IR 독해와 전부 일치.
```
- logic_note: ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)

### `e7b9f0` → `f28320` end_check (i=255 · ⚠변경·한 줄(r20))
- 0.5.8 src: `game-ai\src\lib.rs:1211` · one_line: 판 종료(넥서스 마무리 진입) 판정 — 선수의 game_finish 전략(Stable/Flexible/Aggressive)별로 라인 타워·생존/건강 아군 수·적 넥서스 근접 인원을 세어 bool 반환
- 0.6.0 판정: **한 줄** · 패치 요지: 프렐류드 `if v>=3 { if let Some(r)=finish_race { margin=Aggr?tps:Flex?2tps:3tps; if r.total+margin < r.min_defender_arrival {return true} } }`
- RE 정본: `2026-09-17_r20_변경99_배치E_14_디컴대조_원문.md` · 절: `2026-09-17_r20_변경99_배치E_14_디컴대조_원문.md` §8
- sig: `fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, game_core::LineType, &mut game_core::DebugFrameData) -> bool`
- consts: [{"value": 2, "src_line": 1279, "meaning": "Option<GameFinishCheckState> 니치 태그 None(+68 바이트 == 2) — L1279(m14.ll:8772)·L1311(m14.ll:10091) 두 곳. 별도로 L1326 `healthy_ally_50 < live_enemy + 2`(m14.ll:10166) 의 2, player_count 매핑값 2(First/Bottom), required_edge/required_group 2 도 같은 리터럴 · 오라클 실행 확증(27차 배치D: 오라클 o27D_ec.exe Flexible 213 MATCH + line_exists 게이트 29(예측기 미구현 · got=false 정상) + unwrap 패닉 1(champ None 기대) · DIFF 0(_verify27/D/oracle/o27D_ec.log) — 경계 실측: 아군 집결 1,200,000 포함/1,200,001 제외 · 넥서스 600,000 포함/600,001 제외 · hp 40% ok/39% 제외 · epic 1199(<tps·20) vs 1200 · minion_count 6 vs 5 · requir
- 0.5.8 logic 전문:
```
fn end_check(version, rnd, player, data, line, debug) -> bool   // lib.rs:1211
ctx = data.context; cache = data.cache; game = cache.game(&dyn AbstractGame)
L1212: if !rule_scope::line_exists(ctx, line) { return false }           // m14.ll:8736~8737, %20 phi false
L1216: strat = player.strategy(rnd, game)  (sret Strategy 24B)  ; match strat.game_finish(+0x15) {

== Stable (tag 0) → end_check_stable(player, data, line)  lib.rs:1279~1302 ==
L1279: state = build_game_finish_check_state(player, data, line)?   // None(+68==2) → return false (m14.ll:8772→%47)
L1280: required_allies = min(ctx.tutorial.player_count(), 4)   // 외연 동일 표기(표기 불가) · player_count 원값: None/Total=5 · Line=4 · First/Bottom=2 · TopSolo/MidSolo/JungleOnly=1 · MidBottom=3 (MIR runner.rs:296~304) → 접힘값 4/4/2/1/3 (phi m14.ll:8836)
L1282: if (state.line_tower_alive(raw u8 +65) != state.has_enemy_twin_tower(raw u8 +64)) || state.line_tower_alive { return false }
        // IR 문면 `icmp ne %40,%38 ; or ..., %41`(m14.ll:8838~8840). 불 대수상 line_tower_alive || has_enemy_twin_tower 와 동치. 소스 표기는 미확정(표기 불가)
L1286: if !(state.live_ally_count >= required_allies && state.healthy_ally_55_count >= state.live_ally_count && state.near_player_ally_count == state.live_ally_count && state.stable_pushed_line) { return false }   // m14.ll:8843~8849
L1298: match state.live_enemy_count { 0 => return true,                                            // m14.ll:8853
L1302:   1 => return state.near_enemy_nexus_healthy_ally_55_count >= required_allies,               // m14.ll:8858
         _ => return false }                                                                        // %65

== Flexible (tag 1) → end_check_flexible(player, data, line)  lib.rs:1225~1271 ==
team = player.info.team; enemy = 1 - team; pos = player.info.position as usize
L1225: if !(game.tick() > setting.tower_attack_disable_tick) {
L1226:   (t1, t2) = cache.tower(line, enemy)   // (top|mid|bottom)_tower[enemy], _tower2[enemy]
L1230:   if !(t1.is_none() && t2.is_none()) { return false } }   // m14.ll:8942~8948 → %620 false
L1234: enemy_nexus = cache.nexus[enemy].unwrap()   // None → unwrap_failed 패닉
L1235~1236: live_enemy_count = cache.iter_champions(enemy).count()   // player_champion[enemy] 의 Some 개수(5칸 완전 언롤)
L1237: has_enemy_twin_tower = if game.tick() > setting.tower_attack_disable_tick { false } else {
L1238:     cache.twin_towers[enemy].len() != 0 }
L1243: champ = cache.player_champion[team][pos].unwrap()   // None → unwrap_failed 패닉
L1244: near_ally = cache.iter_champions(team).filter(|a| a.distance_sq(champ) < 1440000000001).count()   // 자기 자신 포함(dist 0)
L1247: if near_ally != cache.iter_champions(team).count() { return false }   // 생존 아군 전원이 1,200,000 안에 모여야 함 (m14.ll:9433~9434)
L1253: ok_ally = cache.iter_champions(team).filter(|a| a.hp*100 / a.stat_cached.hp > 39).count()   // stat_cached.hp==0 → div_by_zero 패닉
L1254: epic_remain = game.get_game_mode().as_moba().map_or(0, |m| m.remain_epic_time(team))   // = MobaMode.epic_minion_buff_time[team]
       if epic_remain < setting.tick_per_second * 20 {
L1260:   if live_enemy_count == 0 { return true }  if has_enemy_twin_tower { return false }     // m14.ll:9647~9649 (%382: 반환값 = live_enemy==0)
       } else {
L1255:   cond = (ok_ally >= live_enemy_count.saturating_sub(1) && blackboard[team].minion_state(line).minion_count > 5) || live_enemy_count == 0
         if cond { return true }  if has_enemy_twin_tower { return false }                        // m14.ll:9675~9681 (%393: 반환값 = cond)
       }
L1266~1269: required_edge = match ctx.tutorial.player_count() { 2 => 1, 3 => 2, _ => 3 }   // 1명·4명 모드 모두 3 (m14.ll:9687~9708)
L1271: near_nexus = cache.iter_champions(team).filter(|a| a.distance_sq(enemy_nexus) < 360000000001).count()
       return near_nexus >= required_edge + live_enemy_count                                       // m14.ll:10010~10011

== Aggressive (tag 2) → end_check_aggressive(player, data, line)  lib.rs:1307~1342 ==
L1307: if !(game.tick() > setting.tower_attack_disable_tick) {
L1308:   (t1,t2) = cache.tower(line, 1-team); if !(t1.is_none() && t2.is_none()) { return false } }   // m14.ll:10079~10084
L1311: state = build_game_finish_check_state(player, data, line)?   // None → false (m14.ll:10091→%590)
L1312: required_group = min(player_count, 3)   // player_count 5(None/Total)·4(Line)·3(MidBottom) → 3 · 2(First/Bottom) → 2 · 1(TopSolo/MidSolo/JungleOnly) → 1 (phi m14.ll:10153 `[3,%569]·[2,%591]·[1,%592]`)
L1314: if state.line_tower_alive { return false }
L1318: if !(state.healthy_ally_50_count >= required_group && state.near_finish_line_healthy_ally_50_count >= required_group && state.aggressive_pushed_line) { return false }   // m14.ll:10158~10162
L1326: if !(state.live_enemy_count != 0 && state.healthy_ally_50_count < state.live_enemy_count + 2) { return true }   // 적 전멸 또는 건강 아군이 적+2 이상 (m14.ll:10165~10169)
L1334: if !state.has_enemy_twin_tower {
L1335:   if state.healthy_ally_50_count > state.live_enemy_count && state.near_enemy_nexus_healthy_ally_50_count >= required_group { return true } }   // m14.ll:10175~10178
L1340: if state.healthy_ally_50_count > state.live_enemy_count && state.has_epic_buff {
L1342:   return state.near_finish_line_healthy_ally_50_count >= min(ctx.tutorial.player_count(), 4) }   // 접힘값 4(None/Line/Total)/2/1/3 — player_count 원값은 None/Total=5 (m14.ll:10187~10210 · MIR runner.rs:302~304)
       return false
}

분기 극성 근거: %620 phi(m14.ll:10214) — false:{%275,%47,%51,%55,%65,%90,%590,%553,%593,%610,%595} true:{%62,%600,%606} 값:{%63→%64, %382→%383, %527→%530, %393→%402, %617→%619}.
rnd: 본문 gen_range 0회(strategy 에 전달만). version/debug 미사용.
```
- logic_note: ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)

### `e01c40` → `e82c00` defensive_crisis (i=3 · ⚠변경·한 줄(r20))
- 0.5.8 src: `game-ai\src\buff_value.rs:17` · one_line: target 주변 적 챔프를 추려 (2초내 죽을 위기, 곧 쓸 CC기 위협) 두 bool 을 낸다
- 0.6.0 판정: **한 줄** · 패치 요지: 6번째 인자 extra_tick · `die_imminent = death < (v>=3 ? extra_tick : 0) + tps*2` · 짝 e82c00
- RE 정본: `2026-09-17_r20_변경99_배치B_14_디컴대조_원문.md` · 절: `2026-09-17_r20_변경99_배치B_14_디컴대조_원문.md` §2
- sig: `fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, &mut game_core::DebugFrameData) -> game_ai::DefensiveCrisis`
- consts: [{"value": 1, "src_line": 22, "meaning": "enemy_ix = 1 - player.info.team — 적팀 인덱스 ★**src_line 정정 : 22**(`%21 = sub i64 1, %20` @m10.ll:33897 의 `!dbg !40704` = `buff_value.rs:22`, 인라인 없음. L21 에는 `llvm.lifetime.start` 하나뿐)(대상 챔프 배열·블랙보드 둘 다 이 인덱스)", "kind": "인덱스", "ev": 4}, {"value": 2, "src_line": 22, "meaning": "팀 배열 길이 — `enemy_ix`(=1-player.info.team)가 2 이상이면 panic_bounds_check(len=2). 즉 팀은 0/1 뿐. IR `%22 = icmp ult i64 %21, 2`(m10.ll:33899, `!dbg !40699` = simulation.rs:1905 `iter_champions` ← buff_value.rs:22)가 `player_champion[enemy_ix]` 첨자 검사다 — 같은 구문을 00·01·02 는 전부 consts 에 싣고 있는데 03 만
- 0.5.8 logic 전문:
```
fn defensive_crisis(version, rnd, player, data, target, debug) -> DefensiveCrisis

[L19] tps = data.context(+0x8).setting(+0x8).tick_per_second(+0x12f8)
[L22] enemy_ix = 1 - player.info.team(+0x930) // ★L21 은 `let` 슬롯뿐이고 이 계산은 **L22** 다 // ult 2 이면 panic_bounds_check(len=2)

[L21~26] near_enemies: Vec<&Entity, &Bump> =
 data.cache.player_champion(+0x1e0)[enemy_ix] // [Option<&Entity>;5], stride 40
 .iter().filter_map(id) // = AbstractGameWithCache::iter_champions (Some 만)
 .filter(|e| { // ★클로저 본체는 담당 범위 밖:
 // m10.ll 55868~55976 (Entity::call_mut 심)
 [L23] r = plan_legacy::old::battle::max_range(e, target)
 [L24] if Entity::distance_sq(e, target) > (r + 30000)^2 { return false }
 // distance_sq = abs_diff(x)^2 + abs_diff(y)^2, 좌표는 Entity+0x660/+0x668
 [L25] if fight_model::is_ignored_well_enemy(version, player, e) { return false }
 // = (e.team(+0x0) == TeamType::Player(enemy_ix))
 // && path_finder::is_enemy_well_danger(version, player, e.x, e.y)
 // 즉 '적 진영 샘(well) 위험구역에 서 있는 적'은 세지 않는다
 [L26] return data.blackboard(+0x10)[enemy_ix]
 .is_recent_visible(data.cache.game(&dyn AbstractGame), player, e)
 })
 .collect_in(data.context.pool(+0x0)) // bumpalo Vec::from_iter_in

[L30] if near_enemies.len(+0x18) == 0 { return DefensiveCrisis{ die_imminent:false, cc_threat:false } }
 // ⚠LLVM 이 여기서 아래 두 계산을 통째로 건너뛴다(둘 다 false 로 접힘)

// ── 출력 1: die_imminent ───────────────────────────────
[L30] die_imminent = false
[L32] tp = data.cache.player_by_champion_id(target.id(+0x5c0)) // Option<&PlayerState>, null 검사
 if tp != null {
[L33] die = fight_check::check_kill_die_tick(
 version, rnd, data,
 judger = tp, // target 을 소유한 플레이어
 focus = target,
 enemy = near_enemies.clone(), // 같은 pool 에 복제
 towers = Vec::new_in(pool), // ★항상 빈 벡터 (ptr=dangling(8), cap=0, len=0)
 debug)
[L35] die_imminent = (die < tps << 1) // = die < tps*2 (2초 이내 사망)
 }
 // tp == null 이면 die_imminent 는 false 로 남는다

// ── 출력 2: cc_threat ──────────────────────────────────
[L41] cc_threat = near_enemies.iter().any(|e| {
[L43~45] ★L42 에는 명령이 0개다. 슬롯 3칸이 **L43(skill) / L44(skill2) / L45(ult)** 에 한 줄씩 있고,
// `ty == 13` 비교는 소스에 직접 있는 게 아니라 각 `Entity::*_cooldown()`(entity.rs:1775/1791/1806) 안에 있으며
// LLVM 이 CSE 해 **L43 하나로** 접었다.
[L43] (c1,c2,c3) = if e.ty(+0x68) == 13 /*Champion*/ {
 (e.ty.skill_cooldown(+0xb8), e.ty.skill2_cooldown(+0xc0), e.ty.ult_cooldown(+0xc8))
 } else { (0,0,0) }
 slots = [ (&e.skill_effect(+0x4c8), c1),
 (if e.level(+0x5c8) > 2 { &e.skill2_effect(+0x500) } else { &NONE }, c2),
 (if e.level > 4 { &e.ult_effect(+0x538) } else { &NONE }, c3) ]
[L47] slots.iter().any(|(opt, cool)| {
[L48] cool <= tps // 1초 안에 쓸 수 있는 스킬만
 && opt.as_ref().is_some_and(|e| fight_check::effect_cc_time(version, e).is_some())
 // 니치: (*opt)+0x30 == -1 이면 None. ★소스 구문 확정(12차 배치A): `is_some() && … unwrap()` 2단이 아니라 **`as_ref().is_some_and(..)` 한 구문**이다. `dloc.py m10.ll 40903 40944` 가 인라인 사슬을 편다 — m10.ll:34172 `%112 = icmp eq i64 %111, 1` 은 `option.rs:742 as_ref ← buff_value.rs:48`, m10.ll:34144 `%98 = icmp ugt i64 %85, %17` 은 `option.rs:430 is_some ← buff_value.rs:48 ← option.rs:661 **is_some_and** ← buff_value.rs:48` 이다. 동작은 동일(단락 평가 순서도 같다)
 // 그 이펙트가 군중제어(CC) 시간을 갖는가
 })
 })
 // 세 슬롯은 IR 에서 완전히 펼쳐져 슬롯1→슬롯2→슬롯3 순으로 검사되고,
 // 하나라도 참이면 즉시 cc_threat=true 로 빠져나온다(남은 적은 안 본다).

[L53] return DefensiveCrisis{ die_imminent, cc_threat }

※ 부작용: 게임 구조체에 직접 쓰는 곳은 없다. rnd/debug 는 &mut 로 check_kill_die_tick 에
 넘어가므로 그쪽에서 변할 수 있고, near_enemies 는 bump pool 에 할당됐다가 반환 전 drop 된다.
```
- logic_note: ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)

### `defa20` → `fd7b40` is_end (i=8 · ⚠변경·한 줄(r20))
- 0.5.8 src: `game-ai\src\plan_legacy\old\epic\hunt_and_poke.rs:163` · one_line: 에픽(Morgard) 캠프 헌트앤포크 플랜을 접을지 판정 — 목표이탈·적 근접·에픽 리스폰 대기시간
- 0.6.0 판정: **한 줄** · 패치 요지: L164 `tp.3e4!=2 → true` + `tp.404==2 && tp.cd5!=0 → true` · L172 setup_like = (404==2 ? cd6 : 400)==1
- RE 정본: `2026-09-17_r20_변경99_배치B_14_디컴대조_원문.md` · 절: `2026-09-17_r20_변경99_배치B_14_디컴대조_원문.md` §10
- sig: `fn(&game_ai::plan_legacy::old::EpicHuntAndPokePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bool`
- consts: [{"value": 4, "src_line": 164, "meaning": "JungleType::Morgard 의 **태그값 4** (dienum JungleType 4). ⚠`src_line=164` 는 맞지만 **IR 에 리터럴이 없다** — L164 `take_active(Morgard)` 와 L172 `take_setup_like(Morgard)` 는 `icmp eq i8 (TeamPlan+0x41f), 0` 하나로 완전히 접혔고, `i8 4` 가 살아남는 곳은 **L169 `camp_pos`(m10.ll:7691) · L174 `v24_…`(m10.ll:7709) 둘뿐**이다. take_active/take_setup_like/camp_pos/v24_..._release_to_passive 4곳 전부에 이 캠프 상수가 박혀 있다 = 이 플랜은 에픽 전용", "kind": "태그", "ev": 3}, {"value": 0, "src_line": 164, "meaning": "MainObjective::Morgard 태그값. TeamPlan+0x41f 를 이 값과 비교하는 것이 take_active(Morgard) 의 접힌 실체(MainObjectiv
- 0.5.8 logic 전문:
```
// hunt_and_poke.rs:163 EpicHuntAndPokePlan::is_end(self, version, _rnd, player, data, team_plan, _debug) -> bool
// self / _rnd / _debug 는 본문에서 한 번도 읽지 않는다(readnone / gep 없음).

let team = player.info.team; // PlayerState+0x930
let game = data.cache.game; // &dyn AbstractGame (data ptr + vtable)

// ── L164 : 팀 주목표가 더 이상 에픽이 아니면 즉시 종료 ─────────────────
// 소스: if !team_plan.take_active(JungleType::Morgard) { return true }
// IR 실체: objective 태그 바이트 한 번 비교로 접힘
if (u8)TeamPlan[0x41f] != 0 { return true } // 0 = MainObjective::Morgard, 255 = None

// ── L169 : 에픽 캠프 좌표 ──────────────────────────────────────────
let (cx, cy) = data.context.map.camp_pos(JungleType::Morgard, team == 0); // (u64,u64)

// ── L172 : setup 단계인가 ─────────────────────────────────────────
// 소스: let setup_like = team_plan.take_setup_like(JungleType::Morgard)
let setup_like = ((u8)TeamPlan[0x41f] == 0) && ((u8)TeamPlan[0x420] == ObjectPhase::Setup /*1*/);

if setup_like { // L173
 // L174 — 오브젝티브 규율 계층이 '수동으로 풀어라'라고 하면 종료
 if TeamPlan::v24_objective_setup_should_release_to_passive(team_plan, version, player, data, JungleType::Morgard) {
 return true;
 }

 // L179 — 우리 팀 시야에 에픽 캠프 셀이 보이는가
 if game.is_visible_cell(team, cx / 32000, cy / 32000) { // vtable +0x100

 // L180~181 — 적팀 챔피언 5칸 중 '최근 목격된' 것만 남기고, 캠프에서 가장 가까운 하나
 let nearest = data.cache.player_champion[1 - team] // AGWC+0x1e0, [5]칸
 .iter()
 .filter_map(|c| *c) // iter_champions: None 칸 스킵
 .filter(|e| data.blackboard[1 - team].is_recent_visible(game, player, e))
 .min_by_key(|e| distance_sq(e.x, e.y, cx, cy));
 // ↑ filter 술어와 key 계산의 실제 본체는 담당 범위 밖:
 // m10.ll 6028~6196 (모노모피된 min_by_key). 술어 = Blackboard::is_recent_visible.

 match nearest {
 None => return true, // L183 — ★**구조적 도달 불가(죽은 경로)**. 판정반전 R2: 여기 오려면 v24==false 여야 하고 그건 `v23 != 0` 가지뿐인데, v23 의 슬롯 술어(is_some ∧ HP%>=40 ∧ 캠프거리<=180000 ∧ is_recent_visible)가 L180 필터(is_some ∧ is_recent_visible)의 **진부분집합**이라 v23!=0 이면 min_by_key 에 원소가 반드시 있다 ⟹ nearest==None 이 성립 불가
 Some(e) => { // L184
 let d2 = distance_sq(e.x, e.y, cx, cy); // abs_diff 제곱합 (utils.rs:6)
 if d2 > 22500000000 /* 150000^2 */ { return true } // 적이 캠프에서 멀다 → 종료
 }
 }
 }
 // is_visible_cell == false 이거나 적이 충분히 가까우면 아래로 흐른다
}
// setup_like == false 도 아래로 흐른다

// ── L193 : 에픽 몬스터가 지금 살아 있나 ────────────────────────────
let moba = game.get_game_mode().as_moba().unwrap(); // vtable +0x40, GameMode 태그 0 = Moba
 // 태그 != 0 이면 Option::unwrap 패닉(game.rs:231 / option.rs:1013)
let epic = moba.jungle_runner.epic.live_list.get(0) // MobaMode+0x198 (len==0 → None)
 .and_then(|id| game.get_entity_by_id(*id)); // vtable +0x1f0
if epic.is_some() { return false } // 살아 있으면 계속 헌트

// ── L194 : 죽어 있으면, 리젠까지 얼마나 남았나 ─────────────────────
let moba = game.get_game_mode().as_moba().unwrap(); // ★IR 상 get_game_mode 를 실제로 두 번 호출한다
return moba.jungle_runner.epic.next_respawn_tick // MobaMode+0x1b0
 .saturating_sub(game.tick()) // vtable +0x28
 > 15 * data.context.setting.tick_per_second; // GameSetting+0x12f8
// → 리젠까지 15초 초과로 남았으면 종료, 15초 이내면 계속 대기(포킹 유지)

// 요약 — true(종료) 가 되는 경로는 5가지:
// (a) 팀 주목표가 Morgard 가 아님 [L164]
// (b) setup 단계 + 규율계층이 passive 로 풀라고 함 [L174]
// (c) setup + 캠프 셀 시야 O + 보이는 적 챔프 0명 [L183] ★도달 불가( R2 · 범위 = objective==Some(Morgard{Setup}) 경로. Serpen 판 serpen/hunt_and_poke.rs:162 는 미확인)
// (d) setup + 캠프 셀 시야 O + 최근접 적이 150000 밖 [L184]
// (e) 에픽 죽어 있고 리젠까지 15초 초과 남음 [L194]
// false(계속) 가 되는 경로는 2가지:
// (f) 에픽이 살아 있음 [L194]
// (g) 에픽 죽었지만 리젠 15초 이내 [L194]
```
- logic_note: ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
