# -*- coding: utf-8 -*-
# illust_055.py — banpick_illust 0.5.4→0.5.5 잔여 재핀 (geom .rdata 6 + mid-func 6 + SLOTS + 훅 프롤로그)
#   함수 16건 시드 = MIGRATION.md §7.5 (version-migrator 확정)
import re, struct
import bp055 as B
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
md = Cs(CS_ARCH_X86, CS_MODE_64); md.detail = True
roff, sec, owner = B.roff, B.sec, B.owner

# 0.5.4 확정값 (RE\2026-08-05) / 0.5.5 함수 시드 (§7.5)
ILLUST_GET_O, ILLUST_GET_N = 0x1ffd970, 0x1edf1f0
CARD_DRAW_O,  CARD_DRAW_N  = 0x1da8410, 0x1966dd0
FX_SET_N = 0x1951330
CONT_A_O, CONT_B_O = 0x1e16c90, 0x1db2370
BLOCK_O = 0x338b480          # geom 상수 블록 시작 (CARD_RECT)
ZIG_TGT_O = 0x338b510        # -180.0 (블록+0x90)
SLOTS_O = 0x3fe2000

CONSTS = [("RVA_C_CARD_RECT",  0x338b480, 4), ("RVA_C_SNAP_RECT",  0x338b4b0, 4),
          ("RVA_C_NORMAL",     0x338b4c0, 2), ("RVA_C_LINE_DIR",   0x338b4e0, 2),
          ("RVA_C_LINE_START", 0x338b4f0, 2), ("RVA_C_LINE_ANCHOR",0x338b500, 2)]

def search(d, secs, pat, where=(".rdata", ".data")):
    out = []
    for w in where:
        s = sec(secs, w)
        if not s: continue
        va, vsz, rr, rs = s
        for m in re.finditer(re.escape(pat), d[rr:rr + rs]):
            out.append((w, va + m.start()))
    return out

def f32_at(d, secs, rva):
    o = roff(secs, rva)
    return struct.unpack_from("<f", d, o)[0] if o is not None else None

def calls_in(d, secs, fns, fn):
    """함수 fn=(start,end) 안의 e8 직접호출 (site, target) — target 이 pdata 함수시작인 것만"""
    starts = calls_in._starts.get(id(fns))
    if starts is None:
        starts = set(s for s, e in fns)
        calls_in._starts[id(fns)] = starts
    s, e = fn
    o = roff(secs, s)
    blob = d[o:o + (e - s)]
    out = []
    for m in re.finditer(b"\xe8", blob):
        i = m.start()
        if i + 5 > len(blob): break
        disp = struct.unpack_from("<i", blob, i + 1)[0]
        tgt = s + i + 5 + disp
        if tgt in starts:
            out.append((s + i, tgt))
    return out
calls_in._starts = {}

def callers_of(d, secs, fns, target):
    """전 .text e8 스캔으로 target 콜러 (site, owner_fn) 수집"""
    va, vsz, rr, rs = sec(secs, ".text")
    blob = d[rr:rr + rs]
    out = []
    for m in re.finditer(b"\xe8", blob):
        i = m.start()
        if i + 5 > len(blob): break
        disp = struct.unpack_from("<i", blob, i + 1)[0]
        if va + i + 5 + disp == target:
            f = owner(fns, va + i)
            if f: out.append((va + i, f))
    return out

def dis_fn(d, secs, fns, start):
    f = owner(fns, start); o = roff(secs, f[0])
    return list(md.disasm(d[o:o + (f[1] - f[0])], f[0])), f

def rip_target(i):
    for op in i.operands:
        if op.type == 3 and op.mem.base == 41:
            return i.address + i.size + op.mem.disp
    return None

# ══════════════════════════════════════════════════════════════════
print("=" * 100); print("① geom .rdata float 상수 — 0.5.4 바이트를 0.5.5 에서 검색"); print("=" * 100)
deltas = {}; cand = {}
for nm, r, n in CONSTS:
    o = roff(B.SO, r)
    pat = B.DO[o:o + n * 4]
    vals = struct.unpack("<" + "f" * n, pat)
    hits = search(B.DN, B.SN, pat)
    cand[nm] = hits
    print(f"  {nm:18s} 0.5.4 0x{r:<9x} {[round(v,4) for v in vals]} → 0.5.5 후보 {len(hits)}개: "
          + ", ".join(f"{w}@0x{a:x}" for w, a in hits[:8]))
    if len(hits) == 1:
        deltas[nm] = hits[0][1] - r
print("  단일매치 델타들:", sorted(set(deltas.values())), "→", {n: hex(d) for n, d in deltas.items()})

