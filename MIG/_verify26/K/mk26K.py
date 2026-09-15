# -*- coding: utf-8 -*-
u"""26차 배치K patch.json 생성 — mkpatch 참조구현 경유. 실행: python -X utf8 _verify26/K/mk26K.py"""
import sys, io
sys.path.insert(0, r"C:\tfm2mods\MIG")
import mkpatch

p = mkpatch.Patch(round=26, batch="K")
ORC = u"오라클 _verify26/K/oracle/oK.rs (케이스당 프로세스 1개 · 로그 oK_cases.log)"

# ───────────── 234 JungleSubPlan::score ─────────────
p.fix("/specs[234]/one_line",
      old=u"interaction_score 기반 + Attack/Skill/Skill2 는 calculate_jungle_action_score 가산, 대상이 (팀)정글몹이면 최소 1 보장",
      new=u"interaction_score 기반 · Attack 은 base+calculate_jungle_action_score, ★Skill/Skill2 는 calculate_jungle_action_score 값으로 base 를 대체(base 미가산), 대상이 정글몹(is_jungle(1))이면 최소 1 보장",
      evidence=u"m02.ll:40038 `%71 = add i64 %70, %24`(Attack 만 base 가산) vs 40081 `%91 = tail call …calculate_jungle_action_score` 직후 add 없이 phi 40065 `[ %91, %88 ]`·`[ %91, %96 ]`·smax(%91,1) / Skill2 40124 `%114` 동일. 줄길이 산술: jungle.rs L159 151자 − L171 142자 = 9 = `base + `(7) + attack→skill(2). " + ORC + u" sc_skill_enemy: interaction_score=-1 인데 jungle_score=0(=calculate_jungle_action_score 값) · sc_attack_enemy: jungle_score=-1(=base+0)",
      behavior_change=True, found_by="new")
p.fix("/specs[234]/logic",
      old=u"Skill{target_id} => {   // idx 13 · L170~L173 동형: &champ.skill(+0x580) · skill_effect(+0x4f8/+0x4c8)\n        Some(t) → score = base + calculate_jungle_action_score(..., &champ.skill, skill_effect.unwrap(), t) /*L171*/;",
      new=u"Skill{target_id} => {   // idx 13 · L170~L173 · &champ.skill(+0x580) · skill_effect(+0x4f8/+0x4c8) · ★Attack 과 달리 base 미가산\n        Some(t) → score = calculate_jungle_action_score(..., &champ.skill, skill_effect.unwrap(), t) /*L171 · IR 40081 %91 이 add 없이 phi/smax 로 직행 · 오라클 sc_skill_enemy 확인*/;",
      evidence=u"m02.ll:40081~40087 — %91 호출 결과가 그대로 phi(40065 `[ %91, %88 ]`,`[ %91, %96 ]`)·`%101 = smax(%91, 1)` 로 감. Attack arm 40038 `add i64 %70, %24` 만 base 가산. " + ORC,
      behavior_change=True, found_by="new")
p.fix("/specs[234]/logic",
      old=u"score = base + calculate_jungle_action_score(..., act, eff.as_ref().unwrap() /*level≤2 면 여기서 unwrap_failed*/, t) /*L183*/;",
      new=u"score = calculate_jungle_action_score(..., act, eff.as_ref().unwrap() /*level≤2 면 여기서 unwrap_failed*/, t) /*L183 · ★base 미가산 — IR 40124 %114 가 add 없이 phi 40065 `[ %114, %111 ]`·smax(%114,1)*/;",
      evidence=u"m02.ll:40124~40130 · phi 40065 `[ %114, %111 ]`,`[ %114, %119 ]` · 40146 `smax(%114, 1)` — %24(base) 와의 add 는 Attack arm(40038) 에만 있다",
      behavior_change=True, found_by="new")
