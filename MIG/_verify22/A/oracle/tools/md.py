# -*- coding: utf-8 -*-
"""메타데이터 !N 을 출력 (irdump.load 재사용). 사용: python md.py m11.ll 55750 55751 55752"""
import sys
from irdump import load, chain
sys.stdout.reconfigure(encoding='utf-8', errors='replace')
name = sys.argv[1]
lines, md = load(name)
for a in sys.argv[2:]:
    i = int(a)
    s = md.get(i, '?')
    print(f'!{i} = {s[:400]}')
    if 'DILocation' in s:
        print('   chain:', chain(md, i))
