---

### `251` battle_action — 자기 챔피언의 기본공격/스킬/스킬2 후보(SmallActionPlay Attack·Skill·Skill2)를 근처 적(가시·타겟가능·사거리+이동여유 이내)과 근처 아군(스킬 CastingTarget 통과·사거리+walk 여유 이내), 셀프버프(should_add_self_etc_buff_action)로 모아 bumpalo Vec 으로 돌려준다. 배치 E = 머리~Attack 루프~Skill 적/아군 루프(0~710); 스킬 셀프버프·Skill2 전 경로·반환은 배치 F(713~786).

| 항목 | 값 |
|---|---|
| id | `fight_check__battle_action` |
| 심볼 | `_RNvNtCshdEBA0ozCnw_7game_ai11fight_check13battle_action` |
| 소스 | `game-ai\src\fight_check.rs:620` |
| IR | `m15.ll` 23700~25612행 |
| 경로·가시성 | `game_ai::battle_action` · **pub** |
| 계층 | 점수화·술어 |
| exe | `eb6100` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r18` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, usize) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay>
```

**TLS 메모 절(`sig.tls` · 정정 경로 `/specs[251]/sig/tls/<키>`)**

없음 — 본문 23700~25612 에 LocalKey/call_once/llvm.threadlocal.address/__getit 참조 0건(grep) · reach 콜리 요약에도 TLS 접점 콜리 없음

<details><summary>인자 6개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | bumpalo::collections::vec::Vec<SmallActionPlay> (32B) | 반환 슬롯 · IR 속성 sret([32 x i8]) writeonly captures(none) dereferenceable(32) · 배치 E 범위에서는 쓰지 않는다(지역 candidates %24 를 배치 F L786 에서 memcpy) · (배치 F) L785 에서 지역 candidates(%24) 32B 를 memcpy 로 채운다 | 4 |
| 1 | 1 | _version | usize (i64) | AI 버전 게이트 — 본문 23701~25612 에서 사용 0회(grep) · 이름도 `_version` · (배치 F) L762 ally_buff_walk_allowance(version,…) 인라인에도 분기 없음 | 4 |
| 2 | 2 | _rnd | &mut StdRng | IR 속성 readnone align 16 captures(none) · 본문 사용 0회 · gen_range 호출 사이트 0개(순서 없음) · (배치 F) gen_range 호출 사이트 0 (본문 전체) | 4 |
| 3 | 3 | player | &PlayerState (2528B) | IR 속성 readonly dereferenceable(2528) · info.team(+0x930)·info.position@tag(+0x9c0) 읽기 · 클로저 s_0/s0_0 에 캡처 · is_dash_worth 에 전달 · (배치 F) 배치 F 범위에선 콜리 인자 전달만(should_add_self_etc_buff_action · is_dash_worth) | 4 |
| 4 | 4 | data | &OperationData (24B: +0 cache &AbstractGameWithCache · +8 context &GameContext · +0x10 blackboard &[Blackboard;2]) | IR 속성 readonly dereferenceable(24) · cache.player_champion · context.pool(bump) · blackboard[적팀].small_actions · SmallActionAttack/Skill::new(data,..) · is_dash_worth(data,..) · can_target(self=data,..) · (배치 F) +0 cache(&AbstractGameWithCache) · +8 context(&GameContext) 읽음 | 4 |
| 5 | 5 | _end_delay | usize (i64) | 본문 사용 0회(grep) · 이름도 `_end_delay` | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
// fight_check.rs:0~710 (배치 E)
// 호출 계약(콜리 요약): can_attack/can_skill/block_move(&Entity)->bool · Effect::range_adjust(&Effect, caster:&Entity, target:&Entity)->i64 · Effect::expected_damage_target(&Effect, ctx:&GameContext, caster:&dyn AbstractEntity(ptr,vtable @anon.56 88B), target:&Entity)->i64 · CastingTarget::check(&CastingTarget(4B), caster, target)->bool · SmallActionAttack/Skill::new(sret 24B initializes((0,17)), data:&OperationData, target_id:usize) · utils::is_dash_worth(data, player, champ, target, skill_damage:i64)->bool(m04.ll:48412, 내부 미독 — 시그니처만) · OperationData::can_target(&self, game:&dyn AbstractGame(ptr,vtable), player:&PlayerState, target:&Entity)->bool(g15.ll:66255) · EffectType vtable +0x120 can_move(&self)->bool · +0xa0 expected_buff_deep(sret Option<BuffState> 288B, &self, ctx:&GameContext, caster:&dyn AbstractEntity)

[L621] _t = prof::start(71)   // prof::ENABLED(atomic u8, monotonic)==0 → None(nanos=-1) · 계측, 판정 무관
[L622] team = player.info.team(+0x930);  if team>=2 → panic_bounds_check(team,2)
       pos  = player.info.position@tag(+0x9c0, i32 zext)
       champ = data.cache.player_champion[team][pos](cache+0x1e0 + team*40 + pos*8).unwrap()   // null → unwrap_failed(option.rs:1013) 패닉
[L624~625] bump = data.context(+0x8).pool(+0x0)
       near_allies: bumpalo Vec<&Entity> = cache.iter_champions(team)     // = player_champion[team][0..5].iter().filter_map(|o| *o)  (aux m01:37609)
           .filter(closure0 |a| a.id(+0x5c0) != champ.id && a.distance_sq(champ) < 22500000000 /*150000²*/)   // aux m15:58121 · 자기 자신 제외
           .collect_in(bump)
[L627~631] enemy_row = player_champion[1 - team]
       near_enemies_with_action: bumpalo Vec<(SmallAction<&Entity>, &Entity)> = enemy_row.iter().enumerate()
           .map(s_0 |(i, c)| (data.blackboard(+0x10)[1 - player.info.team].small_actions[i] /*Blackboard+0x78 + i*24, 24B 복사*/, c))   // aux m12:21412 · 1-team<2, i<5 bounds check
           .filter(s0_0 |(act, c)| act.is_some()(tag != -1) && c.is_some()(non-null) && data.can_target(cache.game, player, c))   // aux m15:58177
           .map(s1_0 |(act, c)| (act.unwrap(), c.unwrap()))   // aux m01:46785 · unwrap 실패 = 패닉(fight_check.rs:631)
           .collect_in(bump)   // 원소 32B: +0 SmallAction(24B) · +0x18 &Entity
[L634] candidates = Vec::<SmallActionPlay>::new_in(bump)   // ptr=8(dangling) · a=bump · cap=0 · len=0
[L636] attack: Option<&Effect> = champ.attack_effect(+0x490).as_ref()     // None ⇔ +0x4c0 == -1
[L637] skill  = champ.skill_effect(+0x4c8).as_ref()                      // None ⇔ +0x4f8 == -1
[L638] skill2 = champ.skill2_effect().as_ref()   // entity.rs:1693 인라인: if level(+0x5c8) > 2 { &self.skill2_effect(+0x500) } else { &NONE(@anon.58, +0x30=-1) } → None ⇔ (그 주소+0x30) == -1. 사용은 배치 F
[L640] move_speed = champ.stat_cached.move_speed(+0x640)

