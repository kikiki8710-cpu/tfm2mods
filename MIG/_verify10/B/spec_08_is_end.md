---

### `08` is_end — 에픽(Morgard) 캠프 헌트앤포크 플랜을 접을지 판정 — 목표이탈·적 근접·에픽 리스폰 대기시간

| 항목 | 값 |
|---|---|
| id | `epic_hunt_and_poke__is_end` |
| 심볼 | `_RNvMNtNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old4epic13hunt_and_pokeNtB2_19EpicHuntAndPokePlan6is_end` |
| 소스 | `game-ai\src\plan_legacy\old\epic\hunt_and_poke.rs:163` |
| IR | `m10.ll` 7655~7899행 |
| 경로·가시성 | `game_ai::plan_legacy::old::EpicHuntAndPokePlan::is_end` · **pub** |
| 계층 | 레거시 플랜 |
| exe | `defa20` (hunt_and_poke) · 675바이트 · 165명령 |
| 라운드 | 기준 `r6` · 통과 6회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_ai::plan_legacy::old::EpicHuntAndPokePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bool
```

<details><summary>인자 7개</summary>

| # | i | 이름 | 타입 | 역할 |
|---|---|---|---|---|
| 0 | 1 | self |  | readonly captures(none) — 본문에서 단 한 번도 역참조하지 않는다(gep 없음). focus_epic_only/vision_only/v46_flee 모두 미사용 |
| 1 | 2 | version |  | AI 버전 게이트. 이 함수 자체엔 분기 없고 v24_objective_setup_should_release_to_passive 로 그대로 전달만 됨 |
| 2 | 3 | _rnd |  | readnone — 미사용 |
| 3 | 4 | player |  | info.team(+0x930)만 읽고, 나머지는 필터 클로저로 전달 |
| 4 | 5 | data |  | cache(+0x0)/context(+0x8)/blackboard(+0x10) 세 필드 전부 사용 |
| 5 | 6 | team_plan |  | objective(+0x41f, 3B) 만 읽음. 이 함수에서 쓰기는 없다(이름이 take_* 지만 실제 mutate 없음) 근거: tcx 정본 sig 에 `mut` 없음 + 같은 명세의 `sig.tcx` 문자열과 자기모순이었다 + IR m10.ll:7655 의 `%5` 에 store 0건(gep 2곳 모두 load) |
| 6 | 7 | _debug |  | readnone — 미사용 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
// hunt_and_poke.rs:163 EpicHuntAndPokePlan::is_end(self, version, _rnd, player, data, team_plan, _debug) -> bool
// self / _rnd / _debug 는 본문에서 한 번도 읽지 않는다(readnone / gep 없음).

let team = player.info.team; // PlayerState+0x930
let game = data.cache.game; // &dyn AbstractGame (data ptr + vtable)

// ── L164 : 팀 주목표가 더 이상 에픽이 아니면 즉시 종료 ─────────────────
// 소스: if !team_plan.take_active(JungleType::Morgard) { return true }
// IR 실체: objective 태그 바이트 한 번 비교로 접힘
if (u8)TeamPlan[0x41f] != 0 { return true } // 0 = MainObjective::Morgard, 255 = None

// ── L169 : 에픽 캠프 좌표 ──────────────────────────────────────────
let (cx, cy) = data.context.map.camp_pos(JungleType::Morgard, team == 0); // (u64,u64)

// ── L172 : setup 단계인가 ─────────────────────────────────────────
// 소스: let setup_like = team_plan.take_setup_like(JungleType::Morgard)
let setup_like = ((u8)TeamPlan[0x41f] == 0) && ((u8)TeamPlan[0x420] == ObjectPhase::Setup /*1*/);

if setup_like { // L173
 // L174 — 오브젝티브 규율 계층이 '수동으로 풀어라'라고 하면 종료
 if TeamPlan::v24_objective_setup_should_release_to_passive(team_plan, version, player, data, JungleType::Morgard) {
 return true;
 }

 // L179 — 우리 팀 시야에 에픽 캠프 셀이 보이는가
 if game.is_visible_cell(team, cx / 32000, cy / 32000) { // vtable +0x100

 // L180~181 — 적팀 챔피언 5칸 중 '최근 목격된' 것만 남기고, 캠프에서 가장 가까운 하나
 let nearest = data.cache.player_champion[1 - team] // AGWC+0x1e0, [5]칸
 .iter()
 .filter_map(|c| *c) // iter_champions: None 칸 스킵
 .filter(|e| data.blackboard[1 - team].is_recent_visible(game, player, e))
 .min_by_key(|e| distance_sq(e.x, e.y, cx, cy));
 // ↑ filter 술어와 key 계산의 실제 본체는 담당 범위 밖:
 // m10.ll 6028~6196 (모노모피된 min_by_key). 술어 = Blackboard::is_recent_visible.

 match nearest {
 None => return true, // L183 — ★**구조적 도달 불가(죽은 경로)**. 판정반전 R2: 여기 오려면 v24==false 여야 하고 그건 `v23 != 0` 가지뿐인데, v23 의 슬롯 술어(is_some ∧ HP%>=40 ∧ 캠프거리<=180000 ∧ is_recent_visible)가 L180 필터(is_some ∧ is_recent_visible)의 **진부분집합**이라 v23!=0 이면 min_by_key 에 원소가 반드시 있다 ⟹ nearest==None 이 성립 불가
 Some(e) => { // L184
 let d2 = distance_sq(e.x, e.y, cx, cy); // abs_diff 제곱합 (utils.rs:6)
 if d2 > 22500000000 /* 150000^2 */ { return true } // 적이 캠프에서 멀다 → 종료
 }
 }
 }
 // is_visible_cell == false 이거나 적이 충분히 가까우면 아래로 흐른다
}
// setup_like == false 도 아래로 흐른다

// ── L193 : 에픽 몬스터가 지금 살아 있나 ────────────────────────────
let moba = game.get_game_mode().as_moba().unwrap(); // vtable +0x40, GameMode 태그 0 = Moba
 // 태그 != 0 이면 Option::unwrap 패닉(game.rs:231 / option.rs:1013)
let epic = moba.jungle_runner.epic.live_list.get(0) // MobaMode+0x198 (len==0 → None)
 .and_then(|id| game.get_entity_by_id(*id)); // vtable +0x1f0
if epic.is_some() { return false } // 살아 있으면 계속 헌트

// ── L194 : 죽어 있으면, 리젠까지 얼마나 남았나 ─────────────────────
let moba = game.get_game_mode().as_moba().unwrap(); // ★IR 상 get_game_mode 를 실제로 두 번 호출한다
return moba.jungle_runner.epic.next_respawn_tick // MobaMode+0x1b0
 .saturating_sub(game.tick()) // vtable +0x28
 > 15 * data.context.setting.tick_per_second; // GameSetting+0x12f8
// → 리젠까지 15초 초과로 남았으면 종료, 15초 이내면 계속 대기(포킹 유지)

// 요약 — true(종료) 가 되는 경로는 5가지:
// (a) 팀 주목표가 Morgard 가 아님 [L164]
// (b) setup 단계 + 규율계층이 passive 로 풀라고 함 [L174]
// (c) setup + 캠프 셀 시야 O + 보이는 적 챔프 0명 [L183] ★도달 불가( R2 · 범위 = objective==Some(Morgard{Setup}) 경로. Serpen 판 serpen/hunt_and_poke.rs:162 는 미확인)
// (d) setup + 캠프 셀 시야 O + 최근접 적이 150000 밖 [L184]
// (e) 에픽 죽어 있고 리젠까지 15초 초과 남음 [L194]
// false(계속) 가 되는 경로는 2가지:
// (f) 에픽이 살아 있음 [L194]
// (g) 에픽 죽었지만 리젠 15초 이내 [L194]
```

