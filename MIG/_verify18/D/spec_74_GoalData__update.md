---

### `74` GoalData::update — 매 평가마다 GoalData 상태 갱신: 힐 커밋(v2+) · 본진수비 틱 · 적 5명 리전 추적(5초 만료) · 에픽/세르펜 태세 갱신

| 항목 | 값 |
|---|---|
| id | `goal_data__GoalData_update` |
| 심볼 | `_RNvMNtCshdEBA0ozCnw_7game_ai9goal_dataNtB2_8GoalData6update` |
| 소스 | `game-ai\src\goal_data.rs:82` |
| IR | `m09.ll` 4395~5225행 |
| 경로·가시성 | `game_ai::GoalData::update` · **pub** |
| 계층 | 기타 |
| exe | `dccc60` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r9` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&mut game_ai::GoalData, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData)
```

<details><summary>인자 6개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | self | &mut GoalData(248B) | goal_data.rs:11 · enemy_region[5]@0x0(stride 24) · epic@0x78 · serpen@0xb0 · last_base_defense_tick@0xe8 · heal_commit@0xf0 | 4 |
| 1 | 1 | version | usize | AI 버전. update_heal_commit 인라인부(goal_data.rs:28)에서만 `version < 2 → 힐커밋 스킵`. 그 외 분기 없음 | 4 |
| 2 | 2 | rnd | &mut StdRng(320B) | readnone — 본문에서 안 읽음. 콜리 update_plan 에도 poison 으로 넘김 | 4 |
| 3 | 3 | player | &PlayerState(2528B) | info.team@0x930 · info.position@0x9c0 사용 | 4 |
| 4 | 4 | data | &OperationData(24B) | cache@0x0 · context@0x8 | 4 |
| 5 | 5 | debug | &mut DebugFrameData(224B) | ctx.debug 일 때만 add_log / +0xa0 HashMap<usize,Vec<String>> 에 문자열 push | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn update(&mut self, version, _rnd, player, data, debug):
  // ---- (A) update_heal_commit (goal_data.rs:27~57, 인라인) ----
  if version >= 2 {                                   // :28  version<2 → 이 블록 전체 스킵
    champ = cache.player_champion[player.team][player.pos]   // :31 (team>=2 → bounds panic)
    if champ == None { self.heal_commit = false; }   // :32~33
    else {
      before = self.heal_commit                       // :35
      if nexus_final_stand(player,data)      { new = false }   // :36
      else if nexus_is_critical(player,data) { new = false }   // :37
      else if before {                                          // :39
        if champ.hp < champ.stat_cached.hp { (store 없음 = true 유지) } else { new = false }   // :40 풀피 도달 → 해제
      } else {                                                  // :44
        (lx,ly,rx,ry) = map.fountains[player.team]
        in_heal_area = lx<=champ.x<=rx && ly<=champ.y<=ry      // :45
        hp_ratio = champ.hp*100 / max(champ.stat_cached.hp,1)  // :46
        if in_heal_area && hp_ratio < 36 && base_defense_focus(player,data) { new = true }   // :47 (&& 단락: base_defense_focus 는 앞 둘 통과 시만 호출)
        else (store 없음 = false 유지)
      }
      if store 됐고 before != new && ctx.debug { debug.add_log(format!("HEALCOMMIT T{team} {pos:?} {start|end} hp={hp*100/max_hp}%")) }  // :51~55
    }
  }
  // ---- (B) 본진수비 틱 ----
  if base_defense_focus(player,data) { self.last_base_defense_tick = tick() }   // :84~85
  // ---- (C) 적 리전 추적 ----
  for p in cache.game.iter_player() {                 // :87
    if p.team == player.team { continue }            // :88 아군 스킵
    pos = p.position.as_index()                       // :91
    if p.is_dead() {                                  // :93 (play_state tag==0)
      region = if p.team==0 {0} else {26}             // :95
    } else {                                          // :98
      echamp = cache.player_champion[p.team][pos]; if None → goto 만료검사
      if !is_visible(p.team, echamp.id) → goto 만료검사      // :99
      region = map.regions[min(echamp.y/32000,29)][min(echamp.x/32000,29)]   // :101
    }
    self.enemy_region[pos] = Some{region, last_known: tick()}   // :103~105
    만료검사: if let Some(r)=self.enemy_region[pos] { if r.last_known + tps*5 < tick() { self.enemy_region[pos]=None } }   // :107~109
  }
  // ---- (D) 오브젝트 태세 ----
  self.epic.update_plan(player, data, /*enemy_region=*/self, _)       // :114
  self.serpen.update_plan(player, data, /*enemy_region=*/self, debug) // :115
  // ---- (E) 디버그 ----
  if ctx.debug { champ = player_champion[player.team][pos]; if Some → debug.map[champ.id].push("epic_stance: {}","serpen_stance: {}","epic_enemy_tick: {}","epic_enemy_killed_tick: {}","epic_ally_tick: {}","epic_ally_killed_tick: {}") }   // :117~126 (epic 쪽 4틱만 출력, serpen 틱은 미출력)
