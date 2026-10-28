# -*- coding: utf-8 -*-
"""레이아웃 .ui 의 노드 경로별 x/y/width/height/anchor/visible 를 뽑는다. 사용: ui_geom.py <ui파일> [경로접두어...] [--depth N]"""
import sys, re
sys.stdout.reconfigure(encoding="utf-8")
NODE = re.compile(r"^\s*#([A-Za-z_][A-Za-z0-9_]*)\s*:\s*([A-Za-z_]+)\s*\{")
PROP = re.compile(r"^\s*(x|y|width|height|anchor_x|anchor_y|pivot_x|pivot_y|visible|child_type)\s*:\s*([^;{]+)")
argv = sys.argv[1:]
depth = 99
if "--depth" in argv:
    i = argv.index("--depth"); depth = int(argv[i + 1]); del argv[i:i + 2]
args = argv
path, prefixes = args[0], args[1:]
stack, props, order, lines = [], {}, [], {}
for n, ln in enumerate(open(path, encoding="utf-8", errors="replace"), 1):
    m = NODE.match(ln)
    first = True
    if not m and stack and stack[-1]:
        p = PROP.match(ln)
        if p:
            props.setdefault("/".join(s for s in stack if s), {}).setdefault(p.group(1), p.group(2).strip())
    for ch in ln:
        if ch == "{":
            name = m.group(1) if (m and first) else ""
            first = False
            stack.append(name)
            if name:
                k = "/".join(s for s in stack if s)
                order.append(k); lines[k] = n
        elif ch == "}" and stack:
            stack.pop()
for k in order:
    if prefixes and not any(k.startswith(p) for p in prefixes):
        continue
    if k.count("/") >= depth:
        continue
    print("%5d %-60s %s" % (lines[k], k, " ".join("%s=%s" % kv for kv in props.get(k, {}).items())))
