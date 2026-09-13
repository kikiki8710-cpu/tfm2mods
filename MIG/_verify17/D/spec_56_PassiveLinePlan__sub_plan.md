---

### `56` PassiveLinePlan::sub_plan — 수동 라인전 서브플랜 선택 — Recall / LineSafe / LineWait / LineDefense{style,line,action_type} 중 하나를 낸다

| 항목 | 값 |
|---|---|
| id | `passive_line__sub_plan` |
| 심볼 | `_RNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old12passive_lineNtB2_15PassiveLinePlan8sub_plan` |
| 소스 | `game-ai\src\plan_legacy\old\passive_line.rs:848` |
| IR | `m04.ll` 24962~27342행 |
| 경로·가시성 | `game_ai::plan_legacy::old::PassiveLinePlan::sub_plan` · **pub** |
| 계층 | 레거시 플랜 |
| exe | `d2c5d0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r8` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_ai::plan_legacy::old::PassiveLinePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> game_ai::plan_legacy::sub_plan::SubPlan
```

<details><summary>인자 8개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | &mut SubPlan(72B) | 반환. 태그 +0x0(i64, 니치: 2=LineDefense 3=LineSafe 4=LineWait 5=Recall), 페이로드 +0x8~ | 4 |
| 1 | 1 | self | &PassiveLinePlan(280B) | readonly. in_recall(+0x110)·v46_flee(+0x112)·v46_flee_acute(+0x113)·v46_flee_cover(+0x115)·line(+0x116) | 4 |
| 2 | 2 | version | usize | 본문 분기 없음. can_near_enemies_range 에 전달만 | 4 |
| 3 | 3 | rnd | &mut StdRng(320B) | can_near_enemies_range 에 전달만 | 4 |
| 4 | 4 | player | &PlayerState(2528B) | info.team(+0x930)·info.position(+0x9c0)·is_recent_visible 인자 | 4 |
| 5 | 5 | data | &OperationData(24B) | cache·context·blackboard 전부 사용 | 4 |
| 6 | 6 | team_plan | &TeamPlan(1064B) | objective(+0x41f 태그, +0x420 라인) 읽기 + can_near_enemies_range 의 self | 4 |
| 7 | 7 | debug | &mut DebugFrameData(224B) | can_near_enemies_range 에 전달만 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn sub_plan(&self, version, rnd, player, data, team_plan, debug) -> SubPlan

[L849] if self.in_recall(+0x110) { [L850] return Recall }
[L851] if self.v46_flee(+0x112) && !self.v46_flee_cover(+0x115) {
[L852]   if self.v46_flee_acute(+0x113) { [L854] return LineWait{line: self.line} } else { [L858] return LineSafe{line: self.line} } }

[L863] gank_or_dive_here = team_plan.objective(+0x41f) ∈ {Gank(8), Dive(9)} && objective.line(+0x420) == self.line
[L864] line = self.line
[L867] if !gank_or_dive_here {
[L868]   if let Some(p) = self.check_bot_lane_2v1(player, data) { return p } }   // ── 인라인 (passive_line.rs:1018~1041)
       //  [L1019] if line != Bottom(2) → None
       //  [L1020] tick = game.tick(); if tutorial ∈{0,5,7,8} && !(tick < setting.epic_jungle.first_spawn_tick(+0x8a8).saturating_sub(tps*30)) → None   // 초반에만
       //  [L1021] if position(+0x9c0) <= 2 → None      (Bottom/Support 만)
       //  [L1023] partner_pos = position==3 ? 4 : 3
       //  [L1024] (lx,ly,rx,ry) = map.fountains[team](+0x6d70)
       //  [L1025] partner = cache.player_champion[team][partner_pos]
       //  [L1027] partner_absent = partner.is_none() || partner.is_in_return()(ty==13 && action_state==1) || (lx<=p.x<=rx && ly<=p.y<=ry)
       //          if !partner_absent → None
       //  [L1031] enemies_on_lane = enemy champs .filter(|c| is_near_line(context, c.x, c.y, Bottom) && bb[1-team].is_recent_visible(game, player, c)).count()
       //  [L1035] if enemies_on_lane < 2 → None
       //  [L1037] champ = cache.player_champion[team][position].unwrap(); [L1038] hp_ratio = champ.hp*100/champ.stat_cached.hp
       //  [L1039] wave_pushed = bb[team].bottom_minion_state.from_mid(+0x60) < 1000
       //  [L1041] Some(if hp_ratio > 50 || wave_pushed { LineSafe{line: Bottom} } else { Recall })

[L884] is_gank_target_line = objective == Gank(8) && objective.line == line
[L888] if is_gank_target_line {
[L889]   champ = player_champion[team][position].unwrap(); [L890] jungler = player_champion[team][Jungle(1)]
[L891]   jungler_ready = jungler.map_or(false, |j| {
[L892]     nearby  = distance_sq(j, champ) < 40000000001            // < 200000^2+1
[L893]     in_bush = map.bushes(+0x1c98)[min(j.y/32000,29)][min(j.x/32000,29)] != 0
[L894]     hidden  = !game.is_visible(1-team, j.id)                 // vtable 0xf8
[L895]     nearby && in_bush && hidden })
[L899]   line_style = position==Jungle(1) ? Defensive(1) : Aggressive(0)
[L906]   action_type = if jungler_ready { Normal(1) } else {
[L911]     from_mid = bb[team].<line>_minion_state.from_mid(+0x10)
[L912]     from_mid < 2001 ? Normal(1) : Pull(0) }
       } else {
[L899]   line_style = position==Jungle(1) ? Defensive(1) : Aggressive(0)
[L918]   champ = player_champion[team][position].unwrap()
[L919]   has_near_enemy_champion = enemy champs .any(|c| distance_sq(c, champ) < 22500000000 && bb[1-team].is_recent_visible(game, player, c))
[L922]   action_type = if has_near_enemy_champion { bb[team].<line>.minion_power(+0x18) < 0 ? Push(2) : Pull(0) } else { Push(2) }
       }

[L935] front_minion = bb[team].<line>_minion_state.front_minion (Option<usize>).and_then(|id| game.get_entity_by_id(id))   // vtable 0x1f0
[L938] if front_minion.is_none() → [L1013] return LineDefense{style: line_style, line, minion_action_type: action_type}
[L939] champ = player_champion[team][position].unwrap()
[L940] can_near_enemy = team_plan.can_near_enemies_range(version, rnd, player, data, fm.x, fm.y, 150000, debug).len()   // bumpalo Vec, +0x18
[L942] near_allies = ally champs .filter(|c| distance_sq(c, champ) < 22500000000 || distance_sq(c, fm) < 22500000000).count()   // 자기 자신 포함
[L945] nearest_tower = <line>_tower[team].or(<line>_tower2[team])
[L946]   .or(cache.twin_towers[team].iter().min_by_key(|t| distance_sq(t, LineType::get_start_position(&self.line, setting, team))))   // ★eager: min_by_key 는 항상 계산(aux m12.ll)
[L952]   .unwrap_or(cache.nexus[team].unwrap())
[L958] nexus = cache.nexus[team].unwrap()
[L959] if distance_sq(fm, nexus) < distance_sq(nearest_tower, nexus) → return LineDefense{..}     // 전방 미니언이 타워보다 안쪽
[L960] if !(near_allies < can_near_enemy && distance_sq(fm, nearest_tower) > 28899999999) → return LineDefense{..}
       // 이하: 열세 && 전방 미니언이 타워에서 170000 이상 떨어짐
[L962] minion_diff = bb[team].<line>.minion_count(+0x20)
[L968] is_object_far_line = match team_plan.objective {
[L969]   Some(Morgard(0)) => line == Bottom, [L971] Some(Serpen(1)) => line == Top,
[L972]   _ => if moba.map_or(false, |m| m.epic.live_list.len(+0x1a8) != 0) { [L973] line == Bottom }
[L974]        else { moba.map_or(false, |m| m.serpen.live_list.len(+0x1d8) != 0) && line == Top } }
[L980] giveup_object = match line {
[L981]   Top    => moba.and_then(|m| m.serpen.live_list[0] → get_entity_by_id).map_or(false, |s| s.is_visible_from(champ)),   // champ.team Neutral ⇒ true / visible_state[champ.team]==Visible
[L984]   Bottom => moba.and_then(|m| m.epic.live_list[0] → get_entity_by_id).map_or(false, |mg| mg.is_visible_from(champ)),
         Mid    => false }
[L990] if is_object_far_line {
[L993]   if can_near_enemy > 3 || giveup_object {
[L994]     if minion_diff > 2 { return Recall } else { [L997] return LineWait{line} } }
         return LineDefense{..}
       } else {
[L1003]  if let Tower(info) = nearest_tower.ty (+0x68 == 2) { [L1004] if info.nearest_enemy(+0x88).is_some() { return Recall } }
[L1013]  return LineDefense{style: line_style, line, minion_action_type: action_type}
       }
```