```

**`mem` 메모리 접근 29건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | player 및 iter_player 의 각 p (goal_data.rs:31,88,118) | 4 | OK |  |
| 1 | PlayerState | 0x9c0 | info.position@tag(i32) | r | Position::as_index (entity.rs:581) → 0 Top/1 Jungle/2 Mid/3 Bottom/4 Support | 4 | OK |  |
| 2 | PlayerState | 0x9c8 | play_state@tag | r | PlayerState::is_dead (player.rs:1660) = tag==0(Die). goal_data.rs:93 | 4 | OK |  |
| 3 | OperationData | 0x0 | cache | r | &AbstractGameWithCache | 4 | OK |  |
| 4 | OperationData | 0x8 | context | r | &GameContext | 4 | OK |  |
| 5 | AbstractGameWithCache | 0x0 | game.data_ptr(&dyn AbstractGame: data,vtable) | r | vtable+0x28 tick · +0xf8 is_visible · +0x208 iter_player (divtable 실측) | 3 | OK |  |
| 6 | AbstractGameWithCache | 0x1e0 | player_champion[team][pos] | r | [[Option<&Entity>;5];2] — null=None. goal_data.rs:31,98,118 | 4 | OK |  |
| 7 | GameContext | 0x8 | setting | r | &GameSetting | 4 | OK |  |
| 8 | GameContext | 0x20 | map | r | &MapDef | 4 | OK |  |
| 9 | GameContext | 0x3b | debug(bool) | r | goal_data.rs:51,117 — 디버그 로그 게이트 | 4 | OK |  |
| 10 | GameSetting | 0x12f8 | tick_per_second | r | goal_data.rs:108 (tps*5 만료) | 4 | OK |  |
| 11 | MapDef | 0x6d70 | fountains[team] (lx,ly,rx,ry) | r | map_def.rs:235 fountain(team) — stride 32B, .0@+0 lx / .1@+8 ly / .2@+16 rx / .3@+24 ry. goal_data.rs:44 | 4 | OK |  |
| 12 | MapDef | 0x38b8 | regions[cy][cx] | r | [[usize;30];30] — regions[clamp(y/32000,0,29)][clamp(x/32000,0,29)]. goal_data.rs:101 | 4 | OK |  |
| 13 | Entity | 0x5c0 | id | r | 적 챔피언 id → is_visible 인자 / 디버그 HashMap 키 | 4 | OK |  |
| 14 | Entity | 0x660 | x | r |  | 4 | OK |  |
| 15 | Entity | 0x668 | y | r |  | 4 | OK |  |
| 16 | Entity | 0x670 | hp | r | 현재 HP | 4 | OK |  |
| 17 | Entity | 0x628 | stat_cached.hp | r | 최대 HP (hp_ratio 분모, max(.,1)) | 4 | OK |  |
| 18 | GoalData | 0xf0 | heal_commit(before) | r | goal_data.rs:35 | 4 | OK |  |
| 19 | GoalData | 0x0 | enemy_region[pos]@tag / +0x10 last_known | r | goal_data.rs:107 만료 검사 (stride 24) | 4 | OK |  |
| 20 | GoalData | 0x88 | epic.epic_enemy_tick · epic.stance@0xa8 · serpen.stance@0xe0 | r | 디버그 출력 전용(goal_data.rs:121~126) — 판정에 안 쓰임 | 4 | OK |  |
| 21 | GoalData | 0xf0 | heal_commit | w | ★인라인 update_heal_commit(goal_data.rs:27~57). 0: champ None(:32) · nexus_final_stand(:36) · nexus_is_critical(:37) · before&&hp>=max_hp(:40) / 1: !before && in_heal_area && hp_ratio<36 && base_defense_focus(:47). 그 외 경로(before&&hp<max · !before 조건미충족)는 store 없음=값 유지 | 4 | OK | 0 / 1 |
| 22 | GoalData | 0xe8 | last_base_defense_tick | w | goal_data.rs:84~85 — base_defense_focus(player,data)==true 일 때만 | 4 | OK | cache.game.tick() |
| 23 | GoalData | 0x0 + pos*24 | enemy_region[pos]@tag | w | goal_data.rs:103~105 Some 기록(적 사망 or 가시) / :109 None(last_known+tps*5 < tick) | 4 | OK | 1(Some) 또는 0(None) |
| 24 | GoalData | 0x8 + pos*24 | enemy_region[pos].region | w | goal_data.rs:95,101 | 4 | OK | 사망: team==0→0, else 26 / 생존·가시: map.regions[cy][cx] |
| 25 | GoalData | 0x10 + pos*24 | enemy_region[pos].last_known | w | goal_data.rs:104 | 4 | OK | cache.game.tick() |
| 26 | GoalData | 0x78 | epic(EpicStanceData 56B) | w | goal_data.rs:114 — self+0x78 을 self 로, GoalData 전체(self)를 enemy_region 인자로 넘김 | 4 | OK | EpicStanceData::update_plan 콜리가 갱신 |
| 27 | GoalData | 0xb0 | serpen(SerpenStanceData 56B) | w | goal_data.rs:115 | 4 | OK | SerpenStanceData::update_plan 콜리가 갱신 |
| 28 | DebugFrameData | 0xa0 | HashMap<usize(champ.id), Vec<String>> | w | goal_data.rs:117~126 — ctx.debug && 내 champ Some 일 때만 | 4 | 오귀속(사전은 다른 필드를 준다) | epic_stance/serpen_stance/epic_*_tick 6줄 |

**`consts` 상수 8건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 2 | 28 | 임계 | version 게이트: version < 2 이면 update_heal_commit 전체 스킵(힐 커밋 없음) | 4 |
| 1 | 36 | 47 | 임계 | hp_ratio(%) < 36 이어야 힐 커밋 시작 (hp*100/max(max_hp,1)) | 4 |
| 2 | 100 | 46 | 계수 | hp 백분율 계수 (hp*100/max_hp) — :46,:55 두 곳 | 4 |
| 3 | 5 | 108 | 계수 | enemy_region 만료 = tps*5 (5초) — `mul i64 %tps, 5`; last_known + tps*5 < tick 이면 None | 4 |
| 4 | 32000 | 101 | 계수 | 셀 크기(좌표→셀 변환) — 임계 아님 | 4 |
| 5 | 29 | 101 | 인덱스 | 셀 인덱스 clamp 상한 (30x30 격자) | 4 |
| 6 | 0 | 95 | 태그 | 사망 적(team 0)의 리전 코드 = 0 (본진 리전 추정: 확정 근거 없음 — unknown 참조) | 5 |
| 7 | 26 | 95 | 산출값 | 사망 적(team 1)의 리전 코드 = 26 (본진 리전 추정: 확정 근거 없음) | 5 |

**`knobs` 조정점 3건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 힐 커밋 시작 HP% | goal_data.rs:47 | 36 | 올리면 더 높은 HP 에서도 우물 힐 커밋을 걸어 풀피까지 우물에 머문다(본진수비 문맥+우물 안에서만). 내리면 커밋이 드물어진다 | 4 | 기존 |
| 1 | 적 위치 기억 만료(초) | goal_data.rs:108 | 5 | 올리면 안 보인 적을 더 오래 '그 리전에 있다'고 가정 → EpicStance/SerpenStance 의 enemy_in_epic 판정(is_none_or)에서 None 취급이 늦어진다. 내리면 빨리 None(=적이 어디든 있을 수 있음)으로 돌아간다 | 4 | 기존 |
| 2 | 힐 커밋 버전 게이트 | goal_data.rs:28 | 2 | v1 은 힐 커밋 자체가 없다 | 4 | 기존 |

<details><summary>`callees` 피호출자 25건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | add_log | game_core::DebugFrameData::add_log | pub | fn(&mut game_core::DebugFrameData, &game_core::OperationData, &game_core::PlayerState, std::string::String) | game-core\src\simulation\game\frame.rs:54 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | as_index | game_core::BigGoal::as_index | pub | fn(&game_core::BigGoal) -> usize | game-core\src\simulation\ai_interface.rs:338 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 2 | as_index | game_core::Position::as_index | pub | fn(&game_core::Position) -> usize | game-core\src\simulation\entity.rs:580 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 3 | base_defense_focus | game_ai::plan_legacy::old::base_defense_focus | pub | fn(&game_core::PlayerState, &game_core::OperationData) -> bool | game-ai\src\plan_legacy\old\defense_nexus.rs:200 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | is_dead | game_core::PlayState::is_dead | pub | fn(&game_core::PlayState) -> bool | game-core\src\simulation\state\player.rs:1659 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 5 | is_visible | game_core::World::is_visible | pub | fn(&game_core::World, usize, usize) -> bool | game-core\src\simulation\game\data.rs:384 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 6 | is_visible | game_core::VisibleState::is_visible | pub | fn(&game_core::VisibleState) -> bool | game-core\src\simulation\game\data.rs:121 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 7 | is_visible | game_core::AbstractGame::is_visible | pub | fn(&Self/#0, usize, usize) -> bool | game-core\src\simulation.rs:125 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 8 | iter_player | game_core::AbstractGame::iter_player | pub | fn(&Self/#0) -> game_core::PlayerIter | game-core\src\simulation.rs:181 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 9 | iter_player | <game_core::Game as game_core::AbstractGame>::iter_player | pub | fn(&game_core::Game) -> game_core::PlayerIter | game-core\src\simulation\game.rs:3756 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 10 | iter_player | <game_core::SingleLaneGame as game_core::AbstractGame>::iter_player | pub | fn(&game_core::SingleLaneGame) -> game_core::PlayerIter | game-core\src\simulation\game.rs:4015 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 11 | nexus_final_stand | game_ai::plan_legacy::old::nexus_final_stand | pub | fn(&game_core::PlayerState, &game_core::OperationData) -> bool | game-ai\src\plan_legacy\old\defense_nexus.rs:190 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 12 | nexus_is_critical | game_ai::plan_legacy::old::nexus_is_critical | pub | fn(&game_core::PlayerState, &game_core::OperationData) -> bool | game-ai\src\plan_legacy\old\defense_nexus.rs:139 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 13 | push | <game_core::Staff as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::staff | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\staff.rs:38 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 14 | push | <game_core::Athlete as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:434 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 15 | push | <game_core::Contract as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:2498 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 16 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 17 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 18 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 19 | update | game_ai::GoalData::update | pub | fn(&mut game_ai::GoalData, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) | game-ai\src\goal_data.rs:82 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 257개 중 상위 3개 |
| 20 | update | game_ai::EpicStanceData::update | pub | fn(&mut game_ai::EpicStanceData, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &[std::option::Option<game_ai::EnemyRegionInfo>; 5_usize], &mut game_core::DebugFrameData) | game-ai\src\goal_data.rs:158 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 257개 중 상위 3개 |
| 21 | update | game_ai::plan_legacy::types::BigPlan::update | pub | fn(&mut game_ai::plan_legacy::types::BigPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PositioningScoreData, &mut game_core::DebugFrameData) | game-ai\src\plan_legacy\types.rs:198 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 257개 중 상위 3개 |
| 22 | update_heal_commit | game_ai::GoalData::update_heal_commit | in:game_ai::goal_data | fn(&mut game_ai::GoalData, usize, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) | game-ai\src\goal_data.rs:27 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 23 | update_plan | game_ai::EpicStanceData::update_plan | pub | fn(&mut game_ai::EpicStanceData, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &[std::option::Option<game_ai::EnemyRegionInfo>; 5_usize], &mut game_core::DebugFrameData) | game-ai\src\goal_data.rs:266 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 24 | update_plan | game_ai::SerpenStanceData::update_plan | pub | fn(&mut game_ai::SerpenStanceData, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &[std::option::Option<game_ai::EnemyRegionInfo>; 5_usize], &mut game_core::DebugFrameData) | game-ai\src\goal_data.rs:399 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
</details>

⚠**미매칭 5개**: `else`, `format_inner`, `grow_one`, `insert_no_grow`, `rustc_entry`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 4곳** (m13.ll:10523, m13.ll:15485, m13.ll:16508, m13.ll:19115) · **형제 7개** (GoalData)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::GoalData as std::clone::Clone>::clone | pub | game-ai\src\goal_data.rs:10 | True | fn(&game_ai::GoalData) -> game_ai::GoalData |
| 1 | <game_ai::GoalData as std::default::Default>::default | pub | game-ai\src\goal_data.rs:10 | True | fn() -> game_ai::GoalData |
| 2 | <game_ai::GoalData as std::fmt::Debug>::fmt | pub | game-ai\src\goal_data.rs:10 | True | fn(&game_ai::GoalData, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 3 | game_ai::GoalData::update_heal_commit | in:game_ai::goal_data | game-ai\src\goal_data.rs:27 | False | fn(&mut game_ai::GoalData, usize, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 4 | game_ai::GoalData::has_near_line_enemy | pub | game-ai\src\goal_data.rs:60 | False | fn(&game_ai::GoalData, game_core::LineType, &game_core::MapDef) -> bool |
| 5 | game_ai::GoalData::has_near_enemy | pub | game-ai\src\goal_data.rs:78 | False | fn(&game_ai::GoalData, usize, &game_core::MapDef) -> bool |
| 6 | game_ai::GoalData::update | pub | game-ai\src\goal_data.rs:82 | False | fn(&mut game_ai::GoalData, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |

**`open` 5건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | is_visible(team,id) 의 team 인덱스 의미 — World::is_visible(g07.ll:171567~171640)은 entity[id].visible_state[team]==Visible 인데, 이 함수는 (적 팀, 적 챔프 id) 로 부른다. visible_state[t] 가 '팀 t 소속 엔티티를 상대팀이 보는가'인지 '팀 t 가 보는가'인지는 game_core 시야 갱신 코드(미탐색) 확인 필요. SerpenStance/EpicStance 는 반대로 (내 팀, 오브젝트 id)로 부른다 | 4 |  |
| 1 | 표기 불가 | goal_data.rs:107 의 만료 검사가 Some 기록 직후에도 실행되는 순서(IR 기준 :103~105 store 후 :107 load) — 소스가 별도 if 인지 같은 블록인지는 column 부재로 표기 불가(동작은 확정) | 4 |  |
| 2 | 표기 불가 | update_heal_commit 의 소스 표기(별도 fn 인지 메서드인지) — DISubprogram 이름만 확인(inlinedAt 루트 :83). 동작은 확정 | 4 |  |
| 3 | 미탐색 | constants 의 3/5 ("end"/"start" 문자열 길이 select i64 5, i64 3 · goal_data.rs:54) 는 판정 상수가 아니라 제외 | 4 |  |
| 4 | 미탐색 | debug HashMap 키가 Entity.id(+0x5c0)인 것은 rustc_entry 인자에서 확인, 값 Vec<String> 의 소비처는 미탐색 | 4 |  |

<details><summary>`closed` 1건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

| # | 물음 | 닫은 근거 |
|---|---|---|
| 0 | 사망 적 리전 코드 0/26 의 의미 — 팀별 본진 리전으로 추정되나 MapDef.regions 값표를 안 읽어 미확정(미탐색: MapDef 데이터 덤프 / region_centers[0],[26] 좌표 대조) | 본문에 해소 표기가 있다 |
</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

