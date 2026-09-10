# MIG 도구 인벤토리

> ★**이 파일은 `mktools.py` 가 생성한다. 손으로 고치지 마라** — 다음 재생성에서 날아간다.
> 도구를 만들면 **파일 첫 docstring 줄**을 제대로 쓰고 `python -X utf8 mktools.py` 를 돌려라.
> 분류에 없는 새 도구는 맨 아래 **미분류** 절에 자동으로 뜬다(거기 보이면 `mktools.py` 의 `CAT` 에 한 줄 추가).

**어떤 상황에 무엇을 집는가 = `METHOD_MAP.md` §0**(라우팅표). 이 문서는 *무엇이 있는가* 만 센다.

전체 104개 · 생성 시각 기준 자동 집계 (`_` 로 시작하는 1회용 스크래치는 제외)

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

## 명세 파이프라인 — 배치를 굴릴 때

절차 = `SPEC_RUNBOOK.md`. 규격 = `SPEC_GUIDE.md`.

| 도구 | 하는 일 |
|---|---|
| `pick20.py` | 명세 작성 시범 배치용 **대표 표본**을 뽑는다. |
| `qcspec.py` | 함수 명세(JSON)를 **IR 본문과 기계적으로 대조**해 지어낸 내용을 거른다. |
| `speccmp.py` | **같은 함수를 독립적으로 두 번 명세한 결과**를 비교해 신뢰도를 잰다. |
| `mkspec20.py` | 20함수 명세를 하나로 합쳐 `specs20.json` 을 만든다. |
| `mkspec_md.py` | `specs20.json` 을 REPORT 용 마크다운 명세로 편다. |
| `mkmap_html.py` | `TFM2 AI 함수 지도` 아티팩트에 **20함수 상세 명세**를 붙인다. |
| `corpus.py` | "640개를 다 파악한다"의 **실제 분량**을 잰다. |
| `corpus2.py` | "파악해야 할 소스 함수"의 진짜 개수. |

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
| `logsnap.py` | 인게임 검증 전/후 **모드 로그 스냅샷과 diff**. |
| `modbisect.py` | 크래시 범인 모드 이분탐색 도구. |
| `apgate.py` | `tfm2_ai_adjust` 의 `apply_*` 바이트패치 체인을 cfg 로 on/off 해서 |
