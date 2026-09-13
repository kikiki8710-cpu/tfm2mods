import re,pickle
FN,S,md=pickle.load(open('fn.pkl','rb'))
def loc(i):
    s=md.get(i,'')
    m=re.match(r'(?:distinct )?!DILocation\(line: (\d+),(?: column: \d+,)? scope: !(\d+)(?:, inlinedAt: !(\d+))?',s)
    if not m: return None
    return int(m.group(1)),int(m.group(2)),(int(m.group(3)) if m.group(3) else None)
def subprog(scope):
    seen=set(); i=scope
    while i is not None and i not in seen:
        seen.add(i); s=md.get(i,'')
        m=re.search(r'DISubprogram\(name: "([^"]+)"',s)
        if m:
            f=re.search(r'file: !(\d+)',s); l=re.search(r'line: (\d+)',s)
            return m.group(1),(int(f.group(1)) if f else None),(int(l.group(1)) if l else None)
        m=re.search(r'scope: !(\d+)',s)
        i=int(m.group(1)) if m else None
    return None
def fname(fi):
    s=md.get(fi,'')
    m=re.search(r'DIFile\(filename: "([^"]+)"',s)
    return m.group(1) if m else None
def chain(dbg):
    out=[]; i=dbg
    while i is not None:
        lc=loc(i)
        if not lc: break
        line,scope,inl=lc
        sp=subprog(scope)
        out.append((line,sp[0] if sp else None,fname(sp[1]) if sp else None))
        i=inl
    return out
def dbg_of(line):
    m=re.search(r'!dbg !(\d+)',line)
    return int(m.group(1)) if m else None
def root(line):
    d=dbg_of(line)
    if d is None: return None
    c=chain(d)
    return c[-1] if c else None
