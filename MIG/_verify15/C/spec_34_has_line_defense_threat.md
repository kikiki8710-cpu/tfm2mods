---

### `34` has_line_defense_threat — 해당 라인의 미니언 웨이브가 우리 쪽으로 밀렸고(from_mid<-3000 또는 count<-2), 라인 근처 적 미니언 중 tower_id 를 노리는 놈이 있으면 true

| 항목 | 값 |
|---|---|
| id | `defense_nexus__has_line_defense_threat` |
| 심볼 | `_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old13defense_nexus23has_line_defense_threat` |
| 소스 | `game-ai\src\plan_legacy\old\defense_nexus.rs:587` |
| IR | `m04.ll` 60400~60577행 |
| 경로·가시성 | `game_ai::plan_legacy::old::has_line_defense_threat` · **pub** |
| 계층 | 레거시 플랜 |
| exe | `13886640` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r7` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_core::PlayerState, &game_core::OperationData, game_core::LineType, usize) -> bool
```

<details><summary>인자 4개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | player | &PlayerState(2528B) | info.team(+0x930) 만 읽는다 | 4 |
| 1 | 2 | data | &OperationData(24B) | +0x0 cache(minions 호출) / +0x8 context(pool·is_near_line) / +0x10 blackboard | 4 |
| 2 | 3 | line | LineType(i8, range 0..3) | 0 Top / 1 Mid / 2 Bottom (tcxdict --enum LineType). minion_state 선택과 is_near_line 인자 | 3 |
| 3 | 4 | tower_id | usize | 방어 대상 타워 엔티티 id. 적 미니언의 nearest_enemy 와 비교 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn has_line_defense_threat(player: &PlayerState, data: &OperationData, line: LineType, tower_id: usize) -> bool

[L588] team = player.info.team (+0x930)                                  // >=2 → panic
        ms: &BrainMinionParameter = data.blackboard(+0x10)[team].minion_state(line)
            // blackboard.rs:379~382 인라인: Top→+0x0 top_minion_state / Mid→+0x28 mid_minion_state / Bottom→+0x50 bottom_minion_state

[L589] wave_is_pushed_in = ms.from_mid(+0x10) < -3000            // i64, 먼저 검사
                        || ms.minion_count(+0x20) < -2           // i32, 앞이 거짓일 때만
[L590] if !wave_is_pushed_in { return false }

[L594] enemy_minions: Vec<&Entity,&Bump> = data.cache.minions(1 - team, data.context.pool(+0x0))
        result = enemy_minions.iter().any(|m| {                  // closure$0 인라인, 순서대로 단락
[L595]      map_regions::is_near_line(data.context, m.x(+0x660), m.y(+0x668), line)   // _gcbc g09.ll:157890
[L599]      && m.ty(+0x68) == EntityType::Minion(1)
[L600]      && m.ty.Minion.info.nearest_enemy(+0x88 tag / +0x90 val) == Some(tower_id)
        })
[L604] drop(enemy_minions); return result

※ 즉 '우리 라인 웨이브가 밀린 상태' 이면서 '그 라인 근처의 적 미니언이 이 타워를 가장 가까운 적으로 잡고 있음' 이면 위협.
   웨이브가 안 밀렸으면 미니언 목록을 아예 안 만든다(minions 호출 자체를 건너뜀).
