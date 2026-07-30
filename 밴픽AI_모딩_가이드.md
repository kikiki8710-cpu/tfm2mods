# TFM2 밴픽 AI 모딩 가이드 — 밴할 때 픽도 같이 보는 법

> 기준: TFM2 **0.5.2** (buildid 24310934) / SDK `sdk_052`
> 작성 계기: 밴픽 순서 커스텀 모드(`tfm2_banpick_order`) 만들면서 알아낸 것 정리

---

## 요약 (바쁘면 이것만)

- 밴픽 AI에 개입하는 **공식 확장점은 `ModDraftScoreHook`** 하나다.
- **함정: `score_ban`이 받는 컨텍스트에는 픽 목록이 비어 있다.**
  `ctx.ally_pick` / `ctx.enemy_pick` 필드는 **존재하는데 밴 호출에선 항상 빈 슬라이스**다.
- 이유: 게임 네이티브 밴 AI 자체가 **픽을 전혀 안 본다**(설계). 바닐라는 밴이 항상 먼저라
  밴 시점엔 픽이 없으니 문제가 없었다.
- 그래서 밴에서 픽을 보려면 **직접 상태를 확보**해야 한다. 아래 3가지 방법.

---

## 1. 확장점 기본

### 등록

```rust
use mod_api::*;

#[derive(Debug)]              // ★Debug 필수
struct MyDraftAi;

impl ModDraftScoreHook for MyDraftAi {
    fn id(&self) -> &str { "my_mod.draft_ai" }   // ★필수

    fn score_ban(
        &self,
        ctx: &DraftScoreContext,
        candidate: usize,      // 후보 챔피언 인덱스
        base_score: f32,       // 게임이 계산한 점수
    ) -> DraftScoreDecision {
        DraftScoreDecision::Pass
    }

    fn score_pick(&self, ctx: &DraftScoreContext, candidate: usize, base_score: f32)
        -> DraftScoreDecision { DraftScoreDecision::Pass }
}

fn init(_ctx: &GameCtx) -> ModRegistration {
    let mut reg = ModRegistration::new("my_mod");
    reg.add_draft_score_hook(MyDraftAi);
    reg
}
declare_mod!(init);
```

### 반환값

| 변형 | 뜻 |
|---|---|
| `Pass` | 게임 점수 그대로 (개입 안 함) |
| `Add(f32)` | 게임 점수에 **더함** ← 대부분 이걸 쓰면 된다 |
| `Replace(f32)` | 게임 점수를 **덮어씀** |

**호출 시점**: 게임 AI가 후보 점수를 다 계산한 **뒤**. 후보 하나당 한 번씩 불린다.

### 컨텍스트 필드

```
DraftScoreContext {
    phase,                  // Ban / Pick
    available_champions,    // 아직 안 뽑힌 챔피언들
    ally_ban, enemy_ban,    // 밴 목록
    ally_pick, enemy_pick,  // ★밴 호출에선 비어 있음 (아래 참고)
    is_explore,
    difficulty,
}
```

`ally`/`enemy`는 **지금 행동하는 팀 기준**이다.

---

## 2. ★함정: 밴 호출에서 픽이 안 보인다

`score_pick`에서는 `ally_pick`/`enemy_pick`이 정상적으로 채워져 온다.
그런데 **`score_ban`에서는 항상 빈 슬라이스**다.

디스어셈으로 확인한 사실:

- 네이티브 밴 추천 함수는 **밴 목록 2개만** 인자로 받는다 (픽은 전달조차 안 됨)
- 밴 가치 평가에서 "우리 조합 / 상대 조합" 배열은 **빈 값으로 하드코딩**돼 있다
- 모드 훅에 넘기는 컨텍스트의 픽 슬롯도 **빈 슬라이스 상수**

즉 **게임 밴 AI는 "이 챔프가 그 자체로 얼마나 센가 + 우리 플랜에 맞나"만 본다.**
상대 조합을 보고 카운터 밴하는 기능은 원래 없다.

