# -*- coding: utf-8 -*-
"""25차 배치 B patch.json 생성 (mkpatch 참조구현 사용)"""
import sys, io
sys.path.insert(0, r"C:\tfm2mods\MIG")
import mkpatch
p = mkpatch.Patch(round=25, batch="B")
S = "/specs[196]"
ORC = "오라클 o196.exe(_verify25\\B\\oracle\\o196.rs · 케이스당 프로세스 1개 · scen 0/2/3/4/5/6/7/8/9 + seed 1/2/3)"

# ── ① TLS 절 정정 (G-없음 축: sig.tls 는 어떤 게이트도 안 본다) ──
CC = S + "/sig/tls[0]/call_conditions"
p.fix(CC,
      old="TLS 접점은 전부 콜리 내부: 배치 D 범위에서 순서대로 ①nontarget_windup_perceived(L890, 적 챔피언 루프 안 최대 5회, 21616)",
      new="TLS 접점은 전부 콜리 내부(★25차 B 전이 스캔 — _gaibc defidx BFS 992 define + _gcbc 147 define, LocalKey<..>::with<closure> 호출 심볼로 판정: TLS 에 닿는 직접 콜리 = position_score_at_position(L903·L222·L860) · lane_minion_position_action(L183·L356) · v22_lane_tower_pressure_attack_allowed(L389) · check_kill_die_tick(L259) · v47_siege_stance(L774) · retain<closure#7>→score(L742) · retain<s8_0>→unsafe_v19_non_champion_walkup(L788) · score(L837) · get_input(L853). TLS 0건 = nontarget_windup_perceived · battle_action(L928·L809) · line_minion_action_candidates · attack_summon_action · attack_structure_skill_action · battle_ally_action · v30_line_champion_action_tower_aggro_risk(L736·L810) · closure#3/#5 · Around/RunAway/Trace/Attack 생성자 · game_core 콜리 전부): 배치 D 범위에서 순서대로 ①nontarget_windup_perceived(L890, 적 챔피언 루프 안 최대 5회, 21616 — TLS 없음)",
      evidence="a6_tls3.py/a7_paths.py(스크래치): 루트→콜리 BFS 에서 `8LocalKeyI…EE4with<closure>` 호출 심볼을 TLS 접점으로 집계. 예: m07.ll:48462 `call @…LocalKey<RefCell<SiegeStanceCache>>::with<v47_siege_stance::closure#0>` · m15.ll check_kill_die_tick → `…RefCell<DieTickCache>…with<check_kill_die_tick::closure#0>` · battle_action(m15.ll:23700~25612) 본문·전이 콜리에 with 호출 0건",
      behavior_change=False, found_by="new", kind="실오류")
p.fix(CC,
      old="②position_score_at_position(L903, 21742; 순수 sret — POS_EVAL_CACHE 는 position_eval_at 계열이며 이 함수는 position_eval_at 를 직접 부르지 않는다)",
      new="②position_score_at_position(L903, 21742 — ★TLS 간접 작성자: 본체 m07.ll:34232~34265 가 x/32000·y/32000 을 umin 29 로 셀 양자화한 뒤 셀중심(*32000+16000)으로 m07.ll:34263 `tail call @…position_eval_at` → POS_EVAL_CACHE, 그 안 position_eval_at_uncached → EPC_CACHE·PE_CAND_MASKS·PE_PLAYER_CTX·ATTACK_DMG_CACHE·TOWER_MINION_CNT_CACHE·SLOT_READY_MEMO·CC_TIME_MEMO·v47_siege_stance(SIEGE_STANCE_CACHE)·resolve_fight_full(RESOLVE_FIGHT_CACHE). 이 함수 IR 에 position_eval_at 심볼이 없는 것은 한 단계 아래라서다)",
      evidence="m07.ll:34255~34263 `%13 = mul nuw nsw i64 %10, 32000` `%14 = add nuw nsw i64 %13, 16000` … `tail call void @_RNvNtCshdEBA0ozCnw_7game_ai13position_eval16position_eval_at(ptr sret([56 x i8]) %0, i64 %1, ptr %2, …)`; m07.ll:34241~34244 `%9 = udiv i64 %5, 32000` `%10 = llvm.umin.i64(%9, 29)`. 배치 F 자신도 L860 에 「★TLS POS_EVAL_CACHE(콜리 내부 position_eval_at)」라 적어 배치 D 문면과 자기모순이었다",
      behavior_change=False, found_by="new", kind="실오류")
