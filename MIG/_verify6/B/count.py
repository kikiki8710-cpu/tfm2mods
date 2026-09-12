# -*- coding: utf-8 -*-
u"""BRIEF_FACTS §1 집계 스코프(mem+consts+knobs)로 직접 재집계 — 브리핑 검증용."""
import json, io, sys
sys.stdout.reconfigure(encoding='utf-8', errors='replace')
d = json.load(io.open(r'C:\tfm2mods\MIG\_spec\specs20_v3.json', encoding='utf-8'))
B = {'A': range(0, 5), 'B': range(5, 10), 'C': range(10, 15), 'D': range(15, 20)}
print('%-4s %-5s %4s %4s %4s %4s %4s %5s %7s' % ('배치', '함수', 'ev1', 'ev2', 'ev3', 'ev4', 'ev5', '계', 'ev>=4'))
tot = [0] * 6
for b, rng in B.items():
    c = [0] * 6
    for i in rng:
        s = d['specs'][i]
        for f in ('mem', 'consts', 'knobs'):
            for r in s.get(f) or []:
                e = int(r.get('ev') or 4)
                c[e] += 1
    n = sum(c)
    print('%-4s %-5s %4d %4d %4d %4d %4d %5d %6.1f%%' % (b, '%02d~%02d' % (min(rng), max(rng)),
          c[1], c[2], c[3], c[4], c[5], n, 100.0 * (c[4] + c[5]) / n))
    for k in range(6):
        tot[k] += c[k]
n = sum(tot)
print('%-4s %-5s %4d %4d %4d %4d %4d %5d %6.1f%%' % ('계', '00~19', tot[1], tot[2], tot[3], tot[4], tot[5], n,
      100.0 * (tot[4] + tot[5]) / n))
print()
print('--- 배치B 함수별 ev>=4 행수 (mem/consts/knobs) + sig.vis ---')
for i in range(5, 10):
    s = d['specs'][i]
    per = {}
    hi = 0
    for f in ('mem', 'consts', 'knobs'):
        k = 0
        for r in s.get(f) or []:
            if int(r.get('ev') or 4) >= 4:
                k += 1
        per[f] = '%d/%d' % (k, len(s.get(f) or []))
        hi += k
    print('  %02d %-34s ev>=4 %2d  (mem %s, consts %s, knobs %s)  vis=%s' % (
        i, s['name'], hi, per['mem'], per['consts'], per['knobs'], s['sig'].get('vis')))
print()
print('--- open/notes 전량 ---')
for i in range(0, 20):
    s = d['specs'][i]
    for o in s.get('open') or []:
        print('  open %02d [%s] ev%s %s' % (i, o.get('class'), o.get('ev'), (o.get('q') or '')[:110]))
for i in range(0, 20):
    s = d['specs'][i]
    for o in s.get('notes') or []:
        t = o if isinstance(o, str) else json.dumps(o, ensure_ascii=False)
        print('  note %02d %s' % (i, t[:110]))
