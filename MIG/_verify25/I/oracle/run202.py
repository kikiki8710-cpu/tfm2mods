"""run202.py — 25차 I 오라클 드라이버: 케이스마다 game/mine 프로세스를 따로 띄워 대조한다.
사용: python -X utf8 run202.py [케이스이름...]   (없으면 전부)
출력: oracle/o202_<case>_{game,mine}.log + 표(stdout) + summary.json
"""
import io, os, re, sys, json, subprocess
HERE = os.path.dirname(os.path.abspath(__file__))
EXE = os.path.join(os.environ["LOCALAPPDATA"], "Temp", "tfm2_spanprobe", "o202.exe")
J = dict(cx=432000, cy=656000)            # 미드 인접 정글 셀(scan: y20 gx13 = J0)
B = dict(cx=560000, cy=528000)            # bush 14 셀(scan: y16 gx17) — 드라이버가 champ_bush==bush 까지 보정
def E(i, dx, dy, seen=1, **kw):
    d = {f"e{i}x": B["cx"] + dx, f"e{i}y": B["cy"] + dy, f"e{i}seen": seen}
    for k, v in kw.items(): d[f"e{i}{k}"] = v
    return d
def EJ(i, dx, dy, seen=1, **kw):
    d = {f"e{i}x": J["cx"] + dx, f"e{i}y": J["cy"] + dy, f"e{i}seen": seen}
    for k, v in kw.items(): d[f"e{i}{k}"] = v
    return d
def A_(i, dx, dy, base=B, **kw):
    d = {f"a{i}x": base["cx"] + dx, f"a{i}y": base["cy"] + dy}
    for k, v in kw.items(): d[f"a{i}{k}"] = v
    return d