p.fix(CC,
      old="③base_positioning 안 lane_minion_position_action(L183, 22206)",
      new="③base_positioning 안 lane_minion_position_action(L183, 22206 — choose_goal>push_candidate>position_score_at_position 경유 POS_EVAL_CACHE 계열; 조건 = L905 조기귀환 아님 && L179 front_minion 이 있고 아군 타워보다 앞)",
      evidence="a7_paths.py: lane_minion_position_action > SmallActionLaneMinionPosition::choose_goal > push_candidate > position_score_at_position > position_eval_at (with<position_eval_at::closure#0>)",
      behavior_change=False, found_by="new", kind="보강")
p.fix(CC,
      old="⑦v22_lane_tower_pressure_attack_allowed(L389, 23402)",
      new="⑦v22_lane_tower_pressure_attack_allowed(L389, 23402 — v47_siege_stance(SIEGE_STANCE_CACHE)→resolve_fight→resolve_fight_full(RESOLVE_FIGHT_CACHE); 조건 = attack_tower_action 안 L374 champ Some · L375~377 can_target 적 타워 존재 · L380 attack_effect Some · L385 dist² ≤ max_dist²)",
      evidence="a7_paths.py: v22_lane_tower_pressure_attack_allowed > tower_discipline::v47_siege_stance(with<v47_siege_stance::closure#0/#1>) > old::fight_model::resolve_fight > resolve_fight_full(with<resolve_fight_full::closure>)",
      behavior_change=False, found_by="new", kind="보강")
p.fix(CC,
      old="v47_siege_stance(SIEGE_STANCE_CACHE) 는 **배치 E L774 에서 1회**(27112, 조건: L771 nearest_enemy_tower.id == …)",
      new="v47_siege_stance(SIEGE_STANCE_CACHE) 직접 호출은 **배치 F L774 에서 1회**(27112, 조건: L769 nearest_enemy_tower Some · L770 ty==Tower(2) · L771 info.nearest_enemy.map(|e| e.1) == Some(champ.id) — 타워가 **나를** 물고 있을 때) · 간접으로는 position_score_at_position(L903/L222/L860)·lane_minion_position_action·v22(L389)·unsafe_v19(L788, v47_tower_focus_position_dangerous 경유)·score(L742/L837, evaluate_action→position_eval_at 경유)에서도 닿는다",
      evidence="m14.ll:26992~27112: `br i1 %1046`(L769) → 26999 `icmp eq i64 %…, 2`(L770) → 27086 tag / 27107 `== champ+0x5c0`(L771, Option<usize>::eq) → 27112 invoke v47_siege_stance. L774 는 line_defense.rs 769~879 = 배치 F 범위(배치 E 는 456~766). 간접 경로 = a7_paths.py 출력",
      behavior_change=False, found_by="new", kind="실오류")
