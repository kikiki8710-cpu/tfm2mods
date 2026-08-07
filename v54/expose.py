# -*- coding: utf-8 -*-
"""은닉 노브 116개 중 살아있는 28개만 편집기 탭에 재노출.
   B(죽은 노브 36) · C(디버그 3) · D(설명없음 49)는 그대로 은닉 유지 — 은닉된 데는 이유가 있었다."""
import sys, io, json
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

P = 'C:/tfm2mods/ai_adjust_editor/src/main.rs'
t = io.open(P, encoding='utf-8').read()
A = set(json.load(io.open('C:/tfm2mods/v54/hidden_class.json', encoding='utf-8'))['A_재노출'])

GROUPS = [
    ('§◆ 교전에 들어갈지 정하는 확률', ['eng_role2', 'eng_role3', 'eng_role4', 'eng_role_def', 't_engage', 'engage_base']),
    ('§◆ 아군 전투에 합류할지', ['rc_join_weight', 'rc_join_adv', 'rc_join_rescue', 'rc_join_obj_mult', 'rc_join_dnear', 'rc_join_dmid']),
    ('§◆ 포탑을 전력·위협으로 어떻게 셀지', ['tower_dps', 'tower_range', 'tower_threat',
                                            'ally_tower_dps_move', 'ally_tower_hp_move', 'ally_tower_range_move']),
    ('§◆ 머릿수를 보고 물러날지', ['numbers_margin', 'numbers_min_enemy', 'numbers_range_move', 'numbers_threat_move']),
    ('§◆ 선수 능력치를 판단에 어떻게 반영할지', ['stat_judg_ref', 'stat_neutral', 'stat_noise_shift', 'stat_pos_div']),
    ('§◆ 그 밖', ['aggr_lane', 'dd_n_thr']),
]
keys, missing = [], []
for label, ks in GROUPS:
    live = [k for k in ks if k in A]
    for k in ks:
        if k not in A:
            missing.append(k)
    if live:
        keys.append('"%s"' % label)
        keys += ['"%s"' % k for k in live]
n = sum(1 for x in keys if not x.startswith('"§'))
assert n == len(A), '분류 A %d개 중 %d개만 배치됨 — 누락 %s' % (len(A), n, sorted(A - {x.strip('"') for x in keys}))

TAB = ''' Tab{ id:"regrouped", title:"• 교전 · 합류 · 포탑 · 능력치 (다시 꺼낸 노브)", keys:&[
 ''' + ','.join(keys) + ''',], note:
 "<b>배선은 돼 있는데 편집기에서 사라져 있던 노브</b>들입니다. 살아 있는 것만 골라 다시 꺼냈습니다.<br>\\
 함께 숨어 있던 나머지는 <b>일부러 그대로 두었습니다</b> — 대부분 게임 쪽 코드가 사라져 값을 바꿔도 아무 일도 일어나지 않는 것들이고(설명에 ⛔로 표시돼 있습니다), 나머지는 개발·검증용이거나 무슨 값인지 아직 확인되지 않은 것들입니다.<br>\\
 <b>교전 확률</b>과 <b>합류</b>는 성향을 크게 바꾸는 레버입니다. <b>포탑</b>·<b>머릿수</b>는 언제 물러날지를 정합니다.<br>\\
 ⚠<b>전부 -1(원본)이 기본</b>이라 그냥 두면 게임과 같습니다. 한 번에 하나씩 실험하세요.", },
'''
i = t.index(' Tab{ id:"pathsys"')
t = t[:i] + TAB + t[i:]
io.open(P, 'w', encoding='utf-8', newline='\n').write(t)
print('재노출 %d개 / 새 탭 regrouped 생성' % n)
if missing:
    print('(분류 A 밖이라 제외: %s)' % ' '.join(missing))
