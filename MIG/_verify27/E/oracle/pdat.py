# -*- coding: utf-8 -*-
"""pdat.py <rva_hex> — .pdata 에서 rva 를 감싸는 함수(begin,end) 와 그 주변 항목 출력"""
import sys, struct
import pefile
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
EXE = r"C:\Users\jungs\Desktop\claude\tfm2\tfm2_0.5.8\TeamfightManager2.exe"
pe = pefile.PE(EXE, fast_load=True)
want = int(sys.argv[1], 16)
for s in pe.sections:
    if s.Name.rstrip(b"\0") == b".pdata":
        d = s.get_data()
        ents = [struct.unpack_from("<III", d, i)[:2] for i in range(0, len(d) - 11, 12)]
        ents.sort()
        for k, (b, e) in enumerate(ents):
            if b <= want < e:
                print("contains: %#x..%#x (%d B)" % (b, e, e - b))
                for j in range(max(0, k - 2), min(len(ents), k + 3)):
                    print("   %#x..%#x (%d B)" % (ents[j][0], ents[j][1], ents[j][1] - ents[j][0]))
                break
        else:
            print("no .pdata entry contains %#x" % want)
            # nearest
            lo = [x for x in ents if x[0] <= want]
            if lo:
                b, e = lo[-1]; print("  prev entry %#x..%#x" % (b, e))
            hi = [x for x in ents if x[0] > want]
            if hi:
                b, e = hi[0]; print("  next entry %#x..%#x" % (b, e))
