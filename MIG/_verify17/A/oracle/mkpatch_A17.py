# -*- coding: utf-8 -*-
"""17차 배치A patch.json 생성기 — specs[40]~[44]. 실행: cd /c/tfm2mods/MIG && python -X utf8 _verify17/A/oracle/mkpatch_A17.py"""
import sys, io
sys.path.insert(0, r"C:\tfm2mods\MIG")
sys.stdout.reconfigure(encoding="utf-8", errors="replace")
import mkpatch

p = mkpatch.Patch(round=17, batch="A")
ORC = u"오라클 v17A_o1.tsv 520/520 MATCH(#42·#43 동시, 축 main_goal{0..3}×elapsed{0,tps-1,tps,tps+1,10tps}×main_objective{None,0..11}×with_dive{F,T}, setting_ok=true tps=60)"

# ─────────────────────────── #40 v3_assign_anchor ───────────────────────────
S = "/specs[40]"
# open[0] camp_pos bool 인자 — g07/g02 본문 독해로 해소
p.fix(S + "/open[0]",
      old=u"'0번 팀 기준 좌표 대칭' 으로 추정. 본문은 _gcbc/g07.ll:152570 에 있으나 안 읽음(미탐색)",
      new=u"★해소 — g07.ll:152585 `%9 = xor i1 %2, true` → `%10 = zext`(map_def.rs:213 team_idx = !bool as usize) 이고, "
          u"클로저(g02.ll:7066~7133, map_def.rs:221~224)가 `map.camps`(MapDef+0x68 ptr/+0x70 len, CampDef 40B={pos:[(u64,u64);2] +0x0, ty:JungleType +0x20})를 "
          u"`c.ty == jungle`(g02.ll:7109 `icmp eq i8 %76, %69`)로 find 해 **`c.pos[team_idx]`**(g02.ll:7133 gep {i64,i64} %71, %42)를 돌려준다(못 찾으면 (0,0), TLS CAMP_POS_MEMO[jungle 8][team 2] 메모). "
          u"⟹ bool true(=팀 0) → pos[0], false → pos[1] = 캠프의 팀별 대칭 좌표 선택. 「사실 서술」이다",
      evidence=u"g07.ll:152585 `%9 = xor i1 %2, true, !dbg !184290`(!184290=map_def.rs:213) · g02.ll:7109 `%77 = icmp eq i8 %76, %69` · g02.ll:7133 `%84 = getelementptr inbounds nuw { i64, i64 }, ptr %71, i64 %42` · tcxdict CampDef(40B) pos +0x0 [(u64,u64);2] / ty +0x20 · MapDef 0x68 camps.buf.ptr",
      behavior_change=False, found_by="new", kind=u"보강")
# open[2] PassiveJunglePlan team vs player_team — 생성자 IR 로 해소
p.fix(S + "/open[2]",
      old=u"둘의 의미 차이는 미탐색",
      new=u"★해소 — PassiveJunglePlan::new(m04.ll:28355~28358, passive_jungle.rs:36)는 team(+0x48)·player_team(+0x50) **둘 다 인자 team(%2)** 으로 세팅하고, "
          u"new_counter_jungle(m04.ll:28131/28208 `sub i64 1, %2`, passive_jungle.rs:70 → 28316 store +72 ← %43, 28318 store +80 ← %2)은 **team = 1 − player_team(상대 정글 쪽)**, player_team = 내 팀이다. "
          u"⟹ `team` = 지금 도는 정글의 진영(카운터정글이면 상대 진영), `player_team` = 내 팀. v3_assign_anchor 가 `p.team == 0` 을 넘기므로 카운터정글 중엔 **상대 진영 쪽 캠프 좌표**가 앵커가 된다. 「사실 서술」이다",
      evidence=u"m04.ll:28355 `%9 = getelementptr inbounds nuw i8, ptr %0, i64 72` + 28356 `store i64 %2, ptr %9` + 28358 `store i64 %2, ptr %10`(+80) · m04.ll:28208 `%28 = sub i64 1, %2, !dbg !41732`(passive_jungle.rs:70) · m04.ll:28316 `store i64 %43, ptr %55`(+72, %43=phi[%36,%30]←1−%2) · 28318 `store i64 %2, ptr %56`(+80) · tcxdict PassiveJunglePlan team +0x48 / player_team +0x50",
      behavior_change=False, found_by="new", kind=u"보강")
