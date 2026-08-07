# -*- coding: utf-8 -*-
"""e9jt 를 켜짐(=1)으로. 편집기에서 `기본 1` 인데 cfg 가 0 이라 체크 해제·꺼짐으로 보였다."""
import sys, io, re, os
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

MOD = ('C:/Program Files (x86)/Steam/steamapps/common/Teamfight Manager2'
       '/mods/tfm2_ai_adjust')
for p in [MOD + '/tfm2_ai_adjust.cfg', MOD + '/config/테스트C.cfg']:
    t = io.open(p, encoding='utf-8').read()
    hits = len(re.findall(r'(?m)^\s*e9jt\s*=.*$', t))
    if hits:
        t = re.sub(r'(?m)^\s*e9jt\s*=.*$', 'e9jt = 1', t)
    else:
        t = t.rstrip('\n') + ('\n# [08-06] 교전 판단 점프테이블 경로(정확도↑) — 기본 1(켜짐)\ne9jt = 1\n')
    io.open(p, 'w', encoding='utf-8', newline='\n').write(t)
    v = re.search(r'(?m)^\s*e9jt\s*=\s*(\S+)', io.open(p, encoding='utf-8').read()).group(1)
    print('%-22s e9jt = %s   (기존 줄 %d개)' % (os.path.basename(p), v, hits))
