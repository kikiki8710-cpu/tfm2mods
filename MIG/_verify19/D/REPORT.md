# 19차 배치 D 보고 — #98 evaluate_steal_for_target · #99 should_steal_now · #100 TeamPlan::update_steal · #101 line_backfight_support_focus

기계 적용분 = `_verify19/D/patch.json` (정정 22 · ev상향 28 · 지시오류 4 · `applypatch.py 19 --only D --dry` = 22/22 · 28/28 · 실패 0).
`dossierfresh.py 19 D` = 착수 시·제출 시 둘 다 **FRESH**(도장 20:36:18).

## 0. 실행한 것 (명령줄 그대로)
```
python -X utf8 dossierfresh.py 19 D                                   # FRESH x2
PYTHONIOENCODING=utf-8 python -X utf8 specgate.py --only {98,99,100,101}
PYTHONIOENCODING=utf-8 python -X utf8 specgate.py                     # 전량 — 담당분 grep
PYTHONIOENCODING=utf-8 python -X utf8 srclinecheck.py 101 --verbose
python -X utf8 dloc.py /c/tfm2mods/_gaibc/m04.ll 63575 63563 73342 73353 73340 73351 64079
python -X utf8 tcxdict.py TeamPlan 0x1ba / 0x1b9 / 0x418 / 0x120 · --enum StealAction / StealTarget / BigPlan / TutorialType · --deep
python -X utf8 dienum.py StealAction
(스크래치) m09.ll 24466~25498 store/memset/memcpy 전수 + gep 루트 추적 → self 쓰기 34행 대조
(스크래치) sharedchk.variant_split 에 re.split(u"[.@]") 적용 시험
sh _verify3/build.sh _verify19/D/oracle/v19D_o1.rs  →  %TEMP%\tfm2_spanprobe\v19D_o1.exe {1..6}  (케이스당 프로세스 1개, 로그 oracle/v19D_o1_case{1..6}.log)
PYTHONIOENCODING=utf-8 python -X utf8 applypatch.py 19 --only D --dry
```

## 1. §4 게이트 판정

| 게이트 | 대상 | 판정 | 근거 |
|---|---|---|---|
| **G20 R1** TeamPlan 0x1ba (99/100) | `current_steal_session@tag` vs `…@Some.0.enemy_jungler_alive_at_start` | **오탐(게이트 결함)** — 명세는 옳다 | `tcxdict TeamPlan 0x1ba` 가 **두 행을 동시에** 돌려준다(bool 필드 + `tag (Niche)`). DWARF m09.ll:75569 `!4971 DISCR_EXACT=2`, tag offset 848bit=0x6a → 0x150+0x6a. 게이트가 못 본 이유 = `sharedchk.variant_split` 이 `.` 만 구분자로 써서 `@Some` 의 variant 성분을 못 본다. **수정안 `re.split(u"[.@]", …)`** — 스크래치 시험에서 이 건 + [16] `attack_effect.range/…@Some.0.range` + [41] `big_goal[].1@Battle…/…@Some.0@Battle…` 3건이 양립으로 바뀌고, `ty.Champion.action_state@tag` vs `ty@Champion.0.action_state@tag`([13]↔[101]) 는 여전히 안 잡힌다(`.0` 첨자 — `skel` 쪽, 별도) |
| **G12** #101 consts[4]/[5]/[7]/[8]/[9] | 291/291/330/330/288 | **실오류 5/5**(줄만 틀림, behavior_change 0) | 293/293/331/331/346. IR 원문 `!dbg` 사슬 인용은 patch.json evidence. 강후보와 일치(consts[5] 만 약후보 293 = `select … !dbg !63575` 귀속) |
| **G7** #101 open[2] | `is_recent_visible … 본문 미독` | **과열림 확정** — 산문 처리 요청(§3) | `_gcbc g07.ll:157005` define 직접 확인: ①`is_visible_cell(team@+0x930, id@+0x5c0)`(vtable +0xf8) 참이면 즉시 true ②(vtable +0x150)(id) null 이면 false ③`self.last_visible[+0x1e0][pos@+0x9c0] + 120 >= tick()` |
| **G16** params `i` 규약 | 4함수 | **적합(0건)** | P3 = 소스 1-based · sret=0. #98 0~12(sret+12인자) · #99 1~5 · #100 1~6 · #101 0~4 — tcx 인자 수와 전부 일치 |
| G10/G15/G18/G19 | 4함수 | 0건 | 전량 실행에서 담당분 없음 |

도시에 §4 는 「이 배치 몫 = 1건」이라 했으나 실제는 **G12 5 + G7 1 + G20 1 = 7건**이다(§5 ①).

## 2. 무검사 축 표본 대조