# consts[8] '추정' 제거(ev5→4) + camp_pos 의미 확정
p.fix(S + "/consts[8]/meaning",
      old=u"내 팀이 0번 팀인가(캠프 좌표 대칭 선택으로 추정)",
      new=u"내 팀이 0번 팀인가. camp_pos 는 이 bool 로 CampDef.pos[0](true)/pos[1](false) 을 고른다(g07.ll:152585 `xor i1 %2, true` → 인덱스 · g02.ll:7133 pos[team_idx]) — IR m13.ll:10872 `%83 = icmp eq i64 %2, 0`",
      evidence=u"m13.ll:10872 `%83 = icmp eq i64 %2, 0, !dbg !21090`(handler.rs:1789) · g07.ll:152585 · g02.ll:7133",
      behavior_change=False, found_by="new", kind=u"보강")
# logic — 필드 표기 `f(+0x..)` 가 harvest_callees 에 호출로 긁혀 callees 잡음(team/line/jungle/plan/v3_armed/player_champion) 생성 → `f[+0x..]`
for old, new in [
    (u"player.info.team (<2)", u"player.info.team[<2]"),
    (u"data.cache.player_champion(+0x1e0)[team][pos]", u"data.cache.player_champion[+0x1e0][team][pos]"),
    (u"champ.x(+0x660), champ.y(+0x668)", u"champ.x[+0x660], champ.y[+0x668]"),
    (u"self.v3_armed(+0x1808)", u"self.v3_armed[+0x1808]"),
    (u"self.plan(+0x5e8 태그)", u"self.plan[+0x5e8 태그]"),
    (u"line_anchor(p.line(+0x706))", u"line_anchor(p.line[+0x706])"),
    (u"line_anchor(p.line(+0x618))", u"line_anchor(p.line[+0x618])"),
    (u"data.context.map(+0x20).camp_pos(p.jungle(+0x650), p.team(+0x638) == 0)", u"data.context.map[+0x20].camp_pos(p.jungle[+0x650], p.team[+0x638] == 0)"),
]:
    p.fix(S + "/logic", old=old, new=new,
          evidence=u"필드 읽기(IR load, 호출 아님): m13.ll:10741(+480) 10751(+1632) 10755(+1640) 10762(+6152) 10782(+1512) 10801(+1798) 10845(+1560) 10830(+1616) 10832(+1592) 10827(ctx+32). `name(` 표기가 mkspec3.harvest_callees(CALLNAME) 에 호출로 긁혀 callees 에 game_view::ClientData::team 등 잡음 6행을 만들었다",
          behavior_change=False, found_by="new", kind=u"보강")
p.fix(S + "/logic",
      old=u"[L1796] d2 = abs_diff(here.x, dest.x)² + abs_diff(here.y, dest.y)²",
      new=u"[L1796] d2 = abs_diff(here.x, dest.x)² + abs_diff(here.y, dest.y)²   // utils::distance_sq(utils.rs:9) 인라인",
      evidence=u"m13.ll:10914~10916 `distance_sq:9 <=v3_assign_anchor:1796` (!21096 사슬)",
      behavior_change=False, found_by="new", kind=u"보강")
# mem ev → 3 (tcxdict)
for j, ev in enumerate([
    u"tcxdict AbstractGameWithCache 0x1e0 = player_champion[0][0]@tag(Niche, &Entity)",
    u"tcxdict Entity 0x660 = x u64", u"tcxdict Entity 0x668 = y u64",
    u"tcxdict LegacyPlanHandler 0x1808 = v3_armed bool",
    u"tcxdict LegacyPlanHandler 0x5e8 = plan@tag(Niche 8B)",
    u"tcxdict LegacyPlanHandler 0x706 = plan@PassiveLine.0.line@tag",
    u"tcxdict LegacyPlanHandler 0x618 = plan@LineGanker.0.line@tag",
    u"tcxdict LegacyPlanHandler 0x650 = plan@PassiveJungle.0.jungle@tag",
    u"tcxdict LegacyPlanHandler 0x638 = plan@PassiveJungle.0.team usize",
    u"tcxdict GameContext 0x20 = map &MapDef",
]):
    p.ev(S + "/mem[%d]" % j, evidence=ev, to=3, frm=4, found_by="reused")

