# 배치 A — 실변경 6 (0.5.8 RVA → 0.6.0 RVA · exe 정렬 힌트 · 0.5.8 명세 요지)

## `cc5fc0` → `ea8450` RecallSubPlan::score
- 명령 101→108 · 정렬 101 · exe 판정 ❌구조 변경 · 블록이동 9 · 콜리 주의 1
- 구조 차이: 구 0명령 / 신 7명령 짝 없음
- 분기 차이: cc6027 → cc6127/ea850f ; cc60d6 → cc611a/ea8519
- 소형 즉치: 0xcc6014:0x7→0x6 · 0xcc606a:0xb→0xa · 0xcc609b:0xd→0xc
- 즉치: 0x7→0x6 · 0x1ffdc→0x13 · 0xb→0xa · 0xd→0xc
- 변위: rdi+0x930→0xa00 · rdi+0x9c0→0xa90 · rdx+-0xf→-0xe · rdx+0x80→0xa0 · rax+0x80→0xa0
- 콜리 주의: d57540→f72620 변경
- 0.5.8 명세 #237 `recall__Recall__score` src game-ai\src\plan_legacy\sub_plan\recall.rs:42 · one_line: 귀환 서브플랜 액션 점수: interaction_score 를 기저로 도주/귀환류 +50, 그 외는 양수면 CC 스킬만 +50·나머지 /3, 음수면 ×3
- 0.5.8 logic(앞 1800자):
```
fn score(&self, version, parameter, rnd, player, data, action, debug) -> i64   [recall.rs:42]
  base = action_score::interaction_score(version, rnd, player, data, parameter, action, debug)   [L43 · m02.ll:40326]
  idx = 논리 variant idx(action 태그 +0xb1)                                                        [L46 · 40329~40354]
  match action {
    RunAway(0) | Recall(1) | AroundRunAway(5) => return base + 50                                  [L47 · 40364]
    _ => {                                                                                          [L48]
      if base > 0 {                                                                                 [L48 · 40360 sgt]
        champ = data.cache.player_champion[player.info.team][player.info.position].unwrap()         [L49 · 40372~40393 · team<2 바운드체크]
        match action {
          Skill(13)  => if champ.skill_effect.as_ref().unwrap().ty.expected_cc_time().is_some()  { base + 50 } else { base / 3 }   [L51 · 40439~40463 → 40471 / 40500]
          Skill2(14) => if champ.skill2_effect()/*level>2 ? &skill2_effect : &None*/.as_ref().unwrap().ty.expected_cc_time().is_some() { base + 50 } else { base / 3 }   [L52 · 40422~40431, 40478~40492]
          _ (Around·AroundHide·AroundRegion·Positioning·AroundPosition·AroundPositionBush·AroundBush·LaneMinionPosition·Trace·Attack·Ult·Stop) => base / 3   [L55 · 40500]
        }
      } else { base * 3 }                                                                           [L58 · 40368]
    }
  }                                                                                                 [L60 ret · 40505]

주의: Ult(15)·Attack(12) 은 CC 검사 없이 base/3. Skill2 가지는 champ.level<=2 이면 정적 None → unwrap 패닉 경로(40496 · 실전에선 Skill2 후보 자체가 안 생기므로 도달 조건은 콜러 쪽). gen_range 사이트 0. self/paramete
```

