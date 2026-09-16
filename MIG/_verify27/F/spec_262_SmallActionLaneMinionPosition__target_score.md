---

### `262` SmallActionLaneMinionPosition::target_score — 라인 미니언 후보 하나의 공격 매력도 점수(Option<i64>, 항상 Some·≥1): 킬 타이밍·HP%·이 미니언이 노리는 아군(타워/미니언)·사거리 내·거리 페널티·난수 0..=10

| 항목 | 값 |
|---|---|
| id | `SmallActionLaneMinionPosition__target_score` |
| 심볼 | `_RNvMNtNtCshdEBA0ozCnw_7game_ai12small_action11lane_minionNtB2_29SmallActionLaneMinionPosition12target_score` |
| 소스 | `game-ai\src\small_action\lane_minion.rs:294` |
| IR | `m11.ll` 43890~44235행 |
| 경로·가시성 | `game_ai::SmallActionLaneMinionPosition::target_score` · **in:game_ai** |
| 계층 | 기타 |
| exe | `e25030` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r18` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &game_core::OperationData, &game_core::PlayerState, &game_core::Entity, &game_core::Effect, &game_core::Entity, std::option::Option<&game_ai::MinionWaveSnapshot>, &mut rand::rngs::std::StdRng) -> std::option::Option<i64>
```

**TLS 메모 절(`sig.tls` · 정정 경로 `/specs[262]/sig/tls/<키>`)**

- `name`: 없음
- `role`: 없음 — 본문(43890~44235)에 `LocalKey`/`call_once`/`llvm.threadlocal.address`/fn-포인터 상수(`@anon.* = constant ptr @…call_once`) 참조 0. 유일한 @anon 참조는 @anon.dfa1f8a0e3a880d1d1859ec099fe0b0f.29(Entity as AbstractEntity vtable, m11.ll:35)로 TLS 아님
- `key`: 해당 없음
- `layout`: 해당 없음
- `invalidation`: 해당 없음
- `call_conditions`: 콜리 중 expected_damage_target(gc)·MinionWaveSnapshot::find·hp_at_tick 내부의 TLS 접점은 미열람(계약만) — 이 함수 자체의 접점은 0

