---

### `105` passive_plan — 판단 없이도 하는 기본 행동 플랜(패시브) 선택 — 스틸/오브젝트/에고웨이브/버프창/라인 리드 순으로 BigPlan 과 발원코드(u8) 반환

| 항목 | 값 |
|---|---|
| id | `handler__LegacyPlanHandler_passive_plan` |
| 심볼 | `_RNvMNtNtCshdEBA0ozCnw_7game_ai11plan_legacy7handlerNtB2_17LegacyPlanHandler12passive_plan` |
| 소스 | `game-ai\src\plan_legacy\handler.rs:1855` |
| IR | `m13.ll` 6495~9987행 |
| 경로·가시성 | `game_ai::plan_legacy::handler::LegacyPlanHandler::passive_plan` · **in:game_ai::plan_legacy::handler** |
| 계층 | 플랜 핸들러 |
| exe | `e46bc0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r11` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> (game_ai::plan_legacy::types::BigPlan, u8)
```

<details><summary>인자 7개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | &mut (BigPlan, u8) 392B | +0 BigPlan 384B, +384 u8 = mf 발원 코드(호출자 handle_chat_inner 는 이 u8 을 버린다) | 4 |
| 1 | 1 | self | &LegacyPlanHandler(6168B) | ★가변성 판정 = `&self`. 근거 ①tcx sig `fn(&LegacyPlanHandler, ..)` ②본문 내 %1 파생 포인터 store 0건, 유일한 쓰기가 `atomicrmw add` @0x500(team_plan.eo_cover_picks, V54Counter=AtomicUsize) ③readonly 가 빠진 이유가 바로 그 원자 쓰기(내부 가변성) — _docs 「passive_plan이 &self라 내부 가변 카운터(V54Counter 패턴)」와 일치 | 3 |
| 2 | 2 | version | usize | 이 함수 본문에선 분기 없음 — serpen/epic_passive_plan·with_best·v2_obj_restore_safe·v3_repair_done 에 전달만 | 4 |
| 3 | 3 | rnd | &mut StdRng(320B) | :1919 gen_range(0..1000) 1회 직접 사용 | 4 |
| 4 | 4 | player | &PlayerState(2528B) |  | 4 |
| 5 | 5 | data | &OperationData(24B) |  | 4 |
| 6 | 6 | debug | &mut DebugFrameData(224B) | 본문 직접 접근 0건 — 하위 호출 전달만 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
// handler.rs:1858~1872 ① 정글러 스틸
is_jungle = (position == Jungle)
if is_jungle {
  match self.team_plan.steal_action {                                                   // :1861 (0x510/0x511)
    Some(Lurk(target)):  Serpen && serpen_exists(tutorial∈{None0,MidBottom5,Line7,Total8}) → return (SerpenHuntAndPoke{focus:false, vision_only:true, flee:false}, 1)   // :1871~1872
                         Epic && morgard_exists(tutorial==0 || tutorial>=7)             → return (EpicHuntAndPoke{focus:false, vision_only:true, flee:false}, 1)     // :1865~1866
    Some(Commit(target)): Serpen && serpen_exists → return (SerpenHuntAndPoke{focus_serpen_only:true, vision_only:false}, 1)   // :1868~1869
                          Epic && morgard_exists  → return (EpicHuntAndPoke{focus_epic_only:true, vision_only:false}, 1)      // :1862~1863
    _ (None / None(0) / exists 실패) → 계속
  }
}
// :1878~1892 ② 팀 목표 직결 플랜
match self.team_plan.objective {
  Nexus(line)      → return (AttackNexus{team: 1-my_team, line}, 2)           // :1879
  Defense          → return (DefenseNexus{team: my_team}, 3)                  // :1883
  DefenseLine(line)→ return (PassiveLine::new(line), 4)                       // :1886~1887
  Dive(line)       → return (PassiveLine::new(line), 5)                       // :1891~1892
  _ → 계속
}
// :1897~1920 ③ 에고웨이브(v2)
match self.v2_egowave {                                                                  // 0x1815
  2 → return (PassiveLine::new(self.v2_egowave_line), 6)                               // :1903
  0 → if !is_jungle {                                                                    // :1905
        roam = roaming_ratio; ego = ego_ratio; my_line = Top→Top, Mid→Mid, else Bottom  // :1906~1908
        line = fallback_line(ctx, my_line)                                             // :1913
        ms = blackboard[team].minion_state(line)                                       // :1914
        if ms.from_mid < -2000 && ms.minion_count < -2 {                               // :1915 (우리 웨이브가 크게 밀림)
          base = 500 - roam; if base >= 1 { p = base*ego/500; if rnd.gen_range(0..1000) < p { return (PassiveLine::new(line), 6) } }   // :1916~1920
        }
      }
  _ → 계속
}
// :1928~1939 ④ PressTower 목표
if objective == PressTower(line) {
  if let Some((x,y)) = self.v3_assign_anchor(player, data) {                              // :1929
    if is_near_line(ctx,x,y,line) || (roaming_ratio > 399 && side(line,x,y) /* Top: height-y>=x · Mid: is_near_mid_line · Bottom: height-y<=x */) { return (PassiveLine::new(line), 7) }   // :1931~:1939
  }
}
// :1946~1960 ⑤ ComebackPick 목표
if objective == ComebackPick(line) {
  if is_jungle { return (LineGanker{chats:[], setup_limit:0, wait_limit: tick+tps*15, line, phase:Setup}, 8) }   // :1948~1950
  if let Some((x,y)) = v3_assign_anchor(..) { if is_near_line(..) || side(line,x,y) { return (PassiveLine::new(line), 8) } }   // :1953~1960 (roam 조건 없음)
}
// :1967~1970 ⑥ 웨이브 우선 정리
if team_plan.should_delay_object_setup_for_wave_priority(player,data) {
  if let Some(l) = team_plan.should_player_clear_wave_priority_line(player,data) { return (PassiveLine::new(l), 9) } else → ⑨(:2040) 로 점프(⑦⑧ 건너뜀)   // :1969
} else if game.as_moba().map_or(false,|m| m.epic_minion_buff_time[team]!=0) {            // :1968 우리 에픽 버프 창
  if let Some(l) = should_player_clear_wave_priority_line(..) { return (PassiveLine::new(l), 9) }
}
// :1974~1995 ⑦ 정글러: 오브젝트 셋업 중 손상된 캠프 마무리
if is_jungle && objective ∈ {Morgard,Serpen} && objective.phase == Setup(1) && self.plan is PassiveJungle(pj) {   // :1974~1981
  me = player_champion[team][Jungle]; if None → ⑧
  camp_live = as_moba().map(|m| m.jungle_runner.get_jungle_live_list(pj.jungle, pj.team==0)).unwrap_or_default()   // :1983
  if !camp_live.is_empty() {                                                            // :1984
    (cx,cy) = map.camp_pos(pj.jungle, pj.team==0)                                       // :1985
    if distance_sq(me,(cx,cy)) < 14400000001 && camp_live.iter().any(|id| get_entity_by_id(id).map_or(false,|e| e.hp < e.stat_cached.hp)) { return (PassiveJungle(pj.clone()), 10) }   // :1986~1991
  }
}
// :2000~2034 ⑧ 오브젝트 전용 패시브(serpen/epic)
if player_champion[team][position].is_some() {                                           // :2000
  if objective is Serpen{phase} {                                                       // :2001
    anchor = v3_depart_anchor(v3_armed, team, position, ctx)                            // :2002
    match serpen_passive_plan(version,rnd,player,data,&team_plan,phase,anchor,debug) {
      Some(p) → if v2_armed && (p.tag&30)==14 /*SerpenHuntAndPoke|Battle*/ && !v2_obj_restore_safe(..) { drop(p) → ⑨ } else { return (p, 11) }   // :2007~2012
      None → if v2_armed && v2_obj_part == Some(0x50|phase) && serpen_exists && v2_obj_restore_safe(..) { return (SerpenHuntAndPoke{false,false,false}, 11) }   // :2014~2017
    }
  }
  if objective is Morgard{phase} {                                                      // :2021
    anchor = v3_depart_anchor(..); match epic_passive_plan(..) {                          // :2022
      Some(p) → if v2_armed && (p.tag&30)==12 /*EpicHuntAndPoke|Battle*/ && !v2_obj_restore_safe(..) { drop → ⑨ } else { return (p, 12) }   // :2024~2029
      None → if v2_armed && v2_obj_part == Some(0x60|phase) && morgard_exists && v2_obj_restore_safe(..) { return (EpicHuntAndPoke{false,false,false}, 12) }   // :2031~2034
    }
  }
}
// :2040~2107 ⑨ 에픽옵스 커버 라인 (objective None 일 때만)
if objective.is_none() {
  enemy_buff = as_moba().map_or(false,|m| m.epic_minion_buff_time[1-team]!=0)          // :2041
  recent_giveup = epic_giveup_tick.is_some_and(|t| tick.saturating_sub(t) <= tps*10)    // :2042~2043
  our_buff = v3_epicops_armed && as_moba().map_or(false,|m| m.epic_minion_buff_time[team]!=0)   // :2048~2049
  if (enemy_buff || recent_giveup || our_buff) && player_champion[team][position].is_some() {   // :2051~2052
    best=None; best_score=0
    for line in valid_lines(tutorial) /* {0,7,8}:[T,M,B] 5:[M,B] 4:[M] 2:[T] {1,3}:[B] 6:[] */ {   // :2057
      ms = blackboard[team].minion_state(line); if ms.from_mid > -3001 || ms.minion_count > -4 → continue   // :2058~2060
      if count(아군 i≠me: player_champion[team][i] 살아있고 is_near_line(ctx, e.pos, line)) != 0 → continue   // :2065~2071 (closure#4)
      if enemy_buff || recent_giveup || !our_buff {                                      // (루프 언스위치) 일반 모드
        (ax,ay) = v3_assign_anchor(..).unwrap_or(me.pos)                                // :2087
        ok = is_near_line(ctx,ax,ay,line) || side(line,ax,ay)                            // :2088~2093
      } else {                                                                            // our_buff 전용 모드
        ok = (line 의 포지션 == my position)                                             // :2077~2082
      }
      if ok { score = -ms.minion_count*1000 - ms.from_mid; if score > best_score { best_score=score; best=line } }   // :2096~2097
    }
    if let Some(l) = best {                                                              // :2103
      if !(enemy_buff || recent_giveup || !our_buff) { team_plan.eo_cover_picks.fetch_add(1) }   // :2104~2105 (우리 버프 창 커버 픽 계측)
      return (PassiveLine::new(l), 13)                                                   // :2107
    }
  }
}
// :2113~2126 ⑩ 초반 라인 페이즈 기본
if is_line_phase(ctx,tick) /* tutorial∉{0,5,7,8} || tick < epic.first_spawn_tick - tps*30 */ {   // :2113
  match position { Top→(PassiveLine(Top),14) · Mid→(PassiveLine(Mid),14) · Bottom→(PassiveLine(Bottom),14) · Support→(PassiveLine(Bottom),14)   // :2116~2119,:2126
                   Jungle→ pj = PassiveJunglePlan::with_best(..); pj.last_lead_action_tick = self.last_jungle_lead_action_tick; return (PassiveJungle(pj), 27) }   // :2121~2124
}
// :2128~2186 ⑪ 우리 에픽 버프 창(중반 이후)
if let Some(m) = as_moba() {
  if m.epic_minion_buff_time[team] != 0 {                                                // :2128
    if v3_epicops_armed {                                                                // :2132
      if objective != Repair { if let Some(f) = v3_epic_formation(rnd,player,data) { return (PassiveLine::new(f.line), 15) } }   // :2133~2134
    }
    if objective ∈ {PressEpic(l), SplitEpic(l)} { return (PassiveLine::new(l), 15) }     // :2137~2139
    if objective == Repair && !v3_repair_done(version,team,position,cache) { return (ActiveRecall, 16) }   // :2142~2150
    cand = [Top if line_exists(Top)/*tutorial∈{0,2,7,8}*/ && enemy(1-team) top_tower|top_tower2 살아있음, Mid if line_exists(Mid)/*{0,4,5,7,8}*/ && mid_tower*, Bottom if line_exists(Bottom)/*{0,1,3,5,7,8}*/ && bottom_tower*]   // :2154~2170
    if !cand.is_empty() && player_champion[team][position].is_some() {                   // :2173~2174
      l = cand.iter().min_by_key(|line| distance_sq(me.pos, map.region_pos[map.line_region(line, team, lead[team])])).unwrap()   // :2175~2181
      return (PassiveLine::new(l), 17)                                                   // :2183
    }
  }
}
// :2189~2260 ⑫ 라인 리드 기반 최종 폴백
top_lead/mid_lead/bottom_lead = cache.*_lead[team]                                       // :2189~2191
top_cnt/mid_cnt/bottom_cnt = count(i in 0..5: blackboard[team].in_big_line(i, Top/Mid/Bottom))   // :2192~2194
A_x = x_lead < 3 ; B_x = line_exists(x) && x_lead < 3 && x_cnt == 0 && player_champion[team][x 포지션].is_none()   // :2195~2197 (x∈{Top,Mid,Bottom}; 포지션 Top=0,Mid=2,Bottom=3)
match position {                                                                        // :2199
  Top:    A_top → (Line Top,18) ; else B_mid → (Mid,19) ; else B_bottom && champion[Mid] 없음 → (Bottom,19) ; else (Top,18)   // :2202~2209
  Jungle: if champion[Top] 없음 && champion[Mid] 없음 && (살아있는 아군 캠프 수 < 3 /*aux 정글 closure*/) && champion[Bottom] 없음 {   // :2217~2221
            B_mid → (Mid,19) ; else B_top → (Top,19) ; else B_bottom → (Bottom,19) ; else (Mid,19)    // :2222~2229
          } else { pj = with_best(..); pj.last_lead_action_tick = self.last_jungle_lead_action_tick; (PassiveJungle(pj), 27) }   // :2232~2234
  Mid:    A_mid → (Mid,18) ; else B_top → (Top,19) ; else B_bottom → (Bottom,19) ; else (Mid,18)   // :2238~2245
  Bottom: A_bottom → (Bottom,18) ; else B_top && champion[Mid] 없음 → (Top,19) ; else B_mid && champion[Top] 없음 → (Mid,19) ; else (Bottom,18)   // :2249~2256
  Support: (PassiveLine(fallback_line(tutorial, Mid)), 18)                             // :2260 (tutorial {0,4,5,7,8}→Mid, {1,3}→Bottom, 2→Top, 6→Bottom)
}

// 발원 코드 표(ret.1): 1 스틸 포크 · 2 AttackNexus · 3 DefenseNexus · 4 DefenseLine · 5 Dive 라인 · 6 에고웨이브 · 7 PressTower 앵커 · 8 ComebackPick · 9 웨이브우선 라인 · 10 손상캠프 유지 · 11 serpen 패시브 · 12 epic 패시브 · 13 에픽옵스 커버 · 14 초반 기본 라인 · 15 버프창 라인 · 16 Repair 귀환 · 17 버프창 최근접 라인 · 18 리드 폴백(자기/기본 라인) · 19 리드 폴백(다른 라인) · 27 with_best 정글
```

