# 순서도의 설정값 아래에 **설명문을 바로 표시**한다(지금은 '설명' 버튼을 눌러야 보임).
#  설명문엔 <b> 같은 태그가 섞여 있어 평문으로 바꿔 그린다.
import io, sys
sys.stdout.reconfigure(encoding="utf-8")
P = "src/main.rs"
s = io.open(P, encoding="utf-8").read()

# ── ① HTML 태그 제거 헬퍼 ──
HELPER = '''
/// 설명문은 HTML 조각(`<b>`·`<br>`·`\\` 줄이음)을 섞어 쓴다. 순서도에 그대로 그리면 태그가 보이므로
/// 평문으로 바꾼다. 굵게 표시는 못 살리지만 순서도에선 한 줄 요약이면 충분하다.
fn desc_plain(k: &str) -> Option<String> {
  let raw = desc_static(k)?;
  let mut out = String::with_capacity(raw.len());
  let mut in_tag = false;
  for c in raw.chars() {
    match c {
      '<' => in_tag = true,
      '>' => { in_tag = false; }
      _ if in_tag => {}
      '\\n' => out.push(' '),
      _ => out.push(c),
    }
  }
  // 태그 자리에서 생긴 이중 공백 정리
  let mut t = String::with_capacity(out.len());
  let mut sp = false;
  for c in out.chars() {
    if c == ' ' { if !sp { t.push(c); } sp = true; } else { sp = false; t.push(c); }
  }
  let t = t.trim().to_string();
  if t.is_empty() { None } else { Some(t) }
}

fn base_line(k: &str, def: &Option<String>) -> String {'''
A = "fn base_line(k: &str, def: &Option<String>) -> String {"
assert A in s
s = s.replace(A, HELPER, 1)

# ── ② 순서도 그리드에 설명 줄 추가 ──
B_OLD = """                              ui.label(egui::RichText::new(
                                base_line(k, &def))
                                .small().weak());
                            });
                            self.value_ctl(ui, k, 220.0);"""
B_NEW = """                              ui.label(egui::RichText::new(
                                base_line(k, &def))
                                .small().weak());
                            });
                            self.value_ctl(ui, k, 220.0);
                            // ★설명을 바로 옆에 그린다 — 버튼을 눌러야 보이면 훑어보기가 안 된다.
                            if let Some(d) = desc_plain(k) {
                              ui.vertical(|ui| {
                                ui.set_max_width(430.0);
                                Self::para(ui, &d, Self::F_DIM, Some(12.5));
                              });
                            } else { ui.label(""); }"""
assert B_OLD in s
s = s.replace(B_OLD, B_NEW, 1)

# 열 수 3 → 4
C_OLD = """                        .num_columns(3).striped(true).spacing([12.0, 6.0]).show(ui, |ui| {"""
C_NEW = """                        .num_columns(4).striped(true).spacing([12.0, 8.0]).show(ui, |ui| {"""
assert C_OLD in s
s = s.replace(C_OLD, C_NEW, 1)

# '설명' 버튼은 남기되 라벨을 '자세히'로 (본문은 이미 보이므로)
s = s.replace('if ui.small_button("설명").clicked() {', 'if ui.small_button("자세히").clicked() {', 1)

io.open(P, "w", encoding="utf-8").write(s)
print("순서도 설명문 인라인 표시 배선 완료")
