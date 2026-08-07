# 팀전술의 특정 값일 때만 적용되는 노브에 그 조건을 설명문 앞에 명시.
#  근거 = 분기대장 §4 노브↔전술 조건표(2026-08-03 전수 확정) + RE\2026-08-03_Strategy-소비처-전수맵
#  ⚠"전술 무관"으로 확정된 군에는 아무것도 붙이지 않는다(없는 조건을 만들지 않기 위해).
import io, re, sys
sys.stdout.reconfigure(encoding="utf-8")
P = "src/main.rs"
s = io.open(P, encoding="utf-8").read()

# key -> 앞에 붙일 조건 문구
COND = {}

def mark(keys, text):
    for k in keys: COND[k] = text

# ── ① 정글 전술(jng) 갈래 ──
mark(["gk_wait", "gk_hp_base_gank", "gk_window_margin"],
     "【정글 전술에 따라 갈림】 이 값이 걸리는 자리가 <b>성장·커버 / 라인개입 / 카정</b>에 나뉘어 있습니다. "
     "특히 <b>라인개입</b>일 때 가장 많이 발화합니다. ")

# ── ② 에픽·세르펜 견제(ec_*·ep_*) — 스플릿 포지션 일치 조건 ──
mark(["ec_oz_hp", "ec_iz_hp", "ec_self_hp_low", "ec_valid_hp", "ec_commit_hp",
      "ec_count_hp", "ec_engage_dist2", "ec_count_radius", "ec_vision_ticks"],
     "【내 포지션이 전술이 지정한 스플릿 포지션일 때만】 그 외 포지션 선수에게는 이 값이 반영되지 않습니다. ")

# ── ③ 견제 도달 게이트 — 2경로만 전술 전용 ──
mark(["poke_reach_bonus", "poke_serpen_slot"],
     "【주 경로는 전술 무관 · 두 갈래만 전술 전용】 <b>포탑=다이브</b>와 <b>수비=교전</b>일 때만 타는 경로가 따로 있습니다. ")

# ── ④ 라인 배정(dd_*) — 발화는 무관, 배정만 전술 영향 ──
mark(["dd_frontier_mult", "dd_ratio_thr", "dd_ivar2_thr", "dd_near_dist",
      "dd_main_near_dist", "dd_gatee_dist", "dd_cover_count", "dd_survivor_thr",
      "dd_facet_thr", "dd_cover_role_min", "dd_f22e80_margin"],
     "【발화는 전술과 무관 · 다만 “어느 라인에 배정되는지”는 전술 영향】 값 자체는 항상 작동합니다. ")

# ── ⑤ 에픽·세르펜 사냥 — 마무리(fin) 전용은 딱 하나 ──
COND["eh_reach_margin"] = ("【전술 ‘마무리’가 <b>처치 우선</b>일 때만】 교전 우선이면 이 필터 자체가 사라져 후보가 전원 통과합니다. "
                           "★전술이 실제로 행동을 가르는 <b>단 두 곳</b> 중 하나입니다. ")
mark(["eh_flee_clear_hp", "eh_recall_radius", "eh_around_radius", "eh_trace_arrive",
      "eh_band_low", "eh_band_high", "eh_commit_hp", "eh_commit_r_low",
      "eh_commit_r_high", "eh_abort_hp", "eh_abort_dist", "eh_score_norm"],
     "【전술 무관】 사냥 판단 안에 있지만 전술 값과 상관없이 항상 작동합니다. ")

# ── 적용: desc_static 항목 앞에 조건 문구를 끼운다 ──
i = s.index("fn desc_static("); j = s.index("\n}", i)
body = s[i:j]
n, miss = 0, []
for k, cond in COND.items():
    m = re.search(r'("%s" => ")' % re.escape(k), body)
    if not m: miss.append(k); continue
    if body[m.end():m.end() + 3] == "【": continue      # 이미 붙음
    body = body[:m.end()] + cond + body[m.end():]
    n += 1
s = s[:i] + body + s[j:]
io.open(P, "w", encoding="utf-8").write(s)
print("조건 문구 %d/%d 삽입" % (n, len(COND)))
for k in miss: print("  설명 없음:", k)