<details><summary>인자 8개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | _version | usize (i64 %0) | DI 이름 `_version`(arg 1, !66257) · 본문 참조 0회(dbg_value 도 poison). IR 속성 없음 | 4 |
| 1 | 2 | data | &OperationData(24B) %1 | define 줄 속성: noalias noundef readonly captures(none) dereferenceable(24). +0 cache(→ AbstractGameWithCache · 그 +0 = &dyn AbstractGame 팻포인터 data_ptr/+8 vtable_ptr, m11.ll:44169~44174) · +8 context(&GameContext %11 · 그 +8 setting → GameSetting+0x12f8 tick_per_second). +0x10 blackboard 는 안 읽음 | 4 |
| 2 | 3 | _player | &PlayerState(2528B) %2 | DI 이름 `_player`(arg 3, !66259) · 본문 참조 0회(dbg_value poison). define 줄 속성: noalias readonly captures(none) — dereferenceable 없음(미사용이라 속성 축소) | 4 |
| 3 | 4 | champ | &Entity(1728B) %3 | define 줄 속성: noalias noundef readonly captures(address, read_provenance) dereferenceable(1728). 시전자. 읽는 필드: +0 team@tag · +8 team@Player.0 · +0x438 stat_buff_cached.range · +0x470 stat_buff_cached.radius_mult · +0x5c8 level · +0x640 stat_cached.move_speed · +0x660 x · +0x668 y · +0x680 radius. expected_damage_target 에 &dyn AbstractEntity(vtable @anon.dfa1f8a0e3a880d1d1859ec099fe0b0f.29 = Entity as AbstractEntity, m11.ll:35)로, range_adjust·Entity::distance·attack_speed_mult 에 &Entity 로 전달. DI 별칭 caster/other/self | 4 |
| 4 | 5 | atk | &Effect(56B) %4 | define 줄 속성: noalias noundef readonly captures(address, read_provenance) dereferenceable(56). 챔피언 기본공격 이펙트. 읽는 필드: +0x10 range · +0x18 growth_range · +0x20 start_timing. expected_damage_target/range_adjust 의 self | 4 |
| 5 | 6 | target | &Entity(1728B) %5 | define 줄 속성: noalias noundef readonly captures(address, read_provenance) dereferenceable(1728). 평가 대상 미니언. 읽는 필드: +0x68 ty@tag · +0x88 ty@Minion.info.nearest_enemy@tag · +0x90 ty@Minion.info.nearest_enemy@Some.0 · +0x470 radius_mult · +0x5c0 id · +0x628 stat_cached.hp · +0x660 x · +0x668 y · +0x670 hp · +0x680 radius. DI 별칭 self/f | 4 |
| 6 | 7 | wave_snapshot | Option<&MinionWaveSnapshot(2320B)> %6 | define 줄 속성: noalias noundef readonly captures(address) dereferenceable_or_null(2320). null = None(option.rs:1542 and_then 인라인, m11.ll:44014). Some 이면 MinionWaveSnapshot::find(self, target.id) 만 호출 — 필드 직접 읽기 0 | 4 |
| 7 | 8 | rnd | &mut StdRng(320B) %7 | define 줄 속성: noalias noundef align 16 dereferenceable(320) — readonly 없음(가변). gen_range 호출 사이트 정확히 1곳: m11.ll:44158 `StdRng::gen_range::<i64, RangeInclusive<i64>>(rnd, 0..=10)`(lane_minion.rs:347) — 모든 경로에서 무조건 1회(블록 %136 은 유일한 ret 경로). writes 는 StdRng 내부(ChaCha12 상태·라이브러리) 로 이 함수 본문의 직접 store 아님 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn target_score(_version, data, _player, champ, atk, target, wave_snapshot, rnd) -> Option<i64>   // lane_minion.rs:294
// 296
attack_damage = atk.expected_damage_target(data.context, champ as &dyn AbstractEntity, target)   // i64, 계약만
// 297
hp_ratio = target.hp * 100 / max(target.stat_cached.hp, 1)          // usize 산술(udiv)
// 298  attack_range = atk.range(champ) + atk.range_adjust(champ,target) + champ.radius() + target.radius()
//   Effect::range(caster) 인라인(effect.rs:26) = atk.range + caster.stat_buff_cached.range + (caster.level-1)*atk.growth_range
//   Entity::radius() 인라인(entity.rs:1511~1515) = if radius_mult==0 { radius } else { radius*(radius_mult+100)/100 }
// 299~302
walk_dist  = target.distance(champ).saturating_sub(attack_range)       // Entity::distance(target, champ) 계약만
walk_tick  = walk_dist / max(champ.stat_cached.move_speed, 1)
start_tick = atk.start_timing * 100 / max(champ.attack_speed_mult(), 1)
impact_tick = start_tick + walk_tick
// 306~316  타이밍 점수
score = 0; has_timing_reason = false
if let Some(traj) = wave_snapshot.and_then(|s| s.find(target.id)) {       // find: Option<&MinionHpTrajectory>
    predicted_hp = traj.hp_at_tick(impact_tick)                             // i64
    if predicted_hp > 0 && predicted_hp <= attack_damage + 5 {              // 308 (icmp sgt 두 번: >0 참·>dmg+5 거짓)
        score = 140                                                         // 309
    } else if predicted_hp > 0 && predicted_hp <= attack_damage * 2         // 311 (shl 1)
              && traj.expected_death_tick <= impact_tick + tps {            // 312
        score = 70                                                          // 313
    } else {                                                                // 315 (predicted_hp<=0 도 여기)
        score = if traj.expected_death_tick > tps * 2 { 0 } else { 25 }     // shl 1
    }
}   // 스냅샷 None 또는 find None → score 0 유지
// 320~326  현재 HP
if target.hp > attack_damage + 5 {                                          // 320 (usize ugt)
    if hp_ratio < 26 { score += 50 }                                        // 323~324
    else if hp_ratio < 51 { score += 25 }                                   // 325
} else { score += 90; has_timing_reason = true }                            // 321
// 329~338  이 미니언이 노리는 상대(= 내 편)
if target.ty@tag == Minion(1) && target.ty.Minion.info.nearest_enemy is Some(id) {   // 329 (select i1: && 단락)
    if let Some(enemy) = data.cache.game.get_entity_by_id(id) {             // 330 vtable+0x1f0
        if enemy.team == champ.team {                                        // 331 TeamType PartialEq 인라인
            if enemy.ty.is_tower() /* 태그 2 Tower · 3 Nexus */ { score += 80; has_timing_reason = true }   // 332~333
            else if enemy.ty@tag == Minion(1) { score += 25 }                // 335~336 (switch case 1)
        }
    }
}
// 342  이미 사거리 안
if target.distance_sq(champ) <= attack_range * attack_range { score += 20 }   // ugt → 가산 없음(select)
// 346~347
distance_penalty = utils::distance(champ.x, champ.y, target.x, target.y)     // 인자 순서 (x2,y2,x1,y1)=(champ, target)
score = score + distance_penalty / -3000 + rnd.gen_range(0..=10)             // sdiv(0 방향 절삭) · gen_range 1회·무조건
// 351~352
return Some(max(score, 1))                                                   // smax · 유일한 ret

