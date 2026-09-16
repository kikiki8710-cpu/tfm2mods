# 배치 G — 변경 99 중 티어 2/3 7 (0.5.8 RVA → 0.6.0 RVA · exe 정규화 정렬 힌트 · 0.5.8 명세 요지)

정규화 정렬(`--norm`)이 이미 흡수한 것: PlayerState +0xd0 · Blackboard 0x2e8→0x5c8 · SmallActionPlay 태그/idx 재번호 · BattleSubPlanGoal 재번호 · Effect vt 슬롯(4 삽입) · LPH 오프셋 6. 아래 「잔여」 가 정렬로 못 닫은 차이.

## `ccc010` → `e60420` sub_plan  (591→613B · Δ+22)
- 명령 143→146 · 정렬 121 · exe 판정 ⚠정렬 불가(미확정) · 정규화 4 · 블록이동 3 · 스택슬롯 8 · 콜리 주의 1
- 잔여 구조: 구 22명령(test|r15, r15…) / 신 25명령(mov|rdi, qword ptr [r14 + I]…) 짝 없음
- 잔여 분기: ccc0fa → ccc0ff/e6050c ; ccc127 → ccc155/e6056d ; ccc131 → ccc155/e6056d ; ccc1a7 → ccc208/e60629 ; ccc1af → ccc208/e60629
- 잔여 변위: r14+0x628→0x670 · rax+0x8→0x20 · rax+0x10→0x28 · rax+0x12→0x2c
- 콜리 주의: e7a8c0→f27140 변경
- 정규화 흡수: 오프셋표 [rbx+930→a00] · 오프셋표 [rbx+9c0→a90] · SmallActionPlay 태그 1→0 · SmallActionPlay 태그 0→1
- 0.5.8 명세 #7 `epic_hunt_and_battle__sub_plan` src game-ai\src\plan_legacy\old\epic\hunt_and_battle.rs:28 · one_line: 에픽 사냥+교전 플랜의 서브플랜 선택 — 귀환/부시은신/에픽사냥 3택
- 0.5.8 logic(앞 1500자):
```
// L29
team = player.info.team // PlayerState+0x930
if team >= 2 { panic_bounds_check(team, 2) }
pos = player.info.position.as_index() // PlayerState+0x9c0, 0..=4 (인라인, 바운드체크 없음)
champ = data.cache.player_champion[team][pos] // cache+0x1e0, [5]ptr stride 40 + pos*8
if champ == null { option::unwrap_failed() } // .unwrap()

// L31
if champ.stat_cached.hp == 0 { panic_const_div_by_zero() }
hp_ratio = champ.hp * 100 / champ.stat_cached.hp // usize 나눗셈(백분율)

// L32
mode = (*data.cache.game.vtable[0x40])(data.cache.game.ptr) // get_game_mode -> GameMode
if discriminant(mode) != 0 { option::unwrap_failed() } // as_moba().unwrap(), 0=Moba
moba = payload(mode) // &MobaMode
live = moba.jungle_runner.epic.live_list // Vec<usize> (MobaMode+0x198)
epic: Option<&Entity> =
 if live.len == 0 { None }
 else { (*data.cache.game.vtable[0x1f0])(data.cache.game.ptr, live[0]) } // get_entity_by_id
 // 소스상 live_list.get(0).and_then(|e| data.cache.game.get_entity_by_id(*e))

// L33
(lx, ly, rx, ry) = data.context.map.fountains[team] // MapDef+0x6d70, stride 32

// L34
is_in_heal_area = (champ.x >= lx && champ.x <= rx)
 && (champ.y >= ly && champ.y <= ry) // x는 lx..rx, y는 ly..ry (단축평가: x 실패면 false)

// L35
can_upgrade_item = game_ai::upgrade_item(version, rnd, player,
 data.cache.game /*fat ptr 2개*/,
 data.context).is_some()
 // 반환 Option<(usize,usize)>(24B) 의 판별자 != 0 을 is_some 으로 씀

// L36 — 귀환 판정
if epic.is_some_and(|e| e.hp == e.stat_cached.hp) // 에픽이 살아있고 풀피(=아직 아무도 안 때림)
 && ( hp_ratio < 51
 |
```

