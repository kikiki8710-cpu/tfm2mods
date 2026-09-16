---

### `265` v54_aoe_ally_heal_value — 광역 힐/실드 이펙트가 반경 안 근접 아군(제외 id 제외)에게 주는 기대 힐+실드 가치를 HP 가중 합산(i64)

| 항목 | 값 |
|---|---|
| id | `buff_value__v54_aoe_ally_heal_value` |
| 심볼 | `_RNvNtCshdEBA0ozCnw_7game_ai10buff_value23v54_aoe_ally_heal_value` |
| 소스 | `game-ai\src\buff_value.rs:562` |
| IR | `m10.ll` 35859~36086행 |
| 경로·가시성 | `game_ai::buff_value::v54_aoe_ally_heal_value` · **in:game_ai** |
| 계층 | 점수화·술어 |
| exe | `e02bc0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r18` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &game_core::Effect, &game_core::OperationData, &game_ai::ScoreParameter, &game_core::Entity, &game_core::Entity, usize, &std::boxed::Box<dyn [Binder { value: Trait(game_core::Action), bound_vars: [] }] + 'static, std::alloc::Global>) -> i64
```

**TLS 메모 절(`sig.tls` · 정정 경로 `/specs[265]/sig/tls/<키>`)**

TLS 접점 0 — 본문(35859~36086)에 `call_once`/`LocalKey`/`threadlocal`/fn-포인터 상수 참조 없음. 단 콜리 champion_hp_value 는 m00.ll:91035 에 `LocalKey<RefCell<HashMap>>::with` 클로저 define 이 있어 TLS 메모 소비자일 가능성 — 이 함수의 접점이 아니라 콜리 내부(범위 밖·계약만)

<details><summary>인자 8개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | version | usize (i64 %0) | 본문에서 %0 참조 0회 — 이 함수에서는 분기·전달 모두 없음(미사용) | 4 |
| 1 | 2 | effect | &Effect(56B) %1 | define 줄 속성: noalias readonly captures(address, read_provenance). +0 ty(Arc<dyn EffectType> 팻포인터: +0 ArcInner ptr · +8 vtable ptr)만 직접 읽고, expected_heal_target/expected_shield_target 에 self 로 전달 | 4 |
| 2 | 3 | data | &OperationData(24B) %2 | define 줄 속성: noalias readonly captures(address, read_provenance). +0 cache(&AbstractGameWithCache · 그 +0 = &dyn AbstractGame 팻포인터 %55/%56) · +8 context(&GameContext %40). possible_risk·champion_hp_value 에도 전달 | 4 |
| 3 | 4 | parameter | &ScoreParameter(5384B) %3 | define 줄 속성: noalias readonly captures(address, read_provenance). +0x14b8 near_allies.buf.ptr · +0x14d0 near_allies.len 만 읽음(원소 ChampionScoreParameter 216B). champion_hp_value 에 전달 | 4 |
| 4 | 5 | champ | &Entity(1728B) %4 | define 줄 속성: noalias readonly captures(address, read_provenance). 본문 필드 읽기 0 — 시전자(caster)로 expected_heal_target/expected_shield_target 의 3번째 인자(&dyn AbstractEntity · vtable @anon.ff23c5838f81fa2a3acb125bb4b6e568.29 = Entity as AbstractEntity)에 전달만 | 4 |
| 5 | 6 | anchor | &Entity(1728B) %5 | define 줄 속성: noalias readonly captures(none). +0x660 x · +0x668 y 만 읽음 — 반경 판정의 중심점(이펙트 착지 기준 엔티티). DI 에 `other = %5` 도 있음(Entity::distance 류 인라인의 형식인자명) | 4 |
| 6 | 7 | exclude_id | usize (i64 %6) | near_allies 원소의 +0x58 id 와 `icmp eq` — 같으면 그 아군을 건너뜀(571). 직접 대상(이미 별도 평가된 대상)의 id 로 추정 — 호출자 미열람(범위 밖) | 4 |
| 7 | 8 | action | &Box<dyn Action>(16B 팻포인터) %7 | define 줄 속성: noalias readonly captures(none) dereferenceable(16). +0 data_ptr(%41) · +8 vtable_ptr(%43). vtable +0x80 슬롯 = Action::duration(&self)->usize 를 1회 호출(582) | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn v54_aoe_ally_heal_value(version, effect, data, parameter, champ, anchor, exclude_id, action) -> i64  [buff_value.rs:562~591]
// version(%0) 은 본문에서 미사용.
566: let Some(radius) = effect.ty.expected_ally_area_radius() else { return 0 };   // vtable +0xd0 · Option<u64> = {i1 tag, i64}
570: let mut v: i64 = 0;
570: for ap in parameter.near_allies.iter() {                // ScoreParameter+0x14b8 ptr / +0x14d0 len · 216B stride · len==0 이면 return 0
571:   if ap.id == exclude_id { continue; }                  // +0x58
574:   let Some(ally) = data.cache.game.get_entity_by_id(ap.id) else { continue; };   // vtable +0x1f0 · null=None
577:   let r = ally.radius() + radius;                       // Entity::radius 인라인: mult=ally+0x470(i32); mult==0 ? ally+0x680 : (ally+0x680 * (mult+100)) / 100  (udiv)
578:   let dx = |ally.x - anchor.x|; let dy = |ally.y - anchor.y|;   // +0x660/+0x668, unsigned abs diff
578:   if dx*dx + dy*dy > r*r { continue; }                  // icmp ugt (제곱 비교, 경계 포함 = 반경 안)
581:   let heal = effect.expected_heal_target(data.context, champ as &dyn AbstractEntity, ally as &dyn AbstractEntity);   // usize
582:   let incoming = ap.possible_risk(data, action.duration() + 30) + ap.applyed_damage + ap.risk_epic_damage;   // +0x70, +0x88 (583)
584:   let shield = min(incoming * 3, effect.expected_shield_target(data.context, champ, ally));   // llvm.smin (signed)
585:   if (heal + shield) as i64 > 0 {                        // icmp sgt
586:     let hp_value = champion_hp_value(data, parameter, ap);   // i64
587:     v += hp_value * (heal + shield) / max(ally.hp as i64, 1);   // sdiv · 분모 = 현재 HP (Entity+0x670)
     }
   }
591: return v;

분기 순서(IR 블록): %8 radius None→%125(0) / %24 len==0→%125(0) / %48 id==exclude→%123(continue) / %54 entity null→%123 / %75 dist²>r²→%123 / %95 heal+shield<=0→%112(v 유지) / %115 가산→%112. 루프 종료 = 원소 포인터(%50)==end(%30).
부작용 0(writes 없음 · 인자 전부 readonly/captures(none)). rnd 인자 없음 · gen_range 0.
```

