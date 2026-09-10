# -*- coding: utf-8 -*-
u"""필드 이름 -> (타입, 오프셋) 역검색.
사용: python -X utf8 tcxfield.py <필드명정규식> [오프셋(16진, 선택)]
"""
import json, io, os, sys, re, functools
TCX = r"C:\tfm2mods\MIG\_tcx"
CRATES = ("game_ai", "game_core", "game_view")

@functools.lru_cache(maxsize=8)
def load(cr):
    with io.open(os.path.join(TCX, cr + ".json"), encoding="utf-8") as f:
        return json.load(f)

def main():
    rx = re.compile(sys.argv[1])
    want = int(sys.argv[2], 16) if len(sys.argv) > 2 else None
    for cr in CRATES:
        for it in load(cr)["items"]:
            if "adt" not in it: continue
            lay = it.get("layout") or {}
            vs = it["adt"]["variants"]
            for vi, v in enumerate(vs):
                off = None
                if lay:
                    vl = lay.get("vlayouts")
                    off = vl[vi]["offsets"] if vl and vi < len(vl) else lay.get("offsets")
                for fi, fd in enumerate(v["fields"]):
                    if not rx.search(fd["n"]): continue
                    o = off[fi] if off and fi < len(off) else None
                    if want is not None and o != want: continue
                    print("%-10s %-64s %-6s +0x%-6s %s : %s" % (
                        cr, it["p"], v["n"], format(o, "x") if o is not None else "?", fd["n"], fd["t"]))

main()
