"""D3EEC0 후보 scratch1000. actual/runtime/native/entry승인 없음."""
U=(1<<64)-1
METRIC_NAMES={1000:['next_peer_cost_improves','next_peer_first_candidate']}
class DomainError(ValueError):pass
def signed(x):return x-(1<<64)if x>>63 else x
def evaluate(pid,args,m):
    if pid!=1000:raise DomainError('SCRATCH_ID_NOT_OWNED')
    if not isinstance(args,list)or len(args)!=4 or any(type(x)is not int or not 0<=x<=U for x in args):raise DomainError('ARGUMENTS')
    out,cap,flag,_=args
    if not 0x10000<=out<out+24<=1<<48:raise DomainError('OUTPUT_SPAN')
    metrics=[];fields=set()
    def read(b,o=0,w=8,seed=False):
        p=(b+o)&U
        if not 0x10000<=p<p+w<=1<<48:raise DomainError('INPUT_SPAN')
        if not seed and p<out+24 and out<p+w:raise DomainError('OUTPUT_INPUT_ALIAS')
        fields.add((p,w))
        if len(fields)>96:raise DomainError('FIELD_BUDGET')
        return m.read(p,0,w)
    def metric(n,l,r,rel,p):metrics.append(dict(name=n,lhs=l,rhs=r,relation=rel,predicate=bool(p),observed_branch=False))
    cost,index=read(out,seed=True),read(out,8,seed=True)
    teamref=read(cap,0x18);world=read(cap,8);lines=read(cap,0x20);excludedref=read(cap)
    bb=read(cap,0x10);allowedref=read(cap,0x28);reference=read(cap,0x30);excluded=read(excludedref)
    linebest=255
    for i in range(5):
        if i==excluded:continue
        team=read(teamref)
        if team>=2:raise DomainError('TEAM_PANIC_WITH_PARTIAL_OUTPUT')
        entity=read(world,0x238+team*40+i*8)
        if not entity:continue
        line=read(lines,i,1)
        if line==255:continue
        if flag&255:
            state=read(bb,team*0xac8+i*32+0x7f0,1)
            if state:continue
            currentline=read(bb,team*0xac8+i*32+0x7f1,1)
            if currentline!=line:continue
        allowed=read(allowedref);first=read(allowed,0,1)
        if first!=255 and first!=line:
            if read(allowed,1,1)!=line:continue
        rx,ry=read(reference,0x658),read(reference,0x660)
        x,y=read(entity,0x658),read(entity,0x660)
        d2=(((rx-x)&U)**2+((ry-y)&U)**2)&U
        recordline=0 if line==0 else 1 if line==1 else 2
        progress=read(bb,team*0xac8+recordline*0x28+0x10)
        candidate=(d2//1000000+2*progress)&U
        firstcandidate=linebest==255;improves=signed(candidate)<signed(cost)
        metric('next_peer_first_candidate',linebest,255,'==',firstcandidate)
        metric('next_peer_cost_improves',signed(candidate),signed(cost),'<',improves)
        if firstcandidate or improves:cost,index,linebest=candidate,i,line
    return dict(expected=out,mask=U,output_ptr=out,metrics=metrics,
        output_fields=[dict(address=out+8,width=8,expected=index),dict(address=out,width=8,expected=cost),dict(address=out+16,width=1,expected=linebest)])
