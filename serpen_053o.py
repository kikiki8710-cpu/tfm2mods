# -*- coding: utf-8 -*-
# serpen_053o.py — UILOADER 확증(문자열 xref) + UIPARSER 독립 투표 검증.
#   UILOADER 는 바이트동일 모노모픽 copy가 다수라 RVA 스왑만 하면 엉뚱한 copy를 훅해 조용히 미발화한다.
#   ⇒ "asset/base/ui/layout/ingame" LEA 직후 호출되는 게터가 정답(소스 주석의 확정 방법).
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
        nm = d[o:o + 8].rstrip(b"\x00").decode("utf-8", "replace")
        vsz, va, rsz, rraw = struct.unpack_from("<IIII", d, o + 8)
        secs.append((nm, va, max(vsz, rsz), rraw, rsz))
    P = pickle.load(open(pk, "rb"))["idx"]
    fn = {(int(k, 16) if isinstance(k, str) else k): v for k, v in P.items()}
    return d, secs, fn


def finder(d, secs):
    def rd(rva, n):
        for nm, va, vs, rr, rs in secs:
            if va <= rva < va + vs:
                return d[rr + (rva - va): rr + (rva - va) + n]
    def find_str(s):
        out = []
        pat = s.encode()
        for nm, va, vs, rr, rs in secs:
            if nm not in (".rdata", ".data", ".text"):
                continue
            i = rr
            end = rr + rs
            while True:
                i = d.find(pat, i, end)
                if i < 0:
                    break
                out.append((nm, va + (i - rr)))
                i += 1
        return out
    return rd, find_str


for tag, path, pk in (("0.5.2", r"C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.2\TeamfightManager2.exe",
                       r"C:\tfm2mods\_fnidx_052.pkl"),
                      ("0.5.3", r"C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.3\TeamfightManager2.exe",
                       r"C:\tfm2mods\_fnidx_053.pkl")):
    D, SECS, FN = load(path, pk)
    RD, FS = finder(D, SECS)
    va, vsz, rraw, rsz = [s[1:] for s in SECS if s[0] == ".text"][0]
    blob = D[rraw:rraw + rsz]
    starts = sorted(FN)
    print("=" * 84)
    print(f"[{tag}] 'asset/base/ui/layout/ingame' 문자열 → LEA xref → 직후 call 타깃")
    locs = [r for nm, r in FS("asset/base/ui/layout/ingame") if nm == ".rdata"]
    print(f"   문자열 위치 {[hex(x) for x in locs[:6]]} (총 {len(locs)})")
    tgt = collections.Counter()
    # rip-relative LEA (48 8d ??) 스캔
    i = 0
    while True:
        i = blob.find(b"\x48\x8d", i)
        if i < 0 or i + 7 > len(blob):
            break
        modrm = blob[i + 2]
        if (modrm & 0xC7) == 0x05:  # mod=00 rm=101 → rip-rel
            disp = struct.unpack_from("<i", blob, i + 3)[0]
            site = va + i
            trg = site + 7 + disp
            if trg in locs:
                j = bisect.bisect_right(starts, site) - 1
                f = starts[j] if j >= 0 else None
                # 직후 8명령 안의 첫 E8 call
                b2 = RD(site, 64)
                for ins in md.disasm(b2, site):
                    if ins.mnemonic == "call" and ins.bytes[0] == 0xE8:
                        t = ins.address + 5 + struct.unpack_from("<i", ins.bytes, 1)[0]
                        tgt[t] += 1
                        break
        i += 1
    print(f"   → 게터 후보: " + ", ".join(f"{t:#x}({c}회, size={FN.get(t,{}).get('size')})"
                                        for t, c in tgt.most_common(5)))
    # main 계열도
    locs2 = [r for nm, r in FS("asset/base/ui/layout/main") if nm == ".rdata"]
    tgt2 = collections.Counter()
    i = 0
    while True:
        i = blob.find(b"\x48\x8d", i)
        if i < 0 or i + 7 > len(blob):
            break
        if (blob[i + 2] & 0xC7) == 0x05:
            disp = struct.unpack_from("<i", blob, i + 3)[0]
            site = va + i
            if site + 7 + disp in locs2:
                for ins in md.disasm(RD(site, 64), site):
                    if ins.mnemonic == "call" and ins.bytes[0] == 0xE8:
                        tgt2[ins.address + 5 + struct.unpack_from("<i", ins.bytes, 1)[0]] += 1
                        break
        i += 1
    print(f"   → (main 계열) " + ", ".join(f"{t:#x}({c}회)" for t, c in tgt2.most_common(5)))
