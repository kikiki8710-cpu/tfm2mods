# -*- coding: utf-8 -*-
import struct, pickle, collections, bisect
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
    def sec(self,rva):
        for nm,va,vsz,rraw,rsz in self.secs:
            if va<=rva<va+vsz: return nm
        return "?"
    def read(self,rva,n):
        o=self.roff(rva); return None if o is None else self.raw[o:o+n]
    def body(self,rva):
        s=self.fn.get(rva,{}).get("size"); return None if not s else self.read(rva,s)
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
        return self.by_tgt
    def callseq(self, cont):
        n=self.fn[cont]["size"]; b=self.read(cont,n); out=[]; i=0
        while True:
            i=b.find(b"\xe8",i)
            if i<0 or i+5>n: break
            rel=struct.unpack_from("<i",b,i+1)[0]; site=cont+i; tgt=site+5+rel
            if tgt in self.fn: out.append((i,tgt,self.fn[tgt]["size"]))
            i+=1
        return out
    def data_refs(self,rva):
        want=struct.pack("<Q",0x140000000+rva); out=[]
        for nm,va,vsz,rraw,rsz in self.secs:
            if nm not in (".rdata",".data"): continue
            blob=self.raw[rraw:rraw+rsz]; i=0
            while True:
                i=blob.find(want,i)
                if i<0: break
                out.append((nm,va+i)); i+=8
        return out
    def find_bytes(self, pat, secname=".text"):
        out=[]
        for nm,va,vsz,rraw,rsz in self.secs:
            if nm!=secname: continue
            blob=self.raw[rraw:rraw+rsz]; i=0
            while True:
                i=blob.find(pat,i)
                if i<0: break
                out.append(va+i); i+=1
        return out
def O():
    return Img(r"C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.3\TeamfightManager2.exe", r"C:\tfm2mods\_fnidx_053.pkl","053")
def Nw():
    return Img(r"C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.4\TeamfightManager2.exe", r"C:\tfm2mods\_fnidx_054.pkl","054")
