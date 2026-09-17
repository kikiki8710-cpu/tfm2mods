# defcd0→fd7f70 EpicHuntAndPokePlan::sub_plan

## logic_060
```
// hunt_and_poke.rs:30~96 (+ check_recall :103~160 인라인). 분기 순서대로. Serpen 판(df0e90→dcbe00)과 동형 — slot 1 · ca5 · +0x404/+0x403 · goal_data.epic +0x88/+0x98 · fallback Bottom · E 에 거리 게이트.
// SubPlan 태그(★0.6.0 u64 0x8000000000000000+idx): Steal ..10 · EpicCheck ..08 · EpicHunt ..09 · EpicPoke ..0a · Recall ..03 · LineDefense ..00 · ★0.6.0 ObjContest ..11 {slot @+8, poke @+0x10}
t = player.info.team(+0xa00); pos = player.info.position(+0xa90)
champ = cache.player_champion[t][pos].unwrap()                                   // :32

// ── A. 스틸 모드 (:35~44)
if self.focus_epic_only || self.vision_only {
   moba = game.get_game_mode() (vt+0x40) → tag 0 = Moba 아니면 unwrap 패닉      // :36
   epic = moba.jungle_runner.epic.live_list.first().and_then(|id| game.get_entity_by_id(id))   // :37 (moba+0x1a0 ptr/+0x1a8 len · vt+0x1f0)
   if epic.is_some() → return Steal{ last_vision_tick:0, target:Some(Epic), commit:self.focus_epic_only }   // :39
   else → return EpicCheck{ move_check:false }                                        // :44
}

// ── B. 일반 (:47~)
hp_ratio = champ.hp*100 / champ.stat_cached.hp                                    // :47 (max_hp 0 → 패닉)
can_upgrade_item = upgrade_item(version, rnd, player, game, ctx).is_some()          // :48 · 콜리 v3: e7a8c0→f27140(동치)
epic = moba.jungle_runner.epic.live_list.first().and_then(get_entity_by_id)         // :49
if epic.is_none() → return EpicCheck{false}                                        // :50~51
(x0,y0,x1,y1) = map.fountains[t] (map+0x6d70+t*0x20)                               // :56
is_in_heal_area = x0<=champ.x<=x1 && y0<=champ.y<=y1                                // :57
// ★0.6.0 :58 v3 재작성 (v<3 `epic.hp==max && (hp_ratio<51 || can_upgrade || (in_heal && hp<max)) → Recall` 사장)
if epic.hp(+0x670) == epic.stat_cached.hp(+0x628) && (can_upgrade_item || (is_in_heal_area && champ.hp < champ.stat_cached.hp)) {   // ★0.6.0 hp_ratio<51 항 삭제
   if tp.v6_obj(+0xcc7) != 1 → return Recall                                        // ★0.6.0 (cc7 상시 1)
   if !finish_before_enemy(tp, 1, player, data) → return Recall                     // ★0.6.0 ef7590(slot 1)
   tp.c30(+0xc30) += 1   // 귀환 보류 · 계속                                          // ★0.6.0
}

// ── C. 팀 오브젝트 게이트 (:63~64)
if tp.serpen_scene_none(+0x3e4) != 2 → return Recall                                // :63 ★0.6.0 `objective != Some(Morgard{..})` 이중모드 치환 ①(세르펜 신씬이 살아 있으면 Morgard 아님)
if tp.morgard_scene_none(+0x404) == 2 && tp.objective_tag(+0xcd5) != 0 → return Recall   // ★0.6.0 ②(양 씬 None 이면 레거시 objective 가 Morgard(0) 여야)
if tp.cc7 && bt(tp.v6_assemble_released[1](+0xca5) as u32, pos) { tp.c38(+0xc38) += 1; return Recall }   // ★0.6.0 v3 부적격→귀환 비트(version>2 게이트 · 상시)
phase = if tp.+0x404 == 2 { tp.objective_phase(+0xcd6) } else { tp.morgard_phase(+0x400) }               // ★0.6.0
if tp.cc7 && phase == 2 (Assemble) → return ObjContest{ slot:1, poke: epic.hp < epic.stat_cached.hp }     // ★0.6.0 신규
if phase == Hunt(3) {                                                                // :64 take_hunt_commit
   strategy = if game.is_solorank()(vt+0xe8) { player.strategy(+0x568) } else { game.strategy(team)(vt+0x108) }   // :65 (구 player.strategy(rnd, game))
   if strategy.object_buildup == Split(p) (tag<5) && p == pos                        // :66
      && tp.v4_buildup(+0xcc2) == 0                                                  // ★0.6.0 조건 추가(cc2 상시 0 = 실효 동치)
      && v25_objective_splitter_can_stay(version, player, data, Morgard(4))          // :68 · 콜리 v3: →f88b30
      → return LineDefense{ style:Aggressive(0), line:fallback_line(ctx, Bottom) (tutorial∈{0,1,3,5,6,7,8}→Bottom(2) · 2→표 143a873c0 · 4→표 143a87350), minion_action_type:Push(2) }   // :70~73
   // ★0.6.0 신규(구 :77 앞): v3
   if tp.cc7 && !contest_can_hold(tp, 1, pos, player, data) && min(goal_data.epic.epic_enemy_tick(+0x88), goal_data.epic.epic_ally_tick(+0x98)) > tps*5 { tp.c38 += 1; return Recall }   // ★0.6.0 (v<=2 판 `cc7 && hp_ratio<50 && min>5tps → Recall` 사장)
   if !tp.cc7 || tp.+0x404 == 2 || tp.morgard_stance_grind(+0x403) & 1 != 0 → return EpicHunt{ need_recall:false }   // :77 ★0.6.0 조건부
   else → return ObjContest{ slot:1, poke:true }                                                                   // ★0.6.0
}

// ── D. check_recall(self, version, player, data, goal_data, debug) (:79, 본문 :103~160) — 그대로
   champ = player_champion[t][pos].unwrap()                                          // :104
   if heal_area(t).contains(champ) && champ.hp < champ.max_hp → true                 // :107~108 (팀0 x<=64000&&y>=896000&&y<=960000 / 팀1 x>=892000&&x<=960000&&y<=64000)
   hp_ratio = champ.hp*100/champ.max_hp                                              // :112
   if min(goal_data.epic.epic_enemy_tick(+0x88), epic_ally_tick(+0x98)) <= tps*5 → false   // :115
   if self.v46_flee {                                                                // :124
      if self.v46_flee_threats.iter().any(|id| game.get_entity_by_id(id).is_some_and(|e| dist_sq(e,champ) < 40000000001)) → true   // :125~126
      self.v46_flee=false; self.v46_flee_threats.clear()                              // :131
   } else {
      (code, threats) = v46_flee_gate_check(version, player, data, champ)             // :134 · 콜리 v3: d3b2a0→eed7f0(특성 A/B · v3 *tps)
      if code == 0 { self.v46_flee=true; self.v46_flee_threats = threats.to_vec();    // :136~137
                     if ctx.debug(+0x3b) { debug.add_log(data, player, format!(.. tick, position, .. threats)) }   // :138~139
                     return true }                                                    // :140
   }
   enemy_cnt = player_champion[1-t].iter().flatten().filter(|e| map.regions[e.y/32000][e.x/32000]==7 && blackboard[1-t](bb+(1-t)*0x5c8).is_recent_visible(game, player, e)).count()   // :147~149
   ally_cnt  = player_champion[t].iter().flatten().filter(|a| regions==7 && blackboard[t].is_recent_visible(game, player, a)).count()                 // :154~155
   return hp_ratio < 21 && ally_cnt < enemy_cnt                                      // :160
if check_recall(..) → return Recall                                                  // :79~80

// ── E. 캠프 확인/견제 (:82~89) — 그대로
if team_plan.v24_objective_setup_should_check_camp(version, player, data, goal_data, Morgard(4)) → return EpicCheck{false}   // :82~83 · 콜리 v3: dd5db0→f0cfe0
if dist_sq(champ, epic) < 22500000001 → return EpicPoke                             // :86 → :87
else → return EpicCheck{false}                                                       // :89
```

