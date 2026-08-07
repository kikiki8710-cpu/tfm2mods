# 편집기 설정값 배치·설명 감사: 탭 구조 덤프 + 누락/고아/중복 탐지
import re, sys, collections, io, os

SRC = os.path.join(os.path.dirname(os.path.abspath(__file__)), "src", "main.rs")
CFG = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\mods\tfm2_ai_adjust\tfm2_ai_adjust.cfg"
s = open(SRC, encoding="utf-8").read()

STR = re.compile(r'"([^"\\]*(?:\\.[^"\\]*)*)"')
TAB = re.compile(r'Tab\{ id:"([a-z0-9_]+)", title:"([^"]*)", keys:&\[(.*?)\], note:', re.S)

tabs = []
for m in TAB.finditer(s):
    tabs.append((m.group(1), m.group(2), STR.findall(m.group(3))))

lines, allk, placed = [], [], {}
for tid, title, items in tabs:
    ks = [i for i in items if not i.startswith("\u00a7")]
    allk += ks
    lines.append("\n== [%s] %s  (%d\ud0a4)" % (tid, title, len(ks)))
    cur = "(\uc139\uc158\uc5c6\uc74c)"
    for i in items:
        if i.startswith("\u00a7"):
            cur = i; lines.append("  " + i)
        else:
            lines.append("     " + i); placed.setdefault(i, []).append((tid, cur))

# desc_static / orig_val 키 수집
def keys_of(fnname):
    i = s.index("fn %s(" % fnname)
    j = s.index("\n}", i)
    return set(re.findall(r'"([a-z0-9_]+)" =>', s[i:j]))
desc = keys_of("desc_static")
orig = keys_of("orig_val")

cfgk = set()
if os.path.exists(CFG):
    for ln in open(CFG, encoding="utf-8"):
        ln = ln.strip()
        if ln and not ln.startswith("#") and " = " in ln:
            cfgk.add(ln.split(" = ")[0].strip())

U = set(allk)
rep = []
rep.append("\ud0ed %d\uac1c / \ubc30\uce58\ub41c \uace0\uc720\ud0a4 %d / cfg\ud0a4 %d" % (len(tabs), len(U), len(cfgk)))
dup = [k for k, c in collections.Counter(allk).items() if c > 1]
rep.append("\n[\uc911\ubcf5 \ub178\ucd9c %d]" % len(dup))
for k in dup: rep.append("  %-24s %s" % (k, placed[k]))
orphan = sorted(cfgk - U)
rep.append("\n[cfg\uc5d0\ub9cc \uc788\uace0 \ud0ed\uc5d0 \uc5c6\ub294 \ud0a4 %d]" % len(orphan))
for k in orphan: rep.append("  " + k)
ghost = sorted(U - cfgk)
rep.append("\n[\ud0ed\uc5d0\ub9cc \uc788\uace0 cfg\uc5d0 \uc5c6\ub294 \ud0a4 %d]" % len(ghost))
for k in ghost: rep.append("  " + k)
nodesc = sorted(U - desc)
rep.append("\n[\uc124\uba85\uc774 \uc5c6\ub294 \ud0a4 %d]" % len(nodesc))
for k in nodesc: rep.append("  " + k)
noorig = sorted(U - orig)
rep.append("\n[\uc6d0\ubcf8\uac12\uc774 \uc5c6\ub294 \ud0a4 %d]" % len(noorig))
for k in noorig: rep.append("  " + k)

open("tabdump.txt", "w", encoding="utf-8").write("\n".join(lines))
open("audit.txt", "w", encoding="utf-8").write("\n".join(rep))
sys.stdout.reconfigure(encoding="utf-8")
print("\n".join(rep))
