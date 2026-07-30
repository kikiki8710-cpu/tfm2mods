# -*- coding: utf-8 -*-
# verify_053.py — 0.5.3 후보 RVA를 0.5.2 원본과 바이트/디스어셈 대조로 실측 검증.
#   _MIGRATE_053.md 의 "유력/확정" 등급은 통계적 추정이므로, 훅 설치 전 실측이 필수(지시서 §2).
#   검사: ① 프롤로그 N바이트 명령열 동형 ② orig_len 명령경계 정확 일치 ③ rip-rel/상대분기 유무
import sys, io, pefile
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8")

OLD = r"C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.2\TeamfightManager2.exe"
NEW = r"C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.3\TeamfightManager2.exe"
md = Cs(CS_ARCH_X86, CS_MODE_64); md.detail = True


class Img:
    def __init__(self, path):
        pe = pefile.PE(path, fast_load=True)
        self.base = pe.OPTIONAL_HEADER.ImageBase
        self.secs = []
        with open(path, "rb") as f:
            self.raw = f.read()
        for s in pe.sections:
            nm = s.Name.rstrip(b"\x00").decode(errors="replace")
            self.secs.append((nm, s.VirtualAddress, max(s.Misc_VirtualSize, s.SizeOfRawData),
                              s.PointerToRawData, s.SizeOfRawData))
        pe.close()

    def sec_of(self, rva):
        for nm, va, vsz, ptr, rsz in self.secs:
            if va <= rva < va + vsz:
                return nm, va, vsz, ptr, rsz
        return None

    def read(self, rva, n):
        s = self.sec_of(rva)
        if not s:
            return None
        nm, va, vsz, ptr, rsz = s
        off = rva - va
        if off >= rsz:
            return b"\x00" * n
        return self.raw[ptr + off: ptr + off + n]

    def text_range(self):
        for nm, va, vsz, ptr, rsz in self.secs:
            if nm == ".text":
                return va, va + vsz
        return None


def dis(img, rva, nbytes):
    b = img.read(rva, nbytes + 32)
    if b is None:
        return None
    return list(md.disasm(b, img.base + rva))


def skel(insns, upto):
    """명령 스켈레톤: (mnemonic, op_str에서 즉시값/변위 제거) — upto 바이트까지"""
    out, acc = [], 0
    for i in insns:
        if acc >= upto:
            break
        out.append((i.mnemonic, i.op_str, i.size))
        acc += i.size
    return out


def boundary_ok(insns, orig_len):
    acc = 0
    for i in insns:
        acc += i.size
        if acc == orig_len:
            return True, acc
        if acc > orig_len:
            return False, acc
    return False, acc


def has_riprel(insns, upto):
    acc = 0
    for i in insns:
        if acc >= upto:
            break
        if "rip" in i.op_str:
            return True
        # 상대분기(rel8/rel32)도 트램폴린 복사 시 깨짐
        if i.group(1) or i.group(7):   # JUMP=1, BRANCH_RELATIVE=7 (capstone x86 group ids)
            return True
        acc += i.size
    return False


def relbranch(insns, upto):
    acc = 0
    for i in insns:
        if acc >= upto:
            break
        m = i.mnemonic
        if m.startswith("j") or m == "call" or m == "loop":
            return True
        acc += i.size
    return False


def check(name, old_rva, new_rva, orig_len, cmp_bytes=32):
    o, n = IMG_O, IMG_N
    io_ = dis(o, old_rva, cmp_bytes)
    inn = dis(n, new_rva, cmp_bytes)
    if io_ is None or inn is None:
        print(f"[{name}] ✗ 섹션밖 (old={o.sec_of(old_rva)} new={n.sec_of(new_rva)})")
        return False
    so, sn = skel(io_, cmp_bytes), skel(inn, cmp_bytes)
    ob, nb = o.read(old_rva, cmp_bytes), n.read(new_rva, cmp_bytes)
    bok, bacc = boundary_ok(inn, orig_len)
    rr = relbranch(inn, orig_len) or has_riprel(inn, orig_len)
    same_pro = [(m, s) for m, s, _ in so[:6]] == [(m, s) for m, s, _ in sn[:6]]
    exact = ob[:orig_len] == nb[:orig_len]
    st = "✓" if (bok and not rr) else "✗"
    print(f"[{name}] {st} old=0x{old_rva:x} new=0x{new_rva:x} orig_len={orig_len}")
    print(f"    경계일치={bok}(acc={bacc})  rel/rip={rr}  프롤로그6동형={same_pro}  선두{orig_len}B바이트동일={exact}")
    print(f"    OLD: {ob[:orig_len].hex(' ')}")
    print(f"    NEW: {nb[:orig_len].hex(' ')}")
    if not same_pro:
        print(f"    OLD asm: {' | '.join(f'{m} {s}' for m,s,_ in so[:6])}")
        print(f"    NEW asm: {' | '.join(f'{m} {s}' for m,s,_ in sn[:6])}")
    return bok and not rr


IMG_O = Img(OLD)
IMG_N = Img(NEW)
print("== .text 범위 ==")
print(f"  0.5.2 .text = {IMG_O.text_range()[0]:#x} .. {IMG_O.text_range()[1]:#x}")
print(f"  0.5.3 .text = {IMG_N.text_range()[0]:#x} .. {IMG_N.text_range()[1]:#x}")
print("== 섹션 ==")
for nm, va, vsz, ptr, rsz in IMG_N.secs:
    print(f"  {nm:10s} rva={va:#x} vsz={vsz:#x} end={va+vsz:#x}")
print()

TARGETS = [
    # (이름, 0.5.2 RVA, 0.5.3 후보, orig_len)
    ("RVA_RETREAT",        0x1b94670, 0xe00350,  12),
    ("RVA_FC59A0",         0x1bdb3e0, 0xe168d0,  12),
    ("RVA_CONDGATE",       0x21338d0, 0xc550b0,  15),
    ("RVA_MOVEPRI",        0x2134240, 0xc559e0,  13),
    ("RVA_DISC18_HANDLER", 0x2376320, 0xd94d00,  12),
    ("RVA_DISC19_HANDLER", 0x2380820, 0xdece30,  12),
    ("RVA_GENERIC_BUILD",  0x22b2280, 0xe06c10,  12),
    ("RVA_ITEMNET_SCORER", 0x1b9cce0, 0x10587e0, 12),
    ("LOADER_RVA",         0x5ac950,  0x91ab0,   24),
    ("PARSER_RVA",         0x24b5a00, 0x1a6530,  20),
]
print("== 훅 대상 프롤로그 실측 ==")
for t in TARGETS:
    check(*t)
    print()
