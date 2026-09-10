# 함수 명세 작성 지침 (game_ai IR 독해)

TFM2 게임의 AI 판단 계층(`game_ai` 크레이트)을 **LLVM IR 원본**에서 읽어 함수 명세를 만든다.
목표는 "이 함수가 무엇을 어떤 조건으로 판정하는가"를 **나중에 사람이 읽고 개조 지점을 고를 수 있을 만큼** 적는 것.

## 0-0. ★먼저 이 네 줄만 지켜도 절반은 간다

1. **오프셋은 `reads`(읽기)/`writes`(쓰기)에만.** `constants` 는 판정에 쓰이는 값만 — 자세한 표는 §3.
2. **모르면 `unknown`.** 같은 것 2회 시도해서 안 되면 즉시 내려놓아라. 감점 아니다.
3. **도구 넷을 먼저 쳐라** — `distruct`(구조체) `dienum`(열거형) `fnparts`(클로저 위치) `divtable`(vtable 슬롯). §1.
4. **JSON 은 Write 도구로 써라.** heredoc 은 깨진다.

## 0. 대전제 — 추측을 쓰지 마라

이 작업의 가치는 **정확성**에 있다. 분량이 아니다.
- 확실한 것만 단정형으로 쓴다.
- 애매하면 `unknown` 배열에 **무엇을 왜 모르는지** 적는다. 모르는 걸 적는 건 감점이 아니라 **필수**다.
- 추측을 사실처럼 쓰면 QC에서 걸리거나, 더 나쁘게는 안 걸리고 남아서 나중에 잘못된 판단의 근거가 된다.
- ⚠**이름으로 성격을 단정하지 마라.** 실제 사고: `path_finder` 라는 이름 때문에 "길찾기니까 AI 판단과 무관"으로 분류했는데, 실제로는 그 결과 좌표가 판단 게이트의 입력이었다. **호출 관계와 본문으로만 판단할 것.**

## 1. 재료

### ★★먼저 이것부터 — `distruct.py` (오프셋 사전)
**1차 배치에서 담당자 20명 전원이 같은 데서 시간을 잃었다: 시간의 60~70%가 "이 `+0x8c` 가 무슨 필드인가"를 알려고 `!N → 멤버 → 타입 → variant → 멤버` 를 손으로 5~6단 타는 데 갔다.** 그건 이제 사전 조회다.

```bash
cd /c/tfm2mods/MIG
PYTHONIOENCODING=utf-8 python distruct.py MapDef              # 구조체 전체 필드
PYTHONIOENCODING=utf-8 python distruct.py Entity 0x640        # 오프셋 → 필드 역조회
PYTHONIOENCODING=utf-8 python distruct.py LegacyPlanHandler
```
구조체 7,355개 · 필드 17,003개가 들어 있다(전 IR 1회 스캔). **DWARF 를 손으로 타기 전에 반드시 여기부터 찾아봐라.**

★**오프셋을 같이 주면 중첩을 끝까지 뚫는다**(절대 오프셋으로 답한다). 6차 신설로 이제 뚫는 범위:
- 중첩 구조체: `distruct PlayerState 0x930` → `info.team`
- **배열 원소 타입**: `array$<enum2$<..Option<&Entity> > >(총 16B)`, 중첩 배열도 `array$<array$<usize>(총 240B)>(총 7200B)` (~~`array$<?>`~~ 는 없어졌다)
- **`Vec`/`String` 내부**: `distruct MobaMode 0x1a8` → `live_list.len`, `0x1a0` → `live_list.buf.inner.ptr`
- 같은 이름의 **열거형**이 있으면 `dienum` 으로 넘겨준다(`Chat`·`CastingTarget`·`CancelReason`).
  후보가 전부 `vtable_type$`/표준라이브러리면 "게임 타입이 아니다"라고 말해 준다.
⚠단 **오프셋 없이** 타입만 물으면 최상위 필드 목록만 나온다. 중첩이 필요하면 오프셋을 줘라.

