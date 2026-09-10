# 검증 리포트 — 배치 D (15~19)

> 2026-09-11 / 게임 0.5.8 / SDK `sdk_058` / toolchain `nightly-2026-05-24`
> 대상 = `_verify\15..19.json` · 정본 반영처 = `_spec\specs20.json`
> §11 원문 = `REPORT\tfm2_ai_adjust\RE\2026-09-11_20함수-반증검증-4갈래-정정13건.md`

게임 0.5.8 / SDK `sdk_058` / toolchain `nightly-2026-05-24` / 2026-09-11

## 0. 판정 요약

| # | 함수 | 판정 |
|---|---|---|
| 15 | `single_try_engage` | **⚠정정 1**(`team_plan` 을 `&mut` 로 전달 = 거짓) · **➕보강 3** |
| 16 | `max_range_nearly_can_use` | **✅확인**(40/50/60 표·switch 표·오프셋 전부 일치) · **➕보강 1** · 문서위생 1 |
| 17 | `DeathMatchBattle::new` | **⚠정정 2**(`main_objective` 타입 / `flee_die` 상수명) · **➕보강 1**(unknown 1개 닫힘) |
| 18 | `v3_epicops_buff_window` | **⚠정정 2**(피호출자 시그니처=argpromotion 아티팩트 / `Chat::Press` 줄번호) · **➕보강 1** |
| 19 | `best_jungle_goal` | **✅확인 — 오류 없음.** 오라클 실행 **10/10 MATCH** |
| 공통 | 방법론 | **🔁범위정정 1** — 「오라클은 `&OperationData`·`&PlayerState` 구성 불가」가 **실행으로 반증됨** |

기계 검증(`D_audit.py` = `tcxaudit` 재사용): 담당 5건의 `reads`/`writes` + 산문 오프셋 **153건 → 오귀속 0 · 밀림 0 · 부분일치 1 · 확인불가 3**.
- 부분일치 1 = `15(prose)` 의 `cache+0x8` — `AbstractGameWithCache.game` 이 **16B 팻포인터**라 +0x8 에 시작 leaf 가 없을 뿐. **오탐**(주장은 맞다).
- 확인불가 3 = 전부 `AbstractGame vtable +0x28/+0x40/+0x1f0` = 구조체가 아니라 `divtable` 소관. **오탐**.

## 1. ★공통 🔁범위정정 — 오라클로 `&OperationData`·`&PlayerState`·`&Entity` 를 **정상 생성할 수 있다**

**무효가 된 판정**: `METHOD_MAP.md §1 ⑥ 한계 2` 와 `IR_TOOLKIT.md §7` 의 "인자에 `&OperationData`·`&PlayerState` 가 들어가면 pub 생성자가 없어 **구성 불가**".

**반증(실행 증거)** — `_verify\D_oracle1.rs` → `D_oracle2.rs` 가 컴파일·링크·실행 전부 성공(크래시·UB 0):

```
Game::new(seed, bool, &GameSetting, &MapSetting, &MapDef)                      pub
game.add_player(GamePlayer::new(id,"p",team,pos,AthleteStat::default(),
        "swordman", Arc::new(SwordmanChampionInfo::default()), vec![]))        pub   ← Arc<dyn ChampionInfo> 도 pub Default 구현체 존재
game.start_game(&mut StdRng, &ctx)                                             pub
AbstractGameWithCache::new(&game as &dyn AbstractGame, &ctx)                   pub
OperationData::new(&cache, &ctx, &[Blackboard::default(); 2])                  pub
game.get_player_by_position(team,pos) -> Option<&PlayerState>                  pub  (start_game 후 10/10 Some)
cache.player_champion[t][p] -> Option<&Entity>                                 pub  (Some)
TeamPlan::default() / DebugFrameData::default()                                pub
```

