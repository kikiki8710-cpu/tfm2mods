"""확정 운영14계약을 전용JSON으로 생성. 공용 생성기/제품은 수정하지 않는다."""
from pathlib import Path
import json,sys,hashlib
sys.dont_write_bytecode=True
ROOT=Path(__file__).resolve().parent.parent
sys.path.insert(0,str(ROOT/'target/verify_deps'))
from keystone import Ks,KS_ARCH_X86,KS_MODE_64
from capstone import Cs,CS_ARCH_X86,CS_MODE_64
from capstone.x86 import X86_OP_MEM,X86_OP_REG
from gen_sites import PE,REGS
REPORT=Path(r'C:/Users/jungs/Desktop/claude/tfm2/mods_report/tfm2_detail_tactics')
BASE=REPORT/'static_input_20261023/input_candidates.json'
ledger=json.loads(BASE.read_text(encoding='utf-8'));rows={r['id']:r for r in ledger['candidates']}
pe=PE(r'C:/Program Files (x86)/Steam/steamapps/common/Teamfight Manager2/TeamfightManager2.exe')
assert hashlib.sha256(pe.data).hexdigest().upper()==ledger['exe_sha256']
md=Cs(CS_ARCH_X86,CS_MODE_64);md.detail=True;ks=Ks(KS_ARCH_X86,KS_MODE_64)
def caller(off,cases):return dict(kind='caller',return_off=hex(off),cases=[dict(ret_rva=hex(ret),src=src) for ret,src in cases])
collapse_generic=caller(0x78,[(0xd45fad,'stack:48'),(0xdbd5fc,'stackmem:40:3d0'),(0xdbd760,'stackmem:40:3d0'),(0xdbdcc4,'stackmem:40:3d0')])
collapse_opportunity=caller(0xc8,[(0xdbd515,'stackmem:90:3d0')])
pressure=caller(0x68,[(0xdb3ff5,'stack:e8'),(0xdb521d,'stack:c8'),(0xdb5520,'stack:60')])
def respawn(epic):
    off=0xc8 if epic else 0xd8; rbp=0x90 if epic else 0xa0
    nested=caller(off+0x60,[(0xd4357c,f'stackmem:{rbp:x}:1768'),(0xd4b9da,f'stackmem:{rbp:x}:1768')])
    return caller(off,[(0xd5eb79 if epic else 0xd5eb39,'stack:a0' if epic else 'stack:b0'),(0xdcdda5 if epic else 0xdcfd77,'stackmem:90:18' if epic else 'stackmem:a0:78'),(0xed1f7c if epic else 0xed1f96,nested)])
config={
'C007':('objective_power_allowance','오브젝트 전력 보완값','아군 체력 합에 더하는 여유. 높게 하면 진입 조건을 완화한다',200,100,300,0,1000,'오브젝트'),
'C022':('push_start_base','푸시 개시 기본 점수','능력치 보정 전 개시 문턱. 높게 하면 더 높은 점수를 요구한다',130,110,150,0,1000,'푸시·타워'),
'C023':('other_lane_collapse_profile','다른 라인 붕괴 경계','다른 라인 지표와 단계 문턱을 함께 조절. 높게 하면 일찍 푸시를 포기한다',100,75,125,50,150,'라인·이동'),
'C027':('ambush_engage_radius','매복 교전 대상 반경','매복 판단 중 적 후보를 살피는 반경. 시야·최근 관측 조건은 유지한다',250000,200000,300000,1000,2000000,'위치·기술'),
'C028':('objective_lane_pressure_profile','오브젝트 전 라인 압박 문턱','라인 단계·지표·인원 문턱을 함께 조절. 높게 하면 라인 압박 후보를 넓힌다',100,50,150,50,150,'오브젝트'),
'C030':('defense_leave_hp','방어 계획 이탈 체력(%)','일반 방어 체력 문턱. 위협 경로는 이 값보다10 낮게 적용한다',31,21,41,11,100,'전투·생존'),
'C031':('structure_skill_bonus','구조물 기술 후보 가산','넥서스 공격 계획에서 종류4 구조물을 겨냥한 첫 두 기술 점수에만 가산',3,0,6,0,100,'푸시·타워'),
'C032':('epic_final_allies_min','에픽 최종 아군 인원 문턱','지터 보정 후 인원 문턱. 상대 보정값0 예외와 선승인은 유지한다',3,2,4,0,5,'오브젝트'),
'C033':('boss_respawn_wait_seconds','보스 리스폰 대기(초)','현재 보스가 없을 때 남은 리스폰 시간이 이 값보다 크면 계획을 끝낸다',15,10,20,1,120,'오브젝트'),
'C034':('boss_reposition_improvement','보스 사냥 재배치 개선값','국소 재배치에 요구하는 점수 개선. 기존 거리 대체조건은 유지한다',8,4,12,0,1000,'오브젝트'),
'C035':('boss_poke_damage_factor','보스 견제 예상피해 가중치','예상피해를 체력과 비교하는 배수. 높게 하면 위험 판단이 보수적이다',2,1,3,1,10,'오브젝트'),
'C037':('active_recall_slack_profile','마무리 전략의 귀환 종료 여유(초)','현재 전략0은 선택 초, 전략1은1초 적게 적용. 큰 계획 종료 판단만 조절한다',3,2,4,1,30,'오브젝트'),
'C045':('camp_time_margin_profile','캠프 경쟁 시간 여유(%)','생존 여유1초와 상대 처리시간 비교의 반초 여유를 같은 비율로 조절한다',100,50,150,0,300,'정글'),
'D001':('gank_lane_tower_wave_profile','갱 후보 포탑·웨이브 경계','포탑 반경·미니언 차이·웨이브 위치 필터를 함께 조절. 높게 하면 보수적이다',100,75,125,50,150,'정글'),
}
def source(id,w):
    name=w['name'];a=int(w['rva'],16)
    if id=='C007':return 'mem:rbp:c8'
    if id=='C022':return 'stack:60'
    if id=='C023':return collapse_generic if name.startswith('generic') else collapse_opportunity if name.startswith('opportunity') else 'mem:rbp:3d0'
    if id=='C027':return 'mem:rbp:2580' if 'sq1' in name else 'stack:50'
    if id=='C028':return pressure
    if id=='C030':return caller(0x48,[(0xf0b56d,'reg:r14')])
    if id=='C031':return 'reg:rdi'
    if id=='C032':return 'stack:70'
    if id=='C033':return respawn('epic' in name)
    if id=='C034':return 'mem:rbp:478'
    if id=='C035':return 'mem:rbp:'+{0xe92f74:'90',0xe92ac1:'d0',0xe988f8:'230',0xe96af2:'220'}[a]
    if id=='C037':return 'reg:r15'
    if id=='C045':return 'mem:rbp:e0'
    if id=='D001':return caller(0x148,[(0xe2f987,'stackmem:140:-8')])
    raise ValueError(id)
