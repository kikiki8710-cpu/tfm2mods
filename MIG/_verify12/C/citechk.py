# -*- coding: utf-8 -*-
u"""명세 본문의 IR 줄 인용(`mNN.ll:1234`, `IR 11529·11546`)을 실제 IR 원문과 대조한다. (12차 배치C)
어떤 게이트도 안 보는 축 — 10차·11차가 여기서 오기를 냈다."""
import io, json, os, re, sys
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
MIG = r"C:\tfm2mods\MIG"
DIRS = [r"C:\tfm2mods\_gaibc", r"C:\tfm2mods\_gcbc", r"C:\tfm2mods\_gvbc"]
D = json.load(io.open(os.path.join(MIG, "_spec", "specs20_v3.json"), encoding="utf-8"))
CACHE = {}


def line(f, n):
    if f not in CACHE:
        for d in DIRS:
            p = os.path.join(d, f)
            if os.path.exists(p):
                CACHE[f] = io.open(p, encoding="utf-8", errors="replace").read().split("\n")
                break
        else:
            CACHE[f] = None
    src = CACHE[f]
    if src is None or not (1 <= n <= len(src)):
        return None
    return src[n - 1].strip()


CITE = re.compile(r"([mgv]\d\d)\.ll:(\d{3,7})(?:\s*[~·,]\s*(\d{3,7}))*")
NUM = re.compile(r"(\d{3,7})")

for i in (10, 11, 12, 13, 14):
    s = D["specs"][i]
    print(u"\n===== [%02d] %s" % (i, s.get("name")))
    for fld in ("logic", "one_line"):
        pass
    blob = json.dumps(s, ensure_ascii=False)
    seen = set()
    for m in re.finditer(r"([mgv]\d\d)\.ll:([\d~·, \-]+)", blob):
        f = m.group(1) + ".ll"
        for n in NUM.findall(m.group(2))[:6]:
            n = int(n)
            if (f, n) in seen:
                continue
            seen.add((f, n))
            t = line(f, n)
            print(u"  %s:%-7d %s" % (f, n, (t or u"<없음>")[:150]))
