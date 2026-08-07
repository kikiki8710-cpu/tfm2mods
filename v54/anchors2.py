# -*- coding: utf-8 -*-
"""Location 앵커 추출 v2 — anchors.py 의 오탐락(false negative) 수정판.

v1 버그: .rs 문자열을 정규식 **매치 시작 주소**로만 색인해서, 앞에 구분자 없이
다른 ASCII 가 붙어 있으면 매치 시작이 밀려 Location 의 ptr 과 안 맞았다.
  실제 피해: 0.5.3 plan 디스패처(c559e0)의 attack_nexus.rs 참조가 통째로 누락 →
  "0.5.4 신규 파일"로 오판할 뻔했다.
v2: ptr/len 을 먼저 읽고 **그 위치의 바이트열이 ASCII + '.rs' 로 끝나는지**로 검증.

산출: <ver>_srcmap2.tsv (함수단위) / locsite2 로 사이트단위 조회
"""
import io, os, re, sys, bisect, collections
sys.path.insert(0, r'C:\tfm2mods\v54')
from pe2 import load

_C = {}


def locmap(e):
    if e.ver in _C:
        return _C[e.ver]
    _, rd_va, _, rd_ra, rd_rsz = [s for s in e.sections if s[0] == '.rdata'][0]
    blob = e.raw[rd_ra:rd_ra + rd_rsz]
    ib = e.imagebase
    d = {}
    for i in range(0, len(blob) - 24, 8):
        ptr = int.from_bytes(blob[i:i + 8], 'little')
        if not (ib + rd_va <= ptr < ib + rd_va + len(blob)):
            continue
        ln = int.from_bytes(blob[i + 8:i + 16], 'little')
        if not (8 <= ln <= 200):
            continue
        o = ptr - ib - rd_va
        if o + ln > len(blob):
            continue
        s = blob[o:o + ln]
        if not s.endswith(b'.rs'):
            continue
        if any(c < 0x20 or c > 0x7e for c in s):
            continue
        line = int.from_bytes(blob[i + 16:i + 20], 'little')
        if 0 < line < 100000:
            d[rd_va + i] = (s.decode('latin1'), line)
    _C[e.ver] = d
    return d


def lea_sites(e):
    """[(site_rva, src, line)] .text 전체."""
    d = locmap(e)
    _, t_va, _, t_ra, t_rsz = [s for s in e.sections if s[0] == '.text'][0]
    body = e.raw[t_ra:t_ra + t_rsz]
    out = []
    for m in re.finditer(rb'[\x48\x4c]\x8d[\x05\x0d\x15\x1d\x25\x2d\x35\x3d]', body):
        o = m.start()
        if o + 7 > len(body):
            continue
        disp = int.from_bytes(body[o + 3:o + 7], 'little', signed=True)
        t = t_va + o + 7 + disp
        if t in d:
            out.append((t_va + o, d[t][0], d[t][1]))
    return out


def build(ver):
    e = load(ver)
    fns = e.funcs()
    starts = [f[0] for f in fns]
    hits = collections.defaultdict(set)
    for site, src, line in lea_sites(e):
        i = bisect.bisect_right(starts, site) - 1
        if i >= 0 and fns[i][0] <= site < fns[i][1]:
            hits[fns[i]].add((src, line))
    p = os.path.join(r'C:\tfm2mods\v54', '%s_srcmap2.tsv' % ver)
    with io.open(p, 'w', encoding='utf-8', newline='') as f:
        for (s, en), st in sorted(hits.items()):
            srcs = sorted(set(x[0] for x in st))
            lines = sorted(set(x[1] for x in st))
            f.write('%06x\t%06x\t%s\t%s\n' % (s, en, ' | '.join(srcs),
                                              ','.join(str(x) for x in lines[:14])))
    return len(hits), p


if __name__ == '__main__':
    sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
    a = sys.argv[1:]
    if a and a[0] == 'sites':
        e = load(a[1]); lo, hi = int(a[2], 16), int(a[3], 16)
        for s, src, ln in lea_sites(e):
            if lo <= s < hi:
                print('%06x  %s:%d' % (s, src, ln))
    else:
        for v in (a or ['053', '054']):
            n, p = build(v)
            print('%s: 함수 %d개 → %s' % (v, n, p))
