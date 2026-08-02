# compare_builds.py — 아이템 빌드 덤프 A/B 비교
#
# tfm2_item_tactics 의 `dump_builds.trigger` 로 뽑은 CSV 두 개를 비교한다.
#   A = 기준(모드 OFF 시즌) / B = 비교(모드 ON 시즌)
#
# 사용:
#   python compare_builds.py A.csv B.csv                 # 요약 + 바뀐 셀 목록
#   python compare_builds.py A.csv B.csv --pos 0         # 특정 포지션만
#   python compare_builds.py A.csv B.csv --champ knight  # 특정 챔프만
#   python compare_builds.py A.csv B.csv --full          # 전 셀 상세
#
# 출력 3부:
#   ① 요약        — 1위 빌드가 바뀐 셀 수, 완전 동일 셀 수
#   ② 아이템 등장 빈도 변화 — 어떤 아이템이 추천에서 늘고 줄었나 (가장 해석하기 쉬운 지표)
#   ③ 셀별 변화   — 챔프·포지션마다 A→B 1위 빌드

import csv, sys, argparse
from collections import Counter, defaultdict


def load(path):
    """CSV → {(champ,pos): [(rank, score, [items…]), …]}. `#` 머리말은 건너뛰되 경고는 모아 둔다."""
    cells, notes = defaultdict(list), []
    with open(path, encoding='utf-8') as f:
        lines = []
        for ln in f:
            if ln.startswith('#'):
                notes.append(ln.rstrip('\n'))
            else:
                lines.append(ln)
    for r in csv.DictReader(lines):
        key = (r['champion'], int(r['position']))
        items = [r[f'item{i}'] for i in range(4)]
        cells[key].append((int(r['rank']), float(r['score']), items))
    for k in cells:
        cells[k].sort()
    return cells, notes


def build_str(items):
    # 빌드는 집합이다 — 순서가 달라도 같은 빌드로 본다.
    return ' + '.join(sorted(items))


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('a'); ap.add_argument('b')
    ap.add_argument('--pos', type=int, default=None)
    ap.add_argument('--champ', default=None)
    ap.add_argument('--full', action='store_true')
    args = ap.parse_args()

    A, na = load(args.a)
    B, nb = load(args.b)

    for tag, notes in (('A', na), ('B', nb)):
        for n in notes:
            if '⚠' in n:
                print(f'[{tag}] {n}')

    keys = sorted(set(A) & set(B))
    if args.pos is not None:
        keys = [k for k in keys if k[1] == args.pos]
    if args.champ:
        keys = [k for k in keys if k[0] == args.champ]
    only_a, only_b = sorted(set(A) - set(B)), sorted(set(B) - set(A))
    if only_a or only_b:
        print(f'⚠ 한쪽에만 있는 셀: A만 {len(only_a)} / B만 {len(only_b)} (비교에서 제외)')
    if not keys:
        print('비교할 셀이 없습니다.'); return

    # ── ① 요약 ──
    top1_changed, all3_same, top3_setsame = 0, 0, 0
    for k in keys:
        a1, b1 = build_str(A[k][0][2]), build_str(B[k][0][2])
        if a1 != b1:
            top1_changed += 1
        sa = [build_str(x[2]) for x in A[k]]
        sb = [build_str(x[2]) for x in B[k]]
        if sa == sb:
            all3_same += 1
        if set(sa) == set(sb):
            top3_setsame += 1

    n = len(keys)
    print(f'\n=== ① 요약 (셀 {n}개 = 챔프 × 포지션) ===')
    print(f'  1위 빌드가 바뀐 셀      : {top1_changed:5d}  ({top1_changed/n*100:.1f}%)')
    print(f'  상위3이 순서까지 동일   : {all3_same:5d}  ({all3_same/n*100:.1f}%)')
    print(f'  상위3 구성만 동일(순서X): {top3_setsame:5d}  ({top3_setsame/n*100:.1f}%)')

    # ── ② 아이템 등장 빈도 변화 ──
    # 상위3 빌드 전체에서 각 아이템이 몇 번 등장하는가. 추천 경향 변화를 가장 직관적으로 보여준다.
    ca, cb = Counter(), Counter()
    for k in keys:
        for _, _, items in A[k]:
            ca.update(items)
        for _, _, items in B[k]:
            cb.update(items)
    allit = sorted(set(ca) | set(cb), key=lambda i: cb[i] - ca[i])
    print(f'\n=== ② 아이템 등장 빈도 변화 (상위3 빌드 기준) ===')
    print(f'  {"아이템":<28} {"A":>6} {"B":>6} {"변화":>8}')
    for it in allit:
        d = cb[it] - ca[it]
        if d == 0 and not args.full:
            continue
        print(f'  {it:<28} {ca[it]:>6} {cb[it]:>6} {d:>+8}')
    if not args.full:
        same = [i for i in allit if cb[i] == ca[i]]
        if same:
            print(f'  (변화 없는 아이템 {len(same)}종 생략 — --full 로 전체 표시)')

    # ── ③ 셀별 변화 ──
    print(f'\n=== ③ 셀별 1위 빌드 ({"전체" if args.full else "바뀐 것만"}) ===')
    shown = 0
    for k in keys:
        a, b = A[k][0], B[k][0]
        sa, sb = build_str(a[2]), build_str(b[2])
        if sa == sb and not args.full:
            continue
        shown += 1
        print(f'  {k[0]:<22} pos{k[1]}')
        print(f'      A  {a[1]:.4f}  {sa}')
        print(f'      B  {b[1]:.4f}  {sb}')
    if shown == 0:
        print('  (1위 빌드가 바뀐 셀 없음)')


if __name__ == '__main__':
    main()
