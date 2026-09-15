# -*- coding: utf-8 -*-
"""o206 오라클 드라이버 — 케이스당 프로세스 1개. 결과 = o206_cases.log (world/RESULT/self_diff 3줄씩).
usage: python run206.py [케이스이름 접두 필터]"""
import io, os, subprocess, sys
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
EXE = r"C:\Users\jungs\AppData\Local\Temp\tfm2_spanprobe\o206.exe"
HERE = os.path.dirname(os.path.abspath(__file__))
LOG = os.path.join(HERE, "o206_cases.log")

C = "maxhp=1000 "
CASES = [
    # ── A: L1073/L1079 분수 안 (check_recall 진입 즉시)
    ("A1_fountain_full",      C + "line=0 me=0 hp=100",                       "in_recall=0"),
    ("A2_fountain_990",       C + "line=0 me=0 hp=99",                        "in_recall=1"),
    ("A3_fountain_1",         C + "line=0 me=0 hpabs=1",                      "in_recall=1"),
    # ── B: 분수 밖(내 타워 위치) · 적 없음 → L1439 hp<11 / 점수 경로
    ("B1_tower_hp10",         C + "line=0 me=0 pos=tower hp=10",              "in_recall=1"),
    ("B2_tower_hp11",         C + "line=0 me=0 pos=tower hp=11",              "in_recall=0"),
    ("B3_tower_hp30",         C + "line=0 me=0 pos=tower hp=30",              "in_recall=0"),
    ("B4_tower_hp60",         C + "line=0 me=0 pos=tower hp=60",              "in_recall=0"),
    # ── B': 적 5명이 내 타워 옆(+50000..) · 가시 → near_enemies=5 → is_more_enemy → heal_score L1340~1342 (80/70/60/0)
    ("B5_more_enemy_hp30",    C + "line=0 me=0 pos=tower epos=tower vis=1 hp=30",  "in_recall=1"),
    ("B6_more_enemy_hp31",    C + "line=0 me=0 pos=tower epos=tower vis=1 hp=31",  "in_recall=1"),
    ("B7_more_enemy_hp50",    C + "line=0 me=0 pos=tower epos=tower vis=1 hp=50",  "in_recall=1"),
    ("B8_more_enemy_hp51",    C + "line=0 me=0 pos=tower epos=tower vis=1 hp=51",  "in_recall=1"),
    ("B9_more_enemy_hp70",    C + "line=0 me=0 pos=tower epos=tower vis=1 hp=70",  "in_recall=1"),
    ("B10_more_enemy_hp71",   C + "line=0 me=0 pos=tower epos=tower vis=1 hp=71",  "in_recall=0"),
    ("B11_more_enemy_novis",  C + "line=0 me=0 pos=tower epos=tower vis=0 hp=30",  "in_recall=0"),
    ("B12_jungle_more_hp30",  C + "line=1 me=1 pos=tower epos=tower vis=1 hp=30",  "in_recall=1"),
    ("B13_jungle_more_hp81",  C + "line=1 me=1 pos=tower epos=tower vis=1 hp=81",  "in_recall=0"),
    ("B14_jungle_more_hp80",  C + "line=1 me=1 pos=tower epos=tower vis=1 hp=80",  "in_recall=1"),
    # ── C: L232~239 gank_hold(>24) + L254~269 (갱 라인==내 라인)
    ("C1_gank_same_hp25",     C + "line=0 me=0 pos=tower obj=gank oline=0 hp=25", "in_recall=0 chats_len=0"),
    ("C2_gank_same_hp24",     C + "line=0 me=0 pos=tower obj=gank oline=0 hp=24", "chats_len=1 tag=17 +1=0"),
    ("C3_gank_same_hp24_fnt", C + "line=0 me=0 obj=gank oline=0 hp=24",           "in_recall=1 chats_len=1 tag=17"),
    ("C4_dive_same_hp25",     C + "line=0 me=0 pos=tower obj=dive oline=0 hp=25", "in_recall=0 chats_len=0"),
    ("C5_gank_other_hp25",    C + "line=0 me=0 pos=tower obj=gank oline=2 hp=25", "in_recall=0 chats_len=0"),
    ("C6_gank_same_hp25_cmt", C + "line=0 me=0 pos=tower obj=gank oline=0 hp=25 commit0=1", "in_recall=0 v46_commit=0"),
    # ── D: L271~299 인접 갱라인 && in_recall && hp>69
    ("D1_adj_top_mid_hp70",   C + "line=0 me=0 obj=gank oline=1 hp=70",           "in_recall=0 chats_len=1 tag=12 +1=1 +8=0"),
    ("D2_adj_top_mid_hp69",   C + "line=0 me=0 obj=gank oline=1 hp=69",           "in_recall=1 chats_len=0"),
    ("D3_adj_vis_enemy",      C + "line=0 me=0 obj=gank oline=1 hp=70 epos=tower vis=1", "in_recall=1 chats_len=0"),
    ("D3b_adj_vis_far",       C + "line=0 me=0 obj=gank oline=1 hp=70 epos=etower vis=1", "in_recall=0 chats_len=1"),
    ("D4_adj_mid_top",        C + "line=1 me=2 obj=gank oline=0 hp=70",           "in_recall=0 chats_len=1 tag=12 +1=0"),
    ("D5_adj_mid_bot",        C + "line=1 me=2 obj=gank oline=2 hp=70",           "in_recall=0 chats_len=1 tag=12 +1=2"),
    ("D6_nonadj_top_bot",     C + "line=0 me=0 obj=gank oline=2 hp=70",           "in_recall=1 chats_len=0"),
    ("D7_nonadj_bot_top",     C + "line=2 me=3 obj=gank oline=0 hp=70",           "in_recall=1 chats_len=0"),
    ("D8_adj_bot_mid",        C + "line=2 me=3 obj=gank oline=1 hp=70",           "in_recall=0 chats_len=1 tag=12 +1=1"),
    ("D9_adj_dive",           C + "line=0 me=0 obj=dive oline=1 hp=70",           "in_recall=0 chats_len=1 tag=12"),
    ("D10_adj_not_in_recall", C + "line=0 me=0 pos=tower obj=gank oline=1 hp=70", "in_recall=0 chats_len=0"),
    # ── E: v46_commit 선행 → L244~245 / v46_clear 사이트
    ("E1_commit_keep",        C + "line=0 me=0 pos=tower hp=100 commit0=1",       "in_recall=1 v46_commit=1"),
    ("E2_commit_in_heal",     C + "line=0 me=0 hp=100 commit0=1",                 "in_recall=0 v46_commit=0"),
    ("E3_commit_no_phase",    C + "line=0 me=0 pos=tower hp=100 commit0=1 tick=40000", "in_recall=0 v46_commit=0"),
    ("E4_commit_phase_edge",  C + "line=0 me=0 pos=tower hp=100 commit0=1 tick=34199", "v46_commit=1"),
    ("E5_commit_phase_edge2", C + "line=0 me=0 pos=tower hp=100 commit0=1 tick=34200", "v46_commit=0"),
    # ── F: 도주 종료 v46_flee_end (:323~332) — 커밋 중이면 :345 에서 종료
    ("F1_flee_end_nohit",     C + "line=0 me=0 pos=tower hp=100 commit0=1 flee0=1 entry0=100 eflags0=2 appr0=1 hpe0=500 hpm0=500", "flee_episodes=[(100, Top, 3)] nohit=1"),
    ("F2_flee_end_hit",       C + "line=0 me=0 pos=tower hp=100 commit0=1 flee0=1 entry0=100 eflags0=4 appr0=0 hpe0=500 hpm0=499", "flee_episodes=[(100, Top, 4)] nohit=0"),
    ("F3_flee_end_noentry",   C + "line=0 me=0 pos=tower hp=100 commit0=1 flee0=1 appr0=1 hpe0=500 hpm0=500", "flee_episodes=[] nohit=0 v46_flee=0"),
    # ── G: 도주 유지 (:365 else) — 위협 잔존/해소
    ("G1_flee_keep_threats",  C + "line=0 me=0 pos=tower hp=50 flee0=1 entry0=100 hpe0=600 hpm0=600 threats0=e0,e1 epos=tower", "hold_ticks=1 acute=1 approached=1 refuge=1 hp_min=500"),
    ("G2_flee_threats_gone",  C + "line=0 me=0 pos=tower hp=50 flee0=1 entry0=100 hpe0=600 hpm0=600 threats0=e0,e1 epos=etower", "flee_end episode flags=0 hp_min=500"),
    ("G3_flee_keep_far",      C + "line=0 me=0 pos=tower hp=50 flee0=1 entry0=100 hpe0=600 hpm0=600 threats0=e0 epos=tower enear=200000", "still? acute=0"),
    ("G4_flee_still_notacute", C + "line=0 me=0 pos=tower hp=50 flee0=1 entry0=100 hpe0=600 hpm0=600 threats0=e0 epos=tower edx=100000", "hold=1 acute=0 approached=0 refuge=1 v46_flee=1"),
    ("G5_flee_still_edge_out", C + "line=0 me=0 pos=tower hp=50 flee0=1 entry0=100 hpe0=600 hpm0=600 threats0=e0 epos=tower edx=250001", "flee_end (>250000)"),
    ("G6_flee_still_edge_in",  C + "line=0 me=0 pos=tower hp=50 flee0=1 entry0=100 hpe0=600 hpm0=600 threats0=e0 epos=tower edx=250000", "still (<=250000) hold=1 acute=0"),
    ("G7_flee_hpmin_keep",     C + "line=0 me=0 pos=tower hp=50 flee0=1 entry0=100 hpe0=600 hpm0=400 threats0=e0 epos=tower edx=100000", "hp_min stays 400"),
    # ── H: L1255~1261 적 타워 표적 잡음(nearest_enemy Some) · L1248~1251 귀환 잔여 <= tps
    ("H1_etnear_hp30",         C + "line=0 me=0 pos=tower hp=30 etnear=1", "in_recall=1 (L1261 hp<31)"),
    ("H2_etnear_hp31",         C + "line=0 me=0 pos=tower hp=31 etnear=1", "in_recall=0"),
    ("H3_etnear_hp30_enemies", C + "line=0 me=0 pos=tower hp=30 etnear=1 epos=tower vis=1", "in_recall=1 (L1256 hp<31)"),
    ("H4_etnear_hp31_enemies", C + "line=0 me=0 pos=tower hp=31 etnear=1 epos=tower vis=1", "in_recall=1 (heal 70-5)"),
    ("R1_return_remain60",     C + "line=0 me=0 pos=tower hp=100 ret=60", "in_recall=1 (remain 60 <= tps 60)"),
    ("R2_return_remain61",     C + "line=0 me=0 pos=tower hp=100 ret=59", "in_recall=0 (remain 61 > 60)"),
    ("R3_return_remain0",      C + "line=0 me=0 pos=tower hp=100 ret=120", "in_recall=1 (remain 0)"),
    ("R4_return_over",         C + "line=0 me=0 pos=tower hp=100 ret=121", "in_recall=1 (sat_sub 0)"),
    ("D3c_adj_vis_fountain",   C + "line=0 me=0 obj=gank oline=1 hp=70 vis=1",  "적이 자기 분수(라인 밖)에 있고 가시 → is_near_line? → 결과 관찰"),
    ("D11_adj_hp70_gank_top_me_mid", C + "line=1 me=2 obj=gank oline=0 hp=70 vis=1 epos=tower", "적이 내(Mid) 타워에 가시 → my_line_enemy_visible → 유지"),
]

def run(name, args):
    p = subprocess.run([EXE] + args.split(), capture_output=True, text=True, encoding="utf-8", errors="replace", timeout=120)
    out = p.stdout
    lines = {}
    for l in out.splitlines():
        k = l.split("\t", 1)[0]
        lines[k] = l
    return lines, p.returncode, p.stderr[-300:]

if __name__ == "__main__":
    flt = sys.argv[1] if len(sys.argv) > 1 else ""
    with io.open(LOG, "a", encoding="utf-8") as f:
        for (name, args, expect) in CASES:
            if flt and not name.startswith(flt): continue
            lines, rc, err = run(name, args)
            w = lines.get("world", ""); r = lines.get("RESULT", "?"); d = lines.get("self_diff", "")
            sa = lines.get("self_after", "")
            print(f"### {name}\n  args: {args}\n  expect: {expect}\n  {w[:400]}\n  {r[:600]}\n  {d[:600]}")
            if "v46_pending" in sa:
                print("  " + sa[sa.find("v46_pending"):][:500])
            if rc != 0: print("  rc", rc, err)
            f.write(f"### {name}\nargs: {args}\nexpect: {expect}\n{w}\n{r}\n{d}\n{sa}\n\n")
