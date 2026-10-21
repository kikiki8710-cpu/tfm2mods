# 0.6.2: 지정 disp 값이 신 exe 에서 무엇으로 바뀌었는지 — 구 exe 에서 disp32 를 쓰는 함수 → GM 매칭 신 함수 → 명령정렬 투표
import sys,re,struct,difflib,collections; sys.path.insert(0,'MIG'); from _x065 import *
CTX=None
args=sys.argv[1:]
if args and args[0].startswith('ctx='): CTX=int(args[0][4:],16); args=args[1:]
T=[int(x,16) for x in args]
# GM: skel 유일 > head 유일 > 같은 size 유일
bs=collections.defaultdict(list); bh=collections.defaultdict(list)
for k,v in PN.items(): bs[v['skel']].append(k); bh[v['head']].append(k)
def gm(o):
    f=PO[o]; c=bs.get(f['skel'],[])
    if len(c)==1: return c[0]
    h=bh.get(f['head'],[])
    if len(h)==1: return h[0]
    same=[x for x in c if PN[x]['size']==f['size']]
    return same[0] if len(same)==1 else None
d,secs=O; tx=[s for s in secs if s[0]==b'.text'][0]; _,va,vs,ro=tx
blob=d[ro:ro+vs]
RX=re.compile(r'\[(r\w+)(?: \+ r\w+\*\d)? \+ (0x[0-9a-f]+)\]')
def key(i): return i.mnemonic+'|'+re.sub(r'0x[0-9a-f]+','N',i.op_str)
res={t:collections.Counter() for t in T}; nf={t:0 for t in T}
for t in T:
    owners=set()
    for m in re.finditer(re.escape(struct.pack('<I',t)),blob):
        o=owner(SO,m.start()+va)
        if o is not None and PO.get(o) and m.start()+va < o+int(PO[o]['size']): owners.add(o)
    if CTX is not None:
        pk=struct.pack('<I',CTX)
        owners={o for o in owners if pk in rd(O,o,int(PO[o]['size']))}
    nf[t]=len(owners)
    for o in list(owners)[:80]:
        n=gm(o)
        if n is None: continue
        A=insns(O,o,int(PO[o]['size'])); B=insns(N,n,int(PN[n]['size']))
        if len(A)>30000: continue
        sm=difflib.SequenceMatcher(a=[key(i) for i in A],b=[key(i) for i in B],autojunk=False)
        for a0,b0,sz in sm.get_matching_blocks():
            for k in range(sz):
                ia,ib=A[a0+k],B[b0+k]
                ma=RX.findall(ia.op_str); mb=RX.findall(ib.op_str)
                for (ra,da),(rb,db) in zip(ma,mb):
                    if int(da,16)==t: res[t][hex(int(db,16))]+=1
for t in T: print(hex(t),'fns',nf[t],'->',res[t].most_common(4))