### ★★`fnparts.py` (함수 조각 위치) — **본문 읽기 전에 먼저 쳐라**
1~3차 `unknown` 478건 중 **64건(13.4%)이 "판정의 알맹이가 담당 범위 밖"** 이었다.
이터레이터 술어는 담당 줄범위엔 `call_mut` 심만 남고 본체가 **다른 `.ll`** 에 있다.
```bash
PYTHONIOENCODING=utf-8 python fnparts.py defensive_crisis
```
→ 클로저·이터레이터·`call_mut` 심의 **파일과 줄범위**를 전부 찍어준다.
⚠"본문에 술어가 없다"고 결론내기 전에 **반드시** 이걸 돌려라.
- 조각이 담당 함수의 것이면 그 범위를 **`aux` 로 선언**하면 QC 가 같이 검증한다(§ "담당 범위 밖" 참조).
- ★**소유 타입 열을 봐라.** 같은 이름의 메서드가 다른 타입에 있으면 그걸 읽고 명세에 박는 사고가 난다
  (5차 실측: `LineGankCoverPlan::target_bush_v30` ≠ 담당 함수의 `LineGankerPlan::target_bush_v30`).
  ~~"소유 타입 2종" 경고가 뜨면 조각을 버려라~~ → **6차 정정**: 그 경고가 망글 백레퍼런스 오파싱으로
  가짜(`e_y`·`U_IN`)를 내던 게 고쳐졌다. 이제 소유자는 이름 바로 앞 성분으로 잡는다.

### ★★`divtable.py` (dyn 트레이트 vtable 슬롯) — "원리적으로 불가"가 아니다
`unknown` 478건 중 **36건(7.5%)이 vtable 슬롯 이름**이었고 대부분 "game_core DWARF 가
없어 원리적 불가"로 포기했다. **틀렸다.** vtable **전역 상수**에 함수포인터가 순서대로 들어 있다.
```bash
PYTHONIOENCODING=utf-8 python divtable.py AbstractGame 0x28    # → tick
PYTHONIOENCODING=utf-8 python divtable.py AbstractGame 0x100   # → is_visible_cell
PYTHONIOENCODING=utf-8 python divtable.py AbstractGame 0x1f0   # → get_entity_by_id
```

### ★★그리고 `dienum.py` (열거형 태그 사전)
`distruct.py` 가 구조체를 맡고, **열거형은 이쪽**이다. 2차 배치 담당자 거의 전원이
"구조체는 즉답인데 열거형은 여전히 손"이라고 보고해 만들었다.
```bash
PYTHONIOENCODING=utf-8 python dienum.py SubPlan        # 태그 전표
PYTHONIOENCODING=utf-8 python dienum.py SubPlan 5      # 태그 5 가 무엇인지
```
★**태그를 주면 페이로드 레이아웃까지 나온다**(5차 신설). 4차에서 3명이 독립적으로 요청했고,
`MainObjective` 를 "byte1 이 태그" 로 오독한 사고의 직접 원인이던 공백이다.
```bash
PYTHONIOENCODING=utf-8 python dienum.py MainObjective 0
#   태그 0 → Morgard
#   페이로드 Morgard — 페이로드 시작 enum+0x0 / 아래는 **enum 선두 기준 절대 오프셋**
#     enum+0x1    phase        ObjectPhase
#     enum+0x2    with_battle  bool
```
⚠**태그 값**과 **태그 위치**는 별개 함정이다. 3바이트 열거형에서 어느 바이트가 태그인지
가정하지 마라 — 위 출력의 `enum+오프셋` 이 절대 기준이다.
- "페이로드 시작 enum+0x0" 인데 첫 필드가 `+0x1` 인 것은 정상이다(0번 바이트가 태그).
  6차에 두 명이 "한 칸 밀렸나" 하고 되짚어서 문구를 못 박았다. **필드 줄의 오프셋이 절대값**이다.
- 필드는 **오프셋 순으로 정렬**돼 나온다(6차 신설 — 전엔 DWARF 선언 순이라 뒤섞여 보였다).

⚠**사전에 없거나 미심쩍으면 DWARF 로 확인하라.** 이 사전은 손을 줄이는 도구지 면제권이 아니다.
아래 "태그 ≠ variant 인덱스" 규칙은 **그대로 유효**하다.

