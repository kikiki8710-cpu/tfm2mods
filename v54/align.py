# -*- coding: utf-8 -*-
"""★align.py — 명령열 **문맥 정렬**로 0.5.3 사이트를 0.5.4로 옮긴다.

왜 필요한가: reloc/sweep 은 "같은 모양 명령이 함수 안에 몇 개인가"를 셌다.
함수 본문이 바뀌면(골격 60~90%) 개수가 어긋나 전부 미확정이 된다.
이 도구는 개수 대신 **위치**를 쓴다 — 두 함수의 명령열을 difflib 로 정렬해
053 의 i번째 명령이 054 의 몇 번째에 대응하는지 구하고, 그 자리 주변 문맥을 보여준다.

정렬 토큰(레벨):
  L1 = 니모닉 + 피연산자 구조(레지스터 유지, 상수/변위는 K 로 가림)  ← 기본
  L2 = 니모닉 + 피연산자 타입(레지스터도 가림)                        ← 레지스터 변경 대비

판정:
  equal 블록 안 → 1:1 대응 (확신 상)
  replace/insert 블록 안 → 앞뒤 equal 앵커로 구간을 좁히고 후보만 제시 (사람이 확정)

⚠stdout 재래핑 금지(reloc/sweep 이 이미 감쌈).
"""
import io, os, re, sys, difflib, collections

sys.path.insert(0, r'C:\tfm2mods\v54')
import reloc as R
import capstone

B = 0x140000000
E3, E4 = R.E3, R.E4
_K = re.compile(r'0x[0-9a-f]+')
_R = re.compile(r'\b(r[a-z0-9]{1,3}|e[a-z]{2}|[a-d][lh]|[sd]il|[sb]pl|xmm\d+)\b')


def t1(i):
    return i.mnemonic + '|' + _K.sub('K', i.op_str)


def t2(i):
    return i.mnemonic + '|' + _R.sub('R', _K.sub('K', i.op_str))


def enc(i):
    """(imm_off, imm_size, disp_off, disp_size)"""
    e = getattr(i, 'encoding', None)
    if not e:
        return (0, 0, 0, 0)
    return (getattr(e, 'imm_offset', 0), getattr(e, 'imm_size', 0),
            getattr(e, 'disp_offset', 0), getattr(e, 'disp_size', 0))


def site_desc(i):
    """이 명령의 패치 슬롯 = (off, w, 값). 즉치 우선, 없으면 변위."""
    io_, is_, do_, ds_ = enc(i)
    if is_:
        return (io_, is_, int.from_bytes(i.bytes[io_:io_ + is_], 'little', signed=False))
    if ds_:
        return (do_, ds_, int.from_bytes(i.bytes[do_:do_ + ds_], 'little', signed=False))
    return (0, 0, 0)


class Pair:
    def __init__(self, fs, fe, bs, be, ratio):
        self.f3, self.f4, self.ratio = (fs, fe), (bs, be), ratio
        self.a = R.insns(E3, fs, fe)
        self.b = R.insns(E4, bs, be)
        self.ia = {x.address - B: k for k, x in enumerate(self.a)}
        self.sm1 = difflib.SequenceMatcher(None, [t1(x) for x in self.a],
                                           [t1(x) for x in self.b], autojunk=False)
        self.sm2 = difflib.SequenceMatcher(None, [t2(x) for x in self.a],
                                           [t2(x) for x in self.b], autojunk=False)
        self.op1 = self.sm1.get_opcodes()
        self.op2 = self.sm2.get_opcodes()

    def map_idx(self, i, ops):
        """053 index → (054 index or None, 판정, 구간)"""
        for tag, a1, a2, b1, b2 in ops:
            if a1 <= i < a2:
                if tag == 'equal':
                    return (b1 + (i - a1), 'equal', (b1, b2))
                return (None, tag, (b1, b2))
        return (None, 'oob', (0, 0))

    def bracket(self, i, ops):
        """i를 감싸는 앞·뒤 equal 앵커의 054 index 범위."""
        lo, hi = 0, len(self.b)
        for tag, a1, a2, b1, b2 in ops:
            if tag == 'equal':
                if a2 <= i:
                    lo = b2
                elif a1 > i:
                    hi = b1
                    break
        return lo, hi


_pc = {}


def pair_of(rva):
    f = E3.func_of(rva)
    if not f:
        return None
    if f in _pc:
        return _pc[f]
    pr = R.pair_fn(f[0], f[1])
    _pc[f] = Pair(f[0], f[1], pr[0], pr[1], pr[2]) if pr else None
    return _pc[f]


def show(rva, ctx=4):
    P = pair_of(rva)
    print('=== 053 %06x ===' % rva)
    if not P:
        print('  짝 함수 없음')
        return
    print('  fn3 %06x-%06x → fn4 %06x-%06x (골격 %.0f%%)'
          % (P.f3[0], P.f3[1], P.f4[0], P.f4[1], P.ratio * 100))
    i = P.ia.get(rva)
    if i is None:
        print('  ⚠053 명령 경계 아님')
        return
    ins = P.a[i]
    o, w, v = site_desc(ins)
    print('  053: %-22s %-34s slot off=%d w=%d val=%d(0x%x)'
          % (ins.bytes.hex(), ins.mnemonic + ' ' + ins.op_str, o, w, v, v))
    print('  053 문맥:')
    for k in range(max(0, i - ctx), min(len(P.a), i + ctx + 1)):
        x = P.a[k]
        print('   %s %06x %-20s %s %s' % ('>' if k == i else ' ', x.address - B,
                                          x.bytes.hex()[:20], x.mnemonic, x.op_str))
    for nm, ops in (('L1', P.op1), ('L2', P.op2)):
        j, tag, rng = P.map_idx(i, ops)
        if j is not None:
            y = P.b[j]
            oo, ww, vv = site_desc(y)
            print('  %s 정렬: equal → 054 %06x  %-22s %-34s slot off=%d w=%d val=%d(0x%x)'
                  % (nm, y.address - B, y.bytes.hex(), y.mnemonic + ' ' + y.op_str, oo, ww, vv, vv))
            print('     054 문맥:')
            for k in range(max(0, j - ctx), min(len(P.b), j + ctx + 1)):
                x = P.b[k]
                print('      %s %06x %-20s %s %s' % ('>' if k == j else ' ', x.address - B,
                                                     x.bytes.hex()[:20], x.mnemonic, x.op_str))
            return
        else:
            lo, hi = P.bracket(i, ops)
            print('  %s 정렬: %s 블록 — 054 후보구간 [%06x .. %06x] (%d명령)'
                  % (nm, tag, P.b[lo].address - B if lo < len(P.b) else 0,
                     P.b[min(hi, len(P.b) - 1)].address - B, hi - lo))
            cands = [(k, P.b[k]) for k in range(lo, min(hi, len(P.b)))
                     if P.b[k].mnemonic == ins.mnemonic]
            for k, y in cands[:16]:
                oo, ww, vv = site_desc(y)
                mark = '★' if vv == v else ' '
                print('     %s %06x %-22s %-34s off=%d w=%d val=%d(0x%x)'
                      % (mark, y.address - B, y.bytes.hex(), y.mnemonic + ' ' + y.op_str, oo, ww, vv, vv))
            if not cands:
                print('     (같은 니모닉 후보 없음)')


if __name__ == '__main__':
    # reloc 은 import 시 stdout 을 감싸지 않는다(main 에서만) → 여기서 1회만 감싼다.
    sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
    for a in sys.argv[1:]:
        show(int(a, 16))
        print()
