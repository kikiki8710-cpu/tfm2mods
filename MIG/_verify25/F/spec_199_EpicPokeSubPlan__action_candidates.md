---

### `199` EpicPokeSubPlan::action_candidates — 에픽(Morgard) 견제 서브플랜의 소액션 후보: v27 규율 액션 우선 → old 후보 중 사거리·타워 안전한 공격/스킬만 남기고, 없으면 견제 이동(Trace/Around/RunAway) 1개

| 항목 | 값 |
|---|---|
| id | `epic_poke__EpicPoke__action_candidates` |
| 심볼 | `_RNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy8sub_plan9epic_pokeNtB2_15EpicPokeSubPlan17action_candidates` |
| 소스 | `game-ai\src\plan_legacy\sub_plan\epic_poke.rs:14` |
| IR | `m02.ll` 40509~45070행 |
| 경로·가시성 | `game_ai::plan_legacy::sub_plan::EpicPokeSubPlan::action_candidates` · **pub** |
| 계층 | 기타 |
| exe | `cc6170` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r16` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&mut game_ai::plan_legacy::sub_plan::EpicPokeSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay>
```

**TLS 메모 절(`sig.tls` · 정정 경로 `/specs[199]/sig/tls/<키>`)**

- `direct`: 본체(m02.ll 40509~45070) + aux 전 구간에 LocalKey::with / thread_local / __getit 접점 0건(grep 실측). 이 함수는 TLS 를 직접 읽거나 쓰지 않는다.
- `indirect_order`: TLS 메모를 가진 콜리의 호출 순서(미러 설계용, 본체 줄 기준): ① v27_objective_discipline_action(40621, L15) ② action_candidates_old(40641, L19) ③ retain#4 안 aoe_heal_covers_low_ally(아군 대상 Skill/Skill2/Ult 원소마다) ④ retain#5 안 EpicPokeSubPlan::score → interaction_score(INTER_CTX 후보) ×act_actions 원소 ⑤ get_move_action: nontarget_windup_perceived ×적 챔프(41158) → position_score_at_position(41291) → position_eval_at(POS_EVAL_CACHE 후보) → check_kill_die_tick(41359, near_enemies.clone() 인자) → near_enemies 루프 range_misjudge_rng/roll → expected_damage_target(42377) ⑥ L224 루프 range_misjudge_rng/roll · is_recent_visible ⑦ L237 score ×move_actions(43896 + fold) → interaction_score ⑧ get_input(44224) ⑨ position_score_at_position(44252, L255). v47_siege_stance·v48_cast_beams·champion_hp_value 는 이 본체에 직접 호출 없음(콜리 내부 여부는 각 자식 명세 소관).
- `note`: get_move_action 의 위 순서는 L299 조기 return(RunAway 단일) 이면 ⑤의 position_score_at_position 까지만 실행된다.

