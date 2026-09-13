---

### `55` EntityPositioningCache::new — champ↔e 쌍의 위치평가 캐시(424B) 생성 — 적이면 피해비율·사거리(제곱/확장/절반), 아군이면 힐·실드 비율·사거리

| 항목 | 값 |
|---|---|
| id | `score_parameter__new` |
| 심볼 | `_RNvMNtCshdEBA0ozCnw_7game_ai15score_parameterNtB2_22EntityPositioningCache3new` |
| 소스 | `game-ai\src\score_parameter.rs:136` |
| IR | `m07.ll` 5210~7048행 |
| 경로·가시성 | `game_ai::score_parameter::EntityPositioningCache::new` · **pub** |
| 계층 | 기타 |
| exe | `d815e0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r8` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &game_core::Entity, &game_core::Entity, &game_core::PlayerState, &game_core::PlayerState, &game_core::ChampionCache, &game_core::ChampionCache, bool) -> game_ai::score_parameter::EntityPositioningCache
```

<details><summary>인자 9개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | &mut EntityPositioningCache(424B) | 반환 구조체. 두 분기 각각 43 필드 전부 채움(아래 writes) | 4 |
| 1 | 1 | _version | usize | 미사용 | 4 |
| 2 | 2 | champ | &Entity(1728B) | 판단 주체(자기 챔프). '-ed'(attacked/skilled/…) 계열의 caster | 4 |
| 3 | 3 | e | &Entity(1728B) | 상대 엔티티. attack/skill/… 계열의 caster. team 비교로 적/아군 분기 | 4 |
| 4 | 4 | player | &PlayerState(2528B) | info.position(+0x9c0) 만 읽음 → e_cache 인덱스 | 4 |
| 5 | 5 | eplayer | &PlayerState(2528B) | info.position → champ_cache 인덱스, info.parameter(+0x180) → skill_hit_delay_min/max | 4 |
| 6 | 6 | champ_cache | &ChampionCache(800B) | champ 의 포지션별 피해/힐/실드 표. [eposition] 로 읽음 | 4 |
| 7 | 7 | e_cache | &ChampionCache(800B) | e 의 표. [position] 로 읽음 | 4 |
| 8 | 8 | has_near_enemy | bool | 아군 분기에서만: true 면 실드 값을 힐에 합산, false 면 실드=0 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn new(_version, champ, e, player, eplayer, champ_cache, e_cache, has_near_enemy) -> EntityPositioningCache

[L139] position  = player.info.position(+0x9c0)
[L140] eposition = eplayer.info.position
// 공용 인라인 헬퍼
//  max_range(eff, caster)  = eff.range(+0x10) + eff.growth_range(+0x18)*(caster.level-1) + caster.stat_buff_cached.range(+0x438)   [effect.rs:26]
//  radius(x)               = x.stat_buff_cached.radius_mult(+0x470)==0 ? x.radius(+0x680) : x.radius*(mult+100)/100          [entity.rs:1511~1515]
//  slot2(x) = x.level>2 ? &x.skill2_effect(+0x500) : &NONE ; slot3(x) = x.level>4 ? &x.ult_effect(+0x538) : &NONE ; NONE = casting(-1)
//  range_sqs(r) = (r*r, (r+32000)^2, (r>>1)^2)                                                                             [score_parameter.rs:128~131]
//  ratio(a_sec, a, hp) = min((a_sec>>1 + a)*100 / max(hp,1), 150)
//  linear(eff) = eff.is_some() && eff.ty.linear_move_speed()(vtable+0xf8).is_some()

[L141] if champ.team != e.team {            // ── 적 분기 (L142~238)
[L142]   attack_range = max_range(e.attack_effect.unwrap(), e) + range_adjust(&e.attack_effect, e, champ) + radius(e) + radius(champ) + 18000
[L144]   skill_range  = e.skill_effect.is_some() ? max_range(skill,e) + range_adjust(skill,e,champ) + radius(e) + radius(champ) + 18000 : 0
[L150]   skill2_range = slot2(e) 동형 (None→0)
[L156]   ult_range    = slot3(e) 동형 (None→0)
[L162]   attacked_range = max_range(champ.attack_effect.unwrap(), champ) + range_adjust(&champ.attack_effect, champ, e) + radius(champ) + radius(e)   // +18000 없음
[L164]   skilled_range  = champ.skill_effect 동형(18000 없음, None→0)
[L170]   skilled2_range = slot2(champ) 동형 ; [L176] ulted_range = slot3(champ) 동형
[L183]   (attack_range_sq, attack_range_ext_sq) = (attack_range^2, (attack_range+32000)^2)     // half 없음
[L184~186] (skill|skill2|ult)_range_{sq,ext_sq,half_sq} = range_sqs(..)
[L187]   (attacked_range_sq, attacked_range_ext_sq) ; [L188~190] (skilled|skilled2|ulted)_range_{sq,ext_sq,half_sq}
[L193]   attack_ratio = ratio(e_cache.attack_per_sec[position], e_cache.attack[position], champ.hp)
[L194]   skill_ratio  = ratio(e_cache.skill_per_sec[position],  e_cache.skill[position],  champ.hp)
[L195]   is_skill_linear_move  = linear(e.skill_effect)
[L200]   skill2_ratio = ratio(e_cache.skill2_per_sec[position], e_cache.skill2[position], champ.hp) ; [L201] is_skill2_linear_move = linear(slot2(e))
[L202]   ult_ratio    = ratio(e_cache.ult_per_sec[position],    e_cache.ult[position],    champ.hp) ; [L203] is_ult_linear_move = linear(slot3(e))
[L204]   attacked_ratio = ratio(champ_cache.attack_per_sec[eposition], champ_cache.attack[eposition], e.hp)
[L205~207] skilled_ratio / skilled2_ratio / ulted_ratio = champ_cache.(skill|skill2|ult)[eposition] 동형, 분모 e.hp
[L236]   hit_delay_min = eplayer.info.parameter.skill_hit_delay_min() ; [L237] hit_delay_max = ..max()
[L238]   dx2 = [0;7], dy2 = [0;7]
         return { 위 전부 }

} else {                                    // ── 아군 분기 (L242~355)
[L242]   skill_heal   = e_cache.skill_heal_sec[position]>>1  + e_cache.skill_heal[position]
[L243]   if has_near_enemy { [L244] skill_shield = e_cache.skill_shield_sec[position]>>1 + e_cache.skill_shield[position] } else { skill_shield = 0 }
[L248]   skill2_heal  = e_cache.skill2_heal_sec>>1 + skill2_heal ; [L250] skill2_shield = has_near_enemy ? skill2_shield_sec>>1 + skill2_shield : 0
[L255]   ult_heal     = ult_heal_sec>>1 + ult_heal            ; [L257] ult_shield    = has_near_enemy ? ult_shield_sec>>1 + ult_shield : 0
[L262~267] skill_healed/skill_shielded/skill2_healed/skill2_shielded/ult_healed/ult_shielded = champ_cache.*[eposition] 동형 — ★실드는 has_near_enemy 무관하게 항상
[L269]   skill_range  = e.skill_effect.is_some() ? max_range(skill,e) + range_adjust(skill,e,champ) + radius(e) + radius(champ) : 0     // 18000 없음
[L275]   skill2_range = slot2(e) 동형 ; [L281] ult_range = slot3(e) 동형
[L287]   skilled_range = champ.skill_effect 동형 ; [L293] skilled2_range = slot2(champ) ; [L299] ulted_range = slot3(champ)
[L306~311] 6 종 range_sqs
[L315]   skill_ratio  = min((skill_heal + skill_shield)*100 / max(champ.hp,1), 150)     ; [L316] is_skill_linear_move = linear(e.skill_effect)
[L317]   skill2_ratio = min((skill2_heal + skill2_shield)*100 / max(champ.hp,1), 150)   ; [L318] is_skill2_linear_move = linear(slot2(e))
[L319]   ult_ratio    = min((ult_heal + ult_shield)*100 / max(champ.hp,1), 150)         ; [L320] is_ult_linear_move = linear(slot3(e))
[L322]   skilled_ratio  = min((skill_healed + skill_shielded)*100 / max(e.hp,1), 150)
[L323]   skilled2_ratio = min((skill2_healed + skill2_shielded)*100 / max(e.hp,1), 150)
[L324]   ulted_ratio    = min((ult_healed + ult_shielded)*100 / max(e.hp,1), 150)
[L353]   hit_delay_min/max = eplayer.info.parameter.skill_hit_delay_min()/max()
[L355]   dx2 = dy2 = [0;7]
         return { attack_ratio: 0, attacked_ratio: 0, attack_range_sq: 0, attack_range_ext_sq: 0, attacked_range_sq: 0, attacked_range_ext_sq: 0, 나머지 위 값 }
}
```

