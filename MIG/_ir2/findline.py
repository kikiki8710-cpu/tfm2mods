# -*- coding: utf-8 -*-
import io,re,sys,glob,os
sys.stdout.reconfigure(encoding='utf-8',errors='replace')
srcname=sys.argv[1]; targets=set(int(x) for x in sys.argv[2].split(','))
dirs=sys.argv[3] if len(sys.argv)>3 else r'C:\tfm2mods\_gaibc'
RE_FILE=re.compile(r'file: !(\d+)'); RE_SCOPE=re.compile(r'scope: !(\d+)')
RE_DIFILE=re.compile(r'!DIFile\(filename: "([^"]*)"'); RE_SPNAME=re.compile(r'!DISubprogram\(name: "([^"]*)"')
for path in sorted(glob.glob(os.path.join(dirs,'m*.ll')))+sorted(glob.glob(os.path.join(dirs,'g*.ll'))):
    txt=io.open(path,encoding='utf-8',errors='replace').read().split('\n')
    md={}
    for ln in txt:
        if ln.startswith('!'):
            m=re.match(r'^!(\d+) = (.*)$',ln)
            if m: md[int(m.group(1))]=m.group(2).rstrip()
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
    hit=set()
    for k,v in md.items():
        if v.startswith('!DILocation'):
            m=re.match(r'!DILocation\(line: (\d+)(?:, column: \d+)?, scope: !(\d+)',v)
            if m and int(m.group(1)) in targets:
                f,fn=sf(int(m.group(2)))
                if f==srcname: hit.add(k)
    if not hit: continue
    for i,l in enumerate(txt):
        m=re.search(r'!dbg !(\d+)',l)
        if m and int(m.group(1)) in hit:
            print('%s:%d  %s'%(os.path.basename(path),i+1,l.strip()[:150]))
