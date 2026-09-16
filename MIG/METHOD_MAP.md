# 분석 방법 지도 — 무엇을 알고 싶을 때 무엇을 집는가

> 2026-09-11 작성. 게임 0.5.8 / SDK `sdk_058` / toolchain `nightly-2026-05-24`.
> **이 문서는 라우터다.** 각 방법의 실행 상세는 아래 링크로 간다.

## 문서 4종 세트에서 이 문서의 자리

| 문서 | 누가 읽나 | 무엇 |
|---|---|---|
| **`METHOD_MAP.md`(이 문서)** | **무엇부터 할지 정하는 사람** | 상황 → 방법 라우팅, 각 방법의 한계 |
| `IR_TOOLKIT.md` | 재료를 꺼내는 사람 | 추출·도구 사용법 (§1~§7) |
| `SPEC_GUIDE.md` | 명세를 쓰는 에이전트 | JSON 규격, 함정, 자체검사 |
| `SPEC_RUNBOOK.md` | 배치를 굴리는 메인 세션 | 라운드 수·수렴 판정·병합·저장 |
| ★`REPORT\tfm2_judge_verify\00_상태원장.md` | **이어받는 세션(맨 먼저)** | 20함수별 **현재 상태**(RVA·bit·ev1·기구·잔여) + 정본 위치 지도 — 2026-09-13 신설 |
| `REPORT\tfm2_judge_verify\04_분석방법_정리.md` | 파이프라인 전체를 보는 사람 | **v2(09-14 · r7~r12 91함수 반영)** 이해(명세)·재현(rlib)·검증(ev1) 3축 · 정적 S0~S7 · 런타임 R0~R6 · ev1 기구 결정트리 §4-A · **DIFF≠0 트리아지 §4-B** · 착수 절차 §8 · gensweep 키워드 사전 §9 |
| `MIG\whatsdone.py <키워드|#NN>` | 착수 전 누구나 | §7 「전에 했나」 grep 을 기록처 8곳에 한 번에 |

---

# 0. ★30초 라우팅 표

| 알고 싶은 것 | 1순위 | 2순위 | 그래도 안 되면 |
|---|---|---|---|
| 구조체 **오프셋 / 필드 이름** | **`tcxdict`** ⑦ | `distruct` ②(2차 폴백) | 런타임 훅 ⑧ |
| 열거형 **태그(니치 포함)** | **`tcxdict --enum`** ⑦ | `dienum` ② | 오라클 `o_disc` ⑥ |
| 함수가 **무엇을 판정하나** | **`_docs` grep** ④ → **IR 본문** ① | `_gcbc`/`_gvbc` 폴백 ① | Ghidra ⑧ |
| 함수의 **소스 파일:줄:칸** | **`_tcx\*.json` 의 `sp`** ⑦ | rustc 진단 프로브 ⑤ | — |
| **한 줄 안 표현식 순서** (`A \|\| B`) | MIR span ⑦ (**`mir=1` 일 때만**) | — | **재료 부재로 닫아라** |
| **빈 블록·주석 줄**의 정체 | **`rmeta_srcmap`** ③ (줄 길이 산술) | — | exe 디스어셈 ⑧ |
| ★**`mir=0` 함수의 소스 한 줄 내용** | **줄 길이 산술**(③+⑦+① 결합, `IR_TOOLKIT §8`) | — | exe 디스어셈 ⑧ |
| **`pub` 함수의 정확한 동작** | ★**오라클 진리표** ⑥ | MIR ⑦ | IR ① |
| **코드값 → 의미** 표 | **`fieldall2`** + store 지점 phi | `_docs` 주석 ④ | — |
| **vtable 슬롯** 이름 | `divtable` | `_gcbc` 정적 vtable 전역 ① | 런타임 덤프 ⑧ |
| **우리 문서가 맞나** | **`tcxaudit`** ⑦ | `speccmp`(명세 2판 대조) | — |
| **내 재현이 맞나** | **런타임 훅 DIFF=0** ⑧ | 오라클 ⑥ | — |
| **명세가 완결됐나** | **`specgate`**(G1~G13) | `qcspec`(지어낸 것) | — |
| **지난 라운드 정정이 살아있나** | **`auditrounds`** | `applypatch <N> --dry` | — |
| **배치에 뭘 넘기나** | ★**`mkdossier <N>`**(정본 무손실 조립 · `--axis` 축모드 · 7차~) · 신선도 `dossierfresh` · ~~`mkbrief`~~=STALE(요약) | `SPEC_RUNBOOK §S5-d·e` | — |
| **배치 결과를 어떻게 받나** | **`mkpatch`**(배치가 씀) → **`applypatch`**(내가 붙임) | — | — |
| **정본이 지저분하다** | **`cleanspec --dry`** | — | — |
| ★**어떤 축이 아직 검사 안 되나** | **`SPEC_RUNBOOK §S5-c`** 표 | `specgate --gate G12/G13` | — |
| ★**분기 도달 가능성 / 사장(NA) 코드**(큰 함수 서브트리) | **`reach.py`**(한 함수 CFG 상수 접기 · `--version 2 --gamemode 0`) → **`reach_tree.py`**(제약/무제약 BFS 차집합 · `_next\reach\tree_<rva>.json`) ① | `irann.py` 손 독해 ① | 런타임 1단계 프로브 발화 0 ⑧ |
| ★**미명명 exe 함수의 이름**(dllmatch 미연결·오매칭 의심) | **`namebyline.py`**(패닉 Location 줄지문) + **`namebycaller.py`**(호출자 exe 콜리↔IR 콜리 차집합) ①⑦ — 바이트↔IR 줄수 비까지 세 축 일치 | `dllmatch.py`(jaccard ≥0.9 만) | Ghidra 진입부 대조 ⑧ |
| ★**exe 함수 하나의 정체 판정(Ghidra 없이)**(「이 RVA 가 무슨 함수냐」 · 지도 주소 의심) | **`fnprobe.py <RVA>`**(capstone 프로파일: 크기·call·상수·Location) ↔ **`irprobe.py <define>`**(IR 짝: 인자·call 집계·줄 집합) + **`locfind.py <파일:줄>`**(패닉 Location → IR define 역추적 · `@anon.<hash>.N` 대응) ①⑧ — 09-13 Ghidra 다운 중 6/6 판정·재확인 6/6 유지 | `namebyline.py`/`namebycaller.py`(위 행) | Ghidra 디컴+xref ⑧ — ⚠**「인라인 여부」만은 xref 로**(주소 기각 ≠ 인라인 · ANA 감사도구 §22-85) |

