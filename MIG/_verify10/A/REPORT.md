# 10차 배치 A — 전수 감사 보고 (담당 `00`~`04`)

> 신선도: 착수 시 `[A] FRESH`, patch 제출 직전 재확인 `[A] FRESH` (도장 2026-09-11 21:16:34).
> 방향: 지시대로 **IR→명세** 를 주 방향으로 잡았다. 5함수 전부 `ir.frm~ir.to` 를 통째로 떠서
> **gep 오프셋·리터럴·분기 전량**을 뽑고, 그 다음에 명세 표와 맞췄다.

---

## 0. 한 줄 결론

| 축 | 결과 |
|---|---|
| `mem` 오프셋·방향 (5함수 107행) | **불일치 0** — 단 `04` 에 **행 하나가 통째로 없었다**(Vec+0x0 ptr) |
| `consts` 값·src_line (40행) | **불일치 0**(`dloc` 전수). `02` 한 행이 **서로 다른 두 줄의 상수 2개**를 담고 있었다 |
| `knobs.where` (49행) | ★**실오류 2 · 보강 2** — 전부 **백틱 IR 인용이 없는 행**에서 나왔다 |
| `closed[]` 근거 유효성 (38건) | ★**7건 무효**(판정반전 1 포함) — 도구·history 가 이미 답했는데 전파가 안 됐다 |
| `logic`·`one_line`·`layer`·`callees`·`notes`·`siblings`·`callers` | 내용 불일치 0 · 단 **도시에 렌더가 `siblings` 를 통째로 비웠다** |
| `behavior_change` | **0건** |

정정 16 · ev상향 2 · **스스로 반증해 버린 후보 7건**.

---

## 1. 실제로 실행한 것 (명령줄 그대로)

```bash
cd /c/tfm2mods/MIG
python -X utf8 dossierfresh.py 10 A                    # 착수 / 제출 직전 2회

# IR 원문 (담당 범위 전량)
sed -n '43960,44370p' /c/tfm2mods/_gaibc/m04.ll        # 00 ult
sed -n '39984,40103p' /c/tfm2mods/_gaibc/m05.ll        # 01 calculate_jungle_action_score
sed -n '34867,34976p' /c/tfm2mods/_gaibc/m12.ll        # 02 sub_plan
sed -n '33864,34294p' /c/tfm2mods/_gaibc/m10.ll        # 03 defensive_crisis
sed -n '58175,58644p' /c/tfm2mods/_gaibc/m04.ll        # 04 handle_line_defense

# src_line 사슬 (inlinedAt 루트까지)
python -X utf8 dloc.py m04.ll 56281 56110 56218 56370 56400 56407 56416 56421 56434 56441 \
                              56472 56492 56503 56505 56526 56528 56534 56544 56546 56550
python -X utf8 dloc.py m05.ll 44379 44384 44391 44393 44395 44402 44403 44404 44407 44409 \
                              44410 44411 44412 44413 44416 44417
python -X utf8 dloc.py m12.ll 62937 62946 62954 62962 62963 62965 62967 62968 62969 62970 \
                              62977 62984 62985 62986 62988 62989 62990
python -X utf8 dloc.py m10.ll 40699 40704 40711 40713 40718 40740 40749 40754 40760 40815 \
                              40817 40819 40824 40848 40855 40868 40892 40893 40903 40909 \
                              40944 40949 40950 40960
python -X utf8 dloc.py m15.ll 42663 42670 42677 42684 42506

# 타입 정본
python -X utf8 tcxdict.py AbstractGameWithCache
python -X utf8 tcxdict.py AbstractGameWithCache 0x130
python -X utf8 tcxdict.py "bumpalo::collections::vec::Vec<'{erased}, &'{erased} game_core::Entity>"
python -X utf8 tcxdict.py "bumpalo::collections::raw_vec::RawVec<'{erased}, &'{erased} game_core::Entity>" --deep
python -X utf8 tcxdict.py MobaMode
python -X utf8 tcxdict.py Strategy
python -X utf8 tcxdict.py --enum Position

# 줄 길이 산술
python -X utf8 rmeta_srcmap.py game_ai action_score.rs 518 550
python -X utf8 rmeta_srcmap.py game_ai "plan_legacy\old\attack_nexus.rs" 31 55
python -X utf8 rmeta_srcmap.py game_ai buff_value.rs 17 55

# knobs.where 반증 (담당 범위 밖 인용)
awk 'NR>=31120 && NR<=33053 && /icmp ugt i64 %[0-9]+, %33,/ {print NR": "$0}' /c/tfm2mods/_gaibc/m15.ll
sed -n '115823,115910p' /c/tfm2mods/_gcbc/g15.ll
awk 'NR>=105150 && NR<=105240 && (/udiv/||/mul i64/) {print NR": "$0}' /c/tfm2mods/_gcbc/g15.ll
sed -n '60445,60460p' /c/tfm2mods/_gaibc/m04.ll

PYTHONIOENCODING=utf-8 python -X utf8 applypatch.py 10 --only A --dry
# -> [A] 정정 16/16 · ev상향 2/2 · 동작 변경 0건
```

