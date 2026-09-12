# 4차 반증검증 — 배치 B (05~09) 보고

게임 0.5.8 / SDK `sdk_058` / toolchain `nightly-2026-05-24` / 2026-09-11
작업 파일 = `C:\tfm2mods\MIG\_verify4\B\`
(`B4_o09m.rs` · `B4_o0708.rs` · `B4_o09f.rs` · `_patch1.py` · `_patch2.py` · `_xcheck.py` · 이 문서)

---

## 0. 한 줄 결론

**실오류 3건**(전부 09 · 전부 「3차 정정이 한 필드만 반영」 계열) ·
**새 발견 7건**(오라클 2건은 3차의 「미도달」·「재료 부재」를 뚫었다) ·
**과열림 8건 + class 재분류 2건**(3차가 이미 닫은 것이 v3 `open[]` 에 그대로 남아 있다) ·
`ev<=3`/`ev2` 표본 재확인 **뒤집힘 0** (07 24/24 · 08 12/12 · 09 1800/1800 · IR 표본 6/6).

★이 라운드의 구조적 소득 = **`specgate` G6 의 사각지대 확정**: G6 는 `logic` 만 본다.
실제 오염은 **`knobs[].effect`** 에 있었다(09 E2 — `history[5]` 는 옳은데 `knobs[10]` 이 옛 결론).

---

## 1. 실오류 — JSON 경로 패치 목록

### E1. `/specs[9]/closed[0]/q` — 3차 E2 정정이 **이동만 되고 본문은 그대로**

```
경로: /specs[9]/closed[0]/q  (v2 출처 = specs[9].resolved/open 이동분)
구(말미): "... 나머지 7곳(plan_legacy::handler)은 런타임 version SSA 값을 넘긴다
           ⟹ 주 경로에서는 살아 있는 버전 게이트다. 재구현 시 0 하드코딩 금지.
           미탐색 = 피호출자 내부의 version 분기"
신(말미): "... 나머지 7곳(plan_legacy::handler)은 런타임 version SSA 값을 넘긴다(관측은 유효).
           단 ~~주 경로에서는 살아 있는 버전 게이트다 · 0 하드코딩 금지~~ → **거짓**:
           피호출자 1단(m07.ll:48227 `#dbg_value(i64 poison, !60288)`)과 2단(m07.ll:47470
           `enemy_minion_line_action_damage_at` 도 `i64 poison`)에서 죽어 있다
           ⟹ 체인 전체에서 죽은 인자. 0 하드코딩도 결과 불변(시그니처 호환용으로만 유지)."
```
- 근거: `/specs[9]/sig/params[0]/role`(**ev 2**)이 이미 "체인 전체에서 죽은 인자"라고 확정했는데
  같은 스펙의 `closed[0]/q` 가 반대로 끝난다 = **스펙 내부 자기모순**(3차 E2 의 재발이 아니라 **미반영**).
- 재실측(이번 라운드): `B4_o09f.exe` → `VERSION base=true diff=0 all=[false×0 / true×12]`
  (version 12종 `0,1,2,3,12,24,30,40,46,50,60,99` 전부 동일).
- ev: 2
- ⟹ **BRIEF §1① 의 「3차 정정이 전부 반영됐다」는 이 건에 대해 거짓**이다.

### E2. `/specs[9]/knobs[10]`(에픽버프 위험표 게이트) — **분기 극성 + 기작 오류**

```
경로: /specs[9]/knobs[10]/value , /specs[9]/knobs[10]/effect   (v2 출처 = knobs)
구 value : "≠0 이면 엄격표"
구 effect: "강제로 0 을 읽게 하면 **항상 완화표** → 적 미니언 웨이브를 훨씬 덜 무서워한다.
            강제 ≠0 이면 항상 엄격표"
신 value : "≠0 이면 엄격표 — ★단 09 경로에서는 표 선택에 영향이 없다"
신 effect: "★09 의 호출부는 `champion_action` 에 **리터럴 true** 를 넘긴다(m15.ll:35472
            `i1 noundef zeroext true`). 표 선택은 `m07.ll:48291 %36 = or i1 %6, %35`
            = `or(champion_action, buff≠0)` ⟹ **09 경로는 buff 값과 무관하게 항상 엄격
            (is_dangerous)표**다. '강제로 0 을 읽게 하면 항상 완화표'는 09 에서 성립하지 않는다.
            buff 의 **실제** 개입 지점은 한 단 아래 `enemy_minion_line_action_damage_at`
            (minion_wave_risk.rs:132) 이다 — buff≠0 이면 그 함수가
            `enemy_minion_wave_risk_damage_at` 으로 **damage 모델을 통째로 위임**한다.
            실측(이 환경): buff=0 → damage 73(라인액션 모델) / buff≠0 → 128(=wave_risk 값)
            ⟹ 방향은 '덜 무서워진다'가 맞지만 **기작이 표 선택이 아니라 모델 교체**다."
