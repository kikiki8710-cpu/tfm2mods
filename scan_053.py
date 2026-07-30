# -*- coding: utf-8 -*-
# scan_053.py — 고유 바이트 지문으로 0.5.3 .text 전역 스캔 (마스크 없는 정확 시퀀스).
#   재컴파일로 함수 위치는 전부 바뀌었지만 "본문 특징 시퀀스"는 살아남는 경우가 많다.
#   .pdata 함수 경계와 교차해 "그 시퀀스를 품은 함수의 시작 RVA"를 돌려준다.
import struct, sys, io
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8")


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
    magic = struct.unpack_from("<H", d, opt)[0]
    ddir = opt + (112 if magic == 0x20b else 96)
    ex_rva, ex_sz = struct.unpack_from("<II", d, ddir + 3 * 8)
    return d, ib, secs, ex_rva, ex_sz


def sec(secs, nm):
    for n, va, vsz, rraw, rsz in secs:
        if n == nm:
            return va, vsz, rraw, rsz
    return None


def pdata(d, secs, ex_rva, ex_sz):
    for n, va, vsz, rraw, rsz in secs:
        if va <= ex_rva < va + max(vsz, rsz):
            po = rraw + (ex_rva - va)
            break
    out = [struct.unpack_from("<III", d, po + i * 12)[:2] for i in range(ex_sz // 12)]
    out.sort()
    return out


def owner(fns, rva):
    lo, hi = 0, len(fns) - 1
    while lo <= hi:
        m = (lo + hi) // 2
        if fns[m][0] <= rva < fns[m][1]:
            return fns[m]
        if rva < fns[m][0]:
            hi = m - 1
        else:
            lo = m + 1
    return None


def scan(d, secs, fns, sig, label, limit=25):
    va, vsz, rraw, rsz = sec(secs, ".text")
    blob = d[rraw: rraw + rsz]
    hits, i = [], 0
    while True:
        i = blob.find(sig, i)
        if i < 0:
            break
        rva = va + i
        o = owner(fns, rva)
        hits.append((rva, o))
        i += 1
    print(f"[{label}] sig={sig.hex(' ')}  → {len(hits)}건")
    for rva, o in hits[:limit]:
        if o:
            print(f"    @0x{rva:<9x} fn=0x{o[0]:x} size={o[1]-o[0]} (fn+{rva-o[0]:#x})")
        else:
            print(f"    @0x{rva:<9x} (함수경계 밖)")
    return hits


if __name__ == "__main__":
    tgt = sys.argv[1] if len(sys.argv) > 1 else "new"
    P = (r"C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.3\TeamfightManager2.exe" if tgt == "new"
         else r"C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.2\TeamfightManager2.exe")
    d, ib, secs, e, z = load(P)
    fns = pdata(d, secs, e, z)
    print(f"== {tgt} ({len(fns)} 함수) ==\n")
    # __rust_alloc: cmp rdx,0x11 / jae +0x11 / xor edx,edx / mov r8,rcx
    scan(d, secs, fns, bytes.fromhex("4883fa11731131d24989c8"), "ALLOC 본문")
    print()
    # __rust_dealloc: cmp r8,0x11 / jb +4 / mov rsi,[rsi-8]
    scan(d, secs, fns, bytes.fromhex("4983f81172044 88b76f8".replace(" ", "")), "DEALLOC 본문")
