---

### `151` SmallActionSkill::get_input — 소액션 '스킬1 시전': 대상 존재·적 우물 위험·논타겟 검사 후 abstract_input::skill 로 입력 생성, Skill 이면 is_act 세움

| 항목 | 값 |
|---|---|
| id | `SmallActionSkill__get_input` |
| 심볼 | `_RNvMs1_NtNtCshdEBA0ozCnw_7game_ai12small_action4castNtB5_16SmallActionSkill9get_input` |
| 소스 | `game-ai\src\small_action\cast.rs:133` |
| IR | `m07.ll` 11808~11934행 |
| 경로·가시성 | `game_ai::SmallActionSkill::get_input` · **in:game_ai** |
| 계층 | 기타 |
| exe | `d845e0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r14` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&mut game_ai::SmallActionSkill, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData) -> std::option::Option<game_core::Input>
```

<details><summary>인자 7개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | Option<Input>(32B) | 반환 슬롯. 아래 returns 참조 | 4 |
| 1 | 1 | &mut self | &mut SmallActionSkill(24B) | +0x8 target(usize 엔티티 id) 읽기 · +0x10 is_act 쓰기. +0x0 start_tick 은 안 씀 | 4 |
| 2 | 2 | version | usize | 본문 분기 없음 — is_enemy_well_danger·safe_move·skill 에 전달만 | 4 |
| 3 | 3 | rnd | &mut StdRng(320B) | check_nontarget·abstract_input::skill 에 전달만(본문에서 직접 안 읽음) | 4 |
| 4 | 4 | player | &PlayerState(2528B) | readonly. info.team(+0x930)·info.position(+0x9c0) | 4 |
| 5 | 5 | data | &OperationData(24B) | readonly. +0x0 cache → game(dyn) get_entity_by_id · player_champion | 4 |
| 6 | 6 | positioning_score | &PositioningScoreData(2760B) | abstract_input::skill 에 전달만 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn get_input(&mut self, version, rnd, player, data, positioning_score) -> Option<Input>   [cast.rs:133~154]
  L134: target = data.cache.game.get_entity_by_id(self.target)   // vtable+0x1f0
        None → L152: return None (sret +0x0 = -1)
  L135: champ = data.cache.player_champion[player.team][player.position].unwrap()   // None 이면 패닉(unwrap_failed)
  L136: if target.team == TeamType::Player(1 - player.team) {           // 적 소속이면
  L137:   if path_finder::is_enemy_well_danger(version, player, target.x, target.y) {
  L138:     return None } }
  L141: skill = champ.skill_effect.as_ref().unwrap()                  // None 이면 패닉
        if check_nontarget(rnd, player, data.cache, skill, target) {   // 잎: true = 논타겟 검사 통과(시전 가능)
  L145:   input = abstract_input::skill(version, rnd, player, data, positioning_score, target)
  L146:   if input.tag == 3 (Some(Input::Skill)) { L147: self.is_act = true }
          return input
        } else {
  L142:   return safe_move_avoiding_enemy_well(version, player, data, champ, target.x, target.y)   // 잎 sret
        }
```

