---

### `148` serpen_action_score — 세르펜 사냥 행동 점수 = interaction_score + 행동별 보정(RunAway: 위험옵션 시 근접 아군 없으면 -10 · Trace: 세르펜 캠프가 사거리 밖이면 -99999 · Attack/Skill/Skill2: calculate_serpen_action_score, 대상 없으면 -99999) · 세르펜 대상 공격류는 최소 1

| 항목 | 값 |
|---|---|
| id | `serpen_hunt__serpen_action_score` |
| 심볼 | `_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy8sub_plan11serpen_hunt19serpen_action_score` |
| 소스 | `game-ai\src\plan_legacy\sub_plan\serpen_hunt.rs:843` |
| IR | `m02.ll` 64597~65165행 |
| 경로·가시성 | `game_ai::plan_legacy::sub_plan::serpen_action_score` · **pub** |
| 계층 | 기타 |
| exe | `ccf670` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r14` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64
```

<details><summary>인자 7개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | version | usize | interaction_score 전달만 | 4 |
| 1 | 2 | rnd | &mut StdRng(320B) | interaction_score 전달만 — 이 함수 자체 write 0건. calculate_serpen_action_score 에는 전달 안 됨(DeadArg 제거) | 4 |
| 2 | 3 | player | &PlayerState(2528B) | info.team(+0x930)·info.position 태그(+0x9c0) ; calculate_* 에 전달 | 4 |
| 3 | 4 | data | &OperationData(24B) | cache→player_champion·champions·game.get_entity_by_id ; context→pool(+0)·map(+0x20) | 4 |
| 4 | 5 | parameter | &ScoreParameter(5384B) | player.risk_epic_damage(+0x9a0) 읽음 ; interaction_score 전달 | 4 |
| 5 | 6 | action | &SmallActionPlay(184B) | get_action() 인라인(태그 +0xb1) · target_id(+0x8) | 4 |
| 6 | 7 | debug | &mut DebugFrameData(224B) | interaction_score 전달만 — 이 함수 자체 write 0건 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn serpen_action_score(version, rnd, player, data, parameter, action, debug) -> i64   // serpen_hunt.rs:843
  let champ = data.cache.player_champion[player.info.team][player.info.position].unwrap();   // L844
  let base = interaction_score(version, rnd, player, data, parameter, action, debug);          // L845
  let mut score = base + match action.get_action() {   // L847
    SmallAction::RunAway /*Play 0·1·5*/ => {                                                  // L849
      if parameter.player.risk_epic_damage == 0 { return base }   // 64676 → %269(base)  ⚠match 값이 아니라 조기 반환(L896 검사 안 거침 — 결과 동일)
      let champions = data.cache.champions(player.info.team, ctx.pool);                       // L851 (bumpalo Vec<&Entity>, 아군 팀 챔피언)
      let near = champions.iter().any(|x| x.distance_sq(champ) <= 40000000000 && x.hp >= champ.hp);   // L852 (64806~64826: dist²>2e11 → 다음, x.hp<champ.hp → 다음)
      drop(champions);                                                                        // 64840 (bumpalo dealloc 인라인)
      if near { 0 } else { -10 }                                                              // 64838 phi
    }
    SmallAction::Trace{..} /*Play 11*/ => {                                                   // L885
      let camp = map.camp_pos(JungleType::Serpen /*5*/, true);                                // L885 (dbg 이름 epic_camp_pos)
      let attack_dist = champ.attack_effect.unwrap().range(champ);                             // L886 = eff.range + 30000 + champ.stat_buff_cached.range + (champ.level-1)*eff.growth_range + champ.radius()  [radius() = radius_mult==0 ? radius : radius*(mult+100)/100]
      if champ.distance_sq(camp) < attack_dist² { 0 } else { -99999 }                         // L887 (65066~65067) → 반환(L896 검사 없음)
    }
    SmallAction::Attack{target_id} /*Play 12*/ => match game.get_entity_by_id(target_id) { None => -99999, Some(t) => calculate_serpen_action_score(player, data, champ.attack_effect.as_ref().unwrap() /*+0x490*/, t) }   // L862~863
    SmallAction::Skill{target_id}  /*Play 13*/ => … calculate_serpen_action_score(player, data, champ.skill_effect.unwrap() /*+0x4c8*/, t)   // L869~870
    SmallAction::Skill2{target_id} /*Play 14*/ => … calculate_serpen_action_score(player, data, champ.skill2_effect().unwrap() /*level>2 ? +0x500 : None→패닉*/, t)   // L876~877
    SmallAction::Around{..}(Play 2·3·10) | Ult(15) | Stop(16) => 0                            // 64981 phi 0
    SmallAction::Positioning|AroundPosition (Play 4·6·7·8·9) => return base                   // 65163 phi %24 (조기반환, 결과 동일)
  };
  // L896~908: if let Some(target_id) = action.target_id() /*Attack·Skill·Skill2 만*/ { if let Some(t) = game.get_entity_by_id(target_id) { if t.ty.is_serpen() /*태그 6*/ { score = max(score, 1) } } }
  score   // L912
