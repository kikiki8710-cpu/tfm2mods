"""새 준비본·회차 영수증만 생성한다. 과거 prepared/REPORT와 게임 폴더는 수정하지 않는다."""
import argparse,hashlib,json,re,shutil
from pathlib import Path
p=argparse.ArgumentParser();p.add_argument('--report',required=True);a=p.parse_args()
ROOT=Path(__file__).resolve().parents[1]
REPORT=Path(a.report).resolve();REPORT.mkdir(parents=True,exist_ok=True)
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
version=json.loads((ROOT/'mod.mod_info').read_text(encoding='utf-8'))['version']
assert version=='0.2.4'
dest=ROOT/('prepared_auto24_v'+version.replace('.',''))/'tfm2_ai_analysis_verify'
assert not dest.exists(),'준비본 덮어쓰기 금지'
assert not (REPORT/'stage_receipt_v024.json').exists(),'영수증 덮어쓰기 금지'
cfg=(ROOT/'verify.cfg').read_text(encoding='utf-8');assert '\narmed=false\n' in '\n'+cfg
text=(ROOT/'src/generated.rs').read_text(encoding='utf-8')
ids=[(int(x),int(y,16)) for x,y in re.findall(r'ProbeDef \{ id: (\d+), rva: (0x[0-9a-f]+)',text)]
assert len(ids)==37 and {x for x,_ in ids}==set(range(37)) and len({y for _,y in ids})==37
catalog=json.loads((ROOT/'catalog.json').read_text(encoding='utf-8'));rows={int(f['rva'],16):f for f in catalog['functions']}
for _,rva in ids:
    assert sha(ROOT/'tools/data'/('probe_%x.bin'%rva))==rows[rva]['body_sha256']
dest.mkdir(parents=True)
for name in ['mod.mod_info','verify.cfg','README.md']:
    shutil.copy2(ROOT/name,dest/name)
shutil.copy2(ROOT/'target/release/tfm2_ai_analysis_verify.dll',dest/'tfm2_ai_analysis_verify.dll')
scope=dict(version='0.6.4',mod_version=version,census_total=741,selected_models=37,runtime_verified=False,
           scope='기존741카탈로그동결보존. 이번37모델준비이며실게임관측과별도.',
           functions=[dict(id=i,rva=hex(rva),semantic_id=rows[rva]['semantic_id'],body_sha256=rows[rva]['body_sha256'],
                 bytes=rows[rva]['bytes'],model_implemented=True,entry_prepared=True,runtime_samples=0,runtime_verified=False) for i,rva in ids])
(dest/'wave05_scope.json').write_text(json.dumps(scope,ensure_ascii=False,indent=2),encoding='utf-8')
receipt=dict(version=version,models=37,armed=False,game_launches=0,
 files=[dict(path=x.name,size=x.stat().st_size,mtime_ns=x.stat().st_mtime_ns,sha256=sha(x)) for x in dest.iterdir() if x.is_file()],
 source_files=[dict(path=str(x.relative_to(ROOT)),sha256=sha(x)) for x in (ROOT/'src').glob('*.rs')],
 tool_files=[dict(path=str(x.relative_to(ROOT)),sha256=sha(x)) for x in (ROOT/'tools').glob('*wave05*.py')])
(REPORT/'stage_receipt_v024.json').write_text(json.dumps(receipt,ensure_ascii=False,indent=2),encoding='utf-8')
print(json.dumps(dict(stage=str(dest),dll_sha256=sha(dest/'tfm2_ai_analysis_verify.dll'),models=37)))