**`mem` 메모리 접근 55건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | LegacyPlanHandler | 0x510 | team_plan.steal_action@tag | r | :1861 None(-1)/None(0)/Lurk(1)/Commit(2) | 4 | OK |  |
| 1 | LegacyPlanHandler | 0x511 | team_plan.steal_action@Some.0@Lurk/Commit.0 (StealTarget) | r | 0=Epic,1=Serpen | 4 | OK |  |
| 2 | LegacyPlanHandler | 0x517 | team_plan.objective | r | MainObjective 태그: 0 Morgard,1 Serpen,2 Defense,3 DefenseLine,4 Nexus,5 PressEpic,6 SplitEpic,7 Repair,8 Gank,9 Dive,10 PressTower,11 ComebackPick,-1 None | 4 | OK |  |
| 3 | LegacyPlanHandler | 0x518 | team_plan.objective@Some.0 페이로드1(line/phase) | r |  | 4 | OK |  |
| 4 | LegacyPlanHandler | 0x5e8 | plan | r | :1981 PassiveJungle(7) 판정 | 4 | OK |  |
| 5 | LegacyPlanHandler | 0x5f0 | plan@PassiveJungle.0 (104B) | r | :1991 PassiveJunglePlan::clone 원본 | 4 | OK |  |
| 6 | LegacyPlanHandler | 0x638 | plan@PassiveJungle.0.team | r | :1983 team==0 → get_jungle_live_list bool 인자 | 4 | OK |  |
| 7 | LegacyPlanHandler | 0x650 | plan@PassiveJungle.0.jungle | r | :1983/:1985 JungleType | 4 | OK |  |
| 8 | LegacyPlanHandler | 0x148 | team_plan.obj_spawn.epic_giveup_tick@tag | r | :2042 | 4 | OK |  |
| 9 | LegacyPlanHandler | 0x150 | team_plan.obj_spawn.epic_giveup_tick@Some.0 | r | :2043 tick.saturating_sub(t) <= tps*10 | 4 | OK |  |
| 10 | LegacyPlanHandler | 0x500 | team_plan.eo_cover_picks (AtomicUsize) | r | :2105 atomicrmw add 1 monotonic — 읽고 쓰는 유일한 self 필드 | 4 | OK |  |
| 11 | LegacyPlanHandler | 0x14b8 | last_jungle_lead_action_tick | r | :2123/:2233 → PassiveJunglePlan.last_lead_action_tick | 4 | OK |  |
| 12 | LegacyPlanHandler | 0x1800 | v2_obj_part@tag | r | :2014/:2031 Option<u8> is_some | 4 | OK |  |
| 13 | LegacyPlanHandler | 0x1801 | v2_obj_part@Some.0 | r | == (0x50\|phase) Serpen / (0x60\|phase) Morgard 비교 | 4 | OK |  |
| 14 | LegacyPlanHandler | 0x1807 | v2_armed | r | :2007/:2014/:2024/:2031 | 4 | OK |  |
| 15 | LegacyPlanHandler | 0x1808 | v3_armed | r | :2002/:2022 v3_depart_anchor 인자(i8) | 4 | OK |  |
| 16 | LegacyPlanHandler | 0x180a | v3_epicops_armed | r | :2048/:2132 | 4 | OK |  |
| 17 | LegacyPlanHandler | 0x180c | v2_egowave_line | r | :1903 | 4 | OK |  |
| 18 | LegacyPlanHandler | 0x1815 | v2_egowave | r | :1897 0/2/기타 3분기 | 4 | OK |  |
| 19 | PlayerState | 0x9c0 | info.position@tag | r | Top0 Jungle1 Mid2 Bottom3 Support4 | 4 | OK |  |
| 20 | PlayerState | 0x930 | info.team | r |  | 4 | OK |  |
| 21 | PlayerState | 0x180 | info.parameter | r | &AthleteParameter → roaming_ratio/ego_ratio | 4 | OK |  |
| 22 | OperationData | 0x0 | cache | r |  | 4 | OK |  |
| 23 | OperationData | 0x8 | context | r |  | 4 | OK |  |
| 24 | OperationData | 0x10 | blackboard | r | &[Blackboard;2] | 4 | OK |  |
| 25 | AbstractGameWithCache | 0x0 | game.data_ptr | r |  | 4 | OK |  |
| 26 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r | +0x28 tick, +0x40 get_game_mode(as_moba), +0x1f0 get_entity_by_id | 4 | OK |  |
| 27 | AbstractGameWithCache | 0x1e0 | player_champion | r | 생존 판정 다수. +0x1e0+team*40+pos*8 | 4 | OK |  |
| 28 | AbstractGameWithCache | 0x180 | top_tower[enemy]/top_tower2[enemy] (0x180/0x190) | r | :2154 line_exists&&tower 있음 | 4 | 오귀속(사전은 다른 필드를 준다) |  |
| 29 | AbstractGameWithCache | 0x1a0 | mid_tower/mid_tower2 (0x1a0/0x1b0) | r | :2155 | 4 | 오귀속(사전은 다른 필드를 준다) |  |
| 30 | AbstractGameWithCache | 0x1c0 | bottom_tower/bottom_tower2 (0x1c0/0x1d0) | r | :2156 | 4 | 오귀속(사전은 다른 필드를 준다) |  |
| 31 | AbstractGameWithCache | 0x21c0 | top_lead[team] | r | :2189 (aux min_by_key 에서도 line_region 인자) | 4 | OK |  |
| 32 | AbstractGameWithCache | 0x21d0 | mid_lead[team] | r | :2190 | 4 | OK |  |
| 33 | AbstractGameWithCache | 0x21e0 | bottom_lead[team] | r | :2191 | 4 | OK |  |
| 34 | GameContext | 0x0 | pool | r | :2159 bumpalo Vec<LineType>::new_in | 4 | OK |  |
| 35 | GameContext | 0x8 | setting | r |  | 4 | OK |  |
| 36 | GameContext | 0x20 | map | r | :1985 camp_pos / aux line_region | 4 | OK |  |
| 37 | GameContext | 0x38 | tutorial | r | rule_scope 인라인(morgard_exists/serpen_exists/valid_lines/line_exists/fallback_line/is_line_phase) | 4 | OK |  |
| 38 | GameSetting | 0x12f8 | tick_per_second | r |  | 4 | OK |  |
| 39 | GameSetting | 0x12c0 | height | r | is_top_side: height - y >= x / is_bottom_side: height - y <= x | 4 | OK |  |
| 40 | GameSetting | 0x8a8 | epic_jungle.first_spawn_tick | r | :2113 is_line_phase: tick < first_spawn_tick - tps*30 | 4 | OK |  |
| 41 | MobaMode | 0x18 | jungle_runner | r | :1983 get_jungle_live_list(&self+0x18, ..) | 4 | OK |  |
| 42 | MobaMode | 0x240 | epic_minion_buff_time[team] (0x240/0x248) | r | !=0 이면 그 팀 에픽 버프 창 | 4 | OK |  |
| 43 | Blackboard | 0x0 | top/mid/bottom_minion_state (0x0/0x28/0x50) | r | minion_state(line) 인라인 | 4 | 오귀속(사전은 다른 필드를 준다) |  |
| 44 | Blackboard | 0x10 | *_minion_state.from_mid | r | < -2000 (:1915) / <= -3001 (:2060) 밀린 웨이브 | 4 | OK |  |
| 45 | Blackboard | 0x20 | *_minion_state.minion_count | r | < -2 (:1915) / <= -4 (:2060) | 4 | OK |  |
| 46 | Entity | 0x660 | x | r |  | 4 | OK |  |
| 47 | Entity | 0x668 | y | r |  | 4 | OK |  |
| 48 | Entity | 0x670 | hp | r | :1987 캠프 몬스터 hp < max | 4 | OK |  |
| 49 | Entity | 0x628 | stat_cached.hp | r |  | 4 | OK |  |
| 50 | MapDef | 0x6ba0 | region_pos[27] (x,y) | r | aux min_by_key | 4 | 오귀속(사전은 다른 필드를 준다) |  |
| 51 | LegacyPlanHandler | 0x500 | team_plan.eo_cover_picks | w | :2105 — 우리 버프 창 커버 픽(밀린웨이브 정리 배정) 채택 계측. ★self 에 대한 유일한 쓰기(내부 가변성) | 4 | OK | atomicrmw add 1 (monotonic) |
| 52 | (sret) | 0x0 | ret.0 BigPlan@tag | w |  | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) | 12 EpicHuntAndPoke / 14 SerpenHuntAndPoke / 16 AttackNexus / 17 DefenseNexus / 3 PassiveLine / 10 LineGanker / 7 PassiveJungle / 8 ActiveRecall / serpen·epic_passive_plan 결과 memcpy 384 |
| 53 | (sret) | 0x8 | ret.0 페이로드 | w |  | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) | PassiveLinePlan::new(line) 인라인(:1887 등 20곳): v46_flee_entry None(0 @+8), chats/v46_committers/v46_flee_threats 빈 Vec(cap0 ptr8 len0 @+0x20/+0x38/+0x50), v46_pending 0(136B @+0x68), bool 6개 0(@+0x118..), line @+0x11e ; LineGankerPlan(:1949): chats 빈, setup_limit 0, wait_limit tick+tps*15 @+0x28, line @+0x30, phase Setup(7) @+0x31 ; *HuntAndPoke(:1863/1866/1869/1872/2017/2034): v46_flee_threats 빈 Vec @+8, +0x20 focus_only, +0x21 vision_only, +0x22 v46_flee ; AttackNexus(:1879): team=1-my @+8, line @+0x10 ; DefenseNexus(:1883): team=my @+8 ; PassiveJungle: 104B memcpy @+8 |
| 54 | (sret) | 0x180 | ret.1 발원 코드 u8 | w | _docs 「코드는 simulator MF_SRC_NAMES와 1:1」 | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) | 1(:1863,:1866,:1869,:1872) 2(:1879) 3(:1883) 4(:1887) 5(:1892) 6(:1903,:1920) 7(:1939) 8(:1949,:1960) 9(:1970) 10(:1991) 11(:2012,:2017) 12(:2029,:2034) 13(:2107) 14(:2117~2119,:2126) 15(:2134,:2139) 16(:2150) 17(:2183) 18(:2203,:2209,:2239,:2245,:2250,:2256,:2260) 19(:2205,:2207,:2223,:2225,:2227,:2229,:2241,:2243,:2252,:2254) 27(:2124,:2234) |

