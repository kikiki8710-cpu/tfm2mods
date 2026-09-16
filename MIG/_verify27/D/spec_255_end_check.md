---

### `255` end_check — 판 종료(넥서스 마무리 진입) 판정 — 선수의 game_finish 전략(Stable/Flexible/Aggressive)별로 라인 타워·생존/건강 아군 수·적 넥서스 근접 인원을 세어 bool 반환

| 항목 | 값 |
|---|---|
| id | `end_check` |
| 심볼 | `_RNvCshdEBA0ozCnw_7game_ai9end_check` |
| 소스 | `game-ai\src\lib.rs:1211` |
| IR | `m14.ll` 8724~10217행 |
| 경로·가시성 | `game_ai::end_check` · **in:game_ai** |
| 계층 | 기타 |
| exe | `e7b9f0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r18` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, game_core::LineType, &mut game_core::DebugFrameData) -> bool
```

**TLS 메모 절(`sig.tls` · 정정 경로 `/specs[255]/sig/tls/<키>`)**

- `name`: FINISH_AGG_CACHE(thread_local! RefCell<FinishAggCache> 656B · lib.rs:1376)
- `role`: 이 함수 본문 자체는 TLS 접점 0(m14.ll 8724~10217 에 LocalKey/@anon fn-포인터 상수 참조 없음). TLS 는 콜리 build_game_finish_check_state(m14.ll:7703, fastcc internal) 안에서만 `LocalKey<RefCell<FinishAggCache>>::with(m00.ll:72405 · @anon.b0108….71 = FINISH_AGG_CACHE call_once fn-포인터)` 로 읽고/쓴다 → 소비자(간접)
- `key`: 콜리 계약 밖(미탐색) — FinishAggCache = slots[2][5] Option<FinishAggregates 64B>(니치 태그 +0x39 = has_epic_buff 바이트) 로 보이나(tcxdict --deep) 키 구성(tick/team/pos)은 콜리 명세 소관
- `layout`: FinishAggregates 64B: +0 live_ally_count +8 live_enemy_count +0x10 healthy_ally_50 +0x18 healthy_ally_55 +0x20 near_player_ally +0x28 near_enemy_nexus_healthy_50 +0x30 near_enemy_nexus_healthy_55 +0x38 has_enemy_twin_tower +0x39 has_epic_buff
- `invalidation`: 미탐색(콜리 소관)
- `call_conditions`: end_check 가 콜리를 부르는 지점 2곳: Stable 경로 L1279(m14.ll:8769 · 조건 line_exists && game_finish==Stable) / Aggressive 경로 L1311(m14.ll:10088 · 조건 line_exists && game_finish==Aggressive && (tick>tower_attack_disable_tick || 적 라인 타워 2기 모두 None)). Flexible 경로는 콜리를 부르지 않음(TLS 무관)

<details><summary>인자 6개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | version | usize(i64 %0) | IR 속성 없음 · 본문에서 미사용(DI 이름 `_version` 으로만 남음 · m14.ll:8871) · AI 버전 게이트 — 이 함수엔 분기 없음 | 4 |
| 1 | 2 | rnd | &mut StdRng(320B, ptr %1) | dereferenceable(320) · noalias · IR 속성에 readonly 없음(=&mut) · 본문에서 직접 읽기/쓰기 0 — PlayerState::strategy 에 그대로 전달만(m14.ll:8745) · gen_range 호출 사이트: 이 함수 본문 0회. strategy 콜리 내부 사용 여부는 계약 밖(미탐색) | 4 |
| 2 | 3 | player | &PlayerState(2528B, ptr %2) | readonly · captures(address, read_provenance) · info.team(+0x930) · info.position 태그(+0x9c0) 읽음 · strategy/build_game_finish_check_state 에 전달 | 4 |
| 3 | 4 | data | &OperationData(24B, ptr %3) | readonly · captures(none) · +0x0 cache(&AbstractGameWithCache) · +0x8 context(&GameContext) · +0x10 blackboard(&[Blackboard;2]) | 4 |
| 4 | 5 | line | LineType(i8 %4, range 0..3) | 값 인자 · 메모리태그 0=Top 1=Mid 2=Bottom(tcxdict --enum LineType) — 라인 타워 쌍 선택·blackboard minion_state 선택 | 3 |
| 5 | 6 | debug | &mut DebugFrameData(224B, ptr %5) | readnone · captures(none) — 본문에서 완전 미사용 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn end_check(version, rnd, player, data, line, debug) -> bool   // lib.rs:1211
ctx = data.context; cache = data.cache; game = cache.game(&dyn AbstractGame)
L1212: if !rule_scope::line_exists(ctx, line) { return false }           // m14.ll:8736~8737, %20 phi false
L1216: strat = player.strategy(rnd, game)  (sret Strategy 24B)  ; match strat.game_finish(+0x15) {

== Stable (tag 0) → end_check_stable(player, data, line)  lib.rs:1279~1302 ==
L1279: state = build_game_finish_check_state(player, data, line)?   // None(+68==2) → return false (m14.ll:8772→%47)
L1280: required_allies = ctx.tutorial.player_count()   // None/Line/Total=4 · First/Bottom=2 · TopSolo/MidSolo/JungleOnly=1 · MidBottom=3
L1282: if (state.line_tower_alive(raw u8 +65) != state.has_enemy_twin_tower(raw u8 +64)) || state.line_tower_alive { return false }
        // IR 문면 `icmp ne %40,%38 ; or ..., %41`(m14.ll:8838~8840). 불 대수상 line_tower_alive || has_enemy_twin_tower 와 동치. 소스 표기는 미확정(표기 불가)
L1286: if !(state.live_ally_count >= required_allies && state.healthy_ally_55_count >= state.live_ally_count && state.near_player_ally_count == state.live_ally_count && state.stable_pushed_line) { return false }   // m14.ll:8843~8849
L1298: match state.live_enemy_count { 0 => return true,                                            // m14.ll:8853
L1302:   1 => return state.near_enemy_nexus_healthy_ally_55_count >= required_allies,               // m14.ll:8858
         _ => return false }                                                                        // %65

== Flexible (tag 1) → end_check_flexible(player, data, line)  lib.rs:1225~1271 ==
team = player.info.team; enemy = 1 - team; pos = player.info.position as usize
L1225: if !(game.tick() > setting.tower_attack_disable_tick) {
L1226:   (t1, t2) = cache.tower(line, enemy)   // (top|mid|bottom)_tower[enemy], _tower2[enemy]
L1230:   if !(t1.is_none() && t2.is_none()) { return false } }   // m14.ll:8942~8948 → %620 false
L1234: enemy_nexus = cache.nexus[enemy].unwrap()   // None → unwrap_failed 패닉
L1235~1236: live_enemy_count = cache.iter_champions(enemy).count()   // player_champion[enemy] 의 Some 개수(5칸 완전 언롤)
L1237: has_enemy_twin_tower = if game.tick() > setting.tower_attack_disable_tick { false } else {
L1238:     cache.twin_towers[enemy].len() != 0 }
L1243: champ = cache.player_champion[team][pos].unwrap()   // None → unwrap_failed 패닉
L1244: near_ally = cache.iter_champions(team).filter(|a| a.distance_sq(champ) < 1440000000001).count()   // 자기 자신 포함(dist 0)
L1247: if near_ally != cache.iter_champions(team).count() { return false }   // 생존 아군 전원이 1,200,000 안에 모여야 함 (m14.ll:9433~9434)
L1253: ok_ally = cache.iter_champions(team).filter(|a| a.hp*100 / a.stat_cached.hp > 39).count()   // stat_cached.hp==0 → div_by_zero 패닉
L1254: epic_remain = game.get_game_mode().as_moba().map_or(0, |m| m.remain_epic_time(team))   // = MobaMode.epic_minion_buff_time[team]
       if epic_remain < setting.tick_per_second * 20 {
L1260:   if live_enemy_count == 0 { return true }  if has_enemy_twin_tower { return false }     // m14.ll:9647~9649 (%382: 반환값 = live_enemy==0)
       } else {
L1255:   cond = (ok_ally >= live_enemy_count.saturating_sub(1) && blackboard[team].minion_state(line).minion_count > 5) || live_enemy_count == 0
         if cond { return true }  if has_enemy_twin_tower { return false }                        // m14.ll:9675~9681 (%393: 반환값 = cond)
       }
L1266~1269: required_edge = match ctx.tutorial.player_count() { 2 => 1, 3 => 2, _ => 3 }   // 1명·4명 모드 모두 3 (m14.ll:9687~9708)
L1271: near_nexus = cache.iter_champions(team).filter(|a| a.distance_sq(enemy_nexus) < 360000000001).count()
       return near_nexus >= required_edge + live_enemy_count                                       // m14.ll:10010~10011

== Aggressive (tag 2) → end_check_aggressive(player, data, line)  lib.rs:1307~1342 ==
L1307: if !(game.tick() > setting.tower_attack_disable_tick) {
L1308:   (t1,t2) = cache.tower(line, 1-team); if !(t1.is_none() && t2.is_none()) { return false } }   // m14.ll:10079~10084
L1311: state = build_game_finish_check_state(player, data, line)?   // None → false (m14.ll:10091→%590)
L1312: required_group = match player_count { 4 => 3, 3 => 3, 2 => 2, 1 => 1 }  (= min(player_count,3) · m14.ll:10153)
L1314: if state.line_tower_alive { return false }
L1318: if !(state.healthy_ally_50_count >= required_group && state.near_finish_line_healthy_ally_50_count >= required_group && state.aggressive_pushed_line) { return false }   // m14.ll:10158~10162
L1326: if !(state.live_enemy_count != 0 && state.healthy_ally_50_count < state.live_enemy_count + 2) { return true }   // 적 전멸 또는 건강 아군이 적+2 이상 (m14.ll:10165~10169)
L1334: if !state.has_enemy_twin_tower {
L1335:   if state.healthy_ally_50_count > state.live_enemy_count && state.near_enemy_nexus_healthy_ally_50_count >= required_group { return true } }   // m14.ll:10175~10178
L1340: if state.healthy_ally_50_count > state.live_enemy_count && state.has_epic_buff {
L1342:   return state.near_finish_line_healthy_ally_50_count >= ctx.tutorial.player_count() }   // 4/2/1/3 원값 (m14.ll:10209~10210)
       return false
}

분기 극성 근거: %620 phi(m14.ll:10214) — false:{%275,%47,%51,%55,%65,%90,%590,%553,%593,%610,%595} true:{%62,%600,%606} 값:{%63→%64, %382→%383, %527→%530, %393→%402, %617→%619}.
rnd: 본문 gen_range 0회(strategy 에 전달만). version/debug 미사용.
```

