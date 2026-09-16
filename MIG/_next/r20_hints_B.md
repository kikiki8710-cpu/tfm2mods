# 배치 B — 변경 99 중 티어 2/3 14 (0.5.8 RVA → 0.6.0 RVA · exe 정규화 정렬 힌트 · 0.5.8 명세 요지)

정규화 정렬(`--norm`)이 이미 흡수한 것: PlayerState +0xd0 · Blackboard 0x2e8→0x5c8 · SmallActionPlay 태그/idx 재번호 · BattleSubPlanGoal 재번호 · Effect vt 슬롯(4 삽입) · LPH 오프셋 6. 아래 「잔여」 가 정렬로 못 닫은 차이.

## `d69f80` → `fe6840` v17_runaway_counterattack_bonus  (2271→2271B · Δ+0)
- 명령 581→581 · 정렬 581 · exe 판정 ✅동치(정규화: SmallActionPlay·오프셋표) · 콜리 주의 2
- 콜리 주의: 12857f0→1643790 변경 ; d433f0→fd47a0 ptr:미지(-587B)
- 정규화 흡수: SmallActionPlay idx 4→3 · 오프셋표 [rdx+930→a00] · 오프셋표 2e8→5c8 · 오프셋표 [rax+9c0→a90] · 오프셋표 [rcx+1e0→3e8]
- 0.5.8 명세 #217 `battle_common__v17_runaway_counterattack_bonus` src game-ai\src\plan_legacy\sub_plan\battle_common.rs:270 · one_line: RunAway 골 중 가시 적 챔피언 target 에 대한 '역공(counterattack)' 가산점(0~95) — 즉사/콤보즉사/무거운 교환 중 하나일 때만, 사거리·유입피해·대상HP·아군/적 수로 가감
- 0.5.8 logic(앞 1500자):
```
L280  if !matches!(goal, BattleSubPlanGoal::RunAway) { return 0 }          // 태그 4 (old/battle.rs:264 인라인 비교)
L281  if target.team == champ.team { return 0 }
L282  if target.ty 태그 != 13(Champion) { return 0 }                         // L281·L282 한 select 로 접힘
L283  if target.stat_buff_cached.undying { return 0 }
L284  let enemy_team = 1 - player.info.team;   if !data.blackboard[enemy_team].is_recent_visible(game, player, target) { return 0 }
L288  let current_damage = effect.expected_damage_target(data.context, champ as &dyn AbstractEntity, target);
L289  let ready_damage   = v17_ready_counterattack_damage(data, champ, target);     // IR 은 (cache, context, champ, target) 로 승격
L290  let target_hp = max(target.hp, 1);
L291  let lethal_now   = current_damage >= target_hp;      // signed
L292  let combo_lethal = ready_damage   >= target_hp;      // signed
L293  let escape_is_costly = v17_escape_is_costly(player, data, parameter, champ);
L294  let close_counter = v17_close_counterattack_target(champ, target);   // 인라인 L471~473: reach = max_range(champ,target) + 20000 + champ.stat_cached.move_speed*12; dist2(target,champ) <= reach²
L296  let heavy_trade = escape_is_costly && close_counter && ready_damage*100 >= target_hp*70;   // dbg: %94=(ready*100 < hp*70) 에 DW_OP_not · 분기 방향도 일치
L297  if !lethal_now && !combo_lethal && !heavy_trade { return 0 }
L301  let mut bonus = if lethal_now { 75 } else if combo_lethal { 48 } else { 14 };
L309  if escape_is_costly { bonus += 10 }
L313  let 
```

## `e01c40` → `fe7120` defensive_crisis  (897→889B · Δ-8)
- 명령 209→209 · 정렬 65 · exe 판정 ⚠정렬 불가(미확정) · 정규화 1 · 블록이동 33 · 스택슬롯 3 · 콜리 주의 3
- 잔여 구조: 구 144명령(lea|rbp, [rsp + I]…) / 신 144명령(mov|r14, r8…) 짝 없음
- 잔여 분기: e01c7b → e01fae/fe71e0 ; e01cf6 → e01d7e/fe71e0 ; e01d0e → e01d94/fe71e0 ; e01d7c → e01d9b/fe71d8 ; e01d89 → e01f80/fe71e0
- 잔여 소형 즉치: 0xe01ee9:0x1→0x9
- 잔여 즉치: 0xd8→0x58 · 0x8→0x78 · 0x3→0xfa01 · 0x5→0x27101 · 0x3→0xfa01 · 0x1→0x9 · 0x1→0xfa01 · 0x1→0xfa01 · 0xd8→0x58
- 잔여 변위: r12+0x8→0x668
- 콜리 주의: ca1ca0→fe3320 불일치(mig060 는 e0b8b0) ; caa6a0→deb270 미지(+699B) ; d31bb0→fe5c40 미지
- 정규화 흡수: SmallActionPlay 태그 1→2
- 0.5.8 명세 #3 `buff_value__defensive_crisis` src game-ai\src\buff_value.rs:17 · one_line: target 주변 적 챔프를 추려 (2초내 죽을 위기, 곧 쓸 CC기 위협) 두 bool 을 낸다
- 0.5.8 logic(앞 1500자):
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
[L32] tp = data.cache.player_by_champion_id(target.id(+0x5c0)) // Opti
```

## `e657a0` → `d6bf10` v2_response_retreat_stance  (877→921B · Δ+44)
- 명령 190→192 · 정렬 183 · exe 판정 ⚠정렬 불가(미확정) · 정규화 6 · 블록이동 11 · 스택슬롯 58 · 콜리 주의 2
- 잔여 구조: 구 7명령(mov|qword ptr [rsp + I], rbp…) / 신 9명령(mov|qword ptr [rsp + I], r14…) 짝 없음
- 잔여 즉치: 0xf8→0x118 · 0xf8→0x118
- 콜리 주의: ca0240→e0cb10 불일치(mig060 는 e0b8b0) ; eb82d0→eda920 변경
- 정규화 흡수: SmallActionPlay idx 4→3 · 오프셋표 [r8+930→a00] · 오프셋표 [r8+9c0→a90] · BattleSubPlanGoal 3→2 · SmallActionPlay idx 4→3 · SmallActionPlay idx 4→3
- 0.5.8 명세 #6 `engage__v2_response_retreat_stance` src game-ai\src\plan_legacy\handler\engage.rs:13 · one_line: 후퇴 태세: 가장 가까운 적을 찾아 죽기까지 1초 넘게 버티면 KitingBack, 아니면 RunAway
- 0.5.8 logic(앞 1500자):
```
fn v2_response_retreat_stance(self, version, rnd, player, data, debug) -> BattleSubPlanGoal

