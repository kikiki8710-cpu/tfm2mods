# -*- coding: utf-8 -*-
# vtslot2_053.py — vtslot_053.py 후속. 소스 테이블의 **베이스 버전 판정** + 슬롯 함수 분류.
#   발견: 52종 중 48종이 0.5.2 에서 함수 시작이 아니다 ⟹ 0.5.1 값 그대로일 가능성.
#   여기서: 0.5.1 / 0.5.2 / 0.5.3 세 exe 에서 각 RVA 의 함수시작 여부 + 프롤로그 디스어셈을 나란히 찍어
#           ①어느 버전 기준인지 확정 ②슬롯 함수를 (상수스텁 / 단순게터 / 복합) 으로 분류한다.
import struct, sys, io, re, collections
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8")
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
md = Cs(CS_ARCH_X86, CS_MODE_64)

SRC = r"C:\tfm2mods\tfm2_ai_adjust\src\disc19_repro.rs"
EXES = {
    "0.5.1": r"C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.1\TeamfightManager2.exe",
    "0.5.2": r"C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.2\TeamfightManager2.exe",
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
    magic = struct.unpack_from("<H", d, opt)[0]
    ddir = opt + (112 if magic == 0x20b else 96)
    ex, ez = struct.unpack_from("<II", d, ddir + 3 * 8)
    po = None
    for nm, va, vsz, rraw, rsz in secs:
        if va <= ex < va + max(vsz, rsz):
            po = rraw + (ex - va)
    fns = sorted({struct.unpack_from("<III", d, po + i * 12)[0] for i in range(ez // 12)})
    return dict(d=d, ib=ib, secs=secs, fns=set(fns))


def roff(secs, rva):
    for nm, va, vsz, rraw, rsz in secs:
        if va <= rva < va + max(vsz, rsz):
            o = rva - va
            return rraw + o if o < rsz else None
    return None


def dis(E, rva, maxn=6):
    o = roff(E["secs"], rva)
    if o is None:
        return "(범위밖)"
    b = E["d"][o:o + 32]
    out = []
    for i in md.disasm(b, rva):
        out.append(f"{i.mnemonic} {i.op_str}".strip())
        if i.mnemonic in ("ret", "jmp") or len(out) >= maxn:
            break
    return "; ".join(out)


def classify(txt):
    """프롤로그 문자열 → 슬롯 함수 유형"""
    t = txt.replace(" ", "")
    if re.match(r"^xoreax,eax;ret", t):
        return "CONST0"
    if re.match(r"^moval,1;ret", t) or re.match(r"^mov(eax|al),1;ret", t):
        return "CONST1"
    if re.match(r"^moveax,\d?x?[0-9a-f]*;ret", t):
        return "CONSTn"
    if re.match(r"^movrax,qwordptr\[rcx\+?[0-9a-fx]*\];ret", t):
        return "GETTER"
    if re.match(r"^mov(eax|rax),(d|q)wordptr\[rcx", t) and t.endswith("ret"):
        return "GETTER"
    if t.startswith("movqwordptr[rsp+0x10],rdx;push"):
        return "BIGFN"          # 표준 대형 프롤로그
    if t.startswith("push") or t.startswith("subrsp"):
        return "FN"
    return "?"


ES = {k: load(v) for k, v in EXES.items()}

txt = open(SRC, encoding="utf-8").read()
lines = txt.splitlines()
sites = collections.OrderedDict()
for ln, s in enumerate(lines, 1):
    for m in re.finditer(r"0x([0-9a-f]{6,7})\s*(=>|\|)", s):
        sites.setdefault(int(m.group(1), 16), []).append(ln)

print(f"소스 match 아암 고유 RVA = {len(sites)}종\n")
cnt = collections.Counter()
rows = []
for r in sorted(sites):
    st = {k: (r in ES[k]["fns"]) for k in ES}
    d51, d52, d53 = dis(ES["0.5.1"], r), dis(ES["0.5.2"], r), dis(ES["0.5.3"], r)
    c51 = classify(d51)
    rows.append((r, st, c51, d51, d52, d53))
    cnt[(st["0.5.1"], st["0.5.2"], st["0.5.3"])] += 1

print("함수시작 여부 조합 (0.5.1, 0.5.2, 0.5.3) → 개수")
for k, v in cnt.most_common():
    print(f"  {k} → {v}")

print("\n" + "=" * 140)
print("RVA별 3버전 비교 (fn= .pdata 함수시작)")
print("=" * 140)
for r, st, c51, d51, d52, d53 in rows:
    flag = "".join("Y" if st[k] else "·" for k in ("0.5.1", "0.5.2", "0.5.3"))
    print(f"\n0x{r:<9x} fn[051/052/053]={flag}  유형(0.5.1)={c51}   L{sites[r][0]}")
    print(f"   051 | {d51}")
    print(f"   052 | {d52}")
    print(f"   053 | {d53}")

print("\n" + "=" * 140)
print("0.5.1 기준 유형 분포")
print("=" * 140)
tc = collections.Counter(x[2] for x in rows)
for k, v in tc.most_common():
    print(f"  {k:8s} {v}")
