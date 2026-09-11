# 3차 반증검증 — 배치 D (15~19) 보고서

게임 0.5.8 / SDK `sdk_058` / toolchain `nightly-2026-05-24` / 2026-09-11
작업 파일 = `C:\tfm2mods\MIG\_verify3\D\` (프로브 `D3_o1.rs`~`D3_o11.rs`, 출력 `o1.txt`~`o11.txt`)

---

## 0. 1순위 — 2차 배치 D 오염 재실측 결과

`start_game()` 만 부르는 **깨끗한 프로브**(`D3_o1.rs`, `init_tower`/`init_nexus` 재호출 없음)로 재측정했다.

| 항목 | 2차(오염) | 3차(깨끗) | 판정 |
|---|---|---|---|
| `world.tower_ids.len()` | 32 | **16** | towerchk 결론 재확인 ✓ |
| 팀당 타워 수 | 16 | **8** | ✓ |
| `twin_towers[0]/[1].len()` | 4 / 4 | **2 / 2** | ✓ |
| 팀0 타워 좌표 중복 | 8 | **0** | ✓ |
| `iter_towers_without_nexus(t)` 원소 수 | 10 | **8** | ✓ |
| `Effect::is_in_range` game==mine | 900/900 | **676/676 (mismatch 0)** | 유지 ✓ |
| `engage_requires_dive` 진리표 | 300쌍 / true 100 | **260쌍 / true 80** | 값만 축소, 결론 동일 ✓ |
| `best_jungle_goal` | 10/10 MATCH | **10/10 MATCH** | ✓ |
| `max_range_nearly_can_use`(타워 시전자) | 10000 | **10000** | ✓ |

★**깨끗한 세계에서 `is_in_range` 재현 검증을 크게 강화했다.** 실물 26개는 대부분 false 로 몰려
판별력이 약하므로(true 26 = 자기쌍뿐), `Entity` 를 복제해 `casting`(4종)·`range`·`growth_range`·
`level`·`radius`·`radius_mult`·거리를 변주한 **합성 엔티티 1,440개**로 전수 대조했다:

```
iir_syn  SUMMARY  n=1440  ok=2,073,600  mismatch=0  game_true=836,256  game_false=1,237,344
eff_range_syn  ok=1440 mismatch=0      (Effect::range   재현식)
ent_radius_syn ok=1440 mismatch=0      (Entity::radius  재현식)
```
⟹ v3 `/specs[15]/history[4]` 의 `is_in_range`·`Effect::range`·`Entity::radius` 재현식은
**true 40% 인 판별력 있는 표본 207만 쌍에서 오류 0**. `casting == Targeting` 게이트도 이 표본에서
실제로 갈렸으므로(비-Targeting 3종 포함) **오라클 확증으로 승격**된다.

★**부수 확증 — `iter_towers_without_nexus` 의 구성/순서가 포인터 동일성으로 확정됐다.**
```
itertowers  team=0  n=8  expect_n=8  PTR_IDENTICAL=true  nexus_in_iter=false
itertowers  team=1  n=8  expect_n=8  PTR_IDENTICAL=true  nexus_in_iter=false
```
`expect` = `[top_tower, mid_tower, bottom_tower, top_tower2, mid_tower2, bottom_tower2][team].flatten()`
+ `twin_towers[team]` — v3 가 IR(`_gcbc/g15.ll:108971`)로 읽은 순서 그대로이며 **넥서스는 실제로 제외**된다.
⟹ `/specs[15]/history[3]` 이 `ev 4`(IR 독해) → **`ev 2`(오라클 실행)** 로 올라간다.

★**`Effect::is_in_range` 경계 극성(`<=`)도 실행으로 확정.** 타워(radius 10000, range 0, Targeting)를
시전자로 두고 대상 챔피언 radius 를 바꿔 경계를 밟았다 — 두 개의 독립 데이터점:
```
D3_o3 Z:  대상 radius=0    → 거리 10000 true / 10001 false      (R = 10000)
D3_o4 J:  대상 radius=5000 → 거리 15000 true / 15001 false      (R = 10000 + 5000)
```
⟹ `dist_sq <= total*total` 의 `<=` 와 `caster.radius() + target.radius()` 가산이 둘 다 실행 확정.

### 다른 오염 항목 탐색 결과
15~19 의 오라클 근거 주장 전수를 grep 해 **타워 수/좌표에 의존하는 주장은 `/specs[15]` 3곳뿐**임을
확인했다(19 의 `10/10`·`is_cleared` 전부 false, 16 의 타워 시전자 10000, 18 의 `player_champion`
`[[Option<&Entity>;5];2]`, 17 의 `MainObjective` 3B 는 전부 타워 수와 무관하고 재측정에서 그대로다).
**그중 2곳이 아직 옛 값을 갖고 있다 → §1 P-1, P-2.**

---

## 1. 패치 목록 (JSON 경로)

### P-1 `/specs[15]/logic` — 오염된 값이 의사코드에 그대로 남아 있다 ★오류
249 행 주석:
```
구: … nexus(+0x170)는 별도 필드라 실제 제외. 실측 팀당 10개(6+4, twin 2쌍은 좌표 중복)
신: … nexus(+0x170)는 별도 필드라 실제 제외. ★실측 팀당 8개(이름있는 6칸 + twin_towers 2개,
    좌표 중복 0). ~~팀당 10개(6+4, twin 2쌍 좌표 중복)~~ 은 오라클 설정 버그(init_tower 재호출)
    오염값 — 3차 배치 D 깨끗한 프로브 재측정(`_verify3\D\o1.txt`: tower_ids 16 · 팀당 8 ·
    twin 2/2 · 좌표중복 0 · PTR_IDENTICAL=true)
