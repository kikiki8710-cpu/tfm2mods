"""0.6.4 census741과현행proof를검사해관측용카탈로그를생성한다. 게임/기존분석수정없음."""
from __future__ import annotations
import argparse,bisect,hashlib,json,struct
from pathlib import Path
import capstone

BASELINE='a1d05045405a12e2bed182daeafcad8c8a40cf29ab68ba6ea5bfd30257439582'
APPROVED=[0xdd87a0,0xe52ee0,0xd98bd0,0xe49c00,0xfa9dc0,0xf51e50,0xe9eb70,0xfdb9a0]
MOD=Path(__file__).resolve().parents[1]
DEFAULT_WORKSPACE=Path('C:/Users/jungs/Desktop/claude/tfm2')
def sha(data):return hashlib.sha256(data).hexdigest()
def readj(p):return json.loads(p.read_text(encoding='utf-8'))
def dump(p,x):p.parent.mkdir(parents=True,exist_ok=True);p.write_text(json.dumps(x,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')

class PE:
 def __init__(self,path):
  self.path=Path(path);self.data=self.path.read_bytes();assert sha(self.data)==BASELINE,'예상exe SHA불일치'
  nt=struct.unpack_from('<I',self.data,0x3c)[0];assert self.data[nt:nt+4]==b'PE\0\0'
  n=struct.unpack_from('<H',self.data,nt+6)[0];size=struct.unpack_from('<H',self.data,nt+20)[0];op=nt+24;assert struct.unpack_from('<H',self.data,op)[0]==0x20b
  self.image_base=struct.unpack_from('<Q',self.data,op+24)[0];assert self.image_base==0x140000000
  self.sections=[]
  for k in range(n):
   p=op+size+k*40;name=self.data[p:p+8].rstrip(b'\0').decode('ascii');vs,va,rs,rp=struct.unpack_from('<IIII',self.data,p+8);flags=struct.unpack_from('<I',self.data,p+36)[0]
   self.sections.append(dict(name=name,virtual_address=va,virtual_size=vs,raw_size=rs,raw_pointer=rp,characteristics=flags))
  pd,sz=struct.unpack_from('<II',self.data,op+112+3*8);raw,_=self.loaded(pd,sz);self.pdata=[struct.unpack_from('<III',raw,k)[:2]for k in range(0,sz-11,12)];self.pdata.sort();self.starts=[x[0]for x in self.pdata]
 def owner(self,rva):
  k=bisect.bisect_right(self.starts,rva)-1
  if k>=0 and self.pdata[k][0]<=rva<self.pdata[k][1]:return self.pdata[k]
  return None
 def loaded(self,rva,size):
  assert size>=0
  sec=next((s for s in self.sections if s['virtual_address']<=rva and rva+size<=s['virtual_address']+max(s['virtual_size'],s['raw_size'])),None)
  assert sec is not None,('섹션범위밖',hex(rva),size)
  off=rva-sec['virtual_address'];n=max(0,min(size,sec['raw_size']-off));start=sec['raw_pointer']+off
  return self.data[start:start+n]+b'\0'*(size-n),{'section':sec['name'],'storage':'FILE'if n==size else 'FILE_AND_BSS_ZERO'if n else'BSS_ZERO','file_bytes':n,'zero_bytes':size-n}

def safe_prologue(seq):
 """기계적진입적격성만판정한다. RAX/liveABI승인은별도로요구한다."""
 out=b''
 for i in seq:
  if i.group(capstone.CS_GRP_JUMP)or i.group(capstone.CS_GRP_CALL)or i.group(capstone.CS_GRP_RET)or i.group(capstone.CS_GRP_INT)or i.mnemonic in ['syscall','ud2','int3']:
   return b'',f'12B전제어이전:{hex(i.address)} {i.mnemonic}'
  if any(o.type==capstone.x86.X86_OP_MEM and o.mem.base==capstone.x86.X86_REG_RIP for o in i.operands):return b'',f'12B전RIP상대:{hex(i.address)}'
  if len(out)+i.size>32:return b'','완전명령프롤로그32B초과'
  out+=i.bytes
  if len(out)>=12:return out,'MECHANICAL_PROLOGUE_OK_ABI_APPROVAL_SEPARATE'
 return b'','본문12B미만'

def find_scope_rows(x):
 """명시적scope안에서rva행만찾는다. 구현/승인false를추정으로뒤집지않음."""
 out={}
 def walk(v,key=None):
  if isinstance(v,dict):
   a=v.get('rva',v.get('current_rva',key))
   try:a=int(a,0)if isinstance(a,str)else int(a)
   except(TypeError,ValueError):a=None
   if a in APPROVED and any(k in v for k in ['model_implemented','entry_rax_input_live','paired_wrapper_prepared','entry_rax_live_in','rax_live_in','scope_state','rax_input_absent']):out[a]=v
   for k,y in v.items():walk(y,k)
  elif isinstance(v,list):
   for y in v:walk(y)
 walk(x);return out

def body_hashes(x):
 out=[]
 def walk(v):
  if isinstance(v,dict):
   for k,y in v.items():
    if k in ['body_sha256','current_body_sha256','new_body_sha256']and isinstance(y,str):out.append(y)
    elif isinstance(y,(dict,list)):walk(y)
  elif isinstance(v,list):
   for y in v:walk(y)
 walk(x);return out

def rust_string(s):return json.dumps(str(s),ensure_ascii=False)

def preparation_metadata(prep):
 """A의 구현완료 영수증은 구현만 승인한다. 빌드/paired 준비를 추론하지 않는다."""
 if 'implemented_model_ids' not in prep:return prep
 assert list(map(lambda x:int(x,16),prep['id_rva_order']))==APPROVED
 out=dict(prep)
 out.update(schema_version=1,baseline_sha256=BASELINE,
  source_files={prep['model_file']:prep['model_file_sha256'],prep['source_scope_file']:prep['source_scope_sha256']},
  probes=[dict(rva=hex(APPROVED[k]),model_implemented=True,paired_wrapper_prepared=False)for k in prep['implemented_model_ids']])
 return out

def generate(workspace,installed,scope_path=None,preparation_path=None):
 report=workspace/'mods_report/tfm2_ai_adjust';parallel=report/'analysis_064/parallel';inventory_path=parallel/'inventory.json';inherit_path=parallel/'inherited_revalidation_064.json'
 inv=readj(inventory_path);inh=readj(inherit_path);assert inv['exe_sha256']==inh['exe_sha256']==BASELINE
 assert len(inv['functions'])==741 and inh['functions_count']==58
 fixed=PE(workspace/'tfm2_0.6.4/TeamfightManager2.exe');actual=Path(installed);assert sha(actual.read_bytes())==BASELINE,'현행설치본문불일치'
 md=capstone.Cs(capstone.CS_ARCH_X86,capstone.CS_MODE_64);md.detail=True
 scope=readj(scope_path)if scope_path else{};approvals=find_scope_rows(scope);inherited={int(x['rva'],16):x for x in inh['functions']};recheck=parallel/Path(inh['source']).parent
 inputs={str(p.resolve()):sha(p.read_bytes())for p in [inventory_path,inherit_path,fixed.path,actual]}
 if scope_path:inputs[str(scope_path.resolve())]=sha(scope_path.read_bytes())
 if preparation_path:
  prep=preparation_metadata(readj(preparation_path));assert prep['schema_version']==1 and prep['baseline_sha256']==BASELINE
  inputs[str(preparation_path.resolve())]=sha(preparation_path.read_bytes())
  for path,h in prep.get('source_files',{}).items():
   q=Path(path);assert sha(q.read_bytes())==h,('모델준비sourceSHA불일치',path);inputs[str(q.resolve())]=h
  assert prep.get('source_files'),'모델준비source SHA필수'
  for a,ap in find_scope_rows(prep).items():
   if ap.get('paired_wrapper_prepared')is True:assert prep.get('tests_passed')is True,'paired준비에는실행된시험PASS필수'
   approvals[a]={**approvals.get(a,{}),**ap}
 rows=[];probes={};data=MOD/'tools/data';data.mkdir(parents=True,exist_ok=True)
 for f in inv['functions']:
  a=int(f['rva'],16);end=int(f['end'],16);body,storage=fixed.loaded(a,end-a);assert len(body)==f['bytes']and sha(body)==f['body_sha256'];assert fixed.owner(a)==(a,end),('PDATA불일치',hex(a),fixed.owner(a),hex(end));assert storage['storage']=='FILE'
  seq=list(md.disasm(body,a));assert len(seq)==f['instructions']and sum(i.size for i in seq)==len(body)
  if a in inherited:
   inherit=inherited[a];proof=recheck/inherit['review'];assert sha(proof.read_bytes())==inherit['review_sha256'];kind='INHERITED_CURRENT_VERSION_REVALIDATED'
  else:
   record=f.get('current_batch_review');assert record and record.get('proof'),('현행계약proof없음',hex(a));proof=report/record['proof'];kind='CURRENT_LOCAL_CONTRACT'
  assert proof.exists(),str(proof);pj=readj(proof);inputs[str(proof.resolve())]=sha(proof.read_bytes());ph=body_hashes(pj)
  evidence_files={}
  for name in ['body.bin','capture.json','instruction_ir.json','raw_numeric_inventory.json','field_consumers.json','calls.json','branches.json','parameters.json']:
   q=proof.parent/name
   if q.exists():
    h=sha(q.read_bytes());inputs[str(q.resolve())]=h;evidence_files[name]={'path':str(q.resolve()),'sha256':h}
  records=pj.get('parameters',pj.get('numeric_meanings',[]))
  if ph:assert f['body_sha256']in ph,('proof의본문SHA불일치',hex(a),str(proof))
  sid=pj.get('semantic_id')or (f.get('current_batch_review') or {}).get('semantic_id')or f.get('semantic_anchor_candidate')or f['function_key'];name=pj.get('name')or pj.get('title')or sid
  pro,reason=safe_prologue(seq);ap=approvals.get(a,{})
  live=False if ap.get('rax_input_absent') is True else ap.get('entry_rax_input_live',ap.get('entry_rax_live_in',ap.get('rax_live_in')))
  abi_approved=live is False and ap.get('abi_approved',True)is True
  implemented=ap.get('model_implemented')is True;wrapper=ap.get('paired_wrapper_prepared')is True
  entry_ready=a in APPROVED and bool(pro)and abi_approved
  ready=entry_ready and implemented and wrapper
  out=data/f'probe_{a:x}.bin';out.write_bytes(body)
  row=dict(function_key=f['function_key'],catalog_id=len(rows),rva=hex(a),end_rva=hex(end),bytes=len(body),instructions=len(seq),semantic_id=str(sid),name=str(name),source=f['primary_source'],group=f['group'],body_sha256=sha(body),body_file=str(out.relative_to(MOD)),proof_kind=kind,proof_path=str(proof.resolve()),proof_sha256=sha(proof.read_bytes()),proof_body_sha256_explicit=bool(ph),source_evidence_files=evidence_files,source_consumer_records=records,source_consumer_scope='원문 계약의 분류를 보존. 전량이 독립 조절 가능한 임계값이라는 뜻이 아님',section=storage,approved_probe_id=APPROVED.index(a)if a in APPROVED else None,mechanical_entry_prologue=pro.hex(),mechanical_entry_reason=reason,entry_rax_input_live=live,abi_approved=abi_approved,entry_ready=entry_ready,model_implemented=implemented,paired_wrapper_prepared=ready,runtime_state='runtime_no_samples',runtime_samples=0,runtime_verified=False,thresholds=ap.get('thresholds',[]),output_comparison_mask=ap.get('rax_compare_mask'),adapter_abi=ap.get('abi'),scope='paired_wrapper_prepared'if ready else'unsupported'if a==0xe60c20 else'spec_only',unsupported_reason_codes=['STACK_ARGUMENT_5_REQUIRED']if a==0xe60c20 else[],original_unknowns=pj.get('unknowns',pj.get('remaining',pj.get('original_limitations',[]))),actual_provider_verified=False,whole_transitive_verified=False,apply_allowed=False)
  rows.append(row)
  if entry_ready:probes[a]=row
 assert len(rows)==741 and sum(x['bytes']for x in rows)==1609795
 # PROBES는 승인8개 중 기계진입/ABI 조건을 모두 통과한 항목만 공개한다.
 text=['// 자동 생성: 현행 PE/proof 고정 근거. 직접 편집하지 않는다.','pub const BASELINE_EXE_SHA256: &str = '+rust_string(BASELINE)+';','pub const CENSUS_FUNCTIONS: usize = 741;','pub struct ProbeDef { pub id: usize, pub rva: usize, pub semantic_id: &\'static str, pub name: &\'static str, pub body: &\'static [u8], pub prologue: &\'static [u8], pub source: &\'static str }','pub static PROBES: &[ProbeDef] = &[']
 for a in APPROVED:
  if a not in probes:continue
  q=probes[a];pro=bytes.fromhex(q['mechanical_entry_prologue']);text.append(' ProbeDef { id: '+str(q['approved_probe_id'])+', rva: '+hex(a)+', semantic_id: '+rust_string(q['semantic_id'])+', name: '+rust_string(q['name'])+', body: include_bytes!("../tools/data/probe_'+f'{a:x}'+'.bin"), prologue: &['+','.join(hex(b)for b in pro)+'], source: '+rust_string(q['source'])+' },')
 text.append('];');(MOD/'src/generated.rs').write_text('\n'.join(text)+'\n',encoding='utf-8')
 catalog=dict(status='GENERATED_STATIC_CATALOG_NOT_RUNTIME_VALIDATION',version='0.6.4',exe_sha256=BASELINE,census_functions=741,census_scope=inv['scope'],approved_execution_rvas=list(map(hex,APPROVED)),runtime_samples=0,model_scope_source=str(scope_path)if scope_path else None,model_preparation_source=str(preparation_path)if preparation_path else None,counts=dict(unsupported=sum(x['scope']=='unsupported'for x in rows),spec_only=sum(x['scope']=='spec_only'for x in rows),paired_wrapper_prepared=sum(x['paired_wrapper_prepared']for x in rows),entry_ready=sum(x['entry_ready']for x in rows),model_implemented=sum(x['model_implemented']for x in rows)),functions=rows,limits=['741은기존census이며전체AI완전집합/741runtime모델완료를의미하지않음','entry적격/모델구현/paired준비/실측0은서로다른상태'])
 dump(MOD/'catalog.json',catalog)
 evidence=dict(status='FIXED_AND_INSTALLED_PE_PDATA_BODY_PROOF_SHA_MATCH',exe_sha256=BASELINE,source_files=inputs,sections=fixed.sections,body_files={str((MOD/x['body_file']).resolve()):x['body_sha256']for x in rows},generated_files={str((MOD/n).resolve()):sha((MOD/n).read_bytes())for n in ['catalog.json','src/generated.rs']},generator_sha256=sha(Path(__file__).read_bytes()),new_runtime_cases=0,prior_sources_modified=False)
 dump(MOD/'evidence_manifest.json',evidence);return catalog

def main():
 p=argparse.ArgumentParser();p.add_argument('--workspace',type=Path,default=DEFAULT_WORKSPACE);p.add_argument('--installed-exe',type=Path,default=Path('C:/Program Files (x86)/Steam/steamapps/common/Teamfight Manager2/TeamfightManager2.exe'));p.add_argument('--model-scope',type=Path);p.add_argument('--model-preparation',type=Path);args=p.parse_args();out=generate(args.workspace,args.installed_exe,args.model_scope,args.model_preparation);print(json.dumps(out['counts'],ensure_ascii=False))
if __name__=='__main__':main()
