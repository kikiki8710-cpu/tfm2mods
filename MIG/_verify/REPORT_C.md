# REPORT_C — 7차 반증 배치 (담당 10·11·12·13·14)

> 게임 0.5.8 / SDK `sdk_058` / toolchain `nightly-2026-05-24` / 2026-09-11
> 임무 = **반증**. 맞는 것은 "확인"만 하고, 틀린 곳·빠진 곳·범위가 과한 판정만 상세히 적는다.

## 0. 판정 요약

| # | 함수 | 판정 | 한 줄 |
|---|---|---|---|
| 10 | `should_end_object_finish_kill_priority_battle` | ✅확인 + ➕보강 | 오류 없음. `is_recent_visible` 확정사실(관측 주체·120틱)이 본문에 반영 안 됨 |
| 11 | `v3_fall_back_to_passive` | ✅확인 | **오류 없음.** 공유사실(`rule_scope.rs:96`·`SubPlan::merge`·`mf_src`) 전부 반영돼 있음 |
| 12 | `handle_chat` | ✅확인(본체) + ➕보강 4 | 게이트/트레이스 래퍼 본체는 무결. `handle_chat_inner` 서술에 **미커버 구간 4개** |
| 13 | `target_bush_v30` (Cover) | ⚠정정 1 + 🔁범위정정 1 + ➕ | `logic` 안 `is_top_side` **극성 반전**(같은 파일 `resolved` 와 자기모순) / "부시 22·23·24 영구 제외"가 과대 |
| 14 | `LineGankerPlan::update` | ⚠정정 1 + 🔁범위정정 1 + ➕대형 1 | 같은 극성 반전이 `logic`·`constants`·`knobs` 전반 / "확정 불가"가 실은 **미탐색**(오라클로 해소) / **`target_bush_v41`(lead 표) 통째 누락** |

## 0-b. 기계 검증 결과 (제출 전 필수 절차)

- `tcxaudit --prose` : 담당 5파일에서 산문 오프셋 주장 **1건**만 매칭(`effect+0x18` → `growth_range`, **OK**). 나머지는 구조화 `reads/writes` 라 산문 스캐너에 안 걸림 → 자체 감사 도구를 새로 씀.
- **`_verify/C_audit.py` → `_verify/C_audit.tsv`** : 5명세의 `reads`+`writes` **109행 전수**를 `tcxdict` 사전과 대조.
  - **대조성공 100 / 오귀속 0 / 밀림 0**
  - SKIP 6 = vtable 슬롯 5(`tcxdict` 소관 아님, `divtable` 소관) + 형식 1(`0x18/0x20` 복합표기)
  - 확인불가 3 = `AbstractGameWithCache+0x8`(= `game: &dyn` 16B 팻포인터의 뒤 절반. tcx 는 참조 경계에서 멈춘다 = **도구 한계**이고 명세 주장은 맞다. specs20 감사 때와 동일 현상)
- 열거형은 전부 `tcxdict --enum` 으로 재확인: `MainObjective`/`ObjectPhase`/`ObjectFinishStrategy`/`GameMode`/`TutorialType`/`BigPlan`/`BigGoal`/`SubPlan`/`Position`/`TraceLevel`/`TraceEventType`/`CancelReason`/`Chat(17)`/`LineGankerPhase`/`AroundBushOutlineType` — **틀린 태그 0**.
- ★새 오라클 1건 실행(아래 §4-A) : `_verify/C_o_side.rs` → `_verify/C_o_side.tsv`.

---

## 1. `10_should_end_object_finish_kill_priority_battle` — ✅확인

### 재확인한 것 (전부 일치, 오류 없음)
- 진입 게이트 `and i24 %4, 65534` / `icmp eq 768` : m10.ll:49632~49634 **원문 그대로**. `tag ∈ {0,1} ∧ phase==3(Hunt)`, `with_battle` 미검사 — 맞다.
- 출구 phi(m10.ll:50099) 전수 확인: `%5→false`(마스크) · `%10→false`(전략) · `%27/%28→true`(비-Moba) · `%29/%37→true`(live_list 빈) · `%45→true`(entity null) · `%81/%102/%123/%144/%165→false`(적 5슬롯 언롤) · `%167→true`. **명세의 반환 논리와 완전 일치.**
- `Strategy+0xf object_finish` / `ObjectFinishStrategy::KillPriority=0` / `ObjectPhase::Hunt=3` — tcx 확정.
- `blackboard[1-team]` 인덱싱 : IR 의 gep 가 `iter_champions` 와 **같은 `%56`** 을 쓰는 것 확인(m10.ll:49797). 명세 서술 그대로.
- `can_enemy_hit_objective` 의 `19600000000 = 140000²` 조기탈출 : m10.ll:47496~47497 `icmp ugt %22, 19600000000` → 확인.