p.fix("/specs[234]/logic",
      old=u"if t.is_jungle() /*L160 · ty@tag==4 && ty.info.camp_type.0(+0x98) < 2*/",
      new=u"if t.is_jungle(1) /*L160 · is_jungle(&EntityType, team: usize)->bool(tcx entity.rs:1377) · 호출 인자 리터럴 1(DI `team = i64 1` m02.ll:39912~39914) · 인라인 = ty@tag==4 && ty.info.camp_type.0(+0x98) <= team → IR `ult 2` 로 접힘*/",
      evidence=u"m02.ll:39912~39914 `#dbg_value(i64 1, !46553 …)` ×3 (!46553 = DILocalVariable name:\"team\" scope:!46554 is_jungle · inlinedAt L160/172/184) · _tcx game_core.json is_jungle sig `fn(&EntityType, usize) -> bool` · rmeta_srcmap entity.rs:1377 = 48자 = `  pub fn is_jungle(&self, team: usize) -> bool {`",
      behavior_change=False, found_by="new")
p.fix("/specs[234]/knobs[1]/where",
      old=u"entity.rs:1378 (jungle.rs:160 인라인)",
      new=u"jungle.rs:160/172/184 `t.is_jungle(1)` 의 인자 리터럴 1 (entity.rs:1378 `camp_type.0 <= team` 인라인 → IR m02.ll:40054/40097/40140 `icmp ult i64 %78, 2` 로 접힘)",
      evidence=u"DI `team = i64 1` ×3 (m02.ll:39912~39914 · scope is_jungle · inlinedAt jungle.rs:160/172/184) · 같은 크레이트의 다른 호출부(m02.ll:37133~37146 · m14.ll:38884/42007)도 전부 1",
      behavior_change=False, found_by="new")
p.fix("/specs[234]/knobs[1]/effect",
      old=u"단 이 값은 game_core 헬퍼 안이라 이 함수에서 못 바꿈",
      new=u"★값의 출처는 이 함수의 호출 인자 `is_jungle(1)`(team=1 → `camp_type.0 <= 1` = IR `< 2`)라 여기서 바꿀 수 있다(인자 0 이면 camp_type.0==0 만 · 2 이면 ≤2). camp_type.0 의 의미(CampDef.pos[2] 진영 인덱스 추정)는 미확정",
      evidence=u"m02.ll:39912~39914 DI team=1 · tcx is_jungle(&EntityType, usize)->bool",
      behavior_change=False, found_by="new")
p.fix("/specs[234]/consts[6]/meaning",
      old=u"is_jungle 둘째 조건: t.ty.info.camp_type.0 < 2 (entity.rs:1378 인라인).",
      new=u"is_jungle 둘째 조건: t.ty.info.camp_type.0 < 2 (entity.rs:1378 인라인 · ★리터럴 2 는 호출 인자 `is_jungle(team=1)` 의 `camp_type.0 <= team` 이 접힌 값 — DI team=1 ×3).",
      evidence=u"m02.ll:39912~39914 · 40054 `icmp ult i64 %78, 2 ;L1378<160`", behavior_change=False, found_by="new", kind=u"보강")
p.fix("/specs[234]/open[0]",
      old=u"entity.rs:1378 is_jungle 의 `camp_type.0 < 2` 가 무슨 분류인지(팀 진영 0/1 vs 중립 2? · 캠프 등급?) — game_core 헬퍼 인라인만 보임. 미탐색 = _gcbc 의 is_jungle define/주석 · Jungle::camp_type 생성처.",
      new=u"entity.rs:1378 is_jungle 의 `camp_type.0 < 2` 가 무슨 분류인지 — 26차: 시그니처 fn(&EntityType, team: usize)->bool 이고 호출 인자 1 이 `camp_type.0 <= team` 에 접혀 `<2` 가 됐다(DI team=1 ×3). 남은 물음 = camp_type: (usize, JungleType) 의 .0 의미(CampDef.pos: [(u64,u64);2] 진영 인덱스 추정 · 미탐색 = _gcbc Jungle 스폰처의 camp_type.0 store · 그 값이 0/1 뿐이면 is_jungle(1) ≡ tag==4).",
      evidence=u"tcx game_core::EntityType::is_jungle sig · m02.ll:39912~39914 · tcxdict CampDef/Jungle", behavior_change=False, found_by="new", kind=u"보강")
