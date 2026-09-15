---

### `194` HideSubPlan::action_candidates — [배치 L · hide.rs:0~101] Hide 서브플랜 행동 후보 루트: 적 시야 노출 갱신 → check_move 전엔 부시 위치 기준 Morgard/Serpen 캠프로 이동(AroundPosition) → check_move 후엔 부시 근처 최근접 적을 찾아 노출 시 아군/적 수·1v1 판정으로 전투(battle_action+Trace)/도주(RunAway), 비노출 시 부시 대기(AroundBush)+정글 스틸(Attack/Skill/Skill2). 배치 M(103~139) = check_move 거짓 분기·enemy_spotted_me 분기·반환

| 항목 | 값 |
|---|---|
| id | `hide__Hide__action_candidates` |
| 심볼 | `_RNvMNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy8sub_plan4hideNtB2_11HideSubPlan17action_candidates` |
| 소스 | `game-ai\src\plan_legacy\sub_plan\hide.rs:19` |
| IR | `m02.ll` 20312~25343행 |
| 경로·가시성 | `game_ai::plan_legacy::sub_plan::HideSubPlan::action_candidates` · **pub** |
| 계층 | 기타 |
| exe | `cb7540` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r16` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&mut game_ai::plan_legacy::sub_plan::HideSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay>
```

<details><summary>인자 8개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | bumpalo Vec<SmallActionPlay> 32B | ptr@0 · bump@+8 · cap@+0x10 · len@+0x18. 본 배치는 지역 res(%46, L20 m02.ll:20383~20389 Vec::new_in(bump)) 에 push/extend 만 하고, sret 로의 memcpy 는 L136/139(배치 M) 에서 \| (배치 M) ptr@0 · bump@+0x8 · cap@+0x10 · len@+0x18. 지역 alloca %46 을 만들어 채운 뒤 hide.rs:138 에서 32B memcpy 로 반환(IR 25340). 원소 184B · 태그 @+0xb1(니치, untagged=AroundPosition) — tcxdict --enum SmallActionPlay | 3 |
| 1 | 1 | self | &mut HideSubPlan(16B) | IR 속성 `noalias captures(none) dereferenceable(16)` — readonly 없음 = &mut. initializes 속성 없음(쓰기가 전부 조건부). 필드: +0 bush(usize) · +8 out_line(AroundBushOutlineType 1B) · +9 check_move(bool) · +0xa enemy_spotted_me(bool). 본 배치 쓰기 = +0xa(L27=1, L30=0) · +9(L45/L52=1) — writes 참조 \| (배치 M) IR 속성 noalias·비-readonly = &mut. 배치 M 범위에서는 **읽기만**(+0x9 check_move, IR 23986). self 쓰기(check_move=1 @L45·L52, enemy_spotted_me @L27·L30)는 전부 배치 L 범위 | 4 |
| 2 | 2 | version | usize | 전 함수에서 분기 없음 — battle_action 1번째 인자로만 전달(본 배치 L80 m02.ll:23732 · L87 23790; 배치 M 에 3곳 더). grep `%2` 전수 = 5 호출뿐 \| (배치 M) 배치 M 범위에선 분기 없음. battle_action 1번 인자로 그대로 전달(IR 25182·25240·25317) | 4 |
| 3 | 3 | rnd | &mut StdRng(320B) | L43/L50 AroundPosition::new_with_out_line · L96 AroundBush::new_with_out_line · L76 can1v1win · L80/L87 battle_action 에 그대로 전달. 이 함수 본체가 직접 읽지 않음 \| (배치 M) can1v1win 1번 인자 · battle_action 2번 인자로 전달만 | 4 |
| 4 | 4 | player | &PlayerState(2528B) | +0x930 info.team(L22) · +0x9c0 info.position@tag(L22, i32) 읽음. L61 클로저 캡처(is_recent_visible player 인자) · L84/L91 RunAway::new_with_skill · L96 AroundBush::new_with_out_line(_player) · can1v1win · battle_action 전달 \| (배치 M) readonly. can1v1win/battle_action/attack_summon_action/new_with_skill 에 전달 | 4 |
| 5 | 5 | data | &OperationData(24B) | +0 cache(&AbstractGameWithCache) · +8 context(&GameContext; +0 pool=bump, +8 setting, +0x20 map) · +0x10 blackboard(&[Blackboard;2]) \| (배치 M) readonly. +0 cache(&AbstractGameWithCache) 로 player_champion 순회 · 콜리 전달 | 4 |
| 6 | 6 | _parameter | &ScoreParameter(5384B) | DI 이름 `_parameter`. 전 함수에서 미사용(define 줄 외 `%6` 출현 0) — 시그니처만 채움 \| (배치 M) readonly. 배치 M 범위 미사용 | 4 |
| 7 | 7 | _debug | &mut DebugFrameData(224B) | DI 이름 `_debug`. IR 속성 readnone = 전 함수에서 읽지 않음(정정: tcx 시그니처는 `&mut DebugFrameData`, 이 함수엔 %8 없음 — 인자 8개 %0~%7) \| (배치 M) readnone. 미사용 | 3 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
// hide.rs:0~101 (배치 L)
// 진입: 함수 머리. 인자 = (&mut self HideSubPlan16B, version, rnd, player, data, _parameter, _debug) -> Vec<SmallActionPlay>

