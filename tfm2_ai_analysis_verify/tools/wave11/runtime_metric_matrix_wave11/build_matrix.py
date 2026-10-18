"""53모델/162지표 명세와 고정 STOP 표본을 결박하는 REPORT 전용 집계기."""
from pathlib import Path
import argparse,collections,csv,hashlib,importlib.util,json,shutil
H=Path(__file__).resolve().parent;B=H.parent
ANALYZER=B/'analyze_logs_wave11.py'
PLAN=B/'metric_plan_wave11/matrix_plan.json'
BASELINE='a1d05045405a12e2bed182daeafcad8c8a40cf29ab68ba6ea5bfd30257439582'

def sha(p):
    h=hashlib.sha256()
    with Path(p).open('rb')as f:
        for block in iter(lambda:f.read(1024*1024),b''):h.update(block)
    return h.hexdigest()
def no_duplicates(pairs):
    d={}
    for k,v in pairs:
        if k in d:raise ValueError('중복JSON키:'+k)
        d[k]=v
    return d
def load(p):return json.loads(Path(p).read_text(encoding='utf-8-sig'),object_pairs_hook=no_duplicates)
def save(p,x):Path(p).write_text(json.dumps(x,ensure_ascii=False,indent=2)+'\n',encoding='utf8')
def module(p,n):
    spec=importlib.util.spec_from_file_location(n,p);m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m);return m

def prepare():
    frozen=load(B/'source_pin_manifest.json')['files']
    assert frozen[str(PLAN.resolve())]==sha(PLAN)and frozen[str(ANALYZER.resolve())]==sha(ANALYZER)
    p=load(PLAN)
    assert p['prepared_models']==53 and p['named_metric_entries']==162
    p.update(schema_version=1,expected_run_id=None,run_id=None,input_origin='unknown',runtime_samples_received=0,
        actual_runtime_comparisons_rechecked=0,fixture_comparisons=0,loaded_dll_identity_verified=False,
        source_commit=None,reported_deployed_dll_sha256=None,functions=[],
        readonly_sources_sha256={str(PLAN.resolve()):sha(PLAN),str(ANALYZER.resolve()):sha(ANALYZER),
            str((B/'source_pin_manifest.json').resolve()):sha(B/'source_pin_manifest.json'),
            str(Path(__file__).resolve()):sha(Path(__file__))})
    a=module(ANALYZER,'v025_matrix_analyzer_dependency')
    for i,rva in enumerate(a.RVAS):
        p['functions'].append(dict(probe_id=i,rva=hex(rva),runtime_compared_samples=None,
            runtime_output_write_samples=None,all_coefficients_verified=False))
    return p

def aggregate(events,name,denominator):
    # v023/v024의 동일 집계 정의: 같은 함수표본 안에서 지표가 반복돼도 sample분모는 한 번.
    pairs=collections.Counter();sample_lines=set();yes=set();no=set();counts=collections.Counter()
    for line,event in events:
        for m in event['metrics']:
            if m['name']!=name:continue
            lhs,rhs=int(m['lhs']),int(m['rhs']);assert m['observed_branch']is False
            pairs[lhs,rhs]+=1;sample_lines.add(line);(yes if m['predicate']else no).add(line)
            counts['below'if lhs<rhs else'equal'if lhs==rhs else'above']+=1
            counts['predicate_true'if m['predicate']else'predicate_false']+=1
    lhs=[a for a,b in pairs];rhs=[b for a,b in pairs];diff=[a-b for a,b in pairs]
    return dict(sample_denominator=denominator,samples_evaluating_metric=len(sample_lines),
        samples_not_evaluating_metric=denominator-len(sample_lines),metric_evaluation_denominator=sum(pairs.values()),
        **{k:counts[k]for k in ('below','equal','above','predicate_true','predicate_false')},
        samples_with_true=len(yes),samples_with_false=len(no),samples_with_both_truths=len(yes&no),
        exact_rhs_minus_one=sum(c for(a,b),c in pairs.items()if a==b-1),
        exact_rhs_plus_one=sum(c for(a,b),c in pairs.items()if a==b+1),
        lhs_range=[str(min(lhs)),str(max(lhs))]if lhs else None,rhs_range=[str(min(rhs)),str(max(rhs))]if rhs else None,
        distinct_rhs_count=len(set(rhs)),relative_difference_range=[str(min(diff)),str(max(diff))]if diff else None,
        observed_value_pairs=[dict(lhs=str(a),rhs=str(b),evaluations=c)for(a,b),c in sorted(pairs.items())],
        missing_buckets=[x for x in ('below','equal','above')if not counts[x]],
        direct_original_branch_observations=0,all_coefficients_verified=False,
        runtime_status='PREDICATE_OBSERVED_ONLY'if pairs else'METRIC_NOT_OBSERVED')

