# -*- coding: utf-8 -*-
# vtslot_diff_053.py — 확정 매핑쌍의 **전체 함수 명령 대조**.
#   왜: 매핑은 "앞 24명령 지문"으로 잡았다. 그런데 실측 2건에서 **명령열은 같고 구조체 필드 오프셋만 시프트**된 사례가 나왔다
#       (0x1dce1d0→0xf14c60 = -0x10 / 0x1d1f630→0xf01df0 = -0x18).
#       ⟹ 주소만 갈아끼우면 **재현 로직이 엉뚱한 필드를 읽는다**. 쌍마다 disp 차이를 전수로 뽑아야 한다.
#   출력: 니모닉열이 어긋나는 지점 / disp(메모리 변위)가 다른 지점 목록.
import struct, sys, io, re, json, collections
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8")
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
md = Cs(CS_ARCH_X86, CS_MODE_64)

ROOT = r"C:\Users\dev\Desktop\claude\tfm2"
PAIRS = [   # (구 RVA, 베이스, 신 RVA)
    (0x50fc80, "0.5.0_3", 0x9d2a0), (0x5418a0, "0.5.0_3", 0xc04f0),
    (0x9a1230, "0.5.0_3", 0xf45090), (0x9c8850, "0.5.0_3", 0xf451d0),
    (0xb024b0, "0.5.0_3", 0xff71b0), (0x19ed250, "0.5.0_3", 0xfe5c20),
    (0x19ed260, "0.5.0_3", 0xfe5230), (0x19f2f60, "0.5.0_3", 0xee24c0),
    (0x1a13cb0, "0.5.0_3", 0x2f840), (0x1a3a240, "0.5.0_3", 0xf025c0),
    (0x1e85540, "0.5.0_3", 0x1269e60),
    (0x1a5ee60, "0.5.1", 0x1092130), (0x1a671e0, "0.5.1", 0xf01ff0),
    (0x1ce1070, "0.5.1", 0xfe3cf0), (0x1ce1090, "0.5.1", 0xfe3d10),
    (0x1ce10f0, "0.5.1", 0x1254e30), (0x1d1ed70, "0.5.1", 0x12100f0),
    (0x1d1edd0, "0.5.1", 0x1210150), (0x1f23680, "0.5.1", 0x11fe400),
    (0x1f236f0, "0.5.1", 0x1558f10), (0x1f23d30, "0.5.1", 0x11fe750),
    (0x1f23d70, "0.5.1", 0x11fe790), (0x1f23dd0, "0.5.1", 0x11fe7f0),
    (0x1f23eb0, "0.5.1", 0x11fe8d0), (0x1f23f90, "0.5.1", 0x11fe9b0),
    (0x1f77e30, "0.5.1", 0x1228160), (0x1faac80, "0.5.1", 0xfe3cd0),
    (0x1fabac0, "0.5.1", 0xfe3d80), (0x1ff1970, "0.5.1", 0x127d9a0),
    (0x2291570, "0.5.1", 0x12c9a50), (0x23a49f0, "0.5.1", 0x11fe750),
    (0x23b5770, "0.5.1", 0x120fcb0), (0x23b5790, "0.5.1", 0x120fcd0),
    (0x23b5890, "0.5.1", 0x120fdd0), (0x23bd3d0, "0.5.1", 0x12282b0),
    (0x23bd430, "0.5.1", 0x1228310), (0x1d1f630, "0.5.1", 0xf01df0),
    (0x1bbe3c0, "0.5.0_3", 0x13b4ea0), (0x1d328e0, "0.5.1", 0xf15710),
    (0x20958d0, "0.5.1", 0x12d3be0), (0x23a4d90, "0.5.1", 0x11ff4a0),
    (0x23a5080, "0.5.1", 0x11ff790), (0x23bd370, "0.5.1", 0x1228250),
    (0x23a4f60, "0.5.1", 0x11ff670), (0x23a4f80, "0.5.1", 0x11ff690),
    (0x1f23a60, "0.5.1", 0x11fe580), (0x1d204c0, "0.5.1", 0x1203390),
    (0x1dce1d0, "0.5.1", 0xf14c60),
]
BASES = {"0.5.1": "tfm2_0.5.1", "0.5.0_3": "tfm2_0.5.0_3", "0.5.0_2": "tfm2_0.5.0_2"}
TARGET = "tfm2_0.5.3"
DISP = re.compile(r"\[(\w+) ([+\-]) (0x[0-9a-f]+)\]")


