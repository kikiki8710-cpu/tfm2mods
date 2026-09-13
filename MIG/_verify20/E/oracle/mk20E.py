# -*- coding: utf-8 -*-
import sys, io, json
sys.path.insert(0, r"C:\tfm2mods\MIG")
import mkpatch
p = mkpatch.Patch(round=20, batch="E")
S = "/specs[110]"

# ── A. sig.ret(v2 returns): positioning_score 콜리 갱신 주장 반증 ──
p.errors.append({"path": S+"/sig/returns", "kind": "실오류",
  "old": "콜리 경유 team_plan·positioning_score 갱신",
  "new": "콜리 경유 team_plan 갱신 가능(`&TeamPlan` 인자가 readonly 없음 = V54Counter AtomicUsize 0x3f8/0x400/0x408 내부가변성) · positioning_score 는 콜리 갱신 없음(BattlePlan::update %5 `readonly dereferenceable(2760)` · check_kill 인자 poison)",
  "evidence": "m13.ll:37661 `ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(2760) %1290, ptr noundef nonnull align 8 %139` (positioning_score=readonly · team_plan=readonly 없음) · m10.ll:23429 BattlePlan::update define %5 readonly · m13.ll:43887 check_kill 호출 6번째 인자 `ptr noalias nonnull readonly … poison` · tcxdict TeamPlan 0x3f8 v54_fs_pairings=V54Counter(std::sync::atomic::Atomic<usize>)",
  "behavior_change": False, "found_by": "new"})

# ── B. params[1] version 게이트 목록 ──
p.fix(S+"/sig/params[1]/role",
  old="게이트: >=2(ugt 1) 분기 411·423·535·(solokill 내부)",
  new="게이트: >=2(ugt 1) 분기 411(423 은 %1258 재사용)·535·786(792 는 %3459 재사용)·828(837 은 %4142 재사용) — solokill 인라인(43322~44660) 안엔 version 분기 없음(check_kill/can_near_enemies_range/try_engage 인자 전달만)",
  evidence="m13.ll:37607 `%1258 = icmp ugt i64 %1257, 1` (L411) · 37677 `br i1 %1258` (L423) · 38792 `%1759 = icmp ugt i64 %1758, 1` (L535) · 43157 `%3459 = icmp ugt i64 %3458, 1` (L786) · 43189 `br i1 %3459` (L792) · 44952 `%4142 = icmp ugt i64 %4141, 1` (L828) · 45007 `br i1 %4142` (L837) · 43322~44660 에 `icmp ugt i64 %x, 1` 0건, %3520(version 로드)의 소비처는 43887·44021·44633 호출 인자뿐",
  behavior_change=False, found_by="new")

# ── C. params[2] rnd 직접 소비(gen_range) 보강 ──
p.fix(S+"/sig/params[2]/role",
  old="콜리(strategy/try_engage/update/resolve_join_stake/…)에 전달만",
  new="콜리(strategy/try_engage/update/resolve_join_stake/gen_range(L803·848 채팅 확률, 시드 스트림 1회 소비)/…)에 전달만",
  evidence="m13.ll:43237 · 45050 `invoke noundef i64 @…StdRng…Rng9gen_range…(ptr … %2, …)` (L803·L848)",
  behavior_change=False, found_by="new", kind="보강")

# ── D. check_kill: positioning_score 도 poison ──
p.fix(S+"/logic",
  old="check_kill(version, rnd/*IR: poison — 콜리가 안 씀*/, player, data, &self.positioning_score, debug)",
  new="check_kill(version, rnd/*IR: poison — 콜리가 안 씀*/, player, data, &self.positioning_score/*IR: poison — 콜리가 안 씀*/, debug)",
  evidence="m13.ll:43887 `invoke void @…check_kill(ptr … sret … %29, i64 noundef %3520, ptr noalias nonnull align 16 poison, ptr … %3, ptr … %4, ptr noalias nonnull readonly align 8 captures(address, read_provenance) poison, ptr … %5)` · define m13.ll:54538 `%2 readnone captures(none)` · `%5 readonly captures(none)`(noundef 없음)",
  behavior_change=False, found_by="new", kind="보강")
