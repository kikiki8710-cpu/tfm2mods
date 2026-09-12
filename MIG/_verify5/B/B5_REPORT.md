# 5차 반증검증 — 배치 B (`specs[5]`~`specs[9]`) 보고서

게임 0.5.8 / 2026-09-11 / 담당 = `05 v50_fold_dive_episode` · `06 v2_response_retreat_stance` ·
`07 EpicHuntAndBattlePlan::sub_plan` · `08 EpicHuntAndPokePlan::is_end` · `09 check_favorable_engage_formation`

산출물 전량 = `MIG\_verify5\B\` (프로브 6개 + TSV 6개 + 이 문서). **`_spec\*.json` 은 읽기만 했다.**

---

## 1. 결론 한 줄

**실오류 7 · 판정반전 3 · 새 발견 10 · `ev4 → ev2` 상향 46행.**
`open` 3건 중 **1건은 class 오분류였고(09), 2건은 그대로 유효**(05 grow_one · 07 OR 항 순서).
`notes` 1건(09)은 반증 시도 결과 **유지**.

---

## 2. 정정 목록

### E1 (실오류·판정반전) `08` v23 6번째 인자는 「틱 창」이 아니라 **HP% 하한**

- 경로 `/specs[8]/knobs[7]`
- 구 → 신
  - `what`: `"최근 가시" 창` → **`캠프 주변 적 카운트의 적 HP% 하한 (min_hp_ratio)`**
  - `value`: `40틱` → **`40` (= `hp*100/stat_cached.hp < 40` 이면 그 적을 세지 않는다)**
  - `effect`: `적 존재 인정 시간` → **`올리면 건강한 적만 세어 v24 가 「풀어라(true)」를 더 자주 내고, 내리면 빈사 적까지 세어 셋업을 더 오래 유지한다. 「최근」의 실제 창은 이 인자가 아니라 Blackboard::is_recent_visible 내부의 고정 120틱이다`**
  - `ev`: `4` → **`2`**
- 근거 3중
  1. **DWARF 인자명** — `_gaibc/m15.ll` 의 `!61645 = !DILocalVariable(name: "min_hp_ratio", arg: 6, ... line: 31)`.
     같은 위치의 5번째는 `!61644 ... name: "range"` 다(`radius` 아님). 선언 = `objective_helpers.rs:31`.
  2. **IR 본문** — `_gaibc/m15.ll:55684~55692`:
     `%26 = load Entity+0x628` → `%27 = icmp eq %26, 0`(0이면 div_by_zero 패닉) →
     `%30 = load Entity+0x670` → `%31 = mul %30, 100` → `%32 = udiv %31, %26` →
     **`%33 = icmp ult i64 %32, %5`** → 참이면 그 슬롯 skip. `%5` = 6번째 인자.
     거리 비교는 그 **다음** 블록(`%50 > %20`, `%20 = mul %4,%4`)이고, 최근성은 그 뒤의
     `Blackboard::is_recent_visible` 호출이 담당한다.
  3. **오라클 실행** `B5_o3.tsv` `#H` — 적 1명을 캠프에서 100000 거리에 두고 HP% 만 흔들었다:
     `1·20·38·39 → v24=true` / `40·41·60·100 → v24=false`. 경계가 **정확히 39/40**.
     정수나눗셈 축도 일치(`#H2`: 399/1000 → true, 400/1000 → false).
- 동일 오류가 **같은 명세의 다른 자리에도** 있다 → E2.

### E2 (실오류) `08` history 서술의 같은 오기

- 경로 `/specs[8]/history[?]`(= `v24_objective_setup_should_release_to_passive 내부 미독해` 항목의 `now`)
- 구 → 신
  `if v23_recent_visible_enemies_near_point(player, data, camp.x, camp.y, radius=180000, recent=40틱) != 0 { return false }`
  → **`if v23_recent_visible_enemies_near_point(player, data, camp.x, camp.y, range=180000, min_hp_ratio=40) != 0 { return false }`**
- 근거: E1 과 동일. ★이 항목은 `now` 쪽에 **「★확정」**으로 적혀 있었다 ⟹ **판정반전 R1**.
- ★구조적 교훈: `knobs[].what/value` 와 `history[].now` 가 **같은 오독을 두 군데** 갖고 있었고
  `G5`/`G6`/`G8` 셋 다 못 잡았다(세 게이트 모두 `logic` 산문 ↔ 표만 대조한다).

### E3 (실오류) `08` knobs[0].note = **오귀속 + STALE**

- 경로 `/specs[8]/knobs[0]/note`
- 현재 내용: `★오라클로 15 * tick_per_second 임계를 측정하지 못했다(입력 판별력 부재, 2차배치B) …
  미탐색 = MobaMode.next_respawn_tick 주입 / 틱 진행`
- 문제 2개
  1. **오귀속** — 이 note 는 `knobs[1]`(에픽 리젠 대기 `15`)의 것이다. `knobs[0]` 은 거리 임계 `22500000000` 이다.
  2. **STALE** — 그 미탐색은 **3차에 이미 해소**됐다(`/specs[8]/consts[5]` 의 `★오라클 13/13 … 정확히 900`).
     이번 5차에도 재현했다(`B5_o3.tsv` `#T`: 잔여 900 → false, 901 → true).
