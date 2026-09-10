# TFM2 IR 툴킷 — SDK rlib 에서 원본 소스급 정보를 꺼내는 법

> **한 줄**: TFM2 SDK 의 rlib 들은 `-C embed-bitcode` 로 빌드돼 **아카이브 멤버 자체가 날 LLVM 비트코드**다.
> `llvm-dis` 만 돌리면 **디버그정보가 붙은 원본 IR**(심볼·구조체 필드명·소스 파일·줄번호)이 통째로 나온다.
> **Ghidra 디컴파일보다 훨씬 정확하다.** exe 에서 못 읽던 것이 여기선 읽힌다.

이 문서는 **AI 판단 계층 전용이 아니다.** UI·시뮬레이션·에셋 등 **어느 부분이든** 같은 방법이 통한다.
작성 2026-09-10 / 게임 0.5.8 / SDK `C:\tfm2mods\sdk_058`.

> **문서 4종 세트.** 무엇부터 할지 정하는 라우터 = **`METHOD_MAP.md`**(상황 → 방법, 각 방법의 한계).
> 이 문서는 그중 **재료를 꺼내는 법** 편이다.

---

## 0. 왜 이걸 먼저 읽어야 하나 — 실측된 손실

game_ai 판단함수 20개 명세를 6라운드 돌리며 남긴 `unknown` 157건 중:

- **9건**이 *"game_core 는 별도 크레이트라 본문이 없다 ⟹ 확인 불가"*
- **8건**이 *"`Arc<dyn Trait>` 이라 vtable 전역이 없다 ⟹ 원리적으로 불가"*

**둘 다 거짓이었다.** 해당 rlib 을 안 뽑았을 뿐이다. `_gcbc` 를 뽑자 17건이 즉시 풀렸고,
`Blackboard::is_recent_visible`(AI 시야 판정의 핵심) 같은 것이 한 번에 나왔다.

⟹ **"이 크레이트엔 없다"고 결론내기 전에 반드시 `rlib2ll.py --list` 를 먼저 보라.**

---

## 1. 재료 — 어떤 크레이트를 뽑을 수 있나

```bash
cd /c/tfm2mods/MIG
PYTHONIOENCODING=utf-8 python rlib2ll.py --list
```

2026-09-10 기준 **비트코드가 든 rlib 이 수십 개**다. 큰 순서로:

| rlib | 크기 | 뽑은 곳 | 무엇이 들었나 |
|---|---|---|---|
| `libgame_core` | 364MB | **`C:\tfm2mods\_gcbc\g00~g15.ll`** (992MB) | 시뮬레이션 본체 — Entity·Effect·Blackboard·MapDef·JungleRunner·전투 해결 |
| `libgame_view` | 324MB | **`C:\tfm2mods\_gvbc\v00~v15.ll`** (1,270MB) | **UI 전부** — StrategyUIRunner(272) · dropdown(110) · ItemInfo(168) · personal_tactics(5) |
| `libgame_ai` | 71MB | **`C:\tfm2mods\_gaibc\*.ll`** (303MB) | AI 판단 계층 — plan_legacy·fight_check·path_finder·action_score |
| `libengine_ui` | 7.6MB | 미추출 | UI 프레임워크(노드·레이아웃) |
| `libengine_core` / `libengine_asset` / `libimage` … | — | 미추출 | 엔진·에셋 |

새로 뽑기:
```bash
PYTHONIOENCODING=utf-8 python rlib2ll.py engine_ui      # → C:\tfm2mods\_engbc\ (관례대로)
PYTHONIOENCODING=utf-8 python rlib2ll.py <크레이트> --out D:\tmp\x --prefix x
```
game_view 는 324MB → **25초**에 끝났다. 디스크만 있으면 비싸지 않다.

⚠**`.ll` 파일 하나가 50~75MB 다. 절대 통째로 읽지 마라.**

---

---

## 1-b. ★★`lib.rmeta` — 개발자가 쓴 주석이 그대로 들어 있다 (2026-09-11 발견)

rlib 에는 `.ll` 로 안 뽑는 멤버가 하나 더 있다: **`lib.rmeta`**(game_core 는 131MB).
Rust 메타데이터라 **비공개 필드 이름·소스 경로·그리고 문서화 주석 원문**이 들어 있다.

```bash
cd /c/tfm2mods/MIG
PYTHONIOENCODING=utf-8 python rmetadocs.py            # 3개 크레이트 → _docs\*.txt (7초)
```

**실측(0.5.8)**: 한국어 주석 **2,925개 · 132,551자**

| 크레이트 | rmeta | 주석 |
|---|---|---|
| `game_core` | 131.0 MB | **1,975개** |
| `game_ai` | 7.5 MB | **807개** |
| `game_view` | 33.9 MB | 143개 |

