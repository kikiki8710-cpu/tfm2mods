# 3차 반증검증 — 배치 C (인덱스 10~14) 보고서

게임 0.5.8 / SDK `sdk_058` / toolchain `nightly-2026-05-24` / 2026-09-11

작업 파일 = `MIG\_verify3\C\` (`C3_a.rs`/`C3_b.rs`/`C3_c.rs`/`C3_d.rs`/`C3_e.rs` + 각 `.tsv`).
`_verify\`·`_verify2\`·`_spec\` 는 읽기만 했다. `specs20_v3.json` 직접 수정 없음.
게이트: `specgate.py` 0건 / `tcxaudit.py --prose` 이 문서 대상 0건.

---

## 0. 결론 요약

| 구분 | 건수 |
|---|---|
| 확정 오류(패치 필요) | **8** |
| open 종결(근거 승급) | **17** |
| 브리핑(BRIEF/도구) 자체 오류 | **3** |
| 값·구조·극성이 스펙과 어긋난 사례 | **0** (v30 진리표 26칸·오프셋 표본 전부 일치) |

가장 큰 성과는 **14 의 최대 미결이 반증됐다**는 것이다: 「v30≠v41 이라 `TargetMissing` 이 원리적으로
발화하지 않는다」는 서술은 **틀렸다**. 자연 상태에서 두 선택기는 12/12 동일하고, 취소는 실제로 발화한다.

두 번째 성과는 **`*_lead` 산출식이 닫혔다**는 것이다(실행 6/6 일치).

---

## 1. 확정 오류 — JSON 경로 패치 목록

### E1 `/specs[13]/siblings` — 통째로 교체

    구: {"plan": null, "count": 0, "entries": [],
         "note": "Plan 타입 메서드가 아니다(자유 함수)."}
    신: {"plan": "LineGankCoverPlan", "count": 10, "entries": [
          "clone(cover.rs:9)", "fmt(cover.rs:9)", "new(cover.rs:17)", "goal(cover.rs:21)",
          "update(cover.rs:25, mir=1)", "is_end(cover.rs:30)", "next_plan(cover.rs:34)",
          "target_bush_v30(cover.rs:134)", "target_bush(cover.rs:196)", "sub_plan(cover.rs:224)"],
         "note": "LineGankCoverPlan 의 impl 메서드다. 부시 선택기는 v30 + target_bush 둘뿐(v41 없음)."}

근거: `python spec3lib.py sib LineGankCoverPlan` (10건). `sig.tcx.path` 가
`game_ai::plan_legacy::old::LineGankCoverPlan::target_bush_v30` 이므로 자유 함수일 수 없다.

### E2 `/specs[14]/siblings` — 통째로 교체

    구: {"plan": null, "count": 0, "entries": [],
         "note": "Plan 타입 메서드가 아니다(자유 함수)."}
    신: {"plan": "LineGankerPlan", "count": 14, "entries": [
          "clone(ganker.rs:9)", "fmt(ganker.rs:9)", "new(ganker.rs:27)",
          "new_with_phase(ganker.rs:31)", "goal(ganker.rs:35)", "update(ganker.rs:39)",
          "is_end(ganker.rs:64)", "is_cancel(ganker.rs:69)", "make_gank_battle(ganker.rs:76)",
          "next_plan(ganker.rs:99)", "target_bush_v30(ganker.rs:248)",
          "target_bush_v41(ganker.rs:310)", "target_bush(ganker.rs:351)", "sub_plan(ganker.rs:379)"]}

근거: `python spec3lib.py sib LineGankerPlan` (14건).
⚠**이 두 건은 BRIEF §4 함정10 이 막으려던 바로 그 누락**이고, 하필 그 함정의 실사례(14)에서
자동 열거가 실패했다. `knobs` 산문에는 v41/target_bush 가 들어 있어 정보가 완전히 사라진 것은 아니다.

### E3 `/specs[14]/knobs[4]/effect` — 판정 반전 (**이 배치 최대 미결**)

    구: "★v30 != v41 인 구간에서는 update 의 도착 판정이 성립할 수 없어
         CancelReason::TargetMissing 취소가 원리적으로 발화하지 않는다."
    신: "★**발화한다.** 자연 상태(lead=2 · 1차타워 생존 · nearest_enemy=None)에서 v30 == v41 이다 —
         Top t0 3=3 / Top t1 6=6 / Mid t0 11·14 / Mid t1 11·14 / Bottom t0 15=15 / Bottom t1 20=20
         (12/12 일치, 실행). 불일치는 lead 가 0 이거나 타워가 파괴됐거나 타워가 적을 인지한 구간에
         한정된다(전체 72칸 중 32칸 일치). v41 은 tower 상태·nearest_enemy 를 전혀 안 보고
         lead 와 사이드만 보기 때문이다."
    ev: 4 -> 2

같은 취지의 문장이 `/specs[14]/open[?]`·`/specs[14]/knobs[5]` 산문에도 있으면 함께 정정.

실행 증거(`C3_b.tsv` `arrive` 행) — 챔프를 v30 목표 부시 셀에 놓고 `LineGankerPlan::update` 호출:

| line | team | champ_bush | 결과 |
|---|---|---|---|
| Top | 0 | 3 | `chats:[Cancel(TargetMissing)] phase:Cancel` |
| Top | 1 | 6 | `chats:[Cancel(TargetMissing)] phase:Cancel` |
| Bottom | 0 | 15 | `chats:[Cancel(TargetMissing)] phase:Cancel` |
| Bottom | 1 | 20 | `chats:[Cancel(TargetMissing)] phase:Cancel` |

(`GoalData::default()` 라 `has_near_line_enemy` 가 false 인 상태. 다른 부시 21개에서는 발화하지 않아
**도착 판정이 v30 값 한 칸에서만 성립**하는 것도 같이 확인됐다.)

### E4 `/specs[10]/sig/params[0]/role` + `/specs[10]/open[1]` — `version` 은 죽은 인자

    구(params[0].role): "AI 버전. 이 함수 자체엔 분기 없음 —
                         path_finder::is_enemy_well_danger 에 그대로 전달만 함"
    구(open[1]):        "path_finder::is_enemy_well_danger 내부 미확인 —
                         version 이 여기서만 쓰이므로 버전 분기는 전적으로 이 함수 몫이다."
    신(params[0].role): "AI 버전. ★**죽은 인자**다 — 이 함수도, 유일한 소비처
                         is_enemy_well_danger 도 version 을 읽지 않는다. 10 에는 버전 분기가 없다."
    신(open[1]) -> closed: "is_enemy_well_danger 전문 확정(아래 §2.2). version 미사용."
    ev: 4 -> 2 (오라클) + 4 (IR 전문)

근거 ①`_gaibc/m03.ll:144500~144535` 전문 독해 — `%0`(version) 이 한 번도 로드되지 않는다.
②오라클: `ver ∈ 0..5` × `team ∈ {0,1}` × 17×17 격자(=3,468칸) 전수 결과 동일(`C3_c.tsv` `well` 행).

### E5 `/specs[11]/open[4]` — "뒤 8B" 는 패딩이 아니라 별도 반환값

    구: "passive_plan 이 392B sret 인데 384B 만 plan 으로 복사한다.
         뒤 8B 가 무엇인지(패딩인지 별도 반환값인지) 이 범위에서는 확인 불가"
    신 -> closed: "tcx 정본이 답이다 — passive_plan 의 반환형은 (BigPlan, u8) 다.
         392 = BigPlan 384 + u8 1 + 정렬 패딩 7. 이 호출자는 u8(+384)을 **버린다**
         (m13.ll: 392B alloca 에 sret 받고 384B 만 memcpy).
         u8 의 의미는 미탐색(범위: passive_plan 본문 = 담당 줄범위 밖)."
    ev: 4 -> 3

근거: `callees[]` 에 이미 실린 tcx 시그니처
`LegacyPlanHandler::passive_plan(...) -> (game_ai::plan_legacy::types::BigPlan, u8)`
(handler.rs:1855). ★이미 자동 생성돼 스펙 안에 있던 값이라, `open` 이 그것을 안 본 것이 원인이다.

### E6 `/specs[10]/consts` 에 상수 1개 추가 + `/specs[10]/knobs` 에 노브 1개 추가

    consts 신규:
      value:   19600000000
      src_line: 1188            (can_enemy_hit_objective, fight_model.rs:1188)
      meaning: "적↔오브젝트 거리제곱 하드컷 = 140000^2. dx^2+dy^2 이 이 값을 넘으면
                이펙트 검사 없이 즉시 false. 25000 여유치와 무관하게 잘린다."
      kind: 임계 / ev: 2

    knobs 신규:
      what:  "적이 오브젝트를 때릴 수 있다고 볼 최대 거리(하드컷)"
      where: "fight_model.rs:1188 can_enemy_hit_objective (m10.ll:47474 icmp ugt 19600000000)"
      value: 19600000000
      effect: "이 값을 키우면 아무리 먼 적도 '이펙트 사거리+여유' 검사까지는 가고,
               줄이면 25000 을 키워도 전투가 일찍 끝난다. 실측 경계 = 거리 140000 true / 140001 false."

### E7 `/specs[14]/mem` — 한 행에 두 오프셋 (BRIEF §5 위반) + `dir` 오기

    구: {"base":"LineGankerPlan", "offset":"0x18/0x20", "name":"setup_limit / wait_limit",
         "value":"(변경 없음)", "dir":"w", ...}
    신: 두 행으로 분리하고 dir 을 비사용 표기로 바꾼다
      행1: base "LineGankerPlan" / offset "0x18" / name "setup_limit" / dir "-" /
           note "이 함수는 읽지도 쓰지도 않는다(참고용). 소비처는 next_plan/is_end 로 추정 — 미탐색"
      행2: base "LineGankerPlan" / offset "0x20" / name "wait_limit"  / dir "-" / note 동일

오프셋 자체는 tcx 로 맞음을 확인했다(아래 §3 표본). `dir:"w"` 는 `tcxaudit` 를 무력화한다.

### E8 `/specs[12]/open[3]` — 이미 답이 `shared` 에 있는데 open 이 옛 값

    구(ev 5): "format! 템플릿 @anon...62 는 2바이트 압축 constant(c\"\\C0\\00\")라 문자열 조각이 없다 —
              Debug 포맷 인자 1개짜리(\"{:?}\")로 추정. 리터럴 조각을 직접 확인하지는 못함"
    신 -> closed: "`shared.포맷템플릿_문법.결론` 이 이미 종결한 항목이다: `\\C0\\00` = 옵션 없는
              플레이스홀더 1개 + 종료 = `\"{}\"` 골격이고, Debug 여부는 인자 배열의 fmt 함수포인터가 정한다.
              ★IR 직접 근거도 있다 — 인자 배열에 `<Option<MainObjective> as Debug>::fmt` 가 2회,
              `<Chat as Debug>::fmt` 가 1회 store 된다(m13.ll rel 104/137/171).
              ⟹ 세 format! 전부 `{:?}` 로 **확정**(추정 아님)."
    ev: 5 -> 4