- 신: note 삭제(또는 `knobs[1]` 로 이동하되 「3차에 해소, 임계 900 확정」으로 갱신).

### E4 (실오류) `08` knobs[0].ev = 2 가 **근거 없는 상향이었다**

- 경로 `/specs[8]/knobs[0]/ev`
- 5차 착수 시점에 `22500000000` 은 **오라클로 측정된 적이 없다.** 같은 명세가
  `/specs[8]/consts[5]` 에서 「setup 경로는 `v24_…=true` 가 먼저 발화해 **(c)(d) 는 오라클 미도달**」이라고
  명시하는데, `22500000000` 은 **(d) 경로 전용 상수**다 ⟹ ev2 는 성립할 수 없었다.
- 신: 이번 라운드에 **실제로** ev2 가 됐다(E-신규① 참조) — 값은 그대로 `2` 로 두되 근거를 이번 실행으로 교체.

### E5 (실오류) `08` 같은 사실에 ev 가 둘

- 경로 `/specs[8]/knobs[1]/ev`
- 구 → 신: `4` → **`2`**. 같은 사실(`15 × tick_per_second`)을 `/specs[8]/consts[5]` 는 이미 `ev 2`
  (오라클 13/13)로 갖고 있다. 5차 재현 = `B5_o3.tsv` `#T`.

### E6 (실오류) `09` 1286 의 `9` 는 **노브가 아니다**(0~74 구간에서 완전 무효)

- 경로 `/specs[9]/knobs[3]`
- 구 → 신
  - `what`: `후퇴로/측면 판정 임계(cos²·sin² 공통)` → **`후퇴로 판정 임계(cos², 1283 전용)`**
  - `effect` 에 다음을 추가: **`★1286 의 sin² 판정(cross_sq*100 > lp*9)은 도달 시 항상 참이라
    이 값을 0~74 어디로 바꿔도 결과가 바뀌지 않는다(무효 노브). 실효 노브는 1283 뿐이다.`**
  - `ev`: `4` → **`2`**(1283 쪽만)
- 근거 ①**대수 증명** — `dot² + cross² = lp` 가 정수에서 정확히 성립한다(라그랑주 항등식, `ally_len_sq ≥ 1`·
  `retreat_len_sq ≥ 1` 이 앞 분기에서 보장됨). 1302 에 도달하는 조건은 `!is_front && !is_rear` 이고
  - `dot > 0` & `!is_front` ⟹ `4·dot² ≤ lp` ⟹ `cross² ≥ 0.75·lp` ⟹ `100·cross² ≥ 75·lp > 9·lp` (참)
  - `dot < 0` & `!is_rear` ⟹ `100·dot² ≤ 9·lp` ⟹ `cross² ≥ 0.91·lp` ⟹ `100·cross² ≥ 91·lp > 9·lp` (참)
  - `dot == 0` ⟹ `cross² = lp` ⟹ `100·lp > 9·lp` (참)
  ⟹ **항상 참**. 구속조건은 `dot>0` 가지의 75 이므로 임계가 **75 미만이면 무효**다.
- 근거 ②**실행** `B5_o1.tsv` `J_sweep` — 난수 200만 + `J_exhaust(-6..6)^4` 전수 격자 28,561 에서
  `lagrange_violations = 0`.
- 근거 ③**민감도** `B5_o2.tsv` `#SENS nflank(1286)` — 같은 3,000 시나리오에서
  `0·1·5·9·25·50·74·75 → true 789건(델타 ±0)`, `80 → -2`, `91 → -5`, `100 → -34`.
  ⟹ 셋 다 같은 결론.

### E7 (실오류·class 오분류) `09` open[0] = `재료 부재` → **`미탐색`**

- 경로 `/specs[9]/open[0]/class`
- 구: `재료 부재` (`game-ai\src\fight_check.rs 원본 소스가 이 환경에 없어 대조하지 못했다`)
- 신: **`미탐색`** + 탐색 범위 = **`IR_TOOLKIT.md §8 「줄 길이 산술」(rmeta_srcmap)`**
- 근거: `fight_check.rs` 는 rmeta SourceMap 에 **있다** —
  `python rmeta_srcmap.py game_ai fight_check 1196 1260` 실행 결과
  `src_len=57853 raw_len=59270 lines=1417 w=1 mb=3285 crlf=1417`, 파싱 에러 0.
  tcx `sp`(1196:1–1202:10) 과 `L1196=49B … L1202=12B` 가 7줄 시그니처로 정합한다.
  ★같은 방법을 **06 이 이미 썼다**(`engage.rs` L22 「53자 ±0」). 그 값을 이번에 교차검증했다:
  `rmeta_srcmap game_ai handler\engage 13 34` → `L22 bytes=54` = 53 + LF 1 ⟹
  IR_TOOLKIT §8 의 「`bytes` 는 LF 를 포함, 내용 = 값 − 1」 규약이 실측으로 재확인된다.
  ⟹ 「원본 소스가 없다」는 참이지만 **「그래서 대조 불가」는 거짓**이다.
- ★게이트 사각지대: `G10 class 오분류 = 0` 인데 이 건을 못 잡는다. G10 은 「확정된 사실이 `미탐색` 자리에
  앉은 것」만 보고, **반대 방향(도달 가능한 것이 `재료 부재` 자리에 앉은 것)** 을 안 본다.

