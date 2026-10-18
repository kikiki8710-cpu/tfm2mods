"""고정자료결박/39모델 remap/부분출력/fixture-origin 집계 시험. 실제표본0."""
from pathlib import Path
import copy,hashlib,importlib.util,json
H=Path(__file__).resolve().parent;B=H.parent
spec=importlib.util.spec_from_file_location('matrix39_test',H/'build_matrix.py');M=importlib.util.module_from_spec(spec);spec.loader.exec_module(M)
A=M.module(M.ANALYZER,'matrix39_fixture_analyzer');SOURCE=B/'test_cases'
RESULT=[]
def save(p,x):M.save(p,x)
def load(p):return M.load(p)
def trial(name,source='normal_new2_255',change_summary=None,change_raw=None,change_analysis=None,
          origin='python_fixture',expected_run=None,want='valid',analysis_origin='python_fixture'):
    out=H/'test_cases'/name;out.mkdir(parents=True,exist_ok=True)
    s=load(SOURCE/source/'summary.json')
    s.update(capture_enabled=False,in_flight=0,queue_pending=0,queued_events=0,control_sequence=2,stop_pending_sequence=2)
    rows=[json.loads(t)for t in(SOURCE/source/'observations.jsonl').read_text(encoding='utf8').splitlines()if t.strip()]
    if change_summary:change_summary(s)
    if change_raw:change_raw(rows)
    save(out/'summary.json',s)
    (out/'observations.jsonl').write_text(''.join(json.dumps(x)+'\n'for x in rows),encoding='utf8')
    # 형식오류 자체는 별도 fresh 분석기 시험에서 이미 다루었다. 이 시험은 결박/후처리 gate다.
    try:
        a=A.analyze(out/'summary.json',out/'observations.jsonl',input_origin=analysis_origin,expected_run_id=s['run_id'])
    except(ValueError,AssertionError)as e:
        assert want=='reject';RESULT.append(dict(name=name,status='PASS',error=str(e),phase='independent_analyzer',actual_runtime_samples=0));return
    if change_analysis:change_analysis(a,out)
    save(out/'analysis.json',a)
    dest=out/'matrix';dest.mkdir(exist_ok=True)
    error=None;matrix=None
    try:matrix=M.fixed_matrix(M.prepare(),out/'summary.json',out/'observations.jsonl',out/'analysis.json',dest,origin,expected_run or s['run_id'])
    except(AssertionError,ValueError,KeyError)as e:error=str(e)or'gate 거절'
    if want=='reject':assert error is not None,(name,'wrong input accepted')
    else:
        assert error is None,(name,error)
        assert matrix['expected_run_id']==s['run_id']
        assert matrix['actual_runtime_comparisons_rechecked']==matrix['runtime_samples_received']==0
        assert matrix['input_origin']==analysis_origin and matrix['all_coefficients_verified']is False
        assert matrix['loaded_dll_identity_verified']is False
        M.write(matrix,dest)
        assert '실제 게임 실측은 아직0' in(dest/'matrix.md').read_text(encoding='utf8')
        assert '현재plan의관측값/분모null은미수신이다'not in(dest/'matrix.md').read_text(encoding='utf8')
        if source=='normal_new2_255':
            assert matrix['fixture_comparisons']==255 and matrix['observed_functions']==2
            assert matrix['functions'][37]['runtime_compared_samples']==86
            assert matrix['functions'][38]['runtime_compared_samples']==169
            assert len(matrix['rows'])==124 and not any(r['rva']=='0x1016b00'for r in matrix['rows'])
    RESULT.append(dict(name=name,status='PASS',error=error,expectation=want,actual_runtime_samples=0))

