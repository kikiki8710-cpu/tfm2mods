# -*- coding: utf-8 -*-
"""바이트 시그니처(공백구분 hex, ?? 와일드카드) 를 .text 전역에서 찾아 함수/소스로 귀속.
  python sig.py 054 "4d 8b 90 58 06 00 00 41 b9 07 ?? ?? ??"
"""
import io, os, re, sys, bisect
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
sys.path.insert(0, r'C:\tfm2mods\v54')
from pe2 import load
D = r'C:\tfm2mods\v54'
def srcmap2(ver):
    rows = []
    for ln in io.open(os.path.join(D, '%s_srcmap2.tsv' % ver), encoding='utf-8'):
        s, e, src, l = ln.rstrip('\n').split('\t'); rows.append((int(s,16), int(e,16), src))
    rows.sort(); return rows
ver, sg = sys.argv[1], sys.argv[2]
e = load(ver); sm = srcmap2(ver); ks = [r[0] for r in sm]
_, tva, tvsz, tra, trsz = [s for s in e.sections if s[0] == '.text'][0]
body = e.raw[tra:tra+trsz]
pat = b''.join(b'.' if t in ('??','?') else re.escape(bytes([int(t,16)])) for t in sg.split())
n = 0
for m in re.finditer(pat, body, re.S):
    rva = tva + m.start()
    i = bisect.bisect_right(ks, rva) - 1
    src = sm[i][2] if i >= 0 and sm[i][0] <= rva < sm[i][1] else '?'
    f = e.func_of(rva)
    print('%06x  fn %s  %s' % (rva, ('%06x' % f[0]) if f else '?', src[:100]))
    n += 1
print('-- %d건' % n)
