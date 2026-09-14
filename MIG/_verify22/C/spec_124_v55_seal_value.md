---

### `124` v55_seal_value — 부분 봉인(평타/스킬 봉쇄) 이펙트의 가치 — 봉인 초 동안 대상 t 가 못 내는 DPS(캐시 평균)를 t 최대HP 로 캡·hp_value 로 환산해 0..80 점

| 항목 | 값 |
|---|---|
| id | `buff_value__v55_seal_value` |
| 심볼 | `_RNvNtCshdEBA0ozCnw_7game_ai10buff_value14v55_seal_value` |
| 소스 | `game-ai\src\buff_value.rs:516` |
| IR | `m10.ll` 33434~33861행 |
| 경로·가시성 | `game_ai::buff_value::v55_seal_value` · **in:game_ai** |
| 계층 | 점수화·술어 |
| exe | `e019d0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r13` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &game_core::Effect, &game_core::OperationData, &game_core::Entity, &game_core::Entity, i64) -> i64
```

<details><summary>인자 6개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | version | usize | 본문에서 미사용(dbg_value 만, 분기 0건 — m10.ll:33436) | 4 |
| 1 | 2 | effect | &Effect(56B) | noalias readonly captures(none). ty(Arc<dyn EffectType>) 의 data/vtable 만 읽음 | 4 |
| 2 | 3 | data | &OperationData(24B) | noalias readonly captures(none). cache(+0x0)·context(+0x8) 읽음 | 4 |
| 3 | 4 | champ | &Entity(1728B) | noalias readonly captures(address,read_provenance). 본문에서 직접 필드 읽기 없음 — `&dyn AbstractEntity`(vtable @anon.ff23c5838f81fa2a3acb125bb4b6e568.29, 88B) 로 expected_seal 에 전달만 | 4 |
| 4 | 5 | t | &Entity(1728B) | noalias readonly captures(none). 봉인 대상. id(+0x5c0)·stat_cached.hp(+0x628)·hp(+0x670) 읽음 | 4 |
| 5 | 6 | hp_value | i64 | 대상 HP 1 당 가치(호출자가 산정). erased 에 곱함 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn v55_seal_value(version, effect, data, champ, t, hp_value) -> i64  [buff_value.rs:516]
L519: seal = effect.ty.expected_seal(data.context, champ as &dyn AbstractEntity)   // vtable+0xc8 간접호출, sret 16B
      if seal is None (byte9==2) → L520 return 0
L522: let Some(tp) = data.cache.player_by_champion_id(t.id) else return 0   // t 가 플레이어 챔피언이 아니면 0
L525: c = &data.cache.player_champion_cache[tp.info.team][tp.info.position as usize]   // team<2 bounds check
L526: sec = max(seal.ticks / tps, 1)   // tps = data.context.setting.tick_per_second (0 이면 div_by_zero 패닉)
L527: stopped = 0
L528: if seal.blocks_attack { L529: stopped += sum(c.attack_per_sec[0..5]) / 5 }
L531: if seal.blocks_skill  { L532~534: stopped += (sum(c.skill_per_sec[0..5]) + sum(c.skill2_per_sec[0..5]) + sum(c.ult_per_sec[0..5])) / 5 }
L536: erased = min(stopped * sec, t.stat_cached.hp)   // usize, umin — 봉인 동안 사라지는 화력을 대상 최대HP 로 캡
L537: v = ((erased as i64 * hp_value) / max(t.hp as i64, 1) / 2).min(80)   // sdiv — hp_value 부호 따라 음수 가능
L538: return v
극성: blocks_attack/blocks_skill 은 각각 독립 if(둘 다 참이면 둘 다 가산). 조기반환 2경로 모두 0.
```

