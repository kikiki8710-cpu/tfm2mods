# -*- coding: utf-8 -*-
"""파서 수정(룩어헤드)으로 새로 드러난 살아있는 노브 8개를 regrouped 탭에 추가.
   이미 노출한 `*_move`(라인전 전용 값)의 **기준값**들이라 짝이 맞지 않던 상태였다."""
import sys, io, re
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

P = 'C:/tfm2mods/ai_adjust_editor/src/main.rs'
t = io.open(P, encoding='utf-8').read()

ADD = {
    '§◆ 포탑을 전력·위협으로 어떻게 셀지': ['ally_tower_dps', 'ally_tower_hp', 'ally_tower_range'],
    '§◆ 머릿수를 보고 물러날지': ['numbers_range', 'numbers_threat', 'numbers_min_enemy_move'],
    '§◆ 선수 능력치를 판단에 어떻게 반영할지': ['stat_influence'],
    '§◆ 교전에 들어갈지 정하는 확률': ['engage_thr_mult'],
}
m = re.search(r'(Tab\{\s*id:"regrouped".*?keys:&\[)(.*?)(\], note:)', t, re.S)
body = m.group(2)
added = []
for sec, keys in ADD.items():
    anchor = '"%s",' % sec
    if anchor not in body:
        print('  앵커 없음: %s' % sec); continue
    live = [k for k in keys if '"%s"' % k not in body]
    if not live:
        continue
    body = body.replace(anchor, anchor + ''.join('"%s",' % k for k in live), 1)
    added += live
t = t[:m.start(2)] + body + t[m.end(2):]
io.open(P, 'w', encoding='utf-8', newline='\n').write(t)
print('regrouped 탭에 %d개 추가: %s' % (len(added), ' '.join(added)))