CASES = {
 # --- L101 phase
 "A1_phase_change_jungle": dict(phase=3, **J),
 "A2_phase_setup": dict(phase=1, **J),          # tag 7 → 진행(정글 아님·안 보임 → L142 → L155 None)
 # --- L107 노출+정글: L122 분기
 "B1_v1_allies_gt_enemies": dict(version=1, vis=1, **J, **EJ(0, 100000, 0), **A_(0, 50000, 0, base=J)),   # allies 2 > enemies 1 → L127 mgb(v1 Some)
 "B2_v1_enemies_eq": dict(version=1, vis=1, **J, **EJ(0, 100000, 0)),                                      # allies 1 == enemies 1 → else → L136 Response
 "B3_v1_enemies_zero_far_seen": dict(version=1, vis=1, **J, **EJ(0, 400000, 0)),                            # enemies_near 0 → else → 멀리서 보이는 적 → L136
 "B4_v1_no_seen": dict(version=1, vis=1, **J, **EJ(0, 100000, 0, seen=0)),                                  # 안 보임 → nearest None → L142
 "B5_v1_allies_gt_hp39": dict(version=1, vis=1, **J, **EJ(0, 100000, 0, hp=39, maxhp=100), **A_(0, 50000, 0, base=J)),  # 적 hp 39% → enemies 0 → else → L136(보이므로)
 "B5b_v1_allies_gt_hp40": dict(version=1, vis=1, **J, **EJ(0, 100000, 0, hp=40), **A_(0, 50000, 0, base=J), e0maxhp=100),
 "B6_v1_dist_250000": dict(version=1, vis=1, **J, **EJ(0, 250000, 0), **A_(0, 50000, 0, base=J)),         # d²=250000² < 62500000001 → 셈
 "B6b_v1_dist_250001": dict(version=1, vis=1, **J, **EJ(0, 250001, 0), **A_(0, 50000, 0, base=J)),        # 안 셈 → enemies 0 → L136
 "B7_v2_allies_gt": dict(version=2, vis=1, **J, **EJ(0, 100000, 0), **A_(0, 50000, 0, base=J)),           # v2: mgb 가 None 이면 L142 로
 "B8_v1_nearest_pick": dict(version=1, vis=1, **J, **EJ(0, 100000, 0), **EJ(1, 60000, 0), **EJ(2, 60000, 0)),  # 최근접(첫 최소) = e1
 # --- L142~L155
 "C1_not_bush": dict(version=1, **J),                                   # champ_bush != bush → None(L155)
 "C2_in_bush_empty": dict(version=1, **B),                              # bush 일치 · 적 없음 → L245 None
 # --- L158~ 루프
 "D1_loop_dbg": dict(version=1, **B, **E(0, 100000, 0)),                # 적 1 근접 · 아군 1 → L205 continue? → check_kill → L232 d>range → L240
 "D2_ally_gt": dict(version=1, **B, **E(0, 100000, 0), **A_(0, 30000, 0)),   # allies2 2 > enemies2 1 → L205 통과 → mtt(ms=1 → 거대) → L216: mdt>120 && a>=e → L217
 "D3_ally_gt_ms": dict(version=1, **B, **E(0, 100000, 0, ms=10000), **A_(0, 30000, 0)),  # mtt ≤120 → edt ≤60? else a>e → L214
 "D4_ally_eq_wait": dict(version=1, **B, **E(0, 100000, 0), **A_(0, 30000, 0), **E(1, 100000, 30000), wait=99999),  # a2==e2 → L205 continue(edt>30) ...
 "D5_range_in": dict(version=1, **B, **E(0, 15000, 0)),                 # 루프: a1<=e1 & edt>30 → continue; L232: d=15000 ≤ range → L241 mgb
 "D6_range_out": dict(version=1, **B, **E(0, 30000, 0)),                # d=30000 > range → L240 None
 "D7_wait_gate": dict(version=1, **B, **E(0, 100000, 0), **A_(0, 30000, 0), wait=3181, hp=1),  # L216 실패 유도 어려움 — 관찰용
 "D8_v2_loop": dict(version=2, **B, **E(0, 100000, 0), **A_(0, 30000, 0)),
 "D9_two_enemies_order": dict(version=1, **B, **E(0, 120000, 0), **E(1, 80000, 0), **A_(0, 30000, 0), **A_(2, 30000, 10000)),  # 슬롯 순서대로 e0 먼저
 "D10_unseen_enemy_near": dict(version=1, **B, **E(0, 100000, 0, seen=0), **A_(0, 30000, 0)),  # 안 보이면 near_enemies_p 에 없음 → L245
 # --- 피해량·이동속도 주입으로 die_tick 게이트 열기 (die_tick ≈ 접근시간(거리-사거리)/ms + 처치시간)
 "E1_L212_edt_le60": dict(version=1, **B, **E(0, 100000, 0, ms=10000, hp=1, maxhp=100), ms=10000, cdmg=100000, hp=5000, maxhp=5000),   # edt≤30 → L205 통과(a1<=e1 이어도) → mtt 17 → L211 → L212
 "E2_L205_continue": dict(version=1, **B, **E(0, 100000, 0, ms=10000, hp=100000, maxhp=100000), ms=10000, cdmg=1, hp=5000, maxhp=5000),  # edt>30 & 1<=1 → continue → L240
 "E2b_L214_ally": dict(version=1, **B, **E(0, 100000, 0, ms=10000, hp=100000, maxhp=100000), **A_(0, 30000, 0), ms=10000, cdmg=1, hp=5000, maxhp=5000),  # edt>60 · a2 2>1 → L214
 "E3_L217": dict(version=1, **B, **E(0, 100000, 0), **A_(0, 30000, 0), ms=10000),   # mtt 거대 · mdt 거대>120 · 2>=1 → L217
 "E4_L216_false_wait181": dict(version=1, **B, **E(0, 150000, 0, range=200000, dmg=100000), **A_(0, 30000, 0), ms=10000, hp=1, maxhp=100, cdmg=1, wait=3181),  # mdt≤120 → L218 rem181>180 → 아무것도 → L240
 "E4b_L220_L221_wait180": dict(version=1, **B, **E(0, 150000, 0, range=200000, dmg=100000), **A_(0, 30000, 0), ms=10000, hp=1, maxhp=100, cdmg=1, wait=3180),  # rem180 → L220 2>1 → L221
 "E4c_L220_false": dict(version=1, **B, **E(0, 150000, 0, range=200000, dmg=100000), ms=10000, hp=1, maxhp=100, cdmg=1, wait=0),  # a2 1>1 거짓 → L240
 "E5_check_kill": dict(version=1, **B, **E(0, 100000, 0, hp=1, maxhp=100), **A_(0, 30000, 0, dmg=100000), ms=10000, hp=5000, maxhp=5000, cdmg=100000),
 "E6_v2_hopeless": dict(version=2, **B, **E(0, 100000, 0, dmg=1000, hp=5000, maxhp=5000, ms=10000), **A_(0, 30000, 0), hp=100, maxhp=100, cdmg=30),
 "E7_v2_L212": dict(version=2, **B, **E(0, 100000, 0, ms=10000, hp=1, maxhp=100), ms=10000, cdmg=100000, hp=5000, maxhp=5000),
 "E8_dist_cut_150000": dict(version=1, **B, **E(0, 150000, 0, ms=10000, hp=1, maxhp=100), ms=10000, cdmg=100000, hp=5000, maxhp=5000),   # d²=cut² → 포함(<=) → L212
 "E8b_dist_cut_150001": dict(version=1, **B, **E(0, 150001, 0, ms=10000, hp=1, maxhp=100), ms=10000, cdmg=100000, hp=5000, maxhp=5000),  # 제외 → L232 d>range → L240
 # --- check_kill 경로·사거리 경계
 "F1_check_kill_L227": dict(version=1, **B, **E(0, 100000, 0, range=200000, dmg=100000, hp=1, maxhp=100), **A_(0, -140000, 0), ms=10000, hp=1, maxhp=100, cdmg=100000, wait=3181),  # na=[0,1]>ne=[0] · na2=[1]<=... L216 거짓(mdt≤120) → L218 대기 → check_kill?
 "F1b_check_kill_L227_v2": dict(version=2, **B, **E(0, 100000, 0, range=200000, dmg=100000, hp=1, maxhp=100), **A_(0, -140000, 0), ms=10000, hp=1, maxhp=100, cdmg=100000, wait=3181),
 "F2_range_eq_20000": dict(version=1, **B, **E(0, 20000, 0)),     # d == attack_range(20000) → ugt 거짓 → L241
 "F3_range_20001": dict(version=1, **B, **E(0, 20001, 0)),        # d > range → L240 None
 "F4_L227_seed": dict(version=1, **B, **E(0, 100000, 0, range=200000, dmg=100000, hp=1, maxhp=100), **A_(0, -140000, 0), ms=10000, hp=1, maxhp=100, cdmg=100000, wait=3181, seed=1234),
}

