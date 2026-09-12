> ⚠**이 파일은 배치 C 가 직접 못 남겼다.** 그 하네스가 보고서 `.md` 작성을 막아서,
> 배치가 최종 응답으로 돌려준 본문을 **메인이 원문 그대로** 옮겼다(2026-09-11).
> `patch.json` 과 보조 산출물(`mkpatch_c.py`·`verify_targets.py`)은 배치가 직접 디스크에 썼다.

---

# 10차 배치 C — 전수 감사 보고 (함수 `10`~`14`, 게임 0.5.8)

**제출물**: `patch.json` — 정정 **15** · ev상향 **2** · `behavior_change` **1** · `brief_errors` **9**.
`applypatch.py 10 --only C --dry` = **정정 15/15 · ev상향 2/2 · 실패 0**. `dossierfresh.py 10 C` = 착수·제출 직전 둘 다 **FRESH**.

보조 산출물: `mkpatch_c.py`(`old` 문면을 정본에서 직접 추출해 오타 차단) · `verify_targets.py`(정본 **사본**에 적용해 «그 행이 맞는지» 확인 — 이게 아니었으면 `ev_up` 오조준을 못 잡았다).

## 0. 실제로 실행한 것 (명령줄 그대로)

```bash
cd /c/tfm2mods/MIG
python -X utf8 dossierfresh.py 10 C                       # 착수 FRESH · 제출 직전 재확인 FRESH
PYTHONIOENCODING=utf-8 python -X utf8 specgate.py         # 기준선: 19게이트 전부 0 (G9 참고 1건)

# IR 원문 — 담당 5함수 전 범위 + 콜리
sed -n '49611,50100p' /c/tfm2mods/_gaibc/m10.ll           # 10 본체
sed -n '49498,49610p' /c/tfm2mods/_gaibc/m10.ll           # 10 objective_entity_id_for_main_objective(아웃오브라인)
awk 'NR>=47454 && NR<=47625' /c/tfm2mods/_gaibc/m10.ll    # 10 can_enemy_hit_objective
awk 'NR>=12238 && NR<=12479' /c/tfm2mods/_gaibc/m13.ll    # 11 본체
awk 'NR>=157000 && NR<=157060' /c/tfm2mods/_gcbc/g07.ll   # Blackboard::is_recent_visible (120틱)
awk 'NR>=29383 && NR<=29689' /c/tfm2mods/_gaibc/m13.ll    # 12 본체
awk 'NR>=53199 && NR<=53760' /c/tfm2mods/_gaibc/m13.ll    # 12 rule_scope::chat_allowed 전문
awk 'NR>=11483 && NR<=11731' /c/tfm2mods/_gaibc/m10.ll    # 13 본체
awk 'NR>=94569 && NR<=94941' /c/tfm2mods/_gaibc/m08.ll    # 14 본체

# !dbg 사슬(inlinedAt 루트까지)
python -X utf8 dloc.py m10.ll 55333 55359 55339 55515 55433 55396 55413 55431 55376 55385 55386 55306 55430 55321 55533 55534
python -X utf8 dloc.py m13.ll 21933 21934 21935 21937 21938 21940 21946 21961 21963 21967 21980 21981 21982 21983 21986 21990 21993 21996 21999 22003
python -X utf8 dloc.py m13.ll 56858 56692 56897 56737 56861 56872 56883 56894 56904 56927 56637
python -X utf8 dloc.py m13.ll 36342 36344 36346 36347 36380 36386 36391 36392 36329 36428 36458
python -X utf8 dloc.py m10.ll 23235 23236 23237 23190 23191 23192 23218 23219 23209 23231 23232 23233 23180 23095 23129
python -X utf8 dloc.py m08.ll 55714 55724 55725 55741 55742 55743 55697 55698 55699 55738 55739 55740 55745 55753 55849 55882 55571 55574 55683 55613
python -X utf8 dloc.py m10.ll 53163 53175 53199                # ★19600000000 의 진짜 줄

# tcx 정본
python -X utf8 tcxdict.py Entity 0x5c8 / 0x500 / 0x538
python -X utf8 tcxdict.py LineGankerPlan  /  LineGankerPlan 0x0 · 0x8 · 0x10
python -X utf8 tcxdict.py LegacyPlanHandler 0x1802 · 0x180a · 0x530
python -X utf8 tcxdict.py AbstractGameWithCache 0x21c0 · 0x21d0 · 0x21e0
python -X utf8 tcxdict.py --enum TutorialType                  # variant 9개 = 외연 동일성의 근거

# 제출 전
python -X utf8 _verify10/C/mkpatch_c.py
PYTHONIOENCODING=utf-8 python -X utf8 applypatch.py 10 --only C --dry
PYTHONIOENCODING=utf-8 python -X utf8 _verify10/C/verify_targets.py
```

