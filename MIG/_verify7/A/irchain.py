# -*- coding: utf-8 -*-
u"""IR 줄범위를 훑어 각 명령의 !dbg inlinedAt 사슬을 (파일, 줄) 로 펼쳐 찍는다.

사용: python -X utf8 irchain.py <m04.ll> <from> <to> [--grep PAT] [--own abstract_input.rs]
G12/G13 판정의 근거를 손으로 확인하기 위한 조회 전용 도구.
"""
import io, os, re, sys

sys.stdout.reconfigure(encoding="utf-8", errors="replace")
IRDIR = r"C:\tfm2mods\_gaibc"
LOC = re.compile(r"!DILocation\(line: (\d+)(?:, column: \d+)?, scope: !(\d+)(?:, inlinedAt: !(\d+))?\)")
FILEOF = re.compile(r'filename: "([^"]+)"')
DBG = re.compile(r"!dbg !(\d+)")

_cache = {}


def load(f):
    if f in _cache:
        return _cache[f]
    p = f if os.path.isabs(f) else os.path.join(IRDIR, f)
    src = io.open(p, encoding="utf-8", errors="replace").read().split("\n")
    meta = {}
    for ln in src:
        if ln.startswith("!"):
            m = re.match(r"^!(\d+) = (.*)$", ln)
            if m:
                meta[m.group(1)] = m.group(2)
    _cache[f] = (src, meta)
    return _cache[f]


def scope_file(meta, sid, d=0):
    cur = sid
    while cur and cur in meta and d < 40:
        t = meta[cur]
        m = re.search(r"file: !(\d+)", t)
        if m:
            fn = FILEOF.search(meta.get(m.group(1), ""))
            if fn:
                return fn.group(1).split("\\")[-1]
        m2 = re.search(r"scope: !(\d+)", t)
        if not m2:
            return "?"
        cur = m2.group(1)
        d += 1
    return "?"


def chain(meta, n):
    out, cur, d = [], n, 0
    while cur and cur in meta and d < 48:
        m = LOC.search(meta[cur])
        if not m:
            break
        line, scope, inl = m.groups()
        out.append((scope_file(meta, scope), int(line)))
        if not inl:
            break
        cur, d = inl, d + 1
    return out


def main():
    f = sys.argv[1]
    a = int(sys.argv[2])
    b = int(sys.argv[3])
    pat = None
    own = None
    args = sys.argv[4:]
    i = 0
    while i < len(args):
        if args[i] == "--grep":
            pat = re.compile(args[i + 1])
            i += 2
        elif args[i] == "--own":
            own = args[i + 1]
            i += 2
        else:
            i += 1
    src, meta = load(f)
    for k in range(a - 1, min(b, len(src))):
        ln = src[k]
        if pat and not pat.search(ln):
            continue
        m = DBG.search(ln)
        ch = chain(meta, m.group(1)) if m else []
        if own:
            ch = [(fn, li) for (fn, li) in ch]
        print(u"%-7d| %-118s | %s" % (k + 1, ln.strip()[:118],
                                      u" <- ".join(u"%s:%d" % (fn, li) for fn, li in ch)))


main()
