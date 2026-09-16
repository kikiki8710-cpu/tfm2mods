# MIG 도구 인벤토리

> ★**이 파일은 `mktools.py` 가 생성한다. 손으로 고치지 마라** — 다음 재생성에서 날아간다.
> 도구를 만들면 **파일 첫 docstring 줄**을 제대로 쓰고 `python -X utf8 mktools.py` 를 돌려라.
> 분류에 없는 새 도구는 맨 아래 **미분류** 절에 자동으로 뜬다(거기 보이면 `mktools.py` 의 `CAT` 에 한 줄 추가).

**어떤 상황에 무엇을 집는가 = `METHOD_MAP.md` §0**(라우팅표). 이 문서는 *무엇이 있는가* 만 센다.

전체 172개 · 생성 시각 기준 자동 집계 (`_` 로 시작하는 1회용 스크래치는 제외)

## ★간접 호출 실측 · 지도 밖 호출자 편입 (2026-09-16 신설)

진입부 프로브 스텁의 리턴 주소 히스토그램(probe.rs RETREC)으로 「누가 몇 번 불렀나」를 재고, 지도에 얹는다. 순서 = probe20 --map-all → 판 → retedges(→ retmerge) → fnmap_ret → fnmap_ext → fnmap_dead. 리플레이를 바꿔도 표본은 안 바뀐다(03 §45).

| 도구 | 하는 일 |
|---|---|
| `retedges.py` | probe20.txt 의 리턴 주소 히스토그램(`RET` 행)을 「호출자 함수 → 피호출 함수」 간선으로 환산한다. (2026-09-16 신설) |
| `retmerge.py` | 여러 판의 `retedges.py` 출력을 합친다(간선 합집합 · 횟수 합산 · 판별 존재 표). (2026-09-16 신설) |
| `fnmap_ret.py` | 실측 호출 간선(`retedges.py` 출력)을 AI함수지도.html 에 얹는다. (2026-09-16 신설) |
| `fnmap_ext.py` | 실측으로 드러난 「지도 밖 호출자」를 AI함수지도 노드로 편입한다(계층 `ext`). (2026-09-16 신설) |
| `fnmap_dead.py` | 「아예 도달 못하는」 함수를 지도에서 옆으로 격리한다(`dead` 표식 · 전체 지도 오른쪽 격리 상자). (2026-09-16 신설 · 유저 지시) |
| `cmp_runs.py` | (docstring 없음 — 채워라) |
| `upwalk.py` | (docstring 없음 — 채워라) |
| `retrec_asm.py` | (docstring 없음 — 채워라) |
| `extname.py` | 지문(패닉 Location) 없는 지도 밖 exe 함수의 정체를 「exe 콜리 집합 ↔ IR 콜리 집합」으로 좁힌다. (2026-09-16 신설) |
| `extid.py` | 지문(패닉 Location) 없는 exe 함수의 IR 짝을 「상수 지문(변위·즉치) + 콜리 이름 + 크기」로 좁힌다. (2026-09-16 신설) |

## 밴픽 IR 카탈로그 (별도 세션 09-14 · banpick 소스 define 전수)

bpcatalog → bpdump(irann 방식 일괄 주석) → bpname(RVA 지문 실명).

| 도구 | 하는 일 |
|---|---|
| `bpcatalog.py` | 밴픽 관련 소스 파일에 속한 IR define(함수) 전수 카탈로그(파일·줄·define 위치·크기). python bpcatalog.py [--out <json>] [--files <regex>] |
| `bpdump.py` | bpcatalog.json 의 전 함수를 irann 방식(소스줄 주석·잡음 제거)으로 일괄 덤프한다. python bpdump.py <catalog.json> <outdir> |
| `bpname.py` | exe RVA 들의 패닉 Location 지문을 bpcatalog.json(밴픽 IR 카탈로그·_gcbc/_gvbc 포함)의 함수 소스 범위와 대조해 실명을 판정한다. python bpname.py <catalog.json> <rva>... |

## ★exe 함수 정체 판정 — Ghidra 없이 (2026-09-13 신설)