---

## 1. 함수별 발견

### `10` should_end_object_finish_kill_priority_battle

**전수 대조**: `logic` 의 분기·조건·반환이 IR 과 **전건 일치**. 반환 `phi`(m10.ll:50098) 13개 인입을 하나씩 대조(게이트 실패 `false` / 비‑Moba·빈 live_list·엔티티 null·루프 완주 `true` / `can_enemy_hit_objective` 참 5곳 `false`). `mem` 22행 = IR 의 `getelementptr`·`load` 전량과 1:1, 누락 0.

**빠져 있던 것 · 틀려 있던 것**

| 무엇 | 실제 | 왜 안 잡혔나 |
|---|---|---|
| `consts[5].src_line = 1188` | **1190** — `m10.ll:47496` `icmp ugt i64 %22, 19600000000, !dbg !53163`, `!53163` 루트 = `fight_model.rs:1190`. 1188 은 `can_enemy_hit_objective` 의 **함수 머리줄**(tcx `sp.l`) | **G12 사각지대** — `srclinecheck.py` 는 명세 `ir.frm~to`(49611~50100) 안만 훑는데 이 상수는 47496 에 있다 ⟹ 후보 0건으로 조용히 통과 |
| `knobs[5].where = fight_model.rs:1188` | **1189~1190** — `!53082 = !DILocalVariable(name:"steal_position_range", line: 1189)` 가 리터럴 140000 선언줄, 제곱 비교는 1190 | **G13 사각지대** — `whereline.py` 의 `LINEREF` 는 `[mg]\d{2}\.ll` 만 매치. `*.rs:줄` 형태 `where` 는 **검사 대상이 아니다** |
| `history[1]` 의 `m10.ll:47501` | 47501 은 다음 `gep +1216` 줄. 19600000000 은 **47496** | `history` 는 G17 이 「표에 실렸나」만 본다 |
| ★`closed[3]` 이 **반증된 주장을 그대로 달고 있다** — "version 이 여기서만 쓰이므로 버전 분기는 전적으로 이 함수 몫이다" | `is_enemy_well_danger` 는 `%0`(version)을 **한 번도 로드하지 않는다**(`m03.ll:144500~144563`). 같은 명세의 `sig.params[0]`·`history[3]` 이 이미 뒤집었다 | `closed[]` 는 무검사 축이고, 도시에가 「닫은 근거」를 아예 못 찍는다(§3‑1) ⟹ 배치에는 *반증된 문장 + 빈 답* 만 보인다. **behavior_change = true** |
| `mem[12]`·`mem[15]` 의 `dir` 이 **근거문과 모순** | 근거문이 「`dir` 은 `"-"` 가 맞다」고 자기 입으로 적었는데 표는 `r` | v2 행에 `dir` 키가 없어 `mkspec3.py:381` 이 배열 소속에서 `r` 을 파생. **규약은 있다** — `14 mem[25]/[26]` 은 v2 에 `dir:"-"` 가 박혀 정상 |
| ★**knobs 에 없던 노브** — `can_enemy_hit_objective` 의 **스킬2 `level>2` · 궁 `level>4`** 게이트 | `m10.ll:47564` `icmp ugt i64 %46, 2` / `47594` `ugt %46, 4`, `%46` = `Entity+0x5c8 level`(tcx). 임계 미달이면 그 슬롯 대신 **빈 이펙트 상수** `@anon…40` 을 넣어 사거리 판정이 사실상 실패(47566·47596 `select`) | `history[7]` 이 3차에 확정했는데 `knobs` 로 전파가 안 됐다(**G17 의 빈틈**). `knobs[5]`(140000² 하드컷)가 이미 같은 콜리 내부 상수를 싣고 있으므로 수록 기준은 동일 |