**`mem` 메모리 접근 16건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | Effect | 0x0 | ty.ptr (Arc<dyn EffectType> 데이터 포인터 → ArcInner) | r | m10.ll:35873. ArcInner 헤더 16B 뒤 round_up(16, vtable+16 align) 위치의 dyn 객체를 self 로 vtable 호출 | 4 | OK |
| 1 | Effect | 0x8 | ty.vtable | r | m10.ll:35875. vtable+16 = align(35877) · vtable+0xd0(208) = EffectType::expected_ally_area_radius(&self)->Option<u64>(35883~35884, divtable EffectType 0xd0 · 일치율 94%) | 3 | 부분일치(첫 성분(컨테이너)만 맞고 **마지막 성분이 다르다** — 사람이 봐야 ) |
| 2 | ScoreParameter | 0x14b8 | near_allies.buf.ptr | r | m10.ll:35895~35896 (gep 5304). 원소 ChampionScoreParameter 216B stride(35908 mul 216) | 4 | OK |
| 3 | ScoreParameter | 0x14d0 | near_allies.len | r | m10.ll:35898~35899 (gep 5328). 0 이면 즉시 0 반환(35918) | 4 | OK |
| 4 | ChampionScoreParameter | 0x58 | id | r | m10.ll:35947~35949 (gep 88). == exclude_id 이면 continue(571). get_entity_by_id 인자(574) | 4 | OK |
| 5 | ChampionScoreParameter | 0x70 | applyed_damage | r | m10.ll:36026~36027 (gep 112). incoming 합산(583) | 4 | OK |
| 6 | ChampionScoreParameter | 0x88 | risk_epic_damage | r | m10.ll:36029~36030 (gep 136). incoming 합산(583) | 4 | OK |
| 7 | OperationData | 0x0 | cache (&AbstractGameWithCache) | r | m10.ll:35922. cache+0 = game(&dyn AbstractGame 팻포인터: 35953 data · 35954 vtable) → vtable+0x1f0(496) = AbstractGame::get_entity_by_id(game, id)->Option<&Entity>(null=None) (35955~35958, divtable AbstractGame 0x1f0 일치율 98%) | 3 | OK |
| 8 | OperationData | 0x8 | context (&GameContext) | r | m10.ll:35928~35929. expected_heal_target/expected_shield_target 2번째 인자 | 4 | OK |
| 9 | Entity | 0x470 | stat_buff_cached.radius_mult (i32) | r | ally. m10.ll:35965~35968. Entity::radius() 인라인(entity.rs:1511~1515): 0 이면 radius 그대로, 아니면 radius*(mult+100)/100 | 4 | OK |
| 10 | Entity | 0x680 | radius | r | ally. m10.ll:35972~35973 / 35979~35980 (gep 1664). Entity::radius() 인라인 | 4 | OK |
| 11 | Entity | 0x660 | x | r | ally(35990~35991) 와 anchor(35924~35925). 거리 제곱 계산(578) | 4 | OK |
| 12 | Entity | 0x668 | y | r | ally(35994~35995) 와 anchor(35926~35927). 거리 제곱 계산(578) | 4 | OK |
| 13 | Entity | 0x670 | hp | r | ally 현재 HP. m10.ll:36060~36061 (gep 1648). 분모 max(hp,1)(587) — 최대 HP 가 아니라 현재 HP | 4 | OK |
| 14 | Box<dyn Action> | 0x0 | data_ptr | r | m10.ll:35930. vtable+0x80 슬롯 호출의 self | 4 | 확인불가(tcx 사전에 타입 없음) |
| 15 | Box<dyn Action> | 0x8 | vtable_ptr | r | m10.ll:35932~35933 (+128 = Action::duration, divtable Action 0x80 · 일치율 58% — 두 vtable 전역(TargetProjectileAction/TargetAttackAction) 모두 슬롯 0x80 = duration 으로 일치) | 3 | 확인불가(tcx 사전에 타입 없음) |

