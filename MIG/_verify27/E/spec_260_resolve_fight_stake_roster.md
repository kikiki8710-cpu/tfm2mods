---

### `260` resolve_fight_stake_roster — 합류 편성(roster)으로 교전 저울 3회 — 절대 판정 / 묶인 아군만 남긴 포기 판정 / 포기 net 기준 차분 판정 → 차분을 채택하되 라인이 갈리면 rescue_ally(최근접 묶인 아군) 표시

| 항목 | 값 |
|---|---|
| id | `fight_model__resolve_fight_stake_roster` |
| 심볼 | `_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old11fight_model26resolve_fight_stake_roster` |
| 소스 | `game-ai\src\plan_legacy\old\fight_model.rs:645` |
| IR | `m10.ll` 47870~48401행 |
| 경로·가시성 | `game_ai::plan_legacy::old::fight_model::resolve_fight_stake_roster` · **in:game_ai** |
| 계층 | 레거시 플랜 |
| exe | `e0b030` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r18` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &game_core::OperationData, &game_core::Entity, &[(&game_core::Entity, i64, bool)], &[&game_core::Entity], i8, std::option::Option<&game_core::Entity>, usize) -> game_ai::plan_legacy::old::FightPrediction
```

<details><summary>인자 11개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) absolute→반환 | FightPrediction 64B | `dead_on_unwind noalias writable sret([64 x i8]) captures(none) dereferenceable(64)` · ⚠`writeonly` 아님 — L664 에서 `absolute.line`(+0x38) 을 되읽는다(m10.ll:48195~48196). initializes 속성 없음. DI 이름 `absolute`(첫 resolve_fight_full 의 sret 으로 직접 씀 m10.ll:48003) | 4 |
| 1 | 1 | version | usize | `noundef` · 본문 분기 없음 — resolve_fight_full ×3 에 그대로 전달 | 4 |
| 2 | 2 | data | &OperationData(24B) | `noalias readonly captures(none) dereferenceable(24)` · L648 data.context.pool(bump) 만 직접 읽고(m10.ll:47909~47911) 나머지는 콜리 전달 | 4 |
| 3 | 3 | champ | &Entity(1728B) | `noalias readonly captures(address, read_provenance) dereferenceable(1728)` · id(@0x5c0)·x/y(@0x660/0x668) 를 필터(L656)·최근접 키(L666)에서 읽음 | 4 |
| 4 | 4 | roster.data_ptr | &[(&Entity, i64, bool)] 팻포인터 data | `noalias nonnull readonly captures(address, read_provenance)` · 원소 24B: +0 &Entity(DI `a`) · +8 i64(도착 틱 → arrivals) · +16 bool(DI `bound`, 묶임 여부). tcx sig 확정 `&[(&Entity, i64, bool)]` | 3 |
| 5 | 4 | roster.len | usize | `range(i64 0, 384307168202282326)` = 2^63/24 → stride 24 확정 | 4 |
| 6 | 5 | near_enemies.data_ptr | &[&Entity] data | `noalias nonnull readonly` · 본문에서 안 읽음 — 세 콜리에 그대로 전달 | 4 |
| 7 | 5 | near_enemies.len | usize | `range(i64 0, 1152921504606846976)` = 2^60 → stride 8 | 4 |
| 8 | 6 | committed_dir | i8 | `noundef` · 절대·차분 호출엔 그대로, 포기 호출엔 0 고정(m10.ll:48127). 본문 분기 없음(값 의미는 콜리 소관 — _docs: 이탈 커밋 = -1) | 4 |
| 9 | 7 | tower | Option<&Entity> | `noalias readonly dereferenceable_or_null(1728)` · null=None. 본문에서 안 읽음 — 세 콜리에 전달 | 4 |
| 10 | 8 | judge_accuracy | usize | `noundef` · 본문 분기 없음 — 세 콜리에 전달 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
// fight_model.rs:645~647  pub(crate) fn resolve_fight_stake_roster(version, data:&OperationData, champ:&Entity, roster:&[(&Entity /*a*/, i64 /*arrival*/, bool /*bound*/)], near_enemies:&[&Entity], committed_dir:i8, tower:Option<&Entity>, judge_accuracy:usize) -> FightPrediction
// L648~649
bump = data.context.pool
allies:   bumpalo Vec<&Entity> = new_in(bump)
arrivals: bumpalo Vec<i64>     = new_in(bump)
// L650~652  (편성 전원 → 절대 판정 입력)
for (a, arrival, _bound) in roster { allies.push(a); arrivals.push(arrival) }
// L654  ① 절대 판정 — sret 에 직접
absolute = resolve_fight_full(version, data, champ, &allies, near_enemies, committed_dir, tower, judge_accuracy, &arrivals, baseline=0)
// L655~656  묶인 아군(자기 제외)만
remaining: bumpalo Vec<&Entity> = roster.iter().filter(|(a,_,bound)| *bound && a.id != champ.id).map(|(a,_,_)| *a).collect_in(bump)
// L658
if remaining.is_empty() { return absolute }          // 묶인 아군 없음 → 절대 판정 그대로(rescue_ally·line_absolute 는 콜리가 쓴 값)
// L661  ② 포기 판정 — 묶인 아군만, 커밋 없음(dir 0), 도착 없음, baseline 0
abandon = resolve_fight_full(version, data, champ, &remaining, near_enemies, 0, tower, judge_accuracy, &[], 0)
// L662~663  ③ 차분 판정 — ①과 같은 입력에 baseline = abandon.net_value
diff = resolve_fight_full(version, data, champ, &allies, near_enemies, committed_dir, tower, judge_accuracy, &arrivals, abandon.net_value)
// L664
diff.line_absolute = absolute.line
// L665~666  라인이 갈렸으면(차분이 절대를 뒤집음) 근거 아군 = champ 에서 가장 가까운 remaining
if diff.line != absolute.line {
   diff.rescue_ally = remaining.iter().min_by_key(|a| ((a.x-champ.x)²+(a.y-champ.y)² /*u64 절대차 제곱합*/, a.id)).map(|a| a.id)   // remaining.len>0 이므로 항상 Some
}
// L668~669
return diff   // (drops: remaining · arrivals · allies)

