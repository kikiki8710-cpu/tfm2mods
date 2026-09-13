---

### `82` epic_passive_plan — 에픽(모가드) 목표의 수동 국면에서 내 BigPlan(PassiveLine 라인 / EpicHuntAndPoke / None)을 고른다

| 항목 | 값 |
|---|---|
| id | `epic__epic_passive_plan` |
| 심볼 | `_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old4epic17epic_passive_plan` |
| 소스 | `game-ai\src\plan_legacy\old\epic.rs:317` |
| IR | `m09.ll` 62422~64312행 |
| 경로·가시성 | `game_ai::plan_legacy::old::epic_passive_plan` · **pub** |
| 계층 | 레거시 플랜 |
| exe | `de92d0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r10` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, game_ai::plan_legacy::team_plan::ObjectPhase, &game_ai::plan_legacy::team_plan::TeamPlan, std::option::Option<(u64, u64)>, &mut game_core::DebugFrameData) -> std::option::Option<game_ai::plan_legacy::types::BigPlan>
```

<details><summary>인자 9개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | Option<BigPlan>(384B) | tag +0x0: -1=None / 3=PassiveLine(payload +0x8 PassiveLinePlan 280B, line=+0x11e) / 12=EpicHuntAndPoke(payload +0x8 EpicHuntAndPokePlan 32B) | 4 |
| 1 | 1 | version | usize | 본문 분기 없음. v25_objective_splitter_should_join_contest(L340)에만 전달. ⚠can_near_enemies_range(L461)에는 poison 으로 전달(안 씀) | 4 |
| 2 | 2 | rnd | &mut StdRng | PlayerState::strategy(L327)·can_near_enemies_range(L461)에 전달 | 4 |
| 3 | 3 | player | &PlayerState(2528B) | info.team(+0x930)·info.position@tag(+0x9c0) | 4 |
| 4 | 4 | data | &OperationData(24B) | +0x0 cache / +0x8 context / +0x10 blackboard[2] | 4 |
| 5 | 5 | phase | ObjectPhase(i8) | L324 switch: 3 Hunt → EpicHuntAndPoke 즉시 / 1 Setup → 본문 / 그 외(0 None·2 Assemble) → None(L497) | 4 |
| 6 | 6 | team_plan | &TeamPlan | vision.last_visible_pos[p](+0x230+16p)·vision.last_checked_ticks[p](+0x2d0+8p)·obj_spawn.epic_camp_last_visible_tick(+0x80) | 4 |
| 7 | 7 | depart_anchor | Option<(u64,u64)>(24B, by-ref dead_on_return) | L399 unwrap_or(내 챔프 좌표) — 캠프까지 이동시간 계산의 출발점 | 4 |
| 8 | 8 | debug | &mut DebugFrameData | readnone — 본문에서 읽지도 쓰지도 않음. can_near_enemies_range 에는 poison 전달 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
ctx = data.context; if !spawn_epic(ctx.tutorial) (태그∈1..6) → return None                                   [L318-319]
champ = cache.player_champion[team][position].unwrap()                                                  [L322]
match phase { Hunt(3) → return EpicHuntAndPoke(default) [L325]; Setup(1) → 아래; _ → return None [L497] } [L324]
strategy = player.strategy(rnd, game)                                                                     [L327]
opposite_object_pressure = v23_enemy_object_pressure(player, data, Serpen)                                [L329]
if opposite_object_pressure { if let Some(line) = v23_objective_setup_pressure_line(player,data,&[Top,Mid]) → return PassiveLine{line} } [L330-332]
match strategy.object_buildup {                                                                            [L337]
  Split(pos) [L339]: if pos == my position { if !v25_objective_splitter_should_join_contest(version,player,data,Morgard) → return PassiveLine{Bottom} [L340-341] } → Gather 로 진행
  Flexible [L347]: camp = map.camp_pos(Morgard, team==0)
    can_reach = Σ_{p∈0..5, 적 챔프 e=player_champion[1-team][p] 존재} [                                    [L350-359]
        last_pos = team_plan.vision.last_visible_pos[p]; d = distance(last_pos, camp).sat_sub(150000)      [L352-353]
        can_move = (game.tick() − vision.last_checked_ticks[p]).sat_sub * e.move_speed                    [L354-355]
        e.hp*100/e.max_hp > 49 && can_move >= d && !blackboard[1-team].is_recent_visible(game, player, e) ] [L357]
    if can_reach < 2 [L362]: if blackboard[team].bottom_minion_state.minion_count < -3 && count{p≠me && blackboard[team].in_big_line(p,Bottom)}==0 → return PassiveLine{Bottom} [L361-366]
    if can_reach <= 2 (L362 count<2 에서 바텀 조건 실패한 경우 + L370 count==2) && dist²(champ, camp) < 320000² [L370] && is_enemy_side(ctx, team, champ.x, champ.y) [L371 = is_blue_side(x−y+height > width) XOR team==0]:
        top=bb[team].top_minion_state.minion_count; mid=bb[team].mid_minion_state.minion_count             [L373-374]
        if top_lead[team] < 3 { if mid_lead[team] < 3 { return PassiveLine{ if top<mid Top else Mid } [L377-380] } else return PassiveLine{Top} [L383] }
        else if mid_lead[team] < 3 → return PassiveLine{Mid} [L385]  (그 외 → Gather 로 진행) [L376-386]
    그 외 → Gather 로 진행
  Gather (및 위 폴백) [L394]: camp = map.camp_pos(Morgard, team==0)
    moba = game.get_game_mode().as_moba().unwrap(); remain = moba.jungle_runner.epic.next_respawn_tick.sat_sub(tick) [L397]
    pos = depart_anchor.unwrap_or((champ.x,champ.y)); travel = distance(pos,camp) / champ.move_speed (0이면 패닉) [L399-402]
    if remain > tps*2 + travel → return None                                                               [L403-404]
    if tick − team_plan.obj_spawn.epic_camp_last_visible_tick > tps*3 [L407]:
        near = team_plan.can_near_enemies_range(rnd, player, data, camp, 150000)                            [L461]
        if near.len() > 2 → return EpicHuntAndPoke                                                          [L462-463]
        if top_lead>2 { if mid_lead>2 → EpicHuntAndPoke [L470-471] else → PassiveLine{Mid} [L490] }
        else if mid_lead<3 → PassiveLine{ if top<mid Top else Mid } [L478-485] else → PassiveLine{Top} [L488]
    else:
        nearest = player_champion[1-team].filter(|e| bb[1-team].is_recent_visible(game,player,e)).min_by_key(dist²(e,camp)) [L408-409, aux m12]
        if top_lead>2 && mid_lead>2 → return EpicHuntAndPoke                                               [L412-413]
        if let Some(e)=nearest [L420]: if dist²(e,camp) < 150000² → return EpicHuntAndPoke [L421-422]
            top/mid = bb[team] minion_count [L424-425]; if top_lead<3 { if mid_lead<3 → PassiveLine{top<mid?Top:Mid} [L428-431] else PassiveLine{Top} [L434] } else if mid_lead<3 → PassiveLine{Mid} [L436] else → None [L439]
        else (적 시야 없음) [L442-457]: 같은 lead/minion 표 → Top/Mid(L447-454) 또는 None [L457]
}
```

