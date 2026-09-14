---

### `118` base_attacking_minion_uncached — 우리 본진 구조물(넥서스·쌍둥이 타워)을 때리는 적 미니언 중 내 챔피언과 가장 가까운 놈의 id (없으면 None)

| 항목 | 값 |
|---|---|
| id | `defense_nexus__base_attacking_minion_uncached` |
| 심볼 | `_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old13defense_nexus30base_attacking_minion_uncached` |
| 소스 | `game-ai\src\plan_legacy\old\defense_nexus.rs:279` |
| IR | `m04.ll` 62109~62401행 |
| 경로·가시성 | `game_ai::plan_legacy::old::defense_nexus::base_attacking_minion_uncached` · **in:game_ai::plan_legacy::old::defense_nexus** |
| 계층 | 레거시 플랜 |
| exe | `d405d0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r13` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_core::PlayerState, &game_core::OperationData) -> std::option::Option<usize>
```

<details><summary>인자 2개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | player | &PlayerState (2528B) noalias readonly | info.team(0x930)·info.position(0x9c0) 만 읽음 | 4 |
| 1 | 2 | data | &OperationData (24B) noalias readonly | cache(+0) 만 읽음. context/blackboard 미사용 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn base_attacking_minion_uncached(player, data) -> Option<usize> {   // defense_nexus.rs:279
  let team = player.info.team;                                                        // :280 L62123
  let nexus = data.cache.nexus[team]?;                                                 // :281 L62131~62136 (None → 반환 None, slot1 undef)
  let champ = data.cache.player_champion[team][player.info.position.as_index()]?;     // :282 L62145~62154
  let twins = &data.cache.twin_towers[team];                                            // :283 L62165~62166 (bumpalo Vec<&Entity>)
  let is_structure = |id: usize| id == nexus.id || twins.iter().any(|t| t.id == id);   // :284 closure#0 (aux L66255~66303 인라인) — 넥서스 먼저, 없으면 쌍둥이 타워 선형 any
  data.cache.iter_minions(1 - team)                                                     // :285 L62173~62174 적 미니언 top→mid→bottom Chain (56B 이터레이터)
      .filter(|m| m.ty@tag == 1 /*Minion*/ && m.ty@Minion.info.nearest_enemy.is_some_and(|id| is_structure(id)))   // :286 closure#1 (aux L66224~66306) — '지금 우리 구조물을 때리는(=nearest_enemy 가 구조물 id) 미니언'
      .map(|m| (champ.distance_sq(m), m))                                              // :287 closure#2 — 첫 원소는 본문 L62347~62375, 나머지는 aux(m11) L23478~23506 인라인. distance_sq = |dx|²+|dy|² (utils.rs:7~9, abs_diff 후 wrapping mul)
      .min_by_key(|(d, _)| *d)                                                          // :287 reduce → Map::fold(m12.ll:16716) → Chain::fold(m11 L23303~) ; compare = Ord::cmp(acc.0, new.0), acc 유지 조건 is_le (L23520~23524) ⇒ 동거리면 **먼저 나온 것(top→mid→bottom 순)** 유지
      .map(|(_, m)| m.id)                                                               // :288 closure#3 L62392~62393 (Entity+0x5c0)
}   // :289 L62159~62161 {tag, id}

[구현 메모] Filter 의 첫 원소 탐색은 세 구간(top/mid/bottom)을 각각 try_fold(find::check) 인스턴스로 호출(L62231·L62274·L62319, m11.ll 33409/34234/34729) 하고 술어는 closure#1::call_mut(Q/QQ/QQQ) 로 아웃라인. 이후 원소는 Map<Filter<Chain>>::fold 한 번(L62380).
[콜리 계약] iter_minions(&cache, team) -> Chain<Chain<Copied<Iter<&Entity>>,Copied<Iter>>,Copied<Iter>> (sret 56B) = top_minions[team] ⧺ mid_minions[team] ⧺ bottom_minions[team] (g15.ll:102624~)
```

