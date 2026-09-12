# -*- coding: utf-8 -*-
u"""크래시 지점 `tfm2_judge_verify.dll+0x2d931` 이 **어느 함수 안**인지 특정한다.

WER 기록: `Exception 0xc0000005` · `Faulting module tfm2_judge_verify.dll` · `Fault offset 0x2d931`.
⟹ 그 RVA 를 품는 `.pdata`(RUNTIME_FUNCTION) 엔트리를 찾아 함수 경계를 얻고,
   그 구간을 capstone 으로 디스어셈해 **터진 명령**을 본다.
"""
import io
import struct
import sys

import capstone

sys.stdout.reconfigure(encoding="utf-8", errors="replace")
DLL = (r"C:\Program Files (x86)\Steam\steamapps\common"
       r"\Teamfight Manager2\mods\tfm2_judge_verify\tfm2_judge_verify.dll")
FAULT = 0x2d931

d = open(DLL, "rb").read()
pe = struct.unpack_from("<I", d, 0x3c)[0]
nsec = struct.unpack_from("<H", d, pe + 6)[0]
optsz = struct.unpack_from("<H", d, pe + 20)[0]
magic = struct.unpack_from("<H", d, pe + 24)[0]
imgbase = struct.unpack_from("<Q", d, pe + 24 + 24)[0]
ddoff = pe + 24 + (112 if magic == 0x20b else 96)
secs = []
for i in range(nsec):
    o = pe + 24 + optsz + i * 40
    name = d[o:o + 8].rstrip(b"\0").decode("ascii", "replace")
    vsz, va, rsz, ra = struct.unpack_from("<IIII", d, o + 8)
    secs.append((name, va, vsz, ra, rsz))

print(u"ImageBase=0x%x · 섹션 %d개" % (imgbase, nsec))
for n, va, vsz, ra, rsz in secs:
    mark = u"  ← fault 여기" if va <= FAULT < va + max(vsz, rsz) else u""
    print(u"  %-8s va=0x%-8x vsz=0x%-7x raw=0x%-8x%s" % (n, va, vsz, ra, mark))


def r2o(rva):
    for n, va, vsz, ra, rsz in secs:
        if va <= rva < va + max(vsz, rsz):
            return ra + (rva - va)
    return None


# .pdata = Exception directory = data dir index 3
edir_rva, edir_sz = struct.unpack_from("<II", d, ddoff + 3 * 8)
print(u"\n.pdata rva=0x%x size=%d (%d 엔트리)" % (edir_rva, edir_sz, edir_sz // 12))
po = r2o(edir_rva)
found = None
for k in range(edir_sz // 12):
    beg, end, unw = struct.unpack_from("<III", d, po + k * 12)
    if beg <= FAULT < end:
        found = (beg, end, unw)
        break

if not found:
    print(u"⛔fault 를 품는 RUNTIME_FUNCTION 이 없다 — 리프 함수이거나 .pdata 밖")
    sys.exit(0)

beg, end, unw = found
print(u"★함수 경계: 0x%x ~ 0x%x  (%d B) · fault 는 +0x%x 지점"
      % (beg, end, end - beg, FAULT - beg))

off = r2o(beg)
md = capstone.Cs(capstone.CS_ARCH_X86, capstone.CS_MODE_64)
md.detail = False
print(u"\n--- 디스어셈 (★ = 터진 명령) ---")
lines = []
for ins in md.disasm(d[off:off + (end - beg)], beg):
    lines.append((ins.address, ins.mnemonic, ins.op_str, ins.size))
for i, (a, m, o, sz) in enumerate(lines):
    if a <= FAULT < a + sz:
        lo, hi = max(0, i - 14), min(len(lines), i + 8)
        for j in range(lo, hi):
            aa, mm, oo, _ = lines[j]
            star = u"  ★★★" if aa == a else u""
            print(u"  +0x%-6x %-8s %s%s" % (aa, mm, oo, star))
        break
else:
    print(u"  (fault 가 명령 경계에 안 맞는다 — 앞부분만 출력)")
    for a, m, o, sz in lines[:30]:
        print(u"  +0x%-6x %-8s %s" % (a, m, o))
