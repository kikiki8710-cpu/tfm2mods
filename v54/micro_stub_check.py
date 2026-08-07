# -*- coding: utf-8 -*-
"""class_micro.rs 의 스텁 인코더를 파이썬으로 그대로 재현해 디스어셈으로 검증한다.

왜 필요한가: 스텁은 손으로 짠 기계어라 한 바이트만 틀려도 게임이 죽는다(진단도 어렵다 —
스텁은 어느 모듈에도 속하지 않아 크래시 로그가 module=unknown 만 남긴다).
Rust 를 빌드해 게임에 붙이기 전에, 같은 규칙으로 만든 바이트를 사람이 읽을 수 있는
어셈블리로 되돌려 눈으로 확인한다."""
import sys, io, capstone
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

RN = {0: 'rax', 1: 'rcx', 2: 'rdx', 3: 'rbx', 4: 'rsp', 5: 'rbp', 6: 'rsi', 7: 'rdi',
      8: 'r8', 9: 'r9', 10: 'r10', 11: 'r11', 12: 'r12', 13: 'r13', 14: 'r14', 15: 'r15'}


def rex(w, r, b):
    v = 0x40 | ((1 if w else 0) << 3) | ((1 if r >= 8 else 0) << 2) | (1 if b >= 8 else 0)
    return b'' if v == 0x40 else bytes([v])


def mem_rsp(reg):
    return bytes([((reg & 7) << 3) | 0x04, 0x24])


def value_op(op, dst, src=0):
    o = b''
    if op == 'mov32':
        o += rex(False, dst, 0) + b'\x8b' + mem_rsp(dst)
    elif op == 'add':
        o += rex(True, dst, 0) + b'\x03' + mem_rsp(dst)
    elif op == 'cmp':
        o += rex(True, dst, 0) + b'\x3b' + mem_rsp(dst)
    elif op == 'imul':
        if dst != src:
            o += rex(True, src, dst) + b'\x89' + bytes([0xc0 | ((src & 7) << 3) | (dst & 7)])
        o += rex(True, dst, 0) + b'\x0f\xaf' + mem_rsp(dst)
    elif op == 'leaadd':
        o += rex(True, dst, 0) + b'\x03' + mem_rsp(dst)
    return o


def build(idx, op, dst, src, self_reg, pre, tail, ret_addr, cb, preserve):
    s = b''
    if preserve:
        s += b'\x9c'                                     # pushfq
    s += b'\x48\x8d\x64\x24\xf8'                          # lea rsp,[rsp-8]
    s += bytes([0x50, 0x51, 0x52, 0x41, 0x50, 0x41, 0x51, 0x41, 0x52, 0x41, 0x53, 0x55])
    s += b'\x48\x8d\xa4\x24\xa0\xff\xff\xff'              # lea rsp,[rsp-0x60]
    for k in range(6):
        s += bytes([0x0f, 0x11, 0x44 | ((k & 7) << 3), 0x24, k * 16])
    s += rex(True, self_reg, 2) + b'\x89' + bytes([0xc0 | ((self_reg & 7) << 3) | 2])
    s += b'\x48\x89\xe5'                                  # mov rbp,rsp
    s += b'\x48\x8d\x64\x24\xd0'                          # lea rsp,[rsp-0x30]
    s += b'\x48\x83\xe4\xf0'                              # and rsp,-16
    s += b'\xb9' + idx.to_bytes(4, 'little')              # mov ecx,idx
    s += b'\x48\xb8' + cb.to_bytes(8, 'little') + b'\xff\xd0'
    s += b'\x48\x89\x85' + (0xa0).to_bytes(4, 'little')   # mov [rbp+0xa0],rax
    s += b'\x48\x89\xec'                                  # mov rsp,rbp
    for k in range(6):
        s += bytes([0x0f, 0x10, 0x44 | ((k & 7) << 3), 0x24, k * 16])
    s += b'\x48\x8d\x64\x24\x60'                          # lea rsp,[rsp+0x60]
    s += bytes([0x5d, 0x41, 0x5b, 0x41, 0x5a, 0x41, 0x59, 0x41, 0x58, 0x5a, 0x59, 0x58])
    s += pre
    s += value_op(op, dst, src)
    s += tail
    s += b'\x48\x8d\x64\x24\x08'                          # lea rsp,[rsp+8]
    if preserve:
        s += b'\x9d'                                     # popfq
    s += b'\xff\x25\x00\x00\x00\x00' + ret_addr.to_bytes(8, 'little')
    return s


md = capstone.Cs(capstone.CS_ARCH_X86, capstone.CS_MODE_64)
CASES = [
    ('cs_lead_attack  mov eax,30',      'mov32', 0, 0, 15, b'', b'', True),
    ('ex_order_hold   add rax,10',      'add', 0, 0, 15, b'', b'', False),
    ('mv2_avoid_coef  imul rax,rcx,400', 'imul', 0, 1, 12, b'', b'', False),
    ('mv2_avoid_bias  cmp rax,1500',    'cmp', 0, 0, 6, b'', b'', False),
    ('bt_vision_mem   add r13,0x78',    'add', 13, 0, 14, b'', b'', False),
    ('ex_think_min    lea rcx,[rax+rax*2+400]', 'leaadd', 1, 0, 13,
     b'\x48\x8d\x0c\x40', b'', True),
]
for name, op, dst, src, self_reg, pre, tail, preserve in CASES:
    code = build(3, op, dst, src, self_reg, pre, tail, 0x140db869f, 0x7ff800112233, preserve)
    print('══', name, ' self=%s  len=%d' % (RN[self_reg], len(code)))
    bad = 0
    for ins in md.disasm(code, 0x200000000):
        print('   %-24s %s %s' % (ins.bytes.hex(), ins.mnemonic, ins.op_str))
        if ins.mnemonic == '(bad)':
            bad += 1
    tot = sum(1 for _ in md.disasm(code, 0x200000000))
    consumed = sum(i.size for i in md.disasm(code, 0x200000000))
    print('   → 명령 %d개, 소비 %d/%d 바이트%s' % (tot, consumed, len(code),
          '  ⚠디코드 실패 구간 있음' if consumed < len(code) - 8 else ''))
    print()