### ➕보강 (공유사실 미반영)
1. **`is_recent_visible` 의 의미가 본문에 없다.** `logic` 은 여전히 `(★인덱스가 e_team 인 것은 관측 사실 — unknown 참조)` 로 남아 있고, `unknown[0]`/`unknown[1]` 도 옛 문구 그대로다(`resolved` 에 "_shared 참조"만 한 줄). 확정된 뜻을 본문에 박아야 한다:
   - `Blackboard[T].last_visible[pos]` = **팀 (1−T) 가 팀 T 의 pos 챔피언을 마지막으로 본 틱**.
   - 여기서 `T = e_team` 이므로 `1−T = 내 팀` ⟹ 이 루프의 뜻은 **"내 팀이 최근에 본 적 챔피언인가"** 이고, 명세가 남긴 "적 팀 blackboard 로 적을 묻는 게 무슨 뜻인지 모르겠다"는 의문은 해소된다.
   - "최근" = **`last_visible[pos] + 120 ≥ tick`, tps=60 ⟹ 2.0초 고정**(tps 스케일 안 됨). 지금 보이면 즉시 true(먼저 검사).
   - 파생 노브: 120 을 올리면 **적이 시야에서 사라져도 한동안 "아직 근처"로 쳐서 오브젝트 전투를 안 끝낸다.** `knobs` 에 없다.
2. `constants` 65534 의 설명 `"byte0 의 bit0 만 남기고"` 는 읽기에 따라 반대로 읽힌다(실제는 **bit0 을 마스크에서 뺀다**). 괄호의 `(태그 0/1 허용)` 로 뜻은 복구되므로 정정이 아니라 표현 지적만.

---

## 2. `11_v3_fall_back_to_passive` — ✅확인 (오류 없음)

전 항목을 IR 원문(m13.ll:12238~12479)과 대조했고 **틀린 곳이 없다.**

- 게이트 3단: `icmp ult %1, 2`(v<2 return) → `icmp eq mode,2`(DeathMatch return) → `icmp eq mode,1`(SingleLane 하드코딩 플랜). 전부 일치.
- SingleLane 플랜 store 5개(`+8=0, +16=inttoptr 8, +24=0, +32=0, +33=1, tag=4`) — `SinglePlanLine{chats@0x0,in_recall@0x18,line@0x19}` + BigPlan 페이로드 시작 `+0x8` 로 환산해 **완전 정합**.
- 튜토리얼 화이트리스트 6표: IR switch 원문과 1칸도 안 어긋남 — Top`{0,2,7,8}` / Mid`{0,4,5,7,8}` / Bottom`{0,1,3,5,7,8}` / Epic `add -7, ult -6`={0,7,8} / Serpen`{0,5,7,8}` / Jungle `add -8, ult -7` ∪ `==6` = {0,6,8}.
- `4·5·6 → label %61` 명시 case, default `%41 unreachable` — 확인. `resolved` 가 이미 `rule_scope.rs:96` `|` 결합 arm(77자 검산)까지 반영.
- 오프셋 전량 IR 리터럴로 재확인: `+248=0xf8 team_plan` · `+1512=0x5e8 plan` · `+1896=0x768 sub_plan` · `+2448=0x990 positioning_score` · `+5648/5656=mf_swap` · `+5672=v3_lapse_passive_fallbacks` · `%0` 자체가 `dereferenceable(248)`=GoalData.
- 공유사실 반영 상태: `SubPlan::merge` 화이트리스트 `{4,7,8,9,11,12,15,16}`(= 태그 `{6,9,10,11,13,14,17,18}` 의 리맵 인덱스) **정확**. `mf_swap.__0` load 0건(순수 텔레메트리)도 반영됨.

