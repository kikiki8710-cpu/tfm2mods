# -*- coding: utf-8 -*-
"""probe_tbl 의 n>12 항목 진입 명령 열거 + 함수 안으로 들어오는 상대 분기(패치 구간 fn+1..fn+n 을 타깃) 스캔"""
import io, re, struct, sys
sys.stdout.reconfigure(encoding="utf-8")
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
EXE = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe"
TBL = r"C:\tfm2mods\tfm2_judge_verify060\src\probe_tbl.rs"
d = io.open(EXE, "rb").read()
pe = struct.unpack_from("<I", d, 0x3c)[0]; nsec = struct.unpack_from("<H", d, pe + 6)[0]; opt = struct.unpack_from("<H", d, pe + 20)[0]
text = None
for i in range(nsec):
    o = pe + 24 + opt + i * 40; name = d[o:o + 8].rstrip(b"\0")
    if name == b".text": text = struct.unpack_from("<IIII", d, o + 12)
tva, tvs, traw, trs = text
probes = [(int(m.group(1)), int(m.group(2), 16), m.group(3), len(m.group(4).split(","))) for m in re.finditer(r"idx: (\d+), rva: 0x([0-9a-f]+), name: \"([^\"]*)\".*?orig: &\[([^\]]*)\]", io.open(TBL, encoding="utf-8").read())]
cs = Cs(CS_ARCH_X86, CS_MODE_64)
for idx, rva, name, n in probes:
    if n > 12:
        code = d[traw + rva - tva: traw + rva - tva + n]
        print(u"[%d] %x %s n=%d: %s" % (idx, rva, name, n, u" ; ".join(u"%s %s" % (i.mnemonic, i.op_str) for i in cs.disasm(code, 0x140000000 + rva))))
# 패치 구간 내부(fn+1..fn+n-1)로 들어오는 rel32/rel8 분기 스캔(.text 전체 선형 · 근사)
targets = {}
for idx, rva, name, n in probes:
    for k in range(1, n): targets[rva + k] = (idx, rva, name)
hits = []
code = d[traw: traw + trs]
# rel32 jmp/jcc/call: E8/E9 rel32 · 0F 8x rel32 · EB/7x rel8 — 바이트 스캔(오탐 가능 · 후보만)
import struct as st
for i in range(len(code) - 5):
    b = code[i]
    if b in (0xe8, 0xe9):
        t = tva + i + 5 + st.unpack_from("<i", code, i + 1)[0]
        if t in targets: hits.append((tva + i, b, t, targets[t]))
    elif b == 0x0f and 0x80 <= code[i + 1] <= 0x8f:
        t = tva + i + 6 + st.unpack_from("<i", code, i + 2)[0]
        if t in targets: hits.append((tva + i, b, t, targets[t]))
    elif b == 0xeb or 0x70 <= b <= 0x7f:
        t = tva + i + 2 + st.unpack_from("<b", code, i + 1)[0]
        if t in targets: hits.append((tva + i, b, t, targets[t]))
print(u"패치 구간 내부 분기 후보 %d" % len(hits))
for h in hits[:40]: print(u"  from %x op %02x → %x (idx %d %x %s)" % (h[0], h[1], h[2], h[3][0], h[3][1], h[3][2]))
