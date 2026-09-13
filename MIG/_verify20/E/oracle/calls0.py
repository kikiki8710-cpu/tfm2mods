from stores import resolve, defs
from common import *
import re,sys
for k,ln in enumerate(FN):
    s=ln.strip()
    if not re.search(r'\b(call|invoke)\b',s): continue
    if 'llvm.dbg' in s or 'llvm.lifetime' in s or 'llvm.assume' in s: continue
    m=re.search(r'@([\w$.]+)\(',s)
    name=m.group(1) if m else '?'
    argstr=s[m.end():] if m else s
    args=re.findall(r'(%[\w.]+)',argstr)
    hits=[]
    for a in args:
        b,o=resolve(a)
        if b=='%0' or b.startswith('%0+') or b.startswith('*%0'):
            hits.append((a,b,hex(o)))
    if hits:
        # find dbg on this or next line
        r=root(ln) or root(FN[k+1]) if k+1<len(FN) else None
        print(S+k, name[:80], hits, r[0] if r else None)
