---

### `112` nontarget_windup_perceived — 관측자(player)가 적 캐스터의 논타겟 시전 윈드업을 '이미 인지했는가' — 최근시야 AND 경과틱 >= 반응틱(시드고정 추첨)

| 항목 | 값 |
|---|---|
| id | `utils__nontarget_windup_perceived` |
| 심볼 | `_RNvNtCshdEBA0ozCnw_7game_ai5utils26nontarget_windup_perceived` |
| 소스 | `game-ai\src\utils.rs:553` |
| IR | `m04.ll` 53108~53231행 |
| 경로·가시성 | `game_ai::nontarget_windup_perceived` · **pub** |
| 계층 | 기타 |
| exe | `d399a0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r13` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity) -> bool
```

<details><summary>인자 4개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | version | usize (i64 %0) | 본문에서 한 번도 읽지 않음(dbg_value 만). 분기 없음 | 4 |
| 1 | 2 | player | &PlayerState (2528B) noalias readonly | 관측자. 읽기: info.team(0x930)·info.id(0x928)·info.parameter(0x180) | 4 |
| 2 | 3 | data | &OperationData (24B) noalias readonly | cache(+0x0)·blackboard(+0x10) 만 읽음. context(+0x8) 미사용 | 4 |
| 3 | 4 | caster | &Entity (1728B) noalias readonly | 시전자. dbg 이름 caster/self 둘 다 %3. 읽기: ty@tag(0x68)·action_state(0x70/0x78)·id(0x5c0) | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn nontarget_windup_perceived(version, player, data, caster) -> bool {   // utils.rs:553
  // :557  적 팀 블랙보드로 '최근 시야' 판정
  let bb = &data.blackboard[1 - player.info.team];            // L53116~53125 (bounds len 2 → panic)
  if !bb.is_recent_visible(data.cache.game /*&dyn AbstractGame (data,vtable)*/, player, caster) {   // L53130 (game_core 본문 g07.ll:157005)
      return false;                                              // L53144 phi false
  }
  // :560  캐스터의 현재 액션 경과틱 (Entity::action_time 인라인, entity.rs:1593~1609)
  let elapsed: usize = if caster.ty@tag == 13 /*Champion*/ {   // L53138~53141
      match caster.action_state@tag /*+0x70*/ {                  // L53149~53159
          3 /*Attack*/ | 4 /*Skill*/ | 5 /*Skill2*/ | 6 /*Ult*/ => caster.action_state.time /*+0x78*/,
          0 | 1 | 2 => 0,                                        // Idle/Return/Move
      }
  } else { 0 };                                                  // L53189 phi
  // :561  관측자 회피 기준치(로비 유효, 컨디션 미적용)
  let a = player.info.parameter.skill_avoid_base();             // L53191~53192, 0..=100
  // :562~564  캐스터 소유 선수의 skill_hit 기준치, 없으면 50
  let h = data.cache.player_by_champion_id(caster.id)           // L53194~53196 (Option<&PlayerState>, null=None)
              .map(|p| p.info.parameter.skill_hit_base())        // L53204~53205 closure$0 utils.rs:563
              .unwrap_or(50);                                     // L53212
  // :565  시전 시작틱 = 현재틱 − 경과틱 (포화 뺄셈)
  let cast_start = game.tick().saturating_sub(elapsed);         // L53214~53219 vtable+0x28=tick, llvm.usub.sat
  // :566  결정론 시드 = (관측자 id) ^ (캐스터 id << 20) ^ (시전시작틱 << 40)
  let seed = player.info.id ^ (caster.id << 20) ^ (cast_start << 40);   // L53221~53226
  // :567  반응틱 추첨(시드 고정) 과 경과틱 비교
  elapsed >= skill_avoid_react_ticks(seed, a, h)                // L53228~53229 icmp uge
}   // :568

[game_core 콜리 skill_avoid_react_ticks(seed, avoid, hit) — g11.ll:265042 독해]
  d = sat(100 − avoid); d2 = d*d
  lo = 2 + d2*58/10000 ; hi = 6 + d2*114/10000      // avoid100 → 2..6틱, avoid1 → 58..117틱 (docs: '회피 100=2~6틱, 50=16~34틱, 1=59~118틱')
  base = splitmix64(seed) 를 [lo, hi] 균등 사상(widening mul)  // 시드 같으면 같은 값 → 매 평가 재롤 없음
  deception = d * (12 * min(hit,100)^2 / 10000) / 100           // '시전자 기만 12·(hit/100)²·(100−avoid)/100 틱'
  return base + deception

[game_core 콜리 Blackboard::is_recent_visible(bb, game, player, entity) — g07.ll:157005 독해]
  if game.is_visible(player.team, entity.id) /*vtable+0xf8*/ { true }
  else if let Some(p) = game.get_player_by_champion_id(entity.id) /*vtable+0x150*/ { bb.last_visible[p.position.as_index()] + 120 >= game.tick() }   // 120틱 = 2초@60tps
  else { false }
```

