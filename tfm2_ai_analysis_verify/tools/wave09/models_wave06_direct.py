"""wave06 direct2 scratch. 저장Memory만사용;game/helper0;ABI설치적격은별도."""
MASK=(1<<64)-1
METRIC_NAMES={37:['w06_effect_action_kind_allowed','w06_effect_level_ge3','w06_effect_level_ge5','w06_effect_kind_present','w06_effect_action_match'],38:['w06_iterator_state_zero','w06_iterator_line_equal','w06_iterator_entity_present']}
class DomainError(ValueError):pass
def u(x):return x&MASK
def span(a,w):
 if type(a)is not int or type(w)is not int or not 0x10000<=a<a+w<=1<<48:raise DomainError('ADDRESS_SPAN')
 return(a,a+w)
def overlap(a,b):return a[0]<b[1]and b[0]<a[1]
class Memory:
 def __init__(self,rows):self.fields={(r['address'],r['width']):r['value']for r in rows};self.used=[]
 def read(self,b,offset=0,width=8):
  k=(u(b+offset),width)
  if k not in self.fields:raise DomainError('MISSING_READ')
  if k not in self.used:
   if len(self.used)>=96:raise DomainError('FIELD_BUDGET')
   self.used.append(k)
  return self.fields[k]&((1<<(8*width))-1)
def evaluate(pid,args,m):
 if not isinstance(args,list)or len(args)!=4 or any(type(a)is not int or not 0<=a<=MASK for a in args):raise DomainError('ARGUMENTS')
 a=args;metrics=[];outbase=a[0];output=span(outbase,17 if pid==37 else 24);cursor=None
 def met(n,l,r,rel,p):metrics.append(dict(name=n,lhs=l,rhs=r,relation=rel,predicate=bool(p),observed_branch=False))
 def read(b,o=0,w=8,initial_cursor=False):
  address=u(b+o);s=span(address,w)
  if overlap(s,output):raise DomainError('OUTPUT_INPUT_ALIAS')
  if cursor is not None and not initial_cursor and overlap(s,cursor):raise DomainError('CURSOR_INPUT_ALIAS')
  return m.read(address,0,w)
 def done(expected,mask,fields):return dict(expected=u(expected),mask=mask,output_fields=[dict(address=p,width=w,expected=u(v))for p,w,v in fields],metrics=metrics)
 if pid==37:
  k=u(a[2]-6);allowed=k<=3;met('w06_effect_action_kind_allowed',k,3,'<=',allowed)
  if not allowed:return done(0,0,[(u(outbase+16),1,2)])
  actor=a[1];kind=a[2]
  if kind in[6,7]:effect=u(actor+(0x488 if kind==6 else 0x4c0));tag=read(effect,0x30,4)
  else:
   level=read(actor,0x5c0);minimum=3 if kind==8 else 5;ready=level>=minimum
   met('w06_effect_level_ge3'if kind==8 else'w06_effect_level_ge5',level,minimum,'>=',ready)
   if not ready:raise DomainError('STATIC_DESCRIPTOR_REQUIRES_LOADED_CAPTURE')
   effect=u(actor+(0x4f8 if kind==8 else 0x530));tag=read(effect,0x30,4)
  present=tag!=0xffffffff;met('w06_effect_kind_present',tag,0xffffffff,'!=',present)
  if not present:raise DomainError('E20D00_EFFECT_KIND_NONE_GAME_PANIC')
  current_kind,current_state=read(actor,0x68),read(actor,0x70);difference=(current_kind^15)|(current_state^(kind-3));matches=difference==0
  met('w06_effect_action_match',difference,0,'==',matches)
  return done(effect,MASK,[(outbase,8,effect),(u(outbase+8),8,a[3]),(u(outbase+16),1,int(matches))])
 if pid==38:
  iterator=a[1];begin,end=read(iterator),read(iterator,8)
  if begin==end:return done(outbase,MASK,[(outbase,8,0)])
  if end<begin or end-begin>32:raise DomainError('ITERATOR_CAPTURE_LIMIT32_NOT_SOURCE_BOUND')
  cursor=span(iterator,8)
  wrapper=read(a[2]);teamref_a=read(wrapper,8);team_a=read(teamref_a)
  if team_a>1:raise DomainError('ITERATOR_BB_TEAM_PANIC_AFTER_CURSOR_WRITE')
  lineref=read(a[2],8);world=read(a[3]);teamref_b=read(a[3],8);bb=read(wrapper);bank=u(bb+team_a*0xac8);index=begin
  while index!=end:
   key=read(iterator,u(0x10+index*4),4);state=read(bank,key*32+0x7f0,1);zero=state==0
   met('w06_iterator_state_zero',state,0,'==',zero)
   if zero:
    line,wanted=read(bank,key*32+0x7f1,1),read(lineref,0,1);same=line==wanted;met('w06_iterator_line_equal',line,wanted,'==',same)
    if same:
     team_b=read(teamref_b)
     if team_b>1:raise DomainError('ITERATOR_WORLD_TEAM_PANIC_AFTER_CURSOR_WRITE')
     entity=read(world,0x238+team_b*40+key*8);met('w06_iterator_entity_present',entity,0,'!=',entity!=0)
     if entity:
      index=u(index+1);reference=read(a[3],0x10);rx,ry=read(reference,0x658),read(reference,0x660);ex,ey=read(entity,0x658),read(entity,0x660)
      d2=u(u(rx-ex)**2+u(ry-ey)**2)
      return done(outbase,MASK,[(iterator,8,index),(u(outbase+8),8,d2),(u(outbase+16),8,key),(outbase,8,1)])
   index=u(index+1)
  return done(outbase,MASK,[(iterator,8,end),(outbase,8,0)])
 raise DomainError('MODEL_NOT_OWNED')
