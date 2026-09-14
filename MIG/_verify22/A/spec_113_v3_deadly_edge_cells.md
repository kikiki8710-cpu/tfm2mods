---

### `113` v3_deadly_edge_cells — v3 위험가격: 적 타워 1기의 한 방 피해로 내 챔피언이 셀 1칸을 건너는 동안 받는 기대피해를 HP 화폐로 환산해 '치명 엣지 셀 수'(2..60)를 낸다

| 항목 | 값 |
|---|---|
| id | `tower_discipline__v3_deadly_edge_cells` |
| 심볼 | `_RNvNtCshdEBA0ozCnw_7game_ai16tower_discipline20v3_deadly_edge_cells` |
| 소스 | `game-ai\src\tower_discipline.rs:304` |
| IR | `m07.ll` 49636~49773행 |
| 경로·가시성 | `game_ai::v3_deadly_edge_cells` · **pub** |
| 계층 | 기타 |
| exe | `d97700` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r13` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &game_core::PlayerState, &game_core::OperationData) -> u32
```

<details><summary>인자 3개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | version | usize | AI 버전 게이트. `icmp ult %0, 2`(L305) — version<2 이면 즉시 2 반환 | 4 |
| 1 | 2 | player | &PlayerState(2528B) | 읽기 전용(readonly). info.team·info.position 만 읽음 | 4 |
| 2 | 3 | data | &OperationData(24B) | 읽기 전용. +0 cache(&AbstractGameWithCache) · +8 context(&GameContext) 만 읽음. +0x10 blackboard 은 안 읽음 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn v3_deadly_edge_cells(version, player, data) -> i32 {
  // L305
  if version < 2 { return 2; }
  // L308
  let team = player.info.team;                 // +0x930, <2 아니면 panic_bounds_check
  let pos  = player.info.position as usize;    // +0x9c0 태그
  let champ = match data.cache.player_champion[team][pos] { Some(e) => e, None => return 2 };   // cache+0x1e0 + team*40 + pos*8
  // L311~313 : 적 타워(넥서스 제외) 중 '공격 효과가 있는' 첫 타워의 (한 방 기대피해, 공격 쿨)
  let (shot, cool) = match data.cache.iter_towers_without_nexus(1 - team)
        .find_map(|t| {
            let eff = t.attack_effect.as_ref()?;                      // Entity+0x4c0 == -1 → None → 다음 타워
            Some((eff.expected_damage_target(data.context, t as &dyn AbstractEntity, champ),   // Effect+0 = Entity+0x490
                  t.attack_cooltime()))
        }) { Some(v) => v, None => return 2 };
  // L316
  let edge_ticks = max(32000 / max(champ.stat_cached.move_speed, 1), 1);   // 셀 1칸 통과 틱
  // L317
  let edge_dmg = shot.saturating_mul(edge_ticks) / max(cool, 1);            // 한 칸 건너는 동안 기대피해
  // L318
  let cells = (edge_dmg.saturating_mul(30) / max(champ.hp, 1)) as i32 + 2;  // 30/현재HP 화폐
  // L319~320
  min(cells, 60)
}