**`consts` 상수 22건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | -2000 | 1915 | 임계 | 에고웨이브 라인 조건: minion_state(fallback_line).from_mid < -2000 (우리 웨이브가 밀림) | 4 |
| 1 | -2 | 1915 | 임계 | && minion_count < -2 | 4 |
| 2 | 500 | 1916 | 계수 | base = 500 - roaming_ratio ; p = base*ego_ratio/500 (‰). base<1 이면 불참 | 4 |
| 3 | 1000 | 1919 | 미상 | gen_range(0..1000) < p 이면 PassiveLine(fallback_line) 채택(발원 6). :2096 점수 = -minion_count*1000 - from_mid 에도 사용 | 4 |
| 4 | 399 | 1937 | 임계 | PressTower 목표 라인: anchor 가 라인 근처가 아니어도 roaming_ratio > 399 && 라인 쪽(side) 이면 PassiveLine(line) | 4 |
| 5 | 15 | 1950 | 계수 | ComebackPick 정글러 LineGanker.wait_limit = tick + tps*15 | 4 |
| 6 | 14400000001 | 1986 | 임계 | 120000^2 + 1 — 내 위치와 캠프 좌표 distance_sq < 이 값(≤120000) 이면 손상된 캠프 계속(PassiveJungle 유지, 발원 10) | 4 |
| 7 | 10 | 2043 | 태그 | epic_giveup_tick 이 tps*10 이내면 '최근 포기' (에픽옵스 커버 라인 진입 조건) | 4 |
| 8 | -3001 | 2060 | 임계 | 커버 후보 라인: from_mid > -3001 이면 제외(즉 ≤ -3001 필요) | 4 |
| 9 | -4 | 2060 | 임계 | && minion_count ≤ -4 (> -4 면 제외) | 4 |
| 10 | 30 | 2113 | 계수 | is_line_phase: tick < epic first_spawn_tick - tps*30 (또는 tutorial∉{None,MidBottom,Line,Total}) 이면 초반 라인 페이즈 기본 플랜(발원 14/27) | 4 |
| 11 | 3 | 2195 | 임계 | top/mid/bottom_lead[team] < 3 이면 그 라인 '리드 부족' → 라인 배정 후보. ObjectPhase::Hunt(3)·MainObjective::DefenseLine(3)·Position::Bottom(3) 도 3 (본문의 `shl nuw nsw i64 %len, 3` 은 Vec<usize> 바이트 stride(×8)이지 상수 3 이 아님 — QC 경고 사유) | 4 |
| 12 | 80 | 2014 | 미상 | v2_obj_part == Some(0x50 \| phase) — Serpen 복원 코드(0x50 + ObjectPhase). `or disjoint i8 %phase, 80` | 4 |
| 13 | 96 | 2031 | 미상 | v2_obj_part == Some(0x60 \| phase) — Morgard 복원 코드 | 4 |
| 14 | 12 | 1863 | 태그 | BigPlan::EpicHuntAndPoke 메모리태그(idx10+2). :2025 `(tag&30)==12` = EpicHuntAndPoke\|EpicHuntAndBattle 판정 | 4 |
| 15 | 14 | 1869 | 태그 | BigPlan::SerpenHuntAndPoke 태그. :2008 `(tag&30)==14` = SerpenHuntAndPoke\|SerpenHuntAndBattle. 발원코드 14(:2117) 와 값 동일 | 4 |
| 16 | 16 | 1879 | 태그 | BigPlan::AttackNexus 태그 | 4 |
| 17 | 17 | 1883 | 태그 | BigPlan::DefenseNexus 태그 | 4 |
| 18 | 7 | 1981 | 센티널 | BigPlan::PassiveJungle 태그(:1981,:1991,:2124,:2234) / LineGankerPhase::Setup 니치태그(:1949) / MainObjective::Repair(:2132,:2142) | 4 |
| 19 | 8 | 2150 | 태그 | BigPlan::ActiveRecall 태그 — Repair 미완이면 귀환 | 4 |
| 20 | 5 | 2137 | 태그 | objective 태그-5 <u 2 ⟺ PressEpic(5)\|SplitEpic(6) → 그 라인 PassiveLine(발원 15). JungleType::Serpen(5)·TutorialType::MidBottom(5) 도 5 | 4 |
| 21 | 27 | 2124 | 산출값 | 발원 코드 27 = PassiveJungle::with_best (aux line_region 의 27 region 상한과 값 동일) | 4 |

