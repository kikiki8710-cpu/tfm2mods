# -*- coding: utf-8 -*-
"""25차 A · spec 188 patch.json 생성(mkpatch 참조구현 사용). 실행: python -X utf8 mk188patch.py"""
import sys, io
sys.stdout.reconfigure(encoding='utf-8')
sys.path.insert(0, r"C:\tfm2mods\MIG")
import mkpatch
p = mkpatch.Patch(round=25, batch="A")
S = "/specs[188]"

# ── G20 R1/R2 표기 통일(194 와 같은 규약) ──
p.fix(S + "/mem[107]/offset", old="0x0 -> Vec 원소", new="0x0 -> 원소[len..]", kind="보강",
      evidence="G20 R2: #194 mem[52]/[53] 의 extend 표기 `0x0 -> 원소[len..]` 와 같은 자리(Extend::extend 가 res.len 이후에 append · m02.ll:35480 ;L539). 값이 아니라 표기 차이 → 통일",
      behavior_change=False, found_by="new")
p.fix(S + "/mem[110]/offset", old="0x0 -> Vec 원소[0]", new="0x0 -> 원소[len]", kind="보강",
      evidence="G20 R2: #194 의 push 표기 `0x0 -> 원소[len]` 와 같은 자리. L605 truncate(0) 직후라 len==0 이지만 push 는 [len] 에 쓴다(m02.ll:36279 ;L606 · gep %len 인덱스). 오라클 C11d 실측 len=1·[0]=RunAway",
      behavior_change=False, found_by="new")
p.fix(S + "/mem[108]/offset", old="0x0 -> Vec 원소[len]", new="0x0 -> 원소[len]", kind="보강",
      evidence="같은 배열의 표기 통일(#194 규약). m02.ll:35913 ;L554 push",
      behavior_change=False, found_by="new")
p.fix(S + "/mem[109]/name", old="truncate(0) → len=0", new="len (truncate(0) → 0)", kind="보강",
      evidence="G20 R1: (res,0x18) 을 #194 는 `len` 으로 부른다. 같은 자리(bumpalo Vec len@+0x18 · m02.ll:36265 ;L605 truncate). 이름을 len 으로 맞추고 동작은 괄호로",
      behavior_change=False, found_by="new")

# ── sret variant live 표 ──
p.fix(S + "/sig/params[0]/role",
      old="반환값. ptr@0 · bump@+0x8 · cap@+0x10 · len@+0x18.",
      new="반환값. ptr@0 · bump@+0x8 · cap@+0x10 · len@+0x18. ★(25차 A) 이 함수가 원소로 넣는 variant = RunAway 3(L403 new_dodge · bp:287/292/300/346 new · bp:276 new_with_ult · L606 new) · AroundHide 6(bp:146) · AroundPosition(untagged, +0xb1=outline_type=0 · L525/bp:198 new_with_radius) · Trace 14(bp:114/136/140/153/155/205/207/220/222 인라인) · Attack 15(L554 + base_battle_action extend) · Skill 16/Skill2 17/Ult 18(L539·bp:314 base_battle_action extend 만 — base_battle_action 본문 m02.ll:66196~74096 의 태그 store 는 15/16/17/18 뿐) · version<2 전용 AroundRunAway 8(bp:334/342 · 오라클 C21 version=1 실측) · 그 외(Recall 4·Around 5·AroundRegion 7·Positioning 9·AroundPositionBush 11·AroundBush 12·LaneMinionPosition 13·Stop 19)는 도달 불가. 원소 184B live 바이트(그 밖은 alloca 잔재): RunAway (0,0x38)+(0x7d)+(0x80,0x84)+(0xb1) [new/new_dodge/new_with_ult initializes((0,56),(125,126),(128,132)) m08.ll:92041/92133/92185] · AroundHide (0,0x30)+(0x75)+(0xb1) [initializes((0,48),(117,118)) m08.ll:110209] · AroundPosition (0,0x68)+(0xad)+(0xb0,0xb2) [new_with_radius m08.ll:103143~103165 · +0x30~0x58 은 wait_around sret 40B 전부 store(m04.ll:33088~33096) · path_finder 72B 중 +0x45(=0xad) 태그 2 만 · +0xb0=6 · +0xb1=0] · Trace (0,8)+(0x55)+(0x58,0x96)+(0xb1) [m02.ll:29144~29168 등 5사이트 · +0x91 attack_range_only 는 bp:136 만 1] · Attack/Skill/Skill2/Ult (0,0x11)+(0xb1) [initializes((0,17)) m07.ll:7129/7645/12020/12535] · AroundRunAway(NA) (0,0x38)+(0x7d)+(0x80,0x82)+(0xb1) [SmallActionAround::new m08.ll:92385].",
      kind="보강", evidence="IR 태그 store(+177) 전수 m02.ll:29168~36277 20사이트 + 생성자 define initializes 속성 + new_with_radius/wait_around 본문 store 전수 + 오라클 o188 32케이스(태그·필드 실측)",
      behavior_change=False, found_by="new")

