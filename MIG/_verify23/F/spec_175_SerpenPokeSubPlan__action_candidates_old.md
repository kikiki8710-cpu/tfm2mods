---

### `175` SerpenPokeSubPlan::action_candidates_old — 세르펜 견제 서브플랜의 행동 후보 목록 생성 — 논타겟 회피/궤적이면 도주 단일 후보, 아니면 세르펜 상태별(부재/비가시·원거리/치명사거리) 이동 후보 1개 + 근접 적 도주 + 전투/소환수 후보를 덧붙인다. EpicPokeSubPlan 판과 구조 동일, 차이 = region 게이트 없음·150000 거리 게이트·region 2·치명 시 Around(serpen)

| 항목 | 값 |
|---|---|
| id | `SerpenPokeSubPlan__action_candidates_old` |
| 심볼 | `_RNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy8sub_plan11serpen_pokeNtB2_17SerpenPokeSubPlan21action_candidates_old` |
| 소스 | `game-ai\src\plan_legacy\sub_plan\serpen_poke.rs:418` |
| IR | `m14.ll` 16900~18615행 |
| 경로·가시성 | `game_ai::plan_legacy::sub_plan::SerpenPokeSubPlan::action_candidates_old` · **pub** |
| 계층 | 기타 |
| exe | `e80070` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r14` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&mut game_ai::plan_legacy::sub_plan::SerpenPokeSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay>
```

