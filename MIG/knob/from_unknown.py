# -*- coding: utf-8 -*-
"""★`imm_unknown.txt` 를 **표 누락 명세**로 삼아 orig_table 에 행을 추가한다.

가드가 거부한 자리를 런타임이 `(rva, off, w)` 로 정확히 적어 준다 — 소스 파서가 못 읽는
형태(`p!(base+mv, mpre, mpre.len(), 4, ..)` 처럼 prefix·off 가 변수)도 여기엔 실측값이 남는다.
⟹ 정적 파서의 사각지대를 런타임이 메워 주는 구조. expect_orig 는 exe 실측으로 채운다.
"""
import io, re, sys, struct
sys.path.insert(0, r'C:\tfm2mods\MIG')
sys.stdout.reconfigure(encoding='utf-8', errors='replace')
import pefile, mig_verify as MV
pe=pefile.PE(MV.GAME_EXE, fast_load=True)
S=[(s.VirtualAddress,s.VirtualAddress+s.Misc_VirtualSize,s.get_data()) for s in pe.sections]
def rd(r,n):
    for a,b,d in S:
        if a<=r<b: return d[r-a:r-a+n]
U=r'C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\mods\tfm2_ai_adjust\imm_unknown.txt'
want=[]
for line in io.open(U,encoding='utf-8',errors='replace'):
    m=re.match(r'(0x[0-9a-fA-F]+)\s+off=(\d+)\s+w=(\d+)', line.strip())
    if m: want.append((int(m.group(1),16), int(m.group(2)), int(m.group(3))))
print("가드가 거부한 자리 %d개" % len(want))

P=r'C:\tfm2mods\tfm2_ai_adjust\src\orig_table.rs'
src=io.open(P,encoding='utf-8').read()
ROW=re.compile(r'^(\s*)\((0x[0-9a-fA-F]+),\s*(\d+),\s*(\d+),\s*(\d+)\),(.*)$', re.M)
ms=list(ROW.finditer(src))
tab=[dict(rva=int(m.group(2),16),off=int(m.group(3)),w=int(m.group(4)),
          orig=int(m.group(5)),tail=m.group(6).rstrip()) for m in ms]
head,tailtxt=src[:ms[0].start()],src[ms[-1].end():]
keys={(r['rva'],r['off'],r['w']) for r in tab}
add=0
for rva,off,w in want:
    if (rva,off,w) in keys:
        print("   이미 있음 0x%x" % rva); continue
    b=rd(rva, off+w+4)
    if b is None or len(b)<off+w:
        print("   ★읽기 실패 0x%x" % rva); continue
    cur=(b[off] if w==1 else struct.unpack('<H',b[off:off+2])[0] if w==2
         else struct.unpack('<I',b[off:off+4])[0] if w==4
         else struct.unpack('<Q',b[off:off+8])[0])
    tab.append(dict(rva=rva,off=off,w=w,orig=cur,
                    tail='   // ★0.5.8 인게임 검증에서 imm_unknown 으로 드러난 누락(재핀 짝의 mov 쪽)'))
    keys.add((rva,off,w)); add+=1
    print("   추가 0x%-8x off=%d w=%d orig=%d" % (rva,off,w,cur))
tab.sort(key=lambda r:(r['rva'],r['off'],r['w']))
body='\n'.join("    (0x%x, %d, %d, %d),%s"%(r['rva'],r['off'],r['w'],r['orig'],r['tail']) for r in tab)
io.open(P+'.bak_unknown','w',encoding='utf-8',newline='').write(src)
io.open(P,'w',encoding='utf-8',newline='').write(head+body+tailtxt)
ok=all(tab[i]['rva']>=tab[i-1]['rva'] for i in range(1,len(tab)))
print("\n추가 %d행 → 표 %d행 / 오름차순 %s" % (add,len(tab),'OK' if ok else '★깨짐'))