**`mem` 메모리 접근 52건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | PassiveLinePlan | 0x110 | in_recall | r | IR 272. true 면 즉시 Recall | 4 | OK |  |
| 1 | PassiveLinePlan | 0x112 | v46_flee | r | IR 274 | 4 | OK |  |
| 2 | PassiveLinePlan | 0x115 | v46_flee_cover | r | IR 277 | 4 | OK |  |
| 3 | PassiveLinePlan | 0x113 | v46_flee_acute | r | IR 275. flee 중 acute 면 LineWait, 아니면 LineSafe | 4 | OK |  |
| 4 | PassiveLinePlan | 0x116 | line (LineType) | r | IR 278. 0 Top/1 Mid/2 Bottom | 4 | OK |  |
| 5 | TeamPlan | 0x41f | objective (Option<MainObjective>) | r | IR 1055. (tag & ~1)==8 ⟺ Gank(8)\|Dive(9); ==8 Gank; 0 Morgard; 1 Serpen | 4 | OK |  |
| 6 | TeamPlan | 0x420 | objective.Gank.line / Dive.line | r | IR 1056 | 4 | OK |  |
| 7 | OperationData | 0x0 | cache | r | &AbstractGameWithCache. +0x0/+0x8 = &dyn AbstractGame | 4 | OK |  |
| 8 | OperationData | 0x8 | context | r | &GameContext | 4 | OK |  |
| 9 | OperationData | 0x10 | blackboard | r | &[Blackboard;2] | 4 | OK |  |
| 10 | GameContext | 0x38 | tutorial | r | IR 56. check_bot_lane_2v1 시간 게이트: ∈{0,5,7,8} 일 때만 first_spawn 기준 적용 | 4 | OK |  |
| 11 | GameContext | 0x8 | setting | r | &GameSetting | 4 | OK |  |
| 12 | GameContext | 0x20 | map | r | &MapDef | 4 | OK |  |
| 13 | GameSetting | 0x8a8 | epic_jungle.first_spawn_tick | r | IR 2216. 2v1 검사는 tick < first_spawn_tick - tps*30 인 초반에만 | 4 | OK |  |
| 14 | GameSetting | 0x12f8 | tick_per_second | r | IR 4856 | 4 | OK |  |
| 15 | MapDef | 0x6d70 | fountains[team] (lx,ly,rx,ry) | r | IR 28016+32*team. 파트너가 이 사각형 안이면 '부재' | 4 | OK |  |
| 16 | MapDef | 0x1c98 | bushes[yi][xi] ([30][30] usize) | r | IR 7320 + 240*yi + 8*xi. != 0 이면 부시 셀 | 4 | OK |  |
| 17 | dyn AbstractGame vtable | 0x28 | tick | r | divtable | 3 | 확인불가(vtable 슬롯(구조체 아님) — divtable.py 소관) |  |
| 18 | dyn AbstractGame vtable | 0x40 | get_game_mode | r | divtable → GameMode(태그 0=Moba, +0x8 &MobaMode) | 3 | 확인불가(vtable 슬롯(구조체 아님) — divtable.py 소관) |  |
| 19 | dyn AbstractGame vtable | 0xf8 | is_visible(team, entity_id) | r | divtable. 정글러가 적팀에 보이는가 | 3 | 확인불가(vtable 슬롯(구조체 아님) — divtable.py 소관) |  |
| 20 | dyn AbstractGame vtable | 0x1f0 | get_entity_by_id(id) -> Option<&Entity> | r | divtable. front_minion·serpen·morgard 조회 | 3 | 확인불가(vtable 슬롯(구조체 아님) — divtable.py 소관) |  |
| 21 | MobaMode | 0x1a8 | jungle_runner.epic.live_list.len | r | IR 424 | 4 | OK |  |
| 22 | MobaMode | 0x1a0 | jungle_runner.epic.live_list.ptr | r | IR 416. [0] = 모르가드 엔티티 id | 4 | OK |  |
| 23 | MobaMode | 0x1d8 | jungle_runner.serpen.live_list.len | r | IR 472 | 4 | OK |  |
| 24 | MobaMode | 0x1d0 | jungle_runner.serpen.live_list.ptr | r | IR 464. [0] = 세르펜 엔티티 id | 4 | OK |  |
| 25 | PlayerState | 0x930 | info.team | r | IR 2352 | 4 | OK |  |
| 26 | PlayerState | 0x9c0 | info.position@tag (i32) | r | IR 2496. 0 Top/1 Jungle/2 Mid/3 Bottom/4 Support | 4 | OK |  |
| 27 | AbstractGameWithCache | 0x1e0 | player_champion | r | IR 480 | 4 | OK |  |
| 28 | AbstractGameWithCache | 0x180 | top_tower[] | r | IR 384+32*line (top 0x180/mid 0x1a0/bottom 0x1c0) | 4 | OK |  |
| 29 | AbstractGameWithCache | 0x190 | top_tower2[] | r | IR 400+32*line | 4 | OK |  |
| 30 | AbstractGameWithCache | 0x130 | twin_towers[team] (bumpalo Vec: ptr +0, len +0x18) | r | IR 304+32*team | 4 | OK |  |
| 31 | AbstractGameWithCache | 0x170 | nexus[] | r | IR 368. None 이면 unwrap_failed 패닉 | 4 | OK |  |
| 32 | Blackboard | 0x0 | top_minion_state (BrainMinionParameter 40B) | r | Top +0x0 / Mid +0x28 / Bottom +0x50 | 4 | OK |  |
| 33 | BrainMinionParameter | 0x0 | front_minion (Option<usize>: tag +0, id +8) | r | None 이면 LineDefense 즉시 | 4 | OK |  |
| 34 | BrainMinionParameter | 0x10 | from_mid (i64) | r | L912 action_type 게이트(<2001) · L1039 wave_pushed(<1000, bottom) | 4 | OK |  |
| 35 | BrainMinionParameter | 0x18 | minion_power (i64) | r | L922 <0 이면 Push | 4 | OK |  |
| 36 | BrainMinionParameter | 0x20 | minion_count (i32) | r | L962 minion_diff. >2 면 Recall | 4 | OK |  |
| 37 | Entity | 0x660 | x | r | IR 1632 | 4 | OK |  |
| 38 | Entity | 0x668 | y | r | IR 1640 | 4 | OK |  |
| 39 | Entity | 0x5c0 | id | r | IR 1472. 정글러 id → is_visible | 4 | OK |  |
| 40 | Entity | 0x68 | ty@tag (EntityType) | r | IR 104. 13=Champion(is_in_return) · 2=Tower(L1003) | 4 | OK |  |
| 41 | Entity | 0x70 | ty.Champion.action_state@tag | r | IR 112. ==1 이면 is_in_return (entity.rs:1648~1649 인라인) | 4 | OK |  |
| 42 | Entity | 0x88 | ty.Tower.info.nearest_enemy@tag | r | IR 136. != 0(Some) 이면 타워가 적을 물고 있음 → Recall | 4 | OK |  |
| 43 | Entity | 0x0 | team@tag | r | champ. 1=Neutral 이면 is_visible_from 무조건 true | 4 | OK |  |
| 44 | Entity | 0x8 | team@Player.0 | r | → 오브젝트 visible_state[team] | 4 | OK |  |
| 45 | Entity | 0x38 | visible_state | r | IR 56+24*team. 0=Visible | 4 | OK |  |
| 46 | Entity | 0x628 | stat_cached.hp | r | IR 1576. 0 이면 div_by_zero 패닉 | 4 | OK |  |
| 47 | Entity | 0x670 | hp | r | IR 1648 | 4 | OK |  |
| 48 | SubPlan(sret) | 0x0 | tag | w | 게임 상태에는 쓰지 않는다. self 는 readonly | 4 | OK | 5 Recall / 3 LineSafe / 4 LineWait / 2 LineDefense |
| 49 | SubPlan(sret) | 0x8 | LineSafe.line / LineWait.line / LineDefense.style | w | L850~858·868·997·1013 | 4 | OK | line(self.line 또는 Bottom) / line_style |
| 50 | SubPlan(sret) | 0x9 | LineDefense.line | w | L1013 | 4 | OK | line |
| 51 | SubPlan(sret) | 0xa | LineDefense.minion_action_type | w | L1013 | 4 | OK | action_type (0 Pull/1 Normal/2 Push) |