### 오라클 (시도 판정)
`v3_fall_back_to_passive` 는 pub 이지만 인자에 **`&PlayerState`(2528B)·`&OperationData`·`&mut DebugFrameData`** 가 있어 pub 생성자·`Default` 가 없다 ⟹ **이 방식으로는 구성 불가**(IR_TOOLKIT §7 한계 2). 억지 제로버퍼는 쓰지 않았다. 단 **판정의 실체인 `rule_scope::goal_allowed` 는 이미 오라클 진리표로 검증됨**(`_oracle/goal_allowed.json`)이라 실질 공백은 없다.

### ➕(아주 작음)
- `still_unknown[0]` 이 "`EpicPoke(12)`·`SerpenPoke(15)` 는 switch 에 없어 항상 덮어쓰기"라고 정확히 짚었는데, **`_shared.SubPlan_merge.그외` 목록에는 `EpicPoke` 가 빠져 있다**(8개만 나열, 실제 덮어쓰기 대상은 9개). 공유사실 쪽을 고치는 게 맞다.

---

## 3. `12_handle_chat` — ✅확인(본체) + ➕보강 4

### 재확인 (오류 없음)
- 게이트 3단 IR 원문 일치: `%3+2496(0x9c0) == from` → `ctx+56(0x38)` 별 4-arm switch → `rule_scope::chat_allowed` 아웃오브라인 호출 → `ctx+57(0x39) == 0` 이면 inner 후 return.
- `TraceEventType::CallHandled` 태그·페이로드 **7필드 오프셋 전부 tcx 일치**(`chat@0x8 / plan_before@0x20 / plan_after@0x38 / objective_before@0x50 / objective_after@0x68 / from@0x80 / misunderstood@0x84`), 태그 `9223372036854775823` = i64 `-9223372036854775793` ✓, `PendingTraceEvent` 184B·`tick@0xb0` ✓.
- `pending_trace_events` cap/ptr/len = `0x858/0x860/0x868` ✓.

### ➕보강 — `handle_chat_inner` 서술의 **미커버 구간 4개** (전부 이 배치에서 직접 재확인)
1. ★**`chat.rs:245~290` = 합류 거절 후 "글로벌 궁 예약" — 명세에 통째로 없다.** `resolved` 의 Battle arm 서술이 `chat.rs:139~241` 에서 끊긴다.
   IR 실측(m13.ll:30495~30620): `chats.cap==len` → `Entity::can_ult(champ)` → **`self.pending_global_ult_target(+0x530) == None`** → 거리 게이트(`>199999`) → `iter_champions(...).count()==0` → 최종 `store i64 1, +0x530` · `+0x538 = 대상 Entity+0x5c0(id)` · `+0x540 = tick + tps*3`.
   tcx: `LegacyPlanHandler+0x530 pending_global_ult_target : Option<(usize,usize)>` (24B) — **확정**. 유효기간 **3초**(공유사실의 소비처 = `auction::get_small_action` 1곳).
   ⟹ "콜을 거절했다"가 곧 "아무것도 안 함"이 아니다. **거절 대신 글로벌 궁을 예약한다** — 재구현에서 빠지면 궁 사용 타이밍이 통째로 달라진다.
2. ★**`exit_src` 17~21 의 기록 조건이 없다.** IR 실측: `ff_battle_exit`(`+0x15f8=.2 tick`, `+0x1600=.0 코드`) 에 **코드 18**(m13.ll:32325) / **19**(m13.ll:33004) 를 쓰는데, 진입 게이트가 `icmp ugt version, 1` 이 **거짓**(=`version ≤ 1`)일 때 `%1203` 이고 그 안에서 `plan tag == 9(Battle)` 일 때만 store 한다(m13.ll:32256·32313).
   ⟹ **`exit_src` 17~21 은 v0/v1 + Battle 플랜에서만 실기록.** 현행 v3 세이브에서 이 계측이 0이어도 정상이라는 뜻이라 검증 지표로 쓸 때 반드시 필요한 조건이다.
   ⚠부수 함정: m13.ll:33188 의 `store i8 21` 은 `+0x1610`(**`mf_swap.0`**)이지 `ff_battle_exit` 가 아니다. 두 필드가 같은 숫자 이름표를 공유하니(`_shared` 함정 8) 혼동 주의.