「이 RVA 가 어느 IR 함수인가」. capstone 프로파일 ↔ IR define 프로파일 ↔ 패닉 Location 지문. 인라인 여부만은 xref(§22-85).

| 도구 | 하는 일 |
|---|---|
| `fnprobe.py` | exe 함수 하나의 프로파일(capstone · Ghidra 대체). (2026-09-13 · ghidra-re 스크래치 승격) |
| `irprobe.py` | IR define 하나의 프로파일: 인자·call 집계·DILocation 줄 집합 (2026-09-13 · ghidra-re 스크래치 승격 · fnprobe.py 의 IR 쪽 짝). |
| `locfind.py` | 패닉 Location(파일:줄[:열]) 상수를 참조하는 IR define 을 역추적한다 (2026-09-13 · ghidra-re 스크래치 승격 · `@anon.<hash>.N` 대응). |
| `namebyline.py` | 지도 미명명(`?`) exe 함수를 **패닉 Location 줄번호 지문**으로 IR define 에 잇는다 (2026-09-13) |
| `namebycaller.py` | 지도 미명명(`?`) exe 함수에 「호출자 차집합」으로 IR 심볼 이름을 붙인다 (2026-09-13) |
| `r8addr.py` | 라운드 JSON(`_spec\\rN\\*.json`) 의 exe 주소 후보를 **aimap 패닉 Location ∩ IR 본문 줄범위**로 찾는다. (2026-09-13 신설) |
| `offscan.py` | .pdata 함수 전수에서 「메모리 오퍼랜드 변위(disp)·즉치 집합」을 모두 가진 함수를 찾는다. (2026-09-13 신설) |
| `rvaname.py` | exe RVA 목록의 **IR 실명**을 패닉 Location 지문으로 일괄 판정한다. (2026-09-13 신설 · fnprobe+locfind 합성) |
| `argscan.py` | exe 함수의 **진입 스택 인자 슬롯 전수 스캔**(capstone 선형 디스어셈 · 디컴 불필요). |
| `mergespec.py` | 한 함수를 여러 배치가 분책으로 쓴 부분 명세(`<id>.partA.json` …)를 **하나의 명세 JSON** 으로 합친다. (2026-09-14 신설 · r12 update) |

## ★사전 — 타입·오프셋을 묻는 곳

**여기부터 친다.** DWARF 를 손으로 타지 마라.

| 도구 | 하는 일 |
|---|---|
| `tcxdict.py` | tcxdict — rustc `TyCtxt` 권위 덤프로 만든 **구조체/열거형 정본 사전**. |
| `tcxaudit.py` | tcxaudit — 이미 발표한 오프셋 주장을 **tcx 정본 사전(tcxdict)** 으로 전수 재확인. |
| `tcxverify.py` | 명세에서 쓰고 있는 핵심 오프셋을 tcx layout_of 로 표적 검증(중첩 구조체까지 재귀 해석). |
| `tcxcross.py` | tcx 권위 덤프 vs 기존 DWARF 역산 사전(distruct.json / dienum.json) 기계 대조. |
| `tcxfield.py` | 필드 이름 -> (타입, 오프셋) 역검색. |
| `tcxq.py` | _tcx\<crate>.json 조회 헬퍼. |
| `tcxpub.py` | 크레이트의 pub(선언 가시성) API 표면 추출. 사용: python -X utf8 tcxpub.py <crate> > out.txt |
| `tcxbuild.ps1` | 커스텀 rustc 드라이버 빌드. 사용: powershell -File C:\tfm2mods\MIG\tcxbuild.ps1 -Src C:\tfm2mods\MIG\_tcxdrv\tcxdump.rs |
| `tcxrun.ps1` | 커스텀 rustc 드라이버 실행(SDK rlib 전부 링크). 사용 예: |
| `divtable.py` | `dyn Trait` 의 **vtable 슬롯 오프셋 → 함수 이름**. |
| `distruct.py` | ⚠**2차 폴백** — 키가 leaf 이름 1개뿐이라 동명끼리 조용히 덮어쓴다. 1차 = `tcxdict.py` · IR 디버그정보에서 **구조체 오프셋 → 필드 이름** 사전을 한 번에 뽑는다. |
| `dienum.py` | ⚠**2차 폴백** — tcx 와 모순 0이나 커버리지 39.7% 결손. 1차 = `tcxdict.py --enum` · 열거형의 **태그값 ↔ variant 이름** 사전. |

