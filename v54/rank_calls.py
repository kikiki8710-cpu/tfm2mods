# -*- coding: utf-8 -*-
"""후보 함수의 **실제 재현 비용**으로 재정렬 — 크기가 아니라 `call` 개수가 지배한다.

프로젝트 원칙(완전 재구현): 재현 시 내부 FUN_*·vtable 슬롯 호출까지 전부 Rust 로 직접 구현한다.
⟹ 함수가 작아도 호출이 있으면 그 callee 들까지 재현해야 하므로 진짜 비용은 **호출 폐포**다.
call 0 개인 함수(leaf)가 있으면 그게 오늘 당장 뚫을 수 있는 표적이다."""
import sys, io, re, glob, os, struct
import bisect as _bs
import pefile, capstone
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

SRC = 'C:/tfm2mods/tfm2_ai_adjust/src'
EXE = r'C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe'
pe = pefile.PE(EXE, fast_load=True)
data = pe.__data__
pd = [s for s in pe.sections if s.Name.rstrip(b'\x00') == b'.pdata'][0]
raw = data[pd.PointerToRawData: pd.PointerToRawData + pd.SizeOfRawData]
funcs = []
for i in range(0, len(raw) - 11, 12):
    b, e, _ = struct.unpack_from('<III', raw, i)
    if b == 0:
        break
    funcs.append((b, e))
funcs.sort()
starts = [f[0] for f in funcs]


def owner(rva):
    i = _bs.bisect_right(starts, rva) - 1
    if i < 0:
        return None
    b, e = funcs[i]
    return (b, e) if b <= rva < e else None


def rva2off(rva):
    for s in pe.sections:
        if s.VirtualAddress <= rva < s.VirtualAddress + max(s.Misc_VirtualSize, s.SizeOfRawData):
            return s.PointerToRawData + (rva - s.VirtualAddress)
    return None


md = capstone.Cs(capstone.CS_ARCH_X86, capstone.CS_MODE_64)


def calls(b, e):
    off = rva2off(b)
    direct = indirect = 0
    for ins in md.disasm(data[off:off + (e - b)], 0x140000000 + b):
        if ins.mnemonic == 'call':
            if ins.op_str.startswith('0x'):
                direct += 1
            else:
                indirect += 1
    return direct, indirect


# ── 사이트 수집(rank_targets 와 동일 규약) ──
FN = re.compile(r'^\s*(?:pub\s+)?(?:unsafe\s+)?fn\s+([A-Za-z0-9_]+)')
LET = re.compile(r'let\s+(\w+)\s*=\s*tune\(\s*"([a-zA-Z0-9_]+)"')
ADDR = re.compile(r'base\s*\+\s*(0x[0-9a-fA-F]+)')
VAR = re.compile(r'\b(?:b1|b4|sq)\(\s*(\w+)\s*,')
TUNE = re.compile(r'tune\(\s*"([a-zA-Z0-9_]+)"')

knob_sites, imm, judge = {}, set(), set()
for f in sorted(glob.glob(os.path.join(SRC, '*.rs'))):
    cur, v2k = '?', {}
    for ln in io.open(f, encoding='utf-8'):
        m = FN.match(ln)
        if m:
            cur = m.group(1); v2k = {}
        ap = cur.startswith('apply_')
        if not ln.lstrip().startswith('//'):
            for k in TUNE.findall(ln):
                if k != 'key':
                    (imm if ap else judge).add(k)
        if ap:
            for v, k in LET.findall(ln):
                v2k[v] = k
            if 'patch_imm_bytes' in ln or re.search(r'\bp!\(', ln):
                ads = [int(a, 16) for a in ADDR.findall(ln)]
                for v in VAR.findall(ln):
                    if v in v2k:
                        knob_sites.setdefault(v2k[v], set()).update(ads)

imm_only = imm - judge
byfn = {}
for k in imm_only:
    for a in knob_sites.get(k, ()):
        r = owner(a)
        if r:
            byfn.setdefault(r, set()).add(k)

rows = []
for (b, e), ks in byfn.items():
    d, i = calls(b, e)
    rows.append((d + i, e - b, len(ks), b, d, i, sorted(ks)))
rows.sort(key=lambda r: (r[0], r[1]))
print('%-10s %6s %5s %5s %5s  %s' % ('함수', '크기', 'call', '직접', '간접', '노브'))
for tot, size, nk, b, d, i, ks in rows[:18]:
    print('%#-10x %6d %5d %5d %5d  %s' % (b, size, tot, d, i, ', '.join(ks)[:74]))
print('\n후보 함수 %d개 · call 0개(leaf) = %d개' % (len(rows), sum(1 for r in rows if r[0] == 0)))
