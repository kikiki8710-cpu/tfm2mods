# -*- coding: utf-8 -*-
"""경기 중 멈춤의 원인인 계측 프로브를 끈다.

ct_hunt=1 : 30틱마다 전체 replay 를 훑어 replay 당 전술 12필드를 format! 하고 파일에 append.
            경기 중에는 replay 집합이 갈려서 매 스캔이 수천 건을 '새 replay' 로 다시 뱉는다.
            → 한 판에 ct_hunt.txt 20MB / 110,509줄. 메인 스레드가 문자열+디스크에 잡아먹혀 **재생이 멈춘다.**
            (핫패스 파일 IO 금지 규칙 위반. cfg 주석에도 '찾으면 0으로' 라고 적혀 있었다.)
sp_seen=1 : subplan 후퇴발동 누적 측정. 가벼우나 목적 달성 후 켜둘 이유가 없다."""
import sys, io, re, os
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

MOD = ('C:/Program Files (x86)/Steam/steamapps/common/Teamfight Manager2'
       '/mods/tfm2_ai_adjust')
STAGE = 'C:/Users/dev/AppData/Local/Temp/claude/aiadj_rel_0806/tfm2_ai_adjust'
FILES = [MOD + '/tfm2_ai_adjust.cfg', MOD + '/config/테스트A.cfg',
         MOD + '/config/테스트B.cfg', MOD + '/config/테스트C.cfg',
         STAGE + '/tfm2_ai_adjust.cfg', STAGE + '/config/테스트A.cfg',
         STAGE + '/config/테스트B.cfg', STAGE + '/config/테스트C.cfg']

for p in FILES:
    if not os.path.exists(p):
        print('  [없음] %s' % p); continue
    t = io.open(p, encoding='utf-8').read()
    hits = []
    for k in ('ct_hunt', 'sp_seen'):
        pat = re.compile(r'(?m)^(\s*%s\s*=\s*)(\S+)(.*)$' % k)
        m = pat.search(t)
        if m and m.group(2) != '0':
            t = pat.sub(lambda mm: mm.group(1) + '0' + mm.group(3), t)
            hits.append('%s %s→0' % (k, m.group(2)))
    if hits:
        io.open(p, 'w', encoding='utf-8', newline='\n').write(t)
    tag = os.path.basename(os.path.dirname(p))[:6]
    print('  %-8s %-20s %s' % (tag, os.path.basename(p), ', '.join(hits) or '(이미 꺼짐)'))
