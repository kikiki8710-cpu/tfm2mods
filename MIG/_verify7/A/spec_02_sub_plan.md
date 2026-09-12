---

### `02` sub_plan — 넥서스 공격 플랜의 서브플랜 선택 — 샘에서 회복중이면 Recall, 적 쌍둥이탑이 남아있으면 LineDefense, 다 밀었으면 AttackNexus

| 항목 | 값 |
|---|---|
| id | `attack_nexus__sub_plan` |
| 심볼 | `_RNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old12attack_nexusNtB2_15AttackNexusPlan8sub_plan` |
| 소스 | `game-ai\src\plan_legacy\old\attack_nexus.rs:31` |
| IR | `m12.ll` 34867~34976행 |
| 경로·가시성 | `game_ai::plan_legacy::old::AttackNexusPlan::sub_plan` · **pub** |
| 계층 | 레거시 플랜 |
| exe | `caf9f0` (attack_nexus) · 1049바이트 · 228명령 |
| 라운드 | 기준 `r4` · 통과 4회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_ai::plan_legacy::old::AttackNexusPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> game_ai::plan_legacy::sub_plan::SubPlan
```

<details><summary>인자 7개</summary>

| # | i | 이름 | 타입 | 역할 |
|---|---|---|---|---|
| 0 | 1 | self |  | 필드 team:usize@0x0 · line:LineType@0x8. 본문은 line 만 읽는다(team 은 안 씀 — team 은 player.info.team 에서 가져옴) |
| 1 | 2 | version |  | AI 버전 게이트. ★이 함수에선 한 번도 참조되지 않음(분기 없음) |
| 2 | 3 | rnd |  | readnone — 난수 미사용 |
| 3 | 4 | player |  | info.team(0x930) · info.position(0x9c0) 만 읽음 |
| 4 | 5 | data |  | cache(0x0) · context(0x8) 사용. blackboard(0x10) 미사용 |
| 5 | 6 | _team_plan |  | readnone — 미사용(이름의 _ 접두와 일치) |
| 6 | 7 | _debug |  | readnone — 미사용 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn AttackNexusPlan::sub_plan(&self, version, rnd, player, data, _team_plan, _debug) -> SubPlan

// ── L36 : 내 챔피언 얻기 ──────────────────────────────
team = player.info.team // PlayerState+0x930, team>=2 면 패닉(len=2)
pos = player.info.position.as_index() // PlayerState+0x9c0 (i32 range[0,5)) 을 zext, entity.rs:580 인라인
champ = data.cache.player_champion[team][pos].unwrap()
 // AbstractGameWithCache+0x1e0 + 40*team + 8*pos, null 이면 option::unwrap_failed

// ── L37 : 우리 팀 샘(fountain) 사각형 ────────────────
(lx, ly, rx, ry) = data.context.map.fountain(team)
 // MapDef+0x6d70 + 32*team 의 (u64,u64,u64,u64), map_def.rs:234~235 인라인

// ── L38 : 샘 안에 있나 ───────────────────────────────
in_x = (champ.x >= lx) && (champ.x <= rx) // Entity+0x660
if !in_x { goto NOT_IN_AREA } // 블록24 -> 55
in_y = !((champ.y < ly) || (champ.y > ry)) // Entity+0x668
if !in_y { goto NOT_IN_AREA } // 블록39 -> 55
is_in_heal_area = in_x && in_y // = true 인 경로만 아래로

// ── L41 : 샘 안 + 체력 미만 → 계속 회복 ──────────────
if champ.hp < champ.stat_cached.hp { // Entity+0x670 < Entity+0x628
 return SubPlan::Recall // tag 5
}
// (HP 가 꽉 찼으면 아래로 떨어진다)

NOT_IN_AREA:
// ── L45 : 적 쌍둥이탑이 남아있나 ─────────────────────
has_enemy_twin_tower = !data.cache.twin_towers[1 - team].is_empty()
 // AbstractGameWithCache+0x130 + 32*(1-team), Vec.len 은 +0x18 (=본문 오프셋 328/360)
 // bumpalo::collections::vec::Vec::is_empty(vec.rs:1635) -> len(vec.rs:1616) 인라인

// ── L47/48 : 최종 분기 ───────────────────────────────
// ★L47~51 = **return 없는 꼬리표현식 + else**, 조건은 `if has_enemy_twin_tower` **긍정형**이다
// (근거 = 줄길이 ±0 4줄 + IR m12.ll:34955~34971 분기방향)
if has_enemy_twin_tower { // L47
 SubPlan::LineDefense(LineDefenseSubPlan { // L48
 style: LineStyle::Aggressive, // 0
 line: self.line, // AttackNexusPlan+0x8 (LineType)
 minion_action_type: MinionActionType::Push, // 2
 }) // tag 2
} else { // L49
 SubPlan::AttackNexus // L50, tag 16 — 적 쌍둥이탑 전멸 = 넥서스 직행
} // L51

// ★주의: Recall 은 '샘으로 귀환하라'가 아니라 '이미 샘 안에 서 있고 HP 가 안 찼으니
// 그대로 회복을 유지하라' 다. 샘 밖에서는 HP 가 아무리 낮아도 이 함수는 Recall 을 내지 않는다.
// ★version / rnd / _team_plan / _debug 는 본문에서 전혀 쓰이지 않는다(버전 게이트 없음, 난수 없음).
```

