# eba9b0→edb240 fight_check::check_kill_die_tick_uncached

## logic_060
```
// ★0.6.0 ABI(w1 §1-2 확정): fn check_kill_die_tick_uncached(version, data, judger: &PlayerState, focus: &Entity,
//     enemy: &Vec<&Entity>, towers: &Vec<&Entity>, simple: bool, ignore_nuke: bool, no_noise: bool,
//     extra: &Option<(x:u64, y:u64, t:u64)>) -> usize
//   (구 (version,_rnd,data,judger,focus,enemy,towers,_debug) — rnd/debug 삭제 · bool×3 + Option 추가 · §A 의 「&ally」 표기는 오기 = towers)
//   TLS 캐시 래퍼 = eda920 (키: version·judger.id·focus.id·Option 4q·enemy id≤8·towers id≤12·len×2·bool×3·seed(vt0x20)·tick(vt0x28) · enemy.len≥9 ‖ towers.len≥13 이면 캐시 우회)
//   ★런타임 실측: 콜러 43 사이트 29.7M 호출 전부 extra=None(spec_patch §A version 행) — Some 경로는 아래에 적되 배경 sim 기준 사장.
// ★v2 사장: `version < 3` 가지(= 0.5.8 본체 + simple/ignore_nuke/no_noise 게이트 · w1 §1-3) 는 v3 런타임에서 도달 불가 — 아래는 v3 가지만.

fn check_kill_die_tick_uncached(version, data, judger, focus, enemy, towers, simple, ignore_nuke, no_noise, extra) -> usize
 // ── 공통 준비 ──
 player = cache.player_by_champion_id(focus.id)   // ★0.6.0 d72be0: cache+0x1e0 슬롯 10개 id(+0x5c0) 비교 → +0x230+idx*8 PlayerState(0 이면 unwrap_failed)
 tps_raw = setting(+0x12f8) ; tps = max(tps_raw, 1)                                           // setting = data.context+8
 // ★0.6.0 삭제: bucket = tick/(tps*2) — v3 시드에 tick 버킷 없음(2초 드리프트 제거)
 // (A) ★0.6.0 undying: 조기 반환(i64::MAX) 대신 「남은 무적 틱」 하한
 undying_floor = if focus.stat_buff_cached.undying(+0x488) {
     // focus.effect_buffs = Vec @ +0x2e0 ptr / +0x2e8 len · stride 0x120 · 원소 +0xd0 u8==1 이 undying 버프
     m = max over such e of match e.duration_tag(+0 u32) { 0 /*무한*/ => i64::MAX, 1 /*틱*/ => e[+8], _ /*만료*/ => 0 }
     max(m, 1) } else { 0 }
 // (B) ★0.6.0 시드 = 적/타워 id 집합 해시 (G = 0x9E3779B97F4A7C15)
 h = 0
 for e in enemy  { v = e.id*G + G ; h ^= rotl64(v, e.id % 61) }                                  // ★0.6.0 (디컴 edc5xx: -0x61c8864680b583eb == G · % 0x3d)
 for t in towers { v = t.id*0xC2B2AE3D27D4EB4F + 0x52E2C3AC16D26F29 ; h ^= rotl64(v, t.id % 59) }   // ★0.6.0 (% 0x3b)
 rng = NoiseRng( h ^ (focus.id << 24) ^ (judger.info.id(+0x9f8) * G) )                          // ★0.6.0 judger.id +0x928→+0x9f8 · splitmix64 noise() 본체는 0.5.8 과 동일
 ja = 100 + 9*min(judger[+0x480]*judger[+0x218]/1000, 100)   // judge_accuracy 인라인(구 +0x450→+0x480)
 d = if simple || no_noise { 0 } else { (1000 - ja) >> 1 }   // ★0.6.0 = (900 - 9*min(..,100))>>1 · d==0 이면 noise()==1000 고정(rng 는 계속 소비)
 noise() := rng.range_usize(1000 - d, 1000 + d)             // 0.5.8 동일(splitmix64 → hi64(z*(hi-lo+1))+lo)
 (X, Y, floor_t) = match extra { Some((x,y,t)) => (x,y,t), None => (focus.x(+0x660), focus.y(+0x668), 0) }   // ★0.6.0 (v3 만 읽음)
 cc(p) := cache.player_champion_cache[p.team][p.pos]   // ★0.6.0 인덱스 = qword[team*0x212 + pos*0x6a + …](ChampionCache 0x320→0x350 · 팀 stride 0x1090)
 fpos := player.pos(+0xa90)

 // (C) ★0.6.0 적 챔피언 → 레코드 Vec (bumpalo · 원소 0x28B = 5 qword)
 struct Rec { arrival: u64, nuke: u64, dps: u64, ult_delayed: u64, ult_delay: u64 }
 recs = Vec::new_in(pool)
 for pchamp in enemy {
   p = cache.player_by_champion_id(pchamp.id).unwrap() ; f = fpos
   // *_cooldown() = Entity ty(+0x68) 별 필드(JT 14엔트리: 13/8→+0xb0, 4/7→+0xe8, 5/6→+0x1f0, 10→+0xf0, 11→+0xd8, 1→+0xb8, 2→+0x110, 12→+0xd0, 9→+0xc8, 0/3→무조건 통과) · > tps_raw 면 해당 항 0·rng 미소비
   burst = 0
   if pchamp.can_attack() || pchamp.attack_cooldown() <= tps_raw { burst = cc(p).attack[f] * noise() / 1000 }                    // rng#1
   if pchamp.can_skill()  || pchamp.skill_cooldown()  <= tps_raw { burst = max(burst, cc(p).skill[f]  * noise() / 1000) }        // rng#2
   if pchamp.can_skill2() || pchamp.skill2_cooldown() <= tps_raw { burst = max(burst, cc(p).skill2[f] * noise() / 1000) }        // rng#3
   (imm, delayed, delay) = (0, 0, 0)
   if !simple && (pchamp.can_ult() || pchamp.ult_cooldown() <= tps_raw) {                       // ★0.6.0 simple 이면 rng#4 도 미소비
     n = noise() ; ult = cc(p).ult[f] * n / 1000 ; delay = cc(p)[0x4b]                            // rng#4 · ★0.6.0 신규 캐시 필드 0x4b = 궁 피해 지속 틱(Effect vt 0x48)
     if delay != 0 { imm = min(cc(p)[0x46 + f] * n / 1000, ult) ; delayed = ult - imm }          // ★0.6.0 신규 캐시 [0x46..0x4a] = 궁 총피해 중 첫 tps/4 틱 즉시분(빌더 1517380)
     else { imm = ult }
   }
   a_ps  = if !pchamp.is_block_attack() { cc(p).attack_per_sec[f] * noise() / 1000 } else { 0 }                                                          // rng#5
   s_ps  = if !pchamp.is_block_skill() && !(pchamp.is_block_move_skill() && pchamp.skill_effect.map_or(false,|e| e.can_move()/*vt 0x140 ★0.6.0 구 0x120*/)) { cc(p).skill_per_sec[f]  * noise() / 1000 } else { 0 }   // rng#6
   s2_ps = if !pchamp.is_block_skill() && !(pchamp.is_block_move_skill() && pchamp.skill2_effect().map_or(false,|e| e.can_move())) { cc(p).skill2_per_sec[f] * noise() / 1000 } else { 0 }   // rng#7 · skill2_effect() = level(+0x5c8)>2 ? Some : None
   nuke = max(imm, burst)
   arrival = if simple { 0 } else {                                                              // ★0.6.0 신규: 도달 틱
     reach = de9900(data, pchamp, focus)   // ★0.6.0 TLS 메모 → deb270: 사용가능(165e7a0)한 attack/skill/skill2(lv≥3)/ult(lv≥5) 중 최대 사거리 = base(+0x4a0)+(lv-1)*growth(+0x4a8)+stat_buff.range(+0x438)+vt0x108(eff,a,t)+a.radius+t.radius
     dist  = isqrt_dist(pchamp.xy, (X,Y))   // 1660a00
     t = if dist > reach { (dist - reach) / max(pchamp.move_speed(+0x640), 1) } else { 0 }
     if extra.is_some() { max(t, floor_t) } else { t } }
   (nuke, delayed) = if ignore_nuke { (0, 0) } else { (nuke, delayed) }                          // ★0.6.0
   recs.push(Rec{ arrival, nuke, dps: a_ps + s_ps + s2_ps, ult_delayed: delayed, ult_delay: delay })   // 디컴 local_118[len*5 + 0..4]
 }
 // (D) 타워 — 0.5.8 L1049~1054 동일 (rng#8 dps · rng#9 nuke) → base_dps, base_nuke
 base_dps = 0 ; base_nuke = 0
 for tower in towers {
   damage = tower.attack_effect.unwrap().expected_damage_target(ctx, tower, focus)   // 1643790 estimate_damage_to(eff,ctx,caster,static,target)
   base_dps  += (noise() * tps_raw * damage / 1000) / tower.attack_cooltime()        // cooltime 0 → div_by_zero 패닉   rng#8
   base_nuke += damage * noise() / 1000                                               // rng#9
 }
 // (E) others — !simple 일 때만 · 거리 기준점 = (X,Y) ★0.6.0
 if !simple { for e in cache.others[1 - player.team] {
   if dist²(e, (X,Y)) > 22500000000 { continue }                                      // > 150,000
   if !game.is_visible(judger.team, e.id) { continue }                                // vt +0xf8
   if let Some(atk) = e.attack_effect(+0x4c0 != -1) { damage = atk.expected_damage_target(ctx, e, focus)
     base_dps += damage * tps_raw / max(e.attack_cooltime(), 1) ; base_nuke += damage }   // 노이즈 없음
 } }
 // (F) aura · heal · 미니언 웨이브 — !simple 일 때만 ★0.6.0 (simple 이면 heal=0 · 차감도 없음)
 heal = 0
 if !simple {
   for pchamp in enemy { for b in pchamp.effect_buffs { base_dps += b.expected_aura_dps_at(ctx, pchamp, focus) } }          // Buff vt 0x80
   for ally in cache.iter_champions(player.team) { for b in ally.effect_buffs { heal += b.expected_aura_heal_at(ctx, ally, focus) } }   // Buff vt 0x88
   enemy_epic_buff := game.get_game_mode() is Moba(m) && m.epic_minion_buff_time[1 - player.team](+0x240+team*8) != 0
   low_enough_to_care_minions := focus.hp(+0x670)*100 <= max(focus.stat_cached.hp(+0x628),1)*75
   line_phase := if tutorial(ctx+0x38) ∈ {0,5,7,8} { game.tick() < setting.epic_jungle.first_spawn_tick(+0x8a8).saturating_sub(tps_raw*30) } else { true }
   if enemy_epic_buff || (low_enough_to_care_minions && !line_phase) { base_dps += enemy_minion_wave_risk_dps_at(version, data, focus, X, Y) }   // ★0.6.0 1006be0 · 좌표 (X,Y)
 }
 revive_hp = Σ focus.effect_buffs.revive_bonus_hp(ctx, focus)   // Buff vt 0x90
 // (G) ★0.6.0 타임라인 (구 `(revive+hp−nuke)*60/max(dps,1)` 전면 교체)
 pool = sat_mul(revive_hp + focus.hp, tps)                       // ★HP·nuke 모두 tps 배율 → 반환 단위 = 틱(tps 종속 · 구 *60 고정 아님)
 bp = [0u64; 96] ; n = 1                                          // 정렬·유일 삽입 · 96 초과분 무시(bounds 0x60)
 for r in &recs { insert_sorted_unique(bp, &mut n, r.arrival) ; if r.ult_delay != 0 && r.ult_delayed != 0 { insert_sorted_unique(bp, &mut n, sat(r.arrival + r.ult_delay)) } }
 acc = if ignore_nuke { 0 } else { sat_mul(base_nuke, tps) }
 for i in 0..n {
   t = bp[i] ; dps = base_dps
   if !recs.is_empty() {
     for r in &recs { if r.arrival == t { acc = sat(acc + sat_mul(tps, r.nuke)) } }
     if pool <= acc { return max(t, undying_floor) }
     for r in &recs { if r.arrival <= t {
       dps = sat(dps + r.dps)
       end = sat(r.arrival + r.ult_delay)
       if t < end { dps = sat(dps + sat_mul(tps, r.ult_delayed) / r.ult_delay) } } }
   } else if pool <= acc { return max(t, undying_floor) }
   net = sat(dps - heal)
   if i + 1 == n {                                                 // 마지막 구간
     if net == 0 { return max(pool, undying_floor) }              // dps 0 이면 pool(=hp*tps) 그대로
     return max(t + ceil((pool - acc) / net), undying_floor) }    // ceil = (pool-acc+net-1)/net
   next = bp[i+1]
   if net > 0 { cand = t + ceil((pool - acc) / net) ; if cand < next { return max(cand, undying_floor) } }
   acc = sat(acc + sat_mul(net, next - t))
 }
 // 모든 곱은 u128 검사 후 u64::MAX 포화(sat_mul) · 드롭: recs(bumpalo)
 // NoiseRng 추첨 순서(v3): 적 챔피언마다 최대 7회(attack·skill·skill2·ult(!simple)·attack_ps·skill_ps·skill2_ps, 각 조건부) → 타워마다 2회 → others/버프 0회. rng(StdRng) 인자 자체가 사라짐.
```

