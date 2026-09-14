---

### `141` SmallActionPlay::get_input — 소액션 → 엔진 입력(Option<Input>) 디스패처 — 적 우물 위험 시 탈출 Move 선출력, 종류별 get_input 위임, Move 결과의 우물 회피 보정, v2+ 에서 '제자리 Move' 를 목표점 Move 로 승격

| 항목 | 값 |
|---|---|
| id | `SmallActionPlay__get_input` |
| 심볼 | `_RNvMNtCshdEBA0ozCnw_7game_ai12small_actionNtB2_15SmallActionPlay9get_input` |
| 소스 | `game-ai\src\small_action.rs:157` |
| IR | `m11.ll` 42814~43329행 |
| 경로·가시성 | `game_ai::SmallActionPlay::get_input` · **pub** |
| 계층 | 기타 |
| exe | `e23fa0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r14` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&mut game_ai::SmallActionPlay, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &mut game_core::DebugFrameData) -> std::option::Option<game_core::Input>
```

<details><summary>인자 8개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | Option<Input>(32B) | +0 태그 i64: -1 None(니치) / 0 Move(+8 x,+16 y) / 1 Return / 2..5 Attack·Skill·Skill2·Ult(+8 InputTarget 24B) | 4 |
| 1 | 1 | self | &mut SmallActionPlay(184B) | IR 속성: noalias·align 8·dereferenceable(184) — readonly 없음 = &mut. 본문 직접 store 0건(태그 +0xb1 · goal 필드 load 만) — 가변성은 종류별 get_input 콜리에 그대로 전달하기 위한 것 | 4 |
| 2 | 2 | version | usize | 분기: version > 1 (L168 · L212) — v2+ 에서 탈출점 도달 판정·제자리 Move 승격 활성 | 4 |
| 3 | 3 | rnd | &mut StdRng(320B) | 본문 미사용 — 종류별 get_input 에 전달 | 4 |
| 4 | 4 | player | &PlayerState(2528B) | +0x930 team · +0x9c0 position 태그 · path_finder 콜리 인자 | 4 |
| 5 | 5 | data | &OperationData(24B) | +0x0 cache(player_champion) · +0x8 context(setting/map) | 4 |
| 6 | 6 | positioning_score | &PositioningScoreData(2760B) | 본문 미사용 — 콜리 전달 | 4 |
| 7 | 7 | debug | &mut DebugFrameData(224B) | 본문 미사용 — RunAway/Recall/Around/AroundRunAway/AroundBush/LaneMinionPosition/Trace 의 get_input 에만 전달(8인자) · AroundHide/AroundRegion/Positioning/AroundPosition/AroundPositionBush/Attack/Skill/Skill2/Ult 는 7인자(debug 없음) | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
L159: champ = data.cache.player_champion[team][position]?  — None → return None(-1)
L160~161: if is_enemy_well_danger(version, player, champ.x, champ.y) || is_recent_enemy_well_damage_danger(version, player, champ) {
  L162: (x,y) = enemy_well_escape_position(ctx.setting, ctx.map, player, champ.x, champ.y)   [ScalarPair (x,y)]
  L168: if version <= 1 || distance_sq(champ,(x,y)) >= 4000001 (>2000) → L169: return Some(Move{x,y})
        (v2+ 이고 탈출점에 이미 도달(≤2000)이면 통과)
}
L174~178: phase = match self { RunAway|AroundRunAway→72, LaneMinionPosition→66, Trace→67, Attack|Skill|Skill2|Ult→47, _→46 } ; _t = ProfTimer::start(phase)  (prof::ENABLED 일 때만 Instant::now · 텔레메트리)
L181~200: input: Option<Input> = match self {
  RunAway(0)→SmallActionRunAway::get_input(self,version,rnd,player,data,positioning_score,debug)   L182
  Recall(1)→SmallActionRecall::get_input(…,debug) L183 · Positioning(6)→SmallActionPositioning::get_input(… 7인자) L184
  Around(2)→SmallActionAround::get_input(…,debug) L185 · AroundHide(3)→(7인자) L186 · AroundRegion(4)→(7인자) L187
  AroundRunAway(5)→SmallActionAround::get_input(…,debug) L188 (Around 와 같은 페이로드 타입) · AroundPosition(7)→(7인자) L189
  AroundPositionBush(8)→(7인자) L190 · AroundBush(9)→(…,debug) L191 · LaneMinionPosition(10)→fastcc get_input(…,debug) L192 · Trace(11)→(…,debug) L193
  Attack(12)/Skill(13)/Skill2(14)/Ult(15)→cast::*::get_input(… 7인자) L194~197
  Stop(16)→ Some(Move{champ.x, champ.y})  L200 (태그 store 는 다음 단계 Move 분기로 접힘)
}
L204: out = avoid_enemy_well_move_input(version, player, data, input)  [인라인 small_action.rs:34~41]:
      if input.tag==0 (Some(Move{x,y})): champ = player_champion[..]? (None→None) ; out = safe_move_avoiding_enemy_well(version, player, data, champ, x, y)  [잎22 · sret 32B Option<Input>]
      else: out = input 그대로(memcpy 32B)
L212: if version > 1 && out is Some(Move) {
  L213: (x,y) = out.Move ; L215: (ax,ay) = Game::adjust_position(ctx.map, ctx.setting, x, y)
  L216: if distance_sq(champ,(ax,ay)) < 4000001 (보정 후 제자리 ≤2000) {
    L217: if let Some((gx,gy)) = self.target_position()  [인라인 230~240: RunAway/Positioning/AroundPosition/AroundPositionBush→(+0x8,+0x10) · Recall→(+0x50,+0x58) · Around/AroundHide/LaneMinionPosition→(+0x10,+0x18) · AroundBush→(+0x18,+0x20) · Trace→(+0x68,+0x70) · 그 외(AroundRegion/AroundRunAway/Attack~Stop)→None] {
      L218: (gx2,gy2) = adjust_position(map, setting, gx, gy)
      L219: if distance_sq(champ,(gx2,gy2)) > 4000000 (>2000) → L220: out = Some(Move{gx, gy})   (원 목표 좌표, 보정 전 값)
    }
  }
}
L227: return out  (ProfTimer drop: ENABLED 였으면 PHASE_NANOS[phase]+=ns, PHASE_CALLS[phase]+=1)
```

