# -*- coding: utf-8 -*-
import sys
sys.path.insert(0,r'C:\tfm2mods')
from bo_054 import N,BASE
from capstone import Cs,CS_ARCH_X86,CS_MODE_64
md=Cs(CS_ARCH_X86,CS_MODE_64)
R={'rax':0,'rcx':1,'rdx':2,'rbx':3,'rsp':4,'rbp':5,'rsi':6,'rdi':7,'r8':8,'r9':9,'r10':10,'r11':11,'r12':12,'r13':13,'r14':14,'r15':15}
R8={'al':0,'cl':1,'dl':2,'bl':3,'sil':6,'dil':7,'r8b':8,'r9b':9,'r10b':10,'r11b':11,'r12b':12,'r13b':13,'r14b':14,'r15b':15}
def modrm(m,r,rm): return bytes([(m<<6)|((r&7)<<3)|(rm&7)])
def mov_rr(d,s):
    a,b=R[d],R[s]; return bytes([0x48|((b>=8)<<2)|(a>=8),0x89])+modrm(3,b,a)
def movzx_r8(dst,src8):
    d,s=R[dst],R8[src8]; return bytes([0x40|((d>=8)<<2)|(s>=8),0x0f,0xb6])+modrm(3,d,s)
def save(r,off):
    x=R[r]; return bytes([0x48|((x>=8)<<2),0x89])+modrm(1,x,4)+b'\x24'+bytes([off])
def load(r,off):
    x=R[r]; return bytes([0x48|((x>=8)<<2),0x8b])+modrm(1,x,4)+b'\x24'+bytes([off])
VOL=['rcx','rdx','r8','r9','r10','r11']; SLOT={r:0x28+8*i for i,r in enumerate(VOL)}; RSLOT=0x58
b=bytearray()
b+=b'\x48\x83\xec\x70'
for r in VOL: b+=save(r,SLOT[r])
b+=movzx_r8('r11','dl')
b+=mov_rr('r10','r13')
b+=mov_rr('r9','rcx')
for a in (0x21612d4,0x21612dc,0x21612e3,0x21612ea):
    ins=next(md.disasm(bytes(N.read(a,16)),0)); b+=ins.bytes
b+=mov_rr('rcx','r9'); b+=mov_rr('rdx','r11'); b+=mov_rr('r8','r10')
fn_off=len(b)+2
b+=b'\x48\xb8'+b'\x00'*8+b'\xff\xd0'
b+=b'\x88\x44\x24'+bytes([RSLOT])
for r in VOL: b+=load(r,SLOT[r])
b+=b'\x0f\xb6\x44\x24'+bytes([RSLOT])
b+=b'\x48\x83\xc4\x70'
join_off=len(b)+6
b+=b'\xff\x25\x00\x00\x00\x00'+b'\x00'*8
print('len',len(b),'fn_off',fn_off,'join_off',join_off)
for i in md.disasm(bytes(b),0x1000): print(f'  {i.bytes.hex():<22} {i.mnemonic} {i.op_str}')
print('\nRUST:')
print(', '.join(f'0x{x:02x}' for x in b))
print('SIG(new site 0x21612c3):', N.read(0x21612c3,8).hex(' '))
