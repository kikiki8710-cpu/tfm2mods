---

### `88` v2_apply_assign_commit — v2 이상: 오브젝트 사냥 플랜 참여 래치(v2_obj_part)와 라인 배정 래치(v2_assign)를 갱신하고, 조건 충족 시 plan/src 를 래치된 값으로 되돌려 커밋을 유지한다

| 항목 | 값 |
|---|---|
| id | `handler__v2_apply_assign_commit` |
| 심볼 | `_RNvMNtNtCshdEBA0ozCnw_7game_ai11plan_legacy7handlerNtB2_17LegacyPlanHandler22v2_apply_assign_commit` |
| 소스 | `game-ai\src\plan_legacy\handler.rs:518` |
| IR | `m13.ll` 11530~12235행 |
| 경로·가시성 | `game_ai::plan_legacy::handler::LegacyPlanHandler::v2_apply_assign_commit` · **in:game_ai::plan_legacy::handler** |
| 계층 | 플랜 핸들러 |
| exe | `e4b070` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r10` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, &mut u8, &mut game_core::DebugFrameData)
```

<details><summary>인자 8개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | self | &mut LegacyPlanHandler (6168B) | 0x517·0x518(team_plan.objective) 읽기, 0x1800~0x1803(v2_obj_part·v2_assign) 읽기+쓰기. writes 전수 = writes 항목 참조 | 4 |
| 1 | 2 | version | usize | L520 `version < 2` 면 즉시 return(전 로직 v2 전용). v2_obj_restore_safe 에 전달 | 4 |
| 2 | 3 | rnd | &mut StdRng (320B, align16) | 본문 직접 사용 없음. v2_obj_restore_safe 에만 전달 | 4 |
| 3 | 4 | player | &PlayerState (2528B) | +0x930 info.team, +0x9c0 info.position 만 직접 읽음(mf2_cover_need_excluding_me 인라인). v2_obj_restore_safe 에 전달 | 4 |
| 4 | 5 | data | &OperationData (24B) | +0x0 cache, +0x8 context(+0x38 tutorial), +0x10 blackboard[2] | 4 |
| 5 | 6 | plan | &mut BigPlan (384B) | 태그 읽기(L527·L565) + 통째 덮어쓰기(L533·L558, drop_glue 후 재구성) | 4 |
| 6 | 7 | src | &mut u8 | 플랜 소스 코드. 읽기: 11/12 = 오브젝트 사냥 분기, 17/18/19 = 라인 배정 분기. 쓰기: L559 `*src = csrc` | 4 |
| 7 | 8 | debug | &mut DebugFrameData (224B) | 본문 직접 사용 없음. v2_obj_restore_safe 에만 전달 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
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

