# -*- coding: utf-8 -*-
"""vt+0x28(0xeb00→0xeb30) 변경만 되돌린다.
사유: 근거가 약했다. 0xeaf8 은 0.5.4 exe 에 **0회**(죽음 확정)였지만,
      0xeb00 은 0.5.4 에도 **15회 살아 있다** — '덜 흔해졌다'는 것이 '틀렸다'는 뜻은 아니다.
      그런데 이 변경 이후 `checked=756 → 10`(적용 체인 사망)이 2판 연속 재현됐다.
      확정 근거가 있는 두 변경(athlete_id 0x800, vt+0x20 0xeb28)은 **유지**하고 이것만 되돌려
      원인을 한 변수로 가린다."""
import sys, io, re, os, glob
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

SRC = 'C:/tfm2mods/tfm2_ai_adjust/src'
total = 0
for f in sorted(glob.glob(os.path.join(SRC, '*.rs'))):
    t = io.open(f, encoding='utf-8').read()
    t2, n = re.subn(r'0xeb30(?![0-9a-fA-F])', '0xeb00', t)
    if n:
        io.open(f, 'w', encoding='utf-8', newline='\n').write(t2)
        print('  %-22s 0xeb30 → 0xeb00 : %d곳' % (os.path.basename(f), n))
        total += n
print('\n되돌림 %d곳' % total)

print('\n=== 유지되어야 할 확정 변경 ===')
for f in sorted(glob.glob(os.path.join(SRC, '*.rs'))):
    t = io.open(f, encoding='utf-8').read()
    a = len(re.findall(r'0xeb28(?![0-9a-fA-F])', t))
    b = len(re.findall(r'O_ATHLETE_ID: usize = 0x800', t))
    if a or b:
        print('  %-22s vt+0x20(0xeb28) %d곳 · athlete_id=0x800 %s' %
              (os.path.basename(f), a, 'O' if b else '-'))
