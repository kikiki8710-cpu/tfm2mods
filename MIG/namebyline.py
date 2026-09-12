#!/usr/bin/env python3
# -*- coding: utf-8 -*-
u"""namebyline.py — 지도 미명명(`?`) exe 함수를 **패닉 Location 줄번호 지문**으로 IR define 에 잇는다 (2026-09-13)

원리: 지도(fndata)의 `l`(exe 함수 안 panic Location 의 소스 줄)과 `m`(파일) 은 exe 쪽 지문이다. IR 쪽은 define 본문의
      `!dbg` → DILocation(line, file) 집합(인라인 체인 포함). **l 의 줄 전부를 그 파일에서 담고 있는 define** 이 후보,
      호출자의 IR 파일(같은 CGU)에서 먼저 찾고 없으면 전 파일. 크기(IR 줄 수 ↔ exe 바이트)로 순위.
      `namebycaller.py`(호출자 차집합)와 짝 — 이쪽은 줄번호가 있는 것만 잡지만 정밀하다.

사용: python -X utf8 MIG\\namebyline.py [subtree json] [--all-files]
출력: RVA · 후보 심볼(짧은 이름) · 근거(줄 매칭 수/크기) · `_next\\reach\\names_byline.json`
"""
import io, os, re, sys, json, collections

sys.stdout.reconfigure(encoding="utf-8", errors="replace")
HERE = os.path.dirname(os.path.abspath(__file__))
CORP = r"C:\tfm2mods\_gaibc"
OUT = os.path.join(HERE, "_next", "reach")
sys.path.insert(0, HERE)
import reach_tree as RT

LOC = re.compile(rb'^!(\d+) = (?:distinct )?!DILocation\(line: (\d+),(?: column: \d+,)? scope: !(\d+)(?:, inlinedAt: !(\d+))?\)')
SUB = re.compile(rb'^!(\d+) = distinct !DISubprogram\(name: "[^"]*".*?file: !(\d+)')
BLK = re.compile(rb'^!(\d+) = !DILexicalBlock(?:File)?\(scope: !(\d+)(?:, file: !(\d+))?')
FIL = re.compile(rb'^!(\d+) = !DIFile\(filename: "([^"]*)"')
DBG = re.compile(rb'!dbg !(\d+)')
_cache = {}


def load_file(fn):
    u"""파일 하나: define 별 {(파일basename, line)} 집합 (인라인 체인 포함) + 메타."""
    if fn in _cache: return _cache[fn]
    path = os.path.join(CORP, fn)
    loc, sub, blk, fil = {}, {}, {}, {}
    defs = []; cur = None
    with io.open(path, "rb") as f:
        for i, raw in enumerate(f, 1):
            if raw.startswith(b"!"):
                m = LOC.match(raw)
                if m: loc[int(m.group(1))] = (int(m.group(2)), int(m.group(3)), int(m.group(4)) if m.group(4) else None); continue
                m = SUB.match(raw)
                if m: sub[int(m.group(1))] = int(m.group(2)); continue
                m = BLK.match(raw)
                if m: blk[int(m.group(1))] = (int(m.group(2)), int(m.group(3)) if m.group(3) else None); continue
                m = FIL.match(raw)
                if m: fil[int(m.group(1))] = os.path.basename(m.group(2).decode("utf-8", "replace").replace("\\", "/")); continue
                continue
            if raw.startswith(b"define"):
                m = RT.DEFRE.match(raw); cur = [m.group(1).decode() if m else "?", i, set(), 0]; defs.append(cur); continue
            if cur is None: continue
            cur[3] += 1
            for d in DBG.findall(raw): cur[2].add(int(d))
            if raw.startswith(b"}"): cur = None
    def scope_file(s, seen=None):
        seen = seen or set()
        while s is not None and s not in seen:
            seen.add(s)
            if s in sub: return fil.get(sub[s])
            if s in blk:
                if blk[s][1] is not None: return fil.get(blk[s][1])
                s = blk[s][0]; continue
            return None
        return None
    res = {}
    for name, ln, dbgs, n in defs:
        pairs = set()
        for d in dbgs:
            x = d; depth = 0
            while x is not None and depth < 8:
                l = loc.get(x)
                if not l: break
                pairs.add((scope_file(l[1]), l[0])); x = l[2]; depth += 1
        res[name] = (pairs, n, ln)
    _cache[fn] = res
    return res


def main():
    a = [x for x in sys.argv[1:] if x.endswith(".json")]
    sub_p = a[0] if a else os.path.join(HERE, "_next", "subtree_e4c5c0.json")
    sub = json.load(io.open(sub_p, encoding="utf-8"))
    rows = {int(r["a"], 16): r for r in sub["rows"]}
    h = io.open(r"C:\Users\jungs\Desktop\claude\tfm2\mods_report\tfm2_ai_adjust\AI함수지도.html", encoding="utf-8").read()
    fnd = {int(e["a"], 16): e for e in json.loads(re.search(r'<script id="fndata" type="application/json">(.*?)</script>', h, re.S).group(1)) if not e["a"].startswith("spec-")}
    idx = RT.defidx()
    tree = json.load(io.open(os.path.join(OUT, "tree_e4c5c0.json"), encoding="utf-8"))
    r2s = {int(v["rva"], 16): v["sym"] for v in tree["verdict"] if v.get("sym")}
    r2s[int(tree["root"], 16)] = [s for s in idx if s.endswith("17LegacyPlanHandler6update")][0]
    targets = [r for r, row in rows.items() if row["name"] == "?" or (r not in r2s and row["kind"] != "거대")]
    if "--only" in sys.argv:
        only = {int(x, 16) for x in sys.argv[sys.argv.index("--only") + 1].split(",")}
        targets = [r for r in targets if r in only]
    out = {}
    for r in targets:
        e = fnd.get(r); row = rows[r]
        if not e or not e.get("l"): print(u"0x%x %s: 줄 지문 없음" % (r, row["name"])); continue
        lines = set(e["l"]); mod = (e["m"] or "").split("/")[-1]; fname = mod + ".rs"
        files = []
        for c in e.get("c", []):
            cs = r2s.get(int(c, 16))
            if cs and cs in idx: files.append(idx[cs][0])
        if "--all-files" in sys.argv or not files:
            files = sorted({v[0] for v in idx.values()})
        cands = []
        for fn in dict.fromkeys(files):
            res = load_file(fn)
            for name, (pairs, n, ln) in res.items():
                if "7game_ai" not in name: continue
                got = {l for (f, l) in pairs if f == fname}
                if lines <= got:
                    cands.append((name, fn, n, len(got), ln))
        cands.sort(key=lambda x: (abs(x[2] * 2.2 - row["bytes"]), x[2]))   # IR 줄수 ≈ exe 바이트/2.2 (경험칙) 로 근접순
        print(u"\n0x%x %s [%s] %dB l=%s 호출자=%s" % (r, row["name"], e["m"], row["bytes"], sorted(lines), e.get("c")))
        for name, fn, n, g, ln in cands[:5]:
            print(u"    %-5s %5d줄 %s:%d  %s" % ("", n, fn, ln, RT.short(name)[:150]))
        out["%x" % r] = [[name, fn, n, ln] for name, fn, n, g, ln in cands[:5]]
    ofn = "names_byline_all.json" if "--all-files" in sys.argv else "names_byline.json"
    json.dump(out, io.open(os.path.join(OUT, ofn), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    print(u"-> names_byline.json")
    return 0


if __name__ == "__main__":
    sys.exit(main())
