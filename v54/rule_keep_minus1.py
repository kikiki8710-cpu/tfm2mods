# -*- coding: utf-8 -*-
"""★유저 규칙(2026-08-06): **내부값은 -1로 두고, 보여주기만 원본값으로.**

이유 — `-1` 은 "그 바이트를 건드리지 않는다", 실제 숫자는 "원본과 같다고 믿는 값으로 덮어쓴다" 이다.
둘은 결과가 같아 보이지만, 내가 채운 기본값이 하나라도 틀리면 그대로 오패치가 되고
그 오패치는 `applied=N/N` · `blocked=0` 으로 **전부 정상으로 보인다**(08-06 실사고 2건:
pe_noise_exempt · -1 펼치기 9건). 그러니 만지지 않은 노브는 끝까지 `-1` 로 남겨야 한다.

구현: set_val 에서 **입력값이 그 노브의 원본값과 같으면 `-1` 로 저장**한다.
  · 토글(is_toggle)은 제외 — 거기서 -1 은 0/1 규약에 없다.
  · 원본값이 숫자가 아닌 키도 제외.
화면 표시는 기존 shown_val() 이 그대로 원본값을 보여주므로 사용자 경험은 변하지 않는다."""
import sys, io
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

P = 'C:/tfm2mods/ai_adjust_editor/src/main.rs'
t = io.open(P, encoding='utf-8').read()

OLD = ''' fn set_val(&mut self, k: &str, v: &str) {
 if let Some(&i) = self.model.map.get(k) {'''
NEW = ''' fn set_val(&mut self, k: &str, v: &str) {
 // ★[08-06 유저 규칙] **내부값은 -1, 표시만 원본값.**
 //   -1 = "그 바이트를 안 건드림" / 숫자 = "원본과 같다고 믿는 값으로 덮어씀".
 //   내가 채운 기본값이 틀리면 그대로 오패치이고, 그 오패치는 applied=N/N·blocked=0 이라
 //   지표상 전부 정상으로 보인다(08-06 pe_noise_exempt·-1 펼치기 실사고). 그래서
 //   **원본값과 같은 입력은 -1 로 저장**해 "안 건드림" 상태를 유지한다.
 //   토글은 제외 — 0/1 규약에 -1 이 없다.
 let v: &str = if !is_toggle(k) && orig_val(k).map_or(false, |o| o == v) { "-1" } else { v };
 if let Some(&i) = self.model.map.get(k) {'''
assert OLD in t, 'set_val 원문 불일치'
t = t.replace(OLD, NEW, 1)
io.open(P, 'w', encoding='utf-8', newline='\n').write(t)
print('set_val 규칙 반영 완료')
