# 20함수 4차 반증검증 — 공용 브리핑 (2026-09-11 / 게임 0.5.8)

> ⚠**3차 브리핑에는 오류 3건이 있었고, 세 건 다 「내가 범위를 과잉 단정한 것」이었다.**
> (「05 end_reason 의미는 exe 디스어셈만 남음」→호출자 IR에 전량 있었다 /
>  「07 L36 **두** OR 항」→세 항 /
>  「16 은 재료 부재(전 범위), 다시 파지 마라」→줄 길이 산술로 뚫렸다)
> **⟹ 이번 브리핑은 「무엇이 불가능한지」를 말하지 않는다.** 재탐색 여부는 v3 의 `class`/`ev` 와
> 네 판단으로 정하라. 아래 사실 주장은 전부 작성 직전에 실측했고 근거를 붙였다 —
> **그래도 틀린 게 있으면 그것 자체를 보고하라.** 그게 이 라운드의 정당한 산출물이다.

---

## 0. 목표 (3차와 동일)

**`오류 0 · 새 발견 0` 을 확인하는 것.**
- 오류/누락이 나오면 성과 → 정확한 JSON 경로로 보고
- **안 나오면 그것도 성과** → "오류 없음". **억지 정정을 만들지 마라.**
- **표기·설명이 흔들리는 것은 문제로 취급하지 않는다.** 값·구조·시그니처·오프셋·분기 극성이
  틀린 것만 오류다.

## 1. 3차 이후 달라진 것 (이걸 먼저 알아야 중복 작업을 안 한다)

### ① 3차 정정이 전부 반영됐다
실오류 19건 + 과열림 13건이 v3 에 들어갔다. `open 101 → 63` · `closed 66 → 104`.
**3차 리포트에서 이미 지적된 것을 다시 찾지 마라** — 자기 배치의 3차 원문을 먼저 읽어라:
`REPORT\tfm2_ai_adjust\RE\2026-09-11_20함수-3차반증검증-배치{A,B,C,D}.md`

### ② 3차의 잔존 오염원(`logic` 산문)에 기계 게이트가 걸렸다
3차 실오류의 지배적 부류는 **표/`history` 의 정정이 `logic` 의사코드에 안 번지는 것**이었다
(배치 D 5건 중 3건 · A E1/E2 · C E8 · B E2). 이제 두 게이트가 막는다:
- **G6** — 표가 `~~구~~` 로 정정한 옛 값이 `logic` 에 살아 있으면 적발
- **G5** — `sig.tcx` 인자 목록 ↔ `params[].type` 가변성을 **위치별로** 대조
  (알려진 양성 1건 = 3차 배치B 의 `08 &mut TeamPlan` 을 정확히 잡고 오탐 0 확인)

⟹ **G5/G6 가 놓치는 같은 부류가 남아 있으면 그것을 찾아라.** 게이트의 사각지대가 이 라운드의 표적이다.

### ③ 내 도구 결함 2건이 고쳐졌다
- `siblings` 망글링 파싱이 **20개 전부 실패**하던 것 → **12/20 정상 검출**(나머지 8은 실제로 자유 함수).
  순차 워커·`re.finditer` 둘 다 안 되고 위치별 독립 스캔이어야 한다.
- `specgate` G3 가 `plan == null` 을 무조건 통과시키던 구멍 → 자유함수 화이트리스트 도입.
- `mem.chk` 가 **449행 전부 "조회실패"** 였던 것(`norm_base` 가 `tcxdict` 아니라 `tcxaudit` 소속이라
  AttributeError) → 이제 `tcxaudit` **실제 판정**이 박힌다: `OK 431 · 확인불가 18(전부 vtable) · 조회실패 0`.