## `e83080` → `f66a30` AttackNexusSubPlan::score  (778→730B · Δ-48)
- 명령 175→168 · 정렬 157 · exe 판정 ⚠정렬 불가(미확정) · 정규화 3 · 블록이동 14 · 스택슬롯 5 · 패닉스텁 재배열 2 · 콜리 주의 2
- 잔여 구조: 구 18명령(mov|r15, r8…) / 신 11명령(mov|rax, qword ptr [rsp + I]…) 짝 없음 ; e830e2 피연산자 형 ; e830f2 피연산자 형 ; e83256 피연산자 형 ; e8325e 피연산자 형
- 잔여 분기: e831d0 → e832c8/f66bcc
- 잔여 즉치: 0x590→0x580
- 잔여 변위: r9+0x500→0x4c8
- 콜리 주의: d57540→f72620 변경 ; d59940→f749a0 변경
- 정규화 흡수: 오프셋표 [r12+930→a00] · 오프셋표 [r12+9c0→a90] · SmallActionPlay 태그 7→6
- 0.5.8 명세 #235 `attack_nexus__AttackNexus__score` src game-ai\src\plan_legacy\sub_plan\attack_nexus.rs:158 · one_line: AttackNexus 서브플랜의 후보 액션 점수: interaction_score + Attack/Skill/Skill2 는 calculate_action_score(Push) 가산(Skill/Skill2 는 대상이 타워면 +3), 대상 소실=-99999, Around 는 0(조회만), 그 외 0
- 0.5.8 logic(앞 1500자):
```
fn score(&self, version, parameter, rnd, player, data, action, debug) -> i64   // attack_nexus.rs:158 · self=ZST
let champ = data.cache.player_champion[player.info.team][player.info.position as usize].unwrap();   // L159
let base = interaction_score(version, rnd, player, data, parameter, action, debug);   // L160
let action_type = MinionActionType::Push;   // DI 상수 i8 2
let add = match action.get_action() {   // L164 · SmallActionPlay 태그 switch
    Attack{target_id} => if let Some(t) = data.cache.game.get_entity_by_id(target_id) /*L169*/ {
            let effect = champ.attack_effect.as_ref().unwrap();   // L170
            calculate_action_score(version, rnd, player, data, parameter, &champ.attack, effect, champ.attack_speed_mult(), t, Push, debug)   // L171 · ★tower_bonus 없음
        } else { -99999 },
    Skill{target_id} => if let Some(t) = get_entity_by_id(target_id) /*L177*/ {
            let tower_bonus = if t.is_tower() /*L178 · ty@tag==2*/ { 3 } else { 0 };
            let effect = champ.skill_effect.as_ref().unwrap();   // L184
            calculate_action_score(version, rnd, player, data, parameter, &champ.skill, effect, champ.cooldown_reduce(false), t, Push, debug) + tower_bonus   // L185
        } else { -99999 },
    Skill2{target_id} => if let Some(t) = get_entity_by_id(target_id) /*L191*/ {
            let tower_bonus = if t.is_tower() /*L193*/ { 3 } else { 0 };
            let effect = (if champ.level > 2 { champ.skill2_effect.as_ref() } else { None }).unwrap(
```