# ── &mut self 쓰기 전수 ──
p.fix(S + "/sig/params[1]/role",
      old="IR 속성 initializes((43,44),(45,46)) = +0x2b v48_dodge_claim · +0x2d last_bail_gate 가 무조건 쓰기 표면(L359/360).",
      new="IR 속성 initializes((43,44),(45,46)) = +0x2b v48_dodge_claim · +0x2d last_bail_gate 가 무조건 쓰기 표면(L359/360). ★(25차 A) %1 파생 gep 16개·load/store 전수: 쓰기 = +0x0 store i64 1 (29017 bp:110) · +0x8 store id (29018 bp:110) · +0x20 (33974 L511) · +0x2b 0(27739 L359)/1(33960 L510) · +0x2d 0(27741 L360)/1(32504 L417)/2(33448 L484)/3(36263 L604) — 5필드가 전부, 힙 없음(48B 전부 스칼라, tcxdict). 읽기 = +0x0/+0x8(bp:100) · +0x10(L411/441/463/491/518/537/544/560/bp:73/120) · +0x18 · +0x20(L398) · +0x28(bp:114/135~141/152/204/219) · +0x29(L421 closure$2 env 주소) · +0x2c(bp:135). ⚠+0x2a dive_local·+0x2b·+0x2d 는 이 함수 안 읽기 0회(last_bail_gate 는 _docs L588 「v3 계측·관측 전용」). 오라클 self_diff 실측: +0x8 19→23(C16b bp:110) · +0x2d 0→3(C11d L604).",
      kind="보강", evidence="scratch selfw.py: m02.ll 27592~36306 에서 `ptr %1` 파생 gep/load/store 전수(위 줄번호) · tcxdict BattleSubPlan --deep(48B 필드 16 leaf 전부 스칼라) · 오라클 o188 self 48B 전후 바이트 diff",
      behavior_change=False, found_by="new")

# ── rnd / _debug 콜리 속성 ──
p.fix(S + "/sig/params[3]/role",
      old="직접 읽기 없음. check_kill_die_tick(L449/451)·fight_participants(L473)·resolve_fight_stake(L478) 에 전달(가변)",
      new="직접 읽기 없음. check_kill_die_tick(L449/451)·fight_participants(L473)·resolve_fight_stake(L478) 에 전달(가변) — ★(25차 A) 콜리 define 속성: check_kill_die_tick 의 rnd 인자(arg#1)는 `readnone`(m15.ll define) = 미사용 · fight_participants(arg#2)·resolve_fight_stake(arg#2)는 속성 없음(소비 가능) · SmallActionAroundHide::new(bp:147)·SmallActionAround::new(NA) 의 rnd 는 `readnone` · 실소비 확정 = new_with_radius(L526·bp:198 → wait_around, 오라클 C13d rnd_used=true, 그 외 30케이스 false). 소비 순서(한 경로): [L473 fight_participants | L478 resolve_fight_stake] → L526 new_with_radius(kite Some 이면 return) 또는 폴스루 → bp:198 new_with_radius",
      kind="보강", evidence="scratch rndflow.py: %3 이 인자로 들어가는 call 10사이트(m02.ll 29236·29965·31502·31631·33042·33064·33259·33264·35035·36242) + 각 콜리 define 인자 속성 · 오라클 rnd 복제본 gen() 비교",
      behavior_change=False, found_by="new")
