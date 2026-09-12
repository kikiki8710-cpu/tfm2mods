# 7차 반증검증 — 배치 D (함수 `15`~`19`) 보고서

> 게임 0.5.8 · 2026-09-11 · 정본 스탬프 확인 완료
> 본 보고의 **본체는 `patch.json`** 이다(정정 16 · ev상향 7 · 브리핑오류 5, `--dry` 전건 통과).
> 이 문서는 도시에 §5-b 가 요구하는 4항목(판정 어휘 / 실행한 명령줄 / 내 지시의 오류 / 판정 반전)이다.

---

## §0 먼저 — 정본 신선도 확인 (실제로 돌렸다)

```bash
python -c "import hashlib,io;print(hashlib.sha256(io.open('_spec/specs20_v3.json','rb').read()).hexdigest()[:16])"
# -> dab6d7c803e1be63   (도시에 표기와 일치)
python -c "import hashlib,io;print(hashlib.sha256(io.open('_spec/specs20.json','rb').read()).hexdigest()[:16])"
# -> 6320fa42c1107160   (일치)
```
따라서 도시에는 최신이다. `DOSSIER_D.md` 1310줄 전량을 읽었다.

---

## §1 ★★가장 먼저 보고할 것 — **내 지시(도시에)의 오류 6건**

도시에 §5-b-3 이 「내 지시의 오류를 찾으면 그것부터 보고하라」고 했다. 6건 나왔고,
그중 **2건은 이 배치의 보고를 통째로 무효화할 뻔했다.**

### (1) ★★치명 — §5 의 `patch.json` 스키마가 **틀렸다**
도시에 §5 는 이렇게 쓰라고 했다:
```json
{ "round": "r7", "batch": "D", "entries": [ {"path": "...", "old": 0, "new": 1, "why": "..."} ] }
```
그러나 실제로 이걸 읽는 `applypatch.py`(L299·L311)는 **`errors[]` 와 `ev_up[]`** 만 본다.
키 이름도 `why` 가 아니라 **`evidence`** 이고, `kind`·`behavior_change`·`found_by` 가 더 필요하다.
`round` 도 `int` 다(L145 `"round": int(self.round)`).

즉 **§5 대로 냈으면 `errors` 가 비어 있어 「정정 0/0」으로 조용히 통과**했을 것이다.
이건 5차의 「ev 상향 492행 중 317행 유실」과 **정확히 같은 형태의 사고**다 —
계약을 산문으로 다시 적은 자리에서 났다.
정본은 `applypatch.py` 머리말과 `mkpatch.py` 머리말이고, **도시에는 그걸 가리키기만 해야 한다.**

### (2) §5 의 `"op": "append"` 는 **구현이 없다**
도시에 §5: 「새 항목 추가는 `"op": "append"` + `"path": "/specs[i]/notes"` + `"new": {...}`」.
`applypatch.apply_error` 에 `op` 를 읽는 코드가 없다(치환 전용). `open`/`notes` 는
**기존 항목의 문면을 `old`→`new` 로 치환**하는 경로만 있다(L188~196).
그래서 이번 라운드에 새로 알아낸 사실(짝캠프 매핑 · TLS 메모 경고)은 전부
**기존 행의 문면을 확장**하는 방식으로 실었다.

### (3) 주 표적으로 지목된 **G13 은 배치 D 에 0건**이다
지시문은 「G12·**G13**(`knobs.where` ±2 에 인용 명령 없음)·G9」를 1순위로 지목했다. 실측:
```bash
python -X utf8 specgate.py            # 전량
# G13 knobs.where 대조 = 8건 — 전부 specs 00 / 03 / 04 / 14 (다른 배치 몫)
for i in 15 16 17 18 19; do python -X utf8 specgate.py --only $i --gate G13; done   # 전부 0건
```
도시에 §4 자체는 G13 절을 **싣지 않았다**(정확했다). 지시문만 틀렸다.

### (4) §4 의 머리 숫자가 corpus 총계인데 「이 배치 몫만 추렸다」고 적혀 있다
`**[G12 src_line 대조] 47건**` 아래에 10건, `**[G9 callees 오염] 4건**` 아래에 1건이 실려 있다.
47·4 는 전 20함수 총계다(실행 확인: `specgate.py` 전량 = 총 59건, G12 47 · G13 8 · G9 4).
배치 몫은 **G12 10 · G9 1 = 11건**이고 그건 지시문 숫자와는 맞는다.

### (5) ★§4 가 준 「실제 후보」 목록은 **믿으면 안 된다**
`srclinecheck.py:84` 의 리터럴 정규식 `(?<![\w.\-])<값>(?![\w.])` 이
**SSA 레지스터(`%3`·`%21`·`%22`)·`align 4`·`dereferenceable(816)`** 까지 긁는다.
- 18 `consts[5]`(21) / `consts[6]`(22) 의 「실제 후보 = **[638]**」 → 전량 `%21`·`%22` 노이즈. 진짜 후보 0개.
- 16 `consts[6]`(3) 의 「실제 후보 = **[0]**」 → 진짜 사이트는 switch case 줄이라 아예 안 잡혔다.

후보를 그대로 채택했으면 **맞는 값 6개를 틀린 값으로 갈아엎었을 것**이다.

