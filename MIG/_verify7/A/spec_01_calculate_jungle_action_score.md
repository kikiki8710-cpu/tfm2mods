---

### `01` calculate_jungle_action_score — 정글러의 액션 대상(t)을 대상 종류별 계수 x 예상피해/HP 비율로 점수화

| 항목 | 값 |
|---|---|
| id | `action_score__calculate_jungle_action_score` |
| 심볼 | `_RNvNtCshdEBA0ozCnw_7game_ai12action_score29calculate_jungle_action_score` |
| 소스 | `game-ai\src\action_score.rs:520` |
| IR | `m05.ll` 39984~40103행 |
| 경로·가시성 | `game_ai::calculate_jungle_action_score` · **pub** |
| 계층 | 점수화·술어 |
| exe | `d5ba80` (action_score) · 366바이트 · 93명령 |
| 라운드 | 기준 `r4` · 통과 4회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &std::boxed::Box<dyn [Binder { value: Trait(game_core::Action), bound_vars: [] }] + 'static, std::alloc::Global>, &game_core::Effect, &game_core::Entity) -> i64
```

<details><summary>인자 7개</summary>

| # | i | 이름 | 타입 | 역할 |
|---|---|---|---|---|
| 0 | 1 | _rnd |  | 본문에서 전혀 안 씀(소스에서도 `_` 접두 = 미사용 확정) |
| 1 | 2 | player |  | info.team, info.position 두 필드만 읽어 챔피언 엔티티를 찾는 용도 |
| 2 | 3 | data |  | cache(+0x0), context(+0x8) 사용. blackboard(+0x10)는 안 씀 |
| 3 | 4 | _parameter |  | 본문에서 전혀 안 씀 — 이 함수의 계수는 전부 하드코딩 리터럴 |
| 4 | 5 | _action |  | 본문에서 전혀 안 씀 |
| 5 | 6 | effect |  | expected_damage_target 의 self 로 통째로 전달. 필드 접근 없음 |
| 6 | 7 | t |  | 평가 대상 엔티티. ty(+0x68) / ty.Jungle.info.camp_type.0(+0x98) / hp(+0x670) 를 읽음 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
순수 함수(store 0개, 모든 인자 readonly/readnone). 부작용은 panic 뿐.

// :522
team = player.info.team // PlayerState+0x930, usize
if team >= 2 { panic_bounds_check }
pos = player.info.position.as_index() // PlayerState+0x9c0, entity.rs:580-581, 0..=4
champ = data.cache.player_champion[team][pos] // AbstractGameWithCache+0x1e0, [[Option<&Entity>;5];2]
if champ.is_none() { Option::unwrap 실패 panic }

// :523 champ 는 &dyn AbstractEntity 로 코어싱돼 (데이터ptr, vtable=anon.17) 2워드로 전달됨.
// anon.17 = Entity as AbstractEntity 의 vtable(divtable 로 확인: slot0=drop_glue, size 0x6c0, slot0x18=x)
value = effect.expected_damage_target(data.context, champ as &dyn AbstractEntity, t) // i64

// :524
hp = t.hp // Entity+0x670

// :526~536 else-if 체인이 EntityType 판별자(Entity+0x68) switch 하나로 접힘.
// 소스 순서는 !dbg 줄번호로 복원: 526 → 528 → 530/531 → 536
coef = if t.ty.is_nexus() { 200 } // :526 tag 3
 else if t.ty.is_tower() { 80 } // :528 tag 2
 else if t.ty.is_jungle(0) || t.ty.is_jungle(1) { // :530 tag 4 && camp_type.0 in {0,1} = 양 팀 캠프 전부 (IR 의 `ult 2` 는 `x==0||x==1` 로 접힌 것)
 if hp <= value { 40 } else { 20 } // ★then/else 반전형 확정 — L531=20자라 `if hp > value {`(19자)는 길이가 안 맞는다 // :531 signed 비교(icmp sgt), select
 }
 else if t.ty.is_any_type_minion() { -10 } else { 0 }; // :536 tag 1 / 그 외 전부
// ⚠ tag 4(Jungle) 의 camp_type.0 은 팀 인덱스라 {0,1} 뿐이다. 소스는 `is_jungle(0)||is_jungle(1)`(entity.rs:1377 동등 비교)이고 IR 의 `ult 2` 는 그 둘이 접힌 것이다 
// ⚠ Champion(tag 13)·Epic(5)·Serpen(6) 등도 전부 coef = 0 → 점수 0

// :542 같은 `is_jungle(0)||is_jungle(1)` 를 한 번 더 평가(판별자 재비교 + camp_type.0 재로드)
based = if t.ty.is_jungle(0) || t.ty.is_jungle(1) { 5 } else { 0 }; // L542=57자 ±0

// :548 hp == 0 이면 panic_const_div_by_zero, (hp == -1 && coef*value == i64::MIN)이면 panic_const_div_overflow
return min(coef, coef * value / hp) + based; // sdiv(내림 아님 — 0 방향 절삭), Ord::min(cmp.rs:1078)

// 의미: coef * (예상피해/대상HP) 를 coef 로 상한. 즉 '치명타 한 방에 잡을 수 있으면 계수 만점,
// 아니면 깎아내리는 비율 점수'. 곱셈이 먼저라 정수 절삭 손실은 작다.
// ⚠ coef 가 음수(Minion, -10)인 경우 min 의 방향이 뒤집힌다: 예상피해가 HP 를 넘길수록
// coef*value/hp 가 더 작아져(더 음수) min 이 그걸 고른다 = 미니언 오버킬이 더 큰 감점.
// 반대로 피해가 HP 미만이면 항상 -10 으로 클램프된다.
```

**`mem` 메모리 접근 8건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev |
|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | usize. 2 미만 아니면 panic_bounds_check(len=2) — player_champion 첫 인덱스 · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 |
| 1 | PlayerState | 0x9c0 | info.position | r | Position(i32, range 0..5). Position::as_index()(entity.rs:580-581)로 0..=4 인덱스화 — player_champion 둘째 인덱스 · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 |
| 2 | OperationData | 0x0 | cache | r | &AbstractGameWithCache · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 |
| 3 | OperationData | 0x8 | context | r | &Context(64B). expected_damage_target 2번째 인자로 전달 · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 |
| 4 | AbstractGameWithCache | 0x1e0 | player_champion | r | [[Option<&Entity>;5];2] (80B). [team][position] 로 인덱싱해 champ 획득. None 이면 Option::unwrap 실패 panic · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 |
| 5 | Entity | 0x68 | ty (EntityType 판별자) | r | i64, range 0..14. switch 로 1=Minion / 2=Tower / 3=Nexus / 4=Jungle 분기. 나머지(0 None, 5 Epic, 6 Serpen, 7 Ghoul, 8 SmallJiangshi, 9 Bear, 10 Eagle, 11 Revenant, 12 Illusion, 13 Champion)는 default · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 |
| 6 | Entity | 0x98 | ty.Jungle.info.camp_type.__0 | r | tuple(usize, JungleType) 의 usize 쪽 = **팀 인덱스(0 블루 / 1 레드)**. 소스는 `is_jungle(0) \|\| is_jungle(1)`(인자와 **동등 비교**, entity.rs:1377 `fn(&EntityType, usize) -> bool`)이며, IR 의 `ult 2` 는 그 동등 비교 둘이 LLVM 에서 접힌 형태다 · tcx 정본 대조( A6_o6.tsv(실주소 차 보충, 10/10 MATCH) JUNGLE team0→+0x98=0 / team1→+0x98=1 (camp_type.__0 = 팀 인덱스)) | 3 |
| 7 | Entity | 0x670 | hp | r | usize 필드지만 i64 로 로드해 signed 비교/sdiv 에 사용. 0 이면 panic_const_div_by_zero · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 |

**`consts` 상수 12건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 2 | 522 | 임계 | player_champion 첫 축 길이 = 2(팀 수). team >= 2 면 panic_bounds_check | 4 |
| 1 | 3 | 526 | 태그 | EntityType 판별자 3 = Nexus (is_nexus(), entity.rs:1399-1400 인라인) · 오라클 실행 확증( A6_o3.tsv SCORE/HPSWEEP — Nexus tag=3 → coef 200, game==mine MATCH) | 2 |
| 2 | 200 | 526 | 임계 | 대상이 Nexus 일 때의 계수 coef — 이 함수 최대 계수 · 오라클 실행 확증( A6_o3.tsv SCORE/HPSWEEP — Nexus 2개 전부 game=200 MATCH) | 2 |
| 3 | 2 | 528 | 태그 | EntityType 판별자 2 = Tower · 오라클 실행 확증( A6_o3.tsv SCORE/HPSWEEP — Tower tag=2 → coef 80 MATCH) | 2 |
| 4 | 80 | 528 | 임계 | 대상이 Tower 일 때의 계수 coef · 오라클 실행 확증( A6_o3.tsv SCORE/HPSWEEP — Tower 2개 전부 game=80 MATCH) | 2 |
| 5 | 4 | 530 | 태그 | EntityType 판별자 4 = Jungle · 오라클 실행 확증( A6_o3.tsv SCORE/HPSWEEP — Jungle tag=4 12개 전부 MATCH) | 2 |
| 6 | 20 | 531 | 임계 | Jungle 이면서 hp > value(예상피해)일 때의 계수 = 이번 이펙트로 못 잡는 캠프 ★소스 L531 은 **`if hp <= value { 40 } else { 20 }` 형태**(20 이 else-arm) — rmeta_srcmap L531=20자. 어느 표기인지(`hp <= value` / `value >= hp`, 둘 다 11자)는 **표기 불가**이나 **arm 반전 구조는 확정** · 오라클 실행 확증( A6_o3.tsv SCORE/HPSWEEP — HPSWEEP hp=497 이상에서 coef 20 MATCH) | 2 |
| 7 | 40 | 531 | 임계 | Jungle 이면서 hp <= value 일 때의 계수 = 이번 이펙트로 처치 가능한 캠프(두 배 우대) ★소스 L531 은 **`if hp <= value { 40 } else { 20 }` 형태**(20 이 else-arm) — rmeta_srcmap L531=20자. 어느 표기인지(`hp <= value` / `value >= hp`, 둘 다 11자)는 **표기 불가**이나 **arm 반전 구조는 확정** · 오라클 실행 확증( A6_o3.tsv SCORE/HPSWEEP — HPSWEEP hp=496 경계에서 40 채택 — `hp<value` 대립가설 25 를 기각(MATCH 판별)) | 2 |
| 8 | 1 | 536 | 태그 | EntityType 판별자 1 = Minion | 4 |
| 9 | -10 | 536 | 임계 | 대상이 Minion 일 때의 계수 — 정글러가 미니언을 때리는 것에 대한 감점 | 4 |
| 10 | 0 | 536 | 임계 | 그 외 모든 EntityType(Champion 포함!)의 계수 = 0 → 점수 0 · 오라클 실행 확증( A6_o3.tsv SCORE/HPSWEEP — Champion tag=13 → coef 0, game=0 MATCH) | 2 |
| 11 | 5 | 542 | 임계 | based 보너스. is_jungle() 이면 최종 점수에 +5 (동점 시 정글 캠프 우선) · 오라클 실행 확증( A6_o3.tsv SCORE/HPSWEEP — Jungle 만 based=5 가 붙고 Tower/Nexus/Champion 은 0 — 20행 MATCH) | 2 |

**`knobs` 조정점 7건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev |
|---|---|---|---|---|---|
| 0 | Nexus 계수 | action_score.rs:526 (m05.ll:40016, phi [200, %22]) | 200 | 올리면 정글러가 넥서스 타격 액션을 다른 모든 대상보다 더 강하게 선호. 내리면 넥서스 다이브를 덜 함. 상한 205 의 대부분이 이 값이라 리턴 range 도 같이 움직인다 · 오라클 실행 확증( A6_o3.tsv SCORE 20행 game==mine — Nexus 200) | 2 |
| 1 | Tower 계수 | action_score.rs:528 (m05.ll:40016, phi [80, %37]) | 80 | 올리면 정글러가 포탑 공격을 정글 캠프(20/40)보다 더 우선. 40 이하로 내리면 포탑보다 캠프를 먼저 친다 · 오라클 실행 확증( A6_o3.tsv SCORE 20행 game==mine — Tower 80) | 2 |
| 2 | Jungle 캠프 계수(처치 불가) | action_score.rs:531 (m05.ll:40045, select false-arm) | 20 | 올리면 이번 이펙트로 못 잡는 캠프도 계속 때리게 됨. 내리면 딜이 모자란 캠프를 빨리 포기 · 오라클 실행 확증( A6_o3.tsv SCORE 20행 game==mine — Jungle 20(처치불가)) | 2 |
| 3 | Jungle 캠프 계수(처치 가능) | action_score.rs:531 (m05.ll:40045, select true-arm) | 40 | 올리면 막타 낼 수 있는 캠프를 더 집요하게 마무리. 80 이상이면 포탑보다 캠프 막타를 우선하게 된다 · 오라클 실행 확증( A6_o3.tsv SCORE 20행 game==mine — Jungle 40(처치가능)) | 2 |
| 4 | Minion 계수(감점) | action_score.rs:536 (m05.ll:40016, phi [-10, %46]) | -10 | 0 으로 올리면 정글러가 미니언을 쳐도 무감점(라인 개입/막타 스틸 증가). 더 내리면 미니언 근처를 강하게 회피. 음수라 min 방향이 뒤집혀 오버킬일수록 감점이 커지는 부작용도 같이 커진다 | 4 |
| 5 | 정글 보너스 based | action_score.rs:542 (m05.ll:40085, select ? 5 : 0) | 5 | 올리면 계수가 같거나 비슷한 대상들 사이에서 정글 캠프가 항상 이김. 40 이상으로 올리면 캠프가 포탑을 추월 · 오라클 실행 확증( A6_o3.tsv SCORE 20행 game==mine — based 5) | 2 |
| 6 | 정글 캠프의 **소유 팀** 게이트 | action_score.rs:530 · 542 — 소스는 `t.ty.is_jungle(0) \|\| t.ty.is_jungle(1)`(줄길이 52자/57자 ±0 검산). IR `icmp ult camp_type.__0, 2`(m05.ll:40059~40063)는 **그 OR 의 LLVM 접힘**이지 소스 임계가 아니다 |  | `is_jungle(player.info.team)` 으로 바꾸면 **아군 진영 캠프만**, `is_jungle(1 - team)` 이면 **카운터정글 전용**이 된다(camp_type.__0 = 팀 인덱스). 정글 스코어 가산 +5(m05.ll:40092)도 같이 따라간다. `camp_type.__0` 은 팀 인덱스(0/1)이고, `EntityType::is_jungle(&self, usize)`(entity.rs:1377)는 **인자와의 동등 비교**라 상수 2 는 소스에 없다 · 오라클 실행 확증( A6_o3.tsv SCORE 20행 game==mine — camp0=0(블루)·camp0=1(레드) 캠프가 **둘 다** based=5 를 받는다 = `is_jungle(0)\|\|is_jungle(1)`) | 2 |

<details><summary>`callees` 피호출자 8건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 |
|---|---|---|---|---|---|
| 0 | as_index | game_core::Position::as_index | pub | fn(&game_core::Position) -> usize | game-core\src\simulation\entity.rs:580 |
| 1 | as_index | game_core::BigGoal::as_index | pub | fn(&game_core::BigGoal) -> usize | game-core\src\simulation\ai_interface.rs:338 |
| 2 | expected_damage_target | game_core::Effect::expected_damage_target | pub | fn(&game_core::Effect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + , &game_core::Entity) -> usize | game-core\src\simulation\effect.rs:91 |
| 3 | expected_damage_target | game_core::Projectile::expected_damage_target | pub | fn(&game_core::Projectile, &game_core::GameContext, &game_core::Entity, &game_core::Entity) -> usize | game-core\src\simulation\projectile.rs:1287 |
| 4 | is_any_type_minion | game_core::EntityType::is_any_type_minion | pub | fn(&game_core::EntityType) -> bool | game-core\src\simulation\entity.rs:1260 |
| 5 | is_jungle | game_core::EntityType::is_jungle | pub | fn(&game_core::EntityType, usize) -> bool | game-core\src\simulation\entity.rs:1377 |
| 6 | is_nexus | game_core::EntityType::is_nexus | pub | fn(&game_core::EntityType) -> bool | game-core\src\simulation\entity.rs:1399 |
| 7 | is_tower | game_core::EntityType::is_tower | pub | fn(&game_core::EntityType) -> bool | game-core\src\simulation\entity.rs:1385 |
</details>

⚠**미매칭 2개**: `llvm.smin.i64`, `vtable`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 3곳** (m02.ll:40037, m02.ll:40081, m02.ll:40124) · **형제 0개** 

**`open` 0건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**

(없음 — 이 함수는 `open` 이 비었다. 그래도 **게이트 미해소(§4)와 반증은 유효하다**.)

<details><summary>`closed` 7건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

| # | 물음 | 답 | ev |
|---|---|---|---|
| 0 | camp_type.__0(usize, Entity+0x98)의 의미. DWARF 로 타입만 확정됨 — EntityType::Jungle(Variant4).info: jungle::Jungle 의 camp_type: tuple$<usize, JungleType>. '< 2' 가 캠프 그룹/티어/스폰차수 중 무엇인지는 미확정(추정도 근거 없음). JungleType 자체는 0 Rhino/1 Mushroom/2 Stump/3 Bee/4 Morgard/5 Serpen 이지만 비교 대상은 JungleType(__1)이 아니라 __0 이다 |  |  |
| 1 | is_jungle()·is_nexus()·is_minion() 의 소스 본문. 전 24개 .ll 에 독립 define 이 없고(grep 'define.*EntityType9is_jungle' → 0건) 전부 인라인+switch 접힘이라, is_nexus=판별자3 / is_minion=판별자1 은 관측 사실이지만 '술어 이름'은 !dbg 로 확인된 is_jungle·is_nexus 둘뿐이다. tag 1 → -10 경로의 술어 이름(is_minion 으로 추정)은 미확정 |  |  |
| 2 | Effect::expected_damage_target(game_core) 내부 미확인 — 별도 크레이트라 _gaibc 에 본문 없음. value 의 단위(절대 피해량인지 스케일된 값인지) 불명. 다만 hp 와 같은 축으로 나눠지므로 HP 와 동일 단위로 추정 |  |  |
| 3 | effect(56B) 내부 필드를 이 함수는 한 번도 읽지 않아 어떤 이펙트인지 여기서는 알 수 없다. 대상 종류만 보고 채점하므로 이펙트 종류별 차등은 expected_damage_target 안에 있다 |  |  |
| 4 | 소스 527·529·532~535·537~541 줄에 대응하는 IR 이 하나도 없다. 중괄호/공백인지 최적화로 소멸한 코드인지 확인 불가(원본 .rs 없음) |  |  |
| 5 | writes 가 빈 배열인 이유: 이 함수에는 store 명령이 0개이고 모든 포인터 인자가 readonly/readnone(%0 은 readnone) 이라 쓰기 없음이 확정이다. '조사 못 함'이 아니다 |  |  |
| 6 | 함수 이름이 jungle 이지만 정글 캠프 전용이 아니라 '정글러(포지션) 액션 점수'다. 호출부를 안 봐서 어떤 액션 후보 루프에서 t 가 공급되는지는 미확인 |  |  |
</details>

<details><summary>`history` 정정 이력 5건 (참조용 — 본문 아님)</summary>

| # | 옛 값 | 현재 | 근거 |
|---|---|---|---|
| 0 | camp_type.__0(Entity+0x98)의 의미 — 캠프 그룹/티어/스폰차수 중 무엇인가 | ★확정 = 팀 인덱스(0=블루 / 1=레드). store 지점 = JungleCampState::spawn 의 map 클로저(_gcbc/g09.ll:47809): camp_type.__0 = (!is_blue_side) as usize, camp_type.__1 = self.ty. ⟹ camp_type 은 get_camp_state(team, ty) 인자쌍과 정확히 같은 (team, ty) 쌍이다. |  |
| 1 | Effect::expected_damage_target 의 value 단위 | ★확정 = 절대 피해량(HP 포인트). 방어/마저 경감까지 끝낸 실제 HP 감소량이다. 본체 _gcbc/g06.ll:52355~52846. return umax(raw_phys*100/(def_eff+100),1) + umax(raw_magic*100/(mr_eff+100),1). hp*r/100 로 %성분을 절대값으로 환산하므로 스케일된 값이 아니다. ⟹ hp 로 나누면 그대로 '최대체력 대비 비율'. |  |
| 2 | is_minion 술어 이름 — !dbg 로 확인된 것은 is_jungle·is_nexus 둘뿐 | ★**확정 — `entity.rs:1256`**(2026-09-11 rmeta SourceMap + rustc 프로브). ~~1381~~ 은 **오답**이었다 — 그 줄은 `is_any_jungle` 이고 `is_jungle` 이 1377 이다. `EntityType::is_minion(&self, <인자1>) -> bool` 시그니처 줄 56자. 인접: `is_any_type_minion` 1260 / `is_top_minion` 1264 / `is_mid_minion` 1268 / `is_bottom_minion` 1272 / `is_epic` 1369 / `is_serpen` 1373 / `is_tower` 1385. 검산 = 시그니처 길이 공식 `26 + len(name)` 이 IR 확정 줄 14개 전부와 일치. **태그 1 = Minion 은 종전대로 확정.** 전문 = RE6-09-11_rmeta-SourceMap-rustc프로브-Span복원.md |  |
| 3 | `is_jungle()` 내부 임계 `camp_type.__0 < 2` = 정글 캠프 티어 컷 (constants 1 + knobs 1 + new_knobs 1) | ★★**정정(2026-09-11 검증배치 A) — 그 노브는 존재하지 않는다. 3항목 삭제.** `EntityType::is_jungle` 은 **인자를 받는다**: `pub is_jungle(&self, usize) -> bool`(entity.rs:1377, mir=True). MIR 본문 = `_7 = (((*_1 as Jungle).0).2).0 : usize` / `_6 = Eq(move _7, copy _2)` — **상수 2 가 어디에도 없다**. IR 의 `icmp ult camp_type.0, 2` 는 **`x==0 \|\| x==1` 의 LLVM 접힘**이다. 소스 복원(줄 길이 ±0): `530` = `  } else if t.ty.is_jungle(0) \|\| t.ty.is_jungle(1) {`(52자) · `542` = `  let based = if t.ty.is_jungle(0) \|\| t.ty.is_jungle(1) {`(57자) · `536` = `  } else if t.ty.is_any_type_minion() {`(39자). ⟹ **진짜 노브는 티어가 아니라 「어느 팀 캠프를 세느냐」다**: 소스가 `is_jungle(0) \|\| is_jungle(1)` = **양 팀 캠프 전부**로 못박혀 있다. `is_jungle(player.info.team)` 이면 아군 진영 캠프만, `is_jungle(1 - team)` 이면 카운터정글 전용이 된다. ⚠이 3항목은 같은 파일의 `resolved`(`camp_type.__0` = 팀 인덱스)와 **서로 모순**이었고 `resolved` 가 맞았다. ★교훈: **IR 에 `< N` 상수가 보인다고 소스 임계로 읽지 마라.** 호출 술어의 **tcx 시그니처를 먼저 보라** — 인자를 받는 술어면 그 상수는 십중팔구 접힘이다. |  |
| 4 | `-10` 경로의 술어 이름 = `is_minion`(entity.rs:1256) 로 확정 | ★**정정(2026-09-11 검증배치 A)**: 실제는 **`is_any_type_minion`**(entity.rs:**1260**)이다. `is_minion(&self, LineType)` 은 **LineType 판별자 비교를 반드시 수반**하는데(MIR `_9=discriminant(Minion.0.0); _10=discriminant(_2); _6=Eq(_9,_10)`), `m05.ll:40027` switch 는 **태그 1 → 곧바로 `phi [-10]`** 이고 LineType 로드·비교가 **0개**이며 이 함수엔 LineType 지역변수도 없다. `is_any_type_minion` MIR = `Eq(discriminant, 1)` 한 줄로 IR 과 정확히 일치(+ 줄 길이 39자 ±0). **태그 1 = Minion 자체는 유효.** |  |
</details>