**`mem` 메모리 접근 21건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev |
|---|---|---|---|---|---|---|
| 0 | TeamPlan | 0x41f | objective | r | Option<MainObjective>(3B)의 태그 바이트. !range !19221 = {i8 -1, i8 12} → 유효값 {255}∪{0..11}, 255=None, 0=Morgard. 두 번 로드(L164 게이트, L172 setup_like) · tcx 정본 대조( tcxdict/tcxaudit **tcx 정본** 대조 OK(오귀속 0·밀림 0) — 오프셋·필드명 한정) | 3 |
| 1 | TeamPlan | 0x420 | objective.Morgard.phase | r | MainObjective enum+0x1 = ObjectPhase(1B). dienum MainObjective 0 으로 확인. 태그==0 문맥에서만 읽는다 | 3 |
| 2 | PlayerState | 0x930 | info.team | r | usize. camp_pos 의 blue-side 인자 / is_visible_cell 인자 / 1-team(적팀 인덱스) 산출에 사용 · tcx 정본 대조( tcxdict **tcx 정본** 대조 OK + ⓐ `offset_of!` 실행 교차검증 MISMATCH 0 (B6_o1.tsv, 78행+구조체크기 7건) — 오프셋·필드명 한정) | 3 |
| 3 | OperationData | 0x0 | cache | r | &AbstractGameWithCache. gep 없이 plain load 라 C3 에는 안 잡힐 수 있음 · tcx 정본 대조( tcxdict **tcx 정본** 대조 OK + ⓐ `offset_of!` 실행 교차검증 MISMATCH 0 (B6_o1.tsv, 78행+구조체크기 7건) — 오프셋·필드명 한정) | 3 |
| 4 | OperationData | 0x8 | context | r | &GameContext · tcx 정본 대조( tcxdict **tcx 정본** 대조 OK + ⓐ `offset_of!` 실행 교차검증 MISMATCH 0 (B6_o1.tsv, 78행+구조체크기 7건) — 오프셋·필드명 한정) | 3 |
| 5 | OperationData | 0x10 | blackboard | r | &[Blackboard; 2] — 필터 클로저 환경으로 전달(본문에선 로드만) · tcx 정본 대조( tcxdict **tcx 정본** 대조 OK + ⓐ `offset_of!` 실행 교차검증 MISMATCH 0 (B6_o1.tsv, 78행+구조체크기 7건) — 오프셋·필드명 한정) | 3 |
| 6 | GameContext | 0x8 | setting | r | &GameSetting · tcx 정본 대조( tcxdict **tcx 정본** 대조 OK + ⓐ `offset_of!` 실행 교차검증 MISMATCH 0 (B6_o1.tsv, 78행+구조체크기 7건) — 오프셋·필드명 한정) | 3 |
| 7 | GameContext | 0x20 | map | r | &MapDef(28112B) — dereferenceable(28112) 로 크기 교차검증됨 · tcx 정본 대조( tcxdict **tcx 정본** 대조 OK + ⓐ `offset_of!` 실행 교차검증 MISMATCH 0 (B6_o1.tsv, 78행+구조체크기 7건) — 오프셋·필드명 한정) | 3 |
| 8 | GameSetting | 0x12f8 | tick_per_second | r | usize. 15배 해서 리스폰 대기 임계로 사용 · tcx 정본 대조( tcxdict **tcx 정본** 대조 OK + ⓐ `offset_of!` 실행 교차검증 MISMATCH 0 (B6_o1.tsv, 78행+구조체크기 7건) — 오프셋·필드명 한정) | 3 |
| 9 | AbstractGameWithCache | 0x0 | game.data_ptr | r | &dyn AbstractGame 팻포인터의 데이터 절반 · tcx 정본 대조( tcxdict **tcx 정본** 대조 OK + ⓐ `offset_of!` 실행 교차검증 MISMATCH 0 (B6_o1.tsv, 78행+구조체크기 7건) — 오프셋·필드명 한정) | 3 |
| 10 | AbstractGameWithCache | 0x8 | game.vtable | r | &dyn AbstractGame 팻포인터의 vtable 절반(816B — divtable 이 보고한 vtable 총 크기와 일치) | 3 |
| 11 | AbstractGameWithCache | 0x1e0 | player_champion | r | [[Option<&Entity>;5];2] (80B). gep [5 x ptr] 를 인덱스 (1-team) 으로 — 즉 적팀 챔피언 5칸 슬라이스. iter_champions(simulation.rs:1904(=선언줄)) 인라인 ⚠1차의 「인라인 루트가 1905 이므로 1904 는 1줄 차」는 **오독**이다 : tcx 실측 `iter_champions` 선언 = 1904:3-1904:85, 그 안의 `{closure#0}`(filter_map 술어) = 1905:50-1905:53. 루트가 1905 인 것은 클로저 줄이기 때문이고 **1904 가 맞다**. 같은 함정 2건: `Position::as_index` 580/581 · `Entity::distance_sq` 2157/2158 | 3 |
| 12 | AbstractGame::vtable | 0x28 | tick | r | divtable AbstractGame 0x28 (일치율 98%). 현재 틱 반환 | 3 |
| 13 | AbstractGame::vtable | 0x40 | get_game_mode | r | divtable AbstractGame 0x40. 반환 {i64 tag, ptr payload} = GameMode(128b). tag 0 = Moba | 3 |
| 14 | AbstractGame::vtable | 0x100 | is_visible_cell | r | divtable AbstractGame 0x100. (team, cell_x, cell_y) -> bool | 3 |
| 15 | AbstractGame::vtable | 0x1f0 | get_entity_by_id | r | divtable AbstractGame 0x1f0. (usize id) -> Option<&Entity>(널=None) | 3 |
| 16 | MobaMode | 0x1a0 | jungle_runner.epic.live_list.ptr | r | MobaMode+0x198 = jungle_runner.epic.live_list: Vec<usize>. 이 판의 Vec 배치는 {?@+0x0, ptr@+0x8, len@+0x10} · tcx 정본 대조( tcxdict/tcxaudit **tcx 정본** 대조 OK(오귀속 0·밀림 0) — 오프셋·필드명 한정) | 3 |
| 17 | MobaMode | 0x1a8 | jungle_runner.epic.live_list.len | r | ==0 이면 `get(0)` = None 으로 접힘 (소스는 `get(0)` 이다 — 인라인 체인이 `slice/mod.rs:572 get<usize,usize>` 이고 `fn=first` 프레임은 없다. `first()` 는 `get` 을 부르지 않고 슬라이스 패턴으로 구현돼 있어 체인에 나타나지 않는다) · tcx 정본 대조( tcxdict/tcxaudit **tcx 정본** 대조 OK(오귀속 0·밀림 0) — 오프셋·필드명 한정) | 3 |
| 18 | MobaMode | 0x1b0 | jungle_runner.epic.next_respawn_tick | r | usize. JungleCampState(48B) 의 +0x18. distruct 로 0x198/0x1b0 둘 다 교차검증 | 3 |
| 19 | Entity | 0x660 | x | r | u64 월드 좌표 · tcx 정본 대조( tcxdict **tcx 정본** 대조 OK + ⓐ `offset_of!` 실행 교차검증 MISMATCH 0 (B6_o1.tsv, 78행+구조체크기 7건) — 오프셋·필드명 한정) | 3 |
| 20 | Entity | 0x668 | y | r | u64 월드 좌표 · tcx 정본 대조( tcxdict **tcx 정본** 대조 OK + ⓐ `offset_of!` 실행 교차검증 MISMATCH 0 (B6_o1.tsv, 78행+구조체크기 7건) — 오프셋·필드명 한정) | 3 |

