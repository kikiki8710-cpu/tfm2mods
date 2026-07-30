# -*- coding: utf-8 -*-
# vtslot7_053.py — ★슬롯 함수 매핑기 v2 (vtslot6 개선판). **이것이 결과 산출용 정본 도구.**
#
# vtslot6 대비 개선:
#   ① 지문 2단계: strict(구조체 필드 변위 **보존**, 분기타겟·rip-rel 만 마스킹) → 실패 시 loose(전 숫자 마스킹).
#      ⟹ `mov rax,[rcx+0x18]` 과 `mov rax,[rcx+0x20]` 이 뒤섞이던 문제 해소.
#   ② 후보를 1:1 로 강제하지 않고 **집합**으로 낸다.
#      이유: 게임은 CGU 마다 같은 동작의 스텁을 복제한다(`xor eax,eax; ret` 다수).
#            우리 코드는 런타임에 읽은 슬롯 값을 match 하므로 **그 슬롯에 실제로 등장하는 동작동일 함수 전부**를 등재해야 커버리지가 온전하다.
#   ③ 슬롯 분포 벡터(어느 슬롯에 몇 번)를 함께 찍어 사람이 검수 가능하게 한다.
#
# 출력: _vtslot_053_map.json  = {소스RVA: {"base":버전, "strict":bool, "new":[0.5.3 RVA...], ...}}
import struct, sys, io, re, json, collections
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8")
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
md = Cs(CS_ARCH_X86, CS_MODE_64)

SRC = r"C:\tfm2mods\tfm2_ai_adjust\src\disc19_repro.rs"
ROOT = r"C:\Users\dev\Desktop\claude\tfm2"
BASES = ["tfm2_0.5.1", "tfm2_0.5.0_3", "tfm2_0.5.0_2", "tfm2_0.5.0"]
TARGET = "tfm2_0.5.3"

BR = re.compile(r"^(j\w+|call|loop\w*)$")
RIP = re.compile(r"\[rip [+\-] 0x[0-9a-f]+\]")
HEX = re.compile(r"0x[0-9a-f]+")


def load(name):
    p = rf"{ROOT}\{name}\TeamfightManager2.exe"
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
    return dict(name=name, d=d, ib=ib, secs=secs)


def roff(secs, rva):
    for nm, va, vsz, rraw, rsz in secs:
        if va <= rva < va + max(vsz, rsz):
            o = rva - va
            return rraw + o if o < rsz else None
    return None


def fp(E, rva, maxn=24):
    """(strict, loose, human) 지문."""
    o = roff(E["secs"], rva)
    if o is None:
        return None, None, ""
    st, lo, hu = [], [], []
    for i in md.disasm(E["d"][o:o + 200], rva):
        ops = i.op_str
        s = RIP.sub("[rip+I]", ops)
        if BR.match(i.mnemonic):
            s = HEX.sub("I", s)                 # 분기·호출 타겟만 마스킹
        st.append(f"{i.mnemonic} {s}".strip())
        lo.append(f"{i.mnemonic} {HEX.sub('I', ops)}".strip())
        hu.append(f"{i.mnemonic} {ops}".strip())
        if i.mnemonic in ("ret", "jmp") or len(st) >= maxn:
            break
    if not st:
        return None, None, ""
    return " | ".join(st), " | ".join(lo), "; ".join(hu)


def enum_vtables(E):
    d, ib, secs = E["d"], E["ib"], E["secs"]
    txt = [s for s in secs if s[0] == ".text"][0]
    tva, tsz = txt[1], max(txt[2], txt[4])

    def is_code(v):
        return v > ib and tva <= (v - ib) < tva + tsz

    vts = []
    for nm, va, vsz, rraw, rsz in secs:
        if nm not in (".rdata", ".data"):
            continue
        end = rraw + rsz
        p = rraw + ((-va) % 8)
        while p + 24 <= end:
            a = struct.unpack_from("<Q", d, p)[0]
            b = struct.unpack_from("<Q", d, p + 8)[0]
            c = struct.unpack_from("<Q", d, p + 16)[0]
            if is_code(a) and b < 0x10000 and 0 < c <= 64 and (c & (c - 1)) == 0:
                slots, q, off, miss = {}, p + 24, 0x18, 0
                while q + 8 <= end and off < 0x600:
                    v = struct.unpack_from("<Q", d, q)[0]
                    if is_code(v):
                        slots[off] = v - ib
                        miss = 0
                    else:
                        miss += 1
                        if miss >= 2:
                            break
                    q += 8
                    off += 8
                if len(slots) >= 4:
                    vts.append(slots)
                    p = q
                    continue
            p += 8
    return vts


