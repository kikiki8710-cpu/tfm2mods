---

### `223` DefenseNexusSubPlan::score — 넥서스 방어 서브플랜 액션 점수: interaction_score 기저 + (v≥2·공격류 대상이 본진구조물 때리는 미니언이면 +100) + 액션별 가산(Attack/Skill/Skill2=calculate_action_score(Push) · 대상 없으면 -99999 / Around·AroundHide·LaneMinionPosition=대상이 적 미니언이고 넥서스 최근접 전방미니언이면 +5)

| 항목 | 값 |
|---|---|
| id | `defense_nexus__DefenseNexus__score` |
| 심볼 | `_RNvMs_NtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy8sub_plan13defense_nexusNtB4_19DefenseNexusSubPlan5score` |
| 소스 | `game-ai\src\plan_legacy\sub_plan\defense_nexus.rs:311` |
| IR | `m14.ll` 48981~49570행 |
| 경로·가시성 | `game_ai::plan_legacy::sub_plan::DefenseNexusSubPlan::score` · **pub** |
| 계층 | 기타 |
| exe | `e95ae0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r17` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_ai::plan_legacy::sub_plan::DefenseNexusSubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64
```

**TLS 메모 절(`sig.tls` · 정정 경로 `/specs[223]/sig/tls/<키>`)**

없음 — 본문 `@anon.*` 참조 6개(.196~.201)는 전부 panic Location · `<KEY…call_once>` fn-포인터 상수 참조 0 · LocalKey::with 0 (interaction_score/calculate_action_score/is_base_attacking_minion 내부는 콜리 계약 밖)

<details><summary>인자 8개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | self | &DefenseNexusSubPlan (24B) | IR `readonly captures(none)` · 본문에서 읽지 않음(last_gate·focus 미참조) | 4 |
| 1 | 2 | version | usize | L318 `version > 1` 게이트 1회(m14.ll:49031 icmp ugt %1,1) · interaction_score/calculate_action_score 1번 인자로 통과. reach(version=2)에서 접힘 @26 → 항상 true | 4 |
| 2 | 3 | parameter | &ScoreParameter (5384B) | IR `readonly` · 본문 직접 읽기 없음 · interaction_score 5번·calculate_action_score 5번 인자로 통과 | 4 |
| 3 | 4 | rnd | &mut StdRng (320B) | IR 속성 없음(=&mut) · 본 함수 gen_range 사이트 0 · interaction_score 2번·calculate_action_score 2번 인자로 통과만 | 4 |
| 4 | 5 | player | &PlayerState (2528B) | IR `readonly` · info.team(0x930)·info.position(0x9c0) 읽음 · is_base_attacking_minion 1번 인자 | 4 |
| 5 | 6 | data | &OperationData (24B) | IR `readonly` · cache(0x0)·context(0x8)·blackboard(0x10) 모두 읽음 | 4 |
| 6 | 7 | action | &SmallActionPlay (184B) | IR `readonly` · 태그 +0xb1 과 페이로드 +0x8(target) 만 읽음 | 4 |
| 7 | 8 | debug | &mut DebugFrameData (224B) | IR 속성 없음(=&mut) · 본 함수 직접 store 0 · interaction_score 7번·calculate_action_score 11번 인자로 통과만 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn score(&self, version, parameter, rnd, player, data, action, debug) -> i64   [defense_nexus.rs:311]
  team = player.info.team (bounds <2)                                                         [L312 · m14.ll:49000~49006]
  champ = data.cache.player_champion[team][player.info.position].unwrap()                     [L312 · 49011~49020 · None→unwrap_failed 49037]
  base = action_score::interaction_score(version, rnd, player, data, parameter, action, debug)   [L313 · 49029]
  if version > 1 {                                                                            [L318 · 49031 ugt · reach: 항상 true]
    // L319: 공격류 4종만 target 꺼냄 (tag 15..18 = Attack/Skill/Skill2/Ult)
    target_is_base_attacker = match action { Attack(a)|Skill(a)|Skill2(a)|Ult(a) => Some(a.target), _ => None }   [49042~49046, 49081~49082]
        .and_then(|id| data.cache.game.get_entity_by_id(id))        // closure#0 L320 · vtable+0x1f0 · 49084~49097
        .is_some_and(|e| old::is_base_attacking_minion(player, data, e))   // closure#1 L321 · 49106
    if target_is_base_attacker { base += 100 }                                                [L322 · 49108~49109 select]
  }
  idx = 논리 variant idx(action 태그 +0xb1; tag>2 ? tag-3 : 7)                              [L329 · 49052~49075 switch]
  extra = match action {
    Attack(12)  => match game.get_entity_by_id(a.target) {                                    [L334 · 49128~49138]
                     None => -99999,
                     Some(t) => calculate_action_score(version, rnd, player, data, parameter, &champ.attack(0x570), champ.attack_effect.as_ref().unwrap()(0x490·tag 0x4c0!=-1), champ.attack_speed_mult(), t, MinionActionType::Push(2), debug) }   [L335~336 · 49502~49513]
    Skill(13)   => None => -99999 | Some(t) => calculate_action_score(…, &champ.skill(0x580), champ.skill_effect.unwrap()(0x4c8·tag 0x4f8), champ.cooldown_reduce(false), t, Push, debug)   [L342~344 · 49143~49153, 49524~49535]
    Skill2(14)  => None => -99999 | Some(t) => { eff = if champ.level>2 { &champ.skill2_effect } else { &None }  // Entity::skill2_effect 인라인 entity.rs:1693
                     calculate_action_score(…, &champ.skill2(0x590), eff.as_ref().unwrap()(0x500·tag 0x530), champ.cooldown_reduce(false), t, Push, debug) }   [L350~352 · 49158~49168, 49545~49568]
    Around(2) | AroundHide(3) | LaneMinionPosition(10) =>                                     [L358 · 49113~49123]
      match game.get_entity_by_id(a.target) { None => 0, Some(t) =>
        if t.team != champ.team && t.is_any_type_minion() {                                   [L359~360 · 49181~49211(TeamType PartialEq) · 49196~49199(ty tag==1)]
          bb = &data.blackboard[1 - team]                    // 적 팀 블랙보드                 [L361 · 49214~49217]
          top    = bb.minion_state(Top).front_minion.and_then(|id| game.get_entity_by_id(id))     // closure#2 L362 · 49219~49238
          mid    = bb.minion_state(Mid).front_minion.and_then(…)                                  // closure#3 L364 · 49241~49261
          bottom = bb.minion_state(Bottom).front_minion.and_then(…)                               // closure#4 L366 · 49264~49284
          nexus  = data.cache.nexus[team].unwrap()                                                [L367 · 49286~49291 · None→unwrap_failed 49455]
          front_minions: bumpalo::Vec<&Entity> = Vec::from_iter_in(vec![top,mid,bottom].into_iter().filter_map(|m| m), data.context.pool)   // closure#5 L370 · 49312~49352 (aux m09 try_fold · m14 call_mut 항등)
          nearest = front_minions.iter().min_by_key(|m| dist_sq(m.pos, nexus.pos))   // closure#6 L372 · dist_sq = |dx|²+|dy|² (49417~49451 첫 원소 · aux m12.ll:30107~30247 나머지 fold · 동점이면 앞 원소 유지 30227~30229)
          match nearest { None => 0, Some(n) => if n.id == t.id { 5 } else { 0 } }              [L373~374 · 49467~49478]
        } else { 0 }                                                                            [L359 거짓 → 49171 phi 0]
      }
    _ (RunAway·Recall·AroundRegion·AroundRunAway·Positioning·AroundPosition·AroundPositionBush·AroundBush·Trace·Ult·Stop) => 0   [L329 switch → %97 phi 0]
  }
  return base + extra                                                                          [L329/394 · 49172~49173]

주의: ①Ult(15) 는 L319 의 +100 가산 대상이지만 L329 매치에서는 0 가산(calculate_action_score 호출 없음). ②공격류 3종은 target 부재 시 -99999(사실상 배제)지만 Around 계열은 0. ③Skill2 가지는 champ.level<=2 이면 unwrap 패닉(49558) — 도달 조건은 콜러(후보 생성)에 있음. ④gen_range 사이트 0 · self 읽기 0 · debug 직접 store 0. ⑤front_minions 는 적 팀 블랙보드(blackboard[1-team])의 라인별 front_minion id 3개를 엔티티로 해석한 것 — '넥서스에 가장 가까운 적 전방 미니언' 을 정리 대상으로 선호.
```