**`mem` 메모리 접근 42건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | OperationData | 0x0 | cache | r | &AbstractGameWithCache(m14.ll:8741) | 4 | OK |
| 1 | OperationData | 0x8 | context | r | &GameContext(m14.ll:8735) | 4 | OK |
| 2 | OperationData | 0x10 | blackboard | r | &[Blackboard;2] — Flexible L1255 에서만(m14.ll:9652) | 4 | OK |
| 3 | AbstractGameWithCache | 0x0 | game.data_ptr | r | &dyn AbstractGame 데이터 포인터(m14.ll:8742) | 4 | OK |
| 4 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r | vtable — 슬롯 0x28 tick(m14.ll:8889) · 0x40 get_game_mode(m14.ll:9612) (divtable AbstractGame) | 3 | OK |
| 5 | AbstractGameWithCache | 0x148 | twin_towers[enemy].len | r | `gep {{ptr,ptr,i64},i64} cache, enemy`(stride 32B) + 328 → twin_towers[0].len=0x148 / [1]=0x168 (m14.ll:9056~9060) · has_enemy_twin_tower = len != 0 | 4 | OK |
| 6 | AbstractGameWithCache | 0x170 | nexus[enemy] | r | Option<&Entity> · None 이면 unwrap_failed 패닉(L1234 · m14.ll:8960~8977) | 4 | OK |
| 7 | AbstractGameWithCache | 0x180 | top_tower[enemy] | r | line=Top 일 때 phi 384 (m14.ll:8934) | 4 | OK |
| 8 | AbstractGameWithCache | 0x190 | top_tower2[enemy] | r | line=Top 일 때 phi 400 (m14.ll:8935) | 4 | OK |
| 9 | AbstractGameWithCache | 0x1a0 | mid_tower[enemy] | r | line=Mid 일 때 phi 416 | 4 | OK |
| 10 | AbstractGameWithCache | 0x1b0 | mid_tower2[enemy] | r | line=Mid 일 때 phi 432 | 4 | OK |
| 11 | AbstractGameWithCache | 0x1c0 | bottom_tower[enemy] | r | line=Bottom 일 때 phi 448 | 4 | OK |
| 12 | AbstractGameWithCache | 0x1d0 | bottom_tower2[enemy] | r | line=Bottom 일 때 phi 464 | 4 | OK |
| 13 | AbstractGameWithCache | 0x1e0 | player_champion[team][0..5] | r | [[Option<&Entity>;5];2] · `gep [5 x ptr], cache+480, team` (m14.ll:8981~9045) — Flexible 경로에서 적/아군 챔피언 순회(iter_champions 인라인) | 4 | OK |
| 14 | GameContext | 0x8 | setting | r | &GameSetting(m14.ll:8892) | 4 | OK |
| 15 | GameContext | 0x38 | tutorial@tag | r | TutorialType 1B — TutorialType::player_count(runner.rs:295) 인라인 switch(m14.ll:8808~8820, 9685~9697, 10128~10140, 10187~10197) | 4 | OK |
| 16 | GameSetting | 0x13f8 | tower_attack_disable_tick | r | tick > 이 값이면 라인 타워 검사 생략(m14.ll:8894, 10029) | 4 | OK |
| 17 | GameSetting | 0x12f8 | tick_per_second | r | ×20 = 에픽 버프 잔여 임계(m14.ll:9640) | 4 | OK |
| 18 | PlayerState | 0x930 | info.team | r | enemy = 1 - team (m14.ll:8865, 10035) | 4 | OK |
| 19 | PlayerState | 0x9c0 | info.position@tag | r | Position 4B 태그(Top0 Jungle1 Mid2 Bottom3 Support4) → player_champion[team][pos] 인덱스(m14.ll:8867, 9069) | 4 | OK |
| 20 | Entity | 0x660 | x | r | distance_sq(entity.rs:2158→utils.rs:7) 인라인 | 4 | OK |
| 21 | Entity | 0x668 | y | r |  | 4 | OK |
| 22 | Entity | 0x628 | stat_cached.hp | r | 최대 HP — 0 이면 panic_const_div_by_zero(m14.ll:9456~9460) | 4 | OK |
| 23 | Entity | 0x670 | hp | r | hp*100/stat_cached.hp > 39 (m14.ll:9464~9468) | 4 | OK |
| 24 | MobaMode | 0x240 | epic_minion_buff_time[team] | r | MobaMode::remain_epic_time(game.rs:210) 인라인 · `gep i64, mode+576, team` (m14.ll:9633~9635) | 4 | OK |
| 25 | Blackboard | 0x20 | top_minion_state.minion_count | r | i32 · blackboard[team](stride 744B) + line 선택(+0/+40/+80 · blackboard.rs:379~382 인라인) + 0x20 (m14.ll:9654~9673) | 4 | OK |
| 26 | Blackboard | 0x48 | mid_minion_state.minion_count | r | line=Mid — gep 40 + gep 32 두 단계로 접힘(본문에 72 리터럴은 alloca 크기뿐) | 4 | OK |
| 27 | Blackboard | 0x70 | bottom_minion_state.minion_count | r | line=Bottom — gep 80 + gep 32 두 단계로 접힘(본문에 112 리터럴 없음 · C3 경고 사유) | 4 | OK |
| 28 | Strategy(sret 24B 스택 %9) | 0x15 | game_finish | r | GameFinishStrategy 1B: 0=Stable 1=Flexible 2=Aggressive (m14.ll:8746~8752) | 4 | OK |
| 29 | Option<GameFinishCheckState>(스택 %8/%7 72B) | 0x44 | has_epic_buff / 니치 태그 | r | 바이트 2 = None(콜리 m14.ll:7763 `store i8 2` 로 확인) · Some 이면 bool | 4 | 확인불가(tcx 사전에 타입 없음) |
| 30 | GameFinishCheckState | 0x0 | live_ally_count | r | Stable | 4 | OK |
| 31 | GameFinishCheckState | 0x8 | live_enemy_count | r | Stable/Aggressive | 4 | OK |
| 32 | GameFinishCheckState | 0x10 | healthy_ally_50_count | r | Aggressive | 4 | OK |
| 33 | GameFinishCheckState | 0x18 | healthy_ally_55_count | r | Stable | 4 | OK |
| 34 | GameFinishCheckState | 0x20 | near_player_ally_count | r | Stable | 4 | OK |
| 35 | GameFinishCheckState | 0x28 | near_enemy_nexus_healthy_ally_50_count | r | Aggressive | 4 | OK |
| 36 | GameFinishCheckState | 0x30 | near_enemy_nexus_healthy_ally_55_count | r | Stable | 4 | OK |
| 37 | GameFinishCheckState | 0x38 | near_finish_line_healthy_ally_50_count | r | Aggressive | 4 | OK |
| 38 | GameFinishCheckState | 0x40 | has_enemy_twin_tower | r | Stable/Aggressive | 4 | OK |
| 39 | GameFinishCheckState | 0x41 | line_tower_alive | r | Stable/Aggressive | 4 | OK |
| 40 | GameFinishCheckState | 0x42 | stable_pushed_line | r | Stable | 4 | OK |
| 41 | GameFinishCheckState | 0x43 | aggressive_pushed_line | r | Aggressive | 4 | OK |

