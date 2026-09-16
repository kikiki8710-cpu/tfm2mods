---

### `252` v16_knight_ult_zone_bonus — BattleSubPlan/DeathBattleSubPlan::score 가 후보 액션마다 부르는 '나이트 궁(KnightUltAction) 존 보너스' i64(0..75). action 이 KnightUltAction 이고 target 이 아군 챔피언일 때만: 존(range)·영향권(range+35000) 안의 아군 수·위협받는 아군 수·압박 점수(예상 피해×HP가치/HP)·영향권 내 가시 적 수(배치 I, L185~239) 를 세고, 배치 J(L243~268) 가 존 내 적 수 등을 더 세어 가중합(허용 압박/3≤18 · 위협 아군×14 · 가시 적×3 · 적>아군 페널티 −12 …)을 0..75 로 clamp 해 돌려준다. 그 외(다른 액션·적/비챔피언 target)는 0.

| 항목 | 값 |
|---|---|
| id | `battle_common__v16_knight_ult_zone_bonus` |
| 심볼 | `_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy8sub_plan13battle_common25v16_knight_ult_zone_bonus` |
| 소스 | `game-ai\src\plan_legacy\sub_plan\battle_common.rs:177` |
| IR | `m05.ll` 55831~57754행 |
| 경로·가시성 | `game_ai::plan_legacy::sub_plan::battle_common::v16_knight_ult_zone_bonus` · **in:game_ai** |
| 계층 | 기타 |
| exe | `d68090` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r18` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_core::PlayerState, &game_core::OperationData, &game_ai::ScoreParameter, &std::boxed::Box<dyn [Binder { value: Trait(game_core::Action), bound_vars: [] }] + 'static, std::alloc::Global>, &game_core::Entity, &game_core::Entity) -> i64
```

**TLS 메모 절(`sig.tls` · 정정 경로 `/specs[252]/sig/tls/<키>`)**

- `name`: 없음(직접) — 본문 55831~57754 에 LocalKey/call_once/llvm.threadlocal.address/__getit 참조 0건(grep) · 간접 접점 1: 콜리 utils::champion_hp_value(m04.ll:49583) 가 `LocalKey<RefCell<HashMap<usize,i64,ahash::RandomState>>>::with`(m00.ll:91035) 메모를 읽고 쓴다
- `role`: 소비자(간접 · 콜리 경유) — 이 함수는 키를 만들지도 무효화하지도 않음
- `key`: champion_hp_value 가 조립(m04.ll:49599~49618): game.seed()(vtable+0x20 · divtable) · game.tick()(vtable+0x28) · parameter.player.id(+0x970) · &p · &parameter 를 40B 클로저 환경으로 넘김 — 실제 해시 키 구성은 자식 명세(champion_hp_value_uncached r13 잎 / 캐시 래퍼) 소관
- `layout`: HashMap<usize(j), i64(x), ahash::RandomState> in RefCell in thread_local — 망글 `RefCellTyjjINt…HashMapjx…RandomState` 로 확인
- `invalidation`: 이 함수 범위 밖(미독 · 자식 명세 소관)
- `call_conditions`: L212(자기 자신) 또는 L219(near_allies 에서 찾은 아군) — 존 내 아군 1명당 최대 1회, near_allies 에 없는 아군이면 호출 안 함(L222 (0,50)). 호출 순서: possible_risk → champion_hp_value (블록 %573 57336→57338) · 존 내 아군 순서 = player_champion[team][0..5] 슬롯 순