## 재료 추출 — rlib 에서 꺼내기

IR·rmeta 를 뽑고 훑는다.

| 도구 | 하는 일 |
|---|---|
| `rlib2ll.py` | SDK rlib 에서 **LLVM IR(.ll)** 를 뽑는다. 크레이트 아무거나. |
| `bundlegrep.py` | 게임 **데이터/에셋 번들** 전체를 바이트로 훑어 패턴을 찾는다. |
| `rmetadocs.py` | rlib 들의 `lib.rmeta` 에서 **한국어 문서화 주석**을 빠르게 훑는다. |
| `rmeta_srcmap.py` | rmeta 의 SourceMap(SourceFile 테이블)을 디코드한다. |
| `rmetagrep.py` | lib.rmeta 원본 바이트에서 패턴 주변 문맥(문자열 런)을 뽑는다. |
| `rmetadump.py` | lib.rmeta 의 읽을 수 있는 문자열 런을 **전부**(ASCII 전용 포함) 오프셋과 함께 덤프. |
| `dqctx.py` | rmeta 한국어 주석을 **주변 식별자 컨텍스트와 함께** 조회한다. |
| `dqraw.py` | rmeta 원시 바이트 구간을 '읽을 수 있게' 덤프. 사용: dqraw.py <crate> <off> <len> |
| `rmeta_probe1.py` | (docstring 없음 — 채워라) |
| `rmeta_probe2.py` | (docstring 없음 — 채워라) |

## IR 독해 — 본문을 읽을 때

함수 조각 찾기 → 줄번호 잇기 → 값·조건 추적.

| 도구 | 하는 일 |
|---|---|
| `fnparts.py` | 한 함수의 **조각 전부**(클로저·모노모픽 인스턴스)를 전 IR 에서 찾는다. |
| `dloc.py` | !DILocation 체인을 inlinedAt 루트까지 펼친다. |
| `dbgchain.py` | !dbg 노드의 inlinedAt 사슬을 전부 펼친다.  python dbgchain.py <ll> <mdid> [<mdid>...] |
| `llann.py` | IR 구간을 소스 줄번호로 주석해 출력.  python llann.py <ll> <from> <to> [--root] |
| `srcmap.py` | IR 줄범위를 '게임 소스 줄' 로 접어 보여준다. |
| `irann.py` | SDK LLVM IR(.ll) 한 함수를 **읽을 수 있게** 주석·축약해 출력한다. |
| `ann.py` | ann.py <ll> <start> <end> — IR 를 소스줄 주석과 함께 덤프(inlinedAt 루트 + 인라인 체인). |
| `ann2.py` | ann2.py <ll> <line> [before] [after] — 저장지점 주변을 요약해서 본다(호출 이름 전개). |
| `ann3.py` | ann3.py <ll> <line[,line...]> [before] [after] — 저장지점 주변 요약(호출/분기/self-store 만). |
| `irfn.py` | (docstring 없음 — 채워라) |
| `irctx.py` | (docstring 없음 — 채워라) |
| `dl.py` | (docstring 없음 — 채워라) |
| `guard.py` | 어떤 store 지점이 **어떤 조건 아래** 실행되는지 역추적한다. |
| `inlsites.py` | 인라인된 헬퍼의 **호출 사이트**를 전부 찾는다. |
| `vbr.py` | 함수 본문에서 특정 SSA 인자(기본 %1 = version)에 대한 비교를 전부 뽑고 |
| `reach.py` | IR 한 함수의 CFG 에서 「알려진 상수 조건」을 접어 사장 블록·사장 호출부를 가른다 (교훈 68 도구화 · 2026-09-13) |
| `reach_tree.py` | 루트 함수에서 IR 호출 그래프를 따라가며 「도달 가능성」을 봉인한다 (교훈 68 · 2026-09-13) |
| `rootcut.py` | 거대 함수 IR 을 **루트 소스 줄**(`!dbg`→`inlinedAt` 최상위) 기준으로 분책 경계를 잘라 준다 (2026-09-15 · r12 `upd_blocks.json` 방식 도구화) |
| `fieldall2.py` | 함수 스코프를 지켜서 gep 오프셋 → store 를 잡는다. |
| `fieldall.py` | 오프셋에 store 하는 i8 **전부**(상수 + 레지스터). fieldcodes.py 확장. |
| `fieldcodes.py` | ⛔**오염** — gep 결과 레지스터를 **파일 전역**으로 매칭해 다른 함수의 동명 `%N` 을 잡는다. 후속 정본 = `fieldall2.py` · `<구조체>+<오프셋>` 에 **저장되는 u8 코드값**을 전수로 뽑고 소스 줄을 붙인다. |
| `knobscan.py` | game_ai IR 에서 **매직 상수 전수 목록**을 뽑는다(노브 후보 인벤토리). |
| `cachedfns.py` | game_ai 의 **메모 래퍼**(`*_cached` / `*_uncached` 관례)를 전수 나열한다. |
| `cachekeys.py` | 캐시 래퍼가 부르는 `LocalKey::with` **인스턴스 심볼**을 뽑는다. |
| `memofns.py` | game_ai / game_core 안의 **메모 캐시를 쓰는 함수**를 전수 조사한다. |
| `tlsscan.py` | game_ai / game_core 의 **TLS 전역(메모 캐시)** 전수와 크기. |
| `coreglobals.py` | game_core / game_ai 의 **가변 전역**(비-TLS)을 전수 나열한다. |