**`mem` 메모리 접근 15건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | SmallActionSkill | 0x8 | target | r | 대상 엔티티 id → game.get_entity_by_id (m07.ll:11821~11825) | 4 | OK |  |
| 1 | OperationData | 0x0 | cache | r | &AbstractGameWithCache (m07.ll:11817) | 4 | OK |  |
| 2 | AbstractGameWithCache | 0x0 | game | r | &dyn AbstractGame: data ptr +0x0, vtable +0x8 → 슬롯 +0x1f0 get_entity_by_id (m07.ll:11818~11826) | 4 | OK |  |
| 3 | AbstractGameWithCache | 0x1e0 | player_champion[team][position] | r | None → unwrap_failed 패닉 (m07.ll:11849~11854) | 4 | OK |  |
| 4 | PlayerState | 0x930 | info.team | r | ≥2 → panic_bounds_check | 4 | OK |  |
| 5 | PlayerState | 0x9c0 | info.position@tag | r | i32 → zext 인덱스 | 4 | OK |  |
| 6 | Entity | 0x0 | team@tag | r | target.team: 0=Player (L136 인라인 entity.rs:1127) | 4 | OK |  |
| 7 | Entity | 0x8 | team@Player.0 | r | == 1 - my team 이면 적 (m07.ll:11871~11880) | 4 | OK |  |
| 8 | Entity | 0x660 | x | r | target.x → is_enemy_well_danger / safe_move (L137·L142) | 4 | OK |  |
| 9 | Entity | 0x668 | y | r | target.y | 4 | OK |  |
| 10 | Entity | 0x4f8 | skill_effect@tag | r | champ.skill_effect None(-1) → unwrap_failed 패닉 (m07.ll:11885~11887) | 4 | OK |  |
| 11 | Entity | 0x4c8 | skill_effect@Some.0 | r | &Effect(56B) → check_nontarget 인자 (m07.ll:11903~11905) | 4 | OK |  |
| 12 | SmallActionSkill | 0x10 | is_act | w | L147 · m07.ll:11932 — abstract_input::skill 결과 태그가 3(Some(Input::Skill)) 일 때만. 그 외 경로는 self 무변경 | 4 | OK | 1 (true, i8) |
| 13 | (sret) | 0x0 | Option<Input>@tag | w | L152 대상 없음(m07.ll:11837) · L138 적 우물 위험(m07.ll:11899). 8B 만 기록 | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) | -1 (None) |
| 14 | (sret) | 0x0 | Option<Input> 전체 | w | L142 safe_move_avoiding_enemy_well(sret) / L145 abstract_input::skill(sret) — 32B 중 live 는 콜리 규약 | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) | 콜리가 채움 |

**`consts` 상수 5건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | -1 | 152 | 센티널 | Option<Input>::None 니치 태그(store i64 -1, m07.ll:11837·11899). 같은 -1 이 Entity.skill_effect None 태그(i32) 비교(L141, m07.ll:11887)에도 | 4 |
| 1 | 2 | 135 | 임계 | team 인덱스 bounds(len 2) — 임계 아님 | 4 |
| 2 | 0 | 136 | 태그 | TeamType::Player 메모리태그 0 — target.team 이 Player 인지 (m07.ll:11863) | 4 |
| 3 | 1 | 136 | 산출값 | `1 - team` = 적 팀 (m07.ll:11872) | 4 |
| 4 | 3 | 146 | 태그 | Input::Skill 메모리태그 3 — abstract_input::skill 결과가 Some(Skill) 인지 (m07.ll:11924) | 4 |

**`knobs` 조정점 0건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev |
|---|---|---|---|---|---|

<details><summary>`callees` 피호출자 10건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | check_nontarget | game_ai::small_action::cast::check_nontarget | in:game_ai::small_action::cast | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::Effect, &game_core::Entity) -> bool | game-ai\src\small_action\cast.rs:38 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 2 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 3 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 4 | get_input | game_ai::PathFinder::get_input | pub | fn(&mut game_ai::PathFinder, &game_core::PlayerState, &game_core::OperationData, game_ai::SafeMoveWithSkill, bool) -> game_core::Input | game-ai\src\path_finder.rs:856 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 22개 중 상위 3개 |
| 5 | get_input | game_ai::SmallActionUlt::get_input | in:game_ai | fn(&mut game_ai::SmallActionUlt, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData) -> std::option::Option<game_core::Input> | game-ai\src\small_action\cast.rs:261 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 22개 중 상위 3개 |
| 6 | get_input | game_ai::SmallActionPlay::get_input | pub | fn(&mut game_ai::SmallActionPlay, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &mut game_core::DebugFrameData) -> std::option::Option<game_core::Input> | game-ai\src\small_action.rs:157 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 22개 중 상위 3개 |
| 7 | is_enemy_well_danger | game_ai::is_enemy_well_danger | pub | fn(usize, &game_core::PlayerState, u64, u64) -> bool | game-ai\src\path_finder.rs:1032 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 8 | safe_move_avoiding_enemy_well | game_ai::safe_move_avoiding_enemy_well | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, u64, u64) -> std::option::Option<game_core::Input> | game-ai\src\abstract_input.rs:122 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 9 | skill | game_ai::skill | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_core::Entity) -> std::option::Option<game_core::Input> | game-ai\src\abstract_input.rs:199 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
</details>

