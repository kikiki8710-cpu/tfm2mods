# -*- coding: utf-8 -*-
"""④ 감사에서 확인된 편집기 설명 오류를 수정한다.
   (1) 원본값이 코드와 다른 것  (2) 사이트 수가 코드와 다른 것  (3) 설명 자체가 없는 것
   ★설명이 틀리면 사용자가 값을 잘못 넣는다 = 노브가 '의도한 동작'을 못 하는 것과 같다."""
import sys, io, re, os, glob, json
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

P = 'C:/tfm2mods/ai_adjust_editor/src/main.rs'
t = io.open(P, encoding='utf-8').read()

# ── 코드 실제값 재계산(audit2 와 동일 로직) ──────────────────
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

# ── (1) 원본값 정정 ─────────────────────────────────────────
n1 = 0
old = '"ldsc_early_mask" => "조기 반환 태그 묶음(원본 128611)'
if old in t:
    t = t.replace(old, '"ldsc_early_mask" => "조기 반환 태그 묶음(원본 129123 — 0.5.4에서 비트 9가 추가돼 128611에서 바뀌었습니다)', 1)
    n1 += 1

# ── (2) 사이트 수 정정 ──────────────────────────────────────
SITEPAT = re.compile(r'([0-9]+)\s*(곳|사이트)')
n2, changed = 0, []
for k, real in sorted(code_sites.items()):
    m = re.search(r'"%s"\s*=>\s*"' % re.escape(k), t)
    if not m:
        continue
    s0 = m.end()
    # 문자열 끝 찾기(이스케이프 고려)
    j, n = s0, len(t)
    while j < n:
        if t[j] == '\\': j += 2; continue
        if t[j] == '"': break
        j += 1
    body = t[s0:j]
    if not re.search(r'[가-힣]', body):
        continue
    def rep(mm):
        global n2
        if int(mm.group(1)) != real:
            return '%d%s' % (real, mm.group(2))
        return mm.group(0)
    nb = SITEPAT.sub(rep, body)
    if nb != body:
        changed.append((k, body[:0], real))
        t = t[:s0] + nb + t[j:]
        n2 += 1

# ── (3) 없는 설명 추가 ──────────────────────────────────────
ADD = {
 "pe_kind_mask": "자리 판단에서 <b>어떤 종류의 이득을 셈에 넣을지</b> 고르는 비트마스크. 비트를 빼면 그 종류(예: 구조물 압박, 아군 보호)를 자리 선택에서 아예 무시합니다. ⚠종류별 비트 대응은 아직 확정되지 않았습니다 — 한 비트씩 바꿔가며 확인하세요. -1=원본",
 "pe_mode_mask": "자리 판단을 <b>어떤 팀 작전에서 적용할지</b> 고르는 비트마스크. 비트를 빼면 그 작전 중에는 자리 재평가를 하지 않고 원래 위치를 지킵니다. ⚠작전별 비트 대응 미확정. -1=원본",
}
n3 = 0
anc = ' "pl_obj_role" => "에픽'
if anc in t:
    i = t.index(anc)
    ins = ''.join(' "%s" => "%s",\n' % (k, v) for k, v in ADD.items() if '"%s" =>' % k not in t or True)
    add = ''.join(' "%s" => "%s",\n' % (k, v) for k, v in ADD.items()
                  if not re.search(r'"%s"\s*=>\s*"[^"]*[가-힣]' % k, t))
    if add:
        t = t[:i] + add + t[i:]
        n3 = add.count('=>')

io.open(P, 'w', encoding='utf-8', newline='\n').write(t)
print('원본값 정정 %d건 / 사이트 수 정정 %d건 / 설명 신규 %d건' % (n1, n2, n3))
for k, _, r in changed:
    print('   %-24s -> %d곳' % (k, r))
