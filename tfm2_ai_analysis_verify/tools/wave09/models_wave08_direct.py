"""runtime42/43 준비 독립식. 입력읽기만 수행하며 게임 helper는 호출하지 않는다."""
U=(1<<64)-1
METRIC_NAMES={42:['w08_wave_own_nonempty','w08_wave_mode_le8','w08_wave_lane_pair_zero','w08_wave_distance_within_reference','w08_wave_qualifying_count_ge6'],
 43:['w08_objective_mode_le8','w08_objective_candidate_admitted','w08_objective_rank_below3','w08_objective_metadata_below2000','w08_objective_state_below2','w08_objective_first_candidate','w08_objective_cost_improves']}
class DomainError(ValueError):pass

def signed(x):return x-(1<<64)if x>>63 else x
def signed32(x):return x-(1<<32)if x>>31 else x

def evaluate(pid,args,m):
    if pid not in METRIC_NAMES:raise DomainError('MODEL_NOT_OWNED')
    if not isinstance(args,list)or len(args)!=4 or any(type(a)is not int or not 0<=a<=U for a in args):raise DomainError('ARGUMENTS')
    metrics=[];fields=set()
    def read(b,o=0,w=8):
        p=(b+o)&U
        if not 0x10000<=p<p+w<=1<<48:raise DomainError('INPUT_SPAN')
        fields.add((p,w))
        if len(fields)>96:raise DomainError('W08_FIELD_CAPTURE_CAP96')
        return m.read(p,0,w)
    def mark(n,l,r,rel,p):metrics.append(dict(name=n,lhs=l,rhs=r,relation=rel,predicate=bool(p),observed_branch=False))
    def done(x):return dict(expected=x,mask=255,output_ptr=None,output_fields=[],metrics=metrics)
    def d2(ax,ay,bx,by):return (((ax-bx)&U)**2+((ay-by)&U)**2)&U
    if pid==42:
        team,cache,context,_=args
        if team>1:raise DomainError('W08_WAVE_TEAM_GAME_PANIC')
        base=read(cache,0x1c8+team*8)
        if not base:raise DomainError('W08_WAVE_BASE_GAME_PANIC')
        ownlen=read(cache,0x188+team*32);mark('w08_wave_own_nonempty',ownlen,0,'!=',ownlen!=0)
        if not ownlen:return done(0)
        ownvec=read(cache,0x170+team*32);first=read(ownvec)
        fx,fy=read(first,0x658),read(first,0x660)
        bx,by=read(base,0x658),read(base,0x660)
        mode=read(context,0x38,1);valid=mode<=8;mark('w08_wave_mode_le8',mode,8,'<=',valid)
        if not valid:return done(0)
        reference=d2(fx,fy,bx,by);opponent=1-team;records=0
        lanes={0:[0,1,2],1:[2],2:[0],3:[2],4:[1],5:[1,2],6:[],7:[0,1,2],8:[0,1,2]}[mode]
        for lane in lanes:
            # OR의두피연산자는항상읽는다. 첫Q가비0이어도두번째load를생략하지않는다.
            a=read(cache,0x1d8+lane*32+team*8);b=read(cache,0x1e8+lane*32+team*8)
            pair=a|b;mark('w08_wave_lane_pair_zero',pair,0,'==',pair==0)
            if pair:continue
            header=cache+opponent*32+[0x10,0x90,0xd0][lane]
            length=read(header,0x18)
            if not length:continue
            if length>24 or records+length>24:raise DomainError('W08_WAVE_RECORD_CAPTURE_CAP24_NOT_SOURCE_BOUND')
            vector=read(header);count=0
            for index in range(length):
                entity=read(vector,index*8);x,y=read(entity,0x658),read(entity,0x660)
                distance=d2(x,y,bx,by);inside=distance<=reference
                mark('w08_wave_distance_within_reference',distance,reference,'<=',inside)
                count=(count+inside)&U;records+=1
            accepted=count>=6;mark('w08_wave_qualifying_count_ge6',count,6,'>=',accepted)
            if accepted:return done(1)
        return done(0)
    player,data,candidates,length=args
    if not length:return done(255)
    # 원문은첫3load뒤에team진단분기로간다. 그분기는일부FF/일부panic이다.
    context=read(data,8);team=read(player,0x800);mode=read(context,0x38,1)
    if team>=2:raise DomainError('W08_OBJECTIVE_DIAGNOSTIC_TEAM_DOMAIN_UNSUPPORTED')
    world=read(data);bb=read(data,0x10)
    valid=mode<=8;mark('w08_objective_mode_le8',mode,8,'<=',valid)
    if not valid:return done(255)
    if mode==6:return done(255)
    if length>32:raise DomainError('W08_OBJECTIVE_CANDIDATE_CAPTURE_CAP32_NOT_SOURCE_BOUND')
    answer=255;best=0;bank=(bb+team*0xac8)&U
    for index in range(length):
        key=read(candidates,index,1)
        # 세가지코드경로의admission은같지만load종류/short circuit은아래에서유지한다.
        if mode==2:admitted=key==0
        elif mode==4:admitted=key==1
        elif key==0:admitted=bool(0x185>>mode&1)
        elif key==2:admitted=True
        else:admitted=bool(0x1b1>>mode&1)
        mark('w08_objective_candidate_admitted',int(admitted),0,'!=',admitted)
        if not admitted:continue
        group=0 if key==0 else 2 if key==2 else 1
        lane=(bank+group*40)&U
        rank=read(world,0x2d8+team*8+key*16);lowrank=rank<3
        mark('w08_objective_rank_below3',rank,3,'<',lowrank)
        metadata=read(lane,0x10)
        if not lowrank:
            lowmeta=signed(metadata)<2000
            mark('w08_objective_metadata_below2000',signed(metadata),2000,'<',lowmeta)
            if not lowmeta:
                state=read(lane,0x20,4);lowstate=signed32(state)<2
                mark('w08_objective_state_below2',signed32(state),2,'<',lowstate)
                if not lowstate:continue
        state=read(lane,0x20,4)
        cost=(rank*10000+metadata+signed32(state)*1000)&U
        first=answer==255;improves=signed(cost)<signed(best)
        mark('w08_objective_first_candidate',answer,255,'==',first)
        mark('w08_objective_cost_improves',signed(cost),signed(best),'<',improves)
        if first or improves:answer=key;best=cost
    return done(answer)
