# 3차 반증검증 — 배치 B (05~09) 보고

게임 0.5.8 / SDK `sdk_058` / toolchain `nightly-2026-05-24` / 2026-09-11
작업 파일 = `C:\tfm2mods\MIG\_verify3\B\` (`B3_o07.rs` · `B3_o08.rs` · 이 문서)

---

## 0. 한 줄 결론

**실오류 2건**(둘 다 「한 스펙 안에서 자기모순」 유형) · **open 21건 중 16건 닫음** ·
**신규 확정 9건**(오라클 2 · MIR 2 · IR 5). ev<=3 표본 재확인 9건 중 뒤집힌 것 **1건**(= 실오류 E1).

---

## 1. 실오류 — JSON 경로 패치 목록

### E1. `/specs[8]/sig/params[5]/type`

```
구: "&mut TeamPlan(1064B)"
신: "&TeamPlan(1064B)"
```

근거: tcx 정본(`_tcx\game_ai.json` i=2543)의 sig 가
`... &'^6 game_ai::plan_legacy::team_plan::TeamPlan, ...` 로 **mut 이 없다.**
같은 스펙의 `sig.tcx` 문자열도 이미 `&TeamPlan` 이라 **스펙 내부 자기모순**이다.
IR `m10.ll:7655` 의 6번째 인자 `%5` 에 store 는 0건(gep 2곳 모두 load).
ev: 3 (v3 우선순위 = `sig.tcx` 가 정본)

※ `role` 본문("이 함수에서 쓰기는 없다")은 맞다. 틀린 것은 `type` 문자열 하나다.
※ 부수 관측(오류 아님): `m10.ll:7655` 의 `%5` 에는 `readonly`·`noalias`·`dereferenceable` 이 없다.
같은 `&TeamPlan` 을 받는 `v24_objective_setup_should_release_to_passive`(m09.ll:22104)에는 `readonly` 가 있다.
TeamPlan 에 내부가변성 필드는 없다(tcx 38필드 전수 확인). 속성 누락 이유는 **미탐색**이고,
어느 쪽이든 본문에 store 가 0건이라 판정에는 영향이 없다.

### E2. `/specs[9]/open[0]/q` — v3 정정이 한 곳만 반영된 자기모순

`/specs[9]/sig/params[0]/role` 은 **ev 2** 로 「죽은 인자 · 0 하드코딩도 결과 불변」인데,
`/specs[9]/open[0]/q` 는 여전히 「**주 경로에서는 살아 있는 버전 게이트**다. 재구현 시 0 하드코딩 금지」로 끝난다.

```
구(open[0] 말미): "... 나머지 7곳(plan_legacy::handler)은 런타임 version SSA 값을 넘긴다
                   ⟹ 주 경로에서는 살아 있는 버전 게이트다. 재구현 시 0 하드코딩 금지.
                   미탐색 = 피호출자 내부의 version 분기"

신             : "... 나머지 7곳(plan_legacy::handler)은 런타임 version SSA 값을 넘긴다(관측은 유효).
                   단 ~~주 경로에서는 살아 있는 버전 게이트다 · 0 하드코딩 금지~~ → 거짓:
                   피호출자 1단(m07.ll:48226 enemy_minion_line_action_danger_damage_at)의 version 이
                   #dbg_value(i64 poison) 이고 그 안에서 2단(enemy_minion_line_action_damage_at)에도
                   i64 poison 을 넘긴다 ⟹ 체인 전체에서 죽은 인자(3차배치B 재확인)."
조치: 이 항목을 open[] → closed[] 로 이동.
ev: 4(IR) + 2(2차 오라클 400x12 diff 0)
```

⚠ 이 항목은 `meta.corrections` 가 경고한 정확한 실패모드(「정정이 한 곳만 반영」)의 재발이다.

---

## 2. open[] 처리 — 21건 중 16건 닫음

판정 어휘 = **닫힘 / 표기 불가 / 재료 부재(범위 열거) / 미탐색**.

### 05 `v50_fold_dive_episode` — 6건 중 5건 닫음, 1건 재분류

