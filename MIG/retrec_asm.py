# -*- coding: utf-8 -*-
# 리턴 주소 히스토그램 스텁 조각(손 인코딩) — capstone 으로 검증하고 Rust 상수로 출력
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
CODE_OFF = 8          # 스텁 +8 = lock inc 시작
TBL = 0x100           # +0x100 addr[16] · +0x180 cnt[16] · +0x200 overflow
b = bytearray()
b += bytes([0xF0, 0x48, 0xFF, 0x05, 0xF0, 0xFF, 0xFF, 0xFF])   # lock inc qword [rip-16] (카운터 +0)
b += bytes([0x50, 0x51, 0x52, 0x41, 0x50, 0x41, 0x51])          # push rax,rcx,rdx,r8,r9
b += bytes([0x4C, 0x8B, 0x44, 0x24, 0x28])                       # mov r8,[rsp+0x28]  (ret addr)
b += bytes([0x4C, 0x89, 0xC1])                                   # mov rcx,r8
b += bytes([0x48, 0xC1, 0xE9, 0x04])                             # shr rcx,4
b += bytes([0x48, 0x83, 0xE1, 0x0F])                             # and rcx,15
lea_at = len(b)
b += bytes([0x48, 0x8D, 0x15, 0, 0, 0, 0])                       # lea rdx,[rip+disp] → +TBL
disp = TBL - (CODE_OFF + len(b))
b[lea_at + 3:lea_at + 7] = disp.to_bytes(4, "little", signed=True)
b += bytes([0x45, 0x31, 0xC9])                                   # xor r9d,r9d
loop = len(b)
b += bytes([0x48, 0x8B, 0x04, 0xCA])                             # mov rax,[rdx+rcx*8]
b += bytes([0x4C, 0x39, 0xC0])                                   # cmp rax,r8
je1 = len(b); b += bytes([0x74, 0])                              # je hit
b += bytes([0x48, 0x85, 0xC0])                                   # test rax,rax
jnz = len(b); b += bytes([0x75, 0])                              # jnz next
b += bytes([0x31, 0xC0])                                         # xor eax,eax
b += bytes([0xF0, 0x4C, 0x0F, 0xB1, 0x04, 0xCA])                 # lock cmpxchg [rdx+rcx*8],r8
je2 = len(b); b += bytes([0x74, 0])                              # je hit
b += bytes([0x4C, 0x39, 0xC0])                                   # cmp rax,r8
je3 = len(b); b += bytes([0x74, 0])                              # je hit
nxt = len(b)
b += bytes([0x48, 0xFF, 0xC1])                                   # inc rcx
b += bytes([0x48, 0x83, 0xE1, 0x0F])                             # and rcx,15
b += bytes([0x41, 0xFF, 0xC1])                                   # inc r9d
b += bytes([0x41, 0x83, 0xF9, 0x10])                             # cmp r9d,16
jl = len(b); b += bytes([0x7C, 0])                               # jl loop
b += bytes([0xF0, 0x48, 0xFF, 0x82]) + (0x100).to_bytes(4, "little")   # lock inc qword [rdx+0x100] (overflow)
jmpd = len(b); b += bytes([0xEB, 0])                             # jmp done
hit = len(b)
b += bytes([0xF0, 0x48, 0xFF, 0x84, 0xCA]) + (0x80).to_bytes(4, "little")  # lock inc qword [rdx+rcx*8+0x80]
done = len(b)
b += bytes([0x41, 0x59, 0x41, 0x58, 0x5A, 0x59, 0x58])          # pop r9,r8,rdx,rcx,rax
def rel8(at, tgt): b[at + 1] = (tgt - (at + 2)) & 0xff
rel8(je1, hit); rel8(jnz, nxt); rel8(je2, hit); rel8(je3, hit); rel8(jl, loop); rel8(jmpd, done)
md = Cs(CS_ARCH_X86, CS_MODE_64)
for i in md.disasm(bytes(b), 0x1000 + CODE_OFF):
    print("%4x: %-28s %s %s" % (i.address - 0x1000, i.bytes.hex(), i.mnemonic, i.op_str))
print("len", len(b))
print("pub const RETREC: [u8; %d] = [%s];" % (len(b), ", ".join("0x%02x" % x for x in b)))