**`mem` 메모리 접근 13건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | L62123~62124 (bounds <2 L62126/L62139) | 4 | OK |
| 1 | PlayerState | 0x9c0 | info.position@tag → as_index() | r | L62145~62147 entity.rs:581 인라인, i32 태그를 그대로 인덱스로 | 4 | OK |
| 2 | OperationData | 0x0 | cache (&AbstractGameWithCache) | r | L62130 | 4 | OK |
| 3 | AbstractGameWithCache | 0x170 | nexus[team] (Option<&Entity>, null=None) | r | L62131~62133 gep 368 + team*8 (tcxdict 0x170) | 3 | OK |
| 4 | AbstractGameWithCache | 0x1e0 | player_champion[team][pos] (Option<&Entity>) | r | L62148~62151 gep 480 + team*40 + pos*8 | 4 | OK |
| 5 | AbstractGameWithCache | 0x130 | twin_towers[team] (bumpalo Vec<&Entity> 32B: +0 buf.ptr, +0x18 len) | r | L62165~62166 gep 304 + team*32 → &Vec 을 is_structure 클로저에 캡처(L62169~62171). aux L66261~66277 에서 ptr(+0)·len(+24) 읽어 순회 | 4 | OK |
| 6 | AbstractGameWithCache | 0x10/0x50/0x90 | top/mid/bottom_minions[1-team] (콜리 iter_minions 내부) | r | L62174 iter_minions(cache, 1-team) — 본문 g15.ll:102624: top(0x10)→mid(0x50)→bottom(0x90) 순 Chain | 4 | 오귀속(사전은 다른 필드를 준다) |
| 7 | Entity | 0x5c0 | id | r | 본문 L62392~62393 결과 미니언 id(closure#3 :288) · aux L66255~66257/L66300~66302 넥서스·타워 id 비교(is_structure :284) | 4 | OK |
| 8 | Entity | 0x660 | x | r | L62347~62348(내 챔피언)·L62355~62356(미니언) distance_sq(entity.rs:2158 → utils.rs:7~9) closure#2 :287 | 4 | OK |
| 9 | Entity | 0x668 | y | r | L62351~62352 / L62359~62360 | 4 | OK |
| 10 | Entity | 0x68 | ty@tag (EntityType) | r | aux L66224~66226 `== 1` Minion (closure#1 :286) | 4 | OK |
| 11 | Entity | 0x88 | ty@Minion.info.nearest_enemy@tag (Option<usize>) | r | aux L66230~66236 is_some_and (tcxdict Entity 0x88) | 3 | OK |
| 12 | Entity | 0x90 | ty@Minion.info.nearest_enemy@Some.0 (usize = 공격 대상 id) | r | aux L66243~66244 → is_structure(id) | 4 | OK |

**`consts` 상수 3건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 2 | 281 | 임계 | nexus/player_champion 1차원(팀) 길이 bounds check L62126 / L62139 panic len 2. 판정값 아님 | 4 |
| 1 | 1 | 285 | 태그 | `1 - team` = 적 팀(L62173 `sub nuw nsw i64 1, %10`) → iter_minions(적 미니언). ★aux 에서는 EntityType::Minion 메모리태그 1 (L66226 `icmp eq i64 %7, 1`, tcxdict --enum EntityType idx1=tag1) | 3 |
| 2 | 0 | 289 | 태그 | 반환 Option 태그 None (L62158/L62398 phi 0) | 4 |

**`knobs` 조정점 2건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 구조물 판정 집합(넥서스 + 쌍둥이 타워) | defense_nexus.rs:284 (aux L66255~66303) | nexus.id ∪ twin_towers[team][*].id | 여기에 1차 타워를 넣으면 라인 타워를 미는 미니언도 '본진 정리 대상'이 돼 넥서스 방어 플랜이 라인까지 나감 | 4 | 기존 |
| 1 | 선택 키 = 내 챔피언과의 제곱거리 최소 | defense_nexus.rs:287 (m11 L23520~23524) | distance_sq 오름차순, 동률=먼저 나온 것(top→mid→bottom) | 예: 넥서스와의 거리로 바꾸면 '가장 깊이 들어온' 미니언부터 정리 | 4 | 기존 |

<details><summary>`callees` 피호출자 7건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | as_index | game_core::BigGoal::as_index | pub | fn(&game_core::BigGoal) -> usize | game-core\src\simulation\ai_interface.rs:338 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
| 1 | as_index | game_core::Position::as_index | pub | fn(&game_core::Position) -> usize | game-core\src\simulation\entity.rs:580 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
| 2 | base_attacking_minion_uncached | game_ai::plan_legacy::old::defense_nexus::base_attacking_minion_uncached | in:game_ai::plan_legacy::old::defense_nexus | fn(&game_core::PlayerState, &game_core::OperationData) -> std::option::Option<usize> | game-ai\src\plan_legacy\old\defense_nexus.rs:279 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 3 | distance_sq | game_core::utils::distance_sq | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:6 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
| 4 | distance_sq | game_core::Entity::distance_sq | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2157 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
| 5 | is_structure | game_core::EntityType::is_structure | pub | fn(&game_core::EntityType) -> bool | game-core\src\simulation\entity.rs:1391 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 6 | iter_minions | game_core::AbstractGameWithCache::<'a, 'b>::iter_minions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5753 ~ game_core[6a30]::simulation::{impl#5}::iter_minions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1847 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
</details>

⚠**미매칭 3개**: `is_le`, `llvm.memcpy.p0.p0.i64`, `try_fold`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 3곳** (m00.ll:90541, m04.ll:59507, m04.ll:61387) · **형제 0개** 

**`open` 3건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 표기 불가 | min_by_key 의 key 클로저 소스 표기(`\|(d,_)\| *d` 인지 `\|t\| t.0` 인지) — 인라인이라 표기 불가. 동작(튜플 0번 u64 비교, acc 유지 is_le)은 aux L23520~23524 로 확정 | 4 |  |
| 1 | 미탐색 | docs 는 'O(1) 판정' 함수를 따로 언급(game_ai.txt:302)하지만 이 함수는 목록 스캔판(_uncached) — 캐시판 래퍼는 이 명세 밖 | 4 |  |
| 2 | 미탐색 | QQ/QQQ 술어 인스턴스(m04 66614~66711, 66714~66816)는 md5 가 Q 와 다르나 참조 중첩(load 추가)만 다르다고 판단 — 줄 단위 전수 대조는 안 함(범위: 구조 비교만) | 4 |  |

**`notes` 1건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | closure#0(is_structure)의 `\|\|` 순서 — L66255~66258 에서 nexus.id 비교가 먼저이고 실패 시 twins.any 로 가는 것은 확정. 소스 한 줄 안 순서는 column 0 이라 IR 로만 확정 | 4 | 사실 서술 |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

