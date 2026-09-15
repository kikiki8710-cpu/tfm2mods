---

### `200` SerpenPokeSubPlan::action_candidates — 세르펜 포킹 서브플랜의 소액션 후보 생성: v27 규율액션 우선 → 구후보 중 공격형만 3단 필터 → 비면 이동후보(도주/추적/세르펜 배회)에서 최고점 1개

| 항목 | 값 |
|---|---|
| id | `serpen_poke__SerpenPoke__action_candidates` |
| 심볼 | `_RNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy8sub_plan11serpen_pokeNtB2_17SerpenPokeSubPlan17action_candidates` |
| 소스 | `game-ai\src\plan_legacy\sub_plan\serpen_poke.rs:15` |
| IR | `m14.ll` 13287~16897행 |
| 경로·가시성 | `game_ai::plan_legacy::sub_plan::SerpenPokeSubPlan::action_candidates` · **pub** |
| 계층 | 기타 |
| exe | `e7caf0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r16` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&mut game_ai::plan_legacy::sub_plan::SerpenPokeSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay>
```

**TLS 메모 절(`sig.tls` · 정정 경로 `/specs[200]/sig/tls/<키>`)**

- `name`: (직접 접점 없음)
- `role`: 간접 소비자/작성자(콜리 경유)
- `key`: -
- `layout`: -
- `invalidation`: -
- `call_conditions`: 본문 m14.ll:13287~16897 에 `LocalKey::with`/thread_local 심볼 0건(grep). TLS 메모를 가진 콜리의 **호출 순서**(런타임 미러 설계용): (1) L16 v27_objective_discipline_action (2) L20 action_candidates_old(Some 이 아닐 때만) (3) L94~ retain: 콜리 없음 (4) L200~204 retain(s3_0): score(...) 를 act_actions 원소마다 1회 (5) L298 position_score_at_position(champ.x, champ.y, Objective) (6) [L300 거짓일 때] L319 check_kill_die_tick(champ, near_enemies, []) (7) L218 act_actions 비었을 때만: L238 score(...) 를 move_actions 원소마다 1회 → L248 get_input(best.clone()) → [Move(x,y) 일 때] L256 position_score_at_position(x, y, Objective). 각 콜리가 어느 TLS(POS_EVAL_CACHE·HP_VALUE_MEMO 등)를 쓰는지는 콜리 명세 소관 — 여기서는 순서만 확정(추정 아님·IR 호출 순서 그대로)

<details><summary>인자 9개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | bumpalo::Vec<SmallActionPlay> (32B) | ptr@+0 · bump@+8 · cap@+0x10 · len@+0x18 (m14.ll:13842~13847 Vec::new_in 패턴으로 재확인). 원소 184B · 태그 @+0xb1 | 4 |
| 1 | 1 | self | &mut SerpenPokeSubPlan (0B ZST) | tcxdict: 필드 0·0B. 본문에서 self 는 retain(s3_0) 클로저 캡처(%60+0)와 score 호출 인자로만 전달되고 읽기/쓰기 0. action_candidates_old 에는 `ptr poison` 으로 넘김(m14.ll:13420) ⟹ 쓰기 표면 없음 | 3 |
| 2 | 2 | version | usize | 본 함수 직접 분기 0. 콜리(v27·_old·nontarget_windup·position_score·range_misjudge·check_kill_die_tick·score·get_input·Around*::new)로 그대로 전달 | 4 |
| 3 | 3 | rnd | &mut StdRng (320B) | 직접 read/write 0. range_misjudge_roll·check_kill_die_tick·Around/AroundRegion::new·score·get_input·v27·_old 로 전달(콜리가 소비) | 4 |
| 4 | 4 | player | &PlayerState (2528B, readonly) | info.team(+0x930)·info.position(+0x9c0)·info.parameter(+0x180 → positioning_accuracy) | 4 |
| 5 | 5 | data | &OperationData (24B, readonly) | cache(+0)·context(+8 → pool@+0=bump, setting@+8 → tick_per_second@+0x12f8)·blackboard(+0x10, [Blackboard;2]) | 4 |
| 6 | 6 | parameter | &ScoreParameter (5384B, readonly) | positioning_score(+0x9f0, PositioningScoreData 2760B) 만 직접 참조(position_score_at_position·get_input 인자). 나머지는 score/_old 로 전달 | 4 |
| 7 | 7 | team_plan | &TeamPlan | v27_objective_discipline_action 의 &self 로만 전달 | 4 |
| 8 | 8 | debug | &mut DebugFrameData (224B) | 직접 read/write 0. check_kill_die_tick·score·get_input 으로 전달 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
action_candidates(self: ZST, version, rnd, player, data, parameter, team_plan, debug) -> Vec<SmallActionPlay>
# 기호: team = player.info.team(+0x930) · enemy = 1-team · champ = data.cache.player_champion[team][player.position].unwrap() (L23) · dist²(a,b) = |ax-bx|²+|ay-by|² (u64) · E_champs = cache.player_champion[enemy] (Option<&Entity> 5칸 flatten) · visible_to(e, t) = (t is Neutral) || e.visible_state[t.idx]==Visible (entity.rs:1482 인라인)

L16: if let Some(a) = team_plan.v27_objective_discipline_action(version, rnd, player, data, JungleType::Serpen(5)) { L17: return vec_in(bump=data.context.pool)[a] }   # v27 sret 184B, 태그@+177 != 255 이면 Some
L20: old_actions = self.action_candidates_old(version, rnd, player, data, parameter)   # self 는 poison(ZST)
L23: champ = player_champion[team][position].unwrap()
L24~26: nearest_enemy_tower: Option<&Entity> = cache.iter_towers_without_nexus(enemy).filter(|t| t.can_target(+0x6b9) && t.block_target_tick(+0x6a0)==0).min_by_key(|t| dist²(t, champ))
L29~92: act_actions = old_actions.iter().filter(s0_0).map(|a| a.clone()).collect_in(bump)
  s0_0(a) [캡처 game, champ, &nearest_enemy_tower]:
    L32: if let Some(id)=a.target_id() [태그 15..18 만 Some, 페이로드+8] && let Some(t)=game.get_entity_by_id(id) && t.ty != Champion(13) { return true }   # 비챔피언(미니언·세르펜·타워) 대상은 무조건 통과
    L33: match a {
      Attack(15) L36: target=get_entity_by_id(a.target)? else false; L37: eff=champ.attack_effect.unwrap(); L38: if !eff.is_in_range(champ,target) →false; L38~39: if let Some(t)=nearest_enemy_tower { if t.attack_effect.unwrap().is_in_range(t, champ) && target.ty!=Tower(2) { return target.team == champ.team } }  return true   # 적 타워 사거리 안에서 타워 아닌 적(다른 팀)을 때리면 제거(어그로)
      Skill(16) L46: target? else false; L47: eff=champ.skill_effect.unwrap(); L48: !is_in_range→false; L48~49: (타워 어그로 조건 동일) 걸리면 target.team!=champ.team →false; L49: if eff.ty.expected_move_on_hit() || eff.ty.expected_rush_effect() { L50: return nearest_enemy_tower.map_or(true, |t| !t.attack_effect.unwrap().is_in_range_ex(t, target, t.x,t.y, target.x,target.y, 15000)) [s0_0::s2_0] }  return true   # 이동형 스킬은 대상이 타워 사거리+15000 밖일 때만
      Skill2(17) L61~66: 동일, eff = (champ.level>2 ? champ.skill2_effect : None).unwrap(), 클로저 s4_0
      Ult(18) L76~81: 동일, eff = (champ.level>4 ? champ.ult_effect : None).unwrap(), 클로저 s6_0
      그 외(RunAway..Trace·Stop·AroundPosition) → false }
L94: act_actions.retain(s2_0) [캡처 data, champ, player, &version]:
  s2_0(a): match a { Skill(16) L97 / Skill2(17) L130 / Ult(18) L163 → 아래; 그 외 → keep }
    L97: target = get_entity_by_id(a.target)? else remove; L98: if target.team != champ.team → keep (적 대상은 그대로)
    L100: eff = (Skill: champ.skill_effect / Skill2: level>2 ? skill2 : None / Ult: level>4 ? ult : None).unwrap()   # None 이면 unwrap 패닉 경로
    L101: has_heal = eff.ty.expected_heal(ctx, champ as &dyn AbstractEntity) != 0; L102: has_shield = expected_shield(..) != 0; L103: has_buff = expected_buff(..) is Some(sret 288B, 태그@+72 != -1)   # DW_OP_not 1회 = 극성 반전 · 분기 방향으로 확정
    L104~106: near_enemy = E_champs.any(|e| dist²(e,target) <= 120000² && visible_to(e, champ.team)) || cache.iter_towers(enemy).any(같은 술어) [s2_0::s_0] || cache.jungles.any(같은 술어)
    L107: hp_ratio = target.hp*100 / target.stat_cached.hp(max)   # max 0 이면 div_by_zero 패닉
    L108~114 (IR 동치 재구성): keep = ( !(has_heal && !has_buff && hp_ratio > 79) || aoe_heal_covers_low_ally(version, eff, data.cache, player, target) ) && ( !has_shield || has_buff || near_enemy )
      # 즉 ①건강한 아군(>79%)에 순수 힐(버프 없음)은 AoE 힐이 저체력 아군을 덮을 때만 ②실드형(버프 없음)은 근처(120000) 적이 있을 때만. 힐만 있고 실드·버프 없는 경우는 hp_ratio<=79 이면 keep(near_enemy 무관)
L200: act_actions.retain(|a| self.score(version, parameter, rnd, player, data, a, debug) >= -30)  [s3_0, L201~202]
L205: move_actions = self.get_move_actions(version, rnd, player, data, &parameter.positioning_score, debug)   # 인라인, 본문 L280~416:
  L280: res = Vec::new_in(bump); L282: champ 재unwrap
  L285~291: has_non_target_action_range = E_champs.any(|c| nontarget_windup_perceived(version, player, data, c) && c.ty==Champion && match c.action_state { Skill(4)=>c.skill_effect, Skill2(5)=>(level>2?skill2:None), Ult(6)=>(level>4?ult:None), _=>continue }.unwrap().is_in_range(c, champ))
  L298: ps = position_score_at_position(version, player, data, positioning_score, champ.x, champ.y, Objective(11))
  L300: if ps.on_trajectory(+0x30) || ps.on_periodic_trajectory(+0x31) || has_non_target_action_range { L302: res.push(RunAway::new_with_skill(data, player, 5, true) as RunAway(3)); L303: return res }
  L313: runaway = false; L314: pa = player.parameter.positioning_accuracy(); min_v = pa; L316: max_v = 2000 - pa
  L318: near_enemies = E_champs.filter(|e| visible_to(e, champ.team) && dist²(e,champ) < 160000²).collect_in(bump)   [get_move_actions::closure0]
  L319: me_die_tick = check_kill_die_tick(version, rnd, data, player, champ, near_enemies.clone(), Vec::new_in(bump), debug)
  L322~323: objective_entity = game.get_game_mode().as_moba().unwrap()  [비Moba → unwrap 패닉] .jungle_runner.serpen.live_list.first().and_then(|id| game.get_entity_by_id(id))
  L325: for enemy in near_enemies {
    L326: jrng = range_misjudge_rng(version, data, player, enemy.id); L327: mr = max_range_can_use(champ, enemy) * range_misjudge_roll(rnd, &jrng, min_v, max_v) / 1000; L328: emr = max_range_can_use(enemy, champ) * roll / 1000; L329: emr_near = max_range_nearly_can_use(enemy, champ, 40) * roll / 1000
    L330: dist = dist²(champ, enemy); L333: near_obj = objective_entity.map_or(true, |o| dist²(enemy, o) < 200000²+1)
    L336: if me_die_tick < ctx.setting.tick_per_second {   # 1초 안에 죽는 위급
      L351: emr2 = max_range_nearly_can_use(enemy, champ, 60) * roll / 1000
      L353: if dist > mr² && emr2 < mr { L355: if near_obj { L356: res.push(Trace::new(data, enemy.id, 5) as Trace(14)) } }   # 내가 더 길고 아직 못 닿음 → 추적
      else { L358: runaway |= dist <= emr2² }
    } else L337: if enemy.remain_action_time() > 10 && mr >= 1 {   # 999 비교 = roll*range > 999
      L338: if dist > mr² && near_obj { L339: res.push(Trace(enemy.id, 5)) }
    } else L341: if emr < mr && dist > mr² { L343: if near_obj { L344: res.push(Trace(enemy.id, 5)) } }
    else { L346: runaway |= dist <= emr_near² }
  }
  L365~368: for e in cache.others[enemy] { if let Some(atk)=e.attack_effect { range = e.stat_buff_cached.range + atk.range + atk.growth_range*(e.level-1) + atk.range_adjust(e, champ) + radius_adj(e) + radius_adj(champ)  [radius_adj = mult==0 ? radius : radius*(mult+100)/100]; if dist²(champ,e) <= range² { runaway = true; break } } }
  L377: if res.is_empty() {
    L378: serpen = get_game_mode().as_moba().unwrap().serpen.live_list.first().and_then(get_entity_by_id)
    match serpen { None → L398: res.push(AroundRegion::new(version, rnd, data, player, 2, 5) as AroundRegion(7))
      Some(s) if s.ty != Serpen(6) → L395: res.push(AroundRegion(2, 5))
      Some(s) → L380: if visible_to(s, champ.team) && dist²(s, champ) <= 150000² {
          L383: atk = s.attack_effect.unwrap(); range = atk.range + 20000 + champ.stat_buff_cached.range + atk.growth_range*(champ.level-1) + radius_adj(champ) + radius_adj(s)   # ★champ.level·champ.stat_range 와 s.atk.range 혼합(IR 그대로)
          L384: dmg = atk.expected_damage_target(ctx, caster=s as &dyn AbstractEntity, target=champ)
          L386: if dmg*2 < champ.hp || range² < dist²(champ,s) { L389: res.push(Around::new(version, rnd, data, player, s.id, 5) as Around(5)) } else { L387: res.push(RunAway::new(data, player, 5) as RunAway(3)) }
        } else { L381: res.push(Around::new(version, rnd, data, player, s.id, 5)) } }
    L401: if game.is_visible(enemy, champ.id) && ( L403: E_champs.any(|c| c.team != champ.team && dist²(c,champ) < 150000²+1) || L404: cache.others[enemy].any(|e| dist²(e,champ) < 150000²+1) ) { L406: res.push(RunAway::new(data, player, 5)) }
  }
  L411: if runaway { L412: res.push(RunAway::new_with_skill(data, player, 5, false)) }
  L415: return res
L208~213: if let Some(t)=nearest_enemy_tower && t.ty==Tower(2) && let Some((_, id))=t.tower.nearest_enemy(+0x88 tag, +0x98 id) && id == champ.id { act_actions.truncate(0); move_actions.truncate(0); move_actions.push(RunAway::new_with_skill(data, player, 5, true)) }
L218: if act_actions.is_empty() {
  L219: pa = positioning_accuracy(); min_v = pa; L221: max_v = 2000 - pa
  L225: if E_champs.any(|c| { L226: jrng = range_misjudge_rng(version, data, player, c.id); L227: emr = max_range_can_use(c, champ)*roll(rnd,&jrng,min_v,max_v)/1000 + 10000; L228: mr = max_range_can_use(champ, c); L230: data.blackboard[team].is_recent_visible(game, player, c) && L231: dist²(c,champ) <= emr² && L232: (c.ty != Champion || c.action_state ∈ {Idle,Return,Move}) && !c.block_input() && L234: mr == 0 })   # 적이 나를 때릴 수 있고 자유롭고 나는 못 닿음 (마지막 두 항의 소스 순서는 컬럼 부재로 미확정)
    { L235: return vec![RunAway::new_with_skill(data, player, 5, false)] }
  L238: best = move_actions.iter().max_by_key(|m| self.score(version, parameter, rnd, player, data, m, debug)).unwrap()   # move_actions 비면 unwrap 패닉(get_move_actions 는 항상 ≥1 push 하므로 정상 경로엔 없음)
  L242~247: trajectory_possible = game.iter_projectile().any(|p| p.team != champ.team && !matches!(p.move_type, Target|TargetSplash) && !(BouncingTarget 이면서 첫 워드==1 [projectile.rs:134 인라인, 의미 미확정]) && dist²(champ, p) < 420000²) || E_champs.any(|c| matches!(c.rush_state, Rush|RushPenetrate))
  if trajectory_possible {
    L248: input = best.clone().get_input(version, rnd, player, data, positioning_score, debug)
    L252: match input { Some(Move{x,y}) → L256: ps = position_score_at_position(version, player, data, positioning_score, x, y, Objective); L258: on_traj = ps.on_trajectory || ps.on_periodic_trajectory; L260: if on_traj { L261: return vec![RunAway::new_with_skill(data, player, 5, false)] } else { L263: return vec![best.clone()] }
                  Some(그 외) → L266: return vec![best.clone()]; None → L269: return vec![best.clone()] }
  } else { L269: return vec![best.clone()] }
} else { L274: return act_actions }
```

