# -*- coding: utf-8 -*-
"""임의 RVA 주변을 디스어셈한다. 사용: python dis_at.py <rva> [before] [after]"""
import sys, io, struct
import bisect as _bs
import pefile, capstone
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

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
rva = int(sys.argv[1], 16)
before = int(sys.argv[2]) if len(sys.argv) > 2 else 12
after = int(sys.argv[3]) if len(sys.argv) > 3 else 20
fb, fe = owner(rva) or (rva - 0x60, rva + 0x100)
print('함수 %#x ~ %#x (길이 %d) / 대상 %#x' % (fb, fe, fe - fb, rva))
off = rva2off(fb)
ins = list(md.disasm(data[off: off + (fe - fb)], 0x140000000 + fb))
idx = next((i for i, x in enumerate(ins) if x.address - 0x140000000 == rva), None)
if idx is None:
    print('명령 경계에 없음(다른 명령 중간일 수 있음)'); idx = 0
lo, hi = max(0, idx - before), min(len(ins), idx + after)
for x in ins[lo:hi]:
    r = x.address - 0x140000000
    print('  %08x  %-22s %-9s %s%s' % (r, x.bytes.hex(), x.mnemonic, x.op_str,
                                       '   <<<' if r == rva else ''))
