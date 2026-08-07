# -*- coding: utf-8 -*-
"""달성률 표기 갱신 — 0.5.4에서 무엇이 재측정됐고 무엇이 아닌지를 분명히 한다.
   ★기존 퍼센트는 '판단 트리'에 대한 0.5.3 측정이다. 0.5.4가 판단 트리를 안 바꿨으므로
     그대로 유효하지만, **이번에 다시 잰 것은 아니다** — 그 사실을 적지 않으면 독자가 최신 측정으로 오해한다."""
import sys, io
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

P = (r'C:\Users\dev\AppData\Local\Temp\claude\C--Users-dev-Desktop-claude-tfm2--claude-worktrees-'
     r'item-tactics-conflict-check-86a5d3\e2b9bb3b-0660-4ff5-9ee7-51903acd7108\scratchpad\flow.html')
t = io.open(P, encoding='utf-8').read()
n = 0


def sub1(old, new, tag):
    global t, n
    if old not in t:
        print('  [건너뜀] %s' % tag); return
    t = t.replace(old, new, 1); n += 1
    print('  [ok] %s' % tag)


# ── 1. 범례에 "달성률이 무엇을 잰 것인가" 명시 ──────────────
sub1('''      <span><i class="dot rnd"></i><b>판단력이 끼어드는 곳</b></span>
    </div>''',
'''      <span><i class="dot rnd"></i><b>판단력이 끼어드는 곳</b></span>
      <span><i class="dot part"></i><b>0.5.4 신설 · 조사 진행 중</b></span>
    </div>
    <div class="legend" style="border-left:2px solid var(--brass)">
      <span>각 층의 <b>달성률</b>은 <b>판단 트리</b>를 얼마나 훑었는지를 가리킵니다.
      0.5.4는 판단 트리를 바꾸지 않았으므로(플랜↔담당 함수 대응 1:1 동일) 그 값이 그대로 유효하지만,
      <b>이번 버전에서 다시 잰 것은 아닙니다</b> — 0.5.3 시점 측정입니다.
      반면 <b>0.5.4에 새로 생긴 층</b>(경로·거리)은 이번에 처음 조사한 것이라 따로 표시했습니다.</span>
    </div>''', '범례 주석')

# ── 2. 실행층(6) 배지 옆에 0.5.4 신설 층 상태 추가 ──────────
sub1('''          <span class="status"><i class="dot ok"></i>확정 · 약 99%</span>''',
'''          <span class="status"><i class="dot ok"></i>판단 트리 약 99%
            &nbsp;·&nbsp; <i class="dot part"></i>경로 층(0.5.4 신설) 약 70%</span>''', '실행층 배지')

# ── 3. 경매층 배지 ─────────────────────────────────────────
sub1('''          <span class="status"><i class="dot ok"></i>점수식·선택 절차 확정</span>''',
'''          <span class="status"><i class="dot ok"></i>점수식·선택 절차 확정
            &nbsp;·&nbsp; <i class="dot part"></i>강제 귀환(0.5.4 신설) 확정</span>''', '경매층 배지')

# ── 4. 판단 생성층 배지 ────────────────────────────────────
sub1('''          <span class="status"><i class="dot ok"></i>확정 · 약 85%</span>
        </div>
        <div class="rvas">
          <span class="rva">0xd452e0<em>plan 생산 본체</em></span>''',
'''          <span class="status"><i class="dot ok"></i>확정 · 약 85%
            &nbsp;·&nbsp; <i class="dot part"></i>넥서스 공격(0.5.4 신설) 확정</span>
        </div>
        <div class="rvas">
          <span class="rva">0xd452e0<em>plan 생산 본체</em></span>''', '판단생성 배지')

# ── 5. "아직 안 본 곳" — 0.5.4에서 새로 열린 것 추가 ────────
sub1('''    <div class="row">
      <span><b>후보가 하나도 안 남았을 때</b>의 처리''',
'''    <p style="margin-top:.8rem"><b>0.5.4에서 새로 열린 칸</b> — 아래는 이번 버전에 생긴 것이라
      위 목록과 별개입니다.</p>
    <div class="row">
      <span><b>대각 이동을 아예 금지</b>하는 스위치 — 값이 코드가 아니라 <b>데이터 영역의 방향표 4벌</b>에 있고,
        그 표 앞부분을 무관한 함수가 나눠 쓰고 있어 손대는 방식이 다릅니다. <b>보류 중</b>입니다</span>
      <span><b>위험지대 우회 칸 수</b>의 산식은 확인했지만, <b>위험원을 무엇으로 세는지</b>의 전체 목록은
        아직 다 훑지 못했습니다</span>
      <span><b>중립 몬스터가 나를 노리는지</b>를 보는 판정 — 대상 종류가 정글·에픽·세르펜이라는 것은
        <b>근거가 강한 추정</b>이고, 한 경기 계측이면 확정됩니다</span>
      <span><b>불사 상태 특례</b>가 실제로 어떤 선수에게 걸리는지 — 필드는 확정했지만
        발동하는 선수를 실제로 본 적은 없습니다</span>
    </div>
    <p style="margin-top:1rem"><b>이전부터 남아 있던 칸</b></p>
    <div class="row">
      <span><b>후보가 하나도 안 남았을 때</b>의 처리''', '0.5.4 신규 공백')

io.open(P, 'w', encoding='utf-8', newline='\n').write(t)
print('\n적용 %d건' % n)
