# 5차 반증검증 — 배치 D (`specs[15]`~`specs[19]`) 보고서

게임 0.5.8 / SDK `sdk_058` / toolchain `nightly-2026-05-24` / 2026-09-11
작업 파일 = `C:\tfm2mods\MIG\_verify5\D\`
프로브 8개 = `D5_o17.rs` `D5_o17b.rs` `D5_o16.rs` `D5_o16b.rs` `D5_o19.rs` `D5_o19b.rs` `D5_o19c.rs` `D5_o1518.rs`
출력 = `o17.txt` `o17b.txt` `o16.txt` `o16b.txt` `o19.txt` `o19b_m1.txt` `o19b_m2.txt` `o19c.txt` `o1518.txt`
집계 스크립트 = `evup.py` → `evup.txt` / 게이트 출력 = `gate_specgate.txt` `gate_audit4.txt` `gate_tcxaudit.txt`

---

## 1. 결론 한 줄

**실오류 3 · 판정반전 0 · 새 발견 21 · `ev4→ev2` 상향 128행(내 대역 ev≥4 비율 91.7% → 37.5%) · `ev<=3` 뒤집힘 0.**
배치 D 의 `ev2`(오라클 실행) 행이 **0 → 128** 이 됐고, 그 과정에서 실오류 3건이 드러났다.
오라클 대조는 전부 `game == mine` 형태로 재서 **총 236/236 MATCH**(o16 66 + o16b 56 + o19 29 + o19b 6 + o19c 79), 불일치 0.
4차의 판정반전 **R2(`base_sub_goal` 축 = 적 우물 위험)는 실행으로 확증**됐다(12/12) — 반전이 아니라 굳히기다.

---

## 2. 정정 목록 (JSON 경로 · 구 → 신 · 근거)

### E1 `/specs[17]/logic` — `chats` 용량이 1이 아니라 **4** ★실오류(값)

```
구: chats.grow_one();                           // cap 0 -> 1 (Vec::push 인라인)
신: chats.grow_one();                           // cap 0 -> **4** (RawVec::MIN_NON_ZERO_CAP — size_of::<Chat>()=24 ≤ 1024 이므로 4)
```

**근거(실행, 2중)** — `_verify5\D\o17b.txt`
```
vec_empty   w0=0            w1=8            w2=0                      ← 내가 만든 빈 Vec<Chat>
vec_push1   w0=4            w1=1952448747376 w2=1  cap_api=4 len_api=1 ← push 1회
vec_push2                                          cap_api=4 len_api=2 ← push 2회도 여전히 4
```
그리고 `DeathMatchBattle` 안의 그 Vec 자체도 `cap=4`(`o17b.txt` `tick` 4행 전부 `chats_cap=4 chats_len=1`,
`o17.txt` `A/0x0f8_chats_w0 = 4`).
부수로 확정된 것 — **Vec 3워드의 메모리 순서는 `cap`@+0x0 / `ptr`@+0x8 / `len`@+0x10`** 이다
(빈 Vec 이 `0 / 8 / 0` = cap0·dangling8·len0 이고, 원소 1개일 때 셋째 워드가 1).
⟹ `/specs[17]/mem[18]/note` 의 "빈 Vec(ptr=dangling 8, len=0, cap=0)" 은 **맞다**. 고칠 곳은 `logic` 한 줄뿐이다.

### E2 `/specs[19]/logic` — "이 함수는 '일반 캠프' 전용" 은 **반증된다** ★실오류(판정 과잉)

```
구: [807] jungle_camps: [JungleType;4] = [Rhino(0), Mushroom(1), Bee(3), Stump(2)]
          ※ Morgard(4)/Serpen(5)는 후보에 없다 = **이 함수는 '일반 캠프' 전용.**
신: ※ Morgard(4)/Serpen(5)는 **후보 배열에 없다**. 단 **반환값이 일반 캠프로 한정되지는 않는다** —
    824~829 폴백(champ==None)은 `now_camp` 를 그대로 돌려주므로 `now_camp=Some(Morgard|Serpen)` 이면
    **Morgard/Serpen 이 그대로 반환된다**(실행 확인). 소비측이 반환을 {Rhino,Mushroom,Stump,Bee}로
    가정하면 틀린다.
```
**근거(실행)** — `_verify5\D\o19.txt` (champ 를 `cache.player_champion[0][1] = None` 으로 만든 뒤)
```
champNone_nowcamp  Some(Rhino)    game=Rhino     MATCH(그대로 유지)
champNone_nowcamp  Some(Morgard)  game=Morgard   MATCH(그대로 유지)
champNone_nowcamp  Some(Serpen)   game=Serpen    MATCH(그대로 유지)
```
배열 주장 자체는 참이고(`camp_idx` 교차확인, §4 N10), 틀린 것은 거기서 끌어낸 결론이다.
`knobs[4]`("이 경로는 not_cleared 필터를 무시한다")는 이 사실과 정합하므로 손대지 않는다.

### E3 `/specs[16]/mem[1..4]/name` + `/shared/Entity_공통_오프셋` — `EntityType::Champion` 은 **튜플 variant** ★실오류(구조 표기)

```
구: /specs[16]/mem[1]/name = "ty.Champion.attack_cooldown / ty.SmallJiangshi.info.attack_cooldown"
    /specs[16]/mem[2]/name = "ty.Champion.skill_cooldown / ty.Minion.info.attack_cooldown"
    /specs[16]/mem[3]/name = "ty.Champion.skill2_cooldown"
    /specs[16]/mem[4]/name = "ty.Champion.ult_cooldown / ty.Bear.info.attack_cooldown"
    (그리고 `shared.Entity_공통_오프셋` 의 0xb0·0xb8·0xc0·0xc8 네 행도 같은 표기)
신: "ty.Champion.**0**.attack_cooldown / ty.SmallJiangshi.info.attack_cooldown"  (나머지 3행 동형)
```
**근거 2중**
1. `tcxdict --enum EntityType` → `페이로드 Champion — enum+0x8  **0**  game_core::Champion (112B)`
   (같은 출력에서 `Minion`·`Tower`·`SmallJiangshi` 등 나머지 12 variant 의 필드명은 **`info`**)
