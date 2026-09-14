#!/usr/bin/env python3
# -*- coding: utf-8 -*-
u"""subtree_rank.py — 상위 함수의 exe 호출 서브트리를 뽑아 「아래에서 닫는」 작업 순서표를 만든다 (2026-09-13)

원리(원장 §3-B 결정): 선별은 top-down(이 표), 작업은 bottom-up(잎부터). 점수 = 호출자 수 / 크기 —
재사용이 높고 작은 함수가 먼저. 이미 명세/ev1 이 있는 것과 제외 사유(데스매치·튜토리얼 전용·game_core 경계·
CRT)는 따로 표기한다. 도달 가능성(사장 서브트리) 봉인은 IR 이 필요하므로 **여기서 하지 않는다**(→ `irann.py`).

입력: `cg_exe.json`(exe 콜그래프) · `REPORT\\tfm2_ai_adjust\\AI함수지도.html`(이름·모듈·크기) · `_spec\\specs20_v3.json`(명세 유무) ·
      `REPORT\\tfm2_judge_verify\\00_상태원장.md`(ev1)
사용: python -X utf8 MIG\\subtree_rank.py [루트RVA=0xe4c5c0[,RVA2,…]] [--depth 5] [--minus 0xe4c5c0] [--tag action]
      (09-14: 루트 여러 개 = 합집합 · `--minus` = 그 루트의 서브트리를 뺀다(이미 닫은 계층) · `--tag` = 출력 파일명)
출력: `_next\\subtree_<rva|tag>.md`(표) + `.json`(기계용)
"""
import io, json, os, re, sys, time

sys.stdout.reconfigure(encoding="utf-8", errors="replace")
HERE = os.path.dirname(os.path.abspath(__file__))
MAP = r"C:\Users\jungs\Desktop\claude\tfm2\mods_report\tfm2_ai_adjust\AI함수지도.html"
LEDGER = r"C:\Users\jungs\Desktop\claude\tfm2\mods_report\tfm2_judge_verify\00_상태원장.md"


def _newest(base):
    u"""base 정본과 worktree 사본 중 **더 새로운** 것(09-14: 워크트리에서 자라는 동안 base 가 stale)."""
    import glob
    cands = [base] + glob.glob(base.replace(u"\\mods_report\\", u"\\.claude\\worktrees\\*\\mods_report\\"))
    cands = [c for c in cands if os.path.exists(c)]
    return max(cands, key=os.path.getmtime) if cands else base
EXCL_MOD = {"death_battle": u"데스매치 전용(MOBA 미사용 · #17 과 같은 판정)", "deathmatch": u"데스매치 전용"}
CORE_HINT = (0x1200000, 0x1900000)   # game_core/engine 대역(대략) — 경계 함수는 명세 대상 아님(계약만)


