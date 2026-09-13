# -*- coding: utf-8 -*-
"""17차 배치 D patch.json 생성 — mkpatch 참조구현 사용. 실행: python -X utf8 _verify17/D/oracle/mkpatch_D.py"""
import sys, io, json, os
sys.path.insert(0, r"C:\tfm2mods\MIG")
import mkpatch

p = mkpatch.Patch(round=17, batch="D")
SCR = (r"C:\Users\jungs\AppData\Local\Temp\claude\C--Users-jungs-Desktop-claude-tfm2--claude-worktrees-swap-order-button-style-fe9e04"
       r"\8c613d5e-d69c-428f-aff5-fe06054da7f0\scratchpad\v17D")

# ═══════════════════════════════ 55 EntityPositioningCache::new ═══════════════════════════════
# G12 consts[2] src_line=142 — 오탐. 리터럴이 `-1`(add) 이라 게이트 정규식 `,\s*1` 이 못 본다.
p.fix("/specs[55]/consts[2]/src_line", old=142, new=142, kind=u"오탐",
      evidence=u"m07.ll:5327 `%39 = add i64 %38, -1, !dbg !20585` (!20585 = effect.rs:26 `Effect::range` < inlinedAt score_parameter.rs:142). "
               u"142 가 맞다. 게이트가 못 본 이유: 소스 `level - 1` 이 IR 에선 `add …, -1` 로 접혀 리터럴이 -1 인데 "
               u"`srclinecheck.litpat(1)` 은 `, 1` 만 찾는다(`, -1` 의 `-` 가 경계를 깬다). 동형 14곳(5558·5865·5910·5955·6288·6367·6445·6528·6607·6685·6838·6876·6914) 전부 `-1`",
      behavior_change=False, found_by="new", force=True)
# 값 자체는 IR 리터럴로(consts[9] 의 관행: value=IR 리터럴 · folded_from=소스값)
p.fix("/specs[55]/consts[2]/value", old=1, new=-1, kind=u"보강",
      evidence=u"m07.ll:5327 `add i64 %38, -1` — 본문에 실제로 있는 숫자는 -1 (SPEC_GUIDE §3 「value 는 본문 숫자 그대로」). 소스값 1 은 folded_from",
      behavior_change=False, found_by="reused")
p.fix("/specs[55]/consts[2]/meaning",
      old=u"growth_range * (level - 1) — Effect::max_range(caster) 인라인(effect.rs:26)",
      new=u"level - 1 — growth_range 에 곱할 레벨 가감분(소스 리터럴 1 → IR `add i64 %38, -1` 로 접힘, folded_from 1). game_core::Effect::range(&self, caster) 인라인(effect.rs:25~26, mir=1 xinl=1)",
      kind=u"보강",
      evidence=u"m07.ll:5327 !dbg !20585 사슬: `range`[game-core\\src\\simulation\\effect.rs:26] < `new`[score_parameter.rs:142]. tcx: AssocFn pub game_core::Effect::range effect.rs:25 mir=True xinl=True. `Effect::max_range` 라는 항목은 tcx 에 없다",
      behavior_change=False, found_by="new")
p.fix("/specs[55]/logic",
      old=u"//  max_range(eff, caster)  = eff.range(+0x10) + eff.growth_range(+0x18)*(caster.level-1) + caster.stat_buff_cached.range(+0x438)   [effect.rs:26]",
      new=u"//  eff_range(eff, caster)  = game_core::Effect::range(&eff, caster) 인라인 = eff.range(+0x10) + eff.growth_range(+0x18)*(caster.level-1) + caster.stat_buff_cached.range(+0x438)   [effect.rs:25~26 · tcx pub mir=1 xinl=1]",
      evidence=u"위와 동일(m07.ll:5327·5386~5390 !dbg !20585 = effect.rs:26 scope `range`)", behavior_change=False, found_by="new")
# 헬퍼 이름 오기: logic 의 `max_range(eff, caster)` 는 tcx 에 없는 이름이고, 자동수집이 game_ai::max_range(battle.rs:2234)를 후보로 실었다.
p.fix("/specs[55]/logic", old=u"max_range(", new=u"eff_range(",
      evidence=u"a55 전 범위 인라인 출처 집계: effect.rs:26 130회·battle.rs 0회. !20585/!20705/!20716/!20727 의 scope 이름 = `range`(effect.rs) ⟹ 헬퍼 = game_core::Effect::range(&self, caster). "
               u"`game_ai::max_range`(fn(&Entity,&Entity)->u64, battle.rs:2234) 는 이 함수와 무관 — callees[3] 후보는 산문 이름에서 긁힌 오염(G9 역방향)",
      behavior_change=False, found_by="new")
