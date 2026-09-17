# dccc60→ff11f0 GoalData::update

## logic_060
```
fn update(&mut self, version, _rnd, player, data, debug):   // goal_data.rs:82 · ff11f0
  // ---- (A) update_heal_commit (goal_data.rs:27~57, 인라인) ----
  // ★v2 사장: `if version >= 2` 게이트(:28)는 v3 에서 항상 참 → 블록 상시 실행
  champ = cache.player_champion[+0x1e0][player.team(+0xa00)][player.pos(+0xa90)]   // :31 (team>=2 → bounds panic)   // ★0.6.0 +0x930/+0x9c0→+0xa00/+0xa90
  if champ == None { self.heal_commit(+0xf0) = false; }   // :32~33
  else {
    before = self.heal_commit                       // :35
    // ★0.6.0 신규(goal_data.rs:27 앞 삽입 · version==2 이면 스킵 → v3 에서 활성): 불사(undying) 버프 중이면 힐 커밋 해제
    if champ.stat_buff_cached.undying(+0x488) == 1 {
      remain = champ.stat_buff(Vec +0x2d8: ptr@+0x2e0, len@+0x2e8 · 원소 0x120B).iter()
                 .filter(|b| b.undying(+0x118))
                 .map(|b| match b.duration(+0x48 태그) { Permanent(0) => usize::MAX, Time(1){tick@+0x50} => tick, WithShield(2) => 0 })
                 .max()                              // undying 버프 없으면 None → 구 경로(아래)로
      if let Some(r) = remain && r > tps(+0x12f8) { new = false; goto STORE }   // r <= tps 면 구 경로
    }
    if nexus_final_stand(player,data)      { new = false }   // :36 — 실체: 캐시 헬퍼 db0160(seed,tick,player,data) 반환 비트 0x100   // 콜리 v3: db0160(구 c87850)
    else if nexus_is_critical(player,data) { new = false }   // :37 — 인라인: nexus = cache.nexus[+0x170+team*8]; nexus.hp*100/max(nexus.max_hp,1) < 51 && nexus_under_direct_attack(player, data)   // 콜리 v3: e77ec0
    else if before {                                          // :39
      if champ.hp(+0x670) < champ.stat_cached.hp(+0x628) { (store 없음 = true 유지) } else { new = false }   // :40 풀피 도달 → 해제
    } else {                                                  // :44
      (lx,ly,rx,ry) = map(ctx+0x20).fountains[+0x6d70+team*0x20]
      in_heal_area = lx<=champ.x<=rx && ly<=champ.y<=ry      // :45
      hp_ratio = champ.hp*100 / max(champ.stat_cached.hp,1)  // :46
      if !in_heal_area && hp_ratio < 36 && base_defense_focus(player,data) { new = true }   // :47 ★우물 밖에서만 (&& 단락) · 콜리 v3: e74ac0
      else (store 없음 = false 유지)
    }
    STORE: if store 됐고 before != new && ctx.debug(+0x3b) { debug.add_log(format!("HEALCOMMIT T{team} {pos:?} {start|end} hp={hp*100/max_hp}%")) }  // :51~55 · 콜리 v3: 13f0790(구 16fc160)
  }
  // ---- (B) 본진수비 틱 ----
  if base_defense_focus_cached(player,data) { self.last_base_defense_tick(+0xe8) = tick() }   // :84~85 — 실체: db0160 반환 비트 (0x1 | 0x10000)
  // ---- (C) 적 리전 추적 ----
  for p in cache.game.iter_player() {                 // :87 (vt+0x208 · PlayerState stride 0xab0)   // ★0.6.0 stride 0x9e0→0xab0
    if p.team(+0xa00) == player.team { continue }    // :88 아군 스킵
    pos = p.position(+0xa90).as_index()               // :91
    if p.is_dead() {                                  // :93 (+0xa98 == 0)   // ★0.6.0 +0x9c8→+0xa98
      region = if p.team==0 {0} else {26}             // :95
    } else {                                          // :98
      echamp = cache.player_champion[p.team][pos]; if None → goto 만료검사
      if !game.is_visible(vt+0xf8)(p.team, echamp.id) → goto 만료검사      // :99
      region = map.regions(+0x38b8)[min(echamp.y/32000,29)][min(echamp.x/32000,29)]   // :101 (stride 0xf0)
    }
    self.enemy_region[pos](stride 0x18) = Some{tag 1, region, last_known: tick()}   // :103~105
    만료검사: if let Some(r)=self.enemy_region[pos] { if r.last_known + tps*5 < tick() { self.enemy_region[pos]=None } }   // :107~109
  }
  // ---- (D) 오브젝트 태세 ----
  self.epic(+0x78).update_plan(player, data, /*enemy_region=*/self, _)       // :114 · 콜리 v3: ff4940(구 de1ee0)
  self.serpen(+0xb0).update_plan(player, data, /*enemy_region=*/self, debug) // :115 · 콜리 v3: ff2450(구 dd73b0)
  // ---- (E) 디버그 ----
  if ctx.debug { champ = player_champion[player.team][pos]; if Some → debug.map[champ.id].push("epic_stance: {}","serpen_stance: {}","epic_enemy_tick: {}","epic_enemy_killed_tick: {}","epic_ally_tick: {}","epic_ally_killed_tick: {}") }   // :117~126
```

