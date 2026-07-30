# -*- coding: utf-8 -*-
# vtslot3_053.py — 소스 vtable 슬롯 테이블의 **베이스 버전 특정**.
#   0.5.1/0.5.2 어디서도 함수시작이 아닌 RVA 가 다수 ⟹ 더 옛 버전(0.5.0 계열) 기준 의심.
#   보유 exe 백업 전부에 대해 "각 RVA 가 함수 시작처럼 보이는가"를 점수화해 베이스를 찾는다.
#   판정 = ① .pdata 함수시작 ② 표준 프롤로그/짧은 스텁 패턴 (leaf 는 .pdata 에 없을 수 있으므로 둘 다 본다)
import struct, sys, io, re, glob, os, collections
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8")
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
md = Cs(CS_ARCH_X86, CS_MODE_64)

SRC = r"C:\tfm2mods\tfm2_ai_adjust\src\disc19_repro.rs"
ROOT = r"C:\Users\dev\Desktop\claude\tfm2"
GAME = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe"


def load(p):
    d = open(p, "rb").read()
    pe = struct.unpack_from("<I", d, 0x3c)[0]
    nsec = struct.unpack_from("<H", d, pe + 6)[0]
    opt = pe + 24
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
    fns = {struct.unpack_from("<III", d, po + i * 12)[0] for i in range(ez // 12)}
    return dict(d=d, secs=secs, fns=fns, size=len(d))


def roff(secs, rva):
    for nm, va, vsz, rraw, rsz in secs:
        if va <= rva < va + max(vsz, rsz):
            o = rva - va
            return rraw + o if o < rsz else None
    return None


PROLOG = re.compile(
    r"^(mov qword ptr \[rsp \+ 0x\w+\], (rdx|rcx|r8|r9)"     # 표준 인자 스필
    r"|push (rbp|rbx|rsi|rdi|r1[2-5])"
    r"|sub rsp, 0x\w+"
    r"|xor eax, eax"
    r"|mov (al|eax), 0x?\w*"
    r"|mov rax, qword ptr \[rcx"
    r"|mov eax, dword ptr \[rcx"
    r"|movzx eax, "
    r"|movss xmm0, "
    r"|ret)")


def looks_like_fnstart(E, rva):
    o = roff(E["secs"], rva)
    if o is None:
        return False, ""
    b = E["d"][o:o + 24]
    ins = list(md.disasm(b, rva))
    if not ins:
        return False, ""
    head = f"{ins[0].mnemonic} {ins[0].op_str}".strip()
    return bool(PROLOG.match(head)), "; ".join(f"{i.mnemonic} {i.op_str}".strip() for i in ins[:3])


txt = open(SRC, encoding="utf-8").read()
sites = []
for ln, s in enumerate(txt.splitlines(), 1):
    for m in re.finditer(r"0x([0-9a-f]{6,7})\s*(=>|\|)", s):
        r = int(m.group(1), 16)
        if r not in [x[0] for x in sites]:
            sites.append((r, ln))
RVAS = [r for r, _ in sites]

paths = []
for d in sorted(glob.glob(os.path.join(ROOT, "tfm2_0.5.*"))):
    p = os.path.join(d, "TeamfightManager2.exe")
    if os.path.exists(p):
        paths.append((os.path.basename(d), p))
if os.path.exists(GAME):
    paths.append(("설치본(현행)", GAME))

print(f"슬롯 RVA {len(RVAS)}종 · exe 후보 {len(paths)}개\n")
print(f"{'exe':<18}{'크기':>12}  {'.pdata시작':>10} {'프롤로그형':>10} {'합격(둘중1)':>12}")
print("-" * 70)
best = []
for nm, p in paths:
    E = load(p)
    a = sum(1 for r in RVAS if r in E["fns"])
    b = sum(1 for r in RVAS if looks_like_fnstart(E, r)[0])
    c = sum(1 for r in RVAS if (r in E["fns"]) or looks_like_fnstart(E, r)[0])
    print(f"{nm:<18}{E['size']:>12,}  {a:>10} {b:>10} {c:>12}")
    best.append((c, nm, p, E))

best.sort(reverse=True, key=lambda x: x[0])
c, nm, p, E = best[0]
print(f"\n★베이스 추정 = {nm} ({c}/{len(RVAS)} 합격)\n")
print("=" * 120)
print(f"{nm} 기준 각 RVA 프롤로그")
print("=" * 120)
for r, ln in sites:
    ok, head = looks_like_fnstart(E, r)
    print(f"0x{r:<10x} {'fn' if r in E['fns'] else '  '} {'OK' if ok else '  '}  L{ln:<6} {head}")
