# -*- coding: utf-8 -*-
# shadowcall2_053.py — shadowcall_053 잔여 4종. 전체 스켈레톤이 불일치(본문 변경)하므로 조건을 단계적으로 완화한다.
#   단계: 앞 24명령 → 16 → 12 지문 일치, 그 위에 크기 ±20% · 콜러 수 ±30% 로 랭킹.
#   ⚠완화 매칭은 근거가 약하다 ⟹ 결과는 "후보"이고, 채택하려면 콜러 함수 대응 등 2차 근거가 필요하다.
import struct, sys, io, re, collections
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8")
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
md = Cs(CS_ARCH_X86, CS_MODE_64)

ROOT = r"C:\Users\dev\Desktop\claude\tfm2"
TARGETS = [
    (0x20a3fd0, "tfm2_0.5.0_3", "L472 fn(6인자)->u64"),
    (0x1c974a0, "tfm2_0.5.0_3", "L636 슬롯 비교대상(콜러0=vtable 전용)"),
    (0x237d910, "tfm2_0.5.0_3", "L2393 F237d"),
    (0x236b6b0, "tfm2_0.5.1", "L2821 Fn2090"),
]
TGT = "tfm2_0.5.3"
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
    E = dict(d=d, ib=ib, secs=secs, rng=rng, starts=sorted(rng),
             tva=txt[1], tsz=max(txt[2], txt[4]))
    # 함수포인터 풀(콜러 0 인 vtable 전용 함수를 위해)
    pool = collections.Counter()
    for s, va, vsz, rr, rsz in secs:
        if s not in (".rdata", ".data"):
            continue
        p = rr + ((-va) % 8)
        while p + 8 <= rr + rsz:
            v = struct.unpack_from("<Q", d, p)[0]
            if v > ib and E["tva"] <= (v - ib) < E["tva"] + E["tsz"]:
                pool[v - ib] += 1
            p += 8
    E["pool"] = pool
    return E


def ro(E, r):
    for nm, va, vsz, rr, rsz in E["secs"]:
        if va <= r < va + max(vsz, rsz):
            o = r - va
            return rr + o if o < rsz else None
    return None


def head(E, r, n):
    o = ro(E, r)
    if o is None:
        return None
    out = []
    for i in md.disasm(E["d"][o:o + 24 * n + 40], r):
        s = RIP.sub("[rip+I]", i.op_str)
        if BR.match(i.mnemonic):
            s = HEX.sub("I", s)
        out.append(f"{i.mnemonic} {s}".strip())
        if len(out) >= n:
            break
    return " | ".join(out) if len(out) == n else None


def callers(E):
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


E = {v: load(v) for v in {t[1] for t in TARGETS} | {TGT}}
ET = E[TGT]
CN = callers(ET)
CB = {v: callers(E[v]) for v in {t[1] for t in TARGETS}}

print("0.5.3 head 색인 중...", file=sys.stderr)
IDX = {n: collections.defaultdict(list) for n in (24, 16, 12)}
for r in ET["starts"]:
    for n in (24, 16, 12):
        h = head(ET, r, n)
        if h:
            IDX[n][h].append(r)

for rva, bs, desc in TARGETS:
    EB = E[bs]
    sz = EB["rng"].get(rva, 0) - rva
    co = CB[bs].get(rva, 0)
    po = EB["pool"].get(rva, 0)
    print("\n" + "=" * 126)
    print(f"0x{rva:x} [{bs[5:]}] {desc}   크기={sz} 콜러={co} 함수ptr참조={po}")
    for n in (24, 16, 12):
        h = head(EB, rva, n)
        cands = IDX[n].get(h, []) if h else []
        if not cands:
            continue
        print(f"  ── 앞 {n}명령 일치: {len(cands)}종")
        rows = []
        for f in cands:
            fsz = ET["rng"].get(f, 0) - f
            fc, fp = CN.get(f, 0), ET["pool"].get(f, 0)
            ok_sz = sz and abs(fsz - sz) <= max(24, sz * 0.20)
            ok_co = abs(fc - co) <= max(3, co * 0.30)
            ok_po = abs(fp - po) <= max(2, po * 0.30)
            rows.append((ok_sz + ok_co + ok_po, f, fsz, fc, fp, ok_sz, ok_co, ok_po))
        rows.sort(reverse=True)
        for sc, f, fsz, fc, fp, a, b, c in rows[:6]:
            print(f"     {'★' if sc >= 3 else ' '}0x{f:<10x} 크기={fsz:<7}{'✓' if a else '✗'} "
                  f"콜러={fc:<5}{'✓' if b else '✗'} fptr={fp:<4}{'✓' if c else '✗'}")
        break
    else:
        print("  → 12명령 지문으로도 후보 0")
