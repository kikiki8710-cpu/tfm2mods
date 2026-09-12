---

### `07` sub_plan — 에픽 사냥+교전 플랜의 서브플랜 선택 — 귀환/부시은신/에픽사냥 3택

| 항목 | 값 |
|---|---|
| id | `epic_hunt_and_battle__sub_plan` |
| 심볼 | `_RNvMNtNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old4epic15hunt_and_battleNtB2_21EpicHuntAndBattlePlan8sub_plan` |
| 소스 | `game-ai\src\plan_legacy\old\epic\hunt_and_battle.rs:28` |
| IR | `m02.ll` 48918~49137행 |
| 경로·가시성 | `game_ai::plan_legacy::old::EpicHuntAndBattlePlan::sub_plan` · **pub** |
| 계층 | 레거시 플랜 |
| exe | `None` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r4` · 통과 4회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_ai::plan_legacy::old::EpicHuntAndBattlePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &mut game_core::DebugFrameData) -> game_ai::plan_legacy::sub_plan::SubPlan
```

<details><summary>인자 8개</summary>

| # | i | 이름 | 타입 | 역할 |
|---|---|---|---|---|
| 0 | 0 | (sret) |  | 반환값 out-ptr. 태그 i64 @+0x0 |
| 1 | 1 | self |  | 필드 1개: target_bush: Option<usize> (태그 @0x0, 값 @0x8) |
| 2 | 2 | version |  | 이 함수에선 분기에 안 쓰임. upgrade_item 으로 그대로 전달만 |
| 3 | 3 | rnd |  | 이 함수에선 직접 안 씀. upgrade_item 으로 전달만 |
| 4 | 4 | player |  | info.team(0x930), info.position(0x9c0) 만 읽음 |
| 5 | 5 | data |  | cache(0x0)=&AbstractGameWithCache, context(0x8)=&GameContext |
| 6 | 6 | goal_data |  | epic(0x78) 스탠스의 tick 두 개만 읽음 |
| 7 | 7 | _debug |  | readnone — 본문에서 전혀 안 씀 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
// L29
team = player.info.team // PlayerState+0x930
if team >= 2 { panic_bounds_check(team, 2) }
pos = player.info.position.as_index() // PlayerState+0x9c0, 0..=4 (인라인, 바운드체크 없음)
champ = data.cache.player_champion[team][pos] // cache+0x1e0, [5]ptr stride 40 + pos*8
if champ == null { option::unwrap_failed() } // .unwrap()

// L31
if champ.stat_cached.hp == 0 { panic_const_div_by_zero() }
hp_ratio = champ.hp * 100 / champ.stat_cached.hp // usize 나눗셈(백분율)

// L32
mode = (*data.cache.game.vtable[0x40])(data.cache.game.ptr) // get_game_mode -> GameMode
if discriminant(mode) != 0 { option::unwrap_failed() } // as_moba().unwrap(), 0=Moba
moba = payload(mode) // &MobaMode
live = moba.jungle_runner.epic.live_list // Vec<usize> (MobaMode+0x198)
epic: Option<&Entity> =
 if live.len == 0 { None }
 else { (*data.cache.game.vtable[0x1f0])(data.cache.game.ptr, live[0]) } // get_entity_by_id
 // 소스상 live_list.get(0).and_then(|e| data.cache.game.get_entity_by_id(*e))

// L33
(lx, ly, rx, ry) = data.context.map.fountains[team] // MapDef+0x6d70, stride 32

// L34
is_in_heal_area = (champ.x >= lx && champ.x <= rx)
 && (champ.y >= ly && champ.y <= ry) // x는 lx..rx, y는 ly..ry (단축평가: x 실패면 false)

// L35
can_upgrade_item = game_ai::upgrade_item(version, rnd, player,
 data.cache.game /*fat ptr 2개*/,
 data.context).is_some()
 // 반환 Option<(usize,usize)>(24B) 의 판별자 != 0 을 is_some 으로 씀

// L36 — 귀환 판정
if epic.is_some_and(|e| e.hp == e.stat_cached.hp) // 에픽이 살아있고 풀피(=아직 아무도 안 때림)
 && ( hp_ratio < 51
 || can_upgrade_item
 || (champ.hp < champ.stat_cached.hp && is_in_heal_area) )
{
 return SubPlan::Recall // 태그 5
}

// L41 — 부시 은신 판정
if data.context.setting.tick_per_second + goal_data.epic.epic_ally_tick
 > goal_data.epic.epic_ally_killed_tick
 && self.target_bush.is_some()
{
 // L42~43
 return SubPlan::Hide(HideSubPlan {
 bush: self.target_bush.unwrap(), // self+0x8
 out_line: AroundBushOutlineType::Outline, // 1
 check_move: false,
 enemy_spotted_me: false,
 }) // 태그 9
}