**`consts` 상수 29건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 5 | 850 | 태그 | SubPlan 태그 5 = Recall (L850·L1004·L994 세 곳) | 4 |
| 1 | 3 | 858 | 태그 | SubPlan 태그 3 = LineSafe | 4 |
| 2 | 4 | 854 | 태그 | SubPlan 태그 4 = LineWait | 4 |
| 3 | 2 | 1013 | 태그 | SubPlan 태그 2 = LineDefense | 4 |
| 4 | -2 | 863 | 계수 | objective 태그 & ~1 == 8 ⟺ Gank(8) 또는 Dive(9) | 4 |
| 5 | 8 | 863 | 태그 | MainObjective 태그 8 = Gank (L884 는 정확히 8 만) | 4 |
| 6 | 2 | 1019 | 임계 | LineType 2 = Bottom — check_bot_lane_2v1 은 self.line==Bottom 에서만 | 4 |
| 7 | 30 | 1020 | 계수 | tick < first_spawn_tick.saturating_sub(tps*30) 일 때만 2v1 검사 (첫 에픽 스폰 30초 전까지) | 4 |
| 8 | 2 | 1021 | 임계 | position > 2 (Bottom(3)/Support(4)) 만 2v1 검사 | 4 |
| 9 | 3 | 1023 | 임계 | partner_pos = position==Bottom(3) ? Support(4) : Bottom(3) | 4 |
| 10 | 13 | 1027 | 태그 | EntityType 13 = Champion — is_in_return 의 전제 | 4 |
| 11 | 1 | 1027 | 태그 | Champion.action_state 태그 1 = 귀환 중(is_in_return) | 4 |
| 12 | 2 | 1035 | 임계 | enemies_on_lane(바텀 라인 근처·최근 가시 적) < 2 이면 2v1 아님 | 4 |
| 13 | 100 | 1038 | 계수 | hp_ratio = hp*100/max_hp | 4 |
| 14 | 1000 | 1039 | 임계 | wave_pushed = bb[team].bottom_minion_state.from_mid < 1000 (signed) | 4 |
| 15 | 50 | 1041 | 임계 | hp_ratio > 50 \|\| wave_pushed → LineSafe{Bottom}, 아니면 Recall | 4 |
| 16 | 40000000001 | 892 | 임계 | 200000^2+1 — 정글러↔내 챔프 제곱거리 < 이 값이면 nearby | 4 |
| 17 | 29 | 893 | 인덱스 | 부시 격자 인덱스 clamp 상한(30x30). min(y/32000, 29), min(x/32000, 29) | 4 |
| 18 | 32000 | 893 | 계수 | 셀 크기 — 정글러 좌표 → 부시 격자 | 4 |
| 19 | 1 | 899 | 태그 | line_style = (position == Jungle(1)) ? Defensive(1) : Aggressive(0) | 4 |
| 20 | 2001 | 912 | 임계 | 갱크 대상 라인이고 정글러가 숨어 있지 않을 때: from_mid < 2001 이면 Normal(1), 아니면 Pull(0) | 4 |
| 21 | 22500000000 | 919 | 임계 | 150000^2 — 적 챔프↔내 챔프 제곱거리 < 이 값 && 최근 가시 → has_near_enemy_champion (L942 near_allies 도 같은 임계) | 4 |
| 22 | 0 | 922 | 임계 | has_near_enemy 일 때 bb[team].<line>.minion_power < 0 이면 Push(2) 아니면 Pull(0); 적 없으면 Push(2) | 4 |
| 23 | 150000 | 940 | 미상 | can_near_enemies_range(.., front_minion.x, front_minion.y, 150000, ..) — 전방 미니언 150000 안에 올 수 있는 적 목록 | 4 |
| 24 | 28899999999 | 960 | 임계 | 170000^2 - 1 — dist²(front_minion, nearest_tower) > 이 값 (≥170000) 이고 near_allies < can_near_enemy 면 위험 분기로 | 4 |
| 25 | 3 | 993 | 임계 | can_near_enemy(도달 가능 적 수) > 3 이면 오브젝트 가시 여부와 무관하게 후퇴 판단 | 4 |
| 26 | 2 | 994 | 임계 | minion_diff(minion_count) > 2 이면 Recall, 아니면 LineWait{line} | 4 |
| 27 | 2 | 1003 | 임계 | EntityType 2 = Tower — nearest_tower 가 타워이고 nearest_enemy 가 Some 이면 Recall | 4 |
| 28 | 0 | 969 | 임계 | 오브젝트 visible_state[champ.team].tag == 0(Visible) = giveup_object(오브젝트가 보이면 포기) | 4 |