## `ccaa10` → `eacee0` EpicPokeSubPlan::score
- 명령 179→181 · 정렬 178 · exe 판정 ❌구조 변경 · 블록이동 5 · 콜리 주의 1
- 구조 차이: 구 1명령(jmp|I…) / 신 3명령(lea|edx, [rcx - I]…) 짝 없음
- 분기 차이: ccab51 switch case 수 8→7(정렬 실패) ; ccabab → ccabc3/ead095
- 소형 즉치: 0xccab46:0x7→0x6 · 0xccab98:0x7→0x6 · 0xccac0e:0x18→0x10 · 0xccac13:0x10→0x8 · 0xccac1a:0x20→0x18 · 0xccac1f:0x18→0x10
- 즉치: 0x7→0x6 · 0x7→0x6 · 0x1ffdc→0x13 · 0x18→0x10 · 0x10→0x8 · 0x20→0x18 · 0x18→0x10 · 0x38→0x20 · 0x30→0x18 · 0x10→0x38
- 변위: rdi+0x930→0xa00 · rdi+0x9c0→0xa90
- 콜리 주의: d57540→f72620 변경
- 0.5.8 명세 #138 `EpicPokeSubPlan__score` src game-ai\src\plan_legacy\sub_plan\epic_poke.rs:521 · one_line: 에픽 포킹 서브플랜 행동 점수 = interaction_score ± (에픽 미가시·도주계열이면 /2) + 에픽 지향 보너스 5
- 0.5.8 logic(앞 1800자):
```
fn score(&self, version, parameter, rnd, player, data, action, debug) -> i64   // epic_poke.rs:521
  let s = interaction_score(version, rnd, player, data, parameter, action, debug);   // L522 (m02.ll:46781)
  let champ = data.cache.player_champion[player.info.team][player.info.position].unwrap();   // L524
  // L525: let epic = match data.cache.game.get_game_mode() { Moba(m) => m.jungle_runner.epic.live_list.first().and_then(|id| game.get_entity_by_id(*id)), _ => unwrap_failed(패닉) }
  //   live_list.len==0 → None (46839) ; 아니면 get_entity_by_id(live_list[0]) (46855, Option<&Entity> — null=None)
  let champ_region = map.regions[min(champ.y/32000,29)][min(champ.x/32000,29)];   // L526 (46866~46885)
  let bonus = if champ_region == 7 /*Epic*/ { 0 }   // L528 — 이미 에픽 지역이면 보너스 없음
    else match action.get_action() {                 // L532 (small_action.rs:309 인라인, 니치 태그→idx)
      SmallAction::Around{target_id} /*Play idx 2 Around·3 AroundHide·10 LaneMinionPosition*/ => {   // L534
         // get_game_mode() 재호출(46950) → Moba 아니면 패닉 ; live_list.first() == Some(&target_id) ? 5 : 0 (47013~47015; len==0 이면 0)
      }
      SmallAction::AroundPosition{x,y} /*Play idx 4 AroundRegion(x@16,y@24)·7 AroundPosition(48,56)·8 AroundPositionBush(8,16)·9 AroundBush(24,32)*/ => {   // L541
         let region = map.regions[min(y/32000,29)][min(x/32000,29)]; if region == 7 {5} else {0}   // L541~542 (46966~46981)
      }
      _ /*RunAway·Recall·AroundRunAway·Positioning·Trace·Attack·Skill·Skill2·Ult·Stop*/ => 0
    };
  // L556: let visible = epic.is_some_and(|e| e.is_visible_from(champ))
  //   is_visible_from(entity.rs:1482) = match champ.player_team() { None(team 태그 Neutral=1, 47033) => true, Some(t) => e.visible_state[t] is Visible(태그 0, data.rs:122, 47050) }
  let objective_score = i
```