p.fix(CC,
      old="v48_cast_beams / champion_hp_value / interaction_score / position_eval_at 는 이 함수 IR 에 심볼 0건(콜리 내부 간접). 미러 재현 시 배치 D 구간의 콜리 호출 순서를 위 ①~⑨ 그대로 유지해야 한다",
      new="champion_hp_value(HP_VALUE_MEMO)·lane_anchor_gain(LAG_MEMO)·interaction_score(INTER_CTX)·last_stand_flags(LAST_STAND_MEMO, nexus_final_stand/base_defense_focus 경유)·check_kill_die_tick(DIE_TICK_CACHE) 는 score()(L742 closure#7 원소별·조건부 = gated ? target_is_ally 일 때만 : 항상 · L837 max_by_key 원소별 항상) 경유 간접 접점이고 position_eval_at 은 position_score_at_position 바로 아래다. ★이 루트에서 **도달 0** = v48_cast_beams(CAST_BEAMS, BattleSubPlan 전용) · max_range_cached/fight_kit_range_cached(MAX_RANGE_CACHE — max_range_can_use/nearly/max_poke_range 는 캐시를 안 쓴다) · CAMP_POS_MEMO/LANE_PATH_MEMO/NEAR_LANE_GRID/BATTLE_RECENCY(game_core, 이 루트의 game_core 콜리 147 define 전이 스캔에서 with 호출 0건 — 유일한 game_core TLS 접점은 get_input 아래 nt_trace_record_aim 의 std RandomState::new KEYS 로 판정 무관). 미러 재현 시 TLS 접점 호출 순서(전 구간) = L903 position_score_at_position → (L183 lane_minion_position_action) → (L389 v22) → [L742 score ×n] → L222 position_score_at_position → L259 check_kill_die_tick → (L356 lane_minion_position_action) → (L774 v47_siege_stance) → [L788 unsafe_v19 ×n] → L809 battle_action(TLS 없음) → [L837 score ×n] → (L853 get_input) → (L860 position_score_at_position); 괄호 = 조건부, [×n] = 원소별",
      evidence="a7_paths.py: L742/L837 score > plan_legacy::action_eval::evaluate_action > utils::champion_hp_value(with<champion_hp_value::closure#0>) · > action_score::interaction_score(with<interaction_ctx…>) > fight_check::check_kill_die_tick · > old::defense_nexus::nexus_final_stand(with<last_stand_flags::closure>) · evaluate_action(with<lane_anchor_gain::closure#0>). MAX_RANGE_CACHE 소유자 = `13MaxRangeCacheEE4with…16max_range_cached` / `…22fight_kit_range_cached`(m00/m10.ll) 뿐이고 max_range_can_use(m10.ll:51116~51500) 본문에 with 호출 0건. CAST_BEAMS with 소유자 = plan_legacy::sub_plan::battle(m02.ll) — 도달 집합 밖. 호출 순서 = calls.json(fn.ll !dbg 사슬 루트 줄)",
      behavior_change=False, found_by="new", kind="실오류")

# ── ② G12 오탐: consts[14] value 15 src_line 390 ──
p.fix(S + "/consts[14]/meaning",
      old="SmallActionPlay::Attack 메모리태그 15 (23416 phi)",
      new="SmallActionPlay::Attack 메모리태그 15 (23416 phi — ★G12 오탐: `%858 = phi i8 [ 15, %861 ], [ -1, %853 ]` 은 phi 상수 인입이라 !dbg 가 없고, 그 store(23418)의 !dbg !38559 는 attack_tower_action **line 0** 아티팩트. 인입 블록 %861 의 종결 명령 = 23412 `invoke @SmallActionAttack::new` !dbg !38563 = attack_tower_action.rs 390 < 934 < 442 이므로 src_line 390 이 맞다. 게이트가 낸 472 는 closure#3(별도 define m14.ll:59817)의 `tag-15` 리터럴)",
      evidence="m14.ll:23412 `invoke void @…SmallActionAttack3new(ptr sret([24 x i8]) %52, …) to label %861 …, !dbg !38563` (!38563 → line 390 scope attack_tower_action, inlinedAt 934 → 442) · 23416 `%858 = phi i8 [ 15, %861 ], [ -1, %853 ]`(!dbg 없음) · 23418 `store i8 %858, ptr %859, align 1, !dbg !38559`(!38559 = line 0) — SPEC_RUNBOOK §S5-b G12 교정 ② 유형",
      behavior_change=False, found_by="reused", kind="오탐")