### (6) `mkpatch` <-> `applypatch` 의 **정수 필드 계약이 서로 어긋난다**
- `mkpatch.fix`(L100): 값 필드는 `tgt == old` **동등 비교** → `old` 가 `int` 여야 통과(문서 예시도 `old=21, new=22`)
- `applypatch.already`(L95): `(e.get("new") or u"")[:80]` → `new` 가 `int` 면 **`TypeError` 로 죽는다**
```
TypeError: 'int' object is not subscriptable
```
둘 다 만족하는 타입이 없다. 이번엔 **문자열로 내고**(applypatch 의 `conv` 가 원래 타입으로 되돌린다)
`mkpatch` 검증만 `force=True` 로 넘겼다(정본 현재값 2401·654 는 손으로 확인).
고칠 곳은 `applypatch.already` 첫 줄: `new = str(e.get("new") or u"")[:80]`.
**이 배치의 최대 수확 2건이 전부 `src_line` 정수 패치였다** — 그대로 뒀으면 1건도 못 냈다.

---

## §2 게이트 미해소 11건 — **한 건씩 IR 을 직접 열어 판정**

### §2-a 결론표

| # | 항목 | 도시에가 준 「실제 후보」 | **판정** | 근거 |
|---|---|---|---|---|
| 1 | [15] consts[5] `3`@256 | [245] | 명세가 맞다(게이트 오탐) | `switch` case |
| 2 | [15] consts[6] `4`@256 | [242,245,248,249,253] | 명세가 맞다 | `switch` case |
| 3 | [15] consts[7] `7`@256 | [245] | 명세가 맞다 | `switch` case |
| 4 | [16] consts[0] `-1`@2401 | [2399,2403,…] | **실오류 → 2399** | `!dbg` 사슬 |
| 5 | [16] consts[5] `0`@2398 | [2399,2402,…] | 판정보류(명세 유지) | 접힌 초기화 |
| 6 | [16] consts[6] `3`@2402 | **[0]** | 명세가 맞다 | `switch` case |
| 7 | [18] consts[0] `1`@635 | [637,638,…] | 명세가 맞다 | `switch` case |
| 8 | [18] consts[1] `2`@635 | [664,666,…] | 명세가 맞다 | `switch` case |
| 9 | [18] consts[4] `25`@654 | [638,658] | **실오류 → 658** | `!dbg` 사슬 |
| 10 | [18] consts[5][6] `21`/`22`@674/676 | **[638]** | 판정보류(명세 유지) | merged `line:0` |
| 11 | [15] G9 `chats` | — | 오탐(손확인 통과) | tcx: `Field pub` |

**10건 중 실오류는 2건.** 나머지 8건은 게이트 오탐이고, 그 원인은 **3가지 형태**로 정리된다.

### §2-b 실오류 1 — [16] `consts[0]` src_line **2401 → 2399**

```bash
cd /c/tfm2mods/MIG/_verify7/D/oracle && python -X utf8 g12probe.py 16 -1
```
```
51936  %7   = icmp eq i32 %6,   -1   chain=[('option.rs', 742), ('battle.rs', 2399)]
51954  %16  = icmp eq i32 %15,  -1   chain=[('option.rs', 742), ('battle.rs', 2407)]
52106  %93  = icmp eq i32 %92,  -1   chain=[('option.rs', 742), ('battle.rs', 2414)]
52205  %160 = icmp eq i32 %159, -1   chain=[('option.rs', 742), ('battle.rs', 2421)]
52026  %41  = add i64 %40,      -1   chain=[('effect.rs',  26), ('battle.rs', 2403)]   # (level-1)
52126 / 52223 / 52313 도 같은 형태 (2410 / 2417 / 2424)
```
`-1` 이 나오는 소스 줄은 **2399·2407·2414·2421**(Option 니치) + **2403·2410·2417·2424**((level-1)).
**2401 에는 어떤 `!DILocation` 도 매핑되지 않는다** — 본문이 참조하는 `battle.rs` 줄 전체 집합이
`[2399,2402,2403,2407,2409,2410,2414,2416,2417,2421,2423,2424,2429]` 다. 첫 사이트 **2399** 로 정정.

### §2-c 실오류 2 — [18] `consts[4]` src_line **654 → 658**

```bash
python -X utf8 g12probe.py 18 25 --real
```
```
7013  REAL  store i8 25, ptr %59   chain=[('mod.rs',1933),('mod.rs',1045),('mod.rs',1004),('epic.rs', 658)]
```
`654` 는 `objective = Some(Serpen{phase,with_battle})` 의 세 스토어가 쓰는 줄이다
(m09.ll **6977 / 6979 / 6981**, `!dbg !14813 = epic.rs:654`).
`Chat::SerpenSetup` push 는 **658**. 같은 명세의 `history[6]` 이 이미
「**658**(chats.push(Chat::SerpenSetup(0)))」을 복원해 뒀는데 **`consts` 표만 옛 값**이었다.
**G8(표에 옛 값)의 전형인데 G8 이 못 잡는다** — G8 은 `logic`<->표만 대조하고 `history` 는 안 본다.

### §2-d 게이트 오탐 3형태 — 원인과 수정안

**형태 1.  switch case 줄에는 !dbg 가 없다** (오탐 6건 = 본 배치 오탐의 75%)

    switch i64 %125, label %126 [
      i64 3, label %127          ; 리터럴은 여기, !dbg 는 없다
      i64 4, label %127
      i64 7, label %127
    ], !dbg !38930               ; !DILocation(line: 256) = modes.rs:256  <= 명세가 맞다

srclinecheck 은 **한 텍스트 줄 안에 리터럴과 !dbg 가 같이 있어야** 센다. 그래서
[15] 3/4/7@256, [16] 3@2402 (= entity.rs:1748 <= battle.rs:2402, 14-way attack_cooldown switch),
[18] 1/2@635 (v3_epicops_repair_need 반환 dispatch)가 전부 「IR 사슬에 없다」로 찍혔다.