**`mem` 메모리 접근 34건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | m14.ll:49000~49002 · player_champion 1차 인덱스 · `<2` 바운드체크(panic_bounds_check 49006) · nexus[team] 인덱스(49287) · blackboard[1-team] 인덱스(49214) | 4 | OK |
| 1 | PlayerState | 0x9c0 | info.position@tag | r | i32 zext · player_champion 2차 인덱스 (m14.ll:49011~49013, player.rs:581 인라인) | 4 | OK |
| 2 | OperationData | 0x0 | cache | r | m14.ll:49014 | 4 | OK |
| 3 | OperationData | 0x8 | context | r | m14.ll:49349~49350 · context+0x0 pool(&Bump) 을 bumpalo Vec 할당자로 (49351) | 4 | OK |
| 4 | OperationData | 0x10 | blackboard (&[Blackboard;2]) | r | m14.ll:49215~49217 · 원소 744B stride · 인덱스 `1 - team`(적 팀 블랙보드 = 적 미니언 상태) | 4 | OK |
| 5 | GameContext | 0x0 | pool | r | m14.ll:49351 · from_iter_in 의 &Bump | 4 | OK |
| 6 | AbstractGameWithCache | 0x0 | game (&dyn AbstractGame data ptr) | r | m14.ll:49084 / 49116 / 49131 / 49146 / 49161 | 4 | OK |
| 7 | AbstractGameWithCache | 0x8 | game vtable ptr | r | vtable+0x1f0(496) = AbstractGame::get_entity_by_id(divtable 실측) 간접호출 6사이트 (49090~49093 · 49119~49121 · 49134~49136 · 49149~49151 · 49164~49166 · 49233/49256/49279) | 3 | OK |
| 8 | AbstractGameWithCache | 0x1e0 | player_champion[team][pos] | r | [2][5] Option<&Entity>(니치 null=None) → unwrap_failed(49037) (m14.ll:49015~49020) | 4 | OK |
| 9 | AbstractGameWithCache | 0x170 | nexus[team] | r | Option<&Entity> null=None → unwrap_failed(49455) (m14.ll:49286~49291, L367) | 4 | OK |
| 10 | SmallActionPlay | 0xb1 | 태그(니치 1B) | r | m14.ll:49032~49033 · L319 `tag-15 <u 4`(=Attack15/Skill16/Skill2 17/Ult18) · L329 논리 idx = tag>2 ? tag-3 : 7(AroundPosition) · assume(tag!=10) | 4 | OK |
| 11 | SmallActionPlay | 0x8 | 페이로드.target (usize · Attack/Skill/Skill2/Ult/Around/AroundHide/LaneMinionPosition 공통 +0x8) | r | m14.ll:49081~49082(L320) · 49113~49114(L358) · 49128~49129(L334) · 49143~49144(L342) · 49158~49159(L350) | 4 | OK |
| 12 | Blackboard | 0x0 | top_minion_state.front_minion@tag | r | m14.ll:49219~49224 · Option<usize> tag(i64) trunc→i1 · L362 (minion_state(Top) 인라인 blackboard.rs:381) | 4 | OK |
| 13 | Blackboard | 0x8 | top_minion_state.front_minion@Some.0 | r | m14.ll:49228~49233 · get_entity_by_id(id) | 4 | OK |
| 14 | Blackboard | 0x28 | mid_minion_state.front_minion@tag | r | m14.ll:49241~49247 · L364 | 4 | OK |
| 15 | Blackboard | 0x30 | mid_minion_state.front_minion@Some.0 | r | m14.ll:49251~49256 | 4 | OK |
| 16 | Blackboard | 0x50 | bottom_minion_state.front_minion@tag | r | m14.ll:49264~49270 · L366 (blackboard.rs:382) | 4 | OK |
| 17 | Blackboard | 0x58 | bottom_minion_state.front_minion@Some.0 | r | m14.ll:49274~49279 | 4 | OK |
| 18 | Entity | 0x0 | team@tag (TeamType: 0 Player/1 Neutral) | r | t 와 champ 양쪽 (m14.ll:49181~49187, L359 `t.team != champ.team` derive PartialEq entity.rs:1127) | 4 | OK |
| 19 | Entity | 0x8 | team@Player.0 (팀 번호) | r | 태그가 둘 다 0(Player)일 때만 비교 (m14.ll:49208~49210) | 4 | OK |
| 20 | Entity | 0x68 | ty@tag (EntityType) | r | `== 1`(Minion) = Entity::is_any_type_minion 인라인 entity.rs:1261 (m14.ll:49196~49198, L360) | 4 | OK |
| 21 | Entity | 0x490 | attack_effect@Some.0 (Effect) | r | champ · calculate_action_score 7번 인자 (m14.ll:49508, L335) — Attack 가지 | 4 | OK |
| 22 | Entity | 0x4c0 | attack_effect@tag (casting 니치 i32) | r | -1 = None → unwrap_failed(49518) (m14.ll:49502~49505) | 4 | OK |
| 23 | Entity | 0x4c8 | skill_effect@Some.0 (Effect) | r | m14.ll:49530 (L343) — Skill 가지 | 4 | OK |
| 24 | Entity | 0x4f8 | skill_effect@tag (casting 니치 i32) | r | -1 → unwrap_failed(49540) (m14.ll:49524~49527) | 4 | OK |
| 25 | Entity | 0x500 | skill2_effect@Some.0 (Effect) | r | m14.ll:49562 (L351, entity.rs:1694) — Skill2 가지(level>2) | 4 | OK |
| 26 | Entity | 0x530 | skill2_effect@tag (casting 니치 i32) | r | -1 → unwrap_failed(49558) (m14.ll:49552~49555) | 4 | OK |
| 27 | Entity | 0x570 | attack (Box<dyn Action>) | r | calculate_action_score 6번 인자 &Box (m14.ll:49511, L336) | 4 | OK |
| 28 | Entity | 0x580 | skill (Box<dyn Action>) | r | m14.ll:49533 (L344) | 4 | OK |
| 29 | Entity | 0x590 | skill2 (Box<dyn Action>) | r | m14.ll:49566 (L352) | 4 | OK |
| 30 | Entity | 0x5c0 | id | r | nearest_front_minion.id == t.id (m14.ll:49473~49477, L374) | 4 | OK |
| 31 | Entity | 0x5c8 | level | r | Entity::skill2_effect 인라인 `level > 2` (m14.ll:49545~49548, entity.rs:1693, L351) | 4 | OK |
| 32 | Entity | 0x660 | x | r | 전방미니언·넥서스 거리² (m14.ll:49420~49421, 49428~49429 · aux m12.ll:30183~30184, 30191~30192) | 4 | OK |
| 33 | Entity | 0x668 | y | r | m14.ll:49424~49425, 49432~49433 · aux m12.ll:30187~30188, 30195~30196 | 4 | OK |

