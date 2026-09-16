---

### `266` TeamPlan::can_near_enemies_range — (x,y) 반경 d 안에 '있을 수 있는' 비가시 적 챔피언 목록 — 안 보인 시간·이속·판단력 추정으로 후보를 걸러 bumpalo Vec<&Entity> 로 반환

| 항목 | 값 |
|---|---|
| id | `TeamPlan__can_near_enemies_range` |
| 심볼 | `_RNvMs1_NtNtCshdEBA0ozCnw_7game_ai11plan_legacy9team_planNtB5_8TeamPlan22can_near_enemies_range` |
| 소스 | `game-ai\src\plan_legacy\team_plan.rs:483` |
| IR | `m09.ll` 25960~26127행 |
| 경로·가시성 | `game_ai::plan_legacy::team_plan::TeamPlan::can_near_enemies_range` · **pub** |
| 계층 | 기타 |
| exe | `dd9f30` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r18` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, u64, u64, u64, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< &game_core::Entity>
```

**TLS 메모 절(`sig.tls` · 정정 경로 `/specs[266]/sig/tls/<키>`)**

없음 — 본체(m09 25960~26127)와 aux 2조각의 @anon 참조는 @anon.cadd6…69 · @anon.17be…68 두 건뿐이고 둘 다 panic Location 상수(locfind 확인). llvm.threadlocal.address 0 · LocalKey::with 0. 콜리 unseen_estimated_pos / awareness_lapse_state 내부의 TLS 여부는 미조사(계약만)

