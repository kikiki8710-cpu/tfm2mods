# dd5db0→f0cfe0 TeamPlan::v24_objective_setup_should_check_camp

## logic_060
```
fn v24_objective_setup_should_check_camp(&self, _version, player, data, goal_data, camp: JungleType) -> bool
  team = player.info.team(+0xa00); enemy = 1 - team; ctx = data.context; cache = data.cache
  // L91: 태세 게이트 — ★0.6.0 TeamPlan 이중모드(구 take_setup_like(camp) = objective(+0x41f)/phase(+0x420))
  //   self.v4_serpen: Option 니치 +0x3e4(2 = None) · phase +0x3e0 / self.v5_morgard: 니치 +0x404(2 = None) · phase +0x400 / 레거시 objective +0xcd5 · phase +0xcd6
  (setup_ok, lanes) =
     if self.v4_serpen.is_some() /*+0x3e4 != 2*/ {                       // ★0.6.0 세르펜 씬 활성 → camp 는 Serpen 이어야
        (camp == Serpen(5) && self.v4_serpen.phase(+0x3e0) == 1 /*Setup*/, [Bottom(2), Mid(1)])
     } else if self.v5_morgard.is_some() /*+0x404 != 2*/ {              // ★0.6.0 모가드 씬 활성 → camp 는 Morgard 이어야
        (camp == Morgard(4) && self.v5_morgard.phase(+0x400) == 1, [Top(0), Mid(1)])
     } else {                                                            // 둘 다 None → 레거시 = 구 take_setup_like 그대로(오프셋만)
        match camp { Serpen(5)  => (self.objective(+0xcd5) == 1 /*Serpen*/  && self.+0xcd6 == 1, [Bottom, Mid]),
                     Morgard(4) => (self.objective(+0xcd5) == 0 /*Morgard*/ && self.+0xcd6 == 1, [Top, Mid]),
                     _ => (false, _) } }
  if !setup_ok { return false }
  // L95: v24_objective_setup_relevant_lanes_ready(player, data, camp) (인라인)
  //   lanes 표: anon 3a8525b = \00\01 (Morgard) / 3a8525d = \02\01 (Serpen)   // ★0.6.0 주소(구 33e0b8b/33e0b8d)
  //   ready = v23_objective_setup_pressure_line(player, data, &lanes, 2).is_none()   (반환 i8 == -1) · 콜리 v3: f892d0
  if !ready { return false }
  // L99~106: 캠프 쪽 절반에 있는 건강한 아군 수
  side(c) = camp==Morgard ? is_top_side(ctx,c.x,c.y) [= height - y < x] : is_bottom_side [= height - y > x]   (map_regions.rs:24 / :30)
  healthy_side(c) = c.hp*100/c.stat_cached.hp > 39 && side(c) && is_near_mid_line(ctx, c.x, c.y)   // IR 실체(양 버전 동일): hp%≥40 && !(x>192000 && (h−y)<x && (h−y)<w−192000 [Morgard 형] || 대칭) || |x−(h−y)| < 64000 · height +0x12c0 · width +0x12b8
  healthy_side_allies = cache.iter_champions(team).filter(healthy_side).count()      // L100~101, [team] 5칸
  N = player_count(ctx) 인라인 → 표 3a863e0[tutorial(+0x38)] (TopSolo/MidSolo/JungleOnly: 1, 그 외: 2)   // ★0.6.0 표 주소(구 33e16e8)
  if healthy_side_allies < N { return false }     // L106 (samesign ult)
  // L110~111
  camp_pos = ctx.map.camp_pos(camp, team == 0)   // 1416a7af0
  my_idx = player.info.position 태그(i32, +0xa90)
  if cache.player_champion[team][my_idx].is_none() { return false }
  // L123~132: checker 선정 — 같은 술어를 만족하는 아군 중 (캠프 거리 + 역할 가중) 최소
  role_bias(i) = 표 3a863b8[i] = { Top 60000, Jungle 0, Mid 40000, Bottom 60000, Support 20000 }   // ★0.6.0 표 주소(구 33e16c0)
  checker_position = iter_champions(team).enumerate().filter(|(_,c)| healthy_side(c))   // L117~120 closure#2 · 콜리 v3: dc3de0(구 e3b570 · 동일)
                      .min_by_key(|(i,c)| distance(c.x,c.y, camp_pos)/*141660a00*/.saturating_add(role_bias(i)))   // L124~130 closure#3 · 콜리 v3: db8d70(구 e2fa70 · 동일)
                      .map(|(i,_)| i)
  if checker_position != Some(my_idx) { return false }   // L132 (None 도 false)
  // L137~142
  target_object = (camp != Morgard)     // Morgard→false(0), Serpen→true(1) 로 5번째 인자
  if is_object_being_taken_by_enemy(player, data, goal_data, self, target_object) { return true }   // 콜리 v3: f88690
  // L146~155
  tick = game.tick()   (vt +0x28)
  camp_last_visible_tick = camp==Morgard ? self.obj_spawn.epic_camp_last_visible_tick(+0xa0) : self.obj_spawn.serpen_camp_last_visible_tick(+0xa8)   // ★0.6.0 오프셋(구 +0x80/+0x88)
  camp_recently_checked = camp_last_visible_tick + tps*2 >= tick
  camp_visible = game.is_visible_cell(team, camp_pos.x/32000, camp_pos.y/32000)   (vt +0x100)
  if !(camp_recently_checked && camp_visible) { return true }
  // L160~173 (콜리 v3: ef36b0 · 구 dcc100): 숨은 건강한 적이 마지막 관측 이후 캠프까지 이동할 수 있었나
  return (0..5).filter_map(|p| cache.player_champion[enemy][p].map(|c| (p,c))).any(|(p,c)| {
     c.hp*100/c.stat_cached.hp >= 50                                   // L162 (IR: <50 이면 skip)
     && !data.blackboard[enemy].is_recent_visible(game, player, c)       // L165 · bb last_seen +0x3e8(구 +0x1e0)
     && { last_pos = self.vision.last_visible_pos[p] (+0x520+p*0x10);     // L169 · ★0.6.0 오프셋(구 +0x238)
          d = distance(last_pos, camp_pos).saturating_sub(150000);        // L170
          can_move = tick.saturating_sub(self.vision.last_checked_ticks[p] (+0x5c0+p*8)) * c.stat_cached.move_speed;  // L171~172 · ★0.6.0 오프셋(구 +0x2c8)
          can_move >= d }                                                 // L173 (IR: can_move < d 이면 skip)
  })
```

