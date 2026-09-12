# -*- coding: utf-8 -*-
u"""`13 knobs[7]` 의 「부시 ID 그리드」(@anon….269, _gcbc/g07.ll:275) 상수를 디코드한다. (9차 배치C)

값 칸이 비어 있던 행이라 **무엇을 담을 수 있는지** 재려고 만들었다.
`[7200 x i8]` 이 정말 u64 격자인지, 몇 × 몇인지, 어떤 ID 가 나오는지를 IR 원문에서 직접 센다.
"""
import collections, io, math, re, struct, sys

sys.stdout.reconfigure(encoding="utf-8", errors="replace")
ln = io.open(r"C:\tfm2mods\_gcbc\g07.ll", encoding="utf-8", errors="replace").readlines()[274]
m = re.search(r'c"(.*)"', ln)
s = m.group(1)
out = bytearray()
i = 0
BS = chr(92)
while i < len(s):
    if s[i] == BS:
        out.append(int(s[i + 1:i + 3], 16))
        i += 3
    else:
        out.append(ord(s[i]))
        i += 1
print(u"bytes = %d" % len(out))
vals = [struct.unpack_from("<Q", out, k * 8)[0] for k in range(len(out) // 8)]
print(u"u64 개수 = %d" % len(vals))
c = collections.Counter(vals)
print(u"출현 ID = %s" % sorted(c))
print(u"ID별 칸수 = %s" % sorted(c.items()))
n = len(vals)
print(u"정사각 = %s (변 %d)" % (math.isqrt(n) ** 2 == n, math.isqrt(n)))
for d in (27, 30, 25, 24, 36):
    if n % d == 0:
        print(u"  %d × %d 도 가능" % (d, n // d))
