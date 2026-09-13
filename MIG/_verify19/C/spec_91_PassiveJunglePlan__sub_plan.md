---

### `91` PassiveJunglePlan::sub_plan — 패시브 정글 플랜의 서브플랜 — check_recall(인라인)이 참이면 Recall, 아니면 Jungle{team,camp,check_move:false}

| 항목 | 값 |
|---|---|
| id | `passive_jungle__PassiveJunglePlan_sub_plan` |
| 심볼 | `_RNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old14passive_jungleNtB2_17PassiveJunglePlan8sub_plan` |
| 소스 | `game-ai\src\plan_legacy\old\passive_jungle.rs:134` |
| IR | `m04.ll` 28488~29367행 |
| 경로·가시성 | `game_ai::plan_legacy::old::PassiveJunglePlan::sub_plan` · **pub** |
| 계층 | 레거시 플랜 |
| exe | `d2e500` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r10` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_ai::plan_legacy::old::PassiveJunglePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> game_ai::plan_legacy::sub_plan::SubPlan
```

<details><summary>인자 7개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | SubPlan(72B) | %0. 태그 5=Recall / 6=Jungle(payload +8 team, +0x10 camp, +0x11 check_move) | 4 |
| 1 | 1 | self | &PassiveJunglePlan(104B) | %1. team(+0x48)·jungle(+0x60) 만 읽음 | 4 |
| 2 | 2 | version | usize | %2. effect_buff_target 에 그대로 전달만 — 본문 분기 없음 | 4 |
| 3 | 3 | rnd | &mut StdRng(320B) | %3. readnone — 미사용 | 4 |
| 4 | 4 | player | &PlayerState(2528B) | %4 | 4 |
| 5 | 5 | data | &OperationData(24B) | %5 | 4 |
| 6 | 6 | debug | &mut DebugFrameData(224B) | %6. readnone — 미사용 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
sub_plan(L135~140) = if self.check_recall(version,rnd,player,data,debug) { Recall } else { Jungle{team:self.team, camp:self.jungle, check_move:false} }
check_recall 전체가 인라인(passive_jungle.rs:143~206):
L143: team = player.info.team; champ = cache.player_champion[team][player.info.position].unwrap()
L145: (lx,ly,rx,ry) = context.map.fountains[team]
L146: is_in_heal_area = lx<=champ.x<=rx && ly<=champ.y<=ry
L147: if is_in_heal_area && champ.hp < champ.stat_cached.hp { return true(Recall) }   // 분수 안에서 회복 중이면 계속 귀환
L154: moba = cache.game.get_game_mode().as_moba().unwrap(); camp_live = moba.jungle_runner.get_jungle_live_list(self.jungle, self.team == 0)
L155: if !camp_live.is_empty() {
  L156: camp_pos = map.camp_pos(self.jungle, self.team == 0)
  L157: if champ.distance_sq(camp_pos) < 14400000001 {
    L159~162: edpt = camp_live.iter().filter_map(|id| get_entity_by_id(id)).map(|e| e.attack_effect.as_ref().map(|ae| ae.expected_damage_target(ctx, e, champ) * 1000 / e.attack_cooltime()).unwrap_or(0)).sum()   (cooltime 0 → div_by_zero panic)
    L164: die_tick = if edpt == 0 { usize::MAX } else { champ.hp * 1000 / edpt }
    L165: if die_tick > tps { return false(Jungle) }   // 캠프 옆에서 1초 안에 안 죽으면 계속 정글
  } }
L172: can_heal = champ.stat_buff_cached.vamp > 0
  L173~175: || champ.skill_effect.as_ref().map(|e| e.expected_heal_target(ctx, champ, champ) > 0 || effect_buff_target(version, e, ctx, champ, champ).map(|b| b.vamp > 0).unwrap_or(false)).unwrap_or(false)
  L176~178: || champ.skill2_effect()[level>2 게이트].as_ref().map(같은 식).unwrap_or(false)
  (IR 순서: vamp → skill1 heal → skill1 buff → skill2 heal → skill2 buff, 단락 평가)
