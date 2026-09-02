#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""posdiff.py — 구/신 함수를 **명령 순서대로 나란히 걸어** 메모리 disp 가 달라진 자리만 뽑는다.

왜: `offsets.py check` 는 "어떤 오프셋이 사라지고 생겼나"(집합 diff)까지만 본다.
    집합만 보면 같은 수가 여러 구조체에서 쓰일 때 짝짓기가 흔들린다.
    **명령 인덱스가 같은 자리끼리** 비교하면 "이 필드가 저기로 갔다"가 1:1로 확정된다.
    (2026-09-02: serpen `O_ENTITY_ACCESSOR 0x1e0→0x1f0` 을 이 방식으로 확정 — 크래시의 진범)

사용:
  python MIG\posdiff.py --old <구exe> --oldpkl <구pkl> --new <신exe> --newpkl <신pkl>
                        --fn <구RVA> <신RVA> [--lo 0x40] [--hi 0xffff]
"""
import sys, os, re, argparse, collections

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
sys.stdout.reconfigure(encoding='utf-8', errors='replace')
from repin import Img  # noqa: E402
import capstone        # noqa: E402

_md = capstone.Cs(capstone.CS_ARCH_X86, capstone.CS_MODE_64)
DISP = re.compile(r'\[(r[a-z0-9]+) \+ 0x([0-9a-f]+)\]')


def walk(img, fn):
    """(명령 인덱스, 니모닉, [(베이스레지스터, disp)]) 목록. 스택 상대는 제외."""
    code = img.read(fn, img.fn[fn]['size'])
    out = []
    for k, ins in enumerate(_md.disasm(code, fn)):
        ds = [(m.group(1), int(m.group(2), 16)) for m in DISP.finditer(ins.op_str)
              if m.group(1) not in ('rbp', 'rsp')]
        out.append((k, ins.address, ins.mnemonic, ins.op_str, ds))
    return out


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--old', required=True), ap.add_argument('--oldpkl', required=True)
    ap.add_argument('--new', required=True), ap.add_argument('--newpkl', required=True)
    ap.add_argument('--fn', nargs=2, required=True, help='구RVA 신RVA (0x…)')
    ap.add_argument('--lo', type=lambda x: int(x, 0), default=0x40)
    ap.add_argument('--hi', type=lambda x: int(x, 0), default=0xffff)
    a = ap.parse_args()
    O = Img(a.old, a.oldpkl)
    N = Img(a.new, a.newpkl)
    of, nf = int(a.fn[0], 0), int(a.fn[1], 0)
    ow, nw = walk(O, of), walk(N, nf)
    print('구 %s (%d 명령) ↔ 신 %s (%d 명령)' % (hex(of), len(ow), hex(nf), len(nw)))
    if len(ow) != len(nw):
        print('⚠명령 수가 다르다(%d vs %d) — 본문 변경. 위치 대조 신뢰도가 떨어지니 결과를 문맥으로 재확인할 것.'
              % (len(ow), len(nw)))
    n = min(len(ow), len(nw))
    votes = collections.Counter()
    rows = []
    for i in range(n):
        _, oa, om, oo, od = ow[i]
        _, na, nm, no, nd = nw[i]
        if om != nm or len(od) != len(nd):
            continue                     # 명령이 어긋나면 그 자리는 판단 보류
        for (obr, ov), (nbr, nv) in zip(od, nd):
            if ov == nv or not (a.lo <= ov <= a.hi):
                continue
            votes[nv - ov] += 1
            rows.append((oa, na, om, obr, ov, nv))
    print('\n델타 분포: %s' % [(hex(k), v) for k, v in votes.most_common(6)])
    print('\n%-12s %-12s %-8s %-5s %-8s -> %-8s' % ('구주소', '신주소', '명령', '베이스', '구off', '신off'))
    seen = set()
    for oa, na, om, br, ov, nv in rows:
        if (ov, nv, br) in seen:
            continue
        seen.add((ov, nv, br))
        print('%-12s %-12s %-8s %-5s %#-8x -> %#-8x  (Δ%+#x)'
              % (hex(oa), hex(na), om, br, ov, nv, nv - ov))
    print('\n(같은 (구off,신off,베이스) 조합은 한 번만 표시 — 전수는 rows %d개)' % len(rows))


if __name__ == '__main__':
    main()