# ─────────────────────────── #41 fight_participants ───────────────────────────
S = "/specs[41]"
p.fix(S + "/consts[8]/src_line", old=26, new=631,
      evidence=u"m10.ll:39800 `%155 = add i64 %154, -1, !dbg !45851` · !45851 사슬 = effect.rs:26(scope range) ← fight_model.rs:631(closure$1) ← option.rs:1162(map) ← fight_model.rs:631(fight_participants). src_line 은 spec.src(fight_model.rs) 기준 줄이라(srclinebase.check_spec `fn == own`) 26 은 effect.rs 줄이다",
      behavior_change=False, found_by="reused", kind=u"실오류")
p.fix(S + "/consts[8]/meaning",
      old=u"Effect::range 인라인: growth_range * (level - 1)",
      new=u"Effect::range(effect.rs:26) 인라인: growth_range * (level - 1) 의 오프셋가감 −1(레벨 1 기준) — m10.ll:39800 `add i64 %154, -1`",
      evidence=u"m10.ll:39798~39801 `%154 = load i64, ptr %153`(+1480 level) → `%155 = add i64 %154, -1` → `%156 = mul i64 %155, %152`(+1192 growth_range)",
      behavior_change=False, found_by="new", kind=u"보강")
p.fix(S + "/consts[10]/src_line", old=623, new=624,
      evidence=u"m10.ll:39897 `%181 = phi i8 [ 1, %118 ], [ 1, %113 ], [ 0, %171 ], [ 0, %176 ]` — 인입 블록 %113/%118 의 !dbg = fight_participants:624(push, !45783/!45795). 623 은 ally_is_bound invoke 줄(39690)이고 리터럴 1 의 귀속 블록은 624",
      behavior_change=False, found_by="reused", kind=u"실오류")
p.fix(S + "/logic",
      old=u"[L623]   if ally_is_bound(version, rnd, data, player, a, fight_enemies, debug) { out.push((a, 0, true)); continue }",
      new=u"[L623]   if ally_is_bound(version, rnd, data, player, a, fight_enemies, debug) {\n[L624]     out.push((a, 0, true)); continue }",
      evidence=u"m10.ll:39690 invoke ally_is_bound !dbg fight_participants:623 · 39709~39726 push 블록 !dbg :624",
      behavior_change=False, found_by="new", kind=u"보강")
for old, new in [
    (u"data.context.setting.tick_per_second(+0x12f8) * 6", u"data.context.setting.tick_per_second[+0x12f8] * 6"),
    (u"team = player.info.team(+0x930) (<2)   Some(a) = cache.player_champion(+0x1e0)[team][i]", u"team = player.info.team[+0x930] (<2)   Some(a) = cache.player_champion[+0x1e0][team][i]"),
    (u"sp = max(a.stat_cached.move_speed(+0x640), 1)", u"sp = max(a.stat_cached.move_speed[+0x640], 1)"),
    (u"a.stat_buff_cached.range(+0x438) + e.range(+0x4a0) + e.growth_range(+0x4a8)*(a.level(+0x5c8)-1)", u"a.stat_buff_cached.range[+0x438] + e.range[+0x4a0] + e.growth_range[+0x4a8]*(a.level[+0x5c8]-1)"),
]:
    p.fix(S + "/logic", old=old, new=new,
          evidence=u"필드 읽기(IR load): m10.ll:39460(+4856) 39499(+2352) 39502(+480) 39777(+1600) 39795/39797/39799/39803(+1184/+1192/+1480/+1080). `name(` 표기가 harvest_callees 에 긁혀 callees 에 game_view::ClientData::team · nightmare::tick_per_second · WindowStatView::level 잡음이 생겼다",
          behavior_change=False, found_by="new", kind=u"보강")
