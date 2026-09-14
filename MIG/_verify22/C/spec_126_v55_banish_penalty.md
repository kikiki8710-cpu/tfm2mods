---

### `126` v55_banish_penalty — 추방(banish) 이펙트의 벌점 — 추방 초 동안 아군(자신 제외·150000 이내)이 대상 t 에 못 넣는 DPS 합을 t 최대HP 로 캡·hp_value 환산해 0..80

| 항목 | 값 |
|---|---|
| id | `buff_value__v55_banish_penalty` |
| 심볼 | `_RNvNtCshdEBA0ozCnw_7game_ai10buff_value18v55_banish_penalty` |
| 소스 | `game-ai\src\buff_value.rs:596` |
| IR | `m10.ll` 34297~34563행 |
| 경로·가시성 | `game_ai::buff_value::v55_banish_penalty` · **in:game_ai** |
| 계층 | 점수화·술어 |
| exe | `e02020` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r13` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &game_core::Effect, &game_core::OperationData, &game_core::PlayerState, &game_core::Entity, &game_core::Entity, i64) -> i64
```

<details><summary>인자 7개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | version | usize | 본문 분기 없음 — effect_cc_time(version, effect) 에 그대로 전달만 (m10.ll:34328) | 4 |
| 1 | 2 | effect | &Effect(56B) | noalias readonly captures(address,read_provenance). ty vtable 슬롯 has_banish_deep 호출 + effect_cc_time 인자 | 4 |
| 2 | 3 | data | &OperationData(24B) | noalias readonly captures(none). cache(+0x0)·context(+0x8) | 4 |
| 3 | 4 | player | &PlayerState(2528B) | noalias readonly captures(none). info.team(+0x930) 만 읽음 — 아군 순회 팀 | 4 |
| 4 | 5 | champ | &Entity(1728B) | noalias readonly captures(none). id(+0x5c0) 만 — 아군 순회에서 자기 자신 제외 | 4 |
| 5 | 6 | t | &Entity(1728B) | noalias readonly captures(none). 추방 대상. id·x·y·stat_cached.hp·hp 읽음 | 4 |
| 6 | 7 | hp_value | i64 | 대상 HP 1 당 가치 — lost 에 곱함 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn v55_banish_penalty(version, effect, data, player, champ, t, hp_value) -> i64  [buff_value.rs:596]
L598: if !effect.ty.has_banish_deep() { return 0 }   // vtable+0xc0, fn(&self)->bool
L601: let Some(banish_sec) = effect_cc_time(version, effect).map(|c| max(c / tps, 1)) else { return 0 }
      // effect_cc_time -> Option<usize> (ScalarPair {tag,val}: tag i64 bit0 = Some) · tps = data.context.setting.tick_per_second (0 → div_by_zero 패닉)
L604: let Some(tp) = data.cache.player_by_champion_id(t.id) else { return 0 }
L607: ti = tp.info.position as usize
L608: ally_dps = 0
L609: for ally in data.cache.iter_champions(player.info.team) {   // player_champion[team][0..5] filter_map, team bounds<2
L610:   if ally.id == champ.id || dist2(ally.(x,y), t.(x,y)) > 150000² { continue }   // 분기 순서: id 비교 먼저, 거짓일 때만 거리(abs_diff² 합)
L613:   let Some(ap) = data.cache.player_by_champion_id(ally.id) else { continue }
L616:   c = &data.cache.player_champion_cache[ap.info.team][ap.info.position as usize]   // team bounds<2
L617:   ally_dps += c.attack_per_sec[ti] + c.skill_per_sec[ti] + c.skill2_per_sec[ti] + c.ult_per_sec[ti]
      }
