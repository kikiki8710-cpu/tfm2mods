"""0.6.4 고정본문 독립식. 게임 호출0. 입력 read 순서·폭과 출력8B를 보존한다."""
METRIC_NAMES={27:['w05_history_recent_after_cutoff','w05_history_row_after_cutoff','w05_history_tag_equal','w05_history_distance_squared'],28:['w05_economy_remaining_percent_lt45','w05_economy_remaining_percent_lt25','w05_economy_damage_percent_gt11'],32:['w05_cursor_excluded_equal','w05_cursor_entity_present','w05_cursor_line_equal'],33:['w05_cursor_excluded_equal','w05_cursor_entity_present','w05_cursor_line_equal']}
METRIC_NAMES[34]=['w05_cursor_excluded_equal','w05_cursor_line_equal','w05_cursor_entity_present','w05_cursor_near_distance_shift8']
for _id in[35,36]:METRIC_NAMES[_id]=['w05_cursor_skip_role4','w05_cursor_excluded_equal','w05_cursor_entity_present','w05_cursor_line_equal']
MASK=(1<<64)-1;MIN=-(1<<63)
class DomainError(ValueError):pass
def u(x):return x&MASK
def s(x):
 x=u(x);return x-(1<<64)if x>>63 else x
def trunc(n,d):
 if not d or(n==MIN and d==-1):raise DomainError('SIGNED_DIV_PANIC')
 return(abs(n)//abs(d))*(-1 if(n<0)!=(d<0)else 1)
def mixeddiv(n,d):
 n=u(n);d=u(d)
 if not d:raise DomainError('ZERO_DIV_PANIC')
 return n//d if(n|d)>>32==0 else trunc(s(n),s(d))
class Memory:
 def __init__(self,fields):
  self.fields={(f['address'],f['width']):f['value']for f in fields};self.used=[]
 def read(self,b,offset=0,width=8):
  k=(u(b+offset),width)
  if k not in self.fields:raise DomainError('MISSING_READ')
  if k not in self.used:
   if len(self.used)>=96:raise DomainError('FIELD_BUDGET')
   self.used.append(k)
  return self.fields[k]&((1<<(8*width))-1)
def evaluate(pid,a,m):
 r=m.read;metrics=[]
 def metric(name,lhs,rhs,relation,predicate):metrics.append(dict(name=name,lhs=lhs,rhs=rhs,relation=relation,predicate=bool(predicate),observed_branch=False))
 def out(x,mask=MASK,fields=None):return dict(expected=u(x),mask=mask,metrics=metrics,output_fields=fields or[])
 if pid==27:
  cap=r(a[0]);player=r(cap,8);own=r(player,0x800);opp=u(1-own)
  if opp>1:raise DomainError('EA07F0_TEAM_BOUNDS')
  role=a[1]
  if role>=5:raise DomainError('EA07F0_ROLE_BOUNDS')
  data=r(cap);world=r(data);entity=r(world,0x238+opp*40+role*8)
  if not entity:return out(0)
  bb=r(data,0x10);bank=u(bb+opp*0xac8);clock=r(world,0x1b8);cutoff=max(clock-20,0)
  sub=r(bank,role*32+0x7f1,1);lo=r(bank,role*32+0x7f8);hi=r(bank,role*32+0x800);stamp=r(bank,role*32+0x7e8)
  metric('w05_history_recent_after_cutoff',stamp,cutoff,'>',stamp>cutoff)
  if stamp<=cutoff:tag=r(bank,role*32+0x7f0,1)
  else:
   selected=None
   for i in range(4):
    row=u(bank+role*0xe0+0x388+i*56);time=r(row,0x18);before=r(row,0,1);after=r(row,0x20,1)
    if time==0 and before==255 and after==255:continue
    metric('w05_history_row_after_cutoff',time,cutoff,'>',time>cutoff)
    if time>cutoff:
     if selected is None:selected=(before,r(row,1,1),r(row,8),r(row,0x10))
     break
    selected=(after,r(row,0x21,1),r(row,0x28),r(row,0x30))
   if selected is None:
    record=u(bank+0x310+role*24);selected=(r(record,0,1),r(record,1,1),r(record,8),r(record,0x10))
   tag,sub,lo,hi=selected
  if tag==255:return out(0)
  request=r(cap,0x18);requested=r(request,0,1)
  metric('w05_history_tag_equal',tag,requested,'==',tag==requested)
  if tag!=requested:return out(0)
  if tag<=5:
   reqsub=r(request,1,1);reqlo=r(request,8)
   if tag==0 and sub!=reqsub:return out(0)
   if tag==1 and(sub!=reqsub or lo!=reqlo):return out(0)
   if tag==4 and lo!=reqlo:return out(0)
   if tag==5:
    if lo&1:
     reqhi=r(request,0x10)
     if hi!=reqhi or not(reqlo&1):return out(0)
    elif reqlo&1:return out(0)
  x,y=r(entity,0x658),r(entity,0x660);xref,yref=r(cap,0x20),r(cap,0x28);cx,cy=r(xref),r(yref)
  d2=u(u(x-cx)**2+u(y-cy)**2);inside=d2<62500000001
  metric('w05_history_distance_squared',d2,62500000001,'<',inside)
  return out(entity if inside else 0)
 if pid==28:
  if r(a[0],0x480,1):return out(0)
  maximum,hp=r(a[0],0x620),r(a[0],0x668);den=maximum+(maximum==0);damage,scale=a[1],a[2]
  remain=max(s(u(hp-damage)),0);n=u(remain*100);q=mixeddiv(n,den)
  metric('w05_economy_remaining_percent_lt45',q,45,'<',q<45)
  result=0
  if q<45:
   result=trunc(s(u((45-q)*scale)),60)
   metric('w05_economy_remaining_percent_lt25',q,25,'<',q<25)
   if q<25:result=u(result+trunc(s(scale),5))
  damagepct=mixeddiv(u(damage*100),den)
  metric('w05_economy_damage_percent_gt11',damagepct,11,'>',damagepct>11)
  if damagepct>11:
   ratio=mixeddiv(u(scale*damage),hp+(hp==0));result=u(result+trunc(ratio,2))
  return out(result)
 if pid in[32,33,34,35,36]:
  cursor=a[0];begin,end=r(cursor),r(cursor,8)
  empty=begin==end if pid==32 else begin>=end
  if empty:return out(0,255)
  if pid==32:
   if end<begin or(end-begin)%8 or(end-begin)//8>32:raise DomainError('CURSOR_ARRAY_CAPTURE_LIMIT32')
  elif end-begin>32:raise DomainError('CURSOR_RANGE_CAPTURE_LIMIT32')
  def rr(base,offset=0,width=8):
   addr=u(base+offset)
   if addr+width>1<<64 or cursor+8>1<<64:raise DomainError('ADDRESS_SPAN_WRAP')
   if addr<cursor+8 and cursor<addr+width:raise DomainError('CURSOR_INPUT_ALIAS')
   return r(base,offset,width)
  cap=a[1];teamref=rr(cap,0x18)
  if pid==32:world,excludedref,bb,lineref=rr(cap,8),rr(cap),rr(cap,0x10),rr(cap,0x20)
  elif pid==34:bb,lineref,excludedref,world,anchor=rr(cap,0x10),rr(cap,0x20),rr(cap),rr(cap,8),rr(cap,0x28)
  else:excludedref,world,bb,lineref=rr(cap),rr(cap,8),rr(cap,0x10),rr(cap,0x20)
  excluded=rr(excludedref);nextvalue=begin;found=False
  while nextvalue!=end:
   key=rr(nextvalue)if pid==32 else nextvalue;nextvalue=u(nextvalue+(8 if pid==32 else 1))
   if pid in[35,36]:
    fixed=key==4;metric('w05_cursor_skip_role4',key,4,'==',fixed)
    if fixed:continue
   skip=key==excluded;metric('w05_cursor_excluded_equal',key,excluded,'==',skip)
   if skip:continue
   team=rr(teamref)
   if team>1:raise DomainError('CURSOR_TEAM_BOUNDS_PANIC_WRITES_NEXT')
   if key>=5:raise DomainError('CURSOR_ID_BOUNDS_PANIC_WRITES_NEXT')
   if pid==34:
    if rr(bb,team*0xac8+key*32+0x7f0,1):continue
    wanted=rr(lineref,0,1);line=rr(bb,team*0xac8+key*32+0x7f1,1);equal=line==wanted;metric('w05_cursor_line_equal',line,wanted,'==',equal)
    if not equal:continue
   present=rr(world,0x238+team*40+key*8)
   metric('w05_cursor_entity_present',present,0,'!=',present!=0)
   if not present:continue
   if pid==34:
    x,y,ax,ay=rr(present,0x658),rr(present,0x660),rr(anchor,0x658),rr(anchor,0x660);distance=u(u(x-ax)**2+u(y-ay)**2)>>8;near=distance<87890625
    metric('w05_cursor_near_distance_shift8',distance,87890625,'<',near)
    if near:found=True;break
    continue
   state=rr(bb,team*0xac8+key*32+0x7f0,1)
   if state:continue
   line=rr(bb,team*0xac8+key*32+0x7f1,1);wanted=rr(lineref,0x11c if pid==32 else 0,1)
   equal=line==wanted;metric('w05_cursor_line_equal',line,wanted,'==',equal)
   if equal:found=True;break
  return out(int(found),255,[dict(address=cursor,width=8,expected=nextvalue)])
 raise DomainError('MODEL_NOT_OWNED')
