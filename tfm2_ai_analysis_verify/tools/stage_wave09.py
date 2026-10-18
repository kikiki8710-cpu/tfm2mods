"""준비 모델 수와 실제 설치 요청 수를 구분한 새 회차 준비본만 만든다."""
import argparse,hashlib,json,re,shutil
from pathlib import Path
p=argparse.ArgumentParser();p.add_argument('--report',required=True);a=p.parse_args()
M=Path(__file__).resolve().parents[1];R=Path(a.report).resolve();version=json.loads((M/'mod.mod_info').read_text(encoding='utf-8'))['version'];assert version=='0.2.8'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
D=M/'prepared_auto24_v028/tfm2_ai_analysis_verify';assert not D.exists() and not (R/'stage_receipt_v028.json').exists()
cfg=(M/'verify.cfg').read_text(encoding='utf-8');assert '\narmed=false\n' in '\n'+cfg
selected={int(x,16) for x in re.search(r'^probes=(.*)$',cfg,re.M).group(1).split(',')};assert len(selected)==13
defs=[(int(x),int(y,16)) for x,y in re.findall(r'ProbeDef \{ id: (\d+), rva: (0x[0-9a-f]+)',(M/'src/generated.rs').read_text(encoding='utf-8'))]
assert len(defs)==47 and {i for i,_ in defs}==set(range(47)) and len({r for _,r in defs})==47
assert selected<={r for _,r in defs};assert not selected&{0xe1b5b0,0x1016b00,0xe53290}
catalog={int(f['rva'],16):f for f in json.loads((M/'catalog.json').read_text(encoding='utf-8'))['functions']}
for _,rva in defs:assert sha(M/'tools/data'/('probe_%x.bin'%rva))==catalog[rva]['body_sha256']
D.mkdir(parents=True)
for name in ['mod.mod_info','verify.cfg','README.md']:shutil.copy2(M/name,D/name)
shutil.copy2(M/'target/release/tfm2_ai_analysis_verify.dll',D/'tfm2_ai_analysis_verify.dll')
scope=dict(game_version='0.6.4',mod_version=version,census_total=741,prepared_models=47,selected_models=13,runtime_verified=False,scope='기존741고정추적집합. 준비47과이번13설치요청은별개이며실제설치/실측은아직0.',functions=[dict(id=i,rva=hex(r),semantic_id=catalog[r]['semantic_id'],body_sha256=catalog[r]['body_sha256'],bytes=catalog[r]['bytes'],model_implemented=True,entry_prepared=True,install_requested=r in selected,runtime_samples=0,runtime_verified=False) for i,r in defs],limits=['기존44보존/새5·5·6인수3모델. actual provider·원본EH·전역LTO미검증','미선택34개는이번실게임실패나설치거절로세지않음'])
(D/'wave09_scope.json').write_text(json.dumps(scope,ensure_ascii=False,indent=2),encoding='utf-8')
receipt=dict(version=version,models=47,prepared_models=47,selected_models=13,armed=False,game_launches=0,files=[dict(path=x.name,size=x.stat().st_size,mtime_ns=x.stat().st_mtime_ns,sha256=sha(x)) for x in D.iterdir() if x.is_file()],source_files=[dict(path=str(x.relative_to(M)),sha256=sha(x)) for x in (M/'src').glob('*.rs')],tool_files=[dict(path=str(x.relative_to(M)),sha256=sha(x)) for x in (M/'tools').glob('*wave09*.py')])
(R/'stage_receipt_v028.json').write_text(json.dumps(receipt,ensure_ascii=False,indent=2),encoding='utf-8');print(json.dumps(dict(stage=str(D),dll_sha256=sha(D/'tfm2_ai_analysis_verify.dll'),prepared=47,selected=13)))
