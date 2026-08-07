# -*- coding: utf-8 -*-
"""손으로 확정하려는 사이트를 **exe 실바이트로 검증**한다. 표를 믿지 않는다."""
import io, sys

sys.path.insert(0, r'C:\tfm2mods\v54')
import reloc as R
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
B = 0x140000000
E3, E4 = R.E3, R.E4

CASES = [
    ('pe_minion_add',   0xccd3f3, 0xcabc0a, '값이 4096000000 → 4096000001 로 바뀌었는지'),
    ('pe_wall_risk',    0xcc9eaf, 0xca8936, 'prefix 48c700(rax) → 49c707(r15) 인지'),
    ('스로틀 하한 400', 0xd0cb6b, 0xe76cb0, 'add(7B,off3) → lea(8B,off4) 로 모양이 바뀌었는지'),
    ('pe_noise_exempt', 0xcd0e38, 0xcaf845, '100000 → 100 인지 (⚠캡 상수와 값 충돌)'),
    ('pe_well_risk a',  0xcca0a6, None,     '054 에 대응이 있는지'),
    ('pe_well_risk b',  0xcca0ad, None,     '동상'),
]

for name, a, b, q in CASES:
    print('■ %s   — %s' % (name, q))
    i3 = next((i for i in E3.dis(a, 16)), None)
    print('   053 %06x  %-24s %s %s' % (a, i3.bytes.hex(), i3.mnemonic, i3.op_str))
    if b:
        i4 = next((i for i in E4.dis(b, 16)), None)
        print('   054 %06x  %-24s %s %s' % (b, i4.bytes.hex(), i4.mnemonic, i4.op_str))
    else:
        print('   054 (후보 미지정)')
    print()

# 9999 즉치가 054 position_eval 본문에 몇 개인지 — pe_well_risk 판정 근거
f = E4.func_of(0xca87f0)
n = [(i.address - B, i.bytes.hex(), i.mnemonic + ' ' + i.op_str)
     for i in R.insns(E4, f[0], f[1]) if '0x270f' in i.op_str]
print('054 ca87f0 안의 9999(0x270f) 즉치 %d곳:' % len(n))
for x in n:
    print('   %06x %-24s %s' % x)