---

## 2. open 종결 — 새로 확정한 것

### 2.1 ★`*_lead` 산출식 (14 open[5]·knobs[5] 의 남은 절반)

**실행 6/6 일치.** 규칙:

```
lead[line][team] =
    let seq = map.lane_seq(line, team);      // MapDef::lane_seq(&self, LineType, usize) -> [usize;7], pub
    let mut lead = 0;
    for (i, r) in seq.iter().enumerate() {
        let p: i32 = cache.region_point[*r];
        let ok = if team == 0 { p > 2 } else { p < -2 };   // ★team 별로 부호가 반대
        if ok { lead = i } else { break }
    }
    lead                                     // 0..=6 (그래서 target_bush_v41 의 `lead < 7`)
```

- IR 근거: `_gcbc/g15.ll` `AbstractGameWithCache::new_with_prev_cache`(simulation.rs:1313)
  - 6개 `lane_seq` 호출 = (Top,0)/(Mid,0)/(Bottom,0)/(Top,1)/(Mid,1)/(Bottom,1) — g15.ll:102977 부근
  - team0 체인 = `icmp sgt i32 …, 2` (g15.ll:103476 등 7곳)
  - team1 체인 = `icmp slt i32 …, -2` (g15.ll:104067 등 7곳)
  - 저장 지점 6개 = g15.ll:104398~104410