p.fix(S + "/sig/params[8]/role",
      old="check_kill_die_tick(L449/451)·fight_participants(L473)·resolve_fight_stake(L478) 에 그대로 전달(원문 전 범위 6회 등장 = define + 5회 인자 전달)",
      new="check_kill_die_tick(L449/451)·fight_participants(L473)·resolve_fight_stake(L478) 에 그대로 전달(원문 전 범위 6회 등장 = define + 5회 인자 전달) — ★(25차 A) check_kill_die_tick 의 _debug 인자(arg#7)는 define 속성 `readnone` = 실제 미사용(L449/451/593 3회 모두) · fight_participants(arg#11)·resolve_fight_stake(arg#13)만 `dereferenceable(224)` 무속성 = 쓰기 가능. 즉 _debug 가 실제로 바뀔 수 있는 경로는 L472 분기 안(kill_secure 통과 후)뿐",
      kind="보강", evidence="scratch rndflow.py: %8 전달 call 5사이트(33042·33064·33259·33264·36242) + 콜리 define 인자 속성",
      behavior_change=False, found_by="new")

# ── TLS 절: 전이 도달 확정 ──
p.fix(S + "/sig/tls[1]/name", old="(콜리 내부 TLS — 미검증)", new="(콜리 내부 TLS — 25차 A 콜그래프 전이 확정)", kind="보강",
      evidence="scratch cgbuild.py/tlsreach.py: _gaibc+_gcbc define 36,096 개 콜그래프 · 루트에서 BFS(도달 define 331 · LocalKey::with 인스턴스 19)",
      behavior_change=False, found_by="new")
