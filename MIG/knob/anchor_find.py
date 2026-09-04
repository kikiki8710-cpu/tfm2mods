# -*- coding: utf-8 -*-
"""git 각 리비전의 테이블 주소가 **어느 exe 에서 짝이 맞는지** 대조 — 진짜 앵커 찾기."""
import io, os, re, sys, struct, subprocess
sys.stdout.reconfigure(encoding='utf-8', errors='replace')
ANA=r'C:\Users\jungs\Desktop\claude\tfm2'
CUR=(r"C:\Program Files (x86)\Steam\steamapps\common"
     r"\Teamfight Manager2\TeamfightManager2.exe")
class Exe:
    def __init__(s,p):
        d=s.d=open(p,'rb').read()
        e=struct.unpack_from('<I',d,0x3c)[0]
        n=struct.unpack_from('<H',d,e+6)[0]
        ss=e+24+struct.unpack_from('<H',d,e+20)[0]
        s.secs=[]
        for i in range(n):
            o=ss+i*40
            va=struct.unpack_from('<I',d,o+12)[0]
            vsz=struct.unpack_from('<I',d,o+8)[0]
            rsz=struct.unpack_from('<I',d,o+16)[0]
            pr=struct.unpack_from('<I',d,o+20)[0]
            s.secs.append((va,max(vsz,rsz),pr))
    def rd(s,r,n):
        for va,sz,pr in s.secs:
            if va<=r<va+sz: return s.d[pr+(r-va):pr+(r-va)+n]
VERS=['0.4.10','0.4.13','0.4.14','0.5.0','0.5.1','0.5.2','0.5.3','0.5.4','0.5.5','0.5.6','0.5.7']
E={}
for v in VERS:
    p=os.path.join(ANA,'tfm2_'+v,'TeamfightManager2.exe')
    if os.path.isfile(p): E[v]=Exe(p)
E['0.5.8']=Exe(CUR)
WID={'PE_CAP':(4,150),'PE_STG':(4,180),'TH_LEA':(4,32000),'TH_CAP':(4,150),
     'PATH_STEP640':(4,640),'PATH_STEP896':(4,896),'PATH_RISK1281':(4,1281),'PATH_HEUR':(1,7)}
ENT=re.compile(r'\(\s*(0x[0-9a-fA-F]+)\s*,\s*&\[([^\]]*)\]\s*,\s*(\d+)\s*\)')
def tables(txt):
    out={}
    for m in re.finditer(r'static\s+(\w+)\s*:\s*\[\(usize,\s*&\[u8\],\s*usize\);\s*(\d+)\]\s*=\s*\[(.*?)\n\s*\];', txt, re.S):
        if m.group(1) in WID:
            out[m.group(1)]=[(int(a,16), [int(x,16) for x in re.findall(r'0x[0-9a-fA-F]+',p)], int(o))
                             for a,p,o in ENT.findall(m.group(3))]
    return out
def score(tb, ex):
    w,orig=WID[tb[0]] if False else (0,0)
    return 0
revs = subprocess.run(['git','-C',r'C:\tfm2mods','log','--format=%h %ad','--date=short','--all',
                       '--','tfm2_ai_adjust/src/detour.rs'], capture_output=True, text=True).stdout.split('\n')
revs=[r.split()[0] for r in revs if r.strip()]
print("detour.rs 리비전 %d개 — 앞뒤로 훑어 테이블 스냅샷의 앵커를 찾는다\n" % len(revs))
best=None
seen=set()
for rv in revs:
    txt = subprocess.run(['git','-C',r'C:\tfm2mods','show',rv+':tfm2_ai_adjust/src/detour.rs'],
                         capture_output=True, text=True, encoding='utf-8', errors='replace').stdout
    T=tables(txt)
    if not T: continue
    key=tuple(sorted((k, tuple(a for a,_,_ in v)) for k,v in T.items()))
    if key in seen: continue
    seen.add(key)
    tot=sum(len(v) for v in T.values())
    row=[]
    for v,ex in E.items():
        okc=0
        for nm,ents in T.items():
            w,orig=WID[nm]
            for a,pre,off in ents:
                b=ex.rd(a, off+w+4)
                if not b or len(b)<off+w or list(b[:len(pre)])!=pre: continue
                cur=b[off] if w==1 else struct.unpack('<I',b[off:off+4])[0]
                if cur==orig: okc+=1
        row.append((okc,v))
    row.sort(reverse=True)
    print("  %s  총 %3d  최고적중 %s(%d) / 2위 %s(%d)" % (rv, tot, row[0][1], row[0][0], row[1][1], row[1][0]))