| ★**ev1 DIFF≠0 의 정체**(런타임 대조가 갈릴 때) | **`04_분석방법_정리.md §4-B` 트리아지 표**(undef 페이로드 · 패딩 · 계측 카운터 · Vec 삼중항 · 죽은 슬롯 · extra) + `sweep20.txt` 첫 DIFF 덤프·갈림 오프셋 히스토그램 ⑧ | `tcxdict --deep/--enum` ⑦ | 재현 오류 의심은 표 전부 기각한 뒤 |
| ★**튜플/구조체 원소의 패딩·레이아웃**(ELEM_LIVE 등록) | **IR `getelementptr inbounds nuw { … }` 타입**(그 Vec 의 push 사이트 grep) ① — 소형 필드가 앞일 수 있다 | `tcxdict <타입> --deep`(구조체만 · 튜플은 안 준다) ⑦ | 런타임 갈림 오프셋 ⑧ |
| ★**exe 인자 배치가 IR define 과 같나**(internal · sweep 편입 전) | **`argscan.py 0x<RVA> [--caller]`**(진입 스택 슬롯 전수) ⑧ | `abiagree.py` ① | ghidra-re 대응표 — ArgumentPromotion 이면 `EXE_ABI_UNRECOVERABLE`(간접 검증) |
| ★**`&mut self` 함수의 힙 쓰기 표면**(HEAP_SUBST 명세) | **`heapsurf.py <rva> --depth 5`**(전이적 · 루트 절대 오프셋) ① + `tcxdict --deep`(중첩 Vec·Option<Vec> 니치) ⑦ | IR drop_in_place/grow_one grep ① | ⚠String 필드는 치환 금지(03 §37) |
| ★**거대 함수(≥5k IR줄) 명세 분책** | 루트 줄 지도(`!dbg`→`inlinedAt` 최상위 · 소스 줄별 IR 수 · `_next\upd_blocks.json` 방식) → 배치별 소스 줄 범위 → **`mergespec.py`** 합본 | `dloc.py`/`inlsites.py` ① | — |
| ★**exe RVA 목록 → IR 실명 일괄** | **`rvaname.py`**(패닉 Location 줄지문 다중 일치) ①⑧ | `namebyline`/`namebycaller` | ghidra-re |
| ★**누가 이 함수를 부르나 — 간접(vtable/JT/fn-ptr) 포함**(정적 깊이 0 노드 · 「아무도 안 부름」 의심) | **`probe20.py --map-all --out …\probe20_tbl.rs`**(1순위 · 지도 전 노드 AUX 진입부 프로브 · idx u16 · probe.rs CAP 1024/CS_SLOT0 1000 · ⚠명세만 프로브하면 지도 411 노드 빈칸) → 빌드 → **RETREC 판**(`tfm2_judge_verify\src\probe.rs` 진입부 프로브 스텁 리턴 주소 히스토그램 · 게이트 0x0 1판 · 슬롯당 16칸+overflow) → **`retedges.py <probe20.txt> --tag 판N`**(리턴 rva→.pdata 감싸는 함수=호출자 · `_next\retedges_run8.json`) → **`fnmap_ret.py`**(지도 im/ic/ie 열·초록 선) ⑧ — 09-16 ~~판 7(명세 247 만): 새 간접 22·지도 밖 77·깊이0 129중 39~~→**판 8(653/653): 새 간접 27·지도 밖 호출자 172·깊이0 129중 108** · 실측 수치엔 분모(프로브 노드 수) 병기 · **여러 판 합본 = `retmerge.py`**(retedges_runN.json 들 → 판별 존재 표 · fnmap_ret 이 그대로 읽음 · 판 8+9 = 지도 v22) · 두 판 발화수 대조 = `cmp_runs.py`(\|Δ\|>10% 목록) · ⚠**리플레이를 바꿔도 표본은 안 바뀐다**(09-16 낮 실측: 다른 리플레이·리플레이 없음 모두 ±2% 동일 = 카운터가 재는 것은 로드 직후 배경 리그 sim · 표본 축 = 세이브 상태 · `03 §45`) | `callsites.py`(직접 `call rel32` 만 · **간접은 안 잡힘**) · `namebycaller.py` | Ghidra xref(⚠.pdata 오탐) — ⚠한 판 = 하한 · tail-jmp 체인은 못 봄(동일 횟수 묶음=체인) · 손 인코딩 스텁은 `retrec_asm.py` capstone 되읽기 · `REPORT\tfm2_judge_verify\03_시행착오.md §44` |