---

## 2. 함수별

### `00 ult` — 정정 0 · ev상향 1

* **IR→명세 완전성**: 본문이 만지는 게임 구조체 오프셋은
  `%3`{0x930,0x9c0} · `%4`{0x0,0x8} · cache{0x1e0} · GameContext{0x8,0x20} ·
  Entity{0x0,0x8,0x38,0x438,0x470,0x538,0x5c8,0x660,0x668,0x680} ·
  Effect{0x0,0x8,0x10,0x18,0x28,0x30} · vtable{0x10,0xf8,0x118} · sret{0x0,0x8} 뿐이고
  **전부 표에 있다.** 표에 없는 유일한 메모리 접근은 `%8`(24B alloca = `Option<InputTarget>`) 니치
  로드인데 **스택 지역이라 `mem` 대상이 아니다** → 결함으로 세지 않았다.
* 줄 인용 전수 대조(파일줄 = 덤프줄 + 43959): `44025` · `44062/44063/44064/44065/44066` ·
  `44068/44070` · `44085/44087` · `44245~44247` · `44266/44268` · `44298/44300` · `44272` ·
  `44313` — **9건 전부 ±0**.
* `consts` 10행 `src_line` 도 `dloc` 전수 확인(281/286/293/191/328/333/326).
* **범위 밖 노브 2건도 실측**: `knobs[7]` 4,000,000 = m04.ll:43950 `icmp ugt i64 %310, 4000000`
  (`!dbg` = abstract_input.rs:134) · `knobs[8]` 2000/1500 = 43467 `icmp slt i64 %49, 2000` /
  43717 `icmp sgt i64 %189, 1500`(`!dbg` = approach_dodge_steer 61·85 <- safe_move:143). 둘 다 ±0.
* `mem[11]` 이 인용한 `sext i32 %9 to i64 @43306` · `%33 @43339` 도 원문 확인 — 맞다.
* `callees` 30건 중 실제 IR call 은 v2 `calls` 12건이 정확히 덮는다.
* **ev상향 1건**: `consts[0]`(=2) 근거가 IR 뿐이라 ev4 였는데 `tcxdict` 가
  `player_champion: [[Option<&Entity>;5];2]`(80B) 를 직접 주므로 tcx 정본 대조(ev3)로 내린다.

반증한 후보 (00):
1. 「`mem[26]` 이 `store i64 5` 를 빠뜨렸다」 → **오탐.** v2 원본에 `value: "-1 (None) 또는 5"` 가
   있고 **md 렌더가 `value` 칸을 안 그린다.**