**형태 2.  접힌 초기화는 phi 로만 남고 !dbg 가 line: 0 이다**

    %12 = phi i64 [ %80, %74 ], [ 0, %3 ], [ 0, %8 ], [ 0, %29 ], !dbg !56166   ; line: 0

= `let mut range = 0;` (battle.rs:**2398**, 4차에 줄 길이 산술로 +-0 복원). 2398 은 **참인 소스 줄인데
IR 위치가 없다.** 이 축은 원리적으로 IR 대조가 불가능하다. **명세를 유지**한다.

**형태 3.  병합된 위치(merged location)도 line: 0**

    %155 = select i1 %149, i8 21, i8 22, !dbg !15003   chain=[... , ('epic.rs', 0)]
    store i8 %155, ptr %142, !dbg !14953               chain=[... , ('epic.rs', 0)]

674(Chat::Press)와 676(Chat::PressChange)의 두 push 가 한 블록으로 합쳐지며 줄이 0 으로 떨어졌다.
674/676 의 근거는 IR 이 아니라 **줄 길이 산술(rmeta_srcmap)** 이고(4차 배치D), 그건 ev3 급 재료다.
**명세를 유지**한다. (참고: 같은 사슬의 is_none 은 epic.rs:673 으로 살아 있다 — consts[7] 은 OK.)

**수정판을 실제로 만들어 돌렸다** (`_verify7/D/oracle/srclinecheck2.py`):

    python -X utf8 srclinecheck2.py

    specs[15] single_try_engage            원본 3 -> 수정판 0
    specs[16] max_range_nearly_can_use     원본 2 -> 수정판 2   남은 것 = [(0,2401,..),(5,2398,..)]
    specs[18] v3_epicops_buff_window       원본 5 -> 수정판 1   남은 것 = [(4,654,[658])]
    ---- G12 총계: 원본 47건 -> 수정판 14건 (오탐 33건 제거)

- **배치 D: 10건 -> 3건**, 남은 3건이 **정확히 내가 손으로 찾은 실오류 2건 + 형태2 1건**이다.
- corpus 로도 **47 -> 14**. 즉 지금 G12 가 보고하는 것의 **70%가 오탐**이다.
- 부수: 더 엄한 리터럴 판정(타입 접두 필수) 덕에 **specs[9] 에서 원본이 못 본 실오류 1건이 새로 나왔다**
  (consts[4] src_line=1283 인데 실제 1286). 오탐 제거가 **탐지력을 깎지 않았다.**
- 이 수정판을 srclinecheck.py 에 반영할 것을 제안한다(도구 수정이므로 메인이 판단).

### §2-e G9 chats — 손확인 통과(오탐)

logic 의 `drop(battle.chats /*+0x68 Vec<Chat>*/)` 가 「필드 표기인데 tcx 에 동명 함수가 있다」로 걸렸다.

    Field   pub  game_ai::plan_legacy::old::SinglePlanBattle::chats   <= 수신자 타입의 '필드'
    AssocFn pub  game_ai::plan_legacy::types::BigPlan::chats          <= 다른 타입의 메서드

IR 교차검증: m13.ll:33723 `%128 = getelementptr .. i64 104`(=0x68) -> Vec<Chat>::drop. **필드가 맞다.**
SPEC_RUNBOOK §S5-c 대로 손확인 은 눈으로 넘기는 것이 정상이므로 **정정하지 않는다.**
**G9 수정 제안**: 수신자 타입을 logic/sig 에서 풀어, **그 타입에 동명 필드가 있으면 아예 올리지 말 것.**
현재 corpus 의 손확인 10개(chats·team·position·line·nearest_enemy·target_bush)가 전부 그 형태다.

---

## §3 ★§4-b 「검사받지 않는 축」 3개 — 처음 검사했다

세 축 모두 **검사기를 새로 만들어** 돌렸다. 소스는 `_verify7/D/oracle/` 에 있다.

| 축 | 검사기 | 배치 D 결과 | corpus 결과 |
|---|---|---|---|
| mem.dir | memdir.py | 검사 126 · **강한 불일치 1** · 오프셋 미검출 29 | 검사 450 · 강한 불일치 **9** · 미검출 88 |
| consts.kind | constkind.py | 검사 32 · R1 2 · R2 1 · R3 5 | 검사 186 · R1 3 · R2 12 · R3 43 |
| sig.params.role | paramrole.py | 검사 29 · **C1 거짓 0** · C2 6(전건 오탐) | 검사 110 · C1 거짓 **5** · C2 9 |

### §3-a mem.dir — 실오류 1건 (배치 D)

**검사기 원리**: IR 의 getelementptr(정적 바이트 오프셋을 **누적**) -> load/store 의 포인터 피연산자를
따라가 (베이스 레지스터, 오프셋) -> {r,w} 표를 만든다. 명세 행의 오프셋이 표에 **있는데** 주장한
방향이 없으면 **강한 불일치**. 오프셋이 아예 없으면 약한 신호(동적 인덱스 · memcpy/memset 대상 ·
주소만 넘기고 읽기는 피호출자 안 — 전부 정상).

    python -X utf8 memdir.py 15 16 17 18 19

    ===== specs[18] v3_epicops_buff_window  (m09.ll:6879~7264) =====
      mem[15] TeamPlan            +0xc8  dir=w  **강한 불일치**  IR=r
    ---- 합계: 검사 126 · 강한 불일치 1 · 오프셋 미검출 29