이건 **IR 에 없는 정보**다. 예:
```
0=None 1=Serpen 2=Morgard 3=Repair 4=PressEpic 5=SplitEpic 6=수비 7=Nexus 8=PressTower 9=기타
[v48] 시전자 슬롯(0=skill, 1=skill2, 2=ult)의 논타겟 투사체 프로필
path[i]로 진입하는 엣지를 계획이 어떤 등급으로 알고 골랐는가 (0=Allow, 1=Soft, 2=Danger, 3=Deadly)
v7: Epic/Serpen 처치 시점 감지 — 관측팀(1-team) 기준으로 team 팀의 시야 밖 챔피언 위치를 오브젝트 위치로 인식 갱신
```
즉 **우리가 IR 에서 역추적하던 코드표를 개발자가 이미 적어 뒀다.**
`_docs\*.txt` 를 **먼저 grep 하라** — IR 을 읽기 전에.

⚠**주석은 소스와 어긋날 수 있다**(stale comment). IR 사실과 충돌하면 **IR 이 정본**이다.
다만 위 `v7` 주석은 `apply_object_kill_inference` 와 `blackboard[T]` 의미를 정확히 확인해 줬다 —
**교차검증 재료로는 최상급**이다.

★**아직 안 캔 것**: rmeta 에는 (크레이트 설정에 따라) `#[inline]`·제네릭 함수의 **MIR** 과
**Span(줄+열)** 도 들어갈 수 있다. 디코더가 필요해 이 세션에서는 확인하지 않았다.
"IR 에 `column:` 이 없어 같은 줄 안의 순서를 못 정한다"는 판정은 **IR 한정**이고,
**rmeta 경로는 미탐색**이다. `_ => None` 팔이 접혔다고 포기하기 전에 여기를 보라.

---

## 2. 작업 순서 — 이 순서를 지키면 대부분 20분 안에 끝난다

### ① 사전부터 조회한다 (손으로 DWARF 타지 마라)

```bash
cd /c/tfm2mods/MIG
PYTHONIOENCODING=utf-8 python distruct.py Entity            # 구조체 전체 필드
PYTHONIOENCODING=utf-8 python distruct.py Entity 0x670      # ★오프셋 → 필드 역조회
PYTHONIOENCODING=utf-8 python dienum.py SubPlan             # 열거형 태그 전표
PYTHONIOENCODING=utf-8 python dienum.py EntityType 13       # 태그 → variant + 페이로드 레이아웃
```

- `distruct.json` = **구조체 26,416개 / 필드 75,602개** (game_ai + game_core 합본)
- `dienum.json` = **열거형 7,016개**

**오프셋을 같이 주면 중첩을 끝까지 뚫는다** — `distruct PlayerState 0x930` → `info.team`,
`distruct MobaMode 0x1a8` → `jungle_runner.epic.live_list.len`.
배열 원소 타입·`Vec` 내부(`cap@0x0/ptr@0x8/len@0x10`)까지 나온다.

> 1차 배치 실측: 함수당 25~30분 중 **12~15분**이 "이 `+0x8c` 가 뭐냐"를 손으로 5~6단 타는 데 갔다.
> 사전 도입 후 그 구간이 **사실상 0**이 됐다.

### ② 함수 조각을 먼저 찾는다 (본문 읽기 **전에**)

```bash
PYTHONIOENCODING=utf-8 python fnparts.py defensive_crisis
PYTHONIOENCODING=utf-8 python fnparts.py attack_nexus::sub_plan   # 모듈로 좁히기
```

이터레이터 술어(`filter`/`min_by_key` 클로저)는 담당 줄범위엔 `call_mut` 심만 남고
**본체가 다른 `.ll` 에 있다**. 안 찾으면 "술어가 없다"고 잘못 결론낸다(1~3차 `unknown` 의 13.4%).

★**`_gaibc` 에 없으면 자동으로 `_gcbc` 를 훑는다**(7~9초). "외부 심볼이라 본문 없음"은 이제 도구가 직접 확인한다.

### ③ vtable 슬롯 이름

```bash
PYTHONIOENCODING=utf-8 python divtable.py AbstractGame 0x28   # → tick
PYTHONIOENCODING=utf-8 python divtable.py EffectType 0x118    # → on_caster (_gcbc 폴백)
```
DWARF `vtable_type$` 엔 `__method8` 처럼 번호만 있다. 이 도구는 **vtable 전역 상수의 함수포인터 배열을 순서대로** 읽는다.
과반(≥50%) 슬롯이 그 트레이트 이름을 가져야 결과를 준다(엉뚱한 vtable 반환 방지).

### ④ 그 다음에 본문을 읽는다

```bash
grep -n "^define.*<이름>" /c/tfm2mods/_gcbc/*.ll     # 줄 찾고
sed -n '157005,157050p' /c/tfm2mods/_gcbc/g07.ll     # 그 범위만
```

---

## 3. 측정된 함정 — 전부 실제로 밟은 것들

