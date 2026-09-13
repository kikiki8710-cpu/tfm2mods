---

### `66` evaluate_gank_opportunity_with_score — 정글러 관점 특정 라인의 갱 성공 가능성을 점수화하고 판단력 노이즈를 곱해 required_score 와 비교

| 항목 | 값 |
|---|---|
| id | `passive_jungle__evaluate_gank_opportunity_with_score` |
| 심볼 | `_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old14passive_jungle36evaluate_gank_opportunity_with_score` |
| 소스 | `game-ai\src\plan_legacy\old\passive_jungle.rs:693` |
| IR | `m04.ll` 62981~63740행 |
| 경로·가시성 | `game_ai::plan_legacy::old::evaluate_gank_opportunity_with_score` · **pub** |
| 계층 | 레거시 플랜 |
| exe | `d40f10` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r9` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, game_core::LineType, i32) -> (bool, i32, i32)
```

<details><summary>인자 5개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | rnd | &mut StdRng | error_ratio 의 gen_range 에만 사용 (L800) | 4 |
| 1 | 2 | player | &PlayerState(2528B) | info.team(0x930)·info.position@tag(0x9c0)·stat.judgement(0x218) 사용 | 4 |
| 2 | 3 | data | &OperationData(24B) | cache(+0)·context(+8)·blackboard(+0x10) 전부 사용 | 4 |
| 3 | 4 | line | LineType(u8: 0=Top 1=Mid 2=Bottom) | 갱 대상 라인. 타워 선택(0x180+line*0x20)·is_near_line·in_big_line 에 사용 | 4 |
| 4 | 5 | required_score | i32 | evaluated_score >= required_score 가 반환 bool (L803) | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn evaluate_gank_opportunity_with_score(rnd, player, data, line, required_score) -> (ok, evaluated, actual)
  self = data.cache; team = player.info.team; enemy_team = 1 - team  (L702, bounds<2)
  // L701~705 적 라이너 수집
  line_enemies = self.iter_champions(enemy_team)            // player_champion[enemy_team][0..5] 의 Some 만
      .filter(|e| is_near_line(data.context, e.x, e.y, line)          // L703 (aux m04.ll:66359)
               && data.blackboard[enemy_team].is_recent_visible(game, player, e))  // L704 (aux 66376) — 인덱스가 1-team 임에 주의
      .collect()
  if line_enemies.is_empty() → return (false, 0, 0)          // L708~709
  // L713~716 아군 라이너 수집
  line_allies = (0..5)
      .filter(|pos| data.blackboard[team].in_big_line(pos, line))   // L714: big_goal[pos] == Line{line}
      .filter_map(|pos| self.player_champion[team][pos])           // L715
      .filter(|a| a.hp*100 / a.stat_cached.hp > 40)                // L716 (aux m04.ll:66416)  ※ 자기 자신 제외 없음
      .collect()
  if line_allies.is_empty() → return (false, 0, 0)           // L720
  if line_enemies.len() > line_allies.len() + 1 → return (false, 0, 0)   // L724 (정글러 합류 감안)
  jungler_champ = self.player_champion[team][player.info.position] else return (false,0,0)  // L728
  target_enemy = line_enemies.iter().min_by_key(|e| (|e.x-j.x|)² + (|e.y-j.y|)²)  else return  // L733 (정글러에게 가장 가까운 적)
  nearest_ally = line_allies.iter().min_by_key(|a| dist²(a, target_enemy)) else return           // L738
  // L747~755 적 체력
  ehp = (target_enemy.hp*100 / target_enemy.stat_cached.hp) as i32
  score = if ehp > 79 { -40 } else if ehp > 59 { (80-ehp) >> 1 } else if ehp > 39 { 80-ehp } else { 90-ehp }
  // L759~772 적 타워 거리
  enemy_tower = tower(line)[enemy_team].or(tower2(line)[enemy_team])   // 0x180+line*0x20 / 0x190+line*0x20
  match enemy_tower {
    Some(t) => { d = distance(target_enemy.xy, t.xy);
                 if d < 130000 { score -= 60 } else if d < 160000 { /* 변화 없음 */ } else if d < 200000 { score += 20 } else { score += 40 } }
    None    => score += 35 }
  // L776~777 아군 체력
  ahp = (nearest_ally.hp*100 / nearest_ally.stat_cached.hp) as i32
  if ahp < 50 { score -= 30 } else if ahp > 69 { score += 10 }
  // L784~785 아군↔대상 거리
  ad = distance(nearest_ally.xy, target_enemy.xy)
  if ad > 120000 { score -= 25 } else if ad < 80000 { score += 15 }
  // L792 수적 우위
  if line_allies.len() + 1 > line_enemies.len() { score += 20 }
  actual_score = score
  // L799~801 판단력 노이즈 (utils.rs:480 error_ratio 인라인)
  error = (1000 - player.stat.judgement) / 20            // judgement 1000 → 0, 0 → 50
  pct = 100 + rnd.gen_range(-error ..= error)             // (assert low<=high, 위반 시 panic)
  evaluated_score = actual_score * pct / 100               // i32 sdiv (0 방향 절사)
  return (evaluated_score >= required_score, evaluated_score, actual_score)   // L803
