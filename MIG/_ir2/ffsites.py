# -*- coding: utf-8 -*-
import io,re,sys,os,glob
sys.stdout.reconfigure(encoding='utf-8',errors='replace')
RE_LOC=re.compile(r'!DILocation\(line: (\d+)(?:, column: (\d+))?, scope: !(\d+)(?:, inlinedAt: !(\d+))?')
RE_FILE=re.compile(r'file: !(\d+)'); RE_SCOPE=re.compile(r'scope: !(\d+)')
RE_DIFILE=re.compile(r'!DIFile\(filename: "([^"]*)"'); RE_SPNAME=re.compile(r'!DISubprogram\(name: "([^"]*)"')
def analyze(path, pat, extra=None):
    txt=io.open(path,encoding='utf-8',errors='replace').read().split('\n')
    md={}
    for ln in txt:
        if ln.startswith('!'):
            m=re.match(r'^!(\d+) = (.*)$',ln)
            if m: md[int(m.group(1))]=m.group(2)
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
    def chain(did):
        out=[];cur=did;g=0
        while cur is not None and g<40:
            g+=1;s=md.get(cur,'');m=RE_LOC.match(s)
            if not m: break
            f,fn=sf(int(m.group(3)))
            out.append('%s:%s(%s)'%(f,m.group(1),(fn or '')[:40]))
            cur=int(m.group(4)) if m.group(4) else None
        return ' | '.join(out)
    rx=re.compile(pat)
    for i,l in enumerate(txt):
        if rx.search(l):
            m=re.search(r'!dbg !(\d+)',l)
            print('%s:%d  %s'%(os.path.basename(path),i+1,chain(int(m.group(1))) if m else ''))
            m2=re.search(r'ff_note_battle_swap\(([^)]*)\)',l)
            if m2: print('        args: %s'%m2.group(1)[:200])
            else: print('        %s'%l.strip()[:200])
for p in sorted(glob.glob(r'C:\tfm2mods\_gaibc\m*.ll')):
    analyze(p, r'ff_note_battle_swap\(ptr')