근거: _verify3\D\D3_o1.rs / o1.txt (start_game 만 호출)
```
⚠`history[4]` 에는 정정이 이미 들어 있으나 **`logic` 블록은 옛 값**이다 —
`logic_note` 가 스스로 경고한 "정정이 표에만 반영되고 logic 이 옛 값으로 남는 사고"가 재발했다.

### P-2 `/specs[15]/closed[1]/why` — 오염값을 근거로 인용 중 ★오류
```
구: "1차 배치D: 6칸 순서 + twin_towers 꼬리 확정, 2차 오라클 팀당 10개"
신: "1차 배치D: 6칸 순서 + twin_towers 꼬리 확정. 3차 오라클 **팀당 8개**(포인터 동일성 확인,
     `_verify3\D\o1.txt`). ~~2차 오라클 팀당 10개~~ = init_tower 재호출 오염값"
근거: _verify3\D\o1.txt
```

### P-3 `/specs[15]/logic` 248·253 행 — 시그니처 오류 ★오류
```
구: SinglePlanBattle::new_dive(version, &BattlePlanGoal::TryKill(target_id, 60), data, player)
    SinglePlanBattle::new(version, &BattlePlanGoal::TryKill(target_id, 60), data, player)
신: SinglePlanBattle::new_dive(version, BattlePlanGoal::TryKill(target_id, 60), data, player)
    SinglePlanBattle::new(version, BattlePlanGoal::TryKill(target_id, 60), data, player)
    (goal 은 **by-value `BattlePlanGoal`** — ABI indirect 라 IR 에선 ptr 로 보인다)
근거: tcx 정본 `fn(usize, BattlePlanGoal, &OperationData, &PlayerState) -> SinglePlanBattle`
      (spec3lib.py fn new / single_battle.rs:31·48) + 실제 컴파일 통과(`D3_o3.rs` 는 값 전달로만 빌드됨)
```
`history[2]` 의 "부수" 항목이 이미 이 사실을 적었는데 `logic` 이 옛 표기다(P-1 과 같은 실패 모드).

### P-4 `/specs[15]/logic` 250 행 — 패턴 형태 오류 ★오류
```
구: .and_then(|e| if let EntityType::Tower(tt) = e.ty /*+0x68 == 2*/ { Some(tt) /*+0x128*/ } else { None })
신: .and_then(|e| if let EntityType::Tower { info } = &e.ty /*+0x68 == 2*/ { Some(info.ty) /*+0x128*/ } else { None })
근거: `EntityType::Tower` 는 **튜플 variant 가 아니라 struct variant `{ info: Tower }`**
      (rustc: "expected tuple struct or tuple variant, found struct variant `EntityType::Tower`",
       help 가 `EntityType::Tower { info: _ }` 를 제시) · `tcxdict --enum EntityType`
      `TowerType` 은 `Tower.ty`
```
오프셋 값(+0x128)은 맞다 — `Entity.ty` = 0x68, `EntityType` 페이로드 = +0x8, `Tower.ty` = +0xb8 ⟹ 0x128.
(한 행 = 한 오프셋으로 다시 적으면)
- `game_core::Entity` `0x68` = `ty` (EntityType, 480B)
- `game_core::EntityType::Tower` 페이로드 `+0x8` = `info` (Tower, 192B)
- `game_core::Tower` `0xb8` = `ty` (TowerType, 1B)

### P-5 `/specs[16]/open[0]` — 「확인 불가」가 거짓 ★★오류 (이번 라운드 최대 수확)
```
구: "battle.rs:2399/2407/2414/2421 의 원본 소스 텍스트는 확인 불가 — 이 저장소에 game-ai 소스
     트리가 없다(C:\tfm2mods 및 sdk_058 에 game-ai/src 없음). 줄번호는 DWARF !DILocation 에서만
     복원했다"  class: "미탐색", ev: 3