// 분기 순서: remaining 비었나(L658) → 라인 갈림(L665). version/tower/judge_accuracy/near_enemies/committed_dir 은 이 함수에서 분기 없음.
// rnd 인자 없음 · gen_range 0회. 세 콜리 호출 순서 = absolute → abandon → diff (모두 같은 data 로 — 콜리가 TLS/캐시를 쓰면 이 순서가 관측 순서).
```

**`mem` 메모리 접근 17건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | OperationData | 0x8 | context (&GameContext) | r | m10.ll:47909~47910 L648 | 4 | OK |  |
| 1 | GameContext | 0x0 | pool (&bumpalo::Bump) | r | m10.ll:47911 — allies/arrivals/remaining 세 bumpalo Vec 의 할당자(DI `bump`) | 4 | OK |  |
| 2 | (&Entity,i64,bool) | 0x0 | a: &Entity | r | m10.ll:47955 L651 allies.push(a) · aux m01.ll:28092 map 클로저 · aux m10.ll:56556 필터 | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) |  |
| 3 | (&Entity,i64,bool) | 0x8 | arrival tick i64 | r | m10.ll:48025~48026 L652 arrivals.push | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) |  |
| 4 | (&Entity,i64,bool) | 0x10 | bound: bool | r | aux m10.ll:56548~56551 필터 L656 첫 조건(false 면 id 비교 생략 — `&&` 단락) | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) |  |
| 5 | Entity | 0x5c0 | id (usize) | r | aux m10.ll:56557~56561 필터 `a.id != champ.id` · m10.ll:48333~48334·48357~48358 L666 키/결과 id · aux m12.ll:25542~25543 | 4 | OK |  |
| 6 | Entity | 0x660 | x (u64) | r | m10.ll:48304~48305(a)·48312~48313(champ) L666 인라인 거리(entity.rs:2158 → \|dx\| 계산 3147:7) · aux m12.ll:25513~25522 | 4 | OK |  |
| 7 | Entity | 0x668 | y (u64) | r | m10.ll:48308~48317 · aux m12.ll:25517~25526 | 4 | OK |  |
| 8 | FightPrediction(absolute, sret) | 0x38 | line (FightLine u8) | r | m10.ll:48195~48196 L664 되읽기 → diff.line_absolute · L665 비교 | 4 | OK |  |
| 9 | FightPrediction(diff, 로컬 %15) | 0x38 | line | r | m10.ll:48203~48204 L665 `diff.line != absolute.line`(derive PartialEq 인라인 fight_model.rs:243) | 4 | OK |  |
| 10 | FightPrediction(abandon, 로컬 %16) | 0x30 | net_value (i64) | r | m10.ll:48189~48190 L663 → 세 번째 resolve_fight_full 의 baseline 인자 | 4 | OK |  |
| 11 | bumpalo Vec<&Entity>(remaining %18) | 0x18 | len | r | m10.ll:48088~48090 L658 `len == 0` · 48237 L666 min_by_key 범위 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 12 | sret | 0x0 | FightPrediction 전체 64B | w | m10.ll:48003 · 48211 | 4 | 확인불가(tcx 사전에 타입 없음) | resolve_fight_full(absolute) 의 sret 직접 기록(L654) → remaining 있으면 diff 64B memcpy 로 덮음(L668) |
| 13 | FightPrediction(diff %15) | 0x39 | line_absolute | w | m10.ll:48197~48198 L664 — memcpy 로 sret +0x39 에 도달 | 4 | OK | absolute.line |
| 14 | FightPrediction(diff %15) | 0x20 | rescue_ally.tag | w | m10.ll:48362·48365 L666 — `diff.line != absolute.line` 일 때만. Some ⇔ remaining 비어있지 않고 min 존재(remaining.len>0 이 전제라 실제로 항상 Some — 48278 `len==0` 분기는 구조상만 live) | 4 | OK | 1(Some) \| 0(None) |
| 15 | FightPrediction(diff %15) | 0x28 | rescue_ally.value | w | m10.ll:48363·48367 | 4 | OK | 최근접 remaining 의 id \| undef(None) |
| 16 | stack | 0x0 | allies: bumpalo Vec<&Entity>(32B %20) · arrivals: bumpalo Vec<i64>(32B %19) · remaining: bumpalo Vec<&Entity>(32B %18) · abandon(64B %16) · diff(64B %15) | w | m10.ll:47914~47934(new_in: ptr=8(dangling) bump cap=0 len=0) · 48021~48024/48063~48066 push · 48081 from_iter_in. 함수 밖 &mut 인자 없음(전 인자 readonly). bump 할당은 pool 의 부작용(chunk 소비) | 4 | 확인불가(★모호: 동명 def_path 3개 [('game_core::Hunter) | L648~663 로컬 |

**`consts` 상수 3건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 0 | 661 | 태그 | 포기(abandon) 호출의 committed_dir=0(m10.ll:48127 i8 0) · arrivals 빈 슬라이스(len 0) · baseline 0 — 「묶인 아군만으로, 커밋 없이, 도착 없이」의 절대 판정. 같은 0 이 L654 절대 호출 baseline(48003 마지막 인자)과 L658 `remaining.len()==0`(48090) 에도 | 4 |
| 1 | 1 | 666 | 태그 | rescue_ally = Some 태그(m10.ll:48362 phi) · bumpalo push additional=1(계측 아님·용량) | 4 |
| 2 | 8 | 648 | 미상 | `inttoptr (i64 8 to ptr)` = 빈 bumpalo Vec 의 dangling 포인터(align 8)(m10.ll:47914·47921) · L661 빈 arrivals 슬라이스 ptr(48127). 판정 아님 | 4 |

**`knobs` 조정점 4건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 포기 판정의 committed_dir 고정값 | fight_model.rs:661 | 0 | abandon 저울에 커밋 방향을 안 준다(=중립). 호출 인자 committed_dir 을 그대로 넘기면 「이탈 커밋 중 포기」가 절대 판정과 같은 편향을 받아 차분(baseline)이 줄어듦 | 4 | 기존 |
| 1 | 포기 판정의 arrivals | fight_model.rs:661 | 빈 슬라이스 | 묶인 아군의 도착 틱을 0으로(=이미 도착) 보는 게 아니라 도착 정보를 아예 안 준다 — 콜리가 arrivals 부재를 어떻게 다루는지는 콜리 명세 | 4 | 기존 |
| 2 | remaining 필터 | fight_model.rs:656 | bound && id != champ.id | bound=false 인 편성원은 포기 판정에서 빠진다 → 그들만 있으면 remaining 비어 차분 없이 절대 판정 반환 | 4 | 기존 |
| 3 | rescue_ally 선택 기준 | fight_model.rs:666 | min (dist², id) | 가장 가까운 묶인 아군(동률이면 id 작은 쪽). 소비자는 이 아군 방향일 때만 뒤집기를 채택(_docs) — 기준을 바꾸면 호응 면허 대상이 바뀜 | 4 | 기존 |

<details><summary>`callees` 피호출자 8건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | is_empty | game_core::PatchSetting::is_empty | pub | fn(&game_core::PatchSetting) -> bool | game-core\src\patch_engine.rs:853 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 1 | is_empty | game_core::ModAiRegistry::is_empty | pub | fn(&game_core::ModAiRegistry) -> bool | game-core\src\mod_ai.rs:15 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 2 | is_empty | game_core::PublicChampionEvidence::is_empty | pub | fn(&game_core::PublicChampionEvidence) -> bool | game-core\src\banpick\public_evidence.rs:96 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 3 | push | <game_core::Staff as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::staff | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\staff.rs:38 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 6개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 4 | push | <game_core::Athlete as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:434 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 6개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 5 | push | <game_core::Contract as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:2498 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 6개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 6 | resolve_fight_full | game_ai::plan_legacy::old::fight_model::resolve_fight_full | in:game_ai::plan_legacy::old::fight_model | fn(usize, &game_core::OperationData, &game_core::Entity, &[&game_core::Entity], &[&game_core::Entity], i8, std::option::Option<&game_core::Entity>, usize, &[i64], i64) -> game_ai::plan_legacy::old::FightPrediction | game-ai\src\plan_legacy\old\fight_model.rs:324 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 7 | resolve_fight_stake_roster | game_ai::plan_legacy::old::fight_model::resolve_fight_stake_roster | in:game_ai | fn(usize, &game_core::OperationData, &game_core::Entity, &[(&game_core::Entity, i64, bool)], &[&game_core::Entity], i8, std::option::Option<&game_core::Entity>, usize) -> game_ai::plan_legacy::old::FightPrediction | game-ai\src\plan_legacy\old\fight_model.rs:645 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
</details>

⚠**미매칭 5개**: `diff`, `drop  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `llvm.memcpy.p0.p0.i64`, `llvm.memset.p0.i64`, `reserve_internal_or_panic`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 3곳** (m02.ll:33304, m10.ll:16386, m10.ll:16527) · **형제 0개** 

