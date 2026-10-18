"""Python 수식/손상검출 fixture 시험. native/Rust/게임 실행 결과로 합산하지 않는다."""
from pathlib import Path
import copy,importlib.util,json,tempfile,unittest

ROOT=Path(__file__).resolve().parents[1]
spec=importlib.util.spec_from_file_location('round02_analysis_test_subject',ROOT/'tools/analyze_logs_round02.py')
A=importlib.util.module_from_spec(spec);spec.loader.exec_module(A)


class Fixture:
    def __init__(self):self.fields={};self.used=[]
    def put(self,address,width,value):self.fields[(address,width)]=value
    def q(self,address,value):self.put(address,8,value)
    def xy(self,entity,x,y):self.q(entity+0x658,x);self.q(entity+0x660,y)
    def read(self,base,offset=0,width=8):
        key=((base+offset)&A.MASK,width)
        if key not in self.fields:raise A.ModelDomainError('MISSING_FIXTURE_FIELD')
        if key not in self.used:self.used.append(key)
        return self.fields[key]
    def reads(self):return [dict(address=hex(a),width=w,value=hex(self.fields[(a,w)]))for a,w in self.used]


def fixture(pid):
    f=Fixture();args=[0x1000,0x1100,0,0];want=0
    if pid in (0,15):
        args=[0x1000,1,0,0];f.q(0x10a0,1);f.q(0x1000,10);f.q(0x1098,0);want=2
    elif pid==1:
        args=[0x1000,0,0,0];f.q(0x1000,0x2000);f.q(0x2238,1);want=1
    elif pid==2:
        args=[0,0x100000001,0x2000,0];f.q(0x2238,0);f.q(0x2240,1);want=1
    elif pid==3:
        args=[0,0x2000,0,0];f.q(0x21d0,0x3000);f.xy(0x3000,0,0)
        for j in range(5):f.q(0x2238+j*8,0)
        f.q(0x2238,0x4000);f.xy(0x4000,600000,0);want=1
    elif pid==4:
        args=[0x1000,0x1100,0,0];f.q(0x1000,0x2000);f.q(0x1100,1);f.q(0x2000,0x3000);f.q(0x3000,0)
        f.q(0x2018,0x3100);f.q(0x3100,0);f.q(0x2010,0x4000);row=0x4020
        f.put(row+0x7f0,1,0);f.q(0x2020,0x5000);f.put(0x511c,1,2);f.put(row+0x7f1,1,2)
        f.q(0x2008,0x6000);f.q(0x6240,0x7000);f.q(0x2028,0x8000)
        f.xy(0x7000,150000,0);f.xy(0x8000,0,0);want=0
    elif pid==5:
        f.q(0x1000,0x2000);f.q(0x2000,0x3000);f.q(0x3008,0x4000);f.q(0x4800,0)
        f.q(0x1100,0x5000);f.q(0x3000,0x6000);f.q(0x55b8,1);f.q(0x6260,0x5000)
        f.q(0x3010,0x7000);f.q(0x7000,10);f.q(0x3018,0x8000);f.q(0x8000,20);f.q(0x3020,0x9000);f.q(0x9000,10);want=1
    elif pid==6:
        args=[0,0x2000,0x3000,0];f.q(0x2800,0);f.q(0x3010,0x4000);f.q(0x42c0,1);f.put(0x2890,4,0)
        f.q(0x42b8,0x5000);f.put(0x5046,1,0);f.put(0x5040,1,1);f.q(0x3000,0x6000);f.q(0x6238,0x7000)
        f.q(0x7620,100);f.q(0x7668,25);want=1
    elif pid==7:
        args=[0x1000,3,0,0];f.q(0x1840,1);f.q(0x1090,3);want=0x1000
    elif pid in (8,9,10,11):
        args=[0x1000,0x1008,0,0];f.q(0x1000,0x2000);f.q(0x2620,100)
        hp={8:40,9:49,10:50,11:39}[pid];f.q(0x2668,hp);want=int(pid in (8,10))
    elif pid==12:
        args=[0,0x2000,0,0];f.q(0x2260,0x3000);f.xy(0x3000,150000,0);want=1
    elif pid==13:
        args=[0x1000,0x2000,0,0];f.q(0x2008,0x2100);f.q(0x2100,0x4000);f.q(0x1000,0x3000)
        f.q(0x3000,0x5000);f.q(0x45b8,2);f.q(0x55b8,2);want=1
    elif pid==14:
        args=[0x1000,0x2000,0,0];f.q(0x2008,0x3000);f.q(0x3620,100);f.q(0x1000,0x4000);f.q(0x4000,0x5000)
        f.q(0x3668,40);f.q(0x4008,0x6000);f.put(0x5000,1,4);f.q(0x3658,192000);want=1
    return args,f,want


