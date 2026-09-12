# 5차 반증검증 — 배치 C (`specs[10]`~`specs[14]`) 보고서

게임 0.5.8 / 2026-09-11 / SDK `sdk_058` / toolchain `nightly-2026-05-24`
산출물 폴더 = `C:\tfm2mods\MIG\_verify5\C\` (다른 배치 폴더는 읽지도 쓰지도 않았다)

---

## 1. 결론 한 줄

**실오류 3 · 판정반전 1 · 분류오류 2 · 새 발견 4 ·
`ev4 → ev2` 상향 = 172행(내 배치 `ev4` 239행의 72.0%) + `ev3 → ev2` 3행**
값 오류(오프셋·상수·부시표·임계)는 **0건**이다 — 4차의 「배치 C 실오류 0」은 반증 시도 후에도 유지된다.
이번 라운드에 뒤집힌 것은 전부 **문면·시그니처·판정어휘** 쪽이고, 그중 2건은 「불가/무관」 판정이었다.

오라클 실행 요약(전건 자체 대조, 프로브 4개):

| 프로브 | 무엇 | 결과 |
|---|---|---|
| `o13.rs` | `specs[13]` 부시 전표 (`LineGankCoverPlan::sub_plan` 경유) | **300/300** |
| `o13.rs` | `specs[14]` `update` 취소사유② (ganker 판 v30 전수) | **780/780** |
| `o13.rs` | `specs[14]` `target_bush_v41` lead 전표 (★Mid 봇사이드 포함) | **84/84** |
| `o13.rs` | `specs[14]` HP 취소 임계 경계 | 409(40%)=취소 / 410(41%)=유지 |
| `o11.rs` | `shared.rule_scope_게이트` 19열 × tutorial 9 | **171/171** |
| `o11.rs` | `specs[11]` version·게임모드·tutorial 게이트 + 6168B 바이트 diff | 전건 일치 |
| `o11b.rs` | `specs[12]` `handle_chat` 전수(chat2 × tut9 × trace3 × from5) | **270/270** |
| `o10.rs` | `specs[10]` `Blackboard::is_recent_visible` 120틱 경계 | **8/8** |
| `o10b.rs` | `specs[10]` `can_enemy_hit_objective` 25000·19600000000·레벨게이트 | 전건 일치 |

---

## 2. 정정 목록

### E1 (실오류) `/specs[11]/one_line` — 버전 하한이 v3 이 아니라 v2 다
```
구: "AI v3 이상에서 현재 플랜을 패시브 플랜으로 되돌리는 폴백 — 튜토리얼 스코프 허용 시에만 적용"
신: "AI v2 이상에서 현재 플랜을 패시브 플랜으로 되돌리는 폴백 — 튜토리얼 스코프 허용 시에만 적용"
```
근거 = **오라클 실행**(`_verify5/C/o11.out` §S11, `o11.rs:181~199`). `version` 0·1 은 6168B 바이트 diff 가
**완전히 비고** `v3_lapse_passive_fallbacks` 가 0, `version` **2** 부터 `fallbacks=1 · mf_swap.0=29` 로 발화한다.
`/specs[11]/consts[0]`(`value:2`, `src_line:417`, "v2 미만에는 이 폴백 자체가 없다")·`/specs[11]/logic`
(`if version < 2: return`)과도 `one_line` 만 어긋나 있었다 — 정본 우선순위상 `one_line` 이 틀린 것.
⚠**도구 사각지대**: `specgate.py:59` 의 G1 은 `one_line` 을 검사 텍스트에 **포함하지만**, 규칙이
「센티널 폭(usize::MAX↔i64::MAX)」·「Option 겹수」·「consts[].src_line ↔ meaning 의 같은 파일 줄번호」
3종뿐이라(`specgate.py:51~89`) **버전 하한 수치 모순은 원리적으로 못 잡는다.**
제안 = G1 에 「`one_line`/`logic`/`consts` 의 `v(\d+) 이상|version < (\d+)` 를 뽑아 대조」 규칙 1줄 추가.

### E2 (실오류) `/specs[12]/logic` — `position_exists` 의 인자가 `(from, tutorial)` 이 아니다
```
구: // rule_scope::position_exists(from, tutorial) 이 통째로 인라인돼
신: // rule_scope::position_exists(&GameContext, Position) 이 통째로 인라인돼
```
근거 = **rustc 실컴파일**(`_verify5/C/sig0.rs:15`):
`position_exists(/* &GameContext<'_, '_> */, /* game_core::Position */)` — 인자는 2개이고
**첫 인자가 `&GameContext`**, 두 번째가 발화자 포지션이다. `tutorial` 은 인자가 아니라 컨텍스트 안의 필드다
(2차의 `12 p7 &Chat → Chat` 과 같은 부류의 시그니처 오기).

### E3 (실오류) `/shared/rule_scope_게이트/본체` — `goal_allowed` 의 2번 인자는 **값 전달**이다
```
구: rule_scope::goal_allowed(context: &GameContext(64B), goal: &BigGoal(24B)) -> bool
신: rule_scope::goal_allowed(context: &GameContext(64B), goal: BigGoal(24B)) -> bool   // 값 전달
```
근거 = **rustc 실컴파일**(`_verify5/C/sig0.rs:13`): `two arguments of type &GameContext<'_, '_> and BigGoal are missing`.
`&BigGoal` 로 넘기면 컴파일 거부된다(실측). 같은 줄에 있는 `plan_allowed(&GameContext, &BigPlan)` 은
**참조 전달이 맞다** — 두 형제의 전달 방식이 다르다는 것이 이 정정의 요지다.

### E4 (판정반전) `/specs[14]/notes[0]` — cover 판 `target_bush_v30` 은 「담당 함수와 무관」이 아니다
```
구: fnparts target_bush_v30 이 내놓는 m10.ll 11483~11731 은 LineGankCoverPlan 쪽 동명 함수라
    담당 함수와 무관 — 그래서 위 상수들은 담당 줄범위(94569~94941) 안에서 직접 확인한 값이다