<details><summary>인자 6개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | player | &PlayerState (2528B) | IR 속성 noalias readonly captures(address, read_provenance) dereferenceable(2528) · info.team(+0x930) 읽기 · is_recent_visible(…, player, …) 인자 · champion_hp_value 에는 안 넘김(그쪽은 data/parameter) · 클로저$1/$2 캡처 · (배치 J) (배치 J) 내 범위에선 is_recent_visible 의 player 인자로만 전달(L243 ×5). team 로드(+0x930)는 배치 I(L201). | 4 |
| 1 | 2 | data | &OperationData (24B: +0 cache &AbstractGameWithCache · +8 context · +0x10 blackboard &[Blackboard;2]) | IR 속성 noalias readonly captures(address, read_provenance) dereferenceable(24) · cache.game(dyn 팻포인터)·cache.player_champion · blackboard[1-team] · possible_risk/champion_hp_value 에 전달 · context(+8)는 배치 I 범위에서 읽지 않음 · (배치 J) (배치 J) data.cache.player_champion[team](L251 자기팀 5칸 · ll 56922/56979/57038/57097/57156) · [1-team](L243 적팀 5칸 · 로드 ll 56552/56610/56676/56742/56808 · gep %143/%171/%199/%227 정의는 배치 I L239 CSE) · data.blackboard[1-team](%84 · is_recent_visible self) · data.cache.game 팻포인터(%117/%118 · 로드는 배치 I L237). | 4 |
| 2 | 3 | parameter | &ScoreParameter (5384B) | IR 속성 noalias readonly captures(address, read_provenance) dereferenceable(5384) · player(+0x918 ChampionScoreParameter) 의 applyed_damage/risk_damage · near_allies(+0x14b8 ptr · +0x14d0 len) 선형 탐색 · champion_hp_value 에 통째 전달 · version(+0x14f8) 은 배치 I 범위에서 읽지 않음 · (배치 J) (배치 J) 내 범위에서 읽지 않음. | 4 |
| 3 | 4 | action | &Box<dyn Action> (16B 팻포인터: +0 data_ptr · +8 vtable_ptr) | IR 속성 noalias readonly captures(none) dereferenceable(16) · L185 vtable+0x68 as_any() → &dyn Any(data, vtable) → Any vtable+0x18 type_id() 16B sret 로 KnightUltAction 판별(downcast_ref) · 이후 knight_ult.range(+0x10)·duration(+0x30) 만 읽음 · (배치 J) (배치 J) 배치 I(L185~188) 에서 TypeId 비교로 KnightUltAction 으로 다운캐스트한 %16(DI 이름 knight_ult) 을 내 범위에서 +0x18 damage_reduce 로 읽음(L253 · ll 57214). 호출자(BattleSubPlan::score)는 champ.ult(Entity+0x5a0) 또는 empty(+0x5b0) 를 넘긴다. | 4 |
| 4 | 5 | champ | &Entity (1728B) — 자기 챔피언 | IR 속성 noalias readonly captures(none) dereferenceable(1728) · team(+0/+8)·id(+0x5c0) 만 읽음(L188 팀 비교 · L207 자기 자신 판별) · (배치 J) (배치 J) x/y(+0x660/+0x668 · L251 self_cover 의 중심 · ll 56904~56907) · id(%66 · 로드는 배치 I 호이스트 ll 55992 · L263 에서 target.id 와 비교). | 4 |
| 5 | 6 | target | &Entity (1728B) — 액션 대상(궁 시전 대상 아군) | IR 속성 noalias readonly captures(none) dereferenceable(1728) · team(+0/+8)·ty@tag(+0x68)·x/y(+0x660/+0x668) 읽기 · 존 중심으로 쓰임(L202/L238 distance_sq 의 other) · 클로저$2 캡처 · (배치 J) (배치 J) x/y(%62/%64 · 로드는 배치 I 호이스트 ll 55988/55990 · L243 enemies_in_zone 의 중심) · id(+0x5c0 · L263 · ll 57264). | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
// battle_common.rs:0~239 (배치 I)
// 호출자(정적 2곳 · 둘 다 version 분기 없이 직접 invoke · 이 함수엔 version 인자 자체가 없음): BattleSubPlan::score(battle.rs:760 · m02.ll:38216) 와 DeathBattleSubPlan::score(death_battle.rs:769 · m15.ll:12656) — 둘 다 calculate_action_score 뒤 `v21_runaway_defensive_cc_bonus → v16_gambler_ult_cc_bonus → ★v16_knight_ult_zone_bonus → v17_runaway_counterattack_bonus` 순서로 후보 액션마다 호출(인자 = player, data, parameter, &candidate.action(Box<dyn Action>), champ, target). 판 8 프로브 미설치 · 발화수 미계측.
// 콜리 계약(그 밖의 콜리 = 시그니처·반환 의미만): Blackboard::is_recent_visible(&self(744B), game:&dyn AbstractGame(ptr,vtable), player:&PlayerState, target:&Entity)->bool (g07.ll:157005 · blackboard.rs:346) = game.is_visible(player.info.team, target.id)(vtable+0xf8) || get_player_by_champion_id(target.id)(vtable+0x150) 가 Some 이고 self.last_visible[+0x1e0][p.info.position] + 120 >= game.tick()(vtable+0x28) · TLS 없음 / ChampionScoreParameter::possible_risk(&self(216B), data:&OperationData, tick_cut:usize)->i64 (m07.ll:7315 · score_parameter.rs:1407) = self.risk_possible(+0x18, PossibleGain 24B) 중 tick_cut 이내 항목의 값 합(출처 중복 제거 · 본문 미상세) / utils::champion_hp_value(data, parameter:&ScoreParameter, p:&ChampionScoreParameter)->i64 (m04.ll:49583) = TLS HashMap 메모 래퍼(자식 명세 champion_hp_value_uncached r13) / AbstractGameWithCache::iter_champions(team) = player_champion[team][0..5].iter().filter_map(closure#0 |x| *x) (simulation.rs:1905 인라인 · aux m09:66150).
// 2-space 들여쓰기 · 아래 소스 복원은 IR 정본 + rmeta 줄 길이 산술(L185 82 · L188 61 · L201 60 · L207 56 · L214 98 · L225 78 · L226 110 · L228 45 · L231 22 · L232 79 · L236 69 · L237 111 전부 ±1 정합)로 표기한 것.

[L185] let Some(knight_ult) = action.as_any()(vtable+0x68 · {ptr,ptr}).downcast_ref::<KnightUltAction>() else { [L186] return 0 }
       // = Any vtable+0x18 type_id() 16B == 0x7D0A…(u128 상수 166200278922247868797912749936351073298) · 성공 시 knight_ult = action.data_ptr(%16, llvm.assume non-null)
[L188] if target.team != champ.team [entity.rs:1127 derive PartialEq · cmp.rs:264 ne] || !target.is_champion() [entity.rs:1404 · ty@tag(+0x68) != 13] { [L189] return 0 }
       // team 비교: 태그(+0) 다르면 ne · 둘 다 0(Player) 이면 +0x8 인덱스 비교 · 둘 다 1(Neutral) 이면 같음 → is_champion 만 검사(%41). ⚠target 은 '아군 챔피언'이어야 통과(자기 자신 포함 — 아래 L207 참조)
