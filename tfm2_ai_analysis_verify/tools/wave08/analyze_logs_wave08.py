"""wave08의44모델을 저장 read 값만으로 독립 재계산하는 오프라인 분석기.

게임/모드/설정/원본함수를 호출하지 않는다. Rust 모델을 실행하거나 로그 expected를 정답으로 쓰지 않는다.
"""
from __future__ import annotations
import argparse, collections, hashlib, importlib.util, json
from pathlib import Path

MASK=(1<<64)-1
BASELINE='a1d05045405a12e2bed182daeafcad8c8a40cf29ab68ba6ea5bfd30257439582'
RVAS=[14518176, 15019744, 14257104, 14982144, 16424384, 16064080, 15330160, 16628128, 14420912, 14421888, 15890064, 15890528, 15672336, 16422400, 16881808, 16656128, 16587248, 16594928, 16682800, 14791728, 14806496, 14163168, 14162896, 15024528, 16755056, 16597600, 14421376]
RVAS += [15337456, 14812400, 14415664, 14200000, 14200528, 13881296, 13882416, 13881552, 13881904, 13882160]
RVAS += [0xe20d00,0x1014470]
RVAS += [0xd8a1f0,0xd3eec0]
RVAS += [14206592, 16563120, 14943584]
METRIC_NAMES={0: ['forecast_remainder_is_zero'], 1: [], 2: [], 3: ['all_near_nexus_radius_squared'], 4: ['same_lane_radius_shift8'], 5: ['first_stamp_recent', 'second_stamp_expired'], 6: ['record_hp_minimum_percent'], 7: [], 8: ['hp_percent_ge40'], 9: ['hp_percent_ge50'], 10: ['hp_percent_ge50'], 11: ['hp_percent_ge40'], 12: ['opponent_except_role1_d2'], 13: ['same_anchor_id', 'role_radius_squared_inclusive'], 14: ['middle_minimum_hp40', 'middle_tag4_x_le192000', 'middle_tag4_x_le_q', 'middle_tag4_q_ge_width_minus192000', 'middle_tag4_gap_lt64000', 'middle_tag5_q_le_x', 'middle_tag5_q_le192000', 'middle_tag5_x_ge_width_minus192000', 'middle_tag5_gap_lt64000'], 15: ['forecast_tail_index_ge_count', 'forecast_first_interval_lt5'], 16: ['optional_scale_tag_exact1', 'optional_scale_signed_cap'], 17: ['scaled_delta_positive_after_wrap'], 18: ['lane_record26_nonzero', 'lane_record27_signed_positive', 'lane_record25_nonzero', 'lane_opponent_structure_present'], 19: ['w04_lookup_count_zero', 'w04_lookup_id_equal', 'w04_lookup_end_equals_count'], 20: ['w04_forecast_index_ge_count', 'w04_forecast_time_lt5'], 21: ['w04_count_excluded_id', 'w04_count_slot_present'], 22: ['w04_opponent_slot_present'], 23: ['w04_pressure_mode_lt9', 'w04_pressure_mode_allowed', 'w04_pressure_progress_le_minus3001', 'w04_pressure_count_lt_minus3'], 24: ['w04_lane_open_first_absent', 'w04_lane_open_second_absent'], 25: ['w04_candidate_hp_percent_ge30'], 26: ['w04_geometry_hp_percent_ge50', 'w04_geometry_tag4_x_le192000', 'w04_geometry_tag4_x_le_q', 'w04_geometry_tag4_q_ge_cut', 'w04_geometry_tag4_gap_lt64000', 'w04_geometry_tag5_q_le_x', 'w04_geometry_tag5_q_le192000', 'w04_geometry_tag5_x_ge_cut', 'w04_geometry_tag5_gap_lt64000']}


class ModelDomainError(ValueError):
    """실제 반환0과 구별하는 미지원/읽기부재 상태."""


def integer(x):
    if isinstance(x,bool) or not isinstance(x,(str,int)):raise ValueError('정수 형식 오류')
    return int(x,0) if isinstance(x,str) and x.lower().startswith(('0x','-0x')) else int(x)


def u64(x):
    x=integer(x)
    if not 0<=x<=MASK:raise ValueError('u64 범위 오류')
    return x


def wrap(x):return x&MASK
def squared(x,y,ax,ay):return wrap(wrap(x-ax)**2+wrap(y-ay)**2)
def signed(x):return x-(1<<64) if x>>63 else x
def div5_tz(x):return x//5 if x>=0 else -((-x)//5)


class Snapshot:
    def __init__(self,reads):
        if not isinstance(reads,list) or len(reads)>96:raise ValueError('read 배열/96필드 예산 오류')
        self.fields={}
        self.recorded=[]
        self.used=[]
        for row in reads:
            address,width,value=u64(row['address']),integer(row['width']),u64(row['value'])
            if type(row['width']) is not int or width not in (1,2,4,8) or value>=1<<(width*8):raise ValueError('read 폭/값 오류')
            key=(address,width)
            if key in self.fields:raise ValueError('동일 address/width 중복 캡처')
            self.fields[key]=value
            self.recorded.append(key)

    def read(self,base,offset=0,width=8):
        key=(wrap(base+offset),width)
        if key not in self.fields:raise ModelDomainError('MISSING_RECORDED_FIELD:'+hex(key[0])+':'+str(width))
        if key not in self.used:self.used.append(key)
        return self.fields[key]

    def exact_consumption(self):
        if self.used!=self.recorded:raise ValueError('Rust Capture의 첫 read 순서/소비집합과 독립 재계산 불일치')