★**도구가 157개다(09-14 mktools). 이름을 외우지 말고 이 표와 `TOOLS.md` 를 봐라.**
`TOOLS.md` = *무엇이 있는가*(자동 생성, 분류별) / 이 표 = *어떤 상황에 무엇을 집는가*.
⚠도구를 만들면 **첫 docstring 줄을 쓰고 `python -X utf8 mktools.py`** 를 돌려라 —
안 하면 다음 세션이 그 도구를 못 찾고 **같은 걸 다시 만든다**(6차에 네 배치가 `mkpatch.py` 를 각자 만들었다).

---

# 1. 재료 8종 — 각각 무엇이고 언제 쓰나

## ① SDK rlib → LLVM IR  ★주력
**무엇**: SDK rlib 이 `-C embed-bitcode` 로 빌드돼 **아카이브 멤버 자체가 날 LLVM 비트코드**다. `llvm-dis` 만 돌리면 **디버그정보가 붙은 원본 IR**(심볼·필드명·소스 파일·줄번호)이 통째로 나온다.
**어떻게**: `python rlib2ll.py <크레이트>` → `_gaibc`(game_ai 303MB) / `_gcbc`(game_core 992MB) / `_gvbc`(game_view 1,270MB)
**언제**: **판정 로직을 읽는 기본 재료.** 이게 1순위다.
**★규칙 2개**
- `_gaibc` 에 `declare` 만 있으면 **`_gcbc`/`_gvbc` 에서 `define` 을 찾아라.** (1~6차에서 17건이 "본문 없음"으로 포기됐는데 전부 `_gcbc` 에 있었다.)
- `!dbg` 가 없어 보여도 **`inlinedAt` 루트까지 타라**(`dloc.py`).
**한계**: **`column:` 이 전 모듈 0** ⟹ 한 줄 안의 표현식 순서 복원 불가. 상수 접힘. 빈 블록은 명령이 없어 `!DILocation` 자체가 없다.
→ `IR_TOOLKIT.md §1~§3`