베이스별 표를 직접 뽑아 확인:

    ('%0', 192=0xc0 chats.cap)   ['r']
    ('%0', 200=0xc8 chats.ptr)   ['r']        <= store 0건
    ('%0', 208=0xd0 chats.len)   ['r', 'w']
    ('%142', 0) ['w']  ('%142', 1) ['w']  ('%142', 8) ['w']   <= 실제 쓰기 대상 = 힙 버퍼 원소

IR:

    %140 = getelementptr inbounds nuw i8, ptr %0, i64 200
    %141 = load ptr, ptr %140                          ; chats.ptr 를 읽는다
    %142 = getelementptr inbounds nuw { i8, [23 x i8] }, ptr %141, i64 %152
    store i8 %155, ptr %142                            ; 태그     @원소+0
    store i8 %41,  ptr %143                            ; LineType @원소+1
    store i64 0,   ptr %144                            ; usize    @원소+8

**TeamPlan+0xc8 은 이 함수에서 읽기 전용**이고(그 읽기는 mem[8] 이 이미 담고 있다),
쓰기는 **힙 버퍼 원소**에 들어간다. 베이스/오프셋/이름을 그렇게 정정했다(patch 4건).
같은 명세 mem[16](chats.len w)은 진짜 store 가 있으므로 정상.

**이 축이 처음 검사됐다는 증거**: corpus 450행 중 **9건**이 강한 불일치다(배치 D 는 1건).
나머지 8건은 다른 배치 몫이라 손대지 않았다 — 메인에 넘긴다:
00 mem[27] · 04 mem[26] · 05 mem[27]·mem[29] · 12 mem[17]·mem[23]·mem[24] · 14 mem[26].

### §3-b consts.kind — ★이 축은 「검사받지 않는 축」이 아니라 「사람이 쓰지 않는 축」이다

**가장 중요한 발견이다.** kind 는 사람이 매기는 값이 **아니라** mkspec3.py:329 가
meaning 문면을 키워드 매칭해 파생시키는 값이다:

    kind = (태그   if any(k in m for k in (태그, 판별자, variant)) else
            센티널 if any(k in m for k in (센티널, 니치, 0xff, MAX)) else
            인덱스 if 인덱스 in m else 임계)

결론 3가지:

1. **kind 를 patch 로 직접 고치면 no-op 이다** — v2(constants[])에 kind 키가 없어
   mkspec3 이 매번 다시 파생시킨다. **고쳐야 하는 것은 meaning 이다.**
2. kind 오류는 (a)meaning 문면 (b)분류기 우선순위 버그 둘 중 하나다. IR 과 직접 대조되는 축이 아니다.
3. 그래서 §4-b 표의 「consts.kind 186행 무검사」라는 분류 자체가 부정확하다 —
   **진짜 무검사 축은 meaning 의 문면**이고, kind 는 그 그림자다.

**검사기**(constkind.py)는 그래서 IR 용법 부류(SWITCH/EQ/REL/STORE/ARITH/GEP/SELECT)를 따로 뽑아
파생값과 붙여 본다. 규칙은 보수적으로 셋만 뒀다.

    python -X utf8 constkind.py 15 16 17 18 19

    specs[15] consts[1] 60  kind=임계 IR용법=STORE        R3 비교에 안 쓰인다
    specs[15] consts[2] 1   kind=임계 IR용법=ARITH        R3 비교에 안 쓰인다
    specs[16] consts[4] 100 kind=임계 IR용법=ARITH,SELECT R3
    specs[16] consts[5] 0   kind=임계 IR용법=EQ,SWITCH    R2
    specs[17] consts[3] -1  kind=태그 IR용법=STORE        R1 센티널을 태그로
    specs[19] consts[1] 1   kind=임계 IR용법=GEP,STORE    R3
    specs[19] consts[2] 3   kind=임계 IR용법=GEP,STORE    R3
    specs[19] consts[4] -1  kind=태그 IR용법=EQ           R1 센티널을 태그로

**정정한 것 6건** (전부 meaning 을 고쳐 파생을 바꿨다. 적용 후 파생값을 메모리에서 시뮬레이션해 확인):

| 행 | 옛 kind | 새 kind | 왜 |
|---|---|---|---|
| 17 consts[3] -1 | 태그 | **센티널** | 같은 대상(Option None 니치)을 15/16/18 은 '센티널', 17/19 만 '태그' 로 불렀다. 분류기 우선순위(태그>센티널) 때문에 **문면에 '태그' 한 글자가 섞이면 센티널이 태그로 앉는다** |
| 19 consts[4] -1 | 태그 | **센티널** | 같음. 오라클 실측 Option<JungleType>::None = [255] |
| 19 consts[0..3] 0/1/3/2 | 임계 | **태그** | JungleType **판별자**다. IR 용법에 REL(임계 비교)이 0건이고 STORE/GEP 뿐. meaning 에 '태그/판별자' 가 없어 기본값 임계 로 떨어져 있었다 |

**정정하지 않고 남긴 것 3건 — 판정: 표기 불가(어휘 부족)**
15 consts[1](60) · 15 consts[2](1, 1-team) · 16 consts[4](100, 퍼센트 기준)
셋 다 **비교에 안 쓰이는 산술/페이로드 상수**인데 4어휘(태그/임계/인덱스/센티널)에
담을 칸이 없어 전부 catch-all 인 임계 로 앉는다.
**제안: kind 어휘에 `산술`(계수·기준값)과 `페이로드`(전달 전용)를 추가**하고
mkspec3.py:329 에 키워드를 추가하라. 지금 임계 111/186(60%)은 분류가 아니라 **기본값**이다.

**다음 라운드의 게이트 제안 (G14 consts.kind 정합)** — constkind.py 의 R1/R2/R3 를 specgate 에 편입:

- R1: 값이 -1/255/i64::MAX 인데 kind=태그 -> 니치 센티널 오분류 (corpus 3건)
- R2: IR 용법에 SWITCH 가 있고 REL 이 없는데 kind=임계 -> 판별자 오분류 (corpus 12건)
- R3: kind=임계 인데 IR 용법에 비교(REL/EQ/SWITCH)가 **하나도 없다** (corpus 43건)

한계(범위 명시): 같은 값이 함수 안 여러 곳에 나오면 용법 집합이 합쳐져 흐려진다
(16 consts[2]=2 는 level>2 임계이자 EntityType::Tower 태그라 R2 오탐이 났고, 그래서
R2 를 「SWITCH 이면서 REL 없음」으로 좁혔다). **R3 43건은 대부분 어휘 부족**이지 오분류가 아니다 —
어휘를 먼저 늘리고 게이트를 붙여야 오탐 폭발을 피한다.

### §3-c sig.params.role — C1 거짓 0건(배치 D), 검사기 C2 는 기각

**검사기 원리**: role 산문에서 **반증 가능한 주장 2종**만 뽑아 IR 과 대조한다.

- C1: 「본문에서 안 쓴다 / 그대로 전달만 한다」 -> 그 인자 레지스터가 gep/load/store 의
  포인터 피연산자로 **한 번도** 안 나와야 한다. 한 번이라도 나오면 거짓.
- C2: 「+0xNNN 을 읽는다」 -> 그 인자에서 파생된 gep 오프셋 집합에 있어야 한다.

    python -X utf8 paramrole.py 15 16 17 18 19

    specs[15]  p2 version %2 ptr사용=0   p3 rnd %3 0   p6 target_id %6 0   p7 debug %7 0
    specs[16]  p2 tick %2 ptr사용=0
    specs[17]  p0 version %1 ptr사용=0   p3 player %4 ptr사용=0
    specs[18]  p1 version %1 0  p2 rnd %2 0  p5 goal_data %5 0  p6 plan %6 0
    specs[19]  p0 version %0 0  p1 rnd %1 0  p4 team_plan %4 0  p5 now_camp %5 0  p6 debug %6 0
    ---- 검사 29건 · C1 거짓 0 · C2 과대 6 · 건너뜀 0함수

**배치 D 의 「안 쓴다/전달만」 주장 15건이 전부 참이다.** 특히

- 18 open[0] 의 「version 은 v3_serpen_contest_clear_win 에만 전달」 — %1 포인터 사용 0회로 **재확인**
- 19 의 「version·rnd·team_plan·debug 는 클로저 환경에만 담긴다」 — 전부 직접 역참조 0회
- 17 의 「version·player 는 본문에서 한 번도 로드되지 않는다」 — 0회

**C2 는 6건 전부 오탐이라 기각한다** (판정: 설계 결함). 원인은 **베이스 미구분**이다:
role 이 적은 +0x8·+0x1e0·+0x10·+0x20 은 **그 인자가 아니라 그 인자가 가리키는 것의
한 단계 아래 필드**다(data.cache(+0x0) -> cache.player_champion(+0x1e0);
goal 의 TryKill(+0x8, +0x10) 은 '읽는다'가 아니라 '레이아웃 서술'; 19 는 **클로저 환경** 오프셋).
**고치는 법**: +0xN 바로 앞 토큰이 **그 파라미터 이름 자신**이거나 param.field 형태일 때만
그 파라미터의 주장으로 센다. 아니면 「베이스 불명」으로 분류해 세지 않는다.

**corpus 에는 C1 거짓이 5건 있다**(전부 data/self 를 「안 씀」이라 적었는데 +0x0/+0x8/+0x10
을 실제로 읽는다). 배치 D 몫이 아니라 손대지 않았다 — 메인에 넘긴다.

**부수 발견 — sig.params 스키마가 명세마다 다르다**: 15 는 sret 반환 슬롯을 params[0] (sret ret)
로 **싣고**, 17 은 같은 sret 반환인데 **안 싣는다**(4개 = Rust 인자 수). 검사기가 define 인자 수와
대조하다 걸렸다. 오류는 아니지만 기계 대조를 매번 막는다 — **규약 통일 제안**.

---

## §4 오라클 — ev>=4 내리기 (프로브 2개 · 4회 실행)

도시에 §1 이 경고한 대로 15·18 은 in:game_ai 라 직접 진입이 막힌다.
`_tcx/game_ai.json` 의 v/p 를 **둘 다** 조회해 진입 가능한 것만 골랐다
(METHOD_MAP 6-한계1 의 「모듈 경로로 단정하지 마라」 규칙).

| 대상 | v | 진입 |
|---|---|---|
| v3_epicops_repair_need | **in:game_ai** | 불가 (18 consts[0]/[1]/knobs[0]/[6..9] 미해소) |
| is_object_being_taken_by_enemy | **in:game_ai** | 불가 (18 knobs[1]) |
| is_cleared · is_side_cleared · best_jungle_goal | pub | 가능 |
| TeamPlan::next_respawn_tick | **Field pub**(+0x378) | 주입 가능 |