**`consts` 상수 20건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 2 | 312 | 길이 | player_champion 1차 길이 바운드체크 team<2 (m14.ll:49002) | 4 |
| 1 | 1 | 318 | 임계 | `version > 1` — v2 이상에서만 본진구조물-타격 미니언 +100 가산 (m14.ll:49031 icmp ugt) · reach(version=2) 접힘 true | 4 |
| 2 | 10 | 319 | 센티널 | SmallActionPlay 니치 태그 10(암묵 AroundPosition 자리)은 안 나옴 → llvm.assume (m14.ll:49042, 49052) | 4 |
| 3 | -15 | 319 | 태그 | `tag - 15` (m14.ll:49044) — 태그 15..18 = Attack/Skill/Skill2/Ult 를 한 번에 판별 | 4 |
| 4 | 4 | 319 | 임계 | `(tag-15) <u 4` (m14.ll:49045) → 공격류 4종이면 target 을 꺼냄 | 4 |
| 5 | 100 | 322 | 계수 | target 이 is_base_attacking_minion 이면 base += 100 (m14.ll:49108~49109 add/select) | 4 |
| 6 | -3 | 329 | 센티널 | 니치 태그 → 논리 idx 변환 `tag-3` (niche_start=3) (m14.ll:49054) | 4 |
| 7 | 2 | 329 | 임계 | `tag >u 2` 이면 tag-3 아니면 7 (m14.ll:49055) | 4 |
| 8 | 7 | 329 | 태그 | 태그 ≤2 → 논리 idx 7 = AroundPosition(untagged) (m14.ll:49056 select) | 4 |
| 9 | -99999 | 334 | 산출값 | Attack(L334)/Skill(L342)/Skill2(L350) 에서 get_entity_by_id(target) 가 None 이면 가산값 -99999 (m14.ll:49171 phi from %67/%77/%87) — 사실상 후보 배제 | 4 |
| 10 | -1 | 335 | 센티널 | Option<Effect> 니치 None 판별값(casting 태그 i32 = -1) → unwrap 패닉 (m14.ll:49504 / 49526 / 49554) | 4 |
| 11 | 2 | 336 | 태그 | MinionActionType::Push(태그 2, utils.rs:768) 을 calculate_action_score 10번 인자로 (m14.ll:49513, 49535, 49568 `i8 2`) | 4 |
| 12 | 2 | 351 | 임계 | Entity::skill2_effect 인라인 `level > 2` — 2레벨 이하면 skill2_effect 를 정적 None 으로 봐 unwrap 패닉 (m14.ll:49547, entity.rs:1693) | 4 |
| 13 | 0 | 359 | 태그 | TeamType 태그 0 = Player — 양쪽 Player 일 때만 payload(팀 번호) 비교 (m14.ll:49191) | 4 |
| 14 | 1 | 360 | 태그 | EntityType 태그 1 = Minion — `t.is_any_type_minion()` (m14.ll:49198) | 4 |
| 15 | 1 | 361 | 임계 | `1 - team` = 적 팀 인덱스로 blackboard 선택 (m14.ll:49214) | 4 |
| 16 | 5 | 374 | 태그 | Around/AroundHide/LaneMinionPosition 의 target 이 넥서스 최근접 전방 적 미니언이면 +5 (m14.ll:49478 select) | 4 |
| 17 | 496 | 320 | 미상 | dyn AbstractGame vtable 슬롯 0x1f0 = get_entity_by_id (divtable 실측) (m14.ll:49090, 49119, 49134, 49149, 49164) | 3 |
| 18 | 24 | 370 | 미상 | `vec![top,mid,bottom]` Box<[Option<&Entity>;3]> 힙 할당 24B (m14.ll:49313) — 판정 상수 아님 | 4 |
| 19 | 3 | 370 | 태그 | vec! 원소 수 3 (len/cap, m14.ll:49346) — 판정 상수 아님. ⚠본문의 `shl %167, 3`(49368) 은 slice len×8(ptr 크기) 바이트 환산이지 이 3과 무관 | 4 |

