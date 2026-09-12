---

### `13` target_bush_v30 — 라인갱크 커버 시 숨을 부시 인덱스(2~21)를 라인·팀·타워상태·챔프위치로 고른다

| 항목 | 값 |
|---|---|
| id | `line_gank_cover__target_bush_v30` |
| 심볼 | `_RNvMNtNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old9line_gank5coverNtB2_17LineGankCoverPlan15target_bush_v30` |
| 소스 | `game-ai\src\plan_legacy\old\line_gank\cover.rs:134` |
| IR | `m10.ll` 11483~11731행 |
| 경로·가시성 | `game_ai::plan_legacy::old::LineGankCoverPlan::target_bush_v30` · **in:game_ai::plan_legacy::old::line_gank::cover** |
| 계층 | 레거시 플랜 |
| exe | **없음** — 「exe 에 독립 함수가 없다(인라인·`define internal fastcc`)」인지 **「조인 실패」**인지는 이 칸만으로 못 가른다. `dllmatch.py`·`name2rva.py` 로 확인하라 |
| 라운드 | 기준 `r4` · 통과 4회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_ai::plan_legacy::old::LineGankCoverPlan, &game_core::PlayerState, &game_core::OperationData) -> usize
```

<details><summary>인자 3개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | self | &LineGankCoverPlan(40B) | IR에는 self.line(LineType, 0x20) 만 %0:i8 로 인자승격돼 전달. chats/wait_limit 는 이 함수에서 안 씀 | 4 |
| 1 | 2 | player | &PlayerState(2528B) | IR에서 %1:i64 = player.info.team(PlayerState+0x930), %2:i32 = player.info.position(PlayerState+0x9c0) 두 스칼라로 승격. team>=2 면 panic_bounds_check(len=2) | 4 |
| 2 | 3 | data | &OperationData(24B) | IR에서 %3:ptr = data.cache(&AbstractGameWithCache, +0x0), %4:ptr = data.context(&GameContext, +0x8) 로 승격. data.blackboard(+0x10)은 안 씀 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn target_bush_v30(self:&LineGankCoverPlan, player:&PlayerState, data:&OperationData) -> usize

 line = self.line // %0:i8 LineType {0=Top,1=Mid,2=Bottom}
 team = player.info.team // %1:i64, 0..=1 (>=2 면 panic_bounds_check(len 2))
 pos = player.info.position // %2:i32, player_champion 의 5칸 색인
 game = data.cache // %3 &AbstractGameWithCache
 ctx = data.context // %4 &GameContext

 // L135 : AbstractGameWithCache::tower(line, team) (game-core/simulation.rs:1822~1826, 인라인)
 // 1차타워가 None 이면 2차타워로 폴백 (Option::or)
 tower = match line {
 Top => game.top_tower[team] .or(game.top_tower2[team]), // 0x180 / 0x190
 Mid => game.mid_tower[team] .or(game.mid_tower2[team]), // 0x1a0 / 0x1b0
 Bottom => game.bottom_tower[team].or(game.bottom_tower2[team]), // 0x1c0 / 0x1d0
 };

 // L136~141 : 그 라인 타워가 1·2차 모두 없음(전부 파괴) -> 라인/팀만 보고 즉시 반환
 if tower.is_none() {
 return match line {
 Top => if team==0 { 2 } else { 16 }, // L138
 Mid => if team==0 { 4 } else { 17 }, // L139
 Bottom => if team==0 { 9 } else { 21 }, // L140
 };
 }

 tower = tower.unwrap(); // L144
 champ = game.player_champion[team][pos].unwrap(); // L145 (None 이면 패닉)

 match line { // L147

 // ── Top ───────────────────────────────────────── L149~L160
 Top => {
 // L149 : match &tower.ty { EntityType::Tower(info) => ... , _ => unreachable!() (L160) }
 // (Entity+0x68 판별자 != 2 이면 여기서 패닉. info = Entity+0x70 = &Tower)
 if tower.ty.is_tower2() { // L150 : 판별자==2 && TowerType(0x128) > 4
 return if team==0 { 3 } else { 6 }; // L151
 } else if info.nearest_enemy.is_some() { // L153 : Entity+0x88 태그 != 0
 return if team==0 { 6 } else { 3 }; // L154 ★좌우 반전
 } else {
 return if team==0 { 3 } else { 6 }; // L156 (L151 과 동일 값)
 }
 }

 // ── Mid ───────────────────────────────────────── L164~L171 (info 를 안 꺼냄 = unreachable 없음)
 Mid => {
 if tower.ty.is_tower2() { // L164
 // L165 : map_regions::is_top_side(ctx, champ.x, champ.y)
 // IR 술어 = (ctx.setting.height - champ.y) <u champ.x = **!is_top_side** (오라클 확증). `is_top_side` 원식 = x + y <= height
 if (ctx.setting.height - champ.y) < champ.x {
 return if team==0 { 13 } else { 18 }; // L168
 } else {
 return if team==0 { 8 } else { 12 }; // L166
 }
 } else {
 // L171 : 같은 부등식. ★team 을 전혀 보지 않는다
 return if (ctx.setting.height - champ.y) < champ.x { 14 } else { 11 };
 }
 }

 // ── Bottom ────────────────────────────────────── L179~L190 (Top 의 완전 대칭)
 Bottom => {
 // L179 : match &tower.ty { EntityType::Tower(info) => ..., _ => unreachable!() (L190) }
 if tower.ty.is_tower2() { // L180
 return if team==0 { 15 } else { 20 }; // L181
 } else if info.nearest_enemy.is_some() { // L183
 return if team==0 { 20 } else { 15 }; // L184 ★좌우 반전
 } else {
 return if team==0 { 15 } else { 20 }; // L186
 }
 }
 }
 // L194 ret

요약된 판정 구조
 1) 라인 타워가 남아있는가 -> 없으면 라인×팀 고정표(2/16, 4/17, 9/21)
 2) 남아있는 타워가 2차타워인가(TowerType>4) -> 방어선이 뒤로 밀렸으므로 다른 부시
 3) Top/Bottom 은 그 타워가 적을 인지 중인가(nearest_enemy.is_some) -> 부시를 반대쪽으로 스왑 
 4) Mid 는 팀 대신/추가로 맵 대각선(height-y vs x)으로 챔프가 어느 사이드에 있는지를 본다.
 1차타워 생존 시(L171)에는 team 을 아예 무시하고 사이드만으로 11/14 를 고른다.

부수효과 없음 — store 명령이 하나도 없는 순수 함수.

⚠**같은 플랜의 `LineGankCoverPlan::update`(cover.rs:25)는 본문이 비어 있다** — MIR 이 `bb0: T return` 한 줄뿐이다(`_tcx/mirdump_game_ai.txt:12997~12999`). 이 플랜을 재구현할 때 `update` 에 로직이 있다고 가정하면 헛수고한다 .
```

