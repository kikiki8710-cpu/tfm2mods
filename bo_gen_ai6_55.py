# -*- coding: utf-8 -*-
"""AI6 인라인 phase 6사이트 스텁 생성기 (0.5.5). bo_gen_ai6.py 이식 + post-arm 부작용 재현."""
import json
from bo_055 import N, BASE
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
MD = Cs(CS_ARCH_X86, CS_MODE_64)
R = {'rax':0,'rcx':1,'rdx':2,'rbx':3,'rsp':4,'rbp':5,'rsi':6,'rdi':7,'r8':8,'r9':9,'r10':10,'r11':11,'r12':12,'r13':13,'r14':14,'r15':15}
R8 = {'al':0,'cl':1,'dl':2,'bl':3,'spl':4,'bpl':5,'sil':6,'dil':7,'r8b':8,'r9b':9,'r10b':10,'r11b':11,'r12b':12,'r13b':13,'r14b':14,'r15b':15}
def modrm(mod,reg,rm): return bytes([(mod<<6)|((reg&7)<<3)|(rm&7)])
def mov_rr(dst,src):
    d,s=R[dst],R[src]; return bytes([0x48|((s>=8)<<2)|(d>=8),0x89])+modrm(3,s,d)
def movzx_r8(dst,src8):
    d,s=R[dst],R8[src8]; rex=0x40|((d>=8)<<2)|(s>=8); return bytes([rex,0x0f,0xb6])+modrm(3,d,s)
def movzx_mem_rbp(dst,disp):
    d=R[dst]; rex=0x40|((d>=8)<<2); return bytes([rex,0x0f,0xb6])+modrm(2,d,5)+disp.to_bytes(4,'little',signed=True)
def shr1(reg):
    r=R[reg]; return bytes([0x48|(r>=8),0xd1])+modrm(3,5,r)
def save(reg,off):
    r=R[reg]; return bytes([0x48|((r>=8)<<2),0x89])+modrm(1,r,4)+b'\x24'+bytes([off])
def load(reg,off):
    r=R[reg]; return bytes([0x48|((r>=8)<<2),0x8b])+modrm(1,r,4)+b'\x24'+bytes([off])
def mov_out(dst8):
    d=R8[dst8]; pre=bytes([0x40|(d>=8)]) if (d>=8 or d in (4,5,6,7)) else b''; return pre+b'\x88'+modrm(3,0,d)
SUBSP=b'\x48\x83\xec\x70'; ADDSP=b'\x48\x83\xc4\x70'; CALLRAX=b'\xff\xd0'
VOL=['rcx','rdx','r8','r9','r10','r11']; SLOT={r:0x28+8*i for i,r in enumerate(VOL)}; RSLOT=0x58

# 0.5.5 재핀 파라미터. side = post-dispatch arm 부작용 명령 주소(스텁이 재현, non-vol 대상).
SITES=[
 dict(name='ai_reco1', patch=0x1cb8b72, join=0x1cb900e, total='rcx', ban='rdx', ban_doubled=True, rule=('reg','r12b'), out='al', side=[]),
 dict(name='ai_reco2', patch=0x1cb8d60, join=0x1cb96a7, total='rcx', ban='r8',  ban_doubled=True, rule=('reg','r12b'), out='al', side=[0x1cb8d6b,0x1cb8d6f]),
 dict(name='ai_comp',  patch=0x1cba2a5, join=0x1cba40c, total='rcx', ban='rdx', ban_doubled=True, rule=('reg','r12b'), out='al', side=[0x1cba2b0]),
 dict(name='ai_bb1',   patch=0x1cbb38d, join=0x1cbb7bf, total='rax', ban='rcx', ban_doubled=True, rule=('reg','r10b'), out='r8b', side=[]),
 dict(name='ai_bb2',   patch=0x1cbb479, join=0x1cbb5d0, total='rcx', ban='rsi', ban_doubled=True, rule=('mem_rbp',0x198), out='al', side=[]),
 dict(name='ai_bb3',   patch=0x1cbbeea, join=0x1cbc1ba, total='rcx', ban='r13', ban_doubled=False, rule=('reg','r14b'), out='al', side=[]),
]
def gen(site):
    b=bytearray()
    kind=site['rule'][0]
    rule_first = movzx_r8('r11',site['rule'][1]) if kind=='reg' else movzx_mem_rbp('r11',site['rule'][1])
    b+=SUBSP
    for r in VOL: b+=save(r,SLOT[r])
    b+=rule_first
    b+=mov_rr('r10',site['ban']); b+=mov_rr('r9',site['total'])
    b+=mov_rr('rcx','r9'); b+=mov_rr('rdx','r11'); b+=mov_rr('r8','r10')
    if site['ban_doubled']: b+=shr1('r8')
    fn_off=len(b)+2
    b+=b'\x48\xb8'+b'\x00'*8; b+=CALLRAX
    b+=b'\x88\x44\x24'+bytes([RSLOT])
    for r in VOL: b+=load(r,SLOT[r])
    b+=b'\x0f\xb6\x44\x24'+bytes([RSLOT])
    if site['out']!='al': b+=mov_out(site['out'])
    # post-dispatch arm 부작용 재현 (non-vol rbx/r14 대상 — volatile 복원 뒤라 안전)
    for a in site['side']:
        ins=next(MD.disasm(bytes(N.read(a,16)),0)); b+=ins.bytes
    b+=ADDSP
    join_off=len(b)+6
    b+=b'\xff\x25\x00\x00\x00\x00'+b'\x00'*8
    return bytes(b),fn_off,join_off
if __name__=='__main__':
    rows=[]
    for s in SITES:
        code,fo,jo=gen(s); span=s['join']-s['patch']
        sig=N.read(s['patch'],8).hex()
        print(f"== {s['name']}: patch={s['patch']:#x} join={s['join']:#x} span={span} stub={len(code)}B fn@{fo} join@{jo} sig={sig}")
        assert span>=14, f"span<14 {s['name']}"
        for ins in MD.disasm(code,0x1000):
            print(f"   {ins.address:#06x} {ins.bytes.hex():<20} {ins.mnemonic} {ins.op_str}")
        rows.append(dict(name=s['name'],patch=s['patch'],join=s['join'],sig=sig,stub=code.hex(),fn_off=fo,join_off=jo))
    json.dump(rows,open(r'C:\tfm2mods\_bo_ai6_stubs_55.json','w'),indent=1)
    print("\n// ── AI6 (0.5.5) hooks.rs 붙여넣기 ──")
    print("const AI6: [(usize, usize, &[u8], &[u8], usize, usize); 6] = [")
    for r in rows:
        sig=', '.join(f'0x{r["sig"][i:i+2]}' for i in range(0,16,2))
        stub=', '.join(f'0x{r["stub"][i:i+2]}' for i in range(0,len(r['stub']),2))
        print(f"    // {r['name']}")
        print(f"    (0x{r['patch']:x}, 0x{r['join']:x}, &[{sig}], &[{stub}], {r['fn_off']}, {r['join_off']}),")
    print("];")
