# 7차 반증검증 — 배치 B (함수 05~09)

> ⚠**이 파일은 배치 B 가 직접 못 남겼다.** 그 하네스가 보고서 `.md` 작성을 막아서,
> 배치가 최종 응답으로 돌려준 본문을 **메인이 원문 그대로** 여기에 저장했다(2026-09-11).
> 기계 적용분 `patch.json` 과 오라클 산출물은 배치가 직접 디스크에 썼다.

---

# 1. 제출물

| 항목 | 값 |
|---|---|
| `MIG\_verify7\B\patch.json` | **정정 8 · ev상향 6 · 브리핑오류 7 = 21항목** |
| 사전 검증 | `python -X utf8 applypatch.py 7 --only B --dry` → **정정 8/8 · ev상향 6/6 성공, 실패 0** |
| 고친 게이트 | **18/18** (G12 17 + G9 1) — **전부 「게이트 오탐」으로 반전. 명세가 맞았다** |
| 신설 검사기 | `memdircheck.py`(G14) · `kindcheck.py`(G15) · `rolecheck.py`(G16) · `srclinecheck2.py`(G12 교체판) · `irdump.py` |
| 오라클 | `B7_o1~o4.rs/.tsv` (1본은 실패 설계로 기각·보존) |

정정 경로: `/specs[5]/mem[27]/{base,offset,note}` · `/specs[6]/consts[0]/meaning` · `/specs[7]/consts[1]/meaning` · `/specs[7]/consts[5]/meaning` · `/specs[8]/consts[0]/meaning` · `/specs[8]/consts[3]/meaning`
ev상향: `/specs[8]/knobs[3]`,`[5]` · `/specs[9]/consts[0]`,`knobs[5]`,`[8]`,`[9]` (전부 4→2)

§0 해시 확인 실행함 — `dab6d7c803e1be63` / `6320fa42c1107160`, 도시에와 일치.

---

# 2. ★내 지시(도시에)의 오류 7건 — 이것부터

### B-1 ★★§5 의 `patch.json` 스키마가 실제 계약과 다르다
도시에는 `{"round":"r7","entries":[{path,old,new,why}]}` 로 적었다. `applypatch.py`(L325·341)가 읽는 것은 **`errors[]`**(`path`/`kind`/`old`/`new`/`evidence`/`behavior_change`/`found_by`)와 **`ev_up[]`** 다. `entries` 는 어디서도 안 읽힌다. `round` 도 도시에는 문자열 `"r7"`, 계약은 정수 `7`.
⟹ **도시에대로 냈으면 14건이 조용히 전량 무시된다** — 이 라운드가 막으려던 「5차 317행 유실」과 같은 형태.

### B-2 §5 의 `"op": "append"` 는 존재하지 않는다
`applypatch.py` 에 `op` 분기가 없다. **`notes[]` 에 새 항목을 추가하는 경로가 계약에 없다**(기존 문면 치환만).

### B-3 ★★§4-b 의 `consts.kind`(186행)는 명세의 축이 아니라 **파생값**이다
`mkspec3.py:329` 가 `meaning` 문자열 키워드로 만든다. v2 정본의 `constants[]` 행은 `value`/`src_line`/`meaning` **3키뿐**이고 `kind` 키가 없다 ⟹ `/specs[i]/consts[j]/kind` 패치는 **원천 불가**. 고칠 자리는 `meaning` 이거나 파생 규칙.

### B-4 §1 `ev≥4(미실행)` 집계 스코프 미기재
도시에 수치(7/5/6/4/5)는 **`consts`+`knobs` 만** 센 값이다(`open`·`notes`·**`sig.params` 제외**). 담당 5함수의 `sig.params[].ev==4` 만 **26행**. 5차의 같은 사고가 재발.