## ★오라클 — SDK 함수를 진짜 실행

`-C lto=fat` 로 rlib 이 exe 로 링크된다. `pub` 이면 **진리표로 검증**할 수 있다. 전문 = `IR_TOOLKIT §7`.

| 도구 | 하는 일 |
|---|---|
| `spanprobe.ps1` | (docstring 없음 — 채워라) |
| `rdocprobe.ps1` | (docstring 없음 — 채워라) |
| `probe.py` | (docstring 없음 — 채워라) |
| `aiprobe.py` | "이 함수가 실제 경기에서 뜨긴 하는가" 를 재기 위한 **카운트 전용 프로브 표** 생성기. |

## 명세 파이프라인 ① 정본 만들기

`_spec\specs20.json`(v2, 손이 닿는 정본) → `specs20_v3.json`(생성물). **v3 를 직접 고치지 마라.**

| 도구 | 하는 일 |
|---|---|
| `mkspec3.py` | mkspec3 — `_spec\\specs20.json`(v2) → `_spec\\specs20_v3.json`(v3) 재구성. (2026-09-11) |
| `spec3lib.py` | spec3lib — 명세 v3 의 **게이트 필드를 코퍼스에서 자동으로 채운다.** (2026-09-11 신설) |
| `mkspec3_md.py` | mkspec3_md — `specs20_v3.json` → REPORT 용 마크다운. (2026-09-11) |
| `mkspec20.py` | 20함수 명세를 하나로 합쳐 `specs20.json` 을 만든다. |
| `mkspec_md.py` | `specs20.json` 을 REPORT 용 마크다운 명세로 편다. |
| `cleanspec.py` | cleanspec — 정본 본문에서 **고고학 지층을 걷어낸다**. (2026-09-11 신설, 유저 지시) |
| `mkmap_html.py` | `TFM2 AI 함수 지도` 아티팩트에 **20함수 상세 명세**를 붙인다. |

## ★명세 파이프라인 ② 라운드 4계약 — **경계마다 사람 손을 뺀다**

절차 = `SPEC_RUNBOOK.md §S5-d`. 6라운드 실측에서 오류의 상당수가 「사람이 손으로 옮겨 적는 경계」에서 났다.
**지시** `mkdossier` → **보고** `patch.json` → **반영** `applypatch` → **검증** `auditrounds`.
★7차부터 지시 = `mkbrief`(요약) 가 아니라 **`mkdossier`(정본 무손실 조립)** 이다 — 6차까지의 지시 오류가 **전부 「줄여 적은 자리」**에서 났다.

