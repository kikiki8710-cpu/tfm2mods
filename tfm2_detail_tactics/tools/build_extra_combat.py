"""전투21축의 수동 검토 계약을 네이티브 적용 target으로 조립한다."""
from pathlib import Path
import sys,json,hashlib
sys.path.insert(0,str(Path(__file__).parent))
sys.path.insert(0,str(Path(__file__).parents[1]/'target/verify_deps'))
from keystone import Ks,KS_ARCH_X86,KS_MODE_64
from gen_sites import PE
MOD=Path(__file__).parents[1]
REPORT=Path(r'C:\Users\jungs\Desktop\claude\tfm2\mods_report\tfm2_detail_tactics\implement_20261024')
BASE=REPORT/'combat_contracts.json'
data=json.loads(BASE.read_text(encoding='utf-8'));rows={r['id']:r for r in data['candidates']}
pe=PE(r'C:\Program Files (x86)\Steam\steamapps\common\Teamfight Manager2\TeamfightManager2.exe')
assert hashlib.sha256(pe.data).hexdigest().upper()==data['exe_sha256']
ks=Ks(KS_ARCH_X86,KS_MODE_64)
def asm(s):return bytes(ks.asm(s)[0])
def calc(s):return {'calc':'extra_combat::'+s}
def add(d=0):return {'kind':'Add','value':d}
def result(dst,width=64):return f'mov {dst},'+('qword' if width==64 else 'dword')+' ptr [rsp-16]'
def op(m,dst,width=64):
 return 'push r11; mov r11,qword ptr [rsp-8]; '+m+' '+dst+(',' if ',' not in dst else ',')+('r11' if width==64 else 'r11d')+'; pop r11'
def cap(dst,reg):return 'push r11; mov r11,qword ptr [rsp-8]; cmp '+dst+',r11; mov '+reg+',r11d; pop r11'
def site(cid,index,first,source,tf,dynamic,chk=None,notes='',flags_mask='0xcd5'):
 w=rows[cid]['windows'][index];rva=int(w['rva'],0);raw=bytes.fromhex(w['bytes'])
 assert pe.read(rva,len(raw))==raw
 if isinstance(dynamic,str):dynamic=asm(dynamic)
 return {'rva':w['rva'],'win_hex':raw.hex(' '),'first_len':first,'src':source,'team_chk':chk,'tf':tf,
  'dynamic_hex':dynamic.hex(' '),'notes':notes,'fixture_flags_mask':flags_mask}
def tactic(cid,ident,name,desc,default,low,high,minv,maxv,tab,sites,scope='Player',references=None):
 return {'audit_id':cid,'id':ident,'name':name,'desc':desc,'scope':scope,'default':default,'low':low,'high':high,
 'min':minv,'max':maxv,'kind':'Value','tab':tab,'pos_mask':31,'sites':sites,'reference_windows':references or [],
 'default_policy':'기본 단계는 원본 트램폴린. 선택값이 원본과 같으면 동적 연산도 원본과 동일. 주체 회수/내 계약 선수 검증 실패는 원본.'}
out=[]
out.append(tactic('C002','pickoff_hp_bands','픽오프 저체력 기준(%)','하단 체력 문턱과 그보다20 높은 문턱을 함께 조절한다. 높을수록 체력 있는 적도 우선한다',61,41,71,1,80,'갱·로밍',[
 site('C002',0,4,'mem:rbp:0x98',add(20),op('cmp','r8')),
 site('C002',1,4,'mem:rbp:0x98',add(),op('cmp','r8'))]))
out.append(tactic('C003','structure_retreat_keep_hp','구조물 공격 중 후퇴 유지 체력(%)','추적 중 적 구조물을 공격하며 이미 후퇴 요청이 있을 때, 이 체력 이하면 후퇴 요청을 유지한다',65,50,80,0,100,'푸시·타워',[
 site('C003',0,4,'mem:rbp:0x660',add(),op('cmp','rax'),'mem:rbp:0x5b8')]))
out.append(tactic('C010','chase_speed_ratio','추격 속도비 기준(10분율)','적 이동속도×10과 내 속도×이 값을 비교한다. 낮을수록 속도 차이로 추격을 멈추기 쉽다',9,7,11,1,20,'전투·생존',[
 site('C010',1,4,'mem:rbp:0x218',calc('chase_ratio_rdx'),result('rdx')),
 site('C010',3,4,'mem:rbp:0x348',calc('chase_ratio_rcx'),result('rcx'))],references=[dict(w,keep_original=True,reason='적속도×10은 원래 ADD/LEA·wrap/flags까지 유지') for w in [rows['C010']['windows'][0],rows['C010']['windows'][2]]]))
