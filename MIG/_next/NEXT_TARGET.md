# 차기 RE 대상 사전등록 — `LegacyPlanHandler::update`

> 2026-09-11 / 게임 0.5.8 / 4차 반증검증이 병렬로 돌는 중에 준비
> ⚠`_spec\specs20*.json` 은 4차 에이전트가 읽고 있어 **건드리지 않았다.**

## 1. 대상 선정 근거 (실측)

`game-ai\src\plan_legacy\handler.rs:685` · `vis=pub` · `mir=False`
IR = `C:\tfm2mods\_gaibc\m13.ll:14467~28943`

`_gaibc` 의 `define` **4,231개를 전수 측정**(`_next\irsize.py`)한 결과 **모든 축에서 1위**:

| 지표 | 값 | 2위 |
|---|---|---|
| IR 줄 | **14,476** | 11,379 (`TeamPlan::update_objective_after_steal`) |
| `switch` | **42** | 22 |
| `select` / `phi` | 112 / 196 | |
| **call/invoke** | **1,216** (고유 심볼 **89종**) | 746 |
| `br` | 730 | 751(2위가 약간 많음) |

- game_ai 에서 2,000줄 초과 **94개** · 5,000줄 초과 **21개** · 10,000줄 초과 **3개** → 그 3개 중 1위.
- 우리 20개 명세 중 가장 큰 것(`10`, 489줄)의 **29.6배**.

⚠**tcx 의 `sp`(def_span)로 길이를 재면 안 된다** — 그건 **함수 시그니처 범위**라 game_ai 전 함수가
최대 17줄로 나온다(실측). 본문 길이는 **IR `define` 본문 줄 수**로 재야 한다.

### 왜 "제일 크다" 말고도 이게 맞는가
1. **AI 계층 최상위 진입점.** `_gaibc`·`_gcbc`·`_gvbc` **세 코퍼스 전부에서 호출부 0곳** →
   게임 exe 가 직접 부른다(매 틱 AI 판단의 출발점).
2. **우리 20개 중 5개를 직접 호출**: `05 v50_fold_dive_episode` · `09 check_favorable_engage_formation` ·
   `12 handle_chat` · `15 single_try_engage` · `17 DeathMatchBattle::new` → 기존 명세가 하위 문서로 붙는다.
3. **오라클이 열린다**(§3 실증). `&mut self` 라 3차 배치C 의 **스냅샷 바이트 diff** 기법이 그대로 먹는다.
4. **부수 효과**: 3차 배치B 가 `05`·`06` 의 오라클이 `E0624` 로 막혀 "`AiAgent::update` 로 전체 시뮬
   간접 관측"을 미탐색으로 남겼다. 이 함수는 `pub` 이고 **05 를 직접 부른다** → 그 경로가 뚫린다.
5. **모드가 이미 절반 안다.** `tfm2_ai_adjust\src\judge\agent_twin.rs`(1,006줄)가 `LegacyPlanHandler`
   (6,168B)를 DI 로 전개해 twin 비교에 쓴다("에이전트의 가장 큰 상태(플랜 계층 전체)").
   **구조체는 아는데 그걸 굴리는 함수를 모르는 상태.**

### 각오할 것
- `mir=False` → MIR 없음. **IR + DWARF + 줄 길이 산술 + 오라클** 조합으로 가야 한다(20개 때와 동일).
- 29.6배라 한 세션에 안 끝난다 → 블록 분할 + 배치 병렬이 전제.
- 1,216 call 중 상당수가 디스패치일 가능성이 있어 **"넓고 얕은" 쪽**일 수 있다.

### 2순위 후보 (깊이를 원하면)
`position_eval_at_uncached`(position_eval.rs:371, 9,480줄, **`phi` 266 = 분기 밀도 1위**, 순수 채점).
⚠`in:game_ai::position_eval` 이라 직접 호출은 막히고 크레이트 루트 재수출
**`game_ai::position_score_at_cell`(pub)** 을 경유해야 한다.

## 2. 사전(tcxdict) 등록 — 결론: **거의 필요 없다**