- ★**2차의 「`region_point[r] < -2` 인 동안 전진」은 team1 쪽만 맞았다.** team0 은 `> 2` 다.
  6/6 불일치의 원인은 규칙이 틀린 게 아니라 **team 부호를 놓친 것**이다.
- 실측(`C3_a.tsv`/`C3_b.tsv`): 시드 1234·`start_game` 직후
  `region_point = [8,3,0,-6,6,7,0,0,6,7,-3,-3,-7,3,2,-2,0,0,-6,7,-7,-3,6,-6,3,-7,-8]`,
  `lane_seq(Top,0)=[0,5,22,17,18,20,26]` → 8>2 ✓, 7>2 ✓, 6>2 ✓, rp[17]=0 ✗ → **lead=2**.
  6개 (line,team) 조합 전부 계산값 = 실측값 = 2. 틱을 7,200 까지 굴려도 불변.
- `AbstractGameWithCache` 오프셋(한 행 = 한 오프셋, tcx 확인):

| base | offset | name |
|---|---|---|
| AbstractGameWithCache | 0x2218 | region_point ([i32;27]) |
| AbstractGameWithCache | 0x21c0 | top_lead ([usize;2]) |
| AbstractGameWithCache | 0x21d0 | mid_lead ([usize;2]) |
| AbstractGameWithCache | 0x21e0 | bottom_lead ([usize;2]) |

- **남는 미탐색(범위 명시)**: `region_point` **자체의** 산출식. IR 형태는
  `region_point[i] = (blue_regions[i] ? +5 : 0) + blue_cnt[i] - (red_regions[i] ? 5 : 0) - red_cnt[i]`
  (g15.ll:103585~103615) 이고 `blue_regions`/`red_regions`(simulation.rs:1394/1395, `[bool;27]`)는
  `blue_regions[lane_seq[i]] = true` 형태로 채워진다(g15.ll:105878 등 12곳). 이 채우기 루프의
  판정 조건은 **game_core 캐시 생성 로직이라 배치 C 담당 범위 밖**이다.
  ⚠**함정 기록**: 처음에 g15.ll 을 102716~104570 만 훑고 "stores 0건 ⟹ region_point ≡ 0" 이라는
  틀린 결론을 냈다. 함수는 **102716~108636** 이었다. `awk` 범위를 함수 끝(`다음 define`)으로
  맞추지 않으면 이렇게 뒤집힌다.