_WAVE04=None
def load_wave04():
    global _WAVE04
    path=Path(__file__).with_name('models_wave04.py')
    if hashlib.sha256(path.read_bytes()).hexdigest()!='3ca5666fae562c7dfb527caba023033ae91608e06a8c16ad2f5b8695717283c1':
        raise ValueError('wave04독립식SHA변경: 재검토필요')
    if _WAVE04 is None:
        spec=importlib.util.spec_from_file_location('wave04_readonly_model_dependency',path)
        _WAVE04=importlib.util.module_from_spec(spec);spec.loader.exec_module(_WAVE04)
    return _WAVE04


_WAVE05_CACHE={}
WAVE05_PINS={'models_wave05_direct.py': 'e34f8101f550cb77d51ccf9f89ceb2de609c7243091b9a672a4a282db9e34de5', 'models_wave05_outputs.py': '1bdf343d91024ea88de6ba297be41b25f258848c83ecbe1748eed7d48752c1c4', 'models_wave04.py': '3ca5666fae562c7dfb527caba023033ae91608e06a8c16ad2f5b8695717283c1'}
def load_wave05(pid):
    name='models_wave05_direct.py' if pid in (27,28,32,33,34,35,36) else 'models_wave05_outputs.py'
    path=Path(__file__).with_name(name)
    if hashlib.sha256(path.read_bytes()).hexdigest()!=WAVE05_PINS[name]:raise ValueError('wave05독립식SHA변경')
    if name not in _WAVE05_CACHE:
        spec=importlib.util.spec_from_file_location(name.removesuffix('.py')+'_independent',path)
        module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module);_WAVE05_CACHE[name]=module
    return _WAVE05_CACHE[name]

def no_duplicate_keys(pairs):
    result={}
    for key,value in pairs:
        if key in result:raise ValueError('중복JSON키:'+key)
        result[key]=value
    return result

def check_outputs(recorded,wanted):
    if not isinstance(recorded,list) or not isinstance(wanted,list) or len(recorded)!=len(wanted):raise ValueError('출력필드수불일치')
    if len(wanted)>96:raise ValueError('출력필드관측예산초과')
    used=set();same=True
    for actual,expected in zip(recorded,wanted):
        address=u64(expected['address']);width=integer(expected['width']);value=u64(expected['expected'])
        if type(expected['width']) is not int or width not in (1,2,4,8) or value>=1<<(8*width):raise ValueError('예상출력폭/값오류')
        if address+width>1<<64:raise ValueError('예상출력주소overflow')
        span=set(range(address,address+width))
        if used&span:raise ValueError('예상출력중복/겹침')
        used|=span
        if type(actual['width']) is not int or integer(actual['width'])!=width or u64(actual['address'])!=address:raise ValueError('독립출력주소/폭불일치')
        if u64(actual['expected'])!=value:raise ValueError('독립출력예상값불일치')
        got=u64(actual['actual'])
        if got>=1<<(8*width):raise ValueError('실제출력폭범위초과')
        same &= got==value
    return same



WAVE06_PINS={'models_wave06_direct.py': 'd2ef12cbc5e0ea644c6380fd8731e3c01b92de82ef4691ab3eed801789997605', 'models_wave06_iterator.py': 'e918353874f76614d1924ab1b22bc4917a8cbb1690a0ddb325fba0c41fc693e7'}
WAVE06_ROUTE={37:('models_wave06_direct.py',37),38:('models_wave06_iterator.py',39)}
_WAVE06_CACHE={}

def load_wave06(runtime_id):
    # scratch38/1016B00은 여기로 연결하지 않는다. runtime38은 scratch39다.
    if runtime_id not in WAVE06_ROUTE:raise ModelDomainError('WAVE06_RUNTIME_ID_NOT_ROUTED')
    name,scratch_id=WAVE06_ROUTE[runtime_id]
    path=Path(__file__).with_name(name)
    if hashlib.sha256(path.read_bytes()).hexdigest()!=WAVE06_PINS[name]:raise ValueError('wave06독립식SHA변경')
    if name not in _WAVE06_CACHE:
        spec=importlib.util.spec_from_file_location(name.removesuffix('.py')+'_independent',path)
        module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module);_WAVE06_CACHE[name]=module
    return _WAVE06_CACHE[name],scratch_id


WAVE07_PINS={'models_d8a1f0.py': 'a962172d64cb93124f649e49aff20f89f3e5dfd2a948f8da14f88ad53df6eba4', 'models_next.py': '8b76b7f57eb355b769059fe05ffbd16170cf9fdc16663d438c1c9c9435796652'}
_WAVE07_CACHE={}
def load_wave07(pid):
    if pid not in (39,40):raise ModelDomainError('WAVE07_RUNTIME_ID_NOT_ROUTED')
    name='models_d8a1f0.py'if pid==39 else'models_next.py'
    path=Path(__file__).with_name(name)
    if hashlib.sha256(path.read_bytes()).hexdigest()!=WAVE07_PINS[name]:raise ValueError('wave07독립식SHA변경')
    if name not in _WAVE07_CACHE:
        spec=importlib.util.spec_from_file_location(name.removesuffix('.py')+'_independent',path)
        module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module);_WAVE07_CACHE[name]=module
    return _WAVE07_CACHE[name]



