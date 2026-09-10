# 20함수 3차 반증검증 — 공용 브리핑 (2026-09-11 / 게임 0.5.8)

> ⚠**이 문서의 모든 주장은 작성 직전에 실측으로 확인했다.** 2차 브리핑에는 오류 3건이 있었고
> (「pub(crate) 모듈은 오라클 불가」·「D_oracle4.rs 를 복사해 시작해라」·「default 챔피언은 이펙트가 비었다」)
> 그중 하나는 **버그 있는 템플릿을 지목**했다. 그래서 이번엔 근거 파일·실측 출력을 함께 적었다.
> **여기 적힌 것도 틀렸다면 그것 자체를 보고하라** — 그게 이 라운드의 정당한 산출물이다.

---

## 0. 이번 라운드의 목표는 다르다

1·2차는 "정정을 찾는 것"이었다. **3차의 목표는 `오류 0 · 새 발견 0` 을 확인하는 것**이다.
- **오류/누락이 나오면** 그건 성과다 → 정확한 JSON 경로로 보고
- **안 나오면** 그것도 성과다 → "오류 없음"으로 보고. **억지 정정을 만들지 마라**
- 표기·설명이 흔들리는 것(같은 뜻의 다른 문장)은 **문제로 취급하지 않는다.**
  값·구조·시그니처·오프셋·분기 극성이 틀린 것만 오류다

## 1. 정본은 v3 명세 하나다

```
정본 JSON  : C:\tfm2mods\MIG\_spec\specs20_v3.json
읽기용 MD  : mods_report\tfm2_ai_adjust\06_판단함수_명세_v3.md   (JSON 의 투영, 손으로 고치지 말 것)
```

### ★정본 우선순위 (v3 가 새로 명문화한 것)
**오프셋 = `mem` / 상수 = `consts` / 시그니처 = `callees`·`sig.tcx`** 가 정본이고,
**`logic`(의사코드)과 어긋나면 정본이 맞다.**
v2 는 같은 사실을 2~4곳에 중복 보관해서 정정이 한 곳만 반영되는 사고가 반복됐다.

### ★증거 등급 `ev` — 이번 라운드의 판정 기준
| ev | 뜻 | 3차에서 |
|---|---|---|
| 1 | 런타임 실측(game==mine DIFF=0) | 뒤집히면 **사고** |
| 2 | SDK 오라클 실행 | 뒤집히면 **사고** |
| 3 | tcx/MIR/DWARF 정본 | 뒤집히면 **사고** |
| 4 | LLVM IR 독해 | 뒤집히는 것은 정상 |
| 5 | 추론(근거 약함) | 우선 검증 대상 |

⟹ **`ev 5` 부터 치고, `ev 4` 를 그다음, `ev<=3` 은 표본만 확인하라.**

### ★이미 자동 생성된 필드 — 다시 만들지 마라
`callees[].sig` · `callers` · `siblings` 는 tcx·IR 코퍼스에서 **기계 생성**된다(`spec3lib.py`).
1·2차 누락의 3대 원인이 정확히 이 셋이었다. 도구:
```
python -X utf8 spec3lib.py fn    <이름>      # 3크레이트에서 함수 조회(경로·가시성·sp·mir·sig)
python -X utf8 spec3lib.py calls <망글링심볼> # 호출부 전수 (09 는 0.7초에 8곳)
python -X utf8 spec3lib.py def   <망글링심볼> # define 이 어느 코퍼스에 있나
python -X utf8 spec3lib.py sib   <PlanType>  # 형제 진입점 전수
```
⚠`calls`/`def` 에 넘기는 심볼은 **leading underscore 를 그대로** 둬라(IR 은 `@_RNv...`).

## 2. `open[]` 만 남은 일이다

v3 의 `open[]` = **진짜 열린 것**, `closed[]` = 닫힌 것(근거와 함께 보존).
⚠분류 기본값은 **열림**이다 — 초판 자동 판정이 `get_input_target 내부`·`base_sub_goal 내부` 같은
진짜 미탐색까지 닫아서, 명시 근거(`_spec\closelist.py`)가 있는 것만 닫도록 되돌렸다.
⟹ `open` 에 이미 닫힌 게 섞여 있으면 **그것도 보고**하라(과하게 열린 쪽은 안전한 실수다).