## changes
- ABI: `(version,_rnd,data,judger,focus,enemy,towers,_debug)` → `(version,data,judger,focus,&enemy,&towers,simple,ignore_nuke,no_noise,&Option<(x,y,t)>)`(래퍼 eda920 동일) — rnd/debug 삭제 · bool×3·Option 추가.
- L976 `undying → return i64::MAX` 삭제 → `undying_floor`(활성 undying 버프 잔여 틱 최댓값·무한이면 MAX, 최소 1) 을 모든 반환에 하한으로 적용.
- L986~990 시드: tick 버킷 삭제 · `h = XOR(rotl(e.id*G+G, e.id%61)) ^ XOR(rotl(t.id*C1+C2, t.id%59))` 뒤 `^ (focus.id<<24) ^ (judger.id*G)`.
- L994~995 노이즈 폭: `simple‖no_noise → d=0`.
- L1021~1023 궁: `!simple` 게이트 + 즉시분/지연분 분리(캐시 `cc[0x46+fpos]`, `cc[0x4b]`).
- 신규 arrival(적별 도달 틱 = (dist−reach)/ms · reach=de9900) · Option 이 Some 이면 위치 대체 + `max(arrival, t)`.
- L1058~1099 others/aura/heal/minion: `!simple` 게이트 · others·minion 좌표 = (X,Y).
- L1104~1107 반환식 → 타임라인(pool=hp*tps · 96 브레이크포인트 · 구간별 누적) · `ignore_nuke` 면 챔피언 nuke/초기 acc 0.
- 콜리 v3: d31bb0→d72be0 player_by_champion_id · 12857f0→1643790 estimate · d958b0→1006be0 minion_wave · can_move vt 0x120→0x140 · 신규 de9900/deb270(최대 사거리)·1660a00(isqrt 거리)·dc0380(bumpalo grow).
- 오프셋: judger.id +0x928→+0x9f8 · judge 파라미터 +0x450→+0x480 · fpos +0x9c0→+0xa90 · ChampionCache stride 0x320→0x350(qword idx team*0x212+pos*0x6a).

