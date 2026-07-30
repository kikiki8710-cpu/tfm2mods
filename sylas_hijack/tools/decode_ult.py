# effect_dump.txt CUR 블록(궁 실제 실행 effect 트리) → schema로 타입 붙여 디코드
# 사용: python decode_ult.py [champ]   (생략=전체 요약)
import re, json, sys, io
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8", errors="replace")
DUMP   = r"C:\tfm2mods\sylas_hijack\effect_dump.txt"
SCHEMA = r"C:\tfm2mods\sylas_hijack\tools\effect_vtable_schema.json"
SHEET  = r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\bundle_unpacked_full\setting\champion_info.champion_info_sheet"

SCH = json.load(open(SCHEMA, encoding="utf-8"))["vtables"]
# 비-effect(트리에서 스킵/표시): item info / stat runner / no-op / code
NONEFFECT = {"BaseItemInfo", "EntityStatBuffPassiveRunner", "?"}

def ty(vt):
    return SCH.get(vt) or SCH.get(hex(int(vt, 16)))

class Node:
    __slots__ = ("depth", "vt", "sz", "d", "words", "strs", "kids")
    def __init__(s, depth, vt, sz, d, words):
        s.depth = depth; s.vt = vt; s.sz = sz; s.d = d; s.words = words; s.strs = {}; s.kids = []

def parse_cur():
    """champ -> [root Node,...] (가장 노드 많은 CUR 블록)."""
    txt = open(DUMP, encoding="utf-8", errors="replace").read()
    blocks = re.split(r"(?=^===== )", txt, flags=re.M)
    per = {}  # champ -> (nodecount, [roots])
    for b in blocks:
        m = re.match(r"===== (\w+) CUR ", b)
        if not m: continue
        champ = m.group(1)
        roots = []; stack = []
        for line in b.splitlines():
            nm = re.match(r"( *)vt=RVA:(0x[0-9a-f]+) sz=(0x[0-9a-f]+) d=(0x[0-9a-f]+) \[(.*)\]", line)
            if nm:
                indent = len(nm.group(1)); depth = indent // 2 - 1
                words = [int(x, 16) for x in nm.group(5).split()] if nm.group(5).strip() else []
                node = Node(depth, nm.group(2), nm.group(3), nm.group(4), words)
                while stack and stack[-1][0] >= depth: stack.pop()
                if stack: stack[-1][1].kids.append(node)
                else: roots.append(node)
                stack.append((depth, node)); continue
            sm = re.match(r' *s@\+(0x[0-9a-f]+)="(.*)"', line)
            if sm and stack: stack[-1][1].strs[sm.group(1)] = sm.group(2)
        nc = sum(1 for _ in walk(roots))
        if champ not in per or nc > per[champ][0]:
            per[champ] = (nc, roots)
    return {c: v[1] for c, v in per.items()}

def walk(roots):
    for r in roots:
        yield r
        yield from walk(r.kids)

def sheet_ult(name):
    try: return json.load(open(SHEET, encoding="utf-8")).get(name, {}).get("ult", {})
    except Exception: return {}

def render(n, ind=0):
    e = ty(n.vt); pad = "  " * ind
    strs = ("  " + " ".join(f'"{v}"' for v in n.strs.values())) if n.strs else ""
    if e:
        t = e.get("type", "?"); disc = e.get("disc")
        tag = f"{t}" + (f"#{disc}" if disc is not None else "")
        skip = t in NONEFFECT
        mark = " [비-effect]" if skip else ""
        print(f"{pad}{tag}{mark}  (vt {n.vt}){strs}")
        fields = e.get("fields") or []
        vs = e.get("val_start")
        if fields and vs and not skip:
            start = int(vs, 16) // 8
            vals = [(fn, n.words[start + i]) for i, fn in enumerate(fields) if start + i < len(n.words)]
            shown = [f"{fn}={v}" for fn, v in vals if v != 0]
            if shown: print(f"{pad}    {' '.join(shown)}")
        if skip: return  # 비-effect는 자식 안 봄
    else:
        print(f"{pad}?{n.vt} sz{n.sz} w={[hex(w) for w in n.words[:6]]}{strs}")
    for k in n.kids: render(k, ind + 1)

def main():
    data = parse_cur()
    tgt = sys.argv[1] if len(sys.argv) > 1 else None
    if not tgt:
        print(f"CUR 캡처 {len(data)}챔프. (노드수 / 미상노드)")
        for c in sorted(data):
            roots = data[c]; tot = sum(1 for _ in walk(roots))
            unk = sum(1 for n in walk(roots) if not ty(n.vt))
            print(f"  {c}: {tot} (미상 {unk})")
        return
    if tgt not in data: print(f"[{tgt}] CUR 없음"); return
    print(f"===== {tgt} =====")
    print(f"시트 ult: {json.dumps(sheet_ult(tgt), ensure_ascii=False)}\n")
    for r in data[tgt]: render(r)

if __name__ == "__main__": main()
