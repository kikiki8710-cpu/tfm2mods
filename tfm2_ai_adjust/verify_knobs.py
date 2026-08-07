# 배선한 byte-patch 사이트를 exe 파일에 대고 정적 검증한다.
#   각 항목 = (키, RVA, prefix, imm_off, width, 기대 원본값)
#   PASS = 그 주소의 바이트가 prefix로 시작하고 immediate가 기대 원본값과 일치
import struct, sys

EXE = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe"
data = open(EXE, "rb").read()

# ── PE 헤더 파싱: RVA → 파일 오프셋 ──
pe_off = struct.unpack_from("<I", data, 0x3c)[0]
assert data[pe_off:pe_off+4] == b"PE\0\0"
nsec = struct.unpack_from("<H", data, pe_off + 6)[0]
opt_sz = struct.unpack_from("<H", data, pe_off + 20)[0]
sec_off = pe_off + 24 + opt_sz
secs = []
for i in range(nsec):
    o = sec_off + i * 40
    name = data[o:o+8].rstrip(b"\0").decode(errors="replace")
    vsz, va, rsz, ra = struct.unpack_from("<IIII", data, o + 8)
    secs.append((name, va, max(vsz, rsz), ra, rsz))

def r2f(rva):
    for name, va, vsz, ra, rsz in secs:
        if va <= rva < va + vsz:
            off = rva - va
            if off >= rsz:
                return None      # .bss 류 = 파일에 없음
            return ra + off
    return None

def rd(rva, n):
    f = r2f(rva)
    if f is None: return None
    return data[f:f+n]

def imm(rva, off, w):
    b = rd(rva + off, w)
    if b is None or len(b) < w: return None
    return int.from_bytes(b, "little")

