"""부분출력5/6인수와고정4정렬3인수의독립식. 게임helper/쓰기0."""
U=(1<<64)-1;HIGH=1<<63
ARITIES={50:5,51:6,52:3}
METRIC_NAMES={50:['w11_entity_byte_zero','w11_entity_team_lt2','w11_entity_type_is15','w11_entity_direct_slot_match','w11_entity_scan_slot_match','w11_extra_bit_already_set','w11_entity_slice_len_zero'],
51:['w11_nexus_y_ge_min','w11_nexus_x_ge_min','w11_nexus_x_le_max','w11_nexus_y_le_max','w11_nexus_hp_lt_max','w11_opponent_nexus_present','w11_opponent_flag_exact1','w11_opponent_payload_or_zero'],
52:['w11_sort4_pair01_key_lt','w11_sort4_pair23_key_lt','w11_sort4_tails_key_lt','w11_sort4_heads_key_lt','w11_sort4_middle_key_lt']}
class DomainError(ValueError):pass

def evaluate(pid,args,memory):
 if pid not in ARITIES:raise DomainError('MODEL_NOT_OWNED')
 if not isinstance(args,list)or len(args)!=ARITIES[pid]or any(type(x)is not int or not 0<=x<=U for x in args):raise DomainError('OUTPUT11_ARITY_OR_ARGUMENTS')
 out=args[1]if pid==52 else args[0];size={50:24,51:40,52:32}[pid]
 def span(p,w):
  if not 0x10000<=p<p+w<=1<<48:raise DomainError('W11_INPUT_OR_OUTPUT_SPAN')
 span(out,size);memo={};metrics=[]
 def read(b,o=0,w=8):
  p=(b+o)&U;span(p,w)
  if p<out+size and out<p+w:raise DomainError('W11_OUTPUT_INPUT_ALIAS_UNSUPPORTED')
  key=(p,w)
  if key not in memo:
   if len(memo)>=96:raise DomainError('W11_UNIQUE_READ_CAPTURE_CAP96_NOT_SOURCE_BOUND')
   v=memory.read(p,0,w)
   if type(v)is not int or not 0<=v<1<<(w*8):raise DomainError('W11_CAPTURE_VALUE_WIDTH')
   memo[key]=v
  return memo[key]
 def mark(n,l,r,rel,p):metrics.append(dict(name=n,lhs=l,rhs=r,relation=rel,predicate=bool(p),observed_branch=False))
 def done(fields):return dict(expected=0 if pid==52 else out,mask=0 if pid==52 else U,output_ptr=out,
  output_fields=[dict(address=(out+o)&U,width=w,expected=v)for o,w,v in fields],metrics=metrics)
 if pid==50:
  _,world,main,slice_ptr,length=args
  def resolve(entity):
   byte=read(entity,0,1);mark('w11_entity_byte_zero',byte,0,'==',byte==0)
   if byte!=0:return None
   team=read(entity,8);mark('w11_entity_team_lt2',team,2,'<',team<2)
   if team>=2:return None
   rawtype=read(entity,0x68,4);mark('w11_entity_type_is15',rawtype,15,'==',rawtype==15)
   if rawtype==15:
    position=read(entity,0xd8,4);pointer=read(world,0x238+team*40+position*8)
    equal=pointer==entity;mark('w11_entity_direct_slot_match',pointer,entity,'==',equal)
    if equal:return team*5+position
   for position in range(5):
    pointer=read(world,0x238+team*40+position*8);equal=pointer==entity;mark('w11_entity_scan_slot_match',pointer,entity,'==',equal)
    if equal:return team*5+position
   return None
  index=resolve(main)
  if index is None:return done([(0,8,0)])
  mark('w11_entity_slice_len_zero',length,0,'==',length==0)
  mask=0
  if length:
   byte_len=(length*8)&U;offset=0
   while True:
    entity=read(slice_ptr,offset);i=resolve(entity)
    if i is None:return done([(0,8,0)])
    bit=1<<(i&31);already=bool(mask&bit);mark('w11_extra_bit_already_set',int(already),0,'!=',already)
    if already:return done([(0,8,0)])
    mask=(mask|bit)&0xffffffff;offset=(offset+8)&U
    if offset==byte_len:break
  return done([(8,8,index),(16,2,mask&0xffff),(0,8,1)])
 if pid==51:
  _,state,_,_,player,data=args;team=read(player,0x800)
  if team>=2:raise DomainError('W11_NEXUS_TEAM_GAME_PANIC')
  role=read(player,0x890,4);world=read(data);own=read(world,0x238+team*40+role*8)
  if own==0:raise DomainError('W11_NEXUS_OWN_NULL_GAME_PANIC')
  y=read(own,0x660);context=read(data,8);root=read(context,0x20);base=0x6d70+team*32
  ymin=read(root,base+8);inside=y>=ymin;mark('w11_nexus_y_ge_min',y,ymin,'>=',inside)
  if inside:
   x=read(own,0x658);xmin=read(root,base);inside=x>=xmin;mark('w11_nexus_x_ge_min',x,xmin,'>=',inside)
   if inside:
    xmax=read(root,base+16);inside=x<=xmax;mark('w11_nexus_x_le_max',x,xmax,'<=',inside)
    if inside:
     ymax=read(root,base+24);inside=y<=ymax;mark('w11_nexus_y_le_max',y,ymax,'<=',inside)
  if inside:
   hp=read(own,0x668);maximum=read(own,0x620);low=hp<maximum;mark('w11_nexus_hp_lt_max',hp,maximum,'<',low)
   if low:return done([(0,8,HIGH+3)])
  enemy=1-team;nexus=read(world,0x1c8+enemy*8);mark('w11_opponent_nexus_present',nexus,0,'!=',nexus!=0)
  if nexus:
   flag=read(nexus,0x6b1,1);mark('w11_opponent_flag_exact1',flag,1,'==',flag==1)
   if flag==1:
    length=read(world,0x188+enemy*32);q=read(nexus,0x698);joined=length|q;mark('w11_opponent_payload_or_zero',joined,0,'==',joined==0)
    if joined==0:return done([(0,8,HIGH+14)])
  byte=read(state,8,1);return done([(8,8,0),(24,8,0),(32,2,0),(34,1,byte),(35,1,2),(0,8,HIGH)])
 source,_,capture=args
 i1=read(source,8);header=read(capture);length=read(header,24)
 if i1>=length:raise DomainError('W11_SORT4_INDEX1_GAME_PANIC')
 i0=read(source)
 if i0>=length:raise DomainError('W11_SORT4_INDEX0_GAME_PANIC')
 array=read(header);a1=read(array,(i1*8)&U);x1=read(a1,0x658);y1=read(a1,0x660)
 reference=read(capture,8);ry=read(reference,0x660);rx=read(reference,0x658)
 a0=read(array,(i0*8)&U);x0=read(a0,0x658);y0=read(a0,0x660)
 i3=read(source,24)
 if i3>=length:raise DomainError('W11_SORT4_INDEX3_GAME_PANIC')
 i2=read(source,16)
 if i2>=length:raise DomainError('W11_SORT4_INDEX2_GAME_PANIC')
 a3=read(array,(i3*8)&U);x3=read(a3,0x658);y3=read(a3,0x660)
 a2=read(array,(i2*8)&U);x2=read(a2,0x658);y2=read(a2,0x660)
 def key(x,y):return (((x-rx)&U)**2+((y-ry)&U)**2)&U
 keys=[key(x0,y0),key(x1,y1),key(x2,y2),key(x3,y3)];values=[i0,i1,i2,i3]
 b=keys[0]<keys[1];d=keys[2]<keys[3];mark('w11_sort4_pair01_key_lt',keys[0],keys[1],'<',b);mark('w11_sort4_pair23_key_lt',keys[2],keys[3],'<',d)
 lh,lt=(1,0)if b else(0,1);rh,rt=(3,2)if d else(2,3)
 tails=keys[lt]<keys[rt];heads=keys[lh]<keys[rh];mark('w11_sort4_tails_key_lt',keys[lt],keys[rt],'<',tails);mark('w11_sort4_heads_key_lt',keys[lh],keys[rh],'<',heads)
 mid0=rh if tails else lt;mid0=lh if heads else mid0
 mid1=lt if heads else rh;mid1=rt if tails else mid1
 swap=keys[mid0]<keys[mid1];mark('w11_sort4_middle_key_lt',keys[mid0],keys[mid1],'<',swap)
 ordered=[rh if heads else lh,mid1 if swap else mid0,mid0 if swap else mid1,lt if tails else rt]
 return done([(n*8,8,values[k])for n,k in enumerate(ordered)])
