# apply060 batch_01.md — 반영(logic_060) 브리핑 · 함수 9

★**version = 3 런타임 확정(09-17)** — v<3/v2 경로는 명세에서 제외(사장). 태그/오프셋/vt 슬롯 = `spec_patch_060.md` §A(dispcheck 추가 행 포함). 출력 형식·절차 = `mkapply060.py` 도크스트링. 출력 = `_next/apply060/out/<old>.md`.

RE 정본 파일: 2026-09-17_r21_심층_w11_미발견12_마무리_update_on_dead_score클로저_best_jungle_goal_make_gank_battle_stake_원문.md

### `d40b20` → `fafe50` best_jungle_goal (i=19 · 변경(r21 w11: v2 동치 · v3 3블록 · allow_invade 인자))
- 0.5.8 src: `game-ai\src\plan_legacy\old\passive_jungle.rs:806` · one_line: 정글러가 다음에 갈 캠프(JungleType)를 고른다 — 미클리어 캠프 중 내 챔프에서 가장 가까운 것. ★출구는 셋이다: ①본선 = 미클리어 캠프 중 제곱거리 최소 ②전부 클리어(817) = `next_respawn_tick` 이 가장 빠른 캠프(**이미 잡힌 캠프가 나온다**) ③내 챔프 엔티티가 없음(824) = `now_camp` 를 그대로(**Morgard/Serpen 도 가능**) 또는 4개 중 랜덤 — ②③은 미클리어 필터를 안 탄다
- 0.6.0 판정: **심층(r21)** · 패치 요지: 7번째 인자 **allow_invade**(passive_plan: `tick>=LPH+0x20d8` · PassiveJungle: `tick>=self+0x78` · fa66b6: 1) · **v2 = 0.5.8 [807~835] 동치**(camps [0,1,3,2] · not_cleared e052c0/faf060 · min_by next_respawn/dist² · camp_pos TLS 메모이즈 1416a7af0) · **v3 3블록**: A 120000² 내 live 캠프(하드코딩 좌표 blue Rhino(752000,496000)/Mushroom(592000,176000)/Stump(448000,256000)/Bee(800000,351000) · red x↔y) min dist² · obj_soon 플래그(tutorial∈{0,5,7,8} · 30tps 이내 epic/serpen 리스폰) · B allow_invade 역정글(vt+0x108 byte+0x11 캠프 쌍 · faf2a0 · dbd370 · 적 정글러 리스폰 vs 도달 dbe860 — **미독**) · C′ vt+0x108 byte+0x11 로 not_cleared 필터({1,2}/{0,3} · 비면 폴백)
- RE 정본: `2026-09-17_r21_심층_w11_미발견12_마무리_update_on_dead_score클로저_best_jungle_goal_make_gank_battle_stake_원문.md` · 절: w11 §B1
- sig: `fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan, std::option::Option<game_core::JungleType>, &mut game_core::DebugFrameData) -> game_core::JungleType`
- consts: [{"value": 0, "src_line": 807, "meaning": "jungle_camps[0] = JungleType::Rhino — ★`JungleType` 의 **판별자**다(임계가 아니다). 오라클 D7_o1 실측 태그 = Rhino 0 · Mushroom 1 · Bee 3 · Stump 2 (dienum JungleType 0=Rhino). 같은 값 0 이 ① get_game_mode 반환 tag==0(=Some) 판정 ② player.info.team==0(=블루 진영) 판정에도 쓰인다 · 오라클 실행 확증( 오라클 game==mine 236/236) · 오라클 실행 확증(5차 배치D: 5차 배치D 오라클 game==mine 236/236)", "kind": "태그", "ev": 2}, {"value": 1, "src_line": 807, "meaning": "jungle_camps[1] = JungleType::Mushroom — ★`JungleType` 의 **판별자**다(임계가 아니다). 오라클 D7_o1 실측 태그 = Rhino 0 · Mushroom 1 · Bee 3 · Stump 2 · 오라클 실행 확증( 오라클 game==mi
- 0.5.8 logic 전문:
```
fn best_jungle_goal(version, rnd, player, data, team_plan, now_camp, debug) -> JungleType

[807] jungle_camps: [JungleType;4] = [Rhino(0), Mushroom(1), Bee(3), Stump(2)]
// ※ 후보 배열에는 Morgard(4)/Serpen(5)가 없다. 다만 **반환값이 일반 캠프로 한정되지는 않는다** — 824~829 폴백이 `now_camp` 를 **그대로 돌려주므로** Morgard/Serpen 도 나올 수 있다 (실측 `champNone_nowcamp Some(Morgard) game=Morgard`). ⚠재구현에서 반환값을 일반 캠프로 필터링하면 **동작이 달라진다**.

[814-815] not_cleared_camps: bumpalo Vec<JungleType> =
 jungle_camps.into_iter()
 .filter(|c| !is_cleared(*c, player.info.team, _version, _rnd, player, data, team_plan, offset=0, _debug))
 .map(identity)
 .collect_in(data.context.pool)
 ※ filter 술어는 담당 범위 밖(call_mut 심 = m04.ll 66311~66341)에 있고,
 본체는 passive_jungle::is_cleared(passive_jungle.rs:841). 결과를 xor true 로 뒤집는다
 = '아직 안 잡힌 캠프만' 남긴다. offset 은 리터럴 0 으로 고정.

[817] if not_cleared_camps.is_empty() { // 전부 클리어된 상태. ★소스 형태는 `is_empty()` 다 — m04.ll:62643 의 !dbg 사슬이 `len<JungleType>`@vec.rs:1617 ← `is_empty<JungleType>`@vec.rs:1636 ← passive_jungle.rs:817 이다. IR 에서는 `load(len @+0x18)` + `icmp eq i64 …, 0` 으로 접힌다
[818] return jungle_camps.into_iter()
[819] .min_by_key(|c| data.cache.game.get_game_mode() // vtable +0x40, indirect call
                        .unwrap() // tag!=0 이면 option::unwrap_failed
                        .jungle_runner // MobaMode +0x18, 480B
                        .get_camp_state(player.info.team, *c).next_respawn_tick)
[821] .unwrap();
 // ★DWARF 정본(11차 배치D): vtable +0x40 간접호출(m04.ll:62706~62708)의 !dbg 는 scope 가
 // `closure$2 @passive_jungle.rs:819` 이고 inlinedAt 루트가 818 이다 ⟹ get_game_mode() 는
 // **키 클로저 안**에 있다(LLVM 이 루프 밖으로 호이스트했을 뿐, 818 의 `let` 이 아니다).
 // 체인 끝 `.unwrap()` 은 **821**(!71892 = option.rs:1011 ← 821).
 // ★즉 '가장 먼저 리스폰될 캠프'로 미리 간다. 동점이면 배열 앞쪽(Rhino→Mushroom→Bee→Stump)이 이긴다
 // (fold 가 cmp<Greater 일 때만 교체 = min_by 의 first-wins).
 }

[823] if player.info.team >= 2 { panic_bounds_check(team, 2) } // 배열 경계
 let champ: Option<&Entity> = data.cache.player_champion[player.info.team][player.info.position];
 // player_champion = AbstractGameWithCache +0x1e0, [[Option<&Entity>;5];2] (team stride 40B, position stride 8B)

[824] if champ.is_none() { // 내 챔프 엔티티가 없다(사망/미스폰 등)
[825] let camp = match now_camp {
 Some(c) => c, // 지금 가던 캠프를 그대로 유지
[829] None => *jungle_camps.choose(rnd).unwrap(), // 4개 중 랜덤(choose 가 None 이면 unwrap_failed)
 };
 return camp; // ※ not_cleared 필터를 전혀 안 탄다
 }

[832] let champ: &Entity = champ.unwrap();
[833] return not_cleared_camps.into_iter()
 .min_by_key(|c| {
[834] let (cx, cy) = data.context.map.camp_pos(*c, player.info.team == 0);
[835] let dx = abs_diff(cx, champ.x); // Entity +0x660
 let dy = abs_diff(cy, champ.y); // Entity +0x668
 dx*dx + dy*dy // 제곱거리 (sqrt 없음)
 })
 .unwrap();
 // ★본선 판정: 아직 안 잡힌 캠프 중 내 챔프에서 제곱거리가 최소인 것.
 // 동점이면 not_cleared_camps 의 앞 원소(=jungle_camps 배열 순서)가 이긴다.

// 부수효과 없음: 구조체 write 는 하나도 없고 지역 alloca(배열/Vec/IntoIter)만 쓴다.
// 예외 경로에서 Vec/RawVec/IntoIter 의 Drop 과 drop_glue 를 호출하는 것이 전부.
```
- logic_note: ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)

### `e06df0` → `ee5e80` resolve_fight_stake (i=35 · 변경(r21 w11: bias · arrivals=ee3c80 포킹취약 6tps+1))
- 0.5.8 src: `game-ai\src\plan_legacy\old\fight_model.rs:571` · one_line: 교전 저울(resolve_fight_full)을 '묶인 아군을 버린 판' 기준선으로 한 번 더 돌려 차분 판정(FightPrediction)을 낸다
- 0.6.0 판정: **심층(r21)** · 패치 요지: `bias = aggr?1 : def?-1 : 0` · **`arrivals[i] = ee3c80(version,ctx,near_allies[i],near_enemies) 포킹취약 ? 6tps+1 : 0`** → absolute/diff 에 (bias, &arrivals) · abandon 은 (0, &[]) · v<2 경로 bias 만 · L582/L587/L594~597 동일
- RE 정본: `2026-09-17_r21_심층_w11_미발견12_마무리_update_on_dead_score클로저_best_jungle_goal_make_gank_battle_stake_원문.md` · 절: w11 §B5
- sig: `fn(usize, &mut rand::rngs::std::StdRng, &game_core::OperationData, &game_core::PlayerState, &game_core::Entity, &[&game_core::Entity], &[&game_core::Entity], i8, std::option::Option<&game_core::Entity>, usize, &mut game_core::DebugFrameData) -> game_ai::plan_legacy::old::FightPrediction`
- consts: [{"value": 2, "src_line": 574, "meaning": "version < 2 이면 차분 저울 없이 resolve_fight_full 한 번으로 끝(`icmp ult i64 %1, 2`)", "kind": "임계", "ev": 4}, {"value": 0, "src_line": 591, "meaning": "abandon 저울의 committed_dir = 0 (히스테리시스 없음) — 묶인 아군만 남긴 가정 판이라 기존 커밋 방향을 안 준다", "kind": "태그", "ev": 4}, {"value": 0, "src_line": 587, "meaning": "remaining.len == 0 이면 absolute 를 그대로 반환", "kind": "태그", "ev": 4}, {"value": 1, "src_line": 597, "meaning": "rescue_ally Option<usize> 의 Some 태그(phi [1,%108]); None 은 0", "kind": "태그", "ev": 4}]
- 0.5.8 logic 전문:
```
fn resolve_fight_stake(version, rnd, data, player, champ, near_allies, near_enemies, committed_dir, tower, judge_accuracy, debug) -> FightPrediction

// ── 경로 A: 구버전 ────────────────────────────────────────────
[L574] if version < 2 {
[L575]   return resolve_fight_full(version, data, champ, near_allies, near_enemies,
                                  committed_dir, tower, judge_accuracy, arrivals=&[], baseline=0)
       }

// ── remaining = '교전에 묶인' 아군(나 제외) ─────────────────────
[L582~585] remaining: Vec<&Entity, &Bump> = near_allies.iter().copied()
             .filter(|a| {                      // ★closure#0 = aux m10.ll 56168~56212 (call_mut 심)
[L584]         a.id(+0x5c0) != champ.id(+0x5c0)  // 자기 자신 제외 (같으면 즉시 false, ally_is_bound 호출 안 함)
               && fight_model::ally_is_bound(version, rnd, data, player, a, near_enemies, debug)
             })
             .collect_in(data.context(+0x8).pool(+0x0))   // bumpalo from_iter_in

// ── 절대 판정 ───────────────────────────────────────────────
[L586] absolute = resolve_fight_full(version, data, champ, near_allies, near_enemies,
                                     committed_dir, tower, judge_accuracy, &[], baseline=0)

// ── 경로 B: 묶인 아군이 없으면 절대 판정 그대로 ─────────────────
[L587] if remaining.len(+0x18) == 0 { drop(remaining); return absolute }

// ── 경로 C: 차분 저울 ─────────────────────────────────────────
[L591] abandon = resolve_fight_full(version, data, champ,
                                    allies = remaining(ptr +0x0, len +0x18),   // ★아군을 '묶인 아군'만으로
                                    near_enemies, committed_dir = 0,           // ★히스테리시스 없음
                                    tower, judge_accuracy, &[], baseline=0)
[L592~593] diff = resolve_fight_full(version, data, champ, near_allies, near_enemies,
                                     committed_dir, tower, judge_accuracy, &[],
                                     baseline = abandon.net_value(+0x30))   // ★'버린 판'의 순가치를 기준선으로
[L594] diff.line_absolute(+0x39) = absolute.line(+0x38)
[L595] if diff.line(+0x38) != absolute.line(+0x38) {
[L597]   diff.rescue_ally(+0x20/+0x28) = remaining.iter()
             .min_by_key(|a| ( (a.x(+0x660).abs_diff(champ.x))^2 + (a.y(+0x668).abs_diff(champ.y))^2, a.id(+0x5c0) ))   // ★키 = (dist², id) 튜플 — 동거리면 id 작은 쪽
             .map(|a| a.id(+0x5c0))              // 나와 가장 가까운 묶인 아군 = 구조 대상
       }
       // 같으면 diff.rescue_ally 는 resolve_fight_full 이 준 값 그대로
[L599] drop(remaining); return diff

※ 세 저울(absolute/abandon/diff)은 모두 arrivals=&[] (합류 없음). 부작용은 rnd/debug 가 ally_is_bound 로 &mut 전달되는 것뿐.
※ 순서: absolute 는 remaining 수집 뒤에, abandon → diff 순으로 호출된다(IR 블록 24→42→50→54).
```
- logic_note: ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)

