# -*- coding: utf-8 -*-
import io, sys, collections
sys.path.insert(0, r'C:\tfm2mods\v54')
from scan import Scanner, src_of
S = {}
def get(v):
    if v not in S: S[v] = Scanner(v)
    return S[v]
def cmpc(val, width=4, aionly=True):
    res = {}
    for ver in ('053','054'):
        s = get(ver)
        by = collections.defaultdict(list)
        for a,f,i in s.hits(val,width):
            src,_ = src_of(ver, f[0])
            src = src or '(nosrc)'
            if aionly and 'game-ai' not in src: continue
            by[src].append((a,f[0],i.mnemonic,i.op_str))
        res[ver]=by
    keys = sorted(set(res['053'])|set(res['054']))
    print('#### %d (0x%x) w%d   053=%d  054=%d' % (val,val,width,
        sum(len(v) for v in res['053'].values()), sum(len(v) for v in res['054'].values())))
    for k in keys:
        a,b = res['053'].get(k,[]), res['054'].get(k,[])
        mark = '  ' if len(a)==len(b) else '★'
        print('%s %-88s %2d -> %2d' % (mark, k[:88], len(a), len(b)))
        if len(a)!=len(b):
            for lbl,lst in (('053',a),('054',b)):
                for x in lst[:12]:
                    print('        [%s] %06x fn %06x  %s %s' % (lbl,x[0],x[1],x[2],x[3]))
    print()
if __name__=='__main__':
    for arg in sys.argv[1:]:
        if ':' in arg:
            v,w = arg.split(':'); cmpc(int(v,0), int(w))
        else:
            cmpc(int(arg,0))
