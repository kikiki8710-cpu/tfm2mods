# -*- coding: utf-8 -*-
"""바이트패치 노브 사이트에서 **self 엔티티가 살아 있는가**를 전수 스캔한다.

왜: 클래스별 값을 열려면 그 상수 자리에서 "지금 판단하는 챔프"를 알아야 한다.
    self 가 레지스터에 살아 있으면 **재현 없이** 마이크로 디투어로 값만 갈아끼울 수 있다
    (RE\\2026-08-07_클래스별노브_확장가능성.md 부록 B).

self 로드 시그니처: `mov r64, [reg + reg*8 + 0x1e0]`
    = champions[side][role] — side*5*8 스케일 + 팀배열 오프셋 0x1e0.
    0xe59f10(ex_attack_*)·0xcdc170(bv_focus_max)·0xf3d600 에서 동일 패턴 확인.

생존 판정(보수적): 로드 이후 사이트 이전에 그 레지스터에 쓰는 명령이
    **`pop` 뿐**이면 살아 있다고 본다(조기이탈 경로의 에필로그 복원).
    다른 write 가 하나라도 있으면 탈락시킨다.

교체 가능성: 사이트 명령부터 연속 5바이트 이상이어야 `E9 rel32` 가 들어간다.
    ⚠분기 타깃이 그 안에 있으면 안 되므로, 함수 내 분기 목적지 집합과 겹치는지도 본다."""
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
FUNCS = []
for i in range(0, len(raw) - 11, 12):
    b, e, _ = struct.unpack_from('<III', raw, i)
    if b == 0:
        break
    FUNCS.append((b, e))
FUNCS.sort()
ST = [f[0] for f in FUNCS]


def own(r):
    i = _bs.bisect_right(ST, r) - 1
    return FUNCS[i] if i >= 0 and FUNCS[i][0] <= r < FUNCS[i][1] else None


def foff(r):
    for s in pe.sections:
        if s.VirtualAddress <= r < s.VirtualAddress + max(s.Misc_VirtualSize, s.SizeOfRawData):
            return s.PointerToRawData + (r - s.VirtualAddress)
    return None


md = capstone.Cs(capstone.CS_ARCH_X86, capstone.CS_MODE_64)
md.detail = True
SELF_LOAD = re.compile(r'^(r\w+), qword ptr \[\w+ \+ \w+\*8 \+ 0x1e0\]$')
_cache = {}


def analyze(fn):
    """함수 1개 디스어셈 → (명령리스트, self로드 목록, 분기타깃 집합)"""
    if fn in _cache:
        return _cache[fn]
    b, e = fn
    o = foff(b)
    ins = list(md.disasm(data[o:o + (e - b)], 0x140000000 + b)) if o else []
    loads, targets = [], set()
    for x in ins:
        if x.mnemonic == 'mov':
            m = SELF_LOAD.match(x.op_str)
            if m:
                loads.append((x.address - 0x140000000, m.group(1)))
        if x.mnemonic[0] == 'j' or x.mnemonic.startswith('loop'):
            if x.op_str.startswith('0x'):
                targets.add(int(x.op_str, 16) - 0x140000000)
    _cache[fn] = (ins, loads, targets)
    return _cache[fn]


def writes_to(ins, reg, lo, hi):
    """(lo, hi) 구간에서 reg 에 쓰는 명령들 → [(rva, mnemonic)]"""
    out = []
    for x in ins:
        r = x.address - 0x140000000
        if not (lo < r < hi):
            continue
        try:
            _rd, wr = x.regs_access()
        except Exception:
            continue
        names = {x.reg_name(g) for g in wr}
        if reg in names or reg.replace('r', 'e', 1) in names:
            out.append((r, x.mnemonic))
    return out


