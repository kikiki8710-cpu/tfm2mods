# -*- coding: utf-8 -*-
"""⑤ rnd(%3)·_debug(%8)·team_plan(%7) 이 전달되는 call 사이트 전수 + 콜리 define 의 해당 인자 속성(readonly/readnone)"""
from load import *
import re, io, glob
defs = {}
def find_define(sym):
    if sym in defs: return defs[sym]
    for f in sorted(glob.glob(r'C:\tfm2mods\_gaibc\m*.ll')):
        with io.open(f, encoding='utf-8', errors='ignore') as fh:
            for ln, l in enumerate(fh, 1):
                if l.startswith('define') and ('@' + sym + '(') in l:
                    defs[sym] = (f[-6:], ln, l); return defs[sym]
    defs[sym] = None; return None
def argattr(defline, idx):
    inner = defline[defline.find('(')+1:]
    # 인자 분할: 최상위 콤마
    depth = 0; cur = ''; args = []
    for ch in inner:
        if ch == '(': depth += 1
        elif ch == ')':
            if depth == 0: break
            depth -= 1
        if ch == ',' and depth == 0: args.append(cur); cur = ''
        else: cur += ch
    args.append(cur)
    return args[idx].strip() if idx < len(args) else '?'
for reg, nm in (('%3', 'rnd'), ('%8', '_debug'), ('%7', 'team_plan'), ('%6', 'parameter')):
    print('=====', nm, reg)
    for i in range(LO, HI+1):
        s = L(i)
        if ('call' in s or 'invoke' in s) and re.search(re.escape(reg) + r'(?![0-9])', s):
            m = re.search(r'@([A-Za-z0-9_.$]+)\(', s)
            if not m or m.group(1).startswith('llvm.'): continue
            sym = m.group(1)
            # 인자 인덱스
            inner = s[s.find('@' + sym + '(') + len(sym) + 2:]
            depth = 0; cur = ''; args = []
            for ch in inner:
                if ch == '(': depth += 1
                elif ch == ')':
                    if depth == 0: break
                    depth -= 1
                if ch == ',' and depth == 0: args.append(cur); cur = ''
                else: cur += ch
            args.append(cur)
            idx = next((k for k, a in enumerate(args) if a.strip().endswith(' ' + reg) or a.strip() == reg), None)
            if idx is None: continue
            d = find_define(sym)
            attr = argattr(d[2], idx) if (d and idx is not None) else ('(define 없음)' if not d else '?')
            print('L%d ;L%s  %s  arg#%s  %s' % (i, chain(i), sym[-70:], idx, attr[:110]))
