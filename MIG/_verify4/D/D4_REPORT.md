# 4차 반증검증 — 배치 D (15~19) 보고서

게임 0.5.8 / SDK `sdk_058` / toolchain `nightly-2026-05-24` / 2026-09-11
작업 파일 = `C:\tfm2mods\MIG\_verify4\D\` (`ann.py`·`mkprobe.py`·`body_o1.rs`·`D4_o1.rs`·`body_o2.rs`·`D4_o2.rs`·`o1.txt`·`o2.txt`·`f16.ll`·`stdv_a.txt`·`v3_1[5-9].json`)

---

## 0. 요약 — 실오류 6건 · 판정반전 3건 · 신규확정 3건 · ev<=3 뒤집힘 0건

| # | 대상 | 종류 |
|---|---|---|
| E1 | `/specs[16]/logic` 슬롯1 구조 (필드 읽기 → **메서드 4종 호출 + `as_ref()`**) | 실오류(구조·호출) |
| E2 | `/specs[16]/callees` 에 게임 함수 **4개 누락** (`Entity::attack_effect`/`skill_effect`/`skill2_cooldown`/`ult_cooldown`) | 실오류(정본 누락) |
| E3 | `/specs[15]/mem[8]/name` = `ty.Tower.0` — 3차가 `logic` 에서 고친 **튜플 variant 오기가 `mem` 에 그대로** | 실오류(구조) |
| E4 | `/specs[17]/logic` "틱·누적 계열 **8필드**" → 실제 **9필드** | 실오류(값) |
| E5 | `/specs[18]/mem[15]` `TeamPlan 0xc8/0xd0` 묶음 행 (한 행 = 한 오프셋 위반) | 실오류(감사 무력화) |
| E6 | 3차 P-12 **미적용** — `/specs[18]/open[2]` 가 `/specs[17]/closed[6]` 과 같은 사안인데 여전히 open | 과열림 |
| R1 | `single_tower_dive_is_viable` 「9축 전부 무관」 → **대상 hp 는 판별 축이다**(1999/2000 경계) | ★판정반전 |
| R2 | `base_sub_goal` 「갈림은 가시성이 아니다 / 거리 임계 1e6 이하 없음」 → **적 우물 위험 판정**이 축 | ★판정반전 |
| R3 | METHOD_MAP ⑥한계1 「`fight_check`·`path_finder` 는 `pub(crate)` 라 막힌다」 → **핵심 진입점 3개가 `pub`** | ★판정반전 |
| N1 | `battle.rs:2397~2429` **전 33줄 소스 복원(±0)** — 3차가 남긴 「마지막 1자 모순」 해소 | 신규확정 |
| N2 | `single_tower_dive_is_viable` 본체 전량 + **판별식 확정**(game==mine 12/12) | 신규확정 |
| N3 | ★**오라클 함정 신규** — `check_kill_die_tick` 은 **TLS 메모**라 한 프로세스에서 세계를 바꿔도 첫 값이 재생된다 | 신규확정 |
| — | `ev<=3` 표본 재확인 (sig.tcx 5건 + sig.params 1건 + mem 3건 = 전수) | 뒤집힘 **0건** |
| — | `callees_unmatched` 판정 술어 혼입 | **0건**(3차와 동일) |

---

## 1. 패치 목록 (JSON 경로)

### E1 `/specs[16]/logic` — 네 이펙트 슬롯이 **필드 읽기가 아니라 메서드 호출**이다 ★실오류
```
구(4곳):
  if let Some(e) = champ.attack_effect {     // Entity+0x490, casting(+0x30) != -1 이면 Some
  if let Some(e) = champ.skill_effect {      // Entity+0x4c8
  if let Some(e) = champ.skill2_effect() {   ← 메서드로는 적었으나 `if let` 한 겹으로 접어 놓음
  if let Some(e) = champ.ult_effect() {

신(소스 원문, 줄 단위):
  2399  let attack_effect = champ.attack_effect().as_ref();
  2401  if let Some(effect) = attack_effect {
  2407  let skill_effect = champ.skill_effect().as_ref();
  2408  if let Some(effect) = skill_effect {
  2414  let skill2_effect = champ.skill2_effect().as_ref();
  2415  if let Some(effect) = skill2_effect {
  2421  let ult_effect = champ.ult_effect().as_ref();
  2422  if let Some(effect) = ult_effect {
```
**근거 3중**
1. **DWARF 지역변수 타입** — `_gaibc\m10.ll` `!56150`(2399) `!56154`(2407) `!56158`(2414) `!56162`(2421) 의
   변수 이름이 각각 `attack_effect`/`skill_effect`/`skill2_effect`/`ult_effect` 이고 **타입이 `!3973 =
   enum2$<Option<ref$<Effect>>>`** = `Option<&Effect>`. 반면 `!56152`(2401)·`!56156`(2408)·`!56160`(2415)·
   `!56164`(2422) 는 이름 `effect`, **타입 `!3295 = ref$<Effect>`** = `&Effect`.
   ⟹ 슬롯1 은 `Option<&Effect>` 를 **`let` 으로 묶는 줄**이고, 슬롯2 가 `if let` 이다(한 겹이 아니라 두 줄).
2. **`Option::as_ref` 인라인 프레임이 슬롯1 줄에 붙는다** — `m10.ll:51933~51937` 의 dbg 체인 =
   `as_ref<Effect>@option.rs:742 <- max_range_nearly_can_use@battle.rs:2399`, 읽는 오프셋이
   `Entity+0x490`(attack_effect) 과 `Entity+0x4c0`(니치). 2407/2414/2421 도 동일.
   2414/2421 은 그 앞에 `skill2_effect@entity.rs:1693` / `ult_effect@entity.rs:1701` 프레임이 **한 겹 더** 있다
   (레벨 게이트 때문에 본문이 비지 않아 프레임이 남았다). 2399/2407 은 본문이 `&self.<field>` 한 줄이라
   인라인 후 명령이 0개가 되어 프레임이 접혔다 — **"프레임이 없다 = 메서드가 아니다"가 아니다.**
3. **tcx 정본에 네 메서드가 전부 있다** (`spec3lib.py fn`):
   `game_core::Entity::attack_effect` entity.rs:**1684** / `skill_effect` **1688** / `skill2_effect` **1692** /
   `ult_effect` **1700** — 넷 다 `pub fn(&Entity) -> &Option<Effect>` · mir=True · xinl=True.
4. **줄 길이 산술이 ±0 로 확정한다**(`rmeta_srcmap game_ai "plan_legacy\old\battle.rs" 2393 2432`,
   ⚠도구 출력 `bytes` 는 개행 1B 를 포함하므로 **본문 길이 = bytes − 1**):

| 슬롯 | 줄 | 실측 본문길이 | 복원 텍스트 | 계산 |
|---|---|---|---|---|
| 1 | 2399/2407/2414/2421 | 53·51·53·47 | `  let N = champ.N().as_ref();` | 27+2n (n=13·12·13·10) |
| 2 | 2401/2408/2415/2422 | 39·38·39·36 | `  if let Some(effect) = N {` | 26+n |
| 3 | 2402/2409/2416/2423 | 78·77·78·75 | `    if effect.target.check(champ, target) && champ.<cd>() <= tick {` | 62+len(cd) |
| 4 | 2403/2410/2417/2424 | **131 (4줄 동일)** | `      range = range.max(effect.range(champ) + effect.range_adjust(champ, target) + champ.radius() as u64 + target.radius() as u64);` | 상수 |
| 5 | 2404/2411/2418/2425 | 5 | `    }` | |
| 6 | 2405/2412/2419/2426 | 3 | `  }` | |

⟹ **3차가 남긴 "슬롯1 마지막 1자 모순"은 해소된다**(3차의 후보 `if let Some(N) = &champ.N {` 는 길이는 같지만
`Option<&Effect>` 타입·`as_ref` 프레임·2401 의 별도 `if let` 을 설명하지 못한다).
★슬롯4 는 `Entity::radius` 반환이 **`usize`** 이고 `range` 지역변수가 **`u64`**(DWARF `!56148` type `!121 = u64`)라
`as u64` 두 번이 **타입상 강제**되며, 그렇게 하면 131 에 ±0 로 맞는다(`as u64` 없는 형태는 117).
★슬롯3 의 `&&` 피연산자 순서는 **CFG 로 확정**된다 — `check` 를 먼저 부르고(m05… 아니라 m10.ll:52193)
그 다음 블록에서 쿨다운을 읽으므로 단축평가가 보존돼 있다(브리핑 함정 §6).
⚠**표기 한계(적용 범위)**: `<= tick` vs `tick >=` 방향과 슬롯4 의 `range.max(..)` vs `max(range, ..)` 는
길이가 같아 **줄 길이로는 못 가른다**. 여기 적은 것은 IR 비교 방향(`icmp ugt cooldown, tick` 의 부정)과
`llvm.umax` 에 맞춘 표기다.

★부수 정정 — `logic` 의 "주의 1) 평타 결과에는 max 가 없다" 는 **컴파일러 접힘 설명으로는 맞지만
소스에는 네 줄 다 `range.max(...)` 가 있다**(131 이 4줄 완전 동일). "소스에 max 가 없다"로 읽히지 않게 고칠 것.

### E2 `/specs[16]/callees` — 게임 함수 4개 누락 ★실오류 (자동생성이 `logic` 오류를 그대로 받아간다)
```
추가해야 하는 것:
  game_core::Entity::attack_effect    fn(&Entity) -> &Option<Effect>   entity.rs:1684  pub mir=T xinl=T
  game_core::Entity::skill_effect     fn(&Entity) -> &Option<Effect>   entity.rs:1688  pub mir=T xinl=T
  game_core::Entity::skill2_cooldown  fn(&Entity) -> usize             entity.rs:1789  pub mir=T xinl=T
  game_core::Entity::ult_cooldown     fn(&Entity) -> usize             entity.rs:1804  pub mir=T xinl=T
근거: DWARF 인라인 프레임(m10.ll !56212 skill2_effect / !56226 ult_effect / !56217 skill2_cooldown /
      !56231 ult_cooldown) + as_ref 프레임의 2399·2407 부착 + spec3lib.py fn 조회
```
⚠**구조적 원인**: `callees` 는 `logic` 산문에서 leaf 이름을 긁어 tcx 로 조회한다(`callees_note`).
`logic` 이 `champ.ty.skill2_cooldown /*+0xc0*/` 처럼 **필드 접근으로 적으면 함수로 안 잡힌다.**
⟹ **`logic` 오류가 「자동 생성이라 신뢰할 수 있다」던 `callees` 까지 오염시킨다.** specgate 에
「`mem` 의 `<base>+<off>` 가 실은 인라인된 메서드인지」를 교차검사하는 게이트(G7)를 제안한다.

### E3 `/specs[15]/mem[8]/name` — 3차가 `logic` 에서 고친 오기가 `mem` 에 살아 있다 ★실오류
```
구: name = "ty.Tower.0"   note = "ty 판별자(+0x68)+0xc0 = Tower variant 페이로드 i8 = TowerType"
신: name = "ty.Tower.info.ty"
    note = "EntityType::Tower 는 **struct variant `{ info: Tower }`**(튜플 아님).
            Entity+0x68(ty) → 페이로드 +0x8(info) → Tower+0xb8(ty) = 0x128. ~~ty.Tower.0~~ 은
            3차 배치D P-4 가 `logic` 에서만 고친 튜플 variant 오기의 잔존분"
근거: tcxdict --enum EntityType → `페이로드 Tower … enum+0x8 info game_core::Tower(192B)` (필드명이 `info`, `0` 아님)
      tcxdict Tower → `0xb8 ty game_core::TowerType(1B)`
```
★**이것이 이번 라운드 1순위 과제(G6 사각지대)의 실물이다.** G6 는 `mem/consts/knobs` 의 `~~취소선~~` 이
`logic` 에 남았는지만 본다 — **`logic` 이 고쳐지고 `mem` 이 안 고쳐진 역방향은 못 잡는다.**
게이트 제안(G8): 3차/4차 패치 스크립트가 `logic` 만 `rep()` 한 항목에 대해 같은 문자열이
`mem[].name`/`note` 에 남아 있는지 역검사.

한 행 = 한 오프셋으로 다시 적으면:
- `Entity+0x68` = `ty` (EntityType, 480B)
- `Tower+0xb8` = `ty` (TowerType, 1B)
- `Entity+0x128` = `ty.Tower.info.ty` (TowerType, 1B)

### E4 `/specs[17]/logic` — memset 그룹 필드 개수가 8이 아니라 9다 ★실오류(값)
```
구: // 틱·누적 계열 8필드 = memset(0x120, 0, 80) 로 일괄 0:
신: // 틱·누적 계열 9필드 = memset(0x120, 0, 80) 로 일괄 0:
근거: 열거된 이름이 9개(trade_lean/lean_last_tick/scene_change_tick/last_act_tick/idle_spec_tick/
      last_swing_tick/ep_follow_until/idle_prev_pos/last_unseal_tick)이고 `mem` 에도 9행이 있다.
      바이트 검산: 8×8B + idle_prev_pos 16B = 80B = memset 길이 ⟹ 9필드가 정답.
      (`DeathMatchBattle+0x120` ~ `DeathMatchBattle+0x168`, tcxdict DeathMatchBattle 로 전량 일치 확인)
```
★같은 문단의 "필드 36개 전부 초기화"는 **맞다**(tcxdict = 36필드, logic 열거 합계도 36).

### E5 `/specs[18]/mem[15]` — 한 행에 오프셋 2개 ★실오류(기계 감사 무력화)
```
구: { base: "TeamPlan", offset: "0xc8/0xd0", name: "chats(push)", dir: "w" }
신: 2행으로 분리
    { base:"TeamPlan", offset:"0xc8", name:"chats.ptr (push 대상 슬롯 계산)", dir:"w" }
    { base:"TeamPlan", offset:"0xd0", name:"chats.len (읽고 +1 해서 되쓴다)", dir:"w" }
근거: meta.corrections 가 2026-09-11 에 **14 의 `0x18/0x20` 묶음 행을 같은 이유로 2행 분리**했는데
      18 의 이 행은 남았다. 브리핑 §5 「한 행에 오프셋을 묶지 마라」 위반.
      `TeamPlan+0xc0` = chats(Vec<Chat>, 24B) 이므로 0xc8·0xd0 은 그 내부 워드다(tcxdict TeamPlan).
```

### E6 `/specs[18]/open[2]` — 3차 P-12 가 적용되지 않았다 (과열림)
```
open[2].q = "Chat::Repair / SerpenSetup / Press / PressChange 가 공통으로 갖는 usize 필드(enum+0x8) …
             dienum 이 준 이름이 __0/__1 뿐이라 그 0 의 의미 확정 불가"
→ `/specs[17]/closed[6]` 가 **같은 사안을 이미 닫았다**("Chat 은 game-core 정의 · 튜플 variant 라
  이름이 애초에 존재할 수 없다 · 값은 항상 리터럴 0 · 소비처 0건 = 죽은 슬롯").
  3차 리포트 P-12 가 이 이동을 제안했으나 `patch3.py`·`closelist.py` 에 반영되지 않았다
  (P-13/P-14 는 반영돼 open 5→3 이 됐는데 P-12 만 빠졌다).
⟹ closed 로 이동 + why = "17 closed[6] 과 동상 — 튜플 variant 라 이름 부재, 소비처 0건(죽은 슬롯).
   적용 범위 = `_gaibc`·`_gcbc` 전수 스캔"
부수 확증(4차): tcxdict --enum Chat →
  `Chat::Repair` 페이로드 = `enum+0x8` usize 1개 · `Chat::SerpenSetup` 도 `enum+0x8` usize 1개 ·
  `Chat::Press`/`PressChange` 는 `enum+0x1` LineType + `enum+0x8` usize
  ⟹ v3 가 "공통으로 갖는 usize 필드(enum+0x8)"라고 쓴 것은 **정확하다**(Repair/SerpenSetup 은 `__0`,
     Press/PressChange 는 `__1` 이라 이름만 다르고 오프셋은 같다).
```

---

## 2. ★판정반전 3건

### R1 `/specs[15]` `single_tower_dive_is_viable` — 「9축 전부 무관」은 **오라클 캐시 아티팩트**였다
3차가 `closed[3]`/`history[8]`/§4 표에 "RNG·타워hp·**대상hp**·공격력·방어력·사거리·거리·아군수·배치
**9축을 실측 배제**"로 남겼다. **대상 hp 는 판별 축이다.**

**케이스당 프로세스 1개**로 다시 재면(`D4_o2.exe <case>`, `o2.txt`):
```
o11_1999  stack=true  thp=1999  twr_hp=100  viable=true   kdt_me=120000  kdt_tgt=119940
o11_2000  stack=true  thp=2000  twr_hp=100  viable=false  kdt_me=120000  kdt_tgt=120000
o11_2001  stack=true  thp=2001  twr_hp=100  viable=false  kdt_me=120000  kdt_tgt=120060
```
`kdt_tgt = target.hp × 60`(tps) 로 선형이고 **경계는 `kdt_tgt < 120000` ⟹ target hp 1999/2000**.
3차가 무관으로 본 이유 2개:
1. **TLS 캐시**(→N3). `D3_o10.rs`·`D3_o11.rs` 는 한 프로세스에서 `viable!()` 매크로를 세계를 바꿔가며
   수십 번 호출한다 — 첫 호출의 답이 그대로 재생된다. 4차 실측(`o2.txt`):
   `seq`(o11 먼저) → o10 세계가 **true**(정답 false) / `rseq`(o10 먼저) → o11 세계가 **false**(정답 true).
2. 3차 o10 세계는 target hp 가 이미 2000 이라 `kdt_tgt` 가 **천장(120000)에 붙어 있었다** ⟹ 타워 hp 를
   1~2,000,000 으로 흔들어도 판별력이 0. **타워 hp 무관은 그 세계에 한해 참**이다(4차 별프로세스 재확인:
   `o10_1`/`o10_1000`/`o10_100000`/`o10_2000000` 전부 false).

⟹ `closed[3]`·`history[8]`·§4 표의 "9축 배제"를 **"그 세계(target hp 2000)에서만 배제 · 그리고
그 측정은 캐시 오염 가능"** 으로 정정하고, **target hp 를 축으로 등재**해야 한다.

### R2 `/specs[17]` `base_sub_goal` — 「갈림은 가시성이 아니다 / 거리 임계 1e6 이하 없음」의 범위 정정
3차 `history[3]`: "★갈림은 **가시성이 아니다** — `visible_state`·`can_target`·`invisible_tick`·`hp=0`·
`visible_map`·`exist_map` **6축을 실측 배제**했다. 남은 미탐색 = 그 판별 축." + "거리 임계는 1,000,000
이하에 없음 ⟹ 거리 아닌 다른 양일 가능성".

**IR 로 축이 확정된다**(`_gaibc\m10.ll:29294~29372`, `base_sub_goal@battle.rs:70~93`):
```
70  match self  (tag < 2 = TryKill|Support  /  tag >= 2 = Response|Avoid)
71  let target = self.<TryKill|Support>.__0                      // goal+0x8
72  if game.get_entity_by_id(target)          // vtable +0x1f0 (dyn AbstractGame)
73       .is_some_and(|e| is_ignored_well_enemy(version, player, e))  { End }
    else { Trace { focus: target } }
```
`is_ignored_well_enemy`(**pub**, `fight_model.rs:754`, `fn(usize, &PlayerState, &Entity) -> bool`)의 본문:
- `fight_model.rs:755` — `PlayerState+0x930`(info.team) 을 읽어 `1 - team` 을 만들고
  `entity.team`(**`Entity+0x0`**, `TeamType` 16B: 판별자 +0x0 / 팀 인덱스 +0x8)과 `PartialEq::eq`(entity.rs:1127) 비교
  ⟹ **"그 엔티티가 적 팀인가"**
- `fight_model.rs:756` — `path_finder::is_enemy_well_danger(version, player, e.x, e.y)`
  (**pub**, `path_finder.rs:1032`, `fn(usize, &PlayerState, u64, u64) -> bool`) · 좌표는 `Entity+0x660`/`Entity+0x668`
  ⟹ **"그 좌표가 적 우물(well) 위험 구역인가"**

⟹ **축 = 「target 이 적 팀 챔프이고 그 좌표가 적 우물 위험 구역에 있다」** ⟹ `End`. 아니면 `Trace`.
3차의 관측 3개가 전부 이 하나로 설명된다:
- 「타워·존재하지 않는 id → 거리 무관 Trace」 = 팀 비교에서 탈락(타워의 `team` 이 적팀 챔프 조건을 못 맞춘다) ✓
- 「가시성 6축 무관」 = **엔티티/월드 쪽 가시성이 아니라 좌표-우물 판정**이므로 당연히 안 움직인다 ✓
  ⟹ 판정 어휘를 "가시성이 아니다"(무범위)에서 **"`visible_state`/`can_target`/`invisible_tick`/`hp`/
     `visible_map`/`exist_map` 6축이 아니다"** 로 좁힐 것.
- 「초기 스폰 배치에서만 End」 = 스폰 지점이 자기 진영 우물이라 `is_enemy_well_danger` 가 true ⟹
  **거리처럼 보였을 뿐 실제로는 위치-우물 관계**다. "거리 임계가 1e6 이하에 없다"는 참이지만
  **"거리 아닌 다른 양"의 정체가 우물 구역**이다.

부수: 같은 함수의 **적 챔프 스캔**(battle.rs:81~83)도 축이 확정된다 —
`iter_champions(1 - team)` 를 `Blackboard::is_recent_visible`(**pub**, `blackboard.rs:346`,
`fn(&Blackboard, &dyn AbstractGame, &PlayerState, &Entity) -> bool`, battle.rs:82)로 거르고
**제곱거리 `< 40000000001`**(= 거리 200000, battle.rs:83, m10.ll:29533)로 한 번 더 거른다.
⟹ 3차가 "가시성"으로 흔든 6축은 전부 **엔티티/월드** 쪽이고, 이 함수가 실제로 보는 가시성은
**블랙보드(팀 기억)** 다. 3차 프로브는 `Blackboard::default()` 를 넘겼으므로 이 술어가 상수였다.
(★신규 상수 후보 = `/specs[17]/consts` 에 `40000000001` (`kind: 임계`, `src_line: 83`, 의미 = 제곱거리 200000²) 추가 제안)

### R3 METHOD_MAP ⑥ 한계1 — 「`fight_check`·`path_finder` 는 `pub(crate)` 라 막힌다」가 과잉이다
`_tcx\pubapi_game_ai.txt` 실측:
```
232: game_ai::check_kill_die_tick        (fight_check.rs:917)   pub   ← 캐시된 진입점
     game_ai::is_enemy_well_danger      (path_finder.rs:1032)  pub
     game_ai::plan_legacy::old::is_ignored_well_enemy (fight_model.rs:754) pub
반면 game_ai::fight_check::check_kill_die_tick_uncached (rs:970) 는 in:game_ai::fight_check (막힘)
```
`fight_check` 모듈 아이템 128개 중 `pub` 은 5개지만, **크레이트 루트로 재수출된 진입점은 `pub`** 이다
(모듈 경로가 `game_ai::check_kill_die_tick` 이라 "fight_check" 로 grep 하면 안 보인다).
⟹ METHOD_MAP ⑥의 한계 목록을 **"모듈 전체가 막힌 게 아니라 `*_uncached` 같은 내부 함수만 막힌다.
재수출 여부는 `pubapi_*.txt` 로 확인하라"** 로 고칠 것. 4차는 실제로 `check_kill_die_tick` 를
프로브에서 직접 호출해 **링크·실행 성공**했다(`o1.txt`·`o2.txt` 의 `kdt_me`/`kdt_tgt` 칼럼).

---

## 3. 신규확정 3건

### N1 `battle.rs:2397~2429` 전 33줄 복원 (16, ±0)
```
2397  pub fn max_range_nearly_can_use(champ: &Entity, target: &Entity, tick: usize) -> u64 {   (86)
2398    let mut range = 0;                                                                      (20)
2399    let attack_effect = champ.attack_effect().as_ref();                                      (53)
2400                                                                                            (0)
2401    if let Some(effect) = attack_effect {                                                    (39)
2402      if effect.target.check(champ, target) && champ.attack_cooldown() <= tick {             (78)
2403        range = range.max(effect.range(champ) + effect.range_adjust(champ, target)
                              + champ.radius() as u64 + target.radius() as u64);                (131, 한 줄)
2404      }                                                                                      (5)
2405    }                                                                                        (3)
2406                                                                                            (0)
2407~2412  skill  블록 (2407=51 · 2408=38 · 2409=77 · 2410=131 · 2411=5 · 2412=3)
2413       (0)
2414~2419  skill2 블록 (54-1=53 · 39 · 78 · 131 · 5 · 3)
2420       (0)
2421~2426  ult    블록 (47 · 36 · 75 · 131 · 5 · 3)
2427                                                                                            (0)
2428    range                                                                                    (7)
2429  }                                                                                          (1)
```
- **함수 선언은 indent 0, 본문 indent 2**(2398 = 20자 ⟹ `  let mut range = 0;`).
- ⚠**3차의 DWARF 어휘 함정**: `!56151`(2399) → `!56155`(2407) → `!56159`(2414) → `!56163`(2421) 의
  `DILexicalBlock.scope` 가 **앞 블록을 부모로 갖는다**. 이것을 소스 중첩으로 읽으면 4겹 중첩이 되는데,
  **닫는 괄호 길이(각 블록이 `    }`+`  }` 로 끝나고 2428 이 indent 2)가 4블록은 형제임을 확정한다.**
  rustc 의 source-scope 는 같은 블록 안 `let` 이후 문장을 자식 스코프로 넣기 때문에 생기는 아티팩트다
  (→ METHOD_MAP 에 적을 새 함정).

### N2 `single_tower_dive_is_viable` 본체 전량 + 판별식 (15)
`_gaibc\m05.ll:44249~44820`(`single_battle.rs:891~945`). ★3차는 `_gaibc` 에 define 이 있는지 보지 않고
오라클 이분탐색만 했다.
```
891  pub fn single_tower_dive_is_viable(version, rnd, player, data, team_plan, target, debug) -> bool
900   let champ = data.cache.player_champion[player.info.team][player.info.position.as_index()];
      // PlayerState+0x930 (team, <2 경계검사) · PlayerState+0x9c0 (position) · AbstractGameWithCache+0x1e0
      if champ.is_none() { return false }                        // ★유일한 조기 탈출
904   let near_enemies = data.cache.iter_champions(1 - team).filter(closure$0).collect_in(pool)
909     // pool = data.context(+0x8).pool(+0x0)
912   let near_allies  = (0..5).filter(closure$1).filter_map(closure$2).collect_in(pool)
922   let dive_tower = data.cache.iter_towers_without_nexus(..)
924       .min_by_key(|t| t.distance_sq(target))                 // closure$3
925       .filter(|t| t.distance(target) <= t.attack_effect.unwrap().range(t) + 15000
                                            + t.radius() + target.radius())   // closure$4
930   let n = near_enemies.len().div_ceil(near_allies.len().max(1))
                                                                 // umax(len,1) → (n-1+len)/n
932   let my_threats = near_enemies.iter().copied().take(n).collect_in(pool)
936   let die_me  = check_kill_die_tick(version, rnd, data, player, champ,  my_threats,
                                        Vec::from_iter_in(dive_tower, pool), debug)
938   let die_tgt = check_kill_die_tick(version, rnd, data, player, target, near_allies.clone(),
                                        Vec::new_in(pool), debug)
944   die_tgt < die_me                                           // ★반환
```
- **신규 상수 `15000`** (m05.ll:44670, closure$4 의 사거리 여유. champion_radius 10000 의 1.5배) —
  `/specs[15]/consts` 에 추가 제안(`kind: 임계`, `src_line: 925`).
- **신규 상수 `1`**(m05.ll:44696 `llvm.umax.i64(len, 1)` = `near_allies.len().max(1)` 0除算 가드, rs:930).
- ⚠`closure$4` 는 `dive_tower.attack_effect` 가 None 이면 **`Option::unwrap` 패닉**(m05.ll:44618,
  `option.rs:1013`). 타워는 항상 평타 이펙트를 갖는다는 전제가 코드에 박혀 있다.
- **game==mine 12/12**: 내 재현 `die_tgt < die_me` 를 프로브에서 두 번의 `check_kill_die_tick` 호출로
  직접 계산해 `single_tower_dive_is_viable` 의 반환과 대조 — `o1.txt` 4행 + `o2.txt` 8행 **전부 일치**.
  ⚠**적용 범위**: 내 프로브는 `near_enemies`/`near_allies`/`my_threats` 를 각각 1원소로 **단순화**한
  세계에서만 대조했다(closure$0~$2 의 술어는 읽지 않았다). 반환식(rs:944)과 두 호출의 인자 역할은
  IR 로 확정이고, **집합 구성 술어 3개는 여전히 미탐색**이다.
- 오프셋 한 행 = 한 오프셋:
  - `PlayerState+0x930` = `info.team` (usize)
  - `PlayerState+0x9c0` = `info.position` (Position, i32)
  - `AbstractGameWithCache+0x1e0` = `player_champion` ([[Option<&Entity>;5];2])
  - `Entity+0x4a0` = `attack_effect.range` (u64)
  - `Entity+0x4a8` = `attack_effect.growth_range` (u64)
  - `Entity+0x4c0` = `attack_effect.casting` (CastingType, Option 니치)
  - `Entity+0x438` = `stat_buff_cached.range` (usize)
  - `Entity+0x470` = `stat_buff_cached.radius_mult` (i32)
  - `Entity+0x5c8` = `level` (usize)
  - `Entity+0x680` = `radius` (usize)
  - `Entity+0x660` = `x` (u64)
  - `Entity+0x668` = `y` (u64)

### N3 ★오라클 함정 신규 — `check_kill_die_tick` 는 **TLS 메모**다 (TEMPLATE 9번째 함정)
`_gaibc\m15.ll:27042~28034`(`check_kill_die_tick`) 안에
`std::thread::LocalKey<core::cell::RefCell<game_ai::fight_check::DieTickCache>>::with` 호출 2개 +
미스 경로 `check_kill_die_tick_uncached`.
캐시 키 = `game_ai::fight_check::DieTickKey`(192B, `tcxdict`):
- `DieTickKey+0x0` = `enemy` ([usize; 8])
- `DieTickKey+0x40` = `tower` ([usize; 12])
- `DieTickKey+0xa0` = `version` (usize)
- `DieTickKey+0xa8` = `judger` (usize)
- `DieTickKey+0xb0` = `focus` (usize)
- `DieTickKey+0xb8` = `enemy_len` (u8)
- `DieTickKey+0xb9` = `tower_len` (u8)

★**키에 hp·스탯·좌표가 하나도 없다 — 전부 엔티티 id 다.** 오라클에서 `Game` 을 새로 만들어도 엔티티 id 는
같으므로 **키가 반복되고 스탯을 바꾼 두 번째 세계가 첫 세계의 답을 받는다.** 실측(`o2.txt`):
```
o11_50        (단독 실행)  viable=true    ← 정답
o10_0         (단독 실행)  viable=false   ← 정답
seq1_o11 → seq2_o10 → seq3_o10again      : true / **true** / **true**   ← 2·3행이 오염
rseq1_o10 → rseq2_o11                    : false / **false**           ← 2행이 오염
```
⟹ **`check_kill_die_tick` 를 (직간접으로) 타는 함수는 오라클에서 케이스당 프로세스를 새로 띄워야 한다.**
간접 경유 함수 = `single_tower_dive_is_viable` · `SinglePlanBattle::update`(교전 판단) 등 fight 계열 전부.
`_verify3\TEMPLATE.rs` 의 함정 목록에 추가하고, 프로브는 `main` 에서 **인자로 케이스 1개만** 재는 형태로
쓰는 것을 정본화할 것(`_verify4\D\body_o2.rs` 가 참조 구현).

⚠**3차 앵커의 신뢰도 유보**: `_verify3\D\D3_o10.rs`·`D3_o11.rs` 는 `real_setting()` 이 아니라
`Default::default()` + `tick_per_second=60` 만 세팅한다 ⟹ width/height/champion_radius/visible_distance 가 0
(= 3차 자신이 정본화한 TEMPLATE 함정 ①). 4차는 `real_setting()` 으로 재실행해 단독 실행 시 결론이
같음을 확인했지만(`o2.txt` `o10_0`=false · `o11_50`=true), **그 두 파일을 "재현 앵커"로 인용할 때는
캐시 오염 + 세팅 오염 두 유보를 함께 적어야 한다.**

---

## 4. 오류 없음으로 확인한 것

### `ev<=3` 전수 재확인 — 뒤집힘 0건
| 항목 | 재확인 방법 | 결과 |
|---|---|---|
| `/specs[15]/sig/tcx` | `spec3lib.py fn single_try_engage` | 문자 단위 일치 ✓ (`vis=in:game_ai`, modes.rs:241, mir=False) |
| `/specs[16]/sig/tcx` | `spec3lib.py fn max_range_nearly_can_use` | 일치 ✓ (`pub`, battle.rs:2397) |
| `/specs[17]/sig/tcx` | `spec3lib.py fn new` | 일치 ✓ (`pub`, death_battle.rs:746) |
| `/specs[18]/sig/tcx` | `spec3lib.py fn v3_epicops_buff_window` | 일치 ✓ (`in:game_ai`, epic.rs:634) |
| `/specs[19]/sig/tcx` | `spec3lib.py fn best_jungle_goal` | 일치 ✓ (`pub`, passive_jungle.rs:806) |
| `/specs[15]/sig/params[1]` (self 공유) | tcx sig 첫 인자가 `&LegacyPlanHandler` | ✓ |
| `/specs[15]/mem[5]` `LegacyPlanHandler+0xf8` | `tcxdict LegacyPlanHandler` | `0xf8 team_plan TeamPlan(1064B)` ✓ |
| `/specs[17]/mem[41]` `DeathMatchBattle+0x17b` | `tcxdict DeathMatchBattle` | `0x17b main_objective Option<MainObjective>(3B)` ✓ |
| `/specs[19]/mem[7]` `MobaMode+0x18` | `tcxdict MobaMode` | `0x18 jungle_runner JungleRunner(480B)` ✓ (`chk` 는 3차 P-10 대로 `OK` 로 갱신돼 있다) |

### `logic` ↔ 정본 눈 대조 — 위 E1~E5 외에 어긋남 없음
전수 검산한 것(전부 `tcxdict` 정본과 ±0):
- **16 의 14-way 쿨다운 switch 12개 오프셋 전량**: `Entity+0x68`(ty) 페이로드 시작 `+0x8` 기준
  Minion 0x48→`Entity+0xb8` · Tower 0xa0→`Entity+0x110` · Jungle 0x78→`Entity+0xe8` ·
  Epic/Serpen 0x180→`Entity+0x1f0` · Ghoul 0x78→`Entity+0xe8` · SmallJiangshi 0x40→`Entity+0xb0` ·
  Bear 0x58→`Entity+0xc8` · Eagle 0x80→`Entity+0xf0` · Revenant 0x68→`Entity+0xd8` ·
  Illusion 0x60→`Entity+0xd0` · Champion 0x40/0x48/0x50/0x58→`Entity+0xb0/0xb8/0xc0/0xc8` **12/12 일치**
- **16 의 Effect 4필드**: `Effect+0x10` range · `Effect+0x18` growth_range · `Effect+0x28` target ·
  `Effect+0x30` casting, 그리고 4슬롯 베이스 `Entity+0x490`/`Entity+0x4c8`/`Entity+0x500`/`Entity+0x538`
  에 더한 16개 파생 오프셋 **전부 일치**. `BuffState+0xc8`=range · `BuffState+0x100`=radius_mult,
  `Entity+0x370`=stat_buff_cached ⟹ `Entity+0x438`/`Entity+0x470` **일치**
- **17 의 `DeathMatchBattle` 36필드 전량** (0x0~0x17e) `tcxdict` 와 **완전 일치**
- **18 의 `TeamPlan` 4필드**: `TeamPlan+0xc0` chats · `TeamPlan+0x410` eo_serpen_punish_issues ·
  `TeamPlan+0x41e` v3_press_chat_line(Option<LineType> 1B) · `TeamPlan+0x41f` objective
  (Option<MainObjective> 3B) — 그리고 `MainObjective` 태그 7=Repair · 1=Serpen(페이로드
  `enum+0x1` phase · `enum+0x2` with_battle) **전부 일치**
- **18 의 Chat 태그 4개**: 21 Press · 22 PressChange · 23 Repair · 25 SerpenSetup **일치**(`tcxdict --enum Chat`)
- **19 전량**: `GameContext+0x0` pool · `GameContext+0x20` map · `JungleCampState+0x18`
  next_respawn_tick · `MobaMode+0x18` jungle_runner · `PlayerState+0x930`/`PlayerState+0x9c0` ·
  `Entity+0x660`/`Entity+0x668` · `AbstractGameWithCache+0x1e0` — **일치**.
  `JungleType` 선언순서 Rhino/Mushroom/**Stump**/Bee 와 `jungle_camps` 배열순서
  Rhino(0)/Mushroom(1)/Bee(3)/Stump(2) 가 다르다는 v3 의 지적도 **정확**
- **17 `BattlePlanGoal`**(24B, 태그 +0x0 범위 0..4, TryKill 페이로드 `enum+0x8`/`enum+0x10`) ·
  **15 `BattleSubPlanGoal`**(16B, 0=Trace 1=Protect 2=Kiting 3=KitingBack 4=RunAway 5=Assassin
  6=AssassinReady 7=End, 페이로드 `enum+0x8` = `focus`) — v3 의 태그 3/4/7 설명 **일치**

### `callees_unmatched` — 판정 술어 혼입 0건 (3차와 동일)
15 (없음) / 16 `casting`(=`Effect+0x30` 필드) · `llvm.umax.i64`(인트린식) / 17·18 `grow_one`(std) / 19 (없음).

### 표기 부류(브리핑 §0 기준 **오류 아님**) — 기록만 남김
- `/specs[16]/consts[2]/meaning` "level>2" · `consts[3]` "level>4" — 2차가 `logic` 을 MIR 정본
  `>=3`/`>=5` 로 고쳤는데 `consts` 는 옛 표기다(`mem[24]`/`mem[25]`/`mem[26]` note 도 동일).
  **정수 비교라 값은 동등**하므로 오류로 세지 않는다. 단 **G6 가 못 잡는 같은 부류**(역방향 미반영)라
  `~~level>2~~ → level>=3` 형태로 통일해 두면 다음 라운드가 다시 안 본다.
- `/specs[16]/closed[0]/q` 의 "`range_adjust` … -> **i64**" — tcx 정본은 **u64**
  (`game_core::Effect::range_adjust : fn(&Effect,&Entity,&Entity) -> u64`). `closed[].q` 는 당시의
  질문 원문이고 `callees` 에는 u64 로 정확히 들어 있으므로 정본은 무오류. 다만 `q` 를 grep 하는 세션이
  오독할 수 있으니 `~~i64~~ → u64` 를 권한다.

---

## 5. 「16 의 open 이 0 인 것이 맞는가」 — 지시 항목에 대한 답

**맞지 않다. 최소 1건은 다시 열어야 하고, 그 1건은 이번에 닫혔다.**
- `/specs[16]/closed[5]/why` 가 "3차 배치D: 줄 길이 산술로 4블록 동시 일치 복원(재료 부재 아님)" 로
  **완전 해소**를 주장하는데, 3차 리포트 본문과 `/specs[16]/history[7]` 은 **"⚠남은 모순(미탐색):
  슬롯1 정합은 필드 접근을 가리키는데 IR 은 메서드 인라인을 말한다(1자 차)"** 로 끝난다.
  ⟹ `closed[5]` 는 **과닫힘**이었다(3차 §4 표가 "미탐색"으로 등재한 항목이 `open[]` 에 없다).
- 4차가 그 모순을 **해소**했다(E1/N1). 그러므로 지금은 `closed` 로 두는 것이 맞지만,
  `why` 를 **"4차 배치D: DWARF 지역변수 타입(`Option<&Effect>` vs `&Effect`) + `as_ref` 프레임 +
  `Entity::{attack,skill,skill2,ult}_effect` 4메서드 존재로 슬롯1·2 분리 확정, 33줄 전량 ±0 복원"**
  으로 갱신해야 한다.

★**같은 과닫힘이 2건 더 있다** — 3차 §4 「다음 세션에 남기는 것」 4항목 중 **3개가 `open[]` 에 없다**:
| 3차가 미탐색으로 넘긴 것 | v3 의 현재 위치 | 판정 |
|---|---|---|
| 16 `battle.rs` 2399/2407/2414/2421 마지막 1자 | `closed[5]` | 과닫힘 → 4차에 해소(E1/N1) |
| 15 `single_tower_dive_is_viable` 판별 축 | `closed[3]`(why 가 `is_in_range` 근거만 인용) | **과닫힘** → 4차에 해소(N2) + 축 배제표 반전(R1) |
| 17 `base_sub_goal` 챔프 target 의 Trace↔End 술어 | `closed[2]`·`closed[3]` | **과닫힘** → 4차에 해소(R2) |
| 15 `update` `tick>=121` 의 소스 대응 | `open[1]` 이 포괄 | OK |
⟹ `closelist.py` 가 **3차 §4 표를 `open[]` 으로 승격하지 않았다.** 4차가 세 건 다 닫았으므로 실害는
없지만, **"3차 리포트 §4 표 → 다음 라운드 `open[]`" 승격을 파이프라인에 넣을 것**을 제안한다.

---

## 5-b. 남은 `open[]` 8건 전수 처분 (4차 판정)

| 경로 | 4차 판정 | 근거 |
|---|---|---|
| `/specs[15]/open[0]` version 분기 | **closed 로 이동(과열림)** | `history[7]` 이 오라클로 version 0·1·2·3·4·5·10·30·31·32·33·50·100 전부 동일 확인. 본문 IR 에도 version 비교 0건 |
| `/specs[15]/open[1]` `update` 의 sub_goal 게이트 | **open 유지 · 범위 축소** | 거리 100000/250000·tick 120 게이트는 3차 확정(`history[7]`). 남은 것은 **`SinglePlanBattle::update_v32`(single_battle.rs:345~870, `vis=in:single_battle`) 내부 개별 술어**뿐. ⚠`check_kill_die_tick` 를 타므로 오라클은 **케이스당 프로세스 1개**(N3) |
| `/specs[16]` open 0건 | **1건은 과닫힘이었고 4차에 해소** | §5 참조. `closed[5]/why` 갱신 필요 |
| `/specs[17]/open[0]` AbstractGame vtable 슬롯의 구현체 의존성 | ★**closed — 실측으로 닫힘** | 아래 vtable 전수표 |
| `/specs[18]/open[0]` goal_data·plan 의 판정 사용 필드 | **closed 로 이동(과열림)** | `/specs[18]/history[4]` 가 같은 질문을 이미 답했다 — `goal_data` 는 `GoalData+0xc0`(serpen.epic_enemy_tick) **1개뿐**, `plan` 은 `BigPlan::goal()` 판별자 태그 하나 |
| `/specs[18]/open[1]` version 분기 | **closed 로 이동(과열림)** | `history[2]` = "버전별 분기표는 존재하지 않는다" + 3차 오라클 14칸(version 7종 × team 2) 전부 동일 |
| `/specs[18]/open[2]` Chat `enum+0x8` 의 0 | **closed 로 이동(3차 P-12 미적용)** | E6 |
| `/specs[19]/open[0]` map 클로저 | **closed 로 이동(과열림)** | `q` 본문이 스스로 "unknown 이 아니라 사실로 logic 에 적었다"고 말한다(3차 P-14 와 같은 부류) |
| `/specs[19]/open[1]` 배열 길이 4·2 를 consts 에서 뺀 것 | **closed 로 이동(판정값 아님)** | `SPEC_GUIDE §3` 규칙 준수 서술. `/specs[15]/closed[5]`·`/specs[17]/closed[5]` 와 동상 |

⟹ **4차 후 배치 D 의 진짜 `open` 은 2건**(`/specs[15]/open[1]` 축소판 + `single_tower_dive_is_viable`
클로저 3개 신규 open). 나머지 6건은 과열림이거나 4차에 닫혔다.

### `/specs[17]/open[0]` 을 닫는 근거 — `AbstractGame` 정적 vtable 전수 대조
`AbstractGame` 구현체는 **정확히 3개**다(`_tcx` 3크레이트 전량 스캔: `game_core::Game` ·
`game_core::SingleLaneGame` · `game_core::DeathMatchGame`). ⚠v3 의 note 가 든 `ExpectedGame` 은
**tcx 에 존재하지 않는 이름**이다(divtable.py 쪽 라벨로 보인다 — note 를 실제 구현체 이름으로 고칠 것).
`_gcbc\g15.ll` 의 세 정적 vtable 전역을 파싱해 슬롯을 바이트 오프셋으로 매겼다
(`@anon.85885fe0…` 계열, 레이아웃 = `ptr drop` + `[16 x i8]`(size,align) + `ptr` × 98 = **816B / 101슬롯**,
18 의 `mem[4]` note 가 인용한 `dereferenceable(816)` 과 일치):

| vtable 오프셋 | Game | SingleLaneGame | DeathMatchGame |
|---|---|---|---|
| `+0x28` | `tick` | `tick` | `tick` |
| `+0x40` | `get_game_mode` | `get_game_mode` | `get_game_mode` |
| `+0xe8` | `is_solorank` | `is_solorank` | `is_solorank` |
| `+0x108` | `strategy` | `strategy` | `strategy` |
| `+0x180` | `set_strategy` | `set_strategy` | `set_strategy` |
| `+0x1f0` | `get_entity_by_id` | `get_entity_by_id` | `get_entity_by_id` |

**18/18 일치.** Rust 의 trait-object vtable 레이아웃은 **트레잇 단위**(메서드 순서가 트레잇 선언 순서로 고정,
구현체마다 함수 포인터만 다름)이므로 이는 언어 보장이기도 하다.
⟹ 배치 D 의 `chk = "확인불가(vtable 슬롯)"` 3건(`/specs[15]/mem[3]` `+0x1f0` ·
`/specs[17]/mem[5]` `+0x28` · `/specs[19]/mem[6]` `+0x40`)을
**`"3구현체 정적 vtable 전수 일치(_gcbc\g15.ll, 816B/101슬롯)"`** 로 갱신 제안(ev 4→3).
재현 스크립트 = `_verify4\D\vtab.txt` 를 만든 인라인 파서(보고서 §6 절차와 같은 방식).

---

## 6. 도구 게이트

```
python -X utf8 specgate.py
   G1 자기모순=0  G2 호출부 전수=0  G3 형제 함수=0  G4 술어 시그니처=0
   G5 sig 정본 대조=0  G6 logic 미반영=0        총 0건          ← 기준선 유지

python -X utf8 tcxaudit.py                    # 기준선
   총 550건  오귀속=0  밀림=0  부분일치=0  확인불가=18  OK=532
python -X utf8 tcxaudit.py --prose C:\tfm2mods\MIG\_verify4\D\D4_REPORT.md
   총 737건  오귀속=0  밀림=0  부분일치=2  확인불가=18  OK=717      ← 통과(+187건, 오귀속·밀림 0)
```
⚠**부분일치 2건은 둘 다 같은 기존 항목이다** — `specs20.json` 의 `cache+0x8`(팻포인터 vtable 절반,
`cache` 는 `OperationData` 의 필드명이라 `AbstractGameWithCache+0x8` 로 정규화되지 않는다) 와,
**이 보고서가 그 항목을 설명하며 `cache+0x8` 이라는 문자열을 인용한 것을 스캐너가 주장으로 집은 것**.
후자는 내 오프셋 주장이 아니다. ⟹ 다음 세션은 이 2건을 쫓지 말 것(스캐너 특성).
⚠**브리핑 §5 의 기준선 숫자가 실측과 다르다** — 브리핑은 "인자 없이 돌리면 766건 · 부분일치 2 ·
확인불가 18" 이라고 했으나 실측은 **550건 · 부분일치 0 · 확인불가 18** 이다(`--prose` 를 주면
`resolved_2026-09-10.json` 까지 추가로 훑어 735건이 되고, 부분일치 1건은 `specs20.json` 의
`cache+0x8` 팻포인터 기존 항목이다). ⟹ **내 문서가 기여한 185건은 오귀속 0 · 밀림 0 ·
부분일치 0 · 확인불가 0.** 브리핑의 766/2 는 3차 보고서를 `--prose` 로 물린 값으로 보인다.
⚠이 보고서는 브리핑 §5 대로 오프셋을 **`Type+0xNNN` 형태로 한 행 = 한 오프셋**으로 적었다
(표 형식만 쓰면 `--prose` 스캐너가 한 건도 못 잡는다 — 3차 배치D 실측).

---

## 7. 다음 세션에 남기는 것 (판정 어휘 + 적용 범위)

| 대상 | 판정 | 적용 범위 |
|---|---|---|
| `single_tower_dive_is_viable` 의 `closure$0`·`closure$1`·`closure$2` 술어 (near_enemies/near_allies 구성) | **미탐색** | 본체·반환식·closure$3·closure$4 는 4차에 확정(N2). 세 클로저는 `_gaibc\m01.ll:40105`·`m05.ll:62146`·`m05.ll:62237` 에 define 이 있다 — **재료 있음** |
| `check_kill_die_tick` 내부(= die tick 계산) | **미탐색** | `pub` 이라 오라클 직접 호출 가능(4차 실증). `_gaibc\m15.ll:27042~28034` = 캐시 래퍼, 본체는 `check_kill_die_tick_uncached`(rs:970, `in:game_ai::fight_check`). ★캐시 키에 스탯이 없으므로 **케이스당 프로세스 1개** 필수 |
| `16` 슬롯3 의 `<= tick` 방향 · 슬롯4 의 `range.max()` vs `max(range,)` | **표기 불가** | 줄 길이가 동일(길이 산술 한정). IR 의 `icmp ugt`/`llvm.umax` 는 양쪽 표기 모두와 양립 |
| `base_sub_goal` 의 `Response`/`Avoid` 경로(tag>=2)가 `KitingBack(3)`/`RunAway(4)` 로 갈리는 조건 | **미탐색** | 4차는 `TryKill`/`Support`(tag<2) 경로만 확정했다. tag>=2 는 `m10.ll:29374~29997`(battle.rs:80~93)에 있고 `Blackboard::is_recent_visible`+제곱거리 4e10 필터를 탄다 — **재료 있음** |
| 3차 `D3_o*.rs` 앵커 전체 | **보류(신뢰도 유보)** | ①한 프로세스 다중 측정 = `check_kill_die_tick` TLS 캐시 오염 ②`real_setting()` 미적용(width/height/champion_radius=0). 인용 시 두 유보를 함께 적을 것 |
