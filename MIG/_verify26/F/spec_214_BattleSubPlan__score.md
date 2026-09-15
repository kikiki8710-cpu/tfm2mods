---

### `214` BattleSubPlan::score — 전투 서브플랜 액션 점수: interaction_score 기저에 액션 종류(도주/추적/평타/스킬/스킬2/궁)별 대상·태세·CC·처치 가능성 보정을 얹어 i64 로 반환(대상 소실·도주중 돌진스킬은 -99999)

| 항목 | 값 |
|---|---|
| id | `battle__Battle__score` |
| 심볼 | `_RNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy8sub_plan6battleNtB2_13BattleSubPlan5score` |
| 소스 | `game-ai\src\plan_legacy\sub_plan\battle.rs:616` |
| IR | `m02.ll` 37119~38402행 |
| 경로·가시성 | `game_ai::plan_legacy::sub_plan::BattleSubPlan::score` · **pub** |
| 계층 | 기타 |
| exe | `cc3080` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r17` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_ai::plan_legacy::sub_plan::BattleSubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64
```

**TLS 메모 절(`sig.tls` · 정정 경로 `/specs[214]/sig/tls/<키>`)**

직접 접점 없음 — 본문에 `@anon.* = constant ptr @<KEY…call_once>` 참조 0 · LocalKey::with 0 · llvm.threadlocal.address 0. (콜리 max_range_cached 가 TLS `MaxRangeCache`(RefCell) 소비자 — m00.ll:79611 LocalKey::with<max_range_cached::closure> · 계약 밖)