판정 어휘 — 범위를 반드시 써라:
- **표기 불가** = 동작 확정, 소스 표기만 외연 동일(예: 같은 길이의 두 표기)
- **재료 부재** = 어디를 뒤졌는지 **범위를 열거**해야 쓸 수 있다
- **미탐색** = 불가능이 아니라 아직 안 읽은 것. **대부분 여기다**

1·2차에서 「원리적 불가」가 **9건 뒤집혔고 원인은 매번 같았다: 가진 재료의 한계를 문제의 한계로 착각.**

## 3. 재료 — 전부 실측으로 확인한 것만

### ① SDK 실행 오라클
**모듈 가시성으로 「불가」를 판정하지 마라.** 모듈이 `in:game_ai` 라도 아이템이 크레이트 루트로
재수출되면 호출된다. 방금 재확인(`spec3lib.py fn`):
```
game_ai::check_favorable_engage_formation   v: pub      (모듈 fight_check 는 in:game_ai)
game_ai::defensive_crisis                   v: pub      (모듈 buff_value  는 in:game_ai)
game_ai::check_kill_die_tick                v: pub
game_ai::effect_cc_time                     v: pub
```
반대로 **진짜 막힌 것도 있다**: `LegacyPlanHandler` 의 `v50_fold_dive_episode`·
`v2_response_retreat_stance` 는 `error[E0624]: method is private` **실측**(`_verify2\B\B_o4_blocked.rs`).
⟹ 판정은 `_tcx\pubapi_game_ai.txt` + tcx `v` 로 하고, 의심되면 **한 번 컴파일해 본다.**

**빌드**: `sh C:\tfm2mods\MIG\_verify2\A\A2_build.sh <내프로브.rs>` → `%TEMP%\tfm2_spanprobe\<이름>.exe`

⚠★**템플릿을 복사할 때 지워야 하는 두 줄**
```rust
game.start_game(&mut rnd, ctx);
game.init_tower(ctx);      // ← 지워라
game.init_nexus(setting, map);  // ← 지워라
```
`start_game()` 이 **이미** 타워·넥서스를 만든다. 재호출하면 전부 2배가 된다 — 방금 갈라서 실측
(`_verify2\towerchk.rs`, 실행 출력):
```
start_game만        tower_ids 16 · twin_towers 2/2 · 팀당 8  · 좌표중복 0
+init_tower/nexus   tower_ids 32 · twin_towers 4/4 · 팀당 16 · 좌표중복 8
```
⚠**2026-09-11 시점의 프로브 전부가 재호출 형태다** — `_verify\D_oracle4.rs`·`_verify2\A\A2_oracle3.rs`·
`_verify2\D\D2_oracle5.rs`·`_verify2\C\C_o11.rs`·`_verify2\B\B_o2.rs`. 구조는 참고하되 그 두 줄은 지워라.
(2차 배치 D 의 「타워 팀당 10개, twin 2쌍 좌표 중복」이 이 오염이었고, v3 에 정정해 뒀다.)

**그 외 확인된 사실**
- `GameSetting::default().tick_per_second == 0`. pub 필드라 `setting.tick_per_second = 60;` 한 줄이면 되고
  **`MapDef::moba()` 호출 이전에** 넣어야 한다.
- `SwordmanChampionInfo::default()` 는 **이펙트가 비어 있지 않다**(`attack_effect` = Some, casting=Targeting).
  0 인 것은 `Effect.range`·`growth_range`·챔피언 `radius` 다. **`radius=10000` 인 타워를 시전자로 주면 값이 나온다.**
- **입력 완전 통제**: `AbstractGameWithCache` 전 필드 pub · `Entity` 전 필드 pub+Clone ·
  `Game::world.entity.get_mut`/`values_mut` · `Blackboard` 필드 pub · `Game::mode.epic_minion_buff_time` pub.
  Entity 를 복제해 x/y/hp 를 바꾸고 `cache.player_champion[t][p] = Some(&내엔티티)` 로 꽂으면 진리표를 짤 수 있다
  (`Vec<Entity>` 를 cache 보다 먼저 선언해야 수명이 맞는다).
