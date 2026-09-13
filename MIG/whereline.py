# -*- coding: utf-8 -*-
u"""whereline — `knobs[].where` 의 **IR 줄번호와 인용 명령**을 실제 `.ll` 과 대조한다. (2026-09-11)

## 왜 만드나 — 「검사받지 않는 축」의 두 번째
6차 배치A 가 찾아낸 규칙: **오류는 검사받지 않는 축에 고인다.**
`consts[].src_line` 이 5라운드 동안 어떤 게이트도 안 보는 축이었고, `G12` 를 붙이자 즉시 56건이 나왔다.

축별 게이트 커버리지를 세어 보니 **982행이 같은 상태**였다:
  `mem.dir` 451 · **`knobs.where` 221** · `consts.kind` 186 · `sig.params.role` 124

이 중 `knobs.where` 는 **가장 강하게 검사할 수 있다** — 133/221 행이 IR 줄번호를 담고,
그중 25행은 **백틱으로 명령까지 인용**한다. 줄이 맞는지뿐 아니라 **인용이 진짜 거기 있는지**까지 본다.

## 판정
- `m04.ll 44033행` / `m04.ll:44033` 꼴을 뽑아 그 줄을 실제로 읽는다.
- 백틱 인용(`` `icmp ugt i64 %30, 4` ``)이 있으면 **그 줄(±2)에 들어 있는지** 본다.
  ±2 를 허용하는 이유 = IR 이 한 논리 연산을 2~3줄로 쪼개기도 하고, 인용이 공백만 다를 수 있다.
- 인용의 SSA 번호(`%30`)는 **재생성 때마다 바뀔 수 있으므로** 대조에서 뺀다(핵심 토큰만 본다).

사용:  python -X utf8 whereline.py          단독 리포트
       (specgate 가 `check_spec` 을 불러 G13 으로 쓴다)
"""
import io, json, os, re, sys

sys.stdout.reconfigure(encoding="utf-8", errors="replace")
HERE = os.path.dirname(os.path.abspath(__file__))
IRDIRS = (r"C:\tfm2mods\_gaibc", r"C:\tfm2mods\_gcbc", r"C:\tfm2mods\_gvbc")
LINEREF = re.compile(r"\b([mg]\d{2}\.ll)[ :]+(\d{3,7})")
QUOTE = re.compile(r"`([^`]{6,80})`")
# IR 명령 오프코드로 시작하는 인용만 대조 대상으로 본다
IROP = re.compile(r"^(icmp|fcmp|call|invoke|getelementptr|load|store|add|sub|mul|shl|lshr|ashr|"
                  r"and|or|xor|select|phi|br|switch|sext|zext|trunc|bitcast|udiv|sdiv|urem|srem|"
                  r"llvm\.|tail call|%\w+ = )")
SSA = re.compile(r"%[\w.]+")
_cache = {}


def irline(f, n):
    u"""`.ll` 의 n 번째 줄(1-base). 없으면 None."""
    if f not in _cache:
        for d in IRDIRS:
            p = os.path.join(d, f)
            if os.path.exists(p):
                _cache[f] = io.open(p, encoding="utf-8", errors="replace").read().split("\n")
                break
        else:
            _cache[f] = None
    src = _cache[f]
    if not src or n < 1 or n > len(src):
        return None
    return src[n - 1]


_TYTOK = re.compile(r"\b(?:i1|i8|i16|i32|i64|i128|ptr|float|double|void|noundef|nonnull|nsw|nuw|samesign|"
                    r"noalias|readonly|readnone|zeroext|signext|dereferenceable\(\d+\)|align\s*\d+|tail|call|invoke|fastcc)\b")