**`knobs` 조정점 5건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 본진구조물-타격 미니언 공격 가산 | sub_plan/defense_nexus.rs:322 | 100 | 올리면 v≥2 에서 넥서스·쌍둥이를 때리는 적 미니언을 향한 Attack/Skill/Skill2/Ult 후보가 다른 후보를 압도한다 · 내리면 interaction_score 기저 순위에 가까워진다 | 4 | 기존 |
| 1 | 공격류 대상 부재 패널티 | sub_plan/defense_nexus.rs:334/342/350 | -99999 | Attack/Skill/Skill2 의 target 엔티티가 사라졌을 때 후보를 사실상 배제 — 값을 키우면(덜 음수) 죽은 대상 후보가 살아남을 수 있다 | 4 | 기존 |
| 2 | 넥서스 최근접 전방미니언 접근 가산 | sub_plan/defense_nexus.rs:374 | 5 | 올리면 Around/AroundHide/LaneMinionPosition 이 넥서스에 가장 가까운 적 전방 미니언을 대상으로 잡을 때 더 선호된다(현재는 기저 대비 미세 가산) | 4 | 기존 |
| 3 | 버전 게이트 | sub_plan/defense_nexus.rs:318 | 1 | `version > 1` — v1 이하에서는 +100 가산 자체가 없다(reach version=2 에서 접힘) | 4 | 기존 |
| 4 | 미니언 액션 타입 | sub_plan/defense_nexus.rs:336/344/352 | 2 | calculate_action_score 에 Push(2) 고정 전달 — Pull(0)/Normal(1) 로 바꾸면 calculate_action_score 내부 미니언 처리 분기가 바뀐다(내부는 콜리 계약 밖) | 4 | 기존 |

