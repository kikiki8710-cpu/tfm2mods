# -*- coding: utf-8 -*-
u"""mem.dir 기계 대조 시제품 (7차 배치A) — `mem[].dir`(r/w) 를 IR 의 load/store 와 직접 대조한다.

방법:
  1) 담당 줄범위에서 `%X = getelementptr inbounds nuw i8, ptr %B, i64 N` 을 모아 `%X -> (base, N)` 표를 만든다.
     (`[5 x ptr]`/`{i64,..}` 처럼 **타입 인덱싱** gep 은 바이트 오프셋이 아니므로 stride 로 따로 기록)
  2) `load .., ptr %X` → (N, 'r') · `store .., ptr %X` → (N, 'w') 로 센다.
     gep 을 거치지 않고 인자 포인터를 바로 쓰면 오프셋 0 이다(gep 접힘).
  3) 명세의 각 `mem[j]` 오프셋이 **주장한 방향으로** 관측됐는지 본다.

한계(설계상 — 다음 라운드가 이걸 고치면 게이트가 된다):
  · 오프셋만 보고 **베이스 타입은 안 본다** — 서로 다른 구조체가 같은 오프셋을 쓰면 구분 못 한다.
  · 담당 범위 밖(aux: 클로저·인라인 안 된 헬퍼)의 접근은 「범위밖」으로 빠진다.

사용: python -X utf8 memdir.py <spec인덱스...>
"""
import io, json, os, re, sys

sys.stdout.reconfigure(encoding="utf-8", errors="replace")
HERE = os.path.dirname(os.path.abspath(__file__))
MIG = os.path.dirname(os.path.dirname(HERE))
IRDIR = r"C:\tfm2mods\_gaibc"
D = json.load(io.open(os.path.join(MIG, "_spec", "specs20_v3.json"), encoding="utf-8"))

GEPB = re.compile(r"^\s*%([\w.]+) = getelementptr inbounds nuw i8, ptr %([\w.]+), i64 (-?\d+)")
GEPT = re.compile(r"^\s*%([\w.]+) = getelementptr inbounds nuw ([^,]+), ptr %([\w.]+),")
LOAD = re.compile(r"^\s*%([\w.]+) = load [^,]+, ptr %([\w.]+)")
STORE = re.compile(r"^\s*store [^,]+, ptr %([\w.]+)")
MEMCPY = re.compile(r"llvm\.mem(cpy|set)[^(]*\(ptr[^%]*%([\w.]+)")


def parse(f, a, b):
    src = io.open(os.path.join(IRDIR, f), encoding="utf-8", errors="replace").read().split("\n")
    body = src[a - 1:b]
    off = {}          # ssa -> 절대 바이트 오프셋(같은 사슬 누적)
    seen = {}         # 오프셋 -> set('r','w')
    for ln in body:
        if "#dbg_" in ln:
            continue
        m = GEPB.match(ln)
        if m:
            dst, base, n = m.group(1), m.group(2), int(m.group(3))
            off[dst] = off.get(base, 0) + n
            continue
        m = GEPT.match(ln)
        if m:
            off[m.group(1)] = off.get(m.group(3), 0)   # 배열 인덱싱: 베이스 오프셋 유지
            continue
        m = LOAD.match(ln)
        if m:
            seen.setdefault(off.get(m.group(2), 0), set()).add("r")
            continue
        m = STORE.match(ln)
        if m:
            seen.setdefault(off.get(m.group(1), 0), set()).add("w")
            continue
        m = MEMCPY.search(ln)
        if m:
            seen.setdefault(off.get(m.group(2), 0), set()).add("w")
    return seen


for arg in sys.argv[1:]:
    i = int(arg)
    sp = D["specs"][i]
    f, a, b = sp["ir"]["file"], sp["ir"]["frm"], sp["ir"]["to"]
    seen = parse(f, a, b)
    print(u"\n=== specs[%d] %s  (%s %d~%d)" % (i, sp["name"], f, a, b))
    nok = nbad = nout = 0
    for j, m in enumerate(sp.get("mem") or []):
        o = m.get("offset")
        try:
            n = int(str(o), 16) if str(o).startswith("0x") else int(o)
        except Exception:
            n = None
        d = (m.get("dir") or "").strip()
        got = seen.get(n, set()) if n is not None else set()
        if not got:
            st, nout = u"범위밖/접힘", nout + 1
        elif d in got:
            st, nok = u"OK", nok + 1
        else:
            st, nbad = u"★불일치 (IR=%s)" % (u"".join(sorted(got)),), nbad + 1
        print(u"  mem[%-2d] %-34s %-8s dir=%-2s  IR=%-4s %s"
              % (j, (m.get("base") or u"")[:34], o, d, u"".join(sorted(got)) or u"-", st))
    print(u"  -> OK %d · 불일치 %d · 범위밖/접힘 %d" % (nok, nbad, nout))
