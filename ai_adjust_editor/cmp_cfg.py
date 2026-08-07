# 두 설정 프리셋 비교. 값이 다른 키만 뽑고, 편집기 설명을 붙여 사람이 읽을 수 있게 만든다.
import re, io, sys, os
sys.stdout.reconfigure(encoding="utf-8")

CD = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\mods\tfm2_ai_adjust\config"
A, B = sys.argv[1], sys.argv[2]

def load(p):
    d = {}
    for ln in io.open(os.path.join(CD, p), encoding="utf-8", errors="replace"):
        ln = ln.strip()
        if not ln or ln.startswith("#"): continue
        if " = " in ln:
            k, v = ln.split(" = ", 1); d[k.strip()] = v.strip()
        elif "=" in ln:
            k, v = ln.split("=", 1); d[k.strip()] = v.strip()
    return d

a, b = load(A), load(B)

# 편집기 설명·원본값
src = io.open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "src", "main.rs"), encoding="utf-8").read()
def keys_of(fn):
    i = src.index("fn %s(" % fn); j = src.index("\n}", i)
    return dict(re.findall(r'"([a-z0-9_]+)" => "((?:[^"\\]|\\.)*)"', src[i:j]))
DESC, ORIG = keys_of("desc_static"), keys_of("orig_val")
def plain(t):
    t = re.sub(r'<[^>]+>', '', t or '')
    t = re.sub(r'【[^】]*】\s*', '', t)
    t = re.sub(r'\s+', ' ', t)
    return t.strip()

both = sorted(set(a) & set(b))
diff = [k for k in both if a[k] != b[k]]
onlyA = sorted(set(a) - set(b))
onlyB = sorted(set(b) - set(a))

print("=== %s  vs  %s ===" % (A, B))
print("키 수: %s=%d  %s=%d  공통=%d" % (A, len(a), B, len(b), len(both)))
print("값이 다른 키: %d개 / %s에만: %d / %s에만: %d\n" % (len(diff), A, len(onlyA), B, len(onlyB)))

def num(x):
    try: return float(x)
    except: return None

print("%-24s %12s %12s %10s  %s" % ("키", A, B, "원본", "설명"))
print("-" * 130)
for k in diff:
    va, vb = a[k], b[k]
    o = ORIG.get(k, "—")
    na, nb = num(va), num(vb)
    tag = ""
    if na is not None and nb is not None and na != 0:
        r = nb / na
        if r >= 2 or r <= 0.5: tag = " ★%.1f배" % r
    print("%-24s %12s %12s %10s  %s%s" % (k, va, vb, o, plain(DESC.get(k, ""))[:80], tag))

if onlyA:
    print("\n[%s 에만 있는 키] %s" % (A, " ".join(onlyA)))
if onlyB:
    print("\n[%s 에만 있는 키] %s" % (B, " ".join(onlyB)))
