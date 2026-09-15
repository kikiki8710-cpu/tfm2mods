# -*- coding: utf-8 -*-
"""25차 배치D patch.json 생성 (mkpatch 참조구현 사용)."""
import sys, io, json
sys.path.insert(0, r"C:\tfm2mods\MIG")
import mkpatch
S = 198
p = mkpatch.Patch(round=25, batch="D")
P = lambda rest: "/specs[%d]/%s" % (S, rest)

# ── G12 ──
p.fix(P("consts[7]/src_line"), old=232, new=234,
      evidence=u"m15.ll:13307 `%232 = icmp ult i64 %230, 6, !dbg !27896` — !27896 의 inlinedAt 사슬 루트 = epic_hunt.rs:234(min_by_key 안 Chain 슬라이스 len 6 bounds assume, 사슬 …<2706<3416<3387<234). "
               u"232 에는 리터럴 6 을 가진 IR 이 없다(13203 `call iter_towers_without_nexus(sret [120 x i8])` 뿐 · 6 은 콜리 sret 안에서만 생성). ChampionActionState::Ult(6) 은 13848 switch 라벨(`i64 6, label %422`)이라 !dbg 없음",
      behavior_change=False, found_by="reused")
p.fix(P("consts[36]/src_line"), old=446, new=59,
      evidence=u"리터럴은 본체가 아니라 closure$0 의 별도 define(m15.ll:57952 v27_objective_safe_attack_position::{closure#0}::call_mut)에 있다: m15.ll:58021 `%43 = icmp ult i64 %42, 67600000001, !dbg !63539` 사슬 L59<298(core function.rs 는 외부). "
               u"본체 m15.ll 12793~18168 에는 67600000001 이 0건. 같은 명세의 consts[11]/[12](클로저 상수 → 클로저 자기 줄 272/325) 규약과 정합",
      behavior_change=False, found_by="reused")
p.fix(P("consts[37]/src_line"), old=446, new=59,
      evidence=u"m15.ll:58053 `%61 = icmp ult i64 %60, 48400000001, !dbg !63539` (closure$0, 사슬 L59<298) · m15.ll:58112 `%33 = icmp ult i64 %32, 48400000001, !dbg !63606` (closure$1 define 58062, 사슬 L85<298). 본체엔 0건. 첫 사용 줄 59 로 귀속(85 는 meaning 에 병기돼 있음)",
      behavior_change=False, found_by="reused")
p.fix(P("consts[70]/meaning"),
      old=u"ProjectileMoveType::Target 메모리태그(switch 4 = 6-2) → is_targeting",
      new=u"ProjectileMoveType::Target 메모리태그(switch 4 = 6-2) → is_targeting. ★리터럴 6 은 IR 에 없다: m15.ll:17647 `%1848 = add nsw i64 %1846, -2`(사슬 L134<481<2893<2494<2897<480) 뒤 17650~17653 switch 라벨 `i64 4`/`i64 5`/`i64 7` 에 접혀 있고 switch 라벨엔 !dbg 가 없다 — G12 가 481 을 못 보는 도구 오탐(귀속 481 유지)",
      kind=u"오탐", behavior_change=False, found_by="reused",
      evidence=u"m15.ll:17643~17654: gep +64(move_type) → `icmp ne i64 %1846, 9` assume → `add nsw i64 %1846, -2` → `icmp samesign ugt i64 %1846, 1` → select(…, 7) → switch [4→%1870(Target 6), 5→%1870(TargetSplash 7), 7→%1851(`icmp eq i64 %1846, 1` BouncingTarget 판별)]. 값 6 을 가진 !dbg 명령 자체가 없다")

# ── G16 ──
p.fix(P("sig/params[3]/role"),
      old=u"L238/L579 PlayerState::strategy(정의부 readnone=미사용)",
      new=u"L238/L579 PlayerState::strategy(strategy 정의부 g15.ll:130480 의 rnd 인자 %2 가 소비되지 않음 — 이 함수 define 의 %3 속성은 noalias·align 16·dereferenceable(320) 뿐)",
      kind=u"오탐", behavior_change=False, found_by="new",
      evidence=u"m15.ll:12793 define … `ptr noalias noundef align 16 dereferenceable(320) %3` (readnone/readonly 없음 = &mut 정합). 문면의 `readnone` 은 콜리 strategy 의 define(_gcbc/g15.ll:130480 `ptr noalias noundef readnone align 16 captures(none) dereferenceable(320) %2`) 속성을 인용한 것 — G16 이 낱말을 이 함수 인자 속성 주장으로 읽은 오탐. 낱말을 치워 게이트를 닫는다")