L180: is_in_fight = get_jungle_live_list(self.jungle, self.team==0).iter().any(|id| L181 get_entity_by_id(id).is_some_and(|e| L182 e.ty == Jungle && L184 e.ty.info.focused == Some(champ.id)))
L191: if is_in_fight {
  L192: camp_live = get_jungle_live_list(…)  (3번째 호출)
  L193~195: edpt = max(camp_live.iter().filter_map(get_entity_by_id).map(|e| e.attack_effect.as_ref().unwrap().expected_damage_target(ctx,e,champ)*1000 / e.attack_cooltime()).sum(), 1)   (★attack_effect None 이면 unwrap 패닉 — L162 와 달리 unwrap_or 없음)
  L197: die_tick = champ.hp * 1000 / edpt
  L199: return die_tick <= tps   // 1초 안에 죽으면 Recall, 아니면 Jungle
} else {
  L205: hp_ratio = champ.hp * 100 / champ.stat_cached.hp   (max 0 → div_by_zero panic)
  L206: return if can_heal { hp_ratio < 21 } else { hp_ratio < 41 }
}
```

**`mem` 메모리 접근 36건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | PassiveJunglePlan | 0x48 | team | r | 28503. `team==0` 이 get_jungle_live_list/camp_pos 의 bool 인자(28624) · Jungle payload team 에 복사 | 4 | OK |  |
| 1 | PassiveJunglePlan | 0x60 | jungle | r | JungleType i8 (28505) — 캠프 종류 | 4 | OK |  |
| 2 | PlayerState | 0x930 | info.team | r | 28506. <2 아니면 bounds panic — ★self.team 이 아니라 player.info.team 으로 champ 를 찾음 | 4 | OK |  |
| 3 | PlayerState | 0x9c0 | info.position@tag | r | as_index (28535) | 4 | OK |  |
| 4 | OperationData | 0x0 | cache | r | 28508 | 4 | OK |  |
| 5 | OperationData | 0x8 | context | r | 28510 | 4 | OK |  |
| 6 | AbstractGameWithCache | 0x1e0 | player_champion | r | unwrap (28541~28546) | 4 | OK |  |
| 7 | AbstractGameWithCache | 0x0 | game.data_ptr | r | 28596 | 4 | OK |  |
| 8 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r | 슬롯 0x40=get_game_mode(28599; 3회 호출) · 0x1f0=get_entity_by_id(28755, 29025, 29211) | 4 | OK |  |
| 9 | GameMode | 0x0 | tag | r | {i64,ptr} tag 0=Moba 아니면 as_moba().unwrap() 패닉 (28605) | 4 | OK |  |
| 10 | MobaMode | 0x18 | jungle_runner | r | JungleRunner(480B) — get_jungle_live_list 의 &self (28623) | 4 | OK |  |
| 11 | GameContext | 0x20 | map | r | 28554 | 4 | OK |  |
| 12 | GameContext | 0x8 | setting | r | 28867, 29331 | 4 | OK |  |
| 13 | GameSetting | 0x12f8 | tick_per_second | r | tps — die_tick 비교 기준 (28869, 29333) | 4 | OK |  |
| 14 | MapDef | 0x6d70 | fountains | r | (lx,ly,rx,ry) stride 32B: +0 lx(28559) +8 ly(28583) +16 rx(28562) +24 ry(28580) | 4 | OK |  |
| 15 | Entity | 0x660 | x | r | champ.x (28566) | 4 | OK |  |
| 16 | Entity | 0x668 | y | r | champ.y (28586, 28669) | 4 | OK |  |
| 17 | Entity | 0x670 | hp | r | champ.hp (28609, 28857, 29125, 29324) | 4 | OK |  |
| 18 | Entity | 0x628 | stat_cached.hp | r | 최대 HP (28611, 29110) — 0 이면 div_by_zero panic(29133) | 4 | OK |  |
| 19 | Entity | 0x3f0 | stat_buff_cached.vamp | r | i32 > 0 이면 can_heal (28654~28656) | 4 | OK |  |
| 20 | Entity | 0x4f8 | skill_effect@tag | r | i32 -1 = None (28893~28895) | 4 | OK |  |
| 21 | Entity | 0x4c8 | skill_effect@Some.0 | r | &Effect(56B) → expected_heal_target / effect_buff_target (28899) | 4 | OK |  |
| 22 | Entity | 0x5c8 | level | r | skill2_effect() 인라인: level > 2 이면 &skill2_effect 아니면 정적 None(@anon.16) (28931~28935) | 4 | OK |  |
| 23 | Entity | 0x500 | skill2_effect@Some.0 | r | &Option<Effect> (+48 = 0x530 tag) (28934, 28937) | 4 | OK |  |
| 24 | Entity | 0x4c0 | attack_effect@tag | r | 정글 몹의 평타 이펙트 Option 니치 -1 (28791, 29246) | 4 | OK |  |
| 25 | Entity | 0x490 | attack_effect@Some.0 | r | 정글 몹 &Effect → expected_damage_target(…, champ) (28797, 29252) | 4 | OK |  |
| 26 | Entity | 0x68 | ty@tag | r | 정글 몹 EntityType 태그 == 4(Jungle) (29051~29053) | 4 | OK |  |
| 27 | Entity | 0x88 | ty@Jungle.info.focused@tag | r | Entity+0x70 payload + Jungle.focused(0x18): Option<usize> tag (29060~29062) | 4 | OK |  |
| 28 | Entity | 0x90 | ty@Jungle.info.focused@Some.0 | r | 정글 몹이 노리는 엔티티 id == champ.id 면 is_in_fight (29066, 29072~29073) | 4 | OK |  |
| 29 | Entity | 0x5c0 | id | r | champ.id (29027, 29067) | 4 | OK |  |
| 30 | BuffState | 0x48 | duration@tag | r | Option<BuffState> None 니치(-1) 판정 = is_some (28921~28922, 28964~28965) | 4 | OK |  |
| 31 | BuffState | 0x80 | vamp | r | i32 > 0 → 흡혈 버프 스킬 (28924~28925, 28967~28968) | 4 | OK |  |
| 32 | SubPlan(sret) | 0x0 | tag | w | 29365. phi 29364: %335 경로만 6 | 4 | OK | 5(Recall) \| 6(Jungle) |
| 33 | SubPlan(sret) | 0x8 | Jungle.team | w | 29356 (태그 6 경로만) | 4 | OK | self.team |
| 34 | SubPlan(sret) | 0x10 | Jungle.camp | w | 29358 | 4 | OK | self.jungle |
| 35 | SubPlan(sret) | 0x11 | Jungle.check_move | w | 29360 | 4 | OK | false(0) |

**`consts` 상수 10건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 14400000001 | 157 | 임계 | 120000^2 + 1 — champ↔캠프 좌표 distance_sq < 14400000001 = 거리 ≤ 120000(3.75셀) 이면 '캠프에 붙어 있음' | 4 |
| 1 | 1000 | 162 | 계수 | 틱당 기대피해 = expected_damage*1000 / attack_cooltime (×1000 스케일). die_tick = hp*1000 / 합산 (L164, L197) | 4 |
| 2 | 0 | 164 | 센티널 | 합산 피해 0 이면 die_tick = usize::MAX(-1) (L164) / vamp > 0, heal > 0 판정 (L172, L174, L177) | 4 |
| 3 | -1 | 164 | 센티널 | usize::MAX (die_tick 무한) · Option 니치 None 태그(skill_effect/attack_effect/BuffState.duration) | 4 |
| 4 | 2 | 176 | 임계 | skill2_effect() 인라인(entity.rs:1693): level > 2 일 때만 스킬2 존재 | 4 |
| 5 | 4 | 182 | 태그 | EntityType 태그 4 = Jungle — 정글 몹만 focused 검사 | 4 |
| 6 | 1 | 194 | 인덱스 | is_in_fight 경로 합산 피해 max(sum,1) — 0 나눗셈 방지 (llvm.umax) | 4 |
| 7 | 100 | 205 | 계수 | hp_ratio = hp*100 / stat_cached.hp | 4 |
| 8 | 21 | 206 | 임계 | can_heal(흡혈/힐 보유) 시 귀환 HP%: hp_ratio < 21 | 4 |
| 9 | 41 | 206 | 임계 | can_heal 아님 시 귀환 HP%: hp_ratio < 41 | 4 |

**`knobs` 조정점 5건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 귀환 HP% (흡혈/힐 보유) | passive_jungle.rs:206 | 21 | 올리면 흡혈·힐 챔프도 더 높은 HP 에서 귀환 | 4 | 기존 |
| 1 | 귀환 HP% (일반) | passive_jungle.rs:206 | 41 | 올리면 정글러가 더 일찍 귀환(정글 시간 손실↑), 내리면 저체력 정글 지속 | 4 | 기존 |
| 2 | 캠프 근접 반경 | passive_jungle.rs:157 | 14400000001 | 올리면 더 먼 곳에서도 '캠프 옆 die_tick 판정'을 타서 HP% 귀환 판정을 건너뛴다(die_tick>tps 면 Jungle 유지) | 4 | 기존 |
| 3 | die_tick 지평 | passive_jungle.rs:165,199 (tps 그대로) | 1000 | die_tick 은 ×1000 스케일이라 tps(=1초)와 비교. 계수를 바꾸면 '몇 초 안에 죽는가' 지평이 바뀐다 — 1000 을 2000 으로 하면 2초 지평 | 4 | 기존 |
| 4 | 스킬2 해금 레벨 | entity.rs:1693 (인라인) | 2 | level>2 에서만 스킬2 힐/흡혈이 can_heal 에 기여 (game_core 쪽 규칙, AI 노브 아님) | 4 | 기존 |

<details><summary>`callees` 피호출자 25건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | as_moba | game_core::GameMode::<'a>::as_moba | pub | fn(game_core::GameMode<'a/#0>) -> std::option::Option<&'a/#0 game_core::MobaMode> | game-core\src\simulation\game.rs:230 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 1 | attack_cooltime | game_core::Entity::attack_cooltime | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1766 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 2 | camp_pos | game_core::MapDef::camp_pos | pub | fn(&game_core::MapDef, game_core::JungleType, bool) -> (u64, u64) | game-core\src\simulation\map_def.rs:207 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 3 | check_recall | game_ai::plan_legacy::old::SinglePlanLine::check_recall | in:game_ai::plan_legacy::old::single_line | fn(&game_ai::plan_legacy::old::SinglePlanLine, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> bool | game-ai\src\plan_legacy\old\single_line.rs:281 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 4 | check_recall | game_ai::plan_legacy::old::PassiveLinePlan::check_recall | in:game_ai::plan_legacy::old::passive_line | fn(&game_ai::plan_legacy::old::PassiveLinePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> bool | game-ai\src\plan_legacy\old\passive_line.rs:1068 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 5 | check_recall | game_ai::plan_legacy::old::PassiveJunglePlan::check_recall | in:game_ai::plan_legacy::old::passive_jungle | fn(&game_ai::plan_legacy::old::PassiveJunglePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> bool | game-ai\src\plan_legacy\old\passive_jungle.rs:142 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 6 | distance_sq | game_core::utils::distance_sq | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:6 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 7 | distance_sq | game_core::Entity::distance_sq | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2157 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 8 | effect_buff_target | game_ai::effect_buff_target | pub | fn(usize, &game_core::Effect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + , &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + ) -> std::option::Option<game_core::BuffState> | game-ai\src\fight_check.rs:390 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 9 | expected_damage_target | game_core::Effect::expected_damage_target | pub | fn(&game_core::Effect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + , &game_core::Entity) -> usize | game-core\src\simulation\effect.rs:91 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 10 | expected_heal_target | game_core::Effect::expected_heal_target | pub | fn(&game_core::Effect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + , &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + ) -> usize | game-core\src\simulation\effect.rs:109 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 11 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 12 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 13 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 14 | get_game_mode | game_core::AbstractGame::get_game_mode | pub | fn(&Self/#0) -> game_core::GameMode | game-core\src\simulation.rs:78 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 15 | get_game_mode | <game_core::Game as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::Game) -> game_core::GameMode | game-core\src\simulation\game.rs:1707 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 16 | get_game_mode | <game_core::SingleLaneGame as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::SingleLaneGame) -> game_core::GameMode | game-core\src\simulation\game.rs:4248 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 17 | get_jungle_live_list | game_core::JungleRunner::get_jungle_live_list | pub | fn(&game_core::JungleRunner, game_core::JungleType, bool) -> std::vec::Vec<usize, std::alloc::Global> | game-core\src\simulation\entity\jungle.rs:743 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 18 | is_empty | game_core::PatchSetting::is_empty | pub | fn(&game_core::PatchSetting) -> bool | game-core\src\patch_engine.rs:853 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 19 | is_empty | game_core::ModAiRegistry::is_empty | pub | fn(&game_core::ModAiRegistry) -> bool | game-core\src\mod_ai.rs:15 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 20 | is_empty | game_core::PublicChampionEvidence::is_empty | pub | fn(&game_core::PublicChampionEvidence) -> bool | game-core\src\banpick\public_evidence.rs:96 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 21 | skill2_effect | game_core::Entity::skill2_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1692 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 22 | sub_plan | game_ai::plan_legacy::types::BigPlan::sub_plan | pub | fn(&mut game_ai::plan_legacy::types::BigPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PositioningScoreData, &mut game_core::DebugFrameData) -> game_ai::plan_legacy::sub_plan::SubPlan | game-ai\src\plan_legacy\types.rs:230 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 16개 중 상위 3개 |
| 23 | sub_plan | game_ai::plan_legacy::old::BattlePlan::sub_plan | pub | fn(&game_ai::plan_legacy::old::BattlePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> game_ai::plan_legacy::sub_plan::SubPlan | game-ai\src\plan_legacy\old\battle.rs:2043 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 16개 중 상위 3개 |
| 24 | sub_plan | game_ai::plan_legacy::old::SinglePlanLine::sub_plan | pub | fn(&game_ai::plan_legacy::old::SinglePlanLine, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> game_ai::plan_legacy::sub_plan::SubPlan | game-ai\src\plan_legacy\old\single_line.rs:84 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 16개 중 상위 3개 |
</details>

⚠**미매칭 3개**: `drop  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `llvm.assume`, `llvm.umax.i64`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m02.ll:8425) · **형제 16개** (PassiveJunglePlan)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::plan_legacy::old::PassiveJunglePlan as std::clone::Clone>::clone | pub | game-ai\src\plan_legacy\old\passive_jungle.rs:20 | True | fn(&game_ai::plan_legacy::old::PassiveJunglePlan) -> game_ai::plan_legacy::old::PassiveJunglePlan |
| 1 | <game_ai::plan_legacy::old::PassiveJunglePlan as std::fmt::Debug>::fmt | pub | game-ai\src\plan_legacy\old\passive_jungle.rs:20 | True | fn(&game_ai::plan_legacy::old::PassiveJunglePlan, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 2 | game_ai::plan_legacy::old::PassiveJunglePlan::new | pub | game-ai\src\plan_legacy\old\passive_jungle.rs:35 | False | fn(&mut rand::rngs::std::StdRng, usize) -> game_ai::plan_legacy::old::PassiveJunglePlan |
| 3 | game_ai::plan_legacy::old::PassiveJunglePlan::with_best | pub | game-ai\src\plan_legacy\old\passive_jungle.rs:51 | False | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> game_ai::plan_legacy::old::PassiveJunglePlan |
| 4 | game_ai::plan_legacy::old::PassiveJunglePlan::new_counter_jungle | pub | game-ai\src\plan_legacy\old\passive_jungle.rs:63 | False | fn(&mut rand::rngs::std::StdRng, usize, game_core::FocusedAreaStrategy) -> game_ai::plan_legacy::old::PassiveJunglePlan |
| 5 | game_ai::plan_legacy::old::PassiveJunglePlan::is_counter_jungle | pub | game-ai\src\plan_legacy\old\passive_jungle.rs:103 | True | fn(&game_ai::plan_legacy::old::PassiveJunglePlan) -> bool |
| 6 | game_ai::plan_legacy::old::PassiveJunglePlan::update | pub | game-ai\src\plan_legacy\old\passive_jungle.rs:107 | False | fn(&mut game_ai::plan_legacy::old::PassiveJunglePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) |
| 7 | game_ai::plan_legacy::old::PassiveJunglePlan::goal | pub | game-ai\src\plan_legacy\old\passive_jungle.rs:130 | True | fn(&game_ai::plan_legacy::old::PassiveJunglePlan) -> game_core::BigGoal |
| 8 | game_ai::plan_legacy::old::PassiveJunglePlan::sub_plan | pub | game-ai\src\plan_legacy\old\passive_jungle.rs:134 | False | fn(&game_ai::plan_legacy::old::PassiveJunglePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> game_ai::plan_legacy::sub_plan::SubPlan |
| 9 | game_ai::plan_legacy::old::PassiveJunglePlan::check_recall | in:game_ai::plan_legacy::old::passive_jungle | game-ai\src\plan_legacy\old\passive_jungle.rs:142 | False | fn(&game_ai::plan_legacy::old::PassiveJunglePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> bool |
| 10 | game_ai::plan_legacy::old::PassiveJunglePlan::next_plan | pub | game-ai\src\plan_legacy\old\passive_jungle.rs:228 | False | fn(&mut game_ai::plan_legacy::old::PassiveJunglePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &game_core::PositioningScoreData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> std::option::Option<game_ai::plan_legacy::types::BigPlan> |
| 11 | game_ai::plan_legacy::old::PassiveJunglePlan::has_deep_pushed_enemy | in:game_ai::plan_legacy::old::passive_jungle | game-ai\src\plan_legacy\old\passive_jungle.rs:305 | False | fn(&game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, game_core::LineType) -> bool |
| 12 | game_ai::plan_legacy::old::PassiveJunglePlan::valid_deep_push_gank | in:game_ai::plan_legacy::old::passive_jungle | game-ai\src\plan_legacy\old\passive_jungle.rs:337 | False | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, game_core::LineType, &mut game_core::DebugFrameData) -> bool |
| 13 | game_ai::plan_legacy::old::PassiveJunglePlan::lead_action_v37 | in:game_ai::plan_legacy::old::passive_jungle | game-ai\src\plan_legacy\old\passive_jungle.rs:386 | False | fn(&mut game_ai::plan_legacy::old::PassiveJunglePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut game_core::DebugFrameData) -> std::option::Option<game_ai::plan_legacy::types::BigPlan> |
| 14 | game_ai::plan_legacy::old::PassiveJunglePlan::valid_gank_line | in:game_ai::plan_legacy::old::passive_jungle | game-ai\src\plan_legacy\old\passive_jungle.rs:636 | False | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan, game_core::LineType, &mut game_core::DebugFrameData) -> bool |
| 15 | game_ai::plan_legacy::old::PassiveJunglePlan::lead_action | in:game_ai::plan_legacy::old::passive_jungle | game-ai\src\plan_legacy\old\passive_jungle.rs:685 | False | fn(&mut game_ai::plan_legacy::old::PassiveJunglePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut game_core::DebugFrameData) -> std::option::Option<game_ai::plan_legacy::types::BigPlan> |

**`open` 6건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 표기 불가 | L164 die_tick 이 usize::MAX 인 경우(캠프 몹 피해 0)와 L165 `> tps` 비교가 소스에서 `unwrap_or(usize::MAX)` 인지 `if edpt==0 {MAX}` 인지 — 외연 동일, 표기 불가 | 4 |  |
| 1 | 표기 불가 | L173~178 의 `\|\|` 결합에서 skill1 vs skill2 순서는 IR 블록 순서(28654→28893→28931)로 복원 — 같은 줄 안 순서는 column 부재로 표기 불가 | 4 |  |
| 2 | 미탐색 | effect_buff_target(fight_check.rs:390) 반환 Option<BuffState> 의 None 니치가 duration@tag == -1 로 표현된다는 것은 IR 검사식(28921)에서 역산 — BuffType 태그 값 표는 미조회 | 4 |  |
| 3 | 미탐색 | get_jungle_live_list(jungle.rs:743)·camp_pos(map_def.rs:207)·expected_heal_target·effect_buff_target 본문 미독 — 계약만 기재 | 4 |  |
| 4 | 미탐색 | L192 에서 get_jungle_live_list 를 3번째로 다시 호출하는 이유(L154 결과 재사용 안 함) — 소스 구조 추정 불가, 동작상 동일 목록 | 5 |  |
| 5 | 미탐색 | rnd/debug 인자는 readnone 이라 미사용 확정, version 은 effect_buff_target 인자로만 전달 | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

