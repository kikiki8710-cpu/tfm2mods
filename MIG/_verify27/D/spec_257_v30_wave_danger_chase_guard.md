---

### `257` v30_wave_danger_chase_guard — 추격 목표(focused)를 향한 정지점(stance)에서 window 틱 동안 맞을 적 미니언 라인 피해를 재고, 피해 0/경미/즉살 가능/웨이브보다 먼저 킬 가능이면 None(추격 계속·wave_obs 코드 기록), 아니면 RunAway / KitingBack(focus) / End 중 하나로 추격을 끊는다

| 항목 | 값 |
|---|---|
| id | `battle__v30_wave_danger_chase_guard` |
| 심볼 | `_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old6battle27v30_wave_danger_chase_guard` |
| 소스 | `game-ai\src\plan_legacy\old\battle.rs:2079` |
| IR | `m10.ll` 52377~52937행 |
| 경로·가시성 | `game_ai::plan_legacy::old::battle::v30_wave_danger_chase_guard` · **in:game_ai** |
| 계층 | 레거시 플랜 |
| exe | `e0dfc0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r18` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &game_core::OperationData, &game_core::Entity, &game_core::Entity, u64, usize, bool, usize, usize, bool, bool, bool, &mut u8, &mut u8, &mut bool) -> std::option::Option<game_ai::plan_legacy::old::BattleSubPlanGoal>
```

**TLS 메모 절(`sig.tls` · 정정 경로 `/specs[257]/sig/tls/<키>`)**

없음 — 본문에 LocalKey/call_once fn-포인터 상수 참조 0 · llvm.threadlocal.address 0. (콜리 minion_wave_risk::* 내부 TLS 여부는 콜리 계약 밖)

<details><summary>인자 15개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | version | usize (i64 %0) | L2094·L2138·L2157·L2173 에서 `version > 1`(icmp ugt 1) 분기. reach(version=2 gamemode=0)는 전부 true 로 접혀 `enemy_minion_wave_risk_damage_at`(L2097)·`enemy_minion_wave_danger_damage_at`(L2141) 호출부가 사장(NA). 콜리 4종의 1번째 인자로도 전달 | 4 |
| 1 | 2 | data | &OperationData(24B) %1 | define 줄 속성: readonly · captures(address, read_provenance). +0x0 cache(&AbstractGameWithCache · L2104 nexus[et]) · +0x8 context(&GameContext · L2087/L2088 setting.tick_per_second · map) | 4 |
| 2 | 3 | champ | &Entity(1728B) %2 | define 줄 속성: readonly. 나(추격자). 읽는 필드 = team@tag(0x0)·team.0(0x8)·x/y(0x660/0x668)·stat_cached.move_speed(0x640)·hp(0x670)·stat_cached.hp(0x628) | 4 |
| 3 | 4 | focused | &Entity(1728B) %3 | define 줄 속성: readonly. 추격 목표. 읽는 필드 = team@tag(0x0)·team.0(0x8)·visible_state[t]@tag(0x38+24t)·x/y·move_speed(0x640)·id(0x5c0 → KitingBack.focus) | 4 |
| 4 | 5 | max_range | u64 (i64 %4) | 내 사거리. 0 이면 stance 계산 불가 → None(L2206 인라인 battle_chase_stance). from_distance = max_range.saturating_sub(15000) (L2225·L2115) | 4 |
| 5 | 6 | focused_die_tick | usize (i64 %5) | 목표가 죽기까지 예상 틱. L2158 `> tps/2` · L2159 `<= my_die_tick+tps/2` · L2180 `focused_die_tick.saturating_add(walk_tick) < min(wave_die_tick, my_die_tick)` | 4 |
| 6 | 7 | focused_is_in_range | bool (i1 %6) | L2157 needs_to_move · L2159 즉살창 · L2178 kill_reachable 생략 · L2194 KitingBack 선택 | 4 |
| 7 | 8 | my_die_tick | usize (i64 %7) | 내가 죽기까지 예상 틱. L2159 saturating_add(tps/2) · L2180 umin | 4 |
| 8 | 9 | target_hp_ratio | usize (i64 %8) | 목표 HP% . L2159 `< 26` 만 | 4 |
| 9 | 10 | can_runaway | bool (i1 %9) | L2192 RunAway 허용 게이트 | 4 |
| 10 | 11 | return_to_objective | bool (i1 %10) | L2196 can_runaway 경로에서 End(7) vs RunAway(4) 선택 | 4 |
| 11 | 12 | open_eval | bool (i1 %11) | L2102 투영 stance(projected_stance) 계산 게이트(true 일 때만) · L2157 needs_to_move 강제 true | 4 |
| 12 | 13 | wave_obs | &mut u8 %12 | define 줄 속성: writeonly · captures(none) · dereferenceable(1) · initializes 속성 없음. 관측 코드 출력(writes 참조). 조기 None 3경로(L2083 같은 팀 · max_range==0 · 비가시)에서는 쓰지 않는다 | 4 |
| 13 | 14 | wave_pct_out | &mut u8 %13 | writeonly · initializes 없음. L2130 에서 1회 store(그 이후 모든 경로). 조기 None 3경로에선 미기록 | 4 |
| 14 | 15 | wave_danger_out | &mut bool %14 | writeonly · initializes 없음. L2147 에서 1회 store(wave_damage≠0 경로에서만). 조기 None 3경로 + wave_damage==0 경로(L2131)에선 미기록 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
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
tps = ctx.setting.tick_per_second                                                          // L2088
window = min(tps*3, max(tps, walk_tick + tps*2))                                           // L2089 (= clamp(walk_tick+2tps, tps, 3tps))
wave_damage = if version > 1 { enemy_minion_line_action_damage_at(version, data, champ, stance_x, stance_y, window, true, true) }   // L2095 (live)
              else { enemy_minion_wave_risk_damage_at(version, data, champ, stance_x, stance_y, window) }                          // L2097 ★사장(reach: version>1 접힘)
projected_stance = None
if version > 1 && open_eval && focused.team is Player(et) {                                // L2102~2103 (Neutral 이면 생략)
  if let Some(nexus) = data.cache.nexus[et] {                                              // L2104 (bounds 2 → panic_bounds_check)
    // closure$0 (L1162<2104): 적이 1초간 자기 넥서스로 도주한 위치 → 거기서 내 쪽으로 (max_range−15000) 떨어진 점 = 투영 stance
    dx = nexus.x − focused.x ; dy = nexus.y − focused.y ; sz = max(isqrt(dx²+dy²), 1)    // L2105~2107
    flee = focused.move_speed * tps                                                        // L2108
    (px,py) = adjust_position(map, setting, focused.x + flee*dx/sz, focused.y + flee*dy/sz) // L2109~2111
    ddx = champ.x − px ; ddy = champ.y − py ; dsz = max(isqrt(ddx²+ddy²), 1)              // L2112~2114
    fd = max_range.saturating_sub(15000)                                                   // L2115
    (sx2,sy2) = adjust_position(map, setting, px + ddx*fd/dsz, py + ddy*fd/dsz)           // L2116~2118
    projected_stance = Some((sx2,sy2))
    wd2 = enemy_minion_line_action_damage_at(version, data, champ, sx2, sy2, window, true, true)   // L2127
    wave_damage = max(wave_damage, wd2)                                                    // L2128
  }
}
wave_pct = min(wave_damage*100 / max(champ.hp,1), 254) ; *wave_pct_out = wave_pct as u8   // L2130 ★첫 기록
if wave_damage == 0 { *wave_obs = 1; return None }                                         // L2131~2132
hp_pct = champ.hp*100 / max(champ.stat_cached.hp, 1)                                       // L2136
danger_damage = if version > 1 { enemy_minion_line_action_danger_damage_at(version, data, champ, stance_x, stance_y, window, true, true) }   // L2139 (live)
                else { enemy_minion_wave_danger_damage_at(version, data, champ, stance_x, stance_y, window) }                          // L2141 ★사장
if let Some((sx2,sy2)) = projected_stance { danger_damage = max(danger_damage, enemy_minion_line_action_danger_damage_at(version, data, champ, sx2, sy2, window, true, true)) }   // L2143~2145
*wave_danger_out = danger_damage != 0                                                      // L2147
// ── 3. 경미하면 통과 ──
if danger_damage == 0                                                                      // L2148
   && !(wave_pct > 11 && hp_pct < 51)                                                      // L2149
   && !((wave_pct > 4 && hp_pct < 36) || hp_pct < 26) {                                    // L2150 (줄 안 순서는 column 0 이라 표기 불가 · 외연 동일)
  *wave_obs = 2; return None                                                               // L2153
}
// ── 4. 즉살 창 ──
needs_to_move = if focused_is_in_range && !(version > 1 && open_eval) { walk_tick >= tps/3 } else { true }   // L2157
immediate_kill_window = if focused_die_tick > tps/2 { focused_is_in_range && target_hp_ratio < 26 && focused_die_tick <= my_die_tick.saturating_add(tps/2) } else { true }   // L2158~2159
if immediate_kill_window && wave_damage < champ.hp && !(wave_pct > 14 && hp_pct < 26) { *wave_obs = 3; return None }   // L2160~2161
// ── 5. 웨이브보다 먼저 킬 ──
if version > 1 && wave_damage < champ.hp {                                                 // L2173
  wave_die_tick = champ.hp.saturating_mul(window) / wave_damage                            // L2174 (umul.with.overflow → -1)
  kill_reachable = focused_is_in_range || champ.move_speed > focused.move_speed            // L2178~2179 (in_range 면 비교 생략)
  if kill_reachable && focused_die_tick.saturating_add(walk_tick) < min(wave_die_tick, my_die_tick) { *wave_obs = 3; return None }   // L2180~2181
}
// ── 6. 발동 여부 ──
if !(hp_pct < 46 || needs_to_move || wave_damage >= champ.hp) { *wave_obs = 4; return None }   // L2186~2187 (추격 유지)
*wave_obs = 5                                                                              // L2190
if can_runaway && (wave_damage >= champ.hp || hp_pct < 46) { return Some(RunAway) }       // L2192 (태그 4)
if focused_is_in_range { return Some(KitingBack{ focus: focused.id }) }                   // L2194~2195 (태그 3 · 페이로드 = focused.id)
if can_runaway { return Some(if return_to_objective { End } else { RunAway }) }           // L2196 (7 / 4)
return Some(End)                                                                           // L2194 else (태그 7, !can_runaway & !in_range)
// 극성 메모: None = '추격 계속'(가드 미발동), Some = 추격을 끊는 대체 목표. wave_obs 1~4 는 None 사유 코드, 5 = 발동. rnd 인자 없음 · gen_range 0.
// 콜리 계약: enemy_minion_line_action_damage_at / _danger_damage_at(version, data, target:&Entity, x, y, window_tick, champion_action:bool=true, predict_retarget:bool=true) -> i64(피해량) (m07.ll:47470 / 48226) · Entity::distance(&self,&other)->u64(g06.ll:84197) · adjust_position(&MapDef,&GameSetting,x,y)->(x,y)(g15.ll:72245)
```

