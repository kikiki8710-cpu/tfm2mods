---

### `129` should_add_self_etc_buff_action — 자기/기타 버프(EffectType::etc_buff) 액션 후보를 넣을지 — etc_buff 가 아니면 항상 true, etc_buff 면 (평타사거리+30000 또는 90000)+이속×30 안에 표적 가능한 적(챔프·소환수·미니언·정글·타워·넥서스)이 하나라도 있을 때만 true

| 항목 | 값 |
|---|---|
| id | `fight_check__should_add_self_etc_buff_action` |
| 심볼 | `_RNvNtCshdEBA0ozCnw_7game_ai11fight_check31should_add_self_etc_buff_action` |
| 소스 | `game-ai\src\fight_check.rs:608` |
| IR | `m15.ll` 34622~35407행 |
| 경로·가시성 | `game_ai::fight_check::should_add_self_etc_buff_action` · **in:game_ai** |
| 계층 | 점수화·술어 |
| exe | `ebcbd0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r13` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_core::PlayerState, &game_core::OperationData, &game_core::Entity, &game_core::Effect) -> bool
```

<details><summary>인자 5개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | player | &PlayerState(2528B) | info.team 만 읽음(34734~34735) + can_target 인자 | 4 |
| 1 | 2 | data | &OperationData(24B) | +0 cache. can_target 의 self | 4 |
| 2 | 3 | champ | &Entity(1728B) | 내 챔피언. attack_effect·level·range 스탯·radius·move_speed·x/y | 4 |
| 3 | 4 | effect.ty.data_ptr | ptr (Arc<dyn EffectType> ArcInner*) | ArcInner 데이터 오프셋 = round_up(16, align)(34643~34648) → &dyn EffectType self | 4 |
| 4 | 4 | effect.ty.vtable_ptr | ptr (EffectType vtable) | vtable+0x90(144) = etc_buff (divtable EffectType 0x90 · tcx 메서드 idx14 와 정합) 호출(34649~34651) | 3 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn should_add_self_etc_buff_action(player, data, champ, effect:&Effect) -> bool
  if !effect.ty.etc_buff() { return true }                                   // L609 vtable+0x90 (34649~34652 → %314 true)
  return has_nearby_self_etc_buff_target(player, data, champ)                // L609 (fight_check.rs:586~606 전부 인라인)

fn has_nearby_self_etc_buff_target(player, data, champ) -> bool   // fight_check.rs:586
  let range = champ.attack_effect.as_ref()
      .map(|a| a.range + 30000 + champ.stat_buff_cached.range + (champ.level-1)*a.growth_range + radius(champ))   // L587~588 closure#0 (34676~34717; 표적 반경·range_adjust 없음)
      .unwrap_or(90000)                                                          // L589 (34724)
  let r2 = (range + champ.stat_cached.move_speed*30)²                            // L590~591 (34726~34733)
  let enemy = 1 - player.info.team                                               // L593 (34736; <2 아니면 panic)
  let near_op = |e| data.can_target(cache.game, player, e) && distance_sq(e, champ) <= r2   // closure#1/#2/#3 (L595·597·599) — OperationData::can_target 호출
  let near_ent = |e| e.can_target && e.block_target_tick==0 && distance_sq(e, champ) <= r2   // closure#4/#5/#6 (L601·603·605) — Entity 필드
     cache.player_champion[enemy].iter().flatten().any(near_op)                 // L594~595 (34796~35108, 5칸 전개; ugt r2 → 다음)
  || cache.others[enemy].iter().any(near_op)                                    // L596~597 (35116~35207)
  || cache.iter_minions(enemy).any(near_op)                                     // L598~599 (35211~35232; aux m06 8632 Chain::any → m11 30762 try_fold(closure#3 인라인, 30830·30863) · m11 36721 try_fold → m06 48754 call_mut 심)
  || cache.jungles.iter().any(near_ent)                                         // L600~601 (35237~35329)
  || cache.iter_towers_without_nexus(enemy).any(near_ent)                       // L602~603 (35333~35339; aux m06 34663: 타워 6칸(34775~34812) + 쌍둥이 slice try_fold(34842))
  || cache.nexus[enemy].is_some_and(near_ent)                                   // L604~605 (35342~35397; None → false 35349, can_target 실패 → false 35366)
극성(35400 phi): 각 부류에서 `dist_sq > r2`(ugt) 가 false 인 첫 후보 → true. 6부류 전부 실패(마지막 넥서스 None/불가/원거리) → false. 34652: etc_buff false → 34405 phi true.
```