2. rustc 진단 — `_verify5\D\D5_o16.rs` 초판이 `EntityType::Champion(ch)` 는 통과했고
   `EntityType::Minion(i)`/`Tower(i)` 는 **E0164 "expected tuple struct or tuple variant, found struct variant"**
   로 반려됐다(빌드 로그). ⟹ Champion 만 튜플, 나머지는 struct variant 다.

★**이것은 4차 D-E3(`ty.Tower.0` → `ty.Tower.info.ty`)의 거울상**이다. 4차는 struct variant 를 튜플로 적은 것을
고쳤고, 여기 남은 것은 **튜플 variant 를 struct 처럼(=`.0` 생략) 적은** 4행이다. 같은 배치가 같은 라운드에
한쪽만 고쳤다 — 재구현 시 `ty.Champion.attack_cooldown` 을 그대로 쓰면 컴파일되지 않는다.

### 보강(실오류 아님) — `/specs[16]/knobs[6]/effect` 의 제외 목록이 5개 중 2개만 적혀 있다

```
구: "*InCC 타깃팅이 어떤 CC 를 'CC 상태'로 볼지. Taunt(7)·Animation(10)이 빠져 있다"
신: "... HARD_CC = {0 Airborne, 1 Stun, 2 Bind, 6 ForceMove, 8 Fear, 9 Charm}.
     제외 = 3 BlockAttack · 4 BlockSkill · 5 BlockMoveSkill · 7 Taunt · 10 Animation (11 variant 중 5개)"
```
근거 = `tcxdict --enum CCState` (40B, 판별자 enum+0x0 4B, variant 11개) ↔ `_gcbc\g06.ll:87035~87042`·`87205~87212`
의 `switch i32 {0,1,2,6,8,9}`. 집합 {0,1,2,6,8,9} 자체는 **정확**하므로 값 오류는 아니고, 제외 목록의 누락이다.
부수 교차검증: IR 이 순회하는 원소 stride 40 = `size_of::<CCState>()` 40, 태그 `load i32` = 판별자 4B — 일치.

### `chk` 갱신 제안 2건 (vtable 슬롯 "확인불가" 해소)

| 경로 | 구 `chk` | 신 | 근거 |
|---|---|---|---|
| `/specs[17]/mem[5]` (`AbstractGame vtable+0x28` = `tick`) | `확인불가(vtable 슬롯)` | `OK` | `o17b.txt` — `world.tick` 을 0/1/4321/999999 로 바꿀 때 `start_tick`(구조체 +0x110)이 **4/4 동일하게 추종** |
| `/specs[19]/mem[6]` (`vtable+0x40` = `get_game_mode`) | `확인불가(vtable 슬롯(구조체 아님))` | `OK` | `o19c.txt` `FALLBACK` — 817 폴백에 실제로 도달해 `GameMode::Moba` 페이로드의 `jungle_runner` 를 타고 최소 리스폰 캠프를 골랐다(6/6) |

### `open`/`notes` 처리

- `/specs[15]/open[0]`(`version` 이 이 함수 안에서 분기를 만드는가, class=미탐색) — **부분 해소**.
  `single_try_engage` 자체는 `vis=in:game_ai` 라 호출 불가지만, 그 하위 4개 중 **3개가 `pub`** 이라 실측했다:
  `DeathMatchBattle::new` version 0/1/2/3/10/50/60 **7/7 동일**(`o17.txt` `ver` 7행),
  `SinglePlanBattle::new`/`new_dive`/`new_region` 도 version 축 반응 없음(`o1518.txt`).
  남은 미탐색 = `SinglePlanBattle::update` 와 `single_tower_dive_is_viable` 의 version 축(이 둘은 세계 구성이 필요).
  ⟹ `open[0]` 을 닫지 말고 **범위를 좁혀** 두는 것을 제안(`new`/`new_dive` 계열은 무영향 확정).
- `/specs[18]/notes[0]`(version 은 `v3_serpen_contest_clear_win` 에만 전달, class=사실 서술) — **반증 시도했고 반증 못 했다.**
  `v3_serpen_contest_clear_win`(pub)을 version 1/2/3/30/50/54/58/60 × 팀2 × 포지션5 = 80조합으로 직접 호출 →
  **80/80 false**(`o1518.txt` `E/uo`). version 축 자체의 판별력은 이 세계에서 0이라 "version 분기 없음"을
  **반박하는 증거는 나오지 않았다**. notes 유지.

---

## 3. ★`ev4 → ev2` 로 올린 행 (이번 라운드 주 산출물)

집계는 `_verify5\D\evup.py` 가 **JSON 경로 실재를 검증한 뒤** 찍는다(`evup.txt`):

```
상향 대상 128행  (구 ev 분포: {4: 117, 3: 11})
오프셋만 실측(18) 12행
경로 검증 OK — 전 행 실재
D(15~19)  분모=216
  전: ev3=18  ev4=198
  후: ev2=128  ev3=7  ev4=81
  ev>=4 비율: 91.7% → 37.5%
함수별 상향: specs[15]=13  specs[16]=38  specs[17]=51  specs[19]=26
```
전량 목록은 `evup.txt` 하단(`/specs[i]/필드[j]` 한 줄씩). 근거 요약:

### `/specs[16]` `max_range_nearly_can_use` — 38행 (`o16.txt` 66/66 + `o16b.txt` 56/56 = **122/122 MATCH**)

