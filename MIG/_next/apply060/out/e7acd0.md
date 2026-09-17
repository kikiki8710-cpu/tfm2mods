# e7acd0→f27590 should_recall_to_shop

## logic_060
```
fn should_recall_to_shop(version, rnd, player, data, goal_data) -> bool   // lib.rs:1642 · f27590
let tps = ctx.setting.tick_per_second;                                                  // L1647 (ctx = data[1] · setting = ctx+8 · +0x12f8)
let epic_alive   = game.get_game_mode().as_moba().map_or(false, |m| m.jungle_runner.epic.live_list.len()   != 0);   // L1648 (vtable+0x40 → (tag,&moba) · tag 0 = Moba · MobaMode+0x1a8)
let serpen_alive = game.get_game_mode().as_moba().map_or(false, |m| m.jungle_runner.serpen.live_list.len() != 0);   // L1649 (MobaMode+0x1d8)
if epic_alive   && min(goal_data.epic.epic_ally_tick(+0x88),   goal_data.epic.epic_enemy_tick(+0x98))   <= tps*20 { return false; }   // L1650 (ugt 면 통과)
if serpen_alive && min(goal_data.serpen.epic_ally_tick(+0xc0), goal_data.serpen.epic_enemy_tick(+0xd0)) <= tps*20 { return false; }   // L1651
if !utils::can_recall(rnd, player, data) { return false; }                                 // L1656 · 콜리 v3: de2a90

if player.info.item_builds(+0x558/+0x560).len() == 0 {                                    // L1660   // ★0.6.0 +0x4e8/+0x4f0→+0x558/+0x560
    if buy_item(_, rnd, player, _, _, ctx).is_some() { return true; }                       // L1661 · 콜리 v3: f27ea0(변경 — 별도 명세)
    return upgrade_item(_, rnd, player, _, _, ctx).is_some();                               // L1662/1666 · 콜리 v3: f27140(동치)
}
// L1664 return item_v26_affordable(player, &ctx.item_list, rnd)   ← 인라인(아래 L1671~1695)
let item_list = &ctx.item_list;                                                            // L1664 (GameContext+0x30 → ptr@+8/len@+0x10)
let (build_slot, inventory_index) = item_v26_slot(player, item_list) else { return false };   // L1671~1672 · 콜리 v3: f3bae0 · 외곽 None 니치 = +0x8 == -1
// L1674
let target = player.info.item_builds.iter().copied()
    .filter(|&i| item_list.get(i).map_or(false, |it| it.is_active()/*vt+0x50*/))
    .nth(build_slot) else { return false };
// L1680~1687
let next_item = if let Some(inv_idx) = inventory_index {
    let inv_item = player.info.items(+0x510/+0x518).get(inv_idx) else { return false };      // L1681   // ★0.6.0 items +0x4a0/+0x4a8→+0x510/+0x518
    random_item_next_toward_target(item_list, inv_item.key()/*vt+0x58*/, item_list[target].key(), rnd) else { return false }   // L1682 · 콜리 v3: e565b0
} else {
    let path = random_item_build_path(item_list, target, rnd) else { return false };         // L1687 · 콜리 v3: e56130 (외곽 -1 = None)
    if path.is_empty() { return false; }
    path[0]
};
// L1692 — ★0.6.0: 인라인 from_iter(활성 보유템 인덱스 Vec) → 헬퍼 count_active_owned 16f2f40(player, item_list.ptr, item_list.len) 로 교체 · 임계 2→3
let active_cnt = player.info.items.iter().filter(|it| item_list.iter().any(|li| li.is_active() && li.key()==it.key())).count();   // 16f2f40
if active_cnt > 3 && item_list[next_item].tier()/*vt+0x70*/ == 0 { return false; }     // ★0.6.0 (구 > 2) · [next_item] bounds 패닉 가능
// L1695
return player.info.gold(+0xa68) >= item_list[next_item].price()/*vt+0x68*/;             // ★0.6.0 gold +0x998→+0xa68 · [next_item] bounds 패닉 가능
```
// ★v2 사장: 없음(version 미사용).

## changes
- L1692: `active.len() > 2` → **`active_cnt > 3`**. 카운트 산출은 `active_inventory_item_indices` 인라인(Vec 생성)에서 헬퍼 `16f2f40 -> usize`(Vec 없이 카운트) 로 교체 — 술어(보유템 key 가 활성 목록에 존재)는 동일.
- 오프셋: items +0x510/+0x518 · item_builds +0x558/+0x560 · gold +0xa68 · goal_data 틱 오프셋(+0x88/+0x98/+0xc0/+0xd0)·MobaMode(+0x1a8/+0x1d8)·tps(+0x12f8) 불변.
- 콜리 RVA: can_recall→de2a90 · buy_item→f27ea0(변경) · upgrade_item→f27140 · item_v26_slot→f3bae0 · random_item_next_toward_target→e565b0 · random_item_build_path→e56130 · 신규 16f2f40.

## verified
- Ghidra 0.6.0 `f27590` 전문 디컴: 에픽/세르펜 게이트(min · tps*0x14) · de2a90 · item_builds len==0 → f27ea0==1 → true / f27140 · f3bae0 · nth(build_slot) 스킵 루프 · inv 분기 e565b0 / e56130 · `16f2f40(player, ptr, len)` → `3 < cnt` → tier(vt+0x70)==0 → false · `price(vt+0x68) <= gold(+0xa68)`.
- Ghidra 0.5.8 `e7acd0`(ghidra_beta) 디컴: 동일 구조에서 `2 < local_48`(인라인 from_iter d7a360 결과 len) — 임계 2→3 변경 확증.
- Ghidra 0.6.0 `16f2f40` 전문 디컴: `Σ_it [∃li: is_active(li) && name(li)==name(it)]`.
- 미확인: 콜리 내부(de2a90/f3bae0/e565b0/e56130) 0.6.0 디컴은 안 함(브리핑 콜리 변경 없음).

## confidence
A — 변경 한 줄 양버전 디컴 대조.
