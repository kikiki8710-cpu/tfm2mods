# -*- coding: utf-8 -*-
"""③ 순서도(FLOW) 감사 — 순서도가 가리키는 키가 실재/배선/설명을 갖는지, 탭과 커버리지가 맞는지."""
import sys, io, re, json
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

ed = io.open('C:/tfm2mods/ai_adjust_editor/src/main.rs', encoding='utf-8').read()
st = json.load(io.open('C:/tfm2mods/v54/audit_state.json', encoding='utf-8'))
wired, in_tab, desc = set(st['wired']), set(st['in_tab']), st['desc']

m = re.search(r'(?:static|const)\s+FLOW\s*:\s*[^=]+=\s*&?\[', ed)
if not m:
    print('FLOW 정의를 못 찾음'); sys.exit(1)
i = ed.index('[', m.end() - 1)
d, j = 0, i
while j < len(ed):
    if ed[j] == '[': d += 1
    elif ed[j] == ']':
        d -= 1
        if d == 0: break
    j += 1
body = ed[i:j + 1]

KEY = re.compile(r'^[a-z][a-zA-Z0-9_]{2,}$')


def scan(s):
    out, k, n = [], 0, len(s)
    while k < n:
        if s[k] == '"':
            p, buf = k + 1, []
            while p < n:
                if s[p] == '\\': buf.append(s[p:p + 2]); p += 2; continue
                if s[p] == '"': break
                buf.append(s[p]); p += 1
            out.append(''.join(buf)); k = p + 1
        elif s[k] == '/' and k + 1 < n and s[k + 1] == '/':
            while k < n and s[k] != '\n': k += 1
        else:
            k += 1
    return out


flow_keys = {x for x in scan(body) if KEY.match(x)}
# 순서도 문자열엔 라벨(한글)도 섞이므로, 배선키/탭키/설명키 중 하나라도 해당되면 '키'로 본다
known = wired | in_tab | set(desc)
flow_keys = {x for x in flow_keys if x in known or re.match(r'^[a-z0-9]+_[a-z0-9_]+$', x)}

print('순서도가 참조하는 키 = %d' % len(flow_keys))


def show(t, s, lim=48):
    s = sorted(s)
    print('\n= %s : %d건' % (t, len(s)))
    for k in range(0, min(len(s), lim), 6):
        print('   ' + '  '.join(s[k:k + 6]))
    if len(s) > lim: print('   ... 외 %d건' % (len(s) - lim))


# FLOW 는 접두어 항목(`ex_attack_` 처럼 _ 로 끝남)으로 묶음을 가리킨다 — 접두어 매칭으로 커버리지를 센다.
pref = {x for x in flow_keys if x.endswith('_')}
exact = flow_keys - pref
print('  (정확키 %d + 접두어 %d) 접두어: %s' % (len(exact), len(pref), ' '.join(sorted(pref))))


def covered(k):
    return k in exact or any(k.startswith(p) for p in pref)


show('*순서도의 정확키 중 배선 안 됨(순서도가 거짓말)', exact - wired)
show('*순서도의 접두어 중 해당 노브가 하나도 없음(死 묶음)',
     {p for p in pref if not any(k.startswith(p) for k in wired)})
show('탭에 있는데 순서도가 전혀 안 다룸', {k for k in in_tab if not covered(k)})
show('배선됐는데 순서도·탭 둘 다 없음(완전 은닉)',
     {k for k in wired if k not in in_tab and not covered(k)})
