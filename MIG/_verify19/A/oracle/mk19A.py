# -*- coding: utf-8 -*-
"""19차 배치A patch.json 생성 (mkpatch 참조구현 사용). 실행: python -X utf8 _verify19/A/oracle/mk19A.py"""
import sys, io
sys.path.insert(0, r"C:\tfm2mods\MIG")
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
import mkpatch

p = mkpatch.Patch(round=19, batch="A")
O1 = u"오라클 실행 확인(_verify19/A/oracle/v19A_o1.rs · v19A_o1.tsv, 17/17 MATCH, setting_ok=true): "

# ══ G12 ══════════════════════════════════════════════════════════════
# [78] consts[6] — 오탐
p.fix("/specs[78]/consts[6]/src_line", old=433, new=433, kind=u"오탐",
      evidence=(u"명세가 옳다. m04.ll:59888 `%128 = sdiv i64 %116, 6, !dbg !69047`(!69047 = DILocation line 433 in objective_defense_role) 직후 "
                u"59889 `#dbg_value(i64 %128, !68470, !DIExpression(DW_OP_plus_uconst, 1, DW_OP_stack_value), !69048)` · "
                u"!68470 = DILocalVariable(name: \"needed_dps\", line: 433). 리터럴 1 은 DIExpression 안(DW_OP_plus_uconst)에만 있어 "
                u"srclinecheck 가 못 본다 — 게이트 보강안: `#dbg_value` 의 `DW_OP_plus_uconst N`/`DW_OP_constu N` 을 그 DILocalVariable.line 의 약한 후보로"),
      behavior_change=False, found_by="reused")

# [83] consts[9]/[10] — 실오류 107→106 (리터럴 자체의 사슬)
ev = (u"m10.ll:8180 `%149 = select i1 %148, i64 64000, i64 960000, !dbg !20297` · 8198 `select i1 %148, i64 960000, i64 64000, !dbg !20297`; "
      u"dloc !20297 = heal_area game.rs:319 ← check_recall hunt_and_poke.rs:106 ← sub_plan:79. 107 은 그 값을 레지스터로 비교하는 줄"
      u"(8189 `icmp ule i64 %151, %149, !dbg !20299`=107)이라 리터럴 사슬에 없다. consts[11]/[12](891999·896000)는 107 에 리터럴이 직접 있어 그대로 맞다")
p.fix("/specs[83]/consts[9]/src_line", old=107, new=106, evidence=ev, behavior_change=False, found_by="reused")
p.fix("/specs[83]/consts[10]/src_line", old=107, new=106, evidence=ev, behavior_change=False, found_by="reused")
p.fix("/specs[83]/consts[9]/meaning", old=u"[game_core heal_area(game.rs:319) 인라인]",
      new=u"[game_core heal_area(game.rs:319) 인라인 — :106 에서 (x0,y0,x1,y1) 취득, :107 에서 비교]",
      evidence=ev, behavior_change=False, found_by="reused", kind=u"보강")

# [83] consts[16] — 실오류 148→149 (closure$1:149 / closure$2:155)
ev = (u"m10.ll:8622 `%304 = icmp eq i64 %302, 7, !dbg !20881`(5회 언롤 8622/8679/8737/8795/8853) · dloc !20881 = closure$1 hunt_and_poke.rs:149 ← … ← check_recall:151 ← sub_plan:79; "
      u"아군 쪽 8962 `icmp eq i64 %411, 7, !dbg !21125` = closure$2:155 ← check_recall:157. 148 은 사슬에 없다")
p.fix("/specs[83]/consts[16]/src_line", old=148, new=149, evidence=ev, behavior_change=False, found_by="reused")
p.fix("/specs[83]/knobs[6]/where", old=u"hunt_and_poke.rs:148", new=u"hunt_and_poke.rs:149 (아군 쪽 :155)",
      evidence=ev, behavior_change=False, found_by="reused", kind=u"보강")
p.fix("/specs[83]/mem[18]/note", old=u"(:148, :154)", new=u"(:149 closure$1, :155 closure$2)",
      evidence=ev, behavior_change=False, found_by="reused", kind=u"보강")

# [83] consts[21] — 실오류 89→87 + logic/mem[34] 줄 주석 교정
ev = (u"m10.ll:9295 `%544 = icmp ult i64 %543, 22500000001, !dbg !19668`(:86) · 9296 `br i1 %544, label %549, label %547`; "
      u"%549 = 9311 `store i64 12, ptr %0, !dbg !21204`(dloc = sub_plan:87 = EpicPoke) · %547 = 9307 `store i64 10, ptr %0, !dbg !21203`(:89 = EpicCheck). "
      u"명세 logic 은 :87/:89 를 서로 바꿔 적었다(판정 방향은 맞다)")