**`mem` 메모리 접근 70건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x9c0 | info.position@tag (i32) | r | IR 2496. player→position, eplayer→eposition | 4 | OK |  |
| 1 | PlayerState | 0x180 | info.parameter (AthleteParameter 744B) | r | IR 384. eplayer 의 것 → skill_hit_delay_min/max(self) | 4 | OK |  |
| 2 | Entity | 0x0 | team@tag (TeamType) | r | champ·e 둘 다. 태그와 Player.0(+0x8) 모두 같아야 아군 분기(entity.rs:1127 PartialEq 인라인) | 4 | OK |  |
| 3 | Entity | 0x8 | team@Player.0 | r | IR 8 | 4 | OK |  |
| 4 | Entity | 0x4c0 | attack_effect@tag (CastingType 니치, -1=None) | r | IR 1216. e(적 분기 L142)·champ(L162) — None 이면 unwrap_failed 패닉 | 4 | OK |  |
| 5 | Entity | 0x490 | attack_effect@Some.0 (Effect 56B) | r | IR 1168. range_adjust 의 self | 4 | OK |  |
| 6 | Entity | 0x4a0 | attack_effect.range | r | IR 1184 | 4 | OK |  |
| 7 | Entity | 0x4a8 | attack_effect.growth_range | r | IR 1192 | 4 | OK |  |
| 8 | Entity | 0x4f8 | skill_effect@tag | r | IR 1272. -1 이면 skill_range=0 | 4 | OK |  |
| 9 | Entity | 0x4c8 | skill_effect (Effect) | r | IR 1224. +0x10 range(1240) · +0x18 growth_range(1248) · +0x0 ty.Arc ptr · +0x8 ty.vtable(1232) | 4 | OK |  |
| 10 | Entity | 0x500 | skill2_effect (Effect) | r | IR 1280. level > 2 일 때만, 아니면 정적 None(@anon..31). 태그 +0x30(=+48) | 4 | OK |  |
| 11 | Entity | 0x538 | ult_effect (Effect) | r | IR 1336. level > 4 일 때만. 태그 +0x30 | 4 | OK |  |
| 12 | Entity | 0x5c8 | level | r | IR 1480. growth_range*(level-1) 및 슬롯 개방 게이트 | 4 | OK |  |
| 13 | Entity | 0x438 | stat_buff_cached.range | r | IR 1080. caster 의 사거리 버프(Effect::max_range 인라인, effect.rs:26) | 4 | OK |  |
| 14 | Entity | 0x470 | stat_buff_cached.radius_mult (i32) | r | IR 1136. 0 이면 radius 그대로, 아니면 radius*(mult+100)/100 (entity.rs:1511~1515 인라인) | 4 | OK |  |
| 15 | Entity | 0x680 | radius | r | IR 1664. caster·target 양쪽 반경을 사거리에 더함 | 4 | OK |  |
| 16 | Entity | 0x670 | hp | r | IR 1648. ratio 분모 max(hp,1). 적 분기: champ.hp(attack~ult), e.hp(attacked~ulted) / 아군 분기: champ.hp(heal), e.hp(healed) | 4 | OK |  |
| 17 | Effect | 0x8 | ty.vtable | r | Arc<dyn EffectType> 팻포인터 vtable. +0xf8 슬롯 = linear_move_speed (divtable, g02.ll @anon..1131) | 3 | 부분일치(첫 성분(컨테이너)만 맞고 **마지막 성분이 다르다** — 사람이 봐야 ) |  |
| 18 | dyn EffectType vtable | 0xf8 | linear_move_speed() -> Option<usize> | r | IR 248. 반환 {i64,i64} 의 .0==1(Some) 이면 is_*_linear_move=true | 4 | 확인불가(vtable 슬롯) |  |
| 19 | ChampionCache | 0x0 | attack[pos] | r | IR 0+8p (e_cache[position]) | 4 | OK |  |
| 20 | ChampionCache | 0x190 | attack_per_sec[pos] | r | IR 400+8p | 4 | OK |  |
| 21 | ChampionCache | 0x28 | skill[pos] | r | IR 40+8p | 4 | OK |  |
| 22 | ChampionCache | 0x1b8 | skill_per_sec[pos] | r | IR 440+8p | 4 | OK |  |
| 23 | ChampionCache | 0x50 | skill2[pos] | r | IR 80+8p | 4 | OK |  |
| 24 | ChampionCache | 0x1e0 | skill2_per_sec[pos] | r | IR 480+8p | 4 | OK |  |
| 25 | ChampionCache | 0x78 | ult[pos] | r | IR 120+8p | 4 | OK |  |
| 26 | ChampionCache | 0x208 | ult_per_sec[pos] | r | IR 520+8p | 4 | OK |  |
| 27 | ChampionCache | 0xa0 | skill_heal[pos] | r | IR 160+8p (아군 분기) | 4 | OK |  |
| 28 | ChampionCache | 0x230 | skill_heal_sec[pos] | r | IR 560+8p | 4 | OK |  |
| 29 | ChampionCache | 0x118 | skill_shield[pos] | r | IR 280+8p | 4 | OK |  |
| 30 | ChampionCache | 0x280 | skill_shield_sec[pos] | r | IR 640+8p | 4 | OK |  |
| 31 | ChampionCache | 0xc8 | skill2_heal[pos] | r | IR 200+8p | 4 | OK |  |
| 32 | ChampionCache | 0x258 | skill2_heal_sec[pos] | r | IR 600+8p | 4 | OK |  |
| 33 | ChampionCache | 0x140 | skill2_shield[pos] | r | IR 320+8p | 4 | OK |  |
| 34 | ChampionCache | 0x2a8 | skill2_shield_sec[pos] | r | IR 680+8p | 4 | OK |  |
| 35 | ChampionCache | 0xf0 | ult_heal[pos] | r | IR 240+8p | 4 | OK |  |
| 36 | ChampionCache | 0x2d0 | ult_heal_sec[pos] | r | IR 720+8p | 4 | OK |  |
| 37 | ChampionCache | 0x168 | ult_shield[pos] | r | IR 360+8p | 4 | OK |  |
| 38 | ChampionCache | 0x2f8 | ult_shield_sec[pos] | r | IR 760+8p | 4 | OK |  |
| 39 | AthleteStat | 0x70 | skill_hit | r | IR 112 (AthleteParameter.stat 내부). skill_hit_delay_min/max 가 읽음(_gcbc g15.ll:126017/125998 — 범위 밖 관측) | 4 | OK |  |
| 40 | EntityPositioningCache | 0x0 | attack_ratio | w | L193 / L313 | 4 | OK | 적: min((e_cache.attack_per_sec[pos]/2 + e_cache.attack[pos])*100 / max(champ.hp,1), 150) / 아군: 0 |
| 41 | EntityPositioningCache | 0x8 | skill_ratio | w | L194 / L315 | 4 | OK | 적: min((skill_per_sec/2 + skill)*100/max(champ.hp,1),150) / 아군: min((skill_heal + skill_shield)*100/max(champ.hp,1),150) |
| 42 | EntityPositioningCache | 0x10 | skill2_ratio | w | L200 / L317 | 4 | OK | 적: skill2 동형 / 아군: (skill2_heal+skill2_shield) 동형 |
| 43 | EntityPositioningCache | 0x18 | ult_ratio | w | L202 / L319 | 4 | OK | 적: ult 동형 / 아군: (ult_heal+ult_shield) 동형 |
| 44 | EntityPositioningCache | 0x20 | attacked_ratio | w | L204 / L313 | 4 | OK | 적: min((champ_cache.attack_per_sec[epos]/2 + champ_cache.attack[epos])*100/max(e.hp,1),150) / 아군: 0 |
| 45 | EntityPositioningCache | 0x28 | skilled_ratio | w | L205 / L322 | 4 | OK | 적: champ_cache.skill 동형 / 아군: min((skill_healed + skill_shielded)*100/max(e.hp,1),150) — 아군 실드는 has_near_enemy 무관하게 항상 포함 |
| 46 | EntityPositioningCache | 0x30 | skilled2_ratio | w | L206 / L323 | 4 | OK | 적: skill2 동형 / 아군: (skill2_healed+skill2_shielded) 동형 |
| 47 | EntityPositioningCache | 0x38 | ulted_ratio | w | L207 / L324 | 4 | OK | 적: ult 동형 / 아군: (ult_healed+ult_shielded) 동형 |
| 48 | EntityPositioningCache | 0x40 | skill_range | w | L144~146 / L269~271 | 4 | OK | e.skill_effect 있으면 max_range(e)+range_adjust(eff,e,champ)+radius(e)+radius(champ) [+18000 적 분기만], 없으면 0 |
| 49 | EntityPositioningCache | 0x48 | skill2_range | w | L150~152 / L275~277 | 4 | OK | e.level>2 && skill2_effect 있으면 동형, 아니면 0 |
| 50 | EntityPositioningCache | 0x50 | ult_range | w | L156~158 / L281~283 | 4 | OK | e.level>4 && ult_effect 있으면 동형, 아니면 0 |
| 51 | EntityPositioningCache | 0x58 | skilled_range | w | L164~166 / L287~289 | 4 | OK | champ.skill_effect 기준 동형(+18000 없음, 양 분기) |
| 52 | EntityPositioningCache | 0x60 | skilled2_range | w | L170~172 / L293~295 | 4 | OK | champ.level>2 && skill2_effect |
| 53 | EntityPositioningCache | 0x68 | ulted_range | w | L176~178 / L299~301 | 4 | OK | champ.level>4 && ult_effect |
| 54 | EntityPositioningCache | 0x70 | attack_range_sq | w | L183 / memset | 4 | OK | 적: attack_range^2 (attack_range = e.attack_effect.max_range(e)+range_adjust+radius(e)+radius(champ)+18000) / 아군: 0 |
| 55 | EntityPositioningCache | 0x78 | attack_range_ext_sq | w | L183 | 4 | OK | 적: (attack_range+32000)^2 — IR 은 (attack_range-18000+50000)^2 로 접힘 / 아군: 0 |
| 56 | EntityPositioningCache | 0x80 | skill_range_sq / 0x88 ext_sq / 0x90 half_sq | w | L184 / L306. range_sqs 헬퍼(score_parameter.rs:128~131) 인라인 | 4 | OK | r^2 / (r+32000)^2 / (r>>1)^2 (r=skill_range) |
| 57 | EntityPositioningCache | 0x98 | skill2_range_sq / 0xa0 / 0xa8 | w | L185 / L307 | 4 | OK | skill2_range 기준 동형 |
| 58 | EntityPositioningCache | 0xb0 | ult_range_sq / 0xb8 / 0xc0 | w | L186 / L308 | 4 | OK | ult_range 기준 동형 |
| 59 | EntityPositioningCache | 0xc8 | attacked_range_sq / 0xd0 attacked_range_ext_sq | w | L187 / memset. half_sq 없음 | 4 | OK | 적: attacked_range^2 / (attacked_range+32000)^2 (attacked_range = champ.attack_effect.max_range(champ)+range_adjust(eff,champ,e)+radius(champ)+radius(e), 18000 없음) / 아군: 0 |
| 60 | EntityPositioningCache | 0xd8 | skilled_range_sq / 0xe0 / 0xe8 | w | L188 / L309 | 4 | OK | skilled_range 기준 |
| 61 | EntityPositioningCache | 0xf0 | skilled2_range_sq / 0xf8 / 0x100 | w | L189 / L310 | 4 | OK | skilled2_range 기준 |
| 62 | EntityPositioningCache | 0x108 | ulted_range_sq / 0x110 / 0x118 | w | L190 / L311 | 4 | OK | ulted_range 기준 |
| 63 | EntityPositioningCache | 0x120 | hit_delay_min | w | L236 / L353. _gcbc 본문: (min(skill_hit,100)*25+300)/100 | 4 | OK | AthleteParameter::skill_hit_delay_min(&eplayer.info.parameter) |
| 64 | EntityPositioningCache | 0x128 | hit_delay_max | w | L237 / L354. _gcbc 본문: (min(skill_hit,100)*60+500)/100 | 4 | OK | AthleteParameter::skill_hit_delay_max(&eplayer.info.parameter) |
| 65 | EntityPositioningCache | 0x130 | dx2[7] | w | L238 / L355. 양 분기 모두 0 으로 초기화 — 이 생성자에서는 채우지 않음 | 4 | OK | 전부 0 (memset 56B) |
| 66 | EntityPositioningCache | 0x168 | dy2[7] | w | 동상 | 4 | OK | 전부 0 (memset 56B) |
| 67 | EntityPositioningCache | 0x1a0 | is_skill_linear_move | w | L195 / L316 | 4 | OK | e.skill_effect.map(\|s\| s.ty.linear_move_speed().is_some()).unwrap_or(false) |
| 68 | EntityPositioningCache | 0x1a1 | is_skill2_linear_move | w | L201 / L318 | 4 | OK | (level>2 게이트 적용된) skill2_effect 동형 |
| 69 | EntityPositioningCache | 0x1a2 | is_ult_linear_move | w | L203 / L320 | 4 | OK | (level>4) ult_effect 동형 |

