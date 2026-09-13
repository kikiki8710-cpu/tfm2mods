# -*- coding: utf-8 -*-
"""함수 범위 안에서 inlinedAt 사슬에 등장하는 모든 DISubprogram(name, linkageName) 집합 = 인라인된 콜리 목록."""
import re, sys, collections
sys.path.insert(0, __import__('os').path.dirname(__file__))
from irlib import LL
ll = LL(sys.argv[1]); a = int(sys.argv[2]); b = int(sys.argv[3])
def sp_of_scope(i):
    seen=set()
    while i is not None and i not in seen:
        seen.add(i); s=ll.md.get(i,'')
        if 'DISubprogram' in s[:40]:
            n=re.search(r'name: "([^"]+)"',s); l=re.search(r'linkageName: "([^"]+)"',s); f=re.search(r'file: !(\d+)',s); ln=re.search(r'line: (\d+)',s)
            return (n.group(1) if n else '?', l.group(1) if l else '', ll.files.get(int(f.group(1))) if f else '', ln.group(1) if ln else '')
        m=re.search(r'scope: !(\d+)',s); i=int(m.group(1)) if m else None
    return None
cnt=collections.Counter(); ex={}
for n in range(a,b+1):
    d=ll.dbg_of(n)
    if not d: continue
    i=d; seen=set()
    while i is not None and i not in seen:
        seen.add(i); s=ll.md.get(i,'')
        if '!DILocation' not in s[:30]: break
        sc=re.search(r'scope: !(\d+)',s); ia=re.search(r'inlinedAt: !(\d+)',s)
        sp=sp_of_scope(int(sc.group(1))) if sc else None
        if sp and ia:  # inlined frame
            cnt[sp]+=1; ex.setdefault(sp,n)
        i=int(ia.group(1)) if ia else None
for sp,c in sorted(cnt.items(), key=lambda kv:(kv[0][2] or '',kv[0][0])):
    print(f"{c:5d} {sp[0][:70]:70s} {str(sp[2]).split(chr(92))[-1]}:{sp[3]}  {sp[1][:90]}")
