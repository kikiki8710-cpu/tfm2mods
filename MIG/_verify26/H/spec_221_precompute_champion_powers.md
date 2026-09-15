---

### `221` precompute_champion_powers — 챔피언 1명의 (attack_power, util_power_base, cc_time×inv_cd, buff_inv_cd_count) 4값을 ChampionCache 평균과 스킬 3종으로 계산해 &mut ChampionScoreParameter 에 쓰고 (seed,tick,team,pos,version) TLS 메모로 재사용

| 항목 | 값 |
|---|---|
| id | `utils__precompute_champion_powers` |
| 심볼 | `_RNvNtCshdEBA0ozCnw_7game_ai5utils26precompute_champion_powers` |
| 소스 | `game-ai\src\utils.rs:792` |
| IR | `m04.ll` 53234~53976행 |
| 경로·가시성 | `game_ai::precompute_champion_powers` · **pub** |
| 계층 | 기타 |
| exe | `d39ba0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r17` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &game_core::OperationData, &mut game_ai::ChampionScoreParameter)
```

**TLS 메모 절(`sig.tls` · 정정 경로 `/specs[221]/sig/tls/<키>`)**

- `name`: game_ai::utils::CHAMP_POWERS_MEMO (thread_local! RefCell<ChampPowersMemo> · ChampPowersMemo 496B utils.rs:778 · IR 접점 = @anon.168add0ea037d45d276f5936ae758fe5.235 = constant ptr @…CHAMP_POWERS_MEMO…call_once (m04.ll:248) 를 LocalKey::with 에 넘김)
- `role`: 작성자+소비자(이 함수가 유일한 접점 — closure#0 조회(L795) → miss 면 계산 → closure#4 저장(L858))
- `key`: 전역 키 = (seed: usize, tick: usize) 한 쌍(ChampPowersMemo+0x1e0 seed · +0x1e8 tick — RefCell 값이 borrow 플래그 8B 뒤에 있어 IR 오프셋은 +488/+496). 슬롯 키 = slot[team][pos] (team<2, pos<5 bounds check) + 슬롯 .0 == version
- `layout`: ChampPowersMemo = { slot: [[Option<(usize,i64,i64,i64,i64)>;5];2] (48B×10 = 0x0~0x1e0 · Option tag 8B Direct: 0=None 1=Some · .0=version +0x8 · .1=attack_power +0x10 · .2=util_power_base +0x18 · .3=cc_time_x_inv_cd +0x20 · .4=buff_inv_cd_count +0x28), seed u64 +0x1e0, tick usize +0x1e8 } (tcxdict --deep ChampPowersMemo)
- `invalidation`: closure#0 진입 시 memo.seed != seed || memo.tick != tick 이면 seed·tick 갱신 + 10슬롯 tag 전부 0(None) (m00.ll:78026~78046). 즉 tick 이 바뀌면 전 슬롯 무효. version 불일치는 무효화가 아니라 그 슬롯만 miss(다른 version 값이 그 자리에 덮어써진다)
- `call_conditions`: closure#0 = 무조건 1회(L795). closure#4 = 캐시 miss 경로에서만 1회(L858) — 저장 closure 는 memo.seed==seed && memo.tick==tick 일 때만 slot[team][pos] = Some((version, a, u, cc, b)) 를 쓰고 아니면 아무것도 안 한다(m00.ll:78141~78156). RefCell 이중 borrow 시 panic_already_borrowed · TLS 파괴 후 접근 시 panic_access_error

<details><summary>인자 3개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | version | usize (i64 %0) |  | 4 |
| 1 | 2 | data | &OperationData (24B, ptr %1) |  | 4 |
| 2 | 3 | p | &mut ChampionScoreParameter (216B, ptr %2) |  | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn precompute_champion_powers(version: usize, data: &OperationData, p: &mut ChampionScoreParameter)  // utils.rs:792
  let seed = data.cache.game.seed();   // L793 vtable+0x20
  let tick = data.cache.game.tick();   // L794 vtable+0x28
  // L795~803 closure#0 (m00.ll 77956~78120): CHAMP_POWERS_MEMO.with(|m| {
  //   let m = m.borrow_mut();                      // RefCell 플래그 0→-1 (이미 빌려져 있으면 panic_already_borrowed)
  //   if m.seed != seed || m.tick != tick { m.seed=seed; m.tick=tick; for s in m.slot.iter_mut() { *s = None } }  // 10슬롯 tag=0
  //   let s = m.slot[p.team][p.pos];               // bounds check 2·5
  //   if let Some((ver, a, u, cc, b)) = s && ver == version { Some((ver,a,u,cc,b)) } else { None } })
  let cached: Option<(usize,i64,i64,i64,i64)> = …;
  if let Some((_, a, u, cc, b)) = cached {          // L804 (tag == 1)
    prof::PHASE_CALLS[110] += 1 (if prof::ENABLED)  // L805
    p.attack_power = a; p.util_power_base = u; p.cc_time_x_inv_cd = cc; p.buff_inv_cd_count = b;  // L806~809
    return;                                        // L864
  }
  prof::PHASE_CALLS[111] += 1 (if prof::ENABLED)    // L813
  let cache = &data.cache.player_champion_cache[p.team][p.pos];   // L814 (0x280 + team*4000 + pos*800; bounds 2·5)
  // L818~821: atk_total = Σ_{i=0..5} (cache.attack_per_sec[i] + cache.skill_per_sec[i] + cache.skill2_per_sec[i] + cache.ult_per_sec[i])
  //           (fold 5회 전개; 각 항 wrapping add, 0x190/0x1b8/0x1e0/0x208)
  p.attack_power = atk_total / 5;                   // L821 udiv (usize)
  // L825~826: heal   = Σ_{i} (skill_heal_sec[i] + skill2_heal_sec[i] + ult_heal_sec[i])       (0x230/0x258/0x2d0)
  // L828~829: shield = Σ_{i} (skill_shield_sec[i] + skill2_shield_sec[i] + ult_shield_sec[i]) (0x280/0x2a8/0x2f8)
  p.util_power_base = heal/5 + shield/5;            // L830 udiv 둘 → add nuw nsw
  let mut cc_acc: i64 = 0; let mut buff_acc: i64 = 0;   // L833~834
  if let Some(champ) = data.cache.player_champion[p.team][p.pos] {   // L835 (0x1e0 + team*40 + pos*8, null=None)
    let slots = [ (champ.skill_effect(), champ.skill_cooltime()),      // L837: &champ.skill_effect(+0x4c8) 무조건
                  (champ.skill2_effect(), champ.skill2_cooltime()),    // L838: level(+0x5c8) > 2 ? &skill2_effect(+0x500) : &None
                  (champ.ult_effect(),   champ.ult_cooltime()) ];      // L839: level > 4 ? &ult_effect(+0x538) : &None
    //   ⚠3개 cooltime 은 slots 조립 시점에 전부 먼저 호출된다(효과 None 이어도 호출됨) — 순서 skill→skill2→ult
    for (eff_opt, cd) in slots {                     // L842 순서 skill, skill2, ult
      let Some(eff) = eff_opt else { continue };     // Option<Effect> 니치: eff+0x30(i32) == -1 → skip
      let cd_i = max(cd, 1) as i64;                  // L843 llvm.umax
      if let Some(cc_time) = fight_check::effect_cc_time(version, eff) {   // L844 (Option<usize>: {tag,val})
        cc_acc += (cc_time as i64 * 1000) / cd_i;    // L845 sdiv (오버플로 검사 → panic_const_div_overflow)
      }
      if fight_check::effect_buff_target(version, eff, data.context, champ as &dyn AbstractEntity, champ as &dyn AbstractEntity).is_some() {  // L847 sret 288B, +0x48 i32 != -1
        buff_acc += 1000 / cd_i;                     // L848 sdiv
      }
    }
  }
  p.cc_time_x_inv_cd = cc_acc;                      // L853
  p.buff_inv_cd_count = buff_acc;                   // L854
  let store = (version, p.attack_power, p.util_power_base, cc_acc, buff_acc);   // L856 (p 에서 재load)
  let (pt, pp) = (p.team, p.pos);                   // L857
  // L858~863 closure#4 (m00.ll 78123~78242): CHAMP_POWERS_MEMO.with(|m| { let m = m.borrow_mut(); if m.seed == seed && m.tick == tick { m.slot[pt][pp] = Some(store) } })
  // L864 return

주의: effect_cc_time / effect_buff_target 는 r13 잎(계약만): effect_cc_time(version: usize, effect: &Effect(56B)) -> Option<usize> ({i64 tag, i64 val}) · effect_buff_target(version, effect: &Effect, context: &GameContext(64B), caster: &dyn AbstractEntity(팻), target: &dyn AbstractEntity(팻)) -> Option<BuffState>(sret 288B, None 니치 = +0x48 i32 == -1). 이 함수는 caster=target=champ 로 부른다(vtable = Entity 의 AbstractEntity vtable @anon.6).
```