---

## 3. 판정반전 3건

| # | 대상 | 구 판정 | 신 판정 | 근거 |
|---|---|---|---|---|
| **R1** | `08` v23 6번째 인자 | 「★확정 = `recent=40틱`」 | **`min_hp_ratio=40`(HP% 하한)** | E1 (DWARF 인자명 + IR + 오라클 39/40) |
| **R2** | `08` 경로 (c) `L183 nearest==None → return true` | 「오라클 **미도달**(미탐색)」 | **구조적으로 도달 불가(죽은 경로)** | §4-② |
| **R3** | `09` `L1307 else → front_allies += 1` | 살아 있는 분기로 서술 | **도달 불가(죽은 가지)** | E6 의 대수 증명 + 실행 0건 |

### R2 증명 — `08` (c) 는 왜 죽었나

1. `is_end` 가 L179 에 도달하려면 `setup_like && v24 == false` 여야 한다.
2. `v24` 의 exit phi(`_gaibc/m09.ll:22157`)는
   `[ %45(pressure_line), %41 ] · [ false, %5 ] · [ false, %18 ] · [ false, %14 ] · [ false, %10 ] · [ true, %31 ]` 이고
   `%31` 이 곧 `is_visible_cell == true` 반환 경로다.
3. ⟹ **`v24 == false` 이면서 `is_visible_cell == true`** 이려면 `%18` 로 빠져야 하고,
   `%18` 은 `v23_recent_visible_enemies_near_point(...) != 0` 가지다.
4. v23 의 슬롯 술어 = `player_champion[1-team][p].is_some()` ∧ `HP% ≥ 40` ∧ `캠프 거리 ≤ 180000`
   ∧ `Blackboard::is_recent_visible(...)` — **is_end 의 L180 필터(`is_some` ∧ `is_recent_visible`)의 진부분집합**이고,
   **같은 배열·같은 틱·같은 blackboard** 를 쓴다.
5. ⟹ `v23 != 0` 이면 L181 `min_by_key` 에 원소가 최소 1개 있다 ⟹ `nearest == None` 이 성립 불가.
6. 실행 확인 `B5_o3.tsv` `#C` — 적 슬롯 전부 `None` / 전부 저HP / 캠프 셀 시야 차단 **3종 전부**
   `v24=true` 로 (b) 가 먼저 나간다.
⟹ **(c) 는 미탐색이 아니라 도달 불가**다. `open`/`notes` 에 「도달 불가(범위 = setup 게이트가 켜진 경우)」로 적어야 한다.
 ⚠범위 명시: 이 증명은 **`objective == Some(Morgard{Setup})` 경로**에 대한 것이다. `Serpen` 판
 (`serpen/hunt_and_poke.rs:162`)은 같은 구조로 보이지만 이번에 확인하지 않았다.

---

## 4. ★`ev4 → ev2` 로 올린 행 (이번 라운드 주 산출물, 46행)

프로브 = `B5_o1.rs` ~ `B5_o6.rs`, 출력 = 같은 이름의 `.tsv`. 전부 `setting_ok=true`(`real_setting()` 사용,
`init_tower`/`init_nexus` 미호출 — `TEMPLATE.rs` 함정 ①②).

### ① `09` — 임계 **단독 격리** 44/44 MATCH (`B5_o1.tsv`) + 3,000/3,000 재구현 대조 (`B5_o2.tsv`)

2·4차의 400/400·1800/1800 은 **합성 동작**을 맞춘 것이라 상수 하나하나의 경계는 안 갈렸다.
이번엔 아군 배치를 기하학적으로 설계해 **상수 하나만 움직이면 결과가 뒤집히는** 케이스를 만들었다.

| 경로 | 구 → 신 | 경계 실측 | 케이스 |
|---|---|---|---|
| `/specs[9]/consts[2]` (40) | ev4 → **ev2** | `HP% 39 → false / 40 → true` | `A_hp`, `A2_hpdiv`(399·400/1000) |
| `/specs[9]/consts[3]` (100) | ev4 → **ev2** | 백분율 스케일 = `hp*100/max` 정수나눗셈 | `A2_hpdiv` |
| `/specs[9]/consts[1]` (100000) | ev4 → **ev2** | `d = er+100000 → 포함 / +1 → 제외`, er 3종(100000·200000·400000) 전부 | `B_slack` |
| `/specs[9]/consts[5]` (4) | ev4 → **ev2** | `dy 173205 → front / 173206 → flank` (= `3dx² > dy²`) | `C_front4` |
| `/specs[9]/consts[4]` (9) | ev4 → **ev2** | `dy 158989 → rear / 158990 → flank` (= `91dx² > 9dy²`) | `D_rear9` |
| `/specs[9]/consts[6]` (5) · `/specs[9]/consts[7]` (6) | ev4 → **ev2** | `champ_to_base 547722 → true / 547723 → false` | `E_ratio56` |
| `/specs[9]/consts[8]` (1) | ev4 → **ev2** | 적이 분수 중심 위 → 무조건 true / +1 → false · 겹친 아군 = front | `F_zeroR`, `G_overlap_front` |
| `/specs[9]/knobs[0]` · `knobs[1]` · `knobs[2]` · `knobs[3]` · `knobs[4]` | ev4 → **ev2** | 위 경계 + `#SENS` 민감도표 | `B5_o2.tsv` |
| `/specs[9]/knobs[6]` (인원 조건) | ev4 → **ev2** | `rear1→true / front1→false / front2+비율→true / flank1→false / flank2→true / flank1+front1→true` | `H_*` 6종 |
| `/specs[9]/mem` `Entity+0x660` | ev4 → **ev3 권고** | 런타임 오프셋 출력 `x=0x660` | `B5_o1.tsv` `entity_offsets` |
| `/specs[9]/mem` `Entity+0x668` | ev4 → **ev3 권고** | 런타임 오프셋 출력 `y=0x668` | 〃 |
| `/specs[9]/mem` `Entity+0x670` | ev4 → **ev3 권고** | 런타임 오프셋 출력 `hp=0x670` | 〃 |
| `/specs[9]/mem` `Entity+0x628` | ev4 → **ev3 권고** | 런타임 오프셋 출력 `stat_cached.hp=0x628` | 〃 |
| `/specs[9]/mem` `Entity+0x5c0` | ev4 → **ev3 권고** | 런타임 오프셋 출력 `id=0x5c0` | 〃 |

