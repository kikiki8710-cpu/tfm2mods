"""typed6 독립모델3개. FILE9 selector표는canonical지원도메인이며loaded값증명아님."""
U=(1<<64)-1;MAGIC1000=0x20c49ba5e353f7cf;MAGIC100=0x28f5c28f5c28f5c3
MODE_OFFSETS=[0x13f8,0x1400,0x1400,0x1400,0x1400,0x1408,0x1400,0x13f8,0x13f8]
METRIC_NAMES={p:['w10_band_mode_nonzero','w10_band_stat_below100','w10_band_low_signed_positive','w10_band_high_signed_le_low','w10_band_raw_high_below_clamped_low','w10_band_bucket_div32_path']for p in[47,48]}
METRIC_NAMES[49]=['w10_tower_tick_before_cutoff','w10_tower_effect_present','w10_tower_structure_modifier_nonzero','w10_tower_own_modifier_nonzero','w10_tower_square_overflow','w10_tower_distance_strict_inside']
ARITIES={47:6,48:6,49:6}
class DomainError(ValueError):pass
def signed32(x):x&=0xffffffff;return x-(1<<32)if x>>31 else x
def div1000(v):return (((v>>3)*MAGIC1000)>>64)>>4
def scaled_radius(modifier,radius):return radius if modifier==0 else (((( ((modifier+100)&U)*radius)&U)>>2)*MAGIC100>>64)>>2
def band_coefficients(stat,mode):
 t=min(stat,100);a=max(50-t,0);b=max(t-50,0)
 if mode:low=(200-2*t+a)&0xffffffff;high=(400-4*t+2*a-b)&0xffffffff
 else:r=max(100-stat,0);low=(6*r+a-b)&0xffffffff;high=(5*r+300+2*(a-b))&0xffffffff
 lo=max(signed32(low),0);hi=lo if signed32(high)<=lo else high
 return lo,hi,signed32(low),signed32(high)
def evaluate(pid,args,memory):
 if pid not in ARITIES:raise DomainError('MODEL_NOT_OWNED')
 if not isinstance(args,list)or len(args)!=6 or any(type(x)is not int or not 0<=x<=U for x in args):raise DomainError('STACKED_ARITY_OR_ARGUMENTS')
 fields=set();metrics=[]
 def read(b,o=0,w=8):
  p=(b+o)&U
  if not 0x10000<=p<p+w<=1<<48:raise DomainError('INPUT_SPAN')
  fields.add((p,w))
  if len(fields)>96:raise DomainError('W10_EED_CAPTURE_UNIQUE_FIELDS96_NOT_SOURCE_BOUND'if pid==49 else'W10_CAPTURE_FIELDS96')
  return memory.read(p,0,w)
 def mark(n,l,r,rel,p):metrics.append(dict(name=n,lhs=l,rhs=r,relation=rel,predicate=bool(p),observed_branch=False))
 def done(value):return dict(expected=value,mask=255 if pid==49 else U,output_ptr=None,output_fields=[],metrics=metrics)
 if pid in[47,48]:
  _,player,data,array,length,modeword=args;stat=read(player,0x1f8);mode=modeword&255
  mark('w10_band_mode_nonzero',mode,0,'!=',mode!=0);mark('w10_band_stat_below100',stat,100,'<',stat<100)
  low,high,raw_low_signed,raw_high_signed=band_coefficients(stat,mode)
  mark('w10_band_low_signed_positive',raw_low_signed,0,'>',raw_low_signed>0)
  mark('w10_band_high_signed_le_low',raw_high_signed,low,'<=',raw_high_signed<=low)
  rawlo=div1000((low*length)&U);rawhi=div1000((high*length)&U);last=(length-1)&U;lo=min(rawlo,last)
  nonstandard=rawhi<lo;mark('w10_band_raw_high_below_clamped_low',rawhi,lo,'<',nonstandard)
  hi=min(rawlo if nonstandard else rawhi,last)
  if hi>=length:raise DomainError('W10_BAND_RANGE_GAME_PANIC')
  end=(hi+1)&U
  if end<lo:raise DomainError('W10_BAND_RANGE_GAME_PANIC')
  span=(end-lo)&U
  if span==0:raise DomainError('W10_BAND_EMPTY_SPAN_GAME_PANIC')
  cache=read(data);context=read(data,8);now=read(cache,0x1b8)
  if pid==47:seed=read(player,0x7f8)
  config=read(context,8);rate=read(config,0x12f8)
  if pid==48:seed=read(player,0x7f8)
  divisor=max((rate*6)&U,1);highbits=(now|divisor)>>32;mark('w10_band_bucket_div32_path',highbits,0,'==',highbits==0)
  mixed=(((now//divisor)<<32)&U)^seed;z=((mixed^(mixed>>33)^0x51ce)*0x9e3779b97f4a7c15)&U;hashed=z^(z>>29)
  index=lo+hashed%span;return done((array+index*(24 if pid==47 else 16))&U)
 context,cache,player,x,y,pad=args
 now=read(cache,0x1b8);mode=read(context,0x38,1);config=read(context,8)
 if mode>=len(MODE_OFFSETS):raise DomainError('W10_EED_MODE_OUTSIDE_PINNED_TABLE9_NOT_SOURCE_BOUND')
 cutoff=read(config,MODE_OFFSETS[mode]);active=now<cutoff;mark('w10_tower_tick_before_cutoff',now,cutoff,'<',active)
 if not active:return done(0)
 team=read(player,0x800)
 if team>=2:raise DomainError('W10_EED_TEAM_GAME_PANIC')
 role=read(player,0x890,4);own=read(cache,0x238+team*40+role*8)
 if not own:return done(0)
 enemy=1-team;base=read(cache,0x1c8+enemy*8)
 fixed=[read(cache,o+enemy*8)for o in[0x1d8,0x1f8,0x218,0x1e8,0x208,0x228]]
 ptr=read(cache,0x170+enemy*32);length=read(cache,0x188+enemy*32);end=(ptr+length*8)&U
 def check(actor):
  niche=read(actor,0x4b8,4);present=niche!=0xffffffff;mark('w10_tower_effect_present',niche,0xffffffff,'!=',present)
  if not present:return False
  raw_mod=read(actor,0x468,4);mod=signed32(raw_mod);mark('w10_tower_structure_modifier_nonzero',mod,0,'!=',mod!=0)
  rad=read(actor,0x678);structure_radius=scaled_radius(mod,rad)
  raw_mod=read(own,0x468,4);mod=signed32(raw_mod);mark('w10_tower_own_modifier_nonzero',mod,0,'!=',mod!=0)
  rad=read(own,0x678);own_radius=scaled_radius(mod,rad)
  level=read(actor,0x5c0);growth=read(actor,0x4a0);base_range=read(actor,0x498);buff=read(actor,0x430)
  reach=(base_range+((pad+15000)&U)+buff+(((level-1)&U)*growth)+structure_radius+own_radius)&U
  boundary=U if reach==U else reach+1;square=boundary*boundary;overflow=square>>64!=0
  mark('w10_tower_square_overflow',square>>64,0,'!=',overflow)
  if overflow:return True
  ax=read(actor,0x658);ay=read(actor,0x660);dx=(x-ax)&U;dy=(y-ay)&U;distance=(dx*dx+dy*dy)&U
  inside=distance<(square&U);mark('w10_tower_distance_strict_inside',distance,square&U,'<',inside);return inside
 for actor in fixed:
  if actor and check(actor):return done(1)
 while ptr!=end:
  actor=read(ptr);ptr=(ptr+8)&U
  if check(actor):return done(1)
 if base and check(base):return done(1)
 return done(0)
