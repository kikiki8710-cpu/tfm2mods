"""이벤트 트레이서 판독 — 0.6.5 시작 직후 우물 대기 (2026-10-21).

events_<run>.jsonl(모드 로그 폴더, 기본 E:\\claude\\tfm2\\logs\\game_mods\\tfm2_ai_analysis_verify) 를 읽어
경기별·선수별로 "어느 조건에서 막혀 우물에 남았나 / 언제 나갔나"를 시간순으로 정리한다.
출력 = E:\\claude\\tfm2\\logs\\tfm2_ai_analysis_verify\\fountain_wait\\<run>_report.md (+ 화면 요약)
사용: python -B -X utf8 tools/events_report.py [events_*.jsonl ...] [--first-sec 180]
"""
import argparse, collections, glob, json, os, sys

LOG_IN = r"E:\claude\tfm2\logs\game_mods\tfm2_ai_analysis_verify"
OUT = r"E:\claude\tfm2\logs\tfm2_ai_analysis_verify\fountain_wait"
SUB = {0: "LineDefense", 1: "LineSafe", 2: "LineWait", 3: "Recall", 4: "Jungle", 5: "Battle"}  # RE 이름표(0x3b06668)
BIG = {1: "PassiveLine", 5: "PassiveJungle", 7: "big7(d<2)"}
FPS = 60  # 모드 프레임(post_update) — 화면 프레임 기준 근사 초


def sub_name(v):
    if v is None:
        return "?"
    if v >= 1 << 63:
        tag = v & 0xffff
        return SUB.get(tag, f"sub{tag}")
    return "data-variant"  # 데이터를 품은 변형(포인터 값) — 이름표 밖


def big_name(d):
    if d is None:
        return "?"
    i = d - 2 if d >= 2 else 7
    return BIG.get(i, f"big{i}")


def load(paths):
    rows = []
    for p in paths:
        for ln in open(p, encoding="utf-8"):
            try:
                rows.append(json.loads(ln))
            except ValueError:
                pass
    return rows


def split_matches(rows):
    """프레임 순서로 보고, 같은 선수의 t 가 크게 되돌아가거나 프레임 공백이 30초 넘으면 새 경기."""
    rows.sort(key=lambda r: r["f"])
    out, cur, last_f, last_t = [], [], None, {}
    for r in rows:
        key = (r.get("team"), r.get("slot"))
        t = r.get("t")
        new = last_f is not None and r["f"] - last_f > 30 * FPS
        if t is not None and key in last_t and last_t[key] is not None and t + 600 < last_t[key]:
            new = True
        if new and cur:
            out.append(cur); cur = []; last_t = {}
        cur.append(r); last_f = r["f"]; last_t[key] = t
    if cur:
        out.append(cur)
    return out


def report(match, first_sec):
    f0 = match[0]["f"]
    t0 = min((r["t"] for r in match if r.get("t") is not None), default=None)
    by = collections.defaultdict(list)
    for r in match:
        by[(r.get("team"), r.get("slot"))].append(r)
    lines = [f"- 프레임 {f0}~{match[-1]['f']} ({(match[-1]['f'] - f0) / FPS:.0f}초) · 이벤트 {len(match)} · t0={t0}", "",
             "| 팀 | 슬롯 | 라인 | 빅플랜 | 첫 커밋 서브플랜 | 우물대기 강제(force) | 강제 구간(초) | 마지막 short | wait/travel(마지막) | gate false | 우물 이탈(비Recall 첫 커밋, 초) |",
             "|---|---|---|---|---|---|---|---|---|---|---|"]
    for (team, slot), rs in sorted(by.items(), key=lambda x: (x[0][0] or 0, x[0][1] or 0)):
        commits = [r for r in rs if r["ev"] == "commit"]
        force = [r for r in rs if r["ev"] == "force"]
        shop = [r for r in rs if r["ev"] == "shop"]
        cmpv = [r for r in rs if r["ev"] == "cmp"]
        gate_f = sum(1 for r in rs if r["ev"] == "gate" and r.get("gate") == 0)
        lane = next((r.get("lane") for r in commits if r.get("lane") is not None), "?")
        big = big_name(commits[0].get("big")) if commits else "?"
        first_sub = sub_name(commits[0].get("sub")) if commits else "?"
        fr = f"{(force[0]['f'] - f0) / FPS:.1f}~{(force[-1]['f'] - f0) / FPS:.1f}" if force else "-"
        last_short = shop[-1].get("short") if shop else "-"
        wt = f"{cmpv[-1].get('wait')}/{cmpv[-1].get('travel')}" if cmpv else "-"
        leave = next(((r["f"] - f0) / FPS for r in commits if sub_name(r.get("sub")) != "Recall"), None)
        lines.append(f"| {team} | {slot} | {lane} | {big} | {first_sub} | {len(force)} | {fr} | {last_short} | {wt} | {gate_f} | "
                     f"{'-' if leave is None else f'{leave:.1f}'} |")
    lines += ["", f"### 선수별 첫 {first_sec}초 서브플랜 흐름(커밋 기준, 바뀔 때만)", ""]
    for (team, slot), rs in sorted(by.items(), key=lambda x: (x[0][0] or 0, x[0][1] or 0)):
        seq, prev = [], None
        for r in rs:
            if r["ev"] != "commit" or (r["f"] - f0) / FPS > first_sec:
                continue
            s = sub_name(r.get("sub"))
            if s != prev:
                hp = f"{r.get('hp')}/{r.get('maxhp')}" if r.get("hp") is not None else "?"
                seq.append(f"{(r['f'] - f0) / FPS:.1f}s {s}(골드 {r.get('gold')}, hp {hp}, ({r.get('x')},{r.get('y')}))")
                prev = s
        lines.append(f"- 팀{team} 슬롯{slot}: " + (" → ".join(seq) if seq else "(커밋 기록 없음)"))
    return lines


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("files", nargs="*")
    ap.add_argument("--first-sec", type=int, default=180)
    a = ap.parse_args()
    files = a.files or sorted(glob.glob(os.path.join(LOG_IN, "events_*.jsonl")), key=os.path.getmtime)[-1:]
    if not files:
        sys.exit(f"이벤트 파일 없음: {LOG_IN}")
    rows = load(files)
    cnt = collections.Counter(r["ev"] for r in rows)
    matches = split_matches(rows)
    run = os.path.basename(files[-1]).replace("events_", "").replace(".jsonl", "")
    out = [f"# 우물 대기 이벤트 판독 — run {run}", "", f"입력: {', '.join(files)}", f"이벤트 수: {dict(cnt)} · 경기 {len(matches)}개", "",
           "- force = 우물 대기 블록이 Recall 을 강제한 틱(조건 a~j 모두 참). short = 다음 아이템까지 모자란 골드. wait/travel 단위 = 틱.",
           "- 서브플랜 이름: 0 LineDefense · 1 LineSafe · 2 LineWait · 3 Recall · 4 Jungle · 5 Battle (RE 이름표). data-variant = 값을 품은 변형.", ""]
    for k, m in enumerate(matches, 1):
        out += [f"## 경기 {k}", ""] + report(m, a.first_sec) + [""]
    os.makedirs(OUT, exist_ok=True)
    dst = os.path.join(OUT, f"{run}_report.md")
    open(dst, "w", encoding="utf-8").write("\n".join(out) + "\n")
    print("\n".join(out[:6]))
    print("→", dst)


if __name__ == "__main__":
    main()