**`mem` 메모리 접근 18건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | Effect | 0x0 | ty.ptr(ArcInner*) | r | m10.ll:33451 · Arc<dyn EffectType> data 포인터. 데이터 = ptr + ((vtable.align-1)&~15) + 16 (ArcInner 헤더 뒤, L441<2127<2445<519 인라인) | 4 | OK |
| 1 | Effect | 0x8 | ty.vtable | r | m10.ll:33452~33453 | 4 | 부분일치(첫 성분(컨테이너)만 맞고 **마지막 성분이 다르다** — 사람이 봐야 ) |
| 2 | vtable(EffectType) | 0x10 | align | r | m10.ll:33454~33455 · ArcInner 데이터 오프셋 계산용 | 4 | 확인불가(tcx 사전에 타입 없음) |
| 3 | vtable(EffectType) | 0xc8 | expected_seal | r | m10.ll:33462~33464 · 슬롯 25 (divtable EffectType 0xc8 → expected_seal) | 3 | 확인불가(tcx 사전에 타입 없음) |
| 4 | OperationData | 0x8 | context | r | m10.ll:33460~33461 · &GameContext, expected_seal 인자 + tps 조회 | 4 | OK |
| 5 | OperationData | 0x0 | cache | r | m10.ll:33480 · &AbstractGameWithCache | 4 | OK |
| 6 | Entity | 0x5c0 | id | r | m10.ll:33481~33482 · t.id → player_by_champion_id | 4 | OK |
| 7 | PlayerState | 0x930 | info.team | r | m10.ll:33493~33494 · tp.info.team, bounds<2 검사(33495~33499) | 4 | OK |
| 8 | PlayerState | 0x9c0 | info.position@tag | r | m10.ll:33504~33506 · i32 태그 zext → 캐시 2차 인덱스(Position 0..4, 검사 없음) | 4 | OK |
| 9 | AbstractGameWithCache | 0x280 | player_champion_cache[team][position] | r | m10.ll:33507~33509 · [[ChampionCache;5];2], stride 4000/800 | 4 | OK |
| 10 | GameContext | 0x8 | setting | r | m10.ll:33511~33512 | 4 | OK |
| 11 | GameSetting | 0x12f8 | tick_per_second | r | m10.ll:33513~33514 · 0 이면 panic_const_div_by_zero(33528, Location buff_value.rs:526:13) | 4 | OK |
| 12 | ChampionCache | 0x190 | attack_per_sec[0..5] | r | m10.ll:33537~33608 · blocks_attack 일 때 5원소 합/5 (L529) | 4 | OK |
| 13 | ChampionCache | 0x1b8 | skill_per_sec[0..5] | r | m10.ll:33613~33674 · blocks_skill 일 때 합 (L532) | 4 | OK |
| 14 | ChampionCache | 0x1e0 | skill2_per_sec[0..5] | r | m10.ll:33681~33742 · blocks_skill 일 때 합 (L533) | 4 | OK |
| 15 | ChampionCache | 0x208 | ult_per_sec[0..5] | r | m10.ll:33749~33812 · blocks_skill 일 때 합 (L534) | 4 | OK |
| 16 | Entity | 0x628 | stat_cached.hp | r | m10.ll:33839~33840 · t 의 최대 HP(캐시 스탯) — erased 상한 (L536, umin) | 4 | OK |
| 17 | Entity | 0x670 | hp | r | m10.ll:33846~33847 · t 현재 HP — 분모 max(hp,1) (L537) | 4 | OK |

**`consts` 상수 7건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 2 | 519 | 센티널 | Option<SealProfile> None 니치 태그 — sret 16B 의 byte9(blocks_skill 자리)==2 면 None → return 0 (m10.ll:33465~33468). SealProfile 16B={ticks@0 usize, blocks_attack@8 bool, blocks_skill@9 bool}(tcxdict) | 3 |
| 1 | 1 | 526 | 임계 | sec = max(seal.ticks / tps, 1) — 봉인 초 하한 (llvm.umax, m10.ll:33522) | 4 |
| 2 | 5 | 529 | 계수 | ChampionCache *_per_sec[5] 5 슬롯 평균 분모(udiv 5, m10.ll:33608·33830) — 대상 5 포지션 상대 DPS 평균 | 4 |
| 3 | 1 | 537 | 임계 | 분모 하한 max(t.hp, 1) (llvm.smax, m10.ll:33851) | 4 |
| 4 | 2 | 537 | 임계 | sdiv 2 — 환산값 절반 (m10.ll:33852) | 4 |
| 5 | 80 | 537 | 임계 | 결과 상한 min(v, 80) (llvm.smin, m10.ll:33855) | 4 |
| 6 | 0 | 520 | 태그 | 조기 반환값 — 프로필 None(L520) / 대상이 플레이어 챔피언 아님(L522 let-else) 두 경로 (phi m10.ll:33859) | 4 |