**`mem` 메모리 접근 29건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | Entity(focused %3) | 0x0 | team@tag | r | L2083 TeamType::eq(entity.rs:1127 derive) focused.team == champ.team 이면 None (52399~52410·52531~52534). L2102 `trunc %16` = Neutral(1) 이면 projected_stance 생략 | 4 | OK |  |
| 1 | Entity(focused %3) | 0x8 | team.0 (Player 페이로드) | r | L2083 태그 둘 다 Player 일 때 팀 번호 비교 · L2103 et = focused.player_team → cache.nexus[et] 인덱스(0..2 bounds check) | 4 | OK |  |
| 2 | Entity(champ %2) | 0x0 | team@tag | r | L2083 eq · L2214 is_visible_from(entity.rs:1482) player_team(1136): Neutral(1) 이면 항상 가시로 통과 | 4 | OK |  |
| 3 | Entity(champ %2) | 0x8 | team.0 | r | L2214 t = champ.player_team → focused.visible_state[t] 인덱스(bounds 2) | 4 | OK |  |
| 4 | Entity(focused %3) | 0x38 | visible_state[t]@tag (stride 24, t=champ.team.0) | r | L2214 data.rs:122 is_visible: 태그 0(Visible) 이 아니면 None (52457~52461). VisibleState Direct 태그 0 Visible/1 Invisible/2 Unknown | 4 | OK |  |
| 5 | Entity(champ %2) | 0x660 | x | r | L2212(사거리 안이면 stance=내 위치) · L2218 dx=champ.x−focused.x · L2112 ddx=champ.x−px | 4 | OK |  |
| 6 | Entity(champ %2) | 0x668 | y | r | L2212 · L2219 · L2113 | 4 | OK |  |
| 7 | Entity(focused %3) | 0x660 | x | r | L2218 · L2226 stance x 원점 · L2105 dx=nexus.x−focused.x · L2109 px 원점 | 4 | OK |  |
| 8 | Entity(focused %3) | 0x668 | y | r | L2219 · L2227 · L2106 · L2110 | 4 | OK |  |
| 9 | Entity(champ %2) | 0x640 | stat_cached.move_speed | r | L2230 walk_tick 분모 max(ms,1) · L2179 kill_reachable = champ.ms > focused.ms | 4 | OK |  |
| 10 | Entity(focused %3) | 0x640 | stat_cached.move_speed | r | L2108 flee = focused.ms × tps(1초 도주 투영) · L2179 | 4 | OK |  |
| 11 | Entity(champ %2) | 0x670 | hp | r | L2130 wave_pct 분모 · L2136 hp_pct 분자 · L2160/L2173/L2186/L2192 `wave_damage < hp` · L2174 hp.saturating_mul(window) | 4 | OK |  |
| 12 | Entity(champ %2) | 0x628 | stat_cached.hp (최대 HP) | r | L2136 hp_pct = hp*100/max(stat_cached.hp,1) | 4 | OK |  |
| 13 | Entity(focused %3) | 0x5c0 | id | r | L2195 KitingBack{focus: focused.id} 페이로드 (52927~52928) | 4 | OK |  |
| 14 | OperationData(data %1) | 0x0 | cache | r | &AbstractGameWithCache (L2104, 52594) | 4 | OK |  |
| 15 | OperationData(data %1) | 0x8 | context | r | &GameContext (L2087 52413~52414 · L2088) | 4 | OK |  |
| 16 | GameContext | 0x8 | setting | r | &GameSetting → adjust_position 2번째 인자 · tps (52503·52546~52547) | 4 | OK |  |
| 17 | GameContext | 0x20 | map | r | &MapDef → adjust_position 1번째 인자 (52501~52502 · 52619~52620) | 4 | OK |  |
| 18 | GameSetting | 0x12f8 | tick_per_second | r | L2088 tps (52548~52549 gep 4856) | 4 | OK |  |
| 19 | AbstractGameWithCache | 0x170 | nexus[et] Option<&Entity> (stride 8, null=None) | r | L2104 적 팀 넥서스 (52595~52597). None 이면 projected_stance 생략 | 4 | OK |  |
| 20 | Entity(nexus) | 0x660 | x | r | L1162<2104 closure$0: 적이 자기 넥서스로 도주한다고 가정한 투영 (52623~52624) | 4 | OK |  |
| 21 | Entity(nexus) | 0x668 | y | r | 52625~52626 | 4 | OK |  |
| 22 | *wave_obs(u8, %12) | 0x0 | 관측 코드 | w | L2132 (52733) wave_damage == 0 → None | 4 | 확인불가(tcx 사전에 타입 없음) | 1 |
| 23 | *wave_obs(u8, %12) | 0x0 | 관측 코드 | w | L2153 (52800) danger 0 이고 경미(hp_pct·wave_pct 임계 미달) → None | 4 | 확인불가(tcx 사전에 타입 없음) | 2 |
| 24 | *wave_obs(u8, %12) | 0x0 | 관측 코드 | w | L2161 (52858) 즉살창 & 웨이브가 나를 못 죽임 & !(wave_pct>14 && hp_pct<26) → None / L2181 (52908) 웨이브·내 사망보다 먼저 킬 도달 → None | 4 | 확인불가(tcx 사전에 타입 없음) | 3 |
| 25 | *wave_obs(u8, %12) | 0x0 | 관측 코드 | w | L2187 (52916) hp_pct≥46 & !needs_to_move & wave_damage<hp → None(추격 유지) | 4 | 확인불가(tcx 사전에 타입 없음) | 4 |
| 26 | *wave_obs(u8, %12) | 0x0 | 관측 코드 | w | L2190 (52912) 가드 발동(추격 중단) — 이후 Some(RunAway/KitingBack/End) 반환 | 4 | 확인불가(tcx 사전에 타입 없음) | 5 |
| 27 | *wave_pct_out(u8, %13) | 0x0 | 웨이브 피해 % | w | L2130 (52714~52728) 1회. 조기 None 3경로(L2083·L2206·L2214)에선 미기록 | 4 | 확인불가(tcx 사전에 타입 없음) | min(wave_damage*100 / max(champ.hp,1), 254) as u8 |
| 28 | *wave_danger_out(bool, %14) | 0x0 | 위험 피해 존재 | w | L2147 (52779~52781) 1회. 조기 None 3경로 + L2131(wave_damage==0) 경로에선 미기록 | 4 | 확인불가(tcx 사전에 타입 없음) | danger_damage != 0 |

