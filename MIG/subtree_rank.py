#!/usr/bin/env python3
# -*- coding: utf-8 -*-
u"""subtree_rank.py — 상위 함수의 exe 호출 서브트리를 뽑아 「아래에서 닫는」 작업 순서표를 만든다 (2026-09-13)

원리(원장 §3-B 결정): 선별은 top-down(이 표), 작업은 bottom-up(잎부터). 점수 = 호출자 수 / 크기 —
재사용이 높고 작은 함수가 먼저. 이미 명세/ev1 이 있는 것과 제외 사유(데스매치·튜토리얼 전용·game_core 경계·
CRT)는 따로 표기한다. 도달 가능성(사장 서브트리) 봉인은 IR 이 필요하므로 **여기서 하지 않는다**(→ `irann.py`).

입력: `cg_exe.json`(exe 콜그래프) · `REPORT\\tfm2_ai_adjust\\AI함수지도.html`(이름·모듈·크기) · `_spec\\specs20_v3.json`(명세 유무) ·
      `REPORT\\tfm2_judge_verify\\00_상태원장.md`(ev1)
사용: python -X utf8 MIG\\subtree_rank.py [루트RVA=0xe4c5c0] [--depth 5]
출력: `_next\\subtree_<rva>.md`(표) + `_next\\subtree_<rva>.json`(기계용)
"""
import io, json, os, re, sys, time

sys.stdout.reconfigure(encoding="utf-8", errors="replace")
HERE = os.path.dirname(os.path.abspath(__file__))
MAP = r"C:\Users\jungs\Desktop\claude\tfm2\mods_report\tfm2_ai_adjust\AI함수지도.html"
LEDGER = r"C:\Users\jungs\Desktop\claude\tfm2\mods_report\tfm2_judge_verify\00_상태원장.md"
EXCL_MOD = {"death_battle": u"데스매치 전용(MOBA 미사용 · #17 과 같은 판정)", "deathmatch": u"데스매치 전용"}
CORE_HINT = (0x1200000, 0x1900000)   # game_core/engine 대역(대략) — 경계 함수는 명세 대상 아님(계약만)


def main():
    a = [x for x in sys.argv[1:] if not x.startswith("--")]
    root = int(a[0], 16) if a else 0xe4c5c0
    maxd = int(sys.argv[sys.argv.index("--depth") + 1]) if "--depth" in sys.argv else 5
    cg = {int(k): set(v) for k, v in json.load(io.open(os.path.join(HERE, "cg_exe.json"), encoding="utf-8")).items()}
    rev = {}
    for x, cs in cg.items():
        for c in cs:
            rev.setdefault(c, set()).add(x)
    h = io.open(MAP, encoding="utf-8").read()
    data = json.loads(re.search(r'<script id="fndata" type="application/json">(.*?)</script>', h, re.S).group(1))
    by = {int(e["a"], 16): e for e in data if not e["a"].startswith("spec-")}
    v3 = json.load(io.open(os.path.join(HERE, "_spec", "specs20_v3.json"), encoding="utf-8"))["specs"]
    spec_addr = {int(s["exe"]["addr"], 16): s["i"] for s in v3 if s.get("exe") and s["exe"].get("addr")}
    ev1 = {}
    for l in io.open(LEDGER, encoding="utf-8"):
        m = re.match(r"^\|\s*(\d{2})\s*\|", l)
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
    # BFS
    depth = {root: 0}; order = [root]; i = 0
    while i < len(order):
        n = order[i]; i += 1
        if depth[n] >= maxd:
            continue
        for c in sorted(cg.get(n, ())):
            if c not in depth and c in by:
                depth[c] = depth[n] + 1; order.append(c)
    core_edges = sorted({c for n in order for c in cg.get(n, ()) if c not in by and CORE_HINT[0] <= c < CORE_HINT[1]})
    rows = []
    for c in order[1:]:
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
    tag = "%x" % root
    jp = os.path.join(HERE, "_next", "subtree_%s.json" % tag)
    io.open(jp, "w", encoding="utf-8", newline="\n").write(json.dumps(dict(root=tag, generated=time.strftime("%Y-%m-%d"), rows=rows), ensure_ascii=False, indent=1))
    L = [u"# `%s::%s`(0x%s) 호출 서브트리 선별표 — 작업 순서 = 아래에서 위로 (생성 %s · 게임 0.5.8)" % (by[root]["m"], by[root]["n"], tag, time.strftime("%Y-%m-%d")),
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
