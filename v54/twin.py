# -*- coding: utf-8 -*-
"""쌍둥이 함수 충돌 해소 — 0.5.3 `cc3340`(ex_skill2_level)·`cc3570`(ex_ult_level)이
둘 다 0.5.4 `ca6700` 에 100% 일치했다. 크기·골격이 같은 제네릭 인스턴스라
골격만으로는 못 가른다. **호출자 문맥**으로 가른다.
"""
import io, sys, collections

sys.path.insert(0, r'C:\tfm2mods\v54')
import reloc as R
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
B = 0x140000000
E3, E4 = R.E3, R.E4


def callers(E, smap, target):
    out = []
    for s, e, src in smap:
        for i in R.insns(E, s, e):
            if i.mnemonic == 'call' and i.op_str.startswith('0x'):
                try:
                    t = int(i.op_str, 16) - B
                except ValueError:
                    continue
                if t == target:
                    out.append((i.address - B, s, src))
    return out


def cmps(E, s, e, needle='0x5b0'):
    return [(i.address - B, i.bytes.hex(), i.mnemonic + ' ' + i.op_str)
            for i in R.insns(E, s, e) if i.mnemonic == 'cmp' and needle in i.op_str]


print('=== 053 쌍둥이 ===')
for fs in (0xcc3340, 0xcc3570):
    f = E3.func_of(fs)
    print(' %06x-%06x (%dB)' % (f[0], f[1], f[1] - f[0]))
    for a, b, t in cmps(E3, *f):
        print('    %06x %-24s %s' % (a, b, t))

print('\n=== 054 cast.rs 계열 480~640B 함수 전수 ===')
cands = []
for s, e, src in R.S4:
    if 'cast.rs' in src and 480 <= e - s <= 640:
        c = cmps(E4, s, e)
        cands.append((s, e, src, c))
        print(' %06x-%06x (%dB) %s' % (s, e, e - s, src[:66]))
        for a, b, t in c:
            print('    %06x %-24s %s' % (a, b, t))

print('\n=== 호출자 대조 ===')
for fs in (0xcc3340, 0xcc3570):
    cs = callers(E3, R.S3, fs)
    print(' 053 %06x ← %s' % (fs, ' , '.join('%06x[%s]' % (a, src.split(chr(92))[-1][:26]) for a, s, src in cs) or '없음'))
for s, e, src, c in cands:
    cs = callers(E4, R.S4, s)
    print(' 054 %06x ← %s' % (s, ' , '.join('%06x[%s]' % (a, sr.split(chr(92))[-1][:26]) for a, ss, sr in cs) or '없음'))