3. `stake` 3변수의 소스 줄 — 명세는 최종식만 `chat.rs:218` 로 적었다. 공유사실 기준 `stake` = `:212` / `stake_commit` = `:215` / `stake_veto` = `:216`. **`stake_veto` 는 행동 영향이 있다**(명세의 최종식 `can_help && ((old_pass && !stake_veto) || stake_commit)` 에 이미 들어 있으므로 모순은 없고, 줄번호만 보강).
4. `ff_call_*` **8칸 계측표**가 없다(`knobs` 에 `ff_call_too_far` 만 1회 언급). `+0x15b8 recv / +0x15c0 ignored / +0x15c8 in_battle / +0x15d0 no_help / +0x15d8 too_far / +0x15e0 low_hp / +0x15e8 bail / +0x15f0 join` (chat.rs:141/144/151/204/205/206/239/236). **"콜을 받고도 왜 안 갔는지"가 이 8칸으로 분해**되므로 인게임 검증 지표로 바로 쓸 수 있다 — `new_knobs` 에 올릴 값어치가 있다.
5. (신뢰도 표기) `chat_allowed` 표는 **오라클 진리표로 (A)~(E) 틀린 칸 0** 검증이 끝난 것인데(`_oracle/chat_allowed.json`), 명세에는 IR 근거만 적혀 있다. 신뢰도 등급을 올려 적어두는 편이 낫다.

---

## 4. `13_target_bush_v30` — ⚠정정 1 + 🔁범위정정 1 + ➕

### ⚠정정 ① `logic` 의 `is_top_side` 극성이 **반전**돼 있다 (같은 파일 `resolved` 와 자기모순)

`logic` L165 주석: `map_regions::is_top_side(ctx, champ.x, champ.y)` 의 `본문식 = (ctx.setting.height - champ.y) <u champ.x` .
**틀렸다.** IR 의 `icmp ult (height − y), x` 는 `is_top_side` 가 아니라 **그 부정**이다.

- 같은 파일 `resolved[1]` 은 이미 `is_top_side = (x + y ≤ height)` 라고 옳게 적고 있다 ⟹ **명세 내부 모순.**
- **실행으로 확정**(아래 §4-A 오라클): 9개 표본 전부에서 `ry_lt_x == !is_top_side`.

정정된 읽기(동작은 그대로, **뜻이 뒤집힌다**):

| 소스 줄 | 조건 | 부시 |
|---|---|---|
| L166 | `is_top_side` **참** (= 탑 절반) | team0 **8** / team1 **12** |
| L168 | `is_top_side` **거짓** (= 봇 절반) | team0 **13** / team1 **18** |
| L171 | `is_top_side` 참 → **11** · 거짓 → **14** | (team 무관) |

즉 **탑 사이드 → 8/12/11, 봇 사이드 → 13/18/14** 다. `logic` 을 그대로 읽으면 정확히 반대로 재구현된다.
(`constants` 는 raw 식 `(height−y)<x` 로 적혀 있어 **영향 없음** — 오염된 건 `logic` 주석 한 곳뿐이다.)

### 🔁범위정정 ② `new_knobs` 의 "부시 ID 1/22/23/24 는 갱킹 은신 후보에서 **영구 제외**" — 과대

`target_bush_v30` **한정으로는 맞다**. 그러나 같은 `LineGankerPlan` 에는 **`target_bush_v41`**(`ganker.rs:310`, `_gaibc/m08.ll:94136~94353`)이 있고 이쪽은 **부시 7 과 23 을 반환한다**(§5 표 참조). `target_bush_v41` 은 `LineGankerPlan::sub_plan`(m08.ll:94960)이 무조건 호출한다 = 실제 은신 목표다.
⟹ 판정을 **"`target_bush_v30` 이 반환하지 않는 값"** 으로 좁혀야 한다. 두 함수를 합친 실제 미사용 부시는 **1, 5, 10, 19, 22, 24** 다(적용 범위: `line_gank` 계열 `target_bush_*` 2종 기준. `near_jungle_bush` 등 다른 부시 선택기는 미탐색).