**`mem` 메모리 접근 26건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | LegacyPlanHandler | 0x517 | team_plan.objective | r | mf2_obj_key(handler.rs:2314) 인라인. 0=Morgard, 1=Serpen 만 키 생성. 그 외(None 포함) → L525 경로는 return, L545 경로는 v2_obj_part 클리어 | 4 | OK |  |
| 1 | LegacyPlanHandler | 0x518 | team_plan.objective@Some.0@Morgard.phase (=Serpen.phase) | r | ObjectPhase(0 None/1 Setup/2 Assemble/3 Hunt). key = phase \| (Serpen? 0x50 : 0x60) | 4 | OK |  |
| 2 | LegacyPlanHandler | 0x1800 | v2_obj_part@tag | r | Option<u8> 태그. 1=Some | 4 | OK |  |
| 3 | LegacyPlanHandler | 0x1801 | v2_obj_part@Some.0 | r | 래치된 key 와 비교(L530·L535·L545) | 4 | OK |  |
| 4 | LegacyPlanHandler | 0x1802 | v2_assign@Some.0.0 | r | csrc(u8). L552 match: 19 → cover 검사, 17 → epic-line 검사, 그 외 → 클리어 | 4 | OK |  |
| 5 | LegacyPlanHandler | 0x1803 | v2_assign@tag (=Some.0.1 cline LineType) | r | 니치: 0xff(-1)=None, 0/1/2 = Top/Mid/Bottom(=cline) | 4 | OK |  |
| 6 | BigPlan(plan) | 0x0 | @tag | r | L527: (tag & 28)==12 ⇔ tag∈{12,13,14,15} = Epic/SerpenHuntAndPoke/Battle. L565: tag==3 = PassiveLine. tag==6 은 assume(불가) | 4 | OK |  |
| 7 | BigPlan(plan) | 0x11e | @PassiveLine.0.line@tag | r | L566: v2_assign = Some((*src, plan.line)) | 4 | OK |  |
| 8 | OperationData | 0x0 | cache | r | &AbstractGameWithCache | 4 | OK |  |
| 9 | OperationData | 0x8 | context | r | &GameContext | 4 | OK |  |
| 10 | OperationData | 0x10 | blackboard | r | &[Blackboard;2] (744B stride). team 으로 인덱스(bounds<2, panic @handler.rs:2298) | 4 | OK |  |
| 11 | GameContext | 0x38 | tutorial | r | TutorialType. serpen_exists/morgard_exists/line_exists(rule_scope.rs:25·46·50) 가 runner.rs spawn_* 매치로 인라인 | 4 | OK |  |
| 12 | PlayerState | 0x930 | info.team | r | usize 0..1 | 4 | OK |  |
| 13 | PlayerState | 0x9c0 | info.position@tag | r | i32 Position. as_index(entity.rs:581) → 자기 자신 제외용 me | 4 | OK |  |
| 14 | AbstractGameWithCache | 0x0 | game.data_ptr / game.vtable_ptr | r | dyn AbstractGame 팻포인터. vtable+0x40 get_game_mode(as_moba, game.rs:231), vtable+0x1d8 get_player_champion_by_position | 4 | OK |  |
| 15 | AbstractGameWithCache | 0x180 | top_tower[] (line*32 + team*8 → top/mid/bottom_tower) | r | mf2_epic_line_valid L2325 `tower(line, 1-team)`: 0x180+line*0x20+(1-team)*8 — 상대팀 1차 타워 | 4 | OK |  |
| 16 | AbstractGameWithCache | 0x190 | top_tower2[] (line*32 + team*8 → top/mid/bottom_tower2) | r | 동 L2325 `.or(tower2)`: 0x190+line*0x20+(1-team)*8 — 상대팀 2차 타워. 둘 중 하나라도 Some 이면 유효 | 4 | OK |  |
| 17 | AbstractGameWithCache | 0x21c0 | top_lead[team] (Top: 0x21c0, Mid: 0x21d0, Bottom: 0x21e0; +team*8) | r | mf2_cover_need_excluding_me L2309: lead[line][team] < 3 | 4 | OK |  |
| 18 | MobaMode | 0x240 | epic_minion_buff_time[team] | r | remain_epic_time(game.rs:210) 인라인: [team] (bounds<2, panic @game.rs:210) != 0 이어야 유효 | 4 | OK |  |
| 19 | LegacyPlanHandler | 0x1800 | v2_obj_part@tag | w | L529: plan 이 Epic/Serpen Hunt 계열이면 Some(key) 로 래치. L538: !alive && 래치==key 면 None. L546: 래치 있는데 현재 objective 키와 다르면(또는 objective 가 Morgard/Serpen 아님) None | 4 | OK | 1 (L529) / 0 (L538·L546) |
| 20 | LegacyPlanHandler | 0x1801 | v2_obj_part@Some.0 | w | L529 에서만 씀 | 4 | OK | key = phase \| (src==11 ? 0x50 : 0x60)… 정확히는 mf2_obj_key(objective) = phase \| (Serpen 0x50 / Morgard 0x60) |
| 21 | LegacyPlanHandler | 0x1802 | v2_assign@Some.0.0 | w | L566: plan 이 PassiveLine 이고 src∈{17,19} 일 때 | 4 | OK | *src (17 또는 19) |
| 22 | LegacyPlanHandler | 0x1803 | v2_assign@tag (=cline) | w | L562: 래치된 (csrc,cline) 검사 실패(라인 미존재·커버 불필요·에픽 라인 무효·csrc∉{17,19}) 시 None | 4 | OK | plan.line (L566) / 0xff=None (L562) |
| 23 | BigPlan(plan) | 0x0 | @tag | w | drop_glue(BigPlan) 후 재구성. L533: +0x8=0, +0x10=8(dangling), +0x18..+0x22 = 0 (빈 Vec + 0 필드 = ~HuntAndPokePlan 기본값). L558: PassiveLinePlan 기본값(+0x8=0 v46_flee_entry None, +0x20/+0x38/+0x50 빈 Vec(cap0,ptr8,len0), +0x68.. 0), +0x11e = cline | 4 | OK | 14 SerpenHuntAndPoke (src==11) / 12 EpicHuntAndPoke (src==12) — L533 ; 3 PassiveLine — L558 |
| 24 | BigPlan(plan) | 0x11e | @PassiveLine.0.line@tag | w | L558 | 4 | OK | cline (v2_assign 의 라인) |
| 25 | u8(src) | 0x0 | *src | w | L559: 래치 복원 시 소스 코드도 래치값으로 되돌림 | 4 | OK | csrc (v2_assign.0 = 17 또는 19) |

