# -*- coding: utf-8 -*-
# _ui071\i18n_diff.py 사본 — 대상 폴더만 0.7.2 로 바꿈 (인자 = recdiff 폴더 · 생략 시 이 파일 옆 recdiff)
import os, sys, re, difflib, glob
sys.stdout.reconfigure(encoding='utf-8')
D = sys.argv[1] if len(sys.argv) > 1 else os.path.join(os.path.dirname(os.path.abspath(__file__)), 'recdiff')
def strs(b):
    # 길이접두 또는 NUL/제어문자로 구분된 텍스트 조각을 utf-8 로 분리
    t = b.decode('utf-8', 'replace')
    return [s for s in re.split(r'[\x00-\x08\x0b-\x1f�]+', t) if s.strip()]
for old in sorted(glob.glob(os.path.join(D, 'i18n__*.old'))) + sorted(glob.glob(os.path.join(D, 'data_champion__*.old'))) + sorted(glob.glob(os.path.join(D, 'champion_info_sheet__*.old'))):
    new = old[:-4] + '.new'
    a = open(old, 'rb').read(); b = open(new, 'rb').read()
    print('=====', os.path.basename(old), len(a), len(b))
    sa, sb = strs(a), strs(b)
    print(' 조각', len(sa), len(sb))
    sm = difflib.SequenceMatcher(None, sa, sb, autojunk=False)
    n = 0
    for op, i1, i2, j1, j2 in sm.get_opcodes():
        if op == 'equal':
            continue
        n += 1
        if n > 120:
            print('  ...'); break
        print(' ', op, [x[:200] for x in sa[i1:i2]][:8], '=>', [x[:200] for x in sb[j1:j2]][:8])
