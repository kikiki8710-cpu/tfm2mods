# -*- coding: utf-8 -*-
import io,sys
sys.path.insert(0,r'C:\tfm2mods\v54')
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
import reloc as R
for a in sys.argv[1:]:
    x=int(a,16); f=R.E3.func_of(x)
    pr=R.pair_fn(f[0],f[1])
    print('fn3 %06x-%06x (%dB) → %s'%(f[0],f[1],f[1]-f[0],
        ('054 %06x-%06x  골격 %.0f%%'%(pr[0],pr[1],pr[2]*100)) if pr else 'None(소스앵커 없음/후보없음)'))