**`mem` 메모리 접근 32건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | OperationData | 0x8 | context | r | &GameContext | 4 | OK |  |
| 1 | OperationData | 0x0 | cache | r | &AbstractGameWithCache (player_champion·top_lead·mid_lead) | 4 | OK |  |
| 2 | OperationData | 0x10 | blackboard | r | &[Blackboard;2] — [team]=아군판(minion_state·in_big_line), [1-team]=적판(is_recent_visible) | 4 | OK |  |
| 3 | GameContext | 0x38 | tutorial | r | TutorialType 태그. spawn_epic(runner.rs:263)→morgard_exists(rule_scope.rs:46) 인라인: 태그∈{1..6}이면 에픽 없음 | 4 | OK |  |
| 4 | GameContext | 0x20 | map | r | &MapDef → camp_pos(Morgard, team==0) | 4 | OK |  |
| 5 | GameContext | 0x8 | setting | r | &GameSetting | 4 | OK |  |
| 6 | GameSetting | 0x12f8 | tick_per_second | r | L403 tps*2 / L407 tps*3 | 4 | OK |  |
| 7 | GameSetting | 0x12b8 | width | r | is_blue_side(map_regions.rs:7) 인라인 | 4 | OK |  |
| 8 | GameSetting | 0x12c0 | height | r | is_blue_side 인라인 | 4 | OK |  |
| 9 | PlayerState | 0x930 | info.team | r | 0/1. 적팀 = 1-team | 4 | OK |  |
| 10 | PlayerState | 0x9c0 | info.position@tag | r | Position 0 Top/1 Jungle/2 Mid/3 Bottom/4 Support — champ 인덱스·Split 비교·closure#0 자기 제외 | 4 | OK |  |
| 11 | AbstractGameWithCache | 0x1e0 | player_champion | r | [2][5] Option<&Entity>, stride 40B/team 8B/pos | 4 | OK |  |
| 12 | AbstractGameWithCache | 0x21c0 | top_lead[team] | r | L376/412/427/445/470/481 — <3 / >2 임계로 라인 선택·EpicHuntAndPoke 결정 | 4 | OK |  |
| 13 | AbstractGameWithCache | 0x21d0 | mid_lead[team] | r | 위와 짝 | 4 | OK |  |
| 14 | Entity | 0x660 | x | r | 챔프 좌표 | 4 | OK |  |
| 15 | Entity | 0x668 | y | r |  | 4 | OK |  |
| 16 | Entity | 0x640 | stat_cached.move_speed | r | L354 적 can_move / L401 내 이동시간 | 4 | OK |  |
| 17 | Entity | 0x628 | stat_cached.hp | r | 최대 HP (0이면 div_by_zero 패닉) | 4 | OK |  |
| 18 | Entity | 0x670 | hp | r | 현재 HP — hp*100/max > 49 | 4 | OK |  |
| 19 | TeamPlan | 0x230 | vision.last_visible_pos[i] | r | 적 p 의 마지막 관측 좌표 (u64,u64), +16p (L352) | 4 | OK |  |
| 20 | TeamPlan | 0x2d0 | vision.last_checked_ticks[i] | r | 적 p 마지막 확인 틱, +8p (L355) | 4 | OK |  |
| 21 | TeamPlan | 0x80 | obj_spawn.epic_camp_last_visible_tick | r | L407 캠프 마지막 관측 틱 | 4 | OK |  |
| 22 | Blackboard | 0x20 | top_minion_state.minion_count | r | i32, L373/424/442/478 — top<mid 비교로 Top/Mid 선택 | 4 | OK |  |
| 23 | Blackboard | 0x48 | mid_minion_state.minion_count | r | i32 | 4 | OK |  |
| 24 | Blackboard | 0x70 | bottom_minion_state.minion_count | r | i32, L361 < -3 (바텀 밀림) | 4 | OK |  |
| 25 | Strategy | 0x0 | object_buildup@tag | r | ObjectBuildupStrategy: 태그 5 Gather / 6 Flexible / 그외(=Position 값) Split(position) — L337 switch | 4 | OK |  |
| 26 | MobaMode | 0x1b0 | jungle_runner.epic.next_respawn_tick | r | L397 game.get_game_mode().as_moba().unwrap() | 4 | OK |  |
| 27 | AbstractGame(vtable) | 0x28 | tick | r | divtable AbstractGame 0x28 | 3 | 오귀속(사전은 다른 필드를 준다) |  |
| 28 | AbstractGame(vtable) | 0x40 | get_game_mode | r | divtable AbstractGame 0x40 → GameMode{tag 0=Moba, +8 &MobaMode} | 3 | 오귀속(사전은 다른 필드를 준다) |  |
| 29 | Option<BigPlan>(sret) | 0x0 | tag | w | L319·L497·L404·L439·L457 None / L325·L413·L422·L463·L471 EpicHuntAndPoke / 그 외 PassiveLine | 4 | 확인불가(tcx 사전에 타입 없음) | -1 None / 3 PassiveLine / 12 EpicHuntAndPoke |
| 30 | Option<BigPlan>(sret) | 0x11e | line | w | = payload(+0x8) 내 PassiveLinePlan.line(+0x116). 나머지 필드 = 0/빈 Vec(dangling ptr 8) 초기화(PassiveLinePlan::new 인라인) | 4 | 확인불가(tcx 사전에 타입 없음) | 0 Top / 1 Mid / 2 Bottom / L332는 setup_pressure_line 반환값 |
| 31 | EpicHuntAndPokePlan(payload +0x8) | 0x0 | v46_flee_threats·focus_epic_only·vision_only·v46_flee | w | memset 11B — 전부 기본값 | 4 | OK | 빈 Vec(cap0, ptr 8, len0)·false·false·false |