# open[5] 「표기 불가」 → 반전: !dbg scope 가 compute_range_sq(130) 를 가리켜 소스 표기가 확정된다
p.fix("/specs[55]/open[5]/q",
      old=u"attack_range_ext_sq 의 50000 은 18000+32000 접힘으로 읽었다(IR 이 %73(+18000 전 값)에 50000 을 더함). 소스가 (attack_range+32000) 인지 (base+50000) 인지는 표기 불가(외연 동일)",
      new=u"attack_range_ext_sq 의 50000 은 L183 `compute_range_sq(attack_range)` 인라인 안의 `range + 32000`(score_parameter.rs:130)에 18000 이 LLVM 재결합으로 접힌 값이다 — "
          u"m07.ll:5744 `%301 = add i64 %73, 50000, !dbg !20782` 의 scope 가 `compute_range_sq`[score_parameter.rs:130] < `new`[183] 이고 5741 `%300 = mul i64 %74, %74`(!20779 = :129 < 183). "
          u"소스 표기 = (attack_range+32000)^2 로 확정했다. L183 에는 :131(half, `lshr 1`) 명령이 없어 반환 튜플의 세 번째 성분이 버려진 것(struct 에 attack_range_half_sq 없음)으로 추정한다",
      evidence=u"m07.ll:5741 `%300 = mul i64 %74, %74, !dbg !20779`(:129<183) · 5744 `%301 = add i64 %73, 50000, !dbg !20782`(:130<183) · dbgchain !20782 → compute_range_sq line130 → new line183. "
               u"「표기 불가」 판정은 !dbg 의 scope 함수명을 안 본 것 — 판정 반전(표기 불가 → 확정)",
      behavior_change=False, found_by="new")
p.fix("/specs[55]/consts[7]/meaning",
      old=u"attack_range_ext_sq = (attack_range - 18000 + 50000)^2 = (attack_range + 32000)^2 — 18000+32000 이 접힌 값",
      new=u"attack_range_ext_sq = (attack_range + 32000)^2 — L183 compute_range_sq(score_parameter.rs:130 `r+32000`) 인라인에 +18000(L142) 이 LLVM 재결합으로 접혀 `add i64 %73, 50000` 이 된 계수(folded 18000+32000)",
      kind=u"보강", evidence=u"m07.ll:5744 !dbg !20782 = compute_range_sq:130 < new:183", behavior_change=False, found_by="new")

# ═══════════════════════════════ 56 PassiveLinePlan::sub_plan ═══════════════════════════════
# G12 consts[28] src_line=969 — 실오류. 969 의 리터럴은 `icmp eq i8 %233, 2`(line==Bottom) 뿐이다.
p.fix("/specs[56]/consts[28]/src_line", old=969, new=982,
      evidence=u"visible_state==Visible 비교 = m04.ll:27304~27308 `%970 = gep %935, 56` · `%971 = gep {i64,[2 x i64]} %970, %936`(stride 24 · team) · `%972 = load` · `%973 = icmp eq i64 %972, 0` — 전부 `!dbg !40323 = !DILocation(line: 0, scope: sub_plan)`(LLVM 이 두 인라인 사이트의 공통식을 블록 %969 로 sink). "
               u"is_visible_from(entity.rs:1481) 인라인 사이트 = 27198 !40499(entity.rs:1136<1482<**982**) 와 27281 !40561(…<**985**) 이고 %936 = phi[%931(982 경로), %962(985 경로)]. "
               u"969 는 27058 `%871 = icmp eq i8 %233, 2, !dbg !40322`(player.rs:988<969 = `line == Bottom`) 이라 값 0 이 없다. 첫 사이트 982 로 정정(985 는 meaning 에)",
      behavior_change=False, found_by="new")