3차·4차가 못 댄 이유는 「`SwordmanChampionInfo::default()` 는 이펙트가 비어 전 구간 0」이었다.
**해법 = `Effect`(7필드 전부 pub) + `AttackEffect`(72B 전 필드 pub)를 직접 조립 + `Entity` 41필드가 전부 pub
이라는 사실**(§4 N20). 네 슬롯에 서로 다른 사거리(1000/2000/3000/4000, growth 10/20/30/40)를 심어 축별로 쓸었다.

| 상향 행 | 실행 근거 |
|---|---|
| `mem[13]`0x490 `mem[18]`0x4c8 `mem[23]`0x500 `mem[24]`0x538 | `o16b.txt` `eoff_attack`/`eoff_skill`/`eoff_s2u` — 주소 산술 실측 + 슬롯별 `None` 전환이 반환값을 바꿈 |
| `mem[14]`0x4a0 `mem[15]`0x4a8 `mem[16]`0x4b8 `mem[17]`0x4c0 / `mem[19]`0x4d8 `mem[20]`0x4e0 `mem[21]`0x4f0 `mem[22]`0x4f8 / `mem[27]`~`mem[30]` Effect+0x10·0x18·0x28·0x30 | `eoff_rel  Effect.range=+0x10  Effect.growth_range=+0x18  Effect.target=+0x28  Effect.casting=+0x30` (상대) + 절대(`+0x4a0`…`+0x568`). `range`/`growth_range` 는 레벨 스윕이, `target` 은 `CastingTarget::Ally` 전환이, `casting` 은 `None` 니치가 반환값을 바꿔 **판별력 있음** |
| `mem[0]`0x68 `ty` | `o16.txt` `tyChampion_cd9999`=0 vs `tyNexus`=5840 vs `tyNone`=5840. 그리고 `ty_tag=13` 직독 |
| `mem[1]`0xb0 `mem[2]`0xb8 `mem[3]`0xc0 `mem[4]`0xc8 | `o16b.txt` `cdoff_champion  attack=+0xb0  skill=+0xb8  skill2=+0xc0  ult=+0xc8` + 슬롯별 단독지배 쿨 경계(아래) |
| `mem[9]`0x110 Tower | `o16b.txt` `ent tower id=2 ty_tag=2 attack_cd_off=+0x110` → `entcd tower cd=40` 포함 / `cd=41` 배제 |
| `mem[11]`0x438 `mem[12]`0x470 `mem[25]`0x5c8 `mem[26]`0x680 | `sbc1/777/100000`(+1/+777/+100000) · `rm` 6케이스 · 레벨 8케이스 · radius 반영 |
| `consts[0]` −1 | `o16.txt` `niche  None_casting_i32=-1  Some(Targeting)_casting_i32=0` |
| `consts[1]` 13 | `tyChampion_cd9999`=**0**(전 슬롯 폐기) ↔ `tyNexus`=5840(전 슬롯 통과). 실제 Nexus 엔티티도 `attack_cooldown()`=0 |
| `consts[2]` 2 / `consts[3]` 4 | `lvlgate` — `skill2_getter_some` false(lv1,2)→true(lv3+), `ult_getter_some` false(lv1~4)→true(lv5+). 반환 3600·3620·**4660**·4690·**5760**·5800·5840·6280 |
| `consts[4]` 100 | `rm0_0`5840 / `rm50_0`6190 / `rm0_50`6290 / `rm100_100`7440 / `rm-50_0`5490 / `rm1_0`5847 — `r*(m+100)/100` 이 6/6 산술 일치(**i32 음수도 유효**) |
| `consts[5]` 0 | `allNone  game=0` |
| `consts[6]` 3 | 실제 Nexus 엔티티(tag 3): 페이로드에 Champion 쿨다운이 남아 있어도 `attack_cooldown()`=0, `cd` 4값 전부 51600 |
| `knobs[0]` tick | 슬롯 0~3 각각 `cd50` 고정 후 `tick` 40/49 → 2600(폐기), **50**/51/60 → 51600(통과). ⟹ `cd <= tick`(`<` 아님) 확정 |
| `knobs[1]` 2 `knobs[2]` 4 `knobs[3]` 13 `knobs[4]` 100 | 위 consts 와 동일 근거 |

**미상향(범위 명시)** — `mem[5]`0xd0(Illusion) `mem[6]`0xd8(Revenant) `mem[7]`0xe8(Jungle·Ghoul)
`mem[8]`0xf0(Eagle) `mem[10]`0x1f0(Epic·Serpen), 그리고 `mem[1]`의 SmallJiangshi 절반 · `mem[2]`의 Minion 절반.
**이유 = 그 엔티티 타입을 세계에 만들지 못했다**(tick 0 의 `jungle_ids`/`minion_ids` 가 빈 배열 —
브리핑 함정⑤ 대로 미니언은 `minion_wave_setting` + 600틱이 필요하다). tcx 산술로는 12/12 일치(4차 확인).
`knobs[5]`(180) `knobs[7]`(쿨감 100) `knobs[8]`(바닥값 3) 도 이 함수가 읽지 않아 미상향.

### `/specs[17]` `DeathMatchBattle::new` — 51행 (`o17.txt` · `o17b.txt`)

★**열쇠 = `DeathMatchBattle` 이 `pub` + `derive(Debug)`** 라 `region`/`flee_die`/`seal_basis` 등
**private 필드까지 이름과 값이 그대로 찍힌다.** 여기에 384B 원바이트 덤프를 겹쳐 오프셋까지 봤다.

