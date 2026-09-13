---

### `61` bush_distance_sq — 부시 id 가 일치하는 후보 셀(정적 71쌍) 중 (x,y) 에서 가장 가까운 셀 중심까지의 제곱거리 최솟값(Option)

| 항목 | 값 |
|---|---|
| id | `steal__bush_distance_sq` |
| 심볼 | `_RNvNtNtCshdEBA0ozCnw_7game_ai11plan_legacy5steal16bush_distance_sq` |
| 소스 | `game-ai\src\plan_legacy\steal.rs:20` |
| IR | `m07.ll` 54011~54195행 |
| 경로·가시성 | `game_ai::plan_legacy::steal::bush_distance_sq` · **in:game_ai::plan_legacy::steal** |
| 계층 | 기타 |
| exe | `d9aa80` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r9` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, u64, u64, &game_core::MapDef) -> std::option::Option<u64>
```

<details><summary>인자 4개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | bush | usize | 찾을 부시 id. map.bushes[by][bx] 와 == 비교(m07.ll:54127~54130) | 4 |
| 1 | 2 | x | u64 | 기준 x 좌표(월드 단위, 셀=32000) | 4 |
| 2 | 3 | y | u64 | 기준 y 좌표 | 4 |
| 3 | 4 | map | &MapDef(28112B) | bushes 격자만 읽음(+0x1c98) | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
static CANDIDATES: [(bx,by); 71] = [(0,0),(1,0),(2,0),(19,0),(20,0),(21,0),(0,1),(0,2),(7,4),(8,4),(9,4),(20,4),(20,5),(12,6),(4,7),(4,8),(29,8),(4,9),(14,9),(29,9),(29,10),(26,11),(6,12),(12,12),(13,12),(26,12),(12,13),(26,13),(9,14),(26,14),(20,15),(21,15),(17,16),(21,16),(16,17),(17,17),(25,18),(0,19),(25,19),(0,20),(4,20),(5,20),(15,20),(25,20),(0,21),(15,21),(16,21),(25,21),(25,22),(29,24),(18,25),(19,25),(20,25),(21,25),(22,25),(29,25),(11,26),(12,26),(13,26),(14,26),(29,26),(28,28),(29,28),(8,29),(9,29),(10,29),(24,29),(25,29),(26,29),(28,29),(29,29)]  // @anon.81aa2713ddfa9e15f7e3c0757f8f3b51.188 (m07.ll:210) 디코드

fn bush_distance_sq(bush, x, y, map) -> Option<u64> {
  CANDIDATES.into_iter()
    .filter(|(bx,by)| map.bushes[by][bx] == bush)        // steal.rs:22 — 인덱스 순서 [by][bx] (m07.ll:54127~54130). by,bx 각각 <30 bounds check(둘 다 정적 테이블 값이라 실제 패닉 불가)
    .map(|(bx,by)| distance_sq(x, y, bx*32000+16000, by*32000+16000))  // steal.rs:23 — utils.rs:6~9 distance_sq 인라인: abs_diff(x,cx)^2 + abs_diff(y,cy)^2 (u64, wrapping 없음: mul 은 nuw 표기 없음)
    .min()                                                 // steal.rs:24 — Ord::cmp 기반 min_by: 동률이면 먼저 나온 값 유지(aux m12.ll:18392~18395 `cmp(acc,new) < Greater ? acc : new`)
}

IR 구조: 본체(m07.ll)는 first-match 탐색(find)을 인라인해 첫 일치 셀의 거리 %50 을 초기값으로 만들고, 나머지(인덱스 %20 부터 71 까지)는 aux 의 fold 함수로 넘겨 같은 filter/map 을 반복하며 min 을 누적한다. 일치 셀이 하나도 없으면 tag 0(None).

주의: 후보 테이블 71쌍은 소스에 박힌 상수 배열이고 map.bushes 의 실제 부시 셀 전수가 아니다 — 이 목록 밖의 부시 셀은 map.bushes 값이 일치해도 절대 후보가 되지 않는다.
```

**`mem` 메모리 접근 1건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | MapDef | 0x1c98 | bushes[0][0] | r | [[usize;30];30] 격자. bushes[by][bx] 로 접근(행=by, 열=bx — %31=gep [30 x i64] by, %32=gep i64 bx, m07.ll:54127~54128). bounds check by→bx 순서(30) | 4 | OK |

**`consts` 상수 4건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 71 | 23 | 길이 | 정적 후보 셀 배열 길이(@anon.…188 = [(usize,usize);71], 1136B). 아래 logic 에 전 71쌍 전개 | 4 |
| 1 | 30 | 22 | 임계 | 격자 한 변(30x30) — bx/by 의 bounds check 상한(panic_bounds_check), 판정 임계 아님 | 4 |
| 2 | 32000 | 23 | 미상 | 셀 → 월드 좌표 변환(셀 크기). 임계 아님 | 4 |
| 3 | 16000 | 23 | 미상 | 셀 중심 오프셋(32000/2). 임계 아님 | 4 |

**`knobs` 조정점 1건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 후보 셀 테이블 71쌍 | steal.rs:23 (정적 배열, IR @anon.…188 m07.ll:210) | 71 | 쌍을 추가/제거하면 그 부시로 가는 거리 계산에 포함/제외된다. 현행은 테이블 밖 부시 셀을 못 본다 | 4 | 기존 |

<details><summary>`callees` 피호출자 5건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | bush_distance_sq | game_ai::plan_legacy::steal::bush_distance_sq | in:game_ai::plan_legacy::steal | fn(usize, u64, u64, &game_core::MapDef) -> std::option::Option<u64> | game-ai\src\plan_legacy\steal.rs:20 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | check | game_core::CastingTarget::check | pub | fn(&game_core::CastingTarget, &game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\effect\type.rs:227 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
| 2 | check | game_core::transfer::SellGuard::check | pub | fn(&game_core::Database, usize, game_core::Position) -> game_core::transfer::SellGuardResult | game-core\src\transfer\roster_blueprint.rs:564 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
| 3 | distance_sq | game_core::utils::distance_sq | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:6 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
| 4 | distance_sq | game_core::Entity::distance_sq | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2157 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
</details>

**호출처 4곳** (m07.ll:53981, m07.ll:56833, m07.ll:56879, m11.ll:36903) · **형제 0개** 

**`open` 2건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | 정적 테이블 71쌍이 소스에서 어떤 이름의 const 인지(파일 스코프 const 인지 함수 내 리터럴인지) — IR 에는 익명 상수 @anon.…188 로만 남고 이름 심볼 없음. 동작엔 영향 없음 | 4 |  |
| 1 | 표기 불가 | distance_sq 내부의 abs_diff 는 IR 이 `icmp ult + select` 로 접혀 있고 실제 소스가 abs_diff 인지 max-min 인지는 표기 불가(외연 동일). dloc 체인은 uint_macros.rs:3147 abs_diff 를 가리켜 abs_diff 가 유력 | 4 |  |

**`notes` 1건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | closure$0 의 튜플 패턴이 `(bx, by)` 인지 `&(bx, by)` 인지 등 소스 표기 — 동작(bushes[by][bx]) 은 확정 | 4 | 사실 서술 |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