**`knobs` 조정점 11건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 에고웨이브 발동 웨이브 임계 | handler.rs:1915 | from_mid < -2000 && minion_count < -2 | 완화하면 라이너가 덜 밀린 웨이브에도 fallback 라인으로 감 | 4 | 기존 |
| 1 | 에고웨이브 참가 확률 | handler.rs:1916~1919 | (500-roam)*ego/500 ‰ | 500 을 올리면 로밍 성향이 높아도 참가 | 4 | 기존 |
| 2 | PressTower 라인 측면 허용 로밍 | handler.rs:1937 | roaming_ratio > 399 | 내리면 로밍 낮은 선수도 앵커가 라인 쪽이면 PressTower 라인 배정 | 4 | 기존 |
| 3 | ComebackPick 정글 대기 | handler.rs:1950 | tps*15 | 올리면 LineGanker 대기 연장 | 4 | 기존 |
| 4 | 손상 캠프 마무리 거리 | handler.rs:1986 | 120000^2+1 | 올리면 더 먼 캠프도 마무리하러 감(오브젝트 셋업 지연) | 4 | 기존 |
| 5 | 최근 에픽 포기 창 | handler.rs:2043 | tps*10 | 올리면 포기 후 더 오래 커버 라인 모드 | 4 | 기존 |
| 6 | 커버 후보 라인 웨이브 임계 | handler.rs:2060 | from_mid ≤ -3001 && minion_count ≤ -4 | 완화하면 더 많은 라인이 커버 후보 | 4 | 기존 |
| 7 | 커버 라인 점수 | handler.rs:2096 | -minion_count*1000 - from_mid | 가중 1000 을 바꾸면 미니언 수 vs 밀림 거리 우선순위 변화 | 4 | 기존 |
| 8 | 초반 라인 페이즈 종료 | handler.rs:2113 | tick < first_spawn_tick - tps*30 | 30 을 올리면 더 일찍 중반 로직(버프창/리드 폴백)으로 | 4 | 기존 |
| 9 | 라인 리드 부족 기준 | handler.rs:2195~2197 | lead < 3 | 올리면 자기 라인을 더 쉽게 유지(18), 내리면 다른 라인 이동(19) 증가 | 4 | 기존 |
| 10 | 정글러 라인 전환 캠프 수 | handler.rs:2221 | 살아있는 캠프 < 3 | 올리면 정글러가 캠프가 남아도 라인으로 감 | 4 | 기존 |