**`consts` 상수 14건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev | folded_from |
|---|---|---|---|---|---|---|
| 0 | 0 | 141 | 태그 | TeamType 태그 0 = Player. champ.team == e.team 판정: 태그 같고 (태그 0 이면 Player.0 도 같아야) → 아군 분기, 아니면 적 분기 | 4 |  |
| 1 | -1 | 142 | 센티널 | Option<Effect> 니치 None (Effect+0x30 casting == -1). attack_effect 는 None 이면 unwrap_failed 패닉, skill/skill2/ult 는 range 0 | 4 |  |
| 2 | 1 | 142 | 태그 | growth_range * (level - 1) — Effect::max_range(caster) 인라인(effect.rs:26) | 4 |  |
| 3 | 100 | 143 | 계수 | radius*(radius_mult+100)/100 — 반경 배율(entity.rs:1515). radius_mult==0 이면 곱셈 생략 | 4 |  |
| 4 | 18000 | 142 | 계수 | 적 분기 attack/skill/skill2/ult 사거리에만 +18000 여유(약 0.56셀). attacked~ulted(내 것)·아군 분기엔 없음 | 4 |  |
| 5 | 2 | 150 | 임계 | level > 2 여야 skill2_effect 슬롯 사용(entity.rs:1693 접근자). 아니면 정적 None → range 0·linear false | 4 |  |
| 6 | 4 | 156 | 임계 | level > 4 여야 ult_effect 슬롯 사용(entity.rs:1701) | 4 |  |
| 7 | 50000 | 183 | 계수 | attack_range_ext_sq = (attack_range - 18000 + 50000)^2 = (attack_range + 32000)^2 — 18000+32000 이 접힌 값 | 4 |  |
| 8 | 32000 | 130 | 계수 | range_ext_sq = (range + 32000)^2 — 한 셀 확장 사거리(range_sqs 헬퍼) | 4 |  |
| 9 | 1 | 131 | 태그 | range_half_sq = (range/2)^2 — `lshr i64 %r, 1`. per_sec/2 (L193 등) 도 같은 lshr 1 | 4 | 2 |
| 10 | 1 | 193 | 태그 | 분모 max(hp, 1) — 0 나눗셈 방지 | 4 |  |
| 11 | 100 | 193 | 계수 | ratio = 값*100/hp (백분율) | 4 |  |
| 12 | 150 | 193 | 임계 | 모든 ratio 상한 150(%). min(·,150) | 4 |  |
| 13 | 1 | 195 | 태그 | linear_move_speed() 반환 Option<usize> 의 Some 태그(=1). is_some() 인라인 | 4 |  |