def event(pid,seq=0):
    args,f,want=fixture(pid);ev=A.evaluate(pid,args,f)
    assert ev['expected']==want,(pid,ev,want)
    metrics=[{**m,'lhs':str(m['lhs']),'rhs':str(m['rhs'])}for m in ev['metrics']]
    return dict(schema_version=1,run_id='python_fixture',kind='sample',probe_id=pid,rva=hex(A.RVAS[pid]),
        thread_id=1,seq=seq,comparison='MATCH',reason='',domain='vanilla',args=[hex(x)for x in args],
        expected=hex(want),actual=hex(want),mask=hex(ev['mask']),reads=f.reads(),metrics=metrics,output_fields=[])


def summary(events,ids=None):
    ids=sorted(set(x['probe_id']for x in events))if ids is None else ids
    probes=[]
    for pid in ids:
        counts={k:0 for k in ['calls','sample_attempts','compared','matches','mismatches','skipped','unstable','model_panics',
            'dropped','foreign_matches','foreign_mismatches','processing_errors','unlogged_errors']}
        for e in events:
            if e['probe_id']!=pid:continue
            counts['calls']+=1;counts['sample_attempts']+=1
            key={'MATCH':'matches','MISMATCH':'mismatches','SKIP':'skipped','UNSTABLE':'unstable','FOREIGN_MATCH':'foreign_matches','FOREIGN_MISMATCH':'foreign_mismatches'}[e['comparison']]
            counts[key]+=1
            if e['comparison']in('MATCH','MISMATCH'):counts['compared']+=1
        probes.append(dict(id=pid,rva=hex(A.RVAS[pid]),semantic_id='python_fixture',install_status='NATIVE_NOT_EXECUTED_FIXTURE',foreign_chain=False,**counts))
    return dict(schema_version=1,run_id='python_fixture',baseline_sha256=A.BASELINE,actual_sha256=A.BASELINE,identity_verified=True,
        armed=True,catalog_total=741,prepared_probes=16,io_errors=0,io_lost_or_uncertain_events=0,sample_stride=1,max_samples_per_probe=1000,probes=probes)


def analyze(events,s=None):
    with tempfile.TemporaryDirectory()as temp:
        p=Path(temp);s=summary(events)if s is None else s
        (p/'summary.json').write_text(json.dumps(s),encoding='utf8')
        (p/'events.jsonl').write_text(''.join(json.dumps(e)+'\n'for e in events),encoding='utf8')
        return A.analyze(p/'summary.json',p/'events.jsonl',input_origin='python_fixture')


