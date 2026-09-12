---

### `03` defensive_crisis — target 주변 적 챔프를 추려 (2초내 죽을 위기, 곧 쓸 CC기 위협) 두 bool 을 낸다

| 항목 | 값 |
|---|---|
| id | `buff_value__defensive_crisis` |
| 심볼 | `_RNvNtCshdEBA0ozCnw_7game_ai10buff_value16defensive_crisis` |
| 소스 | `game-ai\src\buff_value.rs:17` |
| IR | `m10.ll` 33864~34294행 |
| 경로·가시성 | `game_ai::defensive_crisis` · **pub** |
| 계층 | 점수화·술어 |
| exe | **없음** — 「exe 에 독립 함수가 없다(인라인·`define internal fastcc`)」인지 **「조인 실패」**인지는 이 칸만으로 못 가른다. `dllmatch.py`·`name2rva.py` 로 확인하라 |
| 라운드 | 기준 `r6` · 통과 6회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, &mut game_core::DebugFrameData) -> game_ai::DefensiveCrisis
```

<details><summary>인자 6개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | version | usize | AI 버전. 이 함수 본문에선 분기에 안 쓰인다 — IR 은 `%0` 을 8B alloca `%11` 에 한 번 spill 하고(m10.ll:33874 `store i64 %0, ptr %11` + `#dbg_declare`) 그 포인터를 클로저 환경·피호출자에 넘길 뿐이다(값 비교 0건). 그대로 effect_cc_time / check_kill_die_tick / is_ignored_well_enemy 로 그대로 전달만 된다 | 4 |
| 1 | 2 | rnd | &mut rand::rngs::std::StdRng(320B, align16) | 본문에서 직접 안 씀. check_kill_die_tick 에만 넘김 | 4 |
| 2 | 3 | player | &PlayerState(2528B) | 판단 주체(아군) 플레이어. info.team 만 읽는다 | 4 |
| 3 | 4 | data | &OperationData(24B) | {0x0 cache: &AbstractGameWithCache(8840B), 0x8 context: &GameContext(64B), 0x10 blackboard: &[Blackboard;2]} | 4 |
| 4 | 5 | target | &Entity(1728B) | 위기 판정 대상 챔피언(보통 아군 자신). ★이 본문이 %4 에서 **직접 읽는 것은 +0x5c0 한 곳뿐**이다 — m10.ll:33981 `%46 = getelementptr inbounds nuw i8, ptr %4, i64 1472` → m10.ll:33982 `%47 = load i64, ptr %46` → m10.ll:33983 `player_by_champion_id(%18, i64 noundef %47)` ⟹ +0x5c0 = 챔피언 id. ~~좌표~~ 는 거짓: m10.ll:33864~34294 범위에 `ptr %4, i64 1632`(x=0x660)·`i64 1640`(y=0x668) gep 가 **0건**이다. %4 는 m10.ll:33929 `store ptr %4, ptr %33` 으로 클로저 환경에 담겨 넘어가므로 좌표 사용이 있다면 그 클로저 쪽이고 이 본문의 '직접'이 아니다 | 4 |
| 5 | 6 | debug | &mut DebugFrameData(224B) | 본문에서 직접 안 씀. check_kill_die_tick 에만 넘김 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn defensive_crisis(version, rnd, player, data, target, debug) -> DefensiveCrisis

[L19] tps = data.context(+0x8).setting(+0x8).tick_per_second(+0x12f8)
[L22] enemy_ix = 1 - player.info.team(+0x930) // ★L21 은 `let` 슬롯뿐이고 이 계산은 **L22** 다 // ult 2 이면 panic_bounds_check(len=2)

[L21~26] near_enemies: Vec<&Entity, &Bump> =
 data.cache.player_champion(+0x1e0)[enemy_ix] // [Option<&Entity>;5], stride 40
 .iter().filter_map(id) // = AbstractGameWithCache::iter_champions (Some 만)
 .filter(|e| { // ★클로저 본체는 담당 범위 밖:
 // m10.ll 55868~55976 (Entity::call_mut 심)
 [L23] r = plan_legacy::old::battle::max_range(e, target)
 [L24] if Entity::distance_sq(e, target) > (r + 30000)^2 { return false }
 // distance_sq = abs_diff(x)^2 + abs_diff(y)^2, 좌표는 Entity+0x660/+0x668
 [L25] if fight_model::is_ignored_well_enemy(version, player, e) { return false }
 // = (e.team(+0x0) == TeamType::Player(enemy_ix))
 // && path_finder::is_enemy_well_danger(version, player, e.x, e.y)
 // 즉 '적 진영 샘(well) 위험구역에 서 있는 적'은 세지 않는다
 [L26] return data.blackboard(+0x10)[enemy_ix]
 .is_recent_visible(data.cache.game(&dyn AbstractGame), player, e)
 })
 .collect_in(data.context.pool(+0x0)) // bumpalo Vec::from_iter_in

[L30] if near_enemies.len(+0x18) == 0 { return DefensiveCrisis{ die_imminent:false, cc_threat:false } }
 // ⚠LLVM 이 여기서 아래 두 계산을 통째로 건너뛴다(둘 다 false 로 접힘)

// ── 출력 1: die_imminent ───────────────────────────────
[L30] die_imminent = false
[L32] tp = data.cache.player_by_champion_id(target.id(+0x5c0)) // Option<&PlayerState>, null 검사
 if tp != null {
[L33] die = fight_check::check_kill_die_tick(
 version, rnd, data,
 judger = tp, // target 을 소유한 플레이어
 focus = target,
 enemy = near_enemies.clone(), // 같은 pool 에 복제
 towers = Vec::new_in(pool), // ★항상 빈 벡터 (ptr=dangling(8), cap=0, len=0)
 debug)
[L35] die_imminent = (die < tps << 1) // = die < tps*2 (2초 이내 사망)
 }
 // tp == null 이면 die_imminent 는 false 로 남는다

