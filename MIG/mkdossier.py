# -*- coding: utf-8 -*-
u"""mkdossier — 배치 하나가 받을 **정본 전량**을 한 파일로 조립한다(요약 금지). (2026-09-11 신설)

## `mkbrief` 와 무엇이 다른가 — **요약하느냐 마느냐**
유저 지적: *"지시에 누락이 생기는 게 있는 정보를 요약하거나 하면서 그런 거 같아.
그러지 말고 명세·분석방법·분석도구를 결과 받을 때마다 수정하고, **이걸 통째로** 넘기는 게 낫겠다."*

이력이 그 말을 뒷받침한다 — 브리핑 오류는 **전부 「줄여 적는 자리」에서 났다**:

| 라운드 | 사고 | 줄인 것 |
|---|---|---|
| 3차 | 「불가능」 3건이 4차에 전부 뒤집힘 | 근거를 빼고 **결론만** 옮겼다 |
| 4차 | `tcxaudit` 기준선 수치를 네 배치 전부가 지적 | 인자가 다른 실행값을 **수치만** 옮겼다 |
| 5차 | `game_ai::is_ignored_well_enemy`(rustc E0425) | 모듈 **경로를 잘랐다** |
| 5차 | ev 상향 492행 중 **317행 유실** | 보고를 **집계표로** 받았다 |
| 6차 | `BRIEF_FACTS.md` 가 정본보다 낡음 | 생성물을 **한 번 만들고 재사용**했다 |

⟹ 도시에는 **자르지 않는다.** `mkbrief` 가 `[:110]` 로 썰던 자리를 전부 전문으로 싣는다.
길면 길게 둔다 — 파일이라 토큰이 아니라 **디스크**를 쓴다.

## 담는 것
 §0 정본 스탬프(sha256) · 신선도 강제      §1 담당 함수와 이번 라운드의 표적
 §2 읽어야 할 정본 문서 = **경로+해시**    §3 담당 함수 명세 **전문**(무손실)
 §4 게이트 미해소 — **이 배치 몫만**       §5 보고 형식 = `patch.json` 스키마
 §6 출력 디렉터리(배치별 분리 — 경합 방지)

사용:
  python -X utf8 mkdossier.py 7             → `_verify7/DOSSIER_{A,B,C,D}.md`
  python -X utf8 mkdossier.py 7 --batch C   → C 만
  python -X utf8 mkdossier.py 7 --check     → 생성 없이 검증만
"""
import re, hashlib
import io
import json
import os
import subprocess
import sys
import time

sys.stdout.reconfigure(encoding="utf-8", errors="replace")
HERE = os.path.dirname(os.path.abspath(__file__))
V3 = os.path.join(HERE, "_spec", "specs20_v3.json")
V2 = os.path.join(HERE, "_spec", "specs20.json")

D = json.load(io.open(V3, encoding="utf-8"))
M, S = D["meta"], D["specs"]

BATCH = (("A", 0, 4), ("B", 5, 9), ("C", 10, 14), ("D", 15, 19))
# ★`--idx lo-hi`(09-13 · r7 잎 20~39 부터): 그 범위를 4등분해 A~D 로 배정한다. 없으면 20함수 기본값.
if "--idx" in sys.argv:
    _lo, _hi = [int(x) for x in sys.argv[sys.argv.index("--idx") + 1].split("-")]
    _n = _hi - _lo + 1; _q = (_n + 3) // 4
    BATCH = tuple((t, _lo + k * _q, min(_hi, _lo + (k + 1) * _q - 1)) for k, t in enumerate("ABCD") if _lo + k * _q <= _hi)
# ★`--batches A:102-104,B:105-107,C:108,D:109,E:110`(09-14 · r11 거대 9): 함수 크기가 3~11k줄로 들쭉날쭉해 등분이 무의미 → 손 배정.
if "--batches" in sys.argv:
    _b = []
    for tok in sys.argv[sys.argv.index("--batches") + 1].split(","):
        t, rg = tok.split(":")
        lo, hi = (rg.split("-") + [None])[:2]
        _b.append((t, int(lo), int(hi if hi else lo)))
    BATCH = tuple(_b)

# ★게이트 수를 **손으로 적지 마라.** 7차에 `G12` 수치가 세 곳에 다르게 적혀 있었다.
try:
    import specgate as _SG
    NGATE = len(_SG.GATES)
except Exception:
    NGATE = 0

EV_FIELDS = ("mem", "consts", "knobs")

# ★★**축 모드**(8차부터) — 배치를 **함수가 아니라 축으로** 나눈다.
#
# 왜 바꾸나: 7차 실측에서 실오류 42건의 **88%가 「이번에 처음 검사한 축」**에서 나왔고,
# **6차까지 검사받던 축에서 새로 나온 오류는 0건**이었다. ⟹ 남은 일은 「함수를 또 도는 것」이
# 아니라 **축을 세우는 것**이다. 그리고 축마다 배치 넷이 검사기를 **각자 하나씩 만들어 놨는데**
# 적발 수가 3 / 7 / 46 으로 갈린다 — 누가 맞는지는 **비교·반증으로만** 정해진다.
# (내가 골라 승격하면 또 소수 표본 과적합이다. 7차에 G12 를 그렇게 고치다 16→22 로 늘렸다.)
AXIS = (
    ("A", "mem_dir", u"`mem[].dir` — 읽기/쓰기 방향", "mem", "dir",
     u"451행. 후보 **4벌**(A·B·C·D)이 전역에서 각각 **불일치 3 / 46 / 7** 로 갈린다."),
    ("B", "consts_kind", u"`consts[].kind` — 상수의 종류", "consts", "kind",
     u"186행. ★**무검사가 아니라 「무측정」 축**이다 — `mkspec3` 이 `meaning` 낱말로 유도하고 "
     u"**잔여를 전부 `임계`로 떨군다**(그래서 「임계 111행」은 분류가 아니라 나머지다). "
     u"게다가 **v2 에 없는 파생 필드**라 `errors[]` 로 직접 못 쓴다."),
    ("C", "params_role", u"`sig.params[].role` — 인자의 역할", "sig.params", "role",
     u"124행. 후보 4벌. ★7차에 배치C 는 「초판 6건이 전부 오탐」이라 했고 배치B 는 「담당 0건」이었다 — "
     u"**절 단위 분해와 SROA 인자 제외**가 설계의 갈림길이다."),
    ("D", "history_prop", u"`history` → 표 **전파**", "history", None,
     u"★7차에 새로 드러난 축. `G8` 이 있지만 **`logic`↔표만** 본다 — "
     u"`history` 가 몇 라운드 전에 확정한 값이 `mem`/`knobs` 표에 안 실려도 통과한다(7차에 최소 3건). "
     u"**이미 답이 나와 있어 분석이 아니라 전파만 하면 되는, 가장 싸게 닫히는 축**이다."),
)