### ★열거형 태그 ≠ variant 인덱스 (최대 함정)
니치 최적화로 판별자가 밀린다. `SubPlan` = idx+2, `LineGankerPhase` = idx+6.
**`dienum` 이 "★니치 밀림" 을 헤더에 찍어준다. 그걸 보라.**

### ★`!dbg` 는 `inlinedAt` 루트까지 타라
`!DILocation` 의 `scope:` 만 보면 인라인된 헬퍼(`push`·`Option::ne`)를 가리켜 쓸모없는 줄이 나온다.
```bash
grep -n '^!14974 = ' m09.ll     # inlinedAt: !14770
grep -n '^!14770 = ' m09.ll     # ★루트 = line 676
```
"소스 줄이 소실됐다"는 판정이 이 방법으로 실제로 뒤집혔다.
⚠**단 `column:` 은 이 빌드에 전 모듈 0건** — 같은 줄 안의 순서(`A || B` 의 A/B)는 여전히 불가.

### ★상수 접힘
`x * 4` → `shl 2` 라 리터럴 4가 본문에 없다. `tps * 2` → `shl 1`.
**리터럴이 오프셋인 경우가 특히 위험하다** — `4856` 을 "은신 타이밍 임계"로 적었는데
실제로는 `GameSetting+0x12f8`(tick_per_second) **오프셋**이었다.

### ★동명 다른 타입
`fnparts target_bush_v30` 이 `LineGankCoverPlan` 것을 줬는데 담당은 `LineGankerPlan` 것이었다.
**소유 타입 열을 보라.** `distruct`/`dienum` 은 크기(IR 의 `dereferenceable(N)`)로 교차검증하라.

### ★사전의 동명 충돌
game_core 를 사전에 넣자 게임 타입이 라이브러리 동명 타입에 덮어써졌다 —
`LineType` Top/Mid/Bottom → EXIF `Exposure/Pixaspect`, `State` → zlib 디코더.
**규칙: 디렉터리 우선순위가 크기보다 먼저.** `_gaibc` 판이 항상 이긴다(수정 완료).
새 크레이트를 사전에 추가할 때 **반드시 구 사전과 전량 대조**하라.

### 파일 쓰기는 **Write 도구로**
bash heredoc 으로 JSON·정규식을 쓰면 `\b`·`\n` 이 실제 제어문자가 돼 조용히 깨진다(세 번 밟았다).

---

## 4. 도구 목록

★**전체 인벤토리 = `MIG\TOOLS.md`** (103개, `mktools.py` 가 **자동 재생성**한다).
여기에 손으로 목록을 두지 않는다 — 이전에 7개만 적힌 채 썩어 있었다(§8 "사실 1개 = 1파일").

- **상황 → 어떤 도구**: `METHOD_MAP.md §0` 라우팅표
- **무엇이 있나**: `TOOLS.md` (분류별. ⛔폐기·⚠2차폴백 표시 포함)
- **새 도구를 만들면**: 파일 첫 docstring 줄을 제대로 쓰고 `python -X utf8 mktools.py` — 분류에 없으면 「미분류」로 뜬다

새 크레이트를 구사전에 넣으려면 `distruct.py`/`dienum.py` 상단 `IRDIRS` 에 경로 추가 후 `--build`.
(단 **1차 사전은 `tcxdict.py`** 다 — §6-b)

## 5. 이 방법으로 실제로 나온 것 (신뢰도 근거)

- game_ai 판단함수 **20개** 명세: 확정 오프셋 442 · 판정상수 186 · 조정 가능한 값 156
- 함수당 최대 **6번 독립 재작성** 후 교차검증 = **판정상수 일치 100%**
- 미확정 157 → **43**(그중 10건은 원리적 불가, 8건은 다른 함수 담당)
- 정본 = `MIG\_spec\specs20.json` / 문서 = `REPORT\tfm2_ai_adjust\06_판단함수_상세명세_20.md`

---

## 6. ★rustc-dev 커스텀 드라이버 (`tcx` 정본) — 2026-09-11 추가

`nightly-2026-05-24` 에 `rustc-dev` 가 설치돼 있어 **컴파일러 내부(`TyCtxt`)를 직접 질의**할 수 있다.
DWARF/IR 역산과 달리 **컴파일러가 직접 말하는 값**이므로, 충돌하면 **`tcx` 가 정본**이다.