### 2.2 `is_enemy_well_danger` 전문 (10 open[1])

`_gaibc/m03.ll:144500~144535` 전량 + 오라클 일치:

```
fn is_enemy_well_danger(_version: usize, player: &PlayerState, x: u64, y: u64) -> bool {
    if player.info.team == 1 {                        // GamePlayer 의 team
        (x <= 64000  && (800000..=960000).contains(&y))     // 적(team0) 우물 = 좌하단
     || (x <= 160000 && (896000..=960000).contains(&y))
    } else {
        ((800000..=960000).contains(&x) && y <= 64000)       // 적(team1) 우물 = 우상단
     || ((896000..=960000).contains(&x) && y <= 160000)
    }
}
```

- IR 은 `add i64 x, -800000` + `icmp ult …, 160001` 형태의 접힌 범위비교다(BRIEF §4-1 부류).
- 두 사각형의 합집합(L자). `version` 미사용.
- 오라클 실측(`C3_c.tsv`): team0 은 (840000..960000, 0..60000) 격자 8칸 true,
  대각 이분탐색 경계 = 64000 true / 64001 false. team1 은 완전 대칭.

### 2.3 `can_enemy_hit_objective` 전문 (10 open[0])

`_gaibc/m10.ll:47454~47620`:

```
fn can_enemy_hit_objective(enemy: &Entity, obj: &Entity, margin: u64) -> bool {
    let dx = enemy.x.abs_diff(obj.x); let dy = enemy.y.abs_diff(obj.y);
    if dx*dx + dy*dy > 19_600_000_000 { return false }        // = 140000^2 하드컷
    for eff in [ enemy.attack_effect, enemy.skill_effect,
                 if enemy.level > 2 { enemy.skill2_effect } else { EMPTY },
                 if enemy.level > 4 { enemy.ult_effect   } else { EMPTY } ] {
        if eff.casting_target != None
           && CastingTarget::check(&eff.casting_target, enemy, obj)
           && Effect::is_in_range_ex(&eff, enemy, obj, ex, ey, ox, oy, margin) { return true }
    }
    false
}
```

⟹ **`25000` 의 단위 = 선형 거리**(제곱 아님·시간 아님). `Effect::is_in_range_ex` 의 **8번째 인자**로
그대로 넘어가 이펙트 사거리에 더해지는 여유치다. 오라클로 확증:
`Effect.range == 0` 인 기본 챔피언끼리는 경계 margin = 정확히 중심거리 d,
반경 10000 인 타워가 끼면 경계 = d − 10000 ⟹ **1차식**(`C3_a.tsv` `hit` 행 27개).

관련 오프셋(한 행 = 한 오프셋):

| base | offset | name |
|---|---|---|
| Entity | 0x490 | attack_effect (Option\<Effect\>) |
| Entity | 0x4c8 | skill_effect |
| Entity | 0x500 | skill2_effect (level>2 게이트) |
| Entity | 0x538 | ult_effect (level>4 게이트) |
| Entity | 0x5c8 | level |

### 2.4 `objective_entity_id_for_main_objective` 전수 (10 open[2])

`live_list` 를 손으로 채워(`game.mode.jungle_runner.epic.live_list = vec![777,888]`,
`serpen.live_list = vec![555,666]`) 12개 variant 전수 실행(`C3_d.tsv` `objid2`):

- `Morgard{phase,with_battle}` → `Some(777)` (= epic.live_list[0]) — phase·with_battle 무관
- `Serpen{...}` → `Some(555)` (= serpen.live_list[0]) — 동일
- 나머지 10개 태그(Defense/DefenseLine/Nexus/PressEpic/SplitEpic/Repair/Gank/Dive/PressTower/ComebackPick)
  → **전부 `None`** ⟹ `_ => None` 팔 확정(ev 2)
- `live_list` 가 비면 0/1 도 `None`

### 2.5 vtable 슬롯 실측 (10 open[5], 11 mem 0x40/0x28)

`&game as &dyn AbstractGame` 의 팻포인터에서 vtable 을 직접 읽어 함수 주소와 대조(`C3_e.tsv`):

| base | offset | name |
|---|---|---|
| dyn AbstractGame vtable | 0x28 | tick (슬롯 5) |
| dyn AbstractGame vtable | 0x40 | get_game_mode (슬롯 8) |
| dyn AbstractGame vtable | 0x108 | strategy (슬롯 33) — 신규 |
| dyn AbstractGame vtable | 0x1f0 | get_entity_by_id (슬롯 62) |

⟹ `divtable` 이 "ExpectedGame impl 기준" 이라 경고를 달았던 세 슬롯이 **`Game` impl 에서도 같은 인덱스**임을
실행으로 확인했다(vtable 레이아웃은 트레이트별로 고정이므로 구현체가 달라도 인덱스는 같고 주소만 다르다).
open[5] 은 이 범위에서 종결. 남는 미탐색 = "실전에서 `Game` 과 `ExpectedGame` 중 어느 쪽이 꽂히는가"
(callees 에 두 impl 이 다 있으므로 둘 다 도달 가능 — 배경 시뮬은 `ExpectedGame`).

