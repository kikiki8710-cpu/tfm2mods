# -*- coding: utf-8 -*-
import io,re,sys
from collections import defaultdict, deque
sys.stdout.reconfigure(encoding='utf-8',errors='replace')
P=r'C:\tfm2mods\_gaibc\m13.ll'
txt=io.open(P,encoding='utf-8',errors='replace').read().split('\n')
md={}
for ln in txt:
    if ln.startswith('!'):
        m=re.match(r'^!(\d+) = (.*)$',ln)
        if m: md[int(m.group(1))]=m.group(2)
RE_LOC=re.compile(r'!DILocation\(line: (\d+)(?:, column: (\d+))?, scope: !(\d+)(?:, inlinedAt: !(\d+))?')
RE_FILE=re.compile(r'file: !(\d+)'); RE_SCOPE=re.compile(r'scope: !(\d+)')
RE_DIFILE=re.compile(r'!DIFile\(filename: "([^"]*)"'); RE_SPNAME=re.compile(r'!DISubprogram\(name: "([^"]*)"')
fc={}
def scope_file(sid,d=0):
    if sid in fc: return fc[sid]
    if d>20: return ('?',None)
    s=md.get(sid,''); fn=None
    m=RE_SPNAME.search(s)
    if m: fn=m.group(1)
    mf=RE_FILE.search(s)
    if mf:
        mm=RE_DIFILE.search(md.get(int(mf.group(1)),''))
        if mm:
            fc[sid]=(mm.group(1),fn); return fc[sid]
    ms=RE_SCOPE.search(s)
    if ms:
        r=scope_file(int(ms.group(1)),d+1)
        if fn and not r[1]: r=(r[0],fn)
        fc[sid]=r; return r
    fc[sid]=('?',fn); return fc[sid]
lc={}
def loc(did):
    if did in lc: return lc[did]
    out=[]; cur=did; g=0
    while cur is not None and g<40:
        g+=1; s=md.get(cur,''); m=RE_LOC.match(s)
        if not m: break
        f,fn=scope_file(int(m.group(3)))
        out.append((f.split('\\')[-1],int(m.group(1)),fn)); cur=int(m.group(4)) if m.group(4) else None
    lc[did]=out; return out
def fmtloc(did):
    ch=loc(did)
    return ' | '.join('%s:%d(%s)'%(f,l,(fn or '')[:34]) for f,l,fn in ch)

target=29802-1
st=target
while not txt[st].startswith('define'): st-=1
en=st
while txt[en]!='}': en+=1
body=txt[st:en+1]; base=st
RE_LBL=re.compile(r'^(\d+):')
blocks={}; cur='entry'; curstart=1
for i,l in enumerate(body):
    m=RE_LBL.match(l)
    if m:
        blocks[cur]=(curstart,i-1); cur=m.group(1); curstart=i+1
blocks[cur]=(curstart,len(body)-1)
RE_BR=re.compile(r'\blabel %(\d+)')
succ=defaultdict(list); pred=defaultdict(list)
for b,(a,z) in blocks.items():
    j=z
    while j>=a and not body[j].strip(): j-=1
    k=j
    while k>a-1:
        s=body[k].strip()
        if s.startswith(('br ','switch ','ret ','unreachable','invoke ','indirectbr')): break
        k-=1
    seg='\n'.join(body[k:j+1])
    for m in RE_BR.finditer(seg):
        succ[b].append(m.group(1)); pred[m.group(1)].append(b)
sw=[]; k=target-base
while True:
    sw.append(body[k])
    if ']' in body[k]: break
    k+=1
segs='\n'.join(sw)
cases=re.findall(r'i8 (-?\d+), label %(\d+)',segs)
targets=sorted(set(t for _,t in cases))
owner=defaultdict(set)
for t in targets:
    seen=set(); dq=deque([t])
    while dq:
        b=dq.popleft()
        if b in seen: continue
        seen.add(b)
        for s in succ.get(b,[]):
            if s not in seen: dq.append(s)
    for b in seen: owner[b].add(t)

want=sys.argv[1]
excl=[b for b in blocks if owner.get(b)=={want}]
# order by position in file
excl.sort(key=lambda b: blocks[b][0])
for b in excl:
    a,z=blocks[b]
    print('--- %%%s  (preds=%s succ=%s)  [ll %d..%d]'%(b,','.join(pred.get(b,[])[:8]),','.join(succ.get(b,[])),a+base+1,z+base+1))
    for j in range(a,z+1):
        l=body[j]
        if not l.strip(): continue
        m=re.search(r'!dbg !(\d+)',l)
        tag=fmtloc(int(m.group(1))) if m else ''
        print('  %-6d %-150s %s'%(j+base+1,l.strip()[:150],tag))