**`consts` 상수 16건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 2 | 1279 | 센티널 | Option<GameFinishCheckState> 니치 태그 None(+68 바이트 == 2) — L1279(m14.ll:8772)·L1311(m14.ll:10091) 두 곳. 별도로 L1326 `healthy_ally_50 < live_enemy + 2`(m14.ll:10166) 의 2, player_count 매핑값 2(First/Bottom), required_edge/required_group 2 도 같은 리터럴 | 4 |
| 1 | 4 | 1280 | 태그 | TutorialType::player_count 인라인 결과 — None(0)/Line(7)/Total(8) → 4명 (phi m14.ll:8836·10209) | 4 |
| 2 | 3 | 1280 | 태그 | player_count: MidBottom(5) → 3명 · required_edge 기본값(L1266~1269) · required_group 상한(L1312 · player_count 4→3) | 4 |
| 3 | 1 | 1280 | 태그 | player_count: TopSolo(2)/MidSolo(4)/JungleOnly(6) → 1명 · required_edge(2명 모드) · required_group(1명 모드) · L1298 `live_enemy == 1` switch case · L1255 saturating_sub(1) | 4 |
| 4 | 0 | 1298 | 태그 | L1298 switch live_enemy_count==0 → true / L1326·L1260·L1255 `live_enemy_count == 0` / L1238 twin_towers.len != 0 / L1253 stat_cached.hp==0 div-by-zero 가드 / map_or 기본값 0(L1254) | 4 |
| 5 | 1440000000001 | 1244 | 임계 | 1200000^2 + 1 — 챔피언 기준 아군 근접 반경 1,200,000(=37.5셀) 제곱비교 `dist2 < 1200000^2+1` ⇔ dist ≤ 1,200,000 (m14.ll:9152·9209·9267·9325·9383) | 4 |
| 6 | 360000000001 | 1271 | 임계 | 600000^2 + 1 — 적 넥서스 기준 아군 근접 반경 600,000(=18.75셀) 제곱비교 (m14.ll:9779·9833·9888·9943·9998) | 4 |
| 7 | 100 | 1253 | 계수 | hp*100/max_hp 백분율 산출(m14.ll:9466) | 4 |
| 8 | 39 | 1253 | 임계 | hp% > 39 ⇔ HP 40% 이상을 ok_ally 로 셈(m14.ll:9468) | 4 |
| 9 | 20 | 1254 | 계수 | tick_per_second * 20 = 20초 — 에픽 미니언 버프 잔여시간 임계(m14.ll:9642) | 4 |
| 10 | 5 | 1255 | 임계 | blackboard[team].<line>_minion_state.minion_count > 5 (i32 sgt · m14.ll:9674) | 4 |
| 11 | 40 | 1255 | 미상 | Blackboard 라인별 BrainMinionParameter stride(mid=+40 · m14.ll:9663) — 임계 아님 | 4 |
| 12 | 80 | 1255 | 미상 | bottom=+80 (m14.ll:9667) — 임계 아님 | 4 |
| 13 | 32 | 1255 | 미상 | BrainMinionParameter+0x20 minion_count 필드 오프셋(m14.ll:9672) — 임계 아님 | 4 |
| 14 | 576 | 1254 | 미상 | MobaMode+0x240 epic_minion_buff_time 필드 오프셋(m14.ll:9633) — 임계 아님 | 4 |
| 15 | 328 | 1238 | 미상 | cache+32*enemy+328 → twin_towers[enemy].len(0x148/0x168) — 임계 아님 | 4 |

