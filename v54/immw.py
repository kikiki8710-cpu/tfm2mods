# -*- coding: utf-8 -*-
"""immw.py — 특정 구조체 오프셋에 **즉시값을 쓰는** 명령을 .text 전역에서 모아 '값↔의미' 표를 만든다.
  python immw.py 054 0x1528 [srcfilter]
mov qword/dword/byte [reg+disp32], imm 만 본다(레지스터 쓰기는 별도 표기).
"""
import sys, io, re, struct, bisect, os
sys.path.insert(0, r'C:\tfm2mods\v54')
from pe2 import load, BASE
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
D = r'C:\tfm2mods\v54'
ver=sys.argv[1]; disp=int(sys.argv[2],0); filt=(sys.argv[3].lower() if len(sys.argv)>3 else '')
e=load(ver)
rows=[]
for ln in io.open(os.path.join(D,'%s_srcmap2.tsv'%ver),encoding='utf-8'):
    s,en,src,l=ln.rstrip('\n').split('\t'); rows.append((int(s,16),int(en,16),src))
rows.sort(); ks=[r[0] for r in rows]
def src(r):
    i=bisect.bisect_right(ks,r)-1
    return rows[i][2] if i>=0 and rows[i][0]<=r<rows[i][1] else ''
_,tva,tvsz,tra,trsz=[s for s in e.sections if s[0]=='.text'][0]
body=e.raw[tra:tra+trsz]
dd=struct.pack('<i',disp)
out=[]
for rex in (b'', b'\x48', b'\x49', b'\x41'):
    for op,w,nm in ((0xc7,4,'qword' if rex in (b'\x48',b'\x49') else 'dword'), (0xc6,1,'byte')):
        forms=[bytes([op,m]) for m in range(0x80,0x88) if m!=0x84]
        forms += [bytes([op,0x84,sib]) for sib in (0x24,0x20,0x00,0x24)]
        for fm in set(forms):
            pre=rex+fm+dd
            for mm in re.finditer(re.escape(pre),body):
                o=mm.start()+len(pre)
                v=struct.unpack_from('<i' if w==4 else '<b', body, o)[0]
                a=tva+mm.start()
                s=src(a)
                if filt and filt not in s.lower(): continue
                out.append((a,nm,v,s,(pre+body[o:o+w]).hex()))
for a,nm,v,s,hx in sorted(out):
    print('%06x  mov %-5s [reg+0x%x], %-12d (0x%x)  %-24s %s'%(a,nm,disp,v,v&0xffffffff,hx,s[:70]))
print('-- %d건'%len(out))
