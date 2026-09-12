---

### `19` best_jungle_goal — 정글러가 다음에 갈 캠프(JungleType)를 고른다 — 미클리어 캠프 중 내 챔프에서 가장 가까운 것

| 항목 | 값 |
|---|---|
| id | `passive_jungle__best_jungle_goal` |
| 심볼 | `_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old14passive_jungle16best_jungle_goal` |
| 소스 | `game-ai\src\plan_legacy\old\passive_jungle.rs:806` |
| IR | `m04.ll` 62570~62978행 |
| 경로·가시성 | `game_ai::plan_legacy::old::best_jungle_goal` · **pub** |
| 계층 | 레거시 플랜 |
| exe | `d40b20` (passive_jungle) · 898바이트 · 228명령 |
| 라운드 | 기준 `r4` · 통과 4회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan, std::option::Option<game_core::JungleType>, &mut game_core::DebugFrameData) -> game_core::JungleType
```

<details><summary>인자 7개</summary>

| # | i | 이름 | 타입 | 역할 |
|---|---|---|---|---|
| 0 | 1 | version |  | AI 버전 게이트. ★이 함수 본문에는 version 비교 분기가 하나도 없다. 주소(%13)만 잡아 filter 클로저 환경 +0x8 에 담아 넘기는데, is_cleared 호출 인자 자리에는 poison 이 들어가 실제로도 안 쓰인다 |
| 1 | 2 | rnd |  | champ 가 None 이고 now_camp 도 None 일 때만 사용(SliceRandom::choose) |
| 2 | 3 | player |  | info.team(0x930) · info.position(0x9c0) 만 읽는다 |
| 3 | 4 | data |  | cache(+0x0)=&AbstractGameWithCache, context(+0x8)=&GameContext |
| 4 | 5 | team_plan |  | 본문에서 직접 안 읽음. filter 클로저 환경 +0x20 에 담겨 is_cleared 의 arg7 로만 전달 |
| 5 | 6 | now_camp |  | -1(=255)=None. 현재 잡고 있던 캠프. champ 가 None 인 경로에서만 쓰인다 |
| 6 | 7 | debug |  | 본문에서 안 씀. filter 클로저 환경 +0x28 에 담기지만 is_cleared 호출 시 poison |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn best_jungle_goal(version, rnd, player, data, team_plan, now_camp, debug) -> JungleType

[807] jungle_camps: [JungleType;4] = [Rhino(0), Mushroom(1), Bee(3), Stump(2)]
// ※ 후보 배열에는 Morgard(4)/Serpen(5)가 없다. 다만 **반환값이 일반 캠프로 한정되지는 않는다** — 824~829 폴백이 `now_camp` 를 **그대로 돌려주므로** Morgard/Serpen 도 나올 수 있다 (실측 `champNone_nowcamp Some(Morgard) game=Morgard`). ⚠재구현에서 반환값을 일반 캠프로 필터링하면 **동작이 달라진다**.

[814-815] not_cleared_camps: bumpalo Vec<JungleType> =
 jungle_camps.into_iter()
 .filter(|c| !is_cleared(*c, player.info.team, _version, _rnd, player, data, team_plan, offset=0, _debug))
 .map(identity)
 .collect_in(data.context.pool)
 ※ filter 술어는 담당 범위 밖(call_mut 심 = m04.ll 66311~66341)에 있고,
 본체는 passive_jungle::is_cleared(passive_jungle.rs:841). 결과를 xor true 로 뒤집는다
 = '아직 안 잡힌 캠프만' 남긴다. offset 은 리터럴 0 으로 고정.

[817] if not_cleared_camps.len() == 0 { // 전부 클리어된 상태
[818] let mode = (*data.cache.game).get_game_mode() // vtable +0x40, indirect call
 .unwrap(); // tag!=0 이면 option::unwrap_failed
 let runner: &JungleRunner = &mode.jungle_runner; // MobaMode +0x18, 480B
[819] return jungle_camps.into_iter()
 .min_by_key(|c| runner.get_camp_state(player.info.team, *c).next_respawn_tick)
 .unwrap();
 // ★즉 '가장 먼저 리스폰될 캠프'로 미리 간다. 동점이면 배열 앞쪽(Rhino→Mushroom→Bee→Stump)이 이긴다
 // (fold 가 cmp<Greater 일 때만 교체 = min_by 의 first-wins).
 }

[823] if player.info.team >= 2 { panic_bounds_check(team, 2) } // 배열 경계
 let champ: Option<&Entity> = data.cache.player_champion[player.info.team][player.info.position];
 // player_champion = AbstractGameWithCache +0x1e0, [[Option<&Entity>;5];2] (team stride 40B, position stride 8B)

[824] if champ.is_none() { // 내 챔프 엔티티가 없다(사망/미스폰 등)
[825] let camp = match now_camp {
 Some(c) => c, // 지금 가던 캠프를 그대로 유지
[829] None => *jungle_camps.choose(rnd).unwrap(), // 4개 중 랜덤(choose 가 None 이면 unwrap_failed)
 };
 return camp; // ※ not_cleared 필터를 전혀 안 탄다
 }

[832] let champ: &Entity = champ.unwrap();
[833] return not_cleared_camps.into_iter()
 .min_by_key(|c| {
[834] let (cx, cy) = data.context.map.camp_pos(*c, player.info.team == 0);
[835] let dx = abs_diff(cx, champ.x); // Entity +0x660
 let dy = abs_diff(cy, champ.y); // Entity +0x668
 dx*dx + dy*dy // 제곱거리 (sqrt 없음)
 })
 .unwrap();
 // ★본선 판정: 아직 안 잡힌 캠프 중 내 챔프에서 제곱거리가 최소인 것.
 // 동점이면 not_cleared_camps 의 앞 원소(=jungle_camps 배열 순서)가 이긴다.

// 부수효과 없음: 구조체 write 는 하나도 없고 지역 alloca(배열/Vec/IntoIter)만 쓴다.
// 예외 경로에서 Vec/RawVec/IntoIter 의 Drop 과 drop_glue 를 호출하는 것이 전부.
```

