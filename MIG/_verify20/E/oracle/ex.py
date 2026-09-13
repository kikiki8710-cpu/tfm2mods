import io,re,sys,json
P=r'C:\tfm2mods\_gaibc\m13.ll'
L=io.open(P,encoding='utf-8',errors='replace').read().split('\n')
S,E=34496,45209
md={}
rx=re.compile(r'^!(\d+) = (.*)$')
for ln in L:
    if ln.startswith('!'):
        m=rx.match(ln)
        if m: md[int(m.group(1))]=m.group(2)
def loc(i):
    s=md.get(i,'')
    m=re.match(r'!DILocation\(line: (\d+), column: (\d+), scope: !(\d+)(?:, inlinedAt: !(\d+))?',s)
    if not m: return None
    return int(m.group(1)),int(m.group(3)),(int(m.group(4)) if m.group(4) else None)
def subprog(scope):
    seen=set()
    i=scope
    while i is not None and i not in seen:
        seen.add(i); s=md.get(i,'')
        m=re.search(r'DISubprogram\(name: "([^"]+)"',s)
        if m:
            f=re.search(r'file: !(\d+)',s); l=re.search(r'line: (\d+)',s)
            lf=re.search(r'linkageName: "([^"]+)"',s)
            return m.group(1),(int(f.group(1)) if f else None),(int(l.group(1)) if l else None),(lf.group(1) if lf else None)
        m=re.search(r'scope: !(\d+)',s)
        i=int(m.group(1)) if m else None
    return None
def fname(fi):
    s=md.get(fi,'')
    m=re.search(r'DIFile\(filename: "([^"]+)"',s)
    return m.group(1) if m else None
def chain(dbg):
    out=[]
    i=dbg
    while i is not None:
        lc=loc(i)
        if not lc: break
        line,scope,inl=lc
        sp=subprog(scope)
        out.append((line,sp[0] if sp else None,fname(sp[1]) if sp else None))
        i=inl
    return out
json.dump({'md_count':len(md)},open('meta.json','w'))
import pickle
pickle.dump((L[S-1:E],S,md),open('fn.pkl','wb'))
print(len(L[S-1:E]))
