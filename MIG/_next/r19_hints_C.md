# 배치 C — 실변경 5 (0.5.8 RVA → 0.6.0 RVA · exe 정렬 힌트 · 0.5.8 명세 요지)

## `d405d0` → `e78a10` base_attacking_minion_uncached
- 명령 190→180 · 정렬 100 · exe 판정 ❌구조 변경 · 블록이동 27 · 스택슬롯 7 · 콜리 주의 1
- 구조 차이: 구 90명령(mov|rax, rcx…) / 신 80명령(mov|r9, rcx…) 짝 없음
- 분기 차이: d40695 → d40704/e78b0b ; d4069d → d406a7/e78cf3 ; d406a5 → d40704/e78b97 ; d406b3 → d406a0/e78b40 ; d406bd → d406a0/e78b80 ; d406cd → d406f7/e78b40
- 즉치: 0x8→0x130
- 변위: rcx+0x930→0xa00 · rax+0x9c0→0xa90 · r8+0x88→0x68 · r8+0x660→0x668
- 콜리 주의: d702b0→e7c120 미지(-455B)
- 0.5.8 명세 #118 `defense_nexus__base_attacking_minion_uncached` src game-ai\src\plan_legacy\old\defense_nexus.rs:279 · one_line: 우리 본진 구조물(넥서스·쌍둥이 타워)을 때리는 적 미니언 중 내 챔피언과 가장 가까운 놈의 id (없으면 None)
- 0.5.8 logic(앞 1800자):
```
fn base_attacking_minion_uncached(player, data) -> Option<usize> {   // defense_nexus.rs:279
  let team = player.info.team;                                                        // :280 L62123
  let nexus = data.cache.nexus[team]?;                                                 // :281 L62131~62136 (None → 반환 None, slot1 undef)
  let champ = data.cache.player_champion[team][player.info.position.as_index()]?;     // :282 L62145~62154
  let twins = &data.cache.twin_towers[team];                                            // :283 L62165~62166 (bumpalo Vec<&Entity>)
  let is_structure = |id: usize| id == nexus.id || twins.iter().any(|t| t.id == id);   // :284 closure#0 (aux L66255~66303 인라인) — 넥서스 먼저, 없으면 쌍둥이 타워 선형 any
  data.cache.iter_minions(1 - team)                                                     // :285 L62173~62174 적 미니언 top→mid→bottom Chain (56B 이터레이터)
      .filter(|m| m.ty@tag == 1 /*Minion*/ && m.ty@Minion.info.nearest_enemy.is_some_and(|id| is_structure(id)))   // :286 closure#1 (aux L66224~66306) — '지금 우리 구조물을 때리는(=nearest_enemy 가 구조물 id) 미니언'
      .min_by_key(|m| champ.distance_sq(m))                                            // :287 closure#2 = min_by_key 의 **key 클로저**(&Entity → u64). 첫 원소는 본문 L62347~62375, 나머지는 aux(m11) L23478~23506 인라인. distance_sq = |dx|²+|dy|² (utils.rs:7~9, abs_diff 후 wrapping mul). ★소스에 `.map(|m| (key, m))`·`|(d,_)| *d` 는 없다 — 튜플 (key, elem) 생성·비교·`|(_, x)| x` 는 std min_by_key 내부(iterator.rs:3387~3390)
      // (std 내부) Map<Filter,key>::min_by(compare)                                        // :287 reduce → Map::fold(m12.ll:16716) → Chain::fold(m11 L23303~) ; compare = Ord::cmp(acc.0, new.0), acc 유지 조건 is_le (L23520~23524) ⇒ 동거리면 **먼저 나온 것(top→mid→bottom 순)** 유지
      .map(|m| m.id)                                               
```

