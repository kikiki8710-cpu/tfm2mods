"""0.7.0 상세전술 승격 — draft_targets → tools/targets/*_0.7.0.json (2026-10-28)

근거 = mods_report/tfm2_detail_tactics/21_0.7.0_대응조사.md + RE/2026-10-28_0.7.0-창판정_G1~G6_*.md.
- AUTO·A : "review" 키만 지운다.
- B      : 아래 FIX 표대로 RE 정정값을 넣는다(추적기 제안을 그대로 쓰지 않는다).
- D      : lane_opportunity_enemy_limit = sites 비움 + "unsupported"(gen_extras 가 UNSUPPORTED 로 보냄).
- 범위   : 0.6.5 와 같게 — 0.7.0 이 새로 부르는 자리(전술 로밍 경로·새 래퍼·3번째 라인 창)는 넣지 않는다.
           공유 함수로 옮겨 간 창은 Caller 게이트로 0.6.5 대응 호출처만 통과시킨다.
사용: python -X utf8 promote_070.py   (다시 돌려도 같은 결과 — 원천은 draft_targets)
"""
import copy, json
from pathlib import Path

HERE = Path(__file__).resolve().parent
DRAFT = HERE / 'draft_targets'
OUT = Path(r'F:\claude\tfm2mods\tfm2_detail_tactics\tools\targets')
V = '0.7.0'


def load(name):
    return json.loads((DRAFT / f'{name}_{V}.json').read_text(encoding='utf-8'))


def strip_review(n):
    if isinstance(n, dict):
        n.pop('review', None)
        for v in n.values():
            strip_review(v)
    elif isinstance(n, list):
        for v in n:
            strip_review(v)


def caller(off, cases):
    return {'kind': 'caller', 'return_off': off, 'cases': [{'ret_rva': r, 'src': s} for r, s in cases]}


log = []


def site_of(t, rva):
    m = [s for s in t['sites'] if s['rva'] == rva]
    assert len(m) == 1, (t['id'], rva, [s['rva'] for s in t['sites']])
    return m[0]


def setv(t, at, **kw):
    s = site_of(t, at)
    for k, v in kw.items():
        log.append(f"{t['id']} {at} {k}: {json.dumps(s.get(k), ensure_ascii=False)[:90]} → {json.dumps(v, ensure_ascii=False)[:90]}")
        s[k] = v
    return s


# ───────────────────────── knobs + subjects ─────────────────────────
knobs, subj = load('knobs'), load('subjects')
K = {k['id']: k for k in knobs['knobs']}
SJ = subj['sites']


def move_knob_site(kid, old, new, asm, imm_at, sj):
    s = next(x for x in K[kid]['sites'] if x['rva'] == old)
    log.append(f"{kid} rva {old} → {new} asm '{asm}' imm_at {imm_at}")
    s.update(rva=new, asm=asm, imm_at=imm_at)
    SJ.pop(old)
    SJ[new] = sj
    log.append(f"{kid} subjects[{new}] = {json.dumps(sj, ensure_ascii=False)}")


# [5] combat_avoid_hp (G2): cmp qword [rbp+0x330],0x29 로 접힘. 주체 = battle.rs 5번째 인자 [rbp+0x400], 팀 [rbp+0x318]
move_knob_site('combat_avoid_hp_41', '0xe52923', '0x1017856', 'cmp qword ptr [rbp + 0x330], 0x29', 7,
               {'src': 'mem:rbp:0x400', 'team_chk': 'mem:rbp:0x318'})
# [3] lane_retreat_final 창2 (G5): 새 함수 0x10ff610 의 0x1100e9e. 0.6.5 범위 = passive_line 본체 호출(ret 0x11035bc)만
#     (래퍼 0x1101120 경유 handler·정글 클로저 경로는 유저 미결 → Caller 게이트로 제외). return_off = 8push+0x2a8 = 0x2e8
move_knob_site('lane_retreat_final_50', '0xf9603d', '0x1100e9e', 'cmp rbx, 0x32', 3,
               {'src': caller('0x2e8', [('0x11035bc', 'mem:rbp:0x1f0')]), 'team_chk': 'mem:rbp:0x1c0'})