[L192] zone_radius: u64 = knight_ult.range(+0x10)
[L193] zone_sq = zone_radius * zone_radius
[L194] influence = zone_radius + 35000
[L195] influence_sq = influence * influence
[L197~199] allies_in_zone = 0i64; threatened_allies = 0i64; pressure_score = 0i64
[L201] team = player.info.team(+0x930); if team >= 2 → panic_bounds_check(team, 2)
       for ally in data.cache.iter_champions(team) {        // player_champion[team][0..5] 슬롯 순 · None(null) 은 closure#0 이 걸러 건너뜀(%95→%92)
[L202]   if ally.distance_sq(target) > zone_sq { [L203] continue }          // |dx|²+|dy|² (x +0x660, y +0x668) · ugt %48 → %90(다음 슬롯)
[L206]   allies_in_zone += 1
[L207]   let (incoming, hp_value) = if ally.id(+0x5c0) == champ.id {
[L208~213]   ( parameter.player.applyed_damage(+0x988) as i64 + parameter.player.risk_damage(+0x998) as i64 + parameter.player.possible_risk(data, knight_ult.duration(+0x30) + 30),
               champion_hp_value(data, parameter, &parameter.player).min(100) )
[L214]   } else if let Some(ally_parameter) = parameter.near_allies(+0x14b8..len +0x14d0, stride 216).iter().find(|p| p.id(+0x58) == ally.id) {   // 선형 탐색, 첫 일치
[L215~220]   ( ally_parameter.applyed_damage(+0x70) as i64 + ally_parameter.risk_damage(+0x80) as i64 + ally_parameter.possible_risk(data, knight_ult.duration + 30),
               champion_hp_value(data, parameter, ally_parameter).min(100) )
[L221]   } else { [L222] (0, 50) };                         // near_allies 에 없으면 possible_risk/champion_hp_value 호출 없음
         // IR: 두 Some 분기가 블록 %573 하나로 합쳐짐(phi %574=p, %575=applyed+risk) → 호출 순서 possible_risk(p, data, duration+30) 다음 champion_hp_value(data, parameter, p) → smin 100 · 이 두 call 은 dbg 소실(root L0)
[L225~227] let enemy_close_to_ally = data.cache.iter_champions(1 - team)
             .any(|enemy| data.blackboard[1 - team](+0x10 → 행 1-team, ★적팀 blackboard).is_recent_visible(data.cache.game, player, enemy)
                          && enemy.distance_sq(ally) <= 100000²)      // 적 행 5칸 언롤(%80/%85/%86/%87/%88) · 단락: 첫 참에서 %707 로 · 결과 i64 0/1(phi %708)
[L228]   if incoming > 0 || enemy_close_to_ally { [L229] threatened_allies += 1 }
[L231]   if incoming > 0 { [L232] pressure_score += ((incoming * hp_value) / (ally.hp(+0x670).max(1) as i64)).min(45) }
         // IR 형태(동치): incoming > 0 → {threatened+=1; pressure+=min(sdiv,45)} / else → threatened += enemy_close_to_ally(0/1), pressure 불변. sdiv 오버플로 가드(i64::MIN / -1 → panic_const_div_overflow)는 가드일 뿐
[L234] }
[L236~239] let enemies_near = data.cache.iter_champions(1 - team)
             .filter(|enemy| data.blackboard[1 - team].is_recent_visible(data.cache.game, player, enemy))   // L237 · closure$2
             .filter(|enemy| enemy.distance_sq(target) <= influence_sq)                                          // L238 (ule %50)
             .count() as i64                                                                                      // L239 · 5칸 언롤 fold(%142→%170→%198→%226→%254) · 언롤 순서 슬롯 0..4
// 배치 I 산출(배치 J 로 넘어가는 살아있는 값): allies_in_zone(%722/%555) · threatened_allies(%721) · pressure_score(%720) · enemies_near(%254) · zone_sq(%48) · champ.id(%66) · target.x/y(%62/%64) · 적 행 %80 · 존 내 아군 순회에서 마지막 ally 포인터(%97)
// → 배치 J(줄 243): 블록 %253(56487~) 에서 enemies_in_zone(L243 · dist<=zone_sq 인 적 수) 부터. 최종 ret 은 %29(57917) — 조기 0 4곳은 배치 I, 정상값 %551 은 배치 J L267~268(umin(smax(score,0),75))

// battle_common.rs:243~268 (배치 J)
// 선행(배치 I): L185~188 action 을 KnightUltAction 으로 다운캐스트(%16=knight_ult) · target.ty==champ.ty(tag 13) 게이트 · L192 zone_radius=knight_ult.range(+0x10) · L193 zone_sq=zone_radius² (%48) · L201~232 루프 → allies_in_zone(%722, i64)·threatened_allies(%721, i64)·pressure_score(%720, i64) · L239 enemies_near(%254, usize)
// 진입: → 배치 I(줄 239) 블록 %250/%225 에서 %253(ll 56486, phi %254=enemies_near) 으로 들어옴. 이탈: L245 → 블록 %29(줄 268 ret, 배치 I 조기반환과 공유) · L268 → %29.
// TLS 접점 없음.

// L243  enemies_in_zone = data.cache.iter_champions(1 - player.team)      // player_champion[1-team] 5칸, None 스킵 (ll 56552~56869)
//           .filter(|x| /*L241*/ data.blackboard[1-team].is_recent_visible(data.cache.game, player, x)
//                    && /*L242*/ x.pos().distance_sq(target.pos()) <= zone_sq)   // (x.x,x.y)=(+0x660,+0x668) · 중심 = target(%62,%64) · icmp ule
//           .count();                                                   // usize (%386) · 5칸 완전 언롤, 각 칸: null? →0 / is_recent_visible false →0 / dist²<=zone_sq → +1
// L245  if enemies_in_zone == 0 && threatened_allies == 0 { return 0; }   // IR: (%386 | %721) == 0 → %29 (ll 56875~56877) · 두 항의 소스 순서는 표기 불가(or 로 접힘)

// L251  self_cover = data.cache.iter_champions(player.team)             // player_champion[team] 5칸(champ 자신 포함), None 스킵 (ll 56904~57210)
//           .filter(|x| /*L250*/ x.pos().distance_sq(champ.pos()) <= zone_sq)   // 중심 = champ(%391,%393 · +0x660/+0x668) · 가시성 검사 없음
//           .count();                                                   // usize (%511)

