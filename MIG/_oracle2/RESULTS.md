# 잔여 6건 배치 결과 (0.5.8 / SDK sdk_058 / 2026-09-11)

산출 도구: `_oracle2\irfn2.py`(IR 함수 본문 1패스 추출), `_oracle2\sig.py`(tcx 시그니처 lifetime 정리).
1차 재료: `_tcx\mirdump_game_ai.txt`(MIR), `_tcx\game_ai.json`·`game_core.json`(tcx), `tcxdict.py`, `_gaibc\*.ll`, `_docs\game_ai.txt`.

---

## 1. `PassiveJunglePlan::is_counter_jungle` — 극성 확정 (MIR)

```
signature  fn(&PassiveJunglePlan) -> bool      v=pub  mir=True  xinl=True
           game-ai\src\plan_legacy\old\passive_jungle.rs:103
logic      self.team != self.player_team          // passive_jungle.rs:104
```
MIR 원문 (`_tcx\mirdump_game_ai.txt:12633`):
```
_2 = copy ((*_1).1: usize)   @passive_jungle.rs:104:5-104:14
_3 = copy ((*_1).2: usize)   @passive_jungle.rs:104:18-104:34
_0 = Ne(move _2, move _3)
```
필드 **선언 순서**(tcx adt) = `0 jungle:JungleType / 1 team:usize / 2 player_team:usize /
3 chats:Vec<Chat> / 4 last_lead_action_tick:usize / 5 counter_jungle_route:Option<CounterJungleRoute>`
⟹ `(*_1).1 = team`, `(*_1).2 = player_team`.

**레이아웃 오프셋**(tcxdict, 선언순서와 다름):
`0x0 chats · 0x18 counter_jungle_route · 0x48 team · 0x50 player_team · 0x58 last_lead_action_tick · 0x60 jungle` (104B)

### 극성 — `is_passive() == !is_counter_jungle()` (확정)
`BigPlan::is_passive` (types.rs:253, mir=True) MIR:
```
switchInt(discriminant) -> [1: true(255), 2: true(256), 5: bb2(257), otherwise: false(258)]
bb2: _4 = Ne(plan.1, plan.2) ; _0 = Not(_4)     @types.rs:257:39-257:64
```
= `PassiveLine⇒true / SinglePlanLine⇒true / PassiveJungle(p)⇒!p.is_counter_jungle() / _⇒false`

★**직전 배치의 `plan+0x50 == plan+0x58` 관측은 오귀속**이었다. `m13.ll:30497~30502` 의 `%0`
는 플랜이 아니라 **`LegacyPlanHandler`** 이고, 비교 대상은 `handler+0x638 / +0x640`
= `plan@PassiveJungle.0.team` / `plan@PassiveJungle.0.player_team`
(BigPlan 페이로드 +0x8 → 0x638−0x8=0x630, PassiveJunglePlan+0x48=team → 0x638 ✔ / +0x50=player_team → 0x640 ✔).
dloc 체인 = `104 in is_counter_jungle → 257 in is_passive → 246 in handle_chat_inner`.
⟹ 그 사이트는 `team == player_team`(= `!is_counter_jungle()` = `is_passive()`) 일 때 통과. **모순 없음.**

덤: `AgentVerHamster::plan_is_counter_jungle` (lib.rs:350, **pub**) =
`matches!(self.<f14>.<f1>, BigPlan::PassiveJungle(p) if p.team != p.player_team)`
(`AgentVerHamster` 필드 idx14 = `LegacyPlanHandler`, 그 idx1 = `BigPlan`).

reads: PassiveJunglePlan+0x48 team(usize) · +0x50 player_team(usize)
writes: 없음 / constants: 없음 / calls: 없음 / knobs: 없음
unknown: **없음**

---

## 2. `BigPlan::goal()` → `BigGoal` 전 variant 산출표 (완전 규정)

