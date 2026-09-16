#!/usr/bin/env python3
# -*- coding: utf-8 -*-
u"""extname.py — 지문(패닉 Location) 없는 지도 밖 exe 함수의 정체를 「exe 콜리 집합 ↔ IR 콜리 집합」으로 좁힌다. (2026-09-16 신설)

원리(namebycaller 의 단순판): exe 함수 X 의 본문에서 `call/jmp rel32` 타깃을 전수 뽑아 지도 이름으로 바꾸면 「X 가 부르는 game_ai 함수 이름 집합」이 나온다.
      IR(`_gaibc\\m00~m15.ll`) 의 모든 `define` 에 대해 `call/invoke @sym` 콜리 집합을 인덱싱해 두고(캐시 `_next\\ir_callees.json`),
      X 의 콜리 이름을 **전부** 부르는 IR 함수를 찾는다. 1개면 정체 확정 후보, 여러 개면 IR 명령수 ↔ exe 바이트 비로 순위.
      ⚠exe 는 LTO 인라인이 많아 X 의 exe 콜리가 IR 콜리의 부분집합이 아닐 수 있다(인라인된 안쪽 함수의 콜리가 X 에 나타남) → 「⊇」 대신 「교집합 비율」로 순위.
사용: python -X utf8 MIG\\extname.py [--all]   (기본 = 지도 ext 노드 중 이름 없는 것 · --all = ext 전부)
출력: `_next\\extname.md` + `_next\\extname.json` {rva: [[sym, score, ir_ins, note], ...]}
"""
import io, json, os, re, struct, sys
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
HERE = os.path.dirname(os.path.abspath(__file__)); sys.path.insert(0, HERE)
import fnmap_ret, retedges, fnmap_ext
IRD = r"C:\tfm2mods\_gaibc"
CACHE = os.path.join(HERE, "_next", "ir_callees.json")

def ir_index():
    if os.path.exists(CACHE):
        return json.load(io.open(CACHE, encoding="utf-8"))
    idx = {}
    for k in range(16):
        p = os.path.join(IRD, "m%02d.ll" % k)
        if not os.path.exists(p): continue
        cur = None; ins = 0; calls = set()
        for ln in io.open(p, encoding="utf-8", errors="replace"):
            if ln.startswith("define "):
                m = re.search(r"@([^\s(]+)\(", ln); cur = m.group(1).strip('"') if m else None; ins = 0; calls = set(); continue
            if cur is None: continue
            if ln.startswith("}"):
                idx[cur] = {"f": "m%02d" % k, "ins": ins, "c": sorted(calls)}; cur = None; continue
            s = ln.strip()
            if not s or s.startswith(";") or s.endswith(":"): continue
            ins += 1
            m = re.search(r"(?:call|invoke)\b[^@]*@([^\s(]+)\(", s)
            if m: calls.add(m.group(1).strip('"'))
    json.dump(idx, io.open(CACHE, "w", encoding="utf-8"))
    return idx

def idents(sym):
    u"""v0 망글에서 식별자 목록"""
    ids, i = [], 0
    while i < len(sym):
        mb = re.match(r"[sB][0-9A-Za-z]*_(?=\d)", sym[i:])   # v0 backref `B<base62>_` · 판별자 `s<base62>_` — 뒤에 <len><ident> 가 온다
        if mb: i += len(mb.group(0)); continue
        m = re.match(r"(\d+)", sym[i:])
        if m:
            n = int(m.group(1)); s = i + len(m.group(1)); ident = sym[s:s + n]
            if n and re.match(r"^[A-Za-z_][A-Za-z0-9_]*$", ident): ids.append(ident.lstrip("_")); i = s + n; continue
        i += 1
    return ids

def exe_callees(d, secs_text, a, ends):
    va, buf = secs_text
    start = int(a, 16); end = ends.get(start, start + 16)
    out = []
    lo, hi = start - va, end - va
    seg = buf[max(0, lo):max(0, hi)]
    for m in re.finditer(b"[\xe8\xe9]", seg):
        i = m.start()
        if i + 5 > len(seg): break
        rel = struct.unpack_from("<i", seg, i + 1)[0]; tgt = start + i + 5 + rel
        if 0x1000 <= tgt < 0x4000000 and tgt != start: out.append(tgt)
    return out

