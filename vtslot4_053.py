# -*- coding: utf-8 -*-
# vtslot4_053.py — 슬롯 함수 매핑 1단계: **vtable 참조 지문 수집**.
#   베이스 = 0.5.1(vtslot3 판정). 각 슬롯 RVA 에 대해
#     ① .rdata/.data 에서 절대주소(IB+rva) 8B 리틀엔디언 등장 위치 전수
#     ② 각 등장에 대해 vtable 시작 추정 → 슬롯 오프셋 산출
#     ③ (슬롯오프셋 → 등장수) 분포
#   ★검증: 소스 주석의 "98 vtable / 15 vtable / 16 vtable / 2 vtable ..." 카운트와 대조.
#   vtable 시작 추정: Rust vtable = [drop_in_place(코드ptr), size(정수), align(정수), method...]
#     ⟹ 뒤로 스캔하며 "코드포인터 다음에 작은 정수 2개" 패턴을 찾는다.
import struct, sys, io, re, collections
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8")

SRC = r"C:\tfm2mods\tfm2_ai_adjust\src\disc19_repro.rs"
EXE = r"C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.1\TeamfightManager2.exe"


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
    return d, ib, secs


def sec_of(secs, rva):
    for nm, va, vsz, rraw, rsz in secs:
        if va <= rva < va + max(vsz, rsz):
            return nm
    return None


def roff(secs, rva):
    for nm, va, vsz, rraw, rsz in secs:
        if va <= rva < va + max(vsz, rsz):
            o = rva - va
            return rraw + o if o < rsz else None
    return None


D, IB, SECS = load(EXE)
TEXT = [s for s in SECS if s[0] == ".text"][0]
TVA, TSZ = TEXT[1], max(TEXT[2], TEXT[4])


def is_code(rva):
    return TVA <= rva < TVA + TSZ


def qw(off):
    return struct.unpack_from("<Q", D, off)[0]


# ── 슬롯 RVA 추출 ────────────────────────────────────────────────
txt = open(SRC, encoding="utf-8").read()
sites = collections.OrderedDict()
for ln, s in enumerate(txt.splitlines(), 1):
    for m in re.finditer(r"0x([0-9a-f]{6,7})\s*(=>|\|)", s):
        sites.setdefault(int(m.group(1), 16), []).append((ln, s.strip()[:96]))

# ── 데이터 섹션에서 절대주소 등장 위치 색인 ───────────────────────
DATA_SECS = [s for s in SECS if s[0] in (".rdata", ".data")]
print(f"데이터 섹션: {[s[0] for s in DATA_SECS]}")

occ = collections.defaultdict(list)     # rva -> [등장 rva]
for r in sites:
    pat = struct.pack("<Q", IB + r)
    for nm, va, vsz, rraw, rsz in DATA_SECS:
        blob = D[rraw:rraw + rsz]
        i = 0
        while True:
            i = blob.find(pat, i)
            if i < 0:
                break
            if i % 8 == (va % 8):        # 8바이트 정렬 엔트리만
                occ[r].append(va + i)
            i += 1


def vtable_start(at):
    """등장 위치 at 에서 뒤로 스캔해 vtable 시작(drop,size,align) 추정."""
    o = roff(SECS, at)
    if o is None:
        return None
    for back in range(0, 0x400, 8):
        p = o - back
        if p < 0:
            break
        a, b, c = qw(p), qw(p + 8), qw(p + 16)
        # drop_in_place = 코드포인터, size/align = 작은 정수(align 은 2의 거듭제곱)
        if (a > IB and is_code(a - IB) and b < 0x10000 and 0 < c <= 64 and (c & (c - 1)) == 0):
            return at - back
    return None


print(f"\n{'RVA':<12}{'총등장':<8}{'슬롯오프셋 분포(오프셋×개수)':<60} 소스줄")
print("=" * 130)
res = {}
for r in sorted(sites):
    ats = occ.get(r, [])
    offs = collections.Counter()
    for at in ats:
        vs = vtable_start(at)
        offs[(at - vs) if vs is not None else -1] += 1
    res[r] = (len(ats), offs)
    dist = " ".join(f"{('?' if k < 0 else hex(k))}×{v}" for k, v in offs.most_common(6))
    print(f"0x{r:<10x}{len(ats):<8}{dist:<60} L{sites[r][0][0]}")

print("\n" + "=" * 130)
print("★소스 주석의 vtable 카운트와 대조할 것 (예: 0x50fc80 '98 vtable', 0x5418a0 '15 vtable')")
