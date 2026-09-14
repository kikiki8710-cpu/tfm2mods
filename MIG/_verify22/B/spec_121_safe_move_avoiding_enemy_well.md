---

### `121` safe_move_avoiding_enemy_well — 이동 목표(x,y)를 적 우물 위험이면 탈출점으로 바꾸고, 아니면 적 논타겟 투사체를 옆으로 비껴가는 조향점(approach_dodge_steer)으로 바꿔 Input::Move 를 만든다

| 항목 | 값 |
|---|---|
| id | `abstract_input__safe_move_avoiding_enemy_well` |
| 심볼 | `_RNvNtCshdEBA0ozCnw_7game_ai14abstract_input29safe_move_avoiding_enemy_well` |
| 소스 | `game-ai\src\abstract_input.rs:122` |
| IR | `m04.ll` 43378~43964행 |
| 경로·가시성 | `game_ai::safe_move_avoiding_enemy_well` · **pub** |
| 계층 | 입력 생성 |
| exe | `d34750` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r13` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, u64, u64) -> std::option::Option<game_core::Input>
```

<details><summary>인자 7개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | *mut Option<Input> (32B) noalias writable writeonly | returns 참조 | 4 |
| 1 | 1 | version | usize (i64 %1) | path_finder 콜리 3개에 전달 + :128 `version > 1` 분기(L43914) | 4 |
| 2 | 2 | player | &PlayerState (2528B) noalias readonly | info.team(0x930)·info.parameter(0x180) + 콜리 전달 | 4 |
| 3 | 3 | data | &OperationData (24B) noalias readonly | cache(+0)·context(+8). blackboard 미사용 | 4 |
| 4 | 4 | champ | &Entity (1728B) noalias readonly | 내 챔피언. x/y·radius·radius_mult·hp | 4 |
| 5 | 5 | x | u64 (i64 %5) | 이동 목표 x (approach_dodge_steer 에선 tx) | 4 |
| 6 | 6 | y | u64 (i64 %6) | 이동 목표 y | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn safe_move_avoiding_enemy_well(version, player, data, champ, x, y) -> Option<Input> {   // abstract_input.rs:122
  if is_enemy_well_danger(version, player, x, y) {                                   // :123 L43387 목표점이 적 우물 위험권
    let (cx, cy) = (champ.x, champ.y);                                                 // :124 L43876~43881
    if is_enemy_well_danger(version, player, cx, cy)                                  // :124 L43884 내 위치도 위험권
       || is_recent_enemy_well_damage_danger(version, player, champ) {                 // :125 L43891 (앞이 false 일 때만 평가 — 분기 순서 확정)
      let (ex, ey) = enemy_well_escape_position(ctx.setting, ctx.map, player, cx, cy);   // :126 L43895~43901
      return Some(Input::Move(ex, ey));                                                // :127 L43906~43910
    }
    if version > 1 {                                                                   // :128 L43914
      let (bx, by) = enemy_well_escape_position(ctx.setting, ctx.map, player, x, y);   // :133 L43922~43928 목표점 기준 탈출점
      if distance_sq(cx, cy, bx, by) > 4_000_000 /*2000²*/ { return Some(Input::Move(bx, by)); }   // :134~135 L43937~43962
      return None;                                                                     // :137 L43954
    }
    return None;                                                                       // :140 L43918 (v1: 위험 목표는 그냥 거부)
  }
  // :143  우물 위험이 아니면 투사체 회피 조향
  let (nx, ny) = approach_dodge_steer(player, data, champ, x, y);                     // 인라인 L43409~43859
  Some(Input::Move(nx, ny))                                                            // :144 L43868~43872
}

