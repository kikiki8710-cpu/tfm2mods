# -*- coding: utf-8 -*-
"""_tcx\<crate>.json 조회 헬퍼.
사용:
  python tcxq.py items <crate> <파일경로부분> [줄시작 줄끝]   # 그 파일(구간) 안의 아이템 def_span 전량
  python tcxq.py lines <crate> <파일경로부분> <a> <b>          # a~b 줄의 바이트 길이
  python tcxq.py adt   <crate> <타입이름부분>                  # 구조체/열거형 필드·판별자·레이아웃
  python tcxq.py grep  <crate> <경로정규식>                    # def_path_str 정규식 검색
  python tcxq.py pub   <crate> [모듈접두]                      # 공개(pub) 아이템 표면
"""
import json, sys, io, os, re, functools

TCX = r"C:\tfm2mods\MIG\_tcx"

@functools.lru_cache(maxsize=8)
def load(cr):
    with io.open(os.path.join(TCX, cr + ".json"), encoding="utf-8") as f:
        return json.load(f)

def files_index(d):
    return {f["f"]: f for f in d["files"]}

def line_bytes(fobj, a, b):
    L = fobj["lines"]; n = len(L); total = fobj["len"]
    out = []
    for ln in range(a, b + 1):
        if ln < 1 or ln > n: continue
        s = L[ln - 1]
        e = L[ln] if ln < n else total
        out.append((ln, e - s))   # 개행 포함 길이
    return out

def main():
    cmd = sys.argv[1]; cr = sys.argv[2]; d = load(cr)
    if cmd == "items":
        pat = sys.argv[3]
        a = int(sys.argv[4]) if len(sys.argv) > 4 else 0
        b = int(sys.argv[5]) if len(sys.argv) > 5 else 10**9
        rows = []
        for it in d["items"]:
            sp = it.get("sp")
            if not sp or pat not in sp["f"]: continue
            if not (a <= sp["l"] <= b): continue
            rows.append((sp["l"], sp["c"], sp["l2"], sp["c2"], it["k"], it["p"], it["v"], it["mir"], it["xinl"]))
        for r in sorted(rows):
            print("%5d:%-4d-%5d:%-4d %-28s %s  vis=%s mir=%d xinl=%d" % (r[0], r[1], r[2], r[3], r[4], r[5], r[6], r[7], r[8]))
    elif cmd == "lines":
        pat = sys.argv[3]; a = int(sys.argv[4]); b = int(sys.argv[5])
        fi = files_index(d)
        for k, v in fi.items():
            if pat not in k: continue
            print("== %s  (nl=%d len=%d)" % (k, v["nl"], v["len"]))
            for ln, nb in line_bytes(v, a, b):
                print("  L%-5d bytes(개행포함)=%d" % (ln, nb))
    elif cmd == "adt":
        pat = sys.argv[3]
        for it in d["items"]:
            if "adt" not in it: continue
            if pat not in it["p"]: continue
            sp = it.get("sp") or {}
            print("### %s  %s  %s:%s  repr=%s" % (it["k"], it["p"], sp.get("f"), sp.get("l"), it["adt"]["repr"]))
            lay = it.get("layout")
            if lay: print("    layout size=%s align=%s offsets=%s single=%s tag=%s enc=%s tagfield=%s" % (
                lay.get("size"), lay.get("align"), lay.get("offsets"), lay.get("single"),
                lay.get("tag"), lay.get("tag_enc"), lay.get("tag_field")))
            for vi, v in enumerate(it["adt"]["variants"]):
                vsp = v.get("sp") or {}
                vl = (lay or {}).get("vlayouts")
                off = vl[vi]["offsets"] if vl and vi < len(vl) else (lay or {}).get("offsets")
                print("  - variant %s discr=%s @L%s off=%s" % (v["n"], v["discr"], vsp.get("l"), off))
                for fi2, fd in enumerate(v["fields"]):
                    o = off[fi2] if off and fi2 < len(off) else "?"
                    print("      +0x%-5s %-34s %s" % (format(o, "x") if isinstance(o, int) else o, fd["n"], fd["t"]))
    elif cmd == "grep":
        rx = re.compile(sys.argv[3])
        for it in d["items"]:
            if rx.search(it["p"]):
                sp = it.get("sp") or {}
                print("%-30s %-70s vis=%-10s mir=%d xinl=%d %s:%s:%s" % (
                    it["k"], it["p"], it["v"], it["mir"], it["xinl"], sp.get("f"), sp.get("l"), sp.get("c")))
    elif cmd == "pub":
        pref = sys.argv[3] if len(sys.argv) > 3 else ""
        for it in d["items"]:
            if it["v"] != "pub": continue
            if pref and not it["p"].startswith(pref): continue
            if it["k"] in ("Use", "ExternCrate", "LifetimeParam", "TyParam", "Mod"): continue
            sp = it.get("sp") or {}
            print("%-28s %s   %s:%s" % (it["k"], it["p"], sp.get("f"), sp.get("l")))

main()
