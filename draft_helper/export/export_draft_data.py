# -*- coding: utf-8 -*-
"""
export_draft_data.py — TFM2 밴픽 도우미용 컴팩트 데이터 추출기.

입력(전부 TFM2.gg save_probe 디버그 덤프 / 게임 i18n):
  - champion_patch_statistics.debug.txt : 패치·챔프·포지션별 승/매치/밴
  - match_replays.debug.txt (~255MB)    : 경기별 팀구성+밴+승패 (시너지/카운터 원천)
  - champion.i18n                        : 한글 이름 (ko.description.<id>.name)
  - candidate_map.tsv                    : candidate_index <-> champion_id (60 base)

출력: draft_data.json (수 MB) — egui 앱이 소비.

버전 churn 격리 지점: 세이브 덤프 경로만 패치마다 갱신하면 됨(앱은 무관).
"""
import json, io, re, sys, os, time
from collections import defaultdict

# ---- 경로 (필요시 인자/환경으로 override) ----
GG = r"C:\Users\dev\Desktop\claude\tfm2\TFM2.gg-latest\TFM2_Meta_Dashboard_v0.3.3 (팀파매.gg)\resources\app\tfm2_meta_dashboard"
SNAP = os.path.join(GG, "data", "save_probe_snapshot")
DEFAULTS = {
    "champ_stats": os.path.join(SNAP, "champion_patch_statistics.debug.txt"),
    "match_replays": os.path.join(SNAP, "match_replays.debug.txt"),
    "i18n": r"C:\Users\dev\Desktop\claude\tfm2\tfm2_0.4.12\bakcup\bundle_unpacked_full\text\champion.i18n",
    "candidate_map": os.path.join(GG, "data", "policy_exports", "candidate_map.tsv"),
    "out": r"C:\tfm2mods\draft_helper\draft_data.json",
}

POSITIONS = ["Top", "Jungle", "Mid", "Bottom", "Support"]


def log(*a):
    print(*a, flush=True)


def load_candidate_map(path):
    rows = []  # (index, id)
    with io.open(path, encoding="utf-8") as f:
        for line in f:
            line = line.rstrip("\n")
            if not line or line.startswith("#"):
                continue
            parts = line.split("\t")
            if len(parts) >= 2 and parts[0].isdigit():
                rows.append((int(parts[0]), parts[1]))
    rows.sort()
    return rows


def load_names(path):
    with io.open(path, encoding="utf-8") as f:
        d = json.load(f)
    desc = d.get("ko", {}).get("description", {})
    return {cid: v.get("name", cid) for cid, v in desc.items()}


def parse_champ_stats(path, valid_ids):
    """패치 전체를 합산: 챔프별 총 밴 / 포지션별 (wins, matches) / 패치별 total_match 합."""
    re_patch = re.compile(r'"([^"]+)": ChampionPatchStatistics \{')
    re_total = re.compile(r'total_match: (\d+),')
    re_champ = re.compile(r'"([^"]+)": ChampionSeasonStatistics \{')
    re_bans = re.compile(r'bans: (\d+),')
    re_pos = re.compile(r'(Top|Jungle|Mid|Bottom|Support): ChampionStatistics \{')
    re_wins = re.compile(r'wins: (\d+),')
    re_matches = re.compile(r'matches: (\d+),')

    total_matches_all = 0
    bans = defaultdict(int)
    bypos = defaultdict(lambda: defaultdict(lambda: [0, 0]))  # champ -> pos -> [wins, matches]

    patch = None
    champ = None
    pos = None
    cur_wins = None

    with io.open(path, encoding="utf-8") as f:
        for line in f:
            s = line.strip()
            m = re_patch.match(s)
            if m:
                patch = m.group(1); champ = None; pos = None
                continue
            if champ is None and patch is not None:
                m = re_total.match(s)
                if m:
                    total_matches_all += int(m.group(1))
                    continue
            m = re_champ.match(s)
            if m:
                champ = m.group(1); pos = None; cur_wins = None
                continue
            if champ is not None and pos is None:
                m = re_bans.match(s)
                if m:
                    bans[champ] += int(m.group(1))
                    continue
            m = re_pos.match(s)
            if m:
                pos = m.group(1); cur_wins = None
                continue
            if pos is not None:
                m = re_wins.match(s)
                if m:
                    cur_wins = int(m.group(1))
                    continue
                m = re_matches.match(s)
                if m and cur_wins is not None:
                    mt = int(m.group(1))
                    cell = bypos[champ][pos]
                    cell[0] += cur_wins
                    cell[1] += mt
                    cur_wins = None
                    pos = None  # block consumed
                    continue

    stats = {}
    for cid in valid_ids:
        bp = {}
        tot_w = tot_m = 0
        for p in POSITIONS:
            w, mt = bypos.get(cid, {}).get(p, [0, 0])
            if mt > 0:
                bp[p] = {"wins": w, "games": mt}
                tot_w += w; tot_m += mt
        stats[cid] = {
            "games": tot_m,
            "wins": tot_w,
            "winrate": (tot_w / tot_m) if tot_m else 0.0,
            "picks": tot_m,  # appearances
            "pickrate": (tot_m / total_matches_all) if total_matches_all else 0.0,
            "bans": bans.get(cid, 0),
            "banrate": (bans.get(cid, 0) / total_matches_all) if total_matches_all else 0.0,
            "by_position": bp,
        }
    return stats, total_matches_all