★**미상향(정직하게 남김)**: `/specs[9]/consts[0]`(2 = `tps×2` 미니언 창)은 **미니언이 필요해 이번에도 ev4**다.
탐색 범위 = `minion_wave_setting` 16필드 + `melee/range_minion` 주입 + `run_tick` 600틱(4차 배치B 레시피).

### ② `08` — **(d) 경로 최초 도달** (`B5_o3.tsv`)

`v24` IR 을 읽어 「`v23 != 0` 이면 `v24=false` 로 빠진다」는 창을 찾은 것이 열쇠였다.

| 경로 | 구 → 신 | 경계 실측 |
|---|---|---|
| `/specs[8]/consts[4]` (22500000000) | ev4 → **ev2** | 캠프 거리 `150000 → false(폴스루) / 150001 → true((d))` |
| `/specs[8]/knobs[0]` | ev2(근거 교체) | 〃 — E4 참조 |
| `/specs[8]/knobs[6]` (180000) | ev4 → **ev2** | `180000 → v24=false((d)) / 180001 → v24=true((b))` |
| `/specs[8]/knobs[7]` (→ min_hp_ratio 40) | ev4 → **ev2** | `39/40` 에서 v24 반전 |
| `/specs[8]/knobs[1]` (15) | ev4 → **ev2** | 리젠 잔여 `900 → false / 901 → true` |
| `/specs[8]/knobs[2]` (setup 게이트) | ev4 → **ev2** | `#P`: `None/Assemble/Hunt → v24=false·is_end=false`, `Setup → v24=true·is_end=true` |
| `/specs[8]/consts[3]` (32000) | ev4 → **ev2** | 캠프 `288000` → 셀 `(9,9)`, `is_visible_cell(0,9,9)=true` 로 L179 진입 |
| `/specs[8]/mem` `TeamPlan+0x41f` | ev4 → **ev2** | `#P` 진리표 |
| `/specs[8]/mem` `TeamPlan+0x420` | ev3 → **ev2** | 〃 (phase 4종) |
| `/specs[8]/mem` `MobaMode+0x1b0` | ev3 → **ev2** | `#T` 잔여 0/899/900/901/1000 |

★**미상향**: `/specs[8]/knobs[4]`(필터 술어 `is_recent_visible`)은 술어 자체라 값 경계가 없다. 단 그 술어의
120틱 창은 아래 ④에서 ev2 가 됐다.

### ③ `07` — 3분기 + 게이트 4종 전부 경계 격리, 31/31 MATCH (`B5_o4.tsv`)