**`knobs` 조정점 4건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 봉인 초 하한 | buff_value.rs:526 | 1 | 올리면 짧은 봉인도 최소 N초 분 화력정지로 계상 → 봉인 가치 ↑ | 4 | 기존 |
| 1 | DPS 평균 분모 | buff_value.rs:529/532 | 5 | 내리면(예 3) stopped 가 커져 봉인 가치 ↑ (5=대상 포지션 슬롯 수라 구조적 상수) | 4 | 기존 |
| 2 | 환산 절반 계수 | buff_value.rs:537 | 2 | 내리면(1) 봉인 가치 2배; 올리면 봉인 가치 ↓ | 4 | 기존 |
| 3 | 결과 상한 | buff_value.rs:537 | 80 | 올리면 고DPS 대상 장기 봉인이 더 큰 점수 — 상한 밖 구간은 hp_value 비례 | 4 | 기존 |

<details><summary>`callees` 피호출자 6건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | expected_seal | game_core::EffectType::expected_seal | pub | fn(&Self/#0, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + ) -> std::option::Option<game_core::SealProfile> | game-core\src\simulation\effect\type.rs:330 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 5개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 1 | expected_seal | <game_core::CombineEffect as game_core::EffectType>::expected_seal | pub | fn(&game_core::CombineEffect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + ) -> std::option::Option<game_core::SealProfile> | game-core\src\simulation\effect\type\combine.rs:88 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 5개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 2 | expected_seal | <game_core::BlockSkillEffect as game_core::EffectType>::expected_seal | pub | fn(&game_core::BlockSkillEffect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + ) -> std::option::Option<game_core::SealProfile> | game-core\src\simulation\effect\type\block_input.rs:66 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 5개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 3 | player_by_champion_id | game_core::AbstractGameWithCache::<'a, 'b>::player_by_champion_id | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> std::option::Option<&'a/#0 game_core::PlayerState> | game-core\src\simulation.rs:1915 | True | True | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | tick_per_second | game_view::view::effect::nightmare::tick_per_second | in:game_view::view::effect::nightmare | fn(&engine_core::assets::Assets) -> f32 | game-view\src\view\effect\nightmare.rs:31 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 5 | v55_seal_value | game_ai::buff_value::v55_seal_value | in:game_ai | fn(usize, &game_core::Effect, &game_core::OperationData, &game_core::Entity, &game_core::Entity, i64) -> i64 | game-ai\src\buff_value.rs:516 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
</details>

⚠**미매칭 4개**: `llvm.smax.i64`, `llvm.smin.i64`, `llvm.umax.i64`, `llvm.umin.i64`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m05.ll:42512) · **형제 0개** 

**`open` 4건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | version(인자1)은 dbg_value 만 있고 본문 분기 0건 — 호출자가 왜 넘기는지는 이 범위 밖(범위 한정: m10.ll:33434~33861) | 4 |  |
| 1 | 미탐색 | expected_seal 의 구현체별 SealProfile.ticks 산정(AP 스케일 반영, _docs game_core.txt:1252~1254)은 game_core 경계 밖 — 여기서는 (ticks, blocks_attack, blocks_skill) 계약만 사용 | 4 |  |
| 2 | 미탐색 | ChampionCache.*_per_sec[5] 의 인덱스 = 대상 포지션(자매 함수 v55_banish_penalty 가 `[ti = tp.info.position]` 로 색인, m10.ll:34537~34548) — 그래서 여기 /5 는 '상대 5포지션 평균 DPS'. 캐시 생성부 자체는 미탐색 | 4 |  |
| 3 | 표기 불가 | L522 의 let-else 인지 `?`/match 인지 표기 불가(외연 동일: None→0) | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

