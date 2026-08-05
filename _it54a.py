# -*- coding: utf-8 -*-
"""앵커 생성 + 콜그래프 투표 전파."""
import sys, os, pickle
sys.path.insert(0, r'C:\tfm2mods')
from _it54g import GO, GN, fn_of
from _it54 import O, N, BASE

AC = r'C:\tfm2mods\_a54_cache.pkl'

def anchors():
    so, sn = {}, {}
    for f, ss in GO['fn_str'].items(): so.setdefault(frozenset(ss), []).append(f)
    for f, ss in GN['fn_str'].items(): sn.setdefault(frozenset(ss), []).append(f)
    m = {}
    for k, vo in so.items():
        vn = sn.get(k)
        if vn and len(vo) == 1 and len(vn) == 1:
            m[vo[0]] = vn[0]
    return m

def propagate(M, rounds=6):
    """콜그래프 양방향 투표: 미매칭 함수를, 이미 매칭된 콜러/콜리 대응으로 추정."""
    for r in range(rounds):
        add = {}
        rev = {v: k for k, v in M.items()}
        # callee 방향: o의 caller가 매칭돼 있으면 그 caller의 n버전의 callee들 중에서 투표
        votes = {}
        for co, cn in M.items():
            lo = GO['callees'].get(co, [])
            ln = GN['callees'].get(cn, [])
            # 순서 기반 대응은 위험 → 다중집합 위치 매칭: 동일 인덱스
            if len(lo) == len(ln):
                for a, b in zip(lo, ln):
                    if a in M or b in rev: continue
                    votes.setdefault(a, {}).setdefault(b, 0)
                    votes[a][b] += 1
        # caller 방향
        for co, cn in M.items():
            lo = GO['callers'].get(co, [])
            ln = GN['callers'].get(cn, [])
            if len(lo) == len(ln):
                for (a, _), (b, _) in zip(sorted(lo), sorted(ln)):
                    if a in M or b in rev: continue
                    votes.setdefault(a, {}).setdefault(b, 0)
                    votes[a][b] += 1
        for a, d in votes.items():
            best = sorted(d.items(), key=lambda x: -x[1])
            if len(best) == 1 or best[0][1] > best[1][1]:
                add[a] = best[0][0]
        # 충돌 제거(1:1 강제)
        cnt = {}
        for v in add.values(): cnt[v] = cnt.get(v, 0) + 1
        n0 = len(M)
        for a, b in add.items():
            if cnt[b] == 1 and b not in rev:
                M[a] = b
        print(f'  round{r}: +{len(M)-n0} -> {len(M)}')
        if len(M) == n0: break
    return M

if os.path.exists(AC):
    M = pickle.load(open(AC, 'rb'))
else:
    M = anchors()
    print('string anchors', len(M))
    M = propagate(M)
    pickle.dump(M, open(AC, 'wb'))

if __name__ == '__main__':
    print('total matched', len(M))
    for t in (0xd0c680, 0x10587e0, 0xeb8810, 0x12b9ab0, 0x960df0, 0x1ab52f0, 0x28f7df0, 0x28e3b10, 0x1bfc80, 0x2e1550, 0x1a6530, 0xebfe50, 0xa5c1e0):
        print(hex(t), '->', hex(M[t]) if t in M else 'MISS')
