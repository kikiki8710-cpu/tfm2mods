# -*- coding: utf-8 -*-
u"""knobval — `knobs[].value`(노브의 **값**)를 IR 원문과 대조한다. (9차 배치C · 2026-09-11)

## 왜 만드나 — `G13` 이 보는 것과 안 보는 것
`G13`(`whereline.py`)은 `knobs[].where` 의 **줄번호와 인용 문면**만 본다. **값은 아무도 안 본다.**
그런데 노브는 **바꿔 쓰라고 있는 칸**이라, `mem`/`consts` 와 달리 값이 틀리면
재구현자가 아니라 **그 표를 보고 설정을 바꾸는 사용자가 직접 당한다.**
7차에 `01 knobs[2]/[3]` 의 select arm 방향이 뒤집혀 있었고(20/40 을 반대로 넣게 된다),
그건 `where` 축에서 **우연히** 걸린 것이지 값 검사가 잡은 게 아니다.

## 판정식 = **반증**(8차 배치B 설계)
| | 분류식 | **반증식(이것)** |
|---|---|---|
| 규칙 | 후보가 있는데 주장이 없으면 불일치 | **주장을 지지하는 관측이 0건일 때만** 불일치 |
| 관측을 늘리면 | 오탐이 는다 | **오탐이 준다** |

⟹ `where` 가 가리키는 IR 창(window)에서 **정수 리터럴을 관측**하고,
   `value` 가 주장하는 정수 중 **하나라도** 그 관측에 있으면 **무죄**다.
   창을 넓히거나 리터럴 문법을 늘리면 **적발이 줄지 늘지 않는다.** 「귀속은 구제에만」이 저절로 성립한다.

## 4개 규칙
- **V1 값 미관측** — 주장 정수가 **인용·창 어디에도 없다**(관측이 있는데도). ★주력
- **V2 인용 불일치** — 인용한 명령 자체에는 없지만 창 어딘가엔 있다(약한 후보 · 기본 OFF)
- **V3 자리표시** — `value` 칸이 **아예 없다**(키 부재). 사용자가 못 고친다
- **V4 arm 충돌** — 같은 `select`/`phi` 를 인용한 두 노브가 **같은 arm 값**을 주장한다(방향 뒤집힘 탐지)

## ⛔「부재」는 결함이 아니다 (8차 `G14` 교훈)
상수가 **접히거나**(constant folding) **콜리 안에서** 쓰이면 IR 창에 안 보이는 게 **정상**이다.
그래서 V1 은 **창에 정수 리터럴이 하나라도 관측될 때만** 발화한다(관측 0 = 판정 유보).
런타임 값(`tick_per_second` 등)·오프셋 표기도 같은 이유로 구제한다.

사용:  python -X utf8 gate.py              단독 리포트
       python -X utf8 gate.py --detail     행별 관측 덤프
       python -X utf8 gate.py --mutate     변이 시험(검출력)
       (specgate 가 `check_spec` 을 불러 G18 로 쓴다)
"""
import io, json, os, re, sys

sys.stdout.reconfigure(encoding="utf-8", errors="replace")
HERE = os.path.dirname(os.path.abspath(__file__))
MIG = os.path.dirname(os.path.dirname(HERE))          # …/MIG
IRDIRS = (r"C:\tfm2mods\_gaibc", r"C:\tfm2mods\_gcbc", r"C:\tfm2mods\_gvbc")

W = 4                 # 앵커 한 줄당 창 반경
W_WIDE = 60           # 구제용 광역 창
RANGE_CAP = 400       # `a~b` 범위 인용의 최대 길이
USE_BODY = False      # ⛔함수 본문 전체를 창으로 쓰는 폴백 — 오탐만 냈다(body_anchors 주석)

FILETOK = re.compile(r"\b([mgv]\d{2})\.ll")
NUM = re.compile(r"-?\d+")
# 백틱 인용 — `whereline.py` 와 같은 문법
QUOTE = re.compile(r"`([^`]{4,400})`")
IROP = re.compile(r"^(icmp|fcmp|call|invoke|getelementptr|load|store|add|sub|mul|shl|lshr|ashr|"
                  r"and|or|xor|select|phi|br|switch|sext|zext|trunc|bitcast|udiv|sdiv|urem|srem|"
                  r"llvm\.|tail call|%[\w.]+ = )")