**`mem` 메모리 접근 28건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | 42829 (gep 2352) bounds 2 | 4 | OK |  |
| 1 | PlayerState | 0x9c0 | info.position@tag | r | 42840 (gep 2496) | 4 | OK |  |
| 2 | OperationData | 0x0 | cache | r | 42843 | 4 | OK |  |
| 3 | OperationData | 0x8 | context | r | 42877 / 43208 | 4 | OK |  |
| 4 | AbstractGameWithCache | 0x1e0 | player_champion[team][position] | r | 42844~42847 · null → None · 43115 재로드(avoid_enemy_well_move_input 인라인) | 4 | OK |  |
| 5 | GameContext | 0x8 | setting | r | 42878 / 43212 → enemy_well_escape_position · adjust_position | 4 | OK |  |
| 6 | GameContext | 0x20 | map | r | 42881 / 43210 | 4 | OK |  |
| 7 | Entity | 0x660 | x | r | 42854 champ.x (탈출 판정·거리·Stop 입력·43138/43226) | 4 | OK |  |
| 8 | Entity | 0x668 | y | r | 42858 champ.y | 4 | OK |  |
| 9 | SmallActionPlay | 0xb1 | @tag | r | 42897/42973/43254 (gep 177) → idx = tag>2 ? tag-3 : 7 | 4 | OK |  |
| 10 | SmallActionPlay | 0x8 | RunAway/Positioning/AroundPosition.goal_x · AroundPositionBush.target_x | r | 43286 phi 8 (target_position 인라인 small_action.rs:230~240) | 4 | OK |  |
| 11 | SmallActionPlay | 0x10 | 위 goal_y · Around/AroundHide/LaneMinionPosition.goal_x | r | phi 16 | 4 | 오귀속(사전은 다른 필드를 준다) |  |
| 12 | SmallActionPlay | 0x18 | Around/AroundHide/LaneMinionPosition.goal_y · AroundBush.target_x | r | phi 24 | 4 | OK |  |
| 13 | SmallActionPlay | 0x20 | AroundBush.target_y | r | phi 32 | 4 | OK |  |
| 14 | SmallActionPlay | 0x50 | Recall.goal_x | r | phi 80 | 4 | OK |  |
| 15 | SmallActionPlay | 0x58 | Recall.goal_y | r | phi 88 | 4 | OK |  |
| 16 | SmallActionPlay | 0x68 | Trace.goal_x | r | phi 104 (L240) | 4 | OK |  |
| 17 | SmallActionPlay | 0x70 | Trace.goal_y | r | phi 112 | 4 | OK |  |
| 18 | Input | 0x0 | @tag | r | 43096/43155 == 0 Move 판정 | 4 | OK |  |
| 19 | Input | 0x8 | Move.x | r | 43124 / 43201 | 4 | OK |  |
| 20 | Input | 0x10 | Move.y | r | 43121 / 43204 | 4 | OK |  |
| 21 | prof::ENABLED | 0x0 | 정적 AtomicBool | r | 42962 계측 on/off(텔레메트리) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 22 | sret Option<Input> | 0x0 | tag | w |  | 4 | 확인불가(tcx 사전에 타입 없음) | -1 None (42866 champ 없음 · 43132 Move 보정 시 champ 없음) / 0 Move (42916 탈출 · 43323 승격) / memcpy 32B (43109) / safe_move_avoiding_enemy_well sret (43128) |
| 23 | sret Option<Input> | 0x8 | Move.x | w |  | 4 | 확인불가(tcx 사전에 타입 없음) | 42918 탈출점 x / 43324 target_position gx |
| 24 | sret Option<Input> | 0x10 | Move.y | w |  | 4 | 확인불가(tcx 사전에 타입 없음) | 42920 / 43325 |
| 25 | &mut self(SmallActionPlay) | - | (직접 쓰기 없음) | w | 본문 store 대상은 %0(sret)·%9(input 지역)·%10(ProfTimer 지역)뿐(grep 확인). self 변경은 종류별 get_input 콜리 소관 | 4 | 확인불가(오프셋 파싱 실패) | - |
| 26 | &mut rnd / &mut debug | - | (직접 쓰기 없음) | w | 콜리 전달만 | 4 | 확인불가(오프셋 파싱 실패) | - |
| 27 | prof::PHASE_NANOS[phase] / PHASE_CALLS[phase] | phase*8 | 정적 계측 카운터 | w | 텔레메트리 — 판정 무관 | 4 | 확인불가(tcx 사전에 타입 없음) | 43189 += elapsed ns · 43193 += 1 (ENABLED 일 때만) |