**`consts` 상수 18건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev | folded_from |
|---|---|---|---|---|---|---|
| 0 | 0 | 2206 | 태그 | max_range == 0 → stance 계산 불가 → None (battle_chase_stance 인라인, 52426) | 4 |  |
| 1 | 15000 | 2225 | 계수 | from_distance = max_range.saturating_sub(15000) — 목표에서 (사거리−15000) 만큼 떨어진 곳을 정지점(stance)으로. L2115 fd 도 동일(투영 stance) | 4 |  |
| 2 | 1 | 2089 | 임계 | ★시프트량: tps*2 → `shl %96, 1`(52551, window 하한 가산) · L2158 tps/2 → `lshr %96, 1`(52823) 로 접힘(folded_from=2). 그 외 리터럴 1 = L2221 sz < 1 (icmp slt, 목표와 겹침 → stance=내 위치·walk_tick 0) / max(move_speed,1)·max(isqrt,1)·max(hp,1)·max(max_hp,1) 0-나눗셈 바닥 / L2094·L2138·L2157·L2173 `version > 1`(ugt 1) | 4 | 2 |
| 3 | 2 | 2153 | 길이 | *wave_obs = 2 (store i8 2, 52800 경미 통과) / L2104·L2214 배열 길이 2 bounds check(nexus[et]·visible_state[t]) | 4 |  |
| 4 | 3 | 2089 | 계수 | window = min(tps*3, max(tps, walk_tick + tps*2)) — 웨이브 피해 적산 창 상한 3초 / L2157 tps/3 (udiv 3, 52815) needs_to_move 임계(걸어갈 틱 ≥ 1/3초) | 4 |  |
| 5 | 100 | 2130 | 계수 | wave_pct = wave_damage*100/hp · L2136 hp_pct = hp*100/max_hp (퍼센트) | 4 |  |
| 6 | 254 | 2130 | 인덱스 | wave_pct_out 상한(umin 254 → u8 trunc). 255 는 미사용 센티널로 남김 | 4 |  |
| 7 | 51 | 2149 | 임계 | hp_pct < 51 (즉 ≤50%) 이고 wave_pct > 11 이면 가드 검토 진입 | 4 |  |
| 8 | 11 | 2149 | 임계 | wave_pct > 11 (≥12%) 짝 | 4 |  |
| 9 | 36 | 2150 | 임계 | hp_pct < 36 이고 wave_pct > 4 이면 가드 검토 진입 | 4 |  |
| 10 | 4 | 2150 | 태그 | wave_pct > 4 (≥5%) 짝 / 반환 태그 4 = BattleSubPlanGoal::RunAway (L2192·L2196) | 4 |  |
| 11 | 26 | 2150 | 임계 | hp_pct < 26 이면 wave_pct 와 무관하게 가드 검토 진입 / L2159 target_hp_ratio < 26 (즉살창 조건) / L2160 hp_pct < 26 & wave_pct > 14 이면 즉살창이라도 코드 3 면제 안 됨 | 4 |  |
| 12 | 14 | 2160 | 임계 | wave_pct > 14 (≥15%) & hp_pct < 26 → 즉살창 면제 취소 | 4 |  |
| 13 | 46 | 2186 | 임계 | hp_pct < 46 이면 needs_to_move 와 무관하게 가드 발동(코드 5) / L2192 can_runaway 시 hp_pct < 46 \|\| wave_damage ≥ hp → RunAway | 4 |  |
| 14 | 3 | 2195 | 태그 | 반환 태그 3 = BattleSubPlanGoal::KitingBack{focus} (Direct 태그, tcxdict --enum) / *wave_obs 코드 3 | 3 |  |
| 15 | 7 | 2196 | 태그 | 반환 태그 7 = BattleSubPlanGoal::End (L2194 !can_runaway & !in_range 도 7) | 4 |  |
| 16 | 5 | 2190 | 산출값 | *wave_obs = 5 (가드 발동) | 4 |  |
| 17 | -1 | 2203 | 센티널 | Option<BattleSubPlanGoal> None 니치 태그 0xFFFFFFFFFFFFFFFF (phi %105) | 4 |  |

