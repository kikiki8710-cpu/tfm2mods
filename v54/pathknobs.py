# -*- coding: utf-8 -*-
"""pathknobs.py - 0.5.4 경로/거리 시스템 노브 사이트를 **exe 에서 직접** 재생성한다.
   (손으로 표를 옮겨 적다 imm_off 를 틀려 크래시 낸 전례가 있어, 사람이 옮기는 단계를 없앤다.)

  python pathknobs.py 054 <그룹>
    risk      db05f0(tower_discipline) 위협원 페널티 산식 5상수
    conv      위험도(칸) -> A* 비용(=칸*640+1) 변환 35곳 (shl7 / lea*5 / inc)
    heur      A* 휴리스틱 배율 shl r,7 (=x128) 54곳
    step640   A* 직교 1칸 비용 640
    step896   A* 대각 1칸 비용 896
    risk1281  위험셀 가산 1281
    fdist     free_dist 본체 간선비용(직교 lea+5 4곳 / 대각 add 7 1곳)
    fdstep    free_dist 소비자 스텝비용(직교 imm 75곳 / 대각 SIB스케일 45곳)
    pos       적 위치추정 임계 14지점 3인코딩 + 속도모델(계수/3000/100) 각 6곳
    dirs      8방향 오프셋표 4사본(.rdata) 참조 분포
  전부: python pathknobs.py 054 all
"""
import sys, collections, struct
sys.path.insert(0, r'C:\tfm2mods\v54')
import cen, capstone
from pe2 import BASE, load
X = capstone.x86

PATH_SRC = ('free_dist', 'path_field', 'path_finder', 'small_action')
DIRS = (0x3271790, 0x3271b08, 0x3282e80, 0x32ce500)
POS_FN = (0xd2ff50, 0xd31590, 0xd31760, 0xd320a0, 0xc8c210, 0x141e000)


def is32(i):
    for b in i.bytes:
        if 0x40 <= b <= 0x4f:
            return not (b & 8)
        break
    return True


def walk(ver, srcpats=PATH_SRC):
    S = cen.sc(ver); M = cen.sm(ver)
    for f in S.funcs:
        src = M.get(f[0], '(nosrc)')
        if srcpats and not any(p in src for p in srcpats):
            continue
        yield f, src, list(S.disf(f))
        S._dis.clear()


_ic = {}


def _ins(ver, rva):
    S = cen.sc(ver)
    f = S.func_of(rva)
    if not f:
        return None
    if f[0] not in _ic:
        _ic[f[0]] = {i.address - BASE: i for i in S.disf(f)}
        S._dis.clear()
    return _ic[f[0]].get(rva)


def p(rva, ver='054'):
    """emit.py 와 동일 규격 — exe 에서 prefix/imm_off/width/원본값을 직접 뽑는다.
       (emit.py 는 import 시 stdout 을 감싸므로 여기선 재구현. scan.py 와 이중 래핑 금지.)"""
    i = _ins(ver, rva)
    if i is None:
        print('    // WARN %06x 명령 시작 아님' % rva)
        return
    e = i.encoding
    off, w = (e.imm_offset, e.imm_size) if e.imm_size else (e.disp_offset, e.disp_size)
    if not w:
        print('    // WARN %06x 즉치/변위 없음: %s %s' % (rva, i.mnemonic, i.op_str))
        return
    pre = ','.join('0x%02x' % b for b in i.bytes[:off])
    val = int.from_bytes(i.bytes[off:off + w], 'little')
    print('    p!(base + 0x%06x, &[%s], %d, %d, /*orig %d*/);   // %s %s'
          % (rva, pre, off, w, val, i.mnemonic, i.op_str))


def g_risk(ver):
    print('# [risk] 위협원 페널티 = clamp(2 + 30*(1칸 이동중 받을피해)/HP, 2, 60) 칸  @ db05f0 tower_discipline.rs:308')
    for a in (0xdb05fc, 0xdb0745, 0xdb07cb, 0xdb07ce, 0xdb07d1, 0xdb077e, 0xdb07b2):
        p(a)
    print('#   db05fc/db0745 = 조기반환 기본 2칸 / db07cb = 하한 +2칸(imm8) / db07ce+db07d1 = 상한 60칸(cmp는 imm8!) / db077e+db07b2 = 민감도 x30')


def _shl7(ver):
    heur, conv = [], []
    for f, src, ins in walk(ver, PATH_SRC + ('local.rs',)):
        for k, i in enumerate(ins):
            if i.mnemonic != 'shl' or not is32(i):
                continue
            o = i.operands
            if o[1].type != X.X86_OP_IMM or o[1].imm != 7:
                continue
            ctx = ' ; '.join('%s %s' % (x.mnemonic, x.op_str) for x in ins[max(0, k - 4):k])
            if 'movzx' in ctx and 'word' in ctx:
                heur.append((i.address - BASE, f[0], src))
            elif '0x64' in ctx:
                conv.append((i.address - BASE, f[0], src))
    return heur, conv


def g_heur(ver):
    h, _ = _shl7(ver)
    print('# [heur] A* 휴리스틱 배율 shl r32,7 (=x128) : %d곳' % len(h))
    for a, _, _ in h:
        p(a)


