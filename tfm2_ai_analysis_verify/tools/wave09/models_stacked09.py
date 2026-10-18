"""wave09의5/5/6인수순수모델. 게임함수호출/외부쓰기/생략인수기본값은없다."""
U=(1<<64)-1
ARITIES={44:5,45:5,46:6}
METRIC_NAMES={44:['w09_move_y_ge_rect_min','w09_move_x_ge_rect_min','w09_move_x_le_rect_max','w09_move_y_le_rect_max','w09_move_now_ge_expiry','w09_move_previous_d2_le_100m'],
45:['w09_selector_lane_is0','w09_selector_lane_is1','w09_selector_diagonal_side','w09_selector_index_lt6'],
46:['w09_count_hp_div32_path','w09_count_hp_ge_input','w09_count_d2_le_radius2']}
class DomainError(ValueError):pass
def evaluate(pid,args,memory):
    if pid not in ARITIES:raise DomainError('MODEL_NOT_OWNED')
    if not isinstance(args,list)or len(args)!=ARITIES[pid]or any(type(v)is not int or not 0<=v<=U for v in args):raise DomainError('STACKED_ARITY_OR_ARGUMENTS')
    metrics=[];fields=set()
    def read(base,offset=0,width=8):
        p=(base+offset)&U
        if not 0x10000<=p<p+width<=1<<48:raise DomainError('INPUT_SPAN')
        fields.add((p,width))
        if len(fields)>96:raise DomainError('W09_FIELD_CAPTURE_CAP96')
        return memory.read(p,0,width)
    def mark(name,lhs,rhs,relation,predicate):metrics.append(dict(name=name,lhs=lhs,rhs=rhs,relation=relation,predicate=bool(predicate),observed_branch=False))
    def done(value):return dict(expected=value,mask=255 if pid==44 else U,output_ptr=None,output_fields=[],metrics=metrics)
    def d2(x,y,ox,oy):return (((x-ox)&U)**2+((y-oy)&U)**2)&U
    if pid==44:
        capture,_,_,player,data=args
        team=read(player,0x800)
        if team>=2:raise DomainError('W09_MOVE_TEAM_GAME_PANIC')
        role=read(player,0x890,4);cache=read(data)
        actor=read(cache,0x238+team*40+role*8)
        if not actor:raise DomainError('W09_MOVE_ACTOR_NULL_GAME_PANIC')
        x=read(actor,0x658);y=read(actor,0x660)
        context=read(data,8);config=read(context,0x20);rect=config+team*32+0x6d70
        lowy=read(rect,8);inside=False;mark('w09_move_y_ge_rect_min',y,lowy,'>=',y>=lowy)
        if y>=lowy:
            lowx=read(rect);mark('w09_move_x_ge_rect_min',x,lowx,'>=',x>=lowx)
            if x>=lowx:
                highx=read(rect,16);mark('w09_move_x_le_rect_max',x,highx,'<=',x<=highx)
                if x<=highx:
                    highy=read(rect,24);inside=y<=highy;mark('w09_move_y_le_rect_max',y,highy,'<=',inside)
        duration=read(capture,0x78);stamp=read(capture,0x60);expiry=(duration+stamp)&U
        now=read(cache,0x1b8);elapsed=now>=expiry;mark('w09_move_now_ge_expiry',now,expiry,'>=',elapsed)
        if elapsed:return done(1)
        oldx=read(capture,0x68);oldy=read(capture,0x70);distance=d2(x,y,oldx,oldy);near=distance<=100000000
        mark('w09_move_previous_d2_le_100m',distance,100000000,'<=',near)
        return done(int(inside or near))
    if pid==45:
        lane,team,position,world,data=args;lane&=255
        zero=lane==0;mark('w09_selector_lane_is0',lane,0,'==',zero)
        if not zero:mark('w09_selector_lane_is1',lane,1,'==',lane==1)
        if team>=2:raise DomainError('W09_SELECTOR_TEAM_GAME_PANIC')
        if lane==0:
            table=[16,6,3,3,3,2,2]if team==0 else[2,3,6,6,6,16,16];offset=0x2d8
        elif lane==1:
            actor=read(world,0x238+team*40+(position&0xffffffff)*8)
            if not actor:raise DomainError('W09_SELECTOR_ACTOR_NULL_GAME_PANIC')
            config=read(data,8);diagonal=read(config,0x12c0);y=read(actor,0x660);cut=(diagonal-y)&U;x=read(actor,0x658)
            side=int(cut<x);mark('w09_selector_diagonal_side',cut,x,'<',bool(side))
            a=11+3*side;b=17+4*side;c=4+5*side
            table=[b,a,a,a,a,c,c]if team==0 else[c,a,a,a,a,b,b];offset=0x2e8
        else:
            table=[21,20,15,15,15,9,7]if team==0 else[9,15,20,20,20,21,23];offset=0x2f8
        index=read(world,offset+team*8);mark('w09_selector_index_lt6',index,6,'<',index<6)
        return done(table[min(index,6)])
    player,data,x,y,radius,threshold=args
    team=read(player,0x800)
    if team>=2:raise DomainError('W09_COUNT_TEAM_GAME_PANIC')
    radius2=(radius*radius)&U;cache=read(data);count=0
    for role in range(5):
        actor=read(cache,0x238+team*40+role*8)
        if not actor:continue
        maximum=read(actor,0x620)
        if not maximum:raise DomainError('W09_COUNT_MAXHP_ZERO_GAME_PANIC')
        hp=read(actor,0x668);numerator=(hp*100)&U;fast=((numerator|maximum)>>32)==0
        mark('w09_count_hp_div32_path',(numerator|maximum)>>32,0,'==',fast)
        pct=numerator//maximum;healthy=pct>=threshold;mark('w09_count_hp_ge_input',pct,threshold,'>=',healthy)
        if not healthy:continue
        ax=read(actor,0x658);ay=read(actor,0x660);distance=d2(ax,ay,x,y);near=distance<=radius2
        mark('w09_count_d2_le_radius2',distance,radius2,'<=',near)
        count+=int(near)
    return done(count)
