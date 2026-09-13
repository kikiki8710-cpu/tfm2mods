from common import *
import re,collections
seen=collections.OrderedDict()
for k,ln in enumerate(FN):
    d=dbg_of(ln)
    if d is None: continue
    i=d
    while i is not None:
        lc=loc(i)
        if not lc: break
        line,scope,inl=lc
        sp=subprog(scope)
        if sp:
            key=(sp[0],fname(sp[1]),sp[2])
            if key not in seen: seen[key]=(S+k,line)
        i=inl
for (n,f,l),(irl,line) in seen.items():
    if 'library' in (f or ''): continue
    print(f"{n[:110]:110s} {f}:{l}  first@{irl} (line {line})")