def run(who, args, extra=None):
    argv = [EXE, f"who={who}"] + [f"{k}={v}" for k, v in args.items()]
    if extra: argv += [f"{k}={v}" for k, v in extra.items()]
    p = subprocess.run(argv, capture_output=True, text=True, encoding="utf-8", errors="replace", timeout=120)
    return p.stdout + ("\nSTDERR:" + p.stderr if p.returncode != 0 else "")

def parse_game(out):
    d = {}
    m = re.search(r"^GAME\t(.*)$", out, re.M)
    if not m: d["panic"] = True; return d
    for kv in m.group(1).split("\t"):
        if "=" in kv:
            k, v = kv.split("=", 1); d[k.strip()] = v.strip()
    m2 = re.search(r"chat0=(Some\(\([^)]*\)\)|None)", m.group(1))
    if m2: d["chat0"] = m2.group(1)
    m = re.search(r'bush: (\d+), champ_bush: (\d+)', out)
    if m: d["bush"] = int(m.group(1)); d["champ_bush"] = int(m.group(2))
    d["infos"] = re.findall(r"^INFO\t(.*)$", out, re.M)
    m = re.search(r"^XCHECK\t(.*)$", out, re.M)
    d["xcheck"] = m.group(1) if m else ""
    return d

def parse_mine(out):
    m = re.search(r"^MINE\t(.*)$", out, re.M)
    return m.group(1) if m else "PANIC?"

