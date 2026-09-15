---

### `211` fight_check::check_kill_die_tick_uncached — focus 챔피언이 지금 교전을 시작하면 몇 틱 뒤에 죽는가(die tick) — 적 챔피언·타워·기타 유닛의 nuke(선제 버스트)와 dps 를 판단력 노이즈 곱으로 합산해 (hp+부활보너스−nuke)*60/dps

| 항목 | 값 |
|---|---|
| id | `fight_check__check_kill_die_tick_uncached` |
| 심볼 | `_RNvNtCshdEBA0ozCnw_7game_ai11fight_check28check_kill_die_tick_uncached` |
| 소스 | `game-ai\src\fight_check.rs:970` |
| IR | `m15.ll` 31120~33053행 |
| 경로·가시성 | `game_ai::fight_check::check_kill_die_tick_uncached` · **in:game_ai::fight_check** |
| 계층 | 점수화·술어 |
| exe | `eba9b0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r17` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &mut rand::rngs::std::StdRng, &game_core::OperationData, &game_core::PlayerState, &game_core::Entity, bumpalo::collections::vec::Vec< &game_core::Entity>, bumpalo::collections::vec::Vec< &game_core::Entity>, &mut game_core::DebugFrameData) -> usize
```

**TLS 메모 절(`sig.tls` · 정정 경로 `/specs[211]/sig/tls/<키>`)**

없음 — 본문(31120~33053) 에 LocalKey/call_once 상수 참조 0건. TLS 메모는 이 함수 밖의 래퍼(check_kill_die_tick, fight_check.rs:917 · 이 배치 범위 밖)에 있을 것으로 보이나 미열람

<details><summary>인자 8개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | version | usize (i64 %0) | 본문 분기 없음. enemy_minion_wave_risk_dps_at 의 1번 인자로 그대로 전달만(m15.ll 32863) | 4 |
| 1 | 2 | _rnd | &mut StdRng | gen_range 호출 사이트 0 — 노이즈는 StdRng 가 아니라 결정론 NoiseRng(splitmix64) 로 뽑는다(아래 logic) | 4 |
| 2 | 3 | data | &OperationData (24B, ptr %1) | cache(+0)·context(+8) 읽음. blackboard(+0x10) 은 직접 안 읽음 | 4 |
| 3 | 4 | judger | &PlayerState (2528B, ptr %2) | 판정 주체 — info.id(+0x928) 는 노이즈 시드, info.parameter(+0x180) 는 judge_accuracy, info.team(+0x930) 은 is_visible 의 팀 | 4 |
| 4 | 5 | focus | &Entity (1728B, ptr %3) | 죽음 판정 대상 챔피언. undying(+0x488)·id(+0x5c0)·x/y(+0x660/+0x668)·hp(+0x670)·stat_cached.hp(+0x628)·effect_buffs(+0x2f0) 읽음 | 4 |
| 5 | 6 | enemy | bumpalo Vec<&Entity> (32B by-value, ptr %4 dead_on_return) | 레이아웃 ptr +0 / len +0x18. 적 챔피언 목록(각 원소마다 player_by_champion_id 로 PlayerState 를 찾고 없으면 unwrap 패닉) | 4 |
| 6 | 7 | towers | bumpalo Vec<&Entity> (32B by-value, ptr %5 dead_on_return) | 적 타워 목록. 각 tower.attack_effect(+0x490) 가 None 이면 unwrap 패닉 | 4 |
| 7 | 8 | _debug | &mut DebugFrameData | exe 6인자(argscan 스택2+레지스터4)와 IR 6인자 일치 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn check_kill_die_tick_uncached(version, _rnd, data, judger, focus, enemy: Vec<&Entity>, towers: Vec<&Entity>, _debug) -> usize   // fight_check.rs:970
 if focus.stat_buff_cached.undying { return i64::MAX }                                   L976
 player = cache.player_by_champion_id(focus.id).unwrap()   // focus 의 PlayerState        L979
 tps_raw = setting.tick_per_second ; tps = max(tps_raw, 1)                                L986
 bucket = game.tick() / (tps*2)   // 2초 버킷 (tps*2==0 이면 div_by_zero 패닉 경로)        L987
 rng = NoiseRng( (judger.info.id * 0x9E3779B97F4A7C15) ^ (focus.id << 24) ^ bucket )      L989~990
 ja = judge_accuracy(&judger.info.parameter)   // 100..=1000                               L994
 d = (1000 - ja) >> 1 ; noise() := rng.range_usize(1000 - d, 1000 + d)   // [lo,hi] 양끝 포함 · splitmix64 → hi64(z * (hi-lo+1)) + lo   L995
 enemy_dps = 0 ; enemy_nuke = 0                                                            L998
 cc(p) := cache.player_champion_cache[p.team][p.pos] ; fpos := player.pos
 for pchamp in enemy {                                                                     L1000
   p = cache.player_by_champion_id(pchamp.id).unwrap()                                     L1002
   nuke = 0
   if pchamp.can_attack() || pchamp.attack_cooldown() <= tps_raw { nuke = cc(p).attack[fpos] * noise() / 1000 }               L1005~1007  ←rng#1
   if pchamp.can_skill()  || pchamp.skill_cooldown()  <= tps_raw { nuke = max(nuke, cc(p).skill[fpos]  * noise() / 1000) }   L1010~1012  ←rng#2
   if pchamp.can_skill2() || pchamp.skill2_cooldown() <= tps_raw { nuke = max(nuke, cc(p).skill2[fpos] * noise() / 1000) }   L1015~1017  ←rng#3
   if pchamp.can_ult()    || pchamp.ult_cooldown()    <= tps_raw { nuke = max(nuke, cc(p).ult[fpos]    * noise() / 1000) }   L1021~1023  ←rng#4
     // *_cooldown() 은 Champion(ty 13) 이면 필드, 아니면 0 → 항상 통과
   if !pchamp.is_block_attack() { enemy_dps += cc(p).attack_per_sec[fpos] * noise() / 1000 }                                   L1026~1028  ←rng#5
   if !pchamp.is_block_skill() && !(pchamp.is_block_move_skill() && pchamp.skill_effect.map_or(false, |e| e.can_move()))
        { enemy_dps += cc(p).skill_per_sec[fpos] * noise() / 1000 }                                                               L1031~1033  ←rng#6
   if !pchamp.is_block_skill() && !(pchamp.is_block_move_skill() && pchamp.skill2_effect().map_or(false, |e| e.can_move()))
        { enemy_dps += cc(p).skill2_per_sec[fpos] * noise() / 1000 }      // skill2_effect() = level>2 ? Some(..) : None    L1036~1038  ←rng#7
   enemy_nuke += nuke                                                                      L1040
 }   // ult_per_sec 는 합산하지 않음
 for tower in towers {                                                                     L1049
   damage = tower.attack_effect.unwrap().expected_damage_target(ctx, tower, focus)          L1050
   enemy_dps  += (noise() * tps_raw * damage / 1000) / tower.attack_cooltime()   // cooltime 0 이면 div_by_zero 패닉   L1051~1053 ←rng#8
   enemy_nuke += damage * noise() / 1000                                                    L1054  ←rng#9
 }
 for e in cache.others[1 - player.team] {                                                  L1058
   if dist²(e, focus) > 22500000000 { continue }          // > 150,000                     L1059
   if !game.is_visible(judger.team, e.id) { continue }                                     L1063
   if let Some(atk) = e.attack_effect {                                                    L1066
     damage = atk.expected_damage_target(ctx, e, focus)                                    L1067
     enemy_dps  += damage * tps_raw / max(e.attack_cooltime(), 1)                          L1068
     enemy_nuke += damage                                   // 노이즈 없음                 L1069
   }
 }
 for pchamp in enemy { for b in pchamp.effect_buffs { enemy_dps += b.expected_aura_dps_at(ctx, pchamp, focus) } }          L1075~1077
 ally_heal = 0 ; for ally in cache.iter_champions(player.team) { for b in ally.effect_buffs { ally_heal += b.expected_aura_heal_at(ctx, ally, focus) } }   L1084~1087
 enemy_dps = enemy_dps.saturating_sub(ally_heal)                                           L1090
 no_enemy_epic := !(game.get_game_mode() is Moba(m)) || m.epic_minion_buff_time[1 - player.team] == 0   // DI 이름 enemy_epic_buff = 이 i1 (값은 '버프 없음' 쪽이 true)   L1095
 hp_over_75 := focus.hp*100 > max(focus.stat_cached.hp,1)*75   // DI 이름 low_enough_to_care_minions   L1096
 line_phase := if tutorial ∈ {None,MidBottom,Line,Total} { game.tick() < setting.epic_jungle.first_spawn_tick.saturating_sub(tps_raw*30) } else { true }   L1097
 if !(no_enemy_epic && (hp_over_75 || line_phase)) { enemy_dps += enemy_minion_wave_risk_dps_at(version, data, focus, focus.x, focus.y) }   L1098~1099  (분기 방향 그대로: no_enemy_epic 거짓이면 무조건 가산)
 revive_hp = Σ focus.effect_buffs.revive_bonus_hp(ctx, focus)                              L1104
 return ((revive_hp + focus.hp).saturating_sub(enemy_nuke) * 60) / max(enemy_dps, 1)       L1107
 // 드롭: enemy·towers Vec (bumpalo, L1110)
 // NoiseRng 추첨 순서(상태는 한 스트림): 적 챔피언마다 최대 7회(attack·skill·skill2·ult·attack_ps·skill_ps·skill2_ps, 각 조건부) → 타워마다 2회(dps, nuke) → others/버프 0회. 총 사이트 9곳. StdRng(_rnd) gen_range 0회.
```

