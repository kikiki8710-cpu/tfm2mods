# 08-03 2차 배선(position_eval + c66800)의 전 사이트를 exe에 대고 정적 검증.
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
CAPCMP = [([0x48,0x81,0xf9],3), ([0x49,0x81,0xfc],3), ([0x48,0x81,0xff],3), ([0x48,0x81,0xfb],3), ([0x48,0x3d],2)]
CAPMOV = [([0xb8],1), ([0xb9],1), ([0xbb],1)]
STGATE = [([0x48,0x81,0xbf,0xb8,0,0,0],7), ([0x48,0x81,0xbf,0xc0,0,0,0],7), ([0x48,0x81,0xb9,0xc8,0,0,0],7)]
def add(lab, rvas, pre, off, w, want):
    for i, r in enumerate(rvas):
        C.append((f"{lab}[{i}]" if len(rvas) > 1 else lab, r, [(pre, off)], w, want))
def addm(lab, rvas, cands, w, want):
    for i, r in enumerate(rvas):
        C.append((f"{lab}[{i}]" if len(rvas) > 1 else lab, r, cands, w, want))

# ── position_eval ──
add("pe_collect(+1)", [0xcca385,0xcca3c0,0xcca454,0xcca490,0xcca523,0xcca560,0xcca5f7,0xcca62d], [0x48,0xb8], 2, 8, 0x9502F9001)
add("pe_collect",     [0xcca6c3,0xcca6f8,0xccaa2e,0xccaa60], [0x48,0xb8], 2, 8, 0x9502F9000)
add("pe_filter(f9)",  [0xccbc19,0xccbda6,0xccbf36,0xccc0a6], [0x48,0x81,0xf9], 3, 4, 87_890_625)
add("pe_filter(fa)",  [0xcccfc8,0xccd068], [0x48,0x81,0xfa], 3, 4, 87_890_625)
add("pe_filter(3d)",  [0xccaeef], [0x48,0x3d], 2, 4, 87_890_624)
add("pe_filter(fa-)", [0xccd108], [0x48,0x81,0xfa], 3, 4, 87_890_624)
add("pe_filter(bb)",  [0xccd76e,0xccd78e,0xccdad8], [0x49,0xbb], 2, 8, 0x53D1AC100)
add("pe_near(+1)",    [0xccbfaa,0xccc11a], [0x49,0x81,0xfe], 3, 4, 19_140_625)
add("pe_near",        [0xccbc86,0xccbe16], [0x49,0x81,0xfe], 3, 4, 19_140_624)
add("pe_minion_add",  [0xccd3f3], [0xb8], 1, 4, 4_096_000_000)
add("pe_champ_threat",[0xccefaa], [0x48,0x81,0xf9], 3, 4, 9_765_625)
add("pe_field_radius",[0xccdcf1], [0x49,0x81,0xf8], 3, 4, 244_140_624)
add("pe_count(+1)",   [0xcd01f5], [0x48,0xb8], 2, 8, 0x35A4E9001)
add("pe_count",       [0xcd0aab], [0x48,0xb8], 2, 8, 0x35A4E9000)
add("pe_reach(c0)",   [0xcce53c,0xcce574,0xcce6dc,0xcce714,0xcce8ac,0xcce8e4,0xccea4c,
                       0xccea84,0xccec2c,0xccec64,0xccedcc,0xccee04,0xcd059b,0xcd05cf], [0x49,0x81,0xc0], 3, 4, 80_000)
