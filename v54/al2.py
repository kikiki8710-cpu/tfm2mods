# -*- coding: utf-8 -*-
"""al2.py — 명시적 함수짝(fn3,fn4)을 주고 사이트를 정렬한다.
사용: python al2.py <fn4start_hex> <site1> [site2 ...]
      (fn3 는 site 로부터 .pdata 로 자동 도출)
"""
import io,sys,collections
sys.path.insert(0,r'C:\tfm2mods\v54')
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
import reloc as R, align as A
E3,E4=R.E3,R.E4
B=0x140000000

def run(fn4s, sites, ctx=6):
    f=E3.func_of(sites[0]); g=E4.func_of(fn4s)
    P=A.Pair(f[0],f[1],g[0],g[1],0.0)
    import difflib
    P.ratio=difflib.SequenceMatcher(None,[i.mnemonic for i in P.a],[i.mnemonic for i in P.b],autojunk=False).ratio()
    print('fn3 %06x-%06x (%d ins)  →  fn4 %06x-%06x (%d ins)  골격 %.0f%%'
          %(f[0],f[1],len(P.a),g[0],g[1],len(P.b),P.ratio*100))
    for rva in sites:
        print('--- 053 %06x'%rva)
        i=P.ia.get(rva)
        if i is None:
            print('   ⚠명령경계 아님'); continue
        ins=P.a[i]
        print('   053:',ins.bytes.hex(),ins.mnemonic,ins.op_str)
        for k in range(max(0,i-ctx),min(len(P.a),i+ctx+1)):
            x=P.a[k]; print('    %s %06x %-18s %s %s'%('>' if k==i else ' ',x.address-B,x.bytes.hex()[:18],x.mnemonic,x.op_str))
        for nm,ops in (('L1',P.op1),('L2',P.op2)):
            j,tag,rng=P.map_idx(i,ops)
            if j is not None:
                y=P.b[j]
                print('   %s equal → 054 %06x  %s %s %s'%(nm,y.address-B,y.bytes.hex(),y.mnemonic,y.op_str))
                for k in range(max(0,j-ctx),min(len(P.b),j+ctx+1)):
                    x=P.b[k]; print('    %s %06x %-18s %s %s'%('>' if k==j else ' ',x.address-B,x.bytes.hex()[:18],x.mnemonic,x.op_str))
                break
            else:
                lo,hi=P.bracket(i,ops)
                lo=max(0,lo); hi=min(hi,len(P.b))
                print('   %s %s 블록 — 054 후보구간 [%06x .. %06x] %d명령'%(nm,tag,
                    P.b[lo].address-B if lo<len(P.b) else 0, P.b[hi-1].address-B if hi>0 else 0, hi-lo))
                for k in range(lo,hi):
                    y=P.b[k]
                    if y.mnemonic==ins.mnemonic:
                        print('      %06x %-18s %s %s'%(y.address-B,y.bytes.hex()[:18],y.mnemonic,y.op_str))
if __name__=='__main__':
    fn4=int(sys.argv[1],16); sites=[int(x,16) for x in sys.argv[2:]]
    run(fn4,sites)
