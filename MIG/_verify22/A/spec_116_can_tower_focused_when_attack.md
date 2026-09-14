---

### `116` can_tower_focused_when_attack — 내가 공격하러 들어가면 이 적 타워가 나를 물(focus) 수 있는가 — 타워 활성 틱·타워 현재 표적(nearest_enemy)·사거리 안 아군 미니언 수로 판정

| 항목 | 값 |
|---|---|
| id | `tower_discipline__can_tower_focused_when_attack` |
| 심볼 | `_RNvNtCshdEBA0ozCnw_7game_ai16tower_discipline29can_tower_focused_when_attack` |
| 소스 | `game-ai\src\tower_discipline.rs:83` |
| IR | `m07.ll` 50962~51097행 |
| 경로·가시성 | `game_ai::can_tower_focused_when_attack` · **pub** |
| 계층 | 기타 |
| exe | `d98740` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r13` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_core::GameContext, &game_core::AbstractGameWithCache, &game_core::PlayerState, &game_core::Entity) -> bool
```

<details><summary>인자 4개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | context | &GameContext(64B) | +0x8 setting 만 읽음 → setting.tower_attack_disable_tick | 4 |
| 1 | 2 | cache | &AbstractGameWithCache(8840B) | game(&dyn AbstractGame 팻포인터 +0 data/+8 vtable)·player_champion·iter_minions 호출 | 4 |
| 2 | 3 | player | &PlayerState(2528B) | info.team·info.position 만 | 4 |
| 3 | 4 | tower | &Entity(1728B) | 적 타워 엔티티(ty 태그 2 = Tower 검사). 술어 클로저 캡처로도 전달 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn can_tower_focused_when_attack(context, cache, player, tower) -> bool {
  // L84 — 타워 공격 비활성 구간(경기 초반)이면 어떤 타워도 못 문다
  if !(cache.game.tick() < context.setting.tower_attack_disable_tick) { return false; }   // vtable+0x28 tick · GameSetting+0x13f8
  (|| {   // L88~119 즉시호출 클로저(인라인)
    // L89
    let team = player.info.team;                                  // +0x930 (<2 아니면 panic)
    let champ = cache.player_champion[team][player.info.position]?;   // cache+0x1e0 · None → false
    let EntityType::Tower { info } = &tower.ty else { return false; };   // Entity+0x68 == 2 (dbg 줄 없음)
    // L92
    if let Some((_, id)) = info.nearest_enemy {                   // Entity+0x88 태그 · +0x98 = .1
        // L93 — 타워가 이미 나를 조준 중
        if id == champ.id { return true; }                        // Entity+0x5c0
        // L97~103 — 타워 사거리 안에 있는 우리 팀 미니언 수
        let cnt = cache.iter_minions(team)
            .filter(|m| tower.attack_effect.as_ref().unwrap().is_in_range(tower, m))   // L98 · Entity+0x490 · None 이면 panic
            .count();
        cnt < 2                                                    // ★L103: 미니언 2기 미만이면 내가 물린다
    } else {
        // L108~111 — 타워가 아무도 안 물고 있으면: 사거리 안 미니언이 하나라도 있어야 안전
        let in_range_minions = cache.iter_minions(team)
            .any(|m| tower.attack_effect.as_ref().unwrap().is_in_range(tower, m));      // L109
        !in_range_minions
    }
  })()
}

