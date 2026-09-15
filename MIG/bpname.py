"""bpname.py — exe RVA 들의 패닉 Location 지문을 bpcatalog.json(밴픽 IR 카탈로그·_gcbc/_gvbc 포함)의 함수 소스 범위와 대조해 실명을 판정한다. python bpname.py <catalog.json> <rva>...

2026-09-15 신설. rvaname.py 는 _gaibc 만 보므로 game_core/game_view 밴픽 함수는 「클로저만」으로 나온다 — 그 보완.
판정 = Location(파일:줄)이 카탈로그 함수의 [src_line, 같은 파일 다음 함수 src_line) 안에 들면 후보. 다수 Location 이 같은 함수를 가리키면 확정.
"""
import io, os, re, sys, json, subprocess, collections
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
HERE = os.path.dirname(os.path.abspath(__file__))

def locs(rva):
    r = subprocess.run([sys.executable, "-X", "utf8", os.path.join(HERE, "fnprobe.py"), rva], capture_output=True, text=True, encoding="utf-8", errors="replace")
    seen = []
    for m in re.finditer(r"((?:game-\w+|src)\\[^\s:]+?):(\d+):(\d+)", r.stdout):
        t = (m.group(1), int(m.group(2)))
        if t not in seen: seen.append(t)
    size = re.search(r"size[=: ]+(\d+)", r.stdout)
    return seen, (size.group(1) if size else "?")

def main():
    cat = json.load(io.open(sys.argv[1], encoding="utf-8"))
    byfile = collections.defaultdict(list)
    for c in cat:
        byfile[c["file"].replace("\\\\", "\\")].append(c)
    for f in byfile: byfile[f].sort(key=lambda c: c["src_line"])
    for rva in sys.argv[2:]:
        ls, size = locs(rva)
        votes = collections.Counter(); detail = {}
        for file, line in ls:
            fns = byfile.get(file)
            if not fns: continue
            # 그 줄을 포함하는 함수 = src_line <= line 인 것 중 가장 큰 src_line (같은 파일)
            cand = [c for c in fns if c["src_line"] <= line]
            if not cand: continue
            best = max(cand, key=lambda c: c["src_line"])
            # 같은 src_line 을 가진 인스턴스가 여럿이면 본체(이름에 closure 없는 것) 우선
            same = [c for c in fns if c["src_line"] == best["src_line"]]
            body = [c for c in same if "closure" not in c["name"] and "{{" not in c["name"]] or same
            key = (body[0]["file"], body[0]["src_line"], body[0]["name"])
            votes[key] += 1
            detail[key] = body[0]
        print(f"■ {rva} size={size} Location {len(ls)}개: " + " · ".join(f"{os.path.basename(f)}:{l}" for f, l in ls[:5]))
        if not votes:
            print("   지문 없음/카탈로그 밖(exe 전용 또는 비밴픽 파일)")
            continue
        for (file, sl, name), n in votes.most_common(4):
            c = detail[(file, sl, name)]
            print(f"   [{n}표] {name}  {os.path.basename(file)}:{sl}  ir={c['root']}/{c['module']}:{c['ll_start']}..{c['ll_end']} ({c['ll_lines']} lines)")

if __name__ == "__main__":
    main()