※ has_timing_reason 은 DI 지역변수로만 존재(44013·44080·44228) — 반환·부작용에 소비처 0(사장 변수 또는 상위 인라인 전 잔재).
※ 평가 순서: 위 소스 줄 순서 그대로(IR 블록 %85→%117/%119→%127→%170…→%136). 320줄 hp 비교와 329줄 minion 분기는 독립 가산(else 아님).
```

**`mem` 메모리 접근 24건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | OperationData | 0x8 | context (&GameContext) | r | m11.ll:43910~43911 (%10/%11). expected_damage_target 2번째 인자 · setting 경유 tps 읽기 | 4 | OK |
| 1 | OperationData | 0x0 | cache (&AbstractGameWithCache) | r | m11.ll:44169 (%171). 그 +0 = &dyn AbstractGame 팻포인터: +0 data_ptr(%174) · +8 vtable_ptr(%173) — nearest_enemy 조회 경로(330)에서만 | 4 | OK |
| 2 | AbstractGameWithCache | 0x0 | game (&dyn AbstractGame · 팻포인터 16B) | r | m11.ll:44170~44174. vtable+0x1f0(496) = AbstractGame::get_entity_by_id(&self, id)->Option<&Entity>(divtable AbstractGame 0x1f0 · 일치율 98%) 를 44182 에서 간접 호출 | 3 | OK |
| 3 | GameContext | 0x8 | setting (&GameSetting) | r | m11.ll:44043~44044 (%95/%96) · 44065~44066 (%111/%112) | 4 | OK |
| 4 | GameSetting | 0x12f8 | tick_per_second | r | m11.ll:44045~44046 (%97/%98 · 315줄 `tps*2` 로 shl 1) · 44067~44068 (%113/%114 · 312줄 `tps + impact_tick`) | 4 | OK |
| 5 | Effect | 0x10 | range | r | m11.ll:43924~43925 (%20/%21). Effect::range(caster) 인라인(effect.rs:26) | 4 | OK |
| 6 | Effect | 0x18 | growth_range | r | m11.ll:43926~43927 (%22/%23). `(champ.level-1) * growth_range` | 4 | OK |
| 7 | Effect | 0x20 | start_timing | r | m11.ll:43997~43998 (%70/%71). `start_timing*100 / max(champ.attack_speed_mult(),1)` = start_tick(301) | 4 | OK |
| 8 | Entity | 0x670 | hp (target) | r | m11.ll:43914~43915 (%13/%14). hp_ratio 분자(297) · 320줄 `target.hp > attack_damage+5` | 4 | OK |
| 9 | Entity | 0x628 | stat_cached.hp (target) | r | m11.ll:43916~43917 (%15/%16). hp_ratio 분모 max(·,1) | 4 | OK |
| 10 | Entity | 0x5c8 | level (champ) | r | m11.ll:43928~43929 (%24/%25). 성장 사거리 계수 level-1 | 4 | OK |
| 11 | Entity | 0x438 | stat_buff_cached.range (champ) | r | m11.ll:43932~43933 (%28/%29). Effect::range 인라인의 caster 버프 사거리 가산 | 4 | OK |
| 12 | Entity | 0x470 | stat_buff_cached.radius_mult (champ 43935 / target 43958) | r | i32. Entity::radius() 인라인(entity.rs:1511~1515): 0 이면 radius 그대로, 아니면 radius*(mult+100)/100 | 4 | OK |
| 13 | Entity | 0x680 | radius (champ 43942·43949 / target 43965·43972) | r | attack_range 에 양쪽 반경 가산(298) | 4 | OK |
| 14 | Entity | 0x640 | stat_cached.move_speed (champ) | r | m11.ll:43991~43992 (%67/%68). walk_tick = walk_dist / max(move_speed,1)(300) | 4 | OK |
| 15 | Entity | 0x5c0 | id (target) | r | m11.ll:44020~44021 (%81/%82). MinionWaveSnapshot::find(snapshot, target.id)(306) | 4 | OK |
| 16 | Entity | 0x68 | ty@tag (target 44097 / enemy 44217) | r | target: `== 1`(Minion) 329줄. enemy(nearest_enemy 엔티티): switch 2(Tower)·3(Nexus) → +80 / 1(Minion) → +25 (332~336, EntityType::is_tower 인라인 entity.rs:1386) | 4 | OK |
| 17 | Entity | 0x88 | ty@Minion.info.nearest_enemy@tag (target) | r | m11.ll:44101~44107 (%132/%133 · trunc nuw to i1). Option<usize> 태그 0=None/1=Some — tcxdict Entity --deep 0x88 (Minion 페이로드 = enum+0x8 Minion.info+0x18) | 3 | OK |
| 18 | Entity | 0x90 | ty@Minion.info.nearest_enemy@Some.0 (target) | r | m11.ll:44175~44176 (%175/%176). get_entity_by_id 의 id 인자(330) | 4 | OK |
| 19 | Entity | 0x0 | team@tag (enemy 44190 / champ 44193) | r | TeamType PartialEq 인라인(entity.rs:1127, 331줄): 판별자 같고, Player(0) 이면 +8 페이로드까지 같아야 참 | 4 | OK |
| 20 | Entity | 0x8 | team@Player.0 (enemy 44210 / champ 44211) | r | 331줄 팀 비교 페이로드(팀 인덱스 usize). 판별자==0(Player) 일 때만 비교(cmp.rs:1878<2123) | 4 | OK |
| 21 | Entity | 0x660 | x (target 44114 / champ 44122) | r | 342줄 distance_sq(entity.rs:2158 → utils.rs:7~9 abs_diff²합) · 346줄 utils::distance 인자 | 4 | OK |
| 22 | Entity | 0x668 | y (target 44118 / champ 44126) | r | 342·346줄 | 4 | OK |
| 23 | MinionHpTrajectory | 0xb8 | expected_death_tick (traj) | r | m11.ll:44041~44042 (%93/%94 · 315줄) · 44063~44064 (%109/%110 · 312줄). find() 가 돌려준 &MinionHpTrajectory(192B) | 4 | OK |

**`consts` 상수 22건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev | folded_from |
|---|---|---|---|---|---|---|
| 0 | 100 | 297 | 계수 | hp_ratio = target.hp*100 / max(target.stat_cached.hp,1) — 백분율 스케일(m11.ll:43921). 같은 100 이 298줄 Entity::radius 인라인(43951·43953·43974·43976 `(mult+100)/100`) 과 301줄 `start_timing*100`(44003) 에도 쓰임 | 4 |  |
| 1 | 1 | 297 | 임계 | umax(·,1) 0-나눗셈 가드 — 297(stat_cached.hp) · 300(move_speed) · 301(attack_speed_mult) 세 곳(cmp.rs:1039 Ord::max 인라인). 351줄 smax(score,1) 은 결과 하한. 306줄 `shl 1`(44047·44058) 은 ×2 접힘(별도 항목) | 4 |  |
| 2 | 5 | 308 | 계수 | 킬 여유: `predicted_hp <= attack_damage + 5`(308, m11.ll:44053) → 140 / `target.hp > attack_damage + 5`(320, 44029) → 즉시 처치 불가 판정 | 4 |  |
| 3 | 140 | 309 | 산출값 | 예측 HP 가 도달 시점에 (0, attack_damage+5] 안 = 내 한 방에 죽는 타이밍 → 기본 점수 140(phi m11.ll:44027 [140,%102]) | 4 |  |
| 4 | 2 | 311 | 태그 | `attack_damage * 2` 가 `shl i64 %12, 1`(m11.ll:44058)로 접힘 · 315줄 `tps * 2` 도 `shl %98, 1`(44047)로 접힘 — 두 곳 모두 IR 에 리터럴 2 없음(value 는 shl 시프트량 1 이 아니라 소스값 2 로 등록·qcspec 경고 예상) | 4 | 2 |
| 5 | 70 | 313 | 산출값 | 예측 HP 가 attack_damage+5 초과 ~ attack_damage*2 이하이고 expected_death_tick <= impact_tick + tps(1초 안에 죽음) → 70(phi 44027 [70,%108]) | 4 |  |
| 6 | 25 | 315 | 산출값 | 그 밖(예측 HP<=0 · 또는 >2배 · 또는 죽음이 1초 뒤): expected_death_tick > tps*2 이면 0, 아니면 25(select 44049). 같은 25 가 325줄 `hp_ratio < 51 → +25`(44085) · 336줄 `enemy.is_minion → +25`(44232) 에도 쓰임 | 4 |  |
| 7 | 0 | 315 | 임계 | expected_death_tick > 2초 → 타이밍 점수 0(select 44049 참 쪽) · 306줄 스냅샷/궤적 없음 → 0(phi 44027) | 4 |  |
| 8 | 90 | 321 | 미상 | target.hp <= attack_damage + 5(지금 한 방) → +90, has_timing_reason=true(m11.ll:44078) | 4 |  |
| 9 | 26 | 323 | 임계 | `hp_ratio < 26`(= <=25%) → +50 (m11.ll:44074 icmp ult) | 4 |  |
| 10 | 50 | 324 | 미상 | HP 25% 이하 보너스(m11.ll:44090) | 4 |  |
| 11 | 51 | 325 | 임계 | `hp_ratio < 51`(= <=50%) → +25 (m11.ll:44084 icmp ult · else-if 이므로 26~50%) | 4 |  |
| 12 | 1 | 329 | 태그 | target.ty@tag == 1 = EntityType::Minion(tcxdict --enum EntityType 메모리태그 1) (m11.ll:44099) | 3 |  |
| 13 | 496 | 330 | 미상 | AbstractGame vtable 슬롯 오프셋 0x1f0 = get_entity_by_id(divtable AbstractGame 0x1f0) (m11.ll:44180) — 배열 stride 아님·간접호출 슬롯 | 3 |  |
| 14 | 0 | 331 | 태그 | TeamType 판별자 0 = Player(tcxdict --enum TeamType) — 판별자가 0 일 때만 +8 페이로드(팀 idx) 추가 비교(m11.ll:44200) | 3 |  |
| 15 | 2 | 332 | 태그 | switch case: EntityType 메모리태그 2=Tower · 3=Nexus → +80 (m11.ll:44219~44221, EntityType::is_tower 인라인 entity.rs:1386) | 4 |  |
| 16 | 3 | 332 | 태그 | EntityType::Nexus 태그(위 항목과 같은 블록 %196) | 4 |  |
| 17 | 80 | 333 | 미상 | target 미니언이 노리는(nearest_enemy) 엔티티가 내 팀 타워/넥서스 → +80, has_timing_reason=true(m11.ll:44226) — 구조물 지키기 우선 | 4 |  |
| 18 | 20 | 342 | 미상 | champ↔target distance_sq <= attack_range² → +20(select 44146: ugt 이면 가산 안 함) — 이미 사거리 안 | 4 |  |
| 19 | -3000 | 346 | 계수 | `utils::distance(champ, target) / -3000`(sdiv 44149) = 거리 3000 당 -1 점(0 방향 절삭) — 거리 페널티 | 4 |  |
| 20 | 10 | 347 | 산출값 | rnd.gen_range(0..=10) 상한(RangeInclusive end, m11.ll:44155 store) — 난수 가산 0~10 | 4 |  |
| 21 | 0 | 347 | 임계 | gen_range 하한(44153 store) · RangeInclusive exhausted 플래그 false(44157 store i8 0) | 4 |  |

**`knobs` 조정점 11건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 킬 여유 마진(예측 HP/현재 HP 와 attack_damage 비교) | lane_minion.rs:308·320 (m11.ll:44053·44029) | 5 | 올리면 한 방에 못 죽이는 미니언도 '지금 죽음'(+90/140)으로 봐서 공격적으로 CS 를 노린다. 내리면 정확히 데미지 이하일 때만 킬 판정 | 4 | 기존 |
| 1 | 킬 타이밍 기본 점수(도달 시 한 방) | lane_minion.rs:309 (m11.ll:44027 phi 140) | 140 | 올리면 곧 죽을 미니언 우선순위↑(다른 항 90/80/50 을 압도). 내리면 현재 HP·구조물 항이 상대적으로 세진다 | 4 | 기존 |
| 2 | 근접 킬 타이밍 점수(2배 이내·1초 안 사망) | lane_minion.rs:313 (m11.ll:44027 phi 70) | 70 | 올리면 아군 미니언이 곧 처리할 대상을 미리 잡아두는 경향↑ | 4 | 기존 |
| 3 | 먼 타이밍 점수 / 2초 컷 | lane_minion.rs:315 (m11.ll:44049 select 0/25 · shl tps,1) | 25 | 죽음이 2초(tps*2) 안이면 25, 아니면 0. 2 를 키우면 늦게 죽는 미니언에도 25 가 붙는다 | 4 | 기존 |
| 4 | 즉시 처치 보너스 | lane_minion.rs:321 (m11.ll:44078) | 90 | 올리면 지금 hp<=dmg+5 인 미니언(막타)을 더 강하게 선호 | 4 | 기존 |
| 5 | HP 25%/50% 이하 보너스 | lane_minion.rs:323~325 (m11.ll:44074·44084·44090·44085) | 26→+50 · 51→+25 | 임계(26/51)를 올리면 더 높은 HP 도 저체력으로 취급해 가산. 보너스(50/25)를 올리면 저체력 미니언 선호↑ | 4 | 기존 |
| 6 | 내 구조물을 때리는 미니언 보너스 | lane_minion.rs:333 (m11.ll:44226) | 80 | 올리면 타워/넥서스에 붙은 적 미니언 우선 정리(수비 성향↑) | 4 | 기존 |
| 7 | 내 미니언을 때리는 미니언 보너스 | lane_minion.rs:336 (m11.ll:44232) | 25 | 올리면 아군 미니언 웨이브 보호 성향↑ | 4 | 기존 |
| 8 | 사거리 안 보너스 | lane_minion.rs:342 (m11.ll:44145) | 20 | 올리면 이동 없이 칠 수 있는 대상을 더 선호(자리 고정 성향↑) | 4 | 기존 |
| 9 | 거리 페널티 분모 | lane_minion.rs:346 (m11.ll:44149 sdiv -3000) | -3000 | 절댓값을 내리면(예 -1500) 먼 미니언 감점이 2배 — 가까운 대상 선호↑. 올리면 거리 무시 경향 | 4 | 기존 |
| 10 | 난수 폭 | lane_minion.rs:347 (m11.ll:44155) | 10 | 0..=N 균등 난수 가산. 올리면 점수 동률 근처에서 선택이 더 무작위, 0 이면 결정적(⚠rnd 소비 횟수는 유지되므로 PRNG 시퀀스는 안 바뀜) | 4 | 기존 |

<details><summary>`callees` 피호출자 15건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | attack_speed_mult | game_core::Entity::attack_speed_mult | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:2433 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | distance | game_core::utils::distance | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:12 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 2 | distance | game_core::Entity::distance | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2161 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 3 | distance_sq | game_core::utils::distance_sq | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:6 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 4 | distance_sq | game_core::Entity::distance_sq | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2157 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 5 | expected_damage_target | game_core::Effect::expected_damage_target | pub | fn(&game_core::Effect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + , &game_core::Entity) -> usize | game-core\src\simulation\effect.rs:91 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 6 | find | game_ai::MinionWaveSnapshot::find | pub | fn(&game_ai::MinionWaveSnapshot, usize) -> std::option::Option<&game_ai::MinionHpTrajectory> | game-ai\src\utils.rs:84 | True | True | 3 | IR 호출 심볼 일치(토큰 경계) |
| 7 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 8 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 9 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 10 | hp_at_tick | game_ai::MinionHpTrajectory::hp_at_tick | pub | fn(&game_ai::MinionHpTrajectory, usize) -> i64 | game-ai\src\utils.rs:40 | True | True | 3 | IR 호출 심볼 일치(토큰 경계) |
| 11 | is_tower | game_core::EntityType::is_tower | pub | fn(&game_core::EntityType) -> bool | game-core\src\simulation\entity.rs:1385 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 12 | radius | game_core::Entity::radius | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1509 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 13 | range_adjust | game_core::Effect::range_adjust | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:29 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 14 | target_score | game_ai::SmallActionLaneMinionPosition::target_score | in:game_ai | fn(usize, &game_core::OperationData, &game_core::PlayerState, &game_core::Entity, &game_core::Effect, &game_core::Entity, std::option::Option<&game_ai::MinionWaveSnapshot>, &mut rand::rngs::std::StdRng) -> std::option::Option<i64> | game-ai\src\small_action\lane_minion.rs:294 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
</details>

⚠**미매칭 5개**: `gen_range`, `llvm.smax.i64`, `llvm.umax.i64`, `llvm.usub.sat.i64`, `range  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 5곳** (m06.ll:45430, m06.ll:48171, m11.ll:15097, m11.ll:27274, m11.ll:55503) · **형제 20개** (SmallActionLaneMinionPosition)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::SmallActionLaneMinionPosition as std::clone::Clone>::clone | pub | game-ai\src\small_action\lane_minion.rs:8 | True | fn(&game_ai::SmallActionLaneMinionPosition) -> game_ai::SmallActionLaneMinionPosition |
| 1 | <game_ai::SmallActionLaneMinionPosition as std::fmt::Debug>::fmt | pub | game-ai\src\small_action\lane_minion.rs:8 | True | fn(&game_ai::SmallActionLaneMinionPosition, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 2 | game_ai::SmallActionLaneMinionPosition::new | pub | game-ai\src\small_action\lane_minion.rs:139 | False | fn(&game_core::OperationData, usize, usize, i64, game_ai::PositionEvalPurpose) -> game_ai::SmallActionLaneMinionPosition |
| 3 | game_ai::SmallActionLaneMinionPosition::should_suppress_direct_minion_attack | pub | game-ai\src\small_action\lane_minion.rs:152 | False | fn(usize, &game_core::OperationData, &game_core::PlayerState, game_core::LineType, &game_core::Entity) -> bool |
| 4 | game_ai::SmallActionLaneMinionPosition::has_current_explicit_minion_action | in:game_ai | game-ai\src\small_action\lane_minion.rs:187 | False | fn(usize, &game_core::OperationData, &game_core::PlayerState, game_core::LineType, &game_core::Entity) -> bool |
| 5 | game_ai::SmallActionLaneMinionPosition::can_use_minion_action_now | in:game_ai | game-ai\src\small_action\lane_minion.rs:208 | False | fn(bool, std::option::Option<&game_core::Effect>, &game_core::Entity, &game_core::Entity) -> bool |
| 6 | game_ai::SmallActionLaneMinionPosition::direct_attack_stance | in:game_ai | game-ai\src\small_action\lane_minion.rs:212 | False | fn(&game_core::OperationData, &game_core::Entity, &game_core::Entity, u64) -> std::option::Option<(u64, u64)> |
| 7 | game_ai::SmallActionLaneMinionPosition::has_safe_minion_attack_stance | in:game_ai | game-ai\src\small_action\lane_minion.rs:226 | False | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, &game_core::Entity, game_core::LineType, u64) -> bool |
| 8 | game_ai::SmallActionLaneMinionPosition::is_safe_minion_attack_stance | in:game_ai | game-ai\src\small_action\lane_minion.rs:272 | False | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, &game_core::Entity, game_core::LineType, u64, u64, u64) -> bool |
| 9 | game_ai::SmallActionLaneMinionPosition::target_score | in:game_ai | game-ai\src\small_action\lane_minion.rs:294 | False | fn(usize, &game_core::OperationData, &game_core::PlayerState, &game_core::Entity, &game_core::Effect, &game_core::Entity, std::option::Option<&game_ai::MinionWaveSnapshot>, &mut rand::rngs::std::StdRng) -> std::option::Option<i64> |
| 10 | game_ai::SmallActionLaneMinionPosition::attack_range | in:game_ai | game-ai\src\small_action\lane_minion.rs:354 | False | fn(&game_core::Entity, &game_core::Entity) -> std::option::Option<u64> |
| 11 | game_ai::SmallActionLaneMinionPosition::local_position_score | in:game_ai | game-ai\src\small_action\lane_minion.rs:359 | False | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, u64, u64, game_ai::PositionEvalPurpose) -> i64 |
| 12 | game_ai::SmallActionLaneMinionPosition::lane_stance_risk | in:game_ai | game-ai\src\small_action\lane_minion.rs:380 | False | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, u64, u64) -> std::option::Option<i64> |
| 13 | game_ai::SmallActionLaneMinionPosition::push_candidate | in:game_ai | game-ai\src\small_action\lane_minion.rs:439 | False | fn(&mut bumpalo::collections::vec::Vec< (u64, u64, i64)>, usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, &game_core::Entity, &game_core::PositioningScoreData, u64, u64, game_ai::PositionEvalPurpose, u64, u64, i64, game_core::LineType, i64) |
| 14 | game_ai::SmallActionLaneMinionPosition::choose_goal | in:game_ai | game-ai\src\small_action\lane_minion.rs:493 | False | fn(&game_ai::SmallActionLaneMinionPosition, usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, &game_core::Entity, &game_core::PositioningScoreData, game_core::LineType) -> std::option::Option<(u64, u64, i64)> |
| 15 | game_ai::SmallActionLaneMinionPosition::get_input | in:game_ai | game-ai\src\small_action\lane_minion.rs:540 | False | fn(&mut game_ai::SmallActionLaneMinionPosition, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &mut game_core::DebugFrameData) -> std::option::Option<game_core::Input> |
| 16 | game_ai::SmallActionLaneMinionPosition::merge | in:game_ai | game-ai\src\small_action\lane_minion.rs:635 | False | fn(&mut game_ai::SmallActionLaneMinionPosition, usize, game_ai::SmallActionLaneMinionPosition) |
| 17 | game_ai::SmallActionLaneMinionPosition::get_action | in:game_ai | game-ai\src\small_action\lane_minion.rs:652 | True | fn(&game_ai::SmallActionLaneMinionPosition) -> game_core::SmallAction |
| 18 | game_ai::SmallActionLaneMinionPosition::is_end | in:game_ai | game-ai\src\small_action\lane_minion.rs:656 | False | fn(&game_ai::SmallActionLaneMinionPosition, &mut rand::rngs::std::StdRng, usize, &game_core::PlayerState, &game_core::OperationData) -> bool |
| 19 | game_ai::SmallActionLaneMinionPosition::near_move_complete | in:game_ai | game-ai\src\small_action\lane_minion.rs:685 | False | fn(&game_ai::SmallActionLaneMinionPosition, &game_core::Entity) -> bool |

