---

### `48` v25_scoped_battle_objective — Morgard/Serpen 목표(Setup·Assemble 단계)가 focus 엔티티 근처(240000)의 국지전이 아니면 목표를 None 으로 지운다

| 항목 | 값 |
|---|---|
| id | `fight_model__v25_scoped_battle_objective` |
| 심볼 | `_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old11fight_model27v25_scoped_battle_objective` |
| 소스 | `game-ai\src\plan_legacy\old\fight_model.rs:1138` |
| IR | `m10.ll` 48404~48630행 |
| 경로·가시성 | `game_ai::plan_legacy::old::fight_model::v25_scoped_battle_objective` · **in:game_ai** |
| 계층 | 레거시 플랜 |
| exe | `e0b730` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r8` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, std::option::Option<game_ai::plan_legacy::team_plan::MainObjective>, &game_core::PlayerState, &game_core::OperationData, std::option::Option<usize>) -> std::option::Option<game_ai::plan_legacy::team_plan::MainObjective>
```

<details><summary>인자 5개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | _version | usize | 미사용(이름부터 _version). 본문 분기 없음 | 4 |
| 1 | 2 | main_objective | Option<MainObjective>(3B, IR i24) | byte0 = 태그(255=None, 0..11=Some variant), byte1 = phase(ObjectPhase), byte2 = with_battle/ready | 4 |
| 2 | 3 | player | &PlayerState(2528B) | info.team(+0x930) 만 읽는다 (camp_pos 의 bool 인자 = team==0) | 4 |
| 3 | 4 | data | &OperationData(24B) | +0x0 cache(&dyn AbstractGame 16B 선두), +0x8 context(→ +0x20 map) | 4 |
| 4 | 5 | focus | Option<usize>(16B = tag i64, id i64) | 전투 초점 엔티티 id. IR 에선 %4(tag)·%5(id) 두 레지스터로 풀림. None 이면 목표 지움 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn v25_scoped_battle_objective(_version, main_objective: Option<MainObjective>, player, data, focus: Option<usize>) -> Option<MainObjective>

[L1145] match main_objective.tag(byte0) {
  255 (None)            → return main_objective (None)
  0  (Morgard{phase,..}) if phase ∈ {Setup(1), Assemble(2)}:
[L1146~1147]  return if v25_is_objective_local_battle(JungleType::Morgard(4), player, data, focus) { main_objective } else { None }
  1  (Serpen{phase,..})  if phase ∈ {Setup(1), Assemble(2)}:
[L1149~1150]  return if v25_is_objective_local_battle(JungleType::Serpen(5),  player, data, focus) { main_objective } else { None }
  _  (Morgard/Serpen 의 다른 phase, 그 외 태그 2..11)
[L1154]       → return main_objective 그대로
}

// ── 인라인된 헬퍼 v25_is_objective_local_battle (fight_model.rs:1158~1166 추정, dloc 로 이름 확정) ──
[L1162] camp = data.context(+0x8).map(+0x20).camp_pos(jungle_type, player.info.team(+0x930) == 0)   // (x,y)
[L1163] focus_local_range = 240000
[L1164] e = focus.and_then(|id| game.get_entity_by_id(id))          // vtable+0x1f0 · null → false
[L1165] return e.is_some_and(|e| {
          dx = abs_diff(e.x(+0x660), camp.x); dy = abs_diff(e.y(+0x668), camp.y)
          dx*dx + dy*dy <= 240000²                                    // Morgard: !(> 57600000000) / Serpen: < 57600000001
        })