**`mem` 메모리 접근 21건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev |
|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | usize. 0 또는 1 이어야 함 — >=2 면 panic_bounds_check(len=2). attack_nexus.rs:36 · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 |
| 1 | PlayerState | 0x9c0 | info.position | r | Position(4B enum, !range [0,5) = Top/Jungle/Mid/Bottom/Support). Position::as_index(entity.rs:580) 가 인라인돼 판별자를 그대로 usize 로 zext · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 |
| 2 | OperationData | 0x0 | cache | r | &AbstractGameWithCache. gep 가 접혀(offset 0) 본문엔 리터럴 0 이 안 보임 · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 |
| 3 | OperationData | 0x8 | context | r | &GameContext · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 |
| 4 | GameContext | 0x20 | map | r | &MapDef · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 |
| 5 | AbstractGameWithCache | 0x1e0 | player_champion | r | [[Option<&Entity>;5];2] (80B). champ = player_champion[team][position], stride 40(팀) + 8(포지션). None 이면 Option::unwrap 실패로 패닉 · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 |
| 6 | AbstractGameWithCache | 0x148 | twin_towers[0].len | r | twin_towers 는 0x130 에 있는 [bumpalo::Vec<&Entity>;2] (팀당 32B). len 은 Vec 안 +0x18 → 0x130+0x18=0x148, 팀 stride 32. 본문은 twin_towers[1-team] 을 읽는다 · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0) ★BUMPVEC cap=8/len=3 로 **len@+0x18 을 처음 판별** ) | 3 |
| 7 | MapDef | 0x6d70 | fountains | r | [(u64,u64,u64,u64);2] (64B). MapDef::fountain(team)(map_def.rs:234) 인라인 — fountains[team], stride 32 · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 |
| 8 | MapDef.fountains[team] | 0x0 | lx | r | 샘(회복지역) 사각형 좌측 x · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0) MapDef.fountains[team] 튜플 오프셋 0x0/0x8/0x10/0x18 + FOUNTAIN 실값) | 3 |
| 9 | MapDef.fountains[team] | 0x8 | ly | r | 샘 사각형 상단 y · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0) MapDef.fountains[team] 튜플 오프셋 0x0/0x8/0x10/0x18 + FOUNTAIN 실값) | 3 |
| 10 | MapDef.fountains[team] | 0x10 | rx | r | 샘 사각형 우측 x · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0) MapDef.fountains[team] 튜플 오프셋 0x0/0x8/0x10/0x18 + FOUNTAIN 실값) | 3 |
| 11 | MapDef.fountains[team] | 0x18 | ry | r | 샘 사각형 하단 y · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0) MapDef.fountains[team] 튜플 오프셋 0x0/0x8/0x10/0x18 + FOUNTAIN 실값) | 3 |
| 12 | Entity | 0x660 | x | r | champ 의 x 좌표(u64) · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 |
| 13 | Entity | 0x668 | y | r | champ 의 y 좌표(u64) · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 |
| 14 | Entity | 0x670 | hp | r | 현재 HP · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 |
| 15 | Entity | 0x628 | stat_cached.hp | r | 최대 HP (stat_cached 는 0x618, EntityStat 안 +0x10) · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0)) | 3 |
| 16 | AttackNexusPlan | 0x8 | line | r | LineType(Top=0/Mid=1/Bottom=2). LineDefense 반환 시 그대로 실려나감 · tcx 정본 대조( A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0) ★ANPLAN new(1,Mid)/new(0,Bottom) 바이트 = [team@+0x0, line@+0x8] (5차 미실측분)) | 3 |
| 17 | SubPlan(sret 반환버퍼 %0) | 0x0 | 판별자(tag) | w | phi 로 모임: 5=Recall(블록49), 2=LineDefense(블록61), 16=AttackNexus(블록55) · tcx 정본 대조( A6_o4.tsv SUBPLAN tag/b8/b9/b10 3분기 전수) | 3 |
| 18 | SubPlan(sret) LineDefense 페이로드 | 0x8 | LineDefenseSubPlan.style | w | LineDefenseSubPlan 은 SubPlan 페이로드 offset 64bit(=바이트8)부터. 필드 비트오프셋 style=0, line=8, minion_action_type=16 · tcx 정본 대조( A6_o4.tsv SUBPLAN tag/b8/b9/b10 3분기 전수) | 3 |
| 19 | SubPlan(sret) LineDefense 페이로드 | 0x9 | LineDefenseSubPlan.line | w | AttackNexusPlan+0x8 에서 읽은 LineType · tcx 정본 대조( A6_o4.tsv SUBPLAN tag/b8/b9/b10 3분기 전수) | 3 |
| 20 | SubPlan(sret) LineDefense 페이로드 | 0xa | LineDefenseSubPlan.minion_action_type | w | Pull=0 / Normal=1 / Push=2 · tcx 정본 대조( A6_o4.tsv SUBPLAN tag/b8/b9/b10 3분기 전수) | 3 |

