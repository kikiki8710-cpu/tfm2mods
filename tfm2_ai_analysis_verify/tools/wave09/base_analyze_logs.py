"""동일 실행의 최종 summary와 bounded JSONL 관측을 오프라인 집계한다.

게임 호출/입력 변경은 하지 않는다. 일치 관측은 전체 분기 또는 의미동치 증명이 아니다.
"""
from __future__ import annotations
import argparse, collections, json
from pathlib import Path

BASELINE = 'a1d05045405a12e2bed182daeafcad8c8a40cf29ab68ba6ea5bfd30257439582'
KINDS = {'MATCH', 'MISMATCH', 'SKIP', 'UNSTABLE', 'FOREIGN_MATCH', 'FOREIGN_MISMATCH'}
COUNTERS = ('calls', 'sample_attempts', 'compared', 'matches', 'mismatches', 'skipped',
            'unstable', 'model_panics', 'dropped', 'foreign_matches', 'foreign_mismatches',
            'processing_errors', 'unlogged_errors')

def readj(path):
    return json.loads(Path(path).read_text(encoding='utf-8-sig'))

def integer(value):
    if isinstance(value, bool) or not isinstance(value, (str, int)):
        raise ValueError('bool은 정수가 아님')
    n = int(value, 0) if isinstance(value, str) and value.lower().startswith(('0x', '-0x')) else int(value)
    return n

def counter(row, key):
    n = integer(row.get(key, 0))
    if n < 0:
        raise ValueError('음수 counter: ' + key)
    return n

def metric_row(m):
    lhs, rhs = integer(m['lhs']), integer(m['rhs'])
    if not (-2**127 <= lhs < 2**127 and -2**127 <= rhs < 2**127):
        raise ValueError('metric i128 범위 초과')
    if type(m['predicate']) is not bool or type(m['observed_branch']) is not bool:
        raise ValueError('metric bool 필드 필요')
    return str(m['name']), 'below' if lhs < rhs else 'above' if lhs > rhs else 'equal'

