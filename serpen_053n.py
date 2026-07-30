# -*- coding: utf-8 -*-
# serpen_053n.py — 0.5.3 실할당자(0x28f7df0, = 0.5.2 0x25d9640 대응)를 부르는 함수 중
#   alloc shim(작고 E8+E9 둘 다로 실할당자에 가는 함수)을 찾는다.
import sys, io, struct, pickle, collections, bisect
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8", errors="replace")
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
md = Cs(CS_ARCH_X86, CS_MODE_64); md.detail = True


def load(p, pk):
    d = open(p, "rb").read()
    pe = struct.unpack_from("<I", d, 0x3c)[0]
    ns = struct.unpack_from("<H", d, pe + 6)[0]
    opt = pe + 24
    st = opt + struct.unpack_from("<H", d, pe + 20)[0]
    secs = []
    for i in range(ns):
        o = st + i * 40
        nm = d[o:o + 8].rstrip(b"\0").decode(errors="replace")
        vsz, va, rsz, rraw = struct.unpack_from("<IIII", d, o + 8)
        secs.append((nm, va, max(vsz, rsz), rraw, rsz))
    P = pickle.load(open(pk, "rb"))["idx"]
    fn = {(int(k, 16) if isinstance(k, str) else k): v for k, v in P.items()}
    return d, secs, fn


D, SECS, FN = load(r"C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.3\TeamfightManager2.exe",
                   r"C:\tfm2mods\_fnidx_053.pkl")
DO, SECO, FNO = load(r"C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.2\TeamfightManager2.exe",
                     r"C:\tfm2mods\_fnidx_052.pkl")


def mk(d, secs):
    def rd(rva, n):
        for nm, va, vsz, rraw, rsz in secs:
            if va <= rva < va + vsz:
                return d[rraw + (rva - va): rraw + (rva - va) + n]
    return rd


RD, RDO = mk(D, SECS), mk(DO, SECO)
va, vsz, rraw, rsz = [s[1:] for s in SECS if s[0] == ".text"][0]
blob = D[rraw:rraw + rsz]
starts = sorted(FN)
TGT = 0x28f7df0
res = collections.defaultdict(lambda: [0, 0])
for opc, idx in ((0xE8, 0), (0xE9, 1)):
    i = 0
    while True:
        i = blob.find(bytes([opc]), i)
        if i < 0 or i + 5 > len(blob):
            break
        rel = struct.unpack_from("<i", blob, i + 1)[0]
        site = va + i
        if site + 5 + rel == TGT:
            j = bisect.bisect_right(starts, site) - 1
            f = starts[j] if j >= 0 else None
            if f is not None and site < f + FN[f]["size"]:
                res[f][idx] += 1
        i += 1

print(f"실할당자 {TGT:#x} 를 참조하는 함수 = {len(res)}개")
both = sorted([(FN[f]['size'], f, v) for f, v in res.items() if v[0] and v[1]])
print(f"  ▶ E8+E9 둘 다 = {len(both)}개")
for s, f, v in both[:10]:
    print(f"     {f:#x} size={s} E8={v[0]} E9={v[1]} 16B={RD(f,16).hex(' ')}")
print("  소형 순 (상위 12):")
for s, f, v in sorted([(FN[f]['size'], f, v) for f, v in res.items()])[:12]:
    print(f"     {f:#x} size={s} E8={v[0]} E9={v[1]} 16B={RD(f,16).hex(' ')}")

print("\n  [참고] 0.5.2 쪽 동일 통계 (실할당자 0x25d9640)")
vao, vso, rro, rso = [s[1:] for s in SECO if s[0] == ".text"][0]
bo = DO[rro:rro + rso]
so_ = sorted(FNO)
r2 = collections.defaultdict(lambda: [0, 0])
for opc, idx in ((0xE8, 0), (0xE9, 1)):
    i = 0
    while True:
        i = bo.find(bytes([opc]), i)
        if i < 0 or i + 5 > len(bo):
            break
        rel = struct.unpack_from("<i", bo, i + 1)[0]
        site = vao + i
        if site + 5 + rel == 0x25d9640:
            j = bisect.bisect_right(so_, site) - 1
            f = so_[j] if j >= 0 else None
            if f is not None and site < f + FNO[f]["size"]:
                r2[f][idx] += 1
        i += 1
print(f"    참조 함수 {len(r2)}개 / E8+E9 둘 다 = "
      f"{[(hex(f), FNO[f]['size']) for f, v in r2.items() if v[0] and v[1]]}")

# 후보 상세 디스어셈
print("\n  후보 디스어셈 (E8+E9 소형 1순위):")
for s, f, v in both[:2]:
    print(f"  --- {f:#x} size={s}")
    for i in md.disasm(RD(f, s), f):
        print(f"     {i.address:#x}  {i.bytes.hex(' '):<22} {i.mnemonic} {i.op_str}")
