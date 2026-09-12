# -*- coding: utf-8 -*-
u"""6차 배치A `patch.json` 생성기. (손으로 158행을 적다 빠뜨리는 것을 막는다 — 5차 유실 317행의 재발 방지)"""
import io, json, os, sys
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
HERE = os.path.dirname(os.path.abspath(__file__))

O1 = u"A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)"
O6 = u"A6_o6.tsv(실주소 차 보충, 10/10 MATCH)"
O2 = u"A6_o2_main.tsv / A6_o2_radius.tsv / A6_o2_samepos.tsv"
O3 = u"A6_o3.tsv"
O4 = u"A6_o4.tsv"
O5 = u"A6_o5.tsv / A6_o5_tps.tsv"

ev = []


def E(path, evid, to=2):
    ev.append({"path": path, "from": 4, "to": to, "evidence": evid})


def MEM(i, idxs, evid):
    for j in idxs:
        ev.append({"path": "/specs[%d]/mem[%d]" % (i, j), "from": 4, "to": 3, "evidence": evid})


# ── specs[0] ult ─────────────────────────────────────────────────────
MEM(0, [0, 1, 2, 3, 4, 5, 6, 7, 9, 10, 12, 13, 14, 15, 16, 17, 19, 20, 21, 22, 26, 28], O1)
MEM(0, [8], u"_verify5/A/A5_o2.tsv TeamType::Player payload@+0x8=1 실측 + " + O1)
MEM(0, [18], O6 + u" EFFECT_ARC word1 == dyn 팻포인터 vtable 주소 일치")
for j, t in [(1, u"level 4 → ult_eff_some=false / level 5 → true (A6_o2 visible 4/5)"),
             (2, u"Option<Input>::None tag@+0x0 = -1 실측"),
             (3, u"CastingType::Position = 1 실측"),
             (4, u"CastingType::Direction = 2 실측"),
             (5, u"casting=Targeting 만 caster_r_ON 이 MATCH_ULT=YES, 나머지 3값은 caster_r_OFF 가 YES"),
             (6, u"VisibleState::Visible = 0 / Unknown = 2 실측 + invisible 케이스가 L340 경로로 갈림"),
             (7, u"radius_mult = 0/50/-50/100/-100/-99 스윕 — radius()=raw*(100+mult)/100 이 6/6 일치, 기준 1000 가설 기각"),
             (8, u"Some(Input::Ult) tag@+0x0 = 5 실측"),
             (9, u"margin 149999/150000/150001 중 150000 만 MATCH_ULT=YES")]:
    E("/specs[0]/consts[%d]" % j, O2 + u" — " + t)
for j, t in [(0, u"level 4 → None / 5 → 이동입력 (A6_o2)"),
             (1, u"margin ±1 로 좌표가 1 씩 움직이고 150000 만 일치"),
             (2, u"casting==Targeting 일 때만 champ.radius() 가 총사거리에 더해짐(caster_r_ON/OFF 대조)"),
             (3, u"radius_mult 스윕으로 기준값 100 판별(1000 가설 기각), 음수 mult 에서 sext 확정")]:
    E("/specs[0]/knobs[%d]" % j, O2 + u" — " + t)

# ── specs[1] calculate_jungle_action_score ───────────────────────────
MEM(1, [0, 1, 2, 3, 4, 5, 7], O1)
MEM(1, [6], O6 + u" JUNGLE team0→+0x98=0 / team1→+0x98=1 (camp_type.__0 = 팀 인덱스)")
for j, t in [(1, u"Nexus tag=3 → coef 200, game==mine MATCH"),
             (2, u"Nexus 2개 전부 game=200 MATCH"),
             (3, u"Tower tag=2 → coef 80 MATCH"),
             (4, u"Tower 2개 전부 game=80 MATCH"),
             (5, u"Jungle tag=4 12개 전부 MATCH"),
             (6, u"HPSWEEP hp=497 이상에서 coef 20 MATCH"),
             (7, u"HPSWEEP hp=496 경계에서 40 채택 — `hp<value` 대립가설 25 를 기각(MATCH 판별)"),
             (10, u"Champion tag=13 → coef 0, game=0 MATCH"),
             (11, u"Jungle 만 based=5 가 붙고 Tower/Nexus/Champion 은 0 — 20행 MATCH")]:
    E("/specs[1]/consts[%d]" % j, O3 + u" SCORE/HPSWEEP — " + t)