# ── rnd 소비 조건(보강) ──
p.fix(P("sig/params[3]/role"),
      old=u"L608~610(적마다 3회)·L627(die_tick<tps 시 4회째) range_misjudge_roll",
      new=u"L608~610(적마다 3회)·L627(die_tick<tps 시 4회째) range_misjudge_roll — ★단 jrng.0(시드 플래그)==false 일 때만 rnd.gen_range 를 소비(m04.ll:49736~49775: `trunc nuw i64 %6 to i1` → true 면 jrng 상태 splitmix 자체 전진·rnd 불변, false 면 StdRng::gen_range). range_misjudge_rng 는 version>1 이면 플래그 1(m04.ll:49681 `icmp ugt i64 %0, 1`) → 현행(version≥2)에서 L608~627·L463 의 roll 은 rnd 를 소비하지 않는다(오라클 o198 N3/N4: 적 근접·롤 3회 경로에서 rnd 불변 실측)",
      kind=u"보강", behavior_change=False, found_by="new",
      evidence=u"m04.ll:49736 define range_misjudge_roll: %6=load jrng+0 → br i1 → %8 블록(jrng+8 splitmix64 갱신·`mul nuw i128`·rnd 미접촉) / %28 블록(`call StdRng::gen_range<RangeInclusive<u64>>(rnd, …)`). m04.ll:49676 range_misjudge_rng: `%5 = icmp ugt i64 %0, 1` → %23 phi [1,%28],[0,%4] = 플래그. 오라클 _verify25/D/oracle/o198_truth.log N3(enemy=1)·N4 rnd_changed=0")