[L642] if attack.is_some() && champ.can_attack() {          // attack None 또는 can_attack false → [L667] 로
[L643]   for (act, e) in near_enemies_with_action.iter() {
[L644]     if !e.is_visible_from(champ) { continue }
             // entity.rs:1482~1483 인라인: match champ.team(+0x0 tag) { Player(t=+0x8) => (t<2 else panic) e.visible_state[t](+0x38+t*24).tag == 0(Visible), Neutral(1) => true }
[L648]     range = attack.range(champ) + attack.range_adjust(champ, e) + champ.radius() + e.radius()
             // Effect::range(effect.rs:26) = range(+0x4a0) + growth_range(+0x4a8)*(champ.level-1) + champ.stat_buff_cached.range(+0x438)
             // Entity::radius(entity.rs:1511~1515) = if stat_buff_cached.radius_mult(+0x470)==0 { radius(+0x680) } else { radius*(mult+100)/100 (udiv) }
[L649]     dist_sq = e.distance_sq(champ) = |e.x-champ.x|² + |e.y-champ.y|²   (+0x660/+0x668 abs_diff)
[L652~653] ms = if *act == SmallAction::RunAway(tag 0) && !e.block_move() { move_speed.saturating_sub(e.stat_cached.move_speed(+0x640)) } else { move_speed }   // 도망치는 적은 상대속도로
[L658]     max_dist = range + ms*30
[L659]     if dist_sq > max_dist*max_dist { continue }   // icmp ugt → 초과면 제외(경계 포함 통과)
[L663]     candidates.push(SmallActionPlay::Attack(SmallActionAttack::new(data, e.id(+0x5c0))))   // 태그 15 @+0xb1 · len==cap 이면 reserve_internal_or_panic
         }
       }
[L667] if skill.is_some() && champ.can_skill() {           // 아니면 → 배치 F(줄 726: skill2 블록)
[L668]   for (act, e) in near_enemies_with_action.iter() {
[L669]     if !e.is_visible_from(champ) { continue }   // L644 와 동일 인라인
[L673]     if !skill.target(+0x4f0, CastingTarget).check(champ, e) { continue }
[L677~678] ms = if *act == RunAway && !e.block_move() { move_speed.saturating_sub(e.move_speed) } else { move_speed }
[L683]     range = skill.range(champ)(+0x4d8 + +0x4e0*(level-1) + +0x438) + skill.range_adjust(champ, e) + champ.radius() + e.radius()
[L684]     dist_sq = e.distance_sq(champ)
[L687]     max_dist = ms*30 + range
[L688]     if dist_sq > max_dist*max_dist { continue }
[L693]     if skill.ty.can_move()(vtable+0x120, Arc<dyn EffectType> deref) && e.is_champion()(ty@tag +0x68 == 13) {
[L694]       skill_damage = skill.expected_damage_target(ctx=data.context, champ as &dyn AbstractEntity(@anon.56), e)
[L695]       if !is_dash_worth(data, player, champ, e, skill_damage) { continue }
           }   // can_move 거짓이거나 적이 챔피언이 아니면 검사 없이 push
[L700]     candidates.push(SmallActionPlay::Skill(SmallActionSkill::new(data, e.id)))   // 태그 16
         }
[L703]   walk = ally_buff_walk_allowance(champ, skill, ctx)   // fight_check.rs:1183 인라인: if skill.ty.expected_buff_deep(ctx, champ as dyn).is_some()(sret+0x48 != -1) { 90 } else { 30 }
         walk_ms = walk * move_speed   // 루프 밖 선계산(%405)
[L704]   for a in near_allies.iter() {                    // 아군 루프: 가시성 검사 없음 · RunAway 보정 없음
[L705]     if !skill.target.check(champ, a) { continue }
[L709~713] max_dist = skill.range(champ) + walk_ms + skill.range_adjust(champ, a) + champ.radius() + a.radius()   // 마지막 가산(a.radius)은 소스 줄 713
[L710]     dist_sq = a.distance_sq(champ)
[L714]     if dist_sq > max_dist*max_dist { continue }
           → 배치 F(줄 718: candidates.push(Skill(a.id)))
         }
         → 배치 F(줄 721~722: should_add_self_etc_buff_action → 셀프 Skill push)
       }
       → 배치 F(줄 726~786: skill2 적/아군/셀프 경로 · 반환 sret=candidates · near_allies/near_enemies drop · ProfTimer drop)

// 클로저 환경 레이아웃(IR 로 확정, aux from_iter_in 이 56B 를 통째 복사): +0 Iter.ptr · +8 Iter.end · +0x10 enumerate.count(0) · +0x18 s_0{blackboard} · +0x20 s_0{player}(=%3) · +0x28 s0_0{data} · +0x30 s0_0{player} · s1_0 = ZST. s0_0 심이 env.0 을 can_target 의 self(dereferenceable(24)=OperationData)로 넘기므로 +0x28=data 확정.
// rnd: gen_range 호출 0회(_rnd readnone).

// fight_check.rs:713~786 (배치 F)
// [문맥 · 배치 E 승계, IR 로 재확인] champ=%55=data.cache.player_champion[team][pos] · candidates=%24 bumpalo Vec<SmallActionPlay>{+0 ptr,+8 bump,+16 cap,+24 len} · near_enemies_with_action=%26 Vec<(SmallAction,&Entity)>(원소 32B: +0 tag, +24 &Entity) · near_allies=%28 Vec<&Entity> · skill=%91=champ.skill_effect(Some 판정 %89: +0x4f8 casting tag != -1) · skill2=%100 (L638: level>2 ? &champ.skill2_effect : 상수 None @anon.58 → %99: +0x530 tag != -1) · move_speed=%102=champ.stat_cached.move_speed · ctx=%62=data.context · champ as &dyn AbstractEntity = (%55, @anon.56 vtable)
// 공통 인라인 헬퍼: Effect::range(champ)[effect.rs:26] = range(+0x10) + growth_range(+0x18)*(level-1) + champ.stat_buff_cached.range(+0x438) · Entity::radius()[entity.rs:1511~1515] = radius_mult(+0x470)==0 ? radius(+0x680) : radius*(mult+100)/100 · Entity::distance_sq(other)[entity.rs:2158→utils.rs:7~9] = |dx|²+|dy|² (abs_diff)

// ===== (A) 스킬1 분기의 꼬리 (%85=1 & %89=0 & %254=1 · L704 `for e in near_allies` 루프 내부, L705 skill.target.check(champ,e) 통과 후) =====
L713: max_dist = [L709 합(배치 E): skill.range(champ) + walk*move_speed + skill.range_adjust(champ,e) + champ.radius()] + e.radius()      // %473 = %472 + %450
L714: if e.distance_sq(champ)[L710 %467] > max_dist*max_dist → continue (→%493, L704 다음 아군)
L718: candidates.push(SmallActionPlay::Skill(SmallActionSkill::new(data, e.id)))   // 태그 16 @+0xb1 · len==cap 이면 reserve_internal_or_panic(len,1,true) 후 ptr[len]←184B, len+=1
      → continue (%493)
// L704 루프 종료(%409=1, 블록 %494):
L721: if skill.target.check(&skill.target, champ, champ) [자기 대상 허용]
        && should_add_self_etc_buff_action(player, data, champ, &skill)   // fastcc ArgumentPromotion: &Effect → (ty.ptr %498, ty.vtable %499)
L722:   candidates.push(Skill(SmallActionSkill::new(data, champ.id)))   // 태그 16
      → %234 (스킬1 분기 합류점)

// ===== (B) 스킬2 분기 (블록 %234 · 세 스킬1 분기 모두 여기로 합류) =====
L726: if skill2.is_some() (%99==0) && champ.can_skill2() (%521)      // 둘 중 하나라도 거짓 → L785 반환
L727:   for (act, e) in near_enemies_with_action:                       // %546 = 원소 ptr, 32B stride
L728:     if !e.is_visible_from(champ) → continue                         // 인라인 entity.rs:1481~1483: champ.team 이 Neutral(tag 1) 이면 true · Player(t) 이면 e.visible_state[t].tag == 0(Visible) [data.rs:122]
L732:     if !skill2.target.check(champ, e) → continue                    // CastingTarget::check(&skill2.target(+0x28), champ, e)
L736:     move_speed' = (*act == SmallAction::RunAway [tag 0] && !e.block_move())
L737:                   ? champ.move_speed.saturating_sub(e.stat_cached.move_speed)      // llvm.usub.sat — 도망치는 적은 상대속도로
                        : champ.move_speed
