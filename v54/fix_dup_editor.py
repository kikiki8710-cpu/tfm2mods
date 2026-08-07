# -*- coding: utf-8 -*-
"""편집기에서 중복 노브(`ex_wait_dist`/`ex_wait_back`) 제거 + `lw_*` 설명을 확인된 사실로 정정.

두 노브가 같은 주소를 패치하면서 **설명이 서로 달랐다**:
  ex_wait_back : "아군 경로의 끝에서 이만큼 뒤 지점"
  lw_back      : "적까지의 경로 길이에서 이만큼 뒤"
디스어셈(0xe727bf `add rax,r12` → `sub rax,0x2bf20` → `cmovae` 0클램프)으로 확인되는 것은
**"경로를 따라간 누적 길이에서 이만큼 뺀 지점"** 까지다. 대상이 적인지 아군인지는
`rsi` 출처가 `lea rsi,[rbx+rdx*8]` 이라 미확정 — 추측을 사실처럼 적지 않는다."""
import sys, io, re
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

P = 'C:/tfm2mods/ai_adjust_editor/src/main.rs'
t = io.open(P, encoding='utf-8').read()
n = 0

# ① 탭에서 제거
OLD = '"§◆ 대기 위치","ex_wait_dist","ex_wait_back",'
if OLD in t:
    t = t.replace(OLD, '', 1); n += 1
    print('  [ok] 탭 항목 제거')
else:
    print('  [건너뜀] 탭 항목')

# ② 원본값 맵에서 제거
for k in ('"ex_wait_back" => "180000",', '"ex_wait_dist" => "180000",'):
    if k in t:
        t = t.replace(k, '', 1); n += 1
print('  [ok] 원본값 맵 정리')

# ③ 설명 제거 (줄 단위)
before = len(t)
t = re.sub(r'(?m)^\s*"ex_wait_(dist|back)" => "[^"]*",\n', '', t)
if len(t) < before:
    n += 1
    print('  [ok] 설명 2개 제거')

# ④ lw_back 설명을 확인된 사실로
OLD_D = '"lw_back" => "앞으로 나갈 때 적과 유지할 간격(유닛, 원본 180000). 적까지의 경로 길이에서 이만큼 뒤에서 멈춥니다. ↑=더 뒤에서 대기(안전) / ↓=적에게 바짝 붙어 대기",'
NEW_D = ('"lw_back" => "대기 지점을 정할 때 물러날 거리(유닛, 원본 180000). **경로를 따라간 누적 길이에서 '
         '이만큼 뺀 지점**에 섭니다(0 미만이면 0). ↑=더 뒤에서 대기(안전) / ↓=바짝 붙어 대기. '
         '⚠기준 대상이 적인지 아군인지는 아직 미확정입니다. 옛 이름 ex_wait_back 도 같은 값으로 동작합니다",')
if OLD_D in t:
    t = t.replace(OLD_D, NEW_D, 1); n += 1
    print('  [ok] lw_back 설명 정정')
else:
    print('  [건너뜀] lw_back 설명 — 원문 불일치')

io.open(P, 'w', encoding='utf-8', newline='\n').write(t)
print('\n적용 %d건' % n)