**`mem` 메모리 접근 11건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | L53116~53118 `1 - team` 으로 적 블랙보드 인덱스(bounds 2, L53134 panic_bounds_check). tcxdict PlayerState 0x930 | 3 | OK |
| 1 | PlayerState | 0x928 | info.id | r | L53221~53222 시드 재료(관측자 id). tcxdict PlayerState 0x928 | 3 | OK |
| 2 | PlayerState | 0x180 | info.parameter (AthleteParameter 744B) | r | L53191 &parameter → skill_avoid_base(). 또 L53204 caster 소유 PlayerState+0x180 → skill_hit_base() | 4 | OK |
| 3 | OperationData | 0x10 | blackboard (&[Blackboard;2]) | r | L53123~53125 `blackboard[1-team]` (Blackboard 744B stride, gep 타입 {…} 는 Blackboard 레이아웃) | 4 | OK |
| 4 | OperationData | 0x0 | cache (&AbstractGameWithCache) | r | L53126 | 4 | OK |
| 5 | AbstractGameWithCache | 0x0 | game.data_ptr (&dyn AbstractGame) | r | L53127 | 4 | OK |
| 6 | AbstractGameWithCache | 0x8 | game.vtable_ptr (816B vtable) | r | L53128~53129; 슬롯 +0x28(40) = AbstractGame::tick (divtable). L53214~53216 `tick(game)` | 3 | OK |
| 7 | Entity | 0x68 | ty@tag (EntityType) | r | L53138~53140 `== 13` Champion (tcxdict --enum EntityType 메모리태그 13). action_time() 인라인 entity.rs:1593 | 3 | OK |
| 8 | Entity | 0x70 | ty@Champion.0.action_state@tag (ChampionActionState) | r | L53149~53159 switch: 3 Attack/4 Skill/5 Skill2/6 Ult → 경과틱, 0 Idle/1 Return/2 Move → 0. entity.rs:1594 | 4 | OK |
| 9 | Entity | 0x78 | ty@Champion.0.action_state@{Attack,Skill,Skill2,Ult}.time | r | L53165~53184 네 variant 모두 +0x78 `time` (tcxdict Entity 0x78). entity.rs:1598/1601/1604/1607 | 3 | 부분일치(첫 성분(컨테이너)만 맞고 **마지막 성분이 다르다** — 사람이 봐야 ) |
| 10 | Entity | 0x5c0 | id | r | L53194~53195 caster.id → player_by_champion_id 인자 + 시드 재료(`<<20`) | 4 | OK |

**`consts` 상수 11건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 1 | 557 | 인덱스 | `1 - player.team` = 적 팀 인덱스(2팀 전제). L53118. 배열 길이 2 는 bounds check(L53119/L53134) | 4 |
| 1 | 2 | 557 | 임계 | blackboard 배열 길이(bounds check 상한, L53119 `icmp ult %7, 2` · L53134 panic_bounds_check len 2). 판정값 아님 | 4 |
| 2 | 13 | 560 | 태그 | EntityType::Champion 메모리태그(tcxdict --enum EntityType idx13=tag13). 인라인 Entity::action_time entity.rs:1593 — 챔피언이 아니면 elapsed=0 | 3 |
| 3 | 3 | 560 | 태그 | ChampionActionState::Attack 태그(entity.rs:1598) → elapsed = action_state.time | 4 |
| 4 | 4 | 560 | 태그 | ChampionActionState::Skill 태그(entity.rs:1601) | 4 |
| 5 | 5 | 560 | 태그 | ChampionActionState::Skill2 태그(entity.rs:1604) | 4 |
| 6 | 6 | 560 | 태그 | ChampionActionState::Ult 태그(entity.rs:1607) | 4 |
| 7 | 0 | 560 | 태그 | elapsed 기본값 0 — 비챔피언 또는 Idle/Return/Move 상태(L53189 phi). 비교 `elapsed >= react` 에서 react>=2 라 0 이면 항상 false | 4 |
| 8 | 50 | 564 | 산출값 | 캐스터 소유 PlayerState 를 못 찾을 때 skill_hit 기본값 `unwrap_or(50)` (L53212 phi [50, %41]; dbg `default = 50` L53115) | 4 |
| 9 | 20 | 566 | 계수 | 시드 합성 `caster.id << 20` 의 시프트량 (L53223 `shl i64 %46, 20`). 배수 접힘 아님 — 소스도 시프트 | 4 |
| 10 | 40 | 566 | 계수 | 시드 합성 `cast_start << 40` 의 시프트량 (L53225 `shl i64 %57, 40`) | 4 |

