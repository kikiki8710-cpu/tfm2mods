# -*- coding: utf-8 -*-
import sys, io, json
sys.path.insert(0, r"C:\tfm2mods\MIG")
import mkpatch
P = "/specs[202]"
p = mkpatch.Patch(round=25, batch="I")
ORC = u"오라클 실행 확인(o202 · 케이스당 프로세스 1개 · game/mine 프로세스 분리 · 41케이스 41 MATCH · _verify25/I/oracle/run202_all.txt)"

# ── G16 sig.params[3] rnd — 콜리 define 인용에 파일을 붙여 자기 주장과 분리(게이트 F8 규약) ──
p.fix(P + "/sig/params[3]/role",
      old=u"직접 소비 0건. make_gank_battle · check_kill 에 전달만(check_kill 쪽 define 은 %2 readnone)",
      new=u"직접 소비 0건(본체 94975~98368 에서 %3 의 load/store 0건 · 본 함수 define 의 %3 속성 = noalias align 16 dereferenceable(320)). make_gank_battle(7곳, %2 로 전달 → version≥2 에서만 BattlePlan::update(m10.ll:23429, %2 = &mut StdRng)가 소비)·check_kill(97474)에 전달만 — check_kill 의 define(m13.ll:54538)은 %2 가 readnone 이라 실제로는 안 읽는다. " + ORC + u": 41케이스 전부 rnd_changed=false(version 1·2 모두, next_u64 대조)",
      evidence=u"m08.ll:94975 define 원문 `ptr noalias noundef align 16 dereferenceable(320) %3`(readnone·readonly 없음) · m13.ll:54538 check_kill define `ptr noalias readnone align 16 captures(none) %2` · m10.ll:23429 BattlePlan::update define `ptr noalias noundef align 16 dereferenceable(320) %2`. 옛 문면은 콜리 define 을 파일 없이 인용해 G16 이 본 함수의 속성 주장으로 읽었다(F8)",
      behavior_change=False, found_by="reused", kind=u"보강")

# ── sret variant 집합 확정: 이 함수가 sret 에 넣을 수 있는 variant = None(-1) · Battle(9) 뿐 ──
p.fix(P + "/sig/params[0]/role",
      old=u"make_gank_battle 경로(7곳)는 sret %0 를 그대로 넘겨 콜리가 채운다(살아있는 바이트 = make_gank_battle 명세)",
      new=u"make_gank_battle 경로(7곳)는 sret %0 를 그대로 넘겨 콜리가 채우는데 그 콜리도 +0 i64 -1(m08.ll:94430·94472) 또는 +0 i64 9 + +8..+0x120 memcpy 280B(94393~94394)만 쓴다 ⇒ ★이 함수의 sret variant 집합 = {None(-1), Battle(9)} 뿐(LineGanker 10·LineGankCover 11·PassiveLine 3·ActiveRecall 8 은 절대 안 나옴) · 살아있는 바이트 = None: [0,8) / Battle: [0,0x120) 이고 [0x120,0x180) 은 항상 alloca 잔재. Battle 페이로드 280B 안에도 BattlePlan::new 가 안 쓰는 구멍이 있다(Response 목표일 때 main_goal 페이로드 bp+0x48..+0x58 = 잔재 — 오라클 B6b: 0xAA 프리필이 sret+84·+92 에 살아남음) — 그 내부 맵은 BattlePlan structlive 소관. " + ORC + u": None 케이스 기록 범위 [(0,8)] · Battle 케이스 [(0,288)] 실측(0xAA 프리필 raw-sret 호출)",
      evidence=u"m08.ll:94394 `store i64 9, ptr %0` · 94430/94472 `store i64 -1, ptr %0` · 94393 memcpy 280B (make_gank_battle 전 반환 지점) · 본체 95059/97158/98207 `store i64 -1` · 97034 `store i64 9` · 97036 memcpy 280B. 오라클 o202 who=game runs 필드",
      behavior_change=False, found_by="new", kind=u"보강")

p.fix(P + "/sig/ret",
      old=u"Option<BigPlan>(sret) — None(-1) / Some(Battle(BattlePlan{goal Response, entry_src 3})) / make_gank_battle 의 반환 그대로",
      new=u"Option<BigPlan>(sret) — None(-1) / Some(Battle(BattlePlan{goal Response(태그 2), entry_src 3})) / make_gank_battle 의 반환 그대로(= None 또는 Battle{goal TryKill(target, min_trace) 태그 0 · entry_src 7}) — variant 는 None·Battle 두 가지뿐",
      evidence=u"m08.ll:94378 `store i64 0, ptr %11`(TryKill 태그) · 94382/94452 `store i8 7`(entry_src=7) · 94394 `store i64 9`. 오라클: Battle 결과의 +8+0x40 goal_tag ∈ {0,2} · +8+0x107 ∈ {7,3}",
      behavior_change=False, found_by="new", kind=u"보강")

