# apply060 batch_03.md — 반영(logic_060) 브리핑 · 함수 9

★**version = 3 런타임 확정(09-17)** — v<3/v2 경로는 명세에서 제외(사장). 태그/오프셋/vt 슬롯 = `spec_patch_060.md` §A(dispcheck 추가 행 포함). 출력 형식·절차 = `mkapply060.py` 도크스트링. 출력 = `_next/apply060/out/<old>.md`.

RE 정본 파일: 2026-09-17_r20_변경99_배치D_14_디컴대조_원문.md · 2026-09-17_r20_변경99_배치E_14_디컴대조_원문.md

### `dce220` → `ef4690` v3_epicops_buff_window (i=18 · ⚠변경·다건(r20))
- 0.5.8 src: `game-ai\src\plan_legacy\old\epic.rs:634` · one_line: 에픽(오브젝트) 국면에서 수리·세르펜징벌 목표를 선점하고, 아니면 압박 라인 변경 채팅을 대표 1명이 발화
- 0.6.0 판정: **다건** · 패치 요지: [2] 세르펜 징벌: cc5==0 이면 구 · else 계약 레코드(+0xc0/d0/d8/e0 · f1e3c0) 검사 후 리셋(+0x3c8..+0x3e4)
- RE 정본: `2026-09-17_r20_변경99_배치D_14_디컴대조_원문.md` · 절: `2026-09-17_r20_변경99_배치D_14_디컴대조_원문.md` §3
- sig: `fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &game_ai::plan_legacy::types::BigPlan) -> bool`
- consts: [{"value": 1, "src_line": 635, "meaning": "v3_epicops_repair_need 반환 태그 1 — '수리 필요 + 채팅까지'. objective=Repair 로 놓고 Chat::Repair 를 발화한다", "kind": "태그", "ev": 4}, {"value": 2, "src_line": 635, "meaning": "v3_epicops_repair_need 반환 태그 2 — '수리 필요하지만 채팅은 없음'. objective=Repair 만 놓는다", "kind": "태그", "ev": 4}, {"value": 7, "src_line": 637, "meaning": "MainObjective::Repair 의 태그값(dienum: 태그=variant 인덱스, 밀림 없음). 637줄·642줄 두 곳에서 저장", "kind": "태그", "ev": 3}, {"value": 23, "src_line": 638, "meaning": "Chat::Repair 의 태그값(24B 열거형판 Chat) · 오라클 실행 확증( Chat::Repair(0) 실값의 태그바이트 직독 = 23 (o1.txt))", "kind": "태그", 
- 0.5.8 logic 전문:
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

// [3] 압박 라인(group_line) 판정과 채팅 — 664~679
let strategy = player.strategy(rnd, data.cache.game); // 664, sret 24B 스택 로컬
let mu = strategy.morgard_use; // Strategy+0x4 (8B enum, i64 로 통째)
let group_line: Option<LineType> = v3_epic_group_line(mu, player, data); // 665
if group_line == None /* -1 */ { return false; } // 665

if self.v3_press_chat_line != group_line { // 666, Option<LineType>::ne

 // 발표자(announcer) 고르기 — 667~670, (0..5).position(closure) 가 통째로 인라인됨
 let announcer: Option<usize> = (0..5).position(|x| {
 data.cache.player_champion[team][x].is_some() // 668, null=None
 && v3_epic_formation_role(mu, Position::from_index(x), player, data)
 .is_some_and(|f| !f.is_split) // 669~670
 });
 // IR 은 슬롯 0..4 를 완전 언롤했다. 각 슬롯에서
 // ptr==null → 다음 슬롯
 // role==None(byte0==2) → 다음 슬롯
 // role.is_split==true → 다음 슬롯
 // 그 외(뭉치는 역할) → 그 인덱스가 announcer
 // 판정식 원문: xor(byte0 != 2, byte0 & 1) == is_some && !is_split

 if announcer == Some(player.info.position.as_index()) { // 671~678, PlayerState+0x9c0
 let chat = if self.v3_press_chat_line.is_none() { // epic.rs:673 의 is_none 이 인라인 (IR 에 섞여 보이는 682 는 `core/src/option.rs` 줄번호다). Chat 구성은 674/676
 Chat::Press(group_line_unwrapped, 0) // 태그 21
 } else {
 Chat::PressChange(group_line_unwrapped, 0) // 태그 22
 };
 self.chats.push(chat); // 태그@+0, LineType@+1, usize 0 @+8
 }

 self.v3_press_chat_line = group_line; // 679 — 발화 여부와 무관하게 항상 갱신
}
return false;
}
```
- logic_note: ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)

### `e5d300` → `d5ff20` try_engage_dive (i=69 · ⚠변경·한 줄(r20))
- 0.5.8 src: `game-ai\src\plan_legacy\handler\engage.rs:109` · one_line: 다이브 재합류 쿨다운·(v2+) 추격 레이스 가망 검사 후 TryKill 다이브 BattlePlan 을 생성·1틱 update 해 이탈 태세면 폐기, 아니면 반환
- 0.6.0 판정: **한 줄** · 패치 요지: `plan.screening=true; update; screening=false; if sub_goal>=2 {None}`
- RE 정본: `2026-09-17_r20_변경99_배치D_14_디컴대조_원문.md` · 절: `2026-09-17_r20_변경99_배치D_14_디컴대조_원문.md` §1
- sig: `fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, usize, std::option::Option<game_core::TowerType>, &mut game_core::DebugFrameData) -> std::option::Option<game_ai::plan_legacy::old::BattlePlan>`
- consts: [{"value": 2, "src_line": 112, "meaning": "`shl i64 %tps, 2` = tps×4 계수(시프트량 2) — dive_rejoin_cd(engage.rs:973) 인라인 → 호출 줄 engage.rs:112 (m13.ll:34327 !dbg !39431 = 973 inlinedAt 112). 같은 값 2 는 팀 bounds check(m13.ll:34360 `icmp ult i64 %41, 2`)에도. 소스 수준 값은 folded_from 참조", "folded_from": 4, "kind": "계수", "ev": 4}, {"value": 1, "src_line": 112, "meaning": "dive_rejoin_cd = last_dive_abandon_tick + 1 + tps*4 — 재합류 허용 조건 tick > cd (즉 어보트 후 4초+1틱 경과). L116 의 `version > 1` 리터럴도 1", "kind": "임계", "ev": 4}, {"value": 60, "src_line": 124, "meaning": "BattlePlanGoal::TryKill.1 에 넣는 리터럴 60 (의미는 이 본문에서 소
- 0.5.8 logic 전문:
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
  dive_committed = !matches!(plan.sub_goal, KitingBack|RunAway|End)                   // L132 (tag 3/4/7)
  if dive_committed { Some(plan) } else { None }                                       // L133
```
- logic_note: ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)

