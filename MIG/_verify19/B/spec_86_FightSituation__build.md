---

### `86` FightSituation::build — 교전 상황 요약 구조체(FightSituation 128B) 조립 — 내 HP%/스킬·궁 준비, 전위 아군 유무, 위기 캐리, 아군/적 DPS 합과 우위를 계산

| 항목 | 값 |
|---|---|
| id | `fight_model__FightSituation_build` |
| 심볼 | `_RNvMs_NtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old11fight_modelNtB4_14FightSituation5build` |
| 소스 | `game-ai\src\plan_legacy\old\fight_model.rs:129` |
| IR | `m10.ll` 28246~29291행 |
| 경로·가시성 | `game_ai::plan_legacy::old::FightSituation::build` · **pub** |
| 계층 | 레거시 플랜 |
| exe | `dfe2a0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r10` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData, usize, usize, usize, usize, usize, &game_core::Entity, usize, bool, bool, bool, bool, bool, bool, usize) -> game_ai::plan_legacy::old::FightSituation
```

<details><summary>인자 21개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | *mut FightSituation(128B) | %0. 출력. 필드 배치는 writes 참조 | 4 |
| 1 | 1 | version | usize | %1. 본문 분기 없음 — get_battle_role·check_kill_die_tick·is_enemy_well_danger·fight_dps 에 그대로 전달(%29 스택에 저장 후 재로드) | 4 |
| 2 | 2 | rnd | &mut StdRng(320B) | %2. check_kill_die_tick 에만 전달 | 4 |
| 3 | 3 | player | &PlayerState(2528B) | %3. info.team(0x930)·info.position 태그(0x9c0) 읽음. is_enemy_well_danger/is_recent_visible/get_battle_role 에 전달 | 4 |
| 4 | 4 | data | &OperationData(24B) | %4. cache(+0)·context(+8)·blackboard(+0x10) 사용 | 4 |
| 5 | 5 | team_plan | &TeamPlan(1064B) | %5. v54 계측 카운터 2개만 atomic 증가(+0x3f8, +0x400) | 4 |
| 6 | 6 | debug | &mut DebugFrameData(224B) | %6. check_kill_die_tick 에 전달만 | 4 |
| 7 | 7 | near_ally_count | usize | %7. 그대로 출력 +0x10 | 4 |
| 8 | 8 | near_enemy_count | usize | %8. 그대로 출력 +0x18 | 4 |
| 9 | 9 | can_near_enemies | usize | %9. 그대로 출력 +0x20 | 4 |
| 10 | 10 | my_die_tick | usize | %10. 그대로 출력 +0x28 | 4 |
| 11 | 11 | my_die_tick_no_tower | usize | %11. 그대로 출력 +0x30 | 4 |
| 12 | 12 | focused | &Entity(1728B) | %12. x/y(0x660/0x668)·id(0x5c0) 읽음 | 4 |
| 13 | 13 | focused_die_tick | usize | %13. 그대로 출력 +0x38 | 4 |
| 14 | 14 | focused_is_in_range | bool | %14. 그대로 출력 +0x70 | 4 |
| 15 | 15 | warn_tower | bool | %15. 그대로 출력 +0x71 | 4 |
| 16 | 16 | is_in_tower_range | bool | %16. 그대로 출력 +0x72 | 4 |
| 17 | 17 | is_in_tower_focused | bool | %17. 그대로 출력 +0x73 | 4 |
| 18 | 18 | is_dive_current_tower | bool | %18. 그대로 출력 +0x74 | 4 |
| 19 | 19 | focused_is_in_tower | bool | %19. 그대로 출력 +0x75 | 4 |
| 20 | 20 | target_hp_ratio | usize | %20. 그대로 출력 +0x48 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
// fight_model.rs:129~238. 판정 없이 FightSituation 을 '조립'하는 함수. prof 타이머(phase 49) 는 prof::ENABLED 일 때만.
t = player.info.team (0x930); pos = player.info.position 태그 (0x9c0)
champ = cache.player_champion[t][pos].unwrap()                         // :140 (None → unwrap_failed 패닉)
my_battle_role = get_battle_role(version, ctx, cache, player)           // :143
my_hp_ratio = champ.hp*100 / max(champ.stat_cached.hp, 1)             // :144
my_skills_ready = champ.skill_cooldown() < 31 || champ.skill2_cooldown() < 31   // :145 (비챔프 엔티티면 cooldown()=0 → true)
my_ult_ready = champ.can_ult()                                         // :146
my_dist_to_focused = dist_sq(champ, focused)                           // :149

// :150~163 has_frontline_ally
has_frontline_ally = (0..5).filter(p != pos)
   .filter_map(p → (player_champion[t][p]?, player_state[t][p]?))
   .any(|(p, ally)| get_battle_role(version, ctx, cache, ally_state) < 2 /*Tanker|Initiator*/ && dist_sq(ally, focused) < my_dist_to_focused)

// :165~191 endangered_carry — 첫 매치에서 중단(find_map)
endangered_carry = None
for p in 0..5 { if p == pos continue;
   ally = player_champion[t][p]?; ally_state = player_state[t][p]?;
   role = get_battle_role(version, ctx, cache, ally_state); if (role & 6) != 2 continue;   // BaseAttacker|SkillCaster 만
   enemies: bumpalo Vec<&Entity> = player_champion[1-t].iter().flatten().filter(|e|          // closure$0 (aux m10:54399)
        dist_sq(e, ally) <= (max_range_cached(data, e, ally) + 30000)^2                       // :175
        && !is_ignored_well_enemy(version, player, e)   /* 인라인: e.team==Player(1-t) && is_enemy_well_danger(version, player, e.x, e.y) */  // :176
        && blackboard[1-t].is_recent_visible(game, player, e))                                   // :177
   die_tick = check_kill_die_tick(version, rnd, data, judger=ally_state, focus=ally, enemy=&enemies, towers=&[] /*빈 Vec*/, debug)   // :180
   if die_tick < tps*2 { endangered_carry = Some(ally.id); break }                              // :182~183
}

// :194~216 ally_dps_sum — ★자기 자신 제외 없음(p==pos 도 포함, dist 0)
ally_dps_sum = 0
for p in 0..5 { ally = player_champion[t][p]?;
   if dist_sq(ally, champ) >= 14400000001 continue;                    // :200  (≤120000 만)
   nearest_enemy_champ = player_champion[1-t].iter().flatten()          // :203~206 (첫 후보는 본체 인라인, 나머지는 aux m12:15597 폴드)
        .filter(|e| !is_ignored_well_enemy(version, player, e) && blackboard[1-t].is_recent_visible(game, player, e))   // closure$5 :204~205
        .min_by_key(|e| dist_sq(e, ally))                              // closure$6 :206 (동점이면 앞쪽 유지)
   if let Some(ne) = nearest_enemy_champ {
      team_plan.v54_fs_pairings += 1 (atomic)                          // :207 계측
      if !blackboard[1-t].is_recent_visible(game, player, ne) { team_plan.v54_fs_unseen_picks += 1 }   // :209 계측(필터를 통과했으므로 사실상 0)
      ally_dps_sum += fight_dps(version, ctx, attacker=ally, enemy=ne)   // :211
   }
}

// :218~223 enemy_dps_sum
enemy_dps_sum = 0
for enemy in cache.iter_champions(player_champion[1-t]) {
   if dist_sq(enemy, champ) >= 22500000001 continue;                  // :219 (≤150000 만)
   if is_ignored_well_enemy(version, player, enemy) continue;          // :220 인라인(TeamType 0 + is_enemy_well_danger)
   if !blackboard[1-t].is_recent_visible(game, player, enemy) continue; // :221
   enemy_dps_sum += fight_dps(version, ctx, attacker=enemy, enemy=champ)   // :222
}
team_dps_advantage = ally_dps_sum - enemy_dps_sum                     // :226 (i64)

write FightSituation { endangered_carry, 패스스루 인자 11개, focused_id=focused.id, my_hp_ratio, team_dps_advantage, ally_dps_sum, enemy_dps_sum, my_skills_ready, my_ult_ready, has_frontline_ally, my_battle_role }   // :228
// prof 타이머 drop(:238): PHASE_NANOS[49] += 경과ns, PHASE_CALLS[49] += 1 (ENABLED 일 때만)
```

