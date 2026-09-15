---

### `209` build_minion_wave_snapshot — 내 챔피언 반경 80000 안 적 미니언(≤12)마다 아군 소스(미니언·타워·정글몹)의 DPS 와 투사체 일회 피해로 5틱 간격 HP 궤적·예상 사망틱을 계산한 MinionWaveSnapshot(sret 2320B) 생성

| 항목 | 값 |
|---|---|
| id | `utils__build_minion_wave_snapshot` |
| 심볼 | `_RNvNtCshdEBA0ozCnw_7game_ai5utils26build_minion_wave_snapshot` |
| 소스 | `game-ai\src\utils.rs:611` |
| IR | `m04.ll` 49922~52222행 |
| 경로·가시성 | `game_ai::build_minion_wave_snapshot` · **pub** |
| 계층 | 기타 |
| exe | `d377a0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r17` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_core::PlayerState, &game_core::OperationData, usize, usize) -> game_ai::MinionWaveSnapshot
```

**TLS 메모 절(`sig.tls` · 정정 경로 `/specs[209]/sig/tls/<키>`)**

없음 — 본문 49922~52222 의 @anon 참조는 panic Location(.223~.232, .36)과 Entity 의 AbstractEntity vtable(.6)뿐, LocalKey::with / call_once fn-포인터 상수 0건

<details><summary>인자 5개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | &mut MinionWaveSnapshot (2320B, ptr %0) |  | 4 |
| 1 | 1 | player | &PlayerState (2528B, ptr %1) |  | 4 |
| 2 | 2 | data | &OperationData (24B, ptr %2) |  | 4 |
| 3 | 3 | prediction_depth | usize (i64 %3) |  | 4 |
| 4 | 4 | source_quality | usize (i64 %4) |  | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn build_minion_wave_snapshot(player: &PlayerState, data: &OperationData, prediction_depth: usize, source_quality: usize) -> MinionWaveSnapshot   // utils.rs:611~615
  let champ: &Entity = data.cache.player_champion[player.info.team][player.info.position as usize].unwrap();   // L617 (team bounds <2; None → unwrap_failed 패닉)
  let mut snapshot = MinionWaveSnapshot::default();   // L618 (전량 0, expected_death_tick 12개 = MAX)
  snapshot.current_tick = data.cache.game.tick();      // L619 vtable+0x28
  let range_sq = 6400000000;                           // L622 (80000²)
  let mut targets: bumpalo Vec<&Entity> = Vec::new_in(data.context.pool);   // L627
  let mut target_ids = [usize::MAX; 12];               // L628
  // ── 1) 적 미니언 수집 (L629~647)
  for e in data.cache.game.iter_entity() {             // L629 vtable+0x200; next() 호출 뒤 `snapshot.count > 11` 이면 break(=12개 채우면 중단)
    if e.ty.tag == 1 (Minion) && e.team != champ.team && e.can_target {   // L633 (IR 평가 순서 ty → team → can_target; TeamType derive PartialEq: tag 비교 → tag==0(Player) 이면 payload 비교)
      let dx = |champ.x - e.x|, dy = |champ.y - e.y|;   // L636 (unsigned 절대차, 인라인 헬퍼 L7~9)
      if dx*dx + dy*dy > range_sq { continue }          // L636  ⇒ 거리² ≤ 80000² 채택
      let traj = &mut snapshot.minions[snapshot.count]; // L640
      traj.entity_id = e.id; traj.current_hp = e.hp; traj.total_hp = e.stat_cached.hp;   // L641~643 (+0x5c0/+0x670/+0x628)
      target_ids[snapshot.count] = e.id;               // L644
      targets.push(e);                                  // L645 (bumpalo reserve_internal_or_panic)
      snapshot.count += 1;                              // L646
    }
  }
  if snapshot.count == 0 { return snapshot; }           // L649~650 (targets drop 후 memcpy 2320B)
  let target_count = snapshot.count;                    // L653
  let find_slot = |id| target_ids[..target_count].iter().position(|&t| t == id);   // L654~657 (target_ids 96B + target_count 를 값 캡처; target_count<13 슬라이스 가드)
  let mut dps_per_tick   = [0i64; 12];                  // L659
  let mut one_shot_damages = [[(0usize,0usize);16]; 12];   // L661 ((arrival_tick, dmg))
  let mut one_shot_count = [0usize; 12];                // L662
  // ── 2) 아군 소스의 지속 DPS (L665~687)
  for src in data.cache.game.iter_entity() {            // L665
    let target_id: Option<usize> = match src.ty.tag {   // L666 switch(+0x68)
      1 (Minion)        if src.team == champ.team => src.ty.info.nearest_enemy      (Minion+0x18 tag / +0x20)   // L667
      2 (Tower)         if source_quality != 0 && src.team == champ.team => src.ty.info.nearest_enemy.map(|t| t.1)  (Tower+0x18 tag / +0x28 = 튜플 .1)   // L668 (IR: sq==0 먼저 검사 → 팀 비교)
      7 (Ghoul)         if src.team == champ.team => nearest_enemy (Ghoul+0x18/+0x20)   // L669
      9 (Bear)          if src.team == champ.team => nearest_enemy (Bear+0x18/+0x20)    // L670
      10 (Eagle)        if src.team == champ.team => nearest_enemy (Eagle+0x0/+0x8)     // L671
      8 (SmallJiangshi) if src.team == champ.team => Some(src.ty.info.target_enemy) (SmallJiangshi+0x90, 항상 Some)   // L672
      _ => continue,                                    // (팀 불일치도 continue)
    };
    let Some(target_id) = target_id else { continue };  // L676
    let Some(slot) = find_slot(target_id) else { continue };   // L677
    let Some(atk_eff) = &src.attack_effect else { continue };  // L678 (+0x490, 니치 +0x4c0 i32 != -1)
    let dmg = atk_eff.expected_damage_target(data.context, src as &dyn AbstractEntity, targets[slot]);   // L679 (targets[slot] bounds check)
    let cooltime = max(src.attack_cooltime(), 1);        // L680 (attack_cooltime 반환 range ≥3 이라 max 는 사실상 무효)
    dps_per_tick[slot] += dmg / cooltime;                // L681 sdiv(i64; 오버플로 검사)
  }
  // ── 3) 투사체 일회 피해 (L688~721) — source_quality ≥ 2 일 때만
  if source_quality > 1 {                               // L688
    for p in data.cache.game.iter_projectile() {        // L689 vtable+0x210
      if let ProjectileMoveType::Target { speed, target_id } = &p.move_type {   // L690 (tag +0x40 == 6)
        let Some(slot) = find_slot(*target_id) else { continue };   // L691
        let e = targets[slot];                          // L692
        let Some(caster) = data.cache.game.get_entity_by_id(p.caster_id) else { continue };   // L693 vtable+0x1f0 (null=None)
        let dist = game_core::utils::distance(p.x, p.y, e.x, e.y);   // L694
        let arrival_tick = dist / max(*speed, 1) + 5;   // L695 (udiv 내림)
        let dmg = p.expected_damage_target(data.context, caster, e);   // L696
        if arrival_tick <= prediction_depth && one_shot_count[slot] < 16 {   // L697 (IR 순서: count<16 계산 후 and)
          one_shot_damages[slot][one_shot_count[slot]] = (arrival_tick, dmg);   // L698
          one_shot_count[slot] += 1;                    // L699
        }
      } else if source_quality != 2 {                   // L703 (sq>1 이므로 ≥3; 소스 표기 >=3 / >2 는 표기 불가)
        let Some(caster) = data.cache.game.get_entity_by_id(p.caster_id) else { continue };   // L705
        if caster.team != champ.team { continue }        // L706 (TeamType PartialEq)
        for slot in 0..target_count {                   // L707
          let e = targets[slot];                        // L708 (bounds check vs targets.len)
          if p.applyed_target.check_projectile(p, e)    // L709 (+0x12c CastingTarget)
             && p.is_in_orbit(e.x, e.y, e.radius())     // L709 (Entity::radius 인라인: radius_mult(+0x470)==0 ? radius(+0x680) : radius*(mult+100)/100)
          {
            let dmg = p.expected_damage_target(data.context, caster, e);   // L710
            if one_shot_count[slot] < 16 {              // L711 (slot<12 bounds check 선행)
              one_shot_damages[slot][one_shot_count[slot]] = (1, dmg);   // L712 arrival_tick = 1
              one_shot_count[slot] += 1;                // L713
            }
          }
        }
      }
    }
  }
  // ── 4) 슬롯별 HP 궤적 (L724~761)
  for slot in 0..target_count {                         // L724 (minions[slot] bounds <12)
    let traj = &mut snapshot.minions[slot];             // L725
    traj.dps_per_tick = dps_per_tick[slot];             // L726
    let num_checkpoints = min(prediction_depth / 5, 18);   // L728
    let mut hp: i64 = traj.current_hp;                  // L729
    let mut death_tick = usize::MAX;                    // L730
    for i in 0..num_checkpoints {                       // L732
      let tick_offset = (i + 1) * 5;                    // L733
      hp -= traj.dps_per_tick * 5;                      // L735
      let prev_tick = i * 5;                            // L738
      for j in 0..one_shot_count[slot] {                // L739 (16-way 전개; count>16 이면 bounds 패닉 — 도달 불가)
        let (arrival, dmg) = one_shot_damages[slot][j];   // L740
        if arrival > prev_tick && arrival <= tick_offset { hp -= dmg }   // L741~742 (반열린 (prev_tick, tick_offset])
      }
      traj.checkpoint_hp[i] = hp;                       // L746
      traj.checkpoint_count = i + 1;                    // L747
      if hp < 1 && death_tick == usize::MAX {           // L749 (IR slt 1 = `<= 0`; 첫 사망만)
        let prev_hp = if i == 0 { traj.current_hp } else { traj.checkpoint_hp[i-1] };   // L751
        if prev_hp > 0 {                                // L752
          let frac = prev_hp * 5 / (prev_hp - hp);      // L753 sdiv(오버플로 검사) — 선형 보간 틱
          death_tick = prev_tick + frac;                // L754
        } else {
          death_tick = tick_offset;                     // L755(추정 줄) — IR phi 에 tick_offset 상수/값 분기
        }
      }
    }
    traj.expected_death_tick = death_tick;              // L761
  }
  snapshot                                              // L764 (targets drop → 2320B memcpy → L765)

특기: prediction_depth < 5 면 num_checkpoints = 0 → 각 슬롯은 dps_per_tick 만 쓰고 expected_death_tick = MAX(IR 전용 분기 %190/194·520~580 unrolled). prediction_depth 5~9 면 체크포인트 1개(%193). 투사체 Target 분기의 arrival_tick 은 프로젝트 좌표→타깃 거리 기반이라 e 가 움직여도 재계산 안 함. TLS 접점 없음(anon 상수 전부 panic Location + Entity vtable @anon.6).
```