**`mem` 메모리 접근 16건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev |
|---|---|---|---|---|---|---|
| 0 | OperationData | 0x0 | cache | r | &AbstractGameWithCache. 818/823줄에서 사용 · tcx 정본 대조( 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) | 3 |
| 1 | OperationData | 0x8 | context | r | &GameContext. 815줄(bump 할당자)·833줄(map)에서 사용 · tcx 정본 대조( 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) | 3 |
| 2 | GameContext | 0x0 | pool | r | &bumpalo::Bump — not_cleared_camps Vec 의 할당자로 from_iter_in 에 전달 · tcx 정본 대조( offset_of!(GameContext, pool)=+0x0 MISMATCH 0 (o1.txt) — 5차에 '관측 가능한 출력 없음'으로 남긴 행) | 3 |
| 3 | GameContext | 0x20 | map | r | &MapDef(28112B) — camp_pos 호출 대상 · tcx 정본 대조( 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) | 3 |
| 4 | AbstractGameWithCache | 0x0 | game.data_ptr | r | ref$<dyn AbstractGame> 의 데이터 포인터 · tcx 정본 대조( 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) | 3 |
| 5 | AbstractGameWithCache | 0x8 | game.vtable | r | ref$<dyn AbstractGame> 의 vtable 포인터 · tcx 정본 대조( 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) | 3 |
| 6 | dyn AbstractGame vtable | 0x40 | get_game_mode | r | divtable.py AbstractGame 0x40 으로 확정. 반환 {i64 tag, ptr}, tag==0 이면 Some(&MobaMode) — 아니면 option::unwrap_failed. ★런타임 확증 : `game.get_game_mode().as_moba()` 가 Some 이고 그 포인터 +0x240+8*team 을 직접 써서 `MobaMode::remain_epic_time(team)` 반환값이 따라 변함(0 → 100000). ⚠`chk` 는 tcxaudit 파생이라 '확인불가(vtable 슬롯)' 로 유지된다 · tcx 정본 대조( 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) | 3 |
| 7 | MobaMode | 0x18 | jungle_runner | r | DWARF !4879 offset:192bit=24B, size:3840bit=480B → JungleRunner. get_camp_state 의 self · tcx 정본 대조( 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) | 3 |
| 8 | JungleCampState | 0x18 | next_respawn_tick | r | ★폴백 경로의 정렬 키. get_camp_state() 결과의 +24 · tcx 정본 대조( 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) | 3 |
| 9 | AbstractGameWithCache | 0x1e0 | player_champion | r | [[Option<&Entity>;5];2] (80B). [team][position] 로 인덱싱 — stride 40(=[5 x ptr]), 요소 8 · tcx 정본 대조( 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) | 3 |
| 10 | PlayerState | 0x930 | info.team | r | 0/1. ①player_champion 1차 인덱스(≥2면 panic_bounds_check) ②get_camp_state 인자 ③camp_pos 의 is_blue(team==0) · tcx 정본 대조( 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) | 3 |
| 11 | PlayerState | 0x9c0 | info.position | r | Position(i32, range 0..5). player_champion 2차 인덱스 · tcx 정본 대조( 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) | 3 |
| 12 | Entity | 0x660 | x | r | 내 챔프 x — 캠프까지 제곱거리 계산 · tcx 정본 대조( 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) | 3 |
| 13 | Entity | 0x668 | y | r | 내 챔프 y · tcx 정본 대조( 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) | 3 |
| 14 | bumpalo Vec<JungleType> | 0x0 | ptr | r | not_cleared_camps 데이터 포인터(IntoIter 시작) · tcx 정본 대조( bumpalo Vec 레이아웃 {ptr@+0x0, a@+0x8, cap@+0x10, len@+0x18} — tcxaudit 대조 OK(같은 제네릭 인스턴스), mem[15] len=+0x18 과 정합) | 3 |
| 15 | bumpalo Vec<JungleType> | 0x18 | len | r | not_cleared_camps 길이. ==0 이면 폴백 경로(817줄) · tcx 정본 대조( 오라클 game==mine 236/236 ⟹ **오프셋은 tcx 가 정본이라 ev3 이 상한**(실행은 교차검증)) | 3 |

