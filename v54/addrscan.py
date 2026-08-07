# -*- coding: utf-8 -*-
"""★addrscan.py — **매크로 형태를 전혀 가정하지 않는** 주소 수준 죽음 판정.

파서(sites.py/sites2.py)는 `p!`/`pany!`/`patch_imm_bytes` 만 본다. 실제 소스엔
`pskip!`·`pm!`·`pmulti!`·튜플루프도 있어 그만큼 사각지대가 생긴다(실측: STGATE 3사이트가
주소는 맞는데 prefix(rdi→rsi)가 어긋나 전부 조용히 죽어 있었다).

이 스크립트는 **소스의 모든 RVA 리터럴**(0xC00000~0x2400000, 주석 제외)을 뽑아
0.5.4 exe 에서 **.pdata 함수 안 명령 시작인가**만 본다.
  · NG(명령경계아님) = 확실히 죽음(그 주소로는 아무것도 못 한다)
  · OK = 주소는 살아 있음 → prefix/off 는 사람이 확인(054 실명령을 같이 출력)
0.5.3 exe 에서만 명령 시작인 것은 "미이전 053 주소"로 따로 표시한다.
"""
import io, os, re, sys, bisect, collections
sys.path.insert(0, r'C:\tfm2mods\v54')
import reloc as R
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
B = 0x140000000
SRCDIR = r'C:\tfm2mods\tfm2_ai_adjust\src'
HEX = re.compile(r'0x([0-9a-fA-F]{5,7})\b')

def mk(E):
    fn = E.funcs(); fs = [s for s, e in fn]
    cache = {}
    def f_of(rva):
        k = bisect.bisect_right(fs, rva) - 1
        if k < 0: return None
        s, e = fn[k]
        return (s, e) if s <= rva < e else None
    def ins(rva):
        f = f_of(rva)
        if not f: return None
        if f[0] not in cache:
            cache[f[0]] = {i.address - B: i for i in R.insns(E, f[0], f[1])}
        return cache[f[0]].get(rva)
    return ins

I3, I4 = mk(R.E3), mk(R.E4)

if __name__ == '__main__':
    files = sys.argv[1:] or ['detour.rs', 'disc19_repro.rs', 'serpen.rs']
    cnt = collections.Counter()
    for fn in files:
        p = os.path.join(SRCDIR, fn)
        if not os.path.exists(p): continue
        for ln, line in enumerate(io.open(p, encoding='utf-8'), 1):
            code = line.split('//')[0]
            if 'base +' not in code and 'usize' not in code and '0x' not in code: continue
            for m in HEX.finditer(code):
                a = int(m.group(1), 16)
                if not (0xc00000 <= a <= 0x2400000): continue
                i4, i3 = I4(a), I3(a)
                if i4 is None:
                    cnt['NG'] += 1
                    print('NG   %s:%-5d %06x  054에 명령없음  %s'
                          % (fn, ln, a, '(053에는 있음=미이전 053주소)' if i3 else ''))
                else:
                    cnt['OK'] += 1
    print('\n주소 리터럴 판정: 054명령시작 %d / 죽음 %d' % (cnt['OK'], cnt['NG']))
