# -*- coding: utf-8 -*-
"""15차 배치 B patch.json 생성기 — mkpatch 참조구현 사용(old 는 정본 대조)."""
import sys, io
sys.path.insert(0, r"C:\tfm2mods\MIG")
sys.stdout.reconfigure(encoding="utf-8")
import mkpatch

p = mkpatch.Patch(round=15, batch="B")

# ⚠mkpatch.Patch.ev() 는 `m.group(3) is None` 을 검사해 2단 배열 경로(`/specs[i]/knobs[j]`)를 전부 거부한다(group(4) 가 idx).
#   applypatch 계약은 2단을 받으므로 여기서 같은 검증(행 실재·mem 상한 3)만 하고 직접 넣는다.
def _ev(self, path, evidence, to=2, frm=4, found_by="reused"):
    m = mkpatch.PATH.match(path)
    if not m or m.group(4) is None:
        raise ValueError(u"ev 상향은 배열 원소만: %s" % path)
    if mkpatch.locate(path) is None:
        raise AssertionError(u"그 행이 없다 — %s" % path)
    fld = m.group(3) or m.group(2)
    if fld == "mem" and int(to) < 3:
        to = 3
    self.ev_up.append({"path": path, "from": int(frm), "to": int(to), "evidence": evidence, "found_by": found_by})
    return self
mkpatch.Patch.ev = _ev
O = u"오라클 _verify15/B/oracle/v15B_o1.rs (tick0 32/32 · tick1000 35/35 MATCH, MISMATCH 0)"
G07 = u"_gcbc g07.ll:157039 `%25 = add i64 %24, 120, !dbg !187710`(!187710 = blackboard.rs:350). 157005 는 define 줄"

# ═════════ #25 v22_visible_enemy_is_runaway_threat ═════════
p.fix("/specs[25]/knobs[2]/where", old=u"g07.ll:157005~157049", new=u"g07.ll:157039",
      evidence=G07, behavior_change=False, found_by="reused")
p.fix("/specs[25]/logic",
      old=u"        if !bb.is_recent_visible(data.cache.game(&dyn AbstractGame, cache+0x0/+0x8), player, enemy) { return false }",
      new=u"        if !bb.is_recent_visible(data.cache.game(&dyn AbstractGame, cache+0x0/+0x8), player, enemy) ||\n"
          u"           is_ignored_well_enemy(version, player, enemy) { return false }\n"
          u"        //  ★소스 표기 확정(줄 길이 산술, rmeta_srcmap fight_model.rs 1168~1176): 하나의 `||` 식이 두 줄에 걸친다 —\n"
          u"        //    L1169(96자) `  if !data.blackboard[1 - player.info.team].is_recent_visible(data.cache.game, player, enemy) ||`\n"
          u"        //    L1170(51자) `    is_ignored_well_enemy(version, player, enemy) {` / L1171(17자) `    return false;` / L1172 `  }`\n"
          u"        //    (별도 두 if 로는 L1170=51 이 `return false;`(17) 와 안 맞고, `&&` 한 줄로는 96 이 안 나온다 — 후보 중 ±0 유일)",
      evidence=u"rmeta_srcmap game_ai fight_model.rs 1166~1178: L1168=152(151+LF: `pub(crate) fn v22_…(…) -> bool {` 검산 151 · tcx sp c2=150 정합) L1169=97 L1170=52 L1171=18 L1172=4 L1173=1 L1174=81 L1175=58 L1176=2. "
               u"IR m10.ll:49412~49425 `br i1 %18, label %20, label %54, !dbg !55057`(1169) → 블록 %31 is_enemy_well_danger !dbg !55071(756 inlinedAt !55063=1170) → phi %55 [false,%10],[false,%31]. "
               u"open[3] 「column 부재로 표기 불가」는 IR 한정 판정이었고 줄 길이 산술로 확정됐다(판정 반전)",
      behavior_change=False, found_by="new")
p.fix("/specs[25]/logic",
      old=u"        if ignored { return false }                          // 적 샘(well) 위험구역에 선 적은 위협으로 안 본다",
      new=u"        if ignored { return false }                          // 적 샘(well) 위험구역에 선 적은 위협으로 안 본다 (= 위 `||` 의 둘째 항; 별도 if 아님)\n"
          u"        //  is_enemy_well_danger(m03.ll:144500~144563, path_finder.rs:1032~1035, version 미사용) = 적 샘 코너의 L자 영역:\n"
          u"        //    player.info.team(+0x930)==1: (x<=64000 && 800000<=y<=960000) || (x<=160000 && 896000<=y<=960000)\n"
          u"        //    else                       : (800000<=x<=960000 && y<=64000) || (896000<=x<=960000 && y<=160000)\n"
          u"        //    (IR `add i64 %2, -800000; icmp ult …, 160001` 등 6쌍, 헬퍼 path_finder.rs:5655~5664 인라인)",
      evidence=u"m03.ll:144500 define is_enemy_well_danger(i64 %0, ptr %1, i64 %2, i64 %3): `%7 = icmp eq i64 %6, 1, !dbg !79990`(team) · 블록8 `%9 = add i64 %2, -800000 / %10 = icmp ult i64 %9, 160001 / %11 = icmp ult i64 %3, 64001` · 블록13 `%14 = icmp ult i64 %2, 64001 / %15 = add i64 %3, -800000 / %16 = icmp ult i64 %15, 160001` · 블록18 `icmp ult i64 %2, 160001 / add i64 %3, -896000 / icmp ult …, 64001` · 블록23 `add i64 %2, -896000 / icmp ult …, 64001 / icmp ult i64 %3, 160001` · %0(version) 미사용. open[1] 해소",
      behavior_change=False, found_by="new", kind=u"보강")