**`knobs` 조정점 3건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 캐스터 skill_hit 기본값(소유 선수 미발견 시) | utils.rs:564 (L53212) | 50 | 올리면 기만 가산틱이 커져 인지가 늦어짐(더 못 피함). 실전에선 챔피언은 항상 소유 선수가 있어 거의 안 탐 | 4 | 기존 |
| 1 | 반응틱 곡선(2..6 / 58 / 114 / 기만 12) — 콜리 내부 | game_core ai_interface::skill_avoid_react_ticks (g11.ll:265042) · 이 함수 밖 | lo=2+d²·58/1e4, hi=6+d²·114/1e4, 기만=12·hit²/1e4·d/100 | 여기서는 바꿀 수 없음(game_core). 인지 시점을 바꾸려면 콜리 재현 필요 | 4 | 기존 |
| 2 | 최근시야 유예 120틱 — 콜리 내부 | game_core Blackboard::is_recent_visible (g07.ll:157005~) `+120` | 120 | 올리면 시야를 잃은 직후에도 더 오래 윈드업을 인지함 | 4 | 기존 |

<details><summary>`callees` 피호출자 17건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | as_index | game_core::BigGoal::as_index | pub | fn(&game_core::BigGoal) -> usize | game-core\src\simulation\ai_interface.rs:338 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 1 | as_index | game_core::Position::as_index | pub | fn(&game_core::Position) -> usize | game-core\src\simulation\entity.rs:580 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 2 | get_player_by_champion_id | game_core::AbstractGame::get_player_by_champion_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::PlayerState> | game-core\src\simulation.rs:145 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 3 | get_player_by_champion_id | <game_core::Game as game_core::AbstractGame>::get_player_by_champion_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::PlayerState> | game-core\src\simulation\game.rs:1874 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 4 | get_player_by_champion_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_player_by_champion_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::PlayerState> | game-core\src\simulation\game.rs:3922 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 5 | is_recent_visible | game_core::Blackboard::is_recent_visible | pub | fn(&game_core::Blackboard, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\game\blackboard.rs:346 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 6 | is_visible | game_core::World::is_visible | pub | fn(&game_core::World, usize, usize) -> bool | game-core\src\simulation\game\data.rs:384 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 7 | is_visible | game_core::VisibleState::is_visible | pub | fn(&game_core::VisibleState) -> bool | game-core\src\simulation\game\data.rs:121 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 8 | is_visible | game_core::AbstractGame::is_visible | pub | fn(&Self/#0, usize, usize) -> bool | game-core\src\simulation.rs:125 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 9 | nontarget_windup_perceived | game_ai::nontarget_windup_perceived | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity) -> bool | game-ai\src\utils.rs:553 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 10 | player_by_champion_id | game_core::AbstractGameWithCache::<'a, 'b>::player_by_champion_id | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> std::option::Option<&'a/#0 game_core::PlayerState> | game-core\src\simulation.rs:1915 | True | True | 3 | IR 호출 심볼 일치(토큰 경계) |
| 11 | skill_avoid_base | game_core::AthleteParameter::skill_avoid_base | pub | fn(&game_core::AthleteParameter) -> usize | game-core\src\simulation\state\player.rs:74 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 12 | skill_avoid_react_ticks | game_core::skill_avoid_react_ticks | pub | fn(u64, usize, usize) -> usize | game-core\src\simulation\ai_interface.rs:218 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 13 | skill_hit_base | game_core::AthleteParameter::skill_hit_base | pub | fn(&game_core::AthleteParameter) -> usize | game-core\src\simulation\state\player.rs:69 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 14 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 15 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 16 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
</details>

⚠**미매칭 2개**: `llvm.usub.sat.i64`, `splitmix64`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 19곳** (m02.ll:9755, m02.ll:12054, m02.ll:18778, m02.ll:19129, m02.ll:27882, m02.ll:28036, m02.ll:38550, m02.ll:41158, m02.ll:45231, m14.ll:13936, m14.ll:17057, m14.ll:19119, m14.ll:21616, m14.ll:24693, m14.ll:30481, m14.ll:46163, m15.ll:13812, m15.ll:19131, m15.ll:19655) · **형제 0개** 

**`open` 3건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | version(%0) 은 본문에서 안 읽는다 — 호출자 측 게이트용 인자로 추정(근거: dbg_value 만 있고 load/비교 0건, L53109) | 4 |  |
| 1 | 미탐색 | skill_avoid_react_ticks 의 [lo,hi] 사상이 `(hi-lo+1)==0` 이면 base=raw splitmix 값(L265058 분기) — 실제 lo<=hi 라 도달 불가로 보이나 콜리 명세 범위 밖 | 4 |  |
| 2 | 미탐색 | action_time() 의 Champion 이외 EntityType 도 경과틱을 가질 수 있는지 — 본문은 tag 13 만 취급(entity.rs:1593). 다른 타입은 0 확정, 소스에 다른 arm 이 있었다면 사장 코드 | 4 |  |

**`notes` 1건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | utils.rs:563 map 클로저의 소스 표기(`\|p\| p.info.parameter.skill_hit_base()`)는 인라인이라 추정 — 동작은 L53204~53205 로 확정 | 5 | 사실 서술 |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