**`consts` 상수 20건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 2 | 520 | 임계 | version < 2 면 아무것도 안 함(v2 게이트) | 4 |
| 1 | -11 | 524 | 계수 | (src-11) as u8 < 2 ⇔ src ∈ {11,12} = 오브젝트 사냥 소스(11=Serpen, 12=Morgard/Epic — L526·L533 에서 src==11 ↔ Serpen 으로 확정) | 4 |
| 2 | 11 | 526 | 태그 | src==11 → serpen_exists / SerpenHuntAndPoke(14); 아니면 morgard_exists / EpicHuntAndPoke(12) | 4 |
| 3 | 80 | 2314 | 산출값 | mf2_obj_key: Serpen 이면 key = phase \| 0x50 | 4 |
| 4 | 96 | 2314 | 산출값 | mf2_obj_key: Morgard 이면 key = phase \| 0x60 | 4 |
| 5 | 5 | 50 | 태그 | serpen_exists(rule_scope.rs:50)=spawn_serpen(runner.rs:267): tutorial ∈ {0 None,5 MidBottom,7 Line,8 Total} 이면 true (⚠본문의 `shl i8 %line, 5` 는 line*32 = tower 배열 stride 이며 이 상수와 무관) | 4 |
| 6 | -7 | 46 | 미상 | morgard_exists(rule_scope.rs:46)=spawn_epic(runner.rs:263): (tutorial-7) as u8 < 250 ⇔ tutorial ∉ {1..6} ⇔ ∈ {0 None,7 Line,8 Total} | 4 |
| 7 | -6 | 46 | 임계 | = 250 (u8). 위 morgard_exists 의 비교 상수 | 4 |
| 8 | 28 | 527 | 계수 | BigPlan 태그 마스크: (tag & 28)==12 ⇔ tag∈{12,13,14,15} = EpicHuntAndPoke/EpicHuntAndBattle/SerpenHuntAndPoke/SerpenHuntAndBattle (매치가 마스크로 접힘) | 4 |
| 9 | 12 | 527 | 태그 | 위 마스크 비교값 겸 L533 EpicHuntAndPoke 메모리태그 | 4 |
| 10 | 14 | 533 | 태그 | SerpenHuntAndPoke 메모리태그(src==11 일 때 select) | 4 |
| 11 | -17 | 548 | 계수 | (src-17) as u8 < 3 ⇔ src ∈ {17,18,19} = 라인 배정 소스 계열 | 4 |
| 12 | 3 | 548 | 태그 | 위 범위 폭(3개). 별도로 L565 BigPlan 태그 3 = PassiveLine, L2309 lead<3 도 같은 리터럴 | 4 |
| 13 | -1 | 551 | 센티널 | v2_assign 니치 None(0xff). L551 is_some 검사·L562 None 저장 | 4 |
| 14 | 19 | 552 | 태그 | csrc 19 → mf2_cover_need_excluding_me(L553). L564 도 src 19 허용 | 4 |
| 15 | 17 | 552 | 태그 | csrc 17 → mf2_epic_line_valid(L554). L564 도 src 17 허용(18 은 래치 갱신 안 함) | 4 |
| 16 | 7 | 25 | 태그 | line_exists(rule_scope.rs:25)=spawn_line_minion(runner.rs:275~291): Top⇔tutorial∈{0,2,7,8} · Mid⇔{0,4,5,7,8} · Bottom⇔{0,1,3,5,7,8} | 4 |
| 17 | 0 | 2309 | 태그 | 커버 필요 조건: 나를 제외한 아군 중 blackboard.in_big_line(pos,line) 인 수 == 0 | 4 |
| 18 | 472 | 2309 | 미상 | vtable 슬롯 0x1d8 = get_player_champion_by_position(team, pos) — 그 라인 담당 챔프가 None 이어야 커버 필요 | 4 |
| 19 | 6 | 527 | 센티널 | llvm.assume(tag != 6): BigPlan 니치 untagged(DeathMatchBattle) 자리 — 판정 아님 | 4 |

