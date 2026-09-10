#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""fieldall.py — 오프셋에 store 하는 i8 **전부**(상수 + 레지스터). fieldcodes.py 확장."""
import io, os, re, sys
from collections import defaultdict
if hasattr(sys.stdout, 'reconfigure'):
    sys.stdout.reconfigure(encoding='utf-8', errors='replace')
IRDIRS = [r'C:\tfm2mods\_gaibc', r'C:\tfm2mods\_gcbc', r'C:\tfm2mods\_gvbc']
RE_GEP = re.compile(r'^\s*(%\w+) = getelementptr[^\n]*?i64 (\d+)(?:$|,|\s)')
RE_ST = re.compile(r'^\s*store i8 (-?\d+|%[\w.]+), ptr (%[\w.]+)[^\n]*?(?:!dbg (![0-9]+))?\s*$')
RE_DBG = re.compile(r'^(![0-9]+) = !DILocation\(line: (\d+)[^\n]*?(?:scope: (![0-9]+))?(?:.*?inlinedAt: (![0-9]+))?\)')
RE_SCOPE = re.compile(r'^(![0-9]+) = (?:distinct )?!DI(?:Subprogram|LexicalBlock|LexicalBlockFile)\([^\n]*?file: (![0-9]+)')
RE_FILE = re.compile(r'^(![0-9]+) = !DIFile\(filename: "([^"]*)"')

def dbg_root(meta, mid, depth=0):
    seen=set()
    while mid and mid in meta and depth<12:
        line,scope,inl = meta[mid]
        if not inl or inl in seen: return line
        seen.add(mid); mid=inl; depth+=1
    return meta[mid][0] if mid in meta else None

def scan(off, only=None, src=None):
    rows=[]
    for d in IRDIRS:
        if not os.path.isdir(d): continue
        for fn in sorted(os.listdir(d)):
            if not fn.endswith('.ll') or (only and fn!=only): continue
            text=io.open(os.path.join(d,fn),encoding='utf-8',errors='replace').read()
            lines=text.split('\n')
            meta,sc2file,files={},{},{}
            for ln in lines:
                if not ln.startswith('!'): continue
                if '!DILocation' in ln:
                    m=RE_DBG.match(ln)
                    if m: meta[m.group(1)]=(int(m.group(2)),m.group(3),m.group(4))
                elif '!DIFile(' in ln:
                    m=RE_FILE.match(ln)
                    if m: files[m.group(1)]=m.group(2)
                elif 'DISubprogram' in ln or 'DILexicalBlock' in ln:
                    m=RE_SCOPE.match(ln)
                    if m: sc2file[m.group(1)]=m.group(2)
            if src and not any(src in v for v in files.values()): continue
            gep={}
            for i,ln in enumerate(lines):
                m=RE_GEP.match(ln)
                if m and int(m.group(2))==off: gep[m.group(1)]=i
            if not gep: continue
            for i,ln in enumerate(lines):
                m=RE_ST.match(ln)
                if not m: continue
                val,reg,dbgid=m.group(1),m.group(2),m.group(3)
                if reg not in gep: continue
                srcline=dbg_root(meta,dbgid) if dbgid else None
                fname=None
                if dbgid and dbgid in meta:
                    sc=meta[dbgid][1]
                    f=sc2file.get(sc)
                    fname=files.get(f) if f else None
                if src and (not fname or src not in fname): continue
                rows.append((fn,i+1,val,srcline,fname))
    return rows

if __name__=='__main__':
    a=sys.argv[1]; off=int(a,16) if a.lower().startswith('0x') else int(a)
    only=src=None; rest=sys.argv[2:]
    while rest:
        t=rest.pop(0)
        if t=='--src' and rest: src=rest.pop(0)
        elif t.endswith('.ll'): only=t
    rows=scan(off,only,src)
    print('+%s(=%d): store %d곳' % (hex(off),off,len(rows)))
    for fn,i,val,sl,fname in sorted(rows,key=lambda r:(r[0],r[1])):
        print('%s:%-7d src=%-6s val=%s' % (fn,i,sl,val))
