# -*- coding: utf-8 -*-
u"""`mkspec3._kind` 규칙②(구제)를 `strong` 대신 **전체 관측 `obs`** 로 보면
`kind` 분포가 어떻게 바뀌는지 전수 측정한다.

근거: `kindchk` 는 «`DBGSTR` 은 구제에는 쓰고 기각에는 안 쓴다»고 명시하고
`SUPPORT[길이] = (DBGSTR, CALLARG, CMP_EQ)` 로 선언해 놓았는데, `_kind` 의 구제 검사가
`strong`(=WEAK 제거본)을 보므로 **DBGSTR·CALLARG 는 영원히 구제에 못 쓰인다.**
"""
import io, json, sys
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
sys.path.insert(0, r"C:\tfm2mods\MIG")
import mkspec3 as M
import kindchk as KC

_EVTAIL = M._EVTAIL


def kind_new(sp, x):
    kmat = _EVTAIL.split(x.get("meaning") or u"")[0]
    word = M._word_kind(kmat)
    if word and KC.neg_hit(kmat, word):
        word = None
    ir = sp.get("ir") or {}
    obs = KC.observe(ir.get("file"), ir.get("frm"), ir.get("to"), x.get("value")) or {}
    strong = dict((k, v) for k, v in obs.items() if k not in KC.WEAK)
    if not strong:
        return word or u"미상"
    if word and any(c in obs for c in KC.SUPPORT.get(word, ())):   # ★obs 로 바꾼 곳
        return word
    for c, k in M.KIND_PRI:
        if c in strong:
            return k
    return word or u"미상"


D = json.load(io.open(r"C:\tfm2mods\MIG\_spec\specs20_v3.json", encoding="utf-8"))
S = D["specs"] if isinstance(D, dict) else D
D2 = json.load(io.open(r"C:\tfm2mods\MIG\_spec\specs20.json", encoding="utf-8"))
S2 = D2["specs"] if isinstance(D2, dict) else D2

n = ch = 0
for i, sp in enumerate(S):
    for j, c in enumerate(sp.get("consts") or []):
        n += 1
        old = c.get("kind")
        new = kind_new(S2[i], S2[i]["constants"][j] if S2[i].get("constants") else c)
        if new != old:
            ch += 1
            print(u"[%02d] c%-2d val=%-12s %s -> %s   | %s" %
                  (i, j, c.get("value"), old, new, (c.get("meaning") or u"")[:60]))
print(u"\n총 consts %d행 · 분류가 바뀌는 행 %d" % (n, ch))