# [14] jungle_stop_hp (G5): Caller 재작성. 0.6.5 대응 6사례만(신규 0x10d1d84·클로저 0xe36b69 제외),
#      래퍼 중첩은 구 0xd4715b 와 같은 문맥인 0xe3f3ae 만(0xe3e7bc = 0.7.0 전술 로밍 경로 0xe3e72d~0xe3eaf9 안 → 제외)
JS = caller('0x188', [
    ('0xe3ecb9', 'stackmem:0x150:0x1bb0'),
    ('0x10cd549', 'stackmem:0x150:0x1e0'),
    ('0x10d087b', 'stackmem:0x150:0xc0'),
    ('0x10d13a7', 'stackmem:0x150:0x3490'),
    ('0x10d2073', 'stackmem:0x150:0x3490'),
    ('0x10e4188', caller('0x1b8', [('0xe3f3ae', 'stackmem:0x150:0x1bb0')])),
])
for s in K['jungle_stop_hp_41']['sites']:
    SJ[s['rva']]['src'] = copy.deepcopy(JS)
log.append(f"jungle_stop_hp_41 subjects ×4 src = {json.dumps(JS)}")
# [27] base_defense_min_enemies (G5): 0xebfbdb 사례 stackmem:0x30:0x3d0 → 0x480 (신규 소비처 4곳은 넣지 않음)
for s in K['base_defense_min_enemies_6']['sites']:
    for c in SJ[s['rva']]['src']['cases']:
        if c['ret_rva'] == '0xebfbdb':
            c['src'] = 'stackmem:0x30:0x480'
log.append("base_defense_min_enemies_6 subjects ×3 case 0xebfbdb → stackmem:0x30:0x480")
# [17] lane_opportunity_own_min (G6): 창 jb rel32 → rel8 — gen_sites 가 신 바이트로 다시 짠다. 문서 필드만 갱신
b = K['lane_opportunity_own_min_2']['sites'][0].get('branch')
if isinstance(b, dict):
    b.update(rva='0x103d1e1', bytes_hex='72 1f', jcc='jb 0x103d202')
    log.append("lane_opportunity_own_min_2 branch(문서) → 72 1f jb 0x103d202")
# [2] line_chase_target_guard_hp · [8] jg_gank_hp_start: knob 의 dyn 은 gen_sites 가 신 명령에서 다시 만든다
#     ([rsp+0x40]→push 보정 0x48, [rbp+0x110]) — 값 수정 불필요(G1·G5 가 지적한 dyn 오류는 extras 형식에만 해당).

# ───────────────────────── extras ─────────────────────────
docs = {p: load(f'extra_{p}') for p in ('lane', 'combat', 'ops', 'verified')}
T = {t['id']: t for d in docs.values() for t in d['tactics']}

# ── G1 ──
setv(T['support_cap_multiplier'], '0x1061763', tf={'calc': 'extra_combat::heal_r8'})
setv(T['support_cap_multiplier'], '0x1061775', tf={'calc': 'extra_combat::shield_r8'})
setv(T['unseen_self_risk_divisor'], '0x105f9bd',
     dynamic_hex='48 ba 56 55 55 55 55 55 55 55 4c 89 f0 48 f7 ea 48 89 d0 48 c1 e8 3f 48 01 d0 48 8b 44 24 f0')
setv(T['forward_damage_credit_percent'], '0x106058e', dynamic_hex='4c 8b 74 24 f0')
setv(T['skill_arrival_prediction_margin'], '0xe90433', team_chk='mem:rsp:0x68')
setv(T['tower_tank_hits'], '0x10da601', src=caller('0x1b8', [
    ('0x1064618', caller('0x208', [('0xeceb08', 'stack:0x1f0'), ('0x107cffc', 'stackmem:0x1b0:0x6a0'), ('0x1106893', 'stack:0x1f0')])),
    ('0x10800c0', 'stackmem:0x1b0:0x6a0'),
    ('0xf7e6ce', 'stackmem:0x1b0:0x458'),
    ('0x10de732', 'stack:0x218'),
    ('0x10de9cc', 'stack:0x198'),
]))
# ── G2 ──
setv(T['battle_risk_retreat_threshold'], '0x1015310', dynamic_hex='48 c1 e1 02 48 8d 0c 89 48 8b 4c 24 f0')
setv(T['battle_reposition_distance_scale'], '0x101705c', dynamic_hex='41 53 4c 8b 5c 24 f8 4d 29 de 41 5b')
setv(T['battle_reposition_distance_scale'], '0x1019634', dynamic_hex='41 53 4c 8b 5c 24 f8 4c 89 9d d0 00 00 00 41 5b')
setv(T['battle_reposition_distance_scale'], '0x101965b', dynamic_hex='41 53 4c 8b 5c 24 f8 4c 89 9d e8 00 00 00 41 5b')
for t in [T['battle_skill_safety_margin']]:  # A — 참고창(원본 유지, 패치 대상 아님) 문서값만 신 위치로
    for w in t.get('reference_windows', []):
        if w.get('rva') == '0xf84eb5':
            w.update(rva='0xf83934', bytes='49 81 cf a0 86 01 00', function='0xf83110', asm=['0xf83934: or r15, 0x186a0'])
            log.append("battle_skill_safety_margin reference_windows 0xf84eb5 → 0xf83934 (문서)")