**`mem` 메모리 접근 20건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | Entity | 0x4c0 | attack_effect@tag(니치) | r | champ. -1=None → 사거리 기본 90000(34667~34670, 34724, fight_check.rs:587~589) | 4 | OK |
| 1 | Entity | 0x4a0 | attack_effect@Some.0.range | r | 34676~34677 (+30000) | 4 | OK |
| 2 | Entity | 0x4a8 | attack_effect@Some.0.growth_range | r | ×(level-1) 34678~34679, 34712~34713 | 4 | OK |
| 3 | Entity | 0x5c8 | level | r | 34685~34686 | 4 | OK |
| 4 | Entity | 0x438 | stat_buff_cached.range | r | 34687~34688 | 4 | OK |
| 5 | Entity | 0x470 | stat_buff_cached.radius_mult | r | champ 반경 배율(34689~34693) — 표적 반경은 더하지 않음 | 4 | OK |
| 6 | Entity | 0x680 | radius | r | champ(34696, 34703) | 4 | OK |
| 7 | Entity | 0x640 | stat_cached.move_speed | r | champ ×30 가산(34726~34729, L590) | 4 | OK |
| 8 | Entity | 0x660 | x | r | champ(34783·34786) 및 각 적 후보(34818 등) | 4 | OK |
| 9 | Entity | 0x668 | y | r | champ(34784·34787) 및 각 적 후보 | 4 | OK |
| 10 | Entity | 0x6b9 | can_target | r | 정글(35280~35283)·타워(aux m06 34775~34777)·넥서스(35359~35361) 는 Entity::can_target(entity.rs:1478) 필드 검사. 챔프·소환수·미니언은 OperationData::can_target 호출 | 4 | OK |
| 11 | Entity | 0x6a0 | block_target_tick | r | ==0 이어야 표적 가능(35286~35288, 35362~35364, aux m06 34778~34780) | 4 | OK |
| 12 | PlayerState | 0x930 | info.team | r | 1-team = 적 인덱스, <2 아니면 panic(34734~34738, 35150) | 4 | OK |
| 13 | OperationData | 0x0 | cache | r | 34741 | 4 | OK |
| 14 | AbstractGameWithCache | 0x0 | game.data_ptr | r | can_target 인자(34810 등) | 4 | OK |
| 15 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r | dr816 (34811 등) | 4 | OK |
| 16 | AbstractGameWithCache | 0x1e0 | player_champion[enemy][0..5] | r | L594~595 적 챔피언 5칸 전개(34742~34743, 34796·34862·34928·34994·35060 null 검사) | 4 | OK |
| 17 | AbstractGameWithCache | 0xf0 | others[enemy].buf.ptr / +24 len | r | L596~597 소환수 순회(35116~35147) | 4 | OK |
| 18 | AbstractGameWithCache | 0xd0 | jungles.buf.ptr / +0xe8 len | r | L600~601 정글몹 순회(35237~35264) — 팀 무관(단일 Vec) | 4 | OK |
| 19 | AbstractGameWithCache | 0x170 | nexus[enemy] | r | L604~605 Option<&Entity>(35342~35349) | 4 | OK |

**`consts` 상수 8건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 30000 | 588 | 계수 | 평타 사거리에 더하는 고정 여유(≈0.94셀) — range = atk.range + 30000 + stat.range + (level-1)*growth + radius(champ) (34714) | 4 |
| 1 | 90000 | 589 | 산출값 | attack_effect 가 없을 때의 기본 탐색 사거리(unwrap_or, 34724) ≈ 2.8셀 | 4 |
| 2 | 30 | 590 | 계수 | 이속 × 30틱(0.5초@60tps) 추가 여유(34728). 최종 r = range + move_speed*30, r² 로 비교(34732) | 4 |
| 3 | 100 | 588 | 계수 | 반경 배율 radius*(mult+100)/100 (entity.rs:1515, 34705~34707) | 4 |
| 4 | -1 | 587 | 센티널 | Option<Effect> 니치 None(i32, 34669) | 4 |
| 5 | 0 | 601 | 태그 | block_target_tick == 0 (Entity::can_target 인라인, 35288·35364·aux 34780) | 4 |
| 6 | 1 | 593 | 인덱스 | 1 - team = 적 팀 인덱스(34736); (level-1)(34712) | 4 |
| 7 | 2 | 593 | 임계 | 팀 배열 bounds(34737) — 임계 아님 | 4 |

**`knobs` 조정점 3건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 평타 사거리 고정 여유 | fight_check.rs:588 (m15.ll:34714) | 30000 | 올리면 더 먼 적이 있어도 자기버프 후보가 생성됨(버프를 더 일찍/자주 고려) | 4 | 기존 |
| 1 | 평타 없을 때 기본 탐색 사거리 | fight_check.rs:589 (m15.ll:34724) | 90000 | attack_effect 없는 챔프의 자기버프 발동 거리 | 4 | 기존 |
| 2 | 이속 × 틱 여유 | fight_check.rs:590 (m15.ll:34728) | 30 | 올리면 이속 높은 챔프가 더 먼 적에도 자기버프 후보 생성 | 4 | 기존 |