| 도구 | 하는 일 |
|---|---|
| `mkdossier.py` | mkdossier — 배치 하나가 받을 **정본 전량**을 한 파일로 조립한다(요약 금지). (2026-09-11 신설) |
| `dossierfresh.py` | dossierfresh — 배치가 **자기가 읽은 지시문이 최신인지** 스스로 확인한다. (7차 배치C 적발로 신설) |
| `mkbrief.py` | ⚠**요약 방식(~6차)** — 브리핑 오류가 전부 「줄여 적은 자리」에서 났다. 7차 정본 = `mkdossier.py`(무손실) · mkbrief — 반증검증 라운드 브리핑의 **사실 절을 데이터에서 생성**한다. (2026-09-11 신설) |
| `mkpatch.py` | mkpatch — 배치가 `patch.json`(정정 계약)을 **안전하게 만드는** 참조 구현. (2026-09-11 신설) |
| `applypatch.py` | applypatch — 배치가 낸 **기계 판독 정정**(`patch.json`)을 v2 정본에 적용한다. (2026-09-11 신설) |
| `auditrounds.py` | auditrounds — **지금까지 모든 라운드의 정정이 아직 살아 있는지** 자동 회귀 검사. (2026-09-11 신설) |
| `specgate.py` | specgate — 명세의 **완결 조건**을 기계로 검사한다. (2026-09-11 신설) |

## ★명세 파이프라인 ③ 게이트 — 완결 조건 검사

`specgate.py` 가 게이트를 돌리고, 아래 것들이 그 게이트가 부르는 **검사기**다.
**전부 「검사받지 않는 축」에서 나왔다** — 붙일 때마다 그 축에 고여 있던 오류가 드러났다(`SPEC_RUNBOOK §S5-c`).
⚠★**적발 건수는 게이트의 성능이 아니라 가설이다.** 붙이자마자 나온 수를 성과로 믿지 말고 **표본을 IR 원문으로 반증**하라 — 7차 `G12` 47건은 검증해 보니 대부분 오탐이었고, 8차엔 네 축의 후보 총 적발 **100여 건 중 진짜가 26건**이었다.
⚠★**「부재」를 결함으로 세지 마라**(콜리 안 접근·상수 접힘·레지스터 승격). `G14` 후보 하나가 그래서 46건을 냈다.
★**검출력은 「변이 시험」으로 재라** — 명세를 일부러 뒤집어 넣고 몇 %를 잡는지 본다(`memdir.py` 실측 53.1%). 그래야 **적발 0 이 무능이 아니라 「그 축에 오류가 없다」**임을 말할 수 있다.

| 도구 | 하는 일 |
|---|---|
| `srclinecheck.py` | G12 `consts[].src_line` 대조 — **9차 배치A 통합판**(`srclinecheck.py` 의 후속). |
| `srclinebase.py` | `consts[].src_line` 기계 대조 — 그 리터럴이 실제로 어느 소스 줄에서 왔는지 **inlinedAt 사슬 전체**로 본다. |
| `whereline.py` | whereline — `knobs[].where` 의 **IR 줄번호와 인용 명령**을 실제 `.ll` 과 대조한다. (2026-09-11) |
| `memdir.py` | **G14 — `mem[].dir`(읽기/쓰기 방향) 기계 대조.** 8차 배치A 통합판. |
| `kindchk.py` | G15 `consts[].kind` — **상수의 종류** 게이트. 8차 배치B 통합판(후보 B·C·D 흡수). |
| `histprop.py` | G17 — 축 `history` → 표 **전파** 검사. (8차 배치 D 신설, 2026-09-11) |
| `paramrole.py` | `sig.params[].role` 기계 대조 — **G16 승격판(9차 배치B)**. |
| `xreflogic.py` | **G18 — `logic` ↔ `mem`/`consts` 상호참조.** 9차 배치D 신설. |
| `knobval.py` | knobval — `knobs[].value`(노브의 **값**)를 IR 원문과 대조한다. (9차 배치C · 2026-09-11) |
| `sharedchk.py` | G20 `shared` — **명세 간 공유 사실 대조** 게이트. |

