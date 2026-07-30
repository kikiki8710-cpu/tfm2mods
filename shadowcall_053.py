# -*- coding: utf-8 -*-
# shadowcall_053.py — `rvascan_053.py` 가 찾아낸 **shadow-CALL 대상 게임함수 RVA** 재핀.
#   대상은 vtable 슬롯이 아니라 `transmute(base + RVA)` 로 직접 호출하는 함수들이다.
#   현재 `code_ptr_ok()` 가드에 막혀 **크래시는 없고 조용히 기본값 반환**(= 기능 死) 상태.
#   매칭: ① 함수 전체 스켈레톤(imm/변위 보존, 분기·rip 마스킹) ② 크기 ③ 콜러 수.
import struct, sys, io, re, collections, bisect
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8")
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
md = Cs(CS_ARCH_X86, CS_MODE_64)

ROOT = r"C:\Users\dev\Desktop\claude\tfm2"
TARGETS = [   # (RVA, 베이스버전, 설명)
    (0x20a3fd0, "tfm2_0.5.0_3", "disc19_repro L472 shadow-call fn(6인자)->u64"),
    (0x1c974a0, "tfm2_0.5.0_3", "disc19_repro L636 슬롯 동일성 비교 대상"),
    (0x1fce700, "tfm2_0.5.0_3", "disc19_repro L733 D19Us"),
    (0x1fbe950, "tfm2_0.5.0_3", "disc19_repro L743 D19Us"),
    (0x237d910, "tfm2_0.5.0_3", "disc19_repro L2393 F237d"),
    (0x236b6b0, "tfm2_0.5.1",   "disc19_repro L2821 Fn2090 (0.5.1에서 한 번 재핀됨)"),
]
TARGET = "tfm2_0.5.3"
BR = re.compile(r"^(j\w+|call|loop\w*)$")
RIP = re.compile(r"\[rip [+\-] 0x[0-9a-f]+\]")
HEX = re.compile(r"0x[0-9a-f]+")


def load(nm):
    d = open(rf"{ROOT}\{nm}\TeamfightManager2.exe", "rb").read()
    pe = struct.unpack_from("<I", d, 0x3c)[0]
    opt = pe + 24
    ib = struct.unpack_from("<Q", d, opt + 24)[0]
    n = struct.unpack_from("<H", d, pe + 6)[0]
    st = opt + struct.unpack_from("<H", d, pe + 20)[0]
    secs = []
    for i in range(n):
        o = st + i * 40
        s = d[o:o + 8].rstrip(b"\0").decode("latin1")
        vsz, va, rsz, rr = struct.unpack_from("<IIII", d, o + 8)
        secs.append((s, va, vsz, rr, rsz))
    magic = struct.unpack_from("<H", d, opt)[0]
    ddir = opt + (112 if magic == 0x20b else 96)
    ex, ez = struct.unpack_from("<II", d, ddir + 3 * 8)
    po = None
    for s, va, vsz, rr, rsz in secs:
        if va <= ex < va + max(vsz, rsz):
            po = rr + (ex - va)
    rng = {}
    for i in range(ez // 12):
        b, e, _ = struct.unpack_from("<III", d, po + i * 12)
        if e > b and e - b < (1 << 20):
            rng[b] = max(rng.get(b, 0), e)
    txt = [s for s in secs if s[0] == ".text"][0]
    return dict(d=d, ib=ib, secs=secs, rng=rng, starts=sorted(rng),
                tva=txt[1], tsz=max(txt[2], txt[4]))


def ro(E, r):
    for nm, va, vsz, rr, rsz in E["secs"]:
        if va <= r < va + max(vsz, rsz):
            o = r - va
            return rr + o if o < rsz else None
    return None


def skel(E, r, maxn=60):
    o = ro(E, r)
    if o is None:
        return None, 0, ""
    end = E["rng"].get(r, r + 400)
    out, hu = [], []
    for i in md.disasm(E["d"][o:o + min(4000, end - r + 8)], r):
        if i.address >= end:
            break
        s = RIP.sub("[rip+I]", i.op_str)
        if BR.match(i.mnemonic):
            s = HEX.sub("I", s)
        out.append(f"{i.mnemonic} {s}".strip())
        hu.append(f"{i.mnemonic} {i.op_str}".strip())
        if len(out) >= maxn:
            break
    return (" | ".join(out) if out else None), end - r, "; ".join(hu[:6])


def callers(E):
    tva = E["tva"]
    for nm, va, vsz, rr, rsz in E["secs"]:
        if nm == ".text":
            break
    blob = E["d"][rr:rr + rsz]
    c = collections.Counter()
    i = 0
    while True:
        i = blob.find(b"\xe8", i)
        if i < 0 or i + 5 > len(blob):
            break
        rel = int.from_bytes(blob[i + 1:i + 5], "little", signed=True)
        t = va + i + 5 + rel
        if va <= t < va + rsz:
            c[t] += 1
        i += 1
    return c


E = {}
for v in {t[1] for t in TARGETS} | {TARGET}:
    E[v] = load(v)
print("콜러 스캔 중...", file=sys.stderr)
CN = callers(E[TARGET])
CB = {v: callers(E[v]) for v in {t[1] for t in TARGETS}}

print("0.5.3 스켈레톤 색인 중...", file=sys.stderr)
IDX = collections.defaultdict(list)
ET = E[TARGET]
for r in ET["starts"]:
    s, sz, _ = skel(ET, r)
    if s:
        IDX[s].append((r, sz))

for rva, bs, desc in TARGETS:
    EB = E[bs]
    s, sz, hu = skel(EB, rva)
    cands = IDX.get(s, [])
    print("\n" + "=" * 128)
    print(f"0x{rva:x} [{bs[5:]}] {desc}")
    print(f"   크기={sz}  콜러={CB[bs].get(rva,0)}  {hu[:100]}")
    if not cands:
        print("   → 후보 0 (전체 스켈레톤 불일치 = 본문 변경)")
        continue
    print(f"   → 후보 {len(cands)}종")
    for f, fsz in sorted(cands, key=lambda x: -CN.get(x[0], 0))[:6]:
        mark = "★" if (fsz == sz and abs(CN.get(f, 0) - CB[bs].get(rva, 0)) <= 2) else " "
        print(f"      {mark}0x{f:<10x} 크기={fsz:<7} 콜러={CN.get(f,0)}")
