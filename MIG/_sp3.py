exec(open('srcmap.py',encoding='utf-8').read())
import struct
E57=r"C:\Users\jungs\Desktop\claude\tfm2\tfm2_0.5.7\TeamfightManager2.exe"
E58=r"C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe"
def jt(p,jtrva,fnrva,tag):
    img=Img(p); img.prep(); b,e=img.frange(fnrva)
    o=img.r2o(jtrva); arms=[]
    for i in range(40):
        v=struct.unpack_from('<i',img.data,o+4*i)[0]
        t=(jtrva+v)&0xffffffff
        if not (b<=t<e): break
        arms.append(t)
    print(tag,"arms=",len(arms))
    for i,t in enumerate(arms):
        print("  arm[%2d] intag=%2d -> %s"%(i,i+2,hex(t)))
    return arms
a=jt(E57,0x3378428,0xdb2760,"0.5.7")
b=jt(E58,0x33d766c,0xcaf9f0,"0.5.8")
