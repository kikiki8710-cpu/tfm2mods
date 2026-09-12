exec(open('srcmap.py',encoding='utf-8').read())
import struct
E57=r"C:\Users\jungs\Desktop\claude\tfm2\tfm2_0.5.7\TeamfightManager2.exe"
A=Img(E57);A.prep()
b,e=A.frange(0xcbf340); o=A.r2o(0x336be88)
arms=[]
for i in range(30):
    v=struct.unpack_from('<i',A.data,o+4*i)[0]; t=(0x336be88+v)&0xffffffff
    if not (b<=t<e): break
    arms.append(t)
print("arms",len(arms))
for i,t in enumerate(arms):
    # find first call after t
    code=A.code(t,120); tgt=None
    for ins in md.disasm(code,BASE+t):
        if ins.mnemonic=='call' and ins.op_str.startswith('0x'): tgt=int(ins.op_str,16)-BASE; break
        if ins.mnemonic=='jmp': break
    print("  arm[%2d] sp=%2d stub=%s handler=%s"%(i,i+2,hex(t),hex(tgt) if tgt else "-"))