신: "★**부분 복원 성공(3차 배치 D)**. 「확인 불가」는 재료를 안 집은 것이었다 — `METHOD_MAP §0`
     의 「`mir=0` 함수의 소스 한 줄 내용 → 줄 길이 산술」이 그대로 적용된다.
     `rmeta_srcmap game_ai plan_legacy\old\battle.rs` = 2,514줄 전량 파싱 OK
     (src_len=139494 · raw_len=142008 · CRLF) + DWARF 지역변수 이름·선언줄.
     확정:
       · **본문 들여쓰기 = 2칸**(L2398 이 20자 ⟹ `  let mut range = 0;`. indent 4 면 22자로 불일치)
       · L2397(86자) = `pub fn max_range_nearly_can_use(champ: &Entity, target: &Entity, tick: usize) -> u64 {`
         — **글자수 정확 일치(±0)**
       · L2398(20자) = `  let mut range = 0;` (변수명 `range` 는 DWARF !56148 정본)
       · 네 블록이 **구조 동일 · 내용 6줄씩**이고 슬롯별 길이 모델이 4블록 동시 일치한다
         (n = len(효과 필드명): attack_effect 13 / skill_effect 12 / skill2_effect 13 / ult_effect 10):
           슬롯1  L2399·2407·2414·2421 = 53·51·53·47 = **27 + 2n**  (이름이 그 줄에 정확히 두 번)
           슬롯2  L2401·2408·2415·2422 = 39·38·39·36 = **26 + n**   (DWARF: 이 줄이 `effect` 를 바인딩)
           슬롯3  L2402·2409·2416·2423 = 78·77·78·75 = **72 + len(접두)** (attack/skill/skill2/ult)
           슬롯4  L2403·2410·2417·2424 = **131 (네 줄 완전 동일)** = 사거리 합산식 한 줄
           슬롯5  L2404·2411·2418·2425 = 5 = `    }`
           슬롯6  L2405·2412·2419·2426 = 3 = `  }`
         (L2400 은 블록 A 안의 빈 줄 1개, L2406·2413·2420 은 블록 사이 빈 줄)
       · 슬롯1 의 유일 정합 표기 = `  if let Some(<name>) = &champ.<name> {`
         (비이름 문자 = `  if let Some(` 14 + `) = &champ.` 11 + ` {` 2 = **27**, 4줄 동시 일치)
       · DWARF 가 준 인라인 접근자 정본(파일 = entity.rs / effect.rs / option.rs):
         `attack_cooldown` 1747 · `skill_cooldown` 1774 · `skill2_effect` 1692 · `skill2_cooldown` 1789 ·
         `ult_effect` 1700 · `ult_cooldown` 1804 · `Effect::range` 25 · `Entity::radius` 1509 ·
         `Option::as_ref<Effect>` 741
     ⚠**남은 모순 = 미탐색**: 슬롯1 의 T=27 정합은 `&champ.skill2_effect`(**필드**)를 가리키는데
     IR/DWARF 는 `Entity::skill2_effect`(1692)·`ult_effect`(1700) **메서드가 인라인**됐다고 말한다.
     `champ.skill2_effect()` 라면 그 두 줄이 각각 1자 길어야 한다(54·48). 슬롯3 도 같은 방향으로
     1자 어긋난다 ⟹ 4줄 공통 템플릿 가정 중 하나가 틀렸다.
     **다음 수단(미탐색)** = `dloc.py` 로 `skill2_effect`/`ult_effect` 서브프로그램의 `inlinedAt`
     줄번호를 확정 → 그 줄이 2414/2421 이 아니면 템플릿이 갈린다."
class: "미탐색"  (유지 — 단 「확인 불가/재료 부재」 어휘는 **삭제**해야 한다)
ev: 3
근거: MIG\rmeta_srcmap.py game_ai "plan_legacy\old\battle.rs" 2395 2426  /  _gaibc\m10.ll 51913~52374 의
      !56143~!56264 (DILocalVariable · DISubprogram)
```
⚠**브리핑 자체의 오류**: 3차 브리핑(및 지시문)은 "16 의 원문은 **재료 부재(전 범위)** 로 종결됐다
(rmeta 에 hash 만·IR/MIR/exe 어디에도 없음) — 다시 파지 마라" 라고 했다. 그런데
①v3 의 실제 `class` 는 `미탐색` 이고 ②**재료는 있었다**(줄 길이 산술 = 배치 D 자신의 18번
`epic.rs 654~657` 을 뚫은 바로 그 방법). "rmeta 에 원문이 없다"는 참이지만 **"원문을 알 수 없다"는
거짓**이다. 이건 `METHOD_MAP §3` 이 경고한 정확히 그 오염 = **가진 재료의 한계를 문제의 한계로 착각**.

### P-6 `/specs[15]/open[0]` — 이미 닫힌 것이 open 에 남아 있다 (과열림, 안전한 실수)
```
open[0].q = "BattlePlanGoal::TryKill 의 두 번째 usize(=60)의 의미 … 틱 예산/유효기간 '추정'이지만 확인 안 함"
→ `/specs[15]/history[1]` 이 이미 **"소비처가 존재하지 않는다 — 노브가 아니다"** 로 닫았고
  적용범위·함정방지까지 붙어 있다. open[0] 은 `closed[]` 로 이동하고 why = "history[1]" 로.
★3차 오라클 추가 확증: `BattlePlanGoal::TryKill(target, __1)` 의 `__1` 을 0·1·30·60·120·99999 로
  바꿔도 `base_sub_goal`·`SinglePlanBattle::new`·`new_dive`·`update` 의 산출이 **전부 동일**
  (`o3.txt` D/D_dive 9행, `o6.txt` 17행 TryKill(C1Top,60) vs (C1Top,0) 전 버전 동일) ⟹ 노브 아님 재확인.
