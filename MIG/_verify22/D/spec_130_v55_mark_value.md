---

### `130` v55_mark_value — 표식(지연 폭발) 이펙트의 가치 — 창(window) 동안 아군이 t 에 넣을 피해/타수로 기대 폭발딜(base+per_hit·hits+accum%)을 만들어 hp_value 환산, 0..160

| 항목 | 값 |
|---|---|
| id | `buff_value__v55_mark_value` |
| 심볼 | `_RNvNtCshdEBA0ozCnw_7game_ai10buff_value14v55_mark_value` |
| 소스 | `game-ai\src\buff_value.rs:724` |
| IR | `m10.ll` 33063~33431행 |
| 경로·가시성 | `game_ai::buff_value::v55_mark_value` · **in:game_ai** |
| 계층 | 점수화·술어 |
| exe | `e01450` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r13` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &game_core::Effect, &game_core::OperationData, &game_core::PlayerState, &game_ai::ScoreParameter, &game_core::Entity, &game_core::Entity, i64, &mut game_core::DebugFrameData) -> i64
```

<details><summary>인자 9개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | version | usize | 본문 미사용(dbg_value 만, 분기·호출 전달 0건 — m10.ll:33070) | 4 |
| 1 | 2 | effect | &Effect(56B) | noalias readonly captures(none). ty vtable 슬롯 expected_mark 호출만 | 4 |
| 2 | 3 | data | &OperationData(24B) | noalias readonly captures(address,read_provenance). cache/context/context.debug/game.tick 읽음 + add_log 인자 | 4 |
| 3 | 4 | player | &PlayerState(2528B) | noalias readonly captures(address,read_provenance). info.team(+0x930)=아군 순회 팀 · info.position(+0x9c0)=디버그 로그 포맷 · add_log 인자 | 4 |
| 4 | 5 | _parameter | &ScoreParameter(5384B) | noalias readonly captures(none). 본문에서 전혀 안 읽음(이름도 `_` 접두) | 4 |
| 5 | 6 | champ | &Entity(1728B) | noalias readonly captures(address,read_provenance). 필드 읽기 없음 — `&dyn AbstractEntity`(vtable @anon.29) 로 expected_mark 에 전달만. ⚠banish 와 달리 아군 순회에서 자기 자신을 제외하지 않음(champ.id 비교 없음) | 4 |
| 6 | 7 | t | &Entity(1728B) | noalias readonly captures(none). 표식 대상. id·x·y·stat_cached.hp·hp | 4 |
| 7 | 8 | hp_value | i64 | 대상 HP 1 당 가치 — explosion 에 곱함 | 4 |
| 8 | 9 | _debug | &mut DebugFrameData(224B) | noalias align 8 dereferenceable(224), readonly 아님 = &mut. 직접 store 0건 — 유일한 쓰기 = DebugFrameData::add_log 호출(m10.ll:33353, 디버그 게이트 안) | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn v55_mark_value(version, effect, data, player, _parameter, champ, t, hp_value, _debug) -> i64  [buff_value.rs:724]
L728: let Some(mark) = effect.ty.expected_mark(data.context, champ as &dyn AbstractEntity) else { L729 return 0 }   // vtable+0xa8, MarkProfile{base,accum_pct,per_hit,max_hits,window}
L731: let Some(tp) = data.cache.player_by_champion_id(t.id) else { return 0 }
L734: tps = data.context.setting.tick_per_second
L735: window_sec = max(mark.window / tps, 1)   // tps==0 → 패닉
L736: ti = tp.info.position as usize
L737: accum_dps = 0;  L738: hits_per_sec_x100 = 0
L739: for ally in data.cache.iter_champions(player.info.team) {   // ⚠champ 자신 포함(제외 비교 없음)
L740:   if dist2(ally.(x,y), t.(x,y)) > 150000² { continue }
L743:   let Some(ap) = data.cache.player_by_champion_id(ally.id) else { continue }
L746:   c = &data.cache.player_champion_cache[ap.info.team][ap.info.position as usize]
L747:   accum_dps += c.attack_per_sec[ti] + c.skill_per_sec[ti] + c.skill2_per_sec[ti] + c.ult_per_sec[ti]
L748:   hits_per_sec_x100 += tps*100 / max(ally.attack_cooltime(), 1)
      }
