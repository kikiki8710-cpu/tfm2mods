# -*- coding: utf-8 -*-
"""순서도 '그 밖의 설정'(catch-all)에 떨어지는 키를 뽑아 접두어별로 묶는다."""
import sys, io, re, json
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

ed = io.open('C:/tfm2mods/ai_adjust_editor/src/main.rs', encoding='utf-8').read()
st = json.load(io.open('C:/tfm2mods/v54/audit_state.json', encoding='utf-8'))
in_tab = set(st['in_tab'])

m = re.search(r'static FLOW: &\[FlowNode\] = &\[', ed)
i = ed.index('[', m.end() - 1)
d, j = 0, i
while j < len(ed):
    if ed[j] == '[': d += 1
    elif ed[j] == ']':
        d -= 1
        if d == 0: break
    j += 1
body = ed[i:j + 1]

# 캐치올 노드는 마지막 FlowNode. 그 앞까지의 prefixes 만 '배정된 것'으로 본다.
cut = body.rindex('FlowNode{ no:"—"')
assigned = set(re.findall(r'"([^"]+)"', ' '.join(re.findall(r'prefixes:&\[([^\]]*)\]', body[:cut]))))
print('배정된 접두어/키 = %d' % len(assigned))


def covered(k):
    return any(k == a or (a.endswith('_') and k.startswith(a)) for a in assigned)


miss = sorted(k for k in in_tab if not covered(k))
print('캐치올로 떨어지는 키 = %d\n' % len(miss))

groups = {}
for k in miss:
    p = k.split('_')[0] if '_' in k else k
    groups.setdefault(p, []).append(k)
for p, v in sorted(groups.items(), key=lambda x: -len(x[1])):
    print('  %-10s %2d  %s' % (p, len(v), ' '.join(v)[:110]))