### 도구 4종의 알려진 한계 (5차 시점 · 이걸 넘어서려 시간 쓰지 마라)
- **`divtable` 은 `Arc<dyn Trait>` 처럼 런타임에 들어오는 vtable 에는 무력**하다. `_gaibc` 에
  그 vtable 전역이 아예 없다(`EffectType` 이 그 예). 정적 `@vtable` 참조에만 통한다.
  ⚠일치율이 헤더에 찍힌다 — **50% 미만이면 도구가 거부**한다. 억지로 다른 이름으로 캐지 마라.
- **`fnparts` 는 `define` 이 있는 조각만** 찾는다. "DWARF 서브프로그램 N개 vs define M개"
  줄이 뜨면 나머지는 **전부 인라인**이라는 뜻이다 — 더 뒤지지 마라.
- **`distruct` 는 동명 타입에 약하다**. `Map` 을 물으면 `core::iter::Map` 이, `Chat` 을 물으면
  32B 구조체판이 나올 수 있다. **크기(IR 의 `dereferenceable(N)`)로 반드시 교차검증**하라.
- 상수 접힘(`x*4` → `shl 2`, 인라인으로 사라진 비교식)은 **원리적으로 복원 불가**다.
  관측 사실만 적고 `unknown` 으로 내려라.
⚠한계: ① 이름이 같은 구조체가 여러 판 있으면 **필드가 가장 많은 판**을 담았다 — 크기가 안 맞으면 손으로 확인하라 ② 배열 필드의 **원소 타입**은 아직 `array$<?>` 로 남는다(총 바이트만 정확하다 — 원소 타입은 IR 의 `[N x T]` 로 확인하라) ③ `Vec`/`String` 은 통짜 24B/32B 로만 나온다(내부 `ptr`/`len`/`cap` 3필드 미노출 — `+0x8=ptr, +0x10=len` 관례를 IR 사용 패턴으로 확인하라).

  ★~~"중첩 구조체는 안 펼쳐져 있다"~~ → **거짓. 3·4·5·6차에서 5번 지적됐다.**
  `distruct <타입> <오프셋>` 형태로 **오프셋을 같이 주면** 중첩을 끝까지 뚫고 **절대 오프셋**으로 답한다
  (`distruct PlayerState 0x930` → `info.team`). 손으로 뺄셈하지 마라 — 실제로 그렇게 낭비한 사례가 있다.
  단 **오프셋 없이** 타입만 물으면 최상위 필드만 나온다. 그건 목록 조회지 한계가 아니다.

- IR 원본 = `C:\tfm2mods\_gaibc\*.ll` (24개, 총 303MB — **절대 통째로 읽지 마라**)
- ★★**`game_core` 본문도 있다 — `C:\tfm2mods\_gcbc\g00.ll ~ g15.ll` (16개, 총 992MB).**
  ~~"game_core 는 별도 크레이트라 본문이 없다 ⟹ 확인 불가"~~ 는 **거짓이다.** 1~6차 명세에서
  최소 9건이 이 이유로 `unknown` 에 실렸는데, `_gcbc` 는 2026-09-08부터 있었고 이 문서가
  가리키지 않았을 뿐이다. `_gaibc` 에 `declare` 만 있으면 **`_gcbc` 에서 `define` 을 찾아라**:
  ```bash
  grep -n "^define.*is_recent_visible" /c/tfm2mods/_gcbc/*.ll
  ```
  (`Blackboard::is_recent_visible` · `Effect::range_adjust` · `Entity::can_ult` ·
   `CastingTarget::check` · `expected_damage_target` · `MapDef::camp_pos` ·
   `JungleRunner::get_camp_state` 전부 여기 본문이 있다.)
  ⚠단 `_gcbc` 는 `_gaibc` 의 3배 크기다. **반드시 `grep -n "^define"` 로 줄을 찾고 그 범위만 `sed`** 로 읽어라.
- 담당 함수의 **정확한 줄범위**가 지시에 있다. 그 범위만 읽어라:
  ```bash
  sed -n '49611,50100p' /c/tfm2mods/_gaibc/m10.ll
  ```
- 메타데이터(`!DILocation` `!DISubprogram` `!DILocalVariable` `!DIDerivedType`)는 파일 **끝쪽**에 몰려 있다. 필요한 `!N` 만 골라 찾아라:
  ```bash
  grep -n '^!9505 = ' /c/tfm2mods/_gaibc/m10.ll
  ```
