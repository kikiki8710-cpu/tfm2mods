"""0.7.2 상세전술 사람 결정 (promote_072.py 가 읽는다 · 2026-11-11)

창 판정 G1~G6 = CHANGED·GONE·UNSURE 0 → DECISIONS 는 비어 있다.
lane_opportunity_enemy_limit 는 0.7.0 부터 extras 초안에 unsupported 가 이미 들어 있어 따로 적지 않는다(G6 = DISABLE 유지).
"""

DECISIONS = {}

# 창 필드 보정 — gen_sites 검증에서 메인이 찾은 것(판정 JSON 원문은 고치지 않는다). 창 이름 → {필드: 값}
FIELD_FIX = {
    # 신 창 `cmp qword ptr [rsp + 0xb8], 0x41` — 변위가 disp8(0x40)→disp32(0xb8) 로 늘어 즉치 위치 5→8 (G1 은 src·team_chk 만 고침)
    'line_chase_target_guard_hp_65@0xfa63e7': {'imm_at': 8},
}

# 노브 정의 보정(전술 id → 필드) — tactics.rs 손 표와 맞춘다(gen_sites 는 default 만 대조한다).
KNOB_FIX = {
    # 라인 수비 사본 창 0x11519b7 = `cmp rax, v+1` → v=127 이면 128 이라 1바이트 즉치를 넘는다. HP% 100 이상은 전부 「항상」이라 뜻 변화 없음.
    'combat_retreat_hp': {'max': 126},
}

# 전술 정의 보정(extras) — 유저 승인 2026-11-11 「추천4개에 설명보강까지」
TACTIC_FIX = {
    # G4 MAJOR: 0.7.2 평가 입구에 「근처에 보이는 적 챔프」 선행 검사 신설(0xe6b8c1) — 값·범위·기본값은 그대로
    'pre_gank_engage_profile': {
        'desc': '100이 원본 HP35·거리200000입니다. 높을수록 선공 허용 범위가 넓습니다. '
                '0.7.2부터는 근처에 보이는 적 챔피언이 있을 때만 선공을 검토합니다(그 거리는 「갱 시도 시 적 확인 거리」).',
    },
}

# 새 전술(0.7.2 신설 판단 · 유저 승인 2026-11-11 · 이름 = 유저 지정) — 창 = re/NEW4.json(창 RE · 18창 검사·등가 통과)
# extra_verified 문서 끝에 붙인다: gen_extras 가 lane/combat/ops 를 감사 ID 로 정렬하므로 거기 넣으면 기존 verified 12개 번호가 밀린다.
import json as _json
import os as _os

_N4 = {t['id']: t for t in _json.load(open(_os.path.join(_os.path.dirname(_os.path.abspath(__file__)), 're', 'NEW4.json'), encoding='utf-8'))['tactics']}
_TF = {  # 콜백 이름 정리(E003 = 「적 접근 시 교전 요청」 — RE 원안의 objective_* 이름은 뜻이 좁아 바꿈)
    'extra_ops::objective_call_wide': 'extra_ops::approach_call_objective',
    'extra_ops::objective_call_enemy': 'extra_ops::approach_call_enemy',
    'extra_ops::objective_call_enemy_free': 'extra_ops::approach_call_enemy_free',
}


def _new(src_id, audit, tid, name, desc, tab, scope, levels, drop=()):
    t = _N4[src_id]
    lo, df, hi, mn, mx = levels
    sites = []
    for s in t['sites']:
        if s['rva'] in drop:
            continue
        s = dict(s)
        calc = s['tf']['calc']
        s['tf'] = {'calc': _TF.get(calc, calc)}
        s['window_name'] = f'{tid}@{s["rva"]}'
        sites.append(s)
    return {'audit_id': audit, 'id': tid, 'name': name, 'desc': desc, 'scope': scope, 'default': df, 'low': lo, 'high': hi,
            'min': mn, 'max': mx, 'kind': 'Value', 'tab': tab, 'pos_mask': 31, 'sites': sites}


