---

### `256` LegacyPlanHandler::update_on_dead — 사망 중 플랜 갱신 — 비수동 플랜을 passive_plan 으로 교체(교체 전 Battle 이면 ff/mf 계측 스냅샷)하고, 기한 지난 수신/대기 채팅을 처리·정리한 뒤 team_plan·GoalData 를 갱신

| 항목 | 값 |
|---|---|
| id | `LegacyPlanHandler__update_on_dead` |
| 심볼 | `_RNvMNtNtCshdEBA0ozCnw_7game_ai11plan_legacy7handlerNtB2_17LegacyPlanHandler14update_on_dead` |
| 소스 | `game-ai\src\plan_legacy\handler.rs:635` |
| IR | `m13.ll` 10120~10671행 |
| 경로·가시성 | `game_ai::plan_legacy::handler::LegacyPlanHandler::update_on_dead` · **pub** |
| 계층 | 플랜 핸들러 |
| exe | `e49a50` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r18` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData)
```

<details><summary>인자 6개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | self | &mut LegacyPlanHandler(6168B, ptr %0) | dereferenceable(6168) · noalias · readonly/readnone/initializes 속성 없음(=&mut · 쓰기 표면은 writes 전수) | 4 |
| 1 | 2 | version | usize(i64 %1) | 값 인자 · 본문 분기 없음 — passive_plan / handle_chat / TeamPlan::update / GoalData::update 에 전달만 | 4 |
| 2 | 3 | rnd | &mut StdRng(320B, ptr %2) | dereferenceable(320) · noalias · readonly 없음(=&mut) · 본문 직접 접근 0 · gen_range 호출 사이트: 본문 0회. 콜리에 전달 순서 = passive_plan(L644, 비수동일 때만) → handle_chat(L659, 허용 채팅마다) → TeamPlan::update(L665) → GoalData::update(L667 · 콜리 define 은 rnd readnone = 소비 안 함 m09.ll:4395) | 4 |
| 3 | 4 | player | &PlayerState(2528B, ptr %3) | readonly · captures(address, read_provenance) · 본문 직접 읽기 0 — 콜리 전달만 | 4 |
| 4 | 5 | data | &OperationData(24B, ptr %4) | readonly · captures(address, read_provenance) · +0x0 cache, +0x8 context 를 꺼내 sanitize_rule_scope/chat_allowed/retain 술어에 전달 · cache.game.tick() 호출 | 4 |
| 5 | 6 | debug | &mut DebugFrameData(224B, ptr %5) | dereferenceable(224) · noalias · readonly 없음 · 본문 직접 접근 0 · passive_plan / handle_chat / TeamPlan::update / GoalData::update 에 전달만 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn update_on_dead(&mut self, version, rnd, player, data, debug)   // handler.rs:635~679
cache = data.cache; ctx = data.context; game = cache.game
L636: self.sanitize_rule_scope(cache, ctx)                                   // m13.ll:10160 (fastcc · 계약)
L639: self.v2_egowave = 0                                                    // +0x1815
L640: self.v2_obj_part = None                                                // +0x1800 태그 0
L642: if !self.plan.is_passive() {   // is_passive(types.rs:254) = PassiveLine | SinglePlanLine | (PassiveJungle && !is_counter_jungle())
        //   is_counter_jungle(passive_jungle.rs:104) = plan.team(+0x638) != plan.player_team(+0x640)
        //   IR: idx = tag>1 ? tag-2 : 4 ; switch idx {1,2 → skip ; 5 → team==player_team ? skip : 교체 ; _ → 교체}   (m13.ll:10167~10187)
L643:   tick = game.tick(); self.ff_note_battle_swap(tick, 16)   // handler.rs:401: if let BigPlan::Battle(b) = &self.plan(태그 9) {
        //   ff_battle_exit = (16, b.entry_src, tick, b.exit_sub, b.ff_wave_obs_open);
        //   ff_battle_exit_latch = (b.ff_exit1_cls, b.ff_wave_open_hp, b.ff_wave_open_pct, b.ff_wave_fire_hp, b.ff_wave_fire_pct, b.ff_wave_open_danger, b.ff_wave_fire_danger, b.dive_abort_src) }
L644:   (new_plan, _u8) = self.passive_plan(version, rnd, player, data, debug)   // 392B sret · .1 은 버림 (m13.ll:10259~10261)
L645:   self.mf_note_swap(game.tick(), 24)   // handler.rs:410: mf_swap = (24, tick) — tick 을 다시 vtable 호출(m13.ll:10264)
L646:   self.plan = new_plan   // 옛 plan drop_glue(BigPlan) → memcpy 384B (unwind 시엔 새 plan 을 memcpy 후 전파 · m13.ll:10283~10286)
      }