- 소스 파일명·줄번호는 `!dbg !N -> !DILocation(line:, scope:)` 사슬로 나온다. **이걸 꼭 쓰라** — 명세의 상수·분기에 소스 줄번호를 붙이면 가치가 몇 배가 된다.
- 참고(있으면 좋고 없어도 됨): `MIG\aimap.json`(IR↔RVA), `MIG\manual.json`(확정 이름), `MIG\knobscan.py` 출력(상수 인벤토리).

### ⚠이 문서의 예시 JSON은 **형식 예시일 뿐 사실 출처가 아니다**
아래 예시에 적힌 오프셋·상수·타입을 **그대로 베끼지 마라.** 반드시 담당 함수 본문에서 확인한 값만 쓴다.
(1차 배치 실사고: 예시에 `0x5c0 = team` 이라고 적혀 있었는데 **실제로는 `ENT_HANDLE`(엔티티 식별자)** 였다.
 한 작성자가 "아군끼리 team 을 비교하면 전원 걸러지니 모순"이라고 짚어 잡혔다. 예시를 의심한 게 맞았다.)

## 2. IR 읽는 요령

- `getelementptr i8, ptr %x, i64 1472` = `x + 0x5c0` **구조체 필드 접근**. 오프셋은 10진수로 나온다 — 16진수로 변환해 적어라(우리 사전이 16진수 기준).
- `icmp ugt i64 %a, 22500000001` 같은 큰 수는 **제곱 거리 임계**인 경우가 많다. `sqrt` 해봐라 — `22500000001 ≈ 150000²`. 이런 건 `meaning` 에 반드시 풀어써라.
- 게임의 좌표 단위: **셀 = 32000, 셀 중심 = +16000**. `32000`/`16000`/`30`(그리드) 는 임계값이 아니라 좌표 변환이다.
- `call ... @_RNv...` 의 망글 심볼은 Rust v0 망글이다. 읽기 어려우면 심볼 안의 **길이접두 이름 성분**을 그대로 뽑아 쓰면 된다(`...11fight_model13ally_is_bound` → `fight_model::ally_is_bound`).
- `phi` 로 모이는 상수들 = 서로 다른 **거부 사유**인 경우가 많다. 값이 다르면 분기가 다르다.
- `br i1 %c, label %A, label %B` 에서 `%c` 를 거슬러 올라가면 판정식이 나온다.
- 클로저(`closure$N`)가 별도 define 으로 있으면, 담당 함수가 그걸 부르는 경우 **그 클로저 본문도 같이 읽어라**(이터레이터 술어가 거기 있다).

### ★1차 배치에서 실제로 사람들이 막힌 3가지 (전부 여기서 시간을 잃었다)

1. **`switch` 로 접힌 술어 체인** — `is_jungle()` 같은 술어가 인라인되면 **판별자 비교가 switch 로 접혀 없어지고 페이로드 로드만** 남는다. 그러면 `dbg_value` 가 말하는 오프셋(예: 판별자 `+104`)과 본문이 실제로 읽는 오프셋(예: 페이로드 `+152`)이 어긋나 보인다. **오프셋을 잘못 읽은 게 아니다** — 어느 switch case 블록 안인지를 보고 "판별자==N 은 이미 확정된 문맥"으로 읽어라.
2. **곱셈이 `shl` 로 접힘** — `x * 4` 는 `shl i128 %x, 2` 로 나와 **리터럴 4가 본문에 없다.** 이때 QC C1을 통과시키려고 상수를 목록에서 빼지 마라. 다른 근거(루프 상한 등)로 등록하고 `meaning` 에 "shl 로 접힘"을 명시하거나, 근거가 없으면 `unknown` 에 적어라.
3. **else-if 체인이 블록으로 흩어짐** — 소스의 3단 `else if` 가 LLVM 에서 여러 진입 블록으로 쪼개진다. `phi` 를 역추적하고 **`!dbg` 의 소스 줄번호로 원래 순서를 복원**하라. 줄번호 없이는 분기 순서를 확정할 수 없다.

### 파일 쓰기 = **반드시 Write 도구**
bash heredoc(`cat > f <<'EOF'`)으로 JSON 을 쓰면 따옴표·긴 문자열 때문에 파서가 죽는다(1차 배치에서 **20명 중 10명 이상이 같은 데 걸렸다**). 처음부터 Write 도구를 써라.

