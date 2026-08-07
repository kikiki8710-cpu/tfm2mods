# -*- coding: utf-8 -*-
"""발행본 흐름도 아티팩트를 0.5.4 기준으로 전면 갱신.
   기존 RE 상세(RVA·조건트리·판단력 4장치)는 그대로 살리고, 0.5.4 신설분과 정정만 얹는다."""
import sys, io, re
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


def insert_before_sub(h4text, html, tag):
    """h4 를 품은 <div class="sub"> 바로 앞에 새 sub 를 끼운다."""
    global t, n
    i = t.find(h4text)
    if i < 0:
        print('  [건너뜀] %s' % tag); return
    j = t.rfind('<div class="sub">', 0, i)
    if j < 0:
        print('  [건너뜀-앵커] %s' % tag); return
    t = t[:j] + html + t[j:]; n += 1
    print('  [ok] %s' % tag)


# ── 1. 버전·날짜 ────────────────────────────────────────────
sub1('<span>0.5.3</span><span>2026-08-03 기준</span>',
     '<span>0.5.4</span><span>2026-08-06 기준</span>', '헤더 버전')
sub1('<b>갱 커버 판단</b>0.5.3에서 만드는 곳 없음',
     '<b>갱 커버 판단</b>0.5.4에서도 만드는 곳 없음', '갱커버 표기')
sub1('RVA는 0.5.3 기준이며 패치마다 바뀝니다', 'RVA는 0.5.4 기준이며 패치마다 바뀝니다', '푸터 RVA')

# ── 2. 리드 문단에 0.5.4 요약 한 줄 ─────────────────────────
sub1('''      리버스엔지니어링으로 지금까지 확인된 범위만 담았습니다. 각 층 오른쪽은 <strong>그 층에 실제로 개입하는 설정값</strong>입니다.''',
'''      리버스엔지니어링으로 지금까지 확인된 범위만 담았습니다. 각 층 오른쪽은 <strong>그 층에 실제로 개입하는 설정값</strong>입니다.<br>
      <b>0.5.4에서 달라진 것</b>: 판단 트리는 그대로입니다(플랜↔담당 함수 대응 1:1 동일).
      실체는 <b>경로·거리 층의 신설</b>과 판단 두 개(<b>경매 중 강제 귀환</b>·<b>넥서스 공격</b>)의 추가입니다.''', '리드')

# ── 3. 판단 생성(2) — 넥서스 공격 ───────────────────────────
insert_before_sub('<h4>맨 위에서 한 번 갈린다</h4>', '''<div class="sub">
          <h4>넥서스로 밀어붙일지 정한다 <em>— 0.5.4 신설</em></h4>
          <ul class="tree">
            <li class="hot"><span class="q">적 타워가 <b>정확히 N개</b> 남았다<br><em>원본 N = 0, 즉 전부 밀어야 한다</em></span><span class="to">→</span>
              <span class="r"><b>넥서스 공격</b>으로 간다</span></li>
            <li><span class="q">아직 타워가 남았다</span><span class="to">→</span>
              <span class="r">대신 <b>라인 방어</b>로 물러난다(폴백)</span></li>
            <li><span class="q">그것도 아니면</span><span class="to">→</span>
              <span class="r">분수에서 <b>귀환 대기</b></span></li>
          </ul>
          <details class="more"><summary>왜 “정확히”가 문제인가</summary>
            <div>
              비교가 <b>이상</b>이 아니라 <b>같음</b>입니다. 그래서 이 값을 2로 올리면
              “타워가 2개일 때만” 넥서스로 가고, 1개나 0개면 오히려 안 갑니다.
              올려서 쓰려면 그 점을 감안해야 합니다.
            </div>
          </details>
        </div>
        ''', '판단2 넥서스공격')