```

**`mem` 메모리 접근 19건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | enemy_team = 1 - team (L702, m04.ll:63006). 자기 팀 인덱스는 L715/L728 에서 | 4 | OK |  |
| 1 | PlayerState | 0x9c0 | info.position@tag | r | jungler_champ = cache.player_champion[team][position] (L728, m04.ll:63174) | 4 | OK |  |
| 2 | PlayerState | 0x218 | info.parameter.stat.judgement | r | error_ratio 입력 (L799, m04.ll:63575) | 4 | OK |  |
| 3 | OperationData | 0x0 | cache(&AbstractGameWithCache) | r | self 로 쓰임 | 4 | OK |  |
| 4 | OperationData | 0x8 | context(&GameContext) | r | closure#0 에서 is_near_line 의 첫 인자 | 4 | OK |  |
| 5 | OperationData | 0x10 | blackboard(&[Blackboard;2]) | r | closure#0: blackboard[enemy_team].is_recent_visible / s_0: blackboard[team].in_big_line | 4 | OK |  |
| 6 | AbstractGameWithCache | 0x0 | game.data_ptr(&dyn AbstractGame data+vtable) | r | closure#0 이 is_recent_visible 에 넘김 (L703) | 4 | OK |  |
| 7 | AbstractGameWithCache | 0x1e0 | player_champion[2][5] | r | gep +480 (m04.ll:63012). iter_champions(enemy_team) 원천 및 jungler_champ·line_allies 조회 | 4 | OK |  |
| 8 | AbstractGameWithCache | 0x180 | top_tower[2] (line*0x20 스트라이드로 mid_tower 0x1a0 / bottom_tower 0x1c0) | r | gep +384 + line<<5 (m04.ll:63448~63453). enemy_tower 1순위 [enemy_team] | 4 | OK |  |
| 9 | AbstractGameWithCache | 0x190 | top_tower2[2] (line*0x20 → mid_tower2 0x1b0 / bottom_tower2 0x1d0) | r | gep +400 (m04.ll:63457). tower 가 None 일 때 폴백 (L759 `.or()`) | 4 | OK |  |
| 10 | Entity | 0x628 | stat_cached.hp (최대 HP) | r | target_enemy(L747)·nearest_ally(L776)·s1_0 필터(L716) HP% 분모. 0 이면 div_by_zero 패닉 | 4 | OK |  |
| 11 | Entity | 0x670 | hp | r | HP% 분자 | 4 | OK |  |
| 12 | Entity | 0x660 | x | r | min_by_key 거리²·distance()·is_near_line | 4 | OK |  |
| 13 | Entity | 0x668 | y | r | 동상 | 4 | OK |  |
| 14 | Blackboard | (참고, 본문 밖) 0xf8 | big_goal[pos].1 (@tag 0xf8 / line 0xf9) | r | in_big_line(pos,line) 내부(_gcbc g07.ll:156563) — 참고용. 본문엔 없음 | 4 | OK |  |
| 15 | Blackboard | (참고, 본문 밖) 0x1e0 | last_visible[position] | r | is_recent_visible 내부(_gcbc g07.ll:157005): last_visible+120 >= tick — 참고용. 본문엔 없음 | 4 | OK |  |
| 16 | sret (bool,i32,i32) | 0x0 | .1 evaluated_score | w | m04.ll:63698 | 4 | 확인불가(tcx 사전에 타입 없음) | actual_score * error_ratio / 100 (sdiv) 또는 0 |
| 17 | sret (bool,i32,i32) | 0x4 | .0 ok | w | m04.ll:63695~63697 | 4 | 확인불가(tcx 사전에 타입 없음) | evaluated_score >= required_score (sge) 또는 false |
| 18 | sret (bool,i32,i32) | 0x8 | .2 actual_score | w | m04.ll:63700 | 4 | 확인불가(tcx 사전에 타입 없음) | 노이즈 전 점수 또는 0 |

**`consts` 상수 29건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 2 | 702 | 임계 | 팀 인덱스 bounds check (enemy_team < 2). 판정 아님 | 4 |
| 1 | 5 | 713 | 산출값 | 포지션 수 — line_allies 후보 Range 0..5 (m04.ll:63102 store i64 5). 판정 아님 (본문의 `shl i8 %line, 5` 는 line*0x20 타워 배열 스트라이드 — 이 5 와 무관) | 4 |
| 2 | 1 | 724 | 계수 | L724: enemies.len() > allies.len()+1 이면 포기 — '정글러 합류 +1' 을 감안한 수적 열세 게이트. L792 에서도 allies.len()+1 > enemies.len() 이면 +20 (본문의 `shl`·`lshr 1` 피연산자 1 은 별개: lshr 1 = (80-hp%)/2 접힘, shl 1 = gen_range 내부 2*error) | 4 |
| 3 | 100 | 747 | 계수 | HP% 환산(hp*100/max_hp) — L747 적, L776 아군, L716 아군 필터, L801 최종 sdiv 100 | 4 |
| 4 | 79 | 748 | 임계 | 적 HP% > 79 (=80% 이상) → score = -40 | 4 |
| 5 | -40 | 748 | 산출값 | 적이 거의 풀피면 기본 점수 -40 (phi m04.ll:63442) | 4 |
| 6 | 59 | 750 | 임계 | 적 HP% > 59 (60~79%) → score = (80 - hp%) >> 1 (= /2, 1..10) | 4 |
| 7 | 80 | 751 | 미상 | 80 - hp% : 60~79% 대는 절반(1..10), 40~59% 대는 그대로(21..40) | 4 |
| 8 | 39 | 752 | 임계 | 적 HP% > 39 (40~59%) → score = 80 - hp% | 4 |
| 9 | 90 | 755 | 계수 | 적 HP% <= 39 → score = 90 - hp% (51..90) | 4 |
| 10 | 35 | 772 | 계수 | 그 라인에 적 타워(tower·tower2 모두)가 없으면 +35 | 4 |
| 11 | 130000 | 762 | 임계 | 적↔적타워 distance < 130000 → -60 (타워 품에 있음) | 4 |
| 12 | 160000 | 764 | 임계 | 130000 <= dist < 160000 → 가감 없음(빈 분기) | 4 |
| 13 | -60 | 763 | 계수 | 타워 130k 이내 페널티 | 4 |
| 14 | 200000 | 766 | 임계 | 160000 <= dist < 200000 → +20 / >= 200000 → +40 | 4 |
| 15 | 20 | 767 | 계수 | 타워 160k~200k 보너스 +20. L792 수적우위 보너스도 +20 | 4 |
| 16 | 40 | 769 | 계수 | 타워 200k 이상 보너스 +40. (s1_0 의 hp%>40 필터 임계도 40 — m04.ll:66416) | 4 |
| 17 | 50 | 777 | 임계 | 가장 가까운 아군 HP% < 50 → -30 | 4 |
| 18 | -30 | 777 | 계수 | 아군 저체력 페널티 | 4 |
| 19 | 69 | 777 | 임계 | 아군 HP% > 69 (70% 이상) → +10 | 4 |
| 20 | 10 | 777 | 계수 | 아군 고체력 보너스 | 4 |
| 21 | 120000 | 785 | 임계 | 아군↔대상 적 distance > 120000 → -25 | 4 |
| 22 | -25 | 785 | 계수 | 아군이 멀리 있음 페널티 | 4 |
| 23 | 80000 | 785 | 임계 | 아군↔대상 적 distance < 80000 → +15 | 4 |
| 24 | 15 | 785 | 계수 | 아군이 붙어 있음 보너스 | 4 |
| 25 | 1000 | 800 | 계수 | error_ratio(utils.rs:484): error = (1000 - judgement) / 20 (인라인) | 4 |
| 26 | 429496729600 | 800 | 계수 | = 100 << 32. utils.rs:486 `100 + r` 가 gen_range 의 widening-multiply 상위워드 산술에 접힘: pct = 100 + (r_sample - error), r_sample ∈ [0, 2*error] | 4 |
| 27 | 63 | 800 | 임계 | rand_chacha 버퍼 인덱스 > 63 이면 refill_wide (라이브러리 내부, 판정 아님) | 4 |
| 28 | 6 | 800 | 미상 | ChaCha 라운드 인자 drounds=6 (ChaCha12, 라이브러리 내부) | 4 |

**`knobs` 조정점 10건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 적 HP 구간 임계 3단 | passive_jungle.rs:748/750/752 | 79/59/39 | 낮추면 더 높은 체력의 적도 갱 대상 점수가 양수가 됨(공격적). 80% 이상은 무조건 -40 시작 | 4 | 기존 |
| 1 | 적 HP 점수식 기저 | passive_jungle.rs:751/753/755 | 80 / 80 / 90 | 올리면 같은 체력에서 점수 상승 | 4 | 기존 |
| 2 | 적↔타워 거리 3단 | passive_jungle.rs:762/764/766 | 130000/160000/200000 | 올리면 타워 근처 적을 더 넓게 '타워 품'으로 봐 갱 억제 | 4 | 기존 |
| 3 | 타워 가감 | passive_jungle.rs:763/767/769/772 | -60/+20/+40/+35(타워 없음) | -60 을 줄이면 타워 다이브 갱 증가 | 4 | 기존 |
| 4 | 아군 HP 임계/가감 | passive_jungle.rs:777 | <50 → -30, >69 → +10 |  | 4 | 기존 |
| 5 | 아군↔적 거리 임계/가감 | passive_jungle.rs:785 | >120000 → -25, <80000 → +15 | 아군이 적에게 붙어 있을수록 갱 선호 | 4 | 기존 |
| 6 | 수적 우위 보너스 | passive_jungle.rs:792 | 20 | allies+1 > enemies 일 때 가산 | 4 | 기존 |
| 7 | 수적 열세 컷 | passive_jungle.rs:724 | enemies > allies+1 | 완화하면 열세 갱도 평가 진행 | 4 | 기존 |
| 8 | 아군 라이너 최소 HP% 필터 | passive_jungle.rs:716 | 40 | 올리면 저체력 아군이 있는 라인은 allies 가 비어 갱 자체 불가 | 4 | 기존 |
| 9 | 판단력 노이즈 폭 | utils.rs:484 | (1000-judgement)/20 | judgement 낮을수록 ±% 폭 커짐(0 이면 ±50%). 상수 20 을 키우면 노이즈 감소 | 4 | 기존 |

<details><summary>`callees` 피호출자 13건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | distance | game_core::utils::distance | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:12 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | evaluate_gank_opportunity_with_score | game_ai::plan_legacy::old::evaluate_gank_opportunity_with_score | pub | fn(&mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, game_core::LineType, i32) -> (bool, i32, i32) | game-ai\src\plan_legacy\old\passive_jungle.rs:693 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 2 | in_big_line | game_core::Blackboard::in_big_line | pub | fn(&game_core::Blackboard, usize, game_core::LineType) -> bool | game-core\src\simulation\game\blackboard.rs:137 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 3 | is_empty | game_core::PatchSetting::is_empty | pub | fn(&game_core::PatchSetting) -> bool | game-core\src\patch_engine.rs:853 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 4 | is_empty | game_core::ModAiRegistry::is_empty | pub | fn(&game_core::ModAiRegistry) -> bool | game-core\src\mod_ai.rs:15 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 5 | is_empty | game_core::PublicChampionEvidence::is_empty | pub | fn(&game_core::PublicChampionEvidence) -> bool | game-core\src\banpick\public_evidence.rs:96 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 6 | is_near_line | game_core::is_near_line | pub | fn(&game_core::GameContext, u64, u64, game_core::LineType) -> bool | game-core\src\simulation\map_regions.rs:139 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 7 | is_recent_visible | game_core::Blackboard::is_recent_visible | pub | fn(&game_core::Blackboard, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\game\blackboard.rs:346 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 8 | iter_champions | game_core::AbstractGameWithCache::<'a, 'b>::iter_champions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5759 ~ game_core[6a30]::simulation::{impl#5}::iter_champions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1904 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 9 | team | game_view::ClientData::team | pub | fn(&game_view::ClientData, usize) -> std::option::Option<std::cell::Ref< game_core::Team>> | game-view\src\logic\client\data.rs:1651 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 10 | team | game_view::ClientDatabase::team | pub | fn(&game_view::ClientDatabase, usize) -> std::option::Option<&game_core::Team> | game-view\src\logic\client\data.rs:968 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 11 | team | game_core::PlayerAiContext::<'a, 'b, 'r>::team | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:583 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 12 | tower | game_core::AbstractGameWithCache::<'a, 'b>::tower | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, game_core::LineType, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:1822 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
</details>

⚠**미매칭 7개**: `collect`, `drop  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `gen_range`, `panic`, `refill_wide`, `reserve_internal_or_panic`, `tower2`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 3곳** (m01.ll:47639, m13.ll:21393, m13.ll:30923) · **형제 0개** 