| # | 원래 | 3차 판정 | 근거 |
|---|---|---|---|
| 0 | 미탐색 ev3 | **닫힘 (ev4)** | end_reason 0~8 코드표 신규 확정 — §3 C3 |
| 1 | 미탐색 ev5 | **닫힘 (ev3)** | 「명명 enum 없음」이 tcx 로 확정 = `game_core::V50DiveEpisode` 의 `end_plan` 필드 타입이 `u8`. 코드 1..9 의 의미는 함수 내부 문자열 매칭이 **곧 정의**이므로 역추정이 아니다. ev 5→3 |
| 2 | 미탐색 ev3 | **재료 부재(범위 열거)** | 확정된 것: DWARF 이름이 `_version`/`_tps`(소스가 `_` 접두로 의도적 미사용 선언) + IR define 이 3인자 + `#dbg_value(i64 poison)`. 「원래 어떤 게이트였나」는 **과거 소스 이력**이고, 탐색 범위 = DWARF · LLVM IR · rmeta 주석 · rmeta SourceMap · tcx 전부 **현재 스냅샷만** 담아 이력이 없다. 미탐색 = 개발사 VCS |
| 3 | 미탐색 ev4 | **닫힘 (ev3)** | 레코드 타입에 그 두 필드가 **없다** — §3 C7 |
| 4 | 미탐색 ev4 | **닫힘(판정 무관)** | `RawVec::grow_one` = std 재할당 정책. `Vec::push` 의 관측 의미(끝에 1개 추가)에 영향 없음 |
| 5 | 미탐색 ev4 | **닫힘 (ev4)** | 소비처 8곳 전수 + 동일 게이트식 — §3 C4 |

부수 확정(05 — `aborted` / `abort_src` 의 산출): 호출부 `m13.ll:23843~23861`(dive_episode.rs:115~116)에서
`aborted` = (BigPlan 메모리태그 == 9, 즉 Battle) && `BattlePlan.dive_abandoned` 이고,
`v50_dive_ep_abort_src` 에는 Battle 일 때 `BattlePlan.dive_abort_src` 를, 아니면 0 을 미리 써 둔다.

- `BattlePlan+0xfa` = `dive_abandoned` (bool)
- `BattlePlan+0x103` = `dive_abort_src` (u8)
- `LegacyPlanHandler+0x5f0` = `plan` 의 `Battle` 페이로드(BattlePlan) 시작

### 06 `v2_response_retreat_stance` — 3건 전부 닫음

| # | 3차 판정 | 근거 |
|---|---|---|
| 0 | **닫힘(지식 공백 아님)** | 값·의미·소스표기가 `knobs` 에 ev3 로 이미 확정돼 있고, `consts` 에 못 넣는 것은 QC C1(담당 줄범위) 규칙 탓이다. 재실측: `m12.ll:41992` = `icmp ult %47, 40000000001`, `m12.ll:42077` = `icmp ult %47, 22500000001` — 스펙과 일치(200000²+1 / 150000²+1) |
| 1 | **닫힘(같은 이유)** | 두 술어의 판정 정본은 이미 `shared.is_recent_visible` 과 `callees`(is_ignored_well_enemy)에 있다 |
| 2 | **표기 불가 + 확정 분리** | **확정** = self 의 어떤 필드도 결과에 영향을 주지 않는다(IR define 의 1번 인자 자체가 소거 = 컴파일러가 미사용을 증명). **표기 불가** = 소스가 `self` 를 텍스트로 언급했는지 — 결과에 영향 없는 죽은 읽기는 IR·MIR·기계어·오라클 어디에도 흔적이 없고, `&self` 는 rustc 미사용 경고 대상도 아니라 진단 프로브로도 안 걸린다 |

재확인(06 본문 표본): `m13.ll:45228` `icmp ult %0, 2` → RunAway.
`m13.ll:45611` `icmp ugt %118, %122`(die > tick_per_second) → 태그 3 KitingBack, 아니면 태그 4.

- `PlayerState+0x930` = `info.team` (IR gep 2352)
- `PlayerState+0x9c0` = `info.position` (IR gep 2496)
- `Entity+0x5c0` = `id` (KitingBack.focus 페이로드로 복사)

### 07 `EpicHuntAndBattlePlan::sub_plan` — 4건 중 3건 닫음, 1건 재분류