p.fix("/specs[25]/logic",
      old=u"        //  ⚠인자 순서: max_range_cached(data, champ, target) 에 (enemy, champ) 가 들어간다 = '적이 아군 champ 를 때릴 수 있는 최대 사거리'",
      new=u"        //  ⚠인자 순서: max_range_cached(data, champ, target) 에 (enemy, champ) 가 들어간다 = '적이 아군 champ 를 때릴 수 있는 최대 사거리'\n"
          u"        //  max_range_cached(m10.ll:50689~51083) = battle::max_range(champ, target)(m10.ll:52940~53308: 평타·스킬·스킬2·궁 4슬롯 각각 CastingTarget::check(target) 통과 시 Effect::range_adjust 의 llvm.umax 누적)\n"
          u"        //    를 TLS `MaxRangeCache`(LocalKey::with 클로저 m00.ll:79611, 10×10 표, 키 = game.seed()[vtable+0x20]·game.tick()[+0x28]·두 엔티티의 player_champion 슬롯 idx(id +0x5c0 로 탐색)) 로 메모.\n"
          u"        //    둘 중 하나라도 player_champion(+0x1e0) 에 없으면 메모 없이 직접 max_range(블록 %179). ⚠오라클 주의: 같은 tick 반복 호출은 첫 값을 재생한다(TLS 메모, TEMPLATE 함정③)\n"
          u"        //  소스 L1174(80자 검산) `  let threat_range = (max_range_cached(data, enemy, champ) + 40000).min(150000);` / L1175(57자) `  enemy.distance_sq(champ) <= threat_range * threat_range`",
      evidence=u"m10.ll:50689 define max_range_cached: %9=gep %1,1472(id) · %11=load cache · 슬롯 +480..+552 10칸 id 비교(phi %92/%93 = team,pos) · 51035 `br i1 %97, label %181, label %179` · 51038 `call battle::max_range(%1,%2)` · 51053 vtable+32(seed)·51058 +40(tick) · 51072 `call …LocalKey<RefCell<MaxRangeCache>>::with(max_range_cached::{closure#0})`. m00.ll:79611~79995 클로저: 79969 `invoke battle::max_range` (miss 경로) 후 store. divtable AbstractGame 0x20=seed/0x28=tick. m10.ll:52940 max_range: CastingTarget::check ×4 + Effect::range_adjust ×4 + llvm.umax. DILocalVariable !55048 threat_range(1174). open[0] 해소",
      behavior_change=False, found_by="new", kind=u"보강")
p.fix("/specs[25]/sig/params[1]/role",
      old=u"판단 주체(아군) 플레이어. info.team(+0x930)만 읽는다",
      new=u"판단 주체(아군) 플레이어. 직접 읽는 필드는 info.team(+0x930)뿐이고, is_recent_visible(player 인자)·is_enemy_well_danger(player 인자, 내부에서 info.team 재독)에 그대로 넘긴다",
      evidence=u"m10.ll:49419 call is_recent_visible(…, %1, %4) · 49443 call is_enemy_well_danger(%0, %1, %28, %30) · m03.ll:144501 `gep %1, 2352` 재독",
      behavior_change=False, found_by="new", kind=u"보강")
p.fix("/specs[25]/one_line",
      old=u"최근 시야에 잡힌 적 챔프가 (적 사거리+40000, 상한 150000) 안이면 도주 위협으로 판정",
      new=u"최근 시야에 잡힌 적 챔프(적 샘 위험구역에 선 적 제외)가 (적 사거리+40000, 상한 150000) 안이면 도주 위협으로 판정",
      evidence=u"m10.ll:49443~49444 is_enemy_well_danger 참 → phi false. one_line 이 두 번째 게이트를 누락",
      behavior_change=False, found_by="new", kind=u"보강")
p.ev("/specs[25]/knobs[2]", to=2, evidence=u"오라클 실행 확인: 같은 game_core define(is_recent_visible)을 #29 경유로 실행 — last_visible=tick-120 → 가시(5) / tick-121 → 비가시(0) / tick-119 → 5 (v15B_o1_tick1000.out o29_win 3/3 MATCH, fog 로 is_visible 전부 false)", found_by="new")
for j in range(11):
    p.ev("/specs[25]/mem[%d]" % j, to=3, evidence=u"tcx 정본 대조(tcxdict): PlayerState 0x930 info.team · OperationData 0x0 cache/0x8 context/0x10 blackboard · AbstractGameWithCache 0x0 game(&dyn 16B: data,vtable) · Entity 0x0 team@tag(TeamType Direct, Player=0)/0x8 team@Player.0/0x660 x/0x668 y; offset_of!(Entity,x/y/team) 0x660/0x668/0x0 (v15B_o1.out)", found_by="reused")

# ═════════ #26 v23_objective_setup_pressure_line ═════════
p.fix("/specs[26]/mem[9]/name",
      old=u"top_lead / mid_lead / bottom_lead ([usize;2] × 3, 16B stride)",
      new=u"top_lead/mid_lead/bottom_lead [line][team]",
      evidence=u"G20 R1: #36 mem[8] 이 같은 (AbstractGameWithCache,0x21c0) 을 `top_lead/mid_lead/bottom_lead [line][team]` 로 부른다 — 표기 통일. tcxdict 0x21c0=top_lead[0]/0x21d0=mid_lead[0]/0x21e0=bottom_lead[0]; offset_of! 동일(v15B_o1.out)",
      behavior_change=False, found_by="reused")
