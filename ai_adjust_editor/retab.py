# 2026-08-04 편집기 배치 정리:
#  ① rt_* → planpick(층① 후퇴 트리거라 경매층이 아니다)
#  ② jg_* → planpick(층② 정글 매퍼. 시전 탭에 있을 이유가 없다)
#  ③ nx_* → disc19(넥서스 공수 탭이 따로 있는데 숨기 탭에 있었다)
#  ④ lt_*·d4_* → lane(둘 다 라인 계열 매퍼)
#  ⑤ def 탭의 eh_* 중복 3키 제거(같은 값이 두 탭에 보이면 혼란)
#  ⑥ feel 탭(t_recall 1키) → recall 탭으로 흡수
#  ⑦ judge 탭 112키를 둘로 분할: 경매·점수 / 자리 위험 계산
import re, os, sys
sys.stdout.reconfigure(encoding="utf-8")
P = os.path.join(os.path.dirname(os.path.abspath(__file__)), "src", "main.rs")
s = open(P, encoding="utf-8").read()
orig = s

def cut(block):
    """블록 문자열을 통째로 들어낸다."""
    global s
    assert block in s, "못 찾음: " + block[:60]
    s = s.replace(block, "", 1)

def put_before(anchor, text):
    global s
    assert anchor in s, "앵커 없음: " + anchor[:60]
    s = s.replace(anchor, text + anchor, 1)

# ── ① rt_* 를 judge 에서 떼어 planpick 으로 ──
RT = '''      "§★★후퇴 판단 (내가 곧 죽는가)","rt_a_offset","rt_a_slope","rt_a_base","rt_b_slope","rt_b_base","rt_c_slope","rt_c_base","rt_deadline_min",\n'''
cut(RT)

# ── ② jg_* 를 cast 에서 떼어냄 ──
JG = '''      "§정글 진행 체력 기준","jg_hp_fight","jg_hp_nofight",\n'''
cut(JG)

# ── ③ nx_* 를 hide 에서 떼어 disc19 로 ──
NX = '''      "§넥서스 공격 · 방어 (빠져 있던 자리)","nx_cull_dist19","nx_around_atk","nx_around_def",'''
cut(NX)

# ── ④ lt_* · d4_* 를 hide 에서 떼어 lane 으로 ──
LT = '''      "§라인 총력전","lt_ally_join","lt_around_radius","lt_phase_mask",\n'''
cut(LT)
D4 = '''      "§★라인 배정 · 봇 듀오","d4_hp_safe","d4_partner_dist","d4_ally_radius_a","d4_ally_radius_b",
        "d4_early_leave","d4_from_mid","d4_from_mid_mode","d4_ally_cnt","d4_minion_cnt","d4_gather_radius",\n'''
cut(D4)

# ── ⑤ def 탭 eh_* 중복 제거 ──
DUP = '''\n      "§★사냥 관련 값은 [12] 탭과 공유","eh_abort_hp","eh_trace_arrive","eh_reach_margin",'''
cut(DUP)

# ── 이동 대상에 삽입 ──
# planpick: rt_* + jg_*
put_before('"pl_obj_role","pl_ganker_gate","pl_serpen_phase_mask","pl_epic_phase_min",',
  '"§★★후퇴 트리거 — 내가 곧 죽는가","rt_a_offset","rt_a_slope","rt_a_base","rt_b_slope","rt_b_base","rt_c_slope","rt_c_base","rt_deadline_min",\n      '
  '"§정글을 계속 돌 체력 기준","jg_hp_fight","jg_hp_nofight",\n      '
  '"§어떤 판단을 만들지 고르는 최상위 단계",')

# disc19: nx_*
put_before('"§◆ 공격 (disc18) — 실작동 byte-patch","oi_an_finish_hp","oi_an_cull_dist",',
  '"§방어 측 교전 컷 · 배회 반경 (빠져 있던 자리)","nx_cull_dist19","nx_around_def",\n      '
  '"§공격 측 배회 반경","nx_around_atk",\n      ')

# lane: lt_* + d4_*
put_before('"§라인 대기 (밀고 나간 뒤 어디서 기다릴지)",',
  '"§★라인 배정 · 봇 듀오 (레인 0=탑 1=미드 2=바텀)","d4_hp_safe","d4_partner_dist",'
  '"d4_ally_radius_a","d4_ally_radius_b","d4_early_leave","d4_from_mid","d4_from_mid_mode",'
  '"d4_ally_cnt","d4_minion_cnt","d4_gather_radius",\n      '
  '"§라인 총력전 (한번에 밀어붙일 때)","lt_ally_join","lt_around_radius","lt_phase_mask",\n      ')

# ── ⑥ feel 탭 흡수 ──
m = re.search(r'\n  Tab\{ id:"feel",.*?\n(?=  Tab\{|\s*\];)', s, re.S)
assert m, "feel 탭 못 찾음"
s = s[:m.start()] + s[m.end():]
put_before('"§복귀배율 RNG/정규화",',
  '"§복귀 성향 다이얼","t_recall",\n      ')

# ── ⑦ judge 분할: pe_* / th_* / ae_* 를 새 탭으로 ──
SEC = []
for pat in [
  '"§★★★자리가 얼마나 위험한가 — 무엇까지 셀지"',
  '"§★★자리 평가 — 거리 여유·감쇠"',
  '"§★자리 평가 — 상한과 배율"',
  '"§★★자리 판단의 흔들림 (판단력 3번째 장치)"',
  '"§★★★자리 위험 수치를 만드는 값"',
  '"§라인 수비 후보 점수 — 접근·미니언 자리"']:
    i = s.index(pat)
    ls = s.rfind("\n", 0, i) + 1
    le = s.index('\n      "§', i)          # 다음 섹션 헤더 직전
    SEC.append(s[ls:le + 1])
for b in SEC: cut(b)
NEWTAB = ('  Tab{ id:"posrisk", title:"• [공통] 자리가 위험한지 계산 (position_eval)", keys:&[\n'
          + "".join(SEC) + '      ], note:\n'
          '    "이동할 자리를 고를 때 <b>그 자리가 얼마나 위험한가</b>를 계산하는 곳입니다. '
          '도망·추격·접근·자리잡기가 <b>전부 이 계산을 공유</b>합니다.<br>\\\n'
          '     ★단위는 <b>내 현재 체력의 몇 %</b>입니다 — 나눗셈과 상한을 <b>쓰는 쪽이 아니라 만드는 쪽</b>에서 걸기 때문에, '
          '체력이 낮을수록 같은 적이 급격히 무서워집니다.<br>\\\n'
          '     <b>th_*</b> = 위험 수치를 <b>만드는</b> 값(사거리 띄·상한·반경) · <b>pe_*</b> = 그걸 <b>쓰는</b> 값 · '
          '<b>ae_*</b> = 라인 수비에서 이동 후보를 채점하는 값.<br>\\\n'
          '     ⚠실제 피해 숫자 자체는 경기 시작 때 만들어둔 표에서 오기 때문에, 여기 값을 바꿔도 '
          '<b>"어느 범위까지 무서워하느냐"만 바뀜고 피해량 자체는 그대로</b>입니다.<br>\\\n'
          '     적용확인 = <b>pe_imm.txt · th_imm.txt · ae_imm.txt</b>." },\n\n')
put_before('  Tab{ id:"planpick",', NEWTAB)

open(P, "w", encoding="utf-8").write(s)
print("재배치 완료 (%d → %d bytes)" % (len(orig), len(s)))
