---

### `78` objective_defense_role — 막판 오브젝트(모가드/세르펜) 처치 vs 본진 방어 인원 분배 — 나는 StayFinish/GoDefend/Continue 중 무엇인가

| 항목 | 값 |
|---|---|
| id | `defense_nexus__objective_defense_role` |
| 심볼 | `_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old13defense_nexus22objective_defense_role` |
| 소스 | `game-ai\src\plan_legacy\old\defense_nexus.rs:382` |
| IR | `m04.ll` 59521~60397행 |
| 경로·가시성 | `game_ai::plan_legacy::old::objective_defense_role` · **pub** |
| 계층 | 레거시 플랜 |
| exe | `d3dcc0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r10` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &game_core::PlayerState, &game_core::OperationData, game_core::JungleType, usize) -> game_ai::plan_legacy::old::DefenseRole
```

<details><summary>인자 5개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | _version | usize | %0. 미사용 | 4 |
| 1 | 2 | player | &PlayerState(2528B) | %1 | 4 |
| 2 | 3 | data | &OperationData(24B) | %2 | 4 |
| 3 | 4 | jungle_type | JungleType(i8) | %3. 4=Morgard / 5=Serpen 만 처리, 그 외는 defend_or_continue(&[]) | 4 |
| 4 | 5 | need_count | usize | %4. i_am_chosen_defender 에 그대로 전달 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
defend_or_continue(exclude: &[usize]) = closure$0(L384): if i_am_chosen_defender(player, data, need_count, exclude) { GoDefend(1) } else { Continue(2) }

L388: if nexus_under_direct_attack(player, data) { return defend_or_continue(&[]) }   // 넥서스 실제 위험 → 분배 없이 방어
L393: obj_id = match jungle_type {
        Morgard(4) => data.cache.game.get_game_mode().as_moba().unwrap().jungle_runner.epic.live_list.get(0).copied()   (L394)
        Serpen(5)  => ...jungle_runner.serpen.live_list.get(0).copied()                                                   (L395)
        _ => return defend_or_continue(&[]) }
L398: obj = obj_id.and_then(|id| cache.game.get_entity_by_id(id)); None → L399 return defend_or_continue(&[])
L402: tps = data.context.setting.tick_per_second
L406: committed: Vec<(i64,usize)> = new_in(context.pool)
L407: for p in 0..5 {
  L408: ally = cache.player_champion[player.info.team][p]  (None → continue)
  L411: if ally.distance_sq(obj) > 40000000000 { continue }
  L414: atk = ally.attack_effect.as_ref()  (None → continue)
  L415: dps = atk.expected_damage_target(context, ally, obj as &dyn(vtable @anon.6)) * tps / max(ally.attack_cooltime(), 1)   (i64 signed 나눗셈, MIN/-1 오버플로 패닉 가드)
  L417: if dps > 0 { L418: committed.push((dps, ally.id)) } }
L421: if committed.is_empty() { L422: return defend_or_continue(&[]) }
L425: combined = Σ committed.dps
L427: if obj.hp > combined * 6 { L428: return defend_or_continue(&[]) }   // 6초 창 안에 못 잡음 → 분배 없음
L432: committed.sort_by(closure$4)  — 비교자 is_less(a,b) = b.dps < a.dps (aux m12.ll:3738~3748 median3: x=is_less(a,b)=`slt b.0,a.0`) ⟹ dps 내림차순 (stable)
L433: needed_dps = obj.hp / 6 + 1
L435: finishers: Vec<usize> = new_in(pool); acc = 0
L436: for (dps,id) in committed {  L437: if acc >= needed_dps { break }  (IR: `acc > hp/6`)
  L440: acc += dps;  L441: finishers.push(id) }
L444: my_champ = cache.player_champion[team][player.info.position]; None → return Continue(2)
L447: if finishers.contains(&my_champ.id) { return StayFinish(0) }
L450: return defend_or_continue(&finishers)   // finisher 제외 풀에서 need_count 명 선발
L451: drop(finishers), drop(committed)
```

