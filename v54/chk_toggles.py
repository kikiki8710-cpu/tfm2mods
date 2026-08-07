# -*- coding: utf-8 -*-
"""편집기가 표시하는 **토글 기본값**이 모드의 실제 static 초기값과 맞는지 전수 대조.
   경로: cfg 로더 arm `"키" => { … STATIC.store(…) }` → `static STATIC: AtomicBool = …::new(init)`
   ⚠static 이름이 키와 다른 경우가 많다(`e9jt` → `E9_JT`) — 이름 추측 금지, arm 본문에서 읽는다."""
import sys, io, re, os, glob
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

SRC = 'C:/tfm2mods/tfm2_ai_adjust/src'
src = ''
for f in sorted(glob.glob(os.path.join(SRC, '*.rs'))):
    src += '\n' + io.open(f, encoding='utf-8').read()

ed = io.open('C:/tfm2mods/ai_adjust_editor/src/main.rs', encoding='utf-8').read()

# is_toggle 목록
mt = re.search(r'fn is_toggle\(k: &str\) -> bool \{\s*matches!\(k,(.*?)\)\s*\}', ed, re.S)
toggles = re.findall(r'"([a-z0-9_]+)"', mt.group(1)) if mt else []

# 편집기 orig_val
m = re.search(r'\nfn orig_val\(', ed)
nx = re.search(r'\nfn \w+\(', ed[m.end():])
seg = ed[m.start(): m.end() + (nx.start() if nx else 0)]
ORIG = {}
for mm in re.finditer(r'"([a-z][a-zA-Z0-9_]{2,})"\s*=>\s*"([^"]*)"', seg):
    ORIG.setdefault(mm.group(1), mm.group(2))
for mm in re.finditer(r'((?:"[a-z][a-zA-Z0-9_]{2,}"\s*\|\s*)+"[a-z][a-zA-Z0-9_]{2,}")\s*=>\s*"([^"]*)"', seg):
    for k in re.findall(r'"([^"]+)"', mm.group(1)):
        ORIG.setdefault(k, mm.group(2))

print('%-18s %-16s %-8s %-10s %s' % ('키', 'static', '초기값', '실제기본', '편집기표시'))
bad = []
for k in toggles:
    mm = re.search(r'"%s"\s*(?:\|[^=]*)?=>\s*\{(?=(.{0,300}))' % re.escape(k), src, re.S)
    stat, init, real = '(arm 없음)', '', '?'
    if mm:
        ms = re.search(r'\b([A-Z][A-Z0-9_]{2,})\s*\.\s*store', mm.group(1))
        if ms:
            stat = ms.group(1)
            md = re.search(r'static %s\s*:\s*Atomic\w+\s*=\s*Atomic\w+::new\(([^)]+)\)' % stat, src)
            if md:
                init = md.group(1).strip()
                real = '켜짐' if init == 'true' else ('꺼짐' if init in ('false', '0') else init)
    shown = ORIG.get(k, '(없음)')
    flag = ''
    if real != '?' and shown != real:
        flag = '  ← ★불일치'
        bad.append((k, real, shown))
    print('%-18s %-16s %-8s %-10s %s%s' % (k, stat, init, real, shown, flag))

print('\n불일치 %d건' % len(bad))
for k, r, s in bad:
    print('   %-18s 실제 %s / 편집기 %s' % (k, r, s))
