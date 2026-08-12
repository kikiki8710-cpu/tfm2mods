# -*- coding: utf-8 -*-
# ct_055b.py — method B: 컨테이너 difflib 정렬로 MISS 사이트 재핀 + BYTEDIFF fixed 재계산.
import sys, io, difflib
import importlib.util
spec = importlib.util.spec_from_file_location("r5", r"C:\tfm2mods\ct_055_repin.py")
R = importlib.util.module_from_spec(spec); spec.loader.exec_module(R)

def norm(i):
    # imm/disp를 정규화한 명령 시그(레지스터·니모닉 유지)
    s = i.mnemonic + " " + i.op_str
    import re
    s = re.sub(r"0x[0-9a-f]+", "#", s)
    return s

def disas_cont(d, secs, fns, cont_start):
    f = R.owner(fns, cont_start)
    o = R.roff(secs, f[0])
    ins = list(R.md.disasm(d[o:o+(f[1]-f[0])], f[0]))
    return ins, f

def align_site(cont4, cont5, site4):
    i4, f4 = disas_cont(R.D4, R.S4, R.F4, cont4)
    i5, f5 = disas_cont(R.D5, R.S5, R.F5, cont5)
    idx = next((k for k,i in enumerate(i4) if i.address <= site4 < i.address+i.size), None)
    if idx is None: return None
    off_in = site4 - i4[idx].address
    a = [norm(i) for i in i4]; b = [norm(i) for i in i5]
    sm = difflib.SequenceMatcher(None, a, b, autojunk=False)
    # site idx가 속한 equal 블록을 찾아 대응 j 계산
    for tag, i1, i2, j1, j2 in sm.get_opcodes():
        if i1 <= idx < i2:
            if tag == "equal":
                j = j1 + (idx - i1)
                ni = i5[j]
                return dict(new=ni.address+off_in, insn=f"{ni.mnemonic} {ni.op_str}",
                            insn4=f"{i4[idx].mnemonic} {i4[idx].op_str}", tag=tag,
                            ratio=sm.ratio(), nbytes=R.D5[R.roff(R.S5,ni.address):R.roff(R.S5,ni.address)+ni.size].hex())
            else:
                # 근접 equal 경계로 근사
                return dict(new=None, tag=tag, ratio=sm.ratio(),
                            insn4=f"{i4[idx].mnemonic} {i4[idx].op_str}")
    return None

# MISS 사이트 (0.5.4 cont -> 0.5.5 cont 는 method A 로 확정한 값)
JOBS = [
    ("allow_dup_players", 0x2310a90, 0x1a95570, 0x2311131),
    ("server_dedup",      0x20d5bf0, 0x216e870, 0x20e42d1),
    ("run_push_gate",     0x231de30, 0x1aa2930, 0x231e838),
]
if __name__ == "__main__":
 print("="*80); print("method B (difflib 컨테이너 정렬)"); print("="*80)
 for name, c4, c5, s4 in JOBS:
    r = align_site(c4, c5, s4)
    if r and r.get("new"):
        print(f"{name:20s} 0x{s4:x} -> 0x{r['new']:x} [{r['tag']} ratio={r['ratio']:.3f}] "
              f"bytes={r['nbytes']}  ({r['insn4']} | {r['insn']})")
    else:
        print(f"{name:20s} 0x{s4:x} -> FAIL {r}")

 # ── CSEND/PAGE_IMM 탐색(이미 완료: CSEND=0x1a913a0, PAGE_IMM=0x1a91bcc) ──
 patt = bytes.fromhex("41c6860c24000005")
 hits5 = [R.T5[0] + i for i in range(len(R.BLOB5)) if R.BLOB5[i:i+8] == patt]
 print(f"  PAGE_IMM 0.5.5={[hex(h) for h in hits5]}")
