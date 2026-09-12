---

### `28` v21_should_defer_support_target — 지원 사거리 밖의 target 을 지금 좇는 대신 보류할지: 사망임박·국지열세·저HP·비가시 적이면 true

| 항목 | 값 |
|---|---|
| id | `fight_model__v21_should_defer_support_target` |
| 심볼 | `_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old11fight_model31v21_should_defer_support_target` |
| 소스 | `game-ai\src\plan_legacy\old\fight_model.rs:1077` |
| IR | `m10.ll` 48962~49144행 |
| 경로·가시성 | `game_ai::plan_legacy::old::fight_model::v21_should_defer_support_target` · **in:game_ai** |
| 계층 | 레거시 플랜 |
| exe | `14728544` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r7` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, &game_core::Entity, usize, usize, usize, usize, usize) -> bool
```

<details><summary>인자 10개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | version | usize | 본문에서 전혀 안 읽힘(분기·전달 모두 없음) | 4 |
| 1 | 2 | player | &PlayerState(2528B) | info.team(+0x930) 만 읽음 — 마지막 단계 blackboard 인덱스·is_recent_visible 인자 | 4 |
| 2 | 3 | data | &OperationData(24B) | context(+0x8)→setting→tick_per_second, cache(+0x0)→game(dyn), blackboard(+0x10) | 4 |
| 3 | 4 | champ | &Entity(1728B) | 판단 주체 챔피언. team·x/y·hp·stat_cached.hp 를 읽는다 | 4 |
| 4 | 5 | target | &Entity(1728B) | 지원(support) 대상 엔티티. team·x/y 를 읽고 is_recent_visible 에 넘김 | 4 |
| 5 | 6 | near_ally_count | usize | 주변 아군 수(호출자 계산). local_outnumbered 와 L1114 비교에만 | 4 |
| 6 | 7 | near_enemy_count | usize | 주변 적 수 | 4 |
| 7 | 8 | can_near_enemies | usize | '접근 가능한 적' 수(호출자 계산). 열세면 !=0, 아니면 >1 로 쓰임 | 4 |
| 8 | 9 | die_tick | usize | 예상 사망 틱(타워 제외). tps·tps*3·tps*4 와 비교 | 4 |
| 9 | 10 | die_tick_with_tower | usize | 타워 포함 예상 사망 틱. tps 와만 비교 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn v21_should_defer_support_target(version, player, data, champ, target, near_ally_count, near_enemy_count, can_near_enemies, die_tick, die_tick_with_tower) -> bool

[L1089] same_team = (target.team == champ.team)        // TeamType derived eq: 태그 같고, 태그==0(Player) 이면 +0x8 도 같아야
        support_range_sq = if same_team { 14400000000 /*120000^2*/ }
[L1092]                    else { let r = battle::max_range_cached(data, champ, target) + 25000; r*r }
        //  ⚠ IR phi %27: [%18 → (max_range+25000)^2] [%16,%22 → 14400000000]. 인자 순서 (data, champ, target) = champ 가 target 을 때리는 최대 사거리
[L1094] if Entity::distance_sq(target, champ) <= support_range_sq { return false }   // icmp ugt 의 반대 → 사거리 안이면 보류 안 함

[L1098] tps = data.context(+0x8).setting(+0x8).tick_per_second(+0x12f8)
[L1099] hp_ratio = champ.hp(+0x670) * 100 / max(champ.stat_cached.hp(+0x628), 1)
[L1100] local_outnumbered = near_enemy_count > near_ally_count           // (dbg: DIArgList(%6,%5) → i1 %66)
[L1102] if die_tick_with_tower <= tps || die_tick <= tps { return true }   // 1초 안 사망 임박 (IR 은 L1102 를 L1100 보다 먼저 평가하지만 부작용 없어 동치)
[L1106] if local_outnumbered {
            if can_near_enemies != 0 || die_tick <= tps*3 { return true }
[L1110] } else {
            if can_near_enemies > 1 && die_tick <= tps*4 /*shl 2*/ { return true }
        }
[L1114] if near_enemy_count >= near_ally_count && hp_ratio < 55 { return true }
[L1118] if target.team != champ.team {                                     // TeamType ne 인라인(cmp.rs:264)
[L1119]     return !data.blackboard(+0x10)[1 - player.info.team(+0x930)].is_recent_visible(data.cache.game, player, target)
            //  적 target 이 최근 가시(직접 가시 or 120틱 내 목격)가 아니면 보류
        }
[L1120] return false