L750: max_hp = t.stat_cached.hp
L751: expected_accum = min(accum_dps * window_sec, max_hp*2)
L752: expected_hits = hits_per_sec_x100 * window_sec / 100
L753: if mark.max_hits > 0 { expected_hits = min(expected_hits, mark.max_hits) }
L756: explosion = mark.base + expected_hits*mark.per_hit + expected_accum*mark.accum_pct/100
L757: explosion = min(explosion, max_hp*2)
L758: v = ((explosion as i64 * hp_value) / max(t.hp as i64, 1)).min(160)
L760: if v > 0 && data.context.debug && data.cache.game.tick() % 6 == 0 {   // 분기 순서: v>0 → debug → tick%6
L761:   _debug.add_log(data, player, format!("\nMARK_WIN T{} {:?} target_pos={:?} v={}", player.info.team, player.info.position, tp.info.position, v)) }
L764: return v
```

**`mem` 메모리 접근 26건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | Effect | 0x0 | ty.ptr(ArcInner*) | r | m10.ll:33087 · data = ptr + ((align-1)&~15) + 16 | 4 | OK |  |
| 1 | Effect | 0x8 | ty.vtable | r | m10.ll:33088~33089 | 4 | 부분일치(첫 성분(컨테이너)만 맞고 **마지막 성분이 다르다** — 사람이 봐야 ) |  |
| 2 | vtable(EffectType) | 0x10 | align | r | m10.ll:33090~33091 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 3 | vtable(EffectType) | 0xa8 | expected_mark | r | m10.ll:33098~33100 · 슬롯 21 (divtable EffectType 0xa8 → expected_mark), sret 48B = Option<MarkProfile>: tag i64@0(1=Some,0=None) · base@8 · accum_pct@16 · per_hit@24 · max_hits@32 · window@40 (dbg fragment 33108~33120 + tcxdict MarkProfile 40B) | 3 | 확인불가(tcx 사전에 타입 없음) |  |
| 4 | OperationData | 0x8 | context | r | m10.ll:33096~33097 | 4 | OK |  |
| 5 | OperationData | 0x0 | cache | r | m10.ll:33122 | 4 | OK |  |
| 6 | Entity | 0x5c0 | id | r | t.id(33124~33125 → player_by_champion_id) · ally.id(33362~33364) | 4 | OK |  |
| 7 | GameContext | 0x8 | setting | r | m10.ll:33136~33137 | 4 | OK |  |
| 8 | GameSetting | 0x12f8 | tick_per_second | r | m10.ll:33138~33139 · tps. 0 이면 div_by_zero 패닉(33164, Location buff_value.rs:735) | 4 | OK |  |
| 9 | PlayerState | 0x9c0 | info.position@tag | r | tp(33151~33153 → ti) · ap(33383~33386 캐시 인덱스) · player(33324, 로그 포맷 Debug) | 4 | OK |  |
| 10 | PlayerState | 0x930 | info.team | r | player(33157~33158 iter_champions 팀, bounds<2 · 로그 포맷 Display) · ap(33370~33372 캐시 1차 인덱스, bounds<2) | 4 | OK |  |
| 11 | AbstractGameWithCache | 0x1e0 | player_champion[team][0..5] | r | m10.ll:33168~33176 · iter_champions 인라인, filter_map call_mut = `\|x\| *x`(m09.ll:66150) | 4 | OK |  |
| 12 | Entity | 0x660 | x | r | t(33181~33182) · ally(33229~33230) | 4 | OK |  |
| 13 | Entity | 0x668 | y | r | t(33183~33184) · ally(33233~33234) | 4 | OK |  |
| 14 | AbstractGameWithCache | 0x280 | player_champion_cache[ap.team][ap.position] | r | m10.ll:33185·33382~33388 | 4 | OK |  |
| 15 | ChampionCache | 0x190 | attack_per_sec[ti] | r | m10.ll:33390~33392 · ti = t 포지션 | 4 | OK |  |
| 16 | ChampionCache | 0x1b8 | skill_per_sec[ti] | r | m10.ll:33393~33395 | 4 | OK |  |
| 17 | ChampionCache | 0x1e0 | skill2_per_sec[ti] | r | m10.ll:33396~33398 | 4 | OK |  |
| 18 | ChampionCache | 0x208 | ult_per_sec[ti] | r | m10.ll:33399~33401 | 4 | OK |  |
| 19 | Entity | 0x628 | stat_cached.hp | r | m10.ll:33260~33261 · max_hp (L750) — expected_accum·explosion 상한 = max_hp*2 | 4 | OK |  |
| 20 | Entity | 0x670 | hp | r | m10.ll:33287~33288 · 분모 max(hp,1) | 4 | OK |  |
| 21 | GameContext | 0x3b | debug | r | m10.ll:33306~33308 · 디버그 로그 게이트(L760) | 4 | OK |  |
| 22 | AbstractGameWithCache | 0x0 | game(&dyn AbstractGame).data_ptr | r | m10.ll:33312 | 4 | OK |  |
| 23 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r | m10.ll:33313~33314 | 4 | OK |  |
| 24 | vtable(AbstractGame) | 0x28 | tick | r | m10.ll:33315~33317 · 슬롯 5 fn(&self)->usize (divtable AbstractGame 0x28 → tick) | 3 | 확인불가(tcx 사전에 타입 없음) |  |
| 25 | DebugFrameData | 0x48 | logs(Vec<DebugLog>) — push | w | ★직접 store 0건. 유일한 &mut 부작용 = game_core::DebugFrameData::add_log(_debug, data, player, String) 호출(m10.ll:33353) — add_log 본문(_gcbc g13.ll:122046)이 logs Vec(len@+0x58 +1, ptr@+0x50 버퍼에 56B DebugLog 기록, 필요시 grow 로 cap@+0x48·ptr 갱신). 발화 조건 = v>0 && data.context.debug && game.tick()%6==0 (L760). 디버그 OFF 프로덕션에선 쓰기 0 | 4 | OK | DebugLog{ …, text: format!("\nMARK_WIN T{team} {player.pos:?} target_pos={tp.pos:?} v={v}") } |

**`consts` 상수 10건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev | folded_from |
|---|---|---|---|---|---|---|
| 0 | 1 | 735 | 임계 | window_sec = max(mark.window / tps, 1) — 창 초 하한 (llvm.umax, m10.ll:33148) (리터럴 1 — shl 시프트량 아님) | 4 |  |
| 1 | 22500000000 | 740 | 임계 | 150000^2 — ally↔t 제곱거리 임계, 초과하면 그 아군 제외 (m10.ll:33254) | 4 |  |
| 2 | 100 | 748 | 계수 | hits_per_sec_x100 += tps*100 / max(ally.attack_cooltime(),1) — 초당 평타 수 ×100 고정소수(mul 33186, udiv 33270 에서 /100 복원, accum_pct 도 /100 33277) | 4 |  |
| 3 | 1 | 748 | 임계 | attack_cooltime 하한 max(_,1) — 0 나눗셈 방지 (llvm.umax, m10.ll:33406) (리터럴 1 — shl 시프트량 아님) | 4 |  |
| 4 | 1 | 751 | 임계 | max_hp*2 — `shl i64 %105, 1`(m10.ll:33264) 로 접힘. expected_accum(L751)·explosion(L757) 상한 = 대상 최대HP 의 2배 | 4 | 2 |
| 5 | 0 | 753 | 임계 | mark.max_hits == 0 이면 히트 상한 무시(select 33272~33274) — _docs game_core.txt:1250 '히트 누적 상한 (0 = 무관)' | 4 |  |
| 6 | 1 | 758 | 임계 | 분모 하한 max(t.hp,1) (llvm.smax, m10.ll:33291) (리터럴 1 — shl 시프트량 아님) | 4 |  |
| 7 | 160 | 758 | 임계 | 결과 상한 min(v,160) (llvm.smin, m10.ll:33295) — seal/banish(80) 의 2배 | 4 |  |
| 8 | 6 | 760 | 계수 | 디버그 로그 샘플링 — game.tick() % 6 == 0 일 때만 add_log (urem 33318) | 4 |  |
| 9 | 0 | 729 | 임계 | 조기 반환 0 — expected_mark None(L729) / t 비플레이어(L731) (phi 33358). 또 L760 `v > 0` 게이트 비교값(icmp sgt 33297) | 4 |  |

**`knobs` 조정점 5건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 창 초 하한 | buff_value.rs:735 | 1 | 올리면 짧은 표식 창도 N초 누적으로 계상 → 표식 가치 ↑ | 4 | 기존 |
| 1 | 아군 포함 거리 임계(제곱) | buff_value.rs:740 | 22500000000 | 올리면 먼 아군의 DPS/타수까지 누적에 포함 → 기대 폭발딜 ↑ | 4 | 기존 |
| 2 | 누적·폭발 상한 배수 | buff_value.rs:751/757 | 2 | max_hp×N. 내리면(1) 과대 폭발딜이 대상 체력으로 캡 → 고DPS 팀에서 가치 ↓ (shl 접힘) | 4 | 기존 |
| 3 | 결과 상한 | buff_value.rs:758 | 160 | 내리면 표식 가치가 다른 버프(80 상한)와 같은 급으로 눌림 | 4 | 기존 |
| 4 | 디버그 로그 샘플링 주기 | buff_value.rs:760 | 6 | 관측 전용(행동 무영향). debug=true 일 때 6틱마다 1회 로그 | 4 | 기존 |

<details><summary>`callees` 피호출자 11건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | add_log | game_core::DebugFrameData::add_log | pub | fn(&mut game_core::DebugFrameData, &game_core::OperationData, &game_core::PlayerState, std::string::String) | game-core\src\simulation\game\frame.rs:54 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | attack_cooltime | game_core::Entity::attack_cooltime | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1766 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 2 | expected_mark | game_core::EffectType::expected_mark | pub | fn(&Self/#0, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + ) -> std::option::Option<game_core::MarkProfile> | game-core\src\simulation\effect\type.rs:315 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 3 | expected_mark | <game_core::CombineEffect as game_core::EffectType>::expected_mark | pub | fn(&game_core::CombineEffect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + ) -> std::option::Option<game_core::MarkProfile> | game-core\src\simulation\effect\type\combine.rs:76 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 4 | expected_mark | <game_core::HitmanUltEffect as game_core::EffectType>::expected_mark | pub | fn(&game_core::HitmanUltEffect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + ) -> std::option::Option<game_core::MarkProfile> | game-core\src\simulation\effect\type\hitman_ult.rs:45 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 5 | iter_champions | game_core::AbstractGameWithCache::<'a, 'b>::iter_champions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5759 ~ game_core[6a30]::simulation::{impl#5}::iter_champions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1904 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 6 | player_by_champion_id | game_core::AbstractGameWithCache::<'a, 'b>::player_by_champion_id | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> std::option::Option<&'a/#0 game_core::PlayerState> | game-core\src\simulation.rs:1915 | True | True | 3 | IR 호출 심볼 일치(토큰 경계) |
| 7 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 8 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 9 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 10 | v55_mark_value | game_ai::buff_value::v55_mark_value | in:game_ai | fn(usize, &game_core::Effect, &game_core::OperationData, &game_core::PlayerState, &game_ai::ScoreParameter, &game_core::Entity, &game_core::Entity, i64, &mut game_core::DebugFrameData) -> i64 | game-ai\src\buff_value.rs:724 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
</details>

⚠**미매칭 6개**: `dist2`, `format_inner`, `llvm.smax.i64`, `llvm.smin.i64`, `llvm.umax.i64`, `llvm.umin.i64`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m05.ll:42509) · **형제 0개** 

**`open` 5건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | expected_mark 구현체별 MarkProfile 산정(HitmanUlt·VoodooShamanCurse·AddEffectBuff·CombineEffect 등, _docs game_core.txt:1247~1250)은 game_core 경계 밖 | 4 |  |
| 1 | 미탐색 | Entity::attack_cooltime(entity.rs:1766) 의 틱 산식은 game_core — 여기선 usize 반환·max(_,1) 만 | 4 |  |
| 2 | 미탐색 | DebugLog(56B) 의 필드 구성·add_log 가 데이터를 어디서 뽑는지(+128 of data 등)는 g13.ll:122046 본문 소관 — 쓰기 표면은 logs Vec push 로만 특정 | 4 |  |
| 3 | 미탐색 | version·_parameter 를 호출자가 넘기는 이유(시그니처 통일로 추정)는 호출자 범위 | 5 |  |
| 4 | 표기 불가 | L760 `A && B && C` 소스 표기 순서는 column 부재로 표기 불가 — IR 분기 순서(v>0 → debug → tick%6) 기록 | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