**`consts` 상수 5건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 0 | 807 | 임계 | jungle_camps[0] = JungleType::Rhino (dienum JungleType 0=Rhino). 같은 값 0 이 ① get_game_mode 반환 tag==0(=Some) 판정 ② player.info.team==0(=블루 진영) 판정에도 쓰인다 · 오라클 실행 확증( 오라클 game==mine 236/236) | 2 |
| 1 | 1 | 807 | 임계 | jungle_camps[1] = JungleType::Mushroom · 오라클 실행 확증( 오라클 game==mine 236/236) | 2 |
| 2 | 3 | 807 | 임계 | jungle_camps[2] = JungleType::Bee ★소스 배열 순서가 Rhino,Mushroom,Bee,Stump 임에 주의(2보다 3이 먼저 저장됨) · 오라클 실행 확증( 오라클 game==mine 236/236) | 2 |
| 3 | 2 | 807 | 임계 | jungle_camps[3] = JungleType::Stump · 오라클 실행 확증( 오라클 game==mine 236/236) | 2 |
| 4 | -1 | 825 | 태그 | Option<JungleType>::None 의 니치 태그(=255). ① now_camp==None 판정(825줄) ② min_by_key 결과 unwrap 잔여검사 2곳(819/833줄, 시드가 있어 실제로는 발생 불가) · 오라클 실행 확증( 오라클 game==mine 236/236) | 2 |

