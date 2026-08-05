# -*- coding: utf-8 -*-
# illust_054e.py — 훅 3종 프롤로그 실측 + ORIG_LEN(12B 이상 최소 명령경계) + rip-rel/chkstk 점검
import bp054 as B
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
md = Cs(CS_ARCH_X86, CS_MODE_64); md.detail = True

HOOKS = [("RVA_FX_SET", 0x1bd8e50, 0x1d92980, 12),
         ("RVA_CARD_DRAW", 0x1bee8e0, 0x1da8410, 12),
         ("RVA_ILLUST_GET", 0x1e91400, 0x1ffd970, 13)]

for nm, old, new, olen in HOOKS:
    print("="*96)
    print(nm)
    for tag, d, secs, r in (("0.5.3", B.DO, B.SO, old), ("0.5.4", B.DN, B.SN, new)):
        o = B.roff(secs, r)
        blob = d[o:o+40]
        ins = list(md.disasm(blob, r))
        acc, bound, txt, riprel = 0, [], [], []
        for i in ins:
            acc += i.size; bound.append(acc)
            txt.append(f"{i.mnemonic} {i.op_str}")
            for op in i.operands:
                if op.type == 3 and op.mem.base == 41:
                    riprel.append((acc-i.size, f"{i.mnemonic} {i.op_str}"))
            if acc >= 32: break
        need = next((b for b in bound if b >= 12), None)
        print(f"  {tag} 0x{r:x}  bytes[0:20]={blob[:20].hex(' ')}")
        print(f"      경계 {bound[:10]}  → 12B 이상 최소경계 = **{need}**"
              + (f"   (현 소스 ORIG_LEN={olen})" if tag=="0.5.3" else ""))
        print(f"      명령: " + " ; ".join(txt[:8]))
        pre = blob[:need]
        print(f"      PROLOGUE[{need}] = " + ", ".join(f"0x{b:02X}" for b in pre))
        rr = [x for x in riprel if x[0] < need]
        print(f"      rip-rel(프롤로그 내) = {rr if rr else '없음'}")
    print()
