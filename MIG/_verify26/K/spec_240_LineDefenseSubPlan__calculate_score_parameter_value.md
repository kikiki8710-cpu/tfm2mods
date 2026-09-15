---

### `240` LineDefenseSubPlan::calculate_score_parameter_value — 라인수비 서브플랜의 ScoreParameter 가중치 세팅: 자기 attack/util_value 를 라인 스타일(공격/수비)과 HP% 로 30/50/70 중 택일, 아군 전원 50, 적 전원 50(수비)/100(공격)

| 항목 | 값 |
|---|---|
| id | `line_defense__LineDefense__calculate_score_parameter_value` |
| 심볼 | `_RNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy8sub_plan12line_defenseNtB2_18LineDefenseSubPlan31calculate_score_parameter_value` |
| 소스 | `game-ai\src\plan_legacy\sub_plan\line_defense.rs:396` |
| IR | `m14.ll` 29736~29903행 |
| 경로·가시성 | `game_ai::plan_legacy::sub_plan::LineDefenseSubPlan::calculate_score_parameter_value` · **pub** |
| 계층 | 기타 |
| exe | `e8aeb0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r17` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_ai::plan_legacy::sub_plan::LineDefenseSubPlan, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::ScoreParameter)
```

**TLS 메모 절(`sig.tls` · 정정 경로 `/specs[240]/sig/tls/<키>`)**

없음 — 본문에 `@anon.* = constant ptr @<KEY…call_once>` 참조 0 · LocalKey::with 0 · llvm.threadlocal.address 0

<details><summary>인자 5개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | self | &LineDefenseSubPlan (3B: +0 style LineStyle · +1 line LineType · +2 minion_action_type) | IR `readonly dereferenceable(3)` · 본문은 +0x0 style 1B 만 읽음(m14.ll:29746) | 4 |
| 1 | 2 | _rnd | &mut StdRng (320B) | IR `readnone` — 소스상 &mut(tcx sig `&mut StdRng`) 이나 본문에서 전혀 안 씀 · gen_range 사이트 0 | 3 |
| 2 | 3 | player | &PlayerState (2528B) | IR `readonly` | 4 |
| 3 | 4 | data | &OperationData (24B) | IR `readonly` · +0x0 cache 만 읽음 | 4 |
| 4 | 5 | parameter | &mut ScoreParameter (5384B) | IR 속성: `noalias noundef align 8 captures(none) dereferenceable(5384)` — readonly/readnone/initializes 없음 = &mut · 쓰기 표면은 writes 전수(자기 필드 2 + 힙 원소 2×N) | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn calculate_score_parameter_value(&self, _rnd, player, data, parameter: &mut ScoreParameter)   [line_defense.rs:396]
  line_style: bool = (self.style == LineStyle::Defensive)        // 태그 1 → true          [L397 · m14.ll:29746~29747]
  champ = data.cache.player_champion[player.info.team][player.info.position].unwrap()      [L399 · 29749~29770 · team<2 체크]
  hp_ratio = champ.hp * 100 / champ.stat_cached.hp                 // usize udiv · max_hp==0 이면 패닉   [L400 · 29774~29787]
  value = if line_style /*Defensive*/ { if hp_ratio > 50 { 50 } else { 70 } }
          else          /*Aggressive*/ { if hp_ratio < 50 { 50 } else { 30 } }             [L401 · 29789~29793]
  parameter.player.attack_value = value                                                    [L418 · 29795~29796]
  parameter.player.util_value   = value                                                    [L419 · 29797~29798]
  for p in parameter.near_allies.iter_mut()  { p.attack_value = 50; p.util_value = 50; }   [L421~423 · 29801~29847]
  for p in parameter.near_enemies.iter_mut() { let v = if line_style { 50 } else { 100 }; p.attack_value = v; p.util_value = v; }   [L426, 435~436 · 29852~29899]
  return                                                                                    [L438 · 29902]

해석: 수비 스타일은 HP 가 절반 이하로 떨어졌을 때 자기 가치를 70 으로 올리고(더 조심/보호) 적 가치는 50 으로 낮춰 본다 · 공격 스타일은 HP 가 절반 이상이면 자기 가치 30 (희생 허용) 이고 적 가치는 100 으로 본다. L402~417 · 424~425 · 427~434 는 IR 에 흔적 없음(빈 줄/주석/닫는 괄호 추정 — 미확인). gen_range 사이트 0.
```

