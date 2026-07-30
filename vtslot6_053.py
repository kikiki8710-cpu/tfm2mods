# -*- coding: utf-8 -*-
# vtslot6_053.py — ★슬롯 함수 자동 매핑기 (0.5.x → 0.5.3).
#
# 배경(vtslot1~5 결과):
#   - `disc19_repro.rs` 의 vtable 슬롯 RVA 테이블 52종은 **버전 혼재**다.
#     0.5.1 에서 vtable 등장이 있는 것 / 0.5.0 계열에서만 유효한 것이 섞여 있다(0.5.2 마이그에서 미재핀).
#   - vtable 슬롯 오프셋(0x28/0x48/0x50/0x58/0x78/0x90/0xc8/0xd8/0x110)은 소스의 `which` 와 일치.
#   - 두 exe 의 슬롯별 함수 분포가 거의 1:1 대응(프롤로그 동일 + 등장수 근사).
#
# 방법:
#   ① 소스 RVA 마다 "vtable 등장이 있는 버전"을 찾아 베이스로 삼는다.
#   ② 베이스에서 (슬롯 오프셋별 등장수, 함수 스켈레톤 지문)을 뽑는다.
#   ③ 0.5.3 의 같은 슬롯에서 스켈레톤이 동일한 함수를 후보로 모으고, 등장수 근접도로 순위를 매긴다.
#   ④ 후보 1개 = 확정 / 복수 = 등장수·슬롯분포 일치도로 랭킹해 사람이 판정.
#
# 스켈레톤 = 명령 니모닉 + 오퍼랜드에서 즉값/변위/분기타겟을 I 로 치환한 문자열(함수 끝 또는 24명령까지).
#   ⟹ 재컴파일로 주소가 바뀌어도 동일, 레지스터 배정이 바뀌면 달라진다(그건 별도 완화 지문으로 처리).
import struct, sys, io, re, json, collections
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8")
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
md = Cs(CS_ARCH_X86, CS_MODE_64)

SRC = r"C:\tfm2mods\tfm2_ai_adjust\src\disc19_repro.rs"
ROOT = r"C:\Users\dev\Desktop\claude\tfm2"
BASES = ["tfm2_0.5.1", "tfm2_0.5.0_3", "tfm2_0.5.0_2", "tfm2_0.5.0"]   # 우선순위 순
TARGET = "tfm2_0.5.3"
RE_NUM = re.compile(r"0x[0-9a-f]+|(?<![a-z0-9])\d+")


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


def skel(E, rva, maxn=24):
    """스켈레톤 지문 + 사람이 읽을 디스어셈."""
    o = roff(E["secs"], rva)
    if o is None:
        return None, ""
    parts, human = [], []
    for i in md.disasm(E["d"][o:o + 160], rva):
        ops = RE_NUM.sub("I", i.op_str)
        parts.append(f"{i.mnemonic} {ops}".strip())
        human.append(f"{i.mnemonic} {i.op_str}".strip())
        if i.mnemonic in ("ret", "jmp") or len(parts) >= maxn:
            break
    if not parts:
        return None, ""
    return " | ".join(parts), "; ".join(human)


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


# ── 소스 테이블 파싱 ──────────────────────────────────────────────
txt = open(SRC, encoding="utf-8").read()
lines = txt.splitlines()
sites = collections.OrderedDict()
for ln, s in enumerate(lines, 1):
    for m in re.finditer(r"0x([0-9a-f]{6,7})\s*(=>|\|)", s):
        sites.setdefault(int(m.group(1), 16), []).append((ln, s.strip()))

# ── exe 로드 ─────────────────────────────────────────────────────
print("exe 로드·vtable 열거 중...", file=sys.stderr)
E = {}
DIST = {}
for nm in BASES + [TARGET]:
    E[nm] = load(nm)
    DIST[nm] = slot_dist(enum_vtables(E[nm]))
    print(f"  {nm}: 슬롯 {len(DIST[nm])}종", file=sys.stderr)

TG = DIST[TARGET]
ET = E[TARGET]

