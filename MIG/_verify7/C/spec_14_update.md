---

### `14` update — 갱커의 갱크 계속/취소 판정 — 저HP거나 목표 부시 도착+적 없음이면 Cancel

| 항목 | 값 |
|---|---|
| id | `line_gank_ganker__update` |
| 심볼 | `_RNvMNtNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old9line_gank6gankerNtB2_14LineGankerPlan6update` |
| 소스 | `game-ai\src\plan_legacy\old\line_gank\ganker.rs:39` |
| IR | `m08.ll` 94569~94941행 |
| 경로·가시성 | `game_ai::plan_legacy::old::LineGankerPlan::update` · **pub** |
| 계층 | 레거시 플랜 |
| exe | `db90f0` (ganker) · 825바이트 · 200명령 |
| 라운드 | 기준 `r6` · 통과 6회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&mut game_ai::plan_legacy::old::LineGankerPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &game_core::PositioningScoreData, &mut game_core::DebugFrameData)
```

<details><summary>인자 8개</summary>

| # | i | 이름 | 타입 | 역할 |
|---|---|---|---|---|
| 0 | 1 | self |  | chats(0x0)/line(0x28) 읽고 chats/phase(0x29) 를 쓴다 = 상태변경 |
| 1 | 2 | _version |  | 본문에서 전혀 안 읽힘 — AI 버전 게이트 없음(dbg_value 만 존재) |
| 2 | 3 | _rnd |  | readnone — 난수 안 씀 |
| 3 | 4 | player |  | info.team(0x930), info.position(0x9c0) 만 읽음 |
| 4 | 5 | data |  | cache(+0x0), context(+0x8) 를 읽음. blackboard(+0x10) 는 안 씀 |
| 5 | 6 | goal_data |  | has_near_line_enemy 의 수신자로만 넘어감(본문에서 필드 직접 로드 없음) |
| 6 | 7 | _positioning_score |  | 본문에서 안 읽힘(readonly 로 표기됐지만 사용 0회) |
| 7 | 8 | _debug |  | readnone — 안 씀 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn update(&mut self, _version, _rnd, player, data, goal_data, _positioning_score, _debug)
// ganker.rs:39. 반환값 없음 — self.chats / self.phase 만 바꾼다.

team = player.info.team // PlayerState+0x930, team<2 아니면 panic_bounds_check(ganker.rs:43)
pos = player.info.position.as_index() // PlayerState+0x9c0, entity.rs:580, range 0..5
cache = data.cache // OperationData+0x0
champ = cache.player_champion[team][pos].unwrap() // cache+0x1e0 + team*40 + pos*8, None 이면 panic(ganker.rs:43)

// ganker.rs:44
max_hp = champ.stat_cached.hp // Entity+0x628 (0 이면 div_by_zero panic)
hp_ratio = champ.hp * 100 / max_hp // Entity+0x670

// ganker.rs:47 — 취소 사유 ①
if hp_ratio < 41 {
 self.chats.push(Chat::Cancel(CancelReason::LowHpSelf)) // ganker.rs:48, 태그17/사유0
 self.phase = LineGankerPhase::Cancel // self+0x29 = 8
 return
}

// ganker.rs:54 — bush = self.target_bush_v30(player, data) [전량 인라인]
line = self.line // self+0x28, 0=Top/1=Mid/2=Bottom
context = data.context // OperationData+0x8
tower = cache.<line>_tower[team].or(cache.<line>_tower2[team]) // (cache+384+line*32)[team] .or( (cache+400+line*32)[team] ) ganker.rs:249

if tower.is_none() { // ganker.rs:250
 bush = match line { // ganker.rs:251
 Top => if team==0 {2} else {16} // 252
 Mid => if team==0 {4} else {17} // 253
 Bottom => if team==0 {9} else {21} // 254
 }
} else {
 t = tower.unwrap() // ganker.rs:258
 // champ 은 위에서 구한 것 재사용 (ganker.rs:259)
 is_tower_variant = (t.ty 판별자 == 2) // Entity+0x68 == EntityType::Tower
 match line { // ganker.rs:261
 Top => { // ganker.rs:263 info = t.ty.Tower 페이로드(Entity+0x70)
 if !is_tower_variant { unreachable!() } // ganker.rs:274
 if t.ty.is_tower2() { // ganker.rs:264 → Entity+0x128 > 4
 bush = if team==0 {3} else {6} // 265
 } else if info.nearest_enemy.is_some() { // 267, Entity+0x88 != 0
 bush = if team==0 {6} else {3} // 268 ← 좌우 반전
 } else {
 bush = if team==0 {3} else {6} // 270
 }
 }
 Mid => {
 if t.ty.is_tower2() { // ganker.rs:278 (판별자==2 && Entity+0x128>4)
 // ganker.rs:279 — map_regions::is_top_side(context, champ.x, champ.y)
 // ry = context.setting.height - champ.y ; IR 술어 = (ry < champ.x) = **!is_top_side**
 // (오라클 확증 — `is_top_side` 원식 = x + y <= height)
 if ry_lt_x /* = !is_top_side */ {
 bush = if team==0 {13} else {18} // 282
 } else {
 bush = if team==0 {8} else {12} // 280
 }
 } else {
 // ganker.rs:285 — 여기만 team 에 무관
 bush = if ry_lt_x /* = !is_top_side */ {14} else {11}
 }
 }
 Bottom => { // ganker.rs:293
 if !is_tower_variant { unreachable!() } // ganker.rs:304
 if t.ty.is_tower2() { // 294
 bush = if team==0 {15} else {20} // 295
 } else if info.nearest_enemy.is_some() { // 297
 bush = if team==0 {20} else {15} // 298 ← 좌우 반전
 } else {
 bush = if team==0 {15} else {20} // 300
 }
 }
 }
}

// ganker.rs:55
map = context.map // GameContext+0x20 (MapDef 28112B)
cy = min(champ.y / 32000, 29)
cx = min(champ.x / 32000, 29)
champ_bush = map.bushes[cy][cx] // MapDef+0x1c98, [30][30] usize

// ganker.rs:56 — 취소 사유 ②
if champ_bush == bush {
 // ganker.rs:57
 if !goal_data.has_near_line_enemy(self.line, map) {
 self.chats.push(Chat::Cancel(CancelReason::TargetMissing)) // ganker.rs:58, 태그17/사유2
 self.phase = LineGankerPhase::Cancel // self+0x29 = 8
 }
}
// ganker.rs:62 return
```

