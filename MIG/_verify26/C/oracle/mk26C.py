# -*- coding: utf-8 -*-
"""26차 배치C patch.json 생성 (mkpatch 참조구현 사용). 실행: python -X utf8 mk26C.py"""
import sys, io
sys.path.insert(0, r"C:\tfm2mods\MIG")
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
import mkpatch

p = mkpatch.Patch(round=26, batch="C")
S = "/specs[206]"
ORC = u"오라클 o206.exe(_verify26/C/oracle · 케이스당 프로세스 1개 · 59케이스 · o206_cases.log)"

# ── G12 src_line: 자기 파일(passive_line.rs) 줄 규약 위반 4건 — 다른 consts 행(35·41·45·55 등)은 전부 자기 파일 줄을 쓴다
p.fix(S + "/consts[7]/src_line", old=704, new=1171,
      evidence=u"m04.ll:20008 `%430 = mul i64 %429, 30, !dbg !33000` · !33000 = DILocation(line 704, scope !32318=GameSetting::is_line_phase **setting.rs**:702, inlinedAt !32326) · !32326 = line 1171(passive_line.rs) < !31524 = 222. 704 는 setting.rs 줄이라 자기 파일 규약 위반 — 자기 파일 최내측 = 1171(둘째 사이트 21035 `select i1 %842, i64 60, i64 30` !34053 = 1400)",
      behavior_change=False, found_by="reused")
p.fix(S + "/consts[10]/src_line", old=1655, new=1248,
      evidence=u"m04.ll:20454 `%606 = icmp eq i64 %605, 13, !dbg !33517` · !33517 = line 1655 scope !32124=Entity::return_time(**entity.rs**:1654) inlinedAt !32132 = line 1248(passive_line.rs) < 222. 1655 는 entity.rs 줄 — 자기 파일 최내측 = 1248",
      behavior_change=False, found_by="reused")
p.fix(S + "/consts[11]/src_line", old=1701, new=1193,
      evidence=u"m04.ll:20260 `%540 = icmp ugt i64 %538, 4, !dbg !33366` · !33366 = line 1701 scope !33367=Entity::ult_effect(**entity.rs**:1700) → !33370 line 1193(passive_line.rs) → !33296 line 332(iterator any) → !31552 line 1179 → 222. 1701 은 entity.rs 줄 — 자기 파일 최내측 = 1193(둘째 사이트 21018 `icmp eq i64 %831, 4` !34051 = 1396)",
      behavior_change=False, found_by="reused")
p.fix(S + "/consts[25]/src_line", old=742, new=1129,
      evidence=u"m04.ll:19733 `%320 = icmp eq i32 %319, -1, !dbg !32710` · !32710 = line 742 scope !32703=`as_ref<Effect>`(**core option.rs**) inlinedAt !31760 = line 1129(passive_line.rs) < 222. 742 는 option.rs 줄 — 자기 파일 최내측 = 1129",
      behavior_change=False, found_by="reused")
# ── G12 오탐 2건: 호이스트 아티팩트 — panelty_score 두 select 가 item_score(L1411) 의 select 사슬에 병합돼 !dbg 가 1411 을 가리킨다
G12_HOIST = (u" — ★G12 오탐: 21043 `%848 = select i1 %757, i64 -10, i64 -5, !dbg !34055` 의 !34055 = line **1411**(item_score 줄)이지만 이는 SimplifyCFG 가 같은 조건(%755/%757)을 쓰는 L1411 item_score 와 L1422 panelty_score 의 if-사슬을 한 select 묶음(21041~21045)으로 호이스트한 아티팩트. −10/−5 는 item_score 값(0/5/30/60/70/100·/2)이 될 수 없고, 바로 뒤 21046 `#dbg_value(i64 poison, !31518)` 의 !31518 = DILocalVariable(name: \"panelty_score\", line: **1422**, scope !31519 = DILexicalBlock line 1422) 가 선언 줄을 준다(값이 poison 이라 srclinecheck 의 DBGVAL 정규식이 못 잡는다). src_line 1422 유지")
p.fix(S + "/consts[20]/meaning", old=u"panelty_score: 적이 라인 리드 → −10 (21043)",
      new=u"panelty_score: 적이 라인 리드 → −10 (21043)" + G12_HOIST,
      evidence=u"m04.ll:21041~21046 원문 · !34055 · !31518 · !31519 (위 문면에 인용)", behavior_change=False, found_by="reused", kind=u"오탐")