`BigPlan` (types.rs:17, 384B, 니치: untagged=4(DeathMatchBattle), niche_variants 0..=15, niche_start=2
⟹ **메모리태그 = idx+2**, idx4 만 태그 없음)
디스패처 IR = `m10... 아니라 m02.ll:6427` `_RNvMNtNtCs..._7game_ai11plan_legacy5typesNtB2_7BigPlan4goal`
(태그 복원: `%5=%3−2; %7 = (%3>1) ? %5 : 4`, `assume(%3 != 6)`)

| idx | variant | BigGoal 산출 | 근거(IR 블록 / MIR) |
|---|---|---|---|
| 0 | `ForcePassive` | **`Recall`** (tag 6) | m02.ll:6460 `store i8 6` |
| 1 | `PassiveLine(p)` | `p.in_recall ? Recall : Line{line: p.line}` | m02 %11/%56/%60 (+280=in_recall, +286=line) / MIR PassiveLinePlan::goal passive_line.rs:211 |
| 2 | `SinglePlanLine(p)` | `p.in_recall ? Recall : Line{line: p.line}` | m02 %15/%61/%65 (+32/+33) / MIR single_line.rs:19 |
| 3 | `SinglePlanBattle(p)` | `SinglePlanBattle::goal(p)` ↓ | m02:6483 tail call |
| 4 | `DeathMatchBattle(p)` | **`Battle{focus: p.sub_goal.focus()}`** (Recall 분기 **없음**) | m02 %21/%66~%78 / MIR death_battle.rs:861 |
| 5 | `PassiveJungle(p)` | `Jungle{camp: p.jungle, team: p.team}` | m02 %25 (+104=jungle, +80=team) / MIR passive_jungle.rs:130 |
| 6 | `ActiveRecall(_)` | `Recall` | m02 %32 / MIR active_recall.rs:13 |
| 7 | `Battle(p)` | `BattlePlan::goal(p)` ↓ | m02:6527 tail call |
| 8 | `LineGanker(p)` | `Line{line: p.line}` | m02 %35 (+48) / MIR ganker.rs:35 |
| 9 | `LineGankCover(p)` | `Line{line: p.line}` | m02 %39 (+40) / MIR cover.rs:21 |
| 10 | `EpicHuntAndPoke(_)` | `Epic` (tag 2) | m02 %43 / MIR hunt_and_poke.rs:22 |
| 11 | `EpicHuntAndBattle(_)` | `Epic` | m02 %44 / MIR epic\hunt_and_battle.rs:15 |
| 12 | `SerpenHuntAndPoke(_)` | `Serpen` (tag 3) | m02 %45 / MIR serpen\hunt_and_poke.rs:22 |
| 13 | `SerpenHuntAndBattle(_)` | `Serpen` | m02 %46 / MIR serpen\hunt_and_battle.rs:15 |
| 14 | `AttackNexus(p)` | `Nexus{team: p.team}` (tag 4) | m02 %47 (+8) / MIR attack_nexus.rs:19 |
| 15 | `DefenseNexus(p)` | `Nexus{team: p.team}` | m02 %51 (+8) / MIR defense_nexus.rs:19 |

### `BattlePlan::goal` (battle.rs:341) = `SinglePlanBattle::goal` (single_battle.rs:77) — **본문 동일**
```
fn goal(&self) -> BigGoal {                                   // 341 / 77
    if self.sub_goal is End(7) or RunAway(4) { BigGoal::Recall }        // 342 / 78
    else { BigGoal::Battle { focus: self.sub_goal.focus() } }           // 345 / 81
}
```
IR: `m10.ll:23386`(BattlePlan) / `m05.ll:27000`(SinglePlanBattle) — 둘 다
`load self+88(sub_goal tag); switch [7→%10, 4→%10]; default: out+8=1, out+16=self+96; phi tag [5 from default, 6 from switch]`.
`BattlePlan`/`SinglePlanBattle` 둘 다 `sub_goal @0x58`(tag 88, payload 96). `BigGoal` tag 5=Battle, 6=Recall.
`focus()` 가 4/7 에서만 None 이라 default 팔의 `Some` 이 상수접힘된 것.