p.ev("/specs[234]/consts[4]", to=2, evidence=u"오라클 실행 확인: sc_attack_missing/sc_skill_missing(target 999999) → jungle_score=-99999 · IR phi 40065 `[ -99999, %33 ]` 직행(base 미가산 형태는 base=0 이라 실행으론 미분리 · IR 확정)", found_by="new")
p.ev("/specs[234]/consts[7]", to=4, frm=4, evidence=u"IR 40060 `llvm.smax.i64(i64 %71, i64 1)` — 오라클은 jungle 개체(cache.jungles=0)가 없어 미실행", found_by="new")
p.ev("/specs[234]/mem[7]", to=3, evidence=u"tcxdict --enum SmallActionPlay: Attack/Skill/Skill2 페이로드(SmallActionAttack 등) +0x8 target · IR gep %6, 8 (40034/40044/40054)", found_by="new")

# ───────────── 235 AttackNexusSubPlan::score ─────────────
p.fix("/specs[235]/consts[9]/meaning",
      old=u"Around arm 값 0 — get_entity_by_id 를 호출하지만 결과(t)를 읽지 않고 0 (Some/None 양쪽 0)",
      new=u"Around arm 값 0 — get_entity_by_id 를 호출하지만 결과(t)를 읽지 않고 0 (Some/None 양쪽 0). ★G12 오탐: 이 0 은 m14.ll:21118 `%73 = phi i64 [ … [ 0, %33 ] …]`(phi 상수 인입 · !dbg 없음)이고 인입 블록 %33 의 명령은 21069 call get_entity_by_id `!dbg` L201 · 21070 br L210 이라 루트 줄 201 이 맞다. 게이트 후보 178/193 은 tower_bonus `select i1 %92, i64 3, i64 0`(21158/21180) 의 다른 상수 0 이다",
      evidence=u"m14.ll:21060~21070 (블록 %33: L0<164 gep · L201 call · L210 br) · 21118 phi(!dbg 없음)",
      behavior_change=False, found_by="reused", kind=u"오탐")
p.fix("/specs[235]/logic",
      old=u"Around{target_id} /*Play idx 2/3/10*/",
      new=u"Around{target_id} /*Play idx 2 Around · 3 AroundHide · 10 LaneMinionPosition — get_action 이 SmallAction::Around{target_id} 를 내는 셋(tcxdict --enum SmallActionPlay) · AroundBush 는 idx 9 로 _ arm*/",
      evidence=u"tcxdict --enum game_ai::SmallActionPlay: idx 9 AroundBush(태그 12) · idx 10 LaneMinionPosition(태그 13) · switch m14.ll:21036/21037/21044 (i8 2,3,10 → %33)",
      behavior_change=False, found_by="new", kind=u"보강")
p.ev("/specs[235]/consts[5]", to=2, evidence=u"오라클 실행 확인: sc_attack_missing/sc_skill_missing → attacknexus_score = base(0) + (-99999) = -99999 (IR 21119 `add i64 %73, %24` 로 base 합산)", found_by="new")
p.ev("/specs[235]/consts[4]", to=2, evidence=u"오라클 실행 확인: sc_attack_enemy/sc_skill_enemy 에서 calculate_action_score(…, MinionActionType::Push, …) 직접 호출값 + base + tower_bonus(0) == attacknexus_score (-1 == -1)", found_by="new")

# ───────────── 236 v15_can_keep_support_pressure ─────────────
p.fix("/specs[236]/open[1]",
      old=u"blackboard 인덱스가 `1 - player.info.team`(적팀 번호)인 의미 — Blackboard 배열이 '팀별 관측 보드'인지 '상대팀에 대한 보드'인지는 game_core 소관(is_recent_visible 내부 미독해)",
      new=u"blackboard 인덱스 `1 - player.info.team` 의 의미 — 26차 해소: Blackboard[T] 는 「T 팀(의 챔피언들)에 대한 보드」(rmeta 주석 `Blackboard[team]은 team 팀 자체 정보 추적, 관측은 1-team`)이고 is_recent_visible(g07.ll:157005~157049 blackboard.rs:348~350) = game.is_visible(player.team, target.id)(vtable+0xf8) || (target 이 플레이어 챔프이고 self.last_visible[target.position](+0x1e0)+120 >= game.tick()). 따라서 blackboard[적팀] 은 내 팀이 관측한 적 챔프의 최근 가시 tick 표다. 오라클 v15_notvis(last_visible=0)=false / v15_near(last_visible=tick)=true",
      evidence=u"_gcbc/g07.ll:157005~157049 · _docs/game_core.txt:21 · " + ORC,
      behavior_change=False, found_by="new", kind=u"보강")