def analyze(summary_path, observations_path, session_path=None, catalog_path=None):
    summary = readj(summary_path)
    if summary.get('schema_version') != 1 or not summary.get('run_id'):
        raise ValueError('summary 스키마 또는 run_id 누락')
    run = summary['run_id']
    session = readj(session_path) if session_path else None
    if session and session.get('run_id') != run:
        raise ValueError('session/summary run_id 불일치')
    baseline = summary.get('baseline_sha256')
    if baseline != BASELINE:
        raise ValueError('summary baseline SHA 불일치')
    if session and session.get('baseline_sha256', baseline) != baseline:
        raise ValueError('session/summary baseline SHA 불일치')
    catalog = readj(catalog_path) if catalog_path else {}
    if catalog and catalog.get('exe_sha256') != baseline:
        raise ValueError('catalog/summary baseline SHA 불일치')
    probes = {}
    for p in summary.get('probes', []):
        pid = integer(p['id'])
        if pid in probes:
            raise ValueError('summary 중복 probe id')
        counts = {k: counter(p, k) for k in COUNTERS}
        if counts['compared'] != counts['matches'] + counts['mismatches']:
            raise ValueError('summary compared != matches + mismatches')
        probes[pid] = dict(id=pid, rva=p['rva'], semantic_id=p.get('semantic_id'),
            install_status=p.get('install_status'), foreign_chain=bool(p.get('foreign_chain')),
            summary_counts=counts, emitted_counts=dict.fromkeys(sorted(KINDS), 0),
            reasons={}, threshold_coverage={}, output_field_differences=0,
            snapshot_predicate_observations=0, directly_observed_branch_observations=0,
            uncompared_metric_observations=0)
    errors, nonsamples, lines = [], 0, 0
    seen = set()
    with Path(observations_path).open(encoding='utf-8-sig') as stream:
        for line_no, line in enumerate(stream, 1):
            if not line.strip():
                continue
            lines += 1
            try:
                x = json.loads(line)
                if x.get('run_id') != run:
                    raise ValueError('JSONL/summary run_id 불일치')
                if x.get('schema_version') != 1:
                    raise ValueError('JSONL 스키마 불일치')
                if x.get('kind') != 'sample':
                    nonsamples += 1
                    continue
                pid = integer(x['probe_id'])
                p = probes[pid]
                if integer(x['rva']) != integer(p['rva']):
                    raise ValueError('probe id/RVA 불일치')
                kind, domain = x['comparison'], x['domain']
                if kind not in KINDS or domain not in ('vanilla', 'foreign_chain'):
                    raise ValueError('comparison/domain 오류')
                if kind.startswith('FOREIGN_') != (domain == 'foreign_chain') and kind not in ('SKIP', 'UNSTABLE'):
                    raise ValueError('foreign/vanilla comparison 혼합')
                args = x['args']
                if len(args) != 4:
                    raise ValueError('정수인자 4개 필요')
                for n in args:
                    if not 0 <= integer(n) < 2**64:
                        raise ValueError('인자 u64 범위 오류')
                metrics = [(m, metric_row(m)) for m in x.get('metrics', [])]
                for r in x.get('reads', []):
                    if r['width'] not in (1, 2, 4, 8) or not 0 <= integer(r['value']) < 2**(8*r['width']):
                        raise ValueError('read 폭/값 오류')
                key = (pid, x['thread_id'], x['seq'])
                if key in seen:
                    raise ValueError('중복 sample sequence')
                # 검증에 실패한 행은 어떠한 MATCH/경계 수에도 넣지 않는다.
                if kind in ('MATCH', 'MISMATCH', 'FOREIGN_MATCH', 'FOREIGN_MISMATCH'):
                    e, a, mask = (integer(x[k]) for k in ('expected', 'actual', 'mask'))
                    if not all(0 <= n < 2**64 for n in (e, a, mask)):
                        raise ValueError('반환 u64 범위 오류')
                    same = (e & mask) == (a & mask)
                    for o in x.get('output_fields', []):
                        width = integer(o['width'])
                        if width not in (1, 2, 4, 8):
                            raise ValueError('output 폭 오류')
                        oe, oa = integer(o['expected']), integer(o['actual'])
                        if not all(0 <= n < 2**(width*8) for n in (oe, oa)):
                            raise ValueError('output 값 범위 오류')
                        same = same and oe == oa
                    if same != (kind in ('MATCH', 'FOREIGN_MATCH')):
                        raise ValueError('comparison과 masked 반환 불일치')
                seen.add(key)
                p['emitted_counts'][kind] += 1
                reason = str(x.get('reason', ''))
                if reason:
                    p['reasons'][reason] = p['reasons'].get(reason, 0) + 1
                for m, (name, position) in metrics:
                    if kind in ('SKIP', 'UNSTABLE'):
                        p['uncompared_metric_observations'] += 1
                        continue
                    cov = p['threshold_coverage'].setdefault(name, dict(below=0, equal=0, above=0,
                        snapshot_predicate=0, observed_branch=0, relations={},
                        vanilla=dict(below=0, equal=0, above=0), foreign_chain=dict(below=0, equal=0, above=0)))
                    cov[position] += 1
                    cov[domain][position] += 1
                    field = 'observed_branch' if m['observed_branch'] else 'snapshot_predicate'
                    cov[field] += 1
                    rel = str(m['relation'])
                    cov['relations'][rel] = cov['relations'].get(rel, 0) + 1
                    p['directly_observed_branch_observations' if m['observed_branch'] else 'snapshot_predicate_observations'] += 1
                for o in x.get('output_fields', []):
                    if integer(o['expected']) != integer(o['actual']):
                        p['output_field_differences'] += 1
            except (ValueError, KeyError, TypeError, OverflowError, AttributeError) as exc:
                # 다른 실행이 섞인 파일은 일부 정상 행으로 성공 판정하지 않는다.
                if 'run_id' in str(exc):
                    raise ValueError(f'{line_no}행: {exc}') from exc
                errors.append(dict(line=line_no, error=str(exc)))
    for p in probes.values():
        c = p['summary_counts']
        emitted = p['emitted_counts']
        mapping = {'compared': emitted['MATCH'] + emitted['MISMATCH'],
            'matches': emitted['MATCH'], 'mismatches': emitted['MISMATCH'],
            'skipped': emitted['SKIP'], 'unstable': emitted['UNSTABLE'],
            'foreign_matches': emitted['FOREIGN_MATCH'], 'foreign_mismatches': emitted['FOREIGN_MISMATCH']}
        p['counter_log_deficits'] = {k: c[k] - logged for k, logged in mapping.items() if c[k] > logged}
        p['calls_not_represented_by_sample_events'] = max(0, c['calls'] - sum(emitted.values()))
        p['sample_attempts_not_represented_by_sample_events'] = max(0, c['sample_attempts'] - sum(emitted.values()))
        p['coverage_incomplete'] = bool(p['counter_log_deficits']) or p['calls_not_represented_by_sample_events'] > 0 or p['sample_attempts_not_represented_by_sample_events'] > 0
        p['no_calls_seen'] = c['calls'] == 0
        p['no_vanilla_comparison'] = c['compared'] == 0
        p['boundary_scope'] = 'snapshot predicate와 직접 관측 branch를 별도 집계; 전체 분기 완료 증명 아님'
        for cov in p['threshold_coverage'].values():
            cov['missing'] = [k for k in ('below', 'equal', 'above') if not cov[k]]
            cov['vanilla_missing'] = [k for k in ('below', 'equal', 'above') if not cov['vanilla'][k]]
            cov['foreign_chain_missing'] = [k for k in ('below', 'equal', 'above') if not cov['foreign_chain'][k]]
        matching_catalog = [q for q in catalog.get('functions', []) if integer(q['rva']) == integer(p['rva'])]
        p['declared_thresholds_not_named_in_log'] = [t['semantic_id'] for q in matching_catalog for t in q.get('thresholds', [])
            if t.get('semantic_id') not in p['threshold_coverage']]
    totals = {k: sum(p['summary_counts'][k] for p in probes.values()) for k in COUNTERS}
    emitted_vanilla = sum(p['emitted_counts'][k] for p in probes.values() for k in ('MATCH', 'MISMATCH'))
    io = counter(summary, 'io_errors')
    uncertain = counter(summary, 'io_lost_or_uncertain_events')
    loss = bool(io) or uncertain > 0 or totals['dropped'] > 0 or totals['unlogged_errors'] > 0 or bool(errors)
    processing = totals['processing_errors'] > 0
    coverage_incomplete = any(p['coverage_incomplete'] for p in probes.values())
    if not summary.get('identity_verified'):
        status = 'IDENTITY_NOT_VERIFIED'
    elif not summary.get('armed'):
        status = 'NOT_ARMED'
    elif summary.get('init_ready') is False:
        status = 'INITIALIZATION_NOT_READY'
    elif totals['mismatches']:
        status = 'VANILLA_MISMATCH_OBSERVED'
    elif loss:
        status = 'INCOMPLETE_WITH_LOG_LOSS'
    elif processing:
        status = 'INCOMPLETE_WITH_PROCESSING_ERRORS'
    elif totals['compared'] == 0:
        status = 'FOREIGN_ONLY_NO_VANILLA_COMPARISON' if totals['foreign_matches'] + totals['foreign_mismatches'] else 'NO_VANILLA_COMPARISON'
    elif emitted_vanilla == 0:
        status = 'SUMMARY_COMPARISONS_WITHOUT_EMITTED_VANILLA_EVIDENCE'
    elif coverage_incomplete:
        status = 'INCOMPLETE_WITH_COUNTER_LOG_GAPS'
    else:
        status = 'VANILLA_MATCH_OBSERVED_LIMITED_SCOPE'
    return dict(schema_version=1, status=status, run_id=run, baseline_sha256=baseline,
        identity_verified=bool(summary.get('identity_verified')), armed=bool(summary.get('armed')),
        catalog_total=summary.get('catalog_total'), prepared_probes=summary.get('prepared_probes'),
        summary_counts=totals, foreign_status='FOREIGN_MISMATCH_OBSERVED' if totals['foreign_mismatches'] else
            'FOREIGN_MATCH_OBSERVED' if totals['foreign_matches'] else 'NO_FOREIGN_COMPARISON',
        io_errors=io, io_lost_or_uncertain_events_upper_bound=uncertain,
        log_loss_or_corruption=loss, capture_processing_errors=processing,
        coverage_incomplete=coverage_incomplete, evidence_incomplete=loss or processing or coverage_incomplete,
        sample_stride=summary.get('sample_stride'), max_samples_per_probe=summary.get('max_samples_per_probe'), input_lines=lines,
        non_sample_lines=nonsamples, invalid_lines=errors, probes=list(probes.values()),
        source_files=dict(summary=str(Path(summary_path).resolve()), observations=str(Path(observations_path).resolve()),
            session=str(Path(session_path).resolve()) if session_path else None),
        limits=['summary는 최종 누적계수, JSONL은 제한 표본이다. 양자의 건수를 합산하지 않는다.',
            'io_lost_or_uncertain_events는 기록을 보장하지 못하는 batch 상한이며 이미 쓰인 일부도 포함할 수 있다. dropped/unlogged_errors와 더해 유일 사례 수로 만들지 않는다.',
            'counter/log 차이는 stride·예산·진행 중 snapshot·미기록 등을 포함할 수 있다. 차이 자체를 전부 IO 손실로 단정하지 않으며 증거 범위는 불완전으로 표시한다.',
            'foreign_chain 일치는 vanilla 일치와 별도이며 원본 게임 의미동치 증명이 아니다.',
            'SKIP/UNSTABLE/read fault/model panic은 비교 성공 건수에 포함하지 않는다.',
            'observed_branch=false metric은 snapshot에서 계산한 가상 predicate이며 실제 branch 측정이 아니다.',
            '모든 관측 MATCH도 전체 분기·입력·실 provider/runtime 완료를 의미하지 않는다.'])

