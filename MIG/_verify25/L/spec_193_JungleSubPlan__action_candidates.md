---

### `193` JungleSubPlan::action_candidates — 정글 캠프(self.camp/team) 사냥 서브플랜 후보 — 위험이면 도주 단일, 아니면 전투 후보 + 캠프 몹 공격/스킬/스킬2 후보(attack_jungle_action 인라인) + 캠프 이동(move_action 인라인)

| 항목 | 값 |
|---|---|
| id | `jungle__Jungle__action_candidates` |
| 심볼 | `_RNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy8sub_plan6jungleNtB2_13JungleSubPlan17action_candidates` |
| 소스 | `game-ai\src\plan_legacy\sub_plan\jungle.rs:107` |
| IR | `m02.ll` 38405~39900행 |
| 경로·가시성 | `game_ai::plan_legacy::sub_plan::JungleSubPlan::action_candidates` · **pub** |
| 계층 | 기타 |
| exe | `cc4260` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r16` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&mut game_ai::plan_legacy::sub_plan::JungleSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay>
```

**TLS 메모 절(`sig.tls` · 정정 경로 `/specs[193]/sig/tls/<키>`)**

- `name`: (직접 접점 없음) — 콜리 경유만
- `role`: 소비자(간접)
- `key`: 본문 LocalKey::with 0건. 콜리 순서: ①nontarget_windup_perceived(L115, 적 챔프마다) ②position_score_at_position(L127, purpose=Objective(11)) → position_eval 계열 캐시 ③battle_action(L136, 무조건) → interaction_score/INTER_CTX·HP_VALUE_MEMO 등 ④attack_summon_action(L137) ⑤MapDef::camp_pos(L140 · L20/L31/L40<148) → game_core CAMP_POS_MEMO ⑥JungleRunner::get_camp_state(L56<147) ⑦Effect::range_adjust / CastingTarget::check(몹마다)
- `layout`: 해당 없음
- `invalidation`: 해당 없음
- `call_conditions`: ②는 위험 판정 전 무조건 1회. ③④는 도주 단일(L131) 을 안 탄 경우 무조건(serpen_check 와 달리 is_visible 게이트 없음). ⑤ L140 은 무조건 1회, L20<148 도 무조건 1회, L31/L40<148 은 적 캠프이고 check_move 미설정이며 적 진영일 때만

<details><summary>인자 7개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | bumpalo::Vec<SmallActionPlay> (32B) | writeonly sret. +0 ptr · +8 bump(=data.context.pool) · +0x10 cap · +0x18 len. 원소 184B | 4 |
| 1 | 1 | self | &mut JungleSubPlan (16B) | noalias captures(none), readonly 없음 = &mut. team:usize@0 · camp:JungleType@+8 · check_move:bool@+9. 쓰기 = check_move 1곳(writes) | 4 |
| 2 | 2 | version | usize | 본문 분기 없음. nontarget_windup_perceived · position_score_at_position · battle_action(_version) 에 전달 | 4 |
| 3 | 3 | rnd | &mut StdRng (320B) | 본문 직접 읽기 없음. battle_action(_rnd) · AroundPosition::new / new_with_out_line 에 전달 | 4 |
| 4 | 4 | player | &PlayerState (2528B) | readonly | 4 |
| 5 | 5 | data | &OperationData (24B) | cache@0 · context@8. blackboard(+0x10)는 이 함수에서 읽지 않음 | 4 |
| 6 | 6 | parameter | &ScoreParameter (5384B) | +0x9f0 positioning_score 주소만 position_score_at_position 에 전달 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn action_candidates(&mut self, version, rnd, player, data, parameter) -> Vec<SmallActionPlay>
L108 bump = data.context.pool; res = Vec::new_in(bump)
L111 team = player.info.team (bounds<2); champ = cache.player_champion[team][player.info.position].unwrap()
L114 enemy_team = 1 - team
     has_non_target_action_range = cache.player_champion[enemy_team].iter().flatten().any(|c|
L115   nontarget_windup_perceived(version, player, data, c) && c.ty==Champion(13) &&
L116   match c.action_state { Skill(4) => e = c.skill_effect.unwrap()(casting==-1 → unwrap_failed) ,
L118                          Skill2(5) => e = (if c.level>2 {c.skill2_effect} else {None}).unwrap() ,
L120                          Ult(6)    => e = (if c.level>4 {c.ult_effect} else {None}).unwrap() , _ => false }
       && matches!(e.casting, Position(1)|Direction(2)) && e.is_in_range(caster=c, target=champ))