# ★9차 — **정적으로 남은 마지막 축들.** 8차 뒤 게이트가 `G12` 하나만 열려 있고,
#   나머지는 「아직 게이트가 없는 축」이다(배치B 가 7차 §6 에 열거한 것 + 8차 보류분).
#   이 라운드 뒤에는 정적으로 더 얻을 게 거의 없다 — 그다음은 **런타임 `game==mine`** 이다
#   (현재 `ev1` = **0행**. 게이트 0 은 「명세가 IR 과 어긋나지 않는다」이지 「게임이 그렇게 돈다」가 아니다).
AXIS9 = (
    ("A", "src_line", u"`consts[].src_line` — G12 **남은 8건 가리기**", "consts", "src_line",
     u"★**이건 「8건 고치기」가 아니라 「8건이 진짜인지 가리기」다.** 8차 배치C 가 이미 **3건을 오탐으로 확정**했다"
     u"(상수가 `shl` 로 접혀 리터럴이 소멸 2 · `phi` 인입이라 `!dbg` 가 없음 1). "
     u"나머지 5건은 아무도 안 봤다. `G12` 는 6차에 붙자마자 56건을 냈는데 검증해 보니 **대부분 오탐**이었던 "
     u"전력이 있다 — **이 축의 역사가 곧 「적발은 가설이다」의 원본 사례**다."),
    ("B", "params_role", u"`sig.params[].role` — **G16 승격**", "sig.params", "role",
     u"8차 배치C 가 통합판을 만들었고 sret 규약도 확정·반영됐다(`P2` 2건이 예고대로 사라졌다). "
     u"⛔그런데 검사기가 함께 내는 **`P4` 6건 · `P6` 14건을 아무도 반증하지 않아 승격을 보류**했다. "
     u"⟹ 그 20건을 IR `define` 원문으로 가리고, 살아남는 것만 남긴 뒤 **`G16` 으로 승격 가능한 상태**로 만들어라."),
    ("C", "knobs_value", u"`knobs[].value` — 노브의 **값**", "knobs", "value",
     u"★`G13` 은 `where`(위치)와 인용 문면만 본다 — **값은 아무도 안 본다.** "
     u"8차 배치B 가 `08 knobs[4]` 의 값이 **자리표시 `0`** 인 것을 발견했다. "
     u"노브는 **바꿔 쓰라고 있는 칸**이라 값이 틀리면 재구현이 아니라 **사용자가 직접 당한다**."),
    ("D", "mem_offset", u"`mem[].offset` 표기 규약 + `logic`↔표 **상호참조**", "mem", "offset",
     u"두 가지를 맡는다. ①**`offset` 표기 규약** — `0x860[len]` 처럼 좌표계가 섞인 값이 있어 "
     u"8차에 검사기 넷이 **각자 다르게 파싱**했고 그게 `A=7 vs D=7` 인데 집합이 다른 원인이었다. "
     u"②**`logic` ↔ `mem`/`consts` 상호참조**(`G18` 제안) — 「`logic` 이 인용한 오프셋·상수가 표에 있나」는 "
     u"**완전 무검사**다. `G6`/`G8` 은 *값이 어긋났나*만 보고 *아예 빠졌나*는 안 본다."),
)

# ★읽어야 할 정본 — **요약하지 않고 경로와 해시만 준다.** 에이전트가 직접 읽는다.
#   여기 목록이 곧 「분석방법 정본」이고, 방법이 늘면 파일을 고치지 이 목록의 설명을 늘리지 않는다.
CANON = [
    (os.path.join(HERE, "METHOD_MAP.md"),
     u"★**「불가」를 쓰기 전에 §0 라우팅표를 보라.** 09-11 에 「원리적 불가」가 7건 뒤집혔고 "
     u"원인은 매번 「가진 재료의 한계를 문제의 한계로 착각」이었다."),
    (os.path.join(HERE, "SPEC_RUNBOOK.md"),
     u"★라운드 절차. **§S5-b 게이트표 · §S5-c 「검사받지 않는 축」 · §S5-d 파이프라인 4계약**."),
    (os.path.join(HERE, "SPEC_GUIDE.md"),
     u"명세 필드의 뜻과 채우는 법."),
    (os.path.join(HERE, "TOOLS.md"),
     u"도구 인벤토리(자동 생성). **무엇이 있는가**만 센다 — 무엇을 집는가는 `METHOD_MAP §0`."),
    (os.path.join(HERE, "_verify3", "TEMPLATE.rs"),
     u"★오라클 작성 템플릿. **함정 ①~⑧ · 수법 ⓐ~ⓕ** — 오라클을 쓸 거면 먼저 읽어라."),
]


def kb(s):
    u"""KB — ★**바이트 기준**이다. (10차 배치D 적발)

    문자 수로 세면 한글이 UTF-8 에서 3바이트라 **~30% 과소**로 찍힌다.
    그 수치로 「60KB 넘지 말라」를 판정하면 실제로는 80KB 짜리를 통과시킨다."""
    return len(s.encode("utf-8")) / 1024.0


def sha16(p):
    try:
        return hashlib.sha256(io.open(p, "rb").read()).hexdigest()[:16]
    except Exception:
        return u"(없음)"


def nlines(p):
    try:
        return sum(1 for _ in io.open(p, encoding="utf-8", errors="replace"))
    except Exception:
        return 0


def fresh_or_die():
    u"""생성물이 정본보다 이르면 틀린다(6차 배치C 적발). `mkbrief.fresh_or_die` 와 같은 규칙."""
    if os.path.getmtime(V3) < os.path.getmtime(V2):
        print(u"★`specs20_v3.json` 이 `specs20.json` 보다 낡았다 — `mkspec3.py` 를 먼저 돌려라.")
        print(u"   v3 %s  <  v2 %s"
              % (time.strftime("%H:%M:%S", time.localtime(os.path.getmtime(V3))),
                 time.strftime("%H:%M:%S", time.localtime(os.path.getmtime(V2)))))
        sys.exit(2)


def run(cmd):
    try:
        e = dict(os.environ, PYTHONIOENCODING="utf-8")
        out = subprocess.run([sys.executable, "-X", "utf8"] + cmd, cwd=HERE,
                             capture_output=True, text=True, encoding="utf-8",
                             errors="replace", env=e, timeout=1800)
        return (out.stdout or u"") + (out.stderr or u"")
    except Exception as ex:
        return u"(실행 실패: %s)" % ex


# ── 게이트 출력을 배치별로 가른다 ──────────────────────────────────────
def gate_split(txt, lo, hi):
    u"""`specgate.py` 출력에서 `  [NN] 이름 ...` 행을 읽어 담당 구간만 남긴다.
    ★들여쓰기 연속행(실제 후보·인용)을 **같이 끌고 온다** — 잘라내면 후보 목록이 사라져
    배치가 「어디로 고쳐야 하는지」를 잃는다(그게 요약 사고의 축소판이다)."""
    out, cur, keep = [], None, False
    for l in txt.split(u"\n"):
        if l.startswith(u"### ["):
            cur, keep = l, False
            continue
        s = l.strip()
        # ★09-14(20차 C 적발): `[108]` 처럼 세 자리 idx 를 두 자리로 읽어 담당 밖으로 버렸다 — `]` 까지 읽는다.
        m_idx = re.match(r"\[(\d+)\] ", s)
        if m_idx:
            idx = int(m_idx.group(1))
            keep = (lo <= idx <= hi)
            if keep:
                if cur:
                    out.append(u"")
                    out.append(u"**%s**" % cur.replace(u"### ", u""))
                    cur = None
                out.append(u"  " + s)
        elif keep and l.startswith(u"        "):
            out.append(u"  " + l.strip())
        elif not s:
            keep = False
    return out


# ── 명세 무손실 렌더 ───────────────────────────────────────────────────
def _cell(v):
    if v is None:
        return u""
    if isinstance(v, (dict, list)):
        return json.dumps(v, ensure_ascii=False).replace(u"|", u"\\|")
    return (u"%s" % v).replace(u"|", u"\\|").replace(u"\n", u" ")


def table(rows, cols, labels):
    u"""★**자르지 않는다.** `mkbrief` 가 `[:110]` 로 썰던 자리 — 여기가 요약 사고의 발원지였다.

    ★★**그리고 「없는 키」도 만들지 않는다.** (10차 배치B·C 적발 — 이 라운드 최악의 결함)
      고정 컬럼 이름이 실제 키와 어긋나 **세 축이 전 라운드·전 배치에서 공백**이었다:
        `sig.params` 의 `ty`(실제 `type`) → 인자표 31행 **타입 전부 공란**
        `siblings.entries` 의 `("name","sym","note")`(실제 `path`/`vis`/`at`/…) → **형제 표 전 행 공란**
        `closed` 의 `("a","ev")`(실제 `why`) → **닫은 근거가 전 함수에서 공백**
      10차는 바로 그 축들을 검사하라고 시킨 라운드였는데 **도시에가 그 칸을 숨기고 있었다.**
      게다가 `history` 는 **함수마다 키가 다른 자유 키**라 고정 컬럼으로는 원리적으로 못 찍는다.

    ⟹ **행에 실제로 있는 키는 빠짐없이 찍는다.** `cols` 는 이제 「앞에 세울 순서」일 뿐이고,
      거기 없는 키는 뒤에 자동으로 붙는다. 이름이 어긋나도 **값이 사라지지 않는다.**
    """
    extra = []
    for r in rows:
        if not isinstance(r, dict):
            continue
        for k in r:
            if k not in cols and k not in extra:
                extra.append(k)
    allc = list(cols) + extra
    alll = list(labels) + extra
    o = [u"| " + u" | ".join(alll) + u" |",
         u"|" + u"---|" * len(alll)]
    for i, r in enumerate(rows):
        if not isinstance(r, dict):
            o.append(u"| %d | %s |" % (i, _cell(r)) + u" |" * (len(allc) - 1))
            continue
        o.append(u"| %d | " % i + u" | ".join(_cell(r.get(c)) for c in allc) + u" |")
    return o


