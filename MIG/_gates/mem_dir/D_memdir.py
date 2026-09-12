# -*- coding: utf-8 -*-
u"""`mem[].dir` 기계 대조 — IR 의 gep→load/store 를 오프셋별로 접어 방향 표를 만든다.

§4-b 의 「무검사 축」 3개 중 `mem.dir`(451행) 을 처음 검사하는 도구다.

원리
  1. `%R = getelementptr inbounds nuw i8, ptr %B, i64 N`  → R = (B, N)  (정적 바이트 오프셋)
     `%R = getelementptr inbounds nuw <TY>, ptr %B, i64 %i` → R = (B, None)  (동적 인덱스)
     gep 가 gep 를 받으면 오프셋을 **누적**한다.
  2. `load  T, ptr %R`  → (base, off) 에 'r'
     `store T V, ptr %R` → (base, off) 에 'w'
     레지스터가 gep 가 아니면 base=그 레지스터 자신, off=0.
  3. 결과 = {오프셋: {'r','w'}} (베이스 무시 접기) + {(base,off): dirs} 둘 다 낸다.

판정
  - 명세 행의 오프셋이 표에 **있는데** 주장한 dir 이 없다 → **강한 불일치**(실오류 후보)
  - 오프셋이 표에 아예 없다 → 약한 신호(다른 베이스·vtable 슬롯·상수접힘·동적 인덱스)

한계(범위 명시)
  - vtable 슬롯(`dyn` 간접호출)은 gep 가 아니라 `load ptr` 후 호출이라 오프셋 표에 'r' 로만 뜬다.
  - `memcpy`/`memset` 대상은 store 로 안 잡힌다 → **`--intr` 로 따로 표시**한다.
  - 베이스를 접으므로 서로 다른 구조체의 같은 오프셋이 섞인다. 그래서 '강한 불일치'만 결함으로 센다.

용법: python -X utf8 memdir.py [specidx ...]
"""
import io, json, os, re, sys
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
MIG = r"C:\tfm2mods\MIG"
IRDIR = r"C:\tfm2mods\_gaibc"

GEP_B = re.compile(r"^\s*%(\d+) = getelementptr[^,]*, ptr (%?[\w.]+), i64 (-?\d+)")
GEP_D = re.compile(r"^\s*%(\d+) = getelementptr[^,]*, ptr (%?[\w.]+), i64 %")
LOAD = re.compile(r"^\s*%(\d+) = load [^,]+, ptr (%?[\w.]+)")
STORE = re.compile(r"^\s*store [^,]+, ptr (%?[\w.]+)")
INTR = re.compile(r"llvm\.(memcpy|memset)\.[^(]*\(ptr[^,]*? (%?[\w.]+)")


def analyze(f, a, b):
    src = io.open(os.path.join(IRDIR, f), encoding="utf-8", errors="replace").read().split("\n")
    gep = {}          # reg -> (base, off|None)
    offmap = {}       # off -> set(dir)
    pairmap = {}      # (base, off) -> set(dir)
    intr = []         # (base, off, kind)

    def resolve(r):
        base, off, d = r, 0, 0
        while base in gep and d < 32:
            nb, no = gep[base]
            if no is None:
                return (nb, None)
            base, off, d = nb, off + no, d + 1
        return (base, off)

    for k in range(a - 1, min(b, len(src))):
        ln = src[k]
        if "#dbg_" in ln:
            continue
        m = GEP_B.match(ln)
        if m:
            gep["%" + m.group(1)] = (m.group(2), int(m.group(3)))
            continue
        m = GEP_D.match(ln)
        if m:
            gep["%" + m.group(1)] = (m.group(2), None)
            continue
        hit = None
        m = LOAD.match(ln)
        if m:
            hit = (m.group(2), "r")
        else:
            m = STORE.match(ln)
            if m:
                hit = (m.group(1), "w")
        if hit:
            base, off = resolve(hit[0])
            if off is not None:
                offmap.setdefault(off, set()).add(hit[1])
                pairmap.setdefault((base, off), set()).add(hit[1])
            continue
        m = INTR.search(ln)
        if m:
            base, off = resolve(m.group(2))
            intr.append((base, off, m.group(1)))
    return offmap, pairmap, intr


def main():
    D = json.load(io.open(os.path.join(MIG, "_spec", "specs20_v3.json"), encoding="utf-8"))
    idxs = [int(x) for x in sys.argv[1:] if x.isdigit()] or list(range(20))
    tot = strong = absent = 0
    for i in idxs:
        sp = D["specs"][i]
        ir = sp.get("ir") or {}
        f, a, b = ir.get("file"), ir.get("frm"), ir.get("to")
        if not f:
            continue
        offmap, pairmap, intr = analyze(f, a, b)
        print(u"\n===== specs[%d] %s  (%s:%d~%d) =====" % (i, sp["name"], f, a, b))
        print(u"  IR 오프셋 방향표 %d개 · memcpy/memset 대상 %d개" % (len(offmap), len(intr)))
        if intr:
            print(u"  intr: %s" % (sorted(set((x[1], x[2]) for x in intr))[:20],))
        for j, r in enumerate(sp.get("mem") or []):
            off = r.get("offset")
            if isinstance(off, str):
                try:
                    off = int(off, 16) if off.startswith("0x") else int(off)
                except Exception:
                    continue
            if off is None:
                continue
            claim = (r.get("dir") or "").strip()
            tot += 1
            have = offmap.get(off)
            if have is None:
                absent += 1
                mark = u"(오프셋 미검출)"
            elif claim in have:
                mark = u"OK"
            else:
                strong += 1
                mark = u"**강한 불일치**  IR=%s" % ("".join(sorted(have)),)
            if mark != u"OK":
                print(u"  mem[%-2d] %-34s +0x%-4x dir=%-2s %s"
                      % (j, (r.get("base") or "")[:34], off, claim, mark))
    print(u"\n---- 합계: 검사 %d · 강한 불일치 %d · 오프셋 미검출 %d" % (tot, strong, absent))


main()
