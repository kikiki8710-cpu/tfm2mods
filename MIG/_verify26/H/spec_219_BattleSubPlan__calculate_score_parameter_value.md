---

### `219` BattleSubPlan::calculate_score_parameter_value — 전투 서브플랜의 ScoreParameter 가중치: 태세(goal)별 자기/적/아군 attack·util_value 기본표(10~100) 세팅 후 전술(tactic)별 TacticModifier 백분율 곱

| 항목 | 값 |
|---|---|
| id | `battle__Battle__calculate_score_parameter_value` |
| 심볼 | `_RNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy8sub_plan6battleNtB2_13BattleSubPlan31calculate_score_parameter_value` |
| 소스 | `game-ai\src\plan_legacy\sub_plan\battle.rs:839` |
| IR | `m02.ll` 36309~37116행 |
| 경로·가시성 | `game_ai::plan_legacy::sub_plan::BattleSubPlan::calculate_score_parameter_value` · **pub** |
| 계층 | 기타 |
| exe | `cc20a0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r17` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_ai::plan_legacy::sub_plan::BattleSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::ScoreParameter)
```

**TLS 메모 절(`sig.tls` · 정정 경로 `/specs[219]/sig/tls/<키>`)**

없음 — 본문·aux 에 `@anon.* = constant ptr @<KEY…call_once>` 참조 0 · LocalKey::with 0

<details><summary>인자 6개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | self | &BattleSubPlan (48B) | IR `readonly dereferenceable(48)` · 읽는 필드 = +0x0/+0x8 support_target(Option<usize>) · +0x10 goal@tag · +0x18 goal.focus · +0x2c tactic@tag | 4 |
| 1 | 2 | version | usize | 본 함수 분기 없음 · v15_can_keep_support_pressure 1번 인자로 통과(m02.ll:36739) | 4 |
| 2 | 3 | _rnd | &mut StdRng (320B) | IR `readnone` — tcx sig 는 &mut 이나 본문 미사용 · gen_range 사이트 0 | 3 |
| 3 | 4 | player | &PlayerState (2528B) | IR `readonly` · info.team(+0x930)·info.position(+0x9c0) 읽음 · v15/any-closure 로 통과 | 4 |
| 4 | 5 | data | &OperationData (24B) | IR `readonly` · +0x0 cache · +0x8 context(aux 에서 is_recent_visible 인자) · +0x10 blackboard(aux) | 4 |
| 5 | 6 | parameter | &mut ScoreParameter (5384B) | IR 속성: `noalias noundef align 8 captures(address, read_provenance) dereferenceable(5384)` — readonly/initializes 없음 = &mut · v15_can_keep_support_pressure 에도 4번 인자로 통과(그쪽은 readonly) | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn calculate_score_parameter_value(&self, version, _rnd, player, data, parameter: &mut ScoreParameter)   [battle.rs:839]
  const trace_non_focus = 30                                                                   [DI 36329]
  match self.goal {                                                                            [L842 · m02.ll:36346 switch]
    Trace{focus} | Assassin{focus} =>                                                          [L843]
      parameter.player.attack_value = 30; .util_value = 30                                     [L844~845]
      for e in near_enemies { v = if goal==Assassin { 60 } else if e.id==focus { 60 } else { trace_non_focus(30) }; e.attack_value=v; e.util_value=v }   [L847~849 · 36439 tag==5 분기 · 36589 select]
      for a in near_allies { a.attack_value=30; a.util_value=30 }                               [L864~866]
    Protect{focus} | Kiting{focus} | KitingBack{focus} =>                                        [L883]
      self 50/50                                                                                [L884~885]
      for e in near_enemies { v = if e.id==focus { 80 } else { 50 }; … }                         [L887~889]
      for a in near_allies { 50/50 }                                                             [L898~900]
    AssassinReady{..} =>                                                                        [L870]
      self 100/100; near_enemies 10/10; near_allies 10/10                                        [L870~880]
    RunAway | End =>                                                                            [L904]
      self 100/100                                                                               [L904~905]
      champ = data.cache.player_champion[player.info.team][player.info.position].unwrap()        [L909 · 36365~36732]
      can_support_fire = v15_can_keep_support_pressure(version, player, data, parameter, self.support_target)   [L910 · 36739]
      near_tower_with_enemy = data.cache.iter_towers_without_nexus(team)                          [L911 · 36742]
          .filter(|t| t.can_target && t.block_target_tick==0 && t.distance²(champ) < 2500000001)   [L912 · aux m06.ll:33073~33116]
          .any(|t| {                                                                              [L913]
              tower_attack_range = t.attack_effect.as_ref().map(|e| e.range(t)).unwrap_or(0)      [L914~916 · 33138~33193 · Effect::range = e.range + t.stat_buff_cached.range + (t.level-1)*e.growth_range + t.radius_adj]
              enemy_team = 1 - player.info.team                                                  [L917 · 33196~33197 · <2 체크]
              cache.player_champion[enemy_team].iter().flatten().any(|c| {                        [L918 · 33204~ (5칸 언롤)]
                  range_with_enemy = c.radius_adj + tower_attack_range                            [L919 · 33260~33294]
                  blackboard[enemy_team].is_recent_visible(cache, context, player, c)             [L920 · 33285]
                    && t.distance²(c) <= range_with_enemy²                                        [L921 · 33301~33324 · `ugt → 다음` 이므로 <=]
              })
          })   // Chain 의 b-half(슬라이스 타워)는 m11.ll try_fold 로 같은 술어 반복(33738)
      enemy_value = if near_tower_with_enemy { 50 } else if can_support_fire { 35 } else { 10 }  [L924 · 36772~36773]
      for e in near_enemies { e.attack_value=enemy_value; e.util_value=enemy_value }              [L927~929]
      for a in near_allies { 100/100 }                                                            [L932~934]
  }
  if self.tactic != BattleTactic::Standard {                                                   [L940 · 36569~36579 switch · Standard → ret]
    modifier = tactic_modifier(self.tactic)     // 인라인 표(fight_model.rs:83~128):           [L941]
      Frontline   : attack 100 · util 100 · self 70  · ally 140 · skills_down None
      BacklineDPS : attack 90  · util 90  · self 130 · ally 100 · runaway_die_tick 150 (L99)
      SkillBurst  : attack 120 · util 100 · self 130 · ally 100 · skills_down Some(60)
      Peel        : attack 70  · util 150 · self 110 · ally 140 · protect_over_flee true (L113)
      AllIn       : attack 130 · util 100 · self 90  · ally 100 · runaway_die_tick 50 (L119)
      Disengage   : attack 100 · util 100 · self 150 · ally 100 · runaway_die_tick 200 (L124)
    enemy_atk_mult = modifier.attack_score_mult
    if let Some(reduced) = modifier.enemy_score_mult_when_skills_down {   // SkillBurst 만        [L945 진입 = tag 3]
      champ = player_champion[team][pos].unwrap()                                                [L945 · 36905~36971]
      usable = champ.can_skill() as usize + champ.can_skill2() as usize + champ.can_ult() as usize   [L946~948 · 36975~36983]
      if usable == 0 { enemy_atk_mult = reduced(60) }  // else 120                               [L949 · 36984~36985]
    }
    for e in near_enemies { e.attack_value = e.attack_value*enemy_atk_mult/100; e.util_value = e.util_value*modifier.util_score_mult/100 }   [L955~957 · 37035~37044 sdiv]
    parameter.player.attack_value = …*modifier.self_score_mult/100; .util_value = …*self_score_mult/100   [L961~962 · 37055~37064]
    for a in near_allies { a.attack_value = …*ally_score_mult/100; a.util_value = …*ally_score_mult/100 }   [L963~965 · 37098~37107]
  }
  return                                                                                        [L968 · 36873]

주의: runaway_die_tick_mult/min_die_tick_for_trace/force_protect/prefer_frontline/protect_over_flee 는 본 함수에서 소비하지 않는다(dbg_value 로만 보임). 곱은 `mul`(nsw 없음)+`sdiv 100` — 음수 값도 부호 유지. gen_range 사이트 0. ScoreParameter 의 다른 필드(wave_snapshot·positioning_score·risk_*·version·v3_turnback_hold)는 본 함수가 읽지도 쓰지도 않는다(v15 콜리 내부는 계약 밖).
```