# ── one_line: else 분기는 '열세' 만이 아니라 근접 적 0 도 포함 ──
p.fix(P + "/one_line",
      old=u"노출·인원우위/열세 대응",
      new=u"노출(적에게 보임+정글 영역) 시 광역 인원우위(아군>적 && 적≠0)면 최근접 가시 적에 make_gank_battle, 아니면(열세·동수·근접 적 0 포함) 가시 적이 하나라도 있으면 즉시 Battle(Response, entry_src 3)+BattleHelp 챗",
      evidence=u"m08.ll:96437~96440 `ugt %443,%645 & ne %645,0` → 705(L124) / 649(L132 else) · 오라클 B3(근접 적 0 · 400000 떨어진 가시 적 1) → tag 9 entry_src 3 chats push(6,23,0) 실측",
      behavior_change=False, found_by="new", kind=u"보강")

# ── consts[7] 60 min_trace: 저장처 확정 ──
p.fix(P + "/consts[7]/meaning",
      old=u"(=1초@60tps 추정, 콜리 명세 참조)",
      new=u"(=1초@60tps 추정, 콜리 명세 참조 — 실측: make_gank_battle 이 BattlePlan::new(goal=TryKill(target, 60)) 로 넘겨 BattlePlan.main_goal(+0x40 태그 0)·+0x50 에 60 이 그대로 저장된다, 오라클 E1/E2b/E3/E4b/F2 min_trace=60)",
      evidence=u"m08.ll:94375~94378 `store i64 %7 (+8)` `store i64 %8 (+16)` `store i64 0` → BattlePlan::new goal 인자 · 오라클 GAME 행 min_trace=60",
      behavior_change=False, found_by="new", kind=u"보강")

# ── ev 상향(오라클로 실행 확인된 행) ──
def ev(path, text, to=2, frm=4, fb="new"):
    p.ev(P + path, evidence=u"오라클 실행 확인: " + text + u" (o202 · 프로세스 분리 · run202_all.txt)", to=to, frm=frm, found_by=fb)