# ── ③ G16 오탐: params[1] role 의 「readonly 없음」 부정문을 속성 주장으로 읽음 ──
P1 = S + "/sig/params[1]/role"
p.fix(P1,
      old="IR 속성 = noalias dereferenceable(3), readonly 없음 ⇒ &mut.",
      new="IR 속성 = `ptr noalias noundef dereferenceable(3) %1` 뿐(읽기전용 표식·initializes 표식 둘 다 미부여) ⇒ &mut. ★오라클 실행 확증: o196.exe 9 시나리오×프로세스에서 self 3B before==after(SELF_UNCHANGED=true 9/9).",
      evidence="m14.ll:21211 define 원문 `ptr noalias noundef dereferenceable(3) %1`; 전 함수 %1 사용처 = gep +1 로드(21875·22445·22758·24848) 와 클로저 env 에 포인터 저장(21931·24355·24904·27276·27674)뿐, %1/+1/+2 대상 store 0건(a3_calls.py). G16 은 문면의 낱말 `readonly` 를 「readonly 를 주장」으로 읽었다 — 부정문(「없음」) 처리가 없는 게이트 결함. " + ORC + " self_bytes before=[01,00,01] after=[01,00,01]",
      behavior_change=False, found_by="new", kind="오탐")
p.fix(P1,
      old="(배치 E) IR 속성에 readonly 없음(=&mut 가능).",
      new="(배치 E) IR 속성에 읽기전용 표식 없음(=&mut 가능).",
      evidence="같은 G16 오탐 재발 방지용 문면 치환(사실 불변)",
      behavior_change=False, found_by="new", kind="오탐")

# ── ④ logic: old_actions 는 이전 틱 행동이 아니다 ──
p.fix(S + "/logic",
      old="old_actions=%128 (Vec<SmallActionPlay>, 이전 틱 행동)",
      new="old_actions=%128 (Vec<SmallActionPlay>, L442 action_candidates_old 가 **이번 호출에서 새로 만든 구판 후보** — 이전 틱 상태가 아니다)",
      evidence="m14.ll:23589/23642 `memcpy %129 <- %78 32`(L937/L908 action_candidates_old 반환) → 23699 `memcpy %128 <- %129 32`(L447). %78 은 L882 `Vec::new_in(bump)`(21480~21485)로 이 호출 안에서 만들어진다. 이전 틱 정보를 읽는 로드는 없다",
      behavior_change=True, found_by="new", kind="실오류")

# ── ⑤ debug 문자열 확정 (오라클) ──
p.fix(S + "/mem[133]/value",
      old="push(format!(anon.121, has_enemy_minion_in_range))",
      new="push(format!(\"has_enemy_minion_in_range: {}\", has_enemy_minion_in_range)) — 오라클 실행 확증: ctx.debug=true 일 때 debug.infos == {champ.id: [\"has_enemy_minion_in_range: false\"]}(scen 3) / [\"… true\"](scen 9)",
      evidence=ORC + " scen 3 출력 `debug.infos\t{18: [\"has_enemy_minion_in_range: false\"]}` · scen 9 `{18: [\"has_enemy_minion_in_range: true\"]}` · scen 0(ctx.debug=false) `{}`",
      behavior_change=False, found_by="new", kind="보강")
p.fix(S + "/logic",
      old="push(format!(anon.121, has_enemy_minion_in_range)) }",
      new="push(format!(\"has_enemy_minion_in_range: {}\", has_enemy_minion_in_range)) }   // 오라클 확증(o196 scen 3/9)",
      evidence=ORC + " scen 3/9 debug.infos 출력",
      behavior_change=False, found_by="new", kind="보강")