// L20  res = Vec::new_in(data.context.pool)                                   // m02.ll:20378~20389
// L22  team = player.info.team(+0x930); assert team<2 (panic_bounds_check 20397)
//      champ = data.cache.player_champion[team][player.info.position@tag(+0x9c0)].unwrap()   // 20410~20423 · None→unwrap_failed 20446
// L23  is_visible_to_enemy = game.is_visible(enemy_team=1-team, champ.id)      // vtable+0xf8 · 20430~20442
// L26  if is_visible_to_enemy {
// L27      self.enemy_spotted_me = true;                                       // store i8 1 → self+0xa 20465
//      } else if self.check_move {                                             // 20455~20457 (표기: `else if` 인지 `else { if }` 인지 표기 불가 — 블록 구조는 이와 동치)
// L30      self.enemy_spotted_me = false;                                      // 20589
//      }
// L34  if !self.check_move {                                                   // 20467~20470 / 20457
// L35      let (x,y) = BUSH_POSITIONS.into_iter().filter(|&(x,y)| map.bushes[y][x] == self.bush).next().unwrap();
//                 // map = data.context.map(+0x20) · bushes @MapDef+0x1c98 [y][x] · 71개 상수 셀 @anon.148 · unwrap 실패 20597
// L37      bx = x*32000+16000; by = y*32000+16000;                             // 20607~20612 (by 는 −(…) 로 접힘)
// L39      if is_top_side(context, bx, by) {   // = (setting.height(+0x12c0) − by) >= bx (MIR map_regions.rs:22~24) · IR %147 = !is_top (20621) → 149(L48) / 151(L40)
// L40          (ex,ey) = map.camp_pos(JungleType::Morgard(4), blue_side = team==0)   // 20630
// L42          if champ.distance_sq(ex,ey) > 100000² {                         // 20726~20747 (dx,dy=|Δ| · dx²+dy² ugt 10000000000)
// L43              res.push(AroundPosition(SmallActionAroundPosition::new_with_out_line(rnd, data, ex, ey, 5, Outline(1))))   // 20757 · push 20778~20797
//              } else {
// L45              self.check_move = true;                                     // 20751
//              }
// L48      } else {
//              (ex,ey) = map.camp_pos(JungleType::Serpen(5), team==0)          // 20626
// L49          if champ.distance_sq(ex,ey) > 100000² {                         // 20642~20663
// L50              res.push(AroundPosition(new_with_out_line(rnd, data, ex, ey, 5, Outline)))   // 20673 · 20694~20713
//              } else {
// L52              self.check_move = true;                                     // 20667
//              }
//          }
//      }
// L57  if self.check_move {                                                    // 재로드 20578 (123) / 20583 (128: visible&&check_move) / 20591 (131: !visible&&check_move)
//      // ---- 거짓 분기: → 배치 M(줄 107) — 123→%211(107) · 128→%244(108/110, enemy_spotted_me=true 상수접힘) · 131→%243(136, enemy_spotted_me=false 접힘)
// L58      let (x,y) = BUSH_POSITIONS…filter(map.bushes[y][x]==self.bush).next().unwrap();   // 20811~20907 · 실패 21288
// L59      bx = x*32000+16000; by = y*32000+16000;                             // 21298~21305 (스택 %39/%38 — 클로저가 참조 캡처)
// L60      enemies = data.cache.player_champion[enemy_team]   (5칸 Option<&Entity>)   // 21306~21313
// L61      nearest_enemy: Option<&Entity> = iter_champions(enemies)
//              .filter(|e| data.blackboard[enemy_team].is_recent_visible(game, player, e)   // 21414 (aux m12.ll:7823 는 blackboard[1-player.info.team] 로 재계산)
// L62                   && e.distance_sq(bx,by) < 250000²+1)                    // 21430~21457 / aux 7839~7866
// L63          .min_by_key(|e| e.distance_sq(champ))                            // 첫 원소 key 21488~21508 → fold 21511(aux) · 동점 시 앞 원소 유지(aux 7920~7924)
//              // 결과 { i64 key, ptr } — ptr null = None (21515~21518)
// L66      if is_visible_to_enemy {                                            // 21519 / 21524
// L67          if let Some(enemy) = nearest_enemy {                            // 21535 (None → L91)
// L70              nearby_allies = iter_champions(player_champion[team]).filter(|e| e.id != champ.id && e.distance_sq(champ) < 150000²+1).count()   // closure$5(L69) · 5칸 unrolled 22596~22919 · 결과 %1035
// L73              nearby_enemies = iter_champions(player_champion[enemy_team]).filter(|e| e.is_visible_from(champ) && e.distance_sq(champ) < 150000²+1).count()
//                      // closure$6(L72) · is_visible_from 인라인(entity.rs:1481): champ.team Neutral(태그1)→항상 참(1043 경로 22973~22919) / Player(t)→ t<2 검사(23271, 실패 panic 23651) 후 e.visible_state[t]@tag==0(23309~23313) · 결과 %1332(23710)
// L75              dominated = nearby_allies < nearby_enemies                   // 23712 icmp ult
// L76              can_fight = !dominated && can1v1win(rnd, player, data, champ, enemy)   // 23714 (dominated 면 호출 생략) · 23717
// L78              if can_fight {                                              // 23728
// L80                  res.extend(battle_action(version, rnd, player, data, 5))   // 23732 · extend 23808
// L81                  res.push(Trace(SmallActionTrace::new(data, enemy.id, 5)))   // 인라인: t=game.get_entity_by_id(enemy.id)(23827) · start_tick=game.tick()(23835) · goal=(t.x,t.y) or (0,0)(23839~23860) · margin 15000 · end_delay 5 · path_finder/last_escape None · 태그 14 (23861~23881) · push 23903~23926
//                  } else {
// L84                  res.push(RunAway(SmallActionRunAway::new_with_skill(data, player, 5, false)))   // 23723 · 태그 3 23738 · push 23760~23784
// L86                  if !dominated {                                         // 23786
// L87                      res.extend(battle_action(version, rnd, player, data, 5))   // 23790 · 23797
//                      }
//                  }
//                  → 배치 M(줄 107) %1424
// L91          } else {   // nearest_enemy None
//                  res.push(RunAway(new_with_skill(data, player, 5, false)))     // 22970 · 태그 3 23933 · push 23955~23979 → 배치 M(107) %1424
//              }
// L93      } else {   // !is_visible_to_enemy
//              if let Some(enemy) = nearest_enemy {                            // 21532
// L94              res.push(AroundBush(SmallActionAroundBush::new_with_target(data, enemy, self.bush, self.out_line)))   // 21543 · 태그 12 21558 · push 21580~21604
//              } else {
// L96              res.push(AroundBush(SmallActionAroundBush::new_with_out_line(rnd, data, player, self.bush, self.out_line)))   // 21552 · 태그 12 22506 · push 22528~22552
//              }
// L101         res.extend(self.steal_jungle_action(rnd, player, data));        // 전부 인라인(hide.rs:142~, define 없음) · extend 23992 → 배치 M(107) %1428
//              // ===== steal_jungle_action(&self, rnd, player, data) 인라인 본체 (루트 줄 101, IR 21619~22502 + 23990~23993) =====
//              // L143 steal_res = Vec::new_in(bump); steal_range = 150000       // 21632~21639
//              // L144 champ = cache.player_champion[team][pos].unwrap()         // 21640~21643 (재로드 · 21756 unwrap_failed)
//              // L147 bush = BUSH_POSITIONS.into_iter().filter(|&(x,y)| map.bushes[y][x]==self.bush).next()   // closure#0 · 21660~21753
//              // L149     .map(|(x,y)| (x*32000+16000, y*32000+16000))         // closure#1 · 21767~21770
//              // L150     .unwrap_or((champ.x, champ.y)) → (bx,by)              // 21774~21808 (phi %505/%506)
//              // L156 for jungle in cache.jungles (Vec<&Entity> +0xd0 ptr / +0xe8 len) {   // 21821~21871 · 끝나면 %1427 extend
//              // L158     if !jungle.ty.is_jungle(enemy_team) { continue }     // ty@tag(+0x68)==4 && ty.Jungle.info.camp_type.0(+0x98)==enemy_team (21879~21888)
//              // L163     if jungle.distance_sq(bx,by) > steal_range² { continue }   // 21891~21913 ugt 22500000000
//              // L168     move_speed = champ.stat_cached.move_speed(+0x640)    // 21916
//              // L171     if champ.can_attack() {                                // 21918
//              // L172         if let Some(atk) = &champ.attack_effect {         // +0x4c0 != -1 (21934~21936)
//              // L173             expected_dmg = atk.expected_damage_target(context, champ as &dyn AbstractEntity, jungle)   // 21942 (vtable @anon.54)
//              // L174             range = atk.range(champ) + atk.range_adjust(champ, jungle) + champ.radius() + jungle.radius()
//              //                      // Effect::range 인라인 = range(+0x4a0) + growth(+0x4a8)*(level(+0x5c8)−1) + stat_buff_cached.range(+0x438) (21947~21957) · range_adjust 21952 · radius 인라인 21958~22002
//              // L175             dist_sq = jungle.distance_sq(champ)             // 22005~22030 (champ.x/y, 부시 아님)
//              // L176             max_dist = range + move_speed*20                // 22031~22038
//              // L179             if jungle.hp(+0x670) <= expected_dmg && dist_sq <= max_dist² {   // 22039~22045 (select = 양쪽 다 계산, 소스 순서 표기 불가)
//              // L180                 steal_res.push(Attack(SmallActionAttack::new(data, jungle.id)))   // 22052 · 태그 15 22057 · push 22079~22105
//              //                  }
//              //              }
//              //          }
//              // L186     if champ.can_skill() {                                 // 21929
//              // L187         if let Some(skill) = &champ.skill_effect {        // +0x4f8 != -1 (22122~22124)
//              // L188             if skill.target.check(champ, jungle) {         // CastingTarget::check(+0x4f0, champ, jungle) 22130
//              // L189                 expected_dmg = skill.expected_damage_target(context, champ, jungle)   // 22138
//              // L190                 range = skill.range(champ) + skill.range_adjust(champ, jungle) + champ.radius() + jungle.radius()   // 22143~22198
//              // L191                 dist_sq = jungle.distance_sq(champ)         // 22201~22226
//              // L192                 max_dist = range + move_speed*20            // 22227~22234
//              // L194                 if jungle.hp <= expected_dmg && dist_sq <= max_dist² {   // 22235~22241
//              // L195                     steal_res.push(Skill(SmallActionSkill::new(data, jungle.id)))   // 22248 · 태그 16 22253 · push 22275~22300
//              //                      }
//              //                  }
//              //              }
//              //          }
//              // L202     if champ.can_skill2() {                                // 22117
//              // L203         if let Some(skill2) = champ.skill2_effect() {     // 인라인(entity.rs:1692): level>2 ? &skill2_effect(+0x500) : &NONE · tag +0x530 != -1 (22315~22322)
//              // L204             if skill2.target.check(champ, jungle) {        // +0x528 · 22329
//              // L205                 expected_dmg = skill2.expected_damage_target(context, champ, jungle)   // 22337
//              // L206                 range = skill2.range(champ) + range_adjust + champ.radius() + jungle.radius()   // 22342~22398
//              // L207                 dist_sq = jungle.distance_sq(champ)         // ~22426
//              // L208                 max_dist = range + move_speed*20            // 22427~22434
//              // L210                 if jungle.hp <= expected_dmg && dist_sq <= max_dist² {   // 22435~22441
//              // L211                     steal_res.push(Skill2(SmallActionSkill2::new(data, jungle.id)))   // 22448 · 태그 17 22453 · push 22475~22499
//              //                      }
//              //                  }
//              //              }
//              //          }
//              //      }   // loop 22312 → 21857
//              // L219 (steal_res 는 언와인드 시 drop_glue 21610 · 정상 경로엔 extend 로 소유권 이전)
//              // ===== steal_jungle_action 끝 =====
//          }
//      } else { → 배치 M (hide.rs:107~) }
// 이후 반환(L136~139: res → sret memcpy · 언와인드 cleanup %57 은 L139 drop_glue) = 배치 M