def load(name):
    d = open(rf"{ROOT}\{name}\TeamfightManager2.exe", "rb").read()
    pe = struct.unpack_from("<I", d, 0x3c)[0]
    opt = pe + 24
    nsec = struct.unpack_from("<H", d, pe + 6)[0]
    sectab = opt + struct.unpack_from("<H", d, pe + 20)[0]
    secs = []
    for i in range(nsec):
        o = sectab + i * 40
        nm = d[o:o + 8].rstrip(b"\0").decode("latin1")
        vsz, va, rsz, rraw = struct.unpack_from("<IIII", d, o + 8)
        secs.append((nm, va, vsz, rraw, rsz))
    magic = struct.unpack_from("<H", d, opt)[0]
    ddir = opt + (112 if magic == 0x20b else 96)
    ex, ez = struct.unpack_from("<II", d, ddir + 3 * 8)
    po = None
    for nm, va, vsz, rraw, rsz in secs:
        if va <= ex < va + max(vsz, rsz):
            po = rraw + (ex - va)
    fns = sorted({struct.unpack_from("<III", d, po + i * 12)[0] for i in range(ez // 12)})
    return d, secs, fns


def ro(secs, r):
    for nm, va, vsz, rraw, rsz in secs:
        if va <= r < va + max(vsz, rsz):
            o = r - va
            return rraw + o if o < rsz else None
    return None


import bisect


def body(d, secs, fns, r, maxn=1200):
    """함수 끝 = ① 다음 .pdata 함수 시작(있으면) ② 없으면 ret + 패딩(int3/nop).
       ⚠슬롯 함수는 leaf 가 많아 .pdata 에 없다 ⟹ 두 기준을 함께 쓴다."""
    o = ro(secs, r)
    if o is None:
        return []
    j = bisect.bisect_right(fns, r)
    limit = fns[j] if j < len(fns) else r + 6000       # 다음 함수 시작
    ins = []
    for i in md.disasm(d[o:o + min(24000, (limit - r) + 16)], r):
        if i.address >= limit:
            break
        ins.append(i)
        if len(ins) >= maxn:
            break
        if i.mnemonic == "ret":
            nxt = d[o + (i.address - r) + i.size]
            if nxt in (0xcc, 0x90, 0x00):
                break
    return ins


E = {k: load(v) for k, v in BASES.items()}
DT, ST, FT = load(TARGET)

nsame = nshift = ndiff = 0
print(f"{'구 RVA':<12}{'→ 0.5.3':<12}{'명령수':<12}{'판정'}")
print("=" * 120)
report = []
for old, bs, new in PAIRS:
    db, sb, fb = E[bs]
    A, B = body(db, sb, fb, old), body(DT, ST, FT, new)
    la, lb = len(A), len(B)
    mism = [i for i in range(min(la, lb)) if A[i].mnemonic != B[i].mnemonic]
    dch = []
    for i in range(min(la, lb)):
        da, dbb = DISP.findall(A[i].op_str), DISP.findall(B[i].op_str)
        if len(da) == len(dbb):
            for x, y in zip(da, dbb):
                if x != y:
                    dch.append((i, A[i].mnemonic, f"{x[0]}{x[1]}{x[2]}", f"{y[0]}{y[1]}{y[2]}"))
        elif da or dbb:
            dch.append((i, A[i].mnemonic, str(da), str(dbb)))
    if mism or la != lb:
        verdict = f"⚠명령열 불일치(첫 {mism[0] if mism else min(la,lb)}번째, {la}↔{lb}명령)"
        ndiff += 1
    elif dch:
        verdict = f"★필드 오프셋 변경 {len(dch)}곳"
        nshift += 1
    else:
        verdict = "완전동일"
        nsame += 1
    print(f"0x{old:<10x}0x{new:<10x}{f'{la}↔{lb}':<12}{verdict}")
    if dch:
        seen = set()
        for i, mn, x, y in dch[:10]:
            k = (x, y)
            if k in seen:
                continue
            seen.add(k)
            print(f"      #{i:<4}{mn:<8}{x:>14}  →  {y}")
    report.append(dict(old=hex(old), new=hex(new), base=bs, na=la, nb=lb,
                       verdict=verdict, disp=[[i, mn, x, y] for i, mn, x, y in dch]))

print(f"\n요약: 완전동일 {nsame} / 필드오프셋 변경 {nshift} / 명령열 불일치 {ndiff}")
json.dump(report, open(r"C:\tfm2mods\_vtslot_053_diff.json", "w", encoding="utf-8"),
          ensure_ascii=False, indent=1)
print("→ _vtslot_053_diff.json 저장")