2. 「`consts[5]`(0@191)와 `consts[7]`(100@333)의 `src_line` 기준층이 다르다」 → 사실이지만
   G12 가 사슬 어느 층이든 받으므로 값 오류가 아니다 → 제기하지 않음.

### `01 calculate_jungle_action_score` — 정정 1(보강) · ev상향 1

* `mem` 8행 = IR 이 만지는 오프셋 8개와 **정확히 1:1**.
* `knobs` IR 줄(40043 phi · 40066 select · 40091 select)과 `consts` 12행 `src_line`
  (522/526/528/530/531/536/542) **전부 ±0**. `40066 select i1 %43, i64 20, i64 40` 의 arm 방향
  (`%43 = icmp sgt i64 %27, %25` = hp>value -> 20)도 일치.
* **`closed[4]` 무효화** — 「소스 527·529·532~535·537~541 이 중괄호인지 소멸 코드인지 **확인 불가**」.
  `rmeta_srcmap` 줄 길이 + IR 로 이미 확정된 arm 값 + history[3] 의 **2칸 들여쓰기** 기준선으로
  **16줄이 ±0 정합**(`    200` 7자 / `    80` 6자 / `      40` 8자 / `    } else {` 12자 …).
  ⟹ **소멸한 코드는 없다.** 범위: 동일 길이 대안 표기는 못 가른다.
* **ev상향 1건**: `consts[0]` 도 같은 근거로 ev4→3.

반증한 후보 (01):
3. 「`sig.params.i` 가 1-based 라 tcx 인자 번호와 어긋난다」 → `#` 열이 0~6 으로 정확하고
   IR 인자 7개 ↔ params 7행이 1:1 이다 → 제기하지 않음.

### `02 sub_plan` — 정정 4(보강) · 도시에 결함 1건 발견

* `mem` 21행(reads 17 + writes 4) = IR 전량과 1:1.
  `%57 = gep {{ptr,ptr,i64},i64}, %17, %56` 의 **stride 32 · 기준이 cache 자신**이라
  `+328`(team=1)/`+360`(team=0)이 되는 것까지 note 와 일치.
* 분기 방향 재검증: `%60 = (len==0)` -> `%67`(phi 16 AttackNexus), else -> `%61`(LineDefense 2).
  ⟹ `has_enemy_twin_tower = len != 0`, `logic` 의 **긍정형 if** 가 맞다.
* `ret void, !dbg !62990` = attack_nexus.rs:**52**, rmeta L52=3자(`  }`) · L51=5자(`    }`) —
  `logic` 의 `} // L51` 과 정합.
* 정정 4건:
  * `closed[2]` 「`if A && B` 인지 중첩 if 인지 구분 불가」 → 같은 파일 **`history[4]` 가 이미
    L38 `let is_in_heal_area = …`(91자 ±0) · L41 `if is_in_heal_area && …`(잔차 1자)로 확정**.
    전파 누락(G17 형인데 G17 은 `unknown` 을 안 본다).
  * `closed[4]` 「bumpalo Vec 앞 24B 필드명 미확인」 → **`tcxdict` 가 그대로 준다**:
    `Vec = {0x0 buf: RawVec(24B), 0x18 len}` / `RawVec = {0x0 ptr.pointer, 0x8 a: &Bump, 0x10 cap}`.
    ⟹ 둘째 워드는 `ptr` 이 아니라 **할당자 `&Bump`**. `distruct` 의 `?`(64B)는 도구 한계였다.
  * `closed[1]` 「`Position::as_index` 재배치 여부 미확정」 → `tcxdict --enum Position` 이
    5 variant 전부 `idx == 선언discr == 메모리태그`(0~4)를 주고, 인라인 본체가
    `zext nneg i32 -> i64` **맨몸 하나**(m12.ll:34888~34889) ⟹ **항등 확정**.
  * `consts[3]`(0@45) — 한 행이 **두 줄의 서로 다른 0** 을 담았다.
    `icmp eq i64 %59, 0`(34957, `!62985` = vec.rs:1636 is_empty <- :45)와
    `store i8 0`(34965, `!62989` = :**48**, LineStyle::Aggressive).
