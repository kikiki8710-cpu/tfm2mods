from h import *
import struct,sys
def scan(img,val):
    d,secs=img; name,va,vs,ro=secs[0]; t=d[ro:ro+vs]; pat=struct.pack('<I',val); out=[]; i=t.find(pat)
    while i>=0: out.append(i+va); i=t.find(pat,i+1)
    return out
def uses(img,S,val):
    res=[]
    for h in scan(img,val):
        f=owner(S,h)
        if f is None: continue
        for back in range(1,12):
            ok=False
            for ins in cs.disasm(rd(img,h-back,20),0x140000000+h-back):
                a=ins.address-0x140000000
                if a==h-back and a<h<a+ins.size and hex(val)+']' in ins.op_str:
                    res.append((a,f,ins.mnemonic,ins.op_str)); ok=True
                break
            if ok: break
    return res
if __name__=='__main__':
    side=sys.argv[1]; img,S=(O,SO) if side=='o' else (N,SN)
    for v in sys.argv[2:]:
        r=uses(img,S,int(v,16)); print('==',v,len(r))
        for a,f,m,o in r:
            if not (m in('movups','movaps','movdqu','movdqa') ) : print('  %x fn %x %s %s'%(a,f,m,o))
