# -*- coding: utf-8 -*-
"""⑤ 설명 '중복 항목' 대응 — 같은 키에 설명이 2개 이상이면 Rust match 는 **앞의 것**만 쓴다.
   앞의 것이 낡아 있으면 뒤를 고쳐봐야 화면엔 낡은 게 나온다(실측: vis_mem_global 143곳).
   ⟹ 사이트 수 정정을 **모든 occurrence** 에 적용하고, 중복 자체를 보고한다."""
import sys, io, re, os, glob, json
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

P = 'C:/tfm2mods/ai_adjust_editor/src/main.rs'
t = io.open(P, encoding='utf-8').read()

SRC = 'C:/tfm2mods/tfm2_ai_adjust/src'
src = ''
for f in sorted(glob.glob(os.path.join(SRC, '*.rs'))):
    src += '\n' + io.open(f, encoding='utf-8').read()
code_sites = {}
for fm in re.finditer(r'\nunsafe fn (\w+)\(\)\s*\{', src):
    s0 = fm.end(); nx = src.find('\nunsafe fn ', s0)
    body = src[s0:nx if nx > 0 else len(src)]
    v2k = dict(re.findall(r'let\s+(\w+)\s*=\s*tune\(\s*"([a-zA-Z0-9_]+)"', body))
    for var, key in v2k.items():
        n = len(re.findall(r'\bb[14]\(\s*%s\s*,' % re.escape(var), body))
        if n:
            code_sites[key] = n
    for tm in re.finditer(r'let v = (?:b4|b1)\((\w+),\s*[0-9_]+\);\s*\n\s*for &\(a, pre, off\) in (\w+)\.iter\(\)', body):
        key = v2k.get(tm.group(1))
        m2 = re.search(r'static %s: \[\(usize, &\[u8\], usize\); (\d+)\]' % tm.group(2), src)
        if key and m2:
            code_sites[key] = int(m2.group(1))

SITEPAT = re.compile(r'([0-9]+)\s*(곳|사이트)')
dups, fixed = [], []
for k, real in sorted(code_sites.items()):
    spans = []
    for m in re.finditer(r'"%s"\s*=>\s*"' % re.escape(k), t):
        s0 = m.end(); j, n = s0, len(t)
        while j < n:
            if t[j] == '\\': j += 2; continue
            if t[j] == '"': break
            j += 1
        if re.search(r'[가-힣]', t[s0:j]):
            spans.append((s0, j))
    if len(spans) > 1:
        dups.append((k, len(spans)))
    hit = False
    for s0, j in reversed(spans):           # 뒤에서부터 고쳐야 인덱스가 안 밀린다
        body = t[s0:j]
        nb = SITEPAT.sub(lambda mm: ('%d%s' % (real, mm.group(2))) if int(mm.group(1)) != real else mm.group(0), body)
        if nb != body:
            t = t[:s0] + nb + t[j:]
            hit = True
    if hit:
        fixed.append((k, real))

io.open(P, 'w', encoding='utf-8', newline='\n').write(t)
print('사이트 수 정정(전 occurrence) %d건' % len(fixed))
for k, r in fixed:
    print('   %-24s -> %d곳' % (k, r))
print('\n*설명 중복 항목(앞의 것만 화면에 나옴) : %d건' % len(dups))
for k, n in dups:
    print('   %-24s %d개' % (k, n))
