# apply060 batch_02.md — 반영(logic_060) 브리핑 · 함수 9

★**version = 3 런타임 확정(09-17)** — v<3/v2 경로는 명세에서 제외(사장). 태그/오프셋/vt 슬롯 = `spec_patch_060.md` §A(dispcheck 추가 행 포함). 출력 형식·절차 = `mkapply060.py` 도크스트링. 출력 = `_next/apply060/out/<old>.md`.

RE 정본 파일: 2026-09-17_r20_변경99_배치F_14_디컴대조_원문.md

### `dff080` → `ddd7e0` base_sub_goal (i=68 · ⚠변경·한 줄(r20))
- 0.5.8 src: `game-ai\src\plan_legacy\old\battle.rs:69` · one_line: BattlePlanGoal(TryKill/Support/Response/Avoid) → 기본 BattleSubPlanGoal 결정: 대상 추적(Trace) / 우물 위험이면 End / 가까운 적 있으면 KitingBack 없으면 RunAway
- 0.6.0 판정: **한 줄** · 패치 요지: `Some(_) => if v>=3 && Response && ally_within_120000 { Kiting } else { KitingBack }`
- RE 정본: `2026-09-17_r20_변경99_배치F_14_디컴대조_원문.md` · 절: `2026-09-17_r20_변경99_배치F_14_디컴대조_원문.md` §4
- sig: `fn(&game_ai::plan_legacy::old::BattlePlanGoal, usize, &game_core::PlayerState, &game_core::OperationData) -> game_ai::plan_legacy::old::BattleSubPlanGoal`
- consts: [{"value": 2, "src_line": 70, "meaning": "self.tag < 2 ⇔ TryKill(0)|Support(1). (팀 bounds check 의 2 도 동일 리터럴)", "kind": "임계", "ev": 4}, {"value": 0, "src_line": 73, "meaning": "Entity.team@tag == 0 (TeamType::Player) — is_ignored_well_enemy 인라인 조건. 반환 tag 0 = Trace", "kind": "태그", "ev": 4}, {"value": 1, "src_line": 73, "meaning": "enemy_team = 1 - player.team (L73·L81)", "kind": "계수", "ev": 4}, {"value": 7, "src_line": 73, "meaning": "BattleSubPlanGoal::End 메모리태그 — 대상이 우물 위험 안의 적 챔피언이면", "kind": "태그", "ev": 4}, {"value": 40000000001, "src_line": 83, "meaning": "200000² + 1 — 적↔나 거리²(|dx|²+|dy|
- 0.5.8 logic 전문:
```
fn base_sub_goal(&self, version, player, data) -> BattleSubPlanGoal
  team = player.info.team; enemy_team = 1 - team
  if self.tag < 2 {                                   // L70: TryKill{0: focus, ..} | Support{0: focus}
    focus = self.payload+8                            // L71
    target = game.get_entity_by_id(focus)             // L72 (vtable+0x1f0)
    // L73: if let Some(t) = target { if is_ignored_well_enemy(version, player, t) { return End } }   (fight_model.rs:754~756 인라인)
    //   is_ignored_well_enemy(t) = t.team == TeamType::Player(enemy_team) && is_enemy_well_danger(version, player, t.x, t.y)
    if target.is_some() && target.team@tag == 0 && target.team.0 == enemy_team
       && is_enemy_well_danger(version, player, target.x, target.y) → return End(7)
    return Trace{focus}(0)                            // L72 (target None 이거나 우물 위험 아님)
  }
  // Response | Avoid
  me = cache.player_champion[team][player.info.position].unwrap()     // L80 (None → unwrap_failed 패닉)
  nearest = cache.iter_champions(enemy_team)                            // L81 player_champion[1-team] 의 Some
      .filter(|e| data.blackboard[enemy_team].is_recent_visible(game, player, e)     // L82 (⚠ 적 팀 판 blackboard)
               && dist²(e, me) < 200000² + 1                                        // L83
               && !is_ignored_well_enemy(version, player, e))                       // L84 (= 적 챔피언이 우물 위험 안이면 제외)
      .min_by_key(|e| dist²(e, me))                                                  // L85 (첫 원소 인라인 + Map::fold)
  match nearest { Some(e) => KitingBack{focus: e.id}(3),   // L87
                  None    => RunAway(4) }                   // L86
```
- logic_note: ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)

### `ec9de0` → `f88880` wave_priority_clearer_position (i=73 · ⚠변경·한 줄(r20))
- 0.5.8 src: `game-ai\src\plan_legacy\team_plan\objective_helpers.rs:245` · one_line: 웨이브 우선 정리 담당 포지션 선정 — 적 미니언 중 우리 넥서스 최근접(없으면 라인 1차 타워 위치)에 가장 가까운 HP 30%+ 아군 포지션
- 0.6.0 판정: **한 줄** · 패치 요지: 4번째 인자 exclude_jungler: `pick(excl).or_else(|| pick(false))` · 콜러 passive_plan 4곳(0/LPH+0x24d5/*rsi/1)
- RE 정본: `2026-09-17_r20_변경99_배치F_14_디컴대조_원문.md` · 절: `2026-09-17_r20_변경99_배치F_14_디컴대조_원문.md` §1
- sig: `fn(&game_core::PlayerState, &game_core::OperationData, game_core::LineType) -> std::option::Option<usize>`
- consts: [{"value": 29, "src_line": 250, "meaning": "HP% 임계 — hp*100/max_hp > 29 (즉 30% 이상)인 아군 챔피언만 후보", "kind": "임계", "ev": 4}, {"value": 100, "src_line": 250, "meaning": "HP 백분율 계산 계수", "kind": "계수", "ev": 4}, {"value": 5, "src_line": 249, "meaning": "포지션 수 0..5 — 판정 아님", "kind": "산출값", "ev": 4}, {"value": 2, "src_line": 258, "meaning": "팀 인덱스 bounds — 판정 아님. L238 switch 의 LineType 태그 2=Bottom. (QC C1 shl 경고는 `shl i8 %2, 6` 의 레지스터 %2 오매칭 — 시프트량 6 = line*0x40 stride 접힘, 판정 아님)", "kind": "태그", "ev": 4}, {"value": 1, "src_line": 259, "meaning": "enemy_team = 1 - team · L238 switch LineType 1=Mid · 반환 S
- 0.5.8 logic 전문:
```
fn wave_priority_clearer_position(player, data, line) -> Option<usize>
let team = player.info.team;
// L246  target = wave_priority_clear_target(player, data, line)  — objective_helpers.rs:258~266 전부 인라인
//   L258  let nexus = cache.nexus[team]                         // +0x170 · None → 폴백
//   L259  let minions = cache.line_minions(line)[1 - team]      // simulation.rs:1807 · 0x10 + line*0x40 + enemy*0x20 (bumpalo Vec<&Entity>)
//   L260  minions.iter().min_by_key(|m| dist_sq(m, nexus))      // closure wave_priority_clear_target0 (인라인) · 빈 목록 → None
//   L261  Some(m) → target = (m.x, m.y)
//   L265  None(넥서스 없음 | 적 미니언 없음) → target = first_tower_position(team, line):
//         Top: team0 (48000,272000) / team1 (272000,48000)   Mid: team0 (368000,592000)/team1 (592000,368000)   Bottom: team0 (688000,912000)/team1 (912000,688000)
// L249~253
(0..5).filter_map(|pos| {
    let c = cache.player_champion[team][pos]?;                 // L249
    if !(c.hp * 100 / c.stat_cached.hp > 29) { return None; }  // L250 · max_hp==0 이면 div_by_zero 패닉
    Some((pos, dist_sq(c, target)))                            // L251
})
.min_by_key(|&(pos, d)| (d, pos))                              // L253 · 키 = (거리², 포지션) — 동거리면 낮은 포지션
.map(|(pos, _)| pos)                                           // L254~255
```
- logic_note: ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)

### `e5ca10` → `d5f3c0` try_engage (i=80 · ⚠변경·다건(r20))
- 0.5.8 src: `game-ai\src\plan_legacy\handler\engage.rs:40` · one_line: target_id 상대로 BattlePlan(일반/다이브)을 만들어 첫 update 까지 돌리고, 게이트(패배직후 쿨·다이브 재진입 쿨·추격무망·다이브 불가·즉시 후퇴 서브골)에 걸리면 None 을 돌려준다
- 0.6.0 판정: **다건** · 패치 요지: v3 게이트 A(LPH+0x2060/68/70 최근 실패 교전 reason∈{7,13,15,16}·3tps·ee8090) · B(bb+0x278/+0x260 타워 250000) · v3 update 2회 · +0x98 min · End→None
- RE 정본: `2026-09-17_r20_변경99_배치F_14_디컴대조_원문.md` · 절: `2026-09-17_r20_변경99_배치F_14_디컴대조_원문.md` §3
- sig: `fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, usize, &mut game_core::DebugFrameData) -> std::option::Option<game_ai::plan_legacy::old::BattlePlan>`
- consts: [{"value": 1, "src_line": 43, "meaning": "version > 1 ⇔ v2+ 게이트(L43·63·81·96). L57 의 `+1` 은 dive_rejoin_cd(engage.rs:973) 안의 리터럴(abandon_tick + (tps*4 + 1), LLVM 이 (abandon+1)+tps*4 로 재결합)", "kind": "임계", "ev": 4}, {"value": 3, "src_line": 44, "meaning": "패배직후 재교전 쿨 = tps*3 (3초). 별도로 L100 의 sub_goal 태그 3 = KitingBack", "kind": "태그", "ev": 4}, {"value": 2, "src_line": 57, "meaning": "folded: dive_rejoin_cd(engage.rs:973) = tps<<2 + 1 = tps*4 + 1 (4초+1틱; `+1` 도 :973 의 리터럴 `add i64 %100, 1 !dbg 973`) — `shl i64 %tps, 2`. 별도로 L74 entry_src=2(다이브), L76 EntityType Tower 태그 2, 팀 bounds 2", "folded_fr
- 0.5.8 logic 전문:
```
try_engage(&self, version, rnd, player, data, target_id, debug) -> Option<BattlePlan>
  if version > 1 && target_id == self.last_lost_fight.0 {                     // L43
    if game.tick() <= self.last_lost_fight.1 + tps*3 { return None } }        // L44~45  패배 직후 같은 상대 3초 재교전 금지
  target = game.get_entity_by_id(target_id)                                  // L52
  in_tower = target.map_or(false, |t| engage_requires_dive(player, data, t)) // L52 closure$0
  battle = if in_tower {                                                     // L53
    t = get_entity_by_id(target_id)?  (None → return None)                   // L54
    if game.tick() <= self.last_dive_abandon_tick + 1 + tps*4 { return None } // L57 (dive_rejoin_cd)
    if version > 1 {                                                         // L63
      if let Some(me) = cache.player_champion[team][pos] {                   // L64
        if open_chase_race_hopeless(version, data, player, me, t) { return None } } }   // L65
    if !tower_dive_is_viable(version, rnd, player, data, &self.team_plan, t, true, debug) { return None }   // L70
    bp = BattlePlan::new_dive(version, &TryKill(target_id, 60), data, player) // L73
    bp.entry_src = 2                                                         // L74
    tower = iter_towers_without_nexus(cache, 1-team).min_by_key(|tw| dist_sq(tw, t))   // L75 closure$1 (첫 최소)
    bp.dive_tower = tower.and_then(|tw| if tw.ty==Tower { Some(tw.info.ty) } else { None })   // L76~77 closure$2
    bp
  } else {
    if version > 1 {                                                         // L81
      if let (Some(me), Some(t)) = (player_champion[team][pos], get_entity_by_id(target_id)) {   // L82~83
        if open_chase_race_hopeless(version, data, player, me, t) { return None } } }   // L84
    bp = BattlePlan::new(version, &TryKill(target_id, 60), data, player)      // L89
    bp.entry_src = 1                                                         // L90
    bp }
  if version > 1 { battle.set_main_objective(self.team_plan.objective) }     // L96~97 (+0xff 3B)
  battle.update(version, rnd, player, data, &self.positioning_score, &self.team_plan, debug)   // L99
  if battle.sub_goal ∈ {KitingBack(3), RunAway(4), End(7)} { drop(battle); return None }   // L100~101
  return Some(battle)                                                        // L102
```
- logic_note: ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)

### `d3b2a0` → `eed7f0` v46_flee_gate_check (i=92 · ⚠변경·다건(r20))
- 0.5.8 src: `game-ai\src\plan_legacy\old\passive_line.rs:38` · one_line: v46 라인 도주 게이트 — 피난처(가장 가까운 타워/넥서스) 사거리 밖에 있을 때 근처 적 중 '나를 잡을 수 있는' 위협(committers)을 뽑고, 없으면 차단 사유 코드를 돌려준다
- 0.6.0 판정: **다건** · 패치 요지: A = aggr?0 : def?240 : 구 · B = 67(특성) : 구 · v3 `*60→*tps`
- RE 정본: `2026-09-17_r20_변경99_배치F_14_디컴대조_원문.md` · 절: `2026-09-17_r20_변경99_배치F_14_디컴대조_원문.md` §7
- sig: `fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity) -> (u8, bumpalo::collections::vec::Vec< usize>)`
- consts: [{"value": 22500000001, "src_line": 61, "meaning": "150000^2+1 — near_enemies: 적↔나 distance_sq < 이 값(≤150000, 4.7셀) (aux 66108); my_towers: 타워↔나 (aux 66207)", "kind": "미상", "ev": 4}, {"value": 22500000000, "src_line": 97, "meaning": "150000^2 — 적의 아군(L97)·내 아군(L115) 이 대상 적 e 로부터 distance_sq > 이 값이면 제외(> 150000 초과)", "kind": "임계", "ev": 4}, {"value": 40, "src_line": 97, "meaning": "적 아군 a 의 hp% < 40 이면 kill 전력에서 제외", "kind": "임계", "ev": 4}, {"value": 100, "src_line": 97, "meaning": "hp*100/max_hp 백분율 · radius*(mult+100)/100", "kind": "계수", "ev": 4}, {"value": 6, "src_line": 70, "meaning": "reac
- 0.5.8 logic 전문:
```
L40: pool = context.pool; L41: tps = setting.tick_per_second; L42: tick = cache.game.tick()[vtable 0x28]; L43: committers = Vec::new_in(pool)
L46: team = player.info.team; refuge = cache.iter_towers_without_nexus(team).min_by_key(closure$0: |t| t.distance_sq(champ))   (fold 은 별도 define — 키 = 타워↔나 distance_sq, 가장 가까운 타워)
L47: refuge = refuge.or(cache.nexus[team])   (team<2 bounds)
L48: refuge None → L49 return (5, [])
L51: under_refuge = refuge.attack_effect.as_ref().map_or(false, closure$1: |atk| champ.distance_sq(refuge) <= (atk.range(refuge) + champ.radius())^2)
     atk.range(e) 인라인(effect.rs:26) = atk.range + e.stat_buff_cached.range + (e.level-1)*atk.growth_range ; radius() 인라인(entity.rs:1511) = mult==0 ? radius : radius*(mult+100)/100
L55: if under_refuge → L56 return (5, [])   (IR 55766: dist_sq > r² 이면 %172 로 계속, 아니면 %181 코드 5 — 권역 안이면 도주 게이트 차단; attack_effect None 이면 map_or(false) → 계속)
L59~62: near_enemies = cache.iter_champions(1-team).filter(closure$2 = aux 66064: |e| e.distance_sq(champ) < 22500000001 && champ.is_visible_from(e)[champ.team Neutral→true; Player(t)→ e.visible_state[t]==Visible] && !is_ignored_well_enemy(version, player, e)).collect(pool)
L64: near_enemies.is_empty() → L65 return (1, [])
L69: my_pos = player.info.position; L70: reaction = champ.attack_duration() + 6; L71: my_ms = champ.stat_cached.move_speed; L72: escape_ticks = champ.distance(refuge) / max(my_ms,1)
L73~76: tower_disable_tick = match context.player_count()[tutorial→인원] { 2 => setting.tower_attack_disable_tick_2v2, 3 => _3v3, _ => tower_attack_disable_tick }   (switch 55889: First/Bottom→2v2, MidBottom→3v3, 그 외→기본)
L79~81: my_towers = iter_towers_without_nexus(team).filter(closure$3 = aux 66153: |t| !(tick > tower_disable_tick) && t.distance_sq(champ) < 22500000001).collect(pool)   ★타워는 tick ≤ tower_attack_disable_tick 일 때만 억지력에 포함
any_margin_pass = false; any_range_pass = false
L86: for e in near_enemies {
  L87: ep = cache.player_by_champion_id(e.id).unwrap(); L88: e_pos = ep.info.position; L89: aggr = ep.info.parameter.aggressive_ratio()
  L90: diff_bound = (1000-aggr)*80/1000 + 80;  L91: die_tick_bound = aggr*45/1000 + 45
  L96: (kill_dps, kill_nuke) = (0,0); for a in iter_champions(1-team) {   // 적 팀 전원(e 포함)
    L97: if a.distance_sq(e) > 22500000000 || a.hp*100/max(a.max_hp,1) < 40 { continue }
    L100: ap = player_by_champion_id(a.id).unwrap(); L101: c = cache.player_champion_cache[ap.team][ap.position]
    L103: nuke = if a.can_attack() || a.attack_cooldown()[ty별 인라인] <= tps { max(0, c.attack[my_pos]) } else { 0 }
    L104: if a.can_skill() || !(a.ty==Champion && a.skill_cooldown > tps) { nuke = max(nuke, c.skill[my_pos]) }
    L105: if a.can_skill2() || !(a.ty==Champion && a.skill2_cooldown > tps) { nuke = max(nuke, c.skill2[my_pos]) }
    L106: kill_dps += c.attack_per_sec[my_pos] + c.skill_per_sec[my_pos] + c.skill2_per_sec[my_pos];  L107: kill_nuke += nuke }
  L109: my_die = champ.hp.saturating_sub(kill_nuke) * 60 / max(kill_dps,1)
  L114: (det_dps, det_nuke) = (0,0); for a in iter_champions(team) {   // 내 팀 전원(나 포함)
    L115: if a.distance_sq(e) > 22500000000 || !(e.team Neutral || a.visible_state[e.team]==Visible) { continue }   // 적이 볼 수 있는 아군만 억지력
    L116: if is_ignored_well_enemy(version, ep, a) { continue }
    L119~126: ap/c 동일; nuke 계산 동일(인덱스 e_pos); det_dps += c.*_per_sec[e_pos]; det_nuke += nuke }
  L128: for t in my_towers { L129: if let Some(atk) = t.attack_effect { L130: dmg = atk.expected_damage_target(ctx, t, e); L131: det_dps += dmg*tps/max(t.attack_cooltime(),1); L132: det_nuke += dmg } }
  L135: e_die = e.hp.saturating_sub(det_nuke) * 60 / max(det_dps,1)
  L144: if diff_bound + my_die > e_die { continue }        // 내가 마진만큼 더 오래 못 버팀 → 위협 아님(교전 여유)
  any_margin_pass = true
  L150: er = e.attack_effect.map(|a| a.range(e)).unwrap_or(0) + e.radius() + champ.radius()
  L152: if e.distance(champ) > er + e.move_speed * reaction { continue }   // 반응시간 안에 못 닿음
  L155: if e.move_speed <= my_ms && my_die >= die_tick_bound { continue }  // 못 쫓아오고 나는 충분히 버팀
  any_range_pass = true
  L160: if my_die > escape_ticks + reaction { continue }   // 피난처까지 도망칠 시간이 있음
  L163: committers.push(e.id) }
L165: if !committers.is_empty() → (0, committers)
L167: else if any_range_pass → (4, [])
L169: else if any_margin_pass → (3, []) else (2, [])
(IR phi 56486~56487: %461=any_range_pass(true at 56439 L155 통과·56483), %462=any_margin_pass(true at L144 통과 이후 전 경로))
```
- logic_note: ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)

### `dc2070` → `ecfa40` SmallActionAroundPositionBush::get_input (i=157 · ⚠변경·다건(r20))
- 0.5.8 src: `game-ai\src\small_action\around.rs:1086` · one_line: 목표 수풀 셀까지 타워 회피 경로탐색(PathFinder) 입력 생성 — 목표 반경 16000 안이면 None
- 0.6.0 판정: **다건** · 패치 요지: PathVerdict: `ok && ((region==2&&flag2)||(region==7&&flag7)) && cheb(next,target)>=2 → 2` · flag=ec17f0(region 내 적)
- RE 정본: `2026-09-17_r20_변경99_배치F_14_디컴대조_원문.md` · 절: `2026-09-17_r20_변경99_배치F_14_디컴대조_원문.md` §2
- sig: `fn(&mut game_ai::SmallActionAroundPositionBush, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData) -> std::option::Option<game_core::Input>`
- consts: [{"value": 256000001, "src_line": 1089, "meaning": "16000² + 1 — 내 위치↔목표 제곱거리 < 이 값(= 거리 ≤ 16000, 반 셀) 이면 도착으로 보고 None 반환 (104661 `icmp ult`) · 오라클 실행 확증(23차 배치C: 오라클 실행 확증: d2=256000000(16000) → None / 256032001(16001) → Some(case1/2))", "kind": "임계", "ev": 2}, {"value": -1, "src_line": 1090, "meaning": "Option<Input>::None 니치 태그 (104685, 104731) · 오라클 실행 확증(23차 배치C: 오라클 실행 확증: None 경로 sret+0 == -1)", "kind": "센티널", "ev": 2}, {"value": 2, "src_line": 1094, "meaning": "Option<PathFinder>::None 니치 태그 비교 (104676·104708 `icmp eq i8 %52, 2`). 또한 player_champion 팀 인덱스 상한 2(panic_bounds_check 104613
- 0.5.8 logic 전문:
```
SmallActionAroundPositionBush::get_input(&mut self, version, rnd, player, data, ps) -> Option<Input>   [around.rs:1086]
 entity = data.cache.player_champion[player.info.team][player.info.position].unwrap()   (L1087; team<2 아니면 panic, None 이면 unwrap 패닉)
 (ex, ey) = (entity.x, entity.y) ; (tx, ty) = (self.target_x, self.target_y)
 if |ex-tx|² + |ey-ty|² < 256000001:   # 16000²+1                                (L1089)
     return None                                                                  (L1090)
 tower_dodge = TowerDodgeContext::new_tower_avoid_v3(version, player, data, tx, ty)   # 416B 지역   (L1093)
 if self.path_finder.is_none():                                                   (L1094, 태그 +0x5d == 2)
     self.path_finder = Some(PathFinder::new_target(rnd, data.context, version, "dodge_tower_cell", ex, ey, tx, ty,
         |sx,sy,nx,ny| /*closure#0, 환경 {version, player, data, ps, &tower_dodge}*/ … -> PathVerdict))     (L1095)
     if self.path_finder.is_none(): return None      # ★사장 분기 — new_target 은 태그 2 를 쓰지 않는다(m03.ll:14343~17815, +0x45 에 0/1 만) · Option::as_mut 의 접히지 않은 match   (L1104)
 self.path_finder.as_mut().update_path(rnd, data.context, ex, ey, tx, ty, |sx,sy,nx,ny| … /*closure s_0, 같은 환경*/)   (L1106)
 return Some(self.path_finder.get_input(player, data, SafeMoveWithSkill::Safe, false))   (L1114; sret 직접 기록)