| 경로 | 구 → 신 | 경계 실측 |
|---|---|---|
| `/specs[7]/consts[0]` (100) | ev4 → **ev2** | `509/1000 → Recall / 510/1000 → EpicHunt` (정수나눗셈) |
| `/specs[7]/consts[2]` (태그 5) · `consts[3]` (9) · `consts[4]` (11) | ev3 → **ev2** | 런타임 sret 태그 실측 |
| `/specs[7]/consts[5]` (1 = Outline) | ev4 → **ev2** | Hide 페이로드 `out_line=1` |
| `/specs[7]/consts[6]` (0) | ev4 → **ev2** | `check_move=0` · `enemy_spotted_me=0` · `need_recall=0` · `live_list.get(0)` |
| `/specs[7]/knobs[0]` · `knobs[4]` (51) | ev4 → **ev2** | `hp 50 → Recall / 51 → EpicHunt` (챔프를 분수 밖으로 옮긴 뒤) |
| `/specs[7]/knobs[3]` · `knobs[5]` (은신 게이트 `tps` 계수 1) | ev4 → **ev2** | `tps60`: `k=1059 → Hide / 1060 → EpicHunt`(a=1000) · `tps30`: `29/30` · `tps1`: `0/1` — **계수 1 · `>` 엄격 확정** |
| `/specs[7]/knobs[6]` (에픽 무손상 `==`) | ev4 → **ev2** | 에픽 `hp=max → Recall` / `hp=max−1 → EpicHunt` |
| `/specs[7]/knobs[1]` · `knobs[7]` (Hide.out_line) | ev4 → **ev2** | bush 0·7·26 전부 `out_line=1` |
| `/specs[7]/knobs[2]` (need_recall 0) | ev4 → **ev2** | EpicHunt 페이로드 `+0x8 = 0` |
| `/specs[7]/mem` `GoalData+0x98` | ev4 → **ev2** | 런타임 오프셋 실측 + 게이트가 이 값에 반응 |
| `/specs[7]/mem` `GoalData+0xa0` | ev4 → **ev2** | 〃 |
| `/specs[7]/mem` `MobaMode+0x1a8` | ev4 → **ev2** | `len=0` → None / 존재하지 않는 id → None / 실 id → Some |
| `/specs[7]/mem` `EpicHuntAndBattlePlan+0x0` | ev4 → **ev2** | `transmute([0,·])` = None, `transmute([1,b])` = Some(b) 쌍으로 자기검증 |
| `/specs[7]/mem` `EpicHuntAndBattlePlan+0x8` | ev4 → **ev2** | bush 0·7·26 그대로 관통 |
| `/specs[7]/mem` `SubPlan+0x8` | ev4 → **ev2** | Hide.bush / EpicHunt.need_recall |
| `/specs[7]/mem` `SubPlan+0x10` | ev4 → **ev2** | `out_line = 1` |
| `/specs[7]/mem` `SubPlan+0x11` | ev4 → **ev2** | `check_move = 0` |
| `/specs[7]/mem` `SubPlan+0x12` | ev4 → **ev2** | `enemy_spotted_me = 0` |
| `/specs[7]/mem` `MapDef+0x6d70` | ev2 유지 | 사각형 4경계 포함성까지 추가 확정(아래 N4) |

### ④ `06` — 담당 함수가 `in:game_ai` 라 **피호출 술어를 개별 실행** (`B5_o5.tsv`)

| 경로 | 구 → 신 | 실측 |
|---|---|---|
| `_shared/is_recent_visible/핵심` (120틱) | → **ev2** | 경과 `0·60·119·120 → true`, `121·200·5000 → false`. **창 = 120틱 이하(`+120 >= tick`)** |
| `_shared/is_recent_visible/blackboard_인덱스_의미` | → **ev2** | 슬롯0만 stale 로 두고 1~4 를 최신으로 해도 `false` ⟹ **`owner.info.position` 으로만 인덱싱** |
| `/specs[6]/mem` `Entity+0x660` · `Entity+0x668` (distance_sq) | ev4 → **ev2** | `Entity::distance_sq` = `|dx|²+|dy|²`, 5케이스 일치 + `utils::distance_sq` 와 동일값 |
| `/specs[6]/consts[2]` (태그 3) | ev4 → **ev3** | `tcxdict --enum BattleSubPlanGoal`: `KitingBack` 메모리태그 **3**, 페이로드 `focus: usize @ +0x8` |
| `/specs[6]/consts[3]` (태그 4) | ev4 → **ev3** | 같은 표: `RunAway` 태그 **4**, **필드 없음** ⟹ 명세의 「페이로드 undef 정상」 재확인 |
| `/specs[6]/knobs[4]` (towers 빈 Vec) | **미상향(ev4 유지)** | 아래 N7 — 판별력 부재. 「towers 무영향」이라고 **쓰지 않는다** |

### ⑤ `05` — `BigPlan::get_name` **16/16 전수** (`B5_o5.tsv` `#A`) + 구조체 실체 확인 (`B5_o6.tsv`)

2차가 「나머지 8 variant 는 private 필드라 구성 불가」로 남긴 것을 `transmute`(0 페이로드 + 태그만) 로 전부 열었다.
**명세 `history` 의 16행 표와 불일치 0.**

| 경로 | 구 → 신 | 실측 |
|---|---|---|
| `/specs[5]/consts[0]` (−1) | ev4 → **ev2** | `LegacyPlanHandler+0x570` 초기값 = **−1**, `size_of = 6168` |
| `/specs[5]/consts[2..10]` (end_plan 1..9) | ev4 → **ev2**(이름 문자열 부분) | 16 variant 이름 전수. 사슬 적용부는 함수가 private 이라 **ev4 유지** |
| `/specs[5]/history` 함정 ①(`starts_with("Recall")` 죽은 가지) | → **ev2** | 16종 중 `"Recall"` 로 시작하는 이름 **0건**(`ActiveRecall` 은 앞 분기가 먹는다) |
| `/specs[5]/history` 함정 ②(`SinglePlanBattle`·`DeathMatchBattle` → 9) | → **ev2** | 실제 이름 `"SinglePlanBattle goal: …"` · `"DeathMatchBattle goal: …"` ⟹ `starts_with("Battle")` 불성립 |
| `/specs[5]/mem` `LegacyPlanHandler+0x1480` | ev4 → **ev2** | 초기값 0 실측 |
| `/specs[5]/mem` `LegacyPlanHandler+0x1811` | ev4 → **ev2** | 초기값 0 실측 |
| `/specs[5]/mem` `LegacyPlanHandler+0x888` | ev4 → **ev2** | 초기 cap 0 |
| `/specs[5]/mem` `LegacyPlanHandler+0x898` | ev4 → **ev2** | 초기 len 0 |
| `/specs[5]/mem` `LegacyPlanHandler+0x5e8` | ev4 → **ev2** | 초기 BigPlan 태그 = **3 = PassiveLine** (`tcxdict --enum BigPlan` 과 정합) |

