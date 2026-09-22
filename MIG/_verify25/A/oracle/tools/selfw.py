# -*- coding: utf-8 -*-
"""③ &mut self(%1) 접근 전수: gep 파생 → load/store 추적"""
from load import *
import re
regs = {'%1': 0}
# 1) %1 파생 gep
for i in range(LO, HI+1):
    s = L(i)
    m = re.match(r'\s*(%\d+) = getelementptr inbounds (?:nuw )?i8, ptr (%\d+), i64 (\d+)', s)
    if m and m.group(2) in regs:
        regs[m.group(1)] = regs[m.group(2)] + int(m.group(3))
        print('GEP', i, m.group(1), '= self+%#x' % regs[m.group(1)], ';L'+chain(i))
# 2) load/store on those
print('---- load/store ----')
for i in range(LO, HI+1):
    s = L(i)
    for r, off in regs.items():
        if re.search(r'(load|store) [^,]+,? ?[^,]*ptr ' + re.escape(r) + r'[,\s]', s) or re.search(r'ptr ' + re.escape(r) + r'[,)]', s):
            kind = 'STORE' if s.strip().startswith('store') else ('LOAD' if '= load' in s else 'USE')
            print(kind, i, 'self+%#x' % off, s.strip()[:110], ';L'+chain(i))
            break
