# -*- coding: utf-8 -*-
"""pe_noise_exempt 의 **단위 설명**을 코드 실제 동작에 맞게 정정.

detour.rs:1809 이 `pnex / 1000` 을 하므로 **cfg 단위는 옛 단위 그대로(기본 100000)** 이다.
편집기는 기본값을 `100`(나눈 뒤 값)으로, 설명은 "0.5.4부터 단위가 1/1000 로 바뀌었다"로 적어 두었는데
둘 다 cfg 사용자 입장에서 틀렸다 — 이 설명을 믿고 100 을 넣으면 v=0 이 되어 노이즈가 통째로 꺼진다."""
import sys, io
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

E = 'C:/tfm2mods/ai_adjust_editor/src/main.rs'
t = io.open(E, encoding='utf-8').read()
n = 0

OLD_ORIG = '"pe_noise_exempt" => "100",'
NEW_ORIG = '"pe_noise_exempt" => "100000",'   # cfg 단위(모드가 내부에서 ÷1000)
if OLD_ORIG in t:
    t = t.replace(OLD_ORIG, NEW_ORIG); n += 1
    print('  [ok] 기본값 100 → 100000')
else:
    print('  [건너뜀] 기본값')

OLD_D = ('0=전원 면제(노이즈 끔), 100=기본. ⚠0.5.4부터 **단위가 1/1000로 바뀌었습니다**'
         '(옛 100000 → 100). 유효범위 0~127, 실효 0~100.')
NEW_D = ('**기본 100000**. ⚠단위는 옛날 그대로입니다 — 모드가 내부에서 1000으로 나눕니다. '
         '`100000`을 넣어야 원본이고, `100`을 넣으면 0이 되어 **전원 면제 = 노이즈가 통째로 꺼집니다**. '
         '유효 입력 0~127000(1000 단위).')
if OLD_D in t:
    t = t.replace(OLD_D, NEW_D); n += 1
    print('  [ok] 설명 정정')
else:
    print('  [건너뜀] 설명')

io.open(E, 'w', encoding='utf-8', newline='\n').write(t)

# 소스 주석도 오해를 부른다 — "설정값 단위가 ÷1000" 은 코드가 나눠주므로 사용자 입장에선 반대다
D = 'C:/tfm2mods/tfm2_ai_adjust/src/detour.rs'
s = io.open(D, encoding='utf-8').read()
O = '    //     ⟹ 설정값 단위가 ÷1000 (100000 → 100). 실효 구간 0~100.'
N = ('    //     ⟹ **패치되는 값**이 ÷1000 이다. ⚠cfg 단위는 옛날 그대로(기본 100000) — 아래 코드가 나눠준다.\n'
     '    //       cfg 에 100 을 넣으면 v=0 이 되어 술어가 항상 참 = 전원 면제 = 노이즈가 꺼진다(08-06 실사고).')
if O in s:
    s = s.replace(O, N); io.open(D, 'w', encoding='utf-8', newline='\n').write(s); n += 1
    print('  [ok] detour.rs 주석 정정')
else:
    print('  [건너뜀] detour.rs 주석')
print('\n정정 %d건' % n)