p.fix("/specs[26]/logic",
      old=u"[L81]   lead = data.cache(+0x0).line_lead(line, team)    // = {top|mid|bottom}_lead[team] (+0x21c0 + line*16 + team*8)",
      new=u"[L81]   lead = data.cache(+0x0).line_lead(team, line)    // = {top|mid|bottom}_lead[team] (+0x21c0 + line*16 + team*8). 시그니처 fn(&self, usize, LineType) — team 이 먼저. 소스(60자 검산) `    let lead = data.cache.line_lead(player.info.team, line);`",
      evidence=u"tcx callees[1] line_lead: fn(&AbstractGameWithCache, usize, LineType) -> usize. rmeta_srcmap objective_helpers.rs L81=61(60+LF) 검산 ±0. 옛 문면은 인자 순서가 뒤집혀 있었다",
      behavior_change=False, found_by="new")
p.fix("/specs[26]/logic",
      old=u"[L80]   ms = &data.blackboard(+0x10)[team].{Top:top(+0x0) | Mid:mid(+0x28) | Bottom:bottom(+0x50)}_minion_state   // Blackboard::minion_state 인라인 (blackboard.rs:379~382)",
      new=u"[L80]   ms = &data.blackboard(+0x10)[team].{Top:top(+0x0) | Mid:mid(+0x28) | Bottom:bottom(+0x50)}_minion_state   // Blackboard::minion_state 인라인 (blackboard.rs:379~382). 소스(76자) `    let minion_state = data.blackboard[player.info.team].minion_state(line);`",
      evidence=u"rmeta_srcmap L80=77(76+LF); `let ms =` 는 66 으로 불일치, `let minion_state =` 만 ±0",
      behavior_change=False, found_by="new", kind=u"보강")
p.fix("/specs[26]/logic",
      old=u"        //  IR 분기 순서: lead>2 → from_mid>1999 → minion_count>1, 셋 다 참일 때만 skip(블록 %52)",
      new=u"        //  IR 분기 순서: lead>2 → from_mid>1999 → minion_count>1, 셋 다 참일 때만 skip(블록 %52)\n"
          u"        //  ★소스 표기 확정(85자 검산 ±0, 후보 8조합 중 유일): `    if lead >= 3 && minion_state.from_mid >= 2000 && minion_state.minion_count >= 2 {`\n"
          u"        //    — 하나의 `&&` 식(중첩 if 면 L83=15 `      continue;`/L84=5 `    }` 와 안 맞음). IR 리터럴 2/1999/1 은 `>=3/>=2000/>=2` 를 LLVM 이 `>` 로 정규화한 것",
      evidence=u"rmeta_srcmap objective_helpers.rs L82=86(85+LF) L83=16 L84=6. `>` 3개=82 / `>=` 3개=85(유일 일치). 오라클 경계: lead 2→미제외·3→제외, from_mid 1999→미제외·2000→제외, minion_count 1→미제외·2→제외 (o26 14/14 MATCH). open[3] 「표기 불가」 판정 반전",
      behavior_change=False, found_by="new")
p.fix("/specs[26]/logic",
      old=u"[L93] return best.map(|(l, _)| l)                     // IR: best 의 LineType 바이트(-1=None) 그대로 반환",
      new=u"[L92] return best.map(|(line, _)| line)               // IR: best 의 LineType 바이트(-1=None) 그대로 반환. 소스 L92(28자 검산) `  best.map(|(line, _)| line)`, L93 = `}`. L87(62자) `    if best.is_none_or(|(_, best_score)| score < best_score) {` / L88(33자) `      best = Some((line, score));` / L86(102자) `    let score = minion_state.from_mid + lead as i64 * 10000 + minion_state.minion_count as i64 * 1000;`",
      evidence=u"rmeta_srcmap L86=103 L87=63 L88=34 L92=29 L93=2(=`}`+LF). ret !dbg !61637 = line 93 은 닫는 중괄호 줄. open[2] 「.map 표기 불가」 판정 반전(줄 길이 산술로 확정)",
      behavior_change=False, found_by="new")
for j, extra in ((4, u" (소스 표기 `lead >= 3`, 85자 줄 검산)"), (5, u" (소스 표기 `from_mid >= 2000`)"), (6, u" (소스 표기 `minion_count >= 2`)")):
    old = {4: u"제외 조건 1: lead > 2 (icmp ugt). 라인 진출 단계가 3 이상",
           5: u"제외 조건 2: minion_state.from_mid > 1999 (icmp sgt, 즉 >= 2000). 미니언 전선이 중앙에서 충분히 전진",
           6: u"제외 조건 3: minion_state.minion_count > 1 (i32 sgt). 아군 미니언 수 우위 2 이상"}[j]
    p.fix("/specs[26]/consts[%d]/meaning" % j, old=old, new=old + extra,
          evidence=u"rmeta_srcmap objective_helpers.rs L82=86 → `>=` 3개 조합만 85 검산 일치", behavior_change=False, found_by="new", kind=u"보강")