WAVE08_PINS={'models_candidates.py': '1b7d7a6a224a0f09b3a31d91221cf90490fe4645d085ac9a1582280801c8dc0c', 'models_wave08_direct.py': 'bdc88e67183b8720c487a7950d1870f19cd7f19a7ed5b0f68878968834917c03'}
WAVE08_ROUTE={41: ('models_candidates.py', 14206592), 42: ('models_wave08_direct.py', 42), 43: ('models_wave08_direct.py', 43)}
_WAVE08_CACHE={}
def load_wave08(pid):
    if pid not in WAVE08_ROUTE:raise ModelDomainError('WAVE08_RUNTIME_ID_NOT_ROUTED')
    name,scratch_id=WAVE08_ROUTE[pid]
    path=Path(__file__).with_name(name)
    if hashlib.sha256(path.read_bytes()).hexdigest()!=WAVE08_PINS[name]:raise ValueError('wave08독립식SHA변경')
    if name not in _WAVE08_CACHE:
        spec=importlib.util.spec_from_file_location(name.removesuffix('.py')+'_wave08_independent',path)
        m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m);_WAVE08_CACHE[name]=m
    return _WAVE08_CACHE[name],scratch_id


def known_metric_names(pid):
    if pid in WAVE08_ROUTE:
        helper,scratch_id=load_wave08(pid)
        return helper.METRIC_NAMES[scratch_id]
    if pid==39:return load_wave07(pid).METRIC_NAMES
    if pid==40:return load_wave07(pid).METRIC_NAMES[1000]
    if pid>=37:
        helper,scratch_id=load_wave06(pid)
        return helper.METRIC_NAMES[scratch_id]
    return load_wave05(pid).METRIC_NAMES[pid] if pid>=27 else METRIC_NAMES[pid]