p.fix("/specs[236]/open[0]",
      old=u"L16 `||` 두 항의 소스 순서(hp<45 vs applyed_cc≠0) — column 정보 부재 + 둘 다 순수 load 라 IR 에 흔적 없음(표기 불가·외연 동일)",
      new=u"L16 `||` 두 항의 소스 순서(hp<45 vs applyed_cc≠0) — 표기 불가·외연 동일. 단 IR 힌트: hp 항(57801~57810)은 !dbg L16 이 있고 applyed_cc 로드(57811~57813)는 !dbg 가 없는 호이스트(SimplifyCFG 가 둘째 블록을 select 로 접은 흔적) 이며 `select i1 %33, i1 true, i1 %36` 의 첫 피연산자가 hp 항 → 소스 순서 hp<45 || cc≠0 과 정합(추정)",
      evidence=u"m05.ll:57811~57814 (applyed_cc gep/load/icmp 에 !dbg 없음 · select 첫 피연산자 %33=hp<45)", behavior_change=False, found_by="new", kind=u"보강")
p.ev("/specs[236]/consts[2]", to=2, evidence=u"오라클 실행 확인: v15_hp44(hp 44/100)=false · v15_hp45=true — 경계 `hp*100/max(stat_hp,1) < 45`", found_by="new")
p.ev("/specs[236]/consts[3]", to=2, evidence=u"오라클 실행 확인: v15_dmg19(applyed_damage 19 · hp 100)=true · v15_dmg20=false — 경계 `dmg*100 < max(hp,1)*20`", found_by="new")
p.ev("/specs[236]/consts[5]", to=2, evidence=u"오라클 실행 확인: 적 대상 거리 100000(v15_ally? 아님 · v15_near 50000)=true · 200000(v15_far)=false · 아군 대상 분기 enemy↔champ 190000(v15_ally4)=false / 100000(v15_ally3)=true", found_by="new")
p.ev("/specs[236]/consts[6]", to=2, evidence=u"오라클 실행 확인(아군 대상 분기): enemy↔ally 130000(v15_ally2)=false · 110000(v15_ally3)=true · 20000(v15_ally)=true", found_by="new")
p.ev("/specs[236]/knobs[0]", to=2, evidence=u"오라클 실행 확인 v15_hp44=false/v15_hp45=true", found_by="new")
p.ev("/specs[236]/knobs[1]", to=2, evidence=u"오라클 실행 확인 v15_dmg19=true/v15_dmg20=false", found_by="new")
p.ev("/specs[236]/knobs[2]", to=2, evidence=u"오라클 실행 확인 v15_ally2(130000)=false/v15_ally3(110000)=true", found_by="new")
p.ev("/specs[236]/knobs[3]", to=2, evidence=u"오라클 실행 확인 v15_far(200000)=false/v15_near(50000)=true/v15_ally4(190000)=false", found_by="new")
p.ev("/specs[236]/mem[13]", to=3, evidence=u"tcxdict ScoreParameter: player@0x918 + ChampionScoreParameter.applyed_cc@0x78 = 0x990 · 오라클 v15_cc(=1)=false", found_by="new")
p.ev("/specs[236]/mem[14]", to=3, evidence=u"tcxdict ScoreParameter: player@0x918 + applyed_damage@0x70 = 0x988 · 오라클 v15_dmg19/20", found_by="new")
p.ev("/specs[236]/mem[3]", to=3, evidence=u"tcxdict Blackboard 744B · last_visible@0x1e0 · 오라클 bb[1].last_visible[0]=tick 으로 true/0 으로 false", found_by="new")

# ───────────── 237 RecallSubPlan::score ─────────────
p.fix("/specs[237]/consts[2]/kind", old=None, new=u"계수",
      evidence=u"m02.ll:40500 `udiv i64 %9, 3` — 제수(감쇠 계수)이지 태그가 아니다(G15 관측 ARITH)", behavior_change=False, found_by="new", kind=u"보강", force=True)
p.fix("/specs[237]/consts[3]/kind", old=None, new=u"계수",
      evidence=u"m02.ll:40368 `mul i64 %9, 3` — 배수", behavior_change=False, found_by="new", kind=u"보강", force=True)