**`mem` 메모리 접근 15건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | AbstractGameWithCache | 0x180 | top_tower[team] | r | [Option<&Entity>;2] — line=Top 일 때 1순위 타워 · tcx 정본 대조( `offset_of!` 실행 대조 MISMATCH 0 (`_verify6/C/o1_off.out`·`o2_off_ai.out`·`o3_off2.out`, 오프셋 51 + 구조체 크기 16 전건 OK) · `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=13 idx=0)) | 3 | OK |
| 1 | AbstractGameWithCache | 0x190 | top_tower2[team] | r | top_tower 가 None 이면 폴백(Option::or, option.rs:1622) · tcx 정본 대조( `offset_of!` 실행 대조 MISMATCH 0 (`_verify6/C/o1_off.out`·`o2_off_ai.out`·`o3_off2.out`, 오프셋 51 + 구조체 크기 16 전건 OK) · `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=13 idx=1)) | 3 | OK |
| 2 | AbstractGameWithCache | 0x1a0 | mid_tower[team] | r | line=Mid 1순위 · tcx 정본 대조( `offset_of!` 실행 대조 MISMATCH 0 (`_verify6/C/o1_off.out`·`o2_off_ai.out`·`o3_off2.out`, 오프셋 51 + 구조체 크기 16 전건 OK) · `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=13 idx=2)) | 3 | OK |
| 3 | AbstractGameWithCache | 0x1b0 | mid_tower2[team] | r | line=Mid 폴백 · tcx 정본 대조( `offset_of!` 실행 대조 MISMATCH 0 (`_verify6/C/o1_off.out`·`o2_off_ai.out`·`o3_off2.out`, 오프셋 51 + 구조체 크기 16 전건 OK) · `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=13 idx=3)) | 3 | OK |
| 4 | AbstractGameWithCache | 0x1c0 | bottom_tower[team] | r | line=Bottom 1순위 · tcx 정본 대조( `offset_of!` 실행 대조 MISMATCH 0 (`_verify6/C/o1_off.out`·`o2_off_ai.out`·`o3_off2.out`, 오프셋 51 + 구조체 크기 16 전건 OK) · `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=13 idx=4)) | 3 | OK |
| 5 | AbstractGameWithCache | 0x1d0 | bottom_tower2[team] | r | line=Bottom 폴백 · tcx 정본 대조( `offset_of!` 실행 대조 MISMATCH 0 (`_verify6/C/o1_off.out`·`o2_off_ai.out`·`o3_off2.out`, 오프셋 51 + 구조체 크기 16 전건 OK) · `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=13 idx=5)) | 3 | OK |
| 6 | AbstractGameWithCache | 0x1e0 | player_champion[team][position] | r | [[Option<&Entity>;5];2] 80B. unwrap — None 이면 unwrap_failed 패닉(cover.rs:145:95) · tcx 정본 대조( `offset_of!` 실행 대조 MISMATCH 0 (`_verify6/C/o1_off.out`·`o2_off_ai.out`·`o3_off2.out`, 오프셋 51 + 구조체 크기 16 전건 OK) · `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=13 idx=6)) | 3 | OK |
| 7 | Entity | 0x68 | ty (EntityType 판별자) | r | 타워 엔티티. ==2 → EntityType::Tower. is_tower2()의 1단계(entity.rs:1308) · tcx 정본 대조( `offset_of!` 실행 대조 MISMATCH 0 (`_verify6/C/o1_off.out`·`o2_off_ai.out`·`o3_off2.out`, 오프셋 51 + 구조체 크기 16 전건 OK) · `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=13 idx=7)) | 3 | OK |
| 8 | Entity | 0x70 | ty payload = &Tower (변수명 info) | - | EntityType::Tower 페이로드의 **시작 표식**이다 — 참조용 행이므로 `dir` 은 `"-"` 가 맞다. m10.ll 11483~11731 에 `getelementptr .. i64 112` 는 **0건**이고, 페이로드 필드는 Entity **절대 오프셋**으로 직접 접근된다(`+0x88` nearest_enemy = gep 136, `+0x128` TowerType = gep 296). Top(L149)/Bottom(L179) 분기에서 쓰이는 것은 그 절대 오프셋들이다. 태그!=2 면 unreachable!()(L160/L190) · tcx 정본 대조( `offset_of!` 실행 대조 MISMATCH 0 (`_verify6/C/o1_off.out`·`o2_off_ai.out`·`o3_off2.out`, 오프셋 51 + 구조체 크기 16 전건 OK) · `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=13 idx=8)) | 3 | OK |
| 9 | Entity | 0x88 | Tower.nearest_enemy 판별자 | r | Tower+0x18, Option<(usize,usize)> 의 태그. !=0 이면 is_some(option.rs:632) · tcx 정본 대조( `offset_of!` 실행 대조 MISMATCH 0 (`_verify6/C/o1_off.out`·`o2_off_ai.out`·`o3_off2.out`, 오프셋 51 + 구조체 크기 16 전건 OK) · `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=13 idx=9)) | 3 | OK |
| 10 | Entity | 0x128 | Tower.ty (TowerType) | r | Tower+0xb8, i8 range[0,8). >4 → Top2/Mid2/Bottom2 = is_second_tower(tower.rs:99~100) · tcx 정본 대조( `offset_of!` 실행 대조 MISMATCH 0 (`_verify6/C/o1_off.out`·`o2_off_ai.out`·`o3_off2.out`, 오프셋 51 + 구조체 크기 16 전건 OK) · `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=13 idx=10)) | 3 | OK |
| 11 | Entity | 0x660 | x | r | player_champion 으로 얻은 아군 챔피언 좌표 x. Mid 분기에서만 사용 · tcx 정본 대조( `offset_of!` 실행 대조 MISMATCH 0 (`_verify6/C/o1_off.out`·`o2_off_ai.out`·`o3_off2.out`, 오프셋 51 + 구조체 크기 16 전건 OK) · `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=13 idx=11)) | 3 | OK |
| 12 | Entity | 0x668 | y | r | 같은 챔피언 좌표 y. Mid 분기에서만 사용 · tcx 정본 대조( `offset_of!` 실행 대조 MISMATCH 0 (`_verify6/C/o1_off.out`·`o2_off_ai.out`·`o3_off2.out`, 오프셋 51 + 구조체 크기 16 전건 OK) · `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=13 idx=12)) | 3 | OK |
| 13 | GameContext | 0x8 | setting (&GameSetting) | r | is_top_side(map_regions.rs:21)가 받는 context 에서 꺼냄 · tcx 정본 대조( `offset_of!` 실행 대조 MISMATCH 0 (`_verify6/C/o1_off.out`·`o2_off_ai.out`·`o3_off2.out`, 오프셋 51 + 구조체 크기 16 전건 OK) · `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=13 idx=13)) | 3 | OK |
| 14 | GameSetting | 0x12c0 | height | r | u64. map_regions.rs:22 의 ry = height - y · tcx 정본 대조( `offset_of!` 실행 대조 MISMATCH 0 (`_verify6/C/o1_off.out`·`o2_off_ai.out`·`o3_off2.out`, 오프셋 51 + 구조체 크기 16 전건 OK) · `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=13 idx=14)) | 3 | OK |

**`consts` 상수 16건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 2 | 138 | 태그 | ① team 인덱스 상한(팀 2개, 초과 시 panic_bounds_check) ② EntityType 판별자 2 = Tower ③ Top 라인 타워 전멸 + team0 일 때의 반환 부시 인덱스 · 오라클 실행 확증( 오라클 `_verify5/C/o13.out §S13` **300/300** (line 3 × team 2 × 타워상태 5 × 셀; 관측 경로 = `LineGankCoverPlan::sub_plan`(pub) 이 `target_bush_v30` 반환을 `SubPlan::Hide{bush}` 로 그대로 내보내는 얇은 래퍼)) | 2 |
| 1 | 16 | 138 | 산출값 | Top 라인 타워 전멸 + team1 반환 부시 · 오라클 실행 확증( 오라클 `_verify5/C/o13.out §S13` **300/300** (line 3 × team 2 × 타워상태 5 × 셀; 관측 경로 = `LineGankCoverPlan::sub_plan`(pub) 이 `target_bush_v30` 반환을 `SubPlan::Hide{bush}` 로 그대로 내보내는 얇은 래퍼)) | 2 |
| 2 | 4 | 139 | 임계 | ① TowerType>4 = 2차타워 판정 임계(5,6,7 = Top2/Mid2/Bottom2) ② Mid 라인 타워 전멸 + team0 반환 부시 · 오라클 실행 확증( 오라클 `_verify5/C/o13.out §S13` **300/300** (line 3 × team 2 × 타워상태 5 × 셀; 관측 경로 = `LineGankCoverPlan::sub_plan`(pub) 이 `target_bush_v30` 반환을 `SubPlan::Hide{bush}` 로 그대로 내보내는 얇은 래퍼)) | 2 |
| 3 | 17 | 139 | 산출값 | Mid 라인 타워 전멸 + team1 반환 부시 · 오라클 실행 확증( 오라클 `_verify5/C/o13.out §S13` **300/300** (line 3 × team 2 × 타워상태 5 × 셀; 관측 경로 = `LineGankCoverPlan::sub_plan`(pub) 이 `target_bush_v30` 반환을 `SubPlan::Hide{bush}` 로 그대로 내보내는 얇은 래퍼)) | 2 |
| 4 | 9 | 140 | 산출값 | Bottom 라인 타워 전멸 + team0 반환 부시 · 오라클 실행 확증( 오라클 `_verify5/C/o13.out §S13` **300/300** (line 3 × team 2 × 타워상태 5 × 셀; 관측 경로 = `LineGankCoverPlan::sub_plan`(pub) 이 `target_bush_v30` 반환을 `SubPlan::Hide{bush}` 로 그대로 내보내는 얇은 래퍼)) | 2 |
| 5 | 21 | 140 | 산출값 | Bottom 라인 타워 전멸 + team1 반환 부시 · 오라클 실행 확증( 오라클 `_verify5/C/o13.out §S13` **300/300** (line 3 × team 2 × 타워상태 5 × 셀; 관측 경로 = `LineGankCoverPlan::sub_plan`(pub) 이 `target_bush_v30` 반환을 `SubPlan::Hide{bush}` 로 그대로 내보내는 얇은 래퍼)) | 2 |
| 6 | 3 | 151 | 산출값 | Top: 2차타워가 살아있거나(L151) 타워가 적을 못 보는 상태(L156)일 때 team0 반환 부시 (team1 은 6) · 오라클 실행 확증( 오라클 `_verify5/C/o13.out §S13` **300/300** (line 3 × team 2 × 타워상태 5 × 셀; 관측 경로 = `LineGankCoverPlan::sub_plan`(pub) 이 `target_bush_v30` 반환을 `SubPlan::Hide{bush}` 로 그대로 내보내는 얇은 래퍼)) | 2 |
| 7 | 6 | 154 | 산출값 | Top: 위와 짝. L154(타워가 적 인지 중)에서는 team0=6/team1=3 으로 좌우가 뒤집힌다 · 오라클 실행 확증( 오라클 `_verify5/C/o13.out §S13` **300/300** (line 3 × team 2 × 타워상태 5 × 셀; 관측 경로 = `LineGankCoverPlan::sub_plan`(pub) 이 `target_bush_v30` 반환을 `SubPlan::Hide{bush}` 로 그대로 내보내는 얇은 래퍼)) | 2 |
| 8 | 13 | 168 | 산출값 | Mid + 2차타워 + (height-champ.y) < champ.x 일 때 team0 반환 부시 [= **!is_top_side**(봇 사이드) 구간이다] · 오라클 실행 확증( 오라클 `_verify5/C/o13.out §S13` **300/300** (line 3 × team 2 × 타워상태 5 × 셀; 관측 경로 = `LineGankCoverPlan::sub_plan`(pub) 이 `target_bush_v30` 반환을 `SubPlan::Hide{bush}` 로 그대로 내보내는 얇은 래퍼)) | 2 |
| 9 | 18 | 168 | 산출값 | 같은 조건 team1 반환 부시 · 오라클 실행 확증( 오라클 `_verify5/C/o13.out §S13` **300/300** (line 3 × team 2 × 타워상태 5 × 셀; 관측 경로 = `LineGankCoverPlan::sub_plan`(pub) 이 `target_bush_v30` 반환을 `SubPlan::Hide{bush}` 로 그대로 내보내는 얇은 래퍼)) | 2 |
| 10 | 8 | 166 | 산출값 | Mid + 2차타워 + 위 부등식이 거짓일 때 team0 반환 부시 · 오라클 실행 확증( 오라클 `_verify5/C/o13.out §S13` **300/300** (line 3 × team 2 × 타워상태 5 × 셀; 관측 경로 = `LineGankCoverPlan::sub_plan`(pub) 이 `target_bush_v30` 반환을 `SubPlan::Hide{bush}` 로 그대로 내보내는 얇은 래퍼)) | 2 |
| 11 | 12 | 166 | 산출값 | 같은 조건 team1 반환 부시 · 오라클 실행 확증( 오라클 `_verify5/C/o13.out §S13` **300/300** (line 3 × team 2 × 타워상태 5 × 셀; 관측 경로 = `LineGankCoverPlan::sub_plan`(pub) 이 `target_bush_v30` 반환을 `SubPlan::Hide{bush}` 로 그대로 내보내는 얇은 래퍼)) | 2 |
| 12 | 14 | 171 | 산출값 | Mid + 1차타워 생존 + (height-champ.y) < champ.x 일 때 반환 부시. ★team 무관 [= **!is_top_side**(봇 사이드) 인 구간] · 오라클 실행 확증( 오라클 `_verify5/C/o13.out §S13` **300/300** (line 3 × team 2 × 타워상태 5 × 셀; 관측 경로 = `LineGankCoverPlan::sub_plan`(pub) 이 `target_bush_v30` 반환을 `SubPlan::Hide{bush}` 로 그대로 내보내는 얇은 래퍼)) | 2 |
| 13 | 11 | 171 | 산출값 | Mid + 1차타워 생존 + 위 부등식 거짓일 때 반환 부시. ★team 무관 · 오라클 실행 확증( 오라클 `_verify5/C/o13.out §S13` **300/300** (line 3 × team 2 × 타워상태 5 × 셀; 관측 경로 = `LineGankCoverPlan::sub_plan`(pub) 이 `target_bush_v30` 반환을 `SubPlan::Hide{bush}` 로 그대로 내보내는 얇은 래퍼)) | 2 |
| 14 | 15 | 181 | 산출값 | Bottom: 2차타워 생존(L181) 또는 타워가 적 미인지(L186)일 때 team0 반환 부시 (team1 은 20) · 오라클 실행 확증( 오라클 `_verify5/C/o13.out §S13` **300/300** (line 3 × team 2 × 타워상태 5 × 셀; 관측 경로 = `LineGankCoverPlan::sub_plan`(pub) 이 `target_bush_v30` 반환을 `SubPlan::Hide{bush}` 로 그대로 내보내는 얇은 래퍼)) | 2 |
| 15 | 20 | 184 | 산출값 | Bottom: 위와 짝. L184(타워가 적 인지 중)에서는 team0=20/team1=15 로 뒤집힘 · 오라클 실행 확증( 오라클 `_verify5/C/o13.out §S13` **300/300** (line 3 × team 2 × 타워상태 5 × 셀; 관측 경로 = `LineGankCoverPlan::sub_plan`(pub) 이 `target_bush_v30` 반환을 `SubPlan::Hide{bush}` 로 그대로 내보내는 얇은 래퍼)) | 2 |

**`knobs` 조정점 11건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 라인 타워 전멸 시 대피 부시 (라인×팀) | cover.rs:138-140 | Top 2/16, Mid 4/17, Bottom 9/21 | 타워가 다 밀린 라인에서 커버 챔프가 숨는 자리를 통째로 옮긴다. 값을 서로 바꾸면 아군 진영 쪽/적 진영 쪽 부시로 커버 위치가 바뀐다 · 오라클 실행 확증( 오라클 `_verify5/C/o13.out §S13` **300/300** (line 3 × team 2 × 타워상태 5 × 셀; 관측 경로 = `LineGankCoverPlan::sub_plan`(pub) 이 `target_bush_v30` 반환을 `SubPlan::Hide{bush}` 로 그대로 내보내는 얇은 래퍼)) | 2 | 기존 |
| 1 | Top 기본 부시 | cover.rs:151, 156 | team0=3 / team1=6 | Top 커버의 상시 대기 부시. 3↔6 을 뒤집으면 두 팀의 대기 위치가 서로 맞바뀐다 · 오라클 실행 확증( 오라클 `_verify5/C/o13.out §S13` **300/300** (line 3 × team 2 × 타워상태 5 × 셀; 관측 경로 = `LineGankCoverPlan::sub_plan`(pub) 이 `target_bush_v30` 반환을 `SubPlan::Hide{bush}` 로 그대로 내보내는 얇은 래퍼)) | 2 | 기존 |
| 2 | Top 타워가 적을 인지 중일 때의 부시 | cover.rs:154 | team0=6 / team1=3 | 적이 타워 사거리에 들어와 있으면 반대쪽 부시로 옮겨 협공 각을 잡는 동작. 3/6 을 L151·L156 과 같게 만들면 이 반응 자체가 사라진다(항상 같은 부시) · 오라클 실행 확증( 오라클 `_verify5/C/o13.out §S13` **300/300** (line 3 × team 2 × 타워상태 5 × 셀; 관측 경로 = `LineGankCoverPlan::sub_plan`(pub) 이 `target_bush_v30` 반환을 `SubPlan::Hide{bush}` 로 그대로 내보내는 얇은 래퍼)) | 2 | 기존 |
| 3 | Mid 2차타워 방어 시 부시 (맵 사이드별) | cover.rs:166, 168 | 부등식 참=13/18, 거짓=8/12 | 미드 2차타워까지 밀린 상황에서 챔프가 서 있는 맵 사이드에 맞춰 부시를 고른다. 두 쌍을 바꾸면 커버가 반대 사이드 부시에서 대기한다 · 오라클 실행 확증( 오라클 `_verify5/C/o13.out §S13` **300/300** (line 3 × team 2 × 타워상태 5 × 셀; 관측 경로 = `LineGankCoverPlan::sub_plan`(pub) 이 `target_bush_v30` 반환을 `SubPlan::Hide{bush}` 로 그대로 내보내는 얇은 래퍼)) | 2 | 기존 |
| 4 | Mid 1차타워 생존 시 부시 (팀 무관) | cover.rs:171 | 부등식 참=14, 거짓=11 | 미드 정상 상황의 커버 부시. 이 줄만 팀을 안 보므로, 팀별로 다르게 하고 싶으면 여기에 team 분기를 넣어야 한다 · 오라클 실행 확증( 오라클 `_verify5/C/o13.out §S13` **300/300** (line 3 × team 2 × 타워상태 5 × 셀; 관측 경로 = `LineGankCoverPlan::sub_plan`(pub) 이 `target_bush_v30` 반환을 `SubPlan::Hide{bush}` 로 그대로 내보내는 얇은 래퍼)) | 2 | 기존 |
| 5 | Bottom 기본 부시 / 인지 시 부시 | cover.rs:181, 186 / 184 | 기본 team0=15,team1=20 / 인지 시 20,15 | Top 의 3·6 과 완전 대칭. 15↔20 스왑 여부가 봇 커버의 좌우 대기 위치를 결정한다 · 오라클 실행 확증( 오라클 `_verify5/C/o13.out §S13` **300/300** (line 3 × team 2 × 타워상태 5 × 셀; 관측 경로 = `LineGankCoverPlan::sub_plan`(pub) 이 `target_bush_v30` 반환을 `SubPlan::Hide{bush}` 로 그대로 내보내는 얇은 래퍼)) | 2 | 기존 |
| 6 | 2차타워 판정 임계 | game-core/simulation/entity/tower.rs:100 (인라인, IR 11529·11546·11590 부근의 `ugt i8 %x, 4`) | 4 | TowerType > 4 (Top2/Mid2/Bottom2)를 '2차타워'로 본다. game_core 쪽 공용 술어라 여기만 바꾸면 다른 판정도 같이 흔들린다 · 오라클 실행 확증( 오라클 `_verify5/C/o13.out §S13` **300/300** (line 3 × team 2 × 타워상태 5 × 셀; 관측 경로 = `LineGankCoverPlan::sub_plan`(pub) 이 `target_bush_v30` 반환을 `SubPlan::Hide{bush}` 로 그대로 내보내는 얇은 래퍼)) | 2 | 기존 |
| 7 | 부시 ID 그리드 | @anon...269 (_gcbc/g07.ll:275) | 30×30 u64 격자(900칸) — 0=부시 없음(829칸) / 1~24=부시 ID(ID당 1~5칸) | ★이 함수(`LineGankCoverPlan::target_bush_v30`)는 2~21 만 낸다. ⚠**「1/22/23/24 영구 제외」는 이 함수 한정**이고 갱커 쪽 형제 `LineGankerPlan::target_bush_v41` 이 **7·23 을 실제로 반환**한다(2026-09-11 실행 확인, `C_o1314_t0.tsv` Bottom lead6). 두 함수를 합친 실제 미사용 = **1, 5, 10, 19, 22, 24** (적용 범위: v30 + v41 기준. 좌표를 돌려주는 `target_bush`(→(u64,u64))·`near_jungle_bush` 는 미탐색). 여기 ID 를 바꾸면 새 은신처가 살아난다 | 2 | 신규 |
| 8 | is_top/bottom_side 대각선 | _gcbc/g15.ll:66114·66116 | x+y ≤ height / ≥ height | `height`(GameSetting+0x12c0)를 바꾸면 탑/봇 판정 경계가 대각선째 평행이동한다 · 오라클 실행 확증( 오라클 `_verify5/C/o13.out §S13` **300/300** (line 3 × team 2 × 타워상태 5 × 셀; 관측 경로 = `LineGankCoverPlan::sub_plan`(pub) 이 `target_bush_v30` 반환을 `SubPlan::Hide{bush}` 로 그대로 내보내는 얇은 래퍼)) | 2 | 신규 |
| 9 | 셀 좌표 클램프 | ★`target_bush_v30` 본문(`_gaibc/m10.ll:11483~11731`)에는 이 클램프가 **없다**(umin·32000·29 전부 0건) — 클램프는 부시 ID 를 셀 좌표로 바꾸는 **호출부** 쪽이다. 실측 확인된 site = `_gaibc/m08.ll:94830 udiv 32000` → `94836/94843 llvm.umin.i64(.., 29)`(`!dbg`→`ganker.rs:55`, = specs[14] `LineGankerPlan::update`). cover 쪽 대응 호출부 줄은 **미탐색** | umin(coord/32000, 29) | 맵 밖 좌표가 **패닉 대신 가장자리 셀로 접힌다**. 재구현 시 이 클램프를 빼면 동작 불일치 | 4 | 신규 |
| 10 | 미니맵 스케일·오프셋 | _gvbc/v10.ll:251049·251158 | 1/3 스케일, +20 오프셋 | 월드 960 → 패널 320 | 4 | 신규 |

<details><summary>`callees` 피호출자 5건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev |
|---|---|---|---|---|---|---|---|---|
| 0 | is_top_side | game_core::is_top_side | pub | fn(&game_core::GameContext, u64, u64) -> bool | game-core\src\simulation\map_regions.rs:21 | True | True | 3 |
| 1 | is_tower2 | game_core::EntityType::is_tower2 | pub | fn(&game_core::EntityType) -> bool | game-core\src\simulation\entity.rs:1308 | True | True | 3 |
| 2 | target_bush_v30 | game_ai::plan_legacy::old::LineGankerPlan::target_bush_v30 | in:game_ai::plan_legacy::old::line_gank::ganker | fn(&game_ai::plan_legacy::old::LineGankerPlan, &game_core::PlayerState, &game_core::OperationData) -> usize | game-ai\src\plan_legacy\old\line_gank\ganker.rs:248 | False | False | 3 |
| 3 | target_bush_v30 | game_ai::plan_legacy::old::LineGankCoverPlan::target_bush_v30 | in:game_ai::plan_legacy::old::line_gank::cover | fn(&game_ai::plan_legacy::old::LineGankCoverPlan, &game_core::PlayerState, &game_core::OperationData) -> usize | game-ai\src\plan_legacy\old\line_gank\cover.rs:134 | False | False | 3 |
| 4 | tower | game_core::AbstractGameWithCache::<'a, 'b>::tower | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, game_core::LineType, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:1822 | True | True | 3 |
</details>

⚠**미매칭 1개**: `panic`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 2곳** (m10.ll:11791, m10.ll:12292) · **형제 10개** (LineGankCoverPlan)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::plan_legacy::old::LineGankCoverPlan as std::clone::Clone>::clone | pub | game-ai\src\plan_legacy\old\line_gank\cover.rs:9 | True | fn(&game_ai::plan_legacy::old::LineGankCoverPlan) -> game_ai::plan_legacy::old::LineGankCoverPlan |
| 1 | <game_ai::plan_legacy::old::LineGankCoverPlan as std::fmt::Debug>::fmt | pub | game-ai\src\plan_legacy\old\line_gank\cover.rs:9 | True | fn(&game_ai::plan_legacy::old::LineGankCoverPlan, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 2 | game_ai::plan_legacy::old::LineGankCoverPlan::new | pub | game-ai\src\plan_legacy\old\line_gank\cover.rs:17 | True | fn(game_core::LineType, usize) -> game_ai::plan_legacy::old::LineGankCoverPlan |
| 3 | game_ai::plan_legacy::old::LineGankCoverPlan::goal | pub | game-ai\src\plan_legacy\old\line_gank\cover.rs:21 | True | fn(&game_ai::plan_legacy::old::LineGankCoverPlan) -> game_core::BigGoal |
| 4 | game_ai::plan_legacy::old::LineGankCoverPlan::update | pub | game-ai\src\plan_legacy\old\line_gank\cover.rs:25 | True | fn(&mut game_ai::plan_legacy::old::LineGankCoverPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &game_core::PositioningScoreData, &mut game_core::DebugFrameData) |
| 5 | game_ai::plan_legacy::old::LineGankCoverPlan::is_end | pub | game-ai\src\plan_legacy\old\line_gank\cover.rs:30 | False | fn(&game_ai::plan_legacy::old::LineGankCoverPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> bool |
| 6 | game_ai::plan_legacy::old::LineGankCoverPlan::next_plan | pub | game-ai\src\plan_legacy\old\line_gank\cover.rs:34 | False | fn(&mut game_ai::plan_legacy::old::LineGankCoverPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> std::option::Option<game_ai::plan_legacy::types::BigPlan> |
| 7 | game_ai::plan_legacy::old::LineGankCoverPlan::target_bush_v30 | in:game_ai::plan_legacy::old::line_gank::cover | game-ai\src\plan_legacy\old\line_gank\cover.rs:134 | False | fn(&game_ai::plan_legacy::old::LineGankCoverPlan, &game_core::PlayerState, &game_core::OperationData) -> usize |
| 8 | game_ai::plan_legacy::old::LineGankCoverPlan::target_bush | in:game_ai::plan_legacy::old::line_gank::cover | game-ai\src\plan_legacy\old\line_gank\cover.rs:196 | False | fn(&game_ai::plan_legacy::old::LineGankCoverPlan, &game_core::PlayerState, &game_core::OperationData) -> (u64, u64) |
| 9 | game_ai::plan_legacy::old::LineGankCoverPlan::sub_plan | pub | game-ai\src\plan_legacy\old\line_gank\cover.rs:224 | False | fn(&game_ai::plan_legacy::old::LineGankCoverPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> game_ai::plan_legacy::sub_plan::SubPlan |

**`open` 0건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**

(없음 — 이 함수는 `open` 이 비었다. 그래도 **게이트 미해소(§4)와 반증은 유효하다**.)

**`notes` 2건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | Top 의 L151(2차타워)과 L156(적 미인지)이 3/6 으로 완전히 같고, Bottom 의 L181/L186 도 15/20 으로 같다. 두 분기를 따로 쓴 이유는 **중복 서술이 아니라 중첩 구조** 때문이다(6차 배치C, 줄 길이 산술로 확정). cover.rs L150(36B) 와 L153(46B) 의 차이 10 = 술어 문자수 차 8(`tower.ty.is_tower2()` 20자 → `info.nearest_enemy.is_some()` 28자) + 들여쓰기 2, L152(19B)↔L155(21B) +2, L157(14B)↔L158(12B) −2 ⟹ 소스는 `else if` 가 **아니라** `else { if … }` 로 한 단 더 들여쓴다. 그래서 L151 과 L156 은 **같은 반환식이 들여쓰기 2칸만 다른 것**이고(54B vs 56B), Bottom 블록(L181 56B / L184 58B / L186 58B)은 리터럴 자리수(3·6 → 15·20) 때문에 Top 대비 전 줄이 +2 다. 세 줄의 값이 겹치는 것은 이 구조가 강제한 것이고 원래 다른 값이었다는 흔적은 없다. | 3 | 사실 서술 |
| 1 | 함수 이름의 `v30` 은 **AI 내부 버전 번호 30** 을 가리키는 도입 시점 표식이고, 런타임 버전 분기와는 무관하다. 근거 ①`_tcx/game_ai.json` 의 아이템 중 `vNN_` 접두가 v2·v3·v15~v17·v21~v28·**v30**·v46~v48·v50·v54·v55·v57 에 걸쳐 190개, `_vNN` 접미가 v3·v15·v26·**v30**·v32·v37·**v41**·v46·v54 에 있다 ②개발자 주석(`_docs/game_ai.txt`)이 `v54+`·`v<54`·`v92+` 처럼 **부등호 비교**로 쓴다 ⟹ 단순 라벨이 아니라 버전 임계다 ③`LineGankCoverPlan` 의 `sub_plan`·`next_plan`·`target_bush_v30` 어디에도 `version` 인자 비교가 없고(`_gaibc/m10.ll:11810~12400` 의 `icmp .*%2` 0건) 갱커 쪽도 `update`→v30 / `sub_plan`·`next_plan`→v41 로 **호출부 하드와이어**다 ⟹ 이 플랜에 버전 게이트 분기는 없다. | 3 | 사실 서술 |

<details><summary>`closed` 4건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

| # | 물음 | 닫은 근거 |
|---|---|---|
| 0 | 반환 usize(2~21)가 어느 부시 테이블의 인덱스인지 — 이 함수 본문에는 테이블 참조가 없고, 반환값을 소비하는 호출측을 담당 범위 밖이라 확인하지 않았다. AbstractGameWithCache+0x2218 region_point(108B) 가 후보로 보이지만 근거 없음(추정). | resolved[0]: MapDef.bushes(+0x1c98) 값으로 확정 |
| 1 | map_regions::is_top_side 의 극성 = **확정**. `is_top_side` = `champ.x + champ.y <= height` = `!(ry < champ.x)` (ry = height - champ.y). IR 의 `icmp ult ry, champ.x` 는 **그 부정**이다 — 2026-09-11 검증배치 C 가 SDK 오라클 실행으로 확정(표본 9개 전부 일치, 대각선 x+y==height 는 top/bottom 양쪽 true). 확인 경로 = `tcxq grep game_core is_top_side` → `vis=pub, mir=1` ⟹ 오라클·MIR 둘 다 열려 있다 | 본문에 해소 표기가 있다 |
| 2 | %0/%1/%2 의 출처 — 세 포인터 인자가 fastcc 인자승격으로 (i8,i64,i32,ptr,ptr) 이 되어 LineGankCoverPlan+0x20 / PlayerState+0x930 / PlayerState+0x9c0 을 읽는 gep 가 본문에 없다. 매핑 근거는 ①DWARF 파라미터 목록(!23059~!23061) ②인라인된 AbstractGameWithCache::tower 의 dbg_value(line=%0, team=%1) ③entity.rs:580 Position 메서드의 dbg 가 `+2496(=0x9c0)` 를 가리키는 것 — 이 세 가지 간접 증거다. 그래서 reads 에는 넣지 않았다. | 4차 배치C(ev3): 간접증거→직접증거. 호출부 m10.ll:11778~11791 에 gep 그대로 |
| 3 | OperationData+0x10(blackboard) 는 이 함수에서 읽지 않는다 — 읽지 않는다는 것만 확실하고, 다른 v* 판본이 쓰는지는 미확인. | 4차 배치C: 선택기 3종 중 target_bush 만 blackboard 를 본다(cover.rs:196 / ganker.rs:351) |
</details>

<details><summary>`history` 정정 이력 8건 (참조용 — 본문 아님)</summary>

| # | 옛 값 | 현재 | 인덱싱 | 그리드 | 부시_좌표표 |
|---|---|---|---|---|---|
| 0 | 반환 usize(2~21)가 어느 부시 테이블의 인덱스인가 | ★확정 — MapDef.bushes(+0x1c98, [30][30] usize) 배열의 '값'이다. 별도 테이블이 아니다. 근거 사슬: 반환값 → SubPlan::Hide{bush} 페이로드(m10.ll:11791~11801, 태그 9) → SmallActionAroundBush::new_with_target(m02.ll:21543) → 그 필터 클로저가 map.bushes[y][x] == bush 로 셀을 고름(m08.ll:113362~113375, +7320=0x1c98). | ★bushes[y_cell][x_cell] (바깥이 y). 근거 = near_jungle_bush 소비처 m02.ll:48891~48903 에서 pos.1/32000 이 바깥, pos.0/32000 이 안쪽. | 원본 = @anon.760a549ece99095c03f6d74498be091c.269 (_gcbc/g07.ll:275), 맵 생성자가 +7320 에 memcpy(g07.ll:152522). 부시 ID 1~24, 0=부시 없음. | [{"id": 1, "cells": "(0,0)(1,0)(2,0)(0,1)(0,2)", "center": [35200, 35200], "note": "★target_bush_v30 이 반환 안 함"}, {"id": 2, "cells": "(19,0)(20,0)(21,0)", "center": [656000, 16000]}, {"id": 3, "cells": "(7,4)(8,4)(9,4)", "center": [272000, 144000]}, {"id": 4, "cells": "(20,4)(20,5)", "center": [656000, 160000]}, {"id": 5, "cells": "(12,6)", "center": [400000, 208000]}, {"id": 6, "cells": "(4,7)(4,8)(4,9)", "center": [144000, 272000]}, {"id": 7, "cells": "(29,8)(29,9)(29,10)", "center": [944000, 304000]}, {"id": 8, "cells": "(14,9)", "center": [464000, 304000]}, {"id": 9, "cells": "(26,11)~(26,14)", "center": [848000, 416000]}, {"id": 10, "cells": "(6,12)", "center": [208000, 400000]}, {"id": 11, "cells": "(12,12)(13,12)(12,13)", "center": [410666, 410666]}, {"id": 12, "cells": "(9,14)", "center": [304000, 464000]}, {"id": 13, "cells": "(20,15)(21,15)(21,16)", "center": [677333, 506666]}, {"id": 14, "cells": "(17,16)(16,17)(17,17)", "center": [549333, 549333]}, {"id": 15, "cells": "(25,18)~(25,22)", "center": [816000, 656000]}, {"id": 16, "cells": "(0,19)(0,20)(0,21)", "center": [16000, 656000]}, {"id": 17, "cells": "(4,20)(5,20)", "center": [160000, 656000]}, {"id": 18, "cells": "(15,20)(15,21)(16,21)", "center": [506666, 677333]}, {"id": 19, "cells": "(29,24)(29,25)(29,26)", "center": [944000, 816000]}, {"id": 20, "cells": "(18,25)~(22,25)", "center": [656000, 816000]}, {"id": 21, "cells": "(11,26)~(14,26)", "center": [416000, 848000]}, {"id": 22, "cells": "(28,28)(29,28)(28,29)(29,29)", "center": [928000, 928000], "note": "★반환 안 함"}, {"id": 23, "cells": "(8,29)(9,29)(10,29)", "center": [304000, 944000], "note": "★반환 안 함"}, {"id": 24, "cells": "(24,29)(25,29)(26,29)", "center": [816000, 944000], "note": "★반환 안 함"}] |
| 1 | map_regions::is_top_side 의 원식 극성 / 부시 그리드의 y축 방향 | ★둘 다 확정 — 전문 = `_shared.맵_좌표계`. **`is_top_side = (x + y ≤ height)`**(인라인 지점의 단락평가 분기 방향으로 확정, _gcbc/g15.ll:66099~66135). 형제 `is_bottom_side = (x + y ≥ height)`, 둘 다 미드 대각선 포함. **y=0 = 게임이 `Top` 이라 부르는 절반**(대칭이 깨지는 데이터 3개가 전부 일치). ⚠단 그 절반이 화면상 위쪽인지는 렌더러 소관이라 이 IR 에 없다. |  |  |  |
| 2 | "Top 절반 = 화면 위쪽" 의 최종 한 걸음 — 렌더러 소관이라 _gaibc/_gcbc 에 없다 | ★**확정 — `_gvbc`(game_view)를 안 봤던 것이다.** 전문 = `_shared.맵_화면방향`. `GameView::render_minimap`(v10.ll:248308)이 월드→화면을 `/3.0` 스케일만 하고 **부호 반전이 없다**. `is_near_top_line`(g09.ll:159311)의 Top ⟺ `x+y < height` 와 합치면 **Top = 작은 x·작은 y = 화면 왼쪽 위**. |  |  |  |
| 3 | logic 의 `is_top_side` 본문식 표기 (자기 파일 resolved 와 내부 모순) | ★**정정(2026-09-11 검증배치 C) — 극성 반전 표기 오류**: IR 의 `icmp ult (height − y), x` 를 `is_top_side` **본문식**이라 적었는데, 그건 **그 부정**이다. `height−y < x` ⟺ `x+y > height` ⟺ **`!is_top_side`**. ⟹ 우리가 발표한 **공식 `is_top_side = (x + y ≤ height)` 자체는 맞다**(Top = 작은 x·작은 y). 틀린 것은 **IR 비교식에 붙인 이름표**뿐이고, 동작 서술은 우연히 맞았다. 다만 **뜻이 정확히 뒤집혀 재구현자를 오도**하므로 정정한다. ★**오라클 실행으로 확정**: `is_top_side` 는 `vis=pub` · `mir=1` 이라 그냥 돌릴 수 있었다 — 표본 9개 전부 `ry_lt_x == !is_top_side`, 대각선(`x+y == height`)은 **top/bottom 양쪽 true**. 🔁**범위정정**: 「별도 define 이 없어(전량 인라인) 확정 불가」 판정은 **과했다** — `tcxq grep game_core is_top_side` 한 번이면 `pub`·`mir=1` 이 보이고 **오라클·MIR 두 경로가 열려 있었다**. 판정 어휘는 **미탐색**이 맞다. |  |  |  |
| 4 | 부시 ID 1·22·23·24 는 갱킹 은신 후보에서 **영구 제외** | 🔁**범위정정(2026-09-11 검증배치 C)**: 그 주장은 **`target_bush_v30` 한정**으로만 맞다. 형제 **`target_bush_v41`** 이 부시 **7·23** 을 반환한다. 두 함수를 합친 **실제 미사용 = 1, 5, 10, 19, 22, 24**. |  |  |  |
| 5 | 명세 13 의 반환표가 IR 추론뿐이었다 | ★**실행 확인**(2026-09-11 2차배치C, `C_o1314.rs` — `LineGankCoverPlan::sub_plan` 이 pub 이라 `SubPlan::Hide{bush}` 를 그대로 읽었다). tick 0(타워 전부 생존·1차타워·`nearest_enemy=None`·챔프 `is_top_side=true`): **Top 3/6 · Mid 11(team 무관) · Bottom 15/20** — L156/L171/L186 예측과 일치. ★특히 **Mid=11 은 `is_top_side` 극성 정정이 옳다는 실행 증거**다(정정 전 표대로면 14 가 나왔어야 한다). 나머지 분기(타워 전멸 2/16·4/17·9/21, 2차타워, `nearest_enemy` 스왑, Mid 봇사이드 8/12·13/18·14)는 **미탐색** — 범위: 기본 챔피언으로는 타워를 못 부수고 챔피언 10명이 전부 `is_top_side=true`. |  |  |  |
| 6 | 13 의 미검증 분기(Mid 봇사이드 등)가 '재료 부재' 인가 | ★**재료 부재가 아니라 설정 데이터 누락(미탐색)이었다**(3차 배치C). `GameSetting::default()` 는 `height == 0` 이라 `height − y` 가 u64 **언더플로**해 `is_top_side` 가 상수 true 가 된다 — 2차의 「챔프 10명 전부 탑사이드」가 그 오진이다. 실전 height(960000)를 넣으면 같은 초기 좌표에서 **Support 2명이 false** 로 갈리고 **13/14 의 26칸을 전부 밟았다**. 정본 템플릿 = `MIG\_verify3\TEMPLATE.rs`. |  |  |  |
| 7 | 선택기 3종(`target_bush`/`_v30`/`_v41`) 중 무엇이 `Blackboard` 를 쓰는가 | ★**`target_bush` 만 쓴다**(4차 배치C). `LineGankCoverPlan::target_bush`(cover.rs:196)가 `OperationData+0x10` 을 읽고(`m10.ll:12124`, `!dbg` 사슬 cover.rs:210 → next_plan 36, `Blackboard::minion_state` blackboard.rs:379~381 인라인), `LineGankerPlan::target_bush`(ganker.rs:351)도 동일(ganker.rs:365 → next_plan 142). **`_v30`·`_v41` 은 blackboard 를 보지 않는다.** |  |  |  |
</details>

