from h import *
import struct
def sites(img,P,f):
    out=[]
    for i in insns(img,f,int(P[f]['size'])):
        a=i.address-0x140000000
        b=bytes(i.bytes)
        if 'rip' in i.op_str:
            # disp4 location: find in modrm; capstone gives disp offset
            m=re.search(r'\[rip ([+-]) (0x[0-9a-f]+)\]',i.op_str)
            d=int(m.group(2),16)*(1 if m.group(1)=='+' else -1)
            t=a+i.size+d
            # disp position: search the 4 bytes of disp in instruction
            db=struct.pack('<i',d); k=b.find(db)
            v=rd(img,t,4)
            fv=struct.unpack('<f',v)[0] if v else None
            out.append(('rip',a,a+k if k>=0 else None,i.mnemonic,i.op_str,t,fv))
        if 'rsp + 0x20' in i.op_str and i.mnemonic=='mov' and '0x43f00000' in i.op_str:
            k=b.find(struct.pack('<I',0x43f00000)); out.append(('imm',a,a+k,i.mnemonic,i.op_str,None,480.0))
    return out