**`mem` 메모리 접근 17건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | LineDefenseSubPlan | 0x0 | style (LineStyle 1B · 태그 0=Aggressive · 1=Defensive) | r | m14.ll:29746~29747 `trunc nuw i8→i1` → DI 변수 `line_style` = (style==Defensive) [L397] | 4 | OK |  |
| 1 | PlayerState | 0x930 | info.team | r | m14.ll:29749~29752 · `<2` 바운드체크 → panic_bounds_check(29755) [L399] | 4 | OK |  |
| 2 | PlayerState | 0x9c0 | info.position@tag (i32) | r | m14.ll:29760~29762 zext nneg · player.rs:581 인라인 · 2차 인덱스(바운드체크 없음 — 열거형 태그 0..4 보장) [L399] | 4 | OK |  |
| 3 | OperationData | 0x0 | cache (&AbstractGameWithCache) | r | m14.ll:29763 | 4 | OK |  |
| 4 | AbstractGameWithCache | 0x1e0 | player_champion[team][pos] (Option<&Entity> · null=None) | r | m14.ll:29764~29770 · [2][5] · None → unwrap_failed(29780) [L399] | 4 | OK |  |
| 5 | Entity | 0x628 | stat_cached.hp (최대 HP) | r | m14.ll:29774~29777 · ==0 이면 panic_const_div_by_zero(29828) [L400] | 4 | OK |  |
| 6 | Entity | 0x670 | hp (현재 HP) | r | m14.ll:29784~29787 · hp_ratio = hp*100 / stat_cached.hp (udiv · 정수 내림) [L400] | 4 | OK |  |
| 7 | ScoreParameter | 0x14b8 | near_allies.buf.ptr | r | m14.ll:29801~29802 (bumpalo Vec · Deref → slice) [L421] | 4 | OK |  |
| 8 | ScoreParameter | 0x14d0 | near_allies.len | r | m14.ll:29804~29805 · 원소 ChampionScoreParameter 216B stride(29814 `mul 216`) [L421] | 4 | OK |  |
| 9 | ScoreParameter | 0x14d8 | near_enemies.buf.ptr | r | m14.ll:29852~29853 [L426] | 4 | OK |  |
| 10 | ScoreParameter | 0x14f0 | near_enemies.len | r | m14.ll:29855~29856 · stride 216(29865) [L426] | 4 | OK |  |
| 11 | ScoreParameter | 0x9c0 | player.attack_value (i64) | w | m14.ll:29795~29796 [L418] · value = Defensive ? (hp_ratio>50 ? 50 : 70) : (hp_ratio<50 ? 50 : 30) | 4 | OK | value ∈ {30,50,70} |
| 12 | ScoreParameter | 0x9c8 | player.util_value (i64) | w | m14.ll:29797~29798 [L419] | 4 | OK | value (attack_value 와 동일) |
| 13 | ChampionScoreParameter (near_allies[i] 힙 원소 · i=0..len) | 0xa8 | attack_value | w | m14.ll:29836~29837 [L422] · for p in parameter.near_allies.iter_mut() 루프(29831~29847) | 4 | OK | 50 |
| 14 | ChampionScoreParameter (near_allies[i] 힙 원소) | 0xb0 | util_value | w | m14.ll:29838~29839 [L423] | 4 | OK | 50 |
| 15 | ChampionScoreParameter (near_enemies[i] 힙 원소 · i=0..len) | 0xa8 | attack_value | w | m14.ll:29879 select · 29888~29889 [L435] · 루프 29882~29899 (len==0 이면 select 자체가 스킵됨 29875~29876) | 4 | OK | Defensive ? 50 : 100 |
| 16 | ChampionScoreParameter (near_enemies[i] 힙 원소) | 0xb0 | util_value | w | m14.ll:29890~29891 [L436] | 4 | OK | Defensive ? 50 : 100 (attack_value 와 동일) |

