---

### `53` check_epic_setup — 에픽(모르가드) 셋업 가능 판정 — 생존/근접 아군·적 수·라인 밀림·판단 페널티로 bool 을 낸다

| 항목 | 값 |
|---|---|
| id | `epic__check_epic_setup` |
| 심볼 | `_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old4epic16check_epic_setup` |
| 소스 | `game-ai\src\plan_legacy\old\epic.rs:16` |
| IR | `m09.ll` 57082~59016행 |
| 경로·가시성 | `game_ai::plan_legacy::old::check_epic_setup` · **pub** |
| 계층 | 레거시 플랜 |
| exe | `de6ce0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r8` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bool
```

<details><summary>인자 6개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | version | usize | 본문 분기 없음. macro_judgement_penalty(version, player) 에만 전달 | 4 |
| 1 | 2 | _rnd | &mut StdRng(320B, align16) | 이름이 _rnd — 본문에서 전혀 안 씀(readnone) | 4 |
| 2 | 3 | player | &PlayerState(2528B) | info.team(+0x930), info.position 태그(+0x9c0, 디버그 로그 전용), is_recent_visible 인자 | 4 |
| 3 | 4 | data | &OperationData(24B) | {0x0 cache, 0x8 context, 0x10 blackboard:&[Blackboard;2]} | 4 |
| 4 | 5 | team_plan | &TeamPlan(1064B) | readonly. obj_spawn.epic_giveup_tick 태그 · vision.last_visible_pos[p] · vision.last_checked_ticks[p] 만 읽음 | 4 |
| 5 | 6 | debug | &mut DebugFrameData(224B) | context.debug 일 때만 +0xa0 infos 맵에 문자열 push | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn check_epic_setup(version, _rnd, player, data, team_plan, debug) -> bool

[L17] context = data.context(+0x8)
      if context.tutorial(+0x38) ∈ {1..=6} (First/TopSolo/Bottom/MidSolo/MidBottom/JungleOnly) { return false }
      // 튜토리얼 None(0)/Line(7)/Total(8) 만 통과

[L21] mode = cache.game.get_game_mode()  (vtable+0x40)   // GameMode 16B
      moba = mode.Moba 페이로드(+0x8) — 태그(+0x0) != 0 이면 unwrap_failed 패닉
      if moba.jungle_runner.epic.live_list.len(+0x1a8) == 0 {      // 에픽이 살아있지 않으면
[L22]   remain_spawn_tick = moba.jungle_runner.epic.next_respawn_tick(+0x1b0).saturating_sub(game.tick())
[L25]   if remain_spawn_tick >= tps(+0x12f8) * 15 { return false }  // 15초 이상 남았으면 아직
      }

[L31] if team_plan.obj_spawn.epic_giveup_tick(+0x50 태그).is_some() { return false }

[L35] team = player.info.team(+0x930)            // >=2 면 panic_bounds_check
      live_ally_count  = cache.player_champion(+0x1e0)[team].iter().filter(Some).count()
[L36] if live_ally_count < 3 { return false }
[L40] live_enemy_count = cache.player_champion[1-team] 의 Some 개수

// ── 근접 아군 (L42~45) ───────────────────────────────
[L44] near_epic_ally = allies.filter(|c| {
        (x,y) = (c.x(+0x660), c.y(+0x668))
        // 인라인된 map_regions::is_top_side(context,x,y) (map_regions.rs:22~24): ry = setting.height(+0x12c0) - y; top = (x > ry)
        // 분기: top 이면 is_near_mid_line(context,x,y) 가 true 여야 통과, top 이 아니면 그냥 통과
        (!is_top_side(x,y) || is_near_mid_line(context,x,y))
[L45]   && c.hp(+0x670)*100 / c.stat_cached.hp(+0x628) > 49        // stat_cached.hp==0 이면 div_by_zero 패닉
      }).count()

[L47] camp = MapDef::camp_pos(context.map(+0x20), JungleType::Morgard(4), team == 0)   // (x,y)

// ── 비시야인데 도달 가능한 적 (L48~56) ───────────────
[L48] out_vision = enemies.enumerate().filter(|(p, c)| {
[L51]   last_pos  = team_plan.vision.last_visible_pos[p] (+0x230+16p)
[L52]   d         = utils::distance(last_pos, camp).saturating_sub(150000)
[L53]   move_speed= c.stat_cached.move_speed(+0x640)
[L54]   can_move  = game.tick().saturating_sub(team_plan.vision.last_checked_ticks[p] (+0x2d0+8p)) * move_speed
[L56]   c.hp*100/c.stat_cached.hp > 49
          && can_move >= d                                       // 마지막 확인 이후 캠프까지 올 수 있었던 시간
          && !blackboard[1-team].is_recent_visible(game, player, c)
      }).count()

// ── 시야에 잡힌 캠프 근처 적 (L60~62) ────────────────
[L61] in_vision = enemies.filter(|c| {
        distance_sq(c.xy, camp) < 22500000001   // 150000 이내 (utils.rs:6~9 인라인, abs_diff^2 합)
[L62]   && c.hp*100/c.stat_cached.hp > 49
        && blackboard[1-team].is_recent_visible(game, player, c)
      }).count()
[L64] near_epic_enemy = in_vision + out_vision

[L66] if context.debug(+0x3b) {
[L67]   champ = cache.player_champion[team][player.info.position(+0x9c0)].unwrap()
[L68]   debug.infos(+0xa0).entry(champ.id(+0x5c0)).or_default().push(format!(<anon.307>, near_epic_ally, near_epic_enemy))
      }

// ── 라인 밀림 (L72~74) ───────────────────────────────
[L72] lines: &[LineType] = match context.tutorial {
        None(0) | Line(7) | Total(8) => [Top, Mid, Bottom]   (anon.291 = 00 01 02)
        First(1) | Bottom(3)         => [Bottom]              (anon.35  = 02)
        TopSolo(2)                   => [Top]                 (anon.36  = 00)
        MidSolo(4)                   => [Mid]                 (anon.31  = 01)
        MidBottom(5)                 => [Mid, Bottom]         (anon.290 = 01 02)
        JungleOnly(6)                => []                    (도달 불가 — L17 에서 이미 false)
      }
[L73] // 이터레이션: blackboard[team] + {Top:+0x0, Mid:+0x28, Bottom:+0x50}.minion_count(+0x20, i32) < -3 인 라인을 찾으면 중단
      no_line_pushed(%710) = lines.iter().all(|l| bb[team].<l>_minion_state.minion_count >= -3)
      // dbg 는 line_pushed = !%710 (DW_OP_not 1회) — 즉 line_pushed = '어느 라인이든 minion_count < -3'. 판정은 분기 방향으로 읽음

[L75] judgement_penalty = macro_judgement_penalty(version, player)   // i32
[L76] setup_score = (near_epic_ally - near_epic_enemy) * 2            // shl 1
                  + (live_ally_count - live_enemy_count)
                  + (no_line_pushed ? 2 : 0)
[L79] if judgement_penalty != 0 && setup_score < judgement_penalty { return false }   // dbg: judgement_ready = !(score < penalty)

[L84] if near_epic_ally < near_epic_enemy {
[L100]  if out_vision > 2 {
[L101]    live_ally_count = 재계산(같은 식)
[L102]    return live_ally_count > 2
        }
        return false
      }
[L86] if live_ally_count + 1 < live_enemy_count { return false }
      if no_line_pushed { return true }                       // %710 → 곧바로 true (L86 위치)
[L90] if near_epic_ally > near_epic_enemy {
[L91]   if live_ally_count < live_enemy_count {
[L94]     return near_epic_ally > near_epic_enemy + 1
        }
        return true
      }
[L97] return live_ally_count > live_enemy_count               // near_epic_ally == near_epic_enemy

※ 반환 phi(%62): [false %721][false %740][%767 %763][true %772][%776 %774][%771 %770][%762 %742]
```