```
- 근거(IR): `m07.ll:48287~48292` — `%32 = gep %24, 576` → `%34 = load` → `%35 = icmp ne %34, 0`
  → `%36 = or i1 %6, %35` → `br %36, %38(엄격), %66(완화)`.
- 근거(오라클 `B4_o09m.exe`): `OR` 8/8 MATCH — `strict = champion_action || buff≠0` 예측과 전건 일치.
  `BUFF 0 → damage_at=73 / wave_risk=128 / equal=false`,
  `BUFF 1 → 128 / 128 / equal=true`, `BUFF 600 → 128 / 128 / equal=true`.
- ev: 2
- ★**같은 사실이 `/specs[9]/history[5]` 에는 이미 옳게 적혀 있다**("09 는 champion_action = true 라
  **항상 is_dangerous 표**를 쓰고"). ⟹ 3차 정정이 `history` 에만 들어가고 `knobs` 로 번지지 않았다.
  **`specgate` G6 가 `logic` 만 보기 때문에 이 부류는 기계적으로 안 잡힌다**(§5).

### E3. `/specs[9]/closed[0]/why` ↔ `/specs[9]/closed[1]/why` — **근거 오귀속(인덱스 밀림)**

```
경로: /specs[9]/closed[0]/why  ·  /specs[9]/closed[1]/why
현재: closed[0](q = "version(p1) 이 실제로 무엇을 가르는지 …")  why = "3차 배치B: 전문 확정(임계표 전량)"
      closed[1](q = "enemy_minion_line_action_danger_damage_at 의 내부 …")
                why = "1차 배치B resolved[0] 전문 확정 + 2차 인자 전건 IR 재확인"
신  : closed[0].why = "2차 배치B: 피호출자 2단 모두 i64 poison + 오라클 version 12종 diff 0 (죽은 인자)"
      closed[1].why = "3차 배치B: 전문 확정(임계표 전량) + 4차 배치B: 피호출자
                       enemy_minion_line_action_damage_at 본문까지 확정(오라클 미니언 주입)"
