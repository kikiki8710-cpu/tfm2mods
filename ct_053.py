# -*- coding: utf-8 -*-
# ct_053.py — comptest_unlock PATCHES(byte-patch) 사이트를 0.5.3 으로 재핀.
#   방법: 사이트를 품은 명령 ±2 의 **정규화 명령 시퀀스**(imm/disp 를 I 로 치환)를 시그로 삼아
#         0.5.3 대응 컨테이너 안에서 검색. + orig 바이트 일치 필수(모드 자체 검증과 동일 조건).
#   ⚠ai_adjust 교훈: 전역 순서대응 금지, 문맥 시그 우선. 유일하지 않으면 미해결로 남긴다(패치 skip=fail-safe).
import re, collections
import bytepatch_053 as B
import dov_053b as G
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
md = Cs(CS_ARCH_X86, CS_MODE_64)
roff, owner, A = B.roff, B.owner, G.A
SRC = r"C:\tfm2mods\tfm2_comptest_unlock\src\tfm2_comptest_unlock.rs"

RE_P = re.compile(
    r'Patch\s*\{\s*name:\s*"([^"]+)"\s*,\s*rva:\s*(0x[0-9a-fA-F]+)\s*,'
    r'(?:[^}]*?)orig:\s*&\[([^\]]*)\]\s*,\s*fixed:\s*&\[([^\]]*)\]', re.S)


def parse():
    src = open(SRC, encoding="utf-8", errors="replace").read()
    out = []
    for m in RE_P.finditer(src):
        b = lambda s: bytes(int(x.strip(), 16) for x in s.split(",") if x.strip())
        out.append(dict(name=m.group(1), rva=int(m.group(2), 16),
                        orig=b(m.group(3)), fixed=b(m.group(4)),
                        line=src[:m.start()].count("\n") + 1))
    return out


def norm(i):
    s = re.sub(r'0x[0-9a-f]+', 'I', i.op_str)
    s = re.sub(r'\b\d+\b', 'I', s)
    return f"{i.mnemonic} {s}"


def dis(d, secs, fns, start):
    f = owner(fns, start)
    if not f:
        return None, None
    o = roff(secs, f[0])
    return list(md.disasm(d[o:o + (f[1] - f[0])], f[0])), f


CTX = 2
sites = parse()
print(f"PATCHES {len(sites)}건\n")
ocache, ncache = {}, {}
res = []
for s in sites:
    rva, orig = s["rva"], s["orig"]
    o = roff(B.SO, rva)
    actual = B.DO[o:o + len(orig)] if o is not None else b""
    if actual != orig:
        print(f"  [STALE] {s['name']:22s} 0x{rva:x} — 0.5.2 orig 불일치 실제={actual.hex(' ')} 기대={orig.hex(' ')}")
        res.append((s, None, "STALE"))
        continue
    f = owner(B.FO, rva)
    if not f:
        # .pdata 없는 leaf (예: daily_remaining) → 본문 전역 시그 스캔으로 처리
        print(f"  [LEAF ] {s['name']:22s} 0x{rva:x} — 컨테이너 없음 → 전역 시그 스캔 필요")
        res.append((s, None, "LEAF"))
        continue
    cont = f[0]
    if cont not in ocache:
        ocache[cont] = dis(B.DO, B.SO, B.FO, cont)
    ins, _ = ocache[cont]
    idx = next((k for k, i in enumerate(ins) if i.address <= rva < i.address + i.size), None)
    if idx is None:
        print(f"  [BAD  ] {s['name']:22s} 0x{rva:x} — 명령 복원 실패")
        res.append((s, None, "BAD"))
        continue
    off_in_ins = rva - ins[idx].address
    sig = [norm(i) for i in ins[max(0, idx - CTX): idx + CTX + 1]]
    pos = idx - max(0, idx - CTX)
    nc = A.get(cont)
    if not nc:
        print(f"  [NOCONT] {s['name']:22s} 0x{rva:x} 컨테이너 0x{cont:x} → 앵커 없음")
        res.append((s, None, "NOCONT"))
        continue
    if nc not in ncache:
        ncache[nc] = dis(B.DN, B.SN, B.FN, nc)
    nins, nf = ncache[nc]
    hits = []
    for k in range(len(nins) - len(sig) + 1):
        if [norm(x) for x in nins[k:k + len(sig)]] != sig:
            continue
        cand = nins[k + pos]
        addr = cand.address + off_in_ins
        no = roff(B.SN, addr)
        if B.DN[no:no + len(orig)] == orig:
            hits.append(addr)
    tag = "OK" if len(hits) == 1 else ("AMBIG" if hits else "MISS")
    res.append((s, hits[0] if len(hits) == 1 else hits, tag))
    print(f"  [{tag:5s}] {s['name']:22s} 0x{rva:x} → "
          + (f"**0x{hits[0]:x}**" if len(hits) == 1 else (f"{len(hits)}건 " + ",".join(hex(h) for h in hits[:4]) if hits else "없음"))
          + f"   컨테이너 0x{cont:x}→0x{nc:x} orig={orig.hex(' ')}")

print("\n요약:", collections.Counter(t for _, _, t in res))