* ★**도시에 결함**: `형제 9개` 표가 9행 전부 공란. 데이터에는 9건이 있고
  `mkdossier.py:266` 컬럼 키(`name`/`sym`/`note`)가 실제 키(`path`/`vis`/`at`/`mir`/`sig`)와 다르다.

반증한 후보 (02):
4. 「`consts[1]`(Recall 5) src_line 42 가 IR(블록49 종결자 = 41)과 어긋난다」 → **오탐.**
   rmeta L42 = 55자 = indent 6 + `return SubPlan::Recall(RecallSubPlan::default());`(49자) ±0.
   IR 귀속은 arm 종결자라 판별력이 없다.
5. 「`open[0]`(version) 은 `사실 서술` 이라 notes 로 가야 한다」 → 전역 물음은 여전히 `미탐색`,
   본문이 범위를 적고 있다 → 제기하지 않음.

### `03 defensive_crisis` — 정정 4(실오류 1 · 보강 3)

* `mem` 22행 = IR 전량과 1:1. `target` 이 `+0x5c0` 한 곳만 직접 읽는다는 `sig` 주석도
  `%46 = gep %4, 1472` @33981 로 재확인(`ptr %4, i64 1632/1640` gep 은 범위 안 0건).
* `consts` 7행 `src_line`(22/43/44/45/48/35/48) **전부 `dloc` ±0**. `closure$1`/`closure$0`
  프레임까지 사슬을 타야 나오는 값들인데 명세가 정확히 반영하고 있다.
* ★**`knobs[10]` 실오류.** `where` = `m15.ll:31432·31504·31688·31780 (cd > tps)` 인데
  **3줄이 그 비교가 아니다**: 31432 = `i64 7, label %98`(switch case 라벨) ·
  31504 = `%117 = load i32, ptr %116`(PlayerState+0x9c0) · 31780 = `238:`(블록 라벨).
  실제 4곳 = **31486·31586·31688·31790**(`icmp ugt i64 %N, %33`, %33 = tps@31242),
  소스 = `fight_check.rs:1005·1010·1015·1021`. G13 은 백틱 인용이 없는 행은
  「줄이 존재하나」만 보므로 사실상 무검사였다.
* `knobs[13]`(보강) `g15.ll:105194` → 실제는 **105197 `mul`(tps×dmg) · 105198 `attack_cooltime` ·
  105211 `udiv`**. 105194 는 피해값을 배열에 쓰는 gep.
* `knobs[5]`(보강) `34264~34268` → 실제 **34265 ~ 34269** — 인용한 memset 이 옛 범위 밖.
* `closed[3]`(보강) 「`is_empty` 인지 LLVM 접힘인지 구분 못 했다」 → len 로드의 `!dbg` 사슬이
  `vec.rs:1617 len <- vec.rs:1636 **is_empty** <- buff_value.rs:30`. 접힘이면 이터레이터 프레임이
  남는다 ⟹ 둘째 가설 기각. 범위: 「L30 에서 `is_empty()` 를 부른다」까지가 확정.
* 확인만 하고 넘긴 것(±0): `knobs[6]` `m15.ll:32956 mul i64 %703, 60` ·
  `knobs[9]` `31294~31297`(`sub 1000,%48`@31295 · `lshr`@31296) ·
  `knobs[11]` `31289 udiv %39,%42`(%42 = `shl %41,1`@31257) · `knobs[12]` `31666 llvm.umax` ·
  `knobs[14]` `g15.ll:105462`(gep **1716 = 0x6b4**).

반증한 후보 (03):
6. 「`logic` 의 `[L53] return DefensiveCrisis{…}` 가 틀렸다 — rmeta L53 은 **1자**(`}`)」 →
   **자기반증.** `ret` 의 `!dbg !40960` 이 **line 53** 이다. 줄 길이 **추론으로 IR 직접근거를
   기각**하는 꼴이라 도시에 규칙(「귀속·추론은 구제에만」)대로 버렸다.