**`mem` 메모리 접근 31건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | Entity(focus) | 0x488 | stat_buff_cached.undying | r | 1160. true 면 즉시 i64::MAX 반환(L976) | 4 | OK |
| 1 | Entity | 0x5c0 | id | r | 1472. focus.id → player_by_champion_id / 노이즈 시드(id<<24) · pchamp.id · e.id(is_visible) | 4 | OK |
| 2 | Entity | 0x68 | ty@tag (EntityType) | r | 104. 인라인된 Entity::attack_cooldown()/skill_cooldown()/skill2_cooldown()/ult_cooldown() 의 switch (13=Champion) | 4 | OK |
| 3 | Entity | 0xb0 | ty@Champion.0.attack_cooldown | r | 176(Champion·SmallJiangshi). 인라인 Entity::attack_cooldown()(entity.rs:1748~1760) 의 variant 별 오프셋 184 Minion · 272 Tower · 232 Jungle/Ghoul · 496 Epic/Serpen · 200 Bear · 240 Eagle · 216 Revenant · 208 Illusion — 전부 tcxdict Entity <off> 로 대조 일치(None/Nexus 는 0 으로 접혀 switch 에서 바로 통과). 값 > tick_per_second 이면 공격 nuke 제외 | 3 | OK |
| 4 | Entity | 0xb8 | ty@Champion.0.skill_cooldown | r | 184 (entity.rs:1776, Champion 아니면 0) | 4 | OK |
| 5 | Entity | 0xc0 | ty@Champion.0.skill2_cooldown | r | 192 (entity.rs:1791) | 4 | OK |
| 6 | Entity | 0xc8 | ty@Champion.0.ult_cooldown | r | 200 (entity.rs:1806) | 4 | OK |
| 7 | Entity | 0x4f8 | skill_effect@tag (i32, -1=None) | r | 1272 = skill_effect(0x4c8)+0x30. Some 이면 +0x4c8 Arc<dyn EffectType> 의 can_move(vtable+0x120) 호출 | 4 | OK |
| 8 | Entity | 0x5c8 | level | r | 1480. 인라인 Entity::skill2_effect(): level > 2 이면 &skill2_effect(0x500) 아니면 정적 None(@anon.58, entity.rs:1693) | 4 | OK |
| 9 | Entity | 0x500 | skill2_effect (Option<Effect>, 태그 +0x30 i32 -1=None) | r | 1280 (level>2 일 때만) | 4 | OK |
| 10 | Entity | 0x490 | attack_effect (Option<Effect>, 태그 +0x4c0 i32 -1=None) | r | 1168/1216. tower: None 이면 unwrap 패닉 / others: None 이면 건너뜀 | 4 | OK |
| 11 | Entity | 0x660 | x | r | 1632. others 거리² · minion_wave_risk 인자 | 4 | OK |
| 12 | Entity | 0x668 | y | r | 1640 | 4 | OK |
| 13 | Entity | 0x670 | hp | r | 1648. focus 현재 HP | 4 | OK |
| 14 | Entity | 0x628 | stat_cached.hp (최대 HP) | r | 1576. hp*100 > max(maxhp,1)*75 판정 | 4 | OK |
| 15 | Entity | 0x2f8 | effect_buffs.ptr / +0x300 len | r | 760/768 (Vec<Box<dyn EffectBuff>>, 원소 16B 팻포인터). pchamp(aura dps) · ally(aura heal) · focus(revive_bonus_hp) | 4 | OK |
| 16 | OperationData | 0x0 | cache (&AbstractGameWithCache) | r |  | 4 | OK |
| 17 | OperationData | 0x8 | context (&GameContext) | r |  | 4 | OK |
| 18 | GameContext | 0x8 | setting (&GameSetting) | r |  | 4 | OK |
| 19 | GameContext | 0x38 | tutorial (TutorialType 태그) | r | 56. {0 None,5 MidBottom,7 Line,8 Total} 이면 line_phase 계산, 그 외 line_phase=true | 4 | OK |
| 20 | GameSetting | 0x12f8 | tick_per_second | r | 4856 | 4 | OK |
| 21 | GameSetting | 0x8a8 | epic_jungle.first_spawn_tick | r | 2216. line_phase = tick < first_spawn_tick.saturating_sub(tps*30) | 4 | OK |
| 22 | AbstractGameWithCache | 0x0 | game (dyn AbstractGame data/vtable +0x8) | r | vtable +0x28 tick · +0xf8 is_visible · +0x40 get_game_mode | 4 | OK |
| 23 | AbstractGameWithCache | 0x280 | player_champion_cache[team][pos] (ChampionCache 800B) | r | 640. 필드 attack(+0)·skill(+0x28)·skill2(+0x50)·ult(+0x78)·attack_per_sec(+0x190)·skill_per_sec(+0x1b8)·skill2_per_sec(+0x1e0), 각 [focus_pos] | 4 | OK |
| 24 | AbstractGameWithCache | 0xf0 | others[1-focus_team] (Vec<&Entity>, ptr+0/len+0x18) | r | 240 | 4 | OK |
| 25 | AbstractGameWithCache | 0x1e0 | player_champion[focus_team][0..5] | r | 480. iter_champions 로 Some 만 | 4 | OK |
| 26 | PlayerState | 0x928 | info.id (judger) | r | 2344. 노이즈 시드 | 4 | OK |
| 27 | PlayerState | 0x180 | info.parameter (AthleteParameter, judger) | r | 384. judge_accuracy() 인자 | 4 | OK |
| 28 | PlayerState | 0x930 | info.team | r | 2352. focus player(others/iter_champions 인덱스, 1-t)·p(cache 인덱스)·judger(is_visible) | 4 | OK |
| 29 | PlayerState | 0x9c0 | info.position@tag (i32) | r | 2496. focus player 의 pos = ChampionCache 배열 인덱스 / p 의 pos = cache[team][pos] | 4 | OK |
| 30 | MobaMode | 0x240 | epic_minion_buff_time[1-focus_team] | r | 576. get_game_mode() 가 Moba 일 때만 (== 0 판정) | 4 | OK |