신: 같은 이름의 두 함수는 **소스 텍스트가 동일한 복제본**이다 —
    rmeta SourceMap 줄 길이가 `cover.rs:134~186` ↔ `ganker.rs:248~300` **53줄 전부 일치**
    (83/63/25/31/69/69/71/9/6/3/32/104/3/22/25/56/36/54/19/46/56/21/56/14/12/17/25/10/8/25/
     34/59/55/19/56/12/17/59/15/19/15/12/10/8/28/56/36/56/19/46/58/21/58).
    다른 것은 `self` 타입뿐이고 그 결과 `self.line` 오프셋만 갈린다
    (`LineGankCoverPlan+0x20` ↔ `LineGankerPlan+0x28`).
    ⟹ **cover 판 `define`(m10.ll:11483)은 ganker 판의 유효한 대리 관측점이다.**
    교차 실증: cover 경유 300/300 ↔ ganker `update` 경유 780/780 (같은 입력에서 같은 표).
```
근거 = `python -X utf8 rmeta_srcmap.py game_ai cover.rs 130 200` / `... ganker.rs 246 300`
(오프셋 차 = 정확히 +114줄) + `_verify5/C/o13.out`.
★**같은 문면이 방법론 문서에도 있다** — `MIG\SPEC_GUIDE.md` §1 `fnparts` 항목의
「5차 실측: `LineGankCoverPlan::target_bush_v30` ≠ 담당 함수의 `LineGankerPlan::target_bush_v30`」.
"소유 타입을 확인하라"는 경고 자체는 유효하지만 **"≠ 이니 조각을 버려라"는 이 사례에선 과잉**이었다
(문자동일 복제본이라 버리면 pub 래퍼를 통한 관측 경로를 스스로 닫는다).

### E5 (분류오류) `/specs[14]/knobs[5]` — Mid `s=true` 가지는 「재료 부재」가 아니었다
```
구: Mid 는 s=false 가지 7칸 일치 … s=true(봇 사이드) 가지는 **재료 부재** —
    start_game 직후 양 팀 챔프 10명이 전부 is_top_side=true. 미탐색 = 챔프를 봇 사이드로 옮긴 상태
신: **해소(ev2, 14/14 일치)**. 방법 = `Entity`(전 필드 pub + Clone)를 복제해 좌표만 바꾸고
    `cache.player_champion[team][pos] = Some(&내엔티티)` 로 꽂는다(`shared.오라클_레시피_함정` 에
    이미 적혀 있던 기법). 실측 전표:
      Mid s=true(봇 사이드)  team0 lead[0..6] = [21, 14, 14, 14, 14, 9, 9]
      Mid s=true(봇 사이드)  team1 lead[0..6] = [ 9, 14, 14, 14, 14, 21, 21]
    (s=false 가지는 4차 값 team0 [17,11,11,11,11,4,4] / team1 [4,11,11,11,11,17,17] 와 재확인 일치)
```
근거 = `_verify5/C/o13.rs:341~381` → `o13.out` `S14_V41_MATCH 84/84`.
★여기서 쓰인 판정어휘가 「재료 부재」였는데, 바로 뒤에 미탐색 범위를 적어 놓고도 그 단어를 썼다.
브리핑 §2③ 이 경고한 **「미탐색을 불가로 적는」** 형태 그대로다.

### E6 (분류오류) `/specs[13]/open[0]/class` — `재료 부재` → `미탐색`
```
구: class="재료 부재" | Top 의 L151(2차타워)과 L156(적 미인지)이 3/6 으로 완전히 같고,
    Bottom 의 L181/L186 도 15/20 으로 같다. 소스에서 두 분기를 따로 쓴 이유는 IR 만으로는 알 수 없다.
신: class="미탐색" (미탐색 범위 = 정확한 소스 표기 복원)
    ➕새 사실: **두 줄은 텍스트가 다르다.** rmeta SourceMap 실측 바이트 길이
      cover.rs L151 = 54 / L156 = 56   (Top 블록)
      cover.rs L181 = 56 / L186 = 58   (Bottom 블록)
    두 블록의 +2 차이는 리터럴 자리수(3·6 → 15·20)로 설명되고, 같은 블록 안의 **L151 vs L156 의 2바이트
    차이는 설명되지 않는다** ⟹ "같은 코드를 중복해 썼다"가 아니라 **같은 값을 내는 다른 표기**다.
    한편 L154(56) == L156(56) / L184(58) == L186(58) 은 길이가 같아 **좌우 스왑만 다른 동일 표기**와 정합.
```
근거 = `python -X utf8 rmeta_srcmap.py game_ai cover.rs 130 200`.
「IR 만으로는 알 수 없다」는 **범위 한정된 불가**였으므로 `재료 부재`(= 어느 재료에도 실체가 없음)가 아니다.
실제로 다른 재료(rmeta SourceMap ③)가 신호를 냈다. 정확한 표기 복원은 식별자 이름 추정이 필요해
**2회 시도 후 미탐색으로 남긴다**(SPEC_GUIDE §"2회 시도 후 unknown").

### N1 (새 발견) `/specs[13]/open[1]` — `v30` = **AI 버전 30** 표식이고, 이 플랜엔 버전 분기가 **아예 없다**
```
구: class="미탐색" | 함수 이름의 v30 이 무엇의 버전인지 — 본문에 버전 게이트 분기가 없다
신: ①`vNN` = AI 버전 NN 이 프로젝트 전반의 명명 규약이다 — `_tcx/game_ai.json` 의 Fn/AssocFn 중
     **143개**가 `vNN_` / `_vNN` 을 달고 있고 분포는 v1·v2·v3·v15·v16·v17·v19·v21~v28·v30(6개)·v32·
     v37·v41(1개=`target_bush_v41`)·v46~v48·v50·v54·v55·v57 이다. 개발자 주석(`_docs/game_ai.txt`)도
     같은 체계다 — "v38: 추격 포기", "v51: 경로상 무진전 2초+", "v62+: 궁극기를 사용한 도주",
     "v68: 이동 스킬 리스크-이득 판단", "[v3 오판 래치 S3]".
   ②`LineGankCoverPlan` 쪽에는 **버전 분기가 한 곳도 없다**: `sub_plan`(m10.ll:11755~11810)·
     `next_plan`(m10.ll:11810~)·`target_bush_v30`(m10.ll:11483) 모두 `version` 인자(%2)가
     `#dbg_value` 로만 등장하고 `icmp` 피연산자로 쓰이지 않는다(11810~12400 구간 grep 0건).
     `sub_plan` 은 `target_bush_v30` 을 **무조건** 호출한다 ⟹ v30 은 **죽은 버전표식**이다.
   ③갱커 쪽도 버전 비교가 아니라 **호출처 고정**이다: `update`→v30 / `sub_plan`·`next_plan`→v41.
   ⟹ class 를 "사실 서술"로 내리고 `closed[]` 로 이관 가능.
