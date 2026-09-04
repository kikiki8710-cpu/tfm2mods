# -*- coding: utf-8 -*-
"""★또 하나의 사각지대 — `static TBL: [(usize, &[u8], usize); N]` 순회형 사이트 감사.

`apply_path_imm` 은 사이트를 소스 배열 4개(PATH_STEP640/896/RISK1281/HEUR)로 들고
`for &(a, pre, off) in TBL.iter()` 로 돈다. 내 감사 파서는 `p!`/`pany!`/`patch_imm_bytes(`
호출만 봤으므로 **이 200 사이트를 통째로 못 봤다**. 런타임은 applied=100/208 로 108개 미적용.
"""
import io, re, sys, struct
sys.path.insert(0, r'C:\tfm2mods\MIG')
sys.stdout.reconfigure(encoding='utf-8', errors='replace')
import pefile, mig_verify as MV
pe=pefile.PE(MV.GAME_EXE, fast_load=True)
SEC=[(s.VirtualAddress,s.VirtualAddress+s.Misc_VirtualSize,s.get_data()) for s in pe.sections]
def rd(r,n):
    for a,b,d in SEC:
        if a<=r<b: return d[r-a:r-a+n]
P=r'C:\tfm2mods\tfm2_ai_adjust\src\detour.rs'
txt=io.open(P,encoding='utf-8').read()
TBL=re.compile(r'static\s+(PATH_\w+)\s*:\s*\[\(usize,\s*&\[u8\],\s*usize\);\s*(\d+)\]\s*=\s*\[(.*?)\n\];', re.S)
ENT=re.compile(r'\(\s*(0x[0-9a-fA-F]+)\s*,\s*&\[([^\]]*)\]\s*,\s*(\d+)\s*\)')
WID={'PATH_STEP640':(4,640),'PATH_STEP896':(4,896),'PATH_RISK1281':(4,1281),'PATH_HEUR':(1,7)}
grand=[0,0]
for m in TBL.finditer(txt):
    nm, n, body = m.group(1), int(m.group(2)), m.group(3)
    w, orig = WID[nm]
    ents=ENT.findall(body)
    dead=[]; badv=[]; alive=0
    for a,pre,off in ents:
        rva=int(a,16); off=int(off)
        pb=[int(x,16) for x in re.findall(r'0x[0-9a-fA-F]+', pre)]
        b=rd(rva, off+w+4)
        if b is None or list(b[:len(pb)])!=pb:
            dead.append(rva); continue
        cur=b[off] if w==1 else struct.unpack('<I', b[off:off+4])[0]
        if cur!=orig: badv.append((rva,cur))
        else: alive+=1
    print("%-14s 선언 %d / 파싱 %d → 살아있음 %d · **죽음 %d** · 값불일치 %d"
          % (nm, n, len(ents), alive, len(dead), len(badv)))
    if dead[:6]: print("      죽은 예: %s" % [hex(x) for x in dead[:6]])
    if badv[:4]: print("      값불일치 예: %s" % [(hex(x),v) for x,v in badv[:4]])
    grand[0]+=alive; grand[1]+=len(ents)
print("\n합계 살아있음 %d / 전체 %d  → 죽은 사이트 %d" % (grand[0], grand[1], grand[1]-grand[0]))
print("(런타임 path_imm.txt = applied=100/208 과 대조)")