// ── approach_dodge_steer(player, data, champ, tx, ty) -> (u64,u64)   abstract_input.rs:54 (인라인)
  if dm_no_dodge() { return (tx, ty); }                                                // :54 L43409 (env DM_NO_DODGE 진단 스위치, OnceLock)
  let avoid = player.info.parameter.skill_avoid_effective();                           // :55 L43414 (0..=100, 컨디션 적용 실효치)
  let enemy_team = 1 - player.info.team;                                               // :56
  let (cx, cy) = (champ.x, champ.y);                                                   // :57
  let crad = champ.radius();  // radius_mult==0 ? radius : radius*(100+mult)/100     // :58 entity.rs:1511~1515
  let (ddx, ddy) = (tx - cx, ty - cy);                                                 // :59 (부호 있는 wrapping)
  let dlen = max(isqrt(ddx²+ddy²), 1);                                                 // :60
  if isqrt(ddx²+ddy²) < 2000 { return (tx, ty); }                                     // :61 L43467 (원시 isqrt 값, max 전)
  let (mut sx, mut sy) = (ddx*1000/dlen, ddy*1000/dlen);                               // :63~64
  let mut threatened = false;                                                          // :65
  for p in game.iter_projectile() {                                                    // :66 vtable+0x210, ProjectileIter::next (Option<&Projectile>, null=끝)
    if !(p.team == TeamType::Player(enemy_team) && p.is_visible) { continue; }        // :67 L43521~43576 (태그0 && +8==enemy_team && +0x131)
    if p.is_targeting() { continue; }                                                  // :68 projectile.rs:134 — Target | TargetSplash | BouncingTarget{target_id: Some}
    if !p.applyed_target.check_projectile(p, champ) { continue; }                     // :69 L43599 (내가 맞을 수 있는 대상군인가)
    let half = projectile_half_width(&p.shape);   // Rect → max(w,h)/2 ; Circle.radius / Line.width / DirDot.0 은 그대로   // :72 (:45~48)
    let rsum = 6000 + avoid*400 + crad + half;                                          // :72 L43488~43498, L43624
    let (perpx, perpy, pl);
    match p.move_type {                                                                 // :73 L43626~43631 (논리idx)
      LinearDist{target_x,target_y,..} => {                                             // :75~86
        let (lx, ly) = (target_x - p.x, target_y - p.y);                                // :75~76
        let remain = isqrt(lx²+ly²); let llen = max(remain, 1);                         // :77~78
        let along = ((cx - p.x)*lx + (cy - p.y)*ly) / llen;                             // :79 (투영 길이)
        if along < -crad || along > remain + crad { continue; }                         // :80 L43687~43691 (`or` 비단락)
        let af = along.clamp(0, remain);                                                // :81 L43694 Ord::clamp
        perpx = cx - (p.x + af*lx/llen); perpy = cy - (p.y + af*ly/llen);               // :82 (최근접점 → 나)
        pl = isqrt(perpx²+perpy²);                                                      // :83
        if pl <= 1500 {                                                                 // :85 L43717 (선 위에 거의 얹힘)
          // :86  접근방향(ddx,ddy)과 같은 쪽의 수직벡터 선택 (dot >= 0 ? (−ly, lx) : (ly, −lx)); pl 은 그대로(작은 값)
          (perpx, perpy) = if ddx*(-ly) + ddy*lx >= 0 { (-ly, lx) } else { (ly, -lx) };   // L43721~43732
        }
      }
      Delayed{..} | Periodic{..} | ApplyIn{..} => {                                     // :90~91 L43634~43650
        perpx = cx - p.x; perpy = cy - p.y; pl = isqrt(perpx²+perpy²);
      }
      _ => continue,                                                                    // Target/TargetSplash/FollowTargetShrinkBarrier/BouncingTarget/Parabolic/Removed
    }
    if !(pl < rsum) { continue; }                                                       // :95 L43748
    threatened = true;                                                                  // :96
    let plen = isqrt(perpx²+perpy²);                                                    // :97 L43756 (방향 정규화용, :86 갱신 반영)
    let caster = game.get_entity_by_id(p.caster_id);                                   // :99 vtable+0x1f0 (Option<&Entity>)
    let dmg = caster.map(|c| p.expected_damage_target(ctx, c, champ)).unwrap_or(0);   // :100 L43775 (i64)
    let value = dmg*100 / max(champ.hp, 1);                                             // :101 HP% (i64::MIN/−1 오버플로 검사 → panic L43799, 도달 불가)
    let value = max(if p.has_cc() {20} else {0}, value).clamp(0, 30);                   // :102 L43792~43804
    if value == 0 { continue; }                                                         // :105 L43795
    let caster_hit = cache.player_by_champion_id(p.caster_id)                           // :108 L43805
                       .map(|cp| cp.info.parameter.skill_hit_effective()).unwrap_or(50);   // :109 L43813~43821
    let eff = (avoid + 50 - caster_hit).clamp(0, 100);                                  // :110 L43823~43828
    let mag = (rsum - pl) * 40 * eff / max(rsum, 1) * value / 30;                       // :111 L43830~43838
    sx += mag * perpx / max(plen, 1);  sy += mag * perpy / max(plen, 1);                // :112~113 L43840~43846
  }
  if !threatened { return (tx, ty); }                                                   // :115 L43528~43533
  let slen = max(isqrt(sx²+sy²), 1);                                                    // :116 L43536~43542
  let nx = cx + sx*dlen/slen;  let ny = cy + sy*dlen/slen;                              // :117~118 (원래 목표 거리 dlen 만큼 조향 방향으로)
  Game::adjust_position(ctx.map, ctx.setting, nx, ny)                                   // :119 L43557

