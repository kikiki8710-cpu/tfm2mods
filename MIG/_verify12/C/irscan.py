# -*- coding: utf-8 -*-
u"""IR->명세 방향 스캔: 담당 함수 IR 범위의 gep 상수 오프셋·리터럴을 전부 뽑아
명세 mem/consts 표에 있는지 대조한다. (12차 배치C)"""
import io, json, os, re, sys
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
MIG = r"C:\tfm2mods\MIG"
GAI = r"C:\tfm2mods\_gaibc"
D = json.load(io.open(os.path.join(MIG, "_spec", "specs20_v3.json"), encoding="utf-8"))

GEP = re.compile(r"getelementptr[^,]*,\s*ptr\s+(%\w+|@[\w.]+)\s*,\s*i64\s+(-?\d+)")
GEP2 = re.compile(r"getelementptr\s+(?:inbounds\s+)?(?:nuw\s+)?\[?[^,]*,\s*ptr\s+(%\w+|@[\w.]+)\s*,\s*i64\s+(-?\d+)")
LIT = re.compile(r"\b(?:icmp|add|sub|mul|udiv|urem|and|or|xor|shl|lshr|ashr|select|switch|store)\b[^\n]*?"
                 r"(-?\d{2,})\b")

for i in (10, 11, 12, 13, 14):
    s = D["specs"][i]
    ir = s.get("ir") or {}
    f, a, b = ir.get("file"), ir.get("frm"), ir.get("to")
    src = io.open(os.path.join(GAI, f), encoding="utf-8", errors="replace").read().split("\n")
    body = src[a - 1:b]
    geps = {}
    for k, ln in enumerate(body):
        for m in GEP2.finditer(ln):
            geps.setdefault(int(m.group(2)), []).append((a + k, m.group(1)))
    memoffs = set()
    for r in s.get("mem") or []:
        o = str(r.get("offset") or "").strip().lower()
        try:
            memoffs.add(int(o, 16) if o.startswith("0x") else int(o))
        except Exception:
            pass
    cvals = set()
    for c in s.get("consts") or []:
        try:
            cvals.add(int(c.get("value")))
        except Exception:
            pass
    print(u"\n=== [%02d] %s  (%s %s~%s)  mem=%d consts=%d" % (i, s.get("name"), f, a, b, len(memoffs), len(cvals)))
    miss = sorted(o for o in geps if o not in memoffs)
    print(u"  gep 오프셋 %d종 / mem 미등재 %d종" % (len(geps), len(miss)))
    for o in miss:
        print(u"    +0x%-5x (%d회) 예: L%d base=%s" % (o, len(geps[o]), geps[o][0][0], geps[o][0][1]))
    # 리터럴
    lits = {}
    for k, ln in enumerate(body):
        if "#dbg" in ln or "!DI" in ln:
            continue
        t = re.sub(r"!dbg.*$", "", ln)
        t = re.sub(r"dereferenceable\(\d+\)", "", t)
        t = re.sub(r"align \d+", "", t)
        t = re.sub(r"getelementptr[^\n]*", "", t)
        for m in re.finditer(r"\b(?:icmp \w+|add nsw|add nuw|add|sub|mul|udiv|urem|and|or|xor|shl|lshr|ashr|select|switch)\b[^\n]*", t):
            for n in re.finditer(r"(?<![\w%.])(-?\d+)(?![\w.])", m.group(0)):
                v = int(n.group(1))
                if abs(v) >= 2:
                    lits.setdefault(v, []).append(a + k)
    ml = sorted(v for v in lits if v not in cvals)
    print(u"  산술/비교 리터럴 %d종 / consts 미등재 %d종: %s"
          % (len(lits), len(ml), u", ".join(u"%d(L%d,%d회)" % (v, lits[v][0], len(lits[v])) for v in ml[:40])))