**`mem` 메모리 접근 37건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | BattleSubPlan | 0x10 | goal@tag (BattleSubPlanGoal · 0 Trace/1 Protect/2 Kiting/3 KitingBack/4 RunAway/5 Assassin/6 AssassinReady/7 End) | r | m02.ll:36343~36355 switch [L842] · 다시 36439 `==5`(Assassin 분기) | 4 | OK |  |
| 1 | BattleSubPlan | 0x18 | goal.focus (usize · Trace/Protect/Kiting/KitingBack/Assassin 페이로드) | r | m02.ll:36404 [L843] · 36462 [L883] — 적 원소 id 와 비교 | 4 | OK |  |
| 2 | BattleSubPlan | 0x0 | support_target@tag (Option<usize>) | r | m02.ll:36736 [L910] · v15_can_keep_support_pressure 5번 인자 | 4 | OK |  |
| 3 | BattleSubPlan | 0x8 | support_target@Some.0 | r | m02.ll:36737~36738 [L910] · 6번 인자 | 4 | OK |  |
| 4 | BattleSubPlan | 0x2c | tactic@tag (BattleTactic 1B · 0 Standard/1 Frontline/2 BacklineDPS/3 SkillBurst/4 Peel/5 AllIn/6 Disengage) | r | m02.ll:36569~36579 switch [L940 `!= Standard` 인라인(cmp.rs:264 ne) + L941 tactic_modifier 인라인] | 4 | OK |  |
| 5 | PlayerState | 0x930 | info.team | r | m02.ll:36365~36368 [L909] · 36905~36908 [L945] · `<2` 바운드체크 | 4 | OK |  |
| 6 | PlayerState | 0x9c0 | info.position@tag (i32) | r | m02.ll:36722~36724 [L909] · 36961~36963 [L945] · player.rs:581 인라인 | 4 | OK |  |
| 7 | OperationData | 0x0 | cache | r | m02.ll:36725 [L909] · 36964 [L945] | 4 | OK |  |
| 8 | OperationData | 0x10 | blackboard (&[Blackboard;2]) | r | m02.ll:36747~36748 [L913] · any 클로저 캡처 → aux 에서 blackboard[enemy_team].is_recent_visible | 4 | OK |  |
| 9 | AbstractGameWithCache | 0x1e0 | player_champion[team][pos] (Option<&Entity> null=None) | r | m02.ll:36726~36732 [L909] · 36965~36971 [L945] · None → unwrap_failed · aux 에서는 [enemy_team][0..5] 전수 순회(m06.ll:33204~33238 등) | 4 | OK |  |
| 10 | ScoreParameter | 0x14d8 | near_enemies.buf.ptr | r | m02.ll:36377~36378 등 [L847/873/887/927/955] | 4 | OK |  |
| 11 | ScoreParameter | 0x14f0 | near_enemies.len | r | m02.ll:36380~36381 등 · 원소 stride 216(ChampionScoreParameter) | 4 | OK |  |
| 12 | ScoreParameter | 0x14b8 | near_allies.buf.ptr | r | m02.ll:36522~36523 등 [L864/878/898/932/963] | 4 | OK |  |
| 13 | ScoreParameter | 0x14d0 | near_allies.len | r | m02.ll:36525~36526 등 | 4 | OK |  |
| 14 | ChampionScoreParameter (near_enemies[i]) | 0x58 | id | r | m02.ll:36501~36503 [L889] · 36586~36588 [L849] — `== focus` | 4 | OK |  |
| 15 | ChampionScoreParameter (near_enemies[i] / near_allies[i]) | 0xa8 | attack_value (i64) | r | 전술 곱 단계에서 read-modify-write: m02.ll:37035~37039 [L956] · 37098~37102 [L964] | 4 | OK |  |
| 16 | ChampionScoreParameter (near_enemies[i] / near_allies[i]) | 0xb0 | util_value (i64) | r | m02.ll:37040~37044 [L957] · 37103~37107 [L965] | 4 | OK |  |
| 17 | ScoreParameter | 0x9c0 | player.attack_value | r | 전술 곱 단계 RMW m02.ll:37055~37059 [L961] | 4 | OK |  |
| 18 | ScoreParameter | 0x9c8 | player.util_value | r | m02.ll:37060~37064 [L962] | 4 | OK |  |
| 19 | Entity (aux · 타워 t) | 0x6b9 | can_target (bool) | r | m06.ll:33073~33075 [L912 filter] | 4 | OK |  |
| 20 | Entity (aux · 타워 t) | 0x6a0 | block_target_tick (usize) | r | m06.ll:33076~33079 [L912] · ==0 요구 | 4 | OK |  |
| 21 | Entity (aux) | 0x660 | x | r | m06.ll:33086~33087(타워) · 33094~33095(내 챔프) · 33301~33302(적 챔프) — Entity::distance(entity.rs:2158) 인라인 dx=\|x1-x2\| | 4 | OK |  |
| 22 | Entity (aux) | 0x668 | y | r | m06.ll:33090~33091 · 33098~33099 · 33305~33306 — dy | 4 | OK |  |
| 23 | Entity (aux · 타워 t) | 0x4c0 | attack_effect@tag (니치 i32 · -1=None) | r | m06.ll:33138~33141 [L914 `t.attack_effect.as_ref()`] · None → tower_attack_range=0 | 4 | OK |  |
| 24 | Entity (aux · 타워 t) | 0x4a0 | attack_effect.range (u64) | r | m06.ll:33147~33148 [L915 → Effect::range effect.rs:26 인라인] | 4 | OK |  |
| 25 | Entity (aux · 타워 t) | 0x4a8 | attack_effect.growth_range (u64) | r | m06.ll:33149~33150 · ×(level-1) | 4 | OK |  |
| 26 | Entity (aux · 타워 t) | 0x5c8 | level (usize) | r | m06.ll:33156~33157 · `level-1` (33183) | 4 | OK |  |
| 27 | Entity (aux · 타워 t) | 0x438 | stat_buff_cached.range (usize) | r | m06.ll:33158~33159 · 사거리 가산 | 4 | OK |  |
| 28 | Entity (aux · 타워 t / 적 챔프 c) | 0x470 | stat_buff_cached.radius_mult (i32) | r | m06.ll:33160~33163 (타워 · entity.rs:1511~1515 인라인) · 33260~33263 (적 챔프 [L919]) | 4 | OK |  |
| 29 | Entity (aux · 타워 t / 적 챔프 c) | 0x680 | radius (usize) | r | m06.ll:33167~33168 / 33174~33178 (타워) · 33267~33278 (적 챔프) — mult==0 ? radius : radius*(100+mult)/100 (udiv) | 4 | OK |  |
| 30 | ScoreParameter | 0x9c0 | player.attack_value | w | m02.ll:36407 [L844] · 36465 [L884] · 36372 [L870] · 36362 [L904] · 이후 tactic!=Standard 이면 ×self_score_mult/100 (37059 [L961]) | 4 | OK | goal 별 기본값: Trace/Assassin=30 · Protect/Kiting/KitingBack=50 · AssassinReady=100 · RunAway/End=100 |
| 31 | ScoreParameter | 0x9c8 | player.util_value | w | m02.ll:36409 [L845] · 36467 [L885] · 36374 [L871] · 36364 [L905] · 37064 [L962] | 4 | OK | attack_value 와 동일 기본값 · ×self_score_mult/100 |
| 32 | ChampionScoreParameter (near_enemies[i] 힙 원소 · i=0..len) | 0xa8 | attack_value | w | m02.ll:36448 (Assassin · L0) · 36591 [L849 · L0] · 36506 [L889 · L0] · 36657 [L874] · 36813 [L928] · 이후 ×attack_score_mult(또는 reduced)/100 sdiv (37039 [L956]) | 4 | OK | Trace: id==focus ? 60 : 30 · Assassin: 60 · Protect/Kiting/KitingBack: id==focus ? 80 : 50 · AssassinReady: 10 · RunAway/End: enemy_value(50/35/10) |
| 33 | ChampionScoreParameter (near_enemies[i] 힙 원소) | 0xb0 | util_value | w | m02.ll:36450 · 36593 · 36508 · 36659 [L875] · 36815 [L929] · 37044 [L957] | 4 | OK | attack_value 와 같은 기본값 · ×util_score_mult/100 |
| 34 | ChampionScoreParameter (near_allies[i] 힙 원소 · i=0..len) | 0xa8 | attack_value | w | m02.ll:36639 [L865] · 36554 [L899] · 36704 [L879] · 36860 [L933] · 37102 [L964] | 4 | OK | Trace/Assassin=30 · Protect/Kiting/KitingBack=50 · AssassinReady=10 · RunAway/End=100 · ×ally_score_mult/100 |
| 35 | ChampionScoreParameter (near_allies[i] 힙 원소) | 0xb0 | util_value | w | m02.ll:36641 [L866] · 36556 [L900] · 36706 [L880] · 36862 [L934] · 37107 [L965] | 4 | OK | attack_value 와 동일 · ×ally_score_mult/100 |
| 36 | (스택) Filter<Chain<…>, closure#0> 128B | 0x78 | predicate 캡처 = &champ | w | m02.ll:36744~36745 [L912] · 지역 alloca %8 — 힙/인자 부작용 아님(참고용) | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) | champ ptr |

