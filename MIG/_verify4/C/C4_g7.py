# -*- coding: utf-8 -*-
u"""G7 제안 — `open[].q` 와 같은 명세 `history[].was` 가 같은 질문인가.
history_note 는 "여기 있는 사실이 mem/consts/knobs/callees 에 반영됐는지는 specgate 가 검사한다"고
적었지만, **`open[]` 이 history 로 이미 닫힌 질문을 물고 있는지는 아무도 검사하지 않는다.**
4차 배치C 실측: 자기 배치 open 20건 중 9건이 이 부류였다."""
import io, json, os, re, sys
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
HERE = os.path.dirname(os.path.abspath(__file__))
MIG = os.path.abspath(os.path.join(HERE, "..", ".."))
D = json.load(io.open(os.path.join(MIG, "_spec", "specs20_v3.json"), encoding="utf-8"))

IDENT = re.compile(r"[A-Za-z_][A-Za-z0-9_]{3,45}|[a-z_]+\.rs:\d+|\+0x[0-9a-f]{2,4}")
STOP = set(u"""this that with from have been will 확인 미확인 내부 확정 불가 없다 본문 명세 함수 
없음 경우 부분 이것 라는 에서 으로 이라 있는 것은 안 봄 담당 범위""".split())


def toks(s):
    return set(t for t in IDENT.findall(s or u"") if t.lower() not in STOP and len(t) > 3)


tot = 0
for i, sp in enumerate(D["specs"]):
    hs = sp.get("history") or []
    for j, o in enumerate(sp.get("open") or []):
        a = toks(o["q"])
        if not a:
            continue
        best, bi = 0.0, None
        for k, h in enumerate(hs):
            b = toks(h.get("was"))
            if not b:
                continue
            jac = len(a & b) / float(len(a | b))
            ov = len(a & b) / float(min(len(a), len(b)))
            sc = max(jac, ov * 0.9)
            if sc > best:
                best, bi = sc, k
        if best >= 0.45:
            tot += 1
            print(u"[G7] /specs[%d]/open[%d]  ~ history[%d]  유사도 %.2f" % (i, j, bi, best))
            print(u"      open : %s" % o["q"][:110])
            print(u"      hist : %s" % hs[bi]["was"][:110])
print(u"\n총 %d건" % tot)
