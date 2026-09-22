"""run25j.py — o25j.exe 케이스 드라이버(케이스당 프로세스 1개). python run25j.py [--build]"""
import io, os, subprocess, sys, re
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
HERE = os.path.dirname(os.path.abspath(__file__))
EXE = os.path.join(os.environ["LOCALAPPDATA"], "Temp", "tfm2_spanprobe", "o25j.exe")
if "--build" in sys.argv:
    r = subprocess.run(["sh", r"C:/tfm2mods/MIG/_verify3/build.sh", os.path.join(HERE, "o25j.rs").replace("\\", "/")], capture_output=True, text=True)
    print("\n".join(l for l in (r.stdout + r.stderr).splitlines() if l.startswith("error")))
T = 100000
# (이름, 인자, 기대) — 기대 = 명세 logic 독해(IR 정정 반영)에서 손으로 도출
CASES = [
    ("nc_notcleared",   "resp0=0 resp1=0",                                        "tag=-1 self무변화 (L261 is_cleared false → None)"),
    ("cd_239",          f"s_last={T-239} mc_top=-3",                             "tag=-1 (쿨다운 240 미경과: 239<240 → lead_action None)"),
    ("cd_240",          f"s_last={T-240} mc_top=-3",                             "tag=3 PassiveLine(Top) (쿨다운 경계 240 통과)"),
    ("g_pl_top",        "mc_top=-3",                                              "tag=3 PassiveLine line=0(Top) · chats=[CoverLine(9,Top)] · last=T"),
    ("g_pl_mc-2",       "mc_top=-2",                                              "tag=-1 (minion_count<-2 경계: -2 는 미달)"),
    ("g_pl_mid",        "mc_mid=-3",                                              "tag=3 PassiveLine line=1(Mid) · chats=[CoverLine(9,Mid)]"),
    ("g_pl_bigline",    "mc_top=-3 bigline_top=1",                                "tag=-1 (in_big_line(0,Top) true → Top 제외 · Mid 미달)"),
    ("g_pl_bigrecall",  "mc_top=-3 bigline_top_tag1=1",                           "tag=3 (big_goal=Recall(태그6) 은 in_big_line false)"),
    ("g_hp69",          "mc_top=-3 hp=690",                                       "tag=-1 (hp_ratio 69 < 70-agg*20/1000=70 → None)"),
    ("g_hp70",          "mc_top=-3 hp=700",                                       "tag=3 (hp_ratio 70 통과)"),
    ("g_side_bottom",   "mc_bot=-3 cx=900000 cy=100000",                          "tag=3 line=2(Bottom) (x > height-y → Bottom side)"),
    ("g_side_top_eq",   "mc_top=-3 mc_bot=-3 cx=860000 cy=100000",                "tag=3 line=0 (x == height-y → Top: 조건은 x > height-y 엄격)"),
    ("g_pair_uncleared", "mc_top=-3 s_jungle=0 rB0=0",                            "tag=-1 (is_side_cleared: 짝 Rhino↔Bee, Bee 미클리어 → None)"),
    ("g_pair_wrongpair", "mc_top=-3 s_jungle=0 rM0=0",                            "tag=3 (Mushroom 은 Rhino 의 짝이 아님 → 영향 없음)"),
    ("t_gate_eq",       f"mc_top=-3 first_spawn={T+1800}",                        "tag=-1 (gate=first_spawn-tps*30=T · tick<T 거짓 → None)"),
    ("t_gate_plus1",    f"mc_top=-3 first_spawn={T+1801}",                        "tag=3 (gate=T+1 · tick<gate → 통과)"),
    ("t_tut_topsolo",   "mc_top=-3 first_spawn=0 tut=2",                          "tag=3 (TopSolo 는 line phase 아님 → 시간 게이트 미적용)"),
    ("t_tut_line",      "mc_top=-3 first_spawn=0 tut=7",                          "tag=-1 (Line(7) 은 line phase → 게이트 적용 → None)"),
    ("t_tut_total",     "mc_top=-3 first_spawn=0 tut=8",                          "tag=-1 (Total(8) line phase)"),
    ("t_tut_midbot",    "mc_top=-3 first_spawn=0 tut=5",                          "tag=-1 (MidBottom(5) line phase)"),
    ("t_tut_jungleonly", "mc_top=-3 first_spawn=0 tut=6",                         "tag=3 (JungleOnly(6) 는 line phase 아님)"),
    ("cj_team_player",  "ej=2 fa=1 s_team=1 s_pteam=1 s_jungle=3 resp1=100000000 resp0=0 rR1=100040", "tag=-1 self무변화 (L569 is_side_cleared 가 player.team=0 사용 → resp0=0 → None; self.team=1 을 썼다면 Rhino 전환)"),
    ("cj_bot_switch",   f"ej=2 fa=1 s_team=1 s_pteam=1 s_jungle=3 rR1={T+40}",  "tag=-1 · self.jungle=Rhino(rp=-2 통과, Bee 는 rp=-3 탈락) · self.team=1 · last=T · chats 없음(remain=40≠0)"),
    ("cj_top_skipped",  f"ej=2 fa=0 s_team=1 s_pteam=1 s_jungle=3 rS1={T+50} rM1={T+40}", "tag=-1 self무변화 (Stump·Mushroom 모두 region_point=-3 < -2 → 탈락)"),
    ("cj_all_botside",  f"ej=2 fa=2 s_team=1 s_pteam=1 s_jungle=3 rR1={T+40} cx=900000 cy=100000", "self.jungle=Rhino (All: height-y=860000 < x=900000 → (Bee,Rhino))"),
    ("cj_all_topside",  f"ej=2 fa=2 s_team=1 s_pteam=1 s_jungle=3 rR1={T+40} rS1={T+50} rM1={T+40} cx=100000 cy=100000", "tag=-1 self무변화 (All: height-y >= x → (Stump,Mushroom) → 둘 다 rp=-3 탈락)"),
    ("cj_all_eq",       f"ej=2 fa=2 s_team=1 s_pteam=1 s_jungle=3 rR1={T+40} cx=860000 cy=100000", "self무변화 (height-y == x → uge 참 → (Stump,Mushroom))"),
    ("cj_chat_300000",  f"ej=2 fa=1 s_team=1 s_pteam=1 s_jungle=3 rR1={T-10} cx=452000 cy=496000", "self.jungle=Rhino · remain=0 · dist=300000(dist²=9e10, ugt 거짓) → chats=[CounterJungle(18,Rhino)]"),
    ("cj_chat_300001",  f"ej=2 fa=1 s_team=1 s_pteam=1 s_jungle=3 rR1={T-10} cx=451999 cy=496000", "self.jungle=Rhino · remain=0 · dist²>9e10 → chats 없음"),
    ("cj_resp_420",     f"ej=2 fa=1 s_team=1 s_pteam=1 s_jungle=3 rR1={T+420} cx=452000 cy=496000 ms=1000", "self.jungle=Rhino (respawn-1=T+419 < travel300+T+120=T+420 참)"),
    ("cj_resp_421",     f"ej=2 fa=1 s_team=1 s_pteam=1 s_jungle=3 rR1={T+421} cx=452000 cy=496000 ms=1000", "tag=-1 self무변화 (T+420 < T+420 거짓 → 후보 없음)"),
    ("cj_hp69",         f"ej=2 fa=1 s_team=1 s_pteam=1 s_jungle=3 rR1={T+40} hp=690",  "tag=-1 self무변화 (hp 게이트 70)"),
    ("gk_none",         "ej=1",                                                    "tag=-1 (Ganking: 가시 적 없음 → lines 빈 → None)"),
    ("tls_nc",          "resp0=0 resp1=0",                                        "camp_memo before/after 모두 비어야(is_cleared 가 camp_pos 전에 false 반환)"),
    ("tls_cleared",     "mc_top=-3",                                              "after 에 (Rhino=0,team0)·(Bee=3,team0) 만 채워져야 (L261 is_cleared + is_side_cleared)"),
    ("tls_cj",          f"ej=2 fa=1 s_team=1 s_pteam=1 s_jungle=3 rR1={T+40}",    "after 에 (Bee,1)[L261]·(Bee,0)(Rhino,0)[L569 is_side_cleared] + (Bee,1)(Rhino,1)[L585/592/619] 채워져야"),
    ("bt_battle",       "s_team=1 s_pteam=0 s_jungle=3 evis=1 edx=100000 edy=0 ehp=500 resp1=0", "tag=9 Battle (또는 RunAway 면 -1) · chats=[Battle(3,id)]"),
    ("bt_hp49",         "s_team=1 s_pteam=0 s_jungle=3 evis=1 edx=100000 edy=0 ehp=500 hp=490 resp1=0", "tag=-1 (hp_ratio 49 > 49 거짓 → 교전 없음 → is_cleared(team=1) false)"),
    ("bt_hp50",         "s_team=1 s_pteam=0 s_jungle=3 evis=1 edx=100000 edy=0 ehp=500 hp=500 resp1=0", "교전 시도 (50>49)"),
    ("bt_far120000",    "s_team=1 s_pteam=0 s_jungle=3 evis=1 edx=120000 edy=0 ehp=500 resp1=0", "교전 시도 (dist²=14400000000 < 14400000001)"),
    ("bt_far120001",    "s_team=1 s_pteam=0 s_jungle=3 evis=1 edx=120001 edy=0 ehp=500 resp1=0", "tag=-1 (dist² 초과 → 근접 적 없음)"),
    ("bt_novis",        "s_team=1 s_pteam=0 s_jungle=3 evis=0 edx=100000 edy=0 ehp=500 resp1=0", "tag=-1 (is_recent_visible false)"),
    ("bt_ehp_higher",   "s_team=1 s_pteam=0 s_jungle=3 evis=1 edx=100000 edy=0 ehp=1000 hp=999 resp1=0", "tag=-1 (hp_ratio 99 < enemy 100 · 레벨 동일 → 교전 안 함)"),
    ("bt_ehp_equal",    "s_team=1 s_pteam=0 s_jungle=3 evis=1 edx=100000 edy=0 ehp=1000 hp=1000 resp1=0", "교전 시도 (hp_ratio >= enemy_hp_ratio 경계 포함)"),
]
only = [a for a in sys.argv[1:] if not a.startswith("--")]
for name, args, expect in CASES:
    if only and name not in only: continue
    cmd = [EXE] + args.split() + ["diag=0"]
    r = subprocess.run(cmd, capture_output=True, text=True, encoding="utf-8", errors="replace")
    out = r.stdout + r.stderr
    io.open(os.path.join(HERE, f"o25j_{name}.log"), "w", encoding="utf-8").write(" ".join(cmd) + "\n" + out)
    tag = re.search(r"RESULT tag=(-?\d+)", out); selfl = re.search(r"SELF (.*)", out); chats = re.findall(r"CHAT\[\d+\] (\S+)", out)
    b286 = re.search(r"byte286=(\d+)", out)
    if name.startswith("tls_"):
        for l in out.splitlines():
            if l.startswith("camp_memo"): print("    " + l[:400])
    print(f"[{name}] rc={r.returncode} tag={tag.group(1) if tag else '?'} line@286={b286.group(1) if b286 else '?'} {selfl.group(1)[:110] if selfl else 'NO SELF'} chats={chats}\n    기대: {expect}")
