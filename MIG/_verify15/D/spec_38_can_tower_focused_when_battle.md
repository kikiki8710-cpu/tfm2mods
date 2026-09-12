---

### `38` can_tower_focused_when_battle — (x,y)+여유 d 가 어느 적 타워의 (사거리+양쪽 반지름+15000) 안에 들어 타워에 맞을 수 있는 위치인가

| 항목 | 값 |
|---|---|
| id | `tower_discipline__can_tower_focused_when_battle` |
| 심볼 | `_RNvNtCshdEBA0ozCnw_7game_ai16tower_discipline29can_tower_focused_when_battle` |
| 소스 | `game-ai\src\tower_discipline.rs:56` |
| IR | `m07.ll` 51100~51309행 |
| 경로·가시성 | `game_ai::can_tower_focused_when_battle` · **pub** |
| 계층 | 기타 |
| exe | `14256336` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r7` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_core::GameContext, &game_core::AbstractGameWithCache, &game_core::PlayerState, u64, u64, u64) -> bool
```

<details><summary>인자 6개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | context | &GameContext(64B) | pool(+0x0)=towers Vec 할당자, setting(+0x8).tower_attack_disable_tick(+0x13f8) | 4 |
| 1 | 2 | cache | &AbstractGameWithCache(8840B) | game(dyn, tick 호출)·player_champion(+0x1e0)·towers() | 4 |
| 2 | 3 | player | &PlayerState(2528B) | info.team(+0x930)·info.position 태그(+0x9c0) 로 내 챔프 조회 | 4 |
| 3 | 4 | x | u64 | 검사할 좌표 x | 4 |
| 4 | 5 | y | u64 | 검사할 좌표 y | 4 |
| 5 | 6 | d | u64 | 추가 여유 거리(호출자가 주는 마진). 타워 사거리에 더해진다 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn can_tower_focused_when_battle(context, cache, player, x, y, d) -> bool

[L57] if !(cache.game.tick() /*vtable+0x28*/ < context.setting(+0x8).tower_attack_disable_tick(+0x13f8)) { return false }
      // 타워 공격 비활성 시각 이후엔 타워 위협 없음

// ── 클로저(L61~77, 본문에 인라인) ─────────────────────────────
[L62] team = player.info.team(+0x930)
      let Some(champ) = cache.player_champion(+0x1e0)[team][player.info.position 태그(+0x9c0)] else { return false }
[L64] towers = cache.towers(1 - team, context.pool)          // 적 타워 Vec<&Entity,&Bump>
[L66] return towers.iter().any(|t| {
[L67]   let Some(attack) = t.attack_effect(+0x4c0 != -1) else { return false }
[L68]   range = attack.range(t)                               // Effect::range 인라인(effect.rs:26):
              = attack.range(+0x4a0) + (t.level(+0x5c8) - 1) * attack.growth_range(+0x4a8) + t.stat_buff_cached.range(+0x438)
[L69]   dist = utils::distance(x, y, t.x(+0x660), t.y(+0x668))
[L70]   dist <= range + t.radius() + champ.radius() + d + 15000
              // Entity::radius() 인라인(entity.rs:1511~1515): mult=stat_buff_cached.radius_mult(+0x470);
              //   mult==0 ? radius(+0x680) : radius*(100+mult)/100
              // IR: %113 = dist >u 합 → 다음 타워 / 아니면 즉시 true
      })
      // towers 는 반환 전 drop