// 사장 코드: reach.py(version=2 · gamemode=0) 결과 블록 351 전부 live · NA 봉인 대상 0

// hide.rs:103~139 (배치 M)
// 승계(배치 L 정의, IR 로 재확인): team=%54=player+0x930 · champ=%68=data.cache.player_champion[team][pos].unwrap() · is_visible_to_enemy=%79=data.cache.game.vtable[0xf8](AbstractGame::is_visible)(1-team, champ.id) (hide.rs:23) · %74 = 1-team(적 팀) · res=%46 빈 bumpalo Vec(ptr=8,bump=data.context.bump,cap=0,len=0)

// 진입: 배치 L 의 line 57 분기(%123 check_move==0 → %211 / %128·%131 → 직접) 와 line 58~101 블록 끝(%1349·%1359·%1404·%1419 → %1424) · line 101 끝(%1427 → %1428 → line 136)

// hide.rs:107  if is_visible_to_enemy && !self.check_move {        // 두 조건 모두 필요. %211: br %79 → %244/%243 · %1424: check_move 재로드(IR 23986) → 1 이면 %243(136), 0 이면 %244(108)
//   근거: CFG 에서 %79=0 으로 접으면 %1424·%244 도달 불가(cfg79.py BFS: 156블록 중 없음) ⟹ %1424 경로는 %79=1 이 정적으로 확정된 경로(jump-threading). 두 조건의 소스 표기 순서는 표기 불가(column 없음).

// hide.rs:108  data.iter_champions(1-team)                          // %245 = &player_champion[1-team] (5 x Option<&Entity>)
// hide.rs:109    .filter(|x| x.is_visible_from(champ))              // closure#7 인라인: champ.team==Neutral(tag 1) → true / Player(t): t<2 bounds(아니면 panic_bounds_check 2) → x.visible_state[t].tag==0(Visible)
// hide.rs:110    .min_by_key(|x| x.distance_sq(champ))              // closure#8: |x.x-champ.x|²+|x.y-champ.y|² (utils.rs:2158). 슬롯 0~4 를 순서대로 훑어 필터 통과 첫 원소를 %328 에서 잡고(first=(dist²,ptr)), 잔여는 aux fold(m12.ll:12641)로: 같은 필터·같은 키·min_by(cmp(acc,new)>Greater 일 때만 교체 = 동률이면 앞 원소 유지). 5슬롯 전부 실패 → %327: nearest_enemy=None
//   let nearest_enemy: Option<&Entity>

// hide.rs:112  if let Some(enemy) = nearest_enemy {                 // %1429: null 검사 → None 이면 %1587(line 132)

// hide.rs:114/115  let nearby_allies = data.iter_champions(team).filter(|x| x.id != champ.id && x.distance_sq(champ) < 22500000001).count();
//     // %66 = player_champion[team] 5슬롯 완전 언롤(%1432~%1580). null skip · id==champ.id 는 0 · 아니면 zext(dist² ult 150000²+1) 누적. ⚠거리 기준점은 enemy 가 아니라 **champ 자신**(%1434/%1435 = champ.x/y 재로드 IR 24043~24044)

// hide.rs:117/118  let nearby_enemies = data.iter_champions(1-team).filter(|x| x.is_visible_from(champ) && x.distance_sq(champ) < 22500000001).count();
//     // %245 5슬롯 완전 언롤. champ.team 으로 루프 언스위치(IR 24410~24415 freeze): Neutral → %1588 경로(가시 검사 생략) / Player(t) → %1705: t<2 이면 %1710 경로(visible_state[t].tag==0 && dist²) / t>=2 이면 %1709 경로(첫 non-null 슬롯에서 panic_bounds_check(t,2))

// hide.rs:120  let dominated = nearby_allies < nearby_enemies;       // icmp ult %1581,%1877 (IR 25162, samesign)
// hide.rs:121  let can_fight = !dominated && can1v1win(rnd, player, data, champ, enemy);   // dominated 면 can1v1win 호출 자체를 안 함(IR 25164 → %1881). DI: can_fight=%1880 는 %1882(호출 뒤)에서만

// hide.rs:123  if can_fight {
// hide.rs:124      res.extend(battle_action(version, rnd, player, data, 5));      // sret %26 → extend(res, vec.ptr, vec.len) → %1909 → line 136
//              } else {
// hide.rs:126      res.push(SmallActionPlay::RunAway(SmallActionRunAway::new_with_skill(data, player, 5, false)));   // sret %24 136B → %25 memcpy · tag 3 @+0xb1 · len==cap 이면 reserve_internal_or_panic(res,len,1,true) · ptr[len]=184B memcpy · len+=1
// hide.rs:127      if !dominated {                                                // IR 25236: br %1878 → %243(136) / %1899(128). 즉 「1v1 승산 없음이지만 수적 열세는 아님」일 때만
// hide.rs:128          res.extend(battle_action(version, rnd, player, data, 5));  // sret %23
//                  }
//              }
//   → line 136
// } else {   // nearest_enemy == None (%1587)
// hide.rs:132      res.push(RunAway(new_with_skill(data, player, 5, false)));    // sret %21 → %22 · tag 3 · push 동일 수순(IR 25266~25314)
// hide.rs:133      res.extend(battle_action(version, rnd, player, data, 5));      // sret %20 (IR 25317~25325)
// }
// }  // end if 107

// hide.rs:136  res.extend(attack_summon_action(player, data));       // %243: sret %19 → extend(IR 25335). 107 이 거짓인 경로(%211·%1424·%1428) 도 전부 여기로 합류
// hide.rs:138  res                                                  // memcpy 32B → sret %0 (IR 25340)
// hide.rs:139  }                                                    // ret (IR 25342). 언와인드 %57: drop_glue(bumpalo Vec<SmallActionPlay>)(res) 뒤 caller 로 (IR 20402)