## 명세 파이프라인 ④ 품질 대조

지어낸 내용 거르기 · 독립 재작성 대조.

| 도구 | 하는 일 |
|---|---|
| `pick20.py` | 명세 작성 시범 배치용 **대표 표본**을 뽑는다. |
| `qcspec.py` | 함수 명세(JSON)를 **IR 본문과 기계적으로 대조**해 지어낸 내용을 거른다. |
| `speccmp.py` | **같은 함수를 독립적으로 두 번 명세한 결과**를 비교해 신뢰도를 잰다. |
| `corpus.py` | "640개를 다 파악한다"의 **실제 분량**을 잰다. |
| `corpus2.py` | "파악해야 할 소스 함수"의 진짜 개수. |

## ★명세 파이프라인 ⑤ 함수 추가·투영·착수 grep (2026-09-13)

신규 함수 = `addspec.py`(⛔`mkspec20` 재실행 금지) · 사람용 설명 = `mkfnexplain.py`(`REPORT\tfm2_judge_verify\05_*`) · 함수지도 갱신 = `fnmap_update.py` → `mkmap_html.py` · 착수 전 8곳 일괄 grep = `whatsdone.py`.

| 도구 | 하는 일 |
|---|---|
| `addspec.py` | 명세 정본(`_spec\\specs20.json` v2)에 **함수 1개를 추가**한다. (2026-09-13 신설) |
| `mkfnexplain.py` | 판단함수 「내용 설명」 문서 생성기 (2026-09-13 신설) |
| `fnmap_update.py` | `AI함수지도` 기반 데이터(fndata 641함수)를 현행 사실로 갱신 + 관계 diff 보고 (2026-09-13) |
| `whatsdone.py` | 「이거 전에 했나?」를 5초로 (CLAUDE.md §7 착수 전 grep 자동화 · 2026-09-13) |
| `subtree_rank.py` | 상위 함수의 exe 호출 서브트리를 뽑아 「아래에서 닫는」 작업 순서표를 만든다 (2026-09-13) |
| `mkextra_specs.py` | 명세 밖 4함수(#21~#24)의 「간이 명세」 JSON 생성 (2026-09-13) |

## ★런타임 검증(ev1) — 주소 검증 → 1단계 프로브 → 2단계 sweep (2026-09-12~13)

절차 = `REPORT\tfm2_judge_verify\04_분석방법_정리.md §3·§4`. 주소는 반증형으로(`rvaverify`·`addrgate`·`callerprof`), 발화는 `probe20`, 대조 래퍼는 `gensweep20`(기구 결정트리 §4), live 맵은 `heapsurf`/`enumlive`/`structlive`.

| 도구 | 하는 일 |
|---|---|
| `rvaverify.py` | **명세 20함수의 `exe.addr` 이 정말 그 함수인가.** (2026-09-12 신설) |
| `addrgate.py` | **`exe.addr` 을 지문 점수로 게이트한다**(신호 S6). (2026-09-12 신설) |
| `callerprof.py` | **「누가 이 주소를 부르는가」로 주소를 검증한다**(신호 S4). (2026-09-12 신설) |
| `callsites.py` | exe `.text` 를 raw 스캔해 **특정 RVA 를 겨누는 `call`/`jmp rel32` 를 전수** 센다. |
| `abiagree.py` | **exe 의 함수가 IR `define` 과 같은 ABI 로 컴파일됐는가.** (2026-09-12 신설) |
| `disrva.py` | 현행 exe 의 RVA 구간을 capstone 으로 디스어셈블해 찍는다 (Ghidra 없이 빠르게 · 읽기 전용). |
| `whohooks.py` | **배포된 모드 dll 중 어느 것이 20함수 RVA 를 들고 있나.** (2026-09-12 신설) |
| `probe20.py` | **20개 AI 판단함수 전용** 카운트 프로브 표 생성기. (2026-09-12 신설) |
| `sweep20chk.py` | **2단계(sweep) 대조가 가능한 함수가 몇 개인가**를 먼저 잰다. (2026-09-12) |
| `gensweep20.py` | **2단계 sweep 코드 생성기**. 명세 20함수(`MIG\\_spec\\specs20_v3.json`)를 |
| `heapsurf.py` | `&mut self` 함수의 **전이적 힙 쓰기 표면**을 루트 self 기준 절대 오프셋으로 전수한다. |
| `enumlive.py` | tcx 열거형 레이아웃에서 **variant 별 살아있는 바이트 범위**(ELEM_LIVE 명세)를 자동 생성한다. |
| `structlive.py` | tcx `--deep` 레이아웃에서 **구조체의 살아있는 바이트 맵**(패딩·Vec 삼중항 제외 · 열거형/Option 은 |

## exe ↔ IR 잇기 — 이름·주소 붙이기

IR 의 이름을 exe RVA 에 잇거나, exe 함수에 이름을 붙인다.

| 도구 | 하는 일 |
|---|---|
| `name2rva.py` | game_ai IR 의 **함수 이름** → 게임 exe **RVA** 를 소스 좌표로 잇는다. |
| `panicloc.py` | SDK 비트코드와 게임 exe 의 **패닉 Location 상수**를 같은 형식으로 뽑아 대조한다. |
| `srcident.py` | ⛔STALE(2026-09-09 · 0.5.8) — 판정 정본 = `panicloc.py` · [STALE 2026-09-09 · 0.5.8] DILocation 기반 1차 방법. **판정 정본 = panicloc.py**. |
| `typeid_map.py` | game_ai IR 의 TypeId(i128) 상수 ↔ exe 의 실제 16B 상수 매핑. |
| `irskew.py` | ⛔폐기 — exe 는 상수가 접혀 소스 줄로 못 되돌린다. 후속 = `irskew2.py`(필드 오프셋 지문) · SDK rlib(IR) 과 배포 exe 의 **소스 어긋남(skew)** 을 전수 탐지한다. |
| `irskew2.py` | SDK rlib(IR) 과 배포 exe 의 소스 어긋남을 **필드 오프셋 지문**으로 탐지한다. |
| `dllmatch.py` | **exe ↔ 내 DLL 의 기계어끼리** 맞춰 exe 함수에 이름을 붙인다(2안). |
| `dmcheck.py` | `dllmatch.json`(2안) 의 이름을 **완전히 다른 신호**로 검산한다. |
| `dupmatch.py` | exe 안의 **중복 인스턴스**에 이름을 물려준다 (dllmatch 의 짝). |
| `percolate.py` | **씨앗 기반 그래프 정합(seeded graph matching / percolation)** 실험. |
| `cg.py` | exe / 내 DLL 의 **직접 호출 그래프**를 만들어 캐시한다(dllmatch 2안의 보강재). |
| `callgraph.py` | exe 전 함수의 직접 호출 대상(call/jmp rel32) 인덱스 빌더. |
| `callees.py` | exe 함수 하나가 **직접 call 하는 대상**을 스캐폴딩 정보(명령수·소스파일)와 함께 나열한다. |
| `callcount.py` | exe 함수 하나가 **직접 call 하는 대상별 호출 횟수**를 센다. |
| `aibound.py` | aibound — Ghidra 의 함수 경계를 **`.pdata` 진실값**과 대조한다 |
| `aiclass.py` | aiclass — AI 계층 640함수를 **역할별로 분류**해 재현 우선순위를 만든다 |
| `aiscope.py` | aiscope — exe 안의 **Rust 모듈 트리 전체**를 뽑아 재현 범위를 실측한다 |
| `ainoret.py` | ainoret — noreturn 으로 **오판된 함수**를 전수로 찾아 해제한다 |
| `aimap.py` | game_ai 계층 **상관관계 지도**를 만든다. |
| `ghidra_syms.py` | rlib 의 DWARF 를 exe RVA 에 이어 **Ghidra 주입용 심볼표**를 만든다. |
| `ghidra_inject.py` | `dllmatch.json`(2안) 의 이름·시그니처를 **Ghidra 에 주입**한다. |
| `ghidra_cycle.ps1` | (docstring 없음 — 채워라) |

## 마이그레이션 — 패치가 왔을 때

진입점은 `run.py`. 상세 = `MODS\MIGRATION.md`.

| 도구 | 하는 일 |
|---|---|
| `run.py` | ★마이그레이션 단일 진입점. **모든 축을 순서대로 돌리고, 하나라도 건너뛸 수 없게 한다.** |
| `mig_verify.py` | MIG 매니페스트 도구 — 마이그 = "매니페스트의 각 엔트리를 새 버전에서 찾는 것" |
| `repin.py` | 매니페스트 STALE 엔트리를 새 exe 에서 재핀(버전 무관 범용 엔진). |
| `sitealign.py` | repin.py plan 결과의 **함수 중간 사이트(MID_*)** 를 명령 단위 정렬로 교차검증한다. |
| `midpin.py` | 함수 **내부**(mid-function) 바이트패치 사이트를 재핀한다. |
| `sitepin.py` | 바이트패치 사이트를 **prefix + 원본 즉시값**으로 재핀한다. |
| `fncheck.py` | 함수 진입 RVA 상수가 **현행 exe 에서도 그 함수인지** 검증하고 재핀 후보를 낸다. |
| `chain.py` | 함수 진입 RVA 를 **버전 체인으로 추적**해 현행 주소를 확정한다. |
| `offsets.py` | ★구조체 오프셋 축을 기계 검사한다 (MIG 의 두 번째 축). |
| `env.py` | ★RVA·오프셋이 아닌 "환경 축"을 기계 검사한다 (MIG 의 세 번째 축). |
| `posdiff.py` | 구/신 함수를 **명령 순서대로 나란히 걸어** 메모리 disp 가 달라진 자리만 뽑는다. |
| `apply_manual.py` | 수동 재핀 적용 — 주석·문자열을 제외한 **코드 위치의 리터럴만** 치환(길이보존 마스킹). |
| `bump_deps.py` | mod.mod_info 의 base 의존 대역을 새 게임 버전으로 올린다. |
| `aidiff.py` | aidiff — AI 판단함수 버전간 자동 대조기 |

## 재현·포팅 — judge 계층 만들 때

| 도구 | 하는 일 |
|---|---|
| `aiport.py` | aiport — AI 판단함수 **버전별 재구현(judge 계층) 드라이버** |
| `aidump.py` | aidump — AI 판단계층 **원본 소스 트리 복원** 스캐폴딩 |
| `aifill.py` | aifill — aidump 스캐폴딩의 `<본문>` 자리를 **디컴 결과로 자동 채운다** |
| `aifix.py` | aifix — 디컴이 실패한 자리를 **Ghidra 쪽에서 고쳐** 다시 뽑는다 |
| `ailink.py` | SDK game_ai 비트코드 rlib 을 **패치해서 재조립**한다(모드에 직접 링크하기 위한 전처리). |
| `gensweep.py` | 판정 계층이 이미 RVA 를 확정해 둔 함수들을 **전수** 링크사본 대조하는 코드를 생성한다. |
| `agent_digest_cmp.py` | agent_link 모드 1(원본 에이전트) vs 모드 2(내 사본 에이전트) 의 get_input 다이제스트 대조. |

## 운영 — 빌드·크래시·로그

| 도구 | 하는 일 |
|---|---|
| `mktools.py` | `MIG\\TOOLS.md`(도구 인벤토리)를 **자동 재생성**한다. |
| `selftest.py` | selftest — 명세 파이프라인 **도구 자신**의 회귀 시험. (10차 신설, 2026-09-11) |
| `logsnap.py` | 인게임 검증 전/후 **모드 로그 스냅샷과 diff**. |
| `modbisect.py` | 크래시 범인 모드 이분탐색 도구. |
| `apgate.py` | `tfm2_ai_adjust` 의 `apply_*` 바이트패치 체인을 cfg 로 on/off 해서 |