**`mem` 메모리 접근 33건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | OperationData | 0x0 | cache | r | &AbstractGameWithCache — L793·L814·L835 | 4 | OK |  |
| 1 | OperationData | 0x8 | context | r | &GameContext — effect_buff_target 3번째 인자(L847) | 4 | OK |  |
| 2 | AbstractGameWithCache | 0x0 | game.data_ptr | r | &dyn AbstractGame 데이터 포인터(L793) | 4 | OK |  |
| 3 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r | vtable+0x20=seed() (L793) · vtable+0x28=tick() (L794) — divtable AbstractGame | 3 | OK |  |
| 4 | AbstractGameWithCache | 0x1e0 | player_champion[team][pos] | r | [[Option<&Entity>;5];2] 니치(null=None) — L835 `if let Some(champ)` | 4 | OK |  |
| 5 | AbstractGameWithCache | 0x280 | player_champion_cache[team][pos] | r | [[ChampionCache(800B);5];2] — cache = &player_champion_cache[team][pos] (L814, team<2 · pos<5 bounds check) | 4 | OK |  |
| 6 | ChampionCache | 0x190 | attack_per_sec[0..5] | r | IR gep 400~432 (cache 기준) — L818 합산 | 4 | OK |  |
| 7 | ChampionCache | 0x1b8 | skill_per_sec[0..5] | r | IR gep 440~472 — L818 | 4 | OK |  |
| 8 | ChampionCache | 0x1e0 | skill2_per_sec[0..5] | r | IR gep 480~512 — L819 | 4 | OK |  |
| 9 | ChampionCache | 0x208 | ult_per_sec[0..5] | r | IR gep 520~552 — L819 | 4 | OK |  |
| 10 | ChampionCache | 0x230 | skill_heal_sec[0..5] | r | IR gep 560~592 — L825 heal | 4 | OK |  |
| 11 | ChampionCache | 0x258 | skill2_heal_sec[0..5] | r | IR gep 600~632 — L825 | 4 | OK |  |
| 12 | ChampionCache | 0x2d0 | ult_heal_sec[0..5] | r | IR gep 720~752 — L825 | 4 | OK |  |
| 13 | ChampionCache | 0x280 | skill_shield_sec[0..5] | r | IR gep 640~672 — L828 shield | 4 | OK |  |
| 14 | ChampionCache | 0x2a8 | skill2_shield_sec[0..5] | r | IR gep 680~712 — L828 | 4 | OK |  |
| 15 | ChampionCache | 0x2f8 | ult_shield_sec[0..5] | r | IR gep 760~792 — L828 | 4 | OK |  |
| 16 | ChampionScoreParameter | 0x60 | team | r | IR gep 96 — 메모 키·cache 인덱스 | 4 | OK |  |
| 17 | ChampionScoreParameter | 0x68 | pos | r | IR gep 104 | 4 | OK |  |
| 18 | Entity | 0x4c8 | skill_effect (Option<Effect> 56B) | r | IR gep 1224 — Entity::skill_effect() 인라인(entity.rs:1689<837). None 판정 = +0x30 casting@tag(i32, IR 1272) == -1 | 4 | OK |  |
| 19 | Entity | 0x500 | skill2_effect | r | IR gep 1280 — Entity::skill2_effect() 인라인(entity.rs:1693<838): level>2 면 &self.skill2_effect 아니면 &None 상수(@anon.16) | 4 | OK |  |
| 20 | Entity | 0x538 | ult_effect | r | IR gep 1336 — Entity::ult_effect() 인라인(entity.rs:1701<839): level>4 면 &self.ult_effect 아니면 &None | 4 | OK |  |
| 21 | Entity | 0x5c8 | level | r | IR gep 1480 — skill2/ult 해금 판정(ugt 2 / ugt 4) | 4 | OK |  |
| 22 | Option<BuffState>(effect_buff_target sret 288B) | 0x48 | duration@tag(BuffType i32) | r | IR gep 72 — == -1 이면 None(L847 `is_some` 판정) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 23 | static gc::simulation::prof::ENABLED | 0x0 | ENABLED(atomic i8) | r | L805·L813 프로파일 게이트 — 판정 무관 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 24 | static gc::simulation::prof::PHASE_CALLS | 0x370 | PHASE_CALLS[110] | r | IR gep 880 — 캐시 히트 카운터 atomicrmw add 1 (ENABLED 일 때만) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 25 | static gc::simulation::prof::PHASE_CALLS | 0x378 | PHASE_CALLS[111] | r | IR gep 888 — 캐시 미스 카운터 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 26 | ChampionScoreParameter | 0xb8 | attack_power | w | IR gep 184. 미스 경로에선 L856 에서 다시 load 해 메모에 저장 | 4 | OK | 히트: cached.1 (L806) / 미스: atk_total/5 (L821, udiv) |
| 27 | ChampionScoreParameter | 0xc0 | util_power_base | w | IR gep 192. 미스 경로 champ Some 이면 L856 에서 재load(%330), None 이면 계산값 그대로 | 4 | OK | 히트: cached.2 (L807) / 미스: heal/5 + shield/5 (L830, udiv 각각 후 add) |
| 28 | ChampionScoreParameter | 0xc8 | cc_time_x_inv_cd | w | IR gep 200 | 4 | OK | 히트: cached.3 (L808) / 미스: cc_acc (L853) |
| 29 | ChampionScoreParameter | 0xd0 | buff_inv_cd_count | w | IR gep 208 | 4 | OK | 히트: cached.4 (L809) / 미스: buff_acc (L854) |
| 30 | TLS ChampPowersMemo | 0x0 -> slot[team][pos] (48B stride, memo+0x0..0x1e0) |  | w | 미스 경로 closure#4. 저장 조건 memo.seed==seed && memo.tick==tick | 4 | 확인불가(tcx 사전에 타입 없음) | Some((version, attack_power, util_power_base, cc_acc, buff_acc)) — tag=1 store + 40B memcpy (m00.ll:78218~78222) |
| 31 | TLS ChampPowersMemo | 0x1e0 | seed / tick (+0x1e8) | w | RefCell borrow 플래그(+0) 도 -1→0 으로 왕복 | 4 | 확인불가(tcx 사전에 타입 없음) | 조회 closure#0 에서 (seed,tick) 불일치 시 갱신 + slot 10개 tag=0 (m00.ll:78026~78046, RefCell 값 기준 IR 오프셋 488/496 · 슬롯 tag 8,56,104,…,440) |
| 32 | static gc::simulation::prof::PHASE_CALLS | 0x370 | [110]/[111] (+0x378) | w | prof::ENABLED 일 때만 — 계측, 판정 무관 | 4 | 확인불가(tcx 사전에 타입 없음) | atomicrmw add 1 |