def render_fn(sp):
    o = []
    a = o.append
    a(u"---")
    a(u"")
    a(u"### `%02d` %s — %s" % (sp["i"], sp["name"], sp.get("one_line") or u""))
    a(u"")
    a(u"| 항목 | 값 |")
    a(u"|---|---|")
    a(u"| id | `%s` |" % sp.get("id"))
    a(u"| 심볼 | `%s` |" % sp.get("sym"))
    a(u"| 소스 | `%s:%s` |" % (sp.get("src"), sp.get("src_line")))
    ir = sp.get("ir") or {}
    a(u"| IR | `%s` %s~%s행 |" % (ir.get("file"), ir.get("frm"), ir.get("to")))
    sig = sp.get("sig") or {}
    a(u"| 경로·가시성 | `%s` · **%s** |" % (sig.get("path"), sig.get("vis")))
    a(u"| 계층 | %s |" % sp.get("layer"))
    ex = sp.get("exe") or {}
    # ★`exe` 가 없으면 **「None(None)·None바이트」로 찍지 마라.** (10차 배치A·C 지적)
    #   그 표기는 「exe 에 독립 함수가 없다(인라인·`internal fastcc`)」와
    #   「조인에 실패했다」를 **구별하지 못한다** — `callers: []` 에 이미 같은 주의가 있다.
    if not ex or not ex.get("addr"):
        a(u"| exe | **없음** — 「exe 에 독립 함수가 없다(인라인·`define internal fastcc`)」인지 "
          u"**「조인 실패」**인지는 이 칸만으로 못 가른다. `dllmatch.py`·`name2rva.py` 로 확인하라 |")
    else:
        a(u"| exe | `%s` (%s) · %s바이트 · %s명령 |"
          % (ex.get("addr"), ex.get("module"), ex.get("bytes"), ex.get("instrs")))
    a(u"| 라운드 | 기준 `%s` · 통과 %s회 |" % (sp.get("base_round"), sp.get("rounds")))
    a(u"")
    a(u"**시그니처(tcx 정본, ev%s)**" % sig.get("ev"))
    a(u"```rust")
    a(sig.get("tcx") or u"(없음)")
    a(u"```")
    if sig.get("params"):
        a(u"")
        a(u"<details><summary>인자 %d개</summary>\n" % len(sig["params"]))
        o += table(sig["params"], ("i", "name", "type", "role", "ev"), (u"#", u"i", u"이름", u"타입", u"역할", u"ev"))
        a(u"</details>")
    a(u"")
    a(u"**의사코드 `logic`**")
    a(u"> %s" % (sp.get("logic_note") or u""))
    a(u"```")
    a(sp.get("logic") or u"(없음)")
    a(u"```")
    a(u"")

    a(u"**`mem` 메모리 접근 %d건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서)."
      % len(sp["mem"]))
    o += table(sp["mem"], ("base", "offset", "name", "dir", "note", "ev"),
               (u"#", u"베이스", u"오프셋", u"이름", u"방향", u"근거", u"ev"))
    a(u"")
    a(u"**`consts` 상수 %d건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다." % len(sp["consts"]))
    o += table(sp["consts"], ("value", "src_line", "kind", "meaning", "ev"),
               (u"#", u"값", u"src_line", u"종류", u"뜻", u"ev"))
    a(u"")
    a(u"**`knobs` 조정점 %d건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다."
      % len(sp["knobs"]))
    o += table(sp["knobs"], ("what", "where", "value", "effect", "ev"),
               (u"#", u"무엇", u"어디", u"값", u"효과", u"ev"))
    a(u"")
    a(u"<details><summary>`callees` 피호출자 %d건 (tcx 자동 생성)</summary>\n" % len(sp["callees"]))
    o += table(sp["callees"], ("name", "path", "vis", "sig", "at"),
               (u"#", u"이름", u"경로", u"vis", u"시그니처", u"정의처"))
    a(u"</details>")
    um = sp.get("callees_unmatched") or {}
    if um.get("names"):
        a(u"")
        a(u"⚠**미매칭 %d개**: %s" % (len(um["names"]), u", ".join(u"`%s`" % x for x in um["names"])))
        a(u"> %s" % um.get("note", u""))
    a(u"")
    cal = sp.get("callers") or {}
    sib = sp.get("siblings") or {}
    a(u"**호출처 %s곳** (%s) · **형제 %s개** %s"
      % (cal.get("count"), u", ".join(cal.get("sites") or []) or u"-",
         sib.get("count"), u"(%s)" % sib.get("plan") if sib.get("plan") else u""))
    if sib.get("entries"):
        a(u"")
        o += table(sib["entries"], ("path", "vis", "at"), (u"#", u"경로", u"vis", u"정의처"))
    a(u"")

    op = sp.get("open") or []
    a(u"**`open` %d건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**" % len(op))
    if op:
        o += table(op, ("class", "q", "ev", "tried"), (u"#", u"분류", u"물음", u"ev", u"시도"))
    else:
        a(u"")
        a(u"(없음 — 이 함수는 `open` 이 비었다. 그래도 **게이트 미해소(§4)와 반증은 유효하다**.)")
    a(u"")
    nt = sp.get("notes") or []
    if nt:
        a(u"**`notes` %d건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**" % len(nt))
        o += table(nt, ("q", "ev"), (u"#", u"내용", u"ev"))
        a(u"")
    cl = sp.get("closed") or []
    a(u"<details><summary>`closed` %d건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>\n"
      % len(cl))
    if cl:
        o += table(cl, ("q", "why"), (u"#", u"물음", u"닫은 근거"))
    a(u"</details>")
    a(u"")
    hs = sp.get("history") or []
    a(u"<details><summary>`history` 정정 이력 %d건 (참조용 — 본문 아님)</summary>\n" % len(hs))
    if hs:
        # ⚠`history` 는 **함수마다 키가 다른 자유 키**다(`오프셋`·`store지점`·`kill_time식` …).
        #   고정 컬럼으로는 원리적으로 못 찍으므로 `was`/`now` 만 앞세우고 나머지는 `table` 이 자동으로 붙인다.
        o += table(hs, ("was", "now"), (u"#", u"옛 값", u"현재"))
    a(u"</details>")
    a(u"")
    return o


# ── 본체 ───────────────────────────────────────────────────────────────

