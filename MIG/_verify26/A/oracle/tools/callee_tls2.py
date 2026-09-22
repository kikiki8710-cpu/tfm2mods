# -*- coding: utf-8 -*-
"""26A: 본문 37862~46714 의 call/invoke 대상 심볼 전수 → 각 define 본문의 TLS 접점(1단계) 스캔."""
import io, re, sys, glob, os, collections
sys.stdout.reconfigure(encoding='utf-8')
L = io.open(r'C:\tfm2mods\_gaibc\m07.ll', encoding='utf-8', errors='replace').read().split('\n')
body = L[37861:46714]
syms = collections.Counter()
for s in body:
    for m in re.findall(r'(?:call|invoke)[^@\n]*@(_R[A-Za-z0-9_.]+)\(', s):
        syms[m] += 1
print('직접 호출 심볼', len(syms))
files = sorted(glob.glob(r'C:\tfm2mods\_gaibc\m*.ll')) + sorted(glob.glob(r'C:\tfm2mods\_gcbc\g*.ll'))
want = {x: None for x in syms}
for f in files:
    txt = io.open(f, encoding='utf-8', errors='replace').read()
    for x in list(want):
        if want[x] is not None: continue
        k = txt.find('\ndefine ')
        key = '@' + x + '('
        pos = 0
        while True:
            k = txt.find(key, pos)
            if k == -1: break
            ls = txt.rfind('\n', 0, k) + 1
            if txt.startswith('define', ls):
                e = txt.find('\n}\n', k)
                b = txt[ls:e]
                hits = [w for w in ('threadlocal', 'LocalKey', 'call_once', '__RUST_STD_INTERNAL_VAL') if w in b]
                want[x] = (os.path.basename(f), txt.count('\n', 0, ls) + 1, hits, b.count('\n'))
                break
            pos = k + 1
    if all(v is not None for v in want.values()): break
for x, v in sorted(want.items(), key=lambda kv: (kv[1] is None, str(kv[1]))):
    print(syms[x], v, x[:120])
