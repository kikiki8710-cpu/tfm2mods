---

### `165` action_eval::evaluate_action — v33 통합 행동 평가 — 라인 앵커 컨텍스트에서 Around 계열 소액션만 채점: positional_gain − danger + last_hit_gain 을 Some(score) 로, 그 외는 None(기존 경로 위임)

| 항목 | 값 |
|---|---|
| id | `action_eval__evaluate_action` |
| 심볼 | `_RNvNtNtCshdEBA0ozCnw_7game_ai11plan_legacy11action_eval15evaluate_action` |
| 소스 | `game-ai\src\plan_legacy\action_eval.rs:67` |
| IR | `m11.ll` 53179~54237행 |
| 경로·가시성 | `game_ai::plan_legacy::action_eval::evaluate_action` · **pub** |
| 계층 | 기타 |
| exe | `e29b40` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r14` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &game_ai::plan_legacy::action_eval::ActionContext, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> std::option::Option<i64>
```

<details><summary>인자 8개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | version | usize | 본문 분기 없음 — position_eval_at 첫 인자로만(53313). evaluation_position 에는 poison(미사용 확인) | 4 |
| 1 | 2 | ctx | &ActionContext(40B) | +0x10 anchor 태그(1=Lane 만 진행) · +0x11 anchor@Lane.line. priority(+0x0) 는 읽지 않음 | 4 |
| 2 | 3 | parameter | &ScoreParameter(5384B) | +0x0 wave_snapshot 태그 · +0x8 wave_snapshot@Some.0 · +0x918 player · +0x998 player.risk_damage · +0x9b0 player.risk_possible_tower | 4 |
| 3 | 4 | rnd | &mut StdRng(320B) | IR 속성 readnone — 본문 미사용(콜리에도 전달 안 함) | 4 |
| 4 | 5 | player | &PlayerState(2528B) | +0x930 team · +0x9c0 position 태그 · (aux) +0x928 info.id 캐시 키 | 4 |
| 5 | 6 | data | &OperationData(24B) | +0x0 cache · +0x8 context | 4 |
| 6 | 7 | action | &SmallActionPlay(184B) | +0xb1 태그 (Around·AroundHide·AroundRegion·AroundPosition·AroundPositionBush·AroundBush·LaneMinionPosition 만) · evaluation_position 에 통째 전달 | 4 |
| 7 | 8 | debug | &mut DebugFrameData(224B) | IR 속성 readnone — 본문 미사용 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
L77: let Anchor::Lane{line} = ctx.anchor (태그 1) else → None
L78: if action ∉ {Around(2),AroundHide(3),AroundRegion(4),AroundPosition(7),AroundPositionBush(8),AroundBush(9),LaneMinionPosition(10)} → None   (RunAway·Recall·AroundRunAway·Positioning·Trace·Attack~Stop 은 기존 경로)
L81/L89: return Some(lane_positioning_score(version, player, data, parameter, action, line))

── lane_positioning_score (인라인 L98~130) ──
L98 : champ = player_champion[team][pos].unwrap()  (None 이면 panic)
L99 : purpose = line_phase_position_eval_purpose(player, data)  (i8 0..8)
L100: dest = action.evaluation_position(_, player, data)                     [Option<(x,y)> sret 24B]
L103: dest_score = dest.map(|(x,y)| position_eval_at(version, player, data, x, y, purpose))   [Option<PositioningScore> 56B · 니치 +0x31==2]
L104: hp_value = champion_hp_value(data, parameter, &parameter.player)
L105: hp = champ.hp.max(1)
L107~108: base_damage = dest_score.map(|s| hp * s.risk.max(0) / 100).unwrap_or(parameter.player.risk_damage)
L109~112: possible_damage = parameter.player.possible_risk(data, 9999) + dest_score.map(|s| hp * s.tower_risk.max(0) / 100).unwrap_or(parameter.player.risk_possible_tower) / 3
L113~116: danger = if champ.stat_buff_cached.undying { 0 } else { base_damage*hp_value/hp + possible_damage*hp_value/hp }   (sdiv 각각)
L124~125: positional_gain = dest_score.map(|s| (s.gain − s.gain_me).max(0) * hp_value / 200).unwrap_or(0)
L127: (ax,ay) = dest.unwrap_or((champ.x, champ.y))
L128: last_hit_gain = lane_anchor_gain(data, player, parameter, ax, ay, line)
L130: return positional_gain − danger + last_hit_gain