L127 ps = position_score_at_position(version, player, data, &parameter.positioning_score, champ.x, champ.y, Objective(11))
L129 if ps.on_trajectory || has_non_target_action_range || ps.on_periodic_trajectory {
L131   res.push(RunAway(new_with_skill(data, player, end_delay=5, with_skill=true)))   // 태그 3
L132   return res }
L136 res.extend(battle_action(version, rnd, player, data, 5))
L137 res.extend(attack_summon_action(player, data))
L140 camp_pos = context.map.camp_pos(self.camp, is_blue_side = self.team==0)
L142 if dist²(champ, camp_pos) < 22500000001 || cache.game.is_visible(enemy_team, champ.id)
L143    || cache.jungles.iter().any(|e| dist²(e, champ) < 22500000001) {   // 단락 평가 순서: 거리 → is_visible → jungles
L144   res.push(RunAway(SmallActionRunAway::new(data, player, 5))) }   // 태그 3
L147 res.extend(attack_jungle_action(data, champ):   // jungle.rs:55~105 인라인
  L55   champ = player_champion[team][pos].unwrap()
  L56   GameMode::Moba(mode) = cache.game.get_game_mode() else unwrap_failed; camp_state = mode.jungle_runner.get_camp_state(self.team, self.camp)
  L59   sub = Vec::new_in(bump)
  L60   move_speed = champ.stat_cached.move_speed
  L62   for id in camp_state.live_list.iter() { target = cache.game.get_entity_by_id(id) (None → continue)
  L63     if champ.can_attack() {
  L64       atk = champ.attack_effect.unwrap()
  L66       max = atk.range + move_speed*30 + champ.stat_buff_cached.range + (champ.level-1)*atk.growth_range + atk.range_adjust(champ, target) + champ.radius() 
  L70       max += target.radius()      // radius() = radius_mult==0 ? radius : radius*(100+mult)/100
  L71       if dist²(target, champ) <= max² { L72 sub.push(Attack(SmallActionAttack::new(data, target.id))) } }   // 태그 15
  L76     if let Some(skill) = &champ.skill_effect (casting != -1) {
  L77       if champ.can_skill() && skill.target.check(champ, target) {
  L78         max = skill.range + move_speed*30 + stat_buff.range + (level-1)*skill.growth_range + skill.range_adjust(champ,target) + champ.radius()
  L82         max += target.radius()
  L84         if dist² <= max² { L85 sub.push(Skill(SmallActionSkill::new(data, target.id))) } } }   // 태그 16
  L90     skill2 = if champ.level>2 {&champ.skill2_effect} else {&None}; if skill2.casting != -1 {
  L91       if champ.can_skill2() && skill2.target.check(champ, target) {
  L92         max = skill2.range + move_speed*30 + (level-1)*skill2.growth_range + stat_buff.range + range_adjust + champ.radius()
  L96         max += target.radius()
  L97         if dist² <= max² { L98 sub.push(Skill2(SmallActionSkill2::new(data, target.id))) } } }   // 태그 17
        }
  L105  sub )
L148 res.push(self.move_action(rnd, data):   // jungle.rs:18~51 인라인
  L20   camp = map.camp_pos(self.camp, self.team==0)
  L22   if player.team == self.team || self.check_move { L51 AroundPosition::new(rnd, data, camp.0, camp.1, 5) }
        else {
  L25     champ = player_champion[team][pos].unwrap()
  L26     if is_enemy_side(team, champ.x, champ.y) {
  L29       if (self.camp as u8 - 1) < 2 {      // Mushroom(1)|Stump(2) → Morgard 쪽
  L31         (ex,ey) = map.camp_pos(Morgard(4), team==0)
  L33         if dist²(champ,(ex,ey)) > 10000000000 { L34 return AroundPosition::new_with_out_line(rnd, data, ex, ey, 5, Outline(1)) }
            } else {                            // Rhino(0)|Bee(3)|… → Serpen 쪽
  L40         (ex,ey) = map.camp_pos(Serpen(5), team==0)
  L41         if dist² > 10000000000 { L42 return new_with_out_line(rnd, data, ex, ey, 5, Outline(1)) } }
          }
          self.check_move = true   // 아군 진영이거나 경유지 100000 이내
  L51     AroundPosition::new(rnd, data, camp.0, camp.1, 5) } )