```
caseA_debug  DeathMatchBattle { main_goal: TryKill(77, 88), sub_goal: Trace { focus: 77 }, stance: Commit,
  tactic: Standard, chats: [Battle(77, 0)], support_target: None, with_dive: false, dive_tower: None,
  start_tick: 0, region: None, help_called: false, well_runaway: None, main_objective: None,
  dive_abandoned: false, death_focus: None, dodge_commit: false, last_stand: false, seal_basis: None,
  flee_die: 9223372036854775807, trade_lean: 0, lean_last_sign: 0, lean_last_tick: 0, scene: Stand,
  hold_scene_basis: None, repo_scene_basis: None, scene_last_from: Stand, scene_change_tick: 0,
  flee_dir: None, had_ult_ready: false, last_act_tick: 0, idle_spec_tick: 0, last_swing_tick: 0,
  far_noout_since: None, ep_follow_until: 0, idle_prev_pos: (0, 0), last_unseal_tick: 0 }   ← 36필드
size_of_DeathMatchBattle 384      size_of_BattlePlanGoal 24
```
- `mem[6]`~`mem[14]` Option 9행 = 바이트 0 확인(`A/0x000_support_target 0` … `A/0x0c0_far_noout_since 0`)
- `mem[15]` 0xd0 = `A/0x0d0_main_goal_tag 0`, `0x0d8=77`, `0x0e0=88` (goal 24B 통째 이동)
- `mem[16]/[17]` 0xe8·0xf0 = `sub_lo=0`(tag Trace) `sub_hi=77`(focus). `caseA_base_sub_goal_direct`(직접 호출)와 **동일**
- `mem[19]` 0x110 `start_tick` = `world.tick` 4/4 추종 → `mem[5]` vtable+0x28 도 함께 ev2
- `mem[20]` 0x118 `flee_die` = **9223372036854775807** (= `consts[2]`)
- `mem[21]`~`mem[29]` memset 9행 = 전부 0 (0x120~0x168) — 4차 E4 의 「8→9필드」 정정 실행 확증
- `mem[30]`~`mem[35]` bool 6행 = 0, `mem[36]`~`mem[39]` = 0 이고 **Debug 이름이 `Commit`/`Standard`/`Stand`/`Stand`**
  ⟹ `dienum` 매핑(0=Commit / 0=Standard / 0=Stand)이 실행으로 확정(ev3 → ev2)
- `mem[40]` 0x17a = **255**, `mem[41]` 0x17b = **255** 이고 `0x17c`/`0x17d` 는 안 씀 → 「단일 `Option<MainObjective>`(3B)」확증
- `mem[42]` 0x17e = 0
- `consts[0]` 0 / `consts[1]` 3 / `consts[3]` −1 = `A_chat0` 바이트 `03 00 00 00 00 00 00 00 | 4d .. | 00 ..`
  (`tcxdict --enum Chat` 이 판별자 **enum+0x0 1B** 라 `logic` 의 "태그 i8 3" 도 맞다)
- `knobs[0]` = 4태그 전수: `TryKill`만 `chats_len=1`, `Support`/`Response`/`Avoid` 는 `chats_w2=0`(빈 Vec)
- `knobs[1]` = `trykill  __0=0 __1=12345` 에서도 `chat0` 의 두 번째 usize 가 **0** (goal 의 `__1` 이 아니다)
- `knobs[2]`/`knobs[3]` = 위 mem[36..38] / mem[20]

**미상향** = `knobs[4]`(team_plan.rs:376·388 의 tps*20) — 이 함수 밖.

### `/specs[19]` `best_jungle_goal` — 26행 (`o19.txt` 29/29 + `o19b.txt` 6/6 + `o19c.txt` 79/79)

| 상향 행 | 근거 |
|---|---|
| `mem[12]`0x660 `mem[13]`0x668 `mem[3]`GameContext+0x20 `knobs[1]` | 챔프 좌표 12점 스윕 — `pos` 12행 전부 `game==mine`, 함께 찍은 `d2[...]` 표가 곧 정렬키(`o19.txt`) |
| `mem[9]`cache+0x1e0 `knobs[4]` `consts[4]` | `cache.player_champion[0][1] = None` 으로 전환 → `champNone_nowcamp` 6/6 이 `now_camp` 를 그대로 반환, `champNone_random` 은 시드별로 Stump/Rhino/Bee 로 갈린다 |
| `mem[10]`0x930 `mem[11]`0x9c0 | `o1518.txt` `B/info.team +0x930` `B/info.position +0x9c0` (주소 산술) + `o19.txt` `plr` 10행에서 팀·포지션에 따라 결과가 갈린다(t1 은 `camp_pos(.., is_blue=false)` 로 뒤집힌 좌표를 씀) |
| `mem[4]`/`mem[5]`/`mem[6]`/`mem[7]`/`mem[8]`/`mem[15]` `knobs[2]` `knobs[0]` `consts[0..3]` | ★**817~819 폴백 경로에 도달**(`o19c.txt` `FALLBACK cleared=4/4`). 최소 `next_respawn_tick` 선택 4/4 + **동점 first-wins 2/2**(`fbk` 6행) |
| `knobs[3]`(offset) `knobs[5]`(+tps) | 필터 술어 `is_cleared` 완전식 **72/72 MATCH**(`o19c.txt` `isc`). `respawn=500` 에서 `offset` 0/1 → true, `offset=500` → false; 같은 줄에서 `eta=345`(Bee)만 false 인 것이 `+tps(60)` 항의 존재를 가른다 |
| `knobs[7]`(캠프 좌표 6쌍) | `o19.txt` `camp_pos` 6행 — blue/red 실측 |
| `mem[0]`/`mem[1]` | `data.cache`·`data.context` 양쪽이 결과를 바꾼다(cache=챔프·모드, context=map) |

**미상향** = `mem[2]`(GameContext+0x0 pool — 관측 가능한 출력이 없음) · `mem[14]`(bumpalo Vec ptr) ·
`knobs[6]`(`is_side_cleared` 의 tps*5 — 이 함수가 직접 호출하지 않음).

### `/specs[15]` `single_try_engage` — 13행 (함수 자체는 `in:game_ai` 라 미호출 · 구조체/피호출자로 실측)

