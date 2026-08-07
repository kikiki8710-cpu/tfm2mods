# 08-04 배선(move2/bv/ae/th/rt)의 전 사이트를 exe에 대고 정적 검증.
import struct
EXE = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe"
D = open(EXE, "rb").read()
p = struct.unpack_from("<I", D, 0x3c)[0]
n = struct.unpack_from("<H", D, p + 6)[0]; o = struct.unpack_from("<H", D, p + 20)[0]
S = []
for i in range(n):
    q = p + 24 + o + i * 40
    vsz, va, rsz, ra = struct.unpack_from("<IIII", D, q + 8); S.append((va, max(vsz, rsz), ra, rsz))
def f(r):
    for va, vsz, ra, rsz in S:
        if va <= r < va + vsz and r - va < rsz: return ra + r - va
def rd(r, k):
    x = f(r); return D[x:x+k] if x is not None else None
def imm(r, off, w):
    b = rd(r + off, w); return int.from_bytes(b, "little") if b and len(b) == w else None

C = []
def add(lab, rvas, pre, off, w, want):
    for i, r in enumerate(rvas):
        C.append((f"{lab}[{i}]" if len(rvas) > 1 else lab, r, [(pre, off)], w, want))
def addm(lab, rvas, cands, w, want):
    for i, r in enumerate(rvas):
        C.append((f"{lab}[{i}]" if len(rvas) > 1 else lab, r, cands, w, want))

# ── move2 ──
add("mv2_snap",       [0xc8694b], [0x48,0x3d], 2, 4, 2000)
add("mv2_coef",       [0xc86a36], [0x48,0x69,0xc2], 3, 4, 400)
add("mv2_coef_adj",   [0xc86a77], [0x48,0x83,0xc2], 3, 1, 50)
add("mv2_margin",     [0xc86a86], [0x48,0x05], 2, 4, 6000)
add("mv2_bias",       [0xc86f23], [0x48,0x3d], 2, 4, 1500)
add("mv2_well_r",     [0xd94766,0xc86807], [0x48,0xb8], 2, 8, 67_600_000_000)
add("mv2_well_d0",    [0xd94863], [0x49,0x69,0xc5], 3, 4, 260_000)
add("mv2_well_d1",    [0xd9486a], [0x4d,0x69,0xce], 3, 4, 260_000)
add("mv2_well_d2",    [0xc8689a], [0x49,0x69,0xc7], 3, 4, 260_000)
add("mv2_well_d3",    [0xc868a1], [0x4d,0x69,0xce], 3, 4, 260_000)
add("mv2_posmode",    [0xd87c92], [0x48,0x83,0x7f,0x18], 4, 1, 10)
# ── bv ──
for i,(c,m) in enumerate([(0xcc5fcf,0xcc5fd6),(0xcc6598,0xcc659f),(0xcc9004,0xcc900b)]):
    add(f"bv_cap160c{i}", [c], [0x48,0x81,0xf9], 3, 4, 160)
    add(f"bv_cap160m{i}", [m], [0xb8], 1, 4, 160)
for i,(c,m) in enumerate([(0xcc690b,0xcc690f),(0xcc6f83,0xcc6f87)]):
    add(f"bv_cap80c{i}", [c], [0x48,0x83,0xf9], 3, 1, 80)
    add(f"bv_cap80m{i}", [m], [0xb8], 1, 4, 80)