for j, t in [(0, u"Nexus 200"), (1, u"Tower 80"), (2, u"Jungle 20(처치불가)"),
             (3, u"Jungle 40(처치가능)"), (5, u"based 5"),
             (6, u"camp0=0(블루)·camp0=1(레드) 캠프가 **둘 다** based=5 를 받는다 = `is_jungle(0)||is_jungle(1)`")]:
    E("/specs[1]/knobs[%d]" % j, O3 + u" SCORE 20행 game==mine — " + t)

# ── specs[2] sub_plan ────────────────────────────────────────────────
MEM(2, list(range(0, 6)) + [7, 12, 13, 14, 15], O1)
MEM(2, [6], O1 + u" ★BUMPVEC cap=8/len=3 로 **len@+0x18 을 처음 판별**(5차는 len==cap==2 라 불가로 남겼다)")
MEM(2, [8, 9, 10, 11], O1 + u" MapDef.fountains[team] 튜플 오프셋 0x0/0x8/0x10/0x18 + FOUNTAIN 실값")
MEM(2, [16], O1 + u" ★ANPLAN new(1,Mid)/new(0,Bottom) 바이트 = [team@+0x0, line@+0x8] (5차 미실측분)")
MEM(2, [17, 18, 19, 20], O4 + u" SUBPLAN tag/b8/b9/b10 3분기 전수")
for j, t in [(1, u"in_heal_low → tag=5 Recall"),
             (2, u"no_twin 케이스에서 적팀(1-team) 트윈타워를 비우자 분기가 갈림"),
             (3, u"twin1_len=0 → AttackNexus / len=2 → LineDefense, b8=0(Aggressive)"),
             (4, u"no_twin → tag=16 AttackNexus"),
             (5, u"LineDefense tag=2 · b10=2(Push) — MinionActionType Pull=0/Normal=1/Push=2 실측(A6_o3)")]:
    E("/specs[2]/consts[%d]" % j, O4 + u" — " + t)
for j, t in [(0, u"in_heal_low(hp 500/999) → Recall / in_heal_full(999/999) → LineDefense"),
             (1, u"twin1_len 2 → LineDefense / 0 → AttackNexus"),
             (2, u"LineDefense{style=Aggressive, line=Mid(=self.line), minion_action_type=Push} Debug 출력 일치"),
             (3, u"FOUNTAIN team0=(0,896000,64000,960000) 실측 + 그 안/밖으로 챔프를 옮겨 분기 확인")]:
    E("/specs[2]/knobs[%d]" % j, O4 + u" / " + O1 + u" — " + t)

# ── specs[3] defensive_crisis ────────────────────────────────────────
MEM(3, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 16, 17, 18, 19, 20, 21], O1)
MEM(3, [10], O1 + u" BUMPVEC ptr@+0x0 + _verify5/A/A5_o7.tsv `*(ptr)` == 원소0 주소 CONFIRMED")
MEM(3, [11], O1 + u" ★BUMPVEC cap=8/len=3 로 len@+0x18 판별(5차 미판별분)")
MEM(3, [13, 14, 15], O6 + u" ty.Champion.skill/skill2/ult_cooldown = +0xb8/+0xc0/+0xc8 실주소 차")
for j, t in [(2, u"level 2 → cc_threat=0 / level 3 → 1 (skill2 슬롯) — history[9] 지시 반영"),
             (3, u"level 4 → cc_threat=0 / level 5 → 1 (ult 슬롯) — history[9] 지시 반영"),
             (4, u"Option<Effect>::None 의 +0x30 = -1 실측(A6_o6)"),
             (5, u"tps 1/6/30/60/120 스윕에서 die 는 60 고정, die_imminent 는 `die<tps*2` 로 정확히 갈림 — tps=30(60<60=false) 이 **strict `<`** 를 판별")]:
    E("/specs[3]/consts[%d]" % j, O5 + u" — " + t)
