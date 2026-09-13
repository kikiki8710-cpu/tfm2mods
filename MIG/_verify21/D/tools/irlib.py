# -*- coding: utf-8 -*-
import io,re,sys,pickle,os
sys.stdout.reconfigure(encoding='utf-8',errors='replace')
LL=r'C:\tfm2mods\_gaibc\m13.ll'
FS,FE=14467,28943
CACHE='m13cache.pkl'
def load():
    if os.path.exists(CACHE):
        return pickle.load(open(CACHE,'rb'))
    lines=io.open(LL,encoding='utf-8',errors='replace').read().split('\n')
    md={}
    rx=re.compile(r'^!(\d+) = (.*)$')
    for ln in lines:
        if ln.startswith('!'):
            m=rx.match(ln)
            if m: md[int(m.group(1))]=m.group(2)
    fn=lines[FS-1:FE]  # index0 = line FS
    d=(lines,md)
    pickle.dump(d,open(CACHE,'wb'))
    return d
lines,md=load()
def diloc(i):
    s=md.get(i,'')
    if 'DILocation' not in s: return None
    l=re.search(r'line: (\d+)',s); sc=re.search(r'scope: !(\d+)',s); ia=re.search(r'inlinedAt: !(\d+)',s)
    return (int(l.group(1)) if l else 0, int(sc.group(1)) if sc else None, int(ia.group(1)) if ia else None)
def scopefile(sc):
    seen=set()
    i=sc
    while i is not None and i not in seen:
        seen.add(i); s=md.get(i,'')
        m=re.search(r'DISubprogram\(name: "([^"]+)"',s)
        if m:
            f=re.search(r'file: !(\d+)',s)
            fn=re.search(r'filename: "([^"]+)"',md.get(int(f.group(1)),'')) if f else None
            return m.group(1), (fn.group(1) if fn else '?')
        m=re.search(r'scope: !(\d+)',s)
        i=int(m.group(1)) if m else None
    return '?','?'
def chain(dbg):
    """returns list of (line, funcname, file) from innermost to root"""
    out=[]; i=dbg; seen=set()
    while i is not None and i not in seen:
        seen.add(i)
        d=diloc(i)
        if not d: break
        line,sc,ia=d
        nm,f=scopefile(sc)
        out.append((line,nm,f))
        i=ia
    return out
DBG=re.compile(r'!dbg !(\d+)')
def linechain(n):
    """n = original .ll line number (1-based)"""
    m=DBG.search(lines[n-1])
    if not m: return None
    return chain(int(m.group(1)))
def root(n):
    c=linechain(n)
    return c[-1][0] if c else None
def fmt(c):
    if not c: return 'nodbg'
    return '<'.join('%d'%x[0] if x[2].endswith('handler.rs') else '%d(%s:%s)'%(x[0],x[2].replace(chr(92),'/').split('/')[-1],x[1][:20]) for x in c)