7. 「`mem[8]`(cache+0x0 game)이 +0x8(vtable)을 따로 안 적었다」 → note 가 `cache+0x0/+0x8 =
   (data, vtable)` 이라 명시 → 오탐.

### `04 handle_line_defense` — 정정 6(실오류 1 · 판정반전 1 · 보강 3 · 행추가 1)

* `mem` 27행 중 26행 = IR 전량과 일치. `%54/%55 phi`(384/400 · 416/432 · 448/464) ↔ mem[11]~[16],
  `%39 = gep %30, 576`(0x240) ↔ mem[8], `%15 = gep %12, 56`(0x38) ↔ mem[5],
  `%167 = gep %8, 14`(0xe) ↔ mem[21], `%21 = gep %20, 16` / `%25 = gep %20, 32` ↔ mem[25]/[26].
* `knobs` IR 줄 `58200` · `58583` · `58635` · `58636` · `60449` · `60455` — **6건 전부 ±0**.
* ★★**`mem` 에 행이 하나 없다** — `Vec<&Entity>(bumpalo) +0x0 ptr`.
  IR 이 두 곳에서 로드한다: `m04.ll:58300 %66 = load ptr, ptr %65`(twin_towers[아군팀]) ·
  `m04.ll:58451 %117 = load ptr, ptr %9`(champions). 표엔 `+0x18 len` 행만 있고 ptr 은 그 행
  **note 안에 묻혀** 있었다 — 같은 명세 `mem[22]~[24]` 가 「한 행에 오프셋을 묶으면 기계 검사가
  안 되므로 라인별로 쪼갰다」고 **자기 규약을 적어 놓고** 어긴 자리다. 같은 접근이 `03` 에는
  `mem[10]` 으로 제대로 실려 있다. ⟹ `op:insert` 로 추가(+ `mem[17]` note 정정: `%68`·`%119` 는
  「벡터」가 아니라 **len 값**).
* ★**`knobs[4]` 실오류 + 구조 정정.** `where` = `g15.ll:115838~115839` 인데
  **115838 은 `unreachable`, 115839 는 빈 줄**. 실제는 `get_start_position`(define @115823) 안의
  **115875·115876 phi 2개(라인별 3쌍)** + **115891~115893 팀 스왑**이다.
  값 서술 「좌표 6종」도 구조를 흐린다 — IR 은 **3쌍만** 갖고 team 축은
  `team==1 ? (x,y) : (y,x)` **x<->y 스왑**으로 만든다 ⟹ **team0 을 team1 과 독립으로 못 바꾼다.**
  (기재된 6값 자체는 전부 맞다 ⟹ `behavior_change=false`.)
* ★**판정 반전 1건** — `closed[0]` 이 ①「소스 표기 확정 불가」 ②`morgard_exists` 구조
  ③「{0,7,8} 은 추정」 셋을 모두 미확정으로 말하는데 **같은 파일 `history[3]` 이 셋 다 뒤집어 놨다**
  (L544=39자·L549=134자 복원 ±0 으로 **부정형 확정**, MIR `switchInt -> [0,7,8:true]`,
  오라클 9값 실행). 명세 안에서 `closed` 와 `history` 가 **정반대**였다.
* `closed[2]`(보강) 「`has_line_defense_threat` 도 define 없는 외부 선언」 → **define 이 있다**
  (`m04.ll:60400`, history[4]). 같은 명세의 `mem[22]~[26]`·`knobs[5]`·`knobs[6]` 이 **이미 그
  define 의 오프셋·임계를 싣고 있으면서** 이 행만 「없다」고 말하는 자기모순.
  `champions` 2번째 인자 = 팀 도 history[3]④ 가 본체(`g15.ll:109887~109895`)로 확정.
