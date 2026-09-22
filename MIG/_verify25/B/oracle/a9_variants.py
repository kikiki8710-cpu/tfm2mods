"""for each SmallActionPlay-producing callee: collect `store i8 <c>` to gep +177 (tag) in body (depth-2 into game_ai callees), and the constructor initializes ranges."""
import io, re, json, os, collections
idx = json.load(io.open(r"C:\tfm2mods\MIG\_next\reach\defidx.json", encoding="utf-8"))
CORP = r"C:\tfm2mods\_gaibc"
files = {}
def body(sym):
    fn, s, t = idx[sym]
    if fn not in files: files[fn] = io.open(os.path.join(CORP, fn), encoding="utf-8", errors="replace").read().split("\n")
    return files[fn][s-1:t], fn, s
CALLRE = re.compile(r"(?:call|invoke)\s+[^@]*@([\w\.\$]+)\(")
TAGS = {3:"RunAway",4:"Recall",5:"Around",6:"AroundHide",7:"AroundRegion",8:"AroundRunAway",9:"Positioning",11:"AroundPositionBush",12:"AroundBush",13:"LaneMinionPosition",14:"Trace",15:"Attack",16:"Skill",17:"Skill2",18:"Ult",19:"Stop",-1:"None"}
def tagstores(sym, depth=0, seen=None):
    seen = seen if seen is not None else set()
    if sym in seen or sym not in idx: return {}
    seen.add(sym)
    b, fn, s = body(sym)
    out = collections.defaultdict(list)
    geps = {}
    for i, l in enumerate(b):
        m = re.match(r"\s*(%\d+) = getelementptr inbounds nuw i8, ptr (%\d+), i64 177\b", l)
        if m: geps[m.group(1)] = m.group(2)
        m = re.match(r"\s*store i8 (-?\d+), ptr (%\d+)", l)
        if m and m.group(2) in geps:
            out[int(m.group(1))].append("%s:%d" % (fn, s + i))
        # inits
    if depth < 2:
        for l in b:
            m = CALLRE.search(l)
            if m:
                c = m.group(1)
                if c in idx and ("SmallAction" in c or "action" in c) and "drop" not in c and "Clone" not in c:
                    for k, v in tagstores(c, depth + 1, seen).items():
                        out[k].extend(v)
    return out
names = ["11lane_minion27lane_minion_position_action","11fight_check13battle_action","11fight_check18battle_ally_action","11lane_minion29line_minion_action_candidates","11fight_check20attack_summon_action","11fight_check29attack_structure_skill_action"]
for n in names:
    for k in [s for s in idx if s.endswith(n)]:
        ts = tagstores(k)
        print("==", n)
        for t in sorted(ts): print("   tag", t, TAGS.get(t, "?"), "x%d" % len(ts[t]), ts[t][:3])
# constructors: initializes of every SmallAction*::new* in game_ai
print("== constructors initializes ==")
for k in sorted(idx):
    if re.search(r"12small_action.*(3new|new_with_skill|new_keep_range|\d+new_[a-z_]+)$", k):
        b, fn, s = body(k)
        m = re.search(r"sret\(\[(\d+) x i8\]\)[^%]*?(initializes\(\([^%]*)?%0", b[0])
        if m: print("  ", re.sub(r".*7game_ai12small_action", "", k)[:70], m.group(1)+"B", (m.group(2) or "").strip())