분기 순서(IR): %19(tick<disable) → %23(team bounds) → %37(champ!=null && ty==2) → %41(nearest_enemy Some?) → Some: %47(id==champ.id → true) / %62(cnt<2) ; None: %50(!any).
순회 = iter_minions(team) 의 3 슬라이스 Chain(Chain(top,mid),bottom 추정 — g15.ll:102624 미독해) 순. count 는 전량 순회, any 는 첫 매치에서 중단. 두 술어 모두 Effect::is_in_range(&tower.attack_effect, caster=tower, target=minion).
```

**`mem` 메모리 접근 14건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | AbstractGameWithCache | 0x0 | game.data_ptr (&dyn AbstractGame) | r | m07:50974 | 4 | OK |
| 1 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r | vtable+0x28 = AbstractGame::tick (divtable) 호출 (m07:50975~50979, L84) | 3 | OK |
| 2 | GameContext | 0x8 | setting (&GameSetting) | r | m07:50980~50981 | 4 | OK |
| 3 | GameSetting | 0x13f8 | tower_attack_disable_tick | r | usize · `tick < tower_attack_disable_tick` 아니면 false (m07:50982~50985, L84) | 4 | OK |
| 4 | PlayerState | 0x930 | info.team | r | <2 아니면 panic_bounds_check (m07:50999~51005, L89) | 4 | OK |
| 5 | PlayerState | 0x9c0 | info.position@tag | r | [5 x ptr] 인덱스 (m07:51010~51015, L89 · 인라인 콜리 줄 581) | 4 | OK |
| 6 | AbstractGameWithCache | 0x1e0 | player_champion[team][position]@Some.0 | r | null=None → false (m07:51013~51018,51023) | 4 | OK |
| 7 | Entity(tower) | 0x68 | ty@tag | r | i64 == 2 (EntityType::Tower) 아니면 false (m07:51020~51023) — 이 load 는 !dbg 없음(L89~90 사이 추정, `champ.is_some() && matches!(tower.ty, Tower{..})` 형태) | 4 | OK |
| 8 | Entity(tower) | 0x88 | ty@Tower.info.nearest_enemy@tag | r | Option<(usize,usize)> 태그(i64 trunc→i1, 1=Some) (m07:51028~51031, L92) | 4 | OK |
| 9 | Entity(tower) | 0x98 | ty@Tower.info.nearest_enemy@Some.0.1 | r | usize = 타워 현재 표적 엔티티 id (dbg `id`) (m07:51034~51035, L92). .0(+0x90)은 안 읽음 | 4 | OK |
| 10 | Entity(champ) | 0x5c0 | id | r | usize · nearest_enemy.1 == champ.id → true (m07:51037~51040, L93) | 4 | OK |
| 11 | Entity(tower, aux) | 0x4c0 | attack_effect@tag (Option<Effect> 니치) | r | i32 == -1(None) 이면 `unwrap_failed` 패닉 (m11:30348~30351,30380 · m11:24869~24873,24918, L98/L109) | 4 | OK |
| 12 | Entity(tower, aux) | 0x490 | attack_effect@Some.0 (&Effect 56B) | r | Effect::is_in_range 의 &self (m11:30352,30413 · m11:24872,24903) | 4 | OK |
| 13 | Filter<Chain,..>(로컬 sret 56B+8) | 0x38 | predicate(캡처 tower) | r | store ptr tower → iter+56 (m07:51057~51058, L98) — 로컬 alloca 쓰기, 인자 쓰기 아님 | 4 | 확인불가(tcx 사전에 타입 없음) |

**`consts` 상수 5건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 2 | 89 | 임계 | info.team bounds-check(len 2) — 판정값 아님 (m07:51001,51005) | 4 |
| 1 | 2 | 89 | 태그 | EntityType 메모리태그 2 = Tower — `icmp eq i64 %35, 2` (m07:51022, tcxdict --enum EntityType idx2=tag2) | 3 |
| 2 | 2 | 103 | 임계 | ★사거리 안 아군 미니언 수 임계: `cnt < 2` → true(타워가 나를 문다). 미니언이 2기 이상이면 타워가 미니언을 계속 물어 안전 (m07:51090) | 4 |
| 3 | 0 | 100 | 미상 | count() fold 초기값 (m07:51079 `i64 0`) — 판정값 아님 | 4 |
| 4 | -1 | 98 | 센티널 | (aux) Entity+0x4c0 == -1 = attack_effect None 니치 → unwrap_failed (m11:30351 · m11:24871) · 또 m07:51060 `default = -1` 은 size_hint 상한 None 표기 | 4 |

**`knobs` 조정점 2건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 안전 미니언 수 임계 | tower_discipline.rs:103 (m07:51090) | 2 | 올리면(예 3) 타워가 다른 표적을 물고 있어도 미니언이 3기 미만이면 '물린다'로 판정 → 타워 진입이 더 보수적. 내리면(1) 미니언 1기만 있어도 안전 판정 → 다이브 성향↑ | 4 | 기존 |
| 1 | 타워 공격 비활성 틱 | GameSetting+0x13f8 tower_attack_disable_tick (m07:50982~50985, L84) | 설정값(런타임) | 게임 설정 필드 — 이 틱 이상이면 항상 false(타워 비활성). 코드 상수 아님 | 4 | 기존 |

<details><summary>`callees` 피호출자 6건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | can_tower_focused_when_attack | game_ai::can_tower_focused_when_attack | pub | fn(&game_core::GameContext, &game_core::AbstractGameWithCache, &game_core::PlayerState, &game_core::Entity) -> bool | game-ai\src\tower_discipline.rs:83 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 1 | is_in_range | game_core::Effect::is_in_range | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\effect.rs:63 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 2 | iter_minions | game_core::AbstractGameWithCache::<'a, 'b>::iter_minions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5753 ~ game_core[6a30]::simulation::{impl#5}::iter_minions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1847 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 3 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 4 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 5 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
</details>

⚠**미매칭 4개**: `llvm.assume`, `llvm.memcpy.p0.p0.i64`, `size_hint`, `try_fold`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 3곳** (m07.ll:51318, m14.ll:20640, m14.ll:48787) · **형제 0개** 

**`open` 5건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | `iter_minions(cache, team)`(g15.ll:102624) 이 돌려주는 3 슬라이스의 정체(top/mid/bottom_minions[team] 추정 — cache 필드 배치 0x10/0x50/0x90 근거) 는 내부 미독해 | 4 |  |
| 1 | 미탐색 | `Effect::is_in_range(&Effect, caster:&Entity, target:&Entity)->bool`(g06.ll:51634) 내부는 안 읽음 — 사거리 비교로 추정(이름·인자만 확정) | 4 |  |
| 2 | 미탐색 | L97 count 경로에서 Chain::size_hint(fastcc m07:63266) 호출 + assume(cnt <= upper) 는 count() 인라인 잔재로 판정과 무관(관측 사실만) | 4 |  |
| 3 | 미탐색 | v22_can_tower_focused_when_attack(tower_discipline.rs:477, m07:51312) 은 별도 함수 — 이 명세 범위 밖 | 4 |  |
| 4 | 미탐색 | 타워의 attack_effect 가 None 이면 미니언 순회 중 unwrap_failed 패닉(m11:30380/24918). 실전 타워는 항상 Some 이라 도달 안 함(추정 — 런타임 미확인) | 5 |  |

**`notes` 1건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | Entity+0x68 == 2 비교(m07:51020~51023)에 !dbg 가 없어 소스 줄(89 또는 90)과 표기(`let EntityType::Tower{info}` vs `matches!`) 는 미확정 — 동작(태그≠2 → false) 은 확정 | 4 | 사실 서술 |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