p.fix(S + "/consts[21]/meaning", old=u"panelty_score: 양쪽 리드 없음 → −5 (21043)",
      new=u"panelty_score: 양쪽 리드 없음 → −5 (21043)" + G12_HOIST,
      evidence=u"m04.ll:21041~21046 원문 · !34055 · !31518 · !31519 (위 문면에 인용)", behavior_change=False, found_by="reused", kind=u"오탐")
# ── G19 knobs[0]: IR 줄번호 오기(20513 → 19513). 46 리터럴은 19513 에만 있다
p.fix(S + "/knobs[0]/where", old=u"passive_line.rs:1096 (m04.ll:20513)",
      new=u"passive_line.rs:1096 (m04.ll:19513 `%225 = icmp ult i64 %120, 46`)",
      evidence=u"m04.ll:19513 `%225 = icmp ult i64 %120, 46, !dbg` (;L1096<222) · 20513 은 `%628 = load i32, ptr %627`(L742<1268 vamp 로드)이라 46 이 없다 — G19 「관측 못 찾음」의 원인은 줄번호 오기",
      behavior_change=False, found_by="reused")
p.fix(S + "/consts[2]/meaning", old=u"hp_ratio < 46 이면 웨이브 피해(≠0)만으로 귀환 확정 (20513)",
      new=u"hp_ratio < 46 이면 웨이브 피해(≠0)만으로 귀환 확정 (19513 `icmp ult i64 %120, 46` · 19514 `icmp uge %222, %114` 와 or)",
      evidence=u"m04.ll:19513~19516 원문", behavior_change=False, found_by="reused")
# ── G19 knobs[15] 오탐: 값이 식(박스)이라 리터럴 대조 불가 — 접힌 리터럴 891999/896000 은 인용 줄에 실재
p.fix(S + "/knobs[15]/effect", old=u"값은 게임 상수 접힘이라 소스 수준에서는 heal_area+마진",
      new=u"값은 게임 상수 접힘이라 소스 수준에서는 heal_area+마진 — ★G19 오탐: value 가 식(박스)이라 리터럴 대조 자체가 안 된다. 접힌 리터럴은 인용 줄에 실재(21305 `icmp ugt i64 %942, 891999` · 21328 `icmp ult i64 %953, 896000` · 21298/21323 `select i1 %939, i64 64000, i64 960000`). 박스 식은 IR 21297~21333 재독해로 재확인: x∈[rx-68000,rx] = (team0 ‖ x>891999) && x<=rx · y∈[ry-64000,ry] = !(y>ry ‖ (team0 && y<896000)) · 오라클 E2(분수 (15000,913000) 에서 v46_clear 발화) 일치",
      evidence=u"m04.ll:21297~21333 원문 · " + ORC + u" E2", behavior_change=False, found_by="reused", kind=u"오탐")
# ── 파일명 오기: minion_state 는 blackboard.rs (ai_interface.rs 아님)
p.fix(S + "/knobs[3]/where", old=u"player.rs:704 is_line_phase (20008)",
      new=u"setting.rs:703~704 GameSetting::is_line_phase 인라인 (20003~20008 `mul i64 %429, 30`)",
      evidence=u"!33000 = DILocation(line 704, scope !32318) · !32318 = DISubprogram is_line_phase linkageName `_RNvMs2_NtCs97f5S1uJLkH_9game_core7settingNtB5_11GameSetting13is_line_phase` file !32319 = game-core\\src\\setting.rs:702 — player.rs 가 아니다(consts[35]·knobs[14] 는 이미 setting.rs)",
      behavior_change=False, found_by="new")