### 도구
| 파일 | 하는 일 |
|---|---|
| `tcxbuild.ps1` | 드라이버 컴파일 (`-Src <드라이버.rs>`) |
| `tcxrun.ps1` | 드라이버 실행 (`-Exe <exe> -Crate <크레이트> -Out <경로>`) — PATH·`--sysroot`·SDK rlib 링크 전부 처리 |
| `_tcxdrv\tcxdump.rs` | ★전량 덤프 → `_tcx\<크레이트>.json` (아이템·span·가시성·MIR가용·시그니처·ADT·layout·줄 테이블) |
| `_tcxdrv\mirchk.rs` | MIR 가용성 집계 + fn 목록 TSV |
| `_tcxdrv\mirdump.rs` | `optimized_mir` 전량을 **문장별 span 포함**으로 덤프 |
| `tcxq.py` | 조회 (`items`/`lines`/`adt`/`grep`/`pub`) |
| `tcxverify.py` | 명세 핵심 오프셋 표적 검증 |
| `tcxfield.py` | 필드명/오프셋 역검색 |
| `tcxcross.py` | distruct/dienum 기계 대조 (`struct`/`enum`/`collide`) |
| `tcxpub.py` | pub API 표면 추출 |
| ★`_tcxdrv\tcxtg.rs` | **타입 레이아웃 그래프** 덤프 → `_tcx\tg_<크레이트>.jsonl` (아래 §6-b) |
| ★★`tcxdict.py` | **구조체/열거형 정본 사전** — `distruct.py`/`dienum.py` 의 대체품 (아래 §6-b) |
| ★`tcxaudit.py` | 이미 발표한 오프셋 주장을 사전으로 **전수 재확인** (아래 §6-c) |

---

### 6-b. ★★`tcxdict.py` — 구조체/열거형 **정본 사전** (2026-09-11 신설, `distruct`/`dienum` 대체)

~~`distruct.py`(구조체) + `dienum.py`(열거형) 부터 친다~~ → **`tcxdict.py` 부터 친다.**
구사전은 DWARF 역산이라 **키가 leaf 이름 하나뿐**이었고, 그래서 동명 타입이 서로를 조용히 덮어썼다.

```bash
cd /c/tfm2mods/MIG
python -X utf8 tcxdict.py --build                 # 사전 생성 (~2초, 12MB)
python -X utf8 tcxdict.py Entity                  # 최상위 필드(오프셋순)
python -X utf8 tcxdict.py Entity --deep           # 절대 오프셋으로 전개(중첩 관통)
python -X utf8 tcxdict.py Entity 0x628            # ★오프셋 → 필드 (→ stat_cached.hp)
python -X utf8 tcxdict.py --enum SubPlan          # variant + 논리인덱스 + **실제 메모리태그**
python -X utf8 tcxdict.py --enum SubPlan 5        # 메모리태그 5 = 무엇인지 + 페이로드
python -X utf8 tcxdict.py --ambig                 # 동명 다중(모호) 전량
python -X utf8 tcxdict.py --stats                 # 통계 + 구사전 커버리지
```
재생성이 필요하면(패치·SDK 교체 시) 먼저 덤프부터:
```powershell
powershell -File C:\tfm2mods\MIG\tcxbuild.ps1 -Src C:\tfm2mods\MIG\_tcxdrv\tcxtg.rs
# 6개 크레이트 (game_ai/game_core/game_view/engine_core/engine_ui/common) — 총 ~20초
powershell -File C:\tfm2mods\MIG\tcxrun.ps1 -Exe C:\tfm2mods\MIG\_tcxdrv\tcxtg.exe -Crate game_core -Out C:/tfm2mods/MIG/_tcx/tg_game_core.jsonl
```

**구사전과 무엇이 다른가 (전부 실측)**
| | `distruct`/`dienum` (DWARF 역산) | `tcxdict` (rustc TyCtxt) |
|---|---|---|
| 키 | **leaf 이름 하나** → 동명 충돌 시 조용히 덮어씀 | **full def_path**. leaf 는 별칭 인덱스, 모호하면 **"모호"로 후보 전량 반환** |
| 커버리지 | 게임 struct leaf 의 **31.8% 누락** / enum 의 **39.7% 누락** | 6개 크레이트의 **모든 ADT**(게임 struct 1,643 · enum 398 전량) |
| `Vec`/`String`/배열/`Option` | 통짜 또는 관례 추정 | **컴파일러가 계산한 실제 오프셋**(`Vec{buf.ptr@0, cap@8, len@0x10}` 등) |
| 열거형 태그 | 메모리 태그만(그 사실이 안 적혀 있어 혼동) | **논리 인덱스 · 선언 discr · 메모리 태그**를 셋 다, 니치 파라미터까지 |
| 소스 위치·가시성 | 없음 | 모든 타입에 `파일:줄` + `pub`/`in:모듈` |
| ⚠함정 | **키 429개가 실제로는 "열거형 variant"** — 독립 struct 로 읽으면 페이로드만큼 밀린다 | `--ambig` / variant 경고로 그 자리에서 알려준다 |