p.fix("/specs[26]/consts[8]/meaning", old=u"score 의 minion_count 가중치",
      new=u"score 의 minion_count 계수(가중치): minion_count as i64 * 1000",
      evidence=u"m15.ll:55503 `%61 = mul nsw i64 %60, 1000, !dbg !61633`(86). kind 가 `미상` 이던 것 — `가중치` 는 kind 어휘에 없어 `계수` 낱말을 넣는다. 오라클: top mc1 fm0 vs mid fm999 → Mid(1000 > 999), mid mc -5 → -5000 최소 선택",
      behavior_change=False, found_by="reused")
p.fix("/specs[26]/sig/params[0]/role", old=u"판단 주체. info.team(+0x930)만 읽는다 — blackboard·line_lead 의 팀 인덱스",
      new=u"판단 주체(IR %0). info.team(+0x930)만 읽는다 — blackboard·line_lead 의 팀 인덱스",
      evidence=u"G16 P1: define 인자 4개(ptr %0, ptr %1, ptr %2, i64 %3) > params 3개 — 슬라이스 팻포인터 승격이라 role 에 `%k` 대응 명기 필요", behavior_change=False, found_by="reused", kind=u"보강")
p.fix("/specs[26]/sig/params[1]/role", old=u"cache(+0x0)·context(+0x8)·blackboard(+0x10) 셋 다 읽는다",
      new=u"(IR %1) cache(+0x0)·context(+0x8)·blackboard(+0x10) 셋 다 읽는다",
      evidence=u"G16 P1 승격 대응 명기", behavior_change=False, found_by="reused", kind=u"보강")
p.fix("/specs[26]/sig/params[2]/role", old=u"후보 라인 슬라이스(원소 1B, LineType 태그 Top0/Mid1/Bottom2). 순서대로 순회",
      new=u"후보 라인 슬라이스(원소 1B, LineType 태그 Top0/Mid1/Bottom2). 순서대로 순회. &[LineType] 팻포인터가 SROA 로 두 레지스터로 승격: IR %2 = ptr, IR %3 = len (m15.ll:55352 `%5 = gep %2, %3` 끝포인터, `icmp eq i64 %3, 0` 빈 슬라이스)",
      evidence=u"m15.ll:55350 define (ptr %0, ptr %1, ptr %2, i64 %3) · 55352~55353", behavior_change=False, found_by="reused", kind=u"보강")
for j in range(10):
    p.ev("/specs[26]/mem[%d]" % j, to=3, evidence=u"tcx 정본 대조(tcxdict) + offset_of!(v15B_o1.out): PlayerState.info 0x0(+team 0x930 tcx) · OperationData 0x0/0x8/0x10 · Blackboard top/mid/bottom_minion_state 0x0/0x28/0x50 · BrainMinionParameter from_mid 0x10/minion_count 0x20 · AbstractGameWithCache top_lead 0x21c0/mid 0x21d0/bottom 0x21e0", found_by="reused")
for j in (1, 2, 3):
    p.ev("/specs[26]/consts[%d]" % j, to=2, evidence=u"오라클 실행 확인: lines=[Bottom]→Bottom(2) · [Mid,Bottom,Top] 동점→Mid(1) · [Top,Mid,Bottom]→Top(0) — 태그 0/1/2 ↔ top/mid/bottom_minion_state 선택 (v15B_o1.out o26 14/14 MATCH)", found_by="new")
p.ev("/specs[26]/consts[4]", to=2, evidence=u"오라클 실행 확인: top lead=2(from_mid 2000, count 2)→미제외(Top 선택) / lead=3→제외(Bottom 선택) (o26)", found_by="new")
p.ev("/specs[26]/consts[5]", to=2, evidence=u"오라클 실행 확인: from_mid 1999(lead 3, count 2)→미제외 / 2000→제외 (o26)", found_by="new")
p.ev("/specs[26]/consts[6]", to=2, evidence=u"오라클 실행 확인: minion_count 1(lead 3, from_mid 2000)→미제외 / 2→제외 (o26)", found_by="new")
p.ev("/specs[26]/consts[7]", to=2, evidence=u"오라클 실행 확인: top lead1/fm0 vs mid fm9999 → Mid(10000>9999) · vs mid fm10000 → Top(동점, 먼저) (o26)", found_by="new")
p.ev("/specs[26]/consts[8]", to=2, evidence=u"오라클 실행 확인: top mc1/fm0 vs mid fm999 → Mid(1000>999) · mid mc=-5 → -5000 으로 최소 선택 (o26)", found_by="new")
p.ev("/specs[26]/consts[9]", to=2, evidence=u"오라클 실행 확인: 세 라인 전부 제외 → None · 빈 슬라이스 → None (o26)", found_by="new")
p.ev("/specs[26]/knobs[0]", to=2, evidence=u"오라클 실행 확인: lead 2→미제외/3→제외 경계 (o26)", found_by="new")
p.ev("/specs[26]/knobs[1]", to=2, evidence=u"오라클 실행 확인: from_mid 1999/2000 경계 (o26)", found_by="new")
p.ev("/specs[26]/knobs[2]", to=2, evidence=u"오라클 실행 확인: minion_count 1/2 경계 (o26)", found_by="new")
p.ev("/specs[26]/knobs[3]", to=2, evidence=u"오라클 실행 확인: lead 1 = from_mid 10000 등가(9999→Mid, 10000→동점 Top) (o26)", found_by="new")
p.ev("/specs[26]/knobs[4]", to=2, evidence=u"오라클 실행 확인: minion_count 1 = from_mid 1000 등가(999→Mid), 음수 -5 → -5000 (o26)", found_by="new")