def main():
    trial('normal255_fixture_only')
    trial('new41_two_429',source='normal_new39_40_429')
    trial('wave08_all_routes',source='wave08_all_normal')
    trial('new47_stacked418',source='stacked47_normal418')
    trial('new47_guard19',source='stacked47_game_guard19_SKIP')
    trial('new47_selected13',source='prepared47_selected13')
    trial('new47_selected13_installed12',source='selected13_installed12_refused1')
    trial('wave08_selected10_scope',source='prepared44_selected10')
    trial('wave08_selected10_installed9_refused1',source='selected10_installed9_safety_refusal1')
    trial('new_insert_nowrite',source='insert_no_write_full_rax')
    trial('new_insert_written_words',source='insert_actual_words_only')
    trial('new_peer_seed_QQBYTE',source='peer_seed_no_candidate_QQBYTE')
    trial('domain_reject26_no_comparison',source='domain_reject_new37_26')
    trial('None_RAX_arbitrary_mask0',source='None_RAX_arbitrary_ffffffffffffffff')
    trial('Some_exact_fields',source='Some_Q_Q_BYTE_order')
    trial('iterator_empty_partial',source='iterator_empty_tag_only')
    trial('iterator_exhausted_cursor',source='iterator_exhausted_cursor_then_tag')
    trial('iterator_some_exact_order',source='iterator_Some_cursor_id_xy_tag_order')
    trial('old10_route990',source='old10_fixture_route_regression990')
    trial('empty_fixture_no_fake_pass',source='empty_no_pass')
    trial('native_sentinel_not_game',source='None_defined_BYTE_only',origin='native_fixture',analysis_origin='native_fixture',
          change_summary=lambda s:s.__setitem__('actual_sha256','ENGINE_SELFTEST_NOT_A_GAME_SESSION'))
    trial('wrong_expected_run',expected_run='OTHER_RUN',want='reject')
    trial('fixture_not_runtime',origin='runtime',want='reject')
    trial('wrong_analysis_run',change_analysis=lambda a,o:a.__setitem__('run_id','OTHER'),want='reject')
    trial('wrong_analysis_expected_run',change_analysis=lambda a,o:a.__setitem__('expected_run_id',None),want='reject')
    trial('analysis_supported_old37',change_analysis=lambda a,o:a.__setitem__('supported_model_ids',list(range(37))),want='reject')
    trial('summary_capture_still_on',change_summary=lambda s:s.__setitem__('capture_enabled',True),want='reject')
    trial('summary_inflight_nonzero',change_summary=lambda s:s.__setitem__('in_flight',1),want='reject')
    trial('summary_pending_queue',change_summary=lambda s:s.__setitem__('queue_pending',1),want='reject')
    trial('stop_ack_mismatch',change_summary=lambda s:s.__setitem__('stop_pending_sequence',1),want='reject')
    trial('stop_control_error',change_summary=lambda s:s.__setitem__('control_error','STOP refused'),want='reject')
    def missing_source(a,o):a['source_files'].pop(str(M.ANALYZER.resolve()))
    trial('analysis_missing_analyzer_pin',change_analysis=missing_source,want='reject')
    def forged_summary(a,o):a['source_files'][str((o/'summary.json').resolve())]='0'*64
    trial('analysis_summary_sha_stale',change_analysis=forged_summary,want='reject')
    trial('analysis_raw_sha_stale',change_analysis=lambda a,o:a['source_files'].__setitem__(str((o/'observations.jsonl').resolve()),'0'*64),want='reject')
    trial('analysis_error_not_pass',change_analysis=lambda a,o:a['independent_errors'].append(dict(error='fake')),want='reject')
    trial('analysis_counter_excess',change_analysis=lambda a,o:a['counter_log_excesses'].append(dict(id=37)),want='reject')
    def coforge(a,o):
        sample=next(p for p in a['probes']if p['id']==37)['raw_validated_samples'][0]
        sample['independent_expected']='0x1'
    trial('analysis_expected_not_oracle',change_analysis=coforge,want='reject')
    trial('blocked_runtime38_RVA',change_raw=lambda rows:rows[-1].__setitem__('rva','0x1016b00'),want='reject')
    # 원본 SHA pin이 바뀌면 재계산 완료라는 선언만으로 수용하지 않는다.
    assert len(M.prepare()['rows'])==124
    def met(lhs,rhs):return dict(name='unit',lhs=lhs,rhs=rhs,predicate=lhs<=rhs,observed_branch=False)
    events=[(1,dict(metrics=[met(9,10),met(10,10)])),(2,dict(metrics=[met(11,10),met(10,10)]))]
    d=M.aggregate(events,'unit',2)
    assert [d[k]for k in ('below','equal','above','exact_rhs_minus_one','exact_rhs_plus_one')]==[1,2,1,1,1]
    assert d['metric_evaluation_denominator']==4 and d['sample_denominator']==d['samples_evaluating_metric']==2
    assert d['samples_with_both_truths']==1 and d['predicate_true']==3 and d['predicate_false']==1
    RESULT.append(dict(name='aggregate_exact_boundary_and_repeated_sample_denominator',status='PASS',actual_runtime_samples=0))
    oldpath=B.parents[2]/'runtime_20261018_round02/runtime_metric_matrix_v023/build_matrix.py'
    old=M.module(oldpath,'old_v023_aggregation_readonly')
    assert old.aggregate_metric(events,'unit',2)==d
    RESULT.append(dict(name='old_v024_reused_aggregation_definition_equal',status='PASS',actual_runtime_samples=0))
    save(H/'validation.json',dict(status='PASS_FIXTURE_MATRIX_PREPARATION_ONLY',tests=len(RESULT),results=RESULT,
        actual_runtime_samples=0,native_executed=0,new_game_calls=0,added_main_cases=0,
        fixture_event_source='기존동결255정상/26SKIP 및990회귀의후처리시험. 새모델증명사례로중복계상하지않는다.'))
    print('PASS',len(RESULT),'matrix tests / runtime0')

if __name__=='__main__':main()
