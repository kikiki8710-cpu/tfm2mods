# dfe2a0→ed5720 FightSituation::build

## logic_060
```
// fight_model.rs:129~238. 판정 없이 FightSituation 을 '조립'하는 함수. prof 타이머(phase 49) 는 prof::ENABLED(DAT_145029209) 일 때만.
// ★0.6.0 sret = 96B(구 128B): +0x00..+0x20 = a7..a11 · +0x28 = a13 · +0x30 focused.id · +0x38 = a20 · +0x40 my_hp_ratio · +0x48 ally_dps_sum · +0x50 enemy_dps_sum · +0x58 my_battle_role(u8) · +0x59..+0x5e = a14..a19(bool 6)
// ★0.6.0 PlayerState team +0xa00 · position +0xa90 · Box<dyn> +0x580/+0x588(구 +0x510/+0x518) / Blackboard stride 0x5c8 · last_seen +0x3e8+p*8 / TeamPlan 카운터 +0x710/+0x718(구 +0x3f8/+0x400)
t = player.info.team (0xa00); pos = player.info.position 태그 (0xa90)
champ = cache.player_champion[t][pos].unwrap()                         // :140 (None → unwrap_failed 패닉)
my_battle_role = get_battle_role(version, ctx, cache, player)           // :143 · Box<dyn> vt+0x20/+0x28/+0x30 인라인 · switch 동일(0x4b0/0x3b6 임계 · 태그 0..5)
my_hp_ratio = champ.hp(+0x670)*100 / max(champ.stat_cached.hp(+0x628), 1)   // :144
// ★0.6.0 삭제: :145 my_skills_ready(skill/skill2 cooldown < 31)
// ★0.6.0 삭제: :146 my_ult_ready(can_ult · 129d130 호출)
// ★0.6.0 삭제: :149 my_dist_to_focused
// ★0.6.0 삭제: :150~163 has_frontline_ally(역할<2 && 더 가까운 아군) 루프 통째
// ★0.6.0 삭제: :165~191 endangered_carry(check_kill_die_tick 루프) 통째

// :194~216 ally_dps_sum — ★자기 자신 제외 없음(p==pos 도 포함, dist 0)
ally_dps_sum = 0
for p in 0..5 { ally = player_champion[t][p]?;
   if dist_sq(ally, champ) >= 14400000001 continue;                    // :200  (≤120000 만) · 0x35a4e9001
   nearest_enemy_champ = player_champion[1-t].iter().flatten()          // :203~206 (첫 후보는 본체 인라인, 나머지는 dbc5d0 폴드 · 구 e32df0)
        .filter(|e| !is_ignored_well_enemy(version, player, e) && blackboard[1-t].is_recent_visible(game, player, e))   // closure$5 :204~205 · 우물 사각형 인라인(64000/160000/800000/0xdac00) · is_recent_visible 인라인 = is_visible(vt 0xf8)(t,id) || get_player(vt 0x150)(id).map_or(false,|p| bb[1-t].last_seen(+0x3e8+p.pos*8)+120 >= tick(vt 0x28))
        .min_by_key(|e| dist_sq(e, ally))                              // closure$6 :206 (동점이면 앞쪽 유지)
   if let Some(ne) = nearest_enemy_champ {
      team_plan.v54_fs_pairings(+0x710) += 1 (atomic)                  // :207 계측 · ★0.6.0 오프셋(구 +0x3f8)
      if !blackboard[1-t].is_recent_visible(game, player, ne) { team_plan.v54_fs_unseen_picks(+0x718) += 1 }   // :209 계측(필터를 통과했으므로 사실상 0) · ★0.6.0 오프셋(구 +0x400)
      ally_dps_sum += fight_dps(version, ctx, attacker=ally, enemy=ne)   // :211 · 콜리 v3: ee0800(구 ebe220 · 본문 동일)
   }
}

// :218~223 enemy_dps_sum
enemy_dps_sum = 0
for enemy in cache.iter_champions(player_champion[1-t]) {
   if dist_sq(enemy, champ) >= 22500000001 continue;                  // :219 (≤150000 만)
   if is_ignored_well_enemy(version, player, enemy) continue;          // :220 인라인(TeamType 0 + is_enemy_well_danger)
   if !blackboard[1-t].is_recent_visible(game, player, enemy) continue; // :221
   enemy_dps_sum += fight_dps(version, ctx, attacker=enemy, enemy=champ)   // :222 · 콜리 v3: ee0800
}
// ★0.6.0 삭제: :226 team_dps_advantage(net_dps) = ally_dps_sum - enemy_dps_sum

write FightSituation { 패스스루 인자 11개(a7..a11 @+0x00.. · a13 @+0x28 · a20 @+0x38 · bool a14..a19 @+0x59..+0x5e), focused_id=focused.id(+0x5c0) @+0x30, my_hp_ratio @+0x40, ally_dps_sum @+0x48, enemy_dps_sum @+0x50, my_battle_role @+0x58 }   // :228 · ★0.6.0 96B
// prof 타이머 drop(:238): PHASE_NANOS[49](145029398) += 경과ns, PHASE_CALLS[49](1450297b8) += 1 (ENABLED 일 때만)
```

## changes
- 구조체 128B → 96B: `endangered_carry`(+0/+8) · `net_dps`(+0x58) · `my_skills_ready`(+0x76) · `my_ult_ready`(+0x77) · `has_frontline_ally`(+0x78) 삭제 → 해당 계산(:145 · :146 · :149 · :150~163 · :165~191 · :226) 전부 소멸(check_kill_die_tick 호출 루프 없음).
- 유지: role · hp_ratio · ally_dps_sum · enemy_dps_sum 식 동일.
- 오프셋/콜리: PS +0x930/+0x9c0→+0xa00/+0xa90 · Box<dyn> +0x510→+0x580 · bb stride 0x5c8 · last_seen +0x3e8 · 카운터 +0x3f8/+0x400→+0x710/+0x718 · min_by e32df0→dbc5d0 · fight_dps ebe220→ee0800.
- version 분기 없음(v3 무관).

## verified
- Ghidra 0.6.0 ed5720 디컴 전문: 인자 21개(sret+20) · role switch(0x4b0/0x3b6 · 태그 2/3/4/5/0) · hp*100/max(max_hp,1) · 아군 루프 0x35a4e9001 · 적 필터(팀 태그 0 && idx==enemy → 우물 사각형) · is_recent_visible 인라인(vt 0xf8/0x150/0x28 · +0x3e8 · +0x78=120) · dbc5d0 fold · LOCK inc +0x710/+0x718 · ee0800 · 적 루프 22500000000 · 96B 기록(+0x30 focused.id · +0x40/+0x48/+0x50 · +0x58 role · +0x59..0x5e) · prof 카운터.
- 미확인: ee0800(fight_dps) 내부(배치 E 「본문 동일」 채택) · role 계산의 Box<dyn> vt 슬롯 의미(구와 동일 패턴).

## confidence
A — 삭제 5필드·유지 4항 전부 0.6.0 디컴으로 확인.