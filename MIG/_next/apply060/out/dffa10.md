# dffa10→e80960 buff_value_v54

## logic_060
```
fn buff_value_v54(version, buff, recv, data, player, parameter, crisis, incoming, epic_incoming, champ_incoming, on_attack_damage, recv_hp_value) -> i64  [buff_value.rs:85 · 0.6.0 e80960]
// ★0.6.0 시그니처: 선두 `version`(rcx · `<3` 비교) + 10번째 `champ_incoming: i64`([rsp+0x258]) 2인자 신설. 스택 인자: player 0x230 · parameter 0x238 · crisis 0x240 · incoming 0x248 · epic_incoming 0x250 · champ_incoming 0x258 · on_attack_damage 0x260 · recv_hp_value 0x268 (rsp = 진입 후 0x1c8 sub 기준).
// ★v2 사장: version<3 분기(§9 구 crisis 항) 는 기술하지 않음.

§1 준비 (L88~101)
L88:  tps = data.context.setting.tick_per_second as i64      // ctx+8 → +0x12f8
L90:  dur_sec = if buff.duration is Time(tick) /*+0x48 u32 == 1 · tick @+0x50*/ { max(tick as i64 / tps, 1) } else { 6 }   // tps==0 → div_by_zero 패닉 · i64::MIN/-1 → overflow 패닉
L94:  window = min(dur_sec, 6)    // dur_sec 은 이후 미사용 · ★0.6.0 §9 에서 window 재사용
L97:  (a_ps, s_ps, u_ps) = if let Some(p) = data.cache.player_by_champion_id(recv.id /*+0x5c0*/) {   // 콜리 d72be0 ★0.6.0 RVA
        c = cache + 0x410 + p.info.team(+0xa00 ★0.6.0)*0x1090 + p.info.position(+0xa90 ★0.6.0)*0x350   // ★0.6.0 PlayerChampionCache 팀 stride 0xfa0→0x1090 · 포지션 0x320→0x350 (team bounds<2)
L99~101: (sum(c.attack_per_sec[0..5] /*@+0x410..0x430*/)/5, (sum(c.skill_per_sec /*@+0x438..*/)+sum(c.skill2_per_sec /*@+0x460..*/))/5, sum(c.ult_per_sec /*@+0x488..0x4a8*/)/5)
      } else { (0,0,0) }
L105: stat = recv.get_stat()   // = stat_cached 복사 (attack, magic_power, hp(+0x628), defence, magic_resistance 사용)
L108: (e_hp,e_def,e_mr,e_n) = (0,0,0,0)
L109: for e in data.cache.iter_champions(1 - player.info.team) { e_hp += e.stat_cached.hp; e_def += …defence; e_mr += …magic_resistance; e_n += 1 }   // L110~114 적팀 5명 · cache+0x1e0+enemy*0x28
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
L162:   atk_range = max(recv.attack_effect.as_ref().map(|e| e.range(recv)).unwrap_or(1), 1)   // Effect::range = range(+0x4a0) + (level(+0x5c8)-1)*growth_range(+0x4a8) + recv.stat_buff_cached.range(+0x438) · attack_effect 태그 +0x4c0(-1=None)
L163:   delta_dps += a_ps * buff.range / atk_range }   // 오버플로 검사 있음(i64 sdiv)
L165: if buff.radius_mult > 0        { L167: delta_dps += s_ps * radius_mult / 200 }

§3 offense_total — 창 동안의 추가 피해 (L170~200)
L170: offense_total = delta_dps * window
L172: if on_attack_damage > 0 { L173: hits = max(window*tps / max(recv.attack_cooltime(),1), 1); L174: offense_total += hits * on_attack_damage }
L177: if e_n > 0 && buff.base_attack_enemy_max_hp_damage != 0 { L178: hits = (같은 식); L179: offense_total += hits * (val * e_hp / 100) }
L181: if e_n > 0 && buff.skill_enemy_max_hp_damage != 0 { L182: offense_total += val * e_hp / 100 }
L184: if buff.self_max_hp_damage != 0 { L185: hits = (같은 식); L186: offense_total += hits * (val * stat.hp / 100) }
L188: if buff.heal_reduce /*+0xc0*/ != 0 {
L190~193: e_heal = Σ_{pos 0..5} (sum(cache[적팀][pos].skill_heal_sec[0..5]) + sum(…skill2_heal_sec[0..5])) / 5   // ult_heal_sec 미포함 · ★0.6.0 배열 위치 cache+0x4e0+enemy*0x1090+pos*0x350 (10 qword 연속 · 구 +0x4b0 = qword +0x96 → +0x9c)
L195:   offense_total += (e_heal * heal_reduce / 100) * window }
L197: if buff.damaged_amplify != 0 {
L199:   near_allies = iter_champions(player.info.team).filter(|e| dist²(e,recv) ≤ 120000²).count()   // 자신 포함
L200:   offense_total += ((a_ps+s_ps+u_ps) * damaged_amplify / 100) * min(near_allies,3) * window }

§4 앵커 적으로 환산 (L204~245)
L204: score = 0
if offense_total > 0 {
L219:   engage_of = |c| c.attack_effect.map(|e| e.range(c)).unwrap_or(0) + c.radius() + c.stat_cached.move_speed*120   // L220~221; radius() = radius_mult(+0x470)==0 ? radius(+0x680) : radius*(mult+100)/100
L223:   recv_engage_range = engage_of(recv)
L224:   caster = cache.player_champion[player.info.team][player.info.position].unwrap()   // None 이면 패닉(unwrap_failed, Location 224)
L225:   caster_engage_range = engage_of(caster)
L226~235: anchor: Option<(ev, ehp)> = parameter.near_enemies /*Vec ptr +0x14d8 · len +0x14f0 ★0.6.0 · 원소 0xd8 · id @+0x58*/.iter()
            .filter(|ep| { let Some(e)=game.get_entity_by_id(ep.id) else {return false};   // L227 (vt+0x1f0)
                           r = recv_engage_range + e.radius(); rc = caster_engage_range + e.radius();   // L228~229
                           dist²(recv,e) ≤ r² || dist²(caster,e) ≤ rc² })                     // L230 (IR 순서: recv 먼저)
            .map(|ep| (champion_hp_value(data, parameter, ep), game.get_entity_by_id(ep.id).map(|e| e.hp).unwrap_or(0)))   // L232~233 · 콜리 champion_hp_value de3d60 ★0.6.0 RVA(구 d37680)
            .filter(|(_, ehp)| *ehp > 0)                                                        // L234
            .max_by_key(|(ev, _)| *ev)                                                          // L235 (동점이면 뒤 원소)
L237:   if let Some((ev, ehp)) = anchor { score = offense_total * ev / max(ehp,1) }
L238:   else if epic_incoming > 0        { score = offense_total * recv_hp_value / max(stat.hp,1) }   // 앵커 없으면 에픽전투 중일 때만 자기 HP 가치
L242:   if buff.attack_speed_mult > 0 && data.context.debug(ctx+0x3b) { L243: print!("\rBUFFV54 tick={} recv={} aps={} mult={} off={} ne={} engage={} anchor={:?}\n", …) }   // 관측 전용
}

§5 쿨감 (L251~264)
L251: if buff.skill_cooldown_mult /*+0x400*/ > 0 || buff.ult_cooldown_mult > 0 {
L252~254: covered = iter_champions(player.info.team).filter(|e| dist²(e,recv) ≤ 60000²).count()
L255:   if covered > 1 {   // 자신 외 아군이 1명 이상 붙어 있을 때만
L257:     cd_gain = if scm > 0 { (s_ps+u_ps) * scm / (scm+100) } else 0            // L259
L261:     if ucm > 0 { cd_gain += u_ps * ucm / (ucm+100) }                          // L262
L264:     score += window * recv_hp_value * cd_gain / max(recv.hp /*+0x670*/,1) } }

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
L321~322: act = cache.player_by_champion_id(recv.id).and_then(|p| data.blackboard[p.info.team].small_actions[p.info.position])   // ★0.6.0 bb stride 0x5c8 · 레코드 +0x1d0+pos*8 (구 +0x78) · team bounds<2
L327:   mobility_realized = match act { Some(Trace(_)) => L328 !parameter.near_enemies.is_empty() /*+0x14f0 != 0*/, Some(RunAway) => incoming > 0, _ => false }   // 태그 재번호 ★0.6.0(SmallActionPlay 3+idx · AroundHide 제거)
L332:   if mobility_realized { L333: ms = move_speed_mult + (ignore_wall ? 10 : 0); L334: score += ms * recv_hp_value / 100 } }

§9 위기 (L339~355) — crisis: Option<&DefensiveCrisis{die_imminent @+0, cc_threat @+1}> · ★0.6.0 v3 전면 재작성
L339: if buff.undying /*+0x118*/ {
        let lethal_now = crisis.is_some_and(|c| champ_incoming > 0 && c.die_imminent);     // ★0.6.0
        if !lethal_now {                                                                    // ★0.6.0: 치명 상황이 아니면 '에픽 전투 가치' 로만 정당화, 못 하면 함수 전체 0
          if epic_incoming <= 0 { return 0; }                                               // ★0.6.0 전체 0 반환(클램프 없이 즉시)
          if window * epic_incoming < max(recv.hp,1) { return 0; }                          // ★0.6.0 창 동안 예상 에픽 피격이 현재 HP 미만이면 0
        }
        score += recv_hp_value;                                                             // (구: c.die_imminent 일 때만 가산 → v3 는 위 게이트 통과 시 무조건 가산)
      }
L344: if let Some(c) = crisis && buff.cc_immune /*+0xf8*/ && c.cc_threat { score += if c.die_imminent { recv_hp_value } else if champ_incoming > 0 { recv_hp_value / 3 } else { 0 } }   // ★0.6.0 `/3` 가산에 champ_incoming>0 조건(구: 무조건)
L353: if let Some(c) = crisis && buff.toughness /*+0xb8*/ != 0 && c.cc_threat && champ_incoming > 0 { score += toughness * recv_hp_value / 200 }   // ★0.6.0 champ_incoming>0 조건 추가

L360: return clamp(score, 0, 160)   // smax 0 → min 160
```

