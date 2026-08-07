# -*- coding: utf-8 -*-
"""릴리스 zip 에 들어갈 tfm2_ai_adjust.cfg 를 테스트C 와 같은 값으로 맞춘다.
   ⚠라이브(게임 폴더 루트)의 cfg 는 건드리지 않는다 — 그건 유저의 작업본이다.
   값은 그대로 두고 머리말 한 줄만 '릴리스 기본값'으로 바꾼다."""
import sys, io, re
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

SRC = (r'C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2'
       r'\mods\tfm2_ai_adjust\config\테스트C.cfg')
DST = (r'C:\Users\dev\AppData\Local\Temp\claude\aiadj_rel_0806'
       r'\tfm2_ai_adjust\tfm2_ai_adjust.cfg')

raw = open(SRC, 'rb').read()
assert raw[:3] != b'\xef\xbb\xbf', 'BOM 이 있다 — 게임 파서가 파일을 통째로 무시한다'
t = raw.decode('utf-8')

lines = t.split('\n')
if lines and lines[0].startswith('# 테스트C'):
    lines[0] = '# tfm2_ai_adjust 릴리스 기본 설정 (게임 0.5.4) — 테스트C 와 같은 값'
out = '\n'.join(lines).encode('utf-8')
open(DST, 'wb').write(out)

print('원본  %s  %d bytes' % ('테스트C.cfg', len(raw)))
print('배치  %s  %d bytes' % ('tfm2_ai_adjust.cfg(스테이징)', len(out)))
print('BOM   첫3바이트 = %s' % open(DST, 'rb').read(3).hex())

# 값(키=값) 라인이 완전히 동일한지 대조 — 머리말 외에는 달라지면 안 된다
def kv(b):
    d = {}
    for ln in b.decode('utf-8').split('\n'):
        s = ln.strip()
        if s and not s.startswith('#') and '=' in s:
            k, v = s.split('=', 1)
            d[k.strip()] = v.strip()
    return d

a, b = kv(raw), kv(out)
print('\n활성 키 수  테스트C=%d  릴리스=%d' % (len(a), len(b)))
diff = [k for k in set(a) | set(b) if a.get(k) != b.get(k)]
print('값이 다른 키: %d개 %s' % (len(diff), diff[:10] if diff else '(없음 — 값 동일)'))