바닐라 순서(밴 전부 → 픽 전부)에선 밴 시점에 픽이 없으니 자연스러운 설계다.
**밴과 픽을 섞는 순서를 만들면 이게 한계로 드러난다.**

---

## 3. 밴에서 픽을 보는 3가지 방법

### 방법 A — `score_pick`에서 캐시해두기 (SDK만, 가장 쉬움)

픽 차례에는 컨텍스트에 픽이 정상으로 오니, 그걸 저장해뒀다가 밴 차례에 쓴다.

```rust
static LAST: Mutex<Option<(Vec<usize>, Vec<usize>)>> = Mutex::new(None);

fn score_pick(&self, ctx: &DraftScoreContext, ..) -> DraftScoreDecision {
    *LAST.lock().unwrap() = Some((ctx.ally_pick.to_vec(), ctx.enemy_pick.to_vec()));
    DraftScoreDecision::Pass
}

fn score_ban(&self, ctx: &DraftScoreContext, ..) -> DraftScoreDecision {
    let picks = LAST.lock().unwrap();   // 여기서 사용
    ...
}
```

- 장점: 네이티브 후킹 없이 SDK만으로 됨
- 단점:
  - **한 박자 늦다** — `score_pick`은 픽이 확정되기 *전*에 불리므로, 직전에 확정된 픽 1개가 빠진다
  - 여러 경기가 동시에 돌면(백그라운드 리그 시뮬) **다른 경기 것이 섞인다**
  - `ally`/`enemy`가 그때의 행동 팀 기준이라 **밴 차례의 팀과 다를 수 있다** (뒤집어 써야 할 수 있음)

### 방법 B — `available_champions` 차집합 (SDK만, 보조용)

드래프트 시작 시점의 챔피언 풀을 저장해두면:

```
사라진 챔프 = 시작풀 − ctx.available_champions
픽된 챔프   = 사라진 챔프 − ctx.ally_ban − ctx.enemy_ban
```

- 장점: **누락 없이 정확**한 "픽된 챔프 집합"을 얻는다
- 단점: **어느 팀이 픽했는지는 알 수 없다** → 방법 A와 섞어 써야 팀 구분이 된다

### 방법 C — 네이티브 커밋 함수 후킹 (정확하지만 고급)

게임이 밴픽을 실제로 기록하는 지점을 후킹해서 양 팀 밴·픽을 그대로 스냅샷한다.
`tfm2_banpick_order` 모드가 쓰는 방식이다.

- 밴픽 기록은 `RunningMatchInfo`의 **레코드**(stride `0x100`)에 들어간다
- 그 레코드에 팀별 4벡터가 있다 (`Vec<String>` = 챔피언 id 문자열)
  - `+0x38/+0x40` 팀1 밴 (ptr/len)
  - `+0x50/+0x58` 팀2 밴
  - `+0x68/+0x70` 팀1 픽
  - `+0x80/+0x88` 팀2 픽
  - 원소는 `String { cap, ptr, len }` = 0x18 바이트
- 커밋 함수(0.5.2에서 `0x1d075d0`)를 트램폴린 후킹해 **커밋 직후 이 4벡터를 읽어두면**
  다음 판단 때 정확한 상태를 쓸 수 있다

- 장점: **정확하고 팀 구분도 된다**
- 단점: RVA 하드코딩 → **게임 패치마다 재탐색 필요**, 후킹 안전수칙 지켜야 함

---

## 4. 챔피언 능력치 읽기

판단 재료(공격력/주문력/체력/방어/마저 등)는 클라이언트에서 얻는다.

```rust
fn post_update(&self, scene: &mut Scene, ..) {
    if let Scene::InGame { data } = scene {
        let db = data.db();
        for id in &db.available_champions {          // 챔피언 id 문자열 목록
            if let Some(c) = db.champion_info(id) {
                let s = c.stat();                    // attack, magic_power, hp,
                                                     // defence, magic_resistance, move_speed
                let g = c.growth();                  // 레벨당 성장치
            }
        }
        // 모드 챔프는 여기에도 있다
        for e in &db.champion_info_sheet.mod_champions { /* e.id, e.stat() */ }
    }
}
```