**`knobs` 조정점 7건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | Flexible 아군 집결 반경(챔피언 기준) | lib.rs:1244 | 1440000000001 | 올리면 더 흩어져 있어도 '전원 근접' 으로 인정돼 finish 진입이 쉬워진다(1,200,000 = 37.5셀) | 4 | 기존 |
| 1 | Flexible 적 넥서스 근접 반경 | lib.rs:1271 | 360000000001 | 올리면 넥서스에서 더 멀어도 near_nexus 로 세어 finish 진입이 쉬워진다(600,000 = 18.75셀) | 4 | 기존 |
| 2 | Flexible ok_ally HP 임계 | lib.rs:1253 | 39 | 내리면 저체력 아군도 ok_ally 로 세어 에픽 버프 중 조기 finish 가 쉬워진다 | 4 | 기존 |
| 3 | 에픽 버프 잔여 임계(초) | lib.rs:1254 | 20 | 내리면 버프가 얼마 안 남아도 L1255 완화 경로(ok_ally·미니언 5+)를 탄다 | 4 | 기존 |
| 4 | Flexible 라인 미니언 수 임계 | lib.rs:1255 | 5 | 내리면 웨이브가 작아도 에픽 버프 경로에서 true | 4 | 기존 |
| 5 | Flexible required_edge 매핑 | lib.rs:1266~1269 | 2명→1 · 3명→2 · 그 외→3 | 내리면 넥서스 근접 인원 요구(required_edge+live_enemy)가 줄어 finish 진입이 쉬워진다 | 4 | 기존 |
| 6 | Aggressive 적 여유 인원 | lib.rs:1326 | 2 | 내리면 healthy_ally_50 이 적보다 덜 많아도 즉시 true | 4 | 기존 |