# ── G3 ──
setv(T['lane_defense_support_margin'], '0x107ec65', rva='0xf31501', win_hex='48 ff c0 48 39 c3', first_len=3,
     src=caller('0x2a8', [('0x107edd4', 'reg:r15')]), dynamic_hex='41 53 4c 8b 5c 24 f8 4c 01 d8 41 5b',
     function=['0xf310b0', '0xf3220a'])
setv(T['lane_attack_move_allowance'], '0x1063d91', dynamic_hex='48 8d 0c 12 41 53 4c 8b 5c 24 f8 4c 89 da 41 5b 48 29 ca')
setv(T['lane_attack_move_allowance'], '0x107d52e', dynamic_hex='4b 8d 04 3f 41 53 4c 8b 5c 24 f8 4d 89 df 41 5b 49 29 c7')
# ── G4 ──
setv(T['roam_retry_extra_seconds'], '0xe42e1e', dynamic_hex='48 01 c9 41 53 4c 8b 5c 24 f8 4c 89 db 41 5b')
bc = T['battle_call_response_seconds']
assert [s['tf']['calc'] for s in bc['sites']] == ['extra_lane::call_early', 'extra_lane::call_diagnostic', 'extra_lane::call_general']
s0, s1, s2 = bc['sites']
# 초기창: 추적기는 진단창 자리(0xe6694b)로 오매핑 → 0xe67037. 창 시점 Player 레지스터가 없어 호출자 프레임(handler)에서 읽는다
log.append(f"battle_call_response_seconds 초기창 rva {s0['rva']} → 0xe67037")
s0.update(rva='0xe67037', win_hex='48 8d 14 89 48 39 d0', first_len=4,
          src=caller('0x9c8', [('0xe3e246', 'stackmem:0x9c0:0x1bb0')]), team_chk=None)
assert s0['dynamic_hex'] == '41 53 4c 8b 5c 24 f8 4c 89 da 41 5b'  # mov rdx,r11 (lea rdx,[rcx+rcx*4] 대체) 그대로
log.append("battle_call_response_seconds 진단창 0xe6694b src mem:rbp:0x8d0 → reg:r15")
s1.update(src='reg:r15')
log.append("battle_call_response_seconds 일반창 0xe66aa0 src → reg:r8, dyn → mov r12,r11")
s2.update(src='reg:r8', dynamic_hex='41 53 4c 8b 5c 24 f8 4d 89 dc 41 5b')
setv(T['relay_undying_limit_centiseconds'], '0xe6524f',
     src=caller('0x468', [('0xe3d913', 'stackmem:0x430:0x1bb0'), ('0xe42d67', 'stackmem:0x430:0x1bb0')]),
     team_chk=caller('0x468', [('0xe3d913', 'stackmem:0x430:0x1b60'), ('0xe42d67', 'stackmem:0x430:0x1b60')]))
setv(T['lane_exchange_advantage_ticks'], '0xe94401',
     src=caller('0x138', [('0xe3b256', 'stackmem:0x130:0x1bb0')]),
     team_chk=caller('0x138', [('0xe3b256', 'stackmem:0x130:0x1b60')]))
# N004: 0xd3dae1 창 삭제(반복자 → 0xfcdb60 호출 0xe55c81 로 치환), 5창 Caller 3사례
sl = T['same_lane_partner_radius']
assert sl['sites'][0]['rva'] == '0xd3dae1'
log.append("same_lane_partner_radius 창 0xd3dae1 삭제(MOVED → 0xfcdb60 호출 ret 0xe55c86 케이스로 흡수)")
sl['sites'] = sl['sites'][1:]
for s in sl['sites']:
    s['src'] = caller('0x58', [('0xe40c9e', 'mem:rbp:0x1bb0'), ('0xe54328', 'mem:rbp:0xe40'), ('0xe55c86', 'mem:rbp:0xe40')])