**`mem` 메모리 접근 58건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | L23·L104(aux). <2 bounds 체크(panic_bounds_check) | 4 | OK |  |
| 1 | PlayerState | 0x9c0 | info.position@tag | r | L23 i32 → player_champion[team][position] | 4 | OK |  |
| 2 | PlayerState | 0x180 | info.parameter | r | L219/L314 AthleteParameter::positioning_accuracy(&self) | 4 | OK |  |
| 3 | OperationData | 0x0 | cache | r | &AbstractGameWithCache | 4 | OK |  |
| 4 | OperationData | 0x8 | context | r | &GameContext → +0 pool(&Bump, Vec 할당자) · +8 setting | 4 | OK |  |
| 5 | OperationData | 0x10 | blackboard | r | L230 &[Blackboard;2][player.team].is_recent_visible | 4 | OK |  |
| 6 | GameSetting | 0x12f8 | tick_per_second | r | L336 me_die_tick < tps (gep 4856) | 4 | OK |  |
| 7 | AbstractGameWithCache | 0x0 | game(data ptr) | r | &dyn AbstractGame 팻포인터 data 절반 — vtable 호출 인자 | 4 | OK |  |
| 8 | AbstractGameWithCache | 0x8 | game(vtable ptr) | r | 슬롯 +0x40 get_game_mode · +0xf8 is_visible · +0x1f0 get_entity_by_id · +0x210 iter_projectile (divtable AbstractGame) | 3 | OK |  |
| 9 | AbstractGameWithCache | 0x1e0 | player_champion[2][5] | r | gep 480: [team][position] Option<&Entity>(니치 null). 적 팀 배열은 L285·L225·L403·L104(aux) 에서 5칸 언롤 | 4 | OK |  |
| 10 | AbstractGameWithCache | 0xf0 | others[team] | r | gep 240 + 32*team: bumpalo Vec<&Entity>(ptr@0 · len@+0x18). L365·L404 적 팀 비챔피언 엔티티 | 4 | OK |  |
| 11 | AbstractGameWithCache | 0xd0 | jungles | r | aux s2_0 L106: gep 208 ptr · 232 len — 정글 엔티티 Vec<&Entity> | 4 | OK |  |
| 12 | MobaMode | 0x1d0 | jungle_runner.serpen.live_list.ptr | r | L322/L378 gep 464: 첫 원소(usize id) → get_entity_by_id | 4 | OK |  |
| 13 | MobaMode | 0x1d8 | jungle_runner.serpen.live_list.len | r | gep 472: 0 이면 objective_entity/serpen = None | 4 | OK |  |
| 14 | ScoreParameter | 0x9f0 | positioning_score | r | gep 2544 → &PositioningScoreData(2760B) | 4 | OK |  |
| 15 | PositioningScore | 0x30 | on_trajectory | r | gep 48 (L300·L258) | 4 | OK |  |
| 16 | PositioningScore | 0x31 | on_periodic_trajectory | r | gep 49 (L300·L258) | 4 | OK |  |
| 17 | Entity | 0x0 | team@tag | r | TeamType: 0=Player(+8 = usize) 1=Neutral. Neutral 이면 가시성 검사 생략(true) | 4 | OK |  |
| 18 | Entity | 0x8 | team@Player.0 | r | 팀 번호 비교(L39·L98·L243·L403) | 4 | OK |  |
| 19 | Entity | 0x38 | visible_state[2] | r | stride 24 · tag 0=Visible. is_visible_to(team) 인라인(entity.rs:1482~1483) | 4 | OK |  |
| 20 | Entity | 0x68 | ty@tag | r | EntityType: 2=Tower · 6=Serpen · 13=Champion | 4 | OK |  |
| 21 | Entity | 0x70 | ty@Champion.action_state@tag | r | ChampionActionState: 4=Skill 5=Skill2 6=Ult (L287~291) · <3(Idle/Return/Move) (L232) | 4 | OK |  |
| 22 | Entity | 0x88 | ty@Tower.info.nearest_enemy@tag | r | L210 Option 태그(bit0) | 4 | OK |  |
| 23 | Entity | 0x98 | ty@Tower.info.nearest_enemy@Some.0.1 | r | L210 타워 타깃 id == champ.id | 4 | OK |  |
| 24 | Entity | 0x308 | rush_state@tag | r | L246: >=0(RushPenetrate, 암묵) 또는 i64::MIN+3(Rush) | 4 | OK |  |
| 25 | Entity | 0x438 | stat_buff_cached.range | r | gep 1080 — L367·L383 사거리 합산 | 4 | OK |  |
| 26 | Entity | 0x470 | stat_buff_cached.radius_mult | r | gep 1136 i32 — 0 이면 radius 그대로, 아니면 radius*(100+mult)/100 | 4 | OK |  |
| 27 | Entity | 0x490 | attack_effect | r | gep 1168 &Effect(Option 니치 = casting tag @+0x4c0 == -1 → None) | 4 | OK |  |
| 28 | Entity | 0x4a0 | attack_effect.range | r | gep 1184 | 4 | OK |  |
| 29 | Entity | 0x4a8 | attack_effect.growth_range | r | gep 1192 × (level-1) | 4 | OK |  |
| 30 | Entity | 0x4c0 | attack_effect@tag | r | gep 1216 i32 -1=None | 4 | OK |  |
| 31 | Entity | 0x4c8 | skill_effect | r | gep 1224 (aux s0_0 L47·s2_0 L100). ty 팻포인터 = +0x4c8 ptr/+0x4d0 vtable | 4 | OK |  |
| 32 | Entity | 0x4d0 | skill_effect.ty(vtable) | r | aux: gep 1232 → dyn EffectType 슬롯 +0x40 expected_heal · +0x48 expected_shield · +0x50 expected_buff · +0x60 expected_rush_effect · +0x68 expected_move_on_hit (divtable EffectType, g02.ll 정적 vtable 94% 일치) | 3 | OK |  |
| 33 | Entity | 0x4f8 | skill_effect@tag | r | gep 1272 i32 -1=None (L287) | 4 | OK |  |
| 34 | Entity | 0x500 | skill2_effect | r | gep 1280 — level>2 일 때만, 아니면 정적 None(@anon.22) → unwrap 패닉 경로(entity.rs:1693) | 4 | OK |  |
| 35 | Entity | 0x538 | ult_effect | r | gep 1336 — level>4 일 때만(entity.rs:1701) | 4 | OK |  |
| 36 | Entity | 0x5c0 | id | r | gep 1472 — Trace 대상·range_misjudge_rng 키·is_visible 인자 | 4 | OK |  |
| 37 | Entity | 0x5c8 | level | r | gep 1480 | 4 | OK |  |
| 38 | Entity | 0x628 | stat_cached.hp(max) | r | aux s2_0 L107: hp_ratio 분모(0 이면 div_by_zero 패닉) | 4 | OK |  |
| 39 | Entity | 0x660 | x | r | gep 1632 | 4 | OK |  |
| 40 | Entity | 0x668 | y | r | gep 1640 | 4 | OK |  |
| 41 | Entity | 0x670 | hp | r | gep 1648 — L386 dmg*2 < champ.hp · aux L107 hp_ratio 분자 | 4 | OK |  |
| 42 | Entity | 0x680 | radius | r | gep 1664 | 4 | OK |  |
| 43 | Entity | 0x6a0 | block_target_tick | r | gep 1696 (aux closure0: ==0) | 4 | OK |  |
| 44 | Entity | 0x6b9 | can_target | r | gep 1721 (aux closure0) | 4 | OK |  |
| 45 | Projectile | 0x0 | team@tag/+8 | r | L243 p.team != champ.team | 4 | OK |  |
| 46 | Projectile | 0x40 | move_type@tag | r | gep 64 — ProjectileMoveType 니치(tag 6=Target 7=TargetSplash 는 제외, tag<=1 → BouncingTarget) | 4 | OK |  |
| 47 | Projectile | 0x100 | x | r | gep 256 | 4 | OK |  |
| 48 | Projectile | 0x108 | y | r | gep 264 | 4 | OK |  |
| 49 | SmallActionPlay | 0xb1 | tag | r | gep 177 — 태그 읽기(L16 v27 None=255 · aux L32/33/95 switch) | 4 | OK |  |
| 50 | SmallActionPlay | 0x8 | Attack/Skill/Skill2/Ult.target | r | aux: target_id() 페이로드(태그 15..18 일 때만 Some) | 4 | OK |  |
| 51 | bumpalo::Vec | 0x18 | len | r | gep 24 — old_actions/act_actions/move_actions/near_enemies len | 4 | 확인불가(★모호: 동명 def_path 2개 [("bumpalo::collecti) |  |
| 52 | Input(Option) | 0x0 | tag | r | L252 get_input sret: -1=None(get_input 본문 m11.ll:42814~ 에서 `store i64 -1` 확인) · 0=Move{x@+8,y@+16} | 4 | OK |  |
| 53 | (sret) Vec<SmallActionPlay> | 0x0 | ptr/bump/cap/len | w | 반환 경로 8개: L17·L235·L261·L263·L266·L269(from_iter_in 1원소) · L274(act_actions 통째) | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) | from_iter_in 결과 32B memcpy 또는 act_actions 32B memcpy(L274) |
| 54 | act_actions(로컬 Vec %63) | 0x18 | len | w | L94 retain(s2_0) · L200 retain(s3_0) · L211 truncate | 4 | 확인불가(tcx 사전에 타입 없음) | retain×2 로 감소 · L211 truncate(0) |
| 55 | move_actions(로컬 Vec %59/%40) | 0x18 | len | w | get_move_actions 내부 res 에 push(Trace/Around/AroundRegion/RunAway) · L213 push(RunAway with_skill=true) | 4 | 확인불가(tcx 사전에 타입 없음) | push 로 증가 · L212 truncate(0) |
| 56 | self | - | (없음) | w | ZST(0B). IR 속성에 initializes 없음 · self 경유 store 0건 ⟹ `&mut self` 쓰기 표면 = 공집합 | 4 | 확인불가(오프셋 파싱 실패) | - |
| 57 | rnd/debug | - | (콜리 경유) | w | 본문 직접 store 0건. 변경은 range_misjudge_roll·Around*::new·check_kill_die_tick·score·get_input·v27·_old 내부(각 콜리 명세 참조) | 4 | 확인불가(오프셋 파싱 실패) | - |

**`consts` 상수 30건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 5 | 16 | 태그 | JungleType::Serpen(tcxdict 태그 5) — v27_objective_discipline_action 의 target(i8). 같은 5 가 usize 로 Trace/Around/AroundRegion/RunAway::new 의 end_delay(L302·L339·L344·L356·L381·L387·L389·L395·L398·L406·L412·L213·L235·L261) 에도 쓰임. shl 피연산자 아님 | 3 |
| 1 | 2 | 209 | 태그 | EntityType::Tower 태그(L209 nearest_tower.ty==2 · aux L38/48/63/78 target.ty!=2) 및 AroundRegion::new 의 target_region=2(L395·L398). aux s2_0/s0_0 의 level>2 는 skill2_effect 가용 게이트. shl 피연산자 아님 | 4 |
| 2 | 13 | 286 | 태그 | EntityType::Champion 태그 — L286 nontarget any · L232 !is_champion · aux L32 비챔피언 대상 조기 true | 4 |
| 3 | 6 | 379 | 태그 | EntityType::Serpen 태그(L379) · ChampionActionState::Ult 태그(L291 switch). shl 피연산자 아님 | 4 |
| 4 | 4 | 287 | 태그 | ChampionActionState::Skill 태그(L287 switch case) · aux level>4 = ult_effect 가용 게이트. shl 피연산자 아님 | 4 |
| 5 | 3 | 232 | 태그 | ChampionActionState 태그 <3 (Idle/Return/Move = 캐스팅 중 아님, entity.rs:1548 인라인) · SmallActionPlay::RunAway 메모리태그 3(store i8 3). shl 피연산자 아님(s2_0 의 shl 3 은 Vec 인덱스 stride) | 4 |
| 6 | 14 | 339 | 태그 | SmallActionPlay::Trace 메모리태그(store i8 14, L339/344/356) | 4 |
| 7 | 7 | 395 | 태그 | SmallActionPlay::AroundRegion 메모리태그(store i8 7, L395/398) | 4 |
| 8 | 11 | 298 | 태그 | PositionEvalPurpose::Objective 메모리태그(i8 11) — position_score_at_position 의 purpose(L298·L256) | 4 |
| 9 | 2000 | 316 | 계수 | max_v = 2000 − positioning_accuracy (range_misjudge_roll 상한, L316·L221) | 4 |
| 10 | 1000 | 327 | 계수 | range_misjudge_roll 결과(‰)를 사거리에 곱한 뒤 /1000 (L327·328·329·351·227) | 4 |
| 11 | 40 | 329 | 태그 | max_range_nearly_can_use(enemy, champ, 40) 의 3번째 인자(쿨 여유 tick, L329) | 4 |
| 12 | 60 | 351 | 미상 | max_range_nearly_can_use(enemy, champ, 60) — me_die_tick<tps 인 위급 분기(L351) | 4 |
| 13 | 10 | 337 | 임계 | enemy.remain_action_time() > 10 (L337) | 4 |
| 14 | 999 | 337 | 임계 | roll*max_range_can_use(champ,enemy) > 999 ⟺ mr(/1000 후) ≥ 1, 즉 '내가 닿을 사거리가 0 이 아님'(L337, 나눗셈 전 값으로 비교) | 4 |
| 15 | 10000 | 227 | 미상 | emr = 적 사거리(오판 롤 적용) + 10000 여유(L227) | 4 |
| 16 | 20000 | 383 | 계수 | 세르펜 공격 사거리 합산에 +20000 여유(L383) | 4 |
| 17 | 100 | 367 | 계수 | radius*(radius_mult+100)/100 (entity.rs:1515 인라인, L367·L383) | 4 |
| 18 | 40000000001 | 333 | 임계 | 200000²+1 — 적이 오브젝트(세르펜) 200000(6.25셀) 이내에 있을 때만 Trace 발행(L333) | 4 |
| 19 | 22500000000 | 380 | 임계 | 150000² — 세르펜이 150000 보다 멀면 Around(serpen) (L380, ugt 비교) | 4 |
| 20 | 22500000001 | 403 | 임계 | 150000²+1 — 적 챔피언/비챔피언이 150000 이내(L403·L404) | 4 |
| 21 | 176400000000 | 244 | 임계 | 420000² — 적 투사체가 420000(13.1셀) 이내면 trajectory_possible(L244) | 4 |
| 22 | -9223372036854775805 | 246 | 센티널 | i64::MIN+3 = RushState::Rush 메모리태그(tcxdict 니치). >=0 은 RushPenetrate(암묵) — 둘 중 하나면 trajectory_possible(L246) | 3 |
| 23 | -1 | 16 | 센티널 | Option 니치 None: v27 sret 태그(i8 255, L16) · Effect casting tag(i32) · get_input sret 태그(i64, L252) | 4 |
| 24 | 14400000001 | 104 | 미상 | (aux s2_0) 120000²+1 — 아군 대상 근처 적 챔피언/타워/정글 탐지 반경(L104~106·137~139·170~172) | 4 |
| 25 | 79 | 108 | 미상 | (aux s2_0) hp_ratio(%) > 79 = 건강한 아군 → 힐은 aoe_heal_covers_low_ally 일 때만 | 4 |
| 26 | 15000 | 51 | 미상 | (aux s0_0::s2_0/s4_0/s6_0) is_in_range_ex 의 사거리 여유 — 이동형 스킬 대상이 적 타워 사거리+15000 안이면 제거 | 4 |
| 27 | -15 | 32 | 태그 | (aux s0_0) tag-15 <4 ⟺ 태그 15..18(Attack/Skill/Skill2/Ult) = target_id() Some (small_action.rs:309 인라인) | 4 |
| 28 | -30 | 202 | 미상 | (aux s3_0) self.score(...) < -30 이면 후보 제거 | 4 |
| 29 | 25600000000 | 318 | 미상 | (aux get_move_actions closure0) 160000²(5셀) — near_enemies 반경(ult 비교) | 4 |

**`knobs` 조정점 14건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 세르펜 포킹 Trace 발행 허용 반경(적↔세르펜) | serpen_poke.rs:333 | 40000000001 | 올리면 세르펜에서 더 먼 적까지 추적(포킹 이탈 증가), 내리면 세르펜 근처 적만 추적 | 4 | 기존 |
| 1 | 세르펜 접근 판정 거리 | serpen_poke.rs:380 | 22500000000 | 올리면 더 먼 세르펜도 '가까움'으로 보고 딜/사거리 판정(RunAway/Around 선택)으로 들어감 | 4 | 기존 |
| 2 | 세르펜 공격 사거리 여유 | serpen_poke.rs:383 | 20000 | 올리면 세르펜 사거리를 더 넓게 봐 RunAway 가 늘고 Around(접근) 가 줆 | 4 | 기존 |
| 3 | 세르펜 피해 도주 임계(dmg*2 < hp) | serpen_poke.rs:386 | 2 | shl 1 로 접힘(dmg<<1). 배수를 올리면 더 약한 세르펜 공격에도 RunAway | 4 | 기존 |
| 4 | 가시 상태에서 적 근접 도주 반경 | serpen_poke.rs:403~404 | 22500000001 | 올리면 더 먼 적에도 RunAway 후보가 생김(안전 우선) | 4 | 기존 |
| 5 | near_enemies 반경 | serpen_poke.rs:318 | 25600000000 | 올리면 더 먼 적까지 check_kill_die_tick·사거리 비교에 포함 | 4 | 기존 |
| 6 | 적 사거리 여유(emr+10000) | serpen_poke.rs:227 | 10000 | 올리면 '적이 나를 때릴 수 있음' 판정이 넓어져 L235 RunAway 조기 반환 증가 | 4 | 기존 |
| 7 | 후보 점수 하한 | serpen_poke.rs:202 | -30 | 올리면(예 -10) 공격 후보가 더 많이 잘려 이동 후보로 넘어감 | 4 | 기존 |
| 8 | 아군 대상 스킬 낭비 필터 반경 | serpen_poke.rs:104~106 | 14400000001 | 올리면 근처 적 판정이 넓어져 실드/힐 낭비 필터가 덜 걸림 | 4 | 기존 |
| 9 | 건강한 아군 힐 차단 HP% | serpen_poke.rs:108 | 79 | 올리면(예 90) 더 많은 아군에게 힐을 허용 | 4 | 기존 |
| 10 | 이동형 스킬 타워 사거리 여유 | serpen_poke.rs:51/66/81 | 15000 | 올리면 타워 근처 대상에게 돌진형 스킬을 덜 씀 | 4 | 기존 |
| 11 | 위급 판정 tick(me_die_tick < tps) | serpen_poke.rs:336 | tick_per_second | 게임 세팅값. 코드 상수 아님 — 바꾸려면 비교식을 패치 | 4 | 기존 |
| 12 | 투사체 회피 감지 반경 | serpen_poke.rs:244 | 176400000000 | 올리면 더 먼 투사체에도 get_input 재평가(도주 전환) 수행 | 4 | 기존 |
| 13 | max_range_nearly_can_use 쿨 여유 | serpen_poke.rs:329/351 | 40 / 60 | 올리면 곧 쓸 수 있는 적 스킬까지 사거리로 봐 runaway 가 늘어남 | 4 | 기존 |

<details><summary>`callees` 피호출자 73건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | action_candidates | game_ai::plan_legacy::sub_plan::SubPlan::action_candidates | pub | fn(&mut game_ai::plan_legacy::sub_plan::SubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\mod.rs:118 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 18개 중 상위 3개 |
| 1 | action_candidates | game_ai::plan_legacy::sub_plan::HideSubPlan::action_candidates | pub | fn(&mut game_ai::plan_legacy::sub_plan::HideSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\hide.rs:19 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 18개 중 상위 3개 |
| 2 | action_candidates | game_ai::plan_legacy::sub_plan::StealSubPlan::action_candidates | pub | fn(&mut game_ai::plan_legacy::sub_plan::StealSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\steal.rs:33 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 18개 중 상위 3개 |
| 3 | action_candidates_old | game_ai::plan_legacy::sub_plan::SerpenPokeSubPlan::action_candidates_old | pub | fn(&mut game_ai::plan_legacy::sub_plan::SerpenPokeSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\serpen_poke.rs:418 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | aoe_heal_covers_low_ally | game_ai::aoe_heal_covers_low_ally | pub | fn(usize, &game_core::Effect, &game_core::OperationData, &game_core::PlayerState, &game_core::Entity) -> bool | game-ai\src\buff_value.rs:543 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 5 | as_moba | game_core::GameMode::<'a>::as_moba | pub | fn(game_core::GameMode<'a/#0>) -> std::option::Option<&'a/#0 game_core::MobaMode> | game-core\src\simulation\game.rs:230 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 6 | block_input | game_core::Entity::block_input | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1501 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 7 | can_target | game_core::Entity::can_target | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1477 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 8 | can_target | game_core::OperationData::<'a, 'b>::can_target | pub | fn(&game_core::OperationData<'a/#0, 'b/#1>, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\state\player.rs:23 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 9 | check_kill_die_tick | game_ai::check_kill_die_tick | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::OperationData, &game_core::PlayerState, &game_core::Entity, bumpalo::collections::vec::Vec< &game_core::Entity>, bumpalo::collections::vec::Vec< &game_core::Entity>, &mut game_core::DebugFrameData) -> usize | game-ai\src\fight_check.rs:917 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 10 | expected_buff | game_core::EffectType::expected_buff | pub | fn(&Self/#0, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + ) -> std::option::Option<game_core::BuffState> | game-core\src\simulation\effect\type.rs:287 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 18개 중 상위 3개 |
| 11 | expected_buff | <game_core::RangeEffect as game_core::EffectType>::expected_buff | pub | fn(&game_core::RangeEffect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + ) -> std::option::Option<game_core::BuffState> | game-core\src\simulation\effect\type\range_effect.rs:94 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 18개 중 상위 3개 |
| 12 | expected_buff | <game_core::AddBuffEffect as game_core::EffectType>::expected_buff | pub | fn(&game_core::AddBuffEffect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + ) -> std::option::Option<game_core::BuffState> | game-core\src\simulation\effect\type\add_buff.rs:28 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 18개 중 상위 3개 |
| 13 | expected_damage_target | game_core::Effect::expected_damage_target | pub | fn(&game_core::Effect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + , &game_core::Entity) -> usize | game-core\src\simulation\effect.rs:91 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 14 | expected_heal | game_ai::expected_heal | pub | fn(&game_core::GameContext, &game_core::Entity, &game_core::Entity) -> usize | game-ai\src\fight_check.rs:266 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 31개 중 상위 3개 |
| 15 | expected_heal | game_core::EffectType::expected_heal | pub | fn(&Self/#0, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + ) -> usize | game-core\src\simulation\effect\type.rs:283 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 31개 중 상위 3개 |
| 16 | expected_heal | <game_core::HealEffect as game_core::EffectType>::expected_heal | pub | fn(&game_core::HealEffect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + ) -> usize | game-core\src\simulation\effect\type\heal.rs:186 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 31개 중 상위 3개 |
| 17 | expected_move_on_hit | game_core::EffectType::expected_move_on_hit | pub | fn(&Self/#0) -> bool | game-core\src\simulation\effect\type.rs:293 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 20개 중 상위 3개 |
| 18 | expected_move_on_hit | <game_core::CombineEffect as game_core::EffectType>::expected_move_on_hit | pub | fn(&game_core::CombineEffect) -> bool | game-core\src\simulation\effect\type\combine.rs:96 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 20개 중 상위 3개 |
| 19 | expected_move_on_hit | <game_core::DelayedEffect as game_core::EffectType>::expected_move_on_hit | pub | fn(&game_core::DelayedEffect) -> bool | game-core\src\simulation\effect\type\delayed.rs:45 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 20개 중 상위 3개 |
| 20 | expected_rush_effect | game_core::EffectType::expected_rush_effect | pub | fn(&Self/#0) -> bool | game-core\src\simulation\effect\type.rs:291 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 9개 중 상위 3개 |
| 21 | expected_rush_effect | <game_core::RushEffect as game_core::EffectType>::expected_rush_effect | pub | fn(&game_core::RushEffect) -> bool | game-core\src\simulation\effect\type\rush.rs:53 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 9개 중 상위 3개 |
| 22 | expected_rush_effect | <game_core::CombineEffect as game_core::EffectType>::expected_rush_effect | pub | fn(&game_core::CombineEffect) -> bool | game-core\src\simulation\effect\type\combine.rs:42 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 9개 중 상위 3개 |
| 23 | expected_shield | game_ai::expected_shield | pub | fn(&game_core::GameContext, &game_core::Entity, &game_core::Entity) -> usize | game-ai\src\fight_check.rs:284 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 22개 중 상위 3개 |
| 24 | expected_shield | game_core::EffectType::expected_shield | pub | fn(&Self/#0, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + ) -> usize | game-core\src\simulation\effect\type.rs:285 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 22개 중 상위 3개 |
| 25 | expected_shield | <game_core::RangeEffect as game_core::EffectType>::expected_shield | pub | fn(&game_core::RangeEffect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + ) -> usize | game-core\src\simulation\effect\type\range_effect.rs:90 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 22개 중 상위 3개 |
| 26 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 27 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 28 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 29 | get_game_mode | game_core::AbstractGame::get_game_mode | pub | fn(&Self/#0) -> game_core::GameMode | game-core\src\simulation.rs:78 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 30 | get_game_mode | <game_core::Game as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::Game) -> game_core::GameMode | game-core\src\simulation\game.rs:1707 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 31 | get_game_mode | <game_core::SingleLaneGame as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::SingleLaneGame) -> game_core::GameMode | game-core\src\simulation\game.rs:4248 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 32 | get_input | game_ai::SmallActionPlay::get_input | pub | fn(&mut game_ai::SmallActionPlay, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &mut game_core::DebugFrameData) -> std::option::Option<game_core::Input> | game-ai\src\small_action.rs:157 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 33 | is_empty | game_core::PatchSetting::is_empty | pub | fn(&game_core::PatchSetting) -> bool | game-core\src\patch_engine.rs:853 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 34 | is_empty | game_core::ModAiRegistry::is_empty | pub | fn(&game_core::ModAiRegistry) -> bool | game-core\src\mod_ai.rs:15 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 35 | is_empty | game_core::PublicChampionEvidence::is_empty | pub | fn(&game_core::PublicChampionEvidence) -> bool | game-core\src\banpick\public_evidence.rs:96 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 36 | is_in_range | game_core::Effect::is_in_range | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\effect.rs:63 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 37 | is_in_range_ex | game_core::Effect::is_in_range_ex | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity, u64, u64, u64, u64, u64) -> bool | game-core\src\simulation\effect.rs:78 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 38 | is_recent_visible | game_core::Blackboard::is_recent_visible | pub | fn(&game_core::Blackboard, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\game\blackboard.rs:346 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 39 | is_visible | game_core::World::is_visible | pub | fn(&game_core::World, usize, usize) -> bool | game-core\src\simulation\game\data.rs:384 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 40 | is_visible | game_core::VisibleState::is_visible | pub | fn(&game_core::VisibleState) -> bool | game-core\src\simulation\game\data.rs:121 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 41 | is_visible | game_core::AbstractGame::is_visible | pub | fn(&Self/#0, usize, usize) -> bool | game-core\src\simulation.rs:125 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 42 | iter_projectile | game_core::AbstractGame::iter_projectile | pub | fn(&Self/#0) -> game_core::ProjectileIter | game-core\src\simulation.rs:182 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 43 | iter_projectile | <game_core::Game as game_core::AbstractGame>::iter_projectile | pub | fn(&game_core::Game) -> game_core::ProjectileIter | game-core\src\simulation\game.rs:3760 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 44 | iter_projectile | <game_core::SingleLaneGame as game_core::AbstractGame>::iter_projectile | pub | fn(&game_core::SingleLaneGame) -> game_core::ProjectileIter | game-core\src\simulation\game.rs:4019 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 45 | iter_towers | game_core::AbstractGameWithCache::<'a, 'b>::iter_towers | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5762 ~ game_core[6a30]::simulation::{impl#5}::iter_towers::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1908 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 46 | iter_towers_without_nexus | game_core::AbstractGameWithCache::<'a, 'b>::iter_towers_without_nexus | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5750 ~ game_core[6a30]::simulation::{impl#5}::iter_towers_without_nexus::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1830 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 47 | keep | game_view::pixel_fx::Dither::keep | in:game_view::pixel_fx | fn(game_view::pixel_fx::Dither, i32, i32) -> bool | game-view\src\view\pixel_fx.rs:49 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 48 | keep | game_view::view::effect::alchemist::Dither::keep | in:game_view::view::effect::alchemist | fn(game_view::view::effect::alchemist::Dither, i32, i32) -> bool | game-view\src\view\effect\alchemist.rs:372 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 49 | keep | game_view::view::projectile::sand_mage::Dither::keep | in:game_view::view::projectile::sand_mage | fn(game_view::view::projectile::sand_mage::Dither, i32, i32) -> bool | game-view\src\view\projectile\sand_mage.rs:240 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 50 | max_range_can_use | game_ai::plan_legacy::old::max_range_can_use | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-ai\src\plan_legacy\old\battle.rs:2431 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 51 | max_range_nearly_can_use | game_ai::plan_legacy::old::max_range_nearly_can_use | pub | fn(&game_core::Entity, &game_core::Entity, usize) -> u64 | game-ai\src\plan_legacy\old\battle.rs:2397 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 52 | nearest_enemy | game_core::Entity::nearest_enemy | pub | fn(&game_core::Entity) -> std::option::Option<usize> | game-core\src\simulation\entity.rs:1819 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 53 | new | game_ai::SmallActionTrace::new | pub | fn(&game_core::OperationData, usize, usize) -> game_ai::SmallActionTrace | game-ai\src\small_action\trace.rs:42 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 54 | new | game_ai::SmallActionAround::new | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::OperationData, &game_core::PlayerState, usize, usize) -> game_ai::SmallActionAround | game-ai\src\small_action\around.rs:23 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 55 | new | game_ai::SmallActionRunAway::new | pub | fn(&game_core::OperationData, &game_core::PlayerState, usize) -> game_ai::SmallActionRunAway | game-ai\src\small_action\move_actions.rs:25 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 56 | new | game_ai::SmallActionAroundRegion::new | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::OperationData, &game_core::PlayerState, usize, usize) -> game_ai::SmallActionAroundRegion | game-ai\src\small_action\around.rs:517 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 57 | new_with_skill | game_ai::SmallActionRunAway::new_with_skill | pub | fn(&game_core::OperationData, &game_core::PlayerState, usize, bool) -> game_ai::SmallActionRunAway | game-ai\src\small_action\move_actions.rs:29 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 58 | nontarget_windup_perceived | game_ai::nontarget_windup_perceived | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity) -> bool | game-ai\src\utils.rs:553 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 59 | position_score_at_position | game_ai::position_score_at_position | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, u64, u64, game_ai::PositionEvalPurpose) -> game_core::PositioningScore | game-ai\src\position_eval.rs:1192 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 60 | positioning_accuracy | game_core::AthleteParameter::positioning_accuracy | pub | fn(&game_core::AthleteParameter) -> usize | game-core\src\simulation\state\player.rs:285 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 61 | range_adjust | game_core::Effect::range_adjust | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:29 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 62 | range_misjudge_rng | game_ai::range_misjudge_rng | pub | fn(usize, &game_core::OperationData, &game_core::PlayerState, usize) -> std::option::Option<game_core::NoiseRng> | game-ai\src\utils.rs:502 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 63 | range_misjudge_roll | game_ai::range_misjudge_roll | pub | fn(&mut rand::rngs::std::StdRng, &mut std::option::Option<game_core::NoiseRng>, u64, u64) -> u64 | game-ai\src\utils.rs:516 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 64 | remain_action_time | game_core::Entity::remain_action_time | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1616 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 65 | score | game_ai::plan_legacy::sub_plan::SerpenPokeSubPlan::score | pub | fn(&game_ai::plan_legacy::sub_plan::SerpenPokeSubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 | game-ai\src\plan_legacy\sub_plan\serpen_poke.rs:508 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 66 | target_id | game_ai::SmallActionTrace::target_id | pub | fn(&game_ai::SmallActionTrace) -> usize | game-ai\src\small_action\trace.rs:427 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 67 | target_id | game_view::view::effect::alchemist::target_id | in:game_view::view::effect::alchemist | fn(game_core::InputTarget) -> std::option::Option<usize> | game-view\src\view\effect\alchemist.rs:126 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 68 | target_id | game_view::view::projectile::crossbowman::target_id | in:game_view::view::projectile::crossbowman | fn(game_core::InputTarget) -> std::option::Option<usize> | game-view\src\view\projectile\crossbowman.rs:1878 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 69 | team | game_view::ClientData::team | pub | fn(&game_view::ClientData, usize) -> std::option::Option<std::cell::Ref< game_core::Team>> | game-view\src\logic\client\data.rs:1651 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 70 | team | game_view::ClientDatabase::team | pub | fn(&game_view::ClientDatabase, usize) -> std::option::Option<&game_core::Team> | game-view\src\logic\client\data.rs:968 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 71 | team | game_core::PlayerAiContext::<'a, 'b, 'r>::team | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:583 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 72 | v27_objective_discipline_action | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v27_objective_discipline_action | pub | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, game_core::JungleType) -> std::option::Option<game_ai::SmallActionPlay> | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:505 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
</details>

⚠**미매칭 17개**: `block_target_tick`, `first`, `get_move_actions`, `map_or`, `next  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 9개는 **전부 다른 함수**라 싣지 않는다`, `on_periodic_trajectory`, `on_trajectory`, `poison`, `push  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 6개는 **전부 다른 함수**라 싣지 않는다`, `radius_adj`, `retain`, `roll`, `s0_0`, `s2_0`, `truncate`, `vec_in`, `visible_to`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m12.ll:35259) · **형제 8개** (SerpenPokeSubPlan)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::plan_legacy::sub_plan::SerpenPokeSubPlan as std::clone::Clone>::clone | pub | game-ai\src\plan_legacy\sub_plan\serpen_poke.rs:9 | True | fn(&game_ai::plan_legacy::sub_plan::SerpenPokeSubPlan) -> game_ai::plan_legacy::sub_plan::SerpenPokeSubPlan |
| 1 | <game_ai::plan_legacy::sub_plan::SerpenPokeSubPlan as std::fmt::Debug>::fmt | pub | game-ai\src\plan_legacy\sub_plan\serpen_poke.rs:9 | True | fn(&game_ai::plan_legacy::sub_plan::SerpenPokeSubPlan, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 2 | <game_ai::plan_legacy::sub_plan::SerpenPokeSubPlan as std::default::Default>::default | pub | game-ai\src\plan_legacy\sub_plan\serpen_poke.rs:9 | True | fn() -> game_ai::plan_legacy::sub_plan::SerpenPokeSubPlan |
| 3 | game_ai::plan_legacy::sub_plan::SerpenPokeSubPlan::action_candidates | pub | game-ai\src\plan_legacy\sub_plan\serpen_poke.rs:15 | False | fn(&mut game_ai::plan_legacy::sub_plan::SerpenPokeSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 4 | game_ai::plan_legacy::sub_plan::SerpenPokeSubPlan::get_move_action | in:game_ai::plan_legacy::sub_plan::serpen_poke | game-ai\src\plan_legacy\sub_plan\serpen_poke.rs:279 | False | fn(&mut game_ai::plan_legacy::sub_plan::SerpenPokeSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 5 | game_ai::plan_legacy::sub_plan::SerpenPokeSubPlan::action_candidates_old | pub | game-ai\src\plan_legacy\sub_plan\serpen_poke.rs:418 | False | fn(&mut game_ai::plan_legacy::sub_plan::SerpenPokeSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 6 | game_ai::plan_legacy::sub_plan::SerpenPokeSubPlan::calculate_score_parameter_value | pub | game-ai\src\plan_legacy\sub_plan\serpen_poke.rs:491 | True | fn(&game_ai::plan_legacy::sub_plan::SerpenPokeSubPlan, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::ScoreParameter) |
| 7 | game_ai::plan_legacy::sub_plan::SerpenPokeSubPlan::score | pub | game-ai\src\plan_legacy\sub_plan\serpen_poke.rs:508 | False | fn(&game_ai::plan_legacy::sub_plan::SerpenPokeSubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 |

**`open` 8건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | L383 사거리 합산에서 champ.level(+0x5c8)·champ.stat_buff_cached.range(+0x438) 와 serpen.attack_effect.range/growth_range 가 섞인다(IR m14.ll:14900~14962 그대로). 소스가 `Entity::effect_range(&self, effect)` 류 헬퍼(self=champ)인지 의도적 혼합인지는 소스 부재로 미확정 — 재현 시 IR 대로 계산하면 됨 | 4 |  |
| 1 | 미탐색 | L243 투사체 필터의 projectile.rs:134 인라인 헬퍼: move_type 태그 6(Target)·7(TargetSplash) 제외는 확정, '태그<=1(BouncingTarget) 이면서 tag 워드==1 이면 제외' 는 BouncingTarget 첫 페이로드 필드 의미를 못 잡음(tcxdict 로 BouncingTarget 9필드 레이아웃 미조회). 헬퍼 이름·의미 = 미탐색(_gcbc projectile.rs:134 define 확인 필요) | 3 |  |
| 2 | 표기 불가 | L232/L234 `!c.block_input() && mr == 0` 의 소스 순서 — IR 은 block_input 호출 후 `or (mr!=0)` 로 접혀 있어 한 줄 안 순서 표기 불가(column 0). 외연 동일 | 4 |  |
| 3 | 미탐색 | s2_0 L108~114 의 소스 문장 구조는 IR 동치식으로만 재구성(L108 if / L110 return / L114 return 의 정확한 술어 분할 미확정 · 동작은 확정) | 4 |  |
| 4 | 미탐색 | get_input 의 sret Option<Input> None 태그가 -1 인 이유(tcxdict 는 Input Direct 태그 0..5) — get_input 본문에서 `store i64 -1` 을 확인했으므로 IR 기준 -1=None 으로 적음. 니치 배치 근거는 미조사 | 3 |  |
| 5 | 미탐색 | v27_objective_discipline_action 이 어떤 variant 를 돌려주는지(반환 원소의 live 바이트) 는 그 함수 명세 소관 — 여기서는 184B 통째 복사만 확인 | 4 |  |
| 6 | 미탐색 | 위 constants 중 aux 표기가 없는 값 5·2·6·4·3 은 여러 의미(태그/인자)로 겹쳐 쓰인다 — meaning 에 전부 열거했으나 QC 는 값 단위로만 대조함 | 4 |  |
| 7 | 미탐색 | SerpenPokeSubPlan::score / action_candidates_old / get_move_actions 의 내부, position_score_at_position·check_kill_die_tick 의 TLS 는 각 명세 소관(열람 금지 규칙) — 본 명세는 호출 순서·인자만 | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