class Round02ModelsTests(unittest.TestCase):
    def test_all16_hand_expected_values(self):
        for pid in range(16):
            args,f,want=fixture(pid);ev=A.evaluate(pid,args,f)
            self.assertEqual(ev['expected'],want,pid);self.assertTrue(all(m['observed_branch']is False for m in ev['metrics']))
            snap=A.Snapshot(f.reads());self.assertEqual(A.evaluate(pid,args,snap),ev);snap.exact_consumption()
    def test_all16_log_roundtrip_synthetic_only(self):
        r=analyze([event(i,i)for i in range(16)])
        self.assertEqual(r['independent_counts']['VANILLA_MATCH'],16);self.assertEqual(r['actual_runtime_comparisons_rechecked'],0)
        self.assertEqual(r['synthetic_fixture_comparisons'],16);self.assertFalse(r['independent_errors'])
    def test_al_mask_preserves_uncompared_high_bits(self):
        e=event(12);e['actual']=hex((0xabcdef12345678<<8)|1);r=analyze([e]);self.assertFalse(r['independent_errors'])
    def test_full_rax_difference_is_confirmed(self):
        e=event(7);e['actual']=hex(int(e['expected'],16)+(1<<32));e['comparison']='MISMATCH'
        r=analyze([e]);self.assertEqual(r['status'],'VANILLA_MISMATCH_INDEPENDENTLY_CONFIRMED')
    def test_forged_expected_and_actual_not_oracle(self):
        e=event(9);e['expected']=e['actual']='0x1';r=analyze([e]);self.assertEqual(len(r['independent_errors']),1)
    def test_false_zero_mask_rejected(self):
        e=event(12);e['mask']='0x0';r=analyze([e]);self.assertEqual(len(r['independent_errors']),1)
    def test_forged_metric_rejected(self):
        e=event(8);e['metrics'][0]['lhs']='41';r=analyze([e]);self.assertEqual(len(r['independent_errors']),1)
    def test_missing_read_not_default_zero(self):
        e=event(9);e['reads'].pop();r=analyze([e]);self.assertEqual(len(r['independent_errors']),1)
    def test_unstable_and_skip_not_success(self):
        e=event(8);e['comparison']='UNSTABLE';e['actual']='0x999';r=analyze([e]);self.assertEqual(r['synthetic_fixture_comparisons'],0)
        e=event(8);e['comparison']='SKIP';e['args']=['0x1000','0x1108','0x0','0x0'];e['reads']=[];e['metrics']=[];e['expected']=e['mask']=None
        r=analyze([e]);self.assertEqual(r['synthetic_fixture_comparisons'],0);self.assertFalse(r['independent_errors'])
    def test_loss_and_no_samples_no_full_pass(self):
        e=event(8);s=summary([e]);s['probes'][0]['compared']=s['probes'][0]['matches']=2;s['probes'][0]['dropped']=1
        r=analyze([e],s);self.assertEqual(r['status'],'INDEPENDENT_RECOMPUTED_WITH_INCOMPLETE_EVIDENCE')
        r=analyze([],summary([],ids=[7]));self.assertEqual(r['status'],'NO_INDEPENDENT_VANILLA_COMPARISON')
    def test_wrong_identity_and_runid_fail_closed(self):
        e=event(8);s=summary([e]);s['actual_sha256']='0'*64
        self.assertRaises(ValueError,analyze,[e],s)
        s=summary([e]);e['run_id']='other';self.assertRaises(ValueError,analyze,[e],s)
    def test_foreign_and_duplicate_not_vanilla(self):
        e=event(8);e['domain']='foreign_chain';e['comparison']='FOREIGN_MATCH';r=analyze([e]);self.assertEqual(r['synthetic_fixture_comparisons'],0)
        e=event(8);r=analyze([e,copy.deepcopy(e)]);self.assertEqual(len(r['independent_errors']),1)
    def test_wrap_division_and_low32(self):
        a,f,_=fixture(8);f.q(0x2620,A.MASK);f.q(0x2668,A.MASK);self.assertEqual(A.evaluate(8,a,f)['expected'],0)
        a,f,_=fixture(2);a[1]=(1<<63)|1;self.assertEqual(A.evaluate(2,a,f)['expected'],1)
        a,f,_=fixture(0);f.q(0x1098,10);f.q(0x1000,(-11)&A.MASK);self.assertEqual(A.evaluate(0,a,f)['expected'],6)


if __name__=='__main__':unittest.main()
