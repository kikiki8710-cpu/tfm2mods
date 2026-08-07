# -*- coding: utf-8 -*-
"""경로 시스템 노브 배선 코드 생성 — pathknobs.py 출력을 **정적 표 + 루프**로 변환.
★크래시2(스택 오버플로) 재발 방지: 200여 사이트를 p! 인라인으로 펼치면 opt-level=1 에서
  호출부마다 스택 슬롯이 생겨 rayon 워커 스택이 터진다. 반드시 표 1개 + 루프 1개.
생성 전 전 사이트를 0.5.4 exe 로 대조(prefix / imm 실제위치 / 원본값 / 명령경계)한다."""
import sys, io, re, subprocess
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
from pe2 import load
import capstone
p = load('054'); md = capstone.Cs(capstone.CS_ARCH_X86, capstone.CS_MODE_64); md.detail = True

out = subprocess.run([sys.executable, 'pathknobs.py', '054'], capture_output=True).stdout.decode('utf-8', 'replace')
LINE = re.compile(r'p!\(base \+ 0x([0-9a-f]+), &\[([^\]]*)\], (\d+), (\d+), /\*orig (-?\d+)\*/\)')

groups, cur = {}, None
for ln in out.split('\n'):
    m = re.match(r'#\s*\[(\w+)\]', ln)
    if m: cur = m.group(1); groups.setdefault(cur, [])
    mm = LINE.search(ln)
    if mm and cur:
        a = int(mm.group(1), 16)
        pre = [int(x, 16) for x in re.findall(r'0x([0-9a-f]+)', mm.group(2))]
        groups[cur].append((a, pre, int(mm.group(3)), int(mm.group(4)), int(mm.group(5))))

def verify(sites, label):
    bad = []
    for a, pre, off, w, orig in sites:
        got = p.rd(a, 24)
        ins = next(md.disasm(got, a), None)
        why = []
        if list(got[:len(pre)]) != pre: why.append('prefix')
        real = None
        if ins:
            e = ins.encoding
            real = e.imm_offset or e.disp_offset or None
            if off + w > ins.size: why.append('경계초과')
        if real is not None and real != off: why.append('imm위치 %d!=%d' % (real, off))
        cur = int.from_bytes(got[off:off+w], 'little')
        if w == 1 and cur > 127: cur -= 256
        if cur != orig: why.append('원본 %d!=%d' % (cur, orig))
        if why: bad.append((hex(a), why))
    print('%-10s %3d사이트  %s' % (label, len(sites), 'OK' if not bad else '⚠%d건 %s' % (len(bad), bad[:3])))
    return not bad

WANT = [('step640','path_orth_cost'), ('step896','path_diag_cost'),
        ('risk1281','path_danger_cost'), ('heur','path_greedy')]
allok = True
for g, name in WANT:
    allok &= verify(groups.get(g, []), g)

RISK = [(0xdb05fc,[0xb8],1,4,2), (0xdb0745,[0xb8],1,4,2), (0xdb07cb,[0x83,0xc1],2,1,2),
        (0xdb07ce,[0x83,0xf9],2,1,60), (0xdb07d1,[0xb8],1,4,60),
        (0xdb077e,[0xb9],1,4,30), (0xdb07b2,[0xb9],1,4,30), (0xd3101c,[0xb8],1,4,3)]
allok &= verify(RISK, 'risk+wave')
print('\n전체 %s' % ('통과' if allok else '⚠실패 — 배선 중단'))
if not allok: sys.exit(1)

def tbl(name, sites, ty='usize'):
    rows = []
    for a, pre, off, w, _ in sites:
        rows.append('    (0x%x, &[%s], %d),' % (a, ','.join('0x%02x' % b for b in pre), off))
    return ('static %s: [(usize, &[u8], usize); %d] = [\n%s\n];\n' % (name, len(sites), '\n'.join(rows)))

src = []
for g, name in WANT:
    src.append(tbl('PATH_' + g.upper(), groups[g]))
io.open('path_tables.rs', 'w', encoding='utf-8', newline='\n').write('\n'.join(src))
print('path_tables.rs 생성: ' + ', '.join('%s=%d' % (g, len(groups[g])) for g, _ in WANT))
