"""오프라인 도구의 실행 경계·집계 실패 조건을 검사한다. 게임 검증 사례가 아니다."""
import importlib.util, json, sys, tempfile, unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT/'tools'))
import analyze_logs as logs
import generate_catalog as catalog
import capstone

class LogsTest(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.dir = Path(self.temp.name)
        self.summary = dict(schema_version=1, run_id='run1', baseline_sha256=logs.BASELINE,
            identity_verified=True, armed=True, catalog_total=741, prepared_probes=1, io_errors=0,
            probes=[dict(id=0, rva='0xdd87a0', semantic_id='test.forecast', install_status='INSTALLED',
                foreign_chain=False, **dict.fromkeys(logs.COUNTERS, 0))])
    def sample(self, kind='MATCH', seq=1, domain='vanilla'):
        return dict(schema_version=1, run_id='run1', kind='sample', probe_id=0,
            rva='0xdd87a0', thread_id=10, seq=seq, comparison=kind, reason='', domain=domain,
            args=['0x0']*4, expected='0x1', actual='0x1', mask='0xff', reads=[], metrics=[], output_fields=[])
    def run_analysis(self, rows=(), session=None):
        (self.dir/'summary.json').write_text(json.dumps(self.summary), encoding='utf8')
        (self.dir/'observations.jsonl').write_text(''.join(json.dumps(x)+'\n' for x in rows), encoding='utf8')
        if session:
            (self.dir/'session.json').write_text(json.dumps(session), encoding='utf8')
        return logs.analyze(self.dir/'summary.json', self.dir/'observations.jsonl', self.dir/'session.json' if session else None)
    def test_zero_samples_is_not_success(self):
        out = self.run_analysis()
        self.assertEqual(out['status'], 'NO_VANILLA_COMPARISON')
        self.assertTrue(out['probes'][0]['no_calls_seen'])
    def test_foreign_only_not_vanilla(self):
        self.summary['probes'][0].update(calls=1, sample_attempts=1, foreign_matches=1)
        out = self.run_analysis([self.sample('FOREIGN_MATCH', domain='foreign_chain')])
        self.assertEqual(out['status'], 'FOREIGN_ONLY_NO_VANILLA_COMPARISON')
        self.assertEqual(out['summary_counts']['compared'], 0)
    def test_fault_and_unstable_not_compared(self):
        self.summary['probes'][0].update(calls=2, sample_attempts=2, skipped=1, unstable=1)
        a, b = self.sample('SKIP'), self.sample('UNSTABLE', seq=2)
        a['reason'] = 'READ_FAULT'
        out = self.run_analysis([a, b])
        self.assertEqual(out['summary_counts']['compared'], 0)
        self.assertEqual(out['probes'][0]['reasons']['READ_FAULT'], 1)
    def test_run_id_mix_rejected(self):
        row = self.sample(); row['run_id'] = 'old_run'
        with self.assertRaisesRegex(ValueError, 'run_id'):
            self.run_analysis([row])
        with self.assertRaisesRegex(ValueError, 'run_id'):
            self.run_analysis(session=dict(run_id='old_run'))
    def test_snapshot_threshold_is_not_actual_branch(self):
        self.summary['probes'][0].update(calls=3, sample_attempts=3, compared=3, matches=3)
        rows = []
        for n, lhs in enumerate([119999, 120000, 120001], 1):
            row = self.sample(seq=n)
            row['metrics'] = [dict(name='range', lhs=str(lhs), rhs='120000', relation='<=', predicate=lhs<=120000, observed_branch=False)]
            rows.append(row)
        out = self.run_analysis(rows)
        cov = out['probes'][0]['threshold_coverage']['range']
        self.assertEqual([cov[k] for k in ('below', 'equal', 'above')], [1, 1, 1])
        self.assertEqual(cov['snapshot_predicate'], 3)
        self.assertEqual(cov['observed_branch'], 0)
        self.assertEqual(cov['missing'], [])
        self.assertIn('전체 분기', logs.markdown(out))
    def test_io_loss_blocks_complete_result(self):
        self.summary['probes'][0].update(calls=1, sample_attempts=1, compared=1, matches=1, dropped=1)
        self.assertEqual(self.run_analysis()['status'], 'INCOMPLETE_WITH_LOG_LOSS')
    def test_masked_al_match_not_full_rax_match(self):
        self.summary['probes'][0].update(calls=1, sample_attempts=1, compared=1, matches=1)
        row = self.sample(); row['actual'] = '0xabcdef01'
        out = self.run_analysis([row])
        self.assertEqual(out['probes'][0]['emitted_counts']['MATCH'], 1)
        row['mask'] = '0xffffffffffffffff'
        out = self.run_analysis([row])
        self.assertTrue(out['log_loss_or_corruption'])
        self.assertEqual(out['probes'][0]['emitted_counts']['MATCH'], 0)
    def test_counter_invariant_rejected(self):
        self.summary['probes'][0]['compared'] = 1
        with self.assertRaisesRegex(ValueError, 'compared'):
            self.run_analysis()
    def test_output_difference_can_disagree_with_equal_return(self):
        self.summary['probes'][0].update(calls=1, sample_attempts=1, compared=1, mismatches=1)
        row = self.sample('MISMATCH')
        row['output_fields'] = [dict(address='0x10000', width=1, expected='0x0', actual='0x1')]
        out = self.run_analysis([row])
        self.assertEqual(out['status'], 'VANILLA_MISMATCH_OBSERVED')
        self.assertEqual(out['probes'][0]['emitted_counts']['MISMATCH'], 1)
        self.assertEqual(out['probes'][0]['output_field_differences'], 1)
    def test_summary_only_is_not_emitted_evidence(self):
        self.summary['probes'][0].update(calls=1, sample_attempts=1, compared=1, matches=1)
        self.assertEqual(self.run_analysis()['status'], 'SUMMARY_COMPARISONS_WITHOUT_EMITTED_VANILLA_EVIDENCE')
    def test_unstable_metric_does_not_fill_stable_boundary(self):
        self.summary['probes'][0].update(calls=1, sample_attempts=1, unstable=1)
        row = self.sample('UNSTABLE')
        row['metrics'] = [dict(name='range', lhs='120000', rhs='120000', relation='<=', predicate=True, observed_branch=False)]
        p = self.run_analysis([row])['probes'][0]
        self.assertEqual(p['threshold_coverage'], {})
        self.assertEqual(p['uncompared_metric_observations'], 1)
    def test_foreign_boundary_does_not_fill_vanilla_boundary(self):
        self.summary['probes'][0].update(calls=1, sample_attempts=1, foreign_matches=1)
        row = self.sample('FOREIGN_MATCH', domain='foreign_chain')
        row['metrics'] = [dict(name='range', lhs='120000', rhs='120000', relation='<=', predicate=True, observed_branch=False)]
        c = self.run_analysis([row])['probes'][0]['threshold_coverage']['range']
        self.assertEqual(c['vanilla_missing'], ['below', 'equal', 'above'])
        self.assertEqual(c['foreign_chain_missing'], ['below', 'above'])
    def test_duplicate_sequence_is_corruption(self):
        row = self.sample()
        out = self.run_analysis([row, row])
        self.assertEqual(out['probes'][0]['emitted_counts']['MATCH'], 1)
        self.assertEqual(len(out['invalid_lines']), 1)
    def test_processing_error_and_null_skip_are_not_comparisons(self):
        self.summary['probes'][0].update(calls=1, sample_attempts=1, skipped=1, processing_errors=1)
        row = self.sample('SKIP'); row.update(expected=None, mask=None, reason='OUTPUT_READ_ERROR')
        out = self.run_analysis([row])
        self.assertEqual(out['status'], 'INCOMPLETE_WITH_PROCESSING_ERRORS')
        self.assertEqual(out['summary_counts']['compared'], 0)
        self.assertEqual(out['invalid_lines'], [])
        self.assertTrue(out['evidence_incomplete'])
    def test_uncertain_io_upper_bound_and_unlogged_are_not_added_cases(self):
        self.summary.update(io_lost_or_uncertain_events=7)
        self.summary['probes'][0].update(calls=1, sample_attempts=1, skipped=1, dropped=1, unlogged_errors=1)
        out = self.run_analysis()
        self.assertEqual(out['status'], 'INCOMPLETE_WITH_LOG_LOSS')
        self.assertEqual(out['io_lost_or_uncertain_events_upper_bound'], 7)
        self.assertEqual(out['summary_counts']['calls'], 1)
        self.assertEqual(out['summary_counts']['unlogged_errors'], 1)
    def test_counter_exceeds_logged_evidence_is_incomplete(self):
        self.summary['probes'][0].update(calls=10, sample_attempts=2, compared=2, matches=2)
        out = self.run_analysis([self.sample()])
        self.assertEqual(out['status'], 'INCOMPLETE_WITH_COUNTER_LOG_GAPS')
        self.assertFalse(out['log_loss_or_corruption'])
        self.assertTrue(out['coverage_incomplete'])
        self.assertEqual(out['probes'][0]['counter_log_deficits']['matches'], 1)

class EntryTest(unittest.TestCase):
    def prologue(self, data):
        md = capstone.Cs(capstone.CS_ARCH_X86, capstone.CS_MODE_64); md.detail=True
        return catalog.safe_prologue(list(md.disasm(bytes.fromhex(data), 0x1000)))
    def test_complete_instruction_boundary(self):
        pro, _ = self.prologue('56574883ec28488b01488b01488b01')
        self.assertEqual(len(pro), 12)
    def test_branch_before_twelve_rejected(self):
        self.assertEqual(self.prologue('4883ec284885d274714989d0')[0], b'')
    def test_rip_relative_rejected(self):
        self.assertEqual(self.prologue('488b0511223344909090909090')[0], b'')
    def test_call_rejected(self):
        self.assertEqual(self.prologue('e8000000009090909090909090')[0], b'')

if __name__ == '__main__':
    unittest.main()