```
A/size_of_LegacyPlanHandler 6168   A/team_plan +0xf8 size=1064   A/positioning_score +0x990 size=2760
B/size_of_PlayerState 2528         B/info.team +0x930            B/info.position +0x9c0
C/size_of_SinglePlanBattle 144     C/sub_goal +0x58              C/dive_tower +0x8c
D/tower0 id=2 ty_tag=2 info.ty@+0x128 =Top       D/tower2 ty_tag=2 info.ty@+0x128 =Top2
```
`mem[4]`0x930 · `mem[5]`0xf8 · `mem[6]`0x990 · `mem[7]`Entity+0x68 · `mem[8]`Entity+0x128 ·
`mem[11]`SPB+0x58 · `mem[12]`SPB+0x8c · `mem[13..15]` BattlePlanGoal 스택 0x0/0x8/0x10
(= `SinglePlanBattle::new` 의 Debug 가 `main_goal: TryKill(77, 60)` 을 그대로 보여준다) · `consts[0]`0 · `consts[1]`60 · `consts[3]`2.
★`mem[8]`/`consts[3]` 은 **4차 D-E3(`ty.Tower.info.ty`) 정정의 실행 확증**이다.

**미상향** = `mem[3]`(vtable+0x1f0) — `get_entity_by_id` 는 동작을 확인했지만 **슬롯 오프셋 자체**는 런타임에
못 읽었다(트레이트 메서드 주소를 얻는 경로가 없음) → `divtable` 근거로 ev3 유지.

### `/specs[18]` `v3_epicops_buff_window` — **강 상향 0행 · 오프셋만 12행**

이 함수는 `vis=in:game_ai`(tcx)이고 주요 피호출자 3개 중 2개(`is_object_being_taken_by_enemy` ·
`v3_epicops_repair_need`)도 `in:game_ai` 다. 상위 `pub` 진입점 `TeamPlan::update_objective` 로
**version 8 × 팀 2 × 포지션 5 = 80조합**을 돌렸으나 `objective`/`chats`/`eo_serpen_punish_issues` 가
**전부 불변**이었다(`o1518.txt` `E/uo`) — 세계에 에픽·세르펜도, 부상 챔프도, `group_line` 도 없다.
⟹ **판정 = "이 세계 구성으로는 도달 불가 · 미탐색 = 에픽/세르펜 스폰 + 부상 상태 + group_line 이 성립하는 세계"**
(불가가 아니다). 오프셋만 주소 산술로 실측했으니 `ev2(오프셋)` 로 분리 표기할 것을 제안한다:
```
E/size_of_TeamPlan 1064   E/chats +0xc0   E/objective +0x41f   E/eo_serpen_punish_issues +0x410
E/next_respawn_tick +0x378
B/info.team +0x930        B/info.position +0x9c0
```
대상 = `/specs[18]/mem[0,1,7,8,9,10,12,13,14,15,16,17]` (12행). **함수가 그 필드를 만지는 것은 미확인**이라
`ev2`(강)로 올리지 말고 별 등급으로 두는 것이 정직하다.

---

## 4. 새 발견 21건