# ── 4. 경매(5) — 강제 귀환 ──────────────────────────────────
insert_before_sub('<h4>① 거르기', '''<div class="sub">
          <h4>0.5.4 신설 — 경매 도중 판을 엎는 귀환 <em>— 다른 모든 후보를 제친다</em></h4>
          <ul class="tree">
            <li><span class="q">AI 사양 버전이 <b>2 이상</b><br><em>실측값 = 2, 즉 켜져 있다</em></span><span class="to">→</span>
              <span class="r">아래 판정을 한다. 아니면 이 판단 자체가 없다</span></li>
            <li class="hot"><span class="q">지금 도망칠 때 <b>맞을 피해 &gt; 현재 체력</b></span><span class="to">→</span>
              <span class="r">기지 코너로 물러나는 행동을 <b>점수 99999</b>로 올린다 — 사실상 무조건 1위</span></li>
            <li class="stop"><span class="q">단, 우리 넥서스가 <b>실제로 맞는 중</b></span><span class="to">→</span>
              <span class="r">취소하고 원래 경매를 계속한다</span></li>
          </ul>
          <details class="more"><summary>점수를 낮추면 “경쟁”하게 만들 수 있다</summary>
            <div>
              99999는 다른 모든 후보를 무조건 이깁니다. 이 점수를 낮추면 <b>정말 급할 때만</b>
              채택되도록 만들 수 있습니다. 비교 대상을 현재 체력 대신 <b>최대 체력</b>으로 바꾸면
              발동 자체가 훨씬 드물어집니다.<br>
              ⚠딸려 나가는 값들(도망 목적지·유지 시간·허용 행동)은 바꿔도 되지만,
              <b>경로탐색 항목과 행동 태그는 건드리면 안 됩니다</b> — 초기화되지 않은 자료를 읽거나
              페이로드 자리가 어긋납니다.
            </div>
          </details>
        </div>
        ''', '경매 강제귀환')

# ── 5. 실행(6) — 경로 시스템 ────────────────────────────────
insert_before_sub('<h4>① 지금 다시 생각할 때인가', '''<div class="sub">
          <h4>0.5.4 신설 — 목적지가 정해진 뒤, 어디로 걸어갈지 <em>0.5.4 증가분의 대부분이 여기</em></h4>
          <ul class="tree">
            <li><span class="q">한 칸 이동 비용</span><span class="to">→</span>
              <span class="r">직교 <b>640</b> / 대각 <b>896</b>(≈640×√2). 대각을 올리면 <b>계단식</b>으로 움직인다</span></li>
            <li class="hot"><span class="q">지나갈 자리가 위험하다<br><em>타워 사거리 등</em></span><span class="to">→</span>
              <span class="r">우회 칸 수 = <code>하한 + 민감도 × (한 칸에서 받을 피해 ÷ 체력)</code>, 하한 2 ~ 상한 60칸</span></li>
            <li><span class="q">미니언 웨이브가 죽는 자리</span><span class="to">→</span>
              <span class="r">비용 <b>1281</b> 가산 — 이 값은 <code>2칸 × 640 + 1</code>이다.
                경로 위험은 전부 <b>“몇 칸 돌아갈지”를 비용으로 옮긴 것</b>이다</span></li>
            <li><span class="q">경로를 얼마나 대충 빨리 찾을지</span><span class="to">→</span>
              <span class="r">탐욕도(원본 7 = 휴리스틱 ×128). <b>0이면 완전탐색</b> — 경로는 좋아지지만 CPU를 많이 쓴다</span></li>
          </ul>
          <details class="more"><summary>⚠비용을 “내리는” 쪽은 위험하다</summary>
            <div>
              길찾기 휴리스틱이 <code>2^탐욕도 × 남은거리</code>라, 간선 비용을 그보다 <b>낮추면</b>
              최단경로 보장이 깨집니다. 크래시가 아니라 <b>길이 이상해지는</b> 형태로 나타납니다.
              안전선은 <b>직교 640 이상 · 대각 896 이상</b>이고, <b>올리는 방향은 언제나 안전</b>합니다.
            </div>
          </details>
        </div>
        ''', '경로 시스템')

# ── 6. 판단력 ③ — pe_noise_exempt 정정 ──────────────────────
sub1('''<dt>설정값</dt><dd><code>pe_noise_exempt</code> <code>pe_noise_amp</code> <code>pe_noise_amp_mode2</code></dd>''',
'''<dt>설정값</dt><dd><code>pe_noise_exempt</code> <code>pe_noise_amp</code> <code>pe_noise_amp_mode2</code></dd>
          <dt>정정</dt><dd>면제선이 보는 스탯은 판단력이 아니라 <b>포지셔닝</b>입니다(2026-08-05 재확인).
            단위도 0.5.4에서 <b>1/1000로 바뀌어</b> 옛 100000이 지금은 100입니다</dd>''', 'pe_noise 정정')