p.fix("/specs[56]/consts[28]/meaning",
      old=u"오브젝트 visible_state[champ.team].tag == 0(Visible) = giveup_object(오브젝트가 보이면 포기)",
      new=u"오브젝트 visible_state[champ.team].tag == 0(Visible) = giveup_object(오브젝트가 보이면 포기). Entity::is_visible_from(entity.rs:1481) 인라인 — 사이트 L982(Top·serpen)/L985(Bottom·morgard). "
          u"⚠비교 명령 자체(m04.ll:27308 `icmp eq i64 %972, 0`)는 `!DILocation(line: 0)` 이라 G12 가 어느 줄에도 못 붙인다(두 사이트 공통식이 블록 %969 로 sink 됨)",
      kind=u"보강", evidence=u"m04.ll:27304~27308 !dbg !40323 = line 0 · 27198/27281 인라인 사이트", behavior_change=False, found_by="new")
# G18 — logic 이 인용한 +0x60 = Blackboard.bottom_minion_state.from_mid 행 삽입 (v3 mem 인덱스 37 = BrainMinionParameter 블록 바로 뒤)
p.errors.append({"op": "insert", "path": "/specs[56]/mem", "at": 37, "guard": u"bottom_minion_state.from_mid", "guard_key": "name",
                 "new": {"base": "Blackboard", "offset": "0x60", "name": u"bottom_minion_state.from_mid",
                         "note": u"IR 96 (m04.ll:25537 `gep <Blackboard> %bb, team` → 25538 `gep i8 …, 96` → 25539 load i64 → 25540 `icmp slt i64 %226, 1000`). L1039 wave_pushed — bottom(+0x50) 의 from_mid(+0x10). tcx 정본 대조: tcxdict Blackboard 0x50 = bottom_minion_state, BrainMinionParameter 0x10 = from_mid",
                         "dir": "r"},
                 "kind": u"보강", "evidence": u"G18: logic L1039 가 `from_mid(+0x60)` 을 인용하는데 표에 없었다. m04.ll:25537~25540 + tcxdict Blackboard/BrainMinionParameter",
                 "behavior_change": False, "found_by": "new"})
# L1020 — 인라인 헬퍼 귀속. tutorial 게이트·30초 창은 passive_line 의 식이 아니라 game_core 공용 헬퍼 3개의 인라인이다.
p.fix("/specs[56]/logic",
      old=u"[L1020] tick = game.tick(); if tutorial ∈{0,5,7,8} && !(tick < setting.epic_jungle.first_spawn_tick(+0x8a8).saturating_sub(tps*30)) → None   // 초반에만",
      new=u"[L1020] tick = game.tick()/*vtable+0x28*/; if !context.is_line_phase(tick) → None   // 초반(첫 에픽 스폰 30초 전)에만\n"
          u"       //   ★인라인 GameContext::is_line_phase(&self, tick)[runner.rs:397~399, pub] = !self.tutorial(+0x38).spawn_epic() || self.setting(+0x8).is_line_phase(tick)\n"
          u"       //     TutorialType::spawn_epic()[runner.rs:262~263] 자리의 switch = 태그 ∈{0 None,5 MidBottom,7 Line,8 Total} → 시간 게이트, 그 외(1,2,3,4,6) → 게이트 없이 통과\n"
          u"       //     GameSetting::is_line_phase(&self, tick)[setting.rs:702~704, pub] = tick < epic_jungle.first_spawn_tick(+0x8a8).saturating_sub(tick_per_second(+0x12f8)*30)",
      evidence=u"m04.ll:25099 `%59 = tail call i64 %58(ptr %54)`(vtable+40 tick) · 25105~25110 `switch i8 %61 [0,7,8,5 → %62] default %72` !dbg !39007 = spawn_epic[runner.rs:263] < is_line_phase[runner.rs:399] < check_bot_lane_2v1[passive_line.rs:1020] · "
               u"25116 `gep %64, 2216` !39009 = is_line_phase[setting.rs:703] · 25121 `mul i64 %68, 30` !39016 = [setting.rs:704] · 25123 usub.sat · 25124 `icmp ult i64 %59, %70` → br %72/%232. "
               u"tcx: GameContext::is_line_phase runner.rs:397 pub mir=1 xinl=1 · GameSetting::is_line_phase setting.rs:702 pub mir=1 xinl=1 · TutorialType::spawn_epic runner.rs:262 pub mir=1 xinl=1 (셋 다 define 없음=항상 인라인)",
      behavior_change=False, found_by="new")
