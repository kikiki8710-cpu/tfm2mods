"""wave03의 19모델을 저장 read 값만으로 독립 재계산하는 오프라인 분석기.

게임/모드/설정/원본함수를 호출하지 않는다. Rust 모델을 실행하거나 로그 expected를 정답으로 쓰지 않는다.
"""
from __future__ import annotations
import argparse, collections, hashlib, importlib.util, json
from pathlib import Path

MASK=(1<<64)-1
BASELINE='a1d05045405a12e2bed182daeafcad8c8a40cf29ab68ba6ea5bfd30257439582'
RVAS=[0xdd87a0,0xe52ee0,0xd98bd0,0xe49c00,0xfa9dc0,0xf51e50,0xe9eb70,0xfdb9a0,
      0xdc0bb0,0xdc0f80,0xf27690,0xf27860,0xef2410,0xfa9600,0x1019890,0xfe2700,0xfd19f0,0xfd37f0,0xfe8f30]
METRIC_NAMES={0:['forecast_remainder_is_zero'],1:[],2:[],3:['all_near_nexus_radius_squared'],
 4:['same_lane_radius_shift8'],5:['first_stamp_recent','second_stamp_expired'],6:['record_hp_minimum_percent'],7:[],
 8:['hp_percent_ge40'],9:['hp_percent_ge50'],10:['hp_percent_ge50'],11:['hp_percent_ge40'],
 12:['opponent_except_role1_d2'],13:['same_anchor_id','role_radius_squared_inclusive'],
 14:['middle_minimum_hp40','middle_tag4_x_le192000','middle_tag4_x_le_q','middle_tag4_q_ge_width_minus192000',
     'middle_tag4_gap_lt64000','middle_tag5_q_le_x','middle_tag5_q_le192000','middle_tag5_x_ge_width_minus192000','middle_tag5_gap_lt64000'],
 15:['forecast_tail_index_ge_count','forecast_first_interval_lt5'],16:['optional_scale_tag_exact1','optional_scale_signed_cap'],17:['scaled_delta_positive_after_wrap'],18:['lane_record26_nonzero','lane_record27_signed_positive','lane_record25_nonzero','lane_opponent_structure_present']}


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


def evaluate(pid,args,snapshot):
    """expected/mask/metrics를 입력에서 계산. 게임 출력은 인자로 받지 않는다."""
    if not 0<=pid<19:raise ModelDomainError('UNSUPPORTED_MODEL_ID')
    if not isinstance(args,list) or len(args)!=4:raise ValueError('인수4개 필요')
    a=[u64(x) for x in args]
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
    path=Path(__file__).with_name('analyze_logs.py')
    if hashlib.sha256(path.read_bytes()).hexdigest()!='ce6a147ef432bfc619fe586d1698b217caf24e78e29a60bb998b433cd6b56b87':
        raise ValueError('기존 분석기 의존SHA 변경: 재검토 필요')
    spec=importlib.util.spec_from_file_location('analysis_round01_readonly_dependency',path)
    module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module)
    return module,path


