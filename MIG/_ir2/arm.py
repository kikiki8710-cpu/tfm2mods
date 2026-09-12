# -*- coding: utf-8 -*-
import io,re,sys,os,json
from collections import defaultdict, deque
sys.stdout.reconfigure(encoding='utf-8',errors='replace')

P=r'C:\tfm2mods\_gaibc\m13.ll'
txt=io.open(P,encoding='utf-8',errors='replace').read().split('\n')
N=len(txt)

# --- metadata ---
RE_MD=re.compile(r'^!(\d+) = (.*)$')
md={}
for ln in txt:
    if ln.startswith('!'):
        m=RE_MD.match(ln)
        if m: md[int(m.group(1))]=m.group(2)

RE_LOC=re.compile(r'!DILocation\(line: (\d+)(?:, column: (\d+))?, scope: !(\d+)(?:, inlinedAt: !(\d+))?')
RE_FILE=re.compile(r'file: !(\d+)')
RE_SCOPE=re.compile(r'scope: !(\d+)')
RE_DIFILE=re.compile(r'!DIFile\(filename: "([^"]*)"')
RE_SPNAME=re.compile(r'!DISubprogram\(name: "([^"]*)"')
RE_LINKNAME=re.compile(r'linkageName: "([^"]*)"')

filecache={}
def scope_file(sid, depth=0):
    if sid in filecache: return filecache[sid]
    if depth>20: return ('?','?')
    s=md.get(sid,'')
    fn=None
    m=RE_SPNAME.search(s)
    if m: fn=m.group(1)
    mf=RE_FILE.search(s)
    if mf:
        f=md.get(int(mf.group(1)),'')
        mm=RE_DIFILE.search(f)
        if mm:
            r=(mm.group(1), fn)
            filecache[sid]=r
            return r
    ms=RE_SCOPE.search(s)
    if ms:
        r=scope_file(int(ms.group(1)),depth+1)
        if fn and not r[1]: r=(r[0],fn)
        filecache[sid]=r
        return r
    filecache[sid]=('?',fn)
    return filecache[sid]

loccache={}
def loc(did):
    """return chain list of (file, line, subprogname) from innermost to root"""
    if did in loccache: return loccache[did]
    out=[]
    cur=did; guard=0
    while cur is not None and guard<40:
        guard+=1
        s=md.get(cur,'')
        m=RE_LOC.match(s)
        if not m: break
        line=int(m.group(1)); sc=int(m.group(3))
        f,fn=scope_file(sc)
        out.append((f,line,fn))
        cur=int(m.group(4)) if m.group(4) else None
    loccache[did]=out
    return out

# --- find function containing line 29802 (1-based) ---
target=29802-1
st=target
while st>=0 and not txt[st].startswith('define'): st-=1
en=st
while en<N and txt[en]!='}': en+=1
print('FN lines %d..%d'%(st+1,en+1))
print(txt[st][:300])

body=txt[st:en+1]
base=st  # 0-based offset

# parse blocks
RE_LBL=re.compile(r'^(\d+):')
blocks={}   # label -> (startidx, endidx) within body
order=[]
cur=None; curstart=None
# entry block label = implicit; use 'entry'
cur='entry'; curstart=1
for i,l in enumerate(body):
    m=RE_LBL.match(l)
    if m:
        if cur is not None:
            blocks[cur]=(curstart,i-1); order.append(cur)
        cur=m.group(1); curstart=i+1
if cur is not None:
    blocks[cur]=(curstart,len(body)-1); order.append(cur)

RE_BR=re.compile(r'\blabel %(\d+)')
succ=defaultdict(list)
for b,(a,z) in blocks.items():
    # terminator = last non-empty line
    for j in range(z,a-1,-1):
        l=body[j].strip()
        if not l: continue
        break
    # collect all label refs in terminator (switch spans multiple lines)
    # find terminator start: search back for line starting with br/switch/ret/unreachable/invoke
    k=j
    while k>a-1:
        s=body[k].strip()
        if s.startswith(('br ','switch ','ret ','unreachable','invoke ','indirectbr')): break
        k-=1
    seg='\n'.join(body[k:j+1])
    for m in RE_BR.finditer(seg):
        succ[b].append(m.group(1))
print('blocks=%d'%len(blocks))

# switch cases
sw_i=29802-1-base
seg=[]
k=sw_i
while True:
    seg.append(body[k])
    if ']' in body[k]: break
    k+=1
segs='\n'.join(seg)
default=re.search(r'switch i8 %\d+, label %(\d+)',segs).group(1)
cases=re.findall(r'i8 (-?\d+), label %(\d+)',segs)
print('default=%s cases=%d'%(default,len(cases)))
tgt2vals=defaultdict(list)
for v,l in cases: tgt2vals[l].append(int(v))

# reachability: which case targets reach each block
targets=sorted(set(t for _,t in cases))
owner=defaultdict(set)
for t in targets:
    seen=set()
    dq=deque([t])
    while dq:
        b=dq.popleft()
        if b in seen: continue
        seen.add(b)
        for s in succ.get(b,[]):
            if s not in seen: dq.append(s)
    for b in seen: owner[b].add(t)

# chat.rs lines per block
def blk_lines(b):
    a,z=blocks[b]
    res=set()
    for j in range(a,z+1):
        for m in re.finditer(r'!dbg !(\d+)',body[j]):
            ch=loc(int(m.group(1)))
            for f,ln,fn in ch:
                if f.endswith('chat.rs'): res.add(ln)
    return res

VAR={}
for t in targets:
    excl=[b for b in blocks if owner.get(b)=={t}]
    lines=set()
    for b in excl: lines|=blk_lines(b)
    vals=sorted(tgt2vals[t])
    VAR[t]=(vals,sorted(lines),len(excl))

for t in sorted(VAR,key=lambda x:VAR[x][1][0] if VAR[x][1] else 99999):
    vals,lines,nb=VAR[t]
    print('tgt %%%-5s vals=%-14s nblk=%-4d lines=%s'%(t,vals,nb,(str(lines[:6])+'..'+str(lines[-3:])) if len(lines)>9 else lines))
