# -*- coding: utf-8 -*-
"""주어진 IR 줄이 속한 블록의 '지배 분기 사슬'을 역추적한다."""
import io,re,sys,os
from collections import defaultdict,deque
sys.stdout.reconfigure(encoding='utf-8',errors='replace')
path=sys.argv[1]; L=int(sys.argv[2]); depth=int(sys.argv[3]) if len(sys.argv)>3 else 12
for base in (r'C:\tfm2mods\_gaibc',r'C:\tfm2mods\_gcbc',r'C:\tfm2mods\_gvbc'):
    if os.path.exists(os.path.join(base,path)): path=os.path.join(base,path); break
txt=io.open(path,encoding='utf-8',errors='replace').read().split('\n')
md={}
for ln in txt:
    if ln.startswith('!'):
        m=re.match(r'^!(\d+) = (.*)$',ln)
        if m: md[int(m.group(1))]=m.group(2).rstrip()
RE_FILE=re.compile(r'file: !(\d+)'); RE_SCOPE=re.compile(r'scope: !(\d+)')
RE_DIFILE=re.compile(r'!DIFile\(filename: "([^"]*)"'); RE_SPNAME=re.compile(r'!DISubprogram\(name: "([^"]*)"')
fc={}
def sf(sid,d=0):
    if sid in fc: return fc[sid]
    if d>20: return ('?',None)
    s=md.get(sid,''); fn=None
    m=RE_SPNAME.search(s)
    if m: fn=m.group(1)
    mf=RE_FILE.search(s)
    if mf:
        mm=RE_DIFILE.search(md.get(int(mf.group(1)),''))
        if mm: fc[sid]=(mm.group(1).split('\\')[-1],fn); return fc[sid]
    ms=RE_SCOPE.search(s)
    if ms:
        r=sf(int(ms.group(1)),d+1)
        if fn and not r[1]: r=(r[0],fn)
        fc[sid]=r; return r
    fc[sid]=('?',fn); return fc[sid]
def loc(d,dep=0):
    s=md.get(d,'')
    m=re.match(r'!DILocation\(line: (\d+)(?:, column: \d+)?, scope: !(\d+)(?:, inlinedAt: !(\d+))?',s)
    if not m: return '?'
    f,fn=sf(int(m.group(2)))
    r='%s:%s(%s)'%(f,m.group(1),(fn or '')[:28])
    if m.group(3) and dep<4: r+=' < '+loc(int(m.group(3)),dep+1)
    return r
i=L-1
st=i
while not txt[st].startswith('define'): st-=1
en=i
while txt[en]!='}': en+=1
body=txt[st:en+1]; base=st
print('FN %d..%d  %s'%(st+1,en+1,re.search(r'@([\w.$]+)\(',txt[st]).group(1)[-80:]))
RE_LBL=re.compile(r'^(\d+):')
blocks={}; cur='entry'; curstart=1
order=[]
for k,l in enumerate(body):
    m=RE_LBL.match(l)
    if m:
        blocks[cur]=(curstart,k-1); order.append(cur); cur=m.group(1); curstart=k+1
blocks[cur]=(curstart,len(body)-1); order.append(cur)
def blk_of(idx):
    for b,(a,z) in blocks.items():
        if a<=idx<=z: return b
    return None
def term(b):
    a,z=blocks[b]
    j=z
    while j>=a and not body[j].strip(): j-=1
    k=j
    while k>a-1:
        s=body[k].strip()
        if s.startswith(('br ','switch ','ret ','unreachable','invoke ','indirectbr','cleanupret','catchswitch')): break
        k-=1
    return '\n'.join(body[k:j+1])
pred=defaultdict(list)
for b in blocks:
    for m in re.finditer(r'\blabel %(\d+)',term(b)): pred[m.group(1)].append(b)
def defof(v):
    for k,l in enumerate(body):
        if l.strip().startswith('%'+v+' ='): return (k+base+1,l.strip())
    return None
target=blk_of(i-base)
print('target block %%%s'%target)
seen=set(); q=deque([(target,0)])
while q:
    b,d=q.popleft()
    if b in seen or d>depth: continue
    seen.add(b)
    for p in pred.get(b,[]):
        t=term(p)
        if t.startswith('br i1') or t.startswith('switch'):
            m=re.search(r'!dbg !(\d+)',t)
            print('  %s<- %%%s : %s   [%s]'%('  '*d,p,t.replace('\n',' ')[:130], loc(int(m.group(1))) if m else ''))
            mv=re.match(r'br i1 %(\d+)',t)
            if mv:
                dd=defof(mv.group(1))
                if dd: print('     %s   cond %%%s = %s   @%d'%('  '*d,mv.group(1),dd[1][:130],dd[0]))
        q.append((p,d+1))