### 2.6 `is_recent_visible` 120틱 (10 knobs[4])

tick=300 으로 굴린 뒤 `Blackboard::last_visible[*]` 를 직접 세팅(`C3_c.tsv` `recentvis`):
back=0/100/119/**120 → true**, back=**121**/200/600 → false. ⟹ `last_visible + 120 >= tick` 확정(ev 2).
`Blackboard` 오프셋: `Blackboard + 0x1e0 = last_visible ([usize;5])`, 크기 744B — 둘 다 tcx 일치.

### 2.7 `TutorialType::player_count` 값표 (11 open[0], 12 closed[0] 재확인)

| tut | None | First | TopSolo | Bottom | MidSolo | MidBottom | JungleOnly | Line | Total |
|---|---|---|---|---|---|---|---|---|---|
| player_count | 5 | 2 | 1 | 2 | 1 | 3 | 1 | 4 | 5 |

`player_count() == 5` ⟺ tut ∈ {None(0), Total(8)}. 여기에 `|| tut == JungleOnly(6)` 를 or 하면
IR 이 접어 놓은 집합 **{0, 6, 8}** 과 정확히 일치. ⟹ **11 open[0] 의 "어떤 값과 비교했는가" = 5.**
`== 5` 와 `>= 5` 는 최댓값이 5 라 외연이 같아 **표기 불가**(12 closed[0] 과 같은 판정).
`open[0]` 이 지적한 "다른 모듈(m04.ll)의 3그룹 분해"는 `player_count` 값이 {1,2,3,4,5} 5종이라
비교 상수가 다르면 그룹이 달라지는 게 정상이며 모순이 아니다.

### 2.8 `goal_allowed` 전수 진리표 (11 open[1] 의 기능 절반)

9 튜토리얼 × 12 goal = 108칸 전수(`C3_c.tsv` `goal_allowed`). 스펙 `logic` 과 **전칸 일치**:

| goal | true 인 tutorial |
|---|---|
| Line(Top) | None, TopSolo, Line, Total |
| Line(Mid) | None, MidSolo, MidBottom, Line, Total |
| Line(Bottom) | None, First, Bottom, MidBottom, Line, Total |
| Jungle(*) | None, JungleOnly, Total |
| Epic | None, Line, Total |
| Serpen | None, MidBottom, Line, Total |
| **Nexus / Battle / Recall** | **9종 전부 (항상 true)** |

⟹ open[1] 의 "Nexus/Battle/Recall 이 허용되는가"는 **항상 허용**으로 종결(ev 2).
"개별 arm 인지 `_ => true` 인지"는 외연이 같아 **표기 불가**로 남긴다.

### 2.9 `get_game_mode` 2회 호출 (11 open[3])

같은 `&Game` 에 대해 두 번 부르면 `as_moba()` 페이로드 포인터가 동일(`0x16dd969e90` 2회).
⟹ 상태의존 아님(ev 2, 범위: `Game` impl). CSE 가 안 된 것은 `&dyn` 간접호출이 opaque 하기 때문.

### 2.10 `GameMode` 페이로드 (11 open[2])

tcx: `GameMode` = 16B / 태그 +0x0 / 페이로드 **+0x8**.
Moba(0) = `&MobaMode`, SingleLane(1) = `&SingleLaneMode`, DeathMatch(2) = `&DeathMatchMode`.
⟹ open[2] 종결(ev 3). consts 의 DeathMatch=2 · SingleLane=1 도 재확인.

### 2.11 `chat_allowed` 전수 진리표 (12 open[1])

33 Chat × 9 tutorial = 297칸 전수(`C3_c.tsv` `chat_allowed`). 세 부류로 갈린다:

- **무조건 허용(13종)**: Start, Battle, BattleDive, BattleHelp, BattleStop, Ok, Reject, Cancel,
  Lead, Repair, DefenseNexus, EarlyPlan, ReadySignal
- **`line_exists(line)` 의존(14종)**: Mia(=`position_exists`), JungleCheck, BattleLine, GankRequest,
  CoverLine, GankLineCover, HideLine, LineCover, DefenseLine, AttackNexus, GankDive, PressTower,
  ComebackPick, GankPlan
- **개별 스코프**: CounterJungle = 정글 허용 {None, JungleOnly, Total} /
  Split·Press·PlayCall·MorgardPrepare = `morgard_exists` {None, Line, Total} /
  SerpenPrepare = `serpen_exists` {None, MidBottom, Line, Total}

### 2.12 `position_exists` 전수 (12 게이트 2 표본 재확인)