사전은 tcx 에서 자동 생성돼 **타입 16,355 · by_path 5,082 · by_leaf 2,295** 가 이미 들어 있다.

### 명세 작성자가 실제로 칠 짧은 이름 60종 시험 → **60/60 OK · 모호 0 · 없음 0**
(`LegacyPlanHandler`·`BigPlan`·`SubPlan`·`TeamPlan`·`GoalData`·`Blackboard`·`MainObjective`·
`LineGankerPhase`·`V50DiveEpisode`·`BrainMinionParameter`·`FightPrediction`·`ScoreParameter` … 전부 유일 해결)
⟹ **별칭 등록은 불필요.** 착수 첫날부터 `tcxaudit` 이 그대로 돈다.

### 손으로 등록한 것 = 트레이트 @SKIP 뿐
`resolve_name` 은 **타입 사전**이라 트레이트를 못 찾는다(구조체·열거형만 있다).
`AbstractGame+0x28` 같은 vtable 슬롯 표기가 `확인불가` 로 떨어지지 않게 `tcxaudit.ALIAS` 에
**`@SKIP:트레이트`** 로 박았다(기존 `dyn AbstractGame vtable` 관례와 같은 방식):
`AbstractGame` · `ChampionInfo` · `EffectType` · `ItemInfo` · `ModeBrain` · `AiAgent` · `Action` ·
`PlayerInputAi` · `EntityPassiveRunner` · `BanpickRunner`

### 참고 — 제네릭 컨테이너는 등록하면 **틀린다**
후보 1,996종 시험에서 모호 34건이 나왔는데 거의 전부 제네릭이다:
`PhantomData`(후보 1,487) · `Result`(628) · `Option`(557) · `RawVec`(416) · `Vec`(415) ·
`HashMap`(202) · `Unique`(139) · `Box`(138) · `UnsafeCell`(87) · `Arc`(36) · `Rc`(32) ·
`RefCell`(50) · `bumpalo::Vec`(12) · `Container`(5) · `DataTable`(20) · `Style`(52).
⟹ **단일 별칭을 박으면 잘못된 인스턴스로 해결된다.** 필요한 것은 별칭이 아니라
**그 필드가 실제로 쓰는 구체 인스턴스 경로**이고, 그건 아래 §4 필드표가 준다.

## 3. ★오라클 실증 — **된다**

프로브 `_next\h1_probe.rs` (빌드 = `sh MIG\_verify3\build.sh …`), 실행 출력:
```
setting_ok	true	width=960000	height=960000	tps=60	champ_radius=10000
sizeof_LegacyPlanHandler	6168	(기대 6168)
towers	16	twin0=2	twin1=2

version	ret	diff_runs	diff_bytes	first_runs
0	ok	17	57	0x148..0x149,0x514..0x515,0x708..0x70e,0x710..0x711,…
1	ok	18	63	0x148..0x149,0x150..0x156,0x514..0x515,0x708..0x70e,…
2	ok	23	68	0x148..0x149,0x150..0x156,0x514..0x515,0x548..0x549,0x551..0x553,0x559..0x55b,…
3	ok	20	62	(40·50·60 과 동일)
```
확정 사실:
- **`LegacyPlanHandler::update` 를 직접 호출할 수 있다**(pub, 크래시·패닉 0).
- 생성자 = `LegacyPlanHandler::new(version: usize, &mut StdRng, team: usize, Position)` (tcx, pub).
- `update` 시그니처 = `fn(&mut self, usize version, &mut StdRng, &PlayerState, &OperationData,
  &mut DebugFrameData, bool)`.
- **6,168B 스냅샷 diff 로 `writes` 를 오프셋 단위로 읽을 수 있다**(필드 pub 불필요, 반환형 `()` 무관).
- ★**version 게이트가 이미 보인다**: `ver 0` 은 `+0x150..0x156` 이 안 바뀌고, `ver 2` 에서만
  `+0x548`·`+0x551`·`+0x559`(= `v3_dest` 대역)가 추가로 바뀐다. `ver 3/40/50/60` 은 동일(20 runs).
  ⟹ **버전 분기가 최소 3구간({0}, {1}, {2}, {3+})으로 갈린다**는 것이 첫 실행에서 나왔다.