L649: handled_chats: Vec<(usize,Position,Chat)> = Vec::new()
L650: for (tick, from, chat) in self.received_chats.iter() {                 // 원소 40B
L651:   if *tick > game.tick() { continue }    // 미래 채팅은 남김 (m13.ll:10368~10369)
L652:   handled_chats.push((*tick, *from, chat.clone()))   // grow_one 가능
      }
L655: self.received_chats.retain(|(t,_,_)| *t > game.tick())   // aux m06.ll:640 — 처리분(tick ≤ now) 제거
L656: for (tick, from, chat) in handled_chats {             // IntoIter · from==-1 이면 None 종료(니치 · 실질 불가)
L657:   if rule_scope::chat_allowed(ctx, &chat) {
L658:     misunderstood = self.take_misunderstood_received_chat(tick, from, chat.clone())   // fastcc → bool (계약)
L659:     self.handle_chat(version, rnd, player, data, from, chat, misunderstood, debug)   // (계약)
        } }
L661: drop(handled_chats IntoIter)
L662: self.misunderstood_received_chats.retain(|(t,_,_)| *t > game.tick())   // aux m06.ll:776
L665: self.team_plan.update(version, rnd, player, data, debug)               // (계약 · &mut TeamPlan 1064B)
L666: self.chats.extend(self.team_plan.chats.drain(..))                     // TeamPlan+0xc0 → self+0x1b8 비움, self.chats(+0x7c8) 에 붙임
L667: self.data.update(version, rnd, player, data, debug)                   // GoalData::update(&mut self.data @+0x0) (계약 · rnd readnone)
L668: self.sanitize_rule_scope(cache, ctx)
L670: for (tick, chat) in self.chats_wait.iter() {                          // 원소 32B
L671:   if *tick > game.tick() { continue }
L672:   if rule_scope::chat_allowed(ctx, chat) {
L673:     self.chats.push(chat.clone()) } }                                  // grow_one 가능
L677: self.chats_wait.retain(|(t,_)| *t > game.tick())                      // aux m06.ll:2458
L678: self.sanitize_rule_scope(cache, ctx)
L679: return