p.fix(S+"/open[7]/q",
  old="check_kill(L173) 호출에 rnd 자리가 `ptr poison` — 콜리가 rnd 를 안 읽는다는 뜻(IR 확정)",
  new="check_kill(L173) 호출에 rnd·positioning_score 두 자리가 `ptr poison`(m13.ll:43887) — 콜리가 둘 다 안 읽는다는 뜻(IR 확정 · define m13.ll:54538 %2 readnone / %5 readonly captures(none))",
  evidence="m13.ll:43887 (위와 동일) · m13.ll:54538 check_kill define 속성",
  behavior_change=False, found_by="new", kind="보강")

# ── E. mem[6] 필드명 오귀속 ──
p.fix(S+"/mem[6]/name", old="champions[2][5]", new="player_champion[2][5]",
  evidence="tcxdict AbstractGameWithCache 0x1e0 = `player_champion[0][0]@Some.0` ([[Option<&Entity>;5];2] 80B) — champions()/iter_champions 는 접근자 이름, 필드명은 player_champion(다른 명세 전부 player_champion 표기)",
  behavior_change=False, found_by="reused")

# ── F. 스택 BattlePlan 쓰기 전용 행 dir r→w ──
DIRW = {62:("0x78","chats.len","W 37675·37817·38779·38897·43218·45031 (`store i64 0`), load 0건"),
        63:("0xff","main_objective@tag","W 43206·45019 (`store i24 %…`), load 0건"),
        67:("0xb8","dive_join_tick","W 45002 (`store i64 %4160`), load 0건"),
        68:("0xf6","with_dive","W 44991 (`store i8 1`), load 0건"),
        69:("0x107","entry_src","W 37655·38058·43188·45006 (`store i8 5/3/10/11`), load 0건"),
        70:("0xfc","ff_stake_join","W 37837, load 0건")}
for i,(off,nm,ev) in DIRW.items():
    p.errors.append({"path": S+"/mem[%d]/dir"%i, "kind": "실오류", "old": None, "new": "w",
      "evidence": "m13.ll 본 함수 범위 280B alloca(BattlePlan) +%s 접근 전수: %s — 읽기 없이 쓰기만이라 dir=r 은 반대 방향(note 자체가 「쓰기」라 적음)"%(off,ev),
      "behavior_change": False, "found_by": "new"})

# ── G. ObjectivePostureKind variant 이름 ──
p.fix(S+"/mem[49]/note", old="tag∈{3,4} → true (variant 이름 = unknown 참조)",
  new="tag∈{3 WaitGroup, 4 SoftDisengage} → true (tcxdict --enum ObjectivePostureKind: 0 Commit 1 HoldCamp 2 Screen 3 WaitGroup 4 SoftDisengage · Direct 인코딩)",
  evidence="tcxdict --enum ObjectivePostureKind (team_plan.rs:137, 1B Direct) 3=WaitGroup 4=SoftDisengage · tcxdict ObjectivePosture 0x50 kind",
  behavior_change=False, found_by="reused", kind="보강")
p.fix(S+"/logic", old="// 인라인: p.kind[+0x50] 태그 ∈ {3,4}",
  new="// 인라인: p.kind[+0x50] 태그 ∈ {3 WaitGroup, 4 SoftDisengage}",
  evidence="tcxdict --enum ObjectivePostureKind", behavior_change=False, found_by="reused", kind="보강")

# ── H. 인라인 콜리 호출 표기 명시(callees 자동수집이 `name(` 만 긁는다) ──
p.fix(S+"/logic", old="// dive_rejoin_cd 인라인(…:973)",
  new="// dive_rejoin_cd(version, tps) 인라인(engage.rs:971~973 · fn(usize,usize)->usize) = tps*4 + 1 — version 인자는 `#dbg_value(i64 poison)` = 미사용",
  evidence="m13.ll:38039 `#dbg_value(i64 poison, !37133)`(!37133 = version arg1 line 971) · 38040 `#dbg_value(i64 %1473, !37136)`(tps) · 38041 `%1474 = shl i64 %1473, 2` · 38042 `%1475 = add i64 %1467, 1` · 38043 `add` · 38044 `icmp ugt i64 %1464, %1476` · tcx game_ai::plan_legacy::handler::engage::dive_rejoin_cd sig fn(usize, usize) -> usize",
  behavior_change=False, found_by="new", kind="보강")
