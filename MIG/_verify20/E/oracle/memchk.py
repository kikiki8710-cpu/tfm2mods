from stores import resolve, defs
from common import *
import re,json,collections
sp=json.load(open(r'C:\tfm2mods\MIG\_spec\specs20_v3.json',encoding='utf-8'))['specs'][110]
acc=collections.defaultdict(set)   # (base,off) -> ops
gepoffs=collections.Counter()
for k,ln in enumerate(FN):
    s=ln.strip()
    m=re.match(r'%[\w.]+ = getelementptr (?:inbounds )?(?:nuw )?i8, ptr [%\w.]+, i64 (-?\d+)',s)
    if m: gepoffs[int(m.group(1))]+=1
    m=re.match(r'(%[\w.]+ = load .+?|store .+?), ptr ([%\w.]+)',s)
    if m:
        b,o=resolve(m.group(2)); acc[(b,o)].add('R' if s.startswith('%') else 'W')
BASE={'PlayerState':'%3','OperationData':'%4','LegacyPlanHandler':'%0'}
for i,r in enumerate(sp['mem']):
    off=int(str(r['offset']).split('[')[0].split(' ')[0],16) if str(r['offset']).startswith('0x') else None
    b=BASE.get(r['base'])
    if off is None: print(i,r['base'],r['offset'],'?'); continue
    if b:
        ops=acc.get((b,off),set())
        # also derived loads via gep in nested (e.g. team_plan sub-fields)
        print(i,r['base'],hex(off),r['dir'],'IR:',''.join(sorted(ops)) or ('gep-only' if gepoffs[off] else 'NONE'))
    else:
        print(i,r['base'],hex(off),r['dir'],'gep#',gepoffs[off])