# ═════════ #27 nexus_under_direct_attack ═════════
p.fix("/specs[27]/knobs[1]/where", old=u"g07.ll:157005~157049", new=u"g07.ll:157039",
      evidence=G07, behavior_change=False, found_by="reused")
p.fix("/specs[27]/logic",
      old=u"       //  (_gcbc g15.ll:109546, simulation.rs:1898~) = player_champion(+0x1e0)[enemy_ix] 의 Some 만 모은 벡터",
      new=u"       //  (_gcbc g15.ll:109887~110016, simulation.rs:1896~) = player_champion(+0x1e0)[enemy_ix][0..5] 의 Some 만 모은 벡터 — ★필터 없음(죽음·Illusion·hp 검사 0건: 본문에 Entity 필드 gep 없음, 비교는 null·루프끝(40)·cap 뿐)",
      evidence=u"grep -n '^define.*AbstractGameWithCache9champions' g15.ll → 109887 (109546 은 minions 의 define — 옛 문면은 둘이 뒤바뀜). 109887~110016 icmp = `%18 == 40`(루프 끝) · `%23 == null`(Some) · `%26 == %25`(cap). open[0] 해소",
      behavior_change=False, found_by="new")
p.fix("/specs[27]/logic",
      old=u"       //  (_gcbc g15.ll:109887) = top_minions(+0x10)/mid_minions(+0x50)/bottom_minions(+0x90)[enemy_ix] 를 이어붙인 벡터",
      new=u"       //  (_gcbc g15.ll:109546~109886, simulation.rs:1853~) = top_minions(+0x10)/mid_minions(+0x50)/bottom_minions(+0x90)[enemy_ix] 세 Vec 을 이어붙인 벡터 — ★필터 없음(Entity 필드 gep 0건, cache+0x10/+0x50/+0x90 만 읽음). 캐시 Vec 자체가 죽은 미니언을 빼는지는 AbstractGameWithCache::new 소관(미탐색)",
      evidence=u"g15.ll:109546 define minions: gep %1 +16/+80/+144 만, `getelementptr … i64 (104|1616|1648|1472)` 0건. 오라클: top_minions[1] 에 nearest_enemy=Some(nexus.id) 미니언 push → true, bottom_minions 로 옮겨도 true, 아군 목록(top_minions[0])에 넣으면 false",
      behavior_change=False, found_by="new")
p.ev("/specs[27]/consts[1]", to=2, evidence=u"오라클 실행 확인: 같은 미니언을 아군(team 0) top_minions 에 넣으면 false, 적(1-team) 목록이면 true (v15B_o1.out o27)", found_by="new")
p.ev("/specs[27]/consts[2]", to=2, evidence=u"오라클 실행 확인: 적 챔프 d=240000(최근가시) → true / 240001 → false (o27, 경계 포함)", found_by="new")
p.ev("/specs[27]/consts[3]", to=2, evidence=u"오라클 실행 확인: 0x68 태그 1(Minion)+nearest=Some(nexus.id) → true / 태그 2(Tower) 같은 페이로드 → false (o27)", found_by="new")
p.ev("/specs[27]/knobs[0]", to=2, evidence=u"오라클 실행 확인: 240000 true / 240001 false (o27)", found_by="new")
p.ev("/specs[27]/knobs[1]", to=2, evidence=u"오라클 실행 확인: d=240000 이라도 last_visible=0,tick=1000(fog) → false / last_visible=tick → true; #29 경유 창 경계 120/121 (v15B_o1_tick1000.out)", found_by="new")
for j in range(16):
    p.ev("/specs[27]/mem[%d]" % j, to=3, frm=(3 if j == 13 else 4), evidence=u"tcx 정본 대조(tcxdict) + offset_of!(v15B_o1.out: nexus 0x170, player_champion 0x1e0, top/mid/bottom_minions 0x10/0x50/0x90, Entity ty 0x68·id 0x5c0·x/y) · 0x68/0x88/0x90 은 오라클에서 그 오프셋을 써서 결과가 뒤집힘(런타임 확인)", found_by="reused")

# ═════════ #28 v21_should_defer_support_target ═════════
p.fix("/specs[28]/knobs[6]/where", old=u"g07.ll:157005~ `add", new=u"g07.ll:157039 `add",
      evidence=G07, behavior_change=False, found_by="reused")
p.fix("/specs[28]/logic",
      old=u"        support_range_sq = if same_team { 14400000000 /*120000^2*/ }\n[L1092]                    else { let r = battle::max_range_cached(data, champ, target) + 25000; r*r }",
      new=u"        support_range = if same_team { 120000 /*L1090 리터럴; IR 은 제곱 14400000000 으로 접음*/ }\n[L1092]                 else { battle::max_range_cached(data, champ, target) + 25000 }\n"
          u"        //  ★소스 표기 확정(줄 길이 산술 ±0): L1089(52자) `  let support_range = if target.team == champ.team {` / L1090(10자) `    120000` / L1091 `  } else {` /\n"
          u"        //    L1092(49자) `    max_range_cached(data, champ, target) + 25000` / L1093 `  };` / L1094(65자) `  if target.distance_sq(champ) <= support_range * support_range {`\n"
          u"        //    ⟹ open[1] 의 (a): support_range 는 제곱 전 값이고 L1094 에서 `support_range * support_range`(IR `mul i64 %20, %20, !dbg !54701`=1094)",
      evidence=u"rmeta_srcmap game_ai fight_model.rs 1077~1121: L1089=53 L1090=11 L1091=11 L1092=50 L1093=5 L1094=66 (각 +LF). DILocalVariable !54687 support_range(1089) · IR 48989 `%21 = mul i64 %20, %20, !dbg !54701`(1094) · phi %27 [14400000000] 은 120000² 접힘. open[1] 「표기 불가」 판정 반전",
      behavior_change=False, found_by="new")