## `e0dfc0` → `dea9b0` v30_wave_danger_chase_guard
- 명령 573→572 · 정렬 571 · exe 판정 ❌구조 변경
- 구조 차이: 구 2명령(movzx|eax, byte ptr [rsp + I]…) / 신 1명령(movzx|r13d, byte ptr [rsp + I]…) 짝 없음
- 소형 즉치: 0xe0e7e8:0x4→0x3 · 0xe0e809:0x4→0x3 · 0xe0e81a:0x3→0x2 · 0xe0e822:0x7→0x4
- 즉치: 0x4→0x3 · 0x4→0x3 · 0x3→0x2 · 0x7→0x4
- 0.5.8 명세 #257 `battle__v30_wave_danger_chase_guard` src game-ai\src\plan_legacy\old\battle.rs:2079 · one_line: 추격 목표(focused)를 향한 정지점(stance)에서 window 틱 동안 맞을 적 미니언 라인 피해를 재고, 피해 0/경미/즉살 가능/웨이브보다 먼저 킬 가능이면 None(추격 계속·wave_obs 코드 기록), 아니면 RunAway / KitingBack(focus) / End 중 하나로 추격을 끊는다
- 0.5.8 logic(앞 1800자):
```
// old/battle.rs:2079 v30_wave_danger_chase_guard(version, data, champ, focused, max_range, focused_die_tick, focused_is_in_range, my_die_tick, target_hp_ratio, can_runaway, return_to_objective, open_eval, wave_obs:&mut u8, wave_pct_out:&mut u8, wave_danger_out:&mut bool) -> Option<BattleSubPlanGoal>
// ── 0. 전제 ──
if focused.team == champ.team { return None }                                            // L2083 (TeamType::eq 인라인: 태그 같고 (Neutral 둘 다 | Player 번호 같음)) · out 3개 미기록
// ── 1. stance = battle_chase_stance(data, champ, focused, max_range) (old/battle.rs:2205 인라인, L2087) ──
if max_range == 0 { return None }                                                          // L2206 · 미기록
dist = Entity::distance(champ, focused)                                                    // L2210
if dist <= max_range { (stance_x,stance_y,walk_tick) = (champ.x, champ.y, 0) }             // L2211~2212
else {
  if !focused.is_visible_from(champ) { return None }   // L2214: champ.team Player(t) 이면 focused.visible_state[t]==Visible 필요, Neutral 이면 통과 · 미기록
  dx = champ.x − focused.x ; dy = champ.y − focused.y ; sz = isqrt(dx²+dy²)               // L2218~2220
  if sz < 1 { (stance_x,stance_y,walk_tick) = (champ.x, champ.y, 0) }                      // L2221
  else {
    from_distance = max_range.saturating_sub(15000)                                        // L2225
    (x,y) = adjust_position(ctx.map, ctx.setting, focused.x + dx*from_distance/sz, focused.y + dy*from_distance/sz)   // L2226~2228
    walk_dist = dist.saturating_sub(max_range) ; move_speed = max(champ.move_speed, 1)     // L2229~2230
    (stance_x,stance_y,walk_tick) = (x, y, walk_dist / move_speed)                         // L2231~2232
  }
}
// ── 2. 창·웨이브 피해 ──
tps = ctx.setting.tick_per_second                               
```

