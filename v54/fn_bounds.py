# -*- coding: utf-8 -*-
"""주어진 RVA 를 포함하는 함수의 경계를 .pdata(예외 테이블)에서 찾고, 진입부를 디스어셈한다.

Ghidra 두 인스턴스 모두 구버전 exe 라 0.5.4 주소를 못 찾는다. .pdata 의 RUNTIME_FUNCTION
(begin, end, unwind) 배열은 x64 PE 의 정본 함수 경계라 이걸로 대신한다.

사용: python fn_bounds.py <rva 16진> [디스어셈 줄수]"""
import sys, io, bisect, struct
import pefile, capstone
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

EXE = r'C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe'
pe = pefile.PE(EXE, fast_load=True)
pe.parse_data_directories(directories=[pefile.DIRECTORY_ENTRY['IMAGE_DIRECTORY_ENTRY_EXCEPTION']])
data = pe.__data__

pd = None
for s in pe.sections:
    if s.Name.rstrip(b'\x00') == b'.pdata':
        pd = s
raw = data[pd.PointerToRawData: pd.PointerToRawData + pd.SizeOfRawData]
funcs = []
for i in range(0, len(raw) - 11, 12):
    b, e, u = struct.unpack_from('<III', raw, i)
    if b == 0:
        break
    funcs.append((b, e))
funcs.sort()
starts = [f[0] for f in funcs]
print('.pdata 함수 %d개' % len(funcs))


def find(rva):
    i = bisect.bisect_right(starts, rva) - 1
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
md.detail = False
for arg in sys.argv[1:2] or ['e58cf1']:
    rva = int(arg, 16)
    r = find(rva)
    if not r:
        print('%#x : 함수 경계 못 찾음' % rva); continue
    b, e = r
    print('\n=== RVA %#x 를 포함하는 함수 : %#x ~ %#x (길이 %d) ===' % (rva, b, e, e - b))
    n = int(sys.argv[2]) if len(sys.argv) > 2 else 40
    off = rva2off(b)
    code = data[off: off + min(e - b, n * 15)]
    for k, ins in enumerate(md.disasm(code, 0x140000000 + b)):
        if k >= n:
            break
        mark = '  <<< 사이트' if (ins.address - 0x140000000) == rva else ''
        print('  %08x  %-9s %s%s' % (ins.address - 0x140000000, ins.mnemonic, ins.op_str, mark))
