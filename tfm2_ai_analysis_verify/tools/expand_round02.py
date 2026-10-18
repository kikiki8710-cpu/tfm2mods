"""동결 catalog와 독립 감사된 Jcc 제안을 사용해 round02 진입 목록을 확장한다."""
from pathlib import Path
import hashlib,json

ROOT=Path(__file__).resolve().parents[1]
REPORT=Path('C:/Users/jungs/Desktop/claude/tfm2/mods_report/tfm2_ai_analysis_verify/runtime_20261018_round02')
catalog=json.loads((ROOT/'catalog.json').read_text(encoding='utf-8'))
rows={int(x['rva'],16):x for x in catalog['functions']}
proposal=json.loads((REPORT/'model_expansion/entry_proposals.json').read_text(encoding='utf-8'))
original=(REPORT/'source_before/generated.rs').read_text(encoding='utf-8')
lines=[line for line in original.splitlines() if 'ProbeDef { id:' in line]
defs={int(line.split('id: ')[1].split(',')[0]):line.replace(' },',', branches:&[] },') for line in lines}
for p in proposal['probes']:
    rva=int(p['rva'],16);f=rows[rva];pid=p['model_id']
    path=ROOT/'tools/data'/f'probe_{rva:x}.bin'
    body=path.read_bytes()
    assert hashlib.sha256(body).hexdigest()==p['body_sha256']==f['body_sha256']
    assert body.startswith(bytes.fromhex(p['prologue_hex']))
    branches=','.join('crate::hook::BranchFixup{offset:%d,len:%d,target_rva:%s}'%(b['offset'],b['length'],b['target_rva']) for b in p['jcc_fixups'])
    defs[pid]=' ProbeDef { id: %d, rva: 0x%x, semantic_id: %s, name: %s, body: include_bytes!("../tools/data/probe_%x.bin"), prologue: &[%s], source: %s, branches:&[%s] },'%(pid,rva,json.dumps(f['semantic_id'],ensure_ascii=False),json.dumps(f['name'],ensure_ascii=False),rva,','.join('0x%02x'%b for b in bytes.fromhex(p['prologue_hex'])),json.dumps(f['source'],ensure_ascii=False),branches)
prefix=original.split('pub static PROBES:')[0].replace("pub source: &'static str }","pub source: &'static str, pub branches: &'static [crate::hook::BranchFixup] }")
(ROOT/'src/generated.rs').write_text(prefix+'pub static PROBES: &[ProbeDef] = &[\n'+'\n'.join(defs[k] for k in sorted(defs))+'\n];\n',encoding='utf-8')
print('전체 본문 SHA 고정 진입정의',len(defs))
