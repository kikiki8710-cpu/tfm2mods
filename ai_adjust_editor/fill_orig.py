# detour.rs 에서 각 노브의 "원본값"을 자동 회수해 편집기 orig_val() 의 빈칸을 채운다.
#  회수 규칙 2가지
#   ① tune("k", D) 의 D 가 -1 이 아니면 **D 가 곧 원본값**이다(모드 규약).
#   ② D 가 -1(=원본유지 센티널)이면 같은 줄 주석의 `원본 N` 을 읽는다.
#  둘 다 없으면 건드리지 않는다(추측 금지).
import re, os, sys, glob
sys.stdout.reconfigure(encoding="utf-8")

SRCDIR = r"C:\tfm2mods\tfm2_ai_adjust\src"
MAIN = os.path.join(os.path.dirname(os.path.abspath(__file__)), "src", "main.rs")
d = "\n".join(open(f, encoding="utf-8", errors="replace").read()
              for f in sorted(glob.glob(os.path.join(SRCDIR, "*.rs"))))
s = open(MAIN, encoding="utf-8").read()

i = s.index("fn orig_val("); j = s.index("\n}", i)
have = set(re.findall(r'"([a-z0-9_]+)" =>', s[i:j]))

STR = re.compile('"([^"\\\\]*(?:\\\\.[^"\\\\]*)*)"')
TAB = re.compile(r'Tab\{ id:"([a-z0-9_]+)", title:"([^"]*)", keys:&\[(.*?)\], note:', re.S)
placed = set()
for m in TAB.finditer(s):
    placed.update(k for k in STR.findall(m.group(3)) if not k.startswith("\u00a7"))

got = {}
src = {}
for m in re.finditer(r'tune\(\s*"([a-z0-9_]+)"\s*,\s*(-?0[xX][0-9a-fA-F]+|-?\d+)\s*\)', d):
    k, v = m.group(1), m.group(2)
    if k in got: continue
    if v != "-1":
        neg = v.startswith("-")
        vv = v.lstrip("-")
        n = int(vv, 16) if vv.lower().startswith("0x") else int(vv)
        got[k] = str(-n if neg else n); src[k] = "default"
    else:
        ls = d.rfind("\n", 0, m.start()) + 1
        le = d.find("\n", m.end())
        c = re.search(r'\uc6d0\ubcf8(?:\uac12)?\s*[:=]?\s*(0[xX][0-9a-fA-F]+|[-\u2212]?\d+)', d[ls:le])
        if c:
            v2 = c.group(1).replace("\u2212", "-")
            neg = v2.startswith("-"); v2 = v2.lstrip("-")
            n = int(v2, 16) if v2.lower().startswith("0x") else int(v2)
            got[k] = str(-n if neg else n); src[k] = "comment"

add = {k: v for k, v in got.items() if k in placed and k not in have}
rest = sorted(placed - have - set(add))
print("\ud68c\uc218 %d\ud0a4(default %d / comment %d) \u2192 \ucd94\uac00 \ub300\uc0c1 %d\ud0a4"
      % (len(got), sum(1 for k in got if src[k] == "default"),
         sum(1 for k in got if src[k] == "comment"), len(add)))
print("\n\uc5ec\uc804\ud788 \uc6d0\ubcf8\uac12 \uc5c6\ub294 \ud0a4 %d\uac1c:" % len(rest))
print(" ".join(rest))

if "--write" in sys.argv and add:
    ent = "\n".join('    "%s" => "%s",' % (k, add[k]) for k in sorted(add))
    anchor = 'fn orig_val(k: &str) -> Option<&\'static str> {\n  Some(match k {\n'
    s = s.replace(anchor, anchor
                  + "    // \u2500\u2500 detour.rs \uc5d0\uc11c \uc790\ub3d9 \ud68c\uc218(2026-08-04 \uac10\uc0ac) \u2500\u2500\n"
                  + ent + "\n", 1)
    open(MAIN, "w", encoding="utf-8").write(s)
    print("\norig_val \uc5d0 %d\ud0a4 \ucd94\uac00" % len(add))