### ✅확인 / ➕
- 반환 16값(2,3,4,6,8,9,11,12,13,14,15,16,17,18,20,21)·타워 6오프셋(0x180~0x1d0)·`player_champion@0x1e0`·`Entity 0x68/0x70/0x88/0x128`·`GameSetting+0x12c0 height` — 전부 IR·tcx 재확인, 오류 없음.
- ★`resolved` 의 **부시 좌표표 24개를 오라클로 독립 재현했다**(§4-A). 셀 수·중심좌표가 전부 일치(11/13/18 만 반올림 1 차이). 부시 ID 는 `MapDef.bushes` 그리드의 **값**이고 1~24, 0=부시 없음 — **재확인 완료**. 이 표는 신뢰도를 "IR 추론"에서 **"오라클 실행"** 으로 올려도 된다.

### §4-A 새 오라클 — `_verify/C_o_side.rs` → `_verify/C_o_side.tsv`
`game_core::is_top_side` 는 **`vis=pub`·`mir=1`** 이고 `GameContext` 는 정상 생성 가능하므로 그냥 실행하면 된다.

    powershell -ExecutionPolicy Bypass -File C:/tfm2mods/MIG/spanprobe.ps1 -Src C:/tfm2mods/MIG/_verify/C_o_side.rs \
      -Extra "--extern bumpalo=C:/tfm2mods/sdk_058/mod-sdk/deps/libbumpalo-dafef1f270bdb02f.rlib -C lto=fat -C codegen-units=1 -C opt-level=1"

`width=height=960000` 으로 세팅한 실측(발췌):

| x | y | x+y | `is_top_side` | `is_bottom_side` | `(h−y)<x` |
|---|---|---|---|---|---|
| 0 | 0 | 0 | true | false | false |
| 80000 | 820000 | 900000 | true | false | false |
| 900000 | 0 | 900000 | true | false | false |
| 480000 | 480000 | **960000** | **true** | **true** | false |
| 864000 | 96000 | **960000** | **true** | **true** | false |
| 959999 | 959999 | 1919998 | false | true | true |

⟹ **`is_top_side = (x+y ≤ height) = !((height−y) < x)`, `is_bottom_side = (x+y ≥ height)`, 대각선은 양쪽 포함** — `_shared.맵_좌표계` 와 완전 일치, 실행으로 확정. 부수로 `nexus_pos=[(96000,864000),(864000,96000)]`, `get_start_position` 6칸도 그대로 재현됐다.

---

## 5. `14_LineGankerPlan::update` — ⚠정정 1 + 🔁범위정정 1 + ➕대형 1

### ⚠정정 ① `is_top_side` 극성 반전이 **`logic`·`constants`·`knobs` 세 군데 전부**에 퍼져 있다
13번과 같은 오류인데 **13번보다 나쁘다** — 13번은 `constants` 가 raw 식이라 무사했지만, 14번은 `constants` 가 이름으로 적혀 있어 같이 뒤집힌다.

| 위치 | 현재 서술 | 정정 |
|---|---|---|
| `logic` ganker.rs:279 | `결과 = (ry < champ.x)` | `is_top_side = !(ry < x)`. 소스는 `if is_top_side { 줄280 } else { 줄282 }` |
| `constants` 13/18 | "`is_top_side` **참**" | **거짓**(봇 사이드) |
| `constants` 8/12 | "`is_top_side` **거짓**" | **참**(탑 사이드) |
| `constants` 14 / 11 | 14="참" / 11="거짓" | **14=거짓 / 11=참** |
| `knobs` "Mid 라인 위/아래 구분선" | "결과 = ry < champ.x" | 같은 반전 |

### 🔁범위정정 ② `unknown[2]` 의 "**확정 불가**" 는 **미탐색**이었다
원문: "ganker.rs:279 소스 조건의 극성. … 별도 define 이 없어(전량 인라인) 확정 불가".
`define` 이 없다는 것은 맞지만, **`is_top_side` 는 `vis=pub` + `mir=1`** 이다(`_tcx/game_core.json`, `map_regions.rs:21:1`). 즉 ①SDK 실행 오라클 ②rmeta MIR 두 경로가 모두 열려 있었다. 실제로 오라클 1회(§4-A)로 해소됐다.
⟹ 판정 어휘를 `표기 불가` → **`미탐색(오라클·MIR)`** 으로 바꿔야 한다. 이 문구를 그대로 두면 다음 세션이 진짜로 포기한다(METHOD_MAP §3 의 "최악의 오염").
같은 이유로 `unknown[3]`("부시가 맵 어디인지 모른다")도 `resolved` 가 이미 닫았으니 문구를 정리해야 한다.