**`consts` 상수 9건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 2 | 814 | 임계 | team 상한(bounds check <2) · L838 level > 2 이면 skill2_effect 유효(Entity::skill2_effect 인라인) | 4 |
| 1 | 5 | 814 | 임계 | pos 상한(bounds check <5) · L818~830 ChampionCache 배열 원소 5개 합산(0..5 fold 가 5회 전개) · L821/L830 `/5` 평균(udiv) | 4 |
| 2 | 4 | 839 | 임계 | level > 4 이면 ult_effect 유효, 아니면 &None (Entity::ult_effect 인라인 entity.rs:1701) | 4 |
| 3 | -1 | 842 | 센티널 | Option<Effect> None 니치 = casting@tag(i32, Effect+0x30) == -1 → 그 슬롯 skip. L847 Option<BuffState> None 니치(+0x48 i32) == -1 도 동일. L845 sdiv 오버플로 검사(cd == -1)에도 등장 | 4 |
| 4 | 1 | 843 | 인덱스 | cd_i = max(cooltime, 1) (llvm.umax) — 0 나눗셈 방지. L804 cached Option tag==1(Some) · L858 저장 tag=1 도 이 값 | 4 |
| 5 | 1000 | 845 | 계수 | cc_acc += cc_time*1000 / cd_i (L845, sdiv) · buff_acc += 1000 / cd_i (L848). 쿨다운 역수를 1000 배 정수화 | 4 |
| 6 | 110 | 805 | 미상 | prof phase id(캐시 히트) — dbg_value 로만 남고 실제 코드는 PHASE_CALLS+880(=110*8) 로 접힘 | 4 |
| 7 | 111 | 813 | 미상 | prof phase id(캐시 미스) — PHASE_CALLS+888 로 접힘 | 4 |
| 8 | -9223372036854775808 | 845 | 태그 | i64::MIN — sdiv 오버플로 패닉 검사(cc_time*1000 == MIN && cd == -1) 컴파일러 삽입 | 4 |