p.fix(S + "/open[1]",
      old=u"Entity::distance 는 _gcbc 에 본문(안 읽음). 제곱거리인지 실거리인지 미확인 — reach(사거리 단위)와 뺄셈하므로 실거리로 추정",
      new=u"★해소 — Entity::distance(g06.ll:84197, entity.rs:2161)는 utils::distance(x,y,x2,y2)(g06.ll:87697, utils.rs:12~39) 호출이고, 그 안은 distance_sq(abs_diff²+abs_diff², utils.rs:9) → d²≤1,000,000 이면 usqrt(표+뉴턴, utils.rs:18) / 아니면 비트폭 기반 이분 isqrt(utils.rs:20~29) → **정수 제곱근 = 실거리**다. reach(사거리) 와 같은 단위. 「사실 서술」이다",
      evidence=u"g06.ll:84208 `%11 = tail call noundef i64 @…9game_core5utils8distance(i64 %4, i64 %6, i64 %8, i64 %10)` · g06.ll:87721~87723 `distance_sq:9` mul/mul/add · 87726 `icmp ult i64 %15, 1000001` · 87754~87813 `usqrt:66~80` · 87826~87841 `distance:27~29` 이분탐색 · 87846 `ret i64 %80`",
      behavior_change=False, found_by="new", kind=u"보강")
p.fix(S + "/open[3]",
      old=u"시그니처가 range(&self, &Entity) 인지 클로저가 더 더하는지는 dbg 로 구분 못 함. 합계식만 확정",
      new=u"★해소 — tcx 시그니처 `Effect::range(&Effect, &Entity) -> u64`(effect.rs:25, callees[10]) 이고, 합계식 전부(+0x4a0 range + +0x4a8 growth_range×(+0x5c8 level−1) + +0x438 stat_buff_cached.range)가 !45851 사슬상 **scope=range(effect.rs:26)** 안에 있다(closure$1 은 :631 `|e| e.range(a)` 호출만, 더하는 명령 0). 「사실 서술」이다",
      evidence=u"dloc m10.ll !45851 = line 26 in range[effect.rs] ← line 631 in closure$1 ← option.rs:1162 map ← fight_participants:631 · m10.ll:39794~39804 전부 `!dbg !45851` · 39810 phi 직전 closure$1 스코프 산술 명령 0",
      behavior_change=False, found_by="new", kind=u"보강")
p.fix(S + "/open[4]",
      old=u"소스가 min().unwrap() 인지 unwrap_or 인지는 표기 불가(unwrap_or<u64> DISubprogram 이 보이나 값이 접힘)",
      new=u"형태는 **`.min().unwrap_or(_)`** 로 확정 — !45505 = `line 0 in unwrap_or<u64>` inlinedAt fight_model.rs:**632**(631 의 reach 용 !45503 과 별개 인라인 사이트). 기본값 리터럴만 컴파일러가 Some 을 증명해 접혔으므로 그 값은 표기 불가",
      evidence=u"m10.ll:39849~39850 `#dbg_value(i64 poison, !45498, …, !45505)` · dloc !45505 = line 0 in unwrap_or<u64>[option.rs] ← line 632 in fight_participants · !45503 = unwrap_or<u64> ← 631",
      behavior_change=False, found_by="new", kind=u"보강")
p.fix(S + "/open[5]",
      old=u"본체는 안 읽음 — 이름으로 min_by(Ord::cmp) 임만 확정",
      new=u"★해소 — 본체 m12.ll:32239~32320: 첫 원소 거리(%1)를 acc 로 받아 나머지 적마다 `%24 = Entity::distance(a, e)` → `%25 = Ord::cmp(acc, d)` → `%26 = icmp slt i8 %25, 1`(acc ≤ d) → `%27 = select %26, acc, d`. 즉 **순수 최소값(동률이면 앞 원소 유지)** = `.min()`. 「사실 서술」이다",
      evidence=u"m12.ll:32299 `%24 = call … Entity8distance(…%23, …%22)` · 32304 `%25 = call noundef i8 @…Ord3cmp…call_once` · 32305 `%26 = icmp slt i8 %25, 1` · 32306 `%27 = select i1 %26, i64 %20, i64 %24` · 32316 `%31 = phi i64 [ %1, %2 ], [ %27, %18 ]`",
      behavior_change=False, found_by="new", kind=u"보강")