L742:     max_dist = skill2.range(champ) + skill2.range_adjust(champ, e) + champ.radius() + e.radius()
L743:     dist_sq = e.distance_sq(champ)
L746:     max_dist += move_speed' * 30                                      // 30틱 이동 여유 (합산 순서: ms*30 + range + growth*(lv-1) + buff_range + range_adjust + champ.r + e.r)
L747:     if dist_sq > max_dist*max_dist → continue
L752:     if skill2.ty.can_move() [EffectType vtable+0x120, Arc::deref 인라인] && e.is_champion() [ty tag == 13]:
L753:        skill_damage = skill2.expected_damage_target(ctx, champ as &dyn AbstractEntity, e)
L754:        if !is_dash_worth(data, player, champ, e, skill_damage) → continue      // 이동형(대시) 스킬2는 적 챔피언에게 가치 있을 때만
L759:     candidates.push(Skill2(SmallActionSkill2::new(data, e.id)))   // 태그 17
      (루프 종료 %547=1 → 블록 %677)
L762:   walk = ally_buff_walk_allowance(_version, data, champ, &skill2) [인라인 fight_check.rs:1181~1183]
            = skill2.ty.expected_buff_deep(ctx, champ as &dyn AbstractEntity) [vtable+0xa0, sret Option<BuffState> 288B] .is_none() [+0x48 tag == -1] ? 30 : 90
L763:   for e in near_allies:                                              // %701, 8B stride
L764:     if !skill2.target.check(champ, e) → continue
L768:     max_dist = skill2.range(champ) + walk*champ.move_speed + skill2.range_adjust(champ, e) + champ.radius()    // 합산 순서: range + walk*ms + growth*(lv-1) + buff_range + range_adjust + champ.r
L769:     dist_sq = e.distance_sq(champ)
L772:     max_dist += e.radius()
L773:     if dist_sq > max_dist*max_dist → continue
L777:     candidates.push(Skill2(SmallActionSkill2::new(data, e.id)))   // 태그 17 · 아군 대상은 대시/피해 판정 없음
      (루프 종료 %702=1 → 블록 %787)
L780:   if skill2.target.check(champ, champ) && should_add_self_skill2_action(player, data, champ, &skill2) [인라인 fight_check.rs:612~617]:
            L613: action = champ.skill2() [entity.rs:1668~1669: level > 2 ? &champ.skill2(+0x590) : &champ.empty(+0x5b0)]  (Box<dyn Action>)
            L614: if let Some(p) = action.as_any() [Action vtable+0x68] .downcast_ref::<PrisonerSkill2Action>() [Any vtable+0x18 type_id == i128 1684068…3956]
            L615:     → p.has_enemy_champion_target_or_action_threat(data.cache.game (&dyn AbstractGame), champ)
            L617: else → should_add_self_etc_buff_action(player, data, champ, &skill2)   // 프로모션 (%791 ty.ptr, %792 ty.vtable) · llvm.assume non-null
L781:     candidates.push(Skill2(SmallActionSkill2::new(data, champ.id)))   // 태그 17 자기 대상
      → %523