## verified
- Ghidra 0.6.0 edb240 디컴(1545줄 스팟체크): `if (param_1 < 3)` v2/v3 분기(L136) · 적 시드 루프 `id*G+G · rotl(id%61)`(edc5xx) · 타워 `*0xC2B2AE3D27D4EB4F+0x52E2C3AC16D26F29 · rotl(id%59)`(edc61e~) · 궁 `cc[fpos+0x46]`/`cc[0x4b]`(L860~864) · `pool = (Σrevive + hp(+0x670)) * tps` u128 포화(L1113~1120) · `memset 0x300` + 96 브레이크포인트 정렬삽입(bounds 0x60 · L1127~1290) · 반환 경로 max(undying_floor) 구조.
- 확인 못 한 지점: 타임라인 (G) 의 마지막-구간 `net==0 → pool` 분기와 `cand<next` 비교 세부는 RE w1 등가 Rust 를 그대로 옮김(디컴 재대조 생략) · undying 버프 스캔(+0x2e0/+0x2e8/+0xd0) 은 w1 §1-5 근거 · `extra` Some 콜러 부재는 정적 스캔+런타임 실측(29.7M None) 근거.

## confidence
A — 신규 v3 알고리즘 전문이 RE w1 등가 Rust 로 확정돼 있고 핵심(시드·캐시 필드·pool·브레이크포인트)을 0.6.0 디컴에서 재확인. 잔여 추정은 캐시 필드 명칭뿐(산식은 확정).