### B-5 `applypatch.apply_evup` 이 3단 경로를 못 받는다 (도구 결함)
`parse_path` 는 `outer`(`sig`)를 돌려주는데 `apply_evup` 이 그걸 **버리고** `resolve(spec,"params",j)` 를 부른다 ⟹ `/specs[i]/sig/params[j]` ev 상향은 항상 실패. `apply_error` 는 같은 경로를 제대로 처리한다 — **한 파일 안에서 두 함수가 다른 계약**. 그래서 위 26행은 도구를 고치기 전엔 누구도 못 내린다.

### B-6 `mkpatch.locate` 정규식이 3단 경로를 못 받는다
`applypatch.PATH` 는 받는데 참조구현은 `ValueError`. 6차가 `applypatch` 만 고쳤다.

### B-7 ★★ev 증거 부착이 `kind` 파생을 오염시킨다
`apply_evup` 은 근거를 `meaning` **본문에** 덧붙이고, `mkspec3` 은 그 `meaning` 에서 `kind` 를 판다. 실제 사고: **07 `consts[1]`(값 51, `hp_ratio<51` 임계)** 이 5차 증거문 「…**태그** 5=Recall…」 때문에 `kind=태그` 로 뒤집혔다. ⟹ 증거를 별도 키로 빼거나 `mkspec3` 이 `· 오라클 실행 확증(` 이후를 잘라내고 파생해야 한다.

> **판정 반전도 오류로 셈(§5-b 4항): 이번 반전은 17건.** 도시에 §4 가 「명세가 틀렸다」고 지목한 G12 17건이 전부 「게이트가 틀렸다」로 뒤집혔다. 책임은 지시가 아니라 검사기에 있다.

---

# 3. 표적① — 게이트 18건 전수 (실행 명령 그대로)

```bash
cd /c/tfm2mods/MIG
for i in 5 6 7 8 9; do python -X utf8 specgate.py --only $i; done     # 착수: G12 17 + G9 1
cd _verify7/B/oracle
python -X utf8 irdump.py 5 ; python -X utf8 irdump.py 5 --line 146 ; python -X utf8 irdump.py 5 --val 9
python -X utf8 srclinecheck2.py 5 6 7 8 9     # 40행 · 불일치 0 · 접힘 보류 3
python -X utf8 srclinecheck2.py               # 전 20함수 186행 · 불일치 4 · 접힘 보류 19
```

현행 G12 의 전제(「리터럴과 `!dbg` 가 같은 물리 줄」)가 깨지는 자리 **5가지**를 특정했고, 담당 18건은 전부 그중 하나였다.

| # | 오탐 기제 | 실례 | 건수 |
|---|---|---|---|
| ① | **`invoke` 는 두 줄** — 리터럴은 첫 줄, `!dbg` 는 `to label …` 연속행 | `%18 = invoke …starts_with(…, i64 noundef 11)`(m13.ll:29009) / `!dbg !35999`(29010) | 05 `[11][12][13]` 3 |
| ② | **`phi` 의 상수는 들어온 블록 소속** (phi 의 `!dbg` 는 하나, 상수는 N개) | `%74 = phi i8 [1,%33],[2,%36],[3,%41],[4,%46],[7,%66],[5,%51],[6,%61],[6,%56]`(29153) | 05 6 · 06 2 · 07 2 |
| ③ | **`line:0`(LLVM 병합 위치)** — CSE 되면 위치가 0. 진짜 자리는 소비자 | `%184 = mul i128 %175, 9, !dbg !46436` → `line: 0`, 소비자 `icmp`(1283) | 09 `[4]` 1 |
| ④ | **상수 접힘** — `×2→shl 1`, `×4→shl 2`, enum 매핑→태그비교 | 08 L164 `take_active(Morgard)` → `icmp eq i8 (TeamPlan+0x41f), 0` | 08 `[0]` · 09 `[0][5]` 3 |
| ⑤ | **SSA·메타데이터 번호를 리터럴로 셈**(`%9`,`%11`,`!12115`) | 07 태그 9·11 의 「실제 후보」가 전부 이 잡음 | 07 2건에 중첩 |

