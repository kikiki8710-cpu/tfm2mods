# -*- coding: utf-8 -*-
# 0.5.4 -> 0.5.5 exe2exe skeleton-hash 매칭 엔진
import struct, pickle, bisect
OLD=r"C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.4\TeamfightManager2.exe"
NEW=r"C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.5\TeamfightManager2.exe"
class Img:
    def __init__(s, exe, pkl):
        d=open(exe,"rb").read(); s.raw=d
        pe=struct.unpack_from("<I",d,0x3c)[0]; nsec=struct.unpack_from("<H",d,pe+6)[0]; opt=pe+24
        sectab=opt+struct.unpack_from("<H",d,pe+20)[0]; s.secs=[]
        for i in range(nsec):
            o=sectab+i*40; nm=d[o:o+8].rstrip(b"\0").decode(errors="replace")
            vsz,va,rsz,rraw=struct.unpack_from("<IIII",d,o+8); s.secs.append((nm,va,max(vsz,rsz),rraw,rsz))
        P=pickle.load(open(pkl,"rb")); s.fn={(int(k,16) if isinstance(k,str) else k):v for k,v in P["idx"].items()}
        s.by_skel=P["by_skel"]; s.by_head=P.get("by_head",{})
        s.starts=sorted(s.fn)
    def roff(s,rva):
        for nm,va,vsz,rraw,rsz in s.secs:
            if va<=rva<va+vsz: return rraw+(rva-va)
    def read(s,rva,n):
        o=s.roff(rva); return None if o is None else s.raw[o:o+n]
    def owner(s,rva):
        i=bisect.bisect_right(s.starts,rva)-1
        if i<0: return None
        st=s.starts[i]; return st if rva<st+s.fn[st]["size"] else None
O=Img(OLD,r"C:\tfm2mods\_fnidx_054.pkl")
N=Img(NEW,r"C:\tfm2mods\_fnidx_055.pkl")

def match_fn(rva):
    """함수시작 rva를 0.5.5로 매칭. returns (result, new_rva, note)"""
    f=O.fn.get(rva)
    if not f: return ("NOT_FN_START", None, "0.5.4 함수시작 아님")
    sk=f["skel"]; cands=N.by_skel.get(sk,[])
    if len(cands)==1:
        nr=cands[0]; note=f"size {f['size']}->{N.fn[nr]['size']}"
        return ("UNIQUE", nr, note)
    if len(cands)==0:
        # head fallback
        hc=N.by_head.get(f["head"],[])
        return ("NONE", None, f"skel 후보0 / head후보 {len(hc)}")
    # MULTI: size로 필터
    same=[c for c in cands if N.fn[c]["size"]==f["size"]]
    if len(same)==1: return ("UNIQUE_bySize", same[0], f"skel {len(cands)}후보 size로 유일")
    return ("MULTI", cands, f"{len(cands)}후보")

def prologue(img, rva, n=14):
    b=img.read(rva,n); return b.hex() if b else None

def match_mid(site, orig_hex=None):
    """mid-func 바이트패치 site를 컨테이너-델타로 매핑 + orig 대조"""
    own=O.owner(site)
    if own is None: return ("NO_OWNER", None, "0.5.4 owner 없음")
    off=site-own; res,nown,note=match_fn(own)
    if res.startswith("UNIQUE"):
        nsite=nown+off
        out={"owner_old":own,"owner_new":nown,"off":off,"new_site":nsite}
        if orig_hex:
            ob=O.read(site,len(orig_hex)//2); nb=N.read(nsite,len(orig_hex)//2)
            out["old_orig"]=ob.hex() if ob else None
            out["new_bytes"]=nb.hex() if nb else None
            out["orig_match"]= (nb.hex()==orig_hex.lower()) if nb else False
        return ("OWNER_"+res, nsite, out)
    return ("OWNER_"+res, None, f"owner={hex(own)} off={hex(off)} {note}")
