---

### `35` resolve_fight_stake — 교전 저울(resolve_fight_full)을 '묶인 아군을 버린 판' 기준선으로 한 번 더 돌려 차분 판정(FightPrediction)을 낸다

| 항목 | 값 |
|---|---|
| id | `fight_model__resolve_fight_stake` |
| 심볼 | `_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old11fight_model19resolve_fight_stake` |
| 소스 | `game-ai\src\plan_legacy\old\fight_model.rs:571` |
| IR | `m10.ll` 42357~42695행 |
| 경로·가시성 | `game_ai::plan_legacy::old::fight_model::resolve_fight_stake` · **in:game_ai** |
| 계층 | 레거시 플랜 |
| exe | **없음** — 「exe 에 독립 함수가 없다(인라인·`define internal fastcc`)」인지 **「조인 실패」**인지는 이 칸만으로 못 가른다. `dllmatch.py`·`name2rva.py` 로 확인하라 |
| 라운드 | 기준 `r7` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &mut rand::rngs::std::StdRng, &game_core::OperationData, &game_core::PlayerState, &game_core::Entity, &[&game_core::Entity], &[&game_core::Entity], i8, std::option::Option<&game_core::Entity>, usize, &mut game_core::DebugFrameData) -> game_ai::plan_legacy::old::FightPrediction
```

<details><summary>인자 11개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | version | usize | AI 버전. <2 이면 차분 저울 없이 resolve_fight_full 단일 호출로 즉시 반환(L574~575). >=2 이면 스택에 저장돼 ally_is_bound 클로저에 &usize 로 캡처된다 | 4 |
| 1 | 2 | rnd | &mut StdRng(320B, align16) | 본문에서 직접 안 씀. 클로저 캡처(+16)로 ally_is_bound 에만 전달 | 4 |
| 2 | 3 | data | &OperationData(24B) | +0x8 context → +0x0 pool(bumpalo) 만 본문에서 직접 읽음(remaining Vec 할당자). resolve_fight_full·ally_is_bound 에 그대로 전달 | 4 |
| 3 | 4 | player | &PlayerState(2528B) | 본문에서 직접 안 씀. 클로저 캡처(+32)로 ally_is_bound 에만 전달 | 4 |
| 4 | 5 | champ | &Entity(1728B) | 판단 주체 챔피언. id(+0x5c0)·x/y(+0x660/+0x668) 를 읽는다 | 4 |
| 5 | 6 | near_allies | &[&Entity] (ptr %6, len %7) | 근처 아군. 필터 원본이자 absolute/diff 저울의 아군 집합 | 4 |
| 6 | 7 | near_enemies | &[&Entity] (ptr %8, len %9) | 근처 적. 세 저울 모두에 그대로, 클로저(+40/+48)로 ally_is_bound 에도 전달 | 4 |
| 7 | 8 | committed_dir | i8 | 히스테리시스 방향. absolute·diff 에는 그대로, abandon 저울에는 0 고정(L591) | 4 |
| 8 | 9 | tower | Option<&Entity> | 세 저울에 그대로 전달. 본문에서 직접 안 읽음 | 4 |
| 9 | 10 | judge_accuracy | usize | 세 저울에 그대로 전달 | 4 |
| 10 | 11 | debug | &mut DebugFrameData | 클로저 캡처(+56)로 ally_is_bound 에만 전달 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn resolve_fight_stake(version, rnd, data, player, champ, near_allies, near_enemies, committed_dir, tower, judge_accuracy, debug) -> FightPrediction

// ── 경로 A: 구버전 ────────────────────────────────────────────
[L574] if version < 2 {
[L575]   return resolve_fight_full(version, data, champ, near_allies, near_enemies,
                                  committed_dir, tower, judge_accuracy, arrivals=&[], baseline=0)
       }

// ── remaining = '교전에 묶인' 아군(나 제외) ─────────────────────
[L582~585] remaining: Vec<&Entity, &Bump> = near_allies.iter().copied()
             .filter(|a| {                      // ★closure#0 = aux m10.ll 56168~56212 (call_mut 심)
[L584]         a.id(+0x5c0) != champ.id(+0x5c0)  // 자기 자신 제외 (같으면 즉시 false, ally_is_bound 호출 안 함)
               && fight_model::ally_is_bound(version, rnd, data, player, a, near_enemies, debug)
             })
             .collect_in(data.context(+0x8).pool(+0x0))   // bumpalo from_iter_in

// ── 절대 판정 ───────────────────────────────────────────────
[L586] absolute = resolve_fight_full(version, data, champ, near_allies, near_enemies,
                                     committed_dir, tower, judge_accuracy, &[], baseline=0)

// ── 경로 B: 묶인 아군이 없으면 절대 판정 그대로 ─────────────────
[L587] if remaining.len(+0x18) == 0 { drop(remaining); return absolute }

// ── 경로 C: 차분 저울 ─────────────────────────────────────────
[L591] abandon = resolve_fight_full(version, data, champ,
                                    allies = remaining(ptr +0x0, len +0x18),   // ★아군을 '묶인 아군'만으로
                                    near_enemies, committed_dir = 0,           // ★히스테리시스 없음
                                    tower, judge_accuracy, &[], baseline=0)
[L592~593] diff = resolve_fight_full(version, data, champ, near_allies, near_enemies,
                                     committed_dir, tower, judge_accuracy, &[],
                                     baseline = abandon.net_value(+0x30))   // ★'버린 판'의 순가치를 기준선으로
[L594] diff.line_absolute(+0x39) = absolute.line(+0x38)
[L595] if diff.line(+0x38) != absolute.line(+0x38) {
[L597]   diff.rescue_ally(+0x20/+0x28) = remaining.iter()
             .min_by_key(|a| (a.x(+0x660).abs_diff(champ.x))^2 + (a.y(+0x668).abs_diff(champ.y))^2)
             .map(|a| a.id(+0x5c0))              // 나와 가장 가까운 묶인 아군 = 구조 대상
       }
       // 같으면 diff.rescue_ally 는 resolve_fight_full 이 준 값 그대로
[L599] drop(remaining); return diff

※ 세 저울(absolute/abandon/diff)은 모두 arrivals=&[] (합류 없음). 부작용은 rnd/debug 가 ally_is_bound 로 &mut 전달되는 것뿐.
※ 순서: absolute 는 remaining 수집 뒤에, abandon → diff 순으로 호출된다(IR 블록 24→42→50→54).
```

