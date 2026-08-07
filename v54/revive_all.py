# -*- coding: utf-8 -*-
"""★revive_all.py — 죽은 사이트 전수를 문맥정렬(align)로 판독해 판정표 TSV 생성.

입력: 죽은 053 주소 목록(인자 또는 _deadaddrs.txt)
      + 소스에서 파싱한 (prefix, imm_off, width)  ← 053 기준. **054 값은 여기서 재계산.**
출력: revive_054.tsv  및 화면 표
컬럼: 053rva 054rva 새prefix 새imm_off w 새orig 판정 근거 확신 fnname
"""
import io, os, re, sys, collections

sys.path.insert(0, r'C:\tfm2mods\v54')
import align as A
import reloc as R
import sites as S1
import sites2 as S2
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
B = 0x140000000
E3, E4 = R.E3, R.E4

SITE = {}
for x in S1.parse() + S2.parse():
    SITE.setdefault(x['rva'], []).append(x)


def ctx_score(P, i, j, n=4):
    ok = tot = 0
    for d in list(range(-n, 0)) + list(range(1, n + 1)):
        ka, kb = i + d, j + d
        if 0 <= ka < len(P.a) and 0 <= kb < len(P.b):
            tot += 1
            if A.t2(P.a[ka]) == A.t2(P.b[kb]):
                ok += 1
    return ok, tot


def slot(ins, want=None):
    """소스가 지정한 off/w 가 있으면 그것, 아니면 capstone 기본 슬롯."""
    o, w, v = A.site_desc(ins)
    if want and (o, w) != want:
        wo, ww = want
        if wo + ww <= len(ins.bytes):
            return (wo, ww, int.from_bytes(ins.bytes[wo:wo + ww], 'little'), False)
        return (o, w, v, False)
    return (o, w, v, True)


def judge(rva):
    src = SITE.get(rva, [])
    want = (src[0]['off'], src[0]['w']) if src else None
    P = A.pair_of(rva)
    res = dict(rva=rva, want=want, r4=None, why='', conf='-')
    if not P:
        res['why'] = '짝함수없음'
        return res
    i = P.ia.get(rva)
    if i is None:
        res['why'] = '053명령경계아님'
        return res
    ins = P.a[i]
    res['ins3'] = ins
    o3, w3, v3, exact3 = slot(ins, want)
    res.update(o3=o3, w3=w3, v3=v3, slot3ok=exact3, fn3=P.f3, fn4=P.f4, ratio=P.ratio)

    got = None
    for nm, ops in (('L1', P.op1), ('L2', P.op2)):
        j, tag, rng = P.map_idx(i, ops)
        if j is None:
            continue
        y = P.b[j]
        if y.mnemonic != ins.mnemonic:
            continue
        c, t = ctx_score(P, i, j)
        got = (nm, j, y, c, t, '%s equal' % nm)
        break
    if got is None:
        lo, hi = P.bracket(i, P.op2)
        cands = [(k, P.b[k]) for k in range(lo, min(hi, len(P.b))) if P.b[k].mnemonic == ins.mnemonic]
        same = [(k, y) for k, y in cands if A.site_desc(y)[2] == v3]
        pool = same if len(same) == 1 else (cands if len(cands) == 1 else [])
        if pool:
            k, y = pool[0]
            c, t = ctx_score(P, i, k)
            got = ('BR', k, y, c, t, 'replace구간 유일후보')
        else:
            res['why'] = 'replace구간 후보%d(값일치%d)' % (len(cands), len(same))
            res['cands'] = [(y.address - B, y) for _, y in cands[:10]]
            return res
    nm, j, y, c, t, tag = got
    o4, w4, v4, _ = slot(y, (A.site_desc(y)[0], w3) if A.site_desc(y)[1] else want)
    # 소스가 지정한 폭을 존중(054 imm 크기가 같아야 함)
    if w3 != w4:
        # 폭이 달라졌다 = 명령 형태가 바뀜 → 확신 강등
        pass
    res.update(r4=y.address - B, ins4=y, o4=o4, w4=w4, v4=v4,
               pre4=y.bytes[:o4].hex(),
               why='%s 문맥%d/%d%s' % (tag, c, t, '' if v4 == v3 else ' ⚠값%d≠%d' % (v4, v3)))
    if nm == 'L1' and c == t and v4 == v3:
        res['conf'] = '상'
    elif c >= t - 1 and v4 == v3:
        res['conf'] = '상' if nm != 'BR' else '중'
    elif c >= t - 2:
        res['conf'] = '중'
    else:
        res['conf'] = '하'
    return res


if __name__ == '__main__':
    args = [a for a in sys.argv[1:] if not a.startswith('-')]
    if args and os.path.exists(args[0]):
        addrs = [int(l.split()[0], 16) for l in io.open(args[0]) if l.strip() and not l.startswith('#')]
    else:
        addrs = [int(a, 16) for a in args]
    rows = []
    print('%-8s %-8s %-14s %-4s %-2s %-12s %-4s %-3s %s'
          % ('053', '054', 'prefix', 'off', 'w', 'orig', '판정', '확신', '근거'))
    for a in addrs:
        d = judge(a)
        if d['r4']:
            print('%06x   %06x   %-14s %-4d %-2d %-12d %-4s %-3s %s'
                  % (a, d['r4'], d['pre4'], d['o4'], d['w4'], d['v4'], '살림', d['conf'], d['why']))
        else:
            print('%06x   %-8s %-14s %-4s %-2s %-12s %-4s %-3s %s'
                  % (a, '-', '-', '-', '-', '-', '미확정', '-', d['why']))
            for ra, y in d.get('cands', []):
                o, w, v = A.site_desc(y)
                print('        후보 %06x %-22s %s %s | off=%d w=%d val=%d' % (ra, y.bytes.hex(), y.mnemonic, y.op_str, o, w, v))
        rows.append(d)
    ok = sum(1 for r in rows if r['r4'])
    print('\n살림 %d / 미확정 %d (총 %d)' % (ok, len(rows) - ok, len(rows)))
    with io.open(r'C:\tfm2mods\v54\revive_054.tsv', 'w', encoding='utf-8', newline='') as f:
        f.write('rva053\trva054\tprefix\toff\tw\torig\t확신\t근거\n')
        for d in rows:
            if d['r4']:
                f.write('%06x\t%06x\t%s\t%d\t%d\t%d\t%s\t%s\n'
                        % (d['rva'], d['r4'], d['pre4'], d['o4'], d['w4'], d['v4'], d['conf'], d['why']))
            else:
                f.write('%06x\t-\t-\t-\t-\t-\t-\t%s\n' % (d['rva'], d['why']))