**`knobs` 조정점 12건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 바텀 2v1 검사 시간창 | passive_line.rs:1020 (m04.ll:25121 `mul i64 %68, 30`) | 30 | 첫 에픽 스폰 30초 전까지만 2v1(파트너 부재·적 2명) 검사. 올리면 더 이른 시점까지만 | 4 | 기존 |
| 1 | 2v1 판정 적 인원 | passive_line.rs:1035 (m04.ll:25497 `icmp samesign ult i64 %201, 2`) | 2 | 바텀 근처 최근가시 적이 2명 이상이어야 2v1. 내리면 1명만 있어도 안전/귀환 판단 | 4 | 기존 |
| 2 | 2v1 시 HP 문턱(%) | passive_line.rs:1041 (m04.ll:25542 `icmp ugt i64 %223, 50`) | 50 | HP > 50% 이거나 웨이브 밀렸으면 LineSafe, 아니면 Recall. 올리면 더 자주 귀환 | 4 | 기존 |
| 3 | 2v1 시 웨이브 밀림 임계 | passive_line.rs:1039 (m04.ll:25540 `icmp slt i64 %226, 1000`) | 1000 | bottom from_mid < 1000 이면 wave_pushed 로 보고 LineSafe 선택 | 4 | 기존 |
| 4 | 정글러 근접 거리 | passive_line.rs:892 (m04.ll:25634 `40000000001`) | 40000000001 | 200000(≈6.25셀) 안의 숨은 정글러만 '준비됨'. 올리면 먼 정글러도 갱 준비로 봐 Normal 행동 | 4 | 기존 |
| 5 | 갱 대기 웨이브 위치 임계 | passive_line.rs:912 (m04.ll:26148 `icmp slt i64 %488, 2001`) | 2001 | 정글러 미준비 시 from_mid <= 2000 이면 Normal, 넘으면 Pull(웨이브 당김). 올리면 Pull 이 줄어듦 | 4 | 기존 |
| 6 | 근접 적/아군 판정 반경 | passive_line.rs:919·942 (m04.ll:25828 `22500000000`) | 22500000000 | 150000(≈4.7셀). 근접 적 존재·근접 아군 수 계산 반경 | 4 | 기존 |
| 7 | 전방 미니언 위협 탐색 반경 | passive_line.rs:940 (m04.ll:26253 can_near_enemies_range 인자 150000) | 150000 | 전방 미니언 150000 안에 도달 가능한 적 수 = can_near_enemy. 올리면 위협을 더 넓게 세어 열세 판정이 잦아짐 | 4 | 기존 |
| 8 | 타워 이탈 거리 | passive_line.rs:960 (m04.ll:27024 `28899999999`) | 28899999999 | 전방 미니언이 가장 가까운 타워에서 170000 이상 떨어져야 후퇴 계열(LineWait/Recall) 검토. 내리면 타워 가까이서도 후퇴 검토 | 4 | 기존 |
| 9 | 후퇴 강제 적 인원 | passive_line.rs:993 (m04.ll:27310 `icmp ugt i64 %535, 3`) | 3 | 도달 가능 적 > 3 이면 오브젝트 가시 여부 무관하게 LineWait/Recall. 내리면 더 쉽게 후퇴 | 4 | 기존 |
| 10 | Recall vs LineWait 미니언 차 | passive_line.rs:994 (m04.ll:27315 `icmp sgt i32 %868, 2`) | 2 | minion_count > 2(아군 웨이브 우세) 면 Recall, 아니면 LineWait. 올리면 Recall 대신 LineWait 이 늘어남 | 4 | 기존 |
| 11 | minion_power 부호로 Push/Pull | passive_line.rs:922 (m04.ll:26118 `icmp slt i64 %473, 0`) | 0 | 근접 적이 있을 때 minion_power < 0 이면 Push(2) 아니면 Pull(0). 극성이 직관과 반대일 수 있음(unknown 참조) | 4 | 기존 |