확인만 하고 넘어간 것: `is_recent_visible` 120틱(`_gcbc/g07.ll:157039` `add i64 %24, 120`) ✓ · `objective_entity_id_for_main_objective` switch(`m10.ll:49508~49512`) ✓ · IR 범위 49611~50100 ✓ · 호출처 3곳(심볼 grep 전수) ✓ · `siblings 0` = 자유 함수라 정상(`specgate.FREE_FN` 에 10 포함) ✓.

### `11` v3_fall_back_to_passive

**전수 대조**: `mem` 23행이 IR 접근 전량과 1:1(누락 0) — `sub_plan` 인자(`%0` deref 248 = `data` / `%85`=+0xf8 / `%86` deref 2760 = +0x990)까지 원문 확인. `consts` 19행 `src_line` 전건 일치(417/421/424/425/427/429/432/433). 튜토리얼 허용집합 6표를 IR switch 로 재계산해 `logic` 과 일치(Top{0,2,7,8}/Mid{0,4,5,7,8}/Bottom{0,1,3,5,7,8}/Epic `(t−7)<u250`={0,7,8}/Serpen{0,5,7,8}/Jungle `(t−8)<u249 ∨ t==6`={0,6,8}).

**틀려 있던 것**: `one_line` 이 ①`정정( , 오라클:` 처럼 **괄호 첫 항이 빈 문자열**로 깨져 있고 ②뒤 절반이 함수 설명이 아니라 「`specgate G1` 이 왜 못 잡았나」라는 **파이프라인 메모**다. 사실(version≥2)은 살리고 절차 메모만 걷어냈다. `one_line` 은 무검사 칸이라 5라운드째 그대로였다.

### `12` handle_chat — **이 라운드 최대 수확**

지시가 시킨 「8차의 두 번째 게이트가 제대로 실렸나」를 IR 원문으로 확인했다. **실려는 있었다** — 단 `knobs[7]` 의 **근거문 꼬리**에 한 문장으로 묻혀 있었고(`knobs` 표를 훑어서는 안 보인다), **의미가 틀렸다.**

8차 표기: `+0x1 첫 u8` → `0=Top∧Mid∧Bottom / 1=Mid∧Bottom / 2=Top / 3=Mid / 4=Bottom / ≥5=무조건`

IR 원문(`m13.ll:53679~53685` = Chat 태그 47·48, `53362~53368` = 태그 49) + `!dbg` 사슬:

```
rule_scope::play_code_allowed(ctx, code)        rule_scope.rs:117~124
  0 → morgard_exists(ctx)     :119   (!56897 → spawn_epic runner.rs:263)
  1 → serpen_exists(ctx)      :120   (!56861 → serpen_exists rule_scope.rs:50)
  2 → line_exists(ctx,Top)    :121   (!56872 → line_exists rule_scope.rs:25)
  3 → line_exists(ctx,Mid)    :122
  4 → line_exists(ctx,Bottom) :123
 ≥5 → 무조건 통과(switch default)
호출부 = chat_allowed rule_scope.rs:173 · 176
```

⟹ 팔은 「집합의 교집합」이 아니라 **「그 목표가 존재하는가」라는 술어**다. `spawn_epic={0,7,8}` 이 우연히 `Top∩Mid∩Bottom` 과, `spawn_serpen={0,5,7,8}` 이 `Mid∩Bottom` 과 같아 **오라클(외연)로는 절대 안 갈린다**(`METHOD_MAP ⑥` 한계 3의 교과서적 사례). `tcxdict --enum TutorialType` 이 **variant 9개뿐**임을 확인했으므로 두 표기의 외연은 전 입력에서 일치 ⟹ 정직하게 **`behavior_change = false`**. 바뀌는 것은 **의미**이고, `line_exists` 3중 AND 로 재구현하면 튜토리얼 종류가 늘어나는 순간 갈라진다.