* `closed[4]`(보강) 「MobaMode 크기 교차검증 못 했다」 → `tcxdict MobaMode` = **640B · 필드 9**,
  `0x240 epic_minion_buff_time: [usize;2]`. 이웃이 전부 `[usize;2]`(0x250/0x260/0x270)라
  오귀속 여지가 있었는데 그것까지 닫힌다.

---

## 3. 「빠져 있던 것」 — 이번 라운드의 본래 표적

| # | 무엇 | 어디 | 어떻게 찾았나 |
|---|---|---|---|
| 1 | **`mem` 행 자체가 없음** — `Vec<&Entity> +0x0 ptr` | `04` | IR→명세 방향. gep/load 전량 열거 후 표와 차집합 |
| 2 | `consts` 한 행에 **다른 줄의 상수 2개** | `02 consts[3]`(0@45 + 0@48) | `dloc` 로 두 `!dbg` 가 다른 줄임을 확인 |
| 3 | `knobs.where` 가 **명령이 아닌 줄**(switch case 라벨·블록 라벨·`unreachable`·빈 줄)을 가리킴 | `03 knobs[10]`(3/4) · `04 knobs[4]`(2/2) | 인용줄을 `sed` 로 직접 열어 봄 |
| 4 | `closed` 가 **이미 답이 난 물음**을 미확정으로 붙들고 있음 | `01`x1 · `02`x3 · `03`x1 · `04`x3 | tcxdict / rmeta_srcmap / dloc / 같은 파일 history |
| 5 | 도시에가 **`siblings` 9행을 통째로 비움**(데이터는 있음) | `02` | v3 JSON 직접 조회 |

**7건이 `closed`** 다. 이 축은 어떤 게이트도 보지 않는데, **바뀐 것은 사실이 아니라 도구**였다 —
`distruct`→`tcxdict` 강등(METHOD_MAP ②), `rmeta_srcmap` 줄 길이 산술, `dloc` 의 inlinedAt 사슬.
⟹ `closed` 는 **닫힐 때의 도구 사정을 화석으로 보존**한다. 도구가 바뀔 때마다 재검토 대상이다.

---

## 4. 다음 라운드에 붙일 게이트 제안

1. ★**`G20 knobs.where 인용줄이 명령인가`** — 이번 실오류 2 + 보강 2 를 **전부** 잡는다.
   `where` 에서 `<파일>:<줄>`(및 `A~B`)을 뽑아 그 줄이 **블록 라벨(`^\d+:`) / 빈 줄 /
   switch case 라벨(`^\s+i\d+ \d+, label`) / `unreachable` / `#dbg_*` 전용**이면 불일치.
   백틱 인용 유무와 무관해 G13 사각지대를 정확히 덮는다. 담당 5함수 `knobs` 49행 중
   IR 줄을 인용하면서 백틱이 없는 행 **13행**, 그중 4행 적발(검출력 ≈ 31%, 음성 대조 9행 전원 통과).
2. **`G21 unknown <-> history 상호참조`** — `G17` 의 짝. `G17` 은 `history`→**표**만 본다.
   이번 `closed` 7건 중 4건이 **같은 파일 `history` 가 정답을 갖고 있는데 `unknown` 이 반대로
   말하는** 형태였다. 판정식은 좁혀야 한다(G17 오탐률 90% 교훈):
   `history[i].was` 와 `unknown[j]` 의 **식별자 3개 이상 일치 + `unknown[j]` 에 `UNSURE` 어휘 존재**.
   이 조건이면 이번 4건은 전부 걸리고 이미 `★해소` 가 붙은 행은 안 걸린다.
