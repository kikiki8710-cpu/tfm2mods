# `rule_scope` 오프라인 오라클 — SDK rlib 직접 링크 진리표

게임 0.5.8 / SDK `C:\tfm2mods\sdk_058\mod-sdk` / toolchain `nightly-2026-05-24` / 작성 2026-09-11

**목적**: `game_ai::plan_legacy::rule_scope` 의 pub 함수를 **우리 프로브 바이너리 안에서** 호출해
IR 독해로 만든 표를 검증한다. 런타임에 게임 헬퍼를 부르는 것이 아니므로 CLAUDE.md §3 "완전 재구현" 원칙에 저촉되지 않는다.

---

## 0. ★재실행 레시피 (셸 한 줄)

`spanprobe.ps1` 은 SDK rlib 이 **날 LLVM 비트코드**라 그냥은 링크가 안 된다(`LNK1136`).
**`-C lto=fat` 을 붙이면 링크된다.** 이게 이 세션에서 새로 뚫린 길이다.

```bash
# 1) 프로브 생성 (선택)
python C:/tfm2mods/MIG/_oracle/gen_oracle.py

# 2) 컴파일 (한 줄) — 산출물은 %TEMP%\tfm2_spanprobe\<파일명>.exe
powershell -ExecutionPolicy Bypass -File C:/tfm2mods/MIG/spanprobe.ps1 \
  -Src C:/tfm2mods/MIG/_oracle/o_main.rs \
  -Extra "--extern bumpalo=C:/tfm2mods/sdk_058/mod-sdk/deps/libbumpalo-dafef1f270bdb02f.rlib -C lto=fat -C codegen-units=1 -C opt-level=1"

# 3) 실행
/c/Users/jungs/AppData/Local/Temp/tfm2_spanprobe/o_main.exe > C:/tfm2mods/MIG/_oracle/o_main.tsv

# o_plan.rs 는 rand 도 필요 (rand 0.8.5 쪽 rlib 을 골라야 함 — 99c6... 는 0.9.1 이라 타입 불일치)
powershell -ExecutionPolicy Bypass -File C:/tfm2mods/MIG/spanprobe.ps1 \
  -Src C:/tfm2mods/MIG/_oracle/o_plan.rs \
  -Extra "--extern bumpalo=C:/tfm2mods/sdk_058/mod-sdk/deps/libbumpalo-dafef1f270bdb02f.rlib --extern rand=C:/tfm2mods/sdk_058/mod-sdk/deps/librand-e2a5dd20f067a3a7.rlib -C lto=fat -C codegen-units=1 -C opt-level=1"
/c/Users/jungs/AppData/Local/Temp/tfm2_spanprobe/o_plan.exe > C:/tfm2mods/MIG/_oracle/o_plan.tsv

# 4) 표 생성
python C:/tfm2mods/MIG/_oracle/mk_tables.py     # -> <함수명>.{json,md}
```

`-ExecutionPolicy Bypass` 없으면 `spanprobe.ps1` 이 실행정책에 막힌다.

## 1. ★GameContext 는 UB 없이 진짜로 만들 수 있다 (제로버퍼 캐스팅 불필요)

`GameContext`(64B)는 **전 필드 pub**이고 `MapDef::moba(&GameSetting)` 이 pub이다.

| 필드 | 타입 | 만드는 법 |
|---|---|---|
| `pool` | `&bumpalo::Bump` | `Bump::new()` |
| `setting` | `&GameSetting` | `Default::default()` |
| `macro_weights` | `&MacroWeights` | `Default::default()` |
| `map_setting` | `&MapSetting` | `Default::default()` |
| `map` | `&MapDef` | `MapDef::moba(&setting)` (Default 없음) |
| `champion_list` | `&Vec<String>` | `Vec::new()` |
| `item_list` | `&Vec<Box<dyn ItemInfo>>` | `Vec::new()` |
| `ignore_minion`/`debug` | `bool` | |
| `tutorial` | `TutorialType` | **오프셋 +0x38(=56)**, MIR `(*ctx).9` |
| `trace_level` | `TraceLevel` | `TraceLevel::Off` |

## 2. 열거형 실제 메모리 태그 (실측 `o_disc.rs`)

> ⚠**정정(2026-09-11 검증배치 A)**: 원래 제목의 "선언 순서와 다른 것 주의" 와 아래 `LineType`/`ObjectPhase` 의 **「선언 순서」 주장은 거짓**이었다. tcx variant `def_span` 실측상 `LineType` 은 `Top 991 / Mid 992 / Bottom 993` 로 **선언 순서 = 태그**다. 아래 **태그 값 자체는 맞다** — 틀린 것은 "선언 순서가 다르다"는 부가 주장뿐이다.