| # | 내용 | 근거 |
|---|---|---|
| N1 | `Vec` 메모리 순서 = `cap`@+0 / `ptr`@+8 / `len`@+0x10. 빈 Vec 의 dangling ptr = 8 | `o17b.txt` `vec_empty` |
| N2 | `BattlePlanGoal::base_sub_goal` — `Response`/`Avoid`(tag≥2) → **`RunAway`(tag 4)**, 페이로드 워드는 미초기화 쓰레기 | `o17.txt` `C_Response`/`D_Avoid` |
| N3 | ★4차 R2 **실행 확증 12/12** — `is_ignored_well_enemy` 가 적팀 챔프 5/5 `true`, 아군 5/5 `false`, 타워 `false`, 미존재 `None`. `base_sub_goal` 의 `End` 는 정확히 그 술어가 true 일 때만 | `o17b.txt` `bsg` 12행 |
| N4 | 적 우물 위험 구역(팀0 기준) = 고x·저y 코너. `(913000,15000)`·`(900000,100000)`·`(959000,1000)` true / `(800000,200000)` 이후 false | `o16b.txt` `well` |
| N5 | `DeathMatchBattle` 의 Option 필드는 **태그만 쓰고 페이로드는 미초기화** — `+0x88`/`+0x98`/`+0xb0`/`+0xb8`/`+0xc8` 에 이전 스택 쓰레기가 남는다. 재구현이 payload 를 0으로 밀면 **바이트 비교가 깨진다**(동작 차이는 없음) | `o17.txt` `caseA` 헥스덤프 |
| N6 | `max_range_nearly_can_use` 전 슬롯·전 축 **122/122** `game==mine` | `o16.txt`/`o16b.txt` `TOTAL` |
| N7 | `radius_mult` 는 **i32 음수도 유효** — `−50` → `radius×50/100`(5490) | `o16.txt` `rm-50_0` |
| N8 | 실제 Nexus 엔티티(tag 3)는 **Champion 페이로드가 남아 있어도** 쿨다운 getter 4종이 전부 0을 돌려준다 = 게이트가 순수하게 `ty` 태그로만 갈린다 | `o16b.txt` `entcd nexus` 4행 |
| N9 | ★`is_cleared`(19 의 필터 술어) **완전식 확정 + 72/72** — `cleared = team_plan.next_respawn_tick[team][camp_idx] > distance(champ.x,champ.y,camp_pos)/champ.stat_cached.move_speed + offset + game.tick() + setting.tick_per_second`. 조기반환 2개(champ None / `!(tick < respawn)`) | `_gaibc\m04.ll:62404~62500` + `o19c.txt` `isc` |
| N10 | `team_plan::camp_idx` = Rhino **0** / Mushroom **1** / **Bee 2** / **Stump 3**, Morgard·Serpen 은 `unreachable!()` 패닉(team_plan.rs:40 — 프로브 1차 실행이 실제로 여기서 죽었다) ⟹ 19 의 배열 순서 `[Rhino,Mushroom,Bee,Stump]` 독립 확증 | `o19c.txt` `camp_idx` |
| N11 | 19 의 **817 폴백 도달** + 최소 `next_respawn_tick` 선택 4/4 + **동점 first-wins 2/2**(Rhino≻Mushroom, Bee≻Stump) | `o19c.txt` `FALLBACK`/`fbk` |
| N12 | 캠프 좌표 6쌍 실측. **Morgard(288000,288000)·Serpen(672000,672000) 은 좌우 미러가 아니라 양팀 동일** — `knobs[7]`("pos[1] 은 자동 미러") 의 예외 | `o19.txt` `camp_pos` + `o19b` `rawcamps` |
| N13 | 19 의 반환값이 Morgard/Serpen 일 수 있다 (§2 E2) | `o19.txt` `champNone_nowcamp` |
| N14 | ★`BattlePlanGoal::TryKill` 의 **두 번째 usize(15 `consts[1]`=60)는 생성 경로에서 완전히 死값**. 0/1/59/60/61/1000/999999 를 넣어도 `SinglePlanBattle::new`·`new_dive`·`new_region`·`DeathMatchBattle::new`·`base_sub_goal`·`chats` 가 **전부 동일**(저장된 `main_goal` 사본만 바뀐다). 미탐색 = `update`/`decide_*` 경유 | `o1518.txt` `C/trykill1` 7행 |
| N15 | `SinglePlanBattle::new_dive` 는 채팅이 **`Chat::BattleDive`** 이고 `with_dive=true`. `new_region` 은 **채팅 0개** + `region=Some(BattleRegion{center_x,center_y,range})` | `o1518.txt` `C/new_dive_debug`·`C/new_region_debug` |
| N16 | `SinglePlanBattle::update` 1틱 후 `sub_goal` = `RunAway`(미존재 타깃·실제 적 챔프 둘 다) — 15 `knobs[0]` 의 미채택 집합(3/4/7)에 걸리는 상태가 기본값 세계에서 실제로 나온다 | `o1518.txt` `C/after_update_sub_goal` |
| N17 | HARD_CC 제외 5종 (§2 보강) | `tcxdict --enum CCState` + `g06.ll` |
| N18 | 18 은 오라클 도달 불가(자신 + 피호출자 2/3 이 `in:game_ai`). 상위 진입점 80조합 무반응, `v3_serpen_contest_clear_win` 80/80 false | `o1518.txt` `E/uo` |
| N19 | ★오라클 함정 신규 2건 — (a) **`MapDef::camp_pos` 도 TLS 메모**(`CAMP_POS_MEMO`: `thread_local RefCell<(usize, [[Option<(u64,u64)>;2];8])>`, tcx). 단 **이번엔 거짓말하지 않았다**: 설정이 다른 두 `MapDef` 가 같은 좌표를 낸 것은 `MapDef.camps` 원본이 실제로 동일하기 때문(메모 우회 = `map.camps` 직독, 프로세스도 갈라 확인). (b) **default 챔프의 `stat_cached.move_speed == 1`** — `distance/move_speed` 형태 함수는 eta 가 거리 스케일로 폭주해 판별력이 죽고, **0 으로 만들면 `is_cleared` 가 div-by-zero 패닉**(`m04.ll:62533`) | `o19b_m1.txt`/`o19b_m2.txt` `rawcamps`·`o19c.txt` `champ_speed_before` |
| N20 | ★★**오라클 확장 일반 열쇠** — `game_core::Entity` 의 **41필드 전부 `pub`**(tcx). `ptr::read` 로 복제 → 필드를 원하는 값으로 조립(옛 값 drop 을 피하려 `ptr::write`, 끝에 `mem::forget`) → **`cache.player_champion` 도 `pub`** 이라 그 엔티티를 캐시에 꽂아 판단함수에 먹인다. 3차·4차가 「입력 판별력 부재」로 닫은 것을 여는 표준 수법 | `D5_o16.rs`·`D5_o19.rs`|
| N21 | ★`derive(Debug)` 가 **private 필드까지 이름과 값을 찍는다** — `DeathMatchBattle`(36필드, 그중 15개가 `in:game_ai::…::death_battle`) · `SinglePlanBattle`(13필드) · `LegacyPlanHandler`. `transmute` 가 필요 없다 | `o17.txt`·`o1518.txt` |

---

## 5. `ev<=3` 표본 재확인 — 뒤집힘 **0건**