**`consts` 상수 14건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 2 | 159 | 임계 | player_champion bounds team<2 (42831) | 4 |
| 1 | 1 | 168 | 임계 | version > 1 (42892 / 43153) — v2+ 게이트. 및 PHASE_CALLS += 1 (43193) | 4 |
| 2 | 4000001 | 168 | 임계 | 2000²+1 — dist² < 4000001 ⇔ 거리 ≤ 2000(1/16 셀) : 탈출점 이미 도달(42937) · 보정 입력이 제자리(43249) | 4 |
| 3 | 4000000 | 219 | 임계 | 2000² — 목표점 보정 좌표와의 dist² > 4000000 ⇔ 거리 > 2000 이면 목표점 Move 로 승격(43319) | 4 |
| 4 | 0 | 169 | 태그 | Input 태그 0 = Move (42916 · 43105 · 43156 · 43323) | 4 |
| 5 | -1 | 159 | 센티널 | Option<Input> None 니치 태그 (42866 · 43132) · ProfTimer 미시작 표식 nanos -1 (42980/43162) | 4 |
| 6 | 72 | 178 | 산출값 | prof phase id: RunAway/AroundRunAway get_input (42941) | 4 |
| 7 | 66 | 176 | 산출값 | prof phase id: LaneMinionPosition (42945) | 4 |
| 8 | 67 | 177 | 산출값 | prof phase id: Trace (42949) | 4 |
| 9 | 47 | 175 | 산출값 | prof phase id: Attack/Skill/Skill2/Ult (42953) | 4 |
| 10 | 46 | 174 | 산출값 | prof phase id: 그 외 소액션 (42957 phi 기본) | 4 |
| 11 | 1000000000 | 227 | 임계 | 초→나노초 (43181, ProfTimer drop 계측) | 4 |
| 12 | 7 | 174 | 센티널 | 니치 untagged AroundPosition 의 논리 idx (42903 select) | 4 |
| 13 | 10 | 174 | 태그 | 태그 10 부재 assume (42899) — AroundPosition 암묵 | 4 |