def norm(s):
    u"""SSA 번호·공백을 지운 비교용 형태. `%30` 같은 번호는 재생성 때 바뀐다.
    ★09-13: **타입·속성 토큰도 지운다** — 배치들이 `irann.py` 주석본(`icmp ult %40, %42` · `llvm.umax.i64(%99, 1)`)에서
      인용하므로 원문(`icmp ult i64 %40, %42` · `call noundef i64 @llvm.umax.i64(i64 %99, i64 1)`)과 문면이 다르다.
      15차 r7 G13 4건이 전부 이 오탐(줄은 정확)이었다. `@` 도 지운다(`@llvm.umax` vs `llvm.umax`)."""
    t = _TYTOK.sub(u"", SSA.sub(u"%", s)).replace(u"@", u"")
    return re.sub(r"\s+", u"", t)


def check_spec(sp):
    u"""★`specgate G13` 진입점. `[(knobs 인덱스, 사유, 상세)]` 를 돌려준다."""
    out = []
    for j, k in enumerate(sp.get("knobs") or []):
        w = k.get("where") or u""
        m = LINEREF.search(w)
        if not m:
            continue                      # 줄번호를 안 적은 행은 이 게이트 대상이 아니다
        f, n = m.group(1), int(m.group(2))
        ln = irline(f, n)
        if ln is None:
            out.append((j, u"IR 줄이 존재하지 않는다", u"%s:%d" % (f, n)))
            continue
        q = QUOTE.search(w)
        if not q:
            continue                      # 인용이 없으면 줄 존재 확인까지만
        frag = q.group(1)
        # ★**진짜 IR 명령만 대조한다.** 초판은 백틱 안을 전부 IR 인용으로 보고 17건을 올렸는데
        #   그중 대부분이 오탐이었다 — 백틱은 함수명(`Blackboard::is_recent_visible`)·
        #   범위 인용(`_gaibc/m13.ll:30495~30620`)·자리표시자(`icmp ult .., 30`)에도 쓰인다.
        #   **오탐을 내는 게이트는 무시당한다**(SPEC_RUNBOOK 의 자기 규칙).
        if not IROP.match(frag.strip()):
            continue
        if ".." in frag or "::" in frag or u"…" in frag:   # 자리표시자·Rust 경로·생략부호는 축약 인용이다(09-13: `lshr i64 …, 1`)
            continue
        # ★09-13(17차): `;` 로 여러 명령을 한 백틱에 인용하거나(`add i8 %10,-1; icmp ult i8 %11, 2`)
        #   `%764 = %95+1` 같은 산술 축약을 쓰는 배치가 있다 → 조각별로 대조하고 **한 조각이라도 맞으면 통과**
        #   (축약 조각은 원문에 없으므로 전부 요구하면 정답이 걸린다). 줄번호 자체가 틀린 것(irann 주석본 줄)만 잡는 게 목적.
        near = u"".join(norm(irline(f, x) or u"") for x in range(n - 2, n + 3))
        frags = [t.strip() for t in frag.split(u";") if t.strip()] or [frag]
        wants = [norm(t) for t in frags]
        wants = [w for w in wants if len(w) >= 6]
        if not wants:
            continue
        if not any(w in near for w in wants):
            out.append((j, u"인용한 명령이 그 줄(±2)에 없다",
                        u"%s:%d  인용=%s" % (f, n, q.group(1)[:60])))
    return out


if __name__ == "__main__":
    D = json.load(io.open(os.path.join(HERE, "_spec", "specs20_v3.json"), encoding="utf-8"))
    tot = bad = 0
    for i, sp in enumerate(D["specs"]):
        rows = check_spec(sp)
        n = sum(1 for k in (sp.get("knobs") or []) if LINEREF.search(k.get("where") or u""))
        tot += n
        bad += len(rows)
        if rows:
            print(u"\n=== specs[%d] %s" % (i, sp["name"]))
            for j, why, det in rows:
                print(u"   knobs[%d] %s — %s" % (j, why, det))
    print(u"\n" + u"=" * 80)
    print(u"IR 줄번호를 담은 knobs %d행 검사 · 불일치 %d건" % (tot, bad))
    print(u"=" * 80)
