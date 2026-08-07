# -*- coding: utf-8 -*-
"""원본 테스트C 에서 **기본값과 달랐던 키**(=실제로 손댄 값)만 추려 지금 어떻게 됐는지 본다.
개발용 계측·프로브 플래그는 제외(전부 0=꺼짐이라 손댄 값이 아니다)."""
import sys, io
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

MOD = ('C:/Program Files (x86)/Steam/steamapps/common/Teamfight Manager2'
       '/mods/tfm2_ai_adjust')
SRC = r'C:\Users\dev\Downloads\tfm2_ai_adjust\tfm2_ai_adjust.cfg'
CUR = MOD + '/config/테스트C.cfg'
DEF = MOD + '/config/default.txt'

PROBE = {'replace', 'disppred', 'move', 'move_tag', 'move_off', 'move_x', 'move_y',
         'dd7cap', 'rngcap', 'rng_repl', 'pgcap', 'pg_a', 'pg_b', 'pg_c', 'tecap',
         'seed_rotate', 'dmgcap', 'candcap', 'gbcap', 'gbcallee', 'strat_rotate',
         'e9a30cap', 'class_probe', 'class_sheet', 'probe'}
ALIAS = {'oi_enable': 'nx_enable', 'oi_dn_count_gate': 'nx_dn_count_gate',
         'oi_dn_nexus_hp': 'nx_dn_nexus_hp', 'oi_dn_hp_crit': 'nx_dn_hp_crit',
         'oi_dn_hp_low': 'nx_dn_hp_low', 'oi_dn_near_dist': 'nx_dn_near_dist',
         'oi_dn_pred_dist': 'nx_dn_pred_dist', 'oi_dn_lane_margin': 'nx_dn_vision_mem',
         'oi_an_count_gate': 'nx_an_count_gate', 'oi_an_finish_hp': 'nx_an_finish_hp',
         'oi_an_cull_dist': 'nx_an_cull_dist'}


def kv(p):
    d = {}
    for ln in io.open(p, encoding='utf-8'):
        s = ln.strip()
        if s and not s.startswith('#') and '=' in s:
            k, v = [x.strip() for x in s.split('=', 1)]
            d[k] = v
    return d


o, c, dd = kv(SRC), kv(CUR), kv(DEF)

custom = []
for k, v in sorted(o.items()):
    if k in PROBE:
        continue
    base = dd.get(k)
    if base is None:          # baseline 에 없는 키 = 기본값 판정 불가
        custom.append((k, v, '(기준없음)')); continue
    if v != base:
        custom.append((k, v, base))

print('원본 테스트C 활성 키 %d 중, **기본값과 달랐던 키 = %d개**\n' % (len(o), len(custom)))
print('%-22s %-10s %-10s %s' % ('키', '원본값', '기본값', '지금'))
print('-' * 74)
for k, v, base in custom:
    if k in ALIAS:
        nk = ALIAS[k]
        now = '%s = %s' % (nk, c.get(nk, '???'))
    elif k in c:
        now = c[k] + ('' if c[k] == v else '   ← 바뀜')
    else:
        now = '(빠짐)'
    print('%-22s %-10s %-10s %s' % (k, v, base, now))
