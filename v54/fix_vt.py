# -*- coding: utf-8 -*-
"""vt+0x20 / vt+0x28 구조체 오프셋을 0.5.4 값으로 일괄 정정.
실측(v54\\offscan.py, 0.5.3 → 0.5.4 disp32 출현수):
    vt+0x20 : 0xeaf8  7 → **0**(죽음)   /  0xeb28  25 → 8
    vt+0x28 : 0xeb00 83 → 15            /  0xeb30  29 → **78**
⚠`0xeb00ba` 는 오프셋이 아니라 **패치 사이트 RVA** 다 — 뒤에 hex 가 붙는 경우는 제외한다."""
import sys, io, re, os, glob
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

SRC = 'C:/tfm2mods/tfm2_ai_adjust/src'
PAIRS = [(r'0xeaf8', '0xeb28', 'vt+0x20'), (r'0xeb00', '0xeb30', 'vt+0x28')]
total = 0
for f in sorted(glob.glob(os.path.join(SRC, '*.rs'))):
    t = io.open(f, encoding='utf-8').read()
    orig = t
    for old, new, tag in PAIRS:
        # 뒤에 hex 가 이어지면 RVA/다른 상수 → 건드리지 않는다
        t, n = re.subn(old + r'(?![0-9a-fA-F])', new, t)
        if n:
            print('  %-22s %s → %s : %d곳' % (os.path.basename(f), old, new, n))
            total += n
    if t != orig:
        io.open(f, 'w', encoding='utf-8', newline='\n').write(t)
print('\n총 %d곳 정정' % total)

print('\n=== 잔존 확인 (오프셋으로 남으면 안 됨) ===')
for f in sorted(glob.glob(os.path.join(SRC, '*.rs'))):
    t = io.open(f, encoding='utf-8').read()
    for m in re.finditer(r'.*0xeaf8(?![0-9a-fA-F]).*|.*0xeb00(?![0-9a-fA-F]).*', t):
        print('  ⚠ %s: %s' % (os.path.basename(f), m.group(0).strip()[:90]))
print('\n=== RVA(건드리면 안 되는 것) 보존 확인 ===')
for f in sorted(glob.glob(os.path.join(SRC, '*.rs'))):
    t = io.open(f, encoding='utf-8').read()
    for m in re.finditer(r'.*0xeb00ba.*', t):
        print('  ok %s: %s' % (os.path.basename(f), m.group(0).strip()[:90]))