def main():
    names = sys.argv[1:] or list(CASES)
    rows = []
    for n in names:
        args = dict(CASES[n])
        args.setdefault("debug", 1)
        # bush 보정: L142 경로 케이스(C/D)는 champ_bush == bush 가 되도록 셀 중심을 최대 4회 따라간다
        g = run("game", args)
        gd = parse_game(g)
        if n[0] in "CDEF" and n != "C1_not_bush":
            for it in range(4):
                if "bush" not in gd or gd["bush"] == gd["champ_bush"]: break
                # bushes 그리드에서 bush id 셀 찾기 → scan 출력 대신 map 재현: 셀 좌표는 미리 표(아래)로
                cell = BUSH_CELLS.get(gd["bush"])
                if not cell: break
                dx = cell[0] - args["cx"]; dy = cell[1] - args["cy"]
                for k in list(args):
                    if re.match(r"[ea]\dx$", k): args[k] += dx
                    if re.match(r"[ea]\dy$", k): args[k] += dy
                args["cx"], args["cy"] = cell
                g = run("game", args); gd = parse_game(g)
        io.open(os.path.join(HERE, f"o202_{n}_game.log"), "w", encoding="utf-8").write(" ".join(f"{k}={v}" for k, v in args.items()) + "\n" + g)
        margs = dict(args)
        if "bush" in gd: margs["bush"] = gd["bush"]
        mo = run("mine", margs)
        io.open(os.path.join(HERE, f"o202_{n}_mine.log"), "w", encoding="utf-8").write(" ".join(f"{k}={v}" for k, v in margs.items()) + "\n" + mo)
        md = parse_mine(mo)
        # 비교: game tag/entry_src/target/min_trace/chat ↔ mine out
        gsum = "PANIC" if gd.get("panic") else ("None" if gd.get("tag", "").startswith("None") else f"Battle es={gd.get('entry_src')} goal={gd.get('goal_tag')} tgt={gd.get('target')} mt={gd.get('min_trace')}")
        mm = re.search(r"out=(None\(\"[^\"]*\"\)|Battle \{[^}]*\})", md)
        msum = mm.group(1) if mm else md
        ok = None
        if gsum == "PANIC": ok = "PANIC" in md
        elif gsum == "None": ok = msum.startswith("None")
        else:
            m2 = re.search(r"entry_src: (\d+), goal_tag: (\d+), target: (\d+), min_trace: (\d+)", msum)
            if m2:
                es, gt, tg, mt = m2.groups()
                ok = es == gd.get("entry_src") and gt == gd.get("goal_tag") and (gt != "0" or (tg == gd.get("target") and mt == gd.get("min_trace")))
            else: ok = False
        chat_g = gd.get("chat0", "None"); chat_m = re.search(r"chat=(\S+)", md); chat_m = chat_m.group(1) if chat_m else "?"
        rows.append(dict(case=n, game=gsum, mine=msum, ok=ok, chat_g=chat_g, chat_m=chat_m, runs=gd.get("runs"), self_diff=gd.get("self_diff"), rnd=gd.get("rnd_changed"), bush=gd.get("bush"), champ_bush=gd.get("champ_bush"), infos=len(gd.get("infos", [])), xcheck=gd.get("xcheck")))
        print(f"{n:28s} ok={ok!s:5s} game={gsum:45s} mine={msum}")
        print(f"{'':28s}  chat g={chat_g} m={chat_m}  runs={gd.get('runs')} self_diff={gd.get('self_diff')} rnd={gd.get('rnd_changed')} bush={gd.get('bush')}/{gd.get('champ_bush')} infos={len(gd.get('infos', []))}")
        for i in gd.get("infos", [])[:6]: print("   INFO", i[:230])
    json.dump(rows, io.open(os.path.join(HERE, "summary.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    print("MATCH", sum(1 for r in rows if r["ok"]), "/", len(rows))

# scan 출력에서 손으로 옮긴 bush id → 대표 셀 중심 좌표 (gx*32000+16000, gy*32000+16000)
BUSH_CELLS = {14: (17 * 32000 + 16000, 16 * 32000 + 16000), 13: (20 * 32000 + 16000, 15 * 32000 + 16000), 18: (15 * 32000 + 16000, 20 * 32000 + 16000),
              17: (4 * 32000 + 16000, 20 * 32000 + 16000), 15: (25 * 32000 + 16000, 18 * 32000 + 16000), 20: (18 * 32000 + 16000, 25 * 32000 + 16000),
              12: (9 * 32000 + 16000, 14 * 32000 + 16000), 11: (12 * 32000 + 16000, 12 * 32000 + 16000), 10: (6 * 32000 + 16000, 12 * 32000 + 16000),
              8: (14 * 32000 + 16000, 9 * 32000 + 16000), 5: (12 * 32000 + 16000, 6 * 32000 + 16000), 6: (4 * 32000 + 16000, 7 * 32000 + 16000),
              4: (20 * 32000 + 16000, 4 * 32000 + 16000), 3: (7 * 32000 + 16000, 4 * 32000 + 16000)}
if __name__ == "__main__":
    main()
