---

### `205` LegacyPlanHandler::get_small_action — SmallActionPlay 실행기 — ScoreParameter 를 만들어 self 에 반사하고(positioning_score·wave_snapshot·회두홀드 래치), v2+ 는 생존 절대규칙(RunAway 99999)/글로벌 궁 지시(Ult 99999)를 선반환, 아니면 sub_plan.action_candidates 를 pre_action 병합 입력검사로 거른 뒤(데스매치만) 후보마다 SubPlan::score 를 judge_noise_ratio[SmallAction 종류] 로 스케일해 (점수,액션) 목록을 만들고(배치 E 끝) → 이후 ignore_action 제외·최고점 선택·동률 rnd.choose·RunAway 폴백·디버그 로그를 거쳐 (parameter, score, action) 을 반환한다(배치 F·G).

| 항목 | 값 |
|---|---|
| id | `auction__LegacyPlanHandler__get_small_action` |
| 심볼 | `_RNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy7handler7auctionNtB4_17LegacyPlanHandler16get_small_action` |
| 소스 | `game-ai\src\plan_legacy\handler\auction.rs:8` |
| IR | `m13.ll` 45629~52585행 |
| 경로·가시성 | `game_ai::plan_legacy::handler::auction::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::get_small_action` · **pub** |
| 계층 | 플랜 핸들러 |
| exe | `e65b10` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r17` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &std::vec::Vec<(usize, game_core::SmallAction), std::alloc::Global>, &mut game_core::DebugFrameData) -> (game_ai::ScoreParameter, i64, game_ai::SmallActionPlay)
```

**TLS 메모 절(`sig.tls` · 정정 경로 `/specs[205]/sig/tls/<키>`)**

- `name`: 없음
- `role`: -
- `key`: -
- `layout`: -
- `invalidation`: -
- `call_conditions`: 함수 전체(45629~52585) 를 `LocalKey|threadlocal|call_once|__getit` 로 grep 0건 · @anon 상수 21종 정의를 전수 확인 — fn-포인터(`constant ptr @…call_once`) 형이 없음. 콜리 내부 TLS(예: check_kill_die_tick 류)는 콜리 명세 소관.

<details><summary>인자 9개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | &mut (ScoreParameter, i64, SmallActionPlay) 5576B | (배치 F) F 범위의 쓰기 = L305 조기 반환 1곳: +0 ← parameter(%95) 5384B memcpy(m13.ll:49352) · +5384(0x1508) ← score i64(49356) · +5392(0x1510) ← run play 184B memcpy(49354) \| (배치 G) IR 속성: dead_on_unwind noalias writable writeonly sret([5576 x i8]) dereferenceable(5576) | 4 |
| 1 | 1 | self | &mut LegacyPlanHandler (6168B) | (배치 F) F 범위의 &mut 쓰기 = pending_trace_events push(L239) 1곳뿐. 읽기 = plan 태그(0x5e8)·Battle.sub_goal 태그(0x648)·&sub_plan(0x768)·&team_plan(0xf8) \| (배치 G) IR 속성: noalias align 8 dereferenceable(6168) (readonly 없음 — 다른 배치가 쓴다) | 4 |
| 2 | 2 | version | usize (i64 %2) | (배치 F) F 에서는 L300 의 `%206 = version > 1`(배치 E L29 계산) 소비와 L303 SubPlan::score 인자로 전달 \| (배치 G) IR 속성: noundef | 4 |
| 3 | 3 | rnd | &mut StdRng (320B, align 16) | (배치 F) F 의 소비 지점 = ①L303 SubPlan::score(조건부) ②L314/L322/L324 SliceRandom::choose 중 정확히 1곳(비조기반환 경로 전부). 순서 = ①→② \| (배치 G) IR 속성: noalias align 16 dereferenceable(320) | 4 |
| 4 | 4 | player | &PlayerState (2528B) | (배치 F) F 는 배치 E 가 로드한 team(0x930)·position(0x9c0) 값을 쓰고, 포인터 자체를 is_recent_visible·SubPlan::score·RunAway::new·add_log 에 전달 \| (배치 G) IR 속성: readonly captures(address, read_provenance) dereferenceable(2528) | 4 |
| 5 | 5 | data | &OperationData (24B) | (배치 F) cache(+0)·context(+8, debug 플래그 0x3b)·blackboard(+0x10) 읽기 \| (배치 G) IR 속성: readonly dereferenceable(24). 로드는 배치 E 블록(L22/L99/L185)에서 이뤄지고 본 범위는 그 SSA 값을 사용 | 4 |
| 6 | 6 | pre_action | &SmallActionPlay (184B) | (배치 F) F 범위에서 사용 0회(grep `ptr %6` = 0) \| (배치 G) IR 속성: readonly dereferenceable(184) · DI 타입 ref$<enum2$<SmallActionPlay>> — Option 아님(항상 존재) | 4 |
| 7 | 7 | ignore_action | &Vec<(usize, game_core::SmallAction)> (24B) | (배치 F) L313 len(+0x10)==0 검사 · L318 클로저에서 ptr(+0x8)·len 으로 순회하며 SmallAction(+8) 만 비교(usize 는 무시) \| (배치 G) IR 속성: readonly dereferenceable(24) | 4 |
| 8 | 8 | debug | &mut DebugFrameData (224B) | (배치 F) L256 infos(0xa0) HashMap entry→Vec<String>::push · L332/L338 add_log 호출(logs 0x48 은 콜리 내부 쓰기) \| (배치 G) IR 속성: noalias align 8 dereferenceable(224) | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
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

