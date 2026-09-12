# -*- coding: utf-8 -*-
u"""`consts[].src_line` 기계 대조 — 그 리터럴이 실제로 어느 소스 줄에서 왔는지 **inlinedAt 사슬 전체**로 본다.

판정: 주장한 `src_line` 이 그 리터럴을 쓰는 명령의 사슬(담당 `.rs` 파일 프레임만) 어디에도 없으면 **틀린 값**.
※ 사슬 전체를 보는 이유 = 인라인된 accessor 는 최내곽이 `entity.rs` 이고, 진짜 소스 줄은 중간 프레임에 있다.
"""
import io, json, os, re, sys
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
HERE = os.path.dirname(os.path.abspath(__file__))
MIG = os.path.dirname(os.path.dirname(HERE))
IRDIR = r"C:\tfm2mods\_gaibc"
LOC = re.compile(r"!DILocation\(line: (\d+)(?:, column: \d+)?, scope: !(\d+)(?:, inlinedAt: !(\d+))?\)")
FILEOF = re.compile(r'filename: "([^"]+)"')
DBG = re.compile(r"!dbg !(\d+)")

_cache = {}


def load(f):
    if f in _cache:
        return _cache[f]
    src = io.open(os.path.join(IRDIR, f), encoding="utf-8", errors="replace").read().split("\n")
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


D = json.load(io.open(os.path.join(MIG, "_spec", "specs20_v3.json"), encoding="utf-8"))
for i in range(5):
    sp = D["specs"][i]
    f, a, b = sp["ir"]["file"], sp["ir"]["frm"], sp["ir"]["to"]
    own = sp["src"].split("\\")[-1]
    src, meta = load(f)
    print(u"\n===== specs[%d] %s  (%s  %s:%d~%d) =====" % (i, sp["name"], own, f, a, b))
    # 범위 안 모든 명령의 사슬에서 '담당 .rs' 프레임 줄 집합
    ownlines = {}
    for k in range(a - 1, min(b, len(src))):
        ln = src[k]
        if "#dbg_" in ln:
            continue
        m = DBG.search(ln)
        if not m:
            continue
        for (fn, li) in chain(meta, m.group(1)):
            if fn == own:
                ownlines.setdefault(li, 0)
                ownlines[li] += 1
    print(u"  본문이 실제로 참조하는 %s 줄 = %s" % (own, sorted(ownlines)))
    for j, c in enumerate(sp.get("consts") or []):
        val, claim = c.get("value"), c.get("src_line")
        pat = re.compile(r"(?<![\w.\-])" + re.escape(str(val)) + r"(?![\w.])")
        found = {}
        for k in range(a - 1, min(b, len(src))):
            ln = src[k]
            if "#dbg_" in ln or not pat.search(ln):
                continue
            m = DBG.search(ln)
            if not m:
                continue
            for (fn, li) in chain(meta, m.group(1)):
                if fn == own:
                    found.setdefault(li, 0)
                    found[li] += 1
        ok = claim in found
        print(u"  consts[%d] value=%-12s claim=L%-5s 실제후보=%-28s %s"
              % (j, val, claim, sorted(found) if found else u"(리터럴 미검출)",
                 u"OK" if ok else (u"**불일치**" if found else u"(판정보류: 리터럴이 접혔거나 범위 밖)")))