**`consts` 상수 7건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 100 | 577 | 계수 | Entity::radius() 인라인(entity.rs:1515): radius*(radius_mult+100)/100 — 반경 배율 백분율 스케일. m10.ll:35981 add 100 · 35983 udiv 100 | 4 |
| 1 | 30 | 582 | 계수 | possible_risk 의 창(틱) = action.duration() + 30 — 시전 시간에 30틱(=0.5초 @60tps, 추정: tps 는 이 함수에서 안 읽음) 여유를 더한 예상 피해 창. m10.ll:36024 | 4 |
| 2 | 3 | 584 | 계수 | shield = min(incoming*3, expected_shield_target) — 실드 가치를 예상 피격(incoming)의 3배로 상한. m10.ll:36034 mul 3 · 36037 llvm.smin | 4 |
| 3 | 0 | 585 | 임계 | `icmp sgt (shield+heal), 0` — 힐+실드 기대량이 양수일 때만 가치 가산. m10.ll:36040 | 4 |
| 4 | 1 | 587 | 임계 | 분모 하한 max(ally.hp, 1) — 0 나눗셈 방지. m10.ll:36064 llvm.smax | 4 |
| 5 | -16 | 566 | 계수 | Arc<dyn EffectType> 데이터 오프셋 계산 `(align-1) & -16` +16 = round_up(16, align) — ArcInner 헤더 뒤 dyn 객체 위치(레이아웃 산술, 판정 상수 아님). m10.ll:35878~35881 | 4 |
| 6 | 16 | 566 | 미상 | ArcInner 헤더 크기(strong+weak) / vtable+16 = align 필드 — 레이아웃 산술. m10.ll:35876·35881 | 4 |

