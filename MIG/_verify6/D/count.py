import json, io
d = json.load(io.open(r'C:\tfm2mods\MIG\_spec\specs20_v3.json', encoding='utf-8'))
tot = {}
gtot = [0]*6
for i, s in enumerate(d['specs']):
    c = [0]*6
    for arr in ('mem', 'consts', 'knobs'):
        for r in s.get(arr, []):
            ev = r.get('ev')
            if ev is None:
                ev = 0
            c[ev] += 1
            gtot[ev] += 1
    n = sum(c)
    ge4 = c[4]+c[5]
    print('%2d %-40s ev1=%d ev2=%d ev3=%d ev4=%d ev5=%d  tot=%d  ge4=%d (%.1f%%)' % (
        i, s['name'], c[1], c[2], c[3], c[4], c[5], n, ge4, 100.0*ge4/n if n else 0))
print('GLOBAL ev1=%d ev2=%d ev3=%d ev4=%d ev5=%d tot=%d ge4=%d' % (gtot[1], gtot[2], gtot[3], gtot[4], gtot[5], sum(gtot), gtot[4]+gtot[5]))
# batch totals
for name, rng in (('A', range(0,5)), ('B', range(5,10)), ('C', range(10,15)), ('D', range(15,20))):
    c = [0]*6
    for i in rng:
        for arr in ('mem','consts','knobs'):
            for r in d['specs'][i].get(arr, []):
                ev = r.get('ev') or 0
                c[ev] += 1
    n = sum(c); ge4 = c[4]+c[5]
    print('BATCH %s ev1=%d ev2=%d ev3=%d ev4=%d ev5=%d tot=%d ge4=%d (%.1f%%)' % (name, c[1],c[2],c[3],c[4],c[5],n,ge4,100.0*ge4/n if n else 0))