def main():
    av = sys.argv[1:]
    H = fnmap_ret.DEF_MAP
    h = io.open(H, encoding="utf-8").read()
    dmap = json.loads(re.search(r'<script id="fndata" type="application/json">(.*?)</script>', h, re.S).group(1))
    BY = {n["a"]: n for n in dmap}
    starts, ends = retedges.pdata_starts()
    # .text
    d = open(retedges.EXE, "rb").read()
    pe = struct.unpack_from("<I", d, 0x3c)[0]; nsec = struct.unpack_from("<H", d, pe + 6)[0]; oh = struct.unpack_from("<H", d, pe + 20)[0]
    for i in range(nsec):
        o = pe + 24 + oh + i * 40
        if d[o:o + 8].rstrip(b"\0") == b".text":
            vsz, va, rsz, ra = struct.unpack_from("<IIII", d, o + 8); break
    text = (va, d[ra:ra + rsz])
    idx = ir_index()
    # IR 심볼 → 마지막 식별자(함수 이름) 역색인
    last_id = {}
    for sym in idx:
        ids = idents(sym); last_id[sym] = ids[-1] if ids else sym
    targets = [n for n in dmap if n.get("L") == "ext" and ("--all" in av or not n.get("n") or (n.get("v") == "cg"))]
    res = {}; L = [u"# 지도 밖 함수 정체 추정(extname · exe 콜리 ↔ IR 콜리)", u"", u"| rva | 현재 이름 | exe 크기 | exe 콜리(이름) | IR 후보(교집합/필요 · IR 명령수) |", u"|---|---|---|---|---|"]
    for n in targets:
        a = n["a"]; ce = exe_callees(d, text, a, ends)
        names = []
        for t in ce:
            ta = "%x" % t
            if ta in BY and BY[ta].get("n"): names.append(BY[ta]["n"])
        need = set()
        for nm in names:
            nm2 = re.sub(r"\(.*$", "", nm).split("::")[-1].strip()
            if re.match(r"^[A-Za-z_][A-Za-z0-9_]*$", nm2): need.add(nm2)
        cands = []
        if need:
            for sym, info in idx.items():
                got = set()
                for c in info["c"]:
                    li = last_id.get(c) or (idents(c)[-1] if idents(c) else c)
                    if li in need: got.add(li)
                if not got: continue
                score = len(got) / len(need)
                if score < 0.5: continue
                size_ratio = min(info["ins"], max(1, n["b"] / 4.0)) / max(info["ins"], max(1, n["b"] / 4.0), 1)
                cands.append((score, size_ratio, sym, info["ins"], sorted(got)))
            cands.sort(key=lambda x: (-x[0], -x[1]))
        res[a] = [[c[2], round(c[0], 2), c[3], u"%d/%d" % (len(c[4]), len(need))] for c in cands[:5]]
        cs = u" · ".join(u"`%s` %s %s(%d)" % (u"::".join([x for x in idents(c[2]) if x not in ("game_ai", "plan_legacy", "sub_plan")][-3:]), u"%d/%d" % (len(c[4]), len(need)), u"★" if c[0] == 1.0 and c[1] > 0.3 else u"", c[3]) for c in cands[:3])
        L.append(u"| `%s` | %s | %dB | %s | %s |" % (a, n.get("n") or u"—", n["b"], u" · ".join(sorted(need))[:90] or u"(지도 이름 콜리 없음)", cs or u"—"))
        print(u"%s %-40s need=%d cands=%d %s" % (a, (n.get("n") or "-")[:40], len(need), len(cands), (u"★" + u"::".join(idents(cands[0][2])[-2:])) if cands and cands[0][0] == 1.0 else u""))
    json.dump(res, io.open(os.path.join(HERE, "_next", "extname.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    io.open(os.path.join(HERE, "_next", "extname.md"), "w", encoding="utf-8").write(u"\n".join(L))

if __name__ == "__main__":
    main()