**`consts` 상수 6건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 2 | 36 | 임계 | 팀 배열 길이 — player.info.team 이 2 이상이면 panic_bounds_check(len=2). 즉 팀은 0/1 뿐 | 4 |
| 1 | 5 | 42 | 태그 | SubPlan::Recall 태그값 (dienum SubPlan: DISCR 5 = Recall). 샘 안 + HP 미만일 때 반환 · 오라클 실행 확증( A6_o4.tsv — in_heal_low → tag=5 Recall) | 2 |
| 2 | 1 | 45 | 인덱스 | 적 팀 인덱스 계산 `1 - team` 의 1 (sub nuw nsw i64 1, %10) · 오라클 실행 확증( A6_o4.tsv — no_twin 케이스에서 적팀(1-team) 트윈타워를 비우자 분기가 갈림) | 2 |
| 3 | 0 | 45 | 임계 | twin_towers[1-team].len() == 0 비교값 — bumpalo Vec::is_empty 인라인. 동시에 LineStyle::Aggressive(0) 저장값이기도 함 · 오라클 실행 확증( A6_o4.tsv — twin1_len=0 → AttackNexus / len=2 → LineDefense, b8=0(Aggressive)) | 2 |
| 4 | 16 | 50 | 태그 | SubPlan::AttackNexus 태그값 (DISCR 16). 적 쌍둥이탑이 하나도 안 남았을 때 반환 ★소스 줄 = **L50**(L46 은 빈 줄 1자다). L50 = 58자 → 내용 57 = indent 6 + `SubPlan::AttackNexus(AttackNexusSubPlan::default())`(51자) ±0 · 오라클 실행 확증( A6_o4.tsv — no_twin → tag=16 AttackNexus) | 2 |
| 5 | 2 | 48 | 태그 | SubPlan::LineDefense 태그값(DISCR 2) 및 MinionActionType::Push(2) — 같은 리터럴 2가 두 용도로 본문에 존재 · 오라클 실행 확증( A6_o4.tsv — LineDefense tag=2 · b10=2(Push) — MinionActionType Pull=0/Normal=1/Push=2 실측(A6_o3)) | 2 |

