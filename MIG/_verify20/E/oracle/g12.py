from common import *
import re,json,collections
d=json.load(open(r'C:\tfm2mods\MIG\_spec\specs20_v3.json',encoding='utf-8'))
sp=d['specs'][110]
# index: literal -> list of (irline, rootline, chain, instr)
lit=collections.defaultdict(list)
rxlit=re.compile(r'(?<![\w.%!@#-])(-?\d+)(?![\w.])')
for k,ln in enumerate(FN):
    s=ln.strip()
    if s.startswith('#dbg') or s.startswith(';') or not s: continue
    if 'getelementptr' in s or s.startswith('call void @llvm.lifetime') : continue
    dbgsrc=s
    if 'invoke' in s and not re.search(r'!dbg',s):
        dbgsrc=FN[k+1]
    if not re.search(r'!dbg',dbgsrc): continue
    # strip metadata tail
    body=re.sub(r', !\w+ !\d+.*$','',s)
    body=re.sub(r'!dbg !\d+','',body)
    # remove align N, dereferenceable(N), sret([N x i8]), i64 type tokens? keep numbers after 'i64 ' etc
    body=re.sub(r'align \d+','',body); body=re.sub(r'dereferenceable\(\d+\)','',body); body=re.sub(r'\[\d+ x i8\]','',body)
    body=re.sub(r'range\(i\d+ -?\d+, -?\d+\)','',body)
    body=re.sub(r'%[\w.]+','',body); body=re.sub(r'i(1|8|16|24|32|64)\b','',body); body=re.sub(r'label \d+','',body)
    nums=set(int(x) for x in rxlit.findall(body))
    if not nums: continue
    c=chain(dbg_of(dbgsrc))
    if not c: continue
    rootl=c[-1][0]
    for n in nums:
        lit[n].append((S+k,rootl,[(x[0],x[1]) for x in c],s[:120]))
res=[]
for i,c in enumerate(sp['consts']):
    v=int(c['value']); sl=int(c['src_line'])
    occ=lit.get(v,[])
    roots=sorted(set(o[1] for o in occ))
    ok=sl in roots
    res.append((i,v,sl,ok,roots[:40],len(occ)))
    print(i,v,sl,'OK' if ok else 'MISS',roots[:40],len(occ))
json.dump({str(k):[(a,b,c) for a,b,c,_ in v] for k,v in lit.items()},open('lit.json','w'))