p.fix(S + "/sig/tls[1]/call_conditions",
      old="가 내부에서 쓸 수 있으나 그 본문은 이 배치 범위 밖(자식 명세 계약) — 호출 순서만 위 항목에 적음",
      new="가 내부에서 쓸 수 있으나 그 본문은 이 배치 범위 밖(자식 명세 계약) — 호출 순서만 위 항목에 적음 ★(25차 A) IR 콜그래프 전이 BFS 로 확정(직접 콜리 85 → 도달 define 331 · with 인스턴스 19): ①L363 nexus_final_stand → last_stand_flags(LAST_STAND_MEMO) [최초 발화 · version>=2 만] ②L390/L1178<518/bp:196(kite_reposition_point) position_score_at_position → position_eval_at(POS_EVAL_CACHE ×2 인스턴스) → uncached 경로에서 ATTACK_DMG_CACHE·TOWER_MINION_CNT_CACHE·EPC_CACHE(entity_positioning_cache_cached)·PE_CAND_MASKS·PE_PLAYER_CTX·CC_TIME_MEMO(slot_cc_time_cached)·SLOT_READY_MEMO(slot_ready_cached)·SIEGE_STANCE_CACHE(v47_siege_stance ×2) → resolve_fight → RESOLVE_FIGHT_CACHE ③L400/L506/L1191<518/bp:196 v48_on_cast_line → CAST_BEAMS ④L421 closure$2 engage_pair_range → FIGHT_KIT_RANGE_CACHE + MAX_RANGE_CACHE ⑤L445/bp:111/bp:113(v21_support_pressure_too_risky)/L575/L581(sg_0) max_range_cached → MAX_RANGE_CACHE ⑥L449/451/593 check_kill_die_tick → DIE_TICK_CACHE(×2 인스턴스) · uncached → enemy_minion_wave_risk_dps_at → ATTACK_DMG_CACHE ⑦L473 fight_participants → ally_is_bound → MAX_RANGE_CACHE + DIE_TICK_CACHE ⑧L478 resolve_fight_stake → RESOLVE_FIGHT_CACHE(resolve_fight_full) + ally_is_bound(MAX_RANGE/DIE_TICK) · L474 resolve_fight_stake_roster → RESOLVE_FIGHT_CACHE 만 ⑨L539/bp:123/160/237/314 base_battle_action → base_defense_focus → LAST_STAND_MEMO. ⛔미도달(루트에서 어떤 경로로도 with 인스턴스에 닿지 않음): HP_VALUE_MEMO(champion_hp_value) · INTER_CTX(interaction_ctx) · CAMP_POS_MEMO · CHAMP_POWERS_MEMO · HIDE_NEAREST · LAG_MEMO · PATH_SCRATCH. ⟹ 「SIEGE_STANCE_CACHE·POS_EVAL_CACHE 가 이 범위 IR 에 안 나타난다」는 직접 호출 기준으로만 참이고 position_score_at_position 경유로 전이 도달한다. 한 틱 최초 발화 순서 = L363 LAST_STAND → L390 POS_EVAL(+하위 8종) → L400 CAST_BEAMS → L421 FIGHT_KIT/MAX_RANGE → L445 MAX_RANGE → L449/451 DIE_TICK → L473/478 (MAX_RANGE·DIE_TICK·RESOLVE_FIGHT) → L506 CAST_BEAMS → L518 루프(POS_EVAL·CAST_BEAMS) → bp:111/113 MAX_RANGE → bp:123~ LAST_STAND(base_defense_focus 조건부) → bp:196 POS_EVAL·CAST_BEAMS → L539 LAST_STAND → L575/581 MAX_RANGE → L593 DIE_TICK",
      kind="보강", evidence="scratch tlsall.py 출력(with 인스턴스 19 · 최단 경로) + tlsreach.py(직접 콜리별 경로) · TLS 전역 목록 tlsscan 동일(m11.ll:54~87 · g11.ll)",
      behavior_change=False, found_by="new")

# ── knobs[28] bail gate 소비처 ──
p.fix(S + "/knobs[28]/effect",
      old="소비처는 이 함수 밖(미확인) — 값 변경은 관측 코드에만 영향할 가능성",
      new="소비처는 이 함수 밖 — _docs game_ai.txt L588~589 「[v3 계측·관측 전용] 직전 후보 생성이 도주 단일 후보로 조기 종료된 사유. 0=아님, 1=회피 하드(궁/risk25/CC/연타피격), 2=지는 판정(resolve_fight Disengage), 3=전대상 치명」 · 이 함수 안 +0x2d 읽기 0회(IR 전수) → 관측 전용 확정(25차 A) · 오라클 C11d 에서 0→3 실측",
      kind="보강", evidence="_docs/game_ai.txt L588~589 원문 · scratch selfw.py(+0x2d load 0건) · 오라클 o188 C11d self_diff +0x2d:0->3",
      behavior_change=False, found_by="new")

# ── notes[1] check_kill_die_tick 0 의미(오라클) ──
p.fix(S + "/notes[1]",
      old="이 배치는 분기 효과(0 → 계속, ≠0 → all_targets_lethal=false)만 확정",
      new="이 배치는 분기 효과(0 → 계속, ≠0 → all_targets_lethal=false)만 확정 · (25차 A 오라클 실측) 내 hp=0 → die_tick=0 → bail 3·truncate·RunAway 대체(C11d) / hp=1 + 적 피해 → 60 / 적 피해 0 또는 hp 충분 → 60000(불사 센티널) → Trace 유지(C11a/b) ⟹ 0 은 '즉사(이미 죽음)' 쪽이고 '처치 불가'는 60000",
      kind="보강", evidence="오라클 o188 C11a/C11b/C11d(kdt=1 로 같은 인자의 check_kill_die_tick 반환값 병기): die_tick 60000/6000/0 ↔ 결과 Trace/Trace/RunAway+bail3",
      behavior_change=False, found_by="new")