### 2-a. #100 `update_steal` writes 34행 ↔ IR store 전수 — **불일치 0**
m09.ll 24466~25498 의 `store`/`memset`/`memcpy` 전부(41개)를 gep 사슬로 `%0`(self) 기준 절대 오프셋으로 환산해 mem[32]~[65] 와 대조. 34행 전부 IR 에 대응 store 가 있고, IR 에만 있는 self 쓰기는 없다(24801/24814 는 로컬 alloca 복사).
- 가변 인덱스 store 귀속: 25210 `%0+[%326]`(select 952/968 = 0x3b8/0x3c8 attempt_count) · 25285 `%0+[%360]`(phi 400/408 = 0x190/0x198 entries) · 25293 `%0+[%366]`(phi 416/424 = 0x1a0/0x1a8 ticks) · 24948 `%0+992[%209]`(0x3e0[target]).
- **Vec push 2건**(self-diff sweep 대비): ① `completed_steal_sessions`(ptr 0xf8 / len 0x100 / cap 0xf0) 원소 112B = `+0 start_tick(%146=load 0x150) · +8 end_tick(%155=tick()) · +0x10 memcpy 88B(0x160~0x1b7) · +0x68 outcome(%207 phi) · +0x69 target · +0x6a enemy_jungler_alive · +0x6b memcpy 5B(패딩)`. ② `chats`(0xc8/0xd0/0xc0) 원소 24B = `+0 태그(select 32 SerpenSteal / 41 MorgardSteal) · +1 is_commit`. 원소 내용은 힙이라 self 바이트 diff 엔 len(grow 시 cap·ptr)만 보인다 — 오라클 case3 `+f0:0→4 +f8:ptr +100:0→1`. 원소 대조는 Debug 출력(수법 ⓑ)으로.
- **None 의 payload 바이트는 undef**: tag==0 이면 0x419/0x41b 는 IR `undef`(m07.ll:55118 `%373 = phi i8 [ undef, %5 ], …` · m09.ll:24560 `select i1 %34, i8 undef, i8 %24`) — 오라클에선 255 로 나왔지만 codegen 산물. **sweep 은 tag==0 일 때 그 바이트를 마스크**(mem[34]/[36] note 보강).

### 2-b. #98/#99 니치 — DWARF 재확인 **일치**
- `Option<StealAction>` None = **255**: m09.ll:75493 `!4895 = DISCR_EXACT … extraData: i64 255`(!4849 Variant0 ∈ !4846 `enum2$<Option<enum2$<StealAction>>>`). 오라클 case1 `+418:1→255`.
- `Option<StealSession>` None = **2**: m09.ll:75569 `!4971 DISCR_EXACT=2`(!4936, tag @848bit). tcxdict 0x1ba·0x120 둘 다 `Niche`. 오라클 case3 `+1ba:1→2`.
- `StealAction` 태그 0/1/2 Direct(tcxdict·dienum 일치), `StealTarget` 0 Epic/1 Serpen.
- (정보) `TeamPlan.current_steal_session` 의 타입은 `game_core::StealSession`(ai_interface.rs).