[game_core 경계 콜리 계약]
  dm_no_dodge() -> bool : env DM_NO_DODGE OnceLock (docs: '능동 회피 전부 끔')
  AthleteParameter::skill_avoid_effective/skill_hit_effective(&self) -> i64 0..=100 (range 속성; g15.ll:126167/126036)
  CastingTarget::check_projectile(&CastingTarget, &Projectile, &Entity) -> bool (g06.ll:86226)
  Projectile::expected_damage_target(&Projectile, &GameContext, caster:&Entity, target:&Entity) -> i64 (g07.ll:166394)
  Projectile::has_cc(&Projectile) -> bool (g07.ll:170248)
  utils::isqrt(i64) -> i64 (g06.ll:87607) · Game::adjust_position(&MapDef,&GameSetting,x,y) -> (x,y) (g15.ll:72245)
  AbstractGame vtable +0x210 iter_projectile(&self) -> ProjectileIter(40B) · +0x1f0 get_entity_by_id(&self,id) -> Option<&Entity>
[game_ai path_finder 콜리(지도 밖)] is_enemy_well_danger(version,&PlayerState,x,y)->bool (m03.ll:144500) · is_recent_enemy_well_damage_danger(version,&PlayerState,&Entity)->bool (m03.ll:146197) · enemy_well_escape_position(&GameSetting,&MapDef,&PlayerState,x,y)->(u64,u64) (m03.ll:144939)
```

**`mem` 메모리 접근 29건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x180 | info.parameter (AthleteParameter) | r | L43413~43414 skill_avoid_effective() (:55) | 4 | OK |  |
| 1 | PlayerState | 0x930 | info.team | r | L43416~43418 `1 - team` = enemy_team (:56) | 4 | OK |  |
| 2 | OperationData | 0x0 | cache (&AbstractGameWithCache) | r | L43391 | 4 | OK |  |
| 3 | OperationData | 0x8 | context (&GameContext 64B) | r | L43392~43394 (expected_damage_target 인자 L43775) · L43895~43896 · L43922~43923 | 4 | OK |  |
| 4 | GameContext | 0x8 | setting (&GameSetting) | r | L43555~43556 adjust_position · L43897~43898/L43924~43925 enemy_well_escape_position | 4 | OK |  |
| 5 | GameContext | 0x20 | map (&MapDef) | r | L43553~43554 · L43899~43900 · L43926~43927 | 4 | OK |  |
| 6 | AbstractGameWithCache | 0x0 | game.data_ptr (&dyn AbstractGame) | r | L43480 | 4 | OK |  |
| 7 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r | L43481~43484 슬롯 +0x210(528)=iter_projectile (sret 40B ProjectileIter) · L43490/L43762 슬롯 +0x1f0(496)=get_entity_by_id (divtable AbstractGame) | 3 | OK |  |
| 8 | Entity | 0x660 | x (champ) | r | L43420~43421 cx · L43876~43877 | 4 | OK |  |
| 9 | Entity | 0x668 | y (champ) | r | L43423~43424 cy · L43880~43881 | 4 | OK |  |
| 10 | Entity | 0x470 | stat_buff_cached.radius_mult (i32) | r | L43426~43429 Entity::radius 인라인 entity.rs:1511~1512: 0 이면 radius 그대로 | 4 | OK |  |
| 11 | Entity | 0x680 | radius | r | L43438~43439 / L43445~43449 `radius*(100+mult)/100` (entity.rs:1513/1515) → crad | 4 | OK |  |
| 12 | Entity | 0x670 | hp (champ) | r | L43492~43494 `max(hp,1)` — 예상피해를 HP% 로 (:101) | 4 | OK |  |
| 13 | Projectile | 0x0 | team@tag (TeamType) | r | L43521~43523 `== 0` Player (entity.rs:1127 derived eq, :67) | 4 | OK |  |
| 14 | Projectile | 0x8 | team@Player.0 (usize) | r | L43568~43569 `== enemy_team` | 4 | OK |  |
| 15 | Projectile | 0x131 | is_visible (bool) | r | L43573~43576 (:67) false 면 skip | 4 | OK |  |
| 16 | Projectile | 0x40 | move_type@tag (ProjectileMoveType, 니치) | r | L43580~43591 is_targeting 인라인(projectile.rs:134): 논리idx = tag>1 ? tag−2 : 7(BouncingTarget 암묵) — idx4 Target/5 TargetSplash → skip, idx7 이면 +0x40 Option<usize> target_id 태그==1(Some) → skip. L43626~43631 (:73) idx0 LinearDist / 1 Delayed / 2 Periodic / 3 ApplyIn 만 처리, 나머지 skip | 4 | OK |  |
| 17 | Projectile | 0x12c | applyed_target (CastingTarget) | r | L43598~43599 check_projectile(&target, p, champ) (:69) | 4 | OK |  |
| 18 | Projectile | 0x10 | shape@tag (ProjectileShape) | r | L43603~43609 projectile_half_width(:45) `== 2` Rect | 4 | OK |  |
| 19 | Projectile | 0x18 | shape 첫 페이로드 (Circle.radius \| Line.width \| Rect.width \| DirDot.0) | r | L43606~43607; Rect 외에는 이 값을 그대로 half_width 로 씀(L43623 phi) | 4 | OK |  |
| 20 | Projectile | 0x20 | shape@Rect.height | r | L43614~43619 `max(height,width) >> 1` (:48) | 4 | OK |  |
| 21 | Projectile | 0x68 | move_type@LinearDist.target_x (=0x40+0x28) | r | L43658~43659 (:75) | 4 | OK |  |
| 22 | Projectile | 0x70 | move_type@LinearDist.target_y | r | L43665~43666 (:76) | 4 | OK |  |
| 23 | Projectile | 0x100 | x (현재 위치) | r | L43634~43636 (:90) · L43660~43661 (:75) | 4 | OK |  |
| 24 | Projectile | 0x108 | y | r | L43641~43643 · L43667~43668 | 4 | OK |  |
| 25 | Projectile | 0xf8 | caster_id | r | L43760~43761 (:99) get_entity_by_id · L43805 (:108) player_by_champion_id | 4 | OK |  |
| 26 | Option<Input>(sret) | 0x0 | tag (i64) | w | L43868·L43906·L43958 store 0 ; L43918·L43954 store −1 | 4 | 확인불가(tcx 사전에 타입 없음) | 0 = Some(Input::Move) / −1 = None |
| 27 | Option<Input>(sret) | 0x8 | Move.x | w | L43870·L43908·L43960 (Some 경로만) | 4 | 확인불가(tcx 사전에 타입 없음) | 조향점 nx / 탈출점 ex / bx |
| 28 | Option<Input>(sret) | 0x10 | Move.y | w | L43872·L43910·L43962 (Some 경로만) | 4 | 확인불가(tcx 사전에 타입 없음) | ny / ey / by |

**`consts` 상수 19건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 1 | 128 | 태그 | `version > 1`(L43914 icmp ugt) — v2+ 만 목표점 자체를 탈출점으로 재조준. ★같은 1 이 :56 `1 - team`(L43418), :60/:78/:97/:111/:116 `max(·,1)` 0나눗셈 방지, :68 BouncingTarget target_id Some 태그(L43594) | 4 |
| 1 | 2000 | 61 | 임계 | approach_dodge_steer: 목표까지 거리(isqrt) < 2000 이면 조향 안 함(그대로 반환) L43467. 셀 1개=32000 이니 1/16 셀 | 4 |
| 2 | 1000 | 63 | 계수 | 초기 조향 벡터 정규화 배율 sx=ddx*1000/dlen, sy=ddy*1000/dlen (L43471~43476) — 회피 항(mag)과 같은 스케일(mag 최대≈40·100/1·30/30=4000) | 4 |
| 3 | 400 | 72 | 계수 | 위협 반경 rsum 의 회피 스탯 항 `avoid*400` (L43488) | 4 |
| 4 | 6000 | 72 | 계수 | 위협 반경 rsum 기본항 6000 (L43497) → rsum = 6000 + avoid*400 + crad + half_width | 4 |
| 5 | 100 | 58 | 임계 | Entity::radius 의 `radius*(100+mult)/100` (L43447~43449, entity.rs:1515). ★:101 `dmg*100/max(hp,1)` = HP% 변환(L43777) · :110 clamp 상한 100(L43828) | 4 |
| 6 | 1500 | 85 | 임계 | LinearDist 궤적선과의 수직거리 <= 1500 이면 '선 위' 로 보고 접근방향 기준 수직 방향으로 회피 방향을 잡음 (L43717 icmp sgt → 초과면 원래 수직벡터 유지) | 4 |
| 7 | 20 | 102 | 산출값 | CC 투사체는 예상피해와 무관하게 value 최소 20 (L43792 select has_cc ? 20 : 0) | 4 |
| 8 | 30 | 102 | 임계 | value 상한 clamp(0,30) (L43804 umin 30) · :111 `mag = … * value / 30` 정규화 분모(L43838) | 4 |
| 9 | 50 | 109 | 계수 | 캐스터 skill_hit 기본값 unwrap_or(50) (L43821 phi) · :110 `avoid + 50 - caster_hit` 의 +50 (L43496 %71) | 4 |
| 10 | 40 | 111 | 계수 | 회피 강도 계수 mag = (rsum−pl)*40*eff/max(rsum,1)*value/30 (L43831) | 4 |
| 11 | 4000000 | 134 | 임계 | 2000² — v2 재조준 탈출점이 내 위치에서 2000 초과 떨어져야 Some(Move), 아니면 None (L43950 icmp ugt) | 4 |
| 12 | 0 | 67 | 태그 | TeamType::Player 태그 0 (L43523) · :144/127/135 Some(Input::Move) 태그 0 · :102 CC 없음 0 · :110 clamp 하한 | 4 |
| 13 | -1 | 137 | 센티널 | Option<Input>::None 니치 태그 (L43918 :140, L43954 :137; tcxdict Option<Input> niche_start=2^64−1) | 3 |
| 14 | 2 | 45 | 센티널 | ProjectileShape::Rect 태그 2 (L43608) → half = max(w,h)/2. ★L43584 `add −2`·L43585 `ugt 1` 는 ProjectileMoveType 니치 디코드(niche_start=2) | 4 |
| 15 | 4 | 68 | 태그 | ProjectileMoveType 논리idx 4 = Target (is_targeting → skip, L43588) | 4 |
| 16 | 5 | 68 | 태그 | 논리idx 5 = TargetSplash (skip, L43589) | 4 |
| 17 | 7 | 68 | 태그 | 논리idx 7 = BouncingTarget(암묵 untagged; L43586 select 기본값) — target_id Some 이면 skip(L43594) | 4 |
| 18 | 3 | 73 | 태그 | 논리idx 1 Delayed/2 Periodic/3 ApplyIn → 투사체 현재 위치 기준 수직벡터(:90~91, L43628~43630). 0 LinearDist → 궤적선 기준(:75~86) | 4 |

**`knobs` 조정점 10건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 위협 반경 기본항 | abstract_input.rs:72 (L43497) | 6000 | 올리면 더 멀리 지나가는 투사체도 회피 대상 → 더 자주/크게 비껴감 | 4 | 기존 |
| 1 | 위협 반경의 회피 스탯 계수 | abstract_input.rs:72 (L43488) | 400 | avoid 100 이면 +40000(1.25셀). 올리면 고회피 선수가 훨씬 넓게 봄 | 4 | 기존 |
| 2 | 조향 시작 최소 거리 | abstract_input.rs:61 (L43467) | 2000 | 내리면 코앞 목표에도 조향 적용(진동 위험), 올리면 근거리 이동은 회피 안 함 | 4 | 기존 |
| 3 | '선 위' 판정 거리 | abstract_input.rs:85 (L43717) | 1500 | 올리면 궤적 정중앙 근처에서 수직 방향을 접근방향 기준으로 잡는 구간이 넓어짐 | 4 | 기존 |
| 4 | CC 투사체 최소 가치 | abstract_input.rs:102 (L43792) | 20 | 올리면 CC 기를 피해량 무관하게 더 강하게 회피(상한 30) | 4 | 기존 |
| 5 | 가치 상한 | abstract_input.rs:102/111 (L43804/L43838) | 30 | HP% 30 이상은 동일 취급. 낮추면 큰 피해기와 작은 피해기의 회피 강도 차이가 줄어듦 | 4 | 기존 |
| 6 | 회피 강도 계수 | abstract_input.rs:111 (L43831) | 40 | sx/sy 초기 스케일 1000 대비 회피 항 크기. 올리면 목표 방향보다 옆으로 더 크게 꺾음 | 4 | 기존 |
| 7 | 회피−명중 상쇄 기준 | abstract_input.rs:110 (L43496/L43823) | 50 | eff = avoid+50−caster_hit. 올리면 명중 높은 캐스터 상대로도 회피가 남음 | 4 | 기존 |
| 8 | v2 재조준 탈출점 최소 이격 | abstract_input.rs:134 (L43950) | 4000000 | 2000². 내리면 아주 가까운 탈출점도 Some(Move) 로 채택(제자리 뱅뱅 위험) | 4 | 기존 |
| 9 | AI 버전 게이트 | abstract_input.rs:128 (L43914) | version > 1 | v1 은 목표점이 우물 위험이면 무조건 None(입력 없음). v2+ 는 탈출점으로 재조준 | 4 | 기존 |

<details><summary>`callees` 피호출자 25건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | adjust_position | game_core::Game::adjust_position | pub | fn(&game_core::MapDef, &game_core::GameSetting, i64, i64) -> (u64, u64) | game-core\src\simulation\game.rs:818 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | approach_dodge_steer | game_ai::abstract_input::approach_dodge_steer | in:game_ai::abstract_input | fn(&game_core::PlayerState, &game_core::OperationData, &game_core::Entity, u64, u64) -> (u64, u64) | game-ai\src\abstract_input.rs:53 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 2 | check_projectile | game_core::CastingTarget::check_projectile | pub | fn(&game_core::CastingTarget, &game_core::Projectile, &game_core::Entity) -> bool | game-core\src\simulation\effect\type.rs:248 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 3 | distance_sq | game_core::utils::distance_sq | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:6 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 4 | distance_sq | game_core::Entity::distance_sq | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2157 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 5 | dm_no_dodge | game_core::dm_no_dodge | pub | fn() -> bool | game-core\src\simulation\entity.rs:206 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 6 | enemy_well_escape_position | game_ai::enemy_well_escape_position | pub | fn(&game_core::GameSetting, &game_core::MapDef, &game_core::PlayerState, u64, u64) -> (u64, u64) | game-ai\src\path_finder.rs:1043 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 7 | expected_damage_target | game_core::Projectile::expected_damage_target | pub | fn(&game_core::Projectile, &game_core::GameContext, &game_core::Entity, &game_core::Entity) -> usize | game-core\src\simulation\projectile.rs:1287 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 8 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 9 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 10 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 11 | has_cc | game_core::Projectile::has_cc | pub | fn(&game_core::Projectile) -> bool | game-core\src\simulation\projectile.rs:1272 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 12 | is_enemy_well_danger | game_ai::is_enemy_well_danger | pub | fn(usize, &game_core::PlayerState, u64, u64) -> bool | game-ai\src\path_finder.rs:1032 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 13 | is_recent_enemy_well_damage_danger | game_ai::is_recent_enemy_well_damage_danger | pub | fn(usize, &game_core::PlayerState, &game_core::Entity) -> bool | game-ai\src\path_finder.rs:1037 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 14 | is_targeting | game_core::ProjectileMoveType::is_targeting | pub | fn(&game_core::ProjectileMoveType) -> bool | game-core\src\simulation\projectile.rs:133 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 15 | isqrt | game_core::utils::isqrt | pub | fn(i64) -> i64 | game-core\src\utils.rs:85 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 16 | iter_projectile | game_core::AbstractGame::iter_projectile | pub | fn(&Self/#0) -> game_core::ProjectileIter | game-core\src\simulation.rs:182 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 17 | iter_projectile | <game_core::Game as game_core::AbstractGame>::iter_projectile | pub | fn(&game_core::Game) -> game_core::ProjectileIter | game-core\src\simulation\game.rs:3760 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 18 | iter_projectile | <game_core::SingleLaneGame as game_core::AbstractGame>::iter_projectile | pub | fn(&game_core::SingleLaneGame) -> game_core::ProjectileIter | game-core\src\simulation\game.rs:4019 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 19 | player_by_champion_id | game_core::AbstractGameWithCache::<'a, 'b>::player_by_champion_id | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> std::option::Option<&'a/#0 game_core::PlayerState> | game-core\src\simulation.rs:1915 | True | True | 3 | IR 호출 심볼 일치(토큰 경계) |
| 20 | projectile_half_width | game_ai::abstract_input::projectile_half_width | in:game_ai::abstract_input | fn(&game_core::ProjectileShape) -> i64 | game-ai\src\abstract_input.rs:44 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 21 | radius | game_core::Entity::radius | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1509 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 22 | safe_move_avoiding_enemy_well | game_ai::safe_move_avoiding_enemy_well | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, u64, u64) -> std::option::Option<game_core::Input> | game-ai\src\abstract_input.rs:122 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 23 | skill_avoid_effective | game_core::AthleteParameter::skill_avoid_effective | pub | fn(&game_core::AthleteParameter) -> usize | game-core\src\simulation\state\player.rs:296 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 24 | skill_hit_effective | game_core::AthleteParameter::skill_hit_effective | pub | fn(&game_core::AthleteParameter) -> usize | game-core\src\simulation\state\player.rs:301 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
</details>

⚠**미매칭 8개**: `bool`, `clamp`, `llvm.assume`, `llvm.memcpy.p0.p0.i64`, `llvm.smax.i64`, `llvm.umax.i64`, `llvm.umin.i64`, `next  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 9개는 **전부 다른 함수**라 싣지 않는다`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 12곳** (m04.ll:44212, m04.ll:44359, m04.ll:44492, m04.ll:44648, m04.ll:44786, m04.ll:44925, m04.ll:45063, m04.ll:45218, m07.ll:11918, m07.ll:12433, m07.ll:12772, m11.ll:43128) · **형제 0개** 

**`open` 6건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | `is_enemy_well_danger(cx,cy) \|\| is_recent_enemy_well_damage_danger` 의 소스 순서는 분기 방향(L43885→L43891)으로 확정했으나, :80 `along < -crad \|\| along > remain+crad` 와 :67 `team && visible` 은 IR 이 비단락(`or`/순차 br)이라 소스 표기는 불가(동작 동일) | 4 |  |
| 1 | 표기 불가 | :102 의 소스가 `.max(if has_cc {20} else {0}).clamp(0,30)` 인지 `clamp` 안에 max 가 있는지 — 표기 불가. 동작: value' = min(max(cc?20:0, value), 30) | 4 |  |
| 2 | 미탐색 | projectile_half_width 가 Line.width 를 반으로 나누지 않는 것이 의도인지 — IR 은 Rect 만 /2 (L43619), 나머지는 +0x18 그대로(L43623 phi). 소스 :45~48 은 인라인이라 arm 구성 추정 | 4 |  |
| 3 | 미탐색 | expected_damage_target 의 ctx 인자 = OperationData.context(GameContext 64B, L43394) 는 확정. 내부 산식은 game_core 명세 범위 밖 | 4 |  |
| 4 | 미탐색 | ProjectileIter 의 순회 순서(투사체 id 순?) — 조향 합산은 교환법칙이 성립해 순서 무관하지만 `threatened` 도 무관. 재현 시 순서 확인 불필요라고 판단(범위: 이 함수 한정) | 4 |  |
| 5 | 미탐색 | path_finder 콜리 3개(is_enemy_well_danger 등)는 game_ai 이지만 지도 밖 — 내부 판정은 이 라운드 범위 밖. 시그니처만 기록 | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

