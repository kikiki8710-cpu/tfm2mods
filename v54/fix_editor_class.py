# -*- coding: utf-8 -*-
"""편집기 수정 ③ — 클래스별 값이 **원리상 안 먹는 노브**에는 클래스 칸을 주지 않는다.

바이트패치 전용 노브(342개)는 exe 기계어 상수를 고치는 방식이라 선수별로 다를 수 없다.
그런데 편집기가 칸을 내주니 누구든 값을 넣게 되고, 그 값은 **아무 효과 없이**
skip_untuned 최적화만 통째로 꺼서 재생을 멈춘다(08-06 사고). 칸 자체를 없앤다."""
import sys, io
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

P = 'C:/tfm2mods/ai_adjust_editor/src/main.rs'
t = io.open(P, encoding='utf-8').read()
n = 0

# ── 1) 모듈 포함 ──────────────────────────────────────────
A = '// 클래스별 오버라이드: "키_class_<en>"'
if '#[path = "class_capable.rs"]' not in t:
    t = t.replace(A, '#[path = "class_capable.rs"] mod class_capable;\n'
                     'use class_capable::CLASS_CAPABLE;\n'
                     '/// 이 노브에 클래스별 값을 줄 수 있는가. 바이트패치 전용 노브는 원리상 불가능하다\n'
                     '/// (exe 기계어 상수를 고치는 방식이라 선수별로 다를 수 없다) — 칸을 아예 내주지 않는다.\n'
                     'fn class_capable(k: &str) -> bool { CLASS_CAPABLE.contains(&k) }\n' + A, 1)
    n += 1
    print('  [ok] 모듈 포함 + class_capable()')

# ── 2) 클래스 모드 렌더: 불가 노브는 안내만 ────────────────
OLD = '''            if self.active_class >= 0 {
              // ── 클래스 오버라이드 모드: '기본 따름' 토글 + 전용값 입력 ──
              let pos = self.active_class as usize;'''
NEW = '''            if self.active_class >= 0 && !class_capable(k) {
              // ★[08-07] 이 노브는 바이트패치 전용 = 클래스별 값이 원리상 적용될 수 없다.
              //   칸을 내주면 값이 들어가고, 그 값은 효과 없이 skip_untuned 최적화만 꺼서
              //   재생을 멈춘다(08-06 사고). 그래서 입력 자체를 막고 이유를 보여준다.
              ui.label(egui::RichText::new("클래스별 지정 불가 (전체 공통)")
                .color(egui::Color32::from_rgb(150, 150, 150)).italics())
                .on_hover_text("이 항목은 게임 코드의 상수를 직접 고치는 방식이라 선수마다 다른 값을 줄 수 없습니다. 전체 공통 값만 쓰입니다.");
            } else if self.active_class >= 0 {
              // ── 클래스 오버라이드 모드: '기본 따름' 토글 + 전용값 입력 ──
              let pos = self.active_class as usize;'''
assert OLD in t, '클래스 렌더부 원문 불일치'
t = t.replace(OLD, NEW, 1); n += 1
print('  [ok] 불가 노브 = 입력 차단 + 사유 표시')

io.open(P, 'w', encoding='utf-8', newline='\n').write(t)
print('\n적용 %d건' % n)
