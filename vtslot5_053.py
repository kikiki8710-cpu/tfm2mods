# -*- coding: utf-8 -*-
# vtslot5_053.py — 슬롯 함수 매핑 2단계: **vtable 전수 열거 + 슬롯 분포 대조**.
#   vtslot4 결과: 소스의 which(0x48/0x50/0x58/0x78/0x90/0xc8/0xd8/0x110)가 실제 vtable 슬롯 오프셋과 일치.
#   여기서 두 exe(베이스 ↔ 0.5.3)의 vtable 을 전수 열거하고, 슬롯 오프셋별
#   "함수 → 등장 vtable 수" 분포를 뽑아 나란히 찍는다.
#   ⟹ 분포 순위 + 함수 바이트 구조로 1:1 대응을 잡는 것이 목표.
#   ⚠0.5.3 은 vtable 메서드 삽입으로 슬롯이 밀렸을 수 있다(§12.11 vt+0x1b8→0x1c8 실례) ⟹ 오프셋 고정 금지, 분포로 확인.
import struct, sys, io, re, collections
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8")
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
md = Cs(CS_ARCH_X86, CS_MODE_64)

EXES = {
    "0.5.1": r"C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.1\TeamfightManager2.exe",
    "0.5.3": r"C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.3\TeamfightManager2.exe",
}


def load(p):
    d = open(p, "rb").read()
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
    return dict(d=d, ib=ib, secs=secs)


def roff(secs, rva):
    for nm, va, vsz, rraw, rsz in secs:
        if va <= rva < va + max(vsz, rsz):
            o = rva - va
            return rraw + o if o < rsz else None
    return None


def enum_vtables(E):
    """Rust vtable = [drop_in_place(코드ptr), size, align, method...]
       .rdata 를 8B 정렬로 훑어 헤더 패턴을 찾고, 이어지는 코드포인터를 슬롯으로 수집."""
    d, ib, secs = E["d"], E["ib"], E["secs"]
    txt = [s for s in secs if s[0] == ".text"][0]
    tva, tsz = txt[1], max(txt[2], txt[4])

    def is_code(v):
        return v > ib and tva <= (v - ib) < tva + tsz

    vts = []
    for nm, va, vsz, rraw, rsz in secs:
        if nm not in (".rdata", ".data"):
            continue
        end = rraw + rsz
        p = rraw + ((-va) % 8)
        while p + 24 <= end:
            a, b, c = (struct.unpack_from("<Q", d, p)[0],
                       struct.unpack_from("<Q", d, p + 8)[0],
                       struct.unpack_from("<Q", d, p + 16)[0])
            if is_code(a) and b < 0x10000 and 0 < c <= 64 and (c & (c - 1)) == 0:
                # 슬롯 수집: 0x18 부터 연속 코드포인터 (null 1개까지는 허용)
                slots = {}
                q = p + 24
                off = 0x18
                miss = 0
                while q + 8 <= end and off < 0x600:
                    v = struct.unpack_from("<Q", d, q)[0]
                    if is_code(v):
                        slots[off] = v - ib
                        miss = 0
                    else:
                        miss += 1
                        if miss >= 2:
                            break
                    q += 8
                    off += 8
                if len(slots) >= 4:
                    vts.append((va + (p - rraw), a - ib, b, c, slots))
                    p = q
                    continue
            p += 8
    return vts


def entry_bytes(E, rva, n=16):
    o = roff(E["secs"], rva)
    return E["d"][o:o + n] if o is not None else b""


def dis(E, rva, maxn=4):
    o = roff(E["secs"], rva)
    if o is None:
        return "(범위밖)"
    out = []
    for i in md.disasm(E["d"][o:o + 40], rva):
        out.append(f"{i.mnemonic} {i.op_str}".strip())
        if i.mnemonic in ("ret", "jmp") or len(out) >= maxn:
            break
    return "; ".join(out)


ES = {}
VT = {}
for k, p in EXES.items():
    ES[k] = load(p)
    VT[k] = enum_vtables(ES[k])
    print(f"{k}: vtable {len(VT[k]):,}개 열거")

# 슬롯 오프셋별 함수 분포
def slot_dist(vts):
    out = collections.defaultdict(collections.Counter)   # off -> Counter(fn_rva)
    for va, drop, size, align, slots in vts:
        for off, fn in slots.items():
            out[off][fn] += 1
    return out


DIST = {k: slot_dist(VT[k]) for k in ES}

WHICH = [0x28, 0x48, 0x50, 0x58, 0x78, 0x90, 0xc8, 0xd8, 0x110]
for off in WHICH:
    a = DIST["0.5.1"].get(off, collections.Counter())
    b = DIST["0.5.3"].get(off, collections.Counter())
    print("\n" + "=" * 132)
    print(f"슬롯 +0x{off:x} — 0.5.1: 고유함수 {len(a)} / 총 {sum(a.values())} vtable   |   "
          f"0.5.3: 고유함수 {len(b)} / 총 {sum(b.values())} vtable")
    print("=" * 132)
    print(f"{'0.5.1 함수':<12}{'수':<6}{'프롤로그':<52}| {'0.5.3 함수':<12}{'수':<6}{'프롤로그'}")
    la, lb = a.most_common(14), b.most_common(14)
    for i in range(max(len(la), len(lb))):
        if i < len(la):
            f1, c1 = la[i]
            s1 = f"0x{f1:<10x}{c1:<6}{dis(ES['0.5.1'], f1, 3)[:50]:<52}"
        else:
            s1 = " " * 70
        if i < len(lb):
            f2, c2 = lb[i]
            s2 = f"0x{f2:<10x}{c2:<6}{dis(ES['0.5.3'], f2, 3)[:50]}"
        else:
            s2 = ""
        print(f"{s1}| {s2}")