p.fix(S + "/mem[35]/note", old=u"ai_interface.rs:379~382 minion_state(line) 스위치",
      new=u"blackboard.rs:378~382 Blackboard::minion_state(line) 스위치",
      evidence=u"19446~19450 switch !dbg !32529 = DILocation(line 379, scope !32186) · !32186 = DISubprogram minion_state linkageName `_RNvMs0_NtNtNtCs97f5S1uJLkH_9game_core10simulation4game10blackboardNtB5_10Blackboard12minion_state` file !23956 = game-core\\src\\simulation\\game\\blackboard.rs line 378 · tcx game_core::Blackboard::minion_state sp = blackboard.rs:378",
      behavior_change=False, found_by="new")
p.fix(S + "/logic", old=u"minion_state 스위치 ai_interface.rs:379~382",
      new=u"minion_state 스위치 blackboard.rs:378~382",
      evidence=u"위 mem[35] 와 같은 근거(!32186 · !23956)", behavior_change=False, found_by="new")
# ── 보강: &mut debug 쓰기 표면(add_log 5 사이트 전수) · &mut rnd(has_line_lead readnone) · ret 문면 · TeamPlan None 니치
p.fix(S + "/mem[122]/note", old=u"직접 store 없음(오프셋 0x48 은 tcxdict DebugFrameData.logs — 본문엔 gep 없음)",
      new=u"직접 store 없음(오프셋 0x48 은 tcxdict DebugFrameData.logs — 본문엔 gep 없음) · ★함수 전체 %7 사용 7건 전수 = add_log 5 사이트(21182 L1162 V46BASEHEAL · 22331 :647 V46TRIG · 22446 :538 V46CLR · 23608 :468 V46FLEE · 24087 :390 V46FLEECLR — 전부 ctx.debug(0x3b) 게이트 안) + has_line_lead 2회(20847·20852 — 그 define 의 debug 인자는 `readnone captures(none)` 이라 쓰기 0) · 오라클 dbg=0 59케이스 debug.logs 0",
      evidence=u"m04.ll 본문 `%7` 비-dbg 참조 7줄(20847·20852·21182·22331·22446·23608·24087) · `define … has_line_lead(… ptr noalias noundef readnone align 8 captures(none) dereferenceable(224) %5)` · " + ORC,
      behavior_change=False, found_by="new", kind=u"보강")
p.fix(S + "/mem[123]/note", old=u"19270·19275 · H 범위 직접 쓰기 0",
      new=u"19270·19275 · 함수 전체 직접 쓰기 0 · has_line_lead(20847·20852)에도 %2 를 넘기지만 그 define 의 rnd 인자는 `readnone`(쓰기 0) — 쓰기 가능 콜리는 buy_item(19270 · rnd 인자에 readonly 없음)·upgrade_item(19275) 둘뿐 · 오라클 59케이스(item_list 빈) rnd 소비 0",
      evidence=u"`define hidden { i64, i64 } @…buy_item(i64 %0, ptr noalias noundef align 16 dereferenceable(320) %1, …)` · `define … upgrade_item(… ptr noalias noundef align 16 dereferenceable(320) %2 …)` · has_line_lead define `ptr noalias noundef readnone align 16 captures(none) dereferenceable(320) %1` · " + ORC + u" rnd_used=false 59/59",
      behavior_change=False, found_by="new", kind=u"보강")
p.fix(S + "/sig/ret", old=u"살아있는 바이트 = 그 1바이트.",
      new=u"판정 출력 바이트 = 그 1바이트. 단 호출자에게 살아남는 &mut self 쓰기는 in_recall 만이 아니다 — writes 표 39행 전부(v46_commit 0x111 · 도주 상태 0x0/0x8/0x10/0x100/0x108/0x112~0x115 · 계측 카운터 0x90~0xf8 · Vec 5개 = chats 0x18·v46_committers 0x30·v46_flee_threats 0x48·trigger_ticks 0x60·flee_episodes 0x78 — 원소는 힙). IR store 전수(self 직접 파생 store 73건 + 힙 원소 store 22건(trigger_ticks 2 · flee_episodes 3×5 · chats 3+2) + memcpy 24B 2 + grow_one 8 + drop_in_place<Vec<usize>> 2)와 writes 표 대조 불일치 0.",
      evidence=u"scratchpad selfwrites.py(m04.ll 19038~24726 %0 파생 store 전수) ↔ writes[0..38] · " + ORC + u" self 280B 전후 diff",
      behavior_change=False, found_by="new", kind=u"보강")
