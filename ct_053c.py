# -*- coding: utf-8 -*-
# ct_053c.py — comptest PATCHES 재핀 3판: orig 바이트 일치를 **요구하지 않는다**.
#   이유: 0.5.3 재컴파일로 점프 거리(75 76 → 75 XX)와 레지스터(cmp r12 → cmp rXX)가 바뀌었다.
#         ⟹ orig 를 고정 시그로 쓰면 전부 MISS. 대신 "명령 종류 + 피연산자 형태"로 후보를 잡고
#            문맥(±4 명령 정규화 시퀀스) 점수로 확정한 뒤, **새 orig/fixed 를 실측 바이트로 재생성**한다.
#   fixed 재생성 규칙(사이트 의미별):
#     nop6/nop2 = 점프 무력화 / imm 교체 = cmp 임계값 / no-op = 기록용(orig==fixed)
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
# 사이트별 처리규칙: kind = jmp_nop(점프→nop) / imm(임계값 교체) / noop(기록용)
KIND = {
    "no_stamina_cost": ("imm", None), "daily_inc_gate": ("imm", 0x7f),
    "server_dedup_real": ("jmp_nop", None), "allow_dup_players": ("jmp_nop", None),
    "server_dedup": ("noop", None), "btn5v5_roster_min_a": ("imm", 5),
    "btn5v5_roster_min_b": ("imm", 5), "btn5v5_warn_text": ("imm", 5),
    "roster_count_gate": ("jmp_nop", None), "collected_gate": ("jmp_nop", None),
    "collect_err_gate": ("jmp_nop", None), "run_push_gate": ("jmp_nop", None),
}
NOP = {1: b"\x90", 2: b"\x66\x90", 3: b"\x0f\x1f\x00", 4: b"\x0f\x1f\x40\x00",
       5: b"\x0f\x1f\x44\x00\x00", 6: b"\x66\x0f\x1f\x44\x00\x00"}


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


def shape(i):
    """명령의 '형태' — 니모닉 + 피연산자 종류(레지스터명은 유지, 상수는 I)"""
    return norm(i)


def dis(d, secs, fns, start):
    f = owner(fns, start)
    if not f:
        return None, None
    o = roff(secs, f[0])
    return list(md.disasm(d[o:o + (f[1] - f[0])], f[0])), f


def find_container(oc):
    v = collections.Counter()
    for cf, cnt in CO.get(oc, {}).items():
        if cf in A:
            for t, k in EN.get(A[cf], {}).items():
                v[t] += 1 if k == cnt else 0.3
    if not v:
        return None
    return v.most_common(1)[0][0]


sites = parse()
ocache, ncache = {}, {}
RES = {}
print(f"PATCHES {len(sites)}건 — 명령형태+문맥 재핀\n")
for s in sites:
    rva, nm, orig = s["rva"], s["name"], s["orig"]
    kind, newimm = KIND.get(nm, (None, None))
    f = owner(B.FO, rva)
    if not f:
        print(f"  [LEAF ] {nm:22s} 0x{rva:x} — 별도 처리")
        continue
    cont = f[0]
    if cont not in ocache:
        ocache[cont] = dis(B.DO, B.SO, B.FO, cont)
    ins, _ = ocache[cont]
    idx = next((k for k, i in enumerate(ins) if i.address <= rva < i.address + i.size), None)
    hit = ins[idx]
    off = rva - hit.address
    sh = shape(hit)
    ctx = [norm(i) for i in ins[max(0, idx - 5): idx + 6]]
    nc = A.get(cont) or find_container(cont)
    if not nc:
        print(f"  [NOCONT] {nm:22s} 컨테이너 0x{cont:x} 도출 실패")
        continue
    if nc not in ncache:
        ncache[nc] = dis(B.DN, B.SN, B.FN, nc)
    nins, nf = ncache[nc]
    cands = []
    for k, i in enumerate(nins):
        if shape(i) != sh or i.size != hit.size:
            continue
        nctx = [norm(x) for x in nins[max(0, k - 5): k + 6]]
        score = sum(1 for a in ctx if a in nctx)
        cands.append((score, i.address, k, i))
    cands.sort(key=lambda x: (-x[0], x[1]))
    if not cands:
        print(f"  [MISS ] {nm:22s} 0x{rva:x} 형태 [{sh}] 후보 0 (컨테이너 0x{nc:x})")
        continue
    uniq = len(cands) == 1 or cands[0][0] > cands[1][0]
    sc, addr, k, ni = cands[0]
    site = addr + off
    nb = bytes(ni.bytes)
    # 새 orig/fixed 생성
    if kind == "jmp_nop":
        norig, nfixed = nb, NOP[len(nb)]
        site = addr                      # 점프 사이트는 명령 시작
    elif kind == "noop":
        norig = nfixed = B.DN[roff(B.SN, site): roff(B.SN, site) + len(orig)]
    elif kind == "imm":
        oi = B.DO[roff(B.SO, rva): roff(B.SO, rva) + len(orig)]
        norig = B.DN[roff(B.SN, site): roff(B.SN, site) + len(orig)]
        nfixed = bytes([newimm if newimm is not None else 0]) + norig[1:] if len(orig) == 1 else None
        if len(orig) > 1:
            # orig 안의 imm 바이트(0xa 등) 를 새 값으로
            nfixed = bytearray(norig)
            for j, bb in enumerate(norig):
                if bb == oi[j] and bb in (0x0a, 0x04, 0x05):
                    nfixed[j] = newimm if newimm is not None else 0
            nfixed = bytes(nfixed)
        if len(orig) == 1:
            nfixed = bytes([newimm if newimm is not None else 0])
    else:
        norig = nfixed = None
    tag = "OK" if uniq else "AMBIG"
    RES[nm] = dict(new=site, orig=norig.hex(' ') if norig else None,
                   fixed=nfixed.hex(' ') if nfixed else None, grade=tag, score=sc,
                   cands=[hex(c[1]) for c in cands[:4]])
    print(f"  [{tag:5s}] {nm:22s} 0x{rva:x} → **0x{site:x}**  점수 {sc}/11 "
          f"(후보 {len(cands)}, 2위 {cands[1][0] if len(cands)>1 else '-'})  [{sh}]")
    print(f"          0.5.2 orig={orig.hex(' ')} fixed={s['fixed'].hex(' ')}  →  "
          f"0.5.3 orig={norig.hex(' ') if norig else '?'} fixed={nfixed.hex(' ') if nfixed else '?'}")

json.dump(RES, open(r"C:\tfm2mods\_ct_053.json", "w", encoding="utf-8"), indent=1, ensure_ascii=False)
print("\n저장: _ct_053.json")