★**ev 서열에 관한 의견** — `mem`(오프셋) 행을 `ev2` 로 올리는 것은 **`METHOD_MAP §5` 신뢰서열과 충돌**한다:
`tcx(정본) > 오라클 실행`. 오프셋의 정본은 tcx 이므로 `mem` 행의 올바른 상한은 **ev3** 이고,
`ev2` 가 의미를 갖는 것은 「그 오프셋이 **판정에 실제로 개입한다**」는 동작 주장을 실행으로 확인했을 때다.
위 표에서 `mem` 행에 `ev2` 를 쓴 것은 후자(게이트 반응 확인)에 해당하는 것들이고,
순수 레이아웃 행(`Entity+0x660` 류)은 **ev3 권고**로 표기했다. → §7 브리핑 지적 ①.

---

## 5. `ev<=3` 표본 재확인 (뒤집힌 것 = 0)

| 대상 | 기존 ev | 재확인 방법 | 결과 |
|---|---|---|---|
| `07` `consts[1]` 51 (ev2, 3차 8/8) | 2 | `B5_o4 #G1` 49·50·51·52 | **일치**(경계 50/51). ⚠단 챔프를 분수 밖으로 옮겨야 재현된다 → N4 |
| `08` `consts[5]` 15 (ev2, 3차 13/13) | 2 | `B5_o3 #T` 잔여 899·900·901 | **일치**(경계 900/901) |
| `09` `sig.params[1].ev=2` (version 죽은 인자) | 2 | `B5_o1 I_version` 0·1·2·3·4·5·6 | **일치**(전부 동일 결과) |
| `09` `knobs[10]` ev2 (에픽버프 게이트가 09 에선 무효) | 2 | 이번에 미니언을 안 만들었으므로 **재확인 안 함**(범위 명시) | 판정 유지 |
| `07` `mem` `MapDef+0x6d70` (ev2) | 2 | `B5_o4 #G3` 4경계 | **일치** + 포함성 확정 |
| `08` `mem` `TeamPlan+0x420` (ev3, dienum) | 3 | `tcxdict --enum ObjectPhase` + `#P` | **일치**(None0/Setup1/Assemble2/Hunt3) |
| `05` `history` BigPlan 태그표 (ev3) | 3 | `tcxdict --enum BigPlan` + 실행 16종 | **일치**(ForcePassive2 … DefenseNexus17, DeathMatchBattle untagged) |
| `06` `ret` BattleSubPlanGoal 태그 3/4 | 4 | `tcxdict --enum` | **일치** → ev3 |

**뒤집힌 `ev<=3` 행 0건.**

### 5-b. `open` 3건 · `notes` 1건 처분

| 경로 | 구 | 처분 | 근거 |
|---|---|---|---|
| `/specs[5]/open[0]` RawVec::grow_one 내부 | 미탐색 | **유지** | 재할당 정책은 std 구현이고 판정에 무관. 우선순위 낮음(브리핑 §1 우선순위 기준 = 장식적) |
| `/specs[7]/open[0]` L36 세 OR 항의 소스 순서 | 표기 불가 | **유지(재확인)** | 3경로 재점검: ①`DILocation.column` 전 모듈 0 ②`mir=0 xinl=0` ③**줄 길이 산술은 `a \|\| b` 교환에 불변** ⟹ 「표기 불가」가 맞다. exe 디스어셈도 LLVM 재결합 후 코드라 소스 순서를 담지 않는다 |
| `/specs[9]/open[0]` src_line 원본 대조 | 재료 부재 | **→ 미탐색 (E7)** | rmeta SourceMap 에 `fight_check.rs` 존재(1417줄, 파싱 0 에러) |
| `/specs[9]/notes[0]` 상수 4·2 의 `shl` 접힘 | 사실 서술 | **유지(반증 실패 = 명세가 맞다)** | IR 실측 — `m15.ll:35424 icmp ult i64 %7, 2`(team bounds-check) · `m15.ll:35471 shl i64 %30, 1`(tps×2) · `m15.ll:35776 shl i128 %176, 2`(dot_sq×4) · **`m15.ll:35567 icmp samesign ult i64 %89, 4`**(아군 루프 상한). 노브 4·2 의 리터럴이 판정식 자리에 없고 다른 자리에 있다는 서술이 4/4 정확하다 |

---

## 6. 새 발견 10건

- **N1** `09` **`L1307 else → front_allies += 1` 은 죽은 가지**, 그리고 **`L1286` 의 `9` 는 무효 노브**(같은 뿌리). → E6/R3
- **N2** `09` rear 의 base-거리 타이브레이크(`L1296/1298/1300`)는 **살아 있다**. 경계는
  `to_base_sq < enemy_to_base_sq` **엄격** — `B5_o2.tsv` `#R`: `to_base 25999 → rear(true) / 26000 → flank(false)`.
  단 **도달하려면 적이 자기 분수 근처에 있어야 한다**(적이 맵 중앙이면 아군이 분수 뒤로 못 가서 구성 불가).
  3,000 난수 시나리오에서 `flank_from_rear` 24회 발화.