⚠**②의 세부 — 배치 A 가 동시에 같은 축을 고쳤으나(런 중 `srclinecheck.py` 갱신됨) 그 판으로는 두 곳이 안 풀린다.** A 판은 선행 블록의 **종결자** `!dbg` 를 쓰는데, 07 `%115` 의 종결자는 `!DILexicalBlockFile(file:!14867)` 을 물어 **line:1·다른 파일**이고 진짜 자리(43)는 블록 **본문**(`!54544`)에 있다. 06 `consts[3]`(RunAway=4)의 `return` 은 선행 블록이 아니라 **합류 블록**(%127/%130)에 접혀 있다. ⟹ 내 판은 **선행 블록 본문 ∪ phi 자신의 블록 본문**을 쓴다. 병합 시 ①③④와 함께 가져가야 한다.

★**「접힘 판정보류」를 새 등급으로 둔 이유**: 접힌 상수는 IR 에 리터럴이 **없는 것이 정상**이다(09 `notes[0]` 가 이미 그렇게 적고 있었다). 불일치로 세면 명세가 옳은데도 붉은 줄이 남고, 손으로 넘기기 시작하면 진짜 결함도 같이 넘어간다(`SPEC_RUNBOOK §S5-c` 자기 규칙). 기계 기준 = 「주장한 줄이 그 함수가 실제로 참조하는 줄 집합 안에 있는가」.

**G9 `07 target_bush` = 오탐.** tcx 조회 4건 중 동명 `AssocFn` 은 `LineGankerPlan`·`LineGankCoverPlan` 의 것이고, 07 의 self 타입 `EpicHuntAndBattlePlan` 에서는 **Field**(hunt_and_battle.rs:10, `mem[20]/[21]` 과 정합). ⟹ **G9 개선안: 동명 함수가 다른 Self 타입이면 후보에서 빼라** — 이 부류가 기계로 닫힌다.

---

# 4. 표적② — 무검사 축 3개

### 4-1 `mem.dir` (451행) → `memdircheck.py` = **G14 제안**
오프셋별 gep SSA 이름(배열 gep 은 체인 전파) → `load`/`store`/`addr` 관측. `dir=r`는 `load|addr`, `dir=w`는 `store` 로 충족.
```bash
python -X utf8 memdircheck.py 5 6 7 8 9   # 106행 · 불일치 1 · 판정보류 2
python -X utf8 memdircheck.py             # 449행 · 불일치 5 · 판정보류 57
```
★**담당 실오류 1건 — `05 mem[27]`**: `LegacyPlanHandler+0x890 · dir=w` 인데 **그 오프셋에 store 가 0건**이다. 실제는
```
m13.ll:29325  %128 = gep i8, ptr %0, i64 2192   ; +0x890
       29327  %129 = load ptr, ptr %128         ; 힙 버퍼
       29329  %130 = gep {104B}, ptr %129, i64 %123
       29331~ store i64 %95, ptr %130 …         ; 22필드 전부 %130+k
```
같은 오프셋에 `mem[23](r,".ptr")`/`mem[27](w)` 두 줄이 나란해 **재구현자가 핸들러 필드에 쓴다고 읽을 수 있었다.** → `base`=`V50DiveEpisode(힙 원소)`, `offset`=`0x0` 으로 정정.

**타 배치 몫 4건(손대지 않음)**: `specs[4] mem[26] BrainMinionParameter+0x20 r인데 store만` / `specs[14] mem[13]·[24] LegacyPlanHandler+0x860 w인데 load만` / `specs[15] mem[15] TeamPlan+0xc8 w인데 load만`. 뒤 셋은 05 와 **같은 「Vec 힙 원소를 소유 구조체 오프셋으로 적는」 형태**로 보인다.
⚠한계: **필요조건 검사**다(같은 오프셋을 다른 구조체가 쓰면 통과). 잡으려는 건 **r↔w 뒤집힘**이고 그건 잡힌다.