실측(`D_oracle2.exe`): `player t{0,1} pos{0..4} = true` 10/10, `champ0 = true`.
⟹ **`pub(crate)` 가 아닌 game_ai 판단함수는 사실상 전부 오라클 대상**이다. 이번에 `best_jungle_goal`·`is_cleared`·`is_side_cleared`·`engage_requires_dive`·`max_range_nearly_can_use` 를 실제로 호출했다.

**적용 범위·남은 한계(정직하게)**
- ✖ **디폴트 데이터는 실전값이 아니다.** `GameSetting::default()` 의 **`tick_per_second = 0`**(실측 출력). 실전은 60. `udiv by tps` 가 있는 함수는 디폴트로 돌리면 무의미/패닉 → **손으로 60 세팅 필요**.
- ✖ `SwordmanChampionInfo::default()` 는 **이펙트가 비어** 있어 `max_range_nearly_can_use` 가 tick 0~100000 전 구간 **0**. 함수가 틀린 게 아니라 **입력에 판별력이 없다**. 실전 챔피언 데이터(`ChampionInfoSheet`/`Assets`) 로딩은 **미탐색**.
- ✖ `pub(crate)` 모듈(`fight_check`/`position_eval`/`path_finder`/`score_parameter`/`buff_value`/`small_action`)은 종전대로 막힘.
- ⚠ `engage_requires_dive` 는 `game_ai::tower_discipline::` 로 못 부른다(모듈 private) — **`game_ai::engage_requires_dive`** 로 재수출.

재현: `spanprobe.ps1 -Src _verify/D_oracle4.rs -Extra "--extern bumpalo=…libbumpalo-dafef1f270bdb02f.rlib --extern rand=…librand-e2a5dd20f067a3a7.rlib -C lto=fat -C codegen-units=1 -C opt-level=1"` → `%TEMP%\tfm2_spanprobe\D_oracle4.exe`

## 2. ★공통 함정 — internal fastcc 함수의 IR 인자 목록 ≠ 소스 인자 목록(ArgumentPromotion)

이번 배치에서 **같은 원인으로 2건**이 잘못 적혀 있었다(둘 다 18). LLVM 이 internal 함수의 `&Struct` 인자를 *실제로 로드하는 필드값*으로 치환한다.
**판별법**: ① `_tcx\*.json` 의 `sig`(정본) ② DWARF `!DILocalVariable(name:…, arg:N)` — argpromotion 후에도 **소스 이름·타입이 남는다**.

실증 `v3_epicops_repair_need`:

| 근거 | 인자 |
|---|---|
| IR define `_gaibc/m09.ll:64975` | `(i64 %0, ptr %1, ptr dereferenceable(384) %2)` |
| 호출부 `m09.ll:6892~6895` | `%10 = load [player+2352]`(=`info.team`) · `%11 = load [data+0]`(=`cache`) |
| **tcx `sig`** | `fn(&PlayerState, &OperationData, &BigPlan) -> u8` |
| **DWARF `!54685~54687`** | `player`(arg1) / `data`(arg2) / `plan`(arg3) |

**부수 소득(오히려 강한 사실)**: argpromotion 은 *모든* 용도가 그 로드일 때만 일어난다 ⟹ `v3_epicops_repair_need` 는 **player 에서 `info.team` 만, data 에서 `cache` 만** 읽는다(그 외 접근 0). 재구현 근거로 쓸 수 있다.

## 3. 15 `single_try_engage` — ⚠정정 1 · ➕보강 3

### ⚠정정 ① `team_plan` 을 `&mut` 로 넘긴다 — **거짓** (명세 3곳)
`signature.params[1].note`("team_plan 은 &mut 로 하위에 전달") / `logic` 245·255행(`&mut self.team_plan`) / `reads[LegacyPlanHandler+0xf8].note`("피호출자가 변경할 수 있음").