## `e81390` → `f64d30` SerpenPokeSubPlan::score
- 명령 184→186 · 정렬 183 · exe 판정 ❌구조 변경 · 블록이동 8 · 콜리 주의 1
- 구조 차이: 구 1명령(jmp|I…) / 신 3명령(lea|ecx, [rax - I]…) 짝 없음
- 분기 차이: e814c3 switch case 수 8→7(정렬 실패) ; e81515 → e8156c/f64ecf ; e8157c → e81517/f64e75
- 소형 즉치: 0xe814b8:0x7→0x6 · 0xe81502:0x7→0x6 · 0xe8158b:0x18→0x10 · 0xe81590:0x10→0x8 · 0xe8159b:0x20→0x18 · 0xe815a0:0x18→0x10
- 즉치: 0x7→0x6 · 0x7→0x6 · 0x1ffdc→0x13 · 0x18→0x10 · 0x10→0x8 · 0x20→0x18 · 0x18→0x10 · 0x38→0x20 · 0x30→0x18 · 0x10→0x38
- 변위: rbx+0x930→0xa00 · rbx+0x9c0→0xa90
- 콜리 주의: d57540→f72620 변경
- 0.5.8 명세 #139 `SerpenPokeSubPlan__score` src game-ai\src\plan_legacy\sub_plan\serpen_poke.rs:508 · one_line: 세르펜 포킹 서브플랜 점수 = 세르펜 없으면 0 · 아니면 interaction_score(미가시·도주계열 /2) + 150k 밖에서 세르펜 지향 보너스 5
- 0.5.8 logic(앞 1800자):
```
fn score(&self, version, parameter, rnd, player, data, action, debug) -> i64   // serpen_poke.rs:508
  let s = interaction_score(version, rnd, player, data, parameter, action, debug);   // L509 (m14.ll:18636) — 세르펜 부재여도 먼저 호출됨
  let champ = data.cache.player_champion[player.info.team][player.info.position].unwrap();   // L511
  // L512: let serpen = match game.get_game_mode() { Moba(m) => m.jungle_runner.serpen.live_list.first().and_then(|id| game.get_entity_by_id(*id)), _ => 패닉 }
  if serpen.is_none() { return 0 }     // L513 (18697 len==0 → 0 ; 18718 entity None → 0)
  // L518: let bonus = if champ.distance_sq(serpen) < 22500000001 { 0 }   // 150000 이내면 보너스 없음 (entity.rs:2158 → utils.rs:9 dx²+dy², 18740~18754)
    else match action.get_action() {    // L522 (small_action.rs:309 인라인)
      SmallAction::Around{target_id} /*Play 2·3·10*/ => { get_game_mode() 재호출(18809, Moba 아니면 패닉); serpen.live_list.first()==Some(&target_id) ? 5 : 0 (18877~18879; len 0 → 0) }   // L524
      SmallAction::AroundPosition{x,y} /*Play 4(16,24)·7(48,56)·8(8,16)·9(24,32)*/ => { map.regions[min(y/32000,29)][min(x/32000,29)] == 7 ? 5 : 0 }   // L531~532
      _ => 0
    };
  // L545: let visible = serpen.is_visible_from(champ) = champ.team Neutral(태그1) ? true : serpen.visible_state[champ.team] is Visible(태그0)
  let objective_score = if visible { s }                                      // 18950 phi: %120(Neutral)·%128(Visible) → s
    else if matches!(action.get_action(), SmallAction::RunAway) /*Play 0 RunAway·1 Recall·5 AroundRunAway*/ { s / 2 }   // L546 (18947)
    else { s };
  objective_score + bonus   // L557~558 (18953)
```

## `ccbb10` → `f3a070` LineSafeSubPlan::score
- 명령 209→211 · 정렬 206 · exe 판정 ❌구조 변경 · 블록이동 16 · 패닉스텁 재배열 2 · switch case 제거 1 · 콜리 주의 3
- 구조 차이: 구 3명령(mov|r8, qword ptr [rsp + I]…) / 신 5명령(mov|rsi, qword ptr [rsp + I]…) 짝 없음
- 분기 차이: ccbc7a → ccbd16/f3a3a3 ; ccbc9c → ccbc41/f3a3af ; ccbcea → ccbc41/f3a3af ; ccbcff → ccbe5c/f3a3af ; ccbd11 → ccbc48/f3a3b1 ; ccbd1b → ccbc43/f3a3b1
- switch: ccbc31 switch 8→7 case #2 제거 · 나머지 7 case 타깃 동일
- 소형 즉치: 0xccbc26:0x7→0x6
- 즉치: 0x7→0x6 · 0x590→0x580 · 0x570→0x590 · 0x580→0x570
- 변위: r14+0x930→0xa00 · r14+0x9c0→0xa90 · r13+0x530→0x4f8 · r13+0x500→0x4c8 · r13+0x4c0→0x530 · r13+0x490→0x500 · r13+0x3fc→0x400 · r13+0x4f8→0x4c0 · r13+0x4c8→0x490 · r13+0x400→0x3fc
- 콜리 주의: d57540→f72620 변경 ; d59940→f749a0 변경 ; e29b40→100c2f0 변경
- 0.5.8 명세 #230 `line_safe__LineSafe__score` src game-ai\src\plan_legacy\sub_plan\line_safe.rs:104 · one_line: LineSafe 서브플랜의 후보 액션 점수: evaluate_action(Lane 우선순위·Lane 앵커) Some 이면 그 값, None 이면 interaction_score+경제보정+액션별 가산(Attack/Skill/Skill2=calculate_action_score, 대상 소실=-99999 · Around 계열은 evaluate_action 이 항상 Some 이라 구경로의 50/100 구조물 가산 arm 은 사장)
- 0.5.8 logic(앞 1800자):
```
fn score(&self, version, parameter, rnd, player, data, action, debug) -> i64   // line_safe.rs:104
// L105: 신경로(액션 평가기) 우선
let ctx = ActionContext { priority: PriorityProfile::Lane /*tag 2*/, anchor: Anchor::Lane { line: self.line } /*+0x10=1, +0x11=line*/ };
if let Some(v) = evaluate_action(version, &ctx, parameter, rnd, player, data, action, debug) {   // 반환 {i64 tag, i64 v}, tag bit0=1 이면 Some
    return v;   // L167
}
// L108: 구경로
let champ = data.cache.player_champion[player.info.team /*+0x930, <2*/][player.info.position as usize /*+0x9c0*/].unwrap();   // None → option::unwrap_failed
let base = interaction_score(version, rnd, player, data, parameter, action, debug);   // L109
let economy_adjustment = line_action_economy_adjustment(version, player, data, parameter, action, MinionActionType::Pull /*i8 0*/);   // L113
// L114: 액션별 가산 — action.get_action() 인라인 → SmallActionPlay 태그(+0xb1) switch
let add = match action.get_action() {
    // idx 12 Attack{target_id=+0x8}  (L118)
    Attack{target_id} => if let Some(t) = data.cache.game.get_entity_by_id(target_id) /*vtable+0x1f0*/ {
            let effect = champ.attack_effect.as_ref().unwrap();   // L119 · +0x4c0 tag≠-1, payload +0x490
            calculate_action_score(version, rnd, player, data, parameter, &champ.attack /*+0x570*/, effect, champ.attack_speed_mult(), t, Pull /*i8 0*/, debug)   // L120
        } else { -99999 },
    // idx 13 Skill{target_id}  (L126)
    Skill{target_id} => if let Some(t) = get_entity_by_id(target_id) {
            let effect = champ.skill_effect.as_ref().unwrap();   // L127 · +0x4f8 / +0x4c8
            calculate_action_score(version, rnd, player, data, parameter, &champ.skill /*+0x580*/, effect, champ.cooldown_reduce(false), t, Pull, debug)   // L128
        } else { -99999 },
    // i
```