※ 부작용 없음. version 은 미사용.
※ 반환 phi(%105): %26→false(사거리 안) / %48,%72,%67,%77→true / %82,%89→false(같은 팀) / %93→!is_recent_visible
```

**`mem` 메모리 접근 18건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | Entity(target) | 0x0 | team@tag (TeamType) | r | champ.team 과 비교(entity.rs:1127 derived eq 인라인). 0=Player 면 +0x8 도 비교 | 4 | OK |
| 1 | Entity(target) | 0x8 | team@Player.0 | r | usize 팀 번호 | 4 | OK |
| 2 | Entity(champ) | 0x0 | team@tag (TeamType) | r |  | 4 | OK |
| 3 | Entity(champ) | 0x8 | team@Player.0 | r |  | 4 | OK |
| 4 | Entity(target) | 0x660 | x | r | distance_sq(target, champ) 의 self 쪽 | 4 | OK |
| 5 | Entity(target) | 0x668 | y | r |  | 4 | OK |
| 6 | Entity(champ) | 0x660 | x | r |  | 4 | OK |
| 7 | Entity(champ) | 0x668 | y | r |  | 4 | OK |
| 8 | OperationData | 0x8 | context | r | &GameContext | 4 | OK |
| 9 | GameContext | 0x8 | setting | r | &GameSetting(5432B) | 4 | OK |
| 10 | GameSetting | 0x12f8 | tick_per_second | r | = tps (IR 오프셋 4856). 사망임박 임계의 단위 | 4 | OK |
| 11 | Entity(champ) | 0x670 | hp | r | usize 현재 HP. hp_ratio 분자(*100) | 4 | OK |
| 12 | Entity(champ) | 0x628 | stat_cached.hp | r | usize 최대 HP. max(.,1) 로 0 나눗셈 방지 후 분모 | 4 | OK |
| 13 | PlayerState | 0x930 | info.team | r | enemy_ix = 1 - team. blackboard 인덱스(>=2 면 panic_bounds_check len=2) | 4 | OK |
| 14 | OperationData | 0x10 | blackboard | r | &[Blackboard;2](744B stride). [enemy_ix] 가 is_recent_visible 의 self | 4 | OK |
| 15 | OperationData | 0x0 | cache | r | &AbstractGameWithCache | 4 | OK |
| 16 | AbstractGameWithCache | 0x0 | game (dyn AbstractGame data ptr) | r | is_recent_visible 인자 | 4 | OK |
| 17 | AbstractGameWithCache | 0x8 | game (dyn AbstractGame vtable ptr) | r |  | 4 | OK |

**`consts` 상수 12건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev | folded_from |
|---|---|---|---|---|---|---|
| 0 | 0 | 1089 | 태그 | TeamType 메모리태그 0 = Player(tcxdict --enum TeamType). 태그가 둘 다 0 이면 +0x8 팀 번호까지 비교해 '같은 팀' 판정 | 3 |  |
| 1 | 25000 | 1092 | 계수 | 적(타팀) target 에 대한 지원 사거리 = max_range_cached(data, champ, target) + 25000 (약 0.8칸) | 4 |  |
| 2 | 14400000000 | 1089 | 산출값 | 120000^2 — 같은 팀(또는 둘 다 Neutral) target 에 대한 지원 사거리 제곱(상수 접힘: 120000 리터럴은 본문에 없음, 3.75칸) | 4 |  |
| 3 | 100 | 1099 | 계수 | hp_ratio = champ.hp*100 / max(stat_cached.hp,1) — 백분율 변환 | 4 |  |
| 4 | 1 | 1099 | 임계 | llvm.umax(stat_cached.hp, 1) — 최대HP 0 나눗셈 가드(core::cmp::max 인라인) | 4 |  |
| 5 | 3 | 1106 | 계수 | 국지 열세(적>아군)일 때 die_tick <= tps*3 (3초 안 사망) 이면 보류 | 4 |  |
| 6 | 2 | 1110 | 임계 | 국지 열세 아닐 때 die_tick <= tps*4 — `shl i64 %54, 2` 로 접힘(리터럴 4 없음) | 4 | 4 |
| 7 | 1 | 1110 | 임계 | 국지 열세 아닐 때 can_near_enemies > 1 (접근 가능한 적 2명 이상) 이어야 die_tick 조건과 함께 보류 | 4 |  |
| 8 | 0 | 1106 | 태그 | 국지 열세일 때 can_near_enemies != 0 (1명이라도) 이면 보류 | 4 |  |
| 9 | 55 | 1114 | 임계 | hp_ratio < 55% 이고 near_enemy_count >= near_ally_count 면 보류 | 4 |  |
| 10 | 1 | 1119 | 임계 | enemy_ix = 1 - player.info.team | 4 |  |
| 11 | 2 | 1119 | 임계 | panic_bounds_check 배열 길이(blackboard [;2]) — 판정값 아님. (같은 값 2 가 L1110 의 shl 시프트량으로도 쓰이는데 그건 위 folded_from=4 항목) | 4 |  |

**`knobs` 조정점 7건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 적 target 지원 사거리 여유분 | fight_model.rs:1092 (IR m10.ll `add i64 %19, 25000`) | 25000 | max_range+25000 안의 적 target 은 무조건 보류하지 않는다. 올리면 더 먼 적도 '지원 가능'으로 보아 보류 판정이 줄고 추적이 늘어난다 | 4 | 기존 |
| 1 | 아군 target 지원 사거리 | fight_model.rs:1089 (IR `phi … 14400000000`) | 14400000000 | 120000(3.75칸) 안의 아군 target 은 보류 안 함. 소스 리터럴은 접혀 있어 바꾸려면 r^2 로 | 4 | 기존 |
| 2 | 사망 임박 창(공통) | fight_model.rs:1102 (IR `icmp ule %9/%8, %54`) | 1 | die_tick(_with_tower) <= tps*1 이면 보류. 리터럴 1 은 곱셈 없이 tps 그대로 비교라 본문에 숫자 없음 | 4 | 기존 |
| 3 | 열세 시 사망 창 | fight_model.rs:1106 (IR `mul i64 %54, 3`) | 3 | 적>아군일 때 3초 안 사망이면 보류. 올리면 열세 상황에서 더 보수적 | 4 | 기존 |
| 4 | 비열세 시 사망 창 | fight_model.rs:1110 (IR `shl i64 %54, 2` = tps*4) | 4 | 적<=아군이어도 접근가능 적 2+ 이고 4초 안 사망이면 보류 | 4 | 기존 |
| 5 | 저HP 보류 임계 | fight_model.rs:1114 (IR `icmp ult i64 %61, 55`) | 55 | HP 55% 미만 + 적>=아군이면 보류. 올리면 더 건강해도 지원을 미룬다 | 4 | 기존 |
| 6 | 최근 가시 시간창 (is_recent_visible 내부, 담당 범위 밖) | game-core blackboard.rs:350 (_gcbc g07.ll:157005~ `add i64 %24, 120`) | 120 | 적 target 을 120틱(2초) 안에 본 적 없으면 보류. 별도 define 이라 constants 에 없음 | 4 | 기존 |

<details><summary>`callees` 피호출자 9건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | distance_sq | game_core::utils::distance_sq | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:6 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
| 1 | distance_sq | game_core::Entity::distance_sq | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2157 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
| 2 | is_recent_visible | game_core::Blackboard::is_recent_visible | pub | fn(&game_core::Blackboard, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\game\blackboard.rs:346 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 3 | max_range_cached | game_ai::plan_legacy::old::max_range_cached | pub | fn(&game_core::OperationData, &game_core::Entity, &game_core::Entity) -> u64 | game-ai\src\plan_legacy\old\battle.rs:2332 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | team | game_view::ClientData::team | pub | fn(&game_view::ClientData, usize) -> std::option::Option<std::cell::Ref< game_core::Team>> | game-view\src\logic\client\data.rs:1651 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 5 | team | game_view::ClientDatabase::team | pub | fn(&game_view::ClientDatabase, usize) -> std::option::Option<&game_core::Team> | game-view\src\logic\client\data.rs:968 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 6 | team | game_core::PlayerAiContext::<'a, 'b, 'r>::team | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:583 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 7 | tick_per_second | game_view::view::effect::nightmare::tick_per_second | in:game_view::view::effect::nightmare | fn(&engine_core::assets::Assets) -> f32 | game-view\src\view\effect\nightmare.rs:31 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 8 | v21_should_defer_support_target | game_ai::plan_legacy::old::fight_model::v21_should_defer_support_target | in:game_ai | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, &game_core::Entity, usize, usize, usize, usize, usize) -> bool | game-ai\src\plan_legacy\old\fight_model.rs:1077 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
</details>

⚠**미매칭 2개**: `llvm.umax.i64`, `setting`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 3곳** (m05.ll:21587, m05.ll:30378, m10.ll:15415) · **형제 0개** 

**`open` 5건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | near_ally_count / near_enemy_count / can_near_enemies / die_tick / die_tick_with_tower 의 계산 방식은 호출자 소관(이 함수 밖) — 미탐색 | 4 |  |
| 1 | 표기 불가 | L1089~1092 의 소스 표기: support_range 가 (a) 제곱 전 값으로 있고 L1094 에서 `support_range*support_range` 인지 (b) 처음부터 제곱값인지는 표기 불가. dbg_value 에 `support_range = %20`(제곱 전) 과 `= %27`(제곱 후 phi) 둘 다 있어 (a) 가 유력하나 단정 안 함. 동작(제곱거리 비교)은 확정 | 4 |  |
| 2 | 미탐색 | L1100 local_outnumbered 의 dbg 줄이 1100 인데 IR 은 L1102 비교(%62~%64)를 먼저 낸다 — 컴파일러 재배치(부작용 없음)로 판단, 소스 순서는 1100→1102 로 기재 | 4 |  |
| 3 | 미탐색 | max_range_cached(data, champ, target) 내부(m10.ll:50689, TLS 캐시)는 안 읽음 | 4 |  |
| 4 | 미탐색 | target 이 Neutral(태그 1) 이고 champ 도 Neutral 인 경우 same_team 취급(14400000000) — 실전 champ 는 항상 Player 라 도달 여부 미확인 | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

