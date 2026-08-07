# -*- coding: utf-8 -*-
"""체크박스(토글) 기본값 표시 정정 — 전수 대조 결과 반영.

실제 기본값 출처(추측 금지):
  ·cfg arm 방식 → arm 본문이 쓰는 static 의 초기값 (`e9jt` → `E9_JT` 처럼 이름이 다르다)
  ·tune 방식    → `tune("키", 기본값)` 의 두 번째 인자
정정 대상:
  gbskip / d4ttd / d15_repl  = 꺼짐   (편집기에 기본값이 아예 없었다)
  d4_repl                    = **켜짐** (static 초기값 true — 유일하게 기본이 켜진 대체 토글)
  fix_skill2_dmg             = 꺼짐   (구 표기 `-1` 은 토글 표시로 부적절)
  fix_hp_ratio / probe / hd_skip_landmark / lt_revive_join = 꺼짐 (tune 기본 0)
"""
import sys, io
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

P = 'C:/tfm2mods/ai_adjust_editor/src/main.rs'
t = io.open(P, encoding='utf-8').read()

# 기존 fix_skill2_dmg 표기(-1) 제거 후 재정의
t = t.replace(' "fix_skill2_dmg" => "-1",\n', '', 1)
t = t.replace(' "fix_hp_ratio" => "0",\n', '', 1)
t = t.replace(' "probe" => "0",\n', '', 1)

NEW = {
    'gbskip': '꺼짐', 'd4ttd': '꺼짐', 'd15_repl': '꺼짐',
    'd4_repl': '켜짐',
    'fix_skill2_dmg': '꺼짐', 'fix_hp_ratio': '꺼짐', 'probe': '꺼짐',
    'hd_skip_landmark': '꺼짐', 'lt_revive_join': '꺼짐',
}
anc = ' // ★[08-06] 마지막 3개'
assert anc in t, '앵커 없음'
ins = (' // ★[08-06] 체크박스 기본값 전수 대조 정정(v54\\chk_toggles.py).\n'
       '//   실제 기본값은 arm 이 쓰는 static 초기값 / tune 두 번째 인자에서만 읽는다 — 키 이름으로 추측하지 않는다.\n'
       + ''.join(' "%s" => "%s",\n' % (k, v) for k, v in NEW.items()))
t = t.replace(anc, ins + anc, 1)
io.open(P, 'w', encoding='utf-8', newline='\n').write(t)
print('토글 기본값 %d개 정정' % len(NEW))