log.append("same_lane_partner_radius 5창 src = Caller(0x58,[0xe40c9e→mem:rbp:0x1bb0, 0xe54328→mem:rbp:0xe40, 0xe55c86→mem:rbp:0xe40])")
# D004: 새 함수 0xe72d80 인라인. 0.6.5 범위 = 고전 로밍 호출(0xe42f9a→ret 0xe42f9f)만, 전술 로밍 경로(ret 0xe3e841) 제외
setv(T['support_roam_adc_min_hp'], '0xd3fa45', rva='0xe736a2', win_hex='48 83 f8 33 48 8b 54 24 58', first_len=4,
     src=caller('0xd8', [('0xe42f9f', 'stackmem:0xa0:0x1bb0')]),
     team_chk=caller('0xd8', [('0xe42f9f', 'stackmem:0xa0:0x1b60')]),
     dynamic_hex='41 53 4c 8b 5c 24 f8 4c 39 d8 41 5b')
# ── G5 ──
setv(T['gank_reevaluate_seconds'], '0x10cd47c', dynamic_hex='41 53 4c 8b 5c 24 f8 4c 89 da 41 5b')
setv(T['camp_deadline_arrival_seconds'], '0x10e1c05', src=caller('0x78', [('0x10d22dc', 'stackmem:40:3490')]))
setv(T['epic_final_allies_min'], '0xed16d0', src='stack:58')
setv(T['boss_respawn_wait_seconds'], '0xee88e3', src=caller('0x68', [
    ('0xe56379', 'stack:40'), ('0xed2418', 'mem:rbp:e0'),
    ('0xf1590c', caller('0xc8', [('0xe3aa2a', 'mem:rbp:1bb0'), ('0xe4331b', 'mem:rbp:1bb0')]))]))
setv(T['boss_respawn_wait_seconds'], '0xee9bc0', src=caller('0x68', [
    ('0xe56339', 'stack:40'), ('0xed4aa6', 'mem:rbp:88'),
    ('0xf15926', caller('0xc8', [('0xe3aa2a', 'mem:rbp:1bb0'), ('0xe4331b', 'mem:rbp:1bb0')]))]))
mv = T['movement_skill_period_margin']
for s in mv['sites']:
    for c in s['src']['cases']:
        old = (c['ret_rva'], c['src'])
        if c['ret_rva'] == '0xe922b9':
            c.update(ret_rva='0xf78e7d', src='stackmem:0x2c0:0x6f0')
        elif c['ret_rva'] == '0x102151c':
            c['src'] = 'stackmem:0x2c0:0x980'
        elif c['ret_rva'] == '0xde4b40':
            c.update(ret_rva='0x104baf3', src='stackmem:0x2b0:0x380')
        if old != (c['ret_rva'], c['src']):
            log.append(f"movement_skill_period_margin {s['rva']} case {old} → {(c['ret_rva'], c['src'])}")
# ── G6 ──
setv(T['push_start_base'], '0x103c0b9', team_chk='stack:88')
setv(T['mia_notice_caution_profile'], '0xecc188', dynamic_hex='41 53 4c 8b 5c 24 f8 4e 39 9c ca f8 0b 00 00 41 5b')
# objective_power_allowance: 새 래퍼 0xeaefb0. 0.6.5 범위 = 구 0xdab3e0 호출처 3곳 대응(0xeb2045·0xebcc14·0xec6f2f)만,
#   신규 래퍼 0xeaf730(← cascade 0xebd01c) 경유 ret 0xeaf767 은 제외. return_off = 8push+0xf8 = 0x138
OPG = caller('0x138', [('0xeb204a', 'mem:rbp:0x40'), ('0xebcc19', 'mem:rbp:0x40'), ('0xec6f34', 'mem:rbp:0x40')])
setv(T['objective_power_allowance'], '0xdabbe7', rva='0xeaf05c', win_hex='bb c8 00 00 00', first_len=5, src=copy.deepcopy(OPG),
     dynamic_hex='41 53 4c 8b 5c 24 f8 44 89 db 41 5b',
     dynamic_asm='push r11; mov r11,qword ptr [rsp-8]; mov ebx,r11d; pop r11', first_asm='mov ebx, 0xc8')
setv(T['objective_power_allowance'], '0xdabcbd', rva='0xeaf150', win_hex='48 81 c3 c8 00 00 00', first_len=7, src=copy.deepcopy(OPG),
     dynamic_hex='41 53 4c 8b 5c 24 f8 4c 01 db 41 5b',
     dynamic_asm='push r11; mov r11,qword ptr [rsp-8]; add rbx,r11; pop r11', first_asm='add rbx, 0xc8')
