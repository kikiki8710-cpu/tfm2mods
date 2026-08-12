# -*- coding: utf-8 -*-
# ct_055d.py — 함수시작 재핀(내부 다중앵커 투표) + DELAY 컨테이너 mid 사이트.
import importlib.util, collections
spec = importlib.util.spec_from_file_location("r5", r"C:\tfm2mods\ct_055_repin.py")
R = importlib.util.module_from_spec(spec); spec.loader.exec_module(R)
import importlib.util as u2
sb = u2.spec_from_file_location("b5", r"C:\tfm2mods\ct_055b.py")
B = u2.module_from_spec(sb); sb.loader.exec_module(B)

def read5(rva,n):
    o=R.roff(R.S5,rva); return R.D5[o:o+n] if o is not None else None
def read4(rva,n):
    o=R.roff(R.S4,rva); return R.D4[o:o+n] if o is not None else None

def find_fn_start(old_fn, kwin=3):
    """0.5.4 함수 내부 명령들을 앵커로 0.5.5 전역스캔 → 히트 owner 투표 = 새 함수시작."""
    ins, f = R.disfn(R.D4, R.S4, R.F4, old_fn)
    if ins is None: return None
    votes = collections.Counter()
    for idx in range(len(ins)):
        sig, mask, so = R.build_sig(R.D4, R.S4, ins, idx, kwin, kwin)
        h5 = R.scan(R.BLOB5, R.T5[0], sig, mask)
        h4 = R.scan(R.BLOB4, R.T4[0], sig, mask)
        if len(h5)==1 and len(h4)==1:
            site5 = h5[0] + so + (ins[idx].address - ins[idx].address)  # sig 시작 정렬
            ow = R.owner(R.F5, h5[0]+so)
            if ow: votes[ow[0]] += 1
    if not votes: return None
    top, n = votes.most_common(1)[0]
    return dict(new_start=top, votes=n, total=sum(votes.values()),
                size4=f[1]-f[0], size5=(R.owner(R.F5,top)[1]-top),
                prol5=read5(top,12).hex())

for name, old in [("ITEMCONV_RVA",0x18429d0),("COLLECT_RVA",0x18f2b50),
                  ("FN_DD_SETOPT_RVA",0x1bfc80),("A15E20_RVA",0xa15e20),
                  ("ARRIVE_FN_RVA",0x2327080),("RUST_ALLOC_RVA",0x28f7df0),
                  ("RPLY_owner",0x2323bb2)]:
    r = find_fn_start(old)
    if r:
        print(f"{name:18s} 0x{old:x} -> 0x{r['new_start']:x}  votes={r['votes']}/{r['total']} "
              f"size {r['size4']}->{r['size5']} prol={r['prol5']}")
    else:
        print(f"{name:18s} 0x{old:x} -> NONE")

# DELAY 컨테이너: 0x2327080 -> ARRIVE_FN find 결과를 cont5로 써서 mid 사이트 difflib
print(); print("DELAY 컨테이너 mid 사이트 (cont5 = ARRIVE_FN 결과):")
af = find_fn_start(0x2327080)
if af:
    c5 = af['new_start']
    for name, s4 in [("DELAY_RVA",0x2327094),("DELAY_RESUME_RVA",0x23270a3),
                     ("EPILOG_RVA",0x232790a)]:
        r = B.align_site(0x2327080, c5, s4)
        if r and r.get("new"):
            print(f"  {name:18s} 0x{s4:x} -> 0x{r['new']:x} [{r['tag']} r={r['ratio']:.3f}] ({r['insn4']} | {r['insn']})")
        else:
            print(f"  {name:18s} 0x{s4:x} -> FAIL {r}")

# RPLY 컨테이너 0x2323aa0 -> RPLY3 새값 0x1aa8370 으로 mid
print(); print("RPLY 컨테이너 mid (cont5=0x1aa8370):")
for name, s4 in [("RPLY_RVA",0x2323bb2),("RPLY_RESUME_RVA",0x2323bc4)]:
    r = B.align_site(0x2323aa0, 0x1aa8370, s4)
    if r and r.get("new"):
        print(f"  {name:18s} 0x{s4:x} -> 0x{r['new']:x} [{r['tag']} r={r['ratio']:.3f}] ({r['insn4']} | {r['insn']})")
    else:
        print(f"  {name:18s} 0x{s4:x} -> FAIL {r}")
