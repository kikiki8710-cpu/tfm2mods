# -*- coding: utf-8 -*-
import os,re,sys
sys.stdout.reconfigure(encoding='utf-8',errors='replace')
SDK=r'C:\tfm2mods\sdk_058\mod-sdk\deps'
def rmeta_path(c):
    for x in os.listdir(SDK):
        if x.startswith('lib'+c+'-') and x.endswith('.rmeta'): return os.path.join(SDK,x)
crate=sys.argv[1]
d=open(rmeta_path(crate),'rb').read()
# find all .rs path strings: printable run ending in .rs
RUN=re.compile(rb'[\x20-\x7e]{4,}\.rs')
seen=[]
for m in RUN.finditer(d):
    seen.append((m.start(),m.end(),m.group(0)))
print('count',len(seen))
for s,e,g in seen:
    print('%08x-%08x  %s'%(s,e,g.decode('ascii','replace')))