── lane_anchor_gain (인라인 L141~259) ──
L141: champ = player_champion[..]? (None→0) ; L142: atk = champ.attack_effect.as_ref()? (None→0)
L143: move_speed = champ.stat_cached.move_speed.max(1) ; L144: start_tick = atk.start_timing*100 / champ.attack_speed_mult().max(1) ; L145: tps = ctx.setting.tick_per_second
L155: (n, entries) = LAG_MEMO.with(|c| … )  [aux: 키 (game.seed(vtable+0x20), game.tick(vtable+0x28), player.info.id, line) 일치 → 캐시 반환 · 불일치 → iter_minions(1-team) 를 visible_from(champ.team)&&Minion&&line 으로 걸러 (tx,ty,attack_range,dmg,tid,bonus) 를 최대 48개 저장 · 48 초과면 n=-1]
  attack_range = champ.stat_buff_cached.range + atk.range + (champ.level−1)*atk.growth_range + atk.range_adjust(champ, target) + radius(champ) + radius(target)   [radius(e) = e.radius_mult==0 ? e.radius : e.radius*(mult+100)/100]
  dmg = atk.expected_damage_target(ctx, champ, target) ; bonus = target.nearest_enemy 가 (아군 팀 && Tower/Nexus) 면 80 아니면 0
L197: if n == -1 (캐시 오버플로) →  L200~231 직접 순회: for target in iter_minions(1-team):
    L201 !target.is_visible_from(&champ.team) → skip ; L204 ty≠Minion → skip ; L207 line≠ → skip
    L210 attack_range(위 식) ; L211 walk_dist = distance(ax,ay,target).saturating_sub(attack_range) ; L212 walk_tick = walk_dist/move_speed ; L213 impact_tick = walk_tick+start_tick ; L214 dmg = expected_damage_target
    L216 s = 0 ; if let Some(snap)=parameter.wave_snapshot && let Some(traj)=snap.find(target.id):
        L217 predicted_hp = traj.hp_at_tick(impact_tick)
        L218 if predicted_hp <= 0            → s = (traj.expected_death_tick > tps*2) ? 0 : 25      (L222)
             elif predicted_hp <= dmg+5     → s = 140
        L220 elif predicted_hp > dmg*2       → s = (death_tick > tps*2) ? 0 : 25                    (L222)
             elif death_tick > impact_tick+tps → s = (death_tick > tps*2) ? 0 : 25                  (L222)
             else                            → s = 70
    L226~227: if target.nearest_enemy = Some(eid) && enemy=get_entity_by_id(eid) && enemy.team == champ.team && enemy.ty ∈ {Tower,Nexus} → s += 80
    L231 best = max(best, s)
  else → L237~256 캐시 순회: for (tx,ty,attack_range,dmg,tid,bonus) in entries[..n]:
    L238~240 walk_dist/walk_tick/impact_tick 동일 ; L243~250 snapshot 있으면 위 s 사다리 동일(find(tid)) 없으면 s=0 ; L254 s += bonus ; L256 best = max(best, s)
