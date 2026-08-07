# -*- coding: utf-8 -*-
"""JT 사이트의 '인덱스 계산식' 문맥 덤프 — movsxd 앞 N개 명령.
  python jtctx.py 054 e145b0 [앞명령수]
"""
import io, sys
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
sys.path.insert(0, r'C:\tfm2mods\v54')
from pe2 import load, BASE
from jtscan import scan, entries, fast_func_of

ver, rva = sys.argv[1], int(sys.argv[2], 16)
back = int(sys.argv[3]) if len(sys.argv) > 3 else 12
e = load(ver)
fn = fast_func_of(e, rva)
for jmp, jt, br, ir, lea, mv in scan(e):
    if fn[0] <= jmp < fn[1]:
        ent = entries(e, jt, fn)
        print('--- jmp %06x JT %06x ent=%d idx=%s ---' % (jmp, jt, len(ent), ir))
        # movsxd(mv) 앞 back개: 함수 시작부터 선형 디스어셈해서 잘라 씀
        ins = e.dis(fn[0], mv - fn[0] + 16)
        seq = [i for i in ins if i.address - BASE <= mv]
        for i in seq[-back:]:
            print('   %06x  %-20s %s %s' % (i.address - BASE, i.bytes.hex(), i.mnemonic, i.op_str))