### 4-2 `consts.kind` (186행) → 축이 아니라 파생값 (B-3). 그래도 `kindcheck.py` = **G15 제안**
소비 오프코드로 분류(`icmp`→임계 / `mul·shl·udiv`→계수 / `gep`→인덱스 / `phi·select·store`→태그 / `call`인자→인자상수). 담당 40행 중 **26행이 관측과 어긋남**.
★진단 = **어휘 부족**. 현행 4종 `{센티널,인덱스,임계,태그}` 에 **계수**(`hp*100`, `tps*2`, `×15`, 셀크기 32000)와 **길이**(`"PassiveLine"`=11, `"PassiveJungle"`=13, `"ActiveRecall"`=12)가 없고, 캐스케이드 **폴스루가 `임계`** 라 둘이 전부 임계로 쓸려 있다.
⟹ 제안: ①어휘 6종 확장 ②IR 오프코드 관측 우선 파생 ③파생 입력에서 **증거 꼬리 절단**(B-7).
확정 오류 4건은 `meaning` 으로 패치: 06`[0]`(임계인데 인덱스) · 07`[1]`(임계인데 태그, B-7 오염) · 07`[5]`(태그인데 임계) · 08`[3]`(**G1 자기모순** — `meaning` 은 「임계 아님」인데 kind 는 「임계」).

### 4-3 `sig.params.role` (124행) → `rolecheck.py` = **G16 제안**
① **인자 개수 불변식** `IR define 인자 수 == spec 인자 수 − 소거주장 수` ② **속성 주장 대조**(`readnone`/`readonly`/`captures(none)` 이 `define` 에 실제로 있나).
**담당 5함수 = 0건**(05 IR3=5−2 ✓ / 06 IR5=6−1 ✓ / 07 IR8 ✓ / 08 IR7 ✓ + `_rnd`(arg2)·`_debug`(arg6) 실제 `readnone` ✓ / 09 IR5 ✓).
**타 배치 5건**: `specs[1] 8vs7` `specs[2] 8vs7` `specs[13] 6vs3` `specs[17] 5vs4` `specs[19] 8vs5`. 앞 넷은 **sret out-ptr 을 `params` 에 안 실은 것**으로 보인다(07 은 `p0 (sret)` 로 싣는다 ⟹ **규약이 명세마다 다르다**).
⚠초판 오탐 1건을 자체 적발: 09 `version` role 의 「**피호출자** 2단이 `i64 poison`」을 「이 함수 인자 소거」로 오독 → `피호출자` 포함 role 은 제외하도록 수정.

---

# 5. 표적③ — `ev≥4` 내리기 (오라클 4본, 빌드 1본 ≈35초)

**`B7_o1` — 08 `MainObjective` 12 variant 전수, 46/46 MATCH.** 2·3차는 4개만 돌렸다. 태그 0(Morgard)만 본문을 계속 타고 **1~11 과 None 은 전부 즉시 true** ⟹ `08 knobs[3]`(캠프 상수 4 고정)·`knobs[5]`(매핑표의 「그 외 None」이 8 variant 에 대해 최초 실행 확인) **4→2**.
덤(새 사실): `camp_pos(Morgard,blue)==camp_pos(Morgard,red)==(288000,288000)` — **Morgard 는 blue-side 인자가 결과를 안 바꾼다**. `Serpen`=(672000,672000). version 10종 전부 동일.

**`B7_o2` 는 실패 설계 — 남긴다.** 미니언 18마리를 세우고 tps 를 쓸었는데 **09 가 전 구간 false** 라 K=1·2·3·4 가 전부 「가능」(판별력 0). 원인 = 대형이 어차피 불리라 미니언 게이트가 결과를 못 바꾼다. ⟹ **유리 대형을 먼저 만들어야 게이트가 관측창이 된다.**