// ── 출력 2: cc_threat ──────────────────────────────────
[L41] cc_threat = near_enemies.iter().any(|e| {
[L43~45] ★L42 에는 명령이 0개다. 슬롯 3칸이 **L43(skill) / L44(skill2) / L45(ult)** 에 한 줄씩 있고,
// `ty == 13` 비교는 소스에 직접 있는 게 아니라 각 `Entity::*_cooldown()`(entity.rs:1775/1791/1806) 안에 있으며
// LLVM 이 CSE 해 **L43 하나로** 접었다.
[L43] (c1,c2,c3) = if e.ty(+0x68) == 13 /*Champion*/ {
 (e.ty.skill_cooldown(+0xb8), e.ty.skill2_cooldown(+0xc0), e.ty.ult_cooldown(+0xc8))
 } else { (0,0,0) }
 slots = [ (&e.skill_effect(+0x4c8), c1),
 (if e.level(+0x5c8) > 2 { &e.skill2_effect(+0x500) } else { &NONE }, c2),
 (if e.level > 4 { &e.ult_effect(+0x538) } else { &NONE }, c3) ]
[L47] slots.iter().any(|(opt, cool)| {
[L48] cool <= tps // 1초 안에 쓸 수 있는 스킬만
 && opt.is_some() // 니치: (*opt)+0x30 != -1
 && fight_check::effect_cc_time(version, opt.unwrap()).is_some()
 // 그 이펙트가 군중제어(CC) 시간을 갖는가
 })
 })
 // 세 슬롯은 IR 에서 완전히 펼쳐져 슬롯1→슬롯2→슬롯3 순으로 검사되고,
 // 하나라도 참이면 즉시 cc_threat=true 로 빠져나온다(남은 적은 안 본다).

[L53] return DefensiveCrisis{ die_imminent, cc_threat }

※ 부작용: 게임 구조체에 직접 쓰는 곳은 없다. rnd/debug 는 &mut 로 check_kill_die_tick 에
 넘어가므로 그쪽에서 변할 수 있고, near_enemies 는 bump pool 에 할당됐다가 반환 전 drop 된다.
