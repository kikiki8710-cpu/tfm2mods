import sys, struct, pefile, collections
OLD = r"C:\Users\jungs\Desktop\claude\tfm2\tfm2_0.5.8\TeamfightManager2.exe"
NEW = r"C:\Users\jungs\Desktop\claude\tfm2\tfm2_0.6.0\TeamfightManager2.exe"
BASE=0x140000000
def scan(p):
    pe=pefile.PE(p, fast_load=True)
    text=[s for s in pe.sections if s.Name.startswith(b'.text')][0]
    tlo=BASE+text.VirtualAddress; thi=tlo+text.Misc_VirtualSize
    rdata=[s for s in pe.sections if s.Name.startswith(b'.rdata')][0]
    rlo=BASE+rdata.VirtualAddress; data=rdata.get_data()
    n=len(data)//8
    q=struct.unpack_from('<%dQ'%n, data, 0)
    res=[]
    for i in range(n-12):
        drop=q[i]; size=q[i+1]; al=q[i+2]
        if al not in (1,2,4,8,16) or size>0x2000: continue
        if not (drop==0 or tlo<=drop<thi): continue
        # count consecutive text slots
        k=0
        while i+3+k<n and tlo<=q[i+3+k]<thi and k<40: k+=1
        if k<8: continue
        res.append((rlo+8*i, size, al, tuple(q[i+3:i+3+k])))
    return res
which=sys.argv[1]
res=scan(OLD if which=='old' else NEW)
# find vtables whose slot idx given matches given rva
if len(sys.argv)>2:
    slot=int(sys.argv[2],16); tgt=int(sys.argv[3],16)+BASE
    hits=[r for r in res if len(r[3])> (slot-0x18)//8 and r[3][(slot-0x18)//8]==tgt]
    print(len(hits))
    for r in hits[:200]:
        print(hex(r[0]), hex(r[1]), r[2], ' '.join(hex(x-BASE) for x in r[3][:14]))
else:
    print(len(res))