for j, ev in enumerate([
    u"tcxdict OperationData 0x0 = cache &AbstractGameWithCache", u"tcxdict OperationData 0x8 = context &GameContext",
    u"tcxdict OperationData 0x10 = blackboard &[Blackboard;2]", u"tcxdict GameContext 0x0 = pool &Bump",
    u"tcxdict GameContext 0x8 = setting &GameSetting", u"tcxdict GameSetting 0x12f8 = tick_per_second usize",
    u"tcxdict PlayerState 0x930 = info.team usize", u"tcxdict AbstractGameWithCache 0x1e0 = player_champion[0][0]@tag(Niche)",
    u"tcxdict TeamPlan 0x0 = ally_battle_stop_tick[0]@tag(8B)", u"tcxdict Blackboard 0xf8 = big_goal[0].1@tag(Niche 1B)",
    u"tcxdict Blackboard 0xf8 big_goal[0].1 + BigGoal::Battle.focus enum+0x8 = 0x100 focus@tag(Option<usize>)",
    u"tcxdict Blackboard 0x100 focus@tag +8 = 0x108 focus@Some.0 (Option<usize> 16B)",
    u"tcxdict Entity 0x5c0 = id usize", u"tcxdict Entity 0x640 = stat_cached.move_speed usize",
    u"tcxdict Entity 0x4c0 = attack_effect@tag(Niche i32)", u"tcxdict Entity 0x4a0 = attack_effect@Some.0.range u64",
    u"tcxdict Entity 0x4a8 = attack_effect@Some.0.growth_range(0x4a0+8, IR +1192)", u"tcxdict Entity 0x5c8 = level usize",
    u"tcxdict Entity 0x438 = stat_buff_cached.range usize",
]):
    p.ev(S + "/mem[%d]" % j, evidence=ev, to=3, frm=4, found_by="reused")

# ─────────────────────────── #42 / #43 with_runaway ───────────────────────────
for S, ir, ln in (("/specs[42]", u"m05.ll", (26867, 26873, 26892)), ("/specs[43]", u"m10.ll", (22841, 22847, 22866))):
    p.fix(S + "/open[2]",
          old=u"game.tick() 은 vtable+0x28 (divtable 정적 vtable 기준). 런타임 구현체는 미확정",
          new=u"game.tick() 은 vtable+0x28 = AbstractGame::tick(divtable 98% 일치, m04/m15 정적 vtable 2개 동일). 구현체는 호출 경로의 게임 구조체가 정한다 — tcx 상 impl 4종: Game(game.rs:1796)·SingleLaneGame·DeathMatchGame(game.rs:3863)·ExpectedGame(expected_game.rs:53). ExpectedGame 의 vtable 은 game_ai 안에서 dodge 계열(m04.ll:35693 dodge_all_move_ex · 40503 dodge_move_forward_to_target · 47229 is_safe_pos)에만 넘어가 이 경로엔 안 온다. 오라클(Game)에서 `set_tick(n)` → `data.cache.game.tick()==n` 실측. 「사실 서술」이다",
          evidence=u"divtable AbstractGame 0x28 → tick(@anon…157 m04 · @anon…80 m15, 98%) · _tcx game_core `<Game as AbstractGame>::tick` 등 impl 4종 · m04.ll:38838/42975/48251 invoke 인자 @anon.168add0ea037d45d276f5936ae758fe5.157 · " + ORC,
          behavior_change=False, found_by="new", kind=u"보강")
    p.fix(S + ("/open[3]" if S.endswith("42]") else "/open[4]"),
          old=u"logic/knobs 에만 실었다",
          new=u"logic/knobs 에만 실었다. IR " + ir + u":%d `xor i1 %%34, true` + 오라클(main_objective=5·with_dive=T → false, F → true 26/26)로 동작 확정 — 「사실 서술」이다" % ln[2],
          evidence=ir + u":%d `%%35 = xor i1 %%34, true` · " % ln[2] + ORC,
          behavior_change=False, found_by="new", kind=u"보강")
    for j in range(6):
        p.ev(S + "/consts[%d]" % j, evidence=ORC + u" · " + ir + u":%d switch/icmp" % (ln[0] if j == 0 else ln[1]), to=2, frm=4, found_by="reused")
    for j in range(3):
        p.ev(S + "/knobs[%d]" % j, evidence=ORC + u"(경계: elapsed=tps→false, tps+1→true · Morgard/Serpen/Defense/Nexus→false · PressEpic=!with_dive)", to=2, frm=4, found_by="reused")
    base = u"SinglePlanBattle" if S.endswith("42]") else u"BattlePlan"
    offs = ((u"0x40", u"main_goal BattlePlanGoal(24B, tag +0x0 Direct 0..3)"), (u"0x80" if base == u"SinglePlanBattle" else u"0xc0", u"start_tick usize"),
            (u"0x8d" if base == u"SinglePlanBattle" else u"0xff", u"main_objective Option<MainObjective>(3B, 니치 -1)"), (u"0x88" if base == u"SinglePlanBattle" else u"0xf6", u"with_dive bool"))
    for j, (o, nm) in enumerate(offs):
        p.ev(S + "/mem[%d]" % j, evidence=u"tcxdict %s %s = %s · 오라클이 이 오프셋에 ptr::write 한 값으로 520/520 MATCH" % (base, o, nm), to=3, frm=4, found_by="reused")
    p.ev(S + "/mem[5]", evidence=u"tcxdict OperationData 0x8 = context &GameContext · GameContext 0x8 = setting", to=3, frm=4, found_by="reused")
    p.ev(S + "/mem[6]", evidence=u"tcxdict GameSetting 0x12f8 = tick_per_second usize · 오라클 tps=60 경계 59/60/61 MATCH", to=3, frm=4, found_by="reused")