**`consts` 상수 21건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev | folded_from |
|---|---|---|---|---|---|---|
| 0 | 9223372036854775807 | 976 | 센티널 | i64::MAX — focus 가 undying 이면 반환하는 검열 die_tick(usize::MAX/2) | 4 |  |
| 1 | 1 | 987 | 태그 | bucket = tick / (tps*2) — `shl i64 %41, 1` 로 접힘(2초 단위 버킷). 별도로 1 은 umax(tps,1)·umax(cooltime,1)·umax(dps,1) 바닥값·`or 1` 에도 쓰임 | 4 | 2 |
| 2 | 24 | 990 | 계수 | focus.id << 24 (`shl i64 %15, 24`) — 노이즈 시드에 id 를 섞는 시프트량 | 4 |  |
| 3 | -7046029254386353131 | 989 | 계수 | 0x9E3779B97F4A7C15 (황금비) — ①시드 = judger.id * 이 값 ②NoiseRng::next_u64 의 상태 증가분(splitmix64, ai_interface.rs:182) | 4 |  |
| 4 | 4354685564936845354 | 1054 | 계수 | 2 × 0x9E3779B97F4A7C15 (mod 2^64) — 타워 루프에서 dps 추첨(L1052) 뒤 nuke 추첨(L1054) 상태 = 시작+2×증가분 으로 접힘 | 4 |  |
| 5 | -4658895280553007687 | 989 | 계수 | 0xBF58476D1CE4E5B9 — splitmix64 1차 믹스 곱(ai_interface.rs:184) | 4 |  |
| 6 | -7723592293110705685 | 989 | 계수 | 0x94D049BB133111EB — splitmix64 2차 믹스 곱(ai_interface.rs:185) | 4 |  |
| 7 | 30 | 989 | 계수 | ①splitmix64 lshr 30 (ai_interface.rs:184) ②tps*30 = 에픽 첫 스폰 30초 전(L1097, `mul i64 %33, 30`) | 4 |  |
| 8 | 27 | 989 | 계수 | splitmix64 lshr 27 (ai_interface.rs:185) | 4 |  |
| 9 | 31 | 989 | 계수 | splitmix64 lshr 31 (ai_interface.rs:186) | 4 |  |
| 10 | 1000 | 995 | 계수 | ①range_max = (1000 - judge_accuracy)/2 · range_min = 1000 - range_max (퍼밀) ②각 기여 = 값 * noise / 1000 | 4 |  |
| 11 | 13 | 1010 | 태그 | EntityType::Champion 태그(tcxdict --enum EntityType 13) — skill/skill2/ult 쿨다운은 Champion 일 때만 읽음 | 3 |  |
| 12 | 2 | 1006 | 임계 | team bounds(<2) · Entity::skill2_effect() 의 level > 2 게이트(entity.rs:1693) | 4 |  |
| 13 | -1 | 1031 | 태그 | Option<Effect> None 태그(i32) — attack/skill/skill2_effect 존재 검사 | 4 |  |
| 14 | 22500000000 | 1059 | 임계 | 150000² — others 유닛↔focus 거리² > 이 값(거리 > 150,000 = 4.69셀) 이면 제외 | 4 |  |
| 15 | 100 | 1096 | 계수 | focus.hp*100 > max(maxhp,1)*75 ⇔ HP > 75% | 4 |  |
| 16 | 75 | 1096 | 계수 | HP 75% 임계(DI 이름 low_enough_to_care_minions — 극성은 분기 방향으로 logic 참조) | 4 |  |
| 17 | 7 | 1097 | 태그 | TutorialType 태그 {0 None, 5 MidBottom, 7 Line, 8 Total} 이면 line_phase 를 실제 계산(runner.rs:263 인라인 헬퍼) | 4 |  |
| 18 | 8 | 1097 | 태그 | 위와 동일(Total) | 4 |  |
| 19 | 5 | 1097 | 길이 | 위와 동일(MidBottom). 5 는 [5 x ptr] 슬라이스 길이(iter_champions)에도 등장 — 접힘 아님, shl 무관 | 4 |  |
| 20 | 60 | 1107 | 계수 | die_tick = 남는HP*60/dps — 초→틱 환산이 tps 가 아니라 리터럴 60 | 4 |  |

