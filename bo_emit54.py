# -*- coding: utf-8 -*-
"""0.5.4 AI parity emit + AI6_6 stub 재생성 — 인코딩 검증용 디스어셈 출력."""
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
md=Cs(CS_ARCH_X86,CS_MODE_64)
def show(tag,b,org=0x149e561):
    print('==',tag,len(b),'bytes')
    for i in md.disasm(bytes(b),org):
        print(f'  {i.address:#x} {bytes(i.bytes).hex():<20} {i.mnemonic} {i.op_str}')

# ── AI1 (site 0x149e561, join 0x149e680) ──
ai1 = bytearray()
ai1 += bytes.fromhex('4c8b5b10')                 # mov r11,[rbx+0x10]
ai1 += bytes.fromhex('4c897570')                 # mov [rbp+0x70],r14
ai1 += bytes.fromhex('4c899d88000000')           # mov [rbp+0x88],r11
ai1 += bytes.fromhex('4b8d0c33')                 # lea rcx,[r11+r14]
ai1 += bytes.fromhex('0fb6d2')                   # movzx edx,dl
ai1 += bytes.fromhex('48b8')+b'\x11'*8           # movabs rax,fn
ai1 += bytes.fromhex('ffd0')                     # call rax
ai1 += bytes.fromhex('0fb6d0')                   # movzx edx,al
ai1 += bytes.fromhex('4c8b5b10')                 # mov r11,[rbx+0x10]
ai1 += bytes.fromhex('4f8d0c33')                 # lea r9,[r11+r14]
ai1 += bytes.fromhex('0fb68e130e0000')           # movzx ecx,byte [rsi+0xe13]
ai1 += bytes.fromhex('0fb68597000000')           # movzx eax,byte [rbp+0x97]
ai1 += b'\xe9'+b'\x22'*4                          # jmp join
show('AI1 patch',ai1,0x149e561)
print('   len',len(ai1),' fn_off',ai1.find(b'\x11'*8))

# ── AI2 (site 0x14a1f1e, join 0x14a1fef) ──
ai2 = bytearray()
ai2 += bytes.fromhex('4c8b8520010000')           # mov r8,[rbp+0x120]
ai2 += bytes.fromhex('4d8b642410')               # mov r12,[r12+0x10]
ai2 += bytes.fromhex('4b8d0c3c')                 # lea rcx,[r12+r15]
ai2 += bytes.fromhex('0fb69518010000')           # movzx edx,byte [rbp+0x118]
ai2 += bytes.fromhex('48b8')+b'\x11'*8
ai2 += bytes.fromhex('ffd0')
ai2 += bytes.fromhex('0fb6c8')                   # movzx ecx,al
ai2 += bytes.fromhex('4b8d143c')                 # lea rdx,[r12+r15]  (총합 복원)
ai2 += bytes.fromhex('0fb687130e0000')           # movzx eax,byte [rdi+0xe13]
ai2 += b'\xe9'+b'\x22'*4
show('AI2 patch',ai2,0x14a1f1e)
print('   len',len(ai2),' fn_off',ai2.find(b'\x11'*8))
