"""고정741 집합의 미배선/미관측을 숨기지 않는 함수별 실측 원장."""
import argparse,collections,json
from pathlib import Path
p=argparse.ArgumentParser();p.add_argument('--scope',required=True);p.add_argument('--summary');p.add_argument('--output-dir',required=True);a=p.parse_args()
root=Path(__file__).resolve().parents[1]
catalog=json.loads((root/'catalog.json').read_text(encoding='utf-8'))
scope=json.loads(Path(a.scope).read_text(encoding='utf-8'))
selected={x['rva']:x for x in scope['functions']}
summary=json.loads(Path(a.summary).read_text(encoding='utf-8')) if a.summary else {}
runtime={x['rva']:x for x in summary.get('probes',[])}
rows=[]
for f in catalog['functions']:
    rva=hex(int(f['rva'],16));s=selected.get(rva);r=runtime.get(rva,{})
    state='UNWIRED'
    if s: state='NO_SAMPLES'
    if r.get('compared',0): state='OBSERVED_OUTPUT_MATCH_ONLY' if not r.get('mismatches',0) else 'MISMATCH_OBSERVED'
    if r.get('dropped',0) or summary.get('io_lost_or_uncertain_events',0): state+=':INCOMPLETE_RECORDS'
    # 명칭으로 자동 제외하지 않는다. deathmatch 여부는 의미 근거가 있을 때만 별도 판정한다.
    rows.append(dict(rva=rva,semantic_id=f['semantic_id'],name=f['name'],source=f['source'],bytes=f['bytes'],
        body_sha256=f['body_sha256'],prepared=bool(s),probe_id=s['id'] if s else None,state=state,
        calls=r.get('calls',0),compared=r.get('compared',0),mismatches=r.get('mismatches',0),
        persisted=r.get('persisted',0),static_numeric_records=f['thresholds'],
        all_thresholds_runtime_verified=False,deathmatch_excluded=False,
        rare_situation_unobserved_acceptable=True,apply_allowed=False))
out=Path(a.output_dir);out.mkdir(parents=True,exist_ok=True)
j=dict(census_total=len(rows),prepared=sum(x['prepared'] for x in rows),
    observed_functions=sum(x['compared']>0 for x in rows),all_thresholds_verified_functions=0,
    run_id=summary.get('run_id'),counts=dict(collections.Counter(x['state'] for x in rows)),functions=rows,
    scope='출력 일치·임계점 경계관측·모든계수 검증을 구별. static_numeric_records에는 비튜닝상수 포함 가능.')
(out/'function_coverage.json').write_text(json.dumps(j,ensure_ascii=False,indent=2),encoding='utf-8')
lines=['# 전체 함수 검증 원장','',f"고정 {len(rows)}개 / 배선 {j['prepared']}개 / 출력 관측 {j['observed_functions']}개. 모든 임계점 검증 완료로 승격한 함수는 0개.",'', '|RVA|함수|배선|비교 표본|차이|상태|','|---|---|---|---:|---:|---|']
lines += [f"|{x['rva']}|{x['name'].replace('|','/')}|{x['prepared']}|{x['compared']}|{x['mismatches']}|{x['state']}|" for x in rows]
(out/'function_coverage.md').write_text('\n'.join(lines)+'\n',encoding='utf-8')
print(json.dumps({k:v for k,v in j.items() if k!='functions'},ensure_ascii=False))
