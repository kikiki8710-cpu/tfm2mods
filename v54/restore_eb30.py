# -*- coding: utf-8 -*-
"""내 되돌리기가 **원래부터 0xeb30 이던 11곳**까지 0xeb00 으로 바꿔버린 것을 복구한다.
방법: HEAD(0.5.3) 에서 0xeb30 을 담은 줄을 뽑아, 그 줄의 0xeb30 을 0xeb00 으로 바꾼 형태를
      현재 파일에서 찾아 되살린다(그 줄들은 마이그레이션에서 손대지 않은 줄이다).
검증: 파일별 복구 수가 (revert수 − fix_vt수) 와 일치해야 한다. disc19=3 · serpen=3 · main=5."""
import sys, io, subprocess
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

SRC = 'C:/tfm2mods/tfm2_ai_adjust/src/'
EXPECT = {'disc19_repro.rs': 3, 'serpen.rs': 3, 'tfm2_ai_adjust.rs': 5}

for fname, exp in EXPECT.items():
    head = subprocess.run(['git', 'show', 'HEAD:tfm2_ai_adjust/src/' + fname],
                          cwd='C:/tfm2mods', capture_output=True)
    htext = head.stdout.decode('utf-8', 'replace')
    keys = [ln.strip() for ln in htext.split('\n') if '0xeb30' in ln]
    cur = io.open(SRC + fname, encoding='utf-8').read().split('\n')
    done = 0
    for k in keys:
        target = k.replace('0xeb30', '0xeb00')
        for i, ln in enumerate(cur):
            if ln.strip() == target:
                cur[i] = ln.replace('0xeb00', '0xeb30', 1)
                done += 1
                break
    io.open(SRC + fname, 'w', encoding='utf-8', newline='\n').write('\n'.join(cur))
    ok = 'OK' if done == exp else '★불일치'
    print('  %-22s 복구 %d / 기대 %d  %s' % (fname, done, exp, ok))

print('\n=== 최종 분포 ===')
import glob, os, re
for f in sorted(glob.glob(SRC + '*.rs')):
    t = io.open(f, encoding='utf-8').read()
    a = len(re.findall(r'0xeb00(?![0-9a-fA-F])', t))
    b = len(re.findall(r'0xeb30(?![0-9a-fA-F])', t))
    c = len(re.findall(r'0xeb28(?![0-9a-fA-F])', t))
    if a or b or c:
        print('  %-22s 0xeb00=%2d  0xeb30=%2d  0xeb28=%2d' % (os.path.basename(f), a, b, c))