**`consts` 상수 27건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev | folded_from |
|---|---|---|---|---|---|---|
| 0 | 6 | 318 | 임계 | spawn_epic 인라인: (tutorial_tag-1) <u 6 ⇔ 태그∈{1 First..6 JungleOnly} → 에픽 없음 → None. 통과 = {0 None,7 Line,8 Total} | 4 |  |
| 1 | -1 | 319 | 센티널 | Option<BigPlan>::None 니치 태그(store i64 -1). L331에선 Option<LineType>::None(icmp eq i8 -1) | 4 |  |
| 2 | 3 | 324 | 임계 | ObjectPhase::Hunt(3) — phase switch → EpicHuntAndPoke 즉시 반환(L325) | 4 |  |
| 3 | 1 | 324 | 태그 | ObjectPhase::Setup(1) — 본문 진입. 그 외 phase → None(L497) (shl 시프트량 아님 — 열거형 태그 리터럴) | 4 |  |
| 4 | 12 | 325 | 센티널 | BigPlan::EpicHuntAndPoke 메모리태그(idx10, 니치+2) | 4 |  |
| 5 | 5 | 329 | 산출값 | JungleType::Serpen(5) — v23_enemy_object_pressure(player,data,Serpen): 반대편 오브젝트(세르펜) 압박 여부 = opposite_object_pressure | 4 |  |
| 6 | 2 | 331 | 길이 | v23_objective_setup_pressure_line 에 넘기는 슬라이스 [Top(0),Mid(1)] 길이 2 (anon.190 = 00 01) | 4 |  |
| 7 | 3 | 332 | 센티널 | BigPlan::PassiveLine 메모리태그(idx1, 니치+2) | 4 |  |
| 8 | 5 | 337 | 센티널 | ObjectBuildupStrategy 니치 시작: 태그>4 이면 (태그-5) → 0 Gather / 1 Flexible; 아니면 Split(payload=Position) | 4 |  |
| 9 | 4 | 337 | 임계 | 위 switch 의 `icmp ugt tag, 4` — Split 페이로드(Position 0..4)와 니치 태그(5,6) 구분 | 4 |  |
| 10 | 4 | 340 | 임계 | JungleType::Morgard(4) — v25_objective_splitter_should_join_contest / camp_pos 인자 | 4 |  |
| 11 | 150000 | 353 | 계수 | 적이 캠프에 '도달'한 것으로 볼 반경. d = distance(last_pos, camp).saturating_sub(150000). L461 can_near_enemies_range 반경도 150000 | 4 |  |
| 12 | 100 | 357 | 계수 | hp*100/max_hp — HP 퍼센트 | 4 |  |
| 13 | 49 | 357 | 임계 | HP% > 49 ⇔ 50% 이상인 적만 '올 수 있는 적'으로 계수 (L357). check 쪽 동일 | 4 |  |
| 14 | 2 | 362 | 임계 | count(can_reach 적) < 2 → 바텀 분기 검사(L361~366, 실패 시 L370 으로) ; L370 `== 2` 와 합쳐 count ≤ 2 이면 캠프 근접 검사 ; > 2 → Gather 로 폴백 | 4 |  |
| 15 | -3 | 365 | 임계 | blackboard[team].bottom_minion_state.minion_count < -3 (바텀 웨이브 3+ 밀림) && 바텀 아군 0 → PassiveLine Bottom | 4 |  |
| 16 | 2 | 366 | 임계 | LineType::Bottom(2) — L341(Split 불참)·L366 PassiveLine.line | 4 |  |
| 17 | 102400000001 | 370 | 임계 | 320000^2+1 — 내 챔프↔에픽 캠프 거리² <u 이면(=320000 이내) 캠프 근접 분기(L371) | 4 |  |
| 18 | 3 | 376 | 임계 | top_lead[team] < 3 / mid_lead[team] < 3 임계 (L376·L427·L445·L481) — 3 이상이면 그 라인은 '앞서 있음' | 4 |  |
| 19 | 1 | 380 | 태그 | LineType::Mid(1) — PassiveLine.line (L380·385·431·436·449·454·485·490). Top(0)은 memset 으로 0 (shl 시프트량 아님 — 열거형 태그 리터럴) | 4 |  |
| 20 | 0 | 397 | 태그 | GameMode 태그 0 = Moba — as_moba(game.rs:231) 인라인, 아니면 unwrap_failed(L397) | 4 |  |
| 21 | 1 | 403 | 태그 | tps*2 — `shl i64 %tps, 1` 로 접힘. (next_respawn_tick − tick) > tps*2 + dist/move_speed 이면 None(L404): 스폰까지 여유가 이동시간+2초보다 길다 | 4 | 2 |
| 22 | 3 | 407 | 임계 | tps*3 (mul 3) — (tick − epic_camp_last_visible_tick) > 3초면 캠프 시야 상실 분기(L461~) / 아니면 최근가시 적 분기(L408~) | 4 |  |
| 23 | 2 | 412 | 임계 | top_lead > 2 && mid_lead > 2 → EpicHuntAndPoke (L412·L470·L471) | 4 |  |
| 24 | 22500000001 | 421 | 임계 | 150000^2+1 — 최근가시 최근접 적↔캠프 거리² <u 이면(150000 이내) EpicHuntAndPoke(L422) | 4 |  |
| 25 | 2 | 462 | 임계 | can_near_enemies_range(캠프,150000).len() > 2 → EpicHuntAndPoke(L463) | 4 |  |
| 26 | 2 | 364 | 임계 | [aux closure#0] LineType::Bottom(2) — blackboard[team].in_big_line(p, Bottom) 로 바텀 아군 계수 | 4 |  |

**`knobs` 조정점 9건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 적 '도달' 반경(Flexible can_reach·can_near_enemies_range) | epic.rs:353 / :461 | 150000 | 올리면 더 먼 적도 캠프에 올 수 있는 적으로 세어 라인 배정이 쉽게 막히고(can_reach↑) 시야상실 분기에선 EpicHuntAndPoke 가 잘 나온다 | 4 | 기존 |
| 1 | 계수 대상 적/최근접 적 HP 하한 | epic.rs:357 | 49 | 내리면 저체력 적도 위협으로 계수 | 4 | 기존 |
| 2 | 바텀 밀림 임계 | epic.rs:365 | -3 | 올리면(예 -1) 바텀이 조금만 밀려도 바텀 커버(PassiveLine Bottom)로 빠진다 | 4 | 기존 |
| 3 | 캠프 근접 판정 반경 | epic.rs:370 | 102400000001 | 320000². 올리면 더 멀리서도 '캠프 근처' 라인 배정 분기를 탄다 | 4 | 기존 |
| 4 | 라인 '앞섬' 임계 top_lead/mid_lead | epic.rs:376,412,427,445,470,481 | 3 | 낮추면 라인 압박이 약해도 EpicHuntAndPoke(둘 다 >2)로 넘어간다 | 4 | 기존 |
| 5 | 스폰 대기 허용 = tps*2 + 이동시간 | epic.rs:403 | 2 | folded(shl 1). 올리면 스폰 훨씬 전부터 캠프 쪽 계획을 세운다(None 덜 반환) | 4 | 기존 |
| 6 | 캠프 시야 상실 판정 | epic.rs:407 | 3 | tps*3. 올리면 시야를 잃어도 '최근가시 적' 분기를 오래 유지 | 4 | 기존 |
| 7 | 최근접 적 → 즉시 HuntAndPoke 반경 | epic.rs:421 | 22500000001 | 150000². 올리면 적이 더 멀어도 에픽 국면으로 전환 | 4 | 기존 |
| 8 | 시야상실 분기 적 수 임계 | epic.rs:462 | 2 | len>2. 내리면 적 2명만 근처여도 EpicHuntAndPoke | 4 | 기존 |

<details><summary>`callees` 피호출자 19건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | as_moba | game_core::GameMode::<'a>::as_moba | pub | fn(game_core::GameMode<'a/#0>) -> std::option::Option<&'a/#0 game_core::MobaMode> | game-core\src\simulation\game.rs:230 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 1 | camp_pos | game_core::MapDef::camp_pos | pub | fn(&game_core::MapDef, game_core::JungleType, bool) -> (u64, u64) | game-core\src\simulation\map_def.rs:207 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 2 | can_near_enemies_range | game_ai::plan_legacy::team_plan::TeamPlan::can_near_enemies_range | pub | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, u64, u64, u64, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< &game_core::Entity> | game-ai\src\plan_legacy\team_plan.rs:483 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 3 | distance | game_core::utils::distance | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:12 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | get_game_mode | game_core::AbstractGame::get_game_mode | pub | fn(&Self/#0) -> game_core::GameMode | game-core\src\simulation.rs:78 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 5 | get_game_mode | <game_core::Game as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::Game) -> game_core::GameMode | game-core\src\simulation\game.rs:1707 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 6 | get_game_mode | <game_core::SingleLaneGame as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::SingleLaneGame) -> game_core::GameMode | game-core\src\simulation\game.rs:4248 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 7 | in_big_line | game_core::Blackboard::in_big_line | pub | fn(&game_core::Blackboard, usize, game_core::LineType) -> bool | game-core\src\simulation\game\blackboard.rs:137 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 8 | is_blue_side | game_core::is_blue_side | pub | fn(&game_core::GameContext, u64, u64) -> bool | game-core\src\simulation\map_regions.rs:6 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 9 | is_enemy_side | game_core::is_enemy_side | pub | fn(&game_core::GameContext, usize, u64, u64) -> bool | game-core\src\simulation\map_regions.rs:57 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 10 | is_recent_visible | game_core::Blackboard::is_recent_visible | pub | fn(&game_core::Blackboard, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\game\blackboard.rs:346 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 11 | spawn_epic | game_core::TutorialType::spawn_epic | pub | fn(&game_core::TutorialType) -> bool | game-core\src\simulation\game\runner.rs:262 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 12 | strategy | game_core::PlayerState::strategy | pub | fn(&game_core::PlayerState, &mut rand::rngs::std::StdRng, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + ) -> game_core::Strategy | game-core\src\simulation\state\player.rs:1576 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 13 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 14 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 15 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 16 | v23_enemy_object_pressure | game_ai::plan_legacy::team_plan::v23_enemy_object_pressure | pub | fn(&game_core::PlayerState, &game_core::OperationData, game_core::JungleType) -> bool | game-ai\src\plan_legacy\team_plan\objective_helpers.rs:178 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 17 | v23_objective_setup_pressure_line | game_ai::plan_legacy::team_plan::v23_objective_setup_pressure_line | pub | fn(&game_core::PlayerState, &game_core::OperationData, &[game_core::LineType]) -> std::option::Option<game_core::LineType> | game-ai\src\plan_legacy\team_plan\objective_helpers.rs:73 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 18 | v25_objective_splitter_should_join_contest | game_ai::plan_legacy::team_plan::v25_objective_splitter_should_join_contest | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, game_core::JungleType) -> bool | game-ai\src\plan_legacy\team_plan\objective_helpers.rs:165 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
</details>

⚠**미매칭 3개**: `else`, `move_speed`, `sat_sub`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m13.ll:7769) · **형제 0개** 