tcx 정본:
```
single_try_engage           : fn(&LegacyPlanHandler, usize, &mut StdRng, &PlayerState, &OperationData, usize, &mut DebugFrameData) -> Option<SinglePlanBattle>
single_tower_dive_is_viable : fn(usize, &mut StdRng, &PlayerState, &OperationData, &TeamPlan, &Entity, &mut DebugFrameData) -> bool
SinglePlanBattle::update    : fn(&mut SinglePlanBattle, usize, &mut StdRng, &PlayerState, &OperationData, &PositioningScoreData, &TeamPlan, &mut DebugFrameData)
```
`self` 부터가 **공유 `&LegacyPlanHandler`** 라 `&mut self.team_plan` 은 성립 불가이고 두 피호출자 모두 `&TeamPlan`(공유)을 받는다. ⟹ **이 계열의 `team_plan` 부작용은 존재하지 않는다.** `&mut` 인 것은 `rnd`·`debug`·`battle` 뿐.
※ 부수: `SinglePlanBattle::new/new_dive` 의 goal 은 `&BattlePlanGoal` 이 아니라 **by-value `BattlePlanGoal`**(ABI indirect 라 IR 에선 ptr).

### ➕보강 ② `iter_towers_without_nexus` 6칸 확정 (unknown 4번 닫힘)
본체 `_gcbc/g15.ll:108971~109045`:
```
iter_towers_without_nexus(team) =
  [ top_tower[team](+0x180), mid_tower[team](+0x1a0), bottom_tower[team](+0x1c0),
    top_tower2[team](+0x190), mid_tower2[team](+0x1b0), bottom_tower2[team](+0x1d0) ]  // 이 순서 그대로
      .into_iter().flatten()
      .chain( twin_towers[team](+0x130, bumpalo Vec<&Entity>).iter().copied() )
```
근거 = 스토어 순서(IR 384/416/448/400/432/464 → sret+24…+64) + `tcxdict AbstractGameWithCache`.
**꼬리는 "나머지 타워 슬라이스"가 아니라 정확히 `twin_towers[team]`**(쌍둥이 타워) — 명세 249행 주석 부정확. `nexus`(+0x170)는 별도 필드라 실제로 제외됨 ✓

### ➕보강 ③ `engage_requires_dive` 판정식 확정 (unknown 5번 절반 닫힘)
`_gaibc/m07.ll:49617` + 술어 `m06.ll:35011~35130`:
```
engage_requires_dive(player, data, e) =
  data.cache.iter_towers_without_nexus(1 - player.info.team)          // ★적팀 타워
     .any(|t| t.attack_effect(+0x490).casting(+0x4c0) != -1           // Some(Effect)?
              && Effect::is_in_range(&t.attack_effect, t, e))         // game_core
```
= **"대상이 적 타워 아무거나의 평타 사거리 안에 있는가"**. 거리 임계는 이 층에 없고 전부 `Effect::is_in_range` 안(**미탐색**). 오라클 실측 20/20 `false`(기본 스폰이 타워 사거리 밖) — 모순 없으나 판별력 없음.

### ➕보강 ④ 소스 줄 배정 검증
`D_lines.py` 로 `inlinedAt` 루트까지 펼친 결과 `modes.rs` **242·243·244·245·248·249·250·251·252·253·255·256·257·258** 전부 실재 ⟹ 명세 줄 배정 일치. vtable `+0x1f0` 간접호출 정확히 **2회**(242/244) ✓

### 확인(오류 0)
`SinglePlanBattle` 144B / `sub_goal@0x58` / `dive_tower@0x8c` ✓ · `BattleSubPlanGoal` `3 KitingBack·4 RunAway·7 End`(니치 밀림 없음) ✓ · `BattlePlanGoal 0 TryKill(2필드)` ✓ · `EntityType 2 Tower/13 Champion` ✓

## 4. 16 `max_range_nearly_can_use` — ✅확인 · ➕보강 1 · 문서위생 1