p.fix("/specs[237]/consts[8]/kind", old=None, new=u"인덱스",
      evidence=u"m02.ll:40458/40487 `gep %58, i64 128` — vtable 슬롯 바이트 오프셋(슬롯 인덱스 16)", behavior_change=False, found_by="new", kind=u"보강", force=True)
p.ev("/specs[237]/consts[0]", to=2, evidence=u"오라클 실행 확인: sc_runaway/sc_recall base(interaction_score)=-99999 → recall_score=-99949(=base+50) · rnd 320B 상태가 interaction_score 단독 호출 후와 동일(gen_range 사이트 0 확인)", found_by="new")
p.ev("/specs[237]/consts[3]", to=2, evidence=u"오라클 실행 확인: sc_attack_enemy/sc_skill_enemy base=-1 → recall_score=-3(=base*3) · sc_stop/sc_trace base=0 → 0", found_by="new")
p.ev("/specs[237]/consts[4]", to=2, evidence=u"오라클 실행 확인: base=-1 → ×3 경로(-3) · base=0 → 0 (sgt 0 거짓 쪽 실행 · base>0 케이스는 interaction_score 가 양수를 내지 않아 미실행)", found_by="new")
p.ev("/specs[237]/knobs[0]", to=2, evidence=u"오라클 실행 확인 sc_runaway/sc_recall: +50", found_by="new")
p.ev("/specs[237]/knobs[3]", to=2, evidence=u"오라클 실행 확인 sc_attack_enemy: base -1 → -3", found_by="new")
p.ev("/specs[237]/mem[0]", to=3, evidence=u"tcxdict --enum SmallActionPlay: 판별자 enum+0xb1 니치(untagged=7 · niche_start=3) · 오라클 tag 3/4 → +50 · 19/14/15/16 → 그 외 arm", found_by="new")

# ───────────── 238 v22_current_line_non_champion_action_tower_risk ─────────────
p.fix("/specs[238]/mem[23]/name",
      old=u"ty@Tower.info.nearest_enemy@Some.0",
      new=u"ty@Tower.info.nearest_enemy@Some.0.1 (튜플 둘째 원소 = 엔티티 id)",
      evidence=u"tcxdict game_core::Entity 0x98 = `ty@Tower.info.nearest_enemy@Some.0.1` (Tower.nearest_enemy@0x18 → Entity 0x70+0x18=0x88 tag · .0@0x90 · .1@0x98). Tower::run_action(g09.ll:146884~146895/147109~147111 tower.rs:241/249/267) 이 +32(.0)에 cur_tick, +40(.1)에 entity id 를 store 하고 L193 이 +40 을 get_entity_by_id 에 넘긴다. " + ORC + u" v22_nearest(.1=champ.id)=true · v22_nearest0(.0=champ.id · .1=900)=false",
      behavior_change=True, found_by="new")
p.fix("/specs[238]/mem[23]/note",
      old=u"aux: 조준 대상 엔티티 id == champ.id 이면 즉시 true",
      new=u"aux(m11.ll:30050~30057 tower+152 == champ+1472): 조준 대상 엔티티 id(.1) == champ.id 이면 즉시 true · .0(+0x90) 은 그 대상을 잡은 tick(Tower::run_action `store cur_tick`) 으로 본문 미사용",
      evidence=u"m11.ll:30050~30057 · g09.ll:146884~146886 (`store i64 %25(cur_tick), ptr %180(+32)` · `store i64 %365(id), ptr %179(+40)`)",
      behavior_change=False, found_by="new", kind=u"보강")
p.fix("/specs[238]/logic",
      old=u"L249:       && nearest_enemy.0(+0x98) == champ.id(+0x5c0) { return true }",
      new=u"L249:       && nearest_enemy.1(+0x98 · 튜플 (잡은 tick, entity id) 의 id) == champ.id(+0x5c0) { return true }",
      evidence=u"tcxdict Entity 0x98 = nearest_enemy@Some.0.1 · 오라클 v22_nearest/v22_nearest0", behavior_change=True, found_by="new")
