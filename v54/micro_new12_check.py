# -*- coding: utf-8 -*-
"""신규 12사이트(ex_order_hold + bt_vision_mem×11)의 스텁을 실제 파라미터로 만들어 디스어셈 검증.

확인 포인트
  ① self 재료 로드가 **게임 rbp 기준**으로 나오는가(앵커 `mov rbp,rsp` 보다 앞이어야 한다)
  ② 값 op 가 `add <dst>, [rsp]` 인가
  ③ tail 의 `cmp <dst>, rax` 가 **스텁의 마지막 플래그 생산자**인가(원본과 동일해야 뒤 jcc 가 맞다)
"""
import sys, io, capstone
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

RAX, RSP, RBP, RSI, RDI, R12, R14, R15, RDX, R8 = 0, 4, 5, 6, 7, 12, 14, 15, 2, 8
md = capstone.Cs(capstone.CS_ARCH_X86, capstone.CS_MODE_64)


def rex(w, r, b):
    v = 0x40 | ((1 if w else 0) << 3) | ((1 if r >= 8 else 0) << 2) | (1 if b >= 8 else 0)
    return b'' if v == 0x40 else bytes([v])


def mem_load(dst, base, disp):
    o = rex(True, dst, base) + b'\x8b' + bytes([0x80 | ((dst & 7) << 3) | (base & 7)])
    if base & 7 == 4:
        o += b'\x24'
    return o + disp.to_bytes(4, 'little', signed=True)


def load(dst, src, delta):
    kind, r, disp = src
    if kind == 'none':
        return rex(True, dst, dst) + b'\x31' + bytes([0xc0 | ((dst & 7) << 3) | (dst & 7)])
    if kind == 'reg':
        return rex(True, r, dst) + b'\x89' + bytes([0xc0 | ((r & 7) << 3) | (dst & 7)])
    if kind == 'mem':
        return mem_load(dst, r, disp)
    if kind == 'stack':
        return mem_load(dst, RSP, disp + delta)
    raise ValueError(kind)


def build_add(dst, a, b, tail, ret_addr, cb):
    """AddR64 사이트 스텁(preserve_flags=false ⟹ delta=0xa8)."""
    delta = 0xa8
    s = b'\x48\x8d\x64\x24\xf8'
    s += bytes([0x50, 0x51, 0x52, 0x41, 0x50, 0x41, 0x51, 0x41, 0x52, 0x41, 0x53, 0x55])
    s += b'\x48\x8d\xa4\x24\xa0\xff\xff\xff'
    for k in range(6):
        s += bytes([0x0f, 0x11, 0x44 | ((k & 7) << 3), 0x24, k * 16])
    s += load(RDX, a, delta) + load(R8, b, delta)
    s += b'\x48\x89\xe5' + b'\x48\x8d\x64\x24\xd0' + b'\x48\x83\xe4\xf0'
    s += b'\xb9\x00\x00\x00\x00' + b'\x48\xb8' + cb.to_bytes(8, 'little') + b'\xff\xd0'
    s += b'\x48\x89\x85' + (0xa0).to_bytes(4, 'little') + b'\x48\x89\xec'
    for k in range(6):
        s += bytes([0x0f, 0x10, 0x44 | ((k & 7) << 3), 0x24, k * 16])
    s += b'\x48\x8d\x64\x24\x60'
    s += bytes([0x5d, 0x41, 0x5b, 0x41, 0x5a, 0x41, 0x59, 0x41, 0x58, 0x5a, 0x59, 0x58])
    s += rex(True, dst, 0) + b'\x03' + bytes([((dst & 7) << 3) | 0x04, 0x24])   # add dst,[rsp]
    s += tail
    s += b'\x48\x8d\x64\x24\x08'
    s += b'\xff\x25\x00\x00\x00\x00' + ret_addr.to_bytes(8, 'little')
    return s


CASES = [
    ('ex_order_hold  0xe747e3  dst=rax  a=[rbp+0x110] b=r12',
     RAX, ('mem', RBP, 0x110), ('reg', R12, 0), b'\x48\x39\xc6'),
    ('bt_vision_mem  r15×5     a=[rbp+0x610] b=[rbp+0x560]',
     R15, ('mem', RBP, 0x610), ('mem', RBP, 0x560), b'\x49\x39\xc7'),
    ('bt_vision_mem  rdi×1  (0xda5234)',
     RDI, ('mem', RBP, 0x610), ('mem', RBP, 0x560), b'\x48\x39\xc7'),
    ('bt_vision_mem  r14×1  (0xda5dae)',
     R14, ('mem', RBP, 0x610), ('mem', RBP, 0x560), b'\x49\x39\xc6'),
    ('bt_vision_mem  rsi×4',
     RSI, ('mem', RBP, 0x610), ('mem', RBP, 0x560), b'\x48\x39\xc6'),
]

KEY = ('mov rdx', 'mov r8', 'mov rbp, rsp', 'add ', 'cmp ', 'jmp ')
for name, dst, a, b, tail in CASES:
    code = build_add(dst, a, b, tail, 0x140da462c, 0x7ff800112233)
    print('══', name, ' len=%d' % len(code))
    ins = list(md.disasm(code, 0x200000000))
    for i in ins:
        t = '%s %s' % (i.mnemonic, i.op_str)
        if any(t.startswith(k) for k in KEY):
            print('   %-20s %s' % (i.bytes.hex(), t))
    print('   소비 %d/%d 바이트\n' % (sum(i.size for i in ins), len(code)))
