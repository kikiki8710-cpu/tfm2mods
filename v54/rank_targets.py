# -*- coding: utf-8 -*-
"""클래스별로 뚫을 후보를 **포함 함수 크기**로 줄 세운다.

클래스별 값을 주려면 그 상수가 런타임에 읽혀야 하고 = 그 계산을 재현(또는 디투어)해야 한다.
재현 비용의 1차 근사 = **그 상수가 든 함수의 크기**. 작을수록 싸다.
사이트 주소는 소스의 patch_imm_bytes(base + 0x...) 에서 뽑고, 함수 경계는 .pdata 에서 얻는다."""
import sys, io, re, glob, os, struct
import bisect as _bs
import pefile
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

SRC = 'C:/tfm2mods/tfm2_ai_adjust/src'
EXE = r'C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe'

# ── .pdata 함수 경계 ──
pe = pefile.PE(EXE, fast_load=True)
data = pe.__data__
pd = [s for s in pe.sections if s.Name.rstrip(b'\x00') == b'.pdata'][0]
raw = data[pd.PointerToRawData: pd.PointerToRawData + pd.SizeOfRawData]
funcs = []
for i in range(0, len(raw) - 11, 12):
    b, e, _u = struct.unpack_from('<III', raw, i)
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


# ── 소스에서 (노브 변수 → 사이트 주소) 수집 ──
# apply_* 함수 본문을 훑으며, 같은 줄의 patch 호출에 등장하는 주소를 그 줄의 노브에 귀속시킨다.
FN = re.compile(r'^\s*(?:pub\s+)?(?:unsafe\s+)?fn\s+([A-Za-z0-9_]+)')
LET = re.compile(r'let\s+(\w+)\s*=\s*tune\(\s*"([a-zA-Z0-9_]+)"')
ADDR = re.compile(r'base\s*\+\s*(0x[0-9a-fA-F]+)')
VAR = re.compile(r'\b(?:b1|b4|sq)\(\s*(\w+)\s*,')

knob_sites = {}     # knob -> set(rva)
for f in sorted(glob.glob(os.path.join(SRC, '*.rs'))):
    cur, var2knob = '?', {}
    for ln in io.open(f, encoding='utf-8'):
        m = FN.match(ln)
        if m:
            cur = m.group(1); var2knob = {}
        if not cur.startswith('apply_'):
            continue
        for v, k in LET.findall(ln):
            var2knob[v] = k
        if 'patch_imm_bytes' in ln or re.search(r'\bp!\(', ln) or re.search(r'\bp\d?!\(', ln):
            addrs = [int(a, 16) for a in ADDR.findall(ln)]
            ks = {var2knob[v] for v in VAR.findall(ln) if v in var2knob}
            for k in ks:
                knob_sites.setdefault(k, set()).update(addrs)

# ── imm 전용 노브만 대상 ──
TUNE = re.compile(r'tune\(\s*"([a-zA-Z0-9_]+)"')
imm, judge = set(), set()
for f in sorted(glob.glob(os.path.join(SRC, '*.rs'))):
    cur = '?'
    for ln in io.open(f, encoding='utf-8'):
        if ln.lstrip().startswith('//'):
            continue
        m = FN.match(ln)
        if m:
            cur = m.group(1)
        for k in TUNE.findall(ln):
            if k == 'key':
                continue
            (imm if cur.startswith('apply_') else judge).add(k)
imm_only = imm - judge

rows = []
for k in sorted(imm_only):
    sites = knob_sites.get(k)
    if not sites:
        continue
    fns = {}
    for a in sites:
        r = owner(a)
        if r:
            fns.setdefault(r, 0)
            fns[r] += 1
    if not fns:
        continue
    size = sum(e - b for (b, e) in fns)
    rows.append((size, len(fns), len(sites), k, sorted(fns)))
rows.sort()
print('%-26s %8s %6s %6s  %s' % ('노브', '함수합계', '함수수', '사이트', '함수 (RVA)'))
for size, nf, ns, k, fs in rows[:34]:
    print('%-26s %8d %6d %6d  %s' % (k, size, nf, ns, ' '.join('%#x' % b for b, _e in fs[:3])))
print('\n대상 %d개 (사이트 주소를 뽑을 수 있었던 것만)' % len(rows))