p.fix("/specs[83]/consts[21]/src_line", old=89, new=87, evidence=ev, behavior_change=False, found_by="reused")
p.fix("/specs[83]/logic", old=u"if dist_sq(champ, epic) < 22500000001 → return EpicPoke                             // :86, :89",
      new=u"if dist_sq(champ, epic) < 22500000001 → return EpicPoke                             // :86 → :87",
      evidence=ev, behavior_change=False, found_by="reused")
p.fix("/specs[83]/logic", old=u"else → return EpicCheck{false}                                                       // :87",
      new=u"else → return EpicCheck{false}                                                       // :89",
      evidence=ev, behavior_change=False, found_by="reused")
p.fix("/specs[83]/mem[34]/note", old=u":44 :51 :83 :87 전부 0", new=u":44 :51 :83 :89 전부 0",
      evidence=ev, behavior_change=False, found_by="reused")

# ══ G5/G16 [80] self 가변성 ═══════════════════════════════════════════
ev = (u"m13.ll:33742 define … try_engage(… `ptr noundef nonnull align 8 %1` …) — %1 에 noalias·readonly·dereferenceable 이 전부 없다. "
      u"같은 타입의 `&self` 메서드 m13.ll:10112 eo_cover_picks(`ptr noundef nonnull align 8 %0`)와 동일, `&mut self` 인 10120 update_on_dead 는 "
      u"`ptr noalias noundef align 8 dereferenceable(6168) %0`. tcx 정본 sig 도 `&LegacyPlanHandler`. "
      u"콜리 계약도 일치: 33865 update(…, `ptr noundef nonnull align 8 %70`(team_plan), …) 의 tcx sig = `&TeamPlan`; "
      u"33949 tower_dive_is_viable(… `ptr noundef nonnull align 8 %113` …) 의 define(fight_model) %4 도 `ptr noundef nonnull align 8` = `&TeamPlan`. "
      u"⟹ self 는 `&self`, team_plan 은 `&`(TeamPlan/LegacyPlanHandler 가 !Freeze 라 noalias/readonly 가 안 붙는 공유참조)")
p.fix("/specs[80]/sig/params[1]/type", old=u"&mut LegacyPlanHandler", new=u"&LegacyPlanHandler",
      evidence=ev, behavior_change=False, found_by="reused")
p.fix("/specs[80]/sig/params[1]/role",
      old=u"본문에서 self 필드 store 없음(읽기: 0x1480·0x1490·0x1498·0x517~0x519·&0x990·&0xf8). ⚠BattlePlan::update 에 &mut team_plan(0xf8, non-readonly) 을 넘기므로 콜리 내부 write 가능성은 범위 밖",
      new=u"`&self`(tcx·IR 정합 — %1 = `ptr noundef nonnull align 8`, noalias·readonly 없음 = !Freeze 타입의 공유참조; eo_cover_picks(&self) 동일, update_on_dead(&mut self) 는 noalias dereferenceable(6168)). 본문 self store 0건(읽기: 0x1480·0x1490·0x1498·0x517~0x519·&0x990·&0xf8). team_plan(0xf8) 은 `&TeamPlan` 으로 tower_dive_is_viable·update 에 전달",
      evidence=ev, behavior_change=False, found_by="reused")
p.fix("/specs[80]/logic", old=u"try_engage(&mut self, version, rnd, player, data, target_id, debug) -> Option<BattlePlan>",
      new=u"try_engage(&self, version, rnd, player, data, target_id, debug) -> Option<BattlePlan>",
      evidence=ev, behavior_change=False, found_by="reused")
p.fix("/specs[80]/logic", old=u"if !tower_dive_is_viable(version, rnd, player, data, &mut self.team_plan, t, true, debug) { return None }   // L70",
      new=u"if !tower_dive_is_viable(version, rnd, player, data, &self.team_plan, t, true, debug) { return None }   // L70",
      evidence=ev, behavior_change=False, found_by="reused")
p.fix("/specs[80]/logic", old=u"battle.update(version, rnd, player, data, &self.positioning_score, &mut self.team_plan, debug)   // L99",
      new=u"battle.update(version, rnd, player, data, &self.positioning_score, &self.team_plan, debug)   // L99",
      evidence=ev, behavior_change=False, found_by="reused")
