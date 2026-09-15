---

### `233` nexus_final_stand_uncached — 최후 단계 판정: 우리 넥서스 존재 AND 쌍둥이 타워 전멸(twin_towers 빈 벡터) AND (적 미니언의 nearest_enemy 가 넥서스 id OR 적 챔프 평타가 넥서스 사거리 안)

| 항목 | 값 |
|---|---|
| id | `defense_nexus__nexus_final_stand_uncached` |
| 심볼 | `_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old13defense_nexus26nexus_final_stand_uncached` |
| 소스 | `game-ai\src\plan_legacy\old\defense_nexus.rs:239` |
| IR | `m04.ll` 61661~61940행 |
| 경로·가시성 | `game_ai::plan_legacy::old::defense_nexus::nexus_final_stand_uncached` · **in:game_ai::plan_legacy::old::defense_nexus** |
| 계층 | 레거시 플랜 |
| exe | `d3fe50` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r17` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_core::PlayerState, &game_core::OperationData) -> bool
```

**TLS 메모 절(`sig.tls` · 정정 경로 `/specs[233]/sig/tls/<키>`)**

- `name`: LAST_STAND_MEMO (game_ai::plan_legacy::old::defense_nexus::LAST_STAND_MEMO · m04.ll:262 `@anon.168add0ea037d45d276f5936ae758fe5.248`)
- `role`: 값 생산자(간접). 본 함수 자체는 TLS 미접촉(@anon 참조 1개 = panic Location .262 · LocalKey::with 0). 작성자 = last_stand_flags::{closure#0}(m00.ll:90215~90610) 이 본 함수를 호출해 튜플 .1(byte1) 에 저장(m00.ll:90537 호출 · 90558 `select %108, 256, 0`). 소비자 = nexus_final_stand(old/defense_nexus.rs:190 · m04.ll:57045)
- `key`: player.info.id (PlayerState+0x928) — nexus_last_stand_uncached 명세 tls 절과 동일
- `layout`: RefCell<(usize seed, usize tick, HashMap<usize,(bool,bool,bool)>)> · 값 byte1 = 본 함수 결과 (상세 = defense_nexus__nexus_last_stand_uncached.json tls.layout)
- `invalidation`: seed 또는 tick 변경 시 HashMap.clear() (m00.ll:90257~90273, 90491~90499)
- `call_conditions`: 메모 miss 시 ①nexus_last_stand_uncached ②본 함수(m00.ll:90537) ③base_attacking_minion_uncached 순으로 호출 · hit 이면 호출 안 됨 · 본 함수는 last_stand 결과와 무관하게 무조건 호출됨(단락 없음 · 90533→90537 직렬)

<details><summary>인자 2개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | player | &PlayerState (2528B) | IR `readonly captures(none)` · info.team(0x930)만 읽음(position 미참조) | 4 |
| 1 | 2 | data | &OperationData (24B) | IR `readonly captures(none)` · cache(0x0)만 읽음 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn nexus_final_stand_uncached(player, data) -> bool   [old/defense_nexus.rs:239]
  team = player.info.team                                                          [L240 · m04.ll:61670~61671]
  let Some(nexus) = data.cache.nexus[team] else { return false }                   [L241 · 61673~61683 · bounds<2]
  if !data.cache.twin_towers[team].is_empty() { return false }                     [L244 · 61691~61697 · len(+0x148) != 0 → false]
  enemy = 1 - team                                                                 [L247 · 61701]
  // ① 적 미니언이 넥서스를 노리는가 (closure#0 L247:44 · 본체 aux m06.ll:48054~48090)
  if data.cache.iter_minions(enemy)                                                [L247 · 61703 sret 56B Chain<Copied<Iter<&Entity>>,Copied<Iter<&Entity>>>]
       .any(|m| matches!(&m.ty, EntityType::Minion(mi) if mi.nearest_enemy == Some(nexus.id)))   [L248 · 61706 Chain::try_fold(aux m06.ll:8709~8782) → call_mut 심 48054~48090: ty tag(+0x68)==1 && nearest_enemy(+0x88 tag, +0x90 val) == Some(nexus.id(+0x5c0))]
     { return true }                                                               [61708 → %79 true]
  // ② 적 챔프 평타가 넥서스 사거리 안인가 (closure#1 L252:43 · closure#1::closure#0 L253:44)
  return data.cache.iter_champions(enemy)                                          [L252 · 5슬롯 언롤 61711~61935]
       .any(|c| c.attack_effect.as_ref().is_some_and(|atk| atk.is_in_range(c, nexus)))   [L253 · 61758/61800/61842/61884/61926 → true · 전부 아님 → 61935 false]
                                                                                   [L255 ret 61939]

주의: ①의 nearest_enemy 는 game_core 미니언 AI 가 매 틱 고른 타깃(Minion+0x18)이라 '넥서스를 실제 때리는 중' 의 근사. ②는 nexus_last_stand_uncached ① 과 같은 판정에서 쌍둥이 검사만 뺀 형태. 두 any 의 순서 = 미니언 → 챔프(단락). closure#0 의 소스 표기(`matches!` vs `if let`)는 표기 불가. gen_range 0 · TLS 직접 접촉 0.
```

**`mem` 메모리 접근 11건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | m04.ll:61670~61673 · `<2` 바운드체크(panic_bounds_check 61686) · nexus[team]·twin_towers[team] 인덱스 · `1-team`(61701) | 4 | OK |
| 1 | OperationData | 0x0 | cache | r | m04.ll:61677 | 4 | OK |
| 2 | AbstractGameWithCache | 0x170 | nexus[team] | r | Option<&Entity> null → return false (m04.ll:61679~61683, L241) | 4 | OK |
| 3 | AbstractGameWithCache | 0x148 | twin_towers[team].len | r | 본문 표기 = `gep (cache + team*32), 328`(m04.ll:61691~61695 · bumpalo Vec::is_empty 인라인 vec.rs:1636→len 1617) · len != 0 → return false (61696~61697, L244) | 4 | OK |
| 4 | AbstractGameWithCache | 0x1e0 | player_champion[1-team][0..5] | r | iter_champions 인라인 · 5슬롯 완전 언롤(m04.ll:61711~61712 base · +8 61767 · +16 61809 · +24 61851 · +32 61893) · null 슬롯 skip (L252) | 4 | OK |
| 5 | Entity | 0x4c0 | attack_effect@tag (casting 니치 i32) | r | 적 챔프 · -1 = None 이면 skip (m04.ll:61745~61748, 61787~61790, 61829~61832, 61871~61874, 61913~61916 · option.rs:742 as_ref · L253) | 4 | OK |
| 6 | Entity | 0x490 | attack_effect@Some.0 (&Effect) | r | Effect::is_in_range 의 self (m04.ll:61751/61758 등 5 사이트 · L253 is_some_and option.rs:661) | 4 | OK |
| 7 | Entity | 0x68 | ty@tag (EntityType) | r | aux m06.ll:48066~48068 · `== 1`(Minion) 이 아니면 closure#0 false (L248) | 4 | OK |
| 8 | Entity | 0x5c0 | id (nexus) | r | aux m06.ll:48075~48076 · 클로저 캡처 &nexus → nexus.id (L248) | 4 | OK |
| 9 | Entity | 0x88 | ty@Minion.info.nearest_enemy@tag (Option<usize> · Minion+0x18) | r | aux m06.ll:48078~48081 · trunc→i1 Some 여부 (option.rs:2439 PartialEq 인라인 · L248) | 4 | OK |
| 10 | Entity | 0x90 | ty@Minion.info.nearest_enemy@Some.0 | r | aux m06.ll:48080~48084 · `== nexus.id` · Some 일 때만 유효(select) (L248) | 4 | OK |

**`consts` 상수 5건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 2 | 241 | 길이 | nexus 배열 길이 바운드체크 team<2 (m04.ll:61673) | 4 |
| 1 | 0 | 244 | 태그 | `twin_towers[team].is_empty()` = len == 0 — 쌍둥이 타워가 하나라도 남아 있으면 false (m04.ll:61696) | 4 |
| 2 | 1 | 247 | 인덱스 | `1 - team` = 적 팀 인덱스 (m04.ll:61701 sub nuw nsw) | 4 |
| 3 | 1 | 248 | 태그 | EntityType 태그 1 = Minion — closure#0 의 `matches!(m.ty, Minion(..))` (aux m06.ll:48068, 48831~ 동형) | 4 |
| 4 | -1 | 253 | 태그 | Option<Effect> None 판별값(casting 태그 i32 = -1) — attack_effect 없는 적 챔프 skip (m04.ll:61747, 61789, 61831, 61873, 61915) | 4 |

**`knobs` 조정점 2건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 쌍둥이 전멸 조건 | old/defense_nexus.rs:244 | 0 | `is_empty()` 대신 `len() <= 1` 로 완화하면 쌍둥이 하나 남았을 때도 최후 단계로 봐 방어 집중이 앞당겨진다 | 4 | 기존 |
| 1 | 미니언 조건의 기준 필드 | old/defense_nexus.rs:248 (closure#0) | 1 | nearest_enemy == nexus.id 를 빼면 적 챔프 사거리만으로 최후 단계 — 미니언 웨이브만으로는 발동 안 함 | 4 | 기존 |

<details><summary>`callees` 피호출자 8건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | is_empty | game_core::PatchSetting::is_empty | pub | fn(&game_core::PatchSetting) -> bool | game-core\src\patch_engine.rs:853 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 1 | is_empty | game_core::ModAiRegistry::is_empty | pub | fn(&game_core::ModAiRegistry) -> bool | game-core\src\mod_ai.rs:15 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 2 | is_empty | game_core::PublicChampionEvidence::is_empty | pub | fn(&game_core::PublicChampionEvidence) -> bool | game-core\src\banpick\public_evidence.rs:96 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 3 | is_in_range | game_core::Effect::is_in_range | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\effect.rs:63 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | iter_champions | game_core::AbstractGameWithCache::<'a, 'b>::iter_champions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5759 ~ game_core[6a30]::simulation::{impl#5}::iter_champions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1904 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 5 | iter_minions | game_core::AbstractGameWithCache::<'a, 'b>::iter_minions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5753 ~ game_core[6a30]::simulation::{impl#5}::iter_minions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1847 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 6 | nearest_enemy | game_core::Entity::nearest_enemy | pub | fn(&game_core::Entity) -> std::option::Option<usize> | game-core\src\simulation\entity.rs:1819 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 7 | nexus_final_stand_uncached | game_ai::plan_legacy::old::defense_nexus::nexus_final_stand_uncached | in:game_ai::plan_legacy::old::defense_nexus | fn(&game_core::PlayerState, &game_core::OperationData) -> bool | game-ai\src\plan_legacy\old\defense_nexus.rs:239 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
</details>

⚠**미매칭 1개**: `try_fold`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m00.ll:90537) · **형제 0개** 

**`open` 5건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | AbstractGameWithCache::iter_minions(&self, team: usize) -> impl Iterator<Item=&Entity>(game_core simulation.rs:1847 · sret 56B Chain<Copied<Iter<&Entity>>,Copied<Iter<&Entity>>>) — 어느 두 슬라이스를 체인하는지(팀별 미니언 목록 2개 추정) 미독 · 시그니처만 | 5 |  |
| 1 | 미탐색 | Effect::is_in_range(&self,&Entity,&Entity)->bool 내부 — 계약만(effect.rs:63) | 4 |  |
| 2 | 미탐색 | Copied<Iter<&Entity>>::try_fold 조각(m11.ll:31457~31536, 36239~36283, 36768~36812)과 두 번째 call_mut 심(m06.ll:48831~48872, `&&`·`&` 수신 변종)은 std 플럼빙/동형 — 본체 술어는 m06.ll:48054~48090 과 동일하게 확인(ty==1 · +136/+144 · nexus.id) | 4 |  |
| 3 | 표기 불가 | closure#0 의 소스 표기 — `matches!(m.ty, EntityType::Minion(ref i) if i.nearest_enemy == Some(nexus.id))` 와 `if let … { … } else { false }` 외연 동일 · 표기 불가 | 4 |  |
| 4 | 미탐색 | [D] ai_adjust judge dn_reach DONE(09-06) — 열람 금지 조건에 따라 판정 기록은 대조하지 않음 · 본 명세는 IR 독립 재현 | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