## `d99f60` → `100b2b0` v30_line_champion_action_tower_aggro_risk
- 명령 168→168 · 정렬 168 · exe 판정 ❌구조 변경 · 콜리 주의 2
- 분기 차이: d9a048 switch case 수 13→12(정렬 실패)
- 소형 즉치: 0xd9a02f:0x7→0x6 · 0xd9a039:0xc→0xb · 0xd9a045:-0xc→-0xb
- 즉치: 0x7→0x6 · 0xc→0xb · -0xc→-0xb
- 변위: r14+0x930→0xa00 · r14+0x9c0→0xa90
- 콜리 주의: 12857f0→1643790 변경 ; d74bd0→e7f5b0 미지(-453B)
- 0.5.8 명세 #232 `tower_discipline__v30_line_champion_action_tower_aggro_risk` src game-ai\src\tower_discipline.rs:160 · one_line: 라인전 구간에서 적 챔피언 대상 공격/스킬 행동이 (즉사시키지 못하면서) 시전 위치를 적 타워 사거리(+15000) 안에 두어 타워 어그로를 끌 위험이 있는가
- 0.5.8 logic(앞 1800자):
```
fn v30_line_champion_action_tower_aggro_risk(version, data, player, action) -> bool
// version(%0) 미사용
L160: tick = cache.game.tick(); if !context.is_line_phase(tick) → return false
      // is_line_phase = !(tutorial ∈ {None,MidBottom,Line,Total}) || tick < first_spawn_tick.saturating_sub(tps*30)
L161: if !(cache.game.tick() < setting.tower_attack_disable_tick) → return false
L165: champ = cache.player_champion[team][pos]? ; None → return false
L169: small_action = action.get_action()   // SmallActionPlay 니치 태그 복원 → Attack/Skill/Skill2/Ult 만, 그 외 → return false
L170: target_id = small_action.get_action_target()  // 4종 모두 payload +0x8 target (blackboard.rs:104~106)
L173: target = cache.game.get_entity_by_id(target_id)? ; None → return false
L176: if target.team == champ.team || !target.is_champion() → return false
L180: effect = v30_line_action_effect(champ, small_action) 인라인(L123~127):
        Attack → champ.attack_effect.as_ref()  / Skill → skill_effect.as_ref()
        Skill2 → champ.skill2_effect() (level>2 아니면 None) / Ult → champ.ult_effect() (level>4 아니면 None)
      None → return false
L183: if !CastingTarget::check(&effect.target, champ, target) → return false
L187: if !(expected_damage_target(effect, context, champ, target) < target.hp) → return false   // 한 방에 죽이면 위험 아님
L191: (stance_x, stance_y) = v30_line_action_stance(data.context /*IR ABI: internal fastcc ArgumentPromotion — tcx 의 &OperationData 인자가 context 포인터(nullable=Option 인코딩)로 승격, m07.ll:52985 `ptr nonnull %10`*/, champ, target, effect)
L192: cache.iter_towers_without_nexus(1 - team)
L193:   .filter(|t| t.can_target())            // +0x6b9 && +0x6a0==0
L194:   .any(|tower| tower.attack_effect.as_ref()
L195:        .is_some_and(|ta| ta.is_in_range_ex(tower, champ, tower.x, tower.y, stance_x, stance_y, 
```

## `e28410` → `e73610` line_action_economy_adjustment
- 명령 295→295 · 정렬 295 · exe 판정 ❌구조 변경 · 블록이동 1 · 콜리 주의 1
- 분기 차이: e28487 switch case 수 13→12(정렬 실패) ; e28731 → e28738/e73938
- 소형 즉치: 0xe28469:0x7→0x6 · 0xe28475:0xc→0xb · 0xe28483:-0xc→-0xb
- 즉치: 0x7→0x6 · 0xc→0xb · -0xc→-0xb
- 변위: rdx+0x930→0xa00 · rdx+0x9c0→0xa90
- 콜리 주의: e27f50→e72ed0 미지(+641B)
- 0.5.8 명세 #147 `lane_economy__line_action_economy_adjustment` src game-ai\src\lane_economy.rs:6 · one_line: 라인 공격/스킬 액션의 경제 보정치 — 적 챔피언/미니언 대상 공격이 받을 응징피해(punish) 대비 허용치(allowance)로 감점(음수)·챔피언 교환은 이득이면 가점
- 0.5.8 logic(앞 1800자):
```
L8 : champ = data.cache.player_champion[player.info.team][player.info.position]  — None(null) → return 0
L12: match action.tag (+0xb1):
  Attack(15): effect=&champ.attack_effect (None→0) ; target_id=action+0x8 ; speed_mult=champ.attack_speed_mult() ; action_duration=champ.attack_duration()          (L14~16)
  Skill (16): effect=&champ.skill_effect  (None→0) ; speed_mult=champ.cooldown_reduce(false) ; action_duration=champ.skill_duration()                                     (L18~20)
  Skill2(17): effect=champ.skill2_effect()[level>2 ? &skill2_effect : &NONE] (None→0) ; cooldown_reduce(false) ; skill2_duration()                                        (L22~24)
  Ult   (18): effect=champ.ult_effect()[level>4 ? &ult_effect : &NONE] (None→0) ; cooldown_reduce(true) ; ult_duration()                                                  (L26~28)
  그 외 13종 → return 0                                                                                                                                                       (L309<12 switch)
L32: target = data.cache.game.get_entity_by_id(target_id) (vtable +0x1f0) — None → 0
L35: if target.team == champ.team → 0 ; if target.ty ∉ {Minion(1), Champion(13)} → 0
L38: if !CastingTarget::check(&effect.target, champ, target) → 0
L42: (stance_x, stance_y, walk_tick) = line_action_stance(data, champ, target, effect)?  — sret 32B Option<(u64,u64,u64)> (+0 태그 i64 trunc→i1, +8/+16/+24) · None → 0 (L43)
L45: window = max( max(effect.start_timing*100 / speed_mult.max(1), action_duration) + walk_tick, 30 ) + 45
L46: punish_damage = line_projected_punish_damage_at(version, player, data, champ, target.ty@tag, stance_x, stance_y, window)
L47: if punish_damage < 1 → 0
L51: hp_value = champion_hp_value(data, parameter, &parameter.player).max(1)
L52: risk_score = 
```