**`knobs` 조정점 5건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev |
|---|---|---|---|---|---|
| 0 | 샘 회복 유지 조건 (HP 상한) | attack_nexus.rs:41 (IR m12.ll:34946~34951, champ+0x670 < champ+0x628) | champ.hp < champ.stat_cached.hp | 'HP 최대치 미만'을 '최대치의 x%' 같은 완화 조건으로 바꾸면 샘에서 더 빨리 나가고, 반대로 두면 풀피가 될 때까지 절대 안 나온다. 현재는 1이라도 깎여 있으면 계속 샘에 머문다 · 오라클 실행 확증( A6_o4.tsv / A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0) — in_heal_low(hp 500/999) → Recall / in_heal_full(999/999) → LineDefense) | 2 |
| 1 | 넥서스 공격 개시 게이트 | attack_nexus.rs:45~47 (IR m12.ll:34955~34962, twin_towers[1-team].is_empty()) | 적 쌍둥이탑 Vec 이 비어 있을 것 | 이 조건을 무시하고 무조건 AttackNexus 를 내면 쌍둥이탑을 남긴 채 넥서스로 돌진한다(=탑 사거리에 갈려 죽음). 반대로 조건을 더 빡세게(예: 억제기까지) 하면 넥서스 러시가 늦어진다 · 오라클 실행 확증( A6_o4.tsv / A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0) — twin1_len 2 → LineDefense / 0 → AttackNexus) | 2 |
| 2 | 쌍둥이탑이 남았을 때의 대체 행동 | attack_nexus.rs:48 (IR m12.ll:34964~34971) | LineDefense{style=Aggressive(0), line=self.line, minion_action_type=Push(2)} | style 을 Defensive(1) 로 바꾸면 넥서스 플랜인데도 수비적으로 라인을 잡고, minion_action_type 을 Pull(0)/Normal(1) 로 바꾸면 미니언을 밀지 않는다. 즉 '넥서스 공격 페이즈의 라인 압박 강도' 노브 · 오라클 실행 확증( A6_o4.tsv / A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0) — LineDefense{style=Aggressive, line=Mid(=self.line), minion_action_type=Push} Debug 출력 일치) | 2 |
| 3 | 샘 사각형 범위 | MapDef.fountains[team] (MapDef+0x6d70, map_def.rs:234) — 데이터쪽 | (lx, ly, rx, ry) | 맵 데이터 값이라 이 함수에선 못 바꾸지만, 넓히면 '샘 안'으로 판정되는 구역이 커져 회복 대기 시간이 길어진다 · 오라클 실행 확증( A6_o4.tsv / A6_o1.tsv(수법 ⓐ offset_of! 일괄, 81/81 MATCH·MISMATCH 0) — FOUNTAIN team0=(0,896000,64000,960000) 실측 + 그 안/밖으로 챔프를 옮겨 분기 확인) | 2 |
| 4 | AttackNexusPlan.team 극성 | _gaibc/m13.ll:6720 (sub i64 1, player.info.team) | 1 - 내팀 = 적팀 | 반전시키면 아군 넥서스를 공격 목표로 삼는 플랜이 만들어진다(디버그용) | 4 |