p.fix("/specs[238]/notes[0]",
      old=u"Tower.nearest_enemy: Option<(usize,usize)> 의 두 번째 usize 의미 미확정(본문에서 첫 원소만 사용)",
      new=u"Tower.nearest_enemy: Option<(usize,usize)> = (잡은 tick, 대상 entity id) — 본문은 둘째 원소(.1 · +0x98)를 champ.id 와 비교하고 첫 원소(.0 · +0x90 · Tower::run_action tower.rs:241/249/267 `store cur_tick`)는 읽지 않는다(26차 tcxdict+_gcbc+오라클 v22_nearest0 로 정정)",
      evidence=u"tcxdict Entity 0x98 · g09.ll:146884~146886 · " + ORC, behavior_change=False, found_by="new")
p.fix("/specs[238]/open[0]",
      old=u"필드 의미(비활성 시작 시각인지 종료 시각인지)는 game_core setting.rs 미독으로 미확정(미탐색 = `_gcbc` 에서 tower_attack_disable_tick 소비처 grep)",
      new=u"26차 해소(사실 서술 → notes 로): tower_attack_disable_tick 은 **비활성 시작 tick** — Tower::run_action(g09.ll:145954~ tower.rs:151~180) 이 tutorial∈{None,Line,Total} 이면 `cur_tick < setting.tower_attack_disable_tick`(+0x13f8) 일 때만 공격 로직(L188~)으로 가고 아니면 nearest_enemy 를 None 으로 지우고 끝낸다. 따라서 v22 의 `tick < disable_tick` = 「타워가 아직 공격하는 구간」 게이트로 극성이 정합(오라클 v22_disabled(=500<tick)=false). ⚠tutorial 1~4·6 은 `_2v2`(+0x1400) · 5 는 `_3v3`(+0x1408) 필드를 쓰는데 v22 는 기본 필드만 읽으므로 튜토리얼 모드에선 실제 비활성 tick 과 어긋난다(사실 서술)",
      evidence=u"_gcbc/g09.ll:146036~146072 (switch tutorial → +5120/+5128/+5112 · `icmp ult cur_tick, disable`) · 146074~146090 (아니면 nearest 0 store) · " + ORC,
      behavior_change=False, found_by="new", kind=u"보강")
p.ev("/specs[238]/mem[10]", to=3, evidence=u"tcxdict Entity 0x28 last_attacked_from@tag(Direct 8B) · 오라클 v22_hittower(=Some(tower id))=true / v22_hitchamp=false / v22_hitallytower=false", found_by="new")
p.ev("/specs[238]/mem[11]", to=3, evidence=u"오라클 v22_hittower/hitchamp/hitallytower — +0x30 의 id 로 get_entity_by_id 후 팀·타워 판정", found_by="new")
p.ev("/specs[238]/mem[22]", to=3, evidence=u"tcxdict Entity 0x88 = nearest_enemy@tag · 오라클 v22_nearest(tag 1)=true / v22_base(tag 0)=false", found_by="new")
p.ev("/specs[238]/mem[23]", to=3, evidence=u"tcxdict Entity 0x98 = nearest_enemy@Some.0.1 · 오라클 v22_nearest/v22_nearest0", found_by="new")
p.ev("/specs[238]/mem[4]", to=3, evidence=u"tcxdict GameSetting 0x13f8 tower_attack_disable_tick · 오라클 v22_disabled(500)=false vs 9999999 에서 v22_nearest=true", found_by="new")
p.ev("/specs[238]/knobs[1]", to=2, evidence=u"오라클 실행 확인 v22_disabled(tower_attack_disable_tick=500 < tick 1000)=false · =9999999 이면 같은 세계에서 true(v22_nearest)", found_by="new")
p.ev("/specs[238]/consts[7]", to=2, evidence=u"오라클 v22_hitallytower(last_attacked_from = 아군 타워)=false · v22_hittower(적 타워)=true — TeamType Player(1-team) 판정", found_by="new")
p.ev("/specs[238]/consts[8]", to=2, evidence=u"오라클 v22_outrange(타워 사거리 80000 밖 200000)=false · v22_nearest(20000)=true — attack_effect Some + is_in_range 게이트", found_by="new")

