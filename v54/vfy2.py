# -*- coding: utf-8 -*-
"""vfy2.py — vfy.py 와 같지만 **즉치가 아닌 슬롯(SIB 스케일비트 등)** 도 검증한다.
   `--sib` 행은 imm/disp 위치 검사를 건너뛰고 prefix+명령경계+그 자리 바이트값만 본다.
   입력: `054rva prefixhex off w orig [sib]`
"""
import io, sys, bisect
sys.path.insert(0, r'C:\tfm2mods\v54')
import reloc as R
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
B = 0x140000000; E4 = R.E4
_FN = E4.funcs(); _FS = [s for s, e in _FN]; _c = {}
def f_of(r):
    k = bisect.bisect_right(_FS, r) - 1
    if k < 0: return None
    s, e = _FN[k]
    return (s, e) if s <= r < e else None
def ins_at(r):
    f = f_of(r)
    if not f: return None
    if f[0] not in _c: _c[f[0]] = {i.address-B: i for i in R.insns(E4, f[0], f[1])}
    return _c[f[0]].get(r)
n=ng=0
for ln in (io.open(sys.argv[1],encoding='utf-8') if len(sys.argv)>1 else sys.stdin):
    p = ln.split('#')[0].split()
    if len(p) < 5: continue
    rva,pre,off,w,orig = int(p[0],16),p[1],int(p[2]),int(p[3]),int(p[4])
    sib = len(p) > 5 and p[5]=='sib'
    i = ins_at(rva); n+=1
    if i is None: print('%06x NG 명령경계아님'%rva); ng+=1; continue
    b=i.bytes; pb=bytes.fromhex(pre); m=[]
    if not b.startswith(pb): m.append('prefix불일치 실제=%s'%b[:len(pb)].hex())
    if len(pb)!=off: m.append('prefix길이%d≠off%d'%(len(pb),off))
    if off+w>len(b): m.append('off+w>len%d'%len(b))
    if not sib:
        e=getattr(i,'encoding',None)
        io_,is_=(getattr(e,'imm_offset',0),getattr(e,'imm_size',0)) if e else (0,0)
        do_,ds_=(getattr(e,'disp_offset',0),getattr(e,'disp_size',0)) if e else (0,0)
        if not ((is_ and off==io_) or (ds_ and off==do_)): m.append('off%d≠imm@%d/disp@%d'%(off,io_,do_))
    v=int.from_bytes(b[off:off+w],'little') if off+w<=len(b) else None
    if v!=orig: m.append('값%s≠%s'%(v,orig))
    if m: print('%06x NG %s [%s %s %s]'%(rva,' / '.join(m),b.hex(),i.mnemonic,i.op_str)); ng+=1
    else: print('%06x OK %-24s %s %s'%(rva,b.hex(),i.mnemonic,i.op_str))
print('\n검사 %d / 실패 %d'%(n,ng))