<details><summary>인자 7개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | bumpalo::collections::vec::Vec<SmallActionPlay> (32B, sret align 8) | 0x0 ptr · 0x8 bump · 0x10 cap · 0x18 len — 32B 전부 live(L16948·16950 store, L16954 memset 16B, 마지막 memcpy 32B L18551/L18612). | 4 |
| 1 | 1 | self | &mut SerpenPokeSubPlan (ZST 0B) | IR `readnone captures(none)` — 읽기·쓰기 0. writes 없음. | 4 |
| 2 | 2 | version | usize | 본문 분기 0. 콜리(nontarget_windup_perceived·position_score_at_position·Around/AroundRegion::new·battle_action/battle_ally_action)에 전달만. | 4 |
| 3 | 3 | rnd | &mut StdRng (320B align 16) | `noalias align 16 dereferenceable(320)`(readonly 없음 = &mut). ★본문 store 0. 전달처 4곳(Around::new·AroundRegion::new·battle_action·battle_ally_action) 모두 `_rnd readnone`(m08.ll:98411/92385, m15.ll:23700/25950) ⟹ 이 경로에서 사실상 불변. | 4 |
| 4 | 4 | player | &PlayerState (2528B) | readonly. info.team(0x930)·info.position(0x9c0). | 4 |
| 5 | 5 | data | &OperationData (24B) | readonly. cache(0x0)·context(0x8). | 4 |
| 6 | 6 | parameter | &ScoreParameter (5384B) | readonly. +0x9f0 positioning_score 주소만 전달. | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn action_candidates_old(&mut self, version, rnd: &mut StdRng, player, data, parameter) -> Vec<SmallActionPlay>  // serpen_poke.rs:418
  let bump = data.context.pool; let mut res = Vec::new_in(bump);   // L419
  let champ = data.cache.player_champion[player.info.team][player.info.position as usize].unwrap();   // L422
  let enemy_team = 1 - player.info.team;                          // L425
  let has_non_target_action_range = data.cache.iter_champions(enemy_team).any(|c|   // L425~431 (Epic L433~439 와 동일)
        nontarget_windup_perceived(version, player, data, c) && c.ty@tag == 13 /*Champion*/   // L426
     && { let eff = match c.action_state@tag { 4 => c.skill_effect.as_ref().unwrap(),   // L427
                                              5 => c.skill2_effect().as_ref().unwrap(),  // L429 (level>2 ? &skill2_effect : &NONE)
                                              6 => c.ult_effect().as_ref().unwrap(),     // L431 (level>4 ? &ult_effect : &NONE)
                                              _ => return false };
          matches!(eff.casting@tag, 1 | 2) && eff.is_in_range(c, champ) });
  let ps = position_score_at_position(version, player, data, &parameter.positioning_score, champ.x, champ.y, 11 /*Objective*/);   // L438~439
  if ps.on_trajectory || (has_non_target_action_range || ps.on_periodic_trajectory) {   // L440 (줄 안 순서 표기 불가)
      res.push(RunAway(SmallActionRunAway::new_with_skill(data, player, 5, true)));   // L442 (태그 3)
      return res;                                                 // L443
  }
  let mut warn = false;                                           // phi %290
  // L450: Moba 아니면 as_moba().unwrap() 패닉. ★Epic 판의 `region = map.regions[..]` 읽기가 여기엔 없다.
  let serpen = data.cache.game.get_game_mode().as_moba().unwrap().jungle_runner.serpen.live_list.first()
               .and_then(|id| data.cache.game.get_entity_by_id(*id));
  match serpen {
    None => res.push(AroundRegion(SmallActionAroundRegion::new(version, rnd, data, player, 2 /*region*/, 5))),   // L470 (태그 7)
    Some(s) if s.ty@tag != 6 /*Serpen*/ => res.push(AroundRegion(…::new(version, rnd, data, player, 2, 5))),   // L467
    Some(s) if !s.is_visible_from(champ) || dist_sq(champ, s) > 22500000000 /*150000²*/ =>   // L452 (is_visible_from: champ.team 이 Player(t) 이고 s.visible_state[t]@tag != 0 이면 안 보임; Neutral 이면 보임 취급. ★Epic 판의 `region != 7` 게이트 대신 거리 게이트)
        res.push(Around(SmallActionAround::new(version, rnd, data, player, s.id, 5))),   // L453 (태그 5)
    Some(s) => {
        let eff = s.attack_effect.as_ref().unwrap();              // L455
        let range = eff.range(champ) /*eff.range+20000+champ.stat_buff_cached.range+(champ.level-1)*eff.growth_range*/ + champ.radius() + s.radius();   // L455
        let dmg = eff.expected_damage_target(data.context, s as &dyn AbstractEntity, champ);   // L456
        if dmg*2 < champ.hp || range*range < dist_sq(champ, s) {   // L458 (dist_sq 는 L452 에서 계산한 dx/dy 재사용)
            res.push(Around(SmallActionAround::new(version, rnd, data, player, s.id, 5)));   // L462 ★Epic 판은 여기서 AroundRegion(7)
        } else { warn = true; res.push(RunAway(SmallActionRunAway::new(data, player, 5))); }   // L460
    }
  }
  if data.cache.game.is_visible(enemy_team, champ.id) {           // L473
      let has_near_enemy = data.cache.iter_champions(enemy_team).any(|c| c.team != champ.team && dist_sq(c, champ) < 22500000001);   // L475 (5칸 언롤·champ.team 태그별 2사본)
      if has_near_enemy { res.push(RunAway(SmallActionRunAway::new(data, player, 5))); }   // L476~477
      if !warn { res.extend(battle_action(version, rnd, player, data, 5)); }        // L480~481
      else     { res.extend(battle_ally_action(version, rnd, player, data, 5)); }   // L483
  }
  res.extend(attack_summon_action(player, data));                 // L486
  res                                                             // L488~489