def sect5_6(a, rnd, tag, lo):
    lo = 0 if lo is None else lo   # 축 모드는 담당 구간이 없다(예시 인덱스만 쓰인다)
    u"""§5 보고 형식 · §6 출력 디렉터리 — **함수 모드와 축 모드가 공유한다.**

    ★한 군데에만 둔다. 7차에 `mkpatch` 와 `applypatch` 가 경로 문법을 **각자 베껴 두었다가**
      한쪽만 고쳐져 3단 경로에서 갈라졌다(`cleanspec.KEEP` 때와 같은 사고).
      지시문의 보고 형식도 같은 성질이라 **복사하면 반드시 갈라진다.**
    """
    # §5 보고 형식
    a(u"## §5 보고 형식 — **`patch.json` 하나. 산문 보고서는 부수적이다**")
    a(u"")
    a(u"★5차에 ev 상향 **492행 중 317행이 유실**됐다. 원인은 보고를 **집계표로 받은 것**이다.")
    a(u"기계가 적용할 수 있는 형식으로만 받는다 — 사람이 옮겨 적는 경계를 없앤다.")
    a(u"")
    a(u"★**이 스키마는 `applypatch.py` 의 실제 계약이다.** 다른 최상위 키를 쓰면 도구가 **적용을 거부**한다")
    a(u"(7차 배치A 적발 — 이 자리에 없는 계약 `entries` 가 예시로 실려 있었고, 그대로 냈으면 **0건 적용**이었다).")
    a(u"")
    a(u"```json")
    a(u"{")
    a(u"  \"round\": %s, \"batch\": \"%s\"," % (rnd, tag))
    a(u"  \"errors\": [")
    a(u"    {\"path\": \"/specs[%d]/consts[3]/src_line\"," % lo)
    a(u"     \"kind\": \"실오류\",")
    a(u"     \"old\": 293, \"new\": 310,")
    a(u"     \"evidence\": \"m04.ll:44120 `store i64 2, !dbg !56400` (!56400 = abstract_input.rs:310).")
    a(u"                   293 은 함수 머리줄이다\",")
    a(u"     \"behavior_change\": false,")
    a(u"     \"found_by\": \"new\"}")
    a(u"  ],")
    a(u"  \"ev_up\": [")
    a(u"    {\"path\": \"/specs[%d]/mem[0]\", \"from\": 4, \"to\": 3," % lo)
    a(u"     \"evidence\": \"tcx 정본 대조: offset_of!(PlayerState, info.team)==0x930\",")
    a(u"     \"found_by\": \"new\"}")
    a(u"  ],")
    a(u"  \"brief_errors\": [\"이 도시에에서 발견한 내 지시의 오류 — 문장으로\"]")
    a(u"}")
    a(u"```")
    a(u"")
    a(u"| 키 | 뜻 |")
    a(u"|---|---|")
    a(u"| `errors[]` | 값·문면 정정. `kind` = `실오류` / `오탐`(게이트가 틀렸다) / `보강` |")
    a(u"| `errors[].evidence` | ★**IR 줄 원문을 인용하라.** 「확인했다」는 근거가 아니다 |")
    a(u"| `errors[].behavior_change` | ★**이 명세대로 재구현하면 틀린 동작이 나오는가**(고친 뒤엔 안 나오는가). "
      u"「지금 재현 코드가 바뀌나」가 **아니다** — 7차에 네 배치가 그렇게 읽어 전건 `false` 로 냈는데, "
      u"실제로는 노브 arm 방향 뒤집힘·쓰기 대상 오기·존재하지 않는 노브 등 **8건이 재구현을 틀리게** 했다 |")
    a(u"| `errors[].found_by` | `new`(이번에 처음) / `reused`(전 라운드 기법 재사용) — **집계에 쓴다** |")
    a(u"| `ev_up[]` | 증거등급 **이동**. `from`/`to` 는 숫자지만 **`ev` 필드를 직접 쓰는 게 아니라** 근거 갱신 요청이다 |")
    a(u"| `brief_errors[]` | ★**내 지시(이 도시에)의 오류.** 매 라운드 나왔다 — 비워 두지 마라 |")
    a(u"")
    a(u"제출 전 **반드시** 사전 검증하라(적용 없이 성공/실패만 본다):")
    a(u"```bash")
    a(u"cd /c/tfm2mods/MIG && PYTHONIOENCODING=utf-8 python -X utf8 applypatch.py %s --only %s --dry"
      % (rnd, tag))
    a(u"```")
    a(u"")
    a(u"경로 문법(`applypatch.py` 가 파싱한다):")
    a(u"")
    a(u"```")
    a(u"/specs[i]/<필드>                     예: /specs[3]/one_line")
    a(u"/specs[i]/<배열>[n]/<키>             예: /specs[3]/consts[2]/src_line")
    a(u"/specs[i]/<바깥>/<필드>              예: /specs[3]/sig/vis")
    a(u"/specs[i]/<바깥>/<배열>[n]/<키>      예: /specs[3]/sig/params[1]/role")
    a(u"```")
    a(u"")
    a(u"규칙:")
    a(u"- `old` 는 **현재 값과 정확히 일치**해야 한다(불일치 = 그 항목만 거부 · 나머지는 적용).")
    a(u"  정수·불리언 필드(`src_line` 등)는 **문자열로 써도 된다** — 도구가 현재 필드 타입으로 되돌려 넣는다.")
    a(u"  ⟹ 네가 본 값이 정본과 다르면 **네 도시에가 낡은 것**이다. §0 해시부터 다시 확인하라.")
    a(u"- ★**행 추가·삭제가 된다** — `{\"op\":\"insert\", \"path\":\"/specs[i]/mem\", \"at\":N, "
      u"\"guard\":\"<식별 문자열>\", \"guard_key\":\"name\", \"new\":{...}}` · `{\"op\":\"delete\", ...}`.")
    a(u"  ⚠**삽입은 뒤 인덱스를 전부 민다** — 같은 배열의 `ev_up` 경로는 **삽입 후 인덱스**로 써라. "
      u"적용 뒤 메인이 `--restamp` 를 돌린다.")
    a(u"  ⛔단 `open`/`notes` 에는 **못 쓴다**(v3 가 v2 의 두 배열을 걸러 만든 것이라 인덱스가 안 맞는다). "
      u"그쪽은 산문으로 보고하면 메인이 넣는다.")
    a(u"- ★**`ev` 를 `errors[]` 로 직접 쓰지 마라** — `ev` 는 근거 문면에서 파생되는 값이다"
      u"(`mkspec3.evtier`). 등급을 옮기려면 **`ev_up[]`** 을 쓰고 `evidence` 에 새 근거를 대라.")
    a(u"  (`mem` 의 `ev` 는 근거가 무엇이든 **상한 3**이다. 오프셋 주장의 최강 근거가 tcx 이기 때문.)")
    a(u"- ⚠**파생 필드는 `errors[]` 로 못 쓴다.** `consts.kind` 는 v2 에 없고 `mkspec3.py` 가 "
      u"`meaning` 의 낱말에서 정한다 — 분류를 고치려면 **`meaning` 쪽 낱말**을 고쳐라(7차 배치A 적발).")
    a(u"- ⛔`_spec/specs20*.json` 을 **직접 쓰지 마라**(읽기 전용). 반영은 메인이 `applypatch.py` 로 한다.")
    a(u"")
    a(u"생성 보조: `python -X utf8 mkpatch.py --help` (참조구현 — 손으로 JSON 을 쓰다 오타를 내지 마라)")
    a(u"")
    a(u"### §5-b 산문 보고서에 **반드시** 적을 것")
    a(u"")
    a(u"1. ★**판정 어휘를 골라 써라** — 「불가」 한 단어로 뭉치면 다음 라운드가 시도조차 안 한다:")
    a(u"   - `미탐색` = 아직 안 해봤다")
    a(u"   - `재료 부재` = 해봤는데 재료가 없다 → **어떤 재료를 어떻게 시도했는지 범위를 열거**하라")
    a(u"   - `표기 불가` = **동작은 확정됐는데** 명세 칸에 담을 형식이 없다")
    a(u"   - `사실 서술` = 물음이 아니다 → `notes[]` 로 보내라(`open` 에 두지 마라)")
    a(u"2. **무엇을 실제로 실행했는지** — 명령줄 그대로. 「확인했다」만 쓰면 재현이 안 된다.")
    a(u"3. **내 지시(이 도시에)의 오류**를 찾으면 그것부터 보고하라. 6차까지 매 라운드 나왔다.")
    a(u"4. **판정 반전도 오류로 센다**(유저 확정). 「전에 A 라 했는데 B 였다」면 그건 오류 1건이다.")
    a(u"")

    # §6 출력
    outdir = u"_verify%s/%s" % (rnd, tag)
    a(u"## §6 출력 디렉터리 — **%s 만 써라**" % outdir)
    a(u"")
    a(u"```")
    a(u"MIG/%s/patch.json      ← 기계 적용분(필수)" % outdir)
    a(u"MIG/%s/REPORT.md       ← 산문 보고서" % outdir)
    a(u"MIG/%s/oracle/         ← 오라클 .rs · 출력 로그" % outdir)
    a(u"```")
    a(u"")
    a(u"⚠**다른 배치 폴더에 쓰지 마라.** 네 배치가 동시에 돌아 경합한다.")
    a(u"⚠`_spec/` 아래 어떤 파일도 쓰지 마라(읽기 전용).")
    a(u"⚠`distruct.json`·`dienum.json` 은 **절대 수정 금지**.")
    a(u"")