def parse_relations(path, valid_ids):
    """match_replays 에서 시너지(같은팀 페어)/카운터(상대 매치업) 집계."""
    re_win = re.compile(r'blue_team_win: (true|false)')
    re_champ = re.compile(r'champion: "([^"]+)"')

    syn = defaultdict(lambda: [0, 0])   # "a|b"(sorted) -> [games, wins(같은팀 동반 승)]
    cnt = defaultdict(lambda: [0, 0])   # "a>b" -> [games, a가 이긴 수]

    log("  match_replays 로딩(255MB)...")
    t0 = time.time()
    with io.open(path, encoding="utf-8", errors="replace") as f:
        content = f.read()
    log("    read %.1fs, split..." % (time.time() - t0))
    chunks = content.split("MatchReplayData {")
    del content
    log("    %d 경기 블록" % (len(chunks) - 1))

    n = 0
    for ch in chunks[1:]:
        bw = re_win.search(ch)
        if not bw:
            continue
        blue_won = bw.group(1) == "true"
        bi = ch.find("blue_team: [")
        ri = ch.find("red_team: [")
        if bi < 0 or ri < 0 or ri < bi:
            continue
        wi = bw.start()
        blue = [c for c in re_champ.findall(ch[bi:ri]) if c in valid_ids]
        red = [c for c in re_champ.findall(ch[ri:wi]) if c in valid_ids]
        if not blue or not red:
            continue
        # 시너지 (팀 내 페어)
        for team, won in ((blue, blue_won), (red, not blue_won)):
            uniq = list(dict.fromkeys(team))
            for i in range(len(uniq)):
                for j in range(i + 1, len(uniq)):
                    a, b = sorted((uniq[i], uniq[j]))
                    cell = syn[a + "|" + b]
                    cell[0] += 1; cell[1] += 1 if won else 0
        # 카운터 (상대 매치업, 방향)
        for a in set(blue):
            for b in set(red):
                ca = cnt[a + ">" + b]; ca[0] += 1; ca[1] += 1 if blue_won else 0
                cb = cnt[b + ">" + a]; cb[0] += 1; cb[1] += 1 if not blue_won else 0
        n += 1
        if n % 5000 == 0:
            log("    %d 경기 처리..." % n)

    log("    완료 %d 경기, %.1fs" % (n, time.time() - t0))
    # 최소 표본 필터 (노이즈 제거)
    syn_out = {k: {"games": v[0], "wins": v[1], "winrate": v[1] / v[0]}
               for k, v in syn.items() if v[0] >= 5}
    cnt_out = {k: {"games": v[0], "wins": v[1], "winrate": v[1] / v[0]}
               for k, v in cnt.items() if v[0] >= 5}
    return syn_out, cnt_out, n


def main():
    p = dict(DEFAULTS)
    out = p["out"]
    log("[1/4] candidate_map / names")
    cmap = load_candidate_map(p["candidate_map"])
    valid_ids = {cid for _, cid in cmap}
    names = load_names(p["i18n"])

    log("[2/4] champion_patch_statistics")
    stats, total_matches = parse_champ_stats(p["champ_stats"], valid_ids)
    log("    total_matches(패치합)=%d" % total_matches)

    log("[3/4] match_replays relations")
    syn, cnt, n_matches = parse_relations(p["match_replays"], valid_ids)
    log("    synergy pairs=%d, counter pairs=%d" % (len(syn), len(cnt)))

    log("[4/4] write %s" % out)
    champions = []
    for idx, cid in cmap:
        bp = stats[cid]["by_position"]
        positions = sorted(bp.keys(), key=lambda pp: -bp[pp]["games"])
        champions.append({
            "id": cid,
            "name": names.get(cid, cid),
            "candidate_index": idx,
            "positions": positions,
        })

    data = {
        "meta": {
            "source": "save_probe_snapshot",
            "total_matches": total_matches,
            "relation_matches": n_matches,
            "champion_count": len(champions),
        },
        "champions": champions,
        "stats": stats,
        "synergy": syn,
        "counter": cnt,
    }
    os.makedirs(os.path.dirname(out), exist_ok=True)
    with io.open(out, "w", encoding="utf-8") as f:
        json.dump(data, f, ensure_ascii=False, separators=(",", ":"))
    sz = os.path.getsize(out)
    log("done. %s (%.2f MB)" % (out, sz / 1e6))


if __name__ == "__main__":
    main()