p.fix("/specs[28]/logic",
      old=u"[L1094] if Entity::distance_sq(target, champ) <= support_range_sq { return false }   // icmp ugt 의 반대 → 사거리 안이면 보류 안 함",
      new=u"[L1094] if target.distance_sq(champ) <= support_range * support_range { return false }   // icmp ugt 의 반대 → 사거리 안이면 보류 안 함",
      evidence=u"L1094 65자 검산(위 항목)", behavior_change=False, found_by="new")
p.fix("/specs[28]/logic",
      old=u"[L1106] if local_outnumbered {\n            if can_near_enemies != 0 || die_tick <= tps*3 { return true }\n[L1110] } else {\n            if can_near_enemies > 1 && die_tick <= tps*4 /*shl 2*/ { return true }\n        }",
      new=u"[L1106] if local_outnumbered && (can_near_enemies > 0 || die_tick <= tps*3) { return true }   // 73자 검산 `  if local_outnumbered && (can_near_enemies > 0 || die_tick <= tps * 3) {` (`!= 0` 표기는 74 로 불일치 → `> 0`; IR 은 icmp ne 0)\n"
          u"[L1110] if can_near_enemies > 1 && die_tick <= tps*4 /*shl 2*/ { return true }              // ★else 없는 독립 if(50자 검산 `  if can_near_enemies > 1 && die_tick <= tps * 4 {`; `} else if` 는 57 로 불일치).\n"
          u"        //  열세 경로가 여기 오면 can_near_enemies==0 이 확정이라 항상 거짓 → LLVM 이 else 구조(블록 %67 은 !local_outnumbered 에서만)로 접음. 외연 동일",
      evidence=u"rmeta_srcmap L1106=74 L1107=17 L1108=4 L1109=1 L1110=51 L1111=17 L1112=4. IR m10.ll:49067 `br i1 %66, label %72, label %67`(1100) · 블록 %67(1110) `icmp ugt i64 %7, 1 / shl i64 %54, 2 / icmp ule` · 블록 %72(1106) `icmp ne i64 %7, 0 / mul i64 %54, 3`. 재구현 결과는 옛 문면과 동일(외연 같음)",
      behavior_change=False, found_by="new")
p.fix("/specs[28]/logic",
      old=u"[L1118] if target.team != champ.team {                                     // TeamType ne 인라인(cmp.rs:264)\n[L1119]     return !data.blackboard(+0x10)[1 - player.info.team(+0x930)].is_recent_visible(data.cache.game, player, target)\n            //  적 target 이 최근 가시(직접 가시 or 120틱 내 목격)가 아니면 보류\n        }\n[L1120] return false",
      new=u"[L1118] return target.team != champ.team &&                                // TeamType ne 인라인(cmp.rs:264). 소스는 꼬리식 한 개: L1118(30자) `  target.team != champ.team &&`\n[L1119]        !data.blackboard(+0x10)[1 - player.info.team(+0x930)].is_recent_visible(data.cache.game, player, target)   // L1119(93자) `    !data.blackboard[1 - player.info.team].is_recent_visible(data.cache.game, player, target)`\n            //  적 target 이 최근 가시(직접 가시 or 120틱 내 목격)가 아니면 보류; 같은 팀이면 false\n[L1120] }   // 함수 끝(`}` 1자). 옛 문면의 `if … { return … } return false` 는 외연 동일한 재서술이었다",
      evidence=u"rmeta_srcmap L1118=31 L1119=94 L1120=2 L1121=1. `if target.team != champ.team {` 는 32 로 불일치이고 L1120 뒤에 `  }`/`  false` 줄이 없다 → 꼬리식. IR 49128 `%102 = xor i1 %101, true` · phi %105 [false,%82],[false,%89]",
      behavior_change=False, found_by="new")
p.fix("/specs[28]/logic",
      old=u"[L1099] hp_ratio = champ.hp(+0x670) * 100 / max(champ.stat_cached.hp(+0x628), 1)",
      new=u"[L1099] hp_ratio = champ.hp(+0x670) * 100 / champ.stat_cached.hp(+0x628).max(1)   // Ord::max 메서드(DISubprogram max<usize> self/other). 줄 길이 61 vs 후보 `  let hp_ratio = champ.hp * 100 / champ.stat_cached.hp.max(1);` 62 — 잔차 1 미해결(#29 L12 도 같은 식이 잔차 1)",
      evidence=u"m10.ll !18411 DISubprogram max<usize> (cmp.rs:1034, self/other) · rmeta_srcmap L1099=62. 잔차 1 은 표기 미확정으로 남긴다", behavior_change=False, found_by="new", kind=u"보강")
p.fix("/specs[28]/knobs[1]/where", old=u"fight_model.rs:1089 (IR `phi … 14400000000`)",
      new=u"fight_model.rs:1090 (소스 리터럴 `    120000` 10자 줄; IR m10.ll `phi … 14400000000` = 120000² 접힘, L1089 블록 종결)",
      evidence=u"rmeta_srcmap L1090=11(10+LF) = `    120000` 유일 후보; DILocalVariable support_range 는 1089 선언", behavior_change=False, found_by="new")