# ── 7. 노브 레일 보강 ───────────────────────────────────────
sub1('<li>pl_obj_role</li>', '<li>pl_obj_role</li><li class="rnd">an_tower_gate</li><li class="rnd">an_attack_sub</li><li>an_fallback</li>', '레일2 an_*')
sub1('<li>au_score_center</li>',
     '<li>au_score_center</li><li class="rnd">auc_flee_version_gate</li><li class="rnd">auc_flee_score</li><li>auc_flee_hp_field</li><li>auc_flee_nexus_mask</li>', '레일5 auc_*')
sub1('<li>ex_order_hold</li>',
     '<li>ex_order_hold</li><li class="rnd">path_orth_cost</li><li class="rnd">path_diag_cost</li><li class="rnd">path_greedy</li><li>path_danger_cost</li><li>path_threat_cap</li>', '레일6 path_*')

# ── 8. 새 섹션: 값이 실제로 먹기까지 ────────────────────────
NEW = '''
  <!-- 값이 먹기까지 -->
  <section class="judgement" style="border-color:var(--brass)">
    <header style="background:var(--brass-bg);border-bottom-color:var(--brass)">
      <h2 style="color:var(--brass)">값을 바꿨는데 아무 일도 안 일어난다면</h2>
      <p>설정값은 저장하는 즉시 걸리지 않습니다. 아래 길을 다 지나야 하고, 한 곳에서 막히면
        <b>그 판의 설정값은 전부 무효</b>입니다.</p>
    </header>
    <div class="jgrid">
      <div class="jcol">
        <h4>먼저 이것부터 확인</h4>
        <p class="where">모드 폴더 · imm_guard_summary.txt</p>
        <dl>
          <dt>checked=756</dt><dd><b>유효한 판</b>입니다. 각 묶음의 적용 수를 읽어도 됩니다</dd>
          <dt>checked=10</dt><dd><b>그 판은 무효</b>입니다. 10은 게임을 켤 때만 적용되는 몫이라,
            나머지 설정값은 아예 걸리지 않았습니다</dd>
          <dt>왜 갈리나</dt><dd>설정값 적용은 <b>후퇴 판단이 한 번이라도 떠야</b> 시작됩니다.
            그 판에 후퇴 판단이 안 떴으면 아무것도 안 걸립니다</dd>
        </dl>
      </div>
      <div class="jcol">
        <h4>그 다음 순서</h4>
        <p class="where">묶음별 <code>*_imm.txt</code></p>
        <dl>
          <dt>applied=N/N</dt><dd>그 묶음은 전부 걸렸습니다</dd>
          <dt>N &lt; M</dt><dd>여러 자리에 걸쳐 있는 값인데 <b>일부만</b> 걸렸다는 뜻입니다 — 반쪽만 작동합니다</dd>
          <dt>파일이 없다</dt><dd>그 묶음이 아예 실행되지 않았습니다</dd>
        </dl>
      </div>
      <div class="jcol">
        <h4>그래도 안 바뀐다면</h4>
        <dl>
          <dt>⛔ 표시</dt><dd>설명 첫머리에 ⛔가 있으면 <b>게임 쪽 코드가 사라진</b> 값입니다.
            바꿔도 아무 일도 일어나지 않습니다</dd>
          <dt>안 뜨는 판단</dt><dd>그 판단 자체가 일반 경기에서 발화하지 않는 경우가 있습니다(예: 단일 라인 전용)</dd>
          <dt>값이 잘림</dt><dd>한 바이트 자리에 들어가는 값은 <b>0~127</b>에서 잘립니다.
            그보다 크게 넣어도 반영되지 않습니다</dd>
        </dl>
      </div>
    </div>
  </section>
'''
sub1('  <!-- 공백 -->', NEW + '\n  <!-- 공백 -->', '새 섹션 삽입')

# ── 9. 푸터에 문서 링크 ─────────────────────────────────────
sub1('현행 값은 <code>MODS\\MIGRATION.md §7</code>.',
     '현행 값은 <code>MODS\\MIGRATION.md §7</code>.<br>'
     '문서 = <code>mods_report\\tfm2_ai_adjust\\</code> 의 '
     '<code>00_흐름도.md</code> · <code>01_구조.md</code> · <code>02_구현정보.md</code> · <code>03_시행착오.md</code>. '
     '설정값 규모(2026-08-06 실측) = 배선 <b>577</b> · 편집기 노출 <b>475</b> · 패치 자리 <b>1001</b>.', '푸터 문서')

io.open(P, 'w', encoding='utf-8', newline='\n').write(t)
print('\n적용 %d건 / 최종 %d바이트' % (n, len(t)))
