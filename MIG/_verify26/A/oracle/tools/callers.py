# -*- coding: utf-8 -*-
"""26A open[7]: 호출처 3곳의 함수 이름 + 호출 뒤 sret(player.action +0x918 / wave_snapshot +0x0 / v3_turnback_hold +0x1500) 에 store 가 있는지."""
import io, re, sys
sys.stdout.reconfigure(encoding='utf-8')
for fn, ln in [('m13.ll', 45850), ('m15.ll', 23147), ('m15.ll', 56921)]:
    L = io.open(r'C:\tfm2mods\_gaibc\\' + fn, encoding='utf-8', errors='replace').read().split('\n')
    # enclosing define
    i = ln - 1
    while i >= 0 and not L[i].startswith('define'): i -= 1
    name = re.search(r'@(\S+)\(', L[i]).group(1)
    j = ln
    while j < len(L) and L[j] != '}': j += 1
    body = L[i:j]
    m = re.search(r'sret\(\[5384 x i8\]\)[^%]*(%\d+)', L[ln-1])
    sret = m.group(1)
    print('==', fn, ln, 'define@', i+1, name[:120], 'sret', sret, '본문줄', j - i)
    # sret 파생 gep 와 store
    off = {sret: 0}
    for k in range(3):
        for s in body:
            mm = re.match(r'\s*(%\d+) = getelementptr inbounds(?: nuw)? i8, ptr (%\d+), i64 (\d+)', s)
            if mm and mm.group(2) in off: off[mm.group(1)] = off[mm.group(2)] + int(mm.group(3))
    for s in body:
        mm = re.match(r'\s*store (\w+) ([^,]+), ptr (%\d+)', s)
        if mm and mm.group(3) in off:
            print('   store +0x%x <- %s %s' % (off[mm.group(3)], mm.group(1), mm.group(2)[:40]), '|', s.strip()[:120])
        mm = re.search(r'memcpy\.p0\.p0\.i64\(ptr [^%]*(%\d+), ptr [^%]*(%\d+), i64 (\d+)', s)
        if mm and (mm.group(1) in off or mm.group(2) in off):
            print('   memcpy', ('dst+0x%x' % off[mm.group(1)]) if mm.group(1) in off else 'dst?', ('src+0x%x' % off[mm.group(2)]) if mm.group(2) in off else 'src?', mm.group(3), '|', s.strip()[:100])
    # 2328 gep 이 있는가
    for s in body:
        if re.search(r'i64 2328\b', s) : print('   2328:', s.strip()[:150])
