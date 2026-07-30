# -*- coding: utf-8 -*-
# serpen_053b.py — 미해결 4종(MOBATICK/RUNNER_CTOR/UIALLOC/ARG_STR)을 전역 콜그래프 앵커로 재핀.
#   방법: .text 전역 E8 스캔 → (콜사이트, 타깃) 인덱스. 타깃이 .pdata 함수시작인 것만 채택.
#   앵커 = 이미 확정된 매칭쌍(런처·씬빌더 등)을 기준으로 "그 함수가 부르는/그 함수를 부르는" 관계 대조.
import sys, io, struct, pickle, collections, bisect
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8", errors="replace")
md = Cs(CS_ARCH_X86, CS_MODE_64)

OLD = r"C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.2\TeamfightManager2.exe"
NEW = r"C:\Users\dev\Desktop\claude\tfm2\tfm2_0.5.3\TeamfightManager2.exe"


class Img:
    def __init__(self, path, pkl, tag):
        self.tag = tag
        d = open(path, "rb").read(); self.raw = d
        pe = struct.unpack_from("<I", d, 0x3c)[0]
        nsec = struct.unpack_from("<H", d, pe + 6)[0]; opt = pe + 24
        sectab = opt + struct.unpack_from("<H", d, pe + 20)[0]
        self.secs = []
        for i in range(nsec):
            o = sectab + i * 40
            nm = d[o:o + 8].rstrip(b"\0").decode(errors="replace")
            vsz, va, rsz, rraw = struct.unpack_from("<IIII", d, o + 8)
            self.secs.append((nm, va, max(vsz, rsz), rraw, rsz))
        P = pickle.load(open(pkl, "rb"))["idx"]
        self.fn = {(int(k, 16) if isinstance(k, str) else k): v for k, v in P.items()}
        self.starts = sorted(self.fn)

    def roff(self, rva):
        for nm, va, vsz, rraw, rsz in self.secs:
            if va <= rva < va + vsz:
                return rraw + (rva - va)

    def read(self, rva, n):
        o = self.roff(rva)
        return None if o is None else self.raw[o:o + n]

    def text(self):
        for nm, va, vsz, rraw, rsz in self.secs:
            if nm == ".text":
                return va, vsz, rraw, rsz

    def owner(self, rva):
        i = bisect.bisect_right(self.starts, rva) - 1
        if i < 0:
            return None
        s = self.starts[i]
        return s if rva < s + self.fn[s]["size"] else None

    def build_calls(self):
        """전역 E8 스캔 → callers[target] = [사이트...], callees[fn] = Counter(target)"""
        va, vsz, rraw, rsz = self.text()
        blob = self.raw[rraw:rraw + rsz]
        fnset = self.fn
        callers = collections.defaultdict(list)
        n = len(blob)
        i = 0
        while True:
            i = blob.find(b"\xe8", i)
            if i < 0 or i + 5 > n:
                break
            rel = struct.unpack_from("<i", blob, i + 1)[0]
            site = va + i
            tgt = site + 5 + rel
            if tgt in fnset:
                callers[tgt].append(site)
            i += 1
        self.callers = callers
        print(f"  [{self.tag}] E8 스캔 완료: 타깃 {len(callers)}개")

    def callees(self, fn_rva):
        b = self.read(fn_rva, self.fn[fn_rva]["size"])
        c = collections.Counter()
        sites = collections.defaultdict(list)
        for i in md.disasm(b, fn_rva):
            if i.mnemonic == "call" and i.bytes[0] == 0xE8:
                rel = struct.unpack_from("<i", i.bytes, 1)[0]
                t = i.address + 5 + rel
                c[t] += 1
                sites[t].append(i.address)
        return c, sites


O = Img(OLD, r"C:\tfm2mods\_fnidx_052.pkl", "0.5.2")
N = Img(NEW, r"C:\tfm2mods\_fnidx_053.pkl", "0.5.3")
O.build_calls(); N.build_calls()

# 확정 앵커쌍 (serpen_053.py 1·2단계에서 실측 확정)
ANCHOR = {
    0x21f8ca0: 0x1535810,   # SERPEN
    0x53aae0:  0xabdf60,    # SPAWN0
    0x539f40:  0xabd340,    # SPAWN1
    0x5ac950:  0x91ab0,     # UILOADER
    0x24b5a00: 0x1a6530,    # UIPARSER
    0x811500:  0x960df0,    # RENDER_STEP
    0x22164a0: 0xfdbbb0,    # DMGA
    0x22d2b20: 0x12c3bb0,   # DMGB
    0xc2f990:  0x1b0aba0,   # KEYRES
    0x1d96870: 0xeb8810,    # LAUNCHER (2단계 확정)
    0x74d510:  0x997740,    # 씬빌더
    0x1554930: 0x229a410,   # 리플레이 핸들러
    0x25c4dd0: 0x28e3b10,   # realloc (item_tactics 확정)
}


