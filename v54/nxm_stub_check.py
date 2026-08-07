# -*- coding: utf-8 -*-
"""nexus_emg.rs 의 **마진(강도) 스텁**(install_nxm_detour)을 재현해 capstone 으로 검증한다.

확인 포인트
  ① `test eax,0x100` / `jnz .body` 가 맨 앞이라 비상이 아닐 때 아무것도 저장하지 않고 빠지는가
  ② reg·side 로드가 **앵커(`mov rbp,rsp`)보다 앞**인가 (rbp 가 아직 게임 프레임이어야 한다)
  ③ 감산 블록 재현이 원본과 같은 순서·의미인가 (`xor eax,eax` → `sub` → `cmovb` ×2)
  ④ 분기가 `.body` 명령 경계에 착지하는가 · 스택 균형

⚠이 파일을 bash heredoc 으로 고치지 말 것(백슬래시 먹힘).
"""
import sys, io, capstone
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

BASE = 0x140000000
PLAIN = 0xe097f5
AFTER = 0xe097db
CB = 0x7ff800112233
O_REG, O_SIDE, O_NEAR = 0x8d0, 0x8c0, 0x880


def build():
    s = bytearray()
    fix = []
    s += b'\xa9\x00\x01\x00\x00'                       # test eax,0x100
    s += b'\x75\x00'; fix.append(len(s) - 1)           # jnz .body
    s += b'\xff\x25\x00\x00\x00\x00' + (BASE + PLAIN).to_bytes(8, 'little')
    body = len(s)
    s += b'\x48\x8d\x64\x24\xf8'                       # lea rsp,[rsp-8]
    s += bytes([0x50, 0x51, 0x52, 0x41, 0x50, 0x41, 0x51, 0x41, 0x52, 0x41, 0x53, 0x55])
    s += b'\x48\x8d\xa4\x24\xa0\xff\xff\xff'
    for k in range(6):
        s += bytes([0x0f, 0x11, 0x44 | ((k & 7) << 3), 0x24, k * 16])
    s += b'\x48\x8b\x8d' + O_REG.to_bytes(4, 'little')     # mov rcx,[rbp+0x8d0]
    s += b'\x48\x8b\x95' + O_SIDE.to_bytes(4, 'little')    # mov rdx,[rbp+0x8c0]
    s += b'\x48\x89\xe5'                                   # mov rbp,rsp   ← 앵커
    s += b'\x48\x8d\x64\x24\xd0' + b'\x48\x83\xe4\xf0'
    s += b'\x48\xb8' + CB.to_bytes(8, 'little') + b'\xff\xd0'
    s += b'\x48\x89\x85' + (0xa0).to_bytes(4, 'little')
    s += b'\x48\x89\xec'
    for k in range(6):
        s += bytes([0x0f, 0x10, 0x44 | ((k & 7) << 3), 0x24, k * 16])
    s += b'\x48\x8d\x64\x24\x60'
    s += bytes([0x5d, 0x41, 0x5b, 0x41, 0x5a, 0x41, 0x59, 0x41, 0x58, 0x5a, 0x59, 0x58])
    s += b'\x31\xc0'                                       # xor eax,eax
    s += b'\x4c\x2b\x2c\x24'                               # sub r13,[rsp]
    s += b'\x4c\x0f\x42\xe8'                               # cmovb r13,rax
    s += b'\x48\x8b\x8d' + O_NEAR.to_bytes(4, 'little')    # mov rcx,[rbp+0x880]
    s += b'\x48\x2b\x0c\x24'                               # sub rcx,[rsp]
    s += b'\x48\x0f\x42\xc8'                               # cmovb rcx,rax
    s += b'\x48\x8d\x64\x24\x08'                           # lea rsp,[rsp+8]
    s += b'\xff\x25\x00\x00\x00\x00' + (BASE + AFTER).to_bytes(8, 'little')
    for p in fix:
        d = body - (p + 1)
        assert 0 <= d <= 127, 'rel8 범위 초과: %d' % d
        s[p] = d
    return bytes(s), body


code, body = build()
md = capstone.Cs(capstone.CS_ARCH_X86, capstone.CS_MODE_64)
md.detail = True
STUB = 0x200000000
print('마진 스텁 %d바이트   .body=+%#x\n' % (len(code), body))

anchor_off = None
reg_off = side_off = None
ok = True
for ins in md.disasm(code, STUB):
    off = ins.address - STUB
    t = '%s %s' % (ins.mnemonic, ins.op_str)
    show = False
    tag = ''
    if off == body:
        tag = '   ← .body'; show = True
    if ins.mnemonic == 'jne':
        tgt = ins.operands[0].imm - STUB
        tag = '   → +%#x %s' % (tgt, '(.body) OK' if tgt == body else '★어긋남')
        if tgt != body: ok = False
        show = True
    if 'rbp + 0x8d0' in t: reg_off = off; show = True
    if 'rbp + 0x8c0' in t: side_off = off; show = True
    if t == 'mov rbp, rsp': anchor_off = off; show = True
    if ins.mnemonic in ('test', 'xor', 'sub', 'cmovb', 'call', 'jmp') or 'rbp + 0x880' in t:
        show = True
    if show:
        print('  +%03x  %-22s %-6s %-30s%s' % (off, ins.bytes.hex(), ins.mnemonic, ins.op_str, tag))

print()
print('② reg 로드(+%#x)·side 로드(+%#x) 가 앵커(+%#x) 보다 앞인가 : %s'
      % (reg_off, side_off, anchor_off,
         'OK' if reg_off < anchor_off and side_off < anchor_off else '★아님 — rbp 가 이미 우리 것'))
consumed = sum(i.size for i in md.disasm(code, STUB))
print('소비 %d/%d 바이트 (끝 8B×2 = 점프 타깃 리터럴)' % (consumed, len(code)))
print('결과:', '정상' if ok else '★분기 어긋남 — 설치 금지')