def evaluate(pid,args,snapshot):
    """expected/mask/metrics를 입력에서 계산. 게임 출력은 인자로 받지 않는다."""
    if not 0<=pid<44:raise ModelDomainError('UNSUPPORTED_MODEL_ID')
    if not isinstance(args,list) or len(args)!=4:raise ValueError('인수4개 필요')
    a=[u64(x) for x in args]
    if pid in WAVE08_ROUTE:
        helper,scratch_id=load_wave08(pid)
        try:return helper.evaluate(scratch_id,a,snapshot)
        except helper.DomainError as error:raise ModelDomainError(str(error)) from error
    if pid>=39:
        helper=load_wave07(pid)
        try:return helper.evaluate(a,snapshot)if pid==39 else helper.evaluate(1000,a,snapshot)
        except helper.DomainError as error:raise ModelDomainError(str(error)) from error
    if pid>=37:
        helper,scratch_id=load_wave06(pid)
        try:return helper.evaluate(scratch_id,a,snapshot)
        except helper.DomainError as error:raise ModelDomainError(str(error)) from error
    if pid>=27:
        helper=load_wave05(pid)
        try:return helper.evaluate(pid,a,snapshot)
        except helper.DomainError as error:raise ModelDomainError(str(error)) from error
    if pid>=19:
        helper=load_wave04()
        try:return helper.evaluate(pid,a,snapshot)
        except helper.DomainError as error:raise ModelDomainError(str(error)) from error
    r=snapshot.read
    metrics=[]
    def met(name,lhs,rhs,relation,predicate):
        metrics.append(dict(name=name,lhs=lhs,rhs=rhs,relation=relation,predicate=bool(predicate),observed_branch=False))
    def out(value,mask=0xff):return dict(expected=wrap(value),mask=mask,metrics=metrics,output_fields=[])

    if pid in (0,15):
        table,time=a[:2]
        if time==0:return out(r(table,0x98),MASK)
        count=r(table,0xa0)
        if count>18:raise ModelDomainError('FORECAST_SAMPLE_CAPTURE_CAPACITY')
        index=time//5
        if pid==15:met('forecast_tail_index_ge_count',index,count,'>=',index>=count)
        if index>=count:return out(r(table,0x98 if count==0 else (count-1)*8),MASK)
        if pid==15:met('forecast_first_interval_lt5',time,5,'<',time<5)
        if time<5:
            after,before,remainder=r(table),r(table,0x98),time
        else:
            if index==0 or index-1>17 or time>=90:raise ModelDomainError('FORECAST_INTERPOLATION_DOMAIN')
            before,after,remainder=r(table,(index-1)*8),r(table,index*8),time-index*5
        value=wrap(before+div5_tz(signed(wrap((after-before)*remainder))))
        if pid==0:met('forecast_remainder_is_zero',remainder,0,'==',remainder==0)
        return out(value,MASK)
    if pid==1:
        if a[1]>1:raise ModelDomainError('TEAM_BOUNDS')
        world=r(a[0]);base=wrap(world+0x238+a[1]*40)
        for off in (0,16,24):
            if r(base,off):return out(1)
        return out(0)
    if pid==2:
        if a[0]>1:raise ModelDomainError('TEAM_BOUNDS')
        base=wrap(a[2]+0x238+a[0]*40)
        for role in range(5):
            if r(base,role*8):return out(int(role==(a[1]&0xffffffff)))
        return out(0)
    if pid==3:
        own=a[0];opp=wrap(1-own)
        if opp>1:raise ModelDomainError('TEAM_BOUNDS')
        nexus=r(a[1],0x1c8+opp*8)
        if not nexus:return out(0)
        nx,ny=r(nexus,0x658),r(nexus,0x660)
        for role in range(5):
            entity=r(a[1],0x238+own*40+role*8)
            if not entity:continue
            d2=squared(r(entity,0x658),r(entity,0x660),nx,ny)
            inside=d2<=360000000000
            met('all_near_nexus_radius_squared',d2,360000000000,'<=',inside)
            if not inside:return out(0)
        return out(1)
    if pid==4:
        capture,role=r(a[0]),r(a[1]);own_ref=r(capture);own_role=r(own_ref)
        if role==own_role:return out(0)
        team_ref=r(capture,0x18);team=r(team_ref)
        if team>1 or role>4:raise ModelDomainError('TEAM_OR_ROLE_BOUNDS')
        bb=r(capture,0x10);row=wrap(bb+team*0xac8+role*32)
        if r(row,0x7f0,1):return out(0)
        extra=r(capture,0x20);lane=r(extra,0x11c,1)
        if r(row,0x7f1,1)!=lane:return out(0)
        world=r(capture,8);entity=r(world,0x238+team*40+role*8)
        if not entity:return out(0)
        anchor=r(capture,0x28)
        x,y,ax,ay=r(entity,0x658),r(entity,0x660),r(anchor,0x658),r(anchor,0x660)
        q=squared(x,y,ax,ay)>>8;inside=q<87890625
        met('same_lane_radius_shift8',q,87890625,'<',inside)
        return out(int(inside))
    if pid==5:
        outer=r(a[0]);inner=r(outer);player=r(inner,8);opp=wrap(1-r(player,0x800))
        if opp>1:raise ModelDomainError('TEAM_BOUNDS')
        candidate=r(a[1]);world=r(inner);candidate_id=r(candidate,0x5b8);matched=None
        for role in range(5):
            entity=r(world,0x238+opp*40+role*8)
            if entity and r(entity,0x5b8)==candidate_id:matched=role;break
        if matched is None:return out(1)
        first_ref=r(inner,0x10);first=r(first_ref,matched*8)
        if first:
            now_ref=r(inner,0x18);now=r(now_ref);limit_ref=r(inner,0x20);limit=r(limit_ref)
            age=max(now-first,0);recent=age<=limit
            met('first_stamp_recent',age,limit,'<=',recent)
            if recent:return out(1)
        second_ref=r(inner,0x28);second=r(second_ref,matched*8)
        if not second:return out(1)
        now_ref,limit_ref=r(inner,0x18),r(inner,0x20)
        now=r(now_ref);age=max(now-second,0);limit=r(limit_ref);expired=age>limit
        met('second_stamp_expired',age,limit,'>',expired)
        return out(int(expired))
    if pid==6:
        player,data=a[1:3];team=r(player,0x800)
        if team>1:raise ModelDomainError('TEAM_BOUNDS')
        bb=r(data,0x10);bank=wrap(bb+team*0xac8);count=r(bank,0x2c0)
        if not count:return out(0)
        if count>32:raise ModelDomainError('RECORD_CAPTURE_CAPACITY')
        role=r(player,0x890,4)
        if role>4:raise ModelDomainError('ROLE_CAPTURE_DOMAIN')
        records=r(bank,0x2b8)
        for index in range(count):
            record=wrap(records+index*80)
            if r(record,0x46,1) or not r(record,0x40+role,1):continue
            world=r(data);entity=r(world,0x238+team*40+role*8)
            if not entity:return out(0)
            maximum=max(r(entity,0x620),1);hp=r(entity,0x668);percent=wrap(hp*100)//maximum
            accepted=percent>=25;met('record_hp_minimum_percent',percent,25,'>=',accepted)
            return out(int(accepted))
        return out(0)
    if pid==7:
        count=r(a[0],0x840)
        if count>12:raise ModelDomainError('FORECAST_RECORD_CAPACITY')
        for index in range(count):
            record=wrap(a[0]+index*176)
            if r(record,0x90)==a[1]:return out(record,MASK)
        return out(0,MASK)
    if pid in (8,9,10,11):
        if a[1]<a[0]:raise ModelDomainError('HP_SLICE_REVERSED')
        diff=a[1]-a[0]
        if diff&7:raise ModelDomainError('HP_SLICE_UNALIGNED_LENGTH')
        n=diff>>3
        if n>32:raise ModelDomainError('HP_SLICE_CAPTURE_CAPACITY')
        threshold=40 if pid in (8,11) else 50;count=0
        for index in range(n):
            entity=r(a[0],index*8)
            if not entity:continue
            maximum=r(entity,0x620)
            if not maximum:raise ModelDomainError('HP_DIVISION_ZERO_GAME_PANIC_DOMAIN')
            hp=r(entity,0x668);percent=wrap(hp*100)//maximum;included=percent>=threshold
            met('hp_percent_ge'+str(threshold),percent,threshold,'>=',included)
            count+=included
        return out(count,MASK)
    if pid==12:
        opp=wrap(1-a[0])
        if opp>1:raise ModelDomainError('EF2410_TEAM_BOUNDS')
        slots=wrap(a[1]+0x238+opp*40)
        for off in (0,16,24,32):
            entity=r(slots,off)
            if not entity:continue
            d2=squared(r(entity,0x658),r(entity,0x660),a[2],a[3]);inside=d2<22500000001
            met('opponent_except_role1_d2',d2,22500000001,'<',inside)
            if inside:return out(1)
        return out(0)
    if pid==13:
        candidate_ref=r(a[1],8);candidate=r(candidate_ref)
        if not candidate:return out(0)
        capture=r(a[0]);anchor=r(capture);candidate_id=r(candidate,0x5b8);anchor_id=r(anchor,0x5b8)
        same=candidate_id==anchor_id;met('same_anchor_id',candidate_id,anchor_id,'==',same)
        if same:return out(1)
        player=r(capture,0x10);opp=wrap(1-r(player,0x800))
        if opp>1:raise ModelDomainError('FA9600_TEAM_BOUNDS')
        role_ref=r(capture,0x18);role=r(role_ref)
        if role>=5:raise ModelDomainError('FA9600_ROLE_BOUNDS')
        cache=r(capture,8);target=r(cache,0x238+opp*40+role*8)
        if not target:raise ModelDomainError('FA9600_MISSING_ROLE')
        radius_ref=r(capture,0x20)
        cx,cy,tx,ty=r(candidate,0x658),r(candidate,0x660),r(target,0x658),r(target,0x660)
        radius=r(radius_ref);d2=squared(cx,cy,tx,ty);r2=wrap(radius*radius);inside=d2<=r2
        met('role_radius_squared_inclusive',d2,r2,'<=',inside)
        return out(int(inside))
    if pid==14:
        entity=r(a[1],8);maximum=r(entity,0x620)
        if not maximum:raise ModelDomainError('MIDDLE_HP_DIVISION_ZERO')
        capture=r(a[0]);tag_ref=r(capture);numerator=wrap(r(entity,0x668)*100);context=r(capture,8)
        percent=numerator//maximum;met('middle_minimum_hp40',percent,40,'>=',percent>=40)
        if percent<40:return out(0)
        tag=r(tag_ref,0,1)
        if tag not in (4,5):return out(0)
        x=r(entity,0x658)
        if tag==4:
            met('middle_tag4_x_le192000',x,192000,'<=',x<=192000)
            if x<=192000:return out(1)
        settings=r(context,8);height=r(settings,0x12c0);q=wrap(height-r(entity,0x660))
        if tag==4:
            gap=wrap(x-q);met('middle_tag4_x_le_q',x,q,'<=',x<=q)
            if x<=q:return out(1)
            cut=wrap(r(settings,0x12b8)-192000);met('middle_tag4_q_ge_width_minus192000',q,cut,'>=',q>=cut)
            if q>=cut:return out(1)
            met('middle_tag4_gap_lt64000',gap,64000,'<',gap<64000)
            return out(int(gap<64000))
        met('middle_tag5_q_le_x',q,x,'<=',q<=x);met('middle_tag5_q_le192000',q,192000,'<=',q<=192000)
        if q<=x or q<=192000:return out(1)
        cut=wrap(r(settings,0x12b8)-192000);met('middle_tag5_x_ge_width_minus192000',x,cut,'>=',x>=cut)
        if x>=cut:return out(1)
        gap=wrap(q-x);met('middle_tag5_gap_lt64000',gap,64000,'<',gap<64000)
        return out(int(gap<64000))
    if pid==16:
        active=a[0]==1;met('optional_scale_tag_exact1',a[0],1,'==',active)
        if not active:return out(a[2],MASK)
        divisor_ref=r(a[3],8);divisor=r(divisor_ref)
        if divisor==0:raise ModelDomainError('WAVE03_DIVISOR_ZERO_GAME_PANIC_DOMAIN')
        factor_ref=r(a[3]);factor=r(factor_ref);n=signed(wrap(a[1]*factor));d=signed(divisor)
        if n==-(1<<63) and d==-1:raise ModelDomainError('WAVE03_SIGNED_DIVISION_OVERFLOW_GAME_PANIC_DOMAIN')
        q=abs(n)//abs(d);q=-q if (n<0)!=(d<0) else q
        cap_ref=r(a[3],0x10);cap=signed(r(cap_ref));met('optional_scale_signed_cap',q,cap,'<=',q<=cap)
        return out(min(q,cap),MASK)
    if pid==17:
        divisor_ref=r(a[0],0x10);divisor=r(divisor_ref)
        if divisor==0:raise ModelDomainError('WAVE03_DIVISOR_ZERO_GAME_PANIC_DOMAIN')
        obj=r(a[0]);factor_ref=r(a[0],8);factor=r(factor_ref);n=signed(wrap(factor*r(obj,0x98)));d=signed(divisor)
        if n==-(1<<63) and d==-1:raise ModelDomainError('WAVE03_SIGNED_DIVISION_OVERFLOW_GAME_PANIC_DOMAIN')
        q=abs(n)//abs(d);q=-q if (n<0)!=(d<0) else q
        subtract_ref=r(a[0],0x18);v=signed(wrap(q-r(subtract_ref)));positive=v>0
        met('scaled_delta_positive_after_wrap',v,0,'>',positive)
        return out(v if positive else 0,MASK)
    if pid==18:
        if a[1]>1:raise ModelDomainError('FE8F30_TEAM_BOUNDS_GAME_PANIC_DOMAIN')
        line=a[2]&0xff
        if line>2:raise ModelDomainError('FE8F30_LINE_CAPTURE_DOMAIN_ONLY_0_TO_2')
        bb=r(a[0],0x10);record=wrap(bb+a[1]*0xac8+line*0x28);ready=r(record,0x26,1)
        met('lane_record26_nonzero',ready,0,'!=',ready!=0)
        if ready:return out(1)
        pressure=r(record,0x27,1);pressure=pressure-256 if pressure>=128 else pressure
        met('lane_record27_signed_positive',pressure,0,'>',pressure>0)
        if pressure>0:return out(0)
        blocked=r(record,0x25,1);met('lane_record25_nonzero',blocked,0,'!=',blocked!=0)
        if blocked:return out(0)
        world=r(a[0]);structure=r(world,[0x10,0x90,0xd0][line]+((1-a[1])<<5)+0x18)
        met('lane_opponent_structure_present',structure,0,'!=',structure!=0)
        return out(int(structure!=0))
    raise ModelDomainError('UNSUPPORTED_MODEL')


