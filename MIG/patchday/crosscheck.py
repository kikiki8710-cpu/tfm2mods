"""crosscheck.py — 리허설 SUMMARY.md 의 수치를 각 도구 산출물(json·log·md)에서 독립 재계산해 대조한다(검증 C 스크래치)."""
import collections
import io
import json
import os
import re
import sys

OUT = sys.argv[1] if len(sys.argv) > 1 else r'F:\claude\tfm2mods\MIG\_patchday\_rehearsal_0.6.4'
S = io.open(os.path.join(OUT, 'SUMMARY.md'), encoding='utf-8').read()
res = []


def chk(name, mine, pat, conv=int):
    m = re.search(pat, S, re.M)
    got = conv(m.group(1)) if m else None
    ok = got == mine
    res.append((ok, name, mine, got))


# 1 aidiff EDITED — aidiff.md 의 EDITED 헤더 수를 다른 방식(줄 단위 'EDITED' 포함 '### ')으로 센다
ad = io.open(os.path.join(OUT, 'aidiff.md'), encoding='utf-8').read().splitlines()
ed = sum(1 for l in ad if l.startswith('### ') and '**EDITED**' in l)
chk('aidiff EDITED', ed, r'EDITED (\d+)개\*\*')

# 2 check 합계 — 로그의 STALE/INVALID 행 수(들여쓴 'STALE '·'INVALID ' 줄)
lg = io.open(os.path.join(OUT, 'logs', '05_check.log'), encoding='utf-8').read().splitlines()
st = sum(1 for l in lg if re.match(r'^\s{4}(STALE|INVALID)\s', l))
hdr = sum(int(m.group(1)) for l in lg for m in [re.match(r'^\S+\s+PASS\s+\d+ / STALE·INVALID\s+(\d+)', l)] if m)
res.append((st == hdr, 'check 헤더합 vs STALE/INVALID 줄 수', st, hdr))
chk('check 합계', st, r'합계 STALE·INVALID = \*\*(\d+)\*\*')

# 3 repin — aligned 맵 원본에서 대상·해결·실패
m = json.load(open(os.path.join(OUT, 'repin_map.aligned.json'), encoding='utf-8'))
n = sum(len(v) for v in m.values())
rs = sum(1 for v in m.values() for e in v.values() if e.get('new'))
chk('repin 대상', n, r'^- 대상 (\d+) · 엔진 해결')
chk('repin 해결', rs, r'엔진 해결 (\d+) · 실패')
chk('repin 실패', n - rs, r'엔진 해결 \d+ · 실패 (\d+) · ')
# 사람 판정 필요 apply 대상/생성물 모드 분리 — 대기열 정의(실패·AGREE 아님·WEAK)를 독립 재계산
fc0 = json.load(open(os.path.join(OUT, 'repin_fncheck.json'), encoding='utf-8'))
GEN = ('tfm2_detail_tactics', 'tfm2_tactics_verify')
resid = [(mod, old) for mod, v in m.items() for old, e in v.items()
         if (not e.get('new')) or ((e.get('align') or {}).get('verdict') and not (e.get('align') or {}).get('verdict').startswith('AGREE'))
         or (fc0.get(mod, {}).get(old) or {}).get('verdict') == 'WEAK']
if '사람 판정 = ' in S:
    chk('repin apply 대상 사람 판정', sum(1 for x in resid if x[0] not in GEN), r'repin apply 대상 사람 판정 = (\d+)건')
    chk('repin 생성물 모드 판정', sum(1 for x in resid if x[0] in GEN), r'생성물 모드 (\d+)건\(')

# 4 fncheck — repin_fncheck.json verdict 분포
fc = json.load(open(os.path.join(OUT, 'repin_fncheck.json'), encoding='utf-8'))
fv = collections.Counter(r['verdict'] for v in fc.values() for r in v.values())
for k in ('STRONG', 'WEAK', 'CLONE'):
    chk('fncheck %s' % k, fv.get(k, 0), r'함수시작 교차검증.*?%s=(\d+)' % k)

# 5 offsets — 변화 모드·함수 수
of = json.load(open(os.path.join(OUT, 'offsets_shifts.json'), encoding='utf-8'))
nm = sum(1 for v in of.values() if v['status'] == 'shift')
nf = sum(len(v['rows']) for v in of.values())
chk('offsets 변화 모드', nm, r'변화 (\d+)개 모드')
chk('offsets 변화 함수', nf, r'변화 \d+개 모드 · (\d+)개 함수')

# 6 상세전술 — track.json tactics 배열에서 직접
tj = json.load(open(os.path.join(OUT, 'detail_tactics', 'track.json'), encoding='utf-8'))
tac = tj['tactics']
va = sum(1 for t in tac if t['verdict'] == 'AUTO')
chk('상세전술 AUTO', va, r'자동확정\(AUTO\) (\d+)')
chk('상세전술 REVIEW', len(tac) - va, r'검토 필요\(REVIEW\) (\d+)')
wins = collections.Counter()
for t in tac:
    pass
# 창 판정: windows 섹션(창 단위)에서 직접
W = tj.get('windows')
if isinstance(W, list):
    wc = collections.Counter(w.get('code') or w.get('verdict') for w in W)
elif isinstance(W, dict):
    wc = collections.Counter((w.get('code') or w.get('verdict')) for w in W.values())
else:
    wc = collections.Counter()
for code in ('SAME', 'REG_RENAME', 'LOGIC_CHANGED', 'COPIES', 'SITE_MISSING', 'OWNER_MISSING'):
    chk('상세전술 창 %s' % code, wc.get(code, 0), r'^\| `%s` \| [^|]+ \| [^|]+ \| (\d+) \|' % code)
wst = collections.Counter(t['worst'] for t in tac)
for code in ('REG_RENAME', 'LOGIC_CHANGED', 'SITE_MISSING', 'OWNER_MISSING'):
    chk('상세전술 worst %s' % code, wst.get(code, 0), r'^\| `%s` \| [^|]+ \| \**(\d+)\** \|' % code)
chk('상세전술 기본값 변경 전술 수', sum(1 for t in tac if t.get('value_changes')), r'게임이 기본값을 바꾼 전술 \((\d+)\)')

# 7 주소록 — addrmap.json rows 에서 직접
am = json.load(open(os.path.join(OUT, 'addrmap', 'addrmap.json'), encoding='utf-8'))
gc = collections.Counter(r['grade'] for r in am['rows'])
chk('주소록 행 수', len(am['rows']), r'^- (\d+)건 · 등급 분포')
for g in ('SAME', 'AMBIGUOUS', 'MOVED_BYTES_CHANGED', 'MISSING', 'FN_EDITED'):
    chk('주소록 %s' % g, gc.get(g, 0), r'\*\*%s\*\* (\d+)' % g)

# 8 핫픽스 표 행 수
hj = json.load(open(r'F:\claude\tfm2mods\MIG\patchday\hotfix_exposure_0.6.5.json', encoding='utf-8'))
sec8 = S.split('## 8.')[1].split('## 9.')[0]
trs = [l for l in sec8.splitlines() if l.startswith('| ') and not l.startswith('| 모드')]
res.append((len(trs) == len(hj['rows']), '핫픽스 표 행 수(json rows)', len(hj['rows']), len(trs)))

bad = 0
for ok, name, mine, got in res:
    print('%s  %-34s 독립=%s  SUMMARY=%s' % ('OK ' if ok else '★NG', name, mine, got))
    bad += (not ok)
print('대조 %d건 · 불일치 %d' % (len(res), bad))
sys.exit(1 if bad else 0)