### ✅ 「'거의' 여유 인자값 표(40/50/60)」 — **전수 확인, 오류 0**
`invoke`/`call` 을 전부 훑어 **호출부 13곳 전수**가 명세 knobs 좌표와 완전 일치:
```
m02.ll:13159=40 13199=60 15125=50 41529=40 41608=60
m14.ll:14280=40 14359=60 25719=40 25790=60 60624=%33(계산값)
m15.ll:14745=40 14785=60 16295=50        (분포 40×5 · 60×5 · 50×2)
```

### ➕보강 — 유일한 계산 인자 `%33` = **`tick_per_second`** (unknown 4번 닫힘)
`m14.ll:60620~60624`: `%31=load[env+24]` → `+4856` → `%33=load`. **`4856=0x12f8` = `GameSetting.tick_per_second`**(`tcxdict GameSetting 0x12f8`).
⟹ 이 호출부의 여유는 **1초**. 실전 tps=60 이면 40/50/60 = **0.67/0.83/1.00초**이고, **이 한 곳만 tps 스케일을 따라간다**. 호출자 = `LineDefenseSubPlan::unsafe_v19_non_champion_walkup` 클로저, 결과에 `+20000` 가산(m14.ll:60625).

### ✅ 나머지(오류 0)
평타 14-way switch 실측 = `case 0,3 → 게이트 스킵` / `1→0xb8` / `2→0x110` / `4,7→0xe8` / `5,6→0x1f0` / `8,13→0xb0` / `9→0xc8` / `10→0xf0` / `11→0xd8` / `12→0xd0`, default `unreachable` — `tcxdict Entity <off>` 로 **variant 별 `attack_cooldown` 이름까지 10/10 일치**. `ty.Champion.{attack,skill,skill2,ult}_cooldown = 0xb0/0xb8/0xc0/0xc8` ✓ 이펙트 4종·level·radius 오프셋 전량 OK ✓

### 문서위생(사실오류는 아니나 다음 세션을 오염)
`unknown[]` 에 **이미 `resolved[]` 가 뒤집은 판정 3건이 원문 그대로** 남아 있다 — ①"cooldown 이 남은 틱인지 … **추정**" ②"`Effect::range_adjust` 본체 확인 불가" ③"`CastingTarget::check` 본체 미확인". **`~~취소선~~ → resolved 참조`로 정정할 것.**

## 5. 17 `DeathMatchBattle::new` — ⚠정정 2 · ➕보강 1

### ⚠정정 ① `main_objective` 타입이 틀렸다
명세 `writes[+0x17b].note` = "**`Option<Option<MainObjective>>`** 3B" → **tcx 정본은 `Option<MainObjective>` (3B) 단일 Option**. (`SinglePlanBattle+0x8d` 도 동일.) 이중 Option 으로 읽으면 니치가 한 겹 어긋나 재구현 코드가 실제로 틀린다.

### ➕보강 ② 그 결과 `unknown` 5번(0x17c/0x17d 정체)이 닫힌다
`tcxdict --enum MainObjective`: 3B, 태그 `+0x0`(0..11), `Morgard`/`Serpen` 페이로드 = **`phase: ObjectPhase @+0x1` · `with_battle: bool @+0x2`**. ⟹ None 니치는 태그바이트 `0xff` 하나이고 **`0x17c/0x17d` 는 그 phase/with_battle 자리** — "확인 안 함" → **확정**.

### ⚠정정 ③(경미) `flee_die` 센티널 이름
`writes[+0x118].note` = "**usize::MAX** 센티널". 실제 저장값 `9223372036854775807 = 0x7FFF…FF` = **`i64::MAX`**(= `usize::MAX/2`). `logic` 블록은 `i64::MAX` 로 맞게 적혀 **두 곳이 서로 모순**.

