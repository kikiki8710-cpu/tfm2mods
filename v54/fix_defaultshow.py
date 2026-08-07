# -*- coding: utf-8 -*-
"""편집기 표시 개선 — `-1`(원본 유지) 대신 **실제 기본값**을 보여준다.
   ①`기본 -1 · 원본 60000` → `기본 60000`
   ②값이 `-1`/빈칸이면 텍스트박스에 실제 기본값을 넣어 보여준다
     ⚠원본값이 숫자가 아닌 것(`주사위 굴림`·`약 387`·`10 / 12 / 15`)은 cfg 에 넣을 수 없는 문자열이라
       그대로 `-1` 을 보여준다. 사용자가 손대지 않으면 저장도 안 된다(변경 이벤트가 없다)."""
import sys, io
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

P = 'C:/tfm2mods/ai_adjust_editor/src/main.rs'
t = io.open(P, encoding='utf-8').read()
n = 0


def sub(old, new, tag, cnt=1):
    global t, n
    if t.count(old) < cnt:
        print('  [건너뜀] %s (%d개 필요, %d개 발견)' % (tag, cnt, t.count(old))); return
    t = t.replace(old, new, cnt); n += cnt
    print('  [ok] %s ×%d' % (tag, cnt))


# ── ① 기본 라벨 ─────────────────────────────────────────────
sub(''' ("-1", Some(o)) => format!("기본 -1 · 원본 {}", o),''',
''' // ★[08-06] `기본 -1` 은 사용자에게 아무 정보가 없다 — 실제 기본값을 보여준다.
 ("-1", Some(o)) => format!("기본 {}", o),''', '기본 라벨')

# ── ② 표시용 헬퍼 추가 ──────────────────────────────────────
sub('''fn base_line(k: &str, def: &Option<String>) -> String {''',
'''/// ★[08-06] 텍스트박스에 보여줄 값 — `-1`(원본 유지)이나 빈칸이면 **실제 기본값**을 대신 보여준다.
///   ⚠기본값이 숫자가 아니면(`주사위 굴림`·`약 387`·`10 / 12 / 15`) cfg 에 넣을 수 없으므로 원래 값을 그대로 둔다.
fn shown_val(k: &str, cur: &str) -> String {
    if !(cur.is_empty() || cur == "-1") { return cur.to_string(); }
    match orig_val(k) {
        Some(o) if !o.is_empty() && o.trim_start_matches('-').chars().all(|c| c.is_ascii_digit()) => o.to_string(),
        _ => cur.to_string(),
    }
}

fn base_line(k: &str, def: &Option<String>) -> String {''', '표시 헬퍼')

# ── ③ 텍스트박스 4곳 ────────────────────────────────────────
sub(' let mut v = cur.clone();', ' let mut v = shown_val(k, &cur);', '텍스트박스(전역)', 2)
sub('              let mut v = cur.clone();', '              let mut v = shown_val(k, &cur);', '텍스트박스(클래스)', 2)

io.open(P, 'w', encoding='utf-8', newline='\n').write(t)
print('\n적용 %d건' % n)
