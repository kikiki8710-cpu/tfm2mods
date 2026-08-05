# -*- coding: utf-8 -*-
# verify_illust_054.py — showcase.rs precheck 를 0.5.4 파일이미지에 그대로 재현
import struct
import bp054 as B
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
md = Cs(CS_ARCH_X86, CS_MODE_64); md.detail = True
D, S, F = B.DN, B.SN, B.FN

def f32s(rva, n):
    o = B.roff(S, rva)
    return list(struct.unpack_from("<"+"f"*n, D, o))

def disp_target(rva):
    """disp4 필드 위치 → 타겟 f32 값 (명령끝 = rva+4)"""
    d = struct.unpack_from("<i", D, B.roff(S, rva))[0]
    t = rva + 4 + d
    o = B.roff(S, t)
    return t, (struct.unpack_from("<f", D, o)[0] if o is not None else None)

C = dict(CARD_RECT=0x338b480, SNAP_RECT=0x338b4b0, LINE_DIR=0x338b4e0,
         LINE_START=0x338b4f0, LINE_ANCHOR=0x338b500, NORMAL=0x338b4c0)
EXP = dict(CARD_RECT=[-180.0,-240.0,360.0,480.0], SNAP_RECT=[0.0,0.0,360.0,480.0],
           LINE_DIR=[360.0,340.0], LINE_START=[-180.0,170.0],
           LINE_ANCHOR=[0.0,170.0], NORMAL=[0.6866,0.727])
ok = True
for k, a in C.items():
    n = len(EXP[k]); v = f32s(a, n)
    good = all(abs(x-y) < 1e-3 for x, y in zip(v, EXP[k]))
    ok &= good
    print(f"  {k:12s} 0x{a:x} = {v}  기대 {EXP[k]}  {'OK' if good else '✗'}")

v = struct.unpack_from("<I", D, B.roff(S, 0x1e17510))[0]
print(f"  I_SNAP_H     0x1e17510 imm32 = {v:#x} (기대 0x43F00000={struct.unpack('<f',struct.pack('<I',v))[0]})  {'OK' if v==0x43F00000 else '✗'}")
ok &= (v == 0x43F00000)

for k, a, e in [("D_SNAP_W",0x1e17526,360.0), ("D_CUT_LO",0x1db23f8,-70.0),
                ("D_CUT_HI",0x1db2406,70.0), ("D_ZIG_X1",0x1e18372,-180.0),
                ("D_ZIG_X2",0x1e18a50,-180.0)]:
    t, val = disp_target(a)
    good = val is not None and abs(val-e) < 1e-3
    ok &= good
    print(f"  {k:12s} 0x{a:x} → 타겟 0x{t:x} = {val}  기대 {e}  {'OK' if good else '✗'}")

sl = f32s(0x3fe2000, 4)
o = B.roff(S, 0x3fe2000)
allz = all(x == 0.0 for x in sl) and D[o-64:o+128] == b"\0"*192
print(f"  SLOTS        0x3fe2000 = {sl}  주변 ±64B 전부 0 = {allz}  {'OK' if allz else '✗'}")
ok &= allz

print(f"\n▶ precheck 종합 = {'PASS' if ok else 'FAIL'}")

print("\n▶ 훅 진입부 원본바이트 대조")
for nm, r, ln in [("FX_SET",0x1d92980,12),("CARD_DRAW",0x1da8410,12),("ILLUST_GET",0x1ffd970,13)]:
    o = B.roff(S, r)
    print(f"  {nm:11s} 0x{r:x} [{ln}] = " + ", ".join(f"0x{b:02X}" for b in D[o:o+ln]))
