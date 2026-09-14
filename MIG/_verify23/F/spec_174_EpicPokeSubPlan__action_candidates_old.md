---

### `174` EpicPokeSubPlan::action_candidates_old — 에픽(모르가드) 견제 서브플랜의 행동 후보 목록 생성 — 논타겟 회피/궤적이면 도주 단일 후보, 아니면 에픽 상태별 이동 후보 1개 + 근접 적 도주 + 전투/소환수 후보를 순서대로 덧붙인다

| 항목 | 값 |
|---|---|
| id | `EpicPokeSubPlan__action_candidates_old` |
| 심볼 | `_RNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy8sub_plan9epic_pokeNtB2_15EpicPokeSubPlan21action_candidates_old` |
| 소스 | `game-ai\src\plan_legacy\sub_plan\epic_poke.rs:426` |
| IR | `m02.ll` 45073~46762행 |
| 경로·가시성 | `game_ai::plan_legacy::sub_plan::EpicPokeSubPlan::action_candidates_old` · **pub** |
| 계층 | 기타 |
| exe | `cc9740` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r14` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&mut game_ai::plan_legacy::sub_plan::EpicPokeSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay>
```

<details><summary>인자 7개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | bumpalo::collections::vec::Vec<SmallActionPlay> (32B, sret align 8) | 결과 후보 목록. 레이아웃(tcxdict 동형 Vec<&Entity> 32B 로 확인): 0x0 ptr · 0x8 bump(&Bump) · 0x10 cap · 0x18 len. 32B 전부 live(L45123·45125 store, L45131 memset 16B로 cap/len=0, 마지막 memcpy 32B L46742/L46760). | 3 |
| 1 | 1 | self | &mut EpicPokeSubPlan (ZST 0B) | IR 속성 `readnone captures(none)` — 본문에서 읽기·쓰기 0. writes 없음. | 4 |
| 2 | 2 | version | usize | 본문 분기 0(icmp 대상 아님). nontarget_windup_perceived · position_score_at_position · Around/AroundRegion::new · battle_action/battle_ally_action 에 그대로 전달. | 4 |
| 3 | 3 | rnd | &mut StdRng (320B align 16) | IR 속성: `noalias align 16 dereferenceable(320)` (readonly 없음 = &mut). ★본문 store 0. 전달처 4곳(Around::new · AroundRegion::new · battle_action · battle_ally_action)이 모두 그 인자를 `_rnd`+`readnone` 으로 선언(m08.ll:98411/92385, m15.ll:23700/25950) ⟹ 이 함수 경로에서 rnd 는 사실상 불변. | 4 |
| 4 | 4 | player | &PlayerState (2528B) | readonly. info.team(0x930)·info.position(0x9c0) 만 직접 읽음. | 4 |
| 5 | 5 | data | &OperationData (24B) | readonly. cache(0x0)·context(0x8) 읽음. | 4 |
| 6 | 6 | parameter | &ScoreParameter (5384B) | readonly. +0x9f0 positioning_score 의 주소만 position_score_at_position 에 넘김(본문 직접 load 없음). | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn action_candidates_old(&mut self, version, rnd: &mut StdRng, player, data, parameter) -> Vec<SmallActionPlay>  // epic_poke.rs:426
  let bump = data.context.pool;                                   // L427
  let mut res = Vec::new_in(bump);                                // L427 (0x0=8,0x8=bump,cap=len=0)
  let champ = data.cache.player_champion[player.info.team][player.info.position as usize].unwrap();  // L430 (team<2 바운즈, None→panic)
  let enemy_team = 1 - player.info.team;                          // L433
  // L433~439: 적 챔프 5칸 순회(any)
  let has_non_target_action_range = data.cache.iter_champions(enemy_team).any(|c|
        nontarget_windup_perceived(version, player, data, c)       // L434
     && c.ty@tag == 13 /*Champion*/                               // L434
     && { let eff = match c.action_state@tag {                    // L435~439
             4 /*Skill*/  => c.skill_effect.as_ref().unwrap(),   // L435 (None 니치 -1 → panic)
             5 /*Skill2*/ => c.skill2_effect().as_ref().unwrap(), // L437 (level>2 ? &skill2_effect : &NONE(정적 anon.19))
             6 /*Ult*/    => c.ult_effect().as_ref().unwrap(),    // L439 (level>4 ? &ult_effect : &NONE)
             _ => return false };
          matches!(eff.casting@tag, 1 /*Position*/ | 2 /*Direction*/)   // 0=Targeting/3=None 이면 false
       && eff.is_in_range(c /*caster*/, champ /*target*/) });
  // L446~448
  let ps: PositioningScore = position_score_at_position(version, player, data, &parameter.positioning_score(0x9f0), champ.x, champ.y, 11 /*Objective*/);
  if ps.on_trajectory(0x30) || (has_non_target_action_range || ps.on_periodic_trajectory(0x31)) {   // L448 — 같은 줄 안 A||B 순서는 표기 불가(IR: 0x30 을 먼저 select)
      res.push(SmallActionPlay::RunAway(SmallActionRunAway::new_with_skill(data, player, 5 /*end_delay*/, true)));  // L450 (태그 3)
      return res;                                                 // L451
  }
  let mut warn = false;                                           // phi %287
  let region = data.context.map.regions[clamp(champ.y/32000,0,29)][clamp(champ.x/32000,0,29)];   // L456
  // L459: Moba 가 아니면 as_moba().unwrap() 패닉(game.rs:231)
  let epic: Option<&Entity> = data.cache.game.get_game_mode().as_moba().unwrap()
        .jungle_runner.epic.live_list.first().and_then(|id| data.cache.game.get_entity_by_id(*id));
  match epic {
    None =>                                   // 살아있는 에픽 없음
        res.push(AroundRegion(SmallActionAroundRegion::new(version, rnd, data, player, 7 /*region*/, 5))),   // L483 (태그 7)
    Some(epic) if epic.ty@tag != 5 /*Epic*/ =>
        res.push(AroundRegion(…::new(version, rnd, data, player, 7, 5))),    // L480
    Some(epic) if region != 7 =>              // L461: 내가 둥지 region 밖
        res.push(Around(SmallActionAround::new(version, rnd, data, player, epic.id, 5))),   // L462 (태그 5)
    Some(epic) if !epic.is_visible_from(champ) =>   // L464 (entity.rs:1482: champ.team.player_team() 이 Some(t) 이고 epic.visible_state[t]@tag != 0(Visible) 이면 '안 보임'; Neutral 이면 보이는 것으로 취급)
        res.push(Around(…::new(version, rnd, data, player, epic.id, 5))),     // L465
    Some(epic) => {
        let eff = epic.attack_effect.as_ref().unwrap();          // L467 (None→panic)
        // Effect::range(effect.rs:26, caster=champ): eff.range + 20000 + champ.stat_buff_cached.range + (champ.level-1)*eff.growth_range
        // Entity::radius(entity.rs:1511~1515): mult==0 ? radius : radius*(100+mult)/100
        let range = eff.range(champ) + champ.radius() + epic.radius();   // L467
        let dmg = eff.expected_damage_target(data.context, epic as &dyn AbstractEntity, champ);   // L468
        // L470: (dmg<<1) < champ.hp  ||  range*range < dist_sq(champ, epic)
        if dmg*2 < champ.hp || range*range < ((champ.x-epic.x)²+(champ.y-epic.y)²) {
            res.push(AroundRegion(…::new(version, rnd, data, player, 7, 5)));   // L474
        } else {                                                    // 2방 안에 죽고 + 에픽 사거리 안
            warn = true;
            res.push(RunAway(SmallActionRunAway::new(data, player, 5)));   // L472 (태그 3)
        }
    }
  }
  // L486
  if data.cache.game.is_visible(enemy_team, champ.id) {           // 적에게 내가 보이는가
      let has_near_enemy = data.cache.iter_champions(enemy_team).any(|c|
            c.team != champ.team && dist_sq(c, champ) < 22500000001 /*150000²+1*/);   // L488 (TeamType == 는 태그+페이로드 비교; 루프는 5칸 언롤 · champ.team 태그 0/≠0 두 사본)
      if has_near_enemy { res.push(RunAway(SmallActionRunAway::new(data, player, 5))); }   // L489~490
      if !warn { res.extend(battle_action(version, rnd, player, data, 5)); }        // L493~494
      else     { res.extend(battle_ally_action(version, rnd, player, data, 5)); }   // L496
  }
  res.extend(attack_summon_action(player, data));                 // L499
  res                                                             // L501~502