p.fix("/specs[80]/mem[4]/note", old=u"&mut 로 tower_dive_is_viable(L70)·update(L99) 에 전달",
      new=u"`&TeamPlan` 으로 tower_dive_is_viable(L70)·update(L99) 에 전달 (IR `ptr noundef nonnull align 8` — !Freeze 공유참조, tcx 콜리 sig 도 `&TeamPlan`)",
      evidence=ev, behavior_change=False, found_by="reused")
# [80] consts[2] 보강 — dive_rejoin_cd = tps*4 + 1
ev2 = (u"m13.ll:33938 `%107 = shl i64 %106, 2, !dbg !39071` · 33939 `%108 = add i64 %100, 1, !dbg !39071`(둘 다 dloc = dive_rejoin_cd engage.rs:973 ← try_engage:57) · "
       u"33940 `%109 = add i64 %108, %107, !dbg !39068`(:57) · 33941 `icmp ugt i64 %98(tick), %109`. ⟹ `+1` 은 L57 이 아니라 dive_rejoin_cd(:973) 안의 리터럴")
p.fix("/specs[80]/consts[2]/meaning", old=u"folded: dive_rejoin_cd(engage.rs:973) = tps<<2 = tps*4 (4초) — `shl i64 %tps, 2`.",
      new=u"folded: dive_rejoin_cd(engage.rs:973) = tps<<2 + 1 = tps*4 + 1 (4초+1틱; `+1` 도 :973 의 리터럴 `add i64 %100, 1 !dbg 973`) — `shl i64 %tps, 2`.",
      evidence=ev2, behavior_change=False, found_by="new", kind=u"보강")
p.fix("/specs[80]/consts[0]/meaning", old=u"L57 의 `+1` 도 같은 리터럴(abandon_tick + 1 + tps*4)",
      new=u"L57 의 `+1` 은 dive_rejoin_cd(engage.rs:973) 안의 리터럴(abandon_tick + (tps*4 + 1), LLVM 이 (abandon+1)+tps*4 로 재결합)",
      evidence=ev2, behavior_change=False, found_by="new", kind=u"보강")

# ══ G16 [81] params i 규약 (sret 없음 → 소스 인자 1..8) ═══════════════
ev = u"규약 정본 07·15: sret=0, 소스 인자 1..n. check_epic_hunt 는 bool 반환(sret 없음)이라 8인자 = i 1..8. m09.ll define 의 %0..%7 이 소스 인자 1..8"
for k in range(8):
    p.fix("/specs[81]/sig/params[%d]/i" % k, old=k, new=k + 1, evidence=ev, behavior_change=False, found_by="reused")

# ══ G20 R1 [82] mem[31] 이름 정규화(보강) ══════════════════════════════
ev = (u"m09.ll:62506 `store i64 12, ptr %0`(:325) · 62508 `store i64 0, ptr %40`(+8 = Vec.cap) · 62510 `store ptr inttoptr (i64 8 to ptr), ptr %41`(+16 = Vec.ptr dangling) · "
      u"62512 `llvm.memset(… %42(+24), i8 0, i64 11)` = Vec.len(8B)+focus_epic_only+vision_only+v46_flee. "
      u"D9-OFF: 원점 EpicHuntAndPokePlan+0x0 의 필드명은 v46_flee_threats(#83 과 동일 문자열)")
p.fix("/specs[82]/mem[31]/name", old=u"v46_flee_threats·focus_epic_only·vision_only·v46_flee", new=u"v46_flee_threats",
      evidence=ev, behavior_change=False, found_by="reused", kind=u"보강")
p.fix("/specs[82]/mem[31]/note", old=u"memset 11B — 전부 기본값",
      new=u"EpicHuntAndPokePlan::default() 인라인: +0 cap=0 · +8 ptr=8(dangling) · +16 memset 11B = len 0 + focus_epic_only(+0x18)/vision_only(+0x19)/v46_flee(+0x1a) 전부 false",
      evidence=ev, behavior_change=False, found_by="reused", kind=u"보강")

# ══ G19 [79] knobs[0] — 오탐 ═══════════════════════════════════════════
p.fix("/specs[79]/knobs[0]/value", old=480, new=480, kind=u"오탐",
      evidence=(u"명세가 옳다. 480 은 콜리 game_core::AthleteParameter::judge_battle_latency 본문(_gcbc g15.ll:126049 define … `range(i64 8, 49)`; +17행 `%11 = sub nuw nsw i16 480, %10` · `udiv i16 %11, 10`)에 있고, "
                u"이 함수(m12.ll:37951 `call judge_battle_latency` → 37957 `icmp ult i64 %121, %119`)는 반환값 비교만 한다(비인라인 별도 define). "
                u"게이트 보강안: `where` 가 괄호로 다른 .rs(player.rs) 를 병기하거나 value 가 비인라인 콜리 상수임을 `effect` 가 말하면 V1 유보"),
      behavior_change=False, found_by="reused")

