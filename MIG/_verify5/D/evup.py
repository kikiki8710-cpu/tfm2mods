# 5차 배치D — ev4/ev3 → ev2 상향 목록. 경로가 실재하는지 검증하고 집계표를 찍는다.
# 사용: cd /c/tfm2mods/MIG && PYTHONIOENCODING=utf-8 python -X utf8 _verify5/D/evup.py
import io, json, sys

SPEC = r'C:\tfm2mods\MIG\_spec\specs20_v3.json'

# (i, field, idx, 근거파일:행표시)
UP = []


def add(i, f, idxs, ev):
    for j in idxs:
        UP.append((i, f, j, ev))


# ── specs[16] max_range_nearly_can_use  (o16.txt / o16b.txt, game==mine 122/122)
add(16, 'mem', [0, 1, 2, 3, 4, 9, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30], 2)
add(16, 'consts', [0, 1, 2, 3, 4, 5, 6], 2)
add(16, 'knobs', [0, 1, 2, 3, 4], 2)
# ── specs[17] DeathMatchBattle::new  (o17.txt / o17b.txt)
add(17, 'mem', list(range(0, 43)), 2)
add(17, 'consts', [0, 1, 2, 3], 2)
add(17, 'knobs', [0, 1, 2, 3], 2)
# ── specs[19] best_jungle_goal  (o19.txt 29/29 · o19b.txt 6/6 · o19c.txt 79/79)
add(19, 'mem', [0, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 15], 2)
add(19, 'consts', [0, 1, 2, 3, 4], 2)
add(19, 'knobs', [0, 1, 2, 3, 4, 5, 7], 2)
# ── specs[15] single_try_engage (함수 자체는 in:game_ai 라 미호출 — 피호출자·구조체로 간접 실측)
add(15, 'mem', [4, 5, 6, 7, 8, 11, 12, 13, 14, 15], 2)
add(15, 'consts', [0, 1, 3], 2)

# ── specs[18] : 함수 도달 불가. 오프셋만 런타임 주소산술로 실측 ⟹ 별도 등급
OFFONLY = [(18, 'mem', j) for j in [0, 1, 7, 8, 9, 10, 12, 13, 14, 15, 16, 17]]

d = json.load(io.open(SPEC, encoding='utf-8'))
specs = d['specs']
bad = []
from collections import Counter
was = Counter()
for (i, f, j, ev) in UP:
    arr = specs[i].get(f) or []
    if j >= len(arr):
        bad.append((i, f, j, 'OUT_OF_RANGE len=%d' % len(arr)))
        continue
    was[arr[j].get('ev')] += 1
for (i, f, j) in OFFONLY:
    arr = specs[i].get(f) or []
    if j >= len(arr):
        bad.append((i, f, j, 'OUT_OF_RANGE len=%d' % len(arr)))

print('상향 대상 %d행  (구 ev 분포: %s)' % (len(UP), dict(was)))
print('오프셋만 실측(18) %d행' % len(OFFONLY))
if bad:
    print('!! 경로 오류', bad)
else:
    print('경로 검증 OK — 전 행 실재')

# 전체 분포 재계산 (mem/consts/knobs 만, 브리핑 §1 표와 같은 분모)
upset = {(i, f, j) for (i, f, j, _) in UP}
for lo, hi, nm in [(15, 20, 'D(15~19)')]:
    before = Counter(); after = Counter()
    for i in range(lo, hi):
        for f in ['mem', 'consts', 'knobs']:
            for j, it in enumerate(specs[i].get(f) or []):
                e = it.get('ev')
                before[e] += 1
                after[2 if (i, f, j) in upset else e] += 1
    tot = sum(before.values())
    print('%s  분모=%d' % (nm, tot))
    print('  전: ' + '  '.join('ev%s=%d' % (k, before[k]) for k in sorted(before)))
    print('  후: ' + '  '.join('ev%s=%d' % (k, after[k]) for k in sorted(after)))
    g4b = sum(v for k, v in before.items() if k and k >= 4)
    g4a = sum(v for k, v in after.items() if k and k >= 4)
    print('  ev>=4 비율: %.1f%% → %.1f%%' % (100.0 * g4b / tot, 100.0 * g4a / tot))

# 함수별 상향 수
per = Counter()
for (i, f, j, _) in UP:
    per[i] += 1
print('함수별 상향: ' + '  '.join('specs[%d]=%d' % (i, per[i]) for i in sorted(per)))

# 상세 목록
print('\n--- 상향 목록 (JSON 경로 / 구 ev) ---')
for (i, f, j, ev) in UP:
    it = specs[i][f][j]
    key = it.get('name') or it.get('what') or it.get('value')
    off = it.get('offset')
    label = ('%s+%s' % (it.get('base'), off)) if off else str(key)
    print('/specs[%d]/%s[%d]  ev%s -> ev2   %s' % (i, f, j, it.get('ev'), label))
print('\n--- 18 오프셋만 실측 (함수 도달 X) ---')
for (i, f, j) in OFFONLY:
    it = specs[i][f][j]
    print('/specs[%d]/%s[%d]  ev%s   %s+%s  %s' % (i, f, j, it.get('ev'), it.get('base'), it.get('offset'), it.get('name')))