def check_metrics(logged,expected):
    if not isinstance(logged,list) or len(logged)!=len(expected):raise ValueError('metric수 불일치')
    for actual,wanted in zip(logged,expected):
        if type(actual['predicate']) is not bool or actual['observed_branch'] is not False:raise ValueError('metric bool/실분기관측 플래그 오류')
        normalized=dict(name=actual['name'],lhs=integer(actual['lhs']),rhs=integer(actual['rhs']),relation=actual['relation'],
            predicate=actual['predicate'],observed_branch=actual['observed_branch'])
        if normalized!=wanted:raise ValueError('독립 metric 재계산 차이:'+wanted['name'])


def load_base():
    path=Path(__file__).with_name('base_analyze_logs.py')
    if hashlib.sha256(path.read_bytes()).hexdigest()!='ce6a147ef432bfc619fe586d1698b217caf24e78e29a60bb998b433cd6b56b87':
        raise ValueError('기존 분석기 의존SHA 변경: 재검토 필요')
    spec=importlib.util.spec_from_file_location('analysis_round01_readonly_dependency',path)
    module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module)
    return module,path



def selection_scope(summary):
    result=dict(prepared_count=len(summary['probes']),selected_ids=[],installed_ids=[],unselected_ids=[],refused_ids=[],pending_or_unknown_ids=[],rows=[])
    for p in summary['probes']:
        pid=integer(p['id']);status=p.get('install_status','UNKNOWN')
        if status=='NOT_SELECTED':
            category='NOT_SELECTED';result['unselected_ids'].append(pid)
            if any(integer(p.get(k,0))!=0 for k in ('calls','sample_attempts','compared','matches','mismatches','skipped','unstable','foreign_matches','foreign_mismatches','dropped')):
                raise ValueError('미선택probe의호출/표본카운터비0')
        elif status=='INSTALLED_UNVALIDATED_RUNTIME':
            category='INSTALLED_RUNTIME_UNVALIDATED';result['selected_ids'].append(pid);result['installed_ids'].append(pid)
        elif status.startswith(('REFUSED_BEFORE_WRITE:','INSTALL_ERROR_ORIGINAL_RETAINED_ENTRY_UNVERIFIED:')):
            category='SELECTED_BUT_REFUSED_OR_ENTRY_UNVERIFIED';result['selected_ids'].append(pid);result['refused_ids'].append(pid)
        else:
            category='PENDING_DISARMED_OR_FIXTURE_UNKNOWN';result['pending_or_unknown_ids'].append(pid)
        result['rows'].append(dict(id=pid,rva=hex(u64(p['rva'])),install_status=status,classification=category,foreign_chain=p.get('foreign_chain',False)))
    result['selected_count']=len(result['selected_ids']);result['installed_count']=len(result['installed_ids'])
    result['scope_complete_from_status']=not result['pending_or_unknown_ids']
    result['limits']='선택/설치는비교완료가아님. NOT_SELECTED와안전거절/entry미확정을분리하며설정파일의독립신원증명은별도.'
    return result

