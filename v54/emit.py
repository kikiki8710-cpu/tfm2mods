# -*- coding: utf-8 -*-
"""주소 목록만 주면 **0.5.4 exe 에서 prefix/imm_off/width/원본값을 직접 뽑아** 코드를 찍는다.

손으로 표를 옮겨 적다가 `imm_off` 를 0.5.3 값으로 물려써 **크래시**를 낸 적이 있다.
그래서 사람이 옮기는 단계를 없앤다 — 주소만 주고 나머지는 exe 가 말하게 한다.

  python emit.py p   ca0725 ca073e ...        # p!(...) 줄 생성 (즉치 자동)
  python emit.py w4  ca0725 ...               # 폭을 4로 강제
"""
import io, sys

sys.path.insert(0, r'C:\tfm2mods\v54')
import reloc as R
import capstone
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
B = 0x140000000
E4 = R.E4
_c = {}


def ins(rva):
    f = E4.func_of(rva)
    if not f:
        return None
    if f[0] not in _c:
        _c[f[0]] = {i.address - B: i for i in R.insns(E4, f[0], f[1])}
    return _c[f[0]].get(rva)


def emit(rva, force_w=0):
    i = ins(rva)
    if i is None:
        return '    // ⚠%06x 명령 시작이 아님' % rva
    e = getattr(i, 'encoding', None)
    io_, is_ = (e.imm_offset, e.imm_size) if e else (0, 0)
    do_, ds_ = (e.disp_offset, e.disp_size) if e else (0, 0)
    if is_:
        off, w = io_, is_
    elif ds_:
        off, w = do_, ds_
    else:
        return '    // ⚠%06x 즉치/변위 없음: %s %s' % (rva, i.mnemonic, i.op_str)
    if force_w:
        w = force_w
    pre = ','.join('0x%02x' % b for b in i.bytes[:off])
    val = int.from_bytes(i.bytes[off:off + w], 'little')
    return ('    p!(base + 0x%06x, &[%s], %d, %d, /*orig %d*/);   // %s %s'
            % (rva, pre, off, w, val, i.mnemonic, i.op_str))


if __name__ == '__main__':
    a = sys.argv[1:]
    fw = 4 if a and a[0] == 'w4' else 0
    if a and a[0] in ('p', 'w4'):
        a = a[1:]
    for x in a:
        print(emit(int(x, 16), fw))