<details><summary>인자 10개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | bumpalo::collections::vec::Vec<&Entity> (32B) | sret · dead_on_unwind noalias writable captures(none) dereferenceable(32) · 반환 컨테이너. 아래 returns 참조 | 4 |
| 1 | 1 | self | &TeamPlan (1064B) | IR 속성 = noundef nonnull align 8 (readonly 없음) · 동작상 읽기만(vision.* 3배열을 클로저 캡처 포인터 self[40] 경유로 read) · &self — 쓰기 0 (writes 절 참조) | 4 |
| 2 | 2 | _version | usize | 미사용 — #dbg_value(i64 poison) · 본문 참조 0 · 버전 분기 없음 | 4 |
| 3 | 3 | rnd | &mut StdRng (320B) | IR 속성 = noalias noundef align 16 dereferenceable(320) (readonly 없음) · 클로저 캡처 self[96] 로만 전달 · gen_range 호출 사이트 1개(aux m01 25196 · team_plan.rs:528 · is_dm 분기 안에서만) — gamemode=0 이면 rnd 소비 0 | 4 |
| 4 | 4 | player | &PlayerState (2528B) | readonly captures(address, read_provenance) · info.parameter(+0x180)·info.team(+0x930)·info.id(+0x928) | 4 |
| 5 | 5 | data | &OperationData (24B) | readonly captures(address, read_provenance) · cache(+0)·context(+8)·blackboard(+0x10) | 4 |
| 6 | 6 | x | u64 | 값 인자 → alloca %21 에 store 후 클로저에 &x 로 캡처(self[56]) · 질의 중심 x (exe 는 entry+0x30 스택 슬롯을 lea 로 그대로 캡처) | 4 |
| 7 | 7 | y | u64 | 값 인자 → alloca %20 · 클로저 캡처 self[64] · 질의 중심 y | 4 |
| 8 | 8 | d | u64 | 값 인자 → alloca %19 · 클로저 캡처 self[72] · 질의 반경. 거리에서 saturating_sub 되는 '허용 반경' | 4 |
| 9 | 9 | _debug | &mut DebugFrameData | IR 속성 = noalias readnone align 8 captures(none) · 미사용(#dbg_value ptr poison) | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn can_near_enemies_range(&self, _version, rnd, player, data, x, y, d, _debug) -> bumpalo Vec<&Entity>
// team_plan.rs:487~489 (m09 25991~26001)
ja = player.info.parameter.judge_accuracy()            // [100,1000]
range_min = 1000 - (1000 - ja)/2                        // [550,1000]  (lshr 1)
range_max = 1000 + (1000 - ja)/2                        // [1000,1450]
// L494 (26003~26013)
is_dm = (data.cache.game.get_game_mode()@tag == 2 /*DeathMatch*/)
// L496~497 (26014~26018 · 75)  ★극성: is_dm 이 참이면 lapse 는 평가조차 안 함(A||B 의 B)
if !is_dm && player_awareness_lapse(player, data) {
    return Vec::new_in(data.context.pool)                // ptr=8 · cap=0 · len=0 (26115~26119)
}
// L499~502 (26022~26041)
judgement_base = player.info.parameter.judgement_base()  // [0,100]
game_seed = game.seed();  tick = game.tick();  tps = data.context.setting.tick_per_second
// L505~537 (26064~26102 → aux m01 24912~25358 from_iter · 캡처 15포인터 = cache,blackboard,player,&is_dm,&tick,self,&tps,&x,&y,&d,&judgement_base,&game_seed,rnd,&range_min,&range_max)
out = Vec::new_in(data.context.pool)
for i in 0..5 {                                          // 적 팀 슬롯 순회 (Range 0..5 · 25983~25992)
  // ── filter 술어 closure0 (L506~535) ──
  et = 1 - player.info.team                              // 25035~25038 · et<2 아니면 panic(506:7)
  c = data.cache.player_champion[et][i]                  // 25053~25057 · i<5 아니면 panic(506:7)
  if c.is_none() { continue }                            // 25092 null → 제외
  e = c.unwrap()
  // L507 (25121~25129)
  if game.is_visible(player.info.team, e.id) { continue } // ★보이는 적은 제외 — 이 목록은 '비가시' 적 전용
  // L510
  move_speed_raw = e.stat_cached.move_speed              // 25132
  // L511 (25135~25137)
  if !is_dm {                                            // ── MOBA/SingleLane ──
    // L512~513 (25140~25151)
    elapsed = tick.saturating_sub(self.vision.last_visible_ticks[i])
    if elapsed > tps*3 {                                 // 3초 넘게 미목격 → 판단력 미시야 추정
      // L519~520 (25154~25161)
      err = unseen_error_radius(judgement_base, elapsed, tps)
      if err > 300000 { continue }                       // 추정 불능 → 제외
      // L523 (25257~25264)
      (ex, ey) = unseen_estimated_pos(game_seed, player.info.id, i, tick, tps, e.x, e.y, err)
      // L525 (25272~25284)
      keep = distance(ex, ey, x, y) <= err + move_speed_raw*3*tps + d   // IR: `dist > err+3·ms·tps+d` 이면 제외
    } else {                                             // ≤3초 → 도달 원반(마지막 목격 + 이속·경과)
      // L515~517 (25164~25185)
      can_move = elapsed * move_speed_raw
      dist_from_last = distance(x, y, self.vision.last_visible_pos[i].0, .1).saturating_sub(d)
      keep = can_move >= dist_from_last                  // IR: `can_move < dist_from_last` 이면 제외
    }
  } else {                                               // ── DeathMatch (gamemode=0 에선 사장 · NA 봉인) ──
    // L528 (25189~25210)  ★gen_range 사이트 1 — 후보 i 마다 1회, tick() 재호출보다 먼저
    move_speed = rnd.gen_range(range_min..=range_max) * move_speed_raw / 1000   // 판단 정확도 노이즈 ±(1000−ja)/2 ‰
    // L531 (25202~25218)
    can_move = game.tick().saturating_sub(self.vision.last_visible_ticks[i]) * move_speed
    // L533 (25220~25234)
    dist_from_last = distance(x, y, self.vision.last_visible_pos[i].0, .1).saturating_sub(d)
    // L534~535 (25235~25254)
    if self.vision.last_visible_ticks[i] > self.vision.mia_call_ticks[i] {
      keep = blackboard[et].is_recent_visible_big_action(game, player, e) && can_move >= dist_from_last   // IR: 호출을 무조건 먼저 수행 후 `and`(25244~25249)
    } else {
      keep = can_move >= dist_from_last                  // 25253
    }
  }
  if !keep { continue }
  // ── filter_map closure_s_0 (L537 · aux m09 66111~66147) ──
  t = data.cache.player_champion[1 - player.info.team][i]   // Option<&Entity> 재조회(바운드 검사 재수행 · 537:23) · None 이면 skip(25295 · 실제론 위에서 Some 확정)
  out.push(t)                                            // 25308~25340 (cap==len 이면 reserve_internal_or_panic)
}
return out                                               // 25355 memcpy 32B

■ 사장 판정(reach.py · version=2 무관(버전 분기 없음) · gamemode=0 → %36=0(본체) · %90=0(클로저)): 본체 5블록 전부 live / 클로저 36블록 중 7 사장 = is_dm 분기(L528~535) → gen_range · distance(L533) · is_recent_visible_big_action 3 호출부 NA. 살아있는 콜리 = panic_bounds_check ×2 · unseen_error_radius · distance ×2(L516·L525) · unseen_estimated_pos · call_mut · reserve_internal_or_panic · Drop::drop(unwind 정리).
■ rnd(StdRng) 소비: gen_range 사이트 1(L528) · is_dm 일 때만 · 순서 = i 오름차순으로 (Some 챔피언 && !is_visible) 인 후보마다 1회(그 뒤 keep 여부와 무관). gamemode=0 → 0회.
■ 호출 순서(부작용 순): judge_accuracy → get_game_mode(vt+0x40) → [!is_dm] player_awareness_lapse → judgement_base → seed(vt+0x20) → tick(vt+0x28) → from_iter 루프.
```

**`mem` 메모리 접근 29건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x180 | info.parameter (AthleteParameter 744B) | r | m09 25991 gep 384 → judge_accuracy(L487)·judgement_base(L499) 인자 | 4 | OK |  |
| 1 | PlayerState | 0x930 | info.team | r | aux m01 25035 gep 2352 · m09 66121 — 1-team 으로 적 팀 인덱스 | 4 | OK |  |
| 2 | PlayerState | 0x928 | info.id | r | aux m01 25258 gep 2344 — unseen_estimated_pos observer_pid | 4 | OK |  |
| 3 | OperationData | 0x0 | cache (&AbstractGameWithCache) | r | m09 26003 | 4 | OK |  |
| 4 | OperationData | 0x8 | context (&GameContext) | r | m09 26035·26111 | 4 | OK |  |
| 5 | OperationData | 0x10 | blackboard (&[Blackboard; 2]) | r | m09 26043 — 클로저 캡처 self[8] | 4 | OK |  |
| 6 | AbstractGameWithCache | 0x0 | game.data_ptr (dyn AbstractGame) | r | m09 26004 · aux 25059 | 4 | OK |  |
| 7 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r | m09 26005 · aux 25060 | 4 | OK |  |
| 8 | AbstractGameWithCache | 0x1e0 | player_champion [[Option<&Entity>;5];2] | r | aux m01 25054 gep 480 · m09 66142 — [1-team][i] (stride 40·8) | 4 | OK |  |
| 9 | GameContext | 0x0 | pool (&Bump) | r | m09 26101(정상)·26113(lapse 빈 Vec) — sret buf.a | 4 | OK |  |
| 10 | GameContext | 0x8 | setting (&GameSetting) | r | m09 26037 | 4 | OK |  |
| 11 | GameSetting | 0x12f8 | tick_per_second | r | m09 26039 gep 4856 → tps | 4 | OK |  |
| 12 | vtable AbstractGame | 0x40 | get_game_mode() -> GameMode(16B {tag,payload}) | r | m09 26007~26009 · divtable 일치율 98% · 태그만 사용 | 3 | 확인불가(tcx 사전에 타입 없음) |  |
| 13 | vtable AbstractGame | 0x20 | seed() -> u64 | r | m09 26025 → game_seed | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 14 | vtable AbstractGame | 0x28 | tick() -> usize | r | m09 26030 → tick(캡처) · aux m01 25202 (is_dm 분기에서 재호출) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 15 | vtable AbstractGame | 0xf8 | is_visible(team, id) -> bool | r | aux m01 25123~25125 gep 248 — is_visible(player.info.team, e.id) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 16 | Entity | 0x5c0 | id | r | aux m01 25121 gep 1472 | 4 | OK |  |
| 17 | Entity | 0x640 | stat_cached.move_speed | r | aux m01 25132 gep 1600 → move_speed_raw | 4 | OK |  |
| 18 | Entity | 0x660 | x | r | aux m01 25260 gep 1632 → unseen_estimated_pos actual_x | 4 | OK |  |
| 19 | Entity | 0x668 | y | r | aux m01 25262 gep 1640 → actual_y | 4 | OK |  |
| 20 | TeamPlan | 0x230 | vision.last_visible_pos[i] (u64,u64) stride 16 | r | aux m01 25164·25212 gep 560 + i*16 — 마지막 목격 좌표 | 4 | OK |  |
| 21 | TeamPlan | 0x280 | vision.mia_call_ticks[i] stride 8 | r | aux m01 25236 gep 640 — is_dm 분기 L534 비교에만 | 4 | OK |  |
| 22 | TeamPlan | 0x2a8 | vision.last_visible_ticks[i] stride 8 | r | aux m01 25142·25213 gep 680 — 마지막 목격 틱 | 4 | OK |  |
| 23 | [Blackboard;2] | 0x2e8 | stride 744 → blackboard[1-team] | r | aux m01 25243 getelementptr 구조체타입 idx %48 — is_recent_visible_big_action self 인자(is_dm 분기만) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 24 | sret Vec<&Entity> | 0x0 | buf.ptr | w | m09 26115 · aux m01 25355 | 4 | 확인불가(tcx 사전에 타입 없음) | 8(dangling · lapse 경로) / from_iter 결과 memcpy 32B |
| 25 | sret Vec<&Entity> | 0x8 | buf.a | w | m09 26117 / aux 24924 | 4 | 확인불가(tcx 사전에 타입 없음) | data.context.pool |
| 26 | sret Vec<&Entity> | 0x10 | cap·len (16B) | w | m09 26119 / aux 25337~25340 (push: ptr[len]=t · len+=1 · cap 부족 시 reserve_internal_or_panic) | 4 | 확인불가(tcx 사전에 타입 없음) | 0·0 (memset) / from_iter 누적 |
| 27 | TeamPlan(&self) | - | (쓰기 없음) | w | &self 불변 참조 · 본체+aux 전 범위에서 self 기준 store 0 (initializes 속성 없음) | 4 | 확인불가(오프셋 파싱 실패) | - |
| 28 | StdRng(rnd) | - | 내부 상태(gen_range 경유) | w | aux m01 25196 gen_range 1사이트 — is_dm 분기 안에서만 · 후보 i 마다 1회 · gamemode=0 이면 0회 | 4 | 확인불가(오프셋 파싱 실패) | - |

**`consts` 상수 8건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev | folded_from |
|---|---|---|---|---|---|---|
| 0 | 1000 | 488 | 계수 | 판단 정확도 스케일 상한(judge_accuracy ∈[100,1000]). range_min = 1000 − (1000 − ja)/2 · range_max = 1000 + (1000 − ja)/2 (m09 25995~26000) · aux L528 `gen*move_speed_raw/1000`(m01 25210) = 이속 배율 ‰ | 4 |  |
| 1 | 1 | 488 | 인덱스 | `(1000 − ja) / 2` 가 `lshr i64 %24, 1` 로 접힘(m09 25996) — 오차 폭 절반. ⚠aux 의 `sub i64 1, %team`(25037·66123)은 별개: 1 − team = 적 팀 인덱스 | 4 | 2 |
| 2 | 2 | 494 | 태그 | GameMode@tag == 2 = DeathMatch (tcxdict --enum GameMode: Moba 0 · SingleLane 1 · DeathMatch 2 · Direct 8B) → is_dm. ⚠aux 의 `icmp ult %48, 2`(25038)는 [Blackboard;2]/player_champion[2] 바운드 검사 | 3 |  |
| 3 | 0 | 505 | 산출값 | Range 시작 — 슬롯 인덱스 i ∈ 0..5 (m09 26094 iter[120]) | 4 |  |
| 4 | 5 | 505 | 산출값 | Range 끝(배타) — 포지션 슬롯 5개 (m09 26096 iter[128]) · aux 바운드 검사 `ult %26, 5`(25042·66128) | 4 |  |
| 5 | 3 | 513 | 미상 | aux L513 `elapsed > tps*3`(m01 25149) — 3초 넘게 안 보였으면 '판단력 미시야 추정' 경로, 아니면 '도달 원반' 경로 | 4 |  |
| 6 | 300000 | 520 | 미상 | aux L520 `err > 300000` → 추정 불능(어디 있는지 전혀 모름) → 후보 제외 (m01 25160). _docs game_core:163 「추정 불능 경계」와 동일 값(300k = 9.375셀) | 4 |  |
| 7 | 3 | 525 | 미상 | aux L525 `move_speed_raw*3*tps`(m01 25279~25280) — 3초 이동 여유. 허용 = err + 3초이동 + d | 4 |  |

**`knobs` 조정점 4건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 미목격 3초 경계(도달원반 ↔ 판단력 추정 전환) | team_plan.rs:513 (aux m01 25149 `mul %tps, 3`) | 3 | 올리면 더 오래 '마지막 목격점+이속·경과' 원반 모델로 보수적 추적(원반이 커져 거의 전원 포함) · 내리면 판단력 오차 추정으로 빨리 넘어가 저판단력 선수가 더 빨리 '모름'(300k 초과) 처리 | 4 | 기존 |
| 1 | 추정 불능 경계 | team_plan.rs:520 (aux m01 25160) | 300000 | 올리면 오차가 커도 후보로 남겨(넓은 범위에 '있을 수 있음') 회피/경계 과잉 · 내리면 미시야 적을 빨리 무시. _docs game_core:163 의 경계와 같은 값 — 둘이 따로 상수라 한쪽만 바꾸면 어긋남 | 4 | 기존 |
| 2 | 추정 위치 허용 이동 여유(초) | team_plan.rs:525 (aux m01 25279 `mul %ms, 3`) | 3 | 올리면 추정 위치에서 더 멀리(3초→N초 이동분) 있는 적도 '근처 가능'으로 포함 → 경계 과잉 · 내리면 놓침 | 4 | 기존 |
| 3 | DM 이속 인지 노이즈 폭(판단 정확도 반영 계수) | team_plan.rs:488~489 (m09 25995~26000 · `/2`=lshr 1) | 1000 ± (1000−ja)/2 | gamemode=0 에선 죽은 값(gen_range 사장). DM 에서 나누는 수를 키우면 저판단력도 이속을 정확히 봄 | 4 | 기존 |

<details><summary>`callees` 피호출자 23건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | can_near_enemies_range | game_ai::plan_legacy::team_plan::TeamPlan::can_near_enemies_range | pub | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, u64, u64, u64, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< &game_core::Entity> | game-ai\src\plan_legacy\team_plan.rs:483 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 1 | distance | game_core::utils::distance | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:12 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 2 | get_game_mode | game_core::AbstractGame::get_game_mode | pub | fn(&Self/#0) -> game_core::GameMode | game-core\src\simulation.rs:78 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 3 | get_game_mode | <game_core::Game as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::Game) -> game_core::GameMode | game-core\src\simulation\game.rs:1707 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 4 | get_game_mode | <game_core::SingleLaneGame as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::SingleLaneGame) -> game_core::GameMode | game-core\src\simulation\game.rs:4248 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 5 | is_recent_visible_big_action | game_core::Blackboard::is_recent_visible_big_action | pub | fn(&game_core::Blackboard, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\game\blackboard.rs:367 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 6 | is_visible | game_core::World::is_visible | pub | fn(&game_core::World, usize, usize) -> bool | game-core\src\simulation\game\data.rs:384 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 7 | is_visible | game_core::VisibleState::is_visible | pub | fn(&game_core::VisibleState) -> bool | game-core\src\simulation\game\data.rs:121 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 8 | is_visible | game_core::AbstractGame::is_visible | pub | fn(&Self/#0, usize, usize) -> bool | game-core\src\simulation.rs:125 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 9 | judge_accuracy | game_core::AthleteParameter::judge_accuracy | pub | fn(&game_core::AthleteParameter) -> usize | game-core\src\simulation\state\player.rs:337 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 10 | judgement_base | game_core::AthleteParameter::judgement_base | pub | fn(&game_core::AthleteParameter) -> usize | game-core\src\simulation\state\player.rs:79 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 11 | player_awareness_lapse | game_ai::player_awareness_lapse | pub | fn(&game_core::PlayerState, &game_core::OperationData) -> bool | game-ai\src\utils.rs:538 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 12 | push | <game_core::Staff as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::staff | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\staff.rs:38 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 13 | push | <game_core::Athlete as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:434 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 14 | push | <game_core::Contract as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:2498 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 15 | seed | game_core::AbstractGame::seed | pub | fn(&Self/#0) -> u64 | game-core\src\simulation.rs:72 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 16 | seed | <game_core::Game as game_core::AbstractGame>::seed | pub | fn(&game_core::Game) -> u64 | game-core\src\simulation\game.rs:1564 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 17 | seed | <game_core::SingleLaneGame as game_core::AbstractGame>::seed | pub | fn(&game_core::SingleLaneGame) -> u64 | game-core\src\simulation\game.rs:3781 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 18 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 19 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 20 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 21 | unseen_error_radius | game_core::unseen_error_radius | pub | fn(usize, usize, usize) -> u64 | game-core\src\simulation\ai_interface.rs:237 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 22 | unseen_estimated_pos | game_core::unseen_estimated_pos | pub | fn(u64, usize, usize, usize, usize, u64, u64, u64) -> (u64, u64) | game-core\src\simulation\ai_interface.rs:246 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
</details>

⚠**미매칭 7개**: `closure0`, `closure_s_0`, `drop  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `gen_range`, `panic`, `reserve_internal_or_panic`, `skip`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 14곳** (m04.ll:26253, m05.ll:20764, m05.ll:29555, m06.ll:41158, m09.ll:12392, m09.ll:63515, m10.ll:14241, m10.ll:14324, m13.ll:16717, m13.ll:35231, m13.ll:36287, m13.ll:38670, m13.ll:41170, m13.ll:44021) · **형제 55개** (TeamPlan)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | game_ai::plan_legacy::old::epic::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_press_epic | pub | game-ai\src\plan_legacy\old\epic.rs:502 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 1 | game_ai::plan_legacy::old::epic::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v3_epicops_buff_window | in:game_ai | game-ai\src\plan_legacy\old\epic.rs:634 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &game_ai::plan_legacy::types::BigPlan) -> bool |
| 2 | game_ai::plan_legacy::old::epic::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_epic_line_change | pub | game-ai\src\plan_legacy\old\epic.rs:684 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData, game_core::LineType) |
| 3 | <game_ai::plan_legacy::team_plan::TeamPlan as std::clone::Clone>::clone | pub | game-ai\src\plan_legacy\team_plan.rs:47 | True | fn(&game_ai::plan_legacy::team_plan::TeamPlan) -> game_ai::plan_legacy::team_plan::TeamPlan |
| 4 | <game_ai::plan_legacy::team_plan::TeamPlan as std::fmt::Debug>::fmt | pub | game-ai\src\plan_legacy\team_plan.rs:47 | True | fn(&game_ai::plan_legacy::team_plan::TeamPlan, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 5 | <game_ai::plan_legacy::team_plan::TeamPlan as std::default::Default>::default | pub | game-ai\src\plan_legacy\team_plan.rs:47 | True | fn() -> game_ai::plan_legacy::team_plan::TeamPlan |
| 6 | game_ai::plan_legacy::team_plan::TeamPlan::mf_note_obj_clear | in:game_ai | game-ai\src\plan_legacy\team_plan.rs:196 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, u8, usize) |
| 7 | game_ai::plan_legacy::team_plan::TeamPlan::sanitize_rule_scope | pub | game-ai\src\plan_legacy\team_plan.rs:200 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, &game_core::GameContext) |
| 8 | game_ai::plan_legacy::team_plan::TeamPlan::objective_target | pub | game-ai\src\plan_legacy\team_plan.rs:230 | True | fn(&game_ai::plan_legacy::team_plan::TeamPlan) -> std::option::Option<game_core::JungleType> |
| 9 | game_ai::plan_legacy::team_plan::TeamPlan::take_active | pub | game-ai\src\plan_legacy\team_plan.rs:243 | True | fn(&game_ai::plan_legacy::team_plan::TeamPlan, game_core::JungleType) -> bool |
| 10 | game_ai::plan_legacy::team_plan::TeamPlan::take_hunt_commit | pub | game-ai\src\plan_legacy\team_plan.rs:248 | True | fn(&game_ai::plan_legacy::team_plan::TeamPlan, game_core::JungleType) -> bool |
| 11 | game_ai::plan_legacy::team_plan::TeamPlan::take_setup_like | pub | game-ai\src\plan_legacy\team_plan.rs:257 | True | fn(&game_ai::plan_legacy::team_plan::TeamPlan, game_core::JungleType) -> bool |
| 12 | game_ai::plan_legacy::team_plan::TeamPlan::mark_objective_misunderstanding | pub | game-ai\src\plan_legacy\team_plan.rs:265 | True | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, game_core::JungleType, usize) |
| 13 | game_ai::plan_legacy::team_plan::TeamPlan::clear_objective_misunderstanding | pub | game-ai\src\plan_legacy\team_plan.rs:277 | True | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan) |
| 14 | game_ai::plan_legacy::team_plan::TeamPlan::init | in:game_ai::plan_legacy::team_plan | game-ai\src\plan_legacy\team_plan.rs:281 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 15 | game_ai::plan_legacy::team_plan::TeamPlan::update | pub | game-ai\src\plan_legacy\team_plan.rs:294 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 16 | game_ai::plan_legacy::team_plan::TeamPlan::can_near_enemies | pub | game-ai\src\plan_legacy\team_plan.rs:444 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< &game_core::Entity> |
| 17 | game_ai::plan_legacy::team_plan::TeamPlan::can_near_enemies_range | pub | game-ai\src\plan_legacy\team_plan.rs:483 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, u64, u64, u64, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< &game_core::Entity> |
| 18 | game_ai::plan_legacy::team_plan::TeamPlan::update_wave_priority_clear_line | in:game_ai::plan_legacy::team_plan | game-ai\src\plan_legacy\team_plan.rs:540 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) |
| 19 | game_ai::plan_legacy::team_plan::TeamPlan::should_player_clear_wave_priority_line | pub | game-ai\src\plan_legacy\team_plan.rs:561 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PlayerState, &game_core::OperationData) -> std::option::Option<game_core::LineType> |
| 20 | game_ai::plan_legacy::team_plan::TeamPlan::should_delay_morgard_for_wave_priority | in:game_ai::plan_legacy::team_plan | game-ai\src\plan_legacy\team_plan.rs:570 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData) -> bool |
| 21 | game_ai::plan_legacy::team_plan::TeamPlan::should_delay_serpen_for_wave_priority | in:game_ai::plan_legacy::team_plan | game-ai\src\plan_legacy\team_plan.rs:574 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData) -> bool |
| 22 | game_ai::plan_legacy::team_plan::TeamPlan::should_delay_object_setup_for_wave_priority | pub | game-ai\src\plan_legacy\team_plan.rs:578 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData) -> bool |
| 23 | game_ai::plan_legacy::team_plan::TeamPlan::should_keep_object_for_contested_wave_priority | in:game_ai::plan_legacy::team_plan | game-ai\src\plan_legacy\team_plan.rs:586 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, game_ai::plan_legacy::team_plan::objective_helpers::WavePriorityObject) -> bool |
| 24 | game_ai::plan_legacy::team_plan::TeamPlan::repair_misunderstood_objective | in:game_ai::plan_legacy::team_plan | game-ai\src\plan_legacy\team_plan.rs:603 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData) -> bool |
| 25 | game_ai::plan_legacy::team_plan::TeamPlan::update_objective | pub | game-ai\src\plan_legacy\team_plan.rs:722 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData) |
| 26 | game_ai::plan_legacy::team_plan::TeamPlan::update_steal | pub | game-ai\src\plan_legacy\team_plan.rs:732 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &game_ai::plan_legacy::types::BigPlan) |
| 27 | game_ai::plan_legacy::team_plan::TeamPlan::update_objective_after_steal | in:game_ai::plan_legacy::team_plan | game-ai\src\plan_legacy\team_plan.rs:910 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData) |
| 28 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v24_objective_setup_lane_pressure_ready | pub | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:7 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, game_core::JungleType, &mut game_core::DebugFrameData) -> bool |
| 29 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v24_objective_setup_should_check_camp | pub | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:88 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, game_core::JungleType) -> bool |
| 30 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v24_objective_setup_should_release_to_passive | pub | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:177 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::PlayerState, &game_core::OperationData, game_core::JungleType) -> bool |
| 31 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v24_objective_setup_relevant_lanes_ready | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:199 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PlayerState, &game_core::OperationData, game_core::JungleType) -> bool |
| 32 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v25_objective_posture | pub | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:207 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::PlayerState, &game_core::OperationData, game_core::JungleType) -> std::option::Option<game_ai::plan_legacy::team_plan::ObjectivePosture> |
| 33 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v27_objective_wait_pos | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:316 | False | fn(game_core::JungleType, usize, &game_core::MapDef) -> (u64, u64) |
| 34 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v27_objective_entity | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:324 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, &game_core::OperationData, game_core::JungleType) -> std::option::Option<&game_core::Entity> |
| 35 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::update_v27_objective_discipline | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:334 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &mut game_core::DebugFrameData) |
| 36 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v27_active_objective_discipline | pub | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:483 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::OperationData, game_core::JungleType) -> std::option::Option<game_ai::plan_legacy::team_plan::ObjectiveDisciplineState> |
| 37 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v27_objective_discipline_blocks_battle | pub | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:497 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::OperationData) -> bool |
| 38 | game_ai::plan_legacy::team_plan::objective_discipline::<impl game_ai::plan_legacy::team_plan::TeamPlan>::v27_objective_discipline_action | pub | game-ai\src\plan_legacy\team_plan\objective_discipline.rs:505 | False | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, game_core::JungleType) -> std::option::Option<game_ai::SmallActionPlay> |
| 39 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_repair_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:7 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PlayerState, &game_core::OperationData) |
| 40 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_defense_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:16 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData) -> bool |
| 41 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_nexus_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:29 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData, game_core::LineType) -> bool |
| 42 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_defense_line_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:46 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData, game_core::LineType) -> bool |
| 43 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_dive_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:88 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData, game_core::LineType) -> bool |
| 44 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_press_tower_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:141 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData, game_core::LineType) -> bool |
| 45 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_press_epic_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:253 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData, game_core::LineType) -> bool |
| 46 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_split_epic_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:334 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData, game_core::LineType) -> bool |
| 47 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_comeback_pick_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:393 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData, game_core::LineType, bool) -> bool |
| 48 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_serpen_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:558 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData, game_ai::plan_legacy::team_plan::ObjectPhase) -> bool |
| 49 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_morgard_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:685 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData, game_ai::plan_legacy::team_plan::ObjectPhase) -> bool |
| 50 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_none_or_gank_objective | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:830 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData) -> bool |
| 51 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_gank_preprocess | in:game_ai | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:1133 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, game_core::LineType) -> bool |
| 52 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_nexus_defense | pub | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:1227 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> bool |
| 53 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_nexus_attack | pub | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:1237 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> std::option::Option<game_core::LineType> |
| 54 | game_ai::plan_legacy::team_plan::objective_handlers::<impl game_ai::plan_legacy::team_plan::TeamPlan>::handle_sub_objective | pub | game-ai\src\plan_legacy\team_plan\objective_handlers.rs:1258 | False | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, &mut game_core::DebugFrameData) |

