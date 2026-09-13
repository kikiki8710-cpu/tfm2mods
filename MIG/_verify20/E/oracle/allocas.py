from stores import resolve, defs
from common import *
import re,collections
# alloca sizes
asz={}
for k,ln in enumerate(FN):
    m=re.match(r'\s*(%[\w.]+) = alloca (.*?), align',ln)
    if m: asz[m.group(1)]=m.group(2)
ops=collections.defaultdict(list)
for k,ln in enumerate(FN):
    s=ln.strip()
    m=re.match(r'store (.+?), ptr ([%\w.]+)',s)
    if m:
        b,o=resolve(m.group(2))
        if b.endswith('(alloca)'):
            r=root(ln); ops[(b,o)].append(('W',S+k,m.group(1)[:30],r[0] if r else None))
    m=re.match(r'%[\w.]+ = load (.+?), ptr ([%\w.]+)',s)
    if m:
        b,o=resolve(m.group(2))
        if b.endswith('(alloca)'):
            r=root(ln); ops[(b,o)].append(('R',S+k,m.group(1)[:10],r[0] if r else None))
big=[a for a,t in asz.items() if '280' in t]
print('280B allocas:',big)
for off in (0x0,0x58,0x60,0x78,0xa8,0xb8,0xf6,0xfc,0xfe,0xff,0x107,0x108):
    rows=[]
    for (b,o),v in ops.items():
        if o==off and b.split('(')[0] in big:
            rows+= [(b,)+x for x in v]
    print(hex(off), sorted(set((x[1],x[2],x[4]) for x in rows)))