p.fix("/specs[28]/consts[2]/src_line", old=1089, new=1090,
      evidence=u"소스 리터럴 120000 은 L1090(10자 줄 `    120000`), L1089 는 `let support_range = if … {` 선언 줄. IR 에는 phi 상수라 !dbg 없음(G12 강한 후보 0)",
      behavior_change=False, found_by="new")
p.fix("/specs[28]/consts[2]/meaning", old=u"120000^2 — 같은 팀(또는 둘 다 Neutral) target 에 대한 지원 사거리 제곱(상수 접힘: 120000 리터럴은 본문에 없음, 3.75칸)",
      new=u"120000^2 — 같은 팀(또는 둘 다 Neutral) target 에 대한 지원 사거리 제곱(상수 접힘: 소스 L1090 `120000` 을 L1094 `support_range * support_range` 와 함께 접음, 3.75칸)",
      evidence=u"줄 길이 산술 L1090/L1094", behavior_change=False, found_by="new", kind=u"보강")
p.fix("/specs[28]/consts[10]/meaning", old=u"enemy_ix = 1 - player.info.team",
      new=u"적팀 인덱스 enemy_ix = 1 - player.info.team (blackboard 첨자)",
      evidence=u"m10.ll:49111 `%87 = sub i64 1, %86` → 49118 gep blackboard[%87]. kind 가 `임계` 로 나온 것은 낱말 부재 — #25/#29 의 같은 상수는 `인덱스`", behavior_change=False, found_by="reused")
p.fix("/specs[28]/consts[8]/meaning", old=u"국지 열세일 때 can_near_enemies != 0 (1명이라도) 이면 보류",
      new=u"국지 열세일 때 can_near_enemies > 0 (소스 표기; IR icmp ne 0) — 접근 가능한 적이 1명이라도 있으면 보류",
      evidence=u"L1106 73자 검산은 `> 0` 만 일치(`!= 0` 은 74)", behavior_change=False, found_by="new", kind=u"보강")
p.ev("/specs[28]/knobs[6]", to=2, evidence=u"오라클 실행 확인: 같은 game_core define(is_recent_visible) — #29 경유 last_visible=tick-120 가시 / tick-121 비가시 (v15B_o1_tick1000.out)", found_by="new")
for j in range(18):
    p.ev("/specs[28]/mem[%d]" % j, to=3, evidence=u"tcx 정본 대조(tcxdict): Entity 0x0 team@tag/0x8 team@Player.0/0x660 x/0x668 y/0x670 hp/0x628 stat_cached.hp · OperationData 0x0/0x8/0x10 · GameContext 0x8 setting · GameSetting 0x12f8 tick_per_second · PlayerState 0x930 info.team · AbstractGameWithCache 0x0 game(&dyn 16B); offset_of!(GameSetting,tick_per_second)=0x12f8·(Entity,hp)=0x670 (v15B_o1.out)", found_by="reused")

# ═════════ #29 v23_recent_visible_enemies_near_point ═════════
p.fix("/specs[29]/knobs[2]/where", old=u"g07.ll:157005~ `add", new=u"g07.ll:157039 `add",
      evidence=G07, behavior_change=False, found_by="reused")
p.fix("/specs[29]/logic",
      old=u"[L35]       && Entity::distance_sq(e, (x, y)) <= range_sq // abs_diff(e.x,x)^2 + abs_diff(e.y,y)^2 (icmp ugt 면 탈락)",
      new=u"[L35]       && utils::distance_sq(e.x, e.y, x, y) <= range * range // ★game_core::utils::distance_sq(x1,y1,x2,y2) 4인자 자유함수가 클로저 L35 에서 직접 인라인(DISubprogram !61866 utils.rs:6, inlinedAt !61876=L35) — Entity::distance_sq 아님(중간 스코프 없음). 인자 순서는 IR `icmp ult %37(e.x), %2(x)` → abs_diff(e.x, x). abs_diff(e.x,x)^2 + abs_diff(e.y,y)^2 (icmp ugt 면 탈락). 소스 L35(55자) `        distance_sq(e.x, e.y, x, y) <= range * range &&`(들여쓰기 8 = 계속행)",
      evidence=u"m15.ll !61866 = DISubprogram distance_sq linkageName _RNvNtCs97f5S1uJLkH_9game_core5utils11distance_sq, !61882 line 7 inlinedAt !61876(35), !61885 abs_diff 3147. rmeta_srcmap objective_helpers.rs L35=56(55+LF). open[3] 「튜플 vs 두 인자 표기 불가」 판정 반전: 4인자 자유함수. callees[1] Entity::distance_sq 는 이 함수의 호출자가 아니다",
      behavior_change=False, found_by="new")
p.fix("/specs[29]/logic",
      old=u"[L33] range_sq = range * range                            // IR %20 (루프 밖 1회)",
      new=u"[L33] range_sq = range * range                            // IR %20 (루프 밖 1회) — 소스엔 변수 없음: L35 의 `range * range` 를 LLVM 이 루프 밖으로 끌어올린 것(L33(17자) = `    .filter(|e| {`)",
      evidence=u"rmeta_srcmap L33=18 → `    .filter(|e| {` 17 검산; %20 에 !dbg 없음", behavior_change=False, found_by="new", kind=u"보강")