### `BattleSubPlanGoal::focus()` (battle.rs:29, MIR 12681)
tag `0 Trace / 1 Protect / 2 Kiting / 3 KitingBack / 5 Assassin / 6 AssassinReady` → `Some(.0 usize)`
tag `4 RunAway / 7 End` → `None`   (`BattleSubPlanGoal` 16B, Direct 태그, 페이로드 `focus:usize @+0x8`)
자매: `is_end` = tag==7 · `is_full_runaway` = tag==4 · `in_active_fight` = tag ∈ {0,1,2,5,6} · `engage_dir` = 0→+1 / 3,4→−1 / else 0

**★비대칭 주의**: `DeathMatchBattle::goal` 은 Recall 분기가 **없다** — `focus()` 가 None 이어도
`Battle{focus: None}` 을 낸다. `BattlePlan`/`SinglePlanBattle` 만 `Recall` 로 빠진다.

reads/writes/constants: 위 표 · calls: `BattleSubPlanGoal::focus` · knobs: 없음(순수 사상)
unknown: **없음**

---

## 3. ★`resolve_fight_uncached` 의 `%331` — **`our_n == 0`** (tower 무관, 증명)

```
signature  fn(tick: usize, op: &OperationData, me: &Entity, our: &[&Entity], their: &[&Entity],
              dir: i8, tower: Option<&Entity>, _: usize, arrivals: &[i64], baseline: i64)
              -> FightPrediction
           v=in:...fight_model  mir=False   fight_model.rs:378
IR         m10.ll:43974 (internal fastcc, ArgumentPromotion 로 OperationData 가 2 ptr 로 쪼개짐)
IR 인자    %1=tick %2,%3=op %4=me %5/%6=our(ptr,len) %7/%8=their(ptr,len)
           %9=dir %10=tower(null=None) %11=usize %12/%13=arrivals %14=baseline
```

### 판정: `%331 = (%154 == 0)`, `%154 = our_n`
```
m10:45082  %331 = phi i1 [ false, %391 ], [ %301, %300 ]
m10:44995  %301 = phi i1 [true,%256],[true,%257],[false,%292],[false,%286],[false,%280],[false,%274],[false,%268]
```
**%300 으로 들어오는 true 간선 2개가 전부 `%154==0` 으로 가드된다:**
- `%256` preds = `{%238, %231}`
  - `%231` ← `%315` 에서 `%316 = icmp eq i64 %154, 0` 가 **참**일 때만 (m10:45043)
  - `%238` preds = `{%233, %234}`; `%233` ← `%231`(⟹%154==0), `%234`→`%238` 은 `%237 = icmp eq %154, 0` 참일 때만 (m10:44618)
  ⟹ **%256 ⟹ %154==0**
- `%257` → `%300` 은 `%262 = icmp eq i64 %154, 0` 참일 때만 (m10:44800)

`%230`/`%232`(`icmp eq ptr %10, null` = tower==None, m10:44548/44559)는 **%256/%257 에 들어가는
간선을 고르는 조건일 뿐**, `%300` 으로 나가는 간선의 조건이 아니다.
⟹ ★**직전 배치의 (a) "tower==None ⇒ 즉시 net=−baseline" 은 오독.** 정답은 "our_n==0".

`%154 = phi [%144,%142],[%971,%967]`, `%971 = %144+1`, `%145 = 5 카운트다운(take(5))`,
`%142` 는 `Copied<Iter<&Entity>>::next()` 루프 ⟹ **%154 = 실제로 취한 아군 수 (≤5)**.
`%537` 이 곧바로 `%539 = icmp ugt i64 %154, 5` 로 가드하는 것도 이 해석과 정합.

### %331 의 3개 사용처 = 전부 "아군 0명 축약 경로"
| IR 줄 | 분기 | true(our_n==0) 쪽 결과 | 소스줄(dloc) |
|---|---|---|---|
| 45208 | `%397` → `%439` / `%398` | 틱별 기여 합산 스킵 | fight_model.rs:464 (`.sum()`) |
| 45702 | `%478` → `%600` / `%484` | `%601 = 0 − %14` = **net = −baseline** | fight_model.rs:468 (`min_by_key`) |
| 45960 | `%534` → `%823` / `%537` | hyst 계산 스킵(=0) | fight_model.rs:510 (`.sum()`) |

