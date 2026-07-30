# -*- coding: utf-8 -*-
# vtslot9_053.py — 잔여 복수후보 배정 + 후보0 해소.
#   vtslot8 잔여 = 코드가 **완전히 동일한 CGU 복제본**이 여럿인 경우.
#   이 함수들(descvt_child 어댑터 등)은 코드가 같고 **어느 vtable 슬롯에 등재됐는지**로 역할(which)이 갈린다
#   ⟹ 후보별 "참조 위치의 슬롯 오프셋 분포"를 구해 소스 which 와 대조해 배정한다.
#   슬롯 오프셋 산출 = 참조 위치에서 뒤로 스캔하며 vtable 헤더(코드ptr, size, align) 탐지(vtslot4 방식).
#   후보0(0x1dce1d0)은 니모닉 시퀀스 지문으로 완화 검색.
import struct, sys, io, re, json, collections
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8")
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
md = Cs(CS_ARCH_X86, CS_MODE_64)

ROOT = r"C:\Users\dev\Desktop\claude\tfm2"
BASES = {"0.5.1": "tfm2_0.5.1", "0.5.0_3": "tfm2_0.5.0_3", "0.5.0_2": "tfm2_0.5.0_2"}
TARGET = "tfm2_0.5.3"

# vtslot8 잔여 (소스RVA, 베이스, 소스에서의 역할 which, 소스줄)
TODO = [
    (0x19ec2c0, "0.5.0_3", 0x78, 1406, "descvt_any_at(obj,0x08,0x10,0x10,0x78) — pred composite"),
    (0x1e65a80, "0.5.0_3", 0x78, 1418, "delegate {data@obj+0x18, vt@obj+0x20}"),
    (0x1e66f40, "0.5.0_3", 0x78, 1412, "any(): ptr@+0x50/len@+0x58 stride 0x18"),
    (0x1eacc00, "0.5.0_3", 0x78, 1413, "delegate 내부 fat ptr {data@obj+0, vt@obj+8}"),
    (0x23a4f60, "0.5.1", 0x58, 1492, "descvt_child(obj+0x18/0x20, which=0x58)"),
    (0x23a4f80, "0.5.1", 0x50, 1476, "descvt_child(obj+0x18/0x20, which=0x50)"),
    (0x1dce1d0, "0.5.1", None, 270, "terminal: flat+ratio*stat/100 (후보0)"),
    # vtslot7 복수후보 2종(같은 아암)
    (0x1f23a60, "0.5.1", 0x28, 263, "챔피언 16B walker(primary)"),
    (0x1d204c0, "0.5.1", 0x28, 263, "챔피언 16B walker(secondary 복제)"),
]

BR = re.compile(r"^(j\w+|call|loop\w*)$")
RIP = re.compile(r"\[rip [+\-] 0x[0-9a-f]+\]")
HEX = re.compile(r"0x[0-9a-f]+")


def load(name):
    d = open(rf"{ROOT}\{name}\TeamfightManager2.exe", "rb").read()
    pe = struct.unpack_from("<I", d, 0x3c)[0]
    nsec = struct.unpack_from("<H", d, pe + 6)[0]
    opt = pe + 24
    ib = struct.unpack_from("<Q", d, opt + 24)[0]
    sectab = opt + struct.unpack_from("<H", d, pe + 20)[0]
    secs = []
    for i in range(nsec):
        o = sectab + i * 40
        nm = d[o:o + 8].rstrip(b"\0").decode(errors="replace")
        vsz, va, rsz, rraw = struct.unpack_from("<IIII", d, o + 8)
        secs.append((nm, va, vsz, rraw, rsz))
    txt = [s for s in secs if s[0] == ".text"][0]
    return dict(name=name, d=d, ib=ib, secs=secs, tva=txt[1], tsz=max(txt[2], txt[4]))


def roff(secs, rva):
    for nm, va, vsz, rraw, rsz in secs:
        if va <= rva < va + max(vsz, rsz):
            o = rva - va
            return rraw + o if o < rsz else None
    return None


def is_code(E, v):
    return v > E["ib"] and E["tva"] <= (v - E["ib"]) < E["tva"] + E["tsz"]


