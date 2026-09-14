---

### `154` SmallActionUlt::get_input — 소액션 '궁극기 시전': 대상 존재·적 우물 위험·레벨(>4)·논타겟 검사 후 abstract_input::ult 로 입력 생성, Ult 이면 is_act 세움

| 항목 | 값 |
|---|---|
| id | `SmallActionUlt__get_input` |
| 심볼 | `_RNvMs5_NtNtCshdEBA0ozCnw_7game_ai12small_action4castNtB5_14SmallActionUlt9get_input` |
| 소스 | `game-ai\src\small_action\cast.rs:261` |
| IR | `m07.ll` 12657~12788행 |
| 경로·가시성 | `game_ai::SmallActionUlt::get_input` · **in:game_ai** |
| 계층 | 기타 |
| exe | `d84a30` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r14` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&mut game_ai::SmallActionUlt, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData) -> std::option::Option<game_core::Input>
```

<details><summary>인자 7개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | Option<Input>(32B) | 반환 슬롯. 아래 returns 참조 | 4 |
| 1 | 1 | &mut self | &mut SmallActionUlt(24B) | +0x8 target(usize 엔티티 id) 읽기 · +0x10 is_act 쓰기. +0x0 start_tick 은 안 씀 | 4 |
| 2 | 2 | version | usize | 본문 분기 없음 — is_enemy_well_danger·safe_move·ult 에 전달만 | 4 |
| 3 | 3 | rnd | &mut StdRng(320B) | check_nontarget·abstract_input::ult 에 전달만(본문에서 직접 안 읽음) | 4 |
| 4 | 4 | player | &PlayerState(2528B) | readonly. info.team(+0x930)·info.position(+0x9c0) | 4 |
| 5 | 5 | data | &OperationData(24B) | readonly. +0x0 cache → game(dyn) get_entity_by_id · player_champion | 4 |
| 6 | 6 | positioning_score | &PositioningScoreData(2760B) | abstract_input::ult 에 전달만 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn get_input(&mut self, version, rnd, player, data, positioning_score) -> Option<Input>   [cast.rs:261~282]
  L262: target = data.cache.game.get_entity_by_id(self.target)   // vtable+0x1f0
        None → L280: return None (sret +0x0 = -1)
  L263: champ = data.cache.player_champion[player.team][player.position].unwrap()   // None 이면 패닉(unwrap_failed)
  L264: if target.team == TeamType::Player(1 - player.team) {           // 적 소속이면
  L265:   if path_finder::is_enemy_well_danger(version, player, target.x, target.y) {
  L266:     return None } }
  L269: eff = champ.ult_effect().unwrap()      // 인라인: if champ.level > 4 { &champ.ult_effect(+0x538) } else { &None(정적 @anon.31) } → None 이면 패닉
        if check_nontarget(rnd, player, data.cache, eff, target) {   // 잎: true = 논타겟 검사 통과(시전 가능)
  L273:   input = abstract_input::ult(version, rnd, player, data, positioning_score, target)
  L274:   if input.tag == 5 (Some(Input::Ult)) { L275: self.is_act = true }
          return input
        } else {
  L270:   return safe_move_avoiding_enemy_well(version, player, data, champ, target.x, target.y)   // 잎 sret
        }