클로저 계약(관측): 두 클로저 모두 환경 {version, player, data, ps, &tower_dodge} 를 잡고 PathFinder 인스턴스(m03.ll 14343~17816 / 31379~32434) 안에 인라인돼 `dodge_tower_cell_with_context(version, player, data, ps, tower_dodge, sx,sy,nx,ny)` 를 부른다(m03 17654·31641 — 호출부에서 version/data/ps 가 `poison` = 콜리가 그 인자를 안 읽는다는 LLVM IPO 증거). check_cell 은 이 함수 경로에서 호출되지 않는다(Bush 계열만).
```
- logic_note: ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)

### `e29b40` → `100c2f0` action_eval::evaluate_action (i=165 · ⚠변경·다건(r20))
- 0.5.8 src: `game-ai\src\plan_legacy\action_eval.rs:67` · one_line: v33 통합 행동 평가 — 라인 앵커 컨텍스트에서 Around 계열 소액션만 채점: positional_gain − danger + last_hit_gain 을 Some(score) 로, 그 외는 None(기존 경로 위임)
- 0.6.0 판정: **다건** · 패치 요지: score += aggressive_gain(aggr: 최근접 적 피해×hp_value/hp · reach+ms*90 감쇠) − defensive_penalty(def: 대칭)
- RE 정본: `2026-09-17_r20_변경99_배치F_14_디컴대조_원문.md` · 절: `2026-09-17_r20_변경99_배치F_14_디컴대조_원문.md` §5
- sig: `fn(usize, &game_ai::plan_legacy::action_eval::ActionContext, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> std::option::Option<i64>`
- consts: [{"value": 1, "src_line": 77, "meaning": "Anchor 태그 1 = Lane (53201) · EntityType 1 Minion (53828) · max(…,1) 가드 · level-1 (53850). (같은 리터럴 1 이 shl 시프트량으로도 쓰이나 그건 아래 folded_from=2 행에 분리 — QC 경고는 이 중복 때문) · 오라클 실행 확증(23차 배치E: 오라클 실행 확인(23차 E o165.rs 14/14 MATCH, 케이스당 프로세스 1개): anchor Free/Objective→None · Attack/Stop→None · AroundBush/Around/LaneMinionPosition→Some · undying→danger 0 · hp 1/1000/100000 스케일 · risk_possible 2건→possible 700→danger 560(hp_value 800) · Some(-560000/-560/-5))", "kind": "태그", "ev": 2}, {"value": 1, "folded_from": 2, "src_line": 222, "meaning": "tps*2 (=2초) 가 `shl i64 
- 0.5.8 logic 전문:
```
L77: let Anchor::Lane{line} = ctx.anchor (태그 1) else → None
L78: if action ∉ {Around(2),AroundHide(3),AroundRegion(4),AroundPosition(7),AroundPositionBush(8),AroundBush(9),LaneMinionPosition(10)} → None   (RunAway·Recall·AroundRunAway·Positioning·Trace·Attack~Stop 은 기존 경로)
L81/L89: return Some(lane_positioning_score(version, player, data, parameter, action, line))

── lane_positioning_score (인라인 L98~130) ──
L98 : champ = player_champion[team][pos].unwrap()  (None 이면 panic)
L99 : purpose = line_phase_position_eval_purpose(player, data)  (i8 0..8)
L100: dest = action.evaluation_position(_, player, data)                     [Option<(x,y)> sret 24B]
L103: dest_score = dest.map(|(x,y)| position_eval_at(version, player, data, x, y, purpose))   [Option<PositioningScore> 56B · 니치 +0x31==2]
L104: hp_value = champion_hp_value(data, parameter, &parameter.player)
L105: hp = champ.hp.max(1)
L107~108: base_damage = dest_score.map(|s| hp * s.risk.max(0) / 100).unwrap_or(parameter.player.risk_damage)
L109~112: possible_damage = parameter.player.possible_risk(data, 9999) + dest_score.map(|s| hp * s.tower_risk.max(0) / 100).unwrap_or(parameter.player.risk_possible_tower) / 3
L113~116: danger = if champ.stat_buff_cached.undying { 0 } else { base_damage*hp_value/hp + possible_damage*hp_value/hp }   (sdiv 각각)
L124~125: positional_gain = dest_score.map(|s| (s.gain − s.gain_me).max(0) * hp_value / 200).unwrap_or(0)
L127: (ax,ay) = dest.unwrap_or((champ.x, champ.y))
L128: last_hit_gain = lane_anchor_gain(data, player, parameter, ax, ay, line)
L130: return positional_gain − danger + last_hit_gain

── lane_anchor_gain (인라인 L141~259) ──
L141: champ = player_champion[..]? (None→0) ; L142: atk = champ.attack_effect.as_ref()? (None→0)
L143: move_speed = champ.stat_cached.move_speed.max(1) ; L144: start_tick = atk.start_timing*100 / champ.attack_speed_mult().max(1) ; L145: tps = ctx.setting.tick_per_second
L155: (n, entries) = LAG_MEMO.with(|c| … )  [aux: 키 (game.seed(vtable+0x20), game.tick(vtable+0x28), player.info.id, line) 일치 → 캐시 반환 · 불일치 → iter_minions(1-team) 를 visible_from(champ.team)&&Minion&&line 으로 걸러 (tx,ty,attack_range,dmg,tid,bonus) 를 최대 48개 저장 · 48 초과면 n=-1]
  attack_range = champ.stat_buff_cached.range + atk.range + (champ.level−1)*atk.growth_range + atk.range_adjust(champ, target) + radius(champ) + radius(target)   [radius(e) = e.radius_mult==0 ? e.radius : e.radius*(mult+100)/100]
  dmg = atk.expected_damage_target(ctx, champ, target) ; bonus = target.nearest_enemy 가 (아군 팀 && Tower/Nexus) 면 80 아니면 0
L197: if n == -1 (캐시 오버플로) →  L200~231 직접 순회: for target in iter_minions(1-team):
    L201 !target.is_visible_from(&champ.team) → skip ; L204 ty≠Minion → skip ; L207 line≠ → skip
    L210 attack_range(위 식) ; L211 walk_dist = distance(ax,ay,target).saturating_sub(attack_range) ; L212 walk_tick = walk_dist/move_speed ; L213 impact_tick = walk_tick+start_tick ; L214 dmg = expected_damage_target
    L216 s = 0 ; if let Some(snap)=parameter.wave_snapshot && let Some(traj)=snap.find(target.id):
        L217 predicted_hp = traj.hp_at_tick(impact_tick)
        L218 if predicted_hp <= 0            → s = (traj.expected_death_tick > tps*2) ? 0 : 25      (L222)
             elif predicted_hp <= dmg+5     → s = 140
        L220 elif predicted_hp > dmg*2       → s = (death_tick > tps*2) ? 0 : 25                    (L222)
             elif death_tick > impact_tick+tps → s = (death_tick > tps*2) ? 0 : 25                  (L222)
             else                            → s = 70
    L226~227: if target.nearest_enemy = Some(eid) && enemy=get_entity_by_id(eid) && enemy.team == champ.team && enemy.ty ∈ {Tower,Nexus} → s += 80
    L231 best = max(best, s)
  else → L237~256 캐시 순회: for (tx,ty,attack_range,dmg,tid,bonus) in entries[..n]:
    L238~240 walk_dist/walk_tick/impact_tick 동일 ; L243~250 snapshot 있으면 위 s 사다리 동일(find(tid)) 없으면 s=0 ; L254 s += bonus ; L256 best = max(best, s)
L259: return best (기본 0)
```
- logic_note: ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)