**`mem` 메모리 접근 48건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | 내 팀 t. 적 팀 = 1-t (`sub nuw nsw i64 1, %41`). 2 이상이면 panic_bounds_check | 4 | OK |  |
| 1 | PlayerState | 0x9c0 | info.position@tag | r | 내 포지션 인덱스(i32, range 0..5) — player_champion[t][pos] 조회·자기 자신 제외 비교에 사용 | 4 | OK |  |
| 2 | OperationData | 0x0 | cache | r | &AbstractGameWithCache(8840B) | 4 | OK |  |
| 3 | OperationData | 0x8 | context | r | &GameContext(64B) | 4 | OK |  |
| 4 | OperationData | 0x10 | blackboard | r | &[Blackboard;2]. 인덱스 1-t(적 팀) 로 is_recent_visible 호출 — _docs game_core.txt:21 「Blackboard[team]은 team 팀 자체 정보 추적, 관측은 1-team」 | 4 | OK |  |
| 5 | AbstractGameWithCache | 0x0 | game.data_ptr | r | &dyn AbstractGame 데이터 포인터 — is_recent_visible 인자 | 4 | OK |  |
| 6 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r | &dyn AbstractGame vtable(816B) — is_recent_visible 인자 | 4 | OK |  |
| 7 | AbstractGameWithCache | 0x1e0 | player_champion | r | [2][5] Option<&Entity>(니치, null=None). [t][pos]=내 챔프(None 이면 unwrap_failed 패닉), [t][p]=아군, [1-t][*]=적 배열(m10:28469 `gep [5 x ptr], %52, %154`) | 4 | OK |  |
| 8 | AbstractGameWithCache | 0x230 | player_state[][] | r | [2][5] Option<&PlayerState>. 아군 p 의 state 를 get_battle_role·check_kill_die_tick(judger) 에 전달 | 4 | OK |  |
| 9 | GameContext | 0x0 | pool | r | bumpalo Bump — 빈 towers Vec(32B `{8, pool, 0, 0}`)·적 Vec 할당자 | 4 | OK |  |
| 10 | GameContext | 0x8 | setting | r | &GameSetting | 4 | OK |  |
| 11 | GameSetting | 0x12f8 | tick_per_second | r | tps. endangered_carry 임계 tps*2 (shl 1) | 4 | OK |  |
| 12 | Entity | 0x0 | team@tag | r | TeamType 태그(0=Player). is_ignored_well_enemy 인라인분(fight_model.rs:755): tag==0 && team.0==1-t | 4 | OK |  |
| 13 | Entity | 0x8 | team@Player.0 | r | 팀 번호 | 4 | OK |  |
| 14 | Entity | 0x68 | ty@tag | r | EntityType 태그. 13=Champion 일 때만 skill_cooldown/skill2_cooldown 읽음(비챔프면 skill_cooldown()=0 → ready 취급) | 4 | OK |  |
| 15 | Entity | 0xb8 | ty@Champion.0.skill_cooldown | r | my_skills_ready 판정(entity.rs:1775~1776 인라인) | 4 | OK |  |
| 16 | Entity | 0xc0 | ty@Champion.0.skill2_cooldown | r | my_skills_ready 판정(entity.rs:1791 인라인) | 4 | OK |  |
| 17 | Entity | 0x5c0 | id | r | focused_id(출력 +0x40)·endangered_carry 의 값(아군 id) | 4 | OK |  |
| 18 | Entity | 0x628 | stat_cached.hp | r | 내 최대 HP. max(.,1) 로 0 나눗셈 방지 | 4 | OK |  |
| 19 | Entity | 0x660 | x | r | distance_sq(entity.rs:2158 인라인) — 내/아군/적/focused 전부 | 4 | OK |  |
| 20 | Entity | 0x668 | y | r | distance_sq | 4 | OK |  |
| 21 | Entity | 0x670 | hp | r | 내 현재 HP → my_hp_ratio = hp*100/max(max_hp,1) | 4 | OK |  |
| 22 | TeamPlan | 0x3f8 | v54_fs_pairings | r | atomicrmw add 1 — ally 마다 nearest_enemy_champ 가 Some 일 때(fight_model.rs:207). 계측 전용 | 4 | OK |  |
| 23 | TeamPlan | 0x400 | v54_fs_unseen_picks | r | atomicrmw add 1 — 선택된 nearest 가 !is_recent_visible 일 때(:209). 계측 전용(선택 결과엔 영향 없음) | 4 | OK |  |
| 24 | FightSituation | 0x0 | endangered_carry@tag | w | Option<usize> 태그: 아군 루프가 5 미만에서 끝났으면(=찾음) 1(Some), 소진이면 0(None) | 4 | OK | zext(찾음) |
| 25 | FightSituation | 0x8 | endangered_carry@Some.0 | w | None 이면 undef | 4 | OK | ally.id(0x5c0) |
| 26 | FightSituation | 0x10 | near_ally_count | w | 패스스루 | 4 | OK | arg7 |
| 27 | FightSituation | 0x18 | near_enemy_count | w | 패스스루 | 4 | OK | arg8 |
| 28 | FightSituation | 0x20 | can_near_enemies | w | 패스스루 | 4 | OK | arg9 |
| 29 | FightSituation | 0x28 | my_die_tick | w | 패스스루 | 4 | OK | arg10 |
| 30 | FightSituation | 0x30 | my_die_tick_no_tower | w | 패스스루 | 4 | OK | arg11 |
| 31 | FightSituation | 0x38 | focused_die_tick | w | 패스스루 | 4 | OK | arg13 |
| 32 | FightSituation | 0x40 | focused_id | w |  | 4 | OK | focused.id(0x5c0) |
| 33 | FightSituation | 0x48 | target_hp_ratio | w | 패스스루 | 4 | OK | arg20 |
| 34 | FightSituation | 0x50 | my_hp_ratio | w | fight_model.rs:144 · udiv | 4 | OK | hp*100 / max(stat_cached.hp,1) |
| 35 | FightSituation | 0x58 | team_dps_advantage | w | i64 wrapping sub (:226) | 4 | OK | ally_dps_sum - enemy_dps_sum |
| 36 | FightSituation | 0x60 | ally_dps_sum | w | :198~216 | 4 | OK | Σ fight_dps(version, ctx, ally, nearest_enemy) |
| 37 | FightSituation | 0x68 | enemy_dps_sum | w | :218~223 | 4 | OK | Σ fight_dps(version, ctx, enemy, me) |
| 38 | FightSituation | 0x70 | focused_is_in_range | w | 패스스루 | 4 | OK | arg14 |
| 39 | FightSituation | 0x71 | warn_tower | w | 패스스루 | 4 | OK | arg15 |
| 40 | FightSituation | 0x72 | is_in_tower_range | w | 패스스루 | 4 | OK | arg16 |
| 41 | FightSituation | 0x73 | is_in_tower_focused | w | 패스스루 | 4 | OK | arg17 |
| 42 | FightSituation | 0x74 | is_dive_current_tower | w | 패스스루 | 4 | OK | arg18 |
| 43 | FightSituation | 0x75 | focused_is_in_tower | w | 패스스루 | 4 | OK | arg19 |
| 44 | FightSituation | 0x76 | my_skills_ready | w | :145 | 4 | OK | skill_cd<31 \|\| skill2_cd<31 (비챔프=true) |
| 45 | FightSituation | 0x77 | my_ult_ready | w | :146 | 4 | OK | Entity::can_ult(me) |
| 46 | FightSituation | 0x78 | has_frontline_ally | w | :150~163 | 4 | OK | any(아군 role∈{Tanker,Initiator} && dist_sq(ally,focused) < my_dist_to_focused) |
| 47 | FightSituation | 0x79 | my_battle_role | w | :143 | 4 | OK | get_battle_role(version, ctx, cache, player) |

