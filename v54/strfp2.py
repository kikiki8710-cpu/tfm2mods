# -*- coding: utf-8 -*-
"""strfp2.py — 확장 문자열지문 매칭. 48 8d / 4c 8d / 0f 10 / 0f 28 rip-rel 전부 스캔."""
import sys,io,re,struct,os,pickle,bisect
sys.stdout=io.TextIOWrapper(sys.stdout.buffer,encoding='utf-8')
import pefile
PAT=re.compile(rb'(?:\x48\x8d|\x4c\x8d)[\x05\x0d\x15\x1d\x25\x2d\x35\x3d]',re.S)
PAT2=re.compile(rb'(?:\x0f\x10|\x0f\x28)[\x05\x0d\x15\x1d\x25\x2d\x35\x3d]',re.S)
AOK=re.compile(rb'[ -~]{6,}')
P={'053':r'C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.3\TeamfightManager2.exe',
   '054':r'C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.4\TeamfightManager2.exe'}
class E:
    def __init__(s,tag):
        pe=pefile.PE(P[tag],fast_load=True); s.tag=tag; s.d=pe.__data__
        sec={x.Name.rstrip(b'\0'):x for x in pe.sections}
        pd=sec[b'.pdata']; raw=s.d[pd.PointerToRawData:pd.PointerToRawData+pd.SizeOfRawData]
        s.fn=[]
        for i in range(0,len(raw)-11,12):
            b,e,_=struct.unpack_from('<III',raw,i)
            if b==0: break
            s.fn.append((b,e))
        s.fn.sort(); s.st=[f[0] for f in s.fn]
        t=sec[b'.text']; s.tva,s.toff,s.tsz=t.VirtualAddress,t.PointerToRawData,t.SizeOfRawData
        r=sec[b'.rdata']; s.rva0,s.roff,s.rsz=r.VirtualAddress,r.PointerToRawData,r.SizeOfRawData
    def own(s,a):
        i=bisect.bisect_right(s.st,a)-1
        return s.fn[i] if i>=0 and s.fn[i][0]<=a<s.fn[i][1] else None
    def strat(s,rva,n=64):
        if not (s.rva0<=rva<s.rva0+s.rsz): return None
        b=s.d[s.roff+(rva-s.rva0):s.roff+(rva-s.rva0)+n]
        m=AOK.match(b); return m.group(0)[:48] if m else None
    def index(s):
        c='C:/tfm2mods/v54/_sfp2_%s.pkl'%s.tag
        if os.path.exists(c): return pickle.load(open(c,'rb'))
        text=s.d[s.toff:s.toff+s.tsz]; idx={}
        for pat,ln in ((PAT,7),(PAT2,7)):
            for m in pat.finditer(text):
                p=m.start()
                if p+ln>len(text): continue
                disp=struct.unpack_from('<i',text,p+3)[0]
                site=s.tva+p; tgt=site+ln+disp
                ss=s.strat(tgt)
                if not ss: continue
                f=s.own(site)
                if f: idx.setdefault(f[0],set()).add(ss)
        pickle.dump(idx,open(c,'wb')); return idx
o,n=E('053'),E('054')
oi,ni=o.index(),n.index()
print('지문보유 053 %d / 054 %d'%(len(oi),len(ni)))
inv={}
for f,ss in ni.items():
    for x in ss: inv.setdefault(x,set()).add(f)
for a in [int(x,16) for x in sys.argv[1:]]:
    f=o.own(a); S=oi.get(f[0]) if f else None
    if not S: print('%06x fn%s 지문없음'%(a,'%06x'%f[0] if f else '?')); continue
    sc={}
    for x in S:
        for g in inv.get(x,()): sc[g]=sc.get(g,0)+1
    best=sorted(sc.items(),key=lambda kv:-kv[1])[:4]
    print('%06x fn%06x 지문%d → %s'%(a,f[0],len(S),
        ', '.join('%06x(%d/%d)'%(g,c,len(S)) for g,c in best)))
