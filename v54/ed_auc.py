# -*- coding: utf-8 -*-
"""편집기에 경매 강제귀환 12노브 + 판단14(넥서스공격) 6노브 등록."""
import io
P = r'C:\tfm2mods\ai_adjust_editor\src\main.rs'
t = io.open(P, encoding='utf-8').read()
assert 'auc_flee_score' not in t

# ── ① 원본값 맵 ──────────────────────────────────────────────
ORIG = ''' // ★[0.5.4 신설] 경매 중 강제 귀환
 "auc_flee_version_gate" => "1", "auc_flee_undying_gate" => "0",
 "auc_flee_hp_field" => "1624", "auc_flee_nexus_mask" => "256",
 "auc_flee_goal_far" => "928000", "auc_flee_goal_near_a" => "32000",
 "auc_flee_goal_near_b" => "32000", "auc_flee_end_delay" => "5",
 "auc_flee_pathfinder" => "2", "auc_flee_with_skill" => "1",
 "auc_flee_score" => "99999", "auc_flee_action_tag" => "3",
 // ★[0.5.4 신설] 판단14 넥서스 공격
 "an_home_wait" => "7", "an_tower_gate" => "0", "an_fallback" => "2",
 "an_attack_sub" => "18", "an_fallback_wave" => "2", "an_fallback_style" => "0",
 // 개별 시야창
'''
t = t.replace(' // 개별 시야창\n', ORIG, 1)

# ── ② 탭 ────────────────────────────────────────────────────
TAB = ''' Tab{ id:"nexus_auction", title:"• [0.5.4] 넥서스 공격 · 경매 강제귀환", keys:&[
 "§◆ 넥서스로 밀어붙일지 정하는 단계","an_tower_gate","an_attack_sub","an_home_wait","an_fallback","an_fallback_wave","an_fallback_style",
 "§◆ 경매 중 강제 귀환 — 켜고 끄기","auc_flee_version_gate","auc_flee_score","auc_flee_hp_field","auc_flee_nexus_mask",
 "§◆ 경매 중 강제 귀환 — 어디로 어떻게","auc_flee_goal_far","auc_flee_goal_near_a","auc_flee_goal_near_b","auc_flee_end_delay","auc_flee_with_skill","auc_flee_action_tag","auc_flee_pathfinder","auc_flee_undying_gate",], note:
 "<b>0.5.4에서 새로 생긴 두 판단</b>입니다. 둘 다 이전 버전엔 없었고, 지금까지 손댈 수 있는 노브가 하나도 없던 자리입니다.<br>\
 <b>넥서스 공격</b>: 적 타워가 몇 개 남았을 때 넥서스로 밀어붙일지, 아직 이르면 대신 무엇을 할지를 정합니다.<br>\
 <b>경매 중 강제 귀환</b>: 경매(전술 입찰)가 도는 동안 \\"지금 도망치면 맞을 피해를 못 견딘다\\"고 보이면 <b>다른 모든 판단을 제치고</b>(점수 99999) 기지 코너로 물러납니다.<br>\
 ⚠<b>이 강제 귀환 전체가 `auc_flee_version_gate` 아래에 있습니다.</b> 게임 내부의 AI 사양 버전이 이 값보다 커야 발동하는데, 그 버전의 실제 값은 아직 확인되지 않았습니다. <b>0으로 낮추면 항상 켜지고</b>, 크게 올리면 완전히 잠급니다.<br>\
 ⚠<b>전부 -1(원본)이 기본</b>이라 그냥 두면 게임과 완전히 같습니다.", },
'''
i = t.index(' Tab{ id:"planpick",')
t = t[:i] + TAB + t[i:]