# ── TLS 절 정정(보강) ──
tls_old = (u"TLS 를 쓰는 콜리를 J 범위가 부르는 순서(미러 설계용): ①L205 Effect::expected_damage_target(need_recall==false 경로) ②L227 v27_objective_discipline_action(need_recall==false) ③L237 battle_action→attack_summon_action→attack_jungle_action ④L244 closure#5(should_ignore_object_finish_kill_priority_target·is_in_range) ⑤L315 retain#7(expected_heal/shield/buff vtable·aoe_heal_covers_low_ally) ⑥L424 retain#8→EpicHuntSubPlan::score(각 액션마다) ⑦L429 get_move_action: L523 nontarget_windup_perceived(적 챔프마다) → L536 position_score_at_position(purpose=Objective 11) → L544 v25_objective_posture → L570 MapDef::camp_pos(★game_core 내부 LocalKey<RefCell<(usize,[[Option<(u64,u64)>;2];8])>> 메모, g02.ll:6914 — 순수 좌표 캐시라 순서 무관)+v23_should_break_objective_hunt_anchor → L599 check_kill_die_tick(TLS 메모 함수, 키에 엔티티 id) → L607~627 range_misjudge_rng/roll·max_range_can_use/nearly → L651 camp_pos+v23_should_break… . 자식 명세(r13/r14/r15)의 tls 절이 정본.")
tls_new = (u"TLS 작성자 도달 순서(IR 호출그래프 전이 폐포 · LocalKey::with 인스턴스 기준 · dyn 디스패치(AbstractGame/EffectType vtable)는 미추적 · 25차D cg.json/tlsreach.txt): "
           u"①L205 expected_damage_target · L227 v27_objective_discipline_action · L237 battle_action/attack_summon_action/attack_jungle_action · L244 closure#5(should_ignore…·is_in_range) · L315 retain#7(aoe_heal_covers_low_ally 포함) = TLS 도달 0 "
           u"②L424 retain#8→score(=epic_action_score 순수 래퍼 m15.ll:20679) → camp_pos CAMP_POS_MEMO · interaction_score INTER_CTX → check_kill_die_tick DIE_TICK_CACHE · position_eval_at POS_EVAL_CACHE(→ uncached: PE_PLAYER_CTX·PE_CAND_MASKS·EPC_CACHE·ATTACK_DMG_CACHE·TOWER_MINION_CNT_CACHE·SLOT_READY_MEMO · v47_siege_stance SIEGE_STANCE_CACHE·RESOLVE_FIGHT_CACHE) · champion_hp_value HP_VALUE_MEMO · minion_wave_risk ATTACK_DMG_CACHE — act_actions 원소마다(비면 0회) "
           u"③L429 get_move_action: L523 nontarget_windup_perceived(TLS 0) → ★L536 position_score_at_position(POS_EVAL_CACHE 계열 · 무조건 1회) → L544 v25_objective_posture→camp_pos CAMP_POS_MEMO(g07.ll, 순수 좌표 캐시) → L570 camp_pos(조건 epic Some && dist²>mr²) → ★L599 check_kill_die_tick DIE_TICK_CACHE(+uncached 경로 ATTACK_DMG_CACHE · 키에 엔티티 id · L538/L554/L573 조기반환 뒤 무조건 1회) → L607~627 range_misjudge_rng/roll·max_range_can_use/nearly = TLS 0(MAX_RANGE_CACHE 는 max_range_cached 전용이라 이 루트에 없음) → L651 camp_pos(조건 epic Some) "
           u"④K L446 v27 인라인(조건 act_actions≠∅ ∧ attacking_objective ∧ epic hp%≥36 ∧ 사거리 안 ∧ threats≠∅): position_score_at_position L100 1회 + position_score_at_cell L124 후보 셀마다 ⑤L476 epic_action_score/score = move_actions 원소마다(②와 같은 집합) ⑥L486 get_input(조건 trajectory_possible∨enemy_rushing): RunAway→nexus_final_stand LAST_STAND_MEMO·shared_free_dist FREE_DIST_CACHE / AroundPosition→positioning_cell_value_at→POS_EVAL_CACHE 계열 / path_finder PATH_SCRATCH·FIELD_SCRATCH ⑦L494 position_score_at_position(조건 Input::Move). "
           u"v48_cast_beams CAST_BEAMS 는 이 루트에서 도달 불가(BattleSubPlan 전용). 자식 명세(r13/r14/r15)의 tls 절이 키·무효화 정본.")
p.fix(P("sig/tls/call_conditions"), old=tls_old, new=tls_new, kind=u"보강", behavior_change=False, found_by="new",
      evidence=u"scratchpad25D/cg.py: _gaibc m*.ll + _gcbc g*.ll 전 define 의 call/invoke 그래프(36,096 define) + thread_local 전역 80종 참조 → tlsreach.py: LocalKey<RefCell<T>>::with 인스턴스를 TLS 접점으로 승격해 직접 콜 사이트별 전이 도달을 BFS(깊이 12). "
               u"결과 tlsreach.txt: TLS 도달 콜 사이트 = 13674(L424 retain#8) · 13949(L536) · 13972(L544→camp_pos) · 14538/15181(camp_pos) · 14637(L599) · 16509/16849(L446) · 17555/17561(L476) · 17883(L486) · 17911(L494) 뿐 · battle_action(29 콜리)·attack_summon_action·attack_jungle_action·v27_objective_discipline_action·closure#5·retain#7·max_range_can_use/nearly·range_misjudge_rng/roll = 도달 0. max_range_cached 호출자 44 중 EpicHunt 없음(BattleSubPlan·battle_common 등)")

# ── G7 과열림: open[13] ──
p.fix(P("open[13]"),
      old=u"epic_action_score · EpicHuntSubPlan::score · get_input · is_recent_visible · range_adjust · block_input 내부는 계약만",
      new=u"epic_action_score · get_input · range_adjust · block_input 내부는 계약만(is_recent_visible 은 확정 항목 · EpicHuntSubPlan::score 는 epic_action_score 의 순수 래퍼로 확정 m15.ll:20679~20681)",
      kind=u"보강", behavior_change=False, found_by="new",
      evidence=u"G7: shared.is_recent_visible 은 확정 항목이라 open 에서 제외. score 래퍼 확정 = m15.ll:20679 define score(%0 readonly captures(none)) 본문이 `tail call epic_action_score(%1,%3,%4,%5,%2,%6,%7)` 한 줄 + ret")

