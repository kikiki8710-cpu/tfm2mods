# -*- coding: utf-8 -*-
"""순서도 '그 밖의 설정' 캐치올(99개)을 정리한다.
   ① 판단 흐름에 속하는 것은 해당 단계(2·4·5·6)로 보낸다.
   ② 남는 엔진·진단·대체 스택은 캐치올 안에서 3개 묶음으로 나눈다."""
import sys, io, re
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

P = 'C:/tfm2mods/ai_adjust_editor/src/main.rs'
t = io.open(P, encoding='utf-8').read()

# ── ① 각 단계에 묶음 추가 ────────────────────────────────────
ADD = [
 ("2", 'FlowGroup{ label:"물러날지 — 머릿수·포탑·능력치",\n'
        ' note:"싸울지 물러설지를 정하는 값들입니다. 근처 아군·적과 포탑을 전력으로 환산해 승산을 보고, '
        '선수 능력치(공격성·에고·판단력)로 그 판단을 흔듭니다. `_move`가 붙은 것은 라인전 전용 값이고, '
        '−1이면 한타값을 그대로 따릅니다.",\n'
        ' prefixes:&["numbers_","ally_tower_","tower_","stat_","adv_prof"] },\n'
        ' FlowGroup{ label:"교전에 들어갈지 — 확률·합류",\n'
        ' note:"교전 대상 우선순위별 진입 확률과, 다른 곳에서 벌어진 싸움에 합류할지를 정합니다.",\n'
        ' prefixes:&["eng_role","engage_","t_engage","rc_join"] },\n'
        ' FlowGroup{ label:"안 보이는 적이 어디 있을지",\n'
        ' note:"마지막으로 본 시각·위치·이동속도로 \\"지금 이쯤 있겠다\\"는 원판을 그립니다. 이 원판이 판단의 입력이 됩니다.",\n'
        ' prefixes:&["eg_"] },\n'
        ' FlowGroup{ label:"넥서스를 칠지 / 기지로 갈지",\n'
        ' note:"적 타워 잔여 수를 보고 넥서스로 밀어붙일지, 아직 이르면 대신 무엇을 할지. 그리고 회복하러 돌아갈 체력 기준입니다.",\n'
        ' prefixes:&["an_","disc16_home_hp","jungle_retreat_threat","aggr_lane"] },'),
 ("4", 'FlowGroup{ label:"전역 궁",\n'
        ' note:"아군이 요청한 전역 궁을 쓸지. 근처에 적이 보이면 억제되기 때문에 원본은 교전 중에 거의 나가지 않습니다.",\n'
        ' prefixes:&["gu_"] },'),
 ("5", 'FlowGroup{ label:"시전 후보 검열",\n'
        ' note:"평타·스킬 후보를 경매에 올리기 전에 거릅니다 — 갈 자리가 위험한가, 곧 받을 피해가 큰가, 사거리가 닿는가.",\n'
        ' prefixes:&["cf_"] },\n'
        ' FlowGroup{ label:"재경매 — 골라놓고 다시 겨루기",\n'
        ' note:"접근·추격을 골랐어도 조건이 맞으면 공격 후보와 한 번 더 겨룹니다.",\n'
        ' prefixes:&["re_"] },\n'
        ' FlowGroup{ label:"경매 중 강제 귀환 (0.5.4 신설)",\n'
        ' note:"경매가 도는 동안 \\"지금 도망칠 피해를 못 견딘다\\"고 보이면 다른 모든 후보를 제치고 기지 코너로 물러납니다.",\n'
        ' prefixes:&["auc_"] },'),
 ("6", 'FlowGroup{ label:"어디로 걸어갈지 (0.5.4 신설)",\n'
        ' note:"목적지가 정해진 뒤 실제 경로를 찾는 층입니다. 직교·대각 한 칸 비용과 위험지대 우회 칸 수가 동선을 정합니다. '
        '⚠비용을 원본보다 낮추면 최단경로 보장이 깨집니다(올리는 쪽은 안전).",\n'
        ' prefixes:&["path_"] },'),
]
for no, grp in ADD:
    m = re.search(r'(FlowNode\{ no:"%s",.*?groups:&\[\s*)' % no, t, re.S)
    if not m:
        print('  단계 %s 를 못 찾음' % no); continue
    t = t[:m.end(1)] + grp + '\n ' + t[m.end(1):]
    print('  단계 %s 에 묶음 추가' % no)

# ── ② 캐치올을 3분류로 ───────────────────────────────────────
OLD = '''FlowGroup{ label:"전체",
 note:"성능·진단·대체 구현 스위치, 그리고 아직 어느 단계에도 배정되지 않은 항목입니다. 이 칸이 비어 있는 것이 정상입니다 — 여기에 판단 관련 값이 보이면 위 단계 중 하나에 배정해야 한다는 뜻입니다.",
 prefixes:&["*"] } ] },'''
NEW = '''FlowGroup{ label:"대체 스택 — 어느 판단을 모드가 대신할지",
 note:"켜면 그 판단을 게임 원본 대신 모드의 재구현이 처리합니다. 끄면 게임 원본이 그대로 돕니다. \\
⚠대체를 끄면 그 판단에 딸린 노브들도 함께 무효가 됩니다.",
 prefixes:&["mp_repl","dd7_repl","recall_repl","engage_repl","cond_repl","d12_repl","d14_repl","poke_repl","nx_repl","d4_repl","d7_repl","d15_repl","e9jt"] },
 FlowGroup{ label:"진단 · 계측",
 note:"개발용입니다. 켜면 진단 파일이 쌓이고 경기가 느려질 수 있습니다. 배포 기본은 전부 꺼짐입니다.",
 prefixes:&["perf_measure","read_bench","probe","replay_reset","sp_seen","hang_","judge_dump","log","skip_untuned"] },
 FlowGroup{ label:"엔진 · 기타",
 note:"읽기 경로 같은 엔진 설정과, 어느 단계에도 속하지 않는 항목입니다.",
 prefixes:&["fast_","self_team_only"] },
 FlowGroup{ label:"아직 배정되지 않음",
 note:"여기가 비어 있는 것이 정상입니다 — 항목이 보이면 위 단계 중 하나에 배정해야 한다는 뜻입니다.",
 prefixes:&["*"] } ] },'''
assert OLD in t, '캐치올 원문 불일치'
t = t.replace(OLD, NEW, 1)
io.open(P, 'w', encoding='utf-8', newline='\n').write(t)
print('캐치올 → 4묶음으로 분리')
