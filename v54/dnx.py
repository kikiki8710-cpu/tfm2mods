# -*- coding: utf-8 -*-
"""defense_nexus 3항 판정기(c797c0) 전용 조사 도구.

  python dnx.py sites 054 c797c0        # 모든 콜사이트의 인자 셋업(직전 40개 명령) + 반환 소비(직후 30개)
  python dnx.py disp 054 170 8          # [base + idx*scale + disp32] 형태로 disp 를 쓰는 명령 전수 (구조체 필드 역추적)
  python dnx.py imm  054 <lo> <hi>      # 함수 구간의 즉시값 전수 (노브 후보)
"""
import io, os, re, struct, sys, bisect, collections
sys.path.insert(0, r'C:\tfm2mods\v54')
from pe2 import load, BASE
from scan import Scanner, src_of
# ⚠stdout 재래핑 금지: scan.py 가 이미 utf-8 로 감쌌다. 다시 감싸면 옛 래퍼 GC 가 buffer 를 닫는다.
import capstone


def sites(ver, tgt):
    S = Scanner(ver)
    body, tva = S.body, S.tva
    out = []
    for m in re.finditer(re.escape(b'\xe8'), body):
        o = m.start()
        if o + 5 > len(body):
            continue
        d = struct.unpack_from('<i', body, o + 1)[0]
        if tva + o + 5 + d == tgt:
            out.append(tva + o)
    for site in out:
        f = S.func_of(site)
        src = src_of(ver, f[0])[0] if f else '?'
        print('=' * 100)
        print('CALLSITE %06x  in fn %06x  %s' % (site, f[0] if f else 0, (src or '')[:90]))
        ins = S.disf(f)
        idx = [k for k, i in enumerate(ins) if i.address - BASE == site]
        if not idx:
            print('  (align mismatch)')
            continue
        k = idx[0]
        for i in ins[max(0, k - 26):k + 26]:
            a = i.address - BASE
            mark = '>>' if a == site else '  '
            print('  %s %06x  %-20s %s %s' % (mark, a, i.bytes.hex()[:20], i.mnemonic, i.op_str))


def disp(ver, d, scale):
    """[base + idx*scale + disp32] 를 읽는 명령 전수 — 구조체 배열 필드 역추적용."""
    S = Scanner(ver)
    body, tva = S.body, S.tva
    pat = struct.pack('<i', d)
    sc = {1: 0, 2: 1, 4: 2, 8: 3}[scale]
    seen = collections.OrderedDict()
    for m in re.finditer(re.escape(pat), body):
        rva = tva + m.start()
        f = S.func_of(rva)
        if not f:
            continue
        for i in S.disf(f):
            a = i.address - BASE
            if not (a <= rva < a + i.size):
                continue
            for op in i.operands:
                if op.type == capstone.x86.X86_OP_MEM and op.mem.disp == d \
                        and op.mem.index != 0 and op.mem.scale == scale:
                    seen[a] = (f, i)
            break
    print('== disp 0x%x scale %d : %d hits' % (d, scale, len(seen)))
    for a, (f, i) in seen.items():
        print('  %06x [fn %06x] %-70s | %s' % (a, f[0], (src_of(ver, f[0])[0] or '')[:70],
                                               i.mnemonic + ' ' + i.op_str))


def imm(ver, lo, hi):
    S = Scanner(ver)
    f = S.func_of(lo)
    for i in S.disf(f):
        a = i.address - BASE
        if not (lo <= a < hi):
            continue
        for oi, op in enumerate(i.operands):
            if op.type == capstone.x86.X86_OP_IMM:
                v = op.imm
                if i.mnemonic in ('call', 'jmp') or i.mnemonic.startswith('j'):
                    continue
                b = i.bytes
                enc = struct.pack('<Q', v & 0xFFFFFFFFFFFFFFFF)
                off = w = None
                for width in (8, 4, 2, 1):
                    e = enc[:width]
                    p = b.rfind(e)
                    if p >= 0 and (width == 8 or (v >= 0 and v < (1 << (8 * width))) or
                                   (-(1 << (8 * width - 1)) <= v < (1 << (8 * width - 1)))):
                        off, w = p, width
                        break
                print('%06x  %-22s %-6s %-34s imm=%d(0x%x) off=%s w=%s'
                      % (a, b.hex(), i.mnemonic, i.op_str, v, v & 0xFFFFFFFFFFFFFFFF, off, w))


if __name__ == '__main__':
    c = sys.argv[1]
    if c == 'sites':
        sites(sys.argv[2], int(sys.argv[3], 16))
    elif c == 'disp':
        disp(sys.argv[2], int(sys.argv[3], 16), int(sys.argv[4]))
    elif c == 'imm':
        imm(sys.argv[2], int(sys.argv[3], 16), int(sys.argv[4], 16))