**`knobs` 조정점 3건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 탈출점 도달/제자리 판정 반경 | small_action.rs:168 · 216 | 4000001 (=2000²+1) | ≤2000 유닛(1/16 셀)이면 '도달/제자리'. 올리면 탈출 Move 를 더 일찍 멈추고, 제자리 판정이 넓어져 목표점 승격이 잦아짐 | 4 | 기존 |
| 1 | 목표점 승격 반경 | small_action.rs:219 | 4000000 (=2000²) | >2000 이면 보정 결과 대신 원 목표점 Move 로 교체. 내리면 더 가까운 목표도 승격 | 4 | 기존 |
| 2 | v2 게이트 | small_action.rs:168 · 212 | version > 1 | v1 이하: 우물 위험이면 무조건 탈출 Move · 제자리 승격 없음 | 4 | 기존 |

<details><summary>`callees` 피호출자 25건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | adjust_position | game_core::Game::adjust_position | pub | fn(&game_core::MapDef, &game_core::GameSetting, i64, i64) -> (u64, u64) | game-core\src\simulation\game.rs:818 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | avoid_enemy_well_move_input | game_ai::small_action::avoid_enemy_well_move_input | in:game_ai::small_action | fn(usize, &game_core::PlayerState, &game_core::OperationData, std::option::Option<game_core::Input>) -> std::option::Option<game_core::Input> | game-ai\src\small_action.rs:33 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 2 | distance_sq | game_core::utils::distance_sq | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:6 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
| 3 | distance_sq | game_core::Entity::distance_sq | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2157 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
| 4 | enemy_well_escape_position | game_ai::enemy_well_escape_position | pub | fn(&game_core::GameSetting, &game_core::MapDef, &game_core::PlayerState, u64, u64) -> (u64, u64) | game-ai\src\path_finder.rs:1043 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 5 | get_input | game_ai::SmallActionUlt::get_input | in:game_ai | fn(&mut game_ai::SmallActionUlt, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData) -> std::option::Option<game_core::Input> | game-ai\src\small_action\cast.rs:261 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 6 | get_input | game_ai::SmallActionTrace::get_input | in:game_ai | fn(&mut game_ai::SmallActionTrace, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &mut game_core::DebugFrameData) -> std::option::Option<game_core::Input> | game-ai\src\small_action\trace.rs:165 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 7 | get_input | game_ai::SmallActionSkill::get_input | in:game_ai | fn(&mut game_ai::SmallActionSkill, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData) -> std::option::Option<game_core::Input> | game-ai\src\small_action\cast.rs:133 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 8 | get_input | game_ai::SmallActionRecall::get_input | in:game_ai | fn(&mut game_ai::SmallActionRecall, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &mut game_core::DebugFrameData) -> std::option::Option<game_core::Input> | game-ai\src\small_action\move_actions.rs:681 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 9 | get_input | game_ai::SmallActionAround::get_input | in:game_ai | fn(&mut game_ai::SmallActionAround, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &mut game_core::DebugFrameData) -> std::option::Option<game_core::Input> | game-ai\src\small_action\around.rs:61 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 10 | get_input | game_ai::SmallActionAttack::get_input | in:game_ai | fn(&mut game_ai::SmallActionAttack, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData) -> std::option::Option<game_core::Input> | game-ai\src\small_action\cast.rs:73 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 11 | get_input | game_ai::SmallActionRunAway::get_input | in:game_ai | fn(&mut game_ai::SmallActionRunAway, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &mut game_core::DebugFrameData) -> std::option::Option<game_core::Input> | game-ai\src\small_action\move_actions.rs:95 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 12 | get_input | game_ai::SmallActionAroundHide::get_input | in:game_ai | fn(&mut game_ai::SmallActionAroundHide, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData) -> std::option::Option<game_core::Input> | game-ai\src\small_action\around.rs:324 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 13 | get_input | game_ai::SmallActionAroundBush::get_input | in:game_ai | fn(&mut game_ai::SmallActionAroundBush, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &mut game_core::DebugFrameData) -> std::option::Option<game_core::Input> | game-ai\src\small_action\around.rs:1195 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 14 | get_input | game_ai::SmallActionPositioning::get_input | in:game_ai | fn(&mut game_ai::SmallActionPositioning, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData) -> std::option::Option<game_core::Input> | game-ai\src\small_action\around.rs:701 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 15 | get_input | game_ai::SmallActionAroundRegion::get_input | in:game_ai | fn(&mut game_ai::SmallActionAroundRegion, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData) -> std::option::Option<game_core::Input> | game-ai\src\small_action\around.rs:529 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 16 | get_input | game_ai::SmallActionAroundPosition::get_input | pub | fn(&mut game_ai::SmallActionAroundPosition, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData) -> std::option::Option<game_core::Input> | game-ai\src\small_action\around.rs:864 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 17 | get_input | game_ai::SmallActionAroundPositionBush::get_input | in:game_ai | fn(&mut game_ai::SmallActionAroundPositionBush, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData) -> std::option::Option<game_core::Input> | game-ai\src\small_action\around.rs:1086 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 18 | get_input | game_ai::SmallActionLaneMinionPosition::get_input | in:game_ai | fn(&mut game_ai::SmallActionLaneMinionPosition, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &mut game_core::DebugFrameData) -> std::option::Option<game_core::Input> | game-ai\src\small_action\lane_minion.rs:540 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 19 | is_enemy_well_danger | game_ai::is_enemy_well_danger | pub | fn(usize, &game_core::PlayerState, u64, u64) -> bool | game-ai\src\path_finder.rs:1032 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 20 | is_recent_enemy_well_damage_danger | game_ai::is_recent_enemy_well_damage_danger | pub | fn(usize, &game_core::PlayerState, &game_core::Entity) -> bool | game-ai\src\path_finder.rs:1037 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 21 | safe_move_avoiding_enemy_well | game_ai::safe_move_avoiding_enemy_well | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, u64, u64) -> std::option::Option<game_core::Input> | game-ai\src\abstract_input.rs:122 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 22 | start | game_core::prof::start | pub | fn(usize) -> std::option::Option<game_core::prof::ProfTimer> | game-core\src\simulation\prof.rs:175 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
| 23 | start | game_view::UIPhaseEffect::start | pub | fn(&mut game_view::UIPhaseEffect) | game-view\src\ui\match_ui\phase_effect.rs:30 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
| 24 | target_position | game_ai::SmallActionPlay::target_position | pub | fn(&game_ai::SmallActionPlay) -> std::option::Option<(u64, u64)> | game-ai\src\small_action.rs:229 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
</details>

