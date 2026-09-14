---

### `133` buff_value_v54 — BuffState 한 개를 받는 recv 에게 얼마나 가치 있나 — 공격(DPS 증가→앵커 적 HP가치 환산)·쿨감·힐·피해경감·기동·위기(undying/cc_immune/toughness) 항을 합산해 0..160

| 항목 | 값 |
|---|---|
| id | `buff_value__buff_value_v54` |
| 심볼 | `_RNvNtCshdEBA0ozCnw_7game_ai10buff_value14buff_value_v54` |
| 소스 | `game-ai\src\buff_value.rs:85` |
| IR | `m10.ll` 30000~33060행 |
| 경로·가시성 | `game_ai::buff_value::buff_value_v54` · **in:game_ai** |
| 계층 | 점수화·술어 |
| exe | `dffa10` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r13` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_core::BuffState, &game_core::Entity, &game_core::OperationData, &game_core::PlayerState, &game_ai::ScoreParameter, std::option::Option<&game_ai::DefensiveCrisis>, i64, i64, i64, i64) -> i64
```

<details><summary>인자 10개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | buff | &BuffState(288B) | noalias readonly captures(address,read_provenance). name(+0x0) 을 제외한 전 필드(36개) 읽음 | 4 |
| 1 | 2 | recv | &Entity(1728B) | noalias readonly captures(address,read_provenance). 버프 수신자(=DILocalVariable 'caster' 별칭도 같은 %1, m10.ll:30023). id·stat_cached·hp·x/y·attack_effect·level·stat_buff_cached.range/radius_mult·radius 읽음, attack_cooltime() 호출 | 4 |
| 2 | 3 | data | &OperationData(24B) | noalias readonly captures(address,read_provenance). cache/context/blackboard 전부 사용 | 4 |
| 3 | 4 | player | &PlayerState(2528B) | noalias readonly captures(none). info.team(+0x930)·info.position(+0x9c0) 만 — 아군/적 순회 팀·시전자(caster) 조회 | 4 |
| 4 | 5 | parameter | &ScoreParameter(5384B) | noalias readonly captures(address,read_provenance). near_enemies(+0x14d8 ptr/+0x14f0 len) 만 읽음 + champion_hp_value 인자 | 4 |
| 5 | 6 | crisis | Option<&DefensiveCrisis(2B)> | noalias readonly captures(address_is_null) dereferenceable_or_null(2) — null=None. die_imminent(+0x0)·cc_threat(+0x1) 읽음 (L339~355) | 4 |
| 6 | 7 | incoming | i64 | 예상 피격량. L277 힐 실현 상한, L284 게이트(>0), L291~311 경감 계산, L332 RunAway 기동 실현 판정 | 4 |
| 7 | 8 | epic_incoming | i64 | L238 에서만 — 앵커 적 없을 때 >0 이면 자기 HP 가치로 공격항 환산 | 4 |
| 8 | 9 | on_attack_damage | i64 | L172~174 — >0 이면 평타 횟수×값을 offense_total 에 가산 | 4 |
| 9 | 10 | recv_hp_value | i64 | recv HP 1 당 가치 — 쿨감·힐·경감·기동·위기 항 전부 이 값으로 환산 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
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