def copy_source(path,out):
    path=Path(path).resolve();dest=out/'source_snapshot';dest.mkdir(exist_ok=True)
    q=dest/(sha(path)[:16]+'_'+path.name)
    shutil.copyfile(path,q);assert sha(path)==sha(q)
    return dict(original_path=str(path),snapshot_path=str(q.resolve()),sha256=sha(path))

def fixed_matrix(plan,summary_path,raw_path,analysis_path,out,origin,expected_run_id,source_commit=None,dll_sha=None):
    assert origin in ('runtime','python_fixture','native_fixture')and expected_run_id
    copies=[copy_source(p,out)for p in (summary_path,raw_path,analysis_path)]
    summary,analysis=load(copies[0]['snapshot_path']),load(copies[2]['snapshot_path'])
    assert summary['run_id']==analysis['run_id']==expected_run_id
    assert analysis['expected_run_id']==expected_run_id and analysis['input_origin']==origin
    if origin=='runtime':assert analysis.get('journal_freshness_verified') is True and analysis.get('journal_audit',{}).get('status')=='PASS_FIXED_STOP_JOURNAL'
    assert analysis['supported_model_ids']==list(range(53))
    assert summary['prepared_probes']==53 and {p['id']for p in summary['probes']}==set(range(53))
    assert len(summary['probes'])==53
    assert summary['baseline_sha256']==BASELINE
    assert summary['actual_sha256']==BASELINE or (origin=='native_fixture'and summary['actual_sha256']=='ENGINE_SELFTEST_NOT_A_GAME_SESSION')
    assert summary['identity_verified']is True and summary['armed']is True and summary['init_ready']is True
    assert summary['capture_enabled']is False and summary['in_flight']==summary['queue_pending']==summary['queued_events']==0
    assert summary['control_sequence']==summary['stop_pending_sequence']and summary['control_sequence']>=2
    assert not summary.get('control_error')
    assert not analysis['independent_errors']and not analysis['counter_log_excesses']
    source_pins=analysis['source_files']
    for p in (summary_path,raw_path,ANALYZER):assert source_pins[str(Path(p).resolve())]==sha(p)
    for p,h in source_pins.items():
        assert sha(p)==h,('독립분석이후source변경',p)
        plan['readonly_sources_sha256'][p]=h
    if dll_sha:
        assert len(dll_sha)==64 and all(c in '0123456789abcdef'for c in dll_sha)
    a=module(ANALYZER,'v025_matrix_fresh_snapshot_recalculation')
    counters={p['id']:p for p in summary['probes']}
    plan['probe_selection_scope']=a.selection_scope(summary)
    for i,p in counters.items():assert a.u64(p['rva'])==a.RVAS[i]
    wanted={}
    for p in analysis['probes']:
        assert p['rva']==hex(a.RVAS[p['id']])
        for sample in p['raw_validated_samples']:
            assert sample['line']not in wanted
            wanted[sample['line']]=(p['id'],sample)
    valid={i:{'vanilla':[],'foreign_chain':[]}for i in range(53)}
    kinds=collections.Counter();raw_kinds=collections.Counter();seqs=set();seen=set();write_samples=collections.Counter()
    output_fields=0;raw_rows=0
    raw_bytes=Path(copies[1]['snapshot_path']).read_bytes()
    assert not raw_bytes or raw_bytes.endswith(b'\n')
    for line_no,text in enumerate(raw_bytes.decode('utf-8-sig').splitlines(),1):
        if not text.strip():continue
        event=json.loads(text,object_pairs_hook=no_duplicates)
        assert event['run_id']==expected_run_id
        assert event.get('kind')=='sample'and type(event.get('schema_version'))is int and event['schema_version']==1
        pid=event['probe_id'];assert type(pid)is int and pid in counters and a.u64(event['rva'])==a.RVAS[pid]
        seq=a.u64(event['seq']);assert seq not in seqs;seqs.add(seq);raw_rows+=1
        raw_kinds[event['comparison']]+=1
        if line_no not in wanted:
            assert event['comparison']in ('SKIP','UNSTABLE')and event['output_fields']==[]
            continue
        seen.add(line_no);expected_pid,proof=wanted[line_no]
        assert pid==expected_pid and seq==proof['seq']
        snapshot=a.Snapshot(event['reads']);ev=a.evaluate(pid,event['args'],snapshot);snapshot.exact_consumption()
        assert a.u64(event['expected'])==ev['expected']==a.u64(proof['independent_expected'])
        assert a.u64(event['mask'])==ev['mask']==a.u64(proof['mask'])
        assert a.u64(event['actual'])==a.u64(proof['actual'])
        a.check_metrics(event['metrics'],ev['metrics'])
        same=((a.u64(event['actual'])^ev['expected'])&ev['mask'])==0
        output_same=a.check_outputs(event['output_fields'],ev['output_fields'])
        same=bool(same and output_same)
        domain=event['domain'];assert domain in valid[pid]
        outcome=('VANILLA'if domain=='vanilla'else'FOREIGN')+('_MATCH'if same else'_MISMATCH')
        assert proof['outcome']==outcome
        assert same==(event['comparison']in ('MATCH','FOREIGN_MATCH'))
        assert event['comparison'].startswith('FOREIGN_')==(domain=='foreign_chain')
        valid[pid][domain].append((line_no,dict(metrics=ev['metrics'])))
        kinds[outcome]+=1
        if domain=='vanilla'and event['output_fields']:write_samples[pid]+=1;output_fields+=len(event['output_fields'])
    assert seen==set(wanted)
    comparison_counts={k:v for k,v in analysis['independent_counts'].items()if k in ('VANILLA_MATCH','VANILLA_MISMATCH','FOREIGN_MATCH','FOREIGN_MISMATCH')}
    assert dict(kinds)==comparison_counts
    vanilla=kinds['VANILLA_MATCH']+kinds['VANILLA_MISMATCH']
    assert analysis['actual_runtime_comparisons_rechecked']==(vanilla if origin=='runtime'else 0)
    for f in plan['functions']:
        pid=f['probe_id'];f.update(runtime_compared_samples=len(valid[pid]['vanilla']),
            runtime_output_write_samples=write_samples[pid],summary_counters=counters[pid])
    for row in plan['rows']:
        pid=row['runtime_id'];row['domains']={name:aggregate(events,row['metric_name'],len(events))for name,events in valid[pid].items()}
        z=row['domains']['vanilla']
        row.update(runtime_samples=len(valid[pid]['vanilla']),metric_evaluations=z['metric_evaluation_denominator'],
            below=z['below'],equal=z['equal'],above=z['above'],exact_T_minus_one=z['exact_rhs_minus_one'],
            exact_T=z['equal'],exact_T_plus_one=z['exact_rhs_plus_one'],predicate_true=z['predicate_true'],predicate_false=z['predicate_false'])
    gaps={k:sum(counters[i][k]for i in counters)for k in ('dropped','unlogged_errors','processing_errors')}
    gaps['io_lost_or_uncertain_events']=summary['io_lost_or_uncertain_events']
    status=('FIXED_CAPTURE_MATRIX_LIMITED_SCOPE'if vanilla else'FIXED_CAPTURE_NO_VANILLA_COMPARISONS')if origin=='runtime'else'FIXTURE_MATRIX_NO_RUNTIME_SAMPLES'
    plan.update(status=status,
        expected_run_id=expected_run_id,run_id=expected_run_id,input_origin=origin,source_snapshots=copies,
        source_commit=source_commit,reported_deployed_dll_sha256=dll_sha,loaded_dll_identity_verified=False,
        actual_runtime_samples=raw_rows if origin=='runtime'else 0,
        actual_runtime_samples_definition='FIXED_INPUT_RAW_SAMPLE_RECORDS_NOT_NEW_EXECUTION',
        runtime_samples_received=raw_rows if origin=='runtime'else 0,raw_sample_records=raw_rows,
        actual_runtime_comparisons_rechecked=vanilla if origin=='runtime'else 0,
        fixture_comparisons=vanilla if origin!='runtime'else 0,comparison_counts=dict(kinds),raw_comparison_labels=dict(raw_kinds),
        independent_analysis_status=analysis['status'],output_fields_compared=output_fields,
        observed_functions=sum(bool(f['runtime_compared_samples'])for f in plan['functions']),
        metric_rows_with_function_sample_denominator=sum(bool(r['runtime_samples'])for r in plan['rows']),
        metric_entries_evaluated=sum(bool(r['metric_evaluations'])for r in plan['rows']),
        metric_evaluations=sum(r['metric_evaluations']for r in plan['rows']),
        reported_observation_loss=gaps,hard_io=dict(io_errors=summary['io_errors'],summary_alias_errors=summary.get('summary_alias_errors',0)),
        loaded_file_constant_verification_scope='NOT_INDEPENDENTLY_RECHECKED_BY_THIS_MATRIX',
        evidence_context='ROOT_FIXED_CAPTURE_CONTEXT_NOT_AUTOMATICALLY_SINGLE_REPLAY',new_game_calls=0,
        helper_rechecks_not_new_runtime_samples=True,vanilla_metric_scope='MATCH_AND_MISMATCH_SEPARATELY_REPORTED',all_coefficients_verified=False)
    return plan