### `dbd260` → `fdb210` SmallActionRecall::get_input (i=184 · ⚠변경·한 줄(r20))
- 0.5.8 src: `game-ai\src\small_action\move_actions.rs:681` · one_line: 귀환 소액션의 틱 입력: 우물 안이면 None, 안전하고 우물까지 도보 시간(dist/move_speed)이 tps*3+60 틱 이상이면 Input::Return(그보다 가까우면 걸어서 간다), 아니면 우물 방향 7x7 셀 후보를 위험·거리로 채점해 goal 을 (재)확정하고 PathFinder(dodge_danger_cell) 로 Move 입력을 만든다.
- 0.6.0 판정: **한 줄** · 패치 요지: v3: 최근가시 적 250000 내 `r1=ckdt(true,false,false)==0 || r2=ckdt(true,true,false)<tps → goal=healp, committed`
- RE 정본: `2026-09-17_r20_변경99_배치F_14_디컴대조_원문.md` · 절: `2026-09-17_r20_변경99_배치F_14_디컴대조_원문.md` §9
- sig: `fn(&mut game_ai::SmallActionRecall, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &mut game_core::DebugFrameData) -> std::option::Option<game_core::Input>`
- consts: [{"value": 32000, "src_line": 682, "meaning": "셀 크기. L682 우물 x/y 좌표(1셀) · L729 잔여 웨이포인트 1개당 거리 · L741·L760·L761·L903·L904 좌표→셀 나눗셈 · L789 edge_ticks 분자 · L834·L858 셀→좌표", "kind": "계수", "ev": 4}, {"value": 928000, "src_line": 682, "meaning": "= 29*32000 : 반대편 우물 좌표. healp = team==0 ? (32000, 928000) : (928000, 32000)", "kind": "산출값", "ev": 4}, {"value": 2, "src_line": 700, "meaning": "version < 2 → legacy 분기(direct_heal=false·enemy_knows=false·free_dist 미사용·choose·is_enemy_danger_cell·direct_cross=true). 또 self+0x45 == 2 = Option<PathFinder>::None 태그(L726·L955·L985) · L1015 SafeMoveWithSkill::M
- 0.5.8 logic 전문:
```
fn get_input(&mut self, version, rnd, player, data, ps, _debug) -> Option<Input>   // move_actions.rs:681
// ── 절1 L682~721: 즉시 판정 ──
team = player.info.team;  healp = team==0 ? (32000, 928000) : (928000, 32000)              // L682
champ = data.cache.player_champion[team][player.info.position].unwrap()                  // L688 (None→panic)
(lx,ly,rx,ry) = ctx.map.fountains[team]                                                    // L689
if lx<=champ.x<=rx && ly<=champ.y<=ry { return None }                                       // L691~692 이미 우물 안
direct_heal = version>=2 && distance(champ, healp) < 200001                                  // L700 (version<2: false, distance 호출 안 함)
if direct_heal { self.goal=(healp); self.goal_committed=true }                              // L702~704
(enemy_positions, visible_enemy_count) = collect_visible_enemy_positions(player,data)       // L707
enemy_knows = version>=2 && enemy_knows_my_position(player,data)                             // L710
eff_risk = |s:&PositioningScore| s.risk + (enemy_knows ? s.unseen_champ_threat : 0)          // closure#0 L711
dist_to_heal_area = distance(champ, healp)                                                   // L714
time = dist_to_heal_area / champ.move_speed                                                  // L716 (speed 0 → panic)
if is_safe_recall(version,rnd,player,data,ps) && time >= setting.tick_per_second*3 + 60 {   // L719 (is_safe_recall 먼저 평가 · ★도보 시간이 3초+60틱 **이상**일 때만 귀환 시전, 그 안쪽은 걸어서 간다 — m08.ll:100017~100021 `%124 = icmp ult i64 %117, %123 ; br i1 %124, label %115(계속), label %125(store i64 1)` · 오라클 dist 480000/240000(time 480/240)→Some(Return), 239999(time 239)→Move)
    return Some(Input::Return) }                                                             // L720
if !direct_heal {                                                                            // L721 (direct_heal 이면 절4 로 직행)
  // ── 절2 L723~757: 진전 추적 · 확정 goal 재평가 ──
  now_tick = cache.game.tick()                                                                // L723
  prog_now = match self.path_finder { Some(pf) if pf.path_len>0 =>                            // L726
        idx=min(pf.index, path_len-1); (wx,wy)=pf.path[idx];                                  // L727~728
        isqrt(dist²(champ,(wx,wy))) + (path_len-1-idx)*32000                                 // L729
     _ => isqrt(dist²(champ, self.goal)) }                                                    // L731
  if prog_now < self.prog_best_dist_sq { self.prog_best_dist_sq=prog_now; self.prog_best_tick=now_tick }   // L733~735
  stale = None
  if self.goal_committed {                                                                    // L738
     if now_tick.saturating_sub(self.prog_best_tick) > 119 {                                  // L739 120틱 진전 없음
         stale = Some((min(goal_x/32000,29), min(goal_y/32000,29))); self.goal_committed=false // L741·L746
     } else if dist²(champ, self.goal) > 143999999 {                                          // L751 (goal 까지 >=12000)
         s = positioning_score_at_position(version,player,data,ps, goal_x,goal_y, Recall)     // L752
         risk_now = positioning_risk_value(version,player, eff_risk(s), s.on_trajectory||s.on_periodic_trajectory)  // L753
         if !(risk_now > self.goal_risk) { goto 절4 }   // L755~757: 위험이 goal_risk 이하로 유지되면 옛 goal 그대로 경로 생성 (dbg 이름 keep_committed_goal = (risk_now > goal_risk), true 면 재탐색)
     }  // dist² <= 143999999 (goal 근처) 이면 재탐색으로 진행
  }
  // ── 절3 L760~939: 후보 셀 채점 ──
  champ_cell=(min(x/32000,29), min(y/32000,29)); (hx,hy)=(healp.x/32000, healp.y/32000); (cx,cy)=(ps.cx, ps.cy)   // L760~765
  candidates: bumpalo Vec<(u64,u64,i64)> in ctx.pool                                          // L766
  vis = enemy_positions[..visible_enemy_count]   (count>5 → panic)                            // L769
  nearby = vis.count(|e| dist²(champ,e) < 40000000000)                                        // L770 closure#1 (200000²)
  risk_weight = nearby>2 ? 5 : nearby==2 ? 3 : 1                                              // L772
  if version>=2 {                                                                             // L776
     f2_enemy_anchor = vis.filter(dist²<62500000000).min_by_key(dist²)  → Option<(ex,ey)>      // L778~780 closure#2/#3 (250000²), 최초 최소 유지
     f2_now_dist = anchor.map(|a| distance(champ, a))                                          // L784 closure#4
     b1_free = Some(shared_free_dist(ctx.map))  (Arc<FreeDistMap>)                             // L787
     edge_ticks = max(32000 / max(champ.move_speed,1), 1)                                      // L789
     edge_dmg_milli = 0; for e in cache.player_champion[1-team].iter_champions() {             // L791 (team := 1-team)
         if data.blackboard[1-team].is_recent_visible(cache.game, player, e)                   // L792 ⚠적팀 blackboard
            && dist²(e, champ) <= 40000000000 {                                                // L795
            edge_dmg_milli = edge_dmg_milli.saturating_add(fight_dps(version,ctx,e,champ).saturating_mul(edge_ticks)) } }   // L798~799
     chase_pct_per_cell = edge_dmg_milli / (max(champ.hp,1)*10)                                // L801
     b1_home_w_per_unit = (chase_pct_per_cell*risk_weight)/5 + 2                               // L802 (sdiv)
     c1_chaser = vis.filter(dist²<40000000000).min_by_key(dist²) → Option<(ex,ey)>            // L810~812 closure#6/#7
     c1_home_me = free[champ_cell → home_cell(hx,hy)]  (u16)                                   // L816 closure#8, index = (cy*30+cx)*900 + (hy*30+hx)
  } else { f2=None; b1_free=None; b1_home_w_per_unit=0; c1_chaser=None }                       // L776 else / L808
  flee_ok: bumpalo Vec<(u64,u64,i64)>                                                          // L818
  for dx in 0..7 { for dy in 0..7 {                                                             // L820~821
     xi=cx-3+dx; yi=cy-3+dy (wrapping)                                                          // L822~823
     if xi>29 || yi>29 || map.walls[yi][xi]!=0 || stale==Some((xi,yi)) { continue }             // L824 (stale 셀 제외)
     if let Some((ex,ey))=f2_enemy_anchor { wx=xi*32000+16000; wy=yi*32000+16000;
        if distance((wx,wy),(ex,ey)) + 8000 < f2_now_dist { continue } }                       // L833~834 앵커 적에게 더 가까워지는 셀 제외
     cell = positioning_score_at_cell(version,player,data,ps, xi,yi, Recall)                    // L838
     nxt = map_setting.path.find_path(champ_cell.x, champ_cell.y, xi, yi)                       // L839
     risk = if let Some((nx,ny))=nxt { next=positioning_score_at_cell(nx,ny,Recall);            // L840~841
              (eff_risk(next) + eff_risk(cell)) / 2 }                                            // L842 (sdiv)
            else { eff_risk(cell) }                                                             // L844
     risk = positioning_risk_value(version,player, risk, cell.on_trajectory||cell.on_periodic_trajectory)   // L846~847
     score = if let Some(free)=b1_free { d=free[(xi,yi)→home]; if d==0xFFFF {d=150};            // L850~851
                 -(risk*risk_weight + b1_home_w_per_unit*d) }                                    // L852
             else { for_nexus=|hx-xi|+|hy-yi|; -10*for_nexus - risk*risk_weight }               // L854~855
     candidates.push((wx,wy,score))                                                             // L858~859
     if let (Some(chaser),Some(free)) = (c1_chaser,b1_free) {                                   // L861
        home_cand = free[(xi,yi)→home];                                                         // L863
        c1_worse_both = home_cand > c1_home_me && home_cand != 0xFFFF && dist²((wx,wy),chaser) < dist²(champ,chaser)   // L864 (u16 비교)
        if c1_worse_both { continue } }                                                         // flee_ok 에 넣지 않음
     flee_ok.push((wx,wy,score))                                                                // L871
  }}
  if version>=2 && !flee_ok.is_empty() { candidates = flee_ok }                                 // L877~878 (교체, 구 candidates drop)
  if candidates.is_empty() {                                                                    // L881~882 (분기 방향으로 극성 확정)
     self.goal=(healp); self.goal_committed=false                                               // L883~884 · L934
  } else {
     candidates.sort_by_key(|&(_,_,s)| -s)                                                      // L886 closure#9 (점수 내림차순)
     (min_range,max_range) = positioning_choice_window(version,player, param.positioning_runaway_min_range(), param.positioning_runaway_max_range(), true)   // L887~889
     len=candidates.len(); min_idx=min(len*min_range/1000, len-1); max_idx=clamp(len*max_range/1000, min_idx, len-1)   // L890~891
     candidates = candidates.into_iter().skip(min_idx).take(max_idx-min_idx+1).collect_in(pool)  // L893~895
     p = version<2 ? candidates.choose(rnd).unwrap() : positioning_window_pick(version,rnd,player,data,&candidates)   // L898~901
     pre_xi=min(goal_x/32000,29); pre_yi=min(goal_y/32000,29)                                  // L903~904 (옛 goal)
     pre = positioning_score_at_position(version,player,data,ps, goal_x,goal_y, Recall); pre_risk=eff_risk(pre)   // L906~907
     pre_score = if let Some(free)=b1_free { d=free[(pre_xi,pre_yi)→home]; d==0xFFFF?150:d; -(pre_risk*risk_weight + b1_home_w_per_unit*d) }   // L908~912
                 else { pre_for_nexus=|hx-pre_xi|+|hy-pre_yi|; -10*pre_for_nexus - pre_risk*risk_weight }   // L914~915
     (gx,gy) = if pre_score < p.s || dist²(champ, self.goal) <= 143999999 { (p.x,p.y) } else { (goal_x,goal_y) }   // L917 옛 goal 이 더 좋고 아직 멀면 유지
     self.goal=(gx,gy)                                                                          // L923~924
     s = positioning_score_at_position(...gx,gy,Recall); self.goal_risk = positioning_risk_value(version,player, eff_risk(s), s.on_trajectory||s.on_periodic_trajectory)   // L929~930
     self.goal_committed = true                                                                 // L932
  }
  self.prog_best_dist_sq = u64::MAX; self.prog_best_tick = now_tick                             // L937~938
  drop(flee_ok, b1_free(Arc 감소), candidates)                                                  // L939
}
// ── 절4 L943~1016: 경로 생성 · Move 입력 ──
tower_dodge = TowerDodgeContext::new_tower_avoid_v3(version,player,data, self.goal_x, self.goal_y)   // L943
pve_hazard_recall = PveHazardContext::new(version,player,data)                                 // L944
policy = SolvePolicy{ deadly_cells: v3_deadly_edge_cells(version,player,data), direct_cross: version<2 }   // L947~948
(enemy_positions, visible_enemy_count) = collect_known_enemy_positions(version,player,data)     // L951 (known 판으로 교체)
enemy_hazard = EnemyHazardContext::new(version,player,data); zone_hazard = ZoneHazardContext::new(version,player,data)   // L953~954
verdict = |sx,sy,nx,ny| -> PathVerdict {                                                       // closure#10 L956~982 (== closure#11 L987~1013)
   if tower_dodge.deadly_band_cell(version,nx,ny) { Deadly }                                    // L958
   else if !dodge_tower_cell_with_context(version,player,data,ps,&tower_dodge,sx,sy,nx,ny) { Danger }   // L961
   else if !direct_heal && version>=2 {                                                         // L965~966
        if let Some(v)=enemy_hazard.verdict(goal_x,goal_y,nx,ny) { v }                          // L967
        else if let Some(v)=zone_hazard.verdict(goal_x,goal_y,nx,ny) { v }                      // L970
        else { pve_hazard_recall.verdict(version,player,data,nx,ny).unwrap_or(Allow) } }        // L978
   else if !direct_heal && is_enemy_danger_cell(nx,ny,goal_x,goal_y,&enemy_positions,count,6400000000,1) { Soft }   // L973 (version<2)
   else { pve_hazard_recall.verdict(version,player,data,nx,ny).unwrap_or(Allow) } }             // L978 (direct_heal 이면 곧장 여기)
if self.path_finder.is_none() {                                                                 // L955
   self.path_finder = Some(PathFinder::new_target_with_policy(rnd,ctx,version,"dodge_danger_cell",policy, champ.x,champ.y, goal_x,goal_y, verdict)) }   // L956
let Some(p) = &mut self.path_finder else { return None };                                        // L985 (구조상 존재, 실질 도달 불가)
p.update_path(rnd,ctx, champ.x,champ.y, self.goal_x,self.goal_y, verdict)                        // L987
return Some(p.get_input(player,data, SafeMoveWithSkill::Must, false))                           // L1015 (sret 직접 기록)
```
- logic_note: ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)

### `cb1ce0` → `e986c0` SerpenHuntSubPlan::action_candidates (i=197 · ⚠변경·한 줄(r20))
- 0.5.8 src: `game-ai\src\plan_legacy\sub_plan\serpen_hunt.rs:201` · one_line: 세르펜 사냥 서브플랜의 행동 후보 목록 생성 — need_recall 판정→Recall 단독 / v27 규율 행동 단독 / 공격 후보(battle·summon·jungle)를 타워·대상·아군스킬·score 필터로 거른 뒤 get_move_action(추적·도주·대기)과 합쳐(431~514, 배치 H·I) 반환
- 0.6.0 판정: **한 줄** · 패치 요지: get_move: `if tp.cc7 { s=efb5b0(tp,0,team0); if dist²(me,serpen) > dist²(me,s) { push AroundPosition::new(s,5) } }`
- RE 정본: `2026-09-17_r20_변경99_배치F_14_디컴대조_원문.md` · 절: `2026-09-17_r20_변경99_배치F_14_디컴대조_원문.md` §13
- sig: `fn(&mut game_ai::plan_legacy::sub_plan::SerpenHuntSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallA`
- consts: [{"value": 2, "src_line": 203, "meaning": "player_champion 1차 배열 길이(team < 2 bounds check) · L545 kind > 2 · L569 kind-3 <u 2 · L524 casting Direction · F1 Tower ty", "kind": "길이", "ev": 4}, {"value": 3, "src_line": 207, "meaning": "need_recall 판정: champ.hp <= dmg*3 (세르펜 평타 3방, `mul i64 %156, 3`). 본문의 `shl …, 3` 은 8B 포인터 stride 라 이 상수와 무관 · 오라클 실행 확증(25차 배치C: 오라클 실행 확인(25차 C o197, 케이스당 프로세스 1개): 세르펜 스폰(공격 50) · champ.hp=3*dmg=150 → need_recall 1·Recall / hp=151 → 0·Trace (o197_case3.log·case4.log))", "kind": "임계", "ev": 2}, {"value": 100, "src_line": 212, "meaning": "hp_ratio = hp*100/max_hp (
- 0.5.8 logic 전문:
```
// serpen_hunt.rs:0~428 (배치 G)
// ── 함수 머리 serpen_hunt.rs:201 · 정본 = m02.ll:11075~ · 이 배치 = 루트 줄 203~428 + 그 줄에 뿌리를 둔 클로저 5개(aux)
// 표기: `L<n>` = 소스 루트 줄 · `%N` = m02.ll 레지스터 · 오프셋은 tcxdict 정본 이름 · 「→ 배치 X(줄 N)」 = 다른 배치 범위로 넘어가는 br
//
// L203 (m02.ll:11261~11283): team = player.info.team(+0x930); if team >= 2 → panic_bounds_check(2)
//        champ = data.cache(+0).player_champion(+0x1e0)[team][player.info.position(+0x9c0 i32)]  ; None(null) → unwrap_failed(%120)
// L204 (11288~11290): if self.need_recall(+0x0 bool) {
// L212 (11310~11313 · 11391~11394):   max_hp = champ.stat_cached.hp(+0x628); max_hp==0 → panic_const_div_by_zero
//                                     hp_ratio = champ.hp(+0x670) * 100 / max_hp
// L213 (11396~11397):   if hp_ratio > 29 { self.need_recall = false }   // IR `icmp ugt 29` — 소스 표기(>29 / >=30)는 표기 불가
//        } else {
// L205 (11297~11348):   gm = data.cache.game.<vtable+0x40 get_game_mode>() ; tag(+0)!=0(Moba 아님) → unwrap_failed(%167)
//                       m = gm.payload(&MobaMode) ; live = m.jungle_runner.serpen.live_list(ptr +0x1d0, len +0x1d8)
//                       serpen: Option<&Entity> = live.first().and_then(|id| game.<vtable+0x1f0 get_entity_by_id>(*id))   // len==0 → None ; get_entity null → None
//                       if let Some(serpen) = serpen {
// L206 (11353~11363):     serpen.attack_effect(+0x490) 의 tag(+0x4c0)== -1(None) → unwrap_failed(%161)
//                         dmg = Effect::expected_damage_target(&serpen.attack_effect, data.context(+0x8), serpen as &dyn(vtable @anon.54 = Entity), champ)
// L207 (11365~11369):     if !(champ.hp(+0x670) > dmg*3) { self.need_recall = true }   // 즉 champ.hp <= 3*dmg
//                       }
//        }
//        (%162: phi need_recall = [1 ← L207 경로, 0 ← L213 경로] → store self+0x0)
// L220 (11382~11384): if self.need_recall {
// L221 (11413~11422):   res = bumpalo Vec::new_in(data.context.pool(+0x0 of GameContext))   // ptr=8(dangling)·bump·cap=0·len=0
// L222 (11425 · 17751~17790):   res.push(SmallActionPlay::Recall(SmallActionRecall::new(data, player, end_delay=5)))   // 136B → 184B 슬롯, tag@+0xb1 = 4(Recall)
// L223 (17792~17794):   return res   (sret 32B memcpy) → %286 = 함수 끝(L514, 배치 I 의 합류 라벨)
//        }
// L226 (11405~11409): if let Some(action) = team_plan.v27_objective_discipline_action(version, rnd, player, data, target=JungleType::Serpen(i8 5)) {   // sret 184B · tag@+0xb1 == -1 → None
// L227 (11430~11437):   return bumpalo::vec![in data.context.pool; action]   (from_iter_in<[SmallActionPlay;1]>) → %286(L514, 배치 I)
//        }
// L231~233 (11444~11685): nearest_enemy_tower: Option<&Entity>
//        = data.cache.iter_towers_without_nexus(1 - team)        // sret 120B = Chain<Flatten<IntoIter<[Option<&Entity>;6]>>, Copied<slice::Iter<&Entity>>>
//          .filter(|t| t.can_target(+0x6b9) && t.block_target_tick(+0x6a0) == 0)     // 클로저 s_0 (첫 6칸은 인라인 루프 11533~11599, 나머지 슬라이스는 try_fold<find::check<s_0>> 11624)
//          .min_by_key(|t| dist²(t, champ))   // (11649~11682: dx=|t.x-champ.x|, dy=|t.y-champ.y|, dx²+dy² ; Map<Filter<..>>::fold(min_by) 11682) ; 결과 ptr → %96
// L236 (11702~11750): act_actions(%94) = { 인라인 헬퍼(L668~673, 파일 미상 — 아래 unknown):
//          res = Vec::new_in(pool);
//          res.extend(fight_check::battle_action(version, rnd, player, data, _end_delay=5));      // L670
//          res.extend(fight_check::attack_summon_action(player, data));                           // L671
//          res.extend(self.attack_jungle_action(team, position(i32), data));                      // L672
//          res }
// L237 (11762~11779): strategy = PlayerState::strategy(player, rnd, data.cache.game)  (24B Strategy)
//        object_finish_objective(%93): Option<&Entity> =
//          if strategy.object_finish(+0xf) == KillPriority(0) {
// L238 (11784~11810):   get_game_mode() ; Moba 아님 → unwrap_failed(%327) ; live_list.len==0 → None
// L239 (11813~11826):   live_list.first().and_then(get_entity_by_id)
//          } else { None }
// L243~312 (11835~11873 · aux m01.ll:564~742): act_actions(%91) = act_actions.iter().filter(F1).map(|a| a.clone()).collect_in(pool)
//        F1 = 클로저 s2_0 (call_mut aux m02.ll:75123~75783) · 캡처 = (game data_ptr, game vtable, player, &object_finish_objective, champ, &nearest_enemy_tower)
//        F1(a) → keep(true) 판정 (IR 극성 = 분기 방향, %313 phi):
//          L245: if let Some(id) = a 의 대상 id(small_action.rs:309 헬퍼 — tag∈{15,16,17,18}=Attack/Skill/Skill2/Ult 이면 Some(+0x8)) {
//          L246:   if let Some(target) = game.get_entity_by_id(id) {
//          L247:     if fight_model::should_ignore_object_finish_kill_priority_target(player, target, *object_finish_objective) { return false }
//                  }}
//          L252: if let Some(id) = a 의 대상 id { if let Some(t) = get_entity_by_id(id) { if t.ty(+0x68) != Champion(13) { return true } } }   // 비-챔피언 대상은 무조건 유지
//          L253: match a {   // 니치 디코드: tag>2 ? tag-3 : 7(AroundPosition)
//            RunAway·Recall·Around·AroundHide·AroundRegion·AroundRunAway·Positioning·AroundPosition·AroundPositionBush·AroundBush·LaneMinionPosition·Trace·Stop → false
//            Attack(15):  L256: target = get_entity_by_id(a.target(+0x8)) else false
//                         L257: eff = champ.attack_effect(+0x490).unwrap()   (tag -1 → unwrap_failed)
//                         L258: if !eff.is_in_range(champ, target) → false
//                               nearest_enemy_tower None → true
//                               Some(t): if !(t.attack_effect.unwrap().is_in_range(t, champ) && target.ty != Tower(2)) → true
//                         L259:          else → return (target.team == champ.team)   // TeamType PartialEq(entity.rs:1127): tag 다르면 false, 둘 다 Neutral → true, Player 면 id 비교
//                                        (해석: 적 타워가 나를 때릴 수 있는 위치에서 타워가 아닌 상대 진영 대상 평타 후보는 버린다)
//            Skill(16):   L266: target = get_entity_by_id(a.target(+0x8)) else false
//                         L267: eff = champ.skill_effect(+0x4c8).unwrap()   (tag +0x4f8 == -1 → unwrap_failed)
//                         L268: if !eff.is_in_range(champ, target) → false ; 타워 게이트 = Attack 과 동일(L269 팀 비교, 불일치 → false; 통과 시 계속)
//                         L269: if !(eff.ty.<EffectType vtable+0x68 expected_move_on_hit>() || eff.ty.<+0x60 expected_rush_effect>()) → false
//                         L270: nearest_enemy_tower None → true ; Some(t) → return s2_0s4_0(target, t) = !t.attack_effect.unwrap().is_in_range_ex(t, target, t.x, t.y, target.x, target.y, 15000)
//            Skill2(17):  L281~285: 위와 동일, eff = (champ.level(+0x5c8) > 2 ? &champ.skill2_effect(+0x500) : &None).unwrap(), 중첩 클로저 s2_0s6_0
//            Ult(18):     L296~300: 위와 동일, eff = (champ.level > 4 ? &champ.ult_effect(+0x538) : &None).unwrap(), 중첩 클로저 s2_0s8_0
//          }
// L314 (11885~11893 · aux m01.ll:2147~5070): act_actions.retain(F2)   · 캡처 = (data, champ, player, &version)
//        F2(a) → keep 판정 (%1260 = 유지, %1269 = 제거):
//          L315: match a { Skill(16) → L317.., Skill2(17) → L351.., Ult(18) → L385.., 그 외(RunAway~Trace·Attack·Stop) → 유지 }
//          [Skill 갈래 L317~335 · Skill2 L351~369 · Ult L385~403 는 effect 만 다르고 동일 구조]
//          L317: target = get_entity_by_id(a.target(+0x8)) ; None → 제거
//          L318: if target.team != champ.team → 유지   (적 대상 스킬은 그대로)
//          L320: eff = champ.skill_effect.unwrap()  (None → 제거)   [Skill2: level>2 ? skill2_effect : None · Ult: level>4 ? ult_effect : None]
//          L321: has_heal   = eff.ty.<vtable+0x40 expected_heal>(data.context, champ as &dyn) != 0
//          L322: has_shield = eff.ty.<+0x48 expected_shield>(ctx, champ) != 0
//          L323: has_buff   = eff.ty.<+0x50 expected_buff>(ctx, champ).is_some()   (sret 288B, tag@+0x48 != -1)
//          L324~327: near_enemy = [player_champion[1-team] · iter_towers(1-team) · cache.jungles(+0xd0) · cache.others(+0xf0)[1-team]]
//                    .any(|e| dist²(e, target) < 14400000001 && e.is_visible_from(&champ.team))   // 120000²+1 · is_visible_from(entity.rs:1481): Neutral→true, Player(t)→visible_state(+0x38)[t].tag == Visible(0)
//          L328: hp_ratio = target.hp(+0x670)*100 / target.stat_cached.hp(+0x628)   (0 → div_by_zero panic)
//          L329: if has_heal && !has_buff && hp_ratio > 79 {
//          L331:    keep = (!has_shield || near_enemy) && buff_value::aoe_heal_covers_low_ally(version, eff, data, player, target)
//                } else {
//          L335:    keep = !has_shield || has_buff || near_enemy   (has_heal&&has_buff 는 무조건 유지)
//                }
// L423 (11903~11917 · aux m01.ll:5073~5262): act_actions.retain(|a| { L424: s = self.score(version, parameter, rnd, player, data, a, debug); L425: !(s < -30) })   // score < -30 → 제거
// L428 (11930~14415): move_actions(%87) = get_move_action(version, rnd, player, data, &parameter.positioning_score(+0x9f0), team_plan, debug)   // serpen_hunt.rs:517 인라인, 아래 전개
//   L517~519: res = Vec::new_in(pool) ; champ = player_champion[team][pos].unwrap()(%408)
//   L522~528: has_non_target_action_range = player_champion[1-team].iter().flatten().any(|c|
//       L523:   utils::nontarget_windup_perceived(version, player, data, c) && c.ty == Champion(13)
//       L524:   && match c.champion.action_state(+0x70) {
//                 Skill(4)  → e = c.skill_effect.unwrap() (tag +0x4f8: -1 → unwrap_failed) ; e.casting(+0x30) ∈ {Position(1), Direction(2)} && Effect::is_in_range(e, c, champ)
//       L526:    Skill2(5) → e = (c.level>2 ? skill2_effect : None).unwrap() ; 동일
//       L528:    Ult(6)    → e = (c.level>4 ? ult_effect : None).unwrap() ; 동일
//                 _ → false })
//   L535: position_score = position_eval::position_score_at_position(version, player, data, positioning_score, champ.x(+0x660), champ.y(+0x668), purpose=Objective(i8 11))   // sret 56B PositioningScore
//   L537: if position_score.on_trajectory(+0x30) || has_non_target_action_range || position_score.on_periodic_trajectory(+0x31) {
//   L539:   res.push(RunAway(SmallActionRunAway::new_with_skill(data, player, end_delay=5, with_skill=true)))   // tag 3
//   L540:   return res → move_actions → L431(배치 H)
//         }
//   L543: posture = team_plan.v25_objective_posture(version, player, data, target=Serpen(i8 5))   // sret 88B Option<ObjectivePosture>, None = focus_enemy tag(+0)== -1
//   L544: if let Some(posture) = posture {
//   L545:   if posture.kind(+0x50) > 2 {   // WaitGroup(3)|SoftDisengage(4)
//   L546:     if kind == SoftDisengage(4) && posture.near_enemy_count(+0x38) != 0 {
//   L547:       res.push(RunAway(new_with_skill(data, player, 5, false)))  }
//   L549:     res.push(AroundPosition(SmallActionAroundPosition::new(rnd, data, posture.wait_pos.0(+0x20), .1(+0x28), end_delay=5)))   // 니치 untagged(태그 store 없음)
//   L550:     if data.context.debug(+0x3b) {
//   L551:       debug.infos(+0xa0 HashMap<usize,Vec<String>>).entry(champ.id(+0x5c0)).or_insert(vec![]).push(format!(@anon.126 …{:?}, posture.kind))  }
//   L553:     return res → L431(배치 H)
//           }
//   L556:   if kind == Screen(2) && posture.focus_enemy(+0x0 tag)==Some {
//   L557:     focus_enemy = posture.focus_enemy(+0x8)
//   L558:     res.push(Trace(SmallActionTrace::new_attack_range(data, focus_enemy, end_delay=5)))   // = new_attack_range_margin(…, 15000): attack_range_only=true
//           }  (그 외 Commit/HoldCamp/Screen-무초점 → 통과)
//         }
//   L564: serpen = get_game_mode() Moba(m) → m.…live_list.first().and_then(get_entity_by_id) ; Moba 아님 → unwrap_failed(%1293)
//   L565: objective_in_attack_range = false ; if let Some(serpen) = serpen {
//   L566:   mr = max_range(champ, serpen) [인라인 헬퍼 L16~26]: eff = champ.attack_effect.unwrap()(None → unwrap_failed)
//             = eff.range(+0x4a0) + champ.stat_buff_cached.range(+0x438) + eff.growth_range(+0x4a8)*(champ.level(+0x5c8)-1)
//               + Effect::range_adjust(eff, champ, serpen) + radius(champ) + radius(serpen)
//             radius(e) = e.stat_buff_cached.radius_mult(+0x470)==0 ? e.radius(+0x680) : e.radius*(mult+100)/100
//   L567:   if dist²(champ, serpen) <= mr² { objective_in_attack_range = true }
//           else {
//   L568:     camp = MapDef::camp_pos(data.context.map(+0x20), Serpen(i8 5), team==0)
//             if !objective_helpers::v23_should_break_objective_hunt_anchor(player, data, champ, serpen, camp.x, camp.y) {
//   L569:       if !(posture.is_some() && posture.kind ∈ {WaitGroup(3), SoftDisengage(4)}) {   // `kind-3 <u 2`
//   L570:         res.push(Trace(new_attack_range(data, serpen.id(+0x5c0), 5)))   (SmallActionPlay::push 호출)
//   L572:         return res → L431(배치 H)
//         } } } }
//   L577: strategy = player.strategy(rnd, game) ; not_kill_priority = strategy.object_finish(+0xf) != KillPriority(0)
//   L592: positioning_accuracy = AthleteParameter::positioning_accuracy(&player.info.parameter(+0x180)) ; min_v = positioning_accuracy
//   L594: max_v = 2000 - positioning_accuracy
//   L596: near_enemies: bumpalo Vec<&Entity> = data.cache.iter_champions().filter_map(..).filter(F3(champ)).collect   // F3 = 클로저 s1_0(call_mut m02.ll:75047~75120, 캡처 champ): e.is_visible_from(&champ.team)(entity.rs:1482 인라인: champ.team Neutral(tag 1)→true / Player(t)→ e.visible_state[t](+0x38+24t).tag==Visible(0), t>=2 → panic_bounds_check) && dist²(e, champ) < 25600000000 (=160000², 75114 `icmp ult`) — 아군 포함 여부는 iter_champions 심(filter_map) 소관
//   L597: me_die_tick = fight_check::check_kill_die_tick(version, rnd, data, player, champ, near_enemies.clone(), Vec::new_in(pool), debug)
//   L600: runaway = false ; force_runaway = false ; for enemy in near_enemies {
//   L601:   if !(serpen.is_none() || not_kill_priority || fight_model::can_enemy_hit_objective(enemy, serpen, 25000)) { continue }
//   L605:   jrng = utils::range_misjudge_rng(version, data, player, enemy.id)   (16B)
//   L606:   mr       = battle::max_range_can_use(champ, enemy)  * utils::range_misjudge_roll(rnd, &jrng, min_v, max_v) / 1000   // ★아래 `roll` 은 변수가 아니라 매번 새 range_misjudge_roll(rnd,&jrng,min_v,max_v) 호출 — L606 13141 · L607 13152 · L608 13163 · L625 13207 (적 1명당 3회 + me_die_tick<tps 시 1회)
//   L607:   emr      = max_range_can_use(enemy, champ)          * roll / 1000
//   L608:   emr_near = battle::max_range_nearly_can_use(enemy, champ, 40) * roll / 1000
//   L609:   dist = dist²(champ, enemy)
//   L612:   if me_die_tick < data.context.setting(+0x8).tick_per_second(+0x12f8) {
//   L625:     emr = max_range_nearly_can_use(enemy, champ, 60) * range_misjudge_roll(rnd, &jrng, min_v, max_v) / 1000   // ★4번째 roll 호출(m02.ll:13207) — 이 가지(me_die_tick < tps)에서만 rnd 추가 소비
//   L627:     if dist > mr² && emr < mr { L629: res.push(Trace(SmallActionTrace::new(data, enemy.id, 5)))  }   // attack_range_only=false
//   L630:     else if dist <= emr² { runaway = true; force_runaway = true }
//           } else {
//   L613:     if Entity::remain_action_time(enemy) > 10 && (max_range_can_use(champ,enemy)*roll) > 999 {   // ≡ mr >= 1 (표기 불가: 소스는 `mr > 0` 추정)
//   L614:       if dist > mr² { L615: res.push(Trace(new(data, enemy.id, 5))) }
//   L617:     } else if emr < mr && dist > mr² { L619: res.push(Trace(new(data, enemy.id, 5))) }
//   L620:     else if dist <= emr_near² { runaway = true }
//           } }
//   L638: for e in data.cache.others(+0xf0)[1-team] { L639: if let Some(atk)=e.attack_effect { L640: range = max_range(e, champ); L641: if dist²(champ,e) <= range² { runaway=true; force_runaway=true; break } } }
//   L649: break_objective_anchor = serpen.map_or(false, |s| v23_should_break_objective_hunt_anchor(player, data, champ, s, camp_pos(Serpen, team==0)))
//   L651: if runaway && (break_objective_anchor || !objective_in_attack_range || force_runaway) {
//   L652:   res.push(RunAway(new_with_skill(data, player, 5, false)))
//   L653: } else if objective_in_attack_range {
//   L654:   res.truncate(0)
//   L655:   if let Some(serpen) = serpen {
//   L656:     margin = 안전 공격 여유 [인라인 헬퍼 L16~29, 문자열 "v27 serpen safe attack position"(@anon.129) 의 소유 함수 추정]:
//               range = max_range(champ, serpen) ; inner = clamp(range.saturating_sub(10000), 12000, 45000) ; dist = Entity::distance(champ, serpen)
//               if dist + 15000 < range { if inner + dist > range { rnd.gen_range(12000..=inner) } else { 12000 } } else { inner }
//   L657:     res.push(Trace(new_attack_range_margin(data, serpen.id, 5, margin)))   // attack_range_only=true
//   L659:   } else { res.push(Stop) }   // tag 19
//   L661: } else if res.is_empty() {
//   L662:   res.push(RunAway(new_with_skill(data, player, 5, false)))
//         }
//   L665: return res → move_actions(%87) ; near_enemies drop → %1311 = L431(배치 H)
// ── 이 배치 범위 끝. L431 이후(act_actions·move_actions 병합·평가) = 배치 H, L489~514 = 배치 I

// serpen_hunt.rs:431~486 (배치 H)
// 진입: 배치 G 의 428 거대 문이 끝난 뒤 블록 %1311(m02.ll:14417). 로컬: nearest_enemy_tower=%96(Option<&Entity>, 233 기록) · act_actions=%91 · move_actions=%87 · champ=%115 · team=%104 · pos=%110

431: if let Some(nearest_tower) = nearest_enemy_tower {                       // 14421 null 검사
432:   if let EntityType::Tower(info) = &nearest_tower.ty {                     // 14427 +0x68 == 2
433:     if info.nearest_enemy.map(|(_, id)| id) == Some(champ.id) {          // 14442 태그 trunc→i1, 14456 (+0x98) == champ.id(+0x5c0) · option.rs:1161/2440 인라인
434:       act_actions.truncate(0);                                            // 14473
435:       move_actions.truncate(0);                                           // 14478
436:       move_actions.push(SmallActionPlay::RunAway(SmallActionRunAway::new_with_skill(data, player, 5, true)));  // 14484 생성 · 14490 tag=3 · 14492 push
       } } }

441: if !act_actions.is_empty() {                                              // 14437 len(+0x18)==0 → 1994(=456 경로로 직행)
442:   let moba = game.get_game_mode().moba().unwrap();                          // 14502 vtable+0x40 · 14516 tag!=0 → 17736 unwrap_failed(패닉)
       let serpen = moba.jungle_runner.serpen.live_list.get(0).and_then(|id| game.get_entity_by_id(*id));   // 14534 len==0 → None · 14550 vtable+0x1f0 · null → None. (.first() 와 표기 불가)
       if let Some(serpen) = serpen {                                            // None → 455
443:     let attacking_objective = act_actions.iter().any(|a| a.get_action().target() == Some(serpen.id));
           // 클로저$10 인라인: 태그(+0xb1)-15 < 4 (Attack/Skill/Skill2/Ult) && +0x8 target == serpen.id (14601~14618). 빈 슬라이스면 1994(456) 직행(컴파일러 단축)
444:     if attacking_objective {
445:       if let Some(action) = self.v27_objective_safe_attack_position(version, rnd, player, data, &parameter.positioning_score, serpen) {   // ▼ 전량 인라인(helper 43~145)
446:         if data.context.debug {                                          // 15995 ctx+0x3b
447:           debug.infos.entry(champ.id).or_insert(vec![]).push("v27 serpen safe attack position".to_string());   // 16019~16131
             }
449:         return bumpalo::vec![in ctx.pool; SmallActionPlay::AroundPosition(action)];   // 16007 memcpy 184B → 16008 from_iter_in(sret %0) → 함수 종료(배치 G 에필로그 %1956)
450:         (unwind 시 action drop 16266)
           }
         }
       }
455:   // if !act_actions.is_empty() → 1997 = 511(배치 I): act_actions 를 그대로 반환(16275 memcpy %0←%91) · 비면 456 으로
     }

// ───── helper v27_objective_safe_attack_position(serpen_hunt.rs:43~145) — 445 에 인라인. objective = serpen ─────
 43: let champ = data.cache.player_champion[team][pos]?;                         // 14678 재로드 · null → None(1917)
 44: let objective_hp_ratio = objective.hp(0x670)*100 / max(objective.stat_cached.hp(0x628), 1);   // 14686~14694
 45: if objective_hp_ratio < 36 { return None }                                  // 14696
 49: let attack_range = objective_attack_range(champ, objective);
       // = [16] champ.attack_effect.as_ref().unwrap()(0x4c0 tag==-1 → unwrap_failed 14737)
       //   [17] Effect::range(champ)(effect.rs:26 = 0x438 + 0x4a0 + (level(0x5c8)-1)*0x4a8, 14717~14723, 14783~14786) + range_adjust(atk, champ, objective)(14725) + champ.radius()(14729~14755) + objective.radius()(14760~14778)
 52: if distance_sq(champ, objective) > attack_range*attack_range { return None }   // 14816~14831
 56: let threats: bumpalo Vec<&Entity> = game.iter_champions(player_champion[1-team]).filter(|t|                     // 14854~14870 (m01.ll:34222 수집)
 58:      blackboard[player.team].is_recent_visible(game, player, t)             // aux 75824~75839
 59:      && (distance_sq(t, champ) <= 260000² || distance_sq(t, objective) <= 220000²)   // 75879 / 75911
     ).collect_in(ctx.pool);
 61: if threats.is_empty() { return None }                                      // 14879 → 1494 drop → 1917
 65: let hp_ratio = champ.hp*100 / max(champ.stat_cached.hp, 1);               // 15012~15020
 66-68: let current_nearest_enemy = threats.iter().map(|t| distance(champ.x, champ.y, t.x, t.y)).min().unwrap();   // 15071 첫 원소 + m12.ll:31322 fold(min_by Ord::cmp, 동률 시 앞 원소)
 70-72: let threat_contact = threats.iter().any(|t| {
 71:        let range = max(max_range_nearly_can_use(t, champ, 50), 100000) + (if hp_ratio < 50 { 70000 } else { 40000 });   // 15125/15132 · 15103~15104(루프 밖 호이스트) · 15157
 72:        distance_sq(champ, t) <= range*range });                            // 15140~15165 (초과면 다음 원소)
 74: if !threat_contact && !(hp_ratio < 45 && current_nearest_enemy <= 220000) { return None }   // 15169~15172 (`< 220001`)
 78: let keep_attack_range = attack_range.saturating_sub(30000);                 // 15175
 83-85: let allies: Vec<&Entity> = game.iter_champions(player_champion[team]).filter(|a| a.id != champ.id && distance_sq(a, objective) <= 220000²).collect_in(pool);   // 15186~15198 · aux 75929~75970 (m01.ll:34400)
 87-91: let ally_centroid: Option<(u64,u64)> = if allies.is_empty() { None } else { Some((Σa.x / n, Σa.y / n)) };   // 15207 · 15239~15264(x 합) · 15391~15416(y 합) · 15419~15420 udiv n
 93: drop(allies)
 98-99: let (xi, yi) = (positioning_score.cx as i32, positioning_score.cy as i32);   // 15428~15435 (7×7 탐색창 중심 셀)
100-101: let here_value = v27_positioning_value(version, player, &position_score_at_position(version, player, data, positioning_score, champ.x, champ.y, Objective));   // 15437 · 15445~15486
     // v27_positioning_value(34~38): positioning = player.info.parameter.positioning_effective();
     //   gain_weight = (100 + (50-positioning)/10).clamp(95,105); risk_weight = (100 + (positioning-50)/2).clamp(75,125);
     //   trajectory_penalty = if score.on_trajectory || score.on_periodic_trajectory { max(positioning-50, 0) } else { 0 };
     //   value = score.gain*gain_weight/100 - score.risk*risk_weight/100 - trajectory_penalty   (IR: sdiv 100 · sdiv -100)
102: let here_score = here_value + min(current_nearest_enemy, 320000)/10000 + 8;   // 15491 · 15519 · 15609(+8 은 현위치에만) · 15612~15613
103:   + cohesion(champ.x, champ.y)   // closure$6(95): ally_centroid.map_or(0, |(ax,ay)| 320000.saturating_sub(distance(x,y,ax,ay))/10000)  15501~15521 · 15614
105: let mut best: Option<(x, y, score, nearest_enemy)> = None;                 // %1732 tag
106: for dx in 0..7 {  let xb = xi - 3 + dx;                                    // 15570 · 15587 (xb>29 면 내부 루프 통째 skip 15588/15961)
107:   for dy in 0..7 {
109:     let yb = yi - 3 + dy;                                                   // 15664
110:     if xb < 0 || yb < 0 || yb > 29 || map.walls[yb][xb] != 0 { continue }   // 15666~15680 (MapDef+0x78 [30][30])
114-115: let (x, y) = (xb*32000+16000, yb*32000+16000);                         // 15684~15689
116:     if distance_sq((x,y), objective) > keep_attack_range² { continue }      // 15691~15699 (dx² 는 외부루프 15593~15597)
120-123: let nearest_enemy = threats.iter().map(|t| distance(x, y, t.x, t.y)).min().unwrap_or(u64::MAX);   // 15705~15774 · m12.ll:31218 fold
124-125: let cell_value = v27_positioning_value(version, player, &position_score_at_cell(version, player, data, positioning_score, xb, yb, Objective));   // 15777~15822
126:     let enemy_spacing = min(nearest_enemy, 320000)/10000;                   // 15827~15830
127:     let range_slack = min(keep_attack_range.saturating_sub(distance(x, y, objective.x, objective.y)), 80000)/10000;   // 15834~15842 · 15882~15884
128:     let score = cell_value + enemy_spacing + range_slack + cohesion(x, y);  // 15853~15890
130:     if best.map_or(true, |b| score > b.score) {                              // 15899~15902 (score <= best.score 면 유지 → 동률 시 먼저 찾은 셀)
131:        best = Some((x, y, score, nearest_enemy)) }                          // 15905~15916
     } }
136: let (bx, by, best_score, best_nearest) = best?;                             // 15574 tag false → None
137: if best_score < here_score && best_nearest < current_nearest_enemy.saturating_add(30000) { return None }   // 15615~15619
140: if distance_sq(champ, (bx,by)) <= 12000² { return None }                    // 15622~15636 (`< 144000001`)
144: Some(SmallActionAroundPosition::new_with_radius(rnd, data, bx, by, 3, 25000))   // 15640 · 15644~15648 (177B+tag+6B 분리 복사) · None 판정 = byte0xb1 == 0xff(15985)
145: drop(threats)                                                              // 15650
// ───── helper 끝 ─────

455: if act_actions.is_empty() {                                               // 14507 (비지 않으면 → 511 배치 I: act_actions 반환)
456:   let positioning_accuracy = player.info.parameter.positioning_accuracy();   // 16271
458:   let (min_v, max_v) = (positioning_accuracy, 2000 - positioning_accuracy);   // 16395
462:   let danger = game.iter_champions(player_champion[1-team]).any(|c| {      // 16440~16575 (슬롯 0..5 순, any 단락)
463:      let jrng = range_misjudge_rng(version, data, player, c.id);           // 16476
464:      let emr = max_range_can_use(c, champ) * range_misjudge_roll(rnd, &jrng, min_v, max_v) / 1000 + 10000;   // 16484 → 16488 → 16492~16494 (호출 순서 = rnd 미러 순서)
465:      let mr = max_range_can_use(champ, c);                                 // 16496
467:      blackboard[1-team].is_recent_visible(game, player, c)                // 16503 (★적팀 블랙보드)
468:      && distance_sq(c, champ) <= emr*emr                                   // 16511~16540
469:      && !c.is_in_action() && !c.block_input()                             // 16543~16553 (is_in_action: ty==13 && action_state.tag>=3)
471:      && mr == 0 });                                                        // 16557~16560 (IR: (mr!=0 || block_input) → 다음 원소; 469/471 의 && 결합 순서는 표기 불가)
472:   if danger { return bumpalo::vec![in pool; SmallActionPlay::RunAway(SmallActionRunAway::new_with_skill(data, player, 5, false))] }   // 16691 · 17478~17482 → 함수 종료(%2368)
475:   let best_move_action = move_actions.iter().max_by_key(|a| self.score(version, parameter, rnd, player, data, a, debug)).unwrap();   // 16580~16685 (첫 원소는 serpen_action_score 16679 로 인라인 · fold m12.ll:22735, 동률 시 뒤 원소) · 빈 목록이면 unwrap_failed 16710(패닉)
479:   let incoming = game.iter_projectile().any(|p|                             // 16706 vtable+0x210 · 16728 next
480:        p.team != TeamType::Player(team) && !p.is_targeting()               // 16748~16763 (tag0==0 && +8==team → skip) · projectile.rs:134: Target/TargetSplash/BouncingTarget{target_id:Some} → skip (16767~16782)
481:        && distance_sq(champ, (p.x, p.y)) < 420000² );                       // 16789~16811 (+0x100/+0x108)
482-483: let enemy_rushing = game.iter_champions(player_champion[1-team]).any(|c| matches!(c.rush_state, RushState::Rush{..} | RushState::RushPenetrate{..}));   // 16832~16974 5슬롯 언롤 · +0x308 태그 sgt -1(암묵 RushPenetrate) || == 0x8000000000000003(Rush)
484:   if incoming || enemy_rushing {                                            // 2224/2225
485:     let input = best_move_action.clone().get_input(version, rnd, player, data, &parameter.positioning_score, debug);   // 16996 clone → 17007 get_input(sret %74 32B)
486:     drop(clone)                                                            // 17016 (unwind 17012)
         → 489(배치 I): switch input.tag(+0 i64: -1 / 0 / 기타)                 // 17018~17022 · 조건 거짓이면 2223(489, 배치 I)
       }
     }
// 이 범위에서 다른 배치로 넘어가는 지점: 449·472 → 함수 에필로그(배치 G 루트줄 1) · 455 비어있지 않음 → 511(배치 I) · 484 이후 → 489(배치 I) · 모든 unwind → %1326(513, 배치 I)

// serpen_hunt.rs:489~514 (배치 I)
// 진입 문맥(배치 H 소관, 계약만): L455 `if act_actions.is_empty()`(%91.len==0, m02.ll:14506~14510) 의 참 가지 안에서 L475 best_move_action = move_actions.iter().max_by_key(serpen_action_score).unwrap() (%2127, &SmallActionPlay) · L479~483 trajectory_possible: bool · L484~488 `let move_action_input: Option<Input> = if trajectory_possible { best_move_action.clone().get_input(version, rnd, player, data, &parameter.positioning_score, debug) } else { None };` (DI: !20720 line484 type Option<Input> · get_input 호출 m02.ll:17007 · 임시 clone %73 드롭 L486).
//   ⚠IR 상 trajectory_possible==false 경로(%2223, L482 any 루프 소진)는 get_input 을 호출하지 않고 곧장 %2226(L506 경로)으로 점프한다(m02.ll:16986 `br label %2226 ;L489`) = None 케이스와 같은 코드. 또한 L455 거짓 가지(act_actions 비어있지 않음)는 %1997 → L511.

// ── L489 (DILexicalBlock !20723 · 변수 move_action_input: Input 재바인딩 !20722)
// 줄 길이 산술(rmeta_srcmap: L489 = 60자 = 들여쓰기 8 + 52) 와 정합하는 형태: `if let Some(move_action_input) = move_action_input {`
tag = *(move_action_input as *i64)                          // %2232 = load %74 (m02.ll:17018)
switch tag {
  -1 (None)              => goto L506                        // %2226
  0  (Some(Input::Move)) => goto L490                        // %2233
  _  (Some(그 외 5종))    => goto L503                        // %2239
}

// ── L490 (DILexicalBlock !20725 · x,y !20724/!20726 line 490 · 줄 59자 = 10+49 ⇔ `if let Input::Move { x, y } = move_action_input {`)
// IR 은 x,y 로드에 L489 를 붙였다(let-chain 인라인 아티팩트). L490 자체 분기 명령 없음(태그 0 케이스 = 이미 Move 확정).
x = move_action_input.+0x8 ; y = move_action_input.+0x10     // %2237, %2235

// ── L491 한글 주석(28자 mb) · L492 `let on_trajectory = {` (DILexicalBlock !20728) · L493 `let position_score = position_score_at_position(` (!20730)
position_score: PositioningScore(56B) = position_score_at_position(version, player, data, &parameter.positioning_score /*+0x9f0*/, x, y, PositionEvalPurpose::Objective /*i8 11*/)   // m02.ll:17035
// ── L495 (83자 = 14+69 ⇔ `position_score.on_trajectory || position_score.on_periodic_trajectory`)
on_trajectory = position_score.on_trajectory /*+0x30*/ || position_score.on_periodic_trajectory /*+0x31*/   // 단락: +0x30 참이면 +0x31 미로드 (17045~17056)
// L496 `};` — position_score 수명 종료(lifetime.end !26785)

// ── L497 `if on_trajectory {`
if on_trajectory {
  // L498: return bumpalo::collections::Vec::from_iter_in([SmallActionPlay::RunAway(SmallActionRunAway::new_with_skill(data, player, 5 /*end_delay*/, false /*with_skill*/))], data.context.pool)
  ra: SmallActionRunAway(136B) = new_with_skill(data, player, 5, false)   // m02.ll:17072 sret %70
  elem(%71): memcpy 136B ← ra ; elem.+0xb1 = 3 (RunAway 태그)               // 17086~17089
  *sret = from_iter_in([elem], data.context.pool)                            // 17090 → cap1 len1
} else {
  // L500: return from_iter_in([best_move_action.clone()], data.context.pool)
  elem(%68) = Clone::clone(best_move_action /*%2127*/)                       // 17066 ; memcpy 184 → %69 ; 17078 from_iter_in
}
// L502 `} else {` — Some(Move) 가 아닌 Some(_)
// L503: return from_iter_in([best_move_action.clone()], data.context.pool)   // %66 clone 17041 · 17100 from_iter_in
// L505 `} else {` — None (및 trajectory_possible==false 의 스레딩 경로)
// L506: return from_iter_in([best_move_action.clone()], data.context.pool)   // %64 clone 17002 · 17228 from_iter_in
// ⟹ 500/503/506 은 바이트 단위로 동일한 결과(같은 %2127 clone · 같은 bump). 차이는 오직 498 RunAway 로 갈 수 있느냐.

// ── L508 `} else {` (L455 act_actions.is_empty() 의 거짓 가지) · L509~510 한글 주석 · L511 `act_actions`
// %1997 (m02.ll:16275): *sret = memcpy32(act_actions %91) — 이동이므로 %91 은 드롭하지 않음.

// ── L513 `    }` (L455 if 의 닫는 괄호 자리 · 지역 Vec 드롭 · 255 IR줄) — 반환 경로별 순서:
//   489~506 경로: %2258 drop(move_actions %87) → 인라인 dealloc(%87) → %2295 drop(act_actions %91) → dealloc → %2330 drop(act_actions %94) → dealloc → %2365 → L514
//   511 경로:     %1997 drop(%87) → dealloc → %2440 → %2330 drop(%94) → dealloc → L514        (%91 은 이동됨)
//   472 경로(배치 H, %2366→%2368): %1957 drop(%87) → %2369 drop(%91) → %2404 drop(%94) → %2439 → L514
//   각 drop = <Vec<SmallActionPlay> as Drop>::drop(아웃오브라인 · 원소 drop_in_place) + 인라인 RawVec/Bump::dealloc:
//     if cap != 0 { footer = *(buf.a + 0x10); if footer.ptr(+0x20) == buf.ptr { footer.ptr = buf.ptr + cap*184 } }   // 마지막 할당만 되돌림(bumpalo 는 하향 할당)
//   언와인드 정리: %1326 drop_glue(%87)→%333→(%334 phi 로 %91 드롭 여부 분기: 2015 경로만 false)→%2443 drop_glue(%91)→%287 drop_glue(%94)→caller.
// ── L514 `  }` ret void (%286).

// 사장 코드: reach.txt 「사장 호출부」 = L254 unwrap_failed(범위 밖). 이 범위(489~514) 에 NA 콜리 없음.
// 이 범위에서 push 되는 variant 집합: {RunAway(태그3 · new_with_skill(data, player, end_delay=5, with_skill=false)), best_move_action 의 variant(clone, 3사이트) ∈ {RunAway 3, AroundPosition(암묵), Trace 14, Stop 19} — get_move_action 인라인의 태그 store 전수: 3 @12412/13804/14297/14359 · 14 @12344/12961/13284/13420/13526/14112 · 19 @13940 · AroundPosition::new @12400}. self 쓰기 0 · debug 쓰기 0 · TLS 0.
```
- logic_note: ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)

### `e900b0` → `f3da50` get_input (i=208 · ⚠변경·한 줄(r20))
- 0.5.8 src: `game-ai\src\lib.rs:843` · one_line: AI 에이전트 루트(vtable Agent::get_input). 입력 지연 롤→update_state→small_action.get_input(최대 4회, 사이에 update_small_action/폴백/도주 강제)→진단 카운터·StayEvent(디버그)·freeze 감시 갱신 후 (Option<Input>, Vec<TurnEvent>) 반환
- 0.6.0 판정: **한 줄** · 패치 요지: v3 폴백: input None && 도주계(RunAway/Recall/AroundRunAway) && !우물 && 적 200000 내 && !캐스팅 → Move(healp)
- RE 정본: `2026-09-17_r20_변경99_배치F_14_디컴대조_원문.md` · 절: `2026-09-17_r20_변경99_배치F_14_디컴대조_원문.md` §12
- sig: `fn(&mut game_ai::AgentVerHamster, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> (std::option::Option<game_core::Input>, bumpalo::collections::vec::Vec< game_core::TurnEvent>)`
- consts: [{"value": 100, "src_line": 851, "meaning": "delay = gen_range(min..=max) / 100 (udiv). input_delay_* 는 1/100 틱 단위로 추정 — AthleteParameter 미열람 · 오라클 실행 확증(26차 배치D: 오라클 실행 확인(26차 D · o208.exe · AgentVerHamster::new(pub)+get_input 직접 호출) — delay = rnd.gen_range(input_delay_min..=input_delay_max)/100 예측이 12/12 호출에서 next_input_tick 실측과 일치(700..=1200 → 7~12틱))", "kind": "계수", "ev": 2}, {"value": 26, "src_line": 855, "meaning": "ProfTimer phase 인덱스(_t_sai · 'sai' 페이즈) · PHASE_NANOS/PHASE_CALLS[26]", "kind": "산출값", "ev": 4}, {"value": 132, "src_line": 906, "meaning": "PHASE_NANOS/PHASE_CALLS 배열 길이(bou
- 0.5.8 logic 전문:
```
// lib.rs:0~1004 (배치 M)
// 정본 = m14.ll 38800~41939 · 주석본 _next\reach\e900b0.ll · reach.py: 블록 255 전부 살아있음(사장 0) — NA 봉인 없음
// 시그니처: fn get_input(&mut self, rnd:&mut StdRng, player:&PlayerState, data:&OperationData) -> (Option<Input>, bumpalo::Vec<TurnEvent>)

// ── L844 입력 지연 게이트
let game = data.cache.game;                      // %54 data, %56 vtable
let tick = game.tick();                          // vtable+0x28 (호출 1/5)
if tick < self.next_input_tick {                 // self+0x2940 · icmp ult
  // L848  조기반환: (None, Vec::new_in(data.context.pool))
  sret+0 = -1(i64) ; sret+32 = 8(dangling ptr) ; sret+40 = context.pool ; sret+48..64 = 0 (cap,len)
  return;                                        // → %1330 ret(L1083)
}
// L851  입력 지연 롤 — 배치 M 유일의 rnd 직접 소비 사이트(1/1)
let p = &player.info.parameter;                  // player+0x180
let delay = rnd.gen_range(p.input_delay_min() ..= p.input_delay_max()) / 100;   // RangeInclusive<usize> · udiv 100
// L852
self.next_input_tick = game.tick() + delay;      // tick 재호출(2/5) · store self+0x2940
// L854
let mut turn_event: bumpalo::Vec<TurnEvent> = self.update_state(rnd, player, data);   // sret 32B %51 · &mut self 전체(계약: 배치 R/update_state 명세)
// L855
let _t_sai = ProfTimer::new(26);                 // prof::ENABLED(atomic i8)==0 → None(%50+16 = -1) / 아니면 Instant::now, %50 = {26, secs, nanos}
// L856  1차 소액션 입력
let mut input: Option<Input> = self.small_action.get_input(self.version, rnd, player, data, &self.positioning_score, &mut self.debug);
//        인자: (&mut self+0x2858, self+0x2910 load, rnd, player, data, &self+0x1d90, &mut self+0x0) · sret %47 32B

// ── L865  version≥2 이고 입력이 없으면 소액션 갱신 후 재시도
if self.version > 1 && input.is_none() {         // is_none = tag == -1
  // L869
  self.update_small_action(rnd, player, data);   // fastcc · &mut self(계약: 배치 R)
  // L870
  input = self.small_action.get_input(self.version, rnd, player, data, &self.positioning_score, &mut self.debug);   // 2차
  // L876  ★lapse(인지 공백) 중 무입력 → 플랜 폴백
  if self.version > 1 && input.is_none() && self.last_lapse {   // self+0x29ca bool · 세 조건 and(select 체인: version>1 → is_none → last_lapse)
    // L877
    self.plan_system.v3_lapse_noinput_rescues += 1;   // self+0x1b60
    // L878
    self.plan_system.v3_fall_back_to_passive(self.version, rnd, player, data, &mut self.debug /*%1 = self+0x0*/);   // &mut plan_system(self+0x530) · 마지막 인자 = &mut DebugFrameData(tcx 시그니처 · dereferenceable(224)) = self.debug
    // L879
    self.update_small_action(rnd, player, data);
    // L880
    input = self.small_action.get_input(self.version, rnd, player, data, &self.positioning_score, &mut self.debug);   // 3차
    // L888  그래도 없으면 도주 강제 검토
    if input.is_none() {
      // L889
      let team = player.info.team;               // player+0x930 · bounds 2(panic 157)
      if let Some(champ) = data.cache.player_champion[team][player.info.position() as usize] {   // cache+0x1e0 [2][5] · position = player+0x9c0 i32
        // L890
        let (flx, fly, frx, fry) = data.context.map.fountains[team];   // MapDef+0x6d70 + team*32
        // L895~897  분수 안이고 (v1 이거나 아직 풀피 아님) 이면 강제 안 함
        let in_rect = flx <= champ.x && champ.x <= frx && fly <= champ.y && champ.y <= fry;   // Entity+0x660/+0x668
        let force = if in_rect {
          if self.version > 1 { let in_fountain = champ.hp < champ.stat_cached.hp;  /* +0x670 < +0x628 (DI 이름 in_fountain) */  !in_fountain } else { false }
        } else { true };
        if force {
          // L898
          self.plan_system.v3_lapse_move_forces += 1;   // self+0x1b68
          // L899
          self.small_action = SmallActionPlay::RunAway(SmallActionRunAway::new(data, player, 5));   // sret 136B → drop_in_place(old small_action) → memcpy 177B → tag@0x2909 = 3
          // L900
          input = self.small_action.get_input(self.version, rnd, player, data, &self.positioning_score, &mut self.debug);   // 4차
        }
      }
    }
  }
}
// L906
drop(_t_sai);   // Option<ProfTimer>: phase(+16) != -1 이면 PHASE_NANOS[phase] += elapsed_ns(secs*1e9+nanos) · PHASE_CALLS[phase] += 1 (bounds 132 · atomicrmw monotonic)

// ── L909  전투 액션이면 마지막 전투 틱 갱신
let sa_tag = self.small_action.tag();            // self+0x2909 i8 · assume ≠10
if (15..=18).contains(&sa_tag) {                 // Attack/Skill/Skill2/Ult
  // L911
  self.last_combat_tick = game.tick();           // tick 호출(3/5) · self+0x29a0
}

// ── L914  「집(분수)에서 풀피로 서서 안 사는」 진단 (stay_home_full_nobuy)
let team = player.info.team;                     // bounds 2(panic 232)
let champ_opt = data.cache.player_champion[team][player.info.position() as usize];   // %230 · null = None
if let Some(champ) = champ_opt {
  // L915  입력이 제자리인가
  let stays = match input {
    Some(Input::Move{x, y}) => {                 // tag 0 · x@+8 y@+16
      // L917~919
      let (ax, ay) = Game::adjust_position(data.context.map, data.context.setting, x, y);   // {i64,i64} · context+0x20, +0x8
      dist2(champ.pos, (ax, ay)) < 4000001       // dx=|x1-ax| dy=|y1-ay| · dx²+dy² · 2000² +1
    }
    None => true,                                // tag -1 → 바로 L923
    _ => sa_tag == 19,                           // L921  Return/Attack/Skill*/Ult 는 small_action==Stop 일 때만
  };
  // L923
  if stays && !(champ.hp < champ.stat_cached.hp) {   // 풀피
    // L924~925
    let (flx, fly, frx, fry) = data.context.map.fountains[team];
    if flx <= champ.x && champ.x <= frx && fly <= champ.y && champ.y <= fry {
      // L926~927  구매 가능성 — rnd 는 clone 으로 전달(원본 스트림 비소비)
      let can_buy = buy_item(self.version, &mut rnd.clone(), player, game, data.context).is_some()   // Option<usize> {tag,val} · tag==1 → Some · version/game 인자는 IR 에서 poison(콜리 미사용)
                 || upgrade_item(self.version, &mut rnd.clone(), player, game, data.context).is_some();   // Option<(usize,usize)> sret 24B · tag@0 ==0 None (DI 이름 can_buy 는 이 is_none 값에 붙어 있음 — 극성은 분기로: None→계속)
      // L928
      if !can_buy {
        // L929
        self.stay_home_full_nobuy += 1;          // self+0x29a8
        // L930
        self.stay_home_full_nobuy_plan[BigPlan::freeze_plan_cls(&self.plan_system.plan)] += 1;   // self+0x4d0 [usize;6] · plan = self+0xb18
        // L932  (앞에 rnd.clone() 1회 실행되나 결과 미사용 — 콜리 인자 소거)
        let rd: bool = self.plan_system.v3_repair_done(self.version, /*rnd.clone()*/, player, data);   // 실제 IR 인자 (version, team, position i32, cache)
        // L933~935  적 위협 수 (aux closure#1 · count)
        let threat: usize = data.cache.player_champion[1 - team].iter_champions()   // Option<&Entity> 5칸 슬라이스 · None 건너뜀
            .filter(|e| data.blackboard[1 - player.info.team].is_recent_visible(game, player, e)   // 관측 블랙보드 = blackboard[1-team](docs game_core:21) · bounds 2
                     && dist2(e.pos, champ.pos) < 62500000001)   // 250000²+1
            .count();
        // L936~939  키 문자열
        let cut = |s: String| s.chars().take_while(|c| *c != ' ' && *c != '(').collect::<String>();   // closure#0 (aux) · 마스크 (c & 0x1FFFF7)==32
        let key = format!("rd={} lapse={} 적{} | {} | {} | {}", rd, self.last_lapse, threat,
                          self.plan_system.plan.debug_label(),                      // L937 String
                          cut(format!("{:?}", self.plan_system.sub_plan)),         // L938 self+0xc98 · SubPlan::fmt
                          cut(format!("{:?}", self.small_action.get_action())));   // L939 SmallAction::fmt
        // L940
        *self.stay_home_full_nobuy_stack.entry(key).or_insert(0) += 1;   // self+0x2a0 · rustc_entry: Occupied(tag≠-1)→값 포인터 / Vacant→insert_no_grow((key,0)) → 값 = 원소ptr-8 · +1
      }
    }
  }
}

// ── L948  데스매치 idle 입력 통계 (관측 전용 전역)
if game.get_game_mode().tag == 2 /*DeathMatch*/ {   // vtable+0x40 · {i64,ptr}
  // L949
  if let BigPlan::DeathMatchBattle(b) = &self.plan_system.plan {   // tag(self+0xb18) < 2 (untagged 자리 · assume ≠6)
    // L950
    if b.idle_spec_tick == game.tick() {         // self+0xc58 · tick 호출(4/5)
      // L952/961
      CNT_DM_IDLE_INPUT[input.tag + 1].fetch_add(1);   // [7] · None→0 Move→1 … Ult→6
    }
  }
}

// ── L967  입력 기회 카운터
self.input_chances += 1;                         // self+0x2958
// L968
if self.last_lapse { self.lapse_chances += 1; }  // self+0x2998
// L969
let target_dead = self.small_action.is_premise_lost(data);   // bool
// L970
if input.is_none() {
  // L971
  self.noinput_bucket[((self.last_lapse as usize) << 1) | target_dead as usize] += 1;   // self+0x410 [4] · shl 1 = *2
  // L972
  if self.last_lapse {
    // L973
    self.noinput_lapse_action[cls(sa_tag)] += 1;   // self+0x500 [6] · cls = {Trace:0, RunAway|Recall|AroundRunAway:1, Around|AroundHide|AroundRegion|Positioning|AroundPosition|AroundPositionBush|AroundBush:2, Attack|Skill|Skill2|Ult:3, LaneMinionPosition:4, Stop:5}
    // L974
    if let Some(champ) = champ_opt /*재로드 %229*/ {
      // L975~977
      let (flx, fly, frx, fry) = data.context.map.fountains[team];
      let band = if in_rect(champ) { 0 }
                 else { match data.cache.nexus[team] {   // cache+0x170 [2] · closure#2(977:68)
                          Some(n) => if dist2(champ.pos, n.pos) < 67600000001 { 1 } else { 2 },   // 260000²+1
                          None => 2 } };
      self.noinput_lapse_pos[band] += 1;         // self+0x29b0 [3]
    }
  }
}

// ── L983  StayEvent 기록 (디버그 컨텍스트 전용)
if champ_opt.is_some() && data.context.debug {  // context+0x3b bool · and(%231, debug)
  let champ = champ_opt.unwrap();                // L984 재로드
  // L985~993
  let (stays2, kind): (bool, u8) = match input {
    Some(Input::Move{x,y}) => { let (ax,ay) = Game::adjust_position(map, setting, x, y);   // L988~989
                                let s = dist2(champ.pos,(ax,ay)) < 4000001; (s, if s {1} else {2}) }   // L990 · freeze 후 비교
    None => (true, 0),                           // %540 → %579 직행 · kind 0
    _ => (false, 2),
  };
  // L993~994
  if stays2 || sa_tag == 19 /*Stop*/ {           // kind 는 위 값 유지
    // L995~996
    let (flx, fly, frx, fry) = data.context.map.fountains[team];
    let (can_buy, pos_band): (u8, u8) = if in_rect(champ) {
      // L1002~1003  분수 안: 구매 가능성(둘 다 rnd.clone())
      let cb = if buy_item(version, &mut rnd.clone(), player, game, ctx).is_some() { 1 } else { upgrade_item(version, &mut rnd.clone(), player, game, ctx).tag as u8 /*0 None/1 Some*/ };
      (cb, 0)
    } else {
      // L998~999
      (0, match data.cache.nexus[team] { Some(n) => if dist2(champ.pos,n.pos) < 67600000001 {1} else {2}, None => 2 })
    };
    // L1004  push (필드값 대부분은 배치 N 이 1005~1043 에서 계산: tick=game.tick()(5/5 · L1005) · goal · plan_label/plan_detail/sub_plan/action 문자열 · target_kind(&str: 소멸/챔피언/에픽/세르펜/정글몹/미니언/넥서스/타워 중 하나 · phi %1079/%1080) · hp_pct · near_enemy · near_ally · since_combat · in_lapse)
    self.stay_events.push(StayEvent{ tick, team, pos:(champ.x, champ.y /*배치 N %853*/), position: player.info.position(), kind, pos_band, can_buy: can_buy!=0, /* + 배치 N 값들 */ });
    //   → %1078: %24(208B) 조립 후 len==cap 이면 grow_one(self+0x1d78) → memcpy(ptr+len*208) → len+1 (self+0x1d88)
    // → 배치 N(줄 1048) %1129
  } else {
    // → 배치 N(줄 1048) %1130
  }
} else {
  // → 배치 N(줄 1048) %1129 (champ None 이면 %480 → 배치 N(줄 1080) %1133 로 직행: L1048~1075 freeze 감시를 건너뜀)
}
// 이후(배치 N): L1048~1075 freeze 감시(freeze_since/anchor/eps/plan/pos/field_action · freeze_fired) · L1080 trace_escape_* · L1082 sret = (input, turn_event) · L1083 ret

// ── 언와인딩(%97 cleanuppad): 예외 시 Option<ProfTimer> drop(phi %98=true 인 경로만) → Vec<TurnEvent> drop(%70) → caller
// ── rnd 사이트 요약(배치 M): 직접 gen_range 1회(L851, 조기반환 아니면 항상) → 이후 순서: update_state → get_input#1 → [update_small_action → get_input#2 → [v3_fall_back_to_passive → update_small_action → get_input#3 → [get_input#4]]] · clone 5회(L926, L927, L932(미사용), L1002, L1003)는 스트림 비소비

// lib.rs:1005~1083 (배치 N)
// 진입: 배치 M 의 L983 `if champ.is_some() && data.context.debug(+0x3b)` 가 참이고 L985~1004 (kind/pos_band/can_buy 계산 · %580/%631/%632) 를 지나 `self.stay_events.push(StayEvent { ... })`(L1004 · 배치 M) 의 필드식이 내 범위 1005~1043 이다. 배치 M 의 %444(L983 거짓) → L1048 로 직행 · %480(L974 champ None) → L1080 로 직행 · %573(L985 input Some(Move 이외) 경로) → L1049 로 직행.

// ─── [A] StayEvent 필드 (루트 1005~1043 · 값만 내 범위, push 는 배치 M L1004)
tick     = game.tick()                                   // L1005 · vtable+0x28 · %634
plan_label  = self.plan_system.plan.debug_label()        // L1009 · String %23 · 계약: (&BigPlan) -> String(sret 24B)
plan_detail = self.plan_system.plan.get_name()           // L1010 · String %22 · 계약: (&BigPlan) -> String
sub_plan = short(format!("{:?}", self.plan_system.sub_plan))   // L1011 · closure#5(lib.rs:1000) 인라인:
   // short(s) = s[..idx].to_string() where idx = s.chars() 순회 중 첫 c==' ' || c=='(' 의 바이트 오프셋(없으면 s.len())  — m14.ll:40242~40551 (UTF-8 디코더 인라인 · `(c & 0x1FFFF7)==32` · 새 String 할당 try_allocate_in(len,1,1) → memcpy → 원본 drop). 즉 SubPlan variant 이름만 남긴다
action   = format!("{:?}", self.small_action.get_action())       // L1012 · get_action(small_action.rs:308 인라인) 태그별: RunAway(3)/Recall(4)/AroundRunAway(8) → SmallAction::RunAway(0) · Around(5)/AroundHide(6)/LaneMinionPosition(13) → Around(2){target=+0x8} · AroundRegion(7) → AroundPosition(3){+0x10,+0x18} · Positioning(9) → Positioning(1){+0x8,+0x10} · AroundPosition(암묵) → AroundPosition(3){+0x30,+0x38 around_input.target} · AroundPositionBush(11) → AroundPosition(3){+0x8,+0x10} · AroundBush(12) → AroundPosition(3){+0x18,+0x20} · Trace(14) → Trace(4){+0x60} · Attack(15)→Attack(6){+0x8} · Skill(16)→Skill(7) · Skill2(17)→Skill2(8) · Ult(18)→Ult(9) · Stop(19)→Stop(10)
goal     = self.small_action.target_position()          // L1013 · small_action.rs:229 인라인 → Option<(u64,u64)>: RunAway/Positioning/AroundPosition/AroundPositionBush → (+0x8,+0x10) · Recall → (+0x50,+0x58) · Around/AroundHide/LaneMinionPosition → (+0x10,+0x18) · AroundBush → (+0x18,+0x20) · Trace → (+0x68,+0x70) · AroundRegion/AroundRunAway/Attack/Skill/Skill2/Ult/Stop → None
pos      = (champ.x /*L996 배치 M %589*/, champ.y /*L1014 %853*/)
hp_pct   = champ.hp * 100 / max(champ.stat_cached.hp, 1)          // L1016 · Entity+0x670 · +0x628
enemy_row = data.cache.player_champion[1 - team]                    // L1017 · %862 · [Option<&Entity>;5]
bb       = &data.blackboard[1 - team]                              // L1018 · %864/%865 · ★인덱스 = 적팀
near_enemy = enemy_row.iter().filter(|e| e.is_some() && bb.is_recent_visible(game, player, e) && dist²(e, champ) < 150000²+1).count()   // L1019 · 5회 루프(m14.ll:40891~40973) · dist² = |dx|²+|dy|² (utils 2158 인라인)
near_ally  = ally_row(=player_champion[team] %228).iter().filter(|e| e.is_some() && e.id != champ.id && dist²(e, champ) < 150000²+1).count()   // L1021 · 5회 완전 언롤(41009~41341) · ★가시성 검사 없음 · 자기 자신 제외
in_lapse = self.last_lapse                                          // L1023 · +0x29ca
since_combat = if self.last_combat_tick == usize::MAX { usize::MAX } else { game.tick().saturating_sub(self.last_combat_tick) }   // L1024~1025 · tick 재호출(%1058)
target_kind: &str = match self.small_action.get_action() /*L1026 재평가*/ {           // L1026~1040
   Around(id)|Trace(id)|Attack(id)|Skill(id)|Skill2(id)|Ult(id) => {              // L1027 · Trace → +0x60, 나머지 → +0x8
      match game.get_entity_by_id(id) /*L1032 · vtable+0x1f0*/ {
         None => "소멸",                                                       // L1032 null → anon.158
         Some(e) => match e.ty /*+0x68*/ {                                         // L1034
            Champion(13) => "챔피언"(anon.159) · Epic(5) => "에픽"(.160 · L1035) · Serpen(6) => "세르펜"(.161 · L1036)
            Jungle(4) if e.ty.info.camp_type.0 /*+0x98*/ < 2 => "정글몹"(.162 · L1037)
            Tower(2) => "타워"(.165 · L1038) · Nexus(3) => "넥서스"(.164 · L1039)
            _ (Minion/Ghoul/…/Jungle camp_type.0>=2) => "미니언"(.163 · L1040)
         } } }
   _ => ""   // ptr 1 · len 0 (대상 없는 액션)
}
// → StayEvent 208B 조립·push (배치 M L1004 · m14.ll:41412~41532) → L994 로 복귀 후 L1048

// ─── [B] 정지(freeze) 에피소드 계측 (루트 1048~1075)
if let Some(champ) = data.cache.player_champion[team][position] /*L1048 · %231 배치 M L914*/ {
   tick  = game.tick()                                                          // L1049 · %1132
   moved = utils::distance(champ.x, champ.y, self.freeze_anchor.0, self.freeze_anchor.1) > 8000   // L1050
   if moved || input.is_some() /*%442 != -1 · L633 인라인*/ || self.freeze_since == 0 {   // L1051
      self.freeze_since = tick; self.freeze_anchor = (champ.x, champ.y); self.freeze_fired = false   // L1052~1054
   } else if self.freeze_fired {                                                 // L1055
      self.freeze_ticks += 1                                                     // L1056 · 정지 누적 틱
   } else if tick.saturating_sub(self.freeze_since) >= data.context.setting.tick_per_second /*L1057 · 1초 게이트 · IR 은 `< tps` 이면 skip*/ {
      self.freeze_fired = true                                                   // L1058
      cls = small_action_cls(&self.small_action)                                 // L1059 · lib.rs:481 인라인: Trace→0 · RunAway/Recall/AroundRunAway→1 · Around/AroundHide/AroundRegion/Positioning/AroundPosition/AroundPositionBush/AroundBush→2 · Attack/Skill/Skill2/Ult→3 · LaneMinionPosition→4 · Stop→5
      self.freeze_eps[cls] += 1                                                  // L1060
      plan_cls = self.plan_system.plan.freeze_plan_cls()                         // L1061 · types.rs:121 인라인: SinglePlanBattle(idx3)/DeathMatchBattle(4)/Battle(7)→0 · PassiveLine(1)/SinglePlanLine(2)→1 · PassiveJungle(5)→2 · ForcePassive(0)/ActiveRecall(6)→3 · LineGanker(8)/LineGankCover(9)→4 · 그 외(Epic/Serpen 계열 idx10~15)→5
      self.freeze_plan[plan_cls] += 1                                            // L1061
      (flx, fly, frx, fry) = data.context.map.fountains[team]                    // L1063 · MapDef+0x6d70 + team*32
      pos_cls = if flx <= champ.x && champ.x <= frx && fly <= champ.y && champ.y <= fry { 0 /*우물 안*/ }   // L1064
                else { match data.cache.nexus[team] /*L1066*/ { None => 2, Some(n) => if dist²(champ, n) > 260000² { 2 /*필드*/ } else { 1 /*본진 근처*/ } } }   // L1066~1067
      self.freeze_pos[pos_cls] += 1                                              // L1072
      if pos_cls == 2 { self.freeze_field_action[cls] += 1 }                     // L1073 (넥서스 None 경로도 포함 · m14.ll:41882→41884)
      if self.last_lapse { self.freeze_eps_lapse += 1 }                          // L1074
      if premise_lost /*%436 · 배치 M is_premise_lost*/ { self.freeze_eps_dead_target += 1 }   // L1075
   }
   // else (1초 미만) 아무 것도 안 함
}

// ─── [C] self.update_trace_escape_abandon(data) (L1080 · lib.rs:1088~1108 인라인 · 진입 %1133)
tick = data.cache.game.tick()                                                    // L1089 · %1139 (%53 재로드)
abandon_threshold = data.context.setting.tick_per_second * 2                     // L1090 · shl 1
if let Trace(t) = &mut self.small_action /*tag==14 · L1091*/ {
   tid = t.target /*+0x60 · L1092*/
   if t.last_escape == Some(true) /*+0x8d & 1 · L1093*/ {
      if self.trace_escape_target == tid && tick >= self.trace_escape_last_tick {            // L1094
         self.trace_escape_ticks = (tick - self.trace_escape_last_tick) + self.trace_escape_ticks   // L1095
      } else { self.trace_escape_target = tid; self.trace_escape_ticks = 0 }                  // L1097
      self.trace_escape_last_tick = tick                                                     // L1100
      if !(self.trace_escape_ticks < abandon_threshold) && !t.abandoned { t.abandoned = true }   // L1101~1102 · +0x92 ← 1
   } else if self.trace_escape_target == tid {                                               // L1104
      dec = tick.saturating_sub(self.trace_escape_last_tick)                                 // L1106
      self.trace_escape_ticks = self.trace_escape_ticks.saturating_sub(dec)                  // L1107
      self.trace_escape_last_tick = tick                                                     // L1108
   }
}
// ─── [D] 반환 (L1082~1083 · %1328)
ret.1(+32..+64) = turn_event(%51) ; ret.0(+0..+32) = input(%47) ; ret void   // 32B memcpy ×2 · %1330
// 언와인드: %97 → %1332 drop Option<ProfTimer>(%50) → %70 drop bumpalo Vec<TurnEvent>(%51) · 문자열 %23/%22/%21/%17 은 각 cleanuppad 에서 drop(L1043 루트)
```
- logic_note: ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
