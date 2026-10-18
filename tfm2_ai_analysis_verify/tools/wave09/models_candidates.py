"""기존 함수 두 개의 미배선 Python 예상식. 실제 출력/게임 helper를 정답으로 쓰지 않는다."""
U=(1<<64)-1
METRIC_NAMES={0xe1b5b0:['basic_radius_modifier_nonzero','effect_level3_available','effect_time15_branch'],0xd8c680:['merge_signed_score_precedes','merge_unsigned_index_tie']}
class DomainError(ValueError):pass
def signed(x,bits=64):return x-(1<<bits)if x>>(bits-1)else x
def overlap(a,b):return a[0]<b[1]and b[0]<a[1]
def span(a,n):
    if a<0x10000 or n<=0 or a+n>1<<48:raise DomainError('ADDRESS_RANGE_OR_OVERFLOW')
    return a,a+n
def evaluate(rva,args,m):
    if rva==0xe1b5b0:return effect(args,m)
    if rva==0xd8c680:return merge(args,m)
    raise DomainError('SCRATCH_RVA_NOT_OWNED')
def reader(m,out):
    def read(a,o=0,w=8):
        p=(a+o)&U;s=span(p,w)
        if overlap(s,out):raise DomainError('READONLY_OUTPUT_ALIAS')
        return m.read(p,0,w)&((1<<(8*w))-1)
    return read
def metric(name,l,r,rel,p):return dict(name=name,lhs=l,rhs=r,relation=rel,predicate=bool(p),observed_branch=False)
def effect(a,m):
    out,entity,t,_=a;r=reader(m,span(out,72));metrics=[]
    if r(entity,0x4b8,4)==0xffffffff:raise DomainError('BASIC_NONE_PANIC')
    speed=r(entity,0x638);extra=r(entity,0x430);base=r(entity,0x498);per=r(entity,0x4a0);level=r(entity,0x5c0)
    modifier=signed(r(entity,0x468,4),32)
    metrics.append(metric(METRIC_NAMES[0xe1b5b0][0],modifier,0,'!=',modifier!=0))
    rawradius=r(entity,0x678)
    radius=(((modifier+100)*rawradius)&U)//100 if modifier else rawradius
    kind=r(entity,0x68)
    if kind>15:raise DomainError('UNGUARDED_KIND_JT_DOMAIN')
    idx=6 if kind<2 else kind-2
    offsets=[None,0xb8,0x110,None,0xe8,0xd0,0x1e8,0xe8,0xb0,0xc8,0xf0,0xc8,0xc0,0xb0]
    main=max(r(entity,offsets[idx])if offsets[idx]is not None else 0,t)
    skill1=(entity+0x4c0)&U
    if r(entity,0x4f0,4)==0xffffffff:skill1=0
    s1time=max(r(entity,0xb8),t)
    metrics.append(metric(METRIC_NAMES[0xe1b5b0][1],level,3,'>=',level>=3))
    if level<3:raise DomainError('STATIC_NONE_LOADED_ADDRESS_UNSUPPORTED')
    skill2=(entity+0x4f8)&U
    if r(entity,0x528,4)==0xffffffff:skill2=0
    total=(extra+base+radius+((level-1)*per))&U
    s2time=max(r(entity,0xc0),t)
    time15=idx==13;metrics.append(metric(METRIC_NAMES[0xe1b5b0][2],idx,13,'==',time15))
    if not time15:s1time=s2time=t
    values={0:entity,8:(entity+0x488)&U,16:speed,24:total,32:main,40:skill1,48:s1time,56:skill2,64:s2time}
    fields=[dict(address=out+k,width=8,expected=values[k])for k in [0,16,8,24,32,40,48,56,64]]
    return dict(expected=(entity+0x488)&U,mask=U,output_ptr=out,output_fields=fields,metrics=metrics)
def merge(a,m):
    source,n,out,header=a
    if not 2<=n<=16:raise DomainError('MERGE_CAPTURE_LEN_2_TO16')
    output=span(out,n*8);inputspan=span(source,n*8)
    if overlap(output,inputspan):raise DomainError('MERGE_INPUT_OUTPUT_ALIAS')
    r=reader(m,output);half=n//2;lh=0;rh=half;lt=half-1;rt=n-1;fields=[];metrics=[]
    def pair(rightpos,leftpos):
        right=r(source,rightpos*8);length=r(header,8)
        if right>=length:raise DomainError('MERGE_RIGHT_BOUNDS_PANIC')
        left=r(source,leftpos*8)
        if left>=length:raise DomainError('MERGE_LEFT_BOUNDS_PANIC')
        scores=r(header);ls=r((scores+left*8)&U);rs=r((scores+right*8)&U)
        better=signed(ls)<signed(rs)
        metrics.append(metric(METRIC_NAMES[0xd8c680][0],signed(ls),signed(rs),'<',better))
        if ls==rs:
            better=right<left;metrics.append(metric(METRIC_NAMES[0xd8c680][1],right,left,'<',better))
        return right,left,better
    for i in range(half):
        right,left,rightfirst=pair(rh,lh)
        fields.append(dict(address=out+i*8,width=8,expected=right if rightfirst else left))
        right,left,leftlast=pair(rt,lt)
        rh+=int(rightfirst);lh+=int(not rightfirst)
        fields.append(dict(address=out+(n-1-i)*8,width=8,expected=left if leftlast else right))
        lt-=int(leftlast);rt-=int(not leftlast)
    if n%2:
        if lh<lt+1:remaining=r(source,lh*8);lh+=1
        else:remaining=r(source,rh*8);rh+=1
        fields.append(dict(address=out+half*8,width=8,expected=remaining))
    if lh!=lt+1 or rh!=rt+1:raise DomainError('MERGE_RUN_POINTERS_DID_NOT_MEET_PANIC')
    return dict(expected=0,mask=U,output_ptr=out,output_fields=fields,metrics=metrics)