### ➕보강 (대형) — **`target_bush_v41` 이 통째로 빠져 있다. 그리고 이게 실제 목표다.**

명세는 `update` 만 보고 "`target_bush_v30` 이 목표 부시"라고 서술하는데, IR 을 넓혀 보면:

| 함수 | 부시 선택기 | 근거 |
|---|---|---|
| `LineGankerPlan::update`(ganker.rs:39) | **`target_bush_v30`**(ganker.rs:248~) 전량 인라인 | m08.ll:94643~, `dloc !55697 = target_bush_v30 @265 ← update @54` |
| `LineGankerPlan::sub_plan` | **`target_bush_v41`**(ganker.rs:310) 아웃오브라인 호출 | m08.ll:94960 `tail call … target_bush_v41` |

`sub_plan` 은 그 값을 `SubPlan::Hide{ bush, out_line=Outline(1), check_move=0, enemy_spotted_me=0 }`(태그 9)로 내보낸다 — **챔피언이 실제로 가는 곳**이다. `update` 는 `champ_bush == target_bush_v30(...)` 로 "도착"을 판정한다.

⟹ ★**두 값이 다른 구간에서는 `update` 의 도착 판정이 성립할 수 없고, `CancelReason::TargetMissing` 취소가 원리적으로 안 걸린다.** 명세의 knob 주석이 걱정한 "영원히 도착 못 함 → 취소가 안 걸림" 은 가정이 아니라 **현행 코드의 실제 상태**다. (구체 예: Top·team0·`top_lead=0` 이면 v41 은 부시 **16** 으로 보내는데 v30 의 Top·team0 반환집합은 `{2,3,6}` 이라 16 이 없다 ⟹ 그 lead 구간에서는 TargetMissing 취소가 절대 발화하지 않는다.)
`update` 에는 버전 게이트가 없고 `sub_plan` 에도 없다 — **v30/v41 은 AI 버전으로 갈리지 않고 호출자별로 하드와이어**돼 있다.

**`target_bush_v41` 전표** (m08.ll:94136~94353 실측. 인덱스 = `*_lead[team]`, 범위검사 `lead < 7`):

| line | team | lead 0 | 1 | 2 | 3 | 4 | 5 | 6 |
|---|---|---|---|---|---|---|---|---|
| Top(0) | 0 | **16** | 6 | 3 | 3 | 3 | 2 | 2 |
| Top(0) | 1 | 2 | 3 | 6 | 6 | 6 | **16** | **16** |
| Bottom(2) | 0 | 21 | 20 | 15 | 15 | 15 | 9 | **7** |
| Bottom(2) | 1 | 9 | 15 | 20 | 20 | 20 | 21 | **23** |

Mid(1) 은 챔피언 좌표를 쓴다 — `s = !is_top_side`(=봇 사이드) 라 할 때
`table[0] = team0 ? (s?21:17) : (s?9:4)` · `table[1..4] = s?14:11` · `table[5],[6] = team0 ? (s?9:4) : (s?21:17)`
(Mid 만 `player_champion[team][pos]` 를 `unwrap` 한다 = **None 이면 패닉**. Top/Bottom 은 챔피언을 안 본다.)

읽는 필드는 `AbstractGameWithCache` **`top_lead@0x21c0` / `mid_lead@0x21d0` / `bottom_lead@0x21e0`** (`[usize;2]`, IR 리터럴 8640/8656/8672) — **`update` 명세의 `reads` 에는 당연히 없고, 이 세 필드가 갱커 은신 위치를 실제로 지배한다.**