### ④ ★오라클 프로브 정본 템플릿이 생겼다 — **이걸 복사해서 시작하라**
```
템플릿 : C:\tfm2mods\MIG\_verify3\TEMPLATE.rs
빌드   : sh C:\tfm2mods\MIG\_verify3\build.sh <내프로브.rs>   → %TEMP%\tfm2_spanprobe\<이름>.exe
```
방금 재실행해 확인한 출력(이 네 줄이 안 나오면 네 프로브가 오염된 것이다):
```
setting_ok	true	width=960000	height=960000	tps=60	champ_radius=10000	visible=130000
towers	16	twin0=2	twin1=2
is_top_side_true	8/10	(height=0 이면 10/10 로 붕괴한다)
expect	towers=16 twin=2/2 is_top_side_true=8/10
```
템플릿이 해결한 두 함정:
- **`GameSetting::default()` 는 숫자 필드 44개가 0 이다.** `height==0` → `height − y` 가 u64
  **언더플로** → `is_top_side` 가 상수 true. 2차의 「챔프 10명 전부 탑사이드 = 재료 부재」와
  「default 챔피언은 이펙트가 비었다」가 **둘 다 이 오진**이었다. 템플릿은 실전 44줄을 대입한다
  (원천 = `<게임설치>\bundle_unpacked_full\setting\game_setting.game_setting`, 44줄 전부 rustc 통과).
  ⚠`serde_json::from_str` 로 직접 읽을 수는 없다(SDK deps 에 serde rlib 이 둘이라 E0277, 실측).
- **`start_game()` 뒤 `init_tower()`/`init_nexus()` 재호출은 타워를 2배로 만든다**
  (실측 16→32 · twin 2→4 · 팀당 8→16 · 좌표중복 0→8, `_verify2\towerchk.rs`).
  템플릿은 부르지 않는다. ⚠`_verify\D_oracle4.rs`·`_verify2\{A,B,C,D}\*` 의 기존 프로브는 **전부**
  재호출 형태다 — 구조는 참고하되 그 두 줄은 지워라.

### ⑤ ★실전 챔피언 데이터가 열렸다 (3차 배치A 발굴, 방금 재확인)
`_tcx\game_core.json` 실측: **`pub` ChampionInfo 구조체 61종 · `pub effect()` 193건.**
⟹ `Game` 없이 실전 챔피언 이펙트 진리표를 짤 수 있다.
⚠단 `::default()` 는 스탯 0(range 0) · `DataChampionInfo` 만 `Default` 없음 · 5슬롯은 0除算 패닉.
**"이펙트가 없어서 판별력이 없다"는 이제 성립하지 않는다** — 실전 챔피언을 꽂아라.

### ⑥ 오라클 함정 2건 추가 (3차 배치A)
- `GameSetting::default().minion_wave_setting` 이 전부 0이라 `run_tick` 1,200틱에도 **미니언 0마리**
  (손으로 넣으면 6틱에 팀당 6마리).
- `world.strategy[team]` 직접 대입은 `PlayerState::strategy()` 에 **반영되지 않는다** →
  `AbstractGame::set_strategy(team, Strategy)` 를 써라.

## 2. 정본과 읽는 순서

```
정본 JSON : C:\tfm2mods\MIG\_spec\specs20_v3.json
읽기용 MD : mods_report\tfm2_ai_adjust\06_판단함수_명세_v3.md   (JSON 의 투영. 손으로 고치지 말 것)
```

**정본 우선순위** — 오프셋=`mem` / 상수=`consts` / 시그니처=`callees`·`sig.tcx` **> `logic`(의사코드)**.
어긋나면 앞의 것이 맞다.

**증거 등급 `ev`** — 1 런타임실측 / 2 오라클 실행 / 3 tcx·MIR·DWARF / 4 IR 독해 / 5 추론.
**`ev<=3` 이 뒤집히면 사고**(즉시 보고) · **`ev>=4` 가 뒤집히는 것은 정상 수렴.**

**자동 생성 필드는 다시 만들지 마라** — `callees[].sig` · `callers` · `siblings` · `mem[].chk`.
```
python -X utf8 spec3lib.py fn    <이름>        # 3크레이트 함수 조회(경로·가시성·sp·mir·sig)
python -X utf8 spec3lib.py calls <망글링심볼>   # 호출부 전수(09 는 0.7초에 8곳)
python -X utf8 spec3lib.py def   <망글링심볼>   # define 이 어느 코퍼스에 있나(gai/gc/gv 전부 훑는다)
python -X utf8 spec3lib.py sib   <타입명>      # 형제 진입점 전수
```
⚠`calls`/`def` 에 넘기는 심볼은 **leading underscore 를 그대로** 둬라(IR 은 `@_RNv…`).

## 3. `open[]` — 이번 라운드의 표적 (총 63건)