**`knobs` 조정점 4건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | possible_risk 예측 창 여유 틱 | buff_value.rs:582 (m10.ll:36024) | 30 | 올리면 더 긴 창의 예상 피해가 incoming 에 잡혀 실드 상한(incoming*3)이 커진다 → 실드 이펙트의 광역 가치↑. 내리면 시전 직후 피해만 반영해 실드 가치가 보수적으로 줄어든다 | 4 | 기존 |
| 1 | 실드 가치 상한 배율(예상 피격 대비) | buff_value.rs:584 (m10.ll:36034) | 3 | shield = min(incoming*3, expected_shield). 올리면 피해가 적은 아군에게도 실드 기대량이 거의 그대로 인정된다(실드 과대평가↑). 내리면(예: 1) 실제 예상 피격 이하로만 인정돼 실드 남발이 줄어든다 | 4 | 기존 |
| 2 | 가치 가산 게이트(힐+실드 > 0) | buff_value.rs:585 (m10.ll:36040) | 0 | 임계를 올리면 소량 힐/실드는 광역 가치에서 무시된다(후보 유지 조건이 엄격해짐) | 4 | 기존 |
| 3 | HP 분모 하한 | buff_value.rs:587 (m10.ll:36064) | 1 | hp=0 인 아군(사망 직전/사망)의 0 나눗셈 방지. 올리면 저체력 아군 가치 폭증이 완화된다(예: 100 이면 hp<100 은 동일 취급) | 4 | 기존 |

