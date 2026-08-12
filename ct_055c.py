# -*- coding: utf-8 -*-
# ct_055c.py — 나머지 함수시작/mid RVA 재핀(method A) + 훅 프롤로그 검증 + MULTI 판별.
import sys, importlib.util
spec = importlib.util.spec_from_file_location("r5", r"C:\tfm2mods\ct_055_repin.py")
R = importlib.util.module_from_spec(spec); spec.loader.exec_module(R)

HOOK_PROL = bytes.fromhex("554157415641554154565753")       # push8 12B
def read5(rva, n):
    o = R.roff(R.S5, rva); return R.D5[o:o+n] if o is not None else None
def read4(rva, n):
    o = R.roff(R.S4, rva); return R.D4[o:o+n] if o is not None else None
def prol12(rva):
    b = read5(rva, 12); return b.hex() if b else "??"

# 함수시작 훅: 0.5.4 entry n바이트를 orig 로 주고 repin (entry가 clone이면 inner 로 대체)
FN = [
    ("RUN_RVA",      0x231de30, "hook12"),
    ("CGATE_RVA",    0x2310a90, "hook12"),
    ("CSEND_RVA",    0x230c910, "hook12"),
    ("REFRESH_RVA",  0x2306000, "call"),
    ("ITEMCONV_RVA", 0x18429d0, "hook12"),
    ("COLLECT_RVA",  0x18f2b50, "hook12"),
    ("RPLY3_RVA",    0x2323aa0, "hook12"),
    ("ARRIVE_FN_RVA",0x2327080, "hook_n/call"),
    ("A15E20_RVA",   0xa15e20,  "call"),
    ("RUST_ALLOC_RVA",0x28f7df0,"call"),
    ("FN_DD_SETOPT_RVA",0x1bfc80,"ui drop"),
]
print("="*90); print("함수시작 재핀 (method A, entry 12B sig) + 프롤로그"); print("="*90)
derived = {"RUN_RVA":0x1aa2930,"CGATE_RVA":0x1a95570,"CSEND_RVA":0x1a913a0,"REFRESH_RVA":0x1a8aa10}
for name, rva, kind in FN:
    e4 = read4(rva, 8)
    r = R.repin(rva, e4, name, kmin=1, kmax=14)
    st = r["status"]
    if st in ("OK","OK_BYTEDIFF"):
        new = r["new"]
        print(f"{name:18s} 0x{rva:x} -> 0x{new:x} [{st}] k={r['k']} prol={prol12(new)}  <{kind}>")
    else:
        d = derived.get(name)
        dtxt = f" derived=0x{d:x} prol={prol12(d)}" if d else ""
        print(f"{name:18s} 0x{rva:x} -> [{st}] {r.get('hits','')}{dtxt}  <{kind}>")

# derived 확정값 프롤로그 재확인
print(); print("derived 확정값 프롤로그:")
for n,a in derived.items():
    b=read5(a,12); print(f"  {n:14s} 0x{a:x} prol={b.hex() if b else '??'} (HOOK match={b==HOOK_PROL if b else False})")

# DELAY 컨테이너(ARRIVE_FN 0x2327080) 확정 시 DELAY/RESUME/EPILOG 도출
print(); print("DELAY/ARRIVE 컨테이너 mid 사이트 (owner-delta 재시도, cont5 미정 시 skip):")

# MULTI 판별: CTX_DROP(0x22df620), CLONE_CHAMP(0x193d560), DROP_CHAMP(0x182bf30)
print(); print("MULTI 후보 판별 (0.5.4 entry 32B vs 후보 entry 32B 해밍):")
def hamming(a,b): return sum(1 for x,y in zip(a,b) if x!=y)
MULTI = [
    ("CTX_DROP_RVA", 0x22df620, [2232192, 8177648, 27547856, 29193184]),
    ("CLONE_CHAMP_RVA", 0x193d560, [2589600, 8411888, 10602688, 12795728, 28952160]),
    ("DROP_CHAMP_RVA", 0x182bf30, [2228304, 3229440, 16862864, 26247520, 42920384]),
]
for name, old, cands in MULTI:
    e4 = read4(old, 40)
    best=None
    for c in cands:
        e5 = read5(c, 40)
        if e5 is None: continue
        h = hamming(e4, e5)
        if best is None or h < best[1]: best=(c,h)
        print(f"  {name:16s} cand 0x{c:x} ham={h}")
    print(f"    -> {name} 최소해밍 0x{best[0]:x} (ham={best[1]})")

# RUNNER_VT_RVA: .rdata vtable — 0.5.4 값 0x33b91f8 이 가리키는 것 주변으로는 못 감.
# comp_test 러너 타입태그. 재핀법 = ghidra 필요 표기.
print(); print("RUNNER_VT_RVA 0x33b91f8 = .rdata vtable (정적 재핀 난이도 높음 → 별도)")
