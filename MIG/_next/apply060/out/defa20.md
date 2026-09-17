# defa20→fd7b40 is_end

## logic_060
```
// hunt_and_poke.rs:163 EpicHuntAndPokePlan::is_end(self, version, _rnd, player, data, team_plan, _debug) -> bool
// self / _rnd / _debug 는 본문에서 한 번도 읽지 않는다. version 은 v24_release_to_passive 에 전달만.
// ★0.6.0 TeamPlan: v4_serpen 니치 +0x3e4(2=None) · v5_morgard 니치 +0x404(2=None) · v5_morgard.phase +0x400 · 레거시 objective +0xcd5 · phase +0xcd6 (구 +0x41f/+0x420)

let team = player.info.team; // PlayerState+0xa00
let game = data.cache.game; // &dyn AbstractGame (data ptr + vtable)

// ── L164 : 팀 주목표가 더 이상 에픽이 아니면 즉시 종료 ─────────────────
// 소스: if !team_plan.take_active(JungleType::Morgard) { return true }
// ★0.6.0 IR 실체(이중모드):
if team_plan.v4_serpen.is_some() /*+0x3e4 != 2*/ { return true }                         // ★0.6.0 세르펜 씬이 살아 있으면 에픽 플랜 종료
if team_plan.v5_morgard.is_none() /*+0x404 == 2*/ && team_plan.objective(+0xcd5) != 0 /*Morgard*/ { return true }   // ★0.6.0 모가드 씬 없고 레거시 목표도 Morgard 아님 → 종료
// ★0.6.0 삭제: 구 `if (u8)TeamPlan[0x41f] != 0 { return true }`

// ── L169 : 에픽 캠프 좌표 ──────────────────────────────────────────
let (cx, cy) = data.context.map.camp_pos(JungleType::Morgard, team == 0); // (u64,u64) · 1416a7af0

// ── L172 : setup 단계인가 ─────────────────────────────────────────
// 소스: let setup_like = team_plan.take_setup_like(JungleType::Morgard)
// ★0.6.0: (여기 도달 시 +0x3e4==2 는 확정) phase = v5_morgard.is_some() ? v5_morgard.phase(+0x400) : legacy.phase(+0xcd6)
let setup_like = (if team_plan.v5_morgard.is_some() /*+0x404 != 2*/ { team_plan.+0x400 } else { team_plan.+0xcd6 }) == ObjectPhase::Setup /*1*/;
// ★0.6.0 삭제: 구 `((u8)TeamPlan[0x41f] == 0) && ((u8)TeamPlan[0x420] == 1)`

if setup_like { // L173
 // L174 — 오브젝티브 규율 계층이 '수동으로 풀어라'라고 하면 종료
 if TeamPlan::v24_objective_setup_should_release_to_passive(team_plan, version, player, data, JungleType::Morgard) {   // 콜리 v3: f0e390(구 dd7250)
 return true;
 }

 // L179 — 우리 팀 시야에 에픽 캠프 셀이 보이는가
 if game.is_visible_cell(team, cx / 32000, cy / 32000) { // vtable +0x100

 // L180~181 — 적팀 챔피언 5칸 중 '최근 목격된' 것만 남기고, 캠프에서 가장 가까운 하나
 let nearest = data.cache.player_champion[1 - team] // AGWC+0x1e0, [5]칸
 .iter()
 .filter_map(|c| *c) // iter_champions: None 칸 스킵
 .filter(|e| data.blackboard[1 - team].is_recent_visible(game, player, e))   // 콜리 v3: 17d3bc0(bb[1-team] stride 0x5c8, game, vt, player, e) — 5칸 언롤 인라인
 .min_by_key(|e| distance_sq(e.x, e.y, cx, cy));   // 콜리 v3: fold dbb2a0(구 deeda0 인라인 대응)
 // 술어 = Blackboard::is_recent_visible.

 match nearest {
 None => return true, // L183 — ★구조적 도달 불가(죽은 경로 · 판정반전 R2 · 0.6.0 도 같은 구조: v24 false ⇒ v23!=0 ⇒ 필터 원소 존재)
 Some(e) => { // L184
 let d2 = distance_sq(e.x, e.y, cx, cy); // abs_diff 제곱합 (utils.rs:6)
 if d2 > 22500000000 /* 150000^2 · 0x53d1ac100 */ { return true } // 적이 캠프에서 멀다 → 종료
 }
 }
 }
 // is_visible_cell == false 이거나 적이 충분히 가까우면 아래로 흐른다
}
// setup_like == false 도 아래로 흐른다

// ── L193 : 에픽 몬스터가 지금 살아 있나 ────────────────────────────
let moba = game.get_game_mode().as_moba().unwrap(); // vtable +0x40, GameMode 태그 0 = Moba
 // 태그 != 0 이면 Option::unwrap 패닉
let epic = moba.jungle_runner.epic.live_list.get(0) // MobaMode+0x1a0 ptr / +0x1a8 len (len==0 → None)
 .and_then(|id| game.get_entity_by_id(*id)); // vtable +0x1f0
if epic.is_some() { return false } // 살아 있으면 계속 헌트

// ── L194 : 죽어 있으면, 리젠까지 얼마나 남았나 ─────────────────────
let moba = game.get_game_mode().as_moba().unwrap(); // ★IR 상 get_game_mode 를 실제로 두 번 호출한다
return moba.jungle_runner.epic.next_respawn_tick // MobaMode+0x1b0
 .saturating_sub(game.tick()) // vtable +0x28
 > 15 * data.context.setting.tick_per_second; // GameSetting+0x12f8
// → 리젠까지 15초 초과로 남았으면 종료, 15초 이내면 계속 대기(포킹 유지)

// 요약 — true(종료) 가 되는 경로:
// (a1) ★0.6.0 v4_serpen 씬 활성(+0x3e4≠2) [L164]
// (a2) ★0.6.0 v5_morgard 씬 없음(+0x404==2) && 레거시 objective ≠ Morgard(+0xcd5≠0) [L164]
// (b) setup 단계 + 규율계층이 passive 로 풀라고 함 [L174]
// (c) setup + 캠프 셀 시야 O + 보이는 적 챔프 0명 [L183] ★도달 불가
// (d) setup + 캠프 셀 시야 O + 최근접 적이 150000 밖 [L184]
// (e) 에픽 죽어 있고 리젠까지 15초 초과 남음 [L194]
// false(계속): (f) 에픽이 살아 있음 (g) 에픽 죽었지만 리젠 15초 이내
```

