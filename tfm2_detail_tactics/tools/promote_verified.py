"""검증한 12후보의 계약을 정식 원천표로 옮긴다. 과거 QA 자료는 변경하지 않는다."""
import copy, hashlib, json, re
from pathlib import Path
ROOT = Path(__file__).resolve().parent.parent
REPORT = Path(r'C:\Users\jungs\Desktop\claude\tfm2\mods_report\tfm2_detail_tactics')
SOURCE = REPORT/'runtime_candidates_20261025/qa_manifest.json'
QA = Path(r'C:\tfm2mods\_qa_detail_candidates_20261025/src/qa_candidates.rs')
# 감사ID, 저장ID, 탭, 설명. 수치·주체·기계어는 검증된 원천을 유지한다.
SELECTED = [
 ('R01','purchase_recall_walk_ticks','라인·이동','구매를 위한 귀환 판단의 도보시간 문턱입니다. 높을수록 걸어서 돌아갈 수 있는 시간 범위를 넓힙니다. 초 단위가 아닙니다.'),
 ('R02','purchase_recall_threat_radius','라인·이동','구매 귀환 판단에서 적을 위협으로 인정하는 반경입니다. 높을수록 먼 적까지 살핍니다. 원래 거리·활동 조건은 유지합니다.'),
 ('R03','purchase_recall_threat_memory','라인·이동','구매 귀환 판단에서 최근 본 적을 위협으로 기억하는 기간입니다. 높을수록 오래 기억합니다. 초 단위가 아닙니다.'),
 ('LN7-01','same_lane_partner_radius','라인·이동','같은 라인 동료가 가까이 있다고 인정하는 반경입니다. 높을수록 먼 동료까지 인정하며, 거리 밖의 별도 승인 조건은 유지합니다.'),
 ('LN10-01','support_zero_net_penalty','라인·이동','지원 자격을 만족하고 이동 비용을 뺀 지원 평가가 정확히 0일 때 적용하는 감점입니다. 높을수록 그 지원 후보를 덜 고릅니다.'),
 ('J03','camp_deadline_arrival_seconds','정글','일반 캠프의 기한과 도착 시간을 비교할 때 더하는 여유 시간(초)입니다. 높을수록 기한 내 도착을 엄격히 따집니다.'),
 ('G6-01','enemy_base_allies_radius','갱·로밍','존재하는 아군 전원이 상대 기지에 가까운지 판단하는 반경입니다. 현재 판단 선수의 설정을 쓰며 높을수록 넓게 인정합니다.'),
 ('OP10-01','ambush_wait_accept_radius','갱·로밍','매복 시작·재판정의 원래 위치 조건 안에서 대기를 인정하는 반경입니다. 높을수록 넓게 인정하며 적 탐지 반경은 유지합니다.'),
 ('R8C01','tracked_target_chase_bonus','전투·생존','등록된 추적 대상이 사거리 밖에 있을 때 추격 후보에 더하는 점수입니다. 높을수록 그 대상을 더 쫓습니다.'),
 ('R9C01','champion_ultimate_waste_penalty','전투·생존','원래 낭비 검사 조건을 만족한 챔피언 대상 궁극기 후보의 감점입니다. 높을수록 해당 궁극기 사용을 아낍니다. 실제 위력은 유지합니다.'),
 ('L02','lane_opportunity_enemy_limit','라인·이동','라인 기회를 검토할 때 허용하는 상대 인원 상한입니다. 높을수록 상대가 더 많아도 기회 후보를 검토합니다.'),
 ('DC01','skill_arrival_prediction_margin','위치·기술','기술 도달 시점을 예측할 때 더하는 시간 여유입니다. 높을수록 예측 여유가 커집니다. 초 단위가 아니며 기술 속도·재사용 대기시간은 유지합니다.'),
]
def main():
    manifest=json.loads(SOURCE.read_text(encoding='utf-8')); rows=[]
    for n,(audit,id,tab,desc) in enumerate(SELECTED,1):
        old=next(t for t in manifest['tactics'] if t['audit_id']==audit)
        t=copy.deepcopy(old)
        t.update(audit_id=f'N{n:03}',verified_candidate_id=audit,id=id,tab=tab,desc=desc,pos_mask=2 if audit=='J03' else 31,
                 runtime_verified=True,qa_only=False,status='promoted_after_replay_comparison')
        t['knob']='extra_'+t['audit_id'].lower()
        for site in t['sites']:
            if 'calc' in site['tf']:site['tf']['calc']=site['tf']['calc'].replace('qa_candidates::','verified_candidates::')
        rows.append(t)
    out={'game_version':'0.6.5','exe_sha256':manifest['exe_sha256'],
         'source_sha256':hashlib.sha256(SOURCE.read_bytes()).hexdigest(),
         'evidence':['runtime_candidates_20261025/analyst_conclusion.md','runtime_candidates_second_20261025/analyst_conclusion.md'],
         'tactics':rows}
    (ROOT/'tools/targets/extra_verified_0.6.5.json').write_text(json.dumps(out,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    # 실측한 QA와 동일한 변환만 추출한다.
    source=QA.read_text(encoding='utf-8')
    names=['negative','square_shift8','ops_valid','ops_radius_squared','ops_camp_tps','ops_gather_square','combat_rsi_add','combat_negative']
    callbacks=['//! 다시보기에서 변화가 확인된 후보의 읽기 전용 변환. 수치·주체 검사를 보존한다.','use crate::micro::{Frame,Src};']
    for name in names:
        m=re.search(r'(?:pub(?:\(crate\))? )?fn '+name+r'\(',source);assert m,name
        start=m.start(); brace=source.index('{',m.end());end=brace+1;depth=1
        while depth:
            depth+=(source[end]=='{')-(source[end]=='}');end+=1
        callbacks.append(source[start:end])
    (ROOT/'src/verified_candidates.rs').write_text('\n\n'.join(callbacks)+'\n',encoding='utf-8')
    print('12 verified policies /',sum(len(t['sites']) for t in rows),'windows')
if __name__=='__main__':main()
