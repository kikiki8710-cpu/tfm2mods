# -*- coding: utf-8 -*-
"""chkprobe060.py — probe_tbl.rs 의 각 RVA 가 .pdata 함수 시작인지 · 패치 구간 겹침 · 진입부 명령 위험 요소를 검사"""
import io, os, re, struct, sys
sys.stdout.reconfigure(encoding="utf-8")
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
EXE = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe"
TBL = r"C:\tfm2mods\tfm2_judge_verify060\src\probe_tbl.rs"
d = io.open(EXE, "rb").read()
pe = struct.unpack_from("<I", d, 0x3c)[0]; nsec = struct.unpack_from("<H", d, pe + 6)[0]; opt = struct.unpack_from("<H", d, pe + 20)[0]
secs = {}
for i in range(nsec):
    o = pe + 24 + opt + i * 40; name = d[o:o + 8].rstrip(b"\0").decode(); va, vsz, raw, rsz = struct.unpack_from("<IIII", d, o + 12); secs[name] = (va, vsz, raw, rsz)
def rva2off(r):
    for va, vsz, raw, rsz in secs.values():
        if va <= r < va + max(vsz, rsz): return raw + (r - va)
va, vsz, raw, rsz = secs[".pdata"]; starts = set(); ranges = []
for k in range(0, rsz, 12):
    b, e, u = struct.unpack_from("<III", d, raw + k)
    if b == 0: break
    starts.add(b); ranges.append((b, e))
ranges.sort()
probes = [(int(m.group(1)), int(m.group(2), 16), m.group(3), len(m.group(4).split(","))) for m in re.finditer(r"idx: (\d+), rva: 0x([0-9a-f]+), name: \"([^\"]*)\".*?orig: &\[([^\]]*)\]", io.open(TBL, encoding="utf-8").read())]
cs = Cs(CS_ARCH_X86, CS_MODE_64)
bad = 0
import bisect
bs = [r[0] for r in ranges]
for idx, rva, name, n in probes:
    issues = []
    if rva not in starts:
        j = bisect.bisect_right(bs, rva) - 1
        issues.append(u"pdata 시작 아님(가장 가까운 함수 %x..%x)" % ranges[j] if j >= 0 else u"pdata 없음")
    for idx2, rva2, name2, n2 in probes:
        if idx2 != idx and rva < rva2 < rva + n: issues.append(u"겹침: %x %s 가 패치 구간 안" % (rva2, name2))
    code = d[rva2off(rva):rva2off(rva) + n]
    for ins in cs.disasm(code, 0x140000000 + rva):
        if ins.mnemonic in ("lea",) and "rsp" in ins.op_str and "rbp" not in ins.op_str: pass
        if "rax" in ins.op_str and ins.mnemonic not in ("push",) and ("[rax" in ins.op_str or ins.op_str.split(",")[-1].strip() in ("rax", "eax")): issues.append(u"진입 rax 사용: %s %s" % (ins.mnemonic, ins.op_str))
    if issues: bad += 1; print(u"[%d] %x %s n=%d: %s" % (idx, rva, name, n, u" · ".join(issues)))
print(u"probes %d · 의심 %d" % (len(probes), bad))