**`consts` 상수 10건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev | folded_from |
|---|---|---|---|---|---|---|
| 0 | 13 | 145 | 태그 | EntityType 메모리태그 13 = Champion (tcxdict --enum EntityType). skill_cooldown() 인라인의 match 판별 | 3 |  |
| 1 | 31 | 145 | 임계 | 스킬/스킬2 쿨다운 임계(틱). `icmp ult cd, 31` — 소스가 `< 31` 인지 `<= 30` 인지는 표기 불가(외연 동일). 60tps 기준 약 0.5초 | 4 |  |
| 2 | 100 | 144 | 계수 | my_hp_ratio 백분율 계수(hp*100/max_hp) | 4 |  |
| 3 | 2 | 157 | 태그 | has_frontline_ally: BattleRole 태그 < 2 = Tanker(0)\|Initiator(1) (`icmp samesign ult i8 role, 2`) | 4 |  |
| 4 | 6 | 171 | 계수 | endangered_carry 후보 role 마스크: `(role & 6) == 2` ⇔ role ∈ {BaseAttacker(2), SkillCaster(3)} — matches! 가 비트마스크로 접힘 | 4 |  |
| 5 | 30000 | 175 | 미상 | [aux m10.ll:54430] 적 필터 여유 거리: dist_sq(enemy, ally) > (max_range_cached(data,enemy,ally)+30000)^2 이면 제외 (30000 ≈ 0.94셀) | 4 |  |
| 6 | 1 | 182 | 인덱스 | tps*2 (=2초). `shl i64 %tps, 1` 로 접힘. die_tick < tps*2 이면 그 캐리가 endangered_carry | 4 | 2 |
| 7 | 14400000001 | 200 | 임계 | 120000^2 + 1 — 아군 DPS 합산 대상 반경: dist_sq(ally, me) < 이 값 ⇔ 거리 ≤ 120000 (3.75셀). exe 는 `<= 14400000000` 으로 인코딩(동치) | 4 |  |
| 8 | 22500000001 | 219 | 임계 | 150000^2 + 1 — 적 DPS 합산 대상 반경: dist_sq(enemy, me) < 이 값 ⇔ 거리 ≤ 150000 (약 4.7셀) | 4 |  |
| 9 | 0 | 220 | 태그 | TeamType 메모리태그 0 = Player (is_ignored_well_enemy 인라인분 :755 `entity.team == TeamType::Player(1-t)`) | 4 |  |

