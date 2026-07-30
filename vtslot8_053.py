# -*- coding: utf-8 -*-
# vtslot8_053.py — vtslot7 의 잔여(후보0 10종 · 베이스미상 3종) 전역 해소.
#   vtslot7 은 "같은 vtable 슬롯 오프셋"을 제약으로 걸었다. 잔여는 그 제약 때문에 못 찾은 것들이므로
#   여기서는 **데이터 섹션에서 함수포인터로 참조되는 코드주소 전부**를 후보 풀로 삼아 지문 검색한다.
#     - vtable 헤더(drop/size/align) 판정이 실패하는 배열(함수포인터 필드·디스패치 테이블)까지 커버.
#     - 후보의 "참조 위치 오프셋 분포"를 함께 찍어 사람이 슬롯 정합성을 확인할 수 있게 한다.
import struct, sys, io, re, json, collections
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8")
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
md = Cs(CS_ARCH_X86, CS_MODE_64)

ROOT = r"C:\Users\dev\Desktop\claude\tfm2"
MAP = r"C:\tfm2mods\_vtslot_053_map.json"
BASES = ["tfm2_0.5.1", "tfm2_0.5.0_3", "tfm2_0.5.0_2", "tfm2_0.5.0"]
TARGET = "tfm2_0.5.3"

BR = re.compile(r"^(j\w+|call|loop\w*)$")
RIP = re.compile(r"\[rip [+\-] 0x[0-9a-f]+\]")
HEX = re.compile(r"0x[0-9a-f]+")


def load(name):
    d = open(rf"{ROOT}\{name}\TeamfightManager2.exe", "rb").read()
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
    return dict(name=name, d=d, ib=ib, secs=secs)


def roff(secs, rva):
    for nm, va, vsz, rraw, rsz in secs:
        if va <= rva < va + max(vsz, rsz):
            o = rva - va
            return rraw + o if o < rsz else None
    return None


def fp(E, rva, maxn=24):
    o = roff(E["secs"], rva)
    if o is None:
        return None, None, ""
    st, lo, hu = [], [], []
    for i in md.disasm(E["d"][o:o + 200], rva):
        ops = i.op_str
        s = RIP.sub("[rip+I]", ops)
        if BR.match(i.mnemonic):
            s = HEX.sub("I", s)
        st.append(f"{i.mnemonic} {s}".strip())
        lo.append(f"{i.mnemonic} {HEX.sub('I', ops)}".strip())
        hu.append(f"{i.mnemonic} {ops}".strip())
        if i.mnemonic in ("ret", "jmp") or len(st) >= maxn:
            break
    if not st:
        return None, None, ""
    return " | ".join(st), " | ".join(lo), "; ".join(hu)


def fnptr_pool(E):
    """데이터 섹션의 8B 정렬 코드포인터 전수 → {fn_rva: 참조수}"""
    d, ib, secs = E["d"], E["ib"], E["secs"]
    txt = [s for s in secs if s[0] == ".text"][0]
    tva, tsz = txt[1], max(txt[2], txt[4])
    pool = collections.Counter()
    for nm, va, vsz, rraw, rsz in secs:
        if nm not in (".rdata", ".data"):
            continue
        end = rraw + rsz
        p = rraw + ((-va) % 8)
        while p + 8 <= end:
            v = struct.unpack_from("<Q", d, p)[0]
            if v > ib and tva <= (v - ib) < tva + tsz:
                pool[v - ib] += 1
            p += 8
    return pool


E = {nm: load(nm) for nm in BASES + [TARGET]}
ET = E[TARGET]
print("0.5.3 함수포인터 풀 수집 중...", file=sys.stderr)
POOL = fnptr_pool(ET)
print(f"  코드포인터 대상 {len(POOL):,}종", file=sys.stderr)

print("0.5.3 지문 색인 중...", file=sys.stderr)
IS, IL = collections.defaultdict(list), collections.defaultdict(list)
for fn in POOL:
    s, l, _ = fp(ET, fn)
    if s:
        IS[s].append(fn)
        IL[l].append(fn)

recs = json.load(open(MAP, encoding="utf-8"))
todo = [x for x in recs if x["mode"] in ("후보0", "베이스미상")]
print(f"\n잔여 {len(todo)}종 전역 검색\n")

res = {}
for x in todo:
    r = x["rva"]
    # 베이스 재탐색: vtable 등장이 아니라 "함수포인터 풀 등장"으로 판정
    base, pool_b = None, None
    for nm in BASES:
        pb = fnptr_pool(E[nm]) if nm not in globals().get("_PB", {}) else _PB[nm]
        globals().setdefault("_PB", {})[nm] = pb
        if pb.get(r):
            base, pool_b = nm, pb
            break
    if base is None:
        # 풀에 없으면 프롤로그가 가장 함수 같은 버전을 고른다
        for nm in BASES:
            s, l, hu = fp(E[nm], r)
            if s and (s.startswith("push") or s.startswith("sub rsp") or " ret" in s):
                base = nm
                break
    if base is None:
        print(f"0x{r:<10x} L{x['line']:<6} → 베이스 결정 실패")
        continue
    s, l, hu = fp(E[base], r)
    cs, cl = IS.get(s, []), IL.get(l, [])
    cands = cs if cs else cl
    mode = "strict" if cs else ("loose" if cl else "없음")
    ref = pool_b.get(r, 0) if pool_b else 0
    print(f"0x{r:<10x}[{base[5:]}] L{x['line']:<6} 참조수={ref:<4} {mode:<7} 후보 {len(cands)}")
    print(f"     {hu[:120]}")
    for f in sorted(cands, key=lambda f: -POOL[f])[:6]:
        _, _, h2 = fp(ET, f)
        print(f"       0x{f:<10x} 0.5.3참조={POOL[f]:<5} {h2[:96]}")
    res[hex(r)] = dict(base=base, mode=mode, ref=ref,
                       new=[hex(f) for f in sorted(cands, key=lambda f: -POOL[f])],
                       new_ref={hex(f): POOL[f] for f in cands})

json.dump(res, open(r"C:\tfm2mods\_vtslot_053_rest.json", "w", encoding="utf-8"),
          ensure_ascii=False, indent=1)
print("\n→ C:\\tfm2mods\\_vtslot_053_rest.json 저장")
