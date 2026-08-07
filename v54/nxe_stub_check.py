# -*- coding: utf-8 -*-
"""nexus_emg.rs 의 디투어 스텁을 파이썬으로 그대로 재현해 capstone 으로 검증한다.

확인 포인트
  ① 각 명령이 의도대로 디코딩되는가(특히 `[rbx+rax+0x148]`·`[rbx+rcx*8+disp]` 의 SIB)
  ② 분기 rel8 이 `.take`/`.skip` **명령 경계**에 정확히 떨어지는가 (한 바이트만 어긋나도 즉사)
  ③ 스택 균형 — 모든 경로가 push 2회 ↔ pop 2회

⚠이 파일을 bash heredoc 으로 고치지 말 것(백슬래시 먹힘).
"""
import sys, io, capstone
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

BASE = 0x140000000
NXE_FAIL_RVA = 0xce3c23
NXE_PASS_RVA = 0xce3c36
ADDR_MAX = 0x7FF6FD330000      # &NXE_MAX (예시)
ADDR_T2 = 0x7FF6FD330008       # &NXE_T2  (예시)


def build():
    s = bytearray()
    fix_take, fix_skip = [], []
    s += b'\x52'                                   # push rdx
    s += b'\x41\x50'                               # push r8
    s += b'\x48\x8b\x94\x03' + (0x148).to_bytes(4, 'little')   # mov rdx,[rbx+rax+0x148]
    s += b'\x49\xb8' + ADDR_MAX.to_bytes(8, 'little')          # movabs r8,&NXE_MAX
    s += b'\x49\x3b\x10'                           # cmp rdx,[r8]
    s += b'\x76\x00'; fix_take.append(len(s) - 1)  # jbe .take
    s += b'\x49\xb8' + ADDR_T2.to_bytes(8, 'little')           # movabs r8,&NXE_T2
    s += b'\x49\x83\x38\x00'                       # cmp qword [r8],0
    s += b'\x74\x00'; fix_skip.append(len(s) - 1)  # je .skip
    for disp in (0x190, 0x1b0, 0x1d0):
        s += b'\x48\x83\xbc\xcb' + disp.to_bytes(4, 'little') + b'\x00'   # cmp [rbx+rcx*8+disp],0
        s += b'\x74\x00'; fix_take.append(len(s) - 1)                     # je .take
    skip_off = len(s)
    s += b'\x41\x58' + b'\x5a'                     # pop r8 ; pop rdx
    s += b'\xff\x25\x00\x00\x00\x00' + (BASE + NXE_FAIL_RVA).to_bytes(8, 'little')
    take_off = len(s)
    s += b'\x41\x58' + b'\x5a'
    s += b'\xff\x25\x00\x00\x00\x00' + (BASE + NXE_PASS_RVA).to_bytes(8, 'little')
    for lst, tgt in ((fix_take, take_off), (fix_skip, skip_off)):
        for pos in lst:
            d = tgt - (pos + 1)
            assert 0 <= d <= 127, 'rel8 범위 초과: %d' % d
            s[pos] = d
    return bytes(s), skip_off, take_off


code, skip_off, take_off = build()
md = capstone.Cs(capstone.CS_ARCH_X86, capstone.CS_MODE_64)
md.detail = True
STUB = 0x200000000

print('스텁 %d바이트   .skip=+%#x   .take=+%#x\n' % (len(code), skip_off, take_off))
starts, targets = set(), []
for ins in md.disasm(code, STUB):
    off = ins.address - STUB
    tag = ''
    if off == skip_off: tag = '   ← .skip (조건 미충족 → 원본 0 반환 경로)'
    if off == take_off: tag = '   ← .take (조건 충족 → 원본 통과 경로)'
    starts.add(off)
    if ins.mnemonic in ('jbe', 'je'):
        t = ins.operands[0].imm - STUB
        targets.append((off, ins.mnemonic, t))
        tag = '   → +%#x %s' % (t, '(.take)' if t == take_off else '(.skip)' if t == skip_off else '★어긋남')
    print('  +%03x  %-24s %-6s %-34s%s' % (off, ins.bytes.hex(), ins.mnemonic, ins.op_str, tag))

print('\n── 분기 착지 검증 ──')
ok = True
for off, mn, t in targets:
    good = t in starts and t in (skip_off, take_off)
    if not good: ok = False
    print('  +%03x %-4s → +%#x  %s' % (off, mn, t, 'OK 명령 경계' if good else '★어긋남'))
consumed = sum(i.size for i in md.disasm(code, STUB))
print('\n소비 %d/%d 바이트 (끝 8B×2 = 점프 타깃 리터럴)' % (consumed, len(code)))
print('결과:', '전 분기 정상' if ok else '★분기 어긋남 — 설치 금지')