p.fix(S + "/mem[42]/note", old=u"tcxdict --enum MainObjective: Gank=8 Dive=9 (메모리태그=선언discr)",
      new=u"tcxdict --enum MainObjective: Gank=8 Dive=9 (메모리태그=선언discr) · Option None 니치 태그 = 255(0xFF) — 오라클 TeamPlan::default() 의 +0x41f 실측 255 · `and i8 -2` 후 254 ≠ 8 이라 None 은 Gank/Dive 분기에 안 걸린다",
      evidence=ORC + u" world 줄 obj_tag=255(obj=none) · 8(gank) · 9(dive)", behavior_change=False, found_by="new", kind=u"보강")

# ── ev 상향: 오라클 진리표로 실행 확인된 상수·노브·쓰기
EV3 = {"/consts[28]", "/consts[47]", "/consts[56]", "/mem[158]", "/mem[159]"}
def ev(path, ev_text, to=2):
    p.ev(S + path, evidence=u"오라클 실행 확인: " + ev_text + u" (" + ORC + u")", to=to, frm=(3 if path in EV3 else 4), found_by="new")
ev("/consts[6]", u"B9 hp70→in_recall 1(heal 60) · B10 hp71→0 (적 5명 가시·타워 옆) — L1342 `hp>70` 경계")
ev("/consts[13]", u"B9 hp70→1: heal_score 60 + panelty(≥−10) > 49")
ev("/consts[14]", u"B14 정글(me=1) hp80→1 · B13 hp81→0 — L1368 `hp>80 ? 0 : 80`")
ev("/consts[15]", u"H1 적타워 표적중(0x88=1) hp30→1 · H2 hp31→0 (L1261) · B5/B6 hp30/31 heal 80/70")
ev("/consts[24]", u"B1 hp10→1 · B2 hp11→0 (분수 밖·적 없음·L1439)")
ev("/consts[27]", u"C1 gank(8)·C4 dive(9) 둘 다 gank_hold 경로 — `and -2` 마스크")
ev("/consts[28]", u"C1 gank(8)·C4 dive(9)·obj=none(255) 분기 일치")
ev("/consts[30]", u"C1 hp25→gank_hold(in_recall 0·chats 0) · C2 hp24→Cancel 채팅 push")
ev("/consts[35]", u"E4 tick=34199(=36000−1800−1) v46_commit 유지 · E5 tick=34200 v46_clear — is_line_phase 경계")
ev("/consts[36]", u"G6 위협 거리 250000 → still(도주 유지) · G5 250001 → 도주 종료 episode push")
ev("/consts[46]", u"D9 dive 도 인접 라인 HideLineToo — 마스크")
ev("/consts[47]", u"D1 gank · D9 dive 동일 분기")
ev("/consts[48]", u"D4 Mid←Top 인접 · D5 Mid←Bottom 인접 · D8 Bottom←Mid 인접 · D6 Top←Bottom 비인접 · D7 Bottom←Top 비인접")
ev("/consts[49]", u"D4 self=Mid gank=Top → 인접 참")
ev("/consts[50]", u"D5 self=Mid gank=Bottom → 인접 참")
ev("/consts[52]", u"D1 hp70→HideLineToo · D2 hp69→유지 (타워 Some 경로 24574)")
ev("/consts[53]", u"D1/D2 와 같은 경계 70 (타워 None 경로는 미도달 — 세계에 타워가 항상 있다)")
ev("/consts[54]", u"C1 hp25→in_recall 0 · C2/C3 hp24→Cancel(LowHpSelf)")
ev("/consts[56]", u"D1/D4/D5/D8/D9 chats[0] = [12, gank_line, .., +8=0] 바이트 실측")
ev("/consts[57]", u"C2/C3 chats[0] = [17, 0, ..] 바이트 실측")
ev("/consts[58]", u"C2 +1=0(LowHpSelf) · D1 +8=0 실측")
ev("/knobs[6]", u"H1 hp30→1 · H2 hp31→0")
ev("/knobs[9]", u"B1 hp10→1 · B2 hp11→0")
ev("/knobs[12]", u"비정글 적우세: 30→80 · 31/50→70 · 51/70→60 · 71→0 / 정글 적우세: 80→80 · 81→0 (B5~B14)")
ev("/knobs[13]", u"C1 hp25 gank_hold · C2 hp24 Cancel")
ev("/knobs[14]", u"E4/E5 tick 34199/34200 경계")
ev("/knobs[16]", u"G5/G6 250001/250000 경계")
ev("/knobs[21]", u"C1/C2")
ev("/knobs[22]", u"D1/D2")
for i, t in [(121, u"전 59케이스 0x110 전후 diff — 분수 안 hp<max→1 · 만피→0 · 커밋중 L245→1 · L265/L297→0"),
             (124, u"E2 분수(in_heal)·E3 라인국면 종료·C6 gank_hold → 0x111 1→0 · E1 커밋 유지 1"),
             (125, u"C6/E2/E3 v46_committers.len 0 유지(cap/ptr 불변)"),
             (129, u"F1 (100,Top,3)=approached|2 · F2 (100,Top,4) · G2/G5 (100,Top,0) 원소 바이트 실측"),
             (130, u"F1/F2/G2/G5 0x88 0→1"),
             (140, u"G1/G4/G6 0xd8 0→1"), (141, u"G1 0xe0 0→1 · G4 acute 아님→0 유지"),
             (142, u"G1/G4 0xe8 0→1(내 타워 사거리 안)"), (144, u"F1 hp_min==hp_entry → 0xf8 +1 · F2 hp_min<hp_entry → 0"),
             (146, u"G1 hp_min 600→500(=min(hp,hp_min)) · G7 400 유지"), (147, u"F1/F2/G2 take → 0x0 1→0"),
             (150, u"F1/F3/G2 0x112 1→0 (i32 store 로 0x113~0x115 동시 0)"), (151, u"G1 acute → 0x113 1 · G4 → 0"),
             (152, u"G1 0x114 →1 · F1 1→0"), (155, u"G2 0x58 2→0 (threats clear)"),
             (156, u"C2/D1 0x28 0→1"), (158, u"D1 [12,1,..,+8=0] · D5 [12,2,..] 실측 · +2..+7 은 힙 잔재(미기록 확인)"),
             (159, u"C2 [17,0,..] 실측 · +2.. 힙 잔재")]:
    ev("/mem[%d]" % i, t, to=3)

