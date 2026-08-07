# -*- coding: utf-8 -*-
import io,sys,collections
sys.path.insert(0,r'C:\tfm2mods\v54')
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
import reloc as R
B=0x140000000
ver=sys.argv[1]; E=R.E3 if ver=='053' else R.E4
fns=sys.argv[2].split(','); tg=[int(x,16) for x in sys.argv[3].split(',')]
for a in fns:
    f=E.func_of(int(a,16))
    c=collections.Counter()
    for i in R.insns(E,f[0],f[1]):
        if i.mnemonic=='call' and i.op_str.startswith('0x'): c[int(i.op_str,16)-B]+=1
    print('%s: %s'%(a,' '.join('%06x=%d'%(t,c[t]) for t in tg)))