<details><summary>`callees` 피호출자 25건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | attack | game_ai::attack | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_core::Entity) -> std::option::Option<game_core::Input> | game-ai\src\abstract_input.rs:148 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 82개 중 상위 3개 |
| 1 | attack | game_core::Entity::attack | pub | fn(&game_core::Entity) -> &std::boxed::Box<dyn [Binder { value: Trait(game_core::Action), bound_vars: [] }] + 'static, std::alloc::Global> | game-core\src\simulation\entity.rs:1493 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 82개 중 상위 3개 |
| 2 | attack | game_core::EntityInfo::attack | pub | fn(&Self/#0) -> std::boxed::Box<dyn [Binder { value: Trait(game_core::Action), bound_vars: [] }] + 'static, std::alloc::Global> | game-core\src\setting.rs:1153 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 82개 중 상위 3개 |
| 3 | attack_speed_mult | game_core::Entity::attack_speed_mult | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:2433 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | calculate_action_score | game_ai::calculate_action_score | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &std::boxed::Box<dyn [Binder { value: Trait(game_core::Action), bound_vars: [] }] + 'static, std::alloc::Global>, &game_core::Effect, usize, &game_core::Entity, game_ai::MinionActionType, &mut game_core::DebugFrameData) -> i64 | game-ai\src\action_score.rs:8 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 5 | cooldown_reduce | game_core::Entity::cooldown_reduce | pub | fn(&game_core::Entity, bool) -> usize | game-core\src\simulation\entity.rs:2437 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 6 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 7 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 8 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 9 | interaction_score | game_ai::interaction_score | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 | game-ai\src\action_score.rs:616 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 10 | is_any_type_minion | game_core::EntityType::is_any_type_minion | pub | fn(&game_core::EntityType) -> bool | game-core\src\simulation\entity.rs:1260 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 11 | is_base_attacking_minion | game_ai::plan_legacy::old::is_base_attacking_minion | pub | fn(&game_core::PlayerState, &game_core::OperationData, &game_core::Entity) -> bool | game-ai\src\plan_legacy\old\defense_nexus.rs:259 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 12 | minion_state | game_core::Blackboard::minion_state | pub | fn(&game_core::Blackboard, game_core::LineType) -> &game_core::BrainMinionParameter | game-core\src\simulation\game\blackboard.rs:378 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 13 | score | game_ai::plan_legacy::sub_plan::SubPlan::score | pub | fn(&mut game_ai::plan_legacy::sub_plan::SubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 | game-ai\src\plan_legacy\sub_plan\mod.rs:164 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 26개 중 상위 3개 |
| 14 | score | game_ai::plan_legacy::sub_plan::HideSubPlan::score | pub | fn(&game_ai::plan_legacy::sub_plan::HideSubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 | game-ai\src\plan_legacy\sub_plan\hide.rs:238 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 26개 중 상위 3개 |
| 15 | score | game_ai::plan_legacy::sub_plan::StealSubPlan::score | pub | fn(&game_ai::plan_legacy::sub_plan::StealSubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 | game-ai\src\plan_legacy\sub_plan\steal.rs:136 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 26개 중 상위 3개 |
| 16 | skill | game_ai::skill | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_core::Entity) -> std::option::Option<game_core::Input> | game-ai\src\abstract_input.rs:199 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 65개 중 상위 3개 |
| 17 | skill | game_core::Entity::skill | pub | fn(&game_core::Entity) -> &std::boxed::Box<dyn [Binder { value: Trait(game_core::Action), bound_vars: [] }] + 'static, std::alloc::Global> | game-core\src\simulation\entity.rs:1664 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 65개 중 상위 3개 |
| 18 | skill | game_core::ChampionInfo::skill | pub | fn(&Self/#0) -> std::boxed::Box<dyn [Binder { value: Trait(game_core::Action), bound_vars: [] }] + 'static, std::alloc::Global> | game-core\src\setting.rs:1085 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 65개 중 상위 3개 |
| 19 | skill2 | game_ai::skill2 | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_core::Entity) -> std::option::Option<game_core::Input> | game-ai\src\abstract_input.rs:239 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 65개 중 상위 3개 |
| 20 | skill2 | game_core::Entity::skill2 | pub | fn(&game_core::Entity) -> &std::boxed::Box<dyn [Binder { value: Trait(game_core::Action), bound_vars: [] }] + 'static, std::alloc::Global> | game-core\src\simulation\entity.rs:1668 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 65개 중 상위 3개 |
| 21 | skill2 | game_core::ChampionInfo::skill2 | pub | fn(&Self/#0) -> std::boxed::Box<dyn [Binder { value: Trait(game_core::Action), bound_vars: [] }] + 'static, std::alloc::Global> | game-core\src\setting.rs:1086 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 65개 중 상위 3개 |
| 22 | team | game_view::ClientData::team | pub | fn(&game_view::ClientData, usize) -> std::option::Option<std::cell::Ref< game_core::Team>> | game-view\src\logic\client\data.rs:1651 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 23 | team | game_view::ClientDatabase::team | pub | fn(&game_view::ClientDatabase, usize) -> std::option::Option<&game_core::Team> | game-view\src\logic\client\data.rs:968 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 24 | team | game_core::PlayerAiContext::<'a, 'b, 'r>::team | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:583 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
</details>

⚠**미매칭 5개**: `__rust_alloc`, `__rust_no_alloc_shim_is_unstable_v2`, `dist_sq`, `drop  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `handle_alloc_error`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m12.ll:37316) · **형제 11개** (DefenseNexusSubPlan)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::plan_legacy::sub_plan::DefenseNexusSubPlan as std::clone::Clone>::clone | pub | game-ai\src\plan_legacy\sub_plan\defense_nexus.rs:6 | True | fn(&game_ai::plan_legacy::sub_plan::DefenseNexusSubPlan) -> game_ai::plan_legacy::sub_plan::DefenseNexusSubPlan |
| 1 | <game_ai::plan_legacy::sub_plan::DefenseNexusSubPlan as std::default::Default>::default | pub | game-ai\src\plan_legacy\sub_plan\defense_nexus.rs:6 | True | fn() -> game_ai::plan_legacy::sub_plan::DefenseNexusSubPlan |
| 2 | <game_ai::plan_legacy::sub_plan::DefenseNexusSubPlan as std::fmt::Debug>::fmt | pub | game-ai\src\plan_legacy\sub_plan\defense_nexus.rs:6 | True | fn(&game_ai::plan_legacy::sub_plan::DefenseNexusSubPlan, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 3 | game_ai::plan_legacy::sub_plan::DefenseNexusSubPlan::merge | pub | game-ai\src\plan_legacy\sub_plan\defense_nexus.rs:20 | True | fn(&mut game_ai::plan_legacy::sub_plan::DefenseNexusSubPlan, game_ai::plan_legacy::sub_plan::DefenseNexusSubPlan) |
| 4 | game_ai::plan_legacy::sub_plan::DefenseNexusSubPlan::commit_chase | in:game_ai::plan_legacy::sub_plan::defense_nexus | game-ai\src\plan_legacy\sub_plan\defense_nexus.rs:33 | False | fn(&mut game_ai::plan_legacy::sub_plan::DefenseNexusSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 5 | game_ai::plan_legacy::sub_plan::DefenseNexusSubPlan::base_positioning | in:game_ai::plan_legacy::sub_plan::defense_nexus | game-ai\src\plan_legacy\sub_plan\defense_nexus.rs:97 | False | fn(&mut game_ai::plan_legacy::sub_plan::DefenseNexusSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 6 | game_ai::plan_legacy::sub_plan::DefenseNexusSubPlan::attack_minion_action | in:game_ai::plan_legacy::sub_plan::defense_nexus | game-ai\src\plan_legacy\sub_plan\defense_nexus.rs:136 | False | fn(&mut game_ai::plan_legacy::sub_plan::DefenseNexusSubPlan, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 7 | game_ai::plan_legacy::sub_plan::DefenseNexusSubPlan::attack_tower_action | in:game_ai::plan_legacy::sub_plan::defense_nexus | game-ai\src\plan_legacy\sub_plan\defense_nexus.rs:198 | False | fn(&mut game_ai::plan_legacy::sub_plan::DefenseNexusSubPlan, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> std::option::Option<game_ai::SmallActionPlay> |
| 8 | game_ai::plan_legacy::sub_plan::DefenseNexusSubPlan::calculate_score_parameter_value | pub | game-ai\src\plan_legacy\sub_plan\defense_nexus.rs:229 | True | fn(&game_ai::plan_legacy::sub_plan::DefenseNexusSubPlan, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::ScoreParameter) |
| 9 | game_ai::plan_legacy::sub_plan::DefenseNexusSubPlan::action_candidates | pub | game-ai\src\plan_legacy\sub_plan\defense_nexus.rs:245 | False | fn(&mut game_ai::plan_legacy::sub_plan::DefenseNexusSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 10 | game_ai::plan_legacy::sub_plan::DefenseNexusSubPlan::score | pub | game-ai\src\plan_legacy\sub_plan\defense_nexus.rs:311 | False | fn(&game_ai::plan_legacy::sub_plan::DefenseNexusSubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 |

**`open` 5건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | interaction_score(version,rnd,player,data,parameter,action,debug)->i64 · calculate_action_score(version,rnd,player,data,parameter,&Box<dyn Action>,&Effect,usize,&Entity,MinionActionType,debug)->i64 · is_base_attacking_minion(player,data,&Entity)->bool 내부 — 여기서는 계약만(정본 r13~r15 / is_base_attacking_minion 은 O(1) 판정이라는 개발자 주석만 확인) | 3 |  |
| 1 | 미탐색 | Entity::attack_speed_mult(&self)->usize · Entity::cooldown_reduce(&self,bool)->usize 의 내부(game_core) — 8번 인자 의미(공격속도 배율 / 쿨감 배율)는 이름·시그니처만 | 4 |  |
| 2 | 표기 불가 | L319 소스 표기가 `matches!`+`target()` 헬퍼인지 `match … => Some(a.target)` 인지 — 표기 불가(외연 동일 · tag-15<4 로 접힘) | 4 |  |
| 3 | 표기 불가 | L372 min_by_key 의 키가 `dist_sq(m,nexus)` 인지 `nexus.dist_sq(m)` 인지 — 표기 불가(\|dx\|²+\|dy\|² 대칭) | 4 |  |
| 4 | 미탐색 | bumpalo Vec::from_iter_in 본체(m01.ll:27320~27427) 는 std 플럼빙이라 aux 에서 제외 — 판정 상수 없음 | 4 |  |

<details><summary>`closed` 1건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

| # | 물음 | 닫은 근거 |
|---|---|---|
| 0 | closure#5(L370) 가 `filter_map(\|m\| m)` 인지 `flatten()` 인지 — IR 은 FilterMap<IntoIter<Option<&Entity>>, closure#5> 로 확정(항등 call_mut m14.ll:60870~60874) → `filter_map(\|m\| m)` 이 정확 · `.flatten()` 은 아님 | 본문에 해소 표기가 있다 |
</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