**`knobs` 조정점 7건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 판단력 노이즈 폭 | fight_check.rs:995 | 1000 ± (1000-judge_accuracy)/2 | judge_accuracy 가 낮을수록 적 피해 추정이 ±폭 커짐(오판). 상수 1000/2 를 바꾸면 폭 스케일 변경 | 5 | 기존 |
| 1 | 쿨다운 허용 여유 | fight_check.rs:1005/1010/1015/1021 | tick_per_second (1초) | 쿨다운이 1초 이내면 곧 쓸 수 있다고 보고 nuke 에 포함. 올리면 더 긴 쿨다운도 위협으로 계산 | 4 | 기존 |
| 2 | others 유닛 고려 반경 | fight_check.rs:1059 | 22500000000 | 150,000(4.69셀) 안의 비챔피언 유닛만 dps/nuke 에 포함. 올리면 더 먼 정글/에픽도 위협에 포함 | 4 | 기존 |
| 3 | 미니언 웨이브 위험 제외 HP 임계 | fight_check.rs:1096 | 75 | 적 에픽버프 없음 && (HP>75% \|\| 라인전 단계) 이면 미니언 dps 미가산. 내리면 더 낮은 HP 에서도 미니언 무시 | 4 | 기존 |
| 4 | 라인전 단계 종료 시점 | fight_check.rs:1097 | first_spawn_tick - tps*30 | 에픽 첫 스폰 30초 전까지를 라인전으로 봄. 30 을 키우면 라인전이 더 일찍 끝나 미니언 dps 가산 구간 확대 | 4 | 기존 |
| 5 | die_tick 초→틱 환산 | fight_check.rs:1107 | 60 | 리터럴 60. tps 가 60 이 아닌 설정에서는 실제 틱과 어긋남 | 4 | 기존 |
| 6 | 노이즈 버킷 길이 | fight_check.rs:987 | tps*2 | 2초마다 노이즈 시드가 바뀜. 늘리면 같은 오판이 더 오래 유지 | 4 | 기존 |

