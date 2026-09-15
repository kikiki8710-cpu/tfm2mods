# -*- coding: utf-8 -*-
"""annotated d28800.ll / m04.ll 조회 헬퍼. usage:
  python ir.py show 21039 21050        # 주석본 줄 범위
  python ir.py raw 21039 21050         # 원문 m04.ll 줄 범위
  python ir.py grep <regex>            # 주석본 본문 grep (줄번호 원문)
  python ir.py meta <!N>               # 원문 metadata 줄 + inlinedAt 사슬 전개
  python ir.py stores                  # %0(self) / %7(debug) 기준 store 전수
"""
import io, re, sys
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
ANN = r"C:\tfm2mods\MIG\_next\reach\d28800.ll"
RAW = r"C:\tfm2mods\_gaibc\m04.ll"

def load_ann():
    d = {}
    with io.open(ANN, encoding="utf-8", errors="replace") as f:
        for l in f:
            m = re.match(r"\s*(\d+)\|(.*)$", l.rstrip("\n"))
            if m:
                d[int(m.group(1))] = m.group(2)
    return d

_raw = None
def load_raw():
    global _raw
    if _raw is None:
        with io.open(RAW, encoding="utf-8", errors="replace") as f:
            _raw = f.read().split("\n")
    return _raw

def meta_chain(n):
    raw = load_raw()
    idx = {}
    pat = re.compile(r"^!(\d+) = ")
    # build index lazily only for the lines we need: scan once
    out = []
    want = n
    # full index (metadata lines are at the end; scanning 14MB once is ok)
    for i, l in enumerate(raw):
        if l.startswith("!"):
            m = pat.match(l)
            if m:
                idx[int(m.group(1))] = l
    while True:
        l = idx.get(want)
        if l is None:
            out.append(f"!{want} = <없음>"); break
        out.append(l[:300])
        m = re.search(r"inlinedAt: !(\d+)", l)
        if m:
            want = int(m.group(1))
        else:
            # also print scope file
            ms = re.search(r"scope: !(\d+)", l)
            if ms:
                out.append("  scope: " + idx.get(int(ms.group(1)), "")[:300])
            break
    return out

if __name__ == "__main__":
    cmd = sys.argv[1]
    if cmd == "show":
        d = load_ann(); a, b = int(sys.argv[2]), int(sys.argv[3])
        for k in range(a, b + 1):
            if k in d: print(f"{k}|{d[k][:260]}")
    elif cmd == "raw":
        raw = load_raw(); a, b = int(sys.argv[2]), int(sys.argv[3])
        for k in range(a, b + 1):
            print(f"{k}|{raw[k-1][:400]}")
    elif cmd == "grep":
        d = load_ann(); pat = re.compile(sys.argv[2])
        for k in sorted(d):
            if pat.search(d[k]): print(f"{k}|{d[k][:260]}")
    elif cmd == "meta":
        for l in meta_chain(int(sys.argv[2].lstrip("!"))): print(l)
    elif cmd == "rawgrep":
        raw = load_raw(); pat = re.compile(sys.argv[2]); a = int(sys.argv[3]); b = int(sys.argv[4])
        for k in range(a, b + 1):
            if pat.search(raw[k-1]): print(f"{k}|{raw[k-1][:400]}")
