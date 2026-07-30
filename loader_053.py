# -*- coding: utf-8 -*-
# loader_053.py — UI asset loader(LOADER_RVA) 를 문자열 xref 로 확정.
#   왜: 0.5.3 에서 push8 프롤로그는 66,635곳에 등장 = 변별력 0. 프롤로그 일치만으로 잡은 후보는 신뢰할 수 없다.
#   방법: `"asset/base/ui/layout/..."` 리터럴의 주소를 .rdata 에서 찾고 → 그 주소를 lea 하는 .text 사이트를 찾고
#         → 그 직후의 call 타겟을 모아 최빈값을 취한다. (0.5.2 에서 같은 절차가 알려진 정답 0x5ac950 을 재생산하는지 먼저 검증)
import struct, sys, io, re, collections
import bytepatch_053 as B   # ⚠ 이 모듈이 stdout 을 utf-8 로 재래핑한다 — 여기서 또 감싸면 앞 wrapper 가 buffer 를 닫는다

roff, owner = B.roff, B.owner


def sec(secs, nm):
    for n, va, vsz, rr, rs in secs:
        if n == nm:
            return va, vsz, rr, rs
    return None


def find_str(d, secs, lit):
    """.rdata 에서 리터럴 바이트열 위치(RVA) 전부"""
    va, vsz, rr, rs = sec(secs, ".rdata")
    blob = d[rr:rr + rs]
    return [va + m.start() for m in re.finditer(re.escape(lit), blob)]


def lea_to(d, secs, target):
    """target RVA 를 가리키는 `lea reg,[rip+x]` 사이트 (48/4c 8d ModRM /5)"""
    va, vsz, rr, rs = sec(secs, ".text")
    blob = d[rr:rr + rs]
    out = []
    for m in re.finditer(rb'[\x48\x4c]\x8d[\x05\x0d\x15\x1d\x25\x2d\x35\x3d](....)', blob, re.S):
        s = va + m.start()
        rel = int.from_bytes(m.group(1), "little", signed=True)
        if s + 7 + rel == target:
            out.append(s)
    return out


def call_after(d, secs, site, window=64):
    """site 이후 window 바이트 안의 첫 `e8 rel32` 타겟"""
    va, vsz, rr, rs = sec(secs, ".text")
    o = roff(secs, site)
    blob = d[o:o + window]
    for m in re.finditer(rb'\xe8(....)', blob, re.S):
        s = site + m.start()
        rel = int.from_bytes(m.group(1), "little", signed=True)
        return s + 5 + rel
    return None


def analyze(tag, d, secs, fns, lits):
    print(f"===== {tag} =====")
    votes = collections.Counter()
    for lit in lits:
        addrs = find_str(d, secs, lit)
        if not addrs:
            print(f"  '{lit.decode(errors='replace')}' — .rdata 에 없음")
            continue
        for a in addrs:
            sites = lea_to(d, secs, a)
            for s in sites:
                t = call_after(d, secs, s)
                if t:
                    votes[t] += 1
            if sites:
                print(f"  '{lit.decode(errors='replace')}' @0x{a:x} — lea {len(sites)}곳 → "
                      f"call {[hex(call_after(d,secs,x)) for x in sites[:4] if call_after(d,secs,x)]}")
    print(f"  ▶ call 타겟 집계: " + ", ".join(f"0x{t:x}×{c}" for t, c in votes.most_common(6)))
    return votes


LITS = [b"asset/base/ui/layout/main", b"asset/base/ui/layout/strategy",
        b"asset/base/ui/layout/player_info", b"asset/base/ui/layout/training"]

vo = analyze("0.5.2 (정답 = 0x5ac950 이 나와야 방법 검증)", B.DO, B.SO, B.FO, LITS)
print()
vn = analyze("0.5.3", B.DN, B.SN, B.FN, LITS)
print()
print("후보 대조:")
for t, c in vn.most_common(6):
    f = owner(B.FN, t)
    o = roff(B.SN, t)
    print(f"   0x{t:x} ×{c}  fn시작={'0x%x' % f[0] if f else '?'}  선두12B={B.DN[o:o+12].hex(' ')}")
print(f"   (ai_adjust 채택값 0x91ab0 / item_tactics 주장 0x2e1550)")