def load_journal_audit():
    path=Path(__file__).with_name('audit_journal.py')
    if hashlib.sha256(path.read_bytes()).hexdigest()!='02ce1b0efd0a7efffa7053409de2dc58604893cfa8537bac27691c35c0d44de5':raise ValueError('journal감사도구SHA변경')
    spec=importlib.util.spec_from_file_location('wave08_fixed_journal_audit',path)
    m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
    return m


def analyze(summary_path,observations_path,session_path=None,catalog_path=None,input_origin='unknown',expected_run_id=None,journal_path=None,expected_stop_record=None):
    if input_origin not in ('unknown','runtime','native_fixture','python_fixture'):raise ValueError('input_origin 오류')
    summary=json.loads(Path(summary_path).read_text(encoding='utf-8-sig'),object_pairs_hook=no_duplicate_keys)
    journal_review=load_journal_audit().audit(summary_path,journal_path,expected_run_id,expected_stop_record) if journal_path else None
    if type(summary.get('schema_version')) is not int or summary['schema_version']!=1:raise ValueError('summary schema 필요')
    if expected_run_id is not None and summary.get('run_id')!=expected_run_id:raise ValueError('expected run_id 불일치')
    native_selftest=(input_origin=='native_fixture' and summary.get('actual_sha256')=='ENGINE_SELFTEST_NOT_A_GAME_SESSION')
    if summary.get('baseline_sha256')!=BASELINE or (summary.get('actual_sha256')!=BASELINE and not native_selftest):raise ValueError('baseline/actual EXE SHA 불일치')
    if type(summary.get('identity_verified')) is not bool or type(summary.get('armed')) is not bool:raise ValueError('identity/armed bool 누락')
    if summary.get('prepared_probes')!=44 or len(summary.get('probes',[]))!=44:raise ValueError('44 prepared summary 필요')
    base,base_path=load_base()
    original=base.analyze(summary_path,observations_path,session_path,catalog_path)
    probes={}
    for p in summary['probes']:
        pid=integer(p['id'])
        if not 0<=pid<44 or u64(p['rva'])!=RVAS[pid]:raise ValueError('44모델의 id/RVA 불일치')
        if any(k not in p for k in base.COUNTERS):raise ValueError('필수 summary counter 누락')
        if pid in probes:raise ValueError('summary probe id 중복')
        probes[pid]=dict(id=pid,rva=hex(RVAS[pid]),independent_counts=collections.Counter(),metric_coverage={},
            raw_validated_samples=[],known_metrics_not_observed=[],direct_internal_branch_observations=0)
    if set(probes)!=set(range(44)):raise ValueError('44 prepared id 전량 필요')
    probe_selection_scope=selection_scope(summary)
    errors=[]
    seen=set()
    raw=Path(observations_path).read_bytes()
    if raw and not raw.endswith(b'\n'):raise ValueError('관측JSONL불완전마지막행')
    for line_no,line in enumerate(Path(observations_path).read_text(encoding='utf-8-sig').splitlines(),1):
        if not line.strip():continue
        try:
            x=json.loads(line,object_pairs_hook=no_duplicate_keys)
            if x.get('run_id')!=summary['run_id']:raise ValueError('run_id 불일치')
            if x.get('kind')!='sample':continue
            if type(x.get('schema_version')) is not int or x['schema_version']!=1:raise ValueError('event schema 불일치')
            pid=integer(x['probe_id']);p=probes[pid]
            if u64(x['rva'])!=RVAS[pid]:raise ValueError('event id/RVA 불일치')
            thread,seq=integer(x['thread_id']),u64(x['seq'])
            if not 0<=thread<1<<32:raise ValueError('thread_id 범위 오류')
            event_key=seq
            if event_key in seen:raise ValueError('중복 event')
            seen.add(event_key)
            kind,domain=x['comparison'],x['domain']
            if kind not in base.KINDS or domain not in ('vanilla','foreign_chain'):raise ValueError('comparison/domain 오류')
            if kind not in ('SKIP','UNSTABLE') and kind.startswith('FOREIGN_')!=(domain=='foreign_chain'):raise ValueError('foreign/vanilla 혼합')
            snap=Snapshot(x['reads'])
            if kind in ('SKIP','UNSTABLE'):
                if x.get('output_fields')!=[]:raise ValueError('미비교표본출력관측범위불일치')
                try:
                    ev=evaluate(pid,x['args'],snap);snap.exact_consumption()
                    if x.get('expected') is not None:
                        if u64(x['expected'])!=ev['expected'] or u64(x['mask'])!=ev['mask']:raise ValueError('불안정표본의모델예상값차이')
                        check_metrics(x.get('metrics',[]),ev['metrics'])
                    outcome='UNSTABLE_RECOMPUTED_NOT_COMPARED' if kind=='UNSTABLE' else 'SKIP_RECOMPUTABLE_NOT_COMPARED'
                except ModelDomainError:outcome=kind+'_NOT_RECOMPUTABLE_OR_DOMAIN_REJECTED'
                p['independent_counts'][outcome]+=1
                continue
            if not summary['identity_verified'] or not summary['armed'] or summary.get('init_ready') is False:raise ValueError('identity/armed/init_ready없는비교표본')
            ev=evaluate(pid,x['args'],snap);snap.exact_consumption()
            if u64(x['expected'])!=ev['expected'] or u64(x['mask'])!=ev['mask']:raise ValueError('독립 expected/mask 재계산 차이')
            check_metrics(x.get('metrics',[]),ev['metrics'])
            actual=u64(x['actual']);same=(actual&ev['mask'])==(ev['expected']&ev['mask'])
            same &= check_outputs(x.get('output_fields'),ev['output_fields'])
            if same!=(kind in ('MATCH','FOREIGN_MATCH')):raise ValueError('독립판정과 comparison 불일치')
            outcome=('VANILLA' if domain=='vanilla' else 'FOREIGN')+('_MATCH' if same else '_MISMATCH')
            p['independent_counts'][outcome]+=1
            p['raw_validated_samples'].append(dict(line=line_no,seq=seq,thread_id=thread,outcome=outcome,
                independent_expected=hex(ev['expected']),actual=hex(actual),mask=hex(ev['mask']),snapshot_fields=len(snap.used)))
            for metric in ev['metrics']:
                name=metric['name'];position='below' if metric['lhs']<metric['rhs'] else 'above' if metric['lhs']>metric['rhs'] else 'equal'
                coverage=p['metric_coverage'].setdefault(name,{d:dict(below=0,equal=0,above=0)for d in ('vanilla','foreign_chain')})
                coverage[domain][position]+=1
        except (ValueError,KeyError,TypeError,OverflowError,AssertionError) as error:
            if 'run_id' in str(error):raise ValueError(f'{line_no}행: {error}') from error
            errors.append(dict(line=line_no,error=str(error)))
    # 최종 summary보다 로그가 많은 경우도 성공으로 덮지 않는다. 진행 중 snapshot이나
    # 자기시험의 수동 counter reset일 수 있으므로 실제 손실량으로 단정하지 않는다.
    counter_excesses=[]
    for p in original['probes']:
        emitted=p['emitted_counts'];c=p['summary_counts']
        mapping={'compared':emitted['MATCH']+emitted['MISMATCH'],'matches':emitted['MATCH'],
            'mismatches':emitted['MISMATCH'],'skipped':emitted['SKIP'],'unstable':emitted['UNSTABLE'],
            'foreign_matches':emitted['FOREIGN_MATCH'],'foreign_mismatches':emitted['FOREIGN_MISMATCH'],
            'calls':sum(emitted.values()),'sample_attempts':sum(emitted.values())}
        excess={k:logged-c[k] for k,logged in mapping.items() if logged>c[k]}
        if excess:counter_excesses.append(dict(probe_id=p['id'],log_exceeds_summary=excess))
    total=collections.Counter()
    for p in probes.values():
        total.update(p['independent_counts'])
        p['independent_counts']=dict(p['independent_counts'])
        p['known_metrics_not_observed']=[name for name in known_metric_names(p['id']) if name not in p['metric_coverage']]
        for coverage in p['metric_coverage'].values():
            for domain in ('vanilla','foreign_chain'):
                coverage[domain]['missing']=[n for n in ('below','equal','above') if not coverage[domain][n]]
    if errors:status='INDEPENDENT_RECOMPUTATION_OR_CAPTURE_ERRORS'
    elif total['VANILLA_MISMATCH']:status='VANILLA_MISMATCH_INDEPENDENTLY_CONFIRMED'
    elif original['summary_counts']['mismatches']:status='VANILLA_MISMATCH_REPORTED_MISSING_RAW_CONFIRMATION'
    elif not summary['identity_verified']:status='IDENTITY_NOT_VERIFIED'
    elif not summary['armed']:status='NOT_ARMED'
    elif summary.get('init_ready') is False:status='INITIALIZATION_NOT_READY'
    elif counter_excesses:status='COUNTER_LOG_INCONSISTENCY'
    elif original['evidence_incomplete']:status='INDEPENDENT_RECOMPUTED_WITH_INCOMPLETE_EVIDENCE'
    elif total['VANILLA_MATCH']==0:status='NO_INDEPENDENT_VANILLA_COMPARISON'
    else:status='SAVED_VANILLA_SAMPLES_INDEPENDENTLY_MATCH_LIMITED_SCOPE'
    if input_origin=='runtime' and journal_review is None and status in ('INDEPENDENT_RECOMPUTED_WITH_INCOMPLETE_EVIDENCE','SAVED_VANILLA_SAMPLES_INDEPENDENTLY_MATCH_LIMITED_SCOPE'):
        status='INDEPENDENT_RECOMPUTED_JOURNAL_FRESHNESS_UNVERIFIED'
    return dict(schema_version=1,status=status,run_id=summary['run_id'],expected_run_id=expected_run_id,input_origin=input_origin,
        journal_audit=journal_review,journal_freshness_verified=journal_review is not None,probe_selection_scope=probe_selection_scope,
        identity_scope='EXPLICIT_NATIVE_SELFTEST_SENTINEL_NO_GAME_IDENTITY' if native_selftest else 'SUMMARY_EXE_SHA_PIN_MATCH',
        original_analysis=original,independent_counts=dict(total),independent_errors=errors,counter_log_excesses=counter_excesses,
        probes=list(probes.values()),supported_model_ids=list(range(44)),
        actual_runtime_comparisons_rechecked=(total['VANILLA_MATCH']+total['VANILLA_MISMATCH'])if input_origin=='runtime'else 0,
        synthetic_fixture_comparisons=(total['VANILLA_MATCH']+total['VANILLA_MISMATCH'])if input_origin in ('native_fixture','python_fixture')else 0,
        whole_runtime_verified=False,new_game_calls=0,
        loaded_dll_identity_independently_verified=False,
        source_files={str(Path(p).resolve()):hashlib.sha256(Path(p).read_bytes()).hexdigest() for p in
            [summary_path,observations_path,base_path,Path(__file__),Path(__file__).with_name('audit_journal.py')]+([journal_path]if journal_path else[])+[Path(__file__).with_name(name) for name in dict(WAVE05_PINS,**WAVE06_PINS,**WAVE07_PINS,**WAVE08_PINS)]},
        limits=['로그expected를정답으로쓰지않고저장read에서재계산',
            'Rust모델/게임함수를호출하지않는다. 원문·기존계약을복원한별도Python식이다.',
            'snapshot predicate와실제내부분기직접관측은다르다. 이44모델의observed_branch는전부false다.',
            'SKIP/UNSTABLE/FOREIGN은vanilla일치분모밖이며미저장표본을재계산하지않는다.',
            'summary/JSONL건수합산금지. stride·예산·큐/IO유실·진행중snapshot은서로구분한다.',
            'native/python fixture는실제게임관측이아니다. input_origin=runtime도외부실행맥락/DLL신원입증은별도다.',
            'summary의EXE SHA선언대조가실제로로드된DLL/sourceSHA를독립입증하지는않는다. 배포/세션영수증은별도연결해야한다.',
            '전후read재확인이통과해도ABA·객체수명·coherent동시snapshot·설치quiescence/안전성증명이아니다.',
            '모든저장표본일치도전체함수/전체입력/전체계수·문턱/741함수런타임완료가아니다.'])