**`mem` 메모리 접근 145건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | LegacyPlanHandler | 0x768 | sub_plan | r | L11·L93·L119·L139·L146·L185 — &mut 로 calculate_score_parameter_value/action_candidates/score 에, & 로 combat_fallback 에; 태그 i64 직접 로드(L139·L146) \| (배치 F) &SubPlan 을 L303 SubPlan::score(self 인자)·L261 println·L332/L338 format Debug 로 전달 | 4 | OK |  |
| 1 | LegacyPlanHandler | 0xf8 | team_plan | r | L93 action_candidates 7번째 인자(콜리 define 은 `ptr nonnull align 8` 만 — readonly 속성 없음) \| (배치 F) L261 println `{:?}` 인자(&TeamPlan, %415) | 4 | OK |  |
| 2 | LegacyPlanHandler | 0x530 | pending_global_ult_target@tag | r | L56 Option<(usize,usize)> 태그(i64 0/1) | 4 | OK |  |
| 3 | LegacyPlanHandler | 0x538 | pending_global_ult_target@Some.0.0 (target_id) | r | L56~L75 | 4 | OK |  |
| 4 | LegacyPlanHandler | 0x540 | pending_global_ult_target@Some.0.1 (until_tick) | r | L58 game.tick() 과 비교 | 4 | OK |  |
| 5 | LegacyPlanHandler | 0x5e8 | plan@tag | r | L127·L165 BigPlan 태그(9=Battle) \| (배치 F) L215(m13.ll:47280) BigPlan 니치 태그 — 9=Battle 만 통과(`assume != 6`) | 4 | OK |  |
| 6 | LegacyPlanHandler | 0x648 | plan@Battle.0.sub_goal@tag | r | L128 BattleSubPlanGoal 태그 0..7 (BigPlan 페이로드 +0x8 → BattlePlan+0x58) \| (배치 F) L215(47286) = BigPlan+0x60 = BattlePlan+0x58 sub_goal 태그(Direct) — 4=RunAway. select 로 접혀 태그 무관하게 로드됨 | 4 | OK |  |
| 7 | LegacyPlanHandler | 0x780 | sub_plan@DefenseNexus.0.last_gate (u8) | r | L140 | 4 | OK |  |
| 8 | LegacyPlanHandler | 0x79d | sub_plan@Battle.0.last_bail_gate (u8) | r | L141 | 4 | OK |  |
| 9 | LegacyPlanHandler | 0x560 | judge_noise_plan@tag | r | L166 Option<Discriminant<BigPlan>> 태그 | 4 | OK |  |
| 10 | LegacyPlanHandler | 0x568 | judge_noise_plan@Some.0 | r | L166 판별자 값(논리 idx) | 4 | OK |  |
| 11 | LegacyPlanHandler | 0x17a0 | judge_noise_ratio [i64;11] | r | L175 88B 를 지역 %71 로 복사해 채점 클로저에 캡처 | 4 | OK |  |
| 12 | LegacyPlanHandler | 0x1809 | v3_turnback_scene_prev | r | L35 래치 읽기 | 4 | OK |  |
| 13 | LegacyPlanHandler | 0x1540 | v48_dodge_step_picks | r | L148 +=1 (read-modify-write) | 4 | OK |  |
| 14 | LegacyPlanHandler | 0x1548 | v48_dodge_flee_picks | r | L147 +=1 | 4 | OK |  |
| 15 | LegacyPlanHandler | 0x858 | pending_trace_events (Vec: cap +0x858 · ptr +0x860 · len +0x868) | r | L201 push | 4 | OK |  |
| 16 | PlayerState | 0x930 | info.team | r | L22 player_champion[team], L32 nexus[team], L69/L193 적팀=1-team \| (배치 F) 값 %190(배치 E L22 로드) — L217 iter_towers_without_nexus(team) · L227 `1-team` · L332/L338 로그 `T{}` \| (배치 G) L350 로그 `T{team}` 인자(%189, 배치 E 에서 gep) | 4 | OK |  |
| 17 | PlayerState | 0x9c0 | info.position@tag (i32) | r | L22 player_champion[team][position] | 4 | OK |  |
| 18 | PlayerState | 0x180 | info.parameter (AthleteParameter 744B) | r | L17·L18·L154 메서드 self | 4 | OK |  |
| 19 | OperationData | 0x0 | cache (&AbstractGameWithCache) | r | %196 (배치 E L22 로드) → champ · game 팻포인터 | 4 | OK |  |
| 20 | OperationData | 0x8 | context (&GameContext) | r | L37 debug · L115/L185 pool(Bump) · L189 trace_level \| (배치 G) %654 (배치 E L185) → +0x0 pool | 4 | OK |  |
| 21 | OperationData | 0x10 | blackboard (&[Blackboard;2]) | r | L70/L194 blackboard[1-team] 을 is_recent_visible 의 self 로 | 4 | OK |  |
| 22 | AbstractGameWithCache | 0x0 | game (&dyn AbstractGame: data +0, vtable +8) | r | vtable 슬롯 0x28 tick · 0x40 get_game_mode · 0x1f0 get_entity_by_id (divtable AbstractGame) | 3 | OK |  |
| 23 | AbstractGameWithCache | 0x170 | nexus[2] (Option<&Entity>) | r | L32 nexus[team] | 4 | OK |  |
| 24 | AbstractGameWithCache | 0x1e0 | player_champion[2][5] (Option<&Entity>) | r | L22 [team][position] unwrap · L69/L193 [1-team][0..5] · L199 [team][0..5] | 4 | OK |  |
| 25 | GameContext | 0x0 | pool (&Bump) | r | L115·L185 bumpalo Vec 할당자 | 4 | OK |  |
| 26 | GameContext | 0x39 | trace_level@tag (0=Off) | r | L189 | 4 | OK |  |
| 27 | GameContext | 0x3b | debug (bool) | r | L37 | 4 | OK |  |
| 28 | Entity | 0x488 | stat_buff_cached.undying | r | L30 base_exempt 1항 | 4 | OK |  |
| 29 | Entity | 0x538 | ult_effect (Option<Effect> 56B) | r | L66 Entity::ult_effect() 인라인(entity.rs:1701): level>4 이면 &self.ult_effect, 아니면 상수 None(@anon.76 = casting 태그 -1) | 4 | OK |  |
| 30 | Entity | 0x560 | ult_effect@Some.0.target (CastingTarget, 4B) | r | L73 CastingTarget::check 의 self (선택된 ult_effect +0x28) | 4 | OK |  |
| 31 | Entity | 0x568 | ult_effect@tag (casting 태그 i32, -1=None) | r | L67 (선택된 ult_effect +0x30) | 4 | OK |  |
| 32 | Entity | 0x5c0 | id | r | L199 자기 자신 제외 | 4 | OK |  |
| 33 | Entity | 0x5c8 | level | r | L66 `level > 4` | 4 | OK |  |
| 34 | Entity | 0x628 | stat_cached.hp (최대 HP) | r | L192 hp%% 분모 (0 이면 panic_const_div_by_zero) | 4 | OK |  |
| 35 | Entity | 0x640 | stat_cached.move_speed | r | L100 move_speed (필터 클로저 임계) | 4 | OK |  |
| 36 | Entity | 0x660 | x | r | L72·L196·L199·L112 distance_sq / 체비셰프 | 4 | OK |  |
| 37 | Entity | 0x668 | y | r |  | 4 | OK |  |
| 38 | Entity | 0x670 | hp | r | L50 생존 임계 · L192 hp%% 분자 | 4 | OK |  |
| 39 | ScoreParameter | 0x0 | wave_snapshot@tag | r | L19 store 1 (Some) | 4 | OK |  |
| 40 | ScoreParameter | 0x8 | wave_snapshot@Some.0 (MinionWaveSnapshot 2320B) | r | L19 memcpy | 4 | OK |  |
| 41 | ScoreParameter | 0x9f0 | positioning_score (PositioningScoreData 2760B) | r | L12 self 로 복사 · L115 클로저 캡처(&) | 4 | OK |  |
| 42 | ScoreParameter | 0x1500 | v3_turnback_hold (bool) | r | L35 store · L38 로그 | 4 | OK |  |
| 43 | SoloHuntObs(16B, solo_hunt_obs sret) | 0x8 | al120 (u8) | r | +0x0 slack(u64) · +0x8 al120 · +0x9 ally_ctx · +0xa en150 · +0xb chasers · +0xc aa_safe(bool) · +0xd kit_safe(bool) — DWARF !49211~!49220 | 3 | OK |  |
| 44 | SoloHuntObs | 0x9 | ally_ctx (u8) | r | L34 `> 1` 이면 장면 아님 | 4 | OK |  |
| 45 | SoloHuntObs | 0xa | en150 (u8) | r | L34 al120+1 < en150 (열세) | 4 | OK |  |
| 46 | SoloHuntObs | 0xb | chasers (u8) | r | L34 != 0 (피추격) | 4 | OK |  |
| 47 | SoloHuntObs | 0xc | aa_safe (bool) | r | L34 최종값(사거리 밖) | 4 | OK |  |
| 48 | SmallActionPlay | 0xb1 | @tag (니치 1B: 3 RunAway … 19 Stop, untagged AroundPosition) | r | L103·L147·L179·L190 · 반환 슬롯 +0x15c1 | 4 | OK |  |
| 49 | (i64, SmallActionPlay) 192B (with_score 원소) | 0x89 | .1@RunAway.0.with_ult (SmallActionRunAway+0x81) | r | L190 is_ult_escape: .1 태그(+0xb9)==3 && with_ult. 원소 = { i64 score @+0, SmallActionPlay @+0x8 } | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) |  |
| 50 | bumpalo Vec<T> 32B (candidates·filtered·with_score) | 0x18 | len | r | +0x0 ptr · +0x8 &Bump · +0x10 cap · +0x18 len (m01.ll:48966~48979 초기화 순서로 확정) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 51 | ProfTimer 24B (지역 _t_csp/_t_ws/_t_ac/_t_sl) | 0x10 | nanos (i32, -1=비활성) | r | +0 phase(i64 33/35/36/37) · +8 Instant.secs · +0x10 Instant.nanos; drop 시 nanos!=-1 → elapsed → PHASE_NANOS[phase]+=ns, PHASE_CALLS[phase]+=1 (phase<132 바운즈) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 52 | Option<Input> 32B (get_input sret, 필터 클로저) | 0x8 | Move.x (u64) · +0x10 Move.y | r | +0 태그 i64: -1 None · 0 Move · 1 Return · 2 Attack · 3 Skill · 4 Skill2 · 5 Ult | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 53 | LegacyPlanHandler | 0x868 | pending_trace_events.len | r | L239 push 전 len 읽기(48545) — 쓰기는 writes 참조 | 4 | OK |  |
| 54 | OperationData | 0x0 | cache | r | &AbstractGameWithCache(%196) — L217 iter_towers_without_nexus self · game 팻포인터 소스 | 4 | OK |  |
| 55 | OperationData | 0x8 | context | r | &GameContext(%654) — L254/L329 debug 플래그 | 4 | OK |  |
| 56 | OperationData | 0x10 | blackboard | r | L228(48254) &[Blackboard;2] → `[1 - team]` 원소를 is_recent_visible 에 전달 | 4 | OK |  |
| 57 | GameContext | 0x3b | debug | r | L254(47223 %654+59)·L329(%691 재사용) — 디버그 로그 분기 게이트 | 4 | OK |  |
| 58 | AbstractGameWithCache | 0x0 | game.data_ptr | r | &dyn AbstractGame 데이터(%454) — 배치 E L99 로드, F 는 vtable 호출 self 로 사용 | 4 | OK |  |
| 59 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r | %456 — F 의 슬롯: +0x28 tick(L202·L240) · +0x40 get_game_mode(L301) · +0x1f0 get_entity_by_id(L273·L331·closure) \| (배치 G) %456 — +0x1f0 슬롯 로드 | 4 | OK |  |
| 60 | AbstractGameWithCache | 0x1e0 | player_champion[team][pos] / [1-team][0..5] | r | %200 champ = 배치 E L22 로드. F 는 L227(48251) `[5 x ptr]` 행 `1-team` 을 잡아 L228·L237 에서 5칸 순회(null=None 건너뜀) | 4 | OK |  |
| 61 | PlayerState | 0x9c0 | info.position | r | &Position(%193) — L332/L338 로그 `{:?}` 인자로만 \| (배치 G) L350 로그 `{:?}` 인자(%193) | 4 | OK |  |
| 62 | Entity(tower) | 0x660 | x | r | L219 key·L222·L229 distance_sq | 4 | OK |  |
| 63 | Entity(tower) | 0x668 | y | r | 동상 | 4 | OK |  |
| 64 | Entity(tower) | 0x4c0 | attack_effect@tag | r | L224(48191) i32 == -1 → None(니치) → tower_range=0 | 4 | OK |  |
| 65 | Entity(tower) | 0x4a0 | attack_effect@Some.0.range | r | L225 closure#9 (48200) | 4 | OK |  |
| 66 | Entity(tower) | 0x4a8 | attack_effect@Some.0.growth_range | r | L225 (48202) × (level-1) | 4 | OK |  |
| 67 | Entity(tower) | 0x5c8 | level | r | L225 (48209) effect.rs:26 `range(level)` 인라인 | 4 | OK |  |
| 68 | Entity(tower) | 0x438 | stat_buff_cached.range | r | L225 (48211) 가산 | 4 | OK |  |
| 69 | Entity(tower/enemy) | 0x470 | stat_buff_cached.radius_mult | r | entity.rs:1511 `radius()` 인라인: 0 이면 radius 그대로, 아니면 radius*(mult+100)/100 — L225(48213)·L230(48335) | 4 | OK |  |
| 70 | Entity(tower/enemy) | 0x680 | radius | r | 동상(48220/48227/48342/48349) | 4 | OK |  |
| 71 | Entity(tower) | 0x6b9 | can_target | r | aux closure#7(L218) entity.rs:1478 `can_target()` = can_target && block_target_tick==0 (m14.ll:58362) | 4 | OK |  |
| 72 | Entity(tower) | 0x6a0 | block_target_tick | r | 동상(m14.ll:58368) == 0 | 4 | OK |  |
| 73 | Entity(enemy) | 0x660 | x | r | L229(48324) 적 챔피언↔타워 distance_sq | 4 | OK |  |
| 74 | Entity(enemy) | 0x668 | y | r | 동상 | 4 | OK |  |
| 75 | Entity(champ) | 0x628 | stat_cached.hp | r | L234(48403) 분모 — 0 이면 panic_const_div_by_zero(Location auction.rs:234:30) | 4 | OK |  |
| 76 | Entity(champ) | 0x670 | hp | r | L234(48409) hp*100/max | 4 | OK |  |
| 77 | Entity(champ) | 0x5c0 | id | r | L256(48671) debug.infos HashMap 키 | 4 | OK |  |
| 78 | Entity(champ/target) | 0x0 | team@tag | r | TeamType 태그(0=Player,1=Neutral) — L273(closure#12)·L331(closure#20) `e.team == champ.team` 인라인(entity.rs:1127 PartialEq) | 4 | OK |  |
| 79 | Entity(champ/target) | 0x8 | team@Player.0 | r | 태그 둘 다 0 일 때만 비교(49101/49589) | 4 | OK |  |
| 80 | Entity(target) | 0x68 | ty@tag | r | L331(49596) entity.rs:1404 `is_champion()` 인라인 == 13 | 4 | OK |  |
| 81 | (i64, SmallActionPlay) 192B 원소 | 0x0 | score | r | with_score 원소 점수 — L256·L270·L291·L310 | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) |  |
| 82 | (i64, SmallActionPlay) 192B 원소 | 0xb9 | play@tag(=play+0xb1) | r | SmallActionPlay 니치 태그(3..=19, 7=AroundPosition 암묵, 10 은 무효 → `assume != 10`). Skill/Skill2/Ult = 16/17/18 (`tag-16 ult 3`) | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) |  |
| 83 | SmallActionPlay | 0x8 | Skill/Skill2/Ult.0.target | r | L271~273·L331: 캐스트 대상 엔티티 id → get_entity_by_id. get_action 인라인에선 Around/AroundHide/LaneMinionPosition(target)·Positioning/AroundPositionBush(x)·Attack/Skill/Skill2/Ult(target) 도 +0x8 | 4 | OK |  |
| 84 | SmallActionPlay | 0x60 | Trace.0.target | r | get_action 인라인(small_action.rs:321) L256/L333/L339 — 튜플 기준 +104 | 4 | OK |  |
| 85 | SmallActionPlay | 0x30 | AroundPosition.0.(x,y)=+0x30/+0x38 | r | get_action 인라인(small_action.rs:317) — 튜플 기준 +56/+64 | 4 | OK |  |
| 86 | bumpalo Vec (with_score/filtered/max_score_actions/without_ignore_action) 32B | 0x18 | len | r | +0 ptr · +8 cap · +0x10 &Bump · +0x18 len — L260·L282·L291·L313~L324·L338 n_cand | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 87 | Vec<(usize,SmallAction)> ignore_action 24B | 0x10 | len | r | std Vec = {cap +0, ptr +8, len +0x10} — L313(49363)·aux closure#18(m14.ll:58495) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 88 | Vec<(usize,SmallAction)> ignore_action 24B | 0x8 | ptr | r | aux closure#18(58493) 원소 32B 순회 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 89 | DebugFrameData | 0xa0 | infos | r | L256(48639) HashMap<usize, Vec<String>> rustc_entry(키=champ.id) | 4 | OK |  |
| 90 | LegacyPlanHandler | 0x768 | sub_plan (SubPlan 태그 i64) | r | L347 Debug fmt 인자(%112) · L361 `== 7`(Battle) · assume != 8(DeathBattle 암묵 홀) | 4 | OK |  |
| 91 | GameContext | 0x0 | pool (&bumpalo::Bump) | r | %655 — L370/383/393/404 collect_in 할당자 | 4 | OK |  |
| 92 | AbstractGameWithCache | 0x0 | game.data_ptr (dyn AbstractGame) | r | %454 — get_entity_by_id 의 self | 4 | OK |  |
| 93 | AbstractGameWithCache | 0x1e0 | player_champion[team][pos] (Option<&Entity>) | r | %200 = champ (배치 E L22 · [2][5]×8B 인덱싱) — L344 team 비교 · L363/364/385/407 인자 | 4 | OK |  |
| 94 | AbstractGame vtable | 0x1f0 | get_entity_by_id (divtable 98% 일치) | r | L344 · L362 간접호출 `fn(&self, id: usize) -> Option<&Entity>`(null = None) | 4 | 확인불가(vtable 슬롯) |  |
| 95 | Entity | 0x0 | team@tag (TeamType: 0 Player / 1 Neutral) | r | L344 t.team vs champ.team 판별자 비교(%1714/%1715) | 4 | OK |  |
| 96 | Entity | 0x8 | team.Player.0 (usize) | r | L344 판별자 같고 Player 면 페이로드 비교(%1719/%1722) | 4 | OK |  |
| 97 | Entity | 0x68 | ty@tag (EntityType · 13 = Champion) | r | L344 `== 13` | 4 | OK |  |
| 98 | Entity | 0x660 | x (u64) | r | L364 target(%1945)·champ(%200) 거리 제곱 | 4 | OK |  |
| 99 | Entity | 0x668 | y (u64) | r | L364 | 4 | OK |  |
| 100 | (i64, SmallActionPlay) 로컬 %49 (192B) | 0x0 | .0 score (i64) | r | L357/365/379/389/400/411/414 반환 점수 · L350 로그 `score=` · (L337 `< -8999999` 은 배치 F) | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) |  |
| 101 | (i64, SmallActionPlay) 로컬 %49 | 0x8 | .1 action (SmallActionPlay, %1475) | r | 반환 memcpy 원본 · 아래 SmallActionPlay 오프셋은 이 +8 을 뺀 값 | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) |  |
| 102 | SmallActionPlay | 0xb1 | 태그 (niche · 본문은 %49+185 / pre_action %6+177) | r | L343 · L345 · L351 · L356 · L360 · L385 · L407 switch · aux sl_0/sp_0 | 4 | OK |  |
| 103 | SmallActionPlay::Trace | 0x60 | target (usize) | r | L343 · L360 target_id (%49+104 / %6+96) · L351/385/407 변환 Trace{target_id} | 4 | 확인불가(★모호: 동명 def_path 8개 [('game_ai::SmallAct) |  |
| 104 | SmallActionPlay::Trace | 0x94 | dive_ignore_tower_escape (bool) | r | L345 로그 `dive=` (%49+156 — Trace 가 아니면 false). qcspec C3 경고 사유: 본문 gep 는 튜플 기준 156 이고 148 은 튜플 +8 을 뺀 SmallActionPlay 기준값(tcxdict SmallActionTrace 0x94 확인) | 3 | 확인불가(★모호: 동명 def_path 8개 [('game_ai::SmallAct) |  |
| 105 | SmallActionPlay::Attack\|Skill\|Skill2\|Ult | 0x8 | target (usize) | r | L343 target_id(%49+16) · aux sl_0 (a+16) · 변환 Attack/Skill/Skill2/Ult{target_id} | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 106 | SmallActionPlay::Around\|AroundHide\|LaneMinionPosition | 0x8 | target (usize) | r | L351/385/407 SmallAction 변환 → Around{target_id} (small_action.rs:313/316/320) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 107 | SmallActionPlay::AroundRegion | 0x10 | goal_x / +0x18 goal_y | r | 변환 → AroundPosition{x,y} (small_action.rs:314) | 4 | OK |  |
| 108 | SmallActionPlay::Positioning | 0x8 | goal_x / +0x10 goal_y | r | 변환 → Positioning{x,y} (small_action.rs:312) | 4 | 확인불가(★모호: 동명 def_path 5개 [('game_ai::SmallAct) |  |
| 109 | SmallActionPlay::AroundPosition | 0x30 | around_input.target_x / +0x38 target_y | r | 변환 → AroundPosition{x,y} (small_action.rs:317) | 4 | 확인불가(★모호: 동명 def_path 2개 [('game_ai::SmallAct) |  |
| 110 | SmallActionPlay::AroundPositionBush | 0x8 | target_x / +0x10 target_y | r | 변환 → AroundPosition (small_action.rs:318) | 4 | OK |  |
| 111 | SmallActionPlay::AroundBush | 0x18 | target_x / +0x20 target_y | r | 변환 → AroundPosition (small_action.rs:319) | 4 | 확인불가(★모호: 동명 def_path 2개 [('game_ai::AroundBu) |  |
| 112 | bumpalo Vec<(i64,SmallActionPlay)> 로컬 %58 (32B · 배치 F 가 채운 채점 후보) | 0x0 | ptr / +0x18 len (원소 192B stride) | r | L370/393 slice iter 원본 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 113 | bumpalo Vec<&Tx> 로컬 %28/%22 | 0x18 | len | r | L378/399 `is_empty()` · L382/403 max · L383/404 재순회 (+0x0 ptr · 원소 8B) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 114 | bumpalo Vec<SmallActionPlay> 로컬 %25/%20 | 0x0 | ptr / +0x18 len | r | L386/408 choose(slice) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 115 | LegacyPlanHandler | 0x990 | positioning_score | w | L12 무조건 | 4 | OK | parameter.positioning_score (memcpy 2760B, 45873) |
| 116 | LegacyPlanHandler | 0x1809 | v3_turnback_scene_prev | w | L36 — version>1 && game_mode!=DeathMatch 일 때만 | 4 | OK | scene_now (46184) |
| 117 | LegacyPlanHandler | 0x530 | pending_global_ult_target@tag | w | L88 — Some 이었는데 L58~L73 게이트 중 하나라도 실패하면 소거 | 4 | OK | 0 (None) (46551) |
| 118 | LegacyPlanHandler | 0x1804 | v3_last_stand | w | L125 무조건 | 4 | OK | nexus_last_stand(player,data) (46834) |
| 119 | LegacyPlanHandler | 0x1805 | v3_final_stand | w | L126 무조건 | 4 | OK | nexus_final_stand(player,data) (46841) |
| 120 | LegacyPlanHandler | 0x1813 | v3_bail_goal (u8) | w | L127~L135 | 4 | OK | plan==Battle ? {Trace 0, Kiting 1, KitingBack 2, RunAway 3, Assassin\|AssassinReady 4, Protect 5, End 6} : 7 (46886~46888) |
| 121 | LegacyPlanHandler | 0x1812 | v3_cand_src (u8) | w | L139~L141 | 4 | OK | sub_plan 이 DefenseNexus: last_gate 1→1, 2→2, else 3 / Battle: last_bail_gate∈1..=3 → +3 (4..6), else candidates.len()==1 ? 7 : 8 / 그 외 0 (46931~46933) |
| 122 | LegacyPlanHandler | 0x1548 | v48_dodge_flee_picks | w | L147 — sub_plan==Battle && candidates.len()==1 && candidates[0].get_action()==SmallAction::RunAway | 4 | OK | +=1 (46943~46947) |
| 123 | LegacyPlanHandler | 0x1540 | v48_dodge_step_picks | w | L148 — 같은 조건에서 get_action()==SmallAction::AroundPosition | 4 | OK | +=1 |
| 124 | LegacyPlanHandler | 0x560 | judge_noise_plan@tag | w | L167 — 현재 플랜 판별자와 다를 때만 | 4 | OK | 1 (Some) (47033) |
| 125 | LegacyPlanHandler | 0x568 | judge_noise_plan@Some.0 | w | L167 | 4 | OK | discriminant(self.plan) = 논리 idx (47034) |
| 126 | LegacyPlanHandler | 0x17a0 | judge_noise_ratio[0..11] (0x17a0~0x17f7) | w | L168~169 — L167 과 같은 조건 | 4 | OK | rnd.gen_range(lo..=hi) ×11 순서대로 (47068) |
| 127 | LegacyPlanHandler | 0x858 | pending_trace_events.push(PendingTraceEvent 184B) | w | L201~202 — trace_level!=Off && with_score 에 is_ult_escape 액션 존재 | 4 | 부분일치(첫 성분(컨테이너)만 맞고 **마지막 성분이 다르다** — 사람이 봐야 ) | event=UltEscape{hp_ratio, visible_enemies, nearby_allies, ult_used:false}, tick=game.tick() (47831~47877; len 0x868 +=1, 필요 시 grow_one) |
| 128 | ScoreParameter(지역 parameter %95) | 0x0 | wave_snapshot | w | L19 — 반환 슬롯 +0x0 으로 이동되므로 반환값의 일부 | 4 | OK | Some(build_minion_wave_snapshot(...)) (45980~45982) |
| 129 | ScoreParameter(지역 parameter) | 0x1500 | v3_turnback_hold | w | L35 — 조건: version>1 && game_mode!=DeathMatch (그 외엔 calculate_score_parameter 가 준 값 유지) | 4 | OK | scene_now \|\| self.v3_turnback_scene_prev (46183) |
| 130 | (sret) | 0x1508 | score | w | L52·L75 선반환 | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) | 99999 (46309, 46544) |
| 131 | (sret) | 0x15c1 | action@tag | w | L52 / L75 | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) | 3 RunAway (46311) / 18 Ult (46546) |
| 132 | (sret) | 0x1510 | action payload | w | L52 / L75 | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) | SmallActionRunAway::new(data, player, 5) 136B / SmallActionUlt::new(data, target_id) 24B |
| 133 | (sret) | 0x0 | ScoreParameter | w | L52 / L75 (다른 배치의 반환도 동일 패턴) | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) | parameter 이동(memcpy 5384B) |
| 134 | LegacyPlanHandler | 0x868 | pending_trace_events.len | w | L239(m13.ll:48575) TowerEngage push — cap==len 이면 grow_one(0x858 RawVec: cap +0x858 · ptr +0x860 도 콜리 안에서 갱신) | 4 | OK | len+1 |
| 135 | LegacyPlanHandler | 0x860 -> pending_trace_events.ptr[len]+0..184 | pending_trace_events 원소(PendingTraceEvent 184B) | w | L239~242 (48530~48539 조립 · 48573 memcpy 184B). 조건 = trace_level!=Off(E L189) && plan==Battle&&sub_goal==RunAway(L215) && 가장 가까운 can_target 아군 타워 존재(L221) && dist²<2500000001(L223). 관측 전용(판정 무영향) | 4 | OK | tag(+0)=-9223372036854775796(=TraceEventType::TowerEngage) · +8 tower_distance=sqrt(dist²)(u64) · +0x10 hp_ratio=hp*100/max_hp · +0x18 enemy_count(가시 적챔프 수) · +0x20 enemy_in_tower_range(u8 0/1) · +0xb0 tick=game.tick() |
| 136 | DebugFrameData | 0xa0 -> infos[champ.id].push(String) | infos 항목 | w | L255~256 debug==true 일 때 with_score 전 원소. 비어 있으면 entry 에 Vec::new()(cap0,ptr=8,len0) 삽입(48690~48708) | 4 | OK | format!("{:?}: {}", play.get_action(), score) — anon.139 |
| 137 | (sret) | 0x0 | (parameter, score, play) | w | m13.ll:49352~49356 · 반환 후 %1931(L416 에필로그)로 점프 | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) | L305 조기 반환: +0 ← %95 5384B · +0x1508 ← SubPlan::score 반환 · +0x1510 ← RunAway play(태그 3) |
| 138 | StdRng(rnd) | 0x100 | index(ChaCha 블록 커서) | w | SliceRandom::choose(m11.ll:14555) → StdRng::gen_range<u32>(m04.ll:14589 · +256=index +272=버퍼 · refill_wide). L314/L322/L324 중 1곳 + L303 SubPlan::score 내부(미열람) | 4 | OK | gen_range(0..len as u32): u32 워드 ≥1개 소비(거부 루프 `while (v*len).lo32 > (len<<ctlz(len))-1` — len==1 이어도 소비, 기대 2회) |
| 139 | (sret) 반환 튜플 | 0x0 | ScoreParameter 5384B | w | 반환 9곳 공통(L357/365/379/387/389/400/409/411/414). ir: m13.ll 50735/50906/50928/51553/51483/51645/52319/52201/51609 | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) | memcpy(%95) — 배치 E 가 만든 score_param 을 그대로 |
| 140 | (sret) 반환 튜플 | 0x1508 | i64 score | w | gep %0, 5384 · store i64 | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) | %49.0 (원 점수) — 단 L387 은 %26(cast max) · L409 는 %21(atk max) |
| 141 | (sret) 반환 튜플 | 0x1510 | SmallActionPlay 184B | w | gep %0, 5392 · memcpy 184 | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) | memcpy(%49+8) — L387/L409 는 choose 결과 clone(%23/%18) |
| 142 | StdRng (rnd) | 0x100 | index (콜리 내부: choose→gen_range<u32>) | w | L386 또는 L408 중 한 곳만 · 본 범위 직접 store 아님(계약) | 4 | OK | +1 per 추출(거부 시 반복) · 64 소진 시 refill_wide |
| 143 | DebugFrameData (debug) | 0x0 | add_log 내부 쓰기 표면 | w | 콜리 계약만 — 표면 미확인 | 4 | 오귀속(사전은 다른 필드를 준다) | String 로그 1건 추가(조건부 L350) |
| 144 | LegacyPlanHandler (self) | 0x0 | (없음) | w | G.ll 슬라이스 store/memcpy 대상 전수 = %0(sret)·로컬 alloca 뿐 | 4 | OK | 본 범위 self 쓰기 0건 |

**`consts` 상수 56건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev | folded_from |
|---|---|---|---|---|---|---|
| 0 | 33 | 9 | 산출값 | ProfTimer phase id (_t_csp = calculate_score_parameter 구간) | 4 |  |
| 1 | 35 | 16 | 산출값 | ProfTimer phase id (_t_ws = wave_snapshot 구간) | 4 |  |
| 2 | 36 | 92 | 산출값 | ProfTimer phase id (_t_ac = action_candidates 구간) | 4 |  |
| 3 | 37 | 174 | 산출값 | ProfTimer phase id (_t_sl = 채점 구간) | 4 |  |
| 4 | 1000000000 | 13 | 임계 | ProfTimer drop: secs*1e9+nanos (ns 환산, 판정 아님) | 4 |  |
| 5 | 1 | 29 | 임계 | `version > 1` = v2 이상에서만 L29~L52(회두 홀드 장면·생존 절대규칙·글로벌 궁) 블록 실행 (icmp ugt, 46081) | 4 |  |
| 6 | 2 | 29 | 태그 | GameMode 태그 2 = DeathMatch (tcxdict --enum GameMode). L29: 데스매치면 L30~38 건너뜀 / L99: 데스매치에서만 필터 / L157: 데스매치면 judge_noise_ratio=0 | 3 |  |
| 7 | 180001 | 32 | 임계 | champ.distance(nexus[team]) < 180001 = ≤180000 (5.625셀) 이면 본진 면제(base_exempt) | 4 |  |
| 8 | 4 | 66 | 임계 | Entity::ult_effect() 인라인: `level > 4`(=5 이상) 이어야 ult_effect 를 본다, 아니면 None 상수 | 4 |  |
| 9 | 22500000001 | 72 | 임계 | 150000²+1 — distance_sq < 이 값 = 150000(4.6875셀) 이내. L72(적 가시 카운트)·L196(적 가시)·L199(아군) 공통 | 4 |  |
| 10 | 99999 | 52 | 산출값 | 선반환 점수(생존 절대규칙 RunAway · 글로벌 궁 Ult) — 경매 최고점을 압도 | 4 |  |
| 11 | 5 | 52 | 태그 | SmallActionRunAway::new 의 3번째 인자(usize) — 도주 발원 코드로 추정(콜리 명세 소관) | 5 |  |
| 12 | 3 | 52 | 센티널 | SmallActionPlay 태그 3 = RunAway (tcxdict --enum SmallActionPlay 니치 시작 3). L190 is_ult_escape 도 ==3 비교 · (경고 사유) 본문의 `shl … 3` 은 다른 배치 범위의 *8 stride 접힘이고 이 3 은 태그값 — 시프트량 아님 | 3 |  |
| 13 | 18 | 75 | 태그 | SmallActionPlay 태그 18 = Ult | 4 |  |
| 14 | 9 | 127 | 센티널 | BigPlan 태그 9 = Battle (니치, idx 7). L127·L165 | 4 |  |
| 15 | 7 | 127 | 태그 | v3_bail_goal: 플랜이 Battle 이 아닐 때 7 / L141: Battle 서브플랜·bail_gate 밖·후보 1개 → 7 | 4 |  |
| 16 | 8 | 141 | 태그 | v3_cand_src: Battle 서브플랜·bail_gate 밖·후보 ≠1개 → 8 | 4 |  |
| 17 | 15 | 139 | 태그 | SubPlan 논리 idx 15 = DefenseNexus (switch 는 태그-2 로 접힘 · 태그 17). 같은 switch 의 5 = Battle(태그 7) | 4 |  |
| 18 | 1000 | 157 | 계수 | judge_noise_ratio = (1000 - judge_accuracy) >> 1 ; lo = 1000 - ratio ; hi = 1000 + ratio ; 채점 스케일 = ratio[k]*score/1000 (천분율) | 4 |  |
| 19 | 1 | 157 | 임계 | `lshr i64 %597, 1` = (1000-judge_accuracy)/2 — 나눗셈 2 가 시프트 1 로 접힘 (47002) | 4 | 2 |
| 20 | 100 | 192 | 계수 | hp_ratio = champ.hp*100 / champ.stat_cached.hp (백분율) | 4 |  |
| 21 | 6 | 180 | 태그 | (aux m01.ll:49165) \|score\| < 6 이면 judge_noise 스케일을 건너뛴다(작은 점수는 노이즈 면제) · (경고 사유) 본문의 `shl … 6` 은 다른 배치 범위의 *64 stride 접힘이고 이 6 은 임계값 — 시프트량 아님 | 4 |  |
| 22 | -15 | 103 | 태그 | (aux m14.ll:58241) 태그-15 < 4 ⇔ 태그 ∈ 15..=18 = Attack/Skill/Skill2/Ult → 입력검사 없이 통과 | 4 |  |
| 23 | -1 | 110 | 태그 | (aux m14.ll:58303) Option<Input> 태그 -1 = None → 후보 탈락 / L67 casting 태그 -1 = ult_effect None | 4 |  |
| 24 | -9223372036854775797 | 201 | 센티널 | TraceEventType 니치 태그 = 2^63+11 → idx 11 UltEscape (tcxdict --enum TraceEventType) | 3 |  |
| 25 | 9 | 215 | 센티널 | BigPlan 메모리태그 9 = Battle(니치: idx7+2). `icmp eq %711, 9`(m13.ll:47283). `assume != 6` 은 DeathMatchBattle 암묵 슬롯 배제 | 4 |  |
| 26 | 4 | 215 | 태그 | BattleSubPlanGoal 태그 4 = RunAway(Direct) — self+0x648 (47287) | 4 |  |
| 27 | 2500000001 | 223 | 임계 | 50000²+1 — 타워↔챔프 dist² < 2500000001 (= ≤ 50000 셀단위 1.56칸)일 때만 TowerEngage 트레이스 생성 (48186). 표기 불가: `<= 2_500_000_000` 과 `< 2_500_000_001` 외연 동일 | 4 |  |
| 28 | -1 | 224 | 센티널 | attack_effect Option 니치 태그(i32 0x4c0) -1 = None → tower_range 0 (48193) | 4 |  |
| 29 | 100 | 225 | 계수 | radius*(radius_mult+100)/100 — entity.rs:1511~1515 `radius()` 인라인(48229/48231, 48351/48353). L234 hp*100/max_hp 의 100 도 동일 리터럴(48411) = 퍼센트 환산 | 4 |  |
| 30 | 5 | 237 | 태그 | player_champion 행 5칸(포지션 수) 순회 상한(48505) — stride 아님, 루프 상한. L228 도 같은 5칸(`%1080 == 40` = 5×8B, 48397) (본문의 `shl …, 5` 는 aux closure#18 의 ignore_action stride 32B 산술 — 이 5 는 시프트량이 아니라 루프 상한) | 4 |  |
| 31 | 40 | 228 | 태그 | 5 슬롯 × 8B 포인터 = 배열 끝 오프셋(48397) — 루프 종료 비교값 | 4 |  |
| 32 | -9223372036854775796 | 239 | 센티널 | TraceEventType 니치 태그 = 0x800000000000000C = TowerEngage(idx12) (48531) | 4 |  |
| 33 | 10 | 256 | 센티널 | SmallActionPlay 태그 10 은 유효하지 않은 값(니치 3..=19 중 7 이 AroundPosition 암묵) — `assume != 10`(48758 등). get_action 인라인 스위치 `tag>2 ? tag-3 : 7` | 4 |  |
| 34 | 7 | 256 | 태그 | get_action 인라인: 태그 ≤2(암묵 AroundPosition) → 논리 idx 7 (48761) | 4 |  |
| 35 | 16 | 271 | 태그 | `tag-16 ult 3` = 태그 16/17/18 = Skill/Skill2/Ult (49055~49056, 49540~49541, aux m14.ll:58406) | 4 |  |
| 36 | 3 | 271 | 태그 | 위 `ult 3` 의 범위 폭(3종) 및 L302 `store i8 3`=SmallActionPlay::RunAway 태그(49329) (본문의 `shl …, 3` 은 포인터 stride 8B 산술 — 이 3 은 시프트량 아님) | 4 |  |
| 37 | -8999999 | 300 | 임계 | max_score < -8999999 (= ≤ -9,000,000) 이면 v2 도주 대체 후보 검토(49275). L337(49552) 도 같은 값으로 디버그 로그 게이트. 표기 불가: `<= -9_000_000` 와 외연 동일 | 4 |  |
| 38 | 1 | 29 | 임계 | `%206 = version > 1`(배치 E L29 · 46081) 을 L300 이 `and` 로 소비 — version ≥ 2 게이트. 값 정의는 배치 E 범위 | 4 |  |
| 39 | 2 | 301 | 태그 | GameMode 태그 2 = DeathMatch → 도주 대체 검토 건너뜀(49317) | 4 |  |
| 40 | 5 | 302 | 태그 | SmallActionRunAway::new(data, player, end_delay=5) 의 셋째 인자(49323) — DI 이름 `end_delay` (shl 피연산자 아님 — 리터럴 인자) | 4 |  |
| 41 | 192 | 291 | 미상 | (i64, SmallActionPlay) 원소 stride 192B (49223 등) — 참고용 | 4 |  |
| 42 | 184 | 314 | 미상 | SmallActionPlay 184B memcpy/원소 stride(49433 등) — 참고용 | 4 |  |
| 43 | 288230376151711744 | 313 | 임계 | 2^58 — Vec::len ≤ isize::MAX/32 `assume`(49365) 컴파일러 힌트, 판정 아님 | 4 |  |
| 44 | 0 | 273 | 태그 | TeamType 태그 0 = Player(페이로드 비교 필요) — 1 = Neutral 은 태그만 같으면 동일(49091/49588) | 4 |  |
| 45 | 13 | 331 | 태그 | EntityType 태그 13 = Champion — `is_champion()` 인라인(49598) | 4 |  |
| 46 | 13 | 356 | 태그 | SmallActionPlay 메모리태그 13 = LaneMinionPosition(idx 10) → 즉시 반환. 같은 값 13 이 L344 에서는 EntityType 태그 Champion(idx 13, Direct) — 두 다른 열거형의 우연 일치 | 4 |  |
| 47 | 14 | 345 | 태그 | SmallActionPlay 메모리태그 14 = Trace(idx 11) → dive_ignore_tower_escape 를 로그에 (L360 의 Trace 분기는 switch 케이스 11 = 태그-3) | 4 |  |
| 48 | 7 | 361 | 태그 | SubPlan 메모리태그 7 = Battle(idx 5, niche_start 2) — Battle 서브플랜일 때만 L362~365 사거리 이탈 검사. (본문의 `select … i8 7` 은 태그<3 방어용 폴딩값이라 무관 · `shl` 무관) | 4 |  |
| 49 | 25000 | 363 | 계수 | support_min_action_range(champ, t) 에 더하는 사거리 여유(좌표 단위 · 셀 32000 의 0.78) — 거리² > (range+25000)² 이면 갈아타기 없이 반환 | 4 |  |
| 50 | 32 | 348 | 태그 | format!("{:?}", sub_plan) 문자열을 앞에서 스캔해 첫 ' '(32) 또는 '('(40) 앞까지 자른다 — 판정식은 `(ch & 0x1FFFF7) == 32` (비트3 마스크 → 32\|40) | 4 |  |
| 51 | 2097143 | 348 | 계수 | 0x1FFFF7 — char 값에서 비트 3 을 지우는 마스크(32 와 40 을 한 비교로) · 플랜 이름 추출 전용 | 4 |  |
| 52 | -15 | 394 | 태그 | aux sp_0: 태그-15 를 ult 3 → 태그 15/16/17 = Attack/Skill/Skill2 만 (Ult 18 제외) — Around 계열 갈아타기 후보 필터 | 4 |  |
| 53 | 3 | 394 | 태그 | aux sp_0 `icmp ult i8 %7, 3` 의 폭(위 -15 와 짝) · 본문 각 switch 의 `add -3`(태그→케이스) 도 같은 값. 본문의 `shl …, 3` 은 &Tx 포인터 배열 stride 8B 계산이라 이 상수와 무관 | 4 |  |
| 54 | -1 | 385 | 임계 | `icmp sgt i64 max_score, -1` = max_score >= 0 (L385 · L407 두 곳) — 최고점 후보가 비음수면 pre_action 검사 없이 갈아탄다 | 4 |  |
| 55 | 0 | 394 | 태그 | aux sp_0 `score > 0`(sgt 0) — Around 계열 갈아타기 후보는 양수 점수만(Trace 계열 sl_0 은 점수 조건 없음) | 4 |  |

**`knobs` 조정점 16건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 회두 홀드 본진 면제 거리 | auction.rs:32 | 180001 | 올리면 넥서스에서 더 멀어도 base_exempt → 표적장면(scene_now)이 안 잡혀 v3_turnback_hold 가 덜 켜진다 | 4 | 기존 |
| 1 | 가시 적/근접 아군 반경(제곱) | auction.rs:72 / 196 / 199 | 22500000001 | 올리면 글로벌 궁 게이트(L73 visible_enemies==0)가 더 자주 막히고, UltEscape 트레이스의 visible_enemies/nearby_allies 가 커진다(트레이스는 계측만) | 4 | 기존 |
| 2 | 글로벌 궁 허용 레벨 | entity.rs:1701 (Entity::ult_effect 인라인, auction.rs:66) | 4 | `level > 4` — 내리면 저레벨에서도 pending_global_ult_target 을 실행한다(game_core 헬퍼라 SDK 밖 변경 필요) | 4 | 기존 |
| 3 | 선반환 점수 | auction.rs:52 / 75 | 99999 | 이 범위 안에서는 즉시 return 이라 값 자체는 비교되지 않는다. 호출자(exe 실행기)가 score 를 다른 후보와 비교한다면 그 쪽 영향 | 4 | 기존 |
| 4 | 판정 노이즈 폭 | auction.rs:157~159 | 1000 | ratio=(1000-judge_accuracy)/2 → [1000-ratio, 1000+ratio] 균등 롤 11개(SmallAction 종류별). 정확도 1000 이면 노이즈 0. 데스매치는 항상 0 | 4 | 기존 |
| 5 | 노이즈 면제 점수 절대값 | auction.rs:180 (aux m01.ll:49165) | 6 | \|score\| < 6 은 스케일 안 함. 올리면 더 많은 후보가 원점수 유지 | 4 | 기존 |
| 6 | 데스매치 필터 이동 임계 | auction.rs:112 (aux m14.ll:58343~58346) | champ.stat_cached.move_speed | Move 입력의 체비셰프 거리 ≥ 이속이어야 후보 유지 — 필드값이라 노브 아님(구조상 제자리 이동 후보 제거) | 4 | 기존 |
| 7 | v2 도주 대체 검토 임계(최고점이 이 값 미만일 때 RunAway 재채점) | auction.rs:300 (m13.ll:49275) | -8999999 | 올리면(예 -5,000,000) 더 높은 점수대에서도 sub_plan.score(RunAway) 와 비교해 도주로 갈아탈 수 있다 · 내리면 거의 발동 안 함. DeathMatch 에선 무효, version ≤1 에선 무효 | 4 | 기존 |
| 8 | 도주 대체 후보의 end_delay | auction.rs:302 (49323) | 5 | SmallActionRunAway::new 셋째 인자 — 의미는 RunAway 명세(r15 SmallActionRunAway) 참조 | 4 | 기존 |
| 9 | 「음수 점수 + 아군 대상 캐스트」 배제 기준 | auction.rs:270 (49046 · aux m14.ll:58397) | 0 | score < 0 인 아군 대상 Skill/Skill2/Ult 후보를 with_score 에서 제거(단 전부 제거되면 원본 유지). 기준을 바꾸면 아군 버프/힐 후보의 생존 범위가 바뀐다 | 4 | 기존 |
| 10 | TowerEngage 트레이스 거리 게이트 | auction.rs:223 (48186) | 2500000001 | 관측 전용 — 올리면 더 먼 아군 타워도 트레이스 대상. 판정·RNG 무영향 | 4 | 기존 |
| 11 | TBHOLD-PICK 디버그 로그 게이트 | auction.rs:337 (49552) | -8999999 | 디버그 전용 — 판정 무영향 | 4 | 기존 |
| 12 | Trace→캐스트 갈아타기 사거리 여유 | auction.rs:363 | 25000 | 올리면 Battle 서브플랜에서 대상이 더 멀어도(사거리+여유 안이면) Attack/Skill 후보 검사로 진행 → 추격 중 캐스트 전환이 잦아짐 · 내리면 사거리 밖 판정이 늘어 Trace 유지 | 4 | 기존 |
| 13 | 갈아타기 최고점 하한 | auction.rs:385 · 407 | 0 | `max_score >= 0`(IR: sgt -1). 하한을 올리면(예: > N) 점수 낮은 캐스트로의 전환이 줄고 pre_action 동일·도착 조건에만 의존 · 내리면(음수 허용) 항상 전환 | 4 | 기존 |
| 14 | Around 계열 후보 점수 하한 | auction.rs:394 (closure#27) | 0 | `score > 0`. 0 이하도 허용하면 Around/Hide/Bush 대기 중 손해 캐스트도 후보가 돼 갈아탈 수 있음 | 4 | 기존 |
| 15 | Around 계열 후보 종류 | auction.rs:394 (closure#27) | Attack\|Skill\|Skill2 (Ult 제외) | Ult 를 포함시키면 대기 중 궁 자동 사용 후보가 열림 · Trace 갈래(L372~375)는 Ult 포함 | 4 | 기존 |

<details><summary>`callees` 피호출자 70건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | action_candidates | game_ai::plan_legacy::sub_plan::SubPlan::action_candidates | pub | fn(&mut game_ai::plan_legacy::sub_plan::SubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\mod.rs:118 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | add_log | game_core::DebugFrameData::add_log | pub | fn(&mut game_core::DebugFrameData, &game_core::OperationData, &game_core::PlayerState, std::string::String) | game-core\src\simulation\game\frame.rs:54 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 2 | build_minion_wave_snapshot | game_ai::build_minion_wave_snapshot | pub | fn(&game_core::PlayerState, &game_core::OperationData, usize, usize) -> game_ai::MinionWaveSnapshot | game-ai\src\utils.rs:611 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 3 | calculate_score_parameter | game_ai::calculate_score_parameter | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> game_ai::ScoreParameter | game-ai\src\score_parameter.rs:1461 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | calculate_score_parameter_value | game_ai::plan_legacy::sub_plan::SubPlan::calculate_score_parameter_value | pub | fn(&mut game_ai::plan_legacy::sub_plan::SubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::ScoreParameter) | game-ai\src\plan_legacy\sub_plan\mod.rs:96 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 5 | can_target | game_core::Entity::can_target | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1477 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 6 | can_target | game_core::OperationData::<'a, 'b>::can_target | pub | fn(&game_core::OperationData<'a/#0, 'b/#1>, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\state\player.rs:23 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 7 | can_ult | game_core::Entity::can_ult | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1734 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 8 | check | game_core::CastingTarget::check | pub | fn(&game_core::CastingTarget, &game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\effect\type.rs:227 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 9 | combat_fallback | game_ai::plan_legacy::sub_plan::SubPlan::combat_fallback | pub | fn(&game_ai::plan_legacy::sub_plan::SubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\mod.rs:157 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 10 | debug | game_core::AbstractBanpickRunner::debug | pub | fn(&Self/#0, &[usize]) | game-core\src\banpick\base_runner.rs:411 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 11 | debug | <game_core::FactoredBanpickAgent as game_core::AbstractBanpickRunner>::debug | pub | fn(&game_core::FactoredBanpickAgent, &[usize]) | game-core\src\banpick\sgd_v2.rs:5891 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 12 | debug | <game_core::LogisticSGDBanpickAgent as game_core::AbstractBanpickRunner>::debug | pub | fn(&game_core::LogisticSGDBanpickAgent, &[usize]) | game-core\src\banpick\sgd.rs:1172 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 13 | distance | game_core::Entity::distance | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2161 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 14 | distance_sq | game_core::utils::distance_sq | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:6 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 15 | distance_sq | game_core::Entity::distance_sq | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2157 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 16 | get_action | game_ai::SmallActionUlt::get_action | in:game_ai | fn(&game_ai::SmallActionUlt) -> game_core::SmallAction | game-ai\src\small_action\cast.rs:286 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 16개 중 상위 3개 |
| 17 | get_action | game_ai::SmallActionPlay::get_action | pub | fn(&game_ai::SmallActionPlay) -> game_core::SmallAction | game-ai\src\small_action.rs:308 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 16개 중 상위 3개 |
| 18 | get_action | game_ai::SmallActionTrace::get_action | in:game_ai | fn(&game_ai::SmallActionTrace) -> game_core::SmallAction | game-ai\src\small_action\trace.rs:403 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 16개 중 상위 3개 |
| 19 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 20 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 21 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 22 | get_game_mode | game_core::AbstractGame::get_game_mode | pub | fn(&Self/#0) -> game_core::GameMode | game-core\src\simulation.rs:78 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 23 | get_game_mode | <game_core::Game as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::Game) -> game_core::GameMode | game-core\src\simulation\game.rs:1707 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 24 | get_game_mode | <game_core::SingleLaneGame as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::SingleLaneGame) -> game_core::GameMode | game-core\src\simulation\game.rs:4248 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 25 | get_input | game_ai::SmallActionPlay::get_input | pub | fn(&mut game_ai::SmallActionPlay, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &mut game_core::DebugFrameData) -> std::option::Option<game_core::Input> | game-ai\src\small_action.rs:157 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 26 | is_champion | game_core::EntityType::is_champion | pub | fn(&game_core::EntityType) -> bool | game-core\src\simulation\entity.rs:1403 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 27 | is_empty | game_core::PatchSetting::is_empty | pub | fn(&game_core::PatchSetting) -> bool | game-core\src\patch_engine.rs:853 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 28 | is_empty | game_core::ModAiRegistry::is_empty | pub | fn(&game_core::ModAiRegistry) -> bool | game-core\src\mod_ai.rs:15 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 29 | is_empty | game_core::PublicChampionEvidence::is_empty | pub | fn(&game_core::PublicChampionEvidence) -> bool | game-core\src\banpick\public_evidence.rs:96 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 30 | is_recent_visible | game_core::Blackboard::is_recent_visible | pub | fn(&game_core::Blackboard, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\game\blackboard.rs:346 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 31 | is_ult_escape | game_ai::SmallActionPlay::is_ult_escape | pub | fn(&game_ai::SmallActionPlay) -> bool | game-ai\src\small_action.rs:437 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 32 | iter_towers_without_nexus | game_core::AbstractGameWithCache::<'a, 'b>::iter_towers_without_nexus | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5750 ~ game_core[6a30]::simulation::{impl#5}::iter_towers_without_nexus::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1830 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 33 | judge_accuracy | game_core::AthleteParameter::judge_accuracy | pub | fn(&game_core::AthleteParameter) -> usize | game-core\src\simulation\state\player.rs:337 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 34 | last_hit_prediction_depth | game_core::AthleteParameter::last_hit_prediction_depth | pub | fn(&game_core::AthleteParameter) -> usize | game-core\src\simulation\state\player.rs:262 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 35 | last_hit_source_quality | game_core::AthleteParameter::last_hit_source_quality | pub | fn(&game_core::AthleteParameter) -> usize | game-core\src\simulation\state\player.rs:271 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 36 | level | <game_ai::WindowStatView<'_> as game_core::AbstractEntity>::level | pub | fn(&game_ai::WindowStatView</#0>) -> usize | game-ai\src\fight_check.rs:96 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 37 | level | game_core::AbstractEntity::level | pub | fn(&Self/#0) -> usize | game-core\src\simulation\entity.rs:718 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 38 | level | <game_core::Entity as game_core::AbstractEntity>::level | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:772 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 39 | merge | game_ai::SmallActionPlay::merge | pub | fn(&mut game_ai::SmallActionPlay, usize, &game_core::Entity, game_ai::SmallActionPlay) | game-ai\src\small_action.rs:397 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 40 | move_near_complete | game_ai::SmallActionPlay::move_near_complete | pub | fn(&game_ai::SmallActionPlay, &game_core::Entity) -> bool | game-ai\src\small_action.rs:413 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 41 | new | game_ai::SmallActionUlt::new | pub | fn(&game_core::OperationData, usize) -> game_ai::SmallActionUlt | game-ai\src\small_action\cast.rs:247 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 42 | new | game_ai::SmallActionRunAway::new | pub | fn(&game_core::OperationData, &game_core::PlayerState, usize) -> game_ai::SmallActionRunAway | game-ai\src\small_action\move_actions.rs:25 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 43 | next | game_view::StatMode::next | in:game_view::ui::match_result_ui | fn(game_view::StatMode) -> game_view::StatMode | game-view\src\ui\match_result_ui.rs:62 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 9개 중 상위 3개 |
| 44 | next | game_view::FlowPeriod::next | in:game_view::ui::finance_ui | fn(&game_view::FlowPeriod) -> game_view::FlowPeriod | game-view\src\ui\finance_ui.rs:180 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 9개 중 상위 3개 |
| 45 | next | game_view::ResultsPeriod::next | pub | fn(game_view::ResultsPeriod) -> game_view::ResultsPeriod | game-view\src\ui\training_ui.rs:44 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 9개 중 상위 3개 |
| 46 | nexus_final_stand | game_ai::plan_legacy::old::nexus_final_stand | pub | fn(&game_core::PlayerState, &game_core::OperationData) -> bool | game-ai\src\plan_legacy\old\defense_nexus.rs:190 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 47 | nexus_last_stand | game_ai::plan_legacy::old::nexus_last_stand | pub | fn(&game_core::PlayerState, &game_core::OperationData) -> bool | game-ai\src\plan_legacy\old\defense_nexus.rs:186 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 48 | position | game_core::PlayerAiContext::<'a, 'b, 'r>::position | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> game_core::Position | game-core\src\mod_ai.rs:587 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 49 | position | game_core::transfer::TeamRosterPlanningContext::position | pub | fn(&game_core::transfer::TeamRosterPlanningContext, game_core::Position) -> std::option::Option<&game_core::transfer::PositionPlanningContext> | game-core\src\transfer\roster_context.rs:234 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 50 | push | <game_core::Staff as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::staff | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\staff.rs:38 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 51 | push | <game_core::Athlete as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:434 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 52 | push | <game_core::Contract as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:2498 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 53 | radius | game_core::Entity::radius | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1509 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 54 | score | game_ai::plan_legacy::sub_plan::SubPlan::score | pub | fn(&mut game_ai::plan_legacy::sub_plan::SubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 | game-ai\src\plan_legacy\sub_plan\mod.rs:164 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 55 | solo_hunt_obs | game_ai::turnback::solo_hunt_obs | pub | fn(&dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , usize, &game_core::Entity) -> game_ai::turnback::SoloHuntObs | game-ai\src\turnback.rs:327 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 56 | start | game_core::prof::start | pub | fn(usize) -> std::option::Option<game_core::prof::ProfTimer> | game-core\src\simulation\prof.rs:175 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 57 | start | game_view::UIPhaseEffect::start | pub | fn(&mut game_view::UIPhaseEffect) | game-view\src\ui\match_ui\phase_effect.rs:30 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 58 | support_min_action_range | game_ai::plan_legacy::sub_plan::support_min_action_range | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-ai\src\plan_legacy\sub_plan\battle.rs:1257 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 59 | team | game_view::ClientData::team | pub | fn(&game_view::ClientData, usize) -> std::option::Option<std::cell::Ref< game_core::Team>> | game-view\src\logic\client\data.rs:1651 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 60 | team | game_view::ClientDatabase::team | pub | fn(&game_view::ClientDatabase, usize) -> std::option::Option<&game_core::Team> | game-view\src\logic\client\data.rs:968 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 61 | team | game_core::PlayerAiContext::<'a, 'b, 'r>::team | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:583 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 62 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 63 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 64 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 65 | to_string | game_core::Position::to_string | pub | fn(&game_core::Position) -> std::string::String | game-core\src\simulation\entity.rs:665 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 4개 중 상위 3개 |
| 66 | to_string | game_core::LineType::to_string | pub | fn(&game_core::LineType) -> std::string::String | game-core\src\simulation\state\player.rs:997 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 4개 중 상위 3개 |
| 67 | to_string | game_core::JungleType::to_string | pub | fn(&game_core::JungleType) -> std::string::String | game-core\src\simulation\entity\jungle.rs:394 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 4개 중 상위 3개 |
| 68 | ult_effect | game_core::Entity::ult_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1700 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 69 | v3_survival_incoming | game_ai::v3_survival_incoming | pub | fn(&game_core::PlayerState, &game_core::OperationData, &game_core::Entity) -> usize | game-ai\src\tower_discipline.rs:367 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
</details>

⚠**미매칭 55개**: `_RNvNtNtCs9ec1k27omRZ_3std2io5stdio6__print`, `block_target_tick`, `candidates`, `champ`, `cleanuppad`, `collect`, `discriminant`, `dist2`, `drop  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `elapsed`, `entry`, `filtered`, `find  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `format_inner`, `from_iter`, `gen_range`, `grow_one`, `handle_error`, `insert_no_grow`, `judge_noise_plan`, `judge_noise_ratio`, `last_bail_gate`, `last_gate`, `move_speed`, `or_insert_with`, `panic`, `parameter`, `pending_global_ult_target`, `pending_trace_events`, `positioning_score`, `readnone`, `reserve_internal_or_panic`, `rustc_entry`, `sl_0`, `so_0`, `sp_0`, `sq_0`, `sqrt`, `ss_0`, `target`, `team_plan`, `to_owned`, `trace_level`, `try_allocate_in`, `try_fold`, `undying`, `v3_bail_goal`, `v3_cand_src`, `v3_final_stand`, `v3_last_stand`, `v3_turnback_hold`, `v3_turnback_scene_prev`, `v48_dodge_flee_picks`, `v48_dodge_step_picks`, `with_ult`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m14.ll:36529) · **형제 41개** (LegacyPlanHandler)

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

**`open` 30건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 재료 부재 | (배치 E) L5·L7·L14·L15·L21·L23~28·L39~49·L53~55·L57·L59·L60·L62·L63·L65·L68·L74·L76~87·L89~91·L95~98·L104~107·L111·L114·L117·L118·L123·L124·L129·L136~138·L142~145·L149~153·L155·L156·L160~164·L170~173·L178(본체)·L179~184(본체)·L187·L188·L195·L197·L198·L200 등 루트 IR 이 0줄인 줄들 = 빈 줄·주석·닫는 괄호·인라인으로 녹은 줄로 추정(e65b10_blocks.json per 에 키 없음) — 소스 부재라 내용 확정 불가(rmeta_srcmap 줄 길이 산술은 미수행 = 미탐색) | 3 |  |
| 1 | 미탐색 | (배치 E) SmallActionRunAway::new(data, player, 5) 의 상수 5 의 의미(도주 발원/모드 코드로 추정) — 콜리 명세(r14 SmallAction*::new 는 목록에 없음) 미열람 | 5 |  |
| 2 | 미탐색 | (배치 E) SmallActionPlay::merge(&mut self, version, &Entity, SmallActionPlay) 의 동작 — 계약만(pre_action 에 후보를 병합해 get_input 으로 실입력을 뽑는 용도로 관측). 자식 명세 목록에 없어 미탐색 | 4 |  |
| 3 | 미탐색 | (배치 E) L34 SoloHuntObs 필드 의미(al120/ally_ctx/en150/chasers/aa_safe/kit_safe/slack) — DWARF 이름만 확보, 값 정의는 turnback::solo_hunt_obs 본체(미열람) | 3 |  |
| 4 | 미탐색 | (배치 E) L70/L194 blackboard 인덱스가 **1-team(적팀 인덱스)** 인 이유 — IR(46426 `gep …, i64 %350`, %350=1-team) 은 확정, 의미(관측 주체 vs 관측 대상 기준)는 Blackboard::is_recent_visible 계약 소관 | 4 |  |
| 5 | 미탐색 | (배치 E) Option<Input> None 태그가 -1 인 인코딩 근거 — tcxdict --enum Input 은 Input 자체만 주고 Option<Input> 니치는 안 준다. IR switch(-1→None, 0→Move) 만을 근거로 적음 | 3 |  |
| 6 | 미탐색 | (배치 E) L38 로그의 `{:?}` 위치 인자 = player.info.position(Position Debug) — 포맷 문자열 @anon.138 의 플레이스홀더 순서(T{} {:?} hold now ex al ctx en ch aa)와 인자 배열 순서(team, position, hold, scene_now, base_exempt, al120, ally_ctx, en150, chasers, aa_safe)로 대응시킴 — 표시 형식은 미검증 | 4 |  |
| 7 | 미탐색 | (배치 E) sub_plan 이 DeathBattle(untagged, 태그 6 자리)일 때 L139 switch 의 `tag>1 ? tag-2 : 6` 경로 — 논리 idx 6 은 어느 case 에도 없어 0 으로 떨어진다(IR 확정). BigPlan L165 의 `≠6 assume` 도 같은 니치 아티팩트 | 4 |  |
| 8 | 미탐색 | (배치 E) L201 pending_trace_events 의 PendingTraceEvent 184B 중 UltEscape 페이로드(+8..+0x21)·tick(+0xb0) 외 바이트(+0x21..+0xb0)는 미기록(undef) — 트레이스 소비 측이 읽지 않는다고 가정하지 않았음(미확인) | 4 |  |
| 9 | 미탐색 | (배치 F) L219 min_by_key 의 동률 규칙(같은 거리 타워가 둘일 때 어느 것을 잡나): core `min_by` 표준(compare==Greater 일 때만 교체 → 앞 원소 유지)으로 추정. 근거는 표준 라이브러리 의미론이며, 실제 fold 본체(m12.ll:16938 → m06.ll:6931 → Copied::fold …)는 3단 위임이라 **IR 로는 미확인**(2회 추적 후 중단). 트레이스 전용 경로라 판정 무영향. | 4 |  |
| 10 | 표기 불가 | (배치 F) L228 closure#10 의 소스 문면 순서(`is_recent_visible && dist<=…` 인지 그 반대인지): IR 은 is_recent_visible 을 무조건 호출(48362)한 뒤 select 로 결합하므로 **표기 불가**(외연 동일 · 호출 부작용 없음). 줄 사슬(L229 거리 · L230 radius · L231 결합)만 확정. | 4 |  |
| 11 | 미탐색 | (배치 F) L235(80자)·L236(113자)의 정확한 문면: IR 루트는 L237(count) 하나에 모여 있고 closure#11 이 236:25 에 있으므로 `enemies.iter().flatten().filter(\|e\| …is_recent_visible…).count()` 로 읽었다 — 줄 분할은 추정. | 4 |  |
| 12 | 미탐색 | (배치 F) L263~266·L268·L283~286·L293~299·L312·L328·L336 = 한글 포함 줄(rmeta srcmap mb>0) 이며 루트 IR 0 → 주석으로 판정(내용은 rmeta 에 원문 없음). | 3 |  |
| 13 | 미탐색 | (배치 F) L303 SubPlan::score 내부의 rnd 소비 수·순서: 콜리 명세 소관(자식 명세 없음 — 「시그니처만」 규칙). 시그니처 = (&SubPlan 72B, version, &ScoreParameter 5384B readonly, &mut StdRng 320B, &PlayerState, &OperationData, &SmallActionPlay 184B readonly, &mut DebugFrameData) -> i64. | 4 |  |
| 14 | 미탐색 | (배치 F) L217 iter_towers_without_nexus(&cache 8840B, team) -> 120B Chain 이터레이터: 어느 타워를 포함하는지(twin_towers·top/mid/bottom tower·tower2 등)는 game_core 소관(시그니처만). | 4 |  |
| 15 | 미탐색 | (배치 F) L256 format anon.139 = "\xC0\x02: \xC0\x00" → `{:?}: {}` 로 읽었다(플레이스홀더 바이트 \xC0 뒤 값은 인자 인덱스/스펙 인코딩 — 정확한 rustc fmt 내부 인코딩은 미검증). 인자 배열은 [SmallAction Debug, &i64 Display] 순(48914~48916). | 4 |  |
| 16 | 미탐색 | (배치 F) L261 println 이 debug 플래그와 무관하게 실행되는지: IR 상 %1177(L260) 은 %688(L254 게이트 거짓 분기)과 %1204(디버그 루프 종료) 양쪽에서 들어오므로 **게이트 없음 확정** — 단 with_score 가 비면 L291 unwrap_failed 로 패닉하므로 실전에서는 후속이 없다(패닉 경로). | 4 |  |
| 17 | 미탐색 | (배치 F) sret 의 정상 경로 최종 쓰기(L343 이후)는 배치 G 소관. F 는 L305 조기 반환만 기술. | 4 |  |
| 18 | 미탐색 | (배치 F) 인라인돼 `call` 이 없는 헬퍼(Entity::distance_sq/radius/can_target/is_champion · SmallActionPlay::get_action · TeamType::eq · SmallAction::eq)는 calls 에 넣지 않고 logic 에 서술(dloc 로 inlinedAt 사슬 확인: entity.rs:2158/1511/1478/1404 · small_action.rs:308~326 · entity.rs:1127). | 4 |  |
| 19 | 표기 불가 | (배치 G) L342~343 의 소스 표기(`match` 한 줄 vs `if let … \| …`) — 표기 불가(동작 확정: Trace/Attack/Skill/Skill2/Ult 만 target_id 추출, 나머지는 L356 직행). column 정보 부재. | 4 |  |
| 20 | 표기 불가 | (배치 G) L344 closure#21 의 `&&` 두 항 소스 순서 — column 부재로 표기 불가. IR 분기 순서는 team 비교 → is_champion(단락) 이며 재현은 이 순서를 따르면 된다. | 4 |  |
| 21 | 표기 불가 | (배치 G) L348 이 `split(\|c\| …).next()` 인지 `find`+슬라이스인지 — 표기 불가(동작 확정: 첫 ' ' 또는 '(' 앞까지, 없으면 전체 · to_string). 이 값은 로그 문자열에만 쓰인다(판정 무영향). | 4 |  |
| 22 | 표기 불가 | (배치 G) L385/L407 의 소스 형태(`if A \|\| (B && C) {…} else {…}` vs 부정형 조기반환) — 표기 불가. 극성은 분기 방향으로 확정: max_score>=0 이면 즉시 갈아타기 경로. | 4 |  |
| 23 | 미탐색 | (배치 G) L407 의 두 번째 항(pre_action 비교·move_near_complete)은 sp_0 가 `score > 0` 을 보장해 max_score >= 0 이 항상 참이므로 **논리상 사장**이다 — 단 reach.py(상수 접기)는 이를 사장으로 못 잡았고(사장 0), IR 에 그대로 남아 있어 NA 봉인은 하지 않았다(판정: 「값 범위 추론으로는 사장, CFG 상수 접기로는 살아있음」). | 4 |  |
| 24 | 미탐색 | (배치 G) support_min_action_range(champ, t) -> u64 의 정확한 계산은 배치 범위 밖(battle.rs:1257~1290 · 261줄): 훑어본 요지 = 기본공격·스킬 1~3 중 CastingTarget::check(t) 통과한 액션의 range_adjust 사거리 최솟값, 없으면 0 — 이 요지는 계약 참고용이며 본 명세의 검증 대상 아님(미탐색 = 상세 분기). | 4 |  |
| 25 | 미탐색 | (배치 G) SmallActionPlay::move_near_complete(&self, champ) -> bool (small_action.rs:413~434 · 훑어봄): 이동형 12 variant 는 dist2(champ.pos, 목표) < 400000001(=20000²+1) · Attack/Skill/Skill2/Ult/Stop 은 false — 계약 참고용(상세 분기·variant 별 목표 오프셋은 미탐색). | 4 |  |
| 26 | 미탐색 | (배치 G) DebugFrameData::add_log(&mut self, &OperationData, &PlayerState, String) 의 쓰기 표면(224B 중 어디)은 미확인 — game_core 콜리(_gcbc) 미열람. | 4 |  |
| 27 | 미탐색 | (배치 G) 반환 튜플 +0x1501~+0x1507 · SmallActionPlay 페이로드 끝~+0xb0 · +0xb2~+0xb7 이 '미기록' 이라는 판정은 IR memcpy(통째 복사)와 alloca 잔재에서 나온 추론 — 런타임 sweep 에서 이 바이트가 갈리면 undef 페이로드로 트리아지할 것. | 4 |  |
| 28 | 미탐색 | (배치 G) 본 범위의 `%691`(L329 게이트 · %1444 → %1474 직행 조건)과 `%1370`/`%513`(드롭플래그)의 정의·의미는 배치 F/E 소관 — 여기서는 진입 엣지로만 기록. | 4 |  |
| 29 | 미탐색 | (배치 G) per 표의 L346(1줄)·L376(1줄)·L388(1줄)·L410(1줄)은 슬라이스에 명령이 없거나 br/phi 뿐(빈 줄·닫는 괄호로 추정) — 로직 없음. | 5 |  |

**`notes` 2건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | (배치 F) L302 SmallActionRunAway::new 의 end_delay=5 의미: 콜리(r15 SmallActionRunAway 계열) 소관 — 여기선 상수 전달만 확정. | 4 | 사실 서술 |
| 1 | (배치 G) sm_0/sq_0 의 max_by_key 동률 시 반환 원소는 마지막 최대 원소(core max_by 규약)지만 본 범위는 점수(.0)만 쓰므로 선택 결과에 무영향 — 원소 자체를 쓰는 코드는 없다(확인). | 4 | 사실 서술 |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