# ══════════════════════════════════════════════════════════════════
print("\n" + "=" * 100); print("② 컨테이너 재핀 — ILLUST_GET 콜러 프로필 대응"); print("=" * 100)
for tag, d_, s_, f_, tgt in (("0.5.4", B.DO, B.SO, B.FO, ILLUST_GET_O),
                             ("0.5.5", B.DN, B.SN, B.FN, ILLUST_GET_N)):
    cs = callers_of(d_, s_, f_, tgt)
    seen = {}
    for site, f in cs:
        seen.setdefault(f, []).append(site)
    print(f"  [{tag}] ILLUST_GET 0x{tgt:x} 콜러 {len(seen)}개:")
    for f, sites in sorted(seen.items()):
        cl = calls_in(d_, s_, f_, f)
        o = roff(s_, f[0])
        body = d_[o:o + (f[1] - f[0])]
        has480 = b"\xc7\x44\x24\x20\x00\x00\xf0\x43" in body
        print(f"     fn 0x{f[0]:x}..0x{f[1]:x} size {f[1]-f[0]:5d}  콜리 {len(set(t for _, t in cl)):2d} 총콜 {len(cl):2d}"
              f"  480imm={'Y' if has480 else 'n'}  콜사이트 {[hex(x) for x in sites]}")

# 컨테이너 A 후보 = 0.5.5 ILLUST_GET 콜러 중 480.0 imm 보유 함수
cs_n = callers_of(B.DN, B.SN, B.FN, ILLUST_GET_N)
contA_n = None
for site, f in cs_n:
    o = roff(B.SN, f[0])
    if b"\xc7\x44\x24\x20\x00\x00\xf0\x43" in B.DN[o:o + (f[1] - f[0])]:
        contA_n = f; break
if contA_n is None:
    print("  !! 컨테이너 A 미발견 — 중단"); raise SystemExit(1)
print(f"\n  ▶ 컨테이너 A(0.5.5) = 0x{contA_n[0]:x} (size {contA_n[1]-contA_n[0]})  [0.5.4 = 0x{CONT_A_O:x}]")

# 컨테이너 B = A 콜리 중 -70/+70 movss rip 참조 보유
contB_n = None
for t in sorted(set(t for _, t in calls_in(B.DN, B.SN, B.FN, contA_n))):
    f = owner(B.FN, t)
    if not f or f[0] != t: continue
    ins, _ = dis_fn(B.DN, B.SN, B.FN, t)
    vals = set()
    for i in ins:
        rt = rip_target(i)
        if rt is not None and i.mnemonic == "movss":
            v = f32_at(B.DN, B.SN, rt)
            if v is not None: vals.add(round(v, 2))
    if -70.0 in vals and 70.0 in vals:
        contB_n = f
        print(f"  ▶ 컨테이너 B(0.5.5) = 0x{t:x} (size {f[1]-f[0]}) — movss ±70.0 보유  [0.5.4 = 0x{CONT_B_O:x}]")
        bc = callers_of(B.DN, B.SN, B.FN, t)
        print(f"     교차: B 콜러 = {sorted(set(hex(f2[0]) for _, f2 in bc))} (0.5.4 = 콜러 1 = A)")
        break
if contB_n is None:
    print("  !! 컨테이너 B 미발견 — 중단"); raise SystemExit(1)

# ══════════════════════════════════════════════════════════════════
print("\n" + "=" * 100); print("③ mid-func 필드 재핀 — 명령정렬 (0.5.4 원본 명령 → 0.5.5 컨테이너 검색)"); print("=" * 100)
CONT = {CONT_A_O: contA_n[0], CONT_B_O: contB_n[0]}
CASES = [("RVA_I_SNAP_H", 0x1e17510, CONT_A_O, "imm"),
         ("RVA_D_SNAP_W", 0x1e17526, CONT_A_O, "disp"),
         ("RVA_D_CUT_LO", 0x1db23f8, CONT_B_O, "disp"),
         ("RVA_D_CUT_HI", 0x1db2406, CONT_B_O, "disp"),
         ("RVA_D_ZIG_X1", 0x1e18372, CONT_A_O, "disp"),
         ("RVA_D_ZIG_X2", 0x1e18a50, CONT_A_O, "disp")]
OLD = {}; cache = {}
for nm, site, cont, kind in CASES:
    if cont not in cache: cache[cont] = dis_fn(B.DO, B.SO, B.FO, cont)
    ins, f = cache[cont]
    hit = next((i for i in ins if i.address <= site < i.address + i.size), None)
    if not hit: print(f"  {nm:14s} 0.5.4 명령 복원 실패"); continue
    off = site - hit.address
    tgt = rip_target(hit)
    val = f32_at(B.DO, B.SO, tgt) if tgt else None
    if kind == "imm": val = struct.unpack_from("<f", B.DO, roff(B.SO, site))[0]
    OLD[nm] = dict(off=off, val=val, mnem=hit.mnemonic, bytes=bytes(hit.bytes),
                   size=hit.size, rel=hit.address - f[0])
    print(f"  {nm:14s} 0.5.4 명령 0x{hit.address:x}(+{hit.address-f[0]:#x}) [{hit.mnemonic}] 값={val}"
          + (f" rip타겟 0x{tgt:x}" if tgt else ""))