- **`&mut Self` 를 받고 `()` 를 돌려주는 함수**는 **구조체 스냅샷 바이트 diff** 로 `writes` 를 기계 대조할 수 있다
  (필드 pub 불필요 — 배치 C 가 `LegacyPlanHandler` 6,168B 로 했다).
- ⚠**오라클과 IR 독해가 어긋나면 IR 독해를 먼저 의심하라.** 2차에서 `*_lead` 산출식이 실행과 6/6
  불일치했고 틀린 쪽은 IR 독해였다.

전문 = v3 `shared.오라클_레시피_함정`.

### ② `_tcx\mirdump_game_core.txt` (403MB · 17,575 바디)
`xinl=1` 인 작은 game_core 헬퍼 술어 본문이 전부 여기 있다. 추출기 `_verify\A_mirget.py`.
★**span 에 칸(column)이 있다** ⟹ game_core 헬퍼는 소스 ±0 복원이 된다.
game_ai 판은 `_tcx\mirdump_game_ai.txt`. ⚠단 `v50_fold_dive_episode`/`v50_track_dive_episode` 는
**거기 없다**(실측 = 재료 부재).

### ③ 패닉 Location 칼럼 — `_verify\B_plocfull.py`
`panicloc.py` 는 칼럼을 버린다. 보정: `bounds_check` 칼럼 = **인덱싱식 시작**,
`.unwrap()` 칼럼 = **메서드 이름 시작**. ⚠한계(실측): 패닉 가능 연산이 없는 줄엔 상수 자체가 없다.

### ④ 줄 길이 산술 — `IR_TOOLKIT.md §8`
`rmeta_srcmap` 줄 길이(**값 − 1** = LF 포함) + `_tcx` `sp`(줄:칸) + tcx 필드명 + IR 분기 유무.
⚠★**들여쓰기는 tcx `sp` 의 `c` 로 고정하라.** `impl` 멤버가 `c=3`(indent 2)이면 **fn 본문은 indent 4** 다 —
2차에서 잔차 2자가 이 보정 하나로 0이 됐다. `A || B` 순서는 길이 불변이라 **원리적으로 못 가른다.**

## 4. 함정 (전부 1·2차에서 실제로 밟은 것)

1. ★**IR 의 `< N` 상수를 소스 임계로 읽지 마라.** `x==0||x==1` → `icmp ult x,2` 는 표준 접힘이다.
   **호출 술어의 tcx `sig` 를 먼저 봐라** — 인자를 받는 술어면 그 상수는 십중팔구 접힘.
   (1차에서 「정글 캠프 티어 컷」 노브가 이렇게 **유령으로 만들어졌다.** 지금은 `callees` 가 자동 생성돼 막힌다.)
   같은 부류: `level > 2` 의 MIR 정본은 `>= 3` 이다.
2. ★**internal 함수의 IR 인자 목록 ≠ 소스 인자 목록**(ArgumentPromotion).
   판별 = tcx `sig` + DWARF `!DILocalVariable(name:…, arg:N)`.
   ➕부수 소득: promotion 이 일어났다는 것 자체가 **"그 인자에서 그 필드만 읽는다"의 증명**이다.
3. ★**IR 의 `i1 true` 가 `bool` 이라는 뜻이 아니다** — 1바이트 열거형의 ABI 표현일 수 있다
   (2차: `is_object_being_taken_by_enemy` 5번 인자가 `WavePriorityObject::Serpen` 이었다).
   같은 부류: **24B 구조체는 Win64 ABI 가 간접전달해 IR 에 포인터로 보인다**(`Chat` 은 값 전달이다).
4. **니치 밀림**(메모리 태그 ≠ 논리 인덱스). ⚠단 **「선언 순서 ≠ 태그」는 실증 사례 0건**이다 —
   1차 브리핑이 그런 함정이 있다고 했는데 **거짓이었다**(LineType/ObjectPhase 둘 다 선언순서=태그).
   `tcxdict --enum` 의 `idx` 와 variant `def_span` 을 믿어라.
