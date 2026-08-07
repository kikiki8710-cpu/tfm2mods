# -*- coding: utf-8 -*-
"""athlete_id 오프셋을 0.5.4 exe 에서 **실측**한다.
근거(소스 주석): struct B 생성자가 `[rsi+ID]←id / [rsi+ID+8]←0 / [rsi+ID+0x10]←team` 을 연속 저장하고,
이 3연속 패턴은 각 버전에서 정확히 1건이다.
  48 89 be <d32>        mov [rsi+d32], rdi
  48 c7 86 <d32+8> ...  mov qword [rsi+d32+8], 0
  48 89 86 <d32+0x10>   mov [rsi+d32+0x10], rax
동시에 provider seed 후보(0xeaf8 vs 0xeb28)의 disp32 출현 수도 센다."""
import sys, io, re, struct
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
from pe2 import load

for ver in ('053', '054'):
    p = load(ver)
    sec = [s for s in p.sections if s[0] == '.text'][0]
    start, size = sec[1], sec[2]
    data = p.rd(start, size)

    hits = []
    for m in re.finditer(rb'\x48\x89\xbe(....)', data, re.S):
        d = struct.unpack('<I', m.group(1))[0]
        if not (0x400 <= d <= 0x2000):
            continue
        tail = data[m.end():m.end() + 40]
        want2 = b'\x48\xc7\x86' + struct.pack('<I', d + 8)
        want3 = b'\x48\x89\x86' + struct.pack('<I', d + 0x10)
        if want2 in tail and want3 in tail:
            hits.append((start + m.start(), d))

    print('[%s] 3연속 저장 패턴 %d건' % (ver, len(hits)))
    for a, d in hits:
        print('    @%#x  athlete_id=%#x  (team=%#x)' % (a, d, d + 0x10))

    for cand in (0xeab8, 0xeaf8, 0xeb28):
        n = data.count(struct.pack('<I', cand))
        print('    provider seed 후보 %#x : disp32 출현 %d회' % (cand, n))
    print()