<details><summary>`callees` 피호출자 18건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | champion_hp_value | game_ai::champion_hp_value | pub | fn(&game_core::OperationData, &game_ai::ScoreParameter, &game_ai::ChampionScoreParameter) -> i64 | game-ai\src\utils.rs:909 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | duration | game_core::Action::duration | pub | fn(&Self/#0) -> usize | game-core\src\setting\action.rs:16 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 197개 중 상위 3개 |
| 2 | duration | game_view::UIPhaseEffect::duration | pub | fn(&game_view::UIPhaseEffect) -> f32 | game-view\src\ui\match_ui\phase_effect.rs:35 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 197개 중 상위 3개 |
| 3 | duration | game_view::ui::match_ui::BanpickShowcaseFx::duration | in:game_view::ui::match_ui | fn(&game_view::ui::match_ui::BanpickShowcaseFx) -> f32 | game-view\src\ui\match_ui.rs:1664 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 197개 중 상위 3개 |
| 4 | expected_ally_area_radius | game_core::EffectType::expected_ally_area_radius | pub | fn(&Self/#0) -> std::option::Option<u64> | game-core\src\simulation\effect\type.rs:334 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 4개 중 상위 3개 |
| 5 | expected_ally_area_radius | <game_core::HealEffect as game_core::EffectType>::expected_ally_area_radius | pub | fn(&game_core::HealEffect) -> std::option::Option<u64> | game-core\src\simulation\effect\type\heal.rs:192 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 4개 중 상위 3개 |
| 6 | expected_ally_area_radius | <game_core::RangeEffect as game_core::EffectType>::expected_ally_area_radius | pub | fn(&game_core::RangeEffect) -> std::option::Option<u64> | game-core\src\simulation\effect\type\range_effect.rs:26 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 4개 중 상위 3개 |
| 7 | expected_heal_target | game_core::Effect::expected_heal_target | pub | fn(&game_core::Effect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + , &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + ) -> usize | game-core\src\simulation\effect.rs:109 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 8 | expected_shield_target | game_core::Effect::expected_shield_target | pub | fn(&game_core::Effect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + , &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + ) -> usize | game-core\src\simulation\effect.rs:114 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 9 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 10 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 11 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 12 | possible_risk | game_ai::ChampionScoreParameter::<'_>::possible_risk | pub | fn(&game_ai::ChampionScoreParameter</#0>, &game_core::OperationData, usize) -> i64 | game-ai\src\score_parameter.rs:1407 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 13 | radius | game_core::Entity::radius | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1509 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 14 | v54_aoe_ally_heal_value | game_ai::buff_value::v54_aoe_ally_heal_value | in:game_ai | fn(usize, &game_core::Effect, &game_core::OperationData, &game_ai::ScoreParameter, &game_core::Entity, &game_core::Entity, usize, &std::boxed::Box<dyn [Binder { value: Trait(game_core::Action), bound_vars: [] }] + 'static, std::alloc::Global>) -> i64 | game-ai\src\buff_value.rs:562 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 15 | version | <game_ai::AgentVerHamster as game_core::AiAgent>::version | pub | fn(&game_ai::AgentVerHamster) -> usize | game-ai\src\lib.rs:428 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 592개 중 상위 3개 |
| 16 | version | game_core::AiAgent::version | pub | fn(&Self/#0) -> usize | game-core\src\simulation\ai_interface.rs:503 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 592개 중 상위 3개 |
| 17 | version | <game_core::Team as engine_core::spitz::SpitzDatable>::version | pub | fn() -> usize | game-core\src\data\team.rs:1696 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 592개 중 상위 3개 |
</details>

⚠**미매칭 4개**: `captures`, `llvm.smax.i64`, `llvm.smin.i64`, `smin`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 2곳** (m05.ll:43438, m05.ll:44061) · **형제 0개** 

**`open` 6건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | exclude_id(%6) 가 호출자에서 정확히 무엇(직접 대상 id / 시전자 id)인지 — 호출자(buff_value 상위·battle 등)는 이 배치 범위 밖. IR 상 near_allies 원소 id 와의 등치 비교만 확정 | 4 |  |
| 1 | 미탐색 | possible_risk 3번째 인자(usize)의 소스 이름 — 콜리 m07.ll:7315 는 범위 밖(계약만: (&ChampionScoreParameter, &OperationData, usize) -> i64). 이 함수는 action.duration()+30 을 넘긴다 | 4 |  |
| 2 | 미탐색 | expected_ally_area_radius 의 런타임 구현체 — Arc<dyn EffectType> 이라 divtable 은 정적 vtable 전역 2개(DokkaebiUltExplosionEffect·CombineEffect)만 보여 준다. 슬롯 0xd0 = expected_ally_area_radius 는 두 전역 모두 일치(94%)·tcx 시그니처 `fn(&Self)->Option<u64>`(type.rs:334) 와 반환 {i64,i64}+trunc i1 이 정합 | 3 |  |
| 3 | 표기 불가 | same-line 순서: 582 의 `possible_risk + applyed_damage + risk_epic_damage` 덧셈 순서는 IR(36028 possible_risk+applyed → 36031 +risk_epic) 기준이며 column 정보 없음 — 정수 덧셈이라 결과 동일(표기 불가·동작 확정) | 4 |  |
| 4 | 미탐색 | 30 틱이 tps 파생인지 리터럴인지 — 본문에 tps 읽기 없고 리터럴 30 이 직접 add 됨(m10.ll:36024) → 리터럴 확정. '0.5초' 환산만 추정(tps=60 가정) | 4 |  |
| 5 | 미탐색 | expected_heal_target/expected_shield_target 내부(usize 반환 → i64 로 그대로 사용) — game_core effect.rs:109/114 · 계약만(범위 밖) | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