**`mem` 메모리 접근 23건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | team (59715); <2 아니면 bounds panic (60000, 60247) | 4 | OK |  |
| 1 | PlayerState | 0x9c0 | info.position@tag | r | as_index → my_champ 인덱스 (60008) | 4 | OK |  |
| 2 | OperationData | 0x0 | cache | r | 59585 | 4 | OK |  |
| 3 | OperationData | 0x8 | context | r | 59698 | 4 | OK |  |
| 4 | AbstractGameWithCache | 0x0 | game.data_ptr | r | 59586 | 4 | OK |  |
| 5 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r | 슬롯 0x40=get_game_mode(59590) · 0x1f0=get_entity_by_id(59655) | 4 | OK |  |
| 6 | AbstractGameWithCache | 0x1e0 | player_champion | r | [2][5] Option<&Entity> (59719~59720, 60011, 60251) | 4 | OK |  |
| 7 | GameMode | 0x0 | tag | r | get_game_mode 반환 {i64,ptr}: tag 0=Moba 아니면 as_moba().unwrap() 패닉 (59595) | 4 | OK |  |
| 8 | MobaMode | 0x1a0 | jungle_runner.epic.live_list.buf.inner.ptr | r | Morgard 경로 (59631) | 4 | OK |  |
| 9 | MobaMode | 0x1a8 | jungle_runner.epic.live_list.len | r | 0 이면 .get(0)=None (59621) | 4 | OK |  |
| 10 | MobaMode | 0x1d0 | jungle_runner.serpen.live_list.buf.inner.ptr | r | Serpen 경로 (59684) | 4 | OK |  |
| 11 | MobaMode | 0x1d8 | jungle_runner.serpen.live_list.len | r | 59674 | 4 | OK |  |
| 12 | GameContext | 0x0 | pool | r | &Bump — Vec::new_in (59705) | 4 | OK |  |
| 13 | GameContext | 0x8 | setting | r | 59700 | 4 | OK |  |
| 14 | GameSetting | 0x12f8 | tick_per_second | r | tps (59701) | 4 | OK |  |
| 15 | Entity | 0x660 | x | r | ally.x / obj.x (59721, 60259) | 4 | OK |  |
| 16 | Entity | 0x668 | y | r | 59722, 60263 | 4 | OK |  |
| 17 | Entity | 0x4c0 | attack_effect@tag | r | i32 -1 = None → 스킵 (60291~60293) | 4 | OK |  |
| 18 | Entity | 0x490 | attack_effect@Some.0 | r | &Effect(56B) → expected_damage_target (60297) | 4 | OK |  |
| 19 | Entity | 0x5c0 | id | r | ally.id 를 committed 에 저장(60343) / my_champ.id 를 finishers 에서 검색(60023) | 4 | OK |  |
| 20 | Entity | 0x670 | hp | r | obj.hp (59843) | 4 | OK |  |
| 21 | stack Vec<(i64,usize)> committed | 0x0 -> committed.ptr[len]+0 | (dps, ally.id) | w | 60383~60388. bumpalo Vec, 함수 끝에서 drop | 4 | 확인불가(tcx 사전에 타입 없음) | push |
| 22 | stack Vec<usize> finishers | 0x0 -> finishers.ptr[len]+0 | ally.id | w | 59988~59991. 마무리조 id 목록 | 4 | 확인불가(tcx 사전에 타입 없음) | push |

**`consts` 상수 9건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 4 | 393 | 태그 | JungleType 태그 4 = Morgard → epic.live_list[0] | 4 |
| 1 | 5 | 393 | 태그 | JungleType 태그 5 = Serpen → serpen.live_list[0]. (같은 5 가 L407 `0..5` 포지션 루프 상한으로도 쓰임) | 4 |
| 2 | 40000000000 | 411 | 임계 | 200000^2 — ally.distance_sq(obj) > 200000² 이면 '붙은 아군' 아님(스킵). 6.25셀 | 4 |
| 3 | -1 | 414 | 센티널 | attack_effect Option 니치 태그 None(i32 -1) / attack_cooltime()==-1(usize::MAX) 오버플로 가드 | 4 |
| 4 | 0 | 417 | 임계 | dps > 0 인 아군만 committed 에 등록 | 4 |
| 5 | 6 | 427 | 계수 | window_secs = 6초. L427 `obj.hp > combined*6` 이면 창 안에 못 잡음 / L433 needed_dps = obj.hp/6 + 1 | 4 |
| 6 | 1 | 433 | 태그 | needed_dps = hp/6 + 1 (dbg_value DW_OP_plus_uconst 1; IR 은 `acc > hp/6` 로 접힘) · DefenseRole GoDefend=1 | 4 |
| 7 | 2 | 384 | 임계 | DefenseRole Continue=2 (select 결과 및 my_champ None 폴백). aux m12.ll:3691 의 `shl %9, 2` 는 median3_rec 라이브러리 산술(n/8*4)로 판정과 무관 — shl 접힘 아님 | 4 |
| 8 | 3 | 382 | 미상 | 반환 i8 range(0,3) — DefenseRole 3변형 | 4 |

**`knobs` 조정점 3건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 처치 창(window_secs) | defense_nexus.rs:427,433 | 6 | 올리면 더 낮은 합산 DPS 로도 '곧 잡는다'고 봐서 finisher 분리(잔류)가 늘고, needed_dps 가 줄어 finisher 인원도 준다 | 4 | 기존 |
| 1 | '붙은 아군' 판정 거리 | defense_nexus.rs:411 | 40000000000 | 올리면 오브젝트에서 더 먼 아군의 DPS 도 합산돼 combined 가 커지고 finisher 후보가 늘어난다 | 4 | 기존 |
| 2 | DPS 하한 | defense_nexus.rs:417 | 0 | dps≤0(평타 없음·데미지 0) 아군은 finisher 후보에서 제외 | 4 | 기존 |