**`mem` 메모리 접근 80건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | OperationData | 0x8 | context | r | m10.ll:30054~30055 · tps·debug | 4 | OK |
| 1 | GameContext | 0x8 | setting | r | m10.ll:30056~30057 | 4 | OK |
| 2 | GameSetting | 0x12f8 | tick_per_second | r | m10.ll:30058~30059 · tps(i64). 0 이면 div_by_zero 패닉(30080, Time 버프일 때만) | 4 | OK |
| 3 | BuffState | 0x48 | duration@tag | r | m10.ll:30061~30064 · BuffType 태그 i32: 1=Time (tcxdict --enum BuffType: Permanent 0/Time 1/WithShield 2) | 3 | OK |
| 4 | BuffState | 0x50 | duration@Time.tick | r | m10.ll:30067~30069 · Time 일 때만 | 4 | OK |
| 5 | OperationData | 0x0 | cache | r | m10.ll:30102 | 4 | OK |
| 6 | Entity | 0x5c0 | id | r | recv.id(30106~30107 L97·33044 L321) · 아군 순회 e 의 id 는 안 읽음 | 4 | OK |
| 7 | PlayerState | 0x930 | info.team | r | p(30114~30117 L98, bounds<2) · player(30450~30451 L109 → 1-team 적팀; 31586·32310 아군팀) · L322 p | 4 | OK |
| 8 | PlayerState | 0x9c0 | info.position@tag | r | p(30125~30127 L98 캐시 인덱스) · player(32020~32022 L224 시전자) · L322 p | 4 | OK |
| 9 | AbstractGameWithCache | 0x280 | player_champion_cache[team][pos] | r | m10.ll:30128~30130(L98 recv 캐시) · 30915~30916(L192 적팀 5포지션 순회) | 4 | OK |
| 10 | ChampionCache | 0x190 | attack_per_sec[0..5] | r | m10.ll:30132~30191 · a_ps = 합/5 (L99) | 4 | OK |
| 11 | ChampionCache | 0x1b8 | skill_per_sec[0..5] | r | m10.ll:30198~30257 · s_ps 일부 (L100) | 4 | OK |
| 12 | ChampionCache | 0x1e0 | skill2_per_sec[0..5] | r | m10.ll:30264~30323 · s_ps = (skill합+skill2합)/5 (L100) | 4 | OK |
| 13 | ChampionCache | 0x208 | ult_per_sec[0..5] | r | m10.ll:30330~30392 · u_ps = 합/5 (L101) | 4 | OK |
| 14 | ChampionCache | 0x230 | skill_heal_sec[0..5] | r | m10.ll:30921~30980(pos0)… · L193 적 힐 추정 | 4 | OK |
| 15 | ChampionCache | 0x258 | skill2_heal_sec[0..5] | r | m10.ll:30987~31034(pos0)… · L193 (ult_heal_sec 은 안 읽음) | 4 | OK |
| 16 | Entity | 0x618 | stat_cached.attack | r | m10.ll:30427~30428 · stat = recv.get_stat()(entity.rs:789 인라인) 의 attack (L105/L128) | 4 | OK |
| 17 | Entity | 0x620 | stat_cached.magic_power | r | m10.ll:30430~30431 · L139 분모 | 4 | OK |
| 18 | Entity | 0x628 | stat_cached.hp | r | recv: 30433~30434(L105 → L186·L239·L277·L310) · 적 e: 30527~30528(L110 e_hp) | 4 | OK |
| 19 | Entity | 0x630 | stat_cached.defence | r | recv 30436~30437(L286 def) · 적 e 30530~30531(L112 e_def) | 4 | OK |
| 20 | Entity | 0x638 | stat_cached.magic_resistance | r | recv 30439~30440(L287 mr) · 적 e 30533~30534(L113 e_mr) | 4 | OK |
| 21 | AbstractGameWithCache | 0x1e0 | player_champion[team][0..5] | r | iter_champions 인라인: 30460~30461(L109 적팀) · 31586~31590(L199 아군) · 32020~32025(L224 시전자 단일) · 32310~32323(L254 아군) | 4 | OK |
| 22 | BuffState | 0x5c | attack_mult | r | m10.ll:30564~30567 L124 | 4 | OK |
| 23 | BuffState | 0x58 | attack | r | m10.ll:30581~30584 L127 | 4 | OK |
| 24 | BuffState | 0x8c | attack_speed_mult | r | m10.ll:30597~30600 L130 · 재사용: L242 디버그 print 게이트(%206)·포맷 인자 | 4 | OK |
| 25 | BuffState | 0x104 | crit_chance | r | m10.ll:30617~30620 L134 | 4 | OK |
| 26 | BuffState | 0x60 | magic_power | r | m10.ll:30634~30637 L138 | 4 | OK |
| 27 | BuffState | 0x64 | magic_power_mult | r | m10.ll:30651~30654 L141 | 4 | OK |
| 28 | BuffState | 0xa8 | defence_penetration | r | m10.ll:30671~30675 L147 | 4 | OK |
| 29 | BuffState | 0xb0 | magic_resistance_penetration | r | m10.ll:30689~30693 L152 | 4 | OK |
| 30 | BuffState | 0xf0 | dot_amplify | r | m10.ll:30718~30721 L156 | 4 | OK |
| 31 | BuffState | 0xc8 | range | r | m10.ll:30746~30749 L160 | 4 | OK |
| 32 | Entity | 0x4c0 | attack_effect@tag | r | m10.ll:30768~30771(L162)·31941~31944(L220 recv)·32040~32043(L220 caster) · Option<Effect> 니치: i32 == -1 이면 None (tcxdict Entity 0x4c0 = Niche 판별자, 위치 casting) | 3 | OK |
| 33 | Entity | 0x4a0 | attack_effect@Some.range | r | m10.ll:30787~30788 · Effect::range(effect.rs:25~26 인라인) = range + (level-1)*growth_range + stat_buff_cached.range | 4 | OK |
| 34 | Entity | 0x4a8 | attack_effect@Some.growth_range | r | m10.ll:30789~30790 | 4 | OK |
| 35 | Entity | 0x5c8 | level | r | m10.ll:30791~30793 · (level-1)*growth | 4 | OK |
| 36 | Entity | 0x438 | stat_buff_cached.range | r | m10.ll:30795~30796 | 4 | OK |
| 37 | BuffState | 0x100 | radius_mult | r | m10.ll:30761~30764 L165 | 4 | OK |
| 38 | BuffState | 0xd0 | base_attack_enemy_max_hp_damage | r | m10.ll:30841~30845 L177 | 4 | OK |
| 39 | BuffState | 0xe0 | skill_enemy_max_hp_damage | r | m10.ll:30866~30870 L181 | 4 | OK |
| 40 | BuffState | 0xd8 | self_max_hp_damage | r | m10.ll:30893~30896 L184 | 4 | OK |
| 41 | BuffState | 0xc0 | heal_reduce | r | m10.ll:30909~30912 L188 | 4 | OK |
| 42 | BuffState | 0xa0 | damaged_amplify | r | m10.ll:31574~31577 L197 | 4 | OK |
| 43 | Entity | 0x660 | x | r | recv 31611~31612(L199)·32344~32345(L254) · 아군 e 31636~31637… · aux m12: recv/caster/e (L230) | 4 | OK |
| 44 | Entity | 0x668 | y | r | recv 31613~31614 · 아군 e 31638~31639 … | 4 | OK |
| 45 | Entity | 0x470 | stat_buff_cached.radius_mult | r | m10.ll:31970~31974(L221 recv)·32069~32073(caster) · aux m12.ll:18653~18657(L228 적 e) · Entity::radius(entity.rs:1509~1515 인라인): mult==0 → radius, 아니면 radius*(mult+100)/100 | 4 | OK |
| 46 | Entity | 0x680 | radius | r | m10.ll:31977·31985 · 32076·32084 · aux m12.ll:18660·18669 | 4 | OK |
| 47 | Entity | 0x640 | stat_cached.move_speed | r | m10.ll:31994~31996(recv)·32093~32095(caster) · engage_of 의 move_speed*120 (L221) | 4 | OK |
| 48 | ScoreParameter | 0x14d8 | near_enemies.buf.ptr | r | m10.ll:32101~32102 L226 · 앵커 후보 슬라이스 | 4 | OK |
| 49 | ScoreParameter | 0x14f0 | near_enemies.len | r | m10.ll:32104~32105(L226 끝 포인터)·32267(L245 print)·32961~32963(L328 is_empty) | 4 | OK |
| 50 | AbstractGameWithCache | 0x0 | game(&dyn AbstractGame).data_ptr | r | m10.ll:32115 L227 | 4 | OK |
| 51 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r | m10.ll:32116~32117 | 4 | OK |
| 52 | vtable(AbstractGame) | 0x1f0 | get_entity_by_id | r | aux m12.ll:18623~18625·18772~18774 · 슬롯 62 fn(&self,usize)->Option<&Entity> (divtable AbstractGame 0x1f0) | 3 | 확인불가(tcx 사전에 타입 없음) |
| 53 | vtable(AbstractGame) | 0x28 | tick | r | m10.ll:32256~32258 L244 · 디버그 print 인자 | 4 | 확인불가(tcx 사전에 타입 없음) |
| 54 | ChampionScoreParameter | 0x58 | id | r | aux m12.ll:18621~18622 · near_enemies 원소의 챔피언 id → get_entity_by_id | 4 | OK |
| 55 | Entity | 0x670 | hp | r | recv: 32686~32687(L264)·32707~32708(L277/279/315) · aux m12 적 e: 18782~18783(L233 ehp) | 4 | OK |
| 56 | GameContext | 0x3b | debug | r | m10.ll:32206~32209 L242 · print 게이트 | 4 | OK |
| 57 | BuffState | 0x90 | skill_cooldown_mult | r | m10.ll:32005~32007 L251/257 | 4 | OK |
| 58 | BuffState | 0xfc | ult_cooldown_mult | r | m10.ll:32008~32010 L251/261 | 4 | OK |
| 59 | BuffState | 0x80 | vamp | r | m10.ll:32317~32320 L271 | 4 | OK |
| 60 | BuffState | 0x74 | hp_regen | r | m10.ll:32699~32704 L274 | 4 | OK |
| 61 | BuffState | 0x68 | defence | r | m10.ll:32763~32765 L288 | 4 | OK |
| 62 | BuffState | 0x6c | defence_mult | r | m10.ll:32766~32768 L288 | 4 | OK |
| 63 | BuffState | 0x78 | magic_resistance | r | m10.ll:32779~32781 L293 | 4 | OK |
| 64 | BuffState | 0x7c | magic_resistance_mult | r | m10.ll:32782~32784 L293 | 4 | OK |
| 65 | BuffState | 0xe8 | damaged_reduce | r | m10.ll:32807~32810 L297 | 4 | OK |
| 66 | BuffState | 0x108 | base_attack_damaged_reduce | r | m10.ll:32828~32831 L300 | 4 | OK |
| 67 | BuffState | 0x110 | skill_damaged_reduce | r | m10.ll:32843~32846 L303 | 4 | OK |
| 68 | BuffState | 0x98 | damage_reflect | r | m10.ll:32859~32862 L306 | 4 | OK |
| 69 | BuffState | 0x70 | hp | r | m10.ll:32875~32877 L310 | 4 | OK |
| 70 | BuffState | 0x84 | hp_mult | r | m10.ll:32878~32880 L310 | 4 | OK |
| 71 | BuffState | 0x88 | move_speed_mult | r | m10.ll:32750~32752 L320/333 | 4 | OK |
| 72 | BuffState | 0x119 | ignore_wall | r | m10.ll:32753~32755 L320/333 | 4 | OK |
| 73 | OperationData | 0x10 | blackboard(&[Blackboard;2]) | r | m10.ll:32913~32914 L322 | 4 | OK |
| 74 | Blackboard | 0x78 | small_actions[pos]@tag | r | m10.ll:32948~32951 · [Option<SmallAction>;5] stride 32, 태그 i64 (Direct: RunAway 0 / Trace 4, tcxdict --enum SmallAction) | 3 | OK |
| 75 | BuffState | 0x118 | undying | r | m10.ll:32923~32926 L339 | 4 | OK |
| 76 | DefensiveCrisis | 0x0 | die_imminent | r | m10.ll:32995~32997(L340)·33018~33020(L346) | 4 | OK |
| 77 | BuffState | 0xf8 | cc_immune | r | m10.ll:32985~32988 L344 | 4 | OK |
| 78 | DefensiveCrisis | 0x1 | cc_threat | r | m10.ll:33012~33015(L346)·33049~33052(L354) | 4 | OK |
| 79 | BuffState | 0xb8 | toughness | r | m10.ll:33004~33007 L353 | 4 | OK |