## `db90f0` → `faa440` update  (825→885B · Δ+60)
- 명령 200→216 · 정렬 193 · exe 판정 ⚠정렬 불가(미확정) · 정규화 2 · 블록이동 1 · 패닉스텁 재배열 2
- 잔여 구조: 구 7명령(lea|rdx, [r11 + rdx*I]…) / 신 23명령(lea|rdx, [r9 + rdx*I]…) 짝 없음
- 잔여 분기: db93ac → db9181/faa4e6
- 잔여 변위: rsi+0x29→0xad · rsi+0x28→0xab
- 정규화 흡수: 오프셋표 [r9+930→a00] · 오프셋표 [r9+9c0→a90]
- 0.5.8 명세 #14 `line_gank_ganker__update` src game-ai\src\plan_legacy\old\line_gank\ganker.rs:39 · one_line: 갱커의 갱크 계속/취소 판정 — 저HP거나 목표 부시 도착+적 없음이면 Cancel
- 0.5.8 logic(앞 1500자):
```
fn update(&mut self, _version, _rnd, player, data, goal_data, _positioning_score, _debug)
// ganker.rs:39. 반환값 없음 — self.chats / self.phase 만 바꾼다.

team = player.info.team // PlayerState+0x930, team<2 아니면 panic_bounds_check(ganker.rs:43)
pos = player.info.position.as_index() // PlayerState+0x9c0, entity.rs:580, range 0..5
cache = data.cache // OperationData+0x0
champ = cache.player_champion[team][pos].unwrap() // cache+0x1e0 + team*40 + pos*8, None 이면 panic(ganker.rs:43)

// ganker.rs:44
max_hp = champ.stat_cached.hp // Entity+0x628 (0 이면 div_by_zero panic)
hp_ratio = champ.hp * 100 / max_hp // Entity+0x670

// ganker.rs:47 — 취소 사유 ①
if hp_ratio < 41 {
 self.chats.push(Chat::Cancel(CancelReason::LowHpSelf)) // ganker.rs:48, 태그17/사유0
 self.phase = LineGankerPhase::Cancel // self+0x29 = 8
 return
}

// ganker.rs:54 — bush = self.target_bush_v30(player, data) [전량 인라인]
line = self.line // self+0x28, 0=Top/1=Mid/2=Bottom
context = data.context // OperationData+0x8
tower = cache.<line>_tower[team].or(cache.<line>_tower2[team]) // (cache+384+line*32)[team] .or( (cache+400+line*32)[team] ) ganker.rs:249

if tower.is_none() { // ganker.rs:250
 bush = match line { // ganker.rs:251
 Top => if team==0 {2} else {16} // 252
 Mid => if team==0 {4} else {17} // 253
 Bottom => if team==0 {9} else {21} // 254
 }
} else {
 t = tower.unwrap() // ganker.rs:258
 // champ 은 위에서 구한 것 재사용 (ganker.rs:259)
 is_tower_variant = (t.ty 판별자 == 2) // Entity+0x68 == EntityType::Tower
 match line { // ganker.r
```