ev("/sig/params[1]", u"Response 케이스(B2/B3/B5/B6b/B8) self 48B diff = chats cap(+0)·ptr(+8)·len(+0x10) 뿐, 그 외 케이스 diff 없음 · chat0=(6, enemy.id, 0)")
ev("/sig/params[3]", u"41케이스 rnd_changed=false")
ev("/sig/params[4]", u"team 0·Jungle(slot 1) 로 player_champion[0][1] 이 champ 로 잡힘(world 행 champ=19)")
ev("/sig/params[8]", u"ctx.debug=1 에서 texts 1건(bush 텍스트, y+20000, Color{1,0,0,1}, 1)·infos[e.id] 1건/후보 실측, debug=0 케이스 0건")
ev("/mem[0]", u"phase=ChangeJungle(태그 0) → None(A1) / Setup(태그 7) → 진행(A2)", to=3)
ev("/mem[2]", u"wait_limit-tick = 181 → L218 참(E4 None) / 180 → 거짓(E4b L221 Battle)", to=3)
ev("/mem[21]", u"champ 셀 (17,16)=bush 14 에서 champ_bush 14 == bush 14 → 진행(C2), 정글 셀 (13,20)=0 ≠ 14 → None(C1)", to=3)
ev("/mem[27]", u"적 hp 39/max 100 → 인원 제외(B5 Response) / 40 → 포함(B5b L127)", to=3)
ev("/mem[28]", u"B5/B5b hp 39↔40 갈림", to=3)
ev("/mem[29]", u"e0 move_speed 10000 → move_to_tower_tick 17(=173850/10000) (D3), 1 → 173850 (D2)", to=3)
ev("/mem[36]", u"attack_range 20000 = radius 10000+10000(mult 0) · d=20000 → L241 / 20001 → L240 (F2/F3)", to=3)
ev("/mem[37]", u"F2/F3 사거리 경계 20000/20001", to=3)
ev("/mem[41]", u"Response 케이스에서만 chats len 0→1, 원소 (i8 6, enemy.id, 0)", to=3)
ev("/mem[43]", u"debug=1 C1: texts=(432000, 676000, \"bush: 14, champ_bush: 0\", Color{r:1,g:0,b:0,a:1}, 1)", to=3)
ev("/mem[44]", u"debug=1 D1: infos[19] = \"target: 23, near_allies_p: [1], near_enemies_p: [0], move_to_tower_tick: 173850, me_die_tick: 9999999999, enemy_die_tick: 9999999999\"", to=3)
ev("/consts[0]", u"A1(ChangeJungle 태그 0<6 → None) / A2(Setup 태그 7 → 진행)", frm=3)
ev("/consts[1]", u"B2 goal_tag=2(Response)", frm=3)
ev("/consts[2]", u"적 거리 250000 → 셈(B6 L127) / 250001 → 안 셈(B6b Response)")
ev("/consts[4]", u"hp% 39 → 제외 / 40 → 포함 (B5/B5b)")
ev("/consts[5]", u"B2 entry_src=3")
ev("/consts[6]", u"Battle 케이스 sret+0 = 9", frm=3)
ev("/consts[7]", u"min_trace=60 이 BattlePlan+0x50 에 그대로(E1/E2b/E3/E4b/F2)")
ev("/consts[8]", u"champ (560000,528000) → 셀 (17,16) champ_bush=14 (C2)")
ev("/consts[10]", u"texts y = 656000+20000 (C1)")
ev("/consts[11]", u"적 거리 150000 → near_enemies_p 포함(E8 L212) / 150001 → 제외(E8b L240)")
ev("/consts[12]", u"None 케이스 sret+0 = -1(0xFFFF…) · get_die_tick_player 6번째 인자 None 으로 die_tick 산출")
ev("/consts[14]", u"enemy_die_tick 8 → L205 통과(E1) / 250006 & 1<=1 → continue(E2)")
ev("/consts[15]", u"mtt 17 → L211 분기(D3/E1) / 173850 → L216 분기(D2/E3)")
ev("/consts[16]", u"edt 8 → L212(E1) / 203346 → L213→L214(E2b)")
ev("/consts[17]", u"mdt 9999999999 → L217(E3) / 0 → L218(E4)")
ev("/knobs[0]", u"B6/B6b 250000↔250001")
ev("/knobs[1]", u"B5/B5b 39↔40")
ev("/knobs[2]", u"E8/E8b 150000↔150001")
ev("/knobs[3]", u"E1(8)↔E2(250006, continue)")
ev("/knobs[4]", u"D3(17)↔D2(173850)")
ev("/knobs[5]", u"E1(8 → L212)↔E2b(203346 → L214)")
ev("/knobs[6]", u"E3(9999999999 → L217)↔E4(0 → L218)")
ev("/knobs[7]", u"E4(rem 181 > 180 → 아무것도)↔E4b(rem 180 → L220→L221)")
ev("/knobs[8]", u"min_trace 60 이 BattlePlan.main_goal(+0x50) 에 그대로 저장(E1/E2b/E3/E4b/F2)", frm=5)
ev("/knobs[9]", u"B2/B3 entry_src=3")

# tls 는 tlsreach 결과 반영 후 mkp_tls.py 에서 추가 — 여기서는 import 로 합친다
import mkp_tls
mkp_tls.add(p, P, ORC)

p.brief_error(u"지시문 초점 ②가 `action_candidates`(bumpalo Vec<SmallActionPlay> 32B · variant RunAway/Around…)와 `attack`(Option<Input> 32B)의 live 바이트를 요구하는데 이 배치(202 next_plan)의 sret 은 Option<BigPlan> 384B 하나뿐이다 — 다른 배치 지시가 섞였다(문안이 r16 루트 공통 템플릿)")
p.brief_error(u"§4 G7 과열림 인용문이 `shared.is_recent_visible … 확정` 이라 하는데 open[1] 의 물음은 「self 가 왜 blackboard[1-team] 인가」(의미)이지 인덱스 사실이 아니다 — 닫히는 근거는 오라클(bb[1].last_visible[i]=tick 으로 적 슬롯 i 가 recent_visible) 이며 게이트 문면만으로는 닫을 수 없다. open/notes 는 patch 로 못 고치니 아래 산문대로 메인이 옮겨야 한다")
p.brief_error(u"지시 ⑥ 「SubPlan self 와 ScoreParameter 는 조립 가능」은 이 함수(self=LineGankerPlan · ScoreParameter 미사용)엔 해당 없음 — LineGankerPlan::new_with_phase(pub)·TeamPlan/PositioningScoreData/DebugFrameData 의 Default(pub) 로 조립했다(도시에가 이 조립 경로를 안 알려줌)")
p.save()