NEW_TACTICS = [
    ('extra_verified', _new('nexus_finish_start_range', 'E001', 'nexus_finish_start_range', '넥서스 마무리 시작 거리(%)',
                            '적 넥서스나 남은 건물이 이 거리 안에 있어야 끝까지 밀어붙인다(100 = 원본 약 25만). 높을수록 멀리서도 끝내러 간다',
                            '푸시·타워', 'Player', (70, 100, 150, 25, 400))),
    ('extra_verified', _new('weak_team_finish_caution', 'E002', 'weak_team_finish_caution', '전력 부족 시 마무리 신중도(%)',
                            '팀 능력이 낮을수록 넥서스 마무리를 더 조심한다(100 = 원본 · 최대 1.95배 엄격). 0이면 능력과 상관없이 시도한다(0.7.1 방식)',
                            '푸시·타워', 'Team', (50, 100, 130, 0, 130))),
    ('extra_verified', _new('objective_engage_call_range', 'E003', 'enemy_approach_engage_range', '적 접근 시 교전 요청 거리(%)',
                            '체력 60% 넘을 때 보이는 적 챔피언이 이 거리 안에 오면 교전을 제안한다(100 = 원본 25만 · 에픽·세르펜을 노릴 땐 적이 그 근처 23만 안이어야 함). 높을수록 먼 적에게도 건다',
                            '전투·생존', 'Player', (75, 100, 125, 25, 400),
                            drop=('0xe6a96d',))),  # 180000² 갈래 = 입구 점프표가 상태 1 만 들여 정적으로 도달 불가(NEW4 §3)
    ('extra_verified', _new('pre_gank_enemy_check_range', 'E004', 'pre_gank_enemy_check_range', '갱 시도 시 적 확인 거리(%)',
                            '갱 도착 전 이 거리 안에 보이는 적 챔피언이 있어야 선공을 검토한다(100 = 원본 25만). 높을수록 멀리 있는 적에게도 선공을 노린다',
                            '갱·로밍', 'Player', (75, 100, 150, 25, 400))),
]

# 새 extra 창 — 없음
NEW_SITES = []

# 노브 범위 유지 창(G1 new_paths) — 0.7.2 가 라인 수비 사전 판정 0x1151760(호출 0xf3bddf·0xef7f8b)을 새로 갈라내면서
# 같은 HP 35% 판정 사본을 따로 갖게 됐다. 안 넣으면 라인 수비 중에는 값을 바꿔도 35% 그대로다(0.7.1 은 같은 창을 지나갔다).
NEW_KNOB_SITES = [
    ('combat_retreat_hp_35', {
        'rva': '0x11519b7', 'asm': 'cmp rax, 0x24', 'imm_at': 3, 'width': 1, 'tf': "'v', 1",
        'role': '0.7.2 라인 수비 사전 판정 사본(0x1151760)',
        'operand_meaning': 'HP% (jb = HP% < 36 = 35 이하면 거부 판단으로 안 감)',
        'branch': {'rva': None, 'bytes_hex': '72 25', 'jcc': 'jb 0x11519e2',
                   'taken_means': 'HP% <= 35 → 0x11519e2 후퇴 점수 계산 진행',
                   'not_taken_means': 'HP% > 35 → 0xfcb940 넥서스 방어 판단 참이면 (1, -99999) 거부'},
    }, {'src': 'reg:r14', 'team_chk': 'reg:rbx'}),
    ('self_risk_ignore_hp_35', {
        'rva': '0x1147abd', 'asm': 'imul rcx, rcx, 0x23', 'imm_at': 3, 'width': 1, 'tf': "'v', 0",
        'role': '0.7.2 라인 수비 간이 위험 판정 사본(0x1147850 · 유일 호출 0x1151997)',
        'operand_meaning': 'rcx = max(maxHP,1)×35 (교차곱) · 다음 cmp rax,rcx; jbe',
        'branch': None,
    }, {'src': 'mem:rbp:0x140'}),
]