L14 if version < 2 { return RunAway } // IR %14: icmp ult %0, 2

L17 team = player.info.team // +0x930, bounds-check team<2
 pos = player.info.position.as_index() // +0x9c0, range [0,5)
 champ = data.cache.player_champion[team][pos] // cache+0x1e0, [5 x ptr] stride 40
 if champ == null { return RunAway } // Option<&Entity> 니치 = null

L20 enemies = data.cache.iter_champions(1 - team) // cache.player_champion[1-team] 5칸을
 // filter_map(|c| *c) 로 null 제거 (인라인됨)
L21..L23 pred(e) = // 클로저 = closure_env$0, 본체는 m12.ll:41916~41998
 data.blackboard[1 - team].is_recent_visible(data.cache.game, player, e) // L21
 && !is_ignored_well_enemy(version, player, c) // ★호출부에 `fight_model::` 접두 없음(L22=53자 ±0) // L22
 && champ.distance_sq(e) < 40000000001 // L23 (= <= 200000^2)
 // 세 조건은 && 단축평가. 하나라도 false 면 그 적은 제외.

L24 nearest = enemies.filter(pred).min_by_key(|e| champ.distance_sq(e))
 // IR 은 이걸 둘로 쪼갬:
 // (a) 담당 본문에서 5칸을 수동 언롤(%30,%52,%58,%64,%70)하며 pred 가 true 인 **첫 원소**를 찾음.
 // 하나도 없으면 nearest=None → L25 로.
 // (b) 그 첫 원소의 키(dx^2+dy^2, Entity::distance_sq)를 시드로 나머지를 fold.
 // fold 본체 = m12.ll:12391~12638 (같은 pred 를 다시 평가하고 키 최소를 고름).
 // dx = x.abs_diff(other.x) (icmp+select 로 접힘), key = dx*dx + dy*dy.

L25 if nearest == None { return RunAway } // IR %103