# ★IR 정수 리터럴 뽑기 = **지워서 남기는 방식**.
#   초판은 `i64 <수>` 처럼 **타입이 바로 앞에 붙은 것만** 셌는데, LLVM 은 이항 연산의 타입을
#   **한 번만** 적는다(`icmp ugt i64 %30, 4`) ⟹ **두 번째 피연산자를 통째로 못 봤다.**
#   그 탓에 첫 실행이 `00 knobs[0] value=4`(인용에 4 가 **적혀 있다**)를 적발로 냈다.
#   ⟹ SSA(`%30`)·메타(`!56400`)·전역(`@foo`)·속성(`#0`)·타입어(`i64`/`ptr`/`label`)·
#      `align N`·`0x…` 를 **먼저 지우고**, 남은 정수를 리터럴로 본다.
_DROP = [
    (re.compile(r"^\s*\d+:\s*(?:;.*)?$"), u""),        # ★블록 라벨 `266:` — 19차 C: [91] knobs[0]=21 의 「관측 266」이 라벨이었다(리터럴 아님)
    (re.compile(r",\s*!dbg\b.*$"), u""),               # 디버그 꼬리
    (re.compile(r"!\w+\s*!?\{?[^,\)]*"), u" "),        # 메타데이터
    (re.compile(r"%[-\w.$]+"), u" "),                  # SSA·블록
    (re.compile(r"@[-\w.$\"]+"), u" "),                # 전역·함수
    (re.compile(r"#\d+"), u" "),                       # 속성 그룹
    (re.compile(r"0x[0-9A-Fa-f]+"), u" "),             # 16진(따로 본다)
    (re.compile(r"\balign\s+\d+"), u" "),
    (re.compile(r"\bdereferenceable\(\d+\)"), u" "),
    (re.compile(r"\b(?:i\d+|u\d+|f\d+|ptr|float|double|void|label|token|x|zeroinitializer|"
                r"nsw|nuw|inbounds|exact|volatile)\b"), u" "),
]
_cache = {}


def irsrc(f):
    if f not in _cache:
        _cache[f] = None
        for d in IRDIRS:
            p = os.path.join(d, f + ".ll")
            if os.path.exists(p):
                _cache[f] = io.open(p, encoding="utf-8", errors="replace").read().split("\n")
                break
    return _cache[f]


def irline(f, n):
    src = irsrc(f)
    if not src or n < 1 or n > len(src):
        return None
    return src[n - 1]