## ② DWARF 역산 사전 (`distruct` / `dienum`)  ⚠2차 폴백으로 강등
**무엇**: IR 의 DWARF 에서 구조체 26,416 / 필드 75,602 / 열거형 7,016 을 역산한 사전.
**언제**: **`tcxdict` 에 없을 때만.** 2026-09-11 교차검증 결과:
- `dienum` = **모순 0**(니치 공식 적용 시 239/239 정합)
- `distruct` = **산술은 건전한데 키가 leaf 이름 1개뿐이라 동명끼리 조용히 덮어쓴다**(대조 1,533건 중 139건이 동명 다중)
**남긴 이유**: DWARF 는 **exe/IR 쪽 독립 증거원**이라 SDK rlib 밖(지역변수 전용 모노모피 인스턴스, `vtable_type$`)을 볼 수 있다.
**★지뢰**: `ViewEffect`/`RangeEffect`/`CasterViewEffect` 는 독립 struct(48·96·24B)와 `DataEffectDef` variant(288B)가 동명인데 사전엔 variant 만 있다 ⟹ **전 필드 +8 밀림.**

## ③ rmeta SourceMap (`rmeta_srcmap.py`)
**무엇**: rmeta 의 `SourceFile` 테이블을 디코드해 **줄별 바이트/문자 길이·오프셋·한글 개수**를 얻는다. game_ai 80 / game_core 319 / game_view 193 파일, 파싱 에러 0.
**언제**: **소스 본문이 없는데 그 줄이 무엇인지 좁혀야 할 때.**
**★실전 위력 — "줄 길이 산술"**
- `SubPlan::merge` 4 arm 배정: 줄 길이 85/81/79/83 ↔ 이름 길이 11/9/8/10 에 대해 `줄길이 − 2×len(name) = 63` 이 네 줄 모두 일치 ⟹ **arm 확정**
- `chat.rs:20` 이 **13자** ⟹ `return self.handle_chat_inner(..)` 는 절대 안 들어감 ⟹ **맨몸 `return;` 확정**
- `epic.rs` "디버그정보 없는 구간"의 정체가 **주석·빈 줄·닫는 괄호**임을 한글 문자수로 분류
**한계**: ★**소스 원문은 rmeta 에 없다**(hash 32B 만, `span_to_snippet` = `SourceNotAvailable`). 길이·좌표만.

## ④ rmeta 개발자 주석 (`rmetadocs.py`)
**무엇**: rlib 의 `lib.rmeta` 에 **개발자가 쓴 한국어 문서화 주석 2,925개(132,551자)** 가 들어 있다. 7초에 추출.
**언제**: ★**IR 을 읽기 전에 무조건 먼저 grep 하라.** 우리가 역추적하려던 코드표가 그대로 적혀 있는 경우가 많다.
**실적**: `ff_call_*` 8칸 순서, `mf_swap` 이 관측 전용이라는 사실, `blackboard[T]` 의미가 전부 주석으로 교차검증됐다.
**⚠주의**: **주석은 stale 일 수 있다. IR 과 충돌하면 IR 이 정본.** (실사례: `ff_battle_exit.__1` 은 주석대로면 `exit_sub` 인데 IR 은 `entry_src`.)

## ⑤ rustc 진단 프로브 (`spanprobe.ps1`)
**무엇**: rmeta 를 링크한 프로브 크레이트를 컴파일하면 **rustc 가 의존 크레이트의 span 을 그대로 찍어낸다.**
- 인자 개수를 틀리게 호출 → `note: defined here --> …:LINE:COL` (**컬럼 0-based**)
- `-Z dump-mir` + 크로스크레이트 MIR 인라인 강제 → 본문 내부 span (**컬럼 1-based, hi exclusive**)
**언제**: `_tcx` 덤프가 없던 시절의 우회로. 지금은 **⑦이 상위 호환**이지만, 가볍게 한 건만 확인할 때는 여전히 빠르다.
**⚠**: 두 경로의 컬럼 기준이 달라 **섞으면 1칸 어긋난다.**

## ⑥ SDK 실행 오라클 (`-C lto=fat`)  ★★확장성 최대