# ── sret variant/live 표(보강) — 런타임 _SAP_EL 재료 ──
p.fix(P("sig/params[0]/role"),
      old=u"빈 Vec 은 ptr=8(dangling) cap=len=0.",
      new=u"빈 Vec 은 ptr=8(dangling) cap=len=0. ★이 함수가 sret 에 넣는 variant 집합(IR 확정) = RunAway 3 · Recall 4 · AroundPosition(untagged, +0xb1=outline_type 0) · Trace 14 · Attack/Skill/Skill2/Ult 15~18 · Stop 19 (Around 5·AroundRegion 7·AroundBush 12·LaneMinionPosition 13 은 불가 — closure#5 가 tag∉15..18 을 전부 false 로 걸러 L512 는 15~18 만, v27_objective_discipline_action 은 None/AroundPosition::new/RunAway 만 반환 m09.ll:19231~19840). 원소 184B 중 live(생성자 initializes/store + 태그): RunAway [0,56)+[125,126)+[128,132)+{177} · Recall [69,70)+[72,129)+{177} · AroundPosition [0,104)+{173}+[176,178) · Trace [0,8)+[85,86)+[88,150)+{177} · Attack계 [0,17)+{177} · Stop {177} — 나머지는 alloca 잔재(memcpy 로 그대로 복사됨).",
      kind=u"보강", behavior_change=False, found_by="new",
      evidence=u"생성자 define 속성/store: RunAway::new_with_skill m08.ll:92086 `initializes((0, 56), (125, 126), (128, 132))` · Recall::new m08.ll:98371 `initializes((69, 70), (72, 129))` · Trace::new/new_attack_range/new_attack_range_margin m02.ll:9506/8914/9343 `initializes((0, 8), (85, 86), (88, 150))` · Attack/Skill/Skill2/Ult::new m07.ll:7129/7645/12020/12535 `initializes((0, 17))` · AroundPosition::new m08.ll:103238(store [0,48)·memcpy 40 ← wait_around m04.ll:33007 [0,40) 전부 store·store 88/96·memcpy 72 ← Option<PathFinder> alloca(+69 만 store → 173)·store 176=6·177=0) 및 new_with_radius 103103 동형. 원소 조립 = 본체 memcpy 136/152 + `store i8 <tag>` @+177(m15.ll 14030·14098·15378·15628·18128 등) → 184B memcpy(push). variant 집합: closure#5 m15.ll:57289~ switch(%52) 13 엣지 → phi false · v27_objective_discipline_action sret 태그 store = -1/-1/-1/3 + AroundPosition::new 직접 sret. 오라클 o198: R0/R2 tag3 · R1/E1/E3/E6/E7 tag4 · E2/E4/E5/N2/N3 tag14 · N1/N4 tag15,16 실측")