**`mem` 메모리 접근 27건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev |
|---|---|---|---|---|---|---|
| 0 | LineGankerPlan | 0x0 | chats(Vec<Chat>) cap | r | push 시 cap==len 비교용. gep 없이 ptr %0 직접 load 라 본문에 리터럴 0 은 안 보임 · tcx 정본 대조( `offset_of!` 실행 대조 MISMATCH 0 (`_verify6/C/o1_off.out`·`o2_off_ai.out`·`o3_off2.out`, 오프셋 51 + 구조체 크기 16 전건 OK) · `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=14 idx=0)) | 3 |
| 1 | LineGankerPlan | 0x10 | chats.len | r | push 경로에서 읽고 +1 로 되씀 · tcx 정본 대조( `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=14 idx=1)) | 3 |
| 2 | LineGankerPlan | 0x28 | line: LineType(0=Top,1=Mid,2=Bottom) | r | target_bush_v30 인라인의 switch 키 + has_near_line_enemy 2번째 인자. ganker.rs:54 · tcx 정본 대조( `offset_of!` 실행 대조 MISMATCH 0 (`_verify6/C/o1_off.out`·`o2_off_ai.out`·`o3_off2.out`, 오프셋 51 + 구조체 크기 16 전건 OK) · `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=14 idx=2)) | 3 |
| 3 | PlayerState | 0x930 | info.team: usize | r | player_champion[team] 인덱스(bounds check len=2) 겸 좌우 미러링 판정(team==0?) 키 · tcx 정본 대조( `offset_of!` 실행 대조 MISMATCH 0 (`_verify6/C/o1_off.out`·`o2_off_ai.out`·`o3_off2.out`, 오프셋 51 + 구조체 크기 16 전건 OK) · `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=14 idx=3)) | 3 |
| 4 | PlayerState | 0x9c0 | info.position: Position(i32, range 0..5) | r | Position::as_index(entity.rs:580) 인라인 → player_champion[team][pos] 인덱스 · tcx 정본 대조( `offset_of!` 실행 대조 MISMATCH 0 (`_verify6/C/o1_off.out`·`o2_off_ai.out`·`o3_off2.out`, 오프셋 51 + 구조체 크기 16 전건 OK) · `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=14 idx=4)) | 3 |
| 5 | OperationData | 0x0 | cache: &AbstractGameWithCache(8840B) | r | gep 없이 %4 직접 load · tcx 정본 대조( `offset_of!` 실행 대조 MISMATCH 0 (`_verify6/C/o1_off.out`·`o2_off_ai.out`·`o3_off2.out`, 오프셋 51 + 구조체 크기 16 전건 OK) · `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=14 idx=5)) | 3 |
| 6 | OperationData | 0x8 | context: &GameContext(64B) | r | · tcx 정본 대조( `offset_of!` 실행 대조 MISMATCH 0 (`_verify6/C/o1_off.out`·`o2_off_ai.out`·`o3_off2.out`, 오프셋 51 + 구조체 크기 16 전건 OK) · `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=14 idx=6)) | 3 |
| 7 | AbstractGameWithCache | 0x1e0 | player_champion[2][5]: Option<&Entity> | r | IR 리터럴 480. +team*40 +pos*8. unwrap 실패 시 panic(ganker.rs:43) · tcx 정본 대조( `offset_of!` 실행 대조 MISMATCH 0 (`_verify6/C/o1_off.out`·`o2_off_ai.out`·`o3_off2.out`, 오프셋 51 + 구조체 크기 16 전건 OK) · `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=14 idx=7)) | 3 |
| 8 | AbstractGameWithCache | 0x180 | top_tower[2]: Option<&Entity> | r | IR 리터럴 384. 실제 인덱싱은 384 + line*32 → line0=top_tower/line1=mid_tower(0x1a0)/line2=bottom_tower(0x1c0). line*32 는 shl 5 로 접힘 · tcx 정본 대조( `offset_of!` 실행 대조 MISMATCH 0 (`_verify6/C/o1_off.out`·`o2_off_ai.out`·`o3_off2.out`, 오프셋 51 + 구조체 크기 16 전건 OK) · `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=14 idx=8)) | 3 |
| 9 | AbstractGameWithCache | 0x190 | top_tower2[2]: Option<&Entity> | r | IR 리터럴 400. 400 + line*32 → mid_tower2(0x1b0)/bottom_tower2(0x1d0). tower.or(tower2) 의 두번째 항 · tcx 정본 대조( `offset_of!` 실행 대조 MISMATCH 0 (`_verify6/C/o1_off.out`·`o2_off_ai.out`·`o3_off2.out`, 오프셋 51 + 구조체 크기 16 전건 OK) · `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=14 idx=9)) | 3 |
| 10 | Entity | 0x628 | stat_cached.hp: usize | r | champ 의 최대 HP(분모). 0 이면 div_by_zero panic · tcx 정본 대조( `offset_of!` 실행 대조 MISMATCH 0 (`_verify6/C/o1_off.out`·`o2_off_ai.out`·`o3_off2.out`, 오프셋 51 + 구조체 크기 16 전건 OK) · `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=14 idx=10)) | 3 |
| 11 | Entity | 0x670 | hp: usize | r | champ 의 현재 HP(분자) · tcx 정본 대조( `offset_of!` 실행 대조 MISMATCH 0 (`_verify6/C/o1_off.out`·`o2_off_ai.out`·`o3_off2.out`, 오프셋 51 + 구조체 크기 16 전건 OK) · `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=14 idx=11)) | 3 |
| 12 | Entity | 0x660 | x: u64 | r | champ 좌표. is_top_side 인자 + bushes 열 인덱스(/32000) · tcx 정본 대조( `offset_of!` 실행 대조 MISMATCH 0 (`_verify6/C/o1_off.out`·`o2_off_ai.out`·`o3_off2.out`, 오프셋 51 + 구조체 크기 16 전건 OK) · `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=14 idx=12)) | 3 |
| 13 | Entity | 0x668 | y: u64 | r | champ 좌표. is_top_side 인자 + bushes 행 인덱스(/32000) · tcx 정본 대조( `offset_of!` 실행 대조 MISMATCH 0 (`_verify6/C/o1_off.out`·`o2_off_ai.out`·`o3_off2.out`, 오프셋 51 + 구조체 크기 16 전건 OK) · `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=14 idx=13)) | 3 |
| 14 | Entity | 0x68 | ty: EntityType 판별자(i64, range 0..14) | r | tower 쪽 Entity. ==2 → EntityType::Tower. EntityType::is_tower2(entity.rs:1308) 의 앞단 · tcx 정본 대조( `offset_of!` 실행 대조 MISMATCH 0 (`_verify6/C/o1_off.out`·`o2_off_ai.out`·`o3_off2.out`, 오프셋 51 + 구조체 크기 16 전건 OK) · `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=14 idx=14)) | 3 |
| 15 | Entity | 0x70 | ty 페이로드 = Tower(192B) 시작 | r | dbg_value 로만 등장(DW_OP_plus_uconst 112). 아래 두 오프셋의 기준 · tcx 정본 대조( `offset_of!` 실행 대조 MISMATCH 0 (`_verify6/C/o1_off.out`·`o2_off_ai.out`·`o3_off2.out`, 오프셋 51 + 구조체 크기 16 전건 OK) · `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=14 idx=15)) | 3 |
| 16 | Entity | 0x88 | ty.Tower.nearest_enemy 태그(Option<(usize,usize)>, range 0..2) | r | Tower+0x18. 0=None. Top/Bottom 라인에서만 검사 · tcx 정본 대조( `offset_of!` 실행 대조 MISMATCH 0 (`_verify6/C/o1_off.out`·`o2_off_ai.out`·`o3_off2.out`, 오프셋 51 + 구조체 크기 16 전건 OK) · `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=14 idx=16)) | 3 |
| 17 | Entity | 0x128 | ty.Tower.ty: TowerType(i8, range 0..8) | r | Tower+0xb8. TowerType::is_second_tower(tower.rs:99) = tag>4 (5=Top2,6=Mid2,7=Bottom2) · tcx 정본 대조( `offset_of!` 실행 대조 MISMATCH 0 (`_verify6/C/o1_off.out`·`o2_off_ai.out`·`o3_off2.out`, 오프셋 51 + 구조체 크기 16 전건 OK) · `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=14 idx=17)) | 3 |
| 18 | GameContext | 0x8 | setting: &GameSetting(5432B) | r | is_top_side 안에서만 씀 · tcx 정본 대조( `offset_of!` 실행 대조 MISMATCH 0 (`_verify6/C/o1_off.out`·`o2_off_ai.out`·`o3_off2.out`, 오프셋 51 + 구조체 크기 16 전건 OK) · `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=14 idx=18)) | 3 |
| 19 | GameContext | 0x20 | map: &MapDef(28112B) | r | IR 리터럴 32. bushes 조회 + has_near_line_enemy 3번째 인자 · tcx 정본 대조( `offset_of!` 실행 대조 MISMATCH 0 (`_verify6/C/o1_off.out`·`o2_off_ai.out`·`o3_off2.out`, 오프셋 51 + 구조체 크기 16 전건 OK) · `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=14 idx=19)) | 3 |
| 20 | GameSetting | 0x12c0 | height: u64 | r | map_regions::is_top_side 의 ry = height - y · tcx 정본 대조( `offset_of!` 실행 대조 MISMATCH 0 (`_verify6/C/o1_off.out`·`o2_off_ai.out`·`o3_off2.out`, 오프셋 51 + 구조체 크기 16 전건 OK) · `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=14 idx=20)) | 3 |
| 21 | MapDef | 0x1c98 | bushes: [30][30] usize (7200B) | r | champ_bush = bushes[clamp(y/32000,0,29)][clamp(x/32000,0,29)] · tcx 정본 대조( `offset_of!` 실행 대조 MISMATCH 0 (`_verify6/C/o1_off.out`·`o2_off_ai.out`·`o3_off2.out`, 오프셋 51 + 구조체 크기 16 전건 OK) · `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=14 idx=21)) | 3 |
| 22 | LineGankerPlan | 0x0 | chats: Vec<Chat> — push | w | Chat 원소 24B, 원소+0 에 태그 17(=Cancel), 원소+1 에 사유 바이트(0=LowHpSelf / 2=TargetMissing). 용량 부족 시 RawVec::grow_one 선행 · tcx 정본 대조( `offset_of!` 실행 대조 MISMATCH 0 (`_verify6/C/o1_off.out`·`o2_off_ai.out`·`o3_off2.out`, 오프셋 51 + 구조체 크기 16 전건 OK) · `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=14 idx=22)) | 3 |
| 23 | LineGankerPlan | 0x10 | chats.len | w | push 의 길이 갱신 · tcx 정본 대조( `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=14 idx=23)) | 3 |
| 24 | LineGankerPlan | 0x29 | phase: LineGankerPhase | w | ★두 취소 경로가 합류한 블록 %182 에서 한 번만 store. 니치 밀림 있는 열거형(6=WaitResponse,7=Setup,8=Cancel,그외=ChangeJungle) · tcx 정본 대조( `offset_of!` 실행 대조 MISMATCH 0 (`_verify6/C/o1_off.out`·`o2_off_ai.out`·`o3_off2.out`, 오프셋 51 + 구조체 크기 16 전건 OK) · `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=14 idx=24)) | 3 |
| 25 | LineGankerPlan | 0x18 | setup_limit | - | 이 함수는 이 리밋을 **읽지도 쓰지도 않는다** — 참고용 행이다(`dir: "-"`). ⚠한 행에 오프셋을 묶어 적으면 tcxaudit 기계 검사가 무력화되므로 라인별로 쪼갰다 . | 3 |
| 26 | LineGankerPlan | 0x20 | wait_limit | - | 이 함수는 이 리밋을 **읽지도 쓰지도 않는다** — 참고용 행이다(`dir: "-"`). ⚠한 행에 오프셋을 묶어 적으면 tcxaudit 기계 검사가 무력화되므로 라인별로 쪼갰다 . | 3 |

**`consts` 상수 21건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 100 | 44 | 임계 | hp_ratio = hp * 100 / max_hp — 백분율 환산 · 오라클 실행 확증( 오라클 `_verify5/C/o13.out §S14` **780/780** (부시 도착 ⟺ Cancel 발화 전건 일치) + HP 경계 409/410) | 2 |
| 1 | 41 | 47 | 임계 | ★HP 취소 임계. hp_ratio < 41 이면 갱크 포기(41% 미만) · 오라클 실행 확증( 오라클 `_verify5/C/o13.out §S14` **780/780** (부시 도착 ⟺ Cancel 발화 전건 일치) + HP 경계 409/410) | 2 |
| 2 | 2 | 261 | 태그 | EntityType 판별자 2 = Tower (is_tower2 의 앞단 비교). 별개로 CancelReason::TargetMissing 의 바이트값도 2 (ganker.rs:58) · 오라클 실행 확증( 오라클 `_verify5/C/o13.out §S14` **780/780** (부시 도착 ⟺ Cancel 발화 전건 일치) + HP 경계 409/410) | 2 |
| 3 | 4 | 264 | 임계 | TowerType::is_second_tower 임계 — tag > 4 (5=Top2 / 6=Mid2 / 7=Bottom2) · 오라클 실행 확증( 오라클 `_verify5/C/o13.out §S14` **780/780** (부시 도착 ⟺ Cancel 발화 전건 일치) + HP 경계 409/410) | 2 |
| 4 | 0 | 252 | 임계 | team == 0 비교(청팀). 목표 부시를 좌우 대칭으로 뒤집는 키. 또한 CancelReason::LowHpSelf 의 바이트값 0 · 오라클 실행 확증( 오라클 `_verify5/C/o13.out §S14` **780/780** (부시 도착 ⟺ Cancel 발화 전건 일치) + HP 경계 409/410) | 2 |
| 5 | 32000 | 55 | 임계 | 셀 크기 — 좌표 → 그리드 셀 변환(임계값 아님) · 오라클 실행 확증( 오라클 `_verify5/C/o13.out §S14` **780/780** (부시 도착 ⟺ Cancel 발화 전건 일치) + HP 경계 409/410) | 2 |
| 6 | 29 | 55 | 인덱스 | 그리드 인덱스 상한. clamp(0,29) 가 umin(x,29) 로 접힘 — bushes 는 30x30 · 오라클 실행 확증( 오라클 `_verify5/C/o13.out §S14` **780/780** (부시 도착 ⟺ Cancel 발화 전건 일치) + HP 경계 409/410) | 2 |
| 7 | 16 | 252 | 임계 | 목표 부시 ID — Top 라인·타워 없음·team!=0 · 오라클 실행 확증( 오라클 `_verify5/C/o13.out §S14` **780/780** (부시 도착 ⟺ Cancel 발화 전건 일치) + HP 경계 409/410) | 2 |
| 8 | 17 | 253 | 태그 | 목표 부시 ID — Mid 라인·타워 없음·team!=0. (동시에 Chat::Cancel 의 태그값 17 도 같은 리터럴) · 오라클 실행 확증( 오라클 `_verify5/C/o13.out §S14` **780/780** (부시 도착 ⟺ Cancel 발화 전건 일치) + HP 경계 409/410) | 2 |
| 9 | 9 | 254 | 임계 | 목표 부시 ID — Bottom 라인·타워 없음·team==0 · 오라클 실행 확증( 오라클 `_verify5/C/o13.out §S14` **780/780** (부시 도착 ⟺ Cancel 발화 전건 일치) + HP 경계 409/410) | 2 |
| 10 | 21 | 254 | 임계 | 목표 부시 ID — Bottom 라인·타워 없음·team!=0 · 오라클 실행 확증( 오라클 `_verify5/C/o13.out §S14` **780/780** (부시 도착 ⟺ Cancel 발화 전건 일치) + HP 경계 409/410) | 2 |
| 11 | 3 | 265 | 임계 | 목표 부시 ID — Top 라인 2차타워/적없음·team==0 (반대는 6) · 오라클 실행 확증( 오라클 `_verify5/C/o13.out §S14` **780/780** (부시 도착 ⟺ Cancel 발화 전건 일치) + HP 경계 409/410) | 2 |
| 12 | 6 | 268 | 임계 | 목표 부시 ID — Top 라인. nearest_enemy 있으면 team==0 이 6, 없으면 3 (좌우 반전) · 오라클 실행 확증( 오라클 `_verify5/C/o13.out §S14` **780/780** (부시 도착 ⟺ Cancel 발화 전건 일치) + HP 경계 409/410) | 2 |
| 13 | 14 | 285 | 임계 | 목표 부시 ID — Mid 라인·비2차타워·**is_top_side 거짓**(= 봇 사이드) (team 무관) [극성 정정 2026-09-11 배치C: IR 술어 `ry < x` 가 **!is_top_side** 였다. 부시 ID 자체는 불변] · 오라클 실행 확증( 오라클 `_verify5/C/o13.out §S14` **780/780** (부시 도착 ⟺ Cancel 발화 전건 일치) + HP 경계 409/410) | 2 |
| 14 | 11 | 285 | 임계 | 목표 부시 ID — Mid 라인·비2차타워·**is_top_side 참**(= 탑 사이드) (team 무관) [극성 정정 2026-09-11 배치C: IR 술어 `ry < x` 가 **!is_top_side** 였다. 부시 ID 자체는 불변] · 오라클 실행 확증( 오라클 `_verify5/C/o13.out §S14` **780/780** (부시 도착 ⟺ Cancel 발화 전건 일치) + HP 경계 409/410) | 2 |
| 15 | 13 | 282 | 임계 | 목표 부시 ID — Mid 라인·2차타워·**is_top_side 거짓**(= 봇 사이드)·team==0 [극성 정정 2026-09-11 배치C: IR 술어 `ry < x` 가 **!is_top_side** 였다. 부시 ID 자체는 불변] · 오라클 실행 확증( 오라클 `_verify5/C/o13.out §S14` **780/780** (부시 도착 ⟺ Cancel 발화 전건 일치) + HP 경계 409/410) | 2 |
| 16 | 18 | 282 | 임계 | 목표 부시 ID — Mid 라인·2차타워·**is_top_side 거짓**(= 봇 사이드)·team!=0 [극성 정정 2026-09-11 배치C: IR 술어 `ry < x` 가 **!is_top_side** 였다. 부시 ID 자체는 불변] · 오라클 실행 확증( 오라클 `_verify5/C/o13.out §S14` **780/780** (부시 도착 ⟺ Cancel 발화 전건 일치) + HP 경계 409/410) | 2 |
| 17 | 8 | 280 | 태그 | 목표 부시 ID — Mid 라인·2차타워·**is_top_side 참**(= 탑 사이드)·team==0. (동시에 LineGankerPhase::Cancel 태그값 8 과 같은 리터럴) [극성 정정 2026-09-11 배치C: IR 술어 `ry < x` 가 **!is_top_side** 였다. 부시 ID 자체는 불변] · 오라클 실행 확증( 오라클 `_verify5/C/o13.out §S14` **780/780** (부시 도착 ⟺ Cancel 발화 전건 일치) + HP 경계 409/410) | 2 |
| 18 | 12 | 280 | 임계 | 목표 부시 ID — Mid 라인·2차타워·**is_top_side 참**(= 탑 사이드)·team!=0 [극성 정정 2026-09-11 배치C: IR 술어 `ry < x` 가 **!is_top_side** 였다. 부시 ID 자체는 불변] · 오라클 실행 확증( 오라클 `_verify5/C/o13.out §S14` **780/780** (부시 도착 ⟺ Cancel 발화 전건 일치) + HP 경계 409/410) | 2 |
| 19 | 15 | 295 | 임계 | 목표 부시 ID — Bottom 라인·team==0 (기본형) · 오라클 실행 확증( 오라클 `_verify5/C/o13.out §S14` **780/780** (부시 도착 ⟺ Cancel 발화 전건 일치) + HP 경계 409/410) | 2 |
| 20 | 20 | 298 | 임계 | 목표 부시 ID — Bottom 라인·team!=0 (기본형). nearest_enemy 있으면 15/20 이 뒤집힘 · 오라클 실행 확증( 오라클 `_verify5/C/o13.out §S14` **780/780** (부시 도착 ⟺ Cancel 발화 전건 일치) + HP 경계 409/410) | 2 |

**`knobs` 조정점 7건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev |
|---|---|---|---|---|---|
| 0 | 갱크 취소 HP 임계 | ganker.rs:47 (IR m08.ll 94643 `icmp ult i64 %32, 41`) | 41 | 올리면 조금만 다쳐도 갱크를 접고 Cancel 로 간다(갱크 성공률↓, 갱커 생존↑). 내리면 저체력에도 갱크를 강행한다. 0 으로 내리면 HP 사유 취소가 사실상 사라진다 · 오라클 실행 확증( 오라클 `_verify5/C/o13.out §S14` **780/780** (부시 도착 ⟺ Cancel 발화 전건 일치) + HP 경계 409/410) | 2 |
| 1 | 목표 부시 ID 표 (라인×팀×타워상태 → 부시 인덱스) | ganker.rs:252~300 (target_bush_v30, update 안에 인라인) | Top 없음 2/16 · Mid 없음 4/17 · Bottom 없음 9/21 · Top 3/6 · Mid 2차타워 13/18(위) 8/12(아래) · Mid 비2차 14/11 · Bottom 15/20 | 갱커가 어느 덤불에 매복하러 가는지를 직접 정한다. 값을 바꾸면 도착 판정(champ_bush==bush)도 같이 바뀌므로 '영원히 도착 못 함 → 취소가 안 걸림' 이 될 수 있다 · 오라클 실행 확증( 오라클 `_verify5/C/o13.out §S14` **780/780** (부시 도착 ⟺ Cancel 발화 전건 일치) + HP 경계 409/410) | 2 |
| 2 | 2차 타워 판정 임계 | tower.rs:99 is_second_tower (IR `icmp samesign ugt i8 %67, 4`) | 4 | 올리면(예: 7) 2차 타워를 1차로 취급해 Top/Bottom 은 nearest_enemy 분기로, Mid 는 14/11 분기로 흘러 목표 부시 세트가 통째로 바뀐다 · 오라클 실행 확증( 오라클 `_verify5/C/o13.out §S14` **780/780** (부시 도착 ⟺ Cancel 발화 전건 일치) + HP 경계 409/410) | 2 |
| 3 | Mid 라인 위/아래 구분선 | map_regions.rs:22~24 `is_top_side` — ry = GameSetting.height - champ.y, **is_top_side = !(ry < champ.x) = (champ.x + champ.y <= height)**. IR 술어 `ry < champ.x` 는 그 **부정**이다(오라클 확정) | GameSetting+0x12c0 (height) | 맵 대각선을 기준으로 Mid 갱크 목표 부시를 13/18↔8/12, 14↔11 로 가른다. height 를 바꾸면 대각선이 기울어져 갱커가 반대편 부시로 간다 | 2 |
| 4 | **갱커 실제 은신 목표 = `target_bush_v41` (이 명세에 통째로 빠져 있었다)** | `target_bush_v41`(ganker.rs:310, `_gaibc/m08.ll:94136~94353`)를 `LineGankerPlan::sub_plan`(m08.ll:94960)이 **무조건** 호출해 `SubPlan::Hide{bush, out_line=Outline(1), check_move=0, enemy_spotted_me=0}`(태그 9)로 내보낸다. 반면 `update`(이 함수)는 `target_bush_v30` 을 인라인해 **도착 판정**에만 쓴다 | 인덱스 = `*_lead[team]`, 범위검사 `lead < 7` | ★★**`TargetMissing` 은 실제로 발화한다.** 자연상태 에서 **v30 == v41 12/12** (Top t0 3=3 · t1 6=6 / Mid 11·14 양팀 / Bottom t0 15 · t1 20). 실행 증거: 챔프를 v30 부시 셀에 놓고 `update` 를 부르면 `chats:[Cancel(TargetMissing)] phase:Cancel` 이 실제로 나온다(4건, 나머지 부시 21개에서는 미발화). 불일치는 **lead=0 · 타워 파괴 · 타워가 적 인지 구간 한정**(72칸 중 32칸 일치)이라 그 구간에서만 취소가 막힌다. ⟹ v30/v41 은 호출자별 하드와이어지만, 두 값이 일치하는 구간에서는 도착 판정이 성립한다. · 오라클 실행 확증( 오라클 `_verify5/C/o13.out §S14` **780/780** (부시 도착 ⟺ Cancel 발화 전건 일치) + HP 경계 409/410) | 2 |
| 5 | ★갱커 매복 덤불을 실제로 지배하는 세 필드 (라인 통제도) | `AbstractGameWithCache` `top_lead@0x21c0` / `mid_lead@0x21d0` / `bottom_lead@0x21e0` (`[usize;2]`, IR 리터럴 8640/8656/8672) — `update` 의 reads 에는 당연히 없다 | target_bush_v41 전표 (m08.ll:94136~94353 실측)  Top(0) team0: [16, 6, 3, 3, 3, 2, 2] team1: [2, 3, 6, 6, 6, 16, 16]  Bottom(2) team0: [21, 20, 15, 15, 15, 9, 7] team1: [9, 15, 20, 20, 20, 21, 23]  Mid(1) 은 챔피언 좌표를 쓴다 — s = !is_top_side(봇 사이드) 라 할 때  table[0] = team0 ? (s?21:17) : (s?9:4) · table[1..4] = s?14:11 · table[5],[6] = team0 ? (s?9:4) : (s?21:17)  ★**실행 재현** : Top·Bottom 14칸 전부 일치, Mid 는 s=false 가지 7칸 일치(team0 [17,11,11,11,11,4,4] / team1 [4,11,11,11,11,17,17]). **lead=7 은 실제로 패닉**(`lead<7` 확인). s=true(봇 사이드) 가지는 ★**해소( , 84/84)**: `Entity` 복제 + 좌표 변경 + `cache.player_champion[t][p]` 주입으로 도달했다. 실측 team0 [21,14,14,14,14,9,9] / team1 [9,14,14,14,14,21,21]. ⟹ 「재료 부재」가 아니라 **주입 수법 미적용**이었다 — `start_game` 직후 양 팀 챔프 10명이 전부 `is_top_side=true`(team0 (15000,913000)…, team1 (913000,15000)…). 미탐색 = 챔프를 봇 사이드로 옮긴 상태. 프로브 `_verify2/C/C_o1314.rs` | **라인 통제가 전진할수록(lead 0->6) 갱커 매복 덤불이 우리 진영 -> 적 진영으로 한 칸씩 밀린다** — 부시 중심좌표로 검산해 단조임을 확인했다(team0 Top: lead0 부시16 (16000,656000) = 아군 넥서스쪽 … lead5/6 부시2 (656000,16000) = 적 넥서스쪽). 이 세 필드를 조정하면 갱커 동선 전체가 바뀐다. ⚠**Mid 만 `player_champion[team][pos]` 를 unwrap 한다 = None 이면 패닉** (Top/Bottom 은 챔피언을 안 본다) · 오라클 실행 확증( 오라클 `_verify5/C/o13.out §S14` **780/780** (부시 도착 ⟺ Cancel 발화 전건 일치) + HP 경계 409/410) | 2 |
| 6 | ★갱커의 부시 선택기는 **셋**이다 | `target_bush_v30`(ganker.rs:248, →usize) = `update`(:54) 인라인 / `target_bush_v41`(ganker.rs:310, →usize) = `sub_plan`(:380, m08.ll:94960) **+ `next_plan`(:144, m08.ll:95509)** / ★**`target_bush`(ganker.rs:351, →(u64,u64) 좌표쌍)** = `next_plan`(:142) 인라인 | — | 1차는 v41 호출부를 `sub_plan` 1곳으로 적었으나 **실제 2곳**이고, `next_plan` 은 좌표를 돌려주는 **세 번째 선택기**(`min_by_key` + `map_regions::near_jungle_bush`)를 함께 쓴다 ⟹ 「이 플랜의 목표 부시」를 재구현할 때 `update`/`sub_plan`/`next_plan` 이 **서로 다른 세 규칙**을 쓴다는 것을 놓치면 안 된다. 부수: `LineGankCoverPlan` 에는 v41 이 없다(v30 + `target_bush` 둘뿐) = 커버 플랜은 항상 v30 | 4 |

<details><summary>`callees` 피호출자 13건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 |
|---|---|---|---|---|---|
| 0 | as_index | game_core::Position::as_index | pub | fn(&game_core::Position) -> usize | game-core\src\simulation\entity.rs:580 |
| 1 | as_index | game_core::BigGoal::as_index | pub | fn(&game_core::BigGoal) -> usize | game-core\src\simulation\ai_interface.rs:338 |
| 2 | has_near_line_enemy | game_ai::GoalData::has_near_line_enemy | pub | fn(&game_ai::GoalData, game_core::LineType, &game_core::MapDef) -> bool | game-ai\src\goal_data.rs:60 |
| 3 | is_top_side | game_core::is_top_side | pub | fn(&game_core::GameContext, u64, u64) -> bool | game-core\src\simulation\map_regions.rs:21 |
| 4 | is_tower2 | game_core::EntityType::is_tower2 | pub | fn(&game_core::EntityType) -> bool | game-core\src\simulation\entity.rs:1308 |
| 5 | push | <game_core::AthleteStat as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::setting::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\setting\athlete.rs:167 |
| 6 | push | <game_core::Athlete as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:434 |
| 7 | push | <game_core::TrainingExp as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:1137 |
| 8 | target_bush_v30 | game_ai::plan_legacy::old::LineGankerPlan::target_bush_v30 | in:game_ai::plan_legacy::old::line_gank::ganker | fn(&game_ai::plan_legacy::old::LineGankerPlan, &game_core::PlayerState, &game_core::OperationData) -> usize | game-ai\src\plan_legacy\old\line_gank\ganker.rs:248 |
| 9 | target_bush_v30 | game_ai::plan_legacy::old::LineGankCoverPlan::target_bush_v30 | in:game_ai::plan_legacy::old::line_gank::cover | fn(&game_ai::plan_legacy::old::LineGankCoverPlan, &game_core::PlayerState, &game_core::OperationData) -> usize | game-ai\src\plan_legacy\old\line_gank\cover.rs:134 |
| 10 | update | game_ai::plan_legacy::old::PassiveLinePlan::update | pub | fn(&mut game_ai::plan_legacy::old::PassiveLinePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PositioningScoreData, &mut game_core::DebugFrameData) | game-ai\src\plan_legacy\old\passive_line.rs:219 |
| 11 | update | game_ai::plan_legacy::old::SinglePlanLine::update | pub | fn(&mut game_ai::plan_legacy::old::SinglePlanLine, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PositioningScoreData, &mut game_core::DebugFrameData) | game-ai\src\plan_legacy\old\single_line.rs:27 |
| 12 | update | game_ai::plan_legacy::old::SinglePlanBattle::update | pub | fn(&mut game_ai::plan_legacy::old::SinglePlanBattle, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) | game-ai\src\plan_legacy\old\single_battle.rs:121 |
</details>

⚠**미매칭 4개**: `grow_one`, `llvm.assume`, `llvm.umin.i64`, `panic`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m02.ll:6984) · **형제 14개** (LineGankerPlan)

| # | 이름 | 심볼 | 비고 |
|---|---|---|---|
| 0 |  |  |  |
| 1 |  |  |  |
| 2 |  |  |  |
| 3 |  |  |  |
| 4 |  |  |  |
| 5 |  |  |  |
| 6 |  |  |  |
| 7 |  |  |  |
| 8 |  |  |  |
| 9 |  |  |  |
| 10 |  |  |  |
| 11 |  |  |  |
| 12 |  |  |  |
| 13 |  |  |  |

**`open` 0건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**

(없음 — 이 함수는 `open` 이 비었다. 그래도 **게이트 미해소(§4)와 반증은 유효하다**.)

**`notes` 1건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev |
|---|---|---|
| 0 | target_bush_v30 은 별도 define 이 없다(update 안에 전량 인라인). fnparts target_bush_v30 이 내놓는 m10.ll 11483~11731 은 LineGankCoverPlan 쪽 동명 함수이고, ★**문자 단위로 동일한 복제본**이다. rmeta SourceMap 줄 길이가 `cover.rs:134~186` ↔ `ganker.rs:248~300` **53줄 전부 일치**(오프셋 차 +114)이고 다른 것은 `self` 타입뿐(`LineGankCoverPlan+0x20` ↔ `LineGankerPlan+0x28`). ⟹ cover 판 `define`(m10.ll:11483)은 **유효한 대리 관측점**이고 실제로 5차 상향 60행을 만들었다. ⚠**같은 오류 문면이 `MIG\SPEC_GUIDE.md` §1 fnparts 항목에도 있다** — 「≠ 이니 조각을 버려라」가 관측 경로를 닫는다 — 그래서 위 상수들은 담당 줄범위(94569~94941) 안에서 직접 확인한 값이다 | 3 |

<details><summary>`closed` 9건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

| # | 물음 | 답 | ev |
|---|---|---|---|
| 0 | _version(p2)·_rnd(p3)·_positioning_score(p7)·_debug(p8) 은 본문에서 한 번도 로드되지 않는다(dbg_value 만). 즉 이 함수엔 AI 버전 게이트도 난수도 없다 — 다만 p7 이 attribute 상 readonly 라 '이름만 _ 접두' 인지 최적화 잔재인지는 확정 못 함 |  |  |
| 1 | EntityType::is_tower2(entity.rs:1308) 라는 이름과 실제 판정이 어긋나 보인다: IR 은 (판별자==2) && (Tower.ty > 4) 이고 dienum EntityType 에는 Tower2 variant 가 없다(태그2=Tower). 이름은 legacy 로 추정 — 확정 근거 없음 |  |  |
| 2 | ganker.rs:279 소스 조건의 극성 = **확정**. `is_top_side` = `champ.x + champ.y <= height` = `!(ry < champ.x)` (ry = height - champ.y). IR 의 `icmp ult ry, champ.x` 는 **그 부정**이다 — 2026-09-11 검증배치 C 가 SDK 오라클 실행으로 확정(표본 9개 전부 일치, 대각선 x+y==height 는 top/bottom 양쪽 true). 소스는 `if !is_top_side`(또는 is_bottom_side 계열)이 맞고 그래서 줄 280 이 먼저 온다. ⚠별도 define 이 없는(전량 인라인) 함수도 오라클·MIR 로 확인할 수 있다 |  |  |
| 3 | 부시 인덱스 2/3/4/6/8/9/11~21 이 맵의 어느 실제 덤불인지는 모른다. MapDef.bushes 배열의 값 사전(인덱스→덤불)을 안 봤다 |  |  |
| 4 | LineGankerPlan 의 setup_limit(0x18)/wait_limit(0x20) 은 이 함수에서 읽지도 쓰지도 않는다. 두 리밋을 실제로 소비하는 곳은 next_plan/is_end 로 추정되나 확인 안 함 |  |  |
| 5 | Chat 원소의 나머지 22바이트(payload+2..24)는 store 되지 않는다 — 초기화 안 된 채로 남는지 Chat 이 실제로 2바이트만 유효한지 확인 안 함 |  |  |
| 6 | has_near_line_enemy(goal_data, line, map) 내부는 안 봄 — 어떤 조건으로 '라인 근처 적'을 판정하는지 모름 |  |  |
| 7 | ⚠**일반화 금지**(2026-09-11 배치C): `_version` 미사용은 **이 함수(update) 본문에 한해** 맞다. 형제 `sub_plan` 이 `target_bush_v41` 을 부르므로 '이 플랜에 버전 게이트가 없다'로 플랜 전체에 일반화하면 안 된다. 미탐색 = `target_bush_v41` 의 Mid 분기 IR 재확인 · `*_lead` 를 누가 갱신하는지 |  |  |
| 8 | ⚠**메인 세션 브리핑 오류 정정(2026-09-11)**: 「`handler.rs:2189~2191` 의 `lead < 3` 이 이 함수에 있다」는 **틀렸다** — `LineGankerPlan::update` 전수 grep 결과 **lead 오프셋 0건**이다. 실제 소비처 = `m13.ll:8916`(`passive_plan`) · `m09.ll:63268` 외 5 · `m05.ll:44960` 외 2 · `m15.ll:55322` 외 1. **미탐색**. |  |  |
</details>

<details><summary>`history` 정정 이력 11건 (참조용 — 본문 아님)</summary>

| # | 옛 값 | 현재 | 근거 |
|---|---|---|---|
| 0 | 부시 인덱스 2~21 이 맵의 어느 실제 덤불인지 | target_bush_v30 항목의 부시 좌표표 참조 — 24개 전부 좌표 확정. |  |
| 1 | EntityType::is_tower2 라는 이름과 실제 판정((판별자==2) && (Tower.ty > 4))이 어긋나 보이는 것 | ★**모순이 아니다 — 확정.** `dienum TowerType`(태그=인덱스, 밀림 없음) = 0 Top / 1 Mid / 2 Bottom / 3 TwinA / 4 TwinB / **5 Top2 / 6 Mid2 / 7 Bottom2**. ⟹ `ty > 4` = {Top2, Mid2, Bottom2} = **2차 타워 3종**. `Tower2` 라는 *variant* 가 없었을 뿐 `TowerType` 의 `*2` 계열을 가리키는 이름이 맞다. 오프셋: `EntityType` 태그 2 = Tower, 페이로드 `enum+0x8 info: Tower(192B)`, `Tower+0xb8 ty: TowerType` ⟹ **EntityType 선두 기준 절대 +0xc0**. ⚠본문 IR 은 `_gcbc`·`_gaibc` 통틀어 define 0건(전 호출지점 인라인)이라 **원리적 복원 불가**지만, DWARF + TowerType 전표로 **간접 확정**했으므로 실질 공백은 없다. |  |
| 2 | logic·constants·knobs 세 곳의 `is_top_side` 표기 | ★**정정(2026-09-11 검증배치 C) — 극성 반전 표기 오류**: IR 의 `icmp ult (height − y), x` 를 `is_top_side` **본문식**이라 적었는데, 그건 **그 부정**이다. `height−y < x` ⟺ `x+y > height` ⟺ **`!is_top_side`**. ⟹ 우리가 발표한 **공식 `is_top_side = (x + y ≤ height)` 자체는 맞다**(Top = 작은 x·작은 y). 틀린 것은 **IR 비교식에 붙인 이름표**뿐이고, 동작 서술은 우연히 맞았다. 다만 **뜻이 정확히 뒤집혀 재구현자를 오도**하므로 정정한다. ★**오라클 실행으로 확정**: `is_top_side` 는 `vis=pub` · `mir=1` 이라 그냥 돌릴 수 있었다 — 표본 9개 전부 `ry_lt_x == !is_top_side`, 대각선(`x+y == height`)은 **top/bottom 양쪽 true**. 🔁**범위정정**: 「별도 define 이 없어(전량 인라인) 확정 불가」 판정은 **과했다** — `tcxq grep game_core is_top_side` 한 번이면 `pub`·`mir=1` 이 보이고 **오라클·MIR 두 경로가 열려 있었다**. 판정 어휘는 **미탐색**이 맞다. ⚠이 함수는 `logic`·`constants`·`knobs` **세 군데가 전부 오염**됐다(예: `constants` 의 "14 = is_top_side 참" → **거짓**이 맞다). |  |
| 3 | (누락) `target_bush_v41` 이 명세에 통째로 없음 | ★★**대형 누락(2026-09-11 검증배치 C)** — `update` 는 `target_bush_v30` 을 인라인해 **"도착"을 판정**하는데, **`LineGankerPlan::sub_plan`(m08.ll:94960)은 무조건 `target_bush_v41` 을 불러 `SubPlan::Hide{bush}` 를 만든다** = **챔피언이 실제로 가는 곳**. 둘 다 버전 게이트가 없고 **호출자별 하드와이어**다. ⟹ ★**`v30 ≠ v41` 인 구간에서는 `CancelReason::TargetMissing` 취소가 원리적으로 발화하지 않는다.** (실례: Top·team0·`top_lead=0` → v41 은 부시 **16** 으로 보내는데 v30 의 반환집합은 `{2,3,6}`) `v41` 전표의 인덱스 = **`*_lead[team]`**(`top/mid/bottom_lead @0x21c0/0x21d0/0x21e0`). 부시 중심좌표로 검산하니 **lead 가 커질수록 매복 덤불이 우리 진영 → 적 진영으로 단조 이동** — `_shared` 의 `*_lead` 정의와 정합. |  |
| 4 | `GoalData::has_near_line_enemy` 내부 — 1차는 `region_dist < 2` 구조로 추정만 | ★**확정**(2026-09-11 2차배치C, IR 전수독해 + 오라클 3/3 MATCH):   ∃i∈0..5: enemy_region[i]=Some(info) ∧ ∃k∈{2,3,4}: map.region_dist(map.line_region(line, 0, k), info.region) < 2 근거 `_gaibc/m09.ll:3948~4260`(5슬롯 × k 3개 = `line_region` 15회 완전 언롤, `region_dist` 표 = `MapDef+21720(0x54d8)` `[[usize;27];27]`, 경계검사 <27 2개, 조기탈출 `ult ..,2`). 실행 부수확정: `last_known` 은 **판정에 미사용** · 5슬롯 대등 · 빈 GoalData → false · **`side` 인자 리터럴 0 은 무해**(`lane_seq(line,1)` 이 역순이라 가운데 3칸 k=2,3,4 집합이 동일: Top{22,17,18}/Mid{4,6,3}/Bottom{8,16,23}). 실측 진리표 = Top{5,7,10,13,17,18,20,22} / Mid{1,2,3,4,6,7,10,11,12,13,14,15,19} / Bottom{2,8,9,14,15,16,23,25}. 프로브 `_verify2/C/C_o_near.rs`. |  |
| 5 | `*_lead` 를 누가 갱신하는지 | ★**갱신 주체 = `AbstractGameWithCache::new_with_prev_cache`**(game-core/simulation.rs:1780) — `_gcbc/g15.ll:104397~104406` 에 6개 store(+8640/8648 top, +8656/8664 mid, +8672/8680 bottom) ⟹ **캐시 생성마다 재계산되는 파생값**이지 누적 상태가 아니다. 읽기 접근자 = `line_lead(team, line)`(simulation.rs:1928, mir=1). 재료 = `region_point: [i32;27] @ +0x2218(8728)`(tcx + `memcpy(dst=+8728,108B,align 4)` 일치). ⚠**산출식은 못 닫았다** — 2차배치C 가 IR(`g15.ll:104280~104360`)에서 읽은 규칙(`region_point[r] < -2` 인 동안 전진)이 **실행과 6/6 불일치**했다(예측 0/3 vs 실측 전부 2). tick 0 실측: 전 라인·전 팀 `lead=2`, `region_point=[8,3,0,-6,6,7,0,0,6,7,-3,-3,-7,3,2,-2,0,0,-6,7,-7,-3,6,-6,3,-7,-8]`. 미탐색 범위 = `_gcbc/g15.ll:104100~104410` 전수 독해. ★교훈: 오라클과 IR 독해가 어긋나면 **IR 독해 쪽을 먼저 의심하라**(이번엔 IR 독해가 틀렸다). |  |
| 6 | 형제 `is_cancel`(ganker.rs:69) 의 의미 — 명세는 phase=Cancel(8) 만 서술 | ★**`is_cancel()` ≠ `phase == Cancel`**(2026-09-11 2차배치C). 실측 = WaitResponse→**true** / Setup→false / Cancel→true / ChangeJungle→false. MIR 확증(`mirdump_game_ai.txt:12918`, ganker.rs:70) = discriminant 를 **2(Cancel)** 와 **0(WaitResponse)** 두 값에 비교하는 `\|\|` 식. `LineGankerPhase` = {WaitResponse 0, Setup 1, Cancel 2, ChangeJungle(JungleType) 3}(선언 인덱스) / 메모리 태그는 니치로 ChangeJungle 0..5, WaitResponse 6, Setup 7, **Cancel 8**(명세의 8 은 맞다). |  |
| 7 | `update` 의 취소 경로(LowHpSelf / TargetMissing)를 실행으로 확인했는가 | ★**미검증(재료 부재)**. `C_o1314.rs` 로 phase 4종 × 라인 3 × 팀 2 × lead 7 = **168회 호출**, phase 변화 0 · 패닉 0. tick 0 은 HP 만복이라 `hp_ratio<41` 이 안 서고, 챔피언이 목표 부시 밖이라 도착 판정도 안 선다 ⟹ 1차의 「v30≠v41 이라 TargetMissing 이 원리적으로 발화하지 않는다」는 **실행으로 확증도 반박도 못 했고 논거는 여전히 IR 뿐이다.** 미탐색 = Entity 좌표를 목표 부시 셀로 옮긴 상태 / HP 조작. |  |
| 8 | `*_lead` 산출식 — 2차 IR 독해가 실행과 6/6 불일치했다 | ★**확정(3차 배치C, ev2, 6/6 일치)**:   lead = { let mut l = 0;            for (i, r) in map.lane_seq(line, team).iter().enumerate() {              let p = region_point[*r];              if (team == 0 && p > 2) \|\| (team == 1 && p < -2) { l = i } else { break }            } l }                      // 0..=6 ★**2차가 6/6 틀린 원인은 규칙이 아니라 team 부호를 놓친 것**이다(team0 `>2`, team1 `<-2`). IR = `g15.ll` `new_with_prev_cache`, `lane_seq` 6회 호출 + 저장 104398~104410. ⚠배치 C 가 기록한 자기 함정: `awk` 범위를 다음 `define` 까지 안 잡아 「store 0건 ⟹ region_point ≡ 0」이라는 틀린 결론을 냈고 **오라클이 즉시 반증**했다. 남은 미탐색 = `region_point` 자체 산출식(game_core 의 blue/red_regions 채우기 루프). |  |
| 9 | `region_point` 자체 산출식(3차 최대 잔여 — 'store 0건'으로 오판했던 항목) | ★**종결**(4차 배치C, ev2 · **27칸 전부 일치**). `AbstractGameWithCache+0x2218` = `[i32;27]`, `new_with_prev_cache`(simulation.rs:1780)의 지역변수 DWARF 이름이 그대로 남아 있었다.   region_point[r] = (blue_regions[r] ? 5 : 0) + blue_dist_one_count[r]                   − (red_regions[r]  ? 5 : 0) − red_dist_one_count[r]   blue_dist_one_count[n] = #{ r : blue_regions[r] && !red_regions[r] && n ∈ MapDef.region_adj[r] }   (red 대칭)   blue/red_regions 마킹 = 팀 타워마다 r = map.regions[y/32000][x/32000] → lane_seq(line, T) 안 인덱스 i → seq[0..=i] 를 true 결합식 store 는 `g15.ll:103609` **1개**(3차가 '0건'으로 본 바로 그 지점). 인접표 = `MapDef+0x0 region_adj: Vec<Vec<usize>>`, 격자 = `MapDef+0x38b8 regions`. 오라클 전수탐색(prefix `(0..8)^6`)으로 **prefix 3** 최소해 확정 — 근거 실측: 각 팀·라인 **1차 타워가 lane_seq 인덱스 2**, 2차가 0 또는 1. `*_lead` 3차 산출식(team0 `p>2` / team1 `p<-2`)도 **6/6 독립 재현**. 미탐색(범위 명시): 마킹 루프의 엔티티 필터(`g15.ll:103366` 3-way phi) · 타워 파괴 후 상태. |  |
| 10 | `setup_limit`(0x18)·`wait_limit`(0x20) 의 소비처 — `next_plan`/`is_end` 로 **추정**(ev5) | ★**`is_end` 만 쓴다**(4차 배치C, ev5→**ev2**). `next_plan` 은 gep **0건**.   is_end = tick >= wait_limit(0x20) ‖ phase==Cancel ‖ (phase==WaitResponse && tick >= setup_limit(0x18)) 근거 = `_gaibc/m08.ll:94519~94567` 전문 + **오라클 진리표 36/36**. 덤: `new(line, a, b)` 의 a=`setup_limit`, b=`wait_limit`. |  |
</details>