L619: lost = min(ally_dps * banish_sec, t.stat_cached.hp)   // umin
L620: v = ((lost as i64 * hp_value) / max(t.hp as i64, 1) / 2).min(80)
L621: return v
비고: 아군 순회에 champ 자신은 제외(자기 DPS 는 세지 않음) — 즉 '내가 추방하면 다른 아군이 t 를 못 때리는 손실'.
```

**`mem` 메모리 접근 21건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | Effect | 0x0 | ty.ptr(ArcInner*) | r | m10.ll:34313 · Arc data = ptr + ((align-1)&~15) + 16 | 4 | OK |
| 1 | Effect | 0x8 | ty.vtable | r | m10.ll:34314~34315 | 4 | 부분일치(첫 성분(컨테이너)만 맞고 **마지막 성분이 다르다** — 사람이 봐야 ) |
| 2 | vtable(EffectType) | 0x10 | align | r | m10.ll:34316~34317 | 4 | 확인불가(tcx 사전에 타입 없음) |
| 3 | vtable(EffectType) | 0xc0 | has_banish_deep | r | m10.ll:34322~34324 · 슬롯 24, fn(&self)->bool (divtable EffectType 0xc0 · tcx EffectType::has_banish_deep type.rs:327) | 3 | 확인불가(tcx 사전에 타입 없음) |
| 4 | OperationData | 0x8 | context | r | m10.ll:34343~34344 | 4 | OK |
| 5 | GameContext | 0x8 | setting | r | m10.ll:34345~34346 | 4 | OK |
| 6 | GameSetting | 0x12f8 | tick_per_second | r | m10.ll:34350~34351 · 0 이면 div_by_zero 패닉(34370) | 4 | OK |
| 7 | OperationData | 0x0 | cache | r | m10.ll:34361 | 4 | OK |
| 8 | Entity | 0x5c0 | id | r | t.id(34363~34364, player_by_champion_id) · champ.id(34401~34402, 자기 제외 비교) · ally.id(34453~34454, 비교+ap 조회) | 4 | OK |
| 9 | PlayerState | 0x9c0 | info.position@tag | r | tp(34376~34378 → ti) · ap(34530~34533 → 캐시 2차 인덱스) | 4 | OK |
| 10 | PlayerState | 0x930 | info.team | r | player(34381~34382, iter_champions 팀·bounds<2) · ap(34517~34519, 캐시 1차 인덱스·bounds<2) | 4 | OK |
| 11 | AbstractGameWithCache | 0x1e0 | player_champion[team][0..5] | r | m10.ll:34388~34396 · [[Option<&Entity>;5];2] 순회(iter_champions 인라인, filter_map call_mut 은 `\|x\| *x` — m09.ll:66150~66155) | 4 | OK |
| 12 | Entity | 0x660 | x | r | t(34403~34404) · ally(34482~34483) — abs_diff 거리 | 4 | OK |
| 13 | Entity | 0x668 | y | r | t(34405~34406) · ally(34486~34487) | 4 | OK |
| 14 | AbstractGameWithCache | 0x280 | player_champion_cache[ap.team][ap.position] | r | m10.ll:34407·34529~34535 | 4 | OK |
| 15 | ChampionCache | 0x190 | attack_per_sec[ti] | r | m10.ll:34537~34539 · ti = t 의 포지션 → 배열 인덱스 = 대상 포지션 | 4 | OK |
| 16 | ChampionCache | 0x1b8 | skill_per_sec[ti] | r | m10.ll:34540~34542 | 4 | OK |
| 17 | ChampionCache | 0x1e0 | skill2_per_sec[ti] | r | m10.ll:34543~34545 | 4 | OK |
| 18 | ChampionCache | 0x208 | ult_per_sec[ti] | r | m10.ll:34546~34548 | 4 | OK |
| 19 | Entity | 0x628 | stat_cached.hp | r | m10.ll:34462~34463 · t 최대HP — lost 상한(umin) | 4 | OK |
| 20 | Entity | 0x670 | hp | r | m10.ll:34469~34470 · t 현재HP — 분모 max(hp,1) | 4 | OK |

**`consts` 상수 7건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 1 | 601 | 임계 | banish_sec = max(cc_ticks / tps, 1) — 추방 초 하한 (llvm.umax, m10.ll:34359, closure#0 buff_value.rs:601 인라인) | 4 |
| 1 | 22500000000 | 610 | 임계 | 150000^2 — ally↔t 제곱거리 임계. dx²+dy² > 이 값이면 그 아군은 제외(continue) (m10.ll:34507) | 4 |
| 2 | 1 | 620 | 임계 | 분모 하한 max(t.hp,1) (llvm.smax, m10.ll:34473) | 4 |
| 3 | 2 | 620 | 임계 | sdiv 2 — 환산값 절반 (m10.ll:34475) | 4 |
| 4 | 80 | 620 | 임계 | 결과 상한 (llvm.smin, m10.ll:34478) | 4 |
| 5 | 0 | 598 | 태그 | 조기 반환 0 — has_banish_deep 거짓(L598) / effect_cc_time None(L601) / t 비플레이어(L604) (phi m10.ll:34339) | 4 |
| 6 | 40 | 609 | 태그 | player_champion[team] 5원소×8B 슬라이스 끝 오프셋(루프 종료 비교 icmp eq %70, 40 — m10.ll:34434). 배열 stride 이지 판정값 아님 | 4 |

**`knobs` 조정점 4건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 추방 초 하한 | buff_value.rs:601 | 1 | 올리면 짧은 추방도 N초분 손실로 계상 → 벌점 ↑ | 4 | 기존 |
| 1 | 아군 포함 거리 임계(제곱) | buff_value.rs:610 | 22500000000 | 올리면 먼 아군의 DPS 까지 손실에 포함 → 벌점 ↑ (150000 = 4.69셀) | 4 | 기존 |
| 2 | 환산 절반 계수 | buff_value.rs:620 | 2 | 내리면 벌점 2배 | 4 | 기존 |
| 3 | 결과 상한 | buff_value.rs:620 | 80 | 올리면 고DPS 팀의 장기 추방 벌점이 더 커짐 | 4 | 기존 |

<details><summary>`callees` 피호출자 8건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | effect_cc_time | game_ai::effect_cc_time | pub | fn(usize, &game_core::Effect) -> std::option::Option<usize> | game-ai\src\fight_check.rs:379 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | has_banish_deep | game_core::EffectType::has_banish_deep | pub | fn(&Self/#0) -> bool | game-core\src\simulation\effect\type.rs:327 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 2 | has_banish_deep | <game_core::BanishEffect as game_core::EffectType>::has_banish_deep | pub | fn(&game_core::BanishEffect) -> bool | game-core\src\simulation\effect\type\banish.rs:17 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 3 | has_banish_deep | <game_core::CombineEffect as game_core::EffectType>::has_banish_deep | pub | fn(&game_core::CombineEffect) -> bool | game-core\src\simulation\effect\type\combine.rs:84 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 4 | iter_champions | game_core::AbstractGameWithCache::<'a, 'b>::iter_champions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5759 ~ game_core[6a30]::simulation::{impl#5}::iter_champions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1904 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 5 | player_by_champion_id | game_core::AbstractGameWithCache::<'a, 'b>::player_by_champion_id | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> std::option::Option<&'a/#0 game_core::PlayerState> | game-core\src\simulation.rs:1915 | True | True | 3 | IR 호출 심볼 일치(토큰 경계) |
| 6 | tick_per_second | game_view::view::effect::nightmare::tick_per_second | in:game_view::view::effect::nightmare | fn(&engine_core::assets::Assets) -> f32 | game-view\src\view\effect\nightmare.rs:31 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 7 | v55_banish_penalty | game_ai::buff_value::v55_banish_penalty | in:game_ai | fn(usize, &game_core::Effect, &game_core::OperationData, &game_core::PlayerState, &game_core::Entity, &game_core::Entity, i64) -> i64 | game-ai\src\buff_value.rs:596 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
</details>

⚠**미매칭 5개**: `dist2`, `llvm.smax.i64`, `llvm.smin.i64`, `llvm.umax.i64`, `llvm.umin.i64`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m05.ll:42515) · **형제 0개** 

**`open` 4건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | effect_cc_time(m15.ll:25822, fight_check.rs:379) 내부(어느 CC 를 추방 틱으로 세는지)는 다른 함수 — 여기선 Option<usize> 계약만 | 4 |  |
| 1 | 미탐색 | has_banish_deep 구현체(CombineEffect/LinearProjectileEffect 는 자식 위임, 기본 impl 은 tcx type.rs:327)의 true 조건은 game_core 경계 밖 — 미탐색 | 3 |  |
| 2 | 표기 불가 | L610 `A \|\| B` 의 소스 표기 순서는 column 부재로 표기 불가 — IR 분기 순서(id → 거리)를 기록 | 4 |  |
| 3 | 미탐색 | 반환값을 호출자가 감산(벌점)으로 쓰는지 가산으로 쓰는지는 호출자(buff_value 상위, 호출자 1) 범위 — 이름(penalty)만으론 단정 안 함 | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