# ── ev 상향(오라클 실행 확증) ──
EV = u"오라클 실행 확인: _verify25/D/oracle/o198.rs (pub 직접 호출 · 케이스당 프로세스 1개 · run198.py 18케이스 · o198_truth.log) 18/18 MATCH — "
p.ev(P("mem[100]"), to=3, evidence=EV + u"need_recall 쓰기: R1/R1b/R2b(nr=1, ratio 29/28/29→유지 1) · R2(ratio 30→0) · E1/E3/E6/E7(→1) · E2/E4/E5(무기록 0) · E8/E9(nr=1, ratio>29 →0)", found_by="new")
p.ev(P("mem[0]"), to=3, evidence=EV + u"need_recall 읽기 L203/L221 분기(nr=0/1 양쪽 경로 실행)", found_by="new")
p.ev(P("consts[2]"), to=2, evidence=EV + u"dmg*3 >= hp 경계: E1 hp=3d(408)→true · E2 hp=3d+1(409)→false · E7 hp=3d-1→true (focused==me)", found_by="new")
p.ev(P("consts[4]"), to=2, evidence=EV + u"hp*100/max > 29 경계: R1 hp=290/1000(29)→유지 · R2 hp=300(30)→해제 · R2b hp=299(29)→유지", found_by="new")
p.ev(P("consts[3]"), to=2, evidence=EV + u"hp_ratio=hp*100/max_hp (R1/R2/R2b/E8/E9)", found_by="new")
p.ev(P("consts[5]"), to=2, evidence=EV + u"Recall 메모리태그 4 (R1/R1b/R2b/E1/E3/E6/E7 elem[0]@+177=4) · JungleType::Morgard 인자는 미검증", found_by="new")
p.ev(P("consts[1]"), to=2, evidence=EV + u"EntityType::Epic 태그 5 게이트(에픽 스폰 케이스 E1~E9 ty_tag=5 실측) · end_delay 5(RunAway/Trace elem end_delay 필드=5)", found_by="new")
p.ev(P("consts[16]"), to=2, evidence=EV + u"Trace 태그 14 (E2/E4/E5/E8/E9/N2/N3 elem[0]@+177=14 · N3 는 new(attack_range_only=0) · E2 는 new_attack_range(attack_range_only@145=1))", found_by="new")
p.ev(P("consts[32]"), to=2, evidence=EV + u"N1/N4: act_actions=Attack15+Skill16(target=epic id) 가 L444 tag-15<4 판정을 지나 L512 로 반환", found_by="new")
p.ev(P("knobs[0]"), to=2, evidence=EV + u"E1/E2/E7 경계(3d, 3d±1)", found_by="new")
p.ev(P("knobs[1]"), to=2, evidence=EV + u"R1/R2/R2b 경계(29/30)", found_by="new")
p.ev(P("sig/params[1]"), to=2, evidence=EV + u"self 쓰기 = +0 1바이트만(m15.ll:13136 단일 store · 오라클 spb 바이트 대조)", found_by="new")
p.ev(P("sig/params[3]"), to=2, evidence=EV + u"Recall 경로·에픽 원거리 Trace 경로·적 근접 롤 경로 전부 rnd 불변(rnd_changed=0 18/18) — version 60 에서 range_misjudge_roll 이 rnd 를 안 씀", found_by="new")

p.brief_error(u"§4 G12 ‘실제 후보’ 4건 중 3건은 후보가 맞고(234 · 59 · 59) 1건(consts[70])은 접힌 상수라 후보 자체가 무의미 — 게이트가 switch 라벨·`add -2` 접힘을 못 본다는 주석을 §4 에 이미 달았으니 오탐 처리 규약은 정상 작동. 다만 §4 머리줄이 ‘이 배치 몫 = 6건’ 인데 실제 나열은 G12 4 + G16 1 + G7 1 = 6 으로 일치 — 오류 없음")
p.brief_error(u"프롬프트 ②의 variant 집합(RunAway·Around 5·AroundRegion 7·AroundPosition·AroundBush 12·Trace·Attack~Ult·Recall·Stop·LaneMinionPosition 13)은 이 함수에 대해 과대다 — Around 5·AroundRegion 7·AroundBush 12·LaneMinionPosition 13 은 IR 상 이 함수 sret 에 들어갈 수 없다(closure#5 필터 + v27_objective_discipline_action 반환 집합). `next_plan Option<BigPlan> 384B` 도 이 함수엔 없는 항목(형제 루트 공용 문안)")
p.brief_error(u"프롬프트 ④의 TLS 작성자 목록 중 max_range_cached(MAX_RANGE_CACHE)·v48_cast_beams(CAST_BEAMS)는 이 루트에서 도달 불가(호출그래프 실측) — ‘직접·간접으로 부르는’ 전제가 이 함수엔 성립하지 않는다")
p.brief_error(u"DOSSIER §2 가 `_verify3/TEMPLATE.rs` 만 가리키는데 TeamPlan/ScoreParameter/DebugFrameData 조립 선례(zeroed ScoreParameter = _verify24/F/oracle/o186.rs:294 · TeamPlan Default = _verify2/B/B_o1.rs:88)는 안내가 없어 재발견해야 했다 — 루트 함수 배치엔 ‘인자 조립 선례 표’를 §2 에 넣어 달라")
out = p.save()
print(out)
