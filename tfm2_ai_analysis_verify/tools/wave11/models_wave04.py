"""wave04독립Python식. scratch범위/게임실행0/로그expected를오라클로사용하지않음."""
MASK=(1<<64)-1
class DomainError(ValueError):pass
def wrap(x):return x&MASK
def signed(x,bits=64):return x-(1<<bits)if x&(1<<(bits-1))else x
def div5(x):return x//5 if x>=0 else-((-x)//5)
class Memory:
    def __init__(self,fields):
        self.fields={(f['address'],f['width']):f['value']for f in fields};self.used=[]
    def read(self,b,o=0,w=8):
        key=(wrap(b+o),w)
        if key not in self.fields:raise DomainError('MISSING_READ')
        if key not in self.used:
            if len(self.used)>=96:raise DomainError('FIELD_BUDGET')
            self.used.append(key)
        return self.fields[key]&((1<<(w*8))-1)
def evaluate(pid,a,m):
    r=m.read;metrics=[]
    def metric(name,lhs,rhs,relation,predicate):metrics.append(dict(name=name,lhs=lhs,rhs=rhs,relation=relation,predicate=bool(predicate),observed_branch=False))
    def out(x,mask=255):return dict(expected=wrap(x),mask=mask,metrics=metrics,output_fields=[])
    if pid==19:
        count=r(a[0],0x840);metric('w04_lookup_count_zero',count,0,'==',count==0)
        if not count:return out(0,MASK)
        for i in range(12):
            key=r(a[0],i*176+0x90);same=key==a[1];metric('w04_lookup_id_equal',key,a[1],'==',same)
            if same:return out(a[0]+i*176,MASK)
            ended=count==i+1;metric('w04_lookup_end_equals_count',count,i+1,'==',ended)
            if ended:return out(0,MASK)
        raise DomainError('E1B430_BOUNDS_PANIC_AFTER_TWELFTH_MISS')
    if pid==20:
        t=a[1]
        if not t:return out(r(a[0],0x98),MASK)
        n=r(a[0],0xa0);k=t//5
        if n>18:raise DomainError('E1EDE0_CAPTURE_COUNT_ABOVE18')
        metric('w04_forecast_index_ge_count',k,n,'>=',k>=n)
        if k>=n:return out(r(a[0],0x98 if not n else(n-1)*8),MASK)
        metric('w04_forecast_time_lt5',t,5,'<',t<5)
        if t<5:after,before,rem=r(a[0]),r(a[0],0x98),t
        else:
            if k==0 or k-1>17 or t>=90:raise DomainError('E1EDE0_INTERPOLATION_PANIC_DOMAIN')
            before,after,rem=r(a[0],(k-1)*8),r(a[0],k*8),t-k*5
        return out(before+div5(signed(wrap(wrap(after-before)*rem))),MASK)
    if pid==21:
        start,end=r(a[0],0x18),r(a[0],0x20)
        if start>=end:return out(0,MASK)
        if end-start>32:raise DomainError('D81CE0_CAPTURE_RANGE_ABOVE32')
        teamref,excludedref,world=r(a[0],0x10),r(a[0]),r(a[0],8);excluded=r(excludedref);count=0
        for i in range(start,end):
            skip=i==excluded;metric('w04_count_excluded_id',i,excluded,'==',skip)
            if skip:continue
            team=r(teamref)
            if team>1:raise DomainError('D81CE0_TEAM_BOUNDS_PANIC')
            if i>=5:raise DomainError('D81CE0_ROLE_BOUNDS_PANIC')
            ptr=r(world,0x238+team*40+i*8);present=ptr!=0;metric('w04_count_slot_present',ptr,0,'!=',present);count+=present
        return out(count,MASK)
    if pid==22:
        start,end=r(a[0],0x10),r(a[0],0x18)
        if start>=end:return out(0,MASK)
        teamref=r(a[0],8);opp=wrap(1-r(teamref))
        if opp>1:raise DomainError('D81BD0_TEAM_BOUNDS_PANIC')
        if start>=5:raise DomainError('D81BD0_START_BOUNDS_PANIC')
        if end>5:raise DomainError('D81BD0_CAPTURE_RANGE_END_ABOVE5')
        world=r(a[0]);count=0
        for i in range(start,end):
            ptr=r(world,0x238+opp*40+i*8);present=ptr!=0;metric('w04_opponent_slot_present',ptr,0,'!=',present);count+=present
        return out(count,MASK)
    if pid==23:
        context=r(a[1],8);mode=r(context,0x38,1);line=a[2]&255;slot=0 if line==0 else 1 if line==1 else 2
        allowed=mode<9 and([0x185,0x1b1,0x1ab][slot]>>mode)&1
        metric('w04_pressure_mode_lt9',mode,9,'<',mode<9);metric('w04_pressure_mode_allowed',int(bool(allowed)),1,'==',bool(allowed))
        if not allowed:return out(0)
        team=r(a[0],0x800)
        if team>=2:raise DomainError('E54190_TEAM_BOUNDS_PANIC')
        bb=r(a[1],0x10);row=wrap(bb+team*0xac8+slot*40);progress=signed(r(row,0x10))
        metric('w04_pressure_progress_le_minus3001',progress,-3001,'<=',progress<=-3001)
        if progress> -3001:return out(0)
        count=signed(r(row,0x20,4),32);metric('w04_pressure_count_lt_minus3',count,-3,'<',count< -3)
        return out(int(count< -3))
    if pid in (24,25):
        if pid==24:
            outer=r(a[0]);capture=r(outer);teamref=r(capture,8);team=r(teamref)
            if team>1:raise DomainError('FFA970_TEAM_BOUNDS_PANIC')
            index=r(a[1])
            if index>=5:raise DomainError('FFA970_ROLE_BOUNDS_PANIC')
            data=r(capture);world=r(data)
            if not r(world,0x238+team*40+index*8):return out(0)
        else:
            index=a[1]
            if index>4:raise DomainError('FD4260_ID_BOUNDS_PANIC')
            wrapper=r(a[0]);excluded=r(wrapper)
            if r(excluded,index,1):return out(0)
            data=r(wrapper,8)
        if index not in (0,2,3):return out(0)
        role={0:0,2:2,3:3}[index];context=r(data,8);mode=r(context,0x38,1)
        mask,offset={0:(0x185,0),2:(0x1b1,32),3:(0x1ab,64)}[role]
        if mode>=9 or not(mask>>mode)&1:return out(0)
        if pid==25:world=r(data)
        first=r(world,0x1d8+offset)
        if pid==24:metric('w04_lane_open_first_absent',first,0,'==',first==0)
        if first:
            second=r(world,0x1e0+offset)
            if pid==24:metric('w04_lane_open_second_absent',second,0,'==',second==0)
            if second:return out(0)
        if pid==24:return out(1)
        teamref=r(wrapper,0x10);team=r(teamref)
        if team>=2:raise DomainError('FD4260_WRAPPER_TEAM_BOUNDS_PANIC')
        if not r(world,0x238+team*40+index*8):return out(0)
        teamref=r(a[0],0x10);team=r(teamref)
        if team>=2:raise DomainError('FD4260_CAPTURE_TEAM_BOUNDS_PANIC')
        worldb=r(a[0],8);entity=r(worldb,0x238+team*40+index*8)
        if not entity:return out(0)
        if r(entity,0x68,4)==15 and(r(entity,0x70)&0xffffffff)==1:return out(0)
        maximum=max(1,r(entity,0x620));pct=wrap(r(entity,0x668)*100)//maximum
        metric('w04_candidate_hp_percent_ge30',pct,30,'>=',pct>=30)
        return out(int(pct>=30))
    if pid==26:
        begin,end=r(a[0]),r(a[0],8)
        if begin==end:return out(0,MASK)
        if end<begin or(end-begin)&7:raise DomainError('DC0D80_INVALID_CAPTURE_SLICE')
        n=(end-begin)//8
        if n>16:raise DomainError('DC0D80_CAPTURE_LENGTH_ABOVE16')
        eventref,context=r(a[0],0x10),r(a[0],0x18);event=r(eventref,0,1);settings=r(context,8);count=0
        for i in range(n):
            entity=r(begin,i*8)
            if not entity:continue
            maximum=r(entity,0x620)
            if not maximum:raise DomainError('DC0D80_DIVISOR_ZERO_GAME_PANIC_DOMAIN')
            if event not in(4,5):continue
            pct=wrap(r(entity,0x668)*100)//maximum;metric('w04_geometry_hp_percent_ge50',pct,50,'>=',pct>=50)
            if pct<50:continue
            x=r(entity,0x658)
            if event==4:
                metric('w04_geometry_tag4_x_le192000',x,192000,'<=',x<=192000)
                if x<=192000:count+=1;continue
                height=r(settings,0x12c0);q=wrap(height-r(entity,0x660));metric('w04_geometry_tag4_x_le_q',x,q,'<=',x<=q)
                if x<=q:count+=1;continue
                cut=wrap(r(settings,0x12b8)-192000);metric('w04_geometry_tag4_q_ge_cut',q,cut,'>=',q>=cut)
                if q>=cut:count+=1;continue
                gap=wrap(x-q);metric('w04_geometry_tag4_gap_lt64000',gap,64000,'<',gap<64000);count+=gap<64000
            else:
                height=r(settings,0x12c0);q=wrap(height-r(entity,0x660))
                metric('w04_geometry_tag5_q_le_x',q,x,'<=',q<=x);metric('w04_geometry_tag5_q_le192000',q,192000,'<=',q<=192000)
                if q<=x or q<=192000:count+=1;continue
                cut=wrap(r(settings,0x12b8)-192000);metric('w04_geometry_tag5_x_ge_cut',x,cut,'>=',x>=cut)
                if x>=cut:count+=1;continue
                gap=wrap(q-x);metric('w04_geometry_tag5_gap_lt64000',gap,64000,'<',gap<64000);count+=gap<64000
        return out(count,MASK)
    raise DomainError('W04_UNSUPPORTED_ID')