**우선순위: `ev` 역순(5→4→3).** `ev<=3` 은 표본 3~5개만 재확인하고 뒤집히면 즉시 보고.

⚠**범위 판정은 네가 하라.** 3차에서 내가 「재료 부재」로 못박은 것이 뚫렸으므로, 이번엔 지정하지 않는다.
v3 의 `class`(미탐색 / 재료 부재 / 표기 불가)와 `open[].q` 본문의 범위 서술을 읽고 판단하되,
**「불가」를 쓰기 전에 `MIG\METHOD_MAP.md §0` 의 재료 목록을 한 번 훑어라.**
1·2·3차에서 「원리적 불가」가 **10건** 뒤집혔고 원인은 매번 같았다:
**가진 재료의 한계를 문제의 한계로 착각.**

## 4. 함정 (전부 1~3차에서 실제로 밟은 것)

1. ★**IR 의 `< N` 상수를 소스 임계로 읽지 마라.** `x==0||x==1` → `icmp ult x,2` 는 표준 접힘.
   **호출 술어의 tcx `sig` 를 먼저 봐라**(이제 `callees` 에 자동으로 들어 있다).
   같은 부류: `level > 2` 의 MIR 정본은 `>= 3` 이다.
2. ★**internal 함수의 IR 인자 목록 ≠ 소스 인자 목록**(ArgumentPromotion). 판별 = tcx `sig` +
   DWARF `!DILocalVariable(name:…, arg:N)`. 부수 소득: promotion 이 일어났다는 것 자체가
   **"그 인자에서 그 필드만 읽는다"의 증명**이다.
3. ★**IR 의 `i1 true` 가 `bool` 이라는 뜻이 아니다** — 1바이트 열거형의 ABI 표현일 수 있다
   (`WavePriorityObject::Serpen`). **24B 구조체는 Win64 가 간접전달**해 IR 에 포인터로 보인다
   (`Chat`·`BattlePlanGoal`·`Strategy` 는 값 전달).
4. ★**variant 가 튜플인지 struct 인지 확인하라** — `EntityType::Tower` 는 `{ info: Tower }` **struct
   variant** 이고 `BattleSubPlanGoal` 도 `{ focus }` 다. `Tower(tt)` 로 쓰면 컴파일도 안 된다.
5. **니치 밀림**(메모리 태그 ≠ 논리 인덱스). ⚠단 **「선언 순서 ≠ 태그」는 실증 사례 0건**이다 —
   1차 브리핑이 그런 함정이 있다고 했는데 거짓이었다. `tcxdict --enum` 의 `idx` 와 variant `def_span` 을 믿어라.
6. **`or`/`and` 피연산자 순서로 소스 순서를 추정하지 마라** — rank 정렬로 설명되며 정보량 0.
   단 **`select i1 A, i1 B, i1 false/true` 로 남았으면 단축평가 보존 = 순서 확정.**
7. **인라인 루트 줄 ≠ 선언 줄.** 루트가 선언줄보다 1 큰 것은 그 줄에 클로저가 있기 때문이다
   (`iter_champions` 1904 / `{closure#0}` 1905 · `Position::as_index` 580/581 ·
   `Entity::distance_sq` 2157/2158). 1차가 이걸로 08 을 잘못 지적했다.