**`B7_o3` — 09 의 창 계수가 `tps×2` 임을 실행으로 고정.** 적 분수 중심 `(926000,32000)` 방향 200000 지점에 rear 아군 1명을 세워 09 가 기본 `true` 가 되게 한 뒤 tps 20점 스윕:
```
tps 40 → r09=true   (2×40=80 < danger 창 임계 81)
tps 41 → r09=false  (2×41=82 ≥ 81)          ← 반전점
K=1 기각(14/6) · **K=2 가능(20/0)** · K=3 기각(17/3) · K=4 기각(16/4)
```
⟹ `09 consts[0]`·`knobs[5]`·`knobs[8]` **4→2**. `shl i64 %30,1` 로 접혀 리터럴이 없던 계수를 실행으로 확정.

**`B7_o4` — 플래그 판별.** 창을 `tps×2` 로 고정하고 `(champion_action, predict_retarget)` 4조합 대조: **(true,false) 만 20/20 일치**, 나머지 3조합 전부 기각 ⟹ `09 knobs[9]` **4→2**. (`.tsv` 열 이름은 o3 것을 물려받아 파일 끝 `#LEGEND` 줄에 매핑을 적어 뒀다.)

### 못 내린 것 — 판정 어휘 + 범위
- `05 consts[4][5][6][7]`(end_plan 3/4/5/6) = **미탐색**. 상위 래퍼 `LegacyPlanHandler::update` 경유로는 **도달 불가** — `update` 가 매 틱 재플래닝해 빈 10인 세계에서는 코드 1·2·7·8·9 만 나온다(`B6_o2.tsv #B`·`#G` 실측). **미탐색 = AI 가 그 플랜을 실제로 고르는 세계 만들기**(장기 `run_tick` + 에픽/부시/갱 주입). 「불가」 아님.
- `05 consts[1]`·`knobs[0]`(end_reason 7) = **미탐색**. `end_reason` 은 호출부(`v50_track_dive_episode` 클로저)가 정한다(`B6_o2.tsv #A`=1). 주입 창구 없음.
- `05 consts[13]`(길이 12) = **표기 불가(부분)**. 값·줄은 확정(m13.ll:29103), 접두 길이는 외연이 같아 실행에 안 드러난다.
- `06` 전 행 = **재료 부재(직접 호출)**. `in:game_ai` + `update` 경유로는 06 의 반환인지 못 가른다. **미탐색 = `AttackEffect` 조립으로 `die > tps` 성립시켜 KitingBack 유도**.
- `07 knobs[8]~[13]`, `08 knobs[4]`, `09 knobs[7]` = **미탐색**(담당 범위 밖). ⚠`09 knobs[7]` 은 4차 배치B 가 이미 임계표를 복원했으니 **근거를 옮기기만 하면 된다**(범위를 지켜 손대지 않음).
- `sig.params[].ev4` **26행** = **도구 결함으로 차단**(B-5).

---

# 6. 다음 라운드에 물을 축 (S5-c 갱신 제안)

`mem.offset` **표기 규약**(`0x860[len]` 같은 값이 섞여 파서가 깨짐) · `sig.params` 에 **sret 을 싣는가**(규약이 명세마다 다름 = rolecheck 5건의 실체) · `knobs.value`(G13 은 `where` 만 본다. `08 knobs[4]` 는 값이 자리표시 `0`) · `callees` **동명 다중**(Self 타입 대조로 기계화 가능) · `logic` ↔ `mem`/`consts` **상호참조**(「logic 이 인용한 오프셋이 mem 표에 있나」는 무검사).

⚠승격 시 각 도구의 첫 docstring 줄을 쓰고 `python -X utf8 mktools.py` 를 돌려야 한다(`METHOD_MAP §0`).
