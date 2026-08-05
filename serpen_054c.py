# -*- coding: utf-8 -*-
# 콜사이트 순서/컨테이너 대조 — clone family 구별용
import sys, io, struct, pickle, collections, bisect
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8", errors="replace")
class Img:
    def __init__(self, path, pkl, tag):
        self.tag=tag; d=open(path,"rb").read(); self.raw=d
        pe=struct.unpack_from("<I",d,0x3c)[0]; nsec=struct.unpack_from("<H",d,pe+6)[0]; opt=pe+24
        sectab=opt+struct.unpack_from("<H",d,pe+20)[0]; self.secs=[]
        for i in range(nsec):
            o=sectab+i*40; nm=d[o:o+8].rstrip(b"\0").decode(errors="replace")
            vsz,va,rsz,rraw=struct.unpack_from("<IIII",d,o+8); self.secs.append((nm,va,max(vsz,rsz),rraw,rsz))
        P=pickle.load(open(pkl,"rb"))["idx"]
        self.fn={(int(k,16) if isinstance(k,str) else k):v for k,v in P.items()}
        self.starts=sorted(self.fn)
    def roff(self,rva):
        for nm,va,vsz,rraw,rsz in self.secs:
            if va<=rva<va+vsz: return rraw+(rva-va)
    def read(self,rva,n):
        o=self.roff(rva); return None if o is None else self.raw[o:o+n]
    def text(self):
        for nm,va,vsz,rraw,rsz in self.secs:
            if nm==".text": return va,vsz,rraw,rsz
    def owner(self,rva):
        i=bisect.bisect_right(self.starts,rva)-1
        if i<0: return None
        s=self.starts[i]; return s if rva<s+self.fn[s]["size"] else None
    def sites(self):
        va,vsz,rraw,rsz=self.text(); blob=self.raw[rraw:rraw+rsz]
        self.by_tgt=collections.defaultdict(list); i=0; n=len(blob)
        while True:
            i=blob.find(b"\xe8",i)
            if i<0 or i+5>n: break
            rel=struct.unpack_from("<i",blob,i+1)[0]; site=va+i; tgt=site+5+rel
            if tgt in self.fn: self.by_tgt[tgt].append(site)
            i+=1
    def data_refs(self, rva):
        """.rdata/.data 에서 이 rva를 가리키는 qword(=vtable 슬롯) 찾기"""
        want=struct.pack("<Q", 0x140000000+rva); out=[]
        for nm,va,vsz,rraw,rsz in self.secs:
            if nm not in (".rdata",".data"): continue
            blob=self.raw[rraw:rraw+rsz]; i=0
            while True:
                i=blob.find(want,i)
                if i<0: break
                out.append((nm, va+i)); i+=8
        return out

O=Img(r"C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.3\TeamfightManager2.exe", r"C:\tfm2mods\_fnidx_053.pkl","053")
N=Img(r"C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.4\TeamfightManager2.exe", r"C:\tfm2mods\_fnidx_054.pkl","054")
O.sites(); N.sites()
import json
for tag, img, rvas in json.loads(sys.argv[1]):
    im = O if img=="O" else N
    for r in rvas:
        r=int(r,16)
        cs=im.by_tgt.get(r,[])
        print(f"[{tag}] {img} {r:#x} size={im.fn.get(r,{}).get('size')} 콜사이트 {len(cs)}: "
              + ", ".join(f"{s:#x}(in {im.owner(s):#x} +{s-im.owner(s):#x})" if im.owner(s) else f"{s:#x}(?)" for s in cs[:8]))
        dr=im.data_refs(r)
        print(f"      데이터참조(vtable 등) {len(dr)}: {[(n,hex(a)) for n,a in dr[:8]]}")