L149 return res
```

**`mem` 메모리 접근 53건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | JungleSubPlan(self) | 0x0 | team | r | camp_pos is_blue_side 인자(self.team==0) · get_camp_state team · player.team 과 비교(L22<148) | 4 | OK |  |
| 1 | JungleSubPlan(self) | 0x8 | camp | r | JungleType. camp_pos ty · get_camp_state ty · (camp-1)<2 로 Mushroom/Stump 판별 (L29<148) | 4 | OK |  |
| 2 | JungleSubPlan(self) | 0x9 | check_move | r | bool (L22<148) | 4 | OK |  |
| 3 | OperationData | 0x8 | context | r | &GameContext | 4 | OK |  |
| 4 | OperationData | 0x0 | cache | r | &AbstractGameWithCache | 4 | OK |  |
| 5 | GameContext | 0x0 | pool | r | &Bump → Vec::new_in (L108 · L59<147 지역 Vec) | 4 | OK |  |
| 6 | GameContext | 0x8 | setting | r | &GameSetting → is_blue_side 인라인 (L26<148) | 4 | OK |  |
| 7 | GameContext | 0x20 | map | r | &MapDef → camp_pos (gep 32) | 4 | OK |  |
| 8 | GameSetting | 0x12b8 | width | r | is_blue_side: (x - y + height) > width (gep 4792) | 4 | OK |  |
| 9 | GameSetting | 0x12c0 | height | r | gep 4800 | 4 | OK |  |
| 10 | PlayerState | 0x930 | info.team | r | gep 2352 (<2 아니면 panic_bounds_check) | 4 | OK |  |
| 11 | PlayerState | 0x9c0 | info.position@tag | r | gep 2496 → player_champion 인덱스 | 4 | OK |  |
| 12 | AbstractGameWithCache | 0x1e0 | player_champion | r | gep 480. [team][pos] unwrap(3곳: L111 · L55<147 · L25<148) · [enemy_team] 5칸 순회(L114) | 4 | OK |  |
| 13 | AbstractGameWithCache | 0xd0 | jungles | r | gep 208(ptr)/232(len). bumpalo Vec<&Entity> — ⚠DI 이름 없음. L143 순회 대상(others 가 아니라 jungles 인 것에 주의: gep 208 = 0xd0 = jungles) | 4 | OK |  |
| 14 | AbstractGameWithCache | 0x0 | game | r | &dyn AbstractGame 팻포인터. vtable+0xf8 is_visible(L142) · +0x40 get_game_mode(L56<147) · +0x1f0 get_entity_by_id(L57<147) | 4 | OK |  |
| 15 | GameMode(vtable 반환 {i64,ptr}) | 0x0 | tag | r | 0=Moba → +8 &MobaMode. 그 외 → unwrap_failed(L56<147) | 4 | OK |  |
| 16 | MobaMode | 0x18 | jungle_runner | r | gep 24 → JungleRunner(480B) → get_camp_state(self.team, self.camp) -> &JungleCampState | 4 | OK |  |
| 17 | JungleCampState | 0x0 | live_list | r | std Vec<usize>: ptr@+8 · len@+0x10 (gep 8/16). 몹 entity id 순회 (L62<147) | 4 | OK |  |
| 18 | Entity | 0x660 | x | r | gep 1632 | 4 | OK |  |
| 19 | Entity | 0x668 | y | r | gep 1640 | 4 | OK |  |
| 20 | Entity | 0x5c0 | id | r | gep 1472. is_visible 인자(champ) · Attack/Skill/Skill2::new target 인자(몹) | 4 | OK |  |
| 21 | Entity | 0x5c8 | level | r | gep 1480. (level-1)*growth_range · >2 skill2_effect · >4 ult_effect | 4 | OK |  |
| 22 | Entity | 0x68 | ty@tag | r | gep 104 ==13 Champion (L115 closure) | 4 | OK |  |
| 23 | Entity | 0x70 | ty@Champion.0.action_state@tag | r | gep 112 switch 4/5/6 (is_in_skill 인라인, L116) | 4 | OK |  |
| 24 | Entity | 0x4c0 | attack_effect@tag | r | gep 1216. -1 → unwrap_failed (L64<147: champ.attack_effect.unwrap()) | 4 | OK |  |
| 25 | Entity | 0x490 | attack_effect@Some.0 | r | gep 1168 &Effect → range_adjust self | 4 | OK |  |
| 26 | Entity | 0x4a0 | attack_effect@Some.0.range | r | gep 1184 | 4 | OK |  |
| 27 | Entity | 0x4a8 | attack_effect@Some.0.growth_range | r | gep 1192 | 4 | OK |  |
| 28 | Entity | 0x4c8 | skill_effect@Some.0 | r | gep 1224 &Effect | 4 | OK |  |
| 29 | Entity | 0x4d8 | skill_effect@Some.0.range | r | gep 1240 | 4 | OK |  |
| 30 | Entity | 0x4e0 | skill_effect@Some.0.growth_range | r | gep 1248 | 4 | OK |  |
| 31 | Entity | 0x4f0 | skill_effect@Some.0.target | r | gep 1264 CastingTarget → check(champ, target) (L77<147) | 4 | OK |  |
| 32 | Entity | 0x4f8 | skill_effect@tag(casting) | r | gep 1272. -1=None → 스킬 후보 skip(L76<147) / 논타겟 판정 1\|2 (L117) | 4 | OK |  |
| 33 | Entity | 0x500 | skill2_effect@Some.0 | r | gep 1280. level>2 일 때만, 아니면 정적 None(@anon…19, casting=-1) | 4 | OK |  |
| 34 | Entity | 0x538 | ult_effect@Some.0 | r | gep 1336 (L120 논타겟 판정 전용) | 4 | OK |  |
| 35 | Effect | 0x10 | range | r | gep 16 (skill2) | 4 | OK |  |
| 36 | Effect | 0x18 | growth_range | r | gep 24 (skill2) | 4 | OK |  |
| 37 | Effect | 0x28 | target | r | gep 40 (skill2 CastingTarget) | 4 | OK |  |
| 38 | Effect | 0x30 | casting | r | gep 48 (skill2/ult casting tag, -1=None) | 4 | OK |  |
| 39 | Entity | 0x640 | stat_cached.move_speed | r | gep 1600. ×30 이 사거리 합에 더해짐 | 4 | OK |  |
| 40 | Entity | 0x438 | stat_buff_cached.range | r | gep 1080. 사거리 합에 더해짐 (effect.rs:26 Effect::range 인라인) | 4 | OK |  |
| 41 | Entity | 0x470 | stat_buff_cached.radius_mult | r | gep 1136. Entity::radius(): mult==0 ? radius : radius*(100+mult)/100 (entity.rs:1511~1515) | 4 | OK |  |
| 42 | Entity | 0x680 | radius | r | gep 1664 (champ·몹 양쪽) | 4 | OK |  |
| 43 | ScoreParameter | 0x9f0 | positioning_score | r | gep 2544 | 4 | OK |  |
| 44 | PositioningScore(sret 56B) | 0x30 | on_trajectory | r | gep 48 (L129) | 4 | OK |  |
| 45 | PositioningScore(sret 56B) | 0x31 | on_periodic_trajectory | r | gep 49 (L129) — DI 는 on_trajectory 로 명명 | 4 | OK |  |
| 46 | AbstractGame vtable | 0xf8 | is_visible | r | gep 248 (L142) | 4 | 확인불가(vtable 슬롯) |  |
| 47 | AbstractGame vtable | 0x40 | get_game_mode | r | gep 64 (L56<147) | 4 | 확인불가(vtable 슬롯) |  |
| 48 | AbstractGame vtable | 0x1f0 | get_entity_by_id | r | gep 496 (L57<147). null=None → 그 id skip | 4 | 확인불가(vtable 슬롯) |  |
| 49 | JungleSubPlan(self) | 0x9 | check_move | w | ★&mut self 쓰기 전수 = 이 1곳(m02.ll:39678, dbg 루트 L0<148). 조건: player.team != self.team && !check_move && ( !is_enemy_side(team, champ) \|\| (camp∈{Mushroom,Stump} ? dist²(champ, Morgard camp) ≤ 100000² : dist²(champ, Serpen camp) ≤ 100000²) ) | 4 | OK | i8 1 |
| 50 | (sret) Vec<SmallActionPlay> | 0x0 -> ptr/bump/cap/len |  | w | m02.ll:38448~38454 초기화 · 38707/38720/39643 extend · 39849/39897 sret memcpy | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) | new_in(bump)·push·extend 후 32B memcpy |
| 51 | (지역) attack_jungle_action res Vec | (alloca %14) |  | w | 언와인드 시 drop_glue(m02.ll:39006) | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) | Attack/Skill/Skill2 push 후 본 res 에 extend(L147) |
| 52 | StdRng(rnd) | (콜리) |  | w | 본문 직접 쓰기 없음 | 4 | 확인불가(오프셋 파싱 실패) | battle_action·AroundPosition::new/new_with_out_line 이 소비 |

**`consts` 상수 17건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 22500000001 | 142 | 임계 | 150000² + 1 — (a) 챔프↔캠프 위치 제곱거리 ult(L142) (b) 적 jungles 엔티티↔챔프 제곱거리 ult(L143) → 어느 하나면 RunAway 추가 | 4 |
| 1 | 10000000000 | 33 | 임계 | 100000² — 적 캠프 침투 경유지(Morgard 캠프 L33 / Serpen 캠프 L41)와의 제곱거리 ugt: 멀면 경유지로 out_line 이동, 아니면 check_move=true 후 캠프로 | 4 |
| 2 | 30 | 66 | 계수 | move_speed × 30 — 사거리 합에 더하는 이동 여유(틱 수 추정 · dbg 없음). Attack/Skill/Skill2 세 곳 공통(%233) | 4 |
| 3 | 100 | 66 | 계수 | Entity::radius(): radius*(100+radius_mult)/100 (entity.rs:1515) | 4 |
| 4 | 11 | 127 | 태그 | PositionEvalPurpose::Objective 메모리태그 | 4 |
| 5 | 13 | 115 | 태그 | EntityType::Champion | 4 |
| 6 | 4 | 116 | 태그 | ChampionActionState::Skill 태그 / JungleType::Morgard(camp_pos L31<148) / level>4 ult_effect | 4 |
| 7 | 5 | 116 | 태그 | ChampionActionState::Skill2 태그 / JungleType::Serpen(camp_pos L40<148) / end_delay=5(RunAway·AroundPosition·battle_action) | 4 |
| 8 | 6 | 116 | 태그 | ChampionActionState::Ult 태그 | 4 |
| 9 | -1 | 117 | 센티널 | Option<Effect> None 니치(casting=-1) · 논타겟 unwrap 경로는 panic, 스킬/스킬2 후보 경로는 skip · camp-1 산술(add nsw i8 -1) | 4 |
| 10 | 1 | 117 | 태그 | CastingType::Position / AroundBushOutlineType::Outline(new_with_out_line 마지막 인자 i8 1) / (level-1) / check_move=1 | 4 |
| 11 | 2 | 117 | 임계 | CastingType::Direction / level>2 skill2_effect / (camp-1) ult 2 → Mushroom(1)\|Stump(2) | 4 |
| 12 | 0 | 56 | 태그 | GameMode::Moba 태그(get_game_mode 반환 +0) / radius_mult==0 분기 / team==0 (is_blue_side) | 4 |
| 13 | 3 | 131 | 태그 | SmallActionPlay::RunAway 메모리태그 (store @+177, L131·L144) | 4 |
| 14 | 15 | 72 | 태그 | SmallActionPlay::Attack 메모리태그 (L72<147) | 4 |
| 15 | 16 | 85 | 태그 | SmallActionPlay::Skill 메모리태그 (L85<147) | 4 |
| 16 | 17 | 98 | 태그 | SmallActionPlay::Skill2 메모리태그 (L98<147) | 4 |

**`knobs` 조정점 5건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 캠프 근접/적 jungles 근접 도주 반경 | jungle.rs:142~143 | 22500000001 | 올리면 캠프에서 더 멀어도, 적 정글몹(jungles)이 더 멀어도 RunAway 후보가 추가됨. 이 후보는 battle_action 뒤·공격 후보 앞에 놓인다 | 4 | 기존 |
| 1 | 적 캠프 침투 경유지 도달 반경 | jungle.rs:33,41 | 10000000000 | 올리면 Morgard/Serpen 경유지에서 더 먼 곳에서 check_move 가 켜져 바로 캠프로 직행(out_line 이동 구간 단축) | 4 | 기존 |
| 2 | 사거리 이동 여유 계수 | jungle.rs:66,78,92 | 30 | move_speed×30 을 사거리에 더함. 올리면 더 먼 몹에도 Attack/Skill/Skill2 후보 생성 | 4 | 기존 |
| 3 | out_line 타입 | jungle.rs:34,42 | 1 | AroundBushOutlineType::Outline. 0(None)/2(Inline) 로 바꾸면 경유 이동 경로 성격 변경(생성자 명세 참조) | 4 | 기존 |
| 4 | position_eval purpose | jungle.rs:127 | 11 | Objective 프로파일 | 4 | 기존 |

<details><summary>`callees` 피호출자 42건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | action_candidates | game_ai::plan_legacy::sub_plan::SubPlan::action_candidates | pub | fn(&mut game_ai::plan_legacy::sub_plan::SubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\mod.rs:118 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 18개 중 상위 3개 |
| 1 | action_candidates | game_ai::plan_legacy::sub_plan::HideSubPlan::action_candidates | pub | fn(&mut game_ai::plan_legacy::sub_plan::HideSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\hide.rs:19 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 18개 중 상위 3개 |
| 2 | action_candidates | game_ai::plan_legacy::sub_plan::StealSubPlan::action_candidates | pub | fn(&mut game_ai::plan_legacy::sub_plan::StealSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\steal.rs:33 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 18개 중 상위 3개 |
| 3 | attack_jungle_action | game_ai::plan_legacy::sub_plan::JungleSubPlan::attack_jungle_action | in:game_ai::plan_legacy::sub_plan::jungle | fn(&mut game_ai::plan_legacy::sub_plan::JungleSubPlan, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\jungle.rs:54 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 4 | attack_jungle_action | game_ai::plan_legacy::sub_plan::EpicHuntSubPlan::attack_jungle_action | in:game_ai::plan_legacy::sub_plan::epic_hunt | fn(&mut game_ai::plan_legacy::sub_plan::EpicHuntSubPlan, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\epic_hunt.rs:147 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 5 | attack_jungle_action | game_ai::plan_legacy::sub_plan::SerpenHuntSubPlan::attack_jungle_action | in:game_ai::plan_legacy::sub_plan::serpen_hunt | fn(&mut game_ai::plan_legacy::sub_plan::SerpenHuntSubPlan, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\serpen_hunt.rs:147 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 6 | attack_summon_action | game_ai::attack_summon_action | pub | fn(&game_core::PlayerState, &game_core::OperationData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\fight_check.rs:836 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 7 | battle_action | game_ai::battle_action | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, usize) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\fight_check.rs:620 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 8 | camp_pos | game_core::MapDef::camp_pos | pub | fn(&game_core::MapDef, game_core::JungleType, bool) -> (u64, u64) | game-core\src\simulation\map_def.rs:207 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 9 | can_attack | game_core::Entity::can_attack | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1527 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 10 | can_skill | game_core::Entity::can_skill | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1708 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 11 | can_skill2 | game_core::Entity::can_skill2 | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1721 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 12 | check | game_core::CastingTarget::check | pub | fn(&game_core::CastingTarget, &game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\effect\type.rs:227 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 13 | get_camp_state | game_core::JungleRunner::get_camp_state | pub | fn(&game_core::JungleRunner, usize, game_core::JungleType) -> &game_core::JungleCampState | game-core\src\simulation\entity\jungle.rs:702 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 14 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 15 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 16 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 17 | get_game_mode | game_core::AbstractGame::get_game_mode | pub | fn(&Self/#0) -> game_core::GameMode | game-core\src\simulation.rs:78 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 18 | get_game_mode | <game_core::Game as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::Game) -> game_core::GameMode | game-core\src\simulation\game.rs:1707 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 19 | get_game_mode | <game_core::SingleLaneGame as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::SingleLaneGame) -> game_core::GameMode | game-core\src\simulation\game.rs:4248 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 20 | is_enemy_side | game_core::is_enemy_side | pub | fn(&game_core::GameContext, usize, u64, u64) -> bool | game-core\src\simulation\map_regions.rs:57 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 21 | is_in_range | game_core::Effect::is_in_range | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\effect.rs:63 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 22 | is_visible | game_core::World::is_visible | pub | fn(&game_core::World, usize, usize) -> bool | game-core\src\simulation\game\data.rs:384 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 23 | is_visible | game_core::VisibleState::is_visible | pub | fn(&game_core::VisibleState) -> bool | game-core\src\simulation\game\data.rs:121 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 24 | is_visible | game_core::AbstractGame::is_visible | pub | fn(&Self/#0, usize, usize) -> bool | game-core\src\simulation.rs:125 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 25 | new | game_ai::SmallActionSkill::new | pub | fn(&game_core::OperationData, usize) -> game_ai::SmallActionSkill | game-ai\src\small_action\cast.rs:119 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 26 | new | game_ai::SmallActionAttack::new | pub | fn(&game_core::OperationData, usize) -> game_ai::SmallActionAttack | game-ai\src\small_action\cast.rs:23 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 27 | new | game_ai::SmallActionRunAway::new | pub | fn(&game_core::OperationData, &game_core::PlayerState, usize) -> game_ai::SmallActionRunAway | game-ai\src\small_action\move_actions.rs:25 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 28 | new | game_ai::SmallActionAroundPosition::new | pub | fn(&mut rand::rngs::std::StdRng, &game_core::OperationData, u64, u64, usize) -> game_ai::SmallActionAroundPosition | game-ai\src\small_action\around.rs:831 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 29 | new_with_out_line | game_ai::SmallActionAroundPosition::new_with_out_line | pub | fn(&mut rand::rngs::std::StdRng, &game_core::OperationData, u64, u64, usize, game_ai::AroundBushOutlineType) -> game_ai::SmallActionAroundPosition | game-ai\src\small_action\around.rs:834 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 30 | new_with_skill | game_ai::SmallActionRunAway::new_with_skill | pub | fn(&game_core::OperationData, &game_core::PlayerState, usize, bool) -> game_ai::SmallActionRunAway | game-ai\src\small_action\move_actions.rs:29 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 31 | nontarget_windup_perceived | game_ai::nontarget_windup_perceived | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity) -> bool | game-ai\src\utils.rs:553 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 32 | position_score_at_position | game_ai::position_score_at_position | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, u64, u64, game_ai::PositionEvalPurpose) -> game_core::PositioningScore | game-ai\src\position_eval.rs:1192 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 33 | push | <game_core::Staff as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::staff | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\staff.rs:38 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 34 | push | <game_core::Athlete as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:434 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 35 | push | <game_core::Contract as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:2498 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 36 | radius | game_core::Entity::radius | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1509 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 37 | range_adjust | game_core::Effect::range_adjust | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:29 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 38 | skill_effect | game_core::Entity::skill_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1688 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 39 | team | game_view::ClientData::team | pub | fn(&game_view::ClientData, usize) -> std::option::Option<std::cell::Ref< game_core::Team>> | game-view\src\logic\client\data.rs:1651 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 40 | team | game_view::ClientDatabase::team | pub | fn(&game_view::ClientDatabase, usize) -> std::option::Option<&game_core::Team> | game-view\src\logic\client\data.rs:968 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 41 | team | game_core::PlayerAiContext::<'a, 'b, 'r>::team | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:583 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
</details>

⚠**미매칭 3개**: `extend`, `move_action`, `reserve_internal_or_panic`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m12.ll:35207) · **형제 9개** (JungleSubPlan)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::plan_legacy::sub_plan::JungleSubPlan as std::clone::Clone>::clone | pub | game-ai\src\plan_legacy\sub_plan\jungle.rs:6 | True | fn(&game_ai::plan_legacy::sub_plan::JungleSubPlan) -> game_ai::plan_legacy::sub_plan::JungleSubPlan |
| 1 | <game_ai::plan_legacy::sub_plan::JungleSubPlan as std::fmt::Debug>::fmt | pub | game-ai\src\plan_legacy\sub_plan\jungle.rs:6 | True | fn(&game_ai::plan_legacy::sub_plan::JungleSubPlan, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 2 | game_ai::plan_legacy::sub_plan::JungleSubPlan::new | pub | game-ai\src\plan_legacy\sub_plan\jungle.rs:15 | True | fn(usize, game_core::JungleType) -> game_ai::plan_legacy::sub_plan::JungleSubPlan |
| 3 | game_ai::plan_legacy::sub_plan::JungleSubPlan::base_positioning | in:game_ai::plan_legacy::sub_plan::jungle | game-ai\src\plan_legacy\sub_plan\jungle.rs:19 | False | fn(&mut game_ai::plan_legacy::sub_plan::JungleSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> game_ai::SmallActionPlay |
| 4 | game_ai::plan_legacy::sub_plan::JungleSubPlan::attack_jungle_action | in:game_ai::plan_legacy::sub_plan::jungle | game-ai\src\plan_legacy\sub_plan\jungle.rs:54 | False | fn(&mut game_ai::plan_legacy::sub_plan::JungleSubPlan, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 5 | game_ai::plan_legacy::sub_plan::JungleSubPlan::action_candidates | pub | game-ai\src\plan_legacy\sub_plan\jungle.rs:107 | False | fn(&mut game_ai::plan_legacy::sub_plan::JungleSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 6 | game_ai::plan_legacy::sub_plan::JungleSubPlan::score | pub | game-ai\src\plan_legacy\sub_plan\jungle.rs:152 | False | fn(&game_ai::plan_legacy::sub_plan::JungleSubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 |
| 7 | game_ai::plan_legacy::sub_plan::JungleSubPlan::calculate_score_parameter_value | pub | game-ai\src\plan_legacy\sub_plan\jungle.rs:197 | True | fn(&game_ai::plan_legacy::sub_plan::JungleSubPlan, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::ScoreParameter) |
| 8 | game_ai::plan_legacy::sub_plan::JungleSubPlan::merge | pub | game-ai\src\plan_legacy\sub_plan\jungle.rs:214 | True | fn(&mut game_ai::plan_legacy::sub_plan::JungleSubPlan, game_ai::plan_legacy::sub_plan::JungleSubPlan) |

**`open` 6건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 표기 불가 | L66/L78/L92 사거리 합의 항 순서: effect.rs:26 Effect::range(caster) 인라인이 range + stat_buff.range + (level-1)*growth 를 담고 move_speed*30(%233, dbg 없음)이 어느 소스 줄(26 vs 66)의 항인지 IR 재결합으로 표기 불가 — 합 자체는 확정 | 4 |  |
| 1 | 미탐색 | L142 `is_visible(enemy_team, champ.id)` 의 의미('적에게 내가 보이는가')는 divtable 슬롯명 + 인자 순서(team, id)로 추정. ExpectedGame 구현 본문은 안 봄 | 3 |  |
| 2 | 미탐색 | L56<147 non-Moba 게임모드에서 unwrap_failed(@anon…189) — 정글 서브플랜이 Moba 외 모드에서 호출되는지 여부는 범위 밖 | 4 |  |
| 3 | 미탐색 | 정적 None Effect(@anon…19: casting=-1) 로 인해 논타겟 판정(L118/L120)에서 level≤2 Skill2 / level≤4 Ult 상태의 적은 unwrap_failed 로 패닉 — 실전 도달 여부(그 레벨에서 그 상태 불가) 는 game_core 규칙이라 미확인 | 4 |  |
| 4 | 미탐색 | get_camp_state(team, camp) 의 내부 인덱싱(blue/red × 4캠프, Morgard/Serpen 입력 시 동작)은 g09.ll:137367 안 — 안 봄 | 4 |  |
| 5 | 미탐색 | AroundPosition::new_with_out_line 의 rnd 소비 여부·AroundPosition 184B 내부 live 바이트는 생성자 명세(r14 new_with_out_line) 참조 | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