// 배치 M 이 push 하는 variant 집합 = {RunAway(태그 3)} 2사이트(126·132), 생성자 인자 (data, player, end_delay=5, with_skill=false) 동일. 나머지 원소는 battle_action ×3 · attack_summon_action ×1 의 반환 Vec 을 extend.
// 배치 M 범위 내 self(%1) 쓰기: 없음. TLS(LocalKey::with) 접점: 없음(함수 전체 0건, 콜리 can1v1win/battle_action/attack_summon_action 내부는 그 명세 소관).
// 사장 코드: reach.txt 사장 호출부 0 — NA 봉인 대상 없음.
```

**`mem` 메모리 접근 59건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | HideSubPlan(self) | 0x0 | bush | r | L35(m02.ll:20504)·L58(20838)·L147(21684) BUSH_POSITIONS 검색 키 `map.bushes[y][x] == self.bush` · L94(21531)/L96(21527) AroundBush 생성자 bush 인자 | 4 | OK |  |
| 1 | HideSubPlan(self) | 0x8 | out_line(AroundBushOutlineType) | r | L94(21542)/L96(21551) AroundBush 생성자 out_line 인자로 그대로 전달(i8 0 None/1 Outline/2 Inline) | 4 | OK |  |
| 2 | HideSubPlan(self) | 0x9 | check_move | r | L28(20455) · L34(20467) · L57(20578/20583/20591) 분기 | 4 | OK |  |
| 3 | PlayerState | 0x930 | info.team | r | L22(20390) team; 배열 인덱스 <2 검사(20393) · enemy_team = 1-team(20433) · camp_pos blue_side = team==0(20622) | 4 | OK |  |
| 4 | PlayerState | 0x9c0 | info.position@tag(i32→usize) | r | L22(20411~20412) player_champion 두번째 인덱스 | 4 | OK |  |
| 5 | OperationData | 0x0 | cache(&AbstractGameWithCache) | r | L22(20413) | 4 | OK |  |
| 6 | OperationData | 0x8 | context(&GameContext) | r | L20(20378~20379) · L39(20615) setting · L35/L58/L147(20477/20811/21660) map | 4 | OK |  |
| 7 | OperationData | 0x10 | blackboard(&[Blackboard;2]) | r | L61(21314~21315) → [enemy_team](21387 gepS) is_recent_visible self 인자; aux 안에선 [1-player.info.team] 로 재계산+bounds check(m12.ll:7808~7815) | 4 | OK |  |
| 8 | GameContext | 0x0 | pool(&Bump) | r | L20(20381) res=Vec::new_in(bump) · L143(21632) steal res | 4 | OK |  |
| 9 | GameContext | 0x8 | setting(&GameSetting) | r | L39(20615~20616) is_top_side 인라인 | 4 | OK |  |
| 10 | GameContext | 0x20 | map(&MapDef) | r | L35/L58/L147 bushes 조회 · L40/L48 camp_pos self | 4 | OK |  |
| 11 | GameSetting | 0x12c0 | height | r | L39(20617~20618) is_top_side: ry = height − by | 4 | OK |  |
| 12 | MapDef | 0x1c98 | bushes[30][30](usize) | r | L35(20503,20569~20571)/L58(20837,20903~20905)/L147(21683,21749~21751) bushes[y][x] == self.bush (x,y 각각 <30 bounds check → panic_bounds_check) | 4 | OK |  |
| 13 | AbstractGameWithCache | 0x0 | game.data_ptr(&dyn AbstractGame) | r | L23(20430) is_visible self · L61 캡처(21342) · L81(23823) get_entity_by_id/tick self | 4 | OK |  |
| 14 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r | L23(20432,20440 슬롯+0xf8 is_visible) · L81(23824~23826 슬롯+0x1f0 get_entity_by_id · 23833 슬롯+0x28 tick) | 4 | OK |  |
| 15 | AbstractGameWithCache | 0x1e0 | player_champion[2][5](Option<&Entity>, 40B stride) | r | L22(20420~20423) [team][pos] → champ(unwrap, None 이면 option.rs:1013 unwrap_failed 20446) · L60(21306) [enemy_team] 5칸 슬라이스 · L70(22596) [team] · L73(22978) [enemy_team] | 4 | OK |  |
| 16 | AbstractGameWithCache | 0xd0 | jungles.buf.ptr(Vec<&Entity>) | r | L156(21821~21822) 정글 엔티티 순회 시작 | 4 | OK |  |
| 17 | AbstractGameWithCache | 0xe8 | jungles.len | r | L156(21824~21825) 순회 끝 = ptr + len*8 | 4 | OK |  |
| 18 | Entity | 0x5c0 | id | r | champ: L23(20438) is_visible 인자 · L69(22591,22612~22615 e.id != champ.id) · enemy: L81(23814) Trace target · jungle: L180/L195/L211(22050/22246/22446) Attack/Skill/Skill2 target \| (배치 M) L114 `x.id != champ.id`(자기 제외). champ.id 는 %75(배치 L gep) 를 %1433 에서 재로드(IR 24042), x.id = gep+1472 (IR 24063) | 4 | OK |  |
| 19 | Entity | 0x660 | x | r | distance_sq 인라인(entity.rs:2158): champ L42/L49/L63/L69/L72/L150/L175/L191/L207 · 적 L62 · 정글 L163 · target_entity L81(23845) \| (배치 M) distance_sq(utils.rs:2158 인라인): \|x1-x2\|²+\|y1-y2\|². champ.x=%336(IR 21260) · 각 x 의 x(IR 21252 등) | 4 | OK |  |
| 20 | Entity | 0x668 | y | r | 위와 짝 \| (배치 M) 위와 짝. champ.y=%338(IR 21264) | 4 | OK |  |
| 21 | Entity | 0x0 | team@tag(TeamType: 0 Player/1 Neutral) | r | L72(22959~22961) is_visible_from(champ) 인라인: Neutral 이면 항상 가시(1043 경로), Player 면 visible_state[team] | 4 | OK |  |
| 22 | Entity | 0x8 | team@Player.0(usize) | r | L72(22962~22964, 23271 <2 bounds → panic 23651) visible_state 인덱스 | 4 | OK |  |
| 23 | Entity | 0x38 | visible_state[2]@tag(24B stride; 0=Visible) | r | L72(23309~23312) 적 e.visible_state[champ.team]@tag == 0 이어야 nearby_enemies 에 셈 | 4 | OK |  |
| 24 | Entity | 0x68 | ty@tag(EntityType) | r | L158(21879~21881) ==4 Jungle (is_jungle 인라인 entity.rs:1378) | 4 | OK |  |
| 25 | Entity | 0x98 | ty@Jungle.info.camp_type.0(usize) | r | L158(21885~21887) == enemy_team — 적 진영 캠프인 정글만 스틸 대상 | 4 | OK |  |
| 26 | Entity | 0x640 | stat_cached.move_speed | r | L168(21916) champ; max_dist 가산 move_speed*20 (L176/192/208) | 4 | OK |  |
| 27 | Entity | 0x670 | hp | r | jungle: L179(22040)/L194(22236)/L210(22436) hp <= expected_dmg | 4 | OK |  |
| 28 | Entity | 0x5c8 | level | r | champ: Effect::range 인라인(effect.rs:26) (level−1)*growth_range L174/190/206(21949,22145,22315) · L203(22315~22317) skill2_effect(): level>2 여야 Some | 4 | OK |  |
| 29 | Entity | 0x438 | stat_buff_cached.range | r | champ: Effect::range 인라인 가산 L174/190/206(21950/22146/22346) | 4 | OK |  |
| 30 | Entity | 0x470 | stat_buff_cached.radius_mult(i32) | r | Entity::radius 인라인(entity.rs:1511~1515) champ(21958)/jungle(21981) L174·190·206 | 4 | OK |  |
| 31 | Entity | 0x680 | radius | r | Entity::radius 인라인 champ(21964/21970)·jungle(21987/21995) | 4 | OK |  |
| 32 | Entity | 0x490 | attack_effect@Some.0(Effect 56B) | r | L172~174(21939 atk=champ+0x490) expected_damage_target/range_adjust self · +0x4a0 range · +0x4a8 growth_range 읽음(21947~21948) | 4 | OK |  |
| 33 | Entity | 0x4c0 | attack_effect@tag(i32 니치; -1=None) | r | L172(21934~21936) != -1 이어야 공격 후보 | 4 | OK |  |
| 34 | Entity | 0x4c8 | skill_effect@Some.0(Effect) | r | L187~190(22127 skill=champ+0x4c8; +0x4d8 range 22143 · +0x4e0 growth 22144) | 4 | OK |  |
| 35 | Entity | 0x4f0 | skill_effect@Some.0.target(CastingTarget) | r | L188(22130) CastingTarget::check(&skill.target, champ, jungle) | 4 | OK |  |
| 36 | Entity | 0x4f8 | skill_effect@tag(i32; -1=None) | r | L187(22122~22124) | 4 | OK |  |
| 37 | Entity | 0x500 | skill2_effect@Some.0(Effect) | r | L203(22317 select: level>2 ? &champ.skill2_effect : &NONE(@anon.19, tag -1)) · +0x510 range/+0x518 growth = skill2+16/+24(22342~22345) · +0x528 target = +40(22327) · +0x530 tag = +48(22319~22321) | 4 | OK |  |
| 38 | Blackboard | 0x0 | [enemy_team] (744B stride) | r | L61(21387) is_recent_visible self = data.blackboard[1-team] | 4 | OK |  |
| 39 | HideSubPlan | 0x9 | check_move | r | hide.rs:107 두 번째 조건. %1424 (IR 23986) 에서 재로드 — 배치 L(L45·L52) 에서 1 로 바뀔 수 있어서. 값 1 이면 108~133 전체 건너뜀 | 4 | OK |  |
| 40 | AbstractGameWithCache | 0x1e0 | player_champion[2][5] (Option<&Entity>, 니치 null=None) | r | base gep(+480) 는 배치 L(L22, IR 20420). 배치 M 은 행 인덱싱만: %245=[1-team] (L108, IR 20915) · %66=[team] (L22 정의, L115 에서 사용). 5슬롯 순회 · null 슬롯 skip | 4 | OK |  |
| 41 | Entity | 0x0 | team@tag (TeamType: 0=Player,1=Neutral) | r | champ(%68) 의 팀 태그. is_visible_from 인라인(entity.rs:1136 player_team): Neutral 이면 무조건 가시. IR 20992(L110) · 24410(L118 언스위치 freeze) | 4 | OK |  |
| 42 | Entity | 0x8 | team@Player.0 (usize) | r | champ 의 팀 번호 t. t<2 bounds 검사(panic_bounds_check) 뒤 visible_state[t] 인덱스. IR 20998 · 24413 | 4 | OK |  |
| 43 | Entity | 0x38 | visible_state[t]@tag (stride 24B: {i64,[2 x i64]}) | r | 적 x 의 visible_state[t].tag == 0(Visible) 이면 가시(data.rs:122 is_visible). IR 21013~21016(L110) · 24759~24762(L118) | 4 | OK |  |
| 44 | bumpalo Vec<SmallActionPlay> (콜리 반환 32B) | 0x18 | len | r | battle_action/attack_summon_action 반환 Vec 의 +0 ptr · +0x18 len 을 읽어 extend(IR 25244~25247 · 25255~25258 · 25321~25324 · 25332~25335) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 45 | HideSubPlan(self) | 0xa | enemy_spotted_me | w | L26 is_visible_to_enemy 참이면 true; 거짓이고 self.check_move 참이면 false; 거짓이고 check_move 거짓이면 손대지 않음. 배치 M(L107~)이 이 값을 읽는지는 그쪽 명세 | 4 | OK | 1 (L27 m02.ll:20465) / 0 (L30 m02.ll:20589) |
| 46 | HideSubPlan(self) | 0x9 | check_move | w | check_move 가 거짓일 때만 도달(L34) — 챔피언이 목표 캠프 위치 반경 100000 안에 들어오면 true 로 전환. 이후 L57 분기는 같은 틱에 갱신값(%95 재로드 20578)을 본다 | 4 | OK | 1 (L45 m02.ll:20751 · L52 m02.ll:20667) |
| 47 | res(%46 지역 Vec) | 0x0 | Vec::new_in(bump) 32B 초기화 | w | L20 m02.ll:20383~20389 | 4 | 확인불가(★모호: 동명 def_path 40개 [('game_ai::path_fi) | ptr=8(dangling)·bump·cap=0·len=0 |
| 48 | res(%46) | 0x0 -> 원소[len] | push AroundPosition(태그 없음) | w | L43(m02.ll:20757 생성 · 20795~20797 push) Morgard 경로 / L50(20673 · 20711~20713) Serpen 경로. 184B 통째 memcpy, +0xb1 별도 store 없음(=untagged) | 4 | 확인불가(★모호: 동명 def_path 40개 [('game_ai::path_fi) | SmallActionAroundPosition::new_with_out_line(rnd, data, ex, ey, end_delay=5, outline_type=1 Outline) |
| 49 | res(%46) | 0x0 -> 원소[len] | push AroundBush(태그 12) | w | L94 m02.ll:21543 생성·21558 태그·21602~21604 push / L96 21552·22506·22550~22552 | 4 | 확인불가(★모호: 동명 def_path 40개 [('game_ai::path_fi) | L94: SmallActionAroundBush::new_with_target(data, nearest_enemy, self.bush, self.out_line) / L96: ::new_with_out_line(rnd, data, player, self.bush, self.out_line) |
| 50 | res(%46) | 0x0 -> 원소[len] | push Trace(태그 14) | w | L81 m02.ll:23861~23881 필드 store(+0=0 · +0x55=2 · +0x58 tick · +0x60 target · +0x68/+0x70 target_entity.x/y 또는 0,0 · +0x78 15000 · +0x80 5 · +0x88 0 · [0x90,0x95) 0 · +0x95 2 · +0xb1 14) · 23924~23926 push. can_fight 경로에서 battle_action extend 뒤에 push | 4 | 확인불가(★모호: 동명 def_path 40개 [('game_ai::path_fi) | SmallActionTrace::new(data, target=enemy.id, end_delay=5) 인라인(trace.rs:43~48) |
| 51 | res(%46) | 0x0 -> 원소[len] | push RunAway(태그 3) | w | L84 m02.ll:23723 생성·23738 태그·23782~23784 push(!can_fight) / L91 22970·23933·23977~23979 push(노출됐는데 nearest_enemy 없음) | 4 | 확인불가(★모호: 동명 def_path 40개 [('game_ai::path_fi) | SmallActionRunAway::new_with_skill(data, player, end_delay=5, with_skill=false) |
| 52 | res(%46) | 0x0 -> 원소[len..] | extend(battle_action(version, rnd, player, data, 5)) | w | L80 m02.ll:23732 호출·23808 extend(can_fight) / L87 23790·23797(!can_fight && !dominated) | 4 | 확인불가(★모호: 동명 def_path 40개 [('game_ai::path_fi) | Vec<SmallActionPlay> 통째 append(ptr,len) |
| 53 | res(%46) | 0x0 -> 원소[len..] | extend(steal_jungle_action 결과 %16) | w | L101 m02.ll:23992 Extend::extend(res, steal.ptr, steal.len). 비노출(L93) 경로 전용 | 4 | 확인불가(★모호: 동명 def_path 40개 [('game_ai::path_fi) | Attack(15)/Skill(16)/Skill2(17) 0~3×정글 수 |
| 54 | steal res(%16 지역 Vec, steal_jungle_action 인라인) | 0x0 -> 원소[len] | push Attack(15)/Skill(16)/Skill2(17) | w | L180 m02.ll:22052·22057·22103~22105 / L195 22248·22253·22298~22300 / L211 22448·22453·22497~22499. sret 24B → 184B 슬롯 memcpy 후 태그 store(+0xb1=177) | 4 | 확인불가(tcx 사전에 타입 없음) | SmallActionAttack::new(data, jungle.id) / SmallActionSkill::new(data, jungle.id) / SmallActionSkill2::new(data, jungle.id) |
| 55 | res(%46) | 0x18 | len | w | push 마다 m02.ll:20713/20797/21604/22552/23784/23926/23979 store | 4 | 확인불가(★모호: 동명 def_path 40개 [('game_ai::path_fi) | +1 per push |
| 56 | res (지역 bumpalo Vec %46 → sret %0) | 0x18 | len | w | RunAway push 2곳: L126(IR 25232~25234) · L132(IR 25312~25314). len==cap 이면 reserve_internal_or_panic(res, len, 1, true) 먼저(IR 25211 · 25291). 원소 184B memcpy → ptr[len]. extend 4곳(L124·128·133·136) 은 콜리 내부에서 len 갱신 | 4 | 확인불가(★모호: 동명 def_path 40개 [('game_ai::path_fi) | +1 per push |
| 57 | SmallActionPlay (push 원소 alloca %25/%22) | 0xb1 | tag | w | new_with_skill sret 136B 를 memcpy 후 `store i8 3` @+177 (IR 25188 · 25268) | 4 | OK | 3 (=RunAway) |
| 58 | (sret) %0 | 0x0 | Vec 32B | w | hide.rs:138 (IR 25340). 배치 M 범위에는 self(%1) 쓰기 없음 | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) | memcpy(res) |

**`consts` 상수 30건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | 248 | 23 | 미상 | AbstractGame vtable 슬롯 +0xf8 = is_visible(team, id) (divtable). is_visible_to_enemy = game.is_visible(enemy_team, champ.id) | 3 |
| 1 | 71 | 35 | 길이 | game_core::BUSH_POSITIONS 길이(BUSH_NONZERO_COUNT, path_finder.rs:77/100) — 71개 (x,y) 부시 셀 상수 배열이 @anon.148(1136B) 로 인라인돼 L35/L58/L147 세 번 스택 복사(memcpy 20484/20818/21664) | 4 |
| 2 | 30 | 35 | 임계 | MapDef.bushes[30][30] 그리드 폭 — x,y 각각 <30 bounds check(20547/20551 · 20881/20885 · 21727/21731). 임계 아님 | 4 |
| 3 | 32000 | 37 | 미상 | 셀→월드 변환(셀 폭). L37 bx=x*32000+16000 / L59 / L149(steal 안 closure#1) | 4 |
| 4 | 16000 | 37 | 미상 | 셀 중심 오프셋. L37 by 는 -32000*y-16000 형태로 접혀 height 뺄셈에 흡수(20611~20612) | 4 |
| 5 | 4 | 40 | 태그 | JungleType::Morgard 메모리태그 4 (tcxdict --enum JungleType) — is_top_side 참일 때 camp_pos(map, Morgard, team==0) (20630). 또 L158 EntityType::Jungle 태그 4 (21881) | 3 |
| 6 | 5 | 48 | 태그 | JungleType::Serpen 태그 5 — is_top_side 거짓일 때 camp_pos(map, Serpen, team==0) (20626). 또 end_delay=5: L43/L50 AroundPosition · L80/L87 battle_action 5번째 인자 · L81 Trace · L84/L91 RunAway | 4 |
| 7 | 10000000000 | 42 | 임계 | 100000² — champ↔캠프 위치 거리²가 이보다 크면(>) AroundPosition 으로 이동, 아니면 check_move=true (L42 20747 / L49 20663) | 4 |
| 8 | 1 | 43 | 계수 | AroundBushOutlineType::Outline(1) — AroundPosition::new_with_out_line 6번째 인자 i8 1 (20673/20757). 또 enemy_spotted_me/check_move 에 store 하는 true | 4 |
| 9 | 62500000001 | 62 | 임계 | 250000²+1 — 부시 월드 좌표↔적 챔피언 거리² < 이 값(= ≤250000, 약 7.8셀) 이어야 nearest_enemy 후보 (21457 · aux m12.ll:7866) | 4 |
| 10 | 22500000001 | 69 | 임계 | 150000²+1 — L69 아군 수(자기 제외)·L72 적 수(is_visible_from 조건) 세는 반경 dist² < 이 값(≤150000, 약 4.7셀) (22645 등 unrolled 10곳) | 4 |
| 11 | 22500000000 | 163 | 임계 | steal_range²(150000² · L143 `steal_range = 150000` dbg_value) — 정글↔부시 위치 거리² > 이 값이면 스틸 대상 제외 (21912) | 4 |
| 12 | 150000 | 143 | 미상 | steal_range (dbg_value 로만 남음 · 산술은 22500000000 으로 접힘). 부시 위치 기준 스틸 탐색 반경 | 4 |
| 13 | 12 | 94 | 태그 | SmallActionPlay::AroundBush 메모리태그 12 (21558 / L96 22506) | 4 |
| 14 | 14 | 81 | 태그 | SmallActionPlay::Trace 메모리태그 14 (23881) | 4 |
| 15 | 15000 | 81 | 산출값 | SmallActionTrace.attack_range_margin 초기값(trace.rs:new 인라인, 23873) | 4 |
| 16 | 2 | 81 | 센티널 | Option<PathFinder>::None 니치 태그(+0x55=23863) · Option<bool> last_escape None(+0x95=23879) — Trace::new 인라인. 또 L203 `level > 2` (22316) skill2_effect() 게이트 · L72 team<2 bounds | 4 |
| 17 | 3 | 84 | 태그 | SmallActionPlay::RunAway 메모리태그 3 (23738 / L91 23933) | 4 |
| 18 | 496 | 81 | 미상 | AbstractGame vtable 슬롯 +0x1f0 = get_entity_by_id(id)→Option<&Entity> (23825) — Trace goal_x/goal_y 용 | 4 |
| 19 | 40 | 81 | 태그 | AbstractGame vtable 슬롯 +0x28 = tick() (23833) — Trace.start_tick. (또 player_champion 행 stride 40B, 정글 iter 등은 stride) | 4 |
| 20 | 15 | 180 | 태그 | SmallActionPlay::Attack 메모리태그 15 (22057) | 4 |
| 21 | 16 | 195 | 태그 | SmallActionPlay::Skill 메모리태그 16 (22253) | 4 |
| 22 | 17 | 211 | 태그 | SmallActionPlay::Skill2 메모리태그 17 (22453) | 4 |
| 23 | -1 | 172 | 센티널 | Option<Effect> None 니치(i32 casting@tag -1): attack_effect +0x4c0(21935) · skill_effect +0x4f8(22123) · skill2 +0x530(22321 · @anon.19 NONE 상수도 -1) | 4 |
| 24 | 20 | 176 | 계수 | max_dist = 사거리합 + move_speed*20 — 20틱 이동 여유(22031 / L192 22227 / L208 22427) | 4 |
| 25 | 100 | 174 | 계수 | Entity::radius 인라인(entity.rs:1515): radius*(100+radius_mult)/100 (mult==0 이면 radius 그대로 1513) — champ·jungle 양쪽 L174/190/206 | 4 |
| 26 | 0 | 30 | 태그 | self.enemy_spotted_me=false store(20589) · L22/L39 team==0(20622 blue_side) · L72 visible_state@tag==0 Visible(23312) · L81 goal 기본 (0,0) | 4 |
| 27 | 22500000001 | 114 | 임계 | 150000²+1 — `dist² < 150000²+1` ⟺ dist² ≤ 150000² (4.6875셀). L114 아군 근접 판정(IR 24096 등 5회) 과 L117 적 근접 판정(IR 24473 등 10회) 이 같은 값. 원소는 icmp ult 로 나오니 소스는 `<= 150000*150000` 또는 `< …+1` 중 하나(표기 불가·동작 확정) | 4 |
| 28 | 5 | 126 | 산출값 | end_delay=5 — SmallActionRunAway::new_with_skill(data, player, end_delay=5, with_skill=false) 의 3번 인자(L126 IR 25173 · L132 IR 24420) 및 battle_action(version, rnd, player, data, _end_delay=5) 의 5번 인자(L124·128·133). 콜리 DI 변수명으로 확인(new_with_skill %3=end_delay · battle_action %5=_end_delay) | 4 |
| 29 | 3 | 126 | 센티널 | SmallActionPlay::RunAway 메모리 태그(니치: idx0 → 3). `store i8 3` @+0xb1 (IR 25188 · 25268) | 4 |

**`knobs` 조정점 14건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 캠프 접근 완료 반경(check_move 전환) | hide.rs:42 / 49 | 10000000000 | champ↔목표 캠프(Morgard/Serpen) 거리² 가 이보다 크면 계속 AroundPosition 으로 이동. 내리면(반경 축소) 캠프에 더 가까이 가야 check_move=true 가 돼 부시 대기 단계로 늦게 넘어감 · 올리면 더 멀리서 전환 | 4 | 기존 |
| 1 | 부시 기준 최근접 적 탐색 반경(제곱+1) | hide.rs:62 (aux m12.ll:7866) | 62500000001 | 올리면(=250000 확대) 더 먼 적도 nearest_enemy 로 잡혀 노출 시 전투/도주 판정·비노출 시 AroundBush(new_with_target) 대상이 됨 · 내리면 적 없음 경로(L91 RunAway / L96 new_with_out_line) 가 늘어남 | 4 | 기존 |
| 2 | 아군/적 수 세는 반경(제곱+1) | hide.rs:69 / 72 | 22500000001 | dominated(아군<적) 판정 반경. 올리면 먼 적·아군까지 세어 수적 판정이 광역화, 내리면 근접 전투만 반영 | 4 | 기존 |
| 3 | 정글 스틸 탐색 반경 steal_range | hide.rs:143 (산술 163 은 22500000000 으로 접힘) | 150000 | 부시 위치(없으면 champ 위치) 기준 이 거리 안의 적 진영 정글만 Attack/Skill/Skill2 후보. 올리면 더 먼 캠프도 스틸 후보 | 4 | 기존 |
| 4 | 스틸 사거리 이동 여유(move_speed 배수) | hide.rs:176 / 192 / 208 | 20 | max_dist = 사거리합 + move_speed*20. 올리면 사거리 밖 정글도 '닿는다'고 보고 후보 생성 증가 | 4 | 기존 |
| 5 | 스틸 조건 hp ≤ expected_dmg | hide.rs:179 / 194 / 210 | 0 | 한 방(공격/스킬 1회 기대 피해)에 죽는 정글만 후보 — 비교값 없음(등호). 여유를 두려면 expected_dmg 에 계수를 곱하는 형태로 바꿔야 함 | 4 | 기존 |
| 6 | 행동 end_delay | hide.rs:43 / 50 / 80 / 81 / 84 / 87 / 91 | 5 | AroundPosition·battle_action·Trace·RunAway 에 일괄 5 — 올리면 각 소행동의 종료 지연이 길어짐(의미는 각 소행동 명세) | 4 | 기존 |
| 7 | Trace 공격 사거리 여유 | hide.rs:81 (trace.rs:new 인라인) | 15000 | SmallActionTrace.attack_range_margin 초기값(소비처는 Trace::get_input 명세) — Trace::new 의 기본값이라 이 함수에서 바꾸려면 생성 후 필드 덮어쓰기 | 4 | 기존 |
| 8 | 캠프 선택 규칙 | hide.rs:39 | is_top_side → Morgard(4) / else Serpen(5) | 부시가 맵 위쪽(대각선 기준 height−y ≥ x)이면 Morgard 캠프, 아래쪽이면 Serpen 캠프로 이동 — 캠프 종류를 바꾸면 Hide 플랜의 대기 위치가 바뀜 | 4 | 기존 |
| 9 | AroundPosition outline_type | hide.rs:43 / 50 | 1 | Outline(1) 고정. 0 None/2 Inline 으로 바꾸면 AroundPosition 의 부시 외곽/내부 처리 변경(의미는 AroundPosition 명세) | 4 | 기존 |
| 10 | RunAway with_skill | hide.rs:84 / 91 | False | 도주 시 스킬 미사용(new_with_skill 4번째 i1 false). true 로 바꾸면 스킬 도주 | 4 | 기존 |
| 11 | 주변 아군/적 집계 반경 | hide.rs:114 · 117 (IR 24096 · 24473) | 22500000001 | 올리면(반경 확대) 더 먼 아군·적까지 세어 dominated 판정이 넓은 범위의 수적 비교가 된다. 아군·적 두 필터가 같은 값이라 한쪽만 바꾸면 비대칭 판정이 된다 | 4 | 기존 |
| 12 | 도주/전투 행동의 end_delay | hide.rs:124·126·128·132·133 (i64 5) | 5 | SmallActionRunAway.end_delay(+0x18) 초기값. battle_action 쪽은 콜리가 `_end_delay` 로 받아 미사용(콜리 DI 이름 기준·본문은 그 명세 소관) | 4 | 기존 |
| 13 | RunAway 의 with_skill | hide.rs:126 · 132 (i1 false) | False | true 로 바꾸면 도주 중 스킬 사용 허용(SmallActionRunAway.with_skill@+0x80 — 소비처는 RunAway::get_input 명세 소관) | 4 | 기존 |

<details><summary>`callees` 피호출자 51건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | attack_summon_action | game_ai::attack_summon_action | pub | fn(&game_core::PlayerState, &game_core::OperationData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\fight_check.rs:836 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | battle_action | game_ai::battle_action | pub | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, usize) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\fight_check.rs:620 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 2 | camp_pos | game_core::MapDef::camp_pos | pub | fn(&game_core::MapDef, game_core::JungleType, bool) -> (u64, u64) | game-core\src\simulation\map_def.rs:207 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 3 | can1v1win | game_ai::can1v1win | pub | fn(&mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::Entity, &game_core::Entity) -> bool | game-ai\src\utils.rs:398 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | can_attack | game_core::Entity::can_attack | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1527 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 5 | can_skill | game_core::Entity::can_skill | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1708 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 6 | can_skill2 | game_core::Entity::can_skill2 | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1721 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 7 | check | game_core::CastingTarget::check | pub | fn(&game_core::CastingTarget, &game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\effect\type.rs:227 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 8 | distance_sq | game_core::utils::distance_sq | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:6 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 9 | distance_sq | game_core::Entity::distance_sq | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2157 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 10 | expected_damage_target | game_core::Effect::expected_damage_target | pub | fn(&game_core::Effect, &game_core::GameContext, &dyn [Binder { value: Trait(game_core::AbstractEntity), bound_vars: [] }] + , &game_core::Entity) -> usize | game-core\src\simulation\effect.rs:91 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 11 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 12 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 13 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 14 | growth | game_core::ChampionInfo::growth | pub | fn(&Self/#0) -> game_core::EntityStat | game-core\src\setting.rs:1083 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 63개 중 상위 3개 |
| 15 | growth | <game_core::DataChampionInfo as game_core::ChampionInfo>::growth | pub | fn(&game_core::DataChampionInfo) -> game_core::EntityStat | game-core\src\setting\champion\data_driven.rs:2573 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 63개 중 상위 3개 |
| 16 | growth | <game_core::MonkChampionInfo as game_core::ChampionInfo>::growth | pub | fn(&game_core::MonkChampionInfo) -> game_core::EntityStat | game-core\src\setting\champion\monk.rs:62 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 63개 중 상위 3개 |
| 17 | is_jungle | game_core::EntityType::is_jungle | pub | fn(&game_core::EntityType, usize) -> bool | game-core\src\simulation\entity.rs:1377 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 18 | is_recent_visible | game_core::Blackboard::is_recent_visible | pub | fn(&game_core::Blackboard, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\game\blackboard.rs:346 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 19 | is_top_side | game_core::is_top_side | pub | fn(&game_core::GameContext, u64, u64) -> bool | game-core\src\simulation\map_regions.rs:21 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 20 | is_visible | game_core::World::is_visible | pub | fn(&game_core::World, usize, usize) -> bool | game-core\src\simulation\game\data.rs:384 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 21 | is_visible | game_core::VisibleState::is_visible | pub | fn(&game_core::VisibleState) -> bool | game-core\src\simulation\game\data.rs:121 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 22 | is_visible | game_core::AbstractGame::is_visible | pub | fn(&Self/#0, usize, usize) -> bool | game-core\src\simulation.rs:125 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 23 | is_visible_from | game_core::Entity::is_visible_from | pub | fn(&game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1481 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 24 | iter_champions | game_core::AbstractGameWithCache::<'a, 'b>::iter_champions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5759 ~ game_core[6a30]::simulation::{impl#5}::iter_champions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1904 | True | True | 3 | IR 호출 심볼 일치(토큰 경계) |
| 25 | level | <game_ai::WindowStatView<'_> as game_core::AbstractEntity>::level | pub | fn(&game_ai::WindowStatView</#0>) -> usize | game-ai\src\fight_check.rs:96 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 26 | level | game_core::AbstractEntity::level | pub | fn(&Self/#0) -> usize | game-core\src\simulation\entity.rs:718 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 27 | level | <game_core::Entity as game_core::AbstractEntity>::level | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:772 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 28 | new | game_ai::SmallActionSkill::new | pub | fn(&game_core::OperationData, usize) -> game_ai::SmallActionSkill | game-ai\src\small_action\cast.rs:119 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 29 | new | game_ai::SmallActionAttack::new | pub | fn(&game_core::OperationData, usize) -> game_ai::SmallActionAttack | game-ai\src\small_action\cast.rs:23 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 30 | new_with_out_line | game_ai::SmallActionAroundBush::new_with_out_line | pub | fn(&mut rand::rngs::std::StdRng, &game_core::OperationData, &game_core::PlayerState, usize, game_ai::AroundBushOutlineType) -> game_ai::SmallActionAroundBush | game-ai\src\small_action\around.rs:1154 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 31 | new_with_out_line | game_ai::SmallActionAroundPosition::new_with_out_line | pub | fn(&mut rand::rngs::std::StdRng, &game_core::OperationData, u64, u64, usize, game_ai::AroundBushOutlineType) -> game_ai::SmallActionAroundPosition | game-ai\src\small_action\around.rs:834 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 32 | new_with_skill | game_ai::SmallActionRunAway::new_with_skill | pub | fn(&game_core::OperationData, &game_core::PlayerState, usize, bool) -> game_ai::SmallActionRunAway | game-ai\src\small_action\move_actions.rs:29 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 33 | new_with_target | game_ai::SmallActionAroundBush::new_with_target | pub | fn(&game_core::OperationData, &game_core::Entity, usize, game_ai::AroundBushOutlineType) -> game_ai::SmallActionAroundBush | game-ai\src\small_action\around.rs:1178 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 34 | next | game_view::StatMode::next | in:game_view::ui::match_result_ui | fn(game_view::StatMode) -> game_view::StatMode | game-view\src\ui\match_result_ui.rs:62 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 9개 중 상위 3개 |
| 35 | next | game_view::FlowPeriod::next | in:game_view::ui::finance_ui | fn(&game_view::FlowPeriod) -> game_view::FlowPeriod | game-view\src\ui\finance_ui.rs:180 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 9개 중 상위 3개 |
| 36 | next | game_view::ResultsPeriod::next | pub | fn(game_view::ResultsPeriod) -> game_view::ResultsPeriod | game-view\src\ui\training_ui.rs:44 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 9개 중 상위 3개 |
| 37 | push | <game_core::Staff as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::staff | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\staff.rs:38 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 38 | push | <game_core::Athlete as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:434 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 39 | push | <game_core::Contract as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:2498 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 40 | radius | game_core::Entity::radius | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1509 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 41 | range | game_core::Effect::range | pub | fn(&game_core::Effect, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:25 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 42 | range_adjust | game_core::Effect::range_adjust | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:29 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 43 | skill2_effect | game_core::Entity::skill2_effect | pub | fn(&game_core::Entity) -> &std::option::Option<game_core::Effect> | game-core\src\simulation\entity.rs:1692 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 44 | steal_jungle_action | game_ai::plan_legacy::sub_plan::HideSubPlan::steal_jungle_action | in:game_ai::plan_legacy::sub_plan::hide | fn(&game_ai::plan_legacy::sub_plan::HideSubPlan, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> | game-ai\src\plan_legacy\sub_plan\hide.rs:142 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 45 | team | game_view::ClientData::team | pub | fn(&game_view::ClientData, usize) -> std::option::Option<std::cell::Ref< game_core::Team>> | game-view\src\logic\client\data.rs:1651 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 46 | team | game_view::ClientDatabase::team | pub | fn(&game_view::ClientDatabase, usize) -> std::option::Option<&game_core::Team> | game-view\src\logic\client\data.rs:968 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 47 | team | game_core::PlayerAiContext::<'a, 'b, 'r>::team | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:583 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 48 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 49 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 50 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
</details>

⚠**미매칭 9개**: `bounds`, `enemy_team`, `extend`, `height`, `is_top`, `jungles`, `min_by`, `move_speed`, `reserve_internal_or_panic`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m12.ll:35225) · **형제 9개** (HideSubPlan)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::plan_legacy::sub_plan::HideSubPlan as std::clone::Clone>::clone | pub | game-ai\src\plan_legacy\sub_plan\hide.rs:6 | True | fn(&game_ai::plan_legacy::sub_plan::HideSubPlan) -> game_ai::plan_legacy::sub_plan::HideSubPlan |
| 1 | <game_ai::plan_legacy::sub_plan::HideSubPlan as std::default::Default>::default | pub | game-ai\src\plan_legacy\sub_plan\hide.rs:6 | True | fn() -> game_ai::plan_legacy::sub_plan::HideSubPlan |
| 2 | <game_ai::plan_legacy::sub_plan::HideSubPlan as std::fmt::Debug>::fmt | pub | game-ai\src\plan_legacy\sub_plan\hide.rs:6 | True | fn(&game_ai::plan_legacy::sub_plan::HideSubPlan, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 3 | game_ai::plan_legacy::sub_plan::HideSubPlan::new | pub | game-ai\src\plan_legacy\sub_plan\hide.rs:15 | True | fn(usize, game_ai::AroundBushOutlineType) -> game_ai::plan_legacy::sub_plan::HideSubPlan |
| 4 | game_ai::plan_legacy::sub_plan::HideSubPlan::action_candidates | pub | game-ai\src\plan_legacy\sub_plan\hide.rs:19 | False | fn(&mut game_ai::plan_legacy::sub_plan::HideSubPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &mut game_core::DebugFrameData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 5 | game_ai::plan_legacy::sub_plan::HideSubPlan::steal_jungle_action | in:game_ai::plan_legacy::sub_plan::hide | game-ai\src\plan_legacy\sub_plan\hide.rs:142 | False | fn(&game_ai::plan_legacy::sub_plan::HideSubPlan, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bumpalo::collections::vec::Vec< game_ai::SmallActionPlay> |
| 6 | game_ai::plan_legacy::sub_plan::HideSubPlan::calculate_score_parameter_value | pub | game-ai\src\plan_legacy\sub_plan\hide.rs:221 | True | fn(&game_ai::plan_legacy::sub_plan::HideSubPlan, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::ScoreParameter) |
| 7 | game_ai::plan_legacy::sub_plan::HideSubPlan::score | pub | game-ai\src\plan_legacy\sub_plan\hide.rs:238 | False | fn(&game_ai::plan_legacy::sub_plan::HideSubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 |
| 8 | game_ai::plan_legacy::sub_plan::HideSubPlan::merge | pub | game-ai\src\plan_legacy\sub_plan\hide.rs:242 | True | fn(&mut game_ai::plan_legacy::sub_plan::HideSubPlan, game_ai::plan_legacy::sub_plan::HideSubPlan) |

**`open` 16건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 표기 불가 | (배치 L) L26~30 의 소스 표기(`else if self.check_move` 한 문장인지 `else { if … }` 인지) — 블록 구조는 동치라 동작엔 영향 없음(표기 불가: column 정보 없음) | 4 |  |
| 1 | 표기 불가 | (배치 L) L179/L194/L210 의 `hp <= dmg && dist <= max²` 두 조건의 소스 순서 — IR 이 select(양쪽 계산) 로 접혀 표기 불가. 부작용 없는 순수 비교라 동작 무관 | 4 |  |
| 2 | 표기 불가 | (배치 L) L76 `can_fight` 가 `!dominated && can1v1win(...)` 한 줄인지, `if dominated {false} else {can1v1win}` 인지 — 같은 줄(L76) 에서 분기·호출 모두 나와 한 줄 표현으로 봄(표기 불가) | 4 |  |
| 3 | 미탐색 | (배치 L) can1v1win(rnd, player, data, champ, enemy) 의 내부 판정·TLS 사용 — r14 계약만 인용(m04.ll:54936 미독해). rnd/player 는 DI 이름 `_rnd`/`_player`(미사용) | 4 |  |
| 4 | 미탐색 | (배치 L) battle_action(version, rnd, player, data, 5) 의 5번째 usize 인자 이름·의미 — m15.ll:23700 define 의 DI 에 arg 5 이름이 안 남음(추정: end_delay, 같은 줄의 다른 생성자들이 5 를 end_delay 로 받음). 내부 미독해 | 4 |  |
| 5 | 미탐색 | (배치 L) expected_damage_target(effect, context, caster as &dyn AbstractEntity, target) 반환 usize 의 정확한 정의(치명타·방어 반영 여부) — g06.ll:52355 미독해 | 4 |  |
| 6 | 미탐색 | (배치 L) range_adjust(effect, caster, target) 반환 의미(투사체/대상 크기 보정 추정) — game_core 본문 미독해, 시그니처만 | 5 |  |
| 7 | 미탐색 | (배치 L) CastingTarget::check(&target, caster, target_entity) 의 판정 내용 — g?? 미독해, 시그니처만 | 4 |  |
| 8 | 표기 불가 | (배치 L) steal_jungle_action 의 L149 closure#1·L150 unwrap_or 는 IR 상 phi(%505/%506) 로만 남아 `.map().unwrap_or()` 체인 형태는 DI 줄번호(149/150)+iter 체인(option.rs:1043/1041)으로 복원 — 소스 원문 표기 불가 | 4 |  |
| 9 | 미탐색 | (배치 L) L107 이후(check_move 거짓 · enemy_spotted_me 분기) 와 sret 채움(L136/139) 은 배치 M 범위 — 이 배치는 진입 엣지(%211/%243/%244/%1424/%1428)만 기록 | 4 |  |
| 10 | 미탐색 | (배치 L) reads 의 Entity+0x98 은 tcxdict 가 `ty@Bear.info.target_pos@tag` 등 동일 오프셋 variant 여러 개를 주지만, L158 은 ty@tag==4(Jungle) 확정 문맥이라 Jungle.info(+0x70).camp_type(+0x28).0 = +0x98 로 읽음(MIR EntityType::is_jungle 과 일치) | 3 |  |
| 11 | 표기 불가 | (배치 M) hide.rs:127 조건이 소스에서 `!dominated` 인지 `nearby_allies >= nearby_enemies` 인지 — 같은 i1 %1878 을 재사용해 외연 동일(표기 불가) | 4 |  |
| 12 | 표기 불가 | (배치 M) hide.rs:114·117 의 `< 22500000001` 이 소스에서 `<= 150000*150000` 인지 `< 22500000001` 인지 — icmp ult 정규화로 외연 동일(표기 불가) | 4 |  |
| 13 | 미탐색 | (배치 M) hide.rs:110 min_by_key 의 비교 콜리 `@gc::…Entity…call_once`(aux m12.ll:12822) 내부는 u64 Ord::cmp 로 추정(min_by 관례) — 정확한 define 은 안 읽음(미탐색) | 4 |  |
| 14 | 미탐색 | (배치 M) SmallActionRunAway 원소의 live 바이트 정밀 표(+0x84..+0x87 패딩 · path_finder Option 니치 @+0x7d 등)는 new_with_skill 의 initializes 속성 전체를 안 옮김(잘림) — 그 함수 명세 소관 | 4 |  |
| 15 | 미탐색 | (배치 M) hide.rs:104~106 · 111 · 113 · 116 · 119 · 122 · 125 · 129~131 · 134~135 · 137 은 IR 에 루트 줄이 없음(빈 줄/주석/닫는 괄호로 추정, rmeta 줄길이 산술 미실시) | 3 |  |

**`notes` 3건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | (배치 L) Blackboard::is_recent_visible(self, game, player, target) — 왜 blackboard[enemy_team] 을 쓰는지(적 진영 관측판인지 '적에 대한 지식판' 인지)는 Blackboard 명세 몫. IR 인덱스는 1-team 으로 확정 | 4 | 사실 서술 |
| 1 | (배치 L) AroundPosition 은 untagged variant 라 +0xb1 이 outline_type(0..2) — new_with_out_line 이 그 바이트를 반드시 0..2 로 쓰는지는 그 생성자 명세(m08.ll:103172) 몫. 여기선 태그 store 가 없다는 사실만 확정 | 4 | 사실 서술 |
| 2 | (배치 M) hide.rs:107 조건식의 두 항(is_visible_to_enemy, !self.check_move) 소스 표기 순서 — column 정보 부재. 동작(둘 다 참이어야 108 진입)은 CFG 접기로 확정 | 4 | 사실 서술 |

<details><summary>`closed` 1건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

| # | 물음 | 닫은 근거 |
|---|---|---|
| 0 | (배치 M) hide.rs:103 의 IR 4줄은 전부 `llvm.lifetime.end(bx %38 / by %39)`(원문 23984~23985 · 23996~23997, 주석본에선 잡음 제거됨) — 배치 L 의 57~103 블록을 닫는 `}` 로 확정(bx/by 는 L 범위 지역변수). 판정 로직 없음 | 본문에 해소 표기가 있다 |
</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

