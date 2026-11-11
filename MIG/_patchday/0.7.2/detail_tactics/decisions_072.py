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

# 새 전술(0.7.2 신설 판단 · 유저 승인 2026-11-11 · 이름 = 유저 지정) — NEW4.json(창 RE) 결과로 채운다
NEW_TACTICS = []

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