# ══ ev 상향 — #77 오라클 (v19A_o1) ══════════════════════════════════════
def ev77(path, what, frm=4, to=2):
    p.ev(path, evidence=O1 + what, to=to, frm=frm, found_by="reused")
ev77("/specs[77]/consts[0]", u"hp*100/stat_cached.hp — stat_cached.hp=1000 에 hp 300/310/990 을 넣어 hp_ratio 30/31/99 로 갈림(B·C1·C2·D1)")
ev77("/specs[77]/consts[2]", u"version 2+heal_commit → Recall(G1) / version 1+heal_commit → DefenseNexus(G2)")
ev77("/specs[77]/consts[5]", u"danger 경로에서 hp 20% → Recall(E2) · 21% → DefenseNexus(E3) · 20%+샘 안 → DefenseNexus(E4): 경계 `> 20` 확정")
ev77("/specs[77]/consts[6]", u"비위험 경로 샘 밖에서 hp 30% → Recall(C1) · 31% → DefenseNexus(C2): 경계 `< 31` 확정")
ev77("/specs[77]/knobs[2]", u"E2/E3/E4 — hp 20%↔21% 및 샘 안 예외로 arm 방향 확정")
ev77("/specs[77]/knobs[3]", u"C1/C2 — hp 30%↔31% 로 arm 방향 확정")
ev77("/specs[77]/knobs[4]", u"G1/G2 — version 1 에선 heal_commit 무시, 2 에선 즉시 Recall")
ev77("/specs[77]/sig/params[2]", u"version 1/2 로 G1/G2 분기")
ev77("/specs[77]/sig/params[6]", u"GoalData+0xf0(heal_commit) 을 1 로 쓰면 version 2 에서 Recall(G1)")
ev77("/specs[77]/mem[20]", u"GoalData+0xf0 에 1 을 써서 version 2 → Recall(G1), version 1 → 무시(G2)", to=3)
ev77("/specs[77]/mem[16]", u"MapDef+0x6d70..0x6d88 을 읽어 얻은 팀0 샘 rect (0,896000)-(64000,960000) 안/밖으로 D1(Recall)/D2(DefenseNexus) 가 갈림", to=3)
ev77("/specs[77]/mem[9]", u"Entity.hp 를 300/310/990 으로 바꾸면 결과가 갈림(B·C1·C2·D1·D2)", to=3)
ev77("/specs[77]/mem[10]", u"Entity.stat_cached.hp(기본 1 → 1000)로 hp_ratio 분모 확인 — 1 이면 전 케이스 hp_ratio 0", to=3)
ev77("/specs[77]/mem[21]", u"sret+0 i64 = 17(Debug: DefenseNexus) / 5(Debug: Recall) 실측", to=3)
ev77("/specs[77]/mem[22]", u"Debug 출력 `DefenseNexusSubPlan { last_gate: 0, focus: None }`", to=3)
ev77("/specs[77]/mem[23]", u"Debug 출력 `last_gate: 0`", to=3)
ev77("/specs[77]/mem[15]", u"ctx.tutorial=JungleOnly 로 existing_lines_weak 경로(E1~E7) 진입 — 단 under_direct_attack 이 함께 참이라 valid_lines 표 단독 확증은 아님", to=3)

p.brief_error(u"§4 G7 이 [79] open[3] 을 「shared.전투_위협모델.난수 와 7/11 토큰 겹침」으로 지목했는데 그 open 은 judge_battle_latency 필드 오프셋의 **사실 서술**(물음이 아님)이라 과열림이 아니라 분류오류(→ notes)다. 토큰 겹침 휴리스틱이 사실 서술과 물음을 구분하지 못한다")
p.brief_error(u"§4 G20 [82] 3건은 전부 R1 인데 지시 초점은 「G20 은 R2/R4 만(R1 잔여는 variant/구조적)」이라 서로 어긋난다 — R1 을 실을 거면 「구조적이면 보고만」을 §4 에 명시해야 배치가 고치지 않고 넘긴다. 이번엔 #82 mem[31] 한 건만 문자열 정규화(보강)로 닫았다")
p.brief_error(u"§1 표의 ev≥4 수(77=36)는 mem 24행을 포함하는데 mem 은 상한 3 이라 오라클로도 2 로 못 내린다 — 「오라클로 내려라」 대상 수에서 mem 을 빼서 보여 주는 편이 목표 설정에 맞다")
p.save()