**`knobs` 조정점 3건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 커버 필요 판정의 라인 리드 상한 | handler.rs:2309 | 3 | 올리면 리드가 커도(이미 우세한 라인도) 커버 배정(src 19) 래치가 복원된다. 내리면 리드 라인 커버를 빨리 포기 | 4 | 기존 |
| 1 | 오브젝트 사냥 소스 집합 | handler.rs:524 | 11,12 | 이 값 외의 src 에선 v2_obj_part 래치가 세워지지 않는다(복원 불가) | 4 | 기존 |
| 2 | 라인 배정 래치 허용 소스 | handler.rs:548·564 | 17,19 (18 은 검사만·래치 갱신 없음) | 18 을 추가하면 src 18 PassiveLine 도 래치·복원 대상이 된다 | 4 | 기존 |

<details><summary>`callees` 피호출자 16건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | as_moba | game_core::GameMode::<'a>::as_moba | pub | fn(game_core::GameMode<'a/#0>) -> std::option::Option<&'a/#0 game_core::MobaMode> | game-core\src\simulation\game.rs:230 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 1 | drop | <game_core::prof::ProfTimer as std::ops::Drop>::drop | pub | fn(&mut game_core::prof::ProfTimer) | game-core\src\simulation\prof.rs:184 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 2 | get_game_mode | game_core::AbstractGame::get_game_mode | pub | fn(&Self/#0) -> game_core::GameMode | game-core\src\simulation.rs:78 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 3 | get_game_mode | <game_core::Game as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::Game) -> game_core::GameMode | game-core\src\simulation\game.rs:1707 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 4 | get_game_mode | <game_core::SingleLaneGame as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::SingleLaneGame) -> game_core::GameMode | game-core\src\simulation\game.rs:4248 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 5 | get_player_champion_by_position | game_core::AbstractGame::get_player_champion_by_position | pub | fn(&Self/#0, usize, game_core::Position) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:171 | True | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 6 | in_big_line | game_core::Blackboard::in_big_line | pub | fn(&game_core::Blackboard, usize, game_core::LineType) -> bool | game-core\src\simulation\game\blackboard.rs:137 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 7 | line_exists | game_ai::plan_legacy::rule_scope::line_exists | pub | fn(&game_core::GameContext, game_core::LineType) -> bool | game-ai\src\plan_legacy\rule_scope.rs:24 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 8 | mf2_cover_need_excluding_me | game_ai::plan_legacy::handler::mf2_cover_need_excluding_me | in:game_ai::plan_legacy::handler | fn(&game_core::PlayerState, &game_core::OperationData, game_core::LineType) -> bool | game-ai\src\plan_legacy\handler.rs:2293 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 9 | mf2_epic_line_valid | game_ai::plan_legacy::handler::mf2_epic_line_valid | in:game_ai::plan_legacy::handler | fn(&game_core::PlayerState, &game_core::OperationData, game_core::LineType) -> bool | game-ai\src\plan_legacy\handler.rs:2322 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 10 | mf2_obj_key | game_ai::plan_legacy::handler::mf2_obj_key | in:game_ai::plan_legacy::handler | fn(&std::option::Option<game_ai::plan_legacy::team_plan::MainObjective>) -> std::option::Option<u8> | game-ai\src\plan_legacy\handler.rs:2313 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 11 | morgard_exists | game_ai::plan_legacy::rule_scope::morgard_exists | pub | fn(&game_core::GameContext) -> bool | game-ai\src\plan_legacy\rule_scope.rs:45 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 12 | remain_epic_time | game_core::MobaMode::remain_epic_time | pub | fn(&game_core::MobaMode, usize) -> usize | game-core\src\simulation\game.rs:210 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 13 | serpen_exists | game_ai::plan_legacy::rule_scope::serpen_exists | pub | fn(&game_core::GameContext) -> bool | game-ai\src\plan_legacy\rule_scope.rs:49 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 14 | tower | game_core::AbstractGameWithCache::<'a, 'b>::tower | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, game_core::LineType, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:1822 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 15 | v2_obj_restore_safe | game_ai::plan_legacy::handler::LegacyPlanHandler::v2_obj_restore_safe | in:game_ai::plan_legacy::handler | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> bool | game-ai\src\plan_legacy\handler.rs:501 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
</details>