### ★★★열거형 태그 = variant 인덱스가 **아니다** (1차 배치 최대 함정 · 3명이 독립적으로 밟았다)
`store i64 4` 를 보고 DWARF 의 `variant4` 로 읽으면 **틀린다.** 니치 최적화 때문에 판별자가 밀려 있다.
- 실측: `SubPlan` 은 `DISCR = variantIndex + 2` (tag 2=LineDefense, 5=Recall, 16=AttackNexus)
- 실측: `BigPlan` 도 Variant0(ForcePassive)=2, Variant2(SinglePlanLine)=4 (DeathMatchBattle 이 니치로 0..1 을 먹음)

**반드시 각 variant 의 `DISCR_EXACT` static member 의 `extraData: i64 N` 을 확인하라.** 그리고 **페이로드 레이아웃으로 교차검증**하라 — 태그 N 경로가 쓰는 바이트 수/위치가 그 variant 의 필드와 맞는지. 1차 배치에서 이 교차검증으로 오독 2건을 잡았다.

### 클로저는 **다른 `.ll` 파일에 있을 수 있다**
판정의 알맹이(이터레이터 술어)가 담당 줄범위 안에 하나도 없고 `declare hidden` 만 있는 경우가 잦다. **파일 하나만 보고 "술어가 없다"고 결론내지 마라.**
```bash
grep -n "^define.*<함수명>" /c/tfm2mods/_gaibc/*.ll
```
로 전 파일을 훑어라. 1차 배치에서 2명이 이걸로 헛돌았다.

### 담당 범위 **밖**에 있는 상수·호출 처리법 (규칙 통일)
★**6차 신설 — 먼저 `aux` 로 범위를 넓혀라.** `fnparts` 가 알려준 조각(`call_mut` 심·클로저·fold)이
담당 함수의 것이 확실하면, JSON 최상위에 **보조 범위**를 선언하면 그 안의 상수·호출도 `constants`/`calls`
에 정상 등록되고 **QC 가 기계 검증한다**:
```json
"aux": [{"ir_file": "m10.ll", "ir_from": 55868, "ir_to": 55990}]
```
(6차 실측: `defensive_crisis` 는 판정 상수의 1/3(30000·`max_range`·`is_ignored_well_enemy`·
`is_recent_visible`)이 필터 술어 심 안에 있어 **검증 없이** 남았다. 이제 검증된다.
보조 범위도 `define`~`}` 여야 한다 — 아니면 C4 가 반려한다.)

그래도 못 넣는 경우(다른 함수 소유가 확실한 조각 등)에만:
- `constants` **에서 빼고**, `knobs` 에 값 + **어느 파일 몇 줄에 있는지**를 싣고, `unknown` 에 "왜 constants 에 없는지"를 적는다.
- 절대 **그냥 지우지 마라.**

### 접힌 상수는 `folded_from` 으로 (6차 신설)
`tps * 2` 는 IR 에 `shl 1` 로 나와 리터럴 2가 없다. `value: 1` 만 적으면 상수 목록을 훑는
사람이 **"임계가 1"로 오독**한다. 소스 수준 값을 같이 실어라:
```json
{"value": 1, "folded_from": 2, "meaning": "tps*2 (= 2초). `shl i64 %tps, 1` 로 접힘"}
```
`folded_from` 을 적었으면 `meaning` 이 비면 안 된다(C1 반려). `shl` 피연산자로 보이는데
안 적었으면 **경고**가 뜬다.

### ★`!dbg` 가 없어 보여도 **`inlinedAt` 루트까지 타라** (2026-09-10 신설)
"소스 줄이 소실됐다 / 원리적으로 복원 불가"로 내린 판정이 **실제로 뒤집힌 사례가 있다.**
`!DILocation` 은 `scope:` 만 보면 인라인된 헬퍼(`push`·`Option::ne`·`iterator.rs`)를 가리켜
엉뚱한 파일·줄이 나온다. **`inlinedAt:` 를 끝까지 따라가면 원래 호출 줄이 나온다.**