def build(rnd, tag, lo, hi, gate_txt, split=True):
    u"""`split=True` 면 §3(명세 전문)을 **함수별 파일로 뺀다**.

    ★왜 — 7차 첫 시도에서 **배치 B·C 가 600초 무진전으로 죽었다.** 마지막 흔적이
    「도시에를 통째로 읽겠다」였다. 110~152KB 를 한 번에 읽으면 스트림이 멎는다.
    ⟹ **무손실은 유지하되 파일을 나눈다.** 「통째로」는 *정보가 잘리지 않는다*는 뜻이지
    *한 파일이어야 한다*는 뜻이 아니다 — 이걸 혼동해서 읽지도 못할 파일을 만들었다."""
    L = []
    a = L.append
    a(u"<!-- ★`mkdossier.py` 가 생성한다. 손으로 고치지 마라 — 다음 생성에 날아가고,"
      u" 손으로 쓴 수치가 5차까지 반복된 지시 오류의 원인이었다. -->")
    a(u"# %s차 배치 %s 도시에 — 담당 `%02d`~`%02d` (게임 %s)" % (rnd, tag, lo, hi, M.get("game")))
    a(u"")

    # §0 스탬프
    a(u"## §0 정본 스탬프 — **읽기 전에 스스로 확인하라**")
    a(u"")
    a(u"| 정본 | sha256[:16] | mtime |")
    a(u"|---|---|---|")
    a(u"| `_spec/specs20_v3.json` | `%s` | %s |"
      % (sha16(V3), time.strftime("%Y-%m-%d %H:%M:%S", time.localtime(os.path.getmtime(V3)))))
    a(u"| `_spec/specs20.json` (손이 닿는 정본) | `%s` | %s |"
      % (sha16(V2), time.strftime("%Y-%m-%d %H:%M:%S", time.localtime(os.path.getmtime(V2)))))
    a(u"")
    a(u"| `mkdossier.py`(이 파일을 만든 도구) | `%s` | %s |"
      % (sha16(__file__), time.strftime("%Y-%m-%d %H:%M:%S",
                                        time.localtime(os.path.getmtime(__file__)))))
    a(u"")
    a(u"이 파일 생성 = `%s`" % time.strftime("%Y-%m-%d %H:%M:%S"))
    a(u"")
    a(u"### ★★신선도 확인 — **정본만이 아니라 「이 지시문 자신」도 확인하라**")
    a(u"")
    a(u"7차에 배치 C 가 이걸 적발했다: §0 이 `_spec` 해시만 확인시키는데 **지시문 본문은 라운드 중에 바뀐다**")
    a(u"(다른 배치가 내 지시 오류를 보고하면 내가 그 자리에서 고치기 때문). C 가 받은 판은 17:41,")
    a(u"디스크 현재는 17:58 이었고 **`_spec` 해시는 일치하는데 지시문 4곳이 달랐다** — ")
    a(u"그중 하나가 `patch.json` 스키마라 **옛 판대로 냈으면 0건 적용**이었다.")
    a(u"")
    a(u"```bash")
    a(u"cd /c/tfm2mods/MIG")
    a(u"python -X utf8 dossierfresh.py %s %s      # ← 이것만 돌리면 된다" % (rnd, tag))
    a(u"```")
    a(u"`STALE` 이 뜨면 **작업을 멈추고 이 파일과 `spec_*.md` 를 다시 읽어라.**")
    a(u"네 patch 를 낼 때 한 번 더 돌려라 — 그 사이에 또 바뀌었을 수 있다.")
    a(u"")
    a(u"> ★**v3 를 직접 고치지 마라.** 정본은 `specs20.json`(v2)이고 v3 는 `mkspec3.py` 의 생성물이다.")
    a(u"> 네가 할 일은 파일 수정이 아니라 **`patch.json` 제출**이다(§5).")
    a(u"")

    # §1 담당·표적
    a(u"## §1 담당 함수와 이번 라운드의 표적")
    a(u"")
    a(u"| # | 이름 | vis | mem | consts | knobs | open | ev≥4(미실행) |")
    a(u"|---|---|---|---|---|---|---|---|")
    for i in range(lo, hi + 1):
        sp = S[i]
        n4 = sum(1 for f in EV_FIELDS for x in sp[f] if (x.get("ev") or 9) >= 4)
        a(u"| `%02d` | %s | %s | %d | %d | %d | %d | %d |"
          % (i, sp["name"], (sp.get("sig") or {}).get("vis") or u"?",
             len(sp["mem"]), len(sp["consts"]), len(sp["knobs"]), len(sp.get("open") or []), n4))
    a(u"")
    a(u"★**`vis` 가 `pub` 이 아니어도 오라클 직접 진입은 된다**(22차 C·D 실증 · METHOD_MAP ⑥): `define hidden` 심볼은")
    a(u"`extern \"Rust\" { #[link_name=\"<망글 심볼>\"] fn f(..); }` 로 부른다(조건 = game_ai `pub` 항목 1개 이상 참조 · LNK2019).")
    a(u"막히는 것은 **`internal fastcc`** 뿐(심볼 없음) — 그때만 상위 `pub` 래퍼·형제 복제본을 노려라. ~~5차 「pub 아니면 막힘」~~ 은 stale.")
    a(u"")
    a(u"★**`ev≥4`** = IR 독해(4)·추론(5)뿐이라 **실행으로 확인되지 않은 행**이다. 오라클로 내려라.")
    a(u"⚠단 이 수는 **실행 대상 수가 아니다** — `knobs` 상당수가 인라인된 **다른 함수의 줄**을 가리켜")
    a(u"네 함수에 진입해도 안 닿는다(7차 배치A 적발). 숫자를 목표로 삼지 말고 **닿는 것부터** 내려라.")
    a(u"`ev` 는 손으로 매기는 값이 아니라 **근거 문면에서 파생**된다(`mkspec3.evtier`) — ")
    a(u"숫자를 고치려 하지 말고 **근거를 바꿔라**(예: 「오라클 실행 확인: …」을 `note` 에 쓰면 2로 내려간다).")
    a(u"")
    a(u"### ★★게이트가 **전부 0** 이어도 그게 「명세가 맞다」는 뜻이 아니다")
    a(u"")
    a(u"게이트 %d개는 **각자 한 축만** 본다. 아래는 **어떤 게이트도 보지 않는 칸**이다:" % NGATE)
    a(u"")
    a(u"| 무검사 칸 | 왜 위험한가 |")
    a(u"|---|---|")
    a(u"| `one_line` · `layer` | 함수를 한 줄로 요약한 것. 틀리면 **읽는 사람이 통째로 오해**한다 |")
    a(u"| `callees[]` 의 내용 | tcx 에서 자동 생성되지만 **경로·시그니처가 맞는지는 무검사** |")
    a(u"| `closed[]` | 「이미 닫혔다」고 적힌 것들. **닫은 근거가 지금도 유효한지** 아무도 안 본다 |")
    a(u"| `notes[]` | 「확정된 사실 서술」. 그 확정이 맞는지 무검사 |")
    a(u"| `siblings` · `callers` | 자동 열거물. 빠진 게 있어도 모른다 |")
    a(u"| **아예 빠진 것** | ★9차에 `09 knobs[7]`(미니언 위험 임계표)이 **명세 어디에도 없었다**. 게이트는 *있는 칸이 틀렸나*만 본다 |")
    a(u"")
    a(u"⟹ **게이트 0 = 「검사한 축이 깨끗하다」** 이지 **「명세가 완전하다」가 아니다.**")
    a(u"")

    # §2 정본 문서
    a(u"## §2 읽어야 할 정본 — **요약본이 아니라 원문을 읽어라**")
    a(u"")
    a(u"여기 요약은 없다. 아래 파일을 **직접 Read** 하라. 이 표는 「무엇이 어디 있고 지금 내용이 무엇인지」만 준다.")
    a(u"")
    a(u"| 파일 | 줄 | sha256[:16] | 왜 |")
    a(u"|---|---|---|---|")
    for p, why in CANON:
        a(u"| `%s` | %d | `%s` | %s |"
          % (p.replace(HERE + os.sep, u"MIG\\"), nlines(p), sha16(p), why))
    a(u"")

    # §3 명세 전문
    a(u"## §3 담당 함수 명세 **전문** (무손실 — 잘린 칸이 없다)")
    a(u"")
    a(u"> 표의 어떤 칸도 `...` 로 줄이지 않았다. 길어 보이는 게 정상이고, "
      u"**줄인 자리가 곧 지시 오류가 난 자리**였다.")
    a(u"")
    side = []
    if split:
        a(u"★**함수마다 파일이 따로 있다. 한 번에 하나씩 Read 하라.**")
        a(u"(7차 첫 시도에서 도시에를 통째로 읽으려던 배치 둘이 600초 무진전으로 죽었다. "
          u"내용은 그대로고 파일만 나눴다.)")
        a(u"")
        a(u"| # | 이름 | 파일 |")
        a(u"|---|---|---|")
        for i in range(lo, hi + 1):
            fn = u"spec_%02d_%s.md" % (i, S[i]["name"].replace(u"::", u"__"))   # 09-13: `TeamPlan::update` 류 이름의 `::` 는 Windows 파일명 불가(OSError 22)
            a(u"| `%02d` | %s | `MIG\\_verify%s\\%s\\%s` |" % (i, S[i]["name"], rnd, tag, fn))
            side.append((fn, u"\n".join(render_fn(S[i])) + u"\n"))
        a(u"")
    else:
        for i in range(lo, hi + 1):
            L += render_fn(S[i])

    # §4 게이트
    a(u"## §4 게이트 미해소 — **이 배치 몫만 추렸다**")
    a(u"")
    a(u"★**이번 라운드의 주 표적이다.** 6차 통제실험 결론 = 「오류는 **검사받지 않는 축**에 고인다」 — ")
    a(u"`consts.src_line` 은 5라운드 동안 아무도 안 봤고 G12 를 붙이자 **즉시 56건**이 나왔다.")
    a(u"그 오류들은 **1차부터 그대로 있었다.** 아래는 아직 안 닫힌 것들이다.")
    a(u"")
    gs = gate_split(gate_txt, lo, hi)
    if gs:
        # ★건수는 **이 배치 몫만** 센다(7차 배치A 적발 — 게이트 머리줄의 전역 합계를 그대로
        #   실어서 「8/7 인데 47/8 로 적혀 있다」는 오독을 불렀다).
        # ★★**적발 줄과 「상세」 줄을 같이 세지 마라.** (12차 생성 직후 자기적발)
        #   구판은 `  [` + 두 자리 숫자면 다 셌는데, G20 의 상세 줄이
        #   `  [03]`…`` 형태라 **배치A 6건이 10건으로** 찍혔다.
        #   ⟹ 적발 줄만이 `]` 뒤에 **공백 + 함수 이름**을 갖는다. 그걸로 가른다.
        n = sum(1 for x in gs
                if re.match(r"  \[\d+\] ", x))   # 09-14: 세 자리 idx
        a(u"**이 배치 몫 = %d건** (게이트 머리줄의 건수는 **20함수 전역 합계**다 — 혼동 말 것)"
          % n)
        a(u"")
        a(u"⚠★**게이트의 「실제 후보」를 그대로 믿지 마라.** 7차 배치A 가 G12 8건을 전수 검증했더니 "
          u"**전부 오탐**이었다 — `srclinecheck.py` 가 ①`switch` case 라벨 줄에 `!dbg` 가 없어 못 보고 "
          u"②`phi` 상수를 `line 0` 으로 버리고 ③리터럴 정규식이 `%2`·`%16` 같은 SSA 레지스터와 gep "
          u"오프셋까지 잡는다. **지적된 줄의 IR 원문을 직접 열어 확인**하고, 명세가 옳으면 "
          u"`kind: \"오탐\"` 으로 보고하라(고치는 게 아니라 게이트를 고쳐야 한다).")
        a(u"")
        a(u"```")
        L += gs
        a(u"```")
    else:
        a(u"(이 배치 몫 미해소 = **0건**. 그래도 §4-b 의 무검사 축은 남아 있다.)")
    a(u"")
    a(u"### §4-b 아직 **게이트가 없는 축** — 여기를 의심하라")
    a(u"")
    a(u"| 축 | 전체 건수 | 상태 |")
    a(u"|---|---|---|")
    tot_dir = sum(1 for s in S for x in s["mem"] if x.get("dir"))
    tot_kind = sum(1 for s in S for x in s["consts"] if x.get("kind"))
    tot_role = sum(1 for s in S for x in ((s.get("sig") or {}).get("params") or []) if x.get("role"))
    # ★★이 표는 **라운드마다 갱신해야 한다.** 축이 닫히면 「무검사」 표기가 거짓이 된다.
    #   (11차 후속 갱신 — 이 셋은 이제 G14·G15·G16 이 본다.)
    tot_cal = sum(1 for s in S for x in (s.get("callees") or []))
    n_anch = sum(1 for s in S for x in (s.get("callees") or []) if x.get("ev") == 3)
    a(u"| `mem.dir`(읽기/쓰기 방향) | %d | ✅**G14** — IR load/store 대조 |" % tot_dir)
    a(u"| `consts.kind`(상수 종류) | %d | ✅**G15** — IR 소비 오프코드 반증식 |" % tot_kind)
    a(u"| `sig.params.role`(인자 역할) | %d | ✅**G16** — `define` 속성 대조 |" % tot_role)
    a(u"| `callees[]` 의 **경로·시그니처** | %d | ⚠**%d행만 IR 앵커**(`ev3`). 나머지 %d행은 "
      u"`ev4` + 「미확정」 — 같은 leaf 이름 후보가 여럿이라 **하나를 고른 것이 아니라 못 고른 것**이다 |"
      % (tot_cal, n_anch, tot_cal - n_anch))
    a(u"| 명세 **사이**의 같은 사실 | — | ✅**G20**(11차 후속 신설) — 유일한 cross-spec 게이트 |")
    a(u"")
    a(u"⚠`consts.kind` 는 **v2 에 없는 파생 필드**다(`mkspec3.py` 가 `meaning` 의 낱말 + IR 관측으로 정한다) — ")
    a(u"`errors[]` 로 직접 못 쓰니 **`meaning` 쪽 낱말**을 고쳐야 한다(7차 배치A 가 3건 거부당했다).")
    a(u"어휘 = `임계`·`태그`·`센티널`·`인덱스`·`계수`·`산출값`·`오프셋가감`·`길이`(뒤 넷은 확장).")
    a(u"")
    a(u"★**`callees` 의 `ev4` 행을 「틀렸다」로 읽지 마라.** 그건 **판정 보류**(재료 부재)다 — ")
    a(u"11차 후속 이전에는 이 행들이 전부 `ev3`(tcx 정본) 도장을 받고 있었고 그게 **거짓 확증**이었다.")
    a(u"담당 함수의 `ev4` 행 중 **판정에 실제로 쓰이는 술어**가 있으면 IR 호출 심볼로 확정해 달라.")
    a(u"")
    a(u"⟹ 담당 함수에서 **표본을 떠서 직접 대조하라.** 틀린 게 나오면 그건 「그 축 전체가 무검사였다」는 뜻이고,")
    a(u"보고에 **검사기를 어떻게 만들면 되는지**까지 적어라(그게 다음 라운드의 게이트가 된다).")
    a(u"")
    a(u"★**불일치 0 도 결과다.** 7차 배치A 가 `mem.dir` 89행을 대조해 **불일치 0** 을 냈다 — ")
    a(u"「계측기를 붙이면 오류가 나온다」가 법칙이 아니라는 반대 사례이고, 그 축은 닫아도 된다는 뜻이다.")
    a(u"")

    sect5_6(a, rnd, tag, lo)
    return u"\n".join(L) + u"\n", side


