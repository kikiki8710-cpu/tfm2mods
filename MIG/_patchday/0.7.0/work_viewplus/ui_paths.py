# -*- coding: utf-8 -*-
"""레이아웃 .ui 의 노드 경로(#a/#b/#c)를 뽑아 새 판과 옛 판·오버라이드를 경로 단위로 비교한다."""
import sys, os, re
sys.stdout.reconfigure(encoding="utf-8")
HERE = os.path.dirname(os.path.abspath(__file__))
NODE = re.compile(r"^\s*#([A-Za-z_][A-Za-z0-9_]*)\s*:\s*([A-Za-z_]+)\s*\{")

def paths(p):
    # 스택 원소 = 노드 이름(노드 블록) 또는 "" (속성 블록 { })
    out, stack = {}, []
    for ln in open(p, encoding="utf-8", errors="replace"):
        m = NODE.match(ln)
        first = True
        for ch in ln:
            if ch == "{":
                name = m.group(1) if (m and first) else ""
                first = False
                stack.append(name)
                if name:
                    out["/".join(s for s in stack if s)] = m.group(2)
            elif ch == "}" and stack:
                stack.pop()
    return out

def clean(d):
    return d

base = sys.argv[1] if len(sys.argv) > 1 else "layout"
old = clean(paths(os.path.join(HERE, "bundle_ui_0.6.5", "ui", "layout", "banpick", base + ".ui")))
ovr = clean(paths(os.path.join(r"F:\claude\tfm2mods\banpick_view_plus\ui\layout\banpick", base + ".ui")))
for suf in sys.argv[2:] or ["_c", "_d"]:
    new = clean(paths(os.path.join(HERE, "bundle_ui_0.7.0", "ui", "layout", "banpick", base + suf + ".ui")))
    print("=====", base + suf, "nodes", len(new))
    print("-- 0.6.5 에 없던 경로:")
    for k in sorted(set(new) - set(old)):
        print("  +", k, ":", new[k], "" if k in ovr else "   [오버라이드에 없음]")
    print("-- 0.7.0 에서 사라진 경로:")
    for k in sorted(set(old) - set(new)):
        print("  -", k)