add("bv_focus_a", [0xcc71dd], [0x48,0x83,0xfa], 3, 1, 3)
add("bv_focus_b", [0xcc71e1], [0xb8], 1, 4, 3)
add("bv_focus_c", [0xcc54ee], [0x49,0x83,0xf8], 3, 1, 3)
add("bv_focus_d", [0xcc54f2], [0xb8], 1, 4, 3)
add("bv_frad_A",  [0xcc7092,0xcc70e2,0xcc7131,0xcc7180], [0x41,0xb9], 2, 4, 3_600_000_001)
add("bv_frad_A4", [0xcc71c9], [0xb9], 1, 4, 3_600_000_001)
add("bv_frad_B",  [0xcc5a32,0xcc5a84,0xcc5ad3,0xcc5b22], [0x41,0xbb], 2, 4, 3_600_000_001)
add("bv_frad_B4", [0xcc5b78], [0xba], 1, 4, 3_600_000_001)
add("bv_ally_flat",[0xcc9341], [0xbf], 1, 4, 10)
add("bv_ally_capc",[0xcc939c], [0x48,0x83,0xf8], 3, 1, 90)
add("bv_ally_capm",[0xcc93a0], [0xbf], 1, 4, 90)
add("bv_out",     [0xcc93c3], [0xba], 1, 4, 5)
add("bv_b_in",    [0xcc9254], [0xb8], 1, 4, 25)
add("bv_b_out",   [0xcc9259], [0xba], 1, 4, 8)
add("bv_d_in",    [0xcc9332], [0xb8], 1, 4, 90)
add("bv_d_out",   [0xcc9337], [0xba], 1, 4, 30)
add("bv_c_capc",  [0xcc949e], [0x48,0x83,0xf9], 3, 1, 60)
add("bv_c_capm",  [0xcc94a2], [0xba], 1, 4, 60)
add("bv_c_none",  [0xcc92b7], [0x48,0xc7,0xc2], 3, 4, (-100)&0xffffffff)
# ── ae ──
add("ae_mask",    [0xdf58de], [0x41,0xb8], 2, 4, 0x1F863)
add("ae_rsh",     [0xdf5b96], [0x49,0xc1,0xfe], 3, 1, 6)
add("ae_tsh",     [0xdf5bcf], [0x48,0xc1,0xf8], 3, 1, 6)
add("ae_gsh",     [0xdf5cfe], [0x48,0xc1,0xfa], 3, 1, 7)
add("ae_soonA",   [0xdf5f01], [0x41,0xbc], 2, 4, 25)
add("ae_soonB",   [0xdf63b7], [0xba], 1, 4, 25)
add("ae_killA",   [0xdf5fbc], [0x41,0xbc], 2, 4, 140)
add("ae_killB",   [0xdf636e], [0xba], 1, 4, 140)
add("ae_nearA",   [0xdf5fdc], [0x41,0xbc], 2, 4, 70)
add("ae_nearB",   [0xdf638a], [0xba], 1, 4, 70)
add("ae_struct",  [0xdf6430], [0x48,0x8d,0x4a], 3, 1, 80)
add("ae_thr",     [0xdf5b51,0xdf5b9d], [0x41,0xb8], 2, 4, 9999)
# ── th ──
add("th_smg",     [0xd07d1a,0xd07e06,0xd07ee6], [0x48,0x05], 2, 4, 18_000)
add("th_smg2",    [0xd08501], [0x48,0x81,0xc1], 3, 4, 18_000)
add("th_amg",     [0xd0850d], [0x49,0x81,0xc3], 3, 4, 50_000)
LEA32 = [([0x48,0x8d,0x8d],3),([0x48,0x8d,0x8b],3),([0x49,0x8d,0x8a],3),([0x49,0x8d,0x8f],3),
         ([0x4c,0x8d,0xb7],3),([0x4d,0x8d,0x82],3),([0x49,0x8d,0xaf],3),([0x48,0x8d,0xbb],3),
         ([0x48,0x8d,0x8a],3),([0x4c,0x8d,0xa2],3),([0x49,0x8d,0xbe],3),([0x49,0x8d,0xab],3)]
addm("th_band", [0xd0851c,0xd08533,0xd0854a,0xd085a1,0xd085c5,0xd086e4,0xd086f9,
                 0xd09295,0xd092ab,0xd092c8,0xd09394,0xd093ab,0xd093c3], LEA32, 4, 32_000)
CAPMOV2 = [([0xb9],1),([0x41,0xba],2),([0x41,0xbc],2),([0x41,0xbb],2)]
for i,(c,m) in enumerate([(0xd082f2,0xd082f8),(0xd0833f,0xd08345),(0xd0843f,0xd08445),
                          (0xd08564,0xd0856a),(0xd086bc,0xd086c2),(0xd08749,0xd0874f),
                          (0xd0908e,0xd09094),(0xd0910f,0xd09115),(0xd0919f,0xd091a5),
                          (0xd09274,0xd0927a),(0xd09374,0xd0937a),(0xd09406,0xd0940f)]):
    add(f"th_capc{i}", [c], [0x48,0x3d], 2, 4, 150)
    addm(f"th_capm{i}", [m], CAPMOV2, 4, 150)
add("th_coll1", [0xcca385,0xcca3c0,0xcca454,0xcca490,0xcca523,0xcca560,0xcca5f7,0xcca62d],
    [0x48,0xb8], 2, 8, 0x9502F9001)
add("th_coll0", [0xcca6c3,0xcca6f8,0xccaa2e,0xccaa60], [0x48,0xb8], 2, 8, 0x9502F9000)
# ── rt ──
add("rt_a_slope", [0xd64318], [0x69,0xc1], 2, 4, (-800)&0xffffffff)
add("rt_a_base",  [0xd6431e], [0x05], 1, 4, 80_000)
add("rt_a_off",   [0xd6432e], [0x83,0xc0], 2, 1, 80)
add("rt_b_slope", [0xd64335], [0x69,0xc1], 2, 4, 450)
add("rt_b_base",  [0xd6434a], [0x83,0xc0], 2, 1, 45)
add("rt_c_slope", [0xd64351], [0x69,0xc1], 2, 4, 350)
add("rt_c_base",  [0xd64366], [0x83,0xc0], 2, 1, 15)
add("rt_dl_cmp",  [0xd654d5], [0x48,0x83,0xf9], 3, 1, 61)
add("rt_dl_mov",  [0xd654d9], [0xba], 1, 4, 60)
add("jg_fight",   [0xdffebc], [0x48,0x83,0xf8], 3, 1, 21)
add("jg_nofight", [0xdfff00], [0x48,0x83,0xf8], 3, 1, 41)

okc, fails = 0, []
for lab, rva, cands, w, want in C:
    hit = False
    for pre, off in cands:
        g = rd(rva, len(pre))
        if g is not None and list(g) == pre and imm(rva, off, w) == want:
            hit = True; break
    if hit: okc += 1
    else:
        raw = rd(rva, 12)
        fails.append((lab, rva, want, raw.hex(' ') if raw else '?'))

print(f"=== 08-04 배선 정적 검증: {okc}/{len(C)} PASS ===")
if fails:
    print(f"\n--- FAIL {len(fails)}건 ---")
    for lab, rva, want, raw in fails:
        print(f"  {lab:16s} @0x{rva:x}  기대값 {want}")
        print(f"      실제 바이트: {raw}")