```
근거 = 위 tcx 집계 + `/c/tfm2mods/_gaibc/m10.ll` 11810~12400 의 `icmp .*%2` 0건 + `_docs/game_ai.txt`.

### N2 (새 발견) `specs[10]` `can_enemy_hit_objective` 의 25000 은 **선형 가산**임이 실측으로 분해됐다
경계거리 = `20000 + effect.range + margin`, 단 `140000` 에서 하드컷(`_verify5/C/o10b.out §O10b-A`):

| `effect.range` | margin 0 | 1000 | 25000 | 50000 | 100000 | 200000 |
|---|---|---|---|---|---|---|
| 0 | 20000 | 21000 | 45000 | 70000 | 120000 | 140000(컷) |
| 50000 | 70000 | 71000 | 95000 | 120000 | 140000(컷) | 140000 |
| 100000 | 120000 | 121000 | 140000(컷) | 140000 | 140000 | 140000 |

⟹ margin 이 경계에 **1:1** 로 더해진다(제곱 전 선형항).
상수항 20000 의 정체도 **실측으로 분해**했다(`§O10b-E`, `caster.radius`/`obj.radius` 를 직접 갈랐다):

| caster.radius | obj.radius | 경계 d |
|---|---|---|
| 10000 | 10000 | 20000 |
| 0 | 10000 | 10000 |
| 10000 | 0 | 10000 |
| 0 | 0 | **0** |
| 30000 | 10000 | 40000 |
| 10000 | 30000 | 40000 |

⟹ **경계 = `caster.radius` + `objective.radius` + `effect.range` + `margin`** (그 뒤 140000 하드컷).
`Entity+0x680` = `radius`(usize, tcx 확인)이고 기본값 10000 은 `GameSetting.champion_radius` 에서 온다.
⟹ `specs[10]/history[0].now` 의 "`total = effect.range + offset + caster+0x680 + (caster+0x5c8−1)*effect+0x18 + …`"
에서 **"보정들" 중 하나가 대상의 반지름**임이 확정됐다(`Entity+0x5c8` = `level`, tcx 확인).
하드컷은 `d=140000` true / `140001` false 이고,
대각선(dx=dy=k)에서 `k=98994`(dx²+dy²=19,599,624,072) true / `k=98995`(19,600,020,050) false —
**`dx²+dy² <= 19600000000` 형태**임을 확정(max 노름이 아니다).
`/specs[10]/consts[4]`(25000)·`/specs[10]/consts[5]`(19600000000)·해당 knobs 2개 → **ev2**.

⚠**함정 기록**: 첫 프로브(`o10.rs`)는 전 케이스 false 였다. 원인은 캐스터와 오브젝트를 **같은 팀**
챔피언 복제본으로 둬서 `CastingTarget::check`(m10.ll:47509)가 먼저 거부한 것. `Entity.team` 을
갈라 주고 `attack_effect.target = Enemy` · `casting = Targeting` 을 세팅해야 판별력이 생긴다.
(「입력에 판별력이 없다」로 닫기 전에 **CastingTarget 게이트**를 먼저 보라는 레시피 함정이 하나 늘었다.)

### N3 (새 발견) `specs[10]` 의 레벨 게이트가 **독립 실측**으로 확인됐다 — skill2 `level>=3` / ult `level>=5`
`_verify5/C/o10b.out §O10b-C`: skill2 만 살린 캐스터는 level 1·2 false → 3·4·5·6 true /
ult 만 살린 캐스터는 level 1~4 false → 5·6 true.
IR 근거도 같다 — `m10.ll:47561` `%46 = load (Entity+0x5c8) /*level*/` → `icmp ugt %46, 2` 이면
`Entity+0x500`(skill2_effect) 를, 아니면 **전역 빈 Effect**(`@anon.…40`)를 고른다.
`icmp ugt %46, 4` 이면 `Entity+0x538`(ult_effect) 를 고른다.
⟹ `/specs[10]/history[8].now` 의 "skill2 는 `level>2` · ult 는 `level>4`" **확정(ev2)**.
★배치 D 가 `specs[16]` 에서 MIR 로 고친 `>=3`·`>=5` 와 **독립 재료로 일치** — 교차 확증이다.

### N4 (새 발견) `LineGankCoverPlan::update`(cover.rs:25) 의 **본문이 비어 있다**
`_tcx/mirdump_game_ai.txt:12983` 부근 — `### game_ai::plan_legacy::old::LineGankCoverPlan::update [2495]
game-ai\src\plan_legacy\old\line_gank\cover.rs:25:3-27:76` 의 MIR 이 `bb0: T return` 한 줄이다.
`specs[13]` 본체(cover.rs:134)와 다른 함수지만, 이 플랜을 재구현하는 사람이 `update` 에 로직이 있다고
가정하면 헛수고한다 ⟹ `specs[13]/notes[]` 에 사실 서술로 추가할 값이 있다.

### 정정하지 **않은** 것 (반증 실패 = 명세가 맞았다)
- `specs[13]` 부시 전표 16값 · `specs[14]` 부시 전표 · `specs[14]` HP 41 · 2차타워 임계 4 ·
  `is_top_side` 극성(`!is_top_side` = 봇사이드) · `specs[11]` 게이트 4종 · `specs[12]` 게이트 3종 +
  트레이스 레벨 · `shared.rule_scope_게이트` 7행 · `shared.is_recent_visible` 120틱 —
  **전부 실행으로 재확인, 틀린 칸 0.**
- `specs[10]` `objective_entity_id_for_main_objective` 태그 0~11 은 이번 프로브에서 **전건 None** 이었다.
  게임 시작 직후엔 `epic/serpen live_list` 가 비어 있어 0·1 도 None 이 되기 때문이고(명세대로다),
  **0/1 의 "첫 원소" 경로는 이번에 재확인하지 못했다** — 3차의 ev2 를 그대로 유지한다(범위 명시).
- `specs[14]` `lead=7` 패닉은 재측정하지 않았다(4차 ev2 유지).

---

## 3. ★`ev4 → ev2` 상향 목록 (이번 라운드 주 산출물)

경로는 전부 `/specs[i]/<필드>[j]`. "근거"는 이 폴더의 프로브 출력이다.

| 명세 | consts | mem | knobs | 합(ev4→ev2) | 별도 ev3→ev2 |
|---|---|---|---|---|---|
| `specs[10]` | 2 / 6 | 0 / 19 | 3 / 7 | **5** | 0 |
| `specs[11]` | 19 / 19 | 14 / 20 | 11 / 22 | **44** | 0 |
| `specs[12]` | 11 / 11 | 16 / 23 | 6 / 19 | **33** | 3 |
| `specs[13]` | 16 / 16 | 15 / 15 | 8 / 10 | **39** | 0 |
| `specs[14]` | 21 / 21 | 25 / 25 | 5 / 6 | **51** | 0 |
| **합** | **69** | **70** | **33** | **172** | **3** |

(분모는 각 필드의 `ev4` 행 수. `specs[10]`+`[11]`+`[12]`+`[13]`+`[14]` `ev4` 합 = 239 → 172/239 = **72.0%**)

### `specs[10]` — 5행
| 경로 | 값 | 근거 |
|---|---|---|
| `consts[4]` | 25000 | `o10b.out §O10b-A` (margin 1:1 가산, 3×8 격자) |
| `consts[5]` | 19600000000 | `o10b.out §O10b-B` (140000/140001 + 대각선 98994/98995) |
| `knobs[0]` | 25000 | 동상 |
| `knobs[5]` | 19600000000 | 동상 |
| `knobs[4]` | 120 | `o10.out §O10-C` 8/8 (tick 1000↔lv 879/880, 5000↔4879/4880, 300↔179/180) |

### `specs[11]` — 44행 (consts 19 전부 + mem 14 + knobs 11)
`consts[0..18]` **전 19행**:
- `consts[0]` 2(version 하한) · `consts[1]` 2(DeathMatch) · `consts[2]` 1(SingleLane) ·
  `consts[3]` 4(BigPlan::SinglePlanLine) · `consts[4]` 1(LineType::Mid) · `consts[5]` 0(in_recall) —
  근거 `o11.out §S11 게임모드 게이트`: modetag 0/1/2 를 실제로 만들어 대조했다
  (`SingleLaneGame::new`·`DeathMatchGame::new` 가 pub). SingleLane 에서 `plan tag=4`,
  `plan+0x20 = 0`, `plan+0x21 = 1` 이 관측됐고, DeathMatch 는 **diff 전무 + fallbacks=0**.
- `consts[6..14]` TutorialType 태그 0~8 **9행** — 근거 `o11.out §R1` 171/171
  (`line_exists`·`morgard_exists`·`serpen_exists`·`position_exists`·`goal_allowed` 19열 × 9 tutorial).
- `consts[15]` -7 · `consts[16]` -6 · `consts[17]` -8 — ⚠**집합은 ev2**(Epic {0,7,8} / Jungle {0,6,8} 실측),
  **리터럴 인코딩 자체는 ev4 유지**(외연 동일 표기라 실행으로 안 갈린다). `meaning` 만 ev2 로 올릴 것.
- `consts[18]` 29(mf src) — `o11.out §S11`: `mf_swap.0 == 29` 관측.

`mem` — reads 6 / writes 8 = **14행**:
- `OperationData+0x0`(cache) · `OperationData+0x8`(context) — 게이트가 실제로 반응했다.
- `GameContext+0x38`(tutorial) — `o11.out §S11 tutorial 게이트`: 허용집합 {0,2,7,8} 에서만 발화.
- `AbstractGame vtable+0x40`(get_game_mode) — modetag 0/1/2 실측.
- `AbstractGame vtable+0x28`(tick) — `o11b.out §S11b`: `mf_swap.1` 이 tick 값(1·777·100000)과 동일.
- `BigGoal+0x0`(tag) — `o11.out` 의 `plan_goal_tag=0`(Line) 출력.
- writes `LegacyPlanHandler+0x1610`(mf_swap.0) — 바이트 diff 에 항상 등장.
- writes `LegacyPlanHandler+0x1618`(mf_swap.1) — `o11b.out §S11b`: tick 0 일 때만 diff 에 없다
  (초기값과 같아서). tick 1/777/100000 에서 `0x1618..0x1619`/`0x1618..0x161a`/`0x1618..0x161b` 로 등장.
  ★**tick=0 으로만 재면 이 write 를 놓친다** — 레시피 함정으로 등재할 값이 있다.
- writes `LegacyPlanHandler+0x1628`(v3_lapse_passive_fallbacks) — 0→1.
- writes `LegacyPlanHandler+0x5e8`(plan) — diff 가 plan 영역(0x5e8~0x767) 안에서 다발.
- writes `LegacyPlanHandler+0x768`(sub_plan) — diff `0x768..0x769` 외 sub_plan 영역(~0x7b0) 다발.
- writes `BigPlan+0x0`(tag=4) · `BigPlan+0x20`(in_recall=0) · `BigPlan+0x21`(line=Mid=1) — SingleLane 경로 실측.
- writes `BigPlan+0x8`(chats Vec::new) — **부분 확인**: diff 에 `0x5f8`(= plan+0x10, Vec 의 ptr 자리)만 잡힌다.
  `plan+0x8`/`plan+0x18` 은 초기값과 같아 diff 에 안 나온다 ⟹ 이 한 행은 **ev4 유지**를 권한다.
- reads `LegacyPlanHandler+0x0`(data) · `+0xf8`(team_plan) · `+0x990`(positioning_score) ·
  `+0x1628`(RMW 읽기) — 호출이 끝까지 갔다는 것으로만 간접 확인 ⟹ **ev3 권장**(tcxdict 정본 대조 `chk=OK`).

`knobs` — `knobs[0]` AI 버전 하한 · `knobs[1]` 데스매치 제외 · `knobs[2]` 싱글레인 고정 라인 ·
`knobs[3]` 튜토리얼 스코프 허용집합 · `knobs[4]` mf_swap src 코드 · `knobs[5]` 폴백 AI 버전 게이트 ·
`knobs[6]` DeathMatch 제외 · `knobs[7]` SingleLane 고정 플랜 · `knobs[8]` 튜토리얼 허용 화이트리스트 6표 ·
`knobs[9]` mf_swap 출처 코드 · `knobs[10]` 폴백 카운터 = **11행**.
⚠`knobs[11..21]`(SubPlan::merge 보존표 · Steal 시야 max · 로밍/에고 확률 · 정글 캠프 거리 · 라인 수용 상한 …)은
**이 함수 밖의 내용**이라 이번 프로브 범위가 아니다 — `ev4` 유지.

### `specs[12]` — 33행 (consts 11 + mem 16 + knobs 6) + ev3→ev2 3행
- `consts[0..8]` TutorialType 태그 0~8 **9행** — `o11.out §R1` 의 `posTop/posJng/posMid/posBot/posSup`
  5열 × 9 tutorial 이 주장 집합과 45/45 일치. 게이트 실동작은 `o11b.out §S12b` **270/270**.
- `consts[9]` -8 · `consts[10]` -7 — 집합 {0,8}+{6} 은 ev2, 리터럴 인코딩은 ev4 유지.
- `consts[11]` -9223372036854775793 (**ev3 → ev2**) — `o11b.out §S12d`:
  `pending_trace_events[0]` 의 선두 i64 를 날바이트로 읽어 **정확히 -9223372036854775793** 관측.
- mem reads: `PlayerState+0x9c0`(info.position, 자기발화 게이트) · `GameContext+0x38`(tutorial) ·
  `GameContext+0x39`(trace_level, Off/Summary/Detailed 3단 실측) · `OperationData+0x0` ·
  `OperationData+0x8` · `AbstractGameWithCache+0x8`(game vtable 절반) ·
  `AbstractGame vtable+0x28`(tick → 이벤트 `+0xb0`) · `LegacyPlanHandler+0x858`(cap) ·
  `LegacyPlanHandler+0x860`(ptr) · `LegacyPlanHandler+0x868`(len).
- mem writes: `LegacyPlanHandler+0x858` · `LegacyPlanHandler+0x860` · `LegacyPlanHandler+0x868`
  (첫 push 에서 셋 다 diff 에 등장: `0x858..0x859`, `0x860..0x866`, `0x868..0x869`) ·
  `TraceEventType+0x0`(판별자) · `CallHandled+0x80`(from=2=Mid 관측) ·
  `CallHandled+0x84`(misunderstood 0/1 관측) · `PendingTraceEvent+0xb0`(tick).
  ⚠`CallHandled+0x8`(chat) / `CallHandled+0x20`(plan_before) / `CallHandled+0x38`(plan_after) /
  `CallHandled+0x50`(objective_before) / `CallHandled+0x68`(objective_after) 의 String 5행과
  `LegacyPlanHandler+0x5e8`(plan 읽기) · `LegacyPlanHandler+0x517`(team_plan.objective 읽기)는
  **ev4 유지** — 이번에 문자열 내용을 읽지 않았다.
  ★**ev3 → ev2 3행** = `consts[11]`(TraceEventType 판별자) · `AbstractGame vtable+0x28`(tick) ·
  `TraceEventType+0x0`(판별자) — 셋 다 날바이트 실측으로 확정됐다.
- knobs: 자기발화 게이트 / position_exists 집합표 / 트레이스 레벨 게이트 / 정글 콜 {0,6,8} /
  에픽(모르가드) 콜 {0,7,8}(`o11.out §R2` 의 `MorgardGiveUp` 행이 정확히 `1 0 0 0 0 0 0 1 1`) /
  chat_allowed default=true(`Cancel(TargetMissing)` 행이 tutorial 전 9칸 `1`).
  ⚠`chat_allowed` 시그니처는 `(&GameContext, &Chat)` 이다(sig0 실컴파일) — `logic` 의 `chat_allowed(data.context, chat)` 는 값/참조가 모호하다.

### `specs[13]` — 39행 (consts 16 + mem 15 + knobs 8)
전부 `o13.out §S13` **300/300** 으로 덮인다. 관측 경로 = `LineGankCoverPlan::sub_plan`(pub) 이
`target_bush_v30` 의 반환을 `SubPlan::Hide{bush}`(태그 9, bush 는 `SubPlan+0x8`)로 그대로 내보내는
**얇은 래퍼**라는 사실(m10.ll:11755~11810, tail call 11791) — 사설 함수를 pub 창구로 직접 읽은 것이다.
스윕 = line 3 × team 2 × 타워상태 5(없음 / 1차·적없음 / 1차·적있음 / 2차·적없음 / 2차·적있음) ×
셀 26(Mid 만 전수, Top/Bottom 은 2셀). 부수 관측: 반환 SubPlan 의 `+0x10`(out_line)=1,
`+0x11`(check_move)=0, `+0x12`(enemy_spotted_me)=0 이 300회 전부 동일.
- consts 16행: 2·16·4·17·9·21(타워 전멸표) / 3·6(Top) / 13·18·8·12(Mid 2차) / 14·11(Mid 1차) / 15·20(Bottom)
- mem 15행: `AbstractGameWithCache+0x180` · `+0x190` · `+0x1a0` · `+0x1b0` · `+0x1c0` · `+0x1d0` ·
  `+0x1e0` / `Entity+0x68` · `Entity+0x70` · `Entity+0x88` · `Entity+0x128` · `Entity+0x660` ·
  `Entity+0x668` / `GameContext+0x8` / `GameSetting+0x12c0`
  (타워 슬롯 6개는 라인별 `Some`/`None` 을 직접 꽂아 갈랐고, `Entity+0x128`·`Entity+0x88` 은
   TowerType Top↔Top2 와 nearest_enemy None↔Some 을 갈라 반응을 확인했다)
- knobs 8행: `knobs[0]` 타워 전멸 부시 · `knobs[1]` Top 기본 · `knobs[2]` Top 인지 · `knobs[3]` Mid 2차 ·
  `knobs[4]` Mid 1차 · `knobs[5]` Bottom · `knobs[6]` 2차타워 임계 4 · `knobs[8]` is_top/bottom_side 대각선.
  ⚠`knobs[9]`(셀 좌표 클램프) · `knobs[10]`(미니맵 스케일·오프셋)은 이 함수 밖이라 **ev4 유지**.
  `knobs[7]`(부시 ID 그리드)는 이미 ev2.

### `specs[14]` — 51행 (consts 21 + mem 25 + knobs 5)
- `consts[0..20]` **21행 전부** — `o13.out §S14 update 취소사유2` **780/780**
  (line 3 × team 2 × 타워 5 × 셀 26 = 780 케이스에서 "`MapDef.bushes[셀]` == 예측 부시 ⟺ Cancel 발화"가
  전건 일치). HP 임계는 `§S14 update HP 임계`: `hp=409`(ratio 40) 취소 / `hp=410`(ratio 41) 유지 ⟹
  `41` 과 `100`(백분율 환산) 동시 확정. `32000`·`29` 는 `bushes` 인덱싱이 실제로 맞아떨어졌다는 것으로 확정.
- `mem` — `ev4` **25행 전부**. 780 케이스가 line·team·타워상태·셀을 갈라 전건 예측 일치했으므로
  아래 읽기·쓰기가 모두 경로에 있었음이 실행으로 확정된다:
  쓰기 = `LineGankerPlan+0x0`(chats push) · `LineGankerPlan+0x10`(chats.len) · `LineGankerPlan+0x29`(phase).
  `g.phase` / `g.chats` 가 pub 이라 직접 읽었고, 출력이 `Cancel` + `[Cancel(LowHpSelf)]` /
  `[Cancel(TargetMissing)]` ⟹ **Chat 태그 17 · CancelReason 0(LowHpSelf)/2(TargetMissing) 확정**.
  읽기 = `LineGankerPlan+0x28`(line) · `PlayerState+0x930`(team) · `PlayerState+0x9c0`(position) ·
  `OperationData+0x0` · `OperationData+0x8` · `AbstractGameWithCache+0x1e0`(player_champion) ·
  `AbstractGameWithCache+0x180`(top_tower, `+line*32` 로 mid/bottom) ·
  `AbstractGameWithCache+0x190`(top_tower2, 동상) · `Entity+0x628`(stat_cached.hp, 분모) ·
  `Entity+0x670`(hp, 분자) · `Entity+0x660`(x) · `Entity+0x668`(y) · `Entity+0x68`(EntityType 태그) ·
  `Entity+0x70`(Tower 페이로드) · `Entity+0x88`(nearest_enemy 태그) · `Entity+0x128`(TowerType) ·
  `GameContext+0x8`(setting) · `GameContext+0x20`(map) · `GameSetting+0x12c0`(height) ·
  `MapDef+0x1c98`(bushes).
  ⚠`LineGankerPlan+0x18`(setup_limit) · `LineGankerPlan+0x20`(wait_limit)는 `dir:"-"` 참고행이고
  780 케이스에서 값 변화 0 — **ev3 유지**.
- `knobs` 5행: `knobs[0]` HP 41 · `knobs[1]` 부시표 · `knobs[2]` 2차타워 4 ·
  `knobs[4]` `target_bush_v41` 이 `sub_plan` 의 실제 목표(반환 SubPlan 이 `Hide{bush}` 태그 9) ·
  `knobs[5]` `AbstractGameWithCache+0x21c0`(top_lead) / `AbstractGameWithCache+0x21d0`(mid_lead) /
  `AbstractGameWithCache+0x21e0`(bottom_lead) 로 지배되는 84칸 전표(**Mid 봇사이드 14칸 신규**).
  ⚠`knobs[6]`(선택기 셋)은 `target_bush`(좌표쌍, `next_plan` 경유)를 이번에 재지 않아 **ev4 유지**.

### `shared` — 상향 대상
- `shared/rule_scope_게이트/표` 7행 → **ev2** (171/171)
- `shared/rule_scope_게이트/일반경기_무영향`("tutorial=0 은 6표 전부 통과") → **ev2** (R1 의 tut=0 행이 19열 전부 1)
- `shared/is_recent_visible/판정식` 의 **120틱** → **ev2** (8/8 경계)

---

## 4. `ev<=3` 표본 재확인 (뒤집힌 것 = 0)

| 항목 | 기존 ev | 재확인 | 결과 |
|---|---|---|---|
| `specs[10]/mem` Strategy `+0xf`(object_finish) | 2 | 재측정 안 함(함수가 `in:game_ai`) | 유지 |
| `specs[10]/mem` `AbstractGame vtable+0x40` / `+0x1f0` | 3 | `o10.out §O10-B` 가 `objective_entity_id_for_main_objective` 를 실행(그 안에서 두 슬롯을 탄다) — 크래시 0 | 유지 |
| `specs[10]/sig.params[1]` version 죽은 인자 | 2 | `o10.out §O10-D`: `is_enemy_well_danger` version 0~5 × 격자 162칸 **차이 0** | 유지 |
| `specs[10]/consts` MainObjective 태그·phase 위치 | 4(chk OK) | `tcxdict --enum MainObjective`: tag@`+0x0` / phase@`+0x1` / with_battle@`+0x2`, ObjectPhase Hunt=3 | 유지 — **ev3 권장** |
| `specs[11]/mem` vtable `+0x40`·`+0x28` | 3 | 위 §3 대로 ev2 로 상향 | 유지(상향) |
| `specs[11]/mem` `BigPlan+0x0` tag=4 | 3 | SingleLane 실측 tag=4 | 유지(ev2) |
| `specs[12]/consts[11]` TraceEventType 판별자 | 3 | 날바이트 실측 일치 | 유지(ev2) |
| `specs[12]/mem` vtable `+0x28` tick | 3 | 이벤트 `+0xb0` = tick | 유지(ev2) |
| `specs[14]/mem` `LineGankerPlan+0x18`/`+0x20`(dir="-") | 3 | 이 함수는 읽지도 쓰지도 않는다 — 780 케이스 바이트 관측에서 변화 0 | 유지 |
| `specs[14]/knobs[3]` `is_top_side` 극성 | 2 | 300+780+84 케이스 전부 `!is_top_side` 가정으로 맞음 | 유지 |
| `specs[13]/knobs` `is_top_side` | 2 | 동상 | 유지 |
| `shared` Entity/AbstractGameWithCache/GameContext/Tower 오프셋 | — | `tcxdict` 재조회 전건 일치(`Tower+0x18`→`Entity+0x88`, `Tower+0xb8`→`Entity+0x128`) | 유지 |

**`ev<=3` 에서 뒤집힌 항목 0건** — 사고 없음.

---

## 5. 게이트 실측 출력

```
$ cd /c/tfm2mods/MIG && PYTHONIOENCODING=utf-8 python -X utf8 specgate.py
   G1 자기모순=0  G10 class 오분류=0  G2 호출부 전수=0  G3 형제 함수=0  G4 술어 시그니처=0
   G5 sig 정본 대조=0  G6 logic 미반영=0  G7 과열림=0  G8 표에 옛 값=0  G9 callees 오염=4
   총 4건   ← 전부 `손확인`(필드/메서드 동명)