**`open` 8건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | top_lead/mid_lead(AbstractGameWithCache+0x21c0/+0x21d0)의 정확한 의미(타워 수? 웨이브 리드?) — _docs 주석 0건, _gcbc 갱신처 미탐색. 본 명세는 '3 이상 = 앞섬'으로만 적음(IR 임계 사실만) | 4 |  |
| 1 | 미탐색 | L371 is_enemy_side 극성: IR = (x−y+height > width) XOR (team==0) 가 true 일 때 라인 배정 진행. is_blue_side 의 좌표계(y 축 방향) 해석은 미확인 — 식 자체는 IR 정본 | 3 |  |
| 2 | 미탐색 | L337 switch 의 `select 2` 는 Split 페이로드(Position)를 태그 자리에서 읽는 니치 접힘 — Split(pos) 의 pos 가 L339 에서 `%28(내 position) == %50(raw tag)` 로 비교됨. 소스 표현식(`Split(p) if p == my_pos`)은 추정 | 5 |  |
| 3 | 미탐색 | can_near_enemies_range 에 version=poison·debug=poison 전달 — 피호출이 그 인자를 안 읽는다는 뜻(정정 아닌 관측). 반환 Vec 32B 의 +24 = len 만 사용 후 drop | 4 |  |
| 4 | 미탐색 | `i8 5`(Serpen) 를 v23_enemy_object_pressure 에 넘기는 이유(반대 오브젝트 압박)는 인자값과 변수명 opposite_object_pressure 로부터의 추론 | 4 |  |
| 5 | 미탐색 | is_recent_visible 내부(120틱 = 2초 유예, _gcbc g07.ll:157005)는 담당 범위 밖 — knobs 에 미등재 | 4 |  |
| 6 | 미탐색 | exe 대조: fnprobe 0xde92d0 — 패닉 Location epic.rs:322/357/397/402·aux 0xe332e0 호출·상수 0x249f0(150000) 일치. 제곱 임계(102400000001·22500000001)는 fnprobe consts 목록에 안 찍힘(movabs 미수집으로 추정) — 미대조 | 5 |  |
| 7 | 미탐색 | L332 PassiveLine.line 값은 v23_objective_setup_pressure_line 반환(Top/Mid 중 하나) — 그 함수 내부는 다른 배치 소관 | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