```

**`mem` 메모리 접근 22건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | OperationData | 0x0 | cache | r | &AbstractGameWithCache. 적팀 챔프 배열·player_by_champion_id·game(dyn) 출처 · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 | OK |
| 1 | OperationData | 0x8 | context | r | &GameContext · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 | OK |
| 2 | OperationData | 0x10 | blackboard | r | &[Blackboard;2]. 클로저 캡처로만 쓰이고 실제 인덱싱은 call_mut 심(범위 밖)에서 일어남 · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 | OK |
| 3 | GameContext | 0x0 | pool | r | &bumpalo::Bump — near_enemies Vec 의 할당자 · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 | OK |
| 4 | GameContext | 0x8 | setting | r | &GameSetting(5432B) · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 | OK |
| 5 | GameSetting | 0x12f8 | tick_per_second | r | = tps. 이 함수의 두 시간 임계의 기준단위(IR 오프셋 4856) · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 | OK |
| 6 | PlayerState | 0x930 | info.team | r | usize. enemy_ix = 1 - team 으로 적팀 인덱스를 만든다(범위밖이면 panic_bounds_check(len=2)) · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 | OK |
| 7 | AbstractGameWithCache | 0x1e0 | player_champion | r | [[Option<&Entity>;5];2] (80B). [enemy_ix] 를 [5 x ptr] stride 40 으로 인덱싱 = iter_champions · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 | OK |
| 8 | AbstractGameWithCache | 0x0 | game (dyn AbstractGame 데이터 포인터) | r | cache+0x0/+0x8 = &dyn AbstractGame 의 (data, vtable). 클로저 캡처용으로만 로드 · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 | OK |
| 9 | Entity(target) | 0x5c0 | id | r | usize. player_by_champion_id 의 인자 (⚠team 이 아니라 엔티티 id) · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 | OK |
| 10 | Vec<&Entity>(bumpalo, 32B) | 0x0 | ptr | r | near_enemies 의 버퍼 시작 · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0) BUMPVEC ptr@+0x0 + _verify5/A/A5_o7.tsv `*(ptr)` == 원소0 주소 CONFIRMED) | 3 | OK |
| 11 | Vec<&Entity>(bumpalo, 32B) | 0x18 | len | r | 0 이면 즉시 (false,false) 반환. 루프 끝점 = ptr + len*8 · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0) ★BUMPVEC cap=8/len=3 로 len@+0x18 판별 ) | 3 | OK |
| 12 | Entity(near_enemies 원소) | 0x68 | ty (EntityType 태그) | r | i64, range [0,14). ==13 → Champion 이라야 쿨다운 3개를 읽는다 · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 | OK |
| 13 | Entity(near_enemies 원소) | 0xb8 | ty.Champion.skill_cooldown | r | EntityType enum+0x50. Champion 이 아니면 0 으로 취급 · tcx 정본 대조( A6_o6.tsv(실주소 차 보충, 10/10 MATCH) ty.Champion.skill/skill2/ult_cooldown = +0xb8/+0xc0/+0xc8 실주소 차) | 3 | OK |
| 14 | Entity(near_enemies 원소) | 0xc0 | ty.Champion.skill2_cooldown | r | EntityType enum+0x58 · tcx 정본 대조( A6_o6.tsv(실주소 차 보충, 10/10 MATCH) ty.Champion.skill/skill2/ult_cooldown = +0xb8/+0xc0/+0xc8 실주소 차) | 3 | OK |
| 15 | Entity(near_enemies 원소) | 0xc8 | ty.Champion.ult_cooldown | r | EntityType enum+0x60 · tcx 정본 대조( A6_o6.tsv(실주소 차 보충, 10/10 MATCH) ty.Champion.skill/skill2/ult_cooldown = +0xb8/+0xc0/+0xc8 실주소 차) | 3 | OK |
| 16 | Entity(near_enemies 원소) | 0x4c8 | skill_effect: Option<Effect>(56B) | r | 레벨 게이트 없이 항상 후보 · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 | OK |
| 17 | Entity(near_enemies 원소) | 0x4f8 | skill_effect.casting: CastingType(i32) | r | ⚠Option<Effect> 의 니치 판별자. == -1 이면 None (Effect 시작 +0x30) · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 | OK |
| 18 | Entity(near_enemies 원소) | 0x500 | skill2_effect: Option<Effect>(56B) | r | level > 2 일 때만 실제 필드, 아니면 정적 None(@anon...40, +0x30 이 0xFFFFFFFF) · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 | OK |
| 19 | Entity(near_enemies 원소) | 0x538 | ult_effect: Option<Effect>(56B) | r | level > 4 일 때만 실제 필드, 아니면 정적 None · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 | OK |
| 20 | Entity(near_enemies 원소) | 0x5c8 | level | r | usize. skill2/ult 슬롯 개방 게이트 · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 | OK |
| 21 | Effect | 0x30 | casting: CastingType(i32) | r | 슬롯2·3 의 Option 니치 판별자를 %ptr+48 로 읽는다(슬롯1 은 entity+0x4f8 로 접힘) · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 | OK |

**`consts` 상수 7건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 1 | 22 | 인덱스 | enemy_ix = 1 - player.info.team — 적팀 인덱스 ★**src_line 정정 : 22**(`%21 = sub i64 1, %20` @m10.ll:33897 의 `!dbg !40704` = `buff_value.rs:22`, 인라인 없음. L21 에는 `llvm.lifetime.start` 하나뿐)(대상 챔프 배열·블랙보드 둘 다 이 인덱스) | 4 |
| 1 | 13 | 43 | 태그 | EntityType 태그 13 = Champion ★**src_line 정정 : 43**(`icmp eq i64 %72, 13` @m10.ll:34074 사슬 = `entity.rs:1775(Entity::skill_cooldown)` ← `buff_value.rs:43` ← `buff_value.rs:41`. L42 는 함수 전체에서 명령이 **0개**다). ⚠이 비교는 소스에 직접 적힌 게 아니라 **각 `*_cooldown()` 접근자 안에 있고** LLVM 이 CSE 해 L43 하나로 접었다. 이 태그일 때만 스킬 쿨다운 3개를 실값으로 읽고, 아니면 (0,0,0) | 4 |
| 2 | 2 | 44 | 임계 | Entity.level > 2 여야 skill2_effect 슬롯이 열린다(entity.rs:1693 접근자 인라인 ★**src_line 정정 : 44** — `icmp ugt i64 %76, 2` @m10.ll:34081 사슬 = `entity.rs:1693` ← `buff_value.rs:44` ← `41`). 아니면 정적 None · 오라클 실행 확증( A6_o5.tsv / A6_o5_tps.tsv — level 2 → cc_threat=0 / level 3 → 1 (skill2 슬롯) — history[9] 지시 반영) | 2 |
| 3 | 4 | 45 | 임계 | Entity.level > 4 여야 ult_effect 슬롯이 열린다(entity.rs:1701 접근자 인라인 ★**src_line 정정 : 45** — `icmp ugt i64 %76, 4` @m10.ll:34086 사슬 = `entity.rs:1701` ← `buff_value.rs:45` ← `41`). 아니면 정적 None · 오라클 실행 확증( A6_o5.tsv / A6_o5_tps.tsv — level 4 → cc_threat=0 / level 5 → 1 (ult 슬롯) — history[9] 지시 반영) | 2 |
| 4 | -1 | 48 | 센티널 | Option<Effect> 니치 None 표식 — Effect+0x30(casting: CastingType) 이 -1 이면 스킬 이펙트 없음 · 오라클 실행 확증( A6_o5.tsv / A6_o5_tps.tsv — Option<Effect>::None 의 +0x30 = -1 실측(A6_o6)) | 2 |
| 5 | 1 | 35 | 태그 | die_imminent 임계 tps*2 가 `shl i64 %tps, 1` 로 접힘 — 리터럴 2 는 본문에 없다. 즉 '2초 안에 죽는가' · 오라클 실행 확증( A6_o5.tsv / A6_o5_tps.tsv — tps 1/6/30/60/120 스윕에서 die 는 60 고정, die_imminent 는 `die<tps*2` 로 정확히 갈림 — tps=30(60<60=false) 이 **strict `<`** 를 판별) | 2 |
| 6 | 1 | 48 | 태그 | effect_cc_time 이 돌려준 Option<usize> 의 Some 판별자(=1). is_some() 이 discr==1 비교로 인라인됨 · 오라클 실행 확증(7차 배치A: A7_o1.tsv OPTUSIZE — Option<usize> = 16B, Some(7).discr=1 / None.discr=0 실측. Some(0).discr 도 1 이라 history[9] 의 「Some(0) 도 is_some 이라 위협으로 센다」와 정합) | 2 |

**`knobs` 조정점 21건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 사망 임박 시간창 (die_imminent) | buff_value.rs:35 (IR m10.ll:34019 `shl i64 %17, 1`) | 2 | tps*2 틱 = 2초. 올리면 더 여유 있는 상황도 '곧 죽는다'로 보아 방어/후퇴 판단이 빨라지고, 내리면 진짜 마지막 순간에만 위기로 본다. 리터럴 2 는 shl 1 로 접혀 IR 에 없다 ★**실행 확증 **: 같은 판(die_tick=60 고정)에서 tps=60 → `60<120` **true** / tps=6 → `60<12` **false** / tps=1 → **false** (`A2_oracle3.tsv` dcrisisB) ⟹ 임계 `die < tps*2` 실행 확정. ⚠1차는 `GameSetting::default()` 의 `tick_per_second=0` 때문에 이 확인을 못 했다 | 2 | 기존 |
| 1 | CC기 준비 시간창 (cc_threat) | buff_value.rs:48 (IR m10.ll:34144 `icmp ugt i64 %85, %17`) | 1 | 쿨다운 <= tps(=1초)인 스킬만 위협으로 센다. 올리면 더 먼 미래의 CC 도 위협으로 잡아 과민해지고, 0 으로 내리면 지금 당장 쓸 수 있는 스킬만 본다 · 오라클 실행 확증( A6_o5.tsv / A6_o5_tps.tsv / A6_o6.tsv(실주소 차 보충, 10/10 MATCH) — cool<=tps 창: tps=60 → 60 통과/61 탈락, **tps=30 → 30 통과/31 탈락** (임계가 tps 를 따라간다) — history[9] 지시 반영) | 2 | 기존 |
| 2 | 주변 적 판정 반경 여유분 | buff_value.rs:24 — ★담당 범위 밖(m10.ll 55868~55976, call_mut 심 내부) | 30000 | 적 e 가 `max_range(e,target) + 30000` 안에 있어야 near_enemies 에 든다(제곱 비교). 셀 크기 32000 에 가까운 값 = 약 한 칸. 올리면 더 먼 적까지 위기 계산에 포함된다 ★실행 확증 : `game_ai::max_range(foe,me)=0` 인데 제곱거리 1,000,000 이 `(0+30000)^2 = 9e8` 이하라 **필터 통과** → die_imminent 계산 진입. `game_ai::max_range` 는 크레이트 루트 재수출로 **직접 호출 가능** | 2 | 기존 |
| 3 | skill2 슬롯 개방 레벨 | buff_value.rs:**44** 를 통해 인라인된 entity.rs:1693 (IR m10.ll:34081 `icmp ugt i64 %76, 2`) | 2 | Entity.level > 2 여야 skill2_effect 를 위협 후보로 본다. 내리면 저레벨 적의 2번 스킬도 CC 위협으로 계산된다 · 오라클 실행 확증( A6_o5.tsv / A6_o5_tps.tsv / A6_o6.tsv(실주소 차 보충, 10/10 MATCH) — level 2/3 경계 실측) | 2 | 기존 |
| 4 | ult 슬롯 개방 레벨 | buff_value.rs:**45** 를 통해 인라인된 entity.rs:1701 (IR m10.ll:34086 `icmp ugt i64 %76, 4`) | 4 | Entity.level > 4 여야 ult_effect 를 위협 후보로 본다. 내리면 저레벨 적의 궁극기도 CC 위협으로 계산된다 · 오라클 실행 확증( A6_o5.tsv / A6_o5_tps.tsv / A6_o6.tsv(실주소 차 보충, 10/10 MATCH) — level 4/5 경계 실측) | 2 | 기존 |
| 5 | towers 인자를 항상 빈 벡터로 넘김 | buff_value.rs:33 (IR m10.ll:34265~34269, `inttoptr(8)` + memset 0) | 0 | check_kill_die_tick 의 towers 목록이 비어 있어 '포탑 딜'은 사망 예측에서 빠진다. 여기에 실제 포탑을 채우면 포탑 사거리 안의 위기 판정이 훨씬 민감해진다 | 4 | 기존 |
| 6 | die_tick 의 초→틱 계수 | fight_check.rs:1107 = m15.ll:32956 (mul i64 %703, 60) | 60 | 전 사망예측 공통 배율. 올리면 AI 가 전반적으로 과감해지고 내리면 겁이 많아진다 · 오라클 실행 확증( A6_o5.tsv / A6_o5_tps.tsv / A6_o6.tsv(실주소 차 보충, 10/10 MATCH) — ★tps 를 1/6/30/60/120 으로 바꿔도 die 가 **60 으로 고정** ⟹ 이 60 은 tps 가 아니라 하드코딩 리터럴) | 2 | 신규 |
| 7 | judge_accuracy 기반 추정 오차 | `AthleteStat.judgement` = AthleteParameter+0x98 (= PlayerState+0x218) — tcx 확정(AthleteStat 232B, judgement +0x98; AthleteParameter.stat 는 +0x0). `judgement_base()` = `min(stat.judgement, 100)` 이라 **실효 상한 100** | 초기값 **1000**(`AthleteParameter::new`, g15.ll:126475 `store i64 1000`). 갱신식:  if dm_score_gap > 0 {  gap_norm = min(dm_score_gap, 3) * 1000 / 3;  degradation = (100 - min(stat.mental, 100)) * gap_norm / 100 * 110 / 100;  target = max(1000.saturating_sub(degradation), 10);  judgement_mental_ratio = min(judgement_mental_ratio, target); // ★min = 단조 감소(래칫)  } ★`_gcbc` 전 16파일 스캔 결과 `+0x2d0` 필드 store 는 **`new`(1000)와 이 한 줄 둘뿐** ⟹ 한 번 내려간 값은 update 로 절대 안 올라온다 | **멘탈 × 팀 점수차(dm_score_gap)** 두 축으로 판단 정확도를 깎는다 — '멘탈 나간 선수가 오판한다'의 구현 경로가 이 한 쌍이다. 오라클 진리표 **48/48 MATCH**(`_verify2\A\A2_oracle1.tsv`): mental>=100 → 어떤 gap 에서도 1000(무손상) / mental=80 → 928/865/802 / mental=50 → 829/667/505 / mental=0 → 667/334/**109**(사실상 최저). `min(gap,3)` 이라 **gap 3에서 포화**. 반복 호출 [1000,505,505,…] 로 되돌아오지 않는다(래칫 확인). ★**개입 지점 3개**: `min(gap,3)` 의 3 / `110/100` 의 110 / `max(..,10)` 의 하한 10 ★오라클 전수 진리표 : judgement 0/1/10/11/50/99/100/101/111/200/1000 → acc 100/109/190/199/550/991/1000×4. `judgement` 는 **100 에서 포화**, acc 는 **9 간격 이산**. 손계산 `min(judgement*jmr/1000,100)*9+100` 과 11/11 일치 | 2 | 신규 |
| 8 | 적 우물 위험구역 사각형 | m03.ll:144528~144558 | 팀0: (0,800000,64000,960000) ∪ (0,896000,160000,960000) / 팀1: 대칭 | 넓히면 적 우물 근처 적을 더 넓게 '무시 대상'으로 봐 다이브를 덜 시도 | 4 | 신규 |
| 9 | 적 위협 산정 정확도 폭 | m15.ll:31294~31297 (h = (1000-judge_accuracy)>>1) | h | h 를 0 으로 고정하면 난수 제거 → 적 dps/nuke 추정이 **선수 능력치와 무관하게 정확**해진다. 키우면 판단이 도박이 된다 | 4 | 신규 |
| 10 | 위협 판정 윈도우 | fight_check.rs:1005·1010·1015·1021 = m15.ll:31486·31586·31688·31790 (`icmp ugt i64 %N, %33` — %33 = tps, m15.ll:31242 `%33 = load i64, ptr %32` / %32 = setting+0x12f8) | tps | `tps*N` 으로 늘리면 "N초 안에 쓸 수 있는" 슬롯이 전부 위협으로 잡혀 **AI 가 훨씬 겁이 많아진다** | 4 | 신규 |
| 11 | RNG 버킷 길이 | m15.ll:31289 (bucket = tick / (tps*2)) | tps*2 (2초) | 키우면 판단이 더 오래 고정(플리커 감소), 작게 하면 매 틱 흔들린다 | 4 | 신규 |
| 12 | nuke 합산 방식 | m15.ll:31666 (llvm.umax) | max | max → sum 으로 바꾸면 다스킬 챔프의 위협이 폭증한다 | 4 | 신규 |
| 13 | 위협 DPS 의 분모 = 쿨타임 | _gcbc/g15.ll:105197 `%1008 = mul i64 %1007, %1003`(tps × expected_damage_target) · 105198 `%1009 = … Entity15attack_cooltime(%762)` · 105211 `%1015 = udiv i64 %1008, %1009` (dps = tps*dmg / attack_cooltime) — 다른 슬롯도 같은 3연 패턴 | 쿨타임 | 쿨감 조작이 **AI 의 적 위협 인식에 직접** 반영된다 | 4 | 신규 |
| 14 | stat_version 증분 무효화 | Entity+0x6b4 / g15.ll:105462 | u32 | 강제로 흔들면 캐시 전량 재계산 — 디버그·개입 훅 지점 | 4 | 신규 |
| 15 | 방어 관통식 | _gcbc/g06.ll:52772 | def = def*(100−pen)/100 | pen ≥ 100 이면 saturating 으로 def 0 | 4 | 신규 |
| 16 | 최종 피해식 | g06.ll:52706·52846 | max(1, dmg*100/(def+100)), AD·AP 따로 감산 후 합산 | 감산 곡선 자체를 바꾸는 지점 | 4 | 신규 |
| 17 | 최대체력 비례 4항 | BuffState +0xd0/+0xd8/+0xe0/+0xf0 | 평타%·자기최대체력%·스킬%·도트증폭 | %기반 피해의 원천 | 4 | 신규 |
| 18 | expected_target_hp_ratio(vt+0x38) | g06.ll:52505 | ad += target.hp*thr/100 | ★감산 **전에** 더해진다 | 4 | 신규 |
| 19 | 판단 능력치 원본 | `AthleteStat.judgement` = AthleteParameter+0x98 (= PlayerState+0x218) | 선수 능력치 | judge_accuracy 의 입력. 올리면 위협 추정이 정확해진다 — 2026-09-11 검증배치 A 신규 발굴(명세 어디에도 없었다) · 오라클 실행 확증( A6_o5.tsv / A6_o5_tps.tsv / A6_o6.tsv(실주소 차 보충, 10/10 MATCH) — PlayerState+0x218 = 80 (mkgame 이 넣은 AthleteStat.judgement) 실측(A6_o6)) | 2 | 신규 |
| 20 | **멘탈 -> 판단 곱수** | `AthleteParameter.judgement_mental_ratio` = param+0x2d0 (= PlayerState+0x450) | 곱한 뒤 `min(.., 100)` 포화 | **멘탈 상태에 따라 판단 정확도를 흔드는 곱수.** 이것 하나로 '멘탈 나간 선수가 오판한다'가 구현돼 있다. 고정하면 멘탈이 판단에 미치는 영향이 사라진다 — 2026-09-11 검증배치 A 신규 발굴. WARN 갱신 주체(`AthleteParameter::update`)는 **미탐색** · 오라클 실행 확증( A6_o5.tsv / A6_o5_tps.tsv / A6_o6.tsv(실주소 차 보충, 10/10 MATCH) — PlayerState+0x450 = 1000 (judgement_mental_ratio 초기값) 실측(A6_o6)) | 2 | 신규 |

<details><summary>`callees` 피호출자 25건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev |
|---|---|---|---|---|---|---|---|---|
| 0 | check_kill_die_tick | game_ai::check_kill_die_tick | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::OperationData, &game_core::PlayerState, &game_core::Entity, bumpalo::collections::vec::Vec< &game_core::Entity>, bumpalo::collections::vec::Vec< &game_core::Entity>, &mut game_core::DebugFrameData) -> usize | game-ai\src\fight_check.rs:917 | False | False | 3 |
| 1 | defensive_crisis | game_ai::defensive_crisis | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, &mut game_core::DebugFrameData) -> game_ai::DefensiveCrisis | game-ai\src\buff_value.rs:17 | False | False | 3 |
| 2 | distance_sq | game_core::Entity::distance_sq | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2157 | True | True | 3 |
| 3 | distance_sq | game_core::utils::distance_sq | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:6 | True | True | 3 |
| 4 | drop | <game_core::prof::ProfTimer as std::ops::Drop>::drop | pub | fn(&mut game_core::prof::ProfTimer) | game-core\src\simulation\prof.rs:184 | True | True | 3 |
| 5 | effect_cc_time | game_ai::effect_cc_time | pub | fn(usize, &game_core::Effect) -> std::option::Option<usize> | game-ai\src\fight_check.rs:379 | False | False | 3 |
| 6 | is_enemy_well_danger | game_ai::is_enemy_well_danger | pub | fn(usize, &game_core::PlayerState, u64, u64) -> bool | game-ai\src\path_finder.rs:1032 | False | False | 3 |
| 7 | is_ignored_well_enemy | game_ai::plan_legacy::old::is_ignored_well_enemy | pub | fn(usize, &game_core::PlayerState, &game_core::Entity) -> bool | game-ai\src\plan_legacy\old\fight_model.rs:754 | False | False | 3 |
| 8 | is_recent_visible | game_core::Blackboard::is_recent_visible | pub | fn(&game_core::Blackboard, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\game\blackboard.rs:346 | False | False | 3 |
| 9 | iter_champions | game_core::AbstractGameWithCache::<'a, 'b>::iter_champions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5759 ~ game_core[6a30]::simulation::{impl#5}::iter_champions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1904 | True | True | 3 |
| 10 | level | <game_ai::WindowStatView<'_> as game_core::AbstractEntity>::level | pub | fn(&game_ai::WindowStatView</#0>) -> usize | game-ai\src\fight_check.rs:96 | True | True | 3 |
| 11 | level | game_core::AbstractEntity::level | pub | fn(&Self/#0) -> usize | game-core\src\simulation\entity.rs:718 | False | False | 3 |
| 12 | level | <game_core::Entity as game_core::AbstractEntity>::level | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:772 | True | True | 3 |
| 13 | max_range | game_ai::max_range | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-ai\src\plan_legacy\old\battle.rs:2234 | False | False | 3 |
| 14 | player_by_champion_id | game_core::AbstractGameWithCache::<'a, 'b>::player_by_champion_id | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> std::option::Option<&'a/#0 game_core::PlayerState> | game-core\src\simulation.rs:1915 | True | True | 3 |
| 15 | skill2_cooldown | game_core::Entity::skill2_cooldown | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1789 | True | True | 3 |
| 16 | skill2_effect | game_core::Entity::skill2_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1692 | True | True | 3 |
| 17 | skill_cooldown | game_core::Entity::skill_cooldown | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1774 | True | True | 3 |
| 18 | skill_effect | game_core::Entity::skill_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1688 | True | True | 3 |
| 19 | team | game_core::PlayerAiContext::<'a, 'b, 'r>::team | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:583 | True | True | 3 |
| 20 | team | game_view::ClientDatabase::team | pub | fn(&game_view::ClientDatabase, usize) -> std::option::Option<&game_core::Team> | game-view\src\logic\client\data.rs:968 | False | False | 3 |
| 21 | team | game_view::ClientData::team | pub | fn(&game_view::ClientData, usize) -> std::option::Option<std::cell::Ref< game_core::Team>> | game-view\src\logic\client\data.rs:1651 | False | False | 3 |
| 22 | tick_per_second | game_view::view::effect::nightmare::tick_per_second | in:game_view::view::effect::nightmare | fn(&engine_core::assets::Assets) -> f32 | game-view\src\view\effect\nightmare.rs:31 | False | False | 3 |
| 23 | ult_cooldown | game_core::Entity::ult_cooldown | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1804 | True | True | 3 |
| 24 | ult_effect | game_core::Entity::ult_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1700 | True | True | 3 |
</details>

⚠**미매칭 3개**: `player_champion`, `pool`, `setting`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 3곳** (m05.ll:43115, m05.ll:43804, m05.ll:44220) · **형제 0개** 

**`open` 1건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | fnparts 기준 DWARF 서브프로그램 23개 중 별도 define 은 3개뿐 — 나머지 20개(iter_champions, distance_sq, abs_diff, is_some, is_some_and, TeamType PartialEq, Entity 접근자들 등)는 전부 인라인이라 개별 조각으로 존재하지 않는다. ★보강(6차 배치A): 별도 define 3개는 **본체(m10.ll:33864~34294) · call_mut 심(m10.ll:55868~55976) · 이터레이터(m01.ll:37431~37606, 175줄)** 인데 `history[0]` 의 `aux` 에는 **call_mut 심만** 올라가 있다 — m01.ll 쪽 이터레이터 조각은 아직 아무 라운드도 읽지 않았다(미탐색). | 3 |  |

<details><summary>`closed` 8건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

| # | 물음 | 닫은 근거 |
|---|---|---|
| 0 | ★30000(근접 반경 여유분)과 max_range / is_ignored_well_enemy / is_enemy_well_danger / Blackboard::is_recent_visible 호출은 전부 담당 줄범위(33864~34294) 밖이다 — 필터 클로저가 m10.ll 55868~55976 의 Entity::call_mut 심에 인라인돼 있다. SPEC_GUIDE '범위 밖 처리법'대로 constants/calls 에서 빼고 knobs·logic 에만 실었다. 지우지 않았음 | 3차: shared.is_recent_visible + 우물 판정 확정 |
| 1 | Blackboard::is_recent_visible 의 의미 — 인덱스가 blackboard[1 - player.info.team](=적팀 인덱스)인 것은 IR 로 확정했으나, self 가 '적팀이 보는 시야판'인지 '적팀 엔티티에 대한 관측판'인지는 game_core 의 DWARF 가 _gaibc 에 없어 확정 불가. 인자 순서는 declare 로 확인: (&Blackboard(744B), &dyn AbstractGame 의 data, 같은 dyn 의 vtable(816B), &PlayerState(2528B), &Entity(1728B)) | 4차 배치A 지적 → `shared.is_recent_visible.blackboard_인덱스_의미` 에 ★확정으로 답이 있다: `Blackboard[T].last_visible[pos]` = **팀 (1−T) 가 팀 T 의 pos 챔피언을 마지막으로 본 틱** (내 팀 블랙보드 = 적이 나에 대해 아는 것). 근거 = `Blackboard::update` 호출부 `_gcbc/g15.ll:249663` 이 team 0..1 로 돌며 자기 팀 챔프를 `is_visible(1-team, …)` 으로 묻는다 + game_ai 소비처 2곳 교차검증(`path_finder::enemy_knows_my_position` m03.ll:144778 · `v57_summon_command_score` _v57.ll:1~43) |
| 2 | die_imminent 의 곱수 2 는 `shl i64 %17, 1` 로 접혀 리터럴 2 가 본문에 없다. constants 에는 실제 존재하는 1(shift 량)로 등록하고 meaning 에 명시했다 | 4차 배치A: 본문이 스스로 '확인된 사실'이라 말한다 |
| 3 | near_enemies.len()==0 일 때 (false,false) 를 즉시 반환하는 블록(%43)의 !dbg 는 line 30 인데, ★해소(10차 배치A) — **소스에 `Vec::is_empty()` 호출이 실재한다.** len 로드 `%41`(m10.ll:33947 `%40 = getelementptr inbounds nuw i8, ptr %10, i64 24, !dbg !40760`)의 `!dbg` 사슬이 `vec.rs:1617 len ← vec.rs:1636 **is_empty** ← buff_value.rs:30` 이다. LLVM 이 빈 이터레이터의 `any()` 를 접은 것이라면 인라인 프레임에 `is_empty` 가 아니라 이터레이터 쪽 프레임이 남는다 ⟹ 두 번째 가설은 기각된다. ★범위: 「L30 에서 `is_empty()` 를 부른다」까지가 확정이고, 그 자리가 조기 `return` 인지 `let` 바인딩인지는 이 근거로는 안 갈린다(rmeta L30 내용 49자) | 본문에 해소 표기가 있다 |
| 4 | fight_check::effect_cc_time 내부는 안 봄(m15.ll:379~). 반환형이 Option<usize> 라는 것과 Some 이면 CC 로 친다는 것만 확인 | 3차 배치A: 단일 vtable 디스패치 + version 완전 미사용(오라클 1,150회) |
| 5 | fight_check::check_kill_die_tick 내부는 안 봄(m15.ll:917~). 반환값 die 가 '사망까지 남은 틱'이라는 것은 tps 와의 비교(die < tps*2)로 추정한 것이고, 함수 내부로 검증하지는 않았다 | 1차 배치A: resolved 에서 die_tick 공식 확정 + 2차에서 game_ai::check_kill_die_tick 직접 호출(반환 60) |
| 6 | version(p1) 이 이 함수 본문에서는 어떤 분기도 만들지 않는다 — effect_cc_time / check_kill_die_tick / is_ignored_well_enemy 로 전달만 된다. 버전 게이트는 그 안쪽에 있을 것으로 추정 | 3차: shared.is_recent_visible + 우물 판정 확정 |
| 7 | writes 가 빈 배열인 것은 미조사가 아니라 확인된 사실 — 본문의 store 는 전부 스택(클로저 환경 %9, 빈 Vec %7)과 bumpalo Vec 버퍼뿐이고 게임 구조체 필드에 쓰는 곳이 없다. 다만 rnd(&mut StdRng)·debug(&mut DebugFrameData) 는 check_kill_die_tick 에서 변경될 수 있다 → ★해소(6차 배치A): **변하지 않는다**. `defensive_crisis` 호출 전후로 `StdRng` 320B 와 `DebugFrameData` 224B 를 통째로 바이트 diff 한 결과 **14/14 케이스 전부 `rnd_changed=false · debug_changed=false`**(A6_o5.tsv). ★범위: `check_kill_die_tick` 이 실제로 실행된 케이스(die_imminent 계산 진입)만 잰 값이고, `debug`/`rnd` 를 쓰는 다른 입력이 있을 가능성까지 배제하지는 않는다. | 본문에 해소 표기가 있다 |
</details>

<details><summary>`history` 정정 이력 10건 (참조용 — 본문 아님)</summary>

| # | 옛 값 | 현재 | aux | filter | 상수 | 호출 |
|---|---|---|---|---|---|---|
| 0 | ★30000 과 max_range / is_ignored_well_enemy / is_enemy_well_danger / is_recent_visible 이 전부 담당 범위 밖 | aux 범위 m10.ll:55868~55976 (buff_value.rs closure$0 = line 22~26) 로 전부 확인. | [{"ir_file": "m10.ll", "ir_from": 55868, "ir_to": 55976}] | filter(c): dist_sq(c,target) <= (max_range(c,target) + 30000)^2 && !is_ignored_well_enemy(version,player,c) && is_recent_visible(blackboard[1-player.team], game, player, c) | [{"value": 30000, "ir": "m10.ll:55916", "meaning": "max_range(c,target) 결과에 더하는 여유분. 바로 다음 줄에서 제곱해 제곱거리와 비교"}] | ["battle::max_range(c, target)  @m10.ll:55883", "path_finder::is_enemy_well_danger(version, player, c.x, c.y)  @m10.ll:55948 — true 면 즉시 거른다", "Blackboard::is_recent_visible(...)  @m10.ll:55966 — 반환값이 그대로 필터 결과"] |
| 1 | is_ignored_well_enemy 를 calls 에 넣어야 하나 | ★정정 — 호출이 아니라 인라인이다(m10.ll:55934~55948, fight_model.rs:754). is_ignored_well_enemy(version,player,c) = if c.team == Player(1-player.team) { is_enemy_well_danger(version,player,c.x,c.y) } else { false }. calls 가 아니라 logic 에 서술해야 한다. |  |  |  |  |
| 2 | check_kill_die_tick 반환값이 '사망까지 남은 틱'이라는 것은 추정 | ★확정. _gaibc/m15.ll:27042 (fight_check.rs:917) 은 캐시 래퍼, 본체는 m15.ll:31120~33053. 최종식 die_tick = ((revive_hp + focus.hp) ⊖sat enemy_nuke) * 60 / max(enemy_dps,1) (m15.ll:32953~32995). focus.stat_buff_cached.undying(+0x488)이면 i64::MAX 반환. |  |  |  |  |
| 3 | towers 인자를 항상 빈 Vec 으로 넘기는 이유 | ★확정 — towers 루프(fight_check.rs:1049~1054 = m15.ll:32240~32420)가 타워마다 expected_damage_target 을 불러 enemy_dps 와 enemy_nuke 양쪽에 더한다. 빈 Vec 이면 타워 기여가 0 ⟹ die_tick 이 실제보다 길게(=안전하게) 나온다. 실측: 전 54개 호출부 중 빈 Vec 43개 / 실제 타워 11개. ★범위 한정 판정: 타워 딜은 '타워 다이브 판정 계열'(tower_dive_is_viable, single_tower_dive_is_viable, update_v32, action_candidates)에서만 들어가고 위기감지·후퇴태세에서는 전부 빠진다. |  |  |  |  |
| 4 | effect_cc_time 의 Some 값이 무엇인지 | ★확정 = CC 지속 틱. _gaibc/m15.ll:25822~25838(fight_check.rs:379)은 얇은 디스패처로 Effect 의 Arc<dyn EffectType> vtable +0x88 을 호출한다. 슬롯 실측(_gcbc/g04.ll:990): 0x80=expected_cc_time, 0x88=expected_cc_time_deep. ⟹ 부르는 건 deep 판(하위 이펙트까지 재귀해 max). StunEffect 구현이 Some(self.duration) 을 그대로 돌려준다(_gcbc/g04.ll:156490~156494). version 인자는 쓰이지 않는다. |  |  |  |  |
| 5 | CC 로 치는 이펙트가 무엇인지 | expected_cc_time* 가 무조건 Some 을 돌려주는 EffectType 구현(전 _gcbc 스캔: 항상 Some 63건 / 항상 None 324건 / 조건부·재귀 70건): Stun, Airborne, Knockback, Bind, Grab, Pull, Taunt, Fear, Charm, Banish, BlockAttack, BlockSkill, BlockMoveSkill, StatScaledBlockAttack, StatScaledBlockSkill + 챔피언 전용 DokkaebiUltHit, LightningMageUlt, WindMageUlt/Hit, HammererUlt, TaoistUltHit, HitmanSkill. ★행동 차단류 Block* 도 CC 로 계산된다. |  |  |  |  |
| 6 | enemy 챔피언 루프(fight_check.rs:1000~1045)의 슬롯별 가중치 상수 — 2회 시도 후 중단 | ★확정 — **슬롯별 상수 계수는 존재하지 않는다.** 가중치는 `ChampionCache` 테이블 룩업이고 곱해지는 것은 judge_accuracy 기반 난수 하나뿐이다. 전문 = `_shared.전투_위협모델`. ★누산 방식 정정: `nuke` = 챔피언 1명 안에서 **4슬롯의 max**, `enemy_nuke` += 그 max ⟹ **챔피언별 최대 한 방의 합**. `enemy_dps` 만 3슬롯×전 챔피언의 합. |  |  |  |  |
| 7 | ChampionCache 테이블을 채우는 곳(= 진짜 데미지 모델) — 이번 범위 밖 | ★확정. 전문 = `_shared.위협모델_ChampionCache`. `AbstractGameWithCache::new_with_prev_cache`(_gcbc/g15.ll:102716~108634)가 채우고 **`MobaBrain::run_input_phase` 에서 매 틱 재구축**된다. ★인덱스가 필드군마다 다르다(공격류=적팀 index, 힐/실드류=아군 index). ★`stat_version`(Entity+0x6b4) 기반 **증분 무효화**가 있어 안 바뀐 쌍은 재계산하지 않는다. |  |  |  |  |
| 8 | can_move() 의 런타임 구현체 — Arc<dyn> 이라 IR 로 불가(도구 한계) | ★**목록은 확정**(런타임 선택만 불가). 전문 = `_shared.can_move_구현체`. 총 134개 구현 / 항상 true **9개**(이전 "8개" 정정) / 조건부 16개. |  |  |  |  |
| 9 | `effect_cc_time` 내부와 version 의 역할 | ★확정(3차 배치A). `effect_cc_time` = `e.ty.<vt+0x88 = expected_cc_time_deep>()` **단일 vtable 디스패치**이고 **version 인자를 완전히 쓰지 않는다**(IR 등장 0건 + 오라클 1,150 호출 불일치 0). ★`cc_threat` 오라클 **44/44 실행 확증**: `level>2`(skill2)·`level>4`(ult)·`cool<=tps`(60 통과 / 61 탈락) ⟹ consts[2]·consts[3]·knobs[1] **ev 4→2**. CC 후보 = 실전 챔피언 60종 × 4슬롯 중 **41슬롯**(`Some(0)` 도 `is_some()` 이라 위협으로 센다). |  |  |  |  |
</details>

