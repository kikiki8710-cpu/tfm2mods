# -*- coding: utf-8 -*-
"""노브 재배치 **위험도 등급**을 매긴다.

원리: 노브가 든 소스 파일이 0.5.4에서 얼마나 바뀌었는가 = 그 노브가 엉뚱한 자리에
     붙을 위험. 크게 줄어든 파일일수록 "로직이 다른 곳으로 옮겨갔다"는 뜻이므로,
     상수 스캔으로 찾으면 **다른 자리를 잡고 '죽었다'고 오판**하기 쉽다.
"""
import io, os, sys, collections

sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
D = r'C:\tfm2mods\v54'


def srcsize(ver):
    """★결합키(`A | B`)를 **파일 단위로 풀어서** 합산한다.

    ⚠결합키를 그대로 크기로 읽으면 안 된다(2026-08-05 실사고).
      0.5.4에서 utils.rs 패닉이 큰 핸들러에 인라인되면 27KB 본문이
      `line_defense.rs` 버킷 → `line_defense.rs | utils.rs` 버킷으로 통째 이사한다.
      결합키로 세면 line_defense 가 −80% 로 보이지만, 파일 단위로 합산하면 **+1.1%** 다.
      이 착각으로 "판단 파일 대량 삭제"라고 1단계 보고를 냈다가 정정했다.
    """
    tot = collections.Counter()
    cnt = collections.Counter()
    for ln in io.open(os.path.join(D, '%s_srcmap.tsv' % ver), encoding='utf-8'):
        s, e, src, _ = ln.rstrip('\n').split('\t')
        sz = int(e, 16) - int(s, 16)
        for p in set(x.strip() for x in src.split('|') if x.strip()):
            tot[p] += sz
            cnt[p] += 1
    return tot, cnt


def keysize(sz, cnt, src):
    """결합키 하나의 위험도를 그 안에 든 **개별 파일들의 최악값**으로 본다."""
    ps = [p.strip() for p in src.split('|') if p.strip()]
    return (sum(sz.get(p, 0) for p in ps), sum(cnt.get(p, 0) for p in ps), ps)


a_sz, a_ct = srcsize('053')
b_sz, b_ct = srcsize('054')

sites = collections.Counter()
knobs = collections.defaultdict(set)
first = True
for ln in io.open(os.path.join(D, 'knob_sites_053.tsv'), encoding='utf-8'):
    if first:
        first = False
        continue
    k, rva, fs, fe, src, by, asm, where = ln.rstrip('\n').split('\t')
    sites[src] += 1
    if k != '?':
        knobs[src].add(k)

rows = []
for src, n in sites.items():
    sa, sb = a_sz.get(src, 0), b_sz.get(src, 0)
    if src == '(소스미상)':
        tier, note = 'D', '소스 미상 — 개별 확인 필요'
    elif sb == 0:
        tier, note = 'A', '★0.5.4에 같은 소스 조합이 없음 — 통째로 재탐색'
    else:
        d = (sb - sa) / sa if sa else 0
        if d <= -0.30:
            tier, note = 'A', '★%.0f%% 축소 — 로직이 다른 파일로 이동했을 공산' % (d * 100)
        elif abs(d) >= 0.08:
            tier, note = 'B', '%+.0f%% 변화 — 구조 확인 후 재배치' % (d * 100)
        elif a_ct.get(src) != b_ct.get(src):
            tier, note = 'B', '함수 수 %d→%d' % (a_ct.get(src, 0), b_ct.get(src, 0))
        else:
            tier, note = 'C', '크기·함수 수 거의 동일 — 문맥 대조로 확인 가능'
    rows.append((tier, n, src, sa, sb, note, sorted(knobs[src])))

order = {'A': 0, 'B': 1, 'D': 2, 'C': 3}
rows.sort(key=lambda r: (order[r[0]], -r[1]))

tot = collections.Counter()
for r in rows:
    tot[r[0]] += r[1]
print('사이트 위험도  A(재탐색 필수) %d · B(구조확인) %d · D(소스미상) %d · C(대조로 충분) %d\n'
      % (tot['A'], tot['B'], tot['D'], tot['C']))

cur = None
for tier, n, src, sa, sb, note, ks in rows:
    if tier != cur:
        print('\n===== 등급 %s =====' % tier)
        cur = tier
    print('  [%3d사이트] %-64s  %s' % (n, src[:64], note))
    if ks:
        print('              노브: %s' % ', '.join(ks[:14]) + (' …' if len(ks) > 14 else ''))

io.open(os.path.join(D, 'knob_risk.txt'), 'w', encoding='utf-8').write(
    '\n'.join('%s\t%d\t%s\t%d\t%d\t%s\t%s' % (t, n, s, sa, sb, no, ','.join(k))
              for t, n, s, sa, sb, no, k in rows))
print('\n→ knob_risk.txt')