**`consts` 상수 6건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 4 | 164 | 태그 | JungleType::Morgard 의 **태그값 4** (dienum JungleType 4). ⚠`src_line=164` 는 맞지만 **IR 에 리터럴이 없다** — L164 `take_active(Morgard)` 와 L172 `take_setup_like(Morgard)` 는 `icmp eq i8 (TeamPlan+0x41f), 0` 하나로 완전히 접혔고, `i8 4` 가 살아남는 곳은 **L169 `camp_pos`(m10.ll:7691) · L174 `v24_…`(m10.ll:7709) 둘뿐**이다. take_active/take_setup_like/camp_pos/v24_..._release_to_passive 4곳 전부에 이 캠프 상수가 박혀 있다 = 이 플랜은 에픽 전용 | 3 |
| 1 | 0 | 164 | 태그 | MainObjective::Morgard 태그값. TeamPlan+0x41f 를 이 값과 비교하는 것이 take_active(Morgard) 의 접힌 실체(MainObjective->Option<JungleType> 매핑은 인라인 소멸). 태그값 0 의 출처는 **dienum MainObjective**(3차 §7 ev 상향 ⑦ — 이 행만 근거 이름이 빠져 ev4 로 앉아 있었다. 같은 사실을 18 writes 는 'dienum MainObjective 7 = Repair' 로 인용하고 있다) | 3 |
| 2 | 1 | 172 | 태그 | ObjectPhase::Setup 태그값(dienum ObjectPhase 1). TeamPlan+0x420 == 1 이 setup_like | 3 |
| 3 | 32000 | 179 | 계수 | 셀 크기 — 월드좌표를 셀좌표로 나누는 **좌표변환 계수**다(임계가 아니다 — IR 관측도 `udiv` 2회뿐). is_visible_cell 인자용 · 오라클 실행 확증( 오라클 실행 확증: 캠프 288000 → 셀 (9,9), is_visible_cell(0,9,9)=true 로 L179 진입 ) | 2 |
| 4 | 22500000000 | 184 | 임계 | 150000^2 — 캠프↔가장가까운 적챔프 거리 제곱 임계. 이보다 멀면 '적이 캠프 근처에 없다'고 보고 종료 · 오라클 실행 확증( 오라클 실행 확증: 캠프 거리 150000 → 폴스루 / 150001 → (d) 종료 ) | 2 |
| 5 | 15 | 194 | 계수 | tick_per_second 에 곱해지는 초 단위 계수 = 15초. 에픽 리스폰까지 15초 넘게 남았으면 종료. ★**오라클 13/13 **: `15 × tick_per_second` 임계가 **정확히 900** 에서 갈린다(nrt 900 → false, 901 → true). 같은 실행에서 L164 objective 게이트 · L193 에픽 생존 · phase 판정 · version 무영향(9종)까지 동시 확증. ⚠setup 경로는 `v24_…=true` 가 먼저 발화해 (c)(d) 는 오라클 미도달(범위 명시) | 2 |

