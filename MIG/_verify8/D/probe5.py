# -*- coding: utf-8 -*-
u"""probe5 — history 결론의 '코드 페이로드 토큰'이 표/logic 어디에도 없는가.

신호 설계 근거
  history.now 는 자유 서술이라 문장 단위 대조가 불가능하다. 그러나 결론이 **코드에 관한 것**일 때는
  거의 예외 없이 백틱 또는 **굵게** 로 코드 토큰(식별자 · `+0xNN` · `foo.rs:NNN` · 수치)을 박는다.
  ⟹ 그 토큰 집합을 페이로드로 삼고, 그것이 표(mem/consts/knobs)·logic 어디에도 없으면
     「결론이 표로 내려오지 않았다」로 본다.
"""
import io, json, os, re, sys
from collections import Counter
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
HERE = os.path.dirname(os.path.abspath(__file__))
MIG = os.path.abspath(os.path.join(HERE, "..", ".."))
D = json.load(io.open(os.path.join(MIG, "_spec", "specs20_v3.json"), encoding="utf-8"))
S = D["specs"]
STRIKE = re.compile(r"~~(?!~).+?~~")

MARK = re.compile(r"(?:\*\*|`)([^`*\n]{2,80})(?:\*\*|`)")
IDENT = re.compile(r"^[A-Za-z_][A-Za-z0-9_]{3,45}$")
OFFS = re.compile(r"^\+?0x[0-9a-fA-F]{1,5}$")
RSLN = re.compile(r"^[a-z_][a-z0-9_]*\.rs:\d{2,5}$")


def payload(blob):
    out = set()
    for m in MARK.findall(blob):
        for t in re.split(r"[\s(),\[\]<>{}=/|+*&;:!?·]+", m.strip()):
            t = t.strip(u"`*.,'\"")
            if IDENT.match(t) or OFFS.match(t) or RSLN.match(t):
                out.add(t.lower().lstrip("+"))
    return out


def tabletext(sp):
    parts = [sp.get("logic") or u""]
    for f in ("mem", "consts", "knobs", "sig", "notes"):
        parts.append(json.dumps(sp.get(f), ensure_ascii=False))
    return STRIKE.sub(u" ", u"\n".join(parts)).lower()


c = Counter()
for i, sp in enumerate(S):
    tt = tabletext(sp)
    for hi, h in enumerate(sp.get("history") or []):
        nowblob = json.dumps({k: v for k, v in h.items() if k != "was"}, ensure_ascii=False)
        p = payload(STRIKE.sub(u" ", nowblob))
        p -= payload(STRIKE.sub(u" ", json.dumps(h.get("was") or u"", ensure_ascii=False)))
        if not p:
            c["nopayload"] += 1
            continue
        miss = sorted(t for t in p if t not in tt)
        c["has"] += 1
        if len(miss) == len(p):
            c["ALLMISS"] += 1
            print(u"[%02d] h[%d] 전량미전파 %d토큰 %s\n      was=%s"
                  % (i, hi, len(p), sorted(p)[:10], (h.get("was") or u"")[:110]))
print(c)