setv(T['boss_poke_damage_factor'], '0xe96af2', rva='0xf3e184', win_hex='48 01 c0 49 3b 84 24 68 06 00 00', first_len=3, src='mem:rbp:1c0')
setv(T['structure_retreat_keep_hp'], '0xe922df', rva='0xf78eb4', win_hex='48 83 f8 41 0f 86 e9 f9 ff ff', first_len=4,
     src='mem:rbp:0x6f0', team_chk='mem:rbp:0x5d8')
# other_lane_collapse_profile: 범용 0x103b4c0 재작성 → 1·2번째 라인 4창(src reg:r14). 3번째 라인 2창은 유저 미결 → 넣지 않음
oc = T['other_lane_collapse_profile']
tmpl = {k: v for k, v in oc['sites'][0].items() if k not in ('rva', 'win_hex', 'first_len', 'src', 'team_chk', 'tf', 'dynamic_hex',
                                                              'window_name', 'dynamic_asm', 'first_asm', 'review')}
NEW_OC = [
    ('0x103b5d2', '49 81 7b 10 48 f4 ff ff', 8, 'collapse_metric', '41 52 4c 8b 54 24 f8 4d 39 53 10 41 5a', 'generic_metric1',
     'push r10; mov r10,qword ptr [rsp-8]; cmp qword ptr [r11 + 0x10],r10; pop r10', 'cmp qword ptr [r11 + 0x10], -0xbb8'),
    ('0x103b5dc', '41 83 7b 20 fe 7c 71', 5, 'collapse_stage', '41 52 4c 8b 54 24 f8 45 39 53 20 41 5a', 'generic_stage1',
     'push r10; mov r10,qword ptr [rsp-8]; cmp dword ptr [r11 + 0x20],r10d; pop r10', 'cmp dword ptr [r11 + 0x20], -2'),
    ('0x103b606', '49 81 78 10 48 f4 ff ff', 8, 'collapse_metric', '41 53 4c 8b 5c 24 f8 4d 39 58 10 41 5b', 'generic_metric2',
     'push r11; mov r11,qword ptr [rsp-8]; cmp qword ptr [r8 + 0x10],r11; pop r11', 'cmp qword ptr [r8 + 0x10], -0xbb8'),
    ('0x103b610', '41 83 78 20 fe 7c 3d', 5, 'collapse_stage', '41 53 4c 8b 5c 24 f8 45 39 58 20 41 5b', 'generic_stage2',
     'push r11; mov r11,qword ptr [rsp-8]; cmp dword ptr [r8 + 0x20],r11d; pop r11', 'cmp dword ptr [r8 + 0x20], -2'),
]
oc['sites'] = []
for rva, win, fl, calc, dyn, wn, dasm, fasm in NEW_OC:
    s = dict(copy.deepcopy(tmpl))
    s.update(rva=rva, win_hex=win, first_len=fl, src='reg:r14', team_chk=None, tf={'calc': f'extra_ops::{calc}'},
             dynamic_hex=dyn, window_name=wn, dynamic_asm=dasm, first_asm=fasm)
    oc['sites'].append(s)
log.append(f"other_lane_collapse_profile 16창 → 4창 {[x[0] for x in NEW_OC]} (src reg:r14, 3번째 라인 0x103b644/0x103b64e 미포함)")
# ── D ──
d = T['lane_opportunity_enemy_limit']
d['sites'] = []
d['unsupported'] = '0.7.0 판단 소멸(근접 적 >1 거절 → 교전 예측 위임) · 추적기 후보 0x103d093 = 팀 인덱스 범위검사(주입 시 ud2 위험)'
log.append("lane_opportunity_enemy_limit = D(비활성): sites [] + unsupported")

# ───────────────────────── 마무리 ─────────────────────────
for doc in [knobs, subj] + list(docs.values()):
    strip_review(doc)
OUT.mkdir(parents=True, exist_ok=True)
for name, doc in [('knobs', knobs), ('subjects', subj)] + [(f'extra_{p}', docs[p]) for p in docs]:
    ind = 2 if name.startswith('extra_') else 1  # 0.6.5 targets 서식 그대로
    (OUT / f'{name}_{V}.json').write_text(json.dumps(doc, ensure_ascii=False, indent=ind) + '\n', encoding='utf-8', newline='\n')
(HERE / 'promote_070.log.txt').write_text('\n'.join(log) + '\n', encoding='utf-8', newline='\n')
print(f"정정 {len(log)}줄 → {OUT}  (로그 = {HERE / 'promote_070.log.txt'})")