for j, t in [(1, u"cool<=tps 창: tps=60 → 60 통과/61 탈락, **tps=30 → 30 통과/31 탈락** (임계가 tps 를 따라간다) — history[9] 지시 반영"),
             (3, u"level 2/3 경계 실측"),
             (4, u"level 4/5 경계 실측"),
             (6, u"★tps 를 1/6/30/60/120 으로 바꿔도 die 가 **60 으로 고정** ⟹ 이 60 은 tps 가 아니라 하드코딩 리터럴"),
             (19, u"PlayerState+0x218 = 80 (mkgame 이 넣은 AthleteStat.judgement) 실측(A6_o6)"),
             (20, u"PlayerState+0x450 = 1000 (judgement_mental_ratio 초기값) 실측(A6_o6)")]:
    E("/specs[3]/knobs[%d]" % j, O5 + u" / " + O6 + u" — " + t)

# ── specs[4] handle_line_defense ─────────────────────────────────────
MEM(4, [0, 1, 2, 3, 4, 5, 6, 8, 9, 10, 11, 12, 13, 14, 15, 16, 18, 19, 20, 22, 23, 24, 25, 26], O1)
MEM(4, [17], O1 + u" ★BUMPVEC len@+0x18 판별")
MEM(4, [21], O4 + u" STRATEGY morgard_defense_offset=0xe 실측")
for j, t in [(0, u"TutorialType 9값 spawn_epic 실행 = 0/7/8 만 true (A6_o3 SPAWN_EPIC)"),
             (1, u"같은 실행 — tutorial-1 <u 6 게이트 전수"),
             (2, u"3차 배치A 오라클 `_verify3` 반경 경계 d2=40000000000 통과 / 40000400001 탈락 (history[5] 지시 반영)"),
             (3, u"3차 배치A 오라클 인원 구간 20/20, Gather `>1` (history[5] 지시 반영)"),
             (4, u"3차 배치A 오라클 인원 구간 20/20, Battle `{1,2}` (history[5] 지시 반영)")]:
    E("/specs[4]/consts[%d]" % j, t)
for j, t in [(0, u"3차 배치A 오라클 반경 경계 실측 (history[5] 지시 반영)"),
             (1, u"3차 배치A 오라클 Battle 구간 20/20 (history[5] 지시 반영)"),
             (2, u"3차 배치A 오라클 Gather 하한 20/20 (history[5] 지시 반영)"),
             (3, u"A6_o3.tsv SPAWN_EPIC 9값 실행 — tutorial 게이트 전수"),
             (4, u"2차 배치A 오라클 실측 6좌표(history[3]⑤) — 팀별 값까지 확정")]:
    E("/specs[4]/knobs[%d]" % j, t)