<details><summary>`callees` 피호출자 40건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | attack_cooldown | game_core::Entity::attack_cooldown | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1747 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 1 | attack_cooltime | game_core::Entity::attack_cooltime | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1766 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 2 | can_attack | game_core::Entity::can_attack | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1527 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 3 | can_move | game_core::Entity::can_move | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1489 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 4개 중 상위 3개 |
| 4 | can_move | game_core::Champion::can_move | pub | fn(&game_core::Champion) -> bool | game-core\src\simulation\entity\champion.rs:48 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 4개 중 상위 3개 |
| 5 | can_move | game_core::EffectType::can_move | pub | fn(&Self/#0) -> bool | game-core\src\simulation\effect\type.rs:358 | True | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 4개 중 상위 3개 |
| 6 | can_skill | game_core::Entity::can_skill | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1708 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 7 | can_skill2 | game_core::Entity::can_skill2 | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1721 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 8 | can_ult | game_core::Entity::can_ult | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1734 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 9 | check_kill_die_tick_uncached | game_ai::fight_check::check_kill_die_tick_uncached | in:game_ai::fight_check | fn(usize, &mut rand::rngs::std::StdRng, &game_core::OperationData, &game_core::PlayerState, &game_core::Entity, bumpalo::collections::vec::Vec< &game_core::Entity>, bumpalo::collections::vec::Vec< &game_core::Entity>, &mut game_core::DebugFrameData) -> usize | game-ai\src\fight_check.rs:970 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 10 | enemy_minion_wave_risk_dps_at | game_ai::enemy_minion_wave_risk_dps_at | pub | fn(usize, &game_core::OperationData, &game_core::Entity, u64, u64) -> usize | game-ai\src\minion_wave_risk.rs:262 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 11 | expected_aura_dps_at | game_core::EffectBuff::expected_aura_dps_at | pub | fn(&Self/#0, &game_core::GameContext, &game_core::Entity, &game_core::Entity) -> usize | game-core\src\simulation\effect.rs:146 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 12 | expected_aura_dps_at | <game_core::WerewolfUltBuff as game_core::EffectBuff>::expected_aura_dps_at | pub | fn(&game_core::WerewolfUltBuff, &game_core::GameContext, &game_core::Entity, &game_core::Entity) -> usize | game-core\src\simulation\effect\type\werewolf_ult.rs:289 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 13 | expected_aura_dps_at | <game_core::ClownSkill2Buff as game_core::EffectBuff>::expected_aura_dps_at | pub | fn(&game_core::ClownSkill2Buff, &game_core::GameContext, &game_core::Entity, &game_core::Entity) -> usize | game-core\src\setting\champion\clown.rs:464 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 14 | expected_aura_heal_at | game_core::EffectBuff::expected_aura_heal_at | pub | fn(&Self/#0, &game_core::GameContext, &game_core::Entity, &game_core::Entity) -> usize | game-core\src\simulation\effect.rs:151 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 15 | expected_aura_heal_at | <game_core::GuardianSpiritSanctuaryBuff as game_core::EffectBuff>::expected_aura_heal_at | pub | fn(&game_core::GuardianSpiritSanctuaryBuff, &game_core::GameContext, &game_core::Entity, &game_core::Entity) -> usize | game-core\src\simulation\effect\type\guardian_spirit_sanctuary.rs:79 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 16 | expected_aura_heal_at | <game_core::setting::champion::priest::PriestUltEffectRunner as game_core::EffectBuff>::expected_aura_heal_at | pub | fn(&game_core::setting::champion::priest::PriestUltEffectRunner, &game_core::GameContext, &game_core::Entity, &game_core::Entity) -> usize | game-core\src\setting\champion\priest.rs:520 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 17 | expected_damage_target | game_core::Effect::expected_damage_target | pub | fn(&game_core::Effect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + , &game_core::Entity) -> usize | game-core\src\simulation\effect.rs:91 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 18 | get_game_mode | game_core::AbstractGame::get_game_mode | pub | fn(&Self/#0) -> game_core::GameMode | game-core\src\simulation.rs:78 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 19 | get_game_mode | <game_core::Game as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::Game) -> game_core::GameMode | game-core\src\simulation\game.rs:1707 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 20 | get_game_mode | <game_core::SingleLaneGame as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::SingleLaneGame) -> game_core::GameMode | game-core\src\simulation\game.rs:4248 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 21 | is_block_attack | game_core::Entity::is_block_attack | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1505 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 22 | is_block_move_skill | game_core::Entity::is_block_move_skill | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1523 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 23 | is_block_skill | game_core::Entity::is_block_skill | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1519 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 24 | is_visible | game_core::World::is_visible | pub | fn(&game_core::World, usize, usize) -> bool | game-core\src\simulation\game\data.rs:384 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 25 | is_visible | game_core::VisibleState::is_visible | pub | fn(&game_core::VisibleState) -> bool | game-core\src\simulation\game\data.rs:121 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 26 | is_visible | game_core::AbstractGame::is_visible | pub | fn(&Self/#0, usize, usize) -> bool | game-core\src\simulation.rs:125 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 27 | iter_champions | game_core::AbstractGameWithCache::<'a, 'b>::iter_champions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5759 ~ game_core[6a30]::simulation::{impl#5}::iter_champions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1904 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 28 | judge_accuracy | game_core::AthleteParameter::judge_accuracy | pub | fn(&game_core::AthleteParameter) -> usize | game-core\src\simulation\state\player.rs:337 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 29 | player_by_champion_id | game_core::AbstractGameWithCache::<'a, 'b>::player_by_champion_id | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> std::option::Option<&'a/#0 game_core::PlayerState> | game-core\src\simulation.rs:1915 | True | True | 3 | IR 호출 심볼 일치(토큰 경계) |
| 30 | range_usize | game_core::NoiseRng::range_usize | pub | fn(&mut game_core::NoiseRng, usize, usize) -> usize | game-core\src\simulation\ai_interface.rs:201 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 31 | revive_bonus_hp | game_core::EffectBuff::revive_bonus_hp | pub | fn(&Self/#0, &game_core::GameContext, &game_core::Entity) -> usize | game-core\src\simulation\effect.rs:155 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 32 | revive_bonus_hp | <game_core::GuardianSpiritReviveShieldBuff as game_core::EffectBuff>::revive_bonus_hp | pub | fn(&game_core::GuardianSpiritReviveShieldBuff, &game_core::GameContext, &game_core::Entity) -> usize | game-core\src\simulation\effect\type\guardian_spirit_sanctuary.rs:226 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 33 | skill2_cooldown | game_core::Entity::skill2_cooldown | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1789 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 34 | skill2_effect | game_core::Entity::skill2_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1692 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 35 | skill_cooldown | game_core::Entity::skill_cooldown | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1774 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 36 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 37 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 38 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 39 | ult_cooldown | game_core::Entity::ult_cooldown | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1804 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
</details>

⚠**미매칭 4개**: `drop  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `hi64`, `map_or`, `noise`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 2곳** (m15.ll:27399, m15.ll:27956) · **형제 0개** 

**`open` 7건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 재료 부재 | DI 변수명 enemy_epic_buff(%653)·low_enough_to_care_minions(%661)·line_phase 가 값의 극성과 반대로 보인다(enemy_epic_buff 가 buff_time==0 일 때 true). 소스 표기(`!` 위치)는 column 부재로 복원 불가 — logic 은 IR 분기 방향 기준으로 적었고 이름은 참고만 | 4 |  |
| 1 | 미탐색 | 판정 대상 PlayerState 를 focus 에서 찾는데(L979) 그 pos 가 ChampionCache 의 [fpos] 인덱스 = '이 적이 focus 포지션 상대로 기대하는 피해' 표 — ChampionCache 값 자체의 계산(game_core 캐시 갱신)은 이 배치에서 안 읽음 | 4 |  |
| 2 | 미탐색 | NoiseRng::range_usize 의 [lo,hi] 양끝 포함(hi-lo+1 = (1000-ja)\|1 로 접힘)은 _docs game_core 주석(`[lo, hi] 양끝 포함 균등`)과 IR 곱셈 피연산자로 확인했으나 range_usize 본체(ai_interface.rs:197~202)는 인라인이라 별도 define 미확인 | 4 |  |
| 3 | 재료 부재 | EffectType::can_move(vtable +0x120) 는 divtable 일치율 94% 의 CombineEffect vtable 기준 — 런타임 구현체별 의미는 미확인(Arc<dyn> 이라 정적 확정 불가) | 3 |  |
| 4 | 미탐색 | expected_damage_target 의 4번째 인자 @anon.56 = Entity 의 AbstractEntity vtable 상수(첫 슬롯 drop_glue<Entity>, size 0x6c0) 로 읽었으나 vtable 이름 태그는 미확인 | 4 |  |
| 5 | 미탐색 | 래퍼 check_kill_die_tick(fight_check.rs:917, TLS 캐시)·enemy_minion_wave_risk_dps_at 내부는 이 배치 범위 밖(시그니처만) | 4 |  |
| 6 | 미탐색 | ai_adjust judge DIFF=0 기록은 지시대로 열람 안 함 — 이 명세는 IR 단독 독해 | 1 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