p.fix(S+"/logic", old="// set_main_objective 인라인, 3B 복사",
  new="// set_main_objective(objective) 인라인(battle.rs:349~350), 3B 복사",
  evidence="m13.ll:43205 DISubprogram set_main_objective battle.rs:349 (line 350) inlinedAt L794 · tcx game_ai::plan_legacy::old::BattlePlan::set_main_objective",
  behavior_change=False, found_by="new", kind="보강")
p.fix(S+"/logic", old="PassiveJungle(5) → is_counter_jungle = plan.team[0x638] == plan.player_team[0x640]",
  new="PassiveJungle(5) → is_counter_jungle() 인라인(passive_jungle.rs:103~104) = plan.team[0x638] == plan.player_team[0x640]",
  evidence="m13.ll:37264 DISubprogram is_counter_jungle passive_jungle.rs:103 inlinedAt is_passive types.rs:254 ← L391 · 37264~37269 gep 1592·1600 → icmp eq · tcx game_ai::plan_legacy::old::PassiveJunglePlan::is_counter_jungle",
  behavior_change=False, found_by="new", kind="보강")
p.fix(S+"/logic", old="// 인라인: ctx.tutorial ∈ {0,5,7,8} && tick",
  new="// 인라인(GameContext::is_line_phase(tick) runner.rs:397~399 → tutorial.spawn_epic() runner.rs:262~263 + GameSetting::is_line_phase(tick) setting.rs:702~703): ctx.tutorial ∈ {0 None,5 MidBottom,7 Line,8 Total} && tick",
  evidence="m13.ll:41118~41123 `switch i8 %2653 … i8 0/7/8/5 → %2654` !dbg !45535 = spawn_epic:263 ← is_line_phase:399 ← L673 · 41129 DISubprogram is_line_phase setting.rs:702 (line 703) · tcxdict --enum TutorialType 0 None 5 MidBottom 7 Line 8 Total",
  behavior_change=False, found_by="new", kind="보강")

# ── I. 누락 행 삽입 ──
p.errors.append({"op": "insert", "path": S+"/mem", "at": 77, "guard": "dive_tower@tag", "guard_key": "name",
  "new": {"base": "BattlePlan", "offset": "0xfe", "name": "dive_tower@tag", "dir": "w",
          "value": "i8 3 (Some(TowerType::TwinA))",
          "note": "L833 plan.dive_tower = Some(TowerType::TwinA(3)) — 넥서스 공격 다이브(§J) 스택 battle 에 저장 후 self.plan 으로 memcpy. tcxdict BattlePlan 0xfe dive_tower@tag(Niche, 255=None) · tcxdict --enum TowerType 3=TwinA"},
  "evidence": "m13.ll:45003 `%4166 = getelementptr inbounds nuw i8, ptr %57, i64 254` · 45004 `store i8 3, ptr %4166` (!dbg L833) · logic/consts[52] 가 인용하는데 mem 에 없었다(G18 누락)",
  "behavior_change": False, "found_by": "new", "kind": "보강"})
p.errors.append({"op": "insert", "path": S+"/mem", "at": 71, "guard": "hp (현재 HP)", "guard_key": "name",
  "new": {"base": "Entity", "offset": "0x670", "name": "hp (현재 HP)", "dir": "r",
          "note": "hp*100/stat_cached.hp(0x628) — L677 champ(>59) · L684 아군(closure#37 >59) · solokill L161 적(closure#1 <35). tcxdict Entity 0x670 hp"},
  "evidence": "m13.ll:41148 `%2669 = getelementptr inbounds nuw i8, ptr %124, i64 1648` · 41149 `load i64` · 41150 `mul i64 %2670, 100` · 41151 `udiv i64 %2671, %2666` · 41152 `icmp ugt i64 %2672, 59` (!dbg L677) · logic 이 champ.hp 를 인용하는데 mem 에 0x670 행이 없었다(G18 누락)",
  "behavior_change": False, "found_by": "new", "kind": "보강"})

