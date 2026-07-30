# -*- coding: utf-8 -*-
# illust_053b.py — geom mid-func 사이트 재핀(정정판).
#   ⚠상수들은 "명령 시작"이 아니라 **필드 위치**다:
#     RVA_I_SNAP_H = mov dword[rsp+0x20], 480.0 의 **imm4 위치**
#     RVA_D_*      = rip-rel **disp4 위치** (disp_loc+4 = 명령 끝, 타겟 = disp_loc+4+disp)
#   ⟹ 컨테이너를 선형 디스어셈해 (a)해당 필드를 품은 명령을 복원하고
#      (b)0.5.3 컨테이너에서 같은 종류·같은 타겟값(float) 명령을 찾아 필드 위치를 재산출.
import re, struct
import bytepatch_053 as B
import dov_053b as G
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
md = Cs(CS_ARCH_X86, CS_MODE_64)
md.detail = True
roff, owner, A = B.roff, B.owner, G.A


def dis_fn(d, secs, fns, start):
    f = owner(fns, start)
    o = roff(secs, f[0])
    return list(md.disasm(d[o:o + (f[1] - f[0])], f[0])), f


def rip_target(i):
    for op in i.operands:
        if op.type == 3 and op.mem.base == 41:      # rip
            return i.address + i.size + op.mem.disp
    return None


def f32_at(d, secs, rva):
    o = roff(secs, rva)
    return struct.unpack_from("<f", d, o)[0] if o is not None else None


CASES = [("RVA_I_SNAP_H", 0x124e2ba, 0x124db10, "imm"),
         ("RVA_D_SNAP_W", 0x124e2c2, 0x124db10, "disp"),
         ("RVA_D_CUT_LO", 0x1201e19, 0x1201d90, "disp"),
         ("RVA_D_CUT_HI", 0x1201e27, 0x1201d90, "disp"),
         ("RVA_D_ZIG_X1", 0x124e8cf, 0x124db10, "disp"),
         ("RVA_D_ZIG_X2", 0x124efa1, 0x124db10, "disp")]

print("=" * 100)
print("0.5.2 원본 명령 복원")
print("=" * 100)
OLD = {}
cache = {}
for nm, site, cont, kind in CASES:
    if cont not in cache:
        cache[cont] = dis_fn(B.DO, B.SO, B.FO, cont)
    ins, f = cache[cont]
    hit = None
    for i in ins:
        if i.address <= site < i.address + i.size:
            hit = i
            break
    if not hit:
        print(f"  {nm:14s} 명령 복원 실패")
        continue
    off = site - hit.address
    tgt = rip_target(hit)
    val = f32_at(B.DO, B.SO, tgt) if tgt else None
    if kind == "imm":
        val = struct.unpack_from("<f", B.DO, roff(B.SO, site))[0]
    OLD[nm] = dict(ins=hit, off=off, val=val, cont=cont, kind=kind,
                   mnem=hit.mnemonic, ops=hit.op_str, bytes=bytes(hit.bytes))
    print(f"  {nm:14s} 0x{hit.address:x} [{hit.mnemonic} {hit.op_str}] 크기={hit.size} "
          f"필드오프셋=+{off} 값={val} 바이트={bytes(hit.bytes).hex(' ')}")

print("\n" + "=" * 100)
print("0.5.3 컨테이너에서 같은 명령 찾기 (니모닉 + 타겟 float 값 일치)")
print("=" * 100)
ncache = {}
RESULT = {}
for nm, site, cont, kind in CASES:
    if nm not in OLD:
        continue
    o = OLD[nm]
    nc = A.get(cont)
    if not nc:
        print(f"  {nm:14s} 컨테이너 앵커 없음")
        continue
    if nc not in ncache:
        ncache[nc] = dis_fn(B.DN, B.SN, B.FN, nc)
    ins, f = ncache[nc]
    hits = []
    for i in ins:
        if i.mnemonic != o["mnem"]:
            continue
        if kind == "imm":
            if bytes(i.bytes) == o["bytes"]:
                hits.append((i.address, i.address + o["off"], None))
        else:
            t = rip_target(i)
            if t is None:
                continue
            v = f32_at(B.DN, B.SN, t)
            if v is not None and o["val"] is not None and abs(v - o["val"]) < 0.001:
                # disp 필드 위치 = 명령끝 - 4
                hits.append((i.address, i.address + i.size - 4, t))
    print(f"  {nm:14s} 컨테이너 0x{nc:x}(크기 {f[1]-f[0]}) 후보 {len(hits)}건")
    for a, fld, t in hits[:6]:
        print(f"       명령 0x{a:<9x} → 필드 0x{fld:x}" + (f"  타겟 0x{t:x}={f32_at(B.DN,B.SN,t)}" if t else ""))
    if len(hits) == 1:
        RESULT[nm] = hits[0][1]
    elif hits:
        RESULT[nm] = [h[1] for h in hits]

print("\n" + "=" * 100)
print("결과 요약")
print("=" * 100)
for nm, site, cont, kind in CASES:
    r = RESULT.get(nm)
    if isinstance(r, int):
        print(f"  {nm:14s} 0x{site:x} → **0x{r:x}**  (유일)")
    elif r:
        print(f"  {nm:14s} 0x{site:x} → 다중 {[hex(x) for x in r]}")
    else:
        print(f"  {nm:14s} 0x{site:x} → 미해결")