print()
ncache = {}; RESULT = {}
for nm, site, cont, kind in CASES:
    if nm not in OLD: continue
    o = OLD[nm]; nc = CONT[cont]
    if nc not in ncache: ncache[nc] = dis_fn(B.DN, B.SN, B.FN, nc)
    ins, f = ncache[nc]
    hits = []
    for i in ins:
        if i.mnemonic != o["mnem"]: continue
        if kind == "imm":
            if bytes(i.bytes) == o["bytes"]:
                hits.append((i.address, i.address + o["off"], None, i.address - f[0]))
        else:
            t = rip_target(i)
            if t is None: continue
            v = f32_at(B.DN, B.SN, t)
            if v is not None and o["val"] is not None and abs(v - o["val"]) < 0.001 and i.size == o["size"]:
                hits.append((i.address, i.address + i.size - 4, t, i.address - f[0]))
    print(f"  {nm:14s} 컨테이너 0x{nc:x}(크기 {f[1]-f[0]}) 후보 {len(hits)}건  [OLD 상대 +{o['rel']:#x}]")
    for a, fld, t, rel in hits[:8]:
        print(f"       명령 0x{a:<9x}(+{rel:#x}, Δrel={rel-o['rel']:+d}) → 필드 **0x{fld:x}**"
              + (f"  타겟 0x{t:x}={f32_at(B.DN,B.SN,t)}" if t else ""))
    RESULT[nm] = hits

# ══════════════════════════════════════════════════════════════════
print("\n" + "=" * 100); print("④ RVA_SLOTS — 0.5.5 .rdata 최장 0런 + 4K 정렬"); print("=" * 100)
va, vsz, rr, rs = sec(B.SN, ".rdata")
blob = B.DN[rr:rr + rs]
runs = sorted(((m.end() - m.start(), va + m.start(), va + m.end())
               for m in re.finditer(rb"\x00{4096,}", blob)), reverse=True)
print(f"  0.5.5 .rdata(0x{va:x}~0x{va+rs:x}) 4096B+ 0런 상위:")
for L, a, e in runs[:5]:
    print(f"     0x{a:x}..0x{e:x}  {L}B")
if runs:
    L, a, e = runs[0]
    c = (a + 0xfff) & ~0xfff
    if c + 16 <= e:
        o = roff(B.SN, c)
        around = B.DN[o - 64:o + 80]
        print(f"  ▶ 권장 SLOTS = 0x{c:x} (최장런 내부 4K 정렬), 주변±64B 전부0 = {all(b==0 for b in around)}")

# ══════════════════════════════════════════════════════════════════
print("\n" + "=" * 100); print("⑤ 훅 3종 프롤로그 실측 (0.5.5 시드 주소)"); print("=" * 100)
for nm, rva, want_len, want in (("FX_SET", FX_SET_N, 12, bytes([0x55,0x41,0x57,0x41,0x56,0x41,0x55,0x41,0x54,0x56,0x57,0x53])),
                                ("CARD_DRAW", CARD_DRAW_N, 12, bytes([0x55,0x41,0x57,0x41,0x56,0x41,0x55,0x41,0x54,0x56,0x57,0x53])),
                                ("ILLUST_GET", ILLUST_GET_N, 13, bytes([0x55,0x41,0x56,0x56,0x57,0x53,0x48,0x81,0xEC,0x80,0x00,0x00,0x00]))):
    o = roff(B.SN, rva)
    head = B.DN[o:o + 32]
    bounds = []
    pos = 0
    for i in md.disasm(head, rva):
        pos += i.size
        bounds.append(pos)
        if pos >= 28: break
    riprel = any(rip_target(i) is not None for i in md.disasm(head[:want_len], rva))
    ok = head[:len(want)] == want and want_len in bounds
    print(f"  {nm:11s} 0x{rva:x} 진입 20B = {head[:20].hex(' ')}")
    print(f"              경계={bounds}  PROLOGUE일치={'OK' if head[:len(want)]==want else '**불일치**'}"
          f"  ORIG_LEN {want_len} 경계상={'OK' if want_len in bounds else '**NG**'}  진입부 rip-rel={'있음!!' if riprel else '없음'}")

# ══════════════════════════════════════════════════════════════════
print("\n" + "=" * 100); print("⑥ ZIG rip 타겟 교차검증"); print("=" * 100)
zt = set()
for nm in ("RVA_D_ZIG_X1", "RVA_D_ZIG_X2"):
    for a, fld, t, rel in RESULT.get(nm, []):
        if t: zt.add(t)
print(f"  0.5.4 ZIG 타겟 0x{ZIG_TGT_O:x} → 0.5.5 ZIG 타겟들 {[hex(t) for t in sorted(zt)]}")
for t in sorted(zt):
    print(f"     델타 {t - ZIG_TGT_O:+#x}  (블록 델타와 일치해야 함)")