- **N3** `08` **(d) 경로 첫 도달**. 창 = 「최근 목격 적 중 캠프 최근접 거리 ∈ (150000, 180000]」.
  좁은 이유 = v23 가 180000 안을 요구하고 (d) 가 150000 밖을 요구해 **두 상수가 창을 만든다.**
- **N4** `07` ★**챔피언 기본 스폰 위치가 자기 분수대 안**이다. 그래서 경기 개시 직후에는
  `hp < max` 이기만 하면 L36 의 3번째 OR 항이 참이 돼 **HP% 와 무관하게 Recall** 이 나온다
  (1회차 실측: `hp 51·52` 에서도 Recall). 게임 동작이자 프로브 함정이다 —
  HP% 임계를 재려면 챔프를 분수 밖으로 옮겨야 한다.
- **N5** `05` `BigPlan::get_name` 16/16 실행. 특히 `ForcePassive → "ForcePasive"`(원문 오타) 실행 확인,
  `Battle → "Battle support: None goal: TryKill(0, 0) help_called: false start_tick: 0"`.
- **N6** `check_kill_die_tick` 은 **focus 엔티티가 등록된 챔피언이 아니면** `fight_check.rs:979` 의
  `Option::unwrap()` 에서 패닉한다(합성 id `70000` 으로 5/5 패닉). 프로브 레시피에 넣을 값.
- **N7** 기본 챔피언(`SwordmanChampionInfo::default()`)으로는 `check_kill_die_tick` 이
  `towers`·`enemies` 와 무관하게 **`die = 60`(= `tick_per_second`) 고정**이다(5케이스, focus id 를 모두 달리해
  TLS 메모 충돌 배제). ⟹ **06 은 `die > tps` 가 항상 거짓이라 이 입력에서는 항상 `RunAway`.**
  ★이것은 「towers 무영향」이 **아니라 입력 판별력 부재**다. 탐색 범위 = `AttackEffect`(72B, 전 필드 pub)
  직접 조립(`TEMPLATE.rs` 함정 ④).
- **N8** `09` **독립 재구현 ↔ 실함수 3,000/3,000 MATCH**(true 789 / false 2211), 4차의 1800/1800 과
  **다른 시나리오 생성기**(적 주변 ±400000 균등 + HP 1~100 + 슬롯 0~4). 분기 커버리지
  `front 648 / rear 784 / flank_from_rear 24 / flank 453 / else1307 0`.
- **N9** `08` **「Morgard{Setup} 이면 true」는 시나리오 한정 관측**이었다. 2차 history 의
  `Morgard{Setup, wb=any} → true` 는 그 판에서 `v24=true` 였기 때문이고, 이번에
  `Setup + 적 100000` 에서는 **false** 다(`#D` d=100000). 서술에 조건을 붙여야 한다.
- **N10** `is_ignored_well_enemy(3, player, e)` 는 적 분수 사각형보다 **넓다** —
  `(892000,0)`·`(926000,32000)`·`(960000,64000)` 은 물론 사각형 **밖**인 `(891999, 0)` 도 `true` 이고
  `(480000,480000)`·`(0,0)` 은 `false`. 즉 사각형 판정이 아니라 **우물 좌표 기준 반경/영역 판정**으로 보인다
  (06 은 이 술어를 값으로만 쓰므로 판정 무영향). 정확한 식은 **미탐색**(범위 = `fight_model.rs:754` 본문 독해).

---

## 7. 브리핑(§ `_verify5\BRIEF.md`)에 대한 지적

- **①「`ev4` 를 `ev2` 로 끌어올리는 것이 1순위」는 `mem` 행에 대해서는 부적절하다.**
  `METHOD_MAP §5` 의 신뢰서열은 `tcx > 오라클` 이다. 오프셋·필드명의 정본은 tcx 이므로
  `mem` 행의 목표는 **ev4 → ev3** 이고, ev2 는 「그 필드가 판정에 개입한다」는 **동작 주장**에만 의미가 있다.
  브리핑이 「ev 숫자가 작을수록 좋다」로 읽히게 써 놓아서, 곧이곧대로 하면 `mem` 행을
  **정본(tcx)에서 파생(오라클)으로 강등**하게 된다. §1 의 표에 그 구분을 넣는 게 좋다.
- **② 배치 B 의 ev 분포 수치가 실측과 다르다.** 브리핑 §1 표는 `B = ev2 5 / ev3 19 / ev4 168`(계 192,
  ev≥4 = 87.5%)이라고 적었다. `specs20_v3.json` 의 `specs[5..9]` 에서 `ev` 키를 **전수 집계**하면
  `ev2 6 / ev3 87 / ev4 203`(계 296, ev≥4 = **68.6%**)다. 어떤 부분집합을 셌는지 브리핑에 안 적혀 있어
  「내 델타」를 계산할 기준이 없다. 집계 스코프(어느 필드를 세는가)를 명시하거나 집계 스크립트를 같이 주는 게 좋다.
  (참고로 `ev1`=0, `ev5`=0 은 일치한다.)