def markdown(out):
    lines = ['# AI 관측 오프라인 집계', '', f"실행 `{out['run_id']}`: **{out['status']}**", '',
        '함수 | 호출 | vanilla 비교 | 일치 | 차이 | skip | 불안정 | foreign 일치/차이 | drop',
        '--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---:']
    for p in out['probes']:
        c = p['summary_counts']
        lines.append(f"{p['rva']} | {c['calls']} | {c['compared']} | {c['matches']} | {c['mismatches']} | {c['skipped']} | {c['unstable']} | {c['foreign_matches']}/{c['foreign_mismatches']} | {c['dropped']}")
    lines += ['', f"증거 불완전: {out['evidence_incomplete']}; 로그 손실/손상: {out['log_loss_or_corruption']}; 처리 오류: {out['summary_counts']['processing_errors']}; 미기록 오류: {out['summary_counts']['unlogged_errors']}; 잘못된 행: {len(out['invalid_lines'])}",
        f"기록 불확실 이벤트 상한: {out['io_lost_or_uncertain_events_upper_bound']} (중복 가능, 사례 합산 금지)", '']
    for p in out['probes']:
        lines += [f"## {p['rva']}", '', f"호출 없음: {p['no_calls_seen']}; vanilla 비교 없음: {p['no_vanilla_comparison']}", '']
        lines += [f"표본으로 표현되지 않은 호출/시도: {p['calls_not_represented_by_sample_events']}/{p['sample_attempts_not_represented_by_sample_events']}; 비교 계수/로그 부족분: {p['counter_log_deficits']}", '']
        for name, c in p['threshold_coverage'].items():
            lines.append(f"- `{name}`: 아래/동일/위={c['below']}/{c['equal']}/{c['above']}, 미관측={','.join(c['missing']) or '없음'}; snapshot={c['snapshot_predicate']}, 직접branch={c['observed_branch']}")
            lines.append(f"  - vanilla={c['vanilla']}, foreign={c['foreign_chain']}, vanilla 미관측={','.join(c['vanilla_missing']) or '없음'}")
        if p['declared_thresholds_not_named_in_log']:
            lines.append('- 로그 이름과 연결되지 않은 선언 임계값: ' + ', '.join(p['declared_thresholds_not_named_in_log']))
    lines += ['', *('- ' + s for s in out['limits']), '']
    return '\n'.join(lines)

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--summary', type=Path, required=True)
    parser.add_argument('--observations', type=Path, required=True)
    parser.add_argument('--session', type=Path)
    parser.add_argument('--catalog', type=Path)
    parser.add_argument('--output-dir', type=Path, required=True)
    args = parser.parse_args()
    out = analyze(args.summary, args.observations, args.session, args.catalog)
    args.output_dir.mkdir(parents=True, exist_ok=True)
    (args.output_dir/'analysis.json').write_text(json.dumps(out, ensure_ascii=False, indent=2)+'\n', encoding='utf-8')
    (args.output_dir/'analysis.md').write_text(markdown(out), encoding='utf-8')
    print(out['status'])

if __name__ == '__main__':
    main()
