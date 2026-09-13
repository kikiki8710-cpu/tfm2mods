# 19차 배치 C 보고 — specs[91]~[97] (게임 0.5.8)

기계 적용분 = `patch.json`(정정 27 · ev상향 7 · 브리핑오류 4, `applypatch.py 19 --only C --dry` 27/27·7/7 통과).
스크래치(줄번호 부착 IR 주석본) = `%TEMP%\claude\…\scratchpad\v19C\{91_pj,92_v46,93_giveup,94_hunt,95_setup,95_closure,96_passive,97_subplan,81_epic_hunt}.ll` · 오라클 = `oracle/v19C_o1.rs`·`.tsv`.

## 0. 실행한 것 (명령줄)
```
python -X utf8 dossierfresh.py 19 C                 # FRESH (착수·제출 전 2회)
python -X utf8 irann.py … (변형 irann_ln.py: 원문 줄번호 부착) ← m04/m05/m09/m10.ll 의 7함수 + #81 + #95 closure(aux 9908~10013)
python -X utf8 dloc.py C:\tfm2mods\_gaibc\m10.ll 22177   # #97 heal_area 리터럴의 inlinedAt 사슬
python -X utf8 tcxdict.py GoalData|MobaMode|Strategy|AbstractGameWithCache <off> · --enum EarlySerpenStrategy
sh _verify3/build.sh C:/tfm2mods/MIG/_verify19/C/oracle/v19C_o1.rs ; v19C_o1.exe {0..4}   # 케이스당 프로세스 1개
PYTHONIOENCODING=utf-8 python -X utf8 applypatch.py 19 --only C --dry
```

## 1. §4 게이트 판정
| 게이트 | 대상 | 판정 | 근거(원문 줄) |
|---|---|---|---|
| G16 | #93·#94·#95 params.i | **실오류**(20행) | 세 함수 모두 sret 없음(`define … (i64 %0=version …)` m05.ll:49223/45035/47634) → 소스 인자 1..n. 0..n-1 → +1 |
| G12 | #97 consts[9]·[10] src_line 103 | **실오류** → 102 | m10.ll:10171/10189 `select i1 %151, i64 64000, i64 960000` !dbg !22177 = game.rs:319 heal_area ← hunt_and_poke.rs:**102** ← :79. :103 에는 891999/896000 비교만 |
| G18 | #95 `strategy.early_serpen_top` | **오탐**(문면만 주석화) | 본문 Strategy 읽기 = m05.ll:49087 `gep ptr %9, i64 19`(+0x13 early_serpen) 하나. +0x14 는 콜리 is_skip_serpen define(44823~) 안 44903 `gep ptr %5, i64 20`. mem 행 추가는 없는 로드 주장 |
| G19 | #91 knobs[0]=21 | **오탐**(명세 정확) | m04.ll:29351 `%334 = icmp ult i64 %265, 21`(!dbg 없음 · 29130 `br i1 %205(can_heal), label %333, label %267 !dbg L206` 의 타깃 블록) · 29137 `icmp ult i64 %265, 41`. 창의 「266」= 블록 라벨 `266:`(29132) |

### 게이트 개선 제안(다음 라운드용)
- **knobval(G19)**: ①`_DROP` 에 줄머리 블록 라벨 `^\s*\d+:` 추가(266 오탐) ②앵커 줄이 `br`/`switch` 면 **타깃 블록의 첫 4명령**을 창에 포함(점프스레딩으로 `!dbg` 가 떨어진 비교식 구제).
- **xreflogic(G18)**: P2 구제 ⑤를 `//` 만이 아니라 「`(… 요지 …)` 괄호 서술」·「콜리 이름 뒤 `내부`」에도 적용하거나, `callees[]` 에 있는 함수 이름이 같은 문장에 있으면 콜리 내부로 간주.
- **mkspec3 `_symtoks`**: v0 망글 백참조 `B<base62>_` 를 건너뛰어야 한다(아래 §3-①).

