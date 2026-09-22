# -*- coding: utf-8 -*-
"""26A: 로컬 %47(ScoreParameter 5384B) 에 대한 store/memset/memcpy 전수 → 기록 바이트 마스크(SRET_LIVE 재료).
주석본(_next/reach/d8eff0.ll · 줄번호 원문 m07.ll)을 통째 로드. gep 는 `%N = gep %47, i64 K` 축약형과 원문형 둘 다 처리."""
import io, re, sys, collections
sys.stdout.reconfigure(encoding='utf-8')
P = r'C:\tfm2mods\MIG\_next\reach\d8eff0.ll'
rows = []
for l in io.open(P, encoding='utf-8').read().split('\n'):
    m = re.match(r'\s*(\d+)\|(.*)', l)
    if m: rows.append((int(m.group(1)), m.group(2)))
# 1) %47 파생 포인터: {reg: offset}
off = {'%47': 0}
changed = True
while changed:
    changed = False
    for ln, s in rows:
        m = re.match(r'\s*(%\d+) = (?:gep|getelementptr(?: inbounds(?: nuw)?)?) (?:i8, )?(?:ptr )?(%\d+), i64 (-?\d+)', s)
        if m and m.group(2) in off and m.group(1) not in off:
            off[m.group(1)] = off[m.group(2)] + int(m.group(3)); changed = True
print('%47 파생 포인터', len(off))
SZ = {'i8':1,'i16':2,'i32':4,'i64':8,'ptr':8,'i1':1}
mask = bytearray(5384)
ev = []
def mark(a, n, ln, what):
    for i in range(a, min(a+n, 5384)): mask[i] = 1
    ev.append((a, n, ln, what))
for ln, s in rows:
    st = s.split(';')[0]
    m = re.match(r'\s*store (?:volatile )?(i\d+|ptr) [^,]+, ptr (%\d+)', st)
    if m and m.group(2) in off:
        mark(off[m.group(2)], SZ[m.group(1)], ln, st.strip()[:90]); continue
    m = re.search(r'llvm\.memset\.p0\.i64\(ptr [^%]*(%\d+), i8 (\d+), i64 (\d+)', st)
    if m and m.group(1) in off:
        mark(off[m.group(1)], int(m.group(3)), ln, 'memset ' + m.group(2)); continue
    m = re.search(r'llvm\.memcpy\.p0\.p0\.i64\(ptr [^%]*(%\d+), ptr [^%]*(%\d+), i64 (\d+)', st)
    if m and m.group(1) in off:
        mark(off[m.group(1)], int(m.group(3)), ln, 'memcpy from ' + m.group(2)); continue
    # 콜리에 &mut 로 넘기는 %47 파생 포인터(precompute_champion_powers 의 p:&mut ChampionScoreParameter)
    m = re.search(r'precompute_champion_powers\(i64 [^,]+, ptr [^,]+, ptr [^%]*(%\d+)', st)
    if m and m.group(1) in off:
        b = off[m.group(1)]
        # 콜리 writes: +0xb8..+0xd0 (4×i64) — m04.ll:53234 본문 store 184/192/200/208
        for k in (184,192,200,208): mark(b+k, 8, ln, 'callee precompute_champion_powers store p+%d' % k)
        continue
    # &mut 로 넘기는 다른 호출: reserve_internal_or_panic(vec=%47 파생) → ptr/cap 갱신
    m = re.search(r'reserve_internal_or_panic\(ptr [^%]*(%\d+)', st)
    if m and m.group(1) in off:
        b = off[m.group(1)]
        mark(b, 8, ln, 'callee reserve_internal_or_panic vec.ptr'); mark(b+16, 8, ln, 'callee reserve cap')
        continue
# 결과: 기록/미기록 구간
segs = []
cur = mask[0]; start = 0
for i in range(1, 5385):
    v = mask[i] if i < 5384 else None
    if v != cur:
        segs.append((start, i, cur)); start = i; cur = v
print('기록 구간(1)/미기록 구간(0):')
for a, b, v in segs:
    print('  [0x%x,0x%x) %dB %s' % (a, b, b-a, '기록' if v else '★미기록'))
print('기록 바이트 합', sum(mask), '/ 5384  미기록', 5384 - sum(mask))
# 오프셋별 첫 기록 근거(요약)
by = collections.defaultdict(list)
for a, n, ln, w in ev: by[a].append((ln, n, w))
import json
io.open(r'C:\Users\jungs\AppData\Local\Temp\claude\C--Users-jungs-Desktop-claude-tfm2--claude-worktrees-swap-order-button-style-fe9e04\8c613d5e-d69c-428f-aff5-fe06054da7f0\scratchpad\scratchpad26A\sret_stores.json', 'w', encoding='utf-8').write(json.dumps({'%x' % a: v for a, v in sorted(by.items())}, ensure_ascii=False, indent=0))
for a in sorted(by):
    v = by[a]
    print('  +0x%-5x n=%-3d 첫=%s' % (a, len(v), str(v[0])[:110]))