**의미 검산(공유사실 `*_lead` = 라인 시퀀스 자기진영 0 → 적진영 6, 과 정합):** §4-A 오라클로 뽑은 부시 중심좌표로 확인하면 표가 **단조**다 —
team0 Top: lead0 → 부시16 `(16000,656000)`(team0 넥서스 `(96000,864000)` 쪽) … lead5·6 → 부시2 `(656000,16000)`(team1 넥서스 `(864000,96000)` 쪽).
team0 Bottom: lead0 → 부시21 `(416000,848000)` … lead6 → 부시7 `(944000,304000)`.
⟹ **라인 통제가 전진할수록 갱커의 매복 덤불이 우리 진영 → 적 진영으로 한 칸씩 밀린다.** 이게 v41 의 설계 의도다.

### ✅확인 (오류 없음)
- `icmp ult hp*100/max_hp, 41`(m08.ll:94605~94607), `Entity+0x628 max / +0x670 hp`, div-by-zero 패닉 — 확인.
- `phase@0x29 = 8(Cancel)`, `LineGankerPhase` 니치 `+6`, `Chat::Cancel=17` + `CancelReason{LowHpSelf=0, TargetMissing=2}` — tcx 확인.
- `bushes[umin(y/32000,29)][umin(x/32000,29)]`, `MapDef+0x1c98` — 확인.
- `_version`/`_rnd`/`_positioning_score`/`_debug` 미사용 — `update` 본문에 한해 맞다(단 위 ➕ 때문에 "이 플랜에 버전 게이트가 없다"로 **플랜 전체에 일반화하면 안 된다**).

### 미탐색으로 남긴 것 (범위 명시)
- 브리핑이 짚은 **`lead < 3`** 은 **`LineGankerPlan::update` 안에 없다**(m08.ll:94569~94939 전수 grep, `*_lead` 오프셋 8640/8656/8672 출현 0건). `update` 관련 유일한 lead 상수는 v41 쪽 `lead < 7`(범위검사)이다.
  `lead` 를 읽는 다른 지점 = `_gaibc/m13.ll:8916`(`passive_plan`) · `m09.ll:63268·63808·63827·63990·64061·64190` · `m05.ll:44960·52661·52665` · `m15.ll:55322·55440` — **미탐색**(담당 함수 밖).
- `GoalData::has_near_line_enemy`(`_gaibc/m09.ll:3948`) 본문 — 이번에도 안 읽었다. 훑어본 결과 `MapDef+21720(0x54d8) region_dist` 와 `GoalData.enemy_region[i]` 를 `MapDef::line_region(line, 0, k)`(k=2,3,4…) 와 대조해 **홉거리 < 2** 인지 보는 구조로 보인다(**추정**). 검증 방법 = `m09.ll:3948~4100` 전수 독해.

---

## 6. 이번 배치에서 나온 방법론 소득

1. ★**`vis=pub` + `mir=1` 인 헬퍼는 "인라인돼 define 이 없다"가 포기 사유가 되지 못한다.** `is_top_side` 가 정확히 그 사례였고, 명세 2개가 이걸 "확정 불가"/자기모순으로 안고 있었다. **`define` 을 못 찾으면 다음 수는 IR 재탐색이 아니라 `tcxq grep <크레이트> <이름>` 으로 `vis`/`mir` 를 보는 것**이다.
2. ★**`logic` 과 `resolved` 의 자기모순은 QC 가 못 잡는다.** 13번은 `resolved` 에 정답이 있는데 `logic` 이 틀린 채로 남았다 — 재구현자는 `logic` 을 읽는다. 명세 갱신 시 **`resolved` 의 결론을 `logic` 본문에 역반영**하는 단계가 필요하다.
3. ★**`v30`/`v41` 같은 형제 함수는 "버전 게이트"가 아니라 "호출자별 하드와이어"일 수 있다.** 담당 함수만 읽으면 `update` 가 쓰는 v30 이 정답인 줄 알게 된다. **한 플랜을 명세할 땐 그 플랜의 `sub_plan`/`next_plan`/`is_end` 형제까지 최소 한 번은 심볼 목록으로 훑어야 한다**(`grep "^define.*<PlanName>"`).

## 7. 산출물
- `_verify/REPORT_C.md` (이 문서)
- `_verify/C_audit.py` · `_verify/C_audit.tsv` — 담당 5명세 오프셋 109행 전수 대조(오귀속 0)
- `_verify/C_o_side.rs` · `_verify/C_o_side.tsv` — `is_top_side`/`is_bottom_side` 극성 + `MapDef.bushes` 24개 실측 오라클
