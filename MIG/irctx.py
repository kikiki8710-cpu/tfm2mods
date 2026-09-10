import io,sys,re
fn=sys.argv[1]; lines=[int(x) for x in sys.argv[2].split(',')]
txt=io.open(fn,encoding='utf-8',errors='replace').read().split('\n')
defs=[(i,l) for i,l in enumerate(txt) if l.startswith('define')]
import bisect
idx=[d[0] for d in defs]
for L in lines:
    p=bisect.bisect_right(idx,L-1)-1
    name=defs[p][1][:400] if p>=0 else '?'
    print('--- line %d ---' % L)
    print('define@%d: %s' % (defs[p][0]+1, name))
