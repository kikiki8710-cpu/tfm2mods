# -*- coding: utf-8 -*-
# serpen_053.py — tfm2_elemental_serpen 0.5.2→0.5.3 RVA 실측 검증 + 콜그래프 앵커 재핀 (capstone 직접)
#   Ghidra 없이도 ①프롤로그/명령경계 실측 ②컨테이너 콜사이트 대조로 런처류를 확정한다.
import sys, io, struct, pickle, collections
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8", errors="replace")

OLD = r"C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.2\TeamfightManager2.exe"
NEW = r"C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.3\TeamfightManager2.exe"
md = Cs(CS_ARCH_X86, CS_MODE_64); md.detail = False


class Img:
    def __init__(self, path, pkl):
        d = open(path, "rb").read()
        self.raw = d
        pe = struct.unpack_from("<I", d, 0x3c)[0]
        nsec = struct.unpack_from("<H", d, pe + 6)[0]
        opt = pe + 24
        sectab = opt + struct.unpack_from("<H", d, pe + 20)[0]
        self.secs = []
        for i in range(nsec):
            o = sectab + i * 40
            nm = d[o:o + 8].rstrip(b"\0").decode(errors="replace")
            vsz, va, rsz, rraw = struct.unpack_from("<IIII", d, o + 8)
            self.secs.append((nm, va, max(vsz, rsz), rraw, rsz))
        self.fn = {(int(k, 16) if isinstance(k, str) else k): v["size"]
                   for k, v in pickle.load(open(pkl, "rb"))["idx"].items()}
        self.starts = sorted(self.fn)

    def roff(self, rva):
        for nm, va, vsz, rraw, rsz in self.secs:
            if va <= rva < va + vsz:
                return rraw + (rva - va)
        return None

    def read(self, rva, n):
        o = self.roff(rva)
        return None if o is None else self.raw[o:o + n]

    def sec(self, rva):
        for nm, va, vsz, rraw, rsz in self.secs:
            if va <= rva < va + vsz:
                return nm
        return "?"

    def body(self, rva):
        n = self.fn.get(rva)
        return None if n is None else self.read(rva, n)

    def owner(self, rva):
        """rva 를 포함하는 함수 시작"""
        import bisect
        i = bisect.bisect_right(self.starts, rva) - 1
        if i < 0:
            return None
        s = self.starts[i]
        return s if rva < s + self.fn[s] else None

    def calls(self, fn_rva):
        """함수 본문의 직접 call(E8) → [(콜사이트rva, 타깃rva)]"""
        b = self.body(fn_rva)
        if b is None:
            return []
        out = []
        for i in md.disasm(b, fn_rva):
            if i.mnemonic == "call" and i.bytes[0] == 0xE8:
                rel = struct.unpack_from("<i", i.bytes, 1)[0]
                out.append((i.address, i.address + 5 + rel))
        return out


O = Img(OLD, r"C:\tfm2mods\_fnidx_052.pkl")
N = Img(NEW, r"C:\tfm2mods\_fnidx_053.pkl")


def insns(img, rva, n=40):
    b = img.read(rva, n)
    return list(md.disasm(b, rva)) if b else []


def boundary(img, rva, steal):
    """steal 바이트가 명령 경계에 정확히 맞는가 + 그 구간 rel/rip 유무"""
    acc, rel = 0, False
    for i in insns(img, rva, steal + 24):
        if acc >= steal:
            break
        m = i.mnemonic
        if m.startswith("j") or m in ("call", "loop") or "rip" in i.op_str:
            rel = True
        acc += i.size
    return acc == steal, acc, rel


def show(name, old_rva, new_rva, steal):
    if new_rva is None:
        print(f"[{name}] — 미해결")
        return
    ob = O.read(old_rva, 16); nb = N.read(new_rva, 16)
    if nb is None:
        print(f"[{name}] ✗ 섹션밖 new={new_rva:#x}")
        return
    ok, acc, rel = boundary(N, new_rva, steal)
    isfn = new_rva in N.fn
    oi = " | ".join(f"{i.mnemonic} {i.op_str}" for i in insns(O, old_rva, 20)[:6])
    ni = " | ".join(f"{i.mnemonic} {i.op_str}" for i in insns(N, new_rva, 20)[:6])
    st = "✓" if (ok and not rel and isfn) else "✗"
    print(f"[{name}] {st} old={old_rva:#x} new={new_rva:#x} steal={steal} "
          f"경계={ok}(acc={acc}) rel/rip={rel} 함수시작={isfn} "
          f"크기 {O.fn.get(old_rva)}→{N.fn.get(new_rva)}")
    print(f"    OLD16 {ob.hex(' ')}")
    print(f"    NEW16 {nb.hex(' ')}")
    if ob[:steal] != nb[:steal]:
        print(f"    OLDasm {oi}")
        print(f"    NEWasm {ni}")