**전개 규칙**(사전이 중첩을 어디까지 펴는가 — 조회 결과를 해석할 때 필요)
- 재귀 진입: 단일 variant ADT(표준 라이브러리 포함) · 튜플 · 배열 · 열거형
- 열거형은 variant 마다 `@Variant` 세그먼트를 붙이고 태그는 `@tag`
- 배열: 원소 ≤ 8 이면 `[i]` 전부 전개, 더 크면 목록에선 `[0..N] stride=S` 요약 /
  **오프셋 조회에선 `i=(want−base)//stride` 로 해당 원소만 관통**(`MapDef 0x12b8` → `walls[19][14]`)
- **참조/생포인터/`dyn`/클로저에서 멈춘다**(다른 객체 경계) · 최대 깊이 6 · 순환 가드

**한계 (적용 범위를 적는다)**
- **이 방식으로는** SDK rlib 밖의 타입을 모른다. `mod_api` 는 ADT 0건으로 나왔다.
- **비공개·비인라인 함수의 지역변수 전용 모노모피 인스턴스는 못 잡는다** — rmeta 에 그 MIR 이 애초에 없기 때문(재료 부재). 실측 사례 = `best_jungle_goal` 의 `bumpalo Vec<JungleType>`.
  미탐색 = exe DWARF(=`distruct`) 로 그 판을 보는 것.
- vtable 슬롯은 구조체가 아니다 → 그건 계속 `divtable.py` 소관.

### 6-c. `tcxaudit.py` — 발표한 오프셋 주장 전수 재확인
```bash
python -X utf8 tcxaudit.py                                   # specs20 + resolved 의 구조화 표
python -X utf8 tcxaudit.py --prose <문서.md> [<문서2.md> …]    # + 산문 안 `타입+0xNNN` 까지
```
판정 = `OK` / `오귀속` / `밀림`(동명 struct↔variant 로 +N 어긋남) / `부분일치` / `확인불가`.
2026-09-11 실측(0.5.8): **778건 중 OK 754 · 오귀속 4 · 밀림 0 · 부분일치 2 · 확인불가 18(전부 vtable 슬롯)**.
자기검증(일부러 틀린 입력 8건)에서 알려진 오귀속·밀림을 전부 잡는 것을 확인했다.

레시피(3크레이트 전량 재덤프, 총 ~35초):
```
powershell -File C:\tfm2mods\MIG\tcxbuild.ps1 -Src C:\tfm2mods\MIG\_tcxdrv\tcxdump.rs
powershell -File C:\tfm2mods\MIG\tcxrun.ps1 -Exe C:\tfm2mods\MIG\_tcxdrv\tcxdump.exe -Crate game_ai   -Out C:/tfm2mods/MIG/_tcx/game_ai.json
powershell -File C:\tfm2mods\MIG\tcxrun.ps1 -Exe C:\tfm2mods\MIG\_tcxdrv\tcxdump.exe -Crate game_core -Out C:/tfm2mods/MIG/_tcx/game_core.json
powershell -File C:\tfm2mods\MIG\tcxrun.ps1 -Exe C:\tfm2mods\MIG\_tcxdrv\tcxdump.exe -Crate game_view -Out C:/tfm2mods/MIG/_tcx/game_view.json
```

### 이걸로 되는 것 / 안 되는 것
**되는 것**: 모든 아이템의 `def_span`(파일:줄:칸) · 구조체/열거형 필드·타입·**정확한 오프셋(`layout_of`)** · **니치 태그 인코딩**(`untagged_variant`/`niche_variants`/`niche_start`) · 선언 가시성 · fn 시그니처 · 파일별 줄 바이트 길이 · **`xinl=1`/제네릭 함수의 `optimized_mir`**(비교 연산자·상수 보존).

**안 되는 것**: **소스 원문**(`-Z embed-source` 없이 빌드돼 rmeta 에 텍스트 없음) · **비인라인 private 함수의 MIR**(rmeta 에 애초에 미인코딩 — `cross_crate_inlinable==false && 비제네릭` 이면 없다).

