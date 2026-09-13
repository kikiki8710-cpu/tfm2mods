# -*- coding: utf-8 -*-
import irlib,re,sys,collections
L=irlib.lines
FS,FE=irlib.FS,irlib.FE
R0,R1=24343,28943   # my range
GEP=re.compile(r'^\s*(%\d+) = getelementptr (?:inbounds )?(?:nuw )?i8, ptr (%\d+|@\S+), i64 (-?\d+)')
GEPV=re.compile(r'^\s*(%\d+) = getelementptr (?:inbounds )?(?:nuw )?([^,]+), ptr (%\d+)(.*)')
PHI=re.compile(r'^\s*(%\d+) = phi ptr (.*)')
SEL=re.compile(r'^\s*(%\d+) = select i1 %\d+, ptr (%\d+), ptr (%\d+)')
STORE=re.compile(r'^\s*store (.+?), ptr (%\d+|@\S+)')
CALL=re.compile(r'^\s*(?:(%\d+) = )?(?:tail )?(?:call|invoke)\b.*?@(\S+?)\((.*)\)')
ARG=re.compile(r'ptr [^,%@]*(%\d+|@[\w.$]+)')
# derive: name -> (base, offset or None)
der={}
def resolve(v, depth=0):
    """returns (rootbase, offset|None) ; rootbase '%0' for self"""
    if v=='%0': return ('%0',0)
    if v in der:
        b,o=der[v]
        if b is None: return (None,None)
        rb,ro=resolve(b,depth+1) if depth<60 else (None,None)
        if rb is None: return (None,None)
        if ro is None or o is None: return (rb,None)
        return (rb,ro+o)
    return (v,0) if v.startswith('%') and v in ARGS else (None,None)
ARGS={'%0','%1','%2','%3','%4','%5','%6'}
# collect defs over the whole function
for n in range(FS,FE+1):
    l=L[n-1]
    m=GEP.match(l)
    if m:
        der[m.group(1)]=(m.group(2),int(m.group(3))); continue
    m=GEPV.match(l)
    if m:
        # struct gep with unknown offset -> mark offset None
        der[m.group(1)]=(m.group(3),None); continue
    m=PHI.match(l)
    if m:
        srcs=re.findall(r'\[ (%\d+|@\S+|null), %\d+ \]', m.group(2))
        # if all sources resolve to same self offset use it; else record multi
        rs=set()
        for s in srcs:
            if s=='null': continue
            rs.add(resolve(s) if s in der or s in ARGS else (s,0))
        der[m.group(1)]=('PHI:'+','.join(sorted(map(str,rs))),None)
        continue
    m=SEL.match(l)
    if m:
        der[m.group(1)]=('SEL:%s,%s'%(resolve(m.group(2)),resolve(m.group(3))),None); continue
def isself(v):
    b,o=resolve(v)
    if b=='%0': return True,o
    if b and (b.startswith('PHI:') or b.startswith('SEL:')) and "'%0'" in b: return True,b
    return False,None
def rootof(n):
    c=irlib.linechain(n)
    if c is None and n+1<=FE: c=irlib.linechain(n+1)
    return c
out=[]
for n in range(R0,R1+1):
    l=L[n-1]
    m=STORE.match(l)
    if m:
        ok,o=isself(m.group(2))
        if ok: out.append((n,'store',o,m.group(1)[:60],''))
        continue
    m=CALL.match(l)
    if m:
        callee=m.group(2); args=m.group(3)
        argl=ARG.findall(args)
        selfargs=[]
        for a in argl:
            ok,o=isself(a)
            if ok: selfargs.append((a,o))
        if selfargs:
            out.append((n,'call',selfargs,callee[:90],''))
import json
for n,k,o,x,_ in out:
    c=rootof(n)
    print(n, k, o, '|', irlib.fmt(c), '|', x)
