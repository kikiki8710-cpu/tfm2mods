# -*- coding: utf-8 -*-
"""xr.py — 범용 xref: 대상 RVA 를 가리키는 (1)call/jmp rel32 (2)rip-rel lea/mov (3)절대 8바이트 데이터(vtable/함수포인터 테이블)
  python xr.py 054 e8c020
출력의 [DATA] 행은 rva + 그 rva 가 속한 섹션. vtable 이면 슬롯 인덱스 추정을 위해 앞뒤 8슬롯도 찍는다.
"""
import sys, struct, re, bisect
sys.path.insert(0, r'C:\tfm2mods\v54')
from pe2 import load, BASE
import io, os
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
D = r'C:\tfm2mods\v54'

def srcmap2(ver):
    rows=[]
    for ln in io.open(os.path.join(D,'%s_srcmap2.tsv'%ver),encoding='utf-8'):
        s,e,src,l=ln.rstrip('\n').split('\t'); rows.append((int(s,16),int(e,16),src))
    rows.sort(); return rows

def main():
    ver, tgt = sys.argv[1], int(sys.argv[2],16)
    e=load(ver); sm=srcmap2(ver); ks=[r[0] for r in sm]
    def src(rva):
        i=bisect.bisect_right(ks,rva)-1
        return sm[i][2] if i>=0 and sm[i][0]<=rva<sm[i][1] else ''
    def sec_of(rva):
        for n,va,vsz,ra,rsz in e.sections:
            if va<=rva<va+vsz: return n,ra+(rva-va)
        return '?',None
    _,tva,tvsz,tra,trsz=[s for s in e.sections if s[0]=='.text'][0]
    body=e.raw[tra:tra+trsz]
    print('== rel32 (call/jmp) ==')
    n=0
    for o in range(len(body)-5):
        b=body[o]
        if b not in (0xE8,0xE9): continue
        d=struct.unpack_from('<i',body,o+1)[0]
        if tva+o+5+d==tgt:
            a=tva+o; f=e.func_of(a)
            print('  %06x %s  fn %s  %s'%(a,'call' if b==0xE8 else 'jmp',('%06x'%f[0]) if f else '?',src(a)[:70])); n+=1
    print('  -- %d건'%n)
    print('== rip-rel (disp 가 명령 끝) ==')
    n=0
    for o in range(len(body)-4):
        d=struct.unpack_from('<i',body,o)[0]
        if tva+o+4+d==tgt:
            a=tva+o; f=e.func_of(a-3)
            print('  %06x(disp)  ctx=%s  fn %s  %s'%(a,body[o-8:o+4].hex(),('%06x'%f[0]) if f else '?',src(a)[:70])); n+=1
    print('  -- %d건'%n)
    print('== 절대 8바이트 데이터 ==')
    pat=struct.pack('<Q',BASE+tgt); n=0
    for m in re.finditer(re.escape(pat),e.raw):
        off=m.start(); rva=None; sec='?'
        for nm,va,vsz,ra,rsz in e.sections:
            if ra<=off<ra+rsz: rva=va+(off-ra); sec=nm
        print('  file %08x  rva %s  sec %s'%(off,('%06x'%rva) if rva else '?',sec))
        if rva is not None:
            for k in range(-4,5):
                v=struct.unpack_from('<Q',e.raw,off+k*8)[0]
                print('      [%+d] %016x %s'%(k,v,('-> %06x %s'%(v-BASE,src(v-BASE)[:50])) if BASE<=v<BASE+0x10000000 else ''))
        n+=1
    print('  -- %d건'%n)

main()
