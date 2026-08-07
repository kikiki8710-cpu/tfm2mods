# -*- coding: utf-8 -*-
"""마이크로 디투어 사이트의 **원본 창 바이트**를 exe 에서 뽑는다.

class_micro.rs 는 설치 전에 창 전체를 원본과 대조한다(안전규칙 ②). 그 기대 바이트를
손으로 적으면 틀리고, 틀리면 조용히 설치가 안 된다. 여기서 뽑아 그대로 붙여넣는다.

사용: python micro_win.py <rva> [최소창=5]
      RVA 부터 명령 경계를 따라가며 **5바이트 이상**이 되는 첫 지점까지를 창으로 잡고,
      창에 걸친 명령들을 디스어셈해 보여준다(창 안으로 뛰어드는 분기가 없는지는 사람이 확인).
"""
import sys, io, struct, pefile, capstone
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

EXE = r'C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe'
pe = pefile.PE(EXE, fast_load=True)
data = pe.__data__
md = capstone.Cs(capstone.CS_ARCH_X86, capstone.CS_MODE_64)


def rva_to_off(rva):
    for s in pe.sections:
        if s.VirtualAddress <= rva < s.VirtualAddress + max(s.Misc_VirtualSize, s.SizeOfRawData):
            return s.PointerToRawData + (rva - s.VirtualAddress)
    return None


def win_at(rva, minlen=5):
    off = rva_to_off(rva)
    if off is None:
        return None
    blob = data[off:off + 64]
    insns, total = [], 0
    for ins in md.disasm(blob, 0x140000000 + rva):
        insns.append(ins)
        total += ins.size
        if total >= minlen:
            break
    return blob[:total], insns, total


for arg in sys.argv[1:]:
    rva = int(arg, 16) if not arg.isdigit() else int(arg)
    r = win_at(rva)
    if r is None:
        print('rva %#x : 섹션 밖' % rva)
        continue
    win, insns, n = r
    print('── rva %#x  창 %d바이트 ──' % (rva, n))
    for ins in insns:
        print('   %#x  %-22s %s %s' % (ins.address, ins.bytes.hex(), ins.mnemonic, ins.op_str))
    print('   win: &[%s],' % ', '.join('0x%02x' % b for b in win))
    if n > insns[0].size:
        tail = win[insns[0].size:]
        print('   ⚠창이 첫 명령보다 김 → tail(스텁에서 재실행할 바이트) = &[%s]'
              % ', '.join('0x%02x' % b for b in tail))
        print('     ↑ 이 바이트들이 **위치 무관**(rip-상대·분기 없음)인지 위 디스어셈으로 확인할 것.')
    print()