```
내 배치 몫: `--only 13` 1건(`line`·`nearest_enemy`·`position`·`team`), `--only 14` 1건(같은 4개).
**눈으로 확인 결과 전부 정상**이다 — 네 이름은 `logic` 에서 **실제로 필드**로 쓰였다
(`LineGankerPlan.line` / `Tower.nearest_enemy` / `PlayerState.info.position` / `PlayerState.info.team`,
전부 `tcxdict` 로 필드 존재 확인). `--only 10/11/12` 는 **총 0건**.
⟹ **제출 조건(G1~G8·G10 = 0) 충족.**

```
$ PYTHONIOENCODING=utf-8 python -X utf8 _spec/audit4.py
   미반영 0 · STALE 0 · ev불일치 0   → 전건 반영됨
   (내 배치 관련 4차 항목 전건 OK: "C 14 region_point 산출식 종결(27칸)",
    "C 14 setup_limit/wait_limit 소비처 = is_end", "C 13 target_bush 만 blackboard",
    "C 10 divtable 런타임 대조 4/4", "C→17 vtable 구현체 우려 해소")
```

```
$ PYTHONIOENCODING=utf-8 python -X utf8 tcxaudit.py
   총 551건  오귀속=0  밀림=0  부분일치=0  확인불가=18  OK=533        ← 기준선(구조화 표만)
