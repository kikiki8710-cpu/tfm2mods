---

### `10` should_end_object_finish_kill_priority_battle — 오브젝트(Morgard/Serpen) 사냥 전투를 끝낼지 — 그 오브젝트를 때릴 수 있는 적 챔피언이 하나도 안 보이면 true

| 항목 | 값 |
|---|---|
| id | `fight_model__should_end_object_finish_kill_priority_battle` |
| 심볼 | `_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old11fight_model45should_end_object_finish_kill_priority_battle` |
| 소스 | `game-ai\src\plan_legacy\old\fight_model.rs:1214` |
| IR | `m10.ll` 49611~50100행 |
| 경로·가시성 | `game_ai::plan_legacy::old::fight_model::should_end_object_finish_kill_priority_battle` · **in:game_ai** |
| 계층 | 레거시 플랜 |
| exe | `e12050` (fight_model) · 456바이트 · 115명령 |
| 라운드 | 기준 `r6` · 통과 6회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, game_ai::plan_legacy::team_plan::MainObjective) -> bool
```

<details><summary>인자 5개</summary>

| # | i | 이름 | 타입 | 역할 |
|---|---|---|---|---|
| 0 | 1 | version |  | ★AI 버전 게이트가 **아니다** — **죽은 인자**(3차 배치C, ev2). `is_enemy_well_danger` 는 `%0`(version)을 한 번도 로드하지 않는다(`_gaibc/m03.ll:144500~144535` 전문) + 오라클 ver 0~5 × team 0/1 × 17×17 격자 전수 동일. ~~「버전 분기는 전적으로 이 함수 몫」~~ 은 거짓 ⟹ **10 전체에 버전 분기가 없다.** |
| 1 | 2 | rnd |  | 본문에서 직접 읽지/쓰지 않음. PlayerState::strategy 에 그대로 넘기기만 한다 |
| 2 | 3 | player |  | info.team(0x930) 을 읽어 적 팀 인덱스(1-team) 를 만든다 |
| 3 | 4 | data |  | {cache:&AbstractGameWithCache, context:&GameContext, blackboard:&[Blackboard;2]}. context(+0x8)는 이 함수에서 안 쓴다 |
| 4 | 5 | main_objective |  | byte0=태그 / byte1=phase(ObjectPhase) / byte2=with_battle(bool) — dienum MainObjective 0 출력 기준 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
// 진입 게이트 (fight_model.rs:1216) — MainObjective 3바이트를 통째로 마스크 비교
if (main_objective & 0xFFFE) != 0x0300 { return false }
// ⇒ tag(+0x0) ∈ {0=Morgard, 1=Serpen} AND phase(+0x1) == 3(Hunt)
// ⇒ with_battle(+0x2) 는 마스크 밖 = 검사하지 않는다

// 전략 게이트 (1220)
let st: Strategy = PlayerState::strategy(player, rnd, game.data, game.vtable); // 24B sret
if st.object_finish /* +0xf */ != ObjectFinishStrategy::KillPriority /* 0 */ { return false }

// 오브젝트 엔티티 찾기 (1224) — objective_entity_id_for_main_objective(fight_model.rs:1178~1183) 가 인라인됨
let mode = (*game.vtable[0x40])(game.data); // AbstractGame::get_game_mode -> {discr, ptr}
if mode.as_moba().is_none() /* discr != 0 */ { return true } // 비-Moba 모드면 종료
let moba: &MobaMode = mode.payload;
let id: usize = match tag {
 0 /*Morgard*/ => { // 1181
 if moba.jungle_runner.epic.live_list.len() == 0 { return true }
 moba.jungle_runner.epic.live_list[0]
 }
 1 /*Serpen*/ => { // 1183
 if moba.jungle_runner.serpen.live_list.len() == 0 { return true }
 moba.jungle_runner.serpen.live_list[0]
 }
 // 다른 태그 팔은 1216 게이트 때문에 도달 불가 → LLVM 이 제거
};
let objective: &Entity = (*game.vtable[0x1f0])(game.data, id); // get_entity_by_id, 1225
if objective == null { return true } // 1224

// 적 챔피언 주사 (1229~1233)
let e_team = 1 - player.info.team; // 1229. e_team >= 2 면 panic_bounds_check(e_team, 2)
let bb = &data.blackboard[e_team]; // 1230 // `Blackboard[T].last_visible[pos]` = '팀 (1−T) 가 팀 T 의 pos 챔프를 마지막으로 본 틱' ⟹ T=e_team 이므로 1−T=내 팀 = **「내 팀이 본 적인가」**(`_shared.is_recent_visible` 확정)
for enemy in data.cache.player_champion[e_team] { // 슬롯 5개, IR 에서 완전 언롤(+0,+8,+16,+24,+32)
 if enemy.is_none() { continue }
 if !bb.is_recent_visible(game.data, game.vtable, player, enemy) { continue } // 1231

 // 1232: is_ignored_well_enemy(version, player, enemy) 인라인 (fight_model.rs:754~756)
 if enemy.team == TeamType::Player(e_team)
 && path_finder::is_enemy_well_danger(version, player, enemy.x, enemy.y) { continue }

 if can_enemy_hit_objective(enemy, objective, 25000) { return false } // 1233
}
return true; // 1235

// 요약: should_end = !(적 챔피언 중 '최근 보이고' && '우물위험 무시대상이 아니고' && '오브젝트를 여유 25000 안에서 때릴 수 있는' 자가 하나라도 있음)
```

