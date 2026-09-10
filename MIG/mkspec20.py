#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""mkspec20.py — 20함수 명세를 하나로 합쳐 `specs20.json` 을 만든다.

합치는 것 셋:
  ①`_spec/r1..r6` 중 **함수별 최신 판본**(r6 8개 · r4 12개)
  ②`_spec/resolved_2026-09-10.json` — 미확정 해소 오버레이(6갈래 조사 결과)
  ③`TFM2 AI 함수 지도` 아티팩트의 exe 주소(있는 것만 · 15/20)

왜 하나로 합치나: 라운드별 JSON 은 "같은 함수의 여러 판본"이라 읽는 쪽이 매번
어느 게 최신인지 골라야 했다. 지도와 붙이려면 함수당 하나여야 한다.
"""
import io
import json
import os
import re
import sys

if hasattr(sys.stdout, 'reconfigure'):
    sys.stdout.reconfigure(encoding='utf-8', errors='replace')

HERE = os.path.dirname(os.path.abspath(__file__))
SPEC = os.path.join(HERE, '_spec')
ROUNDS = ['r6', 'r5', 'r4', 'r3', 'r2', 'r1']
OUT = os.path.join(HERE, '_spec', 'specs20.json')

# 지도(아티팩트) 저장본 — exe 주소 조인용. 없으면 주소 없이 만든다.
MAP_HTML = os.path.join(
    r'C:\Users\jungs\.claude\projects',
    r'C--Users-jungs-Desktop-claude-tfm2--claude-worktrees-silerus-mode-continue-aed092',
    r'e778e0a6-0076-4d33-8f11-c684449b5443\tool-results',
    r'artifact-6beb477e-1789022031-0994.html')

# 명세 파일명 → (지도 모듈, 지도 이름). 지도는 exe 기준이라 소스 경로와 모듈명이 다르다.
MAPKEY = {
    'abstract_input__ult': ('abstract_input', 'ult'),
    'action_score__calculate_jungle_action_score': ('action_score', 'calculate_jungle_action_score'),
    'attack_nexus__sub_plan': ('attack_nexus', 'sub_plan'),
    'defense_nexus__handle_line_defense': ('defense_nexus', 'handle_line_defense'),
    'engage__v2_response_retreat_stance': ('engage', 'v2_response_retreat_stance'),
    'epic_hunt_and_poke__is_end': ('hunt_and_poke', 'is_end'),
    'fight_check__check_favorable_engage_formation': ('fight_check', 'check_favorable_engage_formation'),
    'fight_model__should_end_object_finish_kill_priority_battle':
        ('fight_model', 'should_end_object_finish_kill_priority_battle'),
    'handler__v3_fall_back_to_passive': ('handler', 'v3_fall_back_to_passive'),
    'handler_chat__handle_chat': ('chat', 'handle_chat'),
    'line_gank_ganker__update': ('ganker', 'update'),
    'modes__single_try_engage': ('modes', 'single_try_engage'),
    'old_battle__max_range_nearly_can_use': ('battle', 'max_range_nearly_can_use'),
    'old_epic__v3_epicops_buff_window': ('epic', 'v3_epicops_buff_window'),
    'passive_jungle__best_jungle_goal': ('passive_jungle', 'best_jungle_goal'),
}
# 지도에 **없는** 5개 — 억지로 붙이지 않고 사유를 적는다.
NOMAP = {
    'buff_value__defensive_crisis':
        '지도(exe 641개)에 `buff_value::defensive_crisis` 항목이 없다. 같은 모듈의 다른 함수는 있으므로 '
        '이 함수만 exe 에서 호출부에 인라인됐거나 지도의 AI 계층 선별에서 빠진 것으로 보인다(미확인).',
    'dive_episode__v50_fold_dive_episode':
        '지도에 `dive_episode` 모듈 자체가 없다. 진단·통계 경로라 AI 판단 계층 선별에서 제외된 것으로 보인다(미확인).',
    'line_gank_cover__target_bush_v30':
        '★인라인 확정 — 6차 실측에서 `LineGankerPlan::update` 안에 전량 인라인돼 별도 define 이 없다. '
        '지도의 `cover` 모듈에는 `next_plan` 하나뿐이다.',
    'epic_hunt_and_battle__sub_plan':
        '지도에 `hunt_and_battle::hunt_and_battle`(caddr ccc010 / ccc3c0, 591B, 근거=기존기록)가 있는데 '
        '이름이 달라 자동 조인이 안 된다. 같은 것인지 미확인이라 붙이지 않았다.',
    'old_death_battle__new':
        '지도의 `death_battle` 모듈에 `new` 항목이 없다(21개 중 미확정 다수). 생성자라 선별에서 빠졌을 수 있다(미확인).',
}
# 오버레이 키 ↔ 명세 파일명
OVKEY = {
    'defensive_crisis': 'buff_value__defensive_crisis',
    'v2_response_retreat_stance': 'engage__v2_response_retreat_stance',
    'epic_hunt_and_poke__is_end': 'epic_hunt_and_poke__is_end',
    'best_jungle_goal': 'passive_jungle__best_jungle_goal',
    'handle_line_defense': 'defense_nexus__handle_line_defense',
    'single_try_engage': 'modes__single_try_engage',
    'should_end_object_finish_kill_priority_battle':
        'fight_model__should_end_object_finish_kill_priority_battle',
    'check_favorable_engage_formation': 'fight_check__check_favorable_engage_formation',
    'max_range_nearly_can_use': 'old_battle__max_range_nearly_can_use',
    'abstract_input__ult': 'abstract_input__ult',
    'calculate_jungle_action_score': 'action_score__calculate_jungle_action_score',
    'target_bush_v30': 'line_gank_cover__target_bush_v30',
    'attack_nexus__sub_plan': 'attack_nexus__sub_plan',
    'v50_fold_dive_episode': 'dive_episode__v50_fold_dive_episode',
    'death_battle__new': 'old_death_battle__new',
    'update__line_gank_ganker': 'line_gank_ganker__update',
    'v3_fall_back_to_passive': 'handler__v3_fall_back_to_passive',
    'handle_chat': 'handler_chat__handle_chat',
    'hunt_and_battle__sub_plan': 'epic_hunt_and_battle__sub_plan',
    'v3_epicops_buff_window2': 'old_epic__v3_epicops_buff_window',
}
# 소스 경로 → 계층
LAYER = [
    (r'abstract_input', '입력 생성'),
    (r'action_score|buff_value|fight_check', '점수화·술어'),
    (r'plan_legacy\\handler', '플랜 핸들러'),
    (r'plan_legacy\\old', '레거시 플랜'),
]


def layer_of(src):
    for pat, name in LAYER:
        if re.search(pat, src or ''):
            return name
    return '기타'


def load_map():
    """(모듈,이름) -> 대표 엔트리, 그리고 (모듈,이름) -> 전체 목록 두 벌을 낸다."""
    if not os.path.exists(MAP_HTML):
        return {}, {}
    h = io.open(MAP_HTML, encoding='utf-8', errors='replace').read()
    m = re.search(r'<script id="fndata" type="application/json">(.*?)</script>', h, re.S)
    if not m:
        return {}, {}
    out, allof = {}, {}
    for e in json.loads(m.group(1)):
        if e.get('n'):
            out.setdefault((e['m'], e['n']), e)
            allof.setdefault((e['m'], e['n']), []).append(e)
    return out, allof


def main():
    ov = json.load(io.open(os.path.join(SPEC, 'resolved_2026-09-10.json'), encoding='utf-8'))
    gmap, gmap_all = load_map()
    specs = []
    for fn in sorted(os.listdir(os.path.join(SPEC, 'r1'))):
        sid = fn[:-5]
        rnd = None
        for r in ROUNDS:
            p = os.path.join(SPEC, r, fn)
            if os.path.exists(p):
                j = json.load(io.open(p, encoding='utf-8'))
                rnd = r
                break
        rounds_done = [r for r in ROUNDS if os.path.exists(os.path.join(SPEC, r, fn))]

        rec = dict(
            id=sid, name=j['name'], sym=j.get('sym'), src=j.get('src'),
            src_line=j.get('src_line'), layer=layer_of(j.get('src')),
            ir=dict(file=j.get('ir_file'), frm=j.get('ir_from'), to=j.get('ir_to')),
            base_round=rnd, rounds=len(rounds_done),
            one_line=j.get('one_line'), signature=j.get('signature'),
            logic=j.get('logic'), reads=j.get('reads') or [], writes=j.get('writes') or [],
            constants=j.get('constants') or [], calls=j.get('calls') or [],
            knobs=j.get('knobs') or [], unknown=j.get('unknown') or [],
        )
        # 지도 조인
        key = MAPKEY.get(sid)
        e = gmap.get(key) if key else None
        if e:
            rec['exe'] = dict(addr=e['a'], module=e['m'], bytes=e['b'], instrs=e.get('i'),
                              evidence=e.get('v'), callers=e.get('o') or [], callees=e.get('c') or [])
            # ⚠★(모듈,이름)이 exe 에 **여러 곳**이면 그대로 붙이면 안 된다.
            #   실제 사고: `hunt_and_poke__is_end` 가 defa20 / df0a90 두 곳인데
            #   각각 Epic 판과 Serpen 판이다(6차 담당자가 "섞지 말 것"이라 경고한 쌍).
            #   자동 조인하면 Serpen 함수에 Epic 명세가 붙는 조용한 오답이 된다.
            dups = gmap_all.get(key) or []
            if len(dups) > 1:
                rec['exe']['ambiguous'] = [x['a'] for x in dups]
        else:
            rec['exe'] = None
            rec['no_map_reason'] = NOMAP.get(sid, '지도에서 못 찾음(사유 미확인)')
        # 오버레이 — 한 함수에 오버레이 키가 여러 개일 수 있으므로 **전부 합친다**
        # (처음엔 next() 로 하나만 집어서 뒤에 추가한 조사 결과가 조용히 사라질 뻔했다).
        rec['resolved'], rec['new_knobs'], rec['still_unknown'] = [], [], []
        for k, v in OVKEY.items():
            if v != sid:
                continue
            o = ov.get(k)
            if not o:
                continue
            rec['resolved'] += (o.get('resolved') or []) + (o.get('resolved_2') or [])
            rec['new_knobs'] += (o.get('new_knobs') or []) + (o.get('new_knobs_2') or [])
            rec['still_unknown'] += o.get('still_unknown') or []
        specs.append(rec)

    doc = dict(meta=dict(
        game='0.5.8', date='2026-09-10',
        what='game_ai 판단함수 20개 상세 명세. 1~6차 IR 독해 + 2026-09-10 미확정 해소.',
        source='C:\\tfm2mods\\MIG\\_spec\\r1..r6 + resolved_2026-09-10.json',
        rounds='함수당 최대 6번 독립 재작성. 최종 라운드 교차검증 = 판정상수 일치 100%.',
        counts=dict(functions=len(specs),
                    reads=sum(len(s['reads']) for s in specs),
                    writes=sum(len(s['writes']) for s in specs),
                    constants=sum(len(s['constants']) for s in specs),
                    calls=sum(len(s['calls']) for s in specs),
                    knobs=sum(len(s['knobs']) for s in specs),
                    new_knobs=sum(len(s['new_knobs']) for s in specs),
                    resolved=sum(len(s['resolved']) for s in specs),
                    unknown=sum(len(s['unknown']) for s in specs),
                    still_unknown=sum(len(s['still_unknown']) for s in specs),
                    joined_to_map=sum(1 for s in specs if s['exe'])),
    ), shared=ov.get('_shared', {}), specs=specs)

    io.open(OUT, 'w', encoding='utf-8', newline='').write(
        json.dumps(doc, ensure_ascii=False, indent=1))
    c = doc['meta']['counts']
    print('%s  (%.0f KB)' % (OUT, os.path.getsize(OUT) / 1024.0))
    for k, v in c.items():
        print('  %-16s %d' % (k, v))


if __name__ == '__main__':
    main()