p.brief_error(u"§1 지시문(v26_prompt_C)의 ②sret 절은 이 배치 대상(void · sret 없음)에 해당 항목이 없다 — 배치 공통 문안이 그대로 실려 「ScoreParameter 5,384B …」 등 남의 함수 재료를 읽게 만든다. 배치별 dossier 에는 그 함수의 sret 타입만 남겨라.")
p.brief_error(u"§4 G12 규약 문면 「그 함수 소스 루트 줄」은 검사기(srclinebase._ownlines)와 다르다 — 실제 판정은 「사슬 안 **자기 파일(passive_line.rs)** 줄 중 하나」이고 루트(222)만이 아니다. 이 함수는 전부 L222 인라인이라 「루트 줄」로 읽으면 62행 전부 222 가 돼야 한다.")
p.brief_error(u"_verify3/TEMPLATE.rs real_setting() 에 `epic_jungle.first_spawn_tick`(0x8a8) 이 없어 default 0 → `is_line_phase` 가 항상 false(v46 커밋·도주 판단이 전부 v46_clear/flee_end 로 접힌다). 이 프로브는 0x8a8 에 36000 을 직접 썼다(실전값 미상). 템플릿 44줄에 epic_jungle 하위 필드를 추가해야 라인 국면 함수가 갈린다.")
p.brief_error(u"§4 G7 「open 이 shared.is_recent_visible 를 부른다」 2건은 open[18]·open[20] 인데 closed[2](배치 I 의 같은 물음)도 같은 shared 사실로 닫힌다 — 게이트가 closed[] 는 안 본다.")
p.save()