| position | true 인 tutorial |
|---|---|
| Top | None, TopSolo, Line, Total |
| Jungle | None, JungleOnly, Total |
| Mid | None, MidSolo, MidBottom, Line, Total |
| Bottom | None, First, Bottom, MidBottom, Line, Total |
| Support | Bottom 과 **완전 동일** |

⟹ 스펙 `logic` 의 `Bottom | Support` 공유 arm 확정. **정정 0.**

### 2.13 13/14 `target_bush_v30` 전수 진리표 — **오류 0**

`LineGankCoverPlan::sub_plan`(pub)이 `SubPlan::Hide{bush}` 로 v30 반환값을 그대로 노출한다.
타워는 **캐시 필드를 직접 None 으로** 만들어(`cache.top_tower[t] = None` 등) 3단계 상태를 만들고,
`nearest_enemy` 는 타워 엔티티(`EntityType::Tower { info }`)를 world 에서 직접 수정했다.
Mid 의 사이드는 챔프를 (600000,600000) 로 옮겨 `is_top_side=false` 를 만들었다(§4-B1 참조).

| line | team | 1차타워 생존·적미인지 | 1차타워·적인지 | 2차타워만 | 전멸 |
|---|---|---|---|---|---|
| Top | 0 | 3 | **6** | 3 | 2 |
| Top | 1 | 6 | **3** | 6 | 16 |
| Bottom | 0 | 15 | **20** | 15 | 9 |
| Bottom | 1 | 20 | **15** | 20 | 21 |

| line | team | 1차타워·탑사이드 | 1차타워·봇사이드 | 2차타워·탑 | 2차타워·봇 | 전멸 |
|---|---|---|---|---|---|---|
| Mid | 0 | 11 | 14 | 8 | 13 | 4 |
| Mid | 1 | 11 | 14 | 12 | 18 | 17 |

- **26칸 전부 스펙 `logic`/`consts` 와 일치.** 2차의 `is_top_side` 극성 정정도 재확인됐다
  (봇사이드에서 14/13/18 이 나온다).
- Mid 1차타워 생존 분기는 **team 무관**(양 팀 11/14) — 스펙 주석 ★와 일치.
- Top/Bottom 은 `nearest_enemy` 가 실질 유일 분기점이라는 요약도 일치.
- 반환값 집합 = {2,3,4,6,8,9,11,12,13,14,15,16,17,18,20,21} — `sig.ret` 의 열거와 **완전 동일**.

### 2.14 `MapDef.bushes` 값 사전 (14 open[1])

`MapDef + 0x1c98 = bushes ([[usize;30];30])`. id 0 은 "부시 없음"(829칸). id 1~24 존재(`C3_e.tsv`):

| id | 중심(x,y) | id | 중심(x,y) | id | 중심(x,y) |
|---|---|---|---|---|---|
| 1 | 35200, 35200 | 9 | 848000, 416000 | 17 | 160000, 656000 |
| 2 | 656000, 16000 | 10 | 208000, 400000 | 18 | 506667, 677333 |
| 3 | 272000, 144000 | 11 | 410667, 410667 | 19 | 944000, 816000 |
| 4 | 656000, 160000 | 12 | 304000, 464000 | 20 | 656000, 816000 |
| 5 | 400000, 208000 | 13 | 677333, 506667 | 21 | 416000, 848000 |
| 6 | 144000, 272000 | 14 | 549333, 549333 | 22 | 928000, 928000 |
| 7 | 944000, 304000 | 15 | 816000, 656000 | 23 | 304000, 944000 |
| 8 | 464000, 304000 | 16 | 16000, 656000 | 24 | 816000, 944000 |

2차 knobs 의 검산(부시 16 = (16000,656000) 아군쪽 · 부시 2 = (656000,16000) 적쪽)과 일치하고,
v41 표에만 나오는 7·23 도 실제 존재하는 부시다.

### 2.15 `hp_ratio < 41` 임계 (14 consts)

`stat_cached.hp = 1000` 으로 올려 `hp` 를 쓸면 ratio 40 → `Cancel(LowHpSelf)`, ratio 41 → 발화 안 함
(`C3_c.tsv` `hpgate`). ⟹ `hp*100/max_hp < 41` 확정(ev 2).
⚠주의: `AthleteStat::default()` 로 만든 챔프는 `stat_cached.hp == 1` 이라 ratio 가 항상 0 이다 —
HP 게이트를 시험하려면 max_hp 를 반드시 올려야 한다(첫 시도에서 이걸 놓쳐 6/6 이 전부 LowHp 였다).

### 2.16 11 `version < 2` 게이트 + writes 오프셋 (표본 재확인)

`v3_fall_back_to_passive` 를 6,168B 스냅샷 diff 로 실행(`C3_d.tsv` `r11`, 9 tut × 5 pos × 5 ver):