# 0.5.3 슬롯별 스켈레톤 인덱스
print("0.5.3 스켈레톤 색인 중...", file=sys.stderr)
TSKEL = {}          # (off, skel) -> [(fn, cnt)]
for off, cnt in TG.items():
    for fn, c in cnt.items():
        sk, _ = skel(ET, fn)
        if sk:
            TSKEL.setdefault((off, sk), []).append((fn, c))

out = []
for r in sorted(sites):
    # ① 베이스 버전 결정 = vtable 등장이 있는 첫 버전
    base = None
    for nm in BASES:
        tot = sum(DIST[nm][off][r] for off in DIST[nm])
        if tot:
            base = nm
            break
    if base is None:
        out.append(dict(rva=r, base=None, lines=sites[r]))
        continue
    EB = E[base]
    offs = {off: DIST[base][off][r] for off in DIST[base] if DIST[base][off][r]}
    sk, human = skel(EB, r)
    # ③ 0.5.3 후보: 같은 슬롯 + 동일 스켈레톤
    cands = collections.Counter()
    for off in offs:
        for fn, c in TSKEL.get((off, sk), []):
            cands[fn] += c
    out.append(dict(rva=r, base=base, offs=offs, tot=sum(offs.values()),
                    skel=sk, human=human, cands=cands, lines=sites[r]))

# ── 리포트 ───────────────────────────────────────────────────────
nb = collections.Counter(x.get("base") for x in out)
print("\n베이스 버전 분포:", dict(nb))
uniq = [x for x in out if x.get("cands") and len(x["cands"]) == 1]
multi = [x for x in out if x.get("cands") and len(x["cands"]) > 1]
none = [x for x in out if x.get("base") and not x.get("cands")]
nobase = [x for x in out if not x.get("base")]
print(f"판정: 후보1개(확정) {len(uniq)} / 복수 {len(multi)} / 후보0 {len(none)} / 베이스미상 {len(nobase)}\n")

print("=" * 136)
print("★후보 1개 = 확정")
print("=" * 136)
for x in uniq:
    fn, c = next(iter(x["cands"].items())), None
    f, cc = fn
    slots = " ".join(f"{hex(k)}×{v}" for k, v in sorted(x["offs"].items()))
    print(f"0x{x['rva']:<10x}({x['base'][5:]}) → ★0x{f:<10x}  슬롯 {slots:<26} 0.5.3등장={cc:<4} L{x['lines'][0][0]}")
    print(f"    {x['human'][:120]}")

print("\n" + "=" * 136)
print("복수 후보 — 등장수 근접도로 랭킹")
print("=" * 136)
for x in multi:
    slots = " ".join(f"{hex(k)}×{v}" for k, v in sorted(x["offs"].items()))
    print(f"0x{x['rva']:<10x}({x['base'][5:]}) 슬롯 {slots:<26} L{x['lines'][0][0]}  {x['human'][:80]}")
    rank = sorted(x["cands"].items(), key=lambda kv: -kv[1])
    for f, c in rank[:6]:
        mark = "★" if c == x["tot"] else " "
        print(f"     {mark}0x{f:<10x} 0.5.3등장={c:<5} (0.5.x={x['tot']})")

print("\n" + "=" * 136)
print("후보 0 (스켈레톤 불일치 = 코드 변경) / 베이스 미상")
print("=" * 136)
for x in none:
    slots = " ".join(f"{hex(k)}×{v}" for k, v in sorted(x["offs"].items()))
    print(f"0x{x['rva']:<10x}({x['base'][5:]}) 슬롯 {slots:<26} L{x['lines'][0][0]}  {x['human'][:90]}")
for x in nobase:
    print(f"0x{x['rva']:<10x}(베이스미상) L{x['lines'][0][0]}  {x['lines'][0][1][:100]}")

json.dump([{k: (v if k != "cands" else dict(v)) for k, v in x.items() if k != "lines"} for x in out],
          open(r"C:\tfm2mods\_vtslot_053.json", "w", encoding="utf-8"), ensure_ascii=False, indent=1)
print("\n→ C:\\tfm2mods\\_vtslot_053.json 저장")