| # | 3차 판정 | 근거 |
|---|---|---|
| 0 | **닫힘 (ev4)** | `m02.ll:48918` 8번째 인자 = `ptr noalias readnone align 8 captures(none)`. `readnone` 은 컴파일러가 「이 포인터로 읽지도 쓰지도 않음」을 보증한 것 |
| 1 | **닫힘 (ev3)** | vtable 슬롯↔메서드는 **트레이트 선언 순서**로 고정 = 구현체 무관 — §3 C5 |
| 2 | **닫힘(규칙상 제외가 맞다)** | 상수 2 는 `player_champion[team]` 배열 상한(bounds-check)이고 판정 임계가 아니다 |
| 3 | **재료 부재로 재분류**(현재 class=미탐색) | L36 세 OR 항의 소스 순서. 범위 = ①`!DILocation` 의 column 이 전 모듈 0 ②`mir=false` ③줄 길이 산술은 교환에 불변 ④IR `or` 평탄화는 피연산자 순서 정보량 0. 남은 미탐색 = exe 디스어셈 · 개발사 소스. **논리값은 순서와 무관하므로 재현에 영향 없음** |

⚠ 과제 지시문의 「07 L36 **두** OR 항」은 오기다. 실제는 **세 항**
(`hp_ratio<51` / `can_upgrade_item` / `(champ.hp < max && is_in_heal_area)`)이고 스펙 쪽이 맞다
(IR `m02.ll:49087~49091`: `%95 = or(%94,%85)`, `%98 = or(%97,%95)`).

### 08 `EpicHuntAndPokePlan::is_end` — 4건 전부 닫음

| # | 3차 판정 | 근거 |
|---|---|---|
| 0 | **닫힘 (ev4)** | self 4필드는 **같은 플랜의 `sub_plan` 이 전부 읽는다** — §3 C8 |
| 1 | **이미 닫힌 것이 open 에 남아 있었다** | `shared.is_recent_visible` 에 판정식이 완전 규정돼 있고 그 `영향` 목록에 `is_end` 가 이미 들어 있다. BRIEF §2 가 말한 「과하게 열린」 사례 → closed 로 이동 권고 |
| 2 | **닫힘 (ev3)** | §3 C5 (07[1]과 동일 근거) |
| 3 | **닫힘 (ev3)** | `<Game as AbstractGame>::get_game_mode` 의 MIR 이 한 줄 순수 함수 — §3 C6 |

### 09 `check_favorable_engage_formation` — 4건 중 1건 닫음(E2), 3건 유지

| # | 3차 판정 | 근거 |
|---|---|---|
| 0 | **닫힘** (E2 정정과 함께) | 피호출자 2단 poison 재확인 |
| 1 | **유지(정확)** | 원본 소스 부재는 사실. 좌표는 tcx 로 재확인 = `game-ai\src\fight_check.rs:1196:3` (ev3) |
| 2 | **유지(정확)** | `shl` 접힘 서술이 IR 과 일치 |
| 3 | **유지(이미 재료 부재)** | 표본 재확인 통과: `m15.ll:35598~35601` = `%97 = select i1 %95, i1 %96, i1 false`(안쪽 `&&` 단축평가 보존, `flank>0` 이 좌항) + `%99 = or i1 %98, %97`(바깥 `||` 평탄화). 스펙과 일치 |

---

## 3. 신규 확정

### C1. 08 전체 진리표 **13/13 MATCH (ev2)** — `_verify3\B\B3_o08.rs`

★**`15 × tick_per_second` 임계가 정확히 900 에서 갈린다**(tps=60, tick=0):
`next_respawn_tick` 899→false / 900→**false** / 901→**true** / 5000→true.
⟹ `/specs[8]/consts` 의 15 와 L194 식(`saturating_sub(tick) > 15*tps`, 등호 미포함)을 **ev4 → ev2**.

같이 확증된 것:

- L164: `objective` = None / Serpen(태그1) / Defense(태그2) → 전부 true
- L193: `live_list = [유효 엔티티 id]` → **false**(에픽 생존 → 계속) / `live_list = [없는 id]` → `get_entity_by_id` 가 null → L194 로 흐름
- `phase` = Assemble·Hunt 는 setup 이 아니다(Setup 만 진입)
- version 9종(0,1,2,3,24,30,46,50,60) 결과 차 **0** (non-setup 경로)
- setup 경로 분해: 이 환경에선 `take_active(Morgard)=true` · `take_setup_like(Morgard)=true` ·
  `v24_objective_setup_should_release_to_passive=true` ⟹ **(b) 경로가 먼저 발화**해 is_end=true.
  `camp_pos(Morgard, blue=true) = (288000, 288000)` → 셀 (9,9) → `is_visible_cell(0,9,9) = false`.
  ⟹ 요약의 (c)·(d) 경로는 **오라클 미도달(미탐색)**. IR 독해는 유효

