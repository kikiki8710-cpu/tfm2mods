# -*- coding: utf-8 -*-
u"""`_rank_callees` 앵커 판정의 두 결함을 고쳐 재계산한다.
  ①`s in sym` 부분문자열 → **길이접두 포함**(`12AbstractGame`) 매칭  ⟹ 거짓 앵커 제거
  ②`<impl ...>` 가 섞인 경로 세그먼트(`<impl game_ai`, `TeamPlan>`)를 식별자만 남기고 정리
     ⟹ 실제로 직접 호출되는 항목이 앵커를 잃던 것을 복구
"""
import io, json, re, sys
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
sys.path.insert(0, r"C:\tfm2mods\MIG")
import mkspec3 as M

IDENT = re.compile(r"^[A-Za-z_][A-Za-z0-9_]*$")


def segs_of(path):
    out = []
    for s in (path or u"").replace(u"<impl ", u"::").replace(u">", u"::").split(u"::"):
        s = s.strip()
        if IDENT.match(s):
            out.append(s)
    return out


def anch_new(path, irsy):
    segs = segs_of(path)
    if not segs:
        return False
    return any(all((u"%d%s" % (len(s), s)) in sym for s in segs) for sym in irsy)


D = json.load(io.open(r"C:\tfm2mods\MIG\_spec\specs20_v3.json", encoding="utf-8"))
S = D["specs"] if isinstance(D, dict) else D
D2 = json.load(io.open(r"C:\tfm2mods\MIG\_spec\specs20.json", encoding="utf-8"))
S2 = D2["specs"] if isinstance(D2, dict) else D2

gain = lose = tot = ev3 = 0
for i, sp in enumerate(S):
    irsy = M._ir_callsyms(S2[i])
    for j, c in enumerate(sp.get("callees") or []):
        tot += 1
        old = (c.get("ev") == 3)
        ev3 += old
        new = anch_new(c.get("path"), irsy)
        if new and not old:
            gain += 1
            print(u"+ [%02d] callees[%d] %s" % (i, j, c.get("path")[:95]))
        if old and not new:
            lose += 1
            print(u"- [%02d] callees[%d] %s" % (i, j, c.get("path")[:95]))
print(u"\n총 %d행 · 현행 ev3 %d행 → 수정판 %d행 (신규앵커 +%d / 거짓앵커 -%d)"
      % (tot, ev3, ev3 + gain - lose, gain, lose))