## `d9ac10` → `e85f00` steal::should_steal_now  (4296→4382B · Δ+86)
- 명령 945→961 · 정렬 945 · exe 판정 ⚠정렬 불가(미확정) · 정규화 4 · 패닉스텁 재배열 2
- 잔여 구조: 구 0명령 / 신 16명령 짝 없음
- 잔여 변위: rbx+0x1ba→0x472 · rbx+0x220→0x510 · rbx+0x1b9→0x471 · rbx+0x230→0x520 · rbx+0x238→0x528 · rbx+0x2a8→0x598 · rbx+0x240→0x530 · rbx+0x248→0x538 · rbx+0x2b0→0x5a0 · rbx+0x250→0x540
- 정규화 흡수: 오프셋표 [rdx+9c0→a90] · 오프셋표 [rdx+930→a00] · SmallActionPlay 태그 0→2 · SmallActionPlay 태그 1→2
- 0.5.8 명세 #99 `steal__should_steal_now` src game-ai\src\plan_legacy\steal.rs:279 · one_line: [v4] 정글러 스틸 액션 결정 — 에픽/세르펜 각각 '적 3명+ 캠프 접근 가능·우리 팀이 안 치는 중·대상 hp<95%·최근 포기 아님' 후보를 걸러 evaluate_steal_for_target 로 평가하고 Commit > Commit > Lurk(둘 다면 적 예상 처치 잔여틱 짧은 쪽) > Lurk > None 으로 합친다
- 0.5.8 logic(앞 1500자):
```
fn should_steal_now(version, player, goal_data, team_plan, data) -> StealAction
  // L286~287
  if player.info.position != Jungle { return None }
  let Some(my_champ) = cache.player_champion[team][position] else { return None }
  // L291~299
  in_session_target = team_plan.current_steal_session.map(|s| s.target)     // None=2 / Epic=0 / Serpen=1
  if my_champ.hp*2 < my_champ.stat_cached.hp { return None }                // L294 hp<50%
  if in_session_target.is_none() && game.tick().saturating_sub(team_plan.last_battle_tick) < tps*5 { return None }   // L299
  // L304~310: 캠프 존에 있을 수 있는 적 수 (5슬롯 count)
  epic_zone_short  = !epic_exists(ctx)   || (0..5).filter(|p| enemy_could_be_at_camp(team, cache, team_plan, p, 288000, 288000)).count() < 3   // L329 조건으로 소비
  serpen_zone_short= !serpen_exists(ctx) || (0..5).filter(|p| enemy_could_be_at_camp(team, cache, team_plan, p, 672000, 672000)).count() < 3   // L358
  // L315~316
  epic_alive   = epic_exists && mob.epic.live_list.len != 0
  serpen_alive = serpen_exists && mob.serpen.live_list.len != 0
  // L319~345: 에픽 후보
  epic_action: (StealAction, usize) =
    if in_session_target == Some(Epic) {                                         // L320~321
      if epic_alive && objective != Morgard { evaluate(Epic, in_session=true) } else { (None, MAX) }
    } else {
      epic_eligible = mob.epic.live_list.get(0) → get_entity_by_id → is_some_and(|e| e.hp*100 < e.stat_cached.hp*95)   // L323~325
      if epic_alive && objective != Morgard(0)
```

## `dd6b40` → `f0dc20` v24_objective_setup_lane_pressure_ready  (1803→1901B · Δ+98)
- 명령 426→445 · 정렬 414 · exe 판정 ⚠정렬 불가(미확정) · 정규화 3 · 블록이동 3 · 콜리 주의 1
- 잔여 구조: 구 12명령(mov|rdi, r9…) / 신 31명령(mov|rdx, r9…) 짝 없음
- 잔여 분기: dd6b76 → dd6b8f/f0de0f ; dd6b88 → dd6b9f/f0dccb ; dd6cdb → dd6ce2/f0e10b
- 콜리 주의: decc00→f238f0 미지(pdata 밖 thunk)
- 정규화 흡수: 오프셋표 [rsi+930→a00] · 오프셋표 [rsi+9c0→a90] · 오프셋표 [rsi+9c0→a90]
- 0.5.8 명세 #67 `objective_discipline__v24_objective_setup_lane_pressure_ready` src game-ai\src\plan_legacy\team_plan\objective_discipline.rs:7 · one_line: 오브젝트(Morgard/Serpen) Setup 단계에서 '라인 정리가 끝나 캠프로 모여도 되는가'를 아군 집결·건강·압박 라인 잔여로 판정
- 0.5.8 logic(앞 1500자):
```
fn v24_objective_setup_lane_pressure_ready(self, version, player, data, goal_data, camp, debug) -> bool
  // L13 take_setup_like(camp) 인라인 (team_plan.rs:258)
  match camp { Morgard(4) => self.objective == Some(Morgard{phase: Setup,..})   // 0x41f==0 && 0x420==1
               Serpen(5)  => self.objective == Some(Serpen{phase: Setup,..})    // 0x41f==1 && 0x420==1
               _ => return false }  else return false
  obj = if camp==Serpen { WavePriorityObject::Serpen(1) } else { Morgard(0) }
  if is_object_being_taken_by_enemy(player, data, goal_data, self, obj) → return false     // L23
  team = player.info.team
  camp_pos = data.context.map.camp_pos(camp, team == 0)                                    // L27
  if !game.is_visible_cell(team, camp_pos.x/32000, camp_pos.y/32000) → return false       // L28 (vtable+0x100)
  if v23_recent_visible_enemies_near_point(player, data, camp_pos.x, camp_pos.y, 180000, 50) != 0 → return false   // L32
  allies = cache.player_champion[team]   (bounds team<2)
  pc = context.tutorial.player_count()   // runner.rs:295~301 인라인: TopSolo/MidSolo/JungleOnly→1, First/Bottom→2, MidBottom→3, None/Line/Total→>=4
  gathered = allies.flatten().filter(|c| c.hp*100/c.max_hp > 49 && dist²(c, camp_pos) < 220000²+1).count()   // L36~39 (5회 언롤)
  if gathered < min(2, pc) → return false                                                  // L40
  healthy = allies.flatten().filter(|c| c.hp*100/c.max_hp > 49).count()                    // L44 (aux s_0)
  if healt
```

