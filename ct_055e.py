# -*- coding: utf-8 -*-
# ct_055e.py — 빠른 bounded 함수시작 재핀 + 프롤로그 확인. 앵커는 최대 40개까지만, 첫 유일 채택.
import importlib.util, sys, collections
spec = importlib.util.spec_from_file_location("r5", r"C:\tfm2mods\ct_055_repin.py")
R = importlib.util.module_from_spec(spec); spec.loader.exec_module(R)
def P(*a): print(*a, flush=True)
def read5(rva,n):
    o=R.roff(R.S5,rva); return R.D5[o:o+n] if o is not None else None
def read4(rva,n):
    o=R.roff(R.S4,rva); return R.D4[o:o+n] if o is not None else None

def find_start(old_fn, kwin=3, cap=60):
    ins, f = R.disfn(R.D4, R.S4, R.F4, old_fn)
    if ins is None: return None
    votes = collections.Counter()
    step = max(1, len(ins)//cap)
    for idx in range(0, len(ins), step):
        sig, mask, so = R.build_sig(R.D4, R.S4, ins, idx, kwin, kwin)
        h5 = R.scan(R.BLOB5, R.T5[0], sig, mask)
        h4 = R.scan(R.BLOB4, R.T4[0], sig, mask)
        if len(h5)==1 and len(h4)==1:
            ow = R.owner(R.F5, h5[0]+so)
            if ow: votes[ow[0]] += 1
    if not votes: return None
    top,n = votes.most_common(1)[0]
    ow=R.owner(R.F5,top)
    return dict(new=top, votes=n, total=sum(votes.values()), size4=f[1]-f[0],
                size5=ow[1]-top, prol=read5(top,12).hex())

for name, old in [("ITEMCONV_RVA",0x18429d0),("COLLECT_RVA",0x18f2b50),
                  ("ARRIVE_FN_RVA",0x2327080),("A15E20_RVA",0xa15e20),
                  ("FN_DD_SETOPT_RVA",0x1bfc80),("RUST_ALLOC_RVA",0x28f7df0)]:
    r=find_start(old)
    if r: P(f"{name:18s} 0x{old:x} -> 0x{r['new']:x} votes={r['votes']}/{r['total']} size {r['size4']}->{r['size5']} prol={r['prol']}")
    else: P(f"{name:18s} 0x{old:x} -> NONE")
