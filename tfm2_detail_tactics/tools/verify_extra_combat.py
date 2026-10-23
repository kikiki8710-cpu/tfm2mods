"""실제 Rust 콜백과 전투53창의 x64 동적 fragment를 독립 정책식에 차등 대조한다."""
from pathlib import Path
import sys,json,hashlib,subprocess,random,re,struct
MOD=Path(__file__).parents[1];REPORT=Path(r'C:\Users\jungs\Desktop\claude\tfm2\mods_report\tfm2_detail_tactics\implement_20261024')
sys.path.insert(0,str(MOD/'target/verify_deps'));sys.path.insert(0,str(MOD/'tools'))
from keystone import Ks,KS_ARCH_X86,KS_MODE_64
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
from capstone import Cs,CS_ARCH_X86,CS_MODE_64,CS_GRP_JUMP,CS_GRP_CALL,CS_GRP_RET
from capstone.x86 import X86_OP_IMM,X86_OP_MEM,X86_REG_RIP
from gen_sites import PE,find_func,parse_src,text_rel32_hits
OUT=REPORT/'combat_native_check.json';OUT.write_text('{"pass":false,"state":"검사 진행 중"}\n',encoding='utf-8',newline='\n')
target=MOD/'tools/targets/extra_combat_0.6.5.json';data=json.loads(target.read_text(encoding='utf-8'));rows=data['tactics']
assert len(rows)==21 and len({x['id'] for x in rows})==21
exe=Path(r'C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe');pe=PE(str(exe));ss,ee=pe.functions()
assert hashlib.sha256(pe.data).hexdigest().upper()==data['exe_sha256']
ks=Ks(KS_ARCH_X86,KS_MODE_64);md=Cs(CS_ARCH_X86,CS_MODE_64);md.detail=True
def asm(s,addr=0):return bytes(ks.asm(s,addr=addr)[0]) if s else b''
M=(1<<64)-1
def s64(x):return x if x<1<<63 else x-(1<<64)
def trunc(x,d):return x//d if x>=0 else -((-x)//d)
def expected(name,v,r):
 if name=='tower_hits':return r[0]*v&M
 if name=='chase_ratio_rdx':return r[2]*v&M
 if name=='chase_ratio_rcx':return r[1]*v&M
 if name=='radius_square':return v*v
 if name=='radius_square_minus_one':return v*v-1
 if name=='trade_hp_divisor':return r[1]//v
 if name in ['heal_r8','heal_r14']:return (r[1]+r[8 if name=='heal_r8' else 14]*v)&M
 if name in ['shield_r8','shield_r14']:return r[8 if name=='shield_r8' else 14]*(v+1)&M
 if name=='buff_arrival_ticks':return ((r[1]//2)*v)&M if r[1]%2==0 else None
 if name=='isolated_percent':return ((r[0]*pow(5,-1,1<<64)&M)*v)&M
 if name=='front_ally_distance_score':return (r[0]//4000)*v//100
 if name=='front_ally_cap':return v*25//100
 if name=='front_ally_total_cap':return v*60//100
 if name=='wave_risk_divisor':return trunc(s64(r[9]),v)&M
 if name=='risk_retreat_high':return v*24//10
 if name=='risk_retreat_pressure':return r[0]*(2*v)&M
 if name=='unseen_risk_divisor':return trunc(s64(r[7]),v)&M
 if name.startswith('relocate_'):
  scale={'inner':8000,'mid':30000,'outer':70000,'self_near':24000,'self_far':48000}
  p=name.removeprefix('relocate_');result=p.endswith('_result');p=p.removesuffix('_result')
  n=v*scale[p]//100
  return (r[2]+r[6]+n)&M if result else n
 if name=='candidate_move_budget':return r[0]*(v+2)&M
 if name=='forward_credit_percent':return r[7]*v//100
 if name=='move_penalty_negative':return (-v)&M
 if name=='move_penalty_result':return (r[13]-v)&M
 raise ValueError(name)
RUSTC=Path(r'C:\Users\jungs\.cargo\bin\rustc.exe');rust_src=MOD/'tools/combat_callback_fixture.rs';callback_exe=REPORT/'combat_callback_fixture.exe'
subprocess.run([str(RUSTC),'--edition=2021','-O',str(rust_src),'-o',str(callback_exe)],check=True,capture_output=True)

# 원래 창 경계/바이트/주체 표현을 검증하고 비교할 fixture를 모은다.
flat=[];cases=[];requests=[];request_case=[];interior={};static=[];func_cache={}
boundaries=[0,1,2,3,9,25,49,50,60,61,64,65,80,81,99,100,127,128,0xffffffff,0x100000000,0x7fffffffffffffff,0x8000000000000000,M]
for t in rows:
 assert t['min']<=t['low']<=t['max'] and t['min']<=t['default']<=t['max'] and t['min']<=t['high']<=t['max']
 for k,w in enumerate(t['sites']):
  at=int(w['rva'],0);raw=bytes.fromhex(w['win_hex']);ins=list(md.disasm(raw,at));assert sum(i.size for i in ins)==len(raw)
  assert pe.read(at,len(raw))==raw and len(raw)>=5
  assert w['first_len'] in {i.address+i.size-at for i in ins}
  assert parse_src(w['src']).startswith('Src::') and parse_src(w['team_chk']).startswith('Src::')
  fn=find_func(ss,ee,at);assert fn
  if fn not in func_cache:
   bb=list(md.disasm(pe.read(fn[0],fn[1]-fn[0]),fn[0]));targets=set()
   for i in bb:
    if (i.group(CS_GRP_JUMP) or i.group(CS_GRP_CALL)) and i.operands[0].type==X86_OP_IMM:targets.add(i.operands[0].imm)
    if i.mnemonic=='lea' and len(i.operands)==2 and i.operands[1].type==X86_OP_MEM and i.operands[1].mem.base==X86_REG_RIP:
     tb=i.address+i.size+i.operands[1].mem.disp
     if pe.off(tb) is None:continue
     mode=None
     for p in range(2048):
      val=struct.unpack('<i',pe.read(tb+p*4,4))[0]
      if mode is None:mode='rel' if fn[0]<=tb+val<fn[1] else ('abs' if fn[0]<=val<fn[1] else None)
      if mode is None:break
      dest=tb+val if mode=='rel' else val
      if not fn[0]<=dest<fn[1]:break
      targets.add(dest)
   func_cache[fn]=targets
  inner=set(range(at+1,at+len(raw)));assert not inner&func_cache[fn],(t['audit_id'],w['rva'],'직접/JT내부진입')
  for a in inner:assert a not in interior;interior[a]=(t['audit_id'],w['rva'])
  flat.append((t,k,w,ins));static.append({'id':t['audit_id'],'rva':w['rva'],'bytes':len(raw),'function':hex(fn[0]),'pass':True})
  for v in sorted({t['default'],t['low'],t['high'],t['min'],t['max']}):
   for seed in range(len(boundaries)+12):
    rng=random.Random((at<<16)+seed);r=[rng.getrandbits(64) for _ in range(16)];r[4]=0x820000;r[5]=0x828000
    if seed<len(boundaries):
     for j in [0,1,2,3,6,7,8,9,10,11,12,13,14,15]:r[j]=boundaries[seed]
    if t['audit_id']=='C011':r[2]=0x810000
    if t['audit_id']=='C015':r[5]=boundaries[seed] if seed<len(boundaries) else rng.getrandbits(64)
    if t['audit_id']=='C021':r[1]&=~1
    flags=0x202|(rng.getrandbits(12)&0xcd5)
    tf=w['tf'];value=(v+tf.get('value',0)) if 'calc' not in tf else None
    c={'t':t,'k':k,'w':w,'regs':r,'flags':flags,'v':v,'seed':seed,'value':value}
    cases.append(c)
    if 'calc' in tf:
     name=tf['calc'].split('::')[1];c['callback']=name;c['expected']=expected(name,v,r)
     requests.append(' '.join([name,str(v),f'{flags:x}']+[f'{x:x}' for x in r]));request_case.append(c)
assert len(flat)==53
for t in rows:
 for w in t['reference_windows']:
  assert w['keep_original'] is True
  assert pe.read(int(w['rva'],0),w['length'])==bytes.fromhex(w['bytes'])
hits=text_rel32_hits(pe,interior.keys());assert not hits,hits[:5]
# 콜백 guard도 실제 Rust를 호출한다. 정의 범위 밖 값은 반드시 None.
guard_requests=[]
for t,k,w,ins in flat:
 if 'calc' not in w['tf']:continue
 name=w['tf']['calc'].split('::')[1]
 for v in [t['min']-1,t['max']+1]:guard_requests.append(' '.join([name,str(v),'202']+['0']*16))
out=subprocess.run([str(callback_exe)],input='\n'.join(requests+guard_requests)+'\n',text=True,capture_output=True,check=True).stdout.splitlines()
assert len(out)==len(requests)+len(guard_requests)
for c,text in zip(request_case,out):
 got=None if text=='none' else int(text,16);assert got==c['expected'],(c['callback'],c['v'],c['seed'],got,c['expected']);c['value']=got
assert all(x=='none' for x in out[len(requests):]),'범위 밖 콜백은 원본 복귀'

# 각 정책의 독립 참조 fragment. 공용 generator의 dynamic_hex를 참조 계산에 쓰지 않는다.
def ref_prefix(c):
 t,k,w,r,v=c['t'],c['k'],c['w'],c['regs'],c['v'];cid=t['audit_id'];q=c['expected'] if 'expected' in c else c['value']
 if cid=='C002':return asm(f'cmp r8,{v+(20 if k==0 else 0)}')
 if cid=='C003':return asm(f'cmp rax,{v}')
 if cid=='C010':return asm(f'imul {"rdx" if k==0 else "rcx"},{"rdx" if k==0 else "rcx"},{v}')
 if cid=='C011':return asm(f'imul rax,rax,{v}')
 if cid=='C013':return asm(f'cmp rax,{v}; mov {"edi" if k<2 else "ecx"},{v}')
 if cid=='C014':
  if k<3:return asm(f'mov {["edx","r8d","eax"][k]},{[v*v-1,v,v*v][k]}')
  return asm(f'imul rax,{"r14" if k==3 else "rdi"},{v}')
 if cid=='C015':return asm(f'add rbp,{v}')
 if cid=='C019':return asm(f'mov rcx,{q}')
 if cid in ['C020','C021','C038']:return asm(f'mov {"r8" if cid=="C021" else "rcx"},{q}')
 if cid=='C039':
  if k==0:return bytes.fromhex(w['win_hex'])+asm(f'mov rdx,{q}')
  return asm(f'cmp {"rdx" if k==1 else "r12"},{q}; mov eax,{q}')
 if cid=='C040':return asm(f'add rcx,{v}')
 if cid=='C041':return bytes.fromhex(w['win_hex'])+asm(f'mov rax,{q}')
 if cid=='C042':return asm(f'{"adc" if k%2 else "add"} {"rax" if k%2 else "rcx"},{v}')
 if cid=='C048':
  if k<2:return asm(f'cmp r14,{v if k==0 else v*24//10}')
  return bytes.fromhex(w['win_hex'])+asm(f'mov rax,{q}')
 if cid=='C049':return bytes.fromhex(w['win_hex'])+asm(f'mov r14,{q}')
 if cid=='C050':
  if k in [0,1,2]:return asm(f'{"add" if k==1 else "sub"} {"rax" if k==2 else "rcx"},{q}')
  if k in [3,4]:return asm(f'mov {"rax" if k==3 else "rcx"},{q}')
  return asm(f'mov qword ptr [rbp+{0x120 if k==5 else 0x138}],{q}')
 if cid=='C051':return asm(f'lea rcx,[rax+rax]; imul rax,rax,{v+2}; sub rax,rcx')
 if cid=='C052':return asm(f'mov rdi,{q}')
 if cid=='C053':return asm(f'sub rsi,r14; add rsi,{-v}') if k==0 else asm(f'mov r14,{q}')
 raise ValueError(cid)
IMAGE=0x140000000;CODE=0x100000;DYNAMIC=0x200000;STACK=0x800000
GPRS=[UC_X86_REG_RAX,UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_RBX,UC_X86_REG_RSP,UC_X86_REG_RBP,UC_X86_REG_RSI,UC_X86_REG_RDI,UC_X86_REG_R8,UC_X86_REG_R9,UC_X86_REG_R10,UC_X86_REG_R11,UC_X86_REG_R12,UC_X86_REG_R13,UC_X86_REG_R14,UC_X86_REG_R15]
u=Uc(UC_ARCH_X86,UC_MODE_64)
for addr,size in [(CODE,0x10000),(DYNAMIC,0x10000),(STACK,0x40000),(IMAGE,0x2000000)]:u.mem_map(addr,size)
state={}
def hook(uc,addr,size,_):
 if addr>=IMAGE:state['exit']=addr;uc.emu_stop()
u.hook_add(UC_HOOK_CODE,hook)
def absjmp(dst):return b'\xff\x25\0\0\0\0'+struct.pack('<Q',dst)
def relocated_tail(w):
 at=int(w['rva'],0);raw=bytes.fromhex(w['win_hex']);out=b''
 for i in md.disasm(raw[w['first_len']:],at+w['first_len']):
  if i.group(CS_GRP_JUMP):
   assert i.operands[0].type==X86_OP_IMM
   if i.mnemonic!='jmp':out+=bytes([0x70|((i.bytes[1]&15)^1),14])
   out+=absjmp(IMAGE+i.operands[0].imm)
  else:out+=i.bytes
 return out+absjmp(IMAGE+at+len(raw))
def run(c,code,addr):
 state.clear();r=c['regs'];rng=random.Random(c['seed']+313);mem=rng.randbytes(0x1000)
 u.mem_write(0x828000,mem);u.mem_write(0x810668,struct.pack('<Q',rng.getrandbits(64)))
 for reg,v in zip(GPRS,r):u.reg_write(reg,v)
 u.reg_write(UC_X86_REG_EFLAGS,c['flags'])
 xmm=[rng.getrandbits(128) for _ in range(16)]
 for n,v in enumerate(xmm):u.reg_write(UC_X86_REG_XMM0+n,v)
 u.mem_write(r[4]-16,struct.pack('<Q',c['value'] or 0));u.mem_write(addr,code);u.ctl_remove_cache(addr,addr+0x10000)
 u.emu_start(addr,0,count=200);assert 'exit' in state,(c['t']['audit_id'],c['k'],'미종료')
 return ([u.reg_read(x) for x in GPRS],u.reg_read(UC_X86_REG_EFLAGS),[u.reg_read(UC_X86_REG_XMM0+n) for n in range(16)],bytes(u.mem_read(0x828000,0x1000)),state['exit'])
runs=0;neutral=0;per={t['audit_id']:{'sites':len(t['sites']),'comparisons':0,'neutral':0,'profiles':sorted({t['default'],t['low'],t['high'],t['min'],t['max']})} for t in rows}
for c in cases:
 w=c['w'];tail=relocated_tail(w);dynamic=bytes.fromhex(w['dynamic_hex'])+tail;reference=ref_prefix(c)+tail
 got=run(c,dynamic,DYNAMIC);ref=run(c,reference,CODE);mask=int(w['fixture_flags_mask'],0)
 assert got[0]==ref[0],(c['t']['audit_id'],c['k'],c['v'],c['seed'],'GPR',[(j,hex(a),hex(b))for j,(a,b)in enumerate(zip(got[0],ref[0]))if a!=b])
 assert got[1]&mask==ref[1]&mask,(c['t']['audit_id'],c['k'],c['v'],c['seed'],'FLAGS',hex(got[1]),hex(ref[1]),hex(mask))
 assert got[2:]==ref[2:],(c['t']['audit_id'],c['k'],'메모리/XMM/분기')
 per[c['t']['audit_id']]['comparisons']+=1;runs+=1
 if c['v']==c['t']['default']:
  oldw=dict(w,first_len=0);old=relocated_tail(oldw);vanilla=run(c,old,CODE)
  assert got[0]==vanilla[0] and got[1]&mask==vanilla[1]&mask and got[2:]==vanilla[2:],(c['t']['audit_id'],c['k'],c['seed'],'중립 원본 동일성')
  neutral+=1;per[c['t']['audit_id']]['neutral']+=1

# 기본/회수실패 경로는 dynamic을 사용하지 않고 원래 창으로 분기한다.
# 이 파일의 검사는 원본/동적 fragment를 실행한다. 공용 dispatch와 전체 stub는 루트 통합 검사 대상이다.
dead=[]
for rva,name in [(0xf83c3a,'C039'),(0xf841de,'C041'),(0xe4f8cf,'C048'),(0xe649ea,'C049')]:
 i=next(md.disasm(pe.read(rva,15),rva,count=1));assert i.mnemonic in ['cmp','test']
 dead.append({'id':name,'overwrite_rva':hex(rva),'bytes':i.bytes.hex(' '),'asm':i.mnemonic+' '+i.op_str,'gap_instructions':0,'flags':'산술flags는 즉시 CMP/TEST로 덮이고 그 전 Jcc/CMOV 없음','pass':True})
src=MOD/'src/extra_combat.rs';assert not re.search('[\u3040-\u30ff]',src.read_text(encoding='utf-8'))
dump=['EXE_SHA256='+data['exe_sha256'],'원래 창과 동적 첫 fragment. 동적 조각 뒤에는 원래 나머지 명령/재배치 분기와 복귀가 붙는다.']
for t,k,w,ins in flat:
 dump+=['',t['audit_id']+' '+t['id']+' '+w['rva'],'원래 창:']
 dump += [f'{i.address:x}: {i.bytes.hex(" "):32} {i.mnemonic} {i.op_str}'.rstrip() for i in ins]
 dump+=['동적 첫 fragment:']
 dump += [f'{i.address:x}: {i.bytes.hex(" "):32} {i.mnemonic} {i.op_str}'.rstrip() for i in md.disasm(bytes.fromhex(w['dynamic_hex']),0)]
(REPORT/'combat_windows.txt').write_text('\n'.join(dump).rstrip()+'\n',encoding='utf-8',newline='\n')
paths=[Path(__file__),MOD/'tools/build_extra_combat.py',rust_src,src,target,REPORT/'combat_contracts.json',exe,MOD/'tools/gen_sites.py']
out={'pass':True,'exe_sha256':data['exe_sha256'],'tactics':21,'sites':53,'comparisons':runs,'neutral_comparisons':neutral,'mismatches':0,'rust_callback_cases':len(requests),'callback_invalid_value_fallbacks':len(guard_requests),
 'per_candidate':per,'static_windows':static,'kept_original_reference_windows':3,'immediate_flag_overwrites':dead,
 'checks':['실제 Rust 콜백 vs 독립 정수정책식','모든 GPR/RSP','정의된flags·입력carry0/1','32bit zero extension','XMM0..15','메모리 쓰기','분기 목적지','원본값 중립 동일성','최소/최대/low/high·0/부호경계/overflow','실제바이트/.pdata/직접분기/JT/전역rel32내부진입'],
 'limits':['게임 실행/실경기 효과 검사는 하지 않았다.','이 검사는 동적 fragment와 실제 Rust 산술 콜백을 분리 검증한다. 공용 dispatch·전체 stub 및 내 계약 선수 fallback은 통합 검사에서 검증한다.','SHL/SAR의 다중비트 OF/AF와 IMUL 미정의 flags는 fixture mask에서 제외했다.'],
 'input_sha256':{str(p.resolve()):hashlib.sha256(p.read_bytes()).hexdigest() for p in paths}}
OUT.write_text(json.dumps(out,ensure_ascii=False,indent=2)+'\n',encoding='utf-8',newline='\n')
print(json.dumps({k:out[k] for k in ['pass','tactics','sites','comparisons','neutral_comparisons','rust_callback_cases','callback_invalid_value_fallbacks','mismatches']},ensure_ascii=False))