add("pe_reach(c1)",   [0xccf4da,0xccf758,0xccfe3f], [0x48,0x81,0xc1], 3, 4, 80_000)
add("pe_reach(r9)",   [0xcd0720,0xcd0754,0xcd0890,0xcd08c8], [0x49,0x81,0xc1], 3, 4, 80_000)
add("pe_outer_band",  [0xccb5af,0xccdac4], [0x48,0x05], 2, 4, 32_000)
add("pe_skillshot",   [0xccfd38], [0x48,0x05], 2, 4, 20_000)
add("pe_bodyblock",   [0xcd00be,0xcd0190,0xcd0b1c], [0x48,0x81,0xc2], 3, 4, 28_000)
add("pe_tower_margin(05)", [0xccccc4], [0x48,0x05], 2, 4, 18_000)
add("pe_tower_margin(c1)", [0xccde6f], [0x49,0x81,0xc1], 3, 4, 18_000)
for i,(ca,mv) in enumerate([(0xccafff,0xccb006),(0xccb1d6,0xccb1dd),(0xccc611,0xccc618),
                            (0xccc97a,0xccc980),(0xccd31c,0xccd323),(0xccd54e,0xccd555),
                            (0xccd8ec,0xccd8f3),(0xccdee8,0xccdeee),(0xcce17c,0xcce182),
                            (0xcce325,0xcce32b),(0xccff97,0xccff9e)]):
    addm(f"pe_cap_cmp{i}", [ca], CAPCMP, 4, 150)
    addm(f"pe_cap_mov{i}", [mv], CAPMOV, 4, 150)
add("pe_predict_cmp", [0xccd5c5], [0x48,0x3d], 2, 4, 140)
add("pe_predict_mov", [0xccd5cb], [0xbb], 1, 4, 140)
add("pe_tower_far",   [0xccce81,0xcccef2], [0x69,0xc0], 2, 4, 656)
add("pe_noise_amp2",  [0xcd0e8e], [0x41,0xbe], 2, 4, 1000)
add("pe_noise_amp",   [0xcd0e9e], [0xb9], 1, 4, 2000)
add("pe_noise_exempt",[0xcd0e38], [0x48,0x81,0xf9], 3, 4, 100_000)
add("pe_kscale(cf)",  [0xcd0db9], [0x48,0x6b,0xcf], 3, 1, 120)
add("pe_kscale(48)",  [0xcd0ddc], [0x49,0x6b,0x48,0x10], 4, 1, 120)
add("pe_mode_mask",   [0xccb681,0xccd609], [0xba], 1, 4, 0x1a1)
add("pe_kind_mask",   [0xccd654], [0xb9], 1, 4, 0x303)
add("pe_wall_risk",   [0xcc9eaf], [0x48,0xc7,0x00], 3, 4, 9999)
add("pe_well(02)",    [0xcca0a6], [0x48,0xc7,0x02], 3, 4, 9999)
add("pe_well(4208)",  [0xcca0ad], [0x48,0xc7,0x42,0x08], 4, 4, 9999)
add("pe_well(b8)",    [0xcca0b5], [0xb8], 1, 4, 9999)
add("pe_ally_gain_cut",[0xcd0a31], [0x48,0x81,0xbd,0x28,0x04,0x00,0x00], 7, 4, 1200)
addm("pe_state_gate", [0xcce400,0xcce77d,0xcceaf4], STGATE, 4, 180)
# ── c66800 ──
add("ldsc_vision",    [0xc66ec3,0xc66f33,0xc66f9e], [0x49,0x83,0xc6], 3, 1, 120)
add("ldsc_skill_fac", [0xc66b95], [0x83,0xc2], 2, 1, 100)
add("ldsc_early_mask",[0xdf58de], [0x41,0xb8], 2, 4, 0x1F863)
add("ldsc_lost_tgt",  [0xc66be3], [0x48,0xc7,0xc1], 3, 4, (-99999) & 0xffffffff)

okc = 0; fails = []
for lab, rva, cands, w, want in C:
    hit = False
    for pre, off in cands:
        g = rd(rva, len(pre))
        if g is not None and list(g) == pre and imm(rva, off, w) == want:
            hit = True; break
    if hit: okc += 1
    else:   fails.append((lab, rva, "불일치", f"기대값 {want}", rd(rva,12)))
print(f"=== 2차 배선 정적 검증: {okc}/{len(C)} PASS ===\n")
for lab, rva, why, det, raw in fails:
    print(f"  [{why}] {lab}  @0x{rva:x}")
    print(f"      {det}")
    print(f"      실제 바이트: {raw.hex(' ') if raw else '?'}")