**`mem` 메모리 접근 30건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | OperationData | 0x8 | context | r | &GameContext | 4 | OK |  |
| 1 | OperationData | 0x0 | cache | r | &AbstractGameWithCache — cache+0x0/+0x8 = &dyn AbstractGame (data, vtable) | 4 | OK |  |
| 2 | OperationData | 0x10 | blackboard | r | &[Blackboard;2](744B 각). [1-team] 을 is_recent_visible 의 self 로, [team] 을 라인 밀림 판정에 씀 | 4 | OK |  |
| 3 | GameContext | 0x38 | tutorial (TutorialType 태그, i8) | r | L17: (tag-1) < 6 즉 태그 1..=6(First~JungleOnly) 이면 즉시 false. L72: 라인 목록 선택 switch 에도 재사용 | 4 | OK |  |
| 4 | GameContext | 0x3b | debug (bool) | r | L66: true 면 디버그 로그 push | 4 | OK |  |
| 5 | GameContext | 0x8 | setting | r | &GameSetting | 4 | OK |  |
| 6 | GameContext | 0x20 | map | r | &MapDef — camp_pos 의 self | 4 | OK |  |
| 7 | GameSetting | 0x12f8 | tick_per_second | r | IR 4856. tps*15 스폰 대기 임계의 단위 | 4 | OK |  |
| 8 | GameSetting | 0x12c0 | height (u64) | r | IR 4800. 인라인된 map_regions::is_top_side 가 ry = height - y 로 씀 | 4 | OK |  |
| 9 | dyn AbstractGame vtable | 0x40 | get_game_mode | r | divtable: 슬롯 0x40 = get_game_mode → GameMode(16B enum: 0x0 태그 8B, 0x8 &MobaMode) | 3 | 확인불가(vtable 슬롯(구조체 아님) — divtable.py 소관) |  |
| 10 | dyn AbstractGame vtable | 0x28 | tick | r | divtable: 슬롯 0x28 = tick. L22 remain_spawn_tick 과 L54 can_move 계산에 호출 | 3 | 확인불가(vtable 슬롯(구조체 아님) — divtable.py 소관) |  |
| 11 | GameMode | 0x0 | tag | r | 0 = Moba 라야 함. 아니면 unwrap_failed 패닉(L21) | 4 | OK |  |
| 12 | MobaMode | 0x1a8 | jungle_runner.epic.live_list.len | r | IR 424. 0 이면 에픽 미생존 → 스폰 대기 검사로 | 4 | OK |  |
| 13 | MobaMode | 0x1b0 | jungle_runner.epic.next_respawn_tick | r | IR 432. remain_spawn_tick = next_respawn_tick.saturating_sub(tick) | 4 | OK |  |
| 14 | TeamPlan | 0x50 | obj_spawn.epic_giveup_tick@tag | r | IR 80. Option<usize> 태그 != 0(Some) 이면 false 반환(L31) | 4 | OK |  |
| 15 | PlayerState | 0x930 | info.team | r | IR 2352. usize, >=2 면 panic_bounds_check(len 2) | 4 | OK |  |
| 16 | PlayerState | 0x9c0 | info.position@tag (i32) | r | IR 2496. 디버그 로그에서 자기 챔프(player_champion[team][position]) 를 찾는 인덱스 | 4 | OK |  |
| 17 | AbstractGameWithCache | 0x1e0 | player_champion | r | IR 480. [team]·[1-team] 을 stride 40 으로. non-null 개수 = live 카운트 | 4 | OK |  |
| 18 | Entity | 0x660 | x | r | IR 1632 | 4 | OK |  |
| 19 | Entity | 0x668 | y | r | IR 1640 | 4 | OK |  |
| 20 | Entity | 0x628 | stat_cached.hp (최대 HP) | r | IR 1576. 0 이면 div_by_zero 패닉. hp*100/이 값 > 49 = 'HP 절반 이상' | 4 | OK |  |
| 21 | Entity | 0x670 | hp | r | IR 1648 | 4 | OK |  |
| 22 | Entity | 0x640 | stat_cached.move_speed | r | IR 1600. can_move = (tick - last_checked_ticks[p]) * move_speed | 4 | OK |  |
| 23 | Entity | 0x5c0 | id | r | IR 1472. 디버그 로그 맵의 키 | 4 | OK |  |
| 24 | TeamPlan | 0x230 | vision.last_visible_pos[p] (u64,u64) | r | IR 560+16p (560..632). 적 p 의 마지막 관측 좌표 — camp 까지 거리의 출발점 | 4 | OK |  |
| 25 | TeamPlan | 0x2d0 | vision.last_checked_ticks[p] | r | IR 720+8p (720..752). 적 p 를 마지막으로 확인한 틱 | 4 | OK |  |
| 26 | Blackboard | 0x0 | top_minion_state (BrainMinionParameter 40B) | r | LineType Top(0) → +0x0, Mid(1) → +0x28, Bottom(2) → +0x50 | 4 | OK |  |
| 27 | Blackboard | 0x20 | top_minion_state.minion_count (i32) | r | IR 각 라인 구조체 +32. < -3 이면 그 라인 '밀림' (Mid=+0x48, Bottom=+0x70) | 4 | OK |  |
| 28 | DebugFrameData | 0xa0 | infos (HashMap<usize, Vec<String>>) | r | IR 160. context.debug 일 때 entry(champ.id).or_default().push(format!(..)) | 4 | OK |  |
| 29 | DebugFrameData | 0xa0 | infos[champ.id] | w | context.debug(+0x3b) 가 true 일 때만. 게임 상태(TeamPlan·PlayerState 등)에는 쓰지 않는다 — team_plan 은 readonly 포인터 | 4 | OK | format!(<anon.307>, near_epic_ally, near_epic_enemy) 문자열 push |