**`mem` 메모리 접근 22건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev |
|---|---|---|---|---|---|---|
| 0 | MainObjective(인자 %4, i24) | 0x0 | tag | r | 0=Morgard, 1=Serpen. 1216 게이트에서 마스크로, 1179 match 에서 trunc i24->i8 로 다시 본다 · tcx 정본 대조( `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=10 idx=0)) | 3 |
| 1 | MainObjective(인자 %4, i24) | 0x1 | phase (ObjectPhase) | r | 1216 게이트가 == 3(Hunt) 을 요구. 본문엔 리터럴 3 이 없고 768(=3<<8)로 접혀 있다 · tcx 정본 대조( `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=10 idx=1)) | 3 |
| 2 | MainObjective(인자 %4, i24) | 0x2 | with_battle (bool) | r | ★마스크 65534 밖 = 이 함수는 with_battle 을 전혀 검사하지 않는다 · tcx 정본 대조( `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=10 idx=2)) | 3 |
| 3 | Strategy(PlayerState::strategy 반환 24B) | 0xf | object_finish (ObjectFinishStrategy 1B) | r | == 0(KillPriority) 이어야 통과. distruct Strategy 로 24B/+0xf 교차검증 ★실행 확인 : `Strategy` 는 24B 이고 raw 덤프의 `+0xf` 바이트가 Debug 의 `object_finish` 와 1:1(기본값 1=BattlePriority). `_verify2/C/C_o10b.tsv` | 3 |
| 4 | PlayerState | 0x930 | info.team (usize) | r | IR 의 gep +2352. 적 팀 = 1 - 이 값 · tcx 정본 대조( `offset_of!` 실행 대조 MISMATCH 0 (`_verify6/C/o1_off.out`·`o2_off_ai.out`·`o3_off2.out`, 오프셋 51 + 구조체 크기 16 전건 OK) · `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=10 idx=4)) | 3 |
| 5 | OperationData | 0x0 | cache (&AbstractGameWithCache) | r | · tcx 정본 대조( `offset_of!` 실행 대조 MISMATCH 0 (`_verify6/C/o1_off.out`·`o2_off_ai.out`·`o3_off2.out`, 오프셋 51 + 구조체 크기 16 전건 OK) · `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=10 idx=5)) | 3 |
| 6 | OperationData | 0x10 | blackboard (&[Blackboard;2]) | r | gep +16. Blackboard 1개 = 744B (dereferenceable(744) 로 교차검증) · tcx 정본 대조( `offset_of!` 실행 대조 MISMATCH 0 (`_verify6/C/o1_off.out`·`o2_off_ai.out`·`o3_off2.out`, 오프셋 51 + 구조체 크기 16 전건 OK) · `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=10 idx=6)) | 3 |
| 7 | AbstractGameWithCache | 0x0 | game.data (&dyn AbstractGame 의 데이터 포인터) | r | game 필드는 16B 팻포인터 · tcx 정본 대조( `offset_of!` 실행 대조 MISMATCH 0 (`_verify6/C/o1_off.out`·`o2_off_ai.out`·`o3_off2.out`, 오프셋 51 + 구조체 크기 16 전건 OK) · `tcxdict` 재조회: `game` = +0x0 16B `&dyn AbstractGame` ⟹ 데이터 절반이 +0x0 (`memchk.tsv` i=10 idx=7)) | 3 |
| 8 | AbstractGameWithCache | 0x8 | game.vtable (&dyn AbstractGame 의 vtable) | r | dereferenceable(816) = AbstractGame vtable 크기와 일치 · tcx 정본 대조( tcx: `AbstractGameWithCache.game` 은 +0x0 의 16B `&dyn` 팻포인터라 **+0x8 = vtable 절반**(Rust 팻포인터 ABI). `tcxdict` 는 팻포인터 뒤 절반을 필드로 세지 않는다(METHOD_MAP ⑦ 한계 명시)) | 3 |
| 9 | AbstractGameWithCache | 0x1e0 | player_champion ([[Option<&Entity>;5];2], 80B) | r | gep +480, 팀당 stride 40B. iter_champions(적팀) 이 여기를 5칸 순회(IR 에선 완전 언롤) · tcx 정본 대조( `offset_of!` 실행 대조 MISMATCH 0 (`_verify6/C/o1_off.out`·`o2_off_ai.out`·`o3_off2.out`, 오프셋 51 + 구조체 크기 16 전건 OK) · `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=10 idx=9)) | 3 |
| 10 | AbstractGame vtable | 0x40 | get_game_mode | r | divtable AbstractGame 0x40. {i64 discr, ptr} 반환 → GameMode | 3 |
| 11 | AbstractGame vtable | 0x1f0 | get_entity_by_id | r | divtable AbstractGame 0x1f0. gep +496 | 3 |
| 12 | MobaMode | 0x198 | jungle_runner.epic.live_list.cap | r | gep +408. MobaMode+0x18=jungle_runner, JungleRunner+0x180=epic, JungleCampState+0x0=live_list(Vec<usize>) · tcx 정본 대조( `offset_of!` 실행 대조 MISMATCH 0 (`_verify6/C/o1_off.out`·`o2_off_ai.out`·`o3_off2.out`, 오프셋 51 + 구조체 크기 16 전건 OK) · `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=10 idx=12)) | 3 |
| 13 | MobaMode | 0x1a0 | jungle_runner.epic.live_list.ptr | r | gep +416 — Morgard 경로가 여기서 [0] 을 읽는다 · tcx 정본 대조( `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=10 idx=13)) | 3 |
| 14 | MobaMode | 0x1a8 | jungle_runner.epic.live_list.len | r | gep +424. 0 이면 즉시 true 반환 · tcx 정본 대조( `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=10 idx=14)) | 3 |
| 15 | MobaMode | 0x1c8 | jungle_runner.serpen.live_list.cap | r | gep +456. JungleRunner+0x1b0=serpen · tcx 정본 대조( `offset_of!` 실행 대조 MISMATCH 0 (`_verify6/C/o1_off.out`·`o2_off_ai.out`·`o3_off2.out`, 오프셋 51 + 구조체 크기 16 전건 OK) · `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=10 idx=15)) | 3 |
| 16 | MobaMode | 0x1d0 | jungle_runner.serpen.live_list.ptr | r | gep +464 — Serpen 경로가 여기서 [0] 을 읽는다 · tcx 정본 대조( `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=10 idx=16)) | 3 |
| 17 | MobaMode | 0x1d8 | jungle_runner.serpen.live_list.len | r | gep +472. 0 이면 즉시 true 반환 · tcx 정본 대조( `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=10 idx=17)) | 3 |
| 18 | Entity(적 챔피언) | 0x0 | team.tag (TeamType 판별자) | r | == 0(Player) 인지 본다 (TeamType 16B) · tcx 정본 대조( `offset_of!` 실행 대조 MISMATCH 0 (`_verify6/C/o1_off.out`·`o2_off_ai.out`·`o3_off2.out`, 오프셋 51 + 구조체 크기 16 전건 OK) · `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=10 idx=18)) | 3 |
| 19 | Entity(적 챔피언) | 0x8 | team.Player.0 (팀 인덱스 usize) | r | gep +8. 1-player.team 과 비교 · tcx 정본 대조( `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=10 idx=19)) | 3 |
| 20 | Entity(적 챔피언) | 0x660 | x (u64) | r | gep +1632. is_enemy_well_danger 인자 · tcx 정본 대조( `offset_of!` 실행 대조 MISMATCH 0 (`_verify6/C/o1_off.out`·`o2_off_ai.out`·`o3_off2.out`, 오프셋 51 + 구조체 크기 16 전건 OK) · `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=10 idx=20)) | 3 |
| 21 | Entity(적 챔피언) | 0x668 | y (u64) | r | gep +1640. is_enemy_well_danger 인자 · tcx 정본 대조( `offset_of!` 실행 대조 MISMATCH 0 (`_verify6/C/o1_off.out`·`o2_off_ai.out`·`o3_off2.out`, 오프셋 51 + 구조체 크기 16 전건 OK) · `tcxdict` 행별 재조회 MATCH (`_verify6/C/memchk.tsv` i=10 idx=21)) | 3 |