## changes
- B(:58): v3 조건 = `epic.hp==max && (can_upgrade || (in_heal && hp<max))`(hp_ratio<51 삭제) → `cc7!=1 || !ef7590(tp,1,..) ? Recall : {c30++; 계속}`.
- C(:63): `objective != Morgard` → `3e4!=2 → Recall ; 404==2 && cd5!=0 → Recall` · 신규 `cc7 && bt(ca5,pos) → {c38++; Recall}` · `phase = (404==2) ? cd6 : 400` · 신규 `cc7 && phase==2 → ObjContest{1, poke: epic.hp<max}`.
- Hunt(:66~77): Split 조건에 `cc2==0` 추가 · strategy 인라인 · 신규 `cc7 && !ef6da0(tp,1,..) && min(gd.88,gd.98)>5tps → {c38++; Recall}` · `!cc7 || 404==2 || 403&1 → EpicHunt{false} else ObjContest{1,true}`.
- D/E 그대로(hp<21 · regions==7 · 200000²+1 · 150000²+1). 콜리: f27140 · eed7f0 · f88b30(…,4) · f0cfe0(…,4) · 신규 ef7590/ef6da0. SubPlan 태그 u64 · ObjContest 0x11. PlayerState +0xa00/+0xa90.
- v<3 분기 사장(B 구식 조건 · Hunt hp_ratio<50 항).

## verified
- capstone 0.6.0 `fd7f70..fd933c`(1171줄) 정독: B(`f27140` · `cmp version,3; jae` · `epic.hp!=max → C` · `can_upgrade → cc7 검사` · `!(in_heal && hp<max) → C` · `cc7!=1 → Recall(..03)` · `ef7590(tp,1,player,data)` false → Recall · `lock inc +0xc30`) · C(`3e4!=2 → Recall` · `404==2 && cd5!=0 → Recall` · `version>2 && cc7 → bt [+0xca5],pos → lock inc +0xc38 → Recall` · phase ptr `cd6`/`+0x400` · `==2 → ObjContest{[+8]=1,[+0x10]=epic.hp<max}` · `==3 → vt+0xe8 → +0x568/vt+0x108` · `tag<5 && ==pos && cc2==0 → f88b30(…,4)` → LineDefense{[+8]=0,[+9]=line,[+0xa]=2} (마스크 0x1eb → 2 / 표 143a873c0·143a87350) · `version>2: cc7==0 → 87a9; ef6da0(tp,1,…) → 87a9; min(gd+0x88,gd+0x98) > 5tps → c38++ Recall` · `87a9: cc7==0 or 404==2 or 403&1 → EpicHunt(..09) else ObjContest{1,1}`) · D(`0x9502f9001` · `eed7f0` · `cmp rsi,7 ×10` regions · `cmp hp_ratio,0x14`) · E(`f0cfe0(…,4)` · `0x53d1ac101 → EpicPoke(..0a)` else EpicCheck(..08)).
- Ghidra 디컴 타임아웃(RE 와 동일). 미확인: D 줄단위(RE w9 「그대로」) · 콜리 내부.

## confidence
A- — 변경 지점 전부 asm 로 확인.