**`consts` 상수 19건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev | folded_from |
|---|---|---|---|---|---|---|
| 0 | 6 | 17 | 태그 | TutorialType 태그 t 에 대해 (t-1) <u 6 ⟺ t ∈ {1..6} = First/TopSolo/Bottom/MidSolo/MidBottom/JungleOnly → 즉시 false. None(0)/Line(7)/Total(8) 만 통과 | 4 |  |
| 1 | -1 | 17 | 미상 | 위 범위검사의 `add nsw i8 %17, -1` — 1..=6 를 0..6 으로 접은 것 | 4 |  |
| 2 | 0 | 21 | 태그 | GameMode 태그 0 = Moba. 아니면 unwrap_failed 패닉(MobaMode 전제) | 4 |  |
| 3 | 15 | 25 | 계수 | 에픽 미생존 시 remain_spawn_tick >= tps*15 (15초 이상 남음) 이면 false | 4 |  |
| 4 | 3 | 36 | 임계 | live_ally_count < 3 이면 false (아군 생존 3명 미만) | 4 |  |
| 5 | 1 | 40 | 태그 | enemy_ix = 1 - team | 4 |  |
| 6 | 100 | 45 | 계수 | hp 백분율: hp*100 / stat_cached.hp (L45·L56·L62 세 필터에서 동일) | 4 |  |
| 7 | 49 | 45 | 임계 | hp% > 49 = HP 절반 이상인 챔프만 근접 카운트 대상(L45 아군·L56 비시야 적·L62 시야 적) | 4 |  |
| 8 | 4 | 47 | 태그 | JungleType 태그 4 = Morgard(에픽). camp_pos(map, Morgard, team==0) | 4 |  |
| 9 | 150000 | 52 | 계수 | 적 p 의 마지막 관측 좌표→에픽 캠프 거리에서 150000 을 saturating_sub — 캠프 반경 150000 안이면 d=0 | 4 |  |
| 10 | 22500000001 | 61 | 임계 | 150000^2 + 1 — 적 현재좌표↔캠프 제곱거리 < 이 값 (= 150000 이내) 이면 in_vision 후보 | 4 |  |
| 11 | -3 | 73 | 임계 | blackboard[team].<line>_minion_state.minion_count < -3 이면 그 라인이 '밀림'(line_pushed) | 4 |  |
| 12 | 2 | 75 | 임계 | 라인 미밀림 보너스: 어느 라인도 minion_count < -3 이 아니면 setup_score 에 +2, 아니면 +0 | 4 |  |
| 13 | 1 | 76 | 태그 | (near_epic_ally - near_epic_enemy) * 2 가 `shl i32 %728, 1` 로 접힘 | 4 | 2 |
| 14 | 0 | 79 | 태그 | judgement_penalty != 0 && setup_score < judgement_penalty 이면 false | 4 |  |
| 15 | 2 | 100 | 임계 | near_epic_ally < near_epic_enemy 일 때: out_vision(비시야 도달가능 적) > 2 여야 다음 검사 | 4 |  |
| 16 | 2 | 102 | 임계 | …그리고 live_ally_count(재계산) > 2 여야 true | 4 |  |
| 17 | 1 | 86 | 태그 | live_ally_count + 1 < live_enemy_count (아군이 2명 이상 적음) 이면 false | 4 |  |
| 18 | 1 | 94 | 태그 | near_epic_ally > near_epic_enemy + 1 (근접 아군이 2명 이상 많음) 이면 true | 4 |  |