5. **`or`/`and` 피연산자 순서로 소스 순서를 추정하지 마라** — rank 정렬로 설명되며 정보량 0.
   단 **`select i1 A, i1 B, i1 false/true` 로 남았으면 단축평가 보존 = 순서 확정.**
6. **인라인 루트 줄 ≠ 선언 줄.** 루트가 선언줄보다 1 큰 것은 그 줄에 클로저가 있기 때문이다
   (`iter_champions` 1904 / `{closure#0}` 1905 · `Position::as_index` 580/581 ·
   `Entity::distance_sq` 2157/2158). **1차가 이걸로 08 을 잘못 지적했다.**
7. **인라인 체인의 `DISubprogram(name:)` 을 봐라** — `first()` vs `get(0)` 은 파일:줄로 못 가른다.
8. **쌍둥이 파일.** `old\epic\hunt_and_battle.rs` ↔ `old\serpen\hunt_and_battle.rs` 는 줄번호까지 같다.
   `hunt_and_poke.rs` 쌍둥이는 **줄번호가 1 밀려 있다.**
   `attack_nexus.rs` 는 `plan_legacy\sub_plan\` 과 `old\` 두 곳에 있다 — `rmeta_srcmap` 조회도 섞인다.
9. **`_gaibc` 에 `declare` 만 있으면 `_gcbc`/`_gvbc` 를 봐라.** 「본문 없음」이 아니다
   (`spec3lib.py def <심볼>` 이 세 코퍼스를 다 훑는다).
10. **형제 진입점을 봐라.** `update` 만 읽으면 `sub_plan`/`next_plan` 이 쓰는 **다른 선택기**를 놓친다
    (2차: 갱커의 부시 선택기가 **셋**이었다). 지금은 `siblings` 가 자동 생성돼 있다.

## 5. 오프셋을 적을 때

⚠**한 행에 오프셋을 묶어 적지 마라.** v3 정리 중 `Blackboard 0x0/0x28/0x50` 한 행이 `tcxaudit` 오귀속으로
잡혔다 — 묶으면 **기계 검사가 무력화된다.** 한 행 = 한 오프셋.

제출 전 필수:
```
python -X utf8 tcxaudit.py --prose <내문서.md>     # 오프셋 전수 재확인 (현재 오귀속 0 · 밀림 0)
python -X utf8 specgate.py                        # 완결 조건 게이트(G1~G4)
```

## 6. 산출물 규칙

- 작업 파일은 **자기 배치 전용 디렉터리에만**: `_verify3\<배치문자>\`
  (`_verify\`·`_verify2\`·`_spec\` 은 **읽기만**.)
- ⛔`_spec\specs20*.json` 을 **직접 고치지 마라** — 패치 목록만 내면 메인이 반영한다.
- ⛔`distruct.json` · `dienum.json` 은 절대 덮어쓰지 마라.
- 보고서 `.md` 작성이 하네스에 막히면(1·2차 모두 막혔다) **본문을 응답에 통째로** 실어라.

### 보고 형식 — 정확한 JSON 경로로
```
/specs[17]/mem[35]/note
  구: "Option<Option<MainObjective>> 3B …"
  신: "Option<MainObjective>(3B) 단일 Option …"
  근거: tcx 정본 + m05.ll:17800
```
경로가 없으면 반영이 안 된다. 표기 = `~~구~~ → 신`(§7 정정형 기록).

## 7. 착수 순서

1. 이 문서 → `MIG\METHOD_MAP.md §0` 라우팅표
2. `06_판단함수_명세_v3.md` 에서 자기 5개를 읽는다 (또는 `specs20_v3.json` 의 `/specs[i]`)
3. 자기 5개의 **`open[]` 을 `ev` 역순(5→4→3)으로** 하나씩 닫으려 시도한다
4. `ev<=3` 항목은 **표본 3~5개만** 재확인한다(뒤집히면 사고니 즉시 보고)
5. `callees_unmatched` 에 **판정에 쓰이는 술어가 섞여 있는지** 확인한다(자동 조회가 놓친 것)
6. 남는 것은 판정 어휘 3종 + **범위**를 붙여 적는다