<details><summary>인자 8개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | self | &BattleSubPlan (48B) | IR `readonly dereferenceable(48)` · 읽는 필드 = +0x0/+0x8 support_target · +0x10 goal@tag · (+0x10 을 &goal 로 콜리에 전달) · +0x28 avoid_unnecessary_tower_trace | 4 |
| 1 | 2 | version | usize | 본 함수 분기 1곳 = L635 `version > 1`(m02.ll:37527) · 나머지는 콜리 통과 | 4 |
| 2 | 3 | parameter | &ScoreParameter (5384B) | IR `readonly` · 본문 직접 읽기 0 · interaction_score/calculate_action_score/battle_common 헬퍼로 통과만 | 4 |
| 3 | 4 | rnd | &mut StdRng (320B) | IR 속성 없음(`noalias noundef align 16 dereferenceable(320)` = &mut) · 본 함수 gen_range 사이트 0 · interaction_score(1회)·calculate_action_score(액션별 1회) 로 통과 | 4 |
| 4 | 5 | player | &PlayerState (2528B) | IR `readonly` · info.team(+0x930)·info.position(+0x9c0)·info.id(+0x928) 읽음 | 4 |
| 5 | 6 | data | &OperationData (24B) | IR `readonly` · +0x0 cache · +0x8 context | 4 |
| 6 | 7 | action | &SmallActionPlay (184B) | IR `readonly` · 태그 +0xb1 · 페이로드 target(+0x8 Attack/Skill/Skill2/Ult · +0x60 Trace) · +0x90 Trace.avoid_unnecessary_tower | 4 |
| 7 | 8 | debug | &mut DebugFrameData (224B) | IR 속성 없음(`noalias noundef align 8 dereferenceable(224)` = &mut) · 본 함수 직접 store 0 · interaction_score/calculate_action_score 로 통과만 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn score(&self, version, parameter, rnd, player, data, action, debug) -> i64   [battle.rs:616]
  champ = data.cache.player_champion[player.info.team][player.info.position].unwrap()      [L617 · 37147~37172]
  base = interaction_score(version, rnd, player, data, parameter, action, debug)             [L618 · 37190]
  match action {                                                                             [L620 · 37193~37218 switch(논리 idx)]
   RunAway(0)|Recall(1)|AroundRunAway(5) =>                                                 [L799]
     cp  = v17_runaway_counterattack_penalty(version, player, data, parameter, champ)        [L799 · 37228]
     dcp = v21_runaway_defensive_cc_hold_penalty(version, player, data, parameter, champ)    [L800 · 37230]
     if v15_can_keep_support_pressure(version, player, data, parameter, self.support_target) { base - 6 - (cp+dcp) }   [L801~802 · 37235, 37317~37319]
     else { base - (cp+dcp) }                                                                [L804 · 37312~37313]
   Trace{target: target_id, avoid_unnecessary_tower} =>                                     [L808]
     if self.avoid_unnecessary_tower_trace && avoid_unnecessary_tower {                      [L808 · 37244~37247, 37334~37340]
       if let Some(target) = get_entity_by_id(target_id) {                                   [L809 · 37343~37350]
         mr = max_range_cached(data, champ, target) + 25000                                  [L810 · 37355~37356]
         if target.distance_sq(champ) > mr*mr                                                 [L811 · 37358~37389]
            && !can_tower_focused(ctx, cache, player, champ.x, champ.y)                       [L812 · 37394 참→건너뜀]
            && can_tower_focused(ctx, cache, player, target.x, target.y)                      [L813 · 37398]
            && can_trace_without_tower(ctx, cache, player.info.id, target.x, target.y, mr)    [L814 · 37404]
         { return base - 30 }                                                                 [L815 · 37408]
       }
     }
     if self.support_target == Some(target_id) {                                             [L819 · 37325~37331]
       if let Some(target) = get_entity_by_id(target_id) {                                   [L821 · 37412~37419 · None → base]
         mr = max_range_cached(data, champ, target) + 25000                                  [L822]
         if target.distance_sq(champ) > mr*mr { base + 10 } else { base }                    [L823 · 37456~37459]
       } else { base }
     } else { base }
   Attack{target} =>                                                                          [L622]
     let Some(t) = get_entity_by_id(target) else { return -99999 }                           [L622 · 37255~37262]
     ae = champ.attack_effect.as_ref().unwrap()                                               [L623 · 37465~37469]
     score = calculate_action_score(version, rnd, player, data, parameter, &champ.attack, ae, champ.attack_speed_mult(), t, Normal(1), debug) + base   [L624 · 37475~37477]
     score += v17_runaway_counterattack_bonus(version, player, data, parameter, &self.goal, champ, t, ae)   [L625 · 37480~37481]
     match t.ty {                                                                              [L626 · 37484~37492]
       Nexus => score + 200                                                                   [L627]
       Epic|Serpen => dmg = ae.expected_damage_target(ctx, champ as &dyn AbstractEntity, t); if dmg < t.hp { max(score/3,1) } else { score + 100 }   [L629~633 · 37508~37524]
       Tower => if version > 1 && v3_tower_burst_feasible(player(→team), data(→cache,ctx), t) { score + 100 } else { score }   [L635~636 · 37527~37535 · fastcc ArgumentPromotion]
       Jungle => if t.ty.is_jungle(1-team /*적 진영*/) { dmg_est = champ.attack_effect.map(|a| a.expected_damage_target(ctx, champ, t)).unwrap_or(0); if dmg_est < t.hp { max(score,1) } else { score + 80 } }   [L638~642 · 37539~37604]
                 else if t.ty.is_jungle(team /*내 진영*/) { max(score,1) } else { score }        [L643~644 · 37548~37555]
       _ => score
     }
   Skill{target} =>                                                                           [L654]
     se = champ.skill_effect.as_ref().unwrap()                                                [L654 · 37271~37275]
     if se.ty.expected_rush_effect() && self.goal == RunAway { return -99999 }                [L654 · 37610~37630]
     let Some(t) = get_entity_by_id(target) else { return -99999 }                           [L658 · 37638~37645]
     score = calculate_action_score(version, rnd, player, data, parameter, &champ.skill, se, champ.cooldown_reduce(false), t, Normal, debug) + base   [L660 · 37652~37655]
     if self.goal == RunAway {                                                                [L662 · 37661]
       if let Some(cc_time) = effect_cc_time(version, se) {                                   [L663 · 37664~37667]
         dist = t.distance(champ); proximity_bonus = 80000.saturating_sub(dist) / 10000       [L664~665 · 37694~37700]
         score += (cc_time*3 + 3) * proximity_bonus                                           [L666 · 37702~37705]
       }
     }
     score += v21_runaway_defensive_cc_bonus(version, player, data, parameter, &self.goal, champ, t, se, effect_cc_time(version, se))   [L669 · 37672~37676]
     score += v17_runaway_counterattack_bonus(version, player, data, parameter, &self.goal, champ, t, se)   [L670 · 37678~37679]
     match t.ty { Nexus => +200 [L672] · Epic|Serpen => dmg=se.expected_damage_target(ctx,champ,t); dmg<hp ? max(score/3,1) : +100 [L674~678] · Jungle => 적 진영: attack_effect 기준 dmg_est<hp ? max(score,1) : +80 [L680~684] / 내 진영: max(score,1) [L685~686] / else score · _ => score }   [L671 · 37682~37802]
   Skill2{target} =>  Skill 과 동형: se2 = champ.skill2_effect()/*level>2 else None*/.unwrap() · rush&&RunAway → -99999 [L696] · t 소실 → -99999 [L700] · calculate_action_score(&champ.skill2, se2, cooldown_reduce(false)) [L702] · RunAway CC 근접 보너스 [L705~708] · v21 [L711] · v17 [L712] · ty 분기 Nexus+200 [L714] / Epic·Serpen [L716~720] / Jungle [L722~727]   [37804~38013]
   Ult{target} =>                                                                             [L738]
     ue_ref = champ.ult_effect()/*level>4 else None*/.unwrap(); if ue_ref.ty.expected_rush_effect() && goal==RunAway { return -99999 }   [L738 · 37300~37309, 38030~38039]
     let Some(t) = get_entity_by_id(target) else { return -99999 }                           [L742 · 38047~38054]
     ult_action = champ.ult()/*level>4 ? &ult : &empty*/                                       [L743 · 38061~38063]
     ue = champ.ult_effect().unwrap().clone()   // Effect 56B 스택 사본 · Arc strong+1        [L744 · 38066~38118]
     cc = effective_ult_cc_time(version, ult_action, &ue) -> Option<usize>                     [L745 · 38119]
     score = calculate_action_score(version, rnd, player, data, parameter, ult_action, &ue, champ.cooldown_reduce(true), t, Normal, debug) + base   [L746 · 38160~38168]
     if goal == RunAway && let Some(cc_time) = cc { proximity 보너스 (cc_time*3+3)*(80000-dist)/10000 }   [L748~752 · 38170~38204]
     score += v21_runaway_defensive_cc_bonus(version, player, data, parameter, &goal, champ, t, &ue, cc)   [L757 · 38183]
     score += v16_gambler_ult_cc_bonus(player, data, parameter, &goal, self.support_target, ult_action, &ue, champ, t)   [L759 · 38211]
     score += v16_knight_ult_zone_bonus(player, data, parameter, ult_action, champ, t)          [L760 · 38216]
     score += v17_runaway_counterattack_bonus(version, player, data, parameter, &goal, champ, t, &ue)   [L761 · 38221]
     match t.ty {                                                                              [L763 · 38234~38240]
       Champion => dmg = ue.expected_damage_target(ctx, champ, t); if dmg >= t.hp { score += 40 }   [L764~767 · 38245~38254]
                   buff = effect_buff_target(version, &ue, ctx, champ, champ)                  [L773 · 38259]
                   if !(cc.is_some() || buff.is_some()) { hp_ratio = t.hp*100/max(t.max_hp,1); if dmg*3 < t.hp && hp_ratio > 70 { score -= 30 } }   [L772~776 · 38263~38291]
       Nexus => score + 200                                                                   [L783 · 38295]
       Jungle => 적 진영: champ.attack_effect 기준 dmg_est<hp ? max(score,1) : +80 [L784~788 · 38329~38397] / 내 진영: max(score,1) [L789~790] / else score
       _ => score
     }
     drop(ue)  // Arc strong-1 · 0 이면 drop_slow                                              [L794 · 38311~38322]
   _ (Around·AroundHide·AroundRegion·Positioning·AroundPosition·AroundPositionBush·AroundBush·LaneMinionPosition·Stop) => base   [37227 case → 38400 phi %26]
  }                                                                                            [L837 ret · 38401]