⟹ **14,476줄을 입출력 진리표로 공략할 수 있다. IR 독해는 보조.**

## 4. `LegacyPlanHandler` 필드 정본표 — 112필드 / 6,168B

전량 = `_next\lph_fields.json`(tcxdict 산출). 우리 20개가 쓰는 것과 정합한 주요 필드:

| 오프셋 | 필드 | 타입 |
|---|---|---|
| `+0x0` | `data` | `game_ai::GoalData` |
| `+0xf8` | `team_plan` | `TeamPlan` |
| `+0x520` | `battle_start_tick` | `Option<usize>` |
| `+0x530` | `pending_global_ult_target` | `Option<(usize, usize)>` |
| `+0x548` | `v3_dest` | `Option<(u64, u64)>` |
| `+0x570` | `v50_dive_ep_live` | `Option<V50DiveEpLive>` |
| `+0x5e8` | `plan` | `BigPlan` |
| `+0x768` | `sub_plan` | `SubPlan` |
| `+0x7c8` | `chats` | `Vec<Chat>` |
| `+0x858` | `pending_trace_events` | `Vec<PendingTraceEvent>` |
| `+0x888` | `v50_dive_episodes` | `Vec<V50DiveEpisode>` |
| `+0x990` | `positioning_score` | `PositioningScoreData` |
| `+0x1468` | `version` | `usize` |
| `+0x1480` | `last_dive_abandon_tick` | `usize` |
| `+0x15f8` | `ff_battle_exit` | `(u8, u8, usize, u8, u8)` |
| `+0x1610` | `mf_swap` | `(u8, usize)` |

### ★모드 DI 판과 교차검증 — **38/38 일치**
`agent_twin.rs` 에서 이름이 겹치는 오프셋 주장 38건을 tcx 와 대조해 **전건 일치**.
⚠내가 처음에 `version` 1건을 불일치로 집었는데 **오탐**이었다 — `0x2858`/`0x2910` 은
`AgentVerHamster` 의 `small_action`/`version`(**다른 구조체**)이고, 모드는 LegacyPlanHandler 를
실제로 `0x530 + 0x1468` 로 읽는다(agent_twin.rs:374·375·395). 같은 leaf 이름을 다른 구조체에서
긁는 것이 오탐의 원인 — **오프셋 대조는 반드시 소속 구조체를 고정하고 하라.**

### ★신규 발견 — 카운터 블록의 슬롯↔이름 매핑이 한 칸 밀린다
`agent_twin.rs:357` 은 「`plan +0x1468‥+0x15f8` = usize **50개**. 포인터도 패딩도 없는 유일하게
완전히 믿을 수 있는 상태」라고 적었다. tcx 실측:
- **필드 수는 49개**, 범위 `+0x1468 ~ +0x15f0`, 총 **400B**
- 그중 **`last_lost_fight`(+0x1490)가 `(usize, usize)` = 16B**, 나머지는 전부 `usize` 8B
- 8B 간격이 아닌 곳도 그 한 군데뿐(`0x1490 → 0x14a0`)

⟹ **바이트로 400B = u64 50칸이라는 모드의 서술은 맞다**(비교용으로는 정확하다).
그러나 **슬롯 인덱스 → 필드 이름 매핑은 `+0x1490` 이후 한 칸 밀린다.**
`counter50[k]` 로 이름을 붙이는 코드·문서를 쓸 때 반드시 보정할 것.

## 5. 남은 사전준비 (착수 시)

- 블록 분할 지도: `m13.ll:14467~28943` 의 `!dbg` inlinedAt 루트로 `handler.rs` 줄 분포를 내면
  자연 블록이 보인다(20개 때 `dloc.py`/`inlsites.py` 로 하던 것).