errors = [
    # ── E1 판정반전 ──────────────────────────────────────────────
    {"path": "/specs[0]/open[0]", "kind": "판정반전",
     "old": u"isqrt 결과 sz 가 0 일 때(챔프와 타깃 좌표가 완전히 같을 때) 334행에서 div-by-zero 패닉 경로가 실제로 존재한다 — 게임이 이 상황을 상위에서 막는지는 이 범위에서 확인 불가",
     "new": u"★해소(6차 배치A) — **sz==0 으로 334행에 도달하는 것은 이 함수 구조상 불가능하다**(상위가 막는 게 아니라 자기가 막는다). "
            u"근거 ①`Effect::is_in_range`(_gcbc/g06.ll:51634~51774) = `dist_sq <= total^2` 이고 total 은 비음수 항의 합뿐 — **하한 사거리가 없다** "
            u"⟹ 거리 0 이면 항상 in-range. ②`get_input_target` 의 None 반환은 정확히 3곳(m04.ll 블록43 champ null / 블록45 !is_in_range / 블록70 `!visible && !is_nontarget`)이고, "
            u"거리 0 에서는 ①때문에 블록45 가 못 뜨며 블록43 은 `ult` 가 이미 champ 를 얻은 뒤라 못 뜬다. ③남은 블록70 은 `!visible` 을 전제하므로, 그 경우 `ult` 는 L328 에서 "
            u"`!visible` 로 갈라 **L340 `safe_move_avoiding_enemy_well`** 로 나간다 — L329~L335(isqrt·나눗셈)에 못 온다. "
            u"④오라클 실행 확증: champ 와 target 좌표를 완전히 같게 두고 casting 4값 × eff_range {0, 200000} = **8/8 전부 `Ult(..)` 반환, Move 0건**(A6_o2_samepos.tsv). "
            u"⟹ IR 의 `panic_const_div_by_zero` 는 LLVM 이 남긴 **죽은 가드**이고, 재현 때 이 패닉을 모사할 필요가 없다. "
            u"★범위: `ult` 안에서의 판정이다. `get_input_target` 내부가 다른 경로로 None 을 낼 수 있게 바뀌면 다시 봐야 한다.",
     "evidence": u"A6_o2_samepos.tsv 8/8 · _gcbc/g06.ll:51762~51774(`icmp ule %83, %84`) · _gaibc/m04.ll:33168~33230",
     "behavior_change": False, "found_by": "reused"},

    # ── E2 과열림(이미 history 가 닫은 물음) ──────────────────────
    {"path": "/specs[2]/open[1]", "kind": "분류오류",
     "old": u"둘이 항상 같은지(다를 수 있는지)는 이 함수만으론 확정 불가",
     "new": u"둘이 같은지는 **이 함수만으론** 확정 불가 — ★해소(6차 배치A): 답은 이미 같은 파일 `history[1]` 에 있다. "
            u"**항상 반대다**(`LegacyPlanHandler::passive_plan` handler.rs:1879 = `sub i64 1, player.info.team`). "
            u"⟹ 이 항목은 `open` 이 아니라 닫힌 것이다(5차까지 `open` 에 남아 있어 매 라운드 재조사 후보로 잡혔다)",
     "evidence": u"/specs[2]/history[1] 원문 + A6_o1.tsv ANPLAN(AttackNexusPlan.team@+0x0 실측)",
     "behavior_change": False, "found_by": "reused"},

    # ── E3~E6 src_line 오기 (defensive_crisis) ────────────────────
    {"path": "/specs[3]/consts[0]", "kind": "실오류",
     "old": u"enemy_ix = 1 - player.info.team — 적팀 인덱스",
     "new": u"enemy_ix = 1 - player.info.team — 적팀 인덱스 ★**src_line 정정(6차 배치A): ~~21~~ → 22**"
            u"(`%21 = sub i64 1, %20` @m10.ll:33897 의 `!dbg !40704` = `buff_value.rs:22`, 인라인 없음. L21 에는 `llvm.lifetime.start` 하나뿐)",
     "evidence": u"_gaibc/m10.ll:33897 !40704 → buff_value.rs:22 (사슬 단일 프레임)",
     "behavior_change": False, "found_by": "reused"},
    {"path": "/specs[3]/consts[1]", "kind": "실오류",
     "old": u"EntityType 태그 13 = Champion. 이 태그일 때만",
     "new": u"EntityType 태그 13 = Champion ★**src_line 정정(6차 배치A): ~~42~~ → 43**"
            u"(`icmp eq i64 %72, 13` @m10.ll:34074 사슬 = `entity.rs:1775(Entity::skill_cooldown)` ← `buff_value.rs:43` ← `buff_value.rs:41`. "
            u"L42 는 함수 전체에서 명령이 **0개**다). ⚠이 비교는 소스에 직접 적힌 게 아니라 **각 `*_cooldown()` 접근자 안에 있고** LLVM 이 CSE 해 L43 하나로 접었다. 이 태그일 때만",
     "evidence": u"_gaibc/m10.ll:34074 !40848 사슬 · atline.py 출력",
     "behavior_change": False, "found_by": "reused"},
    {"path": "/specs[3]/consts[2]", "kind": "실오류",
     "old": u"Entity.level > 2 여야 skill2_effect 슬롯이 열린다(entity.rs:1693 접근자 인라인)",
     "new": u"Entity.level > 2 여야 skill2_effect 슬롯이 열린다(entity.rs:1693 접근자 인라인 ★**src_line 정정(6차 배치A): ~~42~~ → 44**"
            u" — `icmp ugt i64 %76, 2` @m10.ll:34081 사슬 = `entity.rs:1693` ← `buff_value.rs:44` ← `41`)",
     "evidence": u"_gaibc/m10.ll:34081 !40855 사슬",
     "behavior_change": False, "found_by": "reused"},
    {"path": "/specs[3]/consts[3]", "kind": "실오류",
     "old": u"Entity.level > 4 여야 ult_effect 슬롯이 열린다(entity.rs:1701 접근자 인라인)",
     "new": u"Entity.level > 4 여야 ult_effect 슬롯이 열린다(entity.rs:1701 접근자 인라인 ★**src_line 정정(6차 배치A): ~~42~~ → 45**"
            u" — `icmp ugt i64 %76, 4` @m10.ll:34086 사슬 = `entity.rs:1701` ← `buff_value.rs:45` ← `41`)",
     "evidence": u"_gaibc/m10.ll:34086 !40868 사슬",
     "behavior_change": False, "found_by": "reused"},
    {"path": "/specs[3]/knobs[3]", "kind": "실오류",
     "old": u"buff_value.rs:42 를 통해 인라인된 entity.rs:1693",
     "new": u"buff_value.rs:**44**(~~42~~, 6차 배치A 정정) 를 통해 인라인된 entity.rs:1693",
     "evidence": u"_gaibc/m10.ll:34081 !40855 사슬", "behavior_change": False, "found_by": "reused"},
    {"path": "/specs[3]/knobs[4]", "kind": "실오류",
     "old": u"buff_value.rs:42 를 통해 인라인된 entity.rs:1701",
     "new": u"buff_value.rs:**45**(~~42~~, 6차 배치A 정정) 를 통해 인라인된 entity.rs:1701",
     "evidence": u"_gaibc/m10.ll:34086 !40868 사슬", "behavior_change": False, "found_by": "reused"},
    {"path": "/specs[3]/logic", "kind": "실오류",
     "old": u"[L21] enemy_ix = 1 - player.info.team(+0x930)",
     "new": u"[L22] enemy_ix = 1 - player.info.team(+0x930)   // ★줄 정정(6차 배치A): ~~L21~~ — L21 은 `let` 슬롯뿐",
     "evidence": u"_gaibc/m10.ll:33891~33897", "behavior_change": False, "found_by": "reused"},
    {"path": "/specs[3]/logic", "kind": "실오류",
     "old": u"[L42]   (c1,c2,c3) = if e.ty(+0x68) == 13 /*Champion*/ {",
     "new": u"[L43~45] ★줄 정정(6차 배치A): ~~L42~~ — L42 에는 명령이 0개다. 슬롯 3칸이 **L43(skill) / L44(skill2) / L45(ult)** 에 한 줄씩 있고,\n"
            u"//        `ty == 13` 비교는 소스에 직접 있는 게 아니라 각 `Entity::*_cooldown()`(entity.rs:1775/1791/1806) 안에 있으며\n"
            u"//        LLVM 이 CSE 해 **L43 하나로** 접었다.\n"
            u"[L43]   (c1,c2,c3) = if e.ty(+0x68) == 13 /*Champion*/ {",
     "evidence": u"atline.py m10.ll 34060~34135 buff_value.rs 41~46 출력", "behavior_change": False, "found_by": "reused"},

    # ── E10 open 해소 (rnd/debug 불변 실측) ────────────────────────
    {"path": "/specs[3]/open[0]", "kind": "분류오류",
     "old": u"다만 rnd(&mut StdRng)·debug(&mut DebugFrameData) 는 check_kill_die_tick 에서 변경될 수 있다",
     "new": u"다만 rnd(&mut StdRng)·debug(&mut DebugFrameData) 는 check_kill_die_tick 에서 변경될 수 있다 "
            u"→ ★해소(6차 배치A): **변하지 않는다**. `defensive_crisis` 호출 전후로 `StdRng` 320B 와 `DebugFrameData` 224B 를 통째로 바이트 diff 한 결과 "
            u"**14/14 케이스 전부 `rnd_changed=false · debug_changed=false`**(A6_o5.tsv). ★범위: `check_kill_die_tick` 이 실제로 실행된 케이스(die_imminent 계산 진입)만 잰 값이고, "
            u"`debug`/`rnd` 를 쓰는 다른 입력이 있을 가능성까지 배제하지는 않는다.",
     "evidence": u"A6_o5.tsv 14케이스 RESULT 줄", "behavior_change": False, "found_by": "inherited"},

    # ── E11 보강 (specs[4] 잔여 물음) ──────────────────────────────
    {"path": "/specs[4]/open[0]", "kind": "보강",
     "old": u"다만 `rnd`(&mut StdRng)는 readonly 가 아니라 PlayerState::strategy 안에서 소비될 수 있다(그 함수는 안 봄).",
     "new": u"다만 `rnd`(&mut StdRng)는 readonly 가 아니라 PlayerState::strategy 안에서 소비될 수 있다(그 함수는 안 봄). "
            u"→ 6차 배치A 실측: `player.strategy(&mut rnd, &game)` 을 **직접 호출**해 전후 320B 를 비교했더니 **바뀌지 않았다**(A6_o4.tsv `STRATEGY rnd_state_changed=false`). "
            u"★범위: 팀 전략이 확정돼 있는 한 상태 1종에서만 잰 값이라 '난수를 절대 안 쓴다'로 일반화하지 말 것(`TeamColorStrategy_random` 경로가 있다).",
     "evidence": u"A6_o4.tsv STRATEGY 줄 4케이스", "behavior_change": False, "found_by": "inherited"},

    # ── E12 보강 (specs[3] notes[0] 조각 누락) ─────────────────────
    {"path": "/specs[3]/notes[0]", "kind": "보강",
     "old": u"나머지 20개(iter_champions, distance_sq, abs_diff, is_some, is_some_and, TeamType PartialEq, Entity 접근자들 등)는 전부 인라인이라 개별 조각으로 존재하지 않는다",
     "new": u"나머지 20개(iter_champions, distance_sq, abs_diff, is_some, is_some_and, TeamType PartialEq, Entity 접근자들 등)는 전부 인라인이라 개별 조각으로 존재하지 않는다. "
            u"★보강(6차 배치A): 별도 define 3개는 **본체(m10.ll:33864~34294) · call_mut 심(m10.ll:55868~55976) · 이터레이터(m01.ll:37431~37606, 175줄)** 인데 "
            u"`history[0]` 의 `aux` 에는 **call_mut 심만** 올라가 있다 — m01.ll 쪽 이터레이터 조각은 아직 아무 라운드도 읽지 않았다(미탐색).",
     "evidence": u"fnparts.py defensive_crisis 출력", "behavior_change": False, "found_by": "reused"},
]

