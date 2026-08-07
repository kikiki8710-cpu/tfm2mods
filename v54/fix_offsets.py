# -*- coding: utf-8 -*-
"""0.5.4 구조체 오프셋 구값 2건 정정.
근거 = v54\athid54.py 실측:
  · struct B 생성자 3연속 저장이 0.5.4 exe 에 정확히 1건(@0x13cfa1d) → athlete_id = 0x800, team = 0x810
  · provider seed: 0.5.4 .text 에서 disp32 0xeaf8 = **0회**, 0xeb28 = 8회 (0.5.3 은 0xeaf8 = 7회)
"""
import sys, io, re
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

P = 'C:/tfm2mods/tfm2_ai_adjust/src/tfm2_ai_adjust.rs'
t = io.open(P, encoding='utf-8').read()
n = 0


def sub1(old, new, tag):
    global t, n
    if old not in t:
        print('  [건너뜀] %s' % tag); return
    t = t.replace(old, new, 1); n += 1
    print('  [ok] %s' % tag)


# ── ① athlete_id ───────────────────────────────────────────
sub1('''const O_ATHLETE_ID: usize = 0x810;   // ⚠★0.5.4 미검증 — **충돌 가능**.
//   0.5.4에서 team/side 가 +0x820 → **+0x810** 으로 내려왔다(에이전트 2건 명령대조로 확정).
//   이 상수는 원래 '0.5.1 추론' 값이라 근거가 약했고, 이제 side 와 같은 자리를 가리킨다.
//   ⟹ 둘 중 하나는 틀렸다. judge_dump 의 팀 필터가 오작동할 수 있다(진단 전용이라 게임엔 무영향).
//   검증법 = MY_ATHLETES 와 매칭되는지 런타임 확인, 또는 0.5.4 athlete 구조체 Debug 필드 슬라이스 역매핑.''',
'''// ★★[08-06 실측 확정] ~~0x810~~ → **0x800**(0.5.4). 0.5.4에서 athlete 구조체가 −0x10 시프트했다.
//   실측 = 위 3연속 저장 패턴(`48 89 be <ID>` / `48 c7 86 <ID+8>` / `48 89 86 <ID+0x10>`)을
//   0.5.4 exe .text 전역 스캔 → **정확히 1건 @0x13cfa1d, ID=0x800, team=0x810** (v54\\athid54.py).
//   ⚠**구 0x810 은 0.5.4에서 team 이다** — 그대로 두면 team_gate 가 athlete_id 자리에 team(0/1/2)을 읽어
//   MY_ATHLETES 와 절대 매칭되지 않는다 ⟹ **선수/클래스 오버라이드가 크래시 없이 조용히 전멸**한다.
//   (같은 결함이 0.4.x→0.5.x 전환 때 0x698 잔재로 한 번 있었다 — 같은 함정의 재발이다.)
const O_ATHLETE_ID: usize = 0x800;''', 'O_ATHLETE_ID 0x810→0x800')

# ── ② provider seed ────────────────────────────────────────
#    ⚠0xeaf8 은 seed 말고 다른 용도로도 쓰인다(vt+0x20 게터, gchild pos). 전부 같은 −0x10 시프트 대상인지
#    개별 확인이 필요하므로, **seed 로 명시된 3곳만** 바꾸고 나머지는 손대지 않는다.
for old, new, tag in [
    ('let seed = rd_u64(world + 0xeaf8).unwrap_or(0); if seed == 0 { return; }',
     'let seed = rd_u64(world + 0xeb28).unwrap_or(0); if seed == 0 { return; }   '
     '// ★[08-06] 0xeaf8→0xeb28 (0.5.4: 구값은 exe에 0회)', 'seed @186'),
    ('seed: rd_u64(sim+0xeaf8).unwrap_or(0),   // ★[07-29] 경기 식별자(주소 재사용 시 남의 경기 맵 반환 차단)',
     'seed: rd_u64(sim+0xeb28).unwrap_or(0),   // ★[07-29] 경기 식별자(주소 재사용 시 남의 경기 맵 반환 차단) '
     '★[08-06] 0xeaf8→0xeb28', 'seed @2994'),
    ('seed: rd_u64(sim+0xeaf8).unwrap_or(0),   // ★[07-29] 경기 식별자(교차오염 차단)',
     'seed: rd_u64(sim+0xeb28).unwrap_or(0),   // ★[07-29] 경기 식별자(교차오염 차단) ★[08-06] 0xeaf8→0xeb28',
     'seed @3052'),
]:
    sub1(old, new, tag)

io.open(P, 'w', encoding='utf-8', newline='\n').write(t)
print('\n정정 %d건' % n)
print('\n남은 0xeaf8(=seed 아닌 용도, 손대지 않음):')
for m in re.finditer(r'.*0xeaf8.*', t):
    print('   ' + m.group(0).strip()[:110])