// ===== (C) 반환·정리 =====
L785: *sret = candidates (memcpy 32B ← %24)
L786: drop(near_enemies_with_action %26) [Vec<(SmallAction,&Entity)> Drop::drop + RawVec drop] · drop(near_allies %28) · drop(_t: Option<ProfTimer> %29): +0x10 tag != -1 이면 PHASE_NANOS[phase(71)] += elapsed_ns, PHASE_CALLS[71] += 1 (phase ≥ 132 이면 bounds panic — 계측, 판정 무관) · ret
// 언와인드 정리(블록 %44/%77/%105/%486/%513/%669/%779/%834): 각각 ProfTimer / near_allies / candidates+near_enemies / push 중 임시 SmallActionPlay drop_glue — 판정 무관
// 다른 배치로 넘어가는 지점: 블록 %449 의 앞부분(%450~%472 · L709/L710)은 배치 E · %493(→ L704 루프 헤더 %407) 은 배치 E · %234 의 선행자 %108/%233 (L667 can_skill 거짓·attack 분기) 은 배치 E
```

**`mem` 메모리 접근 94건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | L622 team(0/1) · <2 bounds check · 클로저 s_0(aux m12) 도 1-team 계산에 읽음 | 4 | OK |  |
| 1 | PlayerState | 0x9c0 | info.position@tag | r | L622 i32 zext → player_champion 행 인덱스(0..5) | 4 | OK |  |
| 2 | OperationData | 0x0 | cache | r | L622 &AbstractGameWithCache · aux s0_0 에서 cache.game(dyn 팻포인터 +0/+8) 을 can_target 에 전달 | 4 | OK |  |
| 3 | OperationData | 0x8 | context | r | L625 &GameContext · expected_damage_target/expected_buff_deep 의 ctx 인자 | 4 | OK |  |
| 4 | OperationData | 0x10 | blackboard | r | L629 &[Blackboard;2] (클로저 s_0 캡처) | 4 | OK |  |
| 5 | GameContext | 0x0 | pool | r | L625 bump(&Bump) — 세 Vec 의 할당자 | 4 | OK |  |
| 6 | AbstractGameWithCache | 0x1e0 | player_champion[team][pos] | r | L622 [2][5] Option<&Entity>(행 stride 40) · null=None → unwrap 패닉. L625 자기 행 5칸 → iter_champions, L628 상대 행(1-team) 5칸 | 4 | OK |  |
| 7 | Blackboard | 0x78 | small_actions[i] | r | aux m12 L629 · 원소 24B Option<SmallAction<&Entity>> · blackboard[1-team] 의 i 번째(24B memcpy) | 4 | OK |  |
| 8 | Entity | 0x0 | team@tag | r | L644/L669 champ.team: 0=Player(+0x8 idx) · 1=Neutral (is_visible_from 인라인) | 4 | OK |  |
| 9 | Entity | 0x8 | team@Player.0 | r | L644/L669 champ 팀 인덱스 t(<2 bounds check) | 4 | OK |  |
| 10 | Entity | 0x38 | visible_state[t]@tag | r | L644/L669 적 e.visible_state[t] (stride 24) 태그 0=Visible 만 통과 | 4 | OK |  |
| 11 | Entity | 0x68 | ty@tag | r | L693 e.is_champion() ⇔ ==13(Champion) | 4 | OK |  |
| 12 | Entity | 0x438 | stat_buff_cached.range | r | Effect::range(champ) 가산항 (L648/L683/L709 champ) | 4 | OK |  |
| 13 | Entity | 0x470 | stat_buff_cached.radius_mult | r | Entity::radius() 인라인: 0 이면 radius 그대로, 아니면 radius*(mult+100)/100 (champ·e·a 각각) | 4 | OK |  |
| 14 | Entity | 0x490 | attack_effect@Some.0 (Effect 56B) | r | L642~663 range_adjust(&attack,..) self | 4 | OK |  |
| 15 | Entity | 0x4a0 | attack_effect.range | r | L648 | 4 | OK |  |
| 16 | Entity | 0x4a8 | attack_effect.growth_range | r | L648 ×(level-1) | 4 | OK |  |
| 17 | Entity | 0x4c0 | attack_effect@tag (casting 니치) | r | L636 == -1 ⇔ attack None | 4 | OK |  |
| 18 | Entity | 0x4c8 | skill_effect@Some.0 (Effect 56B) / ty.ptr | r | L667~714 range_adjust·expected_damage_target self · +0 Arc<dyn EffectType> 데이터 ptr(L693/L703) | 4 | OK |  |
| 19 | Entity | 0x4d0 | skill_effect.ty vtable ptr | r | L693 슬롯 +0x120(can_move) · L703 슬롯 +0xa0(expected_buff_deep) · vtable+16 = align(ArcInner 데이터 오프셋 계산) | 4 | OK |  |
| 20 | Entity | 0x4d8 | skill_effect.range | r | L683/L709 | 4 | OK |  |
| 21 | Entity | 0x4e0 | skill_effect.growth_range | r | L683/L709 ×(level-1) | 4 | OK |  |
| 22 | Entity | 0x4f0 | skill_effect.target (CastingTarget 4B) | r | L673/L705 CastingTarget::check(&target, champ, e\|a) | 4 | OK |  |
| 23 | Entity | 0x4f8 | skill_effect@tag (casting 니치) | r | L637 == -1 ⇔ skill None | 4 | OK |  |
| 24 | Entity | 0x500 | skill2_effect@Some.0 | r | L638 level>2 일 때만 참조(아니면 정적 NONE @anon.58) — 사용은 배치 F | 4 | OK |  |
| 25 | Entity | 0x530 | skill2_effect@tag (casting 니치, %96+48) | r | L638 == -1 ⇔ skill2 None (정적 NONE 도 +0x30 = -1). ⚠C3 경고 사유: 본문엔 절대 1328 이 아니라 `select(level>2 ? champ+1280 : @anon.58)` 뒤 `gep 48` 로 나타난다(Entity+0x500+0x30) — 접힘 아님, 상대 gep | 4 | OK |  |
| 26 | Entity | 0x5c0 | id | r | L663/L700 SmallActionAttack/Skill::new(data, e.id) · aux closure0 a.id != champ.id | 4 | OK |  |
| 27 | Entity | 0x5c8 | level | r | L638 skill2 게이트(>2) · L648/683/709 growth_range×(level-1) | 4 | OK |  |
| 28 | Entity | 0x640 | stat_cached.move_speed | r | L640 champ move_speed · L653/L678 e.move_speed(RunAway 상대속도 차감) | 4 | OK |  |
| 29 | Entity | 0x660 | x | r | distance_sq(abs_diff²) L649/L684/L710 · aux closure0 | 4 | OK |  |
| 30 | Entity | 0x668 | y | r | distance_sq | 4 | OK |  |
| 31 | Entity | 0x680 | radius | r | Entity::radius() 인라인(champ·e·a) | 4 | OK |  |
| 32 | (SmallAction<&Entity>, &Entity) 원소(32B, near_enemies_with_action) | 0x0 | .0 SmallAction@tag(i64) | r | L652/L677 ==0 ⇔ RunAway (derive PartialEq 판별자 비교, blackboard.rs:81) | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) |  |
| 33 | (SmallAction<&Entity>, &Entity) 원소(32B) | 0x18 | .1 &Entity | r | L644/L669 e (stride 32) | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) |  |
| 34 | Option<BuffState>(288B, expected_buff_deep sret) | 0x48 | @tag (i32 니치) | r | L703 == -1 ⇔ None → walk 30, Some → 90 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 35 | prof::ENABLED (static atomic u8) | 0x0 | ENABLED | r | L621 monotonic load · 0 이면 ProfTimer None(계측 무시) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 36 | OperationData(%4) | 0x0 | cache | r | &AbstractGameWithCache (%51) — L615 has_enemy_champion_target_or_action_threat 에 cache.game 전달 (정의는 L622 · 배치 E) | 4 | OK |  |
| 37 | OperationData(%4) | 0x8 | context | r | &GameContext (%62) — L753 expected_damage_target · L762 expected_buff_deep 인자 (정의는 L625) | 4 | OK |  |
| 38 | AbstractGameWithCache(%51) | 0x0 | game.data_ptr | r | L615 &dyn AbstractGame 팻포인터 data (%817) | 4 | OK |  |
| 39 | AbstractGameWithCache(%51) | 0x8 | game.vtable_ptr | r | L615 &dyn AbstractGame 팻포인터 vtable (%819) | 4 | OK |  |
| 40 | Entity(champ %55) | 0x0 | team@tag | r | L728 is_visible_from 인라인: tag bit0 (TeamType 0=Player 1=Neutral) — trunc i64→i1, 1이면 무조건 가시 | 4 | OK |  |
| 41 | Entity(champ %55) | 0x8 | team@Player.0 | r | L728: 관측 팀 인덱스 t (visible_state[t] 첨자, <2 bounds check) | 4 | OK |  |
| 42 | Entity(e %551) | 0x38 | visible_state[t]@tag | r | L728: stride 24 (VisibleState 24B) · tag 0=Visible 이어야 후보 | 4 | OK |  |
| 43 | Entity(e %551) | 0x68 | ty@tag | r | L752 is_champion(): EntityType 태그 13=Champion | 4 | OK |  |
| 44 | Entity(champ %55) | 0x438 | stat_buff_cached.range | r | L742/L768 Effect::range 인라인 항 (gep 1080) | 4 | OK |  |
| 45 | Entity(champ/e/ally) | 0x470 | stat_buff_cached.radius_mult | r | Entity::radius 인라인(entity.rs:1511) — 0 이면 radius 그대로 (gep 1136) | 4 | OK |  |
| 46 | Entity(champ/e/ally) | 0x680 | radius | r | Entity::radius: mult==0 ? radius : radius*(mult+100)/100 (gep 1664) | 4 | OK |  |
| 47 | Entity(champ %55) | 0x5c0 | id | r | L722/L781 자기 대상 push 의 target (gep 1472) | 4 | OK |  |
| 48 | Entity(e/ally) | 0x5c0 | id | r | L718/L759/L777 SmallActionSkill(2)::new(data, e.id) (gep 1472) | 4 | OK |  |
| 49 | Entity(champ %55) | 0x5c8 | level | r | growth_range*(level-1) (%538=%93-1) · L613 Entity::skill2(): level>2 ? skill2 : empty (gep 1480) | 4 | OK |  |
| 50 | Entity(champ %55) | 0x640 | stat_cached.move_speed | r | %102 — L737 saturating_sub 좌변 · L746 *30 · L768 walk*move_speed (gep 1600) | 4 | OK |  |
| 51 | Entity(e %551) | 0x640 | stat_cached.move_speed | r | L737: act==RunAway && !block_move 일 때 champ.move_speed 에서 뺌 (gep 1600) | 4 | OK |  |
| 52 | Entity(champ/e/ally) | 0x660 | x | r | L743/L769 (배치 E 는 L710) Entity::distance_sq → utils::distance_sq (abs_diff² 합) (gep 1632) | 4 | OK |  |
| 53 | Entity(champ/e/ally) | 0x668 | y | r | distance_sq (gep 1640) | 4 | OK |  |
| 54 | Entity(champ %55) | 0x590 | skill2 (Box<dyn Action> 팻포인터) | r | L613 Entity::skill2() level>2 분기 (select 1424) | 4 | OK |  |
| 55 | Entity(champ %55) | 0x5b0 | empty (Box<dyn Action>) | r | L613 level<=2 분기 (select 1456) | 4 | OK |  |
| 56 | Effect(skill = champ+0x4c8, %90/%91) | 0x0 | ty.ptr (ArcInner<dyn EffectType>) | r | L721 should_add_self_etc_buff_action 프로모션 인자 %498 | 4 | OK |  |
| 57 | Effect(skill) | 0x8 | ty.vtable | r | L721 프로모션 인자 %499 (%386=%91+8) | 4 | 부분일치(첫 성분(컨테이너)만 맞고 **마지막 성분이 다르다** — 사람이 봐야 ) |  |
| 58 | Effect(skill) | 0x28 | target (CastingTarget) | r | L721 CastingTarget::check(&skill.target, champ, champ) (%241=champ+1264) | 4 | OK |  |
| 59 | Effect(skill2 = champ+0x500, %96/%100) | 0x0 | ty.ptr | r | L752 can_move · L762 expected_buff_deep · L780 프로모션 인자 %791 — ArcInner 데이터 = ptr + ((vtable.align-1)&-16) + 16 | 4 | OK |  |
| 60 | Effect(skill2) | 0x8 | ty.vtable | r | 슬롯 +0x120 can_move(L752) · +0xa0 expected_buff_deep(L762) · L780 %792 | 4 | 부분일치(첫 성분(컨테이너)만 맞고 **마지막 성분이 다르다** — 사람이 봐야 ) |  |
| 61 | Effect(skill2) | 0x10 | range | r | L742/L768 Effect::range 인라인 (effect.rs:26) | 4 | OK |  |
| 62 | Effect(skill2) | 0x18 | growth_range | r | × (champ.level-1) | 4 | OK |  |
| 63 | Effect(skill2) | 0x28 | target (CastingTarget) | r | L732/L764/L780 CastingTarget::check (%534=%96+40) | 4 | OK |  |
| 64 | (SmallAction,&Entity) 원소(%546, near_enemies_with_action 32B) | 0x0 | SmallAction@tag | r | L736 == 0 (RunAway) — 파생 PartialEq 가 discr 비교로 접힘 (blackboard.rs:81) | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) |  |
| 65 | (SmallAction,&Entity) 원소(%546) | 0x18 | &Entity e | r | L728 e (gep 24) | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) |  |
| 66 | bumpalo Vec candidates(%24) | 0x0 | buf.ptr | r | push 시 원소 주소 = ptr + len*184 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 67 | bumpalo Vec candidates(%24) | 0x10 | buf.cap | r | len==cap 이면 reserve_internal_or_panic(len, 1, true) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 68 | bumpalo Vec candidates(%24) | 0x18 | len | r | push 후 +1 store | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 69 | bumpalo Vec near_enemies_with_action(%26) | 0x0 | buf.ptr | r | L727 iter 시작 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 70 | bumpalo Vec near_enemies_with_action(%26) | 0x18 | len | r | L727 iter 끝 = ptr + len*32 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 71 | bumpalo Vec near_allies(%28) | 0x0 | buf.ptr | r | L763 iter 시작 (원소 8B &Entity) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 72 | bumpalo Vec near_allies(%28) | 0x18 | len | r | L763 iter 끝 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 73 | Option<BuffState>(sret %8 288B) | 0x48 | duration@tag (BuffType i32) = Option 니치 | r | L762: -1 → None → walk 30, else 90 (gep 72) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 74 | vtable<dyn EffectType>(skill2.ty.vtable) | 0x10 | align | r | Arc::deref 인라인: 데이터 오프셋 = ((align-1) & -16) + 16 (sync.rs:2445) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 75 | vtable<dyn EffectType> | 0xa0 | expected_buff_deep(&self,&GameContext,&dyn AbstractEntity)->Option<BuffState> | r | L762 (divtable g02.ll 일치율 94%) | 3 | 확인불가(tcx 사전에 타입 없음) |  |
| 76 | vtable<dyn EffectType> | 0x120 | can_move(&self)->bool | r | L752 (divtable) | 3 | 확인불가(tcx 사전에 타입 없음) |  |
| 77 | vtable<dyn Action>(champ.skill2/empty +8) | 0x68 | as_any(&self)->&dyn Any | r | L614 (divtable Action 일치율 58% — 이름은 dloc 사슬 downcast_ref 로 교차확인) | 3 | 확인불가(tcx 사전에 타입 없음) |  |
| 78 | vtable<dyn Any> | 0x18 | type_id(&self)->TypeId(16B sret) | r | L614 downcast_ref::<PrisonerSkill2Action> 인라인 (any.rs:229/204) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 79 | Option<ProfTimer>(%29, 지역) | 0x10 | 니치 태그 i32 | r | L786 drop: -1(None, prof::ENABLED==0) 이면 계측 생략 (gep 16) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 80 | Option<ProfTimer>(%29) | 0x0 | phase (=71, L621 store) | r | L786: PHASE_NANOS[phase]/PHASE_CALLS[phase] 첨자 (<132 bounds check) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 81 | Option<ProfTimer>(%29) | 0x8 | Instant | r | L786 Instant::elapsed | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 82 | candidates (지역 %24, bumpalo Vec<SmallActionPlay> 32B → 배치 F 에서 sret 로 복사) | 0x0 | buf.ptr | w | L634 Vec::new_in(bump) · push 시 len==cap 이면 reserve_internal_or_panic(used_cap, 1, true) | 4 | 확인불가(tcx 사전에 타입 없음) | inttoptr 8 (dangling) → reserve 후 bump 힙 ptr |
| 83 | candidates (지역 %24) | 0x8 | buf.a | w | L634 | 4 | 확인불가(tcx 사전에 타입 없음) | bump(= data.context.pool) |
| 84 | candidates (지역 %24) | 0x10 | cap | w | L634 | 4 | 확인불가(tcx 사전에 타입 없음) | 0 (memset 16B) → reserve 가 갱신 |
| 85 | candidates (지역 %24) | 0x18 | len | w | 배치 E 의 push 2사이트(적 Attack · 적 Skill). 아군 Skill push(L718)·셀프 Skill(L722)·Skill2 전부(L759/777/781)는 배치 F | 4 | 확인불가(tcx 사전에 타입 없음) | 0 → push 마다 +1 (L663 Attack · L700 Skill) |
| 86 | candidates 원소(bump 힙, 184B) | 0xb1 | SmallActionPlay@tag | w | alloca %23/%21 에 24B 페이로드 memcpy + 태그 store 후 184B 통째 memcpy → 원소 +0x11..0xb1·+0xb2.. 는 undef | 4 | 확인불가(tcx 사전에 타입 없음) | 15(Attack, L663) / 16(Skill, L700) |
| 87 | candidates 원소 | 0x0 | payload SmallActionAttack/SmallActionSkill(+0 start_tick · +8 target=e.id · +0x10 is_act) | w | L663/L700 | 4 | 확인불가(tcx 사전에 타입 없음) | SmallActionAttack::new(data, e.id) / SmallActionSkill::new(data, e.id) 의 sret 24B(initializes((0,17))) |
| 88 | _t (지역 %29, Option<ProfTimer> 24B) | 0x0 | phase=71 · +8 Instant · +0x10 nanos(-1=None) | w | L621 · 판정 무관 · 배치 F L786 에서 drop(elapsed 기록) | 4 | 확인불가(tcx 사전에 타입 없음) | prof::start(71) — 계측 |
| 89 | sret(%0) | 0x0 | candidates 32B | w | L785 · 유일한 인자 쓰기(&mut 인자 없음: _rnd 는 readnone 미사용) | 4 | 확인불가(tcx 사전에 타입 없음) | memcpy(%0 ← %24, 32) |
| 90 | bump 힙: candidates.ptr[len] (184B 원소) | 0x0 -> +0x18 | SmallActionSkill / SmallActionSkill2 (new() 이 [0,17) 기록) | w | L718/L722(Skill) · L759/L777/L781(Skill2) | 4 | 확인불가(tcx 사전에 타입 없음) | memcpy 24B(alloca %18→%19 … ) 후 원소 전체 184B memcpy |
| 91 | bump 힙: candidates.ptr[len] | 0xb1 | SmallActionPlay 니치 태그 | w | alloca +177 에 store 후 원소 memcpy | 4 | 확인불가(tcx 사전에 타입 없음) | 16(Skill) / 17(Skill2) |
| 92 | 지역 candidates(%24) | 0x18 | len | w | push 마다 · cap 부족 시 reserve_internal_or_panic 이 +0/+0x10 갱신 | 4 | 확인불가(tcx 사전에 타입 없음) | len+1 |
| 93 | @PHASE_NANOS[71] / @PHASE_CALLS[71] (전역 atomic) | 0x0 | 계측 카운터 | w | L786 ProfTimer drop · prof::ENABLED 일 때만 · 판정 무관 | 4 | 확인불가(tcx 사전에 타입 없음) | atomicrmw add elapsed_ns / 1 |

**`consts` 상수 26건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev | folded_from |
|---|---|---|---|---|---|---|
| 0 | 71 | 621 | 산출값 | ProfTimer phase 번호(prof::start(71)) — 계측 전용, 판정 무관 | 4 |  |
| 1 | 2 | 622 | 임계 | 팀 인덱스 상한(team<2, panic_bounds_check) · L644/L669 is_visible_from 의 t<2 · L638 skill2_effect(): level>2 이어야 skill2_effect 참조(아니면 정적 NONE). ※ level>2 vs level>=3 는 외연 동일 = 표기 불가 | 4 |  |
| 2 | -1 | 636 | 센티널 | Option<Effect> None 니치(casting@tag +0x30 == -1): attack(+0x4c0)·skill(+0x4f8)·skill2(+0x530) is_none 판정 · L703 Option<BuffState>@+0x48 == -1 ⇔ None(walk 30) | 4 |  |
| 3 | 0 | 644 | 태그 | VisibleState::Visible 태그(적이 내 팀에 보일 때만 통과) · L652/L677 SmallAction::RunAway 판별자 0 · radius_mult==0 이면 radius 무보정 | 4 |  |
| 4 | 30 | 658 | 계수 | 이동 여유 계수: max_dist = range + move_speed*30 (L658 Attack · L687 Skill) · L703 ally_buff_walk_allowance 의 버프 없음 쪽 walk=30 (max_dist 에 walk*move_speed 가산) | 4 |  |
| 5 | 90 | 703 | 산출값 | ally_buff_walk_allowance: skill.ty.expected_buff_deep(ctx, champ).is_some() 이면 walk=90(아군 버프 스킬은 3배 멀리까지 걸어가 쓸 후보로) | 4 |  |
| 6 | 100 | 648 | 계수 | Entity::radius() 인라인(entity.rs:1511~1515): radius*(radius_mult+100)/100 (udiv) — 퍼센트 보정 | 4 |  |
| 7 | 13 | 693 | 태그 | EntityType::Champion 태그(e.is_champion()) — can_move(대시류) 스킬은 적 챔피언에게만 is_dash_worth 검사 | 4 |  |
| 8 | 15 | 663 | 태그 | SmallActionPlay::Attack 메모리 태그(+0xb1) | 4 |  |
| 9 | 16 | 700 | 태그 | SmallActionPlay::Skill 메모리 태그(+0xb1) | 4 |  |
| 10 | 22500000000 | 625 | 미상 | 150000² — near_allies 반경(aux closure0 m15.ll:58168, icmp ult 즉 거리²<150000², 150000/32000≈4.69셀). 본체엔 없고 aux 심에 있음 | 4 |  |
| 11 | 8 | 634 | 미상 | bumpalo Vec::new_in 의 빈 버퍼 dangling ptr(inttoptr 8 = align) — 판정값 아님, 초기화 표면 기록용 | 4 |  |
| 12 | 30 | 746 | 계수 | 적 대상 스킬2: max_dist += move_speed' * 30 (30틱 = 0.5초@60tps 이동 여유) | 4 |  |
| 13 | 30 | 762 | 계수 | ally_buff_walk_allowance(인라인 fight_check.rs:1183): skill2.ty.expected_buff_deep(ctx,champ) 가 None 이면 walk=30 (틱) | 4 |  |
| 14 | 90 | 762 | 산출값 | ally_buff_walk_allowance: expected_buff_deep 가 Some 이면 walk=90 (딥버프 스킬은 아군에게 더 멀리 걸어감) | 4 |  |
| 15 | 13 | 752 | 태그 | EntityType 메모리태그 13 = Champion (Entity::is_champion 인라인 entity.rs:1404) — 대시 가치 판정은 적 챔피언 대상일 때만 | 4 |  |
| 16 | 0 | 736 | 태그 | SmallAction 태그 0 = RunAway (*act == SmallAction::RunAway) · L728 VisibleState 태그 0 = Visible | 4 |  |
| 17 | 16 | 718 | 센티널 | SmallActionPlay 니치태그 16 = Skill (원소+0xb1) — L718·L722 push | 4 |  |
| 18 | 17 | 759 | 센티널 | SmallActionPlay 니치태그 17 = Skill2 — L759·L777·L781 push | 4 |  |
| 19 | 100 | 742 | 계수 | Entity::radius 인라인(entity.rs:1515): radius*(radius_mult+100)/100 — L713·L742·L768·L772 의 반지름 항 | 4 |  |
| 20 | -1 | 762 | 센티널 | Option<BuffState> 니치 None(+0x48 BuffType tag) · L786 Option<ProfTimer> None(+0x10) · (E) skill2_effect None(+0x530) | 4 |  |
| 21 | -1 | 742 | 태그 | growth_range * (level - 1) — `add i64 %93, -1` 로 접힘 (%538) | 4 | 1 |
| 22 | 2 | 780 | 임계 | Entity::skill2() 인라인(entity.rs:1669, should_add_self_skill2_action L613): level > 2 이면 &champ.skill2(+0x590) 아니면 &champ.empty(+0x5b0) | 4 |  |
| 23 | 168406848281932906149591046147716593956 | 780 | 태그 | TypeId(PrisonerSkill2Action) i128 — L614 as_any().downcast_ref::<PrisonerSkill2Action>() (dloc: any.rs is<…PrisonerSkill2Action>) | 4 |  |
| 24 | 132 | 786 | 길이 | prof::PHASE_NANOS/PHASE_CALLS 배열 길이(bounds check) — 계측, 판정 무관 | 4 |  |
| 25 | 1000000000 | 786 | 임계 | Duration → ns (secs*1e9 + nanos) — 계측, 판정 무관 | 4 |  |

**`knobs` 조정점 9건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | near_allies 반경(아군 스킬 후보 탐색 반경) | fight_check.rs:625 (aux m15.ll:58168, closure0) | 22500000000 | 올리면 더 먼 아군(150000=약 4.69셀 이상)까지 아군 대상 Skill/Skill2 후보에 들어간다 · 내리면 가까운 아군만 | 4 | 기존 |
| 1 | 적 대상 이동 여유 계수(max_dist = range + ms*30) | fight_check.rs:658 (Attack) · :687 (Skill) · 배치 F 의 Skill2 동형 | 30 | 올리면 사거리 밖(30틱 이동 거리 이상)의 적도 Attack/Skill 후보로 넣는다(후속 평가가 걸러줘야 함) · 내리면 사거리 근처 적만 | 4 | 기존 |
| 2 | 아군 버프 스킬 도달 여유(walk) | fight_check.rs:1183 ally_buff_walk_allowance (703 에 인라인) | 90 | expected_buff_deep 이 Some 인 스킬(버프류)에서 아군 후보 max_dist 에 walk*move_speed 가산. 90→ 낮추면 버프 스킬을 멀리 걸어가 쓰는 후보가 줄고, 30(비버프) 쪽을 올리면 비버프 아군 스킬도 멀리서 후보화 | 4 | 기존 |
| 3 | RunAway 상대속도 차감 | fight_check.rs:652~653 · 677~678 | 0 | 적 blackboard small_action 이 RunAway(태그 0)이고 block_move 아님 → ms = 내 ms − 적 ms(saturating). 조건을 없애면 도망 적도 절대속도로 계산해 후보가 늘어난다 | 4 | 기존 |
| 4 | 적 대상 스킬2 이동 여유 틱 | fight_check.rs:746 | 30 | 올리면 더 먼 적(도망 중이면 상대속도 기준)도 스킬2 후보에 넣는다 · 내리면 사거리 안 적만 | 4 | 기존 |
| 5 | 아군 대상 스킬2 도보 여유(딥버프 아님) | fight_check.rs:1183 (ally_buff_walk_allowance, L762 인라인) | 30 | 올리면 버프 스킬2를 위해 더 먼 아군에게도 후보 생성 | 4 | 기존 |
| 6 | 아군 대상 스킬2 도보 여유(expected_buff_deep Some) | fight_check.rs:1183 (L762 인라인) | 90 | 딥버프 스킬2는 아군 3배 거리까지 후보 · 내리면 30 과 같아짐 | 4 | 기존 |
| 7 | 대시 가치 게이트 대상 종류 | fight_check.rs:752 | 13 | Champion(13) 외 적(미니언·정글 등)에는 can_move 스킬2 를 is_dash_worth 검사 없이 후보로 넣는다 — 조건을 없애면 모든 적에 대시 가치 판정 | 4 | 기존 |
| 8 | 스킬2 자기 대상 특례 챔피언 | fight_check.rs:614~615 | TypeId(PrisonerSkill2Action) | Prisoner 스킬2만 has_enemy_champion_target_or_action_threat 로 자기시전 판정 · 그 외는 should_add_self_etc_buff_action | 4 | 기존 |

<details><summary>`callees` 피호출자 48건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | ally_buff_walk_allowance | game_ai::fight_check::ally_buff_walk_allowance | in:game_ai | fn(usize, &game_core::OperationData, &game_core::Entity, &game_core::Effect) -> u64 | game-ai\src\fight_check.rs:1181 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 1 | as_any | <game_ai::AgentVerHamster as game_core::AiAgent>::as_any | pub | fn(&game_ai::AgentVerHamster) -> &dyn [Binder { value: Trait(std::any::Any), bound_vars: [] }] + 'static | game-ai\src\lib.rs:425 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 321개 중 상위 3개 |
| 2 | as_any | game_core::Action::as_any | pub | fn(&Self/#0) -> &dyn [Binder { value: Trait(std::any::Any), bound_vars: [] }] + 'static | game-core\src\setting\action.rs:12 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 321개 중 상위 3개 |
| 3 | as_any | game_core::AiAgent::as_any | pub | fn(&Self/#0) -> &dyn [Binder { value: Trait(std::any::Any), bound_vars: [] }] + 'static | game-core\src\simulation\ai_interface.rs:499 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 321개 중 상위 3개 |
| 4 | attack_effect | game_core::Entity::attack_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1684 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 5 | block_move | game_core::Entity::block_move | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1497 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 6 | can_attack | game_core::Entity::can_attack | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1527 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 7 | can_move | game_core::Entity::can_move | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1489 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 4개 중 상위 3개 |
| 8 | can_move | game_core::Champion::can_move | pub | fn(&game_core::Champion) -> bool | game-core\src\simulation\entity\champion.rs:48 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 4개 중 상위 3개 |
| 9 | can_move | game_core::EffectType::can_move | pub | fn(&Self/#0) -> bool | game-core\src\simulation\effect\type.rs:358 | True | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 4개 중 상위 3개 |
| 10 | can_skill | game_core::Entity::can_skill | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1708 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 11 | can_skill2 | game_core::Entity::can_skill2 | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1721 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 12 | can_target | game_core::OperationData::<'a, 'b>::can_target | pub | fn(&game_core::OperationData<'a/#0, 'b/#1>, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\state\player.rs:23 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 13 | check | game_core::CastingTarget::check | pub | fn(&game_core::CastingTarget, &game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\effect\type.rs:227 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 14 | distance_sq | game_core::utils::distance_sq | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:6 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 15 | distance_sq | game_core::Entity::distance_sq | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2157 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 16 | expected_buff_deep | game_core::EffectType::expected_buff_deep | pub | fn(&Self/#0, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + ) -> std::option::Option<game_core::BuffState> | game-core\src\simulation\effect\type.rs:309 | True | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 27개 중 상위 3개 |
| 17 | expected_buff_deep | <game_core::RushEffect as game_core::EffectType>::expected_buff_deep | pub | fn(&game_core::RushEffect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + ) -> std::option::Option<game_core::BuffState> | game-core\src\simulation\effect\type\rush.rs:59 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 27개 중 상위 3개 |
| 18 | expected_buff_deep | <game_core::RangeEffect as game_core::EffectType>::expected_buff_deep | pub | fn(&game_core::RangeEffect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + ) -> std::option::Option<game_core::BuffState> | game-core\src\simulation\effect\type\range_effect.rs:110 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 27개 중 상위 3개 |
| 19 | expected_damage_target | game_core::Effect::expected_damage_target | pub | fn(&game_core::Effect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + , &game_core::Entity) -> usize | game-core\src\simulation\effect.rs:91 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 20 | has_enemy_champion_target_or_action_threat | game_core::PrisonerSkill2Action::has_enemy_champion_target_or_action_threat | pub | fn(&game_core::PrisonerSkill2Action, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::Entity) -> bool | game-core\src\setting\champion\prisoner.rs:241 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 21 | is_champion | game_core::EntityType::is_champion | pub | fn(&game_core::EntityType) -> bool | game-core\src\simulation\entity.rs:1403 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 22 | is_dash_worth | game_ai::is_dash_worth | pub | fn(&game_core::OperationData, &game_core::PlayerState, &game_core::Entity, &game_core::Entity, usize) -> bool | game-ai\src\utils.rs:98 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 23 | is_visible_from | game_core::Entity::is_visible_from | pub | fn(&game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1481 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 24 | iter_champions | game_core::AbstractGameWithCache::<'a, 'b>::iter_champions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5759 ~ game_core[6a30]::simulation::{impl#5}::iter_champions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1904 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 25 | level | <game_ai::WindowStatView<'_> as game_core::AbstractEntity>::level | pub | fn(&game_ai::WindowStatView</#0>) -> usize | game-ai\src\fight_check.rs:96 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 26 | level | game_core::AbstractEntity::level | pub | fn(&Self/#0) -> usize | game-core\src\simulation\entity.rs:718 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 27 | level | <game_core::Entity as game_core::AbstractEntity>::level | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:772 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 28 | new | game_ai::SmallActionSkill::new | pub | fn(&game_core::OperationData, usize) -> game_ai::SmallActionSkill | game-ai\src\small_action\cast.rs:119 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 29 | new | game_ai::SmallActionAttack::new | pub | fn(&game_core::OperationData, usize) -> game_ai::SmallActionAttack | game-ai\src\small_action\cast.rs:23 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 30 | push | <game_core::Staff as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::staff | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\staff.rs:38 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 31 | push | <game_core::Athlete as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:434 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 32 | push | <game_core::Contract as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:2498 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 33 | radius | game_core::Entity::radius | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1509 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 34 | range | game_core::Effect::range | pub | fn(&game_core::Effect, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:25 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 35 | range_adjust | game_core::Effect::range_adjust | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:29 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 36 | should_add_self_etc_buff_action | game_ai::fight_check::should_add_self_etc_buff_action | in:game_ai | fn(&game_core::PlayerState, &game_core::OperationData, &game_core::Entity, &game_core::Effect) -> bool | game-ai\src\fight_check.rs:608 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 37 | should_add_self_skill2_action | game_ai::fight_check::should_add_self_skill2_action | in:game_ai | fn(&game_core::PlayerState, &game_core::OperationData, &game_core::Entity, &game_core::Effect) -> bool | game-ai\src\fight_check.rs:612 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 38 | skill2 | game_ai::skill2 | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_core::Entity) -> std::option::Option<game_core::Input> | game-ai\src\abstract_input.rs:239 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 65개 중 상위 3개 |
| 39 | skill2 | game_core::Entity::skill2 | pub | fn(&game_core::Entity) -> &std::boxed::Box<dyn [Binder { value: Trait(game_core::Action), bound_vars: [] }] + 'static, std::alloc::Global> | game-core\src\simulation\entity.rs:1668 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 65개 중 상위 3개 |
| 40 | skill2 | game_core::ChampionInfo::skill2 | pub | fn(&Self/#0) -> std::boxed::Box<dyn [Binder { value: Trait(game_core::Action), bound_vars: [] }] + 'static, std::alloc::Global> | game-core\src\setting.rs:1086 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 65개 중 상위 3개 |
| 41 | skill2_effect | game_core::Entity::skill2_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1692 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 42 | skill_effect | game_core::Entity::skill_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1688 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 43 | start | game_core::prof::start | pub | fn(usize) -> std::option::Option<game_core::prof::ProfTimer> | game-core\src\simulation\prof.rs:175 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 44 | start | game_view::UIPhaseEffect::start | pub | fn(&mut game_view::UIPhaseEffect) | game-view\src\ui\match_ui\phase_effect.rs:30 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 45 | team | game_view::ClientData::team | pub | fn(&game_view::ClientData, usize) -> std::option::Option<std::cell::Ref< game_core::Team>> | game-view\src\logic\client\data.rs:1651 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 46 | team | game_view::ClientDatabase::team | pub | fn(&game_view::ClientDatabase, usize) -> std::option::Option<&game_core::Team> | game-view\src\logic\client\data.rs:968 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 47 | team | game_core::PlayerAiContext::<'a, 'b, 'r>::team | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:583 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
</details>

⚠**미매칭 21개**: `bool`, `candidates`, `continue`, `dereferenceable`, `drop  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `drop_glue<Option<ProfTimer>>`, `drop_glue<SmallActionPlay>`, `drop_glue<bumpalo Vec<`, `drop_glue<bumpalo Vec<&Entity>>`, `drop_glue<bumpalo Vec<SmallActionPlay>>`, `elapsed`, `empty`, `enumerate`, `initializes`, `llvm.usub.sat.i64`, `move_speed`, `phase`, `pool`, `reserve_internal_or_panic`, `target`, `try_fold`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 23곳** (m02.ll:10870, m02.ll:11712, m02.ll:19881, m02.ll:20000, m02.ll:23732, m02.ll:23790, m02.ll:25182, m02.ll:25240, m02.ll:25317, m02.ll:38700, m02.ll:40228, m02.ll:46668, m02.ll:47693, m14.ll:18512, m14.ll:19420, m14.ll:22688, m14.ll:27373, m14.ll:31464, m14.ll:47484, m15.ll:13470, m15.ll:20339, m15.ll:20422, m15.ll:21774) · **형제 0개** 