def write(plan,out):
    save(out/'matrix.json',plan)
    actual=plan['input_origin']=='runtime'
    received=f"실측 {plan['runtime_samples_received']:,}줄, 독립대조 {plan['actual_runtime_comparisons_rechecked']:,}건을 수신했다."if actual else'실제 게임 실측은 아직0이다. 아래 값이 있으면 Python/native fixture 검산이며 게임 표본이 아니다.'
    lines=['# v030 53모델·162지표 관측표','',plan['status'],'',received,
        '지표 종류·동적 평가 횟수·계수 개수·고유 함수 수는 별도 분모다. 정확T±1은metric operand 단위이며원본내부분기trace0/전체계수완료false를유지한다.','',
        '|id|RVA|의미|함수표본/지표평가|아래/동일/위|정확T−1/T/T+1|predicate true/false|',
        '|---:|---|---|---|---|---|---|']
    for r in plan['rows']:
        z=r.get('domains',{}).get('vanilla',{})
        lines.append(f"|{r['runtime_id']}|{r['rva']}|{r['meaning']}|{r['runtime_samples']}/{r['metric_evaluations']}|{r['below']}/{r['equal']}/{r['above']}|{r['exact_T_minus_one']}/{r['exact_T']}/{r['exact_T_plus_one']}|{r['predicate_true']}/{r['predicate_false']}|")
    lines+=['','D8은실제쓴QWORD만/무쓰기일때도fullRAX대조. D3은seed0/8 Q와lineBYTE의정확쓰기. None의미쓰기payload/padding은정답을발명하지않는다. mask0인runtime37 None은RAX임의값허용/실BYTE출력만대조한다. runtime38은scratch39이며차단된1016B00은route에없다.','']
    (out/'matrix.md').write_text('\n'.join(lines),encoding='utf8')
    with(out/'matrix.csv').open('w',encoding='utf-8-sig',newline='')as stream:
        columns=['runtime_id','rva','metric_name','meaning','original_value','runtime_samples','metric_evaluations','below','equal','above','exact_T_minus_one','exact_T','exact_T_plus_one','predicate_true','predicate_false']
        w=csv.DictWriter(stream,columns,extrasaction='ignore');w.writeheader()
        for row in plan['rows']:
            x=dict(row);x['original_value']=json.dumps(x['original_value'],ensure_ascii=False);w.writerow(x)
    files={str(p.resolve()):sha(p)for p in out.rglob('*')if p.is_file()and p.name!='freeze_manifest.json'}
    save(out/'freeze_manifest.json',dict(status='FROZEN_MATRIX_OUTPUT',files=files,
        readonly_dependencies=plan['readonly_sources_sha256'],source_tool_sha256=sha(Path(__file__)),
        actual_runtime_comparisons=plan['actual_runtime_comparisons_rechecked'],new_game_calls=0,all_coefficients_verified=False))

def main():
    p=argparse.ArgumentParser(description=__doc__);p.add_argument('--plan',action='store_true')
    for name in ('summary','observations','analysis-json','output-dir'):p.add_argument('--'+name,type=Path,required=name=='output-dir')
    p.add_argument('--expected-run-id');p.add_argument('--input-origin',choices=('unknown','runtime','python_fixture','native_fixture'),default='unknown')
    p.add_argument('--source-commit');p.add_argument('--dll-sha256')
    args=p.parse_args();out=args.output_dir.resolve()
    assert (out.is_relative_to(H.resolve())or out.is_relative_to((B.parent/'analyzer_wave11_runtime_matrix').resolve()))and(not out.exists()or not any(out.iterdir())),'기존출력덮어쓰기/소유범위밖쓰기금지'
    out.mkdir(parents=True,exist_ok=True);plan=prepare()
    if not args.plan:
        assert args.summary and args.observations and args.analysis_json
        plan=fixed_matrix(plan,args.summary,args.observations,args.analysis_json,out,args.input_origin,args.expected_run_id,args.source_commit,args.dll_sha256)
    write(plan,out);print(plan['status'])

if __name__=='__main__':main()
