# -*- coding: utf-8 -*-
"""앵커 추출 — **소스 파일명**으로 함수 신원을 세운다.

Rust 는 패닉 지점마다 `Location { file, line, col }` 을 .rdata 에 심는다.
파일 문자열은 컴파일러가 넣은 것이라 **버전이 바뀌어도 이름이 안 변한다** ⟹
RVA 가 통째로 밀려도 "이 함수가 어느 소스인지"를 잃지 않는다.
(0.5.3 에서 plan 이름을 이 방법으로 확정했고, 스테일 주석을 반박한 것도 이 근거였다.)

산출:  <ver>_srcmap.tsv   함수시작RVA \t 함수끝 \t 소스경로 \t 참조된 줄들
"""
import io, re, sys, os, collections

sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
sys.path.insert(0, r'C:\tfm2mods\v54')
from pe2 import load, BASE

SRC_PAT = re.compile(rb'[A-Za-z0-9_\-]+[\\/](?:[A-Za-z0-9_\-]+[\\/])*[A-Za-z0-9_\-]+\.rs')


def collect(ver):
    e = load(ver)
    rd_name, rd_va, rd_vsz, rd_ra, rd_rsz = [s for s in e.sections if s[0] == '.rdata'][0]
    blob = e.raw[rd_ra:rd_ra + rd_rsz]

    # 1) .rdata 안의 *.rs 문자열 위치 수집
    str_at = {}
    for m in SRC_PAT.finditer(blob):
        rva = rd_va + m.start()
        str_at[rva] = m.group().decode('latin1')
    print('[%s] .rs 문자열 %d개' % (ver, len(str_at)))

    # 2) Location 구조체 = { ptr(8), len(8), line(4), col(4) } — ptr 이 위 문자열을 가리킨다
    locs = []          # (loc_rva, src, line)
    for i in range(0, len(blob) - 24, 8):
        ptr = int.from_bytes(blob[i:i + 8], 'little')
        if ptr < e.imagebase or ptr > e.imagebase + 0x5000000:
            continue
        rva = ptr - e.imagebase
        s = str_at.get(rva)
        if not s:
            continue
        ln = int.from_bytes(blob[i + 8:i + 16], 'little')
        if ln != len(s):
            continue
        line = int.from_bytes(blob[i + 16:i + 20], 'little')
        if 0 < line < 100000:
            locs.append((rd_va + i, s, line))
    print('[%s] Location 구조체 %d개' % (ver, len(locs)))

    # 3) .text 에서 각 Location 을 lea 로 집는 지점 → 그 함수에 소속시킴
    loc_by_rva = {r: (s, l) for r, s, l in locs}
    text = [x for x in e.sections if x[0] == '.text'][0]
    _, t_va, t_vsz, t_ra, t_rsz = text
    body = e.raw[t_ra:t_ra + t_rsz]

    funcs = e.funcs()
    starts = [f[0] for f in funcs]
    import bisect

    hits = collections.defaultdict(set)
    # lea reg,[rip+disp32] : 48 8d ?? disp32   /  4c 8d ?? disp32
    for m in re.finditer(rb'[\x48\x4c]\x8d[\x05\x0d\x15\x1d\x25\x2d\x35\x3d]', body):
        o = m.start()
        if o + 7 > len(body):
            continue
        disp = int.from_bytes(body[o + 3:o + 7], 'little', signed=True)
        tgt = t_va + o + 7 + disp
        if tgt in loc_by_rva:
            site = t_va + o
            i = bisect.bisect_right(starts, site) - 1
            if i >= 0 and funcs[i][0] <= site < funcs[i][1]:
                src, line = loc_by_rva[tgt]
                hits[funcs[i]].add((src, line))
    print('[%s] 소스가 붙은 함수 %d개' % (ver, len(hits)))

    out = os.path.join(r'C:\tfm2mods\v54', '%s_srcmap.tsv' % ver)
    with io.open(out, 'w', encoding='utf-8', newline='') as f:
        for (s, en), st in sorted(hits.items()):
            srcs = sorted(set(x[0] for x in st))
            lines = sorted(set(x[1] for x in st))
            f.write('%06x\t%06x\t%s\t%s\n' % (s, en, ' | '.join(srcs),
                                              ','.join(str(x) for x in lines[:12])))
    print('[%s] → %s' % (ver, out))
    return hits


if __name__ == '__main__':
    for v in (sys.argv[1:] or ['053', '054']):
        collect(v)
