# -*- coding: utf-8 -*-
"""0.5.2 ↔ 0.5.3 함수 한 줄씩 대조 (니모닉 시퀀스 diff)."""
import sys, re, difflib
from _it_scan import O, N, BASE

GPR = (r'\b(?:r(?:ax|bx|cx|dx|si|di|8|9|1[0-5])'
       r'|e(?:ax|bx|cx|dx|si|di)|r(?:8|9|1[0-5])d'
       r'|[abcd][lh]|sil|dil|r(?:8|9|1[0-5])[bw])\b')
def norm(mn, op, regs=True):
    """주소/오프셋 차이를 지우고 '형태'만 남긴다.
       regs=True면 범용 레지스터명까지 R로 정규화(레지스터 재배정 노이즈 제거).
       ⚠rsp/rbp는 남긴다 — 스택 슬롯 구조는 의미가 있다."""
    s = op
    s = re.sub(r'0x[0-9a-f]{6,}', 'ADDR', s)          # 절대주소·큰 상수
    s = re.sub(r'\[rip \+ [^\]]+\]', '[rip+X]', s)
    if regs:
        s = re.sub(GPR, 'R', s)
        s = re.sub(r'\[rsp \+ 0x[0-9a-f]+\]', '[rsp+X]', s)
    return f"{mn} {s}".strip()

def dump(E, start, end):
    out = []
    b = E.read(start, end - start)
    for ins in E.md.disasm(b, BASE + start):
        out.append((ins.address - BASE, norm(ins.mnemonic, ins.op_str), ins.mnemonic + ' ' + ins.op_str))
    return out

def cmp_fn(name, o_rva, n_rva, ctx=2):
    fo = O.func_of(o_rva); fn_ = N.func_of(n_rva)
    a = dump(O, *fo); b = dump(N, *fn_)
    print(f"\n{'='*78}\n■ {name}   0.5.2 {hex(fo[0])}(size {hex(fo[1]-fo[0])})  ↔  0.5.3 {hex(fn_[0])}(size {hex(fn_[1]-fn_[0])})")
    print(f"  명령수 {len(a)} → {len(b)}")
    sm = difflib.SequenceMatcher(None, [x[1] for x in a], [x[1] for x in b], autojunk=False)
    ratio = sm.ratio()
    print(f"  시퀀스 유사도 {ratio:.4f}")
    nblk = 0
    for tag, i1, i2, j1, j2 in sm.get_opcodes():
        if tag == 'equal':
            continue
        nblk += 1
        print(f"\n  --- [{tag}] 0.5.2 +{i1}..{i2} / 0.5.3 +{j1}..{j2} ---")
        for k in range(max(0, i1 - ctx), i1):
            print(f"      = {a[k][0]:#010x}  {a[k][2]}")
        for k in range(i1, i2):
            print(f"    - 0.5.2 {a[k][0]:#010x}  {a[k][2]}")
        for k in range(j1, j2):
            print(f"    + 0.5.3 {b[k][0]:#010x}  {b[k][2]}")
        for k in range(i2, min(len(a), i2 + ctx)):
            print(f"      = {a[k][0]:#010x}  {a[k][2]}")
    if nblk == 0:
        print("  → 완전 동일(정규화 기준)")

if __name__ == '__main__':
    targets = [
        ("buy(구매 진입)",       0x211e070, 0xd0c680),
        ("resolver(빌드 해석)",  0x211e150, 0xd0c770),
        ("next-target(다음 목표)", 0x211d800, 0xd0be50),
        ("helper(owned↔build)",  0x211d900, 0xd0bf50),
    ]
    which = sys.argv[1] if len(sys.argv) > 1 else 'all'
    for nm, o, n in targets:
        if which != 'all' and which not in nm:
            continue
        cmp_fn(nm, o, n)
