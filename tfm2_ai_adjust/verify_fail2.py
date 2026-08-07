# auction_imm(50/55) · score2_imm(33/37)의 **실패 9건**이 어디인지 특정한다.
import struct

EXE = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe"
d = open(EXE, "rb").read()
p = struct.unpack_from("<I", d, 0x3c)[0]
n = struct.unpack_from("<H", d, p + 6)[0]; osz = struct.unpack_from("<H", d, p + 20)[0]
S = []
for i in range(n):
    o = p + 24 + osz + i * 40
    vsz, va, rsz, ra = struct.unpack_from("<IIII", d, o + 8); S.append((va, max(vsz, rsz), ra, rsz))
def f(r):
    for va, vsz, ra, rsz in S:
        if va <= r < va + vsz and r - va < rsz: return ra + r - va
def rd(r, k):
    o = f(r); return d[o:o+k] if o is not None else None
def imm(r, off, w):
    b = rd(r + off, w); return int.from_bytes(b, "little") if b and len(b) == w else None

CMP3 = [[0x48,0x81,0xfe],[0x49,0x81,0xfa],[0x48,0x81,0xfa],[0x49,0x81,0xf9]]
CMP1 = [[0x48,0x83,0xf8],[0x49,0x83,0xf8],[0x48,0x83,0xff],[0x49,0x83,0xff]]

# (그룹, 라벨, rva, 후보 prefix들, imm_off, width, 기대 원본값)
G = []
def add(grp, lab, rvas, pres, off, w, want):
    for i, r in enumerate(rvas):
        G.append((grp, f"{lab}[{i}]" if len(rvas) > 1 else lab, r, pres, off, w, want))

# ══ auction_imm ══
A = "auction"
add(A, "au_noise_amp",  [0xd5febc], [[0xba]], 1, 4, 900)
add(A, "au_score_center(mov)", [0xd5fefc], [[0xbb]], 1, 4, 1000)
add(A, "au_score_center(add)", [0xd5ff04], [[0x48,0x81,0xc6]], 3, 4, 1000)
add(A, "bt_hp_flee",    [0xcab663], [[0x48,0x83,0xf8]], 3, 1, 21)
# 08-05: 0xca920b 제외 — 거긴 HP가 아니라 &sub_plan.with_dive 포인터 산술이다(상수 41만 우연히 겹침).
add(A, "bt_hp_gate",    [0xcab1ef], [[0x48,0x83,0xf8]], 3, 1, 41)
# 08-05: emit 전수 대조로 chase_stop 6곳 / chase_keep 2곳으로 정정(각각 2곳·1곳 누락돼 있었음).
add(A, "bt_chase_stop", [0xca9cf9,0xca9da1,0xca9fb3,0xcaa323,0xcab77e,0xcabb22], [[0x48,0xc7,0x85]], 7, 4, 15000)
add(A, "bt_chase_stop(add)", [0xcac3be], [[0x48,0x81,0xc1]], 3, 4, 15000)
add(A, "bt_chase_keep", [0xcaba72,0xcac136], [[0x48,0xc7,0x85]], 7, 4, 80000)
add(A, "bt_vision_mem", [0xca9930,0xcaa5cf,0xcaaf7e,0xcac8dd,0xcacbac,0xcaceac,0xcad097],
                        [[0x49,0x83,0xc5],[0x48,0x83,0xc6],[0x49,0x83,0xc6],[0x49,0x83,0xc7]], 3, 1, 120)
add(A, "ld_chase_stop", [0xc624cd,0xc62269,0xc6236c,0xc62422], [[0x48,0xc7,0x85]], 7, 4, 15000)
add(A, "ld_ally_near",  [0xc619d2,0xc61a3b,0xc61aa7,0xc61b13,0xc61b77],
                        [[0x49,0x81,0xfa],[0x48,0x3d]], 3, 4, 390625)   # 주의: 두 prefix의 off가 다름