8. **인라인 체인의 `DISubprogram(name:)` 을 봐라** — `first()` vs `get(0)` 은 파일:줄로 못 가른다.
9. **쌍둥이 파일.** `old\epic\hunt_and_battle.rs` ↔ `old\serpen\…` 은 줄번호까지 같다.
   `hunt_and_poke.rs` 쌍둥이는 **줄번호가 1 밀려 있다.** `attack_nexus.rs` 는
   `plan_legacy\sub_plan\` 과 `old\` 두 곳에 있어 `rmeta_srcmap` 조회도 섞인다.
   `single_tower_dive_is_viable` 은 `single_battle::`(pub) 과 `death_battle::`(private) 둘이다.
10. **`_gaibc` 에 `declare` 만 있으면 `_gcbc`/`_gvbc` 를 봐라**(`spec3lib.py def` 가 셋을 다 훑는다).
11. **형제 진입점을 봐라** — `siblings` 에 자동으로 들어 있다(12/20 검출).
12. ★**줄 길이 산술의 들여쓰기는 tcx `sp` 의 `c` 로 고정하라.** `impl` 멤버가 `c=3`(indent 2)이면
    **fn 본문은 indent 4** 다. `A || B` 순서는 길이 불변이라 **원리적으로 못 가른다.**
13. ⚠**오라클과 IR 독해가 어긋나면 IR 독해를 먼저 의심하라.** 2차 `*_lead`(6/6 불일치) ·
    3차 배치C `region_point`(store 0건 오판) 둘 다 IR 독해가 틀렸다.
    **`awk`/`sed` 로 IR 구간을 자를 때 반드시 다음 `define` 까지 잡아라.**

## 5. 오프셋을 적을 때 / 제출 게이트

⚠**`tcxaudit --prose` 는 `Type+0xNNN` 형태만 스캔한다.** 표 형식으로만 적으면 **한 건도 스캔되지 않고
감사가 무의미해진다**(3차 배치D 실측 — 총계가 안 변하는 것으로 겨우 발각). 최소 한 번은 그 형태로 쓸 것.
⚠**한 행에 오프셋을 묶지 마라**(`0x18/0x20` 이 오귀속으로 잡혔다). 한 행 = 한 오프셋.

```
python -X utf8 tcxaudit.py --prose <내문서.md>     # ★기준선을 먼저 재라(아래 참조)
python -X utf8 specgate.py                        # 완결 조건 G1~G6
```
**현재 기준선(방금 실측)**: `specgate` **G1~G6 전부 0** / `tcxaudit --prose` 는 인자 없이 돌리면
`_spec` 만 봐서 **766건 · 오귀속 0 · 밀림 0 · 부분일치 2 · 확인불가 18**.
⟹ **부분일치 2·확인불가 18 은 이미 있는 것이다**(`cache+0x8` 팻포인터 · vtable 슬롯). 네 탓이 아니다.
⚠`--prose` 에 없는 경로를 주면 `[skip]` 만 찍히고 "깨끗함"으로 오독된다 — 총계가 늘었는지 확인하라.

## 6. 산출물 규칙

- 작업 파일은 **`_verify4\<배치문자>\` 안에만.** `_verify\`·`_verify2\`·`_verify3\`·`_spec\` 은 **읽기만.**
- ⛔`_spec\specs20*.json` 직접 수정 금지 — 패치 목록만 내라. ⛔`distruct.json`·`dienum.json` 덮어쓰기 금지.
- 보고서 `.md` 쓰기가 하네스에 막히면(1~3차 모두 막혔다) **본문을 응답에 통째로** 실어라.
- 판정에는 **반드시 적용 범위**를 붙여라.

### 보고 형식 — 정확한 JSON 경로
```
/specs[8]/sig/params[6]/type
  구: "&mut TeamPlan(1064B)"
  신: "&TeamPlan(1064B)"
  근거: tcx 정본 sig 에 mut 없음 + IR m10.ll:7655 의 %5 에 store 0건
```
⚠**v3 는 생성물이다.** 패치 경로는 v3 기준으로 쓰되, 그 값이 v2(`specs20.json`)의 어느 필드에서
오는지 알면 함께 적어라(`logic`/`reads`/`writes`/`constants`/`knobs`/`resolved`).
`open`/`closed` 이동은 `_spec\closelist.py` 소관이니 **"closed 로 이동 + 근거"** 로만 쓰면 된다.

## 7. 착수 순서

1. 이 문서 → `MIG\METHOD_MAP.md §0` 라우팅표
2. **자기 배치의 3차 원문**(`RE\…3차반증검증-배치X.md`)을 읽어 중복을 피한다
3. `06_판단함수_명세_v3.md` 또는 `specs20_v3.json` 의 `/specs[i]` 로 자기 5개를 읽는다
4. `open[]` 을 `ev` 역순으로 닫으려 시도한다 (§1⑤ 실전 챔피언 데이터가 새 지렛대다)
5. `ev<=3` 표본 3~5개 재확인
6. **G5/G6 가 놓치는 `logic` ↔ 정본 불일치가 남았는지** 직접 눈으로 대조한다(§1②)
7. `callees_unmatched` 에 판정 술어가 섞였는지 확인