⚠ 프로브는 BRIEF §3① 대로 `init_tower` / `init_nexus` 를 **호출하지 않았다.**

### C2. 07 L36 진리표 **8/8 (ev2)** — `_verify3\B\B3_o07.rs`

★**`hp_ratio < 51` 경계가 정확히 50/51 에서 갈린다**(ratio 50 → Recall, ratio 51 → EpicHunt).

- 세 번째 OR 항이 `champ.hp < champ.stat_cached.hp && is_in_heal_area` 인 것도 확증:
  ratio=51 + 힐영역 안 → **Recall** / ratio=100(hp==max) + 힐영역 안 → **EpicHunt**(둘째 조건 불성립)
- `epic.is_some_and(|e| e.hp == e.stat_cached.hp)` 단축평가 확증: 에픽 hp=max−1 → EpicHunt / `live_list` 비움 → EpicHunt
- 런타임 태그 확인: **5 = Recall · 11 = EpicHunt**
- `map.fountains[0] = (0, 896000, 64000, 960000)` · `map.fountains[1] = (892000, 0, 960000, 64000)` (2차 배치B 값 재현)
- version 9종 결과 차 **0**
- 이 환경의 `upgrade_item` 은 항상 None(`item_list` 가 비어 있음) ⟹ 둘째 OR 항이 상시 false 로 고정된 상태의 측정이다
- 제약: `EpicHuntAndBattlePlan::target_bush` 필드가 **private**(`in:game_ai::plan_legacy::old::epic::hunt_and_battle`)이라
  `Default`(=None)로만 만들 수 있어 **Hide(태그 9) 경로는 오라클 미도달(미탐색)**. `Default` 는 pub, 필드는 아니다

### C3. 05 `end_reason` 0~8 코드표 (ev4, 신규)

★과제 지시문의 「제3출처는 exe 디스어셈만 남았다」는 **틀렸다.** 코드 산출 지점이 전부
호출자 `LegacyPlanHandler::update`(m13.ll:14467) 안에 인라인된 `v50_track_dive_episode` 에 있다
(`%3506` / `%3508` 두 phi, `m13.ll:23531` · `m13.ll:23538`). 지역변수 `reason` = DWARF `!32145`(dive_episode.rs:107).

| 코드 | 산출 지점 | 조건 |
|---|---|---|
| 0 | dive_episode.rs:66 (호출 m13.ll:23653) | 진행중 에피소드의 `live.tower` 가 새로 계산한 타워와 다름 → 정상 종료(`aborted=false` 리터럴) |
| 1 | closure L39 | 현재 `BigPlan` 이 `Battle`(메모리태그 9)이 아님 |
| 2 | L40 | `BattlePlan.with_dive == false` |
| 3 | L41 | `BattlePlan.dive_tower == None` |
| 4 | L42 (battle.rs:30 `focus`) | `BattleSubPlanGoal` 이 `RunAway`(태그 4) 또는 `End`(태그 7) = `focus` 필드가 없는 종류 |
| 5 | L43 | `get_entity_by_id(sub_goal.focus)` 가 null (대상 소멸) |
| 6 | L44 · L45 | 대상 `Entity.team != TeamType::Player(1−내팀)` / `get_player_by_champion_id` 가 null(챔피언이 아님) |
| 7 | L47 · L50 | `iter_towers_without_nexus(..).filter(..).next() == None`, 또는 그 타워의 `attack_effect == None` |
| 8 | L38 | `cache.player_champion[team][pos] == None` (내 챔피언 엔티티가 캐시에 없음) |

⟹ 스펙의 「end_reason==7 이면 포기 집계 제외」가 **의미까지 확정**된다:
7 = "다이브할 타워 자체가 없었다" 라서 포기로 세지 않는다.
카디널리티도 독립 재확인:

- `GankStatistics+0x810` = `dive_ep_end_reason` (`[usize; 9]`) — 코드 0~8 이 전부 유효
- `GankStatistics+0x428` = `dive_ep_flicker_reason` (`[usize; 9]`) — ⚠ **다른 코드**다(혼동 주의)

### C4. 05 `last_dive_abandon_tick` 소비 게이트 (ev4, 신규)

읽는 곳 8군데가 **전부 같은 식**이다:
`tick > last_dive_abandon_tick + 1 + 4 × GameSetting.tick_per_second` (≈ 4초 + 1틱 재시도 쿨다운)

- engage.rs:57 `try_engage` (m13.ll:33928)
- engage.rs:112 `try_engage_dive` (m13.ll:34317)
- engage.rs:490 / 581 / 628 / 825 `handle_interact_battle` (m13.ll:38031 / 39039 / 40025 / 44934)
- chat.rs:174 / 226 `handle_chat_inner` (m13.ll:30194 / 30665 — `shl` + `or disjoint 1` 형태로 같은 식)

그리고 **두 번째 writer** 발견: handler.rs:1205 (m13.ll:24140) 에서 현재 tick 을 그대로 store 한다.

- `LegacyPlanHandler+0x1480` = `last_dive_abandon_tick` (IR gep 5248)
- `GameSetting+0x12f8` = `tick_per_second` (IR gep 4856)

### C5. `AbstractGame` vtable 슬롯 공식 (ev3, 신규) — 07[1] · 08[2] 를 닫는다

`슬롯 = 0x20 + 8 × (트레이트 메서드 선언 순서 index)` 이고, `0x18` 은 상위트레이트 `Debug::fmt` 다.
슬롯 순서는 **트레이트 정의**(`game-core\src\simulation.rs`)가 정하므로
**구현체(Game / ExpectedGame / SingleLaneGame / DeathMatchGame)와 무관**하다.
tcx `def_span` 줄번호로 정렬해 독립 산출한 결과가 `divtable.py` 의 정적 vtable 전역 표와 **5/5 일치**:

| 슬롯 | idx | 트레이트 선언 줄 | 메서드 |
|---|---|---|---|
| 0x28 | 1 | simulation.rs:73 | `tick` |
| 0x40 | 4 | simulation.rs:78 | `get_game_mode` |
| 0xf8 | 27 | simulation.rs:125 | `is_visible` |
| 0x100 | 28 | simulation.rs:126 | `is_visible_cell` |
| 0x150 | 38 | simulation.rs:145 | `get_player_by_champion_id` |
| 0x1f0 | 58 | simulation.rs:178 | `get_entity_by_id` |

⟹ `shared.AbstractGame_vtable` · `/specs[7]/mem` · `/specs[8]/mem` 의 vtable 행을 **ev4 → ev3** 권고.

### C6. `<Game as AbstractGame>::get_game_mode` 는 순수 (ev3, MIR) — 08[3] 을 닫는다

`_tcx\mirdump_game_core.txt`:

```
### <game_core::Game as game_core::AbstractGame>::get_game_mode  game.rs:1707
  _2 = &((*_1).2: game_core::MobaMode)      @game.rs:1708
  _0 = game_core::GameMode::Moba(copy _2)   @game.rs:1708
  return                                    @game.rs:1709
```

⟹ L193 · L194 의 두 `get_game_mode().as_moba().unwrap()` 은 **같은 `&MobaMode`** 다(부수효과 0, 고정 필드 차용).
같은 덤프에서 `SingleLaneGame` / `DeathMatchGame` 은 Moba 가 아닌 variant 를 돌려주므로
`as_moba().unwrap()` 이 패닉한다 ⟹ **이 플랜은 Moba 모드 전용**이다.
`GameMode` 태그 0 = Moba 도 tcx `--enum` 으로 재확인(Direct 인코딩, 니치 밀림 없음).

### C7. 05 `prev_holder_hp` / `gap_ticks` 가 버려지는 이유 (ev3, tcx) — 05[3] 을 닫는다

레코드 타입 `game_core::V50DiveEpisode`(ai_interface.rs:443, 104B, 22필드)에 그 두 필드가 **아예 없다.**
Live 타입 `game_ai::plan_legacy::handler::dive_episode::V50DiveEpLive`(120B, 20필드)에만 있다.
⟹ 「접는 과정에서 버려진다」가 아니라 **레코드 스키마에 없는 라이브 전용 스크래치**다.

