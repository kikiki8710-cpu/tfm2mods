# -*- coding: utf-8 -*-
"""**공짜로 뚫리는 노브** 찾기.

바이트패치 노브인데, 재현(judge측) 코드가 이미 같은 계산을 하면서 그 값을 **하드코딩**해 둔 자리가 있으면
그 상수를 tune() 으로 바꾸는 것만으로 클래스별이 열린다(재현 비용 0).

방법: apply_* 에서 `b1(var, ORIG)` / `b4(var, ORIG)` / `tune("k", ORIG)` 로 원본값을 얻고,
      judge 측 파일에서 그 리터럴(10진·16진)을 찾는다."""
import sys, io, re, glob, os
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

SRC = 'C:/tfm2mods/tfm2_ai_adjust/src'
FN = re.compile(r'^\s*(?:pub\s+)?(?:unsafe\s+)?fn\s+([A-Za-z0-9_]+)')
TUNE = re.compile(r'tune\(\s*"([a-zA-Z0-9_]+)"\s*,\s*(-?\d+)')
LET = re.compile(r'let\s+(\w+)\s*=\s*tune\(\s*"([a-zA-Z0-9_]+)"')
ORIG = re.compile(r'\b(?:b1|b4|sq)\(\s*(\w+)\s*,\s*(0x[0-9a-fA-F]+|\d[\d_]*)\s*\)')

imm, judge = set(), set()
knob_orig = {}
judge_lines = []          # (파일, 줄번호, 내용)  judge 측만
for f in sorted(glob.glob(os.path.join(SRC, '*.rs'))):
    cur, var2knob = '?', {}
    for i, ln in enumerate(io.open(f, encoding='utf-8'), 1):
        m = FN.match(ln)
        if m:
            cur = m.group(1); var2knob = {}
        is_apply = cur.startswith('apply_')
        if not ln.lstrip().startswith('//'):
            for k in TUNE.findall(ln):
                (imm if is_apply else judge).add(k[0])
            for k in re.findall(r'tune\(\s*"([a-zA-Z0-9_]+)"', ln):
                if k != 'key':
                    (imm if is_apply else judge).add(k)
        if is_apply:
            for v, k in LET.findall(ln):
                var2knob[v] = k
            for v, o in ORIG.findall(ln):
                if v in var2knob:
                    knob_orig.setdefault(var2knob[v], set()).add(int(o.replace('_', ''), 0))
            for k, o in TUNE.findall(ln):
                if int(o) >= 0:
                    knob_orig.setdefault(k, set()).add(int(o))
        else:
            judge_lines.append((os.path.basename(f), i, ln.rstrip()))

imm_only = imm - judge
hits = []
for k in sorted(imm_only):
    for o in knob_orig.get(k, ()):
        if o < 8:            # 0~7 은 우연 일치가 너무 많다
            continue
        pats = [r'\b%d\b' % o, r'\b0x%x\b' % o, r'\b0X%X\b' % o]
        for fn, i, ln in judge_lines:
            if ln.lstrip().startswith('//'):
                continue
            if any(re.search(p, ln) for p in pats):
                hits.append((k, o, fn, i, ln.strip()[:110]))

seen = set()
print('%-24s %-9s %s' % ('바이트패치 노브', '원본값', '재현측에 같은 상수가 있는 자리'))
n = 0
for k, o, fn, i, ln in hits:
    if (k, fn, i) in seen:
        continue
    seen.add((k, fn, i)); n += 1
    if n > 40:
        continue
    print('%-24s %-9s %s:%d\n    %s' % (k, o, fn, i, ln))
print('\n후보 %d건 (수동 확인 필요 — 숫자 일치는 우연일 수 있다)' % len(seen))