# ── 검증 대상 ──
# (키, RVA, prefix bytes, imm_off, width, 기대 원본값)
CHECKS = [
    # ══ line_defense 1회차 — 08-03 누락 보강분 ══
    ("ld_intervene(1회차·신규)", 0xc5f059, [0x48,0x81,0xfa], 3, 4, 9765625),
    ("ld_intervene(2회차·기존)", 0xc61784, [0x48,0x81,0xfa], 3, 4, 9765625),
    ("ld_vision_mem(1회차·신규)", 0xc5eee7, [0x49,0x83,0xc6], 3, 1, 120),
    ("ld_vision_mem(기존)",      0xc63b87, [0x49,0x83,0xc4], 3, 1, 120),
    # ══ line_defense 1회차 신규 노브 ══
    *[(f"ld_around_range[{i}]", a, [0x48,0xc7,0x85], 7, 4, 80000)
      for i, a in enumerate([0xc5f2a8, 0xc5f6d7, 0xc5fbdc, 0xc62ec4, 0xc62fd4, 0xc633a5, 0xc63488])],
    *[(f"ld_around_delay[{i}]", a, [0x48,0xc7,0x85], 7, 4, 5)
      for i, a in enumerate([0xc5f2b3, 0xc5f6e2, 0xc5fbe7])],
    ("ld_mode_mask[0]", 0xc5e61a, [0xba], 1, 4, 0x1a1),
    ("ld_mode_mask[1]", 0xc5e9b3, [0x41,0xb8], 2, 4, 0x1a1),
    ("ld_mode_mask[2]", 0xc5f664, [0xba], 1, 4, 0x1a1),
    *[(f"ld_move_pct[{i}]", a, [0x48,0x83,0xc0], 3, 1, 100)
      for i, a in enumerate([0xc5e31a, 0xc5e55e, 0xc5fe08, 0xc5fe56])],
    ("ld_threat_state", 0xc5e3cc, [0x48,0x83,0x7b,0x68], 4, 1, 13),
    ("ld_rand_min",     0xc60a0c, [0x48,0x83,0xfa], 3, 1, 2),
    # ══ 이동 점수 cat0/2/4 ══
    ("mv0_adv_hi", 0xc7c7ae, [0x41,0xb8], 2, 4, 300),
    ("mv0_adv_lo", 0xc7c7be, [0x41,0xb8], 2, 4, 40),
    ("mv0_risk_shift",    0xc7c7f5, [0x49,0xc1,0xf9], 3, 1, 2),
    ("mv0_engage_shift",  0xc7c803, [0x48,0xc1,0xfa], 3, 1, 9),
    ("mv0_base_penalty",  0xc7c80e, [0x49,0x83,0xc4], 3, 1, 0xfe),
    ("mv0_near_bonus",    0xc7d5a6, [0xbb], 1, 4, 10),
    ("mv0_near_gate",     0xc7d4f0, [0x48,0x81,0x7d,0x68], 4, 4, 950),
    ("mv_tower_margin[0]", 0xc7c664, [0x48,0x81,0xee], 3, 4, 30000),
    ("mv_tower_margin[1]", 0xc7ca21, [0x48,0x81,0xee], 3, 4, 30000),
    ("mv_tower_cap cmp[0]", 0xc7c8ed, [0x48,0x83,0xf8], 3, 1, 100),
    ("mv_tower_cap mov[0]", 0xc7c8f1, [0x41,0xbe], 2, 4, 100),
    ("mv_tower_cap cmp[1]", 0xc7cb06, [0x48,0x83,0xf8], 3, 1, 100),
    ("mv_tower_cap mov[1]", 0xc7cb0a, [0xb9], 1, 4, 100),
    ("mv_tower_cap cmp[2]", 0xc7d392, [0x48,0x83,0xf8], 3, 1, 100),
    ("mv_tower_cap mov[2]", 0xc7d396, [0x41,0xbc], 2, 4, 100),
    ("mv2_gain_shift", 0xc7d3d0, [0x48,0xc1,0xfa], 3, 1, 7),
    *[(f"mv_engage_thr[{i}]", a, [0x41,0xb8], 2, 4, 9999)
      for i, a in enumerate([0xc7c09b, 0xc7c3ea, 0xc7cb7f])],
    ("mv_vision_mem", 0xc7cd8a, [0x48,0x83,0xc7], 3, 1, 120),
    # ══ death_battle 전투 후보 생성기 ══
    *[(f"db_near_ally[{i}]", a, [0x48,0xb8], 2, 8, 0x53D1AC101)
      for i, a in enumerate([0xdafa40, 0xdafab6, 0xdafb2c, 0xdafba2, 0xdafc14])],
    *[(f"db_near_enemy[{i}]", a, [0x48,0xb8], 2, 8, 0x53D1AC101)
      for i, a in enumerate([0xdafce1, 0xdafd68, 0xdafdef, 0xdafe76, 0xdafefd])],
    *[(f"db_lookahead[{i}]", a, [0xb9], 1, 4, 30)
      for i, a in enumerate([0xdaff72, 0xdb09f5, 0xdb1de3, 0xdb2def])],
    ("db_lookahead[minion]", 0xdb05b0, [0x41,0xb9], 2, 4, 30),
    ("db_ult_lookahead[0]", 0xdb4aac, [0x48,0x6b,0x8d], 7, 1, 60),
    ("db_ult_lookahead[1]", 0xdb4c5a, [0x49,0x6b,0x83], 7, 1, 60),
    ("db_execute_hp", 0xdb2ed6, [0x48,0x83,0xf8], 3, 1, 20),
    ("db_lasthit",    0xdb04be, [0x48,0x83,0xf8], 3, 1, 2),
    ("db_skill_hp[0]", 0xdb11e6, [0x48,0x83,0xf8], 3, 1, 79),
    ("db_skill_hp[1]", 0xdb25a6, [0x48,0x83,0xf8], 3, 1, 79),
    ("db_ult_rally[0]", 0xdb43bd, [0xb9], 1, 4, 36000000),
    ("db_ult_rally[1]", 0xdb43d5, [0xb8], 1, 4, 36000000),
    ("db_ult_rally2",   0xdb43da, [0x48,0xb9], 2, 8, 0x1E2CC3100),
    ("db_ult_range",    0xdb441e, [0x48,0x3d], 2, 4, 150000),
    ("db_ult_mask_rally", 0xdb4699, [0xb9], 1, 4, 0x6f),
    ("db_ult_mask_focus", 0xdb492c, [0xb9], 1, 4, 0x4e),
    ("db_ult_mask_safe",  0xdb4953, [0xb9], 1, 4, 0x21),
    ("db_skill2_level", 0xdaf9ab, [0x48,0x83,0xfa], 3, 1, 3),
    ("db_ult_level",    0xdaf9cf, [0x48,0x83,0xfa], 3, 1, 5),
    ("db_safe_margin",  0xc8b99b, [0x48,0x05], 2, 4, 15000),
    *[(f"db_safe_radius[{i}]", a, [0x48,0xb8], 2, 8, 0x35A4E9000)
      for i, a in enumerate([0xc8baf9, 0xc8bbe6, 0xc8bcc2])],
    *[(f"db_safe_mem[{i}]", a, [0x48,0x83,0xc6], 3, 1, 120)
      for i, a in enumerate([0xc8bb5e, 0xc8bc4b, 0xc8bd27, 0xc8be03, 0xc8becc])],
]