def transform(id,w):
    n=w['name']
    if id=='C023':return 'collapse_metric' if 'metric' in n else 'collapse_last_stage' if n=='push_stage3' else 'collapse_stage'
    if id=='C027':return None if 'sq1' in n else 'radius_squared'
    if id=='C028':return 'pressure_'+n.split('_')[0]
    if id=='C030':return 'defense_threat_hp' if 'threat' in n else None
    if id=='C031':return 'structure_rcx' if 'tag12' in n else 'structure_rdx'
    if id=='C033':return 'respawn_seconds'
    if id=='C035':return 'damage_increment'
    if id=='C037':return 'finish_seconds3' if '3TPS' in n else 'finish_increment2'
    if id=='C045':return 'camp_arrival_margin' if 'half' in n else 'camp_survival_margin'
    if id=='D001':return {'tower_distance_with_wave_advantage':'tower_radius_high','tower_distance_base':'tower_radius_low','minion_count_difference_gate':'tower_minion_gate','minion_from_mid_limit':'tower_wave_limit'}[n]
    return None
def dynamic(ins,id):
    # 원래 결과가 R11이면 R10을 임시 레지스터로 보존한다.
    used=set()
    for o in ins.operands:
        if o.type==X86_OP_REG:used.add(ins.reg_name(o.reg).replace('d',''))
        elif o.type==X86_OP_MEM:used.add(ins.reg_name(o.mem.base))
    scratch='r10' if 'r11' in used else 'r11'
    operand=ins.op_str.split(',')[0]
    load=f'push {scratch}; mov {scratch},qword ptr [rsp-8]; '
    if ins.mnemonic in ('mov','movabs','lea') or (id=='C045' and ins.mnemonic=='shr'):
        dest=operand;src=scratch if ins.operands[0].size==8 else scratch+'d'
        core=f'mov {dest},{src}'
    elif ins.mnemonic in ('add','cmp'):
        src=scratch if ins.operands[0].size==8 else scratch+'d'
        if ins.operands[0].type==X86_OP_MEM and ins.reg_name(ins.operands[0].mem.base)=='rsp':
            raise AssertionError('RSP memory requires adjusted displacement')
        core=f'{ins.mnemonic} {operand},{src}'
    else:raise ValueError((id,ins.mnemonic,ins.op_str))
    asm=load+core+f'; pop {scratch}'
    return bytes(ks.asm(asm)[0]),asm
result=[]
for id,c in config.items():
    stable,name,desc,default,low,high,mn,mx,tab=c
    tactic=dict(audit_id=id,id=stable,name=name,desc=desc,scope='Player',default=default,low=low,high=high,min=mn,max=mx,kind='Value',tab=tab,pos_mask=31,sites=[])
    assert rows[id]['status']=='ready'
    for w in rows[id]['windows']:
        a=int(w['rva'],16);raw=bytes.fromhex(w['bytes']);assert pe.read(a,len(raw))==raw
        ins=next(md.disasm(raw,a));calc=transform(id,w);dy,asm=dynamic(ins,id)
        tf={'calc':'extra_ops::'+calc} if calc else {'kind':'Sq1'} if id=='C027' else {'kind':'Add','value':0}
        tactic['sites'].append(dict(rva=w['rva'],win_hex=w['bytes'],first_len=ins.size,src=source(id,w),team_chk='stack:78' if id=='C022' else None,tf=tf,dynamic_hex=dy.hex(' '),notes=rows[id]['formula'],window_name=w['name'],dynamic_asm=asm,first_asm=f'{ins.mnemonic} {ins.op_str}'))
    result.append(tactic)
data=dict(exe_sha256=ledger['exe_sha256'],tactics=result)
path=ROOT/'tools/targets/extra_ops_0.6.5.json';path.write_text(json.dumps(data,ensure_ascii=False,indent=2)+'\n',encoding='utf-8',newline='\n')
print(json.dumps(dict(tactics=len(result),sites=sum(len(t['sites']) for t in result),output=str(path))))