**`consts` 상수 6건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 65534 | 1216 | 태그 | MainObjective 3바이트 진입 마스크 0x00FFFE — byte0 의 bit0 만 남기고(태그 0/1 허용) byte1(phase) 전체를 보고 byte2(with_battle) 는 통째로 무시한다 · 오라클 실행 확증( 오라클 `_verify6/C/o5_mask.out §O6-E` — MainObjective 25 케이스의 3바이트를 날바이트로 읽어 `(v&65534)==768` 을 평가, **통과 집합이 Morgard/Serpen×Hunt×with_battle 4개와 정확히 일치**(오검출 0)) | 2 |
| 1 | 768 | 1216 | 임계 | 마스크 결과 기대값 0x000300 — byte1(phase)==3(ObjectPhase::Hunt) & byte0(tag)∈{0=Morgard,1=Serpen}. 리터럴 3 은 3<<8 로 접혀 본문에 없다 · 오라클 실행 확증( 오라클 `_verify6/C/o5_mask.out §O6-E` — MainObjective 25 케이스의 3바이트를 날바이트로 읽어 `(v&65534)==768` 을 평가, **통과 집합이 Morgard/Serpen×Hunt×with_battle 4개와 정확히 일치**(오검출 0)) | 2 |
| 2 | 0 | 1220 | 태그 | ObjectFinishStrategy::KillPriority 의 태그값. strategy.object_finish(+0xf) 가 이 값이 아니면 즉시 false · 오라클 실행 확증( 오라클 `_verify6/C/o5_mask.out §O6-F` — `Strategy` 24B 날바이트의 `+0xf` = 1 이고 `Debug` 가 `object_finish: BattlePriority` ⟹ KillPriority=0 확정) | 2 |
| 3 | 1 | 1229 | 인덱스 | 적 팀 인덱스 = 1 - player.info.team (팀 2개 전제). iter_champions 와 blackboard 인덱스 양쪽에 같은 값이 쓰인다 · 오라클 실행 확증( 오라클 `_verify6/C/o4_s10.out §O6-C` — `is_ignored_well_enemy(0, player.team=0, enemy)` 가 **enemy.team==1 & 적 우물 안** 조합에서만 true(6조합 전수) ⟹ 적 팀 = 1−team) | 2 |
| 4 | 25000 | 1233 | 임계 | can_enemy_hit_objective 의 3번째 인자 — 적이 오브젝트를 '때릴 수 있다'고 볼 여유치. 게임 좌표 단위(셀=32000)로 보면 셀의 약 0.78배. ★**선형 가산으로 분해됐다**: 경계거리 = `20000 + effect.range + margin`, 단 140000 에서 하드컷 · 오라클 실행 확증( 오라클 `_verify5/C/o10b.out §O10b-A` (margin 1:1 가산, 3×8 격자)) | 2 |
| 5 | 19600000000 | 1188 | 임계 | = 140000² . `can_enemy_hit_objective` 진입 **하드컷** — `dx²+dy² > 19.6e9` 이면 즉시 false. 실측 경계 140000 true / 140001 false . **신규 발굴** · 오라클 실행 확증( 오라클 `_verify5/C/o10b.out §O10b-B` (140000 true / 140001 false, 대각 98994/98995)) | 2 |

