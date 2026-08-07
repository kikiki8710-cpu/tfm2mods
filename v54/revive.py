# -*- coding: utf-8 -*-
"""★revive.py — 죽은 사이트 전수를 align.py 로 자동 판독해 판정표를 만든다.

출력 컬럼: 053rva | 054rva | 새prefix | 새imm_off | w | 새원본값 | 판정 | 근거 | 확신
  · **imm_off/prefix 는 054 실명령의 capstone 인코딩에서 구한다** (053 값 물려쓰기 금지).
  · 확신 상 = L1(레지스터까지 동일) equal 정렬 + 앞뒤 문맥 3명령 일치
    확신 중 = L2(레지스터 가림) equal 정렬 + 문맥 일치, 또는 후보 1개 + 값 일치
    확신 하/미확정 = 후보 여러 개 또는 없음
사용: python revive.py           (판정표)
      python revive.py --tsv     (revive_054.tsv 저장)
"""
import io, os, re, sys, collections

sys.path.insert(0, r'C:\tfm2mods\v54')
import align as A
import reloc as R
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
B = 0x140000000


def ctx_score(P, i, j, n=3):
    """앞뒤 n명령의 L2 토큰 일치 개수 / 2n"""
    ok = 0
    tot = 0
    for d in list(range(-n, 0)) + list(range(1, n + 1)):
        ka, kb = i + d, j + d
        if 0 <= ka < len(P.a) and 0 <= kb < len(P.b):
            tot += 1
            if A.t2(P.a[ka]) == A.t2(P.b[kb]):
                ok += 1
    return ok, tot


def judge(rva, want_off=None, want_w=None, want_val=None):
    P = A.pair_of(rva)
    if not P:
        return dict(rva=rva, ok=False, why='짝함수없음')
    i = P.ia.get(rva)
    if i is None:
        return dict(rva=rva, ok=False, why='053명령경계아님')
    ins = P.a[i]
    o, w, v = A.site_desc(ins)
    if want_off is not None:
        o, w = want_off, want_w
        v = int.from_bytes(ins.bytes[o:o + w], 'little')
    base = dict(rva=rva, ins3=ins, o3=o, w3=w, v3=v,
                fn3=P.f3, fn4=P.f4, ratio=P.ratio, ok=False)

    for nm, ops in (('L1', P.op1), ('L2', P.op2)):
        j, tag, rng = P.map_idx(i, ops)
        if j is None:
            continue
        y = P.b[j]
        if y.mnemonic != ins.mnemonic:
            continue
        oo, ww, vv = A.site_desc(y)
        c, t = ctx_score(P, i, j)
        conf = '상' if (nm == 'L1' and c == t) else ('상' if c >= t - 1 else '중')
        base.update(ok=True, r4=y.address - B, ins4=y, o4=oo, w4=ww, v4=vv,
                    why='%s equal정렬 문맥%d/%d' % (nm, c, t), conf=conf)
        return base

    # replace 블록 — 후보 제시
    lo, hi = P.bracket(i, P.op2)
    cands = [(k, P.b[k]) for k in range(lo, min(hi, len(P.b))) if P.b[k].mnemonic == ins.mnemonic]
    same = [(k, y) for k, y in cands if A.site_desc(y)[2] == v]
    pick = same if len(same) == 1 else (cands if len(cands) == 1 else [])
    if pick:
        k, y = pick[0]
        oo, ww, vv = A.site_desc(y)
        c, t = ctx_score(P, i, k)
        base.update(ok=True, r4=y.address - B, ins4=y, o4=oo, w4=ww, v4=vv,
                    why='replace구간 유일후보(값%s) 문맥%d/%d' % ('일치' if vv == v else '불일치', c, t),
                    conf='중' if c >= t - 1 else '하')
        return base
    base.update(why='replace구간 후보 %d개(값일치 %d)' % (len(cands), len(same)),
                cands=[(y.address - B, y) for _, y in cands[:8]])
    return base


def fmt(d):
    if not d.get('ok'):
        return ('%06x\t-\t-\t-\t-\t-\t미확정\t%s\t-' % (d['rva'], d['why']))
    y = d['ins4']
    pre = y.bytes[:d['o4']].hex()
    return ('%06x\t%06x\t%s\t%d\t%d\t%d\t살림\t%s\t%s'
            % (d['rva'], d['r4'], pre, d['o4'], d['w4'], d['v4'], d['why'], d['conf']))


if __name__ == '__main__':
    targets = [int(x, 16) for x in sys.argv[1:] if not x.startswith('-')]
    if not targets:
        sys.exit('사용: python revive.py <053rva>...')
    print('053rva\t054rva\tprefix\timm_off\tw\torig\t판정\t근거\t확신')
    for a in targets:
        d = judge(a)
        print(fmt(d))
        if not d.get('ok') and d.get('cands'):
            for ra, y in d['cands']:
                o, w, v = A.site_desc(y)
                print('    후보 %06x  %-20s %s %s  off=%d w=%d val=%d' % (ra, y.bytes.hex(), y.mnemonic, y.op_str, o, w, v))