def markdown(result):
    lines=['# round02 독립 입력 재계산 분석','',f'실행 `{result["run_id"]}` / origin `{result["input_origin"]}` / **{result["status"]}**','',
        f'독립 집계 `{result["independent_counts"]}`. 오류 {len(result["independent_errors"])}건. 원집계기 상태 `{result["original_analysis"]["status"]}`.','',
        '|id|RVA|독립집계|미관측metric|','|---:|---|---|---|']
    for p in result['probes']:lines.append(f'|{p["id"]}|{p["rva"]}|{p["independent_counts"]}|{p["known_metrics_not_observed"]}|')
    for p in result['probes']:
        if not p['metric_coverage']:continue
        lines += ['',f'## {p["rva"]} — snapshot predicate','']
        for name,coverage in p['metric_coverage'].items():lines.append(f'- `{name}`: vanilla `{coverage["vanilla"]}`, foreign `{coverage["foreign_chain"]}`')
    lines += ['','## 한계','']+['- '+x for x in result['limits']]+['']
    return '\n'.join(lines)


def main():
    parser=argparse.ArgumentParser()
    parser.add_argument('--summary',required=True,type=Path)
    parser.add_argument('--observations',required=True,type=Path)
    parser.add_argument('--expected-run-id')
    parser.add_argument('--journal',type=Path)
    parser.add_argument('--expected-stop-record',type=int)
    parser.add_argument('--session',type=Path)
    parser.add_argument('--catalog',type=Path)
    parser.add_argument('--input-origin',choices=['unknown','runtime','native_fixture','python_fixture'],default='unknown')
    parser.add_argument('--output-dir',required=True,type=Path)
    args=parser.parse_args()
    result=analyze(args.summary,args.observations,args.session,args.catalog,args.input_origin,args.expected_run_id,args.journal,args.expected_stop_record)
    args.output_dir.mkdir(parents=True,exist_ok=True)
    (args.output_dir/'analysis.json').write_text(json.dumps(result,ensure_ascii=False,indent=2)+'\n',encoding='utf8')
    (args.output_dir/'analysis.md').write_text(markdown(result),encoding='utf8')
    print(result['status'])


if __name__=='__main__':main()