def g_conv(ver):
    _, c = _shl7(ver)
    print('# [conv] 위험도(칸)->비용 변환 shl r32,7 : %d곳 (뒤이어 lea r,[r+r*4] = x5, inc r = +1)' % len(c))
    for a, _, _ in c:
        p(a)


def g_imm(ver, val, tag):
    by = cen.sites(ver, val, 4)
    tot = 0
    print('# [%s] imm %d' % (tag, val))
    for src in sorted(by, key=lambda k: -len(by[k])):
        if not any(q in src for q in PATH_SRC):
            continue
        for a, fs, i in by[src]:
            p(a); tot += 1
    print('#   경로계열 합계 %d곳' % tot)


def g_fdist(ver):
    print('# [fdist] free_dist 본체 dfbd40 간선비용')
    for a in (0xdfbf21, 0xdfbff3, 0xdfc0cb, 0xdfc1a2):
        p(a)
    p(0xdfc278)
    print('#   앞 4개 = 직교 5 (lea disp8, 1..127) / 마지막 = 대각 7 (imm8)')


def g_fdstep(ver):
    orth, diag = [], []
    for f, src, ins in walk(ver):
        for k, i in enumerate(ins):
            if i.mnemonic == 'add' and is32(i):
                o = i.operands
                if o[0].type == X.X86_OP_REG and o[1].type == X.X86_OP_IMM and o[1].imm == 5:
                    orth.append((i.address - BASE, 'add', f[0], src))
            if i.mnemonic == 'lea' and is32(i):
                m = i.operands[1].mem
                if m.index and m.base and m.scale >= 2:
                    if m.disp == 5:
                        orth.append((i.address - BASE, 'lea-fused', f[0], src))
                    if k >= 2 and ins[k - 2].mnemonic == 'xor' and ins[k - 1].mnemonic == 'movzx':
                        diag.append((i.address - BASE, m.scale, f[0], src,
                                     i.encoding.modrm_offset + 1))
    print('# [fdstep] free_dist 소비자 직교비용 %d곳 / 대각 SIB스케일 %d곳' % (len(orth), len(diag)))
    for a, kind, fs, src in sorted(orth):
        print('  ORTH %06x %-10s fn %06x %s' % (a, kind, fs, src[:44]))
        p(a)
    for a, sc, fs, src, sib in sorted(diag):
        print('  DIAG %06x SIB스케일=%d (SIB바이트오프셋=%d, bit6-7만 교체) fn %06x %s'
              % (a, sc, sib, fs, src[:44]))


def g_pos(ver):
    print('# [pos] 적 위치추정 — 판정: (d + 40000) <= 300000  ⟺ d <= 260000')
    print('# 인코딩1 리터럴(1지점)')
    for a in (0xd316cf, 0xd316d5):
        p(a)
    print('# 인코딩2 lea disp + cmp 300000 (2지점, team_plan)')
    for a in (0xc8c586, 0xc8c58d, 0xc8c613, 0xc8c61a):
        p(a)
    print('# 인코딩3 융합 lea -260001 + cmp -300001 (11지점)')
    for a in (0xd3028b, 0xd30292, 0xd3034e, 0xd30355, 0xd31959, 0xd31960,
              0xd31a3a, 0xd31a41, 0xd32219, 0xd32220, 0xd32300, 0xd32307,
              0x141e852, 0x141e858, 0x141e8e2, 0x141e8e8, 0x141e904, 0x141e90a,
              0x141e999, 0x141e99f, 0x141e9af, 0x141e9b5):
        p(a)
    print('# 확산속도 모델: 계수 / +3000 / 100상한 (각 6곳)')
    for a in (0xd300d0, 0xd31693, 0xd3189a, 0xd321b5, 0xc8c54f, 0x141e7e7,
              0xd300e2, 0xd316a5, 0xd318ac, 0xd321c7, 0xc8c561, 0x141e7f9,
              0xd300c0, 0xd31683, 0xd31888, 0xd321a5, 0xc8c536, 0x141e7c2):
        p(a)


def g_dirs(ver):
    e = load(ver)
    print('# [dirs] 8방향 오프셋표 .rdata 4사본 (각 64B, 내용 동일)')
    for a in DIRS:
        b = e.rd(a, 64)
        vs = struct.unpack('<16i', b)
        print('  %07x  %s' % (a, list(zip(vs[0::2], vs[1::2]))))
    print('#   대각 4쌍 = 오프셋 0x20..0x3f. ⚠오프셋 0x00..0x0f 은 무관 함수(2d5f6a0 por xmm0)가 SSE 상수로 공유 → 건드리지 말 것')


G = dict(risk=g_risk, heur=g_heur, conv=g_conv, fdist=g_fdist, fdstep=g_fdstep,
         pos=g_pos, dirs=g_dirs,
         step640=lambda v: g_imm(v, 640, 'step640'),
         step896=lambda v: g_imm(v, 896, 'step896'),
         risk1281=lambda v: g_imm(v, 1281, 'risk1281'))

if __name__ == '__main__':
    ver = sys.argv[1]
    which = sys.argv[2] if len(sys.argv) > 2 else 'all'
    for k in (list(G) if which == 'all' else [which]):
        print('=' * 70)
        G[k](ver)