# ── G7 open[4]/[15]: 해소 문면(삭제는 산문으로 요청) ──
p.fix(S + "/open[4]",
      old="는 game_core 몫 — IR 사실만 기록",
      new="는 game_core 몫 — IR 사실만 기록 → 해소(25차 A): _docs game_core.txt L21 「Blackboard[team]은 team 팀 자체 정보 추적, 관측은 1-team」 = blackboard[1-player.team] 이 적 팀(1-team) 정보를 player.team 관점에서 추적하는 판 · is_recent_visible(g07.ll:157005) 는 game.is_visible(team,id) 참이면 즉시 true, 아니면 bb.last_visible[적 pos]+120 >= tick 으로 판정(오라클 vis=1 = last_visible[pos]=tick 로 재현)",
      kind="보강", evidence="_docs/game_core.txt L21 · _gcbc g07.ll:157005~157040 is_recent_visible 본문(vtable +0xf8 is_visible → +480 last_visible[pos] +120 uge tick) · 오라클 C11a↔C11c(vis 0/1 로 near_enemies 유무 갈림)",
      behavior_change=False, found_by="new")
p.fix(S + "/open[15]",
      old="미확인 — is_recent_visible 자식 명세(game_core) 소관",
      new="미확인 — is_recent_visible 자식 명세(game_core) 소관 → 해소(25차 A): open[4] 와 같은 물음 · _docs game_core.txt L21 로 닫음(사실 서술)",
      kind="보강", evidence="_docs/game_core.txt L21", behavior_change=False, found_by="new")

# ── ev_up: BattleSubPlan 오프셋 행(tcxdict 대조) ──
for i in (0, 1, 2, 3, 53, 54, 55, 56, 93, 94, 95, 101, 102, 105):
    p.ev(S + "/mem[%d]" % i, to=3, evidence="tcxdict BattleSubPlan --deep 대조(0x0 support_target@tag · 0x8 Some.0 · 0x10 goal@tag · 0x18 focus · 0x20 v48_claim_hold_until · 0x28 avoid · 0x29 with_dive · 0x2b v48_dodge_claim · 0x2c tactic@tag · 0x2d last_bail_gate) + IR %1 gep 전수 일치(25차 A)", found_by="reused")
