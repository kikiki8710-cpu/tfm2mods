import io,sys,bisect,re
fn=sys.argv[1]
lines=[int(x) for x in sys.argv[2].split(',')]
txt=io.open(fn,encoding='utf-8',errors='replace').read().split('\n')
defs=[(i,l) for i,l in enumerate(txt) if l.startswith('define')]
idx=[d[0] for d in defs]
def short(d):
    m=re.search(r'@([\w.$]+)\(',d)
    if not m: return d[:80]
    s=m.group(1)
    # crude rust v0 demangle: keep readable segments
    parts=re.findall(r'[A-Za-z_][A-Za-z0-9_]{2,}',s)
    parts=[p for p in parts if not re.match(r'^(Nv|Nt|Cs|Ms|INt|NtNt)',p)]
    return s
for L in lines:
    p=bisect.bisect_right(idx,L-1)-1
    print('%-8d %s'%(L,short(defs[p][1])))