## `eb5840` → `eae650` LineWaitSubPlan::score
- 명령 209→211 · 정렬 206 · exe 판정 ❌구조 변경 · 블록이동 16 · 패닉스텁 재배열 2 · switch case 제거 1 · 콜리 주의 3
- 구조 차이: 구 3명령(mov|r8, qword ptr [rsp + I]…) / 신 5명령(mov|rsi, qword ptr [rsp + I]…) 짝 없음
- 분기 차이: eb59aa → eb5a46/eae983 ; eb59cc → eb5971/eae98f ; eb5a1a → eb5971/eae98f ; eb5a2f → eb5b8c/eae98f ; eb5a41 → eb5978/eae991 ; eb5a4b → eb5973/eae991
- switch: eb5961 switch 8→7 case #2 제거 · 나머지 7 case 타깃 동일
- 소형 즉치: 0xeb5956:0x7→0x6
- 즉치: 0x7→0x6 · 0x590→0x580 · 0x570→0x590 · 0x580→0x570
- 변위: r14+0x930→0xa00 · r14+0x9c0→0xa90 · r13+0x530→0x4f8 · r13+0x500→0x4c8 · r13+0x4c0→0x530 · r13+0x490→0x500 · r13+0x3fc→0x400 · r13+0x4f8→0x4c0 · r13+0x4c8→0x490 · r13+0x400→0x3fc
- 콜리 주의: d57540→f72620 변경 ; d59940→f749a0 변경 ; e29b40→100c2f0 변경
- 0.5.8 명세 #231 `line_wait__LineWait__score` src game-ai\src\plan_legacy\sub_plan\line_wait.rs:159 · one_line: LineWait 서브플랜의 후보 액션 점수: evaluate_action(Lane 우선순위·Lane 앵커) Some 이면 그 값, None 이면 interaction_score+경제보정+액션별 가산(Attack/Skill/Skill2=calculate_action_score, 대상 소실=-99999 · Around 계열은 evaluate_action 이 항상 Some 이라 구경로의 50/100 구조물 가산 arm 은 사장)
- 0.5.8 logic(앞 1800자):
```
fn score(&self, version, parameter, rnd, player, data, action, debug) -> i64   // line_wait.rs:159
// L160: 신경로(액션 평가기) 우선
let ctx = ActionContext { priority: PriorityProfile::Lane /*tag 2*/, anchor: Anchor::Lane { line: self.line } /*+0x10=1, +0x11=line*/ };
if let Some(v) = evaluate_action(version, &ctx, parameter, rnd, player, data, action, debug) {   // 반환 {i64 tag, i64 v}, tag bit0=1 이면 Some
    return v;   // L222
}
// L163: 구경로
let champ = data.cache.player_champion[player.info.team /*+0x930, <2*/][player.info.position as usize /*+0x9c0*/].unwrap();   // None → option::unwrap_failed
let base = interaction_score(version, rnd, player, data, parameter, action, debug);   // L164
let economy_adjustment = line_action_economy_adjustment(version, player, data, parameter, action, MinionActionType::Pull /*i8 0*/);   // L168
// L169: 액션별 가산 — action.get_action() 인라인 → SmallActionPlay 태그(+0xb1) switch
let add = match action.get_action() {
    // idx 12 Attack{target_id=+0x8}  (L173)
    Attack{target_id} => if let Some(t) = data.cache.game.get_entity_by_id(target_id) /*vtable+0x1f0*/ {
            let effect = champ.attack_effect.as_ref().unwrap();   // L174 · +0x4c0 tag≠-1, payload +0x490
            calculate_action_score(version, rnd, player, data, parameter, &champ.attack /*+0x570*/, effect, champ.attack_speed_mult(), t, Pull /*i8 0*/, debug)   // L175
        } else { -99999 },
    // idx 13 Skill{target_id}  (L181)
    Skill{target_id} => if let Some(t) = get_entity_by_id(target_id) {
            let effect = champ.skill_effect.as_ref().unwrap();   // L182 · +0x4f8 / +0x4c8
            calculate_action_score(version, rnd, player, data, parameter, &champ.skill /*+0x580*/, effect, champ.cooldown_reduce(false), t, Pull, debug)   // L183
        } else { -99999 },
    // i
```