# ── 바이트열 미확인 사이트: 후보 prefix 중 하나라도 맞으면 PASS ──
ANY = [("ld_vision_mem(c61667·미확인)", 0xc61667,
        [[0x49,0x83,0xc6],[0x49,0x83,0xc4],[0x49,0x83,0xc5],[0x48,0x83,0xc6],[0x48,0x83,0xc7]],
        3, 1, 120)]

# ── .rdata 테이블 ──
RDATA = [("mv0_adv_m1/0/p1 (.rdata 0x31AA4E8)", 0x31AA4E8, [75, 100, 200]),
         ("sc_adv_m1/0/p1  (.rdata 0x31AA500)", 0x31AA500, [60, 80, 150])]

fails, passes = [], 0
for key, rva, pre, off, w, want in CHECKS:
    got_pre = rd(rva, len(pre))
    if got_pre is None:
        fails.append((key, rva, "주소가 파일 범위 밖", "")); continue
    if list(got_pre) != pre:
        fails.append((key, rva, "prefix 불일치",
                      f"기대 {' '.join(f'{b:02x}' for b in pre)} / 실제 {got_pre.hex(' ')}")); continue
    got = imm(rva, off, w)
    if got != want:
        fails.append((key, rva, "원본값 불일치", f"기대 {want} / 실제 {got}")); continue
    passes += 1

for key, rva, pres, off, w, want in ANY:
    hit = None
    for pre in pres:
        g = rd(rva, len(pre))
        if g is not None and list(g) == pre and imm(rva, off, w) == want:
            hit = pre; break
    if hit: passes += 1
    else:
        cur = rd(rva, 8)
        fails.append((key, rva, "후보 prefix 전부 불일치",
                      f"실제 바이트 {cur.hex(' ') if cur else '?'}"))

for key, rva, wants in RDATA:
    got = [imm(rva + i * 8, 0, 8) for i in range(3)]
    if got == wants: passes += 1
    else: fails.append((key, rva, ".rdata 값 불일치", f"기대 {wants} / 실제 {got}"))

total = len(CHECKS) + len(ANY) + len(RDATA)
print(f"=== 정적 검증: {passes}/{total} PASS ===\n")
if fails:
    print(f"--- FAIL {len(fails)}건 ---")
    for key, rva, why, detail in fails:
        print(f"  [{why}] {key}  @0x{rva:x}")
        if detail: print(f"      {detail}")
else:
    print("전 항목 PASS — prefix·원본값이 exe와 정확히 일치합니다.")