<details><summary>인자 9개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | *mut bumpalo::Vec<SmallActionPlay>(32B) | ptr@0 · bump@+8 · cap@+0x10 · len@+0x18. 기록 지점 6: 40633(L16 vec![v27 액션]) · 44295(L262 vec![best]) · 44307(L260 vec![RunAway]) · 44317(L265 vec![best]) · 44445(L268 vec![best]) · 44698(L234 vec![RunAway]) 는 from_iter_in [1] 로 32B 전부 기록 · 43500(L273) 은 act_actions 32B memcpy. 모든 경로에서 32B 전부 live(niche·패딩 없음) | 4 |
| 1 | 1 | self | &EpicPokeSubPlan(0B ZST) | 본체에서 읽기 0. action_candidates_old 에는 `ptr poison` 으로, score 에는 %1 그대로 전달(closure#5 캡처 +0 / closure#8). IR 속성 noalias·nonnull 만(readonly 없음)이나 쓰기 표면은 0B 라 없음 | 4 |
| 2 | 2 | version | usize | 본체 분기 0. 진입부에서 %68(alloca 8B)에 store(40596) → 콜리 전달: v27 helper(40621) · action_candidates_old(40641) · nontarget_windup_perceived(41158) · position_score_at_position(41291·44252) · check_kill_die_tick(41359) · range_misjudge_rng(41499·43701) · Around/AroundRegion::new · score(43896 + retain#5/max_by_key fold) · get_input(44224) · aoe_heal_covers_low_ally(retain#4) | 4 |
| 3 | 3 | rnd | &mut StdRng(320B) | 본체 직접 read/write 0. 소비 콜리(호출 순서 = PRNG 미러 순서): v27_objective_discipline_action(L15) → action_candidates_old(L19) → retain#5 score×N(L199) → get_move_action 안 check_kill_die_tick(L318) → near_enemies 루프마다 range_misjudge_roll ×3~4(L326·327·328·350) → Around/AroundRegion::new(_rnd 미사용 표기) → L224 루프 range_misjudge_roll ×1/적 → L237 score×N → get_input(L247) | 4 |
| 4 | 4 | player | &PlayerState(2528B) | readonly · 0x930 info.team(40642) · 0x9c0 info.position 태그(40664) · 0x180 info.parameter(positioning_accuracy 41315·43496) · 나머지는 콜리 전달 | 4 |
| 5 | 5 | data | &OperationData(24B) | readonly · +0 cache(&AbstractGameWithCache: +0/+8 &dyn AbstractGame 팻포인터 · +0xf0 others[2] · +0x1e0 player_champion[2][5]) · +8 context(&GameContext: +0 pool · +8 setting(+0x12f8 tick_per_second) · +0x20 map(+0x38b8 regions)) · +0x10 blackboard(&[Blackboard;2], stride 744) | 4 |
| 6 | 6 | parameter | &ScoreParameter(5384B) | readonly · +0x9f0 positioning_score(PositioningScoreData) 만 직접 취해 get_move_action/get_input/position_score_at_position 에 전달(41031) · 통째는 score(retain#5·max_by_key) 로 | 4 |
| 7 | 7 | team_plan | &TeamPlan | L15 v27_objective_discipline_action(team_plan, …) 에만 전달(40621). IR 속성 noundef nonnull 만(readonly·noalias 없음)이나 본체 쓰기 0 | 4 |
| 8 | 8 | debug | &mut DebugFrameData(224B) | 본체 직접 쓰기 0(store 0건) — 콜리 전달만: check_kill_die_tick(41359) · score(43896·retain#5·fold) · get_input(44224) · retain#5 캡처 +48 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn action_candidates(&self, version, rnd, player, data, parameter, team_plan, debug) -> Vec<SmallActionPlay>:
L15  if let Some(a) = team_plan.v27_objective_discipline_action(version, rnd, player, data, JungleType::Morgard) { return vec![a] }   // Option 태그 +0xb1 != 0xFF
L19  let mut old_actions = self.action_candidates_old(version, rnd, player, data, parameter)
L22  team = player.info.team(<2 아니면 bounds 패닉); champ = data.cache.player_champion[team][player.info.position].unwrap()
L23  nearest_enemy_tower: Option<&Entity> = cache.iter_towers_without_nexus(1-team).filter(|t| t.can_target && t.block_target_tick==0).min_by_key(|t| dist²(t,champ))   // 동점=먼저 나온 것
L28  let mut act_actions = old_actions.iter().filter(closure#2).map(|a| a.clone()).collect_in(pool)
     closure#2(a): [캡처 game(&dyn), champ, &nearest_enemy_tower]
       L31 if a ∈ {Attack,Skill,Skill2,Ult} && let Some(t)=game.get_entity_by_id(a.target) && t.ty != Champion → return TRUE (비챔피언 대상 공격은 무조건 통과)
       L32 match a:
         Attack(L35~38): t = get_entity_by_id(a.target) (None→false); champ.attack_effect.unwrap().is_in_range(champ,t) 아니면 false; match nearest_enemy_tower { None → true, Some(tw) → if tw.attack_effect.unwrap().is_in_range(tw, champ) && t.ty != Tower { return same_team(t, champ) /*적이면 false*/ } else true }
         Skill(L45~49): t 없으면 false; champ.skill_effect.unwrap().is_in_range(champ,t) 아니면 false; if let Some(tw)=tower && tw.atk.is_in_range(tw,champ) && t.ty!=Tower && !same_team(t,champ) → false; if skill.ty.expected_move_on_hit() || skill.ty.expected_rush_effect() { return tower.map_or(true, |tw| !tw.attack_effect.unwrap().is_in_range_ex(tw, t, tw.x,tw.y,t.x,t.y, 15000)) } else true   // 돌진/이동 스킬은 타워 사거리(+15000) 안의 대상에게 안 씀
         Skill2(L60~65): 위와 동일, effect = level>2 ? skill2_effect : None (unwrap)
         Ult(L75~80): 위와 동일, effect = level>4 ? ult_effect : None (unwrap)
         그 외 variant → false (이동계 old 액션은 전부 제거)
L93  act_actions.retain(closure#4)  [캡처 data, champ, player, &version] — Skill/Skill2/Ult 만 검사, 나머지 keep:
       t = get_entity_by_id(a.target) (None → 제거); if !same_team(t, champ) → keep (적 대상 스킬은 그대로)
       eff = champ.skill_effect / (level>2 ? skill2 : None) / (level>4 ? ult : None) (None → 제거)
       heal = eff.ty.expected_heal(ctx, champ); shield = expected_shield(..); has_buff = expected_buff(..).is_some()
       near_enemy = 적 챔프.any(dist²(c,t) < 120000²+1 && c 가 champ 팀에 보임) || cache.iter_towers(1-team).any(같은 술어) || cache.jungles.any(같은 술어)
       hp_pct = t.hp*100 / t.stat_cached.hp (max 0 이면 div_by_zero 패닉)
       if heal != 0 { if has_buff → keep; else if hp_pct > 79 → keep iff (shield!=0 || near_enemy) && aoe_heal_covers_low_ally(version, &eff, data, player, t); else keep iff shield!=0 || near_enemy }
       else keep iff shield!=0 || has_buff || near_enemy
L199 act_actions.retain(|a| self.score(version, parameter, rnd, player, data, a, debug) >= -30)
L204 let mut move_actions = self.get_move_action(version, rnd, player, data, &parameter.positioning_score, debug)   // 인라인, 아래
L207 if let Some(t)=nearest_enemy_tower && t.ty==Tower && t.tower.nearest_enemy == Some((_, id)) && id == champ.id { act_actions.truncate(0); move_actions.truncate(0); move_actions.push(RunAway::new_with_skill(data, player, 5, true)) }   // 타워가 나를 조준
L217 if act_actions.is_empty() {
L218   acc = player.info.parameter.positioning_accuracy(); max = 2000-acc
L224   if 적 챔프.any(|c| { jrng=range_misjudge_rng(version,data,player,c.id); emr = max_range_can_use(c,champ)*roll(rnd,&jrng,acc,max)/1000 + 10000; mr = max_range_can_use(champ,c); blackboard[team].is_recent_visible(game,player,c) && dist²(c,champ) <= emr² && (c.ty!=Champion || c.action_state ∈ {Idle,Return,Move}) && mr == 0 && !c.block_input() })
L234     { return vec![RunAway::new_with_skill(data, player, 5, false)] }   // 적이 나를 때릴 수 있는데 나는 못 때림 → 도주
L237   best = move_actions.iter().max_by_key(|a| self.score(version,parameter,rnd,player,data,a,debug)).unwrap()   // 비어 있으면 패닉(get_move_action 은 항상 ≥1 개 반환)
L241   trajectory_possible = game.iter_projectile().any(|p| !same_team(p, champ) && p.move_type ∉ {Target, TargetSplash, BouncingTarget{target_id:Some}} && dist²(champ,p) < 420000²)
L244   if trajectory_possible || 적 챔프.any(|c| c.rush_state ∈ {Rush, RushPenetrate}) {
L247     input = best.clone().get_input(version, rnd, player, data, positioning_score, debug)
L251     match input { Some(Move{x,y}) => { ps = position_score_at_position(version,player,data,positioning_score,x,y,Objective); if ps.on_trajectory || ps.on_periodic_trajectory { return vec![RunAway::new_with_skill(data,player,5,false)] } else { return vec![best.clone()] } }, Some(_) => return vec![best.clone()], None => return vec![best.clone()] }
       }
L268   return vec![best.clone()]
     }
L273 return act_actions

--- get_move_action(version, rnd, player, data, positioning_score, debug) -> Vec (epic_poke.rs:278~424, 인라인) ---
L279 res = Vec::new_in(pool); L281 champ 재조회
L284 has_non_target_action_range = 적 챔프.any(|c| nontarget_windup_perceived(version,player,data,c) && c.ty==Champion && match c.action_state { Skill => c.skill_effect.unwrap(), Skill2 => (level>2?skill2:None).unwrap(), Ult => (level>4?ult:None).unwrap(), _ => return false }.is_in_range(c, champ))
L297 ps = position_score_at_position(version, player, data, positioning_score, champ.x, champ.y, Objective)
L299 if ps.on_trajectory(+0x30) || has_non_target_action_range || ps.on_periodic_trajectory(+0x31) { res.push(RunAway::new_with_skill(data,player,5,true)); return res }
L313 acc = positioning_accuracy(); max = 2000-acc
L317 near_enemies = cache.iter_champions(1-team).filter(|c| c 가 champ 팀에 보임 && dist²(c,champ) < 160000²).collect_in(pool)
L318 me_die_tick = check_kill_die_tick(version, rnd, data, player, champ, near_enemies.clone(), Vec::new_in(pool), debug)
L321 mode = game.get_game_mode() (Moba 아니면 unwrap 패닉); L322 objective_entity = mode.jungle_runner.epic.live_list.first().and_then(|id| game.get_entity_by_id(*id))
L312/324 in_range = false; for enemy in near_enemies:
   jrng = range_misjudge_rng(version,data,player,enemy.id)
   mr = max_range_can_use(champ,enemy)*roll/1000; emr = max_range_can_use(enemy,champ)*roll/1000; emr_near = max_range_nearly_can_use(enemy,champ,40)*roll/1000   // roll 은 매번 새로 호출
   dist = dist²(champ,enemy); near_obj = objective_entity.map_or(true, |o| dist²(enemy,o) < 200000²+1)
   if me_die_tick < setting.tick_per_second { emr2 = max_range_nearly_can_use(enemy,champ,60)*roll/1000; if dist > mr² && emr2 < mr { if near_obj { res.push(Trace(enemy,5)) } } else if dist <= emr2² { in_range = true } }
   else if enemy.remain_action_time() > 10 && mr >= 1 { if dist > mr² && near_obj { res.push(Trace(enemy,5)) } }
   else if emr < mr && dist > mr² { if near_obj { res.push(Trace(enemy,5)) } }
   else if dist <= emr_near² { in_range = true }
L364 for e in cache.others[1-team]: if let Some(atk)=e.attack_effect { rng = e.stat_buff.range + atk.range + (e.level-1)*atk.growth_range + atk.range_adjust(e,champ) + radius_eff(e) + radius_eff(champ); if dist²(champ,e) <= rng² { in_range = true; break } }
L376 if res.is_empty() {
   region = map.regions[min(champ.y/32000,29)][min(champ.x/32000,29)]
   match objective_entity(재조회) {
     Some(obj) if obj.ty==Epic => if region==7 { if obj 가 champ 팀에 보임 { atk=obj.attack_effect.unwrap(); rng = atk.range + 20000 + champ.stat_buff.range + (champ.level-1)*atk.growth_range + radius_eff(champ) + radius_eff(obj); dmg = atk.expected_damage_target(ctx, obj, champ); if dmg*2 < champ.hp || rng² < dist²(champ,obj) { res.push(AroundRegion(7,5)) } else { res.push(RunAway::new(data,player,5)) } } else { res.push(Around(obj.id,5)) } } else { res.push(Around(obj.id,5)) }
     Some(_) | None => res.push(AroundRegion(7,5))
   }
 }
L409 if game.is_visible(1-team, champ.id) && (적 챔프.any(|c| !same_team(c,champ) && dist²(c,champ) < 150000²+1) || cache.others[1-team].any(|e| dist²(e,champ) < 150000²+1)) { res.push(RunAway::new(data,player,5)) }
L419 if in_range { res.push(RunAway::new_with_skill(data,player,5,false)) }
L423 return res

보조: same_team(a,b) = a.team 태그==b.team 태그 && (Neutral || Player idx 같음)(entity.rs:1127 인라인) · '보임' = viewer.team Neutral → true, Player(t) → e.visible_state[t]==Visible(entity.rs:1482 인라인) · radius_eff(e) = radius_mult==0 ? radius : radius*(100+radius_mult)/100 · dist² = |dx|²+|dy|² (u64 wrapping 없음, sub nuw 선택)
```

**`mem` 메모리 접근 59건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | 40642 · <2 바운드체크(40644·40651) · 1-team 이 적 팀 인덱스(40680) | 4 | OK |  |
| 1 | PlayerState | 0x9c0 | info.position@tag | r | 40664 i32 → player_champion[team][pos] | 4 | OK |  |
| 2 | PlayerState | 0x180 | info.parameter | r | AthleteParameter::positioning_accuracy(&self) 인자(41314·43495) | 4 | OK |  |
| 3 | OperationData | 0x0 | cache | r | 40667 | 4 | OK |  |
| 4 | OperationData | 0x8 | context | r | 40630·40977·41060 등 → +0 pool · +8 setting · +0x20 map | 4 | OK |  |
| 5 | OperationData | 0x10 | blackboard | r | 43661 → [Blackboard;2] 를 team(0x930) 로 인덱스(43662 stride 744) → is_recent_visible self | 4 | OK |  |
| 6 | AbstractGameWithCache | 0x0 | game(&dyn AbstractGame data) | r | 40957 · vtable 호출 인자 | 4 | OK |  |
| 7 | AbstractGameWithCache | 0x8 | game vtable | r | 40959 · 슬롯 +0x28 tick(41657) · +0x40 get_game_mode(41368) · +0xf8 is_visible(42465) · +0x1f0 get_entity_by_id(41407·41649·closure#2 76433) · +0x210 iter_projectile(43921) — divtable AbstractGame 실측 | 3 | OK |  |
| 8 | AbstractGameWithCache | 0xf0 | others[1-team] | r | 41996 ({ptr,bump,cap},len 32B stride, +0x18 len 42002) — 적 팀 비챔피언 엔티티(타워·미니언·소환물 등) | 4 | OK |  |
| 9 | AbstractGameWithCache | 0x1e0 | player_champion[team][pos] | r | 40668~40671 (480 = 0x1e0, [5 x ptr] stride 40) · 적 팀 배열 41093(%212) | 4 | OK |  |
| 10 | GameContext | 0x0 | pool(&Bump) | r | 40632·40979·41060 — Vec 할당자 | 4 | OK |  |
| 11 | GameContext | 0x8 | setting | r | 41439/41601 → GameSetting | 4 | OK |  |
| 12 | GameContext | 0x20 | map(&MapDef) | r | 42165 | 4 | OK |  |
| 13 | GameSetting | 0x12f8 | tick_per_second | r | 41602 (4856) — me_die_tick < tps 비교(L335) | 4 | OK |  |
| 14 | MapDef | 0x38b8 | regions[30][30] | r | 42167 (14520) · [y/32000 clamp 29][x/32000 clamp 29] → 지역 번호(L378) | 4 | OK |  |
| 15 | MobaMode | 0x1a0 | jungle_runner.epic.live_list.ptr | r | 41399/42201 (416) · GameMode::Moba(&MobaMode) 페이로드 | 4 | OK |  |
| 16 | MobaMode | 0x1a8 | jungle_runner.epic.live_list.len | r | 41389/42191 (424) · first() → get_entity_by_id = objective_entity(살아있는 에픽) | 4 | OK |  |
| 17 | Entity | 0x0 | team@tag(TeamType: Player 0 · Neutral 1) | r | 42602·43965·44248(closure) 등 — 같은 팀 판정(entity.rs:1127 인라인)·가시성 판정(entity.rs:1482 인라인) | 4 | OK |  |
| 18 | Entity | 0x8 | team@Player.0 | r | 42603 등 팀 인덱스 | 4 | OK |  |
| 19 | Entity | 0x38 | visible_state[t]@tag | r | 42270 (56 + 24t) == Visible(0) — obj.is_visible(champ.team) 인라인(entity.rs:1482~1483) | 4 | OK |  |
| 20 | Entity | 0x68 | ty@tag(EntityType) | r | 41162 ==13 Champion · 42231 ==5 Epic · 43379 ==2 Tower · 43768 !=13 | 4 | OK |  |
| 21 | Entity | 0x70 | ty@Champion.action_state@tag(ChampionActionState) | r | 41189 switch 4 Skill/5 Skill2/6 Ult · 43772 <3(Idle/Return/Move) | 4 | OK |  |
| 22 | Entity | 0x88 | ty@Tower.info.nearest_enemy@tag | r | 43394 (136) Some 판정 | 4 | OK |  |
| 23 | Entity | 0x98 | ty@Tower.info.nearest_enemy@Some.1(id) | r | 43408 (152) == champ.id → 타워가 나를 조준 | 4 | OK |  |
| 24 | Entity | 0x308 | rush_state@tag | r | 44058 (776) ≥0(암묵 RushPenetrate) 또는 == 2^63+3(Rush) → 돌진 중 | 4 | OK |  |
| 25 | Entity | 0x438 | stat_buff_cached.range | r | 42060 (1080) 사거리 합산 | 4 | OK |  |
| 26 | Entity | 0x470 | stat_buff_cached.radius_mult | r | 42072 (1136) 0 이면 radius 그대로, 아니면 radius*(100+mult)/100 | 4 | OK |  |
| 27 | Entity | 0x490 | attack_effect@Some.0(&Effect 56B) | r | 42051 (1168) range_adjust/expected_damage_target self | 4 | OK |  |
| 28 | Entity | 0x4a0 | attack_effect.range | r | 42054 (1184) | 4 | OK |  |
| 29 | Entity | 0x4a8 | attack_effect.growth_range | r | 42056 (1192) × (level-1) | 4 | OK |  |
| 30 | Entity | 0x4c0 | attack_effect@tag(casting i32, -1=None) | r | 42045 (1216) | 4 | OK |  |
| 31 | Entity | 0x4c8 | skill_effect@Some.0 | r | 41216 (1224) action_state Skill 일 때 is_in_range self · closure#2/#4 에서 ty(Arc<dyn EffectType>) vtable 호출 | 4 | OK |  |
| 32 | Entity | 0x4f8 | skill_effect@tag | r | 41199 (1272) -1 → unwrap 패닉 | 4 | OK |  |
| 33 | Entity | 0x500 | skill2_effect@Some.0 | r | 41225 (1280) level>2 일 때만(아니면 정적 None 상수 anon.19) | 4 | OK |  |
| 34 | Entity | 0x538 | ult_effect@Some.0 | r | 41248 (1336) level>4 일 때만 | 4 | OK |  |
| 35 | Entity | 0x5c0 | id | r | 41498 (1472) enemy.id → range_misjudge_rng/Trace target · 42464 champ.id → is_visible · 43414 | 4 | OK |  |
| 36 | Entity | 0x5c8 | level | r | 41222 (1480) >2 / >4 · 42058 (level-1)*growth_range | 4 | OK |  |
| 37 | Entity | 0x628 | stat_cached.hp(최대 HP) | r | retain#4 18416 (1576) hp% 분모(0 이면 div_by_zero 패닉) | 4 | OK |  |
| 38 | Entity | 0x660 | x | r | 40895 등 (1632) 거리 계산 | 4 | OK |  |
| 39 | Entity | 0x668 | y | r | 40899 등 (1640) | 4 | OK |  |
| 40 | Entity | 0x670 | hp | r | 42383 (1648) dmg*2 < hp · retain#4 18422 hp*100/max | 4 | OK |  |
| 41 | Entity | 0x680 | radius | r | 42078 (1664) | 4 | OK |  |
| 42 | Entity | 0x6a0 | block_target_tick | r | 40821 (1696) ==0 (closure#0) | 4 | OK |  |
| 43 | Entity | 0x6b9 | can_target | r | 40818 (1721) (closure#0) | 4 | OK |  |
| 44 | Projectile | 0x0 | team@tag/+8 idx | r | 43965·43971 내 팀 투사체 제외 | 4 | OK |  |
| 45 | Projectile | 0x40 | move_type@tag(ProjectileMoveType) | r | 43984 (64) — Target(6)·TargetSplash(7)·BouncingTarget{target_id:Some}(암묵, +0x40 == 1) 제외 = 논타겟 투사체만 | 4 | OK |  |
| 46 | Projectile | 0x100 | x | r | 44006 (256) | 4 | OK |  |
| 47 | Projectile | 0x108 | y | r | 44010 (264) | 4 | OK |  |
| 48 | ScoreParameter | 0x9f0 | positioning_score | r | 41031 (2544) | 4 | OK |  |
| 49 | SmallActionPlay | 0xb1 | tag | r | 40622 (177) v27 결과 None 판정(-1) · 각 push 직전 태그 store(writes 참조) · closure#2 76415(필터 분기) · retain#4 17578 · retain#5/from_iter 의 이동 시 -1 검사(20298 등) | 4 | OK |  |
| 50 | SmallActionPlay | 0x8 | Attack/Skill/Skill2/Ult.target | r | closure#2 76424·76483 · retain#4 17576 (SmallActionAttack/Skill 24B: start_tick +0 · target +8 · is_act +0x10) | 4 | OK |  |
| 51 | PositioningScore | 0x30 | on_trajectory | r | 41295 (48) · 44262 | 4 | OK |  |
| 52 | PositioningScore | 0x31 | on_periodic_trajectory | r | 41298 (49) · 44268 — DI 이름은 두 곳 모두 `on_trajectory` 로 이 값에 붙어 있음(unknown 참조) | 4 | OK |  |
| 53 | Option<Input> | 0x0 | tag(-1 None · 0 Move · 1..5 기타) | r | 44235 (get_input 결과 32B) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 54 | Option<Input> | 0x8 | Move.x / +0x10 Move.y | r | 44245·44242 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 55 | Blackboard | 0x0 | blackboard[team](744B) | r | 43662 — is_recent_visible self | 4 | 오귀속(사전은 다른 필드를 준다) |  |
| 56 | (sret) | 0x0 | Vec<SmallActionPlay> 32B | w | 6 경로 from_iter_in(단일 원소) + 1 경로 memcpy(act_actions) | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) | signature.returns 참조 |
| 57 | (sret 원소) | 0xb1 | SmallActionPlay 태그 | w | get_move_action 로컬 res 와 move_actions 에 push 되는 원소의 태그 — sret 로는 best.clone() 1개만 나감 | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) | 3(RunAway 44306·44697·43080·43261·43322·43442) · 5(Around 42299·42453) · 7(AroundRegion 42429·42473·42526) · 14(Trace 41695·41837·41945) |
| 58 | rnd/debug/team_plan/self | 0x0 | (직접 쓰기 없음) | w | &mut rnd(320B)·&mut debug(224B) 는 본체 store 0건 — 부작용은 콜리 내부(action_candidates_old·score·get_input·check_kill_die_tick·range_misjudge_roll·Around::new 등) | 4 | 확인불가(tcx 사전에 타입 없음) | - |

**`consts` 상수 36건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 4 | 15 | 임계 | JungleType::Morgard(4) — v27_objective_discipline_action 의 target 인자(i8 range 0..6, 40621). 에픽 = Morgard | 4 |
| 1 | -1 | 15 | 태그 | Option<SmallActionPlay>::None 태그(+0xb1 == 0xFF, 40624; v27 helper 본체 m09.ll 19276 가 store i8 -1) · Option<Effect>::None(casting i32 -1) · Option<Input>::None(+0 i64 -1, 44236) | 4 |
| 2 | 2 | 22 | 센티널 | team < 2 바운드(40644) · EntityType::Tower(43381) · Option<PathFinder>::None 태그 2(41686·41694) · level > 2 → skill2 유효(41224) · 투사체 move_type 니치 접기(43988) | 4 |
| 3 | 1 | 23 | 임계 | 1 - team = 적 팀(40680·41321) · dmg<<1 = dmg*2(42382, shl 로 접힘) · SmallActionTrace 좌표 Some 마커 | 4 |
| 4 | 13 | 285 | 태그 | EntityType::Champion(41164·43770·closure#2 76446) | 4 |
| 5 | 5 | 286 | 태그 | ChampionActionState::Skill2(41193) · EntityType::Epic(42233) · end_delay 5(전 생성자·Trace) · SmallActionPlay::Around 태그 | 4 |
| 6 | 6 | 286 | 임계 | ChampionActionState::Ult(41194) | 4 |
| 7 | 11 | 297 | 태그 | PositionEvalPurpose::Objective(메모리태그 11, tcxdict --enum) — position_score_at_position purpose(41291·44252) | 3 |
| 8 | 2000 | 315 | 계수 | max_v = 2000 - positioning_accuracy (41321·43620) — 사거리 오판 롤 상한(‰) | 4 |
| 9 | 1000 | 326 | 계수 | range_misjudge_roll ‰ 나눗셈(41516·41527·41538·41621·43718) | 4 |
| 10 | 40 | 328 | 태그 | max_range_nearly_can_use(enemy, champ, tick 40) — 40틱 안에 쓸 수 있는 사거리(41529) | 4 |
| 11 | 60 | 350 | 미상 | max_range_nearly_can_use(enemy, champ, tick 60) — 1초 내 사망 예상 시 더 넓은 창(41608) | 4 |
| 12 | 40000000001 | 332 | 임계 | 200000²+1 — 적이 에픽(objective_entity)에서 200000(6.25셀) 미만이면 '에픽 근처'(41595) | 4 |
| 13 | 10 | 336 | 임계 | enemy.remain_action_time() > 10 틱(41755) | 4 |
| 14 | 999 | 336 | 임계 | roll×max_range_can_use > 999 ⇔ mr(=÷1000) ≥ 1 ⇔ 내 사거리 > 0 (41756, 나눗셈 전 곱으로 접힘) | 4 |
| 15 | 15000 | 355 | 산출값 | SmallActionTrace.attack_range_margin(41691·41833·41941) · closure#2::#4 is_in_range_ex offset(4362) | 4 |
| 16 | 14 | 355 | 태그 | SmallActionPlay::Trace 메모리태그(41695·41837·41945) | 4 |
| 17 | 100 | 366 | 계수 | radius*(100+radius_mult)/100 (42087~42089) · retain#4 hp*100/max(18424) | 4 |
| 18 | 32000 | 378 | 인덱스 | 셀 크기 — 좌표→regions 인덱스(42155·42159) | 4 |
| 19 | 29 | 378 | 인덱스 | regions 인덱스 상한 clamp(umin, 42159·42164) | 4 |
| 20 | 7 | 383 | 태그 | map.regions 값 7 = 에픽 지역(42237) · AroundRegion::new target_region 7(42219·42243·42417) · SmallActionPlay::AroundRegion 메모리태그 7(42429) · 투사체 BouncingTarget 논리 idx 7(43990) | 4 |
| 21 | 20000 | 389 | 계수 | 에픽 평타 사거리 여유 +20000 (42369) | 4 |
| 22 | 3 | 393 | 태그 | SmallActionPlay::RunAway 메모리태그 3(42441 등) · action_state < 3 = Idle/Return/Move(43773). 본문의 `shl i64 %x, 3`(43182 등)은 &Entity 포인터 stride ×8 접힘으로 이 상수와 무관 | 4 |
| 23 | 22500000001 | 411 | 임계 | 150000²+1 — 내가 적에게 보일 때 적 챔프/타 엔티티가 150000 미만이면 RunAway 추가(42666·43068) | 4 |
| 24 | 0 | 419 | 태그 | res.len()==0 (42148) · VisibleState::Visible 태그 0(42273) · TeamType::Player 태그 0 · Input::Move 태그 0(44238) | 4 |
| 25 | 10000 | 226 | 미상 | emr = 적 사거리 오판값 + 10000 여유(43720) | 4 |
| 26 | 176400000000 | 243 | 임계 | 420000² — 논타겟 적 투사체가 420000 안이면 trajectory_possible(44027) | 4 |
| 27 | -9223372036854775805 | 245 | 태그 | RushState::Rush 메모리태그(2^63+3, tcxdict --enum RushState) — ≥0 은 암묵 RushPenetrate(44064·44065) | 3 |
| 28 | 9 | 242 | 센티널 | ProjectileMoveType 태그 ≠9(Removed 이전 값, assume) 니치 접기(43986) | 4 |
| 29 | 4 | 242 | 임계 | 투사체 논리 idx 4 Target(43992) / level > 4 → ult 유효(41247) / ChampionActionState::Skill(41192) | 4 |
| 30 | 25600000000 | 317 | 미상 | 160000² — near_enemies 반경(aux m02 76368) | 4 |
| 31 | 14400000001 | 103 | 미상 | 120000²+1 — retain#4: 아군 대상 근처에 보이는 적 챔프/타워/정글몹(aux m01 17805 등) | 4 |
| 32 | 79 | 107 | 미상 | 대상 hp% > 79 (≥80%) 이면 힐은 aoe_heal_covers_low_ally 로만 허용(aux m01 18442·19249·20056) | 4 |
| 33 | -30 | 201 | 미상 | retain#5: score < -30 이면 제거(aux m01 20233 icmp slt) | 4 |
| 34 | 15 | 31 | 태그 | closure#2: 태그-15 < 4 ⇔ Attack(15)/Skill(16)/Skill2(17)/Ult(18)(aux m02 76419·76420) · Attack 메모리태그 | 4 |
| 35 | 12 | 32 | 미상 | closure#2 switch 논리 idx 12 Attack/13 Skill/14 Skill2/15 Ult(76470~76473) | 4 |

**`knobs` 조정점 14건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 견제 이동 후보의 end_delay | epic_poke.rs:301·338·343·355·384·387·393·395·403·406·414·420 (전 생성자 6번째 인자 5) | 5 | 올리면 한 번 정한 Trace/Around/RunAway 를 더 오래 유지(재평가 지연) | 4 | 기존 |
| 1 | '에픽 근처' 판정 반경 | epic_poke.rs:332 | 40000000001 | 올리면(200000 초과) 에픽에서 더 멀리 있는 적도 Trace 견제 대상이 된다 | 4 | 기존 |
| 2 | 에픽 평타 사거리 여유 | epic_poke.rs:389 | 20000 | 올리면 에픽 지역(7) 안에서 RunAway 대신 AroundRegion 을 택하는 거리 문턱이 넓어져 더 멀리서부터 도망친다(2대 치명일 때) | 4 | 기존 |
| 3 | 노출 시 RunAway 트리거 반경 | epic_poke.rs:411·412 | 22500000001 | 올리면(150000 초과) 적에게 보일 때 더 먼 적/타워로도 RunAway 후보가 추가된다 | 4 | 기존 |
| 4 | 논타겟 투사체 경계 반경 | epic_poke.rs:243 | 176400000000 | 올리면(420000 초과) 더 먼 스킬샷에도 best 의 목표 지점 궤적 검사(L255)를 돌려 RunAway 로 바뀔 확률 증가 | 4 | 기존 |
| 5 | 적 사거리 여유(도주 트리거) | epic_poke.rs:226 | 10000 | 올리면 '적이 나를 때릴 수 있는데 나는 못 때림' 도주(L234)가 더 멀리서 발동 | 4 | 기존 |
| 6 | score 컷 | epic_poke.rs:201 | -30 | 올리면(예 0) 점수 낮은 공격/스킬 후보가 더 많이 잘려 이동 후보로 넘어가기 쉬움; 내리면 반대 | 4 | 기존 |
| 7 | 사망 임박 창의 사거리 틱 | epic_poke.rs:350 | 60 | 내리면(40 등) 1초 내 사망 예상 시 적의 '곧 쓸 수 있는' 사거리를 좁게 봐 Trace 를 더 자주 낸다 | 4 | 기존 |
| 8 | 일반 창의 사거리 틱 | epic_poke.rs:328 | 40 | 올리면 in_range(→ RunAway 스킬 없이) 가 더 넓은 적 사거리에서 켜진다 | 4 | 기존 |
| 9 | 아군 힐 허용 hp% | epic_poke.rs:107·140·173 | 79 | 내리면(예 59) 60% 이상 아군에게도 aoe_heal_covers_low_ally 검사 없이는 힐을 안 쓴다 | 4 | 기존 |
| 10 | 아군 스킬 '근처 적' 반경 | epic_poke.rs:103~105·136~138·169~171 | 14400000001 | 올리면 더 먼 적이 있어도 아군 대상 힐/실드 스킬을 유지 | 4 | 기존 |
| 11 | 돌진 스킬 타워 안전 여유 | epic_poke.rs:50·65·80 | 15000 | 올리면 타워 사거리+여유 안의 대상에게 돌진/이동 스킬을 덜 쓴다 | 4 | 기존 |
| 12 | near_enemies 반경 | epic_poke.rs:317 | 25600000000 | 올리면(160000 초과) 더 먼 적까지 사망틱·Trace 계산에 포함 | 4 | 기존 |
| 13 | 에픽 지역 번호 | epic_poke.rs:383·395·403·406 | 7 | 맵 regions 값 7 이 에픽 지역이라는 가정 — 바꾸면 AroundRegion 목적지가 바뀐다 | 4 | 기존 |

<details><summary>`callees` 피호출자 67건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | action_candidates | game_ai::plan_legacy::sub_plan::EpicPokeSubPlan::action_candidates | pub | fn(&mut game_ai::plan_legacy::sub_plan::EpicPokeSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\epic_poke.rs:14 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | action_candidates_old | game_ai::plan_legacy::sub_plan::EpicPokeSubPlan::action_candidates_old | pub | fn(&mut game_ai::plan_legacy::sub_plan::EpicPokeSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\epic_poke.rs:426 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 2 | aoe_heal_covers_low_ally | game_ai::aoe_heal_covers_low_ally | pub | fn(usize, &game_core::Effect, &game_core::OperationData, &game_core::PlayerState, &game_core::Entity) -> bool | game-ai\src\buff_value.rs:543 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 3 | block_input | game_core::Entity::block_input | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1501 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | check_kill_die_tick | game_ai::check_kill_die_tick | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::OperationData, &game_core::PlayerState, &game_core::Entity, bumpalo::collections::vec::Vec< &game_core::Entity>, bumpalo::collections::vec::Vec< &game_core::Entity>, &mut game_core::DebugFrameData) -> usize | game-ai\src\fight_check.rs:917 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 5 | expected_buff | game_core::EffectType::expected_buff | pub | fn(&Self/#0, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + ) -> std::option::Option<game_core::BuffState> | game-core\src\simulation\effect\type.rs:287 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 18개 중 상위 3개 |
| 6 | expected_buff | <game_core::RangeEffect as game_core::EffectType>::expected_buff | pub | fn(&game_core::RangeEffect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + ) -> std::option::Option<game_core::BuffState> | game-core\src\simulation\effect\type\range_effect.rs:94 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 18개 중 상위 3개 |
| 7 | expected_buff | <game_core::AddBuffEffect as game_core::EffectType>::expected_buff | pub | fn(&game_core::AddBuffEffect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + ) -> std::option::Option<game_core::BuffState> | game-core\src\simulation\effect\type\add_buff.rs:28 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 18개 중 상위 3개 |
| 8 | expected_damage_target | game_core::Effect::expected_damage_target | pub | fn(&game_core::Effect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + , &game_core::Entity) -> usize | game-core\src\simulation\effect.rs:91 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 9 | expected_heal | game_ai::expected_heal | pub | fn(&game_core::GameContext, &game_core::Entity, &game_core::Entity) -> usize | game-ai\src\fight_check.rs:266 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 31개 중 상위 3개 |
| 10 | expected_heal | game_core::EffectType::expected_heal | pub | fn(&Self/#0, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + ) -> usize | game-core\src\simulation\effect\type.rs:283 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 31개 중 상위 3개 |
| 11 | expected_heal | <game_core::HealEffect as game_core::EffectType>::expected_heal | pub | fn(&game_core::HealEffect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + ) -> usize | game-core\src\simulation\effect\type\heal.rs:186 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 31개 중 상위 3개 |
| 12 | expected_move_on_hit | game_core::EffectType::expected_move_on_hit | pub | fn(&Self/#0) -> bool | game-core\src\simulation\effect\type.rs:293 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 20개 중 상위 3개 |
| 13 | expected_move_on_hit | <game_core::CombineEffect as game_core::EffectType>::expected_move_on_hit | pub | fn(&game_core::CombineEffect) -> bool | game-core\src\simulation\effect\type\combine.rs:96 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 20개 중 상위 3개 |
| 14 | expected_move_on_hit | <game_core::DelayedEffect as game_core::EffectType>::expected_move_on_hit | pub | fn(&game_core::DelayedEffect) -> bool | game-core\src\simulation\effect\type\delayed.rs:45 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 20개 중 상위 3개 |
| 15 | expected_rush_effect | game_core::EffectType::expected_rush_effect | pub | fn(&Self/#0) -> bool | game-core\src\simulation\effect\type.rs:291 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 9개 중 상위 3개 |
| 16 | expected_rush_effect | <game_core::RushEffect as game_core::EffectType>::expected_rush_effect | pub | fn(&game_core::RushEffect) -> bool | game-core\src\simulation\effect\type\rush.rs:53 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 9개 중 상위 3개 |
| 17 | expected_rush_effect | <game_core::CombineEffect as game_core::EffectType>::expected_rush_effect | pub | fn(&game_core::CombineEffect) -> bool | game-core\src\simulation\effect\type\combine.rs:42 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 9개 중 상위 3개 |
| 18 | expected_shield | game_ai::expected_shield | pub | fn(&game_core::GameContext, &game_core::Entity, &game_core::Entity) -> usize | game-ai\src\fight_check.rs:284 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 22개 중 상위 3개 |
| 19 | expected_shield | game_core::EffectType::expected_shield | pub | fn(&Self/#0, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + ) -> usize | game-core\src\simulation\effect\type.rs:285 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 22개 중 상위 3개 |
| 20 | expected_shield | <game_core::RangeEffect as game_core::EffectType>::expected_shield | pub | fn(&game_core::RangeEffect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + ) -> usize | game-core\src\simulation\effect\type\range_effect.rs:90 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 22개 중 상위 3개 |
| 21 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 22 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 23 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 24 | get_game_mode | game_core::AbstractGame::get_game_mode | pub | fn(&Self/#0) -> game_core::GameMode | game-core\src\simulation.rs:78 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 25 | get_game_mode | <game_core::Game as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::Game) -> game_core::GameMode | game-core\src\simulation\game.rs:1707 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 26 | get_game_mode | <game_core::SingleLaneGame as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::SingleLaneGame) -> game_core::GameMode | game-core\src\simulation\game.rs:4248 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 27 | get_input | game_ai::SmallActionPlay::get_input | pub | fn(&mut game_ai::SmallActionPlay, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &mut game_core::DebugFrameData) -> std::option::Option<game_core::Input> | game-ai\src\small_action.rs:157 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 28 | get_move_action | game_ai::plan_legacy::sub_plan::EpicHuntSubPlan::get_move_action | in:game_ai::plan_legacy::sub_plan::epic_hunt | fn(&mut game_ai::plan_legacy::sub_plan::EpicHuntSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\epic_hunt.rs:517 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 4개 중 상위 3개 |
| 29 | get_move_action | game_ai::plan_legacy::sub_plan::EpicPokeSubPlan::get_move_action | in:game_ai::plan_legacy::sub_plan::epic_poke | fn(&mut game_ai::plan_legacy::sub_plan::EpicPokeSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\epic_poke.rs:278 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 4개 중 상위 3개 |
| 30 | get_move_action | game_ai::plan_legacy::sub_plan::SerpenHuntSubPlan::get_move_action | in:game_ai::plan_legacy::sub_plan::serpen_hunt | fn(&mut game_ai::plan_legacy::sub_plan::SerpenHuntSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\serpen_hunt.rs:516 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 4개 중 상위 3개 |
| 31 | is_empty | game_core::PatchSetting::is_empty | pub | fn(&game_core::PatchSetting) -> bool | game-core\src\patch_engine.rs:853 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 32 | is_empty | game_core::ModAiRegistry::is_empty | pub | fn(&game_core::ModAiRegistry) -> bool | game-core\src\mod_ai.rs:15 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 33 | is_empty | game_core::PublicChampionEvidence::is_empty | pub | fn(&game_core::PublicChampionEvidence) -> bool | game-core\src\banpick\public_evidence.rs:96 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 34 | is_in_range | game_core::Effect::is_in_range | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\effect.rs:63 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 35 | is_in_range_ex | game_core::Effect::is_in_range_ex | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity, u64, u64, u64, u64, u64) -> bool | game-core\src\simulation\effect.rs:78 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 36 | is_recent_visible | game_core::Blackboard::is_recent_visible | pub | fn(&game_core::Blackboard, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\game\blackboard.rs:346 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 37 | is_visible | game_core::World::is_visible | pub | fn(&game_core::World, usize, usize) -> bool | game-core\src\simulation\game\data.rs:384 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 38 | is_visible | game_core::VisibleState::is_visible | pub | fn(&game_core::VisibleState) -> bool | game-core\src\simulation\game\data.rs:121 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 39 | is_visible | game_core::AbstractGame::is_visible | pub | fn(&Self/#0, usize, usize) -> bool | game-core\src\simulation.rs:125 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 40 | iter_champions | game_core::AbstractGameWithCache::<'a, 'b>::iter_champions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5759 ~ game_core[6a30]::simulation::{impl#5}::iter_champions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1904 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 41 | iter_projectile | game_core::AbstractGame::iter_projectile | pub | fn(&Self/#0) -> game_core::ProjectileIter | game-core\src\simulation.rs:182 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 42 | iter_projectile | <game_core::Game as game_core::AbstractGame>::iter_projectile | pub | fn(&game_core::Game) -> game_core::ProjectileIter | game-core\src\simulation\game.rs:3760 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 43 | iter_projectile | <game_core::SingleLaneGame as game_core::AbstractGame>::iter_projectile | pub | fn(&game_core::SingleLaneGame) -> game_core::ProjectileIter | game-core\src\simulation\game.rs:4019 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 44 | iter_towers | game_core::AbstractGameWithCache::<'a, 'b>::iter_towers | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5762 ~ game_core[6a30]::simulation::{impl#5}::iter_towers::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1908 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 45 | iter_towers_without_nexus | game_core::AbstractGameWithCache::<'a, 'b>::iter_towers_without_nexus | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5750 ~ game_core[6a30]::simulation::{impl#5}::iter_towers_without_nexus::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1830 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 46 | keep | game_view::pixel_fx::Dither::keep | in:game_view::pixel_fx | fn(game_view::pixel_fx::Dither, i32, i32) -> bool | game-view\src\view\pixel_fx.rs:49 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 47 | keep | game_view::view::effect::alchemist::Dither::keep | in:game_view::view::effect::alchemist | fn(game_view::view::effect::alchemist::Dither, i32, i32) -> bool | game-view\src\view\effect\alchemist.rs:372 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 48 | keep | game_view::view::projectile::sand_mage::Dither::keep | in:game_view::view::projectile::sand_mage | fn(game_view::view::projectile::sand_mage::Dither, i32, i32) -> bool | game-view\src\view\projectile\sand_mage.rs:240 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 49 | max_range_can_use | game_ai::plan_legacy::old::max_range_can_use | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-ai\src\plan_legacy\old\battle.rs:2431 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 50 | max_range_nearly_can_use | game_ai::plan_legacy::old::max_range_nearly_can_use | pub | fn(&game_core::Entity, &game_core::Entity, usize) -> u64 | game-ai\src\plan_legacy\old\battle.rs:2397 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 51 | new | game_ai::SmallActionAround::new | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::OperationData, &game_core::PlayerState, usize, usize) -> game_ai::SmallActionAround | game-ai\src\small_action\around.rs:23 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 52 | new | game_ai::SmallActionRunAway::new | pub | fn(&game_core::OperationData, &game_core::PlayerState, usize) -> game_ai::SmallActionRunAway | game-ai\src\small_action\move_actions.rs:25 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 53 | new | game_ai::SmallActionAroundRegion::new | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::OperationData, &game_core::PlayerState, usize, usize) -> game_ai::SmallActionAroundRegion | game-ai\src\small_action\around.rs:517 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 54 | new_with_skill | game_ai::SmallActionRunAway::new_with_skill | pub | fn(&game_core::OperationData, &game_core::PlayerState, usize, bool) -> game_ai::SmallActionRunAway | game-ai\src\small_action\move_actions.rs:29 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 55 | nontarget_windup_perceived | game_ai::nontarget_windup_perceived | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity) -> bool | game-ai\src\utils.rs:553 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 56 | position_score_at_position | game_ai::position_score_at_position | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, u64, u64, game_ai::PositionEvalPurpose) -> game_core::PositioningScore | game-ai\src\position_eval.rs:1192 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 57 | positioning_accuracy | game_core::AthleteParameter::positioning_accuracy | pub | fn(&game_core::AthleteParameter) -> usize | game-core\src\simulation\state\player.rs:285 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 58 | range_adjust | game_core::Effect::range_adjust | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:29 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 59 | range_misjudge_rng | game_ai::range_misjudge_rng | pub | fn(usize, &game_core::OperationData, &game_core::PlayerState, usize) -> std::option::Option<game_core::NoiseRng> | game-ai\src\utils.rs:502 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 60 | range_misjudge_roll | game_ai::range_misjudge_roll | pub | fn(&mut rand::rngs::std::StdRng, &mut std::option::Option<game_core::NoiseRng>, u64, u64) -> u64 | game-ai\src\utils.rs:516 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 61 | remain_action_time | game_core::Entity::remain_action_time | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1616 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 62 | score | game_ai::plan_legacy::sub_plan::EpicPokeSubPlan::score | pub | fn(&game_ai::plan_legacy::sub_plan::EpicPokeSubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 | game-ai\src\plan_legacy\sub_plan\epic_poke.rs:521 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 63 | team | game_view::ClientData::team | pub | fn(&game_view::ClientData, usize) -> std::option::Option<std::cell::Ref< game_core::Team>> | game-view\src\logic\client\data.rs:1651 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 64 | team | game_view::ClientDatabase::team | pub | fn(&game_view::ClientDatabase, usize) -> std::option::Option<&game_core::Team> | game-view\src\logic\client\data.rs:968 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 65 | team | game_core::PlayerAiContext::<'a, 'b, 'r>::team | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:583 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 66 | v27_objective_discipline_action | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v27_objective_discipline_action | pub | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, game_core::JungleType) -> std::option::Option<game_ai::SmallActionPlay> | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:505 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
</details>

⚠**미매칭 15개**: `drop  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `first`, `map_or`, `next  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 9개는 **전부 다른 함수**라 싣지 않는다`, `objective_entity`, `on_periodic_trajectory`, `on_trajectory`, `push  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 6개는 **전부 다른 함수**라 싣지 않는다`, `radius_eff`, `reserve_internal_or_panic`, `retain`, `roll`, `same_team`, `truncate`, `try_fold`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m12.ll:35242) · **형제 8개** (EpicPokeSubPlan)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::plan_legacy::sub_plan::EpicPokeSubPlan as std::clone::Clone>::clone | pub | game-ai\src\plan_legacy\sub_plan\epic_poke.rs:9 | True | fn(&game_ai::plan_legacy::sub_plan::EpicPokeSubPlan) -> game_ai::plan_legacy::sub_plan::EpicPokeSubPlan |
| 1 | <game_ai::plan_legacy::sub_plan::EpicPokeSubPlan as std::fmt::Debug>::fmt | pub | game-ai\src\plan_legacy\sub_plan\epic_poke.rs:9 | True | fn(&game_ai::plan_legacy::sub_plan::EpicPokeSubPlan, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 2 | <game_ai::plan_legacy::sub_plan::EpicPokeSubPlan as std::default::Default>::default | pub | game-ai\src\plan_legacy\sub_plan\epic_poke.rs:9 | True | fn() -> game_ai::plan_legacy::sub_plan::EpicPokeSubPlan |
| 3 | game_ai::plan_legacy::sub_plan::EpicPokeSubPlan::action_candidates | pub | game-ai\src\plan_legacy\sub_plan\epic_poke.rs:14 | False | fn(&mut game_ai::plan_legacy::sub_plan::EpicPokeSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 4 | game_ai::plan_legacy::sub_plan::EpicPokeSubPlan::get_move_action | in:game_ai::plan_legacy::sub_plan::epic_poke | game-ai\src\plan_legacy\sub_plan\epic_poke.rs:278 | False | fn(&mut game_ai::plan_legacy::sub_plan::EpicPokeSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 5 | game_ai::plan_legacy::sub_plan::EpicPokeSubPlan::action_candidates_old | pub | game-ai\src\plan_legacy\sub_plan\epic_poke.rs:426 | False | fn(&mut game_ai::plan_legacy::sub_plan::EpicPokeSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 6 | game_ai::plan_legacy::sub_plan::EpicPokeSubPlan::calculate_score_parameter_value | pub | game-ai\src\plan_legacy\sub_plan\epic_poke.rs:504 | True | fn(&game_ai::plan_legacy::sub_plan::EpicPokeSubPlan, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::ScoreParameter) |
| 7 | game_ai::plan_legacy::sub_plan::EpicPokeSubPlan::score | pub | game-ai\src\plan_legacy\sub_plan\epic_poke.rs:521 | False | fn(&game_ai::plan_legacy::sub_plan::EpicPokeSubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 |

**`open` 8건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 표기 불가 | DI 변수 `on_trajectory`(epic_poke.rs:299·254)가 +0x31(on_periodic_trajectory, tcx 정본) 로드에 붙어 있다 — 소스가 `ps.on_trajectory \|\| ps.on_periodic_trajectory` 를 어떤 순서·이름으로 썼는지는 column 부재로 표기 불가(동작은 두 bool 의 OR 로 확정: 41295~41304·44262~44273) | 3 |  |
| 1 | 표기 불가 | L336 `mr >= 1` 은 IR 에서 `roll×max_range_can_use > 999`(41756) 로 접혀 있다 — 소스가 `mr > 0` 인지 `mr >= 1` 인지 외연 동일(표기 불가) | 4 |  |
| 2 | 미탐색 | get_move_action L389 의 사거리 식이 obj(에픽) 의 attack_effect 에 champ 의 level·stat_buff.range 를 곱/합한다(42315~42372) — 소스 의도(헬퍼 인자 순서 실수 가능성)는 IR 로 판정 불가; 동작은 IR 그대로 기록 | 4 |  |
| 3 | 미탐색 | range_misjudge_rng 의 반환 {i64,i64}(jrng 16B) 내부 레이아웃(Option<NoiseRng> 판별) 미확인 — 본체에선 불투명하게 range_misjudge_roll 에 넘길 뿐 | 4 |  |
| 4 | 미탐색 | SmallActionAroundRegion::new(120B) 은 initializes 속성이 없어 생성자가 채우는 정확한 바이트 범위 미확인(tcxdict 레이아웃으로 0..0x30 + 0x75 태그 추정) — 자식 명세/런타임 갈림 오프셋으로 확인 필요 | 3 |  |
| 5 | 미탐색 | EffectType vtable 슬롯 이름(expected_heal/shield/buff/rush_effect/move_on_hit)은 divtable 94% 일치 정적 vtable(g02.ll anon.1131) 기준 — 런타임 Arc<dyn EffectType> 구현체가 같은 트레이트 순서를 쓰는 것은 확실하나 슬롯 의미 이름은 그 일치율에 의존 | 3 |  |
| 6 | 미탐색 | closure#2 Attack 가지에서 타워 사거리 안·비타워 대상일 때 `same_team(target, champ)` 값을 그대로 반환한다(76600~76623) — 공격 대상은 항상 적이라 실질 false 지만, Neutral 대상(정글몹)이 champ 와 같은 태그일 수 없어 사실상 항상 false 인지(정글몹은 L31 에서 이미 true 로 빠짐) 소스 표현은 미확인 | 4 |  |
| 7 | 미탐색 | _docs 개발자 주석에 EpicPokeSubPlan 전용 문구 없음(`epic_poke`/`EpicPoke` grep 0건) — 주석 교차검증 불가 | 4 |  |

**`notes` 2건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | old_actions 의 variant 집합(어떤 이동/공격 variant 가 들어오는지)은 action_candidates_old 자식 명세(r14) 소관 — 여기선 closure#2 가 Attack/Skill/Skill2/Ult 외 전부 버린다는 것만 확정 | 4 | 사실 서술 |
| 1 | 인라인 헬퍼 이름 추정: entity.rs:1127(same_team 류) · entity.rs:1482~1483(visible_state 조회) · projectile.rs:134(타겟형 투사체 판정) · effect.rs:26 부근(사거리 합산) — 이름은 rmeta 소스 부재로 미확인, 동작은 IR 로 확정 | 4 | 사실 서술 |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