## changes
- (A) 앞 신규: `if version>=3 && champ.stat_buff_cached.undying { remain = max(undying 버프 duration: Permanent→MAX · Time→tick · WithShield→0); if remain > tps { heal_commit=false (+로그) } }` — remain ≤ tps 이거나 undying 버프가 목록에 없으면 구 경로.
- `version < 2` 스킵 게이트 = v3 사장(항상 실행).
- 오프셋: PlayerState team/position/is_dead +0xa00/+0xa90/+0xa98 · iter_player stride 0xab0 · Entity undying +0x488 · stat_buff +0x2e0/+0x2e8 · BuffState duration@+0x48(tick@+0x50) · undying@+0x118.
- 콜리 RVA: 캐시 헬퍼 c87850→db0160 · nexus_under_direct_attack→e77ec0 · base_defense_focus→e74ac0 · epic.update_plan de1ee0→ff4940 · serpen.update_plan dd73b0→ff2450 · 디버그 로그 16fc160→13f0790.
- ⚠0.5.8 logic 의 `nexus_final_stand`/`base_defense_focus`(:36/:84) 는 양버전 모두 **캐시 헬퍼(seed,tick 키) 반환 비트**(0x100 / 0x1|0x10000)로 구현돼 있음 — 이름 추상화는 유지하되 재구현 시 db0160 의 반환 레이아웃(추정: byte0 base_defense · byte1 final_stand · byte2 ?)을 별도 확정 필요.

## verified
- Ghidra 0.6.0 `ff11f0` 전문 디컴: `version==2 || undying(+0x488)!=1 → 구 경로` · stat_buff 루프(0x120 stride · +0x118 · +0x48 태그 0/1/2 → MAX/tick/0 · max) · `remain <= tps → 구 경로` / 초과 → heal_commit=false+로그 · db0160 & 0x100 · nexus(+0x170+team*8) hp%<0x33 && e77ec0 · fountain +0x6d70 · hp%<0x24 && e74ac0 · db0160 & 0x10001 → +0xe8=tick · iter_player(vt+0x208 · stride 0xab0 · +0xa00/+0xa90/+0xa98) · 0/0x1a · region +0x38b8 · tps*5 만료 · ff4940/ff2450 · 디버그 6종.
- Ghidra 0.5.8 `dccc60`(ghidra_beta) 디컴 대조: undying 블록 없음 · 나머지 동형(c87850 비트 0x100/0x10001 동일 · stride 0x9e0 · +0x930/+0x9c0/+0x9c8).
- 미확인: db0160/e77ec0/e74ac0/ff4940/ff2450 내부(브리핑 콜리 변경 없음 · RE §1 「(B)(C)(D) 동치」).

## confidence
A — 신규 블록 전문 디컴 · 나머지 양버전 대조.
