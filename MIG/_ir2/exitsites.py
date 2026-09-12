# -*- coding: utf-8 -*-
import io,re,sys
sys.stdout.reconfigure(encoding='utf-8',errors='replace')
RE_LOC=re.compile(r'!DILocation\(line: (\d+)(?:, column: (\d+))?, scope: !(\d+)(?:, inlinedAt: !(\d+))?')
RE_FILE=re.compile(r'file: !(\d+)'); RE_SCOPE=re.compile(r'scope: !(\d+)')
RE_DIFILE=re.compile(r'!DIFile\(filename: "([^"]*)"'); RE_SPNAME=re.compile(r'!DISubprogram\(name: "([^"]*)"')
def go(path, hits):
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
            out.append('%s:%s(%s)'%(f,m.group(1),(fn or '')[:45]))
            cur=int(m.group(4)) if m.group(4) else None
        return ' | '.join(out)
    for h in hits:
        i=h-1
        st=i
        while not txt[st].startswith('define'): st-=1
        mdef=re.search(r'@([\w.$]+)\(',txt[st])
        print('=== %s:%d  in define@%d  %s'%(path[-7:],h,st+1,mdef.group(1)[-70:] if mdef else '?'))
        for k in range(i-1,min(i+6,len(txt))):
            l=txt[k]
            m=re.search(r'!dbg !(\d+)',l)
            print('   %-7d %-120s %s'%(k+1,l.strip()[:120],chain(int(m.group(1))) if m else ''))
go(r'C:\tfm2mods\_gaibc\m13.ll',[10214,11311,20523,24234,32324,33003])
go(r'C:\tfm2mods\_gaibc\m14.ll',[55857])
