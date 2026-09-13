# 17차 배치 D 보고 — #55 EntityPositioningCache::new · #56 PassiveLinePlan::sub_plan (게임 0.5.8)

기계 적용분 = `_verify17/D/patch.json` (정정 17 · ev상향 139 · 브리핑오류 3 · `applypatch.py 17 --only D --dry` = 17/17 · 139/139 · 실패 0).
오라클 = `_verify17/D/oracle/D17_o1.rs` (+ `D17_o1_case{0..10}.out`, `mkpatch_D.py`).

## 0. 지시 오류부터 (brief_errors — 3건 + 1)

1. **담당 범위 불일치**: 호출 메시지는 「배치 D = specs[53]~[56]」 인데 `DOSSIER_D.md` §1·§3 은 `55`~`56` 이고 `spec_53`/`spec_54` 는 `_verify17/C/` 에 있다. 지시문을 따라 #55·#56 만 다뤘다. 메시지의 G4 「#53·#54·#56」·G7 「#53 is_recent_visible」 은 배치 C 몫이다.
2. §4 G4 「실제 후보」 줄이 `bushes, drop  ⟵ IR 에 … 싣지 않는다, first_spawn_tick, from_mid, front_minion, map_` 로 잘려 12개 중 5개만 보이고 `drop` 의 설명문이 목록 원소처럼 끼어 있다(`mkdossier` 가 `callees_unmatched.names` 에 설명 문자열을 섞어 자름).
3. §1 표의 `ev≥4(미실행)` 가 두 함수 모두 **89** 인데 #55 는 mem70+consts14+knobs6=90, #56 은 52+29+12=93 이라 분모를 알 수 없다(집계 스코프 미기재).
4. (patch 밖) §1 의 `vis=pub` 은 #55 에 대해 **오해를 부른다**: 함수는 `pub` 이지만 반환 타입 `EntityPositioningCache` 가 `in:game_ai`(pub(crate), tcx `Struct in:game_ai … score_parameter.rs:79`)라 바깥에서 이름을 못 불러 **오라클 직접 진입이 막힌다**. 「vis」 열은 시그니처에 등장하는 타입의 가시성까지 봐야 한다.

## 1. 게이트 판정 (이 배치 몫 6건 — 전부 처리)

| 게이트 | 대상 | 판정 | 근거(원문 `.ll` 줄) |
|---|---|---|---|
| G12 | #55 consts[2] src_line=142 | **오탐** | m07.ll:5327 `%39 = add i64 %38, -1, !dbg !20585` → !20585 = `range`[effect.rs:26] < `new`[score_parameter.rs:142]. 소스 `level-1` 이 `add …, -1` 로 접혀 리터럴이 **-1** 인데 `srclinecheck.litpat(1)` 은 `, 1` 만 본다(`, -1` 의 `-` 가 경계를 깬다). 동형 14곳 전부 -1. ⟹ value 를 IR 리터럴 -1 로 보강(consts[9] 의 value=IR·folded_from=소스 관행) |
| G12 | #56 consts[28] src_line=969 | **실오류** → 982 | 969 의 유일한 리터럴은 m04.ll:27058 `icmp eq i8 %233, 2`(player.rs:988<969 = `line==Bottom`). visible_state==Visible 비교는 27304~27308 `gep %935,56 · gep {i64,[2 x i64]} …, %936 · load · icmp eq i64 %972, 0` 인데 **전부 `!40323 = !DILocation(line: 0, scope: sub_plan)`** — 두 인라인 사이트(27198 !40499 …<982 / 27281 !40561 …<985, `%936 = phi[%931, %962]`)의 공통식을 LLVM 이 블록 %969 로 sink 했다. 첫 사이트 982 로 정정 |
| G10 | #56 open[6] (L1003 non-Tower → LineDefense) | 사실 맞음 → **사실 서술**로 문면 정정 | m04.ll:27298~27301 `gep %811,104`(+0x68) · `icmp eq i64 %967, 2` · `br … %980(Tower: 27325 gep 136 nearest_enemy, ==0→%519 LineDefense / else %984 tag 5), %519`. tcxdict EntityType 2=Tower·3=Nexus |
| G10 | #56 open[8] (tutorial 1,2,3,4,6 은 게이트 없음) | 사실 맞음 → **사실 서술**로 문면 정정 + 헬퍼 귀속 | m04.ll:25105~25110 `switch i8 %61, label %72 [0,7,8,5 → %62]`, default %72 = L1021 블록. !39007 = `spawn_epic`[runner.rs:263] < `is_line_phase`[runner.rs:399] < 1020 |
| G18 | #56 logic `+0x60` 미등재 | **mem 행 삽입**(at=37, Blackboard 0x60 bottom_minion_state.from_mid, r) | m04.ll:25537 `gep <Blackboard> …, team` · 25538 `gep i8 …, 96` · 25539 load i64 · 25540 `icmp slt i64 %226, 1000`. tcxdict Blackboard 0x50 + BrainMinionParameter 0x10. **오라클 C5/C6 이 +0x60 = 2000/2001 로 Normal/Pull 을 갈라 실행 확증** |
| G4 | #56 callees_unmatched 12개 | **판정 술어 0개** — 필드 10 · std 2 | `bushes`(MapDef 0x1c98) · `first_spawn_tick`(GameSetting 0x8a8, GameSetting::is_line_phase 안에서 읽힘) · `from_mid`/`front_minion`/`minion_count`/`minion_power`(BrainMinionParameter) · `objective`(TeamPlan 0x41f) · `v46_flee`/`v46_flee_acute`/`v46_flee_cover`(PassiveLinePlan 0x112/0x113/0x115) · `drop`(core Drop, bumpalo Vec) · `map_or`(Option). 전부 tcxdict 확인 |
| G15 | #56 consts[23] kind=미상 | 보강 | 150000 은 콜리 인자(m04.ll:26253)라 본문 관측이 CALLARG 뿐 → meaning 에 「반경 임계」 낱말 |