**`knobs` 조정점 7건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev |
|---|---|---|---|---|---|
| 0 | 적이 오브젝트를 때릴 수 있다고 보는 여유치 | fight_model.rs:1233 (can_enemy_hit_objective 3번째 인자) | 25000 | 올리면 더 먼 적까지 '때릴 수 있음'으로 쳐서 전투를 안 끝낸다(오브젝트 앞에 더 오래 눌러앉음). 내리면 적이 바짝 붙어야만 유지 → 오브젝트 마무리 전투를 빨리 접는다 · 오라클 실행 확증( 오라클 `_verify5/C/o10b.out §O10b-A`) | 2 |
| 1 | 진입 phase 게이트 | fight_model.rs:1216 (마스크 65534 == 768) | 768 | 768(=phase 3 Hunt) 을 다른 값으로 바꾸면 다른 오브젝트 페이즈(1=Setup, 2=Assemble)에서도 이 종료판정이 돌게 된다. 0 으로 두면 phase==None 일 때만 돈다 · 오라클 실행 확증( 오라클 `_verify6/C/o5_mask.out §O6-E` — MainObjective 25 케이스의 3바이트를 날바이트로 읽어 `(v&65534)==768` 을 평가, **통과 집합이 Morgard/Serpen×Hunt×with_battle 4개와 정확히 일치**(오검출 0)) | 2 |
| 2 | 진입 마스크 폭 | fight_model.rs:1216 (65534) | 65534 | 현재 byte2(with_battle) 가 마스크 밖이라 전투동반 여부와 무관하게 판정한다. 마스크를 0xFFFFFE 류로 넓히면 with_battle 조건까지 게이트에 넣을 수 있고, 0xFFFF 로 좁히면(=태그 고정) Morgard/Serpen 중 하나만 대상이 된다 · 오라클 실행 확증( 오라클 `_verify6/C/o5_mask.out §O6-E` — MainObjective 25 케이스의 3바이트를 날바이트로 읽어 `(v&65534)==768` 을 평가, **통과 집합이 Morgard/Serpen×Hunt×with_battle 4개와 정확히 일치**(오검출 0)) | 2 |
| 3 | 전략 게이트 | fight_model.rs:1220 (strategy.object_finish == KillPriority) | 0 | 0→1 로 바꾸면 BattlePriority 전략일 때만 이 종료판정이 돈다. 게이트를 없애면 두 전략 모두에서 '적이 안 보이면 전투 종료'가 걸린다 · 오라클 실행 확증( 오라클 `_verify6/C/o5_mask.out §O6-F` — `Strategy` 24B 날바이트의 `+0xf` = 1 이고 `Debug` 가 `object_finish: BattlePriority` ⟹ KillPriority=0 확정) | 2 |
| 4 | "최근 목격" 시간창 | game_core blackboard.rs:346 `Blackboard::is_recent_visible` (`_gcbc/g07.ll:157005~157050`) — fight_model.rs:1231 호출 | 120 | `last_visible[pos] + 120 >= tick` 이면 '최근에 봤다'. tps=60 이라 **2.0초 고정이며 `tick_per_second` 로 스케일되지 않는다.** 올리면 적이 시야에서 사라져도 한동안 '아직 근처'로 쳐서 오브젝트 마무리 전투를 안 끝내고, 내리면 시야 끊기는 즉시 접는다. 형제 4개 중 big_action 계열은 같은 배열에 600틱을 쓴다(`_shared.is_recent_visible_4형제`). ⚠1차가 '노브인데 표에 없다'고 지적한 것이 `resolved` 에만 있었다 · 오라클 실행 확증( 오라클 `_verify5/C/o10.out §O10-C` 8/8 (tick 1000↔lv 879/880, 5000↔4879/4880, 300↔179/180)) | 2 |
| 5 | 오브젝트 타격 가능 거리 하드컷 | fight_model.rs:1188 (= 140000²) | 19600000000 | `can_enemy_hit_objective` 가 이 거리를 넘으면 이펙트·레벨을 보지도 않고 false 를 낸다. 줄이면 원거리 적을 아예 위협으로 안 세고, 늘리면 맵 절반 밖 적까지 계산에 들어온다. 실측 경계 140000/140001 · 오라클 실행 확증( 오라클 `_verify5/C/o10b.out §O10b-B`) | 2 |
| 6 | objective→entity id 표 | m10.ll:49508~49512 switch | 0/1 만 | 다른 variant 를 열면 게이트를 통과시킨 뒤 실제 대상 id 가 생긴다 · 오라클 실행 확증( 오라클 `_verify6/C/o4_s10.out §O6-D` — 300틱 구동 후 `epic.live=1 serpen.live=1` 상태에서 tag0→`Some(41)` · tag1→`Some(40)` ⟹ **live_list 첫 원소 경로 최초 실행 확인**(5차엔 12/12 전부 None)) | 2 |