**`consts` 상수 26건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 30 | 844 | 산출값 | Trace/Assassin 자기 attack/util_value (m02.ll:36407, 36409) · DI 상수 `trace_non_focus = 30`(36329) = Trace 비-focus 적 값 (36589 select 거짓 가지 [L849]) · 아군 30 (36639/36641 [L865/866]) | 4 |
| 1 | 60 | 849 | 산출값 | Trace 에서 id==focus 인 적 / Assassin 에서 모든 적의 attack/util_value (m02.ll:36589 select 참 가지 · 36448/36450 Assassin 루프) | 4 |
| 2 | 5 | 842 | 태그 | goal 태그 5 = Assassin — 적 전원 60 분기 (m02.ll:36439 `icmp eq %10, 5`) | 4 |
| 3 | 50 | 884 | 산출값 | Protect/Kiting/KitingBack 자기 attack/util_value (36465/36467) · 비-focus 적 50 (36504 select 거짓 [L889]) · 아군 50 (36554/36556 [L899/900]) · RunAway/End 에서 near_tower_with_enemy 이면 적 50 (36773 [L924]) | 4 |
| 4 | 80 | 889 | 산출값 | Protect/Kiting/KitingBack 에서 id==focus 인 적의 attack/util_value (m02.ll:36504 select 참 가지) | 4 |
| 5 | 100 | 870 | 계수 | AssassinReady 자기 100 (36372/36374) · RunAway/End 자기 100 (36362/36364 [L904/905]) · RunAway/End 아군 100 (36860/36862 [L933/934]) | 4 |
| 6 | 10 | 874 | 산출값 | AssassinReady 적 10 (36657/36659) · 아군 10 (36704/36706 [L879/880]) · RunAway/End 에서 타워근접적 없고 can_support_fire 거짓이면 적 10 (36772 select 거짓 [L924]) | 4 |
| 7 | 35 | 924 | 산출값 | RunAway/End 에서 near_tower_with_enemy 거짓이고 can_support_fire(v15_can_keep_support_pressure) 참이면 적 attack/util_value (m02.ll:36772 select 참 가지) | 4 |
| 8 | 2 | 909 | 임계 | player_champion 1차 team<2 바운드체크 (36367 · 36907 [L945]) · aux enemy_team<2 (m06.ll:33199) | 4 |
| 9 | 2500000001 | 912 | 미상 | 50000²+1 — 타워↔내 챔프 거리² < 이 값 ⟺ 거리 ≤ 50000(=1.5625셀) 인 타워만 후보 (m06.ll:33115 `icmp ult` · m11.ll 동일) | 4 |
| 10 | 0 | 912 | 태그 | 타워 block_target_tick == 0 요구 (m06.ll:33078) · L914 attack_effect None 이면 tower_attack_range=0 (33193 phi) · L949 usable==0 (36984) | 4 |
| 11 | -1 | 914 | 센티널 | Option<Effect> 니치 None 판별(attack_effect 태그 i32 -1) (m06.ll:33140) · Chain a-half 소진 마킹 store -1 (33729) | 4 |
| 12 | 100 | 915 | 계수 | Entity 반경 보정 radius*(100+radius_mult)/100 (m06.ll:33176~33178 타워 · 33276~33278 적 챔프 [L919] · entity.rs:1515 인라인) | 4 |
| 13 | 1 | 915 | 태그 | Effect::range(effect.rs:26) 인라인 `(level-1)*growth_range` — `add i64 %73, -1`(m06.ll:33183) 로 접힘 · 성장 사거리 계수 | 4 |
| 14 | 1 | 917 | 태그 | enemy_team = 1 - player.info.team (m06.ll:33197 `sub i64 1, %98`) | 4 |
| 15 | 0 | 940 | 태그 | tactic 태그 0 = Standard → 전술 곱 단계 전체 스킵·즉시 return (m02.ll:36572 switch case 0 → %183 ret) | 4 |
| 16 | 120 | 941 | 산출값 | tactic_modifier(SkillBurst).attack_score_mult (m02.ll:36985 select 거짓 가지 · 36895 dbg) — 스킬/궁 하나라도 사용 가능하면 적 attack_value ×120/100 | 4 |
| 17 | 60 | 949 | 산출값 | tactic_modifier(SkillBurst).enemy_score_mult_when_skills_down = Some(60) (`reduced` 36903~36904) — can_skill+can_skill2+can_ult == 0 이면 적 attack_value ×60/100 (36985 select 참 가지) | 4 |
| 18 | 90 | 941 | 산출값 | BacklineDPS: attack_score_mult=90 · util_score_mult=90 (36993~36994 phi from %184) · AllIn: self_score_mult=90 (36995 from %190) | 4 |
| 19 | 130 | 941 | 산출값 | BacklineDPS/SkillBurst: self_score_mult=130 (36995) · AllIn: attack_score_mult=130 (36993 from %190) | 4 |
| 20 | 70 | 941 | 산출값 | Frontline: self_score_mult=70 (36995 phi from %83) · Peel: attack_score_mult=70 (36993 from %189) | 4 |
| 21 | 140 | 941 | 산출값 | Frontline/Peel: ally_score_mult=140 (36996 phi from %83, %189) | 4 |
| 22 | 150 | 941 | 산출값 | Peel: util_score_mult=150 (36994 from %189) · Disengage: self_score_mult=150 (36995 from %191) · BacklineDPS runaway_die_tick_mult=150 (36876 dbg · 본 함수 미소비) | 4 |
| 23 | 110 | 941 | 산출값 | Peel: self_score_mult=110 (36995 from %189) | 4 |
| 24 | 100 | 956 | 계수 | 백분율 제수 — 적/자기/아군 attack·util_value = value*mult/100 (sdiv · m02.ll:37038, 37043, 37058, 37063, 37101, 37106) · 각 tactic 의 mult 기본값 100(Frontline attack/util · Disengage attack/util · SkillBurst util · … 36993~36996 phi) | 4 |
| 25 | 6 | 911 | 태그 | iter_towers_without_nexus 배열 절반 = [Option<&Entity>; 6] (m06.ll:32992, 33045) — 원소 수, 판정값 아님 | 4 |