**`knobs` 조정점 7건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 스킬 준비 판정 쿨다운 임계 | fight_model.rs:145 | 31 | 올리면 쿨이 더 남아 있어도 my_skills_ready=true 로 봐서 교전 적극성이 오르고, 내리면(1) 사실상 즉시 사용 가능일 때만 true | 4 | 기존 |
| 1 | 위기 캐리(endangered_carry) 생존 임계 | fight_model.rs:182 | 2 | tps*2(2초). 올리면 더 여유 있는 캐리도 '위기'로 잡혀 구조/보호 계열 판단이 잦아지고, 내리면 거의 죽기 직전만 잡힌다 | 4 | 기존 |
| 2 | 위기 캐리 적 필터 여유 거리 | fight_model.rs:175 | 30000 | max_range+30000 안의 적만 die_tick 계산에 들어간다. 올리면 더 먼 적까지 위협으로 계산해 die_tick 이 짧아진다(위기 판정 증가) | 4 | 기존 |
| 3 | 아군 DPS 합산 반경 | fight_model.rs:200 | 14400000001 | 120000^2+1. 올리면 더 먼 아군의 DPS 까지 우리 편으로 계산해 team_dps_advantage 가 커진다(교전 낙관) | 4 | 기존 |
| 4 | 적 DPS 합산 반경 | fight_model.rs:219 | 22500000001 | 150000^2+1. 올리면 더 먼 적의 DPS 까지 계산해 team_dps_advantage 가 작아진다(교전 비관). 아군 반경(120000)보다 넓은 비대칭이 기본값 | 4 | 기존 |
| 5 | 전위 아군 역할 집합 | fight_model.rs:157 | 2 | BattleRole < 2(Tanker\|Initiator). 범위를 넓히면 has_frontline_ally 가 쉽게 true | 4 | 기존 |
| 6 | 위기 캐리 후보 역할 집합 | fight_model.rs:171 | 6 | (role&6)==2 ⇔ BaseAttacker\|SkillCaster. 바꾸면 어떤 역할을 '캐리'로 보호할지 달라진다 | 4 | 기존 |