```

**`mem` 메모리 접근 16건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | SmallActionUlt | 0x8 | target | r | 대상 엔티티 id → game.get_entity_by_id (m07.ll:12670~12674) | 4 | OK |  |
| 1 | OperationData | 0x0 | cache | r | &AbstractGameWithCache (m07.ll:12666) | 4 | OK |  |
| 2 | AbstractGameWithCache | 0x0 | game | r | &dyn AbstractGame: data ptr +0x0, vtable +0x8 → 슬롯 +0x1f0 get_entity_by_id (m07.ll:12667~12675) | 4 | OK |  |
| 3 | AbstractGameWithCache | 0x1e0 | player_champion[team][position] | r | None → unwrap_failed 패닉 (m07.ll:12698~12703) | 4 | OK |  |
| 4 | PlayerState | 0x930 | info.team | r | ≥2 → panic_bounds_check | 4 | OK |  |
| 5 | PlayerState | 0x9c0 | info.position@tag | r | i32 → zext 인덱스 | 4 | OK |  |
| 6 | Entity | 0x0 | team@tag | r | target.team: 0=Player (L264 인라인 entity.rs:1127) | 4 | OK |  |
| 7 | Entity | 0x8 | team@Player.0 | r | == 1 - my team 이면 적 (m07.ll:12721~12729) | 4 | OK |  |
| 8 | Entity | 0x660 | x | r | target.x → is_enemy_well_danger / safe_move (L265·L270) | 4 | OK |  |
| 9 | Entity | 0x668 | y | r | target.y | 4 | OK |  |
| 10 | Entity | 0x568 | ult_effect@tag | r | champ.ult_effect None(-1) → unwrap_failed 패닉 (m07.ll:12734~12742). 페이로드 선두+0x30 위치의 i32 태그 | 4 | OK |  |
| 11 | Entity | 0x538 | ult_effect@Some.0 | r | &Effect(56B) → check_nontarget 인자 (m07.ll:12737~12738 select — level 게이트 실패 시 @anon.31 정적 None 으로 대체) | 4 | OK |  |
| 12 | Entity | 0x5c8 | level | r | level(+0x5c8) > 4 여야 ult_effect 를 봄(entity.rs:1701 인라인 Entity::ult_effect) — 아니면 정적 None(@anon.31) 을 unwrap → 패닉 (m07.ll:12736) | 4 | OK |  |
| 13 | SmallActionUlt | 0x10 | is_act | w | L275 · m07.ll:12786 — abstract_input::ult 결과 태그가 5(Some(Input::Ult)) 일 때만. 그 외 경로는 self 무변경 | 4 | OK | 1 (true, i8) |
| 14 | (sret) | 0x0 | Option<Input>@tag | w | L280 대상 없음(m07.ll:12686) · L266 적 우물 위험(m07.ll:12754). 8B 만 기록 | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) | -1 (None) |
| 15 | (sret) | 0x0 | Option<Input> 전체 | w | L270 safe_move_avoiding_enemy_well(sret) / L273 abstract_input::ult(sret) — 32B 중 live 는 콜리 규약 | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) | 콜리가 채움 |

**`consts` 상수 6건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | -1 | 280 | 센티널 | Option<Input>::None 니치 태그(store i64 -1, m07.ll:12686·12754). 같은 -1 이 Entity.ult_effect None 태그(i32) 비교(L269, m07.ll:12742)에도 | 4 |
| 1 | 2 | 263 | 임계 | team 인덱스 bounds(len 2) — 임계 아님 | 4 |
| 2 | 0 | 264 | 태그 | TeamType::Player 메모리태그 0 — target.team 이 Player 인지 (m07.ll:12713) | 4 |
| 3 | 1 | 264 | 산출값 | `1 - team` = 적 팀 (m07.ll:12722) | 4 |
| 4 | 5 | 274 | 태그 | Input::Ult 메모리태그 5 — abstract_input::ult 결과가 Some(Ult) 인지 (m07.ll:12778) | 4 |
| 5 | 4 | 269 | 임계 | `level > 4` — Entity::ult_effect 인라인(entity.rs:1701) 궁극기 해금 레벨 게이트 (m07.ll:12736). 미달이면 정적 None → unwrap 패닉 | 4 |

**`knobs` 조정점 1건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 궁극기 해금 레벨(인라인된 game_core 게이트) | entity.rs:1701 (Entity::ult_effect, m07.ll:12736 인라인) | 4 | game_core 상수 — 이 함수에선 미달 시 unwrap 패닉이므로 호출측이 먼저 걸러야 한다. 값 자체를 바꾸는 노브는 아님(정보성) | 4 | 기존 |

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
| 9 | ult_effect | game_core::Entity::ult_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1700 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
</details>

**호출처 1곳** (m11.ll:43087) · **형제 9개** (SmallActionUlt)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::SmallActionUlt as std::clone::Clone>::clone | pub | game-ai\src\small_action\cast.rs:239 | True | fn(&game_ai::SmallActionUlt) -> game_ai::SmallActionUlt |
| 1 | <game_ai::SmallActionUlt as std::fmt::Debug>::fmt | pub | game-ai\src\small_action\cast.rs:239 | True | fn(&game_ai::SmallActionUlt, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 2 | game_ai::SmallActionUlt::new | pub | game-ai\src\small_action\cast.rs:247 | False | fn(&game_core::OperationData, usize) -> game_ai::SmallActionUlt |
| 3 | game_ai::SmallActionUlt::movement_landing_position | in:game_ai | game-ai\src\small_action\cast.rs:255 | False | fn(&game_ai::SmallActionUlt, &game_core::Entity, &game_core::OperationData) -> std::option::Option<(u64, u64)> |
| 4 | game_ai::SmallActionUlt::get_input | in:game_ai | game-ai\src\small_action\cast.rs:261 | False | fn(&mut game_ai::SmallActionUlt, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData) -> std::option::Option<game_core::Input> |
| 5 | game_ai::SmallActionUlt::update_state | in:game_ai | game-ai\src\small_action\cast.rs:283 | False | fn(&mut game_ai::SmallActionUlt, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 6 | game_ai::SmallActionUlt::get_action | in:game_ai | game-ai\src\small_action\cast.rs:286 | True | fn(&game_ai::SmallActionUlt) -> game_core::SmallAction |
| 7 | game_ai::SmallActionUlt::is_end | in:game_ai | game-ai\src\small_action\cast.rs:289 | False | fn(&game_ai::SmallActionUlt, &mut rand::rngs::std::StdRng, usize, &game_core::PlayerState, &game_core::OperationData) -> bool |
| 8 | game_ai::SmallActionUlt::is_complete | in:game_ai | game-ai\src\small_action\cast.rs:298 | True | fn(&game_ai::SmallActionUlt) -> bool |

**`open` 6건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | is_enemy_well_danger(version, &PlayerState, x, y)->bool (m03.ll:144500) 내부는 안 봄 — 잎 22 아님, 시그니처만 | 4 |  |
| 1 | 미탐색 | check_nontarget(&mut StdRng, &PlayerState, cache: ptr(nullable · internal fastcc 라 원 인자 &OperationData 에서 cache 필드만 승격된 흔적), &Effect(56B), &Entity)->bool — 잎(계약만): 호출 지점 m07.ll:12759 | 4 |  |
| 2 | 미탐색 | safe_move_avoiding_enemy_well(sret Option<Input> 32B, version, &PlayerState, &OperationData, &Entity champ, x, y) — 잎(계약만): 호출 m07.ll:12772 | 4 |  |
| 3 | 미탐색 | abstract_input::ult(sret 32B, version, &mut StdRng, &PlayerState, &OperationData, &PositioningScoreData, &Entity target) — r14 배치 G 담당 아님(m04.ll:43967) — 시그니처만 (호출 m07.ll:12776) | 4 |  |
| 4 | 미탐색 | self.start_tick(+0x0) 은 이 함수에서 읽지도 쓰지도 않음 | 4 |  |
| 5 | 미탐색 | @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.31 = 56B 정적 Option<Effect>::None(+0x30 에 FF FF FF FF) — 레벨 미달 시 unwrap 패닉 경로가 실제 도달 가능한지(호출측 게이트)는 이 함수 범위 밖 | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