**`knobs` 조정점 6건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 적 스킬 사거리 여유분 | score_parameter.rs:142·145·151·157 (m07.ll:5391 `add i64 %73, 18000` 외) | 18000 | 적의 attack/skill/skill2/ult 사거리를 18000(≈0.56셀) 부풀려 '맞을 수 있는 거리'를 보수적으로 잡는다. 올리면 더 멀리서부터 위험으로 보고, 내리면 실제 사거리에 가깝게 판단 | 4 | 기존 |
| 1 | 확장 사거리 폭 (ext_sq) | score_parameter.rs:130·183 (m07.ll:5751 `add i64 %87, 32000` / 5744 `add i64 %73, 50000`) | 32000 | range_ext_sq = (range+32000)^2 — 한 셀 바깥까지를 '곧 사거리에 들어올' 구간으로 본다 | 4 | 기존 |
| 2 | 피해/힐 비율 상한(%) | score_parameter.rs:193~207·315~324 (m07.ll:5835 `umin(…,150)` 외) | 150 | ratio 를 HP 의 150% 에서 자른다. 올리면 초과 화력이 점수에 더 반영 | 4 | 기존 |
| 3 | per_sec 반영 비율 | score_parameter.rs:193 등 (m07.ll:5829 `lshr i64 %338, 1`) | 2 | ratio 분자 = per_sec/2 + 단발값. 즉 지속 피해는 0.5초분만 합산. shift 를 바꾸면 지속 피해 비중이 바뀜 | 4 | 기존 |
| 4 | 아군 실드 합산 게이트 | score_parameter.rs:243~257 (m07.ll `br i1 %8` has_near_enemy) | 1 | has_near_enemy=true 일 때만 아군(e)의 실드가 skill/skill2/ult_ratio 에 합산된다. 내(champ) 실드(skilled_*)는 항상 합산 | 4 | 기존 |
| 5 | skill2/ult 슬롯 개방 레벨 | score_parameter.rs:150·156 → entity.rs:1693/1701 (m07.ll:5415 `ugt %38, 2` / 5465 `ugt %38, 4`) | 2 | level>2 에서 skill2, level>4 에서 ult 사거리·선형이동·ratio 를 계산. 저레벨에서는 0/false | 4 | 기존 |

