"""출력3 독립 Python 계산. 저장된 입력만 사용하며 output actual/expected를 읽지 않는다."""
MASK=(1<<64)-1
METRIC_NAMES={29:['w05_healthy_hp_percent_ge40'],30:['w05_sort_score_left_lt_right','w05_sort_unsigned_index_tie'],31:['w05_record_class_partition','w05_record_metric_left_lt_right','w05_record_signed_index_tie']}
class DomainError(ValueError):pass
def span(a,n):
    if not isinstance(a,int)or not 0x10000<=a<a+n<=1<<48:raise DomainError('W05_ADDRESS_RANGE_OR_OVERFLOW')
    return a,a+n
def intersects(a,b):return a[0]<b[1]and b[0]<a[1]
def signed(x,b=64):return x-(1<<b)if x>>(b-1)else x
def evaluate(pid,args,snapshot):
    if pid not in METRIC_NAMES:raise DomainError('W05_OUTPUT_UNSUPPORTED_ID')
    if len(args)!=4 or any(type(a)is not int or not 0<=a<=MASK for a in args):raise DomainError('W05_ARGUMENTS')
    a=args;outbase=a[0]if pid==29 else a[1];output=span(outbase,24 if pid==29 else 32);metrics=[]
    def read(b,o=0,w=8):
        address=(b+o)&MASK
        if w not in(1,4,8):raise DomainError('W05_READ_WIDTH')
        if intersects(span(address,w),output):raise DomainError('W05_OUTPUT_INPUT_ALIAS')
        return snapshot.read(address,0,w)&((1<<(w*8))-1)
    def met(name,lhs,rhs,op,p):metrics.append(dict(name=name,lhs=lhs,rhs=rhs,relation=op,predicate=bool(p),observed_branch=False))
    if pid==29:
        if a[1]>1:raise DomainError('DBF730_TEAM_BOUNDS_PANIC')
        own=[read(a[2],0x238+a[1]*40+i*8)for i in range(5)];healthy=0
        for ptr in own:
            if not ptr:continue
            maxhp=read(ptr,0x620)
            if not maxhp:raise DomainError('DBF730_DIVISOR_ZERO_GAME_PANIC')
            percent=((read(ptr,0x668)*100)&MASK)//maxhp;good=percent>=40;healthy+=good;met('w05_healthy_hp_percent_ge40',percent,40,'>=',good)
        other=sum(read(a[2],0x238+(1-a[1])*40+i*8)!=0 for i in(1,0,2,3,4));values=[sum(p!=0 for p in own),healthy,other];rax=values[0]
    else:
        if intersects(span(a[0],32),output):raise DomainError('W05_OUTPUT_SOURCE4_ALIAS')
        values=[read(a[0],i*8)for i in range(4)]
        length,data=(a[3],a[2])if pid==30 else(read(a[2],0x10),read(a[2],8))
        if any(i>=length for i in values):raise DomainError('W05_SORT_INDEX_BOUNDS_PANIC')
        def right_first(left,right):
            if pid==30:
                x,y=signed(read(data,(left*8)&MASK)),signed(read(data,(right*8)&MASK));met('w05_sort_score_left_lt_right',x,y,'<',x<y)
                if x!=y:return x<y
                met('w05_sort_unsigned_index_tie',right,left,'<',right<left);return right<left
            lc,rc=read(data,(left*400+0x188)&MASK,1),read(data,(right*400+0x188)&MASK,1);lp,rp=lc>=5,rc>=5;met('w05_record_class_partition',int(lp),int(rp),'!=',lp!=rp)
            if lp!=rp:return lp
            x,y=signed(read(data,(left*400+0x108)&MASK,4),32),signed(read(data,(right*400+0x108)&MASK,4),32);met('w05_record_metric_left_lt_right',x,y,'<',x<y)
            if x!=y:return x<y
            ri,li=signed(right),signed(left);met('w05_record_signed_index_tie',ri,li,'<',ri<li);return ri<li
        for i,j in((0,1),(2,3),(0,2),(1,3),(1,2)):
            if right_first(values[i],values[j]):values[i],values[j]=values[j],values[i]
        rax=values[-1]
    return dict(expected=rax,mask=MASK,metrics=metrics,output_fields=[dict(address=outbase+i*8,width=8,expected=value)for i,value in enumerate(values)])