**`mem` 메모리 접근 14건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | OperationData | 0x8 | context | r | &GameContext. L585 | 4 | OK |  |
| 1 | GameContext | 0x0 | pool | r | &bumpalo::Bump — remaining Vec 의 할당자(from_iter_in 3번째 인자). gep 없이 ctx 선두 load 로 접힘 | 4 | OK |  |
| 2 | Entity(champ / near_allies 원소) | 0x5c0 | id | r | usize. ①클로저: a.id != champ.id 로 자기 자신 제외(aux m10.ll:56172~56178) ②L597: 최근접 remaining 아군의 id 를 rescue_ally 에 기록 | 4 | OK |  |
| 3 | Entity(remaining 원소 / champ) | 0x660 | x | r | u64. L597 min_by_key 키 = distance_sq(a, champ) 의 dx | 4 | OK |  |
| 4 | Entity(remaining 원소 / champ) | 0x668 | y | r | u64. 같은 키의 dy | 4 | OK |  |
| 5 | Vec<&Entity>(bumpalo, 32B) remaining | 0x0 | ptr | r | 버퍼 시작(L591 abandon 아군 슬라이스·L597 순회) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 6 | Vec<&Entity>(bumpalo, 32B) remaining | 0x18 | len | r | 0 이면 absolute 그대로 반환(L587) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 7 | FightPrediction(abandon) | 0x30 | net_value | r | i64. diff 저울의 baseline 인자로 넘긴다(L593) | 4 | OK |  |
| 8 | FightPrediction(absolute) | 0x38 | line | r | FightLine 태그. diff.line_absolute 에 복사(L594)되고 diff.line 과 비교(L595) | 4 | OK |  |
| 9 | FightPrediction(diff) | 0x38 | line | r | absolute.line 과 비교 — 다르면 rescue_ally 를 채운다 | 4 | OK |  |
| 10 | FightPrediction(diff → 반환값) | 0x39 | line_absolute | w | L594. 차분 판정이 절대 판정과 어떻게 달랐는지 호출자가 볼 수 있게 보존 | 4 | OK | absolute.line (+0x38) |
| 11 | FightPrediction(diff → 반환값) | 0x20 | rescue_ally@tag | w | L597. diff.line != absolute.line 일 때만. remaining 이 비어 있을 리 없지만(L587 게이트) min_by_key 가 None 이면 0 | 4 | OK | 1(Some) / 0(None) |
| 12 | FightPrediction(diff → 반환값) | 0x28 | rescue_ally@Some.0 | w | L597. 거리는 dx²+dy² (u64 abs_diff 제곱합) | 4 | OK | champ 에서 가장 가까운 remaining 아군의 id(+0x5c0) |
| 13 | sret %0 | 0x0 | FightPrediction 전체(64B) | w | 반환값 | 4 | 확인불가(tcx 사전에 타입 없음) | absolute(경로 A·B) 또는 diff(경로 C, memcpy 64B L599) |

**`consts` 상수 4건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 2 | 574 | 임계 | version < 2 이면 차분 저울 없이 resolve_fight_full 한 번으로 끝(`icmp ult i64 %1, 2`) | 4 |
| 1 | 0 | 591 | 태그 | abandon 저울의 committed_dir = 0 (히스테리시스 없음) — 묶인 아군만 남긴 가정 판이라 기존 커밋 방향을 안 준다 | 4 |
| 2 | 0 | 587 | 태그 | remaining.len == 0 이면 absolute 를 그대로 반환 | 4 |
| 3 | 1 | 597 | 태그 | rescue_ally Option<usize> 의 Some 태그(phi [1,%108]); None 은 0 | 4 |