# ── ev_up: 오라클 실행 확인 (knobs/consts) ──
p.ev(S + "/knobs[13]", evidence="오라클 실행 확인: C13d AroundPosition radius=24000 실측(Kiting 사거리 안 + game.is_visible)", found_by="new")
p.ev(S + "/knobs[14]", evidence="오라클 실행 확인: C7~C9·C12·C15 Trace margin=15000 실측", found_by="new")
p.ev(S + "/knobs[26]", evidence="오라클 실행 확인: die_tick 0(C11d, 내 hp=0) → RunAway+bail3 / 60·6000·60000(C11a/b) → Trace 유지", found_by="new")
p.ev(S + "/knobs[27]", evidence="오라클 실행 확인: C11d RunAway end_delay=5 실측", found_by="new")
p.ev(S + "/knobs[28]", evidence="오라클 실행 확인: C11d self +0x2d 0→3 실측 · _docs L588 관측 전용", found_by="new")
p.ev(S + "/knobs[29]", evidence="오라클 실행 확인: goal RunAway(C20)·AssassinReady(C2b) 는 근접 가시 적이 있어도 치명 판정 생략(bail 0 · 후보 그대로)", found_by="new")
p.ev(S + "/consts[42]", evidence="오라클 실행 확인: Trace 태그 14 실측(C7 등 12케이스)", found_by="new")
p.ev(S + "/consts[44]", evidence="오라클 실행 확인: AroundHide 태그 6 실측(C2/C2b)", found_by="new")
p.ev(S + "/consts[45]", evidence="오라클 실행 확인: version=1 로 bp:320~345 진입 → AroundRunAway 태그 8 실측(C21) · version=55 에선 미출현(reach 사장 판정과 정합)", found_by="new")
p.ev(S + "/consts[46]", evidence="오라클 실행 확인: C11d truncate(0) 후 push → len=1·[0]=RunAway 실측", found_by="new")
p.ev(S + "/consts[47]", evidence="오라클 실행 확인: goal End → panic 'not yet implemented' 실측(C1)", found_by="new")
p.ev(S + "/consts[48]", evidence="오라클 실행 확인: 패닉 메시지 'not yet implemented' = 19자 실측(C1)", found_by="new")
p.ev(S + "/consts[55]", evidence="오라클 실행 확인: 적 타워 50000 거리에서 Attack(15, target=tower.id) 실측(C19)", found_by="new")
p.ev(S + "/consts[56]", evidence="오라클 실행 확인: die_tick==0 만 치명(C11d) · 60/6000/60000 은 비치명(C11a/b)", found_by="new")
p.ev(S + "/consts[57]", evidence="오라클 실행 확인: bail=3 + RunAway 태그 3 실측(C11d)", found_by="new")
p.ev(S + "/consts[58]", evidence="오라클 실행 확인: L606 RunAway end_delay=5 실측(C11d)", found_by="new")
p.ev(S + "/consts[26]", evidence="오라클 실행 확인: bp:198 AroundPosition around_radius=24000 실측(C13d)", found_by="new")
p.ev(S + "/consts[27]", evidence="오라클 실행 확인: Trace attack_range_margin=15000 실측", found_by="new")

p.brief_error("도시에/프롬프트 초점 ② 가 이 함수의 sret variant 집합에 AroundRegion 7·AroundBush 12·Recall 4·Stop 19·LaneMinionPosition 13 을 넣었으나 IR 태그 store 전수(20사이트)+base_battle_action 본문(태그 15~18 뿐)으로는 RunAway 3·AroundHide 6·AroundPosition(untagged)·Trace 14·Attack~Ult 15~18(+NA AroundRunAway 8)만 도달한다 — 다른 루트(SubPlan::action_candidates 디스패치 상위)의 집합을 그대로 옮긴 것으로 보인다")
p.brief_error("도시에/프롬프트 초점 ② 의 `next_plan`(Option<BigPlan> 384B)·`attack`(Option<Input> 32B) 는 이 함수 시그니처에 없다(반환 = bumpalo Vec<SmallActionPlay> 32B 뿐) — 다른 루트 함수의 지시문이 섞였다")
p.brief_error("§5 경로 문법 안내 `/specs[<idx>]/sig/tls/<키>`(3단+키) 는 tls 가 dict 일 때만이고, #188 의 sig.tls 는 항목 2개 list 라 `/specs[188]/sig/tls[1]/<키>` 로 써야 한다(applypatch ②′ 분기는 dict 전용) — --dry 로 확인")
p.brief_error("spec_188 md 머리표 `exe | cbbdb0 (None) · None바이트 · None명령` — RVA 는 rvaname 확정인데 bytes/insns 메타가 None 으로 비어 있다(mkspec3 exe 메타 결손)")
p.brief_error("callees 표의 base_positioning 3행(Jungle/Battle/LineSafe)이 「⚠간접호출(vtable)」로 붙어 있으나 실제는 BattleSubPlan::base_positioning 이 L536 에 통째 인라인(1,839줄 · bp:144/235/247 drop · bp:354 extend · 사슬 `<536`)이라 vtable 호출이 아니다 — callees 는 v2 `calls` 문자열 배열이라 patch 로 못 고침(생성기 pick 문면 정정 필요)")
p.save()