def axis_rows(field, key):
    u"""그 축의 **전 20함수 행**을 모은다. 축 모드의 §3 본체."""
    out = []
    for s in S:
        if field == "sig.params":
            arr = ((s.get("sig") or {}).get("params") or [])
        else:
            arr = s.get(field) or []
        for j, x in enumerate(arr):
            out.append((s["i"], s["name"], j, x))
    return out


def build_axis(rnd, tag, slug, title, field, key, note, gate_txt):
    L = []
    a = L.append
    a(u"<!-- ★`mkdossier.py --axis` 가 생성한다. 손으로 고치지 마라. -->")
    a(u"# %s차 배치 %s 도시에 — **축 `%s`** (게임 %s)" % (rnd, tag, title, M.get("game")))
    a(u"")
    a(u"## §0 정본 스탬프 — **읽기 전에, 그리고 patch 를 내기 전에 한 번 더**")
    a(u"")
    a(u"| 정본 | sha256[:16] |")
    a(u"|---|---|")
    a(u"| `_spec/specs20_v3.json` | `%s` |" % sha16(V3))
    a(u"| `_spec/specs20.json` | `%s` |" % sha16(V2))
    a(u"| `mkdossier.py` | `%s` |" % sha16(__file__))
    a(u"")
    a(u"```bash")
    a(u"cd /c/tfm2mods/MIG && python -X utf8 dossierfresh.py %s %s" % (rnd, tag))
    a(u"```")
    a(u"`STALE` 이면 **작업을 멈추고 이 파일을 다시 읽어라.** 7차에 배치 C 가 옛 판을 들고 있었고,")
    a(u"그 판의 `patch.json` 스키마가 틀려서 **그대로 냈으면 0건 적용**이었다.")
    a(u"")

    # §1 임무
    a(u"## §1 네 임무 — **함수가 아니라 축 하나를 끝까지 닫는다**")
    a(u"")
    a(u"> 7차 실측: 실오류 42건의 **88%가 「그 라운드에 처음 검사한 축」**에서 나왔고,")
    a(u"> **6차까지 검사받던 축에서 새로 나온 오류는 0건**이었다.")
    a(u"> ⟹ 남은 일은 함수를 또 도는 게 아니라 **축을 세우는 것**이다. 그래서 이번엔 축으로 나눈다.")
    a(u"")
    a(u"**담당 축 = %s** — %s" % (title, note))
    a(u"")
    rows = axis_rows(field, key)
    a(u"이 축의 전 20함수 행 = **%d개**. (§3 에 전량 있다.)" % len(rows))
    a(u"")
    a(u"### 할 일 (순서대로)")
    a(u"")
    if slug == "history_prop":
        a(u"1. `history[]`(정정 이력)의 결론이 `mem`/`consts`/`knobs`/`logic` **표에 실제로 반영됐는지** 전수 대조.")
        a(u"2. 안 실린 것을 `errors[]` 로 낸다. **분석이 아니라 전파**다 — 답은 이미 `history` 에 있다.")
        a(u"3. 그 대조를 **기계로** 만들어라 → `G17` 제안. `G8` 은 `logic`↔표만 보니 그 짝이다.")
        a(u"4. 여력이 남으면 **G9 `callees` 오염 4건**(현재 미해소)도 닫아라. "
          u"7차 배치B 제안 = 「동명 함수가 **다른 Self 타입**이면 후보에서 빼라」.")
    else:
        a(u"1. `MIG\\_gates\\%s\\` 에 **다른 배치들이 만든 검사기 후보가 모여 있다.** "
          u"전부 돌려 보고 **왜 결과가 갈리는지** 규명하라." % slug)
        a(u"2. 하나로 **통합**해서 `MIG\\_verify%s\\%s\\gate.py` 로 낸다. "
          u"통합판은 다른 후보들의 장점을 흡수해야 한다." % (rnd, tag))
        a(u"3. ★★**통합판의 적발을 네가 직접 반증하라.** 표본을 떠서 **IR 원문으로** 확인하고, "
          u"오탐이면 게이트를 고쳐라. **적발 건수는 게이트의 성능이 아니라 가설이다** — ")
        a(u"   7차에 `G12` 가 47건을 적발했는데 검증된 것은 **대부분 오탐**이었다.")
        a(u"4. 남은 **진짜 오류**만 `errors[]` 로 낸다.")
    a(u"")
    a(u"⚠★**7차에 내가 이 축을 고치다 실패한 방식**(같은 함정을 밟지 마라):")
    a(u"`G12` 의 오탐을 고치려고 **귀속 추론**(switch arm·phi 인입)을 후보에 더했더니 "
      u"**16 → 22 로 늘었다.** 판정식이 「후보가 있는데 주장이 없으면 불일치」라서 "
      u"**후보를 늘리는 것이 곧 오탐을 늘린다.**")
    a(u"⟹ **귀속·추론은 「구제」에만 쓰고 「기각」에는 쓰지 마라.** "
      u"추론으로 얻은 근거로 남의 주장을 뒤집으면 그건 게이트가 아니라 또 하나의 추측이다.")
    a(u"")

    # §2 정본
    a(u"## §2 읽어야 할 정본 — **요약본이 아니라 원문을 읽어라**")
    a(u"")
    a(u"| 파일 | 줄 | sha256[:16] | 왜 |")
    a(u"|---|---|---|---|")
    for p, why in CANON:
        a(u"| `%s` | %d | `%s` | %s |"
          % (p.replace(HERE + os.sep, u"MIG\\"), nlines(p), sha16(p), why))
    a(u"")
    if slug != "history_prop":
        a(u"그리고 **`MIG\\_gates\\%s\\` 의 후보 전부**(각 파일 첫 docstring 에 설계 의도와 "
          u"그 배치가 본 실측이 적혀 있다)." % slug)
        a(u"")

    # §3 축 전량 — ★별도 파일로 뺀다(7차에 112KB 도시에를 통째로 읽던 배치 둘이 죽었다)
    a(u"## §3 담당 축 **전량** (%d행 · 무손실)" % len(rows))
    a(u"")
    a(u"★**본문은 아래 파일들에 따로 있다. 한 번에 하나씩 Read 하라.**")
    a(u"(7차에 도시에를 통째로 읽으려던 배치 둘이 600초 무진전으로 죽었다 — 내용은 그대로고 파일만 나눴다.)")
    a(u"")

    # ★함수 5개씩 4조각. 한 조각이 커지지 않게 나눈다.
    GRP = ((0, 4), (5, 9), (10, 14), (15, 19))
    parts = []
    for glo, ghi in GRP:
        sub = [r for r in rows if glo <= r[0] <= ghi]
        if not sub:
            continue
        fn = u"AXIS_ROWS_%02d-%02d.md" % (glo, ghi)
        d2 = []
        w = d2.append
        w(u"# %s차 배치 %s — 축 `%s` · 함수 `%02d`~`%02d` (%d행)"
          % (rnd, tag, title, glo, ghi, len(sub)))
        w(u"")
        w(u"> `mkdossier.py --axis` 생성물. **칸을 하나도 줄이지 않았다** — "
          u"7차까지 지시 오류는 전부 「줄여 적은 자리」에서 났다.")
        w(u"")
        if slug == "history_prop":
            w(u"| # | 함수 | idx | was | now | why |")
            w(u"|---|---|---|---|---|---|")
            for i, nm, j, x in sub:
                w(u"| `%02d` | %s | %d | %s | %s | %s |"
                  % (i, nm, j, _cell(x.get("was")), _cell(x.get("now")), _cell(x.get("why"))))
        else:
            cols = {"mem": ("base", "offset", "name", "dir", "note", "ev"),
                    "consts": ("value", "src_line", "kind", "meaning", "ev"),
                    "knobs": ("what", "where", "value", "effect", "ev"),
                    "sig.params": ("i", "name", "ty", "role", "ev")}[field]
            w(u"| # | 함수 | idx | " + u" | ".join(cols) + u" |")
            w(u"|---|---|---|" + u"---|" * len(cols))
            for i, nm, j, x in sub:
                w(u"| `%02d` | %s | %d | " % (i, nm, j)
                  + u" | ".join(_cell(x.get(c)) for c in cols) + u" |")
        parts.append((fn, u"\n".join(d2) + u"\n"))

    a(u"| 파일 | 행 | 크기 |")
    a(u"|---|---|---|")
    for fn, body in parts:
        a(u"| `MIG\\_verify%s\\%s\\%s` | %d | %.0fKB |"
          % (rnd, tag, fn, body.count(u"\n| `"), kb(body)))
    a(u"")
    a(u"")

    # §4 현재 게이트
    a(u"## §4 지금 게이트 상태 (참고 — 네 축은 여기 **없다**. 그래서 네가 만든다)")
    a(u"")
    line = [l for l in gate_txt.split(u"\n") if u"G1 자기모순" in l]
    a(u"```")
    a(line[0].strip() if line else u"(측정 실패)")
    a(u"```")
    a(u"위 줄이 **생성 시점의 실측**이다. 손으로 옮겨 적은 수치가 아니라 `specgate.py` 출력 그대로다.")
    a(u"⚠**네 축이 아닌 게이트는 손대지 마라**(다른 배치 몫).")
    a(u"⚠**부분 실행 주의**: `--gate GN` 을 쓰면 나머지는 `-`(안 돌림)로 찍힌다. `0` 과 헷갈리지 마라.")
    a(u"")
    return L, parts