**진짜 이유는 0-나눗셈 가드다**: `m10:47056 %822 = sdiv i64 %548, %154` (fight_model.rs:510)
= `hyst = <아군별 합> / our_n`. `our_n==0` 이면 UB/패닉이라 축약한다.
⟹ (b) "타워 없으면 히스테리시스 없음" 도 **오귀속**. 히스테리시스 억제는 `dir`(=%9) 소관이고
`%827`(hyst 크기)이 0 이 되는 건 our_n==0 뿐이다.

### 세르펜 저울 모순 — 해소
`_docs\game_ai.txt:322~324` : *"[v3 EPIC-OPS B-④] 세르펜 경합 교전 저울 … 가정 판이므로
히스테리시스 없음(dir=0)·타워 없음 — resolve_fight 단일 모델 재사용"*
실제 호출부 확인: **`game_ai::plan_legacy::old::serpen::v3_serpen_contest_clear_win`**
(`m05.ll:53367`) 가 `resolve_fight(..., dir = i8 0, tower = ptr null, ...)` 로 호출한다.
another: `game_ai::tower_discipline::v47_siege_stance` (`m07.ll:48811`) = `dir=0, tower=nonnull`.
⟹ tower=None 은 정상 운용 경로이며, %331 과 무관하다. **주석·IR·블록그래프 3자 정합.**

### 덤: 반환 조립(`%838`, m10:47088~47105) — FightPrediction 전 필드 확정
```
out+0x00 focus_target.disc = %826   out+0x08 focus_target.val = %825
out+0x10 soaker.disc       = %333   out+0x18 soaker.val       = %335
out+0x20 rescue_ally.disc  = 0(None, 이 경로에선 항상)
out+0x30 net_value = %824  = their_loss − (our_loss + baseline)   [fight_model.rs:508]
out+0x38 line = %839       out+0x39 line_absolute = %839 (동일)
```
`line` 결정 (`switch i8 %9` = **dir**, `%824`=net, `%827`=hyst):
| dir | 판정 |
|---|---|
| `0` (기본) | `net > hyst` → Commit(0) / `net < −hyst` → Disengage(2) / else Hold(3) |
| `+1` | `net < −hyst` → Disengage(2) / `net >= 0` → Commit(0) / else Hold(3) |
| `−1` | `net > hyst` → Commit(0) / `net <= 0` → Disengage(2) / else Hold(3) |
`FightLine` = Commit0 / CommitAfterJoin1 / Disengage2 / Hold3 (1B Direct)

### (가) 오라클 경로 판정 — **이 방식으로는 (a)/(b) 판별 불가**
구성 가능성 자체는 열려 있다(모두 pub 확인):
`Game::new(u64,bool,&GameSetting,&MapSetting,&MapDef)` · `AbstractGameWithCache::new(&dyn AbstractGame,&GameContext)`
· `Blackboard::default` · `OperationData::new` · `Entity::default` · `TeamPlan::default` · `DebugFrameData::default`.
**막힌 곳**은 두 가지:
1. `&PlayerState` — pub 생성자는 `PlayerState::from_player(GamePlayer)` 뿐이고
   `GamePlayer::new` 이 `Arc<dyn ChampionInfo>` 를 요구한다(`ChampionInfoSheet` 필요). **미탐색**(불가 아님).
2. ★**원리적 한계** — `resolve_fight` 는 `in:game_ai` 라 직접 호출 불가이고, 유일한 pub 간접
   호출자(`tower_dive_is_viable` / `single_tower_dive_is_viable` / `check_kill_die_tick`)는
   `bool`/`usize` 하나만 돌려주며 **`our`/`tower` 를 내부에서 스스로 만든다**.
   즉 외부에서 `our_n==0` 과 `tower==None` 을 **독립 변주할 수 없다** ⟹ 이 두 가설은
   **오라클로 원리적으로 안 갈린다(= 표기 불가)**. IR 블록그래프로만 갈리며, 위에서 갈렸다.