근거: _verify3\D\o3.txt · o6.txt
```

### P-7 `/specs[15]/open[2]` — 절반이 이미 닫혔다 + 나머지 절반이 진전했다
```
구: "engage_requires_dive / single_tower_dive_is_viable 내부 판정식은 안 봄(담당 범위 밖). 이 함수는 bool 만 소비"
신: "`engage_requires_dive` 는 **닫힘**(history[3]+[4] + 3차 오라클 260/260 재현 일치·경계 2점 확정).
     남은 것은 `single_battle::single_tower_dive_is_viable` 뿐이고 3차에서 다음까지 확정했다:
       · **오라클 호출 가능**(`pub`, single_battle.rs:891). ⚠쌍둥이 `death_battle::…`(death_battle.rs:1674)
         는 `in:death_battle` 라 **호출 불가** — 15 가 부르는 것은 `plan_legacy::old::` 로 재수출된
         `single_battle::` 쪽이다(pubapi_game_ai.txt:303).
       · 내부 호출 사슬 = single_battle.rs:**938** → `fight_check.rs:960 check_kill_die_tick`
         → `fight_check.rs:979 check_kill_die_tick_uncached` → **979:59 `Option<&PlayerState>::unwrap()`**
         (백트레이스 원문 = `_verify3\D\D3_o2` 실행 로그)
       · ⟹ **target 은 챔피언이어야 한다.** 타워/넥서스를 target 으로 주면 그 unwrap 이 None 이라
         **패닉**한다(실전 호출부는 `TryKill` 대상 = 챔피언이라 안 밟는다).
       · **RNG 의존 아님**: 같은 입력에 rnd 진행 20회 / 신규 시드 0..19 20회 모두 동일(`o8.txt` 15b).
       · SDK 로 구성 가능한 세계에서 **true·false 둘 다 재현**된다(true = `o7/o8/o11`, false = `o9/o10`).
     ★**남은 미탐색 = 판별 축**. 다음 축들은 3차에서 **전부 무관**으로 실측 배제됐다:
       RNG · 타워 hp(1~2,000,000) · 대상 hp(1~2000) · 액터 공격력(0~5000) · 방어력(0~3000) ·
       이펙트 사거리(0~200000) · 액터↔대상 거리(10000~500000) · 아군 근접 수(1~5) ·
       아군/적 챔프 배치(적층 vs 산개). 재현 앵커 = true `_verify3\D\D3_o11.rs` 6/6 ·
       false `_verify3\D\D3_o10.rs` 29/29 (두 프로브 사이를 이분하면 축이 잡힌다)"
class: "미탐색"
근거: _verify3\D\o2~o11.txt
```

### P-8 `/specs[15]/open[3]` — ★update 게이트가 오라클로 크게 열렸다 (최대 미결의 실질 진전)
`SinglePlanBattle::{new, new_dive, update}` 와 `BattlePlanGoal`·`BattleSubPlanGoal` 이 **전부 `pub`**
이므로 `single_try_engage` 본문을 그대로 재현해 돌릴 수 있다(`D3_o3.rs`~`D3_o5.rs`).
⚠디폴트 데이터로는 판별력이 0 이다 — `SwordmanChampionInfo::default()` 의 **`stat_cached.hp == 1`**
(타워도 hp=1·attack=1) 이라 전 조합이 `RunAway` 로 몰린다(`o3.txt` 전 61행).
`stat`/`stat_cached`(pub)·`attack_effect.range`(pub)·`world.tick`(pub) 을 열면 갈린다:

```
(조건: tick=0 · 비다이브 · champ hp/stat_cached.hp 2000 · attack 5000 · move_speed 3000 ·
 radius 5000 · attack_effect.range 50000 · TeamPlan::default() · PositioningScoreData::default())

거리(actor↔target)  <= 100000              → Kiting { focus: target_id }
                    100001 ~ 250000        → Trace  { focus: target_id }
                    >= 250001              → RunAway
world.tick          <= 120                 → 위 표대로
                    >= 121                 → RunAway   (start_tick 을 0 으로 덮어도 동일)
```
- 두 거리 임계는 **1 단위 이분탐색으로 확정**(`o5.txt` BIS: `Kiting_max_d=100000 / next_d=100001`,
  `last_nonRunAway_d=250000 / RunAway_from_d=250001`).
- **축 비의존**(x축·y축·대각 동일) ⟹ 순수 제곱거리 비교(`o5.txt` AXIS 18행).
- `tick` 임계 120/121 은 **`tick_per_second` 비의존**(tps 30·60·90 모두 120/121, `o5.txt` 3세트).
- **version 비의존**(0·1·2·3·4·5·10·30·31·32·33·50·100 전부 동일 — `o3.txt` A, `o4.txt` K/K2)
  ⟹ `/specs[15]/open[1]`(version 분기) 은 **오라클로도 분기 0** 재확인.
- 무관으로 배제된 축: 액터 hp(1~100%) · 대상 hp(1~100%) · `attack_effect.range`(0~400000) ·
  `battle.start_tick` · `battle.help_called`(`o4.txt` G/H/I).
- ⚠**적용 범위**: 위 임계는 `TeamPlan::default()`(objective=None) · 미니언 0 · `tick=0` 세계의
  관측값이다. v3 `history[5]` 가 든 `222·234 거리 ≥200000 → End`·`245 거리 ≥300000 → End` 는
  **objective Hunt 분기 안**이라 이 세계에선 발화하지 않는다 — 서로 다른 게이트이므로 **모순 아님**.
  `tick>=121 → RunAway` 의 소스 대응(절대 tick 비교인지, `history[5]` 의 `333 tick−120 > start_tick`
  과 어떻게 연결되는지)은 **미탐색** — `start_tick` 을 0/현재틱으로 갈라도 결과가 같아 이 층에선 못 가른다.
- ★부수 실측: `BattleSubPlanGoal` 의 페이로드 variant 는 **튜플이 아니라 struct variant `{ focus }`**
  (rustc: "found struct variant `BattleSubPlanGoal::Trace`"). `tcxdict --enum` 의 필드명 `focus` 와 일치.

### P-9 `/specs[17]/closed[0]` + open — `base_sub_goal` 전수 진리표 확보 (미탐색 → 부분 확정)
`BattlePlanGoal::base_sub_goal` 은 **`pub`**(battle.rs:69)이라 그대로 돌린다(`D3_o6.rs`·`D3_o9.rs`).
```
fn(&BattlePlanGoal, version, &PlayerState, &OperationData) -> BattleSubPlanGoal