<details><summary>`callees` 피호출자 11건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | can_target | game_core::OperationData::<'a, 'b>::can_target | pub | fn(&game_core::OperationData<'a/#0, 'b/#1>, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\state\player.rs:23 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | distance_sq | game_core::utils::distance_sq | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:6 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 2 | distance_sq | game_core::Entity::distance_sq | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2157 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 3 | etc_buff | game_core::EffectType::etc_buff | pub | fn(&Self/#0) -> bool | game-core\src\simulation\effect\type.rs:304 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 31개 중 상위 3개 |
| 4 | etc_buff | <game_core::BanishEffect as game_core::EffectType>::etc_buff | pub | fn(&game_core::BanishEffect) -> bool | game-core\src\simulation\effect\type\banish.rs:84 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 31개 중 상위 3개 |
| 5 | etc_buff | <game_core::CombineEffect as game_core::EffectType>::etc_buff | pub | fn(&game_core::CombineEffect) -> bool | game-core\src\simulation\effect\type\combine.rs:54 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 31개 중 상위 3개 |
| 6 | has_nearby_self_etc_buff_target | game_ai::fight_check::has_nearby_self_etc_buff_target | in:game_ai | fn(&game_core::PlayerState, &game_core::OperationData, &game_core::Entity) -> bool | game-ai\src\fight_check.rs:586 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 7 | iter_minions | game_core::AbstractGameWithCache::<'a, 'b>::iter_minions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5753 ~ game_core[6a30]::simulation::{impl#5}::iter_minions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1847 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 8 | iter_towers_without_nexus | game_core::AbstractGameWithCache::<'a, 'b>::iter_towers_without_nexus | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5750 ~ game_core[6a30]::simulation::{impl#5}::iter_towers_without_nexus::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1830 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 9 | radius | game_core::Entity::radius | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1509 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 10 | should_add_self_etc_buff_action | game_ai::fight_check::should_add_self_etc_buff_action | in:game_ai | fn(&game_core::PlayerState, &game_core::OperationData, &game_core::Entity, &game_core::Effect) -> bool | game-ai\src\fight_check.rs:608 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
</details>

⚠**미매칭 2개**: `llvm.assume`, `try_fold`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 4곳** (m15.ll:24771, m15.ll:25462, m15.ll:26371, m15.ll:26765) · **형제 0개** 

**`open` 3건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | iter_minions/iter_towers_without_nexus(_gcbc g15.ll:102624 / 108971) 의 원소 구성은 미독 — 후자는 Chain<Flatten<IntoIter<[Option<&Entity>;6]>>, Copied<Iter>>(타워 6 + 쌍둥이) 로 타입명에서 추정 | 4 |  |
| 1 | 미탐색 | 6부류 검사 순서(챔프→소환수→미니언→정글→타워→넥서스)는 IR 블록 순서와 소스 줄(594~605) 일치 — `\|\|` 단락 평가라 순서가 결과엔 영향 없음 | 4 |  |
| 2 | 미탐색 | has_nearby_self_etc_buff_target 은 별도 define 없이 전부 인라인(fnparts: 서브프로그램 60 vs define 8, 나머지는 이터레이터 인스턴스) — 다른 호출자(should_add_self_skill2_action 등)에서의 복제본은 본 명세 범위 밖 | 4 |  |

**`notes` 1건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | OperationData::can_target(&self, game_data, game_vtable, &PlayerState, &Entity)->bool 내부 미독(game_core). Entity::can_target(entity.rs:1478) 은 `can_target(0x6b9) && block_target_tick(0x6a0)==0` 로 IR 확정 | 4 | 사실 서술 |

<details><summary>`closed` 2건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

| # | 물음 | 닫은 근거 |
|---|---|---|
| 0 | EffectType::etc_buff(&self)->bool (vtable+0x90) 의 구현별 의미(어떤 이펙트가 '기타 버프'인지)는 game_core 전 구현체 미독 — 이름·시그니처만. 슬롯 이름은 divtable(94%) + tcx 메서드 idx14 ↔ (0x90/8-3-1)=14 정합으로 확정(vtable 에 슬롯 3 추가 항목 1개 존재 추정) | 본문에 해소 표기가 있다 |
| 1 | L588 closure#0 의 사거리 식이 Effect::range() 인라인(effect.rs:26)인지 수동 합산인지 소스 표기 미확정 — 값은 IR 로 확정(표적 반경·range_adjust 미포함이 attack_summon_action 과의 차이) | 본문에 해소 표기가 있다 |
</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