- 줄 길이 표: `rmeta_srcmap game_ai handler.rs <시작> <끝>` — 먼저 함수 끝 줄을 확정해야 한다.
- 89종 피호출 심볼의 `callees` 표는 **`mkspec3.py` 가 자동 생성**한다(사람이 쓸 일 아님).
- `LegacyPlanHandler` 형제 **41개**도 `siblings` 가 자동 생성한다(`spec3lib.py sib LegacyPlanHandler`).

---

# 6. ★prior-work 조회 결과 (§7 착수 전 필수) — **판정 [부분]**

재시도금지·폐기 판정은 **없다.** 그러나 이 함수는 **처음부터가 아니다** — 프로젝트에서 가장 오래
다뤄온 함수이고 0.5.8 자산이 이미 다 깔려 있다. **내 §1~§5 계획을 아래대로 고친다.**

## 6-1. 심볼 등호는 **이미 확정돼 있었다** (내가 5분 쓰려던 것 — 불요)
`C:	fm2mods\MIG\dllmatch.json` 에 문자 단위로 박혀 있다(직접 확인):
```json
{"addr":"0x140e4c5c0","rva":14992832,
 "mangled":"_RNvMNtNtCshdEBA0ozCnw_7game_ai11plan_legacy7handlerNtB2_17LegacyPlanHandler6update",
 "name":"update","mod":"handler","jaccard":0.972,"contain":0.995,"bytes":42054}
```
`dllmatch.py` 는 정본이다(`MEM\DONE.md:143` **DONE** · 3축 = 오프셋지문+콜그래프전파+DWARF줄).
⟹ **`0xe4c5c0`(42,054B) == `handler.rs:685 update` 확정.** `name2rva`/`panicloc` 로 다시 확정하지 마라.

## 6-2. ★내 §1-2 가 틀렸다 — 자식 명세는 **5개가 아니라 7개**
IR 본문의 **직접 call 만** 봐서 2개를 놓쳤다:
- **`11 v3_fall_back_to_passive`** = `handler.rs`(**update 와 같은 파일**)
- **`06 v2_response_retreat_stance`** = `handler\engage.rs`
⟹ handler 계열 명세 = **05 · 06 · 09 · 11 · 12 · 15 · 17 (7개)**.

## 6-3. 구버전에 **game==mine 재현·런타임검증까지 끝난 블록**이 있다 — 페이즈게이트
`ANA\_archive\plan_v2-판단함수-계산식-정본.md:105~130` §2 「드라이버 / 페이즈 게이트」 **✅공식재현·런타임검증**
```
objective  = min(C * A / 1000, 100)
threshold  = objective*9 + min(B,100)*2 + 100      (cap 1000)
roll < threshold → transition_engine 호출 + demote
   A = athlete+0x218 (실효 판단력) · B = +0x238 (에고) · C = +0x380 (동적)
```
**개입 실증까지 됨**: `pg_a=30` 주입 → 라인전→능동 전환률 100% → 57%.
⚠파일 상단 `[STALE-RVA]` = 주소는 0.4.12~13, **공식·구조는 영구 유효**. 메모리 정본 = `MEM	fm2-phasegate-judgement.md`.
⟹ **이 블록은 다시 읽지 말고 승계**한다. 할 일은 0.5.8 좌표로 옮겨 붙이는 것뿐.

## 6-4. 0.5.8 디컴이 이미 있다 — **새로 뜨지 마라**
- Ghidra 디컴 **32,528행 + capstone 선형 디스어셈** = `MIG\decomp\0.5.8\plan_legacy\handler.md:217`
  (`## 0xe4c5c0 — 원본 행 977~2281`, 42,054B). 과거 "디컴 실패"였다가 timeout 2400s/800MB 로 성공한 그 함수다.
- 0.5.7→0.5.8 diff = `MIG\aidiff_057_058.md:706` (`0xe723c0 → 0xe4c5c0 EDITED`, 41,844→42,054B)
- `aimap.json` 의 `0xe4c5c0` 패닉 줄 8개 = `977·1039·1059·1303·1364·1508·1619·2281`(직접 확인)