p.fix("/specs[56]/knobs[0]/where",
      old=u"passive_line.rs:1020 (m04.ll:25121 `mul i64 %68, 30`)",
      new=u"passive_line.rs:1020 → 인라인 GameSetting::is_line_phase setting.rs:704 (m04.ll:25121 `mul i64 %68, 30`) — ★game_core 공용 헬퍼라 여기만이 아니라 is_line_phase 의 모든 호출처(#51 check_press_tower_opportunity 등)가 같이 바뀐다",
      kind=u"보강", evidence=u"m04.ll:25121 !dbg !39016 사슬 = is_line_phase[setting.rs:704] < is_line_phase[runner.rs:399] < 1020", behavior_change=False, found_by="new")
p.fix("/specs[56]/consts[7]/meaning",
      old=u"tick < first_spawn_tick.saturating_sub(tps*30) 일 때만 2v1 검사 (첫 에픽 스폰 30초 전까지)",
      new=u"tick < first_spawn_tick.saturating_sub(tps*30) 일 때만 2v1 검사 (첫 에픽 스폰 30초 전까지) — GameSetting::is_line_phase(setting.rs:702~704) 인라인 계수, 이 함수 고유 상수가 아니다",
      kind=u"보강", evidence=u"m04.ll:25121 !dbg !39016 = setting.rs:704", behavior_change=False, found_by="new")
p.fix("/specs[56]/mem[10]/note",
      old=u"IR 56. check_bot_lane_2v1 시간 게이트: ∈{0,5,7,8} 일 때만 first_spawn 기준 적용",
      new=u"IR 56. check_bot_lane_2v1 시간 게이트: ∈{0,5,7,8} 일 때만 first_spawn 기준 적용 — GameContext::is_line_phase(runner.rs:397) 안의 TutorialType::spawn_epic(runner.rs:263) 인라인 switch(m04.ll:25105)",
      kind=u"보강", evidence=u"m04.ll:25105 switch !dbg !39007 사슬", behavior_change=False, found_by="new")
# G10 open[6] — 사실 서술. 문면을 FACT_TAIL 로 끝내 notes 로 보낸다
p.fix("/specs[56]/open[6]/q",
      old=u"L1003 nearest_tower 가 Tower 가 아닐 때(nexus 로 대체된 경우 ty==3 Nexus): Recall 검사 없이 LineDefense — IR 확정",
      new=u"L1003 nearest_tower 가 Tower 가 아닐 때(nexus 로 대체된 경우 ty==3 Nexus): Recall 검사 없이 LineDefense 로 간다 — m04.ll:27298~27301 `%966 = gep %811, 104`(+0x68 ty@tag) · `%968 = icmp eq i64 %967, 2` · `br %968, %980(Tower→+0x88 nearest_enemy 검사), %519(LineDefense)`. tcxdict EntityType: 2=Tower, 3=Nexus. 「사실 서술」이다",
      evidence=u"m04.ll:27298~27301 + 27325~27328(`gep %811, 136` · `icmp eq i64 %982, 0` → %519 LineDefense / %984 tag 5 Recall) + tcxdict --enum EntityType(2 Tower · 3 Nexus)",
      behavior_change=False, found_by="reused")
# G10 open[8] — 사실 서술 + 헬퍼 귀속
p.fix("/specs[56]/open[8]/q",
      old=u"L1020 tutorial 게이트: 태그 1,2,3,4,6(First/TopSolo/Bottom/MidSolo/JungleOnly)은 시간 게이트 없이 2v1 검사로 진입 — switch default 경로로 확정",
      new=u"L1020 tutorial 게이트: 태그 1,2,3,4,6(First/TopSolo/Bottom/MidSolo/JungleOnly)은 시간 게이트 없이 2v1 검사로 진입한다 — m04.ll:25105~25110 `switch i8 %61, label %72 [0,7,8,5 → %62]` 의 default %72 가 L1021 블록(!dbg !39007 = TutorialType::spawn_epic[runner.rs:263] < GameContext::is_line_phase[runner.rs:399] < 1020). "
          u"⚠집합 {0,5,7,8} 은 is_line_phase 인라인 3사이트(m04.ll:25105 · m13.ll:7888 handler.rs:2113 · #51)에서 일치하지만, 순수 spawn_epic 은 morgard_exists 인라인(m13.ll:53429 `add i8 %40, -7; icmp ult i8 %46, -6`)에서 {0,7,8} 이다 — "
          u"is_line_phase 안에서 5(MidBottom)를 더하는 항(spawn_serpen() 합집합인지 ==MidBottom 인지)은 표기 불가이나 동작(집합)은 확정했다",
      evidence=u"m04.ll:25105~25110 + m13.ll:7888~7893(같은 switch, !19179 = spawn_epic:263 < is_line_phase:399 < handler.rs:2113) + m13.ll:53429~53431(morgard_exists 사이트 {0,7,8}) + m13.ll:53372(spawn_serpen:267 = {0,5,7,8})",
      behavior_change=False, found_by="new")
