"""기존 D8A1F0의 정상 삽입 한 단계. 게임 호출/쓰기 없음; probe id 미배정."""
MASK=(1<<64)-1
INDEX_PREFIX_CAP=20
METRIC_NAMES=['insert_predecessor_score_lt_key','insert_equal_score_unsigned_index_tie']
class DomainError(Exception): pass
def signed(x): return x-(1<<64) if x>>63 else x
def span(a,n):
    if a<0x10000 or n<=0 or a+n>1<<48: raise DomainError('INSERT_ADDRESS_RANGE_OR_OVERFLOW')
    return a,a+n
def overlap(a,b): return a[0]<b[1] and b[0]<a[1]
def evaluate(args,memory):
    begin,pos,header,_=args
    if pos<=begin or (pos-begin)%8: raise DomainError('INSERT_PREDECESSOR_OR_ALIGNMENT')
    count=(pos-begin)//8+1
    if count>INDEX_PREFIX_CAP: raise DomainError('INSERT_INDEX_CAPTURE_CAP')
    indices=span(begin,count*8)
    def read(address,index=False):
        s=span(address,8)
        if not index and overlap(s,indices): raise DomainError('INSERT_READONLY_INDEX_ALIAS')
        return memory.read(address,0,8)&MASK
    def score(ptr,index):
        return read((ptr+index*8)&MASK)
    metrics=[]
    def precedes(pred,key,predscore,keyscore):
        less=signed(predscore)<signed(keyscore)
        metrics.append(dict(name=METRIC_NAMES[0],lhs=signed(predscore),rhs=signed(keyscore),relation='<',predicate=less,observed_branch=False))
        if predscore==keyscore:
            tie=key<pred
            metrics.append(dict(name=METRIC_NAMES[1],lhs=key,rhs=pred,relation='<',predicate=tie,observed_branch=False))
            return tie
        return less
    key=read(pos,True)
    length=read(header+8)
    if key>=length: raise DomainError('INSERT_KEY_BOUNDS_PANIC')
    cursor=pos-8
    pred=read(cursor,True)
    if pred>=length: raise DomainError('INSERT_PREDECESSOR_BOUNDS_PANIC')
    ptr=read(header)
    predscore=score(ptr,pred)
    keyscore=score(ptr,key)
    fields=[]
    if precedes(pred,key,predscore,keyscore):
        hole=pos
        while True:
            fields.append(dict(address=hole,width=8,expected=pred))
            hole=cursor
            if cursor==begin: break
            length=read(header+8)
            if key>=length: raise DomainError('INSERT_LATE_KEY_BOUNDS_PANIC')
            cursor-=8
            pred=read(cursor,True)
            if pred>=length: raise DomainError('INSERT_LATE_PREDECESSOR_BOUNDS_PANIC')
            ptr=read(header)
            predscore=score(ptr,pred)
            keyscore=score(ptr,key)
            if not precedes(pred,key,predscore,keyscore): break
        fields.append(dict(address=hole,width=8,expected=key))
    return dict(expected=pred,mask=MASK,output_ptr=begin if fields else None,output_fields=fields,metrics=metrics)