3. **`G22 mem 행 누락`** — 담당 IR 범위의 `getelementptr … i64 <상수>` 상수 오프셋 집합 ↔
   `mem[].offset` 집합 차집합. ⚠**스택 alloca 기준 gep 와 Vec/이터레이터 내부 gep 을 먼저
   걸러야** 오탐이 지배한다(`00` 의 `%8`, `03` 의 `%9`/`%7` — 8차 `G14` 후보가 46건 냈던 함정).
   거른 뒤 `04` 의 `Vec+0x0` 은 살아남는다.

---

## 5. 내 지시(도시에)의 오류 — `brief_errors[]` 5건

1. ★`mkdossier.py:266` 이 `siblings` 표를 **빈 칸으로 렌더**(컬럼 키 불일치). §3 의
   「어떤 칸도 줄이지 않았다」가 이 칸에서 깨진다. 담당 5함수 중 `siblings.count>0` 은 `02` 하나뿐이라
   다른 라운드에선 안 드러났을 수 있다. 잃은 정보 예: `AttackNexusPlan::next_plan` 이 **`mir=True`** —
   `history[1]` 은 「define 이 없고」까지만 적고 MIR 경로를 안 봤다.
2. ★도시에 §5 는 「**`append` 는 구현이 없다** … 메인이 손으로 넣는다」인데 지시문 본문은
   「행 추가는 `op:insert`」다. **서로 반대다.** `applypatch.py:239~275` 를 직접 읽어야
   `insert`/`delete` 는 있고 `append` 만 없다는 걸 안다.
3. ★`op:insert` 의 **`at` 이 v3 가 아니라 v2 기준**이다. `resolve(cont, field, 0)` 때문에
   `/specs[i]/mem` 은 **항상 `reads`** 를 가리킨다 — `writes` 행은 이 경로로 못 넣고,
   모르면 조용히 엉뚱한 배열에 꽂는다. 도시에 어디에도 없다.
4. ★§4-b 무검사 축 목록에 **`knobs.where` 중 「IR 인용이 없는 행」**이 빠져 있다.
   `03` 의 `knobs` 21건 중 9건이 담당 범위 밖을 가리키고 대부분 백틱 인용이 없다 = G13 무검사.
   이번 실오류 2건이 정확히 거기서 나왔다.
5. `03` 헤더가 `exe | None (None) · None바이트 · None명령` 으로 찍힌다 — `—`(미확정) +
   `exe 미매핑` 이 낫다.

---

## 6. 판정 어휘로 남기는 것

* `미탐색` — `03 open[0]` 의 **이터레이터 조각 `m01.ll:37431~37606`(175줄)**. 이번에도 안 읽었다.
* `표기 불가` — `02` L41 잔차 1자(history[5] 종결) · `01` L531 `hp <= value` <-> `value >= hp`.
  신규: `01` L527~L546 복원은 **줄 길이 기준**이라 같은 길이의 대안 표기를 못 가른다.
* `사실 서술` — `04 closed[0]` 은 물음이 아니라 **이미 답이 난 사실**이었다 → 본문을 그렇게 고쳤다.
* **`불가` 를 쓴 곳은 없다.** `closed` 7건 중 4건은 도구를 바꾸니 열렸고, 3건은 같은 파일
  `history` 를 읽으니 이미 열려 있었다.

---

## 7. 제출물

```
_verify10/A/patch.json   errors 16(실오류 2 · 판정반전 1 · 보강 12 · 행추가 1) · ev_up 2 · brief_errors 5
_verify10/A/REPORT.md    이 파일
```

`applypatch.py 10 --only A --dry` = **정정 16/16 · ev상향 2/2 · 동작 변경 0건**
(`found_by`: new 14 · reused 2).
⚠`/specs[4]/mem at=17` **삽입이 있으므로 적용 후 `--restamp` 필수**(S5-g 인덱스 도장 문제).


---

## 8. 제출 직전에 일어난 일 (라운드 중 도구 변경 2건)

### 8-1. ⛔`applypatch.py` 가 21:46 편집으로 **깨졌다** — 적용 전에 반드시 고쳐라

