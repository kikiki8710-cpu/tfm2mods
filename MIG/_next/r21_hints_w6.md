# r21 티어1 심층 — 웨이브 6 (2) · 0.5.8 명세 logic 전문 + exe 힌트

요구: 0.5.8 명세(아래 logic) 를 기준선으로 0.6.0 디컴을 **줄 단위로 대조**해 「어느 줄이 어떻게 바뀌었나 / 새 분기·조건·상수·필드·콜리」 를 명세 형식(줄번호 참조 · 등가 Rust)으로 낸다. version≥3 분기는 「v3 경로」 로 별도 표기(값 미확정 · v2/v3 둘 다 적는다). 대형 함수는 먼저 콜리 지도·블록 지도(0.5.8 ↔ 0.6.0 매핑)를 만들고 신규 구역을 특정한 뒤 신규 구역만 정독한다.

## `e46bc0` → `d38180` passive_plan  (11615→20085B · Δ+8470)
- 힌트: passive_plan(11.6→20KB · d38180 · LPH::update 13곳 호출 · wave_priority_clearer_position 4곳(exclude_jungler 0/LPH+0x24d5/*rsi/1) · fb8cc0→fb8da0 ambient · d56ac0·fafe50 호출)
- exe 정렬: 명령 2146→3843 · 정렬 1070 · 잔여 구조 10 · 분기 10 · 콜리 주의: 31a01a3→381e0b3 미지 ; 31a01a3→efc410 미지 ; d40b20→eda830 미지 ; d40b20→fafe50 콜리 변경?(J0.80·+2261B) ; d65620→1011d20 변경 ; de03d0→f88880 불일치(mig060 는 f18bf0) ; de92d0→d55830 불일치(mig060 는 fb5ff0) ; dea420→3821813 불일치(mig060 는 fb74d0)
- 0.5.8 명세 #105 `handler__LegacyPlanHandler_passive_plan` src game-ai\src\plan_legacy\handler.rs:1855 · one_line: 판단 없이도 하는 기본 행동 플랜(패시브) 선택 — 스틸/오브젝트/에고웨이브/버프창/라인 리드 순으로 BigPlan 과 발원코드(u8) 반환
- 0.5.8 params: (sret):&mut (BigPlan, u8) 392B(%0 (noalias writeonly, dereferenceable 392) · +0 BigPlan 384) · self:&LegacyPlanHandler(6168B)(%1 (속성 없음: noalias/readonly 둘 다 없음) · ★가변성 판정 = `&self`. 근거 ) · version:usize(%2 · 이 함수 본문에선 분기 없음 — serpen/epic_passive_plan·with_best·v2) · rnd:&mut StdRng(320B)(%3 (noalias, readonly 없음) · :1919 gen_range(0..1000) 1회 직접 사) · player:&PlayerState(2528B)(%4 (readonly)) · data:&OperationData(24B)(%5 (readonly)) · debug:&mut DebugFrameData(224B)(%6 (noalias, readonly 없음) · 본문 직접 접근 0건 — 하위 호출 전달만)
- 0.5.8 logic 전문:
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
      l = cand.iter().min_by_key(|line| distance_sq(me.pos, map.region_centers[map.line_region(line, team, lead[team])])).unwrap()   // :2175~2181
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

## `e65b10` → `d6c2b0` LegacyPlanHandler::get_small_action  (19492→23419B · Δ+3927)
- 힌트: LegacyPlanHandler::get_small_action(19.5→23.4KB · d6c2b0)
- exe 정렬: 명령 3945→4743 · 정렬 3584 · 잔여 구조 10 · 분기 10 · 콜리 주의: 31a01a3→381e0b3 미지 ; 31a01a3→381ebcf 미지 ; 31a01a3→d33800 미지 ; 340e0→340e0 미지(pdata 밖 thunk) ; 6d4d0→6ed50 미지(pdata 밖 thunk) ; c8da90→ebf5b0 미지(+407B) ; ca5010→e12520 미지(+0B) ; ca6700→e13c10 미지(+66B)
- 0.5.8 명세 #205 `auction__LegacyPlanHandler__get_small_action` src game-ai\src\plan_legacy\handler\auction.rs:8 · one_line: SmallActionPlay 실행기 — ScoreParameter 를 만들어 self 에 반사하고(positioning_score·wave_snapshot·회두홀드 래치), v2+ 는 생존 절대규칙(RunAway 99999)/글로벌 궁 지시(Ult 99999)를 선반환, 아니면 sub_plan.action_candidates 를 pre_action 병합 입력검사로 거른 뒤(데스매치만) 후보마다 SubPlan::score 를 judge_noise_ratio[SmallAction 종류] 로 스케일해 (점수,액션) 목록을 만들고(배치 E 끝) → 이후 ignore_action 제외·최고점 선택·동률 rnd.choose·RunAway 폴백·디버그 로그를 거쳐 (parameter, score, action) 을 반환한다(배치 F·G).
- 0.5.8 params: (sret):&mut (ScoreParameter, i64, S(반환 슬롯. define 속성 = `dead_on_unwind noalias noundef writable ) · self:&mut LegacyPlanHandler (6168(`noalias noundef align 8 dereferenceable(6168)` — readonly 없) · version:usize (i64 %2)(AI 버전 게이트. 이 범위: L29 `version > 1`(=v2+) 하나. 스택 %97 에 저장해 &v) · rnd:&mut StdRng (320B, align 16)(`noalias noundef align 16 dereferenceable(320)` — readonly 없) · player:&PlayerState (2528B)(`readonly captures(address, read_provenance)` — 읽기만. info.te) · data:&OperationData (24B)(`readonly`. cache(+0)·context(+8)·blackboard(+0x10) · (배치 F)) · pre_action:&SmallActionPlay (184B)(`readonly`. 이 범위에서는 필터 클로저(L108)에서 clone 원본으로만 읽음 · (배치 F) F) · ignore_action:&Vec<(usize, game_core::Smal(`readonly`. **이 범위(0~201)에서는 안 읽음** — 배치 F(without_ignore_ac) · debug:&mut DebugFrameData (224B)(`noalias noundef align 8 dereferenceable(224)` — readonly 없음)
- 0.5.8 logic 전문:
```
// auction.rs:0~201 (배치 E)
// 표기: L<n> = auction.rs 루트 줄 · (m13.ll:NNNNN) = 원문 IR 줄 · vt[0xNN] = &dyn AbstractGame vtable 슬롯 · 간접호출 없음(이 범위의 sub_plan 호출은 JT 호스트 SubPlan::* 직접 call)

L9  _t_csp = ProfTimer::start(33)            // prof::ENABLED(atomic i8)==0 이면 nanos=-1 (비활성) (45832~45848)
L10 parameter: ScoreParameter = calculate_score_parameter(version, rnd, player, data, debug)   // sret 5384B, rnd·debug 는 콜리 define 이 readnone (45850)
L11 self.sub_plan.calculate_score_parameter_value(version, rnd, player, data, &mut parameter)   // (&mut self.sub_plan 0x768) (45861)
L12 self.positioning_score(0x990) = parameter.positioning_score(0x9f0)   // memcpy 2760B (45873)
L13 drop(_t_csp)   // nanos!=-1 → PHASE_NANOS[33]+=elapsed_ns, PHASE_CALLS[33]+=1 (45878~45930)
L16 _t_ws = ProfTimer::start(35)
L17 prediction_depth = player.info.parameter(0x180).last_hit_prediction_depth()
L18 source_quality  = player.info.parameter.last_hit_source_quality()
L19 parameter.wave_snapshot = Some(build_minion_wave_snapshot(player, data, prediction_depth, source_quality))   // 태그 1 @+0, 2320B @+8 (45980~45982)
L20 drop(_t_ws)
L22 team = player.info.team(0x930) (bounds<2, 46046) ; champ = data.cache.player_champion[team][player.info.position(0x9c0) as usize].unwrap()   // 0x1e0 + team*40 + pos*8 (46051~46066)

L29 if version > 1 {                                                     // (46081) v1 이하는 L56 으로 직행
      if game.get_game_mode()(vt[0x40]) 태그 != 2 /*DeathMatch*/ {     // (46094~46100) 데스매치면 L30~38 건너뛰고 L50 으로
L30     base_exempt = champ.stat_buff_cached.undying(0x488)
L31        || nexus_final_stand(player, data)
L32        || data.cache.nexus[team].is_some_and(|n| champ.distance(n) < 180001)   // (46117~46134)
L33     sh: SoloHuntObs = solo_hunt_obs(game.data, game.vtable, team, champ)     // sret 16B
L34     scene_now = if base_exempt || sh.ally_ctx > 1 { false }
                    else if sh.al120 + 1 < sh.en150 && sh.chasers != 0 { sh.aa_safe }   // SoloHuntObs::is_target_scene 인라인 (46146~46176)
                    else { false }
L35     parameter.v3_turnback_hold(0x1500) = scene_now || self.v3_turnback_scene_prev(0x1809)   // 직전 경매 래치 (46178~46183)
L36     self.v3_turnback_scene_prev = scene_now
L37     if data.context.debug(0x3b) {
L38       debug.add_log(data, player, format!("TBHOLD-EVAL T{} {:?} hold={} now={} ex={} al={} ctx={} en={} ch={} aa={}", team, position, parameter.v3_turnback_hold, scene_now, base_exempt, sh.al120, sh.ally_ctx, sh.en150, sh.chasers, sh.aa_safe)) }
      }
L50   if v3_survival_incoming(player, data, champ) >= champ.hp(0x670)          // (46275~46281) incoming < hp 이면 통과
L51      && !nexus_final_stand(player, data) {
L52     return (parameter, 99999, SmallActionPlay::RunAway(SmallActionRunAway::new(data, player, 5)))   // sret +0x1508=99999, +0x15c1=3 (46293~46313) → ret
      }
    }

L56 if let Some((target_id, until_tick)) = self.pending_global_ult_target(0x530) {   // (46269~46272)
L58   if game.tick()(vt[0x28]) > until_tick → L88                                    // (46322~46332)
L61   if !champ.can_ult() → L88
L64   target = game.get_entity_by_id(target_id)(vt[0x1f0]) ; None → L88               // (46342~46349)
L66   eff = champ.ult_effect()  // 인라인: level(0x5c8) > 4 ? &champ.ult_effect(0x538) : &NONE(@anon.76)
L67   if eff 태그(+0x30) == -1 (None) → L88
L69   enemy = 1 - team
L70~72 visible_enemies = Σ_{i<5} data.cache.player_champion[enemy][i].is_some_and(|e| data.blackboard[enemy].is_recent_visible(game, player, e) && distance_sq(champ, e) < 22500000001)   // 루프 5회 (46427~46511)
L73   if visible_enemies == 0 && CastingTarget::check(&eff.target(+0x28), champ, target) {
L75     return (parameter, 99999, SmallActionPlay::Ult(SmallActionUlt::new(data, target_id)))   // +0x15c1=18 (46528~46548) → ret
      }
L88   self.pending_global_ult_target = None   // 위 게이트 중 하나라도 실패 (46551)
    }

L92 _t_ac = ProfTimer::start(36)
L93 candidates: bumpalo Vec<SmallActionPlay> = self.sub_plan.action_candidates(version, rnd, player, data, &parameter, &self.team_plan(0xf8), debug)   // JT 호스트 → *SubPlan::action_candidates(r16) 분기 (46573)
L94 drop(_t_ac)
L99 if game.get_game_mode() 태그 == 2 /*DeathMatch*/ {                       // (46659~46670) 아니면 candidates 그대로 L125 로
L100  move_speed = champ.stat_cached.move_speed(0x640)
L101~115 filtered = candidates.iter().filter(closure$2).cloned().collect_in(data.context.pool)   // aux m14.ll:58208 (58237~58351)
        closure$2(a):  if a.태그 ∈ 15..=18 (Attack/Skill/Skill2/Ult) → true                      // L103
                       let mut merged = pre_action.clone();                                       // L108
                       merged.merge(version, champ, a.clone());                                   // L109
                       match merged.get_input(version, &mut rnd.clone()/*Array64::clone·실스트림 미소비*/, player, data, &parameter.positioning_score, debug) {   // L110
                         None(-1) → false                                                         // L113
                         Some(Input::Move{x,y})(0) → max(|x-champ.x|, |y-champ.y|) >= move_speed   // L112 체비셰프 ≥ 이속 이면 유지
                         Some(_) → true }
L116  if filtered.len() == 0 {
L119    combat = self.sub_plan.combat_fallback(version, rnd, player, data)
L120    candidates = if combat.len() == 0 { candidates } else { combat }   // (46766~46790)
L121  } else { candidates = filtered }
L122 }
L125 self.v3_last_stand(0x1804)  = nexus_last_stand(player, data)
L126 self.v3_final_stand(0x1805) = nexus_final_stand(player, data)
L127 self.v3_bail_goal(0x1813) = if self.plan 태그(0x5e8)==9 /*Battle*/ { match plan.sub_goal 태그(0x648) { 0 Trace→0, 2 Kiting→1(L130), 3 KitingBack→2(L131), 4 RunAway→3(L132), 5|6 Assassin/Ready→4(L133), 1 Protect→5(L134), 7 End→6(L135) } } else { 7 }   // (46843~46888)
L139 self.v3_cand_src(0x1812) = match self.sub_plan {                                  // switch 태그-2 (46889~46933)
L140   DefenseNexus(idx15) → match last_gate(0x780) { 1→1, 2→2, _→3 }
L141   Battle(idx5)        → if last_bail_gate(0x79d) ∈ 1..=3 { +3 } else if candidates.len()==1 { 7 } else { 8 }
       _ → 0 }
L146 if self.sub_plan 은 Battle(태그7) && candidates.len()==1 {                          // (46934~46940)
L147   match candidates[0].get_action() { SmallAction::RunAway → self.v48_dodge_flee_picks(0x1548) += 1   // 태그 3,4,8 (RunAway/Recall/AroundRunAway)
L148                                     SmallAction::AroundPosition → self.v48_dodge_step_picks(0x1540) += 1   // 태그 7,11,12 + untagged AroundPosition
                                     _ → {} } }                                          // (46958~46991)
L154 judge_accuracy = player.info.parameter.judge_accuracy()
L157 judge_noise_ratio_now = if game.get_game_mode() 태그 == 2 { 0 } else { (1000 - judge_accuracy) >> 1 }   // (46995~47003)
L158 lo = 1000 - ratio ; L159 hi = 1000 + ratio
L165 disc = discriminant(self.plan)  // 태그>1 ? 태그-2 : 4(DeathMatchBattle untagged) (47012~47017)
L166 if self.judge_noise_plan(0x560) != Some(disc) {                                     // (47023~47030)
L167   self.judge_noise_plan = Some(disc)
L168   for i in 0..11 { self.judge_noise_ratio[i](0x17a0+8i) = rnd.gen_range(lo..=hi) }  // ★rnd 소비 11회, i 순서 (47041~47069)
     }
L174 _t_sl = ProfTimer::start(37)
L175 judge_noise_ratio = self.judge_noise_ratio(0x17a0)   // [i64;11] 지역 복사
L177~185 with_score: bumpalo Vec<(i64, SmallActionPlay)> = candidates.into_iter().map(closure$3).collect_in(pool)   // aux m01.ll:48954 (49060~49311)
        closure$3(a):  score = self.sub_plan.score(version, &parameter, rnd/*실스트림*/, player, data, &a, debug)   // L178 (49093)
                       k = a.get_action() 판별자 (SmallAction: RunAway0 · Positioning1 · Around2 · AroundPosition3 · Trace4 · Attack6 · Skill7 · Skill2 8 · Ult9 · Stop10 ; Dodge5 는 생성 안 됨)   // L179
                       if |score| < 6 { score } else { judge_noise_ratio[k] * score / 1000 }   // L180~L183 (49164~49173, sdiv)
                       (score, a)                                                                 // L184
L186 drop(_t_sl)
L189 if data.context.trace_level(0x39) != Off(0) {                                       // (47217~47220) Off 면 → 배치 F(줄 254, %688)
L190   if with_score.iter().any(|(_, a)| a.is_ult_escape())  /* a 태그==3 RunAway && with_ult(+0x81) */ {   // (47250~47276)
L192     hp_ratio = champ.hp * 100 / champ.stat_cached.hp(0x628)   // 분모 0 → panic_const_div_by_zero (47293~47302)
L193~196 visible_enemies = Σ 적팀 5명: is_some && blackboard[1-team].is_recent_visible(game, player, e) && distance_sq < 22500000001   // L72 와 동형 (47364~47449)
L199     nearby_allies = Σ 아군 5명(언롤): is_some && e.id(0x5c0) != champ.id && distance_sq(champ, e) < 22500000001   // 가시성 조건 없음 (47455~47821)
L201     self.pending_trace_events(0x858).push(PendingTraceEvent{ event: UltEscape{hp_ratio, visible_enemies, nearby_allies, ult_used:false}, tick: game.tick()(vt[0x28]) })   // L202 의 tick 호출 포함 (47823~47877)
       }                                                                                  // → 배치 F(줄 215, %710)
     }

// rnd(StdRng) 소비 순서(이 범위): L11 calculate_score_parameter_value(계약) → L93 action_candidates(계약) → [L119 combat_fallback(계약, 데스매치·필터 전멸 시)] → [L169 gen_range ×11 (플랜 판별자 변경 시)] → L178 SubPlan::score × candidates.len() (후보 순서). L110 필터의 get_input 은 rnd 복제본이라 비소비.
// 반환 경로(이 범위): L52 RunAway 99999 · L75 Ult 99999 — 둘 다 parameter 이동 + 즉시 ret(%2439). 나머지는 배치 F·G.
// 사장 코드: reach.py(version=2, gamemode=0) 기준 사장 호출부 0 — NA 봉인 없음.

// auction.rs:202~339 (배치 F)
// 진입: ①%710(L215) ← %697(L190 루프 종료)/%940(L201 push 후) — 즉 E 의 `if context.trace_level(0x39) != Off {…}`(L189, 47217~47220) 블록 안 ②%688(L254) ← %684(L189 trace_level==Off) · %710(L215 불일치) · L221/L223/L239 종료.
// L202: `tick: game.tick()`(vtable+0x28, 47823~47825 %924) — 배치 E 의 L199~201 PendingTraceEvent push(47832 에서 +176 에 저장) 의 마지막 필드. 판정 없음 → 배치 E(줄 201).

// ── [L215~242] TowerEngage 트레이스 (trace_level != Off 안에서만 · 관측 전용) ──
L215 (47280~47289): if self.plan@tag(0x5e8)==9(Battle) && self.plan.Battle.0.sub_goal@tag(0x648)==4(RunAway)  [select 로 접힘 — 0x648 은 태그 무관 로드]
  L217 (47883): let towers = data.cache.iter_towers_without_nexus(player.info.team)   // sret 120B Chain<Flatten<[Option<&Entity>;6]>, Copied<Iter<&Entity>>>
  L218~219: let nearest = towers.filter(closure#7 |t| t.can_target())            // can_target(0x6b9) && block_target_tick(0x6a0)==0 (aux m14.ll:58362~58370)
                           .min_by_key(closure#8 |t| t.distance_sq(champ))          // 인라인 min_by_key: 첫 통과 원소를 find(47914~48088)로 뽑아 dist²(48104~48132) 계산 → 나머지는 Map<Filter,..>::fold(48137, m12.ll:16938) 로 reduce
  L221 (48150): if let Some(tower) = nearest {                                  // null → L254
    L222 (48156~48184): let d2 = tower.distance_sq(champ)                        // |dx|²+|dy|² (u64, 0x660/0x668)
    L223 (48186): if d2 < 2500000001 {                                           // 50000² 이내. 아니면 → L254
      L224 (48191~48194): let tower_range = tower.attack_effect.as_ref()         // 0x4c0 == -1 → None
      L225 (48200~48240): .map(closure#9 |e| e.range + e.growth_range*(tower.level-1) + tower.stat_buff_cached.range + tower.radius())   // radius() = radius_mult==0 ? radius : radius*(radius_mult+100)/100
      L226 (48244): .unwrap_or(0)
      L227 (48248~48251): let enemies = &cache.player_champion[1 - team]         // [Option<&Entity>;5]
      L228~231 (48287~48401): let enemy_in_tower_range = enemies.iter().flatten().any(closure#10 |e|
            data.blackboard[1 - team].is_recent_visible(game, player, e)          // L231 앞 항(48362 호출은 무조건, 결과는 select)
            && e.distance_sq(tower) <= (tower_range + e.radius())²)               // L229~231 (48366~48386)
      L234 (48403~48412): let hp_ratio = champ.hp*100 / champ.stat_cached.hp     // 분모 0 → div_by_zero panic(Location 234:30)
      L235~237 (48463~48506): let enemy_count = enemies.iter().flatten().filter(closure#11 |e| blackboard[1-team].is_recent_visible(game, player, e)).count()
      L239~242 (48516~48575): self.pending_trace_events.push(PendingTraceEvent{ event: TowerEngage{ tower_distance: (d2 as f64).sqrt() as u64 /*L242 fptoui.sat*/, hp_ratio, enemy_count, enemy_in_tower_range }, tick: game.tick() /*L240*/ })
    } }

// ── [L254~256] 디버그 후보 덤프 ──
L254 (47223~47226): if data.context.debug(0x3b) {
  L255~256 (48623~48967): for (score, play) in with_score.iter() {   // 원소 192B: +0 score · +8 play · +0xb9 태그
      debug.infos.entry(champ.id).or_insert_with(Vec::new).push(format!("{:?}: {}", play.get_action(), score))   // get_action = small_action.rs:308~326 인라인(태그→game_core::SmallAction 변환표는 아래 ※)
  } }

// ── [L260~261] 후보 0 개 경고(디버그 게이트 없음 — stdout) ──
L260 (48582~48585): if with_score.len()==0 {
  L261 (48974~48985): println!("!no action candidates, team_plan: {:?}, plan: {:?}, sub_plan: {:?}", self.team_plan, self.plan, self.sub_plan) }

// ── [L269~290] 「음수 점수 + 아군 대상 캐스트」 후보 배제 ──
L269 (48590~48594 / 48993~48997): let bad = closure#12 |&(score, play)| {   // env = {game.data, game.vtable, champ}
    L270: score < 0 (`icmp sgt %1322, -1` 거짓)
    L271: && matches!(play, Skill|Skill2|Ult)  (태그 16..=18)
    L272~273: && game.get_entity_by_id(play.target /*play+8*/).is_some_and(|e| e.team == champ.team)  // TeamType eq 인라인(entity.rs:1127)
};
L277 (49025~49113): let any_bad = with_score.iter().any(closure#13 = bad 인라인);   // 루프 1319~1348, 통과 시 1350
L278 (49144): if !any_bad { with_score 그대로(%58 ← %70, 플래그 %1370=0) }
else {
  L280~281 (49135~49140): let filtered: bumpalo Vec<(i64,Play)> = with_score.iter().filter(closure#14 |x| !bad(x)).cloned().collect()   // aux m14.ll:58379~58462: bad 의 부정 (score≥0 · 비캐스트 · 대상 없음 · 타팀 → true)
  L282 (49151~49154): if filtered.is_empty() { L287: with_score 유지; L289: drop(filtered) }
  L288 (49177): else { with_score = filtered (플래그 %1368=1) }
}

// ── [L291] 최고점 ──
L291 (49192~49274): let max_score = with_score.iter().max_by_key(closure#15 |x| x.0).unwrap().0;   // 비어 있으면 unwrap_failed(Location 291:59). 인라인 max_by_key: 첫 원소를 초기값으로 fold(m12.ll:33083 — 동률은 뒤 원소, 점수만 쓰므로 무영향)

// ── [L300~307] v2 도주 대체 후보 ──
L300 (49275~49277): if version > 1 (%206 · E L29) && max_score < -8999999 {
  L301 (49312~49318): if !matches!(game.get_game_mode() /*vtable+0x40*/, DeathMatch /*태그 2*/) {
    L302 (49323~49329): let run = SmallActionPlay::RunAway(SmallActionRunAway::new(data, player, 5 /*end_delay*/));   // 태그 3 store +177
    L303 (49331~49333): let score = self.sub_plan.score(version, &parameter, rnd, player, data, &run, debug);   // ★rnd 소비 가능(콜리 내부)
    L304 (49342~49344): if score > max_score {
      L305 (49352~49356): return (parameter, score, run);   // sret +0/+5384/+5392 → %1931(L416 에필로그)
    }
    L307 (49347): drop(run)  } }

// ── [L309~326] 최고점 후보 중 하나 선택 (rnd) ──
L309~310 (49288~49309): let max_score_actions: bumpalo Vec<SmallActionPlay> = with_score.iter().filter(closure#16 |x| x.0 == max_score).map(closure#17 |x| x.1.clone()).collect();
L313 (49363~49368): if ignore_action.len() == 0 {
  L314 (49374~49433): result = (max_score, max_score_actions.choose(rnd).unwrap().clone())   // ★rnd ①(비어 있으면 unwrap_failed 314:49 — L291 통과 시 불가)
} else {
  L316~319 (49386~49408): let without_ignore_action: bumpalo Vec<SmallActionPlay> = max_score_actions.iter()
        .filter(closure#18 |a| !ignore_action.iter().any(|(_, ia)| *ia == a.get_action()))   // aux m14.ll:58480~58773 (usize 첫 필드 무시 · SmallAction 태그/페이로드 완전 일치)
        .map(closure#19 clone).collect();
  L321 (49445~49448): if without_ignore_action.is_empty() {
    L322 (49453~49457): result = (max_score, max_score_actions.choose(rnd).unwrap().clone())   // ★rnd ② (unwrap_failed 322:39)
  } else {
    L324 (49462~49464): result = (max_score, without_ignore_action.choose(rnd).unwrap().clone())   // ★rnd ③ (unwrap_failed 324:43)
  }
  L326 (49491~49506): drop(without_ignore_action) }
// choose = len==0 ? None : &v[gen_range(0..len as u32)] — StdRng u32 워드 ≥1 소비(거부 루프, len==1 도 소비)

// ── [L329~339] 디버그 로그 (data.context.debug) ──
L329 (49438): if debug(%691) {
  L330 (49536~49542): if matches!(result.1, Skill|Skill2|Ult) {
    L331 (49557~49599): if game.get_entity_by_id(result.1.target).is_some_and(closure#20 |e| e.team == champ.team && e.is_champion() /*ty@tag==13*/) {
      L332~333 (49608~49817): debug.add_log(data, player, format!("\nBUFFPICK T{} {:?} act={:?} score={} sub={:?}", team, player.info.position, result.1.get_action(), result.0, self.sub_plan)) } }
  L337 (49551~49554): if result.0 < -8999999 {
    L338~339 (49826~50075): debug.add_log(data, player, format!("\rTBHOLD-PICK T{} {:?} act={:?} score={} sub={:?} n_cand={}", team, position, result.1.get_action(), result.0, self.sub_plan, with_score.len())) }
  → 배치 G(줄 343)  [%1701→%1679 · %1490→%1679]
} else → 배치 G(줄 356) [%1444→%1474]

// ※ get_action(small_action.rs:308~326) 인라인 변환표(3회 등장: L256·L333·L339 — 48762~48908 / 49614~49778 / 49831~50026): 스위치값 = tag>2 ? tag-3 : 7 →
//   RunAway·Recall·AroundRunAway → SmallAction::RunAway(0) · Around/AroundHide/LaneMinionPosition → Around{target=play+8}(2) · AroundRegion → AroundPosition{x=+16,y=+24}(3) · Positioning → Positioning{+8,+16}(1) · AroundPosition → AroundPosition{+48,+56}(3) · AroundPositionBush → AroundPosition{+8,+16}(3) · AroundBush → AroundPosition{+24,+32}(3) · Trace → Trace{target=+96}(4) · Attack → Attack{+8}(6) · Skill → Skill{+8}(7) · Skill2 → Skill2{+8}(8) · Ult → Ult{+8}(9) · Stop → Stop(10)
// 언와인드: 모든 invoke 는 %679/%1381/%1434/%1486(L416 cleanup) 로 — 판정 무관.

// auction.rs:342~416 (배치 G)
// 진입: 배치 F 의 L337 `%1490` (score < -8999999 거짓 엣지 → L343 %1679) · L338 `%1701` → %1679 · L329 `%1444`(%691 거짓) → L356 %1474 직행. 이 시점의 상태: %49 = (score: i64, action: SmallActionPlay) 192B 로컬(선택 완료) · %95 = ScoreParameter(배치 E) · %58 = bumpalo Vec<(i64, SmallActionPlay)> 채점 후보(배치 F) · %200 = champ(&Entity) · %454/%456 = data.cache.game 팻포인터 · %655 = data.context.pool.

// ── L342~352: 적 챔피언을 겨눈 액션이면 TBPICK 로그 ─────────────────────────
L342~343  let target_id = match &action.1 {            // switch on tag-3 (m13.ll 50002)
              Trace(t)            => t.target,          // 케이스 11 · %49+104 (Trace+0x60)
              Attack(a)|Skill(a)|Skill2(a)|Ult(a) => a.target,   // 케이스 12~15 · %49+16 (+0x8)
              _ => goto L356 };                          // RunAway·Recall·Around*·Positioning·LaneMinionPosition·Stop
L344      if let Some(t) = game.get_entity_by_id(target_id)      // 간접호출(vtable +0x1f0) · null=None → L356
              .filter(|t| t.team != champ.team              // TeamType: 판별자 다르면 ≠ · 둘 다 Player 면 +0x8 비교 · 둘 다 Neutral 이면 = (closure#21 · IR 순서: team 먼저, 다음 is_champion)
                       && matches!(t.ty, EntityType::Champion{..}))   // +0x68 == 13
          {
L345        let dive = if let Trace(t) = &action.1 { t.dive_ignore_tower_escape /* +0x94 */ } else { false };
L347        let s = format!("{:?}", self.sub_plan);       // anon.62 = "{:?}" · SubPlan Debug (self+0x768)
L348        let sub = s[..첫 ' ' 또는 '(' 의 위치(없으면 끝)].to_string();   // closure#22 `|c| c==' '||c=='('` · UTF-8 디코드 루프 후 `(ch & 0x1FFFF7)==32` · to_owned (try_allocate_in + memcpy)
L349        drop(s);
L350        debug.add_log(data, player, format!("TBPICK T{} {:?} act={:?} score={} dive={} sub={}",   // anon.143
                player.info.team(+0x930), player.info.position(+0x9c0), SmallAction::from(&action.1) /*L351 변환 표 ↓*/, action.0, dive, sub));
L352        drop(sub);
          }
// SmallActionPlay → SmallAction 변환(small_action.rs:309~326 인라인 · L351/L385/L407 세 곳 동일): RunAway|Recall|AroundRunAway → RunAway(0) · Positioning → Positioning{x:+0x8,y:+0x10}(1) · Around|AroundHide|LaneMinionPosition → Around{target:+0x8}(2) · AroundRegion → AroundPosition{+0x10,+0x18}(3) · AroundPosition → AroundPosition{+0x30,+0x38}(3) · AroundPositionBush → AroundPosition{+0x8,+0x10}(3) · AroundBush → AroundPosition{+0x18,+0x20}(3) · Trace → Trace{+0x60}(4) · Attack → Attack{+0x8}(6) · Skill → Skill(7) · Skill2 → Skill2(8) · Ult → Ult(9) · Stop → Stop(10). (SmallAction 태그 Direct 8B · Dodge(5) 는 생성 안 됨)

// ── L356~357: LaneMinionPosition 은 손대지 않는다 ─────────────────────────
L356      if action.1 은 LaneMinionPosition(태그 13) {
L357        return (score_param, action.0, action.1); }      // sret +0 memcpy 5384 · +0x1508 · +0x1510 memcpy 184

// ── L360: 액션 종류별 3갈래 ───────────────────────────────────────────────
L360      match &action.1 {
            Trace(t)  => { /* 갈래 A: L360~391 */ }                                              // 케이스 11
            Around|AroundHide|AroundRegion|AroundPosition|AroundPositionBush|AroundBush|LaneMinionPosition => { /* 갈래 B: L393~413 */ }   // 케이스 2,3,4,7,8,9,10 (LaneMinionPosition 은 L357 에서 이미 반환돼 실제 도달 불가)
            _ /* RunAway·Recall·AroundRunAway·Positioning·Attack·Skill·Skill2·Ult·Stop */ =>
L414          return (score_param, action.0, action.1) }                                          // 케이스 0,1,5,6,12,13,14,15,16 → %2211

// ── 갈래 A (Trace): 같은 대상을 겨눈 Attack/Skill/Skill2/Ult 후보로 갈아타기 ──
L360        let target = t.target;                          // %29 ← %49+104
L361        if matches!(self.sub_plan, SubPlan::Battle(..)) {          // self+0x768 == 7
L362          if let Some(t) = game.get_entity_by_id(target) {          // 간접호출 · None 이면 L370 으로
L363            let r = support_min_action_range(champ, t) + 25000;     // u64
L364            if dist2(t.pos(+0x660,+0x668), champ.pos) > r*r {       // |dx|²+|dy|² · ugt(엄격)
L365              return (score_param, action.0, action.1); } } }       // 사거리 밖: 그대로
L370~371    let cast: Vec<&(i64,SmallActionPlay)> = candidates(%58).iter()
                .filter(|a| match &a.1 { Attack(x)|Skill(x)|Skill2(x)|Ult(x) => x.target == target, _ => false })   // closure#23 sl_0 (aux m14:58776 · L372~375 4 arm)
                .collect_in(pool);
L378        if cast.is_empty() {                                        // %28+0x18 == 0
L379          return (score_param, action.0, action.1); }
L382        let max_score = cast.iter().max_by_key(|a| a.0).unwrap().0;   // closure#24 (aux m12:32331) · unwrap_failed anon.152 (비어있지 않으므로 도달 불가) · 동률이면 마지막 원소지만 점수만 쓴다
L383~384    let best: Vec<SmallActionPlay> = cast.iter().filter(|a| a.0 == max_score).map(|a| a.1.clone()).collect_in(pool);   // closure#25 sn_0 · closure#26 so_0 (aux m01:1469)
L385        if max_score >= 0                                                              // icmp sgt -1
               || (SmallAction::from(&action.1) == SmallAction::from(pre_action)          // blackboard.rs:81 PartialEq: 판별자 같고 Positioning/AroundPosition 은 (x,y) · Around/Trace/Attack/Skill/Skill2/Ult 는 target · RunAway/Dodge/Stop 은 무조건 같음
                   && pre_action.move_near_complete(champ)) {                            // IR 순서: 판별자 → 페이로드 → move_near_complete(단락)
L386          let picked = best.choose(rnd).unwrap().clone();           // ★rnd 소비 1회: gen_range<u32>(0..len) — len==1 이어도 소비(zone 거부 루프 ≈ 50% 재시도) · unwrap_failed anon.153 은 best 비어있을 때(도달 불가: max 원소는 반드시 포함)
L387          return (score_param, max_score, picked);                   // ★점수도 max_score 로 교체
L389        } else { return (score_param, action.0, action.1); }         // 음수 최고점 + 이전 액션과 다르거나 아직 도착 전 → 원래 Trace 유지
L391        // drop(best) · drop(cast) (bumpalo Vec 원소 drop) 

// ── 갈래 B (Around 계열): 대상 무관 Attack/Skill/Skill2 양수 후보로 갈아타기 ──
L393~394    let atk: Vec<&(i64,SmallActionPlay)> = candidates(%58).iter()
                .filter(|(score, act)| matches!(act, Attack|Skill|Skill2) && *score > 0)   // closure#27 sp_0 (aux m14:58989) · 태그-15 ult 3 · Ult 제외 · 대상 조건 없음
                .collect_in(pool);
L399        if atk.is_empty() {
L400          return (score_param, action.0, action.1); }
L403        let max_score = atk.iter().max_by_key(|a| a.0).unwrap().0;   // closure#28 sq_0 (aux m12:32436) · anon.154
L404~405    let best: Vec<SmallActionPlay> = atk.iter().filter(|a| a.0 == max_score).map(|a| a.1.clone()).collect_in(pool);   // closure#29 sr_0 · closure#30 ss_0 (aux m01:1653)
L407        if max_score >= 0 || (SmallAction::from(&action.1) == SmallAction::from(pre_action) && pre_action.move_near_complete(champ)) {   // L385 와 동일 구조(max_score>0 이 보장되므로 첫 항이 항상 참 → 뒤 항은 사실상 사장이나 IR 에는 남아 있음)
L408          let picked = best.choose(rnd).unwrap().clone();           // ★rnd 소비 1회 (L386 과 상호 배타)
L409          return (score_param, max_score, picked);
L411        } else { return (score_param, action.0, action.1); }
L413        // drop(best) · drop(atk)

// ── L416: 함수 끝 공통 드롭 ───────────────────────────────────────────────
// 반환 9곳 전부 → %51(SmallActionPlay 로컬, 배치 F) · %58(채점 후보 Vec) · %70(드롭플래그 %1370 시) · %80(드롭플래그 %513 시) 순서로 drop 후 ret void. 갈래 A/B 에서 clone 을 반환한 경로(L387/L409)는 %1475(원 action) 도 drop_glue(%2210) · 원 action 을 memcpy 로 반환한 경로는 move 라 drop 없음. 언와인드 cleanuppad(%108/%114/%157/%421/%448/%515/%634/%679/%1381/%1434/%1486/%2005/%2085/%2240/%2321) 는 같은 로컬들의 예외 경로 드롭.

// rnd: gen_range 사이트 = L386(갈래 A) · L408(갈래 B) 각 1회 · 상호 배타 · 조건부(후보 비었거나 L365/L389/L411 조기반환이면 0회). 그 외 본 범위에 rnd 접점 없음.
// 사장 코드: reach.txt(version=2 · gamemode=0) 사장 0 — NA 봉인 대상 없음.
```
