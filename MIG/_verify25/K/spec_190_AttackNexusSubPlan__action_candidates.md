---

### `190` AttackNexusSubPlan::action_candidates — 넥서스 공격 서브플랜 후보: 위험(위치평가 궤도/논타겟 사거리)이면 도주 단독, 아니면 넥서스 Around+도주+교전+근접 미니언 평타/스킬+소환수+최근접 타워 평타+구조물 스킬을 모두 쌓는다

| 항목 | 값 |
|---|---|
| id | `attack_nexus__AttackNexus__action_candidates` |
| 심볼 | `_RNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy8sub_plan12attack_nexusNtB2_18AttackNexusSubPlan17action_candidates` |
| 소스 | `game-ai\src\plan_legacy\sub_plan\attack_nexus.rs:118` |
| IR | `m14.ll` 18966~20981행 |
| 경로·가시성 | `game_ai::plan_legacy::sub_plan::AttackNexusSubPlan::action_candidates` · **pub** |
| 계층 | 기타 |
| exe | `e81680` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r16` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&mut game_ai::plan_legacy::sub_plan::AttackNexusSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay>
```

<details><summary>인자 8개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | bumpalo::Vec<SmallActionPlay> (32B) | IR %0 `sret([32 x i8]) writeonly`. 레이아웃 ptr@0 · bump@+8 · cap@+0x10 · len@+0x18 (IR L19018~19024: ptr=dangling 8 · bump=data.context.pool · cap=len=0 memset). 원소 184B, 태그 @+0xb1(=177). | 4 |
| 1 | 1 | self | &mut AttackNexusSubPlan (0B ZST) | IR %1 `readnone captures(none)` — 본문에서 단 한 번도 안 읽음/안 씀(tcxdict: 필드 0개·0B). writes 없음. | 3 |
| 2 | 2 | version | usize | IR %2. 본 함수 자체에는 version 분기 없음 — nontarget_windup_perceived·position_score_at_position·SmallActionAround::new·battle_action 에 그대로 전달(L19119·19241·19303·19420). | 4 |
| 3 | 3 | rnd | &mut StdRng (320B) | IR %3 (readonly 없음=가변). 본문 직접 사용 없음 — SmallActionAround::new(L19303)·battle_action(L19420) 에만 전달. | 4 |
| 4 | 4 | player | &PlayerState (2528B) | IR %4 readonly. info.team(+0x930)·info.position(+0x9c0) 읽음. | 4 |
| 5 | 5 | data | &OperationData (24B) | IR %5 readonly. +0 cache(&AbstractGameWithCache) · +8 context(&GameContext) → context+0 = bump pool. | 4 |
| 6 | 6 | parameter | &ScoreParameter (5384B) | IR %6 readonly. +0x9f0 positioning_score(PositioningScoreData) 를 position_score_at_position 에 참조로 넘김(L19236). 그 외 필드 미사용. | 4 |
| 7 | 7 | debug | &mut DebugFrameData (224B) | IR %7 `readnone captures(none)` — 본문 사용 0회. 쓰기 없음. | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn action_candidates(&mut self /*ZST*/, version, rnd, player, data, parameter, debug /*미사용*/) -> Vec<SmallActionPlay> {
  // rs:119
  let bump = data.context.pool;                                  // data+8 → +0
  let mut res = Vec::new_in(bump);                               // sret %37: ptr=8(dangling)·bump·cap=0·len=0
  // rs:122
  let team = player.info.team;  assert!(team < 2);               // panic_bounds_check
  let champ = data.cache.player_champion[team][player.info.position].unwrap();   // None → unwrap_failed(rs:122)
  // rs:125~131  closure#0
  let has_non_target_action_range = data.cache.player_champion[1-team].iter().flatten().any(|c| {
      nontarget_windup_perceived(version, player, data, c)       // 호출 먼저(rs:126)
      && c.ty.tag == 13 /*Champion*/                             // select 로 접힘 — 소스 순서 A&&B 는 컬럼 없음(둘 다 rs:126)
      && {
          let eff: &Effect = match c.action_state.tag {          // c+0x70
              4 /*Skill*/  => match c.skill_effect.casting { -1 => unwrap_failed!, 1|2 => &c.skill_effect, _ => return false },   // rs:127
              5 /*Skill2*/ => { let e = if c.level > 2 { &c.skill2_effect } else { &NONE }; match e.casting { -1 => unwrap_failed!, 1|2 => e, _ => return false } }, // rs:129
              6 /*Ult*/    => { let e = if c.level > 4 { &c.ult_effect }    else { &NONE }; match e.casting { -1 => unwrap_failed!, 1|2 => e, _ => return false } }, // rs:131
              _ => return false,
          };
          Effect::is_in_range(eff, c /*caster*/, champ /*target*/)           // rs:126~131 (L0 귀속)
      }
  });                                                              // 루프: 5칸(8B×5=40) · Some 인 칸만
  // rs:138~140
  let position_score = position_score_at_position(version, player, data, &parameter.positioning_score, champ.x, champ.y, PositionEvalPurpose::General /*tag 2*/);
  if position_score.on_trajectory /*+0x30, 먼저*/ || (has_non_target_action_range || position_score.on_periodic_trajectory /*+0x31*/) {   // 뒤 둘의 상대순서는 `or` 로 접혀 표기 불가(순수 로드라 외연 동일)
      // rs:142~143
      res.push(SmallActionPlay::RunAway(SmallActionRunAway::new_with_skill(data, player, 5, true)));   // tag 3
      return res;
  }
  // rs:147  base_positioning(version, rnd, player, data)  [rs:11~19 인라인]
  {
      let mut r = Vec::new_in(data.context.pool);                       // rs:12
      let nexus = data.cache.nexus[1-team].unwrap();                     // rs:14  None → unwrap_failed
      r.push(SmallActionPlay::Around(SmallActionAround::new(version, rnd, data, player, nexus.id, 5)));   // rs:16  tag 5
      res.extend(r);                                                     // rs:147 (Extend::extend(res, ptr, len))
  }
  // rs:148
  res.push(SmallActionPlay::RunAway(SmallActionRunAway::new(data, player, 5)));                          // tag 3
  // rs:149
  res.extend(battle_action(version, rnd, player, data, 5));
  // rs:150  attack_minion_action(rnd, player, data)  [rs:21~74 인라인]
  {
      let champ = data.cache.player_champion[team][pos].unwrap();       // rs:22 (재로드, None → unwrap_failed)
      let near_enemy_minions = data.cache.iter_minions(1-team).filter(|x| x.distance_sq(champ) < 6400000000 /*80000²*/);   // rs:23 closure#0 (aux)
      let mut r = Vec::new_in(data.context.pool);                       // rs:25
      let move_speed = champ.stat_cached.move_speed;                     // rs:26 (+0x640)
      for target in near_enemy_minions {                                 // rs:28  (find 3단: Chain 슬라이스 3개, 각각 try_fold 심 호출)
          // rs:29  target.is_visible(champ.team) 인라인
          let visible = match champ.team { Neutral => true, Player(t) => { assert!(t<2); target.visible_state[t].tag == 0 /*Visible*/ } };
          if !visible { continue; }
          // rs:33~44 평타
          if champ.can_attack() {
              let atk = champ.attack_effect.as_ref().unwrap();          // rs:34  None → unwrap_failed
              // rs:36~40  (Effect::range(caster) = range + stat_buff_cached.range + (level-1)*growth_range, effect.rs:26 인라인)
              let max_dist = atk.range + move_speed*30 + champ.stat_buff_cached.range + (champ.level-1)*atk.growth_range
                           + atk.range_adjust(champ, target) + champ.radius() + target.radius();       // radius(): mult==0 ? r : r*(100+mult)/100
              let dist_sq = target.distance_sq(champ);                   // rs:37 (|dx|²+|dy|²)
              if dist_sq <= max_dist*max_dist {                          // rs:41  (`ugt` 면 skip)
                  r.push(SmallActionPlay::Attack(SmallActionAttack::new(data, target.id)));     // rs:42  tag 15
              }
          }
          // rs:46~57 스킬
          if let Some(skill) = &champ.skill_effect {                     // casting@tag(+0x4f8) != -1
              if champ.can_skill() && skill.target.check(champ, target) {           // rs:47 (&& 순서: can_skill 먼저 · IR 분기 순)
                  let max_dist = skill.range + move_speed*30 + champ.stat_buff_cached.range + (champ.level-1)*skill.growth_range
                               + skill.range_adjust(champ, target) + champ.radius()   // rs:48
                               + target.radius();                                     // rs:52
                  if target.distance_sq(champ) <= max_dist*max_dist {                // rs:49·53
                      r.push(SmallActionPlay::Skill(SmallActionSkill::new(data, target.id)));   // rs:54  tag 16
                  }
              }
          }
          // rs:59~68 스킬2
          let skill2 = if champ.level > 2 { &champ.skill2_effect } else { &None };     // rs:59 (정적 None @anon.22)
          if let Some(skill2) = skill2 {                                 // +0x30 casting != -1
              if champ.can_skill2() && skill2.target.check(champ, target) {         // rs:60
                  let max_dist = skill2.range + move_speed*30 + (champ.level-1)*skill2.growth_range + champ.stat_buff_cached.range
                               + skill2.range_adjust(champ, target) + champ.radius()   // rs:61
                               + target.radius();                                      // rs:65
                  if target.distance_sq(champ) <= max_dist*max_dist {                 // rs:62·66
                      r.push(SmallActionPlay::Skill2(SmallActionSkill2::new(data, target.id)));  // rs:67  tag 17
                  }
              }
          }
      }
      res.extend(r);                                                     // rs:73→150
  }
  // rs:151
  res.extend(attack_summon_action(player, data));
  // rs:152  attack_tower_action(version, rnd, player, data) -> Option<SmallActionPlay>  [rs:76~100 인라인]
  {
      let opt = (|| {
          let champ = data.cache.player_champion[team][pos]?;            // rs:77  (None → None, 패닉 아님)
          let nearest_enemy_tower = data.cache.iter_towers(1-team)       // rs:78
              .filter(|x| x.can_target())                                // rs:79 closure#0 = entity.rs:1478: can_target(+0x6b9) && block_target_tick(+0x6a0)==0
              .min_by_key(|x| x.distance_sq(champ))?;                    // rs:80 closure#1 (첫 원소 인라인 · 나머지 fold = aux m06.ll; umin·ugt 교체 = 동률 시 앞 원소)
          let hp_ratio = champ.hp * 100 / max(champ.stat_cached.hp, 1); // rs:82
          if hp_ratio < 56 {                                             // rs:83
              if can_tower_focused_when_attack(data.context, data.cache, player, nearest_enemy_tower) { return None; }   // rs:84
          }
          let move_speed = champ.stat_cached.move_speed;                 // rs:88
          let atk = champ.attack_effect.as_ref()?;                       // rs:89  (None → None)
          let dist = nearest_enemy_tower.distance_sq(champ);             // rs:92
          let max_dist = atk.range + champ.stat_buff_cached.range + (champ.level-1)*atk.growth_range   // rs:93 Effect::range
                       + atk.range_adjust(champ, tower) + champ.radius() + tower.radius()
                       + move_speed*30;                                  // rs:94
          if dist > max_dist*max_dist { return None; }                   // rs:95
          Some(SmallActionPlay::Attack(SmallActionAttack::new(data, nearest_enemy_tower.id)))   // rs:99  tag 15
      })();
      res.extend(opt);                                                   // Option::IntoIter — Some 이면 1 push
  }
  // rs:153
  res.extend(attack_structure_skill_action(player, data));
  // rs:155
  res
}
```

**`mem` 메모리 접근 39건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | IR L19023/19025. `< 2` 바운즈체크(L19026) 후 player_champion[team] 인덱스 · 적 팀 = 1-team(L19059) | 4 | OK |  |
| 1 | PlayerState | 0x9c0 | info.position@tag (i32→zext) | r | IR L19043~19044. player_champion[team][pos] 인덱스 | 4 | OK |  |
| 2 | OperationData | 0x0 | cache (&AbstractGameWithCache) | r | IR L19046 | 4 | OK |  |
| 3 | OperationData | 0x8 | context (&GameContext) | r | IR L19014~19015. context 포인터(%39)는 can_tower_focused_when_attack 1번째 인자로도 전달(L20640) | 4 | OK |  |
| 4 | GameContext | 0x0 | pool (&Bump) | r | IR L19016 → Vec::new_in(bump) | 4 | OK |  |
| 5 | AbstractGameWithCache | 0x1e0 | player_champion[2][5] (Option<&Entity>, 8B stride, 팀 stride 40) | r | IR L19048~19051(내 챔프 [team][pos]) · L19061~19096(적 팀 5칸 순회, 8B×5=40 종료판정 L19145) · L19439(attack_minion_action 재로드) · L20254(attack_tower_action 재로드) | 4 | OK |  |
| 6 | AbstractGameWithCache | 0x170 | nexus[2] (Option<&Entity>) | r | IR L19279~19281. nexus[1-team] = 적 넥서스; None 이면 unwrap_failed(L19307) | 4 | OK |  |
| 7 | ScoreParameter | 0x9f0 | positioning_score (PositioningScoreData 2760B) | r | IR L19236. position_score_at_position 4번째 인자(참조) | 4 | OK |  |
| 8 | PositioningScore(sret 56B) | 0x30 | on_trajectory (bool) | r | IR L19245~19247 (+48) | 4 | OK |  |
| 9 | PositioningScore(sret 56B) | 0x31 | on_periodic_trajectory (bool) | r | IR L19248~19250 (+49). ⚠DI 변수명은 `on_trajectory`(line 140 렉시컬 블록 지역변수)지만 tcx 필드는 on_periodic_trajectory — tcx 우선 | 3 | OK |  |
| 10 | Entity | 0x0 | team@tag (TeamType: 0=Player,1=Neutral) | r | IR L19661~19663 (내 챔프). Neutral 이면 가시성 검사 생략(true) | 4 | OK |  |
| 11 | Entity | 0x8 | team@Player.0 (usize) | r | IR L19667. 미니언 visible_state 인덱스 | 4 | OK |  |
| 12 | Entity | 0x38 | visible_state[team]@tag (VisibleState, stride 24) | r | IR L19674~19677 (미니언 target). ==0(Visible) 만 통과 | 4 | OK |  |
| 13 | Entity | 0x68 | ty@tag (EntityType) | r | IR L19123~19125 (적 챔프). ==13 Champion | 4 | OK |  |
| 14 | Entity | 0x70 | ty@Champion.0.action_state@tag (ChampionActionState) | r | IR L19150~19156 (적 챔프). 4=Skill/5=Skill2/6=Ult 만 분기 | 4 | OK |  |
| 15 | Entity | 0x438 | stat_buff_cached.range | r | IR L19488/19713/19897/20071/20688 — Effect::range(caster) 인라인(effect.rs:26) 가산항 | 4 | OK |  |
| 16 | Entity | 0x470 | stat_buff_cached.radius_mult (i32) | r | IR L19489/19725·19746 등 — Entity::radius() 인라인(entity.rs:1511~1515): mult==0 ? radius : radius*(100+mult)/100 | 4 | OK |  |
| 17 | Entity | 0x490 | attack_effect (Option<Effect> 56B) | r | IR L19484(%200). +0x4a0 range · +0x4a8 growth_range · +0x4c0 casting@tag(-1=None → unwrap_failed L19719) | 4 | OK |  |
| 18 | Entity | 0x4a0 | attack_effect.range | r | IR L19710/20681 | 4 | OK |  |
| 19 | Entity | 0x4a8 | attack_effect.growth_range | r | IR L19711/20683 — (level-1)*growth_range | 4 | OK |  |
| 20 | Entity | 0x4c0 | attack_effect@tag (casting, -1=None) | r | IR L19702~19704 / L20635~20637 | 4 | OK |  |
| 21 | Entity | 0x4c8 | skill_effect (Option<Effect>) | r | IR L19495(%211) · 적 챔프 L19177 | 4 | OK |  |
| 22 | Entity | 0x4d8 | skill_effect.range | r | IR L19894 (%214=+1240) | 4 | OK |  |
| 23 | Entity | 0x4e0 | skill_effect.growth_range | r | IR L19895 (%215=+1248) | 4 | OK |  |
| 24 | Entity | 0x4f0 | skill_effect.target (CastingTarget) | r | IR L19887 CastingTarget::check(&target, champ, minion) (%213=+1264) | 4 | OK |  |
| 25 | Entity | 0x4f8 | skill_effect@tag (casting; -1=None, 1=Position, 2=Direction) | r | IR L19696~19698(내 챔프 None 검사) · L19160~19166(적 챔프 casting 1\|2 만 통과, -1 → unwrap_failed) | 4 | OK |  |
| 26 | Entity | 0x500 | skill2_effect (Option<Effect>) — level>2 일 때만, 아니면 정적 None(@anon.22) | r | IR L19874~19876 (내 챔프) · L19183~19187 (적 챔프). +0x510 range · +0x518 growth · +0x528 target · +0x530 casting@tag | 4 | OK |  |
| 27 | Entity | 0x538 | ult_effect (Option<Effect>) — level>4 일 때만, 아니면 정적 None | r | IR L19206~19210 (적 챔프 Ult 상태). +0x568 casting@tag 1\|2 만 통과 | 4 | OK |  |
| 28 | Entity | 0x5c0 | id (usize) | r | IR L19301~19302(넥서스) · L19811~19812/19990~19991/20164~20165(미니언) · L20754~20755(타워) — SmallAction 생성자 target 인자 | 4 | OK |  |
| 29 | Entity | 0x5c8 | level | r | IR L19487/19874/19896 등. skill2 게이트 >2 · 적 ult 게이트 >4 · (level-1)*growth | 4 | OK |  |
| 30 | Entity | 0x628 | stat_cached.hp (max hp) | r | IR L20618~20619 (attack_tower_action rs:82) max(.,1) 분모 | 4 | OK |  |
| 31 | Entity | 0x640 | stat_cached.move_speed | r | IR L19471~19473 (rs:26) · L20630~20631 (rs:88). ×30 | 4 | OK |  |
| 32 | Entity | 0x660 | x | r | IR L19237~19238(내 위치→position_score) · dist_sq 다수 | 4 | OK |  |
| 33 | Entity | 0x668 | y | r | IR L19239~19240 · dist_sq 다수 | 4 | OK |  |
| 34 | Entity | 0x670 | hp | r | IR L20616~20617 (rs:82) hp_ratio 분자 | 4 | OK |  |
| 35 | Entity | 0x680 | radius | r | IR L19490/19731 등 — Entity::radius() 인라인 | 4 | OK |  |
| 36 | Entity | 0x6a0 | block_target_tick | r | IR L20420~20422 · 20532~20534 (타워 필터 rs:79 ← entity.rs:1478 can_target()) ==0 | 4 | OK |  |
| 37 | Entity | 0x6b9 | can_target (bool) | r | IR L20417~20419 · 20529~20531 (타워 필터) — can_target && block_target_tick==0 | 4 | OK |  |
| 38 | sret Vec<SmallActionPlay> | 0x0..0x20 | res | w | IR L19018~19024 초기화 · push 마다 len(+0x18) store(L19417/20216/20890/20976 등) · L20921/20978 에서 %37 → %0 memcpy 32B. ★`&mut self`(%1)·`&mut rnd`(%3)·`&mut debug`(%7) 에 대한 직접 store 는 본문에 0건 — self 는 ZST(readnone), debug 는 readnone. rnd 는 콜리(SmallActionAround::new·battle_action)가 소비. | 4 | 확인불가(tcx 사전에 타입 없음) | ptr/bump/cap/len |

**`consts` 상수 20건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 2 | 122 | 임계 | player.team 바운즈 상한(팀 2). IR L19026 `icmp ult %45, 2` (panic_bounds_check) | 4 |
| 1 | 13 | 126 | 태그 | EntityType 메모리태그 13 = Champion (tcxdict --enum EntityType). 적 챔프 c.ty 가 Champion 이어야 논타겟 사거리 검사 | 3 |
| 2 | 4 | 127 | 태그 | ChampionActionState 태그 4 = Skill (적 챔프가 스킬 시전 중) → skill_effect 사거리 검사 | 4 |
| 3 | 5 | 129 | 태그 | ChampionActionState 태그 5 = Skill2 → level>2 일 때 skill2_effect. ⚠같은 리터럴 5 가 다른 뜻으로도 쓰임: SmallActionRunAway/Around 생성자 end_delay=5(rs:16·142·148) · battle_action 5번째 usize 인자=5(rs:149) · PositionEvalPurpose 태그가 아님 | 4 |
| 4 | 6 | 131 | 태그 | ChampionActionState 태그 6 = Ult → level>4 일 때 ult_effect | 4 |
| 5 | 1 | 127 | 태그 | CastingType 태그 1 = Position (논타겟 위치 시전) — switch case. 또 max(stat_cached.hp,1)(rs:82 · IR L20622)·(level-1)(rs:36/48/61/93) 의 1 | 4 |
| 6 | -1 | 127 | 센티널 | Option<Effect> None 니치(casting@tag = -1 = 0xFFFFFFFF). 적 챔프 케이스는 unwrap_failed(패닉) · 내 챔프 케이스는 '효과 없음' 분기(rs:34·46·59·89) | 4 |
| 7 | 2 | 138 | 태그 | PositionEvalPurpose 메모리태그 2 = General (position_score_at_position 마지막 인자 i8 2 · IR L19241). ⚠같은 리터럴 2 = skill2 레벨 게이트 `level > 2`(rs:59·129) · CastingType Direction(rs:127) | 4 |
| 8 | 4 | 131 | 임계 | 적 ult 레벨 게이트 `level > 4`(IR L19208) — 4 이하면 ult_effect 대신 정적 None(@anon.22) | 4 |
| 9 | 5 | 142 | 태그 | SmallActionRunAway::new_with_skill(data, player, 5, true) 의 end_delay=5 (생성자가 +0x18 에 저장 · m08.ll L92133 판) · rs:148 new(data,player,5) · rs:16 SmallActionAround::new(..., nexus.id, 5) 의 end_delay=5 · rs:149 battle_action(version,rnd,player,data,5) | 4 |
| 10 | 3 | 142 | 태그 | SmallActionPlay 메모리태그 3 = RunAway (store i8 3 @+177 · IR L19372/20931) | 4 |
| 11 | 5 | 16 | 태그 | SmallActionPlay 메모리태그 5 = Around (store i8 5 @+177 · IR L19313) — base_positioning 인라인 | 4 |
| 12 | 15 | 42 | 태그 | SmallActionPlay 메모리태그 15 = Attack (store i8 15 · IR L19818 미니언 / L20761·20841 타워) | 4 |
| 13 | 16 | 54 | 태그 | SmallActionPlay 메모리태그 16 = Skill (store i8 16 · IR L19997) | 4 |
| 14 | 17 | 67 | 태그 | SmallActionPlay 메모리태그 17 = Skill2 (store i8 17 · IR L20171) | 4 |
| 15 | 6400000000 | 23 | 미상 | 80000² — near_enemy_minions 필터: dist_sq(minion, champ) < 80000² (2.5셀). aux m14.ll L59787/61027/61190 (closure call_mut 심) | 4 |
| 16 | 30 | 36 | 계수 | move_speed × 30 — 사거리 판정에 더하는 예측 이동분(30틱 = 0.5초@60tps 추정). IR L19493 `mul %193, 30`(미니언) · L20741(타워 rs:94) | 4 |
| 17 | 100 | 36 | 계수 | Entity::radius() 인라인: radius*(100+radius_mult)/100 (entity.rs:1511~1515). IR L19738~19740 등 | 4 |
| 18 | 56 | 83 | 임계 | attack_tower_action: hp_ratio(=hp*100/max(max_hp,1)) < 56 이면 can_tower_focused_when_attack 검사 → true 면 타워 후보 없음. IR L20626 | 4 |
| 19 | 100 | 82 | 계수 | hp_ratio 백분율 계수 hp*100. IR L20623 | 4 |

**`knobs` 조정점 5건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 근접 미니언 후보 반경 | attack_nexus.rs:23 (aux m14.ll L59787) | 6400000000 | 올리면(80000² 초과) 더 먼 적 미니언까지 평타/스킬 후보에 들어감. 내리면 코앞 미니언만 | 4 | 기존 |
| 1 | 타워 평타 체력 게이트 | attack_nexus.rs:83 (m14.ll L20626) | 56 | hp% 가 이 값 미만이면 can_tower_focused_when_attack 검사에 걸려 타워 평타 후보가 빠질 수 있다. 올리면 더 건강할 때도 타워 포커스 검사(=더 소극적), 내리면 저체력에도 타워 평타 후보 유지 | 4 | 기존 |
| 2 | 사거리 판정 예측 이동 틱 | attack_nexus.rs:36/48/61/94 (m14.ll L19493·20741) | 30 | move_speed×30 을 사거리에 가산. 올리면 더 먼 대상도 '닿는다'고 보고 평타/스킬 후보 생성(과욕), 내리면 보수적 | 4 | 기존 |
| 3 | 스킬2 레벨 게이트 | attack_nexus.rs:59 (m14.ll L19875) | 2 | level>2 여야 skill2 후보. 게임 규칙(스킬2 해금 레벨)과 짝이라 단독 변경은 의미 없음 | 4 | 기존 |
| 4 | 도주/Around end_delay | attack_nexus.rs:16/142/148 (m14.ll L19259·19303·19366) | 5 | SmallActionRunAway/Around 의 end_delay 필드. 의미(틱/프레임 유지시간 추정)는 생성자·get_input 명세 소관 | 5 | 기존 |

<details><summary>`callees` 피호출자 41건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | action_candidates | game_ai::plan_legacy::sub_plan::SubPlan::action_candidates | pub | fn(&mut game_ai::plan_legacy::sub_plan::SubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\mod.rs:118 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 18개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 1 | action_candidates | game_ai::plan_legacy::sub_plan::HideSubPlan::action_candidates | pub | fn(&mut game_ai::plan_legacy::sub_plan::HideSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\hide.rs:19 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 18개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 2 | action_candidates | game_ai::plan_legacy::sub_plan::StealSubPlan::action_candidates | pub | fn(&mut game_ai::plan_legacy::sub_plan::StealSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\steal.rs:33 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 18개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 3 | attack_minion_action | game_ai::plan_legacy::sub_plan::LineSafeSubPlan::attack_minion_action | in:game_ai::plan_legacy::sub_plan::line_safe | fn(&mut game_ai::plan_legacy::sub_plan::LineSafeSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\line_safe.rs:44 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 5개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 4 | attack_minion_action | game_ai::plan_legacy::sub_plan::LineWaitSubPlan::attack_minion_action | in:game_ai::plan_legacy::sub_plan::line_wait | fn(&mut game_ai::plan_legacy::sub_plan::LineWaitSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\line_wait.rs:99 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 5개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 5 | attack_minion_action | game_ai::plan_legacy::sub_plan::LineDefenseSubPlan::attack_minion_action | in:game_ai::plan_legacy::sub_plan::line_defense | fn(&mut game_ai::plan_legacy::sub_plan::LineDefenseSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\line_defense.rs:368 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 5개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 6 | attack_structure_skill_action | game_ai::attack_structure_skill_action | pub | fn(&game_core::PlayerState, &game_core::OperationData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\fight_check.rs:794 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 7 | attack_summon_action | game_ai::attack_summon_action | pub | fn(&game_core::PlayerState, &game_core::OperationData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\fight_check.rs:836 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 8 | attack_tower_action | game_ai::plan_legacy::sub_plan::AttackNexusSubPlan::attack_tower_action | in:game_ai::plan_legacy::sub_plan::attack_nexus | fn(&mut game_ai::plan_legacy::sub_plan::AttackNexusSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> std::option::Option<game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\attack_nexus.rs:76 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 9 | base_positioning | game_ai::plan_legacy::sub_plan::JungleSubPlan::base_positioning | in:game_ai::plan_legacy::sub_plan::jungle | fn(&mut game_ai::plan_legacy::sub_plan::JungleSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> game_ai::SmallActionPlay | game-ai\src\plan_legacy\sub_plan\jungle.rs:19 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 7개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 10 | base_positioning | game_ai::plan_legacy::sub_plan::BattleSubPlan::base_positioning | in:game_ai::plan_legacy::sub_plan::battle | fn(&mut game_ai::plan_legacy::sub_plan::BattleSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\battle.rs:72 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 7개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 11 | base_positioning | game_ai::plan_legacy::sub_plan::LineSafeSubPlan::base_positioning | in:game_ai::plan_legacy::sub_plan::line_safe | fn(&game_ai::plan_legacy::sub_plan::LineSafeSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\line_safe.rs:15 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 7개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 12 | battle_action | game_ai::battle_action | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, usize) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\fight_check.rs:620 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 13 | can_attack | game_core::Entity::can_attack | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1527 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 14 | can_skill | game_core::Entity::can_skill | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1708 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 15 | can_skill2 | game_core::Entity::can_skill2 | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1721 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 16 | can_target | game_core::Entity::can_target | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1477 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
| 17 | can_target | game_core::OperationData::<'a, 'b>::can_target | pub | fn(&game_core::OperationData<'a/#0, 'b/#1>, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\state\player.rs:23 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
| 18 | can_tower_focused_when_attack | game_ai::can_tower_focused_when_attack | pub | fn(&game_core::GameContext, &game_core::AbstractGameWithCache, &game_core::PlayerState, &game_core::Entity) -> bool | game-ai\src\tower_discipline.rs:83 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 19 | check | game_core::CastingTarget::check | pub | fn(&game_core::CastingTarget, &game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\effect\type.rs:227 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 20 | distance_sq | game_core::utils::distance_sq | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:6 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
| 21 | distance_sq | game_core::Entity::distance_sq | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2157 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
| 22 | is_in_range | game_core::Effect::is_in_range | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\effect.rs:63 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 23 | is_visible | game_core::World::is_visible | pub | fn(&game_core::World, usize, usize) -> bool | game-core\src\simulation\game\data.rs:384 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 7개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 24 | is_visible | game_core::VisibleState::is_visible | pub | fn(&game_core::VisibleState) -> bool | game-core\src\simulation\game\data.rs:121 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 7개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 25 | is_visible | game_core::AbstractGame::is_visible | pub | fn(&Self/#0, usize, usize) -> bool | game-core\src\simulation.rs:125 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 7개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 26 | iter_minions | game_core::AbstractGameWithCache::<'a, 'b>::iter_minions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5753 ~ game_core[6a30]::simulation::{impl#5}::iter_minions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1847 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 27 | iter_towers | game_core::AbstractGameWithCache::<'a, 'b>::iter_towers | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5762 ~ game_core[6a30]::simulation::{impl#5}::iter_towers::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1908 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 28 | new | game_ai::SmallActionSkill::new | pub | fn(&game_core::OperationData, usize) -> game_ai::SmallActionSkill | game-ai\src\small_action\cast.rs:119 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 29 | new | game_ai::SmallActionAround::new | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::OperationData, &game_core::PlayerState, usize, usize) -> game_ai::SmallActionAround | game-ai\src\small_action\around.rs:23 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 30 | new | game_ai::SmallActionAttack::new | pub | fn(&game_core::OperationData, usize) -> game_ai::SmallActionAttack | game-ai\src\small_action\cast.rs:23 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 31 | new | game_ai::SmallActionRunAway::new | pub | fn(&game_core::OperationData, &game_core::PlayerState, usize) -> game_ai::SmallActionRunAway | game-ai\src\small_action\move_actions.rs:25 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 32 | new_with_skill | game_ai::SmallActionRunAway::new_with_skill | pub | fn(&game_core::OperationData, &game_core::PlayerState, usize, bool) -> game_ai::SmallActionRunAway | game-ai\src\small_action\move_actions.rs:29 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 33 | nontarget_windup_perceived | game_ai::nontarget_windup_perceived | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity) -> bool | game-ai\src\utils.rs:553 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 34 | position_score_at_position | game_ai::position_score_at_position | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, u64, u64, game_ai::PositionEvalPurpose) -> game_core::PositioningScore | game-ai\src\position_eval.rs:1192 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 35 | push | <game_core::Staff as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::staff | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\staff.rs:38 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 6개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 36 | push | <game_core::Athlete as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:434 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 6개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 37 | push | <game_core::Contract as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:2498 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 6개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 38 | radius | game_core::Entity::radius | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1509 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 39 | range | game_core::Effect::range | pub | fn(&game_core::Effect, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:25 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 40 | range_adjust | game_core::Effect::range_adjust | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:29 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
</details>

⚠**미매칭 3개**: `block_target_tick`, `extend`, `reserve_internal_or_panic`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m12.ll:35264) · **형제 9개** (AttackNexusSubPlan)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::plan_legacy::sub_plan::AttackNexusSubPlan as std::clone::Clone>::clone | pub | game-ai\src\plan_legacy\sub_plan\attack_nexus.rs:5 | True | fn(&game_ai::plan_legacy::sub_plan::AttackNexusSubPlan) -> game_ai::plan_legacy::sub_plan::AttackNexusSubPlan |
| 1 | <game_ai::plan_legacy::sub_plan::AttackNexusSubPlan as std::default::Default>::default | pub | game-ai\src\plan_legacy\sub_plan\attack_nexus.rs:5 | True | fn() -> game_ai::plan_legacy::sub_plan::AttackNexusSubPlan |
| 2 | <game_ai::plan_legacy::sub_plan::AttackNexusSubPlan as std::fmt::Debug>::fmt | pub | game-ai\src\plan_legacy\sub_plan\attack_nexus.rs:5 | True | fn(&game_ai::plan_legacy::sub_plan::AttackNexusSubPlan, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 3 | game_ai::plan_legacy::sub_plan::AttackNexusSubPlan::base_positioning | in:game_ai::plan_legacy::sub_plan::attack_nexus | game-ai\src\plan_legacy\sub_plan\attack_nexus.rs:11 | False | fn(&game_ai::plan_legacy::sub_plan::AttackNexusSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 4 | game_ai::plan_legacy::sub_plan::AttackNexusSubPlan::attack_minion_action | in:game_ai::plan_legacy::sub_plan::attack_nexus | game-ai\src\plan_legacy\sub_plan\attack_nexus.rs:21 | False | fn(&mut game_ai::plan_legacy::sub_plan::AttackNexusSubPlan, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 5 | game_ai::plan_legacy::sub_plan::AttackNexusSubPlan::attack_tower_action | in:game_ai::plan_legacy::sub_plan::attack_nexus | game-ai\src\plan_legacy\sub_plan\attack_nexus.rs:76 | False | fn(&mut game_ai::plan_legacy::sub_plan::AttackNexusSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> std::option::Option<game_ai::SmallActionPlay> |
| 6 | game_ai::plan_legacy::sub_plan::AttackNexusSubPlan::calculate_score_parameter_value | pub | game-ai\src\plan_legacy\sub_plan\attack_nexus.rs:102 | True | fn(&game_ai::plan_legacy::sub_plan::AttackNexusSubPlan, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::ScoreParameter) |
| 7 | game_ai::plan_legacy::sub_plan::AttackNexusSubPlan::action_candidates | pub | game-ai\src\plan_legacy\sub_plan\attack_nexus.rs:118 | False | fn(&mut game_ai::plan_legacy::sub_plan::AttackNexusSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 8 | game_ai::plan_legacy::sub_plan::AttackNexusSubPlan::score | pub | game-ai\src\plan_legacy\sub_plan\attack_nexus.rs:158 | False | fn(&game_ai::plan_legacy::sub_plan::AttackNexusSubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 |

**`open` 8건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 표기 불가 | rs:140 조건식의 소스 표기 순서 — IR 은 `select on_trajectory ? true : (has_non_target_action_range \| on_periodic_trajectory)` 로 접혀 첫 항만 확정. 뒤 둘은 순수 로드라 외연 동일 → 표기 불가(컬럼 정보 없음). | 4 |  |
| 1 | 표기 불가 | rs:126 `nontarget_windup_perceived(...) && c.ty==Champion` 의 소스 순서 — IR 은 호출 후 select 로 접음(호출은 부작용 없음 가정). 외연 동일이라 표기 불가. | 4 |  |
| 2 | 미탐색 | iter_towers 136B 의 array 6칸 + 슬라이스 + Option 구성이 각각 무엇(레인 타워 6 · 추가 타워 · 넥서스?)인지 — game_core 경계. dloc 사슬로 타입만 확정. | 4 |  |
| 3 | 미탐색 | SmallActionRunAway/Around 의 `5`(end_delay) 및 battle_action 5번째 인자 `5` 의 게임적 의미 — 생성자/콜리 명세 소관(자식 명세 별도). | 4 |  |
| 4 | 미탐색 | 30 = 'move_speed × 30' 이 30틱(0.5초) 예측인지 그리드 상수인지 — 본문에 근거 없음(추정만). effect.rs:26 Effect::range 인라인 안에서 곱해지는지 rs:36 에서 곱해지는지는 add 재결합으로 귀속 흔들림(IR L19493 mul 은 ;L 없음). | 4 |  |
| 5 | 미탐색 | 적 챔프 Ult 케이스(rs:131)에서 level<=4 이면 정적 None 을 unwrap → 패닉 경로가 IR 상 존재(L19222). 실제로 도달 불가(레벨 5 미만은 Ult 상태 불가)로 추정 — 런타임 미검증. | 4 |  |
| 6 | 미탐색 | attack_tower_action 반환 Option<SmallActionPlay> 의 None 니치 인코딩(+177 에 i8 -1 dbg) — Option<SmallActionPlay> 태그값(20?) 은 tcxdict 로 미확인(본 함수 출력에는 안 남아 sret live 와 무관). | 3 |  |
| 7 | 미탐색 | SmallActionAround::new 가 range(+0x28)=80000 을 고정 저장(m08.ll L92385 판 관측) — 본 범위 밖 리터럴이라 constants 에서 제외(C1). Around 후보의 실제 접근 반경 노브는 그 생성자 명세 소관. | 4 |  |

**`notes` 1건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | rs:140 렉시컬 블록의 DI 지역변수 `on_trajectory` 가 실제로는 +0x31(tcx: on_periodic_trajectory)에 붙어 있다. `let PositioningScore{on_periodic_trajectory: on_trajectory, ..}` 같은 재바인딩으로 추정 — 동작(두 bool 모두 OR)은 확정, 소스 표기만 미확정. | 3 | 사실 서술 |

<details><summary>`closed` 1건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

| # | 물음 | 닫은 근거 |
|---|---|---|
| 0 | iter_minions 56B 이터레이터의 세 슬라이스가 각각 어느 레인/종류의 미니언 목록인지 — game_core 경계(본 배치 범위 밖). 순회 순서만 슬라이스 A→B→C 로 확정(IR %195→%196→%197). | 본문에 해소 표기가 있다 |
</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