```

**`mem` 메모리 접근 31건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | m02.ll:64608 | 4 | OK |
| 1 | PlayerState | 0x9c0 | info.position@tag | r | m02.ll:64619 | 4 | OK |
| 2 | OperationData | 0x0 | cache | r | m02.ll:64622 | 4 | OK |
| 3 | OperationData | 0x8 | context | r | m02.ll:64680, 64742 | 4 | OK |
| 4 | GameContext | 0x0 | pool (&Bump) | r | m02.ll:64744 → champions() 할당자 | 4 | OK |
| 5 | GameContext | 0x20 | map (&MapDef) | r | m02.ll:64682 → camp_pos | 4 | OK |
| 6 | AbstractGameWithCache | 0x1e0 | player_champion[team][pos] | r | m02.ll:64623~64626 None→unwrap_failed(64670) | 4 | OK |
| 7 | AbstractGameWithCache | 0x0 | game.data_ptr | r | m02.ll:64701 등 | 4 | OK |
| 8 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r | m02.ll:64702 등 | 4 | OK |
| 9 | vtable(AbstractGame) | 0x1f0 | get_entity_by_id | r | m02.ll:64704~64706, 64719~64721, 64734~64736, 65141~65143 | 4 | 확인불가(tcx 사전에 타입 없음) |
| 10 | ScoreParameter | 0x9a0 | player.risk_epic_damage | r | m02.ll:64674 — 0 이면 RunAway 보정 생략 (tcxdict ScoreParameter 0x9a0) | 3 | OK |
| 11 | SmallActionPlay | 0xb1 | @tag (니치) | r | m02.ll:64639 | 4 | OK |
| 12 | SmallActionPlay | 0x8 | target_id (Attack cast.rs:94 / Skill :160 / Skill2 :222) | r | m02.ll:64698, 64713, 64728, 64990 | 4 | 오귀속(사전은 다른 필드를 준다) |
| 13 | Entity | 0x660 | x | r | champ(64776, 65040)·아군 x(64794) | 4 | OK |
| 14 | Entity | 0x668 | y | r | champ(64778, 65044)·아군(64798) | 4 | OK |
| 15 | Entity | 0x670 | hp | r | champ(64780)·아군(64823) — 아군.hp < champ.hp 면 제외 | 4 | OK |
| 16 | Entity | 0x4c0 | attack_effect@tag (i32, -1=None) | r | champ m02.ll:64690(Trace), 65076(Attack) | 4 | OK |
| 17 | Entity | 0x490 | attack_effect@Some.0 (&Effect) | r | champ m02.ll:65082 → calculate_serpen_action_score | 4 | OK |
| 18 | Entity | 0x4a0 | attack_effect.range | r | m02.ll:64997 Effect::range 인라인(effect.rs:26) | 4 | OK |
| 19 | Entity | 0x4a8 | attack_effect.growth_range | r | m02.ll:64999 | 4 | OK |
| 20 | Entity | 0x5c8 | level | r | m02.ll:65001 (range 성장 (level-1)*growth) · 65113 (skill2_effect: level>2) | 4 | OK |
| 21 | Entity | 0x438 | stat_buff_cached.range | r | m02.ll:65005 | 4 | OK |
| 22 | Entity | 0x470 | stat_buff_cached.radius_mult (i32) | r | m02.ll:65007 Entity::radius 인라인(entity.rs:1511) | 4 | OK |
| 23 | Entity | 0x680 | radius | r | m02.ll:65019, 65026 | 4 | OK |
| 24 | Entity | 0x4f8 | skill_effect@tag | r | champ m02.ll:65095 | 4 | OK |
| 25 | Entity | 0x4c8 | skill_effect@Some.0 | r | champ m02.ll:65101 | 4 | OK |
| 26 | Entity | 0x500 | skill2_effect@Some.0 (level>2 아니면 정적 None 상수 @anon…19) | r | champ m02.ll:65116~65117 | 4 | OK |
| 27 | Entity | 0x30 | (Option<Effect>+0x30 = Entity+0x530) skill2_effect@tag | r | m02.ll:65119~65121 — %246(+0x500 또는 정적 None)+48 로 계산돼 1328 리터럴은 본문에 없음 | 4 | OK |
| 28 | Entity | 0x68 | ty@tag (EntityType) | r | 대상 t m02.ll:65150 == 6 Serpen (entity.rs:1374 is_serpen) | 4 | OK |
| 29 | bumpalo Vec<&Entity>(sret 32B 로컬) | 0x0 | ptr | r | m02.ll:64748 | 4 | 확인불가(tcx 사전에 타입 없음) |
| 30 | bumpalo Vec<&Entity>(sret 32B 로컬) | 0x18 | len | r | m02.ll:64750 (drop 경로: +0x8 bump, +0x10 cap 64854·64862) | 4 | 확인불가(tcx 사전에 타입 없음) |

**`consts` 상수 14건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 40000000000 | 852 | 임계 | 200000² — RunAway 보정: 아군 챔피언 distance_sq > 이것이면 '근접 아님' (m02.ll:64819) | 4 |
| 1 | -10 | 852 | 산출값 | RunAway(RunAway·Recall·AroundRunAway) 인데 200k 내 hp>=내hp 아군이 없으면 -10 (m02.ll:64838 phi) | 4 |
| 2 | 5 | 885 | 태그 | JungleType::Serpen(태그 5, tcxdict --enum JungleType) — camp_pos(map, Serpen, true) (m02.ll:64684). dbg 변수명은 epic_camp_pos | 3 |
| 3 | 30000 | 886 | 계수 | Effect::range 인라인(effect.rs:26): range + 30000 + caster.stat_range + (level-1)*growth + caster.radius() (m02.ll:65035) | 4 |
| 4 | 100 | 886 | 계수 | Entity::radius(entity.rs:1515): radius*(radius_mult+100)/100 (radius_mult!=0 일 때) (m02.ll:65028, 65030) | 4 |
| 5 | -99999 | 887 | 산출값 | Trace: champ↔세르펜캠프 distance_sq >= attack_dist² 면 -99999 (65067) · Attack/Skill/Skill2 대상 None 이면 -99999 (64960, 64967, 64974) | 4 |
| 6 | 6 | 899 | 태그 | EntityType::Serpen 태그 — is_serpen (m02.ll:65152) | 4 |
| 7 | 1 | 908 | 태그 | 세르펜 대상 Attack/Skill/Skill2 는 score = max(score, 1) (llvm.smax m02.ll:65159) | 4 |
| 8 | 2 | 877 | 임계 | skill2_effect(): level > 2 게이트 (m02.ll:65115). ⚠64610 의 2 는 팀 경계검사 | 4 |
| 9 | 0 | 849 | 태그 | parameter.player.risk_epic_damage == 0 → RunAway 보정 생략(64676) · Trace 사거리 내 0(65067) · 근접 아군 존재 0(64838) | 4 |
| 10 | 12 | 896 | 태그 | Play idx-12 <u 3 ⇔ idx 12·13·14 (Attack·Skill·Skill2) — get_action target_id 추출 (m02.ll:64985~64986) | 4 |
| 11 | 3 | 896 | 센티널 | 위 범위 폭 3 (64986). ⚠64643 의 3 은 니치 시작(idx=tag-3) · 64760/64885 의 `shl 3` 은 포인터 stride 8(=8B 원소) 접힘이지 이 상수와 무관 | 4 |
| 12 | -1 | 863 | 센티널 | Option<Effect> None 니치값(i32) → unwrap_failed (64692, 65078, 65097, 65121) | 4 |
| 13 | 10 | 847 | 태그 | llvm.assume(tag != 10) (m02.ll:64641) | 4 |

**`knobs` 조정점 4건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 도주 시 근접 아군 부재 감점 | serpen_hunt.rs:852 (m02.ll:64838) | -10 | 더 음수로 하면 위험옵션(risk_epic_damage≠0) 세르펜 사냥 중 아군 없이 도주하는 행동 억제 강화 | 4 | 기존 |
| 1 | 근접 아군 거리 게이트 | serpen_hunt.rs:852 (m02.ll:64819) | 40000000000 | 200000². 올리면 더 먼 아군도 '근접'으로 인정돼 -10 이 덜 붙음 | 4 | 기존 |
| 2 | Trace 사거리 밖 배제값 | serpen_hunt.rs:887 (m02.ll:65067) | -99999 | 세르펜 캠프가 공격 사거리(range+30000+…) 밖이면 Trace 사실상 배제. 완화하면 원거리 Trace 허용 | 4 | 기존 |
| 3 | 세르펜 대상 공격 최소 점수 | serpen_hunt.rs:908 (m02.ll:65159) | 1 | 올리면 세르펜을 때리는 Attack/Skill/Skill2 가 음수 base 여도 그 값 이상 보장 | 4 | 기존 |

<details><summary>`callees` 피호출자 20건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | calculate_serpen_action_score | game_ai::plan_legacy::sub_plan::serpen_hunt::calculate_serpen_action_score | in:game_ai::plan_legacy::sub_plan::serpen_hunt | fn(&mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &std::boxed::Box<dyn [Binder { value: Trait(game_core::Action), bound_vars: [] }] + 'static, std::alloc::Global>, &game_core::Effect, &game_core::Entity) -> i64 | game-ai\src\plan_legacy\sub_plan\serpen_hunt.rs:914 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | camp_pos | game_core::MapDef::camp_pos | pub | fn(&game_core::MapDef, game_core::JungleType, bool) -> (u64, u64) | game-core\src\simulation\map_def.rs:207 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 2 | champions | game_core::AbstractGameWithCache::<'a, 'b>::champions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize, &'b/#1 bumpalo::Bump) -> bumpalo::collections::vec::Vec< &game_core::Entity> | game-core\src\simulation.rs:1896 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 3 | distance_sq | game_core::utils::distance_sq | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:6 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 4 | distance_sq | game_core::Entity::distance_sq | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2157 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 5 | get_action | game_ai::SmallActionUlt::get_action | in:game_ai | fn(&game_ai::SmallActionUlt) -> game_core::SmallAction | game-ai\src\small_action\cast.rs:286 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 16개 중 상위 3개 |
| 6 | get_action | game_ai::SmallActionPlay::get_action | pub | fn(&game_ai::SmallActionPlay) -> game_core::SmallAction | game-ai\src\small_action.rs:308 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 16개 중 상위 3개 |
| 7 | get_action | game_ai::SmallActionTrace::get_action | in:game_ai | fn(&game_ai::SmallActionTrace) -> game_core::SmallAction | game-ai\src\small_action\trace.rs:403 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 16개 중 상위 3개 |
| 8 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 9 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 10 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 11 | interaction_score | game_ai::interaction_score | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 | game-ai\src\action_score.rs:616 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 12 | is_serpen | game_core::EntityType::is_serpen | pub | fn(&game_core::EntityType) -> bool | game-core\src\simulation\entity.rs:1373 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 13 | radius | game_core::Entity::radius | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1509 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 14 | range | game_core::Effect::range | pub | fn(&game_core::Effect, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:25 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 15 | serpen_action_score | game_ai::plan_legacy::sub_plan::serpen_action_score | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 | game-ai\src\plan_legacy\sub_plan\serpen_hunt.rs:843 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 16 | skill2_effect | game_core::Entity::skill2_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1692 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 17 | target_id | game_ai::SmallActionTrace::target_id | pub | fn(&game_ai::SmallActionTrace) -> usize | game-ai\src\small_action\trace.rs:427 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 18 | target_id | game_view::view::effect::alchemist::target_id | in:game_view::view::effect::alchemist | fn(game_core::InputTarget) -> std::option::Option<usize> | game-view\src\view\effect\alchemist.rs:126 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 19 | target_id | game_view::view::projectile::crossbowman::target_id | in:game_view::view::projectile::crossbowman | fn(game_core::InputTarget) -> std::option::Option<usize> | game-view\src\view\projectile\crossbowman.rs:1878 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
</details>

⚠**미매칭 3개**: `drop  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `llvm.assume`, `llvm.smax.i64`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 2곳** (m02.ll:16679, m02.ll:20307) · **형제 0개** 

**`open` 4건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | champions() 결과에 자기 자신(champ)이 포함되는지 — 포함되면 L852 술어가 자기 자신(dist 0, hp==hp)으로 항상 true 가 되어 -10 이 사장됨. game_core 경계라 안 팜(미탐색 = _gcbc g15.ll:109887 define) | 4 |  |
| 1 | 표기 불가 | L849 risk_epic_damage==0 과 Positioning/AroundPosition 팔이 '조기 return base' 로 컴파일된 것이 소스의 `return` 인지 match 값 0 + 최적화인지 — 표기 불가(결과는 동일하게 base) | 4 |  |
| 2 | 미탐색 | camp_pos 의 bool 인자 true 의 의미(blue/red side 선택으로 추정 — _docs game_core.txt:1288 '캠프 한 기의 배치 (blue side / red side 좌표)') — 안 팜 | 5 |  |
| 3 | 표기 불가 | Effect::range 의 소스가 `+30000` 리터럴인지 상수명인지 — 표기 불가 | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

