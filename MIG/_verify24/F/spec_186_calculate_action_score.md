---

### `186` calculate_action_score — 액션(평타/스킬)을 대상 t 에 쓸 때의 점수 — 미니언은 막타 타이밍·라인 스타일, 챔피언/타워/넥서스는 계수×기대피해/HP + 보너스

| 항목 | 값 |
|---|---|
| id | `action_score__calculate_action_score` |
| 심볼 | `_RNvNtCshdEBA0ozCnw_7game_ai12action_score22calculate_action_score` |
| 소스 | `game-ai\src\action_score.rs:8` |
| IR | `m05.ll` 37635~39981행 |
| 경로·가시성 | `game_ai::calculate_action_score` · **pub** |
| 계층 | 점수화·술어 |
| exe | `d59940` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r15` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &std::boxed::Box<dyn [Binder { value: Trait(game_core::Action), bound_vars: [] }] + 'static, std::alloc::Global>, &game_core::Effect, usize, &game_core::Entity, game_ai::MinionActionType, &mut game_core::DebugFrameData) -> i64
```

<details><summary>인자 11개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | version | usize | 분기 없음 · range_misjudge_rng 에 전달만 | 4 |
| 1 | 2 | rnd | &mut StdRng(320B) | gen_range 11지점 + range_misjudge_roll_i64 8지점(jrng None 이면 rnd 소비) — writes 참조 | 4 |
| 2 | 3 | player | &PlayerState(2528B) | readonly · 0x930 team · 0x9c0 position · 0x180 parameter(last_hit_accuracy) · range_misjudge_rng 인자 | 4 |
| 3 | 4 | data | &OperationData(24B) | readonly · +0 cache · +8 context · +16 blackboard[2] | 4 |
| 4 | 5 | parameter | &ScoreParameter(5384B) | readonly · +0 wave_snapshot Option 태그(1=Some) · +8 minions[12](stride 192) · +0x908 count 만 읽음 | 4 |
| 5 | 6 | action | &Box<dyn Action>(16B) | readonly · 팻포인터 +0 data +8 vtable · vtable+0x90 cooltime(&self, champ) 1회 | 4 |
| 6 | 7 | effect | &Effect(56B) | readonly · +0x20 start_timing · expected_damage_target(self) 인자 | 4 |
| 7 | 8 | speed_mult | usize | 0 이면 L22 udiv 패닉(m05.ll:37826) | 4 |
| 8 | 9 | t | &Entity(1728B) | readonly · 평가 대상 엔티티 | 4 |
| 9 | 10 | ty | MinionActionType i8 range(0,3) | 0 Pull / 1 Normal / 2 Push (tcxdict --enum, Direct) | 3 |
| 10 | 11 | _debug | &mut DebugFrameData(224B) | readnone — 본문 사용 0건(잔여 인자) | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
// L10  team=player.team(<2); champ = cache.player_champion[team][position].unwrap()
// L11  acc = parameter.last_hit_accuracy(); base = 1000-acc; min_v=acc; max_v=2000-acc
// L18  jrng = range_misjudge_rng(version, data, player, t.id)
// L19  value = effect.expected_damage_target(ctx, champ, t) * roll/1000        // roll = range_misjudge_roll_i64(rnd,&jrng,acc,2000-acc), 매번 새 롤
// L20  hp = t.hp*roll/1000;  L21 total = t.stat_cached.hp*roll/1000
// L22  start_timing = (effect.start_timing*100/speed_mult)*roll/1000   (speed_mult==0 → 패닉)
// L23  cooltime = (action.cooltime(champ)*100/speed_mult)*roll/1000    // %88 = roll*raw (÷1000 전) 이 뒤의 29999 비교에 쓰임
// L26  match t.ty {
//  Minion(1) =>
//   L29 if let Some(target)=t.Minion.nearest_enemy.and_then(get_entity_by_id) { match target.ty {
//     Tower(2) => { L32 if Moba && epic_minion_buff_time[1-team]!=0 → return 30;  L37 if target.Tower.ty ∈{TwinA,TwinB} { L38 if Moba && buff[1-team]!=0 → return 70 else → return 50 } }
//     Nexus(3) => { L46 if Moba && buff[1-team]!=0 → return 100 else → return 70 }
//     _ => {} } }
//   L55 if Moba && epic_minion_buff_time[1-team]!=0 → return 20        // 적 에픽버프 미니언
//   L62 if let Some(snap)=parameter.wave_snapshot { L63 if let Some(traj)=snap.minions[..count].find(id==t.id) {   // count>12 → 패닉
//     L68 error_prob = min(base*base/1000, 1000)
//     L72 if gen_range(0..=1000) < error_prob → return -9999               // 오판
//     L76 lane_phase = !(tutorial∈{None,MidBottom,Line,Total} && tick < first_spawn_tick - 30*tps ? false : true)  — 정확히: early=(tutorial∈집합 && tick<spawn-30tps); flag = !early && position!=Jungle   ⚠주의: tutorial∈집합 && tick≥경계 → flag=false 가 아니라 … IR: tutorial∈집합 이면 tick<경계 일 때만 position!=Jungle, 아니면 false; tutorial∉집합 이면 position!=Jungle
//     L89 predicted_hp = hp_at_tick(traj, start_timing)*roll/1000 ; L90 death_tick = traj.expected_death_tick
//     L93 if roll*hp_at_tick > 999 (생존) {
//        can_last_hit(DI명) = predicted_hp > value+5   // = 이번 타격으로 못 죽임
//        will_die_soon = death_tick ≤ cooltime+start_timing
//        L97 if !can_last_hit (지금 죽일 수 있음) {
//          L100 urgency = death_tick > start+5 ? (will_die_soon ? 25 : 15) : 30
//          L113 concurrent = #{other in snap: id≠t.id && 0 < hp_at_tick(other, cooltime+start) ≤ value+5}
//          L126 multi_bonus = concurrent>0 ? (L129 earlier=#{other: id≠t.id && other.death_tick < death_tick}; earlier==0 ? 5 : -5) : 0
//          L147 return flag ? urgency+multi_bonus : urgency+3+multi_bonus
//        } else { L152 if will_die_soon && predicted_hp ≤ 2*value → return ty==Pull ? -9999 : 5 }
//     }
//     L159 if predicted_hp > 3*value { L161 match ty { Pull→-9999, Push→10, Normal→ L168 if roll*cooltime_raw>29999 → (gen<err?10:-9999) else if flag → (gen<err?10:-9999) else → (gen<err?-9999:10) } }
//     else { L178 match ty { Pull→-9999, Push→10, Normal→ L184 gen<err ? 10 : -9999 } }
//   } }
//   // 스냅샷 없음/미등재 → 레거시(L195~)
//   L199 flag(%420) 위와 동일 계산
//   L211 for e in iter_entity(): match e.ty { Minion|Tower|Ghoul|SmallJiangshi|Bear|Eagle }: if e 가 t 를 조준(nearest_enemy/target_enemy == t.id) { dmg = e.attack_effect.expected_damage_target(ctx,e,t) (None→패닉); acd=attack_cooltime(e); ast = attack_effect.start_timing*100/attack_speed_mult(e); if e.state==Attack && !(Minion && is_range) && time<ast && ast(+15 if Tower)-time < start_timing && Attack.target_id==t.id → applyed += dmg; expected += max((cooltime+start_timing)/acd,1)*dmg (acd==0→패닉) }
//   L297 for p in iter_projectile(): if p.move_type==Target(6) { if target_id==t.id { caster=get_entity_by_id(p.caster_id)?; dist=distance(p,t); dmg=p.expected_damage_target(ctx,caster,t); if start_timing ≥ dist/speed+5 → applyed+=dmg; expected+=dmg } } else { caster?; if p.applyed_target.check_projectile(p,t) && p.is_in_orbit(t.x,t.y, radius*(100+radius_mult)/100) { dmg=…; applyed+=dmg; expected+=dmg } }
//   L319 applyed*=roll/1000; L320 expected*=roll/1000; L321 error_prob = 1000 - gen_range(acc..=1000)
//   L325 if value-5+applyed < hp (못 죽임) {
//      L365 if value + total*3/10 + expected < hp { L368 Pull→-9999 · Push→10 · Normal→ L378 cooltime>29999 ? (gen<err?10:-9999) : flag ? (gen<err?10:-9999) : (gen<err?-9999:10) }
//      else { L393 Pull→-9999 · Push→10 · Normal→ L402 gen<err?10:-9999 }
//   } else { L329 if gen_range(0..=1000) < error_prob → return -9999
//      L333 if expected + total*3/10 < hp { L335 Pull→-9999 · Push→10 · Normal→ L345 cooltime>29999 ? (gen<err?10:-9999) : flag ? (gen<err?10:-9999) : (gen<err?-9999:10) }
//      else { L359 gen; flag ? (gen<err ? -9999 : 15) : (gen<err ? -9999 : 20) } }
//  Champion(13) =>
//   L414 bonus=0; if t.team != champ.team { L416 if let Some(ep)=player_by_champion_id(t.id) { L417 a = blackboard[1-team].small_actions[ep.position]; if a.tag∈{Attack,Skill,Skill2} { L419 if let Some(tt)=get_entity_by_id(a.target_id) { L420 match tt.ty { Tower → bonus = twin ? 80 : 10 (L424); Nexus → bonus = 100 (L428) } } } } }
//   → L438 공통
//  _ => bonus = 0 → L438
// }
// L438 value = effect.expected_damage_target(ctx, champ, t) (롤 없음)
// L452 has_epic_buff(DI명) = !(Moba && ptr) || epic_minion_buff_time[team]==0     ⚠이름과 반대 극성처럼 보임 — IR 그대로: '아군 에픽버프 없음' 이 true
// L454 coef = match t.ty {
//   Nexus(3) → 200
//   Tower(2) → L457 in_range_minion = #{e: e.team==champ.team && e.ty==Minion && champ.attack_effect.is_in_range(t, e)}   // champ.attack_effect None 이면 unwrap 패닉
//              L460 cond = match t.Tower.nearest_enemy { Some(id) if id!=champ.id → match get_entity_by_id(id) { Some(e) → e.hp ≥ champ.attack_effect.expected_damage_target(ctx, t, e) || in_range_minion>1, None → in_range_minion>1 }, _ → false }
//              L462 if cond { ty==Push ? (has_epic_buff ? 160 : 240) : (has_epic_buff ? 80 : 160) } else { L476 t.hp > effect.expected_damage_target(ctx,champ,t) ? 0 : 30 }
//   _ → 0 }
// L516 return min(coef, coef*value / t.hp) + bonus     // t.hp==0 → 패닉
```

**`mem` 메모리 접근 53건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | L10 <2 bounds · 1-team = 적 팀 인덱스 | 4 | OK |  |
| 1 | PlayerState | 0x9c0 | info.position@tag | r | L10 player_champion 인덱스 · L77/L200 `position != 1(Jungle)` | 4 | OK |  |
| 2 | PlayerState | 0x180 | info.parameter | r | L11 last_hit_accuracy 인자 | 4 | OK |  |
| 3 | OperationData | 0x0 | cache | r | &AbstractGameWithCache | 4 | OK |  |
| 4 | OperationData | 0x8 | context | r | &GameContext → expected_damage_target 인자 | 4 | OK |  |
| 5 | OperationData | 0x10 | blackboard | r | &[Blackboard;2] · L417 blackboard[1-team](stride 744) | 4 | OK |  |
| 6 | AbstractGameWithCache | 0x0 | game | r | dyn AbstractGame 팻포인터 · vtable +0x28 tick · +0x40 get_game_mode · +0x1f0 get_entity_by_id · +0x200 iter_entity · +0x210 iter_projectile | 4 | OK |  |
| 7 | AbstractGameWithCache | 0x1e0 | player_champion[team][pos] | r | L10 · None 이면 unwrap 패닉(Option 강제) | 4 | OK |  |
| 8 | GameContext | 0x8 | setting | r | &GameSetting | 4 | OK |  |
| 9 | GameContext | 0x38 | tutorial@tag | r | L76/L199 · {0 None,5 MidBottom,7 Line,8 Total} 이면 라인전 시간 게이트 적용 | 4 | OK |  |
| 10 | GameSetting | 0x8a8 | epic_jungle.first_spawn_tick | r | L703 · 라인전 종료 = first_spawn_tick - 30*tps | 4 | OK |  |
| 11 | GameSetting | 0x12f8 | tick_per_second | r | L704 · ×30 | 4 | OK |  |
| 12 | MobaMode | 0x240 | epic_minion_buff_time[team] | r | stride 8 · L32/38/46/55 는 [1-team](적) · L452 는 [team](아군) · !=0 = 버프 중 | 4 | OK |  |
| 13 | ScoreParameter | 0x0 | wave_snapshot@tag | r | L62 · bit0 1=Some | 4 | OK |  |
| 14 | ScoreParameter | 0x8 | wave_snapshot.minions[i] | r | stride 192 · +0x90 entity_id(L63/86/115/131) · +0xb8 expected_death_tick(L90/134) · hp_at_tick 인자 | 4 | OK |  |
| 15 | ScoreParameter | 0x908 | wave_snapshot.count | r | L85 · >12 이면 bounds 패닉 | 4 | OK |  |
| 16 | Effect | 0x20 | start_timing | r | L22 · ×100/speed_mult | 4 | OK |  |
| 17 | Box<dyn Action> | 0x0 | data/vtable | r | L23 vtable+0x90 cooltime(&self, champ:&Entity) → ×100/speed_mult | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 18 | Entity | 0x5c0 | id | r | t.id(L18 jrng 키·대상 대조 다수) · champ.id(L460) | 4 | OK |  |
| 19 | Entity | 0x670 | hp | r | t.hp L20(롤) · L516 분모(0 → 패닉) · L478 · 타워 nearest_enemy e.hp L461 | 4 | OK |  |
| 20 | Entity | 0x628 | stat_cached.hp | r | t 최대 HP → total(L21, 롤) · total*3/10 (L333/365) | 4 | OK |  |
| 21 | Entity | 0x68 | ty@tag | r | t: L26 switch 1 Minion/13 Champion/그외 · 루프 e: 1,2,7,8,9,10 · nearest target: 2 Tower/3 Nexus | 4 | OK |  |
| 22 | Entity | 0x0 | team@tag | r | L414/L458 TeamType 동등비교(Player 면 +0x8 .0 도 비교) | 4 | OK |  |
| 23 | Entity | 0x8 | team@Player.0 | r |  | 4 | OK |  |
| 24 | Entity | 0x70 | ty@*.info.state@tag | r | 루프 e(Minion/Tower/Ghoul/SmallJiangshi/Bear) · bit0 = Attack 상태 · Eagle 은 nearest_enemy@tag | 4 | OK |  |
| 25 | Entity | 0x78 | ty@*.info.state@Attack.target_id | r | == t.id 이면 applyed 가산 · Eagle 은 nearest_enemy Some.0 | 4 | OK |  |
| 26 | Entity | 0x80 | ty@*.info.state@Attack.time | r | time < ast && ast(+15 for Tower) - time < start_timing | 4 | OK |  |
| 27 | Entity | 0x88 | ty@Minion/Ghoul/Bear/Tower.info.nearest_enemy@tag | r | L29(t) · 루프 e | 4 | OK |  |
| 28 | Entity | 0x90 | ty@Minion/Ghoul/Bear.info.nearest_enemy@Some.0 | r | == t.id | 4 | OK |  |
| 29 | Entity | 0x98 | ty@Tower.info.nearest_enemy@Some.0.1 | r | L228 (e) · L460 (t 가 Tower 일 때 조준 엔티티 id) | 4 | OK |  |
| 30 | Entity | 0x100 | ty@SmallJiangshi.info.target_enemy | r | L280 == t.id | 4 | OK |  |
| 31 | Entity | 0x118 | ty@Minion.info.is_range | r | L219 · 원거리 미니언은 applyed(진행중 타격) 미가산 | 4 | OK |  |
| 32 | Entity | 0x128 | ty@Tower.info.ty@tag | r | L37/L424 · -3 <u 2 → TwinA(3)/TwinB(4) | 4 | OK |  |
| 33 | Entity | 0xb0 | ty@Eagle.info.state@tag | r | L271 i8 ==1 Attack | 4 | OK |  |
| 34 | Entity | 0xb8 | ty@Eagle.info.state@Attack.target_id | r | L272 | 4 | OK |  |
| 35 | Entity | 0xc0 | ty@Eagle.info.state@Attack.time | r | L272 | 4 | OK |  |
| 36 | Entity | 0x490 | attack_effect (Option<Effect>) | r | 루프 e / champ(L459-461) · +0x4c0 casting tag == -1 이면 None(unwrap 패닉) · +0x4b0 start_timing | 4 | OK |  |
| 37 | Entity | 0x470 | stat_buff_cached.radius_mult | r | L297 · t.radius*(100+mult)/100 (0 이면 radius) | 4 | OK |  |
| 38 | Entity | 0x680 | radius | r | L297 is_in_orbit 반경 | 4 | OK |  |
| 39 | Entity | 0x660 | x | r | t 좌표 · 투사체 거리 | 4 | OK |  |
| 40 | Entity | 0x668 | y | r |  | 4 | OK |  |
| 41 | Projectile | 0x40 | move_type@tag | r | L298 ==6 Target(assume !=9) | 4 | OK |  |
| 42 | Projectile | 0x48 | move_type@Target.speed | r | L303 (0 → 패닉) | 4 | OK |  |
| 43 | Projectile | 0x50 | move_type@Target.target_id | r | L299 == t.id | 4 | OK |  |
| 44 | Projectile | 0xf8 | caster_id | r | L300/309 get_entity_by_id | 4 | OK |  |
| 45 | Projectile | 0x100 | x | r | L301 | 4 | OK |  |
| 46 | Projectile | 0x108 | y | r | L301 | 4 | OK |  |
| 47 | Projectile | 0x12c | applyed_target | r | L310 CastingTarget::check_projectile(&self, p, t) | 4 | OK |  |
| 48 | Blackboard | 0x78 | small_actions[pos] | r | L417 · stride 24 · +0 tag ∈{6 Attack,7 Skill,8 Skill2} · +8 target_id | 4 | OK |  |
| 49 | PlayerState(eplayer) | 0x9c0 | info.position@tag | r | L417 player_by_champion_id 결과의 포지션 | 4 | OK |  |
| 50 | StdRng(rnd) | 0x0 | rng 상태 전진 | w | ★순서(도달 조건부): roll_i64 ×4 (L19 value · L20 hp · L21 total · L22 start_timing · L23 cooltime = 5회, 항상) → [Minion·snapshot 경로] L72 gen_range(0..=1000) · L89 roll_i64 · L168/L173/L174/L184 gen_range(0..=1000) 중 1 → [Minion·레거시 경로] L319/L320 roll_i64 · L321 gen_range(acc..=1000) · L329 gen_range · L379/387/388/402 또는 L346/354/355/(L359: 1회) gen_range 중 1. roll_i64 는 jrng(Some=v3 래치 NoiseRng, 스택 %27) 가 Some 이면 rnd 를 소비하지 않는다(m04.ll:49855 계약). Champion/기타 경로는 5회 뒤 추가 소비 없음 | 4 | OK | gen_range::<usize,RangeInclusive> 직접 11지점 + range_misjudge_roll_i64 8지점 |
| 51 | stack | - | jrng Option<NoiseRng> 16B (%27) | w | roll_i64 가 &mut 로 전진시킴 — 지역 변수 | 4 | 확인불가(오프셋 파싱 실패) | range_misjudge_rng 반환 |
| 52 | stack | - | EntityIter 64B / ProjectileIter 40B | w | L211/L297/L457 · 지역 | 4 | 확인불가(오프셋 파싱 실패) | iter_entity/iter_projectile sret |

**`consts` 상수 31건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev | folded_from |
|---|---|---|---|---|---|---|
| 0 | 2 | 10 | 태그 | team bounds(<2) · EntityType::Tower 태그(L30/L420) · MinionActionType::Push(L161/178/335/368/393/462) · Tower.ty-3 <u 2(twin) | 4 |  |
| 1 | 1000 | 12 | 임계 | base = 1000 - last_hit_accuracy · 롤 정규화 /1000 · error_prob 상한 umin 1000 · gen_range 0..=1000 · L321 1000 - gen_range(acc..=1000) | 4 |  |
| 2 | 2000 | 13 | 계수 | max_v = 2000 - accuracy → roll_i64(acc ..= 2000-acc) 범위(‰) | 4 |  |
| 3 | 100 | 22 | 계수 | start_timing*100/speed_mult · cooltime*100/speed_mult · 루프 e 의 attack start_timing*100/attack_speed_mult · radius*(100+mult)/100 · Nexus 대상 미니언 적버프 반환 100(L46) · 챔피언 보너스 Nexus 100(L428) | 4 |  |
| 4 | 1 | 26 | 태그 | EntityType::Minion 태그(L26 switch · L458/459 루프) · Position::Jungle(L77/200) · Eagle state Attack(i8 1) · in_range_minion > 1 · umax(…,1) · ★L152 `predicted_hp ≤ 2*value` 의 2 가 `shl nsw i64 %58, 1` 로 접힘(folded_from 2) | 4 | 2 |
| 5 | 13 | 26 | 태그 | EntityType::Champion 태그 | 4 |  |
| 6 | 3 | 30 | 태그 | EntityType::Nexus 태그(L30/L420) · predicted_hp > 3*value(L159) · total*3/10(L333/365) · Tower.ty-3(twin 판정) · small_action tag-6 <u 3 | 4 |  |
| 7 | 30 | 32 | 계수 | 적 미니언이 타워를 조준 + 적 에픽버프 → 30(L32) · 30*tps 라인전 종료 여유(L704) · urgency 30(death_tick ≤ start+5, L100) · 타워 coef 30(처치 가능 시, L478) | 4 |  |
| 8 | 70 | 38 | 산출값 | 쌍둥이타워 조준 + 적버프 → 70(L38) · 넥서스 조준 무버프 → 70(L46) | 4 |  |
| 9 | 50 | 37 | 산출값 | 쌍둥이타워 조준 무버프 → 50 | 4 |  |
| 10 | 20 | 55 | 산출값 | 적 에픽버프 미니언(기타 조준) → 20 · 레거시 막타 경합(비라인전) → 20(L362) | 4 |  |
| 11 | 12 | 63 | 길이 | wave_snapshot.minions 배열 길이(bounds) · 검색 언롤 12회 | 4 |  |
| 12 | 5 | 93 | 태그 | killable 여유 value+5(L93/120/325) · death_tick > start+5(L100) · dist/speed+5(L303) · 반환 5(L155 Normal/Push) · multi_bonus ±5(L138) · TutorialType::MidBottom(5) | 4 |  |
| 13 | 999 | 93 | 임계 | roll*hp_at_tick > 999 ⟺ 예측 HP ≥ 1(생존) — /1000 전 값으로 비교 | 4 |  |
| 14 | 25 | 100 | 산출값 | urgency: death_tick > start+5 && death_tick ≤ cooltime+start → 25 | 4 |  |
| 15 | 15 | 100 | 계수 | urgency: 죽음이 쿨타임 뒤 → 15 · 타워 조준 시간 여유 +15(L233) · 레거시 막타 경합(라인전) → 15(L361) | 4 |  |
| 16 | -9999 | 72 | 산출값 | 기각 점수(오판 롤·Pull 스타일·비막타 타격 등 15개 지점) | 4 |  |
| 17 | 10 | 161 | 태그 | Push 스타일 기본 10 · Normal 오판 롤 10 · 챔피언 보너스 일반 타워 10(L424) · total*3/10 분모 | 4 |  |
| 18 | 29999 | 168 | 임계 | roll*cooltime_raw > 29999 ⟺ 쿨타임(롤 적용) ≥ 30틱 — 느린 공격이면 Normal 도 롤 통과 시 10 (L168/L378/L345) | 4 |  |
| 19 | 0 | 76 | 임계 | TutorialType::None(0) · MinionActionType::Pull(0) · GameMode Moba(0) · Position::Top 아님 · coef 0 | 4 |  |
| 20 | 7 | 76 | 태그 | TutorialType::Line(7) · EntityType::Ghoul(7, 루프) | 4 |  |
| 21 | 8 | 76 | 태그 | TutorialType::Total(8) · EntityType::SmallJiangshi(8) · SmallAction::Skill2(8) | 4 |  |
| 22 | 9 | 212 | 센티널 | EntityType::Bear(9) · ProjectileMoveType 니치 불가 태그 9(assume !=9) | 4 |  |
| 23 | 6 | 298 | 센티널 | ProjectileMoveType::Target 메모리 태그 6(idx4, 니치 start 2) · SmallAction::Attack(6) · tag-6 <u 3 → Attack/Skill/Skill2 | 4 |  |
| 24 | -1 | 215 | 센티널 | Option<Effect>(attack_effect) None 니치 = casting tag 0xFFFFFFFF · L516 hp == -1 && coef*value == i64::MIN → 오버플로 패닉 가드 | 4 |  |
| 25 | 80 | 424 | 산출값 | 챔피언 보너스: 적이 쌍둥이타워 공격 중 → 80 · 타워 coef(Normal/Pull, has_epic_buff) 80(L469) | 4 |  |
| 26 | 200 | 454 | 산출값 | Nexus coef 200 | 4 |  |
| 27 | 160 | 463 | 산출값 | 타워 coef: Push&has_epic_buff 160 / Normal&!has_epic_buff 160 | 4 |  |
| 28 | 240 | 463 | 산출값 | 타워 coef: Push & !has_epic_buff 240 | 4 |  |
| 29 | -9223372036854775808 | 516 | 태그 | i64::MIN — sdiv 오버플로 가드(coef*value == MIN && hp == -1) | 4 |  |
| 30 | 10 | 365 | 태그 | total*3/10 — 최대 HP 30% 여유 | 4 | 10 |

**`knobs` 조정점 8건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 막타 오판 확률 = min((1000-acc)²/1000, 1000) | action_score.rs:68 | 1000 | acc 가 낮을수록 제곱으로 오판(−9999) 증가 | 4 | 기존 |
| 1 | 막타 여유 폭 value+5 | action_score.rs:93/120/325 | 5 | 키우면 '지금 죽일 수 있음' 판정이 관대해져 더 일찍 막타 시도 | 4 | 기존 |
| 2 | urgency 30/25/15 | action_score.rs:100 | 30 | 막타 점수 바닥. 다른 행동(공격/스킬 점수)과의 상대 순위를 정한다 | 4 | 기존 |
| 3 | multi_bonus ±5 | action_score.rs:138 | 5 | 동시에 막타 가능한 미니언이 여럿일 때 가장 먼저 죽는 쪽 우선 | 4 | 기존 |
| 4 | 라인전 종료 = epic first_spawn - 30s | action_score.rs:76/199 (runner 703-704) | 30 | 이 시각 전엔 Normal 스타일이 비막타 타격을 거의 안 함(롤<err 만 10). 이후엔 반대로 대부분 10 | 4 | 기존 |
| 5 | 느린 공격 판정 cooltime ≥ 30틱 | action_score.rs:168/345/378 | 29999 | 쿨이 길면 라인전 중에도 Normal 이 비막타 타격을 롤로만 허용 | 4 | 기존 |
| 6 | 타워 coef 240/160/80 · Nexus 200 · 챔피언 보너스 100/80/10 | action_score.rs:424-428/454-469 | 240 | 구조물 공격 점수 상한. value/hp 비율이 낮으면 coef*value/hp 로 깎임 | 4 | 기존 |
| 7 | 적 에픽버프 미니언 고정 점수 20/30/50/70/100 | action_score.rs:32-55 | 20 | 버프 미니언·구조물 조준 미니언 우선 처치 | 4 | 기존 |

<details><summary>`callees` 피호출자 29건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | attack_cooltime | game_core::Entity::attack_cooltime | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1766 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | attack_speed_mult | game_core::Entity::attack_speed_mult | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:2433 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 2 | check_projectile | game_core::CastingTarget::check_projectile | pub | fn(&game_core::CastingTarget, &game_core::Projectile, &game_core::Entity) -> bool | game-core\src\simulation\effect\type.rs:248 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 3 | cooltime | game_core::Action::cooltime | pub | fn(&Self/#0, &game_core::Entity) -> usize | game-core\src\setting\action.rs:18 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 193개 중 상위 3개 |
| 4 | cooltime | <game_core::EmptyAction as game_core::Action>::cooltime | pub | fn(&game_core::EmptyAction, &game_core::Entity) -> usize | game-core\src\setting\action\common.rs:34 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 193개 중 상위 3개 |
| 5 | cooltime | <game_core::DataActionDef as game_core::Action>::cooltime | pub | fn(&game_core::DataActionDef, &game_core::Entity) -> usize | game-core\src\setting\champion\data_driven.rs:2267 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 193개 중 상위 3개 |
| 6 | distance | game_core::utils::distance | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:12 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 7 | expected_damage_target | game_core::Effect::expected_damage_target | pub | fn(&game_core::Effect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + , &game_core::Entity) -> usize | game-core\src\simulation\effect.rs:91 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 8 | expected_damage_target | game_core::Projectile::expected_damage_target | pub | fn(&game_core::Projectile, &game_core::GameContext, &game_core::Entity, &game_core::Entity) -> usize | game-core\src\simulation\projectile.rs:1287 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 9 | find | game_ai::MinionWaveSnapshot::find | pub | fn(&game_ai::MinionWaveSnapshot, usize) -> std::option::Option<&game_ai::MinionHpTrajectory> | game-ai\src\utils.rs:84 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 10 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 11 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 12 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 13 | hp_at_tick | game_ai::MinionHpTrajectory::hp_at_tick | pub | fn(&game_ai::MinionHpTrajectory, usize) -> i64 | game-ai\src\utils.rs:40 | True | True | 3 | IR 호출 심볼 일치(토큰 경계) |
| 14 | is_in_orbit | game_core::Projectile::is_in_orbit | pub | fn(&game_core::Projectile, u64, u64, u64) -> bool | game-core\src\simulation\projectile.rs:973 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 15 | is_in_range | game_core::Effect::is_in_range | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\effect.rs:63 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 16 | iter_entity | game_core::AbstractGame::iter_entity | pub | fn(&Self/#0) -> game_core::EntityIter | game-core\src\simulation.rs:180 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 17 | iter_entity | <game_core::Game as game_core::AbstractGame>::iter_entity | pub | fn(&game_core::Game) -> game_core::EntityIter | game-core\src\simulation\game.rs:1822 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 18 | iter_entity | <game_core::SingleLaneGame as game_core::AbstractGame>::iter_entity | pub | fn(&game_core::SingleLaneGame) -> game_core::EntityIter | game-core\src\simulation\game.rs:3886 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 19 | iter_projectile | game_core::AbstractGame::iter_projectile | pub | fn(&Self/#0) -> game_core::ProjectileIter | game-core\src\simulation.rs:182 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 20 | iter_projectile | <game_core::Game as game_core::AbstractGame>::iter_projectile | pub | fn(&game_core::Game) -> game_core::ProjectileIter | game-core\src\simulation\game.rs:3760 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 21 | iter_projectile | <game_core::SingleLaneGame as game_core::AbstractGame>::iter_projectile | pub | fn(&game_core::SingleLaneGame) -> game_core::ProjectileIter | game-core\src\simulation\game.rs:4019 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 22 | last_hit_accuracy | game_core::AthleteParameter::last_hit_accuracy | pub | fn(&game_core::AthleteParameter) -> usize | game-core\src\simulation\state\player.rs:246 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 23 | player_by_champion_id | game_core::AbstractGameWithCache::<'a, 'b>::player_by_champion_id | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> std::option::Option<&'a/#0 game_core::PlayerState> | game-core\src\simulation.rs:1915 | True | True | 3 | IR 호출 심볼 일치(토큰 경계) |
| 24 | range_misjudge_rng | game_ai::range_misjudge_rng | pub | fn(usize, &game_core::OperationData, &game_core::PlayerState, usize) -> std::option::Option<game_core::NoiseRng> | game-ai\src\utils.rs:502 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 25 | range_misjudge_roll_i64 | game_ai::range_misjudge_roll_i64 | pub | fn(&mut rand::rngs::std::StdRng, &mut std::option::Option<game_core::NoiseRng>, i64, i64) -> i64 | game-ai\src\utils.rs:525 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 26 | team | game_view::ClientData::team | pub | fn(&game_view::ClientData, usize) -> std::option::Option<std::cell::Ref< game_core::Team>> | game-view\src\logic\client\data.rs:1651 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 27 | team | game_view::ClientDatabase::team | pub | fn(&game_view::ClientDatabase, usize) -> std::option::Option<&game_core::Team> | game-view\src\logic\client\data.rs:968 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 28 | team | game_core::PlayerAiContext::<'a, 'b, 'r>::team | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:583 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
</details>

⚠**미매칭 5개**: `can_last_hit`, `flag`, `gen_range`, `has_epic_buff`, `next  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 9개는 **전부 다른 함수**라 싣지 않는다`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 19곳** (m02.ll:37476, m02.ll:37654, m02.ll:37865, m02.ll:38164, m02.ll:48691, m02.ll:48713, m02.ll:48746, m14.ll:21136, m14.ll:21165, m14.ll:21205, m14.ll:30177, m14.ll:30199, m14.ll:30232, m14.ll:49513, m14.ll:49535, m14.ll:49568, m15.ll:22781, m15.ll:22803, m15.ll:22836) · **형제 0개** 

**`open` 6건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 표기 불가 | L76/L199 라인전 플래그의 소스 표기: IR 은 (tutorial∈{0,5,7,8} → tick<경계 일 때만 position!=Jungle, 아니면 false) / (tutorial∉집합 → position!=Jungle) 로 확정. 두 `if` 의 소스 순서(같은 줄 안)는 표기 불가 | 4 |  |
| 1 | 미탐색 | DI 이름 `can_last_hit`(=predicted_hp > value+5) 와 `has_epic_buff`(=버프시간 0) 는 IR 극성과 이름이 어긋나 보임 — 명세는 IR 극성을 정본으로 적음. 소스 원문 부재라 변수명 오기인지 판정 불가 | 3 |  |
| 2 | 미탐색 | expected_damage_target(효과·투사체)·is_in_orbit·check_projectile·hp_at_tick·range_misjudge_rng 내부 미독(계약만) | 4 |  |
| 3 | 표기 불가 | %88(roll*cooltime_raw) > 29999 는 소스에선 `cooltime*roll/1000 ≥ 30` 류로 추정(표기 불가) — IR 은 ÷1000 전 값 비교로 확정 | 4 |  |
| 4 | 미탐색 | _debug(224B &mut) 는 readnone — 이 빌드에서 사용 0건. 다른 버전에서 디버그 기록에 쓰였을 가능성 | 4 |  |
| 5 | 미탐색 | gamemode=0 접기: 사장 블록 0(reach) — Moba 분기(get_game_mode tag 0)는 항상 산 코드 | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