# ── ⑥ sret variant 집합 · variant 별 live 바이트 (런타임 _SAP_EL 재료) ──
p.fix(S + "/sig/params[0]/role",
      old="from_iter_in 3번째 인자 %1079 = data.context.pool(bump)",
      new="from_iter_in 3번째 인자 %1079 = data.context.pool(bump) ★25차 B variant/live 표: 이 루트가 sret 원소로 낼 수 있는 variant = {RunAway 3 · Around 5 · AroundRunAway 8 · LaneMinionPosition 13 · Trace 14 · Attack 15 · Skill 16 · Skill2 17} (게임 IR 전체에서 +177 태그 store 생산자를 세고 이 루트의 도달 집합(992 define)과 교집합 — Ult 18·Recall 4·Stop 19·AroundRegion 7·AroundBush 12·AroundHide 6·Positioning 9·AroundPositionBush 11·AroundPosition(untagged) 은 생산자가 도달 밖(clone 은 복사만) ⇒ closure#3 Ult arm(529~539)·closure#5 Ult arm(673~713)·closure#7 tag 18·has_runaway Recall(4) case 는 이 루트에서 데이터플로우상 사장). live 바이트(원소 184B, 태그 @+177): RunAway = new/new_with_skill initializes (0,56)+(125,126)+(128,132) [start_tick·goal_x·goal_y·end_delay·goal_risk·prog_best_dist_sq·prog_best_tick · path_finder None 니치 +0x7d=2 · with_skill/with_ult/dodge_trajectory/goal_committed] · Around = (0,56)+(125,126)+(128,130) [+0x80 purpose(with_purpose 덮어씀)·+0x81 escape_mode] · AroundRunAway = Around 와 같고 +0x81=1 (memcpy 129 + 6 뒤 store) · Trace = new (0,8)+(85,86)+(88,150) / new_keep_range (0,16)+(85,86)+(88,150) · Attack/Skill/Skill2 = (0,17) (24B sret → memcpy 24 로 17~24 는 잔재) · LaneMinionPosition = new initializes (0,48)+(117,118)+(120,121) (콜리 lane_minion_position_action 계약). 그 외 바이트(RunAway/Around 56~125·126~128·132~177·178~184 등)는 alloca 잔재로 런타임 대조에서 마스크. 오라클 실측: scen 0 Around{start_tick=1000,target=2,goal=(48000,272000),range=80000,end_delay=5,+125=02,purpose=8,escape=0} · scen 2 RunAway{with_skill=1(L778)} · scen 8 RunAway{with_skill=0(L336→L351)}",
      evidence="m08.ll:92086/92133 `sret([136 x i8]) … initializes((0, 56), (125, 126), (128, 132))`(RunAway::new_with_skill/new) · m08.ll:92385 `initializes((0, 56), (125, 126), (128, 130))`(Around::new) · m02.ll:9506 `(0, 8), (85, 86), (88, 150)`(Trace::new) · m02.ll:8749 `(0, 16), (85, 86), (88, 150)`(new_keep_range) · m07.ll:7129 `initializes((0, 17))`(Attack::new) · m14.ll:22628~22638 AroundRunAway memcpy 129/6 + `store i8 1 +129` + `store i8 8 +177` · 23424 Attack memcpy 24 · 25837 Trace memcpy 152 + `store i8 14`. 생산자 스캔 = a9_variants.py/tag 전수(스크래치): tag 18 생산자 = sub_plan::battle/death_battle::base_battle_action·St::action_candidates 3곳 전부 도달 밖. " + ORC,
      behavior_change=False, found_by="new", kind="보강")

# ── ⑦ ev 상향 (오라클 실행 확증) ──
p.ev(S + "/sig/params[1]", to=2, evidence="오라클 실행 확인: o196.exe 9 시나리오 SELF_UNCHANGED=true 9/9 (self 3B 쓰기 0)", found_by="new")
p.ev(S + "/sig/params[7]", to=2, evidence="오라클 실행 확인: o196.exe scen 4 team_plan 을 0xAB 1064B 쓰레기로 주어도 scen 0 과 sret 바이트 동일(미사용)", found_by="new")
p.ev(S + "/mem[133]", to=3, evidence="오라클 실행 확인: ctx.debug=true 에서만 debug.infos[champ.id] push(scen 3/9), false 면 {}(scen 0/2/4/5~8)", found_by="new")
p.ev(S + "/mem[135]", to=3, evidence="오라클 실행 확인: self before==after 9/9 프로세스", found_by="new")
p.ev(S + "/mem[145]", to=3, evidence="오라클 실행 확인: self before==after 9/9 프로세스", found_by="new")
p.ev(S + "/mem[126]", to=3, evidence="오라클 실행 확인: Around 원소 +0x80 == line_phase_position_eval_purpose(player,data)=8 (scen 0/3/4/5/6/7)", found_by="new")
p.ev(S + "/mem[127]", to=3, evidence="오라클 실행 확인: L353 Around 의 +0x81 escape_mode=0 (scen 0) — AroundRunAway(L924) 만 1 (IR 22636)", found_by="new")
p.ev(S + "/consts[12]", to=2, evidence="오라클 실행 확인: sret 원소 태그 5(Around) 관측(scen 0/3/4/5/6/7)", found_by="new")
p.ev(S + "/consts[26]", to=2, evidence="오라클 실행 확인: L336 RunAway 태그 3 · with_skill=0 관측(scen 8/9)", found_by="new")
p.ev(S + "/consts[51]", to=2, evidence="오라클 실행 확인: L778 RunAway 태그 3 · with_skill=1 관측(scen 2, 적 타워 nearest_enemy.1=champ.id)", found_by="new")
p.ev(S + "/consts[50]", to=2, evidence="오라클 실행 확인: RunAway.end_delay(+0x18)=5 (scen 2/8/9)", found_by="new")
p.ev(S + "/consts[23]", to=2, evidence="오라클 실행 확인: Around.end_delay(+0x30)=5 · RunAway.end_delay=5 (scen 0/2/8)", found_by="new")
p.ev(S + "/knobs[18]", to=2, evidence="오라클 실행 확인: L778 경로 RunAway.end_delay=5 (scen 2)", found_by="new")