### §4-a D7_o1 — 니치·레이아웃·타워 극성

    sh C:/tfm2mods/MIG/_verify3/build.sh D7_o1.rs
    /c/Users/jungs/AppData/Local/Temp/tfm2_spanprobe/D7_o1.exe 60

    setting_ok  width=960000 height=960000 tps=60 champ_radius=10000
    INTEG       towers=16  twin0=2  twin1=2          <= TEMPLATE 2 오염 없음
    A  Option<TowerType>        size=1    bytes=[255]
    A  Option<JungleType>       size=1    bytes=[255]
    A  Option<LineType>         size=1    bytes=[255]
    A  Option<SinglePlanBattle> size=144  inner=144  head_i64=-1
    A  JungleType Rhino=[0] Mushroom=[1] Bee=[3] Stump=[2]
    B  my_team=0  iter_towers_without_nexus(1)  n=8  좌표유일=8  적팀표기일치=8/8
    B  my_team=1  iter_towers_without_nexus(0)  n=8  좌표유일=8  적팀표기일치=8/8
    C  tps=60  best_jungle_goal=Bee

확증된 것:

- **15 consts[4]** — i8 -1(=255) = Option<TowerType>::None / i64 -1 = Option<SinglePlanBattle>::None.
  덤으로 size_of::<Option<SinglePlanBattle>>() == size_of::<SinglePlanBattle>() == **144** 라
  IR 의 dereferenceable(144) / memcpy .. i64 144 / store i64 -1, ptr %0 과 **완전 정합**.
- **15 mem[16]/mem[17]** — 반환 슬롯 +0x0 의 None 니치 = -1 (ev 상한 3으로 상향)
- **15 consts[2]** — 1 - player.info.team 의 극성: 양 팀에서 **8/8 전부 적팀**, 좌표 유일 8
- **19 consts[0..3]** — Bee=3 / Stump=2 를 직접 읽어 「2보다 3이 먼저」 주석을 실행으로 확증
  (기존 근거였던 236/236 은 best_jungle_goal end-to-end 일치라 **태그값 자체의 증거가 아니었다**)
- 부수: best_jungle_goal = Bee — 챔프(32000,928000) 기준 거리 Rhino 496257 / Mushroom 365557 /
  **Bee 343722** / Stump 529694 의 최소와 일치 => 본선 정렬 키 재확인

### §4-b D7_o2 — 19 knobs[6] 짝캠프 마진: 1단위 이분탐색, 12/12 일치

TeamPlan::next_respawn_tick(pub, +0x378)을 주입하고 false->true 경계를 1단위로 찾았다.
★TLS 주의: MapDef::camp_pos 가 CAMP_POS_MEMO 를 쓰므로 **한 프로세스 = 한 tps**(TEMPLATE 3·4).

    sh C:/tfm2mods/MIG/_verify3/build.sh D7_o2.rs
    for t in 30 60 90; do /c/Users/jungs/AppData/Local/Temp/tfm2_spanprobe/D7_o2.exe $t; done

| 캠프 | dist | tps=30 | tps=60 | tps=90 | 모델 |
|---|---|---|---|---|---|
| Rhino | 496257 | 496288 | 496318 | 496348 | dist + tps + 1 |
| Stump | 529694 | 529725 | 529755 | 529785 | dist + tps + 1 |
| Bee | 343722 | 496438 | 496618 | 496798 | **Rhino_dist + 6*tps + 1** |
| Mushroom | 365557 | 529875 | 530055 | 530235 | **Stump_dist + 6*tps + 1** |

=> is_side_cleared(c) 경계 = **max( dist(c) + tps + 1 , dist(짝(c)) + 6*tps + 1 )**
**12칸 손계산 전건 일치, 오류 0.** 두 가지가 나온다:

1. **knobs[6](짝캠프 마진 5초)이 실행으로 확정** — 자기 조건의 +tps(=knobs[5], 1초) 위에
   **5초가 더 붙어 총 6*tps** 다. ev 4 -> 2.
2. ★**새 사실 — 짝 매핑 = Rhino<->Bee · Mushroom<->Stump.** 명세 어디에도 없었다.
   (knobs[6].effect 에 실었다.)

덤으로 is_cleared 경계가 dist + tps + 1 로 나와 history[0] 의 재현식
`next_spawn > move_tick + offset + tick + tick_per_second` 이 **실행으로 확증**됐다
(move_speed == 1 이라 move_tick == dist — TEMPLATE 6 그대로).

### §4-c 내리지 못한 것 — 범위를 적는다

| 대상 | 판정 | 범위 |
|---|---|---|
| 18 consts[0]·[1], knobs[0]·[6]~[9] | **미탐색** | v3_epicops_repair_need 가 v=in:game_ai 이고 **크레이트 루트 재수출이 아니다**(p 조회). 남은 수단 = (a)rmeta MIR(mir=False 라 없다) (b)exe 훅 (c)상위 pub 진입점 발굴 |
| 18 knobs[1] | **미탐색** | is_object_being_taken_by_enemy 도 in:game_ai |
| 18 knobs[3]·[16] (발표자 선정) | **미탐색** | 판정 자체가 private 함수 안. 재료는 v3_epic_formation_role(pub, 이미 ev2)뿐 |
| 18 knobs[19]~[26], 16 knobs[5]~[8], 17 knobs[4] | **미탐색** | 전부 _gcbc/다른 파일의 상수라 이 함수 오라클로는 안 닿는다 |

★**형제 복제본 조사 결과(지시 3 대응)**: TeamPlan 의 pub AssocFn 은 12개뿐이고 그중
handle_press_epic(epic.rs:502, **18 의 version<=1 형제**)와 handle_epic_line_change(epic.rs:684,
**18 바로 다음 함수**)가 pub 이다. handle_epic_line_change(m09.ll:7267~8155)는
Chat::PressChange(태그 **22**) push 를 **6곳**에서 하고 LineType 을 원소 +0x1 에 넣는다 —
**18 mem[15] 의 원소 레이아웃(태그@+0 · LineType@+1 · usize@+8)을 교차확인**해 준다.
다만 v3_epicops_repair_need 분기 자체는 그 안에 없어 consts[0]/[1] 은 여전히 **미탐색**이다.

