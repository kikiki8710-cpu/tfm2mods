# -*- coding: utf-8 -*-
"""테스트C 멈춤 원인 이분탐색.

용의자 = **기본값(default.txt)과 달랐던 키만.** 기본값과 같은 값은 패치해도 같은 값을 덮으므로
동작이 바뀌지 않는다(빈 cfg 로 멀쩡했던 것이 그 근거).

사용: python bisect.py <살릴 그룹 파일 | 'half1' | 'half2' | 'all' | 'none'>
  살아있는 키만 남기고 나머지 용의자는 주석 처리한다. 비용의자(기본값과 같은 키)는 항상 그대로.
결과는 v54/bisect_state.txt 에 기록해 어디까지 좁혔는지 남긴다."""
import sys, io, os, json
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

MOD = ('C:/Program Files (x86)/Steam/steamapps/common/Teamfight Manager2'
       '/mods/tfm2_ai_adjust')
BASE = MOD + '/config/테스트C.cfg'   # ★현재 상태(-1 되돌림 반영분)에서 출발한다
DEF = MOD + '/config/default.txt'


def kv(p):
    d = {}
    for ln in io.open(p, encoding='utf-8'):
        s = ln.strip()
        if s and not s.startswith('#') and '=' in s:
            k, v = [x.strip() for x in s.split('=', 1)]
            d[k] = v
    return d


cur, dd = kv(BASE), kv(DEF)
# 용의자 = 기본값과 다른 키 (기준 없는 키 = 클래스별 개별값 등도 용의자)
susp = sorted(k for k, v in cur.items() if dd.get(k) != v)

arg = sys.argv[1] if len(sys.argv) > 1 else 'half1'
if arg == 'all':
    keep = set(susp)
elif arg == 'none':
    keep = set()
elif arg == 'half1':
    keep = set(susp[:len(susp) // 2])
elif arg == 'half2':
    keep = set(susp[len(susp) // 2:])
elif os.path.exists(arg):
    keep = set(json.load(io.open(arg, encoding='utf-8')))
else:
    keep = set(a for a in arg.split(','))

out = []
off = []
for ln in io.open(BASE, encoding='utf-8'):
    s = ln.strip()
    if s and not s.startswith('#') and '=' in s:
        k = s.split('=', 1)[0].strip()
        if k in susp and k not in keep:
            out.append('# [이분탐색 비활성] ' + s + '\n'); off.append(k); continue
    out.append(ln)
body = ''.join(out)
for p in [MOD + '/tfm2_ai_adjust.cfg']:
    io.open(p, 'w', encoding='utf-8', newline='\n').write(body)

io.open('C:/tfm2mods/v54/bisect_state.txt', 'a', encoding='utf-8').write(
    '[%s] 용의자 %d · 살림 %d · 끔 %d\n  살림: %s\n' % (arg, len(susp), len(keep), len(off), ','.join(sorted(keep))))
print('용의자 %d개 (기본값과 다른 키)' % len(susp))
print('  이번 판에 살린 키 = %d개' % len(keep))
print('  끈 키 = %d개' % len(off))
print('\n살린 키:')
ks = sorted(keep)
for i in range(0, len(ks), 5):
    print('   ' + '  '.join(ks[i:i + 5]))
print('\n첫3바이트 =', open(MOD + '/tfm2_ai_adjust.cfg', 'rb').read(3).hex())
