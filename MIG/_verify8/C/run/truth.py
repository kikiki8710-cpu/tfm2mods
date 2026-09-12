# -*- coding: utf-8 -*-
u"""20함수 전량 — IR define 인자 ↔ spec params 정렬 실태표(sret 규약 결정용 원자료)."""
import io, json, os, re, sys
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
MIG = r"C:\tfm2mods\MIG"
IRDIR = r"C:\tfm2mods\_gaibc"
D = json.load(io.open(os.path.join(MIG, "_spec", "specs20_v3.json"), encoding="utf-8"))


def defline(f, a, b):
    src = io.open(os.path.join(IRDIR, f), encoding="utf-8", errors="replace").read().split("\n")
    for k in range(a - 1, min(a + 6, len(src))):
        if src[k].lstrip().startswith("define"):
            return src[k], src, k + 1
    return None, src, None


def split_args(ln):
    at = ln.find("@")
    i = ln.find("(", at)
    depth, j = 0, i
    while j < len(ln):
        if ln[j] == "(":
            depth += 1
        elif ln[j] == ")":
            depth -= 1
            if depth == 0:
                break
        j += 1
    inner = ln[i + 1:j]
    parts, d, cur = [], 0, ""
    for ch in inner:
        if ch in "([{":
            d += 1
        elif ch in ")]}":
            d -= 1
        if ch == "," and d == 0:
            parts.append(cur.strip()); cur = ""
        else:
            cur += ch
    if cur.strip():
        parts.append(cur.strip())
    return parts


def is_sret(a0, ln):
    if "sret(" in a0:
        return True
    # internal fastcc 는 sret 속성을 떼지만 슬롯은 남는다
    return ("dead_on_unwind" in a0 and "writable" in a0 and "writeonly" in a0)


print(u"| # | 함수 | ret | IRargs | sretIR | spec | p0.name | p0.i | 정렬 |")
print(u"|---|---|---|---|---|---|---|---|---|")
for i in range(20):
    sp = D["specs"][i]
    ir = sp["ir"]
    ln, src, lno = defline(ir["file"], ir["frm"], ir["to"])
    args = split_args(ln)
    ps = (sp.get("sig") or {}).get("params") or []
    s = is_sret(args[0], ln) if args else False
    p0n = (ps[0].get("name") or u"") if ps else u""
    p0i = ps[0].get("i") if ps else None
    listed = u"sret" in p0n
    if len(args) == len(ps):
        al = u"1:1"
    elif len(args) == len(ps) + 1:
        al = u"IR+1(sret 미등재)" if s else u"IR+1(?)"
    else:
        al = u"IR%+d" % (len(args) - len(ps))
    rt = ln.split("@")[0].replace("define", "").strip()
    print(u"| %02d | %s | %s | %d | %s | %d | %s | %s | %s |"
          % (i, sp["name"][:34], rt[-22:], len(args), u"O" if s else u"-", len(ps), p0n, p0i, al))