<details><summary>`callees` 피호출자 28건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | can_near_enemies_range | game_ai::plan_legacy::team_plan::TeamPlan::can_near_enemies_range | pub | fn(&game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, u64, u64, u64, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< &game_core::Entity> | game-ai\src\plan_legacy\team_plan.rs:483 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | check_bot_lane_2v1 | game_ai::plan_legacy::old::SinglePlanLine::check_bot_lane_2v1 | in:game_ai::plan_legacy::old::single_line | fn(&game_ai::plan_legacy::old::SinglePlanLine, &game_core::PlayerState, &game_core::OperationData) -> std::option::Option<game_ai::plan_legacy::sub_plan::SubPlan> | game-ai\src\plan_legacy\old\single_line.rs:231 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 2 | check_bot_lane_2v1 | game_ai::plan_legacy::old::PassiveLinePlan::check_bot_lane_2v1 | in:game_ai::plan_legacy::old::passive_line | fn(&game_ai::plan_legacy::old::PassiveLinePlan, &game_core::PlayerState, &game_core::OperationData) -> std::option::Option<game_ai::plan_legacy::sub_plan::SubPlan> | game-ai\src\plan_legacy\old\passive_line.rs:1018 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 3 | distance_sq | game_core::utils::distance_sq | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:6 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 4 | distance_sq | game_core::Entity::distance_sq | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2157 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 5 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 6 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 7 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 8 | get_start_position | game_core::LineType::get_start_position | pub | fn(&game_core::LineType, &game_core::GameSetting, usize) -> (u64, u64) | game-core\src\simulation\state\player.rs:1013 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 9 | in_recall | game_core::Blackboard::in_recall | pub | fn(&game_core::Blackboard, usize) -> bool | game-core\src\simulation\game\blackboard.rs:175 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 10 | is_in_return | game_core::Entity::is_in_return | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1647 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 11 | is_near_line | game_core::is_near_line | pub | fn(&game_core::GameContext, u64, u64, game_core::LineType) -> bool | game-core\src\simulation\map_regions.rs:139 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 12 | is_recent_visible | game_core::Blackboard::is_recent_visible | pub | fn(&game_core::Blackboard, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\game\blackboard.rs:346 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 13 | is_visible | game_core::World::is_visible | pub | fn(&game_core::World, usize, usize) -> bool | game-core\src\simulation\game\data.rs:384 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 14 | is_visible | game_core::VisibleState::is_visible | pub | fn(&game_core::VisibleState) -> bool | game-core\src\simulation\game\data.rs:121 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 15 | is_visible | game_core::AbstractGame::is_visible | pub | fn(&Self/#0, usize, usize) -> bool | game-core\src\simulation.rs:125 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 16 | is_visible_from | game_core::Entity::is_visible_from | pub | fn(&game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1481 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 17 | line | game_ai::plan_legacy::types::BigPlan::debug_label::line | in:game_ai::plan_legacy::types | fn(game_core::LineType) -> &str | game-ai\src\plan_legacy\types.rs:83 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 18 | line | game_core::TowerType::line | pub | fn(&game_core::TowerType) -> std::option::Option<game_core::LineType> | game-core\src\simulation\entity\tower.rs:90 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 19 | nearest_enemy | game_core::Entity::nearest_enemy | pub | fn(&game_core::Entity) -> std::option::Option<usize> | game-core\src\simulation\entity.rs:1819 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 20 | position | game_core::PlayerAiContext::<'a, 'b, 'r>::position | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> game_core::Position | game-core\src\mod_ai.rs:587 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 21 | position | game_core::transfer::TeamRosterPlanningContext::position | pub | fn(&game_core::transfer::TeamRosterPlanningContext, game_core::Position) -> std::option::Option<&game_core::transfer::PositionPlanningContext> | game-core\src\transfer\roster_context.rs:234 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 22 | sub_plan | game_ai::plan_legacy::types::BigPlan::sub_plan | pub | fn(&mut game_ai::plan_legacy::types::BigPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PositioningScoreData, &mut game_core::DebugFrameData) -> game_ai::plan_legacy::sub_plan::SubPlan | game-ai\src\plan_legacy\types.rs:230 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 16개 중 상위 3개 |
| 23 | sub_plan | game_ai::plan_legacy::old::BattlePlan::sub_plan | pub | fn(&game_ai::plan_legacy::old::BattlePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> game_ai::plan_legacy::sub_plan::SubPlan | game-ai\src\plan_legacy\old\battle.rs:2043 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 16개 중 상위 3개 |
| 24 | sub_plan | game_ai::plan_legacy::old::SinglePlanLine::sub_plan | pub | fn(&game_ai::plan_legacy::old::SinglePlanLine, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> game_ai::plan_legacy::sub_plan::SubPlan | game-ai\src\plan_legacy\old\single_line.rs:84 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 16개 중 상위 3개 |
| 25 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 26 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 27 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
</details>

⚠**미매칭 12개**: `bushes`, `drop  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `first_spawn_tick`, `from_mid`, `front_minion`, `map_or`, `minion_count`, `minion_power`, `objective`, `v46_flee`, `v46_flee_acute`, `v46_flee_cover`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m02.ll:8298) · **형제 18개** (PassiveLinePlan)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::plan_legacy::old::PassiveLinePlan as std::clone::Clone>::clone | pub | game-ai\src\plan_legacy\old\passive_line.rs:176 | True | fn(&game_ai::plan_legacy::old::PassiveLinePlan) -> game_ai::plan_legacy::old::PassiveLinePlan |
| 1 | <game_ai::plan_legacy::old::PassiveLinePlan as std::fmt::Debug>::fmt | pub | game-ai\src\plan_legacy\old\passive_line.rs:176 | True | fn(&game_ai::plan_legacy::old::PassiveLinePlan, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 2 | game_ai::plan_legacy::old::PassiveLinePlan::new | pub | game-ai\src\plan_legacy\old\passive_line.rs:197 | True | fn(game_core::LineType) -> game_ai::plan_legacy::old::PassiveLinePlan |
| 3 | game_ai::plan_legacy::old::PassiveLinePlan::v46_fleeing | pub | game-ai\src\plan_legacy\old\passive_line.rs:207 | True | fn(&game_ai::plan_legacy::old::PassiveLinePlan) -> bool |
| 4 | game_ai::plan_legacy::old::PassiveLinePlan::goal | pub | game-ai\src\plan_legacy\old\passive_line.rs:211 | True | fn(&game_ai::plan_legacy::old::PassiveLinePlan) -> game_core::BigGoal |
| 5 | game_ai::plan_legacy::old::PassiveLinePlan::update | pub | game-ai\src\plan_legacy\old\passive_line.rs:219 | False | fn(&mut game_ai::plan_legacy::old::PassiveLinePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan, &game_core::PositioningScoreData, &mut game_core::DebugFrameData) |
| 6 | game_ai::plan_legacy::old::PassiveLinePlan::v46_carry_over | pub | game-ai\src\plan_legacy\old\passive_line.rs:307 | False | fn(&mut game_ai::plan_legacy::old::PassiveLinePlan, &mut game_ai::plan_legacy::old::PassiveLinePlan) |
| 7 | game_ai::plan_legacy::old::PassiveLinePlan::v46_flee_end | in:game_ai::plan_legacy::old::passive_line | game-ai\src\plan_legacy\old\passive_line.rs:322 | False | fn(&mut game_ai::plan_legacy::old::PassiveLinePlan) |
| 8 | game_ai::plan_legacy::old::PassiveLinePlan::update_v46_flee | in:game_ai::plan_legacy::old::passive_line | game-ai\src\plan_legacy\old\passive_line.rs:343 | False | fn(&mut game_ai::plan_legacy::old::PassiveLinePlan, usize, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 9 | game_ai::plan_legacy::old::PassiveLinePlan::v46_clear | in:game_ai::plan_legacy::old::passive_line | game-ai\src\plan_legacy\old\passive_line.rs:476 | False | fn(&mut game_ai::plan_legacy::old::PassiveLinePlan, bool) |
| 10 | game_ai::plan_legacy::old::PassiveLinePlan::update_v46_lane_recall | in:game_ai::plan_legacy::old::passive_line | game-ai\src\plan_legacy\old\passive_line.rs:488 | False | fn(&mut game_ai::plan_legacy::old::PassiveLinePlan, usize, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 11 | game_ai::plan_legacy::old::PassiveLinePlan::v46_stage1 | in:game_ai::plan_legacy::old::passive_line | game-ai\src\plan_legacy\old\passive_line.rs:663 | False | fn(&game_ai::plan_legacy::old::PassiveLinePlan, usize, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, &game_core::Entity, &game_core::Entity, usize, bool, std::option::Option<&[usize]>, &bumpalo::collections::vec::Vec< &game_core::Entity>) -> bumpalo::collections::vec::Vec< (usize, usize, usize)> |
| 12 | game_ai::plan_legacy::old::PassiveLinePlan::v46_stage2 | in:game_ai::plan_legacy::old::passive_line | game-ai\src\plan_legacy\old\passive_line.rs:783 | False | fn(&game_ai::plan_legacy::old::PassiveLinePlan, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, &game_core::Entity, &bumpalo::collections::vec::Vec< (usize, usize, usize)>) -> bool |
| 13 | game_ai::plan_legacy::old::PassiveLinePlan::sub_plan | pub | game-ai\src\plan_legacy\old\passive_line.rs:848 | False | fn(&game_ai::plan_legacy::old::PassiveLinePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> game_ai::plan_legacy::sub_plan::SubPlan |
| 14 | game_ai::plan_legacy::old::PassiveLinePlan::check_bot_lane_2v1 | in:game_ai::plan_legacy::old::passive_line | game-ai\src\plan_legacy\old\passive_line.rs:1018 | False | fn(&game_ai::plan_legacy::old::PassiveLinePlan, &game_core::PlayerState, &game_core::OperationData) -> std::option::Option<game_ai::plan_legacy::sub_plan::SubPlan> |
| 15 | game_ai::plan_legacy::old::PassiveLinePlan::has_lead | pub | game-ai\src\plan_legacy\old\passive_line.rs:1048 | False | fn(&game_ai::plan_legacy::old::PassiveLinePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> bool |
| 16 | game_ai::plan_legacy::old::PassiveLinePlan::check_recall | in:game_ai::plan_legacy::old::passive_line | game-ai\src\plan_legacy\old\passive_line.rs:1068 | False | fn(&game_ai::plan_legacy::old::PassiveLinePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> bool |
| 17 | game_ai::plan_legacy::old::PassiveLinePlan::next_plan | pub | game-ai\src\plan_legacy\old\passive_line.rs:1451 | True | fn(&mut game_ai::plan_legacy::old::PassiveLinePlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::GoalData, &game_core::PositioningScoreData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData) -> std::option::Option<game_ai::plan_legacy::types::BigPlan> |

**`open` 12건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 표기 불가 | check_bot_lane_2v1 의 partner 판정: IR 분기로 확정한 것은 '파트너 None / 귀환중 / 분수(fountains[team] 사각형) 안 → 계속, 살아서 분수 밖 → None'. dbg 의 partner_absent 는 %207(y>ry) 조각만 가리켜 소스 변수의 정확한 식은 표기 불가 | 4 |  |
| 1 | 미탐색 | L922 action_type: has_near_enemy 이고 minion_power < 0 이면 Push(2), 아니면 Pull(0); 적 없으면 Push(2) — 분기 방향으로 확정. minion_power 의 부호 의미(양수=아군 우세?)는 미확인이라 knob 극성 설명은 보류 | 4 |  |
| 2 | 미탐색 | L899 line_style = zext(position==1): LineStyle 태그 1=Defensive 로 매핑(tcxdict). 정글러(position 1)가 이 함수를 타는 경우가 실제 있는지는 미확인 | 3 |  |
| 3 | 미탐색 | TeamPlan::can_near_enemies_range(m09.ll:25960) 내부는 안 봄. 반환 32B(bumpalo Vec<&Entity>) 의 +0x18 을 len 으로 읽는다 — bumpalo Vec 레이아웃(ptr +0 · len +0x18) 은 twin_towers(+0x130 ptr/+0x148 len) 와 동형이라 추정 | 4 |  |
| 4 | 표기 불가 | nearest_tower 결정 순서: tower1.or(tower2).or(twin_min).unwrap_or(nexus) 로 읽었다. IR 은 twin min_by_key 를 optb 검사 전에 무조건 계산(eager) — `.or(..)` 인지 `.or_else(..)` 인지는 표기 불가이나 결과는 동일 | 4 |  |
| 5 | 미탐색 | min_by_key 의 fold 조각(aux m12.ll:27539~27691)은 각 트윈타워의 LineType::get_start_position 까지 제곱거리만 계산 — 새 상수 없음 | 4 |  |
| 6 | 미탐색 | L1003 nearest_tower 가 Tower 가 아닐 때(nexus 로 대체된 경우 ty==3 Nexus): Recall 검사 없이 LineDefense — IR 확정 | 4 |  |
| 7 | 미탐색 | giveup_object 의 is_visible_from 인라인: champ.team Neutral(태그1) 이면 true. 실제 챔프가 Neutral 인 경우는 없을 것으로 추정(사장 분기) | 5 |  |
| 8 | 미탐색 | L1020 tutorial 게이트: 태그 1,2,3,4,6(First/TopSolo/Bottom/MidSolo/JungleOnly)은 시간 게이트 없이 2v1 검사로 진입 — switch default 경로로 확정 | 4 |  |
| 9 | 미탐색 | map_regions::is_near_line(context, x, y, LineType) · LineType::get_start_position(&line, setting, team) 내부는 안 봄 | 4 |  |
| 10 | 미탐색 | Recall 반환 시 +0x8 에 i8 2 가 저장되는 곳(L868)이 있으나 RecallSubPlan 은 0B 라 의미 없는 공유 store(LLVM 병합) | 4 |  |
| 11 | 미탐색 | version(p2)·rnd(p3)·debug(p7) 은 본문 분기에 안 쓰이고 can_near_enemies_range 로 전달만 | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