**`open` 5건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | roster 튜플의 i64(+8)·bool(+16) 의 소스 필드명은 미확정 — DI 는 `a`(+0)·`bound`(+16) 만 남고 +8 은 이름 없음(m10.ll:48025). +8 은 resolve_fight_full 의 `arrivals` 인자로 가므로 「도착 틱」으로 읽었으나 단위(틱 절대/상대)는 콜리 명세 참조 | 4 |  |
| 1 | 미탐색 | resolve_fight_full 내부(net_value·line 산출, baseline 의 정확한 적용식, arrivals 빈 슬라이스 처리) — 계약만(자식 명세). 특히 sret 의 Option 필드 None 시 값 슬롯 live 여부·rescue_ally 를 콜리가 쓰는지 여부는 콜리 명세로 | 4 |  |
| 2 | 미탐색 | L666 min_by_key 의 `remaining.len()==0` 분기(m10.ll:48278→48281 None) 는 L658 게이트 때문에 실행상 도달 불가 — reach 는 상수 접기 아님으로 live 표기. rescue_ally 는 실제로 항상 Some | 4 |  |
| 3 | 미탐색 | src_line: tcx 645~647(시그니처 3줄) · DISubprogram line 645 — 645 로 기록 | 3 |  |
| 4 | 표기 불가 | committed_dir 의 값 의미(-1/0/1 등)는 이 함수에서 분기 없어 표기 불가(동작은 전달뿐) — _docs 「이탈 커밋(dir=-1)」만 참고 | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