## changes
- L91 게이트 재작성: `v4_serpen.is_some()(+0x3e4≠2) → camp==5 && +0x3e0==1 → lanes [2,1]` / `v5_morgard.is_some()(+0x404≠2) → camp==4 && +0x400==1 → lanes [0,1]` / 둘 다 None → 레거시 `+0xcd5==(camp==5?1:0) && +0xcd6==1`.
- 본문 동일. 오프셋: PS +0x930/+0x9c0→+0xa00/+0xa90 · camp last-seen +0x80/+0x88→+0xa0/+0xa8 · vision last_visible_pos +0x238→+0x520 · last_checked +0x2c8→+0x5c0 · bb last_seen +0x1e0→+0x3e8 · 표 33e0b8b/8d→3a8525b/5d · 33e16e8→3a863e0 · 33e16c0→3a863b8.
- 콜리 v3: pressure_line →f892d0 · being_taken →f88690 · checker 클로저 →dc3de0/db8d70 · 꼬리 any →ef36b0 · dist →141660a00.

## verified
- Ghidra 0.6.0 f0cfe0 디컴 전문: L91 분기 트리(+0x3e4==2 → {+0x404==2 → camp5:cd5==1/camp4:cd5==0 → cd6==1 ; else camp4 && +0x400==1} / +0x3e4≠2 → camp5 && +0x3e0==1) · lanes 포인터 3a8525b/3a8525d · f892d0(…,2) == -1 · 5칸 hp%≥40 + 192000/64000 사이드식 · 표 3a863e0 · camp_pos 1416a7af0 · dc3de0/db8d70 · 141660a00 + 3a863b8[i] · f88690(…,1/0) · +0xa8/+0xa0 · tps*2 · vt 0x100 · ef36b0.
- 바이트 덤프: 3a8525b = 00 01 / 3a8525d = 02 01 / 3a863b8[0] = 0xea60(60000) / 3a863e0 = [2,2,…].
- Ghidra 0.5.8 dd5db0 디컴으로 사이드식(192000/64000)·N 표·바이어스 표가 구와 동일함을 대조(0.5.8 logic 의 `side && is_near_mid_line` 는 그 IR 실체를 추상화한 표기).
- 미확인: ef36b0 꼬리(0.5.8 aux m09.ll 근거 유지) · f892d0/f88690 내부.

## confidence
A — 변경 1건(L91) 디컴 확인 · 본문 동치 양 버전 디컴 대조.