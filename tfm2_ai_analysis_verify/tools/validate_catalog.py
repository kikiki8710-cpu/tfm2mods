"""생성 후 source/body/generated SHA와 준비상태 분해를 별도로 검사한다."""
from pathlib import Path
import hashlib, json
from generate_catalog import MOD, APPROVED, BASELINE, sha, readj

def validate():
    catalog = readj(MOD/'catalog.json')
    evidence = readj(MOD/'evidence_manifest.json')
    assert catalog['exe_sha256'] == evidence['exe_sha256'] == BASELINE
    rows = catalog['functions']
    assert len(rows) == 741 and len({r['function_key'] for r in rows}) == 741
    assert len({r['rva'] for r in rows}) == 741
    assert sum(r['bytes'] for r in rows) == 1609795
    assert sum(r['instructions'] for r in rows) == 355436
    audited = 0
    for key in ('source_files', 'body_files', 'generated_files'):
        for path, expected in evidence[key].items():
            assert sha(Path(path).read_bytes()) == expected, (key, path)
            audited += 1
    assert evidence['generator_sha256'] == sha((MOD/'tools/generate_catalog.py').read_bytes())
    counts = catalog['counts']
    assert counts['spec_only'] + counts['unsupported'] + counts['paired_wrapper_prepared'] == 741
    prepared, entry, implemented = [], [], []
    for r in rows:
        a = int(r['rva'], 16)
        assert not r['runtime_verified'] and not r['actual_provider_verified']
        assert r['runtime_samples'] == 0 and r['runtime_state'] == 'runtime_no_samples'
        assert not r['apply_allowed'] and not r['whole_transitive_verified']
        if r['model_implemented']:
            implemented.append(a)
            assert a in APPROVED
        if r['entry_ready']:
            entry.append(a)
            assert r['abi_approved'] and r['entry_rax_input_live'] is False
            pro = bytes.fromhex(r['mechanical_entry_prologue'])
            assert 12 <= len(pro) <= 32
            assert (MOD/r['body_file']).read_bytes().startswith(pro)
        if r['paired_wrapper_prepared']:
            prepared.append(a)
            assert r['entry_ready'] and r['model_implemented']
    assert len(prepared) == counts['paired_wrapper_prepared']
    assert len(entry) == counts['entry_ready'] and len(implemented) == counts['model_implemented']
    out = dict(status='PASS_STATIC_CATALOG_AUDIT_NOT_RUNTIME', census_functions=741,
        bytes=1609795, instructions=355436, audited_sha_entries=audited, counts=counts,
        entry_ready_rvas=list(map(hex, entry)), independent_game_samples=0,
        new_analysis_functions=0, source_analysis_modified=False,
        catalog_sha256=sha((MOD/'catalog.json').read_bytes()), evidence_manifest_sha256=sha((MOD/'evidence_manifest.json').read_bytes()))
    return out

if __name__ == '__main__':
    result = validate()
    (MOD/'tools/catalog_validation.json').write_text(json.dumps(result, ensure_ascii=False, indent=2)+'\n', encoding='utf8')
    print(json.dumps(result, ensure_ascii=False))
