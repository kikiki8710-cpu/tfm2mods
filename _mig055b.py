# -*- coding: utf-8 -*-
# 마스크-바디 시그니처 폴백 (0.5.4 -> 0.5.5). rip-rel disp / 분기 rel 만 와일드카드.
import struct
from capstone import *
from capstone.x86 import X86_OP_MEM, X86_REG_RIP, X86_OP_IMM, X86_GRP_JUMP, X86_GRP_CALL
import _mig055 as M
md=Cs(CS_ARCH_X86,CS_MODE_64); md.detail=True
def text_sec(img):
    for nm,va,vsz,rraw,rsz in img.secs:
        if nm==".text": return va,rraw,rsz
def make_pattern(img, rva, nbytes):
    code=img.read(rva,nbytes+16); pat=bytearray(); mask=bytearray()
    for ins in md.disasm(code, 0x140000000+rva):
        if len(pat)>=nbytes: break
        wild=any(op.type==X86_OP_MEM and op.mem.base==X86_REG_RIP for op in ins.operands)
        if (ins.group(X86_GRP_JUMP) or ins.group(X86_GRP_CALL)) and any(op.type==X86_OP_IMM for op in ins.operands): wild=True
        for bb in ins.bytes: pat.append(bb); mask.append(0 if wild else 1)
    return bytes(pat),bytes(mask)
def find(img, pat, mask):
    va,raw,rsz=text_sec(img); text=img.raw[raw:raw+rsz]
    pre=bytearray()
    for b,m in zip(pat,mask):
        if m: pre.append(b)
        else: break
    pre=bytes(pre); hits=[]; s=0
    if len(pre)<4: return None
    while True:
        i=text.find(pre,s)
        if i<0: break
        s=i+1; seg=text[i:i+len(pat)]
        if len(seg)==len(pat) and all(mask[j]==0 or seg[j]==pat[j] for j in range(len(pat))): hits.append(va+i)
    return hits
def msig(rva, nb=0xa0):
    pat,mask=make_pattern(M.O,rva,nb); hits=find(M.N,pat,mask)
    return hits, sum(mask)

# ── 문자열 xref + 콜그래프 (0.5.5 exe N 대상) ──
import collections
def str_rvas(img, needle):
    out=set(); d=img.raw; s=0
    while True:
        i=d.find(needle,s)
        if i<0: break
        for nm,va,vsz,rraw,rsz in img.secs:
            if rraw<=i<rraw+rsz: out.add(va+(i-rraw))
        s=i+1
    return out
def callee_of_string(img, needle):
    """needle을 lea rip-rel로 로드 직후 call 하는 함수 RVA 집계"""
    va,raw,rsz=text_sec(img); text=img.raw[raw:raw+rsz]
    srv=str_rvas(img,needle); modrm={0x05,0x0d,0x15,0x1d,0x2d,0x35,0x3d}
    cal=collections.Counter()
    for i in range(len(text)-7):
        if text[i] in (0x48,0x4c) and text[i+1]==0x8d and text[i+2] in modrm:
            disp=struct.unpack_from("<i",text,i+3)[0]
            if (va+i+7+disp) in srv:
                code=text[i:i+300]
                for ins in md.disasm(code, va+i):
                    if ins.group(X86_GRP_CALL) and ins.operands and ins.operands[0].type==X86_OP_IMM:
                        cal[ins.operands[0].imm]+=1; break
    return cal
def callers(img, tgt):
    """tgt(rva) 를 e8 rel32로 호출하는 콜사이트 수"""
    va,raw,rsz=text_sec(img); text=img.raw[raw:raw+rsz]; n=0; i=0
    while True:
        i=text.find(b"\xe8",i)
        if i<0 or i+5>len(text): break
        rel=struct.unpack_from("<i",text,i+1)[0]
        if va+i+5+rel==tgt: n+=1
        i+=1
    return n