def fp(E, rva, maxn=24, mode="strict"):
    o = roff(E["secs"], rva)
    if o is None:
        return None, ""
    out, hu = [], []
    for i in md.disasm(E["d"][o:o + 200], rva):
        ops = i.op_str
        if mode == "strict":
            s = RIP.sub("[rip+I]", ops)
            if BR.match(i.mnemonic):
                s = HEX.sub("I", s)
            out.append(f"{i.mnemonic} {s}".strip())
        elif mode == "loose":
            out.append(f"{i.mnemonic} {HEX.sub('I', ops)}".strip())
        else:                                   # mnem = 니모닉 시퀀스만
            out.append(i.mnemonic)
        hu.append(f"{i.mnemonic} {ops}".strip())
        if i.mnemonic in ("ret", "jmp") or len(out) >= maxn:
            break
    return (" | ".join(out) if out else None), "; ".join(hu)


def refs_of(E, rva):
    """데이터 섹션에서 이 함수를 가리키는 8B 정렬 위치 전수 → [데이터 rva]"""
    pat = struct.pack("<Q", E["ib"] + rva)
    out = []
    for nm, va, vsz, rraw, rsz in E["secs"]:
        if nm not in (".rdata", ".data"):
            continue
        blob = E["d"][rraw:rraw + rsz]
        i = 0
        while True:
            i = blob.find(pat, i)
            if i < 0:
                break
            if (va + i) % 8 == 0:
                out.append(va + i)
            i += 1
    return out


def slot_off(E, at):
    """참조 위치 at 에서 뒤로 스캔해 vtable 헤더를 찾고 슬롯 오프셋 산출."""
    o = roff(E["secs"], at)
    if o is None:
        return None
    for back in range(0, 0x600, 8):
        p = o - back
        if p < 0:
            break
        a = struct.unpack_from("<Q", E["d"], p)[0]
        b = struct.unpack_from("<Q", E["d"], p + 8)[0]
        c = struct.unpack_from("<Q", E["d"], p + 16)[0]
        if is_code(E, a) and b < 0x10000 and 0 < c <= 64 and (c & (c - 1)) == 0:
            return back
    return None


def slot_profile(E, rva):
    c = collections.Counter()
    for at in refs_of(E, rva):
        c[slot_off(E, at)] += 1
    return c


E = {k: load(v) for k, v in BASES.items()}
ET = load(TARGET)

# 0.5.3 지문 색인(함수포인터 풀)
print("0.5.3 함수포인터 풀·지문 색인 중...", file=sys.stderr)
pool = collections.Counter()
for nm, va, vsz, rraw, rsz in ET["secs"]:
    if nm not in (".rdata", ".data"):
        continue
    p = rraw + ((-va) % 8)
    while p + 8 <= rraw + rsz:
        v = struct.unpack_from("<Q", ET["d"], p)[0]
        if is_code(ET, v):
            pool[v - ET["ib"]] += 1
        p += 8
IDX = {"strict": collections.defaultdict(list), "loose": collections.defaultdict(list),
       "mnem": collections.defaultdict(list)}
for fn in pool:
    for m in ("strict", "loose", "mnem"):
        k, _ = fp(ET, fn, mode=m)
        if k:
            IDX[m][k].append(fn)

for rva, bs, which, ln, desc in TODO:
    EB = E[bs]
    print("\n" + "=" * 130)
    print(f"0x{rva:x} [{bs}] L{ln}  which={hex(which) if which else '?'}  {desc}")
    print("=" * 130)
    prof_old = slot_profile(EB, rva)
    print(f"  베이스 슬롯분포: " + " ".join(f"{hex(k) if k is not None else '?'}×{v}" for k, v in prof_old.most_common()))
    cands, mode = [], None
    for m in ("strict", "loose", "mnem"):
        k, hu = fp(EB, rva, mode=m)
        if k and IDX[m].get(k):
            cands, mode = IDX[m][k], m
            break
    _, hu = fp(EB, rva)
    print(f"  베이스 코드: {hu[:118]}")
    print(f"  후보 {len(cands)}종 (지문={mode})")
    hit = []
    for f in sorted(cands, key=lambda f: -pool[f]):
        pr = slot_profile(ET, f)
        s = " ".join(f"{hex(k) if k is not None else '?'}×{v}" for k, v in pr.most_common())
        ok = (which is not None and pr.get(which))
        if ok:
            hit.append(f)
        print(f"     {'★' if ok else ' '}0x{f:<10x} 참조={pool[f]:<4} 슬롯 {s}")
    if which is not None:
        print(f"  ⟹ which=0x{which:x} 에 등재된 후보 = {len(hit)}종: " + ", ".join(hex(f) for f in hit))