**`mem` 메모리 접근 37건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | IR gep 2352 — L617 player_champion[team] 인덱스(bounds <2) | 4 | OK |  |
| 1 | PlayerState | 0x9c0 | info.position@tag (i32) | r | IR gep 2496, zext → [pos] 인덱스(Position 5variant 라 bounds check 없음, entity.rs:581 인라인) | 4 | OK |  |
| 2 | OperationData | 0x0 | cache | r | &AbstractGameWithCache | 4 | OK |  |
| 3 | OperationData | 0x8 | context | r | &GameContext — L627 bump · L679/L696/L710 expected_damage_target 인자 | 4 | OK |  |
| 4 | GameContext | 0x0 | pool (&Bump) | r | targets: bumpalo Vec<&Entity>::new_in(bump) (L627, IR 초기 {ptr=8(dangling), bump, cap 0, len 0}) | 4 | OK |  |
| 5 | AbstractGameWithCache | 0x0 | game.data_ptr | r | &dyn AbstractGame | 4 | OK |  |
| 6 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r | vtable+0x28 tick()(L619) · +0x200 iter_entity()(L629·L665, sret 64B EntityIter) · +0x210 iter_projectile()(L689, sret 40B ProjectileIter) · +0x1f0 get_entity_by_id(id)(L693·L705) — divtable AbstractGame | 3 | OK |  |
| 7 | AbstractGameWithCache | 0x1e0 | player_champion[team][pos] | r | IR gep 480 — L617 .unwrap() (None 이면 unwrap_failed 패닉) | 4 | OK |  |
| 8 | Entity | 0x0 | team@tag (TeamType: 0=Player(usize) 1=Neutral) | r | L633 e.team != champ.team · L667~672 src.team == champ.team · L706 caster.team == champ.team — derive PartialEq 인라인(tag 비교 후 tag==0 이면 +0x8 payload 비교) | 4 | OK |  |
| 9 | Entity | 0x8 | team@Player.0 (usize) | r | 위 팀 비교의 페이로드 | 4 | OK |  |
| 10 | Entity | 0x68 | ty@tag (EntityType) | r | IR gep 104 — L633 ==1(Minion) · L666 switch 1 Minion/2 Tower/7 Ghoul/8 SmallJiangshi/9 Bear/10 Eagle | 4 | OK |  |
| 11 | Entity | 0x88 | ty@Minion.info.nearest_enemy@tag (Minion+0x18) — Ghoul/Bear 도 동일 오프셋(Ghoul+0x18/Bear+0x18) | r | IR gep 136 — L667/669/670 Option<usize> tag | 4 | OK |  |
| 12 | Entity | 0x90 | ty@Minion.info.nearest_enemy@Some.0 (Minion+0x20) — Ghoul/Bear 동일 | r | IR gep 144 — target_id | 4 | OK |  |
| 13 | Entity | 0x88 | ty@Tower.info.nearest_enemy@tag (Tower+0x18) | r | IR gep 136 — L668 Option<(usize,usize)> tag | 4 | OK |  |
| 14 | Entity | 0x98 | ty@Tower.info.nearest_enemy@Some.0.1 (Tower+0x28) | r | IR gep 152 — L668 은 튜플 .1 을 target_id 로 씀(.0 = Tower+0x20 은 안 읽음) | 4 | OK |  |
| 15 | Entity | 0x70 | ty@Eagle.info.nearest_enemy@tag (Eagle+0x0) | r | IR gep 112 — L671 | 4 | OK |  |
| 16 | Entity | 0x78 | ty@Eagle.info.nearest_enemy@Some.0 (Eagle+0x8) | r | IR gep 120 — L671 | 4 | OK |  |
| 17 | Entity | 0x100 | ty@SmallJiangshi.info.target_enemy (SmallJiangshi+0x90, usize) | r | IR gep 256 — L672 Option 아님(항상 Some(target_enemy)) | 4 | OK |  |
| 18 | Entity | 0x490 | attack_effect (Option<Effect> 56B) | r | IR gep 1168 — L678; None 니치 = +0x4c0 casting@tag(i32, IR 1216) == -1 | 4 | OK |  |
| 19 | Entity | 0x470 | stat_buff_cached.radius_mult (i32) | r | IR gep 1136 — L709 Entity::radius() 인라인(entity.rs:1511~1515): mult==0 ? radius : radius*(mult+100)/100 | 4 | OK |  |
| 20 | Entity | 0x680 | radius (usize) | r | IR gep 1664 — L709 is_in_orbit 3번째 인자 | 4 | OK |  |
| 21 | Entity | 0x5c0 | id | r | IR gep 1472 — L641 traj.entity_id · L644 target_ids[count] | 4 | OK |  |
| 22 | Entity | 0x670 | hp | r | IR gep 1648 — L642 traj.current_hp | 4 | OK |  |
| 23 | Entity | 0x628 | stat_cached.hp (최대 HP) | r | IR gep 1576 — L643 traj.total_hp | 4 | OK |  |
| 24 | Entity | 0x660 | x | r | IR gep 1632 — L636 거리² · L694 distance · L709 is_in_orbit | 4 | OK |  |
| 25 | Entity | 0x668 | y | r | IR gep 1640 | 4 | OK |  |
| 26 | Entity | 0x6b9 | can_target (bool) | r | IR gep 1721 — L633 세 번째 조건 | 4 | OK |  |
| 27 | Projectile | 0x40 | move_type@tag (ProjectileMoveType 니치 태그) | r | IR gep 64 — L690 ==6 Target (assume !=9: BouncingTarget 은 무태그라 9는 불가값) | 4 | OK |  |
| 28 | Projectile | 0x48 | move_type@Target.speed | r | IR gep 72 — L695 arrival = dist / max(speed,1) + 5 | 4 | OK |  |
| 29 | Projectile | 0x50 | move_type@Target.target_id | r | IR gep 80 — L691 find_slot(target_id) | 4 | OK |  |
| 30 | Projectile | 0xf8 | caster_id | r | IR gep 248 — L693/L705 get_entity_by_id | 4 | OK |  |
| 31 | Projectile | 0x100 | x | r | IR gep 256 — L694 distance(p.x,p.y,e.x,e.y) | 4 | OK |  |
| 32 | Projectile | 0x108 | y | r | IR gep 264 | 4 | OK |  |
| 33 | Projectile | 0x12c | applyed_target (CastingTarget 4B) | r | IR gep 300 — L709 CastingTarget::check_projectile(&self, p, e) | 4 | OK |  |
| 34 | MinionWaveSnapshot(sret) | 0x908 | current_tick | w | IR gep 2312 (로컬 %18 기준) | 4 | OK | game.tick() (L619) |
| 35 | MinionWaveSnapshot(sret) | 0x900 | count | w | IR gep 2304 | 4 | OK | 수집된 적 미니언 수 0..=12 (L646 +1) |
| 36 | MinionWaveSnapshot(sret) | 0x0 -> minions[slot] (192B stride) | MinionHpTrajectory | w | IR gep 144/152/160/168/176/184 (slot 기준). 나머지 슬롯·바이트는 Default(L618: memset 0 + expected_death_tick=-1) | 4 | 오귀속(사전은 다른 필드를 준다) | +0x90 entity_id=e.id(L641) · +0x98 current_hp=e.hp(L642) · +0xa0 total_hp=e.stat_cached.hp(L643) · +0xa8 dps_per_tick=Σ dmg/cooltime(L726) · +0x0 checkpoint_hp[i]=hp(L746) · +0xb0 checkpoint_count=i+1(L747) · +0xb8 expected_death_tick(L761) |

