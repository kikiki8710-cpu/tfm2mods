# -*- coding: utf-8 -*-
"""문서 작성용 골격 수집 — apply_* 함수별 노브 묶음·사이트 수, 훅 목록, 탭 구성."""
import sys, io, re, os, glob, json
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

SRC = 'C:/tfm2mods/tfm2_ai_adjust/src'
src = ''
for f in sorted(glob.glob(os.path.join(SRC, '*.rs'))):
    src += '\n' + io.open(f, encoding='utf-8').read()

print('== apply_* 함수별 노브·사이트 ==')
rows = []
for fm in re.finditer(r'\nunsafe fn (apply_\w+)\(\)\s*\{', src):
    s0 = fm.end(); nx = src.find('\nunsafe fn ', s0)
    body = src[s0:nx if nx > 0 else len(src)]
    keys = re.findall(r'tune\(\s*"([a-zA-Z0-9_]+)"', body)
    sites = len(re.findall(r'\bp!\(', body)) + len(re.findall(r'patch_imm_bytes\(', body))
    for tm in re.finditer(r'for &\(a, pre, off\) in (\w+)\.iter\(\)', body):
        m2 = re.search(r'static %s: \[\(usize, &\[u8\], usize\); (\d+)\]' % tm.group(1), src)
        if m2:
            sites += int(m2.group(1)) - 1
    out = re.search(r'pth\("([\w.]+)"\)', body)
    rows.append((fm.group(1), len(keys), sites, out.group(1) if out else ''))
for r in sorted(rows, key=lambda x: -x[2]):
    print('  %-26s 노브 %3d  사이트 %4d  %s' % r)
print('  합계 노브 %d / 사이트 %d' % (sum(r[1] for r in rows), sum(r[2] for r in rows)))

print('\n== 설치 훅 ==')
for m in re.finditer(r'install_(?:wrap|detour\w*)\(\s*(RVA_\w+)', src):
    print('  ' + m.group(1))

print('\n== 편집기 탭 ==')
ed = io.open('C:/tfm2mods/ai_adjust_editor/src/main.rs', encoding='utf-8').read()
for m in re.finditer(r'Tab\{\s*id:"(\w+)",\s*title:"([^"]*)".*?keys:&\[(.*?)\], note:', ed, re.S):
    n = len([x for x in re.findall(r'"([^"]*)"', m.group(3)) if not x.startswith('§')])
    print('  %-18s %-46s %d개' % (m.group(1), m.group(2), n))
