"""시험용 개정본을 별도 준비 폴더에 고정한다. 게임 복사 없이 옛 준비본을 보존한다."""
from pathlib import Path
import hashlib,json,re,shutil
ROOT=Path(__file__).resolve().parents[1]
VERSION=json.loads((ROOT/'mod.mod_info').read_text(encoding='utf-8'))['version']
DEST=ROOT/('prepared_round02_v'+VERSION.replace('.',''))/'tfm2_ai_analysis_verify'
REPORT=Path('C:/Users/jungs/Desktop/claude/tfm2/mods_report/tfm2_ai_analysis_verify/runtime_20261018_round02')
def sha(p): return hashlib.sha256(p.read_bytes()).hexdigest()
DEST.mkdir(parents=True,exist_ok=True)
for name in ['mod.mod_info','verify.cfg','README.md']:
    shutil.copy2(ROOT/name,DEST/name)
shutil.copy2(ROOT/'target/release/tfm2_ai_analysis_verify.dll',DEST/'tfm2_ai_analysis_verify.dll')
text=(ROOT/'src/generated.rs').read_text(encoding='utf-8')
ids=[(int(a),int(b,16)) for a,b in re.findall(r'ProbeDef \{ id: (\d+), rva: (0x[0-9a-f]+)',text)]
catalog=json.loads((ROOT/'catalog.json').read_text(encoding='utf-8'))
rows={int(f['rva'],16):f for f in catalog['functions']}
overlay={'version':'0.6.4','mod_version':json.loads((ROOT/'mod.mod_info').read_text(encoding='utf-8'))['version'],'census_total':741,'selected_models':len(ids),
 'runtime_verified':False,'scope':'기존 catalog는 첫 준비시점 동결. 이 파일은 round02 배선 상태만 덮어쓴다.',
 'functions':[{'id':pid,'rva':hex(rva),'semantic_id':rows[rva]['semantic_id'],
 'body_sha256':rows[rva]['body_sha256'],'bytes':rows[rva]['bytes'],'model_implemented':True,
 'entry_prepared':True,'runtime_samples':0,'runtime_verified':False} for pid,rva in ids]}
(DEST/'round02_scope.json').write_text(json.dumps(overlay,ensure_ascii=False,indent=2),encoding='utf-8')
receipt={'files':[{'path':p.name,'size':p.stat().st_size,'sha256':sha(p)} for p in DEST.iterdir() if p.is_file()],
 'source_files':[{'path':str(p.relative_to(ROOT)),'sha256':sha(p)} for p in (ROOT/'src').glob('*.rs')],
 'armed':False,'models':len(ids),'game_launches':0}
(REPORT/('stage_receipt_v'+VERSION.replace('.','')+'.json')).write_text(json.dumps(receipt,ensure_ascii=False,indent=2),encoding='utf-8')
print(json.dumps({'stage':str(DEST),'dll_sha256':sha(DEST/'tfm2_ai_analysis_verify.dll'),'models':len(ids)},ensure_ascii=False))