### ✅ 나머지(IR 전량 대조, 오류 0)
`m05.ll:17695~17825` 직접 대조 — 36필드 전부 초기화 ✓ / `Chat::Battle` = `i8 3`, `+0x8=goal.TryKill.__0`, `+0x10=0` ✓ / `start_tick` = vtable **+40(0x28)** 간접호출 ✓ / `memset(%0+288=0x120, 0, 80)` ✓ / `dive_tower(+0x17a)=-1`, `main_objective(+0x17b)=-1` ✓ / `base_sub_goal(goal, version, player, data)` = tcx `fn(&BattlePlanGoal, usize, &PlayerState, &OperationData)` ✓ / `DeathStance`·`BattleTactic`·`DmScene` 태그 = 선언 인덱스(니치 밀림 0) — `dienum` 으로 적었던 값이 tcx 정본과 일치 ✓ / `Chat::{Serpen,Morgard}Prepare` 튜플 variant ✓

## 6. 18 `v3_epicops_buff_window` — ⚠정정 2 · ➕보강 1

### ⚠정정 ① 피호출자 시그니처 2건이 **argpromotion 아티팩트** (§2)

| 명세 표기 | 소스 정본(tcx + DWARF) |
|---|---|
| `logic`: `v3_epicops_repair_need(team, data.cache, plan)` / `resolved`: "`fn(team, cache, plan)`" | **`fn(player:&PlayerState, data:&OperationData, plan:&BigPlan) -> u8`** |
| `resolved`: `v3_group_press_line(team, cache, ctx, skip)` | **`fn(&PlayerState, &OperationData, Option<LineType>) -> Option<LineType>`** (3인자) |

`reads[PlayerState+0x930].note` 의 "`v3_epicops_repair_need` 의 **1번 인자**" 도 같은 이유로 부정확. ➕보강 = 그 promotion 자체가 "repair_need 는 player 에서 `info.team` 만·data 에서 `cache` 만 쓴다"의 증명.

### ⚠정정 ② `Chat::Press`/`PressChange` 소스 줄 = **674/676**, `is_none` = **673** (682 아님)
`constants[21]`·`constants[22]` 의 `src_line` 이 682, `logic` 주석도 "682줄의 is_none". `dloc !14971` 실측 체인 = `option.rs:633 is_some` ← `option.rs:682 is_none` ← **`epic.rs:673`** ⟹ **682 는 `core/src/option.rs` 줄번호**(숫자 우연 일치). 태그 선택 `select i1 %149, i8 21, i8 22` 의 `!dbg` 루트는 `line:0`(Vec::push 인라인)이고, push 는 `!14972/!14974 = DILocation(line:0, inlinedAt:!14973/!14975)` = **`epic.rs:674`/`676`**. 같은 명세의 `still_unknown` 은 674/676·673 으로 맞게 적혀 있어 **`constants`/`logic` 만 갱신 누락** — `constants` 를 기계 소비하는 도구가 잘못 앵커링한다.
참고: `epic.rs:682` 는 **실재 줄**(`!14808`, `lifetime.end` + 공통 출구 `br`) = 함수 꼬리. 「682 가 없다」가 아니라 「682 는 그 문장이 아니다」.

### ✅ 나머지(오류 0)
`MainObjective 1 Serpen(phase@+1, with_battle@+2)` / `7 Repair` · `ObjectPhase 0 None,1 Setup,2 Assemble,3 Hunt` — **`tcxdict --enum` 상 니치 밀림·선언순서 함정 모두 없음**(브리핑이 경고한 `ObjectPhase` 함정은 이 열거형엔 **해당 없음**: idx=선언discr=메모리태그).
`Chat 21 Press(LineType@+0x1, usize@+0x8)` / `22 PressChange`(동일) / `23 Repair` / `25 SerpenSetup` 페이로드 오프셋까지 tcx 일치 ✓ · 함수 IR 헤더 인자 7개·`version range(i64 2,0)` ✓ · `TeamPlan` 0xc0/0xc8/0xd0/0x410/0x41e/0x41f OK ✓
브리핑이 지목한 「1번 인자 = `strategy.morgard_use`」·「thread_rng 난수」·「epic.rs 651~682 줄 복원」은 **이미 반영돼 있다 — 빠진 것 없음**.

