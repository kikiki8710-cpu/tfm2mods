# -*- coding: utf-8 -*-
"""② 편집기 설명이 말하는 '원본 N' / 'N곳 동시' 가 코드 실제와 맞는지 대조.
   - 코드 실제 원본값 = b1(var, ORIG) / b4(var, ORIG) 의 ORIG
   - 코드 실제 사이트 수 = 그 변수가 쓰인 p! 호출 수 (+ 표루프는 표 길이)
   설명이 틀리면 사용자가 값을 잘못 넣는다 = '의도한 동작'이 깨지는 실질 결함.
"""
import sys, io, re, os, glob, json
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

SRC = 'C:/tfm2mods/tfm2_ai_adjust/src'
st = json.load(io.open('C:/tfm2mods/v54/audit_state.json', encoding='utf-8'))
desc = st['desc']

src = ''
for f in sorted(glob.glob(os.path.join(SRC, '*.rs'))):
    src += '\n' + io.open(f, encoding='utf-8').read()

# 함수 단위로 잘라, 그 안에서 let VAR = tune("key", ...) 매핑 → b1/b4(VAR, ORIG) 수집
code_orig, code_sites = {}, {}
for fm in re.finditer(r'\nunsafe fn (\w+)\(\)\s*\{', src):
    start = fm.end()
    nxt = src.find('\nunsafe fn ', start)
    body = src[start:nxt if nxt > 0 else len(src)]
    var2key = dict(re.findall(r'let\s+(\w+)\s*=\s*tune\(\s*"([a-zA-Z0-9_]+)"', body))
    for var, key in var2key.items():
        origs = re.findall(r'\bb[14]\(\s*%s\s*,\s*([0-9_]+|0x[0-9a-fA-F_]+)\s*\)' % re.escape(var), body)
        if origs:
            vals = set()
            for o in origs:
                o = o.replace('_', '')
                vals.add(int(o, 16) if o.lower().startswith('0x') else int(o))
            code_orig[key] = vals
            code_sites[key] = len(origs)
    # 표+루프 형태: for &(a,pre,off) in TABLE.iter() { ... patch_imm_bytes(...) } 는 표 길이로
    for tm in re.finditer(r'let v = (?:b4|b1)\((\w+),\s*([0-9_]+)\);\s*\n\s*for &\(a, pre, off\) in (\w+)\.iter\(\)', body):
        key = var2key.get(tm.group(1))
        if not key:
            continue
        m2 = re.search(r'static %s: \[\(usize, &\[u8\], usize\); (\d+)\]' % tm.group(3), src)
        code_orig[key] = {int(tm.group(2).replace('_', ''))}
        code_sites[key] = int(m2.group(1)) if m2 else -1

print('코드에서 원본값을 읽어낸 노브 = %d' % len(code_orig))

DIG = re.compile(r'원본\s*(?:값\s*)?(0[xX][0-9a-fA-F]+|[0-9][0-9,]*)')
SITE = re.compile(r'([0-9]+)\s*곳')
bad_val, bad_site, nomention = [], [], []
for k, v in sorted(desc.items()):
    if k not in code_orig:
        continue
    m = DIG.search(v)
    if m:
        raw = m.group(1).replace(',', '')
        said = int(raw, 16) if raw.lower().startswith('0x') else int(raw)
        if said not in code_orig[k]:
            bad_val.append((k, said, sorted(code_orig[k])))
    else:
        nomention.append(k)
    ms = SITE.search(v)
    if ms:
        said_n = int(ms.group(1))
        if code_sites.get(k, 0) not in (said_n, -1):
            bad_site.append((k, said_n, code_sites.get(k)))

print('\n= *설명의 원본값이 코드와 다름 : %d건' % len(bad_val))
for k, s, c in bad_val:
    print('   %-26s 설명=%s  코드=%s' % (k, s, c))
print('\n= *설명의 사이트 수가 코드와 다름 : %d건' % len(bad_site))
for k, s, c in bad_site:
    print('   %-26s 설명=%s곳  코드=%s곳' % (k, s, c))
print('\n= 설명에 원본값 언급이 없음 : %d건' % len(nomention))
for i in range(0, min(len(nomention), 36), 6):
    print('   ' + '  '.join(nomention[i:i + 6]))
if len(nomention) > 36:
    print('   ... 외 %d건' % (len(nomention) - 36))