**`open` 8건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 표기 불가 | 2 = `attack_damage*2`(311)·`tps*2`(315)가 `shl … 1` 로 접혀 IR 에 리터럴 2 가 없다 — constants 에 folded_from 으로 등록(qcspec C1 이 value 2 를 못 찾으면 경고 예상 · 근거 m11.ll:44047·44058). 소스가 `*2` 인지 `<<1` 인지는 표기 불가(동작 동일) | 4 |  |
| 1 | 표기 불가 | 308/311 줄의 `predicted_hp > 0` 이 두 조건에 각각 적혔는지, `if predicted_hp > 0 { … } else {315}` 로 감싼 것인지는 IR 이 %89→%92 직행으로 접어 표기 불가 — 동작(predicted_hp<=0 → 315 블록)은 확정 | 4 |  |
| 2 | 표기 불가 | 332 줄 `is_tower()`(entity.rs:1386 인라인)가 Tower\|Nexus 둘을 포함하는 정의인지, `is_tower()\|\|is_nexus()` 같은 한 줄 결합인지 표기 불가(switch 2·3 이 같은 블록 %196 · column 정보 없음). 동작은 확정: 태그 2 또는 3 → +80 | 4 |  |
| 3 | 미탐색 | 329 줄 `&&` 좌우 순서는 IR select(i1 %131, i1 %134, false) 로 좌=ty==Minion · 우=nearest_enemy.is_some() 으로 읽힘(select 는 양쪽 다 평가하지만 둘 다 순수 load 라 동작 무관) | 4 |  |
| 4 | 미탐색 | has_timing_reason(DI !66281 · lane_minion.rs:304 · i8 0/1) 의 소비처가 본문에 0 — 소스에서 로그/트레이스로만 쓰였거나 사장 변수. 반환에 영향 없음 | 4 |  |
| 5 | 미탐색 | 콜리 내부 미열람(계약만): expected_damage_target(&Effect,&GameContext,&dyn AbstractEntity(vtable @anon.…29),&Entity)->i64 · range_adjust(&Effect,&Entity,&Entity)->u64 · Entity::distance(&,&)->u64 · attack_speed_mult(&Entity)->u64 · MinionWaveSnapshot::find(&self(2320B), id)->Option<&MinionHpTrajectory(192B)>(m11.ll:46561) · MinionHpTrajectory::hp_at_tick(&self, tick)->i64(m11.ll:49805) · utils::distance(x2,y2,x1,y1)->i64 | 4 |  |
| 6 | 표기 불가 | distance_penalty 의 DI 바인딩이 sdiv 전 값(%161)에 붙어 있어 소스가 `let distance_penalty = distance(...); score += distance_penalty / -3000` 인지 `let distance_penalty = distance(...) / -3000` 인지 표기 불가 — 동작 동일 | 4 |  |
| 7 | 미탐색 | `_docs\game_ai.txt` 에 lane_minion/target_score 관련 개발자 주석 0건(grep lane_minion·LaneMinion·choose_goal·target_score·push_candidate·미니언 전부 0) | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