### 2-c. `consts.kind` 표본 — 오분류 12행(전부 `meaning` 낱말로 정정, patch)
| 함수 | 행 | 현재 → 제안 | 왜 |
|---|---|---|---|
| 98 | c5 16000 | 인덱스 → 임계 | `llvm.umax` 하한(15차B #26 c8 동형) |
| 98 | c6 1(=2) | 임계 → 계수 | `shl 1` 접힌 배율 |
| 99 | c2 1(=2) · c3 5 | 태그 → 계수 | shl/mul 배율. 리터럴이 태그값과 겹쳐 CMP_EQ 로 오염 |
| 100 | c6 9 | 센티널 → 태그 | 낱말 `니치` 가 `_SENT` 에 걸림(BigPlan Battle 태그) |
| 100 | c11/c12/c13 | 임계 → 태그 | StealOutcome variant(오라클로 값 확인) |
| 100 | c18 112 · c19 24 | 미상 → 길이 | 「임계 아님」이 부정문 가드로 지워져 잔여 버킷 |
| 101 | c0 115600000000 | 산출값 → 임계 | 클로저 캡처 STORE 가 PRODUCE 로 읽힘 |
| 101 | c3 22500000001 | 미상 → 임계 | 비교가 aux 에만 있어 관측 0 |
| 101 | c8 4 | 태그 → 임계 | `icmp ult 4` 상한. 낱말 `태그` 가 체인 앞에서 선점 |

검사기 제안: ① `kindchk.observe` 가 `ir.aux` 범위도 훑게 ② `_word_kind` 의 `니치` 는 값 서술(`None 니치 = N`)일 때만 센티널, 「니치 idx+2」 같은 태그 설명은 제외 ③ `folded_from` 이 있는 행은 낱말 없이도 `계수` 기본값.
남긴 것: 99 c5/c6 288000/672000(캠프 좌표)은 어휘에 `좌표` 가 없어 `미상` 이 정직 — 어휘 확장 제안. 101 c9 40 은 루프 종료 오프셋(stride)이라 SPEC_GUIDE 상 consts 대상이 아니다(삭제는 메인 판단).

### 2-d. `logic`·`one_line`
- #100 logic 의 L733 게이트·L903/904 저장·세션 종료/Success 분기·outcome 값(1/3/0)·`prev_active = prev.tag > 0` 이 **signed** 비교(m09.ll:24548 `icmp sgt i8 %27, 0` — None=255=-1 이 false)까지 오라클과 일치.
- #101 logic 「L332: ally_targeting_enemy」는 IR 상 332(load)+333(매치) 2줄 → 「L332~333」 보강(patch).
- #98 logic 은 IR 독해 그대로(오라클 미도달, §4).

## 3. `open`/`notes` 산문 요청 (patch 로 못 쓰는 칸)
1. **#101 open[2] 분리·축소**: `is_recent_visible` 부분은 **확정**(shared, g07.ll:157005) → open 에서 내리고, 남는 물음은 「`is_near_line`(_gcbc **g09.ll:157890** define, Top/Mid/Bottom switch · 상수 192001/704000/95999/63999/64000 관측 — 본문 미독) · `distance`(g06.ll:87697 = |dx|²+|dy|² 정수 sqrt, <1000001 빠른 경로)」로 **미탐색**(위치 확정) 유지.
2. **#100 open[3]** (`epic_exists` 참 집합 {0,7,8}) → 오라클이 `TutorialType::None` 에서 에픽 스폰·참 경로(case5 EligibilityLost)를 실측 — 「None 에서 참」 확정. 항목 유지하되 `notes` 에 한 줄 추가 권장.
3. **#100 notes[0]** 「outcome select(len==0,1,3)」 — 오라클 case3(len==0 → EnemyKilled) / case5(len==1 → EligibilityLost) 로 값 실측. 문면에 「오라클 실행 확인(19차D case3/5)」을 붙이면 ev 2.
4. **#99 open[3]** (L383 비교 키 = enemy_tick) — 사실 서술이라 `notes` 로 이동 권장.

## 4. 판정 어휘 — 하지 못한 것과 그 범위
- **재료 부재(범위: default `SwordmanChampionInfo` + AttackEffect 미조립)**: #98 Lurk/Commit 경로와 #99 의 합성(L376~390)은 오라클로 못 갈랐다. case6(에픽 스폰·hp 0/1·goal_data.epic {enemy_tick 600, last_seen tick-10})에서 `should_steal_now` 가 evaluate 까지 진입하지만 `steal_damage_and_range_within` 이 0 → L459 None. 함정 ④ 그대로 — **미탐색 = `AttackEffect`(72B pub) 직접 조립 후 재실행**. 에픽 max_hp 도 1(default 세계) — 에픽 스탯 주입도 같은 미탐색.
- **미탐색**: #100 knobs[2](Battle 플랜 차단)는 `BigPlan::Battle(BattlePlan)` 페이로드 구성이 필요해 미실행(IR 근거 ev4 유지). Lurk/Commit 진입 시 entries/ticks 증가(mem[47]~[50])도 같은 이유.
- **오라클 판별력 메모**: `should_steal_now` 는 case1~6 전부 `None` — 조기 반환이 다 None 이라 외부에서 어느 줄인지 갈리지 않는다. #99 의 ev 를 내리려면 evaluate 가 Lurk/Commit 을 내는 세계가 먼저 필요하다.

## 5. 내 지시(도시에)의 오류 (patch.json `brief_errors` 와 동일)
1. §4 「이 배치 몫 = 1건」— 실제 specgate 는 #101 G12 5건·G7 1건을 더 낸다(mkdossier 가 G20 만 추림).
2. 초점의 「G7 shared 제거」가 도시에 본문에 없다 — 대상은 #101 open[2] 였다.
3. §1 `ev≥4(미실행)` 수치에 mem(상한 3)이 섞여 「오라클로 내려라」가 닿지 않는 행이 포함된다.
4. TEMPLATE.rs 함정 ⑦ 3줄이 `#![allow]` 위(파일 1~3행)라 `//!` 문서 밖(사소).

## 6. 산출물
- `_verify19/D/patch.json` · `_verify19/D/oracle/v19D_o1.rs` · `oracle/v19D_o1_case{1..6}.log`
- 오라클 요약: case1 diff 1B(`+418:1→255`) · case2 4B(`+418:1→0 +419:1→255 +41a:2→0 +41b:1→255`) · case3 16B(세션 종료 EnemyKilled, Vec grow cap 4/len 1, `+3e0`=1000, `+128:77→0`) · case4 17B(Success, `+3c0:0→1`, `+120:0→2`) · case5 16B(EligibilityLost, 에픽 live_list=[41] 2틱 스폰) · case6 4B(prev None→Some(None)).