# ── brief_errors ──
p.brief_error("도시에/의뢰문 「dict 필드 하위 키는 /specs[<idx>]/sig/tls/<키>」 — 196 의 sig.tls 는 **길이 1 리스트**라 실제 경로는 /specs[196]/sig/tls[0]/<키> 다(mkpatch locate 로 확인). dict 라고 쓰면 0건 적용된다.")
p.brief_error("의뢰문 ② 「next_plan 은 Option<BigPlan> 384B … attack 은 Option<Input> 32B」 는 이 함수에 없는 출력이다 — 196 의 sret 는 bumpalo Vec<SmallActionPlay> 32B 하나뿐이고 Option<Input> 은 L853 get_input 의 **지역** sret(%92) 이다. BigPlan 반환 루트(sub_plan/next_plan 계열)용 문장이 그대로 실렸다.")
p.brief_error("의뢰문 ② variant 목록(RunAway·Around·AroundRegion·AroundPosition·AroundBush·Trace·Attack/Skill/Skill2/Ult·Recall·Stop·LaneMinionPosition)은 SmallActionPlay 전체 집합이지 이 루트의 산출 집합이 아니다 — 실제 산출 가능 = {3,5,8,13,14,15,16,17}(Ult·Recall·Stop·AroundRegion/Bush/Position 생산자는 도달 밖).")
p.brief_error("의뢰문 ④ TLS 작성자 목록의 v48_cast_beams(CAST_BEAMS)·max_range_cached(MAX_RANGE_CACHE)·CAMP_POS_MEMO 는 이 루트에서 도달 0 이다(전이 스캔). 반대로 목록에 없는 EPC_CACHE·PE_CAND_MASKS·PE_PLAYER_CTX·ATTACK_DMG_CACHE·TOWER_MINION_CNT_CACHE·SLOT_READY_MEMO·CC_TIME_MEMO·RESOLVE_FIGHT_CACHE·LAG_MEMO·PathScratch/FieldScratch(path_finder) 가 닿는다.")
p.brief_error("§4 G16 「p[1] self readonly — IR %1 = ptr noalias noundef dereferenceable(3)」 은 게이트가 문면의 「readonly 없음」(부정문)을 속성 주장으로 읽은 오탐이다 — srcline/role 대조기에 부정어(없음/미부여/not) 처리가 없다.")
p.brief_error("§4 G12 「consts[14] src_line=390 … 실제 후보 [472]」 — 후보 472 는 closure#3(별도 define) 의 `tag-15` 리터럴이고 이 함수 본문의 15 는 phi 상수 인입(② 유형)이라 강한 후보가 없는데도 불일치로 냈다. 「강한 후보 없음 + claim 있음」은 판정 보류여야 한다.")
p.brief_error("도시에 §1 「open 24」 중 open[2] 는 class=미탐색이지만 물음(is_in_skill 반환형)은 tcx 로 즉답된다: game_core::Entity::is_in_skill = pub fn(&Entity) -> bool (entity.rs:1570, mir=true, xinl=true; DWARF !29181 types {bool, &Entity}). G10 지적대로 notes 로 옮겨야 한다(산문 보고 참조).")
out = p.save()
