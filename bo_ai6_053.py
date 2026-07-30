# -*- coding: utf-8 -*-
"""banpick_order 0.5.3 — AI 조합/추천 함수의 인라인 phase 복제본 6곳 메타 추출.

0.5.2에서는 이 6곳이 phase_from(B)를 '호출'했기에 B 전체대체로 커버됐다(0xefef00 x2,
0xefff70 x1, 0xf00bb0 x2, 0xf014d0 x1). 0.5.3에서는 전부 인라인 → 코치 위임(AI 행동)
경로가 바닐라 순서로 판단해 인터리브에서 멈춘다.

각 복제본에서 (디스패처 jmp 주소 / total·2*ban·rule 레지스터 / 출력 8비트 레지스터 /
합류주소)를 뽑는다. 패치는 사이트에 `jmp [rip+0]`(14B, 레지스터 무클로버) → 스텁.
"""
import sys, json, collections
sys.path.insert(0, r'C:\tfm2mods')
from _it_scan import N, BASE, riprefs
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
from capstone.x86 import X86_OP_REG, X86_OP_IMM

md = Cs(CS_ARCH_X86, CS_MODE_64)
md.detail = True
PT = 0x3277c70
CONTS = [0x188dd30, 0x188f360, 0x1890450, 0x1890fd0]


def sites():
    out = []
    for s in sorted(riprefs(N, PT)):
        f = N.func_of(s)
        if f and f[0] in CONTS:
            out.append((f[0], s))
    return out


def analyze(cont, lea):
    s, e = N.func_of(cont)
    ins = list(md.disasm(N.read(s, e - s), BASE + s))
    at = {x.address - BASE: i for i, x in enumerate(ins)}
    # 1) lea 이전에서 가장 가까운 indirect jmp = 디스패처
    k = max(i for i, x in enumerate(ins) if x.address - BASE <= lea)
    ji = None
    for i in range(k, max(0, k - 80), -1):
        if ins[i].mnemonic == 'jmp' and ins[i].operands[0].type == X86_OP_REG:
            ji = i
            break
    j = ins[ji]
    jmp_reg = j.reg_name(j.operands[0].reg)
    # 2) rule = 디스패처 직전 movzx 의 소스 8비트 레지스터
    rule = None
    for i in range(ji, max(0, ji - 12), -1):
        if ins[i].mnemonic == 'movzx' and ins[i].operands[1].type == X86_OP_REG:
            rule = ins[i].reg_name(ins[i].operands[1].reg)
            break
    # 3) 2*ban = `lea rX,[rY+rY]` 또는 `add rX,rX` 의 목적 레지스터
    dbl = None
    for i in range(ji, max(0, ji - 14), -1):
        x = ins[i]
        if x.mnemonic == 'lea' and '+' in x.op_str:
            dst, mem = x.op_str.split(', ')
            body = mem.strip('[]')
            parts = [p.strip() for p in body.split('+')]
            if len(parts) == 2 and parts[0] == parts[1]:
                dbl = dst
                break
        if x.mnemonic == 'add' and x.operands[0].type == X86_OP_REG and \
           x.operands[1].type == X86_OP_REG and x.operands[0].reg == x.operands[1].reg:
            dbl = x.reg_name(x.operands[0].reg)
            break
    # 4) arm 들: 디스패처 직후 ~ 합류. 첫 arm = `lea rA,[dbl+4]` / `mov out,0xff` / `cmp total,rA`
    out_reg = None
    total = None
    tg = collections.Counter()
    for x in ins[ji + 1: ji + 90]:
        if x.mnemonic == 'mov' and x.operands[1].type == X86_OP_IMM and x.operands[1].imm == 0xff \
           and x.operands[0].type == X86_OP_REG and out_reg is None:
            out_reg = x.reg_name(x.operands[0].reg)
        if x.mnemonic == 'cmp' and total is None and x.operands[0].type == X86_OP_REG \
           and x.operands[1].type == X86_OP_REG:
            total = x.reg_name(x.operands[0].reg)
        if x.mnemonic in ('jae', 'ja') and x.operands[0].type == X86_OP_IMM:
            tg[x.operands[0].imm - BASE] += 1
    join = max(tg.items(), key=lambda kv: (kv[1], kv[0]))[0] if tg else None
    return {
        'cont': cont, 'lea': lea, 'jmp': j.address - BASE, 'jmp_len': j.size,
        'jmp_reg': jmp_reg, 'rule': rule, 'dbl_ban': dbl, 'total': total,
        'out': out_reg, 'join': join,
        'span': (join - (j.address - BASE)) if join else None,
        'sig': N.hexat(j.address - BASE, 14),
    }


if __name__ == '__main__':
    res = [analyze(c, l) for c, l in sites()]
    for d in res:
        print(f"cont={d['cont']:#x} jmp={d['jmp']:#x}({d['jmp_reg']}) total={d['total']} "
              f"dbl_ban={d['dbl_ban']} rule={d['rule']} out={d['out']} "
              f"join={d['join'] and hex(d['join'])} span={d['span']}")
    json.dump(res, open(r'C:\tfm2mods\_bo_ai6_053.json', 'w'), indent=1)
