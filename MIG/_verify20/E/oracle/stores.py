from common import *
import re
# Build def map: %name -> (lineidx, text)
defs={}
for k,ln in enumerate(FN):
    m=re.match(r'\s*(%[\w.]+) = (.*)',ln)
    if m: defs[m.group(1)]=(k,m.group(2))
# resolve pointer to base + offset where base in {%0..%5, alloca}
def resolve(p,depth=0):
    if depth>30: return ('?',0)
    if p in ('%0','%1','%2','%3','%4','%5'): return (p,0)
    if p not in defs: return ('?',0)
    k,t=defs[p]
    m=re.match(r'getelementptr (?:inbounds )?(?:nuw )?(?:\(.*?\), )?i8, ptr ([%\w.]+), i64 (-?\d+)',t)
    if m:
        b,o=resolve(m.group(1),depth+1); return (b,o+int(m.group(2)))
    m=re.match(r'getelementptr (?:inbounds )?(?:nuw )?(?:\(.*?\), )?i8, ptr ([%\w.]+), i64 ([%\w.]+)',t)
    if m:
        b,o=resolve(m.group(1),depth+1); return (b+'+'+m.group(2) if b!='?' else '?',o)
    m=re.match(r'getelementptr (?:inbounds )?(?:nuw )?(\[.*?\]|\w+), ptr ([%\w.]+), i64 (-?\d+)(?:, i64 (-?\d+))?',t)
    if m:
        b,o=resolve(m.group(2),depth+1); return (b+'+gep('+m.group(1)+','+m.group(3)+(','+m.group(4) if m.group(4) else '')+')',o)
    if t.startswith('alloca'): return (p+'(alloca)',0)
    m=re.match(r'load ptr, ptr ([%\w.]+)',t)
    if m:
        b,o=resolve(m.group(1),depth+1); return ('*'+b+'+'+str(o),0)
    m=re.match(r'select .*?, ptr ([%\w.]+), ptr ([%\w.]+)',t)
    if m:
        return ('sel('+str(resolve(m.group(1),depth+1))+'|'+str(resolve(m.group(2),depth+1))+')',0)
    m=re.match(r'phi ptr (.*)',t)
    if m:
        return ('phi',0)
    return ('?:'+t[:40],0)
out=[]
for k,ln in enumerate(FN):
    s=ln.strip()
    m=re.match(r'store (.+?), ptr ([%\w.]+)(,.*)?$',s)
    if m:
        b,o=resolve(m.group(2))
        if b.startswith('%0') or b.startswith('*%0') or 'sel' in b or b=='phi' or b.startswith('%5') or b.startswith('*%'):
            r=root(ln)
            out.append((S+k,'store',b,hex(o),m.group(1)[:60],r))
    m=re.match(r'(?:tail )?call void @llvm\.mem(cpy|set|move)[^(]*\(ptr [^%]*?([%\w.]+), (?:ptr [^%]*?([%\w.]+)|i8 (\d+)), i64 (\d+)',s)
    if m:
        b,o=resolve(m.group(2))
        if b.startswith('%0') or b.startswith('*%0') or 'sel' in b or b=='phi' or b.startswith('%5') or b.startswith('*%'):
            src=m.group(3) or ('i8 '+m.group(4))
            sb=resolve(m.group(3)) if m.group(3) else src
            out.append((S+k,'mem'+m.group(1),b,hex(o),str(sb)+' n='+m.group(5),root(ln)))
if __name__=="__main__":
    for o in out: print(o)