※ 부작용 없음. 합산 순서(IR): (d+15000) + attack.range + stat_range + (level-1)*growth + t.radius + champ.radius.
```

**`mem` 메모리 접근 19건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | AbstractGameWithCache | 0x0 | game (&dyn AbstractGame data) | r | vtable+0x28 tick() 호출 | 4 | OK |
| 1 | AbstractGameWithCache | 0x8 | game vtable | r |  | 4 | OK |
| 2 | GameContext | 0x8 | setting | r | &GameSetting | 4 | OK |
| 3 | GameSetting | 0x13f8 | tower_attack_disable_tick | r | usize. tick >= 이 값이면 즉시 false (IR 오프셋 5112) | 4 | OK |
| 4 | GameContext | 0x0 | pool | r | &Bump. towers() 할당자 (ctx 선두 load) | 4 | OK |
| 5 | PlayerState | 0x930 | info.team | r | usize. >=2 면 panic_bounds_check | 4 | OK |
| 6 | PlayerState | 0x9c0 | info.position@tag | r | i32 → [5] 인덱스 | 4 | OK |
| 7 | AbstractGameWithCache | 0x1e0 | player_champion[team][pos] | r | [[Option<&Entity>;5];2]. None → false | 4 | OK |
| 8 | Vec<&Entity>(bumpalo,32B) towers | 0x0 | ptr | r |  | 4 | 확인불가(tcx 사전에 타입 없음) |
| 9 | Vec<&Entity>(bumpalo,32B) towers | 0x18 | len | r | 0 이면 false | 4 | 확인불가(tcx 사전에 타입 없음) |
| 10 | Entity(적 타워 t) | 0x4c0 | attack_effect@tag | r | i32 니치. -1(None) 이면 그 타워는 건너뜀 | 4 | OK |
| 11 | Entity(t) | 0x4a0 | attack_effect.range | r | u64 기본 사거리 | 4 | OK |
| 12 | Entity(t) | 0x4a8 | attack_effect.growth_range | r | u64 레벨당 사거리 증가 | 4 | OK |
| 13 | Entity(t) | 0x5c8 | level | r | (level-1)*growth_range | 4 | OK |
| 14 | Entity(t) | 0x438 | stat_buff_cached.range | r | usize 사거리 버프 가산 (Effect::range 인라인 effect.rs:26) | 4 | OK |
| 15 | Entity(t) | 0x660 | x | r | utils::distance 인자 | 4 | OK |
| 16 | Entity(t) | 0x668 | y | r |  | 4 | OK |
| 17 | Entity(t / champ) | 0x470 | stat_buff_cached.radius_mult | r | i32 %. Entity::radius() 인라인(entity.rs:1511~1515): 0 이면 radius 그대로, 아니면 radius*(100+mult)/100 | 4 | OK |
| 18 | Entity(t / champ) | 0x680 | radius | r | usize 충돌 반지름 | 4 | OK |

**`consts` 상수 7건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 2 | 62 | 임계 | team 인덱스 상한(panic_bounds_check len=2) | 4 |
| 1 | 1 | 64 | 미상 | enemy = 1 - team → cache.towers(enemy) | 4 |
| 2 | -1 | 67 | 센티널 | attack_effect Option 니치 None — 공격 이펙트 없는 타워는 건너뜀 | 4 |
| 3 | -1 | 68 | 태그 | Effect::range 인라인: (level - 1) * growth_range | 4 |
| 4 | 15000 | 70 | 오프셋가감 | 추가 여유 마진 (셀 32000 의 약 절반). dist <= range + 반지름들 + d + 15000 | 4 |
| 5 | 0 | 70 | 태그 | Entity::radius 인라인: radius_mult == 0 이면 곱셈 생략 | 4 |
| 6 | 100 | 70 | 계수 | Entity::radius 인라인: radius * (100 + mult) / 100 (퍼센트) | 4 |

**`knobs` 조정점 3건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 타워 위험 판정 추가 마진 | tower_discipline.rs:70 (IR m07.ll:51201 `add i64 %5, 15000`) | 15000 | 올리면 타워 사거리보다 더 바깥까지 '맞을 수 있는 위치'로 봐 전투 중 타워 근처를 더 피하고, 내리면 사거리 경계까지 붙는다 | 4 | 기존 |
| 1 | 타워 공격 활성 구간 게이트 | tower_discipline.rs:57 (IR m07.ll:51118 `icmp ult i64 %14, %18`) — 값은 GameSetting.tower_attack_disable_tick(+0x13f8) | setting.tower_attack_disable_tick | 설정값을 내리면 그 틱 이후 이 함수가 항상 false → AI 가 타워를 무시. 게임 규칙 설정이라 AI 노브라기보다 세팅 | 4 | 기존 |
| 2 | 호출자 여유 d | tower_discipline.rs:56 인자 | 호출자 지정 | 호출자가 큰 d 를 주면 더 보수적으로 타워권을 넓게 본다 | 4 | 기존 |

<details><summary>`callees` 피호출자 15건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | attack_effect | game_core::Entity::attack_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1684 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 1 | can_tower_focused_when_battle | game_ai::can_tower_focused_when_battle | pub | fn(&game_core::GameContext, &game_core::AbstractGameWithCache, &game_core::PlayerState, u64, u64, u64) -> bool | game-ai\src\tower_discipline.rs:56 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 2 | distance | game_core::utils::distance | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:12 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 3 | level | <game_ai::WindowStatView<'_> as game_core::AbstractEntity>::level | pub | fn(&game_ai::WindowStatView</#0>) -> usize | game-ai\src\fight_check.rs:96 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 4 | level | game_core::AbstractEntity::level | pub | fn(&Self/#0) -> usize | game-core\src\simulation\entity.rs:718 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 5 | level | <game_core::Entity as game_core::AbstractEntity>::level | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:772 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 6 | radius | game_core::Entity::radius | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1509 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 7 | range | game_core::Effect::range | pub | fn(&game_core::Effect, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:25 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 8 | team | game_view::ClientData::team | pub | fn(&game_view::ClientData, usize) -> std::option::Option<std::cell::Ref< game_core::Team>> | game-view\src\logic\client\data.rs:1651 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 9 | team | game_view::ClientDatabase::team | pub | fn(&game_view::ClientDatabase, usize) -> std::option::Option<&game_core::Team> | game-view\src\logic\client\data.rs:968 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 10 | team | game_core::PlayerAiContext::<'a, 'b, 'r>::team | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:583 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 11 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 12 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 13 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 14 | towers | game_core::AbstractGameWithCache::<'a, 'b>::towers | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize, &'b/#1 bumpalo::Bump) -> bumpalo::collections::vec::Vec< &game_core::Entity> | game-core\src\simulation.rs:1859 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
</details>

⚠**미매칭 4개**: `drop  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `player_champion`, `setting`, `tower_attack_disable_tick`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m10.ll:49244) · **형제 0개** 

**`open` 2건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | AbstractGameWithCache::towers(team, pool) 가 살아있는 타워만 주는지(파괴된 타워 제외 여부)는 game_core 미독 | 4 |  |
| 1 | 미탐색 | tick() 은 divtable(ExpectedGame 정적 vtable) 기준 슬롯 +0x28 이름 — 런타임 구현체 동일 여부 미확인 | 3 |  |

**`notes` 1건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | _docs 의 v47 주석("can_tower_focused 와 동형이되 시즈 스탠스…")은 자매 함수 것으로 보이며 이 함수엔 스탠스 분기가 없다(IR 상 타워 대상 확인 없음) | 4 | 사실 서술 |

<details><summary>`closed` 1건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

| # | 물음 | 닫은 근거 |
|---|---|---|
| 0 | utils::distance(x,y,x2,y2) 의 정확한 정의(유클리드 정수 sqrt 인지)는 game_core 미독 — 제곱비교가 아니라 실거리 비교라는 사실만 확정(dist 와 사거리 합을 직접 비교) | 본문에 해소 표기가 있다 |
</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

