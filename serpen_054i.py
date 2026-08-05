# -*- coding: utf-8 -*-
# string-xref: .rdata 문자열 위치 → lea rip-rel 참조 사이트 → 그 직후 call 타깃 집계
import sys, io, struct, collections, re, json
sys.path.insert(0, r"C:\tfm2mods")
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8", errors="replace")
from s54lib import O, Nw
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
md = Cs(CS_ARCH_X86, CS_MODE_64)

def find_str(img, s):
    pat = s.encode(); out=[]
    for nm,va,vsz,rraw,rsz in img.secs:
        if nm not in (".rdata",".data"): continue
        blob=img.raw[rraw:rraw+rsz]; i=0
        while True:
            i=blob.find(pat,i)
            if i<0: break
            out.append(va+i); i+=1
    return out

def lea_sites(img, targets):
    """.text 전체를 선형 디스어셈하지 않고, lea rip-rel 인코딩을 역산해 스캔"""
    va,vsz,rraw,rsz = img.text()
    blob = img.raw[rraw:rraw+rsz]
    tset=set(targets); hits=[]
    # 48 8d XX(mod=00,rm=101) disp32  → 7바이트, 또는 4c 8d
    for i in range(len(blob)-7):
        b0=blob[i]
        if b0 not in (0x48,0x4c,0x49,0x4d): continue
        if blob[i+1]!=0x8d: continue
        modrm=blob[i+2]
        if (modrm & 0xc7)!=0x05: continue
        disp=struct.unpack_from("<i",blob,i+3)[0]
        tgt = va+i+7+disp
        if tgt in tset: hits.append((va+i, tgt))
    return hits

def next_call(img, site, span=0x60):
    b=img.read(site, span); out=[]
    for ins in md.disasm(b, site):
        if ins.mnemonic=="call" and ins.op_str.startswith("0x"):
            return int(ins.op_str,16)
    return None

for tag, which, s in json.loads(sys.argv[1]):
    img = O() if which=="O" else Nw()
    locs = find_str(img, s)
    print("="*80); print(f"[{tag}] {which} '{s}' → .rdata {len(locs)}곳 {[hex(x) for x in locs[:6]]}")
    hits = lea_sites(img, locs)
    cnt=collections.Counter(); owners=collections.Counter()
    for site,tgt in hits:
        c=next_call(img,site)
        if c is not None and c in img.fn: cnt[c]+=1
        w=img.owner(site)
        if w: owners[w]+=1
    print(f"   lea 참조 {len(hits)}곳")
    print(f"   직후 call 타깃 top: {[(hex(k),v) for k,v in cnt.most_common(6)]}")
    print(f"   참조 소유함수 top: {[(hex(k),v) for k,v in owners.most_common(6)]}")