## `cc9740` → `eabbb0` EpicPokeSubPlan::action_candidates_old  (4360→4460B · Δ+100)
- 명령 950→966 · 정렬 935 · exe 판정 ⚠정렬 불가(미확정) · 정규화 6 · 스택슬롯 70 · 콜리 주의 4
- 잔여 구조: 구 15명령(cmove|r8, rcx…) / 신 31명령(cmove|rdx, rcx…) 짝 없음
- 잔여 분기: cc9d4f → cca6aa/eacb7b
- 콜리 주의: 12857f0→1643790 변경 ; 31a01a3→381e0b3 미지 ; cada80→e920f0 미지(+2B) ; cb0310→e966a0 미지(-2B)
- 정규화 흡수: 오프셋표 [r8+930→a00] · 오프셋표 [r8+9c0→a90] · SmallActionPlay 태그 7→6 · SmallActionPlay 태그 7→6 · SmallActionPlay 태그 7→6
- 0.5.8 명세 #174 `EpicPokeSubPlan__action_candidates_old` src game-ai\src\plan_legacy\sub_plan\epic_poke.rs:426 · one_line: 에픽(모르가드) 견제 서브플랜의 행동 후보 목록 생성 — 논타겟 회피/궤적이면 도주 단일 후보, 아니면 에픽 상태별 이동 후보 1개 + 근접 적 도주 + 전투/소환수 후보를 순서대로 덧붙인다
- 0.5.8 logic(앞 1500자):
```
fn action_candidates_old(&mut self, version, rnd: &mut StdRng, player, data, parameter) -> Vec<SmallActionPlay>  // epic_poke.rs:426
  let bump = data.context.pool;                                   // L427
  let mut res = Vec::new_in(bump);                                // L427 (0x0=8,0x8=bump,cap=len=0)
  let champ = data.cache.player_champion[player.info.team][player.info.position as usize].unwrap();  // L430 (team<2 바운즈, None→panic)
  let enemy_team = 1 - player.info.team;                          // L433
  // L433~439: 적 챔프 5칸 순회(any)
  let has_non_target_action_range = data.cache.iter_champions(enemy_team).any(|c|
        nontarget_windup_perceived(version, player, data, c)       // L434
     && c.ty@tag == 13 /*Champion*/                               // L434
     && { let eff = match c.action_state@tag {                    // L435~439
             4 /*Skill*/  => c.skill_effect.as_ref().unwrap(),   // L435 (None 니치 -1 → panic)
             5 /*Skill2*/ => c.skill2_effect().as_ref().unwrap(), // L437 (level>2 ? &skill2_effect : &NONE(정적 anon.19))
             6 /*Ult*/    => c.ult_effect().as_ref().unwrap(),    // L439 (level>4 ? &ult_effect : &NONE)
             _ => return false };
          matches!(eff.casting@tag, 1 /*Position*/ | 2 /*Direction*/)   // 0=Targeting/3=None 이면 false
       && eff.is_in_range(c /*caster*/, champ /*target*/) });
  // L446~448
  let ps: PositioningScore = position_score_at_position(version, player, data, &parameter.positioning_score(0x
```