goal = Response                 → RunAway                (version·actor 무관)
goal = Avoid                    → RunAway                (version·actor 무관)
goal = TryKill(id, __1)         ┐
goal = Support(id)              ┘ 둘이 **완전히 같은 산출**
   · id 가 **챔피언 엔티티**      → 거리에 따라  Trace { focus: id }  또는  End
   · id 가 타워(적/아군 무관)     → **Trace { focus: id }** (거리 무관 — 902,000 거리에서도 Trace)
   · id 가 **존재하지 않는 id**   → **Trace { focus: id }** (999999 로 확인)
version = 0·1·2·3·30·32·50 전부 동일 ⟹ **이 함수에도 version 분기가 없다**
`__1`(60 vs 0) 무관 ⟹ P-6 재확인
```
- 챔피언 target 의 Trace↔End 갈림은 **가시성이 아니다**: `entity.visible_state[team]`(Visible/
  Unknown/Invisible) · `can_target=false` · `invisible_tick=999` · `hp=0` · `world.visible_map[team]`
  전 격자 1 · `world.exist_map[team]` 전 격자 1 — **여섯 축 모두 산출을 바꾸지 못했다**(`o7.txt` 17a, `o8.txt` 17b).
- 갈림의 실체는 **actor↔target 거리**다(`o8.txt` 17b: 적 챔프를 액터 옆으로 옮기면 End → Trace).
  ⚠단 거리 임계 이분탐색은 `d=999999` 까지 전부 Trace 로 나와(`o9.txt` 17c) **임계가 1,000,000 초과이거나
  거리 아닌 다른 양(예: 격자 셀 인덱스·라인 소속)** 이다 ⟹ **미탐색**. 초기 스폰 배치(제곱거리 ~1.27e6)
  에서만 End 가 나왔다는 것까지가 확정.
- ⟹ `closed[0]`("base_sub_goal 내부는 안 봄")의 **외연은 확정**됐으니 `history` 에 이 표를 싣고,
  남은 "챔피언 target 의 Trace↔End 술어" 만 `open` 으로 남기는 것을 제안한다.

### P-10 `/specs[19]/open[0]` — **닫힘** (`get_camp_state` 전수 표)
`JungleRunner::get_camp_state` 는 **`pub`**(jungle.rs:702). **포인터 동일성**으로 필드 매핑을 확정했다
(`D3_o6.rs`, `o6.txt` 19 24행):
```
ty ∈ {Rhino, Mushroom, Stump, Bee}
    team == 0 → blue_<camp>   (is_blue_side = true)
    team == 1 → red_<camp>    (is_blue_side = false)
    team >= 2 → ★PANIC  unreachable!("team value must be 0 or 1")  @ jungle.rs:713
ty == Morgard → **epic** (is_blue_side = false) — **team 무관**(team 2·5 에서도 정상 반환)
ty == Serpen  → ★PANIC (team 0·1 포함 **전 team**) ⟹ `JungleRunner.serpen` 필드는
                 이 접근자로 **도달 불가**(별도 경로 전용)