reads: 위 · writes: `FightPrediction` 6필드 · calls: `fight_check::expected_dps` 등
knobs: `dir`(호출부 지정) · `baseline` · `hyst` 산식(fight_model.rs:510)
unknown: **재료 부재** — `%548`(hyst 분자)의 항별 의미, `%766/%767`(our/their 손실 누적)의
정확한 단위는 이번 배치 범위 밖(미탐색).

---

## 4. `resolve_fight_stake_roster` 반환 `+0x38` — **`FightPrediction.line : FightLine`** (확정)

```
signature  fn(tick: usize, op: &OperationData, me: &Entity,
              roster: &[(&Entity, i64, bool)], their: &[&Entity],
              dir: i8, tower: Option<&Entity>, _: usize) -> FightPrediction
           v=in:game_ai  mir=False   fight_model.rs:645
```
`FightPrediction` (fight_model.rs:254, **64B**, pub):
`0x00 focus_target:Option<usize> · 0x10 soaker:Option<usize> · 0x20 rescue_ally:Option<usize>
 · 0x30 net_value:i64 · 0x38 line:FightLine · 0x39 line_absolute:FightLine`
⟹ **추정이던 "FightLine" 이 사실로 승격.** `battle.rs:1058` 의 `exit_src=13` 판정
`+0x38 != 0` = `line != FightLine::Commit` = `line ∈ {CommitAfterJoin, Disengage, Hold}`.
값역 {0,1,2,3} 도 `FightLine` variant 4개와 일치.

unknown: 없음

---

## 5. `Chat` 경로의 `team_tag == 0` — **`TeamType::Player`** (확정)

```
game_core::TeamType (entity.rs:1128, 16B, pub, Direct 태그 @+0x0, 8B)
  idx0 = 선언discr 0 = 메모리태그 0 = Player(usize)   페이로드 @enum+0x8
  idx1 = 선언discr 1 = 메모리태그 1 = Neutral        (페이로드 없음)
```
`Entity.team : TeamType` = **`Entity+0x0`** (16B). ⟹
`%149.team_tag == 0 && %149.team == my_team` ≡ `matches!(e.team, TeamType::Player(t) if t == my_team)`
= "중립(정글/오브젝트)이 아니고 내 팀 인덱스와 같다".
논리인덱스 = 선언discr = 메모리태그로 **셋이 전부 일치**(니치 밀림 없음).
덤: `Entity+0x5c0 = id` (IR 의 `+1472`=0x5c0 는 `id`).

unknown: 없음

---

## 6. `stake` / `stake_commit` / `stake_veto` — DWARF 대조 완료

DWARF (`m13.ll:105128~105132`, 파일 `!14732 = chat.rs`):
```
!36571 name:"stake"        line 212  type: enum2$<Option<FightPrediction>>  (512bit)
!36573 name:"stake_commit" line 215  type: bool
!36575 name:"stake_veto"   line 216  type: bool
```
동형 사본이 `handler\engage.rs:423/427/428` 에도 있다(`!39616/!39618/!39620`,
소유 함수 `LegacyPlanHandler::handle_interact_battle`, engage.rs:233).