- **③ 「`notes[]` 를 파지 마라」와 「틀렸다고 보면 반증하라」가 실무에서 충돌한다.**
  09 의 `notes[0]`(상수 4·2 가 `shl` 로 접혀 다른 위치의 같은 리터럴로 QC 를 통과한다)은 반증 시도 후
  **유지**했는데, 그 판단을 하려면 결국 파야 한다. 「파지 마라」가 아니라 **「조사 대상이 아니라 반증 대상이다」**
  로 써야 의도대로 읽힌다(§2③ 의 표현은 맞는데 §3·§0 의 「파지 마라」가 덮어쓴다).
- **④ 게이트 사각지대 2개**(브리핑이 「G1~G8·G10 = 0 이면 제출 가능」이라고 했지만, 0 인데도 남은 오염이 있었다)
  - `G10` 은 「확정 사실이 `미탐색` 자리에」만 본다. **반대 방향(도달 가능한 것이 `재료 부재` 자리에)** 은 안 본다 → E7.
  - `G5`/`G6`/`G8` 은 `logic` ↔ 표만 대조한다. **`knobs[].what`/`value`/`note` 와 `history[].now` 사이의
    모순·오귀속** 은 아무도 안 본다 → E1·E2·E3(같은 오독이 두 군데, note 가 다른 노브에 붙음).
    `knobs[].note` 가 `knobs[].value`/`what` 과 같은 것을 말하는지 검사하는 게이트(가령 note 안에 나오는
    숫자가 그 knob 의 `value`/`where` 와 무관하면 경고)가 있으면 E3 는 기계로 잡힌다.
- **⑤ 브리핑 §2④ 의 함정 목록에 하나 더 필요하다** — N4(**챔피언 기본 스폰이 자기 분수대 안**).
  `is_in_heal_area`·`well` 계열 판정을 재는 프로브가 전부 이 함정을 밟는다. 나는 실제로 밟았다.

---

## 8. 게이트 실측 출력 (제출 전)

`_spec\*.json` 을 **수정하지 않았으므로 델타는 전부 0** 이다(정정은 위 경로로 보고만 한다).

```
$ cd /c/tfm2mods/MIG
$ PYTHONIOENCODING=utf-8 python -X utf8 specgate.py
   G1 자기모순=0  G10 class 오분류=0  G2 호출부 전수=0  G3 형제 함수=0  G4 술어 시그니처=0
   G5 sig 정본 대조=0  G6 logic 미반영=0  G7 과열림=0  G8 표에 옛 값=0  G9 callees 오염=4
   (G9 4건 = 07 target_bush / 13·14 line·nearest_enemy·position·team / 15 chats — 전부 `손확인` 등급,
    ★강함 0건. 눈으로 확인해 넘김 = 정상)
   ⟹ 제출 조건(G1~G8·G10 = 0) 충족. 5차 착수 시점과 **델타 0**.

$ PYTHONIOENCODING=utf-8 python -X utf8 _spec/audit4.py
   미반영 0 · STALE 0 · ev불일치 0   → 전건 반영됨
   (배치 B 항목 전부 OK: B-N4 09 1800/1800 · B-N6 05 오라클 범위 · B-N7 shared 좌우 ·
    ev상향 08 consts 15 → ev2 · ev상향 07 consts 51 → ev2)

$ PYTHONIOENCODING=utf-8 python -X utf8 tcxaudit.py
   총 551건  오귀속=0  밀림=0  부분일치=0  확인불가=18  OK=533
   ⟹ **불변 게이트(오귀속 0 · 밀림 0) 유지**, 델타 0.
```

`tcxaudit.py --prose _verify5/B/B5_REPORT.md` 결과는 §9 에 붙였다.
이 문서의 오프셋은 **한 행에 하나씩** `타입+0xNNN` 형태로만 적었다(4차 D-E5 교훈).

---

## 9. 산출물 목록

| 파일 | 내용 |
|---|---|
| `B5_o1.rs` / `.tsv` | 09 임계 단독 격리 44케이스 + 1307 도달성 스윕(난수 200만 · 전수 격자) |
| `B5_o2.rs` / `.tsv` | 09 rear 타이브레이크 경계 7케이스 + 재구현 3,000 대조 + 상수 민감도표 |
| `B5_o3.rs` / `.tsv` | 08 (c)(d) 공략 · v24 직접 호출 · min_hp_ratio 확인 · 15×tps 재현 · phase 진리표 |
| `B5_o4.rs` / `.tsv` | 07 3분기 진리표 31케이스(HP%·에픽무손상·회복지역 4경계·은신게이트 3 tps·Hide 페이로드) |
| `B5_o5.rs` / `.tsv` | 05 `BigPlan::get_name` 16/16 + 06 피호출 술어 4종 |
| `B5_o6.rs` / `.tsv` | 05 `LegacyPlanHandler` 크기·초기값 실측 |
| `smoke.rs` | `TEMPLATE.rs` 무결성 확인용 사본(towers 16 · twin 2/2 · is_top_side 8/10) |

빌드 = `sh MIG/_verify3/build.sh <프로브.rs>` → `%TEMP%\tfm2_spanprobe\<이름>.exe`.