그 밖에 빠져 있던 것:

| 무엇 | 실제 |
|---|---|
| `mem` 에 **`AbstractGameWithCache+0x0`(game 데이터 포인터) 행이 없다** | `m13.ll:29570` `%66 = load ptr, ptr %65` 이고 그 값이 `vtable+0x28 tick` 의 self 로 들어간다(29575). `10`·`11` 은 같은 팻포인터를 +0x0/+0x8 **두 행**으로 적었는데 12 만 +0x8 한 행 — `mem[5].note` 가 「+0x0=데이터」라 적으면서 **행은 안 만들어** `tcxaudit`·G18 기계 대조에서 빠졌다 |
| `logic` 의 `// 내부 미조사` | 3차 이후 `knobs[3]~[8]`·`closed[1]`(45칸 전수 일치)로 해소됐는데 `logic` 만 옛 문면(**G6 의 빈틈**) |

나머지 `mem` 24행·`logic` 줄주석(9·12·18·22·23·27) 전건 일치.

### `13` target_bush_v30

**새 실오류 0건.** `consts` 16행 `src_line` **16/16** 일치(138·139·140·151·154·156·166·168·171·181·184·186), `mem` 15행·`logic` 전 분기 일치. 9차가 지적한 `knobs[9].where` 는 **이미 고쳐져 있었다**(회귀 없음). 유일한 정정 = `mem[8]`(Entity+0x70) `dir` 모순(`10` 과 동일 형태).

### `14` update

**전수 대조**: `consts` 21행 `src_line` **21/21** 일치, `mem` 오프셋 전건 일치, 인라인 `target_bush_v30` 전 분기 일치.

**빠져 있던 것 — `mem` 에 `LineGankerPlan+0x8`(chats.ptr) 행이 통째로 없다.** 두 취소 경로가 **둘 다 읽는다**(`m08.ll:94879~94880` `gep %0, 8` → `load ptr`, `94921~94922`). 이 포인터 + `len*24` 가 `Chat` 원소를 쓰는 **실제 주소**(`94882` `gep {i8,[23 x i8]}, ptr %164, i64 %158`)라 없으면 push 재현이 불가능하다. `tcxdict LineGankerPlan 0x8` = `chats.buf.inner.ptr.pointer.pointer`. **`12` 명세는 같은 Vec 을 cap/ptr/len 세 행으로 적었다** — 같은 구조에 두 명세의 규격이 갈렸다. 그 외 `mem[15]` `dir` 모순 1건.

---

## 2. 판정 어휘

| 항목 | 판정 | 범위 |
|---|---|---|
| `12 knobs[8]~[18]`(handle_chat_inner 내부 11행) ev4→2 | **미탐색** | 오라클 진입은 가능(`handle_chat` 은 `pub`, `_verify5` 가 이미 6,168B 스냅샷 diff 로 관측). 이번엔 전 칸 IR 대조에 시간을 썼고 오라클은 안 돌렸다 |
| `11 knobs[11]~[21]`(passive_plan·SubPlan::merge 내부 11행) | **미탐색** | 동상 |
| `mem[].dir` 의 `"-"` 기계 검사 | **게이트 설계 가능**(표기 불가 아님) | §3‑6 G20 제안, 반증식이라 오탐 희박 |
| `12 mem` 에 `Chat+0x1`/`+0x2` 행 추가 | **기각 — 부재는 결함이 아니다** | `chat_allowed` 는 인라인이 아닌 실제 call(`m13.ll:29478`) |
| `13 exe = None` | **사실 서술** | `define internal fastcc` 라 exe 에 독립 함수가 없다 |

---

## 3. ★내 지시(도시에)의 오류 — 9건