<details><summary>`callees` 피호출자 9건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 |
|---|---|---|---|---|---|
| 0 | as_moba | game_core::GameMode::<'a>::as_moba | pub | fn(game_core::GameMode<'a/#0>) -> std::option::Option<&'a/#0 game_core::MobaMode> | game-core\src\simulation\game.rs:230 |
| 1 | can_enemy_hit_objective | game_ai::plan_legacy::old::can_enemy_hit_objective | pub | fn(&game_core::Entity, &game_core::Entity, u64) -> bool | game-ai\src\plan_legacy\old\fight_model.rs:1188 |
| 2 | is_enemy_well_danger | game_ai::is_enemy_well_danger | pub | fn(usize, &game_core::PlayerState, u64, u64) -> bool | game-ai\src\path_finder.rs:1032 |
| 3 | is_ignored_well_enemy | game_ai::plan_legacy::old::is_ignored_well_enemy | pub | fn(usize, &game_core::PlayerState, &game_core::Entity) -> bool | game-ai\src\plan_legacy\old\fight_model.rs:754 |
| 4 | is_recent_visible | game_core::Blackboard::is_recent_visible | pub | fn(&game_core::Blackboard, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\game\blackboard.rs:346 |
| 5 | objective_entity_id_for_main_objective | game_ai::plan_legacy::old::objective_entity_id_for_main_objective | pub | fn(&game_core::OperationData, game_ai::plan_legacy::team_plan::MainObjective) -> std::option::Option<usize> | game-ai\src\plan_legacy\old\fight_model.rs:1178 |
| 6 | strategy | <game_core::Game as game_core::AbstractGame>::strategy | pub | fn(&game_core::Game, usize) -> game_core::Strategy | game-core\src\simulation\game.rs:1830 |
| 7 | strategy | game_core::PlayerState::strategy | pub | fn(&game_core::PlayerState, &mut rand::rngs::std::StdRng, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + ) -> game_core::Strategy | game-core\src\simulation\state\player.rs:1576 |
| 8 | strategy | <game_core::ExpectedGame<'a> as game_core::AbstractGame>::strategy | pub | fn(&game_core::ExpectedGame<'a/#0>, usize) -> game_core::Strategy | game-core\src\simulation\expected_game.rs:57 |
</details>

⚠**미매칭 2개**: `phase`, `with_battle`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 3곳** (m05.ll:19959, m05.ll:29032, m10.ll:26341) · **형제 0개** 

**`open` 0건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**

(없음 — 이 함수는 `open` 이 비었다. 그래도 **게이트 미해소(§4)와 반증은 유효하다**.)

**`notes` 2건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev |
|---|---|---|
| 0 | phase 리터럴 3 이 본문에 없다 — 768(=3<<8) 로 상수접힘. constants 에는 실제 본문 값 768 만 올렸다. | 4 |
| 1 | 적 챔피언 슬롯 5칸은 IR 에서 완전 언롤돼 루프 변수가 없다. 5 라는 상수는 배열 길이라 constants 에 넣지 않았다(SPEC_GUIDE §3 표 규칙). | 4 |

<details><summary>`closed` 7건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

| # | 물음 | 답 | ev |
|---|---|---|---|
| 0 | blackboard 인덱스의 의미 — data.blackboard[1 - player.info.team](=적 팀) 을 is_recent_visible 의 self 로 넘긴다. iter_champions 와 완전히 같은 값(%56)을 쓰는 것은 IR 로 확정했으나, '적 팀의 blackboard 로 적 챔피언의 가시성을 묻는' 의미가 무엇인지는 확정 못 함. is_recent_visible 이 player 인자도 함께 받으므로 관측자는 그쪽에서 정해질 가능성이 있다. |  |  |
| 1 | Blackboard::is_recent_visible 본체 — _gaibc 전체에 define 이 없다(declare 만). game_core 쪽 코드라 '최근'의 기준(틱 수 등)을 확인할 수 없다. grep 'define.*Blackboard17is_recent_visible' /c/tfm2mods/_gaibc/*.ll 결과 0건. |  |  |
| 2 | can_enemy_hit_objective(fight_model.rs:1233) 내부 미확인 — 25000 이 거리(제곱 아님)인지, 사거리+여유인지, 시간인지 단위 미확정. 담당 범위 밖이라 본문을 안 봤다. |  |  |
| 3 | path_finder::is_enemy_well_danger 내부 미확인 — version 이 여기서만 쓰이므로 버전 분기는 전적으로 이 함수 몫이다. |  |  |
| 4 | p2 rnd(&mut StdRng, 320B) — 이 함수 본문에서 직접 읽거나 쓰지 않고 PlayerState::strategy 에 그대로 전달만 한다. strategy 가 난수를 소비하는지(=이 함수가 RNG 상태를 전진시키는 부작용이 있는지)는 확인 못 했다. writes 를 빈 배열로 둔 이유가 이것이다 — 이 함수 자신은 어떤 필드에도 store 하지 않는다(본문에 store 명령 0개). |  |  |
| 5 | MainObjective 태그 0/1 이외에서의 원본 동작 — 1216 게이트가 먼저 걸러내므로 인라인된 objective_entity_id_for_main_objective 의 `_ => None` 팔이 LLVM 에서 제거됐다. 원본 함수에는 있을 것으로 보이나 이 범위에서는 복원 불가. |  |  |
| 6 | AbstractGame vtable 슬롯 이름(0x40 get_game_mode / 0x1f0 get_entity_by_id)은 divtable 이 'ExpectedGame' impl 기준으로 준 것이다. 런타임에 어느 구현체가 꽂히는지는 이 도구로 알 수 없다(도구 자체 경고). |  |  |
</details>

<details><summary>`history` 정정 이력 9건 (참조용 — 본문 아님)</summary>

| # | 옛 값 | 현재 | 근거 |
|---|---|---|---|
| 0 | can_enemy_hit_objective 의 25000 이 거리인지 시간인지 | ★확정 = 거리 오프셋(월드 좌표, 제곱 아님). 3번째 인자 이름 = range_margin (_gaibc/m10.ll:116959 !53081, fight_model.rs:1188). 본문(m10.ll:47454~47623)은 그대로 Effect::is_in_range_ex 의 8번째 인자로 넘긴다. is_in_range_ex 본문(_gcbc/g06.ll:51807~51890): total = effect.range + offset + caster+0x680 + (caster+0x5c8-1)*effect+0x18 + range_adjust + 보정들; return dx²+dy² <= total². ⟹ 제곱 전 사거리에 그대로 더해진다. 1셀=32000 이므로 25000 ≈ 0.78셀. |  |
| 1 | 같은 함수의 다른 큰 상수 | 19600000000 = 140000² 은 steal_position_range (fight_model.rs:1189, m10.ll:47501). 적↔오브젝트 거리가 140000(=4.375셀) 초과면 즉시 false. |  |
| 2 | blackboard 인덱스 의미 / is_recent_visible 본체 | _shared 참조 — 둘 다 확정됨. |  |
| 3 | is_enemy_well_danger 의 버전별 분기표 | ★정정 — 버전 분기표는 존재하지 않는다. _gaibc/m03.ll:144500~144563(path_finder.rs:1032)에서 version(%0)은 dbg_value 로만 등장하고 본문에서 한 번도 쓰이지 않는다. 분기는 전부 player.info.team 기준의 두 사각형 OR: 적팀0 → (0,800000,64000,960000) ∪ (0,896000,160000,960000), 적팀1 → 대칭. 경계 포함(icmp ult ..., 64001). |  |
| 4 | MainObjective 태그 0/1 이외에서의 원본 동작 — LLVM 이 `_ => None` 팔을 제거해 복원 불가 | ★**"복원 불가"가 뒤집혔다.** `fnparts` 로 아웃오브라인 본체를 찾았다 — `_gaibc/m10.ll:49498~49608`(fight_model.rs:1178~1186), 게이트 없는 원본이라 `_ => None` 팔이 살아 있다. **0 Morgard → epic.live_list 첫 원소 / 1 Serpen → serpen.live_list 첫 원소 / 2~11 전부 None.** 전문 = `_shared.objective_매핑표.objective_entity_id`. |  |
| 5 | `is_recent_visible` 확정 사실이 logic/unknown 에 미반영, 120틱이 knobs 에 없음 | ➕**보강(2026-09-11 검증배치 C)**: `is_recent_visible` = **"내 팀이 최근 2.0초(120틱 고정) 안에 본 적인가"** 가 `_shared` 에서 이미 확정됐는데 이 명세의 `logic`/`unknown` 에 반영이 안 돼 있었다. **120 은 노브인데 `knobs` 에 없다** — 추가 대상. |  |
| 6 | p2 rnd — `strategy` 가 난수를 소비하는지(RNG 부작용) 확인 못 했다 | ★**소비하지 않는다**(실행 확정, 2026-09-11 2차배치C, `C_o10b.rs`): `PlayerState::strategy` 호출 전후로 같은 시드의 난수열이 **완전 동일(5/5)**, 반환 `Strategy` 도 RNG 상태와 무관 ⟹ 이 함수는 **RNG 부작용이 없고 `writes: []` 가 옳다**. 적용 범위 = Moba·tick 0·`TeamColorStrategy` 12필드 전부 None(기본 팀). 랜덤 전략은 `thread_rng()` 를 쓰므로(`_shared.TeamColorStrategy_random`) 인자 rnd 와 무관. |  |
| 7 | `can_enemy_hit_objective` 의 25000 단위 / `is_enemy_well_danger` 내부 | ★둘 다 확정(3차 배치C). **25000 = 선형 거리 여유치**(`Effect::is_in_range_ex` 의 8번째 인자로 들어간다), skill2 는 `level>2` · ult 는 `level>4` 게이트. `is_enemy_well_danger` = 적 진영 우물 **L자 2사각형**(team 별 대칭) 판정이고 version 미사용. vtable 슬롯 실측 4종: `0x28 tick`(5) / `0x40 get_game_mode`(8) / `0x1f0 get_entity_by_id`(62) / **신규 `0x108 strategy`**(33). `objective_entity_id_for_main_objective` 전수: 태그 0→`epic.live_list[0]`, 1→`serpen.live_list[0]`, **2~11→None**(ev2). |  |
| 8 | `divtable` 슬롯이 `ExpectedGame` 기준이라 런타임 구현체가 다르면 슬롯이 다를 수 있다(우려) | ★**해소**(4차 배치C, **런타임 대조 4/4**). 진짜 `Game` 의 `&dyn` 팻포인터에서 슬롯을 꺼내 간접호출 ↔ 직접호출을 대조: `vtable+0x28 tick` · `+0x40 get_game_mode` · `+0x1f0 get_entity_by_id`(포인터 동일) · `+0x108 strategy`(3차 신규 주장 재확인) **전건 일치**. ⟹ **슬롯 인덱스는 impl 무관**이다. 남는 「어느 impl 이 꽂히나」는 **호출자 성질**이라 이 명세의 미탐색 항목이 아니다. |  |
</details>

