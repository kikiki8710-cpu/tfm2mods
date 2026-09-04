exec(open('srcmap.py',encoding='utf-8').read())
import struct
def load(p):
    img=Img(p); img.prep(); return img
def callees(img,rva):
    fr=img.frange(rva)
    b,e=fr; out=[]
    for ins in md.disasm(img.code(b,e-b),BASE+b):
        if ins.mnemonic=='call' and ins.op_str.startswith('0x'):
            out.append(int(ins.op_str,16)-BASE)
    return b,e,out
def strrefs(img,rva,maxlen=64):
    fr=img.frange(rva); b,e=fr; d=img.data; res=[]
    for ins in md.disasm(img.code(b,e-b),BASE+b):
        for op in ins.operands:
            if op.type==X86_OP_MEM and op.mem.base==X86_REG_RIP:
                t=ins.address+ins.size+op.mem.disp-BASE
                if img.sec(t) not in ('.rdata',): continue
                o=img.r2o(t)
                if o is None: continue
                chunk=d[o:o+maxlen]
                # printable ascii run
                k=0
                while k<len(chunk) and 32<=chunk[k]<127: k+=1
                if k>=6: res.append((ins.address-BASE,t,chunk[:k].decode('latin1')))
    return res