**`knobs` 조정점 9건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | Trace 비-focus 적 가치 | battle.rs:849 (trace_non_focus 상수 = 844 부근) | 30 | 올리면 추격 태세에서 focus 외 적에게도 공격/CC 가치가 생겨 표적이 분산된다 | 4 | 기존 |
| 1 | Trace focus 적 / Assassin 전체 적 가치 | battle.rs:849 | 60 | 올리면 추격·암살 태세에서 적 챔피언 타격 후보의 점수가 커진다(자기 30 대비 상대 비중↑) | 4 | 기존 |
| 2 | Protect/Kiting/KitingBack focus 적 가치 | battle.rs:889 | 80 | 올리면 카이팅·보호 중에도 focus 타격을 더 우선한다 | 4 | 기존 |
| 3 | Protect/Kiting/KitingBack 자기·아군·비-focus 적 가치 | battle.rs:884~900 | 50 | 자기 값을 올리면 카이팅 중 자기 손실 회피가 강해진다 | 4 | 기존 |
| 4 | AssassinReady 적·아군 가치 | battle.rs:874~880 | 10 | 올리면 암살 준비 단계에서 미리 교전에 끼어들 여지가 커진다(자기 100 대비) | 4 | 기존 |
| 5 | RunAway/End 적 가치 3단 | battle.rs:924 | 50 / 35 / 10 | 타워 근접 적 존재·지원 사격 가능·그 외 순 — 올리면 도주 중에도 반격 후보(공격/스킬)가 살아남는다 | 4 | 기존 |
| 6 | 타워 근접 판정 반경 | battle.rs:912 | 2500000001 | 50000²+1 — 키우면 더 먼 아군 타워도 '내 타워' 후보로 봐서 도주 중 적 가치 50 이 더 자주 적용된다 | 4 | 기존 |
| 7 | SkillBurst 스킬 다운 시 적 가치 배율 | fight_model.rs:83~128 (tactic_modifier · SkillBurst) / 소비 battle.rs:949 | 60 | 올리면 스킬·궁이 전부 쿨일 때도 적 타격 가치가 덜 깎인다 | 4 | 기존 |
| 8 | 전술별 배율표(attack/util/self/ally %) | fight_model.rs:83~128 (tactic_modifier 인라인) | Frontline 100/100/70/140 · BacklineDPS 90/90/130/100 · SkillBurst 120/100/130/100 · Peel 70/150/110/140 · AllIn 130/100/90/100 · Disengage 100/100/150/100 | self 를 올리면 자기 생존 가중↑ · ally 를 올리면 아군 보호↑ · attack/util 을 올리면 적 타격/CC 가중↑ (Standard 는 곱 자체를 건너뜀 = 100%) | 4 | 기존 |