```bash
# !14974 의 뿌리를 찾는다 — inlinedAt 이 없어질 때까지 반복
grep -n '^!14974 = ' /c/tfm2mods/_gaibc/m09.ll
#   !DILocation(line: 1, scope: !..., inlinedAt: !14770)   ← scope 만 보면 line 1 (쓸모없음)
grep -n '^!14770 = ' /c/tfm2mods/_gaibc/m09.ll
#   !DILocation(line: 676, scope: !...)                    ← ★루트 = epic.rs:676
```
실측: `v3_epicops_buff_window` 의 "655~663·671~678 복원 불가" 판정이 이 방법으로
**653·654·658·664~670·672·673·676·679 로 복원**됐다. 남은 것만 진짜 부재다.

⚠**단 `column:` 은 이 빌드에 아예 없다**(전 모듈 0건). 그래서 **같은 줄 안의 순서**
(`A || B` 의 A/B 상대 순서 등)는 여전히 원리적으로 복원 불가다. 줄번호와 줄 안 순서를
구분해서 판정하라.

### 확정 안 되면 **2회 시도 후 `unknown`**
1차 배치에서 가장 많이 낭비된 패턴 = "IR 과 디버그정보가 어긋나는 지점"을 끝까지 확정하려다 못 하고 결국 `unknown` 으로 내려놓기까지의 왕복. 대표 사례:
- 인라인만 존재해 `define` 이 없는 헬퍼(`morgard_exists` 등) → 극성(`if m` vs `if !m`) 확정 불가
- ~~`game_core` 트레이트의 vtable 슬롯 이름 → 그 크레이트 DWARF 가 `_gaibc` 에 없어 원리적으로 불가~~
  → **2026-09-10 정정: `_gcbc\g*.ll` 에 game_core 본문·DWARF 가 전부 있다.** 위 §1 참조.
  거기서 `define` 을 찾고, 정적 vtable 전역이 있으면 `divtable` 대신 직접 슬롯을 세라.
- 상수접힘으로 사라진 원래 비교식

**같은 것을 2번 시도해서 안 되면 즉시 `unknown` 에 "무엇을·왜 확정 못 했는지 + 관측된 사실"을 적고 넘어가라.** 그게 감점이 아니라 정답이다.

### `calls` 표기 규칙
- **제네릭 인자를 쓰지 마라**: `Vec::<T>::from_iter_in` (X) → `bumpalo::collections::vec::from_iter_in` (O)
- `invoke` 도 호출이다 — 검사기가 잡으니 그냥 적어라(구버전에선 안 잡혔다)
- 짧은 이름(`check`, `new`)도 이제 매칭된다 — 망글 심볼을 쓸 필요 없다

## 3. 출력 형식 (JSON 1개 파일)

`C:\tfm2mods\MIG\_spec\<함수명>.json` 에 쓴다. UTF-8, 들여쓰기 1칸.
**아래 필드는 전부 필수**(모르면 빈 배열/빈 문자열이 아니라 `unknown` 에 이유를 적는다).

```json
{
 "name": "함수명(짧은 이름)",
 "sym": "IR define 줄의 @ 뒤 심볼 그대로",
 "src": "game-ai\\src\\...\\xxx.rs",
 "src_line": 1214,
 "ir_file": "m10.ll",
 "ir_from": 49611,
 "ir_to": 50100,

 "one_line": "한 줄 요약. '무엇을 판정/계산하는가'. 30~60자.",

 "signature": {
   "params": [
     {"i": 1, "name": "version", "type": "usize", "note": "AI 버전 게이트. 이 함수에선 분기 없음"},
     {"i": 2, "name": "player", "type": "&PlayerState(2528B)", "note": ""}
   ],
   "returns": "bool — true=전투 종료해야 함"
 },

 "reads": [
   {"base": "Entity", "offset": "0x640", "name": "speed", "note": "읽기만 하는 필드"}
 ],

 "writes": [
   {"base": "LegacyPlanHandler", "offset": "0x5e8", "name": "plan", "value": "새 BigPlan",
    "note": "★생성자·상태변경 함수는 여기에. reads 에 섞지 마라"}
 ],

 "constants": [
   {"value": 22500000001, "src_line": 1216, "meaning": "150000^2 + 1 — 사거리 임계(제곱비교)"}
 ],

 "calls": ["game_ai::plan_legacy::old::fight_model::ally_is_bound"],

 "logic": "판정식을 의사코드로. 분기 순서를 지켜서. 여러 줄 가능(\\n).",

 "knobs": [
   {"what": "사거리 임계", "where": "fight_model.rs:1216", "value": 22500000001,
    "effect": "올리면 더 먼 적까지 전투 대상으로 본다"}
 ],

 "unknown": ["p3 가 무엇인지 — 본문에서 안 쓰임", "0xd97300 내부는 안 봄"]
}
```

