# -*- coding: utf-8 -*-
import io,os,sys,re,collections
sys.stdout=io.TextIOWrapper(sys.stdout.buffer,encoding='utf-8')
sys.path.insert(0,r'C:\tfm2mods\v54')
from pe2 import load,BASE
D=r'C:\tfm2mods\v54'
def srcmap(ver):
    m={}
    for ln in io.open(os.path.join(D,'%s_srcmap.tsv'%ver),encoding='utf-8'):
        s,e,src,lines=ln.rstrip('\n').split('\t'); m[int(s,16)]=(src,lines)
    return m
def calls(ver,rva,n):
    e=load(ver); sm=srcmap(ver)
    cnt=collections.Counter()
    for i in e.dis(rva,n):
        if i.mnemonic=='call' and i.op_str.startswith('0x'):
            t=int(i.op_str,16)-BASE
            cnt[t]+=1
    out=[]
    for t,c in cnt.most_common():
        f=e.func_of(t)
        src=sm.get(f[0],('',''))[0] if f else ''
        out.append((t,c,(f[1]-f[0]) if f else 0,src))
    return out
if __name__=='__main__':
    ver,rva,n=sys.argv[1],int(sys.argv[2],16),int(sys.argv[3])
    for t,c,sz,src in calls(ver,rva,n):
        print('%06x x%-3d %6dB  %s'%(t,c,sz,src[:95]))
