# -*- coding: utf-8 -*-
# apply_bp_053.py — 재핀된 byte-patch 사이트 주소를 detour.rs 에 실제로 반영.
#   patch_imm_bytes(base + 0xOLD, ...) 의 0xOLD 를 0x NEW 로 치환하고, 미해결분은 값을 건드리지 않고
#   주석으로 "0.5.3 미해결 = prefix 검증 실패로 skip(fail-safe)" 만 남긴다.
import re, io, sys, json

SRC = r"C:\tfm2mods\tfm2_ai_adjust\src\detour.rs"

# 확정 매핑 (bytepatch_053 / bytepatch2 / bytepatch3 결과)
MAP = {
    # dn 클러스터 A (컨테이너 0x1b92e40 → 0xdec6b0)
    0x1b934a4: 0xdecd00, 0x1b934b0: 0xdecd0c, 0x1b934ec: 0xdecd48, 0x1b9351c: 0xdecd78,
    0x1b9302c: 0xdec8a1, 0x1b93152: 0xdec9cd, 0x1b933d8: 0xdecc38,
    # dn 클러스터 B (0x1bdaaa0 → 0xdf9320)
    0x1bdac25: 0xdf94a5, 0x1bdac95: 0xdf9513,
    # an 클러스터 (0x2376320 → 0xd94d00)
    0x23777fe: 0xd960e8, 0x237780a: 0xd960f4,
    # generic_build (0x22b2280 → 0xe06c10)
    0x22b2555: 0xe06e22, 0x22b2ca5: 0xe07610, 0x22b2bb1: 0xe075c9, 0x22b58ad: 0xe0a328,
    0x22b43ae: 0xe08858,
    # reach cap (0x23ad980 → 0xcdd010) / (0x23ba8d0 → 0xcdfec0)
    0x23ad9d7: 0xcdd067, 0x23ba8f3: 0xcdfeed,
    # sev 클러스터 #1 (0x22dd9a0 → 0xcc9d70)
    0x22e3cdf: 0xcd103f, 0x22e3cf0: 0xcd1050, 0x22e3cf6: 0xcd1056, 0x22e3d00: 0xcd1060,
    0x22e3d06: 0xcd1066, 0x22e3d10: 0xcd1070, 0x22e3d16: 0xcd1076, 0x22e3d2b: 0xcd108b,
    0x22e3d2f: 0xcd108f, 0x22e3d33: 0xcd1093,
    # sev 클러스터 #2 (0x22e6460 → 0xd159f0) ★rel32→rel8 축소로 tr17 오프셋이 +28→+24
    0x22edb5f: 0xd1af8f, 0x22edb65: 0xd1af95, 0x22edb6b: 0xd1af9b, 0x22edb71: 0xd1afa1,
    0x22edb7b: 0xd1afa7,
    # sev 클러스터 #3 (0x22efed0 → 0xcd4b40)
    0x22effff: 0xcd4c6f, 0x22f0005: 0xcd4c75, 0x22f000b: 0xcd4c7b, 0x22f0011: 0xcd4c81,
    0x22f0017: 0xcd4c87, 0x22f001d: 0xcd4c8d, 0x22f0023: 0xcd4c93,
    # sev 클러스터 #4 (0x23a04d0 → 0xc7f640)
    0x23a0c21: 0xc82224, 0x23a0c27: 0xc8222a, 0x23a0c2d: 0xc82230, 0x23a0c33: 0xc82236,
    0x23a0c39: 0xc8223c, 0x23a0c41: 0xc82244, 0x23a0c47: 0xc8224a,
    # disc19 severity (0x2380820 → 0xdece30)
    0x2380e16: None, 0x2380e1c: None, 0x2380e22: None, 0x2380e28: None,
    0x2380e2e: None, 0x2380e3c: None,
    # vis_window
    0x2126ae3: 0x2558d08,
}
# 미해결 = 값 유지 + 주석. (prefix 검증 실패 → patch_imm_bytes 가 false 반환하고 skip)
UNRESOLVED = {
    0x2376e86: "an_cull_dist(disc18): 0.5.3 전역에 imm 0x5f5e0 사이트가 2→1로 줄고 남은 1건은 disc18 후계 밖(fn 0x12c4b70, cmp rdi) = 동일 노브 아님",
    0x2398342: "d19b_hp_low: 시그 `49 83 be b8 00 00 00 1f` 가 0.5.3 전역 0건 = 코드 변경",
    0x2398ef3: "d19b_near1: movabs 0x35a4e9001 동반 시그 0.5.3 전역 0건",
    0x2398f3c: "d19b_near2: 동상",
}

src = open(SRC, encoding="utf-8").read()
orig = src
done, skipped, missing = [], [], []

for old, new in MAP.items():
    pat = re.compile(r'(patch_imm_bytes\(\s*base\s*\+\s*)0x%x\b' % old)
    if new is None:
        continue
    src2, n = pat.subn(lambda m: m.group(1) + hex(new), src)
    if n:
        src = src2
        done.append((old, new, n))
    else:
        missing.append(old)

for old, why in UNRESOLVED.items():
    pat = re.compile(r'(patch_imm_bytes\(\s*base\s*\+\s*0x%x\b[^\n]*)' % old)
    m = pat.search(src)
    if m and "0.5.3 미해결" not in m.group(1):
        src = pat.sub(lambda mm: mm.group(1) + "   // ⛔0.5.3 미해결(값=0.5.2 유지) — " + why +
                      ". prefix 불일치로 patch_imm_bytes 가 skip = fail-safe.", src, count=1)
        skipped.append(old)

open(SRC, "w", encoding="utf-8").write(src)
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8")
print(f"치환 {len(done)}건 / 미해결주석 {len(skipped)}건 / 소스에 없음 {len(missing)}건")
for o in missing:
    print(f"   ⚠ 0x{o:x} 소스에서 못 찾음")
print("변경 여부:", src != orig)
