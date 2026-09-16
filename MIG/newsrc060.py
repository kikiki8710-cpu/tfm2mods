# -*- coding: utf-8 -*-
"""
newsrc060.py — 0.6.0 에만 있는 game_ai 소스 파일(패닉 Location 의 .rs 경로)과 그 파일을 참조하는 0.6.0 함수(RVA·크기·Location 수)를 뽑는다.
새 플랜(Dive/JoinTrait/MorgardTrait/ObjContest …)의 exe 상 위치를 SDK 없이 잡는 첫 단추. 출력 _next\newsrc060.md
"""
import sys, os, io, collections
sys.stdout.reconfigure(encoding="utf-8")
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import mig060 as M
O = M.Exe(M.OLD, "old"); N = M.Exe(M.NEW, "new"); O.build_locidx(); N.build_locidx()
fo = set(f for L in O.fnloc.values() for f, l, c in L); fn = set(f for L in N.fnloc.values() for f, l, c in L)
new_files = sorted(f for f in fn - fo if "game-ai" in f or "game_ai" in f)
gone_files = sorted(f for f in fo - fn if "game-ai" in f or "game_ai" in f)
L = [u"# 0.6.0 신규 game_ai 소스 파일과 참조 함수 (패닉 Location 기준)", u"", u"신규 %d · 사라짐 %d" % (len(new_files), len(gone_files)), u"",
     u"## 사라진 파일", u""] + [u"- `%s`" % f for f in gone_files] + [u"", u"## 신규 파일 → 참조 함수(0.6.0 RVA · 크기 · 그 파일 Location 수 · 줄 범위)", u""]
by_file = collections.defaultdict(list)
for f, LL in N.fnloc.items():
    for path in set(p for p, l, c in LL):
        if path in new_files:
            lines = [l for p, l, c in LL if p == path]
            by_file[path].append((f, N.ends[f] - f, len(lines), min(lines), max(lines)))
for path in new_files:
    L.append(u"### `%s`" % path); L.append(u"")
    for f, sz, n, lo, hi in sorted(by_file[path], key=lambda t: -t[1]):
        L.append(u"- `%x` %6dB · Loc %d · L%d~%d" % (f, sz, n, lo, hi))
    L.append(u"")
io.open(os.path.join(M.HERE, "_next", "newsrc060.md"), "w", encoding="utf-8").write(u"\n".join(L)); print(u"\n".join(L))