# ───────────── 239 SmallActionAroundBush::update_state ─────────────
p.fix("/specs[239]/open[0]",
      old=u"71 후보 셀이 맵 bushes 격자의 부쉬 셀 전체와 일치하는지(하드코딩 목록 vs 맵 데이터) — MapDef 인스턴스 값은 IR 밖(런타임/에셋)",
      new=u"71 후보 셀이 맵 bushes 격자의 부쉬 셀 전체와 일치하는지 — 26차 해소(적용 범위: MapDef::moba(real_setting)): bushes[y][x]!=0 인 셀이 정확히 71개이고 CANDIDATES71 집합과 **완전 일치**(차집합 양방향 0) · 부쉬 id 1..24 · id 당 셀 1~5개 ⟹ 유효 bush id 면 choose 후보 ≥1 이라 unwrap 패닉 경로는 이 맵에선 도달 불가. (다른 맵/모드는 미확인)",
      evidence=u"오라클 ab_map: `nonzero_cells=71 equal=true candidates_not_in_map=[] map_not_in_candidates=[] bush_ids=[1..24]` · @anon…94 1136B 디코드 = 명세 목록과 일치(cand71.py)",
      behavior_change=False, found_by="new", kind=u"보강")
p.ev("/specs[239]/mem[16]", to=3, evidence=u"오라클 ab_reselect: self 120B 전후 diff = qword {0x8, 0x18, 0x20} 정확히 3개 · target=(48000,16000)=(1*32000+16000, 0*32000+16000) · 셀 (1,0) 이 self.bush(1) 와 같은 부쉬", found_by="new")
p.ev("/specs[239]/mem[17]", to=3, evidence=u"오라클 ab_reselect 동일(0x20 · 16000=0*32000+16000)", found_by="new")
p.ev("/specs[239]/mem[18]", to=3, evidence=u"오라클 ab_reselect: change_tick 0 → 1067 = tick(1000)+67 (60..=120 안) · ab_hold(change_tick 5000 > tick) 는 3필드 모두 불변·rnd 0B 변화", found_by="new")
p.ev("/specs[239]/mem[19]", to=2, evidence=u"오라클 ab_debug(ctx.debug=true): dbg.lines 1개 = (32000, 928000, 48000, 16000, Color{1,1,0,1}) — 갱신 후 target 사용 · ab_reselect(debug=false) 는 0개", found_by="new")
p.ev("/specs[239]/consts[2]", to=2, evidence=u"오라클 ab_reselect target_x 48000 = 1*32000+16000", found_by="new")
p.ev("/specs[239]/consts[3]", to=2, evidence=u"오라클 ab_reselect target_y 16000 = 0*32000+16000", found_by="new")
p.ev("/specs[239]/knobs[1]", to=2, evidence=u"오라클 ab_reselect delta 67 ∈ 60..=120 (단일 표본 · 경계 미실측)", found_by="new")
p.ev("/specs[239]/knobs[2]", to=2, evidence=u"오라클 ab_map: 71 후보 == MapDef::moba bushes 비영 셀 71 (완전 일치)", found_by="new")
p.ev("/specs[239]/mem[10]", to=3, evidence=u"tcxdict MapDef 0x1c98 = bushes[0][0] · 오라클 ab_map/ab_reselect 가 map.bushes[y][x]==self.bush 로 선택 셀 검증", found_by="new")

# ───────────── 240 LineDefenseSubPlan::calculate_score_parameter_value ─────────────
p.fix("/specs[240]/sig/params[4]/role",
      old=u"IR 속성: `noalias noundef align 8 captures(none) dereferenceable(5384)` — readonly/readnone/initializes 없음 = &mut · 쓰기 표면은 writes 전수(자기 필드 2 + 힙 원소 2×N)",
      new=u"IR 속성: `noalias noundef align 8 captures(none) dereferenceable(5384)` 뿐(읽기전용·초기화 속성 없음 = &mut 쓰기 대상) · 쓰기 표면 = 자기 필드 2(0x9c0/0x9c8) + 힙 원소 2×N(near_allies/near_enemies 각 +0xa8/+0xb0) · 오라클 ld_*: 5384B 전후 diff 가 qword {0x9c0, 0x9c8} 뿐 · 힙 원소는 +0xa8/+0xb0 외 0",
      evidence=u"m14.ll:29736 define `ptr noalias noundef align 8 captures(none) dereferenceable(5384) %4` — G16 이 부정문(「readonly/readnone … 없음」)의 속성어를 주장으로 읽은 오탐 · 문면에서 속성어 제거. " + ORC,
      behavior_change=False, found_by="new", kind=u"보강")