첫 dry-run(21:38) 은 `정정 16/16 · ev상향 2/2` 로 통과했다. REPORT 를 쓰고 재확인하니:

```
Traceback (most recent call last):
  File "C:\tfm2mods\MIG\applypatch.py", line 556, in <module>
    sys.exit(main())
  File "C:\tfm2mods\MIG\applypatch.py", line 467, in main
    r = apply_error(D, e, log, skipped)
  File "C:\tfm2mods\MIG\applypatch.py", line 280, in apply_error
    WARN.append((u"삽입 — 뒤 인덱스가 밀렸다. 끝나고 `--restamp` 를 돌려라",
NameError: name 'WARN' is not defined
```

280행이 `log.append` → `WARN.append` 로 바뀌었는데(주석에 「10차 배치B·C 가 둘 다 지적」)
**`WARN` 이 어디에도 정의돼 있지 않다** — `grep -n WARN applypatch.py` 가 **280행 한 줄**만 낸다.

* 증상 범위: `op:insert`/`op:delete` 가 든 patch 를 만나는 즉시 **프로세스가 죽는다.**
  한 항목 실패가 아니라 **그 실행의 모든 배치 정정이 0건 적용**으로 끝난다.
* 내 배치는 `/specs[4]/mem` 삽입 1건이 있어 **직격**이다.
* ⛔MIG 루트 도구 수정 금지라 **고치지 않았다.** 대신 scratchpad 래퍼로 `WARN = []` 만 주입해
  검증했고, 그 상태에서 **정정 16/16 · ev상향 2/2 · 동작 변경 0건**이다.
* ★교훈의 모양이 낯익다 — 「경고를 실패 통에 넣지 마라」를 고치다 **경고 자체를 못 찍게** 만들었다.
  9차의 `G12` 「거짓 초록」(예외를 삼켜 0 을 찍음)과 **정확히 반대 방향의 같은 실수**다:
  그때는 죽은 검사가 통과로 보였고, 이번은 산 적용이 통째로 죽는다.

### 8-2. `dossierfresh` 가 **STALE** 을 냈다 — 확인 결과 **내 patch 에는 영향 없음**

```
[A] ★STALE — 생성기 mkdossier.py 도장 af0a7efd074c9da7 → 926d3bef77eb10e4
```

확인한 것:

| 항목 | 결과 |
|---|---|
| `_spec/specs20.json` / `_v3.json` 해시 | `cfad6faecd9d1003` / `9a1edfa8fc364b08` — **§0 도장 그대로** |
| `DOSSIER_A.md`·`spec_*.md` mtime/크기 | **21:16 그대로**(내가 읽은 바이트와 동일) |
| §5 **스키마** | 최상위 키 `round`/`batch`/`errors`/`ev_up`/`brief_errors` **불변**, `op:insert` 형식 **불변** |
| 바뀐 것 | `mkdossier.py:394~398` — **내가 `brief_errors[1]·[3]` 로 낸 지적이 반영**됐다: ~~「`append` 는 구현이 없다」~~ → 「**행 추가·삭제가 된다**」 + 삽입 시 `ev_up` 경로 주의 + `open`/`notes` 에는 못 쓴다 |

★새로 들어온 규칙 하나를 내 patch 에 대조했다 — 「같은 배열의 `ev_up` 경로는 **삽입 후 인덱스**로 써라」.
내 `ev_up` 2건은 `/specs[0]/consts[0]`·`/specs[1]/consts[0]` 이고 삽입은 `/specs[4]/mem` 이라
**배열도 spec 도 겹치지 않는다.** 또 `/specs[4]/mem[17]/note` 정정은 `errors[]` 에서
**삽입보다 앞에** 두어 옛 인덱스(len 행)를 맞춘다 — dry-run 이 그대로 통과한다.

⟹ **재작업 없음.** 단 §0 규칙대로 멈춰서 확인은 했고, 그 확인 절차를 여기 남긴다.