add(A, "ld_intervene",  [0xc61784, 0xc5f059], [[0x48,0x81,0xfa]], 3, 4, 9765625)
add(A, "ld_vision_mem(c5eee7)", [0xc5eee7], [[0x49,0x83,0xc6]], 3, 1, 120)
add(A, "ld_vision_mem(c63b87)", [0xc63b87], [[0x49,0x83,0xc4]], 3, 1, 120)
add(A, "ld_vision_mem(c61667)", [0xc61667], [[0x48,0x83,0xc7]], 3, 1, 120)
add(A, "ld_est_base",   [0xc61cb4], [[0x83,0xc1]], 2, 1, 10)
add(A, "ld_around_range", [0xc5f2a8,0xc5f6d7,0xc5fbdc,0xc62ec4,0xc62fd4,0xc633a5,0xc63488], [[0x48,0xc7,0x85]], 7, 4, 80000)
add(A, "ld_around_delay", [0xc5f2b3,0xc5f6e2,0xc5fbe7], [[0x48,0xc7,0x85]], 7, 4, 5)
add(A, "ld_mode_mask(ba)", [0xc5e61a, 0xc5f664], [[0xba]], 1, 4, 0x1a1)
add(A, "ld_mode_mask(41b8)", [0xc5e9b3], [[0x41,0xb8]], 2, 4, 0x1a1)
add(A, "ld_move_pct",   [0xc5e31a,0xc5e55e,0xc5fe08,0xc5fe56], [[0x48,0x83,0xc0]], 3, 1, 100)
add(A, "ld_threat_state", [0xc5e3cc], [[0x48,0x83,0x7b,0x68]], 4, 1, 13)
add(A, "ld_rand_min",   [0xc60a0c], [[0x48,0x83,0xfa]], 3, 1, 2)
add(A, "tm_cancel_mask", [0xd55bac, 0xd4c4c9], [[0xb9]], 1, 4, 0x0b00)

# ══ score2_imm ══
B = "score2"
add(B, "sc_turret_radius", [0xc7f8b9,0xc7f939,0xc7f9ac,0xc7fa1f,0xc7faa0,0xc7fcaf,
                            0xc80289,0xc80309,0xc80397,0xc80418,0xc8048b,0xc80513], CMP3, 3, 4, 0x53D1AC1)
add(B, "sc_turret_radius(-1)", [0xc7fd67, 0xc80be7], CMP3, 3, 4, 0x53D1AC1 - 1)
add(B, "sc_engage_radius", [0xc80679,0xc80751,0xc80825,0xc808f9,0xc809cd], CMP3, 3, 4, 0x1BF08EA)
add(B, "sc_cell_dist",  [0xc80e02], CMP3, 3, 4, 0x49040441)
add(B, "sc_dive_margin",[0xc7fe82], [[0x48,0x05]], 2, 4, 15000)
for a, o in [(0xc82224,49),(0xc8222a,65),(0xc82230,29),(0xc82236,40),(0xc8223c,17),(0xc82244,25),(0xc8224a,10)]:
    add(B, f"sc_risk@{a:x}", [a], CMP1, 3, 1, o)
add(B, "sc_focus_cap(cmp)", [0xc827c4], CMP1, 3, 1, 80)
add(B, "sc_focus_cap(mov)", [0xc827c8], [[0x41,0xbe]], 2, 4, 80)
add(B, "sc_kill_cap(cmp)", [0xc82f27, 0xc82f76], CMP1, 3, 1, 80)
add(B, "sc_kill_cap(cmp2)", [0xc83391], CMP1, 3, 1, 80)
add(B, "sc_kill_cap(mov)", [0xc83395], [[0xb8],[0xb9],[0xbb]], 1, 4, 80)
add(B, "sc_kill_pct",   [0xc82f69], CMP1, 3, 1, 60)
add(B, "sc_score_vision", [0xc806d9], [[0x48,0x83,0xc6],[0x49,0x83,0xc6],[0x48,0x83,0xc5]], 3, 1, 120)
add(B, "sc_null_score", [0xc83620, 0xc838e1], [[0x48,0xc7,0xc1],[0x48,0xc7,0xc0],[0x49,0xc7,0xc1]], 3, 4,
    (-10) & 0xffffffff)

cnt = {}
fails = []
for grp, lab, rva, pres, off, w, want in G:
    cnt[grp] = cnt.get(grp, [0, 0]); cnt[grp][1] += 1
    hit = False
    for pre in pres:
        g = rd(rva, len(pre))
        if g is None or list(g) != pre: continue
        # prefix 길이가 다른 후보는 imm_off도 그 길이에 맞춘다
        o2 = off if len(pre) == len(pres[0]) else len(pre)
        if imm(rva, o2, w) == want: hit = True; break
    if hit: cnt[grp][0] += 1
    else:
        cur = rd(rva, 12)
        fails.append((grp, lab, rva, cur.hex(' ') if cur else '?'))

for g in ("auction", "score2"):
    okc, tc = cnt.get(g, [0, 0])
    print(f"[{g}] {okc}/{tc}")
print()
print(f"--- 정적으로 안 맞는 사이트 {len(fails)}건 ---")
for grp, lab, rva, cur in fails:
    print(f"  {grp:8s} {lab:26s} @0x{rva:x}   실제바이트: {cur}")