1. ★**`closed[]` 의 「닫은 근거」가 도시에에 한 번도 찍힌 적이 없다.** `mkdossier.py:286` 이 `closed` 표를 키 `("q","a","ev")` 로 렌더하는데 `mkspec3.py:479` 는 `{"q",…,"why"}` 로 만든다 ⟹ **답·ev 칸이 전 함수·전 배치·전 라운드에서 공백.** 이번 라운드가 시킨 검사가 정확히 `closed[]` 근거 유효성인데 도시에가 그 근거를 숨긴다. (`mkspec3_md.py:212` 는 `x["why"]` 를 제대로 찍는다 — **두 렌더러의 계약이 다르다.**)
2. ★**도시에 §5 의 「새 항목 추가(`append`)는 구현이 없다」는 거짓**이고, 같은 지시문 **본문**은 `op:insert` 문법을 안내한다. `applypatch.py:242~277` 에 실제로 구현돼 있다. §5 만 읽은 배치는 「행 추가 불가」로 판단해 *빠진 행* 을 산문에만 적고 끝낸다(8차 배치C 가 실제로 그랬다) — **「가장 값나가는 산출은 빠진 것」이라고 못박은 라운드에 치명적 모순.**
3. `is_closed` 의 근거 귀속 오류: `11 closed[5]`·`[6]` 에 `closed[4]` 의 답(「tcx sig 가 (BigPlan,u8)」)이 붙어 있다. 1번을 고치면 바로 드러난다.
4. **G12 사각지대** — `ir.frm~to` 안만 훑어 **아웃오브라인 콜리의 상수**를 구조적으로 못 본다. 이번 실오류(`10 consts[5]`)가 정확히 거기. 제안: 값이 `ir` 범위에 한 번도 안 나오면 `fnparts` 로 그 값이 있는 함수 범위까지 확장.
5. **G13 사각지대** — `LINEREF` 가 `.ll` 만 매치해 `*.rs:줄` 형태 `where` 는 **아예 검사 대상 밖**. 담당 5함수 `where` 의 과반이 그 형태고 실제로 `10 knobs[5]` 가 함수 머리줄을 가리켰다. 제안: `_tcx` 의 `sp`(파일·줄범위)와 대조해 「그 함수 범위 안인가 / 머리줄 아닌가」만 봐도 잡힌다.
6. **`mem[].dir` 의 `"-"` 규약이 문서화돼 있지 않다.** 제안 **G20**: 근거문에 「`dir` 은 `"-"`」·「읽지도 쓰지도 않는다」가 있는데 행의 `dir` 이 `-` 가 아니면 불일치(반증식).
7. ★**`applypatch.py` 가 성공한 삽입을 「적용 실패」로 찍는다.** `--dry` 가 「★적용 실패 4건」 아래에 insert 경고를 늘어놓고 바로 다음 줄에서 「정정 15 성공 / 0 실패」라 한다. 「빨간불이 상수가 되면 아무도 안 본다」(`§S5-f`)가 **이 도구 자신에게** 생긴 사례.
8. ★**`insert` 경고의 「이후 N행 이동」은 v2 배열 안만 센다.** `/specs[12]/knobs at=3` 은 「이후 0행 이동」이지만 v3 `knobs`=`knobs`+`new_knobs` 라 `new_knobs` 18행의 **v3 인덱스가 전부 +1**. 더 위험한 것은 **`ev_up` 이 `errors` 뒤에 적용된다**는 점(`applypatch.py:460 → 472`) ⟹ 같은 배열에 insert 를 낸 배치가 `ev_up` 경로를 삽입 **전** 인덱스로 쓰면 **조용히 엉뚱한 행에 도장이 찍힌다.** 내 첫 판이 정확히 그랬고 `--dry` 는 `2/2 성공` 으로 통과시켰다 — `verify_targets.py` 를 만들어서야 잡았다. 제안: `apply_evup` 도 `guard`/`guard_key` 를 받게 할 것.
9. `13 exe = None` 이 「`None`(None)·None바이트·None명령」으로 찍혀 **「조인 실패」와 구별되지 않는다**(`callers: []` 에는 이미 같은 주의가 있다).