**`knobs` 조정점 10건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 에픽 스폰 대기 허용 시간 | epic.rs:25 (m09.ll:57198 `mul i64 %54, 15`) | 15 | 에픽이 죽어 있을 때 남은 스폰 시간이 tps*15(15초) 이상이면 셋업 안 함. 올리면 더 일찍부터 에픽 셋업을 고려하고, 내리면 스폰 직전에만 고려 | 4 | 기존 |
| 1 | 아군 최소 생존 수 | epic.rs:36 (m09.ll:57292 `icmp samesign ult i64 %95, 3`) | 3 | 생존 아군 3명 미만이면 무조건 false. 내리면 소수 인원으로도 에픽 시도 | 4 | 기존 |
| 2 | 근접 카운트 HP 하한(%) | epic.rs:45·56·62 (m09.ll:57440 `icmp ugt i64 %146, 49` 외 동형 14곳) | 49 | HP 49% 초과 챔프만 근접 아군/적으로 센다. 올리면 만신창이 챔프를 무시해 셋업 판단이 보수적/공격적으로 바뀜(아군·적 양쪽에 동시 적용) | 4 | 기존 |
| 3 | 에픽 캠프 반경(비시야 적 도달 거리·시야 적 근접 거리) | epic.rs:52 (m09.ll:57836 `usub.sat(…,150000)`) · epic.rs:61 (m09.ll:58278 `22500000001`) | 150000 | 약 4.7셀(셀=32000). 올리면 더 먼 적까지 '캠프 근처'로 잡혀 near_epic_enemy 가 커져 셋업이 어려워짐 | 4 | 기존 |
| 4 | 라인 밀림 임계(minion_count) | epic.rs:73 (m09.ll:58906 `icmp slt i32 %719, -3`) | -3 | 어느 라인이든 blackboard[team] minion_count < -3 이면 '밀림' → +2 보너스 상실 + L86 조기 true 상실. 더 음수로 내리면 라인 압박을 덜 민감하게 봄 | 4 | 기존 |
| 5 | 라인 미밀림 보너스 | epic.rs:75 (m09.ll `%722 = phi i32 [2, %708], [0, %716]`) | 2 | setup_score 가산치. 올리면 라인 상태가 좋을 때 페널티 문턱을 넘기 쉬움 | 4 | 기존 |
| 6 | 근접 차이 가중치 | epic.rs:76 (m09.ll:58919 `shl i32 %728, 1`) | 2 | (near_epic_ally - near_epic_enemy)*2. 생존 차이(×1)보다 근접 차이를 2배로 본다 | 4 | 기존 |
| 7 | 아군 열세 허용폭 | epic.rs:86 (m09.ll:58991 `%764 = %95+1; ult %764, %123`) | 1 | live_ally+1 < live_enemy 면 false. 즉 1명 열세까지만 허용. 올리면 더 큰 열세에서도 시도 | 4 | 기존 |
| 8 | 비시야 도달가능 적 상한(열세 시) | epic.rs:100 (m09.ll:58938 `icmp ugt i64 %459, 2`) | 2 | 근접 아군<적일 때 out_vision > 2 && live_ally > 2 여야 true. 내리면 열세여도 true 가 잘 나옴 | 4 | 기존 |
| 9 | 근접 우위 요구폭(생존 열세 시) | epic.rs:94 (m09.ll:59010 `%775 = %726+1; ugt %724, %775`) | 1 | 생존 수가 밀릴 때 near_epic_ally 가 near_epic_enemy+1 보다 커야(2명 이상 우위) true | 4 | 기존 |

