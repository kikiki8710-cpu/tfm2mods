# -*- coding: utf-8 -*-
"""편집기 두 가지 수정.

①원본값과 같은데 파랑으로 표시되던 것
   `changed = baseline != cur` 만 봤다. cfg 의 `-1` 을 실제 기본값으로 펼치면서
   baseline("-1")과 달라져 전부 '변경됨'으로 잡혔다.
   ⟹ **실제 기본값과도 같으면 변경 아님**으로 본다.

②입력 중에 빈칸이 기본값으로 자동으로 채워져 타이핑을 방해하던 것
   ⟹ 텍스트박스가 **포커스를 잡고 있는 동안에는 손대지 않는다.**
     포커스가 빠졌을 때만 기본값을 채워 보여준다."""
import sys, io
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

P = 'C:/tfm2mods/ai_adjust_editor/src/main.rs'
t = io.open(P, encoding='utf-8').read()
n = 0


def sub(old, new, tag, cnt=1):
    global t, n
    if t.count(old) < cnt:
        print('  [건너뜀] %s (%d 필요 / %d 발견)' % (tag, cnt, t.count(old))); return
    t = t.replace(old, new, cnt); n += cnt
    print('  [ok] %s ×%d' % (tag, cnt))


# ── ① 파랑(변경됨) 판정 ─────────────────────────────────────
sub(' let changed = def.as_ref().map_or(false, |d| d != &cur);',
''' // ★[08-06] baseline 뿐 아니라 **실제 기본값**과도 비교한다.
 //   cfg 의 `-1` 을 실제 기본값으로 펼치면서 baseline("-1")과 달라져 멀쩡한 값이 전부 파랑으로 잡혔다.
 let changed = def.as_ref().map_or(false, |d| d != &cur)
     && orig_val(k).map_or(true, |o| o != cur);''', '변경 판정(전역)')

sub('            let changed = def.as_ref().map_or(false, |d| d != &cur);',
'''            let changed = def.as_ref().map_or(false, |d| d != &cur)
                && orig_val(k).map_or(true, |o| o != cur);''', '변경 판정(클래스)')

# ── ② 포커스 중에는 자동 채움 금지 ──────────────────────────
sub(''' let mut v = shown_val(k, &cur);
 let resp = ui.add_sized([width, 24.0], egui::TextEdit::singleline(&mut v).font(egui::TextStyle::Monospace));
 if resp.changed() { self.set_val(k, v.trim()); }''',
''' // ★[08-06] 입력 중에는 자동 채움 금지 — 포커스가 빠졌을 때만 기본값을 보여준다.
 let tid = ui.make_persistent_id(("valbox", k));
 let focused = ui.ctx().memory(|m| m.has_focus(tid));
 let mut v = if focused { cur.clone() } else { shown_val(k, &cur) };
 let resp = ui.add_sized([width, 24.0],
     egui::TextEdit::singleline(&mut v).id(tid).font(egui::TextStyle::Monospace));
 if resp.changed() { self.set_val(k, v.trim()); }''', '포커스 인지(전역)')

sub('''              let mut v = shown_val(k, &cur);
              let resp = ui.add_sized([COL2_W - 6.0, 24.0], egui::TextEdit::singleline(&mut v).font(egui::TextStyle::Monospace));
              if resp.changed() { self.set_val(k, v.trim()); }''',
'''              // ★[08-06] 입력 중에는 자동 채움 금지(위와 동일 사유).
              let tid = ui.make_persistent_id(("valbox_cls", k));
              let focused = ui.ctx().memory(|m| m.has_focus(tid));
              let mut v = if focused { cur.clone() } else { shown_val(k, &cur) };
              let resp = ui.add_sized([COL2_W - 6.0, 24.0],
                  egui::TextEdit::singleline(&mut v).id(tid).font(egui::TextStyle::Monospace));
              if resp.changed() { self.set_val(k, v.trim()); }''', '포커스 인지(클래스)')

io.open(P, 'w', encoding='utf-8', newline='\n').write(t)
print('\n적용 %d건' % n)