p.fix("/specs[29]/logic",
      old=u"[L32] enemy_ix = 1 - player.info.team(+0x930)           // >=2 → panic_bounds_check(len=2)",
      new=u"[L32] enemy_ix = 1 - player.info.team(+0x930)           // >=2 → panic_bounds_check(len=2). 소스엔 변수 없음 — 함수 본문 = L32~L38 한 식 체인: L32(49자) `  data.cache.iter_champions(1 - player.info.team)` / L33 `    .filter(|e| {` / L34(37자) `      v23_healthy(e, min_hp_ratio) &&` / L35 / L36(91자) `        data.blackboard[1 - player.info.team].is_recent_visible(data.cache.game, player, e)` / L37 `    })` / L38(12자) `    .count()`",
      evidence=u"rmeta_srcmap objective_helpers.rs L31=149(148 시그니처 검산) L32=50 L33=18 L34=38 L35=56 L36=92 L37=7 L38=13 L39=2 — 전 줄 ±0", behavior_change=False, found_by="new", kind=u"보강")
p.ev("/specs[29]/consts[2]", to=2, evidence=u"오라클 실행 확인: maxhp 1000, hp 500(=50%)·490(=49%) → min_hp 50 에서 50% 포함·49% 제외, min 51 에서 50% 제외 (v15B_o1.out o29 8/8 MATCH; ratio == min 포함 = `>=`)", found_by="new")
p.ev("/specs[29]/knobs[0]", to=2, evidence=u"오라클 실행 확인: min_hp_ratio 0/50/51/100 경계 (o29)", found_by="new")
p.ev("/specs[29]/knobs[1]", to=2, evidence=u"오라클 실행 확인: range 199999/200000/200001 에 d=200000 적이 제외/포함/포함, range 0 에 d=0 포함 (o29, 경계 포함)", found_by="new")
p.ev("/specs[29]/knobs[2]", to=2, evidence=u"오라클 실행 확인: last_visible=tick-120 → 5 / tick-121 → 0 / tick-119 → 5 (fog 로 is_visible 전부 false, v15B_o1_tick1000.out o29_win)", found_by="new")
for j in range(10):
    p.ev("/specs[29]/mem[%d]" % j, to=3, evidence=u"tcx 정본 대조(tcxdict) + offset_of!(v15B_o1.out player_champion 0x1e0, Entity x/y/hp/stat_cached) · 0x660/0x668/0x670/0x628 은 오라클에서 그 오프셋에 써서 결과가 바뀜(런타임 확인)", found_by="reused")

# ═════════ 도시에 오류 ═════════
p.brief_error(u"§4 G13 4건(25/27/28/29 knobs `g07.ll:157005`)은 전부 실오류가 맞다 — 그러나 §4 머리말은 「게이트 후보를 믿지 마라」만 강조해 오탐처럼 읽힌다. 실제로 157005 는 define 줄이고 명령은 157039 다(하나의 공통 오기가 4명세에 복제됨 = G20 류 cross-spec 복제 오류인데 G20 은 knobs.where 를 안 본다)")
p.brief_error(u"§4 G20 [27] `Entity 0x90` 항목은 #27 쪽이 tcxdict 표기(`ty@Minion.info.nearest_enemy@Some.0`)와 일치하고 어긋난 쪽은 #34(`ty.Minion.info.nearest_enemy.Some.0`)다 — 배치 B 에 배정됐지만 고칠 자리는 #34(다른 배치). 게이트 보고가 「어느 쪽이 정본과 다른가」를 안 적어 담당이 헛돈다. 제안: sharedchk 가 tcxdict 표기와 일치하는 쪽을 표시")
p.brief_error(u"§1 표의 #25·#28 `in:game_ai` 경고는 맞지만 「상위 pub 래퍼나 형제 복제본을 노려라」는 이 둘엔 해당 재료가 없다(형제 0, 호출자 18/3곳은 전부 큰 플랜 함수). 대신 **줄 길이 산술**(IR_TOOLKIT §8)이 두 함수의 소스 표기를 전 줄 ±0 으로 복원했다 — §1 이 이 방법을 가리키지 않는다")
p.brief_error(u"§4-b `consts.kind` 어휘 안내대로 `상한` 을 써도 #25 consts[4](150000, llvm.umin 클램프)는 `인덱스` 로 파생된다 — kindchk.SUPPORT[임계]=('CMP_ORD',) 뿐이고 MINMAX 는 인덱스/계수/오프셋가감만 지지하기 때문. 클램프 상·하한은 임계다: SUPPORT[임계] 에 'MINMAX' 추가 제안(도구 결함, meaning 으로는 못 고침)")
p.brief_error(u"`mkpatch.Patch.ev()` 가 2단 배열 경로(`/specs[25]/knobs[2]`)를 `ev 상향은 배열 원소만` 으로 거부한다 — `m.group(3) is None` 검사가 틀렸다(3단 경로의 중간 필드 그룹; idx 는 group(4)). 도시에가 「참조구현을 쓰라」고 하는데 참조구현이 ev_up 을 한 건도 못 만든다. 이 배치는 같은 검증을 하는 대체 함수로 우회했다(oracle/mk15B.py 머리)")
p.brief_error(u"§0 `dossierfresh` 는 FRESH 였고 §5 스키마도 applypatch 계약과 일치했다(이번 라운드 지시 오류 0건 확인 항목). 단 §5 예시의 `/specs[25]/consts[3]/src_line 293→310` 은 이 배치의 #25 와 무관한 가상 예시라 헷갈린다 — 예시 인덱스는 배치 밖 번호로")
out = p.save()
print(out)