> ★**09-14(22차 C·D) 범위 반전**: 「pub 만 직접 진입」은 과잉이었다. `define hidden`(비-`internal`) 함수는 프로브에서 `extern "Rust" { #[link_name = "<망글 심볼>"] fn f(…) -> T; }` 로 **직접 링크·실행**된다(124·126·128·130~133 실측 · 케이스 ~100 MATCH). 조건 ①프로브가 `game_ai` pub 항목을 하나라도 참조해야 rlib 이 링크된다(아니면 LNK2019 — `if args().count()>99 { let _ = game_ai::<pub fn> as *const (); }` 한 줄) ②`internal fastcc` 만 불가(IR 로만) ③`&Entity` 필드 세팅은 `std::ptr::write_volatile`(캐스팅 store 유실). 원문 = `REPORT\tfm2_judge_verify\RE\2026-09-14_22차_반증검증_r13_…` §0.
**무엇**: SDK rlib 은 멤버가 날 비트코드라 그냥은 exe 로 안 링크된다(`LNK1136`). **`-C lto=fat -C codegen-units=1` 을 붙이면 rustc 가 비트코드를 병합해 exe 가 나오고, 게임 SDK 함수가 우리 프로세스 안에서 진짜 실행된다.**
**언제**: **대상이 `pub` 이면 무조건 이걸 먼저 고려하라.** 전수 진리표로 IR 독해를 *실행 결과*로 검증할 수 있다.
**실적**: `rule_scope` pub 13개 진리표 → IR 표 (A)~(E) **틀린 칸 0**. 덤으로 `valid_lines`·`fallback_line`·`steal_*`·`main/sub_objective_allowed` 6개가 **새로 완전 규정**.
**인자 만들기**: `GameContext`(64B)는 **전 필드 pub** + `MapDef::moba` pub 이라 **정상 생성**된다(UB 0). 제로버퍼 캐스팅은 최후수단이고, 쓰려면 ①그 함수가 그 필드 외엔 안 읽는지 IR 로 확인 ②열거형에 0 이 유효 판별자인지 `tcxdict --enum` 으로 확인.
**★한계 3가지**
1. **`pub` 인 것만.** ⚠**정정(2026-09-11 4차 검증배치 D) — ~~`fight_check`·`path_finder` 는 `pub(crate)` 라 막힌다~~ 는 과잉이었다.** 모듈 가시성(`in:game_ai`)이 곧 차단은 아니다 — **크레이트 루트에서 재수출된 항목은 `pub`** 이고 직접 호출·실행된다. 실증(전부 프로브 실행 성공): `game_ai::check_kill_die_tick`(fight_check.rs:917) · `game_ai::is_enemy_well_danger`(path_finder.rs:1032) · `game_ai::defensive_crisis` · `effect_cc_time` · `check_favorable_engage_formation`.
   ⚠**정정(2026-09-11 5차 배치D, rustc E0425)**: ~~`game_ai::is_ignored_well_enemy`~~ 는 **그 경로로 존재하지 않는다.** 실제는 **`game_ai::plan_legacy::old::is_ignored_well_enemy`**(fight_model.rs:754) — `v=pub` 이지만 **루트 재수출이 아니다.** 4차 인용은 정확했고 내가 이 표로 승격하면서 모듈 경로를 잘랐다.
   ⟹ ★**교훈: 「막힌다」를 고칠 때 「전부 루트에 있다」로 넘어가지 마라.** 과소 주장을 정정하면서 과대 주장을 만든 실례다. `pub` 여부와 **루트 재수출 여부는 별개**다.
   ⟹ **판정 순서: 모듈 경로로 단정하지 말고 `_tcx\game_ai.json` 의 `v`(가시성)와 `p`(전체 경로)를 **둘 다** 조회하라. 호출은 `p` 그대로 쓴다.** 남는 진짜 차단은 `v` 가 실제로 `pub(crate)` 인 것 + private 필드뿐이고, **private 필드도 `transmute` 로 읽힌다**(4차 배치B 가 07 `Hide` 경로를 이렇게 열었다). `position_eval`·`score_parameter`·`buff_value` 는 이번에 확인하지 않았다(범위 명시).