// L253  let mut score: i64 = (knight_ult.damage_reduce /*+0x18*/ as i64 / 3).min(18);          // ll 57214~57219
// L254  score += 5 + allies_in_zone.min(4) * 6;                          // ll 57223·57242 · 계수 6 은 (z+a)*6 인수분해에서 확정(아래 L257)
// L255  score += threatened_allies.min(4) * 14;                          // ll 57227~57228
// L256  score += (enemies_near.min(4) * 3) as i64;                       // umin · ll 57231~57232
// L257  score += (enemies_in_zone.min(3) as i64) * 6;                    // umin · ll 57235 · IR: %525 = (min(z,3)+min(a,4))*6 (ll 57240~57241)
// L258  score += pressure_score.min(50) / 3;                             // smin·sdiv · ll 57238~57239
//       // 합산 순서는 컴파일러 재결합(ll 57242~57246: ((p/3+5)+t*14)+n*3+(z+a)*6+dmg) — 정수 덧셈이라 결과 동일
// L260  if (self_cover as i64) > allies_in_zone {                        // icmp sgt (ll 57248)
// L261      score -= (self_cover as i64 - allies_in_zone).min(3) * 12;   // sub·smin 3·mul -12 (ll 57252~57257)
//       }
// L263  if target.id == champ.id && allies_in_zone < 2 && enemies_near < 2 { score -= 15; }
//       // IR: id 비교(+0x5c0, ll 57264~57267)가 br, 나머지 두 비교는 블록 %542 에서 and+select(-15) (ll 57270~57274) · 세 조건의 소스 순서는 표기 불가
// L267  score = score.max(0).min(75);                                    // smax 0 → umin 75 (ll 57282·57285)
// L268  return score;                                                    // → 블록 %29 phi %30 [%551,%548] · ret i64 (ll 55917)
```

**`mem` 메모리 접근 27건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | Box<dyn Action> (action) | 0x0 | data_ptr | r | L185 as_any(self=data_ptr) 인자 · downcast 성공 시 그대로 &KnightUltAction(%16 · llvm.assume non-null) | 4 | 확인불가(tcx 사전에 타입 없음) |
| 1 | Box<dyn Action> (action) | 0x8 | vtable_ptr | r | L185 vtable+0x68 = Action::as_any (divtable g02 @anon.1197 슬롯 13, 일치율 58%) → {ptr,ptr} = &dyn Any | 3 | 확인불가(tcx 사전에 타입 없음) |
| 2 | dyn Any vtable | 0x18 | type_id | r | L185 슬롯 3(drop,size,align 다음) · sret 16B(%8) → i128 비교(any.rs:229 downcast_ref → :204 is → :2450 TypeId::eq) | 4 | 확인불가(tcx 사전에 타입 없음) |
| 3 | KnightUltAction | 0x10 | range | r | L192 zone_radius(u64) · 제곱=zone_sq(L193) · +35000=influence(L194) | 4 | OK |
| 4 | KnightUltAction | 0x30 | duration | r | L211/L218 possible_risk 의 tick_cut = duration + 30 (블록 %55 로 호이스팅된 gep %72, load 는 %573 57334) | 4 | OK |
| 5 | Entity | 0x0 | team@tag | r | L188 target.team != champ.team (TeamType derive PartialEq entity.rs:1127 인라인): 태그 0=Player(+0x8 비교) · 1=Neutral(태그만) | 4 | OK |
| 6 | Entity | 0x8 | team@Player.0 | r | L188 태그 둘 다 0 일 때 팀 인덱스 비교(%34/%35) | 4 | OK |
| 7 | Entity | 0x68 | ty@tag | r | L188 target.is_champion() (entity.rs:1404 인라인) ⇔ ==13(Champion) | 4 | OK |
| 8 | Entity | 0x5c0 | id | r | champ.id(%66, 블록 %55 호이스팅) · ally.id(L207 %557) — 자기 자신 판별 및 near_allies.find(\|p\| p.id == ally.id)(L214) \| (배치 J) L263 (ll 57264~57266): target.id == champ.id(%66) → 자기대상 캐스팅 감점 게이트. | 4 | OK |
| 9 | Entity | 0x660 | x | r | distance_sq(entity.rs:2158 인라인 · \|dx\|²+\|dy\|²): target.x(%62 호이스팅) · ally.x(L202 %101 · L227 %596) · enemy.x(L227 %593 · L238 %125…) \| (배치 J) L243 closure#3(L242) 적 챔피언 x(ll 56577 등 5회) · L251 closure#4(L250) 자기팀 챔피언 x(ll 56931 등 5회) · L251 champ.x(ll 56904, 중심). | 4 | OK |
| 10 | Entity | 0x668 | y | r | 위와 짝(%64 · %103 · %597 · %595 · %127…) \| (배치 J) L243/L251 동상(ll 56581 · 56933 · 56906 등). | 4 | OK |
| 11 | Entity | 0x670 | hp | r | L232 ally.hp.max(1) — 압박 점수 분모(umax 1 → sdiv) | 4 | OK |
| 12 | PlayerState | 0x930 | info.team | r | L201 team(0/1) · <2 bounds check(panic_bounds_check(team,2) simulation.rs:1905) · 1-team = 적팀 행/blackboard 인덱스(%79) | 4 | OK |
| 13 | OperationData | 0x0 | cache | r | L201 &AbstractGameWithCache(%51) · L226/L237 cache.game 팻포인터 재로드(%117/%118 · %585/%586) \| (배치 J) %51 (로드는 배치 I L201 ll 55960) — 내 범위에서 player_champion 기저와 game 팻포인터(%117/%118) 로 소비. | 4 | OK |
| 14 | OperationData | 0x10 | blackboard | r | &[Blackboard;2](%83) → %84 = &blackboard[1 - team] (★적팀 blackboard · IR 정본 · 줄 길이 산술도 `data.blackboard[1 - player.info.team]` 37자와 정합) — is_recent_visible 의 &self \| (배치 J) %83 (로드 ll 56009, 배치 I 호이스트) → %84 = &blackboard[1-team](744B stride) 를 L243 is_recent_visible 의 &self 로 전달 ×5. _docs game_core.txt:21 「Blackboard[team]은 team 팀 자체 정보 추적, 관측은 1-team」 ⟹ [1-team] = 적 팀 챔피언에 관한 관측 정보. | 3 | OK |
| 15 | AbstractGameWithCache | 0x0 | game.data_ptr | r | L226/L237 is_recent_visible 의 game 인자(dyn AbstractGame 팻포인터 앞 절반) | 4 | OK |
| 16 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r | 팻포인터 뒤 절반(%118/%586) | 4 | OK |
| 17 | AbstractGameWithCache | 0x1e0 | player_champion[team][0..5] | r | L201 자기 행(gep [5 x ptr] %56, team → %57 · 끝 %58=+40) 을 filter_map(closure#0) 로 순회 · L226/L237 적 행(%80 = 행 1-team) 5칸을 언롤(+0/+8/+16/+24/+32 = %80/%85/%86/%87/%88) · 원소 Option<&Entity> 니치(null=None) \| (배치 J) [[Option<&Entity>;5];2] · 팀당 40B. L251: %57=&[team] 의 +0/+8/+16/+24/+32 로드(ll 56922/56979/57038/57097/57156) · L243: %80=&[1-team] 의 5칸 로드(ll 56552/56610/56676/56742/56808). None(null) 칸은 스킵(iter_champions filter_map). | 4 | OK |
| 18 | ScoreParameter | 0x918 | player (ChampionScoreParameter 216B) | r | L209~212 자기 자신일 때의 p(%73) — possible_risk 의 &self · champion_hp_value 의 3번째 인자 | 4 | OK |
| 19 | ScoreParameter | 0x988 | player.applyed_damage | r | L209 (%74/%75, 블록 %55 호이스팅 · dbg 없음) | 4 | OK |
| 20 | ScoreParameter | 0x998 | player.risk_damage | r | L210 (%76/%77) · %78 = applyed + risk | 4 | OK |
| 21 | ScoreParameter | 0x14b8 | near_allies.buf.ptr | r | L214 iter().find 시작(%68) | 4 | OK |
| 22 | ScoreParameter | 0x14d0 | near_allies.len | r | L214 끝 = ptr + len*216(%71 gep 원소타입 216B) | 4 | OK |
| 23 | ChampionScoreParameter | 0x58 | id | r | L214 find 술어 p.id == ally.id(%565 == %557) · 원소 stride 216(gep %560 +216) | 4 | OK |
| 24 | ChampionScoreParameter | 0x70 | applyed_damage | r | L216 (%568/%569) | 4 | OK |
| 25 | ChampionScoreParameter | 0x80 | risk_damage | r | L217 (%570/%571) · %572 = risk + applyed | 4 | OK |
| 26 | KnightUltAction | 0x18 | damage_reduce | r | L253 (ll 57214~57216): `(damage_reduce as i64)/3` 을 18 로 캡해 score 초기값. base 는 action 을 다운캐스트한 %16(배치 I L185). tcxdict KnightUltAction 80B: +0x10 range(배치 I zone_radius)·+0x18 damage_reduce·+0x30 duration(배치 I %72) 과 오프셋 정합. | 3 | OK |

**`consts` 상수 30건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev | folded_from |
|---|---|---|---|---|---|---|
| 0 | 166200278922247868797912749936351073298 | 185 | 태그 | TypeId::of::<game_core::setting::champion::knight::KnightUltAction>() (u128) — action.as_any().downcast_ref::<KnightUltAction>() 의 판별 상수. 불일치 → return 0. 타입 확정 근거 = DI `knight_ult: ref$<KnightUltAction>`(!56546) + dloc any.rs:229 downcast_ref<KnightUltAction> | 4 |  |
| 1 | 0 | 188 | 태그 | TeamType 태그 0=Player — 둘 다 0 이면 +0x8 팀 인덱스까지 비교(derive PartialEq). 또한 L197~199 누산기 초기값 0, L222 near_allies 미발견 시 incoming=0, L226 any 거짓=0 | 4 |  |
| 2 | 13 | 188 | 태그 | EntityType::Champion 메모리태그(tcxdict --enum EntityType: idx13=discr13=tag13) — target.is_champion() 게이트 | 3 |  |
| 3 | 35000 | 194 | 계수 | influence = zone_radius + 35000 — 존 바깥 영향권 여유(약 1.09셀 · 셀=32000). enemies_near(L238) 반경 = (range+35000)² | 4 |  |
| 4 | 2 | 201 | 임계 | player_champion 행 인덱스 상한(team<2, panic_bounds_check(team, 2) · simulation.rs:1905 iter_champions 인라인) | 4 |  |
| 5 | 30 | 211 | 계수 | possible_risk 의 tick_cut = knight_ult.duration + 30 (L211 자기 자신 · L218 아군 — 컴파일러가 블록 %573 하나로 합쳐 dbg 소실, %577) — 궁 지속시간 + 30틱 이내의 잠재 위험만 합산 | 4 |  |
| 6 | 100 | 212 | 임계 | hp_value = champion_hp_value(..).min(100) 상한(L212 자기 · L219 아군 · smin %581 cmp.rs:1078 min<i64>) | 4 |  |
| 7 | 50 | 222 | 인덱스 | near_allies 에 그 아군의 ChampionScoreParameter 가 없을 때 (incoming, hp_value) = (0, 50) — hp_value 기본값(phi %583 [50,%559]) | 4 |  |
| 8 | 10000000001 | 227 | 임계 | 100000² (=100_000*100_000 · 약 3.125셀) — enemy.distance_sq(ally) <= 100000² 를 LLVM 이 `icmp ult …, 10000000001` 로 정규화. enemy_close_to_ally 반경 | 4 | 10000000000 |
| 9 | 1 | 232 | 인덱스 | ally.hp.max(1) — 압박 점수 분모 0 방지(umax cmp.rs:1039 max<usize>). 또한 L229 threatened_allies += 1 · L206 allies_in_zone += 1 · L239 count 의 zext 증분 | 4 |  |
| 10 | 45 | 232 | 임계 | 아군 1명당 압박 점수 상한: ((incoming*hp_value)/ally.hp.max(1)).min(45) — pressure_score 누적 전 clamp(smin %734) | 4 |  |
| 11 | 0 | 245 | 임계 | `(enemies_in_zone \| threatened_allies) == 0` → return 0 (ll 56875~56877 · `or` 로 접힌 두 `==0` 의 AND) | 4 |  |
| 12 | 3 | 253 | 인덱스 | damage_reduce / 3 (sdiv · ll 57216) | 4 |  |
| 13 | 18 | 253 | 임계 | damage_reduce/3 의 상한(llvm.smin · ll 57219) — score 초기값 | 4 |  |
| 14 | 5 | 254 | 미상 | 기본 가산 +5 (ll 57242) | 4 |  |
| 15 | 4 | 254 | 인덱스 | allies_in_zone 캡(llvm.smin · ll 57223) | 4 |  |
| 16 | 6 | 254 | 계수 | allies_in_zone 계수 — IR 은 `(min(enemies_in_zone,3) + min(allies_in_zone,4)) * 6`(ll 57240~57241, dbg L258) 로 인수분해돼 있어 L254 의 allies 계수와 L257 의 enemies_in_zone 계수가 둘 다 6 임은 확정(어느 줄에 6 이 적혀 있는지는 표기 불가) | 4 |  |
| 17 | 4 | 255 | 인덱스 | threatened_allies 캡(llvm.smin · ll 57227) | 4 |  |
| 18 | 14 | 255 | 계수 | threatened_allies 계수 ×14 (ll 57228) | 4 |  |
| 19 | 4 | 256 | 인덱스 | enemies_near 캡(llvm.umin · ll 57231 · usize) | 4 |  |
| 20 | 3 | 256 | 계수 | enemies_near 계수 ×3 (ll 57232) | 4 |  |
| 21 | 3 | 257 | 계수 | enemies_in_zone 캡(llvm.umin · ll 57235 · usize) · 계수 6 은 L254 항목 참조 | 4 |  |
| 22 | 50 | 258 | 인덱스 | pressure_score 캡(llvm.smin · ll 57238) | 4 |  |
| 23 | 3 | 258 | 인덱스 | min(pressure_score,50) / 3 (sdiv · ll 57239) | 4 |  |
| 24 | 3 | 261 | 인덱스 | (self_cover - allies_in_zone) 캡(llvm.smin · ll 57255) | 4 |  |
| 25 | -12 | 261 | 계수 | 소스 `score -= x.min(3) * 12` 가 `mul -12` + `add` 로 접힘(ll 57256~57257) | 4 | 12 |
| 26 | 2 | 263 | 임계 | allies_in_zone < 2 (icmp slt · ll 57270) 및 enemies_near < 2 (icmp samesign ult · ll 57271) | 4 |  |
| 27 | -15 | 263 | 계수 | 자기대상(target.id==champ.id) && allies_in_zone<2 && enemies_near<2 → score -= 15 가 `add -15` + select 로 접힘(ll 57273~57274) | 4 | 15 |
| 28 | 0 | 267 | 임계 | score.max(0) 하한(llvm.smax · ll 57282) | 4 |  |
| 29 | 75 | 267 | 임계 | score.min(75) 상한(llvm.umin · ll 57285) — define 의 range(i64 0, 76) 과 일치 | 4 |  |

**`knobs` 조정점 19건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 존 영향권 여유(influence = range + 35000) | battle_common.rs:194 | 35000 | 올리면 존 밖 더 먼 가시 적까지 enemies_near 로 세어(배치 J 의 ×3 항) 보너스가 커진다 · 내리면 존 바로 밖 적만 | 4 | 기존 |
| 1 | 아군 위협 판정 반경(enemy_close_to_ally, 100000²) | battle_common.rs:227 | 10000000001 | IR 값은 100000²+1(ult). 올리면 멀리 있는 가시 적만으로도 존 내 아군이 '위협받음'(threatened_allies, incoming<=0 이라도)에 들어가 배치 J 의 ×14 항이 커진다 · 내리면 예상 피해가 있는 아군만 | 4 | 기존 |
| 2 | possible_risk 시간 창 여유(tick_cut = duration + 30) | battle_common.rs:211 · :218 | 30 | 올리면 궁 지속시간보다 더 뒤의 잠재 위험까지 incoming 에 합산 → threatened/pressure 가 늘어 보너스 증가 · 0 이면 지속시간 이내만 | 4 | 기존 |
| 3 | hp_value 상한 | battle_common.rs:212 · :219 | 100 | champion_hp_value 가 100 을 넘는 챔피언의 압박 기여를 100 으로 눌러 pressure_score 의 챔피언 가치 편차를 제한 · 올리면 고가치 아군 보호 성향↑ | 4 | 기존 |
| 4 | near_allies 미등록 아군의 기본 hp_value | battle_common.rs:222 | 50 | incoming=0 이라 L232 에서는 안 쓰이고 값만 남는다(배치 I 범위에서 실효 0 · 배치 J 에서도 hp_value 는 안 쓰임 — 루프 지역변수). 사실상 무효 노브 | 4 | 기존 |
| 5 | 아군 1명당 압박 점수 상한 | battle_common.rs:232 | 45 | ((incoming*hp_value)/hp).min(45). 올리면 한 명이 크게 위험할 때 pressure_score 가 더 커진다(배치 J 에서 pressure/3 ≤ 18 로 다시 눌림 → 총합 54 를 넘는 부분은 무효) · 내리면 여러 명이 고루 위험해야 보너스 | 4 | 기존 |
| 6 | is_recent_visible 최근 가시 창(120틱) | blackboard.rs:346 (g07.ll:157025 `add 120`) — 콜리 내부 | 120 | 현재 안 보여도 마지막 가시 후 120틱(2초@60tps) 이내면 가시로 침. 이 함수 상수 목록엔 없음(콜리 소유) | 4 | 기존 |
| 7 | 조기 탈락 조건(존 안 가시 적 0 && 위협 아군 0) | battle_common.rs:245 | 0 | 이 조건을 없애면 존에 적이 없어도 damage_reduce·아군·압박 항만으로 보너스가 붙는다(최소 5+dmg/3) | 4 | 기존 |
| 8 | damage_reduce 항 상한 | battle_common.rs:253 | 18 | 올리면 감쇠율 높은 궁 레벨에서 보너스 기저가 커진다(damage_reduce/3 이 18 을 넘을 때만 체감) | 4 | 기존 |
| 9 | damage_reduce 나눗수 | battle_common.rs:253 | 3 | 내리면 같은 damage_reduce 로 기저 점수 상승 | 4 | 기존 |
| 10 | 기본 가산 | battle_common.rs:254 | 5 | L245 통과한 모든 후보에 균일 가산 — 올리면 나이트 궁 후보 전반 가점 | 4 | 기존 |
| 11 | allies_in_zone 캡·계수 | battle_common.rs:254 | min 4 × 6 | 존 안 아군 수 반영폭(최대 24) — 계수 올리면 아군 뭉친 곳에 궁 선호 | 4 | 기존 |
| 12 | threatened_allies 캡·계수 | battle_common.rs:255 | min 4 × 14 | 가장 큰 항(최대 56) — 위협받는 아군 수가 보너스 주도. 계수 내리면 수비적 궁 사용 감소 | 4 | 기존 |
| 13 | enemies_near 캡·계수 | battle_common.rs:256 | min 4 × 3 | 영향권(influence, 배치 I) 안 적 수 반영(최대 12) | 4 | 기존 |
| 14 | enemies_in_zone 캡·계수 | battle_common.rs:257 | min 3 × 6 | 존 안 가시 적 수 반영(최대 18) — 캡 3 이라 4명 이상 몰려도 추가 가점 없음 | 4 | 기존 |
| 15 | pressure_score 캡·나눗수 | battle_common.rs:258 | min 50 / 3 | 압박 점수(배치 I 루프) 반영(최대 16) | 4 | 기존 |
| 16 | 자기 주변 아군 과다 감점 | battle_common.rs:260~261 | (self_cover-allies_in_zone).min(3) × 12 | 캐스터 주변 아군이 target 존 안 아군보다 많으면 최대 -36 — 계수 내리면 아군에서 떨어진 대상에도 궁을 쓴다 | 4 | 기존 |
| 17 | 자기대상 캐스팅 감점 | battle_common.rs:263 | -15 (allies_in_zone<2 && enemies_near<2) | target==champ 이고 주변이 한산하면 -15 — 임계 2 를 올리면 자기대상 궁이 더 자주 감점 | 4 | 기존 |
| 18 | 최종 클램프 | battle_common.rs:267 | 0..=75 | 상한 75 를 올리면 항 합(이론 최대 18+5+24+56+12+18+16=149)이 더 반영된다 — 현재는 대부분 상한에서 잘림 | 4 | 기존 |

<details><summary>`callees` 피호출자 31건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | as_any | <game_ai::AgentVerHamster as game_core::AiAgent>::as_any | pub | fn(&game_ai::AgentVerHamster) -> &dyn [Binder { value: Trait(std::any::Any), bound_vars: [] }] + 'static | game-ai\src\lib.rs:425 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 321개 중 상위 3개 |
| 1 | as_any | game_core::Action::as_any | pub | fn(&Self/#0) -> &dyn [Binder { value: Trait(std::any::Any), bound_vars: [] }] + 'static | game-core\src\setting\action.rs:12 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 321개 중 상위 3개 |
| 2 | as_any | game_core::AiAgent::as_any | pub | fn(&Self/#0) -> &dyn [Binder { value: Trait(std::any::Any), bound_vars: [] }] + 'static | game-core\src\simulation\ai_interface.rs:499 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 321개 중 상위 3개 |
| 3 | champion_hp_value | game_ai::champion_hp_value | pub | fn(&game_core::OperationData, &game_ai::ScoreParameter, &game_ai::ChampionScoreParameter) -> i64 | game-ai\src\utils.rs:909 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | distance_sq | game_core::utils::distance_sq | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:6 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 5 | distance_sq | game_core::Entity::distance_sq | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2157 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 6 | duration | game_core::Action::duration | pub | fn(&Self/#0) -> usize | game-core\src\setting\action.rs:16 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 197개 중 상위 3개 |
| 7 | duration | game_view::UIPhaseEffect::duration | pub | fn(&game_view::UIPhaseEffect) -> f32 | game-view\src\ui\match_ui\phase_effect.rs:35 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 197개 중 상위 3개 |
| 8 | duration | game_view::ui::match_ui::BanpickShowcaseFx::duration | in:game_view::ui::match_ui | fn(&game_view::ui::match_ui::BanpickShowcaseFx) -> f32 | game-view\src\ui\match_ui.rs:1664 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 197개 중 상위 3개 |
| 9 | find | game_ai::MinionWaveSnapshot::find | pub | fn(&game_ai::MinionWaveSnapshot, usize) -> std::option::Option<&game_ai::MinionHpTrajectory> | game-ai\src\utils.rs:84 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 10 | get_player_by_champion_id | game_core::AbstractGame::get_player_by_champion_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::PlayerState> | game-core\src\simulation.rs:145 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 11 | get_player_by_champion_id | <game_core::Game as game_core::AbstractGame>::get_player_by_champion_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::PlayerState> | game-core\src\simulation\game.rs:1874 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 12 | get_player_by_champion_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_player_by_champion_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::PlayerState> | game-core\src\simulation\game.rs:3922 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 13 | is_champion | game_core::EntityType::is_champion | pub | fn(&game_core::EntityType) -> bool | game-core\src\simulation\entity.rs:1403 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 14 | is_recent_visible | game_core::Blackboard::is_recent_visible | pub | fn(&game_core::Blackboard, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\game\blackboard.rs:346 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 15 | is_visible | game_core::World::is_visible | pub | fn(&game_core::World, usize, usize) -> bool | game-core\src\simulation\game\data.rs:384 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 16 | is_visible | game_core::VisibleState::is_visible | pub | fn(&game_core::VisibleState) -> bool | game-core\src\simulation\game\data.rs:121 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 17 | is_visible | game_core::AbstractGame::is_visible | pub | fn(&Self/#0, usize, usize) -> bool | game-core\src\simulation.rs:125 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 18 | iter_champions | game_core::AbstractGameWithCache::<'a, 'b>::iter_champions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5759 ~ game_core[6a30]::simulation::{impl#5}::iter_champions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1904 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 19 | possible_risk | game_ai::ChampionScoreParameter::<'_>::possible_risk | pub | fn(&game_ai::ChampionScoreParameter</#0>, &game_core::OperationData, usize) -> i64 | game-ai\src\score_parameter.rs:1407 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 20 | range | game_core::Effect::range | pub | fn(&game_core::Effect, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:25 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 21 | score | game_ai::plan_legacy::sub_plan::SubPlan::score | pub | fn(&mut game_ai::plan_legacy::sub_plan::SubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 | game-ai\src\plan_legacy\sub_plan\mod.rs:164 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 26개 중 상위 3개 |
| 22 | score | game_ai::plan_legacy::sub_plan::HideSubPlan::score | pub | fn(&game_ai::plan_legacy::sub_plan::HideSubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 | game-ai\src\plan_legacy\sub_plan\hide.rs:238 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 26개 중 상위 3개 |
| 23 | score | game_ai::plan_legacy::sub_plan::StealSubPlan::score | pub | fn(&game_ai::plan_legacy::sub_plan::StealSubPlan, usize, &game_ai::ScoreParameter, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &mut game_core::DebugFrameData) -> i64 | game-ai\src\plan_legacy\sub_plan\steal.rs:136 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 26개 중 상위 3개 |
| 24 | team | game_view::ClientData::team | pub | fn(&game_view::ClientData, usize) -> std::option::Option<std::cell::Ref< game_core::Team>> | game-view\src\logic\client\data.rs:1651 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 25 | team | game_view::ClientDatabase::team | pub | fn(&game_view::ClientDatabase, usize) -> std::option::Option<&game_core::Team> | game-view\src\logic\client\data.rs:968 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 26 | team | game_core::PlayerAiContext::<'a, 'b, 'r>::team | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:583 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 27 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 28 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 29 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 30 | type_id | game_view::worker::WorkerMessage::type_id | pub | fn(&game_view::worker::WorkerMessage) -> &str | game-view\src\logic\server\worker.rs:445 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
</details>

⚠**미매칭 23개**: `action`, `allies_in_zone`, `applyed_damage`, `bool`, `champ`, `data_ptr`, `enemies_in_zone`, `enemies_near`, `enemy_close_to_ally`, `llvm.assume`, `llvm.smax.i64`, `llvm.smin.i64`, `llvm.umax.i64`, `llvm.umin.i64`, `near_allies`, `pressure_score`, `risk_damage`, `risk_possible`, `smax`, `target`, `threatened_allies`, `usize`, `zone_sq`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 2곳** (m02.ll:38216, m15.ll:12656) · **형제 0개** 

**`open` 13건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | (배치 I) 정확한 소스 표기(들여쓰기·식 순서)는 rmeta 줄 길이 산술로 복원한 추정 — `L238` 만 길이 48 이 `.filter(\|enemy\| enemy.distance_sq(target) <= influence_sq)`(59) 와 안 맞아 클로저 인자 이름/표현이 다를 것(동작은 IR 로 확정: ule influence_sq) | 3 |  |
| 1 | 표기 불가 | (배치 I) L228 `incoming > 0 \|\| enemy_close_to_ally` 는 줄 길이 44 정합 + IR(else 분기에서 threatened += enemy_close_to_ally) 로 복원한 것. DI 가 enemy_close_to_ally 를 i64 로 보고하나 소스는 bool(any) 일 가능성이 높음 — 동작 동일(표기 불가) | 3 |  |
| 2 | 미탐색 | (배치 I) data.blackboard[1 - team](적팀 blackboard)를 아군 player 와 함께 is_recent_visible 에 넘기는 것이 의도인지(자기팀 blackboard 오타인지) — IR·줄 길이 모두 1-team. 판정 동작은 확정, 의도만 미상. blackboard.last_visible 의 기록 주체(어느 팀 기준 '마지막 가시'인지)는 game_core 경계라 미독 | 3 |  |
| 3 | 미탐색 | (배치 I) possible_risk 내부(중복 제거 기준 필드·합산 필드 오프셋) — 그 밖의 콜리 규칙으로 시그니처·반환 의미만 적음(m07.ll:7315~7564 미상세) | 4 |  |
| 4 | 미탐색 | (배치 I) champion_hp_value TLS 메모의 실제 해시 키 구성·무효화 — 콜리 소유(m00.ll:91035 클로저 · r13 잎 champion_hp_value_uncached 자식 명세 소관) | 4 |  |
| 5 | 미탐색 | (배치 I) 판 8 리턴 주소 실측에 이 함수 프로브가 없어 발화수 미계측 — 정적 호출자 2곳(Battle/DeathBattle score) 뿐이고 vtable/fn-ptr 간접 호출 흔적 없음(grep: invoke 2 · declare 2 · gv 요약) | 4 |  |
| 6 | 미탐색 | (배치 I) 배치 J(줄 243~268): enemies_in_zone(L243)·L245 분기·L251 5칸 언롤 카운트(중심 %391/%393)·가중합 계수(/3≤18 · min4 · ×14 · umin4×3 · umin3 · min50/3 · ×6 · 적>아군 ×−12 · %540==champ.id 분기)·clamp 0..75 — 이 배치 범위 밖, one_line 용으로 훑기만 함 | 4 |  |
| 7 | 표기 불가 | (배치 J) L245 `A==0 && B==0` 의 A/B 소스 순서 — column 0 이고 IR 이 `or` 하나로 접혀 표기 불가(동작은 확정: 둘 다 0 일 때만 0 반환). | 4 |  |
| 8 | 표기 불가 | (배치 J) L263 세 조건(target.id==champ.id / allies_in_zone<2 / enemies_near<2)의 소스 순서 — IR 은 id 비교만 br 이고 나머지는 and+select 로 평탄화(부작용 없음) ⟹ 표기 불가 · 동작은 세 조건 AND 로 확정. | 4 |  |
| 9 | 표기 불가 | (배치 J) 계수 6 이 L254(allies) 와 L257(enemies_in_zone) 중 어느 줄의 리터럴인지 — IR 이 `(min(z,3)+min(a,4))*6` 로 인수분해(ll 57240~57241 dbg L258) ⟹ 둘 다 6 임은 확정, 줄 배정만 표기 불가. | 4 |  |
| 10 | 미탐색 | (배치 J) self_cover(usize count) 와 allies_in_zone(i64) 의 비교가 `icmp sgt`·`sub` 인 점에서 `self_cover as i64` 캐스팅으로 읽었다(추정 · 동작엔 영향 없음: 두 값 모두 0..=5). | 4 |  |
| 11 | 미탐색 | (배치 J) 발화수·발화 조건(런타임): 판 8 에 이 함수 프로브가 없어 미계측. 정적 호출자는 BattleSubPlan::score(battle.rs:760) 1곳(_gaibc 전량 grep). score 안에서 블록 %548 도달 조건은 r17 명세 소관이라 여기 적지 않음. | 4 |  |
| 12 | 미탐색 | (배치 J) blackboard[1-team] 을 &self 로 넘기는 의미는 _docs game_core.txt:21(「Blackboard[team]은 team 팀 자체 정보 추적, 관측은 1-team」) 에 근거해 「적 팀 챔피언에 관한 관측 정보」로 읽었다 — 주석 근거(IR 은 인덱스 1-team 만 확정). | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