p.ev("/specs[240]/consts[2]", to=2, evidence=u"오라클 실행 확인(6케이스 전부 일치): Defensive hp 49→70 · 50→70 · 51→50 (경계 `>50`) / Aggressive 49→50 · 50→30 · 51→30 (경계 `<50`)", found_by="new")
p.ev("/specs[240]/consts[3]", to=2, evidence=u"오라클 ld_1_49/ld_1_50 → attack/util_value 70", found_by="new")
p.ev("/specs[240]/consts[4]", to=2, evidence=u"오라클 ld_0_50/ld_0_51 → 30", found_by="new")
p.ev("/specs[240]/consts[5]", to=2, evidence=u"오라클 ld_1_49_2/ld_0_51_2: near_allies 2원소 +0xa8/+0xb0 = 50/50 · 나머지 214B 0 유지", found_by="new")
p.ev("/specs[240]/consts[6]", to=2, evidence=u"오라클 ld_1_49_2: near_enemies 2원소 = 50/50 (Defensive)", found_by="new")
p.ev("/specs[240]/consts[7]", to=2, evidence=u"오라클 ld_0_51_2: near_enemies 2원소 = 100/100 (Aggressive)", found_by="new")
for j in range(6):
    p.ev("/specs[240]/knobs[%d]" % j, to=2, evidence=u"오라클 ld_<style>_<ratio>_<n> 6케이스 진리표 전부 일치(oK_cases.log)", found_by="new")
for j in (11, 12):
    p.ev("/specs[240]/mem[%d]" % j, to=3, evidence=u"tcxdict ScoreParameter player@0x918 + attack_value@0xa8/util_value@0xb0 = 0x9c0/0x9c8 · 오라클 5384B diff = 이 두 qword 뿐", found_by="new")
for j in (13, 14, 15, 16):
    p.ev("/specs[240]/mem[%d]" % j, to=3, evidence=u"tcxdict ChampionScoreParameter 216B attack_value@0xa8 util_value@0xb0 · 오라클 힙 원소 2×2 관측(그 외 바이트 0)", found_by="new")
p.ev("/specs[240]/mem[0]", to=3, evidence=u"tcxdict LineStyle 1B Aggressive=0/Defensive=1 · 오라클 style 바이트 0/1 로 표가 갈림", found_by="new")
p.ev("/specs[240]/sig/params[1]", to=2, frm=3, evidence=u"오라클 ld_*: rnd 320B 변화 0 (define `readnone` 과 정합)", found_by="new")

p.brief_error(u"§4 G16 [240] 2건은 role 문면의 부정문(「readonly/readnone/initializes 없음」)을 속성 주장으로 읽은 게이트 오탐 — G16 에 kindchk.neg_hit 류 부정문 가드가 없다. 문면에서 속성어를 뺐지만 게이트도 고쳐야 한다.")
p.brief_error(u"§4 G12 [235] consts[9] 는 phi 상수 인입(!dbg 없음) 오탐 — srclinecheck 가 phi 인입 블록의 종결 명령 !dbg 로 귀속하면 201 이 나온다(7차 배치A 가 이미 지적한 결함 ②가 아직 살아 있다).")
p.brief_error(u"프롬프트의 「v22 nearest_enemy.0」 지목은 옳았으나 방향이 반대다: 본문이 쓰는 것은 .0 이 아니라 .1(+0x98) 이고 .0(+0x90) 은 tick 이다 — 명세 3곳(mem 이름·logic·notes)이 전부 .0 으로 적혀 있었다.")
p.brief_error(u"프롬프트 ④ TLS 절 검증 항목(CHAMP_POWERS_MEMO 등)은 이 배치 7함수 어디에도 접점이 없다(LocalKey/call_once/threadlocal/@anon…call_once 참조 0 · tlsgrep.py 전수) — 배치별 공통 문구가 K 에는 해당 없음.")
p.brief_error(u"프롬프트 ②의 sret 목록(ScoreParameter 5,384B 「calculate_score_parameter」)은 K 의 240 에선 sret 가 아니라 &mut 인자다(반환 void) — 살아있는 바이트 = 자기 필드 2 qword + 힙 원소.")
p.save()
