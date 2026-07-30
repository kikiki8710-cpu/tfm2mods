# -*- coding: utf-8 -*-
# ct_053d.py — 남은 사이트 정밀 처리:
#   ①btn5v5 3종 = "cmp r?,0xa + 후속 명령" 패턴을 **전역** 스캔(컨테이너 매칭 실패/오염 회피)
#   ②daily_remaining = pdata 없는 leaf 33B 본문시그 전역 스캔(0.5.2 방식 그대로)
#   ⚠앞 판(ct_053c)에서 btn5v5 는 imm 을 정규화해버려 `cmp r12,0x30` 을 잡는 오답이 났다 ⟹ imm 고정.
import re, collections
import bytepatch_053 as B
import dov_053b as G
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
md = Cs(CS_ARCH_X86, CS_MODE_64)
roff, owner = B.roff, B.owner


def sec(secs, nm):
    for n, va, vsz, rr, rs in secs:
        if n == nm:
            return va, vsz, rr, rs


def scan(d, secs, pat):
    va, vsz, rr, rs = sec(secs, ".text")
    return [va + m.start() for m in re.finditer(re.escape(pat), d[rr:rr + rs])]


def ctxdis(d, secs, rva, back=16, fwd=40):
    o = roff(secs, rva)
    return " | ".join(f"{i.mnemonic} {i.op_str}" for i in md.disasm(d[o:o + fwd], rva))


print("=" * 100)
print("① btn5v5 3종 — 원 8B 시그 전역 스캔 (imm 0xa 고정)")
print("=" * 100)
SIGS = [("btn5v5_roster_min_a", 0xd967cf, bytes.fromhex("4983fc0ab801000000")),
        ("btn5v5_roster_min_b", 0xcf7b68, bytes.fromhex("4983fd0a410f92c5")),
        ("btn5v5_warn_text", 0xd9662c, bytes.fromhex("4883fb0ab838000000"))]
for nm, old, sig in SIGS:
    for tag, d, secs in (("0.5.2", B.DO, B.SO), ("0.5.3", B.DN, B.SN)):
        hits = scan(d, secs, sig)
        print(f"  {nm:22s} {tag} 시그 {sig.hex(' ')} → {len(hits)}건 "
              + ", ".join(f"0x{h:x}" for h in hits[:6]))
    print(f"      0.5.2 문맥: {ctxdis(B.DO, B.SO, old)[:150]}")
    print()

print("=" * 100)
print("② btn5v5 — 시그 완화(cmp r?,0xa 뒤 동일 후속) 전역 스캔")
print("=" * 100)
# cmp r12/r13/rbx, 0xa 는 REX.B/모드에 따라 49 83 f? 0a / 48 83 f? 0a
for nm, old, sig in SIGS:
    core = sig[:4]                       # cmp r?,0xa
    tail = sig[4:]
    for tag, d, secs in (("0.5.2", B.DO, B.SO), ("0.5.3", B.DN, B.SN)):
        # 레지스터 무관: 48/49 83 f8~ff 0a
        va, vsz, rr, rs = sec(secs, ".text")
        blob = d[rr:rr + rs]
        pat = re.compile(rb'[\x48\x49]\x83[\xf8-\xff]\x0a' + re.escape(tail[:2]), re.S)
        hits = [va + m.start() for m in pat.finditer(blob)]
        print(f"  {nm:22s} {tag} [cmp r?,0xa + {tail[:2].hex(' ')}] → {len(hits)}건 "
              + ", ".join(f"0x{h:x}" for h in hits[:8]))
    print()

print("=" * 100)
print("③ daily_remaining — leaf 33B 본문 시그 전역 스캔")
print("=" * 100)
o = roff(B.SO, 0x1f14090)
body = B.DO[o:o + 33]
print(f"  0.5.2 본문 33B: {body.hex(' ')}")
print(f"  디스어셈: {ctxdis(B.DO, B.SO, 0x1f14090, fwd=33)}")
for tag, d, secs in (("0.5.2", B.DO, B.SO), ("0.5.3", B.DN, B.SN)):
    hits = scan(d, secs, body)
    print(f"  {tag} 전체 33B 시그 → {len(hits)}건 " + ", ".join(f"0x{h:x}" for h in hits[:5]))
# 앞 16B 만
for tag, d, secs in (("0.5.2", B.DO, B.SO), ("0.5.3", B.DN, B.SN)):
    hits = scan(d, secs, body[:16])
    print(f"  {tag} 앞 16B 시그   → {len(hits)}건 " + ", ".join(f"0x{h:x}" for h in hits[:5]))
