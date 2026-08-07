# -*- coding: utf-8 -*-
"""corr.py <fn3> <fn4> — 두 함수 지문 대조(크기/명령수/호출대상/문자열/희귀상수)"""
import io,sys,collections,difflib
sys.path.insert(0,r'C:\tfm2mods\v54')
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
import reloc as R
B=0x140000000
def prof(E,a):
    f=E.func_of(a); ins=R.insns(E,f[0],f[1])
    calls=[]; imms=collections.Counter(); rip=[]
    for i in ins:
        if i.mnemonic=='call' and i.op_str.startswith('0x'): calls.append(int(i.op_str,16)-B)
        if 'rip +' in i.op_str or 'rip -' in i.op_str:
            try:
                d=i.op_str.split('rip ')[1].split(']')[0]
                sign=1 if d[0]=='+' else -1
                rip.append(i.address-B+i.size+sign*int(d[1:].strip(),16))
            except: pass
        for tok in i.op_str.split(', '):
            if tok.startswith('0x') and len(tok)>4:
                try: imms[int(tok,16)]+=1
                except: pass
    return f,ins,calls,imms,rip
f3,i3,c3,m3,r3=prof(R.E3,int(sys.argv[1],16))
f4,i4,c4,m4,r4=prof(R.E4,int(sys.argv[2],16))
print('fn3 %06x-%06x %dB %dins  |  fn4 %06x-%06x %dB %dins'%(f3[0],f3[1],f3[1]-f3[0],len(i3),f4[0],f4[1],f4[1]-f4[0],len(i4)))
print('골격 %.0f%%'%(difflib.SequenceMatcher(None,[x.mnemonic for x in i3],[x.mnemonic for x in i4],autojunk=False).ratio()*100))
print('call수 %d/%d  직접call대상(053) %s'%(len(c3),len(c4),['%06x'%x for x in sorted(set(c3))][:14]))
print('                직접call대상(054) %s'%(['%06x'%x for x in sorted(set(c4))][:14]))
def strs(E,rips):
    out=[]
    for a in set(rips):
        s=E.cstr(a) if hasattr(E,'cstr') else None
        if s: out.append(s)
    return out
try:
    print('str3',sorted(set(strs(R.E3,r3)))[:12])
    print('str4',sorted(set(strs(R.E4,r4)))[:12])
except Exception as e: print('str skip',e)
k3=set(m3)-set(m4); k4=set(m4)-set(m3)
print('053전용 상수', ['0x%x'%x for x in sorted(k3)][:16])
print('054전용 상수', ['0x%x'%x for x in sorted(k4)][:16])
