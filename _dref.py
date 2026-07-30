# -*- coding: utf-8 -*-
# _dref.py <exe> <rva> : 해당 함수 주소가 u64 로 등장하는 위치(vtable) + 그 위치를 lea 하는 코드
import sys, io, struct
sys.stdout=io.TextIOWrapper(sys.stdout.buffer,encoding="utf-8")
import capstone
EXE=sys.argv[1]; TG=int(sys.argv[2],16)
raw=open(EXE,'rb').read()
pe=struct.unpack_from("<I",raw,0x3c)[0]; nsec=struct.unpack_from("<H",raw,pe+6)[0]; opt=pe+24
IB=struct.unpack_from("<Q",raw,opt+24)[0]; sectab=opt+struct.unpack_from("<H",raw,pe+20)[0]
secs=[]
for i in range(nsec):
    o=sectab+i*40; nm=raw[o:o+8].rstrip(b"\0").decode(errors="replace")
    vsz,va,rsz,rr=struct.unpack_from("<IIII",raw,o+8); secs.append((nm,va,max(vsz,rsz),rr,rsz))
def roff(rva):
    for nm,va,sz,rr,rsz in secs:
        if va<=rva<va+sz: return rr+(rva-va)
def o2rva(off):
    for nm,va,sz,rr,rsz in secs:
        if rr<=off<rr+rsz: return va+(off-rr)
needle=(IB+TG).to_bytes(8,'little')
hits=[]
i=raw.find(needle)
while i>=0:
    r=o2rva(i)
    if r is not None: hits.append(r)
    i=raw.find(needle,i+1)
print("u64 refs to %#x:"%TG,[hex(h) for h in hits])
# 그 주소들을 lea 하는 코드 찾기 (rip-rel disp 로 정확히 맞는 명령 = 근사: .text 전역 스캔)
md=capstone.Cs(capstone.CS_ARCH_X86,CS_MODE:=capstone.CS_MODE_64); md.detail=True
txt=[s for s in secs if s[0]=='.text'][0]
_,tva,tsz,trr,trsz=txt
code=raw[trr:trr+trsz]
targets=set()
for h in hits:
    for d in range(0,0x60,8):
        targets.add(h-d)
found={}
# lea 는 48 8d XX ... : 전수 디스어셈은 느리니 rip-rel disp 역계산으로 후보 바이트 스캔
for h in sorted(targets):
    pass
# 대신: 각 hit 에 대해 disp = h - (site+7) 형태 가정하고 48 8d 0d/15/1d/... 패턴 스캔
import collections
res=collections.defaultdict(list)
for i in range(len(code)-7):
    if code[i]==0x48 and code[i+1]==0x8d and (code[i+2]&0xc7)==0x05:
        disp=int.from_bytes(code[i+3:i+7],'little',signed=True)
        t=tva+i+7+disp
        if t in targets: res[t].append(tva+i)
    elif code[i]==0x4c and code[i+1]==0x8d and (code[i+2]&0xc7)==0x05:
        disp=int.from_bytes(code[i+3:i+7],'little',signed=True)
        t=tva+i+7+disp
        if t in targets: res[t].append(tva+i)
for t,ss in sorted(res.items()):
    print("  vtable %#x  lea sites: %s"%(t,[hex(s) for s in ss[:12]]))