# ─────────────────────────── #44 take_misunderstood_received_chat ───────────────────────────
S = "/specs[44]"
p.fix(S + "/open[0]",
      old=u"★exe 0xe70c70 은 fnprobe 상 인자 레지스터 rcx 1개만 읽고 패닉 Location 이 handler.rs:1366:15 뿐이라 이 함수(4인자, handler.rs:588~596, 40B memmove 루프)가 아닐 가능성이 크다. 이 함수는 `internal fastcc` 라 유일 호출자에 인라인됐을 수 있다 — 미탐색 = 호출자 정체·exe 내 40B stride 루프 탐색(namebycaller)",
      new=u"★해소 — exe = **0xe4b8c0**(메인 09-13 fp 확정, 이 라운드 disrva 재확인): 352B·97명령·인자 rcx/rdx/r8/r9 4개, `mov rax,[rcx+0x7c0]`(len) `mov r10,[rcx+0x7b8]`(ptr) `lea r11,[rax*8]; lea r11,[r11+r11*4]`(×40) · `movzx ebx,[r9]`…`mov r15,[r9+0x10]`(chat 7바이트 선로드) · 루프 `cmp [r13+r9-0x20],rdx`(tick) `cmp [r13+r9-0x18],r8d`(from) `cmp [r13+r9-0x10],bl`(tag) → 점프테이블 · 일치 시 `dec rax` + 40B 복사(movups×2+mov) + `mov [rcx+0x7c0],rax` + `mov al,1`. 호출자 2곳(0xe49a50·0xe4c5c0). 옛 0xe70c70 은 이 함수가 아니었다(판정 반전)",
      evidence=u"disrva 0xe4b8c0 +0x160: e4b8cd `mov rax,[rcx+0x7c0]` · e4b8dd `mov r10,[rcx+0x7b8]` · e4b8e4/e4b8ec `lea r11,[rax*8]`/`lea r11,[r11+r11*4]` · e4b94d `cmp [r13+r9-0x20],rdx` · e4b954 `cmp [r13+r9-0x18],r8d` · e4b95b `cmp [r13+r9-0x10],bl` · e4b9ee `dec rax` · e4ba15 `mov [rcx+0x7c0],rax` · fnprobe callers @e49ec8(fn e49a50) @e4f87f(fn e4c5c0) — IR m13.ll:12483~12489(+1976/+1984/×40)·13867~13870 과 일대일",
      behavior_change=False, found_by="new", kind=u"실오류")
