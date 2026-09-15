---

### `210` StealSubPlan::action_candidates — 정글러 막타 스틸 후보군: commit 이면 대상 오브젝트 사거리 내 공격/스킬/스킬2/궁 + 진입 지점 대기, Lurk 면 적 근접 시 도주 아니면 대기 부시

| 항목 | 값 |
|---|---|
| id | `steal__Steal__action_candidates` |
| 심볼 | `_RNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy8sub_plan5stealNtB2_12StealSubPlan17action_candidates` |
| 소스 | `game-ai\src\plan_legacy\sub_plan\steal.rs:33` |
| IR | `m02.ll` 25360~27425행 |
| 경로·가시성 | `game_ai::plan_legacy::sub_plan::StealSubPlan::action_candidates` · **pub** |
| 계층 | 기타 |
| exe | `cba660` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r17` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&mut game_ai::plan_legacy::sub_plan::StealSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay>
```

**TLS 메모 절(`sig.tls` · 정정 경로 `/specs[210]/sig/tls/<키>`)**

없음 — 본문의 @anon 참조 7개(155~159·26 = 패닉 Location · 19 = 정적 None<Effect>)가 전부이고 `constant ptr @<KEY…call_once>` 형 fn-포인터 상수 0 · LocalKey::with 0

<details><summary>인자 7개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | bumpalo::Vec<SmallActionPlay> (32B) | sret([32 x i8]) writeonly. +0 ptr · +0x8 bump · +0x10 cap · +0x18 len — 4워드 전부 live(m02.ll:25397~25403 초기화 → 27313 memcpy 32B) | 4 |
| 1 | 1 | self | &mut StealSubPlan (16B) | 소스는 `&mut self`(tcx sig) 지만 define 속성은 `readonly`(m02.ll:25360) → **쓰기 표면 0**. 읽는 필드 = target(+0x8 니치: 0 Epic·1 Serpen·2 None) · commit(+0x9 bool). last_vision_tick(+0) 은 읽지 않음 | 3 |
| 2 | 2 | _version | usize | 미사용(DI `_version` m02.ll:25385) — 본문 분기 없음 | 4 |
| 3 | 3 | rnd | &mut StdRng (320B) | IR 속성 없음(=&mut). 본문 직접 gen_range 사이트 0 · 통과처 2곳만: L47 SmallActionAroundPosition::new(27339) · L104 SmallActionAroundPosition::new_with_out_line(27258) — 둘은 상호배타 경로(폴백 vs 커밋)라 한 호출당 최대 1회 전달 | 4 |
| 4 | 4 | player | &PlayerState (2528B) | IR readonly · info.team(0x930)·info.position(0x9c0) 만 직접 읽고 콜리(Recall::new · RunAway::new_with_skill)에 전달 | 4 |
| 5 | 5 | data | &OperationData (24B) | IR readonly · cache(+0)·context(+8) | 4 |
| 6 | 6 | _parameter | &ScoreParameter (5384B) | IR readonly · 미사용(DI `_parameter` m02.ll:25389) | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn action_candidates(&mut self, _version, rnd, player, data, _parameter) -> Vec<SmallActionPlay>   [steal.rs:33]
  bump = data.context.pool ; res = Vec::new_in(bump)                                              [L34 · 25393~25403]
  champ = data.cache.player_champion[player.info.team /*<2*/][player.info.position]              [L36 · 25402~25434]
  if champ is None { res.push(Recall(SmallActionRecall::new(data, player, 5))); return res }      [L37 · 25537, 27384~27422]

  // ---- 대상 오브젝트 해석 (steal.rs:27/28/30 헬퍼 인라인) ----
  target_ent = match self.target {                                                                 [L41 · 25437~25447]
      None(2)      → 폴백 →                                                                        [→ %872]
      Some(Epic=0) → moba = game.get_game_mode()/*판별자0=Moba 아니면 unwrap 패닉*/ ; moba.jungle_runner.epic.live_list.first()   [L27 · +0x198]
      Some(Serpen) → … .jungle_runner.serpen.live_list.first()                                    [L28 · +0x1c8]
  } → id → game.get_entity_by_id(id)                                                             [L30 · 25514~25525]
  if 해석 실패(live_list 비었거나 엔티티 없음 · %99) 또는 None {                                   [L43 · 25547]
      (x,y) = data.context.map.camp_pos(if self.target==Serpen { JungleType::Serpen(5) } else { Morgard(4) }, player.info.team == 0)   [L44/L45 · 27318~27329]
      res.push(AroundPosition(SmallActionAroundPosition::new(rnd, data, x, y, 5))); return res      [L47 · 27339~27379]
  }
  champ = …(재로드) ; target = (L41 과 같은 해석을 다시 수행).unwrap()                             [L50~51 · 25552~25683 · 두 번째 해석 실패는 unwrap 패닉(149) — 첫 해석이 성공했으므로 동일 값]

  if self.commit {                                                                                 [L53 · 25676~25679]
      move_speed = champ.stat_cached.move_speed                                                    [L54]
      // 공격
      if champ.can_attack() {                                                                      [L56 · 25689]
        if let Some(atk) = &champ.attack_effect {                                                  [L57 · 0x4c0 != -1]
          max = atk.range(champ)/*range + stat_buff.range + growth*(level-1)*/ + atk.range_adjust(champ,target) + champ.radius() + target.radius()   [L58 · 25714~25811]
          d2  = target.distance_sq(champ)                                                          [L59 · 25775~25804]
          max += move_speed*30                                                                     [L60 · 25805]
          if d2 <= max*max { res.push(Attack(SmallActionAttack::new(data, target.id))) }           [L61~62 · 25813~25864 · 조건은 `icmp ugt d2, max²` 의 거짓 가지]
        } }
      // 스킬
      if let Some(skill) = &champ.skill_effect {                                                   [L67 · 0x4f8 != -1]
        if champ.can_skill() && skill.target.check(champ, target) {                               [L68 · 25871, 25891 · && 단락: can_skill 거짓이면 check 미호출]
          max = skill.range(champ) + range_adjust + champ.radius() + target.radius()               [L69]
          d2 = target.distance_sq(champ)                                                           [L70]
          max += move_speed*30                                                                     [L71 · 25989]
          if d2 <= max² { res.push(Skill(SmallActionSkill::new(data, target.id))) }                [L72~73 · 26006~26014]
        } }
      // 스킬2 (level>2 아니면 None)
      if let Some(skill2) = champ.skill2_effect() {                                                [L78 · 25875~25884]
        if champ.can_skill2() && skill2.target.check(champ, target) {                             [L79 · 26024, 26042]
          max = skill2.range(champ) + range_adjust + radii ; d2 ; max += move_speed*30              [L80~82]
          if d2 <= max² { res.push(Skill2(SmallActionSkill2::new(data, target.id))) }              [L83~84 · 26155~26163]
        } }
      // 궁 (level>4 아니면 None)
      if let Some(ult) = champ.ult_effect() {                                                      [L89 · 26028~26035]
        if champ.can_ult() && ult.target.check(champ, target) {                                   [L90 · 26173, 26181]
          max = ult.range(champ) + range_adjust + radii ; d2 ; max += move_speed*30                 [L91~93]
          if d2 <= max² { res.push(Ult(SmallActionUlt::new(data, target.id))) }                    [L94~95 · 26294~26302]
        } }
      tps = data.context.setting.tick_per_second                                                   [L102 · 27033~27036]
      (x,y) = steal_damage_entry_pos(data.context, champ, target, tps*5)                            [L103 · 27038~27039]
      res.push(AroundPosition(SmallActionAroundPosition::new_with_out_line(rnd, data, x, y, 5, Outline)))   [L104 · 27258~27308]
  } else { // Lurk
      near_enemy = data.cache.player_champion[1 - team].iter().flatten()                            [L107 · 26311~26313]
                     .any(|e| e.is_visible_from(champ) /*visible_state[my_team]==Visible*/ && e.distance_sq(champ) <= 130000²)   [26329~26945 · 5명 unrolled ×2판]
      if near_enemy { res.push(RunAway(SmallActionRunAway::new_with_skill(data, player, 5, true))) }   [L109~110 · 27051, 27209~27247]
      else if let Some(t) = self.target {                                                          [L111 · 27044 · None 이면 빈 res 반환 — 위 해석 통과 후라 실제 도달 불가]
          bush = steal_wait_bush(t, player.info.team, champ, data.context.map)                     [L112 · 27057~27059]
          target = (세 번째 해석).unwrap()                                                          [L113 · 27071~27163]
          res.push(AroundBush(SmallActionAroundBush::new_with_target(data, target, bush, Outline)))   [L113 · 27158~27204]
      }
  }
  return res                                                                                       [L120 · 27313]

