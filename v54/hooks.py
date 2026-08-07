# -*- coding: utf-8 -*-
"""훅 대상 함수(RVA_*)를 0.5.4 로 재핀한다.

★바이트패치와 위험도가 다르다. 바이트패치는 prefix·원본값 가드에서 걸리면
  조용히 skip 되지만, **트램폴린 훅은 주소가 틀리면 바로 크래시**다.
  그래서 여기서는 프롤로그 바이트와 orig_len 명령경계까지 같이 검증한다.

  · 짝짓기 = 소스 앵커 + 골격 정렬 (reloc.pair_fn 재사용)
  · 검증  = ① 프롤로그 N바이트 동일 여부 ② orig_len 이 명령경계에 정확히 떨어지는지
            ③ 트램폴린 구간에 rip-상대 명령·분기가 없는지
"""
import io, os, re, sys

sys.path.insert(0, r'C:\tfm2mods\v54')
import reloc as R
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
B = 0x140000000
E3, E4 = R.E3, R.E4

# (이름, 0.5.3 RVA, 이 훅이 쓰는 orig_len). orig_len=0 이면 훅이 아니라 상수 참조.
TARGETS = [
    ('RVA_RETREAT',        0xe00350, 12),
    ('RVA_GENERIC_BUILD',  0xe06c10, 12),
    ('RVA_FC59A0',         0xe168d0, 12),
    ('RVA_CONDGATE',       0xc550b0, 15),
    ('RVA_MOVEPRI',        0xc559e0, 12),
    ('RVA_SUBPLAN_DISPATCH', 0xd98740, 0),
    ('RVA_DISC18_HANDLER', 0xd94d00, 12),
    ('RVA_DISC19_HANDLER', 0xdece30, 12),
    ('RVA_ITEMNET_SCORER', 0x10587e0, 12),
]


def boundaries(E, s, n):
    """함수 시작부터 n바이트 안의 명령 경계 목록."""
    out, tot = [0], 0
    for i in E.dis(s, n + 24):
        tot += len(i.bytes)
        out.append(tot)
        if tot >= n + 16:
            break
    return out


def risky(E, s, n):
    bad = []
    for i in E.dis(s, n + 8):
        if i.address - B - s >= n:
            break
        op = i.op_str
        if 'rip' in op:
            bad.append('rip-상대 %s %s' % (i.mnemonic, op))
        if i.mnemonic in ('jmp', 'je', 'jne', 'jb', 'jae', 'jl', 'jg', 'jle', 'jge', 'call'):
            bad.append('분기/호출 %s %s' % (i.mnemonic, op))
    return bad


for name, old, olen in TARGETS:
    f3 = E3.func_of(old)
    print('■ %s  0.5.3 %06x' % (name, old))
    if not f3:
        print('   .pdata 에 없음 — 수동 확인 필요\n')
        continue
    pr = R.pair_fn(f3[0], f3[1])
    if not pr:
        print('   ★짝 못 찾음 (소스 앵커 없음)\n')
        continue
    bs, be, ratio = pr
    p3 = bytes(E3.rd(old, 20))
    p4 = bytes(E4.rd(bs, 20))
    print('   → 0.5.4 %06x  (크기 %d→%d, 골격 %.0f%%)' % (bs, f3[1] - f3[0], be - bs, ratio * 100))
    print('   프롤로그 20B  053 %s' % p3.hex())
    print('                054 %s   %s' % (p4.hex(), '완전동일' if p3 == p4 else '★다름'))
    if olen:
        b4 = boundaries(E4, bs, olen)
        print('   orig_len %d → 명령경계 %s  %s'
              % (olen, b4[:9], 'OK' if olen in b4 else '★경계 아님 — 이 값 그대로 쓰면 즉사'))
        bad = risky(E4, bs, olen)
        print('   트램폴린 구간 위험요소: %s' % (', '.join(bad) if bad else '없음'))
    print()