def mnem_cos(a, b):
    A, B = O.fn[a]["mnem"], N.fn[b]["mnem"]
    ks = set(A) | set(B)
    import math
    da = math.sqrt(sum(A.get(k, 0) ** 2 for k in ks))
    db = math.sqrt(sum(B.get(k, 0) ** 2 for k in ks))
    if not da or not db:
        return 0.0
    return sum(A.get(k, 0) * B.get(k, 0) for k in ks) / (da * db)


def anchor_repin(name, old_rva, note=""):
    """old_rva 의 caller/callee 중 앵커에 잡힌 것을 이용해 0.5.3 후보를 좁힌다."""
    print("=" * 78)
    print(f"[{name}] 0.5.2 = {old_rva:#x} size={O.fn.get(old_rva,{}).get('size')} {note}")
    ocallers = O.callers.get(old_rva, [])
    ofn = collections.Counter()
    for s in ocallers:
        w = O.owner(s)
        if w is not None:
            ofn[w] += 1
    print(f"  0.5.2 caller 함수 {len(ofn)}개 (콜사이트 {len(ocallers)})")
    # 앵커에 잡힌 caller 로 후보 뽑기
    cand = None
    for cf, cnt in ofn.most_common():
        if cf in ANCHOR:
            nf = ANCHOR[cf]
            cc, _ = N.callees(nf)
            hits = [t for t, k in cc.items() if k == cnt]
            print(f"  앵커 caller {cf:#x}→{nf:#x} (0.5.2 {cnt}회): 0.5.3 동일다중도 타깃 {len(hits)}개")
            s = set(hits)
            cand = s if cand is None else (cand & s)
    if cand:
        scored = sorted(((mnem_cos(old_rva, t), t) for t in cand if t in N.fn), reverse=True)
        print(f"  ▶ 교집합 후보 {len(cand)}개:")
        for c, t in scored[:8]:
            print(f"     {t:#x} cos={c:.4f} size={O.fn[old_rva]['size']}→{N.fn[t]['size']} "
                  f"16B={N.read(t,16).hex(' ')}")
    else:
        print("  ▶ 앵커 caller 없음 — callee 방향 시도")
    return cand


print()
# ── RUNNER_CTOR: 화면 경기 sim Game 생성자 (런처 근처)
anchor_repin("RUNNER_CTOR", 0x1d981e0, "(0.5.2 런처 +0x1970)")
print()
# ── MOBATICK: MobaMode::tick
anchor_repin("MOBATICK", 0x230c290, "(rcx=World, 매 틱)")
print()
# ── ARG_STR: i18n arg(key,&String)
anchor_repin("ARG_STR", 0xfef190, "(i18n 치환 빌더)")
print()
# ── UIALLOC: 게임 힙 alloc — realloc(0x25c4dd0→0x28e3b10) 클러스터 이웃
print("=" * 78)
print("[UIALLOC] 0.5.2 = 0x25c4d30 (alloc). 이웃: dealloc 0x25c4d90 / realloc 0x25c4dd0→0x28e3b10 확정")
o_alloc = 0x25c4d30
print(f"  0.5.2 alloc 16B = {O.read(o_alloc,16).hex(' ')} size={O.fn[o_alloc]['size']} mnem={O.fn[o_alloc]['mnem']}")
print(f"  0.5.2 realloc  16B = {O.read(0x25c4dd0,16).hex(' ')} size={O.fn[0x25c4dd0]['size']}")
print(f"  0.5.3 realloc  16B = {N.read(0x28e3b10,16).hex(' ')} size={N.fn[0x28e3b10]['size']}")
print("  0.5.3 realloc 주변 함수(±0x400):")
lo, hi = 0x28e3b10 - 0x400, 0x28e3b10 + 0x400
for s in N.starts:
    if lo <= s <= hi:
        print(f"    {s:#x} size={N.fn[s]['size']:5d} cos(vs 0.5.2 alloc)={mnem_cos(o_alloc,s):.4f} "
              f"16B={N.read(s,16).hex(' ')}")
