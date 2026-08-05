# -*- coding: utf-8 -*-
import sys, io
sys.path.insert(0, r"C:\tfm2mods")
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8", errors="replace")
from s54lib import O, Nw
o=O(); n=Nw()
so=o.callseq(0xfce740); sn=n.callseq(0x1059bd0)
i0=[k for k,(a,t,s) in enumerate(so) if t==0x1535810][0]
print(f"0.5.3 컨테이너 0xfce740 size={o.fn[0xfce740]['size']} call={len(so)} SERPEN idx={i0}")
for k in range(max(0,i0-8), min(len(so), i0+9)):
    a,t,s=so[k]; print(f"   [{k}] +{a:#x} → {t:#x} size={s}" + ("  ★SERPEN" if t==0x1535810 else ""))
print()
ia=[k for k,(a,t,s) in enumerate(sn) if t==0x13273e0]; ib=[k for k,(a,t,s) in enumerate(sn) if t==0x1328950]
print(f"0.5.4 컨테이너 0x1059bd0 size={n.fn[0x1059bd0]['size']} call={len(sn)} A(0x13273e0) idx={ia} B(0x1328950) idx={ib}")
lo=min(ia+ib); hi=max(ia+ib)
for k in range(max(0,lo-8), min(len(sn), hi+9)):
    a,t,s=sn[k]; tag = "  ◀A" if t==0x13273e0 else ("  ◀B" if t==0x1328950 else "")
    print(f"   [{k}] +{a:#x} → {t:#x} size={s}{tag}")