**`consts` 상수 19건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev | folded_from |
|---|---|---|---|---|---|---|
| 0 | 6400000000 | 622 | 임계 | range_sq = 80000² (= 2.5셀). L636 (dx²+dy²) > range_sq 이면 제외(경계 포함 = 이하 채택) | 4 |  |
| 1 | 2 | 617 | 태그 | team bounds check(<2) · L666 switch EntityType 태그 2 = Tower | 4 |  |
| 2 | 1 | 633 | 태그 | EntityType 태그 1 = Minion(e.ty.is_any_type_minion() 인라인 entity.rs:1261 — IR 은 ==1 만 비교) · L680 max(cooltime,1) · L695 max(speed,1) · L712 비-Target 투사체 arrival_tick = 1 · L747 checkpoint_count=i+1 · L699/713 count += 1 | 4 |  |
| 3 | 11 | 629 | 임계 | 첫 루프 종료 조건 count > 11 (= count >= 12 = MinionWaveSnapshot::minions 슬롯 상한). 소스 표기(>=12 / >11)는 표기 불가 | 4 | 12 |
| 4 | 12 | 654 | 임계 | target_ids[12]·dps_per_tick[12]·one_shot_damages[12]·one_shot_count[12] 배열 크기 = 슬롯 상한 · L725 minions[slot] bounds · [..target_count] 슬라이스 검사(target_count < 13) | 4 |  |
| 5 | 13 | 655 | 임계 | target_count < 13 = target_ids[..target_count] 슬라이스 범위 검사(slice_index_fail 가드, 실제 도달 불가) | 4 |  |
| 6 | 5 | 728 | 임계 | 체크포인트 간격(틱) — num_checkpoints = prediction_depth/5 (udiv) · L733 tick_offset=(i+1)*5 · L738 prev_tick=i*5 · L735 hp -= dps_per_tick*5 (IR: mul -5 후 add) · L753 frac = prev_hp*5/(prev_hp-hp) · L695 arrival += 5(투사체 도달 여유) | 4 |  |
| 7 | -5 | 735 | 계수 | hp -= dps_per_tick*5 가 `mul %dps, -5` + add 로 접힘(부호 흡수) | 4 | 5 |
| 8 | 18 | 728 | 임계 | num_checkpoints 상한 = checkpoint_hp[18] 배열 크기 (llvm.umin) | 4 |  |
| 9 | 16 | 697 | 임계 | one_shot_damages[slot] 원소 상한 — one_shot_count[slot] < 16 일 때만 push(L697·L711); 내부 루프 16-way 전개 | 4 |  |
| 10 | 6 | 690 | 센티널 | ProjectileMoveType 메모리 태그 6 = Target{speed,target_id} (tcxdict --enum: idx4, 니치 start 2) | 3 |  |
| 11 | 9 | 690 | 태그 | llvm.assume(tag != 9) — ProjectileMoveType 에서 9 는 무태그 variant(BouncingTarget) 자리라 절대 안 나옴(컴파일러 힌트, 판정 아님) | 4 |  |
| 12 | 7 | 666 | 태그 | EntityType 태그 7 = Ghoul (소스 후보) | 4 |  |
| 13 | 8 | 666 | 태그 | EntityType 태그 8 = SmallJiangshi | 4 |  |
| 14 | 10 | 666 | 태그 | EntityType 태그 10 = Eagle (9 = Bear 도 switch 케이스) | 4 |  |
| 15 | 100 | 709 | 계수 | Entity::radius() 인라인: radius*(radius_mult+100)/100 (entity.rs:1515) | 4 |  |
| 16 | -1 | 618 | 센티널 | expected_death_tick 초기값 usize::MAX(Default) · L628 target_ids memset 0xFF · L730 death_tick=MAX 센티널(L749 `death_tick == MAX` 로 첫 사망만 기록) · L678 Option<Effect> None 니치(i32 -1) · sdiv 오버플로 검사 | 4 |  |
| 17 | 0 | 649 | 임계 | count == 0 이면 조기 반환 · L668 source_quality == 0 이면 타워 제외 · L752 prev_hp > 0 · L1512 radius_mult == 0 | 4 |  |
| 18 | -9223372036854775808 | 753 | 태그 | i64::MIN — sdiv 오버플로 패닉 검사(컴파일러 삽입, L681/L753) | 4 |  |

