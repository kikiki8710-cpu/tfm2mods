# -*- coding: utf-8 -*-
"""마지막 3개 — 설명에 '원본≈N' 또는 '사이트별 N/N/N' 로만 적혀 있어 자동 추출이 안 되던 것."""
import io, sys
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
P = 'C:/tfm2mods/ai_adjust_editor/src/main.rs'
t = io.open(P, encoding='utf-8').read()
LAST = {'gb_close_radius': '약 387', 'gb_line_range': '약 500', 'gk_wait': '10 / 12 / 15'}
anc = ' // ★[08-06] 4차 보강'
assert anc in t
ins = ' // ★[08-06] 마지막 3개 — 설명에 근사치/사이트별로만 적혀 있던 것.\n' + \
      ''.join(' "%s" => "%s",\n' % (k, v) for k, v in LAST.items())
io.open(P, 'w', encoding='utf-8', newline='\n').write(t.replace(anc, ins + anc, 1))
print('마지막 %d개 보강' % len(LAST))