2. ⚠**정정(2026-09-11 검증배치 D) — 이 한계는 거짓이었다.** ~~"인자에 `&OperationData`·`&PlayerState` 가 들어가면 pub 생성자가 없어 구성 불가"~~ → **정상 생성된다**(컴파일·링크·실행 성공, 크래시·UB 0):
   `Game::new(seed,bool,&GameSetting,&MapSetting,&MapDef)` → `game.add_player(GamePlayer::new(…, Arc::new(SwordmanChampionInfo::default()), …))` → `game.start_game(&mut StdRng,&ctx)` → `AbstractGameWithCache::new(&game as &dyn AbstractGame,&ctx)` → `OperationData::new(&cache,&ctx,&[Blackboard::default();2])` — **전부 pub**. `&PlayerState` = `game.get_player_by_position(team,pos)`(start_game 후 10/10 Some), `&Entity` = `cache.player_champion[t][p]`.
   ⟹ **`pub(crate)` 가 아닌 game_ai 판단함수는 사실상 전부 오라클 대상**이다. 실증: `best_jungle_goal` **10/10 MATCH**.
   ★남는 진짜 한계는 **인자 구성이 아니라 데이터**다: `GameSetting::default()` 의 **`tick_per_second = 0`**(실전 60) — `udiv by tps` 가 있는 함수는 **손으로 60 을 세팅**해야 한다. `SwordmanChampionInfo::default()` 는 **이펙트가 비어** 있어 `max_range_nearly_can_use` 가 전 구간 0 — 함수가 틀린 게 아니라 **입력에 판별력이 없다**. ~~실전 챔피언 데이터 로딩은 미탐색~~ → **해소(2026-09-11 4차 배치A)**: 실전 `ChampionInfo` **61종이 pub** 이고 `Action::effect()` **193개가 pub** 이라 그대로 쓸 수 있다. ⚠단 **실전 것도 액션 파라미터가 0** 이어서 `expected_damage_target` 이 **26/26 전부 0**(조기반환)이었다 ⟹ 판별력을 얻으려면 **`AttackEffect`(72B, 전 필드 pub)를 직접 조립**해야 한다(`Effect` 도 전 필드 pub). 정본 = `_verify3\TEMPLATE.rs` 함정 ④.
   ★한계 추가 — **TLS 메모 함수는 한 프로세스에서 반복 측정하면 안 된다**(4차 배치D). `check_kill_die_tick` 은 캐시 키에 엔티티 id 만 있어 hp·스탯을 바꿔도 **첫 값이 재생**된다 ⟹ 3차의 「9축 전부 무관」이 그 아티팩트였다. **케이스당 프로세스 1개.** 정본 = `TEMPLATE.rs` 함정 ③.
3. ★**외연이 같은 두 표현은 실행으로 절대 안 갈린다.** (`spawn_epic` vs `spawn_epic && line_exists`, `==5` vs `>=5`) — **이건 IR 로만.**
**⚠**: 모드 코드에 이 경로를 넣으면 **§3 완전재구현 원칙 위반**이다. 검증 전용.
→ `IR_TOOLKIT.md §7`

## ⑦ rustc-dev `tcx` 덤프 (`tcxdump`/`tcxtg` → `tcxdict`/`tcxaudit`)  ★타입의 정본
**무엇**: `rustc_private` 커스텀 드라이버로 컴파일러의 `TyCtxt` 를 잡아 **아이템 전량**을 덤프한다 — `def_path` · `DefKind` · **`파일:줄:칸`** · 가시성 · `mir`/`xinl` · 시그니처 · ADT(필드·variant·**실제 판별자**) · **`layout_of`(오프셋·니치)**.
**언제**:
- 구조체/열거형을 **믿을 수 있는 값으로** 알아야 할 때 → `tcxdict`
- **우리가 발표한 오프셋이 맞는지** 감사할 때 → `tcxaudit --prose <문서.md>`
- 어떤 함수가 **`pub` 인지 / MIR 이 있는지** 판정할 때 → `_tcx\*.json` 의 `v`·`mir`
**실적**: 발표분 778건 감사 → **오귀속 4건 적발**(전부 산문 쪽. 구조화 표는 오귀속 0·밀림 0).
**★MIR 가용 조건 (실측 확정)**: rmeta 는 **`cross_crate_inlinable` 또는 제네릭**인 함수에만 MIR 을 싣는다. 전 DefId **210,093개**에서 `xinl=1 && mir=0` **0건**.
⟹ **크고 비인라인인 private 함수의 MIR 은 애초에 없다. 어떤 디코더로도 못 꺼낸다.**
**한계**: ADT 아닌 것(vtable 슬롯), `&dyn` 팻포인터 뒤 절반, SDK rlib 밖 타입.
→ `IR_TOOLKIT.md §6 / §6-b / §6-c`

## ⑧ Ghidra 디컴 + 런타임 훅 (exe)
**무엇**: 원래 방법. 심볼 stripped(`FUN_xxx`), image base `0x140000000`.
**언제**:
- **SDK 밖**(게임 본체 exe 코드), rmeta·IR 에 없는 것
- **런타임 실측이 필요할 때** — 재현 검증의 최종 심판은 **`game==mine` 비트동일**이다
- IR 로 못 편 실코드 줄의 **마지막 남은 경로**
**⚠**: 새 디컴은 **ghidra-re 에이전트 경유**(CLAUDE.md §9). 패치마다 RVA 가 어긋나니 버전 태그 필수.