def analyze(summary_path,observations_path,session_path=None,catalog_path=None,input_origin='unknown'):
    if input_origin not in ('unknown','runtime','native_fixture','python_fixture'):raise ValueError('input_origin 오류')
    summary=json.loads(Path(summary_path).read_text(encoding='utf-8-sig'))
    native_selftest=(input_origin=='native_fixture' and summary.get('actual_sha256')=='ENGINE_SELFTEST_NOT_A_GAME_SESSION')
    if summary.get('baseline_sha256')!=BASELINE or (summary.get('actual_sha256')!=BASELINE and not native_selftest):raise ValueError('baseline/actual EXE SHA 불일치')
    if type(summary.get('identity_verified')) is not bool or type(summary.get('armed')) is not bool:raise ValueError('identity/armed bool 누락')
    base,base_path=load_base()
    original=base.analyze(summary_path,observations_path,session_path,catalog_path)
    probes={}
    for p in summary['probes']:
        pid=integer(p['id'])
        if not 0<=pid<19 or u64(p['rva'])!=RVAS[pid]:raise ValueError('19모델의 id/RVA 불일치')
        if any(k not in p for k in base.COUNTERS):raise ValueError('필수 summary counter 누락')
        probes[pid]=dict(id=pid,rva=hex(RVAS[pid]),independent_counts=collections.Counter(),metric_coverage={},
            raw_validated_samples=[],known_metrics_not_observed=[],direct_internal_branch_observations=0)
    errors=[]
    seen=set()
    for line_no,line in enumerate(Path(observations_path).read_text(encoding='utf-8-sig').splitlines(),1):
        if not line.strip():continue
        try:
            x=json.loads(line)
            if x.get('run_id')!=summary['run_id']:raise ValueError('run_id 불일치')
            if x.get('kind')!='sample':continue
            if x.get('schema_version')!=1:raise ValueError('event schema 불일치')
            pid=integer(x['probe_id']);p=probes[pid]
            if u64(x['rva'])!=RVAS[pid]:raise ValueError('event id/RVA 불일치')
            thread,seq=integer(x['thread_id']),u64(x['seq'])
            if not 0<=thread<1<<32:raise ValueError('thread_id 범위 오류')
            event_key=(pid,thread,seq)
            if event_key in seen:raise ValueError('중복 event')
            seen.add(event_key)
            kind,domain=x['comparison'],x['domain']
            if kind not in base.KINDS or domain not in ('vanilla','foreign_chain'):raise ValueError('comparison/domain 오류')
            if kind not in ('SKIP','UNSTABLE') and kind.startswith('FOREIGN_')!=(domain=='foreign_chain'):raise ValueError('foreign/vanilla 혼합')
            if x.get('output_fields')!=[]:raise ValueError('이19모델에는output_fields가없음')
            snap=Snapshot(x['reads'])
            if kind in ('SKIP','UNSTABLE'):
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
        p['known_metrics_not_observed']=[name for name in METRIC_NAMES[p['id']] if name not in p['metric_coverage']]
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
    return dict(schema_version=1,status=status,run_id=summary['run_id'],input_origin=input_origin,
        identity_scope='EXPLICIT_NATIVE_SELFTEST_SENTINEL_NO_GAME_IDENTITY' if native_selftest else 'SUMMARY_EXE_SHA_PIN_MATCH',
        original_analysis=original,independent_counts=dict(total),independent_errors=errors,counter_log_excesses=counter_excesses,
        probes=list(probes.values()),supported_model_ids=list(range(19)),
        actual_runtime_comparisons_rechecked=(total['VANILLA_MATCH']+total['VANILLA_MISMATCH'])if input_origin=='runtime'else 0,
        synthetic_fixture_comparisons=(total['VANILLA_MATCH']+total['VANILLA_MISMATCH'])if input_origin in ('native_fixture','python_fixture')else 0,
        whole_runtime_verified=False,new_game_calls=0,
        loaded_dll_identity_independently_verified=False,
        source_files={str(Path(p).resolve()):hashlib.sha256(Path(p).read_bytes()).hexdigest() for p in
            [summary_path,observations_path,base_path,Path(__file__)]},
        limits=['로그expected를정답으로쓰지않고저장read에서재계산',
            'Rust모델/게임함수를호출하지않는다. 원문·기존계약을복원한별도Python식이다.',
            'snapshot predicate와실제내부분기직접관측은다르다. 이19모델의observed_branch는전부false다.',
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
    parser.add_argument('--session',type=Path)
    parser.add_argument('--catalog',type=Path)
    parser.add_argument('--input-origin',choices=['unknown','runtime','native_fixture','python_fixture'],default='unknown')
    parser.add_argument('--output-dir',required=True,type=Path)
    args=parser.parse_args()
    result=analyze(args.summary,args.observations,args.session,args.catalog,args.input_origin)
    args.output_dir.mkdir(parents=True,exist_ok=True)
    (args.output_dir/'analysis.json').write_text(json.dumps(result,ensure_ascii=False,indent=2)+'\n',encoding='utf8')
    (args.output_dir/'analysis.md').write_text(markdown(result),encoding='utf8')
    print(result['status'])


if __name__=='__main__':main()