- 경기 중엔 안 변하니 **한 번만 캡처해서 캐시**하면 된다
- ⚠ `ChampionInfoSheet`는 챔프별 필드(`fighter`, `knight`, …)로 돼 있고 배열이 아니다.
  **`db.champion_info(id)` 로 조회**하는 게 정석.
- ⚠ 이 시트를 참조하면 **dll이 2.5MB 이상으로 커진다**(정적 링크).
  `build_inj.ps1` 같은 빌드 스크립트의 크기 가드에 걸리면 `rustc` 직접 빌드로 우회.

---

## 5. 다른 모드와 같이 쓸 때

같은 확장점을 쓰는 모드가 이미 여럿 있다.

| 모드 | 방식 |
|---|---|
| Win-Rate Ban/Pick AI (워크샵, yudra) | 게임 내부 승률 통계 + 패치노트 반영해 보정 |
| TFM2 AI Banpick Policy (팀파매gg) | 대시보드 메타 점수 TSV로 보정 |
| tfm2_banpick_order | 밴 시점에 픽을 보고 보정 |

- **여러 모드가 `Add`를 쓰면 보정이 그대로 누적된다.**
- `priority()`로 순서를 조절할 수 있다.
- 설정으로 **켜고 끌 수 있게 만들어 두는 걸 권장**한다(유저가 조합해서 쓸 수 있게).

---

## 6. 참고: 게임 밴픽 AI가 실제로 하는 일

직접 만들 때 참고하라고 정리.

**흐름**: 턴 발생 → 팀 전용 AI 로드(팀마다 학습 가중치가 다름 = 팀 성향의 실체)
→ 밴/픽 판정 → 후보 추리기 → 점수화 → 상위 K개 중 랜덤 선택

**후보 필터**: 이번 판의 밴·픽 전부 제외 + fearless면 이전 경기 사용 챔프 제외.
선수 보유·티어 제한 같은 건 없다(누구나 아무 챔프).

**밴 vs 픽**

| | 밴 | 픽 |
|---|---|---|
| 보는 것 | 챔프 단독 강도 + 플랜 적합도 | **이 픽으로 끝까지 갔을 때의 최종 5:5 조합** |
| 앞 내다보기 | **없음** | **있음** (남은 픽을 양 팀 다 시뮬레이션) |
| 점수식 | (상대가 가지면 잃는 값) − (내가 갖고 싶은 정도 × 계수) | 완성 조합의 승률 평가 |

**즉 픽은 앞을 내다보는데 밴은 근시안적이다.**

**랜덤성**: 점수 정렬 후 상위 K개 중 하나를 뽑는다. **K는 난이도가 결정**(최상 난이도면
1위 고정, 낮으면 최대 4위까지). 시드가 진행 상황으로 정해져서 **같은 상황이면 항상 같은
선택** → 다시보기가 재현된다.

---

## 7. 알아두면 좋은 함정 모음

- **밴 호출에서 픽은 빈 슬라이스** (이 문서의 핵심)
- **백그라운드 리그 시뮬에서도 훅이 불린다.** 플레이어 경기에만 적용하고 싶으면 게이트를
  걸어야 하는데, **포인터 동일성으로 경기를 식별하면 안 된다** — AI 턴 경로가 구조체를
  복사본으로 넘겨서 매번 주소가 다르다. 팀 ID나 내용으로 대조할 것.
- **크래시 디버깅**: 이 게임은 Rust `panic = abort` 빌드라 패닉이 예외 핸들러를 우회한다
  (VEH·콜스택으로 못 잡음). 대신 **모든 패닉이 `rust_panic_with_hook` 한 곳을 지나며,
  세 번째 인자가 `&Location{ file, line, col }`** 이다. 여기에 훅을 걸면 크래시 위치를
  소스 파일·줄 번호로 바로 알 수 있다. (0.5.2 기준 `0x25d4764`)

---

*이 문서의 RVA는 0.5.2 기준이라 게임 패치 때 달라진다. SDK API 부분은 버전 무관.*