second={'kind':'caller','return_off':'0x208','cases':[
 {'ret_rva':'0xdca088','src':'stack:0x1f0'},{'ret_rva':'0xe0e5a7','src':'stackmem:0x1b0:0x720'}, {'ret_rva':'0xf9960f','src':'stack:0x1f0'}]}
tower_src={'kind':'caller','return_off':'0x1b8','cases':[
 {'ret_rva':'0xdfbb38','src':second},{'ret_rva':'0xe12669','src':'stackmem:0x1b0:0x720'},
 {'ret_rva':'0xf7fdab','src':'stackmem:0x1b0:0x480'},{'ret_rva':'0x10135b7','src':'stack:0x210'}, {'ret_rva':'0x101381c','src':'stack:0x198'}]}
out.append(tactic('C011','tower_tank_hits','타워 탱킹 필요 체력(타)','타워 피해를 이 횟수만큼 견딜 체력이 있는 아군을 탱킹 후보로 고른다. 팀 공통이며 낮을수록 위험을 감수한다',3,2,4,1,10,'푸시·타워',[
 site('C011',0,4,tower_src,calc('tower_hits'),result('rax'),'stack:0x140','알려진5경로 및 dfba90 상위3경로를2단 whitelist로 회수. 알려지지 않은 반환주소는 원본.')],scope='Team'))
out.append(tactic('C013','kill_bonus_cap','처치 가산점 상한','적을 처치할 가능성의 가산점 상한. 높을수록 마무리 공격·기술을 우선한다',100,50,150,0,1000,'전투·생존',[
 site('C013',k,9,'mem:rbp:0x58',add(),cap('rax','edi' if k<2 else 'ecx')) for k in range(6)]))
out.append(tactic('C014','ally_spacing_radius','아군 간격 반경','가까운 아군끼리 벌어지는 반경. 높을수록 서로 넓게 선다',48000,32000,60000,1000,65535,'위치·기술',[
 site('C014',0,5,'mem:rbp:0x290',calc('radius_square_minus_one'),result('edx',32),'mem:rbp:0x1b8'),
 site('C014',1,6,'mem:rbp:0x290',add(),result('r8d',32),'mem:rbp:0x1b8'),
 site('C014',2,5,'mem:rbp:0x290',calc('radius_square'),result('eax',32),'mem:rbp:0x1b8'),
 site('C014',3,7,'mem:rbp:0x290',add(),'push r11; mov r11,qword ptr [rsp-8]; mov rax,r14; imul rax,r11; pop r11','mem:rbp:0x1b8',flags_mask='0xc01'),
 site('C014',4,7,'mem:rbp:0x290',add(),'push r11; mov r11,qword ptr [rsp-8]; mov rax,rdi; imul rax,r11; pop r11','mem:rbp:0x1b8',flags_mask='0xc01')]))
out.append(tactic('C015','near_ally_link_margin','근접 아군 연계 거리 여유','아군의 공격 범위 밖에서도 연계할 수 있다고 인정하는 거리 여유. 높을수록 넓게 합을 맞춘다',30000,15000,60000,0,300000,'위치·기술',[
 site('C015',0,7,'stack:0x40',add(),op('add','rbp'),'stack:0x88','현재 RBP는 사거리 누적값이며 프레임이 아님. 원래 RSP 기준 주체40/팀88.')]))
out.append(tactic('C019','basic_trade_hp_divisor','기본공격 교환 체력 여유(분모)','내 최대 체력/이 값과 적 기본공격 피해를 합친 체력보다 높을 때 안전하다고 본다. 높을수록 교환에 적극적',4,2,8,1,32,'전투·생존',[
 site('C019',0,4,'reg:r13',calc('trade_hp_divisor'),result('rcx'))]))
out.append(tactic('C020','support_cap_multiplier','회복·실드 평가 허용 배수','회복은 잃은 체력+피해평가×이 값, 실드는 피해평가×(이 값+1)까지 가치로 센다. 실제 회복·실드량은 바꾸지 않는다',2,1,4,0,10,'위치·기술',[
 site('C020',0,4,'mem:rbp:0x548',calc('heal_r8'),result('rcx')),
 site('C020',1,4,'mem:rbp:0x548',calc('shield_r8'),result('rcx')),
 site('C020',2,4,'mem:rbp:0x548',calc('heal_r14'),result('rcx')),
 site('C020',3,4,'mem:rbp:0x548',calc('shield_r14'),result('rcx'))]))
out.append(tactic('C021','buff_arrival_value_seconds','지원 기술 도착 가치 기간(초)','도착이 늦는 지원 기술의 가치를 이 기간에 걸쳐 낮춘다. 높을수록 늦게 도착해도 높게 평가한다',6,3,10,1,30,'위치·기술',[
 site('C021',0,4,'mem:rbp:0x548',calc('buff_arrival_ticks'),result('r8'),'','RCX는 직전2TPS. 홀수 입력은 계약 밖으로 보고 원본 복귀.',flags_mask='0xcc5')]))