**호출처 1곳** (m11.ll:43077) · **형제 9개** (SmallActionSkill)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::SmallActionSkill as std::clone::Clone>::clone | pub | game-ai\src\small_action\cast.rs:111 | True | fn(&game_ai::SmallActionSkill) -> game_ai::SmallActionSkill |
| 1 | <game_ai::SmallActionSkill as std::fmt::Debug>::fmt | pub | game-ai\src\small_action\cast.rs:111 | True | fn(&game_ai::SmallActionSkill, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 2 | game_ai::SmallActionSkill::new | pub | game-ai\src\small_action\cast.rs:119 | False | fn(&game_core::OperationData, usize) -> game_ai::SmallActionSkill |
| 3 | game_ai::SmallActionSkill::movement_landing_position | in:game_ai | game-ai\src\small_action\cast.rs:127 | False | fn(&game_ai::SmallActionSkill, &game_core::Entity, &game_core::OperationData) -> std::option::Option<(u64, u64)> |
| 4 | game_ai::SmallActionSkill::get_input | in:game_ai | game-ai\src\small_action\cast.rs:133 | False | fn(&mut game_ai::SmallActionSkill, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData) -> std::option::Option<game_core::Input> |
| 5 | game_ai::SmallActionSkill::update_state | in:game_ai | game-ai\src\small_action\cast.rs:156 | False | fn(&mut game_ai::SmallActionSkill, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 6 | game_ai::SmallActionSkill::get_action | in:game_ai | game-ai\src\small_action\cast.rs:159 | True | fn(&game_ai::SmallActionSkill) -> game_core::SmallAction |
| 7 | game_ai::SmallActionSkill::is_end | in:game_ai | game-ai\src\small_action\cast.rs:162 | False | fn(&game_ai::SmallActionSkill, &mut rand::rngs::std::StdRng, usize, &game_core::PlayerState, &game_core::OperationData) -> bool |
| 8 | game_ai::SmallActionSkill::is_complete | in:game_ai | game-ai\src\small_action\cast.rs:169 | True | fn(&game_ai::SmallActionSkill) -> bool |

**`open` 5건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | is_enemy_well_danger(version, &PlayerState, x, y)->bool (m03.ll:144500) 내부는 안 봄 — 잎 22 아님, 시그니처만 | 4 |  |
| 1 | 미탐색 | check_nontarget(&mut StdRng, &PlayerState, cache: ptr(nullable · internal fastcc 라 원 인자 &OperationData 에서 cache 필드만 승격된 흔적), &Effect(56B), &Entity)->bool — 잎(계약만): 호출 지점 m07.ll:11905, true 면 스킬 입력 생성 경로 | 4 |  |
| 2 | 미탐색 | safe_move_avoiding_enemy_well(sret Option<Input> 32B, version, &PlayerState, &OperationData, &Entity champ, x, y) — 잎(계약만): 호출 m07.ll:11918 | 4 |  |
| 3 | 미탐색 | abstract_input::skill(sret 32B, version, &mut StdRng, &PlayerState, &OperationData, &PositioningScoreData, &Entity target) — 같은 r14 배치(별도 명세 abstract_input__skill.json) | 4 |  |
| 4 | 미탐색 | self.start_tick(+0x0) 은 이 함수에서 읽지도 쓰지도 않음 | 4 |  |

**`notes` 1건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | knobs 없음 — 이 함수 자체엔 조정 가능한 수치가 없다(전부 콜리 안) | 4 | 사실 서술 |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