## 6-5. ★`0xe5d5d0 handle_interact_battle` 과 **한 세트로** 봐야 한다
그동안 "engage 32KB"로만 불린 것의 정체 = **구 `transition_engine`**. 페이즈게이트가 `roll < thr` 일 때
호출하는 **바로 그 전환엔진**이다. 구조규명 문서가 이미 있다(`discovered-PROGRAM-STRUCTURE.md:371` + §3c).
⟹ **update 단독으로 읽으면 전환 경로의 절반이 빈다.**

## 6-6. 형제 실명·RVA 전량 매핑돼 있다 (새 작업 0)
`dllmatch.json` 조회로 IR 심볼 → exe RVA 가 바로 나온다. handler 계열 주요분:

| RVA | 크기 | 실명 | jaccard |
|---|---|---|---|
| `0xe4c5c0` | 42,054 | **`update`** ← 대상 | 0.972 |
| `0xe46bc0` | 11,615 | ★**`passive_plan`** (update 의 직접 콜리) | **1.0** |
| `0xe6b800` | 5,680 | `check_kill` | 0.837 |
| `0xe4b070` | 1,371 | `v2_apply_assign_commit` | 1.0 |
| `0xe6d000` | 1,152 | `calculate_nexus_defense_count` | 0.971 |
| `0xe70c70` | 809 | `take_misunderstood_received_chat` | 1.0 |
| `0xe4abc0` | 768 | `sanitize_rule_scope` | 1.0 |
| `0xe4a780` | 583 | `v3_assign_anchor` | 1.0 |
| `0xe6fe60` | 445 | `v3_fall_back_to_passive` | **0.121** ⚠ |

## 6-7. ★★신규 발견 — `dllmatch.json` 676행 중 **94행이 jaccard < 0.6** 이다
`MEM\INDEX.md:257` 의 품질 기록("676행·주소중복 0·심볼중복 0·자체검증 7/7")은 **유일성**에 대한 것이고
**신뢰도가 아니다.** 우리 20개 명세의 `exe.addr` 를 dllmatch 와 전수 대조한 결과(직접 실행):

| 결과 | 수 |
|---|---|
| **주소 불일치** | **0** ← 좋은 소식 |
| 일치하나 **jaccard < 0.6** | **6** |
| dllmatch 에 없음 / spec.exe 없음 | 6 |

신뢰도 낮은 6건 = `04 handle_line_defense`(j 0.499) · `10 should_end_object_finish_kill_priority_battle`(0.221) ·
`11 v3_fall_back_to_passive`(0.121) · `12 handle_chat`(0.137) · `15 single_try_engage`(0.25) ·
`16 max_range_nearly_can_use`(0.235).
그 밖에 `defensive_crisis` 는 dllmatch 가 `mod=vec`(j 0.5)로 잡고 있고 우리 spec 엔 `exe.addr` 가 없다.

⟹ **주소 자체는 다 맞았지만, dllmatch 의 이름 신뢰도를 확인 없이 쓰면 안 된다.**
exe 쪽 작업(디스어셈·훅·바이트패치)에 RVA 를 쓸 때는 `jaccard`/`contain` 을 같이 읽고,
0.6 미만이면 **패닉 줄 대조로 한 번 더 확정**할 것. 이름충돌 19건은 `MIG\dupconflict.json`
(**자동수정 금지·보류**, `DONE.md:132`).

## 6-8. 적용되는 인접 판정 (재시도금지 계열)
- `DONE.md:83` **재시도금지(문자열 탐색)** — `MF_SRC_NAMES`/`FF_BAIL_NAMES`/`MF_OBJCLR_NAMES` 는
  배포물 전량 부재(사내 상수) ⟹ **update 의 텔레메트리 코드 이름 찾기 금지.**
- `reimpl-tracker.md:36` ④ ⛔**판정(버전무관)** — ★**"전면 교체 후 AB 대조로 결과 동일"은 성립하지 않는다**
  (PRNG 공유 상태). 실무 경로는 **함수 단위 DIFF=0 누적**이다.
  ⟹ 14,476줄을 통째 교체·대조하려 들면 이 벽에 박는다.
