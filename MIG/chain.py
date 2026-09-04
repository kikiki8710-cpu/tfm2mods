# -*- coding: utf-8 -*-
"""chain.py — 함수 진입 RVA 를 **버전 체인으로 추적**해 현행 주소를 확정한다.

왜 필요한가 (2026-09-02 `RVA_RETREAT` 사고):
    repin 의 12B 바이트 서명이 `push8` 같은 **공통 프롤로그**면 유일할 수가 없는데도
    "값이 함수 진입부다" 정도로 통과해 버린다. 그러면 **엉뚱한 함수를 후킹**하고,
    프롤로그 검증(push8)도 통과하므로 훅은 조용히 설치된 뒤 게임을 죽인다.
    ⟹ 신원은 **스켈레톤 지문**(명령 시퀀스 정규화 md5)으로 확인해야 하고,
       두 버전을 건너뛰면 본문이 바뀌어 안 맞으므로 **한 버전씩** 따라가야 한다.

판정
    UNIQUE      다음 버전에서 같은 skel 이 정확히 1곳 → 확정
    BY_SIZE     skel 다수지만 함수 크기로 유일
    HEAD        skel 0곳, skel_head(앞 24명령)로 유일 → 본문만 바뀐 같은 함수
    MULTI/NONE  수동 판정 필요 (콜그래프 투영 등)

사용
  python MIG\chain.py --rva 0xd2f180 --from 0.5.6 --to 0.5.8
  python MIG\chain.py --from-file <name=rva 목록> --from 0.5.6 --to 0.5.8
"""
import argparse, os, pickle, struct, sys

sys.stdout.reconfigure(encoding="utf-8", errors="replace")

ANA = os.path.join("C:", os.sep, "Users", "jungs", "Desktop", "claude", "tfm2")
MODS = os.path.join("C:", os.sep, "tfm2mods")
LIVE = os.path.join("C:", os.sep, "Program Files (x86)", "Steam", "steamapps",
                    "common", "Teamfight Manager2", "TeamfightManager2.exe")

CHAIN = ["0.5.2", "0.5.3", "0.5.4", "0.5.5", "0.5.6", "0.5.7", "0.5.8"]


def paths(v):
    pkl = os.path.join(MODS, "_fnidx_%s.pkl" % v.replace(".", ""))
    exe = LIVE if v == "0.5.8" else os.path.join(ANA, "tfm2_%s" % v, "TeamfightManager2.exe")
    return exe, pkl


class Idx:
    def __init__(s, v):
        exe, pkl = paths(v)
        s.ver = v
        P = pickle.load(open(pkl, "rb"))
        s.fn = {(int(k, 16) if isinstance(k, str) else k):
                dict(x, size=int(x["size"]))
                for k, x in P["idx"].items()}   # size 가 문자열
        s.by_skel = P["by_skel"]
        s.by_head = P.get("by_head", {})

    def get(s, rva):
        return s.fn.get(rva)


def step(a, b, rva):
    """a 의 rva 함수를 b 에서 찾는다."""
    f = a.get(rva)
    if f is None:
        return ("NOT_A_FN", None, "%s 에서 함수 시작이 아님" % a.ver)
    cands = b.by_skel.get(f["skel"], [])
    cands = [int(c, 16) if isinstance(c, str) else c for c in cands]
    if len(cands) == 1:
        return ("UNIQUE", cands[0], "skel 유일 (size %d→%d)" % (f["size"], b.fn[cands[0]]["size"]))
    if len(cands) > 1:
        same = [c for c in cands if b.fn[c]["size"] == f["size"]]
        if len(same) == 1:
            return ("BY_SIZE", same[0], "skel %d후보 → size 로 유일" % len(cands))
        return ("MULTI", cands[:5], "skel %d후보 (size 일치 %d)" % (len(cands), len(same)))
    hc = b.by_head.get(f.get("head"), [])
    hc = [int(c, 16) if isinstance(c, str) else c for c in hc]
    if len(hc) == 1:
        return ("HEAD", hc[0], "skel 0 → head 유일 (본문 변경, size %d→%d)"
                % (f["size"], b.fn[hc[0]]["size"]))
    if len(hc) > 1:
        same = [c for c in hc if abs(b.fn[c]["size"] - f["size"]) < max(64, f["size"] // 8)]
        if len(same) == 1:
            return ("HEAD_SIZE", same[0], "head %d후보 → 크기 근접 유일" % len(hc))
        return ("MULTI_HEAD", hc[:5], "head %d후보" % len(hc))
    return ("NONE", None, "skel/head 후보 0 (함수가 크게 바뀜)")


def track(name, rva, frm, to, idxs):
    i0, i1 = CHAIN.index(frm), CHAIN.index(to)
    cur, trail = rva, []
    for k in range(i0, i1):
        a, b = idxs[CHAIN[k]], idxs[CHAIN[k + 1]]
        st, nxt, note = step(a, b, cur)
        trail.append((CHAIN[k], CHAIN[k + 1], st, nxt, note))
        if not isinstance(nxt, int):
            return cur, trail, False
        cur = nxt
    return cur, trail, True


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--rva", nargs="*", default=[])
    ap.add_argument("--from-file")
    ap.add_argument("--from", dest="frm", default="0.5.6")
    ap.add_argument("--to", dest="to", default="0.5.8")
    ap.add_argument("-v", "--verbose", action="store_true")
    a = ap.parse_args()

    items = [(r, int(r, 16)) for r in a.rva]
    if a.from_file:
        for line in open(a.from_file, encoding="utf-8"):
            line = line.strip()
            if not line or line.startswith("#"):
                continue
            n, v = line.split("=", 1) if "=" in line else (line, line)
            items.append((n.strip(), int(v.strip(), 16)))

    need = CHAIN[CHAIN.index(a.frm):CHAIN.index(a.to) + 1]
    idxs = {}
    for v in need:
        _, pkl = paths(v)
        if not os.path.isfile(pkl):
            print("★ 인덱스 없음: %s (%s) — fnindex.py 로 먼저 생성" % (v, pkl))
            return 1
        idxs[v] = Idx(v)

    print("체인: %s" % " → ".join(need))
    print()
    print("%-22s %-12s %-12s %-10s %s" % ("NAME", "입력RVA", "→ %s" % a.to, "판정", "비고"))
    print("-" * 104)
    for name, rva in items:
        end, trail, ok = track(name, rva, a.frm, a.to, idxs)
        last = trail[-1] if trail else ("", "", "?", None, "")
        got = ("0x%x" % end) if ok else "-"
        same = ok and end == rva
        verdict = ("동일(OK)" if same else "★이동") if ok else "★실패"
        print("%-22s 0x%-10x %-12s %-10s %s" % (name, rva, got, verdict, last[4]))
        if a.verbose or not ok or not same:
            for f, t, st, nxt, note in trail:
                nx = ("0x%x" % nxt) if isinstance(nxt, int) else str(nxt)
                print("      %s→%s  %-11s %-12s %s" % (f, t, st, nx, note))
    return 0


if __name__ == "__main__":
    sys.exit(main())