| 항목 | 재확인 방법 | 결과 |
|---|---|---|
| `/specs[16]/sig/tcx` `fn(&Entity,&Entity,usize)->u64` | 그 시그니처로 **직접 호출·링크·실행** | ✓ (틀리면 컴파일 불가) |
| `/specs[17]/sig/tcx` `fn(usize,BattlePlanGoal,&OperationData,&PlayerState)->DeathMatchBattle` | 동일 | ✓ + `sret 384B` 실측 일치 |
| `/specs[19]/sig/tcx` 7인자 | 동일 | ✓ |
| `/specs[15]/sig/tcx` `vis=in:game_ai` | 호출 시도 → `pub` 아님이 그대로 확인 | ✓ (참) |
| `/specs[18]/sig/tcx` `vis=in:game_ai` | 동일 | ✓ (참) |
| `/specs[15]/sig/params[0]` `Option<SinglePlanBattle>(144B)` | `size_of::<SinglePlanBattle>()` | **144** ✓ |
| `/specs[15]/mem[5]` `LegacyPlanHandler+0xf8` | 주소 산술 | `+0xf8`, size 1064 ✓ |
| `/specs[15]/mem[8]` `Entity+0x128 = ty.Tower.info.ty` | 실제 타워 4개 | `+0x128` = `Top`/`Top2` ✓ |
| `/specs[15]/consts[3]` `EntityType 태그 2 = Tower` | `ty_tag` 직독 | `2` ✓ |
| `/specs[17]/mem[5]` `vtable+0x28 = tick` | `world.tick` 4값 | `start_tick` 4/4 추종 ✓ → `chk` 갱신 |
| `/specs[17]/mem[36..38]` Commit/Standard/Stand | Debug 이름 + 바이트 | 0/0/0 이고 이름 일치 ✓ |
| `/specs[17]/mem[41]` `Option<MainObjective>`(3B) 단일 | `+0x17b`=255, `+0x17c/d` 미기록 | ✓ |
| `/specs[19]/mem[6]` `vtable+0x40 = get_game_mode` | 폴백 도달 | `GameMode::Moba` 반환 ✓ → `chk` 갱신 |
| `/specs[19]/mem[7]` `MobaMode+0x18 = jungle_runner`(480B) | 폴백이 이 필드를 타고 최소값 선택 | 6/6 ✓ |
| `/specs[19]/consts[0]` `JungleType::Rhino = 0` | `tcxdict --enum` + `camp_idx(Rhino)=0` | ✓ |
| `/specs[18]/mem[4]` (`AbstractGameWithCache` 의 dyn 팻포인터 둘째 워드 = vtable, 816B) | **미재확인**(함수 도달 불가) | ev3 유지 |
| `/specs[15]/mem[3]` `vtable+0x1f0` | **미재확인**(슬롯 주소 획득 경로 없음) | ev3 유지 |
| `/specs[18]/mem[12,13,15,16]`·`consts[2]` | **미재확인** | ev3 유지 |
| `callees_unmatched` 판정 술어 혼입 | 15~19 전수 육안 | **0건**(3·4차와 동일) |

---

## 6. 게이트 실측 출력

```
$ cd /c/tfm2mods/MIG && PYTHONIOENCODING=utf-8 python -X utf8 specgate.py
## 명세 완결 조건 검사 (게임 0.5.8)
   G1 자기모순=0  G10 class 오분류=0  G2 호출부 전수=0  G3 형제 함수=0  G4 술어 시그니처=0
   G5 sig 정본 대조=0  G6 logic 미반영=0  G7 과열림=0  G8 표에 옛 값=0  G9 callees 오염=4
### [G9 callees 오염] 4건  ← 전부 `손확인`(07 target_bush / 13·14 line·nearest_enemy·position·team / 15 chats)
```
⟹ **제출 조건(G1~G8·G10 = 0) 충족.** `G9` 4건은 전부 `손확인` 등급(Rust 에서 필드 `chats` 와 메서드
`chats()` 가 공존해 기계로 못 가름) — 내 대역의 `[15] chats` 1건도 실제 `TeamPlan::chats` 필드 접근이라 정상.

```
$ PYTHONIOENCODING=utf-8 python -X utf8 _spec/audit4.py
미반영 0 · STALE 0 · ev불일치 0   → 전건 반영됨          (4차 36항목 회귀, D-E1~E5·R1·R2·N1·N2 전부 OK)

$ PYTHONIOENCODING=utf-8 python -X utf8 tcxaudit.py
총 551건  오귀속=0  밀림=0  부분일치=0  확인불가=18  OK=533

$ PYTHONIOENCODING=utf-8 python -X utf8 tcxaudit.py --prose _verify5/D/D5_REPORT.md
(아래 §6-b)
```
**불변 게이트 = `오귀속 0 · 밀림 0` — 유지.** `확인불가 18`은 전부 vtable 슬롯(구조체 아님)이고,
그중 2건(`17 vtable+0x28` · `19 vtable+0x40`)은 이번에 **런타임으로 해소**했으니 `chk` 문면만 바꾸면
다음 라운드에 `확인불가`가 16으로 내려간다.

### 6-b `--prose` 로 이 보고서를 검사한 결과 (델타)

```
$ PYTHONIOENCODING=utf-8 python -X utf8 tcxaudit.py --prose                      # 기준선(내 보고서 제외)
총 686건  오귀속=0  밀림=0  부분일치=1  확인불가=18  OK=667

$ PYTHONIOENCODING=utf-8 python -X utf8 tcxaudit.py --prose _verify5/D/D5_REPORT.md
총 694건  오귀속=0  밀림=0  부분일치=1  확인불가=18  OK=675
```
**델타 = 스캔 대상 +8행, `오귀속` +0 · `밀림` +0 · `부분일치` +0.**
`부분일치` +1 은 초판에서 내가 `/specs[15]/logic` 의 「cache 의 둘째 워드」 표기를 **오프셋 문면째로 인용**해 생긴 것이었다.
원인은 tcx 가 `AbstractGameWithCache.game` 을 `+0x0` 에서 시작하는 16B 팻포인터 한 필드로 보기 때문이고
(= 그 둘째 워드에서 *시작하는* 필드가 없다), **정본 쪽 `specs20.json` 의 같은 주장도 똑같이 `부분일치`로 잡힌다**
(기준선의 1건이 그것이다). 도구 한계이지 오귀속이 아니다.
그래서 이 보고서는 해당 문장을 "**`AbstractGameWithCache` 의 dyn 팻포인터 둘째 워드**" 로 바꿔
오탐을 만들지 않게 했다 — **재실행 결과 `부분일치=1`(정본 쪽 1건만)로 기준선과 동일**하다.
⚠**제안**: `tcxaudit` 가 팻포인터(16B `&dyn`)의 `+8` 을 `<필드>(+8)` 로 인식하는 예외를 넣으면
이 오탐이 영구히 사라진다(METHOD_MAP §2-C 「반복 오탐은 도구 결함으로 취급」).
이 보고서의 오프셋은 전부 **한 행에 하나씩** 적었다(브리핑 §5 · 4차 D-E5 의 교훈). 출력 = `gate_prose.txt`.

---

## 7. ★내 브리핑/지시문의 오류