**`knobs` 조정점 6건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 적 미니언 수집 반경 | utils.rs:622 (사용 :636) | 6400000000 | 80000²(2.5셀). 올리면 더 먼 웨이브 미니언까지 스냅샷에 들어가 (소비처의) 웨이브 위험/정리 판단 대상이 넓어진다 | 4 | 기존 |
| 1 | 체크포인트 간격 | utils.rs:728·733·735·738·753 | 5 | 5틱 단위 HP 예측. 줄이면 해상도↑(단 18칸 상한이라 예측 범위 = 5×18 = 90틱 → 비례 축소) | 4 | 기존 |
| 2 | 체크포인트 상한 / 슬롯 상한 / 일회피해 상한 | utils.rs:14·67·661 (배열 크기) → :728·:629·:697 | 18 / 12 / 16 | 구조체 레이아웃과 결합돼 있어 단독 변경 불가(MinionHpTrajectory 192B·MinionWaveSnapshot 2320B 가 바뀜) | 4 | 기존 |
| 3 | 투사체 도달 여유 | utils.rs:695 | 5 | arrival_tick = dist/speed + 5. 올리면 Target 투사체 피해가 더 늦은 체크포인트에 반영되고 prediction_depth 밖으로 밀려 탈락하기 쉬움 | 4 | 기존 |
| 4 | source_quality 게이트 | utils.rs:668·688·703 | 0: 타워 제외 / ≥2: 투사체 / ≥3: 비-Target 투사체 | 호출자가 주는 품질 단계. 낮출수록 DPS 과소평가(미니언이 더 오래 사는 것으로 예측) | 4 | 기존 |
| 5 | 비-Target 투사체 도달틱 | utils.rs:712 | 1 | 첫 체크포인트(틱 5)에 즉시 반영. 올리면 후속 체크포인트로 이동 | 4 | 기존 |