```
⟹ v3 가 "team 으로 blue/red, ty 로 캠프를 고르는 것이 자연스럽지만 IR 로는 확인 못 했다" 로 남긴
추정이 **정확히 맞았고 + Morgard/Serpen 특례 2건이 새로 붙는다.** `best_jungle_goal` 은
`jungle_camps` 4칸(Rhino/Mushroom/Bee/Stump)만 넘기므로 실전에서 이 패닉을 밟지 않는다.
- 오프셋 재확인(한 행 = 한 오프셋):
  - `game_core::MobaMode` `0x18` = `jungle_runner` (JungleRunner, 480B) ← `/specs[19]/mem[7]` 의 `chk` 가
    `"조회실패"` 인데 `tcxdict MobaMode` 로는 정상 조회되고 오라클에서 `game.mode.jungle_runner` 로
    실제 접근했다 ⟹ `chk` 를 `"tcxdict OK + 오라클 접근 성공"` 으로 갱신 제안
  - `game_core::JungleCampState` `0x18` = `next_respawn_tick` (usize)
  - `game_core::JungleCampState` `0x28` = `is_blue_side` (bool)
  - `game_core::JungleCampState` `0x29` = `ty` (JungleType, 1B)

### P-11 `/specs[18]/open[0]`·`closed[4]` — epic 헬퍼 3종 진리표 (미탐색 → 규칙표 확정)
`v3_epic_group_line`·`v3_epic_formation_role`·`v3_serpen_contest_clear_win` 이 **전부 `pub`**
(pubapi_game_ai.txt:331~333, serpen.rs:52).
```
v3_epic_formation_role(strategy, position, player, data) -> Option<V3EpicFormation{is_split, line}>
   Gather                      → 전원 { line: Mid,    is_split: false }
   Split14 { position: P }     → P 인 사람만 { line: Bottom, is_split: true } · 나머지 { Mid, false }
   Split131 { position1, position2 }
                               → position1 → { line: Top,    is_split: true }
                                 position2 → { line: Bottom, is_split: true }
                                 나머지     → { line: Mid,    is_split: false }
   (팀 0/1 동일 · 8개 전략 × 2팀 × 5포지션 = 80칸 전수, `o6.txt` 18 16행)

v3_epic_group_line(strategy, player, data) -> Option<LineType>   ★TutorialType 에 의존한다
   tutorial=None        Gather=Some(Mid)     Split14(Top)=Some(Mid)
   tutorial=First       Gather=Some(Bottom)  Split14(Top)=**None**
   tutorial=TopSolo     Gather=Some(Top)     Split14(Top)=Some(Top)
   tutorial=Bottom      Gather=Some(Bottom)  Split14(Top)=**None**
   tutorial=MidSolo     Gather=Some(Mid)     Split14(Top)=Some(Mid)
   tutorial=MidBottom   Gather=Some(Mid)     Split14(Top)=Some(Mid)
   tutorial=JungleOnly  Gather=**None**      Split14(Top)=**None**
   tutorial=Line        Gather=Some(Mid)     Split14(Top)=Some(Mid)
   tutorial=Total       Gather=Some(Mid)     Split14(Top)=Some(Mid)
   (`o7.txt` 18a 9행)

v3_serpen_contest_clear_win(version, player, data, team_plan) -> bool
   = **true** (version 0·1·2·3·30·32·50 × team 0·1 = 14칸 전부) — 디폴트 세계에선 판별력 0
```
⟹ `/specs[18]` 의 `ret` 서술 중 **"group_line==None → false"** 경로가 **실제로 도달 가능**함이
오라클로 확인됐다(`tutorial=JungleOnly`). ⚠적용 범위: `line: Mid` 가 리터럴인지 `group_line` 과
같은 값인지는 이 세계에서 group_line 이 항상 Mid 라 **갈리지 않는다** = 표기 불가 아님, **미탐색**.
- 부수: `PlayerState::strategy` 의 실제 시그니처는 **`(&self, &mut StdRng, &dyn AbstractGame)`**
  (player.rs:1576) — 인자 3개다. `/specs[18]/sig/params[3]/role` 이 "PlayerState::strategy 호출에만
  넘긴다" 로 적은 것과 모순은 없으나, 재구현 시 **game 인자**가 빠지면 컴파일이 안 된다는 점을 적어둘 것.
  실측 `strategy(rnd, game).morgard_use` = `Gather` (양 팀, seed 7).

### P-12 `/specs[18]/open[2]` — 17 이 이미 닫은 것과 같은 사안 (배치 내 불일치)
```
open[2].q = "Chat::Repair / SerpenSetup / Press / PressChange 가 공통으로 갖는 usize 필드(enum+0x8)에
             항상 0 을 넣는데, dienum 이 준 이름이 __0/__1 뿐이라 그 0 의 의미 확정 불가"
→ `/specs[17]/closed[4]` 가 **같은 사안을 "이름이 없다"로 닫았다**: `Chat` 은 game_ai 가 아니라
  `game-core\src\simulation\state\player.rs` 정의이고 **variant 가 튜플**이라 rmeta 필드 테이블의
  이름이 `0`/`1` 뿐 — **의도를 알려주는 이름이 애초에 존재할 수 없다**(도구 한계 아님).
  ⟹ 18 의 open[2] 도 같은 근거로 `closed[]` 로 옮기고, 남길 것은
     "그 0 을 읽는 소비처가 있는가"(17 은 소비처 0건 = 죽은 슬롯으로 판정) 뿐이다.
근거: /specs[17]/closed[4] (RE6-09-11_rmeta-SourceMap-rustc프로브-Span복원.md)
```

### P-13 `/specs[18]/open[3]` — 이미 대부분 닫혔다 (과열림)
```
open[3].q = "epic.rs 655~663 줄에 해당하는 IR 이 없다 …"
→ `/specs[18]/closed[5]` 가 **L654~657 을 소스 ±0 수준으로 복원**했고(`self.objective =
  Some(MainObjective::Serpen { phase: ObjectPhase::Setup, with_battle: true });`), 「DILocation 이
  659·674·676 에 없다」는 전제 자체가 거짓이었음까지 적었다. open[3] 은 **범위를 655~656·662~663 으로
  좁혀서** 남기거나 `closed[]` 로 옮길 것. (655·656 = 위 복원의 계속행, 662~663 = 한글 `//` 주석)