<details><summary>`callees` 피호출자 22건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | as_moba | game_core::GameMode::<'a>::as_moba | pub | fn(game_core::GameMode<'a/#0>) -> std::option::Option<&'a/#0 game_core::MobaMode> | game-core\src\simulation\game.rs:230 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 1 | build_game_finish_check_state | game_ai::build_game_finish_check_state | in:game_ai | fn(&game_core::PlayerState, &game_core::OperationData, game_core::LineType) -> std::option::Option<game_ai::GameFinishCheckState> | game-ai\src\lib.rs:1390 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 2 | distance_sq | game_core::utils::distance_sq | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:6 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 3 | distance_sq | game_core::Entity::distance_sq | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2157 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 4 | end_check | game_ai::end_check | in:game_ai | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, game_core::LineType, &mut game_core::DebugFrameData) -> bool | game-ai\src\lib.rs:1211 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 5 | end_check_aggressive | game_ai::end_check_aggressive | in:game_ai | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, game_core::LineType, &mut game_core::DebugFrameData) -> bool | game-ai\src\lib.rs:1305 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 6 | end_check_flexible | game_ai::end_check_flexible | in:game_ai | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, game_core::LineType, &mut game_core::DebugFrameData) -> bool | game-ai\src\lib.rs:1223 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 7 | end_check_stable | game_ai::end_check_stable | in:game_ai | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, game_core::LineType, &mut game_core::DebugFrameData) -> bool | game-ai\src\lib.rs:1278 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 8 | get_game_mode | game_core::AbstractGame::get_game_mode | pub | fn(&Self/#0) -> game_core::GameMode | game-core\src\simulation.rs:78 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 9 | get_game_mode | <game_core::Game as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::Game) -> game_core::GameMode | game-core\src\simulation\game.rs:1707 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 10 | get_game_mode | <game_core::SingleLaneGame as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::SingleLaneGame) -> game_core::GameMode | game-core\src\simulation\game.rs:4248 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 11 | iter_champions | game_core::AbstractGameWithCache::<'a, 'b>::iter_champions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5759 ~ game_core[6a30]::simulation::{impl#5}::iter_champions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1904 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 12 | line_exists | game_ai::plan_legacy::rule_scope::line_exists | pub | fn(&game_core::GameContext, game_core::LineType) -> bool | game-ai\src\plan_legacy\rule_scope.rs:24 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 13 | minion_state | game_core::Blackboard::minion_state | pub | fn(&game_core::Blackboard, game_core::LineType) -> &game_core::BrainMinionParameter | game-core\src\simulation\game\blackboard.rs:378 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 14 | player_count | game_core::GameRule::player_count | pub | fn(&game_core::GameRule) -> usize | game-core\src\data\match_info.rs:542 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 15 | player_count | game_core::TutorialType::player_count | pub | fn(&game_core::TutorialType) -> usize | game-core\src\simulation\game\runner.rs:294 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 16 | remain_epic_time | game_core::MobaMode::remain_epic_time | pub | fn(&game_core::MobaMode, usize) -> usize | game-core\src\simulation\game.rs:210 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 17 | strategy | game_core::PlayerState::strategy | pub | fn(&game_core::PlayerState, &mut rand::rngs::std::StdRng, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + ) -> game_core::Strategy | game-core\src\simulation\state\player.rs:1576 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 18 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 19 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 20 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 21 | tower | game_core::AbstractGameWithCache::<'a, 'b>::tower | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, game_core::LineType, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:1822 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
</details>

