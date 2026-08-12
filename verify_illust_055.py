# -*- coding: utf-8 -*-
# verify_illust_055.py — showcase.rs precheck 를 0.5.5 파일이미지에 그대로 재현
import struct
import bp055 as B
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
md = Cs(CS_ARCH_X86, CS_MODE_64); md.detail = True
roff, sec = B.roff, B.sec

# ── 재핀 결과 (0.5.5) ──
G = dict(CARD_RECT=0x3431760, SNAP_RECT=0x34317a0, NORMAL=0x34317c0,
         LINE_DIR=0x34317e0, LINE_START=0x34317f0, LINE_ANCHOR=0x3431800)
I_SNAP_H = 0x19e67a0
D_SNAP_W = 0x19e67b6
D_CUT_LO = 0x1970fc8
D_CUT_HI = 0x1970fd6
D_ZIG_X1 = 0x19e7602
D_ZIG_X2 = 0x19e7ce0
SLOTS    = 0x40c5000

def f32s(rva, n):
    o = roff(B.SN, rva)
    return list(struct.unpack_from("<" + "f" * n, B.DN, o))

def u32(rva):
    o = roff(B.SN, rva)
    return struct.unpack_from("<I", B.DN, o)[0]

def disp_target(rva):
    # showcase disp_target_is: 필드에서 i32 disp 읽고 field+4+disp
    o = roff(B.SN, rva)
    disp = struct.unpack_from("<i", B.DN, o)[0]
    tgt = rva + 4 + disp
    return tgt, f32s(tgt, 1)[0]

def approx(a, b, eps=1e-3):
    return all(abs(x - y) < eps for x, y in zip(a, b))

print("=" * 70)
print("geom .rdata 상수 (precheck)")
print("=" * 70)
checks = []
def rep(name, got, want, ok):
    checks.append(ok)
    print(f"  {name:12s} 0x{'':0s}{got}  {'OK' if ok else '**FAIL**'}")

for name, rva, want in (("CARD_RECT", G["CARD_RECT"], [-180, -240, 360, 480]),
                        ("SNAP_RECT", G["SNAP_RECT"], [0, 0, 360, 480]),
                        ("LINE_DIR", G["LINE_DIR"], [360, 340]),
                        ("LINE_START", G["LINE_START"], [-180, 170]),
                        ("LINE_ANCHOR", G["LINE_ANCHOR"], [0, 170]),
                        ("NORMAL", G["NORMAL"], [0.68662357, 0.72701317])):
    v = f32s(rva, len(want))
    ok = approx(v, want)
    checks.append(ok)
    print(f"  {name:12s} 0x{rva:x} = {[round(x,6) for x in v]:}  {'OK' if ok else '**FAIL**'}")

print("=" * 70)
print("mid-func 필드")
print("=" * 70)
v = u32(I_SNAP_H); ok = v == 0x43F00000; checks.append(ok)
print(f"  I_SNAP_H     0x{I_SNAP_H:x} imm32 = 0x{v:08X}  {'OK' if ok else '**FAIL**'}")
for name, rva, want in (("D_SNAP_W", D_SNAP_W, 360.0), ("D_CUT_LO", D_CUT_LO, -70.0),
                        ("D_CUT_HI", D_CUT_HI, 70.0), ("D_ZIG_X1", D_ZIG_X1, -180.0),
                        ("D_ZIG_X2", D_ZIG_X2, -180.0)):
    tgt, val = disp_target(rva)
    ok = abs(val - want) < 1e-3; checks.append(ok)
    print(f"  {name:12s} 0x{rva:x} -> 0x{tgt:x} = {val}  {'OK' if ok else '**FAIL**'}")

print("=" * 70)
print("SLOTS")
print("=" * 70)
sl = f32s(SLOTS, 4)
o = roff(B.SN, SLOTS)
around0 = all(b == 0 for b in B.DN[o - 64:o + 80])
ok = all(x == 0.0 for x in sl) and around0; checks.append(ok)
print(f"  SLOTS        0x{SLOTS:x} = {sl}, 주변±64B 전부0 = {around0}  {'OK' if ok else '**FAIL**'}")

print("=" * 70)
print(f"precheck 종합 = {'PASS' if all(checks) else 'FAIL'}  ({sum(checks)}/{len(checks)})")