```

**`mem` 메모리 접근 55건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | `< 2` 바운즈 후 인덱스 (L16953~16961) | 4 | OK |  |
| 1 | PlayerState | 0x9c0 | info.position | r | Position 태그(i32 0..4) → player_champion 2차 인덱스 (L16973~16981) | 4 | OK |  |
| 2 | OperationData | 0x0 | cache | r | L16974 | 4 | OK |  |
| 3 | OperationData | 0x8 | context | r | L16944 | 4 | OK |  |
| 4 | GameContext | 0x0 | pool | r | &Bump — 결과 Vec 할당자 (L16946) | 4 | OK |  |
| 5 | AbstractGameWithCache | 0x0 | game.data_ptr | r | L17212 | 4 | OK |  |
| 6 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r | L17213~17214 | 4 | OK |  |
| 7 | AbstractGameWithCache | 0x1e0 | player_champion | r | [[Option<&Entity>;5];2] — [my][pos]=champ(unwrap) · [enemy][0..5] 순회 2회 (L16979~16985) | 4 | OK |  |
| 8 | vtable(AbstractGame) | 0x40 | get_game_mode | r | {i64 tag, ptr} = GameMode, tag 0=Moba (L17215~17224) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 9 | vtable(AbstractGame) | 0x1f0 | get_entity_by_id | r | (self, id) → Option<&Entity> (L17254~17258) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 10 | vtable(AbstractGame) | 0xf8 | is_visible | r | (self, enemy_team, champ.id) → bool (L17592~17594) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 11 | MobaMode | 0x1d8 | jungle_runner.serpen.live_list.len | r | 0 이면 세르펜 없음 (L17236~17238) | 4 | OK |  |
| 12 | MobaMode | 0x1d0 | jungle_runner.serpen.live_list.buf.ptr | r | first() 의 usize id → get_entity_by_id (L17246~17258) | 4 | OK |  |
| 13 | ScoreParameter | 0x9f0 | positioning_score | r | 주소만 position_score_at_position 에 전달 (L17174) | 4 | OK |  |
| 14 | PositioningScore(sret 56B, 지역 %25) | 0x30 | on_trajectory | r | L17191~17193 | 4 | OK |  |
| 15 | PositioningScore(sret 56B, 지역 %25) | 0x31 | on_periodic_trajectory | r | L17194~17196 (dbg 이름 on_trajectory 가 붙음) | 4 | OK |  |
| 16 | Entity(적 챔프 c, 클로저 L426~431) | 0x68 | ty@tag | r | == 13 Champion (L17061~17063) | 4 | OK |  |
| 17 | Entity(적 챔프 c) | 0x70 | ty@Champion.0.action_state@tag | r | 4 Skill/5 Skill2/6 Ult 만 (L17088~17093) | 4 | OK |  |
| 18 | Entity(적 챔프 c) | 0x4f8 | skill_effect@tag(casting 니치) | r | -1 None→패닉 · 1,2 통과 · 그 외 false (L17098~17103) | 4 | OK |  |
| 19 | Entity(적 챔프 c) | 0x4c8 | skill_effect | r | &Effect → is_in_range self (L17115) | 4 | OK |  |
| 20 | Entity(적 챔프 c) | 0x5c8 | level | r | skill2_effect(): level>2 / ult_effect(): level>4 아니면 정적 None (L17121~17125·17144~17148) | 4 | OK |  |
| 21 | Entity(적 챔프 c) | 0x500 | skill2_effect | r | L17124 | 4 | OK |  |
| 22 | Entity(적 챔프 c) | 0x538 | ult_effect | r | L17147 | 4 | OK |  |
| 23 | Effect(skill2/ult 선택 결과) | 0x30 | casting@tag / Option 니치 | r | L17127~17131 · 17150~17154 | 4 | OK |  |
| 24 | Entity(champ %48) | 0x660 | x | r | L17175 — position_score 좌표·거리 계산 | 4 | OK |  |
| 25 | Entity(champ %48) | 0x668 | y | r | L17181 | 4 | OK |  |
| 26 | Entity(champ %48) | 0x0 | team@tag | r | TeamType 0=Player/1=Neutral — is_visible_from(L17284) 및 L475 클로저의 team != (L17722) | 4 | OK |  |
| 27 | Entity(champ %48) | 0x8 | team@Player.0 | r | L17295~17297 · 17724 | 4 | OK |  |
| 28 | Entity(champ %48) | 0x5c8 | level | r | Effect::range(caster=champ) (L17366) | 4 | OK |  |
| 29 | Entity(champ %48) | 0x438 | stat_buff_cached.range | r | L17370 | 4 | OK |  |
| 30 | Entity(champ %48) | 0x470 | stat_buff_cached.radius_mult | r | Entity::radius() (L17372) | 4 | OK |  |
| 31 | Entity(champ %48) | 0x680 | radius | r | L17384·17391 | 4 | OK |  |
| 32 | Entity(champ %48) | 0x670 | hp | r | dmg*2 < hp (L17437~17439) | 4 | OK |  |
| 33 | Entity(champ %48) | 0x5c0 | id | r | is_visible(enemy_team, champ.id) (L17590) | 4 | OK |  |
| 34 | Entity(serpen %142) | 0x68 | ty@tag | r | == 6 Serpen (L17277~17279) | 4 | OK |  |
| 35 | Entity(serpen %142) | 0x38 | visible_state[team] | r | [VisibleState;2] 24B stride, 0=Visible (L17304~17307) | 4 | OK |  |
| 36 | Entity(serpen %142) | 0x660 | x | r | champ↔serpen 거리(L452 게이트·L458 사거리 비교 공용, L17323) | 4 | OK |  |
| 37 | Entity(serpen %142) | 0x668 | y | r | L17329 | 4 | OK |  |
| 38 | Entity(serpen %142) | 0x5c0 | id | r | Around::new target (L17317·17459) | 4 | OK |  |
| 39 | Entity(serpen %142) | 0x490 | attack_effect | r | as_ref().unwrap() → &Effect (L17353) | 4 | OK |  |
| 40 | Entity(serpen %142) | 0x4c0 | attack_effect@tag(casting 니치) | r | -1 → unwrap 패닉 (L17354~17357) | 4 | OK |  |
| 41 | Entity(serpen %142) | 0x4a0 | attack_effect.range | r | L17362 | 4 | OK |  |
| 42 | Entity(serpen %142) | 0x4a8 | attack_effect.growth_range | r | L17364 | 4 | OK |  |
| 43 | Entity(serpen %142) | 0x470 | stat_buff_cached.radius_mult | r | serpen.radius() (L17400) | 4 | OK |  |
| 44 | Entity(serpen %142) | 0x680 | radius | r | L17407·17414 | 4 | OK |  |
| 45 | Entity(적 챔프 c, 클로저 L475) | 0x0 | team@tag | r | L17768 | 4 | OK |  |
| 46 | Entity(적 챔프 c, 클로저 L475) | 0x8 | team@Player.0 | r | L17770 | 4 | OK |  |
| 47 | Entity(적 챔프 c, 클로저 L475) | 0x660 | x | r | L17783 | 4 | OK |  |
| 48 | Entity(적 챔프 c, 클로저 L475) | 0x668 | y | r | L17787 | 4 | OK |  |
| 49 | sret Vec(지역 %26 → memcpy 32B → %0) | 0x0 | buf.ptr | w | L16948 | 4 | 확인불가(tcx 사전에 타입 없음) | 8(dangling) → reserve 가 갱신 |
| 50 | sret Vec | 0x8 | buf.a(bump) | w | L16950 | 4 | 확인불가(tcx 사전에 타입 없음) | data.context.pool |
| 51 | sret Vec | 0x10 | buf.cap | w | L16954 | 4 | 확인불가(tcx 사전에 타입 없음) | 0 (memset 16B) |
| 52 | sret Vec | 0x18 | len | w | inline push 6곳 + Vec::push 호출 1곳(L17528) + extend 3곳(L18524·18535·18546) | 4 | 확인불가(tcx 사전에 타입 없음) | 0 → push 마다 +1 (L17519·17583·17645·17697·18506·18610) |
| 53 | sret Vec 원소(184B) | 0xb1 | SmallActionPlay@tag | w | 지역 버퍼에 태그 store 후 184B memcpy | 4 | 확인불가(tcx 사전에 타입 없음) | 3(RunAway: L17526·18461·18565) / 5(Around: L17474·17538) / 7(AroundRegion: L17600·17652) |
| 54 | self / rnd / player / data / parameter | 0x0 | (없음) | w | ★&mut 인자 self·rnd 에 store 0건(본문 store 전수 = Vec 4필드·원소 태그·len). rnd 콜리 4곳 모두 `_rnd readnone`. | 4 | 확인불가(tcx 사전에 타입 없음) | - |

**`consts` 상수 16건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev | folded_from |
|---|---|---|---|---|---|---|
| 0 | 2 | 422 | 태그 | ①player_champion 팀 바운즈(2) ②AroundRegion::new 의 target_region = 2(세르펜 둥지 region, L467·L470) ③CastingType::Direction 태그 ④skill2_effect() 의 `level > 2` ⑤visible_state[team] 바운즈 | 4 |  |
| 1 | 13 | 426 | 태그 | EntityType::Champion 태그 | 4 |  |
| 2 | 4 | 427 | 태그 | ChampionActionState::Skill 태그 · ult_effect() 의 `level > 4` 문턱 | 4 |  |
| 3 | 5 | 429 | 태그 | ①ChampionActionState::Skill2 태그 ②모든 후보 생성자 end_delay ③SmallActionPlay::Around 메모리태그 | 4 |  |
| 4 | 6 | 431 | 태그 | ①ChampionActionState::Ult 태그 ②EntityType::Serpen 태그(L451 `serpen.ty == Serpen`) | 4 |  |
| 5 | -1 | 427 | 센티널 | Option<Effect> None 니치 — as_ref().unwrap() 패닉 값(skill/skill2/ult/serpen.attack_effect) | 4 |  |
| 6 | 1 | 427 | 태그 | CastingType::Position 태그(논타겟). ★L458 의 `shl i64 %236, 1` 은 dmg*2 — 아래 folded 항목 | 4 |  |
| 7 | 1 | 458 | 태그 | dmg*2 가 `shl` 로 접힘 — 세르펜 평타 2방 이하로 죽는가(치명) | 4 | 2 |
| 8 | 11 | 438 | 센티널 | PositionEvalPurpose::Objective 메모리태그(니치, idx9) | 4 |  |
| 9 | 0 | 452 | 태그 | ①VisibleState::Visible ②TeamType::Player ③GameMode::Moba ④live_list.len==0 ⑤radius_mult==0 | 4 |  |
| 10 | 22500000000 | 452 | 임계 | 150000² — champ↔serpen 제곱거리가 이보다 크면(`ugt`) 원거리 → Around(serpen). Epic 판에는 없는 게이트 | 4 |  |
| 11 | 20000 | 455 | 계수 | Effect::range(effect.rs:26) 고정 가산 여유 | 4 |  |
| 12 | 100 | 455 | 계수 | Entity::radius(entity.rs:1515) 퍼센트 기준 | 4 |  |
| 13 | 22500000001 | 475 | 임계 | 150000²+1 — 적 챔프가 150000 이내면 has_near_enemy(`ult`) | 4 |  |
| 14 | 3 | 442 | 태그 | SmallActionPlay::RunAway 메모리태그 · CastingType::None(false 경로) | 4 |  |
| 15 | 7 | 467 | 태그 | SmallActionPlay::AroundRegion 메모리태그(0xb1 store L17600·17652). ⚠Epic 판과 달리 region 번호 7 은 이 함수에 없음(세르펜 region=2) | 4 |  |

**`knobs` 조정점 7건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 세르펜 둥지 region 번호(AroundRegion 목표) | serpen_poke.rs:467·470 | 2 | 세르펜 부재/비-Serpen 엔티티일 때 배회할 region. MapDef.regions 값과 맞아야 함 | 4 | 기존 |
| 1 | 세르펜 접근 거리 게이트 | serpen_poke.rs:452 | 22500000000 | 제곱거리가 이보다 크면(150000 초과) Around(serpen) 로 접근만 한다. 내리면 더 가까이 가서야 사거리/치명 판정으로 넘어간다 | 4 | 기존 |
| 2 | 치명 판정 배수 | serpen_poke.rs:458 (`shl 1`=×2) | 2 | 올리면 더 여유 있는 HP 에서도 치명으로 보고 RunAway(warn)·battle_ally_action 으로 빠진다 | 4 | 기존 |
| 3 | 근접 적 도주 반경 | serpen_poke.rs:475 | 22500000001 | 올리면 더 먼 적 챔프에도 RunAway 후보 추가 | 4 | 기존 |
| 4 | Effect::range 고정 여유 | effect.rs:26 (인라인) | 20000 | 올리면 세르펜 사거리를 넓게 봐 더 일찍 warn/RunAway | 4 | 기존 |
| 5 | 후보 end_delay | serpen_poke.rs:442·453·460·462·467·470·477·481·483 | 5 | 모든 후보의 종료 지연 틱 | 4 | 기존 |
| 6 | position_score purpose | serpen_poke.rs:438 | 11 | Objective 용도 가중치 세트 | 4 | 기존 |

<details><summary>`callees` 피호출자 30건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | action_candidates_old | game_ai::plan_legacy::sub_plan::EpicHuntSubPlan::action_candidates_old | pub | fn(&mut game_ai::plan_legacy::sub_plan::EpicHuntSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\epic_hunt.rs:678 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 1 | action_candidates_old | game_ai::plan_legacy::sub_plan::EpicPokeSubPlan::action_candidates_old | pub | fn(&mut game_ai::plan_legacy::sub_plan::EpicPokeSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\epic_poke.rs:426 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 2 | action_candidates_old | game_ai::plan_legacy::sub_plan::SerpenHuntSubPlan::action_candidates_old | pub | fn(&mut game_ai::plan_legacy::sub_plan::SerpenHuntSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\serpen_hunt.rs:677 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 3 | as_moba | game_core::GameMode::<'a>::as_moba | pub | fn(game_core::GameMode<'a/#0>) -> std::option::Option<&'a/#0 game_core::MobaMode> | game-core\src\simulation\game.rs:230 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 4 | attack_summon_action | game_ai::attack_summon_action | pub | fn(&game_core::PlayerState, &game_core::OperationData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\fight_check.rs:836 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 5 | battle_action | game_ai::battle_action | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, usize) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\fight_check.rs:620 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 6 | battle_ally_action | game_ai::battle_ally_action | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, usize) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\fight_check.rs:1112 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 7 | expected_damage_target | game_core::Effect::expected_damage_target | pub | fn(&game_core::Effect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + , &game_core::Entity) -> usize | game-core\src\simulation\effect.rs:91 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 8 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 9 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 10 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 11 | get_game_mode | game_core::AbstractGame::get_game_mode | pub | fn(&Self/#0) -> game_core::GameMode | game-core\src\simulation.rs:78 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 12 | get_game_mode | <game_core::Game as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::Game) -> game_core::GameMode | game-core\src\simulation\game.rs:1707 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 13 | get_game_mode | <game_core::SingleLaneGame as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::SingleLaneGame) -> game_core::GameMode | game-core\src\simulation\game.rs:4248 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 14 | is_in_range | game_core::Effect::is_in_range | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\effect.rs:63 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 15 | is_visible | game_core::World::is_visible | pub | fn(&game_core::World, usize, usize) -> bool | game-core\src\simulation\game\data.rs:384 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 16 | is_visible | game_core::VisibleState::is_visible | pub | fn(&game_core::VisibleState) -> bool | game-core\src\simulation\game\data.rs:121 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 17 | is_visible | game_core::AbstractGame::is_visible | pub | fn(&Self/#0, usize, usize) -> bool | game-core\src\simulation.rs:125 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 18 | is_visible_from | game_core::Entity::is_visible_from | pub | fn(&game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1481 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 19 | iter_champions | game_core::AbstractGameWithCache::<'a, 'b>::iter_champions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5759 ~ game_core[6a30]::simulation::{impl#5}::iter_champions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1904 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 20 | new | game_ai::SmallActionAround::new | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::OperationData, &game_core::PlayerState, usize, usize) -> game_ai::SmallActionAround | game-ai\src\small_action\around.rs:23 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 21 | new | game_ai::SmallActionRunAway::new | pub | fn(&game_core::OperationData, &game_core::PlayerState, usize) -> game_ai::SmallActionRunAway | game-ai\src\small_action\move_actions.rs:25 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 22 | new | game_ai::SmallActionAroundRegion::new | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::OperationData, &game_core::PlayerState, usize, usize) -> game_ai::SmallActionAroundRegion | game-ai\src\small_action\around.rs:517 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 23 | new_with_skill | game_ai::SmallActionRunAway::new_with_skill | pub | fn(&game_core::OperationData, &game_core::PlayerState, usize, bool) -> game_ai::SmallActionRunAway | game-ai\src\small_action\move_actions.rs:29 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 24 | nontarget_windup_perceived | game_ai::nontarget_windup_perceived | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity) -> bool | game-ai\src\utils.rs:553 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 25 | position_score_at_position | game_ai::position_score_at_position | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, u64, u64, game_ai::PositionEvalPurpose) -> game_core::PositioningScore | game-ai\src\position_eval.rs:1192 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 26 | radius | game_core::Entity::radius | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1509 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 27 | range | game_core::Effect::range | pub | fn(&game_core::Effect, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:25 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 28 | skill2_effect | game_core::Entity::skill2_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1692 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 29 | ult_effect | game_core::Entity::ult_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1700 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
</details>

⚠**미매칭 5개**: `dist_sq`, `extend`, `first`, `push  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 6개는 **전부 다른 함수**라 싣지 않는다`, `reserve_internal_or_panic`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m14.ll:13420) · **형제 8개** (SerpenPokeSubPlan)

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

**`open` 5건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 표기 불가 | L440 한 줄 안의 `\|\|` 3항 소스 순서 — column 없음(표기 불가; 동작은 IR 확정: 0x30 → (has_nt \|\| 0x31)). | 4 |  |
| 1 | 표기 불가 | L458/460/462 의 소스 표기 방향(`if A\|\|B {Around} else {warn;RunAway}` vs 반대) — 줄 순서(460 RunAway 가 462 보다 앞)로 후자 유력, 외연 동일(표기 불가). | 4 |  |
| 2 | 미탐색 | dbg 이름 `on_trajectory` 가 +0x31 로드에 붙은 이유 — 판정 무관. | 4 |  |
| 3 | 미탐색 | 콜리 내부 미독해(계약만): nontarget_windup_perceived(잎22) · attack_summon_action(잎22) · position_score_at_position · battle_action · battle_ally_action · SmallActionRunAway::new/new_with_skill · SmallActionAround::new · SmallActionAroundRegion::new · Effect::is_in_range/expected_damage_target(game_core) · vtable get_game_mode/get_entity_by_id/is_visible. | 4 |  |
| 4 | 미탐색 | Epic 판(L456)의 MapDef.regions 읽기가 Serpen 판에 없는 이유(설계 의도)는 소스 부재로 알 수 없음 — IR 사실만 기록. | 4 |  |

**`notes` 1건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | L452 한 줄 안의 `!is_visible_from \|\| dist>150k` 순서 — IR 은 가시성 검사 후 거리 검사이나 같은 줄이라 표기 순서는 미확정(외연 동일). | 4 | 사실 서술 |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