★**G4 의 근본 원인은 명세가 아니라 `mkspec3.harvest_callees`** 다: logic 이 필드를 `name(+0xNN)` 로 적는 관습(오프셋 주석)이 `CALLNAME = \b(name)\s*\(` 에 걸려 **필드가 호출로 긁힌다**. #55 의 `position`(PlayerAiContext::position / TeamRosterPlanningContext::position 후보 2개)·#56 의 `position`·`line`·`tick`·`in_recall`(→ Blackboard::in_recall 오후보) 이 전부 이것이다. **제안(다음 게이트)**: `harvest_callees` 에서 `name\s*\(\s*[+-]0x` 와 `name\s*\(\s*\+` 를 제외하고, `fn <name>(` 헤더(자기 이름 `sub_plan` 이 후보 16개로 실려 있다)도 제외하라. 효과: #56 미매칭 12 → 2(`drop`·`map_or`, 둘 다 std).

## 2. 무검사 축에서 나온 것

### 2-a. #55 `open[5]` 「표기 불가」 → **판정 반전(확정)** — `!dbg` 의 scope 함수명을 안 본 것
m07.ll:5741 `%300 = mul i64 %74, %74, !dbg !20779`(= `compute_range_sq`[score_parameter.rs:**129**] < new:183) · 5744 `%301 = add i64 %73, 50000, !dbg !20782`(= compute_range_sq:**130** < 183). 즉 L183 은 `compute_range_sq(attack_range)` 호출이고 50000 은 그 안의 `r + 32000`(:130) 에 +18000(L142, `%74 = %73+18000`) 이 LLVM 재결합으로 접힌 것. 소스 표기 = `(attack_range+32000)^2` **확정**. L183 에는 :131(`lshr 1`) 명령이 없어 반환 튜플 3번째(half)가 버려진 것으로 추정(struct 에 attack_range_half_sq 없음 — mem[54]/[55] 와 정합).

### 2-b. #55 logic 헬퍼명 `max_range` = 존재하지 않는 이름 → callees 오염
a55 전 범위의 인라인 출처 집계: `effect.rs:26` 130회, `battle.rs` **0회**. !20585/!20705/!20716/!20727 의 scope 이름 = `range`[game-core\src\simulation\effect.rs]. tcx: `AssocFn pub game_core::Effect::range effect.rs:25 mir=True xinl=True`. `Effect::max_range` 는 tcx 에 없고, 자동수집이 `game_ai::max_range(fn(&Entity,&Entity)->u64, battle.rs:2234)` 를 callees[3] 로 실었다(G9 역방향). logic 을 `eff_range(...)` = `game_core::Effect::range(&eff, caster)` 로 정정.

### 2-c. #56 L1020 = passive_line 의 식이 아니라 **game_core 공용 헬퍼 3개의 인라인** (노브 귀속 오류)
```
passive_line.rs:1020  if !context.is_line_phase(tick) { return None }
  GameContext::is_line_phase(&self, tick)      runner.rs:397~399  pub mir=1 xinl=1   = !tutorial.spawn_epic() || setting.is_line_phase(tick)
    TutorialType::spawn_epic()                 runner.rs:262~263  pub mir=1 xinl=1   switch 태그 ∈{0 None,5 MidBottom,7 Line,8 Total} → 시간 게이트 / 그 외 → 통과
    GameSetting::is_line_phase(&self, tick)    setting.rs:702~704 pub mir=1 xinl=1   tick < epic_jungle.first_spawn_tick(+0x8a8).saturating_sub(tick_per_second(+0x12f8)*30)
```
근거: m04.ll:25105 switch !39007 사슬 · 25116 `gep %64, 2216` !39009(setting.rs:703) · 25121 `mul i64 %68, 30` !39016(setting.rs:**704**) · 25123 `usub.sat` · 25124 `icmp ult i64 %59, %70`. 셋 다 `define` 이 없다(항상 인라인). ⟹ **knobs[0] 의 30 은 이 함수 고유 노브가 아니라 공용 헬퍼의 계수** — 여기만 패치하면 #51 check_press_tower_opportunity 등 다른 호출처와 갈린다(knobs[0].where·consts[7]·mem[10] 에 귀속 보강).