## changes
- 시그니처: `(buff, recv, data, player, parameter, crisis, incoming, epic_incoming, on_attack_damage, recv_hp_value)` → `(version, buff, recv, data, player, parameter, crisis, incoming, epic_incoming, champ_incoming, on_attack_damage, recv_hp_value)`.
- §9 undying: `crisis.die_imminent ? +hp_value : 0` → `!(crisis && champ_incoming>0 && die_imminent)` 이면 `epic_incoming<=0 → return 0` · `window*epic_incoming < max(recv.hp,1) → return 0` · 통과/치명이면 `+hp_value`.
- §9 cc_immune: `/3` 가산에 `champ_incoming > 0` 조건 · toughness 항에 `champ_incoming > 0` 조건.
- PlayerChampionCache: 팀 stride 0xfa0 → 0x1090 · 포지션 0x320 → 0x350 · per-sec 배열 위치 불변(cache+0x410..) · heal 배열 +0x4b0 → +0x4e0.
- Blackboard small_actions 레코드 +0x78 → +0x1d0 · stride 0x2e8 → 0x5c8 · ScoreParameter near_enemies +0x14d8/+0x14f0.
- player.team/pos +0x930/+0x9c0 → +0xa00/+0xa90 · champion_hp_value d37680 → de3d60 · player_by_champion_id → d72be0.
- v2 사장: §9 의 version<3 구식 분기 제거.