def main():
    rnd = sys.argv[1] if len(sys.argv) > 1 else "7"
    only = None
    if "--batch" in sys.argv:
        only = sys.argv[sys.argv.index("--batch") + 1].upper()
    fresh_or_die()

    missing = [p for p, _ in CANON if not os.path.exists(p)]
    if missing:
        print(u"★정본 문서가 없다 — 도시에를 만들지 않는다:")
        for p in missing:
            print(u"   %s" % p)
        return 1

    print(u"게이트 측정 중…")
    gate_txt = run(["specgate.py"])
    if u"G1 자기모순" not in gate_txt:
        print(u"★`specgate.py` 출력이 예상과 다르다 — 도시에를 만들지 않는다.")
        print(gate_txt[:800])
        return 1

    # ── 축 모드 ────────────────────────────────────────────────────────
    if "--axis" in sys.argv:
        d = os.path.join(HERE, "_verify%s" % rnd)
        made2 = []
        # 라운드마다 축이 다르다 — 닫힌 축을 또 돌리지 않는다.
        axes = {"8": AXIS, "9": AXIS9}.get(str(rnd), AXIS9)
        for tag, slug, title, field, key, note in axes:
            if only and tag != only:
                continue
            sd = os.path.join(d, tag)
            if not os.path.isdir(sd):
                os.makedirs(sd)
            L, rowsdoc = build_axis(rnd, tag, slug, title, field, key, note, gate_txt)
            sect5_6(L.append, rnd, tag, None)
            txt = u"\n".join(L) + u"\n"
            out = os.path.join(d, "DOSSIER_%s.md" % tag)
            if "--check" in sys.argv:
                print(u"  [검증] %s (축 %s) %d줄 %.0fKB + 행표 %d조각"
                      % (out, slug, txt.count(u"\n"), kb(txt), len(rowsdoc)))
                continue
            io.open(out, "w", encoding="utf-8").write(txt)
            for fn, body in rowsdoc:
                io.open(os.path.join(sd, fn), "w", encoding="utf-8").write(body)
            stamp = os.path.join(d, "STAMP.json")
            try:
                st = json.load(io.open(stamp, encoding="utf-8"))
            except Exception:
                st = {}
            st[tag] = {"dossier": sha16(out), "axis": slug,
                       "specs": [fn for fn, _ in rowsdoc],
                       "spec_hashes": dict((fn, sha16(os.path.join(sd, fn)))
                                           for fn, _ in rowsdoc),
                       "mkdossier": sha16(__file__), "v3": sha16(V3),
                       "built": time.strftime("%Y-%m-%d %H:%M:%S")}
            io.open(stamp, "w", encoding="utf-8").write(
                json.dumps(st, ensure_ascii=False, indent=1))
            made2.append((out, txt, slug, rowsdoc))
        for out, txt, slug, rowsdoc in made2:
            print(u"%s  [축 %s]  (%d줄 · %.0fKB)" % (out, slug, txt.count(u"\n"), kb(txt)))
            for fn, body in rowsdoc:
                print(u"    %s  (%.0fKB)" % (fn, kb(body)))
            for lbl, s in [(u"본문", kb(txt))] + [(f, kb(b)) for f, b in rowsdoc]:
                if s > 60:
                    print(u"  ⚠%s **%.0fKB** — 한 번에 읽기엔 크다" % (lbl, s))
        return 0

    made = []
    for tag, lo, hi in BATCH:
        if only and tag != only:
            continue
        txt, side = build(rnd, tag, lo, hi, gate_txt, split=("--onefile" not in sys.argv))
        d = os.path.join(HERE, "_verify%s" % rnd)
        sd = os.path.join(d, tag)
        if not os.path.isdir(sd):
            os.makedirs(sd)
        out = os.path.join(d, "DOSSIER_%s.md" % tag)
        if "--check" in sys.argv:
            print(u"  [검증] %s  %d줄 %.0fKB · 분책 %d개"
                  % (out, txt.count(u"\n"), kb(txt), len(side)))
            continue
        io.open(out, "w", encoding="utf-8").write(txt)
        for fn, body in side:
            io.open(os.path.join(sd, fn), "w", encoding="utf-8").write(body)
        made.append((out, txt.count(u"\n"), kb(txt), side))

        # ★생성물 자신의 해시를 남긴다 — `dossierfresh.py` 가 이걸로 「네가 읽은 판이 최신인가」를 답한다.
        #   (7차 배치C 적발: 정본 해시만 확인시키면 **지시문 자신의 변경**을 아무도 못 본다)
        stamp = os.path.join(d, "STAMP.json")
        try:
            st = json.load(io.open(stamp, encoding="utf-8"))
        except Exception:
            st = {}
        st[tag] = {"dossier": sha16(out),
                   "specs": [os.path.basename(f) for f, _ in side],
                   "spec_hashes": dict((os.path.basename(f), sha16(os.path.join(sd, f)))
                                       for f, _ in side),
                   "mkdossier": sha16(__file__),
                   "v3": sha16(V3),
                   "built": time.strftime("%Y-%m-%d %H:%M:%S")}
        io.open(stamp, "w", encoding="utf-8").write(
            json.dumps(st, ensure_ascii=False, indent=1))

    for out, ln, sz, side in made:
        print(u"%s  (%d줄 · %.0fKB)" % (out, ln, sz))
        for fn, body in side:
            print(u"    %s  (%d줄 · %.0fKB)" % (fn, body.count(u"\n"), kb(body)))
    # ★한 파일이 60KB 를 넘으면 경고한다 — 7차에 배치 둘이 그걸 읽다 죽었다.
    big = [(o, s) for o, _, s, _ in made if s > 60] + \
          [(f, kb(b)) for _, _, _, sd2 in made for f, b in sd2 if kb(b) > 60]
    for f, s in big:
        print(u"⚠**%.0fKB** — 한 번에 읽기엔 크다: %s" % (s, f))
    return 0


if __name__ == "__main__":
    sys.exit(main())