```
- 「임계표 전량」은 **미니언 함수 항목**(closed[1])의 근거이고 version 항목(closed[0])의 근거가 아니다.
  `_spec\closelist.py` 가 needle 로 항목을 특정하는데 09 의 두 항목이 같은 함수 이름을 포함해
  0번에 붙은 것으로 보인다(추정 — 재현은 `closelist.py` 의 09 needle 2개 참조).
- ev: 3 (문서 대조)

---

## 2. 과열림 — `open[] → closed[]` 권고 8건 + class 재분류 2건

★**원인 확정**: `_spec\closelist.py` 의 `CLOSE` 목록에 **3차분 항목이 단 한 건도 없다**(1·2차분만).
`patch3.py` 는 `logic`/`params`/`history` 만 고치고 open/closed 이동은 `closelist.py` 소관인데
그 목록이 갱신되지 않았다. ⟹ 내 배치의 `open 13건` 중 **8건이 3차에 이미 닫힌 것**이다.
(BRIEF §2 가 요구한 「open 에 이미 닫힌 게 섞여 있으면 보고」의 대량 사례)

| 경로 | 3차 판정 | 4차 확인 |
|---|---|---|
| `/specs[5]/open[1]` RawVec::grow_one | 닫힘(판정 무관) | 동의 — std 재할당 정책. `Vec::push` 관측 의미 불변 |
| `/specs[6]/open[0]` 거리 임계 40000000001·22500000001 | 닫힘 | 재실측 일치(§4) — `knobs` 에 ev3 로 이미 있고 `consts` 제외는 QC 규칙 탓 |
| `/specs[6]/open[1]` is_recent_visible / is_ignored_well_enemy | 닫힘 | 동의 — 판정 정본이 `shared.is_recent_visible` + `callees` 에 있다 |
| `/specs[6]/open[2]` self 용도 | 닫힘(표기 불가로 분리) | 동의 — IR define 1번 인자 소거 = 결과 무영향은 **확정**, 소스 텍스트 유무만 표기 불가 |
| `/specs[7]/open[0]` `_debug` readnone | 닫힘 | 재확인 — `m02.ll:48918` 8번째 인자 `ptr noalias readnone align 8 captures(none)` |
| `/specs[7]/open[1]` vtable 0x40/0x1f0 구현체 | 닫힘 | 재확인 — `m02.ll:48974 gep %36, 64` / `49016 gep %36, 496`. 슬롯 공식 = `shared` 정본 |
| `/specs[7]/open[2]` 상수 2 = bounds-check | 닫힘 | 재확인 — `m02.ll:48933 icmp ult %11, 2` (team), pos 는 `!range` 로 체크 소멸 |
| `/specs[8]/open[0]` self 4필드 소비처 | 닫힘(C8) | 동의 — 같은 플랜 `sub_plan`(m10.ll:7902~9384)이 전부 읽는다 |

class 재분류 2건(값 변경 없음):
- `/specs[5]/open[0]/class` : `미탐색` → **`재료 부재`**
  범위 = DWARF · LLVM IR · rmeta 주석 · rmeta SourceMap · tcx 전부 **현재 스냅샷만** 담아 소스 이력이 없다.
  미탐색 = 개발사 VCS. (3차 판정 그대로인데 class 가 반영 안 됨)
- `/specs[7]/open[3]/class` : `미탐색` → **`재료 부재`**
  범위 = ①`DILocation.column` 전 모듈 0 ②`mir=0 xinl=0` ③L36 에 패닉 Location 없음
  ④줄 길이 산술은 교환 불변 ⑤IR `or` 평탄화 = 정보량 0.
  ★4차 보강: **세 항이 전부 순수 불리언 로컬/비교**임을 IR 로 재확인했다 —
  `can_upgrade_item`(%85)은 **L35 의 독립 문장**이라(`upgrade_item` 호출 `m02.ll:49067` 이
  `%86 = icmp eq ptr %57, null`(epic.is_some) **분기보다 앞**, 즉 무조건 실행) `||` 안에 없다.
  ⟹ RNG 소비 같은 부수효과로도 순서를 못 가른다. **재료 부재 판정 강화**(3차 판정 유지).

09 의 3건은 **유지**(3차와 동일): `open[0]` 원본 소스 부재 · `open[1]` shl 접힘 표기 · `open[2]` 바깥 `||` 순서.

---

## 3. 새 발견

### N1. ★`enemy_minion_line_action_damage_at` 전문 복원 (minion_wave_risk.rs:130~231)

09 의 미니언 게이트에서 **마지막으로 남아 있던 피호출자 본문**이다
(3차 C9 가 "남은 미탐색 = 이 본문, 미니언이 필요하다"로 남긴 것).
IR = `_gaibc/m07.ll:47470~47998`. 지역변수 이름·줄번호는 **DWARF `!DILocalVariable` 실측**.

```rust
// minion_wave_risk.rs:130  (pub, mir=false xinl=false)
fn enemy_minion_line_action_damage_at(
    version: usize, data: &OperationData, target: &Entity,   // L130
    x: u64, y: u64, window_tick: usize,                      // L131
    champion_action: bool, predict_retarget: bool) -> usize
{
    // L132 ★모델 분기 — 인라인된 enemy_minion_wave_has_epic_buff(L92~96)
    if enemy_minion_wave_has_epic_buff(data, target) {
        return enemy_minion_wave_risk_damage_at(version, data, target, x, y, window_tick);  // L133
    }
    // target.team 이 Player(..) 가 아니면 여기서 이미 0 을 반환(IR: 진입 블록 -> phi 0)
    let target_team = ...;                                   // L136
    let enemy_team  = 1 - target_team;                       // L140
    let window_tick   = window_tick.max(tps / 2);            // L141  ★하한 = 0.5초
    let tracking_tick = window_tick.min(tps);                // L142  ★상한 = 1초
    let retarget_tick = tps / 2;                             // L143
    let mut damage = 0;                                      // L144
    for minion in data.cache.iter_minions(enemy_team) {       // L146
        let EntityType::Minion { info } = &minion.ty else { continue };      // L147 (ty 태그 == 1)
        if !is_near_line(data.context, x, y, info.line) { continue }         // L150
        let attack = minion.attack_effect.as_ref() else { continue };        // L154 (None 이면 스킵)
        let targeting_me       = info.nearest_enemy == Some(target.id);      // L160
        let idle               = info.nearest_enemy.is_none();              // L161 (IR 상수 false 경로)
        let targeting_champion = if targeting_me || !champion_action { false }
                                 else { get_entity_by_id(nearest_enemy)      // L167
                                        .is_some_and(|e| e.is_champion()) }; // L168 (ty 태그 13)
        let retarget_pending   = if targeting_me || !predict_retarget { .. }
                                 else { let victim = get_entity_by_id(id)?;  // L173/L174
                                        victim.team == target.team && !victim.is_champion()
                                        && attack.expected_damage_target(ctx, minion, victim) // L175
                                           * window_tick
                                           >= victim.hp * minion.attack_cooltime().max(1) };  // L176
        // L179: targeting_champion |= retarget_pending
        // L180: if !(idle || targeting_me || targeting_champion) { continue }
        let hit_damage = attack.expected_damage_target(ctx, minion, target); // L184
        if hit_damage == 0 { continue }                                      // L185
        let move_tick   = if targeting_me { tracking_tick } else { retarget_tick };   // L189
        let move_offset = minion.stat_cached.move_speed * move_tick;                  // L190
        let range_offset_base = if targeting_me { move_offset }                       // L191
                                else if targeting_champion { move_offset.min(24_000) }   // L193/194
                                else { (move_offset / 2).min(16_000) };                  // L196
        let range_offset = range_offset_base + 8_000;                                 // L198
        let in_attack_range = attack.is_in_range_ex(minion, target,
                                   minion.x, minion.y, x, y, 0);                      // L200
        if !in_attack_range && !attack.is_in_range_ex(minion, target,
                                   minion.x, minion.y, x, y, range_offset) { continue }  // L201
        let target_weight   = if targeting_me { 100 }                                  // L204
                              else if targeting_champion { 70 } else { 55 };
        let distance_weight = if in_attack_range { 100 }                               // L205
                              else if predict_retarget { 100 }                        // L213
                              else if targeting_me { 70 } else { 50 };
        let followup_damage = hit_damage * window_tick / minion.attack_cooltime().max(1); // L224
        let wave_damage     = followup_damage + hit_damage;                              // L225
        damage = damage.saturating_add(distance_weight * target_weight * wave_damage / 10_000); // L227
    }
    damage.min(target.hp.saturating_mul(2).max(1))            // L230 ★반환 상한
}                                                             // L231
```

**오라클 검증(`B4_o09m.exe`)**
- **반환 상한 `min(max(hp*2,1))` — 10/10 MATCH** (`CAP` 행: hp 0→1 · 1→2 · 2→4 · 3→6 · 10→20 이
  전부 상한에 물리고, hp 36→72, hp 50/100/73/146 은 원시값 73 으로 통과. 예측식 `min(raw, max(hp*2,1))` 전건 일치)
- **`window_tick` 하한 = tps/2 확정**: `WIN w=0,1,15,29,30,31 → 전부 30` (하한이 없으면 w=0 은 후속타 0)
  이후 `w=45→36 · 59→42 · 60→44 · 61→44 · 120→73 · 600→304` (정수 나눗셈 절단으로 60/61 동일)
- **L132 모델 위임 확정**: `BUFF 0 → damage_at 73 ≠ wave_risk 128` / `BUFF 1·600 → damage_at 128 == wave_risk`
- `predict_retarget` 는 결과를 바꾼다(73 → 87). 이 환경의 `champion_action` 은 무영향(미니언이
  아군 챔피언을 타깃하고 있지 않아 `targeting_champion` 분기가 죽어 있다 — **범위 명시**)

**신규 오프셋**(전부 `tcxdict` 정본 대조 완료)
- `Entity+0x68` = `ty` 판별자 (EntityType, Direct · 1=Minion · 13=Champion)
- `Entity+0x88` = `ty@Minion.info.nearest_enemy` 판별자
- `Entity+0x90` = `ty@Minion.info.nearest_enemy@Some.0`
- `Entity+0x11a` = `ty@Minion.info.line` 판별자 (LineType, 1B)
- `Entity+0x490` = `attack_effect@Some.0.ty`(Effect 56B 시작)
- `Entity+0x4c0` = `attack_effect` 판별자 (Niche · **-1 = None** → 그 미니언 스킵)
- `Entity+0x640` = `stat_cached.move_speed`
- `GameSetting+0x1410` = `minion_wave_setting` (MinionWaveSetting 128B)
- `MinionWaveSetting+0x0` = `start_tick`
- `MinionWaveSetting+0x8` = `tick_per_wave`
- `MinionWaveSetting+0x10` = `melee_count`
- `MinionWaveSetting+0x18` = `range_count`
- `MinionWaveSetting+0x20` = `tick_per_spawn`
- `MeleeMinionInfo+0x0` = `attack` (TargetAttackAction 80B)
- `MeleeMinionInfo+0x50` = `stat` (EntityStat 72B)
- `MobaMode+0x240` = `epic_minion_buff_time` (`[usize;2]` · 기존 확정 재확인)

### N2. ★BRIEF §1⑥ 「미니언 0마리」 우회 확립 — 오라클 레시피 추가

`GameSetting::default()` 의 `minion_wave_setting` 16필드가 전부 0이어서 미니언이 안 생기던 것을
실전값(`<게임설치>\bundle_unpacked_full\setting\game_setting.game_setting`)으로 채우고
`melee_minion`/`range_minion`(스탯·성장·attack 액션)까지 넣은 뒤 `Game::run_tick` 600틱:

```
TICKS 100 → minions=18   (이후 300·600 틱에서도 18 유지)
MINION_COUNT team=0 → 9 · team=1 → 9      (3라인 × (melee 2 + range 1))
attack_effect=Some 전량 · hp 400/400(melee) · 250/250(range) · move_speed 800
```
⟹ **`damage_at > 0` 을 이 프로젝트에서 처음 관측**했고 그 위의 모든 검증이 가능해졌다.
⚠`MinionTargetProjectileAction`(range_minion.attack)에는 `attack_ratio` 가 **없다**(`speed` 가 있다) —
`MeleeMinionInfo.attack`(`TargetAttackAction`)와 필드 집합이 다르다(실컴파일 E0609 로 발각).

### N3. ★07 `Hide`(태그 9) 경로 개방 — 3차의 「미도달」 해소

3차는 `EpicHuntAndBattlePlan::target_bush` 가 private 이라 `Default`(=None)로만 만들 수 있어
Hide 경로를 못 밟았다. `transmute` 로 우회한다(구조 근거를 명시해 둔다):

```rust
assert_eq!(std::mem::size_of::<EpicHuntAndBattlePlan>(), 16);   // 실측 16
// 필드 1개 = Option<usize> {tag@+0x0, val@+0x8}, tag 1 = Some
//   (IR m02.ll:49105 `%110 = load i64, ptr %1, !range !12115` + `trunc nuw i64 -> i1`)
let plan: EpicHuntAndBattlePlan =
    unsafe { std::mem::transmute::<[usize;2], EpicHuntAndBattlePlan>([1usize, 7usize]) };
