from common import *
import sys,re
want=set(int(x) for x in sys.argv[1:])
for k,ln in enumerate(FN):
    s=ln.rstrip()
    if s.strip().startswith('#dbg') : continue
    d=dbg_of(ln)
    if d is None and 'invoke' in s and k+1<len(FN): d=dbg_of(FN[k+1])
    if d is None: continue
    c=chain(d)
    if c and c[-1][0] in want:
        inner=c[0][0] if len(c)>1 else ''
        s2=re.sub(r', !(alias\.scope|noalias|noundef|range|nonnull|align|invariant\.load) ![^,]*','',s)
        print(f"{S+k} [{c[-1][0]}{('/'+str(inner)) if inner!='' else ''}] {s2.strip()[:170]}")
