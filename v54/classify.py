# -*- coding: utf-8 -*-
"""은닉 노브 116개 분류 — 재노출(사용자용) vs 은닉유지(디버그/실험/죽은 것)."""
import sys, io, re, json
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

st = json.load(io.open('C:/tfm2mods/v54/audit_state.json', encoding='utf-8'))
wired, in_tab, desc = set(st['wired']), set(st['in_tab']), st['desc']
hidden = sorted(wired - in_tab)

# 디버그/실험 성격을 드러내는 신호
DBG_KEY = re.compile(r'(_repl$|^d19(gate|vis|abil|thr)|shadow|_cmp$|_loop2$|verify|watch|^probe|^sim_|^key$'
                     r'|unchunk|^condcap$|^gbskip$|^d4ttd$|bench|^fast_|^hang_|^perf_)')
DBG_TXT = re.compile(r'디버그|실험용|계측|검증용|ablation|대체 토글|재현 미검증|진단|로그')

# ★진짜 신호는 설명 앞머리의 ⛔(작동안함/폐기) 와 ⚠[은퇴] 다 — 은닉된 이유가 대부분 '이미 죽어서'였다.
DEAD = re.compile(r'⛔|\[은퇴\]|작동하지 않습니다|폐기된 값')
DEV = re.compile(r'개발용|검증용')

buckets = {'A_재노출': [], 'B_은닉유지_죽은노브': [], 'C_은닉유지_디버그': [], 'D_설명없음_보류': []}
for k in hidden:
    d = desc.get(k)
    if not d:
        buckets['D_설명없음_보류'].append((k, ''))
    elif DEAD.search(d):
        buckets['B_은닉유지_죽은노브'].append((k, d))
    elif DEV.search(d) or DBG_KEY.search(k) or DBG_TXT.search(d):
        buckets['C_은닉유지_디버그'].append((k, d))
    else:
        buckets['A_재노출'].append((k, d))

for name in ('A_재노출', 'B_은닉유지_죽은노브', 'C_은닉유지_디버그', 'D_설명없음_보류'):
    v = buckets[name]
    print('\n===== %s : %d건 =====' % (name, len(v)))
    for k, d in v:
        print('  %-24s %s' % (k, (d[:62] + '…') if len(d) > 62 else d))

json.dump({k: [x for x, _ in v] for k, v in buckets.items()},
          io.open('C:/tfm2mods/v54/hidden_class.json', 'w', encoding='utf-8'), ensure_ascii=False)