- `V50DiveEpLive+0x0` = `prev_holder_hp` (`Option<usize>` 16B)
- `V50DiveEpLive+0x50` = `gap_ticks` (usize)

### C8. 08 self 4필드의 소비처 (ev4) — 08[0] 을 닫는다

같은 플랜의 `sub_plan`(m10.ll:7902~9384)이 전부 읽는다:

- `EpicHuntAndPokePlan+0x18` = `focus_epic_only` → hunt_and_poke.rs:35
- `EpicHuntAndPokePlan+0x19` = `vision_only` → hunt_and_poke.rs:35
- `EpicHuntAndPokePlan+0x1a` = `v46_flee` → hunt_and_poke.rs:124 `check_recall`(sub_plan:79 에 인라인)
- `EpicHuntAndPokePlan+0x0` = `v46_flee_threats` (`Vec<usize>`) → hunt_and_poke.rs:125

⟹ `is_end` 가 안 읽는 것은 결함이 아니라 진입점 분담이다.

### C9. 09 미니언 게이트 전문 (ev4, 신규) — 09 의 마지막 큰 미탐색을 대부분 닫는다

`game_ai::enemy_minion_line_action_danger_damage_at`(minion_wave_risk.rs:233, m07.ll:48226~48390).
인자 이름은 DWARF `!60288`~`!60295` 실측:

```rust
fn enemy_minion_line_action_danger_damage_at(
    version: usize,            // 죽은 인자 (#dbg_value(i64 poison))
    data: &OperationData, target: &Entity,
    x: u64, y: u64,            // L234
    window_tick: usize, champion_action: bool, predict_retarget: bool) -> usize
{
    let damage = enemy_minion_line_action_damage_at(              // L235 (version 자리에 i64 poison)
        version, data, target, x, y, window_tick, champion_action, predict_retarget);
    let is_dangerous = if champion_action || enemy_minion_wave_has_epic_buff(data, target) {  // L236
        enemy_minion_wave_is_dangerous(target, damage)            // L237
    } else {
        enemy_minion_wave_is_critical(target, damage)             // L239
    };
    if is_dangerous { damage } else { 0 }                         // L241
}
```

`enemy_minion_wave_is_dangerous(&Entity, usize) -> bool` (minion_wave_risk.rs:64, `in:game_ai`):

```
L65  if dmg == 0 { return false }
L69  hp_pct  = target.hp * 100 / max(target.stat_cached.hp, 1)
L70  dmg_pct = dmg * 100 / max(target.hp, 1)
L71  if dmg >= target.hp || dmg_pct > 49 { true }
L73  else if hp_pct < 66 && dmg_pct > 29 { true }
L74  else if hp_pct < 41 && dmg_pct > 17 { true }
L75  else if hp_pct < 26 && dmg_pct >  9 { true }   // 이 줄만 select 로 단축평가 보존(hp_pct 가 좌항)
     else { false }
```

`enemy_minion_wave_is_critical(&Entity, usize) -> bool` (minion_wave_risk.rs:79, `in:game_ai`):

```
L80  if dmg == 0 { return false }
L84/85  hp_pct / dmg_pct = 위와 같은 식
L86  if dmg >= target.hp { true }
L87  else if hp_pct < 26 && dmg_pct > 34 { true }
L88  else if hp_pct < 16 && dmg_pct > 19 { true }
     else { false }
```

`enemy_minion_wave_has_epic_buff(&OperationData, &Entity) -> bool` (minion_wave_risk.rs:92, `in:game_ai`):
`target.team.player_team()`(entity.rs:1135, MIR: 태그 0 = `Player(team)` → Some / 태그 1 → None)이 None 이면 false →
`data.cache.game.get_game_mode().as_moba()` 가 None 이면 false →
`MobaMode::remain_epic_time(1 − team) != 0` (MIR: `self.epic_minion_buff_time[i]`, 배열 길이 2 바운드체크).

- `MobaMode+0x240` = `epic_minion_buff_time` (`[usize; 2]`)

★09 에 대한 함의 2개:

1. 09 는 `champion_action = true` 를 넘긴다(m15.ll 호출부 7번째 인자 리터럴 `true`) ⟹ **항상 `is_dangerous` 표**를 쓴다.
2. HP 비교 대상 `target` 은 **자기 챔피언**이다(호출부 arg3 = `%18` = `player_champion[team][pos]`), 좌표만 적의 것이다.
   ⟹ 09 게이트의 뜻 = "지금 적 위치로 2초 안에 붙으면 **내가** 미니언 라인에 위험한 피해를 받는가".

남은 **미탐색** = `enemy_minion_line_action_damage_at`(minion_wave_risk.rs:130, **pub**) 본문.
오라클은 가능하지만 미니언이 실제로 있어야 `damage > 0` 이 나온다(현재 프로브 환경은 `damage = 0`).

---

## 4. `ev<=3` 표본 재확인 (9건, 뒤집힘 1건 = E1)

| 대상 | 결과 |
|---|---|
| `/specs[5]/sig` 파라미터 이름·줄(ev3) | OK — DWARF `!35907`~`!35911` 전부 일치(dive_episode.rs:125). `no_contact` = `!35914`(L138), `n` = `!35918`(L145) |
| `/specs[5]/mem` 30행 전수(ev4) | OK — tcxdict 전수 일치 |
| `/specs[6]/knobs` 40000000001 · 22500000001(ev3) | OK — IR 재실측 일치, `<=` 접힘 서술도 일치 |
| `/specs[7]/sig.tcx`(ev3) | OK |
| `/specs[8]/sig.tcx`(ev3) | 문자열 자체는 OK — 단 `params[5].type` 이 이와 모순(**E1**) |
| `/specs[8]/mem` `iter_champions` 1904(ev3) | OK — tcx `def_span` = simulation.rs:1904:3-1904:85, `{closure#0}` = 1905:50-1905:53. 2차 정정이 맞고 1차 지적이 틀렸다. 곁가지 2건도 확인(`Position::as_index` = entity.rs:580, `Entity::distance_sq` = entity.rs:2157) |
| `/specs[9]/sig/params[0]`(ev2) | OK — poison 재확인(단 open[0] 과 모순 = **E2**) |
| `/specs[9]/open[1]` src_line 1196(ev3) | OK — tcx `fight_check.rs:1196:3` |
| `/specs[9]/open[3]` select 단축평가(ev3) | OK — `m15.ll:35598~35601` 일치 |

---

## 5. `callees_unmatched` — 판정 술어 혼입 **0건**

| 스펙 | 미매칭 이름 | 판정 |
|---|---|---|
| 05 | `gap_ticks` `grow_one` `is_contained_in` `llvm.umax.i64` `starts_with` `take` | 전부 std / intrinsic / 필드명. `calls_raw` 원문 확인: `core::str::pattern::Pattern::is_contained_in` · `core::slice::starts_with`. **게임 술어 없음** |
| 06 | (없음) | — |
| 07 | `discriminant` `payload` | `logic` 산문에서 긁힌 잡음 |
| 08 | `llvm.usub.sat.i64` | `saturating_sub` 의 intrinsic |
| 09 | `dist_sq` | **표기 권고(오류 아님)**: 실제 이름은 `game_core::utils::distance_sq(u64,u64,u64,u64) -> u64`(utils.rs:6, pub, xinl). `logic` 의 약칭 때문에 자동 매칭이 실패했다. `logic` 안 `dist_sq` → `distance_sq` 로 통일하면 잡힌다. `calls_raw` 에는 애초에 없다(인라인) |

---

## 6. 과제 지시문 / BRIEF 오류 (BRIEF §0 이 요구한 산출물)

1. **틀림** — 「05 `end_reason` 0~8 의 의미 제3출처는 exe 디스어셈만 남았다」.
   호출자 IR(`m13.ll` 의 `LegacyPlanHandler::update` 에 인라인된 `v50_track_dive_episode`)에 코드 산출 지점이 전부 있다(§3 C3).
   BRIEF §2 가 경고한 「가진 재료의 한계를 문제의 한계로 착각」의 재발이다.
   다만 **spec 쪽 표기는 `미탐색` 이라 옳았다** — 틀린 것은 과제 지시문의 범위 판정이다.
