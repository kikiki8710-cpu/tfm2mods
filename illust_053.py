# -*- coding: utf-8 -*-
# illust_053.py — banpick_illust 잔여 재핀:
#   ①훅 3종(FX_SET/CARD_DRAW/ILLUST_GET) 0.5.3 프롤로그 실측 + ORIG_LEN(명령경계) 산출
#   ②.rdata float 상수 6종 (0.5.2 바이트 그대로 0.5.3 에서 검색)
#   ③mid-func 6종 (컨테이너 안 명령 시그 재탐색)
#   ④RVA_SLOTS (0.5.3 .rdata 의 16B 이상 0 패딩)
import re, struct, collections
import bytepatch_053 as B
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
md = Cs(CS_ARCH_X86, CS_MODE_64)
roff, owner = B.roff, B.owner

HOOKS = [("RVA_FX_SET", 0x11e2370, 0x1bd8e50, 12),
         ("RVA_CARD_DRAW", 0x11f9030, 0x1bee8e0, 12),
         ("RVA_ILLUST_GET", 0xfdabe0, 0x1e91400, 19)]

print("=" * 96)
print("① 훅 프롤로그 실측 (0.5.2 → 0.5.3) + ORIG_LEN 명령경계")
print("=" * 96)
for nm, old, new, olen in HOOKS:
    for tag, d, secs, r in (("0.5.2", B.DO, B.SO, old), ("0.5.3", B.DN, B.SN, new)):
        o = roff(secs, r)
        blob = d[o:o + 32]
        ins = list(md.disasm(blob, r))
        acc, bound = 0, []
        for i in ins:
            acc += i.size
            bound.append(acc)
            if acc >= 24:
                break
        need = next((b for b in bound if b >= 12), None)
        print(f"  {nm:16s} {tag} 0x{r:<9x} {blob[:20].hex(' ')}")
        print(f"      명령경계 {bound[:8]} → 12B 이상 최소경계 = {need}"
              f"{'  (구 ORIG_LEN=%d)' % olen if tag == '0.5.2' else ''}")
    print()

print("=" * 96)
print("② .rdata float 상수 — 0.5.2 바이트를 0.5.3 에서 검색")
print("=" * 96)
CONSTS = [("RVA_C_CARD_RECT", 0x3731380, 4), ("RVA_C_SNAP_RECT", 0x37313b0, 4),
          ("RVA_C_NORMAL", 0x37313c0, 2), ("RVA_C_LINE_DIR", 0x37313e0, 2),
          ("RVA_C_LINE_START", 0x37313f0, 2), ("RVA_C_LINE_ANCHOR", 0x3731400, 2)]


def sec(secs, nm):
    for n, va, vsz, rr, rs in secs:
        if n == nm:
            return va, vsz, rr, rs


def search(d, secs, pat, where=(".rdata", ".data")):
    out = []
    for w in where:
        s = sec(secs, w)
        if not s:
            continue
        va, vsz, rr, rs = s
        for m in re.finditer(re.escape(pat), d[rr:rr + rs]):
            out.append((w, va + m.start()))
    return out


base_hits = None
for nm, r, n in CONSTS:
    o = roff(B.SO, r)
    pat = B.DO[o:o + n * 4]
    vals = struct.unpack("<" + "f" * n, pat)
    hits = search(B.DN, B.SN, pat)
    print(f"  {nm:18s} 0.5.2 0x{r:<9x} {list(vals)} → 0.5.3 후보 {len(hits)}개 "
          + ", ".join(f"{w}@0x{a:x}" for w, a in hits[:6]))

print("\n  ▶ 블록 통째 검색(0x3731380..0x3731410 = 0x90B) — 상대배치 보존 확인")
o = roff(B.SO, 0x3731380)
blk = B.DO[o:o + 0x90]
hits = search(B.DN, B.SN, blk)
print(f"     0.5.2 블록 0x90B → 0.5.3 {len(hits)}곳: " + ", ".join(f"{w}@0x{a:x}" for w, a in hits[:4]))

print("\n" + "=" * 96)
print("③ mid-func — 컨테이너 안 명령 시그 재탐색")
print("=" * 96)
MID = [("RVA_I_SNAP_H", 0x124e2ba, 0x124db10), ("RVA_D_SNAP_W", 0x124e2c2, 0x124db10),
       ("RVA_D_CUT_LO", 0x1201e19, 0x1201d90), ("RVA_D_CUT_HI", 0x1201e27, 0x1201d90),
       ("RVA_D_ZIG_X1", 0x124e8cf, 0x124db10), ("RVA_D_ZIG_X2", 0x124efa1, 0x124db10)]
import dov_053b as G
A = G.A
for nm, site, cont in MID:
    o = roff(B.SO, site)
    raw = B.DO[o:o + 12]
    # 명령 1개 디코드
    ins = next(md.disasm(B.DO[o:o + 16], site), None)
    txt = f"{ins.mnemonic} {ins.op_str}" if ins else "?"
    sz = ins.size if ins else 8
    sig = B.DO[o:o + sz]
    # rip-rel 이면 disp 제외한 앞부분만 시그로
    ripd = None
    if ins and "rip" in ins.op_str:
        rel = struct.unpack_from("<i", B.DO, o + sz - 4)[0] if sz >= 5 else 0
        ripd = site + sz + rel
        sig = B.DO[o:o + sz - 4]
    nc = A.get(cont)
    print(f"  {nm:14s} 0x{site:x} [{txt}] 시그={sig.hex(' ')} "
          + (f"→rip타겟 0x{ripd:x}" if ripd else ""))
    print(f"      컨테이너 0.5.2 0x{cont:x} → 앵커 {'0x%x' % nc if nc else '없음'}", end="")
    if nc:
        f = owner(B.FN, nc)
        no = roff(B.SN, f[0])
        blob = B.DN[no:no + (f[1] - f[0])]
        hits = [m.start() for m in re.finditer(re.escape(sig), blob)]
        print(f" (크기 {f[1]-f[0]}) → 시그 {len(hits)}건: "
              + ", ".join(f"0x{f[0]+h:x}" for h in hits[:6]))
    else:
        print()

print("\n" + "=" * 96)
print("④ RVA_SLOTS — 0.5.3 .rdata 의 16B 0 패딩 후보")
print("=" * 96)
o = roff(B.SO, 0x3fd2b00)
print(f"  0.5.2 슬롯 0x3fd2b00 주변 32B: {B.DO[o-16:o+16].hex(' ')}")
va, vsz, rr, rs = sec(B.SN, ".rdata")
blob = B.DN[rr:rr + rs]
cands = []
for m in re.finditer(rb'(?:\x00{32})', blob):
    a = va + m.start()
    if a % 16 == 0:
        cands.append(a)
print(f"  0.5.3 .rdata(0x{va:x}~0x{va+rs:x}) 32B 이상 0블록 정렬후보 {len(cands)}개, 예: "
      + ", ".join(f"0x{a:x}" for a in cands[:5]) + (f" … 끝 0x{cands[-1]:x}" if cands else ""))
