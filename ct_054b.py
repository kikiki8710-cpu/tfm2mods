# -*- coding: utf-8 -*-
# ct_054b.py — 컨테이너 함수 대응 도출(내부 마스크시그 투표) + 컨테이너 내 명령정렬 재핀.
import sys, io, re, difflib, collections
import ct_054 as C

def norm(i):
    s = re.sub(r'0x[0-9a-f]+', 'I', i.op_str)
    return f"{i.mnemonic} {s}"

def normkeep(i):
    # imm 유지, disp 만 I (분기 타깃은 I)
    if i.mnemonic.startswith("j") or i.mnemonic=="call":
        return f"{i.mnemonic} T"
    return f"{i.mnemonic} {i.op_str}"

def cont_vote(old_cont, nsig=40, win=6):
    """0.5.3 컨테이너 내부에서 마스크시그 다수를 뽑아 0.5.4 전역 스캔 → 소유 함수 투표."""
    ins, f = C.disfn(C.D3, C.S3, C.F3, old_cont)
    votes = collections.Counter()
    tried = 0
    step = max(1, len(ins)//nsig)
    for idx in range(win, len(ins)-win, step):
        sig, mask, so = C.build_sig(C.D3, C.S3, ins, idx, win, win)
        h3 = C.scan(C.BLOB3, C.T3[0], sig, mask)
        if len(h3) != 1: continue
        h4 = C.scan(C.BLOB4, C.T4[0], sig, mask)
        tried += 1
        if len(h4) != 1: continue
        o = C.owner(C.F4, h4[0])
        if o: votes[o] += 1
    return votes, len(ins), tried

def align(old_cont, new_cont):
    i3,_ = C.disfn(C.D3, C.S3, C.F3, old_cont)
    i4,_ = C.disfn(C.D4, C.S4, C.F4, new_cont)
    a = [norm(x) for x in i3]; b = [norm(x) for x in i4]
    sm = difflib.SequenceMatcher(None, a, b, autojunk=False)
    m = {}
    for bl in sm.get_matching_blocks():
        for k in range(bl.size):
            m[bl.a+k] = bl.b+k
    return i3, i4, m, sm.ratio()