**`open` 16건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 표기 불가 | (배치 E) level>2 vs level>=3 (entity.rs:1693 skill2_effect 게이트) — 외연 동일, 표기 불가 | 4 |  |
| 1 | 미탐색 | (배치 E) is_dash_worth(utils.rs:98, m04.ll:48412) 내부 판정 — 시그니처·반환 의미(bool=대시 스킬을 이 적에게 쓸 가치)만. 미독(후속 라운드) | 4 |  |
| 2 | 미탐색 | (배치 E) OperationData::can_target(g15.ll:66255) 내부 — 미독(game_core 경계). 필터 s0_0 의 통과 조건으로만 기록 | 4 |  |
| 3 | 재료 부재 | (배치 E) EffectType::can_move(vtable+0x120)/expected_buff_deep(+0xa0) 의 런타임 구현체 — divtable 은 슬롯 이름만(CombineEffect 판 기준 94% 일치) · 어느 impl 이 꽂히는지는 챔피언별 Arc 라 정적 확정 불가 | 3 |  |
| 4 | 미탐색 | (배치 E) player.info.position@tag(i32) → player_champion 열 인덱스(0..4) 의 variant 대응표는 이 함수엔 없음(값 그대로 인덱스) | 4 |  |
| 5 | 미탐색 | (배치 E) Effect::range_adjust / expected_damage_target / CastingTarget::check 내부 — game_core 경계, 시그니처만 | 4 |  |
| 6 | 미탐색 | (배치 E) @anon.58 정적 NONE(Option<Effect>) 은 [48 x i8] undef + FF FF FF FF + undef — +0x30 태그 외 바이트는 undef 이므로 level<=2 경로에서 skill2 의 다른 필드를 읽는 코드가 있으면 UB 가능성(배치 F 범위에서 skill2 None 이면 안 읽으므로 실제로는 안전 — F 확인 필요) | 4 |  |
| 7 | 미탐색 | (배치 E) ProfTimer(_t) phase 71 의 의미(어느 계측 버킷인지) — 계측 전용이라 미추적 | 4 |  |
| 8 | 표기 불가 | (배치 F) L709 vs L713 항 귀속: `%473 = add %472, %450 ;L713` 에서 %450(e.radius())의 인라인 dbg 는 ;L1511<709 이라 e.radius() 호출 자체는 L709 소스에 있고 L713 은 덧셈만 — 소스 줄바꿈 위치라 동작엔 무관(표기 불가 범주 아님 · 줄 경계만 불확실) | 4 |  |
| 9 | 재료 부재 | (배치 F) Effect::range(&self,&Entity)->u64 (effect.rs:25~26, xinl) 이 range_adjust 를 포함하는지: tcx 시그니처가 2인자라 range_adjust(champ,e) 는 L742/L768 에서 직접 호출로 판단 — IR 의 range_adjust invoke 에 effect.rs:26 inlinedAt 이 없어 일치. 합산 순서(ms*30 이 첫 항)는 IR 정본이고 소스 표기 순서는 column 부재로 확정 불가 | 3 |  |
| 10 | 미탐색 | (배치 F) should_add_self_etc_buff_action(m15.ll:34622 internal fastcc)·is_dash_worth(m04.ll:48412)·expected_damage_target·has_enemy_champion_target_or_action_threat·CastingTarget::check·block_move·range_adjust 내부는 미열람(계약만 · 지시 규칙) | 4 |  |
| 11 | 미탐색 | (배치 F) vtable<dyn Action>+0x68=as_any 는 divtable 일치율 58% — dloc 사슬(any.rs downcast_ref/is<PrisonerSkill2Action>)로 as_any 호출임은 확정, 슬롯 이름의 vtable 실체(어느 impl 이 꽂히는지)는 런타임 미확인 | 3 |  |
| 12 | 미탐색 | (배치 F) SmallActionSkill/Skill2 원소 [0x11,0x18) 패딩·[0x18,0xb1)·[0xb2,0xb8) 은 alloca 잔류값(미기록) — sweep 비교 시 마스킹 필요 · 정확한 잔류 내용은 미탐색 | 4 |  |
| 13 | 표기 불가 | (배치 F) _version(%1)·_end_delay(%5)·_rnd(%2): 본문 1913줄 전체에서 define 줄 외 참조 0 — ally_buff_walk_allowance(version,…) 인라인에도 version 분기 없음(상수접힘인지 원래 미사용인지 표기 불가) | 4 |  |
| 14 | 미탐색 | (배치 F) TLS 접점: 본문에 llvm.threadlocal.address·call_once·__getit·fn-포인터 상수 경유 참조 0 → 없음(전 범위 grep) | 4 |  |
| 15 | 미탐색 | (배치 F) prof::ENABLED / PHASE_NANOS / PHASE_CALLS 는 계측 전역 — 판정 무관으로 분류, 값 의미(phase 71 이름)는 미탐색 | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