### 정의식 (`LegacyPlanHandler::handle_chat_inner`, chat.rs:41, IR `m13.ll:30336~30380`)
```rust
let stake = if tick > 1 {                                  // 212   (%350 = icmp ugt i64 %1, 1)
        resolve_join_stake(tick, rng, op, ps, e1, e2, team_plan, dbg)   // -> Option<FightPrediction>
    } else { None };
let stake_commit = stake.is_some_and(|p| p.line == FightLine::Commit);     // 215
let stake_veto   = stake.is_some_and(|p| p.line == FightLine::Disengage);  // 216
```
근거: `m13.ll:30345` `call resolve_join_stake(sret %38, %1, …)`;
`%354 = load i64 %38; %355 = icmp eq %354, -1` (None 니치);
`%358 = load i8 %38+56` (= `+0x38` = `line`);
`%359 = icmp eq i8 %358, 0` → `stake_commit`;
`%360 = icmp ne i8 %358, 2` + `#dbg_value(!36575, DW_OP_not)` → `stake_veto = !(line != 2) = (line == Disengage)`.
dloc: `!37196 → option.rs:742 as_ref → chat.rs:215`, `!37239/!37266 → fight_model.rs:243 eq → chat.rs:215/216`.

### 소비 지점 (chat.rs:217~219, IR `m13.ll:30381~30420`)
기저 게이트(스테이크 없이 계산되는 판) = `can_help && !too_far && !(hp_pct < thresh)`
(계측 카운터: `DebugFrameData+5584` = !can_help / `+5592` = too_far / `+5600` = hp_pct<thresh)
- `%371 too_far` = `dist_ticks(%314) > tps(%369) * (4 + roaming_ratio*2/1000)`
  · `%314 = Entity::distance(me, target) / target.stat_cached.move_speed(+0x640=1600)`
- `%372 can_help` = `BigPlan::can_help(plan, …)`
- `%323 hp_pct` = `target.hp(+0x670=1648) * 100 / target.stat_cached.hp(+0x628=1576)`  ← 분모는 최대HP
- `%328 thresh` = `60 − aggressive_ratio*20/1000`
- 0-나눗셈 가드 2개(`m13:30286 %318`, `30312 %334`)가 `move_speed==0`/`max_hp==0` 을 막는다

```
if too_far        { emit iff (stake_commit && can_help)   -> code 1 }
else if can_help  {
     if hp_pct < thresh { emit iff stake_commit           -> code 1 }   // 기저는 기각
     else               { emit iff !stake_veto            -> code 0 }   // 기저는 채택
} else { no emit }
```
채택 시 (`%383`~): `chats.push(Chat<tag 15>(0))` 후 `BattlePlan::new(...)` 를 만들고
```
plan.entry_src (+0x107) = 4
plan.ff_stake_join (+0xfc) = if stake_commit { code(%384) } else { 0 }
```
⟹ ★**`ff_stake_join` = "Commit 스테이크가 기존 게이트 기각을 뒤집어 채택된 판"** —
개발자 주석 `_docs\game_ai.txt:574` (`[ff 계측] 저울 합류 — F5 join stake의 Commit이 기존
게이트 기각을 뒤집어 채택된 플랜. 관측 전용.`) 과 **정확히 일치**. 추정 → 사실.
`stake_veto`(Disengage)는 반대로 **기저가 채택하던 판을 억제**한다(행동 영향 있음, 관측 전용 아님).

reads: `FightPrediction+0x38 line` · `Entity+0x670 hp` · `AthleteParameter::aggressive_ratio/roaming_ratio`
       · `GameSetting+0x12f8 tick_per_second` · `LegacyPlanHandler+0x7c8 chats`
writes: `BattlePlan+0xfc ff_stake_join` · `+0x107 entry_src=4` · `LegacyPlanHandler.chats`
        · `DebugFrameData+5584/5592/5600` (계측 카운터)
constants: `4`,`2/1000`(roaming) · `60`,`20/1000`(aggressive) · `6`(m13:30411 `%369*6` = 다른 사이트)
           · Chat 태그 `15` · `entry_src = 4`
calls: `resolve_join_stake`(fight_model.rs:680) · `BigPlan::can_help` · `BattlePlan::new` · `Entity::distance`
knobs: `aggressive_ratio`·`roaming_ratio` 두 선수 파라미터가 임계를 직접 민다
unknown: **미탐색** — engage.rs:423/427/428 사본의 소비 지점(같은 패턴 추정, IR 미독해);
         `Chat` 태그 15 의 이름; `entry_src` 코드표(다른 에이전트 담당).