**`knobs` 조정점 8건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 정지점 안쪽 마진 | old/battle.rs:2225 (m10.ll:52489) · 2115 (52680) | 15000 | stance = 목표에서 (max_range−15000). 올리면 더 가까이 붙는 지점에서 웨이브 피해를 재므로 가드가 더 자주 발동(라인 안쪽일수록 미니언 피해↑), 내리면 사거리 끝에서 재서 덜 발동 | 4 | 기존 |
| 1 | 피해 적산 창 | old/battle.rs:2089 (52551~52559) | clamp(walk_tick+2·tps, tps, 3·tps) | 상한 3초/하한 1초. 상한을 올리면 wave_damage·danger_damage 가 커져 가드 발동↑ | 4 | 기존 |
| 2 | 경미 통과 임계(코드 2) | old/battle.rs:2149~2150 (52785~52795) | wave_pct>11&&hp<51 · wave_pct>4&&hp<36 · hp<26 | 이 셋 모두 거짓이고 danger 0 이면 무조건 추격 유지. 임계를 낮추면(예: 51→40) 가드 검토가 줄어 더 공격적 | 4 | 기존 |
| 3 | 즉살창 목표 HP% | old/battle.rs:2159 (52829) | 26 | focused_die_tick > tps/2 일 때 target_hp_ratio < 26 & 사거리 안 & 내 사망 + 0.5초 이내 킬이면 웨이브 무시(코드 3). 올리면 더 넓은 HP 대에서 킬 우선 | 4 | 기존 |
| 4 | 즉살창 면제 취소 | old/battle.rs:2160 (52852~52853) | wave_pct>14 && hp_pct<26 | 이 조합이면 즉살창이라도 면제 안 됨. 낮추면 저체력 킬 시도가 줄어듦 | 4 | 기존 |
| 5 | 발동 HP% | old/battle.rs:2186 · 2192 (52862 · 52923) | 46 | hp_pct < 46 이면 needs_to_move 와 무관하게 발동, can_runaway 면 RunAway. 올리면 더 일찍 추격 포기 | 4 | 기존 |
| 6 | needs_to_move 걸음 임계 | old/battle.rs:2157 (52815) | tps/3 | 사거리 안·open_eval 아님일 때 stance 까지 걸을 틱 ≥ 1/3초면 '이동 필요'로 보고 발동 후보. 분모 3 을 키우면 더 짧은 이동도 발동 | 4 | 기존 |
| 7 | wave_pct_out 상한 | old/battle.rs:2130 (52726) | 254 | 관측 출력 캡(u8). 판정엔 wave_pct 원값(캡 전 %183)이 쓰이므로 동작 무영향 | 4 | 기존 |

