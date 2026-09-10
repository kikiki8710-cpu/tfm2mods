#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""fieldall2.py — 함수 스코프를 지켜서 gep 오프셋 → store 를 잡는다.
   (fieldcodes.py 는 레지스터 이름을 파일 전역으로 매칭해 함수 간 오염이 있다.)
   gep 체인을 따라 루트 포인터 기준 누적 오프셋을 계산한다."""
import io, os, re, sys
if hasattr(sys.stdout,'reconfigure'): sys.stdout.reconfigure(encoding='utf-8',errors='replace')
IRDIRS=[r'C:\tfm2mods\_gaibc',r'C:\tfm2mods\_gcbc']
RE_GEP=re.compile(r'^\s*(%[\w.]+) = getelementptr\s+(?:inbounds\s+)?(?:nuw\s+)?(?:nusw\s+)?i8, ptr (%[\w.]+), i64 (-?\d+)')
RE_ST=re.compile(r'^\s*store (i8|i16|i32|i64) (-?\d+|%[\w.]+), ptr (%[\w.]+)')
RE_LD=re.compile(r'^\s*(%[\w.]+) = load (i8|i16|i32|i64), ptr (%[\w.]+)')
RE_DBG=re.compile(r'!dbg (![0-9]+)')
RE_DL=re.compile(r'^(![0-9]+) = !DILocation\(line: (\d+)[^\n]*?(?:scope: (![0-9]+))?(?:.*?inlinedAt: (![0-9]+))?\)')
RE_SCOPE=re.compile(r'^(![0-9]+) = (?:distinct )?!DI(?:Subprogram|LexicalBlock|LexicalBlockFile)\([^\n]*?file: (![0-9]+)')
RE_FILE=re.compile(r'^(![0-9]+) = !DIFile\(filename: "([^"]*)"')

def load_meta(lines):
    meta,sc2f,files={},{},{}
    for ln in lines:
        if not ln.startswith('!'): continue
        if '!DILocation' in ln:
            m=RE_DL.match(ln)
            if m: meta[m.group(1)]=(int(m.group(2)),m.group(3),m.group(4))
        elif '!DIFile(' in ln:
            m=RE_FILE.match(ln)
            if m: files[m.group(1)]=m.group(2)
        elif 'DISubprogram' in ln or 'DILexicalBlock' in ln:
            m=RE_SCOPE.match(ln)
            if m: sc2f[m.group(1)]=m.group(2)
    return meta,sc2f,files

def root(meta,mid):
    seen=set(); d=0
    while mid in meta and d<16:
        line,sc,inl=meta[mid]
        if not inl or inl in seen: return line,sc
        seen.add(mid); mid=inl; d+=1
    return (meta[mid][0],meta[mid][1]) if mid in meta else (None,None)

def funcs(lines):
    cur=None
    for i,ln in enumerate(lines):
        if ln.startswith('define'):
            cur=[i,ln]
        elif ln=='}' and cur:
            yield cur[0],i,cur[1]; cur=None

def scan(off,srcfilt=None,only=None,fnfilt=None,alltypes=False):
    out=[]
    for d in IRDIRS:
        if not os.path.isdir(d): continue
        for fn in sorted(os.listdir(d)):
            if not fn.endswith('.ll') or (only and fn!=only): continue
            lines=io.open(os.path.join(d,fn),encoding='utf-8',errors='replace').read().split('\n')
            meta,sc2f,files=load_meta(lines)
            if srcfilt and not any(srcfilt in v for v in files.values()): continue
            for s,e,dl in funcs(lines):
                if fnfilt and fnfilt not in dl: continue
                offs={}
                for i in range(s,e+1):
                    m=RE_GEP.match(lines[i])
                    if m:
                        base=m.group(2); delta=int(m.group(3))
                        if base in offs: offs[m.group(1)]=(offs[base][0],offs[base][1]+delta)
                        else: offs[m.group(1)]=(base,delta)
                for i in range(s,e+1):
                    m=RE_ST.match(lines[i])
                    if not m: continue
                    ty,val,ptr=m.groups()
                    if not alltypes and ty!='i8': continue
                    if ptr not in offs or offs[ptr][1]!=off: continue
                    dm=RE_DBG.search(lines[i]); sl=sc=None
                    if dm: sl,sc=root(meta,dm.group(1))
                    fname=None
                    if dm and dm.group(1) in meta:
                        f=sc2f.get(meta[dm.group(1)][1]); fname=files.get(f) if f else None
                    if srcfilt and (not fname or srcfilt not in fname): continue
                    out.append((fn,i+1,ty,val,sl,offs[ptr][0],dl[:120]))
    return out

if __name__=='__main__':
    a=sys.argv[1]; off=int(a,16) if a.lower().startswith('0x') else int(a)
    rest=sys.argv[2:]; src=only=fnf=None; allt=False
    while rest:
        t=rest.pop(0)
        if t=='--src': src=rest.pop(0)
        elif t=='--fn': fnf=rest.pop(0)
        elif t=='--all': allt=True
        elif t.endswith('.ll'): only=t
    rows=scan(off,src,only,fnf,allt)
    print('+%s(=%d): %d곳'%(hex(off),off,len(rows)))
    for fn,i,ty,val,sl,base,dl in rows:
        print('%s:%-7d src=%-6s %s %s  base=%s'%(fn,i,sl,ty,val,base))