⚠**미매칭 1개**: `drop_glue<BigPlan>`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 3곳** (m13.ll:21521, m13.ll:21645, m13.ll:24300) · **형제 41개** (LegacyPlanHandler)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::plan_legacy::handler::LegacyPlanHandler as std::clone::Clone>::clone | pub | game-ai\src\plan_legacy\handler.rs:13 | True | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler) -> game_ai::plan_legacy::handler::LegacyPlanHandler |
| 1 | <game_ai::plan_legacy::handler::LegacyPlanHandler as std::fmt::Debug>::fmt | pub | game-ai\src\plan_legacy\handler.rs:13 | True | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 2 | game_ai::plan_legacy::handler::LegacyPlanHandler::new | pub | game-ai\src\plan_legacy\handler.rs:228 | False | fn(usize, &mut rand::rngs::std::StdRng, usize, game_core::Position) -> game_ai::plan_legacy::handler::LegacyPlanHandler |
| 3 | game_ai::plan_legacy::handler::LegacyPlanHandler::r2_fight_protected | in:game_ai | game-ai\src\plan_legacy\handler.rs:362 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &game_core::PlayerState, &game_core::OperationData) -> bool |
| 4 | game_ai::plan_legacy::handler::LegacyPlanHandler::ff_note_battle_swap | in:game_ai | game-ai\src\plan_legacy\handler.rs:400 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, u8, usize) |
| 5 | game_ai::plan_legacy::handler::LegacyPlanHandler::mf_note_swap | in:game_ai | game-ai\src\plan_legacy\handler.rs:409 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, u8, usize) |
| 6 | game_ai::plan_legacy::handler::LegacyPlanHandler::v3_fall_back_to_passive | pub | game-ai\src\plan_legacy\handler.rs:416 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 7 | game_ai::plan_legacy::handler::LegacyPlanHandler::eo_cover_picks | pub | game-ai\src\plan_legacy\handler.rs:440 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler) -> usize |
| 8 | game_ai::plan_legacy::handler::LegacyPlanHandler::eo_serpen_punish_issues | pub | game-ai\src\plan_legacy\handler.rs:445 | True | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler) -> usize |
| 9 | game_ai::plan_legacy::handler::LegacyPlanHandler::subplan_is_recall | pub | game-ai\src\plan_legacy\handler.rs:450 | True | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler) -> bool |
| 10 | game_ai::plan_legacy::handler::LegacyPlanHandler::team_objective_code | pub | game-ai\src\plan_legacy\handler.rs:456 | True | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler) -> u8 |
| 11 | game_ai::plan_legacy::handler::LegacyPlanHandler::update_v2_egowave | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:473 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) |
| 12 | game_ai::plan_legacy::handler::LegacyPlanHandler::v2_obj_restore_safe | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:501 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> bool |
| 13 | game_ai::plan_legacy::handler::LegacyPlanHandler::v2_apply_assign_commit | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:518 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, &mut u8, &mut game_core::DebugFrameData) |
| 14 | game_ai::plan_legacy::handler::LegacyPlanHandler::sanitize_rule_scope | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:571 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, &game_core::OperationData) |
| 15 | game_ai::plan_legacy::handler::LegacyPlanHandler::take_misunderstood_received_chat | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:588 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, game_core::Position, game_core::Chat) -> bool |
| 16 | game_ai::plan_legacy::handler::LegacyPlanHandler::enter_line_backfight_support | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:598 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> bool |
| 17 | game_ai::plan_legacy::handler::LegacyPlanHandler::update_on_dead | pub | game-ai\src\plan_legacy\handler.rs:635 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 18 | game_ai::plan_legacy::handler::LegacyPlanHandler::update | pub | game-ai\src\plan_legacy\handler.rs:685 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData, bool) |
| 19 | game_ai::plan_legacy::handler::LegacyPlanHandler::determine_transition_reason | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:1675 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, &str) -> game_core::PlanTransitionReason |
| 20 | game_ai::plan_legacy::handler::LegacyPlanHandler::force_plan_update | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:1709 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData, bool) |
| 21 | game_ai::plan_legacy::handler::LegacyPlanHandler::v3_assign_anchor | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:1773 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, &game_core::PlayerState, &game_core::OperationData) -> std::option::Option<(u64, u64)> |
| 22 | game_ai::plan_legacy::handler::LegacyPlanHandler::v3_depart_anchor | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:1806 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, &game_core::PlayerState, &game_core::OperationData) -> std::option::Option<(u64, u64)> |
| 23 | game_ai::plan_legacy::handler::LegacyPlanHandler::v3_plan_dest | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:1824 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::types::BigPlan) -> std::option::Option<(u64, u64)> |
| 24 | game_ai::plan_legacy::handler::LegacyPlanHandler::v3_repair_done | in:game_ai | game-ai\src\plan_legacy\handler.rs:1847 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bool |
| 25 | game_ai::plan_legacy::handler::LegacyPlanHandler::passive_plan | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:1855 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> (game_ai::plan_legacy::types::BigPlan, u8) |
| 26 | game_ai::plan_legacy::handler::auction::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::get_small_action | pub | game-ai\src\plan_legacy\handler\auction.rs:8 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &std::vec::Vec<(usize, game_core::SmallAction), std::alloc::Global>, &mut game_core::DebugFrameData) -> (game_ai::ScoreParameter, i64, game_ai::SmallActionPlay) |
| 27 | game_ai::plan_legacy::handler::chat::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::handle_chat | pub | game-ai\src\plan_legacy\handler\chat.rs:8 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, game_core::Position, game_core::Chat, bool, &mut game_core::DebugFrameData) |
| 28 | game_ai::plan_legacy::handler::chat::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::handle_chat_inner | in:game_ai | game-ai\src\plan_legacy\handler\chat.rs:41 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, game_core::Position, game_core::Chat, bool, &mut game_core::DebugFrameData) |
| 29 | game_ai::plan_legacy::handler::dive_episode::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::v50_track_dive_episode | in:game_ai | game-ai\src\plan_legacy\handler\dive_episode.rs:35 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &game_core::PlayerState, &game_core::OperationData) |
| 30 | game_ai::plan_legacy::handler::dive_episode::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::v50_fold_dive_episode | in:game_ai | game-ai\src\plan_legacy\handler\dive_episode.rs:125 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, usize, bool, u8) |
| 31 | game_ai::plan_legacy::handler::engage::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::v2_response_retreat_stance | in:game_ai | game-ai\src\plan_legacy\handler\engage.rs:13 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> game_ai::plan_legacy::old::BattleSubPlanGoal |
| 32 | game_ai::plan_legacy::handler::engage::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::try_engage | in:game_ai | game-ai\src\plan_legacy\handler\engage.rs:40 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, usize, &mut game_core::DebugFrameData) -> std::option::Option<game_ai::plan_legacy::old::BattlePlan> |
| 33 | game_ai::plan_legacy::handler::engage::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::try_engage_dive | in:game_ai | game-ai\src\plan_legacy\handler\engage.rs:109 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, usize, std::option::Option<game_core::TowerType>, &mut game_core::DebugFrameData) -> std::option::Option<game_ai::plan_legacy::old::BattlePlan> |
| 34 | game_ai::plan_legacy::handler::engage::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::handle_solokill | in:game_ai | game-ai\src\plan_legacy\handler\engage.rs:136 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 35 | game_ai::plan_legacy::handler::engage::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::handle_interact_battle | in:game_ai | game-ai\src\plan_legacy\handler\engage.rs:233 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 36 | game_ai::plan_legacy::handler::modes::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::update_single_lane | in:game_ai | game-ai\src\plan_legacy\handler\modes.rs:13 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 37 | game_ai::plan_legacy::handler::modes::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::update_deathmatch | in:game_ai | game-ai\src\plan_legacy\handler\modes.rs:56 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 38 | game_ai::plan_legacy::handler::modes::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::single_lane_initiate | in:game_ai | game-ai\src\plan_legacy\handler\modes.rs:196 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 39 | game_ai::plan_legacy::handler::modes::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::single_try_engage | in:game_ai | game-ai\src\plan_legacy\handler\modes.rs:241 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, usize, &mut game_core::DebugFrameData) -> std::option::Option<game_ai::plan_legacy::old::SinglePlanBattle> |
| 40 | game_ai::plan_legacy::handler::modes::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::single_handle_solokill | in:game_ai | game-ai\src\plan_legacy\handler\modes.rs:261 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |

**`open` 4건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | src 코드 11/12/17/18/19 의 이름(어느 enum/상수표인지) — 본문엔 리터럴만. 11↔Serpen·12↔Epic(Morgard) 대응은 L526/L533 에서 확정, 17/19 는 mf2_epic_line_valid/mf2_cover_need 로 각각 「에픽 라인」/「라인 커버」 소스로만 추정 | 5 |  |
| 1 | 미탐색 | v2_obj_restore_safe(m13.ll:11386) 내부 — 별도 함수(호출만), 이 명세 범위 밖 | 4 |  |
| 2 | 표기 불가 | L530 `&& alive` 와 L535 `!alive &&` 의 소스 문장 구조(중첩 if 인지 else-if 인지) — column 부재로 표기 불가. 동작(진리표)은 IR 로 확정 | 4 |  |
| 3 | 미탐색 | as_moba 의 {i64,ptr} 반환 중 i64 의 정확한 의미(GameMode 태그로 추정, 0 = Moba) — game.rs:231 본문 미독 | 5 |  |

<details><summary>`closed` 1건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

| # | 물음 | 닫은 근거 |
|---|---|---|
| 0 | L533 이 만드는 Epic/SerpenHuntAndPokePlan 의 필드 의미(+0x8=0,+0x10=dangling 8,+0x18..=0) — 32B 페이로드 중 Vec 1개+나머지 0 으로만 관측. 생성자 이름 미확정(인라인) | 본문에 해소 표기가 있다 |
</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

