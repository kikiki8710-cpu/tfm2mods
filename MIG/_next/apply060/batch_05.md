# apply060 batch_05.md — 반영(logic_060) 브리핑 · 함수 9

★**version = 3 런타임 확정(09-17)** — v<3/v2 경로는 명세에서 제외(사장). 태그/오프셋/vt 슬롯 = `spec_patch_060.md` §A(dispcheck 추가 행 포함). 출력 형식·절차 = `mkapply060.py` 도크스트링. 출력 = `_next/apply060/out/<old>.md`.

RE 정본 파일: 2026-09-17_r20_변경99_배치A_14_디컴대조_원문.md · 2026-09-17_r20_변경99_배치B_14_디컴대조_원문.md · 2026-09-17_r20_변경99_배치C_14_디컴대조_원문.md

### `e04f50` → `ee3e40` fight_participants (i=41 · ⚠변경·다건(r20))
- 0.5.8 src: `game-ai\src\plan_legacy\old\fight_model.rs:610` · one_line: 교전 참여 아군 목록: 근처 아군 전원 + (전투 선언·묶임·6초내 도착) 원거리 아군을 (엔티티, 도착틱, 묶임) 로 모은다
- 0.6.0 판정: **다건** · 패치 요지: 근접 아군 push `(a, poke_vulnerable? tps*6+1 : 0, false)` · +0xcc1 이면 bb[+0x4d8+i*8] 6초 게이트 · 원거리 `walk<=tps*6 && !poke_vulnerable` · 술어 ee3c80
- RE 정본: `2026-09-17_r20_변경99_배치B_14_디컴대조_원문.md` · 절: `2026-09-17_r20_변경99_배치B_14_디컴대조_원문.md` §8
- sig: `fn(usize, &mut rand::rngs::std::StdRng, &game_core::OperationData, &game_core::PlayerState, &game_core::Entity, &[&game_core::Entity], &[&game_core::Entity], &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< (&game_core::Entity, i64, bool)`
- consts: [{"value": 6, "src_line": 613, "meaning": "horizon = tick_per_second * 6 — 원거리 아군이 교전에 합류하는 데 허용하는 최대 시간 = 6초", "kind": "계수", "ev": 4}, {"value": 0, "src_line": 617, "meaning": "near_allies 원소의 도착틱 = 0 (이미 근처)", "kind": "태그", "ev": 4}, {"value": 2, "src_line": 621, "meaning": "player_champion / blackboard 의 팀 인덱스 상한(panic_bounds_check len=2)", "kind": "임계", "ev": 4}, {"value": 5, "src_line": 619, "meaning": "포지션 루프 0..5", "kind": "임계", "ev": 4}, {"value": 0, "src_line": 620, "meaning": "team_plan.ally_battle_stop_tick[i] 의 None 태그 — None 일 때만 후보", "kind": "태그", "ev": 4}, {"value": 5, "src_line
- 0.5.8 logic 전문:
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
[L631]   reach = a.attack_effect(+0x4c0 != -1).map(|e| e.range() /*= a.stat_buff_cached.range[+0x438] + e.range[+0x4a0] + e.growth_range[+0x4a8]*(a.level[+0x5c8]-1)*/).unwrap_or(0)
[L632]   gap = fight_enemies.iter().map(|e| a.distance(e)).min()   // declared 로 비어있지 않음이 보장돼 첫 원소 무검사 로드
               .saturating_sub(reach)
[L633]   t = gap / sp
[L634]   if t > horizon { continue }
         out.push((a, t, false)) }
[L639] return out

※ 부작용: 게임 구조체 store 0. out 은 bump pool 할당. rnd/debug 는 ally_is_bound 에서 변할 수 있음.
```
- logic_note: ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)

### `e7acd0` → `f27590` should_recall_to_shop (i=71 · ⚠변경·한 줄(r20))
- 0.5.8 src: `game-ai\src\lib.rs:1642` · one_line: 상점 귀환 판정 — 에픽/세르펜 임박·can_recall 게이트 후, 다음 살 아이템(v26 슬롯 빌드경로) 가격을 골드가 넘는지(빌드 있으면 item_v26_affordable 인라인 판정)
- 0.6.0 판정: **한 줄** · 패치 요지: `if active_cnt > 3 && item_list[next].tier()==0 { return false }`(구 >2) · 카운트 = 16f2f40
- RE 정본: `2026-09-17_r20_변경99_배치B_14_디컴대조_원문.md` · 절: `2026-09-17_r20_변경99_배치B_14_디컴대조_원문.md` §5
- sig: `fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData) -> bool`
- consts: [{"value": 20, "src_line": 1650, "meaning": "tps*20 = 20초. 에픽/세르펜 생존 중이고 아군·적군 처치 예상틱 min 이 20초 이내면 귀환 안 함(L1650/1651)", "kind": "계수", "ev": 4}, {"value": 0, "src_line": 1648, "meaning": "GameMode 태그 0 = Moba (tcxdict --enum GameMode). L1660 item_builds.len==0 분기. L1692 tier()==0", "kind": "태그", "ev": 3}, {"value": 1, "src_line": 1661, "meaning": "buy_item 반환 Option<usize> Some 태그(=1) → true. L1680 inventory_index Some 태그", "kind": "태그", "ev": 4}, {"value": -1, "src_line": 1671, "meaning": "item_v26_slot 반환 Option<(usize,Option<usize>)> 외곽 None 니치 태그(+0x8 == -1) → false", "kind": "센티널", "ev": 
- 0.5.8 logic 전문:
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
let item_list = &ctx.item_list;                                                            // L1664 (GameContext+0x30)
let (build_slot, inventory_index) = item_v26_slot(player, item_list) else { return false };   // L1671~1672
// L1674
let target = player.info.item_builds.iter().copied()
    .filter(|&i| item_list.get(i).map_or(false, |it| it.is_active()))
    .nth(build_slot) else { return false };
// L1680~1687
let next_item = if let Some(inv_idx) = inventory_index {
    let inv_item = player.info.items.get(inv_idx) else { return false };                     // L1681
    random_item_next_toward_target(item_list, inv_item.key(), item_list[target].key(), rnd) else { return false }   // L1682
} else {
    let path = random_item_build_path(item_list, target, rnd) else { return false };         // L1687
    if path.is_empty() { return false; }
    path[0]
};
// L1692
let active: Vec<usize> = active_inventory_item_indices(player.info.items, item_list);   // from_iter 인라인
if active.len() > 2 && item_list[next_item].tier() == 0 { return false; }              // [next_item] bounds 패닉 가능
// L1695
return player.info.gold >= item_list[next_item].price();
```
- logic_note: ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)

### `e7b640` → `f27ea0` buy_item (i=23 · ⚠변경·한 줄(r20))
- 0.5.8 src: `game-ai\src\lib.rs:1477` · one_line: 저티어(<4) 보유템이 없고 보유 ≤2개일 때, 활성·구매가능·tier0 아이템 중 챔피언 카테고리(근접/원거리/마법/유틸/암살)와 기본 HP 에 따라 허용 카테고리를 골라 후보를 만들고 무작위 1개 item_list 인덱스를 돌려준다
- 0.6.0 판정: **한 줄** · 패치 요지: `items.len()>2 → continue` 삭제 → 선검사 `active_owned = items.filter(|it| list.any(is_active && name==)).count(); if >3 → None`
- RE 정본: `2026-09-17_r20_변경99_배치A_14_디컴대조_원문.md` · 절: `2026-09-17_r20_변경99_배치A_14_디컴대조_원문.md` §8
- sig: `fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::GameContext) -> std::option::Option<usize>`
- consts: [{"value": 4, "src_line": 1480, "meaning": "has_low_tier_item = 보유템 중 tier() < 4 가 하나라도 있으면 → 즉시 None(구매 대신 업그레이드 대상이 있다는 뜻) · 오라클 실행 확증(15차 배치A: 오라클 o2.tsv(_verify15/A/oracle, pub 래퍼 AgentVerHamster::buy_item/upgrade_item 경유 · rnd 클론으로 gen_range 까지 동일 재현) owned=[ad1](tier1)·[t3](tier3) → None, [t4]·[t4,t5] → 진행 (tier<4 판별))", "kind": "임계", "ev": 2}, {"value": 2, "src_line": 1496, "meaning": "보유템 수 > 2 이면 모든 아이템 skip(=후보 0 → None). 즉 보유 0~2개일 때만 신규 구매 · 오라클 실행 확증(15차 배치A: 오라클 o2.tsv(_verify15/A/oracle, pub 래퍼 AgentVerHamster::buy_item/upgrade_item 경유 · rnd 클론으로 gen_range 까지 동일 재현) owned=[t4,t5
- 0.5.8 logic 전문:
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
      if hp < 1550 { match n { 0 | 2 => category < 2, _ => is_defensive } }        // L1524~1531
      else if player.info.champion.stat().hp < 1800 {                              // L1536 (stat() 재호출 — 별개 sret alloca %9/%8 로 vtable+0x30 을 두 번 부른다. 지역변수 재사용이었다면 불투명 vtable 호출은 CSE 되지 않으므로 호출이 1회였을 것 ⟹ 소스가 stat() 을 두 번 쓴 것은 사실)
        if n == 2 { category < 2 } else { is_defensive } }                          // L1537~1544
      else { is_defensive }                                                         // L1551
    }
    Assassin(4) => match n { 0 | 2 => category < 2, _ => is_defensive },           // L1557~1564
    Range(1) => category < 2,                                                       // L1570
    Magician(2) => is_magic,                                                        // L1575
    Util(3) => { let hp = player.info.champion.stat().hp;                           // L1582
                 if hp < 1550 { is_magic } else { is_defensive } }                  // L1583/1586
    _ => unreachable,
  };
  if ok { candidate.push(i); }                                   // L1592
}
// L1595
if candidate.is_empty() { return None; }
// L1596
Some(candidate[rnd.gen_range(0..candidate.len())])
```
- logic_note: ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)

### `d40f10` → `fb1510` evaluate_gank_opportunity_with_score (i=66 · ⚠변경·한 줄(r20))
- 0.5.8 src: `game-ai\src\plan_legacy\old\passive_jungle.rs:693` · one_line: 정글러 관점 특정 라인의 갱 성공 가능성을 점수화하고 판단력 노이즈를 곱해 required_score 와 비교
- 0.6.0 판정: **한 줄** · 패치 요지: 노이즈 폭 `k=(1000-judge)/10` → `/20` (헬퍼 de2fa0 · RNG 마스크 달라짐)
- RE 정본: `2026-09-17_r20_변경99_배치A_14_디컴대조_원문.md` · 절: `2026-09-17_r20_변경99_배치A_14_디컴대조_원문.md` §7
- sig: `fn(&mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, game_core::LineType, i32) -> (bool, i32, i32)`
- consts: [{"value": 2, "src_line": 702, "meaning": "팀 인덱스 bounds check (enemy_team < 2). 판정 아님", "kind": "임계", "ev": 4}, {"value": 5, "src_line": 716, "meaning": "포지션 수 — line_allies 후보 Range 0..5. 리터럴 5 는 이터레이터 체인(713~716)이 Filter 구조체로 접힐 때 Range.end 로 저장되며 IR 위치는 체인 마지막 `.filter` 줄 716 (m04.ll:63091 `store i64 5, ptr %49` !dbg !72398 → filter.rs:28 ← iterator.rs:957 ← 716). 같은 5 가 715 closure$2 의 player_champion[team][pos] bounds check 에도 나온다(m01.ll:30837 `icmp ult i64 %27, 5`). 판정 아님 (본문의 `shl i8 %line, 5` 는 line*0x20 타워 배열 스트라이드 — 이 5 와 무관)", "kind": "산출값", "ev": 4}, {"value": 1, "src_line": 724, "
- 0.5.8 logic 전문:
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
  ehp = (target_enemy.hp*100 / target_enemy.stat_cached.hp) as i32
  score = if ehp > 79 { -40 } else if ehp > 59 { (80-ehp) >> 1 } else if ehp > 39 { 80-ehp } else { 90-ehp }
  // L759~772 적 타워 거리
  enemy_tower = tower(line)[enemy_team].or(tower2(line)[enemy_team])   // 0x180+line*0x20 / 0x190+line*0x20
  match enemy_tower {
    Some(t) => { d = distance(target_enemy.xy, t.xy);
                 if d < 130000 { score -= 60 } else if d < 160000 { /* 변화 없음 */ } else if d < 200000 { score += 20 } else { score += 40 } }
    None    => score += 35 }
  // L776~777 아군 체력
  ahp = (nearest_ally.hp*100 / nearest_ally.stat_cached.hp) as i32
  if ahp < 50 { score -= 30 } else if ahp > 69 { score += 10 }
  // L784~785 아군↔대상 거리
  ad = distance(nearest_ally.xy, target_enemy.xy)
  if ad > 120000 { score -= 25 } else if ad < 80000 { score += 15 }
  // L792 수적 우위
  if line_allies.len() + 1 > line_enemies.len() { score += 20 }
  actual_score = score
  // L799~801 판단력 노이즈 (utils.rs:480 error_ratio 인라인)
  error = (1000 - player.stat.judgement) / 20            // judgement 1000 → 0, 0 → 50
  pct = 100 + rnd.gen_range(-error ..= error)             // (assert low<=high, 위반 시 panic)
  evaluated_score = actual_score * pct / 100               // i32 sdiv (0 방향 절사)
  return (evaluated_score >= required_score, evaluated_score, actual_score)   // L803
```
- logic_note: ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)