⚠**명세 사이 잠재 충돌(G20 후보)**: `spawn_epic` 의 집합이 두 가지로 관측된다.
- `morgard_exists` 인라인 사이트(m13.ll:53429 `add i8 %40, -7; icmp ult i8 %46, -6` · 53607 `add nsw i8 %91,-1; icmp ult i8 %92, 6`) = **{0,7,8}** — #04·#11·#12·#50 이 이렇게 적었다(맞다).
- `is_line_phase` 인라인 사이트(m04.ll:25105 #56 · m13.ll:7888 handler.rs:2113 · #51) = **{0,5,7,8}** — 같은 `!dbg` 가 `spawn_epic:263` 을 가리키는데 5(MidBottom)가 있다. `spawn_serpen`(runner.rs:267, m13.ll:53372) 이 정확히 {0,5,7,8} 이므로 `is_line_phase` 는 `spawn_epic() || spawn_serpen()`(또는 `|| ==MidBottom`) 일 가능성이 크고 LLVM 이 한 switch 로 합쳤다 — **표기 불가**, 집합은 3사이트 일치로 확정. 이 사실을 `sharedchk` 에 넣지 않으면 다음 라운드가 「#04 vs #56 spawn_epic 불일치」로 잡을 것이다.

### 2-d. mem 오프셋 전수 대조 (tcxdict, 122행) — **불일치 0**
`scratchpad/v17D/memchk.py 55 56` — 각 행을 `tcxdict.py <base> <offset>` 로 조회해 이름 대조. OK 112 · SKIP 10(vtable 슬롯 5 · SubPlan sret 4 · Effect 0x8 ty.vtable = Arc 팻포인터 뒤 절반이라 사전이 `ty.ptr.pointer@0x0` 에서 멈춤 — 오류 아님). 7차 배치A 의 `mem.dir` 0건과 같은 반대 사례. OK 112행에 `tcx 정본 대조` ev_up(→3).

## 3. 오라클 (#56 · `D17_o1.rs` · 케이스당 프로세스 1개 · 11/11 예측 일치)

```bash
cd /c/tfm2mods/MIG && sh _verify3/build.sh _verify17/D/oracle/D17_o1.rs      # 1m20s
for c in 0 1 2 3 4 5 6 7 8 9 10; do %TEMP%/tfm2_spanprobe/D17_o1.exe $c > _verify17/D/oracle/D17_o1_case$c.out; done
```
`setting_ok=true · towers 16 · twin 2/2 · size_of PassiveLinePlan=280 TeamPlan=1064 Blackboard=744 SubPlan=72`. 조립 = TEMPLATE 그대로(real_setting · init_tower 안 부름) + `PassiveLinePlan::new(LineType::Bottom)` + `TeamPlan::default()` + `[Blackboard;2]::default()` 에 raw write.

| C | 조립 | 예측(명세) | 실행 | 확정되는 것 |
|---|---|---|---|---|
| 0 | in_recall(+0x110)=1 | tag 5 | **5 Recall** | consts[0] · mem[0] |
| 1 | v46_flee=1·cover=0·acute=1 | tag 4, +8=2 | **4 LineWait{Bottom}** | consts[2] · mem[1][3] |
| 2 | v46_flee=1·cover=0·acute=0 | tag 3, +8=2 | **3 LineSafe{Bottom}** | consts[1] |
| 3 | v46_flee=1·**cover=1** | 게이트 불통과 → tag 2 {0,2,2} | **2 {Aggressive,Bottom,Push}** | mem[2] 극성 |
| 4 | 기본 | 2 {0,2,2} | **동일** | consts[3]·[22](적 없음→Push) · sret 레이아웃 |
| 5 | objective=8 Gank·line=2·from_mid(+0x60)=2000 | action Normal(1) | **{0,2,1}** | consts[5]·[20] · mem[5][6] · **+0x60 행(G18)** |
| 6 | 같음·from_mid=2001 | Pull(0) | **{0,2,0}** | 경계 `< 2001` · knobs[5] |
| 7 | Gank·line=0(Top)≠self.line·2001 | 비갱크 → Push(2) | **{0,2,2}** | L884 line 일치 조건 |
| 8 | **Dive(9)**·line=2·2001 | is_gank_target_line 은 ==8 만 → Push | **{0,2,2}** | consts[5] 「정확히 8」 |
| 9 | front_minion=Some(champ_id) | front_minion 경로 → LineDefense | **{0,2,2}** | mem[33] |
| 10 | position=Jungle 플레이어 | style Defensive(1) | **{1,2,2}** | consts[19] · mem[26] |

`{:?}` 가 `LineDefenseSubPlan { line, minion_action_type, style }` 필드명까지 찍어 +0x8 style / +0x9 line / +0xa action 을 이름으로 확인했다(수법 ⓑ).

**미실행 가지(다음 라운드 재료 — 전부 「미탐색」, 조립법 있음)**: 2v1 경로(L1019~1041: `GameSetting.epic_jungle.first_spawn_tick` 을 크게 잡아 is_line_phase 를 열고 적 챔프 2명을 `ptr::write` 로 바텀 라인 좌표로 옮긴 뒤 `bb[1].last_visible` 갱신) · has_near_enemy_champion(L919) · minion_power<0 가지(L922) · nearest_tower/열세 분기(L945~1013: `can_near_enemies_range` 가 적을 세야 함) · is_object_far_line/giveup_object(L968~985: Moba 모드 live_list 조작).

## 4. 판정 어휘 정리
- **오탐** 1(G12 #55 consts[2]) — 게이트 `litpat` 이 음수 리터럴을 못 본다. **게이트 개선**: `litpat(val)` 에 `-val` 도 후보로(`level-1` 류는 항상 `add -k` 로 접힌다), 최소한 `(?:\b<ty>\s+<attr>|,\s*)-?<val>`.
- **실오류** 6(#56 consts[28] · #55 logic 헬퍼명 ×2 · #55 open[5] 반전 · #56 logic L1020 귀속 · #56 open[6]/[8] class) — 그중 **판정 반전 1**(표기 불가 → 확정).
- **보강** 9 · **삽입** 1.
- **표기 불가**(범위 명시): `is_line_phase` 안에서 MidBottom(5) 을 더하는 항이 `spawn_serpen()` 인지 `==MidBottom` 인지 — IR 의 한 switch 로 합쳐져 IR·오라클(외연 동일)로는 안 갈린다. 동작(집합 {0,5,7,8})은 확정. **미탐색**: xinl=1 이라 MIR 이 있다 — `_tcx` MIR 덤프로 갈릴 수 있다.
- **미탐색**: #55 오라클 — 타입이 pub(crate) 라 직접 진입 불가. 열려 있는 길 = 호출처 pub 래퍼(m00.ll:77461 · m07.ll:34704) 경유 또는 424B 버퍼로 재선언한 타입에 **함수 심볼을 `extern "Rust"` 로 빌려** 호출(시도 안 했음).
- **G12 도구 개선 제안 2**: 리터럴 명령이 `!DILocation(line: 0)` 이면(#56 consts[28]) 「불일치」가 아니라 **판정 불가**로 분류하고, 그 명령이 쓰는 `phi` 인입 블록의 종결자 줄을 약한 후보로 붙여라(§S5-b 「귀속은 구제만」).

## 5. 실행한 것(재현용)
```
python -X utf8 dossierfresh.py 17 D                          # FRESH (착수·제출 시 2회)
python -X utf8 specgate.py --only 55 / --only 56             # 게이트 현황
python scratchpad/v17D/ann.py _gaibc/m07.ll 5210 7048 a55.ll   # 원문 줄번호 유지 + inlinedAt 루트 주석(irann 은 줄이 밀려 안 씀)
python scratchpad/v17D/ann.py _gaibc/m04.ll 24962 27342 a56.ll
python -X utf8 dbgchain.py _gaibc/m04.ll 40323 40322 39007 39009 39016 · m07.ll 20782 20232 20705 20585 20875 · m13.ll 19179 56835
python -X utf8 tcxdict.py --enum EntityType|TutorialType|MainObjective|SubPlan|LineStyle|MinionActionType|TeamType · Blackboard · BrainMinionParameter · PassiveLinePlan · TeamPlan · MapDef · GameContext · GameSetting 0x8a8/0x12f8 · Entity 0x438/0x470/0x670/0x68 · LineDefenseSubPlan
python scratchpad/v17D/tq.py game_core near setting.rs 695 706 / runner.rs 255 265 / runner.rs 390 402 / entity.rs 1478 1490
python scratchpad/v17D/memchk.py 55 56                       # mem 122행 tcxdict 대조
python -X utf8 _verify17/D/oracle/mkpatch_D.py && python -X utf8 applypatch.py 17 --only D --dry
```
