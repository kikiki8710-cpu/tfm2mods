# -*- coding: utf-8 -*-
"""텍스트박스가 비거나 `-1` 로 남던 4개에 실제 기본값을 넣는다.

근거(소스 주석):
  cf_filter_off    `1 = 2차 검열 전면 무효화` → **끄지 않은 상태가 기본이므로 0**
  cf_flee_kill_off `1 = 몰살 규칙을 끈다(원본 = 몰살함)` → 같은 규약이라 **0**
  gb_close_radius  `근접반경(유닛, 원본≈387 / raw 150000)` → 유닛 입력이라 **387**
  gb_line_range    `라인range(유닛, 원본≈500 / raw 250000)` → **500**

⚠gb_* 는 raw 가 반경²이라 정확히는 √150000≈387.298 / √250000=500.
   387 은 149769 가 되어 원본 150000 과 0.15% 다르다 — **정확한 원본은 `-1` 뿐**이다.
   그래서 cfg 에는 계속 `-1` 을 두고, 편집기 박스에만 기본값을 보여준다(손대지 않으면 저장되지 않음)."""
import sys, io
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

P = 'C:/tfm2mods/ai_adjust_editor/src/main.rs'
t = io.open(P, encoding='utf-8').read()
n = 0
for old, new in [('"cf_filter_off" => "검열 함"',    '"cf_filter_off" => "0"'),
                 ('"cf_flee_kill_off" => "몰살 함"', '"cf_flee_kill_off" => "0"'),
                 ('"gb_close_radius" => "약 387"',   '"gb_close_radius" => "387"'),
                 ('"gb_line_range" => "약 500"',     '"gb_line_range" => "500"')]:
    c = t.count(old)
    if c:
        t = t.replace(old, new); n += c
        print('  [ok] %s  ×%d' % (new, c))
    else:
        print('  [건너뜀] %s' % old)
io.open(P, 'w', encoding='utf-8', newline='\n').write(t)
print('\n정정 %d건' % n)