---

## §5 명세 전문(§3)에 대한 반증 — 게이트 밖에서 찾은 것

### (1) [15] mem[15] · knobs[4] — 표에 옛 값 (G8 형, 게이트가 못 잡음)

둘 다 「의미 미확정 — unknown 참조」 라고 적혀 있다. 그런데 같은 명세의 history[1] 이 이미
**「소비처가 존재하지 않는다 — 노브가 아니다」**로 닫았다 (생성 14곳 전부 리터럴 60,
main_goal.__1(goal+0x10)을 읽는 유일한 코드 = BattlePlanGoal::Debug::fmt).
**정정했다.** G8 은 logic<->표만 보므로 history<->표 형태는 못 잡는다 —
**G8 의 비교 대상에 history 를 추가할 것**을 제안한다(이번에 이 형태가 3건: 15 x2 + 18 consts[4]).

### (2) 근거 인플레이션 — 「오라클 game==mine 236/236」 이 행 단위 근거로 재사용되고 있다

배치 D 의 mem·consts·knobs 수십 행이 전부 같은 문자열을 달고 있고, 그 때문에 ev 가 2/3 으로
내려가 있다. 그런데 236/236 은 **end-to-end 일치 1건**이지 「Option<JungleType>::None 의 니치가
0xff 다」 같은 **개별 주장의 증거가 아니다.**
예: 19 consts[1] = 「jungle_camps[1] = JungleType::Mushroom · 오라클 실행 확증(236/236)」.
이번에 그중 6행을 **직접 측정한 값으로 교체**했다(§4-a). 나머지는 범위 밖이라 남겼다.
**제안**: applypatch.apply_evup 이 붙이는 근거에 **대상 주장이 무엇인지**를 강제하거나,
ev 파생 시 「같은 근거 문자열이 N행 이상에 붙어 있으면 경고」를 게이트로 넣어라.

### (3) ★새 사실 — 16 의 이웃 max_range_cached 는 TLS 메모다 (프로브 함정)

    python -X utf8 tlsscan.py | grep MAX_RANGE
    #   1624 B  m10.ll  ::plan_legacy::old::battle::MAX_RANGE_CACHE::{K#0}::{closure#0}..
    grep -n "MAX_RANGE_CACHE" /c/tfm2mods/_gaibc/m10.ll
    #   51072: .. LocalKey<RefCell<MaxRangeCache>>::with<..max_range_cached..>(..)

- **max_range_nearly_can_use(범위 51913~52374) 자체에는 threadlocal 접근이 0건** — 오라클 안전.
- 그러나 **max_range_cached(m10.ll:51072)가 그 결과를 MAX_RANGE_CACHE(thread_local
  RefCell<MaxRangeCache>, 1624B)로 메모**한다.
- 15 history[5] 가 지목한 update_v32:505 의 dist_sq > max_range_cached^2 게이트를 오라클로
  재려면 **한 프로세스 = 한 케이스**여야 한다(TEMPLATE 함정 3). logic 주의 5)로 실었다.
- 15 open[0](판별력 있는 세계에서 update 의 version 축을 재라)를 다음 라운드가 시도할 때
  **이 함정을 모르면 4차의 check_kill_die_tick 사고를 그대로 반복한다.**

### (4) closed 를 뒤집지는 못했다

closed 전건을 읽었고 반증 근거를 찾지 못했다. **판정: 유지.**
단 16 closed[6] 이 스스로 지적한 문서위생(「unknown[] 에 resolved[] 가 뒤집은 판정 3건이
원문 그대로 남아 있다」)은 **아직 정리되지 않았다** — v3 closed[0][1][2] 에 그대로 있다.
open/notes 는 인덱스가 아니라 문면으로만 주소지정되고 취소선 규약과도 얽혀 있어
**이번 patch 에서는 손대지 않았다**(범위 명시: 문면 치환은 가능하나 cleanspec 규약과 충돌 위험).

---

## §6 판정 어휘 (도시에 §5-b-1)

| 대상 | 판정 | 범위 |
|---|---|---|
| G12 [16]consts[0] 2401 / [18]consts[4] 654 | **실오류 -> 정정** | — |
| G12 나머지 8건 | **게이트 오탐** | srclinecheck 의 줄단위 매칭 한정. 수정판으로 corpus 47->14 실증 |
| G12 [16]consts[5] 0@2398 | **표기 불가** | **IR 로는** 원리적으로 대조 불가(접힌 초기화는 phi + !dbg line:0). 2398 자체는 4차 줄 길이 산술로 +-0 복원된 참값 |
| G12 [18]consts[5][6] 674/676 | **표기 불가** | **IR 로는** 불가(merged location -> line:0). 근거는 줄 길이 산술(ev3) |
| G9 [15]chats | **오탐(손확인 통과)** | tcx 상 수신자 타입의 Field pub |
| mem.dir [18]mem[15] | **실오류 -> 정정** | — |
| consts.kind 6건 | **실오류 -> meaning 정정** | kind 는 파생값이라 직접 패치는 no-op |
| consts.kind 3건(60 / 1-team / 100) | **표기 불가** | 4어휘에 산술/페이로드 칸이 없다. 어휘 추가 제안 |
| sig.params.role C1 15건 | **참(반증 실패)** | IR 포인터 사용 0회로 전건 확인 |
| sig.params.role C2 | **설계 결함 -> 기각** | 베이스 미구분. 고치는 법 §3-c |
| 18 consts[0]·[1]·knobs[0]·[1]·[6]~[9] | **미탐색** | 오라클 진입 불가(in:game_ai, 루트 재수출 아님). 남은 수단 = exe 훅 / 상위 진입점 발굴 |
| 19 knobs[6] | **실행 확증(ev 4->2)** | 12/12 |
| 짝캠프 매핑 Rhino<->Bee · Mushroom<->Stump | **사실 서술(신규)** | knobs[6].effect 에 반영 |
| max_range_cached TLS 메모 | **사실 서술(신규)** | 16 logic 주의 5)에 반영 |
| closed 전건 | **유지** | 반증 근거 없음 |