⚠**미매칭 1개**: `elapsed`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 11곳** (m02.ll:17007, m02.ll:44224, m14.ll:16651, m14.ll:28157, m14.ll:38959, m14.ll:39050, m14.ll:39084, m14.ll:39191, m14.ll:58297, m15.ll:17883, m15.ll:23163) · **형제 18개** (SmallActionPlay)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::SmallActionPlay as std::clone::Clone>::clone | pub | game-ai\src\small_action.rs:8 | True | fn(&game_ai::SmallActionPlay) -> game_ai::SmallActionPlay |
| 1 | <game_ai::SmallActionPlay as std::fmt::Debug>::fmt | pub | game-ai\src\small_action.rs:8 | True | fn(&game_ai::SmallActionPlay, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 2 | game_ai::SmallActionPlay::get_input | pub | game-ai\src\small_action.rs:157 | False | fn(&mut game_ai::SmallActionPlay, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &mut game_core::DebugFrameData) -> std::option::Option<game_core::Input> |
| 3 | game_ai::SmallActionPlay::target_position | pub | game-ai\src\small_action.rs:229 | True | fn(&game_ai::SmallActionPlay) -> std::option::Option<(u64, u64)> |
| 4 | game_ai::SmallActionPlay::evaluation_position | pub | game-ai\src\small_action.rs:245 | False | fn(&game_ai::SmallActionPlay, usize, &game_core::PlayerState, &game_core::OperationData) -> std::option::Option<(u64, u64)> |
| 5 | game_ai::SmallActionPlay::position_eval_purpose | pub | game-ai\src\small_action.rs:270 | True | fn(&game_ai::SmallActionPlay) -> game_ai::PositionEvalPurpose |
| 6 | game_ai::SmallActionPlay::avoid_unnecessary_tower | pub | game-ai\src\small_action.rs:284 | True | fn(&game_ai::SmallActionPlay) -> bool |
| 7 | game_ai::SmallActionPlay::update_state | pub | game-ai\src\small_action.rs:291 | False | fn(&mut game_ai::SmallActionPlay, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 8 | game_ai::SmallActionPlay::get_action | pub | game-ai\src\small_action.rs:308 | True | fn(&game_ai::SmallActionPlay) -> game_core::SmallAction |
| 9 | game_ai::SmallActionPlay::path_finder | pub | game-ai\src\small_action.rs:330 | False | fn(&game_ai::SmallActionPlay) -> std::option::Option<&game_ai::PathFinder> |
| 10 | game_ai::SmallActionPlay::is_end | pub | game-ai\src\small_action.rs:348 | False | fn(&game_ai::SmallActionPlay, &mut rand::rngs::std::StdRng, usize, &game_core::PlayerState, &game_core::OperationData) -> bool |
| 11 | game_ai::SmallActionPlay::is_premise_lost | pub | game-ai\src\small_action.rs:376 | False | fn(&game_ai::SmallActionPlay, &game_core::OperationData) -> bool |
| 12 | game_ai::SmallActionPlay::is_complete | pub | game-ai\src\small_action.rs:386 | True | fn(&game_ai::SmallActionPlay) -> bool |
| 13 | game_ai::SmallActionPlay::merge | pub | game-ai\src\small_action.rs:397 | False | fn(&mut game_ai::SmallActionPlay, usize, &game_core::Entity, game_ai::SmallActionPlay) |
| 14 | game_ai::SmallActionPlay::move_near_complete | pub | game-ai\src\small_action.rs:413 | False | fn(&game_ai::SmallActionPlay, &game_core::Entity) -> bool |
| 15 | game_ai::SmallActionPlay::is_ult_escape | pub | game-ai\src\small_action.rs:437 | True | fn(&game_ai::SmallActionPlay) -> bool |
| 16 | game_ai::SmallActionPlay::extend_action | pub | game-ai\src\small_action.rs:444 | True | fn(&mut game_ai::SmallActionPlay, usize) |
| 17 | game_ai::SmallActionPlay::is_action_complete | pub | game-ai\src\small_action.rs:462 | True | fn(&game_ai::SmallActionPlay) -> bool |

**`open` 5건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 표기 불가 | L160~161 두 위험 술어의 결합이 `\|\|` 인지(IR: 첫째 true 면 둘째 생략 후 탈출 — 단락 OR 로 확정) 및 L168 의 `version<=1 \|\| dist` 결합 순서 — 동작은 확정, 소스 표기만 column 부재로 추정 | 4 |  |
| 1 | 미탐색 | avoid_enemy_well_move_input(small_action.rs:34~41)은 인라인이라 별도 define 없음 — 시그니처 (version, player, data, Option<Input>) → Option<Input> 은 dbg 인자(43099~43102)로 복원 | 4 |  |
| 2 | 미탐색 | L200 Stop 경로: %124 는 x,y 만 store 하고 태그 0 store 가 없다 — 바로 Move 분기(%115)로 점프하므로 컴파일러가 접은 것으로 읽음(Input::Move 확정) | 4 |  |
| 3 | 미탐색 | 종류별 get_input 16종·safe_move_avoiding_enemy_well(잎22 · m04.ll:43378 · (version, player, data, champ, x, y) → sret Option<Input>)·enemy_well_escape_position(m03.ll:144939 · (setting, map, player, x, y) → (x,y))·is_recent_enemy_well_damage_danger(m03.ll:146197)·Game::adjust_position(g15.ll:72245 · (map, setting, x, y) → (x,y)) 내부 미탐색(계약만) | 4 |  |
| 4 | 미탐색 | ProfTimer/PHASE_NANOS/PHASE_CALLS 는 계측 전용 — 판정에 영향 없음(ENABLED 정적 플래그) | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