# consts[23] kind=미상 → 임계(콜리 인자라 IR 관측이 CALLARG=약함 → 낱말 존중)
p.fix("/specs[56]/consts[23]/meaning",
      old=u"can_near_enemies_range(.., front_minion.x, front_minion.y, 150000, ..) — 전방 미니언 150000 안에 올 수 있는 적 목록",
      new=u"can_near_enemies_range(.., front_minion.x, front_minion.y, 150000, ..) — 전방 미니언 150000(≈4.7셀) 안에 올 수 있는 적 목록. 콜리에 넘기는 반경 임계(m04.ll:26253 호출 인자, 본문 비교 없음)",
      kind=u"보강", evidence=u"m04.ll:26253 `can_near_enemies_range(…, i64 noundef 150000, …)` — 이 함수 안에는 150000 비교가 없어 G15 관측이 CALLARG 뿐(미상). 의미는 반경 임계", behavior_change=False, found_by="reused")

# ═══════════════════════════════ ev_up — mem 오프셋 tcxdict 전수 대조(불일치 0) ═══════════════════════════════
def EV(path, evidence, guard=None, found_by="reused", frm=4):
    pp = mkpatch.parse_path(path)
    assert pp and pp[3] is not None, path
    e = {"path": path, "from": int(frm), "to": 3, "evidence": evidence, "found_by": found_by}
    if guard: e["guard"] = guard
    p.ev_up.append(e)

chk = json.load(io.open(os.path.join(SCR, "memchk_out.json"), encoding="utf-8"))
for i in ("55", "56"):
    for (j, hit, base, off, name, star) in chk[i]:
        if hit != "OK":
            continue
        jj = j
        if i == "56" and j >= 37:
            jj = j + 1                      # 위 insert(at=37) 로 밀린다
        EV("/specs[%s]/mem[%d]" % (i, jj),
           evidence=u"tcx 정본 대조(17차 D · tcxdict %s %s): %s" % (base, off, star.replace(u"★", u"")),
           guard=name[:40])

# ═══════════════════════════════ ev_up — 오라클 실행 확증(#56, 11/11 MATCH) ═══════════════════════════════
O = u"오라클 실행 확인(_verify17/D/oracle/D17_o1.rs · 케이스당 프로세스 1개 · D17_o1_case{0..10}.out · setting_ok=true · 11/11 예측 일치): "
def EV2(path, evidence, guard=None, found_by="new", frm=4):
    pp = mkpatch.parse_path(path); assert pp and pp[3] is not None, path
    e = {"path": path, "from": int(frm), "to": 2, "evidence": O + evidence, "found_by": found_by}
    if guard: e["guard"] = guard
    p.ev_up.append(e)