out[-1]['sites'][0]['team_chk']=None
out.append(tactic('C038','position_isolation_risk_percent','고립 위치 위험 가산(%)','주변 아군 없이 적이2명 이상일 때의 위치 위험 가산. 높을수록 혼자 들어가는 위치를 피한다',25,10,50,0,1000,'위치·기술',[
 site('C038',0,4,'mem:rbp:0x480',calc('isolated_percent'),result('rcx'),notes='직전 5X의 모듈러 역원으로 X를 회수해 K×X를 만든다. LEA의 flags 불변 유지.')]))
out.append(tactic('C039','front_ally_risk_scale','튼튼한 아군보다 앞섬 억제(%)','튼튼한 아군보다 앞선 위치의 위험 점수와 두 상한을 함께 비례 조정한다. 높을수록 아군 뒤에 서려 한다',100,50,150,1,400,'위치·기술',[
 site('C039',0,17,'mem:rbp:0x480',calc('front_ally_distance_score'),bytes.fromhex(rows['C039']['windows'][0]['bytes'])+asm(result('rdx')),notes='원래 MUL의 RAX/RCX와 flags를 재현하고 RDX 점수만 교체. 다음 f83c3a CMP가 산술 flags 전부 덮음.',flags_mask='0x4c5'),
 site('C039',1,9,'mem:rbp:0x480',calc('front_ally_cap'),cap('rdx','eax')),
 site('C039',2,9,'mem:rbp:0x480',calc('front_ally_total_cap'),cap('r12','eax'))]))
out.append(tactic('C040','battle_skill_safety_margin','교전 기술 안전 거리 여유','추격·교전에서 기술 범위에 더하는 안전 여유. 높을수록 더 떨어진 위치를 안전하다고 본다. 시야 밖 상한은 원래 규칙 유지',50000,25000,75000,0,300000,'위치·기술',[
 site('C040',k,7,'mem:rbp:0x230',add(),op('add','rcx'),'mem:rbp:0x228') for k in range(1,5)],references=[dict(rows['C040']['windows'][0],keep_original=True,reason='독립 비가시 cap100000/OR(rawVision) 분기는 원본 유지')]))
out.append(tactic('C041','wave_position_risk_divisor','라인 웨이브 위치 위험 할인(분모)','원래 웨이브 안전 조건을 만족할 때 위치 위험을 이 값으로 나눈다. 높을수록 웨이브 주변을 덜 위험하게 본다',4,2,8,1,32,'위치·기술',[
 site('C041',0,15,'mem:rbp:0x480',calc('wave_risk_divisor'),bytes.fromhex(rows['C041']['windows'][0]['bytes'])+asm(result('rax')),notes='원래 15B 부수 효과를 유지하고 몫만 교체. f841de CMP가 flags 전부 덮음.',flags_mask='0x4c5')]))
def caller42(callee,ret):
 cases=[]
 for x in rows['C042']['caller_sources']:
  if x['callee']!=callee:continue
  s=x['src'];src=('stack:'+s['offset']) if s['type']=='Stack' else ('stackmem:'+s['offset']+':'+s['deref'])
  cases.append({'ret_rva':x['ret_rva'],'src':src})
 return {'kind':'caller','return_off':ret,'cases':cases}
out.append(tactic('C042','movement_skill_period_margin','이동기 절약 보행 여유(업데이트)','이동기 절약 모드에서 보행 시간과 기술 주기를 비교할 때 더하는 여유. 초 단위가 아니며 높을수록 이동기를 아낀다',60,30,90,0,600,'위치·기술',[
 site('C042',k,4,caller42('0xf59470' if k<2 else '0xf6ad40','0x2a8' if k<2 else '0x298'),add(),op('adc' if k%2 else 'add','rax' if k%2 else 'rcx'),notes='MOV/PUSH/POP은 입력CF를 바꾸지 않음. ADC는 원래 CF를 받아 실행하고 뒤 CMP가 비교 flags 생성.') for k in range(4)]))
out.append(tactic('C048','battle_risk_retreat_threshold','교전 위험 후퇴 문턱','위치 위험 K/2.4K와 직접 피해평가 비율2K를 함께 조정한다. 낮을수록 작은 위험에도 후퇴한다',10,5,15,1,50,'전투·생존',[
 site('C048',0,4,'mem:rbp:0x430',add(),op('cmp','r14')),
 site('C048',1,4,'mem:rbp:0x430',calc('risk_retreat_high'),op('cmp','r14')),
 site('C048',2,8,'mem:rbp:0x430',calc('risk_retreat_pressure'),bytes.fromhex(rows['C048']['windows'][2]['bytes'])+asm(result('rax')),notes='원래 SHL 부수 flags를 보존하고 H×2K만 교체. 다음 e4f8cf CMP가 모든 산술 flags 덮음.',flags_mask='0x4c5')]))