```

**`mem` 메모리 접근 57건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | usize. `< 2` 바운즈체크 후 인덱스(L45128~45136) | 4 | OK |  |
| 1 | PlayerState | 0x9c0 | info.position | r | game_core::Position 태그(i32, Top0/Jungle1/Mid2/Bottom3/Support4) → player_champion 2차 인덱스(L45148~45156) | 4 | OK |  |
| 2 | OperationData | 0x0 | cache | r | &AbstractGameWithCache (L45149) | 4 | OK |  |
| 3 | OperationData | 0x8 | context | r | &GameContext (L45119) | 4 | OK |  |
| 4 | GameContext | 0x0 | pool | r | &Bump — 결과 Vec 의 할당자(L45121) | 4 | OK |  |
| 5 | GameContext | 0x20 | map | r | &MapDef (L45391) | 4 | OK |  |
| 6 | AbstractGameWithCache | 0x0 | game.data_ptr | r | &dyn AbstractGame 데이터 포인터(L45399) | 4 | OK |  |
| 7 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r | &dyn AbstractGame vtable(L45400) | 4 | OK |  |
| 8 | AbstractGameWithCache | 0x1e0 | player_champion | r | [[Option<&Entity>;5];2] — [my_team][position] = champ(unwrap, L45154~45159) · [enemy_team][0..5] 순회 2회(L433·L488 any) | 4 | OK |  |
| 9 | vtable(AbstractGame) | 0x40 | get_game_mode | r | divtable 0x40. 반환 {i64 tag, ptr} = GameMode(16B); tag 0=Moba (L45402~45404) | 3 | 확인불가(tcx 사전에 타입 없음) |  |
| 10 | vtable(AbstractGame) | 0x1f0 | get_entity_by_id | r | divtable 0x1f0. (self, id) → Option<&Entity>(null=None) (L45441~45445) | 3 | 확인불가(tcx 사전에 타입 없음) |  |
| 11 | vtable(AbstractGame) | 0xf8 | is_visible | r | divtable 0xf8. (self, enemy_team, champ.id) → bool (L45765~45767) | 3 | 확인불가(tcx 사전에 타입 없음) |  |
| 12 | MapDef | 0x38b8 | regions | r | [[usize;30];30] — regions[clamp(y/32000,0,29)][clamp(x/32000,0,29)] (L45393~45397) | 4 | OK |  |
| 13 | MobaMode | 0x1a8 | jungle_runner.epic.live_list.len | r | 0 이면 에픽 없음 (L45423~45425) | 4 | OK |  |
| 14 | MobaMode | 0x1a0 | jungle_runner.epic.live_list.buf.ptr | r | first() 원소(usize 엔티티 id)를 get_entity_by_id 에 전달 (L45433~45443) | 4 | OK |  |
| 15 | ScoreParameter | 0x9f0 | positioning_score | r | PositioningScoreData(2760B) 주소만 position_score_at_position 4번째 인자로 전달 (L45348) | 4 | OK |  |
| 16 | PositioningScore(sret 56B, 지역 %27) | 0x30 | on_trajectory | r | bool (L45361) | 4 | OK |  |
| 17 | PositioningScore(sret 56B, 지역 %27) | 0x31 | on_periodic_trajectory | r | bool (L45364). dbg 이름 `on_trajectory` 가 이 로드에 붙어 있으나 두 바이트 모두 조건에 들어간다 | 4 | OK |  |
| 18 | Entity(적 챔프 c, 클로저 L434~439) | 0x68 | ty@tag | r | EntityType 태그 == 13(Champion) (L45235~45237) | 4 | OK |  |
| 19 | Entity(적 챔프 c) | 0x70 | ty@Champion.0.action_state@tag | r | ChampionActionState 태그 4=Skill/5=Skill2/6=Ult 만 통과, 그 외 false (L45262~45267) | 4 | OK |  |
| 20 | Entity(적 챔프 c) | 0x4f8 | skill_effect@tag(=casting@tag) | r | Option<Effect> 니치: -1=None(unwrap 패닉) · 1=Position/2=Direction 통과 · 0=Targeting/3=None 은 false (L45272~45277) | 4 | OK |  |
| 21 | Entity(적 챔프 c) | 0x4c8 | skill_effect | r | &Effect(56B) → is_in_range 의 self (L45289) | 4 | OK |  |
| 22 | Entity(적 챔프 c) | 0x5c8 | level | r | skill2_effect(): level>2 면 0x500 아니면 정적 None(anon.19) · ult_effect(): level>4 면 0x538 (entity.rs:1693/1701, L45295~45299·45318~45322) | 4 | OK |  |
| 23 | Entity(적 챔프 c) | 0x500 | skill2_effect | r | Option<Effect> (L45298) | 4 | OK |  |
| 24 | Entity(적 챔프 c) | 0x538 | ult_effect | r | Option<Effect> (L45321) | 4 | OK |  |
| 25 | Effect(skill2/ult 선택 결과) | 0x30 | casting@tag / Option 니치 | r | -1=None 패닉 · 1,2 통과 · 그 외 false (L45301~45305, 45324~45328) | 4 | OK |  |
| 26 | Entity(champ %50) | 0x660 | x | r | L45349 — position_score 좌표·거리 계산 | 4 | OK |  |
| 27 | Entity(champ %50) | 0x668 | y | r | L45353 | 4 | OK |  |
| 28 | Entity(champ %50) | 0x0 | team@tag | r | TeamType 태그 0=Player/1=Neutral — is_visible_from 의 player_team()(L45480) 및 L488 클로저의 `c.team != champ.team`(L45877) | 4 | OK |  |
| 29 | Entity(champ %50) | 0x8 | team@Player.0 | r | 팀 인덱스 usize (L45493~45495 · 45879) | 4 | OK |  |
| 30 | Entity(champ %50) | 0x5c8 | level | r | Effect::range(caster=champ): (level-1)*growth_range (L45547) | 4 | OK |  |
| 31 | Entity(champ %50) | 0x438 | stat_buff_cached.range | r | Effect::range 가산항 (L45551) | 4 | OK |  |
| 32 | Entity(champ %50) | 0x470 | stat_buff_cached.radius_mult | r | i32. Entity::radius(): 0 이면 radius 그대로, 아니면 radius*(100+mult)/100 (L45553~45577) | 4 | OK |  |
| 33 | Entity(champ %50) | 0x680 | radius | r | L45565·45572 | 4 | OK |  |
| 34 | Entity(champ %50) | 0x670 | hp | r | dmg*2 < hp 비교 (L45618) | 4 | OK |  |
| 35 | Entity(champ %50) | 0x5c0 | id | r | is_visible(enemy_team, champ.id) (L45763) | 4 | OK |  |
| 36 | Entity(epic %154) | 0x68 | ty@tag | r | == 5(Epic) 검사 (L45463~45465) | 4 | OK |  |
| 37 | Entity(epic %154) | 0x38 | visible_state[team] | r | [VisibleState;2] 24B stride, 태그 0=Visible (is_visible_from, L45502~45505) | 4 | OK |  |
| 38 | Entity(epic %154) | 0x5c0 | id | r | Around::new 의 target (L45487·45515) | 4 | OK |  |
| 39 | Entity(epic %154) | 0x490 | attack_effect | r | Option<Effect> — as_ref().unwrap() → &Effect (L45522) | 4 | OK |  |
| 40 | Entity(epic %154) | 0x4c0 | attack_effect@tag(casting 니치) | r | -1 이면 unwrap 패닉 (L45523~45526) | 4 | OK |  |
| 41 | Entity(epic %154) | 0x4a0 | attack_effect.range | r | Effect::range 기본항 (L45543) | 4 | OK |  |
| 42 | Entity(epic %154) | 0x4a8 | attack_effect.growth_range | r | L45545 | 4 | OK |  |
| 43 | Entity(epic %154) | 0x470 | stat_buff_cached.radius_mult | r | epic.radius() (L45581) | 4 | OK |  |
| 44 | Entity(epic %154) | 0x680 | radius | r | L45588·45595 | 4 | OK |  |
| 45 | Entity(epic %154) | 0x660 | x | r | champ↔epic 거리 (L45625) | 4 | OK |  |
| 46 | Entity(epic %154) | 0x668 | y | r | L45629 | 4 | OK |  |
| 47 | Entity(적 챔프 c, 클로저 L488) | 0x0 | team@tag | r | TeamType == 비교 (L45923) | 4 | OK |  |
| 48 | Entity(적 챔프 c, 클로저 L488) | 0x8 | team@Player.0 | r | L45925 | 4 | OK |  |
| 49 | Entity(적 챔프 c, 클로저 L488) | 0x660 | x | r | distance_sq (L45938) | 4 | OK |  |
| 50 | Entity(적 챔프 c, 클로저 L488) | 0x668 | y | r | L45942 | 4 | OK |  |
| 51 | sret Vec(지역 %28 → memcpy 32B → %0) | 0x0 | buf.ptr | w | L45123 · Vec::new_in(bump) | 4 | 확인불가(tcx 사전에 타입 없음) | 8(dangling, 빈 Vec) → push 시 reserve 가 갱신 |
| 52 | sret Vec | 0x8 | buf.a(bump) | w | L45125 | 4 | 확인불가(tcx 사전에 타입 없음) | data.context.pool |
| 53 | sret Vec | 0x10 | buf.cap | w | L45131 | 4 | 확인불가(tcx 사전에 타입 없음) | 0 (memset 16B) |
| 54 | sret Vec | 0x18 | len | w | inline push 6곳 + Vec::push 호출 2곳(L45533·45710) + extend 3곳 | 4 | 확인불가(tcx 사전에 타입 없음) | 0 → push 마다 +1 (L45701·45756·45809·45852·46662·46757) |
| 55 | sret Vec 원소(184B) | 0xb1 | SmallActionPlay@tag | w | 지역 버퍼에 태그를 박고 184B memcpy 로 원소 슬롯에 복사 | 4 | 확인불가(tcx 사전에 타입 없음) | 3(RunAway: L45708·46616·46721) / 5(Around: L45531·45720) / 7(AroundRegion: L45665·45773·45816) |
| 56 | self / rnd / player / data / parameter | 0x0 | (없음) | w | ★&mut 인자 self·rnd 에 대한 store 0건(본문 store 전수 = Vec 4필드·원소 태그·len 뿐). rnd 를 받는 콜리 4곳 모두 `_rnd readnone`. | 4 | 확인불가(tcx 사전에 타입 없음) | - |

**`consts` 상수 17건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev | folded_from |
|---|---|---|---|---|---|---|
| 0 | 2 | 430 | 임계 | player_champion 1차 인덱스 바운즈(팀 수 2). 동일 값이 is_visible_from 의 visible_state[team] 바운즈(entity.rs:1483)에도 쓰임 | 4 |  |
| 1 | 13 | 434 | 태그 | EntityType::Champion 메모리태그(tcxdict --enum EntityType: idx13=태그13). 적 c 가 챔피언일 때만 | 3 |  |
| 2 | 4 | 435 | 태그 | ChampionActionState::Skill 태그 → c.skill_effect 를 본다. 별도로 ult_effect() 의 `level > 4` 문턱(entity.rs:1701)에도 4 가 쓰임 | 4 |  |
| 3 | 5 | 437 | 태그 | ①ChampionActionState::Skill2 태그 ②EntityType::Epic 태그(L460 `epic.ty == Epic`) ③모든 후보 생성자의 end_delay 인자(RunAway/Around/AroundRegion/battle_action) ④SmallActionPlay::Around 메모리태그(0xb1 store) | 4 |  |
| 4 | 6 | 439 | 태그 | ChampionActionState::Ult 태그 → c.ult_effect() | 4 |  |
| 5 | -1 | 435 | 센티널 | Option<Effect> None 니치(casting@tag=0xFFFFFFFF). skill/skill2/ult 및 epic.attack_effect 의 as_ref().unwrap() 이 이 값이면 패닉 | 4 |  |
| 6 | 1 | 435 | 태그 | CastingType::Position 태그 — 논타겟(Position/Direction)만 통과. ★L470 의 `shl i64 %dmg, 1` 은 dmg*2(folded_from 2) — 아래 별도 항목 | 4 |  |
| 7 | 1 | 470 | 태그 | dmg*2 가 `shl i64 %239, 1` 로 접힘 — 에픽 평타 2방 이하로 죽는가(치명) 판정 | 4 | 2 |
| 8 | 11 | 446 | 센티널 | PositionEvalPurpose::Objective 메모리태그(idx9→태그11, 니치). position_score_at_position 의 purpose | 4 |  |
| 9 | 32000 | 456 | 인덱스 | 셀 크기(좌표→셀 인덱스 변환) — 임계 아님 | 4 |  |
| 10 | 29 | 456 | 인덱스 | regions 그리드 인덱스 clamp 상한(30x30). 좌표 변환용 | 4 |  |
| 11 | 7 | 461 | 태그 | ①에픽 둥지 region 번호 — `region == 7` 이면 둥지 안, 아니면 Around(epic) ②AroundRegion::new 의 target_region ③SmallActionPlay::AroundRegion 메모리태그(0xb1 store) | 4 |  |
| 12 | 0 | 464 | 태그 | ①VisibleState::Visible 태그(is_visible_from) ②TeamType::Player 태그 ③GameMode::Moba 태그(L459) ④live_list.len==0 검사 ⑤radius_mult==0 분기 | 4 |  |
| 13 | 20000 | 467 | 계수 | Effect::range(effect.rs:26) 의 고정 가산 여유(= self.range + 20000 + caster.stat_buff_cached.range + (level-1)*growth_range) | 4 |  |
| 14 | 100 | 467 | 계수 | Entity::radius(entity.rs:1515): radius*(100+radius_mult)/100 의 퍼센트 기준 | 4 |  |
| 15 | 22500000001 | 488 | 임계 | 150000²+1 — 적 챔프가 150000(≈4.7셀) 이내에 있으면 has_near_enemy (제곱거리 `ult` 비교) | 4 |  |
| 16 | 3 | 450 | 태그 | SmallActionPlay::RunAway 메모리태그(0xb1 store · niche_start 3) | 4 |  |

**`knobs` 조정점 6건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 에픽 둥지 region 번호 | epic_poke.rs:461 (및 AroundRegion::new 인자 L474/480/483) | 7 | 바꾸면 '둥지 안/밖' 판정과 AroundRegion 목표 region 이 함께 옮겨간다(MapDef.regions 값과 맞아야 함) | 4 | 기존 |
| 1 | 치명 판정 배수(에픽 평타 몇 방에 죽는가) | epic_poke.rs:470 (`shl 1`=×2) | 2 | 올리면(예 ×3) 더 여유 있는 HP 에서도 '치명'으로 보고 도주(warn)·battle_ally_action 으로 빠진다; 내리면 더 오래 둥지 안에 머문다 | 4 | 기존 |
| 2 | 근접 적 도주 반경 | epic_poke.rs:488 | 22500000001 | 올리면 더 먼 적 챔프에도 RunAway 후보가 추가된다(150000=약 4.7셀) | 4 | 기존 |
| 3 | Effect::range 고정 여유 | effect.rs:26 (인라인) | 20000 | 올리면 에픽 사거리를 더 넓게 봐서 더 일찍 warn/RunAway | 4 | 기존 |
| 4 | 후보 end_delay | epic_poke.rs:450·462·465·472·474·480·483·490·494·496 | 5 | 모든 이동/전투 후보의 종료 지연 틱. 올리면 후보가 더 오래 유지 | 4 | 기존 |
| 5 | position_score purpose | epic_poke.rs:446 | 11 | Objective 용도 가중치 세트 사용. 다른 purpose 로 바꾸면 궤적/on_trajectory 판정 재료가 달라짐(내부는 position_score_at_position 명세) | 4 | 기존 |

<details><summary>`callees` 피호출자 33건 (tcx 자동 생성)</summary>

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
| 25 | player_team | game_core::TeamType::player_team | pub | fn(&game_core::TeamType) -> std::option::Option<usize> | game-core\src\simulation\entity.rs:1135 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 26 | player_team | game_view::ClientData::player_team | pub | fn(&game_view::ClientData) -> std::option::Option<std::cell::Ref< game_core::Team>> | game-view\src\logic\client\data.rs:1579 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 27 | player_team | game_view::ClientDatabase::player_team | pub | fn(&game_view::ClientDatabase) -> &game_core::Team | game-view\src\logic\client\data.rs:1163 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 28 | position_score_at_position | game_ai::position_score_at_position | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, u64, u64, game_ai::PositionEvalPurpose) -> game_core::PositioningScore | game-ai\src\position_eval.rs:1192 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 29 | radius | game_core::Entity::radius | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1509 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 30 | range | game_core::Effect::range | pub | fn(&game_core::Effect, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:25 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 31 | skill2_effect | game_core::Entity::skill2_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1692 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 32 | ult_effect | game_core::Entity::ult_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1700 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
</details>

⚠**미매칭 9개**: `clamp`, `dist_sq`, `extend`, `first`, `on_periodic_trajectory`, `on_trajectory`, `positioning_score`, `push  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 6개는 **전부 다른 함수**라 싣지 않는다`, `reserve_internal_or_panic`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m02.ll:40641) · **형제 8개** (EpicPokeSubPlan)

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

**`open` 6건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 표기 불가 | L448 한 줄 안의 `on_trajectory \|\| has_non_target_action_range \|\| on_periodic_trajectory` 소스 표기 순서 — column 정보 없음(표기 불가, 동작은 IR 확정: 0x30 → (has_nt \|\| 0x31)). | 4 |  |
| 1 | 표기 불가 | L470/472/474 의 소스 표기가 `if A\|\|B {AroundRegion} else {warn;RunAway}` 인지 `if !A&&!B {warn;RunAway} else {AroundRegion}` 인지 — 줄 순서(472 RunAway 가 474 보다 앞)로는 후자가 유력하나 외연 동일(표기 불가). 동작은 IR 대로. | 4 |  |
| 2 | 미탐색 | dbg 이름 `on_trajectory` 가 +0x31(on_periodic_trajectory) 로드에 붙어 있는 이유(지역 변수 바인딩인지 컴파일러 아티팩트인지) — 판정에는 무관. | 4 |  |
| 3 | 미탐색 | L493 의 warn 분기 의미(치명 사거리 안이면 battle_ally_action, 아니면 battle_action)는 IR 확정이나, 두 함수의 내부 차이는 이 명세 범위 밖(r14 중간 콜리 — 시그니처만). | 4 |  |
| 4 | 미탐색 | 콜리 내부 미독해(계약만): nontarget_windup_perceived(잎22) · attack_summon_action(잎22) · position_score_at_position · battle_action · battle_ally_action · SmallActionRunAway::new/new_with_skill · SmallActionAround::new · SmallActionAroundRegion::new · Effect::is_in_range/expected_damage_target(game_core 경계) · AbstractGame vtable get_game_mode/get_entity_by_id/is_visible. | 4 |  |
| 5 | 재료 부재 | regions 그리드가 [y][x] 순서인 근거는 IR 인덱스 순서(외측=y/32000, 내측=x/32000)뿐 — MapDef.regions 의 의미(행=y) 는 tcxdict 로는 확인 불가. | 3 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

