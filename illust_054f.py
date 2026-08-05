# -*- coding: utf-8 -*-
# illust_054f.py — geom mid-func 필드 위치 재핀 (0.5.3 → 0.5.4)
#   RVA_I_SNAP_H = mov dword[rsp+0x20], 480.0 의 imm4 위치
#   RVA_D_*      = rip-rel disp4 위치 (disp_loc+4 = 명령끝, 타겟 = disp_loc+4+disp)
import re, struct
import bp054 as B
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
md = Cs(CS_ARCH_X86, CS_MODE_64); md.detail = True

CONT = {0x1c52950: 0x1e16c90, 0x1bf89a0: 0x1db2370}
CASES = [("RVA_I_SNAP_H", 0x1c531d0, 0x1c52950, "imm"),
         ("RVA_D_SNAP_W", 0x1c531e6, 0x1c52950, "disp"),
         ("RVA_D_CUT_LO", 0x1bf8a28, 0x1bf89a0, "disp"),
         ("RVA_D_CUT_HI", 0x1bf8a36, 0x1bf89a0, "disp"),
         ("RVA_D_ZIG_X1", 0x1c54032, 0x1c52950, "disp"),
         ("RVA_D_ZIG_X2", 0x1c54710, 0x1c52950, "disp")]

def dis_fn(d, secs, fns, start):
    f = B.owner(fns, start); o = B.roff(secs, f[0])
    return list(md.disasm(d[o:o+(f[1]-f[0])], f[0])), f

def rip_target(i):
    for op in i.operands:
        if op.type == 3 and op.mem.base == 41:
            return i.address + i.size + op.mem.disp
    return None

def f32_at(d, secs, rva):
    o = B.roff(secs, rva)
    return struct.unpack_from("<f", d, o)[0] if o is not None else None

print("="*100); print("0.5.3 원본 명령 복원"); print("="*100)
OLD = {}; cache = {}
for nm, site, cont, kind in CASES:
    if cont not in cache: cache[cont] = dis_fn(B.DO, B.SO, B.FO, cont)
    ins, f = cache[cont]
    hit = next((i for i in ins if i.address <= site < i.address+i.size), None)
    if not hit: print(f"  {nm:14s} 명령 복원 실패"); continue
    off = site - hit.address
    tgt = rip_target(hit)
    val = f32_at(B.DO, B.SO, tgt) if tgt else None
    if kind == "imm": val = struct.unpack_from("<f", B.DO, B.roff(B.SO, site))[0]
    OLD[nm] = dict(off=off, val=val, mnem=hit.mnemonic, ops=hit.op_str,
                   bytes=bytes(hit.bytes), size=hit.size, addr=hit.address, rel=hit.address-f[0])
    print(f"  {nm:14s} 명령 0x{hit.address:x}(+{hit.address-f[0]:#x}) [{hit.mnemonic} {hit.op_str}] "
          f"크기={hit.size} 필드+{off} 값={val}")
    print(f"                 바이트={bytes(hit.bytes).hex(' ')}" + (f" rip타겟 0x{tgt:x}" if tgt else ""))

print("\n"+"="*100); print("0.5.4 컨테이너에서 같은 명령 찾기"); print("="*100)
ncache = {}; RESULT = {}
for nm, site, cont, kind in CASES:
    if nm not in OLD: continue
    o = OLD[nm]; nc = CONT[cont]
    if nc not in ncache: ncache[nc] = dis_fn(B.DN, B.SN, B.FN, nc)
    ins, f = ncache[nc]
    hits = []
    for i in ins:
        if i.mnemonic != o["mnem"]: continue
        if kind == "imm":
            if bytes(i.bytes) == o["bytes"]:
                hits.append((i.address, i.address+o["off"], None, i.address-f[0]))
        else:
            t = rip_target(i)
            if t is None: continue
            v = f32_at(B.DN, B.SN, t)
            if v is not None and o["val"] is not None and abs(v-o["val"]) < 0.001 and i.size == o["size"]:
                hits.append((i.address, i.address+i.size-4, t, i.address-f[0]))
    print(f"  {nm:14s} 컨테이너 0x{nc:x}(크기 {f[1]-f[0]}) 후보 {len(hits)}건  [OLD 상대 +{o['rel']:#x}]")
    for a, fld, t, rel in hits[:8]:
        d = rel - o["rel"]
        print(f"       명령 0x{a:<9x}(+{rel:#x}, Δrel={d:+d}) → 필드 **0x{fld:x}**"
              + (f"  타겟 0x{t:x}={f32_at(B.DN,B.SN,t)}" if t else ""))
    if len(hits) == 1: RESULT[nm] = hits[0][1]
    elif hits:
        # 상대오프셋 최근접으로 좁힘
        hits.sort(key=lambda h: abs(h[3]-o["rel"]))
        RESULT[nm] = [h[1] for h in hits]

print("\n"+"="*100); print("결과 요약"); print("="*100)
for nm, site, cont, kind in CASES:
    r = RESULT.get(nm)
    if isinstance(r, int): print(f"  {nm:14s} 0x{site:x} → **0x{r:x}**  (유일)")
    elif r: print(f"  {nm:14s} 0x{site:x} → 다중 {[hex(x) for x in r]}")
    else: print(f"  {nm:14s} 0x{site:x} → 미해결")
