# -*- coding: utf-8 -*-
"""0.7.0 상세전술 적용 범위 확장(유저 결정 2026-10-28 「넓히자」) + 실측 1판 회귀 수정.

순서: promote_070.py(draft → tools/targets) → **이 스크립트**(tools/targets 제자리 수정) → gen_sites.py.
여러 번 돌려도 결과가 같다(사례가 이미 있으면 다시 넣지 않고, 값이 다르면 중단).
근거 = mods_report/tfm2_detail_tactics/RE/2026-10-28_0.7.0-범위확장.md
"""
import json, sys
from pathlib import Path

T = Path(r'F:\claude\tfm2mods\tfm2_detail_tactics\tools\targets')
log = []


def load(n):
    return json.loads((T / n).read_text(encoding='utf-8'))


def save(n, d):
    ind = 2 if n.startswith('extra_') else 1  # promote_070.py 와 같은 서식
    (T / n).write_text(json.dumps(d, ensure_ascii=False, indent=ind) + '\n', encoding='utf-8', newline='\n')


def add_case(src, ret, sub, where):
    """Caller src 에 사례 1개를 넣는다(이미 같으면 그대로, 다르면 중단)."""
    assert isinstance(src, dict) and src['kind'] == 'caller', where
    for c in src['cases']:
        if c['ret_rva'] == ret:
            if c['src'] != sub:
                sys.exit(f'{where}: 사례 {ret} 값 충돌 {c["src"]} != {sub}')
            return
    src['cases'].append({'ret_rva': ret, 'src': sub})
    log.append(f'{where}: +case {ret} -> {json.dumps(sub, ensure_ascii=False)}')


def set_src(site, key, old, new, where):
    cur = site.get(key)
    if cur == new:
        return
    if cur != old:
        sys.exit(f'{where}.{key}: 예상 {old} 인데 {cur}')
    site[key] = new
    log.append(f'{where}.{key}: {json.dumps(old, ensure_ascii=False)} -> {json.dumps(new, ensure_ascii=False)}')


subj = load('subjects_0.7.0.json')
S = subj['sites']

# ── A-1 jungle_stop_hp_41: 0x10e4a30 호출처 8곳 전부(신규 0x10d1d84·클로저 0xe36b69·래퍼 중첩 0xe3e7bc)
for r in ('0x10e4c7d', '0x10e4c71', '0x10e4bbf', '0x10e4bb7'):
    src = S[r]['src']
    add_case(src, '0x10d1d84', 'stackmem:0x150:0x3490', f'jungle_stop {r}')
    add_case(src, '0xe36b69', 'stack:0x160', f'jungle_stop {r}')
    nested = next(c for c in src['cases'] if c['ret_rva'] == '0x10e4188')['src']
    add_case(nested, '0xe3e7bc', 'stackmem:0x150:0x1bb0', f'jungle_stop {r} 래퍼중첩')

# ── A-2 base_defense_min_enemies_6: handler 신규 호출 0xe3cf7b 만(relays 2곳·0xec8f05 중첩은 Player 미확정 → 게이트 유지)
for r in ('0xe9c82c', '0xe9c8cc', '0xe9c98c'):
    add_case(S[r]['src'], '0xe3cf7b', 'stackmem:0x30:0x1bb0', f'base_defense {r}')

# ── A-3 lane_retreat_final 창2: 래퍼 0x1101120 경유(ret 0x11012f2). Src 는 owner 자기 프레임(arg4 Player)
add_case(S['0x1100e9e']['src'], '0x11012f2', 'mem:rbp:0x1f0', 'lane_retreat 0x1100e9e')

# ── 회귀 수정(실측 1판 판독): 호출자 프레임 Src 3전술 10창
for r in ('0xe741d0', '0xe74236', '0xe74299', '0xe742fc', '0xe74315'):
    set_src(S[r], 'src', 'stackmem:0x50:0x1768', 'stackmem:0x50:0x1bb0', f'liner_roam_lane_healthy {r}')
for r in ('0xfc111c', '0xfc1128', '0xfc1134', '0xfc1140'):
    set_src(S[r], 'src', 'mem:rbp:0x548', 'mem:rbp:0x568', f'minion_damage_ratio {r}')
set_src(S['0x10e386c'], 'src', 'mem:rbp:0x2e50',
        {'kind': 'caller', 'return_off': '0x28', 'cases': [
            {'ret_rva': '0x10d151a', 'src': 'mem:rbp:0x3490'},
            {'ret_rva': '0x10cf29d', 'src': 'stackmem:0x100:0x3490'}]},
        'jungle_safe_radius 0x10e386c')

