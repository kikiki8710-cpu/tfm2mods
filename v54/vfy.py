# -*- coding: utf-8 -*-
"""★vfy.py — 제안한 054 사이트를 **exe 실바이트로 최종 검증**한다.
검사 5가지(하나라도 실패하면 쓰지 마라):
  ① 그 RVA 가 .pdata 함수 안의 **명령 시작**인가 (함수 시작부터 선형 디코드)
  ② 실바이트가 제안 prefix 로 시작하는가
  ③ capstone 인코딩의 **imm_offset(또는 disp_offset)이 제안 off 와 정확히 일치**하는가
     ← 2026-08-05 크래시(REX 접두로 off 2→3 밀림)를 막는 핵심 검사
  ④ imm_size 가 제안 width 이상인가 / off+w ≤ 명령길이
  ⑤ 그 자리에서 읽은 값이 제안 orig 와 같은가
입력: TSV/인자 `054rva prefixhex off w orig`
"""
import io, sys
sys.path.insert(0, r'C:\tfm2mods\v54')
import reloc as R
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
B = 0x140000000
E4 = R.E4
_c = {}

def ins_at(rva):
    f = E4.func_of(rva)
    if not f: return None, None
    if f[0] not in _c:
        _c[f[0]] = {i.address - B: i for i in R.insns(E4, f[0], f[1])}
    return _c[f[0]].get(rva), f

def check(rva, pre, off, w, orig):
    i, f = ins_at(rva)
    if i is None:
        return False, '명령경계아님/함수밖 (fn=%s)' % ('%06x' % f[0] if f else '-')
    b = i.bytes
    pb = bytes.fromhex(pre)
    msgs = []
    if not b.startswith(pb): msgs.append('prefix불일치 실제=%s' % b[:len(pb)].hex())
    e = getattr(i, 'encoding', None)
    io_, is_ = (getattr(e,'imm_offset',0), getattr(e,'imm_size',0)) if e else (0,0)
    do_, ds_ = (getattr(e,'disp_offset',0), getattr(e,'disp_size',0)) if e else (0,0)
    if not ((is_ and off == io_) or (ds_ and off == do_)):
        msgs.append('off%d≠실제 imm@%d(%dB)/disp@%d(%dB)' % (off, io_, is_, do_, ds_))
    if off + w > len(b): msgs.append('off+w>명령길이%d' % len(b))
    if len(pb) != off: msgs.append('prefix길이%d≠off%d' % (len(pb), off))
    v = int.from_bytes(b[off:off+w], 'little') if off+w <= len(b) else None
    if v != orig: msgs.append('값%s≠%s' % (v, orig))
    return (not msgs), ('OK  %-24s %s %s' % (b.hex(), i.mnemonic, i.op_str)) if not msgs \
           else ('NG  ' + ' / '.join(msgs) + '   [%s %s %s]' % (b.hex(), i.mnemonic, i.op_str))

if __name__ == '__main__':
    src = sys.stdin if len(sys.argv) < 2 else io.open(sys.argv[1], encoding='utf-8')
    n = ng = 0
    for ln in src:
        p = ln.split('#')[0].split()
        if len(p) < 5: continue
        rva, pre, off, w, orig = int(p[0],16), p[1], int(p[2]), int(p[3]), int(p[4])
        ok, m = check(rva, pre, off, w, orig)
        n += 1
        if not ok: ng += 1
        print('%06x  %s' % (rva, m))
    print('\n검사 %d / 실패 %d' % (n, ng))