| 열거형 | 크기 | 태그 |
|---|---|---|
| `TutorialType` | 1 | None0 First1 TopSolo2 Bottom3 MidSolo4 MidBottom5 JungleOnly6 Line7 Total8 |
| **`LineType`** | 1 | **Top=0, Mid=1, Bottom=2** (⚠선언 순서는 Mid,Bottom,Top — 태그≠인덱스) |
| `Position` | **4** | Top0 Jungle1 Mid2 Bottom3 Support4 |
| `JungleType` | 1 | Rhino0 Mushroom1 Stump2 Bee3 **Morgard4 Serpen5** |
| `StealTarget` | 1 | Epic0 Serpen1 |
| `BigGoal` | 24 | Line0 Jungle1 Epic2 Serpen3 Nexus4 Battle5 Recall6 |
| `StealAction` | 2 | None0 Lurk1 Commit2 |
| **`ObjectPhase`** | 1 | **None=0, Setup=1, Assemble=2, Hunt=3** (⚠선언 순서 Setup,Assemble,Hunt,None) |
| `Chat` | 24 | 선언 인덱스 그대로 0..56 |
| `BigPlan` | 384 | — |

## 3. 산출 파일

- `o_main.rs` / `gen_oracle.py` — 12함수 진리표 프로브 + 생성기 (6462행)
- `o_plan.rs` — `plan_allowed` + u8 코드 0..15 확장 프로브 (10224행)
- `o_disc.rs` — 열거형 태그/크기 실측
- `mk_tables.py` → `<함수명>.{json,md}` 13개 함수 진리표
- `o_main.tsv` / `o_plan.tsv` / `o_disc.tsv` — 원시 실행 출력

## 4. IR 표(A~E) 대조 결과 — **틀린 칸 0**

| 항목 | 판정 | 근거 |
|---|---|---|
| (A) `goal_allowed` 6종 + Nexus/Battle/Recall 무조건 | **전 칸 일치** | `goal_allowed.md` 22행 |
| (B) `chat_allowed` 11행 판정표 | **전 칸 일치** | `chat_allowed.md` 1292행 + m13.ll 라벨 %89/%67/%21/%26/%29/%34/%49/%52/%104 |
| (C) 자매 술어 3종 + `player_count` | **전 칸 일치** | `line_exists.md`,`morgard_exists.md`,`serpen_exists.md`,`position_exists.md` |
| (D) `position_exists` Jungle arm | **일치** + Support arm = Bottom arm(신규) | MIR `position_exists` bb2 는 Bottom(3)/Support(4) 공용 |
| (E) 일반 경기(=`TutorialType::None`) 무영향 | **확증** — tut=None 718행 전부 통과 | `o_main.tsv` |

### 오라클로 구분 불가(= 표기 불가, IR 로만 판정됨)
- chat 20/21/22 `spawn_epic && line_exists(line)` 와 `spawn_epic` 은 **외연 동일**({0,7,8} ⊆ 모든 line_exists 집합).
  IR(m13.ll %89→%94→%97/%98/%99)에 line_exists switch 가 실재하므로 **(B) 표기가 맞다**.
- `position_exists` Jungle arm 의 `player_count()==5` vs `>=5` — 외연 동일(MIR 에서 const-fold됨).

### 4단계로 새로 완전 규정된 것
- `valid_lines` / `fallback_line` — `fallback_line.md` 27칸 전량 확정.
  적합 공식(외연 일치): `if line_exists {line} else { *valid_lines(ctx).last().unwrap_or(&Bottom) }`
  (`.first()` 는 tut=MidBottom,line=Top → Mid 가 되어 실측(Bottom)과 불일치하므로 배제)
- `steal_target_allowed` / `steal_action_allowed` / `main_objective_allowed` / `sub_objective_allowed` — 전량 확정
- `plan_allowed` = **`goal_allowed(ctx, BigPlan::goal(plan))`** (m13.ll:54066, goal_allowed 가 인라인됨)
- 비-pub 3함수(간접 규정):
  - `line_from_code(c)`: 0→Top, 1→Mid, 2→Bottom, **c≥3→None** (c=0..15 실측)
  - `push_line_code_allowed(ctx,c)` = `line_from_code(c).is_none_or(|l| line_exists(ctx,l))`
  - `play_code_allowed(ctx,c)`: 0→morgard_exists, 1→serpen_exists, 2→Top, 3→Mid, 4→Bottom, **c≥5→true** (c=0..15 실측)

### 남은 미확정
- `plan_allowed` 의 `BigPlan::{Battle, SinglePlanBattle, DeathMatchBattle}` 3변형 = **이 방식(SDK 링크 프로브)으로는 재료 부재**
  (`*::new` 가 `&OperationData`,`&PlayerState` 를 요구하는데 둘 다 pub 생성자·Default 없음).
  단 게이트 자체는 `goal_allowed(plan.goal())` 로 완전 규정 — 남은 것은 `BattlePlan::goal()`/`SinglePlanBattle::goal()` 의 BigGoal 산출(별도 함수, rule_scope 밖).
- `valid_lines` 반환 슬라이스의 **순서**는 Debug 출력으로만 확인(Top,Mid,Bottom / Mid,Bottom 등) — 포인터 동일성(promoted 공유 여부)은 미탐색.