<details><summary>`callees` 피호출자 27건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | camp_pos | game_core::MapDef::camp_pos | pub | fn(&game_core::MapDef, game_core::JungleType, bool) -> (u64, u64) | game-core\src\simulation\map_def.rs:207 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | check_epic_setup | game_ai::plan_legacy::old::check_epic_setup | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> bool | game-ai\src\plan_legacy\old\epic.rs:16 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 2 | debug | game_core::AbstractBanpickRunner::debug | pub | fn(&Self/#0, &[usize]) | game-core\src\banpick\base_runner.rs:411 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 3 | debug | <game_core::FactoredBanpickAgent as game_core::AbstractBanpickRunner>::debug | pub | fn(&game_core::FactoredBanpickAgent, &[usize]) | game-core\src\banpick\sgd_v2.rs:5891 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 4 | debug | <game_core::LogisticSGDBanpickAgent as game_core::AbstractBanpickRunner>::debug | pub | fn(&game_core::LogisticSGDBanpickAgent, &[usize]) | game-core\src\banpick\sgd.rs:1172 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 5 | distance | game_core::utils::distance | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:12 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 6 | distance_sq | game_core::utils::distance_sq | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:6 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 7 | distance_sq | game_core::Entity::distance_sq | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2157 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 8 | get_game_mode | game_core::AbstractGame::get_game_mode | pub | fn(&Self/#0) -> game_core::GameMode | game-core\src\simulation.rs:78 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 9 | get_game_mode | <game_core::Game as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::Game) -> game_core::GameMode | game-core\src\simulation\game.rs:1707 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 10 | get_game_mode | <game_core::SingleLaneGame as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::SingleLaneGame) -> game_core::GameMode | game-core\src\simulation\game.rs:4248 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 11 | is_near_mid_line | game_core::is_near_mid_line | pub | fn(&game_core::GameContext, u64, u64) -> bool | game-core\src\simulation\map_regions.rs:127 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 12 | is_recent_visible | game_core::Blackboard::is_recent_visible | pub | fn(&game_core::Blackboard, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\game\blackboard.rs:346 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 13 | is_top_side | game_core::is_top_side | pub | fn(&game_core::GameContext, u64, u64) -> bool | game-core\src\simulation\map_regions.rs:21 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 14 | macro_judgement_penalty | game_ai::plan_legacy::team_plan::macro_judgement_penalty | pub | fn(usize, &game_core::PlayerState) -> i32 | game-ai\src\plan_legacy\team_plan\objective_helpers.rs:15 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 15 | position | game_core::PlayerAiContext::<'a, 'b, 'r>::position | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> game_core::Position | game-core\src\mod_ai.rs:587 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 16 | position | game_core::transfer::TeamRosterPlanningContext::position | pub | fn(&game_core::transfer::TeamRosterPlanningContext, game_core::Position) -> std::option::Option<&game_core::transfer::PositionPlanningContext> | game-core\src\transfer\roster_context.rs:234 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 17 | push | <game_core::Staff as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::staff | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\staff.rs:38 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 18 | push | <game_core::Athlete as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:434 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 19 | push | <game_core::Contract as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:2498 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 20 | team | game_view::ClientData::team | pub | fn(&game_view::ClientData, usize) -> std::option::Option<std::cell::Ref< game_core::Team>> | game-view\src\logic\client\data.rs:1651 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 21 | team | game_view::ClientDatabase::team | pub | fn(&game_view::ClientDatabase, usize) -> std::option::Option<&game_core::Team> | game-view\src\logic\client\data.rs:968 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 22 | team | game_core::PlayerAiContext::<'a, 'b, 'r>::team | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:583 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 23 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 24 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 25 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 26 | tutorial | game_core::Strategy::tutorial | pub | fn() -> game_core::Strategy | game-core\src\simulation\strategy.rs:590 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
</details>

⚠**미매칭 16개**: `drop  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `entry`, `enumerate`, `epic_giveup_tick`, `format_inner`, `grow_one`, `height`, `infos`, `insert_no_grow`, `minion_count`, `move_speed`, `next_respawn_tick`, `no_line_pushed`, `or_default`, `player_champion`, `rustc_entry`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 2곳** (m09.ll:14218, m09.ll:36095) · **형제 0개** 

**`open` 10건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | map_regions::is_top_side(context,x,y) 는 인라인돼 define 이 없다(map_regions.rs:22~24). 본문 관측: ry = setting.height - y; top = (ry < x). '어느 쪽이 top 인가'는 좌표계 해석이라 확정 안 함 — 판정에는 분기 방향(top 이면 is_near_mid_line 필요)만 썼다 | 4 |  |
| 1 | 미탐색 | is_near_mid_line 내부(_gcbc g09.ll:159265~159308)는 확인: max(ry,x) < 192001 → true / min(ry,x) < width(+0x12b8)-192000 → \|x-ry\| < 64000 / 그 외 true. 이 함수 명세에는 calls 로만 실었다 | 4 |  |
| 2 | 표기 불가 | line_pushed 의 소스 극성: dbg 는 DW_OP_not 1회로 line_pushed = !(전 라인 통과). 분기 의미(minion_count < -3 인 라인이 하나라도 있으면 +2 상실·L86 조기 true 상실)는 확정, 변수명이 any 인지 !all 인지는 표기 불가(외연 동일) | 4 |  |
| 3 | 미탐색 | judgement_ready 도 DW_OP_not 1회(= !(setup_score < penalty)). 분기: penalty != 0 && score < penalty → false 로 확정 | 4 |  |
| 4 | 미탐색 | macro_judgement_penalty 내부는 안 봄(m15.ll:51819~, i32 반환 range(-17179869,17179870)). 0 이면 페널티 검사 자체가 꺼진다는 것만 확인 | 4 |  |
| 5 | 미탐색 | Blackboard::is_recent_visible 내부는 안 봄. 인자 = (&blackboard[1-team], game data ptr, game vtable, player, entity). self 가 적팀 판이라는 것은 인덱스(1-team)로만 확정 | 4 |  |
| 6 | 미탐색 | MapDef::camp_pos(map, JungleType(i8), bool) 의 bool 인자에 team==0 을 넘긴다 — bool 의 의미(진영 미러링 여부로 추정)는 _gcbc g07.ll:152570 을 안 읽어 미확인 | 4 |  |
| 7 | 미탐색 | 디버그 문자열 포맷(anon.307)의 내용은 안 읽음. 인자 두 개가 near_epic_ally(%13)·near_epic_enemy(%12) 의 usize Display 인 것만 확인 | 4 |  |
| 8 | 미탐색 | tutorial JungleOnly(6) 의 빈 라인 목록 분기(%700)는 L17 이 태그 1..6 을 먼저 거르므로 사장(NA) 코드 — IR 에는 남아 있다 | 4 |  |
| 9 | 미탐색 | version(p1) 은 이 함수에서 분기를 만들지 않는다 — macro_judgement_penalty 로 전달만 | 4 |  |

<details><summary>`closed` 1건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

| # | 물음 | 닫은 근거 |
|---|---|---|
| 0 | L86 의 두 조건(live_ally+1 < live_enemy → false / no_line_pushed → true)이 소스에서 한 줄인지 두 줄인지 — 둘 다 !dbg L86 이라 구분 불가. 순서는 IR 로 확정(열세 검사가 먼저: 둘 다 참이면 false) | 본문에 해소 표기가 있다 |
</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

