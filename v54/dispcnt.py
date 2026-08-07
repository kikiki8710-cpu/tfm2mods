# -*- coding: utf-8 -*-
"""dispcnt.py — 두 판의 같은 함수에서 **특정 니모닉+특정 즉치/변위값**을 쓰는 명령을
   순서대로 나열해 1:1 대응(순번 짝짓기)이 가능한지 본다. 반쪽노브·순번오프셋 판정용.
사용: python dispcnt.py <053fn> <054fn> <mnemonic> <value>
"""
import io, sys
sys.path.insert(0, r'C:\tfm2mods\v54')
import reloc as R
import align as A
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
B = 0x140000000
f3, f4, mn, val = int(sys.argv[1],16), int(sys.argv[2],16), sys.argv[3], int(sys.argv[4],0)
res = {}
for v, E, fs in (('053', R.E3, f3), ('054', R.E4, f4)):
    g = E.func_of(fs)
    out = []
    for i in R.insns(E, g[0], g[1]):
        if mn != '*' and i.mnemonic != mn: continue
        o, w, x = A.site_desc(i)
        if x == val:
            out.append((i.address-B, i, o, w))
    res[v] = out
    print('%s fn %06x: %d곳' % (v, fs, len(out)))
a, b = res['053'], res['054']
if len(a) == len(b):
    print('★개수 동일 → 순번 1:1 대응')
    for (x, ix, o1, w1), (y, iy, o2, w2) in zip(a, b):
        print('  %06x %-22s %-30s → %06x %-22s %-30s off %d→%d w %d→%d %s'
              % (x, ix.bytes.hex(), ix.mnemonic+' '+ix.op_str, y, iy.bytes.hex(),
                 iy.mnemonic+' '+iy.op_str, o1, o2, w1, w2, '⚠off변경' if o1!=o2 else ''))
else:
    print('⚠개수 다름 — 순번 짝짓기 불가. 전량 나열:')
    for v in ('053','054'):
        print(' [%s]' % v)
        for x, ix, o, w in res[v]:
            print('   %06x %-22s %-30s off=%d w=%d' % (x, ix.bytes.hex(), ix.mnemonic+' '+ix.op_str, o, w))