**`knobs` 조정점 9건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev |
|---|---|---|---|---|---|
| 0 | 적 근접 판정 거리 임계(제곱) | hunt_and_poke.rs:184 (m10.ll:7812 `icmp ugt i64 %85, 22500000000`) | 22500000000 | 올리면 캠프에서 더 멀리 있는 적까지 '아직 위협/포킹 대상'으로 보고 플랜을 유지한다(에픽 근처에 더 오래 눌러앉음). 내리면 적이 조금만 물러나도 setup 단계에서 플랜을 접는다. 값은 거리의 제곱이므로 실제 사거리는 sqrt — 150000(=4.6875셀) ⚠**근거 정정 **: 이 노브는 **(d) 경로 전용**이고 3차가 「(c)(d) 오라클 미도달」로 명시했으므로 실행 근거가 없었다. 5차가 **(d) 경로에 최초 도달**해 경계를 **정확히 150000/150001** 로 격리했다(제곱값 22500000000). ⚠기존 `note` 에 **15×tps 노브의 설명이 오귀속**돼 있던 것도 함께 제거했다. | 2 |
| 1 | 에픽 리젠 대기 허용 시간(초) | hunt_and_poke.rs:194 (m10.ll:7888 `mul i64 %121, 15`) | 15 | 올리면 에픽이 죽어 있어도 더 일찍부터(더 오래) 캠프 앞에 모여 대기한다. 내리면 리젠 직전에야 모인다. tick_per_second 를 곱하므로 단위는 초 ★**실행 확증 **: v23 반경 180000 도 **180000/180001** 로 격리. 같은 사실을 `constants` 는 이미 실행 근거로 갖고 있었는데 이 행만 빠져 있었다. | 2 |
| 2 | setup 단계 게이트 | hunt_and_poke.rs:172~173 (TeamPlan+0x420 == ObjectPhase::Setup(1)) | 1 | 이 값을 Assemble(2)/Hunt(3) 로 바꾸면 (b)(c)(d) 세 종료 경로가 그 단계에서만 작동하게 된다. Setup 이 아닌 단계에서는 현재 적 위치·시야를 아예 보지 않고 (e)(f)(g) 만으로 판정한다는 뜻 — 즉 조립/사냥 단계에 들어가면 적이 멀어져도 이 플랜이 안 풀린다 · 오라클 실행 확증( 오라클 실행 확증: phase None/Assemble/Hunt → v24=false·is_end=false, Setup → true·true (#P 진리표) ) | 2 |
| 3 | 대상 캠프 고정값 | hunt_and_poke.rs:164/169/172/174 (`i8 noundef 4`) | 4 | JungleType::Morgard(에픽). 다른 값으로 바꾸면 같은 코드가 다른 캠프(0=Rhino,1=Mushroom,2=Stump,3=Bee,5=Serpen)를 기준으로 판정한다 — serpen 판은 별도 함수(m10.ll 9387~9873)로 이미 존재 · 오라클 실행 확증(7차 배치B: B7_o1.tsv #M 46/46 MATCH — `MainObjective` **12 variant 전수** 스윕(2·3차는 4개만 돌렸다). 태그 0(Morgard)만 본문을 계속 타고, 1~11 과 None 은 전부 즉시 true ⟹ 이 플랜이 캠프 상수 4(Morgard)에 고정돼 있음이 실행으로 확정) | 2 |
| 4 | 적 챔프 필터 술어(최근 목격) | ★담당 범위 밖 — m10.ll:6130 `Blackboard::is_recent_visible(bb[1-team], game, player, entity)` (모노모피된 min_by_key 본체 m10.ll 6028~6196) | 0 | 이 술어를 항상 true 로 만들면 시야 밖 적도 거리 계산에 들어가 (c)(d) 종료가 훨씬 덜 일어난다(=AI 가 유령 정보를 쓰게 됨). 항상 false 면 (c) 경로로 거의 즉시 종료. value 0 은 자리표시 — 이 항목은 임계값이 아니라 술어 자체다 | 4 |
| 5 | MainObjective→JungleType 표 | m13.ll:29774~29777 select 체인 + m09.ll switch 판 3곳 | 0→4, 1→5, 그 외 None | PressEpic/SplitEpic 등에 캠프를 매핑하려면 **아웃오브라인 본체가 없어 사본 단위로 전부** 고쳐야 한다 · 오라클 실행 확증(7차 배치B: B7_o1.tsv #M — 0→Morgard(계속) / 1 Serpen·2 Defense·3 DefenseLine·4 Nexus·5 PressEpic·6 SplitEpic·7 Repair·8 Gank·9 Dive·10 PressTower·11 ComebackPick → **전부 즉시 true** ⟹ 「0→4 / 1→5 / 그 외 None」 표의 「그 외 None」이 8 variant 에 대해 처음 실행 확인됐다. 덤: `camp_pos(Morgard, blue)==camp_pos(Morgard, red)==(288000,288000)` (Serpen=(672000,672000))) | 2 |
| 6 | 캠프 주변 적 탐지 반경 | objective_discipline.rs:187 | 180000 | 셋업 해제 판정 범위 · 오라클 실행 확증( 오라클 실행 확증: v23 반경 180000 → v24=false((d)) / 180001 → v24=true((b)) ) | 2 |
| 7 | 적 HP 하한 `min_hp_ratio` | objective_discipline.rs:187 | 40 | ★★이 노브는 **`min_hp_ratio = 40`(적 HP 백분율 하한)**: `hp*100/max < 40` 인 적은 **건너뛴다**. 근거 3중 = DWARF `!61645 name:"min_hp_ratio" arg:6` + IR `hp*100/max < %5 → skip` + **오라클 39/40 경계 반전**. 올리면 빈사 적을 무시해 셋업 해제가 둔해지고, 내리면 거의 모든 적을 세어 민감해진다. | 2 |
| 8 | 압박 라인 세트 | objective_discipline.rs:200 | Morgard [Top,Mid] / Serpen [Bottom,Mid] | 오브젝트별 압박 대상 라인 | 4 |

<details><summary>`callees` 피호출자 24건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 |
|---|---|---|---|---|---|
| 0 | as_moba | game_core::GameMode::<'a>::as_moba | pub | fn(game_core::GameMode<'a/#0>) -> std::option::Option<&'a/#0 game_core::MobaMode> | game-core\src\simulation\game.rs:230 |
| 1 | camp_pos | game_core::JungleType::camp_pos | pub | fn(&game_core::JungleType, bool) -> (u64, u64) | game-core\src\simulation\entity\jungle.rs:427 |
| 2 | camp_pos | game_core::MapDef::camp_pos | pub | fn(&game_core::MapDef, game_core::JungleType, bool) -> (u64, u64) | game-core\src\simulation\map_def.rs:207 |
| 3 | distance_sq | game_core::Entity::distance_sq | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2157 |
| 4 | distance_sq | game_core::utils::distance_sq | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:6 |
| 5 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 |
| 6 | get_entity_by_id | <game_core::ExpectedGame<'a> as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::ExpectedGame<'a/#0>, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\expected_game.rs:165 |
| 7 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 |
| 8 | get_game_mode | <game_core::Game as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::Game) -> game_core::GameMode | game-core\src\simulation\game.rs:1707 |
| 9 | get_game_mode | <game_core::SingleLaneGame as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::SingleLaneGame) -> game_core::GameMode | game-core\src\simulation\game.rs:4248 |
| 10 | get_game_mode | <game_core::DeathMatchGame as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::DeathMatchGame) -> game_core::GameMode | game-core\src\simulation\game.rs:5052 |
| 11 | is_end | game_ai::SmallActionPlay::is_end | pub | fn(&game_ai::SmallActionPlay, &mut rand::rngs::std::StdRng, usize, &game_core::PlayerState, &game_core::OperationData) -> bool | game-ai\src\small_action.rs:348 |
| 12 | is_end | game_ai::SmallActionRunAway::is_end | in:game_ai | fn(&game_ai::SmallActionRunAway, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bool | game-ai\src\small_action\move_actions.rs:633 |
| 13 | is_end | game_ai::SmallActionRecall::is_end | in:game_ai | fn(&game_ai::SmallActionRecall, &mut rand::rngs::std::StdRng, usize, &game_core::PlayerState, &game_core::OperationData) -> bool | game-ai\src\small_action\move_actions.rs:1032 |
| 14 | is_recent_visible | game_core::Blackboard::is_recent_visible | pub | fn(&game_core::Blackboard, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\game\blackboard.rs:346 |
| 15 | is_visible_cell | <game_core::Game as game_core::AbstractGame>::is_visible_cell | pub | fn(&game_core::Game, usize, usize, usize) -> bool | game-core\src\simulation\game.rs:1804 |
| 16 | is_visible_cell | <game_core::ExpectedGame<'a> as game_core::AbstractGame>::is_visible_cell | pub | fn(&game_core::ExpectedGame<'a/#0>, usize, usize, usize) -> bool | game-core\src\simulation\expected_game.rs:207 |
| 17 | is_visible_cell | game_core::AbstractGame::is_visible_cell | pub | fn(&Self/#0, usize, usize, usize) -> bool | game-core\src\simulation.rs:126 |
| 18 | take_active | game_ai::plan_legacy::team_plan::TeamPlan::take_active | pub | fn(&game_ai::plan_legacy::team_plan::TeamPlan, game_core::JungleType) -> bool | game-ai\src\plan_legacy\team_plan.rs:243 |
| 19 | take_setup_like | game_ai::plan_legacy::team_plan::TeamPlan::take_setup_like | pub | fn(&game_ai::plan_legacy::team_plan::TeamPlan, game_core::JungleType) -> bool | game-ai\src\plan_legacy\team_plan.rs:257 |
| 20 | tick | <game_core::Game as game_core::AbstractGame>::tick | pub | fn(&game_core::Game) -> usize | game-core\src\simulation\game.rs:1796 |
| 21 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 |
| 22 | tick | <game_core::ExpectedGame<'a> as game_core::AbstractGame>::tick | pub | fn(&game_core::ExpectedGame<'a/#0>) -> usize | game-core\src\simulation\expected_game.rs:53 |
| 23 | v24_objective_setup_should_release_to_passive | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v24_objective_setup_should_release_to_passive | pub | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::PlayerState, &game_core::OperationData, game_core::JungleType) -> bool | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:177 |
</details>

⚠**미매칭 1개**: `llvm.usub.sat.i64`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m02.ll:6740) · **형제 9개** (EpicHuntAndPokePlan)

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

**`open` 0건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**

(없음 — 이 함수는 `open` 이 비었다. 그래도 **게이트 미해소(§4)와 반증은 유효하다**.)

<details><summary>`closed` 10건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

| # | 물음 | 답 | ev |
|---|---|---|---|
| 0 | self(&EpicHuntAndPokePlan) 의 focus_epic_only / vision_only / v46_flee / v46_flee_threats 가 이 함수에서 전혀 안 읽힌다(파라미터에 readonly captures(none), 본문에 %0 대상 gep 0개). 왜 받는지는 sub_plan(m10.ll 7902~9384) 쪽을 봐야 하며 이번 범위에서는 확인 불가 |  |  |
| 1 | version(p2) 이 v24_objective_setup_should_release_to_passive 로만 전달되고 이 함수 안에서는 어떤 분기도 만들지 않는다 — 그 피호출 내부는 안 봤으므로 버전별 차이는 미확정 |  |  |
| 2 | Blackboard::is_recent_visible 의 판정 조건(무엇을 '최근'으로 보는가, 몇 틱인가) 미독해. game_core 쪽 함수라 이번 범위 밖이며, 호출 자체도 담당 줄범위 밖(m10.ll:6130)이라 constants 에 넣지 않고 knobs 에만 실었다 |  |  |
| 3 | blackboard 인덱스 `1 - player.info.team` 의 의미론 미확정 — 관측 사실은 'iter_champions 와 같은 인덱스(적팀 번호)로 blackboard 배열을 인덱싱한다'뿐이다. 그 배열이 '관측 대상 팀별 지식'인지 '소유 팀별 지식'인지는 Blackboard 정의를 봐야 하는데 이번 범위 밖 |  |  |
| 4 | TeamPlan::take_active / objective_target 의 MainObjective -> Option<JungleType> 매핑표 전체가 인라인으로 소멸했다. 관측된 것은 'Morgard 경로 == 태그 0' 하나뿐이고, Serpen 등 다른 variant 가 어떤 JungleType 으로 가는지는 이 함수에서 복원 불가 |  |  |
| 5 | Option<MainObjective> 의 None 센티널 값 255(0xff) 는 !range !19221 = {i8 -1, i8 12} 메타데이터에서만 관측된다. 본문에 리터럴이 없어(단일 `icmp eq 0` 로 접힘) constants 에는 넣지 않았다 |  |  |
| 6 | get_game_mode / tick / is_visible_cell / get_entity_by_id 이름은 divtable 이 찾은 정적 @vtable 전역(ExpectedGame 구현, 일치율 98%) 기준이다. 런타임에 실제로 꽂히는 구현체가 ExpectedGame 인지 다른 것인지는 이 도구로 확인 불가 — 슬롯 번호↔메서드 대응만 신뢰한다 |  |  |
| 7 | get_game_mode().as_moba().unwrap() 이 L193 과 L194 에서 각각 한 번씩, 총 2회 호출된다(%40, %106). 두 반환이 같다는 보장은 IR 어디에도 없다 — 같은 프레임 안이라 동일할 것으로 추정하지만 확정하지 않았다 |  |  |
| 8 | MobaMode 의 Vec<usize> 배치를 {+0x0 ?, +0x8 ptr, +0x10 len} 으로 읽었다. len 이 +0x10(=MobaMode+0x1a8) 이라는 것은 'len==0 검사 후 [0] 인덱싱'이라는 사용 패턴(slice/index.rs:218 get<usize>, slice/mod.rs:576)으로 역추론한 것이지 DWARF 멤버 오프셋으로 직접 확인한 것은 아니다. +0x0 에 오는 것(cap 추정)은 이 함수가 읽지 않아 미확정 |  |  |
| 9 | v24_objective_setup_should_release_to_passive 내부 미독해 — (b) 종료 경로의 실제 조건은 이 명세로는 알 수 없다 |  |  |
</details>

<details><summary>`history` 정정 이력 10건 (참조용 — 본문 아님)</summary>

| # | 옛 값 | 현재 | 근거 |
|---|---|---|---|
| 0 | min_by_key 본체의 필터/키 술어 | ★필터는 is_recent_visible 단 하나뿐이다(hunt_and_poke.rs:180). 거리 임계·is_ignored_well_enemy 등 추가 술어 없음(상수 스윕 결과 판정 임계 0건). 키 = \|c.x-camp_pos.0\|² + \|c.y-camp_pos.1\|² (hunt_and_poke.rs:181). ⟹ '캠프에서 가장 가까운, 최근 목격된 챔피언'. |  |
| 1 | blackboard 인덱스 의미 | _shared.is_recent_visible.blackboard_인덱스_의미 참조 — 확정됨. |  |
| 2 | TeamPlan::take_active / objective_target 의 MainObjective → Option<JungleType> 매핑표(인라인 소멸) | ★확정 — 접히지 않은 인라인 사본에서 복원. **0 Morgard→JungleType 4 / 1 Serpen→5 / 2~11 전부 None.** 전문 = `_shared.objective_매핑표`. ⚠`take_active` 는 `readonly` — 이름과 달리 상태를 비우지 않는다. |  |
| 3 | v24_objective_setup_should_release_to_passive 내부 미독해 | ★확정(objective_discipline.rs:180~197, _gaibc/m09.ll:22104~22175).   `match target { Morgard(4) => objective == Some(Morgard{phase:Setup}), Serpen(5) => Some(Serpen{phase:Setup}), _ => return false }`   `camp = MapDef::camp_pos(map, target, player.team == 0)`   `if v23_recent_visible_enemies_near_point(player, data, camp.x, camp.y, radius=180000, recent=40틱) != 0 { return false }`   `if AbstractGame::is_visible_cell(team, camp.x/32000, camp.y/32000)(vt+0x100) { return true }`   `return v23_objective_setup_pressure_line(player, data, lanes, 2) != -1`  (lanes: Morgard → [Top, Mid] / Serpen → [Bottom, Mid]) |  |
| 4 | `live_list.first()` 로 기재 | ★**정정(2026-09-11 검증배치 B)**: 실제는 **`live_list.get(0)`** 이다. 인라인 체인 `!19322` 실측 = `index.rs:219 get<usize>` → `mod.rs:576 get<usize,usize>`(= `<[T]>::get`, `slice\mod.rs:572` 선언) → `hunt_and_poke.rs:193`. `slice::first()` 는 `get` 을 부르지 않고 슬라이스 패턴으로 구현돼 있어 소스가 `first()` 였다면 `fn=first` 프레임이 나와야 하는데 **없다**. ⟹ **두 명세(07/08)가 서로 어긋나 있었고 틀린 쪽이 08.** ★방법론: 인라인 체인의 **`DISubprogram(name:)`** 을 보면 `파일:줄` 만으로는 못 가르는 것이 이름으로 즉시 갈린다. |  |
| 5 | L164 objective 게이트 / L172 setup_like 가 실제로 그렇게 갈리는가 | ★실행 검증(2026-09-11 2차배치B, `B_o3.rs`). TeamPlan.objective 를 바꿔 `is_end(3,…)` 실측: None(default) → **true**(L164) / Morgard{None\|Assemble\|Hunt, wb=any} → **false** / Morgard{**Setup**, wb=any} → **true**(setup 전용 종료경로 진입) / Serpen{4 phase 전부}·Defense·Nexus(Mid) → **true**(태그≠0 즉시 종료) ⟹ ①`TeamPlan+0x41f==0`(Morgard) 만 통과 ②`+0x420==Setup` 만 추가 종료경로를 켠다 ③**`with_battle` 은 이 함수 판정에 무영향** — 전부 실행 확인. `ObjectPhase` 태그 tcxdict = None 0 / Setup 1 / Assemble 2 / Hunt 3. |  |
| 6 | 08 `15 × tick_per_second` 임계 — 오라클로 못 켰다(입력 판별력 부재) | ★**열렸다. 임계가 정확히 900**(3차 배치B, 오라클 13/13, ev2): `MobaMode.next_respawn_tick` 을 직접 써 넣으면 된다(`Game::mode` 가 pub) — nrt 900→false, 901→true. L164 objective 게이트·L193 에픽 생존·phase 판정·version 무영향(9종)도 동시 확증. setup 경로는 `v24_...=true` 가 먼저 발화해 (c)(d) 는 오라클 미도달(미탐색). |  |
| 7 | `AbstractGame` vtable 슬롯 번호의 근거 | ★**공식 확정**(3차 배치B, ev3): `슬롯 = 0x20 + 8 × (트레이트 선언 순서)`, `0x18` = `Debug::fmt`. **구현체와 무관**하며 `divtable` 표와 6/6 일치 ⟹ vtable 행 전부 ev 4→3. 덤: `<Game as AbstractGame>::get_game_mode` MIR = `GameMode::Moba(&self.mode)` **순수** ⟹ `as_moba()` 2회 호출 동일성 확정(이 플랜이 Moba 전용임도 증명). |  |
| 8 | `objective_discipline.rs:187` 의 40 = **"최근 가시" 창(40틱)** — 「★확정」으로 표기돼 있었다 | ★**거짓이었다**(5차 배치B, 판정반전 R1). 실제 = **`min_hp_ratio = 40`**(적 HP% 하한). DWARF 인자명 + IR 비교식 + 오라클 39/40 반전으로 확정. ⟹ 「★확정」 표기가 붙어 있어도 **근거가 인자명·실행이 아니면 믿지 마라.** |  |
| 9 | `08` 경로 (c) `L183` = 「오라클 미도달 = 미탐색」 | ★**판정반전 R2 — 구조적 도달 불가**(5차 배치B). v24 exit phi + v23 술어가 `is_end` 필터의 **진부분집합**임을 증명했고, 3종 구성 시도가 전부 (b)로 빠졌다. ⟹ 「아직 못 갔다」가 아니라 **갈 수 없다**. 재구현에서 이 가지는 죽은 코드로 취급해도 된다. |  |
</details>