주의: 사장 분기 0 · 접힌 분기 1(reach @215 = L635 `version>1` 이 version=2 이상에서 항상 참). gen_range 사이트 0(콜리 통과만). `%9`(288B)=Option<BuffState> sret · `%10`(56B)=ult_effect 사본. 부동/난수 없음.
```

**`mem` 메모리 접근 33건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | m02.ll:37147~37150 [L617] · `<2` 바운드체크 · 이후 1-team(적 팀) 계산(37539, 37735, 38329) | 4 | OK |
| 1 | PlayerState | 0x9c0 | info.position@tag (i32) | r | m02.ll:37158~37160 [L617 · player.rs:581 인라인] | 4 | OK |
| 2 | PlayerState | 0x928 | info.id | r | m02.ll:37402~37403 [L814] · can_trace_without_tower 3번 인자 | 4 | OK |
| 3 | OperationData | 0x0 | cache (&AbstractGameWithCache) | r | m02.ll:37161 [L617] | 4 | OK |
| 4 | OperationData | 0x8 | context (&GameContext 64B) | r | m02.ll:37392~37393 [L812] · 37506~37507 [L629] · 37531~37532 [L636] 등 — can_tower_focused/expected_damage_target/v3_tower_burst_feasible 인자 | 4 | OK |
| 5 | AbstractGameWithCache | 0x1e0 | player_champion[team][pos] (Option<&Entity> null=None) | r | m02.ll:37162~37172 [L617] · None → unwrap_failed(37224) · = champ | 4 | OK |
| 6 | AbstractGameWithCache | 0x0 | game (&dyn AbstractGame 팻포인터: +0 data · +8 vtable) | r | m02.ll:37255~37260 [L622] · 37343~37348 [L809] · 37412~37417 [L821] · 37638~37643 [L658] · 37836~37841 [L700] · 38047~38052 [L742] — vtable+0x1f0 = get_entity_by_id(target_id) -> Option<&Entity> | 4 | OK |
| 7 | SmallActionPlay | 0xb1 | 태그(니치 1B) | r | m02.ll:37193~37218 [L620 · small_action.rs:309 인라인] · 논리 idx = tag>2 ? tag-3 : 7 · assume(tag!=10) · switch | 4 | OK |
| 8 | SmallActionPlay | 0x8 | Attack/Skill/Skill2/Ult .target (usize) | r | m02.ll:37252~37253 [L94<322<620] · 37267~37268 · 37280~37281 · 37297~37298 | 4 | OK |
| 9 | SmallActionPlay | 0x60 | Trace.target (usize) | r | m02.ll:37241~37242 [L404<321<620] | 4 | OK |
| 10 | SmallActionPlay | 0x90 | Trace.avoid_unnecessary_tower (bool) | r | m02.ll:37334~37339 [L808] · 태그==14(Trace) 재확인과 AND | 4 | OK |
| 11 | BattleSubPlan | 0x28 | avoid_unnecessary_tower_trace (bool) | r | m02.ll:37244~37247 [L808] | 4 | OK |
| 12 | BattleSubPlan | 0x0 | support_target@tag (Option<usize>) | r | m02.ll:37232 [L801] · 37325~37327 [L819 Option::eq 인라인] · 38208 [L759] | 4 | OK |
| 13 | BattleSubPlan | 0x8 | support_target@Some.0 | r | m02.ll:37233~37234 [L801] · 37328~37329 [L819 `== target_id`] · 38209~38210 [L759] | 4 | OK |
| 14 | BattleSubPlan | 0x10 | goal@tag (BattleSubPlanGoal · 4=RunAway) | r | m02.ll:37622~37628 [L654] · 37820~37826 [L696] · 38170~38175 [L748] · 8038 근방 [L738] — `== RunAway` · 또 &self.goal(+0x10) 을 v17/v21/v16 헬퍼에 전달(37479, 37622, 38183 등) | 4 | OK |
| 15 | Entity (champ) | 0x4c0 | attack_effect@tag (니치 i32 · -1=None) | r | m02.ll:37466~37469 [L623 unwrap] · 37567~37568 [L640 map] · 37763~37765 [L682] · 38357~38359 [L786] | 4 | OK |
| 16 | Entity (champ) | 0x490 | attack_effect (Effect 56B 시작) | r | m02.ll:37465 [L623] · calculate_action_score/expected_damage_target 인자 | 4 | OK |
| 17 | Entity (champ) | 0x570 | attack (Box<dyn Action> 16B) | r | m02.ll:37474 [L624 · entity.rs:1494 Entity::attack 인라인] | 4 | OK |
| 18 | Entity (champ) | 0x4f8 | skill_effect@tag (니치 i32) | r | m02.ll:37272~37275 [L654] · None → unwrap_failed(37634) | 4 | OK |
| 19 | Entity (champ) | 0x4c8 | skill_effect (Effect 56B · +0 Arc data · +8 vtable) | r | m02.ll:37271 · 37610~37612 [L654 · Arc<dyn EffectType> 역참조 · vtable+0x10 align 으로 ArcInner 데이터 오프셋(37613~37618)] | 4 | OK |
| 20 | Entity (champ) | 0x580 | skill (Box<dyn Action>) | r | m02.ll:37652 [L660 · entity.rs:1665 인라인] | 4 | OK |
| 21 | Entity (champ) | 0x5c8 | level | r | m02.ll:37283~37287 [L696 · entity.rs:1693 skill2_effect: level>2 ? &skill2_effect : 정적 None(@anon.19)] · 37300~37304 [L738 · entity.rs:1701 ult_effect: level>4 ? &ult_effect : None] · 38061~38063 [L743 Entity::ult: level>4 ? &ult(0x5a0) : &empty(0x5b0)] | 4 | OK |
| 22 | Entity (champ) | 0x500 | skill2_effect (Effect 56B) · +0x530 tag | r | m02.ll:37286~37291 [L696] · 37851~37853 [L701] (1328 = 0x530 니치) | 4 | OK |
| 23 | Entity (champ) | 0x590 | skill2 (Box<dyn Action>) | r | m02.ll:37863 [L702 · entity.rs:1670 인라인] | 4 | OK |
| 24 | Entity (champ) | 0x538 | ult_effect (Effect 56B) · +0x568 tag | r | m02.ll:37303~37308 [L738] · 38062~38069 [L744] · Effect::clone → 스택 56B 사본(%10 · Arc strong+1 atomicrmw 38090 · 오버플로 trap 38123) | 4 | OK |
| 25 | Entity (champ) | 0x5a0 | ult (Box<dyn Action>) / 0x5b0 empty | r | m02.ll:38061~38063 [L743 Entity::ult 인라인 select 1440/1456] | 4 | OK |
| 26 | Entity (champ / target) | 0x660 | x | r | m02.ll:37358~37359(target) · 37366~37367(champ) [L811 Entity::distance_sq(entity.rs:2158) 인라인] · 37427~37436 [L823] · can_tower_focused 좌표 인자(37394, 37398) | 4 | OK |
| 27 | Entity (champ / target) | 0x668 | y | r | m02.ll:37362~37363 · 37370~37371 [L811] · 37431~37440 [L823] | 4 | OK |
| 28 | Entity (target t) | 0x68 | ty@tag (EntityType · 2 Tower/3 Nexus/4 Jungle/5 Epic/6 Serpen/13 Champion) | r | m02.ll:37484~37492 [L626 switch] · 37682~37689 [L671] · 38234~38240 [L763] · entity.rs:1400/1404 인라인 | 4 | OK |
| 29 | Entity (target t) | 0x98 | ty@Jungle.info.camp_type.0 (진영 usize) | r | m02.ll:37541~37549 [L638/643 EntityType::is_jungle(team) entity.rs:1378 인라인: camp==enemy_team / camp<2(=내 팀)] · 37737~37745 [L680/685] · 38331~38339 [L784/789] | 4 | OK |
| 30 | Entity (target t) | 0x670 | hp | r | m02.ll:37510~37512 [L630] · 37591~37593 [L642] · 37718~37720 [L675] · 37789~37791 [L684] · 38250~38252 [L767] · 38384~38386 [L788] · 38282 [L774 hp*100] | 4 | OK |
| 31 | Entity (target t) | 0x628 | stat_cached.hp (최대 HP) | r | m02.ll:38278~38281 [L774] · max(.,1) 로 0 나누기 방지 → hp_ratio | 4 | OK |
| 32 | Option<BuffState> (스택 288B sret) | 0x48 | duration@tag 니치(i32 -1=None) | r | m02.ll:38266~38268 [L773 effect_buff_target(...).is_some()] | 4 | 확인불가(tcx 사전에 타입 없음) |

**`consts` 상수 39건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 2 | 617 | 임계 | player_champion 1차 team<2 바운드체크 (m02.ll:37149) | 4 |
| 1 | 10 | 620 | 센티널 | SmallActionPlay 니치 태그 10(암묵 AroundPosition 자리) 은 절대 안 나옴 → llvm.assume (m02.ll:37195) | 4 |
| 2 | 7 | 620 | 태그 | 태그 ≤2 이면 논리 idx 7 = AroundPosition (m02.ll:37199 select) | 4 |
| 3 | -3 | 620 | 센티널 | 니치 태그 → 논리 idx 변환 `tag-3` (m02.ll:37197) | 4 |
| 4 | -6 | 802 | 계수 | RunAway/Recall/AroundRunAway 이고 v15_can_keep_support_pressure 참이면 base-6 (m02.ll:37317 `add %26, -6`) — 지원 사격 가능하면 도주 후보를 6 깎음 | 4 |
| 5 | 14 | 808 | 태그 | SmallActionPlay 메모리 태그 14 = Trace 재확인(matches! 인라인 · m02.ll:37337) — 이미 Trace 분기라 항상 참 | 4 |
| 6 | 25000 | 810 | 계수 | mr = max_range_cached(data, champ, target) + 25000 (m02.ll:37356 · 37425 [L822]) — 사거리 여유(≈0.78셀) | 4 |
| 7 | -30 | 815 | 계수 | 불필요 타워 추격 감점: Trace 대상이 mr 밖 · 내 위치는 타워 안 아님 · 대상은 타워 안 · can_trace_without_tower 참 → base-30 (m02.ll:37408) · Ult L776 에서도 -30 (38289) | 4 |
| 8 | 10 | 823 | 태그 | Trace 대상 == support_target 이고 dist² > mr² 이면 base+10 (m02.ll:37458~37459) | 4 |
| 9 | -99999 | 622 | 산출값 | 대상 엔티티 소실(get_entity_by_id None · L622/658/700/742) 또는 도주(goal==RunAway) 중 돌진(expected_rush_effect) 스킬/스킬2/궁(L654/696/738) → 사실상 배제 (m02.ll:38400 phi) | 4 |
| 10 | 1 | 624 | 태그 | calculate_action_score 10번 인자 MinionActionType 태그 1 = Normal (m02.ll:37476 `i8 1` · 37654 · 37865 · 38164) | 4 |
| 11 | 496 | 622 | 미상 | dyn AbstractGame vtable 슬롯 0x1f0 = get_entity_by_id (divtable 실측 · m02.ll:37258, 37346, 37415, 37641, 37839, 38050) | 3 |
| 12 | 96 | 654 | 미상 | dyn EffectType vtable 슬롯 0x60 = expected_rush_effect(&self)->bool (g02.ll:1311 정적 vtable 순서: drop·size·align·Debug::fmt·apply·expected_damage·…·expected_move_distance(0x58)·expected_rush_effect(0x60)) (m02.ll:37619, 37817, 38030 근방) | 4 |
| 13 | 4 | 654 | 태그 | BattleSubPlanGoal 태그 4 = RunAway — 돌진 스킬 배제(37628, 37826, 38038) · L748 proximity 보너스 조건(38175) | 4 |
| 14 | 3 | 626 | 태그 | EntityType 태그 3 = Nexus → +200 (37487 · 37685 · 38238) | 4 |
| 15 | 5 | 626 | 태그 | EntityType 태그 5 = Epic (37488 · 37686) | 4 |
| 16 | 6 | 626 | 태그 | EntityType 태그 6 = Serpen (37489 · 37687) | 4 |
| 17 | 2 | 626 | 태그 | EntityType 태그 2 = Tower (Attack 만 · 37490) | 4 |
| 18 | 4 | 626 | 태그 | EntityType 태그 4 = Jungle (37491 · 37688 · 38239) | 4 |
| 19 | 13 | 763 | 태그 | EntityType 태그 13 = Champion (Ult 만 · 38237) | 4 |
| 20 | 200 | 627 | 계수 | 대상이 Nexus 이면 +200 (평타 37500 · 스킬 37710 [L672] · 스킬2 37921 [L714] · 궁 38295 [L783]) | 4 |
| 21 | 100 | 631 | 계수 | Epic/Serpen 을 한 방(expected_damage_target ≥ hp)에 잡을 수 있으면 +100 (37523 · 37731 [L676] · 37942 [L718]) · Tower 이고 version>1 && v3_tower_burst_feasible 이면 +100 (37534 [L636]) | 4 |
| 22 | 3 | 633 | 태그 | Epic/Serpen 처치 불가(dmg < hp) → max(score/3, 1) — sdiv 3 (37516 · 37724 [L678] · 37935 [L720]) | 4 |
| 23 | 1 | 633 | 임계 | 점수 하한 llvm.smax(·,1): Epic/Serpen 처치불가 · 아군/적 정글 처치불가 · 내 정글 (37519, 37554, 37599, 37727, 37750, 37797, 38344, 38392) · L635 `version > 1`(37527) · L774 umax(max_hp,1)(38281) | 4 |
| 24 | 1 | 638 | 임계 | enemy_team = 1 - player.info.team (37539 · 37735 · 38329 `sub nuw nsw 1, %12`) | 4 |
| 25 | 2 | 643 | 임계 | is_jungle(my_team) 인라인 잔여 `camp_type.0 < 2`(적 진영이 아님을 이미 알므로 내 진영 여부) (37548 · 37744 · 38338) | 4 |
| 26 | 80 | 642 | 계수 | 적 진영 정글을 한 방(attack_effect 기준 dmg_est ≥ hp)에 잡으면 +80 (37603 · 37801 [L684] · 38012 [L726] · 38396 [L788]) | 4 |
| 27 | 0 | 640 | 임계 | attack_effect None 이면 dmg_est=0 (37587 phi · 37785 · 38382 select) → 항상 처치불가 경로 | 4 |
| 28 | -1 | 623 | 센티널 | Option<Effect> 니치 None 판별(casting 태그 i32 -1) (37468, 37274, 37291, 37308, 37568, 37853, 38068 …) · Option<BuffState> 니치 (38268) | 4 |
| 29 | 2 | 696 | 임계 | Entity::skill2_effect 인라인 `level > 2` (37285 · entity.rs:1693) | 4 |
| 30 | 4 | 738 | 임계 | Entity::ult_effect/ult 인라인 `level > 4` (37302 · 38061 · entity.rs:1701) | 4 |
| 31 | 80000 | 665 | 계수 | proximity_bonus = 80000.saturating_sub(dist) / 10000 (0..8) — 도주 중 CC 스킬 근접 보너스 기준 거리 2.5셀 (37697 · 37909 [L707] · 38194 [L751]) | 4 |
| 32 | 10000 | 665 | 계수 | proximity 단위(≈0.31셀) 제수 (37699 udiv i32 · 37910 · 38196) | 4 |
| 33 | 3 | 666 | 태그 | 도주 중 CC 스킬: score += (cc_time*3 + 3) * proximity_bonus (37702~37704 · 37913~37915 [L708] · 38199~38201 [L752]) | 4 |
| 34 | 40 | 767 | 계수 | Ult 대상 Champion 을 한 방에 잡으면(dmg ≥ hp) +40 (38253~38254) | 4 |
| 35 | 100 | 774 | 계수 | hp_ratio = t.hp*100 / max(t.stat_cached.hp,1) (38282~38283) | 4 |
| 36 | 70 | 776 | 임계 | Ult vs Champion: CC 없음 && 자기버프 없음 && dmg*3 < hp && hp_ratio > 70 → -30 (38285~38290) — 체력 넉넉한 챔피언에 무CC·무버프 딜궁 낭비 억제 | 4 |
| 37 | 3 | 776 | 태그 | `dmg*3 < t.hp` 3방 이상 필요 판정 (38285) | 4 |
| 38 | 72 | 773 | 센티널 | Option<BuffState> 니치 오프셋 0x48(duration@tag) — is_some 판정 (38266) | 4 |

**`knobs` 조정점 15건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 지원사격 가능 시 도주 감점 | battle.rs:802 | 6 | 올리면 서포트 압박(v15)이 가능한 상황에서 도주/귀환 후보가 더 억제된다 | 4 | 기존 |
| 1 | 불필요 타워 추격 감점 | battle.rs:815 | 30 | 올리면 사거리 밖·타워 안 대상을 향한 Trace 가 더 강하게 배제된다(avoid_unnecessary_tower_trace 플래그일 때만) | 4 | 기존 |
| 2 | 추격 사거리 여유 | battle.rs:810/822 | 25000 | 키우면 '사거리 안'으로 간주되는 범위가 넓어져 타워 추격 감점·support 추격 가산이 덜 발동 | 4 | 기존 |
| 3 | support_target 원거리 추격 가산 | battle.rs:823 | 10 | 올리면 지원 대상이 멀 때 Trace 가 더 잘 뽑힌다 | 4 | 기존 |
| 4 | Nexus 타격 가산 | battle.rs:627/672/714/783 | 200 | 올리면 넥서스 공격/스킬/궁 후보가 압도적으로 우선 | 4 | 기존 |
| 5 | Epic/Serpen 한방 처치 가산 | battle.rs:631/676/718 | 100 | 올리면 막타 가능 시 오브젝트 타격이 더 우선 | 4 | 기존 |
| 6 | Epic/Serpen 처치불가 감쇠 제수 | battle.rs:633/678/720 | 3 | 키우면 막타 불가한 오브젝트 타격 후보가 더 억제된다(하한 1) | 4 | 기존 |
| 7 | 타워 버스트 가산 | battle.rs:636 | 100 | 올리면 v3_tower_burst_feasible 상황에서 평타 타워 타격이 더 우선(version>1) | 4 | 기존 |
| 8 | 적 정글 한방 처치 가산 | battle.rs:642/684/726/788 | 80 | 올리면 적 진영 정글 스틸 막타가 더 우선 | 4 | 기존 |
| 9 | 도주 중 CC 스킬 근접 보너스 기준 거리 | battle.rs:665/707/751 | 80000 | 키우면 더 먼 적에게도 도주 중 CC 스킬 보너스가 붙는다(단위 10000 당 1) | 4 | 기존 |
| 10 | 도주 중 CC 스킬 보너스 계수 | battle.rs:666/708/752 | 3 | 올리면 (cc_time*3+3)*proximity 가 커져 도주 중 CC 스킬 사용이 더 적극적 | 4 | 기존 |
| 11 | 궁 한방 처치 가산 | battle.rs:767 | 40 | 올리면 킬각 궁이 더 우선 | 4 | 기존 |
| 12 | 무CC·무버프 딜궁 낭비 감점 HP% 임계 | battle.rs:776 | 70 | 내리면 더 낮은 HP 의 챔피언에게도 -30 이 적용돼 딜궁을 더 아낀다 · 3방 이상 필요(dmg*3<hp) 조건과 AND | 4 | 기존 |
| 13 | 무CC·무버프 딜궁 낭비 감점 | battle.rs:776 | 30 | 올리면 CC 도 자기버프도 없는 궁을 체력 넉넉한 챔피언에게 쓰는 걸 더 억제 | 4 | 기존 |
| 14 | 배제 점수 | battle.rs:622/654/658/696/700/738/742 | -99999 | 대상 소실·도주중 돌진스킬 — 사실상 선택 불가(다른 후보가 모두 이보다 낮을 일이 없음) | 4 | 기존 |

<details><summary>`callees` 피호출자 35건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | attack_speed_mult | game_core::Entity::attack_speed_mult | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:2433 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | calculate_action_score | game_ai::calculate_action_score | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &std::boxed::Box<dyn [Binder { value: Trait(game_core::Action), bound_vars: [] }] + 'static, std::alloc::Global>, &game_core::Effect, usize, &game_core::Entity, game_ai::MinionActionType, &mut game_core::DebugFrameData) -> i64 | game-ai\src\action_score.rs:8 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 2 | can_tower_focused | game_ai::can_tower_focused | pub | fn(&game_core::GameContext, &game_core::AbstractGameWithCache, &game_core::PlayerState, u64, u64) -> bool | game-ai\src\tower_discipline.rs:9 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 3 | can_trace_without_tower | game_ai::can_trace_without_tower | pub | fn(&game_core::GameContext, &game_core::AbstractGameWithCache, usize, u64, u64, u64) -> bool | game-ai\src\tower_discipline.rs:676 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | cooldown_reduce | game_core::Entity::cooldown_reduce | pub | fn(&game_core::Entity, bool) -> usize | game-core\src\simulation\entity.rs:2437 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 5 | distance | game_core::Entity::distance | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2161 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 6 | distance_sq | game_core::utils::distance_sq | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:6 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 7 | distance_sq | game_core::Entity::distance_sq | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2157 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 8 | drop | <game_core::prof::ProfTimer as std::ops::Drop>::drop | pub | fn(&mut game_core::prof::ProfTimer) | game-core\src\simulation\prof.rs:184 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 9 | effect_buff_target | game_ai::effect_buff_target | pub | fn(usize, &game_core::Effect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + , &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + ) -> std::option::Option<game_core::BuffState> | game-ai\src\fight_check.rs:390 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 10 | effect_cc_time | game_ai::effect_cc_time | pub | fn(usize, &game_core::Effect) -> std::option::Option<usize> | game-ai\src\fight_check.rs:379 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 11 | effective_ult_cc_time | game_ai::plan_legacy::sub_plan::battle_common::effective_ult_cc_time | in:game_ai | fn(usize, &std::boxed::Box<dyn [Binder { value: Trait(game_core::Action), bound_vars: [] }] + 'static, std::alloc::Global>, &game_core::Effect) -> std::option::Option<usize> | game-ai\src\plan_legacy\sub_plan\battle_common.rs:99 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 12 | expected_damage_target | game_core::Effect::expected_damage_target | pub | fn(&game_core::Effect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + , &game_core::Entity) -> usize | game-core\src\simulation\effect.rs:91 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 13 | expected_rush_effect | game_core::EffectType::expected_rush_effect | pub | fn(&Self/#0) -> bool | game-core\src\simulation\effect\type.rs:291 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 9개 중 상위 3개 |
| 14 | expected_rush_effect | <game_core::RushEffect as game_core::EffectType>::expected_rush_effect | pub | fn(&game_core::RushEffect) -> bool | game-core\src\simulation\effect\type\rush.rs:53 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 9개 중 상위 3개 |
| 15 | expected_rush_effect | <game_core::CombineEffect as game_core::EffectType>::expected_rush_effect | pub | fn(&game_core::CombineEffect) -> bool | game-core\src\simulation\effect\type\combine.rs:42 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 9개 중 상위 3개 |
| 16 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 17 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 18 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 19 | interaction_score | game_ai::interaction_score | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 | game-ai\src\action_score.rs:616 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 20 | is_jungle | game_core::EntityType::is_jungle | pub | fn(&game_core::EntityType, usize) -> bool | game-core\src\simulation\entity.rs:1377 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 21 | max_range_cached | game_ai::plan_legacy::old::max_range_cached | pub | fn(&game_core::OperationData, &game_core::Entity, &game_core::Entity) -> u64 | game-ai\src\plan_legacy\old\battle.rs:2332 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 22 | score | game_ai::plan_legacy::sub_plan::SubPlan::score | pub | fn(&mut game_ai::plan_legacy::sub_plan::SubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 | game-ai\src\plan_legacy\sub_plan\mod.rs:164 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 26개 중 상위 3개 |
| 23 | score | game_ai::plan_legacy::sub_plan::HideSubPlan::score | pub | fn(&game_ai::plan_legacy::sub_plan::HideSubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 | game-ai\src\plan_legacy\sub_plan\hide.rs:238 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 26개 중 상위 3개 |
| 24 | score | game_ai::plan_legacy::sub_plan::StealSubPlan::score | pub | fn(&game_ai::plan_legacy::sub_plan::StealSubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 | game-ai\src\plan_legacy\sub_plan\steal.rs:136 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 26개 중 상위 3개 |
| 25 | skill2_effect | game_core::Entity::skill2_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1692 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 26 | ult_effect | game_core::Entity::ult_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1700 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 27 | v15_can_keep_support_pressure | game_ai::plan_legacy::sub_plan::battle_common::v15_can_keep_support_pressure | in:game_ai | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, std::option::Option<usize>) -> bool | game-ai\src\plan_legacy\sub_plan\battle_common.rs:7 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 28 | v16_gambler_ult_cc_bonus | game_ai::plan_legacy::sub_plan::battle_common::v16_gambler_ult_cc_bonus | in:game_ai | fn(&game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_ai::plan_legacy::old::BattleSubPlanGoal, std::option::Option<usize>, &std::boxed::Box<dyn [Binder { value: Trait(game_core::Action), bound_vars: [] }] + 'static, std::alloc::Global>, &game_core::Effect, &game_core::Entity, &game_core::Entity) -> i64 | game-ai\src\plan_legacy\sub_plan\battle_common.rs:109 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 29 | v16_knight_ult_zone_bonus | game_ai::plan_legacy::sub_plan::battle_common::v16_knight_ult_zone_bonus | in:game_ai | fn(&game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &std::boxed::Box<dyn [Binder { value: Trait(game_core::Action), bound_vars: [] }] + 'static, std::alloc::Global>, &game_core::Entity, &game_core::Entity) -> i64 | game-ai\src\plan_legacy\sub_plan\battle_common.rs:177 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 30 | v17_runaway_counterattack_bonus | game_ai::plan_legacy::sub_plan::battle_common::v17_runaway_counterattack_bonus | in:game_ai | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_ai::plan_legacy::old::BattleSubPlanGoal, &game_core::Entity, &game_core::Entity, &game_core::Effect) -> i64 | game-ai\src\plan_legacy\sub_plan\battle_common.rs:270 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 31 | v17_runaway_counterattack_penalty | game_ai::plan_legacy::sub_plan::battle_common::v17_runaway_counterattack_penalty | in:game_ai | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_core::Entity) -> i64 | game-ai\src\plan_legacy\sub_plan\battle_common.rs:353 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 32 | v21_runaway_defensive_cc_bonus | game_ai::plan_legacy::sub_plan::battle_common::v21_runaway_defensive_cc_bonus | in:game_ai | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_ai::plan_legacy::old::BattleSubPlanGoal, &game_core::Entity, &game_core::Entity, &game_core::Effect, std::option::Option<usize>) -> i64 | game-ai\src\plan_legacy\sub_plan\battle_common.rs:476 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 33 | v21_runaway_defensive_cc_hold_penalty | game_ai::plan_legacy::sub_plan::battle_common::v21_runaway_defensive_cc_hold_penalty | in:game_ai | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_core::Entity) -> i64 | game-ai\src\plan_legacy\sub_plan\battle_common.rs:494 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 34 | v3_tower_burst_feasible | game_ai::plan_legacy::sub_plan::battle::v3_tower_burst_feasible | in:game_ai::plan_legacy::sub_plan::battle | fn(&game_core::PlayerState, &game_core::OperationData, &game_core::Entity) -> bool | game-ai\src\plan_legacy\sub_plan\battle.rs:32 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
</details>

⚠**미매칭 3개**: `data`, `drop_slow`, `player`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m12.ll:37260) · **형제 8개** (BattleSubPlan)

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

**`open` 10건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | L838~ 이후 없음 · L619·621·645~653·687~695·728~737·791~798·805~807·816~818·824~836 은 IR 에 !DILocation 없음(빈 줄/주석/닫는 괄호 추정 · rmeta_srcmap 미조회) | 3 |  |
| 1 | 미탐색 | interaction_score(정본 r15)·calculate_action_score(정본 r15) 내부 — 계약만: (version,rnd,player,data,parameter,action,debug)->i64 / (version,rnd,player,data,parameter,&Box<dyn Action>,&Effect,mult:usize,&Entity,MinionActionType,debug)->i64(range ≤340) | 3 |  |
| 2 | 미탐색 | battle_common 헬퍼 6종(v17_runaway_counterattack_penalty/bonus · v21_runaway_defensive_cc_hold_penalty/cc_bonus · v16_gambler_ult_cc_bonus · v16_knight_ult_zone_bonus · effective_ult_cc_time · v15_can_keep_support_pressure) 내부 — tcx 시그니처만(위 logic) · 반환 range: penalty 0..22 · hold_penalty 0..38 · counterattack_bonus 0..95 · defensive_cc_bonus ≤160 · gambler ≤70 · knight 0..75 | 3 |  |
| 3 | 미탐색 | tower_discipline::can_tower_focused(ctx,cache,player,x,y)->bool · can_trace_without_tower(ctx,cache,id,x,y,mr)->bool 내부(정본 r13 잎) — 극성은 분기 방향으로만: L812 는 참이면 건너뜀(=`!`), L813/L814 는 참이어야 감점 | 3 |  |
| 4 | 표기 불가 | L811/L823 `distance_sq > mr*mr` 이 소스에서 `distance() > mr` 이었을 가능성 — 표기 불가(제곱 비교로 접힘 · entity.rs:2158 distance_sq 인라인) | 4 |  |
| 5 | 표기 불가 | L654/696/738 소스가 `expected_rush_effect() && matches!(goal, RunAway)` 인지 순서 반대인지 — 한 줄 안 순서라 표기 불가(IR 은 rush 를 먼저 평가) | 4 |  |
| 6 | 표기 불가 | L748 `goal==RunAway && cc.is_some()` 의 한 줄 안 순서 — 표기 불가 | 4 |  |
| 7 | 미탐색 | v3_tower_burst_feasible 은 internal fastcc ArgumentPromotion(player→team i64 · data→cache,ctx ptr) — exe 인자 배치 검증 시 argscan 필요(계약 밖) | 4 |  |
| 8 | 미탐색 | expected_damage_target 의 caster 인자는 `&dyn AbstractEntity`(data=champ · vtable=@anon.54 = Entity as AbstractEntity) — 어느 메서드가 쓰이는지는 game_core 경계 | 4 |  |
| 9 | 표기 불가 | Ult Champion 분기(L764~776)에서 dmg≥hp 로 +40 을 받은 뒤에도 dmg*3<hp 조건이 거짓이라 -30 은 상호배타 — 소스가 else-if 인지 별개 if 인지 표기 불가(외연 동일) | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