$ PYTHONIOENCODING=utf-8 python -X utf8 tcxaudit.py --prose
   총 686건  오귀속=0  밀림=0  부분일치=1  확인불가=18  OK=667        ← 기준선(+기존 산문)
```
```
$ PYTHONIOENCODING=utf-8 python -X utf8 tcxaudit.py --prose _verify5/C/C5_REPORT.md
   총 741건  오귀속=0  밀림=0  부분일치=2  확인불가=18  OK=721
   ※ ~~취소선~~ 이라 스캔 제외 4건 — resolved_2026-09-10.json 2, specs20.json 2
```
**불변 게이트 = 오귀속 0 · 밀림 0 → 충족.** 내 보고서를 물려도 **오귀속·밀림 델타 0**이다(741 − 686 = 55행 추가).
`부분일치` 가 1 → 2 로 하나 늘었는데, 그 1건은 내가 쓴 `AbstractGameWithCache+0x8` 이고
**기존 `specs20.json` 의 같은 종류 항목과 동일한 양성**이다(tcx 에는 `game : &dyn AbstractGame`(16B)가
`+0x0` 에서 시작하는 한 필드로만 있고, `+0x8` 은 그 팻포인터의 **뒤 절반**이라 "그 오프셋에서 시작하는
필드가 없다"로 잡힌다). `tcxaudit` 의 [확인불가] 목록에도 `modes__single_try_engage` ·
`old_death_battle__new` · `old_epic__v3_epicops_buff_window` · `passive_jungle__best_jungle_goal` 의
같은 `+0x8` 주장이 이미 4건 올라 있다 ⟹ **도구가 팻포인터 뒤 절반을 표현하지 못하는 알려진 한계**이고,
내 행을 지워도 감사 품질이 오르지 않는다. (오프셋은 전부 한 행에 하나씩만 적었다 — 4차 D-E5 함정 회피.)

---

## 6. 브리핑·방법론 문서의 오류

### ① `BRIEF.md` — 내 배치 몫 수치는 **전건 정확**했다
직접 센 값: 배치 C `ev2=5 / ev3=10 / ev4=239`(= mem·consts·knobs 합, `specs[10]` 32 + `[11]` 61 +
`[12]` 53 + `[13]` 41 + `[14]` 52) · `open=2` · `notes=3`. §1·§3 표와 **완전 일치**한다.
전역 수치도 확인했다: `open` 총 11(A 5 / B 3 / C 2 / D 1), `notes` 총 8(A 3 / B 1 / C 3 / D 1).
4차와 달리 이번 브리핑에는 기준선 수치 오류가 없다. ★**지적할 것 없음.**

### ② `MIG\SPEC_GUIDE.md` §1 `fnparts` 항목 — 「≠」가 과잉이다 (E4 와 같은 건)
```
구: ★소유 타입 열을 봐라. … (5차 실측: `LineGankCoverPlan::target_bush_v30`
    ≠ 담당 함수의 `LineGankerPlan::target_bush_v30`).
