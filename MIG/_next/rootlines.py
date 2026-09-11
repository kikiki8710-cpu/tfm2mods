# -*- coding: utf-8 -*-
u"""rootlines — IR 함수 본문의 `!dbg` 를 **inlinedAt 루트까지** 펼쳐
그 함수 자신의 소스 줄 집합을 낸다. (심볼↔RVA 등호 확정용)
"""
import io, json, os, re, sys
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
sym = sys.argv[1] if len(sys.argv) > 1 else \
    "_RNvMNtNtCshdEBA0ozCnw_7game_ai11plan_legacy7handlerNtB2_17LegacyPlanHandler6update"
irs = json.load(io.open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "irsize.json"),
                        encoding="utf-8"))
u = [r for r in irs if r["sym"] == sym][0]
path = os.path.join(r"C:\tfm2mods\_gaibc", u["file"])

LOC = re.compile(r'^!(\d+) = !DILocation\(line: (\d+), column: (\d+), scope: !(\d+)(?:, inlinedAt: !(\d+))?\)')
SUB = re.compile(r'^!(\d+) = distinct !DISubprogram\(name: "([^"]*)".*?file: !(\d+).*?line: (\d+)')
BLK = re.compile(r'^!(\d+) = !DILexicalBlock\(scope: !(\d+)')
FIL = re.compile(r'^!(\d+) = !DIFile\(filename: "([^"]*)"')
loc, sub, blk, fil, body = {}, {}, {}, {}, []
with io.open(path, "rb") as f:
    for i, raw in enumerate(f, 1):
        t = raw.decode("utf-8", "replace").rstrip()
        if u["frm"] <= i <= u["to"]:
            body.append(t)
        if not t.startswith("!"):
            continue
        m = LOC.match(t)
        if m:
            loc[m.group(1)] = (int(m.group(2)), m.group(4), m.group(5)); continue
        m = SUB.match(t)
        if m:
            sub[m.group(1)] = (m.group(2), m.group(3), int(m.group(4))); continue
        m = BLK.match(t)
        if m:
            blk[m.group(1)] = m.group(2); continue
        m = FIL.match(t)
        if m:
            fil[m.group(1)] = m.group(2)


def scope_fn(sid, depth=0):
    while sid in blk and depth < 40:
        sid = blk[sid]; depth += 1
    return sub.get(sid)


def root(lid, depth=0):
    while lid in loc and depth < 60:
        line, sid, inl = loc[lid]
        if inl is None:
            return line, scope_fn(sid)
        lid = inl; depth += 1
    return None, None


ids = set(re.findall(r"!dbg !(\d+)", "\n".join(body)))
byfile = {}
for i in ids:
    line, fn = root(i)
    if line is None or fn is None:
        continue
    f = os.path.basename(fil.get(fn[1], "?").replace("/", "\\"))
    byfile.setdefault((f, fn[0]), set()).add(line)
print(u"본문 !dbg %d개 → inlinedAt 루트 집계" % len(ids))
for (f, fn), ls in sorted(byfile.items(), key=lambda kv: -len(kv[1])):
    ls = sorted(x for x in ls if x)
    if not ls:
        continue
    print(u"  %-22s %-28s 줄 %4d개  %d~%d" % (f, fn[:28], len(ls), ls[0], ls[-1]))