```
결과(`B4_o0708.exe`):
```
HIDE_RAW  tag=9  bush=7  out_line=1  check_move=0  spotted=0
          bytes=[9,0,0,0,0,0,0,0, 7,0,0,0,0,0,0,0, 1,0,0,0,0,0,0,0]
HIDE_DBG  Hide(HideSubPlan { bush: 7, out_line: Outline, check_move: false, enemy_spotted_me: false })
```
⟹ `/specs[7]/logic` 의 Hide 페이로드 4필드(`bush` / `out_line=Outline(1)` / `check_move=false` /
`enemy_spotted_me=false`)가 **전량 실행으로 확증**. `/specs[7]/consts` 의 태그 9 와 `1`(Outline)도 ev4→**ev2**.
- 덤: `bush=Some` + epic 풀피 + ratio 30 → **Recall(5)** 이 이긴다 ⟹ **L36 이 L41 보다 먼저**(ev2 분기 순서).

### N4. ★09 `logic` 독립 재구현 대조 **1800/1800 MATCH** (ev2)

2차의 400/400 은 *같은 프로브가 만든 진리표*였다. 여기서는 `/specs[9]/logic` 의 1203~1348 을
**그대로 Rust 로 옮겨 놓고**(`B4_o09f.rs` 의 `fn mine`) 실제 `game_ai::check_favorable_engage_formation`
과 대조했다 — 무작위 배치 600종 × `engage_range` 3종(100000·200000·400000):

```
RESULT  n=1800  ok=1800  bad=0  true_count=1337     (true 1337 / false 463 — 판별력 확인)
VERSION base=true  diff=0  all=[true×12]            (version 0,1,2,3,12,24,30,40,46,50,60,99)
```
⟹ 각도 분류(`dot*4`/`dot*100 vs lp*9`/`cross*100 vs lp*9`) · 3분류 분기 순서 ·
`rear>0` / `flank>1 || (flank>0 && front>0)` / `front>1 → c2b*5 <= e2b*6` · 빈사 40% 컷 ·
`engage_range+100000` 여유 · i128 부호연산 — **전부 실행으로 확증**.
⚠범위: 미니언 게이트(1208~1209)는 재구현이 아니라 **실제 피호출자를 그대로 호출**했고,
챔프 HP 를 크게 둬 `dmg=0` 이 되도록 했다. 즉 이 1800건은 **대형 판정부의 검증**이다.
- 부수 함정 기록: 아군을 `base.clone()` 으로 만들면 **`id` 가 전부 같아** `ally.id == champ.id` 컷에
  전원 걸려 `true_count=0` 으로 붕괴한다(첫 실행에서 실제로 밟았다). `cache.player_champion[0][k].id` 를
  받아 넣어야 한다. → `shared.오라클_레시피_함정` 에 추가 권고.

### N5. 07 / 08 ev2 경계 재확인 (실전 `GameSetting` 으로) — 24/24

```
07  ratio 49 → tag 5(Recall) · 50 → 5 · 51 → 11(EpicHunt) · 52 → 11 · 100 → 11
    ratio 51 + 힐영역 안 → 5 / ratio 100 + 힐영역 안 → 11 (둘째 조건 hp<max 불성립)
    epic hp=max-1 → 11 / live_list 비움 → 11
