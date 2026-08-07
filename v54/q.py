# -*- coding: utf-8 -*-
"""조회 도구 — 소스 파일명으로 두 판의 함수를 찾는다.

  python q.py find action_score          # 소스명에 'action_score' 든 함수 (양 판)
  python q.py dis 054 c7b730 200         # 디스어셈 (RVA, 길이)
  python q.py fn  054 c7b730             # 그 RVA 가 속한 함수 범위 + 소스
  python q.py imm 054 c7b730 2000 150000 # 함수 구간에서 특정 상수를 쓰는 명령 찾기
  python q.py jt  054 c559e0 3000        # 구간의 점프테이블 후보(lea+jmp reg)

⚠원칙: **0.5.3 RVA 를 0.5.4 에 그대로 대입하지 말 것.** 소스명·구조로 먼저 찾고,
   그 다음에 0.5.3 과 대조한다. (상수만 맞춰 짝지으면 "다른 자리"를 잡는다.)
"""
import io, os, sys

sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
sys.path.insert(0, r'C:\tfm2mods\v54')
from pe2 import load, BASE

D = r'C:\tfm2mods\v54'


def srcmap(ver):
    rows = []
    p = os.path.join(D, '%s_srcmap.tsv' % ver)
    for ln in io.open(p, encoding='utf-8'):
        s, e, src, lines = ln.rstrip('\n').split('\t')
        rows.append((int(s, 16), int(e, 16), src, lines))
    return rows


def cmd_find(pat):
    for ver in ('053', '054'):
        rows = [r for r in srcmap(ver) if pat.lower() in r[2].lower()]
        print('=== %s : %d개 ===' % (ver, len(rows)))
        for s, e, src, lines in sorted(rows, key=lambda r: -(r[1] - r[0])):
            print('  %06x-%06x  %6dB  %s   [줄 %s]' % (s, e, e - s, src[:70], lines[:40]))


def cmd_fn(ver, rva):
    e = load(ver)
    f = e.func_of(rva)
    print('함수 범위:', ('%06x-%06x (%dB)' % (f[0], f[1], f[1] - f[0])) if f else '없음')
    if f:
        for s, en, src, lines in srcmap(ver):
            if s == f[0]:
                print('소스      :', src)
                print('줄        :', lines)


def cmd_dis(ver, rva, n):
    e = load(ver)
    for i in e.dis(rva, n):
        print('%06x  %-24s %s %s' % (i.address - BASE, i.bytes.hex(), i.mnemonic, i.op_str))


def cmd_imm(ver, rva, n, val):
    e = load(ver)
    import capstone
    hit = 0
    for i in e.dis(rva, n):
        for op in i.operands:
            if op.type == capstone.x86.X86_OP_IMM and op.imm == val:
                print('%06x  %-26s %s %s' % (i.address - BASE, i.bytes.hex(), i.mnemonic, i.op_str))
                hit += 1
    print('-- %d건' % hit)


def cmd_jt(ver, rva, n):
    e = load(ver)
    prev = []
    for i in e.dis(rva, n):
        if i.mnemonic == 'jmp' and i.op_str in ('rax', 'rcx', 'rdx', 'rbx', 'rsi', 'rdi',
                                                'r8', 'r9', 'r10', 'r11', 'r12', 'r13', 'r14', 'r15'):
            print('%06x  jmp %s' % (i.address - BASE, i.op_str))
            for p in prev[-6:]:
                print('        %06x  %s %s' % (p.address - BASE, p.mnemonic, p.op_str))
            print()
        prev.append(i)


if __name__ == '__main__':
    a = sys.argv[1:]
    if not a:
        print(__doc__)
    elif a[0] == 'find':
        cmd_find(a[1])
    elif a[0] == 'fn':
        cmd_fn(a[1], int(a[2], 16))
    elif a[0] == 'dis':
        cmd_dis(a[1], int(a[2], 16), int(a[3]))
    elif a[0] == 'imm':
        cmd_imm(a[1], int(a[2], 16), int(a[3]), int(a[4]))
    elif a[0] == 'jt':
        cmd_jt(a[1], int(a[2], 16), int(a[3]))