2. **오기** — 「07 L36 **두** OR 항」 → 실제 **세 항**. spec 쪽이 맞다.
3. **맞음** — 「09 version 은 죽은 인자」. 그리고 그 때문에 spec `open[0]` 이 자기모순이다(**E2**).
4. **맞음** — 「08 `15 × tick_per_second` 는 `MobaMode.next_respawn_tick` 을 직접 써 넣으면 열린다」. 실제로 열렸다(§3 C1).
5. **맞음** — BRIEF §3① `init_tower` / `init_nexus` 재호출 금지. 두 프로브 모두 미호출.
6. **맞음** — BRIEF §3① `GameSetting::default().tick_per_second == 0`, 그리고 `MapDef::moba()` 앞에서 60 을 넣어야 한다는 것까지 그대로.
7. **맞음** — BRIEF §2 「open 에 이미 닫힌 게 섞여 있으면 보고」. 실제 사례 = `/specs[8]/open[1]`.

---

## 7. ev 상향 권고 (값은 그대로, 등급만)

- `/specs[8]/consts` value 15 → **ev 2** (오라클 900/901 경계)
- `/specs[8]/consts` value 1 (ObjectPhase::Setup) → **ev 3** (`take_setup_like` MIR `Eq(_4, const 1_isize)` + tcxdict --enum)
- `/specs[8]/consts` value 0 (MainObjective::Morgard) → **ev 3** (`take_active` MIR + tcxdict --enum idx 0 / 태그 0)
- `/specs[8]/consts` value 4 (JungleType::Morgard) → **ev 3** (tcxdict --enum JungleType 태그 4)
- `/specs[7]/consts` value 51 → **ev 2** (오라클 50/51 경계)
- `/specs[7]/consts` value 5 · 9 · 11 → **ev 3** (tcxdict --enum SubPlan) + 런타임 확인
- `/specs[7]/mem` · `/specs[8]/mem` 의 vtable 행 4개 → **ev 3** (§3 C5)
- `/specs[5]/open[1]` ev 5 → **ev 3** (closed 로 이동하며)

---

## 8. 재현 명령

```
sh  C:\tfm2mods\MIG\_verify2\A\A2_build.sh C:\tfm2mods\MIG\_verify3\B\B3_o08.rs
%TEMP%\tfm2_spanprobe\B3_o08.exe
sh  C:\tfm2mods\MIG\_verify2\A\A2_build.sh C:\tfm2mods\MIG\_verify3\B\B3_o07.rs
%TEMP%\tfm2_spanprobe\B3_o07.exe
```

---

## 9. 제출 게이트 결과 + 도구 결함 1건

```
python -X utf8 tcxaudit.py --prose _verify3\B\B3_REPORT.md
  -> 총 696건  오귀속=0  밀림=0  부분일치=1(기존 specs20.json 건)  확인불가=18(기존)  OK=677
     (이 문서 추가분 17건 전부 OK. 기준선 679건 -> 696건)
python -X utf8 specgate.py
  -> G1 자기모순=0  G2 호출부 전수=0  G3 형제 함수=0  G4 술어 시그니처=0   총 0건
```

★**도구 결함(권고)**: `specgate.py` 의 G1 이 **0건**인데 이 배치는 자기모순 **2건**(E1·E2)을 찾았다.
G1 은 현재 ①짝 토큰 대조(`i64::MAX` vs `usize::MAX` 류) ②`constants[].src_line` vs `meaning` 만 본다.
다음 두 검사를 넣으면 E1·E2 유형이 **기계적으로** 잡힌다:

1. `sig.tcx` 를 파싱해 얻은 인자 타입 목록 ↔ `sig.params[].type` 의 `&` / `&mut` · 타입 이름 대조
   (E1 = `&TeamPlan` vs `&mut TeamPlan` — 한 필드 비교로 잡힌다)
2. 같은 스펙 안에서 **같은 대상에 대한 결론이 반대**인지: `sig.params[i].role` 과 `open[].q` /
   `closed[].q` 가 같은 식별자(예: `version`)를 다루면서 한쪽은 「죽은 인자」, 다른 쪽은
   「살아 있는 게이트」라고 쓰는 경우 (E2). 어휘 쌍 예: `죽은 인자` ↔ `살아 있는`,
   `결과 불변` ↔ `하드코딩 금지`

`tcxaudit` 은 base+offset 만 보므로 이 오염을 원리적으로 못 잡는다(`meta.corrections` 가 이미 지적).
