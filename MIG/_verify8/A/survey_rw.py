# -*- coding: utf-8 -*-
u"""**축 보강 조사** — `dir` 이 한쪽만 적혔는데 IR 은 그 자리를 **양방향으로** 쓰는 행 찾기.

`G14`(=`gate.py`)는 **모순**만 적발한다(r 인데 w 로만 쓰임). 그런데 같은 축에는 다른 실패 모드가 있다:
그 자리를 읽기도 하고 쓰기도 하는데 명세에 **한쪽만** 적힌 경우. 명세가 `r`/`w` 단일값이라
이건 `errors[]` 로 값을 고칠 수 없고(=`표기 불가`), **행이 하나 더 필요하다**.
`applypatch.py` 에 append 가 없으므로 산문으로 보고한다.

⚠**이건 게이트가 아니다** — 후보 나열이고, 각 건은 IR 원문으로 따로 확인해야 한다.
같은 (base, offset) 에 반대 방향 행이 **이미 따로 있으면** 후보가 아니다.

사용: `python -X utf8 survey_rw.py`
"""
import io, json, os, sys

sys.stdout.reconfigure(encoding="utf-8", errors="replace")
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import gate as G

D = json.load(io.open(os.path.join(G.MIG, "_spec", "specs20_v3.json"), encoding="utf-8"))
n = 0
for i, sp in enumerate(D["specs"]):
    ir = sp.get("ir") or {}
    if not ir.get("file"):
        continue
    reads, writes, held_r, held_rw, touch = G.scan(ir["file"], ir["frm"], ir["to"])
    mem = sp.get("mem") or []
    have = set()
    for m in mem:
        mm = G.PUREOFF.match(str(m.get("offset") or ""))
        if mm:
            have.add(((m.get("base") or "").strip(), int(mm.group(1), 16), (m.get("dir") or "").strip()))
    rows = []
    for j, m in enumerate(mem):
        d = (m.get("dir") or "").strip()
        mm = G.PUREOFF.match(str(m.get("offset") or ""))
        if d not in ("r", "w") or not mm:
            continue
        o = int(mm.group(1), 16)
        base = (m.get("base") or "").strip()
        opp = "w" if d == "r" else "r"
        if (base, o, opp) in have:
            continue                       # 반대 방향 행이 이미 따로 있다
        anc = G.base_anchors_for(sp, j, base, reads, writes, held_r, held_rw)
        if not anc:
            continue
        if not any(G.covers(reads if d == "r" else writes, r, o + sh) for (r, sh) in anc):
            continue                       # 주장 방향조차 관측이 없으면 보강 근거가 약하다
        hits = [(r, sh) for (r, sh) in anc
                if G.covers(writes if opp == "w" else reads, r, o + sh)]
        if hits:
            rows.append((j, base, m.get("offset"), m.get("name"), d, opp, hits[0]))
    if rows:
        print(u"\n== specs[%d] %s (%s:%d~%d)" % (i, sp["name"], ir["file"], ir["frm"], ir["to"]))
        for (j, base, off, nm, d, opp, a) in rows:
            n += 1
            print(u"  mem[%-2d] %-32s %-8s dir=%s 인데 %s 도 관측 (루트 %s shift %+d) | %s"
                  % (j, base[:32], off, d, opp, a[0], a[1], (nm or "")[:40]))
print(u"\n합계 후보 %d" % n)
