# -*- coding: utf-8 -*-
"""토글 라벨을 체크박스 **호출 후** 상태에서 만든다.
증상: 클릭한 프레임에 라벨이 클릭 전 값으로 그려져 '체크됨 + 꺼짐'이 함께 보였다.
원인: `let lbl = if on {..}` 를 `ui.checkbox(&mut on, lbl)` 보다 먼저 계산 — checkbox 가 on 을 제자리에서 뒤집는다."""
import io, sys
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
P = 'C:/tfm2mods/ai_adjust_editor/src/main.rs'
t = io.open(P, encoding='utf-8').read()

OLD = '''let mut on = cur == "1" || cur == "true";
 let lbl = if on { "켜짐" } else { "꺼짐" };
 if ui.checkbox(&mut on, lbl).changed() {
 self.set_val(k, if on { "1" } else { "0" });
 }'''
NEW = '''let mut on = cur == "1" || cur == "true";
 // ★라벨은 checkbox **뒤에서** 만든다 — checkbox 가 on 을 제자리에서 뒤집으므로,
 //   앞에서 계산하면 클릭한 프레임에 "체크됨 + 꺼짐"이 같이 보인다(2026-08-06 유저 신고).
 if ui.checkbox(&mut on, "").changed() {
 self.set_val(k, if on { "1" } else { "0" });
 }
 ui.label(if on { "켜짐" } else { "꺼짐" });'''
n = t.count(OLD)
t = t.replace(OLD, NEW)

OLD2 = '''let mut on = cur == "1" || cur == "true";
              let lbl = if on { "켜짐" } else { "꺼짐" };
              if ui.checkbox(&mut on, lbl).changed() {
                self.set_val(k, if on { "1" } else { "0" });
              }'''
NEW2 = '''let mut on = cur == "1" || cur == "true";
              // ★라벨은 checkbox 뒤에서 만든다(위 동일 사유).
              if ui.checkbox(&mut on, "").changed() {
                self.set_val(k, if on { "1" } else { "0" });
              }
              ui.label(if on { "켜짐" } else { "꺼짐" });'''
n2 = t.count(OLD2)
t = t.replace(OLD2, NEW2)
io.open(P, 'w', encoding='utf-8', newline='\n').write(t)
print('토글 라벨 순서 수정: %d + %d 곳' % (n, n2))