// 반환 조립 [L1154]: (main_objective & 0xFFFF00) | new_tag  — phase·with_battle 바이트는 항상 입력값 유지
// 분기 우선순위 근거: switch(i8 byte0) 한 번 → phase 범위검사 → focus.tag(i1) → get_entity null → 거리. phi %60 의 10개 유입이 전부 태그값.
```

**`mem` 메모리 접근 8건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | ==0 을 bool 로 MapDef::camp_pos 세 번째 인자에 넘긴다(팀별 캠프 좌표 대칭 선택으로 추정) | 5 | OK |
| 1 | OperationData | 0x0 | cache | r | &AbstractGameWithCache — 선두 16B 가 &dyn AbstractGame (data, vtable) | 4 | OK |
| 2 | OperationData | 0x8 | context | r | &GameContext | 4 | OK |
| 3 | GameContext | 0x20 | map | r | &MapDef → camp_pos | 4 | OK |
| 4 | AbstractGameWithCache | 0x0 | game.data_ptr | r | dyn AbstractGame 데이터 포인터 | 4 | OK |
| 5 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r | 슬롯 0x1f0 = get_entity_by_id (divtable 98%) | 3 | OK |
| 6 | Entity(focus) | 0x660 | x | r | camp 까지 거리제곱 계산 | 4 | OK |
| 7 | Entity(focus) | 0x668 | y | r |  | 4 | OK |

**`consts` 상수 10건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev | folded_from |
|---|---|---|---|---|---|---|
| 0 | -1 | 1145 | 센티널 | Option<MainObjective> 의 None 니치 = 태그 255(i8 -1). DWARF DISCR_EXACT=255 로 확인. 입력이 None 이면 그대로 None 반환, 국지전 아니면 None 으로 지운다 | 3 |  |
| 1 | 0 | 1145 | 태그 | MainObjective 메모리태그 0 = Morgard{phase, with_battle} | 4 |  |
| 2 | 1 | 1145 | 태그 | MainObjective 메모리태그 1 = Serpen{phase, with_battle} | 4 |  |
| 3 | 2 | 1145 | 임계 | phase 게이트: (phase-1) <u 2 ⟺ phase ∈ {1=Setup, 2=Assemble}. None(0)·Hunt(3) 단계는 손대지 않는다 | 4 |  |
| 4 | 4 | 1147 | 태그 | JungleType 메모리태그 4 = Morgard — camp_pos 의 캠프 종류 인자(dbg `target = i8 4`) | 4 |  |
| 5 | 5 | 1150 | 태그 | JungleType 메모리태그 5 = Serpen — camp_pos 인자 | 4 |  |
| 6 | 240000 | 1163 | 미상 | focus_local_range(dbg_value 로만 존재, 7.5셀). 비교식은 제곱으로 접혀 57600000000 이 된다 | 4 |  |
| 7 | 57600000000 | 1165 | 임계 | 240000² — Morgard 경로: dist_sq(focus, camp) > 240000² 이면 None. `icmp ugt` | 4 | 240000 |
| 8 | 57600000001 | 1165 | 임계 | 240000²+1 — Serpen 경로: dist_sq < 240000²+1 (즉 <= 240000²) 이면 유지. 두 경로 외연 동일(<=) | 4 | 240000 |
| 9 | -256 | 1154 | 계수 | i24 마스크 0xFFFF00 — 입력의 byte1(phase)·byte2(with_battle) 보존, byte0 만 교체 (exe 0xe0b730 의 0xffffff00 과 대응) | 4 |  |

**`knobs` 조정점 2건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 국지전 판정 반경 (focus ↔ 에픽 캠프) | fight_model.rs:1163 (m10.ll:48478 dbg `focus_local_range = i64 240000`; 비교 m10.ll:48521 `icmp ugt i64 %55, 57600000000` · 48620 `icmp ult i64 %102, 57600000001`) | 240000 | focus 엔티티가 캠프에서 이 거리(7.5셀) 안이어야 Morgard/Serpen 목표를 유지. 올리면 먼 곳의 싸움 중에도 에픽 목표가 살아남고, 내리면 캠프 바로 옆 싸움만 에픽 목표로 본다. 두 경로(Morgard/Serpen) 상수를 같이 고쳐야 한다 | 4 | 기존 |
| 1 | 적용 phase 집합 | fight_model.rs:1145 (m10.ll:48423·48429 `add i8 %10,-1; icmp ult i8 %11, 2`) | {Setup, Assemble} | Hunt(3) 단계는 이 함수가 건드리지 않는다. 범위를 3으로 넓히면 사냥 중에도 초점이 멀어지면 목표가 끊긴다 | 4 | 기존 |

<details><summary>`callees` 피호출자 9건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | camp_pos | game_core::MapDef::camp_pos | pub | fn(&game_core::MapDef, game_core::JungleType, bool) -> (u64, u64) | game-core\src\simulation\map_def.rs:207 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 2 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 3 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 4 | team | game_view::ClientData::team | pub | fn(&game_view::ClientData, usize) -> std::option::Option<std::cell::Ref< game_core::Team>> | game-view\src\logic\client\data.rs:1651 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 5 | team | game_view::ClientDatabase::team | pub | fn(&game_view::ClientDatabase, usize) -> std::option::Option<&game_core::Team> | game-view\src\logic\client\data.rs:968 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 6 | team | game_core::PlayerAiContext::<'a, 'b, 'r>::team | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:583 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 7 | v25_is_objective_local_battle | game_ai::plan_legacy::old::fight_model::v25_is_objective_local_battle | in:game_ai | fn(&game_core::PlayerState, &game_core::OperationData, std::option::Option<usize>, game_core::JungleType) -> bool | game-ai\src\plan_legacy\old\fight_model.rs:1156 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 8 | v25_scoped_battle_objective | game_ai::plan_legacy::old::fight_model::v25_scoped_battle_objective | in:game_ai | fn(usize, std::option::Option<game_ai::plan_legacy::team_plan::MainObjective>, &game_core::PlayerState, &game_core::OperationData, std::option::Option<usize>) -> std::option::Option<game_ai::plan_legacy::team_plan::MainObjective> | game-ai\src\plan_legacy\old\fight_model.rs:1138 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
</details>

⚠**미매칭 1개**: `main_objective`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 3곳** (m05.ll:18121, m05.ll:27276, m10.ll:23606) · **형제 0개** 

**`open` 4건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | MapDef::camp_pos(map, JungleType, bool) 의 bool 인자 의미 — team==0 을 넘기는 것만 확인. 본체(_gcbc) 안 봄. '팀 기준 대칭 캠프 좌표 선택'은 추정 | 5 |  |
| 1 | 표기 불가 | Morgard 경로는 `> 240000²`(부정)·Serpen 경로는 `< 240000²+1`(긍정) 으로 컴파일됐다 — 소스 표기(<= vs !(>)) 는 외연이 같아 표기 불가. 동작은 둘 다 dist_sq <= 240000² 이면 유지 | 4 |  |
| 2 | 미탐색 | 인라인 헬퍼 v25_is_objective_local_battle 의 정확한 시작 줄(1158~1161 사이) — dloc 로 이름과 본문 줄(1162~1165)만 확정. 별도 define 없음(fnparts 조각 1개) | 4 |  |
| 3 | 미탐색 | ★exe 짝: 지시의 0xc9a790 은 fnprobe 상 패닉 Location 이 fight_model.rs:960:17·961:7·962:21 이고 240000² 상수가 없어 이 함수가 아니다(지문 0.70 기각). .text 전수 스캔에서 57600000000 이 두 번 들어간 유일 함수 = 0xe0b730(560B, 인자 rdx/r8/r9+스택 2개, vtable+0x1f0 호출, 상수 0xffffff00 = i24 마스크, 호출자 3곳 0xd4be20/0xd52030/0xdfb840) — 이 함수의 진짜 짝으로 **추정**(Ghidra 미사용, 검증 = Ghidra 진입부 대조 또는 namebycaller) | 5 |  |

**`notes` 2건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | _version(p1) 은 IR 에서 한 번도 안 읽힌다 — 확인된 사실 | 4 | 사실 서술 |
| 1 | writes 빈 배열 = 확인된 사실(본문 store 0건, 순수 함수) | 4 | 사실 서술 |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