**`knobs` 조정점 4건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 쿨다운 역수 스케일 | utils.rs:845 / :848 | 1000 | cc_time_x_inv_cd 와 buff_inv_cd_count 의 정수 스케일. 올리면 두 값이 비례해 커져 이를 소비하는 score_parameter 쪽 가중이 커진다(소비처는 이 명세 범위 밖) | 4 | 기존 |
| 1 | 스킬2/궁 해금 레벨 임계 | entity.rs:1693 / :1701 (Entity::skill2_effect / ult_effect 인라인, utils.rs:838~839) | 2 / 4 | level ≤2 면 skill2, ≤4 면 ult 가 cc/buff 누적에서 빠진다 | 4 | 기존 |
| 2 | 평균 분모 | utils.rs:821 / :830 | 5 | ChampionCache 5원소 배열의 평균. 원소 의미(레벨/구간?)는 미확인 — 바꾸면 attack_power·util_power_base 스케일이 바뀐다 | 4 | 기존 |
| 3 | TLS 메모 키 | utils.rs:795~803 (closure#0) | (seed, tick) 전역 + slot[team][pos].0 == version | 같은 tick 안에서 같은 (team,pos) 는 첫 계산값을 재생한다. 시뮬 중 p.team/p.pos 가 같고 version 이 같으면 champ 상태가 바뀌어도(레벨업·효과 변경) 그 tick 안에선 갱신 안 됨 | 4 | 기존 |

<details><summary>`callees` 피호출자 18건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | effect_buff_target | game_ai::effect_buff_target | pub | fn(usize, &game_core::Effect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + , &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + ) -> std::option::Option<game_core::BuffState> | game-ai\src\fight_check.rs:390 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | effect_cc_time | game_ai::effect_cc_time | pub | fn(usize, &game_core::Effect) -> std::option::Option<usize> | game-ai\src\fight_check.rs:379 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 2 | level | <game_ai::WindowStatView<'_> as game_core::AbstractEntity>::level | pub | fn(&game_ai::WindowStatView</#0>) -> usize | game-ai\src\fight_check.rs:96 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 3 | level | game_core::AbstractEntity::level | pub | fn(&Self/#0) -> usize | game-core\src\simulation\entity.rs:718 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 4 | level | <game_core::Entity as game_core::AbstractEntity>::level | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:772 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 5 | precompute_champion_powers | game_ai::precompute_champion_powers | pub | fn(usize, &game_core::OperationData, &mut game_ai::ChampionScoreParameter) | game-ai\src\utils.rs:792 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 6 | seed | game_core::AbstractGame::seed | pub | fn(&Self/#0) -> u64 | game-core\src\simulation.rs:72 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 7 | seed | <game_core::Game as game_core::AbstractGame>::seed | pub | fn(&game_core::Game) -> u64 | game-core\src\simulation\game.rs:1564 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 8 | seed | <game_core::SingleLaneGame as game_core::AbstractGame>::seed | pub | fn(&game_core::SingleLaneGame) -> u64 | game-core\src\simulation\game.rs:3781 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 9 | skill2_cooltime | game_core::Entity::skill2_cooltime | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1796 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 10 | skill2_effect | game_core::Entity::skill2_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1692 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 11 | skill_cooltime | game_core::Entity::skill_cooltime | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1781 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 12 | skill_effect | game_core::Entity::skill_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1688 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 13 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 14 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 15 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 16 | ult_cooltime | game_core::Entity::ult_cooltime | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1811 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 17 | ult_effect | game_core::Entity::ult_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1700 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
</details>

⚠**미매칭 5개**: `borrow_mut`, `llvm.umax.i64`, `panic_access_error`, `panic_already_borrowed`, `with  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 2개는 **전부 다른 함수**라 싣지 않는다`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 3곳** (m07.ll:38324, m07.ll:38664, m07.ll:38872) · **형제 0개** 

**`open` 7건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | ChampionCache 의 [usize;5] 배열 인덱스가 무엇을 뜻하는지(레벨 구간? 시간창?) — 이 함수는 5개를 단순 합·/5 하므로 판정엔 영향 없으나 의미는 game_core 쪽(simulation.rs:1252) 미독 | 4 |  |
| 1 | 미탐색 | closure 이름: fnparts 는 <precompute0>/<precomputes2_0> 로 찍고 tcx 는 {closure#0}(L795)/{closure#4}(L858) — 중간 closure#1~3 은 TLS 초기화(call_once)·fold 클로저로 추정(인라인, define 없음). 명세는 tcx 번호를 따름 | 3 |  |
| 2 | 미탐색 | effect_cc_time / effect_buff_target 내부 — r13 잎 명세 소관(계약만 기록) | 4 |  |
| 3 | 미탐색 | AbstractGame::seed/tick 의 런타임 구현체(vtable 슬롯 이름만 divtable 로 확정 — ExpectedGame 판 기준) | 3 |  |
| 4 | 미탐색 | L856 에서 util_power_base 를 p 에서 재load 하는 경로(%330)와 계산값(%220)이 phi 로 합쳐지는데 값은 동일 — 의미 차이 없음(컴파일러 아티팩트) | 4 |  |
| 5 | 미탐색 | version 별 분기 없음: 이 함수 안에서 version 은 메모 키와 콜리 인자로만 쓰인다(본문 icmp 에 version 없음 — 확인 완료, 미지 아님 · 기록용) | 4 |  |
| 6 | 미탐색 | calls 의 LocalKey::with 는 2회(closure#0 조회 · closure#4 저장)이나 qcspec leaf 매칭 규격상 한 항목으로 적음 — 인스턴스 2개는 aux 절 참조 | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