<details><summary>`callees` 피호출자 11건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | calculate_score_parameter_value | game_ai::plan_legacy::sub_plan::SubPlan::calculate_score_parameter_value | pub | fn(&mut game_ai::plan_legacy::sub_plan::SubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::ScoreParameter) | game-ai\src\plan_legacy\sub_plan\mod.rs:96 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 18개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 1 | calculate_score_parameter_value | game_ai::plan_legacy::sub_plan::HideSubPlan::calculate_score_parameter_value | pub | fn(&game_ai::plan_legacy::sub_plan::HideSubPlan, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::ScoreParameter) | game-ai\src\plan_legacy\sub_plan\hide.rs:221 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 18개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 2 | calculate_score_parameter_value | game_ai::plan_legacy::sub_plan::StealSubPlan::calculate_score_parameter_value | pub | fn(&game_ai::plan_legacy::sub_plan::StealSubPlan, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::ScoreParameter) | game-ai\src\plan_legacy\sub_plan\steal.rs:122 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 18개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 3 | can_skill | game_core::Entity::can_skill | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1708 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | can_skill2 | game_core::Entity::can_skill2 | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1721 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 5 | can_ult | game_core::Entity::can_ult | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1734 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 6 | is_recent_visible | game_core::Blackboard::is_recent_visible | pub | fn(&game_core::Blackboard, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\game\blackboard.rs:346 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 7 | iter_towers_without_nexus | game_core::AbstractGameWithCache::<'a, 'b>::iter_towers_without_nexus | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5750 ~ game_core[6a30]::simulation::{impl#5}::iter_towers_without_nexus::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1830 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 8 | range | game_core::Effect::range | pub | fn(&game_core::Effect, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:25 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 9 | tactic_modifier | game_ai::plan_legacy::old::tactic_modifier | pub | fn(game_ai::plan_legacy::old::BattleTactic) -> game_ai::plan_legacy::old::TacticModifier | game-ai\src\plan_legacy\old\fight_model.rs:83 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 10 | v15_can_keep_support_pressure | game_ai::plan_legacy::sub_plan::battle_common::v15_can_keep_support_pressure | in:game_ai | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, std::option::Option<usize>) -> bool | game-ai\src\plan_legacy\sub_plan\battle_common.rs:7 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
</details>

⚠**미매칭 4개**: `half`, `reduced`, `trace_non_focus`, `try_fold`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m12.ll:35836) · **형제 8개** (BattleSubPlan)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::plan_legacy::sub_plan::BattleSubPlan as std::clone::Clone>::clone | pub | game-ai\src\plan_legacy\sub_plan\battle.rs:7 | True | fn(&game_ai::plan_legacy::sub_plan::BattleSubPlan) -> game_ai::plan_legacy::sub_plan::BattleSubPlan |
| 1 | <game_ai::plan_legacy::sub_plan::BattleSubPlan as std::fmt::Debug>::fmt | pub | game-ai\src\plan_legacy\sub_plan\battle.rs:7 | True | fn(&game_ai::plan_legacy::sub_plan::BattleSubPlan, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 2 | game_ai::plan_legacy::sub_plan::BattleSubPlan::new | pub | game-ai\src\plan_legacy\sub_plan\battle.rs:46 | True | fn(game_ai::plan_legacy::old::BattleSubPlanGoal, std::option::Option<usize>, game_ai::plan_legacy::old::BattleTactic, bool, usize, bool) -> game_ai::plan_legacy::sub_plan::BattleSubPlan |
| 3 | game_ai::plan_legacy::sub_plan::BattleSubPlan::is_dive_local | pub | game-ai\src\plan_legacy\sub_plan\battle.rs:68 | True | fn(&game_ai::plan_legacy::sub_plan::BattleSubPlan) -> bool |
| 4 | game_ai::plan_legacy::sub_plan::BattleSubPlan::base_positioning | in:game_ai::plan_legacy::sub_plan::battle | game-ai\src\plan_legacy\sub_plan\battle.rs:72 | False | fn(&mut game_ai::plan_legacy::sub_plan::BattleSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 5 | game_ai::plan_legacy::sub_plan::BattleSubPlan::action_candidates | pub | game-ai\src\plan_legacy\sub_plan\battle.rs:358 | False | fn(&mut game_ai::plan_legacy::sub_plan::BattleSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 6 | game_ai::plan_legacy::sub_plan::BattleSubPlan::score | pub | game-ai\src\plan_legacy\sub_plan\battle.rs:616 | False | fn(&game_ai::plan_legacy::sub_plan::BattleSubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 |
| 7 | game_ai::plan_legacy::sub_plan::BattleSubPlan::calculate_score_parameter_value | pub | game-ai\src\plan_legacy\sub_plan\battle.rs:839 | False | fn(&game_ai::plan_legacy::sub_plan::BattleSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::ScoreParameter) |

**`open` 7건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | tactic_modifier 의 Frontline·SkillBurst 항목 정확한 소스 줄 — dbg 가 line 0 이라 미확정(BacklineDPS=99·Peel=113·AllIn=119·Disengage=124 는 br 의 !dbg 로 확정) · fight_model.rs:83~128 범위 내 | 4 |  |
| 1 | 표기 불가 | L849 소스가 `goal==Assassin \|\| e.id==focus` 인지 별도 if 인지 — 표기 불가(외연 동일 · 컴파일러가 tag==5 로 루프를 분리) | 4 |  |
| 2 | 미탐색 | v15_can_keep_support_pressure 내부(정본 r13~r14 잎 계약) — (version, player, data, parameter, support_target: Option<usize>) -> bool 만 | 3 |  |
| 3 | 미탐색 | iter_towers_without_nexus(cache, team) 내부 — sret 120B = Chain<Flatten<IntoIter<Option<&Entity>,6>>, Copied<slice::Iter<&Entity>>> 만(game_core 경계 · 어느 타워가 어느 절반에 들어가는지 미탐색) | 4 |  |
| 4 | 미탐색 | Blackboard::is_recent_visible(&self, cache, context, player, entity) -> bool 내부(game_core g07.ll:157005) — 계약만 | 4 |  |
| 5 | 표기 불가 | L946~948 `usable` 이 소스에서 튜플인지 합인지 — DIArgList 2원소 · `(a+b)==0`→`or` 접힘이라 합으로 읽음(표기 불가) | 4 |  |
| 6 | 미탐색 | near_enemies 가 focus 를 포함하지 않을 때(예: focus 가 시야 밖)의 동작은 콜러(ScoreParameter 구성 계층) 범위 밖 | 4 |  |

**`notes` 1건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | Trace/Assassin·Protect 계열의 적 원소 store 가 소스 줄 0(L0) — 헬퍼 클로저/매크로로 추정(36447~36450 · 36505~36508 · 36590~36593) · 소스 표기 미확정 | 5 | 사실 서술 |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

