# 18차 배치 B 보고 — specs[62]~[66] (게임 0.5.8)

신선도: `python -X utf8 dossierfresh.py 18 B` → **FRESH** (착수 시·제출 직전 2회, 도장 2026-09-13 18:15:37).
출력: `_verify18/B/patch.json` (정정 47 · ev상향 0 · brief_errors 4) — `applypatch.py 18 --only B --dry` = **47/47 성공 / 0 실패**.
생성기 = `_verify18/B/mk18B.py`(mkpatch 참조구현 + insert 6건은 dict 직접) · 사후 시뮬레이터 = `_verify18/B/sim18B.py`
(patch 를 v3 **사본**에 적용하고 paramrole/srclinecheck/kindchk/sharedchk/specgate G7·G10·G18 을 재실행 — 정본 무수정).

## 0. 내 지시(도시에)의 오류 — `brief_errors` 4건
1. §4 의 **G7 과 G10 은 같은 항목**(#66 open[0])인데 두 건으로 열거돼 있고, 그 답은 `shared.is_recent_visible.blackboard_인덱스_의미` 에 ★확정으로 이미 있었다.
2. §4 G20 R4 「같은 노브가 39/40」— #64 의 40 은 **본문 리터럴이 아니라 콜리에 넘기는 인자**이고 비교(`icmp ult`)는 콜리 안에 있다. 게이트는 `knobs.what` 문자열 동일성만 보므로 「값 통일」이 아니라 **이름에 리터럴/인자 구분을 담는 것**이 올바른 조치인데 지시문 「문면을 통일」은 이 방향을 말하지 않았다.
3. §5 「`open`/`notes` 에는 못 쓴다」는 insert/delete 얘기이고, **문면 치환**(`/specs[i]/open[k]` + `old` 부분문자열)은 `applypatch` ② 경로(v2 `unknown`/`still_unknown` 문면 탐색)로 된다. 이 사실이 빠져 있어 open 정정 9건을 산문으로 미룰 뻔했다(패치로 냈다 — 거부되면 §1·§2 근거를 notes 로 옮기면 된다).
4. `logic`/`consts.meaning`/`params.role`/`open` 안의 `mNN.ll:LINE` 인용이 **#62 는 8곳 전부, #65 는 5곳, #66 은 3곳**이 현행 `.ll` 과 어긋나 있었는데 어떤 게이트도 이를 보지 않는다(G13 은 `knobs.where` 만, G16 P6 는 `params.role` 만). → §5 검사기 제안.

## 1. 초점 항목 판정

### G16 #65 `check_epic_kill_time_with_hp` — P1 (실오류 아님 · 기재 누락 → 보강 4건)
IR `define … (ptr %0, ptr readonly %1, i64 %2, ptr dereferenceable(1728) %3, i64 noundef %4)` (m09.ll:52666) vs 소스 인자 4개(`!47139~!47142` arg 1~4). 승격 대응을 **role 에** 1:1 로 적었다(옛 판은 `%1`/`%2` 를 `type` 칸에 적어 게이트가 못 봤다):

| params | IR | 근거(줄 원문) |
|---|---|---|
| p[0] data | `%0` = data.context | 52668 `#dbg_value(ptr poison, !47139` · 52700 `%11 = icmp ne ptr %0, null` → 52721 `llvm.assume` · 52722 expected_damage_target 2번째 인자 `dereferenceable(64) %0` · 호출부 22353~22354 `%95 = gep ptr %4, i64 8` → `%96 = load ptr` → 22402 첫 인자 |
| p[1] champions | `%1` = buf ptr · `%2` = len | 52669 poison · 52679~52681 fragment(0,64)/(64,64) dbg_value · 52696 `%9 = icmp eq i64 %2, 0` · 호출부 22400 `%110 = load ptr, ptr %35`(+0) / 22401 `%112 = load i64`(+24) |
| p[2] epic | `%3` | 52670 `#dbg_value(ptr %3, !47141` · 52751/52788 check 3번째 인자 · 52722/52769/52805 expected_damage_target 5번째 인자 |
| p[3] hp | `%4` | 52671 `#dbg_value(i64 %4, !47142` · 유일 use 52829 `%76 = mul i64 %4, 1000` |

시뮬레이션 `paramrole.align` = `{0:['%0'], 1:['%1','%2'], 2:['%3'], 3:['%4']}` · check_spec **OK**.

### G16 #66 `evaluate_gank_opportunity_with_score` — P2 sret (sret 확정 · 행 삽입 1건)
m04.ll:62981 `define void @…(ptr dead_on_unwind noalias noundef writable writeonly sret([12 x i8]) align 4 captures(none) dereferenceable(12) %0, …, i32 noundef %5)` — 반환형 `void` + `sret([12 x i8])` ⟹ `(bool,i32,i32)` 는 **ScalarPair/ScalarTriple 이 아니라 sret 메모리 반환**. `params[0]` 에 `{"i":0,"name":"(sret)"}` 삽입(규약 정본 07·15 와 같은 형식). 레이아웃 = +0x0 `.1 evaluated_score`(63699 `store i32 %303, ptr %0`) · +0x4 `.0 ok`(63698 `store i8 %306, ptr %305`) · +0x8 `.2 actual_score`(63701 `store i32 %263, ptr %307`). 시뮬레이션 P2·P3·P6 **OK**, align `{0:%0 … 5:%5}`.
덤: 기존 `mem[16..18]` 의 store 줄 인용(63698/63695~63697/63700)이 한 줄씩 어긋나 63699/63695~63698/63701 로 정정(실오류 3).

### G20 R4 #63 `39` vs #64 `40` — 오탐 선언(값이 진짜 다르고 둘 다 맞다)
- #63: 본문 인라인 `%36 = mul i64 %35, 100` / `%37 = udiv i64 %36, %30` / **`%38 = icmp ugt i64 %37, 39`** (m09.ll:37800~37802, 598 은 37837~37839) ⟹ HP% ≥ 40.
- #64(및 #62): 리터럴 `i64 noundef 40` 을 `v23_healthy_allies_near_point`/`v23_recent_visible_enemies_near_point` 에 인자로 넘김(m15.ll:54730/54732). 콜리 `v23_healthy`(objective_helpers.rs:12): **`%26 = icmp ult i64 %25, %5`**(m15.ll:53464, 적 쪽 55705) → 미만이면 제외 ⟹ HP% ≥ 40.
- 결론: **같은 임계(HP%≥40)** 를 한쪽은 `> 39` 리터럴로, 다른 쪽은 `≥ 40` 인자로 표현한다. 소스가 `> 39` 인지 `>= 40` 인지는 표기 불가(#63 open[0] 유지). 조치 = #63 knobs[0].what 「…(인라인 리터럴 39 — `ugt 39` ≡ HP%≥40)」, #64 knobs[2].what 은 #62 와 같은 「…(min_hp_ratio)」로 갈라 R4 재발화 방지 + effect 에 등가 관계 명기. 시뮬레이션 `sharedchk` R4(담당 범위) = 0.

### G12 #66 consts[1] (값 5, src_line 713) — 실오류 → 716
강후보 {716, 715, 759} / 약후보 {716, 702}; 713 은 사슬 어디에도 없다. 리터럴 5 = m04.ll:63091 `store i64 5, ptr %49, align 8, !dbg !72398`(Filter 구조체 안의 `Range.end`), !72398 → filter.rs:28 `Filter::new` ← iterator.rs:957 ← **passive_jungle.rs:716**(체인 마지막 `.filter`). 명세가 인용한 63102 는 `store ptr %2` 줄(오기). 715 = closure$2 의 `player_champion[team][pos]` bounds check `icmp ult i64 %27, 5`(m01.ll:30837), 759 = `shl i8 %182, 5`(타워 stride, 무관). meaning 도 정정.

### G15 #63 consts[5] (값 2 「임계 아님」 NEG) — 실오류(문면 자기모순)
관측 = `CMP_ORD` 만(m09.ll:37743 `%21 = icmp ult i64 %20, 2` → panic_bounds_check) ⟹ 파생 kind 는 임계로 고정(`mkspec3` 「배열 길이 = 비교 상한은 임계로 보호」). 「(임계 아님)」을 걷고 「비교 상한이라 임계로 분류하되 게임 판정 노브가 아닌 안전검사」로. 시뮬레이션 kind=임계 · neg_hit=None.

### G10 / G7 #66 open[0] — 사실 서술로 이동(같은 항목)
`shared.is_recent_visible.blackboard_인덱스_의미`(★확정): `Blackboard[T].last_visible[pos]` = 팀 (1−T) 가 팀 T 의 pos 챔피언을 마지막으로 본 틱. 따라서 closure#0 의 `blackboard[1-team].is_recent_visible(e)`(m04.ll:66367~66370 `%22 = sub i64 1, %21`) = 「내 팀이 적 e 를 최근 봤는가」로 **의도된 비대칭**, `in_big_line` 은 자기 팀 계획판이라 `blackboard[team]`(m01.ll:30760) 이 맞다. 같은 패턴 = `v23_recent_visible_enemies_near_point`(m15.ll:55582 `%9 = sub i64 1, %8` → 55666 gep → 55742 호출). 「사실 서술」 선언 → notes 로(시뮬레이션 G7·G10 = 0).

### G18 #62 (4건) — mem 행 5건 삽입(보강)
다른 명세 규약 그대로: AbstractGameWithCache 0x0 `game.data_ptr`(m15.ll:52587 `%18 = load ptr, ptr %17`) · 0x8 `game.vtable_ptr`(52588~52589) · AbstractGame::vtable 0x40 `get_game_mode`(52590 gep 64 → 52592 call) · 0x1f0 `get_entity_by_id`(52746 gep 496 → 52748 call → 52749 null) · 0xf8 `is_visible`(52756 gep 248 → 52758 call). `at`=1~5 로 `reads` 안(dry 출력 확인, `writes` 로 안 샘). 시뮬레이션 G18 = 0.

## 2. 그 밖에 고친 것(담당 함수 표본 대조)
- **#62 `logic` IR 인용 7곳 + params[2] 1곳 전부 어긋남** → 정정(실오류 8). 예: 최종 판정 52741~52743 → 실제 52774~52776 `%89 = icmp ne i64 %88, 0` / `%90 = icmp samesign uge i64 %88, %87` / `%91 = select …`; switch 52549~52552 → 52555~52557; is_visible 52716~52720 → 52756~52758; hp<max 52723~52728 → 52762~52766; extractvalue 52694~52695 → 52739~52740. **판정식·극성 불일치 0**.
- **#65 `logic` 3곳 · consts[2]·[3] · open[4]** 인용 정정(실오류 5). 판정식 불일치 0.
- **#65 div0 패닉 경로 도달 불가** — 콜리 3종 `llvm.umax` 하한: `attack_cooltime` `range(i64 3, 0)`(_gcbc g06.ll:66510) · `skill_cooltime` ≥3(66405) · `skill2_cooltime` ≥1(66747). consts[4]·open[1]·logic 반영(보강).
- **#62/#64 콜리 `v23_*_near_point` 독해** — (a) max_hp==0 div0 패닉 (b) `HP% < min_hp_ratio` 제외(`ult`) (c) 아군 dist² ≤ range²(53497 `ule`) / 적 dist² > range² 제외(55738 `ugt`) (d) 적은 추가로 `blackboard[1-team].is_recent_visible` 참. → #62 open[2] 사실 서술화, #64 open[3] 범위 축소(잔여 `line_exists`·`is_near_line`·`v25_objective_far_split_pressure` = **미탐색** 유지), #62 knobs[1]/consts[7]·#64 knobs[2]/consts[5] 「비교 방향 미독해」 해소, 180000 행(#62 c6·#64 c4) `미상`→`임계` 낱말 보강.
- **#63 open[1]** — `!38030` 사슬 `line 1905 in iter_champions ← 597` + `!38199`(closure$0 ← Filter ← count ← 597) 로 `iter_champions(team).filter(closure$0).count()` 확정 → 사실 서술.
- **#65 open[0]** — dbg_value 로 `dpt`(!47145, 52706·52736·52778·52814) / `tick`(!47143, 52831) 역할 확정 → 사실 서술. open[4] 도 사실 서술 선언.

시뮬레이션 후 open/notes: #62 3/2 · #63 3/1 · #64 5/0 · #65 3/2 · #66 6/1.

## 3. 판정 어휘(§5-b)
- `오탐`: G20 R4(#63/#64).
- `실오류`: #66 consts[1] · #66 mem[16..18] 인용 · #63 consts[5] 문면 · #62 인용 8 · #65 인용 5 — **behavior_change 전건 false**(인용·분류 오류, 재구현 동작 무영향).
- `보강`: #65 params 4행 · #66 sret 행 · #62 mem 5행 · 콜리 독해 반영.
- `사실 서술`(→notes): #66 open[0] · #62 open[2] · #63 open[1] · #65 open[0]/[4].
- `미탐색` 유지: #64 open[3] 잔여 · #65 open[1] 잔여(expected_damage_target·CastingTarget::check) · #66 open[1](m12 fold).
- `표기 불가` 유지: 기존 판정 변경 없음.
- **미실행**: 오라클(⑥). #62/#64/#66 은 pub 이라 대상이지만 이번 배치는 게이트 잔여 정리에 한정 — 인자 조립(`OperationData`+`Blackboard`+에픽 live_list)이 3함수 공통이라 **다음 배치 몫**(미탐색이지 재료 부재 아님). ev 상향 0건인 이유.

## 4. 실행한 명령
```
python -X utf8 dossierfresh.py 18 B                              (2회 FRESH)
python -X utf8 paramrole.py 65 66 --verbose
python -X utf8 dloc.py m04.ll 72398 72397 72289 72304 72349 / m01.ll 41831 / m09.ll 38199 38030 / m15.ll 60138 61853
python -X utf8 srclinecheck.py 66 --verbose  (+ collect() 직접 호출)
python -X utf8 -c "kindchk.observe/neg_hit … (62,63,64,65 consts)"
sed/grep … _gaibc/m09.ll(52666~52833·22353~22402) · m04.ll(62981~63740·66344~66418) · m01.ll(30712~30975)
             · m15.ll(52547~52848·53341~53900·54635~54772·55548~56101) · _gcbc/g06.ll(66405·66510·66747)
python -X utf8 _verify18/B/mk18B.py            → patch.json
python -X utf8 applypatch.py 18 --only B --dry → 47/47
python -X utf8 _verify18/B/sim18B.py           → 담당 범위 G16/G12/G15/G20/G7/G10/G18 전부 0
```

## 5. 검사기 제안
- **G21 인용 대조**: `paramrole` P6(CITE + 백틱 IR 조각, F3~F7)을 `logic`·`consts.meaning`·`mem.note`·`open/notes` 문면에도 적용. 담당 5함수만으로 어긋난 인용 **16곳**(#62 8·#65 5·#66 3) — 전부 무검사 칸. 판정 = 「그 줄(범위)에 조각이 없으면 불일치, 조각 없는 인용은 보류」.
- **G20 R4 억제**: `knobs.value` 가 한쪽은 `CMP_ORD` 리터럴·다른 쪽은 `CALLARG` 로 관측되면 「표현 차이」 soft 처리(`kindchk.observe` 재사용).

## 6. 적용 후 메인이 할 일
- 삽입 6건(#66 params[0] · #62 mem 1~5) → **`--restamp`**. 같은 배열에 인덱스 경로 정정은 섞지 않았다.
- `open` 문면 정정 9건이 거부되면 §1·§2 근거를 notes/closed 로 옮기면 된다.
