# 편집기 UI 정리
#  ① 좌측 탭 목록 스크롤 가능하게 (탭이 19개라 창이 작으면 잘린다)
#  ② 순서도의 '자세히' 버튼 제거 (설명이 이미 옆에 보인다)
#  ③ '설명창 열기' 토글 + 오른쪽 설명 사이드바 제거
import io, sys
sys.stdout.reconfigure(encoding="utf-8")
P = "src/main.rs"
s = io.open(P, encoding="utf-8").read()
n = 0

# ── ① 좌측 탭 스크롤 ──
A = '''      ui.label(egui::RichText::new("탭").weak());
      ui.add_space(2.0);
      for (i, t) in TABS.iter().enumerate() {
        if ui.selectable_label(self.active_tab == i, html_to_text(t.title)).clicked() {
          self.active_tab = i;
        }
      }'''
B = '''      ui.label(egui::RichText::new("탭").weak());
      ui.add_space(2.0);
      // ★탭이 많아 창이 작으면 아래쪽이 잘린다 → 목록 자체를 스크롤 영역으로.
      egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
        for (i, t) in TABS.iter().enumerate() {
          if ui.selectable_label(self.active_tab == i, html_to_text(t.title)).clicked() {
            self.active_tab = i;
          }
        }
      });'''
if A in s: s = s.replace(A, B, 1); n += 1
else: print("① 못 찾음")

# ── ② '자세히' 버튼 제거 (그리드 4열 → 3열) ──
A2 = '''                            if ui.small_button("자세히").clicked() {
                              self.flow_sel = Some(k.to_string());
                              self.flow_desc = true;
                            }
                            ui.end_row();'''
B2 = '''                            ui.end_row();'''
if A2 in s: s = s.replace(A2, B2, 1); n += 1
else: print("② 못 찾음")

A2b = '''                        .num_columns(4).striped(true).spacing([12.0, 8.0]).show(ui, |ui| {'''
B2b = '''                        .num_columns(3).striped(true).spacing([12.0, 8.0]).show(ui, |ui| {'''
if A2b in s: s = s.replace(A2b, B2b, 1); n += 1
else: print("②b 못 찾음")

# 키 이름 클릭 시 설명창 여는 동작도 제거 (창이 없어지므로) — 선택 표시는 남긴다
A2c = '''                                if ui.selectable_label(self.flow_sel.as_deref() == Some(k), t)
                                  .on_hover_text("클릭하면 '설명' 탭에서 자세히 봅니다").clicked() {
                                  self.flow_sel = Some(k.to_string());
                                  self.flow_desc = true;
                                }'''
B2c = '''                                if ui.selectable_label(self.flow_sel.as_deref() == Some(k), t).clicked() {
                                  self.flow_sel = Some(k.to_string());
                                }'''
if A2c in s: s = s.replace(A2c, B2c, 1); n += 1
else: print("②c 못 찾음")

# ── ③ '설명창 열기' 토글 + 사이드바 제거 ──
A3 = '''      let lbl = if self.flow_desc { "설명창 닫기" } else { "설명창 열기" };
      if ui.selectable_label(self.flow_desc, lbl).clicked() { self.flow_desc = !self.flow_desc; }'''
if A3 in s: s = s.replace(A3, "", 1); n += 1
else: print("③ 토글 못 찾음")

A3b = '''      // 설명은 오른쪽 사이드바 — 순서도를 보면서 동시에 읽는다(페이지 이동 없음)
      if self.flow_desc {
        egui::SidePanel::right("flowdesc")
          .resizable(true).default_width(400.0).min_width(300.0)
          .show_inside(root, |ui| { self.flow_desc_ui(ui); });
      }
'''
if A3b in s: s = s.replace(A3b, "", 1); n += 1
else: print("③ 사이드바 못 찾음")

io.open(P, "w", encoding="utf-8").write(s)
print("%d/6 적용" % n)