<details><summary>`callees` 피호출자 22건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | as_moba | game_core::GameMode::<'a>::as_moba | pub | fn(game_core::GameMode<'a/#0>) -> std::option::Option<&'a/#0 game_core::MobaMode> | game-core\src\simulation\game.rs:230 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 1 | attack_cooltime | game_core::Entity::attack_cooltime | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1766 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 2 | contains | game_core::RectU64::contains | pub | fn(&game_core::RectU64, u64, u64) -> bool | game-core\src\setting.rs:40 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 3 | contains | game_core::RectI64::contains | pub | fn(&game_core::RectI64, i64, i64) -> bool | game-core\src\setting.rs:55 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 4 | contains | game_core::setting::champion::nightmare::NightmareWellRect::contains | in:game_core::setting::champion::nightmare | fn(game_core::setting::champion::nightmare::NightmareWellRect, u64, u64) -> bool | game-core\src\setting\champion\nightmare.rs:28 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 5 | distance_sq | game_core::utils::distance_sq | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:6 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 6 | distance_sq | game_core::Entity::distance_sq | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2157 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 7 | expected_damage_target | game_core::Effect::expected_damage_target | pub | fn(&game_core::Effect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + , &game_core::Entity) -> usize | game-core\src\simulation\effect.rs:91 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 8 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 9 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 10 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 11 | get_game_mode | game_core::AbstractGame::get_game_mode | pub | fn(&Self/#0) -> game_core::GameMode | game-core\src\simulation.rs:78 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 12 | get_game_mode | <game_core::Game as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::Game) -> game_core::GameMode | game-core\src\simulation\game.rs:1707 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 13 | get_game_mode | <game_core::SingleLaneGame as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::SingleLaneGame) -> game_core::GameMode | game-core\src\simulation\game.rs:4248 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 14 | i_am_chosen_defender | game_ai::plan_legacy::old::i_am_chosen_defender | pub | fn(&game_core::PlayerState, &game_core::OperationData, usize, &[usize]) -> bool | game-ai\src\plan_legacy\old\defense_nexus.rs:297 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 15 | is_empty | game_core::PatchSetting::is_empty | pub | fn(&game_core::PatchSetting) -> bool | game-core\src\patch_engine.rs:853 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 16 | is_empty | game_core::ModAiRegistry::is_empty | pub | fn(&game_core::ModAiRegistry) -> bool | game-core\src\mod_ai.rs:15 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 17 | is_empty | game_core::PublicChampionEvidence::is_empty | pub | fn(&game_core::PublicChampionEvidence) -> bool | game-core\src\banpick\public_evidence.rs:96 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 18 | nexus_under_direct_attack | game_ai::plan_legacy::old::nexus_under_direct_attack | pub | fn(&game_core::PlayerState, &game_core::OperationData) -> bool | game-ai\src\plan_legacy\old\defense_nexus.rs:121 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 19 | push | <game_core::Staff as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::staff | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\staff.rs:38 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 20 | push | <game_core::Athlete as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:434 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 21 | push | <game_core::Contract as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:2498 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
</details>

⚠**미매칭 8개**: `defend_or_continue`, `drop  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `is_less`, `llvm.assume`, `llvm.memset.p0.i64`, `llvm.umax.i64`, `reserve_internal_or_panic`, `sort_by`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 2곳** (m09.ll:30963, m09.ll:31805) · **형제 0개** 

**`open` 5건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | i_am_chosen_defender(defense_nexus.rs:297) 본문 미독 — 계약: (&PlayerState,&OperationData,need_count:usize,exclude:&[usize])->bool. _docs 에 'v39 적합도 키=(푸셔 못막음?, 웨이브클리어점수 내림차순, 넥서스까지 거리, id) 순 need_count 명' 주석 있음 | 4 |  |
| 1 | 미탐색 | expected_damage_target 의 4번째 인자 vtable @anon.6(Entity 1728B 용, 슬롯 9개)의 트레이트 이름 — game_core 쪽 정적 vtable이라 divtable 로 미조회 | 3 |  |
| 2 | 표기 불가 | L437 의 `acc > obj.hp/6` 가 소스에서 `acc >= needed_dps`(needed_dps=hp/6+1) 인지 `acc > hp/6` 인지 — 외연 동일, 표기 불가 | 4 |  |
| 3 | 표기 불가 | sort_by 비교자(closure$4)가 `b.0.cmp(&a.0)` 인지 `a.0.cmp(&b.0).reverse()` 인지 — 동작(dps 내림차순)은 aux median3 로 확정, 표기 불가. 동점 시 stable 이라 포지션 순 유지 | 4 |  |
| 4 | 미탐색 | L407 루프의 5 와 JungleType 태그 5 가 같은 리터럴이라 C1 상 구분 불가 — meaning 에 병기 | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

