# -*- coding: utf-8 -*-
"""미확정 사이트 하나를 **손으로 판단할 수 있게** 펼쳐 본다.

  python why.py c8689a          # 053 사이트 하나
자동 판정이 실패한 이유(값이 바뀌었나·개수가 달라졌나·오프셋이 밀렸나)를
직접 눈으로 확인하기 위한 도구. **여기서 나온 후보를 자동으로 확정하지 않는다.**
"""
import io, os, sys, collections

sys.path.insert(0, r'C:\tfm2mods\v54')
import reloc as R
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
B = 0x140000000
E3, E4 = R.E3, R.E4


def go(rva):
    f = E3.func_of(rva)
    print('053 사이트 %06x' % rva)
    if not f:
        print('  .pdata 함수 없음')
        return
    print('  함수 %06x-%06x  src=%s' % (f[0], f[1], R.SRC3.get(f[0], '(없음)')))
    i3 = {i.address - B: i for i in R.insns(E3, f[0], f[1])}
    ins = i3.get(rva)
    if ins is None:
        print('  ⚠명령 경계 아님')
        return
    print('  053 명령: %-24s %s %s' % (ins.bytes.hex(), ins.mnemonic, ins.op_str))
    n3 = [a for a, y in sorted(i3.items()) if y.bytes == ins.bytes]
    print('  이 함수 안에서 같은 바이트 %d곳: %s' % (len(n3), ' '.join('%06x' % a for a in n3)))

    pr = R.pair_fn(f[0], f[1])
    if not pr:
        print('  ★0.5.4 짝을 못 찾음 (소스 앵커 없음) — 호출자/콜리로 찾아야 함')
        return
    bs, be, ratio = pr
    print('  054 짝 %06x-%06x (골격 %.0f%%)' % (bs, be, ratio * 100))
    i4 = R.insns(E4, bs, be)
    exact = [y for y in i4 if y.bytes == ins.bytes]
    same_len = [y for y in i4 if y.mnemonic == ins.mnemonic and len(y.bytes) == len(ins.bytes)]
    print('  054에서 바이트 완전일치 %d곳: %s' % (len(exact), ' '.join('%06x' % (y.address - B) for y in exact)))
    print('  054에서 같은 니모닉·길이 %d곳:' % len(same_len))
    for y in same_len[:14]:
        print('      %06x  %-24s %s %s' % (y.address - B, y.bytes.hex(), y.mnemonic, y.op_str))


if __name__ == '__main__':
    for a in sys.argv[1:]:
        go(int(a, 16))
        print()