<details><summary>`callees` 피호출자 36건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | as_moba | game_core::GameMode::<'a>::as_moba | pub | fn(game_core::GameMode<'a/#0>) -> std::option::Option<&'a/#0 game_core::MobaMode> | game-core\src\simulation\game.rs:230 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 1 | camp_pos | game_core::MapDef::camp_pos | pub | fn(&game_core::MapDef, game_core::JungleType, bool) -> (u64, u64) | game-core\src\simulation\map_def.rs:207 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 2 | distance_sq | game_core::utils::distance_sq | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:6 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 3 | distance_sq | game_core::Entity::distance_sq | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2157 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 4 | ego_ratio | game_core::AthleteParameter::ego_ratio | pub | fn(&game_core::AthleteParameter) -> usize | game-core\src\simulation\state\player.rs:439 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 5 | epic_passive_plan | game_ai::plan_legacy::old::epic_passive_plan | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, game_ai::plan_legacy::team_plan::ObjectPhase, &game_ai::plan_legacy::team_plan::TeamPlan, std::option::Option<(u64, u64)>, &mut game_core::DebugFrameData) -> std::option::Option<game_ai::plan_legacy::types::BigPlan> | game-ai\src\plan_legacy\old\epic.rs:317 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 6 | fallback_line | game_ai::plan_legacy::rule_scope::fallback_line | pub | fn(&game_core::GameContext, game_core::LineType) -> game_core::LineType | game-ai\src\plan_legacy\rule_scope.rs:28 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 7 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 8 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 9 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 10 | get_jungle_live_list | game_core::JungleRunner::get_jungle_live_list | pub | fn(&game_core::JungleRunner, game_core::JungleType, bool) -> std::vec::Vec<usize, std::alloc::Global> | game-core\src\simulation\entity\jungle.rs:743 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 11 | in_big_line | game_core::Blackboard::in_big_line | pub | fn(&game_core::Blackboard, usize, game_core::LineType) -> bool | game-core\src\simulation\game\blackboard.rs:137 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 12 | is_empty | game_core::PatchSetting::is_empty | pub | fn(&game_core::PatchSetting) -> bool | game-core\src\patch_engine.rs:853 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 13 | is_empty | game_core::ModAiRegistry::is_empty | pub | fn(&game_core::ModAiRegistry) -> bool | game-core\src\mod_ai.rs:15 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 14 | is_empty | game_core::PublicChampionEvidence::is_empty | pub | fn(&game_core::PublicChampionEvidence) -> bool | game-core\src\banpick\public_evidence.rs:96 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 15 | is_line_phase | game_core::GameSetting::is_line_phase | pub | fn(&game_core::GameSetting, usize) -> bool | game-core\src\setting.rs:702 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 16 | is_line_phase | game_core::GameContext::<'a, 'b>::is_line_phase | pub | fn(&game_core::GameContext<'a/#0, 'b/#1>, usize) -> bool | game-core\src\simulation\game\runner.rs:397 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 17 | is_near_line | game_core::is_near_line | pub | fn(&game_core::GameContext, u64, u64, game_core::LineType) -> bool | game-core\src\simulation\map_regions.rs:139 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 18 | is_near_mid_line | game_core::is_near_mid_line | pub | fn(&game_core::GameContext, u64, u64) -> bool | game-core\src\simulation\map_regions.rs:127 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 19 | line_exists | game_ai::plan_legacy::rule_scope::line_exists | pub | fn(&game_core::GameContext, game_core::LineType) -> bool | game-ai\src\plan_legacy\rule_scope.rs:24 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 20 | line_region | game_core::line_region | pub | fn(game_core::LineType, usize, usize) -> usize | game-core\src\simulation\path_finder.rs:237 | True | True | 3 | IR 호출 심볼 일치(토큰 경계) |
| 21 | line_region | game_core::MapDef::line_region | pub | fn(&game_core::MapDef, game_core::LineType, usize, usize) -> usize | game-core\src\simulation\map_def.rs:239 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 22 | minion_state | game_core::Blackboard::minion_state | pub | fn(&game_core::Blackboard, game_core::LineType) -> &game_core::BrainMinionParameter | game-core\src\simulation\game\blackboard.rs:378 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 23 | morgard_exists | game_ai::plan_legacy::rule_scope::morgard_exists | pub | fn(&game_core::GameContext) -> bool | game-ai\src\plan_legacy\rule_scope.rs:45 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 24 | roaming_ratio | game_core::AthleteParameter::roaming_ratio | pub | fn(&game_core::AthleteParameter) -> usize | game-core\src\simulation\state\player.rs:425 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 25 | serpen_exists | game_ai::plan_legacy::rule_scope::serpen_exists | pub | fn(&game_core::GameContext) -> bool | game-ai\src\plan_legacy\rule_scope.rs:49 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 26 | serpen_passive_plan | game_ai::plan_legacy::old::serpen_passive_plan | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan, game_ai::plan_legacy::team_plan::ObjectPhase, std::option::Option<(u64, u64)>, &mut game_core::DebugFrameData) -> std::option::Option<game_ai::plan_legacy::types::BigPlan> | game-ai\src\plan_legacy\old\serpen.rs:416 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 27 | should_delay_object_setup_for_wave_priority | game_ai::plan_legacy::team_plan::TeamPlan::should_delay_object_setup_for_wave_priority | pub | fn(&game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData) -> bool | game-ai\src\plan_legacy\team_plan.rs:578 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 28 | should_player_clear_wave_priority_line | game_ai::plan_legacy::team_plan::TeamPlan::should_player_clear_wave_priority_line | pub | fn(&game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PlayerState, &game_core::OperationData) -> std::option::Option<game_core::LineType> | game-ai\src\plan_legacy\team_plan.rs:561 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 29 | v2_obj_restore_safe | game_ai::plan_legacy::handler::LegacyPlanHandler::v2_obj_restore_safe | in:game_ai::plan_legacy::handler | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> bool | game-ai\src\plan_legacy\handler.rs:501 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 30 | v3_assign_anchor | game_ai::plan_legacy::handler::LegacyPlanHandler::v3_assign_anchor | in:game_ai::plan_legacy::handler | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, &game_core::PlayerState, &game_core::OperationData) -> std::option::Option<(u64, u64)> | game-ai\src\plan_legacy\handler.rs:1773 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 31 | v3_depart_anchor | game_ai::plan_legacy::handler::LegacyPlanHandler::v3_depart_anchor | in:game_ai::plan_legacy::handler | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, &game_core::PlayerState, &game_core::OperationData) -> std::option::Option<(u64, u64)> | game-ai\src\plan_legacy\handler.rs:1806 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 32 | v3_epic_formation | game_ai::plan_legacy::old::v3_epic_formation | pub | fn(&mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> std::option::Option<game_ai::plan_legacy::old::V3EpicFormation> | game-ai\src\plan_legacy\old\epic.rs:774 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 33 | v3_repair_done | game_ai::plan_legacy::handler::LegacyPlanHandler::v3_repair_done | in:game_ai | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bool | game-ai\src\plan_legacy\handler.rs:1847 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 34 | valid_lines | game_ai::plan_legacy::rule_scope::valid_lines | pub | fn(&game_core::GameContext) -> &[game_core::LineType] | game-ai\src\plan_legacy\rule_scope.rs:13 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 35 | with_best | game_ai::plan_legacy::old::PassiveJunglePlan::with_best | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> game_ai::plan_legacy::old::PassiveJunglePlan | game-ai\src\plan_legacy\old\passive_jungle.rs:51 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
</details>

⚠**미매칭 9개**: `drop  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `else`, `enemy`, `fetch_add`, `gen_range`, `map_or`, `push  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 6개는 **전부 다른 함수**라 싣지 않는다`, `side`, `unwrap_or_default`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 13곳** (m13.ll:10259, m13.ll:12296, m13.ll:21505, m13.ll:21607, m13.ll:24126, m13.ll:24426, m13.ll:31694, m13.ll:31870, m13.ll:31956, m13.ll:32138, m13.ll:32369, m13.ll:32488, m13.ll:32613) · **형제 41개** (LegacyPlanHandler)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::plan_legacy::handler::LegacyPlanHandler as std::clone::Clone>::clone | pub | game-ai\src\plan_legacy\handler.rs:13 | True | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler) -> game_ai::plan_legacy::handler::LegacyPlanHandler |
| 1 | <game_ai::plan_legacy::handler::LegacyPlanHandler as std::fmt::Debug>::fmt | pub | game-ai\src\plan_legacy\handler.rs:13 | True | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 2 | game_ai::plan_legacy::handler::LegacyPlanHandler::new | pub | game-ai\src\plan_legacy\handler.rs:228 | False | fn(usize, &mut rand::rngs::std::StdRng, usize, game_core::Position) -> game_ai::plan_legacy::handler::LegacyPlanHandler |
| 3 | game_ai::plan_legacy::handler::LegacyPlanHandler::r2_fight_protected | in:game_ai | game-ai\src\plan_legacy\handler.rs:362 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &game_core::PlayerState, &game_core::OperationData) -> bool |
| 4 | game_ai::plan_legacy::handler::LegacyPlanHandler::ff_note_battle_swap | in:game_ai | game-ai\src\plan_legacy\handler.rs:400 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, u8, usize) |
| 5 | game_ai::plan_legacy::handler::LegacyPlanHandler::mf_note_swap | in:game_ai | game-ai\src\plan_legacy\handler.rs:409 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, u8, usize) |
| 6 | game_ai::plan_legacy::handler::LegacyPlanHandler::v3_fall_back_to_passive | pub | game-ai\src\plan_legacy\handler.rs:416 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 7 | game_ai::plan_legacy::handler::LegacyPlanHandler::eo_cover_picks | pub | game-ai\src\plan_legacy\handler.rs:440 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler) -> usize |
| 8 | game_ai::plan_legacy::handler::LegacyPlanHandler::eo_serpen_punish_issues | pub | game-ai\src\plan_legacy\handler.rs:445 | True | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler) -> usize |
| 9 | game_ai::plan_legacy::handler::LegacyPlanHandler::subplan_is_recall | pub | game-ai\src\plan_legacy\handler.rs:450 | True | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler) -> bool |
| 10 | game_ai::plan_legacy::handler::LegacyPlanHandler::team_objective_code | pub | game-ai\src\plan_legacy\handler.rs:456 | True | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler) -> u8 |
| 11 | game_ai::plan_legacy::handler::LegacyPlanHandler::update_v2_egowave | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:473 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) |
| 12 | game_ai::plan_legacy::handler::LegacyPlanHandler::v2_obj_restore_safe | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:501 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> bool |
| 13 | game_ai::plan_legacy::handler::LegacyPlanHandler::v2_apply_assign_commit | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:518 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, &mut u8, &mut game_core::DebugFrameData) |
| 14 | game_ai::plan_legacy::handler::LegacyPlanHandler::sanitize_rule_scope | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:571 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, &game_core::OperationData) |
| 15 | game_ai::plan_legacy::handler::LegacyPlanHandler::take_misunderstood_received_chat | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:588 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, game_core::Position, game_core::Chat) -> bool |
| 16 | game_ai::plan_legacy::handler::LegacyPlanHandler::enter_line_backfight_support | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:598 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> bool |
| 17 | game_ai::plan_legacy::handler::LegacyPlanHandler::update_on_dead | pub | game-ai\src\plan_legacy\handler.rs:635 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 18 | game_ai::plan_legacy::handler::LegacyPlanHandler::update | pub | game-ai\src\plan_legacy\handler.rs:685 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData, bool) |
| 19 | game_ai::plan_legacy::handler::LegacyPlanHandler::determine_transition_reason | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:1675 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, &str) -> game_core::PlanTransitionReason |
| 20 | game_ai::plan_legacy::handler::LegacyPlanHandler::force_plan_update | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:1709 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData, bool) |
| 21 | game_ai::plan_legacy::handler::LegacyPlanHandler::v3_assign_anchor | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:1773 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, &game_core::PlayerState, &game_core::OperationData) -> std::option::Option<(u64, u64)> |
| 22 | game_ai::plan_legacy::handler::LegacyPlanHandler::v3_depart_anchor | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:1806 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, &game_core::PlayerState, &game_core::OperationData) -> std::option::Option<(u64, u64)> |
| 23 | game_ai::plan_legacy::handler::LegacyPlanHandler::v3_plan_dest | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:1824 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::types::BigPlan) -> std::option::Option<(u64, u64)> |
| 24 | game_ai::plan_legacy::handler::LegacyPlanHandler::v3_repair_done | in:game_ai | game-ai\src\plan_legacy\handler.rs:1847 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bool |
| 25 | game_ai::plan_legacy::handler::LegacyPlanHandler::passive_plan | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:1855 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> (game_ai::plan_legacy::types::BigPlan, u8) |
| 26 | game_ai::plan_legacy::handler::auction::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::get_small_action | pub | game-ai\src\plan_legacy\handler\auction.rs:8 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &std::vec::Vec<(usize, game_core::SmallAction), std::alloc::Global>, &mut game_core::DebugFrameData) -> (game_ai::ScoreParameter, i64, game_ai::SmallActionPlay) |
| 27 | game_ai::plan_legacy::handler::chat::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::handle_chat | pub | game-ai\src\plan_legacy\handler\chat.rs:8 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, game_core::Position, game_core::Chat, bool, &mut game_core::DebugFrameData) |
| 28 | game_ai::plan_legacy::handler::chat::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::handle_chat_inner | in:game_ai | game-ai\src\plan_legacy\handler\chat.rs:41 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, game_core::Position, game_core::Chat, bool, &mut game_core::DebugFrameData) |
| 29 | game_ai::plan_legacy::handler::dive_episode::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::v50_track_dive_episode | in:game_ai | game-ai\src\plan_legacy\handler\dive_episode.rs:35 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &game_core::PlayerState, &game_core::OperationData) |
| 30 | game_ai::plan_legacy::handler::dive_episode::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::v50_fold_dive_episode | in:game_ai | game-ai\src\plan_legacy\handler\dive_episode.rs:125 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, usize, bool, u8) |
| 31 | game_ai::plan_legacy::handler::engage::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::v2_response_retreat_stance | in:game_ai | game-ai\src\plan_legacy\handler\engage.rs:13 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> game_ai::plan_legacy::old::BattleSubPlanGoal |
| 32 | game_ai::plan_legacy::handler::engage::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::try_engage | in:game_ai | game-ai\src\plan_legacy\handler\engage.rs:40 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, usize, &mut game_core::DebugFrameData) -> std::option::Option<game_ai::plan_legacy::old::BattlePlan> |
| 33 | game_ai::plan_legacy::handler::engage::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::try_engage_dive | in:game_ai | game-ai\src\plan_legacy\handler\engage.rs:109 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, usize, std::option::Option<game_core::TowerType>, &mut game_core::DebugFrameData) -> std::option::Option<game_ai::plan_legacy::old::BattlePlan> |
| 34 | game_ai::plan_legacy::handler::engage::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::handle_solokill | in:game_ai | game-ai\src\plan_legacy\handler\engage.rs:136 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 35 | game_ai::plan_legacy::handler::engage::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::handle_interact_battle | in:game_ai | game-ai\src\plan_legacy\handler\engage.rs:233 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 36 | game_ai::plan_legacy::handler::modes::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::update_single_lane | in:game_ai | game-ai\src\plan_legacy\handler\modes.rs:13 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 37 | game_ai::plan_legacy::handler::modes::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::update_deathmatch | in:game_ai | game-ai\src\plan_legacy\handler\modes.rs:56 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 38 | game_ai::plan_legacy::handler::modes::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::single_lane_initiate | in:game_ai | game-ai\src\plan_legacy\handler\modes.rs:196 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 39 | game_ai::plan_legacy::handler::modes::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::single_try_engage | in:game_ai | game-ai\src\plan_legacy\handler\modes.rs:241 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, usize, &mut game_core::DebugFrameData) -> std::option::Option<game_ai::plan_legacy::old::SinglePlanBattle> |
| 40 | game_ai::plan_legacy::handler::modes::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::single_handle_solokill | in:game_ai | game-ai\src\plan_legacy\handler\modes.rs:261 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |

**`open` 6건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | :1969 should_player_clear_wave_priority_line 이 None 일 때 should_delay 경로(%393)는 ⑨(:2040) 로, our_buff 경로(%396)는 ⑦(:1974) 로 간다 — 소스의 if/else 구조는 추정(IR 분기 목표만 확정) | 4 |  |
| 1 | 미탐색 | Option<BigPlan> 의 None 이 태그 -1 로 표현되는 근거(tcx 니치 값)는 미확인 — IR `icmp eq i64 %tag, -1` 관측만 | 3 |  |
| 2 | 미탐색 | get_jungle_live_list 의 bool 인자 = (PassiveJungle.team == 0) — 의미(블루사이드 여부)는 추정 | 5 |  |
| 3 | 미탐색 | closure#4 의 blackboard 인덱스는 handle_chat_inner 와 달리 [team] — 확인됨. 본문 :1914 도 [team] | 4 |  |
| 4 | 미탐색 | exe 0xe46bc0(11,615B) 와의 디스어셈 대조는 하지 않았다(Ghidra 미사용) | 4 |  |
| 5 | 미탐색 | C3 경고가 뜨는 오프셋이 있으면 memset/phi 접힘(예: PassiveLinePlan 필드 0x20~0x118 은 memset 16/136B 로 접힘) | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