극성 메모: 사거리 게이트 4개 모두 `dist_sq > max²` 이면 건너뜀(ugt 의 참 가지가 skip). Lurk 의 any 는 시야 검사 → 거리 순(&& 단락). gen_range 직접 호출 0(rnd 는 AroundPosition 생성자 2곳에 통과). self 쓰기 0.
```

**`mem` 메모리 접근 34건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | StealSubPlan | 0x8 | target@tag | r | 0 Epic / 1 Serpen / 2 None. L41(25437) · L51(25574 재매치) · L43(25547 ==1) · L111(27044 ==2, 27055 trunc→bool) · L113(27071 ==0) | 4 | OK |  |
| 1 | StealSubPlan | 0x9 | commit | r | bool · L53 (25676~25679) — true=커밋 가지 / false=Lurk 가지 | 4 | OK |  |
| 2 | OperationData | 0x8 | context | r | 25393 · +0 pool(bump 25395) · +0x8 setting(27033) · +0x20 map(27057, 27318, 27325) | 4 | OK |  |
| 3 | OperationData | 0x0 | cache | r | 25425 | 4 | OK |  |
| 4 | GameSetting | 0x12f8 | tick_per_second | r | L102 (27035~27036) · ready_ticks = tps*5 | 4 | OK |  |
| 5 | PlayerState | 0x930 | info.team | r | 25402~25405 `<2` 바운드체크 · L45/L44 `team==0`(is_blue_side) 27320/27327 · L107 `1-team`(적 팀 인덱스) 26311 · L112 steal_wait_bush 인자 | 4 | OK |  |
| 6 | PlayerState | 0x9c0 | info.position@tag | r | i32 zext (25422~25424) | 4 | OK |  |
| 7 | AbstractGameWithCache | 0x1e0 | player_champion[team][pos] / [1-team][0..5] | r | [2][5] Option<&Entity>(null=None) · 내 챔프 25427~25433 · 적 5명 unrolled 26344, 26397, 26453, 26509, 26565(비-Player 팀 판) / 26621, 26693, 26758, 26823, 26888(Player 팀 판) | 4 | OK |  |
| 8 | AbstractGameWithCache | 0x0 | game.data_ptr(+0)/vtable_ptr(+0x8) | r | &dyn AbstractGame · vtable+0x40 get_game_mode(25453, 25469, 25582, 25597, 27074) · vtable+0x1f0 get_entity_by_id(25522, 25649, 27140) | 4 | OK |  |
| 9 | GameMode({i64,ptr}) | 0x0 | 판별자 | r | ==0 → MobaMode ptr · 아니면 unwrap_failed(25502/25528 …) — mode.rs:231 as_moba 류 인라인 | 4 | OK |  |
| 10 | MobaMode | 0x198 | jungle_runner.epic | r | Epic 가지 phi 408 · +0x8 live_list.ptr · +0x10 live_list.len (25483, 25492~25498, 25509~25510) | 4 | OK |  |
| 11 | MobaMode | 0x1c8 | jungle_runner.serpen | r | Serpen 가지 phi 456 | 4 | OK |  |
| 12 | Entity(대상) | 0x5c0 | id | r | Attack/Skill/Skill2/Ult ::new 의 target 인자 (25820, 26004, 26153, 26292) | 4 | OK |  |
| 13 | Entity(내 챔프) | 0x640 | stat_cached.move_speed | r | L54 (25686~25687) · 각 사거리에 move_speed*30 가산 | 4 | OK |  |
| 14 | Entity | 0x490 | attack_effect (Option<Effect> · ty Arc 16B) | r | L57 · Effect 레이아웃: +0x10 range(0x4a0) · +0x18 growth_range(0x4a8) · +0x20 start_timing · +0x28 target(CastingTarget 4B, 0x4b8) · +0x2c attack_type · +0x30 casting(CastingType 4B, 0x4c0 — Option<Effect> 니치 태그, i32 -1=None) | 4 | OK |  |
| 15 | Entity | 0x4c0 | attack_effect@tag | r | == -1 → 공격 후보 건너뜀 (25705~25708) | 4 | OK |  |
| 16 | Entity | 0x4a0 | attack_effect.range | r | Effect::range(caster) 인라인 effect.rs:26 (25714~25715) | 4 | OK |  |
| 17 | Entity | 0x4a8 | attack_effect.growth_range | r | × (level-1) (25716~25717, 25726~25727) | 4 | OK |  |
| 18 | Entity | 0x5c8 | level | r | range 성장 (25718) · skill2_effect(level>2, 25875~25879 entity.rs:1693) · ult_effect(level>4, 26028~26030 entity.rs:1701) | 4 | OK |  |
| 19 | Entity | 0x438 | stat_buff_cached.range | r | Effect::range 가산 (25720~25721 등) | 4 | OK |  |
| 20 | Entity | 0x470 | stat_buff_cached.radius_mult | r | Entity::radius() 인라인 entity.rs:1511~1515: mult==0 ? radius : radius*(mult+100)/100 (25728~25746) — 내 챔프·대상 각각 | 4 | OK |  |
| 21 | Entity | 0x680 | radius | r | 위 radius() 의 기본값 (25735, 25742, …) | 4 | OK |  |
| 22 | Entity | 0x660 | x | r | Entity::distance_sq 인라인 entity.rs:2158 (25775, 25783 …) | 4 | OK |  |
| 23 | Entity | 0x668 | y | r | 동상 (25779, 25787 …) | 4 | OK |  |
| 24 | Entity | 0x4c8 | skill_effect (ty Arc) | r | L67 · +0x4f0 target(CastingTarget) → CastingTarget::check 인자(25890) · +0x4f8 tag(25698~25701) · +0x4d8 range(25898) · +0x4e0 growth_range(25900) | 4 | OK |  |
| 25 | Entity | 0x4f8 | skill_effect@tag | r | == -1 → 스킬 후보 건너뜀 | 4 | OK |  |
| 26 | Entity | 0x4f0 | skill_effect.target (CastingTarget) | r | CastingTarget::check(&effect.target, champ, target) (25890~25891) | 4 | OK |  |
| 27 | Entity | 0x500 | skill2_effect | r | L78 · level>2 일 때만 실체, 아니면 정적 None(@anon.19) · +0x30(0x530) casting 니치 tag · +0x28(0x528) target(CastingTarget)(26041) · +0x10 range(26049) · +0x18 growth_range(26051) | 4 | OK |  |
| 28 | Entity | 0x538 | ult_effect | r | L89 · level>4 일 때만 실체(1336) · +0x30(0x568) casting 니치 tag(26032~26034) · +0x28 target(CastingTarget)(26180) · +0x10 range(26188) · +0x18 growth(26190) | 4 | OK |  |
| 29 | Entity(적 챔프) | 0x38 | visible_state[my_team]@tag | r | L107 Entity::is_visible_from(champ) 인라인 entity.rs:1483 → data.rs:122: `[i64; 3]` stride 24 (gepS 26652~26654) · ==0 Visible 만 통과 · 내 팀 인덱스 `<2` 바운드체크(26616, 26958) | 4 | OK |  |
| 30 | Entity(내 챔프) | 0x0 | team@tag / +0x8 team@Player.0 | r | L107 is_visible_from 의 other.team 분기: 판별자 하위비트(26329~26331 trunc) 가 1(비-Player) 이면 시야 검사 생략판(%525~), 0(Player) 이면 team 인덱스(%520)로 시야 검사판(%629~) | 4 | OK |  |
| 31 | MapDef | 0x0 | (camp_pos · steal_wait_bush 인자) | r | context+0x20 참조만 전달 · 필드 직접 읽기 없음 | 4 | OK |  |
| 32 | sret Vec<SmallActionPlay> | 0x0..0x20 | ptr/bump/cap/len | w | 25397~25403 초기화 · push 는 reserve_internal_or_panic(0,1)+memcpy184+len+1 인라인(25845~25864 등) 또는 SmallActionPlay::push 콜(26014, 26163, 26302) · 27313 memcpy 32B → sret. **self 쓰기 0**(define %1 readonly — `&mut self` 시그니처지만 이 함수는 상태를 바꾸지 않는다 · last_vision_tick 갱신은 다른 함수) | 4 | 확인불가(tcx 사전에 타입 없음) | Vec::new_in(bump) + push 0~5회 |
| 33 | Vec 원소 | 0xb1 | SmallActionPlay 태그 | w | 27386 · 25828 · 26012 · 26161 · 26300 · 27211 · 27168 | 4 | 확인불가(tcx 사전에 타입 없음) | 4(Recall L37) / 15(Attack L62) / 16(Skill L73) / 17(Skill2 L84) / 18(Ult L95) / 3(RunAway L110) / 12(AroundBush L113) / AroundPosition 은 store 없음(생성자가 쓴 outline_type 이 니치) |

**`consts` 상수 32건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 5 | 37 | 계수 | SmallActionRecall::new end_delay (champ None 폴백) (25537) | 4 |
| 1 | 5 | 47 | 계수 | SmallActionAroundPosition::new end_delay (target 해석 실패 폴백 · 캠프 위치 대기) (27339) | 4 |
| 2 | 5 | 103 | 계수 | ready_ticks = tick_per_second * 5 → steal_damage_entry_pos 4번 인자(= 5초 분량 틱) (27038 `mul %783, 5`) | 4 |
| 3 | 5 | 104 | 계수 | SmallActionAroundPosition::new_with_out_line end_delay (커밋 진입 지점 대기) (27258) | 4 |
| 4 | 5 | 110 | 계수 | SmallActionRunAway::new_with_skill end_delay (Lurk 적 근접 도주) (27051) | 4 |
| 5 | 30 | 60 | 계수 | 공격 사거리 판정에 move_speed*30 가산(30틱 = 60tps 기준 0.5초 이동량 — 틱 단위는 추정) (25805) | 5 |
| 6 | 30 | 71 | 계수 | 스킬 사거리 판정 move_speed*30 (25989) | 4 |
| 7 | 30 | 82 | 계수 | 스킬2 사거리 판정 move_speed*30 (26138) | 4 |
| 8 | 30 | 93 | 계수 | 궁 사거리 판정 move_speed*30 (26277) | 4 |
| 9 | 16900000001 | 107 | 임계 | 130000² + 1 — 적 챔프 근접 임계(제곱 · `dist_sq < 130000²+1` ⇔ `<= 130000²` · 130000 = 4.06셀) (26388 등 10곳) | 4 |
| 10 | 2 | 36 | 임계 | player_champion 1차 길이 바운드체크 team<2 (25405) · L107 적팀 visible_state 인덱스 바운드체크(26616) | 4 |
| 11 | 2 | 78 | 임계 | Entity::skill2_effect 인라인 `level > 2` (25877, entity.rs:1693) | 4 |
| 12 | 4 | 89 | 임계 | Entity::ult_effect 인라인 `level > 4` (26028, entity.rs:1701) | 4 |
| 13 | 100 | 58 | 계수 | Entity::radius() 인라인 radius*(radius_mult+100)/100 (25744~25746 등 8곳) | 4 |
| 14 | -1 | 57 | 센티널 | Option<Effect> 니치 None(Effect+0x30 casting:CastingType 의 i32 니치 = -1) — attack/skill/skill2/ult 각각 (25707, 25700, 25883, 26034) | 4 |
| 15 | 1 | 58 | 태그 | growth_range × (level − 1) 의 −1 (25726 `add %171, -1`) · L107 `1 − team` 적 팀 인덱스(26311) · L104/L113 AroundBushOutlineType::Outline 태그 i8 1 (27258, 27158) · L110 with_skill=true | 4 |
| 16 | 4 | 45 | 태그 | JungleType::Morgard 태그 4 → MapDef::camp_pos(map, Morgard, team==0) — target None 또는 Epic 해석 실패 폴백 (27321) | 4 |
| 17 | 5 | 44 | 계수 | JungleType::Serpen 태그 5 → camp_pos(map, Serpen, team==0) — Serpen 해석 실패 폴백 (27328) | 4 |
| 18 | 0 | 44 | 태그 | `team == 0` → camp_pos 의 is_blue_side (27320, 27327) | 4 |
| 19 | 2 | 41 | 센티널 | Option<StealTarget> None 니치 태그 (25445, 27044) | 4 |
| 20 | 1 | 43 | 태그 | StealTarget::Serpen 태그 1 (25547) — 폴백 캠프 선택 | 4 |
| 21 | 3 | 110 | 태그 | SmallActionPlay::RunAway 태그 (27211) | 4 |
| 22 | 4 | 37 | 태그 | SmallActionPlay::Recall 태그 (27386) | 4 |
| 23 | 12 | 113 | 태그 | SmallActionPlay::AroundBush 태그 (27168) | 4 |
| 24 | 15 | 62 | 태그 | SmallActionPlay::Attack 태그 (25828) | 4 |
| 25 | 16 | 73 | 태그 | SmallActionPlay::Skill 태그 (26012) | 4 |
| 26 | 17 | 84 | 태그 | SmallActionPlay::Skill2 태그 (26161) | 4 |
| 27 | 18 | 95 | 태그 | SmallActionPlay::Ult 태그 (26300) | 4 |
| 28 | 408 | 27 | 산출값 | MobaMode.jungle_runner.epic 오프셋 0x198 (phi 25483, 25611, 27102) | 4 |
| 29 | 456 | 28 | 산출값 | MobaMode.jungle_runner.serpen 오프셋 0x1c8 | 4 |
| 30 | 64 | 27 | 미상 | AbstractGame vtable 슬롯 0x40 = get_game_mode | 4 |
| 31 | 496 | 30 | 미상 | AbstractGame vtable 슬롯 0x1f0 = get_entity_by_id | 4 |

**`knobs` 조정점 6건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | Lurk 적 근접 도주 임계 | steal.rs:107 | 16900000001 | 130000²+1. 키우면 더 먼 적에도 도주(RunAway) 후보로 바뀌어 대기 부시를 자주 버린다 · 줄이면 적이 붙을 때까지 부시 대기 | 4 | 기존 |
| 1 | 공격/스킬 사거리 여유(이동 30틱) | steal.rs:60/71/82/93 | 30 | 키우면 아직 사거리 밖인 대상에도 공격/스킬 후보를 만든다(실제 시전 성공은 액션 층) · 0 이면 정확히 사거리 안일 때만 | 4 | 기존 |
| 2 | 커밋 진입 지점 준비 시간 | steal.rs:103 | 5 | tps*5 틱(=5초) 를 steal_damage_entry_pos 의 ready_ticks 로 — 진입 위치 계산 기준 시간(콜리 내부 의미는 r? 범위 밖) | 4 | 기존 |
| 3 | 각 액션 end_delay | steal.rs:37/47/104/110 | 5 | Recall·AroundPosition·RunAway 의 end_delay — 단위·효과는 각 액션 is_end 쪽(범위 밖) | 4 | 기존 |
| 4 | AroundBushOutlineType | steal.rs:104/113 | Outline(1) | Inline(2)/None(0) 으로 바꾸면 부시 대기/진입 대기의 외곽선 처리 방식이 바뀐다(around.rs 쪽 의미) | 4 | 기존 |
| 5 | 폴백 캠프 | steal.rs:44~45 | Serpen→JungleType::Serpen(5) · 그 외→Morgard(4) | 대상 해석 실패 시 대기할 캠프 좌표 — target None 이면 무조건 에픽(Morgard) 캠프 | 4 | 기존 |

<details><summary>`callees` 피호출자 33건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | action_candidates | game_ai::plan_legacy::sub_plan::SubPlan::action_candidates | pub | fn(&mut game_ai::plan_legacy::sub_plan::SubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\mod.rs:118 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 18개 중 상위 3개 |
| 1 | action_candidates | game_ai::plan_legacy::sub_plan::HideSubPlan::action_candidates | pub | fn(&mut game_ai::plan_legacy::sub_plan::HideSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\hide.rs:19 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 18개 중 상위 3개 |
| 2 | action_candidates | game_ai::plan_legacy::sub_plan::StealSubPlan::action_candidates | pub | fn(&mut game_ai::plan_legacy::sub_plan::StealSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\steal.rs:33 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 18개 중 상위 3개 |
| 3 | camp_pos | game_core::MapDef::camp_pos | pub | fn(&game_core::MapDef, game_core::JungleType, bool) -> (u64, u64) | game-core\src\simulation\map_def.rs:207 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | can_attack | game_core::Entity::can_attack | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1527 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 5 | can_skill | game_core::Entity::can_skill | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1708 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 6 | can_skill2 | game_core::Entity::can_skill2 | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1721 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 7 | can_ult | game_core::Entity::can_ult | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1734 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 8 | check | game_core::CastingTarget::check | pub | fn(&game_core::CastingTarget, &game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\effect\type.rs:227 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 9 | distance_sq | game_core::utils::distance_sq | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:6 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 10 | distance_sq | game_core::Entity::distance_sq | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2157 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 11 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 12 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 13 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 14 | get_game_mode | game_core::AbstractGame::get_game_mode | pub | fn(&Self/#0) -> game_core::GameMode | game-core\src\simulation.rs:78 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 15 | get_game_mode | <game_core::Game as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::Game) -> game_core::GameMode | game-core\src\simulation\game.rs:1707 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 16 | get_game_mode | <game_core::SingleLaneGame as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::SingleLaneGame) -> game_core::GameMode | game-core\src\simulation\game.rs:4248 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 17 | is_visible_from | game_core::Entity::is_visible_from | pub | fn(&game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1481 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 18 | new | game_ai::SmallActionUlt::new | pub | fn(&game_core::OperationData, usize) -> game_ai::SmallActionUlt | game-ai\src\small_action\cast.rs:247 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 19 | new | game_ai::SmallActionSkill::new | pub | fn(&game_core::OperationData, usize) -> game_ai::SmallActionSkill | game-ai\src\small_action\cast.rs:119 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 20 | new | game_ai::SmallActionRecall::new | pub | fn(&game_core::OperationData, &game_core::PlayerState, usize) -> game_ai::SmallActionRecall | game-ai\src\small_action\move_actions.rs:659 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 21 | new | game_ai::SmallActionAttack::new | pub | fn(&game_core::OperationData, usize) -> game_ai::SmallActionAttack | game-ai\src\small_action\cast.rs:23 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 22 | new | game_ai::SmallActionAroundPosition::new | pub | fn(&mut rand::rngs::std::StdRng, &game_core::OperationData, u64, u64, usize) -> game_ai::SmallActionAroundPosition | game-ai\src\small_action\around.rs:831 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 23 | new_with_out_line | game_ai::SmallActionAroundPosition::new_with_out_line | pub | fn(&mut rand::rngs::std::StdRng, &game_core::OperationData, u64, u64, usize, game_ai::AroundBushOutlineType) -> game_ai::SmallActionAroundPosition | game-ai\src\small_action\around.rs:834 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 24 | new_with_skill | game_ai::SmallActionRunAway::new_with_skill | pub | fn(&game_core::OperationData, &game_core::PlayerState, usize, bool) -> game_ai::SmallActionRunAway | game-ai\src\small_action\move_actions.rs:29 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 25 | new_with_target | game_ai::SmallActionAroundBush::new_with_target | pub | fn(&game_core::OperationData, &game_core::Entity, usize, game_ai::AroundBushOutlineType) -> game_ai::SmallActionAroundBush | game-ai\src\small_action\around.rs:1178 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 26 | radius | game_core::Entity::radius | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1509 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 27 | range | game_core::Effect::range | pub | fn(&game_core::Effect, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:25 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 28 | range_adjust | game_core::Effect::range_adjust | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:29 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 29 | skill2_effect | game_core::Entity::skill2_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1692 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 30 | steal_damage_entry_pos | game_ai::plan_legacy::steal::steal_damage_entry_pos | pub | fn(&game_core::GameContext, &game_core::Entity, &game_core::Entity, usize) -> (u64, u64) | game-ai\src\plan_legacy\steal.rs:146 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 31 | steal_wait_bush | game_ai::plan_legacy::steal::steal_wait_bush | pub | fn(game_core::StealTarget, usize, &game_core::Entity, &game_core::MapDef) -> usize | game-ai\src\plan_legacy\steal.rs:27 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 32 | ult_effect | game_core::Entity::ult_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1700 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
</details>

⚠**미매칭 3개**: `first`, `push  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 6개는 **전부 다른 함수**라 싣지 않는다`, `reserve_internal_or_panic`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m12.ll:35276) · **형제 9개** (StealSubPlan)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::plan_legacy::sub_plan::StealSubPlan as std::clone::Clone>::clone | pub | game-ai\src\plan_legacy\sub_plan\steal.rs:12 | True | fn(&game_ai::plan_legacy::sub_plan::StealSubPlan) -> game_ai::plan_legacy::sub_plan::StealSubPlan |
| 1 | <game_ai::plan_legacy::sub_plan::StealSubPlan as std::fmt::Debug>::fmt | pub | game-ai\src\plan_legacy\sub_plan\steal.rs:12 | True | fn(&game_ai::plan_legacy::sub_plan::StealSubPlan, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 2 | <game_ai::plan_legacy::sub_plan::StealSubPlan as std::default::Default>::default | pub | game-ai\src\plan_legacy\sub_plan\steal.rs:12 | True | fn() -> game_ai::plan_legacy::sub_plan::StealSubPlan |
| 3 | game_ai::plan_legacy::sub_plan::StealSubPlan::new | pub | game-ai\src\plan_legacy\sub_plan\steal.rs:21 | True | fn(game_core::StealTarget, bool) -> game_ai::plan_legacy::sub_plan::StealSubPlan |
| 4 | game_ai::plan_legacy::sub_plan::StealSubPlan::get_target | in:game_ai::plan_legacy::sub_plan::steal | game-ai\src\plan_legacy\sub_plan\steal.rs:25 | False | fn(&game_ai::plan_legacy::sub_plan::StealSubPlan, &game_core::OperationData) -> std::option::Option<&game_core::Entity> |
| 5 | game_ai::plan_legacy::sub_plan::StealSubPlan::action_candidates | pub | game-ai\src\plan_legacy\sub_plan\steal.rs:33 | False | fn(&mut game_ai::plan_legacy::sub_plan::StealSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 6 | game_ai::plan_legacy::sub_plan::StealSubPlan::calculate_score_parameter_value | pub | game-ai\src\plan_legacy\sub_plan\steal.rs:122 | True | fn(&game_ai::plan_legacy::sub_plan::StealSubPlan, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::ScoreParameter) |
| 7 | game_ai::plan_legacy::sub_plan::StealSubPlan::score | pub | game-ai\src\plan_legacy\sub_plan\steal.rs:136 | False | fn(&game_ai::plan_legacy::sub_plan::StealSubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 |
| 8 | game_ai::plan_legacy::sub_plan::StealSubPlan::merge | pub | game-ai\src\plan_legacy\sub_plan\steal.rs:154 | True | fn(&mut game_ai::plan_legacy::sub_plan::StealSubPlan, game_ai::plan_legacy::sub_plan::StealSubPlan) |

**`open` 6건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | L60/71/82/93 의 `move_speed*30` 에서 30 의 단위(틱 가정 · 60tps 면 0.5초) — 소스 주석 부재 · 추정 | 5 |  |
| 1 | 미탐색 | AroundPosition(new / new_with_out_line)·AroundBush(new_with_target) 원소의 live 바이트 — 생성자 define 에 initializes 속성이 없어 레이아웃 기반 추정(미확정). sweep 대조 시 갈리면 이 세 variant 의 undef 슬롯부터 의심 | 5 |  |
| 2 | 미탐색 | L111 `if let Some(t) = self.target` 의 None 가지(빈 res 반환, 27044→27312) — L41 해석을 통과했으므로 도달 불가로 판단하나 reach.txt(version=2 gamemode=0) 는 상수 접힘이 아니라 모든 블록을 live 로 둔다 · 미확정 아님, 논리상 사장 | 4 |  |
| 3 | 미탐색 | Lurk 의 `is_visible_from` 비-Player 팀 판(%525~, 시야 검사 없이 거리만) 은 챔피언의 team 이 항상 Player 라 실전 도달 불가 — 컴파일러가 Team 판별자 분기를 특수화한 것(entity.rs:1481~1483) | 4 |  |
| 4 | 미탐색 | steal_wait_bush / steal_damage_entry_pos 내부(m07.ll:53908 / 55270) — 본 배치 범위 밖 · 시그니처·반환 의미만 | 4 |  |
| 5 | 표기 불가 | Effect::range(caster) 가 정확히 `range + stat_buff_cached.range + growth_range*(level-1)` 인지 덧셈 순서 — IR 재결합으로 순서 표기 불가(합은 확정) | 4 |  |

**`notes` 1건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | steal.rs:27/28/30 헬퍼(get_game_mode → as_moba → jungle_runner.{epic,serpen}.live_list.first → get_entity_by_id)의 정확한 함수 이름 — 전부 인라인(define 없음) · inlinedAt 줄만 확정. _docs 415/416 의 「에픽/세르펜 스틸 접근 경유지」 주석은 이 헬퍼가 아니라 steal.rs 의 다른 항목(상수/함수)일 수 있어 대응 미확정 | 4 | 사실 서술 |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