⚠**미매칭 5개**: `game_finish`, `has_enemy_twin_tower`, `line_tower_alive`, `llvm.usub.sat.i64`, `map_or`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 4곳** (m09.ll:8281, m09.ll:33040, m09.ll:33131, m09.ll:34165) · **형제 0개** 

**`open` 7건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 표기 불가 | L1282 의 소스 표기: IR 은 `(line_tower_alive != has_enemy_twin_tower) \|\| line_tower_alive`(m14.ll:8838~8840) 로 나오며 불 대수상 `line_tower_alive \|\| has_enemy_twin_tower` 와 외연 동일 — 소스가 어느 쪽인지는 표기 불가(IR·MIR(mir=0)·오라클 어디에도 차이가 안 남음). 동작은 확정 | 2 |  |
| 1 | 표기 불가 | L1255/L1260 의 소스 문장 구조: 두 블록(%382·%393) 모두 '조건 true → return true; has_enemy_twin_tower → return false; 아니면 L1266 진행' 로 동작이 확정되지만, 소스에서 `if … { return … }` 가 몇 문장인지는 표기 불가(column 0) | 4 |  |
| 2 | 미탐색 | PlayerState::strategy(_gcbc g15.ll:130480 · (sret Strategy 24B, &PlayerState, &mut StdRng, &dyn AbstractGame(data,vtable)) 내부에서 rnd 를 소비하는지 — 계약만(이 명세 범위 밖). 소비한다면 end_check 가 PRNG 스트림을 전진시키는 지점이 된다 | 4 |  |
| 3 | 미탐색 | build_game_finish_check_state(m14.ll:7703 · (out 72B, &PlayerState, &OperationData, LineType) → Option<GameFinishCheckState>, None 니치 +68==2) 의 각 필드 계산·TLS FinishAggCache 키/무효화 — 계약만(지시: DIFF 0 · 계약만) | 4 |  |
| 4 | 표기 불가 | TutorialType::player_count(runner.rs:295~301) 은 인라인(별도 define 없음) — 매핑은 4개 switch 사이트에서 일관되게 관측(None/Line/Total=4, First/Bottom=2, TopSolo/MidSolo/JungleOnly=1, MidBottom=3)되어 확정. 단 소스의 match arm 순서는 표기 불가 | 4 |  |
| 5 | 미탐색 | constants 의 2·1·0·3·4 는 여러 의미(니치 None·player_count 결과·산술)가 같은 리터럴을 공유 — meaning 에 사이트별로 분리 기재 | 4 |  |
| 6 | 미탐색 | vtable 슬롯 0x28=tick, 0x40=get_game_mode 는 divtable(정적 vtable ExpectedGame 기준) — 런타임 구현체(Game)에서도 같은 슬롯 순서라는 가정(트레이트 vtable 은 구현체 무관하게 동일 순서이므로 안전) | 3 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