신: 소유 타입이 다르면 **오프셋이 다르다**(cover `+0x20` ↔ ganker `+0x28`)는 뜻이고,
    **본문이 다르다는 뜻은 아니다.** 이 사례는 rmeta SourceMap 줄 길이 53/53 일치로
    **문자 단위 복제본**임이 확인됐다(cover.rs:134~186 ↔ ganker.rs:248~300).
    ⟹ 조각을 버리는 대신 **오프셋만 환산해서 쓰라.**
```
「소유 타입을 확인하라」는 유효하고, 「≠ 이니 버려라」는 관측 경로를 스스로 닫는다.
실제로 이번에 **그 조각(cover 판 define + pub 래퍼)이 `specs[13]` 41행과 `specs[14]` 21행을 ev2 로 올렸다.**

### ③ 레시피 함정으로 추가할 값이 있는 2건 (`shared.오라클_레시피_함정` 후보)
- **`CastingTarget` 게이트를 먼저 봐라.** `can_enemy_hit_objective` 류는 캐스터·대상이 **같은 팀**이면
  사거리 계산에 도달하지도 못하고 false 다(`m10.ll:47509`). 「입력에 판별력이 없다」로 닫기 전에
  `Entity.team` 과 `Effect.target`/`Effect.casting` 을 갈랐는지 확인할 것.
- **`tick=0` 으로만 재면 "틱을 쓰는 write" 를 놓친다.** `mf_swap.1`(`LegacyPlanHandler+0x1618`)은
  tick 0 에서 초기값과 같아 바이트 diff 에 **안 나타난다**. `AbstractGame::set_tick`(pub)으로
  tick 을 0 이 아니게 두고 재라.

---

## 7. 산출물 목록 (`MIG\_verify5\C\`)

| 파일 | 무엇 |
|---|---|
| `C5_REPORT.md` | 이 보고서(정본) |
| `sig0.rs` · `sig1.rs` | 시그니처 발굴용(일부러 인자 수를 틀리게 해 rustc 가 찍게 함) |
| `o13.rs` · `o13.out` | `specs[13]` 전표 300/300 · `specs[14]` 780/780 · v41 84/84 · HP 경계 |
| `o11.rs` · `o11.out` | `shared.rule_scope` 171/171 · `specs[11]` 게이트 3종 · `specs[12]` 게이트 |
| `o11b.rs` · `o11b.out` | `mf_swap.1` 틱 기록 · `specs[12]` 270/270 · 게이트 차단 시 무변경 |
| `o10.rs` · `o10.out` | `is_recent_visible` 8/8 · `objective_entity_id` 태그 12 · version 무영향 162×6 |
| `o10b.rs` · `o10b.out` | `can_enemy_hit_objective` 25000·19600000000·레벨게이트 · 우물 사각형 |
| `_meta.json` · `_shared.json` · `_s10..14.json` | 정본에서 읽기 전용으로 떼낸 사본(대조용) |

⛔`_spec/specs20.json` · `_spec/specs20_v3.json` · `distruct.json` · `dienum.json` **미수정**(읽기만 함).
증거 = mtime 이 내 첫 파일 쓰기(13:49:23)보다 **전부 이전**이다 —
`specs20.json` 10:46:19 / `specs20_v3.json` 13:46:01 / `distruct.json` 09-10 23:07 / `dienum.json` 09-10 23:40.
(`_verify3/TEMPLATE.rs` 10:46:54 · `specgate.py` 12:48:30 도 손대지 않았다.)

### 이번에 밟은 함정 1건 (다음 세션용)
`bash heredoc` 으로 파이썬 패치 스크립트를 만들어 `.rs` 에 블록을 끼워 넣으려 했더니
**`replace` 가 조용히 0건 매칭**하고 스크립트는 성공 메시지를 찍었다(무패치 상태로 빌드·실행까지 진행).
CLAUDE.md 가 이미 금지한 형태다 — **파일 편집은 Edit 도구로**. 결국 Edit 로 바꿔 한 번에 통했다.
빌드 = `sh C:\tfm2mods\MIG\_verify3\build.sh <프로브.rs>` → `%TEMP%\tfm2_spanprobe\<이름>.exe`.
네 프로브 모두 `setting_ok=true`(width/height 960000 · tps 60 · champion_radius 10000)를 먼저 찍는다.