※ 부작용 없음(bump Vec 임시 할당 후 drop).
```

**`mem` 메모리 접근 17건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | gep 2352. >=2 → panic_bounds_check(len 2) (m04.ll:60408~60420) | 4 | OK |
| 1 | OperationData | 0x10 | blackboard | r | &[Blackboard;2]. [team] 을 744B stride gep 로 인덱싱 (m04.ll:60414) | 4 | OK |
| 2 | Blackboard | 0x0 | top_minion_state: BrainMinionParameter(40B) | r | line==Top(0) 일 때 (blackboard.rs:379~382 minion_state 인라인) | 4 | OK |
| 3 | Blackboard | 0x28 | mid_minion_state | r | line==Mid(1), gep 40 | 4 | OK |
| 4 | Blackboard | 0x50 | bottom_minion_state | r | line==Bottom(2), gep 80 | 4 | OK |
| 5 | BrainMinionParameter | 0x10 | from_mid: i64 | r | gep 16. < -3000 이면 웨이브 밀림 (m04.ll:60441~60443) | 4 | OK |
| 6 | BrainMinionParameter | 0x20 | minion_count: i32 | r | gep 32. < -2 이면 웨이브 밀림 (m04.ll:60446~60448) | 4 | OK |
| 7 | OperationData | 0x0 | cache | r | &AbstractGameWithCache — minions(1-team, pool) 의 self | 4 | OK |
| 8 | OperationData | 0x8 | context | r | &GameContext(64B). is_near_line 1인자 | 4 | OK |
| 9 | GameContext | 0x0 | pool: &Bump | r | minions() 결과 Vec 의 할당자 (m04.ll:60457) | 4 | OK |
| 10 | Vec<&Entity>(bumpalo, 32B) | 0x0 | ptr | r | 적 미니언 목록 시작 (m04.ll:60461) | 4 | OK |
| 11 | Vec<&Entity>(bumpalo, 32B) | 0x18 | len | r | 0 이면 any=false. 끝 = ptr + len*8 (shl 3) | 4 | OK |
| 12 | Entity | 0x660 | x | r | gep 1632 — is_near_line 인자 | 4 | OK |
| 13 | Entity | 0x668 | y | r | gep 1640 | 4 | OK |
| 14 | Entity | 0x68 | ty (EntityType 태그) | r | gep 104. ==1 → Minion 이어야 다음 검사 (tcxdict --enum EntityType) | 3 | OK |
| 15 | Entity | 0x88 | ty.Minion.info.nearest_enemy (Option<usize> 태그) | r | gep 136 = enum+0x8 + Minion+0x18. trunc to i1 → Some 여부 | 4 | OK |
| 16 | Entity | 0x90 | ty.Minion.info.nearest_enemy.Some.0 | r | gep 144. == tower_id 비교 (m04.ll:60515~60517) | 4 | OK |

**`consts` 상수 5건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 2 | 588 | 태그 | blackboard 팀 배열 길이 2 (panic_bounds_check). 또 LineType::Bottom 태그 2 (switch case) | 4 |
| 1 | 0 | 588 | 태그 | LineType::Top 태그 0 → top_minion_state (+0x0) | 4 |
| 2 | 1 | 588 | 태그 | LineType::Mid 태그 1 → mid_minion_state (+0x28). 또 enemy team = 1 - team, EntityType::Minion 태그 1 (L599) | 4 |
| 3 | -3000 | 589 | 임계 | minion_state.from_mid < -3000 이면 웨이브가 우리 쪽으로 밀린 것으로 판정 (i64 slt) | 4 |
| 4 | -2 | 589 | 임계 | minion_state.minion_count < -2 이면 웨이브 밀림 판정 (i32 slt). 두 조건은 OR | 4 |

**`knobs` 조정점 2건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 웨이브 밀림 판정 — from_mid 임계 | defense_nexus.rs:589 (IR m04.ll:60443 `icmp slt i64 %22, -3000`) | -3000 | blackboard 의 from_mid(미니언 전선의 중앙 기준 부호 있는 거리, 추정)가 이 값보다 작으면 밀림. 값을 0 쪽으로 올리면(예 -1000) 조금만 밀려도 위협으로 보고, 더 내리면(-6000) 깊이 밀려야 위협으로 본다 | 4 | 기존 |
| 1 | 웨이브 밀림 판정 — minion_count 임계 | defense_nexus.rs:589 (IR m04.ll:60448 `icmp slt i32 %26, -2`) | -2 | blackboard 의 minion_count(아군-적 미니언 수 차, 추정 — 음수=적이 많음)가 -3 이하면 밀림. -1 로 올리면 적 미니언이 2마리만 많아도 위협 | 4 | 기존 |

<details><summary>`callees` 피호출자 8건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | has_line_defense_threat | game_ai::plan_legacy::old::has_line_defense_threat | pub | fn(&game_core::PlayerState, &game_core::OperationData, game_core::LineType, usize) -> bool | game-ai\src\plan_legacy\old\defense_nexus.rs:587 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 1 | is_near_line | game_core::is_near_line | pub | fn(&game_core::GameContext, u64, u64, game_core::LineType) -> bool | game-core\src\simulation\map_regions.rs:139 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 2 | minion_state | game_core::Blackboard::minion_state | pub | fn(&game_core::Blackboard, game_core::LineType) -> &game_core::BrainMinionParameter | game-core\src\simulation\game\blackboard.rs:378 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 3 | minions | game_core::AbstractGameWithCache::<'a, 'b>::minions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize, &'b/#1 bumpalo::Bump) -> bumpalo::collections::vec::Vec< &game_core::Entity> | game-core\src\simulation.rs:1853 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | nearest_enemy | game_core::Entity::nearest_enemy | pub | fn(&game_core::Entity) -> std::option::Option<usize> | game-core\src\simulation\entity.rs:1819 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 5 | team | game_view::ClientData::team | pub | fn(&game_view::ClientData, usize) -> std::option::Option<std::cell::Ref< game_core::Team>> | game-view\src\logic\client\data.rs:1651 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 6 | team | game_view::ClientDatabase::team | pub | fn(&game_view::ClientDatabase, usize) -> std::option::Option<&game_core::Team> | game-view\src\logic\client\data.rs:968 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 7 | team | game_core::PlayerAiContext::<'a, 'b, 'r>::team | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:583 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
</details>

⚠**미매칭 4개**: `drop  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `from_mid`, `minion_count`, `pool`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m04.ll:58442) · **형제 0개** 

**`open` 5건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | from_mid / minion_count 의 정확한 의미(부호 규약·단위)는 BrainMinionParameter 를 채우는 game_core 쪽(blackboard 갱신처)을 안 읽어 추정 — 필드명과 음수 임계로 '우리 쪽으로 밀린 웨이브'로 해석 | 5 |  |
| 1 | 재료 부재 | defense_nexus.rs:589 의 `\|\|` 소스 표기 순서는 column 부재로 확정 불가 — IR 단락 순서(from_mid 먼저) 만 확정 | 4 |  |
| 2 | 미탐색 | is_near_line(context,x,y,line) 의 '근처' 기준 거리는 _gcbc g09.ll:157890 본문을 안 읽어 미탐색 | 4 |  |
| 3 | 미탐색 | AbstractGameWithCache::minions(team, pool) 이 '그 팀 소유 미니언' 을 돌려주는지(live 만인지)는 _gcbc g15.ll:109546 본문 미탐색 — 인자명(team) 으로 1-team = 적 미니언으로 해석 | 4 |  |
| 4 | 표기 불가 | closure$0 의 L599 `ty == Minion` 검사가 소스에서 `if let EntityType::Minion(info)` 인지 `is_minion()` 헬퍼인지는 표기 불가(인라인·외연 동일) | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