**`consts` 상수 9건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 2 | 399 | 길이 | player_champion 1차 길이 바운드체크 team<2 (m14.ll:29751) | 4 |
| 1 | 100 | 400 | 계수 | hp_ratio = hp*100/max_hp — 백분율 환산 (m14.ll:29786) | 4 |
| 2 | 50 | 401 | 임계 | HP% 임계 — Defensive 에선 `hp_ratio > 50`(29789), Aggressive 에선 `hp_ratio < 50`(29791) · 동시에 결과값 50 (29790/29792 select 의 참 가지) | 4 |
| 3 | 70 | 401 | 산출값 | Defensive 이고 hp_ratio<=50 일 때 자기 attack/util_value (m14.ll:29790 select 거짓 가지) | 4 |
| 4 | 30 | 401 | 산출값 | Aggressive 이고 hp_ratio>=50 일 때 자기 attack/util_value (m14.ll:29792 select 거짓 가지) | 4 |
| 5 | 50 | 422 | 임계 | near_allies 전원 attack_value=50 (m14.ll:29837) · L423 util_value=50 (29839) | 4 |
| 6 | 50 | 435 | 임계 | near_enemies 전원 attack/util_value — Defensive 이면 50 (m14.ll:29879 select 참 가지) | 4 |
| 7 | 100 | 435 | 계수 | near_enemies 전원 attack/util_value — Aggressive 이면 100 (m14.ll:29879 select 거짓 가지) | 4 |
| 8 | 0 | 400 | 태그 | stat_cached.hp == 0 → 0 나누기 패닉 가드 (m14.ll:29776) · L421/L426 len==0 루프 스킵(29824, 29875) | 4 |

**`knobs` 조정점 6건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | HP% 임계(자기 가치 전환점) | line_defense.rs:401 | 50 | 올리면 Defensive 는 더 높은 HP 에서도 70(보호) 으로 · Aggressive 는 더 높은 HP 까지 50 으로(30 희생 구간이 줄어듦) | 4 | 기존 |
| 1 | Defensive 저HP 자기 가치 | line_defense.rs:401 | 70 | 올리면 수비 라인전에서 HP 낮을 때 자기 피해 회피가 더 강해진다(attack/util_value 는 ScoreParameter 소비처에서 자기 손실 가중치) | 4 | 기존 |
| 2 | Aggressive 고HP 자기 가치 | line_defense.rs:401 | 30 | 내리면 공격 라인전에서 HP 넉넉할 때 자기 희생을 더 감수한다 | 4 | 기존 |
| 3 | 아군 가치 | line_defense.rs:422~423 | 50 | 올리면 근처 아군 보호/지원 비중이 커진다 | 4 | 기존 |
| 4 | 적 가치(Aggressive) | line_defense.rs:435 | 100 | 올리면 공격 라인전에서 적 챔피언에게 주는 피해/CC 의 가치가 커져 교전 지향 | 4 | 기존 |
| 5 | 적 가치(Defensive) | line_defense.rs:435 | 50 | 내리면 수비 라인전에서 적 챔피언 공격 가치가 더 낮아져 소극적 | 4 | 기존 |