p.fix(S + "/consts[2]/meaning", old=u"new_len = len - 1 (swap_remove)",
      new=u"new_len = len − 1 의 오프셋가감 −1 (swap_remove, m13.ll:13867 `add nsw i64 %8, -1` → 13870 store +0x7c0)",
      evidence=u"m13.ll:13867 `%429 = add nsw i64 %8, -1` · 13870 `store i64 %429, ptr %7`(+1984)", behavior_change=False, found_by="new", kind=u"보강")
p.fix(S + "/consts[3]/meaning", old=u"swap_remove 의 memmove 크기 = 원소 (usize, Position, Chat) 40B (stride 로도 쓰임)",
      new=u"swap_remove 의 memmove 길이 = 원소 (usize, Position, Chat) 40B (m13.ll:13869 `llvm.memmove … i64 40`; 12487 `mul nuw nsw i64 %8, 40` stride 로도 쓰임)",
      evidence=u"m13.ll:13869 `tail call void @llvm.memmove.p0.p0.i64(… i64 40, i1 false)` · 12487 `%9 = mul nuw nsw i64 %8, 40`", behavior_change=False, found_by="new", kind=u"보강")
p.ev(S + "/mem[0]", evidence=u"tcxdict LegacyPlanHandler 0x7b8 = misunderstood_received_chats.buf.inner.ptr · exe e4b8dd `[rcx+0x7b8]`", to=3, frm=4, found_by="reused")
p.ev(S + "/mem[1]", evidence=u"tcxdict LegacyPlanHandler 0x7c0 = misunderstood_received_chats.len · exe e4b8cd `[rcx+0x7c0]`", to=3, frm=4, found_by="reused")
p.ev(S + "/mem[11]", evidence=u"tcxdict LegacyPlanHandler 0x7c0 = misunderstood_received_chats.len · exe e4ba15 `mov [rcx+0x7c0],rax`(rax=len−1)", to=3, frm=4, found_by="reused")

p.brief_errors = [
    u"§4 G10 목록이 [42]/[43] open[2](「미확정」 꼬리)만 잡고 같은 성격의 open[3]/[4](「실었다」 꼬리 = 사실 서술)는 놓쳤다 — specgate.gate10 의 FACT_TAIL 어휘가 mkspec3.FACT_TAIL(넣었다·적었다 등 포함)보다 좁아 두 도구의 판정이 다르다. 게이트 어휘를 mkspec3 와 공유하라",
    u"spec_44 §3 전문이 자기모순이다: 머리표 exe=`e4b8c0`(09-13 fp 확정) 인데 open[0] 은 「exe 0xe70c70 은 이 함수가 아닐 가능성」을 그대로 싣고 있다 — 주소를 고칠 때 open 도 같이 닫혔어야 한다(G7 과열림의 exe 축 판)",
    u"라운드 지시문(프롬프트)의 「G4(callees_unmatched ≥8)」는 배치A 5함수 중 해당이 0건(6/3/4/4/4)이고, RUNBOOK §S5-b 의 G4 는 「술어 시그니처」라 이름이 다르다 — 지시가 가리키는 게이트가 무엇인지 표에 없는 이름을 썼다",
    u"§1 「ev≥4(미실행) 15」인 #42/#43 은 pub·순수·TLS 없음이라 오라클이 30분짜리였다 — vis=pub 함수는 §1 표에 「오라클 우선」 표식을 붙여 주면 배치가 먼저 집는다",
    u"§2 TEMPLATE.rs 빌드(_verify3/build.sh)가 출력 exe 를 %TEMP%\\tfm2_spanprobe\\<파일명>.exe 로 떨구는데 배치 4개가 같은 폴더를 쓴다 — 파일명을 o1.rs 로 지으면 다른 배치와 충돌한다(이번엔 v17A_ 접두로 회피). 지시문에 「프로브 파일명에 배치 접두」를 명시하라",
]
p.save()
print(u"errors=%d ev_up=%d brief=%d" % (len(p.errors), len(p.ev_up), len(p.brief_errors)))