**판정 반전(= 오류로 셈) 2건**: ① `10 closed[3]` 의 「버전 분기는 `is_enemy_well_danger` 몫」 → 거짓 ② `12 knobs[7]` 의 두 번째 게이트 표기 → `morgard_exists`/`serpen_exists` (8차 오라클 결론이 **외연으로는 맞고 의미로는 틀렸다**).

---

## 4. 스스로 반증한 오탐 — **10건** (후보 25 → 진짜 15, 오탐률 40%)

| # | 가설 | 기각 근거 |
|---|---|---|
| 1 | `12 mem` 에 `Chat+0x1/+0x2` 가 빠졌다 | `chat_allowed` 는 실제 call(`m13.ll:29478`) — 본문이 안 만진다 |
| 2 | `13 consts[8]@168`·`[10]@166` 이 뒤바뀌었다 | `!23218=168`(참 팔)·`!23219=166`(거짓). IR 술어가 `!is_top_side` 라 역전이 정상 |
| 3 | `callees[].path` 가 `fight_model` 모듈을 빠뜨렸다 | tcx `p` 는 재수출 경로(`…::old::can_enemy_hit_objective` 로 실제 등재) |
| 4 | `10 siblings = 0` 은 열거 실패 | siblings 는 Plan 타입 메서드 전용, `specgate.FREE_FN` 에 10 포함 |
| 5 | `11 closed[5]/[6]` 은 과닫힘이다 | `history[5]`·`[4]` 에 진짜 답이 있다 — 닫힘은 옳고 `why` 표시만 오귀속 |
| 6 | `14 knobs[2]`(tower.rs:99) ↔ `13 knobs[6]`(tower.rs:100) 모순 | 99=정의줄(tcx), 100=비교줄(`!23180`). 둘 다 유효 |
| 7 | `11 consts` 에 Jungle 상한 `-7` 이 없다 | `consts[15]`(-7) 의 `meaning` 이 두 역할을 모두 적는다 |
| 8 | `11 knobs[3].where`(90~95) 와 switch(91) 불일치 | 범위 표기라 포함. `logic` 은 91~96 을 정확히 적는다 |
| 9 | `12 writes[11]/[12]`(cap·ptr 쓰기)는 이 함수가 안 하는 store | `RawVec::grow_one` 경유, 콜리 효과를 싣는 관례와 동일 기준 |
| 10 | `14 consts` 에 Chat 태그 17 / CancelReason 0·2 / phase 8 이 없다 | 값 중복 제거 관례로 `consts[8]·[4]·[2]·[17]` 에 병기 |

★ 8·9차의 오탐률 70~95% 보다 낮은 이유는 이번이 **게이트 후보를 검증한 게 아니라 IR→명세 방향으로 직접 읽었기** 때문이다. 「게이트가 준 후보」는 오탐이 지배하지만 「IR 원문에서 역으로 찾은 것」은 대부분 진짜였다 — 다음 라운드 설계에 반영할 것.

---

## 5. 제출물 내역

`errors` **15** — `10` 6건(src_line·where·history·closed·dir×2) + **knob 1행 추가** / `11` 1건(one_line) / `12` 2건(두번째게이트 의미·logic) + **mem 1행·knob 1행 추가** / `13` 1건(dir) / `14` 1건(dir) + **mem 1행 추가**.
`ev_up` **2** (ev4→3, 둘 다 tcx 오프셋 대조: `11 knobs[14]` `v2_assign@0x1802` · `12 knobs[9]` `v3_epicops_armed@0x180a`).
`behavior_change` **1** (`10 closed[3]` 의 버전 분기 오주장).

⚠ **적용 시 `--restamp` 필수**(삽입 4건이 v3 인덱스를 민다). ⚠ `ev_up` 의 `/specs[12]/knobs[9]` 는 **삽입 후 인덱스**다(§3‑8).
