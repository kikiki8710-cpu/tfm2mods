# -*- coding: utf-8 -*-
"""두 판의 함수 본문을 **정규화 비교** — '같은 코드인가'를 바이트 이동과 무관하게 판정.

정규화: call/jmp 의 절대 타깃 → <T>, rip-상대 변위 → <R>.
  ⚠구조체 오프셋([rdx+0x810] 등)은 **일부러 남긴다** — 그게 바뀌면 알아야 하니까.
  python bodydiff.py <rva053> <rva054>
"""
import io, re, sys, difflib
if __name__ == '__main__':
    sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
sys.path.insert(0, r'C:\tfm2mods\v54')
from pe2 import load, BASE
from jtscan import fast_func_of

RIP = re.compile(r'rip \+ 0x[0-9a-f]+')
ABS = re.compile(r'^0x1[0-9a-f]{8}$')


def toks(e, s, en):
    out = []
    for i in e.dis(s, en - s):
        o = RIP.sub('rip+<R>', i.op_str)
        if i.mnemonic in ('call', 'jmp') or i.mnemonic.startswith('j'):
            if ABS.match(o):
                o = '<T>'
        out.append(i.mnemonic + ' ' + o)
    return out


def cmp2(a_rva, b_rva, verbose=True):
    ea, eb = load('053'), load('054')
    fa, fb = fast_func_of(ea, a_rva), fast_func_of(eb, b_rva)
    ta, tb = toks(ea, *fa), toks(eb, *fb)
    r = difflib.SequenceMatcher(None, ta, tb).ratio()
    if verbose:
        print('053 %06x-%06x (%dB, %d ins)  ↔  054 %06x-%06x (%dB, %d ins)  유사도 %.4f' %
              (fa[0], fa[1], fa[1] - fa[0], len(ta), fb[0], fb[1], fb[1] - fb[0], len(tb), r))
        if r < 1.0:
            sm = difflib.SequenceMatcher(None, ta, tb)
            n = 0
            for tag, i1, i2, j1, j2 in sm.get_opcodes():
                if tag == 'equal':
                    continue
                n += 1
                if n > 40:
                    print('   ... (이하 생략)'); break
                print('  [%s] 053[%d:%d] %s' % (tag, i1, i2, ta[i1:min(i2, i1 + 4)]))
                print('        054[%d:%d] %s' % (j1, j2, tb[j1:min(j2, j1 + 4)]))
    return r


if __name__ == '__main__':
    cmp2(int(sys.argv[1], 16), int(sys.argv[2], 16))
