"""저장 입력만으로 occupied-cell iterator의 출력/쓰기 순서를 독립 계산한다."""
MASK=(1<<64)-1
SCAN_CAP=16
METRIC_NAMES={39:['w06_cell_occupied','w06_cell_radius_squared_inclusive']}
class DomainError(ValueError):pass
def span(a,n):
    if type(a)is not int or not 0x10000<=a<a+n<=1<<48:raise DomainError('W06_ADDRESS_RANGE_OR_OVERFLOW')
    return a,a+n
def overlap(a,b):return a[0]<b[1]and b[0]<a[1]
def evaluate(pid,args,snapshot):
    if pid!=39:raise DomainError('W06_ITERATOR_UNSUPPORTED_ID')
    if len(args)!=4 or any(type(a)is not int or not 0<=a<=MASK for a in args):raise DomainError('W06_ARGUMENTS')
    a=args;out=span(a[0],32);cursor=span(a[1]+8,8)
    if overlap(out,cursor):raise DomainError('W06_OUT_CURSOR_ALIAS')
    def read(b,o=0,first=False):
        p=(b+o)&MASK;s=span(p,8)
        if overlap(s,out)or(not first and overlap(s,cursor)):raise DomainError('W06_ITERATOR_INPUT_OUTPUT_ALIAS')
        return snapshot.read(p,0,8)
    metrics=[]
    def met(name,lhs,rhs,relation,p):metrics.append(dict(name=name,lhs=lhs,rhs=rhs,relation=relation,predicate=bool(p),observed_branch=False))
    start=read(a[1],8,True);end=read(a[1],16);fields=[]
    if start==end:fields=[(a[0],0)]
    else:
        if start>end:raise DomainError('W06_NONMONOTONIC_CURSOR_DOMAIN')
        checkroot=read(a[1]);xref=read(a[2]);yref=read(a[2],8)
        for scanned in range(SCAN_CAP):
            index=start+scanned;y=read(a[1],(index*16+32)&MASK)
            if y>29:raise DomainError('W06_Y_BOUNDS_PANIC_WITH_CURSOR_WRITE')
            x=read(a[1],(index*16+24)&MASK)
            if x>=30:raise DomainError('W06_X_BOUNDS_PANIC_WITH_CURSOR_WRITE')
            cell=y*240+x*8+0x1c98;occupied=read(checkroot,cell);met('w06_cell_occupied',occupied,0,'!=',occupied!=0)
            if occupied:
                cx=x*32000+16000;cy=y*32000+16000;rx=read(xref);ry=read(yref)
                d2=((((cx-rx)&MASK)**2)+(((cy-ry)&MASK)**2))&MASK;near=d2<=22500000000
                met('w06_cell_radius_squared_inclusive',d2,22500000000,'<=',near)
                if near:
                    outputroot=read(a[1],0x488);id=read(outputroot,cell)
                    fields=[(cursor[0],index+1),(a[0]+8,id),(a[0]+16,cx),(a[0]+24,cy),(a[0],1)];break
            if index+1==end:fields=[(cursor[0],end),(a[0],0)];break
        else:raise DomainError('W06_ITERATOR_SCAN_CAPTURE_CAP')
    return dict(expected=a[0],mask=MASK,output_ptr=a[0],metrics=metrics,output_fields=[dict(address=p,width=8,expected=v)for p,v in fields])