L259: return best (기본 0)
```

**`mem` 메모리 접근 48건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | ActionContext | 0x10 | anchor@tag | r | 53199 == 1 Lane 아니면 None | 4 | OK |  |
| 1 | ActionContext | 0x11 | anchor@Lane.line | r | 53205 line (i8 LineType) | 4 | OK |  |
| 2 | SmallActionPlay | 0xb1 | @tag | r | 53209 (gep 177) idx 2,3,4,7,8,9,10 만 진행 | 4 | OK |  |
| 3 | PlayerState | 0x930 | info.team | r | 53256 bounds 2 · 53626 적 팀 1-team | 4 | OK |  |
| 4 | PlayerState | 0x9c0 | info.position@tag | r | 53267 | 4 | OK |  |
| 5 | OperationData | 0x0 | cache | r | 53270 | 4 | OK |  |
| 6 | OperationData | 0x8 | context | r | 53519 → ctx.setting.tick_per_second · expected_damage_target | 4 | OK |  |
| 7 | AbstractGameWithCache | 0x1e0 | player_champion[team][position] | r | 53271~53274 unwrap(None→panic 53295) · 53540 재로드(lane_anchor_gain 인라인 · None→0) | 4 | OK |  |
| 8 | AbstractGameWithCache | 0x0 | game.data_ptr | r | 53982 | 4 | OK |  |
| 9 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r | 53980 · +0x1f0 get_entity_by_id(53989~53991) | 4 | OK |  |
| 10 | GameContext | 0x8 | setting | r | 53582 | 4 | OK |  |
| 11 | GameSetting | 0x12f8 | tick_per_second | r | 53584 (gep 4856) tps | 4 | OK |  |
| 12 | ScoreParameter | 0x918 | player | r | 53355 (gep 2328) → champion_hp_value · possible_risk(…,9999) | 4 | OK |  |
| 13 | ScoreParameter | 0x998 | player.risk_damage | r | 53414 (gep 2456) dest_score None 폴백 base_damage | 4 | OK |  |
| 14 | ScoreParameter | 0x9b0 | player.risk_possible_tower | r | 53427 (gep 2480) dest_score None 폴백 | 4 | OK |  |
| 15 | ScoreParameter | 0x0 | wave_snapshot@tag | r | 53658/54072 trunc→i1 Some | 4 | OK |  |
| 16 | ScoreParameter | 0x8 | wave_snapshot@Some.0 | r | 53660/54075 → MinionWaveSnapshot::find(snap, id) | 4 | OK |  |
| 17 | PositioningScore | 0x0 | risk | r | 53314 → base_damage = hp*risk.max(0)/100 | 4 | OK |  |
| 18 | PositioningScore | 0x8 | tower_risk | r | 53317 → hp*tower_risk.max(0)/100 /3 | 4 | OK |  |
| 19 | PositioningScore | 0x10 | gain | r | 53321 → positional_gain | 4 | OK |  |
| 20 | PositioningScore | 0x18 | gain_me | r | 53325 | 4 | OK |  |
| 21 | PositioningScore | 0x31 | on_periodic_trajectory(=Option 니치) | r | 53329 (gep 49) == 2 ⇔ Option<PositioningScore> None | 4 | OK |  |
| 22 | Entity | 0x670 | hp | r | 53358 champ.hp.max(1) | 4 | OK |  |
| 23 | Entity | 0x488 | stat_buff_cached.undying | r | 53440 (gep 1160) true → danger=0 | 4 | OK |  |
| 24 | Entity | 0x660 | x | r | 53501 champ.x(dest None 폴백) · 53899 target.x | 4 | OK |  |
| 25 | Entity | 0x668 | y | r | 53505 / 53901 | 4 | OK |  |
| 26 | Entity | 0x4c0 | attack_effect@tag | r | 53551 == -1 → last_hit_gain 0 | 4 | OK |  |
| 27 | Entity | 0x490 | attack_effect@Some.0 | r | 53550 atk | 4 | OK |  |
| 28 | Entity | 0x4b0 | attack_effect@Some.0.start_timing | r | 53571 (gep 1200) start_tick = start_timing*100/attack_speed_mult.max(1) | 4 | OK |  |
| 29 | Entity | 0x4a0 | attack_effect@Some.0.range | r | 53652 (gep 1184) | 4 | OK |  |
| 30 | Entity | 0x4a8 | attack_effect@Some.0.growth_range | r | 53653 (gep 1192) × (level-1) | 4 | OK |  |
| 31 | Entity | 0x5c8 | level | r | 53654 (gep 1480) | 4 | OK |  |
| 32 | Entity | 0x438 | stat_buff_cached.range | r | 53655 (gep 1080) | 4 | OK |  |
| 33 | Entity | 0x470 | stat_buff_cached.radius_mult | r | 53656/53874 (gep 1136) i32 — 0 이면 radius 그대로, 아니면 radius*(mult+100)/100 (entity.rs:1511~1515) | 4 | OK |  |
| 34 | Entity | 0x680 | radius | r | 53657/53881 (gep 1664) champ·target 반경 | 4 | OK |  |
| 35 | Entity | 0x640 | stat_cached.move_speed | r | 53565 (gep 1600) .max(1) | 4 | OK |  |
| 36 | Entity | 0x0 | team@tag | r | 53801 champ.team(player_team) · 54017 enemy.team | 4 | OK |  |
| 37 | Entity | 0x8 | team@Player.0 | r | 53808 / 54035~54036 | 4 | OK |  |
| 38 | Entity | 0x38 | visible_state[team]@tag | r | 53815~53818 (gep 56, stride 24) == 0 Visible | 4 | OK |  |
| 39 | Entity | 0x68 | ty@tag | r | 53826 target == 1 Minion · 54041 enemy (tag & 14)==2 ⇔ Tower(2)/Nexus(3) | 4 | OK |  |
| 40 | Entity | 0x11a | ty@Minion.info.line@tag | r | 53840 (gep 282) == line | 4 | OK |  |
| 41 | Entity | 0x88 | ty@Minion.info.nearest_enemy@tag | r | 53939 (gep 136) Some 이면 | 4 | OK |  |
| 42 | Entity | 0x90 | ty@Minion.info.nearest_enemy@Some.0 | r | 53984 (gep 144) eid → get_entity_by_id | 4 | OK |  |
| 43 | Entity | 0x5c0 | id | r | 53930 target.id → snapshot find | 4 | OK |  |
| 44 | MinionHpTrajectory | 0xb8 | expected_death_tick | r | 53956/53973/54142/54149 (gep 184) | 4 | OK |  |
| 45 | LAG_MEMO entries[i] | 0x0..0x30 | (tx,ty,attack_range,dmg,tid,bonus) 48B | r | 54085~54102 캐시 원소(aux 93201~93211 에서 생성) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 46 | (반환 ScalarPair) | - | Option<i64> | w | &mut 인자(rnd·debug)는 readnone — 쓰기 0건. sret 없음 | 4 | 확인불가(오프셋 파싱 실패) | 54233~54235 |
| 47 | thread_local LAG_MEMO(RefCell, 2352B+1) | 0x0 borrow / 0x8 seed / 0x10 tick / 0x18 pid / 0x20 n / 0x28 line / 0x30 entries[48] | 캐시 재생성(aux) | w | aux m00.ll — 키 (game.seed, game.tick, player.info.id, line) 불일치 시만. 관측 캐시(동일 키 동일 값)라 판정 영향 없음 | 4 | 확인불가(tcx 사전에 타입 없음) | 92877~92885 키 · 93010/93230 n · 93201~93211 entries |

**`consts` 상수 21건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev | folded_from |
|---|---|---|---|---|---|---|
| 0 | 1 | 77 | 태그 | Anchor 태그 1 = Lane (53201) · EntityType 1 Minion (53828) · max(…,1) 가드 · level-1 (53850). (같은 리터럴 1 이 shl 시프트량으로도 쓰이나 그건 아래 folded_from=2 행에 분리 — QC 경고는 이 중복 때문) | 4 |  |
| 1 | 1 | 222 | 태그 | tps*2 (=2초) 가 `shl i64 %tps, 1` 로 접힘(53661/54076) · dmg*2 도 `shl %dmg, 1`(53968/54137) | 4 | 2 |
| 2 | 2 | 98 | 센티널 | player_champion bounds (53258) · visible_state bounds (53810) · Option<PositioningScore> 니치 None (53334) · (ty&14)==2 Tower/Nexus (54044) · tps*2 는 shl 1 (53661/54076) | 4 |  |
| 3 | 100 | 107 | 계수 | hp*risk/100 · hp*tower_risk/100 (53383/53405) · start_timing*100/aspd (53577) · radius*(mult+100)/100 (53867~53869) | 4 |  |
| 4 | 9999 | 109 | 임계 | parameter.player.possible_risk(data, 9999) — 상한/확률 인자 (53388/53418) | 4 |  |
| 5 | 3 | 112 | 태그 | possible_damage = possible_risk + tower_risk_dmg/3 (53437) | 4 |  |
| 6 | 200 | 124 | 계수 | positional_gain = (gain−gain_me).max(0)*hp_value/200 (53492) | 4 |  |
| 7 | 0 | 113 | 임계 | undying 이면 danger=0 (53474) · .max(0) 클램프 · s 초기값·bonus 0 | 4 |  |
| 8 | 140 | 218 | 산출값 | 예측 HP 가 0 < hp ≤ dmg+5: 막타 확정 점수 (53937/54156) | 4 |  |
| 9 | 5 | 218 | 태그 | dmg+5 여유 (53963/54132) | 4 |  |
| 10 | 70 | 220 | 산출값 | dmg+5 < hp ≤ dmg*2 이고 expected_death_tick ≤ impact_tick+tps: 근접 막타 (53937/54156) | 4 |  |
| 11 | 25 | 222 | 산출값 | 그 외(hp≤0 · hp>dmg*2 · 죽음 임박)에서 expected_death_tick ≤ tps*2 면 25, 아니면 0 (53959/54152) | 4 |  |
| 12 | 80 | 227 | 미상 | 미니언의 nearest_enemy 가 아군 타워/넥서스면 +80 (54045 · aux 93188) | 4 |  |
| 13 | 14 | 227 | 태그 | ty 태그 & 14 == 2 ⇔ {2 Tower, 3 Nexus} 마스크 (54043) | 4 |  |
| 14 | -1 | 197 | 태그 | 캐시 n == -1 = 48칸 초과 → 캐시 없이 직접 순회(53620) · Option<Effect> None 태그(53553) | 4 |  |
| 15 | 48 | 237 | 미상 | entries 원소 48B stride(54056) · 배열 48칸(54176 slice_index_fail) | 4 |  |
| 16 | 49 | 237 | 임계 | n < 49 bounds(53675) = entries[..n] 슬라이스 상한 48 | 4 |  |
| 17 | 47 | 167 | 미상 | aux 92916: n > 47 이면 n=-1 오버플로 | 4 |  |
| 18 | 9223372036854775807 | 103 | 센티널 | i64::MAX — dest None 경로의 phi 자리값(53340/53341 · %72=true 라 미사용, 컴파일러 아티팩트) | 4 |  |
| 19 | 7 | 78 | 센티널 | 니치 untagged AroundPosition idx (53215) | 4 |  |
| 20 | 10 | 78 | 태그 | 태그 10 부재 assume (53211) | 4 |  |

**`knobs` 조정점 10건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 위험 점수 스케일 | action_eval.rs:107·111 | 100 | hp*risk/100 — 내리면 위치 위험이 더 크게 감점 | 4 | 기존 |
| 1 | 타워 위험 축소 | action_eval.rs:112 | 3 | tower_risk 기여를 1/3 로. 내리면 타워 사거리 위치 감점↑ | 4 | 기존 |
| 2 | possible_risk 인자 | action_eval.rs:109 | 9999 | possible_risk 의 상한/확률 인자(사실상 무제한) — 내부 의미는 possible_risk 명세 | 4 | 기존 |
| 3 | 포지셔닝 이득 스케일 | action_eval.rs:124 | 200 | (gain−gain_me)*hp_value/200 — 내리면 이득 항 가중↑ | 4 | 기존 |
| 4 | 막타 확정 점수 | action_eval.rs:218 | 140 | impact 시점 예측 HP ≤ dmg+5 인 미니언 존재 시. 올리면 막타 위치 선호↑ | 4 | 기존 |
| 5 | 막타 여유 HP | action_eval.rs:218 | 5 | dmg+5 — 올리면 더 넉넉히 막타 확정 판정 | 4 | 기존 |
| 6 | 근접 막타 점수 | action_eval.rs:220 | 70 | dmg < hp ≤ 2·dmg 이고 곧 죽는 미니언 | 4 | 기존 |
| 7 | 죽음 임박 점수 | action_eval.rs:222 | 25 | expected_death_tick ≤ 2초면 25 (아니면 0) | 4 | 기존 |
| 8 | 타워 표적 미니언 보너스 | action_eval.rs:185·227 | 80 | 아군 타워/넥서스가 때리는 미니언 — 올리면 타워 근처 라인 위치 선호↑ | 4 | 기존 |
| 9 | 캐시 용량 | action_eval.rs:167 | 48 | 미니언 48 초과 시 캐시 포기(직접 순회) — 결과 동일, 성능만 | 4 | 기존 |

<details><summary>`callees` 피호출자 26건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | attack_range | game_ai::SmallActionLaneMinionPosition::attack_range | in:game_ai | fn(&game_core::Entity, &game_core::Entity) -> std::option::Option<u64> | game-ai\src\small_action\lane_minion.rs:354 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 1 | attack_speed_mult | game_core::Entity::attack_speed_mult | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:2433 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 2 | champion_hp_value | game_ai::champion_hp_value | pub | fn(&game_core::OperationData, &game_ai::ScoreParameter, &game_ai::ChampionScoreParameter) -> i64 | game-ai\src\utils.rs:909 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 3 | distance | game_core::utils::distance | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:12 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | evaluation_position | game_ai::SmallActionPlay::evaluation_position | pub | fn(&game_ai::SmallActionPlay, usize, &game_core::PlayerState, &game_core::OperationData) -> std::option::Option<(u64, u64)> | game-ai\src\small_action.rs:245 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 5 | expected_damage_target | game_core::Effect::expected_damage_target | pub | fn(&game_core::Effect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + , &game_core::Entity) -> usize | game-core\src\simulation\effect.rs:91 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 6 | find | game_ai::MinionWaveSnapshot::find | pub | fn(&game_ai::MinionWaveSnapshot, usize) -> std::option::Option<&game_ai::MinionHpTrajectory> | game-ai\src\utils.rs:84 | True | True | 3 | IR 호출 심볼 일치(토큰 경계) |
| 7 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 8 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 9 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 10 | hp_at_tick | game_ai::MinionHpTrajectory::hp_at_tick | pub | fn(&game_ai::MinionHpTrajectory, usize) -> i64 | game-ai\src\utils.rs:40 | True | True | 3 | IR 호출 심볼 일치(토큰 경계) |
| 11 | is_visible_from | game_core::Entity::is_visible_from | pub | fn(&game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1481 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 12 | iter_minions | game_core::AbstractGameWithCache::<'a, 'b>::iter_minions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5753 ~ game_core[6a30]::simulation::{impl#5}::iter_minions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1847 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 13 | lane_anchor_gain | game_ai::plan_legacy::action_eval::lane_anchor_gain | in:game_ai::plan_legacy::action_eval | fn(&game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, u64, u64, game_core::LineType) -> i64 | game-ai\src\plan_legacy\action_eval.rs:139 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 14 | lane_positioning_score | game_ai::plan_legacy::action_eval::lane_positioning_score | in:game_ai::plan_legacy::action_eval | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_ai::SmallActionPlay, game_core::LineType) -> i64 | game-ai\src\plan_legacy\action_eval.rs:96 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 15 | line_phase_position_eval_purpose | game_ai::line_phase_position_eval_purpose | pub | fn(&game_core::PlayerState, &game_core::OperationData) -> game_ai::PositionEvalPurpose | game-ai\src\position_eval.rs:47 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 16 | position_eval_at | game_ai::position_eval_at | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, u64, u64, game_ai::PositionEvalPurpose) -> game_core::PositioningScore | game-ai\src\position_eval.rs:291 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 17 | possible_risk | game_ai::ChampionScoreParameter::<'_>::possible_risk | pub | fn(&game_ai::ChampionScoreParameter</#0>, &game_core::OperationData, usize) -> i64 | game-ai\src\score_parameter.rs:1407 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 18 | radius | game_core::Entity::radius | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1509 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 19 | range_adjust | game_core::Effect::range_adjust | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:29 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 20 | seed | game_core::AbstractGame::seed | pub | fn(&Self/#0) -> u64 | game-core\src\simulation.rs:72 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 21 | seed | <game_core::Game as game_core::AbstractGame>::seed | pub | fn(&game_core::Game) -> u64 | game-core\src\simulation\game.rs:1564 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 22 | seed | <game_core::SingleLaneGame as game_core::AbstractGame>::seed | pub | fn(&game_core::SingleLaneGame) -> u64 | game-core\src\simulation\game.rs:3781 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 23 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 24 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 25 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
</details>

⚠**미매칭 4개**: `anchor`, `best`, `visible_from`, `with  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 2개는 **전부 다른 함수**라 싣지 않는다`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 3곳** (m02.ll:48445, m14.ll:29924, m15.ll:22535) · **형제 0개** 

**`open` 6건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 표기 불가 | L218~222 s 사다리의 소스 형태(if/else if 순서·`>` vs `>=`)는 IR 분기 방향으로 확정했으나 소스 표기(==/>= 외연 동일)는 표기 불가. 특히 hp<=0 경로와 hp>dmg*2 경로가 같은 L222 식으로 합류(phi 53937) | 4 |  |
| 1 | 미탐색 | expected_death_tick 을 `tps*2`(절대 2초?)와 비교하는 L222 의 의미 — 필드가 절대 틱인지 상대 틱인지는 MinionHpTrajectory 생성부(미탐색) 소관 | 4 |  |
| 2 | 미탐색 | position_eval_at(m07.ll:24507 · sret 56B PositioningScore)·line_phase_position_eval_purpose(m07.ll:37719 · i8 0..8)·possible_risk·champion_hp_value·MinionWaveSnapshot::find(m11.ll:46561 · (snap 2320B, id) → *const MinionHpTrajectory or null)·hp_at_tick(m11.ll:49805 · (traj, tick) → i64) 내부 미탐색(계약만) | 4 |  |
| 3 | 미탐색 | iter_minions(cache, team) sret 56B 이터레이터(슬라이스 3개 Chain 형태 · 53628~53647)의 정체(어느 3 집합인지)는 game_core 경계 | 4 |  |
| 4 | 미탐색 | aux closure#0 의 entries 생성 필터(visible_from·Minion·line)와 range/dmg/bonus 식이 직접 순회 경로(L200~231)와 동일함을 IR 로 확인(93070~93211 ↔ 53847~54046) — 단 캐시 경로는 tower bonus 를 캐시 생성 시점 값으로 쓰므로 같은 틱 안에서만 동치 | 4 |  |
| 5 | 미탐색 | m08.ll:15643~17849 `<lane_ancho8>` 는 LAG_MEMO 스레드로컬 lazy-init(std internal get_or_init_slow) — 판정 무관이라 aux 미등록 | 4 |  |

**`notes` 1건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | L127 dest.unwrap_or((champ.x,champ.y)) 의 select(53513~53514)은 dest 태그(%48) 기준 — 확정 | 4 | 사실 서술 |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

