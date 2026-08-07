# -*- coding: utf-8 -*-
"""남은 훅/데이터 상수 재핀 — 소스 앵커가 없거나 .rdata 인 것들."""
import io, sys

sys.path.insert(0, r'C:\tfm2mods\v54')
import reloc as R
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
B = 0x140000000
E3, E4 = R.E3, R.E4

print('■ RVA_SUBPLAN_DISPATCH  053 d98740 → 054 e52990 (에이전트 RE 주장) 검증')
for E, a, tag in ((E3, 0xd98740, '053'), (E4, 0xe52990, '054')):
    p = bytes(E.rd(a, 24))
    f = E.func_of(a)
    print('   %s %06x  크기 %d  프롤로그 %s' % (tag, a, (f[1] - f[0]) if f else 0, p.hex()))
print('   ⟹ 크기 1278 vs 1310, 프롤로그 동형이면 채택')

print('\n■ .rdata TABLE_A  053 0x31c0168 (앞 4엔트리 = [0,1,3,2] u64)')
want = b''.join(x.to_bytes(8, 'little') for x in (0, 1, 3, 2))
cur = bytes(E3.rd(0x31c0168, 32))
print('   053 실값 %s  (기대와 %s)' % (cur.hex(), '일치' if cur == want else '불일치'))


def scan(E, pat, lo=0x3000000, hi=0x3600000):
    hits = []
    step = 0x20000
    a = lo
    while a < hi:
        n = min(step, hi - a)
        try:
            buf = bytes(E.rd(a, n + len(pat)))
        except Exception:
            a += step
            continue
        i = buf.find(pat)
        while i >= 0:
            hits.append(a + i)
            i = buf.find(pat, i + 1)
        a += step
    return hits


h = scan(E4, want)
print('   054 후보 %d개: %s' % (len(h), ' '.join('%x' % x for x in h[:8])))

print('\n■ 데미지시트 desc — 053 disc19(dece30) 안의 `lea r9,[rip]` 로 찾았던 방식 재현')
for (E, fs, tag) in ((E3, 0xdece30, '053'), (E4, 0xdac090, '054')):
    f = E.func_of(fs)
    got = []
    for i in R.insns(E, f[0], f[1]):
        if i.mnemonic == 'lea' and i.op_str.startswith('r9,') and 'rip' in i.op_str:
            t = i.address - B + len(i.bytes) + int(i.op_str.split('+')[-1].strip(' ]'), 16)
            got.append((i.address - B, t))
    print('   %s disc19 안의 lea r9,[rip]: %s'
          % (tag, ' , '.join('%06x→%x' % g for g in got) or '없음'))

print('\n■ C8C_DMG_SHEET — 053 0xdff660(passive_jungle) 안의 lea r9 2사이트였다')
for (E, fs, tag) in ((E3, 0xdff660, '053'), (E4, 0xe619c0, '054')):
    f = E.func_of(fs)
    got = []
    for i in R.insns(E, f[0], f[1]):
        if i.mnemonic == 'lea' and i.op_str.startswith('r9,') and 'rip' in i.op_str:
            t = i.address - B + len(i.bytes) + int(i.op_str.split('+')[-1].strip(' ]'), 16)
            got.append((i.address - B, t))
    print('   %s %06x 안의 lea r9,[rip]: %s'
          % (tag, fs, ' , '.join('%06x→%x' % g for g in got) or '없음'))