**`knobs` 조정점 3건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 차분 저울 활성 버전 게이트 | fight_model.rs:574 (IR m10.ll:42385 `icmp ult i64 %1, 2`) | 2 | 올리면 더 높은 버전까지 구버전(절대 판정 단일)로 동작. 내리면(0/1) 모든 버전이 차분 저울을 탄다 | 4 | 기존 |
| 1 | abandon 저울의 committed_dir 고정값 | fight_model.rs:591 (IR m10.ll:42480 `i8 0`) | 0 | 0 = 묶인 아군만 남긴 가정 판에 히스테리시스를 안 준다. committed_dir 를 그대로 넘기면 기존 커밋 방향이 기준선(baseline)에 스며들어 차분이 작아진다 | 4 | 기존 |
| 2 | rescue_ally 선택 기준 | fight_model.rs:597 (IR m10.ll:42640~42660 min_by_key) | distance_sq 최소 | 가장 가까운 묶인 아군을 구조 대상으로 고른다. 기준을 hp 나 위협도로 바꾸면 구조 대상이 달라진다 | 4 | 기존 |

<details><summary>`callees` 피호출자 5건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | ally_is_bound | game_ai::plan_legacy::old::fight_model::ally_is_bound | in:game_ai | fn(usize, &mut rand::rngs::std::StdRng, &game_core::OperationData, &game_core::PlayerState, &game_core::Entity, &[&game_core::Entity], &mut game_core::DebugFrameData) -> bool | game-ai\src\plan_legacy\old\fight_model.rs:531 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 1 | line | game_ai::plan_legacy::types::BigPlan::debug_label::line | in:game_ai::plan_legacy::types | fn(game_core::LineType) -> &str | game-ai\src\plan_legacy\types.rs:83 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
| 2 | line | game_core::TowerType::line | pub | fn(&game_core::TowerType) -> std::option::Option<game_core::LineType> | game-core\src\simulation\entity\tower.rs:90 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
| 3 | resolve_fight_full | game_ai::plan_legacy::old::fight_model::resolve_fight_full | in:game_ai::plan_legacy::old::fight_model | fn(usize, &game_core::OperationData, &game_core::Entity, &[&game_core::Entity], &[&game_core::Entity], i8, std::option::Option<&game_core::Entity>, usize, &[i64], i64) -> game_ai::plan_legacy::old::FightPrediction | game-ai\src\plan_legacy\old\fight_model.rs:324 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | resolve_fight_stake | game_ai::plan_legacy::old::fight_model::resolve_fight_stake | in:game_ai | fn(usize, &mut rand::rngs::std::StdRng, &game_core::OperationData, &game_core::PlayerState, &game_core::Entity, &[&game_core::Entity], &[&game_core::Entity], i8, std::option::Option<&game_core::Entity>, usize, &mut game_core::DebugFrameData) -> game_ai::plan_legacy::old::FightPrediction | game-ai\src\plan_legacy\old\fight_model.rs:571 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
</details>

⚠**미매칭 6개**: `drop  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `line_absolute`, `net_value`, `pool`, `remaining`, `rescue_ally`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 4곳** (m02.ll:33264, m10.ll:16359, m10.ll:16464, m10.ll:43614) · **형제 0개** 

**`open` 4건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | ally_is_bound(m10.ll:39193 define) 의 내부 조건은 안 읽었다 — 이 명세 범위 밖(별도 함수). '묶인 아군' 의 정의는 그쪽 명세 참조 | 4 |  |
| 1 | 미탐색 | resolve_fight_full(m10.ll:39915) 내부에서 baseline 이 net_value/line 에 어떻게 반영되는지는 안 읽었다 — 여기선 '기준선 인자로 abandon.net_value 를 넘긴다' 는 사실만 확정 | 4 |  |
| 2 | 미탐색 | 경로 A·B 에서 반환되는 absolute 의 line_absolute(+0x39) 값은 resolve_fight_full 이 정한다(이 함수는 안 건드림) — 값 미확인 | 4 |  |
| 3 | 미탐색 | L597 의 `.map(\|a\| a.id)` 는 closure#2(597:90), 키 closure#1(597:52) 로 tcx 에 있으나 IR 에는 인라인돼 별도 define 이 없다(fnparts: 서브프로그램 73 vs define 5). 극성·순서는 본문 phi(%113/%114)로 확정 | 3 |  |

**`notes` 1건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | fnparts 가 잡은 m11.ll 33299·m12.ll 25293·m01.ll 44965 조각은 심볼상 maintain_distance_ally_side/from_iter_in 제네릭 공유 인스턴스라 aux 에 넣지 않았다(이 함수 고유 술어는 m10.ll 56168~56212 call_mut 심 하나) | 4 | 사실 서술 |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