<details><summary>`callees` 피호출자 29건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | attack_cooltime | game_core::Entity::attack_cooltime | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1766 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | build_minion_wave_snapshot | game_ai::build_minion_wave_snapshot | pub | fn(&game_core::PlayerState, &game_core::OperationData, usize, usize) -> game_ai::MinionWaveSnapshot | game-ai\src\utils.rs:611 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 2 | check_projectile | game_core::CastingTarget::check_projectile | pub | fn(&game_core::CastingTarget, &game_core::Projectile, &game_core::Entity) -> bool | game-core\src\simulation\effect\type.rs:248 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 3 | default | <game_ai::GoalData as std::default::Default>::default | pub | fn() -> game_ai::GoalData | game-ai\src\goal_data.rs:10 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 988개 중 상위 3개 |
| 4 | default | <game_ai::EpicStance as std::default::Default>::default | pub | fn() -> game_ai::EpicStance | game-ai\src\goal_data.rs:133 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 988개 중 상위 3개 |
| 5 | default | <game_ai::EpicStanceData as std::default::Default>::default | pub | fn() -> game_ai::EpicStanceData | game-ai\src\goal_data.rs:145 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 988개 중 상위 3개 |
| 6 | distance | game_core::utils::distance | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:12 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 7 | expected_damage_target | game_core::Effect::expected_damage_target | pub | fn(&game_core::Effect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + , &game_core::Entity) -> usize | game-core\src\simulation\effect.rs:91 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 8 | expected_damage_target | game_core::Projectile::expected_damage_target | pub | fn(&game_core::Projectile, &game_core::GameContext, &game_core::Entity, &game_core::Entity) -> usize | game-core\src\simulation\projectile.rs:1287 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 9 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 10 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 11 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 12 | is_in_orbit | game_core::Projectile::is_in_orbit | pub | fn(&game_core::Projectile, u64, u64, u64) -> bool | game-core\src\simulation\projectile.rs:973 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 13 | iter_entity | game_core::AbstractGame::iter_entity | pub | fn(&Self/#0) -> game_core::EntityIter | game-core\src\simulation.rs:180 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 14 | iter_entity | <game_core::Game as game_core::AbstractGame>::iter_entity | pub | fn(&game_core::Game) -> game_core::EntityIter | game-core\src\simulation\game.rs:1822 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 15 | iter_entity | <game_core::SingleLaneGame as game_core::AbstractGame>::iter_entity | pub | fn(&game_core::SingleLaneGame) -> game_core::EntityIter | game-core\src\simulation\game.rs:3886 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 16 | iter_projectile | game_core::AbstractGame::iter_projectile | pub | fn(&Self/#0) -> game_core::ProjectileIter | game-core\src\simulation.rs:182 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 17 | iter_projectile | <game_core::Game as game_core::AbstractGame>::iter_projectile | pub | fn(&game_core::Game) -> game_core::ProjectileIter | game-core\src\simulation\game.rs:3760 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 18 | iter_projectile | <game_core::SingleLaneGame as game_core::AbstractGame>::iter_projectile | pub | fn(&game_core::SingleLaneGame) -> game_core::ProjectileIter | game-core\src\simulation\game.rs:4019 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 19 | nearest_enemy | game_core::Entity::nearest_enemy | pub | fn(&game_core::Entity) -> std::option::Option<usize> | game-core\src\simulation\entity.rs:1819 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 20 | position | game_core::PlayerAiContext::<'a, 'b, 'r>::position | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> game_core::Position | game-core\src\mod_ai.rs:587 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 21 | position | game_core::transfer::TeamRosterPlanningContext::position | pub | fn(&game_core::transfer::TeamRosterPlanningContext, game_core::Position) -> std::option::Option<&game_core::transfer::PositionPlanningContext> | game-core\src\transfer\roster_context.rs:234 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 22 | push | <game_core::Staff as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::staff | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\staff.rs:38 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 23 | push | <game_core::Athlete as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:434 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 24 | push | <game_core::Contract as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:2498 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 25 | radius | game_core::Entity::radius | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1509 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 26 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 27 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 28 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
</details>

⚠**미매칭 9개**: `break`, `drop  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `find_slot`, `llvm.assume`, `llvm.umax.i64`, `llvm.umin.i64`, `next  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 9개는 **전부 다른 함수**라 싣지 않는다`, `reserve_internal_or_panic`, `slice_index_fail`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m13.ll:45975) · **형제 0개** 

**`open` 8건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | Tower.nearest_enemy 가 Option<(usize,usize)> 인데 .1 을 target_id 로 쓴다 — .0(Tower+0x20) 의 의미(거리? 우선순위?) 미확인(game_core Tower 정의 미독). IR 은 +152 만 읽는다 | 4 |  |
| 1 | 표기 불가 | L633 세 조건의 소스 표기 순서: IR 평가 순서(ty → team → can_target)와 같다고 보나 column 정보 없어 표기 불가(동작은 확정) | 4 |  |
| 2 | 표기 불가 | L749 `hp < 1`(IR slt 1) 이 소스에서 `<= 0` 인지 `< 1` 인지 — 외연 동일, 표기 불가 | 4 |  |
| 3 | 표기 불가 | L703 `source_quality != 2` 가 소스에서 `>= 3` / `> 2` 중 무엇인지 — sq>1 문맥에서 외연 동일, 표기 불가. L668 `!= 0` 도 `> 0`/`>= 1` 과 동일 | 4 |  |
| 4 | 미탐색 | EntityIter/ProjectileIter 의 순회 순서(SlotMap 순?) — 수집 12개 초과 시 어느 미니언이 잘리는지는 이 순서에 의존. game_core 경계라 미독 | 4 |  |
| 5 | 미탐색 | Effect::expected_damage_target / Projectile::expected_damage_target / CastingTarget::check_projectile / Projectile::is_in_orbit / utils::distance 내부 — game_core 경계(시그니처·반환 의미만: 앞 둘 -> i64 피해량, check_projectile -> bool 적용대상 여부, is_in_orbit(x,y,radius) -> bool, distance -> i64 ≥0) | 4 |  |
| 6 | 미탐색 | Entity::attack_cooltime 반환이 range(i64 3, 0)(≥3) 인데 max(…,1) 이 있는 이유 — 소스 안전장치로 보임, 판정 무관 | 4 |  |
| 7 | 미탐색 | vtable 슬롯 이름은 divtable(ExpectedGame 판 vtable) 기준 — 런타임 구현체(Game/ExpectedGame)가 어느 쪽인지는 호출자 소관 | 3 |  |

**`notes` 2건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | L755 (else death_tick = tick_offset) 의 정확한 소스 줄 — IR phi 에 !dbg 가 L0 이라 줄번호 미확정(751~756 사이) | 4 | 사실 서술 |
| 1 | L636 거리² 헬퍼의 소스 파일(인라인 체인 L3147<7/8/9<636 — utils 류 `dist_sq` 로 추정, game_core 파일명 미확인). 동작(절대차 제곱합, u64)은 확정 | 5 | 사실 서술 |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