### 함정 (전부 실제로 밟았다)
1. 실행 시 툴체인 `bin` 을 PATH 에 넣어야 `rustc_driver-*.dll` 이 로드된다.
2. `--sysroot` 명시 필수.
3. `visibility`/`is_mir_available` 는 **DefKind 화이트리스트로 가드**(LifetimeParam 등에 부르면 ICE, 9,935 에러 남).
4. `layout_of` 전에 `tcx.erase_and_anonymize_regions` → **그 다음** `has_param()`. `has_param()` 이 `HAS_RE_PARAM` 을 포함해서 순서가 틀리면 lifetime 제네릭 타입이 전부 빠진다.
5. API 는 추측하지 말고 `…\lib\rustlib\rustc-src\rust\compiler\` 를 grep 할 것(이 nightly 에서 `instantiate_identity()` → `Unnormalized`, `v.field_offsets`, `normalized_source_len` 등이 바뀌어 있다).

### 기존 사전과의 관계 (2026-09-11 교차검증 결과)
- `dienum.json` = **정합(239/239, 모순 0)**. 단 그 값은 **메모리 태그**이지 variant 인덱스가 아니다. 변환식 `메모리태그 = niche_start + (variant_index − niche_variants.start)`, `untagged_variant` 는 태그 없음. DWARF 가 i64 부호付로 적어 `-1`/`-9223372036854775808` 로 보이는 것은 같은 비트패턴.
- `distruct.json` = **산술은 건전**하나 **키가 leaf 이름 하나**라 동명 타입이 서로를 덮어쓴다(대조 1,533건 중 **139건 동명 다중**, 그중 9건은 라이브러리 타입이 게임 타입을 밀어냄). 조회 결과는 `size` 를 tcx 와 대조할 것.
- 커버리지: 게임 구조체 1,659개 중 **534개(32.2%)가 distruct 에 없음**, 열거형 400개 중 **162개(40.5%)가 dienum 에 없음**. 없으면 `_tcx` 를 봐라.
- ★**2026-09-11 처분 확정**: `distruct.py`/`dienum.py` = **폐기하지 않되 "2차 폴백"으로 강등**. 1차는 `tcxdict.py`(§6-b).
  근거(실측) — distruct 키 26,416개 중 **91.4%(24,150)가 제네릭 인스턴스**, 1,198만 tcx leaf 와 겹치고,
  **388개는 실제로는 열거형 variant 이름**(독립 struct 로 읽으면 밀린다), 680개는 png/zlib 등 라이브러리 타입이다.
  반대로 **tcx 게임 leaf 2,041개 중 916개(44.9%)가 distruct 에 아예 없다**(엔진은 126개 중 109개).
  구사전을 남기는 이유는 딱 하나 — **exe/DWARF 쪽 증거원이라 SDK rlib 밖(지역변수 전용 모노모피 인스턴스·`vtable_type$`)을 볼 수 있다.**
  다만 그 제네릭 인스턴스가 감싸는 내부 타입은 **1,030종 중 1,008종(97.9%)이 이미 tcxdict 에 있고**, 없는 22종은 전부 트레이트/클로저(=ADT 아님)다.
  ⟹ **충돌하면 tcx 가 정본.** 구사전은 "tcx 에 없을 때만" 본다.
- ★정정: `MapDef +0x12b8 width` / `+0x12c0 height` 는 **오귀속**. 실제는 **`GameSetting.width`/`GameSetting.height`** (MapDef 의 0x12b8 은 `walls[19][14]`).

상세 = `REPORT\tfm2_ai_adjust\RE\2026-09-11_rustc-dev-커스텀드라이버-tcx덤프-MIR가용성판정-0.5.8.md`

## 7. ★★SDK 함수를 **직접 실행**하는 오프라인 오라클 (2026-09-11 신설 — 이 툴킷에서 가장 확장성이 크다)

> **한 줄**: SDK rlib 은 멤버가 날 LLVM 비트코드라 그냥은 실행파일로 링크되지 않는다(`LNK1136: 파일이 잘못되었거나 손상되었습니다`).
> **`-C lto=fat -C codegen-units=1` 을 붙이면 rustc 가 비트코드를 직접 병합해 exe 가 나오고,
> 게임 SDK 함수가 우리 프로세스 안에서 진짜로 실행된다.**

그동안 `spanprobe.ps1` 은 `--emit=metadata`(타입체크)까지만 됐다. 이 플래그 하나로 **`pub` 인 것은 무엇이든
전수 진리표로 검증**할 수 있게 됐다. IR 독해가 맞았는지를 *실행 결과*로 대조하는 길이다.

### 왜 §3 "완전 재구현 원칙"에 저촉되지 않나
이건 **오프라인 오라클**이다 — 게임 프로세스에 붙어 런타임에 게임 헬퍼를 부르는 것이 아니라,
우리 프로브 바이너리 안에서 SDK 함수를 불러 **표를 검증**하는 것이다. 목적이 재현 대체가 아니라 **검증**이다.
⚠단 **모드 코드에 이 경로를 넣으면 그건 §3 위반**이다. 검증 전용으로만 써라.

### 레시피
```
powershell -ExecutionPolicy Bypass -File C:/tfm2mods/MIG/spanprobe.ps1 -Src <프로브.rs> -Extra "--extern bumpalo=C:/tfm2mods/sdk_058/mod-sdk/deps/libbumpalo-<해시>.rlib -C lto=fat -C codegen-units=1 -C opt-level=1"
```
산출물 = `%TEMP%\tfm2_spanprobe\<파일명>.exe` → 실행해서 TSV 로 받는다.

- `-ExecutionPolicy Bypass` 없으면 `spanprobe.ps1` 이 실행정책에 막힌다.
- ⚠**`rand` 는 rlib 을 골라야 한다** — `librand-e2a5dd20f067a3a7` = 0.8.5(맞음), `99c6…` = 0.9.1 이라 타입 불일치.
- 실동작 표본 = `MIG\_oracle\` (프로브 `o_main.rs` / `o_plan.rs` / `o_disc.rs`, 진리표 13개, 전체 레시피는 `_oracle\README.md §0`).

### ★인자를 만드는 법 — 제로버퍼 캐스팅은 대개 필요 없다
`GameContext`(64B)는 **전 필드 pub** 이고 `MapDef::moba(&GameSetting)` 이 pub 이라 **정상 생성된다**(UB·크래시 0건 실측).
`pool` = `Bump::new()` / `setting`·`macro_weights`·`map_setting` = `Default` / `map` = `MapDef::moba(&setting)` / 나머지 `Vec`·`bool`.
`tutorial` 은 **+0x38(=56)**, MIR 상 `(*ctx).9`.

★**먼저 `tcx` 덤프(`_tcx\*.json`)의 `v`(가시성)와 `sig` 로 "만들 수 있는 인자"인지 판정하라.**

⚠**제로버퍼 캐스팅은 최후수단**이고, 쓸 거면 두 조건을 먼저 확인해야 한다:
1. 그 함수가 그 필드 **외에는 아무것도 안 읽는다**는 것을 IR/MIR 로 확인 — 아니면 결과가 **조용히 쓰레기**가 된다
2. 열거형 필드에 **0 이 유효한 판별자**인지 `tcxdict --enum` 으로 확인 — 니치 최적화 때문에 UB 가능

### ★오라클 결과를 자동으로 믿지 마라
실행 결과는 강력하지만 **입력을 잘못 구성하면 조용히 틀린다.** IR 표와 어긋나면
**①입력 구성이 맞았나 ②IR 독해가 맞았나 양쪽을 다 의심**하고 근거를 대라.

그리고 **오라클이 원리적으로 구분 못 하는 것이 있다**(= 틀린 게 아니라 「표기 불가」):
**외연이 같은 두 표현은 실행으로 절대 안 갈린다.** 실측 사례 —
`chat_allowed` 태그 20/21/22 의 `spawn_epic` vs `spawn_epic && line_exists`(`{0,7,8}` ⊆ 모든 `line_exists` 집합이라 동일),
`position_exists` Jungle 의 `player_count()==5` vs `>=5`(최댓값이 5). **이런 건 IR 로만 갈린다.**

### 실측 성과 (신뢰도 근거)
`rule_scope` pub 13개 전수 진리표 → IR 독해로 만든 표 (A)~(E) 대조 **틀린 칸 0**.
덤으로 `valid_lines`(9행)·`fallback_line`(27칸)·`steal_target/action_allowed`·`main/sub_objective_allowed` 가 **새로 완전 규정**됐고,
`position_exists` 의 `Support` arm 이 `Bottom` arm 과 공용이라는 **표에 없던 칸**이 나왔다.
비-pub 3함수(`line_from_code`/`push_line_code_allowed`/`play_code_allowed`)도 **간접 규정**됐다.

### 다음에 이 방법을 쓸 곳
`tcx` 가시성 실측 결과 **`game_ai::plan_legacy`(+`old`/`sub_plan`/`rule_scope`/`types`/`team_plan`/`handler`/`steal`/`action_eval`)가 pub** 이다.
반대로 `small_action`/`path_finder`/`position_eval`/`score_parameter`/`fight_check`/`buff_value` 는 `pub(crate)` 라 이 경로가 막힌다.
⚠**정정(2026-09-11 검증배치 D)**: ~~"인자에 `&OperationData`·`&PlayerState` 가 들어가면 구성 불가"~~ 는 **거짓이었다.**
`Game::new` → `add_player(GamePlayer::new(…, Arc::new(SwordmanChampionInfo::default()), …))` → `start_game` →
`AbstractGameWithCache::new` → `OperationData::new(&cache, &ctx, &[Blackboard::default(); 2])` 가 **전부 pub** 이라 정상 생성된다
(실행·링크 성공, 크래시·UB 0). `&PlayerState` = `game.get_player_by_position(team,pos)`, `&Entity` = `cache.player_champion[t][p]`.
⟹ **`pub(crate)` 가 아닌 game_ai 판단함수는 사실상 전부 오라클 대상**이다(실증: `best_jungle_goal` 10/10 MATCH).

★**남는 진짜 한계는 인자가 아니라 데이터다**:
- `GameSetting::default()` 의 **`tick_per_second = 0`**(실전 60) — `udiv by tps` 가 있는 함수는 **손으로 60 을 세팅**하라
- `SwordmanChampionInfo::default()` 는 **이펙트가 비어** 있어 `max_range_nearly_can_use` 가 전 구간 0 — 함수가 틀린 게 아니라 **입력에 판별력이 없다**. 실전 챔피언 데이터(`ChampionInfoSheet`/`Assets`) 로딩은 **미탐색**
- `pub(crate)` 모듈(`fight_check`/`position_eval`/`path_finder`/`score_parameter`/`buff_value`/`small_action`)은 종전대로 막힘
- ⚠`engage_requires_dive` 는 `game_ai::tower_discipline::` 로 못 부른다(모듈 private) — **`game_ai::engage_requires_dive`** 로 재수출됨

## 8. ★★「줄 길이 산술」 — 소스 본문이 없어도 한 줄을 문자 단위로 복원한다 (2026-09-11 확립)

> **한 줄**: `rmeta_srcmap` 의 **줄별 문자 길이** + `_tcx` 의 **`sp`(줄:칸)** + **tcx ADT 필드명·태그** + **IR 분기 유무** 를
> 겹치면, 소스 원문이 없어도 **그 줄에 무엇이 적혀 있었는지**를 ±0자로 맞출 수 있다.
> 실측: 한 배치에서 **정확 일치 복원 30줄 이상**.

### 왜 되는가
후보 문장을 만들면 **길이가 곧 검산식**이다. 타입·필드명·열거형 variant 이름이 `tcx` 로 확정돼 있으므로
후보의 자유도가 매우 낮고, 길이가 ±0으로 떨어지면 사실상 유일해가 된다.

### ★필수 보정 2개 (모르면 전부 어긋난다)
1. **`rmeta_srcmap` 의 `bytes`/`chars` 는 LF 1자를 포함한다** ⟹ **실제 내용 길이 = 값 − 1**.
   (검증: 빈 줄이 `chars=1` 로 나온다.)
2. **게임 소스는 2‑스페이스 들여쓰기다.** `_tcx` 의 `sp.c` 로 교차검증하라 —
   `chat.rs:8 c=3` · `handler.rs:362 c=3` · `epic.rs:634 c=3` = `fn` 이 3칸째 ⟹ 들여쓰기 2.
   **4칸으로 가정하면 전 줄이 어긋난다.**

### 절차
1. `python rmeta_srcmap.py <크레이트> <경로부분> <a> <b>` 로 줄 길이표를 뽑는다
2. `_tcx\*.json` 에서 그 구간에 걸린 아이템의 `sp`(줄:칸)로 **들여쓰기 깊이**를 고정한다
3. `tcxdict` 로 등장하는 타입의 **필드명·variant 이름·태그**를 확정한다(후보 자유도를 없앤다)
4. IR 에서 **그 줄의 `!dbg` 유무·분기 유무·스토어 대상**을 본다
   ★**분기가 없다 = 그 줄은 `if` 가 될 수 없다** — 이게 결정타가 되는 경우가 많다
5. 후보 문장을 쓰고 **길이를 검산**한다. ±0 이면 확정, 1~2자 남으면 잔차로 명시

### 실전 사례
- `epic.rs:654~657` — 세 스토어가 전부 `!dbg 654` 단일 + 사이에 분기 없음 ⟹ 655 는 **계속행**.
  `MainObjective::Serpen{phase, with_battle}` 가 네임드 필드라 표기가 강제되고 `ObjectPhase::Setup`(5자)이 길이를 맞춘다
- `epic.rs:674/676` — IR 의 `select(is_none(), 21, 22)` ⟹ `Chat::Press` / `Chat::PressChange`.
  **두 줄 길이차 6 = `len("PressChange") − len("Press")`** 로 자기정합
- `chat.rs:20` 이 **13자** ⟹ `return self.handle_chat_inner(..)` 는 물리적으로 불가 ⟹ 맨몸 `return;`
- `SubPlan::merge` 4 arm — `줄길이 − 2×len(name) = 63` 이 네 줄 모두 일치 ⟹ arm 배정 확정
- `rule_scope.rs:96` — `BigGoal::Nexus{..} | Battle{..} | Recall => true,` = **77자 검산 일치**

### 언제 쓰나 / 한계
- **쓸 때**: MIR 이 없는(`mir=0`) 함수의 소스 표기가 필요할 때. 즉 **`§`7 오라클도 `§`6 MIR 도 막힌 자리**.
- **한계**: ①**한글 `//` 라인주석은 rmeta 에 없다**(문서주석 `///` 만 있다) — 주석 원문은 **재료 부재**
  ②동일 길이의 다른 표기를 **원리적으로 못 가른다**(후보가 둘 이상 같은 길이면 잔차로 남긴다)
  ③후보를 못 떠올리면 못 푼다 — **tcx 로 자유도를 먼저 줄이는 게 전제**다

★**교훈**: 2026-09-11 배치에서 `epic.rs` 5줄을 **"재료 부재, 남은 건 exe 디스어셈뿐"** 으로 닫았다가
이 기법으로 **4줄이 뚫렸다.** *"남은 건 X뿐"* 이라고 쓰기 전에 **가진 재료의 조합**을 다 시도했는지 보라.