순서 고정(부작용 순): sanitize → 플래그 리셋 → [비수동] ff 스냅샷 → passive_plan → mf 스냅샷 → plan 교체 → received_chats 처리(handle_chat 순서 = 벡터 순서) → retain → misunderstood retain → TeamPlan::update → chats 병합 → GoalData::update → sanitize → chats_wait 처리 → retain → sanitize.
game.tick() 호출 횟수: 비수동이면 2 + 채팅 순회마다 1 + retain 술어마다 원소당 1 — 값은 같은 틱이라 동일(순수 조회).
```

**`mem` 메모리 접근 48건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | OperationData | 0x0 | cache | r | &AbstractGameWithCache(m13.ll:10157) | 4 | OK |  |
| 1 | OperationData | 0x8 | context | r | &GameContext(m13.ll:10159) — sanitize_rule_scope·chat_allowed 인자 | 4 | OK |  |
| 2 | AbstractGameWithCache | 0x0 | game.data_ptr | r | &dyn AbstractGame(m13.ll:10190) | 4 | OK |  |
| 3 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r | 슬롯 0x28 = tick (divtable AbstractGame) — L643·L645·L651·L671 + retain 술어 3곳 | 3 | OK |  |
| 4 | LegacyPlanHandler | 0x5e8 | plan@tag | r | BigPlan 8B 니치 태그(niche_start=2 · DeathMatchBattle 은 untagged) — 논리 idx = tag>1 ? tag-2 : 4 (m13.ll:10167~10172 · types.rs:254 is_passive 인라인) | 4 | OK |  |
| 5 | LegacyPlanHandler | 0x638 | plan@PassiveJungle.0.team | r | is_counter_jungle(passive_jungle.rs:104) = team != player_team (m13.ll:10182~10186) | 4 | OK |  |
| 6 | LegacyPlanHandler | 0x640 | plan@PassiveJungle.0.player_team | r |  | 4 | OK |  |
| 7 | LegacyPlanHandler | 0x6f7 | plan@Battle.0.entry_src | r | ff_note_battle_swap(handler.rs:401~404) — Battle(태그 9) 일 때만 | 4 | OK |  |
| 8 | LegacyPlanHandler | 0x701 | plan@Battle.0.exit_sub | r |  | 4 | OK |  |
| 9 | LegacyPlanHandler | 0x6f9 | plan@Battle.0.ff_wave_obs_open | r |  | 4 | OK |  |
| 10 | LegacyPlanHandler | 0x700 | plan@Battle.0.ff_exit1_cls | r |  | 4 | OK |  |
| 11 | LegacyPlanHandler | 0x6fa | plan@Battle.0.ff_wave_open_hp | r |  | 4 | OK |  |
| 12 | LegacyPlanHandler | 0x6fb | plan@Battle.0.ff_wave_open_pct | r |  | 4 | OK |  |
| 13 | LegacyPlanHandler | 0x6fc | plan@Battle.0.ff_wave_fire_hp | r |  | 4 | OK |  |
| 14 | LegacyPlanHandler | 0x6fd | plan@Battle.0.ff_wave_fire_pct | r |  | 4 | OK |  |
| 15 | LegacyPlanHandler | 0x6fe | plan@Battle.0.ff_wave_open_danger | r |  | 4 | OK |  |
| 16 | LegacyPlanHandler | 0x6ff | plan@Battle.0.ff_wave_fire_danger | r |  | 4 | OK |  |
| 17 | LegacyPlanHandler | 0x6f3 | plan@Battle.0.dive_abort_src | r |  | 4 | OK |  |
| 18 | LegacyPlanHandler | 0x7f8 | received_chats(cap@+0x7f8 ptr@+0x800 len@+0x808) | r | Vec<(usize,Position,Chat)> 원소 40B {tick i64, from i32, pad, Chat 24B@+16} — L650 순회(m13.ll:10303~10316) | 4 | OK |  |
| 19 | LegacyPlanHandler | 0x7e0 | chats_wait(cap@+0x7e0 ptr@+0x7e8 len@+0x7f0) | r | Vec<(usize,Chat)> 원소 32B {tick i64, Chat 24B@+8} — L670 순회(m13.ll:10534~10547) | 4 | OK |  |
| 20 | LegacyPlanHandler | 0x1b8 | team_plan.chats(TeamPlan+0xc0) | r | L666 drain(..) 원본(m13.ll:10513) | 4 | OK |  |
| 21 | GameContext | 0x0 | (전달만) | r | chat_allowed(&GameContext, &Chat) 인자 — 본문에서 필드 직접 읽기 없음 | 4 | OK |  |
| 22 | LegacyPlanHandler | 0x1815 | v2_egowave | w | L639 무조건(m13.ll:10161~10162) | 4 | OK | 0 |
| 23 | LegacyPlanHandler | 0x1800 | v2_obj_part@tag | w | L640 무조건(m13.ll:10163~10164) · Option<u8> 2B 의 태그 바이트만 씀 | 4 | OK | 0 (=None) |
| 24 | LegacyPlanHandler | 0x1600 | ff_battle_exit.0 | w | L643 ff_note_battle_swap(tick,16) — 조건: !plan.is_passive() && plan 태그==9(Battle) (m13.ll:10215) | 4 | OK | 16 (src) |
| 25 | LegacyPlanHandler | 0x1601 | ff_battle_exit.1 | w | m13.ll:10217 | 4 | OK | plan.Battle.entry_src(+0x6f7) |
| 26 | LegacyPlanHandler | 0x15f8 | ff_battle_exit.2 | w | m13.ll:10218 | 4 | OK | game.tick() |
| 27 | LegacyPlanHandler | 0x1602 | ff_battle_exit.3 | w | m13.ll:10220 | 4 | OK | plan.Battle.exit_sub(+0x701) |
| 28 | LegacyPlanHandler | 0x1603 | ff_battle_exit.4 | w | m13.ll:10222 | 4 | OK | plan.Battle.ff_wave_obs_open(+0x6f9) |
| 29 | LegacyPlanHandler | 0x1608 | ff_battle_exit_latch.0 | w | L403 (m13.ll:10240) | 4 | OK | plan.Battle.ff_exit1_cls(+0x700) |
| 30 | LegacyPlanHandler | 0x1609 | ff_battle_exit_latch.1 | w | m13.ll:10242 | 4 | OK | ff_wave_open_hp(+0x6fa) |
| 31 | LegacyPlanHandler | 0x160a | ff_battle_exit_latch.2 | w | m13.ll:10244 | 4 | OK | ff_wave_open_pct(+0x6fb) |
| 32 | LegacyPlanHandler | 0x160b | ff_battle_exit_latch.3 | w | m13.ll:10246 | 4 | OK | ff_wave_fire_hp(+0x6fc) |
| 33 | LegacyPlanHandler | 0x160c | ff_battle_exit_latch.4 | w | m13.ll:10248 | 4 | OK | ff_wave_fire_pct(+0x6fd) |
| 34 | LegacyPlanHandler | 0x160d | ff_battle_exit_latch.5 | w | m13.ll:10250 | 4 | OK | ff_wave_open_danger(+0x6fe) |
| 35 | LegacyPlanHandler | 0x160e | ff_battle_exit_latch.6 | w | m13.ll:10252 | 4 | OK | ff_wave_fire_danger(+0x6ff) |
| 36 | LegacyPlanHandler | 0x160f | ff_battle_exit_latch.7 | w | L404 (m13.ll:10254) | 4 | OK | dive_abort_src(+0x6f3) |
| 37 | LegacyPlanHandler | 0x1610 | mf_swap.0 | w | L645 mf_note_swap(tick,24 · handler.rs:410) — 조건: !plan.is_passive() (m13.ll:10277) | 4 | OK | 24 (src) |
| 38 | LegacyPlanHandler | 0x1618 | mf_swap.1 | w | m13.ll:10279 | 4 | OK | game.tick() (L645 재호출값) |
| 39 | LegacyPlanHandler | 0x5e8 | plan (BigPlan 384B 통째) | w | L646 — 조건: !plan.is_passive(). 옛 plan 은 drop_glue(BigPlan)(m13.ll:10280) 후 memcpy 384B(m13.ll:10289). 반환 튜플 (BigPlan,u8) 392B 중 .1(u8 @+384) 은 읽지 않고 버림 | 4 | OK | passive_plan(version,rnd,player,data,debug).0 |
| 40 | LegacyPlanHandler | 0x7f8 | received_chats | w | L655 (m13.ll:10359 · 술어 본체 m06.ll:684~688 `icmp ugt tick, game.tick()`) | 4 | OK | retain(\|(tick,_,_)\| tick > game.tick()) → len(+0x808) 갱신·원소 memmove |
| 41 | LegacyPlanHandler | 0x7b0 | misunderstood_received_chats | w | L662 (m13.ll:10500 · m06.ll:820~821) | 4 | OK | retain(같은 술어) → len(+0x7c0) |
| 42 | LegacyPlanHandler | 0xf8 | team_plan (1064B) | w | L665 (m13.ll:10505) | 4 | OK | TeamPlan::update 콜리 쓰기 표면(계약) |
| 43 | LegacyPlanHandler | 0x1b8 | team_plan.chats | w | L666 (m13.ll:10514) | 4 | OK | drain(..) → len 0 |
| 44 | LegacyPlanHandler | 0x7c8 | chats | w | cap@+0x7c8 ptr@+0x7d0 len@+0x7d8 (m13.ll:10518, 10603~10623) | 4 | OK | extend(team_plan.chats.drain(..)) [L666] + push(chat) [L673 · grow_one 가능] |
| 45 | LegacyPlanHandler | 0x0 | data (GoalData 248B) | w | L667 (m13.ll:10523) | 4 | OK | GoalData::update 콜리 쓰기 표면(계약) |
| 46 | LegacyPlanHandler | 0x7e0 | chats_wait | w | L677 (m13.ll:10580 · m06.ll:2502~2503) | 4 | OK | retain(\|(tick,_)\| tick > game.tick()) → len(+0x7f0) |
| 47 | LegacyPlanHandler | 0x0 | (self 전체 — sanitize_rule_scope ×3 / take_misunderstood_received_chat / handle_chat 콜리 쓰기 표면) | w | L636·L668·L678 sanitize_rule_scope(&mut self,&cache,&ctx) · L658 take_misunderstood_received_chat(&mut self,tick,from,chat)->bool · L659 handle_chat(&mut self,version,rnd,player,data,from,chat,misunderstood,debug) | 4 | OK | 계약 |

**`consts` 상수 14건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev | folded_from |
|---|---|---|---|---|---|---|
| 0 | 6 | 642 | 센티널 | `llvm.assume(tag != 6)` — BigPlan 니치 태그 6 은 미사용(2..=17 중 6 = 이론상 DeathMatchBattle 자리) · 판정 상수 아님(m13.ll:10168, 10200) | 4 |  |
| 1 | 4 | 642 | 태그 | 태그 ≤1 이면 논리 idx 4(=DeathMatchBattle untagged) (select m13.ll:10172) — 임계 아님 | 4 |  |
| 2 | 1 | 642 | 임계 | switch 논리 idx 1 = PassiveLine → 수동(교체 안 함) (m13.ll:10174) | 4 |  |
| 3 | 2 | 642 | 센티널 | switch 논리 idx 2 = SinglePlanLine → 수동 / `tag-2` 니치 환산(m13.ll:10170) | 4 |  |
| 4 | 5 | 642 | 태그 | switch 논리 idx 5 = PassiveJungle → is_counter_jungle(team != player_team) 이면 비수동, 아니면 수동 (m13.ll:10176) · ⚠aux m06.ll:2556 의 `shl nsw %35, 5` 는 (usize,Chat) 원소 32B stride 접힘(memmove 바이트수 = remaining*32) 이지 이 5 와 무관 | 4 | 32 |
| 5 | 9 | 643 | 태그 | BigPlan 메모리태그 9 = Battle — ff_note_battle_swap 스냅샷 조건(m13.ll:10202) | 4 |  |
| 6 | 16 | 643 | 산출값 | ff_battle_exit.0 = src 16 (사망 시 배틀 교체 출처 코드) (m13.ll:10215) | 4 |  |
| 7 | 24 | 645 | 산출값 | mf_swap.0 = src 24 (사망 시 플랜 스왑 출처 코드) (m13.ll:10277) | 4 |  |
| 8 | 0 | 639 | 태그 | v2_egowave=0 · v2_obj_part=None(태그 0) · handled_chats Vec::new(cap/len 0) · 부정 없음 | 4 |  |
| 9 | -1 | 656 | 센티널 | IntoIter<(usize,Position,Chat)>::next 의 Option 니치 — Position(i32) == -1 이면 None 취급해 루프 종료(m13.ll:10480). 실제 원소는 0..5 범위(range(i32 0,5)) 라 실질 도달 불가 | 4 |  |
| 10 | 384 | 646 | 미상 | BigPlan 크기 memcpy (m13.ll:10261, 10289) — 임계 아님 | 4 |  |
| 11 | 40 | 650 | 미상 | (usize,Position,Chat) 원소 stride(m13.ll:10343) — 임계 아님 | 4 |  |
| 12 | 32 | 670 | 미상 | (usize,Chat) 원소 stride(m13.ll:10568) — 임계 아님 | 4 |  |
| 13 | 24 | 652 | 산출값 | Chat 24B memcpy (m13.ll:10383·10487·10597·10621) — 리터럴 24 는 mf_swap src 와 공유 | 4 |  |

**`knobs` 조정점 3건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 사망 시 플랜 교체 대상 범위(is_passive 예외) | types.rs:254~257 (handler.rs:642 인라인) | PassiveLine · SinglePlanLine · PassiveJungle(비카운터정글) 은 유지 | 예외를 늘리면 사망 후에도 그 플랜이 유지되고, 줄이면 사망 즉시 passive_plan 으로 초기화된다 | 4 | 기존 |
| 1 | ff_battle_exit src 코드 | handler.rs:643 | 16 | 계측 라벨(사망 교체 출처) — 동작 영향 없음(mf/ff 계열은 관측 전용) | 4 | 기존 |
| 2 | mf_swap src 코드 | handler.rs:645 | 24 | 계측 라벨 — 동작 영향 없음 | 4 | 기존 |

<details><summary>`callees` 피호출자 25건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | chat_allowed | game_ai::plan_legacy::rule_scope::chat_allowed | pub | fn(&game_core::GameContext, &game_core::Chat) -> bool | game-ai\src\plan_legacy\rule_scope.rs:128 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | chats | game_ai::plan_legacy::types::BigPlan::chats | pub | fn(&mut game_ai::plan_legacy::types::BigPlan) -> std::option::Option<&mut std::vec::Vec<game_core::Chat, std::alloc::Global>> | game-ai\src\plan_legacy\types.rs:184 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 2 | ff_note_battle_swap | game_ai::plan_legacy::handler::LegacyPlanHandler::ff_note_battle_swap | in:game_ai | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, u8, usize) | game-ai\src\plan_legacy\handler.rs:400 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 3 | handle_chat | game_ai::plan_legacy::handler::chat::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::handle_chat | pub | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, game_core::Position, game_core::Chat, bool, &mut game_core::DebugFrameData) | game-ai\src\plan_legacy\handler\chat.rs:8 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | is_counter_jungle | game_ai::plan_legacy::old::PassiveJunglePlan::is_counter_jungle | pub | fn(&game_ai::plan_legacy::old::PassiveJunglePlan) -> bool | game-ai\src\plan_legacy\old\passive_jungle.rs:103 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 5 | is_passive | game_ai::plan_legacy::types::BigPlan::is_passive | pub | fn(&game_ai::plan_legacy::types::BigPlan) -> bool | game-ai\src\plan_legacy\types.rs:253 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 6 | mf_note_swap | game_ai::plan_legacy::handler::LegacyPlanHandler::mf_note_swap | in:game_ai | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, u8, usize) | game-ai\src\plan_legacy\handler.rs:409 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 7 | passive_plan | game_ai::plan_legacy::handler::LegacyPlanHandler::passive_plan | in:game_ai::plan_legacy::handler | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> (game_ai::plan_legacy::types::BigPlan, u8) | game-ai\src\plan_legacy\handler.rs:1855 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 8 | player_team | game_core::TeamType::player_team | pub | fn(&game_core::TeamType) -> std::option::Option<usize> | game-core\src\simulation\entity.rs:1135 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 9 | player_team | game_view::ClientData::player_team | pub | fn(&game_view::ClientData) -> std::option::Option<std::cell::Ref< game_core::Team>> | game-view\src\logic\client\data.rs:1579 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 10 | player_team | game_view::ClientDatabase::player_team | pub | fn(&game_view::ClientDatabase) -> &game_core::Team | game-view\src\logic\client\data.rs:1163 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 11 | push | <game_core::Staff as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::staff | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\staff.rs:38 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 12 | push | <game_core::Athlete as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:434 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 13 | push | <game_core::Contract as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:2498 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 14 | sanitize_rule_scope | game_ai::plan_legacy::handler::LegacyPlanHandler::sanitize_rule_scope | in:game_ai::plan_legacy::handler | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, &game_core::OperationData) | game-ai\src\plan_legacy\handler.rs:571 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 15 | take_misunderstood_received_chat | game_ai::plan_legacy::handler::LegacyPlanHandler::take_misunderstood_received_chat | in:game_ai::plan_legacy::handler | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, game_core::Position, game_core::Chat) -> bool | game-ai\src\plan_legacy\handler.rs:588 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 16 | team | game_view::ClientData::team | pub | fn(&game_view::ClientData, usize) -> std::option::Option<std::cell::Ref< game_core::Team>> | game-view\src\logic\client\data.rs:1651 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 17 | team | game_view::ClientDatabase::team | pub | fn(&game_view::ClientDatabase, usize) -> std::option::Option<&game_core::Team> | game-view\src\logic\client\data.rs:968 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 18 | team | game_core::PlayerAiContext::<'a, 'b, 'r>::team | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:583 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 19 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 20 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 21 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 22 | update | game_ai::GoalData::update | pub | fn(&mut game_ai::GoalData, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) | game-ai\src\goal_data.rs:82 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 23 | update | game_ai::plan_legacy::team_plan::TeamPlan::update | pub | fn(&mut game_ai::plan_legacy::team_plan::TeamPlan, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) | game-ai\src\plan_legacy\team_plan.rs:294 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 24 | update_on_dead | game_ai::plan_legacy::handler::LegacyPlanHandler::update_on_dead | pub | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) | game-ai\src\plan_legacy\handler.rs:635 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
</details>

⚠**미매칭 12개**: `BigPlan>`, `Vec<`, `bool`, `drain`, `drop  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `extend`, `extend_trusted`, `grow_one`, `llvm.assume`, `llvm.memcpy.p0.p0.i64`, `plan`, `retain`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 2곳** (m14.ll:35912, m14.ll:56235) · **형제 41개** (LegacyPlanHandler)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::plan_legacy::handler::LegacyPlanHandler as std::clone::Clone>::clone | pub | game-ai\src\plan_legacy\handler.rs:13 | True | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler) -> game_ai::plan_legacy::handler::LegacyPlanHandler |
| 1 | <game_ai::plan_legacy::handler::LegacyPlanHandler as std::fmt::Debug>::fmt | pub | game-ai\src\plan_legacy\handler.rs:13 | True | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 2 | game_ai::plan_legacy::handler::LegacyPlanHandler::new | pub | game-ai\src\plan_legacy\handler.rs:228 | False | fn(usize, &mut rand::rngs::std::StdRng, usize, game_core::Position) -> game_ai::plan_legacy::handler::LegacyPlanHandler |
| 3 | game_ai::plan_legacy::handler::LegacyPlanHandler::r2_fight_protected | in:game_ai | game-ai\src\plan_legacy\handler.rs:362 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &game_core::PlayerState, &game_core::OperationData) -> bool |
| 4 | game_ai::plan_legacy::handler::LegacyPlanHandler::ff_note_battle_swap | in:game_ai | game-ai\src\plan_legacy\handler.rs:400 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, u8, usize) |
| 5 | game_ai::plan_legacy::handler::LegacyPlanHandler::mf_note_swap | in:game_ai | game-ai\src\plan_legacy\handler.rs:409 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, u8, usize) |
| 6 | game_ai::plan_legacy::handler::LegacyPlanHandler::v3_fall_back_to_passive | pub | game-ai\src\plan_legacy\handler.rs:416 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 7 | game_ai::plan_legacy::handler::LegacyPlanHandler::eo_cover_picks | pub | game-ai\src\plan_legacy\handler.rs:440 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler) -> usize |
| 8 | game_ai::plan_legacy::handler::LegacyPlanHandler::eo_serpen_punish_issues | pub | game-ai\src\plan_legacy\handler.rs:445 | True | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler) -> usize |
| 9 | game_ai::plan_legacy::handler::LegacyPlanHandler::subplan_is_recall | pub | game-ai\src\plan_legacy\handler.rs:450 | True | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler) -> bool |
| 10 | game_ai::plan_legacy::handler::LegacyPlanHandler::team_objective_code | pub | game-ai\src\plan_legacy\handler.rs:456 | True | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler) -> u8 |
| 11 | game_ai::plan_legacy::handler::LegacyPlanHandler::update_v2_egowave | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:473 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) |
| 12 | game_ai::plan_legacy::handler::LegacyPlanHandler::v2_obj_restore_safe | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:501 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> bool |
| 13 | game_ai::plan_legacy::handler::LegacyPlanHandler::v2_apply_assign_commit | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:518 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_ai::plan_legacy::types::BigPlan, &mut u8, &mut game_core::DebugFrameData) |
| 14 | game_ai::plan_legacy::handler::LegacyPlanHandler::sanitize_rule_scope | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:571 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, &game_core::OperationData) |
| 15 | game_ai::plan_legacy::handler::LegacyPlanHandler::take_misunderstood_received_chat | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:588 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, game_core::Position, game_core::Chat) -> bool |
| 16 | game_ai::plan_legacy::handler::LegacyPlanHandler::enter_line_backfight_support | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:598 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> bool |
| 17 | game_ai::plan_legacy::handler::LegacyPlanHandler::update_on_dead | pub | game-ai\src\plan_legacy\handler.rs:635 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 18 | game_ai::plan_legacy::handler::LegacyPlanHandler::update | pub | game-ai\src\plan_legacy\handler.rs:685 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData, bool) |
| 19 | game_ai::plan_legacy::handler::LegacyPlanHandler::determine_transition_reason | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:1675 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, &str) -> game_core::PlanTransitionReason |
| 20 | game_ai::plan_legacy::handler::LegacyPlanHandler::force_plan_update | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:1709 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData, bool) |
| 21 | game_ai::plan_legacy::handler::LegacyPlanHandler::v3_assign_anchor | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:1773 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, &game_core::PlayerState, &game_core::OperationData) -> std::option::Option<(u64, u64)> |
| 22 | game_ai::plan_legacy::handler::LegacyPlanHandler::v3_depart_anchor | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:1806 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, &game_core::PlayerState, &game_core::OperationData) -> std::option::Option<(u64, u64)> |
| 23 | game_ai::plan_legacy::handler::LegacyPlanHandler::v3_plan_dest | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:1824 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, &game_core::PlayerState, &game_core::OperationData, &game_ai::plan_legacy::types::BigPlan) -> std::option::Option<(u64, u64)> |
| 24 | game_ai::plan_legacy::handler::LegacyPlanHandler::v3_repair_done | in:game_ai | game-ai\src\plan_legacy\handler.rs:1847 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bool |
| 25 | game_ai::plan_legacy::handler::LegacyPlanHandler::passive_plan | in:game_ai::plan_legacy::handler | game-ai\src\plan_legacy\handler.rs:1855 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> (game_ai::plan_legacy::types::BigPlan, u8) |
| 26 | game_ai::plan_legacy::handler::auction::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::get_small_action | pub | game-ai\src\plan_legacy\handler\auction.rs:8 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_ai::SmallActionPlay, &std::vec::Vec<(usize, game_core::SmallAction), std::alloc::Global>, &mut game_core::DebugFrameData) -> (game_ai::ScoreParameter, i64, game_ai::SmallActionPlay) |
| 27 | game_ai::plan_legacy::handler::chat::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::handle_chat | pub | game-ai\src\plan_legacy\handler\chat.rs:8 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, game_core::Position, game_core::Chat, bool, &mut game_core::DebugFrameData) |
| 28 | game_ai::plan_legacy::handler::chat::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::handle_chat_inner | in:game_ai | game-ai\src\plan_legacy\handler\chat.rs:41 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, game_core::Position, game_core::Chat, bool, &mut game_core::DebugFrameData) |
| 29 | game_ai::plan_legacy::handler::dive_episode::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::v50_track_dive_episode | in:game_ai | game-ai\src\plan_legacy\handler\dive_episode.rs:35 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &game_core::PlayerState, &game_core::OperationData) |
| 30 | game_ai::plan_legacy::handler::dive_episode::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::v50_fold_dive_episode | in:game_ai | game-ai\src\plan_legacy\handler\dive_episode.rs:125 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, usize, bool, u8) |
| 31 | game_ai::plan_legacy::handler::engage::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::v2_response_retreat_stance | in:game_ai | game-ai\src\plan_legacy\handler\engage.rs:13 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) -> game_ai::plan_legacy::old::BattleSubPlanGoal |
| 32 | game_ai::plan_legacy::handler::engage::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::try_engage | in:game_ai | game-ai\src\plan_legacy\handler\engage.rs:40 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, usize, &mut game_core::DebugFrameData) -> std::option::Option<game_ai::plan_legacy::old::BattlePlan> |
| 33 | game_ai::plan_legacy::handler::engage::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::try_engage_dive | in:game_ai | game-ai\src\plan_legacy\handler\engage.rs:109 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, usize, std::option::Option<game_core::TowerType>, &mut game_core::DebugFrameData) -> std::option::Option<game_ai::plan_legacy::old::BattlePlan> |
| 34 | game_ai::plan_legacy::handler::engage::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::handle_solokill | in:game_ai | game-ai\src\plan_legacy\handler\engage.rs:136 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 35 | game_ai::plan_legacy::handler::engage::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::handle_interact_battle | in:game_ai | game-ai\src\plan_legacy\handler\engage.rs:233 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 36 | game_ai::plan_legacy::handler::modes::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::update_single_lane | in:game_ai | game-ai\src\plan_legacy\handler\modes.rs:13 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 37 | game_ai::plan_legacy::handler::modes::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::update_deathmatch | in:game_ai | game-ai\src\plan_legacy\handler\modes.rs:56 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 38 | game_ai::plan_legacy::handler::modes::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::single_lane_initiate | in:game_ai | game-ai\src\plan_legacy\handler\modes.rs:196 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |
| 39 | game_ai::plan_legacy::handler::modes::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::single_try_engage | in:game_ai | game-ai\src\plan_legacy\handler\modes.rs:241 | False | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, usize, &mut game_core::DebugFrameData) -> std::option::Option<game_ai::plan_legacy::old::SinglePlanBattle> |
| 40 | game_ai::plan_legacy::handler::modes::<impl game_ai::plan_legacy::handler::LegacyPlanHandler>::single_handle_solokill | in:game_ai | game-ai\src\plan_legacy\handler\modes.rs:261 | False | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) |