## `db8ba0` → `fb9b70` target_bush_v41  (698→597B · Δ-101)
- 명령 121→101 · 정렬 75 · exe 판정 ⚠정렬 불가(미확정) · 블록이동 13 · 스택슬롯 3
- 잔여 구조: 구 46명령(test|cl, cl…) / 신 26명령(mov|eax, ecx…) 짝 없음 ; db8bc2 피연산자 형 ; db8c1d scale
- 잔여 분기: db8ba6 → db8c6e/fb9c31 ; db8bb2 → db8cd7/fb9c96 ; db8c71 → db8d40/fb9cfb ; db8cd2 → db8d8d/fb9d8b ; db8cda → db8da0/fb9d44
- 잔여 소형 즉치: 0xdb8c62:0x7→0x2 · 0xdb8cc8:0x7→0x1 · 0xdb8d31:0x7→0x1 · 0xdb8d40:0x10→0x15 · 0xdb8d6d:0x2→0x9 · 0xdb8d76:0x2→0x7 · 0xdb8d94:0x7→0x2 · 0xdb8da0:0x15→0x10 · 0xdb8dcd:0x9→0x2 · 0xdb8dd6:0x7→0x2
- 잔여 즉치: 0x7→0x2 · 0x7→0x1 · 0x7→0x1 · 0x10→0x15 · 0x6→0x14 · 0x3→0xf · 0x3→0xf · 0x3→0xf · 0x2→0x9 · 0x2→0x7
- 잔여 변위: r8+0xb→0x4
- 0.5.8 명세 #58 `ganker__target_bush_v41` src game-ai\src\plan_legacy\old\line_gank\ganker.rs:310 · one_line: 갱커가 숨을 부시 id 를 (라인, 팀, 라인 리드 단계 0..6, 미드는 챔프 위치의 상/하 변) 로 룩업표에서 고른다
- 0.5.8 logic(앞 1500자):
```
fn target_bush_v41(line, team, position, cache, context) -> bush_id
  match line {                                                       // L311 switch(LineType 태그)
    Top(0) => {                                                       // L313
      bush_by_lead = if team==0 { [16,6,3,3,3,2,2] }                  // L314
                     else       { [2,3,6,6,6,16,16] }                 // L316
      lead = cache.top_lead[team]        // +0x21c0, team<2 아니면 패닉   L319
      return bush_by_lead[lead]          // lead<7 아니면 패닉             L319~320
    }
    Mid(1) => {                                                       // L322
      champ = cache.player_champion[team][position].unwrap()          // +0x1e0, L322 (None→패닉)
      low = (setting.height - champ.y) <u champ.x   // 인라인 is_top_side 의 IR 식. ★true = !is_top_side(x,y) = 대각선 아래쪽(x + y > height) — is_top_side = `x <= height − y` 확정(17차 배치C 오라클 v17C_o1.tsv 9/9 · 18차 배치A v18A_o1 42,000회). m08.ll:94264 `%64 = sub i64 %63(height), %59(y)` · 94265 `%65 = icmp ult i64 %64, %57(x)` · 94274~94278 `select i1 %65, i64 14, i64 11` 등  L323
      bush_by_lead = if team==0 { [low?21:17, low?14:11 ×4, low?9:4 ×2] }      // L323~ (select)
                     else       { [low?9:4,   low?14:11 ×4, low?21:17 ×2] }
      lead = cache.mid_lead[team]        // +0x21d0                        L337
      return bush_by_lead[lead]                                       // L337~338
    }
    Bottom(2) => {                                         
```