out.append(tactic('C049','unseen_self_risk_divisor','시야 밖 자기 위험 할인(분모)','적 시야에 잡히지 않을 때 기본 자기위험을 이 값으로 나눈다. 다른 위험 가산은 유지하며 높을수록 은폐를 믿는다',3,2,6,1,32,'위치·기술',[
 site('C049',0,26,'mem:rbp:0x548',calc('unseen_risk_divisor'),bytes.fromhex(rows['C049']['windows'][0]['bytes'])+asm(result('r14')),notes='원래 RAX/RDX와 flags 재현 후 R14 몫만 교체. 직후 e649ea TEST CL,CL가 flags 덮음.')]))
out.append(tactic('C050','battle_reposition_distance_scale','교전 재배치 거리 폭(%)','적·자기 중심의 재배치 후보 거리5개를 비례 조정한다. 높을수록 넓은 거리대에서 위치를 찾는다',100,75,125,25,200,'위치·기술',[
 site('C050',0,7,'mem:rbp:0x430',calc('relocate_inner'),op('sub','rcx')),
 site('C050',1,7,'mem:rbp:0x430',calc('relocate_mid'),op('add','rcx')),
 site('C050',2,6,'mem:rbp:0x430',calc('relocate_inner'),op('sub','rax')),
 site('C050',3,8,'mem:rbp:0x430',calc('relocate_mid_result'),result('rax')),
 site('C050',4,8,'mem:rbp:0x430',calc('relocate_outer_result'),result('rcx')),
 site('C050',5,11,'mem:rbp:0x430',calc('relocate_self_near'),'push r11; mov r11,qword ptr [rsp-8]; mov qword ptr [rbp+0x120],r11; pop r11'),
 site('C050',6,11,'mem:rbp:0x430',calc('relocate_self_far'),'push r11; mov r11,qword ptr [rsp-8]; mov qword ptr [rbp+0x138],r11; pop r11')]))
out.append(tactic('C051','candidate_move_budget_margin','공격·기술 후보 탐색 이동 여유','기본 공격·첫 두 기술의 후보 수집에 이동 예산×이 값을 더한다. 실제 기술 사거리·효과 상한은 유지하며 초 단위가 아니다',30,15,60,0,120,'위치·기술',[
 site('C051',k,11,'mem:rbp:0x190',calc('candidate_move_budget'),'lea rcx,[rax+rax]; mov rax,qword ptr [rsp-16]; sub rax,rcx',notes='M의 원래 상대속도 차/0하한을 유지. RCX=2M과 마지막 SUB의 carry/overflow를 정확히 보존; min(base,cap) 뒤 여유.') for k in range(2)]))
out.append(tactic('C052','forward_damage_credit_percent','전진 후 피해 상쇄 비율(%)','이동 후 위험에서 현재 위치 피해의 이 비율을 빼고0하한을 적용한다. 높을수록 전진을 덜 위험하게 본다',50,25,75,0,100,'위치·기술',[
 site('C052',0,3,'mem:rbp:0x548',calc('forward_credit_percent'),result('rdi'),notes='원래 뒤 XOR ESI/SUB RAX,RDI를 재실행해 borrow와0하한 CMOVAE 보존.')]))
out.append(tactic('C053','battle_move_candidate_penalty','교전 중 이동후보 감점','원래 조건을 만족한 이동후보에만 주는 감점. 높을수록 공격·기술에 비해 이동후보를 덜 고른다',6,0,12,0,1000,'전투·생존',[
 site('C053',0,7,'mem:rbp:0x220',calc('move_penalty_negative'),'sub rsi,r14; '+op('add','rsi'),notes='SUB 양수가 아니라 ADD 음수로 재현해 원래 carry/AF까지 보존.'),
 site('C053',1,4,'mem:rbp:0xf0',calc('move_penalty_result'),result('r14'),flags_mask='0xcc5')]))
assert len(out)==21 and {x['audit_id'] for x in out}==set(rows)
target={'exe_sha256':data['exe_sha256'],'tactics':out}
p=MOD/'tools/targets/extra_combat_0.6.5.json';p.write_text(json.dumps(target,ensure_ascii=False,indent=2)+'\n',encoding='utf-8',newline='\n')
print(json.dumps({'tactics':len(out),'sites':sum(len(t['sites']) for t in out),'kept_original_reference_windows':sum(len(t['reference_windows']) for t in out),'file':str(p)},ensure_ascii=False))