### ★★`constants` 와 `reads` 의 경계 (2차 배치에서 사람마다 갈렸다 — 이제 규칙으로 고정)
**구조체 오프셋은 `reads` 에만 적는다. `constants` 에는 넣지 마라.**
`constants` 는 **판정에 쓰이는 값**만 — 임계값·계수·마스크·상수 파라미터.

| 값 | 어디에 | 예 |
|---|---|---|
| 구조체 필드 오프셋 | `reads` **만** | `0x670`(hp), `0x930`(info.team) |
| 거리·시간·비율 임계 | `constants` | `22500000001`(150000²), `41`(HP%) |
| 비트마스크·비교값 | `constants` | `65534`, `768` |
| 열거형 태그값 | `constants` | `store i64 5` 의 5 |
| 좌표 변환 상수 | `constants` (의미를 밝힐 것) | `32000`(셀), `16000`(셀중심) |
| 배열 인덱스·stride | 적지 않는다 | `5`(슬롯수), `40`(stride) |

⚠2차 대조에서 한쪽이 오프셋을 `constants` 에도 적어 **불일치가 최대 13건 부풀었다**(사실은 둘 다 맞았다).
이건 정확도 문제가 아니라 **양식 문제**였고, 그래서 규칙으로 고정한다.

### 필드별 주의
- **`constants`**: `value` 는 **본문에 실제로 있는 숫자 그대로**. QC가 대조한다. 계산해서 바꾼 값(예: 제곱근)은 `meaning` 에만 쓰고 `value` 는 원본 그대로.
- **`calls`**: 본문의 `call` 에 실제 있는 것만. QC가 대조한다. 인라인돼 사라진 함수는 여기 넣지 말고 `logic` 에 서술.
- **`reads`**: 오프셋은 `"0x5c0"` 형식. QC가 10진수로 바꿔 본문에 있는지 본다(경고만 — gep 가 접히면 안 보일 수 있음).
- **`knobs`**: 이게 이 작업의 실용 산출물이다. **"올리면/내리면 어떻게 되는가"** 를 반드시 쓸 것. 확신 없으면 넣지 말고 `unknown` 으로.
- **`logic`**: 가장 중요하다. 분기 조건과 순서를 지켜 의사코드로. 길어도 좋다.

## 4. 제출 전 자체검사 (필수)

```bash
cd /c/tfm2mods/MIG && PYTHONIOENCODING=utf-8 python qcspec.py _spec/<함수명>.json
```

`[PASS]` 가 나올 때까지 고쳐라. 검사 항목:
- **C1 상수** — 적은 상수가 본문에 실제로 있는가
- **C2 호출** — 적은 피호출 함수가 본문 `call` 에 실제로 있는가
- **C3 오프셋** — 적은 오프셋이 본문에 있는가 (경고)
- **C4 범위** — ir_from/ir_to 가 진짜 그 함수 본체인가
- **C5 정직** — `unknown` 필드가 있는가

⚠**QC를 통과시키려고 사실을 빼지 마라.** 상수를 못 찾겠으면 목록에서 지우는 게 아니라, 왜 안 보이는지 `unknown` 에 적어라(인라인·상수접힘 등).

## 5. 하지 말 것

- 게임 실행, Ghidra 사용, 다른 파일 수정 — **전부 금지**. 이 작업은 IR 읽기와 자기 명세 파일 쓰기뿐이다.
- `MEM\` `ANA\` 지식베이스 읽기/쓰기 금지.
- 담당 함수 외의 명세 파일 건드리기 금지.
- git 명령 금지.

## 6. 마지막 보고

명세 파일 경로 + QC 결과 + **실제로 얼마나 걸렸는지 체감**(막힌 지점이 있었다면 무엇인지)을 1~2문단으로. 이 배치는 **속도 실측이 목적**이라 막힌 지점 보고가 결과물만큼 중요하다.
