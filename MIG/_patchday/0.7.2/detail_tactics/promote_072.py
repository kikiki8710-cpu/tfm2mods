"""0.7.2 상세전술 승격 — draft_targets + re/G*.json 판정 → tools/targets/*_0.7.2.json (2026-11-11)

근거 = re/G1~G6.md·json (창별 판정) · 사람 결정은 decisions_072.py 의 DECISIONS·NEW_SITES 에만 적는다.
- 판정 JSON 의 windows[창이름].fix 를 초안에 그대로 넣는다(추적기 제안과 달라지는 필드만 들어 있다).
- verdict 가 GONE·UNSURE·CHANGED 인데 DECISIONS 에 처리 방침이 없으면 멈춘다(사람 판정 없이 승격 금지).
- 전술 비활성: extras = sites 비우고 unsupported · knobs = subjects 의 그 전술 지점 전부에 unsupported(gen_sites 가 전술 단위로 뺀다).
- "review" 키를 전부 지운다. gen_sites.py 의 검증(바이트·asm·즉치·분기 유입)은 그대로 통과해야 한다.
사용: python -X utf8 promote_072.py [--dry]   (다시 돌려도 같은 결과 — 원천은 draft_targets)
0.7.1 판(promote_071.py)과 다른 점: 0.7.1 전용 새 창(0xf89cc2) 블록 삭제(이미 0.7.1 targets 에 들어 있어 초안이 승계) · knobs 비활성 지원 · NEW_SITES.
"""
import copy
import glob
import json
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
DRAFT = HERE / 'draft_targets'
OUT = Path(r'F:\claude\tfm2mods\tfm2_detail_tactics\tools\targets')
BASE = Path(r'F:\claude\tfm2mods\tfm2_detail_tactics\tools\patchday\baseline_0.7.1.json')
V = '0.7.2'
DRY = '--dry' in sys.argv

# 사람 결정: 창 이름 → 'keep'(그 창 유지) · 'drop'(창 삭제) · 전술 id → 'unsupported:<이유>'(이번 버전 비활성)
# NEW_SITES: [(전술 id, 기준 창 rva(복제 원본 · 초안 기준), {덮어쓸 필드})] — 유저 승인을 받은 새 창만
DECISIONS = {}
NEW_SITES = []
FIELD_FIX = {}
try:
    sys.path.insert(0, str(HERE))
    from decisions_072 import DECISIONS  # noqa: E402,F811
    from decisions_072 import NEW_SITES  # noqa: E402,F811
    from decisions_072 import FIELD_FIX  # noqa: E402,F811
except ImportError:
    pass

NAMES = {'knobs': 'knobs', 'subjects': 'subjects', 'extra_lane': 'extra_lane', 'extra_combat': 'extra_combat',
         'extra_ops': 'extra_ops', 'extra_verified': 'extra_verified'}
docs = {k: json.loads((DRAFT / f'{n}_{V}.json').read_text(encoding='utf-8')) for k, n in NAMES.items()}
base = json.loads(BASE.read_text(encoding='utf-8'))
track = json.loads((HERE / 'track.json').read_text(encoding='utf-8'))
twin = {w['name']: w for w in track['windows']}
log = []

verdicts = {}
for p in sorted(glob.glob(str(HERE / 're' / 'G[1-6].json'))):
    d = json.loads(Path(p).read_text(encoding='utf-8'))
    for tid, t in d['tactics'].items():
        assert tid not in verdicts, tid
        verdicts[tid] = t
ids = {t['id'] for t in base['tactics']}
assert set(verdicts) == ids, ('판정 빠짐/초과', sorted(ids - set(verdicts)), sorted(set(verdicts) - ids))

SUBJ = docs['subjects']['sites']
KEYS_KNOB = {'rva', 'asm', 'imm_at', 'src', 'team_chk', 'patch_start', 'prefix_hex', 'tf', 'form', 'calc', 'win_hex', 'first_len', 'first_asm', 'dynamic_hex', 'function'}
DOC_KEYS = {'dynamic_asm'}   # 판정 에이전트가 근거로 덧붙인 설명 필드 — targets 에 넣지 않는다
KEYS_EXTRA = {'rva', 'win_hex', 'first_len', 'first_asm', 'original_asm', 'src', 'team_chk', 'dynamic_hex', 'function', 'tf'}
unsup = []
drop = []
for w in base['windows']:
    name = w['name']
    tv = verdicts[w['tactic_id']]
    r = tv['windows'].get(name)
    tw = twin[name]
    dec = DECISIONS.get(name) or DECISIONS.get(w['tactic_id'])
    if isinstance(dec, str) and dec.startswith('unsupported'):
        continue
    if r is None:
        # AUTO 전술은 창 판정을 안 적어도 된다
        assert tw['auto'], ('창 판정 없음', w['tactic_id'], name)
        continue
    v = r['verdict']
    fix = {k: x for k, x in (r.get('fix') or {}).items() if k not in DOC_KEYS}
    fix.update(FIELD_FIX.get(name, {}))   # 메인이 gen_sites 검증에서 찾은 보정(판정 JSON 원문은 그대로 둔다)
    if v in ('GONE', 'UNSURE', 'CHANGED') and dec is None:
        raise SystemExit(f'사람 결정 필요: {w["tactic_id"]} {name} = {v}')
    if dec == 'drop':
        drop.append(w)
        continue
    if w['source'] == 'knobs':
        site = docs['knobs']['knobs'][w['t_index']]['sites'][w['s_index']]
        bad = set(fix) - KEYS_KNOB
        assert not bad, (name, bad)
        old_key = site['rva'] if tw['new_value_rva'] is not None else w['subj_key']
        ent = SUBJ.get(old_key)
        assert ent is not None, (name, old_key)
        for k, val in fix.items():
            if k in ('src', 'team_chk', 'patch_start', 'prefix_hex'):
                log.append(f'{name} subj {k}: {json.dumps(ent.get(k), ensure_ascii=False)[:80]} → {json.dumps(val, ensure_ascii=False)[:80]}')
                ent[k] = copy.deepcopy(val)
            else:
                log.append(f'{name} {k}: {json.dumps(site.get(k), ensure_ascii=False)[:80]} → {json.dumps(val, ensure_ascii=False)[:80]}')
                site[k] = copy.deepcopy(val)
        if 'rva' in fix and fix['rva'] != old_key:
            SUBJ[fix['rva']] = SUBJ.pop(old_key)
    else:
        site = docs[w['source']]['tactics'][w['t_index']]['sites'][w['s_index']]
        bad = set(fix) - KEYS_EXTRA
        assert not bad, (name, bad)
        for k, val in fix.items():
            log.append(f'{name} {k}: {json.dumps(site.get(k), ensure_ascii=False)[:80]} → {json.dumps(val, ensure_ascii=False)[:80]}')
            site[k] = copy.deepcopy(val)

