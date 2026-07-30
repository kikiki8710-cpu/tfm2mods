# -*- coding: utf-8 -*-
# _jtscan.py <exe> <lo> <hi> : 전 섹션에서 (base+i32) 가 [lo,hi) 에 들어가는 i32 배열(>=8개) 탐색
import sys, io, struct
sys.stdout=io.TextIOWrapper(sys.stdout.buffer,encoding="utf-8")
EXE=sys.argv[1]; raw=open(EXE,'rb').read()
pe=struct.unpack_from("<I",raw,0x3c)[0]; nsec=struct.unpack_from("<H",raw,pe+6)[0]; opt=pe+24
IB=struct.unpack_from("<Q",raw,opt+24)[0]; sectab=opt+struct.unpack_from("<H",raw,pe+20)[0]
secs=[]
for i in range(nsec):
    o=sectab+i*40; nm=raw[o:o+8].rstrip(b"\0").decode(errors="replace")
    vsz,va,rsz,rr=struct.unpack_from("<IIII",raw,o+8); secs.append((nm,va,max(vsz,rsz),rr,rsz))
lo=int(sys.argv[2],16); hi=int(sys.argv[3],16)
for nm,va,sz,rr,rsz in secs:
    if nm not in ('.rdata','.text','.data'): continue
    buf=raw[rr:rr+rsz]
    i=0; N=len(buf)-4
    run=None
    while i<N:
        # 후보: 시작 base 를 자기 위치로 가정하는 방식은 base 미지 → base 를 lo..hi 범위 만족하는 것으로 역산 불가.
        i+=4
    # 대신 base 를 4바이트 정렬 후보로 전수: 너무 큼. 다른 방법: i32 배열의 값 분포가 좁게 뭉쳐있음.
    # base 가 테이블 시작주소일 가능성이 높다(Rust/MSVC 패턴) → 각 정렬위치를 base 로 가정해 연속 엔트리 검사
    i=0
    while i < N:
        base=va+i
        cnt=0
        while True:
            o=i+cnt*4
            if o+4>len(buf): break
            v=int.from_bytes(buf[o:o+4],'little',signed=True)
            t=base+v
            if lo<=t<hi: cnt+=1
            else: break
        if cnt>=8:
            print("%s table %#x  n=%d"%(nm,base,cnt))
            i+=cnt*4
        else:
            i+=4