**`open` 7건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | blackboard 인덱스 비대칭: closure#0 의 is_recent_visible 은 blackboard[1-team](적 팀 판) (m04.ll:66367~66370), line_allies 의 in_big_line 은 blackboard[team](자기 팀 판) (m01.ll:30760). 적팀 판 last_visible 을 자기 시점 가시성으로 쓰는 것이 의도인지는 Blackboard 갱신 코드(미탐색)로만 확정 가능 — IR 로는 인덱스 사실만 확정 | 4 |  |
| 1 | 미탐색 | min_by_key 의 첫 원소 키(dx²+dy²)는 본문에 인라인, 나머지 원소는 `Map::fold`(m12.ll 30690/30833) 호출로 처리 — 본문 인라인분과 동일 식임을 m12 define 이름(s2_0/s3_0)으로 추정, m12 본문은 안 읽음(range 밖·aux 미선언) | 5 |  |
| 2 | 표기 불가 | L764 분기(130k<=d<160k)는 본문 phi 에서 score 그대로(%181→%212) — 소스가 빈 블록인지 `score += 0` 인지는 표기 불가(외연 동일) | 4 |  |
| 3 | 표기 불가 | L750 `(80-ehp)>>1` 은 lshr(부호 없는 시프트) — 소스가 `/2` 인지 `>>1` 인지 표기 불가(60~79 구간에서 피연산자 양수라 외연 동일) | 4 |  |
| 4 | 미탐색 | 튜플 .1/.2 의 이름(evaluated_score/actual_score)은 dbg_value 이름 + 스토어 순서로 확정, 소스 변수명은 dbg 이름 그대로 | 4 |  |
| 5 | 미탐색 | iter_champions::closure0 본문(FilterMap of Option<&Entity>)은 m01.ll 40283 범위에 인라인된 `icmp eq ptr null` 로만 관측 — Some 필터 이외 조건 없음으로 읽음 | 4 |  |
| 6 | 미탐색 | gen_range 의 assert(low<=high, panic m04.ll:63677)은 error>=0 이라 항상 통과 — judgement > 1000 이면 usize 언더플로로 error 가 거대해져 i32 trunc 후 음수 가능성 있으나 judgement 값 범위는 미탐색 | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