<details><summary>`callees` 피호출자 15건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | can_ult | game_core::Entity::can_ult | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1734 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | check_kill_die_tick | game_ai::check_kill_die_tick | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::OperationData, &game_core::PlayerState, &game_core::Entity, bumpalo::collections::vec::Vec< &game_core::Entity>, bumpalo::collections::vec::Vec< &game_core::Entity>, &mut game_core::DebugFrameData) -> usize | game-ai\src\fight_check.rs:917 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 2 | drop | <game_core::prof::ProfTimer as std::ops::Drop>::drop | pub | fn(&mut game_core::prof::ProfTimer) | game-core\src\simulation\prof.rs:184 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 3 | fight_dps | game_ai::fight_dps | pub | fn(usize, &game_core::GameContext, &game_core::Entity, &game_core::Entity) -> usize | game-ai\src\fight_check.rs:32 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | get_battle_role | game_ai::get_battle_role | pub | fn(usize, &game_core::GameContext, &game_core::AbstractGameWithCache, &game_core::PlayerState) -> game_ai::BattleRole | game-ai\src\utils.rs:446 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 5 | is_enemy_well_danger | game_ai::is_enemy_well_danger | pub | fn(usize, &game_core::PlayerState, u64, u64) -> bool | game-ai\src\path_finder.rs:1032 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 6 | is_ignored_well_enemy | game_ai::plan_legacy::old::is_ignored_well_enemy | pub | fn(usize, &game_core::PlayerState, &game_core::Entity) -> bool | game-ai\src\plan_legacy\old\fight_model.rs:754 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 7 | is_recent_visible | game_core::Blackboard::is_recent_visible | pub | fn(&game_core::Blackboard, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\game\blackboard.rs:346 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 8 | iter_champions | game_core::AbstractGameWithCache::<'a, 'b>::iter_champions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5759 ~ game_core[6a30]::simulation::{impl#5}::iter_champions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1904 | True | True | 3 | IR 호출 심볼 일치(토큰 경계) |
| 9 | max_range_cached | game_ai::plan_legacy::old::max_range_cached | pub | fn(&game_core::OperationData, &game_core::Entity, &game_core::Entity) -> u64 | game-ai\src\plan_legacy\old\battle.rs:2332 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 10 | skill2_cooldown | game_core::Entity::skill2_cooldown | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1789 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 11 | skill_cooldown | game_core::Entity::skill_cooldown | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1774 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 12 | team | game_view::ClientData::team | pub | fn(&game_view::ClientData, usize) -> std::option::Option<std::cell::Ref< game_core::Team>> | game-view\src\logic\client\data.rs:1651 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 13 | team | game_view::ClientDatabase::team | pub | fn(&game_view::ClientDatabase, usize) -> std::option::Option<&game_core::Team> | game-view\src\logic\client\data.rs:968 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 14 | team | game_core::PlayerAiContext::<'a, 'b, 'r>::team | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:583 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
</details>

⚠**미매칭 4개**: `compare`, `cooldown`, `dist_sq`, `elapsed`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 3곳** (m05.ll:24749, m05.ll:32755, m10.ll:19693) · **형제 1개** (FightSituation)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | game_ai::plan_legacy::old::FightSituation::build | pub | game-ai\src\plan_legacy\old\fight_model.rs:129 | False | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::team_plan::TeamPlan, &mut game_core::DebugFrameData, usize, usize, usize, usize, usize, &game_core::Entity, usize, bool, bool, bool, bool, bool, bool, usize) -> game_ai::plan_legacy::old::FightSituation |

**`open` 5건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 표기 불가 | fight_model.rs:145 의 31 이 소스에서 `< 31` 인지 `<= 30` 인지 — 외연 동일(표기 불가). tps 연동(tps/2)인지 리터럴인지도 IR 에는 리터럴 31 만 남아 리터럴로 판단(udiv/shl 없음) | 4 |  |
| 1 | 미탐색 | ally_dps_sum 루프(:198)에 p != pos 제외가 없어 자기 자신이 포함되는데 이것이 의도인지(설계) — IR 관측 사실만 기록 | 4 |  |
| 2 | 미탐색 | :209 v54_fs_unseen_picks 는 필터(:205)가 이미 is_recent_visible 을 요구하므로 논리상 발화 불가로 보이나, is_recent_visible 이 상태(tick)에 의존하면 두 호출 사이에 값이 바뀔 수 있는지는 _gcbc 본문 미확인(미탐색) | 4 |  |
| 3 | 미탐색 | exe 0xdfe2a0 대조: exe 는 is_enemy_well_danger(상수 160001/64001 관측)·is_recent_visible(vtable 간접호출 8곳) 을 인라인해 call 수가 IR(직접 call 15종)과 다르다. 판정 상수 22500000001(+0xa3b)·14400000000(+0x788, `<=` 인코딩)·tps 오프셋 4856(+0x66d) 은 exe 본문에서 확인. 30000 은 closure 가 별도 함수라 본체엔 없음(정상). 로직 어긋남은 발견 못 함 | 4 |  |
| 4 | 미탐색 | prof phase id 49·PHASE_NANOS 배열 상한 132 는 계측 상수라 constants 에서 제외(판정 무관) | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