brief_errors = [
    u"`BRIEF.md §2` 가 「`ev>=4` 가 아직 **669행**」이라고 적었는데, 같은 라운드의 생성물 `BRIEF_FACTS.md §1` 은 계 858행 · ev≥4 **91.4%** = **784행**이다. "
    u"산문 절의 수치가 생성 절과 어긋난다(브리핑 자신이 §0 에서 경계한 바로 그 유형 — 「기억으로 사실을 쓴 것」).",
    u"지시문은 배치 A 에게 「`open` 5건 · `notes` 3건」이라고 했고 이는 맞다. 다만 `BRIEF_FACTS.md §3` 의 제목은 「`open` 11건 · `notes` 8건」(20함수 전체)이라 "
    u"같은 라운드 안에서 두 분모가 섞여 있다 — §1 처럼 **스코프 한 줄**을 §3 에도 박는 게 좋다.",
    u"★계약 결함: `applypatch.py` 의 `apply_error` 는 **문자열 필드만** 치환한다(`isinstance(cur, str)`). "
    u"그래서 이번에 찾은 `consts[].src_line` **오기 4건(정수 필드)** 을 `patch.json` 으로는 못 고친다 — `meaning` 문면에 정정을 실어 두었으니 "
    u"정수 갱신은 손으로/도구로 별도 반영해야 한다. (`/specs[3]/consts[0].src_line 21→22`, `[1] 42→43`, `[2] 42→44`, `[3] 42→45`)",
    u"★도구 결함(`mkspec3.evtier`): 근거 문면에 「추정」·「보인다」가 들어가면 무조건 `ev5` 로 떨어진다. "
    u"그런데 배치 A 의 `ev5` 2건은 둘 다 **오탐**이었다 — `/specs[0]/consts[6]` 은 「태그가 0이면 '**보인다**'」(가시성 서술), "
    u"`/specs[3]/knobs[19]` 는 「위협 **추정**이 정확해진다」(추정치라는 명사)다. 불확실성 어휘를 **문장 끝 서술어 위치**에서만 잡도록 좁혀야 한다.",
]

out = {"round": 6, "batch": "A", "errors": errors, "ev_up": ev, "brief_errors": brief_errors}
io.open(os.path.join(HERE, "patch.json"), "w", encoding="utf-8").write(
    json.dumps(out, ensure_ascii=False, indent=1))
print(u"errors=%d  ev_up=%d  brief_errors=%d" % (len(errors), len(ev), len(brief_errors)))
fb = {}
for e in errors:
    fb[e["found_by"]] = fb.get(e["found_by"], 0) + 1
print(u"found_by: " + " · ".join("%s=%d" % kv for kv in sorted(fb.items())))