def slot_dist(vts):
    out = collections.defaultdict(collections.Counter)
    for slots in vts:
        for off, fn in slots.items():
            out[off][fn] += 1
    return out


# ── 소스 파싱 ────────────────────────────────────────────────────
txt = open(SRC, encoding="utf-8").read()
sites = collections.OrderedDict()
for ln, s in enumerate(txt.splitlines(), 1):
    for m in re.finditer(r"0x([0-9a-f]{6,7})\s*(=>|\|)", s):
        sites.setdefault(int(m.group(1), 16), []).append((ln, s.strip()))

print("exe 로드·vtable 열거 중...", file=sys.stderr)
E, DIST = {}, {}
for nm in BASES + [TARGET]:
    E[nm] = load(nm)
    DIST[nm] = slot_dist(enum_vtables(E[nm]))
ET, TG = E[TARGET], DIST[TARGET]

print("0.5.3 지문 색인 중...", file=sys.stderr)
TS, TL = {}, {}          # (off, 지문) -> {fn: cnt}
TCNT = collections.Counter()
TSLOT = collections.defaultdict(collections.Counter)   # fn -> {off: cnt}
for off, cnt in TG.items():
    for fn, c in cnt.items():
        TCNT[fn] += c
        TSLOT[fn][off] += c
        s, l, _ = fp(ET, fn)
        if s:
            TS.setdefault((off, s), collections.Counter())[fn] += c
            TL.setdefault((off, l), collections.Counter())[fn] += c

out = []
for r in sorted(sites):
    base = None
    for nm in BASES:
        if any(DIST[nm][off][r] for off in DIST[nm]):
            base = nm
            break
    rec = dict(rva=r, base=base, line=sites[r][0][0], src=sites[r][0][1][:110])
    if base is None:
        rec.update(mode="베이스미상", new=[])
        out.append(rec)
        continue
    EB = E[base]
    offs = {off: DIST[base][off][r] for off in DIST[base] if DIST[base][off][r]}
    s, l, hu = fp(EB, r)
    rec.update(offs={hex(k): v for k, v in sorted(offs.items())}, tot=sum(offs.values()), human=hu)
    for mode, IDX, key in (("strict", TS, s), ("loose", TL, l)):
        cand = collections.Counter()
        for off in offs:
            for fn, c in IDX.get((off, key), {}).items():
                cand[fn] += c
        if cand:
            rec.update(mode=mode, new=[hex(f) for f, _ in cand.most_common()],
                       new_cnt={hex(f): c for f, c in cand.most_common()},
                       new_slots={hex(f): {hex(k): v for k, v in sorted(TSLOT[f].items())}
                                  for f, _ in cand.most_common()[:4]})
            break
    else:
        rec.update(mode="후보0", new=[])
    out.append(rec)

mc = collections.Counter(x["mode"] for x in out)
print(f"\n판정 모드 분포: {dict(mc)}")
print(f"베이스 분포: {dict(collections.Counter(x['base'] for x in out))}\n")

for want in ("strict", "loose", "후보0", "베이스미상"):
    sel = [x for x in out if x["mode"] == want]
    if not sel:
        continue
    print("=" * 138)
    print(f"■ {want} ({len(sel)}종)")
    print("=" * 138)
    for x in sel:
        bs = x["base"][5:] if x["base"] else "-"
        slots = " ".join(f"{k}×{v}" for k, v in x.get("offs", {}).items())
        n = len(x["new"])
        head = f"0x{x['rva']:<10x}[{bs}] L{x['line']:<6}"
        if n == 1:
            print(f"{head} → ★{x['new'][0]:<11} (유일)   슬롯 {slots}")
        elif n:
            lst = ", ".join(f"{f}×{x['new_cnt'][f]}" for f in x["new"][:5])
            print(f"{head} → {n}후보: {lst}   슬롯 {slots} (합{x.get('tot')})")
        else:
            print(f"{head} → (없음)   슬롯 {slots}")
        if x.get("human"):
            print(f"      {x['human'][:118]}")

json.dump(out, open(r"C:\tfm2mods\_vtslot_053_map.json", "w", encoding="utf-8"),
          ensure_ascii=False, indent=1)
print("\n→ C:\\tfm2mods\\_vtslot_053_map.json 저장")