- `reimpl-tracker.md:82` ⑥ **항상 100% 다른 잡음 필드(판단 금지 9종)** =
  `team_plan`·`battle_start_state`·`positioning_score`·`plan`·`sub_plan`·`debug`/`big_debug`/`small_debug`·`sthfn_stack`
  ⟹ 내 §3 스냅샷 diff 에서 **이 9종을 먼저 마스킹**해야 신호가 보인다.
- `DONE.md:80~82` **DONE** — `mf_swap` 코드 0~29 전량 복원(순수 텔레메트리) · 진단코드표 5종 · exit_src 이중 필드.
- `100퍼-잔여-트래커.md:478` **보류** — agent_link 트윈 비트동일 원인 미확정(`get_input` 99.974~99.978% 가 현 상한).

## 6-9. ★이 함수가 tracker 의 **⬜1순위**다
`ANA@퍼-잔여-트래커.md:474` (= `MEM	fm2-judge-layer.md:286`)
> ⬜(2026-09-09, 0.5.8·judge·**신규·다음 과제**) ★★judge 커버리지 6.0% → **플랜 핸들러 5종 포팅** —
> 미포팅 상위는 전부 플랜 핸들러 = **`0xe4c5c0`(handler 42KB)** · `0xe5d5d0`(engage 32KB) ·
> `0xdf36e0`(old/battle 29KB) · `0xdda220`(objective_discipline 24KB) · `0xd8eff0`(score_parameter 25KB)
> ⟹ 죽어 있는 movepri 노브(`dd_*`·`d4_*`·`ep_*`·`sn_*`·`bt_*`)를 되살리는 길도 이 재포팅.

★**착수 전 교훈 (68)** = 큰 함수는 IR 로 **분기별 도달 가능성부터** 가른다(사장 서브트리는 NA,
실측 NA=0 확인 → tower_dive 에서 **작업량 1/4**). 도구 = **`C:	fm2mods\MIG\irann.py`**.
⟹ 14,476줄·`br` 730 이므로 **이 함수에서 이득이 최대**다. 첫 작업으로 이걸 돌린다.

## 6-10. 모드 측 후킹/재구현 — **미착수 확인**
`tfm2_ai_adjust\src\judge\gen_fns.rs`·`probe_tbl.rs` 에 `0xe4c5c0` **0건**.
`MODS\MIGRATION.md` 에도 미등재 = 모드 상수로 배선된 적 없음.
handler 계열은 호출수 프로브만 등록돼 있고 `role:"plan_handler"` 포팅 목록에 handler.rs 함수가 없다.
⚠`gen_fns.rs` 의 `status` 문자열은 **썩는다**(교훈 69 — `as_d851d0` 가 "todo"인데 실제로는 DIFF=0 이었다).

# 7. 갱신된 착수 순서

1. **`irann.py` 로 분기 도달 가능성 선별**(교훈 68) — 사장 서브트리를 NA 로 봉인. 여기서 작업량이 갈린다.
2. **페이즈게이트 블록은 승계**(§6-3) — 다시 읽지 말고 0.5.8 좌표로 옮긴다.
3. **이미 읽힌 블록 스킵**: `mf_swap` 기록부(`handler.rs:953·998`) · `exit_src 22`(`:1758`) ·
   `entry_src 6`(`:614`) · `update_on_dead`(`:635`, 트윈 100%) · 자식 명세 **7개**.
4. `handle_interact_battle`(`0xe5d5d0`)를 **한 세트로** 편성(§6-5).
5. 오라클 진리표(§3) + 스냅샷 diff — 단 **잡음 필드 9종 마스킹**(§6-8).
6. 검증은 **함수 단위 DIFF=0 누적**(통째 AB 대조 금지).
7. 결론은 record-keeper 로 `DONE.md`+`INDEX.md §2`+`100퍼-잔여-트래커.md:474` 체크오프.
