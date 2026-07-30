# -*- coding: utf-8 -*-
# rvascan_053.py — ★모드 소스 전체에서 **미재핀 하드코딩 RVA**를 찾아내는 감사기.
#
# 왜 필요한가(2026-07-30 실측): `disc19_repro.rs` 의 vtable 슬롯표 52종과 `serpen.rs` 의 `c8c_cast_get` 9종이
#   **0.5.1·0.5.2 마이그에서 통째로 누락**돼 0.5.0_3 세대 값이 방치돼 있었다. 크래시가 없어서(=조용히 fallback)
#   두 번의 버전업 동안 아무도 눈치채지 못했다. ⟹ 사람 눈이 아니라 도구로 훑어야 한다.
#
# 판정법: 소스의 6~7자리 hex 상수 중 .text 범위인 것을 뽑아
#   ① 0.5.3 에서 "함수 시작처럼 보이는가"(.pdata 시작 or 함수포인터 풀 등장 or 유효 프롤로그)
#   ② 구버전(0.5.1/0.5.0_3)에서는 그런가
#   → **구버전에서만 유효 = 미재핀 잔재**로 신고한다.
# ⚠오탐 주의: 순수 상수(오프셋·마스크)도 6자리일 수 있다 ⟹ 결과는 "후보"이고 사람이 문맥으로 확인해야 한다.
import struct, sys, io, re, glob, os, collections
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8")
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
md = Cs(CS_ARCH_X86, CS_MODE_64)

ROOT = r"C:\Users\dev\Desktop\claude\tfm2"
SRCDIR = sys.argv[1] if len(sys.argv) > 1 else r"C:\tfm2mods\tfm2_ai_adjust\src"
VERS = ["tfm2_0.5.3", "tfm2_0.5.1", "tfm2_0.5.0_3"]
SKIP_FILES = ("rva_051.rs", "rva_052.rs", ".preoifix", ".predescvt")


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
    fns = {struct.unpack_from("<III", d, po + i * 12)[0] for i in range(ez // 12)}
    txt = [s for s in secs if s[0] == ".text"][0]
    # 데이터 섹션 함수포인터 풀
    pool = collections.Counter()
    for s, va, vsz, rr, rsz in secs:
        if s not in (".rdata", ".data"):
            continue
        p = rr + ((-va) % 8)
        while p + 8 <= rr + rsz:
            v = struct.unpack_from("<Q", d, p)[0]
            if v > ib and txt[1] <= (v - ib) < txt[1] + max(txt[2], txt[4]):
                pool[v - ib] += 1
            p += 8
    return dict(d=d, ib=ib, secs=secs, fns=fns, pool=pool,
                tva=txt[1], tsz=max(txt[2], txt[4]))


def ro(E, r):
    for nm, va, vsz, rr, rsz in E["secs"]:
        if va <= r < va + max(vsz, rsz):
            o = r - va
            return rr + o if o < rsz else None
    return None


PRO = re.compile(r"^(push |sub rsp|mov qword ptr \[rsp|xor eax, eax|mov (al|eax), |mov rax, qword ptr \[rcx|"
                 r"mov eax, dword ptr \[rcx|movzx eax, |lea rax, \[rcx|cmp qword ptr \[rcx|mov rax, rcx|ret)")


def score(E, r):
    """0.5.x 에서 이 RVA 가 '함수 시작'일 가능성 점수."""
    if not (E["tva"] <= r < E["tva"] + E["tsz"]):
        return 0
    s = 0
    if r in E["fns"]:
        s += 2
    if E["pool"].get(r):
        s += 3                      # 함수포인터로 실제 참조됨 = 강한 근거
    o = ro(E, r)
    if o is not None:
        ins = list(md.disasm(E["d"][o:o + 24], r))
        if ins and PRO.match(f"{ins[0].mnemonic} {ins[0].op_str}".strip()):
            s += 1
        if o >= 1 and E["d"][o - 1] in (0xcc, 0x90):   # 앞이 정렬 패딩
            s += 1
    return s


E = {v: load(v) for v in VERS}
print(f"소스 = {SRCDIR}\n")

rows = []
for p in sorted(glob.glob(os.path.join(SRCDIR, "*.rs"))):
    if any(k in os.path.basename(p) for k in SKIP_FILES):
        continue
    for ln, line in enumerate(open(p, encoding="utf-8").read().splitlines(), 1):
        code = line.split("//")[0]
        for m in re.finditer(r"0x([0-9a-f]{6,7})\b", code):
            r = int(m.group(1), 16)
            s3 = score(E["tfm2_0.5.3"], r)
            s1 = score(E["tfm2_0.5.1"], r)
            s0 = score(E["tfm2_0.5.0_3"], r)
            if max(s1, s0) >= 4 and s3 <= 2:      # 구버전에서만 강하게 함수시작
                rows.append((os.path.basename(p), ln, r, s3, s1, s0, line.strip()[:88]))

print(f"■ 미재핀 후보 {len(rows)}건 (구버전 점수≥4 & 0.5.3 점수≤2)")
print(f"{'파일':<22}{'줄':<7}{'RVA':<12}{'053':<5}{'051':<5}{'0503':<6} 소스")
print("=" * 132)
byf = collections.Counter()
for f, ln, r, s3, s1, s0, src in rows:
    byf[f] += 1
    print(f"{f:<22}L{ln:<6}0x{r:<10x}{s3:<5}{s1:<5}{s0:<6} {src}")
print("\n파일별 집계:", dict(byf))