### `e0b730` → `eec620` v25_scoped_battle_objective (i=48 · 변경·한 줄(r21 w11: phase {1,2}→{1,2,4}))
- 0.5.8 src: `game-ai\src\plan_legacy\old\fight_model.rs:1138` · one_line: Morgard/Serpen 목표(Setup·Assemble 단계)가 focus 엔티티 근처(240000)의 국지전이 아니면 목표를 None 으로 지운다
- 0.6.0 판정: **심층(r21)** · 패치 요지: 한 줄: phase 집합 `{1,2}` → **`{1,2,4}`**(마스크 0x16 · 4 = take_born/Hunt 계) · 거리식·camp_pos·반환 동일
- RE 정본: `2026-09-17_r21_심층_w11_미발견12_마무리_update_on_dead_score클로저_best_jungle_goal_make_gank_battle_stake_원문.md` · 절: w11 §B7
- sig: `fn(usize, std::option::Option<game_ai::plan_legacy::team_plan::MainObjective>, &game_core::PlayerState, &game_core::OperationData, std::option::Option<usize>) -> std::option::Option<game_ai::plan_legacy::team_plan::MainObjective>`
- consts: [{"value": -1, "src_line": 1145, "meaning": "Option<MainObjective> 의 None 니치 = 태그 255(i8 -1). DWARF DISCR_EXACT=255 로 확인. 입력이 None 이면 그대로 None 반환, 국지전 아니면 None 으로 지운다", "kind": "센티널", "ev": 3}, {"value": 0, "src_line": 1145, "meaning": "MainObjective 메모리태그 0 = Morgard{phase, with_battle}", "kind": "태그", "ev": 4}, {"value": 1, "src_line": 1145, "meaning": "MainObjective 메모리태그 1 = Serpen{phase, with_battle}", "kind": "태그", "ev": 4}, {"value": 2, "src_line": 1145, "meaning": "phase 게이트: (phase-1) <u 2 ⟺ phase ∈ {1=Setup, 2=Assemble}. None(0)·Hunt(3) 단계는 손대지 않는다", "kind": "임계", "ev": 4}, {"value":
- 0.5.8 logic 전문:
```
fn v25_scoped_battle_objective(_version, main_objective: Option<MainObjective>, player, data, focus: Option<usize>) -> Option<MainObjective>

[L1145] match main_objective.tag(byte0) {
  255 (None)            → return main_objective (None)
  0  (Morgard{phase,..}) if phase ∈ {Setup(1), Assemble(2)}:
[L1146~1147]  return if v25_is_objective_local_battle(JungleType::Morgard(4), player, data, focus) { main_objective } else { None }
  1  (Serpen{phase,..})  if phase ∈ {Setup(1), Assemble(2)}:
[L1149~1150]  return if v25_is_objective_local_battle(JungleType::Serpen(5),  player, data, focus) { main_objective } else { None }
  _  (Morgard/Serpen 의 다른 phase, 그 외 태그 2..11)
[L1154]       → return main_objective 그대로
}

// ── 인라인된 헬퍼 v25_is_objective_local_battle (fight_model.rs:1158~1166 추정, dloc 로 이름 확정) ──
[L1162] camp = data.context(+0x8).map(+0x20).camp_pos(jungle_type, player.info.team(+0x930) == 0)   // (x,y)
[L1163] focus_local_range = 240000
[L1164] e = focus.and_then(|id| game.get_entity_by_id(id))          // vtable+0x1f0 · null → false
[L1165] return e.is_some_and(|e| {
          dx = abs_diff(e.x(+0x660), camp.x); dy = abs_diff(e.y(+0x668), camp.y)
          dx*dx + dy*dy <= 240000²                                    // Morgard: !(> 57600000000) / Serpen: < 57600000001
        })

// 반환 조립 [L1154]: (main_objective & 0xFFFF00) | new_tag  — phase·with_battle 바이트는 항상 입력값 유지
// 분기 우선순위 근거: switch(i8 byte0) 한 번 → phase 범위검사 → focus.tag(i1) → get_entity null → 거리. phi %60 의 10개 유입이 전부 태그값.
```
- logic_note: ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)

### `de0770` → `f18f90` TeamPlan::update (i=54 · 변경(r21 w11: 레거시 본체 동치 · 감사 프롤로그 · finish_race 에필로그))
- 0.5.8 src: `game-ai\src\plan_legacy\team_plan.rs:294` · one_line: 매 틱 TeamPlan 상태 갱신 — 정글캠프 리스폰 추적·타임아웃 정리·웨이브우선 라인·에픽/세르펜 콜 채팅·적 시야 추적·MIA 채팅
- 0.6.0 판정: **심층(r21)** · 패치 요지: **레거시 본체(f1a5a3~f1d3da) = 0.5.8 L295~L442 동치** · ★정정: **+0xcc7 세팅처 f0f080 은 별도 pre-update 함수(0xea4 · LPH::update 만 호출 · sanitize f105d0+f0e5f0+f10f70 래핑 · hunt_call_pending +0xc98 정리도 여기)** · f18f90 은 bb +0x250/+0xc98/+0x360 참조 없음 · 신규 프롤로그 5.6KB = v6 감사 텔레메트리(+0xa48~/+0xcc8~+0xccf/+0x330/+0x350 · f20f60/f21490 · 결정 영향 없음 **추정**) · **신규 에필로그 `+0x138..+0x158 = fe1ad0 finish_race(player,data)` 캐시**(end_check f28320 등 v3 소비)
- RE 정본: `2026-09-17_r21_심층_w11_미발견12_마무리_update_on_dead_score클로저_best_jungle_goal_make_gank_battle_stake_원문.md` · 절: w11 §B4
- sig: `fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData)`
- consts: [{"value": 0, "src_line": 285, "meaning": "GameMode 태그 0 = Moba. init 은 Moba 가 아니면 리스폰 초기화를 건너뜀(패닉 없음)", "kind": "태그", "ev": 4}, {"value": 2, "src_line": 301, "meaning": "objective_discipline 니치 태그 2 = None (kind 0 SafeWait/1 HardDisengage)", "kind": "센티널", "ev": 4}, {"value": 32000, "src_line": 309, "meaning": "셀 크기 — 캠프 좌표 → 셀 인덱스(xi, yi) 변환(L309·310·357·358·366·367)", "kind": "인덱스", "ev": 4}, {"value": 0, "src_line": 317, "meaning": "get_camp_state(..).live_list.len == 0 (캠프 비어 있음) 일 때만 next_respawn_tick 재계산", "kind": "임계", "ev": 4}, {"value": 10, "src_line": 327, "meaning": "allies[i].last
- 0.5.8 logic 전문:
```
fn update(&mut self, version, rnd, player, data, debug)

[L295] context = data.context; self.sanitize_rule_scope(context)          // 외부 호출(내용 안 봄)
[L296] if !self.is_init(+0x41c) {  // ── init 인라인 (team_plan.rs:281~289)
[L282]   self.is_init = true; team = player.info.team
[L285]   if game.get_game_mode() is Moba(m) {           // 태그 0 아니면 건너뜀
[L288]     for (idx, jungle) in [(0,Rhino0),(1,Mushroom1),(2,Bee3),(3,Stump2)] {
             self.next_respawn_tick[team][idx]   = m.jungle_runner(+0x18).get_camp_state(team,   jungle).+0x18
[L289]       self.next_respawn_tick[1-team][idx] = m.jungle_runner.get_camp_state(1-team, jungle).+0x18
           } } }
[L301] if let Some(st) = self.objective_discipline (+0x149 != 2) { if st.until_tick(+0x140) <= tick { [L302] self.objective_discipline = None } }

// ── 정글캠프 리스폰 추적 (L305~320) — team ∈ {0,1}, jungle ∈ [Rhino,Mushroom,Bee,Stump]
[L308] camp_pos = MapDef::camp_pos(map, jungle, team==0); (xi,yi) = (x/32000, y/32000)
[L311] if game.is_visible_cell(my_team, xi, yi) {                    // ★내 팀 시야 기준 (인자 = player.info.team)
[L314]   if tick >= self.next_respawn_tick[team][idx] {
[L317]     if !(mode is Moba && m.jungle_runner.get_camp_state(team, jungle).live_list.len(+0x10) != 0) {
[L320]       info = JungleType::get_info(&jungle, setting)   // 272B
             self.next_respawn_tick[team][idx] = tick + info.respawn_tick(+0x100)
           } } }

// ── 타임아웃 정리 (L325~346)
[L326] for i in 0..5 { if allies[i].is_some() && allies[i].last_tick + tps*10 < tick { allies[i] = None } }   // 각 i 뒤에
[L331]   if let Some(t) = ally_battle_stop_tick[i] { if t + tps*6 < tick { = None } }   (i 별 교대로 실행)
[L338] if let Some(t) = obj_spawn.serpen_giveup_tick(+0x60) { [L339] if t + tps*20 < tick { [L340] = None } }
[L344] if let Some(t) = obj_spawn.epic_giveup_tick(+0x50)   { [L345] if t + tps*20 < tick { [L346] = None } }
       // ⚠IR: L344 검사는 L338 의 Some 분기 안(%109 는 %98·%112 에서만 도달)… 정확히는 %666 에서 serpen None 이면 %109 로 직접 감 → 결국 두 검사 모두 항상 실행됨

// ── 웨이브 우선 클리어 라인 (L350 → update_wave_priority_clear_line 540~559 인라인)
[L541] if player.strategy(rnd, game).minion_wave(+0x10) == WavePriority(0) {
[L546]   if let Some(line) = self.wave_priority_clear_line(+0x41d) {
[L547]     if rule_scope::line_exists(context, line) {
[L551]       if blackboard[team].<line>_minion_state.minion_count(+0x20/+0x48/+0x70) > 0 {
[L552]         self.wave_priority_clear_line = None; [L558] = find_wave_priority_clear_line(player, data) }
             else { 유지 }
           } else { self.wave_priority_clear_line = None }          // 라인 자체가 룰스코프 밖
[L558]   } else { self.wave_priority_clear_line = find_wave_priority_clear_line(player, data) }
       } else { self.wave_priority_clear_line = None }

[L351] position = player.info.position(+0x9c0)
       if blackboard[team].big_goal[position].1 == Some(BigGoal::Battle{focus: Some(_)}) { [L352] self.last_battle_tick(+0x220) = tick }

// ── 오브젝트 캠프 가시 틱 (L355~369)
[L355] match tutorial { 0|7|8 => {
[L356]   epic_camp = camp_pos(map, Morgard(4), team==0); [L359] if is_visible_cell(team, x/32000, y/32000) { [L360] epic_camp_last_visible_tick(+0x80) = tick } ; fallthrough to serpen }
         5 => serpen only, _ => skip }
[L365]   serpen_camp = camp_pos(map, Serpen(5), team==0); [L368] if is_visible_cell(..) { [L369] serpen_camp_last_visible_tick(+0x88) = tick }

// ── 스폰 콜 채팅 (L374~392) — 서포터만, 10틱마다
[L374] if position == Support(4) && tick % 10 == 0 {
[L375]   tps1 = max(tps, 1)
[L376]   remain_tick = moba.map_or(0, |m| m.epic.next_respawn_tick(+0x1b0)).saturating_sub(tick)
[L378]   if tutorial ∉ 1..=6 && moba.map_or(false, |m| m.epic.live_list.len(+0x1a8) == 0) {
[L379]     if moba.epic.next_respawn_tick.saturating_sub(self.epic_spawn_call_tick(+0x70)) > tps*20 && remain_tick <= tps*20 {
[L381]       self.epic_spawn_call_tick = tick; [L382] remain_second = remain_tick / tps1
[L383]       self.chats.push(Chat::MorgardPrepare(remain_second, 0)) } }
[L386]   remain_tick = moba.map_or(0, |m| m.serpen.next_respawn_tick(+0x1e0)).saturating_sub(tick)
[L387]   if tutorial ∈ {0,5,7,8} && moba.serpen.live_list.len(+0x1d8) == 0 {
[L388]     if serpen.next_respawn_tick.saturating_sub(self.serpen_spawn_call_tick(+0x78)) > tps*20 && remain_tick <= tps*20 {
[L390]       self.serpen_spawn_call_tick = tick; [L391] remain_second = remain_tick / tps1
[L392]       self.chats.push(Chat::SerpenPrepare(remain_second, 0)) } } }

// ── 적 시야 추적 + MIA 콜 (L397~442)
[L397] my_champ = cache.player_champion[team][position]; [L399] if None → return
[L400] for p in game.iter_player() {                              // &[PlayerState] stride 2528
[L401]   pteam = p.info.team; c = cache.player_champion[pteam][p.position]
[L403]   if pteam != team {                                        // 적만 갱신
[L404]     if let Some(c) = c {
[L406]       if c.is_visible_from(my_champ) {   // my_champ.team==Neutral || c.visible_state[my_champ.team].tag==Visible(0)
[L407~411]     last_visible_ticks[pos]=tick; last_checked_ticks[pos]=tick; last_visible_distance[pos]=distance_sq(c,my_champ);
               last_visible_pos[pos]=(c.x,c.y); last_hp_ratio[pos]=c.hp*100/c.stat_cached.hp }
           } else {                                                // 적 챔프 없음(사망)
[L414~416]   last_visible_ticks[pos]=tick; last_checked_ticks[pos]=tick; last_visible_distance[pos]=10^12
[L418]       home = cache.nexus[pteam](+0x170).map(|n| (n.x,n.y)).unwrap_or(map.nexus_pos[pteam](+0x6d50))
[L420]       last_visible_pos[pos] = home } }
       // ⚠아군(pteam==team)도 아래 MIA 검사는 통과한다 — 단 vision[pos] 는 적 pos 로만 기록되므로 같은 pos 의 적 데이터로 검사됨
[L425]   if self.last_call_tick(+0x3f0) + tps < tick {
[L426]     lv = last_visible_ticks[pos]; if lv != 0 {
[L428]       if lv + tps*4 < tick {
[L429]         if last_hp_ratio[pos] > 59 {
[L430]           if last_visible_distance[pos] < 28900000000 (170000^2) {
[L431]             if mia_call_ticks[pos](+0x280) < lv + tps*4 {
[L433]               mia_call_ticks[pos] = tick; [L435] last_call_tick = tick
[L436]               self.chats.push(Chat::Mia(p.position, 0)) } } } } } }
     }
[L442] return
```
- logic_note: ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)

### `e4b070` → `d57b00` v2_apply_assign_commit (i=88 · 변경(r21 w11: obj_key 이중모드 · 헬퍼화 · 라인 래치 꼬리))
- 0.5.8 src: `game-ai\src\plan_legacy\handler.rs:518` · one_line: v2 이상: 오브젝트 사냥 플랜 참여 래치(v2_obj_part)와 라인 배정 래치(v2_assign)를 갱신하고, 조건 충족 시 plan/src 를 래치된 값으로 되돌려 커밋을 유지한다
- 0.6.0 판정: **심층(r21)** · 패치 요지: 필드 v2_obj_part +0x24a2/+0x24a3 · v2_assign +0x24a6/+0x24a7 · [L525] mf2_obj_key 이중모드(`3e4!=2 → 0x50+3e0` · `404!=2 → 0x60+400` · 둘 다 None → +0xdcd objective) · [L531] restore_safe d56570 3-arg · [L553] csrc 19 = `+0x24d6 ? (d753e0 && d750f0==Some(me)) : d75840(..,+0x24d5)` · [L554] csrc 17 = d756b0 · **신규 꼬리(+0x24d6)**: s∈{13,15} && PassiveLine && +0x120==0 → 래치 +0x24e3/+0x24e4(0xff→line · 무효→line · ≠line → plan=PassiveLine{latch}) · s==29 → +0x24e5=line · [L564] 그대로 · 헬퍼 5 본체 미대조
- RE 정본: `2026-09-17_r21_심층_w11_미발견12_마무리_update_on_dead_score클로저_best_jungle_goal_make_gank_battle_stake_원문.md` · 절: w11 §B9
- sig: `fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, &mut u8, &mut game_core::DebugFrameData)`
- consts: [{"value": 2, "src_line": 520, "meaning": "version < 2 면 아무것도 안 함(v2 게이트)", "kind": "임계", "ev": 4}, {"value": -11, "src_line": 524, "meaning": "(src-11) as u8 < 2 ⇔ src ∈ {11,12} = 오브젝트 사냥 소스(11=Serpen, 12=Morgard/Epic — L526·L533 에서 src==11 ↔ Serpen 으로 확정)", "kind": "계수", "ev": 4}, {"value": 11, "src_line": 526, "meaning": "src==11 → serpen_exists / SerpenHuntAndPoke(14); 아니면 morgard_exists / EpicHuntAndPoke(12)", "kind": "태그", "ev": 4}, {"value": 80, "src_line": 2314, "meaning": "mf2_obj_key: Serpen 이면 key = phase | 0x50", "kind": "산출값", "ev": 4}, {"value": 96, "src_line": 2314, "meaning": "
- 0.5.8 logic 전문:
```
if version < 2 { return }                                            // L520
s = *src
if s ∈ {11, 12} {                                                       // L524 오브젝트 사냥 소스
  key = mf2_obj_key(self.team_plan.objective)                          // L525 (handler.rs:2314)
        = match objective { Some(Serpen{phase,..}) => phase|0x50, Some(Morgard{phase,..}) => phase|0x60, _ => return }
  alive = if s==11 { serpen_exists(ctx) } else { morgard_exists(ctx) } // L526 (tutorial 기반, 위 constants)
  if plan.tag ∈ {12,13,14,15} (Epic/Serpen HuntAndPoke/Battle) {      // L527
    self.v2_obj_part = Some(key)                                       // L529
    return
  }
  if self.v2_obj_part == Some(key) && alive {                          // L530
    if v2_obj_restore_safe(version, rnd, player, data, debug) {        // L531
      drop(*plan); *plan = if s==11 { SerpenHuntAndPoke(기본) } else { EpicHuntAndPoke(기본) }  // L533
    }
    return
  }
  if !alive && self.v2_obj_part == Some(key) { self.v2_obj_part = None }  // L535~538
  return
}
// s ∉ {11,12}
if self.v2_obj_part.is_some() && self.v2_obj_part != mf2_obj_key(objective) {   // L545 (objective 가 Morgard/Serpen 아니면 키 없음 → 다름)
  self.v2_obj_part = None                                              // L546
}
if s ∉ {17,18,19} { return }                                            // L548
if let Some((csrc, cline)) = self.v2_assign {                          // L551
  ok = match csrc {                                                    // L552
    19 => mf2_cover_need_excluding_me(player, data, cline),            // L553 (2294~2309)
    17 => mf2_epic_line_valid(player, data, cline),                    // L554 (2323~2325)
    _  => false }
  if ok {                                                              // L557
    drop(*plan); *plan = PassiveLine(PassiveLinePlan::기본(line=cline)) // L558
    *src = csrc                                                        // L559
    return
  }
  self.v2_assign = None                                                // L562
}
match s { 17 | 19 =>                                                    // L564
  if let PassiveLine(p) = plan { self.v2_assign = Some((s, p.line)) }  // L565~566
  _ => {} }

--- mf2_cover_need_excluding_me(player,data,line) [handler.rs:2294~2309] ---
line_exists(ctx.tutorial, line) 아니면 false                            // L2294
me = player.info.position as usize                                    // L2297
n  = (0..5).filter(|p| p != me && blackboard[team].in_big_line(p, line)).count()   // L2298
return n == 0 && cache.<line>_lead[team] < 3 && game.get_player_champion_by_position(team, line→Position{Top0,Mid2,Bottom3}).is_none()   // L2309

--- mf2_epic_line_valid(player,data,line) [handler.rs:2323~2325] ---
line_exists(ctx.tutorial, line) 아니면 false                            // L2323
moba = game.get_game_mode().as_moba()  (tag==0 && ptr!=null 이어야 Some) // L2324
moba.remain_epic_time(team) = epic_minion_buff_time[team] != 0 이어야 함
return cache.tower(line, 1-team) (1차 or 2차 타워) .is_some()          // L2325
```
- logic_note: ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)

### `db8e60` → `fa9d60` LineGankerPlan::make_gank_battle (i=155 · 변경·소(r21 w11: gank_open_site/line/snap 기록 · sub_goal 폐기 동치))
- 0.5.8 src: `game-ai\src\plan_legacy\old\line_gank\ganker.rs:76` · one_line: 갱 대상에 대한 TryKill BattlePlan 을 만들어 BigPlan::Battle 로 반환 — v2+ 는 추격 가망 없음(open_chase_race_hopeless) 또는 첫 update 후 sub_goal 이 KitingBack/RunAway/End 면 None
- 0.6.0 판정: **심층(r21)** · 패치 요지: 소변경: site 인자 → `bp.+0x214 gank_open_site=site` · `bp.+0x205 gank_line=self.line(+0xab)` · **`bp.+0x40 gank_open_snap=Some{MIN,MIN, arrived_tick==0?-1:sat(tick−arrived), 0x0e,0xff,0xff}`**(소비처 미확인) · sub_goal 폐기 `{3,4,7}`→`≥2`(동치) · eeb5b0 = open_chase_race_hopeless(e0ad90 동일 · w1 「escape_possible」 라벨 = 이 함수) · debug 로그 2(sure_kill = site∈{2,6})
- RE 정본: `2026-09-17_r21_심층_w11_미발견12_마무리_update_on_dead_score클로저_best_jungle_goal_make_gank_battle_stake_원문.md` · 절: w11 §B2
- sig: `fn(&game_ai::plan_legacy::old::LineGankerPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_ai::plan_legacy::team_plan::TeamPlan, usize, usize, &mut game_core::DebugFrameData) -> std::option::Option<game_ai::plan_leg`
- consts: [{"value": 1, "src_line": 79, "meaning": "`version > 1` 분기(94368): v≤1 = 옛 경로(update 없이 즉시 Battle), v2+ = 가망 검사 + update + sub_goal 검사", "kind": "임계", "ev": 4}, {"value": 0, "src_line": 87, "meaning": "BattlePlanGoal::TryKill 태그 0 (94378·94448)", "kind": "태그", "ev": 4}, {"value": 7, "src_line": 88, "meaning": "entry_src = 7 (94382·94452, L88). 또한 BattleSubPlanGoal::End 태그 7 (switch 94468, L92~93)", "kind": "태그", "ev": 4}, {"value": 9, "src_line": 96, "meaning": "BigPlan::Battle 메모리 태그 9(니치, 논리 idx 7) (94394)", "kind": "센티널", "ev": 4}, {"value": -1, "src_line": 83, "meaning": "Option<BigPlan>::
- 0.5.8 logic 전문:
```
LineGankerPlan::make_gank_battle(&self /*IR 에서 제거*/, version, rnd, player, data, ps, team_plan, target, arg8, debug) -> Option<BigPlan>   [ganker.rs:76]
 if version > 1:                                                        (L79, 94368)
     me  = data.cache.player_champion[player.team][player.position]     (L80, 94404~94411)
     tgt = game.get_entity_by_id(target)                                (L81, 94412~94417, vtable+0x1f0)
     if me.is_some() && tgt.is_some() && open_chase_race_hopeless(version, data, player, me, tgt):   (L82, 94418~94427)
         return None                                                    (L83, 94430)   # 추격 경주 가망 없음
 plan = BattlePlan::new(version, BattlePlanGoal::TryKill(target, arg8), data, player)   (L87, 94379/94449)
 plan.entry_src = 7                                                     (L88, 94382/94452)
 if version > 1:                                                        (L89~91 — IR 은 L79 분기 결과로 블록 분리)
     plan.set_main_objective(team_plan.objective)   # battle.rs:350 인라인, Option<MainObjective> 3B 복사   (L90, 94453~94458)
     plan.update(version, rnd, player, data, ps, team_plan, debug)      (L92, 94459)
     if plan.sub_goal is KitingBack(3) | RunAway(4) | End(7):           (L92~93, switch 94465~94469)
         drop(plan) ; return None                                       (L93, 94472~94513: chats Vec<Chat> +0x68 · v54_reentry_ticks Vec<usize> +0x80 drop)
 return Some(BigPlan::Battle(plan))                                     (L96, 94392~94394)

요지: 갱 대상에 TryKill 전투플랜을 세우되, v2+ 에선 (a) 추격 경주가 가망 없으면 세우지 않고 (b) 한 틱 update 를 돌려 플랜이 즉시 후퇴/종료로 판정되면 폐기한다. `me`/`tgt` 가 None 이면 (a) 검사를 건너뛰고 플랜을 세운다(94421 → %49).
```
- logic_note: ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)

### `e49a50` → `d51a70` LegacyPlanHandler::update_on_dead (i=256 · 변경(r21 w11: 프롤로그 씬 종료+lock · version 0x5e→실제 · +0x24d6 assign))
- 0.5.8 src: `game-ai\src\plan_legacy\handler.rs:635` · one_line: 사망 중 플랜 갱신 — 비수동 플랜을 passive_plan 으로 교체(교체 전 Battle 이면 ff/mf 계측 스냅샷)하고, 기한 지난 수신/대기 채팅을 처리·정리한 뒤 team_plan·GoalData 를 갱신
- 0.6.0 판정: **심층(r21)** · 패치 요지: AgentVerHamster vt 0x143a86c20 슬롯 +0x88 f4d170 경유 · **0.5.8 은 version 인자 상수 0x5e(94) · 0.6.0 은 실제 version(+0x3608)**(명세에 「구=94 고정」 주석) · 신규 프롤로그: `cc5 && serpen.take_born!=2 → take_born=2; c68++; v4_serpen_lock=Some{tick, f1e3c0(..,5), why:2}` · morgard 동형(cc6/+0x404/f1e3c0(..,4)/+0xf0) · 비패시브 분기 Battle 판정 `tag<2`(니치) · `bp.+0xd0!=2 → self+0x1ef8..+0x1f10 = plan+0x1010..+0x1028`(pending tower_binary) · `+0x24d6 → d57b00 v2_apply_assign_commit` · Vec +0x1200/+0x1188/+0x1198/+0x11e8 · 나머지 동치
- RE 정본: `2026-09-17_r21_심층_w11_미발견12_마무리_update_on_dead_score클로저_best_jungle_goal_make_gank_battle_stake_원문.md` · 절: w11 §A1
- sig: `fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData)`
- consts: [{"value": 6, "src_line": 642, "meaning": "`llvm.assume(tag != 6)` — BigPlan 니치 태그 6 은 미사용(2..=17 중 6 = 이론상 DeathMatchBattle 자리) · 판정 상수 아님(m13.ll:10168, 10200) · 오라클 실행 확증(27차 배치D: 오라클 o27D_uod.exe 6168B 바이트 diff(기본/force=2 ForcePassive/force=9 Battle · _verify27/D/oracle/o27D_uod.log) — 기본(plan 태그 3 PassiveLine): 변경 오프셋 = 0x514(team_plan 콜리)·0x7c8/0x7d0/0x7d8(chats cap/ptr/len: chats_wait 1건 → push)·0x7f0(chats_wait len→0)·0x1800·0x1815 만, mf_swap/ff/plan 불변 / force=2(ForcePassive): +0x5e8~(plan 384B 교체 → 태그 3)·0x1610=24·0x1618=3000 / force=9(Battle): +0x15f8=3000·0x1600=16·0x1608~0x160f 8B 
- 0.5.8 logic 전문:
```
fn update_on_dead(&mut self, version, rnd, player, data, debug)   // handler.rs:635~679
cache = data.cache; ctx = data.context; game = cache.game
L636: self.sanitize_rule_scope(cache, ctx)                                   // m13.ll:10160 (fastcc · 계약)
L639: self.v2_egowave = 0                                                    // +0x1815
L640: self.v2_obj_part = None                                                // +0x1800 태그 0
L642: if !self.plan.is_passive() {   // is_passive(types.rs:254) = PassiveLine | SinglePlanLine | (PassiveJungle && !is_counter_jungle())
        //   is_counter_jungle(passive_jungle.rs:104) = plan.team(+0x638) != plan.player_team(+0x640)
        //   IR: idx = tag>1 ? tag-2 : 4 ; switch idx {1,2 → skip ; 5 → team==player_team ? skip : 교체 ; _ → 교체}   (m13.ll:10167~10187)
L643:   tick = game.tick(); self.ff_note_battle_swap(tick, 16)   // handler.rs:401: if let BigPlan::Battle(b) = &self.plan(태그 9) {
        //   ff_battle_exit = (16, b.entry_src, tick, b.exit_sub, b.ff_wave_obs_open);
        //   ff_battle_exit_latch = (b.ff_exit1_cls, b.ff_wave_open_hp, b.ff_wave_open_pct, b.ff_wave_fire_hp, b.ff_wave_fire_pct, b.ff_wave_open_danger, b.ff_wave_fire_danger, b.dive_abort_src) }
L644:   (new_plan, _u8) = self.passive_plan(version, rnd, player, data, debug)   // 392B sret · .1 은 버림 (m13.ll:10259~10261)
L645:   self.mf_note_swap(game.tick(), 24)   // handler.rs:410: mf_swap = (24, tick) — tick 을 다시 vtable 호출(m13.ll:10264)
L646:   self.plan = new_plan   // 옛 plan drop_glue(BigPlan) → memcpy 384B (unwind 시엔 새 plan 을 memcpy 후 전파 · m13.ll:10283~10286)
      }
L649: handled_chats: Vec<(usize,Position,Chat)> = Vec::new()
L650: for (tick, from, chat) in self.received_chats.iter() {                 // 원소 40B
L651:   if *tick > game.tick() { continue }    // 미래 채팅은 남김 (m13.ll:10368~10369)
L652:   handled_chats.push((*tick, *from, chat.clone()))   // grow_one 가능
      }
L655: self.received_chats.retain(|(t,_,_)| *t > game.tick())   // aux m06.ll:640 — 처리분(tick ≤ now) 제거
L656: for (tick, from, chat) in handled_chats {             // IntoIter · from==-1 이면 None 종료(니치 · 실질 불가)
L657:   if rule_scope::chat_allowed(ctx, &chat) {
L658:     misunderstood = self.take_misunderstood_received_chat(tick, from, chat.clone())   // fastcc → bool (계약)
L659:     self.handle_chat(version, rnd, player, data, from, chat, misunderstood, debug)   // (계약)
        } }
L661: drop(handled_chats IntoIter)
L662: self.misunderstood_received_chats.retain(|(t,_,_)| *t > game.tick())   // aux m06.ll:776
L665: self.team_plan.update(version, rnd, player, data, debug)               // (계약 · &mut TeamPlan 1064B)
L666: self.chats.extend(self.team_plan.chats.drain(..))                     // TeamPlan+0xc0 → self+0x1b8 비움, self.chats(+0x7c8) 에 붙임
L667: self.data.update(version, rnd, player, data, debug)                   // GoalData::update(&mut self.data @+0x0) (계약 · rnd readnone)
L668: self.sanitize_rule_scope(cache, ctx)
L670: for (tick, chat) in self.chats_wait.iter() {                          // 원소 32B
L671:   if *tick > game.tick() { continue }
L672:   if rule_scope::chat_allowed(ctx, chat) {
L673:     self.chats.push(chat.clone()) } }                                  // grow_one 가능
L677: self.chats_wait.retain(|(t,_)| *t > game.tick())                      // aux m06.ll:2458
L678: self.sanitize_rule_scope(cache, ctx)
L679: return

순서 고정(부작용 순): sanitize → 플래그 리셋 → [비수동] ff 스냅샷 → passive_plan → mf 스냅샷 → plan 교체 → received_chats 처리(handle_chat 순서 = 벡터 순서) → retain → misunderstood retain → TeamPlan::update → chats 병합 → GoalData::update → sanitize → chats_wait 처리 → retain → sanitize.
game.tick() 호출 횟수: 비수동이면 2 + 채팅 순회마다 1 + retain 술어마다 원소당 1 — 값은 같은 틱이라 동일(순수 조회).
```
- logic_note: ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)

### `e0b030` → `eebb40` resolve_fight_stake_roster (i=260 · 변경·소(r21 w11: bias 인자))
- 0.5.8 src: `game-ai\src\plan_legacy\old\fight_model.rs:645` · one_line: 합류 편성(roster)으로 교전 저울 3회 — 절대 판정 / 묶인 아군만 남긴 포기 판정 / 포기 net 기준 차분 판정 → 차분을 채택하되 라인이 갈리면 rescue_ally(최근접 묶인 아군) 표시
- 0.6.0 판정: **심층(r21)** · 패치 요지: 소변경: 인자 +player/+debug · 동일 bias → absolute/diff · abandon 0 · L648~669 동치
- RE 정본: `2026-09-17_r21_심층_w11_미발견12_마무리_update_on_dead_score클로저_best_jungle_goal_make_gank_battle_stake_원문.md` · 절: w11 §B6
- sig: `fn(usize, &game_core::OperationData, &game_core::Entity, &[(&game_core::Entity, i64, bool)], &[&game_core::Entity], i8, std::option::Option<&game_core::Entity>, usize) -> game_ai::plan_legacy::old::FightPrediction`
- consts: [{"value": 0, "src_line": 661, "meaning": "포기(abandon) 호출의 committed_dir=0(m10.ll:48127 i8 0) · arrivals 빈 슬라이스(len 0) · baseline 0 — 「묶인 아군만으로, 커밋 없이, 도착 없이」의 절대 판정. 같은 0 이 L654 절대 호출 baseline(48003 마지막 인자)과 L658 `remaining.len()==0`(48090) 에도", "kind": "태그", "ev": 4}, {"value": 1, "src_line": 666, "meaning": "rescue_ally = Some 태그(m10.ll:48362 phi) · bumpalo push additional=1(계측 아님·용량) · 오라클 실행 확증(27차 배치E: 오라클 실행 확인: 27E o260.exe(define hidden 심볼 link_name 직접 진입 · 케이스당 프로세스 1개) — R1/R2·R5·R6 전 케이스 diff.line_absolute == absolute.line(5/5) · R3 자기자신 bound 와 unbound 결과 58B 동일(id 필터) · R6(attack
- 0.5.8 logic 전문:
```
// fight_model.rs:645~647  pub(crate) fn resolve_fight_stake_roster(version, data:&OperationData, champ:&Entity, roster:&[(&Entity /*a*/, i64 /*arrival*/, bool /*bound*/)], near_enemies:&[&Entity], committed_dir:i8, tower:Option<&Entity>, judge_accuracy:usize) -> FightPrediction
// L648~649
bump = data.context.pool
allies:   bumpalo Vec<&Entity> = new_in(bump)
arrivals: bumpalo Vec<i64>     = new_in(bump)
// L650~652  (편성 전원 → 절대 판정 입력)
for (a, arrival, _bound) in roster { allies.push(a); arrivals.push(arrival) }
// L654  ① 절대 판정 — sret 에 직접
absolute = resolve_fight_full(version, data, champ, &allies, near_enemies, committed_dir, tower, judge_accuracy, &arrivals, baseline=0)
// L655~656  묶인 아군(자기 제외)만
remaining: bumpalo Vec<&Entity> = roster.iter().filter(|(a,_,bound)| *bound && a.id != champ.id).map(|(a,_,_)| *a).collect_in(bump)
// L658
if remaining.is_empty() { return absolute }          // 묶인 아군 없음 → 절대 판정 그대로(rescue_ally·line_absolute 는 콜리가 쓴 값)
// L661  ② 포기 판정 — 묶인 아군만, 커밋 없음(dir 0), 도착 없음, baseline 0
abandon = resolve_fight_full(version, data, champ, &remaining, near_enemies, 0, tower, judge_accuracy, &[], 0)
// L662~663  ③ 차분 판정 — ①과 같은 입력에 baseline = abandon.net_value
diff = resolve_fight_full(version, data, champ, &allies, near_enemies, committed_dir, tower, judge_accuracy, &arrivals, abandon.net_value)
// L664
diff.line_absolute = absolute.line
// L665~666  라인이 갈렸으면(차분이 절대를 뒤집음) 근거 아군 = champ 에서 가장 가까운 remaining
if diff.line != absolute.line {
   diff.rescue_ally = remaining.iter().min_by_key(|a| ((a.x-champ.x)²+(a.y-champ.y)² /*u64 절대차 제곱합*/, a.id)).map(|a| a.id)   // remaining.len>0 이므로 항상 Some
}
// L668~669
return diff   // (drops: remaining · arrivals · allies)

// 분기 순서: remaining 비었나(L658) → 라인 갈림(L665). version/tower/judge_accuracy/near_enemies/committed_dir 은 이 함수에서 분기 없음.
// rnd 인자 없음 · gen_range 0회. 세 콜리 호출 순서 = absolute → abandon → diff (모두 같은 data 로 — 콜리가 TLS/캐시를 쓰면 이 순서가 관측 순서).
```
- logic_note: ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)

### `ca6700` → `e13c10` get_small_action_score_closure (i=263 · 변경(r21 w11: v3 veto f71610 · 가중치표 재번호 동치))
- 0.5.8 src: `game-ai\src\plan_legacy\handler\auction.rs:177` · one_line: 각 후보 SmallActionPlay 에 SubPlan::score 기본점수를 매기고, |점수|≥6 이면 액션 분류별 judge_noise_ratio[cat]/1000 배율을 곱해 (점수, 액션) 벡터로 수집
- 0.6.0 판정: **심층(r21)** · 패치 요지: d6c2b0 d6d916 호출(Ghidra xref 누락) · `map(|c| (score(c),c)).collect()` · 가중치표 tag3..9 `[0,0,2,3,0,1,3]`(AroundHide 제거 · 동치) · **v3 veto** `f71610(version,player,data,&cand,env[0x218],env[8]) → score=-9_999_999`(`[TBEVAL]` · v<3 false · DM false · 대상 있는 play(Trace13/Attack14/Skill15/Skill2 16/Ult17) · 대상≠나 · 대상 hp≠0 · 나 undying==0 · !e74a50 · 본체 f717c0~f71f4c 미독) · env 7→8
- RE 정본: `2026-09-17_r21_심층_w11_미발견12_마무리_update_on_dead_score클로저_best_jungle_goal_make_gank_battle_stake_원문.md` · 절: w11 §A2
- sig: `{"tcx": null, "vis": null, "path": null, "mir": null, "ev": 5, "params": [{"i": 0, "name": "(sret)", "type": "bumpalo::collections::vec::Vec<(i64, SmallActionPlay), &Bump> (32B) %0", "role": "define 줄 속성: dead_on_unwind noalias writable sret([32 x i8]) captures(none) dereferenceable(32). 레이아웃 +0 buf`
- consts: [{"value": -1, "src_line": 177, "meaning": "0xFF = Option<SmallActionPlay> 니치 None 센티널(IntoIter::next 인라인 · bumpalo vec.rs:1161). 태그 바이트 +0xb1 == 255 면 루프 종료. 실제 원소의 태그는 0..=19 라 항상 거짓 — 실질 종료 조건은 ptr==end(49034·49310). m01.ll:49073 · 49264", "kind": "센티널", "ev": 4}, {"value": 10, "src_line": 179, "meaning": "llvm.assume(tag != 10): 니치 3+7=10 은 AroundPosition(untagged) 자리라 실제로 나올 수 없는 값. m01.ll:49106~49107", "kind": "센티널", "ev": 4}, {"value": -3, "src_line": 179, "meaning": "니치 복원 idx = tag − 3 (niche_start=3 · tcxdict --enum SmallActionPlay). m01.ll:49108 add nsw -3", "kind": "센티널", "ev": 3},
- 0.5.8 logic 전문:
```
// Vec<(i64, SmallActionPlay), &Bump>::from_iter_in(candidates.into_iter().map(closure$3), bump)   [bumpalo vec.rs:605~609 · 클로저 auction.rs:177~184]
// 부모(get_small_action · 열람 금지 · IR 만 참조)에서 env 구성: sub_plan=&self.sub_plan(+0x768) · version · parameter(지역 5384B) · rnd · player · data · debug · judge_noise_ratio(=self.judge_noise_ratio 88B 복사, auction.rs:175)
let mut v = Vec::new_in(bump);                                   // 48966~48979 {ptr=8, bump, cap=0, len=0}
let n = (iter.end − iter.ptr) / 184;                             // 48995~49001 (size_hint)
if n >= 1 { v.reserve_internal_or_panic(0, n, exact=true) }      // 49010~49025 ((bytes+183) < 367 ⟺ n==0 이면 생략)
while iter.ptr != iter.end {                                     // 49034 · 49310
    let c: SmallActionPlay = *iter.ptr; iter.ptr += 184;         // 49067 · 49079~49082 (177B + tag + 6B 복사)
    if c.tag == 0xFF { break }                                   // 49073 — Option 니치 None(실제 도달 불가)
178:    let base: i64 = sub_plan.score(version, parameter, rnd, player, data, &c, debug);   // 49092~49093 (디스패처 e388c0 · 계약만: (&SubPlan 72B, usize, &ScoreParameter 5384B, &mut StdRng, &PlayerState, &OperationData, &SmallActionPlay 184B, &mut DebugFrameData) -> i64)
179:    let cat: usize = c.get_action() as usize;                // small_action.rs:308~326 인라인 · SmallAction 판별자(game_core blackboard.rs:82 · Direct 0..10)
           // 니치 복원(49106~49110): idx = (tag > 2) ? tag−3 : 7(AroundPosition)
           // match idx(SmallActionPlay 논리 idx) → SmallAction(49111~49162):
           //   0 RunAway·1 Recall·5 AroundRunAway            → 0 RunAway        (L310)
           //   6 Positioning                                 → 1 Positioning    (L312)
           //   2 Around·3 AroundHide·10 LaneMinionPosition    → 2 Around         (L316)
           //   4 AroundRegion·7 AroundPosition·8 AroundPositionBush·9 AroundBush → 3 AroundPosition (L314)
           //   11 Trace → 4 (L321) · 12 Attack → 6 (L322) · 13 Skill → 7 (L323) · 14 Skill2 → 8 (L324) · 15 Ult → 9 (L325) · 16 Stop → 10 (L326)
           //   (5 Dodge 는 미매핑 · 그 외 idx = unreachable 49131)
180:    let score = if base.abs() < 6 { base }                   // 49164~49166 · llvm.abs(poison=false)
183:                else { base * judge_noise_ratio[cat] / 1000 };   // 49169~49173 · mul 순서 IR = ratio*base(표기 불가) · sdiv 절삭
184:    let t = (score, c);                                       // 49188~49192 → 192B 지역 %6
    if v.len == v.cap { v.reserve_internal_or_panic(v.len, 1, true) }   // 49201~49218
    v.ptr[v.len] = t (memcpy 192); v.len += 1;                   // 49292~49300
}
// 루프 종료 후 남은 원소 drop(49249~49287 · 정상 경로에선 ptr==end 라 0회) → sret = v (49324 memcpy 32B)

요지(한 줄 판정식): out[i] = ( |s|<6 ? s : s·W[cat(c_i)]/1000 , c_i )  where s = SubPlan::score(sub_plan, …, &c_i) · W = judge_noise_ratio[0..11] · cat = SmallActionPlay→SmallAction 판별자(위 표). 필터 없음 · 순서 보존 · len(out) = len(in).

&mut 표면: iter(by-value 소비 · ptr 전진) · sret 32B + 원소 192B×len (bump 할당) · rnd·debug 는 콜리 score 에만 전달(이 본문 직접 쓰기 0).
rnd: 이 본문 gen_range 사이트 0.
```
- logic_note: ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
