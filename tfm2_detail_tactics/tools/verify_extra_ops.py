"""운영14개51창: 실제Rust변환+Unicorn 원본/동적 차등, 폭/flags/분기 검사."""
from pathlib import Path
import sys,json,hashlib,re,struct,random,subprocess,collections
sys.dont_write_bytecode=True
ROOT=Path(__file__).resolve().parent.parent
sys.path.insert(0,str(ROOT/'target/verify_deps'))
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
from keystone import Ks,KS_ARCH_X86,KS_MODE_64
from capstone import Cs,CS_ARCH_X86,CS_MODE_64,CS_GRP_JUMP,CS_GRP_CALL,CS_GRP_RET
from capstone.x86 import X86_OP_MEM,X86_OP_REG,X86_OP_IMM,X86_REG_RIP
from gen_sites import PE,REGS,find_func,text_rel32_hits,parse_src,JCC
OUT=Path(r'C:/Users/jungs/Desktop/claude/tfm2/mods_report/tfm2_detail_tactics/implement_20261024');OUT.mkdir(exist_ok=True)
DATAFILE=ROOT/'tools/targets/extra_ops_0.6.5.json';BASE=OUT.parent/'static_input_20261023/input_candidates.json'
data=json.loads(DATAFILE.read_text(encoding='utf-8'));base=json.loads(BASE.read_text(encoding='utf-8'));byid={r['id']:r for r in base['candidates']}
pe=PE(r'C:/Program Files (x86)/Steam/steamapps/common/Teamfight Manager2/TeamfightManager2.exe')
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
assert hashlib.sha256(pe.data).hexdigest().upper()==data['exe_sha256']==base['exe_sha256']
starts,ends=pe.functions();md=Cs(CS_ARCH_X86,CS_MODE_64);md.detail=True;ks=Ks(KS_ARCH_X86,KS_MODE_64)
mask=(1<<64)-1
def asm(s):return bytes(ks.asm(s)[0])
def calculate(calc,v,regs,tps):
    if not calc:return v
    c=calc.split('::')[-1]
    funcs={
      'collapse_metric':lambda:-(300000//v)&mask,
      'collapse_stage':lambda:(-3 if v<100 else -1 if v>100 else -2)&mask,
      'collapse_last_stage':lambda:(-4 if v<100 else -2 if v>100 else -3)&mask,
      'radius_squared':lambda:v*v,
      'pressure_stage':lambda:max(3*v//100,1), 'pressure_metric':lambda:2000*v//100,'pressure_count':lambda:max(2*v//100,1),
      'defense_threat_hp':lambda:v-10, 'structure_rcx':lambda:regs[1]*v&mask,'structure_rdx':lambda:regs[2]*v&mask,
      'respawn_seconds':lambda:tps*v&mask,'damage_increment':lambda:regs[0]*(v-1)&mask,
      'finish_seconds3':lambda:regs[14]*v&mask,'finish_increment2':lambda:regs[14]*(v-2)&mask,
      'camp_survival_margin':lambda:(tps*v//100)&mask,'camp_arrival_margin':lambda:((regs[1]>>1)*v//100)&mask,
      'tower_radius_high':lambda:(140000*v//100)**2+1,'tower_radius_low':lambda:(100000*v//100)**2+1,
      'tower_minion_gate':lambda:2 if v<100 else 0 if v>100 else 1,'tower_wave_limit':lambda:300000//v,
    }
    return funcs[c]()

# 실행하는변환은테스트용재작성식이아닌제품extra_ops.rs를include한다.
harness='''#![allow(dead_code)]
use std::io::{self,BufRead};
mod micro {
 #[derive(Clone,Copy)] pub enum Src {Mem(u8,i32),Mem2(u8,i32,i32)}
 pub struct Frame {pub regs:[u64;16],pub tps:u64}
 impl Frame {pub fn reg(&self,r:u8)->Option<u64>{self.regs.get(r as usize).copied()} pub fn scalar(&self,s:Src)->Option<u64>{match s {Src::Mem(5,0x10)|Src::Mem2(13,8,0x12f8)=>Some(self.tps),_=>None}}}
}
#[path="'''+str(ROOT/'src/extra_ops.rs').replace('\\','/')+'''"] mod extra_ops;
fn main(){for l in io::stdin().lock().lines(){let l=l.unwrap();let mut p=l.split_whitespace();let name=p.next().unwrap();let v:i64=p.next().unwrap().parse().unwrap();let tps:u64=p.next().unwrap().parse().unwrap();let mut regs=[0u64;16];for x in &mut regs{*x=p.next().unwrap().parse().unwrap();}let f=micro::Frame{regs,tps};let z=match name{MATCHES _=>None}; match z{Some(v)=>println!("{}",v),None=>println!("NONE")}}}
'''
names=sorted({s['tf']['calc'].split('::')[-1] for t in data['tactics'] for s in t['sites'] if 'calc'in s['tf']})
harness=harness.replace('MATCHES', ''.join('"'+n+'"=>extra_ops::'+n+'(&f,v),' for n in names))
hp=OUT/'ops_callback_harness.rs';hp.write_text(harness,encoding='utf-8',newline='\n');exe=OUT/'ops_callback_harness.exe'
rustc=Path.home()/'.cargo/bin/rustc.exe'
subprocess.run([str(rustc),'--edition=2021',str(hp),'-o',str(exe)],check=True,capture_output=True)
rusttests=OUT/'ops_callback_tests.exe'
subprocess.run([str(rustc),'--edition=2021','--test',str(hp),'-o',str(rusttests)],check=True,capture_output=True)
unit=subprocess.run([str(rusttests)],check=True,capture_output=True,text=True).stdout
(OUT/'ops_callback_tests.txt').write_text(unit.rstrip()+'\n',encoding='utf-8',newline='\n')

REGIDS=[UC_X86_REG_RAX,UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_RBX,UC_X86_REG_RSP,UC_X86_REG_RBP,UC_X86_REG_RSI,UC_X86_REG_RDI,UC_X86_REG_R8,UC_X86_REG_R9,UC_X86_REG_R10,UC_X86_REG_R11,UC_X86_REG_R12,UC_X86_REG_R13,UC_X86_REG_R14,UC_X86_REG_R15]
CODE=0x100000;DYNAMIC=0x200000;STACK=0x800000;RSP=STACK+0x10000;RBP=STACK+0x18000;MEM=0xa00000;GAME=0x140000000
u=Uc(UC_ARCH_X86,UC_MODE_64)
for a,n in [(CODE,0x10000),(DYNAMIC,0x10000),(STACK,0x30000),(MEM,0x30000),(GAME,0x2000000)]:u.mem_map(a,n)
state={}
def hook(uc,addr,size,_):
    if addr>=GAME:state['exit']=addr;uc.emu_stop()
u.hook_add(UC_HOOK_CODE,hook)
def jump(target):return b'\xff\x25\0\0\0\0'+struct.pack('<Q',target)
def relocate(insns,end):
    out=b''
    for i in insns:
        if i.group(CS_GRP_JUMP):
            target=GAME+i.operands[0].imm
            if i.mnemonic!='jmp':out+=bytes([0x70|(JCC[i.mnemonic]^1),14])
            out+=jump(target)
        else:out+=i.bytes
    return out+jump(GAME+end)
def reference_first(i,value):
    operand=i.op_str.split(',')[0]
    if i.mnemonic in ('mov','movabs','lea','shr'):
        bits=i.operands[0].size*8;return asm(f'mov {operand}, {value&((1<<bits)-1)}')
    used=[i.reg_name(o.reg) for o in i.operands if o.type==X86_OP_REG]+[i.reg_name(o.mem.base) for o in i.operands if o.type==X86_OP_MEM]
    scratch='r10' if 'r11'in used else 'r11'
    operand_size=i.operands[0].size;reg=scratch if operand_size==8 else scratch+'d'
    return asm(f'push {scratch}; mov {scratch},{value}; {i.mnemonic} {operand},{reg}; pop {scratch}')

cases=[];rustlines=[];callback_expected=[]
edges=[0,1,2,0x7fffffff,0x80000000,0xffffffff,0x7fffffffffffffff,0x8000000000000000,0xffffffffffffffff]
for tid,t in enumerate(data['tactics']):
    for s in t['sites']:
        a=int(s['rva'],16);raw=bytes.fromhex(s['win_hex']);insns=list(md.disasm(raw,a));i=insns[0]
        assert s['first_len']==i.size and pe.read(a,len(raw))==raw
        for v in sorted(set([t['default'],t['low'],t['high'],t['min'],t['max']])):
            for seed in range(32):
                rng=random.Random(a+seed*112237+v)
                regs=[rng.getrandbits(64) for _ in range(16)];regs[4]=RSP;regs[5]=RBP
                mem=bytearray(rng.randbytes(0x30000));stack=bytearray(rng.randbytes(0x30000))
                # 모든원래메모리피연산자의기준을실제맵에배치. 검사대상은설정Player와별개다.
                writes=[]
                for ins in insns:
                    for o in ins.operands:
                        if o.type==X86_OP_MEM and ins.mnemonic!='lea':
                            assert not o.mem.index and o.mem.base!=X86_REG_RIP
                            r=REGS.index(ins.reg_name(o.mem.base))
                            if r not in (4,5):regs[r]=MEM+0x10000+r*0x100
                            value=edges[seed%len(edges)] if seed<18 else rng.getrandbits(o.size*8)
                            addr=regs[r]+o.mem.disp
                            writes.append((addr,(value&((1<<(o.size*8))-1)).to_bytes(o.size,'little')))
                if t['audit_id']=='C031':regs[1 if s['window_name'].endswith('tag12')else 2]=seed%2
                if t['audit_id'] in ('C033','C037','C045'):
                    regs[0]=edges[seed%len(edges)] if seed<18 else rng.getrandbits(64)
                    if t['audit_id']=='C037':regs[14]=edges[seed%len(edges)]
                    if 'half' in s['window_name']:regs[1]=edges[seed%len(edges)]
                tps=edges[seed%len(edges)] if seed<18 else rng.getrandbits(64)
                if t['audit_id']=='C033':regs[0]=tps*5&mask
                if t['audit_id']=='C045' and 'half' not in s['window_name']:writes.append((RBP+0x10,tps.to_bytes(8,'little')))
                calc=s['tf'].get('calc');value=calculate(calc,v,regs,tps)
                if s['tf'].get('kind')=='Sq1':value=v*v+1
                flags=0x202|(rng.getrandbits(12)&0xcd5)
                case=(t,s,i,insns,v,seed,regs,tps,writes,bytes(mem),bytes(stack),value,flags)
                cases.append(case)
                if calc:
                    rustlines.append(calc.split('::')[-1]+' '+str(v)+' '+str(tps)+' '+' '.join(map(str,regs)))
                    callback_expected.append(value)
proc=subprocess.run([str(exe)],input='\n'.join(rustlines)+'\n',capture_output=True,text=True,check=True)
actual=[int(x) for x in proc.stdout.splitlines()];assert actual==callback_expected,('Rust callback != independent formula',next(((a,b)for a,b in zip(actual,callback_expected)if a!=b),None))

runs=neutral=0;per_site=collections.Counter()
for t,s,i,insns,v,seed,regs,tps,writes,mem,stack,value,flags in cases:
    tail=relocate(insns[1:],int(s['rva'],16)+len(bytes.fromhex(s['win_hex'])))
    ref=reference_first(i,value)+tail
    dynamic=bytes.fromhex(s['dynamic_hex'])+tail
    native=relocate(insns,int(s['rva'],16)+len(bytes.fromhex(s['win_hex'])))
    def run(code,address):
        state.clear();u.mem_write(MEM,mem);u.mem_write(STACK,stack)
        for addr,b in writes:u.mem_write(addr,b)
        u.mem_write(RSP-16,value.to_bytes(8,'little'))
        for r,x in zip(REGIDS,regs):u.reg_write(r,x)
        for n in range(16):u.reg_write(UC_X86_REG_XMM0+n,(n+1)*(2**64+1))
        u.reg_write(UC_X86_REG_EFLAGS,flags);u.mem_write(address,code);u.ctl_remove_cache(address,address+0x10000)
        u.emu_start(address,0,count=160)
        assert 'exit'in state,(s['rva'],'no exit')
        return [u.reg_read(r) for r in REGIDS],u.reg_read(UC_X86_REG_EFLAGS)&0xcd5,[u.reg_read(UC_X86_REG_XMM0+n)for n in range(16)],state['exit'],bytes(u.mem_read(MEM,0x30000)),bytes(u.mem_read(RSP,0x20000))
    r=run(ref,CODE);d=run(dynamic,DYNAMIC)
    assert r==d,(t['audit_id'],s['rva'],v,seed,'dynamic',[(n,a,b)for n,(a,b)in enumerate(zip(r[:4],d[:4]))if a!=b])
    runs+=1;per_site[s['rva']]+=1
    if v==t['default']:
        n=run(native,CODE)
        assert n==d,(t['audit_id'],s['rva'],seed,'neutral native != dynamic',[(k,a,b)for k,(a,b)in enumerate(zip(n[:4],d[:4]))if a!=b])
        neutral+=1;runs+=1

# 메타데이터와실제PE창의명령경계/분기/JT/기존창충돌을독립대조한다.
own={t['audit_id'] for t in data['tactics']};old=[(r['id'],int(w['rva'],16),w['length'])for r in base['candidates']for w in r['windows']]
ready=[w for r in base['candidates']if r['status']=='ready'for w in r['windows']]
src=(ROOT/'src/sites.rs').read_text(encoding='utf-8');prod=[(int(m[2],16),len(re.findall(r'0x[0-9a-f]+',m[3])),m[1])for m in re.finditer(r'Patch \{ name: "([^"]+)".*?rva: (0x[0-9a-f]+), win: &\[([^\]]+)\]',src)]
# 통합 중 생성 파일에서도 앞53개 제품창을 고정해 새창을 충돌로 오인하지 않는다.
prod=prod[:53];assert len(prod)==53
fcache={};checks=[]
for t in data['tactics']:
    for s in t['sites']:
        a=int(s['rva'],16);n=len(bytes.fromhex(s['win_hex']));fn=find_func(starts,ends,a);assert fn
        if fn not in fcache:
            body=list(md.disasm(pe.read(fn[0],fn[1]-fn[0]),fn[0]));targets=set();tabs=0
            for x in body:
                if (x.group(CS_GRP_CALL)or x.group(CS_GRP_JUMP))and x.operands[0].type==X86_OP_IMM:targets.add(x.operands[0].imm)
                if x.mnemonic=='lea'and len(x.operands)==2 and x.operands[1].type==X86_OP_MEM and x.operands[1].mem.base==X86_REG_RIP:
                    tb=x.address+x.size+x.operands[1].mem.disp
                    if pe.off(tb)is None:continue
                    raw=pe.read(tb,8192);mode=None;entries=[]
                    for j in range(len(raw)//4):
                        e=struct.unpack_from('<i',raw,4*j)[0]
                        if mode is None:mode='rel'if fn[0]<=tb+e<fn[1]else'abs'if fn[0]<=e<fn[1]else None
                        if mode is None:break
                        dest=tb+e if mode=='rel'else e
                        if not fn[0]<=dest<fn[1]:break
                        entries.append(dest)
                    if entries:targets.update(entries);tabs+=1
            fcache[fn]=body,targets,tabs
        body,targets,tabs=fcache[fn];bounds={x.address for x in body}|{fn[1]};inter=set(range(a+1,a+n))
        cross=[id for id,b,z in old if id!=t['audit_id']and a<b+z and b<a+n]
        pp=[name for b,z,name in prod if a<b+z and b<a+n]
        external=text_rel32_hits(pe,inter)
        assert a in bounds and a+n in bounds and not cross and not pp and not targets&inter and not external
        assert parse_src(s['src']);parse_src(s['team_chk'])
        checks.append(dict(audit_id=t['audit_id'],rva=s['rva'],length=n,bytes=pe.read(a,n).hex(' '),first_len=s['first_len'],dynamic_hex=s['dynamic_hex'],function=list(map(hex,fn)),comparisons=per_site[s['rva']],cross_candidate_overlap=cross,product_overlap=pp,interior_direct_or_jt=list(map(hex,targets&inter)),global_rel32_interior=external,jump_tables=tabs,pass_=True))
for r in checks:r['pass']=r.pop('pass_')
inputs=[Path(__file__),ROOT/'tools/build_extra_ops.py',ROOT/'src/extra_ops.rs',DATAFILE,BASE,ROOT/'tools/gen_sites.py']
result=dict(pass_=True,exe_sha256=data['exe_sha256'],tactics=14,sites=51,comparisons=runs,neutral_native_comparisons=neutral,callback_comparisons=len(actual),rust_tests=2,mismatches=0,baseline_ready_windows=len(ready),baseline_all_windows=len(old),production_windows=53,windows=checks,checks=['all GPR incl R11 result','RSP','defined arithmetic flags and DF','XMM0-15','memory width and untouched bytes','relocated Jcc destination','native-neutral equality','actual included Rust callback vs independent formula'],input_sha256={str(p.resolve()):sha(p)for p in inputs})
result['pass']=result.pop('pass_')
(OUT/'ops_verify.json').write_text(json.dumps(result,ensure_ascii=False,indent=2)+'\n',encoding='utf-8',newline='\n')
print(json.dumps({k:v for k,v in result.items()if k not in ['windows','input_sha256']},ensure_ascii=False))
