# -*- coding: utf-8 -*-
"""INDEX.md 의 버전 절 줄들을 **0.5.4 기준 처리 방침**으로 분류한다.

기준(기계정본): 현행 모드 소스 = 0.5.4. `orig_table.rs`(인게임 검증 756사이트) + `detour.rs`의
`base + 0x…` + `rva_054.rs` 에 등장하는 주소 = **0.5.4 실주소**.
`rva_053.rs`/`rva_052.rs` 에만 있는 주소 = 구버전 전용.

분류:
  A 버전무관   — 줄에 RVA 가 없다(공식·구조체 오프셋·판정 로직·교훈). **유지**
  B 0.5.4 확인 — 줄의 RVA 중 하나 이상이 0.5.4 소스에 실재. **유지(현행)**
  C 구버전 전용 — RVA 가 전부 구버전 소스에만 있음. **아카이브**
  D 불명       — RVA 가 어느 소스에도 없음. ★**RE 대상**
"""
import sys, io, re, os
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

SRC = 'C:/tfm2mods/tfm2_ai_adjust/src'
IDX = ('C:/Users/dev/.claude/projects/C--Users-dev-Desktop-claude-tfm2'
       '/memory/INDEX.md')

def addrs(path):
    if not os.path.exists(path):
        return set()
    t = io.open(path, encoding='utf-8', errors='replace').read()
    return {int(a, 16) for a in re.findall(r'0x([0-9a-fA-F]{5,8})', t)}

cur = set()
for f in os.listdir(SRC):
    if f.endswith('.rs') and f not in ('rva_053.rs', 'rva_052.rs'):
        cur |= addrs(os.path.join(SRC, f))
old = addrs(SRC + '/rva_053.rs') | addrs(SRC + '/rva_052.rs')
print('0.5.4 소스 주소 %d개 · 구버전 전용 주소 %d개' % (len(cur), len(old - cur)))

lines = io.open(IDX, encoding='utf-8').read().split('\n')
# 대상 절 = 헤더 텍스트로 식별
TARGET = re.compile(r'^### (0\.5\.3|0\.5\.2|판단 AI 디스패처|모드 상수/아이템|구버전 base)')
STOP = re.compile(r'^### ')
buckets = {'A': [], 'B': [], 'C': [], 'D': []}
sec, cnt = None, {}
for i, ln in enumerate(lines, 1):
    if STOP.match(ln):
        sec = ln[:60] if TARGET.match(ln) else None
        continue
    if not sec or not ln.strip() or ln.lstrip().startswith('>'):
        continue
    if not (ln.lstrip().startswith('- ') or ln.lstrip().startswith('|')):
        continue
    a = {int(x, 16) for x in re.findall(r'0x([0-9a-fA-F]{5,8})', ln)}
    if not a:
        k = 'A'
    elif a & cur:
        k = 'B'
    elif a & old:
        k = 'C'
    else:
        k = 'D'
    buckets[k].append((i, sec, ln))
    cnt[sec] = cnt.get(sec, {})
    cnt[sec][k] = cnt[sec].get(k, 0) + 1

NAME = {'A': 'A 버전무관(유지)', 'B': 'B 0.5.4 확인(유지)',
        'C': 'C 구버전 전용(아카이브)', 'D': 'D 불명 → ★RE 대상'}
print('\n%-14s %6s %6s %6s %6s' % ('절', 'A', 'B', 'C', 'D'))
for s, d in cnt.items():
    print('%-14s %6d %6d %6d %6d' % (s[4:18], d.get('A', 0), d.get('B', 0), d.get('C', 0), d.get('D', 0)))
print('\n합계')
for k in 'ABCD':
    print('  %-22s %d줄' % (NAME[k], len(buckets[k])))

io.open('C:/tfm2mods/v54/index_class.txt', 'w', encoding='utf-8').write(
    '\n\n'.join('=== %s ===\n' % NAME[k] +
                '\n'.join('%5d| %s' % (i, l[:200]) for i, _s, l in buckets[k]) for k in 'DCBA'))
print('\n(전체 = v54/index_class.txt)')
print('\n── D(불명) 표본 12줄 ──')
for i, _s, l in buckets['D'][:12]:
    print('%5d| %s' % (i, l.strip()[:150]))