분기 순서(IR): %6(version<2) → %10(team bounds) → %21(champ null) → %27(find_map tag) → 산술 → umin. 반환 phi(m07:49694) = [2,%3 ; 2,%12 ; 2,%28 ; %61,%53].
find_map 순회 순서 = Chain: 고정 슬롯 [Option<&Entity>;6] (인덱스 0..6 순, null skip) → 추가 타워 슬라이스 순. '첫' 매치에서 멈춘다(try_fold Break) — 어느 타워를 집는지는 iter_towers_without_nexus 의 배열 순서에 종속(내부는 game_core g15.ll:108971 미독해).
```

**`mem` 메모리 접근 10건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | usize · <2 아니면 panic_bounds_check(len 2) (m07:49651~49656, L308) | 4 | OK |
| 1 | PlayerState | 0x9c0 | info.position@tag | r | i32 태그(Top0/Jungle1/Mid2/Bottom3/…) → zext 해 [5 x ptr] 인덱스 (m07:49661~49667, L308 · 인라인 콜리 줄 581) | 4 | OK |
| 2 | OperationData | 0x0 | cache | r | &AbstractGameWithCache (m07:49664) | 4 | OK |
| 3 | OperationData | 0x8 | context | r | &GameContext — find_map 클로저 캡처 f[0] (m07:49678~49679, L312) | 4 | OK |
| 4 | AbstractGameWithCache | 0x1e0 | player_champion[team][position]@Some.0 | r | Option<&Entity> 니치(null=None) · cache+0x1e0 + team*40 + position*8 (m07:49665~49669). None 이면 2 반환 | 4 | OK |
| 5 | Entity(champ=내 챔피언) | 0x640 | stat_cached.move_speed | r | usize · 32000/max(speed,1) = 셀 1칸 통과 틱 (m07:49709~49714, L316) | 4 | OK |
| 6 | Entity(champ=내 챔피언) | 0x670 | hp | r | usize 현재 HP · 분모 max(hp,1) (m07:49760~49765, L318) | 4 | OK |
| 7 | Entity(tower=적 타워, aux) | 0x4c0 | attack_effect@tag (Option<Effect> 니치 = casting@tag) | r | i32 == -1 이면 None → 이 타워 skip (m06:35720~35723 · m11:34830~34833, 클로저 L312 · 인라인 콜리 줄 742) | 4 | OK |
| 8 | Entity(tower=적 타워, aux) | 0x490 | attack_effect@Some.0 (&Effect 56B) | r | expected_damage_target 의 &self (m06:35741,35750 · m11:34836,34845, L313) | 4 | OK |
| 9 | Chain<..> 이터레이터 상태(aux 로컬, sret 120B) | 0x0 | a: Option<Flatten<IntoIter<Option<&Entity>,6>>> (tag -1=None) | r | m06:35621~35623 · +0x8/+0x10 = IntoIter alive 범위 · +0x18 = [Option<&Entity>;6] 데이터 · +0x68 = b: Option<Copied<Iter>> (null=None, m06:35734~35737) | 4 | 확인불가(tcx 사전에 타입 없음) |

**`consts` 상수 10건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 2 | 305 | 임계 | version 임계: `icmp ult i64 %0, 2` → version<2(v0/v1) 이면 계산 없이 2 반환 (m07:49646) | 4 |
| 1 | 2 | 308 | 임계 | info.team 배열 길이 bounds-check(len 2) — 판정값 아님 (m07:49652,49656) | 4 |
| 2 | 1 | 311 | 인덱스 | `sub nuw nsw i64 1, %team` = 상대 팀 인덱스(1-team) → iter_towers_without_nexus(cache, 적팀) (m07:49675) | 4 |
| 3 | 32000 | 316 | 계수 | 셀 1칸 크기(좌표단위). edge_ticks = max(32000 / max(move_speed,1), 1) = 셀 1칸 건너는 데 걸리는 틱 (m07:49714) | 4 |
| 4 | 1 | 316 | 인덱스 | 0 나눗셈 방지 바닥 `llvm.umax(x,1)` 4곳: move_speed·edge_ticks(L316)·cool(L317)·hp(L318) (m07:49713,49717,49739,49764) | 4 |
| 5 | 30 | 318 | 계수 | HP 화폐 계수: cells = (edge_dmg.saturating_mul(30) / max(hp,1)) + 2 — 개발자 주석 '가격 공식 30/현재HP' (m07:49745) | 4 |
| 6 | 2 | 318 | 임계 | 기본 셀 가산 `add i32 %59, 2` — 풀피여도 최소 2셀 (m07:49767) · 동시에 조기반환 기본값 2 (m07:49694 phi) | 4 |
| 7 | 60 | 319 | 임계 | 상한 `llvm.umin.i32(cells, 60)` (m07:49771) | 4 |
| 8 | -1 | 312 | 센티널 | (aux) ①Entity+0x4c0 == -1 = attack_effect None 니치 (m06:35722) ②saturating_mul 오버플로 시 usize::MAX (m07:49735,49758 phi) ③Chain.a None 태그 (m06:35622,35773) | 4 |
| 9 | 6 | 311 | 길이 | (aux) 고정 타워 슬롯 수 [Option<&Entity>;6] — 배열 길이(assume ult 6, m06:35694) · 판정값 아님 | 4 |

**`knobs` 조정점 5건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 위험가격 HP 계수 | tower_discipline.rs:318 (m07:49745) | 30 | 올리면 같은 피해/HP 비율에서 셀 수가 커져 타워 근처 엣지가 더 비싸진다(우회 성향↑). 내리면 타워 관통이 싸진다 | 4 | 기존 |
| 1 | 기본 가산 셀 | tower_discipline.rs:318 (m07:49767) · 조기반환값 305/308/313 (m07:49694) | 2 | 풀피·타워없음 등 모든 경우의 하한. 올리면 타워 밴드가 항상 두꺼워진다 | 4 | 기존 |
| 2 | 셀 수 상한 | tower_discipline.rs:319 (m07:49771) | 60 | 저HP 때 밴드가 사실상 벽이 되는 상한. 내리면 저HP 우회가 약해진다 | 4 | 기존 |
| 3 | 셀 1칸 크기(좌표단위) | tower_discipline.rs:316 (m07:49714) | 32000 | 좌표 변환 상수 — 노브 아님(게임 좌표계 고정) | 4 | 기존 |
| 4 | 버전 게이트 | tower_discipline.rs:305 (m07:49646) | 2 | version<2 면 항상 2. 내리면 구버전 AI 도 v3 가격을 쓴다 | 4 | 기존 |

<details><summary>`callees` 피호출자 4건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | attack_cooltime | game_core::Entity::attack_cooltime | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1766 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | expected_damage_target | game_core::Effect::expected_damage_target | pub | fn(&game_core::Effect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + , &game_core::Entity) -> usize | game-core\src\simulation\effect.rs:91 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 2 | iter_towers_without_nexus | game_core::AbstractGameWithCache::<'a, 'b>::iter_towers_without_nexus | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5750 ~ game_core[6a30]::simulation::{impl#5}::iter_towers_without_nexus::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1830 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 3 | v3_deadly_edge_cells | game_ai::v3_deadly_edge_cells | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData) -> u32 | game-ai\src\tower_discipline.rs:304 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
</details>

⚠**미매칭 6개**: `find_map`, `llvm.umax.i64`, `llvm.umin.i32`, `llvm.umul.with.overflow.i64`, `saturating_mul`, `try_fold`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 5곳** (m02.ll:63739, m08.ll:93233, m08.ll:100049, m08.ll:103662, m08.ll:107176) · **형제 0개** 

**`open` 5건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | `Effect::expected_damage_target`(g06.ll:52355) · `Entity::attack_cooltime`(g06.ll:66510) · `iter_towers_without_nexus`(g15.ll:108971) 내부는 안 읽음(game_core 경계 콜리 — 시그니처만 확정: expected_damage_target(&Effect,&GameContext,&dyn AbstractEntity(data,vtable),&Entity)->usize · attack_cooltime(&Entity)->usize range(3,0)=≥3 · iter_towers_without_nexus(&cache, team)->Chain 120B) | 4 |  |
| 1 | 미탐색 | find_map 이 '첫' 타워에서 멈추므로 적 타워가 여럿이면 어느 타워의 (shot,cool) 이 쓰이는지는 iter_towers_without_nexus 배열 순서에 달렸다 — 그 순서(라인/티어 순?)는 g15.ll 미독해 | 4 |  |
| 2 | 미탐색 | L312 클로저 캡처 f[8]=champ 가 expected_damage_target 의 target(5번째 인자) 으로 들어가고, 타워는 caster(&dyn AbstractEntity, vtable=@anon.eb49…6 = Entity vtable) 로 들어간다 — 인자 역할은 g06.ll DISubprogram !63700 arg 이름(self/context/-/target)으로 확정, 3번째(caster) 이름은 팻포인터라 dbg 에 없음 | 4 |  |
| 3 | 미탐색 | info.position 태그 4 (5번째 variant) 가 무엇인지 안 봄 — [5 x ptr] 라 5슬롯 전부 유효 인덱스 | 4 |  |
| 4 | 미탐색 | `add i32 %59, 2` 는 nsw 없음 → trunc 후 i32 wrap 가능(edge_dmg*30/hp 가 2^31 초과 시). 실전 값에선 도달 어려움(추정) | 5 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