**`knobs` 조정점 8건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev |
|---|---|---|---|---|---|
| 0 | 후보 캠프 집합과 그 순서 | passive_jungle.rs:807 (m04.ll 62596~62602의 store i8 0/1/3/2) | [Rhino, Mushroom, Bee, Stump] | 여기 없는 캠프(Morgard=4, Serpen=5)는 이 함수로는 절대 목표가 되지 않는다. 원소를 늘리면 배열 길이 4가 함께 바뀌어야 하고(choose 의 len 인자·min_by_key 시드), 순서를 바꾸면 거리/리스폰틱 동점 시 승자가 바뀐다 · 오라클 실행 확증( 오라클 game==mine 236/236) | 2 |
| 1 | 본선 정렬 키 = 챔프↔캠프 제곱거리 | passive_jungle.rs:833~835 | dx*dx + dy*dy | 가중치를 넣으면(예: 캠프별 계수) 정글 동선이 '가까운 것 우선'에서 '가치 우선'으로 바뀐다. 현재는 캠프 가치·경험치·팀플랜을 전혀 안 본다 · 오라클 실행 확증( 오라클 game==mine 236/236) | 2 |
| 2 | 폴백 정렬 키 = next_respawn_tick 최소 | passive_jungle.rs:819 (JungleCampState +0x18) | get_camp_state(team, camp).next_respawn_tick | 전부 클리어됐을 때 '가장 먼저 리젠되는 캠프에 미리 대기'. 여기를 거리 기준으로 바꾸면 리젠 선점 플레이가 사라진다 · 오라클 실행 확증( 오라클 game==mine 236/236) | 2 |
| 3 | 필터 = !is_cleared(..., offset=0) | passive_jungle.rs:814 / 술어 본체 passive_jungle.rs:841, 심 = m04.ll 66311~66341 | offset 리터럴 0 | offset 은 is_cleared 의 8번째 인자로 0 고정. 0 이 아닌 값이면 '리스폰 예정 시각을 앞당겨 미리 안 잡힌 것으로 친다' 류의 선점 여유가 될 가능성이 크지만, 본문이 담당 범위 밖이라 단정하지 않는다 · 오라클 실행 확증( 오라클 game==mine 236/236) | 2 |
| 4 | 챔프 엔티티가 없을 때의 폴백 | passive_jungle.rs:824~829 | now_camp 유지, 없으면 4개 중 균등 랜덤 | 이 경로는 not_cleared 필터를 무시하므로 '이미 잡힌 캠프'가 목표로 나올 수 있다. 여기서 not_cleared_camps 를 쓰도록 바꾸면 사망 직후 목표가 더 합리적으로 된다 · 오라클 실행 확증( 오라클 game==mine 236/236) | 2 |
| 5 | 정글 클리어 판단의 여유 1초 | passive_jungle.rs:855 = + tick_per_second (m04.ll:62495~62500) |  | 키우면 캠프를 '비었다'로 볼 확률↓ → 정글러가 더 자주 캠프로 향함 · 오라클 실행 확증( 오라클 game==mine 236/236) | 2 |
| 6 | 짝캠프 마진 5초 | is_side_cleared 의 tps*5 (m04.ll:62553) |  | 두 캠프 연속 클리어 판단의 이동 예산 | 4 |
| 7 | 캠프 좌표 6쌍 | g09.ll:59281/59282 phi |  | pos[1] 은 자동 미러라 한 쌍만 고치면 양팀 동시 이동 · 오라클 실행 확증( 오라클 game==mine 236/236) | 2 |

<details><summary>`callees` 피호출자 9건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 |
|---|---|---|---|---|---|
| 0 | best_jungle_goal | game_ai::plan_legacy::old::best_jungle_goal | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan, std::option::Option<game_core::JungleType>, &mut game_core::DebugFrameData) -> game_core::JungleType | game-ai\src\plan_legacy\old\passive_jungle.rs:806 |
| 1 | camp_pos | game_core::JungleType::camp_pos | pub | fn(&game_core::JungleType, bool) -> (u64, u64) | game-core\src\simulation\entity\jungle.rs:427 |
| 2 | camp_pos | game_core::MapDef::camp_pos | pub | fn(&game_core::MapDef, game_core::JungleType, bool) -> (u64, u64) | game-core\src\simulation\map_def.rs:207 |
| 3 | drop | <game_core::prof::ProfTimer as std::ops::Drop>::drop | pub | fn(&mut game_core::prof::ProfTimer) | game-core\src\simulation\prof.rs:184 |
| 4 | get_camp_state | game_core::JungleRunner::get_camp_state | pub | fn(&game_core::JungleRunner, usize, game_core::JungleType) -> &game_core::JungleCampState | game-core\src\simulation\entity\jungle.rs:702 |
| 5 | get_game_mode | <game_core::Game as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::Game) -> game_core::GameMode | game-core\src\simulation\game.rs:1707 |
| 6 | get_game_mode | <game_core::SingleLaneGame as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::SingleLaneGame) -> game_core::GameMode | game-core\src\simulation\game.rs:4248 |
| 7 | get_game_mode | <game_core::DeathMatchGame as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::DeathMatchGame) -> game_core::GameMode | game-core\src\simulation\game.rs:5052 |
| 8 | is_cleared | game_ai::plan_legacy::old::is_cleared | pub | fn(game_core::JungleType, usize, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut game_core::DebugFrameData) -> bool | game-ai\src\plan_legacy\old\passive_jungle.rs:841 |
</details>

**호출처 3곳** (m04.ll:28421, m04.ll:28455, m04.ll:32758) · **형제 0개** 

**`open` 0건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**

(없음 — 이 함수는 `open` 이 비었다. 그래도 **게이트 미해소(§4)와 반증은 유효하다**.)

<details><summary>`closed` 7건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