**`open` 5건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 표기 불가 | team_plan.rs:534 `is_recent_visible_big_action(...) && can_move >= dist_from_last` 의 소스 상 좌우 순서 — IR 은 호출(invoke)을 무조건 먼저 하고 `and` 로 합치므로(25244~25249) 호출이 왼쪽일 가능성이 높으나 column 정보 0 이라 표기 불가(동작은 확정: 둘 다 참일 때만 keep) | 4 |  |
| 1 | 미탐색 | L531 이 캡처 `tick` 대신 game.tick() 을 재호출하는 이유(값은 동일 프레임이면 같음) — 소스 부재 | 4 |  |
| 2 | 미탐색 | is_recent_visible_big_action · unseen_error_radius · unseen_estimated_pos · awareness_lapse_state 내부(계약만 · 각 _gcbc 위치 callee_contracts) — 범위 밖. 특히 unseen_estimated_pos 의 TLS 사용 여부 미조사 | 4 |  |
| 3 | 미탐색 | exe 0xc99fe0 의 상수 0x7d1(2001)·0xbb8(3000)·0x9c41(40001) 의 IR 대응 — IR 클로저 본문엔 없음. 인라인된 gen_range(UniformInt · range 폭 산술)·unseen_error_radius(40k 상수) 조각으로 추정(미검증 · exe 측만) | 4 |  |
| 4 | 미탐색 | `_next\reach\dd9f30.*` 가 옛 라벨(can_near_enemies) 기준 생성물 — 이 함수의 정식 reach 산출물은 스크래치 r18R\range.reach.json · closure.reach.json 에만 있음(메인이 _next 로 재생성 필요) | 4 |  |

**`notes` 1건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | closure0 의 소스 인자 이름 — DI `x = i64 %26`(24994)은 Iterator::find::check 의 `x`(인라인)로 보이며 캡처 좌표 `x` 와 충돌하므로 사용자 클로저 인자명은 미확정(명세는 i 로 표기) | 4 | 사실 서술 |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