<details><summary>`callees` 피호출자 9건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 |
|---|---|---|---|---|---|
| 0 | as_index | game_core::Position::as_index | pub | fn(&game_core::Position) -> usize | game-core\src\simulation\entity.rs:580 |
| 1 | as_index | game_core::BigGoal::as_index | pub | fn(&game_core::BigGoal) -> usize | game-core\src\simulation\ai_interface.rs:338 |
| 2 | fountain | game_core::MapDef::fountain | pub | fn(&game_core::MapDef, usize) -> (u64, u64, u64, u64) | game-core\src\simulation\map_def.rs:234 |
| 3 | is_empty | game_core::PublicChampionEvidence::is_empty | pub | fn(&game_core::PublicChampionEvidence) -> bool | game-core\src\banpick\public_evidence.rs:96 |
| 4 | is_empty | game_core::PatchSetting::is_empty | pub | fn(&game_core::PatchSetting) -> bool | game-core\src\patch_engine.rs:853 |
| 5 | is_empty | game_core::ModAiRegistry::is_empty | pub | fn(&game_core::ModAiRegistry) -> bool | game-core\src\mod_ai.rs:15 |
| 6 | sub_plan | game_ai::plan_legacy::old::PassiveLinePlan::sub_plan | pub | fn(&game_ai::plan_legacy::old::PassiveLinePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> game_ai::plan_legacy::sub_plan::SubPlan | game-ai\src\plan_legacy\old\passive_line.rs:848 |
| 7 | sub_plan | game_ai::plan_legacy::old::SinglePlanLine::sub_plan | pub | fn(&game_ai::plan_legacy::old::SinglePlanLine, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> game_ai::plan_legacy::sub_plan::SubPlan | game-ai\src\plan_legacy\old\single_line.rs:84 |
| 8 | sub_plan | game_ai::plan_legacy::old::SinglePlanBattle::sub_plan | pub | fn(&game_ai::plan_legacy::old::SinglePlanBattle, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> game_ai::plan_legacy::sub_plan::SubPlan | game-ai\src\plan_legacy\old\single_battle.rs:879 |
</details>

**호출처 1곳** (m02.ll:8484) · **형제 9개** (AttackNexusPlan)

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

**`open` 1건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | `version`(p2) 이 무엇을 게이트하는 값인지 — 이 함수 본문에서 단 한 번도 참조되지 않아 여기서는 확정 불가. AI 버전 분기가 없다는 사실만 확실하다 | 4 |  |

**`notes` 1건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev |
|---|---|---|
| 0 | has_enemy_twin_tower 의 dbg 표현이 이상하다: `#dbg_value(i1 %60, !62934, DIExpression(DW_OP_not, DW_OP_not, ...))` 로 NOT 이 두 번 걸려 있어 문자대로면 항등(= len==0)이 된다. 하지만 ①변수명이 has_enemy_twin_tower 이고 ②%60(len==0) 이 true 일 때 AttackNexus 로 가므로, 의미상 has_enemy_twin_tower = !is_empty() 가 맞다. 2회 확인 후 dbg 표현 쪽을 컴파일러 아티팩트로 판단하고 내려놨음 | 4 |

<details><summary>`closed` 6건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

| # | 물음 | 답 | ev |
|---|---|---|---|
| 0 | AttackNexusPlan.team(+0x0) 은 이 함수에서 안 읽는다 — team 은 player.info.team 으로 다시 구한다. 둘이 같은지는 **이 함수만으론** 확정 불가 — ★해소(6차 배치A): 답은 이미 같은 파일 `history[1]` 에 있다. **항상 반대다**(`LegacyPlanHandler::passive_plan` handler.rs:1879 = `sub i64 1, player.info.team`). ⟹ 이 항목은 `open` 이 아니라 닫힌 것이다(5차까지 `open` 에 남아 있어 매 라운드 재조사 후보로 잡혔다) |  |  |
| 1 | Position::as_index 가 인라인돼 '판별자를 그대로 usize 로' 쓰는 것까지만 확인됨(load i32 !range[0,5] → zext). 판별자↔인덱스 매핑에 재배치가 있는지는 as_index 본체(entity.rs:580)를 안 봤으므로 미확정 |  |  |
| 2 | 블록24(x 범위 밖) / 블록39(y 범위 밖) / 블록49(HP 풀) 세 경로가 모두 블록55 로 합류하므로 `is_in_heal_area` 라는 지역변수 자체는 최적화로 사라졌다. 소스에 `if is_in_heal_area && hp<max` 형태였는지 중첩 if 였는지는 IR 로는 구분 불가(결과 동작은 동일) |  |  |
| 3 | reads 의 `OperationData+0x0`(cache) 은 gep 오프셋 0 이 접혀 본문에 리터럴 0 으로는 안 보인다(QC C3 경고 대상). DWARF !14558 로 확정한 값 |  |  |
| 4 | `AbstractGameWithCache.twin_towers`(0x130) 의 원소 타입을 distruct 는 `?`(64B) 로만 돌려준다. 인라인된 bumpalo Vec::is_empty/len 의 !dbg 로 '팀당 32B bumpalo Vec, len@+0x18' 까지는 확정했으나 앞 24B(ptr/ptr/cap)의 정확한 필드명은 미확인 |  |  |
| 5 | 이 함수는 vtable 디스패치를 하나도 하지 않아 divtable 로 확인할 슬롯이 없었다(cache.game 의 dyn AbstractGame 은 여기서 안 건드림) |  |  |
</details>

<details><summary>`history` 정정 이력 6건 (참조용 — 본문 아님)</summary>

| # | 옛 값 | 현재 | 근거 |
|---|---|---|---|
| 0 | MapDef.fountains 원소가 (좌상x,좌상y,우하x,우하y) 사각형인지 | ★확정 = (x0, y0, x1, y1) 축정렬 사각형. 소비처가 결정적: fight_check(m15.ll:35507~35520, 로컬 elx/ely/erx/ery)가 fountains[1-my_team] 을 읽어 ((erx+elx)/2, (ery+ely)/2) 로 중심점을 만든다. 원소 수 = 64B/32B = 2개(팀별), 인덱스 0=블루. |  |
| 1 | AttackNexusPlan.team(+0x0)과 player.info.team 이 항상 같은지 | ★★**다르다. 항상 반대다.** 유일한 생성 지점 `LegacyPlanHandler::passive_plan`(handler.rs:1879, _gaibc/m13.ll:6714~6725)이 `%103 = sub i64 1, player.info.team` 즉 **1 − 내팀 = 적팀**을 넣는다. 바로 아래 `DefenseNexusPlan`(m13.ll:6733~6742)은 대조적으로 `player.info.team` 을 **그대로** 넣는다. ⟹ `AttackNexusPlan.team` = **공격 목표 넥서스의 팀(적팀)**. 버그는 아니다(`sub_plan` 은 `+0x0` 을 안 읽고 `+0x8 line` 만 읽는다). ⚠**재구현 때 둘을 바꿔치기하면 반대 팀 라인을 집는다.** 전 IR 에서 `store i64 16, ptr` 은 m08(3)·m11(19)·m13(1) 뿐이고 앞 둘은 점수표·fmt 인자로 무관, `AttackNexusPlan::next_plan` 은 define 이 없고 m02.ll 에 `store i64 16` 0건 ⟹ **재생성 경로 없음**. |  |
| 2 | has_enemy_twin_tower 의 dbg 표현에 NOT 이 두 번 걸린 것 — 컴파일러 아티팩트로 판단하고 내려놨음 | ★**아티팩트 확정**(전문 = `_shared.DW_OP_not_아티팩트`). 극성은 분기 방향으로 확정: `%59 = 적팀 twin_tower len`, `%60 = (len == 0)`, `br %60 → SubPlan::AttackNexus(16)` / `else → LineDefense(2)` (m12.ll:34955~34970) ⟹ **`has_enemy_twin_tower = (len != 0)`** 이 맞고 dbg 의 이중 NOT(=항등)이 소스와 어긋난 것이다. 이 이름으로 코드를 짜면 `!is_empty()` 가 정답. |  |
| 3 | 3분기(Recall / LineDefense / AttackNexus)가 실행으로 확인된 적이 없다 | ★오라클 실행 확증(2026-09-11 2차배치A, `_verify2\A\A2_oracle3.tsv`, tps=60): ①샘 안(15000,913000)+hp 500/999 → **Recall** ②샘 밖(500000,500000)+hp 500/999 → **LineDefense{line=Mid(=self.line), Push, Aggressive}** ③적팀 타워 전량 제거(twin_towers[1].len()==0) → **AttackNexus**. `AttackNexusPlan::new(1, Mid)` 로 team=1 을 줬는데 결과 line 이 Mid ⟹ **`+0x0 team` 미참조 / `+0x8 line` 만 사용** 재확증. '샘 안이지만 HP 꽉 참'은 ②와 같은 경로 = 분기 순서 확증. |  |
| 4 | L38 `is_in_heal_area` 표기 잔차 2자(1차) | ★해소 ±0(2026-09-11 2차배치A). L38 = `    let is_in_heal_area = champ.x >= lx && champ.x <= rx && champ.y >= ly && champ.y <= ry;` (실측 91자 = indent 4 + 87). IR 의 `!(y<ly \|\| y>ry)` 는 이것의 De Morgan 변형. ⚠1차의 잔차 2자는 **들여쓰기를 2로 가정**해 생긴 것 — tcx sp 10:3·15:3 ⟹ impl 멤버 indent 2, **fn 본문 indent 4**. L41 은 `if is_in_heal_area && champ.hp < champ.stat_cached.hp {`(55자) 대비 실측 54자 = **잔차 1자**. ⚠쌍둥이: `plan_legacy\sub_plan\attack_nexus.rs` 는 다른 파일(L38=1자) — `old\` 한정 조회 필수. |  |
| 5 | `attack_nexus.rs:41` 잔차 1자 | ★**표기 불가(1자)**(3차 배치A). 같은 함수 8줄이 indent 4/6 으로 ±0 이라 들여쓰기는 이미 맞다. 측정 54자 vs 후보 55자이고 `mir=False` 라 MIR 칸도 없다. 신규 정황: 이 58자 줄이 `passive_line.rs:1073`·`single_line.rs:286` 에 **바이트까지 동일** = 공용 이디엄. |  |
</details>