# ── ③ 설명 ──────────────────────────────────────────────────
D = {
 "auc_flee_version_gate": "경매 중 강제 귀환의 <b>주 스위치</b>(원본 1). 게임 내부의 AI 사양 버전이 이 값보다 커야 발동합니다. 그 버전의 실제 값은 아직 확인되지 않아, 지금 이 판단이 켜져 있는지 자체가 미확인입니다. <b>0으로 낮추면 버전과 무관하게 항상 켜집니다.</b> 크게 올리면 완전히 잠급니다. 0~127. -1=원본",
 "auc_flee_undying_gate": "불사 상태 특례 판정값(원본 0). 불사면 거리 계산을 건너뛰고 '기지 근처'로 칩니다. ⚠발동 여부에는 영향이 없고 기록용 분기에만 쓰입니다 — 사실상 만질 일이 없습니다. -1=원본",
 "auc_flee_hp_field": "'도망 도중 맞을 피해'를 <b>무엇과 비교할지</b>(원본 1624 = 현재 체력). 1552로 바꾸면 최대 체력과 비교하므로 훨씬 드물게 발동합니다. ⚠값이 아니라 읽을 자리를 바꾸는 노브라, 이 두 값 외에는 넣지 마세요. -1=원본",
 "auc_flee_nexus_mask": "'우리 넥서스가 실제로 맞는 중이면 도망 취소' 조건(원본 256). 0이면 취소 조항이 사라져 넥서스가 깨지는 중이어도 물러납니다. 65537로 넓히면 더 자주 취소됩니다. -1=원본",
 "auc_flee_goal_far": "도망 목적지의 먼 쪽 좌표(원본 928000). 맵 한 변이 약 960000이라 사실상 맵 끝, 즉 기지 코너입니다. 줄이면 덜 깊이 물러납니다. -1=원본",
 "auc_flee_goal_near_a": "도망 목적지의 가까운 쪽 좌표(원본 32000). ⚠<b>`auc_flee_goal_near_b`와 반드시 같은 값으로 바꾸세요</b> — 한쪽만 바꾸면 팀 사이드에 따라 목적지가 달라집니다. -1=원본",
 "auc_flee_goal_near_b": "위와 같은 값의 반대 팀 사이드용 사본(원본 32000). ⚠<b>`auc_flee_goal_near_a`와 항상 같이</b>. -1=원본",
 "auc_flee_end_delay": "기지 코너에 도착한 뒤 이 명령을 몇 틱 더 붙잡고 있을지(원본 5). 크게 하면 더 오래 웅크리고, 0이면 도착 즉시 다음 판단으로 넘어갑니다. -1=원본",
 "auc_flee_pathfinder": "경로탐색 사용 여부(원본 2 = 사용 안 함). ⚠<b>바꾸지 마세요</b> — 2 외의 값은 초기화되지 않은 경로탐색 자료를 읽습니다. -1=원본",
 "auc_flee_with_skill": "도망치는 동안 무엇을 허용할지(원본 1 = 스킬만). 바이트 단위로 스킬·궁·궤적 회피·목표 확정이 켜집니다. 0=아무것도 안 씀, 257=스킬+궁. -1=원본",
 "auc_flee_score": "이 강제 귀환에 매기는 점수(원본 99999 = 사실상 무조건 1위라 다른 판단을 전부 이깁니다). 낮추면 다른 판단과 <b>경쟁</b>하게 되어, 정말 급할 때만 채택되게 만들 수 있습니다. -1=원본",
 "auc_flee_action_tag": "강제 귀환이 실제로 내리는 행동(원본 3 = 도주). ⚠<b>태그만 바꾸면 딸려가는 값들의 자리가 안 맞아 오작동합니다</b> — 3 외에는 권장하지 않습니다. -1=원본",
 "an_tower_gate": "넥서스를 직접 치러 갈 수 있는 <b>적 타워 잔여 수</b>(원본 0 = 전부 밀어야 넥서스로 갑니다). ⚠비교가 '정확히 N개'라 값을 올리면 '딱 그 수일 때만' 넥서스로 갑니다. -1=원본",
 "an_attack_sub": "넥서스를 칠 때 실제로 수행할 하위 판단(원본 18). -1=원본",
 "an_home_wait": "넥서스로 갈 조건이 안 됐고 분수에서 대기할 때의 하위 판단(원본 7 = 귀환). -1=원본",
 "an_fallback": "타워가 남아 넥서스로 못 갈 때 대신 할 판단(원본 2 = 라인 방어). 값을 바꾸면 마무리 국면의 성향이 통째로 바뀝니다. -1=원본",
 "an_fallback_wave": "위 폴백에서의 미니언 웨이브 성향(원본 2, <b>추정</b> = 밀기). -1=원본",
 "an_fallback_style": "위 폴백의 스타일 바이트(원본 0). 의미 미확정. -1=원본",
}
anc = ' "pl_obj_role" => "에픽'
i = t.index(anc)
ins = ''.join(' "%s" => "%s",\n' % (k, v) for k, v in D.items())
t = t[:i] + ins + t[i:]

io.open(P, 'w', encoding='utf-8', newline='\n').write(t)
print('편집기 등록 완료: 경매 12 + 판단14 6 = 18노브, 새 탭 nexus_auction')