# ── J. ev_up: tcxdict 로 오프셋·필드명 확인된 mem 행 4→3 (vtable 행 7·8·9·53 제외, 25·40 은 이미 3) ──
TCX = {0:"PlayerState 0x930 = info.team",1:"PlayerState 0x9c0 = info.position@tag(4B Direct)",2:"PlayerState 0x180 = info.parameter(AthleteParameter 744B 선두)",
 3:"OperationData 0x0 = cache",4:"OperationData 0x8 = context",5:"OperationData 0x10 = blackboard",6:"AbstractGameWithCache 0x1e0 = player_champion[0][0]",
 10:"MobaMode 0x1a0 = jungle_runner.epic.live_list.buf.inner.ptr",11:"MobaMode 0x1a8 = jungle_runner.epic.live_list.len",12:"MobaMode 0x1d0 = jungle_runner.serpen.live_list.buf.inner.ptr",13:"MobaMode 0x1d8 = jungle_runner.serpen.live_list.len",
 14:"GameContext 0x20 = map",15:"GameContext 0x8 = setting",16:"GameSetting 0x12f8 = tick_per_second",17:"GameContext 0x3b = debug",
 18:"Entity 0x0 = team@tag",19:"Entity 0x8 = team@Player.0",20:"Entity 0x38 = visible_state[0]@tag (visible_state [VisibleState;2] 48B, [1]=0x50 → stride 24)",21:"Entity 0x5c0 = id",22:"Entity 0x660 = x",23:"Entity 0x668 = y",
 24:"LegacyPlanHandler 0x98 = data.epic.epic_ally_tick",26:"LegacyPlanHandler 0xf8 = team_plan.ally_battle_stop_tick[0]@tag ([Option<usize>;5] 80B stride 16)",27:"LegacyPlanHandler 0x2b2 = team_plan.current_steal_session@tag(Niche = @Some.0.enemy_jungler_alive_at_start)",
 28:"LegacyPlanHandler 0x318 = team_plan.last_battle_tick",29:"LegacyPlanHandler 0x517 = team_plan.objective@tag",30:"LegacyPlanHandler 0x518 = team_plan.objective@Some.0@Morgard.phase / @DefenseLine.0 (variant 별 +1 페이로드)",31:"LegacyPlanHandler 0x519 = team_plan.objective@Some.0@Morgard.with_battle / @ComebackPick.ready",
 32:"LegacyPlanHandler 0x5e8 = plan@tag",33:"LegacyPlanHandler 0x638 = plan@PassiveJungle.0.team",34:"LegacyPlanHandler 0x640 = plan@PassiveJungle.0.player_team",35:"LegacyPlanHandler 0x1480 = last_dive_abandon_tick",36:"LegacyPlanHandler 0x1488 = last_response_bail_tick",37:"LegacyPlanHandler 0x1490 = last_lost_fight.0",38:"LegacyPlanHandler 0x1498 = last_lost_fight.1",39:"LegacyPlanHandler 0x180a = v3_epicops_armed",
 41:"Blackboard 0xf8 = big_goal[0].1@tag (big_goal[0].0 = 0xf0, stride 32)",42:"Strategy 0xc = object_battle@tag",43:"Strategy 0xf = object_finish@tag",44:"Strategy 0x11 = focused@tag",45:"BattlePlan 0x58 = sub_goal@tag",46:"BattlePlan 0xa8 = prev_die_eval",47:"BattlePlan 0x108 = exit_src",48:"FightPrediction 0x38 = line@tag",49:"ObjectivePosture 0x50 = kind@tag",
 50:"AbstractGameWithCache 0x130 = twin_towers[0].buf.ptr.pointer ([bumpalo Vec;2] 64B stride 32)",51:"AbstractGameWithCache 0x170 = nexus[0]@tag",52:"AbstractGameWithCache 0x180 = top_tower[0]@tag (0x190 top_tower2 · 0x1a0 mid_tower · 0x1b0 mid_tower2 · 0x1c0 bottom_tower · 0x1d0 bottom_tower2)",
 54:"MobaMode 0x240 = epic_minion_buff_time[0]",55:"GameContext 0x38 = tutorial@tag",56:"GameSetting 0x8a8 = epic_jungle.first_spawn_tick",57:"MapDef 0x1c98 = bushes[0][0]",58:"Entity 0x68 = ty@tag",59:"Entity 0x128 = ty@Tower.info.ty@tag",60:"Entity 0x628 = stat_cached.hp",61:"Entity 0x640 = stat_cached.move_speed",
 62:"BattlePlan 0x78 = chats.len",63:"BattlePlan 0xff = main_objective@tag",64:"Strategy 0xd = tower_press@tag",65:"Strategy 0xe = morgard_defense@tag",66:"BigGoal 0x0 = @tag",67:"BattlePlan 0xb8 = dive_join_tick",68:"BattlePlan 0xf6 = with_dive",69:"BattlePlan 0x107 = entry_src",70:"BattlePlan 0xfc = ff_stake_join",72:"BattlePlan 0x0 = support_target@tag",
 # 71 이후: hp 삽입(at=71) 뒤 인덱스 +1
 73:"LegacyPlanHandler 0x5e8 = plan@tag",74:"LegacyPlanHandler 0x5f0 = plan@Battle.0(support_target@tag 선두 · BattlePlan 280B)",75:"LegacyPlanHandler 0x7d8 = chats.len",76:"LegacyPlanHandler 0x7d0 = chats.buf.inner.ptr.pointer.pointer",77:"LegacyPlanHandler 0x5e8 = plan@tag"}
