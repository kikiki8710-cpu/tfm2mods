# -*- coding: utf-8 -*-
"""`설정값` 아래 기본값 줄이 `—` 로 나오는 문제.
사유: baseline(default.txt)에 없는 키는 def=None → "—" 로 떨어지는데,
      match 가 ("-1", Some(o)) 만 처리해서 기본값 맵을 쓰지 않았다.
      08-06에 재노출한 키들(교전·합류·포탑·능력치 등)이 baseline 에 없어 전부 여기 걸렸다.
수정: def 가 없거나 `-1`/빈칸이면 기본값 맵(orig_val)으로 폴백한다."""
import io, sys
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
P = 'C:/tfm2mods/ai_adjust_editor/src/main.rs'
t = io.open(P, encoding='utf-8').read()
OLD = ''' let d = def.clone().unwrap_or_else(|| "—".into());
 match (d.as_str(), orig_val(k)) {
 // ★[08-06] `기본 -1` 은 사용자에게 아무 정보가 없다 — 실제 기본값을 보여준다.
 ("-1", Some(o)) => format!("기본 {}", o),
 _ => format!("기본 {}", d),
 }'''
NEW = ''' let d = def.clone().unwrap_or_else(|| "—".into());
 // ★[08-06] `기본 -1`·`기본 —` 은 사용자에게 아무 정보가 없다 — 기본값 맵으로 폴백한다.
 //   baseline(default.txt)에 없는 키는 def=None → "—" 가 되는데, 재노출한 키들이 전부 여기 걸렸다.
 match (d.as_str(), orig_val(k)) {
 ("-1", Some(o)) | ("—", Some(o)) | ("", Some(o)) => format!("기본 {}", o),
 _ => format!("기본 {}", d),
 }'''
assert OLD in t, '앵커 불일치'
io.open(P,'w',encoding='utf-8',newline='\n').write(t.replace(OLD, NEW, 1))
print('base_line 폴백 추가')