<details><summary>`callees` 피호출자 10건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | adjust_position | game_core::Game::adjust_position | pub | fn(&game_core::MapDef, &game_core::GameSetting, i64, i64) -> (u64, u64) | game-core\src\simulation\game.rs:818 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | battle_chase_stance | game_ai::plan_legacy::old::battle::battle_chase_stance | in:game_ai | fn(&game_core::OperationData, &game_core::Entity, &game_core::Entity, u64) -> std::option::Option<(u64, u64, usize)> | game-ai\src\plan_legacy\old\battle.rs:2205 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 2 | distance | game_core::Entity::distance | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2161 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 3 | enemy_minion_line_action_damage_at | game_ai::enemy_minion_line_action_damage_at | pub | fn(usize, &game_core::OperationData, &game_core::Entity, u64, u64, usize, bool, bool) -> usize | game-ai\src\minion_wave_risk.rs:130 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | enemy_minion_line_action_danger_damage_at | game_ai::enemy_minion_line_action_danger_damage_at | pub | fn(usize, &game_core::OperationData, &game_core::Entity, u64, u64, usize, bool, bool) -> usize | game-ai\src\minion_wave_risk.rs:233 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 5 | enemy_minion_wave_danger_damage_at | game_ai::enemy_minion_wave_danger_damage_at | pub | fn(usize, &game_core::OperationData, &game_core::Entity, u64, u64, usize) -> usize | game-ai\src\minion_wave_risk.rs:99 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 6 | enemy_minion_wave_risk_damage_at | game_ai::enemy_minion_wave_risk_damage_at | pub | fn(usize, &game_core::OperationData, &game_core::Entity, u64, u64, usize) -> usize | game-ai\src\minion_wave_risk.rs:6 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 7 | is_visible_from | game_core::Entity::is_visible_from | pub | fn(&game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1481 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 8 | isqrt | game_core::utils::isqrt | pub | fn(i64) -> i64 | game-core\src\utils.rs:85 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 9 | v30_wave_danger_chase_guard | game_ai::plan_legacy::old::battle::v30_wave_danger_chase_guard | in:game_ai | fn(usize, &game_core::OperationData, &game_core::Entity, &game_core::Entity, u64, usize, bool, usize, usize, bool, bool, bool, &mut u8, &mut u8, &mut bool) -> std::option::Option<game_ai::plan_legacy::old::BattleSubPlanGoal> | game-ai\src\plan_legacy\old\battle.rs:2079 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
</details>

⚠**미매칭 9개**: `clamp`, `else`, `llvm.smax.i64`, `llvm.uadd.sat.i64`, `llvm.umax.i64`, `llvm.umin.i64`, `llvm.umul.with.overflow.i64`, `llvm.usub.sat.i64`, `saturating_mul`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 3곳** (m05.ll:24681, m05.ll:32692, m10.ll:19557) · **형제 0개** 

**`open` 5건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | exe 콜리 6(비-panic) vs IR 비-panic 콜리 7(distance·isqrt·adjust_position·line_action_damage_at·line_action_danger_damage_at·wave_risk_damage_at·wave_danger_damage_at) — 어느 하나가 exe 에서 인라인/ICF 병합됐는지 rvaname 으로는 인스턴스만 나와 미판정(0xd96190 2418B·0xd95d00 1154B·0xd96b10 485B 의 개별 대응 미확정). 함수 자체 판정(크기·Location 2/2·인자 15·호출자 3/3)에는 영향 없음 | 4 |  |
| 1 | 표기 불가 | L2150 `(wave_pct>4 && hp_pct<36) \|\| hp_pct<26` 의 줄 안 평가 순서 — column 0 이라 표기 불가(외연 동일, 부작용 없음) | 4 |  |
| 2 | 표기 불가 | L2194/L2196 의 정확한 소스 구조(`!can_runaway && !in_range → End` 가 L2194 의 else 인지 별도 줄인지) — 두 경로 모두 태그 7 로 합쳐져(phi %105 [7,%274]) 외연 동일, 표기 불가 | 4 |  |
| 3 | 미탐색 | wave_obs 코드 1~5 의 소비처(어느 필드/텔레메트리로 가는지)는 호출자 3곳 소관 — 본 함수는 값만 기록 | 4 |  |
| 4 | 미탐색 | danger_damage(=line_action_danger_damage_at)와 wave_damage(=line_action_damage_at)의 의미 차이(위험 피해 vs 기대 피해)는 콜리 minion_wave_risk 명세 소관(계약만: 둘 다 i64 피해량) | 4 |  |

<details><summary>`closed` 1건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

| # | 물음 | 닫은 근거 |
|---|---|---|
| 0 | 0xe119e0 의 정체 — Location old/battle.rs:1036:13 · 간접 call 3 · 콜리 0xe0bf70 · 호출자 0xca1840. 이 함수가 아니라는 것만 확정(지도 라벨 오귀속 의심), 실명은 범위 밖(rvaname 도 인스턴스 1건만) | 본문에 해소 표기가 있다 |
</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