- ver 0·1 → **diff 0건**(완전 no-op) / ver 2·3·5 → 적용 ⟹ `version < 2` 확정(ev 2)
- 적용 시 바뀌는 곳: `LegacyPlanHandler + 0x1610` = 29, `+ 0x1618` = tick(0),
  `+ 0x1628` = 1(카운터), 그리고 `0x5e8`~`0x768` 대역(plan) 과 `0x768`~ 대역(sub_plan)
  ⟹ `mem` 의 writes 전 항목 일치. **정정 0.**
- `rule_scope` 거부 사례도 잡혔다(예: tut=TopSolo 는 5 포지션 전부 diff 0건).

### 2.17 12 게이트 1(자기발화) 표본 재확인

`from == player.info.position` 인 모든 행이 diff 0건, 그 외에는 inner 진입(`C3_d.tsv` `r12`).
tut=First·recv=Top·from=Bottom/Support 에서는 게이트를 통과한 뒤 **inner 에서 패닉**한다
(포지션이 없는 튜토리얼에서 BattleHelp 를 처리하려다 실패). 게이트 모델은 스펙대로다.

---

## 3. `ev<=3` 표본 재확인 (뒤집힘 0건)

| 대상 | 스펙 | tcx 실측 | 판정 |
|---|---|---|---|
| LegacyPlanHandler 크기 | 6168 | 6168 | 일치 |
| LegacyPlanHandler + 0x5e8 | plan | plan | 일치 |
| LegacyPlanHandler + 0x768 | sub_plan | sub_plan | 일치 |
| LegacyPlanHandler + 0x1628 | v3_lapse_passive_fallbacks | 동일 | 일치 |
| LegacyPlanHandler + 0x517 | team_plan.objective | TeamPlan+0x41f (=0x517) | 일치 |
| MainObjective + 0x1 | phase | phase | 일치 |
| MainObjective + 0x2 | with_battle | with_battle | 일치 |
| GamePlayer + 0x930 | info.team | team: usize | 일치 |
| GamePlayer + 0x9c0 | info.position | position: Position | 일치 |
| LineGankCoverPlan + 0x20 | line | line: LineType | 일치 |
| LineGankerPlan + 0x28 | line | line | 일치 |
| LineGankerPlan + 0x29 | phase | phase | 일치 |
| Tower + 0x18 | nearest_enemy | nearest_enemy | 일치 |
| JungleRunner + 0x180 | epic | epic | 일치 |
| JungleRunner + 0x1b0 | serpen | serpen | 일치 |
| Chat 크기/태그 | 24B / 57 variant / Cancel=17 | 24B / 0..=56 / 17 | 일치 |
| Blackboard 크기 | 744 | 744 | 일치 |
| ObjectPhase::Hunt | 3 | 3 | 일치 |

추가로 **14 open[4](Chat 나머지 22B)** 를 종결한다: tcx 레이아웃상 `Chat::Cancel` variant 는
`+0x0` 태그 + `+0x1` `CancelReason` 뿐이고 나머지 22B 는 **이 variant 의 패딩**이다
(다른 variant 는 `+0x8` 에 usize 를 둔다). Rust 는 패딩 초기화를 요구하지 않으므로 "store 안 됨" 이 정상.
⟹ 구조 확정(ev 3), "Chat 이 실제로 2바이트만 유효" 라는 표현은 부정확(24B 중 2B 만 유의미).

**14 open[0]** 도 종결: DWARF `!DILocalVariable(name: "_positioning_score", arg: 7)` 이 있으므로
소스 파라미터 이름이 실제로 `_` 접두다(최적화 잔재 아님). 덤으로 시그니처가 소스
**39/40/41 3줄**에 걸쳐 있다(arg1~5 = L39, arg6 = L40, arg7~8 = L41).

---

## 4. 브리핑·도구 자체의 오류 (BRIEF 가 요구한 산출물)

### B1 ★`GameSetting::default()` 는 `tick_per_second` 만 0 이 아니다

BRIEF §3① 은 `tick_per_second == 0` 만 적었다. 실측(`C3_a.tsv` SANITY):

    GameSetting::default().width  == 0     (실전 960000)
    GameSetting::default().height == 0     (실전 960000)
    GameSetting::default().champion_radius == 0   (실전 10000 — 챔프 Entity.radius 가 0 으로 나온다)

실전 값은 게임 설치 폴더의 **JSON** 에 그대로 있다:
`<게임설치>\bundle_unpacked_full\setting\game_setting.game_setting`
→ `width 960000` · `height 960000` · `tick_per_second 60` · `champion_radius 10000` · `visible_distance 130000`.
⟹ 오라클 프로브 템플릿에 `setting.width/height/champion_radius` 세 줄을 추가해야 한다.

### B2 ★2차의 「챔프 10명이 전부 `is_top_side=true` = 재료 부재」는 원인 오진

