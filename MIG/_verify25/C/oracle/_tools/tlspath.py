"""tlspath.py — 루트 action_candidates 에서 지정 TLS 소유함수(with 인스턴스)까지 최단 경로(정방향 BFS, 경로 포함)."""
import pickle, re, collections, sys
G, t = pickle.load(open('cg.pkl', 'rb'))
root = [s for s in G if s.endswith('SerpenHuntSubPlan17action_candidates')][0]
def sh(s):
    m = re.findall(r'\d+([A-Za-z_][A-Za-z0-9_]*)', s)
    return '::'.join(m[-2:]) if m else s[-40:]
keys = sys.argv[1:] or ['check_kill_die_tick0', 'camp_pos0', 'position_eval_at0', 'interaction_ctx0', 'champion_hp_value0', 'last_stand_flags0', 'v47_siege_stance0', 'pe_player_ctx0', 'expected_attack_damage_cached0', 'entity_positioning_cache_cached0', 'tower_minion_in_range_count_cached0', 'slot_ready_cached0', 'slot_cc_time_cached0', 'resolve_fight_fulls_0', 'max_range_cached0', 'v48_cast_beams0', 'champ_powers', 'PathScratch', 'FieldScratch']
seen = {root: None}; q = collections.deque([root]); order = []
while q:
    s = q.popleft(); order.append(s)
    for c in G.get(s, (frozenset(),))[0]:
        if c not in seen and c in G:
            seen[c] = s; q.append(c)
for k in keys:
    tn = [s for s in order if k in s and (('8LocalKey' in s and '4with' in s) or 'LocalKey' in s)]
    if not tn:
        print('%-40s NOT REACHED' % k); continue
    tn = tn[0]
    path = []; x = tn
    while x is not None: path.append(x); x = seen[x]
    path.reverse()
    print('%-40s depth %d : %s' % (k, len(path) - 1, ' -> '.join(sh(p) for p in path[1:-1])))
