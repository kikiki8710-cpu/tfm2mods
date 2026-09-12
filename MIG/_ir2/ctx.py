# -*- coding: utf-8 -*-
import io,re,sys
sys.stdout.reconfigure(encoding='utf-8',errors='replace')
path=sys.argv[1]; a=int(sys.argv[2]); b=int(sys.argv[3])
import os
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
def L(d,dep=0):
    s=md.get(d,'')
    m=re.match(r'!DILocation\(line: (\d+)(?:, column: \d+)?, scope: !(\d+)(?:, inlinedAt: !(\d+))?',s)
    if not m: return '?'
    f,fn=sf(int(m.group(2)))
    r='%s:%s(%s)'%(f,m.group(1),(fn or '')[:26])
    if m.group(3) and dep<4: r+=' < '+L(int(m.group(3)),dep+1)
    return r
for k in range(a,b+1):
    l=txt[k-1]
    if '#dbg_declare' in l: continue
    m=re.search(r'!dbg !(\d+)',l)
    print('%-7d %-118s %s'%(k,l.strip()[:118],L(int(m.group(1))) if m else ''))
