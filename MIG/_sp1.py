exec(open('srcmap.py',encoding='utf-8').read())
E57=r"C:\Users\jungs\Desktop\claude\tfm2\tfm2_0.5.7\TeamfightManager2.exe"
E58=r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe"
import os
print(os.path.exists(E57),os.path.exists(E58))
a=Img(E57); a.prep()
b=Img(E58); b.prep()
print("0.5.7 movepri", [hex(x) for x in a.frange(0xdb2760)])
print("0.5.8 movepri", [hex(x) for x in b.frange(0xcaf9f0)])
fa=a.frange(0xdb2760); fb=b.frange(0xcaf9f0)
print("sz", hex(fa[1]-fa[0]), hex(fb[1]-fb[0]))