**`open` 5건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | sanitize_rule_scope(m13.ll:11163 · fastcc · (&mut self, &AbstractGameWithCache, &GameContext)) 의 쓰기 표면 — 계약만. m06.ll 에 그 retain 클로저 4개(382·912·1480·2594)가 있어 chats/received_chats/misunderstood/chats_wait 를 rule_scope 로 걸러내는 것으로 보이나 본문 미독(미탐색) | 4 |  |
| 1 | 미탐색 | take_misunderstood_received_chat(m13.ll:12482 · (&mut self, tick, from: Position i32 0..5, &Chat) -> bool) · handle_chat(m13.ll:29383 · (&mut self, version, rnd, player, data, from, &Chat, misunderstood: bool, debug)) · TeamPlan::update(m09.ll:38194) · GoalData::update(m09.ll:4395 · rnd readnone·데이터 &mut 248B) — 전부 계약만(원장 §1 자식 명세 존재) | 4 |  |
| 2 | 미탐색 | L646 unwind 경로(m13.ll:10283~10286): drop_in_place(옛 plan) 이 unwind 하면 새 plan 을 memcpy 한 뒤 재전파 — 정상 경로엔 영향 없음 | 4 |  |
| 3 | 미탐색 | rnd 소비: 본문 0회 확정. 콜리 passive_plan/handle_chat/TeamPlan::update 내부 gen_range 는 계약 밖(미탐색) — PRNG 비트동일 재현 시 이 세 콜리의 호출 순서(L644→L659×n→L665)가 곧 스트림 순서 | 1 |  |
| 4 | 미탐색 | constants 에 stride/크기 리터럴(384·40·32·24)을 실은 이유: 규격상 '적지 않는다' 대상이지만 24 가 mf_swap src 와 리터럴을 공유해 C1 대조 혼동을 막기 위해 meaning 으로 분리 명시 | 4 |  |

**`notes` 1건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | passive_plan(m13.ll:6495 · internal fastcc · (sret (BigPlan,u8) 392B, &mut self, version, &mut rnd, &PlayerState, &OperationData, &mut debug)) 의 선택 로직과 .1(u8) 의 의미 — 계약만(이 배치 범위 밖). update_on_dead 는 .1 을 읽지 않는다 | 4 | 사실 서술 |

<details><summary>`closed` 1건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

| # | 물음 | 닫은 근거 |
|---|---|---|
| 0 | is_passive 의 `assume(tag != 6)`: 태그 6 이 어느 variant 에도 배정되지 않은 이유(DeathMatchBattle 이 untagged 라 자리만 비었는지)는 tcxdict --enum BigPlan 출력만으로 확정(니치 2..=17, idx4 는 암묵) — 판정엔 무관 | 본문에 해소 표기가 있다 |
</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