```

### P-14 `/specs[18]/open[4]` — 본문이 "닫힘"이라고 말하면서 class 가 미탐색
```
open[4].q 본문 = "★**\"죽은 variant\" 로 닫힘(2026-09-11)**… 값 1 을 쓰는 코드 0건"  class: "미탐색"
→ 본문이 근거·적용범위(`_gaibc`·`_gcbc`·`_gvbc` 한정)·남은 미탐색(미추출 rlib·rmeta MIR·exe)까지
  전부 갖췄다. `closed[]` 로 옮기고 `why` = "죽은 variant — IR 3종에서 값 1 store 0건" 로.
  (판정 어휘는 「재료 부재(범위 = `_gaibc`·`_gcbc`·`_gvbc`)」가 정확하다)
```

---

## 2. 오류 없음으로 확인한 것

### `callees_unmatched` — 판정 술어 혼입 **0건**
| spec | names | 판정 |
|---|---|---|
| 15 | (없음) | — |
| 16 | `casting`, `llvm.umax.i64` | `casting` 은 **`Effect` 의 필드**(`Effect+0x30`)로 호출이 아니다 · `llvm.umax.i64` 는 LLVM 인트린식 ⟹ 술어 아님 |
| 17 | `grow_one` | `RawVec::grow_one`(std Vec push 성장) ⟹ 술어 아님 |
| 18 | `grow_one` | 동상 |
| 19 | (없음) | — |

### `ev<=3` 표본 재확인 — 뒤집힌 것 **0건** (해당 항목이 5개 명세 전체에 3개뿐이라 전수)
- `/specs[15]/mem[5]` `LegacyPlanHandler` `0xf8` = `team_plan`, **공유 `&TeamPlan`** →
  tcx 재확인 `single_tower_dive_is_viable : fn(usize, &mut StdRng, &PlayerState, &OperationData,
  &TeamPlan, &Entity, &mut DebugFrameData) -> bool` ✓ 그리고 오라클 프로브가 `&tp`(공유)로 **컴파일 통과** ✓
- `/specs[17]/mem[41]` `DeathMatchBattle` `0x17b` = `main_objective`, `Option<MainObjective>`(3B) →
  `tcxdict DeathMatchBattle` 재확인 ✓ (0x17a=`dive_tower` 1B, 0x17b=`main_objective` 3B, 0x17e=`lean_last_sign`)
- `/specs[19]/mem[7]` `MobaMode` `0x18` = `jungle_runner`(480B) → `tcxdict MobaMode` ✓ + 오라클 실접근 ✓
  (`chk` 값만 갱신 제안 = P-10)

### 그 밖에 재확인하고 **맞았던** 것
- `/specs[19]/logic` `jungle_camps = [Rhino(0), Mushroom(1), Bee(3), Stump(2)]` — 배열 순서가
  열거형 선언 순서(Rhino/Mushroom/**Stump**/Bee)와 다르다는 것까지 정확하다(`tcxdict --enum JungleType`).
- `/specs[19]` `is_cleared`/`is_side_cleared` 초기 전부 false → 깨끗한 세계에서도 8/8 false ✓
- `/specs[16]` 타워 시전자 `max_range_nearly_can_use` = 10000(= tick 0/40/60/100000 무관) ✓
- `/specs[15]` `engage_requires_dive` 극성 `1 − player.info.team` → 깨끗한 세계 260쌍 재현 일치 260/260 ✓
- `/specs[17]` `BattlePlanGoal` = 24B enum, 태그 `+0x0`(0..4), `TryKill` 페이로드 `+0x8`/`+0x10` ✓
  (`tcxdict --enum BattlePlanGoal`: TryKill 은 **튜플** variant 라 필드명이 `0`/`1`)

---

## 3. 도구 게이트

```
python -X utf8 specgate.py
  ## 명세 완결 조건 검사 (게임 0.5.8)
     G1 자기모순=0  G2 호출부 전수=0  G3 형제 함수=0  G4 술어 시그니처=0
  총 0건                                                        ← 통과

python -X utf8 tcxaudit.py --prose C:	fm2mods\MIG\_verify3\D\D3_REPORT.md
  총 716건  오귀속=0  밀림=0  부분일치=1  확인불가=18  OK=697    ← 통과
  (감사 전 기준선 = 680건. 이 보고서 §5 의 36개 오프셋 주장이 **전부 OK**로 들어갔다 —
   부분일치 1건은 이 보고서와 무관한 기존 `specs20.json  base=cache offset=0x8` 항목,
   확인불가 18건도 전부 기존 vtable 슬롯 항목이다.)