### `df0a90` → `dcb890` SerpenHuntAndPokePlan::is_end (i=72 · ⚠변경·다건(r20))
- 0.5.8 src: `game-ai\src\plan_legacy\old\serpen\hunt_and_poke.rs:162` · one_line: 세르펜 헌트/포크 플랜 종료 판정 — 목표가 Serpen 이 아니거나, Setup 국면에서 해제/적 부재/적 원거리, 또는 세르펜이 죽고 리스폰이 15초 넘게 남으면 종료
- 0.6.0 판정: **다건** · 패치 요지: 진입: `tp.3e4==2 → (tp.404!=2 || tp.cd5!=1) → true` · setup_like = (3e4==2)?(404==2&&cd5==1&&cd6==1):(3e0==1)
- RE 정본: `2026-09-17_r20_변경99_배치A_14_디컴대조_원문.md` · 절: `2026-09-17_r20_변경99_배치A_14_디컴대조_원문.md` §10
- sig: `fn(&game_ai::plan_legacy::old::SerpenHuntAndPokePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bool`
- consts: [{"value": 1, "src_line": 163, "meaning": "MainObjective 메모리태그 1 = Serpen (tcxdict --enum MainObjective) · L169 ObjectPhase 태그 1 = Setup · L175 enemy = 1 - team", "kind": "태그", "ev": 3}, {"value": 5, "src_line": 168, "meaning": "JungleType 태그 5 = Serpen (tcxdict --enum JungleType) — camp_pos·take_active·should_release_to_passive 인자", "kind": "태그", "ev": 3}, {"value": 32000, "src_line": 174, "meaning": "셀 크기 — camp 좌표 → 셀 좌표 변환(is_visible_cell 인자). 임계 아님", "kind": "계수", "ev": 4}, {"value": 22500000000, "src_line": 179, "meaning": "150000^2 — 캠프에서 가장 가까운 가시 적 챔피언이 150000(4.69셀) 초과 거리면 종료(ugt)", 
- 0.5.8 logic 전문:
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
if serpen.is_some() { return false; }
// L191
let m = game.get_game_mode().as_moba().unwrap();
return m.jungle_runner.serpen.next_respawn_tick.saturating_sub(game.tick()) > tps * 15;   // vtable+0x28 tick
```
- logic_note: ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)

### `d26900` → `ec4c20` PassiveLinePlan::v46_stage1 (i=213 · ⚠변경·한 줄(r20))
- 0.5.8 src: `game-ai\src\plan_legacy\old\passive_line.rs:663` · one_line: v46 라인 CS 사망예측 1단계 — near_enemies 중 '나를 먼저 죽일 수 있는' 커밋터(e.id, my_die, kill_dps)를 bumpalo Vec 으로 반환
- 0.6.0 판정: **한 줄** · 패치 요지: (c,b,a′) = aggr(+0x49d)?(0,67,32) : def(+0x49e)?(240,67,32) : 구 공식 · 시그니처 (team,my_pos)→player
- RE 정본: `2026-09-17_r20_변경99_배치A_14_디컴대조_원문.md` · 절: `2026-09-17_r20_변경99_배치A_14_디컴대조_원문.md` §12
- sig: `fn(&game_ai::plan_legacy::old::PassiveLinePlan, usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, &game_core::Entity, &game_core::Entity, usize, bool, std::option::Option<&[usize]>, &bumpalo::collections::vec::Vec< &game_core::Entity>) -> bumpalo::collections::vec::Vec< `
- consts: [{"value": -1, "src_line": 671, "meaning": "Option<Effect> attack_effect 의 None 니치 태그(i32, Entity+0x4c0). L671·L746·L768", "kind": "센티널", "ev": 4}, {"value": 100, "src_line": 672, "meaning": "radius*(radius_mult+100)/100 — Entity::radius 백분율(entity.rs:1515). L708 hp*100/max_hp 의 100 도 동일 리터럴", "kind": "계수", "ev": 4}, {"value": 5120, "src_line": 677, "meaning": "phi 에 접힌 GameSetting 오프셋(0x1400 tower_attack_disable_tick_2v2). 5112=0x13f8 기본 · 5128=0x1408 3v3 — 판정값이 아니라 오프셋(reads 참조)", "kind": "산출값", "ev": 4}, {"value": 5, "src_line": 677, "meaning": "TutorialType 태그 5 = MidBottom → 3v3 disable t
- 0.5.8 logic 전문:
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
  e_pos = ep.info.position.as_index()                      // L695
  aggr = ep.info.parameter.aggressive_ratio()              // L696
  diff_bound      = 80 + (1000-aggr)*80/1000               // L697  (80..160)
  die_tick_bound  = 45 + aggr*45/1000                      // L698  (45..90)
  tower_tick_bound= 15 + aggr*35/1000                      // L699  (15..50)

  // 적측 킬 능력 (L704~718): 적 팀(1-team) 챔피언 전원(e 자신 포함, 가시성 무관)
  kill_dps = 0; kill_nuke = 0
  for a in cache.player_champion[1-team].iter().flatten() {
    if distance_sq(a, e) > 150000² { continue }            // L705
    if a.hp*100/max(a.stat_cached.hp,1) < 40 { continue }  // L708
    c = &cache.player_champion_cache[ap.info.team][ap.info.position]  where ap = player_state at a.id 의 (t,p)  // L711~712 (unwrap·bounds 패닉 가드)
    nuke = if a.can_attack() || a.attack_cooldown() <= tps { c.attack[my_pos] } else { 0 }            // L714 (attack_cooldown = ty 별 필드 switch entity.rs:1748)
    if a.can_skill()  || !(a is Champion && a.skill_cooldown  > tps) { nuke = max(nuke, c.skill[my_pos]) }   // L715
    if a.can_skill2() || !(a is Champion && a.skill2_cooldown > tps) { nuke = max(nuke, c.skill2[my_pos]) }  // L716
    kill_dps  += c.attack_per_sec[my_pos] + c.skill_per_sec[my_pos] + c.skill2_per_sec[my_pos]   // L717
    kill_nuke += nuke                                       // L718
  }
  my_die = my_hp.saturating_sub(kill_nuke) * 60 / max(kill_dps,1)   // L720 (틱)

  // 우리측 억지력 (L726~749): 내 팀 챔피언 + 내 타워
  det_dps = 0; det_nuke = 0
  for a in cache.player_champion[team].iter().flatten() {
    if distance_sq(a, e) > 150000² { continue }            // L727
    if !a.is_visible_from(e) { continue }                  // L730 entity.rs:1482 — e.team Neutral 이면 가시, 아니면 a.visible_state[e.team]==Visible
    if is_ignored_well_enemy(version, ep, a) { continue }  // L733
    c = ChampionCache of a (L736~737, unwrap·bounds 패닉 가드)
    nuke = (L739~741: 위와 같은 즉발 규칙, 대상 열 = e_pos)
    det_dps  += c.attack_per_sec[e_pos] + c.skill_per_sec[e_pos] + c.skill2_per_sec[e_pos]   // L742
    det_nuke += nuke                                        // L743
  }
  for t in my_towers {                                      // L745
    if let Some(atk) = t.attack_effect.as_ref() {           // L746
      dmg = atk.expected_damage_target(ctx, t as &dyn AbstractEntity, e)   // L747
      det_dps  += dmg*tps / max(t.attack_cooltime(),1)     // L748
      det_nuke += dmg                                       // L749
    }
  }
  e_die = e.hp.saturating_sub(det_nuke) * 60 / max(det_dps,1)   // L752

  // 게이트 (L755): 둘 중 하나라도 참이면 커밋터 아님
  if my_die + diff_bound > e_die || my_die >= escape_ticks.saturating_sub(tower_tick_bound) { continue }

  // 커밋 판정 (L762~770)
  if e.stat_cached.move_speed > my_ms {                     // L762 — 나보다 빠르면 무조건 커밋터
    push
  } else if my_die < die_tick_bound {                       // L764
    if range_gate {                                         // L767
      e_range = e.attack_effect.map(range 공식).unwrap_or(0)           // L768 closure$3
      // L770: e 가 CS 중인 나에게 닿는가
      if pull + e.distance(front) > e_range + e.move_speed*cs_lock + e.radius() + champ.radius() { continue }
    }
    push
  } else { continue }
  push: committers.push((e.id, my_die, kill_dps))          // L775
}
return committers                                          // L777 (sret memcpy 32B)

극성 근거: L755 %345=or(%344,%342) → br → %411(continue). L762 %349 ugt → %350(push). L764 %358 ult → %359. L770 %402 ugt → %411(skip). 순서는 !dbg 줄번호(755<762<764<767<770).
```
- logic_note: ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)

### `dccc60` → `ff11f0` GoalData::update (i=74 · ⚠변경·한 줄(r20))
- 0.5.8 src: `game-ai\src\goal_data.rs:82` · one_line: 매 평가마다 GoalData 상태 갱신: 힐 커밋(v2+) · 본진수비 틱 · 적 5명 리전 추적(5초 만료) · 에픽/세르펜 태세 갱신
- 0.6.0 판정: **한 줄** · 패치 요지: goal_data.rs:27 앞 `if v>=3 && champ.undying { remain=max(undying buffs duration); if remain>tps { heal_commit=false } }`
- RE 정본: `2026-09-17_r20_변경99_배치C_14_디컴대조_원문.md` · 절: `2026-09-17_r20_변경99_배치C_14_디컴대조_원문.md` §1
- sig: `fn(&mut game_ai::GoalData, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData)`
- consts: [{"value": 2, "src_line": 28, "meaning": "version 게이트: version < 2 이면 update_heal_commit 전체 스킵(힐 커밋 없음)", "kind": "임계", "ev": 4}, {"value": 36, "src_line": 47, "meaning": "hp_ratio(%) < 36 이어야 힐 커밋 시작 (hp*100/max(max_hp,1))", "kind": "임계", "ev": 4}, {"value": 100, "src_line": 46, "meaning": "hp 백분율 계수 (hp*100/max_hp) — :46,:55 두 곳", "kind": "계수", "ev": 4}, {"value": 5, "src_line": 108, "meaning": "enemy_region 만료 = tps*5 (5초) — `mul i64 %tps, 5`; last_known + tps*5 < tick 이면 None", "kind": "계수", "ev": 4}, {"value": 32000, "src_line": 101, "meaning": "셀 크기(좌표→셀 변환) — 임계 아님", "kind": "계수", "ev":
- 0.5.8 logic 전문:
```
fn update(&mut self, version, _rnd, player, data, debug):
  // ---- (A) update_heal_commit (goal_data.rs:27~57, 인라인) ----
  if version >= 2 {                                   // :28  version<2 → 이 블록 전체 스킵
    champ = cache.player_champion[player.team][player.pos]   // :31 (team>=2 → bounds panic)
    if champ == None { self.heal_commit = false; }   // :32~33
    else {
      before = self.heal_commit                       // :35
      if nexus_final_stand(player,data)      { new = false }   // :36
      else if nexus_is_critical(player,data) { new = false }   // :37
      else if before {                                          // :39
        if champ.hp < champ.stat_cached.hp { (store 없음 = true 유지) } else { new = false }   // :40 풀피 도달 → 해제
      } else {                                                  // :44
        (lx,ly,rx,ry) = map.fountains[player.team]
        in_heal_area = lx<=champ.x<=rx && ly<=champ.y<=ry      // :45
        hp_ratio = champ.hp*100 / max(champ.stat_cached.hp,1)  // :46
        if !in_heal_area && hp_ratio < 36 && base_defense_focus(player,data) { new = true }   // :47 ★우물 밖에서만(IR %80/%89 분기) (&& 단락: base_defense_focus 는 앞 둘 통과 시만 호출)
        else (store 없음 = false 유지)
      }
      if store 됐고 before != new && ctx.debug { debug.add_log(format!("HEALCOMMIT T{team} {pos:?} {start|end} hp={hp*100/max_hp}%")) }  // :51~55
    }
  }
  // ---- (B) 본진수비 틱 ----
  if base_defense_focus(player,data) { self.last_base_defense_tick = tick() }   // :84~85
  // ---- (C) 적 리전 추적 ----
  for p in cache.game.iter_player() {                 // :87
    if p.team == player.team { continue }            // :88 아군 스킵
    pos = p.position.as_index()                       // :91
    if p.is_dead() {                                  // :93 (play_state tag==0)
      region = if p.team==0 {0} else {26}             // :95
    } else {                                          // :98
      echamp = cache.player_champion[p.team][pos]; if None → goto 만료검사
      if !is_visible(p.team, echamp.id) → goto 만료검사      // :99
      region = map.regions[min(echamp.y/32000,29)][min(echamp.x/32000,29)]   // :101
    }
    self.enemy_region[pos] = Some{region, last_known: tick()}   // :103~105
    만료검사: if let Some(r)=self.enemy_region[pos] { if r.last_known + tps*5 < tick() { self.enemy_region[pos]=None } }   // :107~109
  }
  // ---- (D) 오브젝트 태세 ----
  self.epic.update_plan(player, data, /*enemy_region=*/self, _)       // :114
  self.serpen.update_plan(player, data, /*enemy_region=*/self, debug) // :115
  // ---- (E) 디버그 ----
  if ctx.debug { champ = player_champion[player.team][pos]; if Some → debug.map[champ.id].push("epic_stance: {}","serpen_stance: {}","epic_enemy_tick: {}","epic_enemy_killed_tick: {}","epic_ally_tick: {}","epic_ally_killed_tick: {}") }   // :117~126 (epic 쪽 4틱만 출력, serpen 틱은 미출력)
```
- logic_note: ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)

### `d851d0` → `ff62e0` position_eval_at_uncached (i=180 · ⚠변경·부분규명(전담)(r20))
- 0.5.8 src: `game-ai\src\position_eval.rs:371` · one_line: 셀 (x,y) 에 내 챔피언이 서 있을 때의 위험/이득 56B PositioningScore 를 계산 — 벽 셀=9999 즉시 반환, 적 우물 위험 셀=risk·tower_risk 9999 에서 시작해 계속 누적(우물 사각 안이면 +well_damage HP% 까지: 오라클 hp=1 → 10149), 정글·에픽 몹 기대피해(HP% ×1/½/⅓ 가중 — ×2 없음)·타워(사거리·미니언 수)·미니언 웨이브·투사체 궤도·적 챔피언 위협(가시/비가시)·아군 교전 이득·purpose 별 보정을 순서대로 누적. POS_EVAL_CACHE miss 시에만 호출자 position_eval_at 이 부른다.
- 0.6.0 판정: **전담** · 패치 요지: 종반 신규 항: 고체력 적 중심점 이격 보너스(cap 25 · purpose 4 ×2 · cap 60) → score+0x6f0 · 나머지 1항 부호 재작성 미해석
- RE 정본: `2026-09-17_r20_변경99_배치C_14_디컴대조_원문.md` · 절: `2026-09-17_r20_변경99_배치C_14_디컴대조_원문.md` §5
- sig: `fn(usize, &game_core::PlayerState, &game_core::OperationData, u64, u64, game_ai::PositionEvalPurpose) -> game_core::PositioningScore`
- consts: [{"value": 48, "src_line": 373, "meaning": "ProfTimer phase id(함수 전체 _t) — 계측용, 판정 아님", "kind": "산출값", "ev": 4}, {"value": 112, "src_line": 406, "meaning": "ProfTimer phase 112 (_t_peu1: 406~447 후보 수집 구간)", "kind": "산출값", "ev": 4}, {"value": 113, "src_line": 448, "meaning": "ProfTimer phase 113 (_t_peu2: 448~ 정글/타워 구간)", "kind": "산출값", "ev": 4}, {"value": 32000, "src_line": 378, "meaning": "셀 크기 — xi = x/32000, yi = y/32000 (좌표변환). effect.rs:26 Effect::range 의 +32000(1셀 여유)도 같은 값(464) · 오라클 실행 확증(24차 배치A: 오라클 o180.exe(_verify24/A/oracle/o180.rs · pub 호출자 game_ai::position_eval_at 경유 · 케이스당 프로세
- 0.5.8 logic 전문:
```
// position_eval.rs:0~507 (배치 A)
// 시그니처: (sret score, version, player, data, x, y, purpose) — 호출자 position_eval_at(293~324)가 POS_EVAL_CACHE miss 때만 부름(tls 절).
373: _t = ProfTimer::start(48)   // prof::ENABLED 가 0 이면 Instant::now 생략(phase 48, nanos 슬롯 PHASE_NANOS+384)
374: champ = data.cache.player_champion[player.info.team][player.info.position.as_index()]   // team<2 bounds 패닉 가드
375: if champ is None → return PositioningScore::default()   // memset(%0, 0, 50)
378: xi = clamp(x/32000, 0, 29);  379: yi = clamp(y/32000, 0, 29)
380: if data.context.map.walls[yi][xi] != 0 →
381:     return PositioningScore{ risk: 9999, ..default }   // store 9999 @0 + memset(@8, 0, 42)
384: inv_hp_q32 = 2^32 / max(champ.hp, 1)
390: score = default (SSA: 8필드 전부 0)   // 400: tower_well_risk = 0 (초기)
393: enemy_team = 1 - team
394: visible = game.is_visible_cell(enemy_team, xi, yi)   // vtable+0x100 · 배치 A 범위에선 미사용(뒤 배치 소비)
395: cell_dist_sq = |ex, ey| axis_distance_sq(x, ex).saturating_add(axis_distance_sq(y, ey))   // closure#0, axis_distance_sq(score_parameter.rs:8~9) = abs_diff(a,b) 의 saturating 제곱
397: tower_well_risk = if path_finder::is_enemy_well_danger(version, player, x, y) { 9999 } else { 0 }
398: score.risk = tower_well_risk;  399: score.tower_risk = tower_well_risk
403: champ_cache = &cache.player_champion_cache[team][pos]
405: (enemy_mask, ally_mask) = pe_cand_masks(seed=game.seed(), tick=game.tick(), team, cache, data.blackboard, enemy_team, player)   // PE_CAND_MASKS TLS · bit i = 적 i 가 small_action 있음 && 챔프 존재 && is_recent_visible / 아군 i 가 small_action 있음 && 챔프 존재
406: _t_peu1 = ProfTimer::start(112)
407: near_enemies_with_action = Vec::new_in(ctx.pool)   // 원소 448B = (EntityPositioningCache 424, &PlayerState, &Entity, dist)
408: for i in 0..5 {
409:     if enemy_mask & (1<<i) == 0 { continue }
412:     e = cache.player_champion[enemy_team][i].unwrap()
413:     dist = cell_dist_sq(e.x, e.y)
414:     if dist > 40000000000 (=200000²) { continue }
418:     eplayer = cache.player_state[enemy_team][i].unwrap()
419:     e_cache = &cache.player_champion_cache[eplayer.team][eplayer.pos]   // eplayer.team<2 bounds 가드
420:     epc = entity_positioning_cache_cached(version, data, champ, e, player, eplayer, champ_cache, e_cache, has_near_enemy=false)   // EPC_CACHE[team][pos][eteam][epos][0]
421:     near_enemies_with_action.push((epc, eplayer, e, dist))
     }
424: has_near_enemy = !near_enemies_with_action.is_empty()
425: near_allies_with_action = Vec::new_in(ctx.pool)
426: for i in 0..5 {
427:     if ally_mask & (1<<i) == 0 { continue }
430:     e = cache.player_champion[team][i].unwrap()
431:     dist = cell_dist_sq(e.x, e.y)
432:     if dist > 40000000000 || e.id == champ.id { continue }   // 자기 자신 제외(or 의 두 항 순서는 IR 상 dist 비교가 먼저 계산되나 단락 없음 — 둘 다 순수)
436:     eplayer = cache.player_state[team][i].unwrap()
437:     e_cache = &cache.player_champion_cache[eplayer.team][eplayer.pos]
438:     epc = entity_positioning_cache_cached(version, data, champ, e, player, eplayer, champ_cache, e_cache, has_near_enemy)   // EPC_CACHE[..][has_near_enemy as usize]
439:     near_allies_with_action.push((epc, eplayer, e, dist))
     }
442: pe_ctx = pe_player_ctx(version, player, data, champ)   // PE_PLAYER_CTX TLS (seed,tick,player.id 키) → PePlayerCtx 40B
443: skill_linear_move = pe_ctx.skill_linear_move(+0x21);  444: skill2_linear_move(+0x22);  445: ult_linear_move(+0x23)   // 배치 A 범위에선 미사용(뒤 배치 소비)
447: drop(_t_peu1)   // phase 112 nanos/calls 누적(ENABLED 일 때만)
448: _t_peu2 = ProfTimer::start(113)
449: for j in cache.jungles.iter() {   // Vec<&Entity> @0xd0/0xe8
450:     jd = cell_dist_sq(j.x, j.y)
451:     if jd > 22499999999 { continue }   // 150000² 이상 제외
454:     match j.ty.tag {
456:       Jungle(4) => if info.focused == Some(champ.id) {   // +0x88 tag==1 && +0x90 == champ.id
457:            damage = expected_attack_damage_cached(attacker=j, target=champ)   // ATTACK_DMG_CACHE[(j.id, champ.id)]
458:            ratio = min((inv_hp_q32*100*damage) >> 32, 150)   // ≈ min(100*damage/hp, 150) 이지만 같지 않다 — inv_hp_q32=floor(2^32/hp) 를 먼저 내림하므로 hp 가 2^32 를 나누지 못하면 1 작다(i128 산술 · 오라클 hp=1000 → 59(소박식 60) · hp=600 → 99(소박식 100)). 재구현은 반드시 Q32 식 그대로
460:            atk = j.attack_effect.as_ref().unwrap()   // +0x4c0 == -1 → unwrap 패닉
461:            if atk.is_in_range_pos(j, champ, x, y) {
462:                score.risk += ratio
            } else {
464:                r = atk.range(j, champ) + j.radius() + champ.radius()   // Effect::range 인라인 = attack_effect.range + 32000 + j.stat_buff_cached.range + (j.level-1)*growth_range + Effect::range_adjust(atk, j, champ); Entity::radius = mult==0 ? radius : radius*(mult+100)/100
466:                if jd <= r*r {   // IR: jd > r*r 이면 skip
467:                    score.risk += ratio >> 1   // 반값
                }
            }
          }
473:       Epic(5) => if info.focused.is_some() {
474:            damage = expected_attack_damage_cached(j, champ);  475: ratio = min((inv_hp_q32*100*damage) >> 32, 150)   // pct_q32 — 458 과 동일한 Q32 식(소박식 100*damage/hp 보다 1 작을 수 있음)
476:            atk = j.attack_effect.as_ref().unwrap(); if atk.is_in_range_pos(j, champ, x, y) {
477:                if info.focused == Some(champ.id) { 478: score.risk += ratio }
480:                else { score.risk += (ratio as u8) / 3 }   // trunc i128→i8 후 udiv 3 (ratio ≤150 이라 손실 없음)
            }
          }
       _ => {}   // Minion/Tower/Serpen/Ghoul/… 무시
     }
     }
489: disable_tick = pe_ctx.disable_tick   // = GameSetting.tower_attack_disable_tick(/2v2/3v3 튜토리얼 분기)
490: if game.tick() > disable_tick { → 병합점 %548: score.gain = 0 유지, 줄 573 으로 (배치 B) }   // ⚠ 방향 주의: tick 이 disable_tick 을 **넘으면** 491~572 타워 절을 통째로 건너뛴다(IR m07.ll:25967~25968 `icmp ugt tick, disable_tick → %548`)
     else {
491:   player_team = champ.player_team().unwrap()   // Entity+0 tag==0(Player) 아니면 unwrap 패닉, 페이로드 +8
492~494: enemy_towers = cache.iter_towers_without_nexus(enemy_team)   // 120B 이터레이터 = [top,top2,mid,mid2,bottom,bottom2] Option 6개 flatten + twin_towers 슬라이스 chain
            .filter(closure#1: |t| cell_dist_sq(t.x,t.y) < 22500000000 || near_enemies_with_action.iter().any(|(_,_,e,_)| e.distance_sq(t) < 4900000000))
495~497: ally_towers = cache.iter_towers_without_nexus(team).filter(closure#2: 동형, near_allies_with_action)
499~500: near_towers = enemy_towers.chain(ally_towers).map(|t| (t, cnt = if t.team != champ.team { tower_minion_in_range_count_cached(data, minion_team=player_team, t) } else { 0 }))   // TOWER_MINION_CNT_CACHE[(t.id, player_team)]
507:   for (t, cnt) in near_towers { → 배치 B(줄 510~) 본문 · 루프 백엣지 %3672→%637 은 score.risk/tower_risk/gain/tower_well_risk 4 phi 를 들고 돈다 }
       루프 종료(%738→%867) → %548 병합 → 줄 573 (배치 B)
     }
// 배치 A 밖: %2619(1001)·%3487(639)·%3635(510)·%3672(507 백엣지)·%4102(1181 반환) 블록은 phi 태그(390/399/507)만 내 범위 줄이고 로직은 B~D 것.

// position_eval.rs:510~675 (배치 B)
// ── 공통(배치 A 정의, 재확인만): champ=%122=cache.player_champion[team][pos] · ctx=%135=data.context · game=%118=cache.game(dyn, +0 data/+8 vtable) · my_id=%231=champ+0x5c0
//   inv_hp_q32=%149=2^32/max(champ.hp,1) · pct_q32(x)[L386~387] = min((inv_hp_q32*100*x)>>32, 150) = 피해/내 현재HP % (상한 150)
//   cell_dist_sq = closure#0(L395) = |x−ex|²+|y−ey|² (saturating) · distance_sq(a,b) = 같은 식(포화 없음)
//   루프 캐리(배치 A L507 `for t in near_towers` phi): score.risk=%640 · tower_risk=%639 · gain=%638 · 지역 tower_well_risk=%641 · cnt=%3636(배치 A closure#3 L498 산출 — 의미는 배치 A)
//
// ===== [A] 타워 루프 본문 L510~565 (진입: 배치 A L507 %841/%862/%863 → %3635 · 탈출: %3672 → 배치 A %637 L507) =====
// L510: let t_atk = t.attack_effect.as_ref().unwrap();                 // Entity+0x490, 태그 +0x4c0 == −1 → unwrap_failed 패닉
// L511: if t.team == champ.team {                                       // team 태그 0(Player) && 팀번호 동일
// L512:   if t.ty == Tower(2) {
// L513:     let any_enemy_in_tower_range = near_enemies_with_action.iter()
// L514:         .find(|e| t_atk.is_in_range(t, e.2));                    // closure#4 · e.2 = 원소+0x1b0 &Entity, stride 448
//           None → continue
// L518:     let damage = expected_attack_damage_cached(data, ctx, t, target);        // TLS AttackDamageCache 키 (t.id, target.id)
// L519:     let damage = damage + tps*damage / t.attack_cooltime();                   // cooltime 0 → div_by_zero 패닉
// L520:     let ratio = pct_q32(damage);
// L521:     if t_atk.is_in_range_pos(t, champ, x, y) {
// L522:       score.gain += (ratio as u8)/3; }
//           continue
//         } else continue
// L525: } else if t.ty == Tower(2) {                                     // 적(또는 비-Player) 타워
// L526:   let damage = expected_attack_damage_cached(data, ctx, t, champ);          // 키 (t.id, my_id)
// L527:   let damage = damage + tps*damage / t.attack_cooltime();
// L528:   let ratio = pct_q32(damage) as i64;
// L529:   let range = t_atk.range(t) + t.radius() + champ.radius();
//         //   Effect::range [effect.rs:26 인라인] = t.stat_buff_cached.range(0x438) + eff.range(0x4a0) + eff.growth_range(0x4a8)*(t.level(0x5c8)−1) + Effect::range_adjust(eff, t, champ)
//         //   Entity::radius [entity.rs:1511~1515 인라인] = if stat_buff_cached.radius_mult(0x470)==0 { radius(0x680) } else { radius*(100+mult)/100 }
// L532:   if !t_atk.is_in_range_ex(t, champ, t.x, t.y, x, y, 18000) { continue }
// L536:   let v47_soaker: (bool, usize) = v47_siege_stance(version, data, player, t);   // {i64,i64}
// L537:   let soaker_is_me = v47_soaker.0 && v47_soaker.1 == my_id;
// L539:   if t.info.nearest_enemy.map(|(_, id)| id == my_id) == Some(true) {        // Tower.nearest_enemy Option<(usize,usize)>: 태그 0x88 · .1 = 0x98 · closure#5
// L540:     let r = if soaker_is_me { ratio/3 } else { ratio };
//           score.risk += r; tower_well_risk += r;                                   // ★tower_risk 불변
//   } else {
// L544:     let dist = cell_dist_sq(t.x, t.y);
// L545:     let coef = if dist < (range/2)² { 100 } else { 50 };
// L547:     let risk = if soaker_is_me                                       { L548: ratio/3 }
// L549:       else if v47_soaker.0 && v47_tower_covered_for_me(data, t, my_id)  { L550: coef*3/100 }   // 3 또는 1
//                //  v47_tower_covered_for_me [tower_discipline.rs:597~613 인라인] = t.ty==Tower && t.nearest_enemy.is_some() && (tid = nearest_enemy.1) != my_id
//                //     && game.get_entity_by_id(tid) is Some(tgt) && t.attack_effect.is_some() && tgt.hp(0x670) > Effect::expected_damage_target(t_atk, ctx, t, @anon.11, tgt)
// L551:       else if !near_enemies_with_action.is_empty()  { L552: if t.nearest_enemy.is_some() { L555: ratio/2 } else { ratio } }
// L557:       else if cnt < 2                                { ratio }
// L559:       else if cnt == 2                               { L560: ((ratio*2)/3 * coef) / 100 }   // i16 산술
//             else                                           { L562: coef*3/100 };
// L565:     score.risk += risk; score.tower_risk += risk; tower_well_risk += risk;
//   }
//
// ===== [B] 라인 미니언 위험 L573~605 (진입: 배치 A L490 %542 · 타워 루프 종료 %867 → %548) =====
// L573: if position_eval_use_legacy_line_minion_risk(version, data, champ, purpose) [position_eval.rs:21~25 인라인, AND 3조건 이 순서]
//   L23: is_line_phase(ctx, tick) [runner.rs:399 / spawn_epic:263 / setting.rs:703~704] = !(ctx.tutorial(0x38) ∈ {0 None,5 MidBottom,7 Line,8 Total}) || tick < sat_sub(setting.epic_jungle.first_spawn_tick(0x8a8), tps*30)
//   L24: purpose ∈ {LineStyle(_)=태그0·1, Lane=8, LaneSafe=10}
//   L25: !enemy_minion_wave_has_epic_buff(data, champ) [minion_wave_risk.rs:92~96] = !(champ.player_team().is_some() && game.get_game_mode().as_moba().map_or(false, |m| m.remain_epic_time(1−team)(MobaMode+0x240) != 0))
//   → 참 = [B-2] legacy(L580) / 거짓 = [B-1](L574)   (if/else 표기 순서는 표기 불가 — 동작만 확정)
// [B-1] L574: let minion_wave_damage = enemy_minion_wave_risk_damage_at(version(poison), data, champ, x, y, L575: tps/2);
//       L576: score.risk += position_eval_minion_wave_risk_score(ctx, purpose, damage, champ.hp, champ.stat_cached.hp) [position_eval.rs:7~15 인라인]
//             L7: pct = position_eval_pct(damage, hp)[score_parameter.rs:53~57] = damage==0 ? 0 : min(damage*100/max(hp,1), 150); risk = min(pct, 140)
//             L8: if pct == 0 { return 0 }
//             L12: if is_line_phase(ctx, tick) L13: && purpose ∈ {LineStyle(_), Lane, LaneSafe} L14: && !enemy_minion_wave_is_dangerous(damage, hp, max_hp)
//                  [minion_wave_risk.rs:69~75] hp_pct=hp*100/max(max_hp,1) · damage_pct=damage*100/max(hp,1) · dangerous = damage>=hp || damage_pct>49 || (hp_pct<66 && damage_pct>29) || (hp_pct<41 && damage_pct>17) || (hp_pct<26 && damage_pct>9)
//             L15: { risk = min(risk/4, 18) }  → risk
// [B-2] L579: let mut range_minion_attack: Option<i64> = None; let mut melee_minion_attack: Option<i64> = None;   // 종류별 ratio 1회 계산 캐시
//       L580: for m in cache.iter_minions(enemy_team).filter(|m| cell_dist_sq(m.x, m.y) < 22500000000) {   // closure#6 = aux · 150000²
//       L582:   let (is_range, target_none, targets_other) = if m.ty == Minion(1) { (L584: m.info.is_range(0x118), L585: m.info.nearest_enemy(0x88).is_none(), L586: nearest_enemy.is_some_and(|id| id != my_id)(closure#7, id 0x90)) } else { (false,false,false) };
//       L590:   let ratio = if is_range { L591: if range_minion_attack.is_none() { L592: r = pct_q32(expected_attack_damage_cached(m, champ)); range_minion_attack = Some(r); r } else { L597: unwrap } }
//                           else       { L594: if melee_minion_attack.is_none() { L595: r = pct_q32(expected_attack_damage_cached(m, champ)); melee_minion_attack = Some(r); r } else { L597: unwrap } };
//       L598:   let ratio = if target_none { ratio/2 } else if targets_other { ratio/3 } else { ratio };
//       L599:   if cell_dist_sq(m.x, m.y) < 4096000001 /*64000²+1 = 2셀*/ { score.risk += ratio } }
// L605: drop(_t_peu2) — ProfTimer 텔레메트리(PHASE_NANOS/PHASE_CALLS fetch_add, prof::ENABLED 게이트)
//
// ===== [C] 기타 적 엔티티 L606~631 =====
// L606: let _t_peu3 = ProfTimer::start(114);
// L607: for e in cache.others[enemy_team](0xf0).iter() {
// L608:   if cell_dist_sq(e.x, e.y) > 22500000000 { continue }
// L611:   let Some(atk) = e.attack_effect.as_ref() else { continue };
// L612:   let damage = expected_attack_damage_cached(data, ctx, e, champ);
// L613:   let ratio = pct_q32(damage);
// L614:   let range = atk.range(e) + e.radius() + champ.radius();
// L615:   let range_ext = range + 32000;
// L616:   let nearest_enemy: Option<usize> = match e.ty { Ghoul(7)|Bear(9) => info.nearest_enemy(0x88/0x90), Eagle(10) => L618: info.nearest_enemy(0x70/0x78), _ => None };
// L622:   let targeting_me = nearest_enemy == Some(my_id);
// L623:   let dist_sq = cell_dist_sq(e.x, e.y);
// L624:   if dist_sq <= range² { L625: if targeting_me { L626: score.risk += ratio } else { L628: score.risk += ratio/2 } }
// L630:   else if dist_sq <= range_ext² { L631: score.risk += (ratio as u8)/3 } }
//
// ===== [D] 투사체 L638~675 (L678 가산 · L679 비-LinearDist · L684 · L687 루프 종료 = 배치 C) =====
// L638: let _t_proj = ProfTimer::start(73); let mut has_pcc = false;   (score.on_trajectory / on_periodic_trajectory = false 초기값)
// L639: for p in game.iter_projectile() (vtable+0x210 → ProjectileIter 40B, ::next) {
// L640:   if distance_sq(x, y, p.x, p.y) > 62499999999 { continue }                         // 250000 이상 = 무시
// L641:   if !(p.is_visible || (p.team == champ.team && p.name == "knight_ult")) { continue }
// L642:   if p.move_type.is_targeting() { continue }                                          // [projectile.rs:134] Target | TargetSplash | BouncingTarget{target_id: Some}
// L645:   let Some(caster) = game.get_entity_by_id(p.caster_id) else { continue };
// L648:   if !p.applyed_target.check_projectile(p, champ) { continue }
// L651:   if !p.is_in_orbit(x, y, champ.radius() + 18000) { continue }
// L655:   if p.team == champ.team {
// L656:     let ratio = position_eval_pct(L657: p.expected_heal_target(ctx, caster, champ) + L658: p.expected_shield_target(ctx, caster, champ), champ.hp);
// L661:     score.risk -= ratio;                                                              // 아군 힐/실드 투사체 = 위험 감산
//   } else {
// L663:     let ratio = pct_q32(p.expected_damage_target(ctx, caster, champ));
// L665:     let mut nearest_other_distance: Option<u64> = None;
//           for a in near_allies_with_action.iter() {
// L666:       if p.applyed_target.check_projectile(p, a.2) && p.is_in_orbit(a.2.x, a.2.y, a.2.radius()) {
// L669:         let dist = distance_sq(p.x, p.y, a.2.x, a.2.y);
// L670:         if nearest_other_distance.is_none_or(|d| dist < d) { nearest_other_distance = Some(dist) } } }   // closure#8 (IR: umin)
// L674:     let d = distance_sq(p.x, p.y, x, y);
// L675:     if is_linear_dist_no_penetrate(p) [projectile.rs:121~122 = LinearDist && !penetrate] {
//               if nearest_other_distance.is_some_and(|nd| nd < d) { continue }             // closure#9 · 아군이 먼저 맞아 막힘
//               → 배치 C(줄684): risk += ratio · on_trajectory = true · has_pcc
//           } else → 배치 C(줄679)
//   } }  루프 latch %3487→%1259 · 종료 %1287 → 배치 C(줄687)

// position_eval.rs:678~960 (배치 C)
// ── 투사체 루프 꼬리 (배치 B L639~677 루프 본문 안, 한 투사체 p=%1265 에 대해 ratio=%3398 이 계산된 뒤) ──
L678: score.risk += ratio;                                   // %3481 (m07.ll:32614)
L679: if p.move_type 논리idx == 2 (Periodic, 메모리태그 4) { score.on_periodic_trajectory = true; }   // %3331==2 → sel (32603~32605)
      else                                                  { score.on_trajectory = true; }
      // 진입 경로가 %3454(배치 B, 투사체 ratio 0 경로?) 이면 on_trajectory=true 만 (32611~32613 phi). 다른 필드는 유지
L684: has_pcc = has_pcc || p.has_cc();                      // 이미 true 면 호출 생략 (32619~32629). has_pcc 는 배치 D 소비
      → 루프 헤더 %1259(배치 B L639) 로 복귀
// ── 투사체 루프 종료 후 ──
L687: drop(timer %79 /*Option<ProfTimer>*/);                // prof 텔레메트리: tag!=-1 이면 PHASE_NANOS[phase]+=elapsed, PHASE_CALLS[phase]+=1 (27935~27991). 언와인드 정리패드 %1254 도 같은 drop
L689: if is_in_well_damage(enemy_team, x, y) {              // game.rs:5654 인라인. enemy_team=1-team(%150) 을 `%112==1` 로 접음
      //   enemy_team==0: (x≤64000 ∧ 800000≤y≤960000) ∨ (x≤160000 ∧ 896000≤y≤960000)
      //   enemy_team==1: (800000≤x≤960000 ∧ y≤64000) ∨ (896000≤x≤960000 ∧ y≤160000)
L690:     score.risk += pct_q32(setting.well_damage, inv_hp_q32)  // = min((well_damage*100*inv_hp_q32)>>32, 150) — 우물 데미지의 HP%, 150 캡 (28056~28069)
      }
L693: let mut max_ratio: i64 = 0;
// ── gain 루프 3종: 내가 상대에게 줄 수 있는 최대 ratio (캐시 튜플 = (EntityPositioningCache, &PlayerState, &Entity, dist_sq)) ──
L694: if !champ.is_block_attack() {                         // %1336 (28052)
L695:   for (cache, _, _, dist) in enemies(%95) {           // 5명, stride 448
L696:     ratio = cache.attacked_ratio;                      // +0x20
L698:     if dist > cache.attacked_range_sq {                // +0xc8
L700:       if dist <= cache.attacked_range_ext_sq {         // +0xd0
L701:         max_ratio = max(max_ratio, ratio/2); } }
L699:     else { max_ratio = max(max_ratio, ratio); }
        }
      }
L706: if champ.skill_effect.is_some() /*0x4f8!=-1*/ && champ.skill_cooldown() < 181 /*Champion 이면 +0xb8, 아니면 0*/ {
L707:   for (cache, _, e, dist) in enemies(%95) {
L708~710: ratio=cache.skilled_ratio(+0x28); range=cache.skilled_range(+0x58);
L711:     if pe_ctx.skill_linear_move {                     // %286
L712:       if dist <= cache.skilled_range_sq(+0xd8) && !(range > e.radius() + 80000 + champ.radius()) {   // 짧은 대시: 사거리 안이면 풀
L713:         max_ratio = max(max_ratio, ratio); }
L714:       else if dist <= cache.skilled_range_half_sq(+0xe8) { L715: max_ratio = max(max_ratio, ratio); }
            else if dist <= skilled_range_sq                { L717: max_ratio = max(max_ratio, ratio/2); }
L718:       else if dist <= cache.skilled_range_ext_sq(+0xe0){ L719: max_ratio = max(max_ratio, ratio/3); }
          } else {
L721:       if dist <= skilled_range_sq                     { L722: max_ratio = max(max_ratio, ratio); }
L723:       else if dist <= skilled_range_ext_sq            { L724: max_ratio = max(max_ratio, ratio/2); }
          }
        }
L728:   for (cache, _, a, dist) in allies(%93) {            // 아군에게도 같은 사다리(대상 아군 스킬 = 힐/버프 추정, 필드명은 동일 skilled_*)
L729~733: ratio=skilled_ratio; range=skilled_range; r_sq/r_half_sq/r_ext_sq = 0xd8/0xe8/0xe0
L734:     if skill_linear_move { L735: (dist≤r_sq ∧ !(range > a.radius()+80000+champ.radius())) → L736 풀 / L737 dist≤half → L738 풀 / dist≤sq → L740 /2 / L741 dist≤ext → L742 /3 }
L744:     else { L745 dist≤sq → 풀 / L746 dist≤ext → L747 /2 }
        }
      }
L752: if champ.skill2_effect().is_some() /*level>2 이고 0x530!=-1*/ && champ.skill2_cooldown() < 181 /*+0xc0*/ {
L753:   for enemies: ratio=skilled2_ratio(+0x30) range=skilled2_range(+0x60) sq/ext/half=0xf0/0xf8/0x100; L757 skill2_linear_move(%289): L758(짧은대시)→L759 풀 / L760 half→L761 풀 / sq→L763 /2 / L764 ext→L765 /3 ; 비직선 L767 sq→L768 풀 / L769 ext→L770 /2
L774:   for allies : 동일 (L778 분기, L779~L786 직선 / L788~L791 비직선)
      }
L796: if champ.ult_effect().is_some() /*level>4 이고 0x568!=-1*/ && champ.ult_cooldown() < 181 /*+0xc8*/ {
L797:   for enemies: ratio=ulted_ratio(+0x38) range=ulted_range(+0x68) sq/ext/half=0x108/0x110/0x118; L801 ult_linear_move(%292): L802→L803 풀 / L804 half→L805 풀 / sq→L807 /2 / L808 ext→L809 /3 ; 비직선 L811 sq→L812 풀 / L813 ext→L814 /2
L818:   for allies : 동일 (L822 분기, L823~L830 직선 / L832~L835 비직선)
      }
L840: score.gain += max_ratio;  score.gain_me = max_ratio;   // gain 은 배치 B 까지의 값(%549)에 가산 (29002~29006)
L846: my_role = pe_ctx.my_role;                              // PePlayerCtx+0x20
L847: is_attacker_role = (my_role & 6) == 2;                 // BaseAttacker|SkillCaster → %1787, 배치 D L1001 소비
L850: drop(timer %80→%76);                                  // prof
L851: timer %75 = ProfTimer::start(phase 115);              // prof::ENABLED 일 때만 Instant::now
// ── 적 챔피언별 위협 루프 (배치 D 로 이어짐) ──
L853: champ_threat_risk = 0; /*score.unseen_champ_threat 루프 phi 초기 0*/
      for (cache, _, e, dist) in enemies(%95) {             // 루프 헤더 %2004, 재진입 %2688(배치 D L1018) 에서 score.risk/unseen_champ_threat/champ_threat_risk 누적값 phi
        attack_ratio=skill_ratio=skill2_ratio=ult_ratio=rush_ratio=0; has_cc=false;
L860:   dist = cache.dist;
L862:   if !(dist < 10000000000 /*100000²*/ || game.is_visible(enemy_team, champ.id) /*vtable+0xf8*/) { continue; }
L866:   ratio = cache.attack_ratio(+0x0);
L867:   if dist > cache.attack_range_sq(+0x70) {
L870:     if dist <= cache.attack_range_ext_sq(+0x78) {
L871:       attack_ratio = max(ratio/2, 0);
L872:       has_cc = slot_cc_time_cached(version, data, e, 0).is_some(); } // else attack_ratio=0, has_cc=false
        } else {
L868:     attack_ratio = max(ratio, 0);
L869:     has_cc = slot_cc_time_cached(version, data, e, 0).is_some();
        }
L874:   if e.is_block_attack() { attack_ratio /= 3; }          // udiv (29590~29591)
L878:   if slot_ready_cached(data, e, 1) {                       // 적 skill 준비됨
L879~881: ratio=cache.skill_ratio(+0x8); range=cache.skill_range(+0x40);
          if cache.is_skill_linear_move(+0x1a0) {
L882:       if dist <= cache.skill_range_ext_sq(+0x88) && !has_cc { L883: has_cc = slot_cc_time_cached(version,data,e,1).is_some(); }
L885:       if dist <= cache.skill_range_sq(+0x80) && !(range > e.radius() + 80000 + champ.radius()) { L886: skill_ratio = max(ratio,0); }
L887:       else if dist <= cache.skill_range_half_sq(+0x90)      { L888: skill_ratio = max(ratio,0); }
            else if dist <= skill_range_sq                        { L890: skill_ratio = max(ratio/2,0); }
L891:       else if dist <= skill_range_ext_sq                    { L892: skill_ratio = max(ratio/3,0); }
          } else {
L894:       if dist <= skill_range_sq { L895: skill_ratio = max(ratio,0); L896: if !has_cc { has_cc = cc(e,1).is_some(); } }
L897:       else if dist <= skill_range_ext_sq { L898: skill_ratio = max(ratio/2,0); L899: if !has_cc { has_cc = cc(e,1).is_some(); } }
          }
        }
L903:   if slot_ready_cached(data, e, 2) {                       // skill2: 필드 +0x10/+0x48/+0x1a1, sq/ext/half=0x98/0xa0/0xa8
L904~927: 위와 동일 사다리 (직선: L907 cc게이트·L908 cc / L910→911 풀 / L912→913 풀 / →915 /2 / L916→917 /3 ; 비직선: L919→920 풀·L922 cc / L923→924 /2·L925 cc)
        }
L929:   if slot_ready_cached(data, e, 3) {                       // ult: 필드 +0x18/+0x50/+0x1a2, sq/ext/half=0xb0/0xb8/0xc0
L930~953: 동일 사다리 (직선: L933 cc게이트·L934 cc / L936→937 풀 / L938→939 풀 / →941 /2 / L942→943 /3 ; 비직선: L945→946 풀·L948 cc / L949→950 /2·L951 cc)
        }
L955:   if e.is_block_skill() { L956: skill_ratio /= 3; L957: skill2_ratio /= 3; L958: ult_ratio /= 3; }   // udiv
L960:   if e.is_block_move_skill() → 배치 D(줄 961, %2393) / else → 배치 D(줄 961, %2388)   // 루프 본문 나머지(rush_ratio·added·score.risk/unseen_champ_threat 누적, %2688 에서 헤더 %1994 로 복귀)는 배치 D
      }

// position_eval.rs:961~1653 (배치 D)  — 실효 961~1181 (1181 = `}`; 1183~ 는 별개 함수)
// 문맥(배치 C 승계): 853 `for cache in near_enemies_with_action(%95)` 루프 안. cache=(EPC .0, &PlayerState .1, ent: &Entity .2, dist: u64 .3 = 셀↔ent 거리²). 루프 진입 조건 862(dist<1e10 || 슬롯 시야) 통과 후 attack_ratio(%2056)·skill_ratio(%2380)·skill2_ratio(%2379)·ult_ratio(%2381)·has_cc(%2273, 872~) 가 배치 C 에서 계산돼 들어온다. 루프 캐리: score.risk(%1997)·score.unseen_champ_threat(%1996)·score.on_trajectory(%1995)·champ_threat_risk(%1999, 초기 0).

// ── 961~968: 이동 가능 스킬 감쇠 (m07.ll:30217~30334)
961: if ent.skill_effect.as_ref().map_or(false, |e| e.ty.can_move())   // Option tag@0x4f8 != -1 · vtable 0x120
       { skill_ratio /= 3 }          // udiv 3 (30247)
964: if (level>2 ? ent.skill2_effect : None).map_or(false, |e| e.ty.can_move()) { skill2_ratio /= 3 }   // 30256~30290
967: if (level>4 ? ent.ult_effect : None).map_or(false, |e| e.ty.can_move()) {
968:   ult_ratio /= 3 }                                                     // 30296~30332

// ── 971~975: 입력 차단 상태 반감 (30212·30359~30366)
971: if Entity::block_input(ent) {
972:   attack_ratio >>= 1;  973: skill_ratio >>= 1;  974: skill2_ratio >>= 1;  975: ult_ratio >>= 1 }

// ── 978~990: 돌진 궤도 위험 rush_ratio (30348~30662)
     rush_ratio = 0;   // on_trajectory 는 기존값 유지
978: match ent.rush_state {   // tag: raw@0x308 <0 ? raw^0x8000… : 4
       RushState::Rush{applyed_effect@0x310, x@0x330 ex, y@0x338 ey, range@0x340, start_tick@0x348, casting_target@0x35c, ..}
979:   | RushState::RushPenetrate{applyed_effect@0x308, x@0x340, y@0x348, range@0x350, start_tick@0x358, casting_target@0x36c, ..} => {
980:     if CastingTarget::check(&casting_target, ent, champ)                          // 30431
981:        && game.tick() >= start_tick                                             // vtable 0x28 · `tick < start_tick` 이면 skip (30443)
982:        && dist_to_line_segment(x, y, ent.x, ent.y, ex, ey)                       // 점=평가 셀, 선분=ent 현위치→돌진 목적지 (30455)
983:             <= range + 20000 + champ.radius()   {                                // radius()=radius_mult 보정 (30460~30481)
984:       let (ad, ap) = applyed_effect.iter().fold((0,0), |(ad,ap),(e,_ct)| {
985:            let (ad2, ap2) = e.ty.expected_damage(ctx, ent as &dyn AbstractEntity);   // vtable 0x28 (30547)
986:            (ad+ad2, ap+ap2) });                                                       // closure#14 인라인 (30513~30570)
987:       let dmg = get_damage::<Entity>(ent, champ, ad, AttackType::Skill(1), DamageType::AD(0))   // 30577
988:               + get_damage::<Entity>(ent, champ, ap, AttackType::Skill(1), DamageType::AP(1));  // 30582
989:       rush_ratio = min(pct_q32(inv_hp_q32, dmg), 150);   // = min((inv_hp_q32*100*dmg)>>32, 150) ≈ 내 HP 대비 % (30590~30596)
990:       if !has_cc { has_cc = applyed_effect.iter().any(|(e,_)| effect_type_cc_time(version, &*e.ty).is_some()) }   // closure#15, .0==1 = Some (30600~30658)
           score.on_trajectory = true;   // 30660 (980 root)
         }   // 980~983 중 하나라도 실패 → rush_ratio=0, on_trajectory 유지 (%2480)
       }
       _ => {}   // None/Move/MoveToTarget
     }

// ── 995~998: 이 적의 risk 합산 규칙 (30377~30688)
995: let risk = if has_cc || has_pcc {          // has_pcc=%1263 = 배치 B 639 투사체 루프 산출
996:     skill_ratio + skill2_ratio + attack_ratio + ult_ratio + rush_ratio          // 전부 합
     } else {
998:     rush_ratio + max(attack_ratio, skill_ratio, skill2_ratio, ult_ratio) };    // 최대 1개 + 돌진

// ── 1001~1008: 후방 딜러의 아군 몸빵 반감 (30697~30829)
1001: if matches!(my_role, BaseAttacker|SkillCaster) && risk > 0 {     // (my_role&6)==2 · my_role = PePlayerCtx+0x20 (배치 C 847)
1003:   if near_allies_with_action.iter().any(|a| {                       // closure#16 인라인, 원소 448B
1004:        dist_sq(a.entity.pos, ent.pos) < cache.dist                   // 아군이 나(셀)보다 적에 가깝고 (30768~30781)
1005:     && dist_to_line_segment(a.entity.x, a.entity.y, x, y, ent.x, ent.y)   // 아군이 셀→적 선분 근처 (30787)
1006:          <= a.entity.radius() + 28000 })                              // 30791~30815
1008:   { risk >>= 1 } }

// ── 1012~1018: 미시야 분할·누적 (30835~30856)
1012: let added = if visible { risk } else { risk / 2 };    // visible=%156 = game.is_visible_cell(enemy_team, xi, yi) (배치 A 394) · sdiv 2
1013~1015: if !visible { score.unseen_champ_threat += risk - added }
1017: score.risk += added;
1018: champ_threat_risk += added;
     // → 853 루프 다음 적

// ── 1024~1030: 루프 종료 후 고립 가산 (29507·30860~31080)
1024: if champ_threat_risk > 0 {
1025:   let cell_a = near_allies_with_action.iter().filter(|a| a.dist <= 120000²).count();   // ult 14400000001 (closure#17)
1026:   let cell_e = near_enemies_with_action.iter().filter(|e| e.dist <= 120000²).count();  // closure#18
1029:   if cell_a == 0 && cell_e > 1 {
1030:     score.risk += champ_threat_risk * 25 * min(cell_e as i64, 4) / 100 } }   // +25%/적, 최대 4명 (31073~31077)
1034: drop(_t)   // ProfTimer phase 115 (배치 C 851) — PHASE_NANOS/CALLS[115] atomic add, prof 전용

// ── 1035~1096: 아군 스킬 지원 gain (31087~31610)
1035: let _t = ProfTimer::new(116)   // prof::ENABLED 일 때만 Instant::now
1036: for a in near_allies_with_action {           // score.gain 캐리 %2779 (초기 %1783 = 배치 C 840)
1038:   let dist = a.dist;   1039: let ally = a.entity;  let mut max_ratio = 0;
1039:   if ally.skill_effect.is_some() {
1040:     let ratio = a.skill_ratio;  1041: let range = a.skill_range;
1042:     if a.is_skill_linear_move {
1043:       if dist <= a.skill_range_sq && !(range > ally.radius() + 80000 + champ.radius())
1044:            { max_ratio = max(ratio, 0) }                       // 31294
1045:       else if dist <= a.skill_range_half_sq { 1046: max_ratio = max(ratio,0) }   // 31212·31233 (⚠1043 조건 실패 후에만)
1047:       else if dist <= a.skill_range_sq  { 1048: max_ratio = max(ratio/2, 0) }    // 1047 은 IR 없음(접힘) · 31298
1049:       else if dist <= a.skill_range_ext_sq { 1050: max_ratio = max(ratio/3, 0) } // 31225·31237
            // 그 밖 → 0
1052:     } else if dist <= a.skill_range_sq { 1053: max_ratio = max(ratio, 0) }    // 31182·31196
1054:       else if dist <= a.skill_range_ext_sq { 1055: max_ratio = max(ratio/2, 0) }   // 31188·31200
          }
1058:   if level>2 && ally.skill2_effect.is_some() {   // 동형: ratio=a.skill2_ratio(0x10) range=a.skill2_range(0x48) linear=0x1a1 sq/ext/half=0x98/0xa0/0xa8
1059~1074: 1062 `dist<=sq && !(range > r_a+80000+r_c)` →1063 ratio / 1064 half →1065 ratio / 1067 ratio/2 / 1068 ext →1069 ratio/3 // 비선형 1071 sq→1072 ratio / 1073 ext→1074 ratio/2
            max_ratio = max(max_ratio, 그 값) }                          // smax 누적 (31380·31348·31355·31441·31448·31387)
1077:   if level>4 && ally.ult_effect.is_some() {   // ratio=ult_ratio(0x18) range=ult_range(0x50) linear=0x1a2 sq/ext/half=0xb0/0xb8/0xc0
1078~1093: 1081/1082/1083/1084/1086/1087/1088 (선형) · 1090/1091/1092/1093 (비선형) 동형 → max_ratio = max(max_ratio, …) }
1096:   score.gain += max_ratio }                                          // 31599

// ── 1101~1127: 탱커/이니시에이터의 후방 딜러 '진입가치' (31163~31639)
1101: if matches!(my_role, Tanker|Initiator) {                              // my_role < 2
1103:   let mut enabled = 0;
       for ap in near_allies_with_action {
1105:     if matches!(get_battle_role(version, ctx, cache, ap.player), BaseAttacker|SkillCaster)   // (role&6)==2 (31632~31643)
1109:       && dist_sq_sat(셀, ap.entity.pos) <= 120000²   {     // umul.with.overflow→-1 · uadd.sat (31659~31723)
1113:       if let Some(e) = near_enemies_with_action.iter().find(|e| {
1114~1116:        let a_to_e_sq = dist_sq(ap.entity.pos, e.entity.pos);  e.dist < a_to_e_sq     // 셀이 아군보다 적에 가깝고 (31790)
1117~1118:     && dist_to_line_segment(x, y, ap.entity.x, ap.entity.y, e.entity.x, e.entity.y) <= champ.radius() + 28000 })  // 셀이 아군→적 선분 위 (31799~31824)
            {
1119:         if let Some(atk) = ap.entity.attack_effect {            // tag@0x4c0 != -1
1120:           let dmg = atk.expected_damage_target(ctx, ap.entity as &dyn AbstractEntity, e.entity);   // 31837
1121:           enabled += dmg * 100 / max(e.entity.stat_cached.hp, 1) } } } }   // 아군 평타의 적 스탯HP% (31842~31849)
1127:   score.gain += enabled }

// ── 1134~1136: 교환 순이익 (31870~31878)
1134: let non_tower_risk = max(score.risk - tower_well_risk, 0);      // %552 = 배치 B 400 줄
1135: let trade_net = gain_me(max_ratio %1782) - non_tower_risk;
1136: score.gain += max(trade_net, 0);
1141: drop(_t)   // phase 116

// ── 1142: score = apply_position_eval_purpose(score, version, player, data, x, y, purpose)  — 인라인(position_eval.rs:29~44) (31947~32010)
       match purpose { LineStyle(Defensive) => risk = risk*120/100 (33)  |  LineStyle(Aggressive) => gain = gain*120/100 (36)
                       LaneSafe => { risk = risk*3/2 (39); tower_risk = tower_risk*3/2 (40) }  |  _ => {} }
       *sret = PositioningScore{ risk, tower_risk, gain, gain_me, adjust: 0, unseen_champ_threat, on_trajectory, on_periodic_trajectory }   // 44줄 store 8건

// ── 1148~1176: 선수 포지셔닝 정확도 노이즈 (32011~32157)
1148: let acc = AthleteParameter::positioning_accuracy(&player.info.parameter);
1152: let noisy = if version > 1 { !matches!(purpose, RunAway|Recall) && acc < 1000 }
1153:             else { acc < 1000 };
       if !noisy { return }   // → 1181 drop 경로 (%3223)
1154: let base_risk = risk - tower_risk;
1155: if base_risk != 0 {                                           // risk == tower_risk 면 return
1157:   let spread = if game.get_game_mode() is DeathMatch(tag 2)    // vtable 0x40
1158:                { 1000 - acc }
1160:                else { (2000 - 2*acc) / 3 };
1165:   let t_salt = if version > 1 { 1166: game.tick() / max(tps*6, 1) }   // 6초 버킷 (tps = setting.tick_per_second)
1168:                else { game.tick() };
1170:   let mut h = player.info.id;
1171:   h = (h ^ t_salt) * 0x9E3779B97F4A7C15;
1172:   h = (h ^ (x/32000)) * K;      // %130 = 클램프 전 x 셀
1173:   h = (h ^ (y/32000)) * K;      // %132
1174:   h ^= h >> 31;
1175:   let factor = (h % (2*spread + 1)) as i64 - spread + 1000;   // 1000 ± spread (‰)
1176:   sret.risk = tower_risk + base_risk * factor / 1000 }   // sdiv, 32154~32157
1181: }   // near_enemies(%95)·near_allies(%93) bumpalo Vec drop · %98(phase 배치 A) ProfTimer drop · ret
```
- logic_note: ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)

### `e8b5e0` → `e9de70` SerpenCheckSubPlan::action_candidates (i=192 · ⚠변경·다건(r20))
- 0.5.8 src: `game-ai\src\plan_legacy\sub_plan\serpen_check.rs:14` · one_line: 서펜 스틸 Lurk(SerpenCheck) 서브플랜의 후보 생성 — 위험(논타겟 윈드업·궤적)이면 도주 단일, 아니면 v25 태세→서펜 캠프/(아군 Rhino) 경유지 대기 + 근접 적(최근 시야 챔프·others 150000 이내)이면 도주 추가 + (적에게 보이면) 전투 후보 + 소환물 공격 후보
- 0.6.0 판정: **다건** · 패치 요지: L25 안 `if tp.cc0==0 {move_check=true} else { near>=REQ[tutorial] || anchor.pos==my_pos }` · 표 [3,2,1,2,1,3,1,3,3] · 바이어스 [60000,0,40000,60000,20000]
- RE 정본: `2026-09-17_r20_변경99_배치C_14_디컴대조_원문.md` · 절: `2026-09-17_r20_변경99_배치C_14_디컴대조_원문.md` §3
- sig: `fn(&mut game_ai::plan_legacy::sub_plan::SerpenCheckSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::Small`
- consts: [{"value": 4900000001, "src_line": 25, "meaning": "70000² + 1 — 아군 Rhino 캠프 접근 판정(제곱거리 ult · 아군 진영일 때만 검사, 적 진영이면 검사 없이 true). 이보다 작으면 move_check=true. 오라클 실행 확인: dx=70000 → 1 · dx=70000,dy=1 → 0 (s_rhino_own_eq/gt) · 오라클 실행 확증(25차 배치L: 오라클 실행 확인(25차 L): s_rhino_own_eq(dsq 4.9e9 → move_check 1) / s_rhino_own_gt(4.9e9+1 → 0) · s_t1_rhino_eq/gt(팀1) 예측 일치)", "kind": "임계", "ev": 2}, {"value": 22500000000, "src_line": 78, "meaning": "150000² — 서펜 캠프와의 제곱거리 ugt: 멀면 경유(rhino/serpen 분기), 아니면 서펜 캠프 주변 대기 · 오라클 실행 확증(25차 배치L: 오라클 실행 확인(25차 L): s_serpen_eq(dsq 2.25e10 → Serpen 직행) / s_serpen_gt_blue(2.25
- 0.5.8 logic 전문:
```
fn action_candidates(&mut self, version, rnd, player, data, parameter, team_plan, debug) -> Vec<SmallActionPlay>
L15  bump = data.context.pool; res = Vec::new_in(bump)
L17  team = player.info.team (bounds<2); champ = data.cache.player_champion[team][player.info.position as usize].unwrap()
L19  if !self.move_check {
L20    if !is_enemy_side(team, champ.x, champ.y)   // 아군 진영일 때만 Rhino 검사. is_enemy_side = (team==0) XOR is_blue_side(x,y), is_blue_side = !((x - y + setting.height) > setting.width)  (map_regions.rs:58, 7~8) · IR %79 = (team==0) XOR ugt = !is_enemy_side (m14.ll:30378~30380)
L23      camp = context.map.camp_pos(JungleType::Rhino, is_blue_side=team==0)
L25      if |champ.x-camp.x|² + |champ.y-camp.y|² < 4900000001 (70000²+1) { self.move_check = true }
       else { self.move_check = true }   // 적 진영이면 즉시 통과 (store 는 dbg L0 블록 %99 공유)
     }
     mc = self.move_check (phi %101)
L35  enemy_team = 1 - team
     has_non_target_action_range = cache.player_champion[enemy_team].iter().flatten().any(|c|
L36    nontarget_windup_perceived(version, player, data, c) && c.ty==Champion(13) &&
L37    match c.action_state { Skill(4) => e=c.skill_effect.unwrap()(casting -1 → panic) ,
L39                           Skill2(5) => e = if c.level>2 {c.skill2_effect} else {DEFAULT_EFFECT(@anon…22)} ,
L41                           Ult(6)    => e = if c.level>4 {c.ult_effect} else {DEFAULT_EFFECT} , _ => false }
       && matches!(e.casting, Position(1)|Direction(2)) && e.is_in_range(caster=c, target=champ))
L48  ps: PositioningScore = position_score_at_position(version, player, data, &parameter.positioning_score, champ.x, champ.y, PositionEvalPurpose::Objective(11))
L50  if ps.on_trajectory(+0x30) || has_non_target_action_range || ps.on_periodic_trajectory(+0x31) {
L52    res.push(RunAway(SmallActionRunAway::new_with_skill(data, player, end_delay=5, with_skill=true)))   // 태그 3
L53    return res }
L57  posture: Option<ObjectivePosture> = team_plan.v25_objective_posture(version, player, data, target=JungleType::Serpen(5))
L58  if let Some(p) = posture {
L59    if p.kind as u8 > 2 {            // WaitGroup(3) | SoftDisengage(4)
L60      if p.kind==SoftDisengage(4) && p.near_enemy_count != 0 {
L61        res.push(RunAway(new_with_skill(data, player, 5, with_skill=false))) }   // 태그 3
L63      res.push(AroundPosition(SmallActionAroundPosition::new(rnd, data, p.wait_pos.0, p.wait_pos.1, end_delay=5)))   // 암묵 variant, 태그 store 없음
L64      if context.debug { L65 debug.infos.entry(champ.id).or_default().push(format!("{:?}", p.kind)) }
L67      return res
       } else if p.kind==Screen(2) && p.focus_enemy.is_some() {
L71      focus = p.focus_enemy.unwrap()
L72      res.push(Trace(SmallActionTrace::new_attack_range(data, target=focus, end_delay=5)))   // 태그 14
       }   // Commit(0)/HoldCamp(1)/Screen-without-focus → 아무것도 안 함
     }
L77  camp = map.camp_pos(Serpen(5), team==0)
L78  if dist²(champ, camp) > 22500000000 (150000²) {
L79    if mc { L80 res.push(AroundPosition::new(rnd, data, camp.0, camp.1, 5)) }
       else { L83 rhino = map.camp_pos(Rhino(0), team==0); L84 res.push(AroundPosition::new(rnd, data, rhino.0, rhino.1, 5)) }
     } else { L87 res.push(AroundPosition::new(rnd, data, camp.0, camp.1, 5)) }
L90  danger = cache.player_champion[enemy_team].iter().flatten().any(|c| data.blackboard[enemy_team].is_recent_visible(cache.game, player, c) && L91 dist²(c, champ) < 22500000001)
       || L92 cache.others[enemy_team].iter().any(|e| dist²(e, champ) < 22500000001)   // 단락: 앞이 참이면 others 미순회
L94  if danger { L95 res.push(RunAway(SmallActionRunAway::new(data, player, end_delay=5))) }   // 태그 3
L98  if cache.game.is_visible(enemy_team, champ.id) {   // vtable+0xf8
L99    res.extend(battle_action(version, rnd, player, data, 5)) }
L101 res.extend(attack_summon_action(player, data))
L103 return res
```
- logic_note: ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
