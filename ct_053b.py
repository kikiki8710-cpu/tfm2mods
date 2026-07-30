# -*- coding: utf-8 -*-
# ct_053b.py — comptest PATCHES 재핀(개선판).
#   ①시그 = 사이트를 품은 **명령 전체 바이트**(rip-rel/rel32 는 마스킹) → 컨테이너 안 후보 수집
#   ②후보가 여럿이면 앞뒤 ±4 명령의 정규화 시퀀스 유사도(공통 개수)로 순위 — 완전일치 요구 X
#   ③1위가 2위보다 확실히 높고 orig 바이트가 맞을 때만 확정. 아니면 미해결(패치 skip=fail-safe)
#   ④컨테이너 앵커가 없으면 콜러-대응 투표로 컨테이너부터 도출
import re, collections, json
import bytepatch_053 as B
import dov_053b as G
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
md = Cs(CS_ARCH_X86, CS_MODE_64)
md.detail = True
roff, owner, A = B.roff, B.owner, G.A
CO, EO, CN, EN = G.CO, G.EO, G.CN, G.EN
SRC = r"C:\tfm2mods\tfm2_comptest_unlock\src\tfm2_comptest_unlock.rs"
RE_P = re.compile(
    r'Patch\s*\{\s*name:\s*"([^"]+)"\s*,\s*rva:\s*(0x[0-9a-fA-F]+)\s*,'
    r'(?:[^}]*?)orig:\s*&\[([^\]]*)\]\s*,\s*fixed:\s*&\[([^\]]*)\]', re.S)


def parse():
    src = open(SRC, encoding="utf-8", errors="replace").read()
    out = []
    for m in RE_P.finditer(src):
        b = lambda s: bytes(int(x.strip(), 16) for x in s.split(",") if x.strip())
        out.append(dict(name=m.group(1), rva=int(m.group(2), 16), orig=b(m.group(3)),
                        fixed=b(m.group(4)), line=src[:m.start()].count("\n") + 1))
    return out


def norm(i):
    return f"{i.mnemonic} " + re.sub(r'\b(0x[0-9a-f]+|\d+)\b', 'I', i.op_str)


def dis(d, secs, fns, start):
    f = owner(fns, start)
    if not f:
        return None, None
    o = roff(secs, f[0])
    return list(md.disasm(d[o:o + (f[1] - f[0])], f[0])), f


def is_rel(i):
    """rel32/rip-rel 을 가진 명령인가 (마지막 4B 가 상대주소)"""
    if i.mnemonic in ("call", "jmp") and i.size == 5:
        return 1
    if i.mnemonic.startswith("j") and i.size == 6:
        return 1
    for op in i.operands:
        if op.type == 3 and op.mem.base == 41:
            return 1
    return 0


def find_container(old_cont):
    """앵커 없는 컨테이너를 콜러-대응 투표로 도출"""
    v = collections.Counter()
    for cf, cnt in CO.get(old_cont, {}).items():
        if cf in A:
            for t, k in EN.get(A[cf], {}).items():
                v[t] += 1 if k == cnt else 0.3
    if not v:
        return None, 0
    top = v.most_common(2)
    return top[0][0], (top[0][1] / max(1e-9, top[1][1]) if len(top) > 1 else 99)


sites = parse()
print(f"PATCHES {len(sites)}건 (0.5.2 orig 검증 → 0.5.3 재핀)\n")
ocache, ncache = {}, {}
out = {}
stat = collections.Counter()
for s in sites:
    rva, orig, nm = s["rva"], s["orig"], s["name"]
    o = roff(B.SO, rva)
    if o is None or B.DO[o:o + len(orig)] != orig:
        print(f"  [STALE] {nm:22s} 0x{rva:x} 0.5.2 orig 불일치")
        stat["STALE"] += 1
        continue
    f = owner(B.FO, rva)
    if not f:
        print(f"  [LEAF ] {nm:22s} 0x{rva:x} 컨테이너 없음(leaf) — 전역 본문시그 스캔 대상")
        stat["LEAF"] += 1
        continue
    cont = f[0]
    if cont not in ocache:
        ocache[cont] = dis(B.DO, B.SO, B.FO, cont)
    ins, _ = ocache[cont]
    idx = next((k for k, i in enumerate(ins) if i.address <= rva < i.address + i.size), None)
    if idx is None:
        print(f"  [BAD  ] {nm:22s} 0x{rva:x}")
        stat["BAD"] += 1
        continue
    hit = ins[idx]
    off = rva - hit.address
    sig = bytes(hit.bytes)
    mask = is_rel(hit)
    if mask:
        sig = sig[:-4]
    ctx = [norm(i) for i in ins[max(0, idx - 4): idx + 5]]
    nc = A.get(cont)
    how = "앵커"
    if not nc:
        nc, ratio = find_container(cont)
        how = f"투표(비 {ratio:.1f})"
    if not nc:
        print(f"  [NOCONT] {nm:22s} 0x{rva:x} 컨테이너 0x{cont:x} 도출 실패")
        stat["NOCONT"] += 1
        continue
    if nc not in ncache:
        ncache[nc] = dis(B.DN, B.SN, B.FN, nc)
    nins, nf = ncache[nc]
    if nins is None:
        print(f"  [NOCONT] {nm:22s} NEW 컨테이너 0x{nc:x} 디스어셈 실패")
        stat["NOCONT"] += 1
        continue
    cands = []
    for k, i in enumerate(nins):
        b = bytes(i.bytes)
        if is_rel(i):
            b = b[:-4]
        if b != sig:
            continue
        addr = i.address + off
        no = roff(B.SN, addr)
        if B.DN[no:no + len(orig)] != orig:
            continue
        nctx = [norm(x) for x in nins[max(0, k - 4): k + 5]]
        score = sum(1 for a in ctx if a in nctx)
        cands.append((score, addr))
    cands.sort(reverse=True)
    if not cands:
        print(f"  [MISS ] {nm:22s} 0x{rva:x} 컨테이너 0x{cont:x}→0x{nc:x}({how}) "
              f"시그 {sig.hex(' ')} [{hit.mnemonic} {hit.op_str}] 없음")
        stat["MISS"] += 1
        continue
    top = cands[0]
    uniq = len(cands) == 1 or cands[0][0] > cands[1][0]
    tag = "OK" if uniq else "AMBIG"
    stat[tag] += 1
    out[nm] = top[1] if uniq else [c[1] for c in cands]
    print(f"  [{tag:5s}] {nm:22s} 0x{rva:x} → " + (f"**0x{top[1]:x}**" if uniq else
          f"{len(cands)}건 " + ",".join(f"0x{a:x}({sc})" for sc, a in cands[:4]))
          + f"   문맥점수 {top[0]}/9  컨테이너 0x{nc:x}({how})  [{hit.mnemonic} {hit.op_str}]")

print("\n요약:", dict(stat))
json.dump({k: (v if isinstance(v, int) else v) for k, v in out.items()},
          open(r"C:\tfm2mods\_ct_053.json", "w"), indent=1)