## `cc3080` → `ea6f40` BattleSubPlan::score
- 명령 948→948 · 정렬 948 · exe 판정 ⚠소형 즉치 변경(7→6·4→3·4→3·4→3·4→3) · 블록이동 200 · 스택슬롯 2 · 패닉스텁 재배열 3 · switch case 제거 1 · 콜리 주의 5
- switch: cc3159 switch 8→7 case #2 제거 · 나머지 7 case 타깃 동일
- 소형 즉치: 0xcc314e:0x7→0x6 · 0xcc33f3:0x4→0x3 · 0xcc3549:0x4→0x3 · 0xcc362a:0x4→0x3 · 0xcc36e1:0x4→0x3 · 0xcc3795:0x4→0x3 · 0xcc3812:0xe→0xd · 0xcc39d5:0x4→0x3 · 0xcc3aa5:0x4→0x3 · 0xcc3b52:0x4→0x3
- 즉치: 0x7→0x6 · 0x4→0x3 · 0x4→0x3 · 0x4→0x3 · 0x4→0x3 · 0x4→0x3 · 0xe→0xd · 0x4→0x3 · 0x4→0x3 · 0x4→0x3
- 변위: r12+0x930→0xa00 · r12+0x9c0→0xa90 · rdx+0x60→0x80 · rdx+0x60→0x80 · rdx+0x88→0xa8 · rdx+0x88→0xa8 · rax+0x928→0x9f8 · rax+0x60→0x80 · rdx+0x88→0xa8 · rdx+0x88→0xa8
- 콜리 주의: 12857f0→1643790 변경 ; d57540→f72620 변경 ; d59940→f749a0 변경 ; d676c0→fe3fd0 변경 ; d69f80→fe6840 변경
- 0.5.8 명세 #214 `battle__Battle__score` src game-ai\src\plan_legacy\sub_plan\battle.rs:616 · one_line: 전투 서브플랜 액션 점수: interaction_score 기저에 액션 종류(도주/추적/평타/스킬/스킬2/궁)별 대상·태세·CC·처치 가능성 보정을 얹어 i64 로 반환(대상 소실·도주중 돌진스킬은 -99999)
- 0.5.8 logic(앞 1800자):
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
            && can_tower_focused(ctx, cache, player, target.x, target.y)                    
```