---

# 2. 프로세스 — 재료를 어떻게 굴리나

## A. 명세 파이프라인 (S0~S7) → `SPEC_RUNBOOK.md`
**핵심 발상**: *"한 번 잘 읽기"가 아니라 "독립적으로 여러 번 읽고 어긋난 곳만 사람이 본다".*
IR 독해는 지어내기보다 **빠뜨리기**가 훨씬 많은데 검사기(`qcspec`)는 지어낸 것만 잡는다. 빠뜨린 건 **독립 2회 대조**(`speccmp`)로만 드러난다.

**실측 수렴 곡선** (같은 20함수를 서로 안 보여주고 재작성):

| 대조 | 판정상수 | 호출 | 필드 |
|---|---|---|---|
| r1 ↔ r2 | **87.0%** | **74.7%** | 84.2% |
| r3 ↔ r4 (사전 투입 후) | 96.6% | 96.2% | 95.7% |
| r5 ↔ r6 (잔여만 재투입) | **100.0%** | 97.9% | 93.0% |

⟹ **1차 독해를 정본으로 삼았으면 판정상수 8개 중 1개가 틀렸다.**
**멈춤 조건 = 판정상수 100%**, 불일치 남은 함수만 재투입(전량 재투입 금지).

## B. 재료 확장 4축 (S6) — `unknown` 이 쌓였을 때
1. **aux 범위** — 담당 줄범위 밖 클로저/헬퍼(`fnparts`)
2. **다른 크레이트** — `_gcbc`(시뮬 본체·정적 vtable) / `_gvbc`(UI·렌더러)
3. **rmeta** — 주석 ④ / SourceMap ③ / tcx ⑦
4. **아웃오브라인 본체** — 인라인돼 사라진 줄 알았던 함수의 `define`

**실적**: `unknown` **157 → 8**, 노브 **99 → 208**.
그리고 **"원리적 불가" 판정이 11건 뒤집혔다**(~~7건~~ → 4차 반증검증에서 4건 추가: `fight_check`/`path_finder` 크레이트 루트 재수출 · 07 `Hide` private 필드 `transmute` · 05 `plan_v50_dive_episodes` 경유 · 실전 `ChampionInfo` 61종 pub) — 원인은 매번 같았다: **가진 재료의 한계를 문제의 한계로 착각.**
★★그리고 4차는 **반대 방향의 사고**를 하나 더 냈다 — 「9축 전부 무관」이라는 **실행 결과가 거짓**이었다(TLS 메모 캐시). ⟹ 오라클은 IR 독해를 검증하지만, **오라클 자신을 검증하는 것은 `define` 의 존재 확인**이다. 재료의 한계를 문제의 한계로 착각하는 것과 **도구의 상태를 세계의 상태로 착각하는 것**이 같은 실패의 양면이다.

## C. 기계 검증 (S6-b) — 읽은 표를 자동으로 대조
- 오프셋을 하나라도 적었으면 → **제출 전에 `tcxaudit --prose`**
- 대상이 `pub` 이면 → **오라클 진리표로 대조**
- ⚠**감사 결과가 "우리 문서 규약"을 벌하면 도구를 고쳐라.** `--prose` 가 `~~취소선~~`(§7 이 남기라고
  요구하는 옛 값)을 되잡아, **정정을 쓸 때마다 오탐이 하나씩 영구히 쌓이고 있었다**(2026-09-11 수정).
  오탐을 손으로 넘기기 시작하면 그 다음부터 **진짜 오귀속도 같이 넘긴다** — 반복 오탐은 도구 결함으로 취급할 것.

---

# 3. ★판정 어휘 — 이게 제일 중요하다

**"X 불가능"이라고 쓰지 마라. "A 방식으로는 X 불가능, 미탐색 = B" 라고 써라.**
범위를 안 적으면 다음 세션이 **다른 방식조차 시도하지 않는다.** 지금까지 **11건**이 그 함정이었다.

| 분류 | 뜻 | 예 |
|---|---|---|
| **표기 불가** | **동작은 확정**, 소스 표기만 불가 | `==5` vs `>=5` — 외연이 동일해 IR·MIR·기계어·오라클 어디에도 차이가 안 남는다 |
| **재료 부재** | 이 재료에 실체가 없음 — **탐색 범위를 열거하라** | `MF_SRC_NAMES` — SDK deps 308개·IR 3종·exe 전량 0건 / 비인라인 private 함수의 MIR |
| **미탐색** | **불가가 아니라 아직 안 읽음** | "`resolve_join_stake` 1,353줄" |