# ── 사이트 수집 (rank_* 와 동일 규약) ──
FN = re.compile(r'^\s*(?:pub\s+)?(?:unsafe\s+)?fn\s+([A-Za-z0-9_]+)')
LET = re.compile(r'let\s+(\w+)\s*=\s*tune\(\s*"([a-zA-Z0-9_]+)"')
ADDR = re.compile(r'base\s*\+\s*(0x[0-9a-fA-F]+)')
VAR = re.compile(r'\b(?:b1|b4|sq)\(\s*(\w+)\s*,')
TUNE = re.compile(r'tune\(\s*"([a-zA-Z0-9_]+)"')

knob_sites, imm, judge = {}, set(), set()
for f in sorted(glob.glob(os.path.join(SRC, '*.rs'))):
    cur, v2k = '?', {}
    pending, pend_ttl = [], 0
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
            # ★[08-07] 추출 보강 — 주소가 **다른 줄**에 있는 형태를 놓쳐 342 중 145만 잡혔다.
            #   ①`for a in [0x…, 0x…] { p!(base + a, …) }` ②`pany!` ③앞줄 주소 + 다음줄 p!
            #   최근 본 주소를 pending 으로 들고 있다가 p! 계열에 귀속시킨다.
            here = [int(a, 16) for a in ADDR.findall(ln)]
            if re.search(r'for\s+&?\(?\s*a\b', ln):
                pending = [int(x, 16) for x in re.findall(r'0x[0-9a-fA-F]{5,7}', ln)
                           if 0xc00000 <= int(x, 16) <= 0x3000000]
                pend_ttl = 4
            if 'patch_imm_bytes' in ln or re.search(r'\bp(any|skip)?!\(', ln):
                ads = here if here else (pending if pend_ttl > 0 else [])
                for v in VAR.findall(ln):
                    if v in v2k:
                        knob_sites.setdefault(v2k[v], set()).update(ads)
            if pend_ttl > 0:
                pend_ttl -= 1
imm_only = sorted(imm - judge)

ok, no_self, dead, nofn = [], [], [], []
for k in imm_only:
    sites = sorted(knob_sites.get(k, ()))
    if not sites:
        continue
    good = []
    for a in sites:
        fn = own(a)
        if not fn:
            continue
        ins, loads, targets = analyze(fn)
        cands = [(lr, reg) for lr, reg in loads if lr < a]
        if not cands:
            continue
        for lr, reg in cands:
            w = writes_to(ins, reg, lr, a)
            if all(m == 'pop' for _r, m in w):
                # 교체 가능성: 사이트 명령부터 5바이트 확보 + 그 안에 분기타깃 없음
                sz = 0
                for x in ins:
                    r = x.address - 0x140000000
                    if r < a:
                        continue
                    if sz > 0 and r in targets:
                        break
                    sz += x.size
                    if sz >= 5:
                        break
                good.append((a, reg, lr, sz))
                break
    if good:
        ok.append((k, len(sites), good))
    else:
        no_self.append(k)

print('바이트패치 전용 노브 %d개 중 사이트 주소를 뽑은 것 = %d개'
      % (len(imm_only), len([k for k in imm_only if knob_sites.get(k)])))
print('★self 가 살아 있는 사이트를 **하나 이상** 가진 노브 = %d개' % len(ok))
print('  self 로드 없음/생존 실패          = %d개' % len(no_self))
print()
print('%-24s %6s %6s  %s' % ('노브', '사이트', '가능', '예시 (사이트 / 레지스터 / self로드 / 확보바이트)'))
for k, ns, good in ok[:32]:
    a, reg, lr, sz = good[0]
    print('%-24s %6d %6d  %#x %-4s ← %#x  %dB%s'
          % (k, ns, len(good), a, reg, lr, sz, '' if sz >= 5 else '  ⚠5B미만'))
if len(ok) > 32:
    print('  ... 외 %d개' % (len(ok) - 32))
io.open('C:/tfm2mods/v54/liveness.txt', 'w', encoding='utf-8').write(
    '\n'.join('%s sites=%d ok=%d %s' % (k, ns, len(g),
              ' '.join('%#x/%s/%dB' % (a, r, s) for a, r, _l, s in g)) for k, ns, g in ok))
print('\n(전체 = v54/liveness.txt)')