---

## §7 판정 반전 (도시에 §5-b-4 — 반전도 오류로 센다)

| # | 전에 A 라 했는데 | 실제 B | 어떻게 |
|---|---|---|---|
| 1 | 18 consts[4] Chat::SerpenSetup 은 **654** 줄 | **658** | 같은 명세 history[6] 이 이미 658 이라 적었다(표만 옛 값) |
| 2 | 16 consts[0] Option 니치 -1 은 **2401** | **2399**(+2407/2414/2421) | 2401 에는 !DILocation 자체가 없다 |
| 3 | 18 mem[15] 는 TeamPlan+0xc8 **쓰기** | TeamPlan+0xc8 은 **읽기 전용**, 쓰기는 힙 버퍼 원소 | mem.dir 첫 검사 |
| 4 | 15 mem[15]·knobs[4] TryKill __1 **의미 미확정** | **소비처 0건 — 노브가 아니다** | history[1] (표 미반영) |
| 5 | 17 consts[3]·19 consts[4] 의 -1 은 **태그** | **니치 센티널** | 오라클 직독 + 15/16/18 과의 불일치 |
| 6 | 19 consts[0..3] 캠프값은 **임계** | JungleType **판별자** | IR 용법에 REL 0건 + 오라클 태그 직독 |
| 7 | (도시에) G12 후보 [638] 이 21/22 의 실제 줄 | **%21/%22 SSA 노이즈**, 진짜 후보 0개 | 정규식 결함 |
| 8 | (도시에) G13 이 배치 D 의 주 표적 | 배치 D **0건** | specgate 실행 |

**오류 총계 = 명세 실오류 9행(정정 16개 필드) + 지시 오류 6건.**

---

## §8 산출물

    _verify7/D/patch.json                 <= ★본 보고 (정정 16 · ev상향 7 · brief_errors 5)
    _verify7/D/REPORT.md                  <= 이 문서
    _verify7/D/oracle/
       g12probe.py          G12 수동 확인 보조(리터럴 줄 원문 + inlinedAt 사슬)
       srclinecheck2.py     ★G12 제안 수정판 (corpus 47->14, 오탐 33 제거)
       srclinecheck2_out.txt
       memdir.py            ★mem.dir 축 검사기 (신설)         · memdir_D.txt
       constkind.py         ★consts.kind 축 검사기 (신설)     · constkind_D.txt
       paramrole.py         ★sig.params.role 축 검사기 (신설) · paramrole_D.txt
       D7_o1.rs / D7_o1_tps60.txt      오라클 1 (니치·타워 극성)
       D7_o2.rs / D7_o2_bs.txt         오라클 2 (짝캠프 마진 이분탐색)
       mkpatch_D.py         patch.json 생성 스크립트 (mkpatch.py import)
       srclinecheck_all.txt 원본 G12 전량 출력

검증:

    python -X utf8 applypatch.py 7 --only D --dry
    # [D] 정정 16/16 · ev상향 7/7
    # 정정 16 성공 / 0 실패 · ev상향 7 성공 / 0 실패 · **동작 변경 0건**
    # 발견 경위(정정): inherited=4 · new=12   /   (ev상향): new=7

적용 후 파생값을 메모리에서 시뮬레이션해 의도대로 바뀌는 것까지 확인했다:

    specs[16].consts[0] src_line=2399   specs[18].consts[4] src_line=658
    specs[17].consts[3] kind=센티널      specs[19].consts[0..3] kind=태그   specs[19].consts[4] kind=센티널
    specs[15].consts[2] ev=2  consts[4] ev=2  writes[4] ev=3  writes[5] ev=3
    specs[19].new_knobs[1](=knobs[6]) ev=2

## §9 다음 라운드에 남기는 것 (우선순위 순)

1. **srclinecheck.py 를 수정판으로 교체** — 지금 G12 보고의 70%가 오탐이다. 오탐을 손으로
   넘기기 시작하면 METHOD_MAP §2-C 의 경고대로 **진짜 결함도 같이 넘긴다.**
2. **applypatch.already 의 int 크래시 수정** (new = str(...)) — 정수 필드 패치가 지금 **전면 불가**다.
3. **consts.kind 어휘 확장(산술/페이로드) 후 G14 신설** — 어휘를 먼저 늘려야 R3 43건이
   오탐 폭발을 안 낸다.
4. **G8 의 비교 대상에 history 추가** — 이번에 이 형태가 3건 나왔고 전부 게이트 밖이었다.
5. **G9 를 수신자 타입 인지로** — 현 손확인 10건이 전부 「수신자 타입의 필드」다.
6. **다른 배치 몫으로 넘기는 것**: mem.dir 강한 불일치 8건(00/04/05x2/12x3/14) ·
   sig.params.role C1 거짓 5건 · G12 수정판이 새로 찾은 specs[9] consts[4] 1283->1286.
7. **18 의 in:game_ai 벽** — v3_epicops_repair_need / is_object_being_taken_by_enemy 가
   오라클 사각지대다. 이 둘이 18 의 ev>=4 25행 중 8행을 붙잡고 있다.