08  15×tps = 900 : remain 899 → false · 900 → false · 901 → true · 5000 → true
    ★tick=0 과 tick=120 **양쪽에서 동일** ⟹ `next_respawn_tick.saturating_sub(tick) > 15*tps` 확정
RESULT ok7=12 bad7=0 ok8=12 bad8=0
```
- `map.fountains[0] = (0, 896000, 64000, 960000)` · `[1] = (892000, 0, 960000, 64000)` —
  **실전 setting(width/height 960000)에서도 3차·2차와 같은 값** ⟹ `MapDef::moba` 의 fountains 는
  `GameSetting` 좌표에 의존하지 않는 상수표다(신규 관측).

### N6. ★05 의 산출물은 오라클로 관측 가능하다 — 과제 지시문 범위 정정

지시문의 「05·06 은 오라클이 `error[E0624]` 로 진짜 막힌다」는 **함수를 직접 호출하는 것**에 대해서만 참이다.
`game_ai::AgentVerHamster`(**pub**, `game-ai\src\lib.rs:122`)에
`pub fn plan_v50_dive_episodes`(`lib.rs:321`)가 있고, `pub fn new`(`lib.rs:368`)로 만들 수 있다.
⟹ 전체 시뮬을 돌린 뒤 **`V50DiveEpisode` 레코드 Vec 자체를 pub 으로 꺼내** `end_plan`(1..9)·
`end_reason`(0..8) 분포와 레코드 스키마(3차 C7 = `prev_holder_hp`/`gap_ticks` 부재)를 ev2 로 검증할 수 있다.
⚠**이번 라운드엔 실행하지 않았다**. 이유(범위 명시) = 05 의 남은 `open` 2건은
①소스 이력(`_version`/`_tps`) ②std 재할당 정책이라 **이 경로로도 답이 안 나온다**.
다음 라운드가 05 의 레코드/코드표를 ev2 로 올리고 싶을 때의 진입점으로 남긴다.
(같은 형제군에 `plan_v46_flee_episodes`·`plan_gank_periods`·`plan_steal_sessions` 등 pub 접근자 다수)

### N7. 3차 C9 / `shared` 의 `||` 피연산자 순서 정정 (논리값 동일 · 재현 무영향)

3차 C9 와 `/specs[9]/history[0]`·`history[5]`·`shared.전투_위협모델.미니언_위험표_게이트` 는
`if champion_action || enemy_minion_wave_has_epic_buff(data, target)` 로 적었다.
IR 은 그 반대다 — `has_epic_buff` 가 **무조건 평가**된다:
```
m07.ll:48247  %10 = load i64, ptr %2        ; target.ty 태그 (champion_action 과 무관)
m07.ll:48249  br i1 %11, label %37, label %12
m07.ll:48266  %21 = tail call { i64, ptr } %20(...)   ; get_game_mode — 간접 호출
m07.ll:48283  panic_bounds_check(...)                 ; 패닉 경로
m07.ll:48291  %36 = or i1 %6, %35                     ; ← 여기서 처음 champion_action 사용
```
간접 vtable 호출과 패닉 경로가 있어 LLVM 은 이 블록을 `%6` 분기 위로 **투기할 수 없다**.
⟹ 소스는 `enemy_minion_wave_has_epic_buff(..) || champion_action` 꼴(또는 비단축 결합)이다.
**표 선택 결과는 `or` 로 동일하므로 판정·재구현에는 영향이 없다** — 표기(단축평가 방향) 정정이다.
권고 패치 경로: `/specs[9]/history[0]/now`(3항) · `/specs[9]/history[5]/now` ·
`/shared/전투_위협모델/미니언_위험표_게이트` 의 `champion_action || has_epic_buff` → `has_epic_buff || champion_action`.

---

## 4. `ev<=3` · `ev2` 표본 재확인 (뒤집힘 0)

| 대상 | 방법 | 결과 |
|---|---|---|
| `/specs[7]/consts` 51 (ev4→2 권고) | 오라클 `B4_o0708` | OK — 49/50→Recall, 51/52→EpicHunt (경계 50/51) |
| `/specs[8]/consts` 15 (ev4→2 권고) | 오라클 `B4_o0708` | OK — 899/900→false, 901→true, tick 0·120 양쪽 |
| `/specs[9]` 진리표 (ev2, 2차 400/400) | **명세→재구현 1800건 대조** | OK — 1800/1800, true 1337 |
| `/specs[9]/sig/params[0]` version (ev2) | 오라클 12종 | OK — diff 0 (단 `closed[0]/q` 와 모순 = **E1**) |
| `/specs[6]/knobs` 40000000001 (ev3·4) | IR `m12.ll:41992` | OK — `icmp ult i64 %47, 40000000001` |
| `/specs[6]/knobs` 22500000001 (ev3·4) | IR `m12.ll:42077` | OK — `icmp ult i64 %47, 22500000001` |
| `/specs[6]/consts` 2 (version<2) | IR `m13.ll:45232` | OK — `icmp ult i64 %0, 2` |
| `/specs[8]/knobs` 22500000000 | IR `m10.ll:7812` | OK — `icmp ugt i64 %85, 22500000000` |
| `/specs[8]/consts` 15 | IR `m10.ll:7888` | OK — `mul i64 %121, 15` |
| `/specs[9]/open[2]` select 단축평가 | IR `m15.ll:35598~35602` | OK — `%97 = select i1 %95(flank>0), i1 %96(front>0), i1 false`, `%99 = or i1 %98(flank>1), %97` |
| `MobaMode` Vec 배치 0x198/0x1a0/0x1a8/0x1b0 | tcxdict | OK — cap/ptr/len/next_respawn_tick 전건 일치 |
| `/specs[7]` vtable 0x40·0x1f0 | IR `m02.ll:48974 / 49016` | OK |
| `/specs[5]`·`/specs[9]` mem `chk` | tcxaudit 내장 | OK — 05 30/30 · 09 14/14 (07·08 의 vtable 6행만 기존 「확인불가」) |

부수 관측(오류 아님): `/specs[9]` 의 카운터는 IR 에서 **`i32`** 다(`icmp sgt i32`). `logic` 은 타입 미표기.

---

## 5. ★`specgate` G5/G6 사각지대 — 이 라운드의 구조적 소득

BRIEF §1② 가 "G5/G6 가 놓치는 같은 부류가 남아 있으면 찾아라"고 했다. **찾았다.**

- **G6 는 `logic` 만 검사한다.** 그런데 `logic` 과 **같은 성질의 산문 필드가 3개 더** 있다:
  `knobs[].effect` · `knobs[].value`(문자열형) · `closed[].q`.
  이 라운드의 실오류 3건은 **전부 그 3개 필드**에 있었고 `logic` 은 깨끗했다(§6 기계 대조 0건).
- 권고 검사 2개(둘 다 이번 3건을 기계적으로 잡는다):
  1. **G7** — `history[].now` / `sig.params[].role` 에 `~~…~~ → 거짓/정정` 형태로 부정된 문구가
     **같은 스펙의 `knobs[].effect` · `closed[].q` · `open[].q` 에 살아 있는지** 대조(= G6 의 대상 확장).
  2. **G8** — `closed[i].why` 의 근거 문장이 `closed[i].q` 의 주제어와 겹치는지(오귀속 탐지).
     09 의 E3 는 `why`="임계표 전량" ↔ `q`="version 이 무엇을 가르는지" 로 **주제어 교집합 0** 이다.
- 추가 권고: `closelist.py` 에 **3차분 CLOSE 항목**을 넣어야 한다(§2). 지금은 라운드가 바뀔 때마다
  같은 항목을 다시 닫는 작업이 반복된다.

---

## 6. `logic` ↔ 정본 기계 대조 (`_xcheck.py`) — 불일치 0

`logic` 안의 모든 `Base+0xNNN` 을 뽑아 그 스펙 `mem[]` 의 오프셋 집합과 대조했다.

| 스펙 | logic 오프셋 | mem 미등재 | 판정 |
|---|---|---|---|
| 05 | 31종 | 6종 (`live+0x10/0x18/0x58/0x60` · `self+0x578` · `self+0x5c0`) | **오류 아님** — 앞 4개는 `V50DiveEpLive` **상대** 오프셋(logic 이 절대값을 괄호로 병기), 뒤 2개는 "읽지 않는다"는 서술 대상. `self+0x570`(Option 판별자)가 곧 `prev_holder_hp` 의 Option 태그라 `self+0x578` = 그 값칸 — 3차 C7 과 정합 |
| 06 | 2종 | 0 | OK |
| 07 | 8종 | 1종 (`MobaMode+0x198`) | **오류 아님** — tcxdict 로 `0x198 = live_list.buf.inner.cap`(Vec 시작) 확인. mem 은 실제로 읽는 `0x1a0`/`0x1a8` 만 싣고 있어 옳다 |
| 08 | 11종 | 1종 (동상) | 동상 |
| 09 | 0종 | 0 | OK |

`callees_unmatched` 에 **판정 술어 혼입 0건**(3차와 동일). 단 표기 권고 2건:
- `/specs[9]/callees[0]` 이 **자기 자신**(`check_favorable_engage_formation`)이다 — 자동 채움이
  `logic` 첫 줄의 `fn <이름>(` 를 호출로 긁은 잡음. 값 오류는 아니나 `callees` 개수(2)가 오해를 부른다.
- `/specs[9]/logic` 의 `dist_sq` → `distance_sq`(`game_core::utils::distance_sq`, utils.rs:6)로 통일하면
  `callees_unmatched` 가 비워진다(3차 권고 미반영).

---

## 7. 이번 라운드의 브리핑/지시문 오류

1. **부분 오류** — BRIEF §1① 「3차 정정이 **전부** 반영됐다 / 실오류 19건 + 과열림 13건이 v3 에 들어갔다」.
   배치 B 기준 `logic`/`params`/`history`/`closed[].why` 는 들어갔지만
   **open→closed 이동 8건 · class 재분류 2건 · ev 상향 8건은 하나도 안 들어갔다**
   (`closelist.py` 에 3차분 CLOSE 항목 0건 · `/specs[7]/consts` 51 은 여전히 ev4 · vtable 6행 여전히 ev4).
2. **부분 오류** — 과제 지시문 「05·06 은 오라클이 `error[E0624]` 로 **진짜** 막힌다」.
   함수 직접 호출은 맞지만 **05 의 산출물은 `AgentVerHamster::plan_v50_dive_episodes`(pub)로 관측 가능**하다(N6).
3. **수치 불일치** — BRIEF §5 「`tcxaudit --prose` 를 인자 없이 돌리면 766건 · 부분일치 2 · 확인불가 18」.
   실측(이 라운드 시작 시점) = **680건 · 오귀속 0 · 밀림 0 · 부분일치 1 · 확인불가 18 · OK 661**.
   (기준선 자체는 문제없으나 "총계가 늘었는지"로 판정할 때 숫자를 680 으로 쓸 것)
4. **맞음** — 「09 의 `enemy_minion_line_action_damage_at` 은 미니언이 필요하다 / §1⑥ 에 우회법이 있다」.
   실제로 그 우회로 열렸다(N1·N2).
5. **맞음** — 「07 `hp_ratio` 50/51 · 08 900 · 09 진리표가 뒤집히면 사고다」. **뒤집히지 않았다**(§4).
6. **맞음** — BRIEF §3① `init_tower`/`init_nexus` 재호출 금지. 세 프로브 전부 미호출,
   자기검증 4줄 `towers 16 / twin 2/2 / is_top_side_true 8/10` 전건 재현.
7. **맞음** — BRIEF §1⑤ 「실전 챔피언 데이터가 열렸다」. 단 이번 라운드에 필요했던 것은
   챔피언이 아니라 **미니언 설정**이었고, 그쪽이 §1⑥ 대로 함정이었다.

---

## 8. ev 상향 권고

- `/specs[7]/consts` value 51 → **ev 2** (`B4_o0708` 49/50 vs 51/52)
- `/specs[7]/consts` value 9 (Hide 태그) → **ev 2** (N3 — 3차엔 미도달이었다)
- `/specs[7]/consts` value 1 (AroundBushOutlineType::Outline) → **ev 2** (N3 바이트 실측 `out_line=1`)
- `/specs[7]/consts` value 5 · 11 → **ev 2** (런타임 태그 확인)
- `/specs[8]/consts` value 15 → **ev 2** (899/900/901, tick 0·120)
- `/specs[9]/consts` 40 · 100 · 9 · 4 · 5 · 6 · 100000 · 1 → **ev 2** (N4 1800/1800 재구현 대조)
- `/specs[7]/mem` · `/specs[8]/mem` vtable 4행(0x28·0x40·0x100·0x1f0) → **ev 3**
  (3차 C5 슬롯 공식이 `shared` 에 들어갔는데 행의 `ev` 는 여전히 4)

---

## 9. 재현 명령

```
sh  C:\tfm2mods\MIG\_verify3\build.sh C:\tfm2mods\MIG\_verify4\B\B4_o09m.rs
%TEMP%\tfm2_spanprobe\B4_o09m.exe
sh  C:\tfm2mods\MIG\_verify3\build.sh C:\tfm2mods\MIG\_verify4\B\B4_o0708.rs
%TEMP%\tfm2_spanprobe\B4_o0708.exe
sh  C:\tfm2mods\MIG\_verify3\build.sh C:\tfm2mods\MIG\_verify4\B\B4_o09f.rs
%TEMP%\tfm2_spanprobe\B4_o09f.exe
python -X utf8 C:\tfm2mods\MIG\_verify4\B\_xcheck.py
```

## 10. 제출 게이트

```
python -X utf8 tcxaudit.py --prose _verify4\B\B4_REPORT.md
python -X utf8 specgate.py
```
결과는 §11 에 기록.

## 11. 제출 게이트 결과

```
python -X utf8 tcxaudit.py --prose _verify4\B\B4_REPORT.md
  -> 총 697건  오귀속=0  밀림=0  부분일치=1  확인불가=18  OK=678
     (기준선 680건 -> 697건 = 이 문서 추가분 17건, **전부 OK**.
      부분일치 1 · 확인불가 18 은 기존 항목 — 개수가 늘지 않았다)
python -X utf8 specgate.py
  -> G1 자기모순=0  G2 호출부 전수=0  G3 형제 함수=0  G4 술어 시그니처=0
     G5 sig 정본 대조=0  G6 logic 미반영=0   총 0건
     ★단 이 라운드의 실오류 3건은 전부 G1~G6 밖이다(§5) — knobs[].effect · closed[].q · closed[].why.
```
