# 순서도 커버리지 감사: 탭에 배치된 키가 순서도 그룹에 전부 잡히는지, 캐치올로 몇 개가 떨어지는지.
import re, io, sys, os
sys.stdout.reconfigure(encoding="utf-8")
P = os.path.join(os.path.dirname(os.path.abspath(__file__)), "src", "main.rs")
s = io.open(P, encoding="utf-8").read()

STR = re.compile('"([^"\\\\]*(?:\\\\.[^"\\\\]*)*)"')
TAB = re.compile(r'Tab\{ id:"([a-z0-9_]+)", title:"([^"]*)", keys:&\[(.*?)\], note:', re.S)
FG  = re.compile(r'FlowGroup\{ label:"([^"]*)",.*?prefixes:&\[(.*?)\] \}', re.S)

placed = set()
for m in TAB.finditer(s):
    placed.update(k for k in STR.findall(m.group(3)) if not k.startswith("§"))

groups = [(m.group(1), STR.findall(m.group(2))) for m in FG.finditer(s)]

def match(k, pres):
    for p in pres:
        if p == "*": return True
        if p.endswith("_"):
            if k.startswith(p): return True
        elif k == p: return True
    return False

used, byg = set(), {}
for lab, pres in groups:
    got = [k for k in sorted(placed) if k not in used and match(k, pres)]
    used.update(got); byg[lab] = got

print("탭 배치 키 %d / 순서도에 잡힌 키 %d" % (len(placed), len(used)))
left = sorted(placed - used)
print("순서도에 안 잡힌 키: %d개 %s" % (len(left), " ".join(left[:20])))

# 캐치올을 빼면 어디로 떨어지는지
used2 = set()
for lab, pres in groups:
    if pres == ["*"]: continue
    used2.update(k for k in sorted(placed) if k not in used2 and match(k, pres))
rest = sorted(placed - used2)
print("\n캐치올('그 밖의 설정')로 떨어지는 키 %d개:" % len(rest))
print(" ".join(rest))
print("\n빈 그룹(키 0개):")
for lab, got in byg.items():
    if not got: print("  -", lab)
