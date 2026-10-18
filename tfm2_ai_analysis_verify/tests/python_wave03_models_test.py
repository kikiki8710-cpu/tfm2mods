"""wave03別oracle경계. 합성시험은실제게임관측이아니다."""
from pathlib import Path
import importlib.util,json,tempfile,unittest
ROOT=Path(__file__).resolve().parents[1]
def module(name,path):
    spec=importlib.util.spec_from_file_location(name,path);m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m);return m
A=module('wave03_oracle',ROOT/'tools/analyze_logs_wave03.py')
OLD=module('wave03_prior_fixture_readonly',ROOT/'tests/python_round02_models_test.py')
Fixture=OLD.Fixture
def scale(divisor=1,cap=40,factor=1):
    f=Fixture();f.q(0x1000,0x2000);f.q(0x1008,0x3000);f.q(0x1010,0x4000)
    f.q(0x2000,factor);f.q(0x3000,divisor);f.q(0x4000,cap);return f
def delta(input=40,divisor=1,subtract=40,factor=1):
    f=Fixture();f.q(0x1000,0x2000);f.q(0x1008,0x3000);f.q(0x1010,0x4000);f.q(0x1018,0x5000)
    f.q(0x2098,input);f.q(0x3000,factor);f.q(0x4000,divisor);f.q(0x5000,subtract);return f
def lane(team=0,line=0,pressure=0,structure=1):
    f=Fixture();f.q(0x1010,0x2000);f.q(0x1000,0x5000);record=0x2000+team*0xac8+line*0x28
    f.put(record+0x26,1,0);f.put(record+0x27,1,pressure);f.put(record+0x25,1,0)
    f.q(0x5000+[0x10,0x90,0xd0][line]+((1-team)<<5)+0x18,structure);return f
def event(pid,args,f,want,seq=0):
    ev=A.evaluate(pid,args,f);assert ev['expected']==want
    return dict(schema_version=1,run_id='python_wave03_fixture',kind='sample',probe_id=pid,rva=hex(A.RVAS[pid]),thread_id=1,seq=seq,
        comparison='MATCH',reason='',domain='vanilla',args=[hex(x)for x in args],expected=hex(want),actual=hex(want),mask=hex(ev['mask']),
        reads=f.reads(),metrics=[{**m,'lhs':str(m['lhs']),'rhs':str(m['rhs'])}for m in ev['metrics']],output_fields=[])
def analyze(events):
    s=OLD.summary([]);s['run_id']='python_wave03_fixture';s['prepared_probes']=19
    for pid in sorted({e['probe_id']for e in events}):
        subset=[e for e in events if e['probe_id']==pid]
        c={k:0 for k in ['calls','sample_attempts','compared','matches','mismatches','skipped','unstable','model_panics','dropped','foreign_matches','foreign_mismatches','processing_errors','unlogged_errors']}
        for k in ['calls','sample_attempts','compared','matches']:c[k]=len(subset)
        s['probes'].append(dict(id=pid,rva=hex(A.RVAS[pid]),**c))
    with tempfile.TemporaryDirectory()as temp:
        p=Path(temp);(p/'s.json').write_text(json.dumps(s));(p/'e.jsonl').write_text(''.join(json.dumps(e)+'\n'for e in events))
        return A.analyze(p/'s.json',p/'e.jsonl',input_origin='python_fixture')

class Wave03Tests(unittest.TestCase):
    def test_all19_hand_fixture_roundtrip(self):
        events=[OLD.event(i,i)for i in range(16)]
        for e in events:e['run_id']='python_wave03_fixture'
        events+=[event(16,[1,41,0,0x1000],scale(),40),event(17,[0x1000,0,0,0],delta(input=41),1),event(18,[0x1000,0,0,0],lane(),1)]
        r=analyze(events);self.assertFalse(r['independent_errors']);self.assertEqual(r['synthetic_fixture_comparisons'],19);self.assertEqual(r['actual_runtime_comparisons_rechecked'],0)
    def test_scale_cap_boundary_and_tag_full64(self):
        events=[event(16,[1,x,0,0x1000],scale(),min(x,40),x)for x in [39,40,41]]
        r=analyze(events);c=r['probes'][0]['metric_coverage']['optional_scale_signed_cap']['vanilla']
        self.assertEqual([c[x]for x in ['below','equal','above']],[1,1,1])
        f=Fixture();self.assertEqual(A.evaluate(16,[0x100000001,0,A.MASK,0],f)['expected'],A.MASK);self.assertFalse(f.used)
    def test_scale_signed_wrap_and_panic_domains(self):
        self.assertEqual(A.evaluate(16,[1,(-8)&A.MASK,0,0x1000],scale(3))['expected'],(-2)&A.MASK)
        self.assertEqual(A.evaluate(16,[1,A.MASK,0,0x1000],scale(2,factor=2))['expected'],A.MASK)
        for divisor,input in [(0,1),(A.MASK,1<<63)]:self.assertRaises(A.ModelDomainError,A.evaluate,16,[1,input,0,0x1000],scale(divisor))
    def test_delta_signed_zero_boundary_and_wrap(self):
        events=[event(17,[0x1000,0,0,0],delta(input=x),max(0,x-40),x)for x in [39,40,41]]
        c=analyze(events)['probes'][0]['metric_coverage']['scaled_delta_positive_after_wrap']['vanilla']
        self.assertEqual([c[x]for x in ['below','equal','above']],[1,1,1])
        self.assertEqual(A.evaluate(17,[0x1000,0,0,0],delta(input=1<<63,subtract=1))['expected'],(1<<63)-1)
        self.assertEqual(A.evaluate(17,[0x1000,0,0,0],delta(input=(1<<63)-1,subtract=A.MASK))['expected'],0)
    def test_lane_signed_byte_boundary_and_early_shortcut(self):
        events=[event(18,[0x1000,0,0,0],lane(pressure=x),int(x!=1),i)for i,x in enumerate([255,0,1])]
        c=analyze(events)['probes'][0]['metric_coverage']['lane_record27_signed_positive']['vanilla']
        self.assertEqual([c[x]for x in ['below','equal','above']],[1,1,1])
        f=Fixture();f.q(0x1010,0x2000);f.put(0x2026,1,1);self.assertEqual(A.evaluate(18,[0x1000,0,0,0],f)['expected'],1);self.assertEqual(len(f.used),2)
    def test_lane_teams_static_table_and_lowbyte_mask(self):
        for team in range(2):
            for line in range(3):
                e=event(18,[0x1000,team,line|0x100,0],lane(team,line),1);e['actual']='0xabcdef0000000001'
                self.assertFalse(analyze([e])['independent_errors'])
        self.assertRaises(A.ModelDomainError,A.evaluate,18,[0x1000,0,3,0],Fixture())
        self.assertRaises(A.ModelDomainError,A.evaluate,18,[0x1000,2,0,0],Fixture())
    def test_new_read_failures_not_zero_and_wrong_expected_rejected(self):
        # 재사용Fixture의읽기예외는동결16도구의ModelDomainError subclass다.
        for pid,args in [(16,[1,1,0,0x1000]),(17,[0x1000,0,0,0]),(18,[0x1000,0,0,0])]:self.assertRaises((A.ModelDomainError,OLD.A.ModelDomainError),A.evaluate,pid,args,Fixture())
        e=event(17,[0x1000,0,0,0],delta(input=41),1);e['expected']=e['actual']='0';self.assertTrue(analyze([e])['independent_errors'])

if __name__=='__main__':unittest.main()