## `e4a780` → `d55830` v3_assign_anchor
- 명령 139→139 · 정렬 139 · exe 판정 ⚠소형 즉치 변경(4→7)
- 소형 즉치: 0xe4a7d7:0x4→0x7
- 즉치: 0x4→0x7
- 변위: rdx+0x1808→0x24b5 · rdx+0x5e8→0xf58 · rdx+0x638→0xfc0 · rdx+0x650→0xfe3 · rdx+0x706→0x107b · rdx+0x618→0x100b
- 0.5.8 명세 #40 `handler__v3_assign_anchor` src game-ai\src\plan_legacy\handler.rs:1773 · one_line: v3_armed 이면 현 BigPlan 의 목적지(라인 1차타워 중점/정글캠프)를, 2칸 안이면 현 위치를 앵커로 낸다
- 0.5.8 logic(앞 1800자):
```
fn v3_assign_anchor(&self, player, data) -> Option<(u64,u64)>   // handler.rs:1773~1799
[L1774] team = player.info.team[<2]   pos = player.info.position.as_index()
        champ = data.cache.player_champion[+0x1e0][team][pos] else { return None }
[L1775] here = (champ.x[+0x660], champ.y[+0x668])
[L1776] if !self.v3_armed[+0x1808] { return Some(here) }
[L1779] line_anchor = |line: LineType| {
[L1780]   mine   = first_tower_position(line, team)        // m15.ll:51623 — 상수표: Top (272000,48000)/(48000,272000), Mid (592000,368000)/(368000,592000), Bottom (912000,688000)/(688000,912000); team==0 이면 앞쪽
[L1781]   theirs = first_tower_position(line, 1 - team)
[L1782]   ((mine.x + theirs.x) >> 1, (mine.y + theirs.y) >> 1) }   // 양 팀 1차 타워의 중점 = 라인 중앙
[L1784] dest = match self.plan[+0x5e8 태그] {
[L1785]   PassiveLine(p)                          => line_anchor(p.line[+0x706]),
[L1786]   LineGanker(p)                           => line_anchor(p.line[+0x618]),
[L1787]   PassiveJungle(p)                        => data.context.map[+0x20].camp_pos(p.jungle[+0x650], p.team[+0x638] == 0),
[L1789]   EpicHuntAndPoke | EpicHuntAndBattle     => map.camp_pos(JungleType::Morgard(4), team == 0),
[L1791]   SerpenHuntAndPoke | SerpenHuntAndBattle => map.camp_pos(JungleType::Serpen(5), team == 0),
          _ /*ForcePassive·SinglePlanLine·SinglePlanBattle·DeathMatchBattle·ActiveRecall·Battle·LineGankCover·AttackNexus·DefenseNexus*/ => return Some(here) }
[L1796] d2 = abs_diff(here.x, dest.x)² + abs_diff(here.y, dest.y)²   // utils::distance_sq(utils.rs:9) 인라인
        return Some( if d2 > 4096000000 /*64000²=2셀*/ { dest } else { here } )

※ 부작용 없음(sret 외 store 0). rnd 없음. 이 함수는 v33+ ActionContext 의 anchor("어디 근처에 있어야 하는가") 공급원으로 보이나 호출측(m13.ll 7002/7166/8193)은 안 봤다.
```