`is_top_side = (x + y <= height)` 인데 `height == 0` 이면 `height - y` 가 u64 언더플로해 **항상 true** 가 된다.
실전 `height = 960000` 을 넣으면 같은 초기 좌표에서 **Support 2명(양 팀)이 `is_top_side = false`** 다
(`(49000, 946000)` → x+y = 995000 > 960000). 즉 「좌표가 전부 탑사이드」가 아니라
**설정값이 0 이라 술어가 상수였던 것**이다. ⟹ 「재료 부재」가 아니라 **설정 데이터 누락(미탐색)**.
이 한 줄 때문에 2차가 `s=true` 가지를 못 밟았고, 3차는 그것만 고쳐 26칸을 다 밟았다.

### B3 `specgate.py` G3 는 `siblings.plan = null` 을 무조건 통과시킨다

`specgate.py:121~123` (v3 분기)는 `sb.get("plan")` 이 truthy 일 때만 검사한다.
13·14 처럼 `plan: null, count: 0` 이면 조용히 통과한다 — G3 가 막으려고 만든 실패(14 의 v41 누락)와
정확히 같은 형태다. 제안: v2 분기처럼 `sym` 에서 `…Plan` 타입을 뽑아 낼 수 있으면
`plan == null` 도 오류로 올릴 것.

---

## 5. `callees_unmatched` 점검 (BRIEF 우선순위 3)

| idx | unmatched | 판정 |
|---|---|---|
| 10 | phase, with_battle | `MainObjective` 의 **필드명**. 술어 아님 — 문제 없음 |
| 11 | data, positioning_score, team_plan | `LegacyPlanHandler` 의 **필드명**. 술어 아님 |
| 12 | format_inner, grow_one | std/alloc 내부. 술어 아님 |
| 13 | panic | 가드. 술어 아님 |
| 14 | grow_one, llvm.assume, llvm.umin.i64, panic | 인트린식·가드. 술어 아님 |

⟹ **판정에 쓰이는 술어가 unmatched 에 섞인 사례 0건.**
다만 `callees[]` 쪽에 **빠진** 술어가 소소하게 있다(값 오류는 아니라 패치 필수는 아니다):
12 의 `TraceLevel::is_enabled`·`Position::eq`, 13/14 의 `TowerType::is_second_tower`(tower.rs:99,
`is_tower2` 안에 인라인), 14 의 `Option::or`. 자동 수집이 `logic` 산문의 이름을 긁는 방식이라
인라인된 하위 술어는 원리적으로 덜 잡힌다.

---

## 6. 남기는 미탐색 (범위 명시)

| 대상 | 판정 | 범위 |
|---|---|---|
| `region_point` 자체의 산출식 | 미탐색 | game_core `new_with_prev_cache` 의 blue/red_regions 채우기 루프(g15.ll:105840~) = 배치 C 담당 범위 밖. lead 규칙은 닫힘 |
| `passive_plan` 이 반환하는 u8 의 의미 | 미탐색 | 함수가 `in:game_ai::plan_legacy::handler` = private 이라 오라클 불가. IR 본문 독해는 담당 줄범위 밖 |
| `handle_chat_inner` 본문 | 미탐색 | m13.ll:29692~33371, 담당 범위 밖 |
| `rule_scope::goal_allowed`/`chat_allowed` 의 **소스 표기** (개별 arm vs `_ => true`, `==5` vs `>=5`) | 표기 불가 | 외연이 동일. MIR·IR·기계어·실행 어디에도 차이가 없다 |
| `is_ignored_well_enemy` 의 나머지 조건 | 미탐색 | 10 의 인라인 사이트(1232)만 확인. 원본 함수 전문은 안 읽음 |
| `LineGankCoverPlan::target_bush`(cover.rs:196) · `LineGankerPlan::target_bush`(ganker.rs:351) 가 blackboard 를 쓰는지 (13 open[3]) | 미탐색 | `target_bush_v41` 은 인자승격으로 `(&cache, &ctx)` 만 받으므로 blackboard 접근 불가(확정). 나머지 두 선택기는 IR 미독해 |
| 함수 이름의 `v30`/`v41` 이 무엇의 버전인가 (13 open[2]) | 미탐색 | tcx·IR·오라클 전부에서 **버전 게이트 없음**을 확인했고, 호출자별 하드와이어라는 것까지가 한계. 이름의 출처는 개발사 소스 밖에 없음 |
| 실전에서 `Game` vs `ExpectedGame` 중 어느 vtable 이 꽂히는가 (10 open[5] 잔여) | 미탐색 | 슬롯 인덱스는 확정. 어느 impl 이냐는 호출자(배경 시뮬 여부)가 정한다 |

## 7. 재분류 제안 (오류는 아니지만 어휘가 어긋난 것)

- `/specs[10]/open[3]` (phase 리터럴 3 이 768 로 접힘) — 「미탐색」이 아니라 **표기 불가(상수접힘)**.
- `/specs[10]/open[4]` (적 슬롯 5칸 언롤) — 질문이 아니라 SPEC_GUIDE §3 표 규칙의 **적용 기록**.
  `open` 이 아니라 `closed` 나 `logic_note` 로 옮기는 게 맞다.
- `/specs[12]/open[6]` (PendingTraceEvent stride 184) — 위와 같은 부류.