## verified
- capstone 0.6.0 e80960 디스어셈: §1 duration 태그(+0x48==1, /tps, max 1, min 6) · d72be0 → +0xa00/+0xa90 → 0x1090/0x350 stride · 0x410/0x438..0x480/0x488 합산 /5 · §3 heal 배열 +0x4e0..+0x528(10 qword)/5 · §4 near_enemies +0x14d8/+0x14f0 · epic_incoming(0x250)>0 · §8 bb 0x5c8·+0x1d0 · incoming(0x248)>0 · §9 전 분기(+0x118 → version(0x50)>2 → crisis(0x240)/champ_incoming(0x258)/die_imminent → epic_incoming → window(r12=[rsp+0x30])*epic_incoming vs max(recv.hp(+0x670),1) → xor eax 즉시 ret · +0xf8 cc_immune · crisis+1 · /3 조건 `version<3 ‖ champ_incoming>0` · +0xb8 toughness 조건 `!(version>=3 && champ_incoming<=0)` · /200 · clamp 0..160).
- 미확인: §2·§3·§5~§7 산식은 RE 「§1~§4 산식 전부 동일」 및 상수 일치에 의존(명령 단위 전수 대조 안 함) · BuffState 필드 오프셋은 undying/cc_immune/toughness/heal_reduce/skill_cooldown_mult 외 미표기(0.5.8 명세 mem 표 기준).

## confidence
A(§9)/B(전체) — 변경 항목은 asm 수준 확정. 본문 나머지는 동치 판정 인용.