**`consts` 상수 17건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev | folded_from |
|---|---|---|---|---|---|---|
| 0 | 1 | 90 | 태그 | BuffType 메모리태그 1 = Time (icmp eq i32 %33, 1 — m10.ll:30063). 리터럴, shl 아님 | 4 |  |
| 1 | 6 | 94 | 인덱스 | window = min(dur_sec, 6) — 버프 지속을 최대 6초로 캡(llvm.umin 30089) · Time 이 아닌 버프(Permanent/WithShield)는 dur_sec=6 (phi 30097) | 4 |  |
| 2 | 5 | 99 | 계수 | ChampionCache *_per_sec[5]/_heal_sec[5] 5 슬롯(=상대 포지션) 평균 분모 (udiv 30412·30414·30415·31047…) | 4 |  |
| 3 | 100 | 125 | 계수 | 퍼센트 분모/승수 — attack_mult·attack_speed_mult·crit·magic_power_mult·관통·힐감·증폭·흡혈·경감·hp_mult·이속(L125~334 전반) | 4 |  |
| 4 | -99 | 150 | 임계 | 관통 후 방어/마저 하한 max(def_after,-99) → 분모 (def_after+100) ≥ 1 (llvm.smax 30708·30736) | 4 |  |
| 5 | 200 | 158 | 계수 | dot_amplify*s_ps/200 (L158) · s_ps*radius_mult/200 (L167) · toughness*hp_value/200 (L355) — 반값 계수 | 4 |  |
| 6 | -1 | 162 | 센티널 | Option<Effect> None 니치(attack_effect.casting i32 == -1) — None 이면 L162 atk_range=1, L220 engage 사거리 0 (icmp eq i32 …, -1: 30770·31943·32042) | 4 |  |
| 7 | 14400000001 | 199 | 임계 | 120000²+1 — near_allies: recv 와 제곱거리 < 이 값(=≤120000, 3.75셀)인 아군(자신 포함) 수 (closure#1 buff_value.rs:199, icmp ult 31670…) | 4 |  |
| 8 | 3 | 200 | 임계 | min(near_allies, 3) — 피해증폭(damaged_amplify) 가치에 곱하는 아군 수 상한 (llvm.umin 31921) · L349 cc_threat 만 있을 때 recv_hp_value/3 (sdiv 33023) | 4 |  |
| 9 | 120 | 221 | 계수 | engage_of(c) = attack_range(c) + radius(c) + stat_cached.move_speed*120 — 이속을 교전 사거리로 환산(2초분, mul 31996·32095) | 4 |  |
| 10 | 3600000001 | 253 | 임계 | 60000²+1 — covered: recv 와 제곱거리 < 이 값(≤60000)인 아군(자신 포함) 수 (closure#7 buff_value.rs:253, icmp ult 32403…) | 4 |  |
| 11 | 0 | 116 | 임계 | 게이트 비교값: e_n>0 · 각 버프 필드 >0/!=0 · offense_total>0(L204) · heal_realized>0(L278) · incoming>0(L284) · d_def/d_mr/d_hp>0 · mitigated>0(L314) · 최종 max(score,0)(L360) | 4 |  |
| 12 | 4 | 327 | 태그 | SmallAction 메모리태그 4 = Trace (switch 32953~32956; 0 = RunAway) — mobility_realized 분기 | 4 |  |
| 13 | 10 | 333 | 산출값 | ms = move_speed_mult + (ignore_wall ? 10 : 0) — 벽무시를 이속 10% 상당으로 (select 32973) | 4 |  |
| 14 | 160 | 360 | 임계 | 최종 상한 min(score,160) (llvm.umin 33041) | 4 |  |
| 15 | 1 | 291 | 임계 | incoming/2 — `lshr i64 %6, 1`(32793·32813·32849·32865) 로 접힘. 방어/마저/평타경감/스킬경감 항은 예상 피격의 절반(물리·마법 반반 가정)에만 적용 | 4 | 2 |
| 16 | 1 | 128 | 임계 | 분모 하한 max(stat.attack,1)·max(stat.magic_power,1)·max(atk_range,1)·max(attack_cooltime,1)·max(hits,1)·max(recv.hp,1)·max(def+100+d_def,1)·max(dur_sec,1) (llvm.smax/umax 다수). 리터럴, shl 아님 | 4 |  |

**`knobs` 조정점 12건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 버프 창 상한(초) | buff_value.rs:94 | 6 | 올리면 긴 버프(Permanent 포함, 기본 6)의 공격/쿨감/힐 항이 비례 증가 → 장기 버프 가치 ↑ | 4 | 기존 |
| 1 | 피해증폭 아군 수 상한 | buff_value.rs:200 | 3 | 올리면 5인 뭉침에서 damaged_amplify 가치 ↑ | 4 | 기존 |
| 2 | 피해증폭 아군 거리 | buff_value.rs:199 | 14400000001 | 120000² — 올리면 멀리 있는 아군도 세어 증폭 가치 ↑ | 4 | 기존 |
| 3 | 이속→교전사거리 환산 계수 | buff_value.rs:221 | 120 | 올리면 앵커 적 후보 반경이 넓어져 offense 환산이 되는 경우 ↑(앵커 없음→0 되는 일 감소) | 4 | 기존 |
| 4 | 쿨감 인정 아군 거리 | buff_value.rs:253 | 3600000001 | 60000² — 올리면 흩어진 팀에서도 covered>1 이 되어 쿨감 가치 발생 | 4 | 기존 |
| 5 | 쿨감 인정 최소 아군 수 | buff_value.rs:255 | 1 | covered > 1 — 내리면(>0) 혼자 있어도 쿨감 가치 인정 | 4 | 기존 |
| 6 | 관통 분모 하한 | buff_value.rs:150 | -99 | 올리면(예 -50) 고관통 버프의 delta_dps 상한이 낮아짐 | 4 | 기존 |
| 7 | 물리/마법 반분 계수 | buff_value.rs:291 | 2 | incoming/2 — 내리면(1) 방어/마저/평타·스킬경감 항이 2배(shl 접힘) | 4 | 기존 |
| 8 | 벽무시 이속 환산 | buff_value.rs:333 | 10 | 올리면 ignore_wall 버프 가치 ↑ | 4 | 기존 |
| 9 | cc_threat 만 있을 때 cc_immune 분할 | buff_value.rs:349 | 3 | recv_hp_value/3 — 내리면 die_imminent 아닌 CC 위협에서도 cc_immune 가치 ↑ | 4 | 기존 |
| 10 | toughness 환산 분모 | buff_value.rs:355 | 200 | 내리면 강인함 버프 가치 ↑ | 4 | 기존 |
| 11 | 최종 상한 | buff_value.rs:360 | 160 | 올리면 복합 버프(여러 항 동시)가 더 높은 점수 | 4 | 기존 |

<details><summary>`callees` 피호출자 19건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | attack_cooltime | game_core::Entity::attack_cooltime | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1766 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | buff_value_v54 | game_ai::buff_value::buff_value_v54 | in:game_ai | fn(&game_core::BuffState, &game_core::Entity, &game_core::OperationData, &game_core::PlayerState, &game_ai::ScoreParameter, std::option::Option<&game_ai::DefensiveCrisis>, i64, i64, i64, i64) -> i64 | game-ai\src\buff_value.rs:85 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 2 | champion_hp_value | game_ai::champion_hp_value | pub | fn(&game_core::OperationData, &game_ai::ScoreParameter, &game_ai::ChampionScoreParameter) -> i64 | game-ai\src\utils.rs:909 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 3 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 4 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 5 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 6 | get_stat | <game_ai::WindowStatView<'_> as game_core::AbstractEntity>::get_stat | pub | fn(&game_ai::WindowStatView</#0>) -> game_core::EntityStat | game-ai\src\fight_check.rs:93 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 4개 중 상위 3개 |
| 7 | get_stat | game_core::AbstractEntity::get_stat | pub | fn(&Self/#0) -> game_core::EntityStat | game-core\src\simulation\entity.rs:713 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 4개 중 상위 3개 |
| 8 | get_stat | game_view::DatabaseEditUIRunner::get_stat | in:game_view::ui::database_edit_ui | fn(&game_core::AthleteStat, &str) -> usize | game-view\src\ui\database_edit_ui.rs:5331 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 4개 중 상위 3개 |
| 9 | is_empty | game_core::PatchSetting::is_empty | pub | fn(&game_core::PatchSetting) -> bool | game-core\src\patch_engine.rs:853 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 10 | is_empty | game_core::ModAiRegistry::is_empty | pub | fn(&game_core::ModAiRegistry) -> bool | game-core\src\mod_ai.rs:15 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 11 | is_empty | game_core::PublicChampionEvidence::is_empty | pub | fn(&game_core::PublicChampionEvidence) -> bool | game-core\src\banpick\public_evidence.rs:96 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 12 | iter_champions | game_core::AbstractGameWithCache::<'a, 'b>::iter_champions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5759 ~ game_core[6a30]::simulation::{impl#5}::iter_champions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1904 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 13 | player_by_champion_id | game_core::AbstractGameWithCache::<'a, 'b>::player_by_champion_id | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> std::option::Option<&'a/#0 game_core::PlayerState> | game-core\src\simulation.rs:1915 | True | True | 3 | IR 호출 심볼 일치(토큰 경계) |
| 14 | radius | game_core::Entity::radius | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1509 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 15 | range | game_core::Effect::range | pub | fn(&game_core::Effect, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:25 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 16 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 17 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 18 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
</details>

⚠**미매칭 9개**: `_RNvNtNtCs9ec1k27omRZ_3std2io5stdio6__print`, `clamp`, `engage_of`, `llvm.memcpy.p0.p0.i64`, `llvm.smax.i64`, `llvm.smin.i64`, `llvm.umax.i64`, `llvm.umin.i64`, `try_fold`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 4곳** (m05.ll:43128, m05.ll:43259, m05.ll:43817, m05.ll:43852) · **형제 0개** 

**`open` 8건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | champion_hp_value(utils.rs:909, define m04.ll:49583, TLS 캐시 경유) 의 산식 — 다른 함수. 여기선 i64 반환 계약만(ev = 그 적의 HP 1 당 가치로 사용) | 4 |  |
| 1 | 미탐색 | Entity::attack_cooltime(entity.rs:1766) 내부 — game_core | 4 |  |
| 2 | 표기 불가 | dur_sec/window 의 소스 표기: Time 아닌 경로가 `else 6` 인지 `unwrap_or(6)` 인지 표기 불가(외연 동일: window=6). dur_sec 자체는 window 외 소비 0건 | 4 |  |
| 3 | 미탐색 | L339/344/353 의 `crisis.is_some() && buff.flag` 소스 순서(column 부재) — IR 은 두 조건을 or 로 합쳐 한 번에 분기 | 4 |  |
| 4 | 미탐색 | L230 `dist(recv)≤r \|\| dist(caster)≤rc` 표기 순서 — IR 분기 순서(recv 먼저) 기록 | 4 |  |
| 5 | 미탐색 | L242 디버그 print 게이트가 attack_speed_mult>0 인 이유(개발자 디버깅 잔재로 보이나 근거 없음) — 관측 전용이라 판정 무영향 | 5 |  |
| 6 | 미탐색 | ChampionCache.*_heal_sec[5] 의 인덱스 의미 — per_sec 과 같은 '대상 포지션'으로 추정(자매 함수 banish/mark 가 per_sec 을 그렇게 색인) · 여기선 5원소 합/5 만 쓰므로 판정에 영향 없음 | 5 |  |
| 7 | 미탐색 | Option<Effect> None 니치가 -1 인 이유(rustc 니치 방향)는 IR 관측 사실만 기록 — tcxdict 는 위치(0x4c0, Niche)만 주고 값은 안 준다 | 3 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