<details><summary>`callees` 피호출자 13건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | linear_move_speed | game_core::EffectType::linear_move_speed | pub | fn(&Self/#0) -> std::option::Option<usize> | game-core\src\simulation\effect\type.rs:347 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 14개 중 상위 3개 |
| 1 | linear_move_speed | <game_core::RushEffect as game_core::EffectType>::linear_move_speed | pub | fn(&game_core::RushEffect) -> std::option::Option<usize> | game-core\src\simulation\effect\type\rush.rs:63 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 14개 중 상위 3개 |
| 2 | linear_move_speed | <game_core::MoveToEffect as game_core::EffectType>::linear_move_speed | pub | fn(&game_core::MoveToEffect) -> std::option::Option<usize> | game-core\src\simulation\effect\type\move_to.rs:70 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 14개 중 상위 3개 |
| 3 | max_range | game_ai::max_range | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-ai\src\plan_legacy\old\battle.rs:2234 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 4 | position | game_core::PlayerAiContext::<'a, 'b, 'r>::position | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> game_core::Position | game-core\src\mod_ai.rs:587 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 5 | position | game_core::transfer::TeamRosterPlanningContext::position | pub | fn(&game_core::transfer::TeamRosterPlanningContext, game_core::Position) -> std::option::Option<&game_core::transfer::PositionPlanningContext> | game-core\src\transfer\roster_context.rs:234 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 6 | radius | game_core::Entity::radius | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1509 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 7 | range | game_core::Effect::range | pub | fn(&game_core::Effect, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:25 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 8 | range_adjust | game_core::Effect::range_adjust | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:29 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 9 | skill2_effect | game_core::Entity::skill2_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1692 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 10 | skill_hit_delay_max | game_core::AthleteParameter::skill_hit_delay_max | pub | fn(&game_core::AthleteParameter) -> usize | game-core\src\simulation\state\player.rs:374 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 11 | skill_hit_delay_min | game_core::AthleteParameter::skill_hit_delay_min | pub | fn(&game_core::AthleteParameter) -> usize | game-core\src\simulation\state\player.rs:367 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 12 | ult_effect | game_core::Entity::ult_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1700 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
</details>

⚠**미매칭 6개**: `casting`, `linear`, `range_sqs`, `ratio`, `slot2`, `slot3`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 2곳** (m00.ll:77461, m07.ll:34704) · **형제 3개** (EntityPositioningCache)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::score_parameter::EntityPositioningCache as std::clone::Clone>::clone | pub | game-ai\src\score_parameter.rs:78 | True | fn(&game_ai::score_parameter::EntityPositioningCache) -> game_ai::score_parameter::EntityPositioningCache |
| 1 | game_ai::score_parameter::EntityPositioningCache::compute_range_sq | in:game_ai::score_parameter | game-ai\src\score_parameter.rs:128 | False | fn(u64) -> (u64, u64, u64) |
| 2 | game_ai::score_parameter::EntityPositioningCache::new | pub | game-ai\src\score_parameter.rs:136 | False | fn(usize, &game_core::Entity, &game_core::Entity, &game_core::PlayerState, &game_core::PlayerState, &game_core::ChampionCache, &game_core::ChampionCache, bool) -> game_ai::score_parameter::EntityPositioningCache |

**`open` 7건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | Effect::range_adjust(&Effect, caster, target) 내부는 안 봄(_gcbc). 적 분기 e 계열은 (eff, e, champ), champ 계열은 (eff, champ, e) 순으로 넘긴다는 것만 확인 | 4 |  |
| 1 | 미탐색 | ChampionCache 각 배열의 인덱스 의미 — e_cache[position(내 포지션)] / champ_cache[eposition(상대 포지션)] 로 읽는 것은 확정. 배열이 '대상 포지션별 기대값'이라는 해석은 인덱싱 패턴에서 추정 | 5 |  |
| 2 | 미탐색 | linear_move_speed 슬롯(0xf8)은 divtable 정적 vtable(DokkaebiUltExplosionEffect 기준 94% 일치)로 이름을 잡았다. 런타임 Arc<dyn EffectType> 이라 실제 구현체는 알 수 없음(슬롯 이름만 유효) | 3 |  |
| 3 | 미탐색 | AthleteParameter::skill_hit_delay_min/max 본문은 _gcbc g15.ll:126017/125998 에서 관측(범위 밖): min=(min(skill_hit,100)*25+300)/100 ∈[3,28], max=(min(skill_hit,100)*60+500)/100 ∈[5,65]. skill_hit = AthleteStat+0x70. 이 명세의 calls 에만 등록 | 4 |  |
| 4 | 미탐색 | dx2/dy2 는 이 생성자에서 항상 0 — 채우는 곳은 다른 함수(미탐색) | 4 |  |
| 5 | 표기 불가 | attack_range_ext_sq 의 50000 은 18000+32000 접힘으로 읽었다(IR 이 %73(+18000 전 값)에 50000 을 더함). 소스가 (attack_range+32000) 인지 (base+50000) 인지는 표기 불가(외연 동일) | 4 |  |
| 6 | 미탐색 | _version(p1) 미사용 | 4 |  |

<details><summary>`closed` 1건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

| # | 물음 | 닫은 근거 |
|---|---|---|
| 0 | 아군 분기의 healed/shielded(champ_cache 쪽, L262~267)는 has_near_enemy 게이트가 없다 — IR 로 확정(%601~%642 는 %8 과 무관하게 로드). 의도인지는 알 수 없음 | 본문에 해소 표기가 있다 |
</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

