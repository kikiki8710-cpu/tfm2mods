import json, io, sys, os
P = r'C:\tfm2mods\MIG\_spec\specs20_v3.json'
d = json.load(io.open(P, encoding='utf-8'))
out = io.open(r'C:\tfm2mods\MIG\_verify5\A\specA.txt', 'w', encoding='utf-8')
for i in range(0, 5):
    s = d['specs'][i]
    out.write('=' * 100 + '\n')
    out.write('### specs[%d] %s / %s\n' % (i, s.get('id'), s.get('name')))
    out.write(json.dumps(s, ensure_ascii=False, indent=1))
    out.write('\n')
out.write('=' * 100 + '\n### SHARED\n')
out.write(json.dumps(d['shared'], ensure_ascii=False, indent=1))
out.close()
print('ok')
