"""bpcatalog.py — 밴픽 관련 소스 파일에 속한 IR define(함수) 전수 카탈로그(파일·줄·define 위치·크기). python bpcatalog.py [--out <json>] [--files <regex>]

2026-09-15 신설(champ_pos_lock 밴픽 정적 전수조사). _gcbc/_gvbc/_gaibc 의 !DISubprogram 을 파일 정규식으로 걸러
define 본문 위치(모듈·시작줄·끝줄·IR 줄수)와 함께 JSON 으로 낸다. 후속 = bpdump.py(irann 일괄 덤프).
"""
import re, sys, json, os, io, glob

ROOTS = [r"C:\tfm2mods\_gcbc", r"C:\tfm2mods\_gvbc", r"C:\tfm2mods\_gaibc"]
DEFAULT_FILES = r"(game-core\\\\src\\\\banpick\\\\|game-core\\\\src\\\\data\\\\coach\.rs|game-view\\\\src\\\\logic\\\\server|game-view\\\\src\\\\ui\\\\match_ui|game-view\\\\src\\\\ui\\\\today_match_ui|banpick_illustration|src\\\\worker\\\\)"

def main():
    out = None; files_re = DEFAULT_FILES
    a = sys.argv[1:]
    while a:
        if a[0] == "--out": out = a[1]; a = a[2:]
        elif a[0] == "--files": files_re = a[1]; a = a[2:]
        else: a = a[1:]
    frx = re.compile(files_re)
    cat = []
    for root in ROOTS:
        for ll in sorted(glob.glob(os.path.join(root, "*.ll"))):
            with io.open(ll, encoding="utf-8", errors="replace") as f:
                lines = f.readlines()
            # DIFile map
            difile = {}
            for ln in lines:
                if ln.startswith("!") and "!DIFile(" in ln:
                    m = re.match(r'!(\d+) = !DIFile\(filename: "([^"]*)"', ln)
                    if m: difile[m.group(1)] = m.group(2)
            # DISubprogram: name, linkageName, file, line
            subp = {}
            for ln in lines:
                if ln.startswith("!") and "!DISubprogram(" in ln:
                    m = re.match(r'!(\d+) = distinct !DISubprogram\(name: "([^"]*)"(?:, linkageName: "([^"]*)")?.*?file: !(\d+), line: (\d+)', ln)
                    if m:
                        subp[m.group(1)] = (m.group(2), m.group(3), difile.get(m.group(4), "?"), int(m.group(5)))
            # defines
            i = 0; n = len(lines)
            while i < n:
                ln = lines[i]
                if ln.startswith("define "):
                    m = re.search(r'@("?[^"(\s]+"?)\(', ln)
                    sym = m.group(1) if m else "?"
                    md = re.search(r'!dbg !(\d+)', ln)
                    j = i + 1
                    while j < n and not lines[j].startswith("}"):
                        j += 1
                    if md and md.group(1) in subp:
                        name, link, file, sline = subp[md.group(1)]
                        if frx.search(file):
                            cat.append({"module": os.path.basename(ll), "root": os.path.basename(root),
                                        "sym": sym, "name": name, "file": file, "src_line": sline,
                                        "ll_start": i + 1, "ll_end": j + 1, "ll_lines": j - i + 1})
                    i = j
                i += 1
    cat.sort(key=lambda c: (c["file"], c["src_line"]))
    if out:
        io.open(out, "w", encoding="utf-8").write(json.dumps(cat, ensure_ascii=False, indent=1))
    by = {}
    for c in cat: by.setdefault(c["file"], []).append(c)
    for file, cs in by.items():
        print(f"{file}  ({len(cs)} fn, {sum(c['ll_lines'] for c in cs)} IR lines)")
    print("TOTAL", len(cat), "fn", sum(c["ll_lines"] for c in cat), "IR lines")

if __name__ == "__main__":
    main()
