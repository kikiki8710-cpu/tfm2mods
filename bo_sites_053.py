# -*- coding: utf-8 -*-
"""tfm2_banpick_order 0.5.3 마이그 — 인라인 phase 디스패처 복제본 전수 분석.

0.5.2에서는 phase_from(B)를 '호출'하던 자리가 0.5.3에서는 대부분 인라인 복제됐다.
각 복제본의 (진입, rule/ban/total 소스, 출력 레지스터, 합류주소, 원본 클로버 집합)을
capstone detail 로 뽑아 모드가 byte-patch 할 수 있는 메타데이터를 만든다.
"""
import sys, json, re, struct
sys.path.insert(0, r'C:\tfm2mods')
from _it_scan import O, N, BASE, riprefs
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
from capstone.x86 import X86_OP_REG, X86_OP_MEM, X86_OP_IMM

PT52 = 0x38397a8
PT53 = 0x3277c70

md = Cs(CS_ARCH_X86, CS_MODE_64)
md.detail = True

VOL = {'rax', 'rcx', 'rdx', 'r8', 'r9', 'r10', 'r11'}
R64 = {'al': 'rax', 'cl': 'rcx', 'dl': 'rdx', 'bl': 'rbx', 'sil': 'rsi', 'dil': 'rdi',
       'r8b': 'r8', 'r9b': 'r9', 'r10b': 'r10', 'r11b': 'r11', 'r12b': 'r12',
       'r13b': 'r13', 'r14b': 'r14', 'r15b': 'r15', 'bpl': 'rbp', 'spl': 'rsp',
       'eax': 'rax', 'ecx': 'rcx', 'edx': 'rdx', 'ebx': 'rbx', 'esi': 'rsi', 'edi': 'rdi',
       'r8d': 'r8', 'r9d': 'r9', 'r10d': 'r10', 'r11d': 'r11', 'r12d': 'r12',
       'r13d': 'r13', 'r14d': 'r14', 'r15d': 'r15'}


def norm(r):
    return R64.get(r, r)


def dis_range(E, start, end):
    b = E.read(start, end - start + 16)
    return list(md.disasm(b, BASE + start))


def find_dispatcher(E, lea_site, pt_base):
    """+0 arm 의 lea 사이트에서 역방향으로 디스패처 진입(`add rX,rX`)을 찾는다."""
    # 디스패처는 lea 앞쪽 ~200B 내: 'add rX,rX' ... 'movsxd' ... 'jmp reg'
    start = lea_site - 260
    ins = dis_range(E, start, lea_site + 8)
    # jmp reg (간접) 찾기
    jmp_i = None
    for i, x in enumerate(ins):
        if x.mnemonic == 'jmp' and len(x.operands) == 1 and x.operands[0].type == X86_OP_REG:
            jmp_i = i
    if jmp_i is None:
        return None
    # jmp 직전 4개 = movsxd / add / (lea table)
    seq = ins[max(0, jmp_i - 6):jmp_i + 1]
    add_i = None
    for i in range(jmp_i, -1, -1):
        x = ins[i]
        if x.mnemonic == 'add' and len(x.operands) == 2 and \
           x.operands[0].type == X86_OP_REG and x.operands[1].type == X86_OP_REG and \
           x.operands[0].reg == x.operands[1].reg:
            add_i = i
            break
    if add_i is None:
        return None
    return ins, add_i, jmp_i


def analyze(E, lea_site, pt_base, name=''):
    r = find_dispatcher(E, lea_site, pt_base)
    if not r:
        return {'lea': lea_site, 'err': 'no-dispatcher'}
    ins, add_i, jmp_i = r
    entry_ins = ins[add_i]
    entry = entry_ins.address - BASE
    ban_reg = entry_ins.reg_name(entry_ins.operands[0].reg)
    # rule = jmp 직전 movzx 의 소스
    rule_src = None
    for i in range(add_i - 3, jmp_i):
        if i < 0:
            continue
        x = ins[i]
        if x.mnemonic == 'movzx':
            op = x.operands[1]
            if op.type == X86_OP_REG:
                rule_src = ('reg', x.reg_name(op.reg))
            else:
                rule_src = ('mem', x.reg_name(op.mem.base), op.mem.disp)
    # 합류 = arm 들의 공통 분기 타겟(가장 큰 주소) — arm 본문을 훑어 수집
    # arm 들은 entry 이후 ~400B 내. 'jae'/'ja' 로 가는 공통 타겟 = join
    body = dis_range(E, entry, entry + 480)
    tgts = {}
    total_src = None
    out_reg = None
    for x in body:
        if x.mnemonic in ('jae', 'ja') and x.operands[0].type == X86_OP_IMM:
            t = x.operands[0].imm - BASE
            tgts[t] = tgts.get(t, 0) + 1
        if x.mnemonic == 'cmp' and len(x.operands) == 2 and total_src is None:
            a, b = x.operands
            # cmp <total>, <ban+k>  형태 (arm 첫 비교)
            if b.type == X86_OP_REG and a.type == X86_OP_REG:
                total_src = ('reg', x.reg_name(a.reg))
            elif b.type == X86_OP_REG and a.type == X86_OP_MEM:
                total_src = ('mem', x.reg_name(a.mem.base), a.mem.disp)
    join = None
    if tgts:
        join = max(tgts.items(), key=lambda kv: (kv[1], kv[0]))[0]
    # 출력 레지스터 = 'mov cl,0xff' 류의 대상
    for x in body:
        if x.mnemonic == 'mov' and len(x.operands) == 2 and \
           x.operands[0].type == X86_OP_REG and x.operands[1].type == X86_OP_IMM and \
           x.operands[1].imm == 0xff:
            out_reg = x.reg_name(x.operands[0].reg)
            break
    # 원본이 클로버하는 volatile 집합 (entry..join)
    clob = set()
    if join:
        for x in dis_range(E, entry, join):
            if x.address - BASE >= join:
                break
            _, wr = x.regs_access()
            for w in wr:
                clob.add(norm(x.reg_name(w)))
    return {'name': name, 'lea': lea_site, 'entry': entry, 'join': join,
            'span': (join - entry) if join else None,
            'ban_reg': ban_reg, 'rule_src': rule_src, 'total_src': total_src,
            'out_reg': out_reg, 'clobber_vol': sorted(clob & VOL),
            'container': (E.func_of(entry) or (None,))[0]}


def run(E, pt, tag):
    out = []
    for s in sorted(riprefs(E, pt)):
        out.append(analyze(E, s, pt))
    print(f"===== {tag}: {len(out)} copies")
    for d in out:
        c = d.get('container')
        print(f"  entry={d.get('entry') and hex(d['entry'])} join={d.get('join') and hex(d['join'])} "
              f"span={d.get('span')} cont={hex(c) if c else 'LEAF'} ban={d.get('ban_reg')} "
              f"rule={d.get('rule_src')} total={d.get('total_src')} out={d.get('out_reg')} "
              f"clob={d.get('clobber_vol')} {d.get('err','')}")
    return out


if __name__ == '__main__':
    a = run(O, PT52, '0.5.2')
    b = run(N, PT53, '0.5.3')
    json.dump({'v052': a, 'v053': b}, open(r'C:\tfm2mods\_bo_sites_053.json', 'w'), indent=1)