# ── 1) 자동매칭이 낸 후보 실측 ────────────────────────────────
print("=" * 78)
print("1) 후보 프롤로그 실측 (steal = 소스가 트램폴린에 복사하는 바이트수)")
print("=" * 78)
CAND = [
    ("SERPEN_RVA",      0x21f8ca0, 0x1535810, 12),
    ("SPAWN_HOOKS[0]",  0x53aae0,  0xabdf60,  12),
    ("SPAWN_HOOKS[1]",  0x539f40,  0xabd340,  12),
    ("UILOADER_RVA",    0x5ac950,  0x91ab0,   12),
    ("UIPARSER_RVA",    0x24b5a00, 0x1a6530,  0),
    ("RENDER_STEP_RVA", 0x811500,  0x960df0,  12),
    ("DMGA_RVA",        0x22164a0, 0xfdbbb0,  12),
    ("DMGB_RVA",        0x22d2b20, 0x12c3bb0, 12),
    ("KEYRES_RVA",      0xc2f990,  0x1b0aba0, 12),
]
for c in CAND:
    show(*c); print()

# ── 2) 콜그래프 앵커: 컨테이너 콜사이트 대조로 런처/생성자 확정 ──
print("=" * 78)
print("2) 콜그래프 앵커 — 컨테이너 콜사이트 대조")
print("=" * 78)

# 씬빌더 컨테이너(확정 매칭): 0.5.2 0x74d510 → 0.5.3 0x997740
# 리플레이 핸들러(확정 매칭): 0.5.2 0x1554930 → 0.5.3 0x229a410
SCENE_O, SCENE_N = 0x74d510, 0x997740
REPL_O, REPL_N = 0x1554930, 0x229a410
LAUNCH_O = 0x1d96870

for nm, o, n in (("씬빌더", SCENE_O, SCENE_N), ("리플레이핸들러", REPL_O, REPL_N)):
    print(f"  {nm}: 0.5.2 {o:#x}(size {O.fn.get(o)}) → 0.5.3 {n:#x}(size {N.fn.get(n)})")

co = O.calls(SCENE_O); cn = N.calls(SCENE_N)
ro = O.calls(REPL_O);  rn = N.calls(REPL_N)
print(f"  콜 개수: 씬빌더 {len(co)}→{len(cn)} / 리플레이 {len(ro)}→{len(rn)}")

# 0.5.2 검증: 런처가 씬빌더에서 몇 번, 리플레이에서 몇 번 불리나
so = collections.Counter(t for _, t in co)
sro = collections.Counter(t for _, t in ro)
print(f"  [0.5.2 기준] 런처 {LAUNCH_O:#x}: 씬빌더 {so[LAUNCH_O]}회 / 리플레이 {sro[LAUNCH_O]}회")
print(f"    씬빌더 콜사이트 = {[hex(a) for a, t in co if t == LAUNCH_O]} (retaddr {[hex(a+5) for a,t in co if t==LAUNCH_O]})")
print(f"    리플레이 콜사이트 = {[hex(a) for a, t in ro if t == LAUNCH_O]} (retaddr {[hex(a+5) for a,t in ro if t==LAUNCH_O]})")

# 0.5.3 후보 = 씬빌더에서 N회 + 리플레이에서 M회 불리는 타깃 (0.5.2와 같은 다중도)
sn_ = collections.Counter(t for _, t in cn)
rn_ = collections.Counter(t for _, t in rn)
want_s, want_r = so[LAUNCH_O], sro[LAUNCH_O]
hits = [t for t in sn_ if sn_[t] == want_s and rn_.get(t, 0) == want_r]
print(f"\n  [0.5.3] 씬빌더 {want_s}회 ∧ 리플레이 {want_r}회 인 타깃 = {[hex(t) for t in sorted(hits)]}")
for t in sorted(hits):
    a = [hex(x) for x, y in cn if y == t]
    b = [hex(x) for x, y in rn if y == t]
    print(f"    {t:#x} size={N.fn.get(t)} 씬빌더콜사이트={a} 리플레이콜사이트={b} "
          f"NEW16={N.read(t,16).hex(' ') if N.read(t,16) else '?'}")

# 참고: 0.5.2 런처 전체 콜사이트(전 exe E8 스캔은 비싸니 컨테이너만)
print(f"\n  0.5.2 런처 진입 16B = {O.read(LAUNCH_O,16).hex(' ')} size={O.fn.get(LAUNCH_O)}")