L26/L30 near_enemies: Vec<&Entity, &Bump> =
 data.cache.iter_champions(1 - team)
 .filter(|e| is_recent_visible && !is_ignored_well_enemy && 
```

## `e7a8c0` → `f27140` upgrade_item  (932→1000B · Δ+68)
- 명령 252→261 · 정렬 223 · exe 판정 ⚠정렬 불가(미확정) · 블록이동 2 · 스택슬롯 37 · 콜리 주의 1
- 잔여 구조: 구 29명령(mov|rcx, qword ptr [rax]…) / 신 38명령(mov|rdi, qword ptr [rax]…) 짝 없음 ; e7a989 피연산자 형 ; e7aacf 피연산자 형 ; e7ab97 피연산자 수
- 잔여 분기: e7a97c → e7a960/f271f0
- 잔여 즉치: 0x88→0xa8 · 0x88→0xa8
- 잔여 변위: r12+0x998→0xa68
- 콜리 주의: 105fae0→16a8100 콜리 변경?(J0.50·+278B)
- 0.5.8 명세 #21 `lib__upgrade_item` src game-ai\src\lib.rs:1603 · one_line: 보유 아이템의 next_tier 후보 중 (활성 · 내 최대티어(4미만 기준)보다 높은 티어 · 가격≤골드) 인 것을 모아 무작위 1개를 (내 슬롯 i, item_list 인덱스) 로 돌려준다
- 0.5.8 logic(앞 1500자):
```
fn upgrade_item(_version, rnd, player, _game, context) -> Option<(usize, usize)>
// L1604~1605
let pool = context.pool; let item_list = context.item_list;   // +0x0 / +0x30
// L1610~1614: 내 아이템 중 tier<4 인 것의 최대 tier, 없으면 0
let my_max_tier = if player.info.items.iter().any(|it| it.tier() < 4) {            // L1610 closure$0 (slice::iter::any)
  player.info.items.iter().filter(|it| it.tier() < 4).map(|it| it.tier()).max().unwrap_or(0)   // L1614 closure$1(filter)·closure$2(map)·max_by fold
} else { 0 };
   // IR: 1차 루프 = any()(m14.ll:17~20 블록, !dbg 1610), 2차 루프 = Filter::next 로 first 를 잡은 뒤 fold(max_by Ord::cmp, !dbg 1614). max 가 None 이면 phi 0(unwrap_failed 호출 없음 = unwrap() 아님)
// L1621
let mut candidate: bumpalo Vec<(usize,usize)> = Vec::new_in(pool);
// L1622
for (i, my_item) in player.info.items.iter().enumerate() {
  // L1623
  for nxt in my_item.next_tier().iter() {            // vtable+0x80 -> &Vec<String>
    // L1625
    if let Some(index) = item_index_by_key(item_list, nxt) {   // Option<usize> {tag, idx}
      let it = &item_list[index];                    // bounds check
      // L1626 (단락 평가 순서 = IR 순서)
      if it.is_active()                              // vtable+0x50
         && it.tier() > my_max_tier                  // vtable+0x70, ugt
         && !(it.price() > player.info.gold)         // vtable+0x68 vs +0x998  ⇔ price <= gold
      {
        candidate.push((i, index));                  // L1627
      }
    }
  }
}
// L1633
if candidate.is_empty() { return N
```

## `e7acd0` → `f27590` should_recall_to_shop  (989→890B · Δ-99)
- 명령 253→238 · 정렬 231 · exe 판정 ⚠정렬 불가(미확정) · 블록이동 13 · 스택슬롯 21 · 콜리 주의 4
- 잔여 구조: 구 22명령(mov|rsi, qword ptr [rax + I]…) / 신 7명령(mov|rax, qword ptr [rsi + I]…) 짝 없음 ; e7af18 피연산자 형 ; e7af62 피연산자 형
- 잔여 분기: e7af05 → e7af7b/f27831 ; e7af79 → e7afd0/f27870
- 잔여 즉치: 0x98→0x68 · 0x98→0x68
- 잔여 변위: r13+0x4f0→0x560 · r13+0x4e8→0x558 · rax+0x4a8→0x10 · rax+0x4a0→0x8 · rsi+0x998→0xa68
- 콜리 주의: d7a360→16f2f40 미지(-385B) ; db4670→e565b0 미지(+73B) ; e7a8c0→f27140 변경 ; e7b640→f27ea0 변경
- 0.5.8 명세 #71 `lib__should_recall_to_shop` src game-ai\src\lib.rs:1642 · one_line: 상점 귀환 판정 — 에픽/세르펜 임박·can_recall 게이트 후, 다음 살 아이템(v26 슬롯 빌드경로) 가격을 골드가 넘는지(빌드 있으면 item_v26_affordable 인라인 판정)
- 0.5.8 logic(앞 1500자):
```
fn should_recall_to_shop(version, rnd, player, data, goal_data) -> bool
let tps = ctx.setting.tick_per_second;                                                  // L1647
let epic_alive   = game.get_game_mode().as_moba().map_or(false, |m| m.jungle_runner.epic.live_list.len()   != 0);   // L1648 (vtable+0x40, MobaMode+0x1a8)
let serpen_alive = game.get_game_mode().as_moba().map_or(false, |m| m.jungle_runner.serpen.live_list.len() != 0);   // L1649 (MobaMode+0x1d8)
if epic_alive   && min(goal_data.epic.epic_ally_tick,   goal_data.epic.epic_enemy_tick)   <= tps*20 { return false; }   // L1650 (ugt 면 통과)
if serpen_alive && min(goal_data.serpen.epic_ally_tick, goal_data.serpen.epic_enemy_tick) <= tps*20 { return false; }   // L1651
if !utils::can_recall(rnd, player, data) { return false; }                                 // L1656

if player.info.item_builds.len() == 0 {                                                   // L1660
    if buy_item(_, rnd, player, _, _, ctx).is_some() { return true; }                       // L1661
    return upgrade_item(_, rnd, player, _, _, ctx).is_some();                               // L1662/1666
}
// L1664 return item_v26_affordable(player, &ctx.item_list, rnd)   ← 별도 함수 game_ai::item_v26_affordable(lib.rs:1670~1696, in:game_ai,
//        fn(&PlayerState, &[Box<dyn ItemInfo>], &mut StdRng)->bool) 이 통째로 인라인(define 없음). 아래 L1671~1695 는 그 함수의 줄이다
let item_list = &ctx.item_list;                                                            // L1664 (Game
```

## `d57540` → `f72620` interaction_score  (9212→9088B · Δ-124)
- 명령 1988→1941 · 정렬 1754 · exe 판정 ⚠정렬 불가(미확정) · 정규화 23 · 블록이동 136 · 스택슬롯 231 · 콜리 주의 4
- 잔여 구조: 구 234명령(lea|r10, [rcx + rcx*I]…) / 신 187명령(lea|r11, [rcx + rcx*I]…) 짝 없음 ; d57553 피연산자 형 ; d57b24 피연산자 형 ; d5820c 피연산자 형 ; d5864a 피연산자 형 ; d5898d 피연산자 형
- 잔여 분기: d57601 → d57608/f7276f ; d57641 → d5768d/f72765 ; d5767d → d57683/f7298b ; d57756 → d5790a/f7298b ; d577e5 → d5790a/f7298b
- 잔여 소형 즉치: 0xd57603:0xa→0x6 · 0xd5762c:0x4→0xa · 0xd5763c:0x6→0x9
- 잔여 즉치: 0x1c8→0x1e8 · 0xa→0x6 · 0x4→0xa · 0x6→0x9 · 0x1c8→0x1e8
- 잔여 변위: r8+0x10→0x18 · r8+0x60→0x8 · r8+0x8→0x30 · r8+0x8→0x10 · r8+0x8→0x60 · r8+0x18→0x8 · r8+0x30→0x8 · rcx+0x510→0x580 · rcx+0x518→0x588 · rcx+0x5c8→0x4a0
- 콜리 주의: 12857f0→1643790 변경 ; d31bb0→d72be0 미지 ; d5bbf0→f77820 변경 ; eb82d0→eda920 변경
- 정규화 흡수: 오프셋표 [r8+930→a00] · 오프셋표 [r8+9c0→a90] · SmallActionPlay 태그 7→6 · SmallActionPlay 태그 8→7 · SmallActionPlay 태그 9→8 · BattleSubPlanGoal 7→4 · 오프셋표 [rcx+928→9f8] · SmallActionPlay idx a→9
- 0.5.8 명세 #183 `action_score__interaction_score` src game-ai\src\action_score.rs:616 · one_line: SmallActionPlay 하나의 상호작용(전투) 점수 i64 — 행동 종류별(RunAway/Around/Trace/Attack/Skill/Skill2/Ult) 피해·위험·탑·근접수 차이로 계산, 조기 거부는 -99999/-9999999
- 0.5.8 logic(앞 1500자):
```
fn interaction_score(version, rnd, player, data, parameter, action, debug) -> i64 {
  let game = data.cache.game;  // &dyn AbstractGame
  let team = player.info.team;  let enemy = 1 - team;
  // L617
  let champ = data.cache.player_champion[team][player.position()].unwrap();
  // L618 (get_action 인라인 · small_action.rs:308)
  let ty: SmallAction = action.get_action();   // RunAway/Recall/AroundRunAway→RunAway(0) · Around/AroundHide/LaneMinionPosition→Around(2,target) · Positioning(1) · AroundRegion/AroundPosition/AroundPositionBush/AroundBush→AroundPosition(3) · Trace→Trace(4,target) · Attack(6)/Skill(7)/Skill2(8)/Ult(9,target) · Stop(10)
  // L619 = interaction_ctx(player, data, champ) 인라인: INTER_CTX TLS (키 seed,tick,pid) → ictx: InterActionCtx
  let ictx = INTER_CTX.with(...);
  // L631~634
  let no_self_risk = if champ.stat_buff_cached.undying { true }
     else if version > 1 { if nexus_final_stand(player,data) { true } else if champ.hp*100 > max(champ.stat_cached.hp,1)*35 { base_defense_focus(player,data) } else { false } }
     else { false };
  // L636~638 (Option<id>.and_then(get_entity_by_id))
  let nearest_enemy_champion = ictx.nearest_vis_enemy → entity;  let near_ally_tower = ictx.near_ally_tower → entity;  let near_enemy_tower = ictx.near_enemy_tower → entity;
  let mut act_score = -99999;  let mut positioning_score = 0;   // L938 (phi 기본)
  match ty {
   RunAway => {                                                         // L647~731
     if champ.undying { retur
```

## `dd73b0` → `ff2450` SerpenStanceData::update_plan  (6536→6697B · Δ+161)
- 명령 1478→1517 · 정렬 1415 · exe 판정 ⚠정렬 불가(미확정) · 정규화 3 · 블록이동 7 · 스택슬롯 53 · 콜리 주의 2
- 잔여 구조: 구 63명령(mov|rsi, qword ptr [rbp + I]…) / 신 102명령(mov|qword ptr [rbp + I], r13…) 짝 없음
- 잔여 분기: dd73f8 → dd7412/ff24b0 ; dd7a61 → dd7b90/ff2dde ; dd7a9e → dd7b93/ff2c8e ; dd7b8e → dd7b93/ff2646 ; dd8166 → dd8216/ff3363
- 콜리 주의: 12857f0→1643790 변경 ; decf00→101b7f0 미지(pdata 밖 thunk)
- 정규화 흡수: 오프셋표 [r15+930→a00] · 오프셋표 [rax+9c0→a90] · 오프셋표 2e8→5c8
- 0.5.8 명세 #75 `goal_data__SerpenStanceData_update_plan` src game-ai\src\goal_data.rs:399 · one_line: 세르펜 태세(None/Check/CheckTry) 결정: 세르펜 존재·시야로 hp 갱신 → 아군/적 처치시간 6종 산출 → 준비되면 CheckTry, 적이 먹을 낌새면 Check
- 0.5.8 logic(앞 1500자):
```
fn update_plan(&mut self, _v, _rnd, player, data, enemy_region, debug):
  moba = cache.game.get_game_mode(); if tag!=0(Moba) { self.stance=None; return }        // :401~403
  serpen_state = &moba.jungle_runner.serpen                                              // :404 (+0x1c8)
  serpen = serpen_state.live_list.get(0).and_then(|id| game.get_entity_by_id(*id))       // :405
  if serpen==None { self.stance=None; return }                                            // :406~407
  if self.last_epic_seen < serpen_state.next_respawn_tick { self.last_epic_hp = serpen.stat_cached.hp; self.last_epic_seen = tick() }   // :413~415 리스폰 후 첫 관측: 풀피 가정
  if is_visible(player.team, serpen.id) { self.last_epic_seen = tick(); self.last_epic_hp = serpen.hp }   // :417~419
  ally_in_epic: Vec<usize> = (0..5).filter(|p| player_champion[team][p].is_some_and(|c| c.distance_sq(serpen) <= 150000²))   // :422~431 closure#2
  as_entity = ally_in_epic.filter_map(|p| player_champion[team][p])                         // :433 closure#3
  self.epic_ally_tick        = check_epic_kill_time_with_hp(ctx, as_entity, serpen, self.last_epic_hp)   // :437
  self.epic_ally_killed_tick = check_epic_killed_time(serpen, as_entity)                     // :438
  enemy_in_epic: Vec<usize> = (0..5).filter(|p| enemy_region[p].is_none_or(|e| {           // :440~456 closure#4 — 위치 모르면(None) 포함
        echamp = player_champion[1-team][p]?  (None→false)                                   // :442~443
        dist = distance(region_
```

## `e04f50` → `ee3e40` fight_participants  (1170→1396B · Δ+226)
- 명령 273→327 · 정렬 227 · exe 판정 ⚠정렬 불가(미확정) · 블록이동 23 · 스택슬롯 67 · 콜리 주의 2
- 잔여 구조: 구 46명령(mov|rdi, r9…) / 신 100명령(mov|qword ptr [rbp + I], r9…) 짝 없음
- 잔여 분기: e04fd8 → e05009/ee3f05 ; e0500c → e0509d/ee3fb4 ; e0501c → e05030/ee4030 ; e05025 → e04fe0/ee3ee7 ; e05027 → e05082/ee40bf
- 잔여 소형 즉치: 0xe05125:0x2→0x5
- 잔여 즉치: 0xc8→0xe8 · 0x2→0x5 · 0xc8→0xe8
- 잔여 변위: rax+0xf8→0x300 · r13+0x100→0x308 · r13+0x108→0x310
- 콜리 주의: ccc9c0→ee3c80 미지(-94B) ; e04c60→ee3000 미지(+102B)
- 0.5.8 명세 #41 `fight_model__fight_participants` src game-ai\src\plan_legacy\old\fight_model.rs:610 · one_line: 교전 참여 아군 목록: 근처 아군 전원 + (전투 선언·묶임·6초내 도착) 원거리 아군을 (엔티티, 도착틱, 묶임) 로 모은다
- 0.5.8 logic(앞 1500자):
```
fn fight_participants(version, rnd, data, player, champ, near_allies, fight_enemies, team_plan, debug) -> Vec<(&Entity, i64, bool), &Bump>   // fight_model.rs:610~640
[L613] horizon = data.context.setting.tick_per_second[+0x12f8] * 6
[L614] out = Vec::new_in(data.context.pool)

// ── 1) 근처 아군은 전원 포함 ──
[L615] for a in near_allies {
[L616]   bound = if a.id == champ.id { false } else { ally_is_bound(version, rnd, data, player, a, fight_enemies, debug) }
[L617]   out.push((a, 0, bound)) }        // ⚠자기 자신이 near_allies 에 있어도 push 된다(bound=false)… 아래 unknown 참조

// ── 2) 원거리 아군(팀 챔프 5명 순회) ──
[L619] for i in 0..5 {
[L620]   if team_plan.ally_battle_stop_tick[i](+0x0, stride16).is_some() { continue }
[L621]   team = player.info.team[+0x930] (<2)   Some(a) = cache.player_champion[+0x1e0][team][i] else continue
[L622]   if a.id == champ.id { continue }   if out.iter().any(|o| o.0.id == a.id) { continue }   // 이미 near 에 있음
[L623]   if ally_is_bound(version, rnd, data, player, a, fight_enemies, debug) {
[L624]     out.push((a, 0, true)); continue }
[L627]   declared = matches!(data.blackboard[team].big_goal[i].1, Some(BigGoal::Battle{focus: Some(f)}))   // +0xf8==5 && +0x100==1
                    && fight_enemies.iter().any(|e| e.id == f /*+0x108*/)
         if !declared { continue }
[L630]   sp = max(a.stat_cached.move_speed[+0x640], 1)
[L631]   reach = a.attack_effect(+0x4c0 != -1).map(|e| e.range() /*= a.stat_buff_cached.range[+0x438] + e.range[+0x4a0] + e.growth_range[+0x4a8]*(a.
```

## `cc5a10` → `eb28e0` JungleSubPlan::score  (641→954B · Δ+313)
- 명령 150→228 · 정렬 139 · exe 판정 ⚠정렬 불가(미확정) · 정규화 3 · 블록이동 4 · 스택슬롯 3 · 콜리 주의 1
- 잔여 구조: 구 11명령(mov|rbx, qword ptr [rsp + I]…) / 신 89명령(mov|rbx, r9…) 짝 없음 ; cc5adb 피연산자 형 ; cc5b3c 피연산자 형
- 잔여 즉치: 0x38→0x68 · 0x38→0x68
- 콜리 주의: d57540→f72620 변경
- 정규화 흡수: SmallActionPlay 태그 7→6 · SmallActionPlay 태그 c→b · SmallActionPlay idx(부호) -c→-b
- 0.5.8 명세 #234 `jungle__Jungle__score` src game-ai\src\plan_legacy\sub_plan\jungle.rs:152 · one_line: Jungle 서브플랜의 후보 액션 점수: interaction_score 기반 · Attack 은 base+calculate_jungle_action_score, ★Skill/Skill2 는 calculate_jungle_action_score 값으로 base 를 대체(base 미가산), 대상이 정글몹(is_jungle(1))이면 최소 1 보장, 대상 소실=-99999, 그 외 액션은 base 그대로
- 0.5.8 logic(앞 1500자):
```
fn score(&self, version, parameter, rnd, player, data, action, debug) -> i64   // jungle.rs:152 · self 미사용
let champ = data.cache.player_champion[player.info.team][player.info.position as usize].unwrap();   // L153
let mut base = interaction_score(version, rnd, player, data, parameter, action, debug);   // L154
match action.get_action() {   // L156 · SmallActionPlay 태그 switch, 12/13/14 만 arm
    Attack{target_id} => {   // idx 12
        if let Some(t) = data.cache.game.get_entity_by_id(target_id) {   // L158
            let score = base + calculate_jungle_action_score(rnd, player, data, parameter, &champ.attack /*+0x570*/, champ.attack_effect.as_ref().unwrap() /*+0x4c0/+0x490*/, t);   // L159
            if t.is_jungle(1) /*L160 · is_jungle(&EntityType, team: usize)->bool(tcx entity.rs:1377) · 호출 인자 리터럴 1(DI `team = i64 1` m02.ll:39912~39914) · 인라인 = ty@tag==4 && ty.info.camp_type.0(+0x98) <= team → IR `ult 2` 로 접힘*/ { base = max(score, 1); /*L161*/ } else { base = score; }
        } else { return -99999; }   // L158 else — phi 직행, base 미가산
    }
    Skill{target_id} => {   // idx 13 · L170~L173 · &champ.skill(+0x580) · skill_effect(+0x4f8/+0x4c8) · ★Attack 과 달리 base 미가산
        Some(t) → score = calculate_jungle_action_score(..., &champ.skill, skill_effect.unwrap(), t) /*L171 · IR 40081 %91 이 add 없이 phi/smax 로 직행 · 오라클 sc_skill_enemy 확인*/; base = t.is_jungle() ? max(score,1) /*L173*/ : score
        None → return -99999
    }
    Skill2{target_id} => {   // idx 14 · L182~L1
```

## `defa20` → `fd7b40` is_end  (675→1072B · Δ+397)
- 명령 165→266 · 정렬 143 · exe 판정 ⚠정렬 불가(미확정) · 정규화 3 · 블록이동 5 · 스택슬롯 22 · 콜리 주의 2
- 잔여 구조: 구 22명령(mov|r15, rdx…) / 신 123명령(cmp|byte ptr [rbx + I], I…) 짝 없음 ; defb14 피연산자 형 ; defb99 피연산자 형
- 잔여 분기: defade → defc04/fd7c34
- 잔여 즉치: 0x88→0x98 · 0x53d1ac101→0x53d1ac100 · 0x88→0x98
- 잔여 변위: r14+0x41f→0x3e4 · r14+0x41f→0x3e4
- 콜리 주의: dd7250→f0e390 변경 ; deeda0→dbb2a0 변경
- 정규화 흡수: SmallActionPlay 태그 0→2 · 오프셋표 [r9+930→a00] · SmallActionPlay 태그 0→2
- 0.5.8 명세 #8 `epic_hunt_and_poke__is_end` src game-ai\src\plan_legacy\old\epic\hunt_and_poke.rs:163 · one_line: 에픽(Morgard) 캠프 헌트앤포크 플랜을 접을지 판정 — 목표이탈·적 근접·에픽 리스폰 대기시간
- 0.5.8 logic(앞 1500자):
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
 .filter(|e| data.blackboard[1 - team].is_recent_visible(gam
```

## `d5bbf0` → `f77820` calculate_interaction_action_score  (19586→20138B · Δ+552)
- 명령 3893→4054 · 정렬 3204 · exe 판정 ⚠정렬 불가(미확정) · 정규화 33 · 블록이동 480 · 스택슬롯 690 · 패닉스텁 재배열 3 · TypeId 1 · 콜리 주의 17
- 잔여 구조: 구 689명령(lea|rax, [r8 + rax*I]…) / 신 850명령(lea|rax, [r8 + rax*I + I]…) 짝 없음 ; d5c131 피연산자 형 ; d5c720 피연산자 형 ; d5c720 피연산자 형 ; d5d077 피연산자 형 ; d5d903 피연산자 수
- 잔여 분기: d5bdf1 → d6072d/f78395 ; d5be64 → d5be7e/f77a9d ; d5be78 → d5c51e/f7c5f3 ; d5bf3a → d5c244/f77e3f ; d5c262 → d5c2a0/f77e96
- 잔여 소형 즉치: 0xd5eea2:0x6→0x1
- 잔여 즉치: 0x938→0x968 · 0x6→0x1 · 0xfa0→0x1090 · 0x320→0x350 · 0x320→0x350 · -0x5555555555555555→0x2aaaaaaaaaaaaaaa · -0x5555555555555555→0x2aaaaaaaaaaaaaaa · 0x938→0x968
- 잔여 변위: rax+0x8→0x668 · rcx+0x18→0x10 · rcx+0xe8→0x10 · rsi+0x464→0x49c · r12+0x438→0x5c8 · r12+0x5c8→0x438 · r11+0x628→0x670 · r11+0x670→0x628 · r15+0x660→0x640 · r13+0x438→0x680
- 데이터: d5c6f8 movdqa 16B 01000000000000000100000000000000→01000000000000000200000000000000 ; d5c79a movdqa 16B 01000000000000000200000000000000→01000000000000000300000000000000 ; d5c827 movdqa 16B 01000000000000000300000000000000→01000000000000000400000000000000 ; d5c8bf movdqa 16B 01000000000000000400000000000000→01000000000000000500000000000000
- 콜리 주의: 12857f0→1643790 변경 ; 128cc90→164b370 미지 ; 12a07d0→e80960 콜리 변경?(J0.00·+6444B) ; 16047b0→17d68b0 미지(+40B) ; 16fc160→de3d60 미지(-131B) ; 31a01a3→381e0b3 미지
- 정규화 흡수: 오프셋표 [r8+930→a00] · 오프셋표 [r8+9c0→a90] · 오프셋표 [r14+930→a00] · 오프셋표 [r14+9c0→a90] · 오프셋표 [rax+9c0→a90] · 오프셋표 [rax+9c0→a90] · 오프셋표 [rax+9c0→a90] · 오프셋표 [rax+9c0→a90]
- 0.5.8 명세 #181 `action_score__calculate_interaction_action_score` src game-ai\src\action_score.rs:980 · one_line: 액션 후보 1개(effect, 대상 t)의 상호작용 점수 i64 — 적/아군/자기 대상별 피해·힐·버프·CC 후속·타워·이동 항을 합산
- 0.5.8 logic(앞 1500자):
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
L1024       if let Some((_,_,walk_tick)) = line_action_stance(data,champ,t,effect) {   // Option<(u64,
```

## `cc20a0` → `ea6240` BattleSubPlan::calculate_score_parameter_value  (4059→3316B · Δ-743)
- 명령 692→610 · 정렬 447 · exe 판정 ⚠정렬 불가(미확정) · 정규화 4 · 블록이동 56 · 스택슬롯 17 · 콜리 주의 1
- 잔여 구조: 구 245명령(mov|rsi, qword ptr [rsp + I]…) / 신 163명령(mov|rbp, rcx…) 짝 없음 ; cc23be 피연산자 형
- 잔여 분기: cc20fc → cc221a/ea6c5f ; cc2136 → cc2177/ea6b81 ; cc2175 → cc2150/ea6a40 ; cc217e → cc221a/ea6724 ; cc2214 → cc2190/ea64ab
- 잔여 소형 즉치: 0xcc2132:0x3→0x7 · 0xcc213a:0x3→0x7 · 0xcc27be:0x7→0x3 · 0xcc2a67:0x7→0x3 · 0xcc2a6f:0x7→0x3
- 잔여 즉치: 0xd8→0xe8 · 0x3→0x7 · 0x3→0x7 · 0x50→0x32 · 0x32→0xa · 0x23→0x3c · 0xa→0x1e · 0x1e→0x32 · 0x1e→0x32 · 0x7→0x3
- 잔여 변위: rsi+0x14b8→0x14d8 · r11+0x180→0x258 · r11+0x188→0x260 · r11+0x208→0x2e0
- 콜리 주의: d74400→e70d80 변경
- 정규화 흡수: SmallActionPlay 마스크 6c0→360 · 오프셋표 [r9+9c0→a90] · 오프셋표 [r9+930→a00] · 오프셋표 [r9+9c0→a90]
- 0.5.8 명세 #219 `battle__Battle__calculate_score_parameter_value` src game-ai\src\plan_legacy\sub_plan\battle.rs:839 · one_line: 전투 서브플랜의 ScoreParameter 가중치: 태세(goal)별 자기/적/아군 attack·util_value 기본표(10~100) 세팅 후 전술(tactic)별 TacticModifier 백분율 곱
- 0.5.8 logic(앞 1500자):
```
fn calculate_score_parameter_value(&self, version, _rnd, player, data, parameter: &mut ScoreParameter)   [battle.rs:839]
  const trace_non_focus = 30                                                                   [DI 36329]
  match self.goal {                                                                            [L842 · m02.ll:36346 switch]
    Trace{focus} | Assassin{focus} =>                                                          [L843]
      parameter.player.attack_value = 30; .util_value = 30                                     [L844~845]
      for e in near_enemies { v = if goal==Assassin { 60 } else if e.id==focus { 60 } else { trace_non_focus(30) }; e.attack_value=v; e.util_value=v }   [L847~849 · 36439 tag==5 분기 · 36589 select]
      for a in near_allies { a.attack_value=30; a.util_value=30 }                               [L864~866]
    Protect{focus} | Kiting{focus} | KitingBack{focus} =>                                        [L883]
      self 50/50                                                                                [L884~885]
      for e in near_enemies { v = if e.id==focus { 80 } else { 50 }; … }                         [L887~889]
      for a in near_allies { 50/50 }                                                             [L898~900]
    AssassinReady{..} =>                                                                        [L870]
      self 100/100; near_enemies 10/10; near_allies 10/10                                        [L870~880]
```

## `cce210` → `fd98c0` SmallActionTrace::get_input  (4742→5984B · Δ+1242)
- 명령 961→1225 · 정렬 737 · exe 판정 ⚠정렬 불가(미확정) · 정규화 6 · 블록이동 185 · 스택슬롯 184 · 콜리 주의 11
- 잔여 구조: 구 224명령(mov|rbx, r9…) / 신 488명령(mov|qword ptr [rbp + I], r9…) 짝 없음 ; cce3c4 피연산자 형 ; cce3c4 피연산자 형 ; cce3e2 피연산자 형 ; cce9d1 피연산자 형 ; cce9f8 IAT ('kernel32.dll', 'GetProcessHeap')≠('kernel32.dll', 'HeapFree')
- 잔여 분기: cce2d4 → cce2ec/fd999d ; cce3e5 → cce538/fd9bdf ; cce41f → cce48a/fd9b2d ; cce4ec → cce535/fd9bd5 ; cce551 → cce59c/fd9c3a
- 잔여 즉치: 0x4a8→0x698 · 0x1a0→0x19a · 0x17→0xe2900 · 0x7d00→0xe2900 · 0x4a8→0x698
- 잔여 변위: r10+0x5c8→0x4a8 · r12+0x8→0x4d0
- 콜리 주의: 12857f0→1643790 변경 ; 31a01a3→381e0cb 미지 ; 31a01bb→13f0790 미지 ; 31a01bb→381e0cb 미지 ; cefd50→381ebcf 불일치(mig060 는 e29770) ; d11bc0→e48880 불일치(mig060 는 e1b5b0)
- 정규화 흡수: 오프셋표 [rax+930→a00] · 오프셋표 [rax+9c0→a90] · SmallActionPlay 태그 2→0
- 0.5.8 명세 #176 `SmallActionTrace__get_input` src game-ai\src\small_action\trace.rs:165 · one_line: 추격(Trace) 소액션의 매 틱 입력 — 대상까지의 최소 사거리 표적점을 목표로 잡고, 타워/우물/적존 회피 정책의 PathFinder 를 (재)생성·갱신한 뒤 타워 escape 판정이면 RunAway 입력, 아니면 경로 입력을 낸다
- 0.5.8 logic(앞 1500자):
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
        self.go
```

## `defcd0` → `fd7f70` EpicHuntAndPokePlan::sub_plan  (3413→5068B · Δ+1655)
- 명령 800→1171 · 정렬 474 · exe 판정 ⚠정렬 불가(미확정) · 정규화 8 · 블록이동 177 · 스택슬롯 84 · 콜리 주의 7
- 잔여 구조: 구 326명령(mov|rsi, qword ptr [rax + I]…) / 신 697명령(mov|rsi, qword ptr [r12 + I]…) 짝 없음 ; df0832 피연산자 형 ; df097a 피연산자 형
- 잔여 분기: defdaa → defde2/fd8170 ; defdc2 → defde2/fd8170 ; defddd → deff38/fd8188 ; defdef → deff38/fd82b0 ; defe4c → defeda/fd816c
- 잔여 소형 즉치: 0xdeff4f:0x3→0x0
- 잔여 즉치: 0xf8→0x118 · 0x33→0x14 · 0xf8→0x118 · 0x3→0x0 · 0x0→0x32
- 잔여 변위: rax+0x628→0x8 · rcx+0x41f→0xcc7 · rcx+0x420→0xcd5 · rax+0x98→0x10 · rax+0x8→0x1f0 · rax+0x10→0x8
- 콜리 주의: 1323a00→2f380b0 미지(-58B) ; 1323a00→381ebcf 미지(-89B) ; d3b2a0→ef6da0 불일치(mig060 는 eed7f0) ; d7bdf0→3821770 미지(-292B) ; dd5db0→f0cfe0 변경 ; e1c100→fed330 미지(pdata 밖 thunk)
- 정규화 흡수: 오프셋표 [r10+930→a00] · 오프셋표 [r10+9c0→a90] · SmallActionPlay 태그 0→1 · 오프셋표 [rdx+508→578] · 오프셋표 [rdx+4f8→568] · 오프셋표 2e8→5c8 · 오프셋표 [r8+930→a00] · 오프셋표 9c0→a90
- 0.5.8 명세 #83 `epic_hunt_and_poke__sub_plan` src game-ai\src\plan_legacy\old\epic\hunt_and_poke.rs:30 · one_line: 에픽(Morgard) 사냥/견제 빅플랜의 서브플랜 선택 — Steal/EpicCheck/Recall/LineDefense/EpicHunt/EpicPoke 중 하나를 sret 로 반환
- 0.5.8 logic(앞 1500자):
```
// hunt_and_poke.rs:30~96 (+ check_recall :103~160 인라인). 분기 순서대로.
t = player.info.team; pos = player.info.position
champ = cache.player_champion[t][pos].unwrap()                                   // :32

// ── A. 스틸 모드 (:35~44)
if self.focus_epic_only || self.vision_only {
   moba = game.get_game_mode() (vtable+0x40) → tag 0 = Moba 아니면 unwrap 패닉      // :36
   epic = moba.jungle_runner.epic.live_list.first().and_then(|id| game.get_entity_by_id(id))   // :37 closure$0
   if epic.is_some() → return Steal{ last_vision_tick:0, target:Some(Epic), commit:self.focus_epic_only }   // :39
   else → return EpicCheck{ move_check:false }                                        // :44
}

// ── B. 일반 (:47~)
hp_ratio = champ.hp*100 / champ.stat_cached.hp                                    // :47 (max_hp 0 → 패닉)
can_upgrade_item = upgrade_item(version, rnd, player, game, ctx).is_some()          // :48
epic = moba.jungle_runner.epic.live_list.first().and_then(get_entity_by_id)         // :49 closure$1
if epic.is_none() → return EpicCheck{false}                                        // :50~51
(x0,y0,x1,y1) = map.fountains[t]                                                    // :56
is_in_heal_area = x0<=champ.x<=x1 && y0<=champ.y<=y1                                // :57
if epic.hp == epic.stat_cached.hp /*에픽 풀피*/ && (hp_ratio < 51 || can_upgrade_item || (is_in_heal_area && champ.hp < champ.stat_cached.hp))   // :58 (줄 안 순서 표기 불가)
   → return Recall                                             
```