| # | 물음 | 답 | ev |
|---|---|---|---|
| 0 | is_cleared 본문(passive_jungle.rs:841~) 은 담당 줄범위 밖이라 안 읽었다 — '클리어됨'의 실제 판정식(리스폰 틱 비교인지 live_list 비어있음인지)은 미확인. 본문에서 확인한 것은 ①call_mut 심이 결과를 xor true 로 뒤집는다 ②인자 순서 (camp, team, _version, _rnd, player, data, team_plan, offset=0, _debug) ③offset 에 리터럴 0 이 들어간다 뿐이다 |  |  |
| 1 | MapDef::camp_pos(map, camp, is_blue) 내부 미확인 — game_core 크레이트라 _gaibc 에 define 이 없고 declare 만 있다. 3번째 bool 인자가 player.info.team == 0 이라는 것만 확정 |  |  |
| 2 | JungleRunner::get_camp_state(self, team, ty) 내부 미확인(같은 이유). JungleRunner 필드 배치(blue_* 4 + red_* 4 + epic + serpen)로 보아 team 으로 blue/red 블록을 고르고 ty 로 캠프를 고르는 것이 자연스럽지만 IR 로는 확인 못 했다 |  |  |
| 3 | get_game_mode() 의 반환 {i64 tag, ptr} 에서 tag==0 을 Some 으로 읽었다. 근거는 ①tag!=0 일 때 core::option::unwrap_failed 로 간다 ②payload+0x18 이 MobaMode.jungle_runner(DWARF !4879, offset 192bit, size 3840bit=480B)와 정확히 맞는다. 다만 GameMode 열거형 자체의 DISCR_EXACT 는 못 찾았다(dienum 에 해당 항목 없음) |  |  |
| 4 | min_by_key 의 fold 는 m12.ll 22555~22732(폴백 경로) / m12.ll 34621~34775(본선 경로)에 있고 담당 범위 밖이다. 두 fold 를 직접 읽어 키 계산식과 first-wins(cmp<Greater 일 때만 교체) 는 확인했지만, constants 에는 담당 범위 상수만 실었다 |  |  |
| 5 | map 클로저(s_0)는 별도 define 이 없다(ZST·인라인). from_iter_in(m01.ll 20914~21098)을 읽어 필터 통과 원소를 그대로 push 하는 항등 사상임을 확인했으므로 unknown 이 아니라 사실로 logic 에 적었다 |  |  |
| 6 | 본문에 배열 길이 4(jungle_camps)·2(team 경계)가 리터럴로 있으나 SPEC_GUIDE §3 표의 '배열 인덱스·stride 는 적지 않는다' 규칙에 따라 constants 에서 뺐다. 대신 knobs 첫 항목에 순서/길이의 영향으로 적어두었다 |  |  |
</details>

<details><summary>`history` 정정 이력 8건 (참조용 — 본문 아님)</summary>