EV2("/specs[56]/consts[0]", u"C0 in_recall(+0x110)=1 → tag 5 Recall(RecallSubPlan)", guard=u"Recall")
EV2("/specs[56]/consts[1]", u"C2 v46_flee=1·cover=0·acute=0 → tag 3 LineSafe{line: Bottom}(+8=2)", guard=u"LineSafe")
EV2("/specs[56]/consts[2]", u"C1 v46_flee=1·cover=0·acute=1 → tag 4 LineWait{line: Bottom}(+8=2)", guard=u"LineWait")
EV2("/specs[56]/consts[3]", u"C3·C4·C7·C8·C9·C10 → tag 2 LineDefense{style,line,minion_action_type} (+8/+9/+10 = 0/2/2)", guard=u"LineDefense")
EV2("/specs[56]/consts[5]", u"C5 objective(+0x41f)=8 Gank·line(+0x420)=2 → Normal(1) 갱크 경로 / C8 태그 9 Dive·같은 라인 → Push(2) 비갱크 경로 — L884 는 정확히 8 만", guard=u"Gank")
EV2("/specs[56]/consts[19]", u"C10 position=Jungle 플레이어로 호출 → style=Defensive(+8=1) / C4 Bottom → Aggressive(0)", guard=u"line_style")
EV2("/specs[56]/consts[20]", u"C5 bottom.from_mid(+0x60)=2000 → Normal(1) / C6 2001 → Pull(0) — 경계 `< 2001` 확정", guard=u"2001")
EV2("/specs[56]/consts[22]", u"C4·C7·C8(근접 적 없음: 시작 위치, 적은 분수) → Push(2) — 「적 없으면 Push(2)」 가지 확인(minion_power<0 가지는 미실행)", guard=u"Push(2)")
EV2("/specs[56]/knobs[5]", u"C5 from_mid=2000 → Normal / C6 2001 → Pull", guard=u"2001")
EV2("/specs[56]/knobs[11]", u"C4·C7·C8 근접 적 없음 → Push(2) 가지만 확인(부호 가지 미실행)", guard=u"minion_power")
EV2("/specs[56]/sig/params[0]", u"반환 SubPlan(72B) 태그 +0x0 = 5/3/4/2, LineDefense 페이로드 +0x8 style·+0x9 line·+0xa minion_action_type (`{:?}` 로 필드명까지 확인)", guard=u"sret")
EV2("/specs[56]/sig/params[1]", u"+0x110/+0x112/+0x113/+0x115 raw write 에 분기가 그대로 반응(C0~C3) · size_of::<PassiveLinePlan>()=280", guard=u"self")
EV2("/specs[56]/sig/params[6]", u"+0x41f/+0x420 raw write 에 갱크 분기가 반응(C5~C8) · size_of::<TeamPlan>()=1064", guard=u"team_plan")
for (j, g, ev) in [(0, u"in_recall", u"C0"), (1, u"v46_flee", u"C1~C3"), (2, u"v46_flee_cover", u"C3 cover=1 → 게이트 불통과"), (3, u"v46_flee_acute", u"C1 acute=1 → LineWait / C2 0 → LineSafe"),
                   (4, u"line", u"+8/+9 에 2(Bottom) 가 나옴"), (5, u"objective", u"C5~C8"), (6, u"objective.Gank.line", u"C7 line=0(Top)≠self.line → 비갱크"),
                   (26, u"info.position", u"C10 Jungle → Defensive"), (33, u"front_minion", u"C9 +0x50=1·+0x58=champ_id → front_minion 경로 진입(LineDefense)"),
                   (37, u"bottom_minion_state.from_mid", u"C5/C6 +0x60 = 2000/2001 이 Normal/Pull 을 가른다(G18 삽입행의 실행 확증)"),
                   (49, u"tag", u"tag 5/3/4/2"), (50, u"LineSafe.line", u"+8 = 2(Bottom) / style 0·1"), (51, u"LineDefense.line", u"+9 = 2"), (52, u"LineDefense.minion_action_type", u"+10 = 2/1/0")]:
    pp = "/specs[56]/mem[%d]" % j
    p.ev_up.append({"path": pp, "from": 4, "to": 3, "evidence": O + ev, "found_by": "new", "guard": g})

p.brief_error(u"호출 메시지는 배치 D 담당을 specs[53]~[56] 이라 했지만 DOSSIER_D.md §1·§3 은 `55`~`56` 이고 #53·#54 의 spec_*.md 는 _verify17/C/ 에 있다(배치 C 담당). 지시문을 따랐고 #53(G7 is_recent_visible)·#54 는 손대지 않았다 — 메시지의 G4 「#53·#54·#56」·G7 「#53」 언급은 배치 C 몫이다.")
p.brief_error(u"§4 G4 의 「실제 후보」 줄이 `bushes, drop  ⟵ IR 에 이 leaf 호출이 있으나 … 싣지 않는다, first_spawn_tick, from_mid, front_minion, map_` 로 잘려 있다 — 12개 중 5개만 보이고 `drop` 의 설명문이 목록 원소처럼 끼어 있다(mkdossier 가 callees_unmatched.names 에 설명 문자열을 섞어 자른다). 전체 12개는 spec_56 ⚠미매칭 줄에서 읽었다.")
p.brief_error(u"§1 표의 `ev≥4(미실행) 89` 가 두 함수 모두 89 로 같다 — #55 mem 70+consts 14+knobs 6 = 90, #56 mem 52+consts 29+knobs 12 = 93 이라 어느 분모에서 나온 수인지 알 수 없다(집계 스코프 미기재 — 5차 지시 오류와 같은 형태).")
p.save()
