# -*- coding: utf-8 -*-
"""전 crate Location 지문(full071·full072.pkl) + s1.pkl(game-ai 짝) 생성. 0.7.1 회차의 s1.py·full*.pkl 을 한 번에 만든다."""
import os
import pickle
import sys

sys.path.insert(0, r'F:\claude\tfm2mods\MIG')
import aidiff as A

HERE = os.path.dirname(os.path.abspath(__file__))
OLD = r'F:\claude\tfm2\tfm2_0.7.1\TeamfightManager2.exe'
NEW = r'F:\claude\tfm2\tfm2_0.7.2\TeamfightManager2.exe'
for path, out in ((OLD, 'full071.pkl'), (NEW, 'full072.pkl')):
    p = os.path.join(HERE, out)
    if os.path.exists(p):
        continue
    img = A.Img(path)
    loc = A.find_locations(img)
    fn = A.census(img, loc, '', verbose=True)
    pickle.dump(fn, open(p, 'wb'))
    print(out, len(fn), flush=True)
io_, fo = A.load(OLD, 'game-ai')
inn, fn = A.load(NEW, 'game-ai')
pairs, gone, added = A.pair(fo, fn)
rows = []
for ko, (kn, d, n) in pairs.items():
    v, extra = A.verdict(fo[ko], fn[kn])
    rows.append((v, ko, kn, extra))
pickle.dump(dict(pairs=pairs, gone=gone, added=added, rows=rows), open(os.path.join(HERE, 's1.pkl'), 'wb'))
print('pairs', len(pairs), 'gone', len(gone), 'added', len(added), flush=True)
