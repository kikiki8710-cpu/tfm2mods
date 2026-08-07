# -*- coding: utf-8 -*-
"""4단계 엔진 — 노브가 들어있던 0.5.3 함수마다 **0.5.4 짝**을 찾는다.

절대 하지 않는 것: 상수값으로 짝짓기. (그게 "설정값이 죽었다" 오판의 원인)
하는 것:
  ① 후보를 **소스 파일명이 겹치는 0.5.4 함수**로만 좁힌다 (패닉 Location = 버전무관 앵커)
  ② 싸구려 mnemonic 히스토그램으로 상위 후보만 추린다
  ③ 그 후보만 **명령 골격 정렬**로 정밀 비교 → 일치율
  ④ 일치율로 판정. 낮으면 "미확정"으로 남기고 사람이 본다. **추측으로 확정하지 않는다.**

산출: pair_054.tsv
"""
import io, os, sys, math, difflib, collections

sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
sys.path.insert(0, r'C:\tfm2mods\v54')
from pe2 import load

D = r'C:\tfm2mods\v54'
E3 = load('053')
E4 = load('054')


def srcmap(ver):
    out = []
    for ln in io.open(os.path.join(D, '%s_srcmap.tsv' % ver), encoding='utf-8'):
        s, e, src, lines = ln.rstrip('\n').split('\t')
        out.append((int(s, 16), int(e, 16), src))
    return out


def parts(src):
    return set(p.strip() for p in src.split('|') if p.strip())


S3, S4 = srcmap('053'), srcmap('054')
# 0.5.4: 소스파일 → 함수들
by_file = collections.defaultdict(list)
for s, e, src in S4:
    for p in parts(src):
        by_file[p].append((s, e, src))

_sk = {}


def skel(E, s, e):
    k = (id(E), s)
    if k not in _sk:
        _sk[k] = [i.mnemonic for i in E.dis(s, e - s)]
    return _sk[k]


def hist(m):
    c = collections.Counter(m)
    n = math.sqrt(sum(v * v for v in c.values())) or 1
    return c, n


def cos(a, b):
    ca, na = a
    cb, nb = b
    return sum(v * cb.get(k, 0) for k, v in ca.items()) / (na * nb)


def main():
    # 노브가 있는 0.5.3 함수들
    fns = {}
    first = True
    for ln in io.open(os.path.join(D, 'knob_sites_053.tsv'), encoding='utf-8'):
        if first:
            first = False
            continue
        k, rva, fs, fe, src, by, asm, where = ln.rstrip('\n').split('\t')
        fs, fe = int(fs, 16), int(fe, 16)
        if fs == 0:
            continue
        d = fns.setdefault((fs, fe, src), {'knobs': set(), 'n': 0})
        d['n'] += 1
        if k != '?':
            d['knobs'].add(k)

    print('노브가 든 0.5.3 함수 %d개 — 짝 찾는 중\n' % len(fns))
    rows = []
    for (fs, fe, src) in sorted(fns, key=lambda x: -(x[1] - x[0])):
        info = fns[(fs, fe, src)]
        sz = fe - fs
        cand = {}
        for p in parts(src):
            for c in by_file.get(p, []):
                cand[c[0]] = c
        cand = [c for c in cand.values() if 0.35 < (c[1] - c[0]) / sz < 2.8]
        if not cand:
            rows.append((fs, fe, src, 0, 0, 0.0, '후보없음', info))
            continue
        h3 = hist(skel(E3, fs, fe))
        cand.sort(key=lambda c: -cos(h3, hist(skel(E4, c[0], c[1]))))
        best = None
        for c in cand[:4]:
            r = difflib.SequenceMatcher(None, skel(E3, fs, fe),
                                        skel(E4, c[0], c[1]), autojunk=False).ratio()
            if best is None or r > best[2]:
                best = (c[0], c[1], r, c[2])
        v = ('동일-이동' if best[2] >= 0.93 else
             '부분수정' if best[2] >= 0.75 else
             '대폭수정' if best[2] >= 0.50 else '미확정')
        rows.append((fs, fe, src, best[0], best[1], best[2], v, info))
        print('  %06x(%6dB) → %06x(%6dB)  %5.1f%%  %-8s  사이트%2d  %s'
              % (fs, sz, best[0], best[1] - best[0], best[2] * 100, v, info['n'],
                 ','.join(sorted(info['knobs'])[:5]) or '-'))

    with io.open(os.path.join(D, 'pair_054.tsv'), 'w', encoding='utf-8', newline='') as fo:
        fo.write('fn053_s\tfn053_e\tsrc\tfn054_s\tfn054_e\tratio\tverdict\tsites\tknobs\n')
        for fs, fe, src, bs, be, r, v, info in rows:
            fo.write('%06x\t%06x\t%s\t%06x\t%06x\t%.3f\t%s\t%d\t%s\n'
                     % (fs, fe, src, bs, be, r, v, info['n'], ','.join(sorted(info['knobs']))))

    c = collections.Counter(r[6] for r in rows)
    ns = collections.Counter()
    for r in rows:
        ns[r[6]] += r[7]['n']
    print('\n판정별 (함수 / 사이트)')
    for k in ('동일-이동', '부분수정', '대폭수정', '미확정', '후보없음'):
        print('  %-6s  함수 %3d   사이트 %3d' % (k, c[k], ns[k]))

    # ★충돌 검출 — 0.5.3 함수 둘 이상이 같은 0.5.4 함수를 가리키면 최소 하나는 틀렸다.
    #   쌍둥이 함수(크기·골격이 같은 제네릭 인스턴스)에서 실제로 발생한다.
    #   자동 확정 금지 대상으로 표시하고 호출자 문맥으로 사람이 가른다.
    tgt = collections.defaultdict(list)
    for fs, fe, src, bs, be, r, v, info in rows:
        if bs:
            tgt[bs].append((fs, r, sorted(info['knobs'])))
    dup = {k: v for k, v in tgt.items() if len(v) > 1}
    if dup:
        print('\n⚠짝 충돌 — 아래는 자동 확정 금지(호출자로 구분해야 함)')
        for t, lst in dup.items():
            print('  0.5.4 %06x ← ' % t + ' , '.join(
                '%06x(%.0f%%, %s)' % (a, r * 100, ','.join(k) or '-') for a, r, k in lst))
    print('\n→ pair_054.tsv')


if __name__ == '__main__':
    main()