// L47 — 기본
return SubPlan::EpicHunt(EpicHuntSubPlan { need_recall: false }) // 태그 11

// 주: L36 의 세 OR 항은 LLVM 이 재결합해 한 블록(%93)에서 전부 계산된다.
// is_some_and 만 단축평가로 지배 분기(%86/%92)로 남아 있어 소스 순서가 확정된다.
// L41 의 && 는 select(i1 %109, i1 %111, false) 형태 — tick 비교가 먼저다.
```

**`mem` 메모리 접근 28건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev |
|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | 팀 인덱스. 2 이상이면 panic_bounds_check · tcx 정본 대조( tcxdict **tcx 정본** 대조 OK + ⓐ `offset_of!` 실행 교차검증 MISMATCH 0 (B6_o1.tsv, 78행+구조체크기 7건) — 오프셋·필드명 한정) | 3 |
| 1 | PlayerState | 0x9c0 | info.position | r | Position(i32, range 0..=4). as_index() 인라인 — 그대로 0~4 인덱스로 씀 · tcx 정본 대조( tcxdict **tcx 정본** 대조 OK + ⓐ `offset_of!` 실행 교차검증 MISMATCH 0 (B6_o1.tsv, 78행+구조체크기 7건) — 오프셋·필드명 한정) | 3 |
| 2 | OperationData | 0x0 | cache | r | &AbstractGameWithCache · tcx 정본 대조( tcxdict **tcx 정본** 대조 OK + ⓐ `offset_of!` 실행 교차검증 MISMATCH 0 (B6_o1.tsv, 78행+구조체크기 7건) — 오프셋·필드명 한정) | 3 |
| 3 | OperationData | 0x8 | context | r | &GameContext · tcx 정본 대조( tcxdict **tcx 정본** 대조 OK + ⓐ `offset_of!` 실행 교차검증 MISMATCH 0 (B6_o1.tsv, 78행+구조체크기 7건) — 오프셋·필드명 한정) | 3 |
| 4 | AbstractGameWithCache | 0x0 | game | r | &dyn AbstractGame 팻포인터 — 데이터ptr @+0x0, vtable @+0x8 · tcx 정본 대조( tcxdict **tcx 정본** 대조 OK + ⓐ `offset_of!` 실행 교차검증 MISMATCH 0 (B6_o1.tsv, 78행+구조체크기 7건) — 오프셋·필드명 한정) | 3 |
| 5 | AbstractGameWithCache | 0x1e0 | player_champion | r | [[Option<&Entity>;5];2] (80B). [team]*40 + [pos]*8 로 인덱싱. null 이면 option::unwrap_failed · tcx 정본 대조( tcxdict **tcx 정본** 대조 OK + ⓐ `offset_of!` 실행 교차검증 MISMATCH 0 (B6_o1.tsv, 78행+구조체크기 7건) — 오프셋·필드명 한정) | 3 |
| 6 | dyn AbstractGame vtable | 0x40 | get_game_mode | r | divtable 확인. GameMode(128bit) 반환 — 판별자 0=Moba | 3 |
| 7 | dyn AbstractGame vtable | 0x1f0 | get_entity_by_id | r | divtable 확인. (id: usize) -> Option<&Entity>(널러블 ptr) | 3 |
| 8 | MobaMode | 0x1a0 | jungle_runner.epic.live_list.ptr | r | Vec<usize> 는 0x198 시작이나 실제 레이아웃이 {cap@+0, ptr@+8, len@+16} 로 재정렬됨 · tcx 정본 대조( tcxdict/tcxaudit **tcx 정본** 대조 OK(오귀속 0·밀림 0) — 오프셋·필드명 한정) | 3 |
| 9 | MobaMode | 0x1a8 | jungle_runner.epic.live_list.len | r | 0 이면 epic=None. 아니면 [0] 번째 id 만 씀 · tcx 정본 대조( tcxdict/tcxaudit **tcx 정본** 대조 OK(오귀속 0·밀림 0) — 오프셋·필드명 한정) | 3 |
| 10 | GameContext | 0x8 | setting | r | &GameSetting · tcx 정본 대조( tcxdict **tcx 정본** 대조 OK + ⓐ `offset_of!` 실행 교차검증 MISMATCH 0 (B6_o1.tsv, 78행+구조체크기 7건) — 오프셋·필드명 한정) | 3 |
| 11 | GameContext | 0x20 | map | r | &MapDef · tcx 정본 대조( tcxdict **tcx 정본** 대조 OK + ⓐ `offset_of!` 실행 교차검증 MISMATCH 0 (B6_o1.tsv, 78행+구조체크기 7건) — 오프셋·필드명 한정) | 3 |
| 12 | GameSetting | 0x12f8 | tick_per_second | r | line41 에픽 타이밍 게이트의 여유값(1초분 tick) · tcx 정본 대조( tcxdict **tcx 정본** 대조 OK + ⓐ `offset_of!` 실행 교차검증 MISMATCH 0 (B6_o1.tsv, 78행+구조체크기 7건) — 오프셋·필드명 한정) | 3 |
| 13 | MapDef | 0x6d70 | fountains | r | [ (lx,ly,rx,ry); 2 ] · stride 32B. team 으로 인덱싱. 내부 +0=lx, +8=ly, +16=rx, +24=ry ★오라클 실측 = `[(0, 896000, 64000, 960000), (892000, 0, 960000, 64000)]`. 명명 구조체가 아니라 **4-튜플 (u64,u64,u64,u64)** 이고 `f.0<=f.2 · f.1<=f.3`(min/max 코너). `fountains[i]` = 팀 i **자기** 분수대 | 3 |
| 14 | Entity | 0x670 | hp | r | champ(현재 HP) 와 epic(현재 HP) 양쪽에서 읽음 · tcx 정본 대조( tcxdict **tcx 정본** 대조 OK + ⓐ `offset_of!` 실행 교차검증 MISMATCH 0 (B6_o1.tsv, 78행+구조체크기 7건) — 오프셋·필드명 한정) | 3 |
| 15 | Entity | 0x628 | stat_cached.hp | r | 최대 HP. 0 이면 panic_const_div_by_zero · tcx 정본 대조( tcxdict **tcx 정본** 대조 OK + ⓐ `offset_of!` 실행 교차검증 MISMATCH 0 (B6_o1.tsv, 78행+구조체크기 7건) — 오프셋·필드명 한정) | 3 |
| 16 | Entity | 0x660 | x | r | champ 좌표 — 분수대 사각형 판정 · tcx 정본 대조( tcxdict **tcx 정본** 대조 OK + ⓐ `offset_of!` 실행 교차검증 MISMATCH 0 (B6_o1.tsv, 78행+구조체크기 7건) — 오프셋·필드명 한정) | 3 |
| 17 | Entity | 0x668 | y | r | champ 좌표 — 분수대 사각형 판정 · tcx 정본 대조( tcxdict **tcx 정본** 대조 OK + ⓐ `offset_of!` 실행 교차검증 MISMATCH 0 (B6_o1.tsv, 78행+구조체크기 7건) — 오프셋·필드명 한정) | 3 |
| 18 | GoalData | 0x98 | epic.epic_ally_tick | r | EpicStanceData(0x78) 내부 +0x20 · tcx 정본 대조( tcxdict **tcx 정본** 대조 OK + ⓐ `offset_of!` 실행 교차검증 MISMATCH 0 (B6_o1.tsv, 78행+구조체크기 7건) — 오프셋·필드명 한정) | 3 |
| 19 | GoalData | 0xa0 | epic.epic_ally_killed_tick | r | EpicStanceData(0x78) 내부 +0x28 · tcx 정본 대조( tcxdict **tcx 정본** 대조 OK + ⓐ `offset_of!` 실행 교차검증 MISMATCH 0 (B6_o1.tsv, 78행+구조체크기 7건) — 오프셋·필드명 한정) | 3 |
| 20 | EpicHuntAndBattlePlan | 0x0 | target_bush 판별자 | r | Option<usize> 태그. trunc i1 로 is_some 판정 · tcx 정본 대조( tcxdict/tcxaudit **tcx 정본** 대조 OK(오귀속 0·밀림 0) — 오프셋·필드명 한정) | 3 |
| 21 | EpicHuntAndBattlePlan | 0x8 | target_bush 값 | r | Some 일 때의 부시 인덱스(usize) · tcx 정본 대조( tcxdict/tcxaudit **tcx 정본** 대조 OK(오귀속 0·밀림 0) — 오프셋·필드명 한정) | 3 |
| 22 | SubPlan(sret) | 0x0 | 판별자 | w | phi 로 모임: 5=Recall, 9=Hide, 11=EpicHunt · tcx 정본 대조( tcxdict/tcxaudit **tcx 정본** 대조 OK(오귀속 0·밀림 0) — 오프셋·필드명 한정) | 3 |
| 23 | SubPlan(sret) | 0x8 | Hide.__0.bush | w | 태그 9 경로에서만 i64 로 기록 · tcx 정본 대조( tcxdict/tcxaudit **tcx 정본** 대조 OK(오귀속 0·밀림 0) — 오프셋·필드명 한정) | 3 |
| 24 | SubPlan(sret) | 0x8 | EpicHunt.__0.need_recall | w | 태그 11 경로에서 i8 0 기록 — 같은 오프셋을 1바이트로 씀 · tcx 정본 대조( tcxdict/tcxaudit **tcx 정본** 대조 OK(오귀속 0·밀림 0) — 오프셋·필드명 한정) | 3 |
| 25 | SubPlan(sret) | 0x10 | Hide.__0.out_line | w | 0=None,1=Outline,2=Inline · tcx 정본 대조( tcxdict/tcxaudit **tcx 정본** 대조 OK(오귀속 0·밀림 0) — 오프셋·필드명 한정) | 3 |
| 26 | SubPlan(sret) | 0x11 | Hide.__0.check_move | w | · tcx 정본 대조( tcxdict/tcxaudit **tcx 정본** 대조 OK(오귀속 0·밀림 0) — 오프셋·필드명 한정) | 3 |
| 27 | SubPlan(sret) | 0x12 | Hide.__0.enemy_spotted_me | w | · tcx 정본 대조( tcxdict/tcxaudit **tcx 정본** 대조 OK(오귀속 0·밀림 0) — 오프셋·필드명 한정) | 3 |

**`consts` 상수 7건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 100 | 31 | 임계 | hp_ratio = hp*100/max_hp — 백분율 변환 계수 · 오라클 실행 확증( 오라클 실행 확증: hp_ratio 정수나눗셈 509/1000 → Recall · 510/1000 → EpicHunt ) | 2 |
| 1 | 51 | 36 | 태그 | ★귀환 HP 임계. hp_ratio < 51 (=HP 50% 이하)이면 귀환 후보. ★**오라클 8/8 **: 경계가 **정확히 50/51** 이다( 재확인 49/50·51/52 24/24). 세 번째 OR 항이 `hp < max && in_heal_area` 인 것도 확증. 태그 5=Recall · 11=EpicHunt 런타임 확인 | 2 |
| 2 | 5 | 36 | 태그 | SubPlan 태그 5 = Recall (dienum 확인) | 3 |
| 3 | 9 | 43 | 태그 | SubPlan 태그 9 = Hide (dienum 확인) | 3 |
| 4 | 11 | 47 | 태그 | SubPlan 태그 11 = EpicHunt (dienum 확인) | 3 |
| 5 | 1 | 43 | 임계 | AroundBushOutlineType::Outline — Hide 서브플랜의 부시 접근 방식 · 오라클 실행 확증( 오라클 실행 확증: Hide 페이로드 out_line=1(Outline) bush 0·7·26 전부 ) | 2 |
| 6 | 0 | 32 | 임계 | live_list.get(0) — 살아있는 에픽 목록의 첫 원소만 본다. 동시에 need_recall/check_move/enemy_spotted_me 의 false 값이기도 함 · 오라클 실행 확증( 오라클 실행 확증: check_move=0 · enemy_spotted_me=0 · need_recall=0 · live_list.get(0) ) | 2 |

**`knobs` 조정점 14건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev |
|---|---|---|---|---|---|
| 0 | 귀환 HP 임계 | hunt_and_battle.rs:36 | 51 | hp_ratio < 51 일 때 에픽 전 귀환. 올리면 더 높은 HP에서도 귀환(=에픽 앞 세팅을 더 자주 함), 내리면 낮은 HP까지 버티며 에픽 사냥을 계속한다 · 오라클 실행 확증( 오라클 실행 확증: hp 50 → Recall / 51 → EpicHunt (챔프를 분수 밖으로 옮긴 뒤) ) | 2 |
| 1 | Hide 서브플랜의 부시 접근 방식 | hunt_and_battle.rs:43 | 1 | AroundBushOutlineType — 1(Outline)=부시 외곽선에 붙음. 0(None)이면 위치 제약 없이, 2(Inline)이면 부시 안쪽으로 들어가 대기한다 · 오라클 실행 확증( 오라클 실행 확증: Hide.out_line=1, bush 0·7·26 전부 ) | 2 |
| 2 | EpicHunt 진입 시 need_recall 플래그 | hunt_and_battle.rs:47 | 0 | 항상 false 로 고정. true 로 바꾸면 EpicHuntSubPlan 이 '먼저 귀환 필요' 상태로 시작한다(하위 서브플랜 동작이 바뀜) · 오라클 실행 확증( 오라클 실행 확증: EpicHunt 페이로드 need_recall=0 ) | 2 |
| 3 | 에픽 은신 타이밍 게이트 | hunt_and_battle.rs:41 | `tick_per_second`(GameSetting+0x12f8) × 계수 1 — ★**리터럴이 아니라 런타임 값**이다. 10진 4856 = 오프셋 **0x12f8**(GameSetting 내 `tick_per_second` 위치) | GameSetting+0x12f8(tick_per_second) 을 여유값으로 써서 `tick_per_second + epic_ally_tick > epic_ally_killed_tick` 일 때만 Hide 로 간다. 이 여유값을 키우면 게이트가 더 오래 열려 부시 은신을 더 자주 고른다 (단 tick_per_second 는 전역 설정이라 여기만 바꾸는 노브는 아님 — 개조하려면 이 비교식 자체를 갈아야 한다) · 오라클 실행 확증( 오라클 실행 확증: tps60 k=1059→Hide/1060→EpicHunt · tps30 29/30 · tps1 0/1 ⟹ 계수 1 · `>` 엄격 ) | 2 |
| 4 | 귀환 HP% 임계 | hunt_and_battle.rs:36 / m02.ll:49087 | 51 | `icmp ult` 라 51 = 'HP 50% 이하'. 올리면 더 건강할 때도 귀환 · 오라클 실행 확증( 오라클 실행 확증: hp 50/51 경계 ) | 2 |
| 5 | 은신 게이트 마진 | hunt_and_battle.rs:41 / m02.ll:49099~49103 | tick_per_second(=1초) | ★리터럴 아님. add 항을 `tps*k` 로 바꾸면 k↑ = 더 소극적(자주 숨음), k=0 = 팽팽하면 사냥 강행 · 오라클 실행 확증( 오라클 실행 확증: tps 3종에서 마진 = tick_per_second × 1 ) | 2 |
| 6 | 에픽 무손상 조건 | m02.ll:49081 (icmp eq epic.hp, epic.max_hp) | == | 이 항을 죽이면 에픽이 이미 깎인 상황에서도 귀환 선택지가 열린다 · 오라클 실행 확증( 오라클 실행 확증: 에픽 hp=max → Recall / hp=max−1 → EpicHunt ) | 2 |
| 7 | Hide.out_line | m02.ll:49122 (store i8 1) | 1 = AroundBushOutlineType::Outline | ★정정 — 0=None / 1=Outline / 2=Inline. 부시 안까지 들어갈지 외곽에 설지 · 오라클 실행 확증( 오라클 실행 확증: Hide.out_line 이 bush 3종 전부 1 ) | 2 |
| 8 | 업글 후보 tier 필터 | lib.rs:1610 / m14.ll:6708 | tier < 4 | 보유템 중 '최고 tier' 산정에서 tier>=4 를 제외. 올리면 상위템 보유 시 추가 업글을 막게 된다 | 4 |
| 9 | 업글 후보 3조건 | lib.rs:1626 / m14.ll:6985·7006·7018 | is_active && tier>cur && price<=gold | `price<=gold` 를 `<=gold*x/100` 로 바꾸면 '돈 아껴두기' 구현 가능 | 4 |
| 10 | 업글 후보 선택 방식 | lib.rs:1634 / m14.ll:6845 | 균등 랜덤 gen_range(0..len) | 점수 기반 선택으로 바꿀 자연스러운 개입 지점. ⚠PRNG 를 소비하므로 리플레이 시드에 영향 | 4 |
| 11 | DPS 레이스에 세는 아군 범위 | m09.ll:66469~66568 | in_same_goal(tag2) + region_dist[r][7]==0 | 넓히면 멀리 있는 아군까지 계산에 포함 → 덜 숨는다 | 4 |
| 12 | 맵 그리드 원본 6종 | _gcbc/g07.ll:274~279 (@anon.760a549…091c.268~.273) | walls/bushes/regions/region_dist/region_centers/is_line_region | **맵 자체를 개조하려면 이 6개 전역이 유일 진입점**(생성자가 memcpy 만 한다 — 계산 없음) | 4 |
| 13 | region_dist 홉거리 행렬 | _gcbc/g07.ll:277 | 27×27 대칭, max 6 | AI 의 '얼마나 먼 지역인가' 임계가 전부 이 정수 홉수 기준 | 4 |

<details><summary>`callees` 피호출자 9건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 |
|---|---|---|---|---|---|
| 0 | as_index | game_core::Position::as_index | pub | fn(&game_core::Position) -> usize | game-core\src\simulation\entity.rs:580 |
| 1 | as_index | game_core::BigGoal::as_index | pub | fn(&game_core::BigGoal) -> usize | game-core\src\simulation\ai_interface.rs:338 |
| 2 | as_moba | game_core::GameMode::<'a>::as_moba | pub | fn(game_core::GameMode<'a/#0>) -> std::option::Option<&'a/#0 game_core::MobaMode> | game-core\src\simulation\game.rs:230 |
| 3 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 |
| 4 | get_entity_by_id | <game_core::ExpectedGame<'a> as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::ExpectedGame<'a/#0>, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\expected_game.rs:165 |
| 5 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 |
| 6 | upgrade_item | <game_ai::AgentVerHamster as game_core::AiAgent>::upgrade_item | pub | fn(&mut game_ai::AgentVerHamster, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::GameContext) -> std::option::Option<(usize, usize)> | game-ai\src\lib.rs:440 |
| 7 | upgrade_item | game_ai::AgentVerHamster::upgrade_item | pub | fn(&mut game_ai::AgentVerHamster, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::GameContext) -> std::option::Option<(usize, usize)> | game-ai\src\lib.rs:1193 |
| 8 | upgrade_item | game_ai::upgrade_item | in:game_ai | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::GameContext) -> std::option::Option<(usize, usize)> | game-ai\src\lib.rs:1603 |
</details>

⚠**미매칭 2개**: `discriminant`, `payload`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m02.ll:8466) · **형제 8개** (EpicHuntAndBattlePlan)

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

**`open` 1건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 표기 불가 | L36 세 OR 항의 소스상 원래 순서 — LLVM 이 %94\|%85, %96&%83 으로 재결합해 한 블록에 몰아넣었고 !dbg 가 전부 line 36 이라 소스 순서를 줄번호로 복원할 수 없다. 논리값은 순서와 무관하므로 판정에는 영향 없음 | 4 |  |

<details><summary>`closed` 9건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

| # | 물음 | 답 | ev |
|---|---|---|---|
| 0 | goal_data.epic.epic_ally_tick / epic_ally_killed_tick 의 정확한 의미(어느 사건의 tick 인지)는 이 함수만으로 확정 불가. 여기서 관측된 사실은 `tick_per_second + ally_tick > ally_killed_tick && target_bush.is_some()` 이면 Hide 라는 것뿐이다. 필드명으로 미루어 '아군이 에픽에 붙은 tick' / '아군이 에픽을 잡은 tick' 으로 추정되나 갱신 지점은 이 범위 밖이라 안 봄 |  |  |
| 1 | game_ai::upgrade_item 내부(m14.ll 6620~7067)는 안 봄 — can_upgrade_item 이 실제로 무엇을 보고 Some 을 내는지 미확인. 여기서는 Option<(usize,usize)>(24B)의 판별자 != 0 만 쓴다 |  |  |
| 2 | version(p2)·rnd(p3) 는 이 함수 본문에서 분기·계산에 전혀 안 쓰이고 upgrade_item 으로만 전달된다. 즉 AI 버전 게이트는 이 함수에 없다 |  |  |
| 3 | _debug(p7) 은 readnone — 본문에서 미사용 |  |  |
| 4 | vtable 슬롯 0x40/0x1f0 은 divtable.py 가 ExpectedGame 구현 테이블에서 get_game_mode / get_entity_by_id 로 해석한 것이다. 실제 런타임 구현체(Game vs ExpectedGame)에 따라 함수 실체는 다를 수 있으나 슬롯 의미는 동일하다고 본다 |  |  |
| 5 | MobaMode 의 live_list 시작 오프셋: distruct 는 0x198 로 보고하는데 IR 은 ptr=0x1a0 / len=0x1a8 을 읽는다. Vec<usize> 내부가 {cap@+0, ptr@+8, len@+16} 로 재정렬된 것으로 판단해 교차검증했으나(len==0 체크 → 0 이면 None, 아니면 ptr 역참조), Vec 필드 순서 자체는 DWARF 로 직접 확인하지 못했다 |  |  |
| 6 | champ 을 얻는 인덱싱에서 team 은 2 로 바운드체크되지만 pos 는 range 0..=4 로 이미 좁혀져 체크가 사라졌다. 상수 2 는 배열 상한(바운드체크)이라 §3 규칙대로 constants 에서 제외했다 |  |  |
| 7 | SubPlan 반환 시 Recall(태그5) 경로는 sret 의 페이로드 영역을 전혀 안 쓴다(태그만 기록) — 호출측이 비초기화 바이트를 읽지 않는다는 전제. 확인 안 함 |  |  |
| 8 | `hunt_and_battle.rs:36` 두 OR 항(`hp_ratio < 51` vs `upgrade_item(..).is_some()`)의 소스 순서 — ★**재료 부재로 종결. 5경로 전부 막힌 것을 실측 확인(2026-09-11 검증배치 B)**:   ① MIR — `mir=0 xinl=0`(rmeta 미인코딩)   ② `DILocation.column` — 전 모듈 0   ③ **패닉 Location 컬럼 — L36 에 패닉 가능 연산이 하나도 없어 상수 자체가 없다**(이 파일 Location 은 21:17·21:95·23:93·29:17·29:95·31:20·32:58·56:58 **8개뿐**, 36행 없음)   ④ **줄 길이 산술 — `A\|\|B` 와 `B\|\|A` 는 길이가 같다. 방법의 불변량이라 원리적 불가**   ⑤ ★**IR 피연산자 순서 — 정보량 0으로 반증됨**. ~~「순서 보존 경향상 `hp_ratio<51` 이 좌항일 가능성」~~ 이라는 추정은 **무효**다: 같은 식의 바깥 `or` 가 `or(%97, %95)` 로 **소스 역순**이고, 두 or 모두 **정의 순서(rank) 내림차순**으로 설명된다. 게다가 `can_upgrade_item`(%85)은 L35 호출 결과라 소스에서 좌항이든 우항이든 **반드시 `%94`(L36 생성)보다 먼저 정의**되므로 관측된 `or(%94,%85)` 는 **어느 소스 순서에서도 동일하게 나온다**. 미탐색 = 게임 exe 디스어셈(같은 IR 산물이라 기대치 낮음) · 개발사 소스. **판정 결과에는 영향 없다.** |  |  |
</details>

<details><summary>`history` 정정 이력 11건 (참조용 — 본문 아님)</summary>

| # | 옛 값 | 현재 | 근거 |
|---|---|---|---|
| 0 | goal_data.epic.epic_ally_tick / epic_ally_killed_tick 의 정확한 의미(어느 사건의 tick 인지) 확정 불가 | ★★확정 — **둘 다 '사건이 일어난 시각(timestamp)'이 아니라 '예상 소요 틱(duration)'이다.** 이름의 `tick` 이 오해를 부른다.   `epic_ally_tick` = **아군이 에픽을 죽이는 데 걸릴 예상 틱** = `check_epic_kill_time_with_hp(epic, ally_list, self.last_epic_hp)`   `epic_ally_killed_tick` = **에픽이 아군 전원을 죽이는 데 걸릴 예상 틱의 합** = Σ_ally( ally.hp*1000 / (에픽 평타 expected_damage_target(ally)*1000 / 에픽 attack_cooltime) ) |  |
| 1 | game_ai::upgrade_item 내부 — can_upgrade_item 이 무엇을 보고 Some 을 내는지 | ★확정(m14.ll:6620~7069, lib.rs:1603). 담당 함수의 로컬명 `can_upgrade_item`(!54394, hunt_and_battle.rs:35)은 실체가 `upgrade_item(...).is_some()` 이고 별도 함수는 IR 에 없다.   cur = 보유 아이템 중 tier<4 인 것들의 최대 tier(없으면 0)  [m14.ll:6708, lib.rs:1610]   후보 = 각 보유템의 next_tier() 키로 item_list 를 찾아 `is_active && tier>cur && price<=gold` 인 것  [6985/7006/7018]   비면 None, 아니면 **균등 랜덤 1개** `gen_range(0..len)`  [6845]   ⟹ 반환 두 usize = **`.0` = 교체 대상인 내 아이템 슬롯 인덱스 / `.1` = 사들일 아이템의 전역 `ctx.item_list` 인덱스** |  |
| 2 | SubPlan Recall(태그5) 경로가 sret 페이로드를 안 쓰는데 호출측이 비초기화 바이트를 읽는지 | ★확정 · 문제 없음. **Recall 의 페이로드는 ZST 다.** m12.ll:!6963~!6970 — `Variant3.value = Recall{__0: RecallSubPlan}`, DISCR_EXACT=5(!6973), `!6970 = DICompositeType(name:"RecallSubPlan", elements: !8)` = **size 속성 없음 + 빈 멤버 = 0바이트**. Recall 구조체의 size 576(=72B)은 enum 전체 패딩일 뿐. 실제 소비자 `RecallSubPlan::action_candidates`(m02.ll:40152)의 self 가 `readnone captures(none)` — LLVM 이 '절대 안 읽음'을 보증한다. |  |
| 3 | 에픽 은신 타이밍 게이트 4856 | ★★**노브 설명 정정 — 4856 은 상수가 아니라 구조체 오프셋이다.** m02.ll:49096~49099 에서 `%59(=data.context) + 8` → `ctx.setting(&GameSetting)`, 그 `+4856` = **`GameSetting+0x12f8 = tick_per_second`**. SPEC_GUIDE §3 규칙상 `reads` 로 가야 하고 `constants` 에 있으면 안 된다. 실제 게이트는 `tick_per_second + epic_ally_tick > epic_ally_killed_tick` = **1초 마진**이고 승수 리터럴은 IR 에 없다(add 1회, 계수 1). |  |
| 4 | L36 세 OR 항의 소스 순서 | 부분확정 + 나머지는 ****재료 부재로 종결**(범위 = still_unknown[0] 의 5경로). ~~원리적 불가~~ 는 디버그정보 경로 한 가지에만 해당하는데 전 범위 표현을 썼다(판정어휘 위반, 2차배치B)**. m02.ll 전체에 `DILocation(column:)` 이 **0건**이라 같은 줄 안의 순서는 디버그정보로 복원 불가. 다만 or-트리 형태(m02.ll:49087~49091)에서 내부 or 가 `{hp_ratio<51, can_upgrade_item}` 을 묶으므로 **세 번째 항 = `(champ.hp < champ.stat_cached.hp && is_in_heal_area)` 로 확정**. 앞 두 항의 상대 순서만 불가(같은 or 명령의 두 피연산자 + InstCombine 정규화). |  |
| 5 | region_dist[r][7] 의 인덱스 7 이 '에픽 지역'인지 — 추정 | ★**확정 = Morgard(에픽) 지역.** 전문 = `_shared.맵_좌표계`. `regions[9][9] = 7`(Morgard 셀 9,9), 교차검증 3중(`region_centers[7] ≈ Morgard 좌표` / `is_line_region[7] = 0` 비라인 / gep `+21776 = 21720 + 7*8`). region 수 = **27 확정**. |  |
| 6 | Blackboard::in_same_goal(slot, X) 의 X(24B, tag 2)가 무슨 Goal 인지 | ★**확정 = `BigGoal::Epic`.** `_gcbc/g07.ll:156608`(blackboard.rs:147), 시그니처 `in_same_goal(&self, position: usize, goal: BigGoal)`. `switch i8` 0~6 = BigGoal 태그(밀림 없음). 태그 2 팔(g07.ll:156745~156763) = `big_goal[position].1 == 2`(Blackboard+0xf0 + position*32, 태그 바이트 +0x8). Epic 은 fieldless 라 페이로드 비교 없음. |  |
| 7 | upgrade_item 의 5번째 인자(%4, ptr nonnull readnone) | ★확정 = **`_game: &dyn AbstractGame` 의 데이터 포인터**(`%4`+`%5` 가 팻포인터 쌍). DWARF 인자 = `_version`/`rnd`/`player`/**`_game`**/`context`. 이름이 언더스코어 접두라 **의도적 미사용** — 본문에서 안 읽히는 게 정상이고 판정에 무관하다. |  |
| 8 | 오라클로 실행 검증이 가능한가 | ★가능(2026-09-11 2차배치B). `EpicHuntAndBattlePlan: Default` + `sub_plan` pub. 실측(`B_o1.tsv`): version ∈ {0,1,2,3,40,50,60} 전부 **tag=11 EpicHunt(EpicHuntSubPlan{need_recall:false})** ⟹ ①태그 11=EpicHunt ②need_recall=false ③**version 게이트 없음**을 실행 확인. 한계(입력 판별력): 에픽 엔티티 부재로 Recall(5) 미판별, target_bush=None 이라 Hide(9) 미판별. 미탐색 = `MobaMode.live_list` 주입. |  |
| 9 | 07 `hp_ratio < 51` 경계와 세 번째 OR 항 | ★**오라클 8/8**(3차 배치B, ev2). `hp_ratio < 51` 경계가 **정확히 50/51** 에서 갈린다. 세 번째 OR 항이 `hp < max && in_heal_area` 인 것도 확증. 태그 5=Recall · 11=EpicHunt 런타임 확인. `target_bush` 가 private 이라 Hide(9) 경로만 미도달. ⚠지시문의 「L36 **두** OR 항」은 오기 — 실제 **세 항**이고 명세가 맞다. |  |
| 10 | 07 `Hide`(태그 9) 경로는 `target_bush` 가 private 이라 오라클 미도달(3차) | ★**개방**(4차 배치B). `transmute` 로 뚫어 `Hide(HideSubPlan{bush:7, out_line:Outline, …})` **4필드 전량 확증**. ⟹ 3차의 「미도달」은 **접근 방법의 한계**였고 경로 자체는 살아 있다(§11 판정범위 규칙의 실례). |  |
</details>