## 7. 19 `best_jungle_goal` — ✅확인 (오류 없음)

**오라클 end-to-end 10/10 MATCH**(`D_oracle4.exe`). SDK 의 실제 반환값과, 명세 규칙("미클리어 캠프 중 제곱거리 최소, 동점이면 배열 앞")의 손계산이 팀×포지션 10칸 전부 일치.

실행이 부수로 확인해 준 것:
- `MapDef::camp_pos` 좌표표가 `resolved.좌표표` 와 **완전 일치** — team0 `Rhino(496000,752000) Mushroom(176000,592000) Bee(351000,800000) Stump(256000,448000)`, team1 대각 미러 ✓
- `is_cleared`/`is_side_cleared` 가 초기 상태 전부 `false` ⟹ `not_cleared_camps`=4 ⟹ **본선 경로가 실제로 돌았다**(폴백 아님) ✓
- `now_camp=Some(Bee)` 를 줘도 결과 불변 ⟹ **`now_camp` 는 챔프 없을 때만** ✓

IR 재대조도 어긋남 0: `jungle_camps` = `store i8 0/1/3/2` = **[Rhino,Mushroom,Bee,Stump]** ✓ / filter 심 = `is_cleared(camp, team, poison, poison, player, data, team_plan, 0, poison)` **xor true** — `version`/`rnd`/`debug` 가 실제 `poison` ✓ / `SliceRandom::choose(jungle_camps, 4, rnd)` = **4칸 배열** 대상 ✓ / `camp_pos(map, camp, icmp eq team,0)` = `is_blue` ✓ / 폴백 진입 블록 `%62` pred = 엔트리뿐(두 경로 배타) ✓ / tcx `sig` 9인자·7인자 일치(argpromotion 함정 없음 — 둘 다 pub) ✓

## 8. 산출물

| 파일 | 무엇 |
|---|---|
| `_verify\D_audit.py` | 담당 5건 `reads`/`writes` + 산문 오프셋 전수 감사(`tcxaudit` 재사용) |
| `_verify\D_lines.py` | IR 줄범위 `!dbg` 를 `inlinedAt` 루트까지 전개. ⚠`#dbg_value(...,!N)` 위치는 안 잡는다(`!dbg !N` 만) — 674/676 을 처음에 놓쳤던 원인 |
| `_verify\D_mdlines.py` | 모듈 전체 `!DILocation` 을 스코프 파일별 집계 |
| `_verify\D_oracle1.rs` | `&OperationData` 구성 가능성 반증 |
| `_verify\D_oracle2.rs` | `add_player`+`start_game` 으로 `&PlayerState`·`&Entity` 확보 |
| `_verify\D_oracle3.rs` | 16/15/19 진리표(디폴트 데이터 한계 포함) |
| `_verify\D_oracle4.rs` | 19 최근접 규칙 손계산 대조 — **10/10 MATCH** |

exe = `%TEMP%\tfm2_spanprobe\D_oracle*.exe`

## 9. 다음 세션에 남기는 것 (3분류)

- **미탐색**: ① `Effect::is_in_range`(game_core) 본체 — `engage_requires_dive` 실제 거리식 ② `SinglePlanBattle::update` 가 `sub_goal` 을 `KitingBack/RunAway/End` 로 떨구는 조건(15 의 실질 판단 전부) ③ 오라클에 **실전 챔피언 데이터** 싣기(`ChampionInfoSheet`/`engine_core::assets::Assets`) — 되면 16·전투계열 전체가 진리표 대상 ④ `setting.tick_per_second = 60` 으로 오라클 재실행
- **재료 부재**: `epic.rs` 648~650·662~663·685~686 의 한글 `//` 라인주석(rmeta 미포함) — 종전 판정 유지
- **표기 불가**: 이 5건에서 새로 발생한 것 없음