<details><summary>`callees` 피호출자 3건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | calculate_score_parameter_value | game_ai::plan_legacy::sub_plan::SubPlan::calculate_score_parameter_value | pub | fn(&mut game_ai::plan_legacy::sub_plan::SubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::ScoreParameter) | game-ai\src\plan_legacy\sub_plan\mod.rs:96 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 18개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 1 | calculate_score_parameter_value | game_ai::plan_legacy::sub_plan::HideSubPlan::calculate_score_parameter_value | pub | fn(&game_ai::plan_legacy::sub_plan::HideSubPlan, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::ScoreParameter) | game-ai\src\plan_legacy\sub_plan\hide.rs:221 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 18개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 2 | calculate_score_parameter_value | game_ai::plan_legacy::sub_plan::StealSubPlan::calculate_score_parameter_value | pub | fn(&game_ai::plan_legacy::sub_plan::StealSubPlan, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::ScoreParameter) | game-ai\src\plan_legacy\sub_plan\steal.rs:122 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 18개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
</details>

**호출처 1곳** (m12.ll:35438) · **형제 17개** (LineDefenseSubPlan)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::plan_legacy::sub_plan::LineDefenseSubPlan as std::clone::Clone>::clone | pub | game-ai\src\plan_legacy\sub_plan\line_defense.rs:7 | True | fn(&game_ai::plan_legacy::sub_plan::LineDefenseSubPlan) -> game_ai::plan_legacy::sub_plan::LineDefenseSubPlan |
| 1 | <game_ai::plan_legacy::sub_plan::LineDefenseSubPlan as std::fmt::Debug>::fmt | pub | game-ai\src\plan_legacy\sub_plan\line_defense.rs:7 | True | fn(&game_ai::plan_legacy::sub_plan::LineDefenseSubPlan, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 2 | game_ai::plan_legacy::sub_plan::LineDefenseSubPlan::new | pub | game-ai\src\plan_legacy\sub_plan\line_defense.rs:15 | True | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, game_core::LineType, game_ai::MinionActionType, game_core::LineStyle) -> game_ai::plan_legacy::sub_plan::LineDefenseSubPlan |
| 3 | game_ai::plan_legacy::sub_plan::LineDefenseSubPlan::new_for_gank | pub | game-ai\src\plan_legacy\sub_plan\line_defense.rs:20 | True | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, game_core::LineType, game_ai::MinionActionType, game_core::LineStyle) -> game_ai::plan_legacy::sub_plan::LineDefenseSubPlan |
| 4 | game_ai::plan_legacy::sub_plan::LineDefenseSubPlan::action_effect | in:game_ai::plan_legacy::sub_plan::line_defense | game-ai\src\plan_legacy\sub_plan\line_defense.rs:24 | False | fn(&game_core::Entity, game_core::SmallAction) -> std::option::Option<&game_core::Effect> |
| 5 | game_ai::plan_legacy::sub_plan::LineDefenseSubPlan::action_is_currently_in_range | in:game_ai::plan_legacy::sub_plan::line_defense | game-ai\src\plan_legacy\sub_plan\line_defense.rs:34 | False | fn(&game_core::Entity, game_core::SmallAction, &game_core::Entity) -> bool |
| 6 | game_ai::plan_legacy::sub_plan::LineDefenseSubPlan::action_walkup_position | in:game_ai::plan_legacy::sub_plan::line_defense | game-ai\src\plan_legacy\sub_plan\line_defense.rs:38 | False | fn(&game_ai::plan_legacy::sub_plan::LineDefenseSubPlan, &game_core::Entity, &game_core::Entity, game_core::SmallAction, &game_core::Effect, &game_core::OperationData) -> std::option::Option<(u64, u64)> |
| 7 | game_ai::plan_legacy::sub_plan::LineDefenseSubPlan::positioning_score_at | in:game_ai::plan_legacy::sub_plan::line_defense | game-ai\src\plan_legacy\sub_plan\line_defense.rs:68 | False | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, u64, u64) -> game_core::PositioningScore |
| 8 | game_ai::plan_legacy::sub_plan::LineDefenseSubPlan::unsafe_v19_non_champion_walkup | in:game_ai::plan_legacy::sub_plan::line_defense | game-ai\src\plan_legacy\sub_plan\line_defense.rs:72 | False | fn(&game_ai::plan_legacy::sub_plan::LineDefenseSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_ai::SmallActionPlay, bool, &mut game_core::DebugFrameData) -> bool |
| 9 | game_ai::plan_legacy::sub_plan::LineDefenseSubPlan::base_positioning | in:game_ai::plan_legacy::sub_plan::line_defense | game-ai\src\plan_legacy\sub_plan\line_defense.rs:154 | False | fn(&game_ai::plan_legacy::sub_plan::LineDefenseSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, std::option::Option<&game_ai::MinionWaveSnapshot>) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 10 | game_ai::plan_legacy::sub_plan::LineDefenseSubPlan::get_move_action_v46 | in:game_ai::plan_legacy::sub_plan::line_defense | game-ai\src\plan_legacy\sub_plan\line_defense.rs:200 | False | fn(&game_ai::plan_legacy::sub_plan::LineDefenseSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, std::option::Option<&game_ai::MinionWaveSnapshot>, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 11 | game_ai::plan_legacy::sub_plan::LineDefenseSubPlan::attack_minion_action | in:game_ai::plan_legacy::sub_plan::line_defense | game-ai\src\plan_legacy\sub_plan\line_defense.rs:368 | False | fn(&mut game_ai::plan_legacy::sub_plan::LineDefenseSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 12 | game_ai::plan_legacy::sub_plan::LineDefenseSubPlan::attack_tower_action | in:game_ai::plan_legacy::sub_plan::line_defense | game-ai\src\plan_legacy\sub_plan\line_defense.rs:372 | False | fn(&mut game_ai::plan_legacy::sub_plan::LineDefenseSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> std::option::Option<game_ai::SmallActionPlay> |
| 13 | game_ai::plan_legacy::sub_plan::LineDefenseSubPlan::calculate_score_parameter_value | pub | game-ai\src\plan_legacy\sub_plan\line_defense.rs:396 | False | fn(&game_ai::plan_legacy::sub_plan::LineDefenseSubPlan, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::ScoreParameter) |
| 14 | game_ai::plan_legacy::sub_plan::LineDefenseSubPlan::action_candidates | pub | game-ai\src\plan_legacy\sub_plan\line_defense.rs:440 | False | fn(&mut game_ai::plan_legacy::sub_plan::LineDefenseSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 15 | game_ai::plan_legacy::sub_plan::LineDefenseSubPlan::action_candidates_old | in:game_ai::plan_legacy::sub_plan::line_defense | game-ai\src\plan_legacy\sub_plan\line_defense.rs:881 | False | fn(&mut game_ai::plan_legacy::sub_plan::LineDefenseSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 16 | game_ai::plan_legacy::sub_plan::LineDefenseSubPlan::score | pub | game-ai\src\plan_legacy\sub_plan\line_defense.rs:940 | False | fn(&game_ai::plan_legacy::sub_plan::LineDefenseSubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 |

**`open` 5건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 표기 불가 | L397 소스가 `self.style == LineStyle::Defensive` 인지 `matches!(self.style, Defensive)`/`!= Aggressive` 인지 — 표기 불가(2-variant 외연 동일 · i1 = 태그 1) | 4 |  |
| 1 | 표기 불가 | L401 `>50`/`<50` 이 소스에서 `>= 51`/`<= 49` 였을 가능성 — 표기 불가(정수 외연 동일) | 4 |  |
| 2 | 미탐색 | L402~L417 · L424~425 · L427~434 의 정체 — IR 에 !DILocation 없음(빈 줄/주석/블록 괄호 추정) · rmeta_srcmap 미조회 | 3 |  |
| 3 | 미탐색 | 이 값(attack_value/util_value)을 어느 소비처가 어떤 계산으로 쓰는지 — 콜리 밖(action_score/interaction_score 계층 · 본 배치 범위 밖) | 4 |  |
| 4 | 미탐색 | self.line(+1)·minion_action_type(+2) 은 본 함수에서 읽지 않음(다른 메서드용) | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