```
⚠**도구 사용 주의(다음 세션용)**: `tcxaudit --prose` 의 스캐너는 `Type+0xNNN` **형태만** 잡는다
(`PROSE` 정규식 = `([A-Za-z_]\w{2,40})\s*\+\s*(0x…)`). 처음에 오프셋을 표 형식
`` `Type` `0x68` = `field` `` 로 적었더니 **한 건도 스캔되지 않아 감사가 무의미했다**(총계가 680 에서
변하지 않는 것으로 발각). 오프셋을 발표할 때는 반드시 `Type+0xNNN` 형태로 한 번은 적을 것.

---

## 4. 다음 세션에 남기는 것 (판정 어휘 + 범위)

| 대상 | 판정 | 범위 |
|---|---|---|
| `single_tower_dive_is_viable` 판별 축 | **미탐색** | RNG·타워hp·대상hp·공격력·방어력·사거리·거리·아군수·배치 **9축은 배제 실측 완료**. 앵커 = `D3_o11.rs`(true 6/6) ↔ `D3_o10.rs`(false 29/29) 이분 |
| `update` `tick>=121 → RunAway` 의 소스 대응 | **미탐색** | 오라클로는 `start_tick` 0/현재틱을 갈라도 동일 ⟹ 이 층에서 못 가름. 다음 = `_gaibc/m05.ll:27043~34389` 의 120 리터럴 xref |
| `base_sub_goal` 챔피언 target 의 Trace↔End 술어 | **미탐색** | 가시성 6축 배제 완료. 거리 임계는 1,000,000 이하에 없음 ⟹ 거리 아닌 양일 가능성 |
| `battle.rs` 2399/2407/2414/2421 원문 마지막 1자 | **미탐색** | 줄 길이 산술로 T=27 까지 확정. 다음 = `dloc.py` 로 `skill2_effect`/`ult_effect` 의 `inlinedAt` 줄 확정 |
| `epic.rs` 648~650·662~663·685~686 한글 `//` 주석 | **재료 부재** | 범위 = rmeta(라인주석 미포함) · `_gaibc`/`_gcbc`/`_gvbc` IR · MIR(mir=0) · exe 디스어셈(렉서가 폐기) |
| `v3_epic_formation_role` 의 `line: Mid` 가 리터럴인가 `group_line` 인가 | **미탐색** | 디폴트 세계에서 group_line 이 항상 Mid 라 안 갈린다. `tutorial=TopSolo`(group_line=Top) 에서 role 을 다시 재면 갈린다 |

---

## 5. 이 보고서가 주장하는 오프셋 전량 (한 행 = 한 오프셋 · tcxaudit 대조용)

- game_core::Entity+0x68 = ty (EntityType, 480B)
- game_core::Entity+0x438 = stat_buff_cached.range (usize)
- game_core::Entity+0x470 = stat_buff_cached.radius_mult (i32)
- game_core::Entity+0x660 = x (u64)
- game_core::Entity+0x668 = y (u64)
- game_core::Entity+0x680 = radius (usize)
- game_core::Entity+0x490 = attack_effect (Option<Effect>, 56B)
- game_core::Entity+0x4c8 = skill_effect (Option<Effect>, 56B)
- game_core::Entity+0x500 = skill2_effect (Option<Effect>, 56B)
- game_core::Entity+0x538 = ult_effect (Option<Effect>, 56B)
- game_core::Tower+0xb8 = ty (TowerType, 1B)
- game_core::Effect+0x10 = range (u64)
- game_core::Effect+0x18 = growth_range (u64)
- game_core::Effect+0x30 = casting (CastingType, 4B)
- game_core::BuffState+0xc8 = range (usize)
- game_core::BuffState+0x100 = radius_mult (i32)
- game_core::MobaMode+0x18 = jungle_runner (JungleRunner, 480B)
- game_core::JungleCampState+0x18 = next_respawn_tick (usize)
- game_core::JungleCampState+0x28 = is_blue_side (bool)
- game_core::JungleCampState+0x29 = ty (JungleType, 1B)
- game_core::JungleRunner+0x180 = epic (JungleCampState, 48B)
- game_core::JungleRunner+0x1b0 = serpen (JungleCampState, 48B)
- game_core::World+0xec98 = tick (usize)
- game_core::AbstractGameWithCache+0x130 = twin_towers
- game_core::AbstractGameWithCache+0x170 = nexus
- game_core::AbstractGameWithCache+0x180 = top_tower
- game_core::AbstractGameWithCache+0x190 = top_tower2
- game_core::AbstractGameWithCache+0x1a0 = mid_tower
- game_core::AbstractGameWithCache+0x1b0 = mid_tower2
- game_core::AbstractGameWithCache+0x1c0 = bottom_tower
- game_core::AbstractGameWithCache+0x1d0 = bottom_tower2
- game_core::AbstractGameWithCache+0x1e0 = player_champion
- game_ai::plan_legacy::handler::LegacyPlanHandler+0xf8 = team_plan
- game_ai::plan_legacy::old::SinglePlanBattle+0x58 = sub_goal (BattleSubPlanGoal, 16B)
- game_ai::plan_legacy::old::SinglePlanBattle+0x8c = dive_tower (Option<TowerType>, 1B)
- game_ai::plan_legacy::old::DeathMatchBattle+0x17a = dive_tower (Option<TowerType>, 1B)
- game_ai::plan_legacy::old::DeathMatchBattle+0x17b = main_objective (Option<MainObjective>, 3B)
