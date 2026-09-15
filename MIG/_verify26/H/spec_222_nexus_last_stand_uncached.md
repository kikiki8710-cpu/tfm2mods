---

### `222` nexus_last_stand_uncached — 본진 결사(last stand) 판정: 적 챔프의 평타가 우리 넥서스·쌍둥이에 닿으면 true / 아니면 (적 챔프 평타가 나에게 닿음 AND 본진구조물 타격 미니언 존재) — 넥서스 없음·내 챔프 없음이면 false

| 항목 | 값 |
|---|---|
| id | `defense_nexus__nexus_last_stand_uncached` |
| 심볼 | `_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old13defense_nexus25nexus_last_stand_uncached` |
| 소스 | `game-ai\src\plan_legacy\old\defense_nexus.rs:205` |
| IR | `m04.ll` 60704~61396행 |
| 경로·가시성 | `game_ai::plan_legacy::old::defense_nexus::nexus_last_stand_uncached` · **in:game_ai::plan_legacy::old::defense_nexus** |
| 계층 | 레거시 플랜 |
| exe | `d3e660` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r17` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_core::PlayerState, &game_core::OperationData) -> bool
```

**TLS 메모 절(`sig.tls` · 정정 경로 `/specs[222]/sig/tls/<키>`)**

- `name`: LAST_STAND_MEMO (game_ai::plan_legacy::old::defense_nexus::LAST_STAND_MEMO · m04.ll:262 `@anon.168add0ea037d45d276f5936ae758fe5.248 = constant ptr @<LAST_STAND_MEMO…call_once>`)
- `role`: 값 생산자(간접). 본 함수 자체는 TLS 미접촉(본문 @anon 참조 1개 = panic Location .260 · LocalKey::with 0). 작성자 = last_stand_flags::{closure#0}(old/defense_nexus.rs:166~183 · 본체 m00.ll:90215~90610 · LocalKey::with 호출부 = nexus_last_stand m04.ll:57036) 이 본 함수를 호출해 튜플 .0 에 저장. 소비자 = nexus_last_stand(defense_nexus.rs:186 · m04.ll:57003 · `.0`) · nexus_final_stand(190 · 57045 · `.1`) · base_defense_focus(200 · 57133 · `.2` 추정—본문 미독)
- `key`: player.info.id (PlayerState+0x928 · m00.ll:90277 gep 2344 · HashMap<usize,(bool,bool,bool)> ahash RandomState)
- `layout`: RefCell<(usize seed, usize tick, HashMap<usize,(bool,bool,bool)>)> — +0 borrow flag · +8 seed(m00.ll:90254~90257) · +16 tick(90269~90271) · +24 HashMap(ctrl ptr +24 · bucket_mask +32 · items +48). 값 i24: byte0=last_stand(본 함수) · byte1=final_stand · byte2=base_attacking_minion_uncached(...).is_some() (m00.ll:90555~90561 insert)
- `invalidation`: seed(AbstractGame vtable+0x20 seed()) 또는 tick(vtable+0x28 tick()) 이 메모와 다르면 seed·tick 갱신 후 HashMap.clear() (m00.ll:90257~90273, 90491~90499 · defense_nexus.rs:171~174) → 같은 시드·같은 틱 안에서는 플레이어당 1회만 계산
- `call_conditions`: 메모 miss(HashMap.get(&player.info.id) None · m00.ll:90291/90471→%104) 일 때 순서 = ①nexus_last_stand_uncached(90533) ②nexus_final_stand_uncached(90537) ③base_attacking_minion_uncached(90541) → insert(90562). hit 이면 세 함수 모두 호출 안 됨(90512~90529). RefCell 재진입 시 panic_already_borrowed(90265)

<details><summary>인자 2개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | player | &PlayerState (2528B) | IR `readonly captures(none)` · info.team(0x930)·info.position(0x9c0) 읽음 · base_attacking_minion_uncached 1번 인자 | 4 |
| 1 | 2 | data | &OperationData (24B) | IR `readonly captures(none)` · cache(0x0)만 읽음(context·blackboard 미참조) | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn nexus_last_stand_uncached(player, data) -> bool   [old/defense_nexus.rs:205]
  team = player.info.team                                                        [L206 · m04.ll:60717~60718]
  let Some(nexus) = data.cache.nexus[team] else { return false }                 [L207 · 60720~60731 · bounds<2]
  twins = &data.cache.twin_towers[team]      // bumpalo Vec<&Entity>            [L210 · 60745~60747]
  enemy = 1 - team                                                               [L213 · 60742]
  // ① 적 챔프 평타가 우리 넥서스나 쌍둥이 중 하나에 닿는가 (closure#0 L213:46 · closure#0::closure#0 L215:51)
  if data.cache.iter_champions(enemy).any(|c| {                                  [L213 · 5슬롯 언롤 60749~61172]
        let Some(atk) = c.attack_effect.as_ref() else { return false };          [L214 · tag 0x4c0 != -1]
        atk.is_in_range(c, nexus) || twins.iter().any(|t| atk.is_in_range(c, t))  [L215 · 60797 / 60822~60840 슬라이스 루프]
     }) { return true }                                                          [→ %190 true]
  // ② 내 챔프
  let Some(me) = data.cache.player_champion[team][player.info.position] else { return false }   [L224 · 61181~61188]
  // ③ 적 챔프 평타가 나에게 닿는가 (closure#1 L227:47 · closure#1::closure#0 L228:44)
  if !data.cache.iter_champions(enemy).any(|c| c.attack_effect.as_ref().is_some_and(|atk| atk.is_in_range(c, me))) { return false }   [L227~228 · 5슬롯 언롤 61193~61384 · option.rs:661 is_some_and]
  // ④ 본진 구조물을 때리는 적 미니언이 있는가
  return base_attacking_minion_uncached(player, data).is_some()                  [L233~234 · 61387~61395]

주의: ①은 넥서스·쌍둥이 '사거리 안' 판정이지 실제 공격 중 여부가 아님(Effect::is_in_range(&self, caster, target) game_core effect.rs:63 — 내부 미독). ②~④ 경로는 '적 챔프가 나를 칠 수 있는 거리 + 미니언이 본진 구조물 타격 중' 의 AND. 5슬롯 언롤은 슬롯 순서(0→4)로 단락 평가 · 결과는 순서 무관(any). 언롤 각 슬롯의 null 은 iter_champions 의 filter_map(None skip). gen_range 0 · TLS 직접 접촉 0(signature.tls 참조).
```

**`mem` 메모리 접근 9건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | m04.ll:60717~60720 · `<2` 바운드체크(panic_bounds_check 60734) · nexus[team]·twin_towers[team]·player_champion[team] 인덱스 · `1-team` 적 팀(60742) | 4 | OK |
| 1 | PlayerState | 0x9c0 | info.position@tag | r | i32 zext · player_champion[team][pos] 2차 인덱스 (m04.ll:61181~61185, L224) | 4 | OK |
| 2 | OperationData | 0x0 | cache | r | m04.ll:60724 | 4 | OK |
| 3 | AbstractGameWithCache | 0x170 | nexus[team] | r | Option<&Entity> 니치 null → return false (m04.ll:60727~60731, L207) | 4 | OK |
| 4 | AbstractGameWithCache | 0x130 | twin_towers[team].buf.ptr (bumpalo Vec<&Entity>, 32B stride) | r | m04.ll:60745~60747, 60803 (L210) · 쌍둥이 타워 슬라이스 시작 | 4 | OK |
| 5 | AbstractGameWithCache | 0x148 | twin_towers[team].len | r | 본문엔 절대 오프셋이 아니라 twins(=cache+0x130+team*32) 기준 `gep %16, 24`(m04.ll:60766) 로만 나타남(C3 경고 사유) · 60805 load · 슬라이스 끝 = ptr + len*8 (60814) | 4 | OK |
| 6 | AbstractGameWithCache | 0x1e0 | player_champion[1-team][0..5] / player_champion[team][pos] | r | [2][5] Option<&Entity>(null=None). 적 팀 5슬롯은 루프 완전 언롤(m04.ll:60749~60750 base · +8 60848 · +16 60931 · +24/+32 이하 동형) — iter_champions(=filter_map(\|c\| *c)) 인라인 · 내 챔프 = [team][pos] (61184~61188, null→false) | 4 | OK |
| 7 | Entity | 0x4c0 | attack_effect@tag (casting 니치 i32) | r | 적 챔프 · -1 = None 이면 그 챔프 skip (m04.ll:60790~60793 등 10 사이트 · option.rs:742 as_ref 인라인 · L214/L228) | 4 | OK |
| 8 | Entity | 0x490 | attack_effect@Some.0 (&Effect) | r | Effect::is_in_range 의 self (m04.ll:60789, 60872 … · L215/L228) | 4 | OK |

**`consts` 상수 4건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 2 | 207 | 길이 | nexus 배열 길이 바운드체크 team<2 (m04.ll:60720) | 4 |
| 1 | 1 | 213 | 태그 | `1 - team` = 적 팀 인덱스 (m04.ll:60742 sub nuw nsw) | 4 |
| 2 | -1 | 214 | 태그 | Option<Effect> None 판별값(casting 태그 i32 = -1) — attack_effect 없는 적 챔프는 후보에서 제외 (m04.ll:60792, 60875, 60958, … L214 · 61212, 61252, 61292, 61332, 61372 L228) | 4 |
| 3 | 1 | 233 | 태그 | base_attacking_minion_uncached 반환 Option<usize> 의 {i64,i64} 첫 워드 == 1 → Some = is_some() (m04.ll:61388~61390, option.rs:430) | 4 |

**`knobs` 조정점 2건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 결사 판정 사거리 기준 | old/defense_nexus.rs:215/228 (Effect::is_in_range · game_core effect.rs:63) | 0 | 본 함수엔 수치 임계가 없다 — 사거리는 적 챔프 attack_effect 의 range 로 결정. 여유 거리를 주려면 is_in_range 대신 거리²+마진 비교로 바꿔야 한다(구조 변경) | 4 | 기존 |
| 1 | ③ 조건 대상 | old/defense_nexus.rs:227~228 | 0 | '적 평타가 나에게 닿음' 을 빼면(항상 true) 미니언이 본진을 때리기만 해도 결사가 되어 방어 집중 모드가 넓어진다 · ④를 빼면 적 챔프 근접만으로 결사 | 4 | 기존 |

<details><summary>`callees` 피호출자 4건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | base_attacking_minion_uncached | game_ai::plan_legacy::old::defense_nexus::base_attacking_minion_uncached | in:game_ai::plan_legacy::old::defense_nexus | fn(&game_core::PlayerState, &game_core::OperationData) -> std::option::Option<usize> | game-ai\src\plan_legacy\old\defense_nexus.rs:279 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | is_in_range | game_core::Effect::is_in_range | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\effect.rs:63 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 2 | iter_champions | game_core::AbstractGameWithCache::<'a, 'b>::iter_champions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5759 ~ game_core[6a30]::simulation::{impl#5}::iter_champions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1904 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 3 | nexus_last_stand_uncached | game_ai::plan_legacy::old::defense_nexus::nexus_last_stand_uncached | in:game_ai::plan_legacy::old::defense_nexus | fn(&game_core::PlayerState, &game_core::OperationData) -> bool | game-ai\src\plan_legacy\old\defense_nexus.rs:205 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
</details>

**호출처 1곳** (m00.ll:90533) · **형제 0개** 

**`open` 7건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | Effect::is_in_range(&self, &Entity caster, &Entity target)->bool 내부(game_core effect.rs:63) — 계약만 · 사거리 보정(range_adjust)·히트박스 포함 여부 미독 | 4 |  |
| 1 | 미탐색 | base_attacking_minion_uncached(player,data)->Option<usize>(정본 r13 잎) — 여기서는 is_some() 만 | 3 |  |
| 2 | 미탐색 | AbstractGameWithCache::iter_champions(team) 의 정확한 소스 형태 — IR 은 `player_champion[team].iter().filter_map(\|c\| *c)` 인라인(impl$5::iter_champions::closure$0)으로 확정 · 정의 위치(game_core) 미조회 | 4 |  |
| 3 | 표기 불가 | closure#0(L214~215) 의 소스 표기 — `let Some(atk)=… else {return false}` 인지 `map_or(false, …)` 인지 표기 불가(외연 동일). closure#1(L228) 은 option.rs:661 인라인으로 `is_some_and` 확정 | 4 |  |
| 4 | 미탐색 | L215 `\|\|` 의 좌우 순서는 IR 분기 순서(nexus 먼저 60797 → twins 60822~)로 확정 · 소스 표기도 같은 순서일 수밖에 없음(단락 평가) | 4 |  |
| 5 | 미탐색 | twin_towers 원소 순서·개수(보통 2)는 cache 구성(game_core) 소관 — 본 함수는 len 만큼 순회 | 4 |  |
| 6 | 미탐색 | base_defense_focus(old/defense_nexus.rs:200 · m04.ll:57133) 가 튜플 .2 를 돌려주는지 — 본문 미독(추정 · 본 배치 범위 밖) | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