## changes
- L164: `tp.+0x41f != 0 → true` → `tp.+0x3e4 != 2 → true` + `tp.+0x404 == 2 && tp.+0xcd5 != 0 → true`.
- L172: `setup_like = +0x41f==0 && +0x420==1` → `(+0x404 != 2 ? +0x400 : +0xcd6) == 1`.
- 나머지(release_to_passive · is_visible_cell · 최근접 적 150000² · 에픽 live/리스폰 15s) 동치.
- 오프셋/콜리: PS +0x930→+0xa00 · bb stride 0x5c8 · v24_release dd7250→f0e390 · is_recent_visible 17d3bc0 · min fold dbb2a0 · camp_pos 1416a7af0 · MobaMode +0x1a0/+0x1a8/+0x1b0 · vt 0x100/0x40/0x1f0/0x28.
- ★배치 B 「+0x3e4=Epic 모드」 추정은 폐기 — w7 전수표(v4_serpen/v5_morgard 니치)와 배치 E §9 가 정본(+0x3e4 = Serpen 씬 · +0x404 = Morgard 씬).

## verified
- Ghidra 0.6.0 fd7b40 디컴 전문: `+0x3e4 != 2 → true` · `+0x404 == 2 && +0xcd5 != 0 → true` · 1416a7af0 · 재검사 `+0x3e4 != 2 → 꼬리`(도달 불가) · `+0x404==2 ? (+0xcd5!=0 → 꼬리 · +0xcd6) : +0x400` == 1 · f0e390(tp, version, player, data, 4) · vt 0x100(team, cx/32000, cy/32000) · 5칸 17d3bc0 언롤 → dbb2a0 fold · 0x53d1ac100 초과 → true · vt 0x40 태그 0 · +0x1a8/+0x1a0 → vt 0x1f0 · +0x1b0 sat_sub tick · tps*15 <.
- 미확인: f0e390 내부 · dbb2a0 fold 동률 규칙(구와 동일 추정).

## confidence
A — 변경 2건 디컴 확인 · 본문 동치 확인.