### `e07430` → `ee6d40` tower_dive_is_viable (i=70 · ⚠변경·다건(r20))
- 0.5.8 src: `game-ai\src\plan_legacy\old\fight_model.rs:935` · one_line: 타워 다이브 실행 가능 판정 — 타워 포함 내 사망틱 vs 대상 사망틱(+탈출여유) 개인 레이스, 실패 시 팀모델(resolve_fight_stake) 로 재판정
- 0.6.0 판정: **다건** · 패치 요지: v3 near_enemies 클로저 e0fc40(대상 기준 150000·최근가시) · pen=(900-9J)*(min(my_die,9999)+min(t_die,9999))/2000 · viable = t_die+escape+pen+tps/2 < my_die
- RE 정본: `2026-09-17_r20_변경99_배치D_14_디컴대조_원문.md` · 절: `2026-09-17_r20_변경99_배치D_14_디컴대조_원문.md` §10
- sig: `fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan, &game_core::Entity, bool, &mut game_core::DebugFrameData) -> bool`
- consts: [{"value": 2, "src_line": 945, "meaning": "team 인덱스 bounds(팀 2개) — 판정 아님. 또한 L1041 FightLine::Disengage 메모리태그 2 (tcxdict --enum FightLine)", "kind": "태그", "ev": 3}, {"value": 5, "src_line": 962, "meaning": "포지션 수 — L962 near_allies · L1018 dive_allies 의 `(0..5)` Range end(store i64 5, m10.ll:42964/43540) 및 player_champion 슬라이스 len(#dbg_value 만) — 판정 아님", "kind": "산출값", "ev": 4}, {"value": 1, "src_line": 951, "meaning": "enemy_team = 1 - team (L951) · version > 1 게이트(L989) · max(speed,1) 0나눗셈 방지(L991/1003) · level-1(L971) · tps>>1", "kind": "임계", "ev": 4}, {"value": 1, "folded_from": 2, "src_li
- 0.5.8 logic 전문:
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
//   where range = ae.range + 15000 + t.stat_buff.range + (t.level-1)*ae.growth_range + target.radius() + t.radius()

// L976~979
let die_tick_with_tower = check_kill_die_tick(version, rnd, data, player, champ, near_enemies.clone(), nearest_enemy_tower.into_iter().collect(), debug);
let target_die_tick     = check_kill_die_tick(version, rnd, data, player, target, near_allies.clone(), vec![], debug);

// L989~1003
let exit_margin = if version > 1 {
    let well = if player.team==1 {(0,960000)} else {(960000,0)};        // 적 우물 (L990)
    let ts = max(target.move_speed, 1);                                // L991
    let well_eta = distance(target.x,target.y, well).saturating_sub(160000) / ts;   // L992
    if target_die_tick > well_eta { return false; }                    // L993 대상이 죽기 전에 우물 도착
    match nearest_enemy_tower {                                        // L1000~1003
      Some(t) => { let reach = Effect::range(t.attack_effect.unwrap(), t, champ);   // L1001 (같은 공식, target→champ)
                   let exit_dist = reach.saturating_sub(t.distance(target));       // L1002
                   exit_dist / max(champ.move_speed, 1) }                          // L1003
      None => 0 }
} else { 0 };

// L1008
let individual_race = target_die_tick.saturating_add(exit_margin + tps/2) < die_tick_with_tower;
// L1015
if !team_model || individual_race { return individual_race; }

// 팀모델 (team_model && !individual_race)
// L1016~1018 dive_allies (s5_0/s6_0): pos in 0..5 → player_champion[my][pos] 중 dist_sq(a,target) < 200000^2+1
// L1019~1022 dive_enemies (s7_0, aux m10:56457~): 적 챔피언 e 중 e.id==target.id || (dist_sq(e,target) < 150000^2+1 && blackboard[1-my].is_recent_visible(game,ctx,player,e))
// L1024~1025
let pred: FightPrediction = resolve_fight_stake(version, rnd, data, player, champ, &dive_allies, &dive_enemies, 0u8, nearest_enemy_tower, judge_accuracy(&player.info.parameter), debug);
let mut line = pred.line;                                              // L1028 (+0x38)
if let Some(aid) = pred.rescue_ally {                                  // L1029 (+0x20/+0x28)
    let far = match game.get_entity_by_id(aid) {                       // L1030 vtable+0x1f0
        None => true,
        Some(a) => { let r = max(max_range_cached(data,target,a), max_range_cached(data,a,target));   // L1031
                     dist_sq(target,a) > (r+30000)^2 }                                              // L1032 (dbg 이름 'helping' 이 이 값에 붙어 있음)
    };
    if far { line = pred.line_absolute; }                              // L1035 (+0x39)
}
return line != FightLine::Disengage;                                   // L1041 (tag 2)
```
- logic_note: ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)

### `d639f0` → `1010060` check_serpen_giveup (i=93 · ⚠변경·한 줄(r20))
- 0.5.8 src: `game-ai\src\plan_legacy\old\serpen.rs:234` · one_line: 세르펜 목표를 포기할지 판정 — 세르펜 부재/과부하/근처·건강 머릿수 열세(판단 페널티 가산)로 결정
- 0.6.0 판정: **한 줄** · 패치 요지: take_hunt_commit = (3e4==2) ? (404==2&&cd5==1&&cd6==3) : (3e0==3)
- RE 정본: `2026-09-17_r20_변경99_배치D_14_디컴대조_원문.md` · 절: `2026-09-17_r20_변경99_배치D_14_디컴대조_원문.md` §2
- sig: `fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bool`
- consts: [{"value": 0, "src_line": 235, "meaning": "TutorialType {0,5,7,8} = serpen_exists. 밖이면 true(포기)", "kind": "태그", "ev": 4}, {"value": 5, "src_line": 246, "meaning": "JungleType::Serpen 태그(camp 변수 · camp_pos 인자)", "kind": "태그", "ev": 4}, {"value": 1, "src_line": 246, "meaning": "team_plan.objective@tag == 1 = Some(MainObjective::Serpen) (take_hunt_commit 인라인, team_plan.rs:249)", "kind": "태그", "ev": 4}, {"value": 3, "src_line": 246, "meaning": "Serpen.phase == 3 = ObjectPhase::Hunt", "kind": "태그", "ev": 4}, {"value": 100, "src_line": 249, "meaning": "hp_ratio = serpen.hp*100/max_hp", "kind": "계수",
- 0.5.8 logic 전문:
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
L277 near_enemy = (L268 과 동일 필터).count()
L281 if is_line_phase(ctx) {
L282     healthy_ally  = (1..5).filter(|i| player_champion[team][i].map_or(false, |c| c.hp*100/c.max_hp > 49)).count()   // Top(슬롯0) 제외
L283     healthy_enemy = (1..5).filter(|i| player_champion[enemy][i] … hp% > 49).count()
L284     penalty = macro_judgement_penalty(version, player)
L285     if healthy_ally + 1 + penalty < healthy_enemy {
L286         return near_ally + penalty < near_enemy
         }
         return false
     } else {
L288     healthy_ally  = player_champion[team].iter_champions().filter(hp% > 49).count()
L289     healthy_enemy = player_champion[enemy].iter_champions().filter(hp% > 49).count()
L290     penalty = macro_judgement_penalty(version, player)
L291     if healthy_ally + 1 + penalty < healthy_enemy {
L292         return near_ally + penalty < near_enemy
         }
         return false
     }
(보조: is_bottom_side(map_regions.rs:28~30) = (setting.height - y <= x) || is_near_mid_line(ctx,x,y);
 is_recent_visible(blackboard.rs:348~354) = 현재 가시 || (해당 적의 last_visible[pos] + 120 >= tick);
 macro_judgement_penalty(objective_helpers.rs:16~18) = ceil(max(400 - judge_accuracy,0)/125) ∈ 0..=4)
```
- logic_note: ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)

### `dffa10` → `e80960` buff_value_v54 (i=133 · ⚠변경·다건(r20))
- 0.5.8 src: `game-ai\src\buff_value.rs:85` · one_line: BuffState 한 개를 받는 recv 에게 얼마나 가치 있나 — 공격(DPS 증가→앵커 적 HP가치 환산)·쿨감·힐·피해경감·기동·위기(undying/cc_immune/toughness) 항을 합산해 0..160
- 0.6.0 판정: **다건** · 패치 요지: 시그니처 +version,+champ_incoming · v3 crisis: undying 이면 !(crisis&&champ_incoming>0&&crisis.0) → epic_incoming<1 → 0 · dur*epic_incoming<hp → 0
- RE 정본: `2026-09-17_r20_변경99_배치D_14_디컴대조_원문.md` · 절: `2026-09-17_r20_변경99_배치D_14_디컴대조_원문.md` §8
- sig: `fn(&game_core::BuffState, &game_core::Entity, &game_core::OperationData, &game_core::PlayerState, &game_ai::ScoreParameter, std::option::Option<&game_ai::DefensiveCrisis>, i64, i64, i64, i64) -> i64`
- consts: [{"value": 1, "src_line": 90, "meaning": "BuffType 메모리태그 1 = Time (icmp eq i32 %33, 1 — m10.ll:30063). 리터럴, shl 아님 · 오라클 실행 확증(22차 배치D: 오라클 실행 확인(22차 배치D o133.rs · #[link_name] 직접 호출 · 실전 게임 cache/data · near_enemies 비움(앵커 None) · 케이스당 프로세스 1개 · 25/25 MATCH vs 독립 재구현) — case13 duration=Time(120) → window 2 · 나머지 Permanent(tag 0) → 6)", "kind": "태그", "ev": 2}, {"value": 6, "src_line": 94, "meaning": "window = min(dur_sec, 6) — 버프 지속을 최대 6초로 캡(llvm.umin 30089) · Time 이 아닌 버프(Permanent/WithShield)는 dur_sec=6 (phi 30097) · 오라클 실행 확증(22차 배치D: 오라클 실행 확인(22차 배치D o133.rs · #[link_name] 직접 호출 · 실전 게임 c
- 0.5.8 logic 전문:
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
L134: if buff.crit_chance > 0        { L136: delta_dps += a_ps * crit_chance / 100 }
L138: if buff.magic_power > 0        { L139: delta_dps += (s_ps+u_ps) * magic_power / max(stat.magic_power,1) }
L141: if buff.magic_power_mult > 0   { L142: delta_dps += (s_ps+u_ps) * magic_power_mult / 100 }
L147: if e_n > 0 && buff.defence_penetration != 0 {
L149:   def_after = max(100 - pen, 0) * e_def / 100
L150:   delta_dps += a_ps * (e_def - def_after) / (max(def_after,-99) + 100) }
L152: if e_n > 0 && buff.magic_resistance_penetration != 0 {
L153:   mr_after = max(100 - mpen, 0) * e_mr / 100
L154:   delta_dps += (s_ps+u_ps) * (e_mr - mr_after) / (max(mr_after,-99) + 100) }
L156: if buff.dot_amplify != 0       { L158: delta_dps += dot_amplify * s_ps / 200 }
L160: if buff.range != 0 {
L162:   atk_range = max(recv.attack_effect.as_ref().map(|e| e.range(recv)).unwrap_or(1), 1)   // Effect::range = range + (level-1)*growth_range + recv.stat_buff_cached.range
L163:   delta_dps += a_ps * buff.range / atk_range }   // 오버플로 검사 있음(i64 sdiv)
L165: if buff.radius_mult > 0        { L167: delta_dps += s_ps * radius_mult / 200 }

§3 offense_total — 창 동안의 추가 피해 (L170~200)
L170: offense_total = delta_dps * window
L172: if on_attack_damage > 0 { L173: hits = max(window*tps / max(recv.attack_cooltime(),1), 1); L174: offense_total += hits * on_attack_damage }
L177: if e_n > 0 && buff.base_attack_enemy_max_hp_damage != 0 { L178: hits = (같은 식); L179: offense_total += hits * (val * e_hp / 100) }
L181: if e_n > 0 && buff.skill_enemy_max_hp_damage != 0 { L182: offense_total += val * e_hp / 100 }
L184: if buff.self_max_hp_damage != 0 { L185: hits = (같은 식); L186: offense_total += hits * (val * stat.hp / 100) }
L188: if buff.heal_reduce != 0 {
L190~193: e_heal = Σ_{pos 0..5} (sum(cache[적팀][pos].skill_heal_sec[0..5]) + sum(…skill2_heal_sec[0..5])) / 5   // ult_heal_sec 미포함
L195:   offense_total += (e_heal * heal_reduce / 100) * window }
L197: if buff.damaged_amplify != 0 {
L199:   near_allies = iter_champions(player.info.team).filter(|e| dist²(e,recv) ≤ 120000²).count()   // 자신 포함
L200:   offense_total += ((a_ps+s_ps+u_ps) * damaged_amplify / 100) * min(near_allies,3) * window }

§4 앵커 적으로 환산 (L204~245)
L204: score = 0
if offense_total > 0 {
L219:   engage_of = |c| c.attack_effect.map(|e| e.range(c)).unwrap_or(0) + c.radius() + c.stat_cached.move_speed*120   // L220~221; radius() = radius*(radius_mult+100)/100 (mult==0 이면 radius)
L223:   recv_engage_range = engage_of(recv)
L224:   caster = cache.player_champion[player.info.team][player.info.position].unwrap()   // None 이면 패닉(unwrap_failed, Location 224)
L225:   caster_engage_range = engage_of(caster)
L226~235: anchor: Option<(ev, ehp)> = parameter.near_enemies.iter()
            .filter(|ep| { let Some(e)=game.get_entity_by_id(ep.id) else {return false};   // L227
                           r = recv_engage_range + e.radius(); rc = caster_engage_range + e.radius();   // L228~229
                           dist²(recv,e) ≤ r² || dist²(caster,e) ≤ rc² })                     // L230 (IR 순서: recv 먼저)
            .map(|ep| (champion_hp_value(data, parameter, ep), game.get_entity_by_id(ep.id).map(|e| e.hp).unwrap_or(0)))   // L232~233
            .filter(|(_, ehp)| *ehp > 0)                                                        // L234 (closure#3 = aux m10.ll:55855)
            .max_by_key(|(ev, _)| *ev)                                                          // L235 (동점이면 뒤 원소)
L237:   if let Some((ev, ehp)) = anchor { score = offense_total * ev / max(ehp,1) }
L238:   else if epic_incoming > 0        { score = offense_total * recv_hp_value / max(stat.hp,1) }   // 앵커 없으면 에픽전투 중일 때만 자기 HP 가치
L242:   if buff.attack_speed_mult > 0 && data.context.debug { L243: print!("\rBUFFV54 tick={} recv={} aps={} mult={} off={} ne={} engage={} anchor={:?}\n", game.tick(), recv.id, a_ps, attack_speed_mult, offense_total, near_enemies.len(), recv_engage_range, anchor) }   // 관측 전용
}

§5 쿨감 (L251~264)
L251: if buff.skill_cooldown_mult > 0 || buff.ult_cooldown_mult > 0 {
L252~254: covered = iter_champions(player.info.team).filter(|e| dist²(e,recv) ≤ 60000²).count()
L255:   if covered > 1 {   // 자신 외 아군이 1명 이상 붙어 있을 때만
L257:     cd_gain = if scm > 0 { (s_ps+u_ps) * scm / (scm+100) } else 0            // L259
L261:     if ucm > 0 { cd_gain += u_ps * ucm / (ucm+100) }                          // L262
L264:     score += window * recv_hp_value * cd_gain / max(recv.hp,1) } }

§6 힐 (L270~279)
L270: heal_total = 0
L271: if buff.vamp > 0     { L272: heal_total = (a_ps * vamp / 100) * window }
L274: heal_total += if buff.hp_regen > 0 { window * hp_regen } else 0
L277: heal_realized = min(max(stat.hp - recv.hp, 0) + incoming, heal_total)   // 잃은 체력+예상피격만큼만 실현
L278: if heal_realized > 0 { L279: score += heal_realized * recv_hp_value / max(recv.hp,1) }

§7 피해 경감 (L284~315)
L284: if incoming > 0 {
L285:   mitigated = 0; def = stat.defence; mr = stat.magic_resistance
L288:   d_def = buff.defence + def * defence_mult / 100
L289:   if d_def > 0 { L291: mitigated = d_def * (incoming/2) / max(def + 100 + d_def, 1) }
L293:   d_mr = buff.magic_resistance + mr * magic_resistance_mult / 100
L294:   if d_mr > 0  { L295: mitigated += d_mr * (incoming/2) / max(mr + 100 + d_mr, 1) }
L297:   if buff.damaged_reduce != 0             { L298: mitigated += damaged_reduce * incoming / 100 }
L300:   if buff.base_attack_damaged_reduce != 0 { L301: mitigated += val * (incoming/2) / 100 }
L303:   if buff.skill_damaged_reduce != 0       { L304: mitigated += val * (incoming/2) / 100 }
L306:   if buff.damage_reflect != 0             { L307: mitigated += reflect * incoming / 100 }
L310:   d_hp = buff.hp + stat.hp * hp_mult / 100
L311:   mitigated += if d_hp > 0 { min(incoming, d_hp) } else 0
L314:   if mitigated > 0 { L315: score += mitigated * recv_hp_value / max(recv.hp,1) } }

§8 기동 (L320~334)
L320: if buff.move_speed_mult > 0 || buff.ignore_wall {
L321~322: act = cache.player_by_champion_id(recv.id).and_then(|p| data.blackboard[p.info.team].small_actions[p.info.position])   // team bounds<2
L327:   mobility_realized = match act { Some(Trace(_)) => L328 !parameter.near_enemies.is_empty(), Some(RunAway) => incoming > 0, _ => false }
L332:   if mobility_realized { L333: ms = move_speed_mult + (ignore_wall ? 10 : 0); L334: score += ms * recv_hp_value / 100 } }

§9 위기 (L339~355) — crisis: Option<&DefensiveCrisis{die_imminent,cc_threat}>
L339: if let Some(c) = crisis && buff.undying   { L340: score += if c.die_imminent { recv_hp_value } else 0 }
L344: if let Some(c) = crisis && buff.cc_immune { L346: if c.cc_threat { if c.die_imminent { L347: score += recv_hp_value } else { L349: score += recv_hp_value / 3 } } }
L353: if let Some(c) = crisis && buff.toughness != 0 { L354: if c.cc_threat { L355: score += toughness * recv_hp_value / 200 } }

L360: return clamp(score, 0, 160)   // smax 0 → umin 160 → range(0,161)
```
- logic_note: ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)

### `eaeda0` → `f6bda0` EpicHuntSubPlan::action_candidates (i=198 · ⚠변경·한 줄(r20))
- 0.5.8 src: `game-ai\src\plan_legacy\sub_plan\epic_hunt.rs:200` · one_line: EpicHunt(모가드 사냥) 서브플랜의 후보 행동 생성: need_recall 갱신→귀환/규율 단일후보 조기반환→전투·소환수·정글 공격후보 필터/정리→get_move_action 이동후보 합성(J: 200~429) → 이후 타워·에픽 위치·최적 이동 선별(K: 432~515)
- 0.6.0 판정: **한 줄** · 패치 요지: get_move 전 `if tp.cc7 { s=efb5b0(tp,1,team0); if dist²(epic,me) > dist²(me,s) { push AroundPosition::new(s,5); skip } }`
- RE 정본: `2026-09-17_r20_변경99_배치D_14_디컴대조_원문.md` · 절: `2026-09-17_r20_변경99_배치D_14_디컴대조_원문.md` §13
- sig: `fn(&mut game_ai::plan_legacy::sub_plan::EpicHuntSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallAct`
- consts: [{"value": 2, "src_line": 202, "meaning": "player_champion 1차원(팀 2) bounds · Tower 태그(closure#5 target.ty != 2) · visible_state 첨자 bounds", "kind": "태그", "ev": 4}, {"value": 5, "src_line": 206, "meaning": "EntityType::Epic 메모리태그(L206) · 모든 생성자의 end_delay=5 · player_champion 2차원 5 · 오라클 실행 확증(25차 배치D: 오라클 실행 확인: _verify25/D/oracle/o198.rs (pub 직접 호출 · 케이스당 프로세스 1개 · run198.py 18케이스 · o198_truth.log) 18/18 MATCH — EntityType::Epic 태그 5 게이트(에픽 스폰 케이스 E1~E9 ty_tag=5 실측) · end_delay 5(RunAway/Trace elem end_delay 필드=5))", "kind": "태그", "ev": 2}, {"value": 3, "src_line": 207, "meaning": "dmg*3 >= ch
- 0.5.8 logic 전문:
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
- logic_note: ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)

### `e8d6f0` → `f3adb0` AgentVerHamster::update_state (i=218 · ⚠변경·다건(r20))
- 0.5.8 src: `game-ai\src\lib.rs:556` · one_line: 틱당 AI 상태 갱신 루트: 이벤트→(lapse 게이트)플랜 update→소액션 update_state→(전제상실/종료 ∧ !can_skip_eval)이면 update_small_action, 아니면 extend_action. 반환 Vec 은 항상 빈 것
- 0.6.0 판정: **다건** · 패치 요지: awareness_lapse 8번째 인자 m=1000+min(+0x490,100)²/20 · plan update 뒤 d52c00(TRAIT_AUD) · agent+0x29dd 플래그
- RE 정본: `2026-09-17_r20_변경99_배치D_14_디컴대조_원문.md` · 절: `2026-09-17_r20_변경99_배치D_14_디컴대조_원문.md` §5
- sig: `fn(&mut game_ai::AgentVerHamster, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bumpalo::collections::vec::Vec< game_core::TurnEvent>`
- consts: [{"value": 1, "src_line": 569, "meaning": "version > 1 ⟹ v2+ 에서만 lapse 를 플랜 update 에 전달(update_plan_lapse). L589 동일 조건으로 is_premise_lost 검사", "kind": "임계", "ev": 4}, {"value": 10, "src_line": 806, "meaning": "can_skip_eval: 마지막 평가 후 10틱 미만이어야 재평가 생략 가능(재평가 예산 창) · 오라클 실행 확증(26차 배치G: 오라클 실행 확인(26차G o218 · _verify26/G/oracle/o218_truth.log): last_eval_tick=2995(3000<3005) → extend(+0x28a0=3000 · rnd 미소비) / 2990(3000<3000 거짓) → update_small_action(태그 4→5 · last_eval_tick=3000))", "kind": "태그", "ev": 2}, {"value": 2, "src_line": 801, "meaning": "GameMode 태그 2 = DeathMatch ⟹ can_skip_eval 항상 false.
- 0.5.8 logic 전문:
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
  { _t_sus = ProfTimer::start(28)                               // L579
    self.small_action.update_state(rnd, player, data, &mut self.debug)   // L580 (+0x2858)
    drop(_t_sus) }                                              // L581
  need_new = (self.version > 1 && self.small_action.is_premise_lost(data))     // L589 — 단락: v≤1 이면 is_premise_lost 미호출
             || self.small_action.is_end(rnd, self.version, player, data)        // L590 — premise_lost 가 true 면 is_end 미호출
  if need_new:
    // premise_lost 경로는 can_skip_eval 을 건너뛰고 바로 재평가(%165→%285). is_end 경로만 아래 검사
    if premise_lost || !self.can_skip_eval(player, data):        // L591 (lib.rs:796~824 인라인)
      _t_usa = ProfTimer::start(29)                              // L594
      self.update_small_action(rnd, player, data)                // L595 (별도 명세)
      drop(_t_usa)                                               // L596
    else:
      self.small_action.extend_action(game.tick())               // L592 → small_action.rs:445: 현 variant 의 start_tick = tick (Recall +0x48 · Trace +0x58 · 나머지 이동계 +0x0 · 캐스트/Stop 은 무시)
  drop(_t_sa)                                                   // L598
  if data.context.debug {                                       // L600 (+0x3b)
    self.debug.merge(self.big_debug.clone())                    // L601
    self.debug.merge(self.small_debug.clone())                  // L602
    if let Some(pf) = self.small_action.path_finder() {         // L604 (null 검사)
      pf.draw_debug(player, data, &mut self.debug) } }          // L605
  return res                                                    // L609 (빈 Vec)

---- can_skip_eval(&self, player, data) -> bool  (lib.rs:796~824 · 인라인 · L591 문맥) ----
  tick = game.tick()                                            // L797 (vtable+0x28)
  if game.get_game_mode().tag == 2 (DeathMatch): return false   // L801 (vtable+0x40)
  if !(tick < self.last_eval_tick + 10): return false           // L806 (+0x2948) — 재평가 예산 10틱
  if self.small_action.is_action_complete(): return false       // L811 → small_action.rs:463: tag∈{15,16,17,18}(Attack/Skill/Skill2/Ult) && payload.is_act(+0x10)
  champ = data.cache.player_champion[player.info.team][player.info.position].unwrap()   // L816 (team ult 2 bounds · None→unwrap_failed)
  if champ.hp < self.last_eval_hp: return false                 // L817 (+0x670 vs +0x2950) — 피해 입었으면 재평가
  current_enemies = count_nearby_enemies(champ, player, data)   // L822 (u16 마스크)
  return current_enemies == self.last_eval_nearby_enemies       // L823 (+0x29c8) — 근접 적 구성 불변이면 생략 가능

※ 분기 극성 근거: %165 `br %164 → %285(재평가) / %166`, %171 `br %170 → %195(can_skip 검사) / %172(아무것도 안 함)`, %258 `eq → %259(extend) / %285(재평가)`.
※ 개발자 주석(_docs game_ai.txt:9~11): is_premise_lost 는 '재평가 예산(can_skip_eval)으로 연장해서는 안 되는 유일한 조건' — IR 과 일치(premise_lost 경로는 can_skip_eval 우회).
```
- logic_note: ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)

### `e8fd70` → `f3d660` AgentVerHamster::item_v26 (i=225 · ⚠변경·한 줄(r20))
- 0.5.8 src: `game-ai\src\lib.rs:1149` · one_line: v26 빌드 경로: 활성 아이템빌드의 build_slot 번째 목표 아이템을 잡고, 인벤 현재템→목표로 가는 다음 아이템(또는 빌드경로 첫 아이템)을 골 조건까지 검사해 반환
- 0.6.0 판정: **한 줄** · 패치 요지: 활성템 `count>=4`(구 len>=3) && !tier → None
- RE 정본: `2026-09-17_r20_변경99_배치D_14_디컴대조_원문.md` · 절: `2026-09-17_r20_변경99_배치D_14_디컴대조_원문.md` §4
- sig: `fn(&mut game_ai::AgentVerHamster, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::GameContext, usize, std::option::Option<usize>) -> std::option::Option<usize>`
- consts: [{"value": 0, "src_line": 1151, "meaning": "nth 카운터 종료(accum==0) · Option 태그 None · L1158 path.len()==0 · L1161 tier()==0 판정", "kind": "태그", "ev": 4}, {"value": 2, "src_line": 1161, "meaning": "active_inventory_item_indices(player,item_list).len() > 2 (samesign ugt 2) ⟹ 활성 인벤 아이템이 3개 이상이면 tier 0 아이템은 거부", "kind": "임계", "ev": 4}, {"value": -1, "src_line": 1157, "meaning": "Option<Vec<usize>> None 니치 — sret+0(cap)==usize::MAX 면 random_item_build_path 가 None", "kind": "센티널", "ev": 4}]
- 0.5.8 logic 전문:
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
  inv_active: Vec<usize> = active_inventory_item_indices(player, item_list)   // lib.rs:542 인라인 + from_iter(aux m06) · 술어(aux m14 call_mut): items[i] 가 item_list 의 어떤 li 와 key 동일(len 동일 && memcmp==0) && li.is_active()
  if inv_active.len() > 2 && item_list[next_item].tier() == 0 { return None }   // tier = vtable+0x70 · 색인은 panic_bounds_check(item_list.len)
  drop(inv_active)
  // L1165
  if player.info.gold >= item_list[next_item].price() { Some(next_item) } else { None }   // price = vtable+0x68 · uge

★rnd 소비: 본문 직접 gen_range 0회. 호출당 정확히 하나 — inventory_index Some → random_item_next_toward_target 1회 / None → random_item_build_path 1회 (내부 gen_range 수는 game_core 경계 · 미명세).
★None 이 되는 경로 9개(38792 phi): 빌드 소진(%22/%43/%59) · inv bounds(%64) · next_toward None(%71) · build_path None(%116) · path 빈(%120) · 3개+ && tier0(%143) · gold 부족(%153 false).
```
- logic_note: ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)

### `e05e70` → `ee4e70` resolve_join_stake (i=49 · ⚠변경·다건(r20))
- 0.5.8 src: `game-ai\src\plan_legacy\old\fight_model.rs:680` · one_line: anchor(적) 주변 교전에 내가 6초 내 합류할 때의 '저울' — 아군만/아군+나 두 판을 resolve_fight_full 로 돌려 차분 FightPrediction 을 낸다
- 0.6.0 판정: **다건** · 패치 요지: 인자 (committed:i8, horizon_sec) · horizon=tps*horizon_sec · cc1 게이트 bb[+0x4d8] 6초 · resolve_fight_full 에 committed 전달·bias 0
- RE 정본: `2026-09-17_r20_변경99_배치E_14_디컴대조_원문.md` · 절: `2026-09-17_r20_변경99_배치E_14_디컴대조_원문.md` §7
- sig: `fn(usize, &mut rand::rngs::std::StdRng, &game_core::OperationData, &game_core::PlayerState, &game_core::Entity, &game_core::Entity, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> std::option::Option<game_ai::plan_legacy::old::FightPrediction>`
- consts: [{"value": 2, "src_line": 682, "meaning": "version < 2 면 None. v2 이상 전용 판정", "kind": "임계", "ev": 4}, {"value": 6, "src_line": 685, "meaning": "horizon = tick_per_second * 6 = 6초. 아군 도착시간 상한(umin)·내 도착시간 컷", "kind": "임계", "ev": 4}, {"value": 1, "src_line": 688, "meaning": "enemy_team = 1 - player.info.team (아군팀 인덱스는 info.team 그대로)", "kind": "인덱스", "ev": 4}, {"value": 5, "src_line": 703, "meaning": "BigGoal 메모리태그 5 = Battle. 아군 i 의 big_goal 이 Battle 이고 focus 가 Some 일 때만 '같은 적을 보고 있나' 대조", "kind": "태그", "ev": 4}, {"value": 1, "src_line": 710, "meaning": "sp = max(move_speed, 1) — 0 나눗셈 방지(llvm.um
- 0.5.8 logic 전문:
```
fn resolve_join_stake(version, rnd, data, player, champ, anchor, team_plan, debug) -> Option<FightPrediction>

[L682] if version < 2 { return None }                      // 41062
[L685] horizon = data.context.setting.tick_per_second * 6    // 6초
[L688] cache = data.cache ; my_team = player.info.team(+0x930) ; enemy_team = 1 - my_team (≥2 면 panic_bounds_check)
[L687~691] enemies: Vec<&Entity,&Bump> = cache.player_champion[enemy_team] (iter_champions, Some 만)
      .filter(|e| {                                   // ★클로저0 = aux m10.ll 56030~56141 (Entity::call_mut 심)
         [L689] e.id == anchor.id                      // anchor 자신은 무조건 포함
              || ( distance_sq(e, anchor) < 22500000001   // ≤150000
         [L690]   && data.blackboard[enemy_team].is_recent_visible(cache.game, player, e)
         [L691]   && !is_ignored_well_enemy(version, player, e) )
                  //  is_ignored_well_enemy 인라인 = (e.team(+0x0) == TeamType::Player(enemy_team)) && path_finder::is_enemy_well_danger(version, player, e.x, e.y)
      }).collect_in(pool)
[L692~693] if enemies.is_empty() { return None }

[L696] allies: Vec<&Entity> = new_in(pool) ; [L697] arrivals: Vec<usize> = new_in(pool)
      nearest_bound: Option<&Entity> = None
      my_pos = player.info.position 태그(+0x9c0)
[L699] for i in 0..5 {
[L700]   if i == my_pos { continue }                          // 나 자신 슬롯 제외
[L701]   if team_plan.ally_battle_stop_tick[i].is_some() { continue }   // 태그(+0x0+16i) != 0
[L702]   a = cache.player_champion[my_team][i] ; if a.is_none() { continue }   // my_team≥2 면 panic
[L703]   bg = data.blackboard[my_team].big_goal[i].1
         focus_match = (bg 태그 == 5 /*Battle*/) && bg.focus.is_some()
[L704]                 && enemies.iter().any(|e| e.id == bg.focus.unwrap())
[L705]   bound = ally_is_bound(version, rnd, data, player, a, &enemies, debug)
[L706]   if !focus_match && !bound { continue }               // 둘 다 아니면 후보 아님
[L707]   if bound {                                           // (focus_match 만 참이면 nearest 갱신 없이 통과)
            nearest_bound = min_by_key((distance_sq(a,anchor), a.id)) — 기존 nb 와 비교:
              dist 같으면 a.id < nb.id, 아니면 dist_a < dist_nb 일 때 a 로 교체 (None 이면 a)
         }
[L710]   sp = max(a.move_speed(+0x640), 1)
[L711]   reach = a.attack_effect.map(|ef| ef.range(a)).unwrap_or(0)
              // Effect::range 인라인 = stat_buff_cached.range(+0x438) + range(+0x4a0) + (level(+0x5c8)-1)*growth_range(+0x4a8)
[L712]   allies.push(a)
[L713]   arrivals.push( min(horizon, sat_sub(Entity::distance(a, anchor), reach) / sp) )
      }
[L715] if allies.is_empty() { return None }

[L719] my_sp = max(champ.move_speed, 1)
[L720] my_reach = champ.attack_effect.map(|ef| ef.range(champ)).unwrap_or(0)   // 같은 식
[L721] my_arrival = sat_sub(Entity::distance(champ, anchor), my_reach) / my_sp
[L722] if my_arrival > horizon { return None }                 // ⚠아군은 umin 으로 잘리지만 나는 컷

[L726] tower = cache.iter_towers_without_nexus(enemy_team)      // Chain<Flatten<[Option<&Entity>;6]>, Copied<slice>> 120B
         .filter(|t| t.can_target(+0x6b9) && t.block_target_tick(+0x6a0) == 0)   // 클로저 s2_0 = aux 56144~56165 (배열부는 본문 인라인 41272~41330, 슬라이스부는 try_fold 심)
[L727]   .min_by_key(|t| distance_sq(t, anchor))               // 첫 원소 키는 본문 인라인(41341~41360), 나머지는 Map::fold 심(m06.ll 30568~30820)
[L728]   .filter(|t| {
[L729]      r = t.attack_effect.unwrap()/*None 이면 unwrap_failed*/ .range(t) + 15000 + anchor.radius() + t.radius()
                // Entity::radius = radius_mult(+0x470)==0 ? radius(+0x680) : radius*(100+mult)/100
[L730]      Entity::distance(t, anchor) <= r
         })                                                    // 아니면 tower = None
[L732] judge = AthleteParameter::judge_accuracy(&player.info.parameter(+0x180))

[L733] without  = resolve_fight_full(version, data, champ, &allies, &enemies, committed_dir=0, tower, judge, &arrivals, baseline=0)
[L734] with_me  = allies.clone() ; [L735] with_arr = arrivals.clone()
[L736] with_me.push(champ) ; [L737] with_arr.push(my_arrival)
[L738] absolute = resolve_fight_full(version, data, champ, &with_me, &enemies, 0, tower, judge, &with_arr, baseline=0)
[L739] diff     = resolve_fight_full(version, data, champ, &with_me, &enemies, 0, tower, judge, &with_arr, baseline=without.net_value(+0x30))
[L740] diff.line_absolute(+0x39) = absolute.line(+0x38)
[L741] diff.rescue_ally(+0x20) = nearest_bound.map(|a| a.id)
[L742] return Some(diff)

※ 부작용: 게임 구조체 쓰기 없음. rnd/debug 는 ally_is_bound 로 &mut 전달. resolve_fight_full 은 TLS ResolveFightCache 메모 래퍼(범위 밖)라 캐시 상태가 바뀔 수 있다.
```
- logic_note: ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
