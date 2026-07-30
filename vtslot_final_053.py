# -*- coding: utf-8 -*-
# vtslot_final_053.py — vtslot7/8/9 결과를 합쳐 **최종 매핑표**를 만들고 충돌·정합성을 검사한다.
#   출력: _vtslot_053_final.md (인계용 표) + 콘솔 검증
import json, collections, sys, io, struct, re
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8")

# ── 확정 매핑(소스RVA → 0.5.3 RVA 리스트) ───────────────────────
# vtslot7 strict/loose 유일
M = {
    0x50fc80: [0x9d2a0], 0x5418a0: [0xc04f0], 0x9a1230: [0xf45090], 0x9c8850: [0xf451d0],
    0xb024b0: [0xff71b0], 0x19ed250: [0xfe5c20], 0x19ed260: [0xfe5230], 0x19f2f60: [0xee24c0],
    0x1a13cb0: [0x2f840], 0x1a3a240: [0xf025c0], 0x1a5ee60: [0x1092130], 0x1a671e0: [0xf01ff0],
    0x1ce1070: [0xfe3cf0], 0x1ce1090: [0xfe3d10], 0x1ce10f0: [0x1254e30], 0x1d1ed70: [0x12100f0],
    0x1d1edd0: [0x1210150], 0x1e85540: [0x1269e60], 0x1f23680: [0x11fe400], 0x1f236f0: [0x1558f10],
    0x1f23d30: [0x11fe750], 0x1f23d70: [0x11fe790], 0x1f23dd0: [0x11fe7f0], 0x1f23eb0: [0x11fe8d0],
    0x1f23f90: [0x11fe9b0], 0x1f77e30: [0x1228160], 0x1faac80: [0xfe3cd0], 0x1fabac0: [0xfe3d80],
    0x1ff1970: [0x127d9a0], 0x2291570: [0x12c9a50], 0x23a49f0: [0x11fe750], 0x23b5770: [0x120fcb0],
    0x23b5790: [0x120fcd0], 0x23b5890: [0x120fdd0], 0x23bd3d0: [0x12282b0], 0x23bd430: [0x1228310],
    0x1d1f630: [0xf01df0],
    # vtslot8 유일
    0x1bbe3c0: [0x13b4ea0], 0x1d328e0: [0xf15710], 0x20958d0: [0x12d3be0], 0x23a4d90: [0x11ff4a0],
    0x23a5080: [0x11ff790], 0x23bd370: [0x1228250],
    # vtslot9 슬롯 배정
    0x23a4f60: [0x11ff670], 0x23a4f80: [0x11ff690],
    0x1f23a60: [0x11fe580, 0x1203390], 0x1d204c0: [0x11fe580, 0x1203390],
}
# 미해결(0.5.3 대응 없음 = 구세대 구현이 사라짐 / 불확실)
UNRESOLVED = {
    0x19ec2c0: "0.5.0_3 세대 pred composite. 0.5.1에서 이미 구현 교체됨(0x1f23eb0 계열이 후계) ⟹ 0.5.3 대응 없음",
    0x1e65a80: "0.5.0_3 세대 delegate. 0.5.3 동일지문 7종 중 슬롯 0x78 등재 0 ⟹ 대응 없음",
    0x1e66f40: "0.5.0_3 세대 any(ptr@0x50). 0.5.1 후계=0x1d1ed70(→0x12100f0) ⟹ 중복",
    0x1eacc00: "0.5.0_3 세대 delegate(fat ptr). 0.5.1 후계=0x23b5790 계열 ⟹ 중복",
    0x1dce1d0: "terminal(flat+ratio). 지문 니모닉만 일치하는 0xf14c60 1종 — ⚠미확정(슬롯 불일치)",
}

SRC = r"C:\tfm2mods\tfm2_ai_adjust\src\disc19_repro.rs"
EXE53 = r"C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.3\TeamfightManager2.exe"

# ── 검사 1: 신 RVA 중복 배정 ────────────────────────────────────
rev = collections.defaultdict(list)
for old, news in M.items():
    for n in news:
        rev[n].append(old)
dup = {n: o for n, o in rev.items() if len(o) > 1}
print("■ 검사1 — 같은 0.5.3 RVA 에 배정된 소스 아암이 2개 이상")
for n, olds in dup.items():
    print(f"  0x{n:x} ← " + ", ".join(hex(o) for o in olds))
print("  (소스가 OR 로 묶은 아암끼리면 정상)\n")

# ── 검사 2: 구 RVA 가 0.5.3 에서도 유효 코드인가(방치 시 오판정 위험) ──
d = open(EXE53, "rb").read()
pe = struct.unpack_from("<I", d, 0x3c)[0]
opt = pe + 24
ib = struct.unpack_from("<Q", d, opt + 24)[0]
nsec = struct.unpack_from("<H", d, pe + 6)[0]
sectab = opt + struct.unpack_from("<H", d, pe + 20)[0]
secs = []
for i in range(nsec):
    o = sectab + i * 40
    nm = d[o:o + 8].rstrip(b"\0").decode(errors="replace")
    vsz, va, rsz, rraw = struct.unpack_from("<IIII", d, o + 8)
    secs.append((nm, va, vsz, rraw, rsz))
txt = [s for s in secs if s[0] == ".text"][0]
tva, tsz = txt[1], max(txt[2], txt[4])
inside = [r for r in list(M) + list(UNRESOLVED) if tva <= r < tva + tsz]
print(f"■ 검사2 — 구 RVA {len(inside)}/{len(M)+len(UNRESOLVED)}종이 0.5.3 .text 범위 안에 있다")
print("  ⟹ 방치하면 '엉뚱한 함수를 우리 로직으로 해석'할 위험 ⟹ **교체(삭제+추가)가 정본**, 병기 금지\n")

# ── 인계표 ──────────────────────────────────────────────────────
src = open(SRC, encoding="utf-8").read().splitlines()
loc = {}
for ln, s in enumerate(src, 1):
    for m in re.finditer(r"0x([0-9a-f]{6,7})\s*(=>|\|)", s):
        loc.setdefault(int(m.group(1), 16), ln)

out = ["# disc19_repro.rs vtable 슬롯 RVA 재핀표 (→ 0.5.3)", "",
       "> 도구 = `vtslot7_053.py`(슬롯제약 지문매칭) · `vtslot8_053.py`(전역 지문) · `vtslot9_053.py`(슬롯 배정).",
       "> 베이스는 **버전 혼재**였다(0.5.1 34종 · 0.5.0_3 14종 · 0.5.0_2 1종) — 0.5.2 마이그에서 이 파일이 통째로 누락됐다는 뜻.", "",
       "| 소스줄 | 구 RVA | 베이스 | → 0.5.3 | 비고 |", "|---|---|---|---|---|"]
for old in sorted(M):
    news = " / ".join(f"`0x{n:x}`" for n in M[old])
    out.append(f"| L{loc.get(old,'?')} | `0x{old:x}` | - | {news} | |")
for old, why in UNRESOLVED.items():
    out.append(f"| L{loc.get(old,'?')} | `0x{old:x}` | - | ⛔없음 | {why} |")
open(r"C:\tfm2mods\_vtslot_053_final.md", "w", encoding="utf-8").write("\n".join(out))
print(f"■ 확정 {len(M)}종 / 미해결 {len(UNRESOLVED)}종 → _vtslot_053_final.md 저장")