1. **경로 오기 — `game_ai::is_ignored_well_enemy` 는 존재하지 않는다.**
   브리핑 §2⑤ 와 `METHOD_MAP.md` ⑥한계1 이 "크레이트 루트 재수출 항목"으로 `game_ai::is_ignored_well_enemy`
   를 열거하는데, rustc 가 **E0425 `cannot find function ... in crate game_ai`** 로 반려했다(`D5_o17b.rs` 1차 빌드).
   실제 경로 = **`game_ai::plan_legacy::old::is_ignored_well_enemy`** (`_tcx\game_ai.json`, `v=pub`).
   같은 목록의 `game_ai::is_enemy_well_danger` 는 **맞다**(이번에 그 경로로 호출·링크·실행 성공 — `o16b.txt` `well` · `o17b.txt` `well`).
   `game_ai::check_kill_die_tick` 은 **이번 라운드에 재확인하지 않았다**(4차 D 의 호출 성공 기록을 토대로 사용 — 범위 명시). ⟹ 「크레이트 루트 재수출」 3건 중 1건이 실은
   `plan_legacy::old` 재수출이다. 4차 D-R3 의 tcx 인용은 정확했는데 **승격 과정에서 모듈 경로가 잘렸다**.
2. **함정③(TLS 메모) 목록이 불완전하다.** `check_kill_die_tick` 만 적혀 있는데 `MapDef::camp_pos` 도
   `CAMP_POS_MEMO`(thread_local `RefCell`)를 쓴다. 이번엔 무해했지만(§4 N19a) **맵 데이터가 다른 두 맵을
   한 프로세스에서 재면 오염될 수 있다.** `TEMPLATE.rs` 함정 목록에 등재를 제안한다 —
   함께 적을 우회법 = **메모를 거치지 않는 원본 필드(`MapDef.camps`, pub)를 직독해 대조**.
3. **함정 목록에 빠진 것 — default 챔프의 `stat_cached.move_speed == 1`.** 함정④가
   "`ChampionInfo::default()` 는 액션 파라미터까지 0" 이라고만 적는데, **속도가 1(0이 아니다)** 이라는 것이
   별개로 중요하다. `distance/move_speed` 를 하는 함수(`is_cleared` 등)는 그대로 쓰면 eta 가 거리 스케일로
   폭주해 **모든 케이스가 한쪽으로 붙는다**(내가 o19b 에서 이걸로 폴백 도달에 실패했다). 그리고 0 으로
   바꾸면 div-by-zero 패닉이다.
4. **§1 표의 `ev4` 열은 `mem`+`consts`+`knobs` 만 센 값이다.** 내 대역 실측: `mem`+`consts`+`knobs` 의
   ev4 = 198 ✓ 이지만 `open`/`notes` 의 ev4 2행이 빠져 있고, `callees` 68행(전부 ev3)은 분모 216 에서도 빠져 있다.
   비율 91.7%(198/216) 자체는 맞으니 **오류라기보다 분모 정의를 표에 명시해 달라**는 요청이다.
5. **지시문의 "`18`(47행)" 은 맞다** — 실측 `mem` 18 + `consts` 8 + `knobs` 27 = 53행 중 ev4 = 13+7+27 = 47 ✓.
   `16`(47) · `17`(47) 도 일치. 다만 **`18` 은 이 대역에서 유일하게 오라클 진입이 막힌 함수**라
   「ev4 가 많은 함수부터」라는 우선순위를 그대로 따르면 시간을 가장 많이 잃는다. 다음 라운드 브리핑에는
   **`sig.vis` 를 우선순위 표에 같이 넣어 달라**(`pub` 인 16·17·19 가 실제로 128행을 냈다).
6. 지시문이 토대로 쓰라고 준 4차 결론 3건(`15` 판별식 `m05.ll:44249~44820` / `16` 33줄 ±0 / `17` 갈림 축)은
   **재조사하지 않았다**(지시대로). 그중 `17` 갈림 축만 실행으로 확증했고(§4 N3), 나머지 2건은 미재확인이다 —
   범위를 명시해 둔다.

---

## 8. 재현 방법

```bash
cd /c/tfm2mods/MIG
sh _verify3/build.sh _verify5/D/D5_o17.rs   && "$LOCALAPPDATA/Temp/tfm2_spanprobe/D5_o17.exe"
sh _verify3/build.sh _verify5/D/D5_o17b.rs  && "$LOCALAPPDATA/Temp/tfm2_spanprobe/D5_o17b.exe"
sh _verify3/build.sh _verify5/D/D5_o16.rs   && "$LOCALAPPDATA/Temp/tfm2_spanprobe/D5_o16.exe"
sh _verify3/build.sh _verify5/D/D5_o16b.rs  && "$LOCALAPPDATA/Temp/tfm2_spanprobe/D5_o16b.exe"
sh _verify3/build.sh _verify5/D/D5_o19.rs   && "$LOCALAPPDATA/Temp/tfm2_spanprobe/D5_o19.exe"
sh _verify3/build.sh _verify5/D/D5_o19b.rs  && "$LOCALAPPDATA/Temp/tfm2_spanprobe/D5_o19b.exe" m1   # m2 도
sh _verify3/build.sh _verify5/D/D5_o19c.rs  && "$LOCALAPPDATA/Temp/tfm2_spanprobe/D5_o19c.exe"
sh _verify3/build.sh _verify5/D/D5_o1518.rs && "$LOCALAPPDATA/Temp/tfm2_spanprobe/D5_o1518.exe"
PYTHONIOENCODING=utf-8 python -X utf8 _verify5/D/evup.py
```
모든 프로브가 첫 줄에 `setting_ok true` 를 찍는다(브리핑 함정① — `real_setting()` 사용).
TLS 메모(함정③) 해당 함수는 이 대역에 없다(`check_kill_die_tick` 경로를 타지 않는다). 그래도
`o16.txt` `reorder`(정·역방향 스윕 동일) 와 `o17.txt` `reorder`(호출 순서 역전) 로 순서 의존을 각각 확인했다.