# ── B liner_roam_self_hp_41: 전술 로밍 경로 자기 HP 검사 창(handler 자기 프레임)
NEW_HP = '0xe3e78c'
want = {'src': 'mem:rbp:0x1bb0', 'team_chk': 'mem:rbp:0x1b60'}
if S.get(NEW_HP) not in (None, want):
    sys.exit(f'subjects {NEW_HP} 충돌')
if NEW_HP not in S:
    S[NEW_HP] = want; log.append(f'subjects +{NEW_HP} {want}')
save('subjects_0.7.0.json', subj)

knobs = load('knobs_0.7.0.json')
k = next(k for k in knobs['knobs'] if k['id'] == 'liner_roam_self_hp_41')
site = {
    'form': 'extra', 'entry': 'liner_roam_self_hp_tactical', 'rva': NEW_HP,
    'win_hex': '4d 85 f6 0f 84 66 03 00 00', 'first_len': 3, 'first_asm': 'test r14, r14',
    'dynamic_hex': '41 53 4c 8b 5c 24 f8 4d 85 db 41 5b',
    'dynamic_asm': 'push r11; mov r11,qword ptr [rsp-8]; test r11,r11; pop r11',
    'tf': {'calc': 'extra_lane::tactical_roam_self_hp'},
    'role': '0.7.0 전술 로밍 경로(Player+0x2c5) — 원본엔 HP 검사 없음. 기본값이 아닐 때만 HP%<값 이면 거부(je 0xe3eafb)',
    'operand_meaning': 'r14 = 자기 엔티티([[rbp+0x1b48]+[rbp+0x1b90]*8]). calc 가 HP%=[r14+0x668]*100/max([r14+0x620],1) < 값 이면 0, 아니면 r14',
}
have = [s for s in k['sites'] if s['rva'] == NEW_HP]
if have:
    if have[0] != site:
        sys.exit('knobs liner_roam_self_hp 추가창 충돌')
else:
    k['sites'].append(site); log.append(f'knobs liner_roam_self_hp_41 +site {NEW_HP}')
save('knobs_0.7.0.json', knobs)

# ── extras
lane = load('extra_lane_0.7.0.json')
d004 = next(t for t in lane['tactics'] if t['id'] == 'support_roam_adc_min_hp')
assert len(d004['sites']) == 1
add_case(d004['sites'][0]['src'], '0xe3e841', 'stackmem:0xa0:0x1bb0', 'support_roam_adc src')
add_case(d004['sites'][0]['team_chk'], '0xe3e841', 'stackmem:0xa0:0x1b60', 'support_roam_adc team_chk')
save('extra_lane_0.7.0.json', lane)

ops = load('extra_ops_0.7.0.json')
c007 = next(t for t in ops['tactics'] if t['id'] == 'objective_power_allowance')
for s in c007['sites']:
    add_case(s['src'], '0xeaf767', 'mem:rbp:0x40', f'objective_power {s["rva"]}')
c023 = next(t for t in ops['tactics'] if t['id'] == 'other_lane_collapse_profile')
note = c023['sites'][0]['notes']
third = [
    {'notes': note, 'rva': '0x103b644', 'win_hex': '48 81 7a 10 48 f4 ff ff', 'first_len': 8, 'src': 'reg:r14', 'team_chk': None,
     'tf': {'calc': 'extra_ops::collapse_metric'}, 'dynamic_hex': '41 53 4c 8b 5c 24 f8 4c 39 5a 10 41 5b',
     'window_name': 'generic_metric3', 'dynamic_asm': 'push r11; mov r11,qword ptr [rsp-8]; cmp qword ptr [rdx + 0x10],r11; pop r11',
     'first_asm': 'cmp qword ptr [rdx + 0x10], -0xbb8'},
    {'notes': note, 'rva': '0x103b64e', 'win_hex': '83 7a 20 fe 7d 09', 'first_len': 4, 'src': 'reg:r14', 'team_chk': None,
     'tf': {'calc': 'extra_ops::collapse_stage'}, 'dynamic_hex': '41 53 4c 8b 5c 24 f8 44 39 5a 20 41 5b',
     'window_name': 'generic_stage3', 'dynamic_asm': 'push r11; mov r11,qword ptr [rsp-8]; cmp dword ptr [rdx + 0x20],r11d; pop r11',
     'first_asm': 'cmp dword ptr [rdx + 0x20], -2'},
]
for w in third:
    h = [s for s in c023['sites'] if s['rva'] == w['rva']]
    if h:
        if h[0] != w:
            sys.exit(f'other_lane_collapse {w["rva"]} 충돌')
    else:
        c023['sites'].append(w); log.append(f'other_lane_collapse +site {w["rva"]}')
save('extra_ops_0.7.0.json', ops)

print('\n'.join(log) if log else '변경 없음(이미 적용됨)')