| # | 옛 값 | 현재 | 근거 |
|---|---|---|---|
| 0 | is_cleared 의 실제 판정식과 극성, offset 의 의미 | ★확정. 극성 = 필터는 !is_cleared(...) (call_mut 심이 xor true, m04.ll:66339). 본체 = _gaibc/m04.ll:62404~62525 (passive_jungle.rs:841~859). 판정식은 리스폰 틱 비교이고 live_list 는 쓰지 않는다:   champ = data.cache.player_champion[player.info.team][player.info.position]; None 이면 false   next_spawn = team_plan.next_respawn_tick[team][camp_idx(camp)]   (TeamPlan+0x378 = [[usize;4];2])   if !(game.tick() < next_spawn) { return false }        // 캠프가 이미 살아있으면 false   dist = distance(champ.x, champ.y, map.camp_pos(camp, team==0))   move_tick = dist / champ.stat_cached.move_speed(+0x640)   return next_spawn > move_tick + offset + game.tick() + tick_per_second |  |
| 1 | 두 fold 의 정체(폴백/본선) | 본체 m04.ll:62645~62646 에서 갈린다 — 필터 통과 Vec 의 len==0 이면 폴백 fold(m12.ll:22555), 아니면 본선 fold(m12.ll:34621).   폴백: 고정 4원소 배열, 키 = get_camp_state(runner, team, camp).next_respawn_tick(+0x18) ⟹ 가장 빨리 리스폰하는 캠프.   본선: 필터 통과 목록, 키 = dist_sq(champ.xy, MapDef::camp_pos(map, camp, team==0)) ⟹ 가장 가까운 캠프.   둘 다 판정 임계 상수 0건. |  |
| 2 | JungleRunner::get_camp_state 의 team/ty → 필드 매핑 | ★확정. _gcbc/g09.ll:137367~137440. team 이 바깥 switch, ty 가 안쪽 switch.   team0: Rhino +0x0 / Mushroom +0x30 / Stump +0x60 / Bee +0x90 / Morgard +0x180(epic)   team1: Rhino +0xc0 / Mushroom +0xf0 / Stump +0x120 / Bee +0x150 / Morgard +0x180(epic)   team>=2: Morgard 만 합법 ⟹ team 2 = 중립 오브젝트 취급   ★ty=5 Serpen 은 전 team 에서 panic — get_camp_state 로는 절대 못 얻는다. get_respawn_tick(g09.ll:137444) 또는 get_jungle_live_list(g09.ll:137470) 를 써야 한다.   JungleCampState(48B) = live_list: Vec<usize> @+0x0 / next_respawn_tick @+0x18 / respawn_count @+0x20 / is_blue_side @+0x28 / ty @+0x29 |  |
| 3 | MapDef::camp_pos 의 좌표표 | ★전체 복원. camp_pos(g07.ll:152570)는 thread-local 메모 래퍼, 본체는 클로저 g02.ll:6914~7180. 실계산 = MapDef.camps(Vec @0x60) 선형탐색해 CampDef.ty == camp 인 원소의 pos[team] 반환, 없으면 (0,0). CampDef(40B) = pos: [(u64,u64);2] @0x0, ty @0x20. ★3번째 bool 은 is_blue 이고 내부에서 (!is_blue) as usize 로 뒤집힌다 ⟹ index 0 = 블루. |  |
| 4 | m12.ll:22671 의 vtable +0x40 슬롯이 Option<&JungleRunner> 를 돌려주는 게터라는 것까지만 확인 | ★확정 — **`game.get_game_mode().as_moba().unwrap().jungle_runner`**. `_gaibc/m12.ll:22671~22697`: `%37 = vtable[+0x40](game)` → `{tag, ptr}`, `tag == 0` 일 때만 통과(아니면 `unwrap_failed`) = `as_moba().unwrap()`, `%44 = payload + 24` 를 `get_camp_state` 에 `dereferenceable(480)` 로 전달. `distruct MobaMode 0x18` → **`jungle_runner : JungleRunner (480B)`** 로 정확히 일치. 폴백 fold 자체도 확정: 키 = `get_camp_state(...)+0x18 next_respawn_tick` ⟹ **리스폰이 가장 빠른 캠프**. |  |
| 5 | (검증) | ✅**오라클 end-to-end 10/10 MATCH(검증배치 D)** — SDK 실제 반환값과 명세 규칙("미클리어 캠프 중 제곱거리 최소, 동점이면 배열 앞")의 손계산이 팀×포지션 **10칸 전부 일치**. **오류 0.** 실행이 부수로 확인해 준 것: `MapDef::camp_pos` 좌표표가 `resolved.좌표표` 와 **완전 일치** · `is_cleared`/`is_side_cleared` 가 초기 상태 전부 false ⟹ `not_cleared_camps`=4 ⟹ **본선 경로가 실제로 돌았다**(폴백 아님) · `now_camp=Some(Bee)` 를 줘도 결과 불변 ⟹ **`now_camp` 는 챔프 없을 때만**. |  |
| 6 | `tick_per_second=0` 한계가 1차 결과를 오염시켰는가 | ★**오염 없음**(2026-09-11 2차배치D). `GameSetting::tick_per_second` 는 pub 필드라 `MapDef::moba` 호출 **이전에** 60 을 대입하면 된다. 재실행 결과 담당 5개 전부 1차와 동일: 19 **10/10 MATCH**(Bee), `is_cleared`/`is_side_cleared` 전부 false. ⟹ tps=0 한계는 **실재하지만 이 함수의 1차 결과를 바꾸지 않았다**. |  |
| 7 | `get_camp_state` 의 team/캠프 매핑 | ★**포인터 동일성 전수표**(3차 배치D). 4캠프는 team 0→`blue_*` / 1→`red_*`, **team>=2 는 패닉**(`unreachable!("team value must be 0 or 1")` @jungle.rs:713). 특례 2건: **Morgard → `epic`(team 무관)** · **Serpen → 전 team 패닉** ⟹ `serpen` 필드는 이 접근자로 **도달 불가**. v3 의 추정이 맞았고 특례만 추가된다. |  |
</details>