★**미탐색을 표기 불가로 적는 것이 최악의 오염이다.** 다음 세션이 진짜로 포기한다.

---

# 4. 함정 톱 10 (전부 실제로 밟았다)

1. ★**열거형 태그 ≠ variant 인덱스.** 니치로 밀린다(`SubPlan`=idx+2, `LineGankerPhase`=idx+6). **반드시 `tcxdict --enum` 의 `메모리태그` 열을 인용하라.**
   ⚠**정정(2026-09-11 검증배치 A)**: ~~「`LineType` 은 태그 Top0/Mid1/Bottom2 인데 선언 순서가 `Mid,Bottom,Top`」~~ 은 **거짓이었다.** tcx variant `def_span` 실측 = `Top player.rs:991 / Mid 992 / Bottom 993` 이고 `tcxdict --enum` 의 `idx`(=rustc `VariantIdx`=선언 순서)도 태그와 **완전히 같다**. 줄 길이 교차검증도 일치(991·992 = 6자 `  Top,`/`  Mid,`, 993 = 8자 `  Bottom`). `ObjectPhase` 도 마찬가지다.
   ⟹ **실증된 함정은 「메모리 태그 ≠ 논리 인덱스(니치 밀림)」 하나뿐**이고, **「선언 순서 ≠ 태그」는 실증 사례 0건**이다. ★교훈: 이 오류는 한 배치의 미검증 주장을 **검증 없이 방법론 문서로 승격**시켜 생겼다. 함정 목록에 올리기 전에 **독립 근거를 요구하라.**
2. ★**`!dbg` 는 `inlinedAt` 루트까지 타라.** scope 만 보면 `line: 1` 이 나온다.
3. ★**`DW_OP_not` 개수로 극성을 읽지 마라.** 이중 이상은 컴파일러 아티팩트(전 모듈 57건, 개수 3·5·6 사례 5건). **극성은 분기 방향으로.**
4. **사전의 동명 충돌** — 라이브러리 타입이 게임 타입을 조용히 덮어쓴다. `tcxdict` 는 **모호하면 후보 전량을 돌려준다**(구사전은 하나를 고른다).
5. **상수 스캔이 값을 놓친다** — 비상수 `phi` 로 들어오는 값은 안 잡힌다(`exit_sub` 6·7, `dive_abort_src` 1·11). **store 지점의 phi 까지 봐야 전수.**
6. **오프셋만으로 필드 스캔하면 다른 구조체가 딸려 온다** — `+0x108` = 2,339곳 → `--src battle.rs` 한정 45곳. `fieldcodes.py` 는 **오염**(파일 전역 매칭), 후속 정본은 **`fieldall2.py`**.
7. **"노브"라고 적기 전에 소비처를 확인하라** — `BattlePlanGoal.__1` 은 생성 14곳 전부 리터럴 60인데 **산술 소비처 0건**. `mf_swap.__0` 도 load 0건 = 순수 텔레메트리.
8. **같은 이름표를 두 필드가 공유할 수 있다** — `exit_src` 0~15 는 `BattlePlan+0x108`, 16~23 은 `LegacyPlanHandler.ff_battle_exit.__0`.
9. ⚠**성능**: 60~75MB `.ll` 에 줄 단위 `sed -n "${k}p"` 루프 **금지**(1시간 낭비 후 강제종료). 도구가 자매 도구보다 훨씬 느리면 **그 자체가 버그 신호**다(`dienum` 40분 → 57초).
10. ⚠**동시성**: 같은 산출물에 두 배치를 띄우지 마라(`dienum.json` 경합 덮어쓰기 **2회** 발생). 배치마다 **산출물 디렉터리를 갈라라**.

---

# 5. 한눈에 보는 신뢰도 서열

**충돌하면 이 순서로 이긴다:**

```
런타임 실측(DIFF=0)  >  tcx(컴파일러 정본)  >  오라클 실행  >  LLVM IR  >  rmeta 주석
```
- **타입·오프셋·판별자**는 `tcx` 가 정본이다(컴파일러가 직접 말한다).
- **오라클이 IR 과 어긋나면 양쪽을 다 의심하라** — 오라클은 입력 구성이 틀리면 **조용히** 틀린다.
- **주석이 IR 과 충돌하면 IR 이 정본**이다.
- 단 **`tcx` 에 없는 것을 "존재하지 않는다"로 읽지 마라**(SDK rlib 밖은 안 보인다).
