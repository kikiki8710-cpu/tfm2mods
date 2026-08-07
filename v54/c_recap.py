# -*- coding: utf-8 -*-
"""원본 테스트C(제보자 Downloads cfg) ↔ 현재 config\테스트C.cfg 전수 대조.
기억이 아니라 파일에서 다시 뽑는다."""
import sys, io, os
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

SRC = r'C:\Users\dev\Downloads\tfm2_ai_adjust\tfm2_ai_adjust.cfg'
CUR = (r'C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2'
       r'\mods\tfm2_ai_adjust\config\테스트C.cfg')
if not os.path.exists(SRC):
    print('원본 없음: %s' % SRC); sys.exit(1)


def kv(p):
    d = {}
    for ln in io.open(p, encoding='utf-8'):
        s = ln.strip()
        if s and not s.startswith('#') and '=' in s:
            k, v = [x.strip() for x in s.split('=', 1)]
            d[k] = v
    return d


a, b = kv(SRC), kv(CUR)
print('원본 활성 키 %d  →  현재 %d' % (len(a), len(b)))

gone = sorted(set(a) - set(b))
new = sorted(set(b) - set(a))
chg = sorted(k for k in set(a) & set(b) if a[k] != b[k])

print('\n[1] 원본에 있었는데 지금 없는 키 : %d' % len(gone))
for k in gone:
    print('    %-22s %s' % (k, a[k]))
print('\n[2] 새로 생긴 키 : %d' % len(new))
for k in new:
    print('    %-22s %s' % (k, b[k]))
print('\n[3] 값이 바뀐 키 : %d' % len(chg))
for k in chg:
    print('    %-22s %s → %s' % (k, a[k], b[k]))
print('\n[4] 값 그대로 유지 : %d' % len(set(a) & set(b) - set(chg)))
