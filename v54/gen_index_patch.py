# -*- coding: utf-8 -*-
"""INDEX.md 반영안 생성 — 기계적으로 만들고, 적용은 record-keeper 가 한다(§9).

설계:
  · 확정 대응이 있는 주소는 **0.5.4 값으로 교체**하고 줄의 버전태그를 0.5.4 로 바꾼다.
    (주소마다 취소선을 치면 한 줄에 5개씩 붙어 읽을 수 없다 — 대신 **원본 줄을 아카이브에 통째 보존**한다)
  · 대응이 없는 주소가 남아 있으면 그 줄은 **부분 갱신**으로 표시한다(어느 게 안 바뀌었는지 명시).
  · 0.5.2/0.5.1 태그 줄은 교체하지 않고 **아카이브로 이동**한다.
출력:
  index_patch.tsv        줄번호 \t 새 줄     (INDEX 에 적용)
  INDEX-0.5.3-rva.md     교체된 줄의 **원본**(이력 보존)
  index_move.tsv         아카이브로 옮길 줄번호 \t 원문
"""
import sys, io, re
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

IDX = (r'C:\Users\dev\.claude\projects\C--Users-dev-Desktop-claude-tfm2'
       r'\memory\INDEX.md')
lines = io.open(IDX, encoding='utf-8').read().split('\n')
nums = [int(x) for x in io.open('archive_lines.txt')]

M, DEAD = {}, {}
for l in io.open('map_053_054.tsv', encoding='utf-8'):
    if l.startswith('#') or not l.strip():
        continue
    p = l.rstrip('\n').split('\t')
    if len(p) < 2:
        continue
    if p[1] == 'DEAD':
        DEAD[int(p[0], 16)] = p[2] if len(p) > 2 else ''
    else:
        M[int(p[0], 16)] = int(p[1], 16)


def ver(ln):
    m = re.findall(r'\(0\.5\.(\d)[,)]', ln)
    if m:
        return '0.5.' + m[-1]
    if re.search(r'0\.5\.2', ln) and not re.search(r'0\.5\.3|0\.5\.4', ln):
        return '0.5.2'
    if re.search(r'0\.5\.3', ln):
        return '0.5.3'
    return '?'


patch, hist, move, partial, shorthand = [], [], [], [], []
for n in nums:
    ln = lines[n - 1]
    v = ver(ln)
    addrs = [int(x, 16) for x in re.findall(r'0x([0-9a-fA-F]{5,8})', ln)]
    hit = [a for a in addrs if a in M]
    # ⚠상한을 두면 .rdata 주소(0x31e2c50 등)가 "미재핀"으로 안 잡힌다 — 하한만 둔다
    miss = [a for a in addrs if a not in M and a not in DEAD and a >= 0xc00000]
    if v in ('0.5.2', '0.5.1') and not hit:
        move.append((n, ln)); continue
    if not hit:
        continue
    # ★접두 공유 축약(`0xcd4cd7/dd/e3/e9` = cd4cd7·cd4cdd·cd4ce3·cd4ce9)은 첫 주소만 바꾸면
    #   나머지가 **옛 접두의 꼬리**로 남아 존재하지 않는 주소가 된다. 그런 줄은 손대지 않는다.
    # ★같은 이유로 **범위 표기**(`0xcef570~0xcf837b`)도 한쪽만 바뀌면 시작>끝이 되어 말이 안 된다.
    # ★그리고 한 주소라도 미재핀이면 그 줄은 신·구가 섞여 더 위험하다 — 전부 매핑됐을 때만 치환한다.
    if (re.search(r'0x[0-9a-fA-F]{5,8}(?:/[0-9a-fA-F]{2,3})+', ln)
            or re.search(r'0x[0-9a-fA-F]{5,8}\s*[~-]\s*0x[0-9a-fA-F]{5,8}', ln)
            or miss):
        shorthand.append((n, ln)); continue
    new = ln
    for a in sorted(set(hit), key=lambda x: -x):          # 긴 것부터 치환(부분치환 방지)
        new = re.sub(r'0x0*%x\b' % a, '%#x' % M[a], new, flags=re.I)
    # 버전 태그 갱신
    new = re.sub(r'\(0\.5\.[123],\s*([\d-]+)\)', r'(0.5.4 재핀 2026-08-07 / 구값 \1)', new)
    if miss:
        new += '  ⚠**부분갱신** — 미재핀 %s' % ' '.join('%#x' % a for a in sorted(set(miss)))
        partial.append(n)
    hist.append((n, ln))
    patch.append((n, new))

io.open('index_patch.tsv', 'w', encoding='utf-8').write(
    '\n'.join('%d\t%s' % (n, s) for n, s in patch))
io.open('index_move.tsv', 'w', encoding='utf-8').write(
    '\n'.join('%d\t%s' % (n, s) for n, s in move))
H = ['# INDEX §2 — 0.5.3 시절 원본 줄 (2026-08-07 아카이브)',
     '#', '# 0.5.4 재핀으로 `MEM\\INDEX.md` 의 주소를 갱신하면서, 원본 줄을 여기에 통째로 보존한다.',
     '# 대응표 정본 = `C:\\tfm2mods\\v54\\map_053_054.tsv` (546쌍).', '']
for n, s in hist:
    H.append('- (구 INDEX:%d) %s' % (n, s.lstrip('- ')))
io.open('INDEX-0.5.3-rva.md', 'w', encoding='utf-8').write('\n'.join(H))

print('INDEX 대상 %d줄 중' % len(nums))
print('  0.5.4 값으로 갱신 = %d줄  (그중 부분갱신 %d줄)' % (len(patch), len(partial)))
print('  아카이브 이동     = %d줄  (0.5.2/0.5.1 태그·대응 없음)' % len(move))
print('  ★접두축약이라 수동 = %d줄' % len(shorthand))
print('  변화 없음         = %d줄' % (len(nums) - len(patch) - len(move) - len(shorthand)))
io.open('index_shorthand.tsv', 'w', encoding='utf-8').write(
    '\n'.join('%d\t%s' % (n, s) for n, s in shorthand))
print('\n예시 3건:')
for n, s in patch[:3]:
    print('  %d| %s' % (n, s[:160]))