def main():
    av = sys.argv[1:]
    skipv = set()
    for f in ("--depth", "--minus", "--tag"):
        if f in av: skipv.add(av.index(f) + 1)
    a = [x for i, x in enumerate(av) if not x.startswith("--") and i not in skipv]
    roots = [int(x, 16) for x in u",".join(a).split(u",") if x] if a else [0xe4c5c0]
    root = roots[0]
    maxd = int(sys.argv[sys.argv.index("--depth") + 1]) if "--depth" in sys.argv else 5
    minus = int(sys.argv[sys.argv.index("--minus") + 1], 16) if "--minus" in sys.argv else None
    tagname = sys.argv[sys.argv.index("--tag") + 1] if "--tag" in sys.argv else None
    cg = {int(k): set(v) for k, v in json.load(io.open(os.path.join(HERE, "cg_exe.json"), encoding="utf-8")).items()}
    rev = {}
    for x, cs in cg.items():
        for c in cs:
            rev.setdefault(c, set()).add(x)
    h = io.open(_newest(MAP), encoding="utf-8").read()
    data = json.loads(re.search(r'<script id="fndata" type="application/json">(.*?)</script>', h, re.S).group(1))
    by = {int(e["a"], 16): e for e in data if not e["a"].startswith("spec-")}
    v3 = json.load(io.open(os.path.join(HERE, "_spec", "specs20_v3.json"), encoding="utf-8"))["specs"]
    spec_addr = {int(s["exe"]["addr"], 16): s["i"] for s in v3 if s.get("exe") and s["exe"].get("addr")}
    ev1 = {}
    for l in io.open(_newest(LEDGER), encoding="utf-8"):
        m = re.match(r"^\|\s*(\d{2,3})\s*\|", l)
        if m:
            c = [x.strip() for x in l.strip().strip("|").split("|")]
            ev1[int(m.group(1))] = re.sub(r"[*`]", "", c[5]) if len(c) > 5 else ""
    extra = {0xdea4a0: 21, 0xdeaa70: 22, 0xec9bf0: 23, 0xd666c0: 24}
    for k in extra.values():
        ev1.setdefault(k, u"DIFF 0(명세 밖·회계 보류)")
    # 크기 폴백: 지도에 크기가 없으면(0/-1) exe .pdata RUNTIME_FUNCTION 으로
    try:
        import pefile
        pe = pefile.PE(r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe", fast_load=True)
        pe.parse_data_directories(directories=[pefile.DIRECTORY_ENTRY["IMAGE_DIRECTORY_ENTRY_EXCEPTION"]])
        pdata = {f.struct.BeginAddress: f.struct.EndAddress - f.struct.BeginAddress for f in pe.DIRECTORY_ENTRY_EXCEPTION}
    except Exception:
        pdata = {}
    # BFS (다중 루트 합집합 · 루트끼리는 깊이 0)
    def bfs(rs, md):
        dp = {r: 0 for r in rs}; od = list(rs); k = 0
        while k < len(od):
            n = od[k]; k += 1
            if dp[n] >= md:
                continue
            for c in sorted(cg.get(n, ())):
                if c not in dp and c in by:
                    dp[c] = dp[n] + 1; od.append(c)
        return dp, od
    depth, order = bfs(roots, maxd)
    if minus is not None:
        mdp, _ = bfs([minus], 99)
        cut = set(mdp) - set(roots)
        order = [n for n in order if n not in cut]
        print(u"--minus 0x%x: %d 노드 제외(이미 닫은 서브트리)" % (minus, len(cut)))
    nroot = len(roots)
    core_edges = sorted({c for n in order for c in cg.get(n, ()) if c not in by and CORE_HINT[0] <= c < CORE_HINT[1]})
    rows = []
    for c in order[nroot:]:
        e = by[c]; b = e.get("b") or 0
        if b <= 0:
            b = pdata.get(c, 0)
        callers = len(rev.get(c, ())); callees_in = sum(1 for x in cg.get(c, ()) if x in by)
        idx = spec_addr.get(c, extra.get(c))
        status = (u"#%02d %s" % (idx, ev1.get(idx, "")) if idx is not None else u"")
        why = EXCL_MOD.get(e["m"], u"")
        if CORE_HINT[0] <= c < CORE_HINT[1]:
            why = u"game_core 경계(명세 대상 아님 · 계약만 확정)"
        kind = u"잎" if callees_in == 0 else (u"중간" if b < 8000 else u"거대")
        score = callers / max(b, 200) * 1000
        rows.append(dict(a="%x" % c, name=e.get("n") or u"?", mod=e["m"], bytes=b, callers=callers, callees=callees_in,
                         depth=depth[c], kind=kind, score=round(score, 2), status=status, excl=why, flags=e.get("f") or ""))
    # 정렬: 제외 → 완료 → 잎(점수↓) → 중간(점수↓) → 거대(크기↑)
    def key(r):
        if r["excl"]: return (4, 0)
        if r["status"]: return (3, 0)
        k = {u"잎": 0, u"중간": 1, u"거대": 2}[r["kind"]]
        return (k, -r["score"] if k < 2 else r["bytes"])
    rows.sort(key=key)
    tag = tagname or ("%x" % root)
    jp = os.path.join(HERE, "_next", "subtree_%s.json" % tag)
    io.open(jp, "w", encoding="utf-8", newline="\n").write(json.dumps(dict(root=tag, generated=time.strftime("%Y-%m-%d"), rows=rows), ensure_ascii=False, indent=1))
    rootdesc = u" + ".join(u"`%s::%s`(0x%x)" % (by[r]["m"], by[r]["n"], r) for r in roots) if len(roots) > 1 else u"`%s::%s`(0x%s)" % (by[root]["m"], by[root]["n"], tag)
    L = [u"# %s 호출 서브트리 선별표 — 작업 순서 = 아래에서 위로 (생성 %s · 게임 0.5.8%s)" % (rootdesc, time.strftime("%Y-%m-%d"), (u" · `--minus 0x%x` 서브트리 제외" % minus) if minus is not None else u""),
         u"", u"> 생성물(`MIG\\subtree_rank.py 0x%s`) · 손편집 금지. 점수 = 호출자수/크기×1000(재사용↑·크기↓ 우선). 종류: 잎=지도 내 콜리 0 · 중간=<8KB · 거대=≥8KB. "
              u"**⬜도달 가능성(사장 서브트리) 봉인은 미실시** — 착수 시 `irann.py`(교훈 68)로 NA 를 먼저 찍고 이 표에서 뺄 것. 이름 `?` = 지도 미명명(dllmatch/percolate 로 이름부터)." % tag,
         u"> 합계: 서브트리 %d · 잎 %d · 중간 %d · 거대 %d · 이미 명세/ev1 %d · 제외(데스매치) %d · game_core 경계 함수 %d(명세 대상 아님 · 계약만)" % (
             len(rows), sum(1 for r in rows if r["kind"] == u"잎" and not r["excl"]), sum(1 for r in rows if r["kind"] == u"중간" and not r["excl"]),
             sum(1 for r in rows if r["kind"] == u"거대" and not r["excl"]), sum(1 for r in rows if r["status"]), sum(1 for r in rows if r["excl"] and u"데스" in r["excl"]), len(core_edges) + sum(1 for r in rows if u"game_core" in r["excl"])),
         u"", u"| 순위 | RVA | 함수 | 모듈 | 크기 | 호출자 | 콜리 | 깊이 | 종류 | 점수 | 상태/제외 |", u"|---|---|---|---|---|---|---|---|---|---|---|"]
    for n, r in enumerate(rows, 1):
        L.append(u"| %d | `0x%s` | `%s` | %s | %s | %d | %d | %d | %s | %s | %s |" % (n, r["a"], r["name"], r["mod"], (u"%dB" % r["bytes"]) if r["bytes"] else u"?", r["callers"], r["callees"], r["depth"], r["kind"], r["score"], (r["excl"] or r["status"] or u"⬜")))
    L += [u"", u"## game_core/engine 경계(지도 밖 · 서브트리가 부르는 것 · 대략 대역 0x1200000~0x1900000)", u"",
          u"`" + u"` `".join("0x%x" % c for c in core_edges[:60]) + u"`" + (u" … +%d" % (len(core_edges) - 60) if len(core_edges) > 60 else u""),
          u"", u"이들은 명세 대상이 아니라 **계약(시그니처·반환 의미)만** 확정한다(tcx `--deep`/오라클). 예: `estimate_damage_to`(0x12857f0 · 호출자 91)."]
    mp = os.path.join(HERE, "_next", "subtree_%s.md" % tag)
    io.open(mp, "w", encoding="utf-8", newline="\n").write(u"\n".join(L))
    print(u"OK %s (%d rows) · %s" % (mp, len(rows), jp))
    top = [r for r in rows if not r["excl"] and not r["status"]][:20]
    for r in top:
        print(u"  %-8s %-45s %-18s %6s c=%-3d %s %s" % (r["a"], r["name"], r["mod"], r["bytes"], r["callers"], r["kind"], r["status"]))


if __name__ == "__main__":
    main()