v3 = json.load(io.open(r"C:\tfm2mods\MIG\_spec\specs20_v3.json", encoding="utf-8"))["specs"][110]["mem"]
for i, ev in TCX.items():
    src = i if i < 71 else i-1          # 삽입(at=71) 전 v3 인덱스 → guard 문자열용
    row = v3[src]
    p.ev_up.append({"path": S+"/mem[%d]"%i, "from": 4, "to": 3, "guard": ("player_champion[2][5]" if i==6 else row["name"]),
                    "evidence": "tcxdict %s (오프셋·필드명 정본 일치)" % ev, "found_by": "reused"})

# ── K. 지시 오류 ──
p.brief_error("§5 는 「행 추가·삭제가 된다」고 하나 참조구현 mkpatch.py 에 insert/delete 헬퍼가 없어 배치가 dict 를 손으로 써야 한다(「손으로 JSON 을 쓰다 오타를 내지 마라」와 모순).")
p.brief_error("v3 `sig.ret` 은 v2 `signature.returns` 인데 applypatch.V2KEY 에 ret→returns 매핑이 없어 `/sig/ret` 경로로는 적용이 안 된다 — 배치가 v2 키(`/sig/returns`)를 알아야 하고, mkpatch.locate 는 v3 만 읽어 그 경로를 거부한다(force 필요).")
p.brief_error("v2 `reads[]` 행엔 `dir` 키가 없어(v3 가 배열 소속으로 파생) `dir` 정정은 old=null 로만 적용되는데 mkpatch.fix 는 v3 값 'r' 과 비교해 거부한다 — 파생 필드 정정 규약(§5 「파생 필드는 errors 로 못 쓴다」)에 dir 의 예외 절차가 없다.")
p.brief_error("§4 「이 배치 몫 미해소 0건」인데 G14 는 `&mut battle` 이 콜리로 나가면 양방향 보류라 스택 BattlePlan 쓰기전용 행 6개의 dir=r 을 못 본다 — 「보류」가 「검사됨」으로 집계된다.")
p.save()