def anchors(where):
    u"""`where` 에서 (파일, 줄) 앵커를 **전부** 뽑는다.

    ★백틱 인용 중 **IR 명령인 것만** 지운다 — 인용 안의 `i64 150000` 같은 값 리터럴을
      줄번호로 오인하면 엉뚱한 창이 열린다. 반대로 **인용을 전부 지우면**
      `` `_gaibc/m08.ll:94136~94353` `` 처럼 **백틱 안에 든 줄 참조를 통째로 잃는다**
      (초판이 `14 knobs[4]` 에서 실제로 그랬다 — 창이 엉뚱한 곳에 열려 오탐이 났다).
    `m04.ll 44025행` · `m05.ll:40059~40063` · `44266·44268행` 같은 꼬리 숫자를 다 받는다."""
    w = QUOTE.sub(lambda m: u" " if IROP.match(m.group(1).strip()) else m.group(1), where or u"")
    out = []
    ms = list(FILETOK.finditer(w))
    for i, m in enumerate(ms):
        f = m.group(1)
        tail = w[m.end(): ms[i + 1].start() if i + 1 < len(ms) else len(w)]
        tail = tail[:80]
        nums = [int(x) for x in NUM.findall(tail) if 100 <= int(x) <= 5000000]
        # 범위 `a~b` 는 양끝 + (짧으면) 사이 전부
        for j, n in enumerate(nums):
            out.append((f, n))
        for j in range(len(nums) - 1):
            a, b = nums[j], nums[j + 1]
            if 0 < b - a <= RANGE_CAP:
                out.append((f, (a + b) // 2))
    return out


def body_anchors(sp, where=u""):
    u"""명세의 `ir{file,frm,to}` = 그 함수의 IR 본문. 폴백 관측원(창 반경은 `body_rad`).

    ⛔★**`where` 가 이 함수 자신의 소스 파일을 가리킬 때만** 쓴다.
      `18 knobs[22]` 의 `where` 는 `serpen.rs:26`(콜리)인데 명세 본문은 `epic.rs` 다 —
      남의 함수 본문에서 그 값을 못 찾았다고 결함으로 세면 그게 바로
      8차 `G14` 가 46건을 헛되이 낸 「**부재를 결함으로 세기**」다.
      초판이 실제로 그렇게 5건(00[7]·00[8]·18[11]·18[22]·18[24])을 냈다."""
    base = (sp.get("src") or u"").replace(u"/", u"\\").split(u"\\")[-1]
    if not base or base not in (where or u""):
        return []
    ir = sp.get("ir") or {}
    f, a, b = ir.get("file"), ir.get("frm"), ir.get("to")
    if not f or not a or not b:
        return []
    f = f[:-3] if f.endswith(".ll") else f
    if not irsrc(f):
        return []
    return [(f, (int(a) + int(b)) // 2)]


RSREF = re.compile(r"\b([\w]+\.rs)\b")
_SL = None


def _srclinecheck():
    u"""`!dbg` 사슬 해석은 MIG 정본 도구(`srclinecheck.py`)를 그대로 쓴다 — 재구현하지 않는다."""
    global _SL
    if _SL is None:
        sys.path.insert(0, MIG)
        import srclinecheck as SL
        _SL = SL
    return _SL


def src_anchors(sp, where):
    u"""★`where` 의 `epic.rs:635` 같은 **소스 줄**을 그 함수 IR 본문 안에서 되짚어 IR 앵커로 바꾼다.

    앵커 없는 행이 53개(24%)였다. 그 행들은 **아무 검사도 못 받는다.**
    ⟹ 명세의 `ir{file,frm,to}` 구간에서 `!dbg` **inlinedAt 사슬**(`srclinecheck.chain`)을 돌려
      그 소스 줄이 실제로 쓰인 IR 줄을 찾는다.
    ⛔사슬에 없으면 **앵커를 만들지 않는다**(판정 유보). `abstract_input.rs:134` 는
      같은 파일이지만 `ult` 본문(:286~333)이 아닌 **다른 함수**라 여기서 걸러진다 —
      「같은 파일이니 본문 창을 쓰자」는 초판이 그래서 3건을 헛되이 냈다."""
    ir = sp.get("ir") or {}
    f, a, b = ir.get("file"), ir.get("frm"), ir.get("to")
    if not f or not a or not b:
        return []
    want = set()
    ms = list(RSREF.finditer(where or u""))
    for i, m in enumerate(ms):
        tail = (where or u"")[m.end(): ms[i + 1].start() if i + 1 < len(ms) else len(where or u"")][:40]
        for n in NUM.findall(tail):
            n = int(n)
            if 1 <= n <= 99999:
                want.add((m.group(1), n))
    if not want:
        return []
    SL = _srclinecheck()
    try:
        src, meta = SL.load(f)
    except Exception:
        return []
    out = []
    # ★09-13(18차 A·C): 노브 리터럴이 클로저/이터레이터 조각(`ir.aux`)에만 있는 경우가 있다(#59 knobs[6] · #70 knobs[5/6]) — aux 도 같은 파일이면 훑는다.
    rngs = [(f, int(a), int(b))] + [(x.get("file"), int(x.get("frm") or 0), int(x.get("to") or 0)) for x in (ir.get("aux") or []) if x.get("file") == f]
    for (ff, a2, b2) in rngs:
        for k in range(a2 - 1, min(b2, len(src))):
            m = SL.DBG.search(src[k])
            if not m:
                continue
            for (fn, ln) in SL.chain(meta, m.group(1)):
                if (fn, ln) in want:
                    out.append((f[:-3] if f.endswith(".ll") else f, k + 1))
                    break
    return out


def body_rad(sp):
    ir = sp.get("ir") or {}
    try:
        return max(10, (int(ir["to"]) - int(ir["frm"])) // 2 + 2)
    except Exception:
        return 200


def denoise(ln):
    for rx, rep in _DROP:
        ln = rx.sub(rep, ln)
    return ln


def lits_of_line(ln):
    return set(int(x) for x in NUM.findall(denoise(ln)))


def window_lits(anch, rad):
    u"""앵커 창의 정수 리터럴 집합과 **읽힌 줄 수**."""
    lits, nread = set(), 0
    for f, n in anch:
        src = irsrc(f)
        if not src:
            continue
        for x in range(max(1, n - rad), min(len(src), n + rad) + 1):
            nread += 1
            lits |= lits_of_line(src[x - 1])
    return lits, nread


def quotes_of(where):
    u"""`where` 의 백틱 인용 중 **진짜 IR 명령**만."""
    out = []
    for q in QUOTE.finditer(where or u""):
        frag = q.group(1).strip()
        if IROP.match(frag):
            out.append(frag)
    return out


def value_lits(v):
    u"""`value` 가 주장하는 정수 집합. `4,000,000` 꼴의 자리구분 쉼표를 먼저 붙인다."""
    if v is None:
        return set()
    if isinstance(v, bool):
        return set()
    if isinstance(v, int):
        return set([v])
    if isinstance(v, float):
        return set([int(v)]) if float(v).is_integer() else set()
    s = re.sub(r"(?<=\d),(?=\d{3}\b)", u"", u"%s" % v)
    return set(int(x) for x in NUM.findall(s))


HANGUL = re.compile(u"[가-힣]")


def numeric_claim(v):
    u"""★V1 의 **전제**: `value` 칸이 정말 「수를 주장」하는가.

    `value` 에는 산문도 들어 있다(`미열거 15개 태그는 무조건 허용`). 거기서 뽑은 `15` 는
    **노브 값이 아니라 산문 속의 수**다 — 그걸 IR 과 대조하면 전건 오탐이 된다
    (초판이 `12 knobs[6]`·`14 knobs[4]` 에서 그렇게 냈다).
    ⟹ 정수이거나, **한글이 섞이지 않은** 수식·수열일 때만 수 주장으로 본다.
    한글이 섞인 칸의 「값 표기」 문제는 이 게이트의 축이 아니다(별도 관측으로만 남긴다)."""
    if isinstance(v, bool) or v is None:
        return False
    if isinstance(v, (int, float)):
        return True
    return not HANGUL.search(u"%s" % v)


# ─────────────────────────────────────────────────────────────────────────────
# 구제 규칙 — ★**구제에만 쓴다. 기각에는 절대 안 쓴다**(7차 G12 교훈)
RUNTIME = re.compile(u"tick_per_second|tps|런타임 값|런타임값|자리표시|리터럴이 아니라")
CALLEE_OWNED = re.compile(u"\(값은 [A-Za-z_]+\.rs|콜리 안|콜리에|게임코어에|game_core 에|코드 상수는 게임코어")
NOTLITERAL = re.compile(u"오프셋|offset|0x[0-9A-Fa-f]+")


def rescues(k, vl, anch, wide_ok=True, rad=None):
    u"""무죄를 지지하는 **추가 관측**. 하나라도 서면 판정 유보.
    `wide_ok=False` = 인용이 있는 행(인용이 정본이라 창으로 구제하지 않는다)."""
    rad = max(W_WIDE, rad or 0)
    w = (k.get("where") or u"") + u" " + (k.get("effect") or u"") + u" " + (k.get("what") or u"")
    # ① 16진 표기로 관측되나 (`4856` = `0x12f8`)
    hx = set()
    for f, n in anch:
        src = irsrc(f)
        if not src:
            continue
        for x in range(max(1, n - rad), min(len(src), n + rad) + 1):
            for h in re.findall(r"0x([0-9A-Fa-f]{1,12})", src[x - 1]):
                try:
                    hx.add(int(h, 16))
                except ValueError:
                    pass
    if vl & hx:
        return u"16진 리터럴로 관측"
    # ② 광역 창 (인용 없는 행 한정)
    wide, _ = window_lits(anch, rad)
    if wide_ok and (vl & wide):
        return u"광역 창(±%d)에서 관측" % rad
    # ③ ★**시프트로 접힌 곱/나눗셈**. `dot_sq * 4` 는 IR 에 `shl i128 %176, 2` 로 나온다
    #    (`09 knobs[2]` 이 정확히 그 경우였다 — 값 4 는 맞는데 리터럴은 2 다).
    for f, n in anch:
        src = irsrc(f)
        if not src:
            continue
        for x in range(max(1, n - rad), min(len(src), n + rad) + 1):
            for sh in re.findall(r"\b(?:shl|lshr|ashr)\b[^,]*,\s*(\d{1,2})\b", src[x - 1]):
                if (1 << int(sh)) in vl:
                    return u"시프트로 접힘(shl %s = ×%d)" % (sh, 1 << int(sh))
    # ④ ★**제곱거리**로 관측되나. 거리 임계는 IR 에서 `dist² > d²−1` 로 나온다
    #    (`15 knobs[7]` = 200000/300000 ↔ IR `39999999999`/`89999999999`).
    for d in vl:
        if 0 < d < 10 ** 7:
            for cand in (d * d, d * d - 1, d * d + 1):
                if cand in wide:
                    return u"제곱거리로 관측(%d² = %d)" % (d, cand)
    # ⑤ ★19차 A: 값이 **비인라인 콜리** 안에 있다고 본문이 자인 — `where` 가 담당 밖 `.rs` 를 괄호로 병기(`(값은 player.rs:352~355 …)`)
    #    하거나 effect 가 「콜리/게임코어/game_core」 를 말하면 이 함수 창엔 리터럴이 없는 게 정상(판정 유보 · [79] 480 = judge_battle_latency g15.ll:126049).
    if CALLEE_OWNED.search(w):
        return u"값이 콜리/게임코어에 있다고 본문이 자인(이 함수는 비교만)"
    # ③ 런타임 값·자리표시 자인
    if RUNTIME.search(w):
        return u"런타임 값/자리표시임을 본문이 자인"
    if NOTLITERAL.search(w):
        return u"오프셋·16진 표기"
    return None


def nodbg_rescue(sp, vl):
    u"""⑥ ★19차 C([91] knobs[0]=21): 점프스레딩/호이스트된 비교는 **`!dbg` 없는 명령**으로 본문 어딘가에 놓인다
    (m04.ll:29351 `icmp ult i64 %265, 21` — `br i1 %205 … !dbg L206` 의 타깃 블록). 앵커 창엔 없지만 이 함수 본문(aux 포함)의
    dbg 없는 명령에 리터럴이 있으면 판정 유보. ⚠dbg 있는 줄은 세지 않는다(그건 창·앵커의 몫 · USE_BODY 오탐 재발 방지)."""
    ir = sp.get("ir") or {}
    segs = [(ir.get("file"), ir.get("frm"), ir.get("to"))] + [(x.get("file") or ir.get("file"), x.get("frm"), x.get("to")) for x in (ir.get("aux") or [])]
    for (f, a, b) in segs:
        if not f or not a or not b:
            continue
        f = f[:-3] if f.endswith(".ll") else f
        src = irsrc(f)
        if not src:
            continue
        for x in range(int(a), min(len(src), int(b)) + 1):
            ln = src[x - 1]
            if "!dbg" in ln or "#dbg" in ln or not re.match(r"\s+%\d+ = (?:icmp|select|add|sub|mul|and|or|xor|lshr|shl)\b", ln):
                continue
            if vl & lits_of_line(ln):
                return u"dbg 없는 명령(점프스레딩/호이스트)에서 관측 %s:%d" % (f, x)
    return None


# ─────────────────────────────────────────────────────────────────────────────
def check_spec(sp, want=("V1", "V3", "V4"), detail=None):
    u"""★`specgate G18` 진입점. `[(knobs 인덱스, 사유, 상세)]`."""
    out = []
    seen_arm = {}                     # (file,line,armvalue) → 첫 노브 인덱스
    for j, k in enumerate(sp.get("knobs") or []):
        where = k.get("where") or u""
        anch = anchors(where)
        # ★`where` 가 `epic.rs:635` 처럼 **소스 줄만** 적은 행이 53개다(전체의 24%).
        #   그 행은 IR 앵커가 없어 **아무 검사도 못 받는다** — 축을 닫는다면서 1/4 을 비우는 셈.
        #   ⟹ 명세 자신이 들고 있는 `ir{file,frm,to}`(그 함수의 IR 본문 범위)를 **폴백 앵커**로 쓴다.
        #   ⚠본문 창은 넓어서 구제가 후하다(=검출력이 낮다). 그래도 **0 보다는 낫다**.
        #   ⚠앵커가 있는 행에는 절대 더하지 않는다 — 관측을 더하면 그 행의 검출력이 떨어진다.
        body = False
        if not anch:
            anch = src_anchors(sp, where)          # ①소스줄 → !dbg 사슬 역추적(정밀)
        # ②~~함수 본문 전체를 창으로~~ = **폐기**.  참조 — 오탐 3건(00[7]·00[8]·10[5])
        if USE_BODY and not anch:
            anch = body_anchors(sp, where)
            body = bool(anch)
        qs = quotes_of(where)
        qlits = set()
        for q in qs:
            qlits |= lits_of_line(q)
        vl = value_lits(k.get("value"))
        rad = body_rad(sp) if body else W
        wl, nread = window_lits(anch, rad)
        obs = qlits | wl
        if detail is not None:
            detail.append((j, k.get("what"), k.get("value"), anch[:4], sorted(vl)[:6],
                           sorted(obs)[:14], nread))

        # ── V3 값 칸 부재
        if "V3" in want and "value" not in k:
            out.append((j, u"`value` 칸이 없다", (k.get("what") or u"")[:50]))

        # ── V1 값 미관측 (반증식) · **관측원을 2등급으로 가른다**
        #
        #   ★**인용이 있으면 인용이 정본이다.** `where` 가 `` `icmp ugt i64 %30, 4` `` 처럼
        #     명령을 통째로 인용했다면 그 명령의 피연산자가 **그 노브가 가리키는 바로 그 수**다.
        #     이때 창(window)까지 구제에 쓰면, IR 어디에나 흔한 수(0·1·2·20·100)가
        #     **무엇이든 무죄로 만들어** 검출력이 죽는다(실측: 변이 포착 31.5% → 아래).
        #   ★인용이 없으면(줄만 적힌 행) 창으로 본다 — 이때만 광역 구제를 허용한다.
        if ("V1" in want and vl and numeric_claim(k.get("value")) and anch and nread and obs):
            if qlits:
                ok, src = bool(vl & qlits), u"인용"
            else:
                ok, src = bool(vl & wl), u"창"
            if not ok:
                r = rescues(k, vl, anch, wide_ok=(not qlits), rad=rad)
                if not r and not qlits:
                    r = nodbg_rescue(sp, vl)
                if not r:
                    out.append((j, u"주장한 값을 IR 관측에서 못 찾는다",
                                u"value=%s · 관측(%s)=%s%s" % (
                                    k.get("value"), src,
                                    sorted(qlits if qlits else obs)[:10],
                                    u" · 인용=%s" % qs[0][:50] if qs else u"")))
                elif detail is not None:
                    detail.append((j, u"[구제] " + r, k.get("value"), anch[:2],
                                   sorted(vl)[:6], [], 0))

        # ── V4 같은 select/phi arm 중복 주장
        if "V4" in want:
            for q in qs:
                if not re.match(r"^(select|phi|%[\w.]+ = (select|phi))", q):
                    continue
                arms = sorted(lits_of_line(q))
                if len(arms) < 2:
                    continue
                key = (u"|".join(sorted(str(a) for a in arms)), tuple(sorted(a for a in anch)[:1]))
                for a in arms:
                    if a in vl:
                        kk = (key, a)
                        if kk in seen_arm and seen_arm[kk] != j:
                            out.append((j, u"같은 select/phi 의 **같은 arm** 을 두 노브가 주장",
                                        u"knobs[%d] 와 겹침 · arm=%s · 인용=%s"
                                        % (seen_arm[kk], a, q[:50])))
                        else:
                            seen_arm[kk] = j
    return out


def load():
    return json.load(io.open(os.path.join(MIG, "_spec", "specs20_v3.json"), encoding="utf-8"))


def run(want, verbose=False):
    D = load()
    tot = rows = 0
    allout = []
    for i, sp in enumerate(D["specs"]):
        det = [] if verbose else None
        r = check_spec(sp, want, det)
        rows += len(sp.get("knobs") or [])
        tot += len(r)
        if verbose and det:
            print(u"\n##### specs[%d] %s" % (i, sp["name"]))
            for d in det:
                print(u"  [%d] %s | value=%r | anch=%s | vl=%s | obs=%s (%d줄)" % d)
        for j, why, dt in r:
            allout.append((i, sp["name"], j, why, dt))
    return rows, allout


if __name__ == "__main__":
    args = sys.argv[1:]
    if "--mutate" in args:
        import mutate                                   # 같은 폴더
        mutate.main()
        sys.exit(0)
    want = ("V1", "V3", "V4")
    if "--v2" in args:
        want = want + ("V2",)
    rows, allout = run(want, "--detail" in args)
    cur = None
    for i, nm, j, why, dt in allout:
        if cur != i:
            print(u"\n=== specs[%d] %s" % (i, nm)); cur = i
        print(u"   knobs[%d] %s — %s" % (j, why, dt))
    print(u"\n" + u"=" * 80)
    print(u"knobs %d행 검사 · 적발 %d건" % (rows, len(allout)))
    print(u"=" * 80)
