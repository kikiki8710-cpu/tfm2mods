# -*- coding: utf-8 -*-
"""① 0.5.4 전용 탭 3개를 없애고 기존 탭에 편입한다.
   근거: 진짜 AttackNexus = disc18, DefenseNexus = disc19 (`disc19_repro.rs:3311`, `rva_051.rs:50`)
        ⟹ 이미 있는 "[실행 18·19] 넥서스 공수" 탭이 넥서스 공격의 제자리다. 새 탭 불필요.
   ② 원본값 맵(orig_val)에 없어 편집기에 '—'로 보이던 키를 설명문의 `원본 N` 에서 뽑아 채운다."""
import sys, io, re, json
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

P = 'C:/tfm2mods/ai_adjust_editor/src/main.rs'
t = io.open(P, encoding='utf-8').read()

# ── ① 탭 재편 ───────────────────────────────────────────────
def drop_tab(tid):
    """Tab{ id:"tid" ... }, 통째로 제거하고 그 keys 목록을 돌려준다."""
    global t
    m = re.search(r'\n?\s*Tab\{\s*id:"%s".*?keys:&\[(.*?)\], note:.*?\},\n' % tid, t, re.S)
    if not m:
        print('  [건너뜀] 탭 %s 없음' % tid); return []
    keys = [x for x in re.findall(r'"([^"]*)"', m.group(1))]
    t = t[:m.start()] + '\n' + t[m.end():]
    print('  탭 제거: %-16s (항목 %d)' % (tid, len(keys)))
    return keys


def add_to_tab(tid, items):
    """기존 탭 keys 끝(`,],`) 앞에 항목을 끼워 넣는다."""
    global t
    m = re.search(r'(Tab\{\s*id:"%s".*?keys:&\[)(.*?)(\], note:)' % tid, t, re.S)
    if not m:
        print('  [건너뜀] 대상 탭 %s 없음' % tid); return
    body = m.group(2).rstrip()
    if not body.endswith(','):
        body += ','
    body += ''.join('"%s",' % x for x in items)
    t = t[:m.start(2)] + body + t[m.end(2):]
    print('  탭 %-16s 에 %d개 편입' % (tid, len([x for x in items if not x.startswith('§')])))


drop_tab('nexus_auction')
drop_tab('pathsys')
drop_tab('nexus_def_misc')

# 넥서스 공격(an_*) + 방어 위험도(d19_sev_*) → 기존 "[실행 18·19] 넥서스 공수"
add_to_tab('disc19', [
    '§◆ 넥서스로 밀어붙일지 (0.5.4 신설)',
    'an_tower_gate', 'an_attack_sub', 'an_home_wait', 'an_fallback', 'an_fallback_wave', 'an_fallback_style',
    '§◆ 넥서스가 위험할 때 — 위험도 사다리',
    'd19_ally_hp', 'nx_dn_count_gate', 'nx_an_count_gate',
    'd19_sev_hp_1', 'd19_sev_hp_2', 'd19_sev_hp_3',
    'd19_sev_ratio_0', 'd19_sev_ratio_1', 'd19_sev_ratio_2', 'd19_sev_ratio_3',
])
# 경매 중 강제 귀환 → 경매층 탭
add_to_tab('judge', [
    '§◆ 경매 중 강제 귀환 (0.5.4 신설)',
    'auc_flee_version_gate', 'auc_flee_score', 'auc_flee_hp_field', 'auc_flee_nexus_mask',
    'auc_flee_goal_far', 'auc_flee_goal_near_a', 'auc_flee_goal_near_b',
    'auc_flee_end_delay', 'auc_flee_with_skill', 'auc_flee_action_tag',
    'auc_flee_pathfinder', 'auc_flee_undying_gate',
])
# 경로·거리 → 이동 만들기 탭
add_to_tab('movein', [
    '§◆ 어디로 걸어갈지 — 경로·거리 (0.5.4 신설)',
    'path_orth_cost', 'path_diag_cost', 'path_greedy',
    'path_threat_floor', 'path_threat_cap', 'path_threat_scale', 'path_threat_default',
    'path_danger_cost', 'path_wave_risk_ret',
])
# 나머지 3개는 성격에 맞는 기존 탭으로
add_to_tab('def', ['disc16_home_hp'])          # disc16 = SerpenHunt(사냥) 부상 홈대기
add_to_tab('regrouped', ['jungle_retreat_threat'])
add_to_tab('engine', ['self_team_only'])

# regrouped 탭 이름을 자연스럽게
t = t.replace('title:"• 교전 · 합류 · 포탑 · 능력치 (다시 꺼낸 노브)"',
              'title:"• [공통] 교전 진입 · 합류 · 포탑 · 능력치"', 1)

io.open(P, 'w', encoding='utf-8', newline='\n').write(t)
print('\n탭 재편 완료')