## 2. 무검사 축 — serpen 4 ↔ epic 동형 교차대조 (전부 IR 원문으로 대조)
| 항목 | serpen(#93~#97) | epic(#50/#53/#81/#82) | 판정 |
|---|---|---|---|
| serpen_exists / morgard_exists | tutorial∈{0,5,7,8} 통과(switch m05.ll:49241) | (tag-1)<u6 → 없음 | 둘 다 IR 일치(대칭 아님 — MidBottom(5)은 세르펜만 존재) |
| blackboard 인덱스 | is_recent_visible self=[1-team](49844/50625 gep …, i64 %99) · in_battle [team] | 동일 | 대칭 ✓ |
| GoalData | serpen.epic_ally_tick **0xd0**(#94 47001 `gep %4, 208`) · epic_enemy_tick **0xc0**(#97 10138 `gep %6, 192`) | epic 0x98/0x88 | tcxdict 확정 ✓ |
| JungleRunner live_list | serpen 0x1d0/0x1d8 · next_respawn 0x1e0 | epic 0x1a0/0x1a8/0x1b0 | tcxdict ✓ |
| 노이즈 시드 솔트 | 94 (#94 47561 `or disjoint i64 %994, 94`) | 233 | 대칭적 상수(함수별 솔트) ✓ |
| 캠프 반경 | hunt 120000²(14400000001) · setup/passive 150000·320000² | 동일 | ✓ |
| 사이드 판정 | is_bottom_side = (height−y ≤ x) ‖ near_mid (#93 49500 `icmp ugt ry, x` → near_mid 검사) | is_top_side = (x ≤ height−y) ‖ near_mid (#81 55700 `icmp ult ry, x` → near_mid 검사) | ★**#81 logic 문면이 틀렸다** — 아래 §3-② |
| giveup 구조 | serpen_count 우세 분기(L263)·is_line_phase 슬롯(1..5) 분기 | 생존 머릿수·캠프 근접·미시야 도달 | **동형이 아니다**(#93 명세대로 IR 확인) |
| hunt 최종식 | `A > B+pen → (is_morgard_giveup ‖ C+bonus ≥ D+pen)`(47612~47624) | `(A>2‖D==0) && pen+B<A && bonus+C ≥ pen+D` | 동형 아님, 각자 IR 일치 ✓ |
| passive Flexible | `near_enemy < 3`(52559) → 320000²·is_enemy_side(52642 `xor i1 %107, %339`) → bottom/mid_lead(0x21e0/0x21d0)<3 | `<2`/`≤2` 2단 + top/mid_lead | 대칭(라인만 다름) ✓ · #96 remain 검사는 Flexible 뒤(52829)만, Gather 는 곧장 L496 ✓ |
| sub_plan 차이 5곳 | objective 태그 1 · JungleType 5 · fallback_line Top(0) · vt+0xe0 jungle_runner 재조회+dist<150000² · Poke 무게이트 | — | m10.ll 10110/11377/11384/10510~10523/11405 로 전부 확인 ✓ |

⟹ 담당 7함수 `logic`·`mem`·`consts`·`knobs` 를 IR 원문과 전수 대조: **불일치 0**(위 게이트 4건 외). 「불일치 0 도 결과」로 기록한다.

## 3. 내 지시 밖에서 나온 것 (메인이 처리할 것)
① **#97 `sig.tcx`/`sig.path` 오귀속** — `&mut EpicHuntAndPokePlan` / `…EpicHuntAndPokePlan::sub_plan` 으로 적혀 있다. 원인 = `mkspec3._symtoks` 가 `…hunt_and_pokeNtB2_21SerpenHuntAndPokePlan8sub_plan` 에서 `B2_` 백참조를 `2`+`_2` 식별자로 오독 → 토큰 `['plan_legacy','old','serpen','hunt_and_poke','_2','S','sub_plan']`(재현: 위 함수를 그대로 돌림). 타입 토큰이 없어 `hit` 동점 → basename `hunt_and_poke.rs` 가 epic/serpen 둘 다 맞고 첫 후보(epic)가 뽑혔다. 형제표 5행이 정답. **파생 필드라 patch 불가** — `_symtoks` 루프에서 `sym[i]=='B'` 이고 `B[0-9a-zA-Z]*_` 에 맞으면 그 길이만큼 건너뛰면 된다. 같은 형태(`NtB2_…`)는 impl 메서드 전부에 있으나 basename 이 유일하면 `mine` 필터가 구제한다 — #91 은 그래서 무사.
② **#81 check_epic_hunt `logic` L198~199 극성 오류(다른 배치 소관, behavior_change=true)** — 문면 `(!is_top_side … [height−y < x] ‖ is_near_mid_line)` 인데 IR m09.ll:55700 `%548 = icmp ult i64 %547(ry), %543(x)` → 참(=!is_top_side)일 때만 55704 is_near_mid_line 검사, 거짓(=is_top_side)이면 곧장 카운트. 즉 **`is_top_side ‖ is_near_mid_line`**(#53 notes[0]·#94 L320 의 serpen 대칭형과 일치). 이대로 재구현하면 Morgard 근처 아군을 바텀 사이드에서 센다.
③ #95·#96 모두 `is_skip_serpen(version=poison …)` — is_skip_serpen define 본문에 %0 사용 0건(보강으로 role 에 적음).

## 4. 오라클 (v19C_o1.tsv · 케이스당 프로세스 1개 · setting_ok true · towers 16/2/2)
| 케이스 | 셋업 | 결과 | 닿은 경로 |
|---|---|---|---|
| 0 giveup | tick 0 · serpen live_list.len 0 | 10/10 true | #93 L241 |
| 1 hunt | 양팀 5명·캠프 근처 0 | 10/10 false | #94 L361 false(A=0 ≯ 0+pen) |
| 2 setup | Jungle~Support · remain 0 | 8/8 false | #95 L81 통과 후 판정식 false(값만 기록) |
| 3 passive | phase 0/1/2/3 | None·Assemble → -1 8/8 · Hunt → 14 4/4 · Setup → 14 4/4 | #96 L418/L427/L503/L496 |
| 4 sub_plan | default plan · 세르펜 없음 | 10/10 tag 13 SerpenCheck{false} | #97 L48~50 |
미실행: #91(PassiveJunglePlan 생성자 미조사)·#92(`in:game_ai`, 루트 재수출 아님). **재료 부재가 아니라 미탐색**.

## 5. open 정리(산문 — 메인이 넣는다)
- #94 open[4] `_plan(&BigPlan) readonly 미접근` → **사실 서술**(IR 45035 define 에 `%5` 는 본문 접근 0건 — 이번 IR 재독으로 재확인). notes 로.
- #95 open[5](`!= 0` vs `> 0` usize) · #93 open[2] · #94 open[3] · #96 open[3] → 「표기 불가」 유지(외연 동일).
- #97 open[0](Epic 판 차이 5곳) → 전부 IR 로 확정(§2 마지막 행) → **사실 서술** 로 이동 권장.