# 이번 버전 비활성 전술
for key, dec in DECISIONS.items():
    if not (isinstance(dec, str) and dec.startswith('unsupported')) or key not in ids:
        continue
    why = dec.split(':', 1)[1]
    t = next(x for x in base['tactics'] if x['id'] == key)
    if t['source'] == 'knobs':
        k = next(x for x in docs['knobs']['knobs'] if x['id'] == t['knob'])
        for s in k['sites']:
            SUBJ.setdefault(s['rva'], {})['unsupported'] = why
    else:
        et = next(x for x in docs[t['source']]['tactics'] if x['audit_id'] == t['audit_id'])
        et['sites'] = []
        et['unsupported'] = why
    unsup.append(key)
    log.append(f'{key} unsupported: {why}')

# 창 하나만 빼는 결정 (지점 삭제)
for w in sorted(drop, key=lambda x: -x['s_index']):
    if w['source'] == 'knobs':
        k = docs['knobs']['knobs'][w['t_index']]
        s = k['sites'].pop(w['s_index'])
        SUBJ.pop(s['rva'], None)
    else:
        docs[w['source']]['tactics'][w['t_index']]['sites'].pop(w['s_index'])
    log.append(f'{w["name"]} 창 삭제')

# 노브 정의 정정(knob_fix) — 0.7.2 G5: base_defense_min_enemies 의 초안 default 2 는 추적기 오탐(팀 번호 범위검사 cmp rcx,1 을 짝지음)
KNOB_FIX = {}
try:
    from decisions_072 import KNOB_FIX  # noqa: E402,F811
except ImportError:
    pass
for tid, tv in verdicts.items():
    kf = dict(tv.get('knob_fix') or {}, **KNOB_FIX.get(tid, {}))
    if not kf:
        continue
    t = next(x for x in base['tactics'] if x['id'] == tid)
    k = next(x for x in docs['knobs']['knobs'] if x['id'] == t['knob'])
    for f, val in kf.items():
        if f == 'why':
            continue
        log.append(f'{tid} knob {f}: {k.get(f)} → {val}')
        k[f] = val

# 노브 새 창 — decisions_072.NEW_KNOB_SITES: [(knob id, site dict, subject dict)]
NEW_KNOB_SITES = []
try:
    from decisions_072 import NEW_KNOB_SITES  # noqa: E402,F811
except ImportError:
    pass
for kid, site, subj in NEW_KNOB_SITES:
    k = next(x for x in docs['knobs']['knobs'] if x['id'] == kid)
    assert all(s['rva'] != site['rva'] for s in k['sites']) and site['rva'] not in SUBJ, (kid, site['rva'])
    k['sites'].append(copy.deepcopy(dict(site, entry=kid)))
    SUBJ[site['rva']] = copy.deepcopy(subj)
    log.append(f'{kid} 노브 창 추가 {site["rva"]} {site["asm"]}')

# 새 창 (유저 승인분만 — decisions_072.NEW_SITES)
for tid, like_rva, over in NEW_SITES:
    t = next(x for x in base['tactics'] if x['id'] == tid)
    if t['source'] == 'knobs':
        raise SystemExit(f'knobs 새 창은 아직 처리 안 함: {tid}')
    et = next(x for x in docs[t['source']]['tactics'] if x['audit_id'] == t['audit_id'])
    assert all(s['rva'] != over['rva'] for s in et['sites']), (tid, over['rva'])
    ns = copy.deepcopy(next(s for s in et['sites'] if s['rva'] == like_rva))
    ns.update(copy.deepcopy(over))
    et['sites'].append(ns)
    log.append(f'{tid} 창 추가 {over["rva"]} (복제 원본 {like_rva})')


def strip_review(n):
    if isinstance(n, dict):
        n.pop('review', None)
        for v in n.values():
            strip_review(v)
    elif isinstance(n, list):
        for v in n:
            strip_review(v)


for d in docs.values():
    strip_review(d)

(HERE / 'promote_072.log.txt').write_text('\n'.join(log) + '\n', encoding='utf-8', newline='\n')
print(f'정정 {len(log)}줄 · 비활성 {unsup} · 창 삭제 {len(drop)} · 새 창 {len(NEW_SITES)} · 노브 새 창 {len(NEW_KNOB_SITES)}')
if not DRY:
    for k, n in NAMES.items():
        ind = 2 if n.startswith('extra_') else 1
        (OUT / f'{n}_{V}.json').write_text(json.dumps(docs[k], ensure_ascii=False, indent=ind) + '\n', encoding='utf-8', newline='\n')
    print('→', OUT)
