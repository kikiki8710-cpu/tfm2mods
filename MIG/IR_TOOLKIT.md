# TFM2 IR 툴킷 — SDK rlib 에서 원본 소스급 정보를 꺼내는 법

> **한 줄**: TFM2 SDK 의 rlib 들은 `-C embed-bitcode` 로 빌드돼 **아카이브 멤버 자체가 날 LLVM 비트코드**다.
> `llvm-dis` 만 돌리면 **디버그정보가 붙은 원본 IR**(심볼·구조체 필드명·소스 파일·줄번호)이 통째로 나온다.
> **Ghidra 디컴파일보다 훨씬 정확하다.** exe 에서 못 읽던 것이 여기선 읽힌다.

이 문서는 **AI 판단 계층 전용이 아니다.** UI·시뮬레이션·에셋 등 **어느 부분이든** 같은 방법이 통한다.
작성 2026-09-10 / 게임 0.5.8 / SDK `C:\tfm2mods\sdk_058`.

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

| 파일 | 하는 일 |
|---|---|
| `rlib2ll.py` | rlib → `.ll` 추출. `--list` 로 후보 확인 |
| `distruct.py` | 구조체 오프셋 → 필드 사전(26,416개). `--build` 로 재생성(59초) |
| `dienum.py` | 열거형 태그 ↔ variant + 페이로드 사전(7,016개). `--build`(57초) |
| `fnparts.py` | 한 함수의 조각 전부(클로저·심·모노모픽) 위치 |
| `divtable.py` | `dyn Trait` vtable 슬롯 → 함수 이름 |
| `qcspec.py` | 명세 기계 검증(C1~C5) — 명세를 쓸 때만 |
| `SPEC_GUIDE.md` | 명세 작성 지침 전문 — **새 조사 배치를 띄우기 전에 읽힐 것** |

새 크레이트를 사전에 넣으려면 `distruct.py` / `dienum.py` 상단의
`IRDIRS = [d for d in (IRDIR, COREDIR) if os.path.isdir(d)]` 에 경로를 추가하고 `--build`.

---

## 5. 이 방법으로 실제로 나온 것 (신뢰도 근거)

- game_ai 판단함수 **20개** 명세: 확정 오프셋 442 · 판정상수 186 · 조정 가능한 값 156
- 함수당 최대 **6번 독립 재작성** 후 교차검증 = **판정상수 일치 100%**
- 미확정 157 → **43**(그중 10건은 원리적 불가, 8건은 다른 함수 담당)
- 정본 = `MIG\_spec\specs20.json` / 문서 = `REPORT\tfm2_ai_adjust\06_판단함수_상세명세_20.md`
