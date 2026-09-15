---

### `208` get_input — AI 에이전트 루트(vtable Agent::get_input). 입력 지연 롤→update_state→small_action.get_input(최대 4회, 사이에 update_small_action/폴백/도주 강제)→진단 카운터·StayEvent(디버그)·freeze 감시 갱신 후 (Option<Input>, Vec<TurnEvent>) 반환

| 항목 | 값 |
|---|---|
| id | `AgentVerHamster__get_input` |
| 심볼 | `_RNvMs1_CshdEBA0ozCnw_7game_aiNtB5_15AgentVerHamster9get_input` |
| 소스 | `game-ai\src\lib.rs:843` |
| IR | `m14.ll` 38800~41939행 |
| 경로·가시성 | `game_ai::AgentVerHamster::get_input` · **pub** |
| 계층 | 기타 |
| exe | `e900b0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r17` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&mut game_ai::AgentVerHamster, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> (std::option::Option<game_core::Input>, bumpalo::collections::vec::Vec< game_core::TurnEvent>)
```

**TLS 메모 절(`sig.tls` · 정정 경로 `/specs[208]/sig/tls/<키>`)**

- `name`: 없음(배치 M 범위)
- `role`: -
- `key`: -
- `layout`: -
- `invalidation`: -
- `call_conditions`: 배치 M 범위(38800~40182·41354~41535 중 루트 ≤1004 블록)에 `LocalKey::with`/`call_once` fn-포인터 상수 참조·`llvm.threadlocal.address` 0건. 프로세스 전역 atomic 만 접촉: game_core::simulation::prof::ENABLED(load monotonic) · PHASE_NANOS[26]/PHASE_CALLS[26](atomicrmw add · ENABLED≠0 일 때만) · game_core::simulation::entity::CNT_DM_IDLE_INPUT[input_tag+1](atomicrmw add · 데스매치 idle_spec_tick 일치 시)

<details><summary>인자 5개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | *mut (Option<Input>, bumpalo::Vec<'a, TurnEvent>) — 64B | (배치 N) 내 범위 쓰기 = L1082(m14.ll:41923~41925): +0..+32 ← %47(input · Option<Input>) memcpy 32B · +32..+64 ← %51(turn_event · bumpalo Vec<TurnEvent>) memcpy 32B. None 조기반환 경로(L848 · 배치 M)는 +0 = -1 · +32 = ptr 8(dangling) · +40 = bump · +48..+64 = 0 | 4 |
| 1 | 1 | self | &mut AgentVerHamster (10704B · lib.rs:122) | (배치 N) 내 범위 직접 쓰기 = stay_events 원소 값(push 자체는 L1004=배치 M) · freeze_* 계측 11필드 · trace_escape_* 3필드 · small_action@Trace.abandoned. 전수는 writes | 4 |
| 2 | 2 | rnd | &mut StdRng (320B) | (배치 N) 내 범위 gen_range 호출 사이트 0 · 어느 콜리에도 전달 안 함 | 4 |
| 3 | 3 | player | &PlayerState (2528B) | (배치 N) 내 범위에서는 배치 M 이 로드한 %221(=+0x930 info.team · L914) · %225(=+0x9c0 info.position · L914) 값을 재사용. is_recent_visible 인자로 %3 그대로 전달(L1019) | 4 |
| 4 | 4 | data | &OperationData (24B) {cache@0x0: &AbstractGameWithCache, context@0x8: &GameContext, blackboard@0x10: &[Blackboard;2]} | (배치 N) +0 cache(%53) · +8 context(%446/%482/%1134) · +16 blackboard(%864 · L1018) 전부 읽음 | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
// lib.rs:0~1004 (배치 M)
// 정본 = m14.ll 38800~41939 · 주석본 _next\reach\e900b0.ll · reach.py: 블록 255 전부 살아있음(사장 0) — NA 봉인 없음
// 시그니처: fn get_input(&mut self, rnd:&mut StdRng, player:&PlayerState, data:&OperationData) -> (Option<Input>, bumpalo::Vec<TurnEvent>)

// ── L844 입력 지연 게이트
let game = data.cache.game;                      // %54 data, %56 vtable
let tick = game.tick();                          // vtable+0x28 (호출 1/5)
if tick < self.next_input_tick {                 // self+0x2940 · icmp ult
  // L848  조기반환: (None, Vec::new_in(data.context.pool))
  sret+0 = -1(i64) ; sret+32 = 8(dangling ptr) ; sret+40 = context.pool ; sret+48..64 = 0 (cap,len)
  return;                                        // → %1330 ret(L1083)
}
// L851  입력 지연 롤 — 배치 M 유일의 rnd 직접 소비 사이트(1/1)
let p = &player.info.parameter;                  // player+0x180
let delay = rnd.gen_range(p.input_delay_min() ..= p.input_delay_max()) / 100;   // RangeInclusive<usize> · udiv 100
// L852
self.next_input_tick = game.tick() + delay;      // tick 재호출(2/5) · store self+0x2940
// L854
let mut turn_event: bumpalo::Vec<TurnEvent> = self.update_state(rnd, player, data);   // sret 32B %51 · &mut self 전체(계약: 배치 R/update_state 명세)
// L855
let _t_sai = ProfTimer::new(26);                 // prof::ENABLED(atomic i8)==0 → None(%50+16 = -1) / 아니면 Instant::now, %50 = {26, secs, nanos}
// L856  1차 소액션 입력
let mut input: Option<Input> = self.small_action.get_input(self.version, rnd, player, data, &self.positioning_score, &mut self.debug);
//        인자: (&mut self+0x2858, self+0x2910 load, rnd, player, data, &self+0x1d90, &mut self+0x0) · sret %47 32B

// ── L865  version≥2 이고 입력이 없으면 소액션 갱신 후 재시도
if self.version > 1 && input.is_none() {         // is_none = tag == -1
  // L869
  self.update_small_action(rnd, player, data);   // fastcc · &mut self(계약: 배치 R)
  // L870
  input = self.small_action.get_input(self.version, rnd, player, data, &self.positioning_score, &mut self.debug);   // 2차
  // L876  ★lapse(인지 공백) 중 무입력 → 플랜 폴백
  if self.version > 1 && input.is_none() && self.last_lapse {   // self+0x29ca bool · 세 조건 and(select 체인: version>1 → is_none → last_lapse)
    // L877
    self.plan_system.v3_lapse_noinput_rescues += 1;   // self+0x1b60
    // L878
    self.plan_system.v3_fall_back_to_passive(self.version, rnd, player, data, &mut self.debug /*%1 = self+0x0*/);   // &mut plan_system(self+0x530) · 마지막 인자 = &mut DebugFrameData(tcx 시그니처 · dereferenceable(224)) = self.debug
    // L879
    self.update_small_action(rnd, player, data);
    // L880
    input = self.small_action.get_input(self.version, rnd, player, data, &self.positioning_score, &mut self.debug);   // 3차
    // L888  그래도 없으면 도주 강제 검토
    if input.is_none() {
      // L889
      let team = player.info.team;               // player+0x930 · bounds 2(panic 157)
      if let Some(champ) = data.cache.player_champion[team][player.info.position() as usize] {   // cache+0x1e0 [2][5] · position = player+0x9c0 i32
        // L890
        let (flx, fly, frx, fry) = data.context.map.fountains[team];   // MapDef+0x6d70 + team*32
        // L895~897  분수 안이고 (v1 이거나 아직 풀피 아님) 이면 강제 안 함
        let in_rect = flx <= champ.x && champ.x <= frx && fly <= champ.y && champ.y <= fry;   // Entity+0x660/+0x668
        let force = if in_rect {
          if self.version > 1 { let in_fountain = champ.hp < champ.stat_cached.hp;  /* +0x670 < +0x628 (DI 이름 in_fountain) */  !in_fountain } else { false }
        } else { true };
        if force {
          // L898
          self.plan_system.v3_lapse_move_forces += 1;   // self+0x1b68
          // L899
          self.small_action = SmallActionPlay::RunAway(SmallActionRunAway::new(data, player, 5));   // sret 136B → drop_in_place(old small_action) → memcpy 177B → tag@0x2909 = 3
          // L900
          input = self.small_action.get_input(self.version, rnd, player, data, &self.positioning_score, &mut self.debug);   // 4차
        }
      }
    }
  }
}
// L906
drop(_t_sai);   // Option<ProfTimer>: phase(+16) != -1 이면 PHASE_NANOS[phase] += elapsed_ns(secs*1e9+nanos) · PHASE_CALLS[phase] += 1 (bounds 132 · atomicrmw monotonic)

// ── L909  전투 액션이면 마지막 전투 틱 갱신
let sa_tag = self.small_action.tag();            // self+0x2909 i8 · assume ≠10
if (15..=18).contains(&sa_tag) {                 // Attack/Skill/Skill2/Ult
  // L911
  self.last_combat_tick = game.tick();           // tick 호출(3/5) · self+0x29a0
}

// ── L914  「집(분수)에서 풀피로 서서 안 사는」 진단 (stay_home_full_nobuy)
let team = player.info.team;                     // bounds 2(panic 232)
let champ_opt = data.cache.player_champion[team][player.info.position() as usize];   // %230 · null = None
if let Some(champ) = champ_opt {
  // L915  입력이 제자리인가
  let stays = match input {
    Some(Input::Move{x, y}) => {                 // tag 0 · x@+8 y@+16
      // L917~919
      let (ax, ay) = Game::adjust_position(data.context.map, data.context.setting, x, y);   // {i64,i64} · context+0x20, +0x8
      dist2(champ.pos, (ax, ay)) < 4000001       // dx=|x1-ax| dy=|y1-ay| · dx²+dy² · 2000² +1
    }
    None => true,                                // tag -1 → 바로 L923
    _ => sa_tag == 19,                           // L921  Return/Attack/Skill*/Ult 는 small_action==Stop 일 때만
  };
  // L923
  if stays && !(champ.hp < champ.stat_cached.hp) {   // 풀피
    // L924~925
    let (flx, fly, frx, fry) = data.context.map.fountains[team];
    if flx <= champ.x && champ.x <= frx && fly <= champ.y && champ.y <= fry {
      // L926~927  구매 가능성 — rnd 는 clone 으로 전달(원본 스트림 비소비)
      let can_buy = buy_item(self.version, &mut rnd.clone(), player, game, data.context).is_some()   // Option<usize> {tag,val} · tag==1 → Some · version/game 인자는 IR 에서 poison(콜리 미사용)
                 || upgrade_item(self.version, &mut rnd.clone(), player, game, data.context).is_some();   // Option<(usize,usize)> sret 24B · tag@0 ==0 None (DI 이름 can_buy 는 이 is_none 값에 붙어 있음 — 극성은 분기로: None→계속)
      // L928
      if !can_buy {
        // L929
        self.stay_home_full_nobuy += 1;          // self+0x29a8
        // L930
        self.stay_home_full_nobuy_plan[BigPlan::freeze_plan_cls(&self.plan_system.plan)] += 1;   // self+0x4d0 [usize;6] · plan = self+0xb18
        // L932  (앞에 rnd.clone() 1회 실행되나 결과 미사용 — 콜리 인자 소거)
        let rd: bool = self.plan_system.v3_repair_done(self.version, /*rnd.clone()*/, player, data);   // 실제 IR 인자 (version, team, position i32, cache)
        // L933~935  적 위협 수 (aux closure#1 · count)
        let threat: usize = data.cache.player_champion[1 - team].iter_champions()   // Option<&Entity> 5칸 슬라이스 · None 건너뜀
            .filter(|e| data.blackboard[1 - player.info.team].is_recent_visible(game, player, e)   // 관측 블랙보드 = blackboard[1-team](docs game_core:21) · bounds 2
                     && dist2(e.pos, champ.pos) < 62500000001)   // 250000²+1
            .count();
        // L936~939  키 문자열
        let cut = |s: String| s.chars().take_while(|c| *c != ' ' && *c != '(').collect::<String>();   // closure#0 (aux) · 마스크 (c & 0x1FFFF7)==32
        let key = format!("rd={} lapse={} 적{} | {} | {} | {}", rd, self.last_lapse, threat,
                          self.plan_system.plan.debug_label(),                      // L937 String
                          cut(format!("{:?}", self.plan_system.sub_plan)),         // L938 self+0xc98 · SubPlan::fmt
                          cut(format!("{:?}", self.small_action.get_action())));   // L939 SmallAction::fmt
        // L940
        *self.stay_home_full_nobuy_stack.entry(key).or_insert(0) += 1;   // self+0x2a0 · rustc_entry: Occupied(tag≠-1)→값 포인터 / Vacant→insert_no_grow((key,0)) → 값 = 원소ptr-8 · +1
      }
    }
  }
}

// ── L948  데스매치 idle 입력 통계 (관측 전용 전역)
if game.get_game_mode().tag == 2 /*DeathMatch*/ {   // vtable+0x40 · {i64,ptr}
  // L949
  if let BigPlan::DeathMatchBattle(b) = &self.plan_system.plan {   // tag(self+0xb18) < 2 (untagged 자리 · assume ≠6)
    // L950
    if b.idle_spec_tick == game.tick() {         // self+0xc58 · tick 호출(4/5)
      // L952/961
      CNT_DM_IDLE_INPUT[input.tag + 1].fetch_add(1);   // [7] · None→0 Move→1 … Ult→6
    }
  }
}

// ── L967  입력 기회 카운터
self.input_chances += 1;                         // self+0x2958
// L968
if self.last_lapse { self.lapse_chances += 1; }  // self+0x2998
// L969
let target_dead = self.small_action.is_premise_lost(data);   // bool
// L970
if input.is_none() {
  // L971
  self.noinput_bucket[((self.last_lapse as usize) << 1) | target_dead as usize] += 1;   // self+0x410 [4] · shl 1 = *2
  // L972
  if self.last_lapse {
    // L973
    self.noinput_lapse_action[cls(sa_tag)] += 1;   // self+0x500 [6] · cls = {Trace:0, RunAway|Recall|AroundRunAway:1, Around|AroundHide|AroundRegion|Positioning|AroundPosition|AroundPositionBush|AroundBush:2, Attack|Skill|Skill2|Ult:3, LaneMinionPosition:4, Stop:5}
    // L974
    if let Some(champ) = champ_opt /*재로드 %229*/ {
      // L975~977
      let (flx, fly, frx, fry) = data.context.map.fountains[team];
      let band = if in_rect(champ) { 0 }
                 else { match data.cache.nexus[team] {   // cache+0x170 [2] · closure#2(977:68)
                          Some(n) => if dist2(champ.pos, n.pos) < 67600000001 { 1 } else { 2 },   // 260000²+1
                          None => 2 } };
      self.noinput_lapse_pos[band] += 1;         // self+0x29b0 [3]
    }
  }
}

// ── L983  StayEvent 기록 (디버그 컨텍스트 전용)
if champ_opt.is_some() && data.context.debug {  // context+0x3b bool · and(%231, debug)
  let champ = champ_opt.unwrap();                // L984 재로드
  // L985~993
  let (stays2, kind): (bool, u8) = match input {
    Some(Input::Move{x,y}) => { let (ax,ay) = Game::adjust_position(map, setting, x, y);   // L988~989
                                let s = dist2(champ.pos,(ax,ay)) < 4000001; (s, if s {1} else {2}) }   // L990 · freeze 후 비교
    None => (true, 0),                           // %540 → %579 직행 · kind 0
    _ => (false, 2),
  };
  // L993~994
  if stays2 || sa_tag == 19 /*Stop*/ {           // kind 는 위 값 유지
    // L995~996
    let (flx, fly, frx, fry) = data.context.map.fountains[team];
    let (can_buy, pos_band): (u8, u8) = if in_rect(champ) {
      // L1002~1003  분수 안: 구매 가능성(둘 다 rnd.clone())
      let cb = if buy_item(version, &mut rnd.clone(), player, game, ctx).is_some() { 1 } else { upgrade_item(version, &mut rnd.clone(), player, game, ctx).tag as u8 /*0 None/1 Some*/ };
      (cb, 0)
    } else {
      // L998~999
      (0, match data.cache.nexus[team] { Some(n) => if dist2(champ.pos,n.pos) < 67600000001 {1} else {2}, None => 2 })
    };
    // L1004  push (필드값 대부분은 배치 N 이 1005~1043 에서 계산: tick=game.tick()(5/5 · L1005) · goal · plan_label/plan_detail/sub_plan/action 문자열 · target_kind(&str: 소멸/챔피언/에픽/세르펜/정글몹/미니언/넥서스/타워 중 하나 · phi %1079/%1080) · hp_pct · near_enemy · near_ally · since_combat · in_lapse)
    self.stay_events.push(StayEvent{ tick, team, pos:(champ.x, champ.y /*배치 N %853*/), position: player.info.position(), kind, pos_band, can_buy: can_buy!=0, /* + 배치 N 값들 */ });
    //   → %1078: %24(208B) 조립 후 len==cap 이면 grow_one(self+0x1d78) → memcpy(ptr+len*208) → len+1 (self+0x1d88)
    // → 배치 N(줄 1048) %1129
  } else {
    // → 배치 N(줄 1048) %1130
  }
} else {
  // → 배치 N(줄 1048) %1129 (champ None 이면 %480 → 배치 N(줄 1080) %1133 로 직행: L1048~1075 freeze 감시를 건너뜀)
}
// 이후(배치 N): L1048~1075 freeze 감시(freeze_since/anchor/eps/plan/pos/field_action · freeze_fired) · L1080 trace_escape_* · L1082 sret = (input, turn_event) · L1083 ret

// ── 언와인딩(%97 cleanuppad): 예외 시 Option<ProfTimer> drop(phi %98=true 인 경로만) → Vec<TurnEvent> drop(%70) → caller
// ── rnd 사이트 요약(배치 M): 직접 gen_range 1회(L851, 조기반환 아니면 항상) → 이후 순서: update_state → get_input#1 → [update_small_action → get_input#2 → [v3_fall_back_to_passive → update_small_action → get_input#3 → [get_input#4]]] · clone 5회(L926, L927, L932(미사용), L1002, L1003)는 스트림 비소비

// lib.rs:1005~1083 (배치 N)
// 진입: 배치 M 의 L983 `if champ.is_some() && data.context.debug(+0x3b)` 가 참이고 L985~1004 (kind/pos_band/can_buy 계산 · %580/%631/%632) 를 지나 `self.stay_events.push(StayEvent { ... })`(L1004 · 배치 M) 의 필드식이 내 범위 1005~1043 이다. 배치 M 의 %444(L983 거짓) → L1048 로 직행 · %480(L974 champ None) → L1080 로 직행 · %573(L985 input Some(Move 이외) 경로) → L1049 로 직행.

// ─── [A] StayEvent 필드 (루트 1005~1043 · 값만 내 범위, push 는 배치 M L1004)
tick     = game.tick()                                   // L1005 · vtable+0x28 · %634
plan_label  = self.plan_system.plan.debug_label()        // L1009 · String %23 · 계약: (&BigPlan) -> String(sret 24B)
plan_detail = self.plan_system.plan.get_name()           // L1010 · String %22 · 계약: (&BigPlan) -> String
sub_plan = short(format!("{:?}", self.plan_system.sub_plan))   // L1011 · closure#5(lib.rs:1000) 인라인:
   // short(s) = s[..idx].to_string() where idx = s.chars() 순회 중 첫 c==' ' || c=='(' 의 바이트 오프셋(없으면 s.len())  — m14.ll:40242~40551 (UTF-8 디코더 인라인 · `(c & 0x1FFFF7)==32` · 새 String 할당 try_allocate_in(len,1,1) → memcpy → 원본 drop). 즉 SubPlan variant 이름만 남긴다
action   = format!("{:?}", self.small_action.get_action())       // L1012 · get_action(small_action.rs:308 인라인) 태그별: RunAway(3)/Recall(4)/AroundRunAway(8) → SmallAction::RunAway(0) · Around(5)/AroundHide(6)/LaneMinionPosition(13) → Around(2){target=+0x8} · AroundRegion(7) → AroundPosition(3){+0x10,+0x18} · Positioning(9) → Positioning(1){+0x8,+0x10} · AroundPosition(암묵) → AroundPosition(3){+0x30,+0x38 around_input.target} · AroundPositionBush(11) → AroundPosition(3){+0x8,+0x10} · AroundBush(12) → AroundPosition(3){+0x18,+0x20} · Trace(14) → Trace(4){+0x60} · Attack(15)→Attack(6){+0x8} · Skill(16)→Skill(7) · Skill2(17)→Skill2(8) · Ult(18)→Ult(9) · Stop(19)→Stop(10)
goal     = self.small_action.target_position()          // L1013 · small_action.rs:229 인라인 → Option<(u64,u64)>: RunAway/Positioning/AroundPosition/AroundPositionBush → (+0x8,+0x10) · Recall → (+0x50,+0x58) · Around/AroundHide/LaneMinionPosition → (+0x10,+0x18) · AroundBush → (+0x18,+0x20) · Trace → (+0x68,+0x70) · AroundRegion/AroundRunAway/Attack/Skill/Skill2/Ult/Stop → None
pos      = (champ.x /*L996 배치 M %589*/, champ.y /*L1014 %853*/)
hp_pct   = champ.hp * 100 / max(champ.stat_cached.hp, 1)          // L1016 · Entity+0x670 · +0x628
enemy_row = data.cache.player_champion[1 - team]                    // L1017 · %862 · [Option<&Entity>;5]
bb       = &data.blackboard[1 - team]                              // L1018 · %864/%865 · ★인덱스 = 적팀
near_enemy = enemy_row.iter().filter(|e| e.is_some() && bb.is_recent_visible(game, player, e) && dist²(e, champ) < 150000²+1).count()   // L1019 · 5회 루프(m14.ll:40891~40973) · dist² = |dx|²+|dy|² (utils 2158 인라인)
near_ally  = ally_row(=player_champion[team] %228).iter().filter(|e| e.is_some() && e.id != champ.id && dist²(e, champ) < 150000²+1).count()   // L1021 · 5회 완전 언롤(41009~41341) · ★가시성 검사 없음 · 자기 자신 제외
in_lapse = self.last_lapse                                          // L1023 · +0x29ca
since_combat = if self.last_combat_tick == usize::MAX { usize::MAX } else { game.tick().saturating_sub(self.last_combat_tick) }   // L1024~1025 · tick 재호출(%1058)
target_kind: &str = match self.small_action.get_action() /*L1026 재평가*/ {           // L1026~1040
   Around(id)|Trace(id)|Attack(id)|Skill(id)|Skill2(id)|Ult(id) => {              // L1027 · Trace → +0x60, 나머지 → +0x8
      match game.get_entity_by_id(id) /*L1032 · vtable+0x1f0*/ {
         None => "소멸",                                                       // L1032 null → anon.158
         Some(e) => match e.ty /*+0x68*/ {                                         // L1034
            Champion(13) => "챔피언"(anon.159) · Epic(5) => "에픽"(.160 · L1035) · Serpen(6) => "세르펜"(.161 · L1036)
            Jungle(4) if e.ty.info.camp_type.0 /*+0x98*/ < 2 => "정글몹"(.162 · L1037)
            Tower(2) => "타워"(.165 · L1038) · Nexus(3) => "넥서스"(.164 · L1039)
            _ (Minion/Ghoul/…/Jungle camp_type.0>=2) => "미니언"(.163 · L1040)
         } } }
   _ => ""   // ptr 1 · len 0 (대상 없는 액션)
}
// → StayEvent 208B 조립·push (배치 M L1004 · m14.ll:41412~41532) → L994 로 복귀 후 L1048

// ─── [B] 정지(freeze) 에피소드 계측 (루트 1048~1075)
if let Some(champ) = data.cache.player_champion[team][position] /*L1048 · %231 배치 M L914*/ {
   tick  = game.tick()                                                          // L1049 · %1132
   moved = utils::distance(champ.x, champ.y, self.freeze_anchor.0, self.freeze_anchor.1) > 8000   // L1050
   if moved || input.is_some() /*%442 != -1 · L633 인라인*/ || self.freeze_since == 0 {   // L1051
      self.freeze_since = tick; self.freeze_anchor = (champ.x, champ.y); self.freeze_fired = false   // L1052~1054
   } else if self.freeze_fired {                                                 // L1055
      self.freeze_ticks += 1                                                     // L1056 · 정지 누적 틱
   } else if tick.saturating_sub(self.freeze_since) >= data.context.setting.tick_per_second /*L1057 · 1초 게이트 · IR 은 `< tps` 이면 skip*/ {
      self.freeze_fired = true                                                   // L1058
      cls = small_action_cls(&self.small_action)                                 // L1059 · lib.rs:481 인라인: Trace→0 · RunAway/Recall/AroundRunAway→1 · Around/AroundHide/AroundRegion/Positioning/AroundPosition/AroundPositionBush/AroundBush→2 · Attack/Skill/Skill2/Ult→3 · LaneMinionPosition→4 · Stop→5
      self.freeze_eps[cls] += 1                                                  // L1060
      plan_cls = self.plan_system.plan.freeze_plan_cls()                         // L1061 · types.rs:121 인라인: SinglePlanBattle(idx3)/DeathMatchBattle(4)/Battle(7)→0 · PassiveLine(1)/SinglePlanLine(2)→1 · PassiveJungle(5)→2 · ForcePassive(0)/ActiveRecall(6)→3 · LineGanker(8)/LineGankCover(9)→4 · 그 외(Epic/Serpen 계열 idx10~15)→5
      self.freeze_plan[plan_cls] += 1                                            // L1061
      (flx, fly, frx, fry) = data.context.map.fountains[team]                    // L1063 · MapDef+0x6d70 + team*32
      pos_cls = if flx <= champ.x && champ.x <= frx && fly <= champ.y && champ.y <= fry { 0 /*우물 안*/ }   // L1064
                else { match data.cache.nexus[team] /*L1066*/ { None => 2, Some(n) => if dist²(champ, n) > 260000² { 2 /*필드*/ } else { 1 /*본진 근처*/ } } }   // L1066~1067
      self.freeze_pos[pos_cls] += 1                                              // L1072
      if pos_cls == 2 { self.freeze_field_action[cls] += 1 }                     // L1073 (넥서스 None 경로도 포함 · m14.ll:41882→41884)
      if self.last_lapse { self.freeze_eps_lapse += 1 }                          // L1074
      if premise_lost /*%436 · 배치 M is_premise_lost*/ { self.freeze_eps_dead_target += 1 }   // L1075
   }
   // else (1초 미만) 아무 것도 안 함
}

// ─── [C] self.update_trace_escape_abandon(data) (L1080 · lib.rs:1088~1108 인라인 · 진입 %1133)
tick = data.cache.game.tick()                                                    // L1089 · %1139 (%53 재로드)
abandon_threshold = data.context.setting.tick_per_second * 2                     // L1090 · shl 1
if let Trace(t) = &mut self.small_action /*tag==14 · L1091*/ {
   tid = t.target /*+0x60 · L1092*/
   if t.last_escape == Some(true) /*+0x8d & 1 · L1093*/ {
      if self.trace_escape_target == tid && tick >= self.trace_escape_last_tick {            // L1094
         self.trace_escape_ticks = (tick - self.trace_escape_last_tick) + self.trace_escape_ticks   // L1095
      } else { self.trace_escape_target = tid; self.trace_escape_ticks = 0 }                  // L1097
      self.trace_escape_last_tick = tick                                                     // L1100
      if !(self.trace_escape_ticks < abandon_threshold) && !t.abandoned { t.abandoned = true }   // L1101~1102 · +0x92 ← 1
   } else if self.trace_escape_target == tid {                                               // L1104
      dec = tick.saturating_sub(self.trace_escape_last_tick)                                 // L1106
      self.trace_escape_ticks = self.trace_escape_ticks.saturating_sub(dec)                  // L1107
      self.trace_escape_last_tick = tick                                                     // L1108
   }
}
// ─── [D] 반환 (L1082~1083 · %1328)
ret.1(+32..+64) = turn_event(%51) ; ret.0(+0..+32) = input(%47) ; ret void   // 32B memcpy ×2 · %1330
// 언와인드: %97 → %1332 drop Option<ProfTimer>(%50) → %70 drop bumpalo Vec<TurnEvent>(%51) · 문자열 %23/%22/%21/%17 은 각 cleanuppad 에서 drop(L1043 루트)
```

**`mem` 메모리 접근 116건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | AgentVerHamster | 0x2940 | next_input_tick | r | L844 tick < next_input_tick 이면 조기반환 | 4 | OK |  |
| 1 | AgentVerHamster | 0x2910 | version | r | 매 사용마다 재로드(%90). L856/870/880/900 get_input 인자 · L865/876/896 `>1` 게이트 · L926/927/932/1002/1003 콜리 인자(buy/upgrade 는 poison 으로 소거됨) | 4 | OK |  |
| 2 | AgentVerHamster | 0x29ca | last_lapse | r | bool. L876 폴백 조건 · L936 키 문자열 · L968 lapse_chances 게이트 · L971 버킷 비트1 · L972 | 4 | OK |  |
| 3 | AgentVerHamster | 0x2909 | small_action@tag (=0x2858+0xb1) | r | SmallActionPlay 니치 태그 1B(untagged AroundPosition · niche_start 3). L909 전투 액션 판정(15..18) · L921/993 Stop(19) · L973 분류 | 4 | OK |  |
| 4 | AgentVerHamster | 0x2858 | small_action (184B) | r | &mut 로 SmallActionPlay::get_input ×4 · get_action(L939) · is_premise_lost(L969) · L899 drop 후 RunAway 로 통째 교체 | 4 | OK |  |
| 5 | AgentVerHamster | 0x0 | debug (DebugFrameData 224B) | r | SmallActionPlay::get_input 의 마지막 인자(&mut DebugFrameData) 로 %1 그대로 전달(=self+0) | 4 | OK |  |
| 6 | AgentVerHamster | 0x1d90 | positioning_score | r | SmallActionPlay::get_input 의 &PositioningScoreData 인자 | 4 | OK |  |
| 7 | AgentVerHamster | 0x530 | plan_system (LegacyPlanHandler 6168B) | r | &mut 로 v3_fall_back_to_passive(L878) · & 로 v3_repair_done(L932 — ArgumentPromotion 으로 실제 인자는 (version, team, position, cache)) | 4 | OK |  |
| 8 | AgentVerHamster | 0xb18 | plan_system.plan@tag (BigPlan 니치 8B) | r | L930 freeze_plan_cls(&plan) · L937 debug_label(&plan) · L949 tag<2(=untagged DeathMatchBattle 의 내부 bool 0/1) 판정, assume ≠6 | 4 | OK |  |
| 9 | AgentVerHamster | 0xc58 | plan_system.plan@DeathMatchBattle.0.idle_spec_tick | r | L950 == tick 이면 CNT_DM_IDLE_INPUT 기록 | 4 | OK |  |
| 10 | AgentVerHamster | 0xc98 | plan_system.sub_plan | r | L938 format!("{:?}") 로 Debug 출력(SubPlan::fmt) → closure#0 절단 → 키 문자열 | 4 | OK |  |
| 11 | AgentVerHamster | 0x2a0 | stay_home_full_nobuy_stack (HashMap<String,usize>) | r | L940 entry(key).or_insert(0) — hashbrown rustc_entry/insert_no_grow | 4 | OK |  |
| 12 | AgentVerHamster | 0x1d78 | stay_events (Vec<StayEvent>) cap@0x1d78 ptr@0x1d80 len@0x1d88 | r | L1004 push 경로: len==cap → grow_one | 4 | OK |  |
| 13 | OperationData | 0x0 | cache (&AbstractGameWithCache) | r | %53 | 4 | OK |  |
| 14 | OperationData | 0x8 | context (&GameContext) | r |  | 4 | OK |  |
| 15 | OperationData | 0x10 | blackboard (&[Blackboard;2]) | r | L934 count 클로저 캡처 → blackboard[1-team].is_recent_visible | 4 | OK |  |
| 16 | AbstractGameWithCache | 0x0 | game.data_ptr | r | %54 · dyn AbstractGame 데이터 | 4 | OK |  |
| 17 | AbstractGameWithCache | 0x8 | game.vtable_ptr | r | %56 | 4 | OK |  |
| 18 | vtable(AbstractGame) | 0x28 | tick (divtable 0x28) | r | L844 · L852 · L911 · L950 · L1005 — 5회 호출(같은 vtable 슬롯 %58) | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 19 | vtable(AbstractGame) | 0x40 | get_game_mode (divtable 0x40) → GameMode{tag i64, ptr} | r | L948 tag==2 DeathMatch | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 20 | AbstractGameWithCache | 0x1e0 | player_champion[team][position] (Option<&Entity> · [5]×2 · 니치 null) | r | L889 · L914 · L974/984 재로드 | 4 | OK |  |
| 21 | AbstractGameWithCache | 0x170 | nexus[team] (Option<&Entity>) | r | L977 · L998 자기 팀 넥서스 거리 밴드 \| (배치 N) L1066 gep 368 + team*8 (m14.ll:41825~41827) · null=None | 4 | OK |  |
| 22 | GameContext | 0x0 | pool (&Bump) | r | L848 빈 Vec 의 할당자 | 4 | OK |  |
| 23 | GameContext | 0x8 | setting (&GameSetting) | r | L918/989 Game::adjust_position 인자 \| (배치 N) L1057 · L1090 | 4 | OK |  |
| 24 | GameContext | 0x20 | map (&MapDef) | r | L890/924/975/995 fountains · L918/989 adjust_position \| (배치 N) L1063 (m14.ll:41799) | 4 | OK |  |
| 25 | GameContext | 0x3b | debug (bool) | r | L983 StayEvent 기록 게이트 | 4 | OK |  |
| 26 | MapDef | 0x6d70 | fountains[team] ((u64,u64,u64,u64) stride 32: flx@+0 fly@+8 frx@+16 fry@+24) | r | gepS 인덱스 team · 4회 재로드 | 4 | OK |  |
| 27 | PlayerState | 0x180 | info.parameter (AthleteParameter) | r | L851 input_delay_min/max(&self) | 4 | OK |  |
| 28 | PlayerState | 0x930 | info.team | r | L889/914 · 클로저 안에서도 재로드 | 4 | OK |  |
| 29 | PlayerState | 0x9c0 | info.position@tag (i32) | r | L889/914 GamePlayer::position()(player.rs:581 인라인) → player_champion 2차 인덱스 · v3_repair_done 인자 · StayEvent.position | 4 | OK |  |
| 30 | Entity | 0x660 | x | r | champ · nexus · 적 챔피언(aux) \| (배치 N) champ.x(%588/%589 · 배치 M L996 로드 · StayEvent.pos.0) · L1019/L1021 상대 x(gep 1632) · L1050 champ.x(41646) · L1064 우물 사각 · L1067 넥서스 x | 4 | OK |  |
| 31 | Entity | 0x668 | y | r | L1014 champ.y(%853 · StayEvent.pos.1) · L1019/L1021 상대 y(gep 1640) · L1050 · L1064 · L1067 | 4 | OK |  |
| 32 | Entity | 0x670 | hp | r | L896/923 hp < stat_cached.hp (풀피 아님) \| (배치 N) L1016 hp_pct 분자 (hp*100) | 4 | OK |  |
| 33 | Entity | 0x628 | stat_cached.hp (최대 HP) | r |  | 4 | OK |  |
| 34 | global | 0x0 | game_core::simulation::prof::ENABLED (atomic i8) | r | L855 ProfTimer::new(phase 26) 게이트 | 4 | 오귀속(그 오프셋에 필드가 없다(패딩/관통불가)) |  |
| 35 | Option<ProfTimer> (_t_sai 로컬 %50 24B) | 0x10 | phase i32 니치: -1 = None | r | layout {phase_idx? i64 26 @+0, Instant.secs @+8, Instant.nanos i32 @+16} — +16 이 -1 이면 비활성 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 36 | AgentVerHamster | 0xb18 | plan_system.plan (BigPlan 태그·본체) | r | L1009 debug_label(&plan) · L1010 get_name(&plan) 인자(m14.ll:40185) · L1061 freeze_plan_cls 인라인: 태그 로드 후 니치 환산(m14.ll:41755~41762 · assume tag≠6 · idx = tag>1 ? tag-2 : 4(DeathMatchBattle 암묵)) | 4 | OK |  |
| 37 | AgentVerHamster | 0xc98 | plan_system.sub_plan (SubPlan) | r | L1011 format!("{:?}", sub_plan) 의 Debug::fmt 인자(m14.ll:40202~40215) | 4 | OK |  |
| 38 | AgentVerHamster | 0x28b1 | small_action@tag (1B 니치) | r | %210(배치 M L909 gep) 재로드 4회: L1012 get_action(m14.ll:40567) · L1013 target_position(40765) · L1026 get_action 재평가(41357) · L1059 small_action_cls(41702) · L1091 ==14(Trace) (41568). 전부 assume tag≠10 · idx = tag>2 ? tag-3 : 7(AroundPosition 암묵) | 4 | 부분일치(첫 성분(컨테이너)만 맞고 **마지막 성분이 다르다** — 사람이 봐야 ) |  |
| 39 | AgentVerHamster | 0x2860 | small_action.<variant>+0x8 (Around/AroundHide/LaneMinionPosition/Attack/Skill/Skill2/Ult.target · RunAway/Positioning/AroundPositionBush.goal_x·target_x) | r | L1012 get_action 페이로드(m14.ll:40599~40729 · gep 10336) · L1013 target_position(x) · L1027 대상 id(41400 phi 10336) | 4 | 부분일치(첫 성분(컨테이너)만 맞고 **마지막 성분이 다르다** — 사람이 봐야 ) |  |
| 40 | AgentVerHamster | 0x2868 | small_action.<variant>+0x10 (Around·AroundHide·LaneMinionPosition.goal_x / AroundRegion.goal_x / RunAway·Positioning.goal_y / AroundPositionBush.target_y) | r | L1012(gep 10344) · L1013(+16) | 4 | 부분일치(첫 성분(컨테이너)만 맞고 **마지막 성분이 다르다** — 사람이 봐야 ) |  |
| 41 | AgentVerHamster | 0x2870 | small_action.<variant>+0x18 (Around류.goal_y / AroundRegion.goal_y / AroundBush.target_x) | r | L1012(gep 10352) · L1013(+24) | 4 | 부분일치(첫 성분(컨테이너)만 맞고 **마지막 성분이 다르다** — 사람이 봐야 ) |  |
| 42 | AgentVerHamster | 0x2878 | small_action@AroundBush.target_y (+0x20) | r | L1012(gep 10360) · L1013(+32) | 4 | OK |  |
| 43 | AgentVerHamster | 0x2888 | small_action@AroundPosition.around_input.target_x (+0x30) | r | L1012 get_action AroundPosition 팔(gep 10376 · m14.ll:40643) | 4 | OK |  |
| 44 | AgentVerHamster | 0x2890 | small_action@AroundPosition.around_input.target_y (+0x38) | r | L1012(gep 10384) | 4 | OK |  |
| 45 | AgentVerHamster | 0x28a8 | small_action@Recall.goal_x (+0x50) | r | L1013 target_position Recall 팔(+80 · m14.ll:40797) | 4 | OK |  |
| 46 | AgentVerHamster | 0x28b0 | small_action@Recall.goal_y (+0x58) | r | L1013(+88) | 4 | OK |  |
| 47 | AgentVerHamster | 0x28b8 | small_action@Trace.target (+0x60) | r | L1012 get_action Trace 팔(gep 10424 · m14.ll:40691) · L1027 대상 id(41400 phi 10424) · L1092 tid(41579) | 4 | OK |  |
| 48 | AgentVerHamster | 0x28c0 | small_action@Trace.goal_x (+0x68) | r | L1013 Trace 팔(+104) | 4 | OK |  |
| 49 | AgentVerHamster | 0x28c8 | small_action@Trace.goal_y (+0x70) | r | L1013 Trace 팔(+112) | 4 | OK |  |
| 50 | AgentVerHamster | 0x28ea | small_action@Trace.abandoned (bool) | r | L1101 `\|\| abandoned` (m14.ll:41633~41636) — 이미 true 면 재설정 생략 | 4 | OK |  |
| 51 | AgentVerHamster | 0x28ed | small_action@Trace.last_escape (Option<bool> 니치 1B) | r | L1093 `& 1` (m14.ll:41583~41585 · 원본 줄 2439<424): 하위 비트 0 = Some(false) 또는 None(니치값 2 는 &1==0) → 도주 미적용 경로 / 1 = Some(true) → 도주 적용 경로 | 4 | OK |  |
| 52 | AgentVerHamster | 0x29a0 | last_combat_tick | r | L1024 == -1(usize::MAX) 검사(m14.ll:41345~41347) · L1025 tick.saturating_sub 피감수(41388) | 4 | OK |  |
| 53 | AgentVerHamster | 0x29ca | last_lapse (bool) | r | %425(배치 M L968 gep) 재로드: L1023 StayEvent.in_lapse(m14.ll:41344) · L1074 freeze_eps_lapse 게이트(41901) | 4 | OK |  |
| 54 | AgentVerHamster | 0x430 | freeze_anchor.0 (x) | r | L1050 distance 인자(m14.ll:41650) | 4 | OK |  |
| 55 | AgentVerHamster | 0x438 | freeze_anchor.1 (y) | r | L1050 distance 인자(m14.ll:41652) | 4 | OK |  |
| 56 | AgentVerHamster | 0x2960 | freeze_since | r | L1051 ==0 (m14.ll:41664) · L1057 tick.saturating_sub(freeze_since)(41685) | 4 | OK |  |
| 57 | AgentVerHamster | 0x29cb | freeze_fired (bool) | r | L1055 분기(m14.ll:41678~41681) | 4 | OK |  |
| 58 | AgentVerHamster | 0x2978 | freeze_ticks | r | L1056 +=1 의 로드 | 4 | OK |  |
| 59 | AgentVerHamster | 0x440 | freeze_eps[cls] ([usize;6]) | r | L1060 +=1 의 로드(gep 1088 + cls*8) | 4 | OK |  |
| 60 | AgentVerHamster | 0x470 | freeze_plan[plan_cls] ([usize;6]) | r | L1061 +=1 의 로드(gep 1136) | 4 | OK |  |
| 61 | AgentVerHamster | 0x2980 | freeze_pos[pos_cls] ([usize;3]) | r | L1072 +=1 의 로드(gep 10624 / 10640=[2]) | 4 | OK |  |
| 62 | AgentVerHamster | 0x4a0 | freeze_field_action[cls] ([usize;6]) | r | L1073 +=1 의 로드(gep 1184) | 4 | OK |  |
| 63 | AgentVerHamster | 0x2968 | freeze_eps_lapse | r | L1074 +=1 의 로드(gep 10600) | 4 | OK |  |
| 64 | AgentVerHamster | 0x2970 | freeze_eps_dead_target | r | L1075 +=1 의 로드(gep 10608) | 4 | OK |  |
| 65 | AgentVerHamster | 0x2928 | trace_escape_target | r | L1080(update_trace_escape_abandon 인라인 · lib.rs:1094/1104) == tid (m14.ll:41587~41589) | 4 | OK |  |
| 66 | AgentVerHamster | 0x2930 | trace_escape_ticks | r | lib.rs:1095 누적 피가수(m14.ll:41624) · 1107 saturating_sub 피감수(41611) | 4 | OK |  |
| 67 | AgentVerHamster | 0x2938 | trace_escape_last_tick | r | lib.rs:1094 tick>=last(m14.ll:41597) · 1095 tick-last · 1106 saturating_sub(41605) | 4 | OK |  |
| 68 | AgentVerHamster | 0x1d78 | stay_events (Vec<StayEvent>) — buf.ptr@+0x1d80 · len@+0x1d88 | r | push 인라인(루트 L1004 = 배치 M · m14.ll:41464~41532): cap(+0x1d78)==len → grow_one · ptr[len] ← 208B · len+=1. 원소 값은 내 범위(logic 참조) | 4 | OK |  |
| 69 | OperationData | 0x0 | cache (&AbstractGameWithCache · %53) | r | cache+0 game data ptr(%54) · cache+8 vtable(%56) · 배치 M 로드 재사용 | 4 | OK |  |
| 70 | OperationData | 0x8 | context (&GameContext · %446/%1134) | r | L1057·L1090 setting · L1063 map | 4 | OK |  |
| 71 | OperationData | 0x10 | blackboard (&[Blackboard;2] · %864) | r | L1018 로드(m14.ll:40834) → L1019 is_recent_visible self = &blackboard[1 - team] | 4 | OK |  |
| 72 | AbstractGameWithCache | 0x1e0 | player_champion[t][p] ([[Option<&Entity>;5];2]) | r | %227(배치 M L914 gep 480): L1017 적팀 행 %862 = [5 x ptr][1-team] · L1021 아군 행 %228=[team] · champ %541=[team][position](배치 M L984 로드) · L1048 재로드 %1131 | 4 | OK |  |
| 73 | GameSetting | 0x12f8 | tick_per_second | r | L1057 정지 게이트(1초 · m14.ll:41688~41690) · L1090 abandon_threshold = tps<<1 (41564~41566) | 4 | OK |  |
| 74 | MapDef | 0x6d70 | fountains[team] ((u64,u64,u64,u64) · stride 32) | r | L1063 gep 28016 + team*32 → (flx, fly, frx, fry) (m14.ll:41802~41814) | 4 | OK |  |
| 75 | Entity | 0x5c0 | id | r | L1021 champ.id(%904 · m14.ll:41009) vs 아군 각 id(gep 1472) ≠ 비교 | 4 | OK |  |
| 76 | Entity | 0x628 | stat_cached.hp (최대 hp) | r | L1016 hp_pct 분모 umax(·,1) | 4 | OK |  |
| 77 | Entity | 0x68 | ty@tag (EntityType 8B 직접) | r | L1034 switch(m14.ll:41491~41500): 13 Champion · 5 Epic · 6 Serpen · 4 Jungle · 2 Tower · 3 Nexus · 그 외 default | 4 | OK |  |
| 78 | Entity | 0x98 | ty@Jungle.info.camp_type.0 (usize) | r | L1037 `< 2` 가드(m14.ll:41509~41511) | 4 | OK |  |
| 79 | AbstractGame(vtable) | 0x28 | tick (slot · divtable) | r | %58 로 L1005(%634) · L1025(%1058) · L1049(%1132) · L1089(%1139) 4회 호출 | 4 | 오귀속(사전은 다른 필드를 준다) |  |
| 80 | AbstractGame(vtable) | 0x1f0 | get_entity_by_id (slot · divtable) | r | L1032 (m14.ll:41404~41407) → Option<&Entity>(null=None) | 4 | 오귀속(사전은 다른 필드를 준다) |  |
| 81 | AgentVerHamster | 0x2940 | next_input_tick | w | L852 · m14.ll:38936 · 조기반환이 아닌 모든 경로에서 1회 | 4 | OK | tick + delay (delay = rnd.gen_range(input_delay_min..=input_delay_max)/100) |
| 82 | AgentVerHamster | 0x1b60 | plan_system.v3_lapse_noinput_rescues | w | L877 · 38940~39074 · version>1 && input None(2차) && last_lapse | 4 | OK | +1 |
| 83 | AgentVerHamster | 0x1b68 | plan_system.v3_lapse_move_forces | w | L898 · 39167 · 도주 강제 경로 | 4 | OK | +1 |
| 84 | AgentVerHamster | 0x2858 | small_action (184B 통째) | w | L899 · 39185~39188 · 도주 강제 경로. 또 &mut 로 SmallActionPlay::get_input ×4 · update_small_action ×2 (콜리 계약) | 4 | OK | drop_in_place(old); memcpy 177B(SmallActionRunAway::new(data, player, 5) 136B 페이로드 + 미초기화 41B); tag@0x2909 = 3 (RunAway) |
| 85 | AgentVerHamster | 0x2909 | small_action@tag | w | L899 · 39188 | 4 | OK | 3 (RunAway) |
| 86 | AgentVerHamster | 0x29a0 | last_combat_tick | w | L911 · 39210 · small_action tag ∈ 15..18(Attack/Skill/Skill2/Ult) | 4 | OK | tick() |
| 87 | AgentVerHamster | 0x29a8 | stay_home_full_nobuy | w | L929 · 39425 · 정지(stays)&&풀피&&분수 안&&구매 불가 | 4 | OK | +1 |
| 88 | AgentVerHamster | 0x4d0 | stay_home_full_nobuy_plan[freeze_plan_cls(&plan)] | w | L930 · 39433 · [usize;6] | 4 | 오귀속(사전은 다른 필드를 준다) | +1 |
| 89 | AgentVerHamster | 0x2a0 | stay_home_full_nobuy_stack[key] | w | L936~940 · 39627~39707 · HashMap 삽입(힙 할당 가능 · insert_no_grow · 값 슬롯은 (String,usize) 원소 +24 = entry 포인터 -8) | 4 | OK | *entry(key).or_insert(0) += 1 (key = format!("rd={} lapse={} 적{} \| {} \| {} \| {}", rd, last_lapse, threat, plan.debug_label(), cut(format!("{:?}",sub_plan)), cut(format!("{:?}",small_action.get_action())))) |
| 90 | AgentVerHamster | 0x2958 | input_chances | w | L967 · 39736 · 조기반환 아니면 무조건 | 4 | OK | +1 |
| 91 | AgentVerHamster | 0x2998 | lapse_chances | w | L968 · 39764 · last_lapse 일 때 | 4 | OK | +1 |
| 92 | AgentVerHamster | 0x410 | noinput_bucket[(last_lapse as u8)<<1 \| target_dead as u8] | w | L971 · 39794 · input None 일 때 · [usize;4] | 4 | 오귀속(사전은 다른 필드를 준다) | +1 |
| 93 | AgentVerHamster | 0x500 | noinput_lapse_action[cls(small_action)] | w | L973 · 39850 · input None && last_lapse · cls: Trace→0 · RunAway/Recall/AroundRunAway→1 · Around/AroundHide/AroundRegion/Positioning/AroundPosition/AroundPositionBush/AroundBush→2 · Attack/Skill/Skill2/Ult→3 · LaneMinionPosition→4 · Stop→5 | 4 | 오귀속(사전은 다른 필드를 준다) | +1 |
| 94 | AgentVerHamster | 0x29b0 | noinput_lapse_pos[band] | w | L976 · 39951 · input None && last_lapse && champ Some · band 0=분수 안 · 1=넥서스 ≤260000 · 2=그 밖/넥서스 없음 | 4 | OK | +1 |
| 95 | AgentVerHamster | 0x1d78 -> stay_events.ptr[len] (208B) · len@0x1d88 +1 · grow_one 시 cap/ptr | stay_events.push(StayEvent{..}) | w | L1004 · 41412~41532 · context.debug && champ Some && (stays \|\| small_action==Stop). tick/team/position/pos.x/kind/pos_band/can_buy 는 배치 M 값, goal·문자열 4개·hp_pct·near_*·since_combat·in_lapse·target_kind 는 배치 N(1009~1043) 값 | 4 | 부분일치(첫 성분(컨테이너)만 맞고 **마지막 성분이 다르다** — 사람이 봐야 ) | StayEvent{goal@0(24B) plan_label@0x18 plan_detail@0x30 sub_plan@0x48 action@0x60 target_kind@0x78(ptr,len) tick@0x88=tick() team@0x90 pos@0x98=(champ.x, champ.y) hp_pct@0xa8 near_enemy@0xb0 near_ally@0xb8 since_combat@0xc0 position@0xc8 can_buy@0xcc in_lapse@0xcd kind@0xce pos_band@0xcf} |
| 96 | global | 0x0 | prof::PHASE_NANOS[phase]/PHASE_CALLS[phase] (phase=26, bound 132) | w | L906 ProfTimer drop · 39024~39041 · ENABLED≠0 일 때만 | 4 | 오귀속(그 오프셋에 필드가 없다(패딩/관통불가)) | atomicrmw add elapsed_ns / +1 |
| 97 | global | 0x0 | entity::CNT_DM_IDLE_INPUT[input_tag+1] ([7 x atomic usize]) | w | L961 · 39749~39753 · 데스매치 && plan==DeathMatchBattle && idle_spec_tick==tick · 인덱스: None(-1)→0 Move→1 Return→2 Attack→3 Skill→4 Skill2→5 Ult→6 | 4 | 오귀속(그 오프셋에 필드가 없다(패딩/관통불가)) | atomicrmw add 1 |
| 98 | AgentVerHamster | 0x1d78 -> stay_events.ptr[len]+0..+208 | stay_events.push(StayEvent{..}) 원소 값 | w | ★루트 L1004(배치 M · m14.ll:41412~41454 store · 41474 grow_one · 41530 memcpy 208B · 41532 len+1) — 값 계산이 내 범위(1005~1043). kind/can_buy/pos_band/position/team 값은 배치 M 계산. 조건: L983 champ.is_some() && context.debug(+0x3b) (배치 M) | 4 | 부분일치(첫 성분(컨테이너)만 맞고 **마지막 성분이 다르다** — 사람이 봐야 ) | goal(+0 tag/+8 x/+16 y)=small_action.target_position() · plan_label(+24)=plan.debug_label() · plan_detail(+48)=plan.get_name() · sub_plan(+72)=short(format!("{:?}",sub_plan)) · action(+96)=format!("{:?}",small_action.get_action()) · target_kind(+120 ptr/+128 len)=&str · tick(+136)=game.tick() · team(+144)=%221 · pos(+152,+160)=(champ.x,champ.y) · hp_pct(+168) · near_enemy(+176) · near_ally(+184) · since_combat(+192) · position(+200)=%225 · can_buy(+204)=%631 · in_lapse(+205)=last_lapse · kind(+206)=%580 · pos_band(+207)=%632 |
| 99 | AgentVerHamster | 0x2960 | freeze_since | w | L1052 (m14.ll:41670) — 앵커 리셋 경로(moved \|\| input.is_some() \|\| freeze_since==0) | 4 | OK | tick(%1132 · L1049) |
| 100 | AgentVerHamster | 0x430 | freeze_anchor.0 | w | L1053 (m14.ll:41671) | 4 | OK | champ.x(%1188) |
| 101 | AgentVerHamster | 0x438 | freeze_anchor.1 | w | L1053 (m14.ll:41672) | 4 | OK | champ.y(%1190) |
| 102 | AgentVerHamster | 0x29cb | freeze_fired | w | 리셋 경로에서 false · 1초 게이트 통과 시 true(에피소드 1회 발화 래치) | 4 | OK | false(L1054 · m14.ll:41674) / true(L1058 · 41701) |
| 103 | AgentVerHamster | 0x2978 | freeze_ticks | w | L1056 (m14.ll:41694~41697) — freeze_fired 이미 true 인 틱마다(정지 누적 틱) | 4 | OK | +1 |
| 104 | AgentVerHamster | 0x440 | freeze_eps[cls] | w | L1060 (m14.ll:41750~41754) · cls = small_action_cls(&small_action) 0~5 | 4 | OK | +1 |
| 105 | AgentVerHamster | 0x470 | freeze_plan[plan_cls] | w | L1061 (m14.ll:41793~41797) · plan_cls = plan.freeze_plan_cls() 0~5 | 4 | OK | +1 |
| 106 | AgentVerHamster | 0x2980 | freeze_pos[pos_cls] | w | L1072 (m14.ll:41869~41873 / 41878~41881 [2] / 41894~41897 [0]) · pos_cls 0 우물 사각 안 · 1 넥서스 260k 이내 · 2 필드(넥서스 None 포함) | 4 | OK | +1 |
| 107 | AgentVerHamster | 0x4a0 | freeze_field_action[cls] | w | L1073 (m14.ll:41885~41889) — pos_cls==2 일 때만 | 4 | OK | +1 |
| 108 | AgentVerHamster | 0x2968 | freeze_eps_lapse | w | L1074 (m14.ll:41909~41912) — last_lapse 일 때 | 4 | OK | +1 |
| 109 | AgentVerHamster | 0x2970 | freeze_eps_dead_target | w | L1075 (m14.ll:41916~41919) — %436(배치 M L~964 small_action.is_premise_lost(data) 결과) 일 때 | 4 | OK | +1 |
| 110 | AgentVerHamster | 0x2928 | trace_escape_target | w | L1080 → lib.rs:1097 (m14.ll:41619) — last_escape=Some(true) 이고 (target≠tid \|\| tick<last_tick) 일 때 | 4 | OK | tid(small_action@Trace.target) |
| 111 | AgentVerHamster | 0x2930 | trace_escape_ticks | w | Trace 이면서 last_escape 여부에 따라 누적/감쇠 | 4 | OK | (tick - last_tick) + ticks (lib.rs:1095 · m14.ll:41623~41630) / 0 (1097 · 41629 phi) / ticks.saturating_sub(tick.saturating_sub(last_tick)) (1107 · 41613~41614) |
| 112 | AgentVerHamster | 0x2938 | trace_escape_last_tick | w | lib.rs:1100 (m14.ll:41631) · 1108 (41615) | 4 | OK | tick(%1139) |
| 113 | AgentVerHamster | 0x28ea | small_action@Trace.abandoned | w | lib.rs:1102 (m14.ll:41640) — trace_escape_ticks >= tps*2 이고 아직 false 일 때 | 4 | OK | true(i8 1) |
| 114 | (sret) | 0x0 | ret.0 Option<Input> | w | L1082 (m14.ll:41925) | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) | %47(input) 32B memcpy |
| 115 | (sret) | 0x20 | ret.1 bumpalo Vec<TurnEvent> | w | L1082 (m14.ll:41923~41924) | 4 | 확인불가(★모호: 동명 def_path 40개 [("<engine_core::pa) | %51(turn_event) 32B memcpy |

**`consts` 상수 43건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev | folded_from |
|---|---|---|---|---|---|---|
| 0 | 100 | 851 | 계수 | delay = gen_range(min..=max) / 100 (udiv). input_delay_* 는 1/100 틱 단위로 추정 — AthleteParameter 미열람 | 5 |  |
| 1 | 26 | 855 | 산출값 | ProfTimer phase 인덱스(_t_sai · 'sai' 페이즈) · PHASE_NANOS/PHASE_CALLS[26] | 4 |  |
| 2 | 132 | 855 | 길이 | PHASE_NANOS/PHASE_CALLS 배열 길이(bounds check) — 판정 아님 | 4 |  |
| 3 | 1000000000 | 906 | 임계 | Duration → ns 환산(secs*1e9 + nanos) | 4 |  |
| 4 | -1 | 865 | 센티널 | Option<Input>::None 의 니치 태그(i64) · L848 store · L865/876/888/915/970/985 비교. ProfTimer 의 -1 은 별개(nanos i32 니치) | 4 |  |
| 5 | 1 | 865 | 임계 | `version > 1` (= version ≥ 2 · L865/876/896 세 게이트). 본문의 `shl … 1`(L971) 은 이 상수가 아니라 last_lapse<<1 비트 합성(그 항목은 value 1/folded_from 2 로 별도 등록) | 4 |  |
| 6 | 1 | 971 | 인덱스 | noinput_bucket 인덱스 = last_lapse*2 + target_dead — `shl nuw nsw i8 %452, 1` 로 접힘 | 4 | 2 |
| 7 | 5 | 899 | 태그 | SmallActionRunAway::new(data, player, 5) 의 3번째 usize 인자 — 의미는 move_actions.rs:25 명세(r14) 참조 · 여기선 리터럴 | 4 |  |
| 8 | 3 | 899 | 태그 | SmallActionPlay 메모리 태그 3 = RunAway(idx0 · niche_start 3) | 4 |  |
| 9 | 10 | 909 | 태그 | SmallActionPlay 태그 10 은 untagged AroundPosition 자리 — 컴파일러 assume ≠10(L909/921/973/993 반복) | 4 |  |
| 10 | 15 | 909 | 태그 | tag-15 < 4 ⟹ tag ∈ {15 Attack,16 Skill,17 Skill2,18 Ult} 이면 전투 액션 → last_combat_tick 갱신 | 4 |  |
| 11 | 4 | 909 | 태그 | 위 범위 폭(4개 variant) | 4 |  |
| 12 | 19 | 921 | 태그 | SmallActionPlay 태그 19 = Stop(idx16) — 비-Move 입력일 때 stays=true 조건(L921) · L993 kind 판정 | 4 |  |
| 13 | 0 | 915 | 태그 | Input 태그 0 = Move → adjust_position 후 거리 판정 분기(L915/985 switch) | 4 |  |
| 14 | 4000001 | 919 | 임계 | 2000^2+1 — 이동 목표(adjust_position 보정 후)와 챔피언 현위치 거리² < 이면 stays(제자리 이동) · L919 · L990 | 4 |  |
| 15 | 2 | 948 | 태그 | GameMode 태그 2 = DeathMatch(get_game_mode) · L949 BigPlan tag<2 = untagged DeathMatchBattle · (L889/914/933 의 2 는 팀 배열 bound) | 4 |  |
| 16 | 6 | 949 | 센티널 | BigPlan 니치에서 untagged DeathMatchBattle 자리(assume ≠6) — 판정 아님 (aux closure#0 의 `shl … 6` 은 UTF-8 디코딩 비트 시프트 — 이 상수와 무관) | 4 |  |
| 17 | 67600000001 | 977 | 임계 | 260000^2+1 — 챔피언↔자기 넥서스 거리² < 이면 pos_band 1(근처) · L977 · L999 | 4 |  |
| 18 | 7 | 973 | 태그 | SmallActionPlay tag≤2 → 논리 idx 7(AroundPosition untagged) 매핑용 select 상수(L973/1026 반복) | 4 |  |
| 19 | 62500000001 | 935 | 미상 | (aux count 클로저) 250000^2+1 — 적 챔피언↔자기 챔피언 거리² < 이면 threat 로 셈 | 4 |  |
| 20 | 2097143 | 931 | 계수 | (aux closure#0) char & 0x1FFFF7 == 32 ⟹ c==' '(32) \|\| c=='('(40) — take_while 술어 `c != ' ' && c != '('` 가 마스크 비교로 접힘 | 4 |  |
| 21 | 32 | 931 | 태그 | (aux closure#0) 위 비교의 우변 ' ' | 4 |  |
| 22 | 1114112 | 931 | 임계 | (aux closure#0) char 상한 assume(0x110000) — 판정 아님 | 4 |  |
| 23 | 100 | 1016 | 계수 | hp_pct = hp*100 / max(stat_cached.hp, 1) — 백분율 (m14.ll:40818) | 4 |  |
| 24 | 1 | 1016 | 임계 | umax(max_hp, 1) 0-나눗셈 가드 (m14.ll:40823 llvm.umax) · ★같은 값 1 이 L1090 `shl i64 %tps, 1`(tps*2 · m14.ll:41566)·L1017 `1 - team`(41826)에도 쓰임 — shl 은 아래 folded_from 항목 참조 | 4 |  |
| 25 | 1 | 1090 | 임계 | abandon_threshold = tick_per_second * 2 (= 2초) — `shl i64 %1144, 1` 로 접힘 (m14.ll:41566) | 4 | 2 |
| 26 | 22500000001 | 1019 | 임계 | 150000² + 1 — dist² < 이 값 ⟺ dist ≤ 150000(150k). L1019 적 근접 카운트(m14.ll:40954) · L1021 아군 근접 카운트(41064·41124·41192·41260·41328) 공용 | 4 |  |
| 27 | 2097143 | 1000 | 계수 | 0x1FFFF7 — `(c & 0x1FFFF7) == 32` 는 c==' '(0x20) \|\| c=='('(0x28) 를 컴파일러가 한 비교로 접은 것(m14.ll:40447~40448). closure#5(lib.rs:1000) sub_plan Debug 문자열을 첫 공백/여는괄호 앞에서 자르는 술어 | 4 |  |
| 28 | 32 | 1000 | 태그 | 위 마스크 비교의 우변 ' '(0x20) (m14.ll:40448) | 4 |  |
| 29 | 1114112 | 1000 | 임계 | 0x110000 — char 유효범위 assume(UTF-8 디코더 인라인 아티팩트 · m14.ll:40413) · 판정값 아님 | 4 |  |
| 30 | 1835008 | 1000 | 계수 | 0x1C0000 — 4바이트 UTF-8 선두 마스크(디코더 인라인 · m14.ll:40395) · 판정값 아님 | 4 |  |
| 31 | 10 | 1012 | 태그 | small_action 태그 10 은 존재하지 않음(AroundPosition 암묵 태그 자리) — `assume tag != 10` (m14.ll:40568 · 40766 · 41358 · 41704 · 41569). 판정값 아님 | 4 |  |
| 32 | 3 | 1012 | 센티널 | SmallActionPlay 니치 시작(niche_start=3): idx = tag>2 ? tag-3 : 7 (m14.ll:40570~40572). 판정값 아님(태그 환산) | 4 |  |
| 33 | 14 | 1091 | 태그 | small_action 메모리 태그 14 = Trace(idx 11) — update_trace_escape_abandon 은 Trace 일 때만 동작 (m14.ll:41571) | 4 |  |
| 34 | 6 | 1061 | 태그 | BigPlan 태그 6 은 존재하지 않음(DeathMatchBattle 암묵 자리) — `assume tag != 6` (m14.ll:41758). 판정값 아님 · 본문의 `shl … 6` 은 UTF-8 디코더(L1000 인라인)의 6비트 시프트로 이 상수와 무관 | 4 |  |
| 35 | 2 | 1061 | 센티널 | BigPlan 니치 시작(niche_start=2): idx = tag>1 ? tag-2 : 4 (m14.ll:41760~41762). ★같은 값 2 가 L1037 `camp_type.0 < 2`(정글몹 가드 · 41511)·L1067 pos_cls 2(필드 · 41867)에도 쓰임 | 4 |  |
| 36 | 13 | 1034 | 태그 | EntityType 태그 13 = Champion → target_kind "챔피언" (m14.ll:41494) | 4 |  |
| 37 | 5 | 1035 | 태그 | EntityType 태그 5 = Epic → "에픽" (m14.ll:41495) · ★같은 값 5 가 small_action_cls Stop→5(기타 · 41748)·freeze_plan_cls 기타 5(41792)에도 | 4 |  |
| 38 | 4 | 1037 | 태그 | EntityType 태그 4 = Jungle → camp_type.0 < 2 이면 "정글몹" 아니면 wildcard "미니언" (m14.ll:41497) | 4 |  |
| 39 | -1 | 1024 | 센티널 | usize::MAX 센티넬: last_combat_tick == MAX 이면 since_combat = MAX(미전투) (m14.ll:41347·41355). L1051 `input tag != -1`(Option<Input>::is_some · 41661)에도 같은 값 | 4 |  |
| 40 | 8000 | 1050 | 임계 | moved = distance(champ, freeze_anchor) > 8000 (8k · 셀 1/4) — 이 이상 움직였으면 정지 앵커 리셋 (m14.ll:41658) | 4 |  |
| 41 | 67600000000 | 1067 | 임계 | 260000² — dist²(champ, nexus[team]) > 이 값 ⟺ dist > 260k → pos_cls 2(필드), 아니면 1(본진 근처) (m14.ll:41866) | 4 |  |
| 42 | 0 | 1051 | 태그 | freeze_since == 0 (아직 앵커 없음) 이면 리셋 (m14.ll:41665) | 4 |  |

**`knobs` 조정점 11건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 입력 지연 분모 | lib.rs:851 | 100 | 내리면(예 50) 같은 input_delay_* 값에서 다음 판단까지의 틱이 2배 — 반응 느려짐. 올리면 빨라짐(0 이면 매 틱) | 4 | 기존 |
| 1 | 제자리 판정 반경 | lib.rs:919 / :990 | 4000001 | 2000 단위(=1/16 셀). 올리면 짧은 이동도 '정지'로 분류 → stay_home_full_nobuy·StayEvent 진단이 더 많이 찍힘(게임 상태·RNG 무영향, 관측 전용) | 4 | 기존 |
| 2 | 위협 감지 반경(적 챔피언) | lib.rs:935 | 62500000001 | 250000(≈7.8 셀). 키 문자열의 '적N' 값만 바뀜 — 진단 전용 | 4 | 기존 |
| 3 | 넥서스 근접 밴드 | lib.rs:977 / :999 | 67600000001 | 260000(≈8.1 셀). noinput_lapse_pos / StayEvent.pos_band 분류만 — 진단 전용 | 4 | 기존 |
| 4 | 도주 강제 시 RunAway 파라미터 | lib.rs:899 | 5 | SmallActionRunAway::new 의 3번째 인자 — 의미는 r14 명세(move_actions.rs:25) 소관. ★이 경로(L898~900)는 실제 입력을 바꾼다(v2+ · lapse 중 3회 무입력 · 분수 밖 또는 풀피) | 4 | 기존 |
| 5 | 버전 게이트 | lib.rs:865 / :876 / :896 | 1 | version ≤ 1 이면 update_small_action 재시도·폴백·도주 강제 전부 비활성(1차 get_input 결과 그대로) — 유일한 판단 개입 스위치 | 4 | 기존 |
| 6 | StayEvent 근접 반경(적·아군 공통) | lib.rs:1019 · 1021 · m14.ll:40954 · 41064 | 22500000001 | 올리면 near_enemy/near_ally 가 더 먼 상대까지 세어 계측 버킷이 바뀐다 · 관측 전용(context.debug 일 때만 push) — AI 판단엔 영향 0 | 4 | 기존 |
| 7 | 정지 앵커 리셋 이동 임계 | lib.rs:1050 · m14.ll:41658 | 8000 | 올리면 더 큰 이동도 '제자리'로 간주해 freeze 에피소드가 더 자주 잡힌다 · 내리면 미세 이동에도 앵커 리셋 → 에피소드↓. 계측 카운터(freeze_*)만 바뀜 — 입력 결과 무관 | 4 | 기존 |
| 8 | 정지 에피소드 확정 시간(1초 = tick_per_second) | lib.rs:1057 · m14.ll:41688 · GameSetting+0x12f8 | tick_per_second | 게임 설정값 자체가 게이트. 계측 전용 | 4 | 기존 |
| 9 | 본진 근처 판정 반경(넥서스 260k) | lib.rs:1067 · m14.ll:41866 | 67600000000 | 올리면 pos_cls 1(본진 근처) 범위↑ · 필드(2) 에피소드↓. 계측 전용 | 4 | 기존 |
| 10 | Trace 도주 포기 임계(tps*2 = 2초) | lib.rs:1090 · m14.ll:41566 (shl 1) | 2 | ★AI 동작에 실제 영향: Trace 액션이 last_escape=Some(true) 인 채 누적 trace_escape_ticks 가 이 값 이상이면 small_action@Trace.abandoned=true 로 래치 → 추격 포기(소비처 = Trace 액션 측 · 이 함수 밖). 올리면 더 오래 타워 회피 후퇴를 반복한 뒤 포기 · 내리면 빨리 포기 | 4 | 기존 |

<details><summary>`callees` 피호출자 45건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | adjust_position | game_core::Game::adjust_position | pub | fn(&game_core::MapDef, &game_core::GameSetting, i64, i64) -> (u64, u64) | game-core\src\simulation\game.rs:818 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | buy_item | game_ai::buy_item | in:game_ai | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::GameContext) -> std::option::Option<usize> | game-ai\src\lib.rs:1477 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 2 | contains | game_core::RectU64::contains | pub | fn(&game_core::RectU64, u64, u64) -> bool | game-core\src\setting.rs:40 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 3 | contains | game_core::RectI64::contains | pub | fn(&game_core::RectI64, i64, i64) -> bool | game-core\src\setting.rs:55 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 4 | contains | game_core::setting::champion::nightmare::NightmareWellRect::contains | in:game_core::setting::champion::nightmare | fn(game_core::setting::champion::nightmare::NightmareWellRect, u64, u64) -> bool | game-core\src\setting\champion\nightmare.rs:28 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 5 | debug | game_core::AbstractBanpickRunner::debug | pub | fn(&Self/#0, &[usize]) | game-core\src\banpick\base_runner.rs:411 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 6 | debug | <game_core::FactoredBanpickAgent as game_core::AbstractBanpickRunner>::debug | pub | fn(&game_core::FactoredBanpickAgent, &[usize]) | game-core\src\banpick\sgd_v2.rs:5891 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 7 | debug | <game_core::LogisticSGDBanpickAgent as game_core::AbstractBanpickRunner>::debug | pub | fn(&game_core::LogisticSGDBanpickAgent, &[usize]) | game-core\src\banpick\sgd.rs:1172 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 8 | debug_label | game_ai::plan_legacy::types::BigPlan::debug_label | pub | fn(&game_ai::plan_legacy::types::BigPlan) -> std::string::String | game-ai\src\plan_legacy\types.rs:69 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 9 | distance | game_core::utils::distance | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:12 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 10 | freeze_plan_cls | game_ai::plan_legacy::types::BigPlan::freeze_plan_cls | pub | fn(&game_ai::plan_legacy::types::BigPlan) -> usize | game-ai\src\plan_legacy\types.rs:121 | True | True | 3 | IR 호출 심볼 일치(토큰 경계) |
| 11 | get_action | game_ai::SmallActionPlay::get_action | pub | fn(&game_ai::SmallActionPlay) -> game_core::SmallAction | game-ai\src\small_action.rs:308 | True | True | 3 | IR 호출 심볼 일치(토큰 경계) |
| 12 | get_entity_by_id | game_core::AbstractGame::get_entity_by_id | pub | fn(&Self/#0, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:178 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 13 | get_entity_by_id | <game_core::Game as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::Game, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3643 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 14 | get_entity_by_id | <game_core::SingleLaneGame as game_core::AbstractGame>::get_entity_by_id | pub | fn(&game_core::SingleLaneGame, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation\game.rs:3975 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 15 | get_game_mode | game_core::AbstractGame::get_game_mode | pub | fn(&Self/#0) -> game_core::GameMode | game-core\src\simulation.rs:78 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 16 | get_game_mode | <game_core::Game as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::Game) -> game_core::GameMode | game-core\src\simulation\game.rs:1707 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 17 | get_game_mode | <game_core::SingleLaneGame as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::SingleLaneGame) -> game_core::GameMode | game-core\src\simulation\game.rs:4248 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 18 | get_input | game_ai::SmallActionPlay::get_input | pub | fn(&mut game_ai::SmallActionPlay, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &mut game_core::DebugFrameData) -> std::option::Option<game_core::Input> | game-ai\src\small_action.rs:157 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 19 | get_name | game_ai::plan_legacy::types::BigPlan::get_name | pub | fn(&game_ai::plan_legacy::types::BigPlan) -> std::string::String | game-ai\src\plan_legacy\types.rs:44 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 20 | input_delay_max | game_core::AthleteParameter::input_delay_max | pub | fn(&game_core::AthleteParameter) -> usize | game-core\src\simulation\state\player.rs:239 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 21 | input_delay_min | game_core::AthleteParameter::input_delay_min | pub | fn(&game_core::AthleteParameter) -> usize | game-core\src\simulation\state\player.rs:232 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 22 | is_premise_lost | game_ai::SmallActionPlay::is_premise_lost | pub | fn(&game_ai::SmallActionPlay, &game_core::OperationData) -> bool | game-ai\src\small_action.rs:376 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 23 | is_recent_visible | game_core::Blackboard::is_recent_visible | pub | fn(&game_core::Blackboard, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\game\blackboard.rs:346 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 24 | iter_champions | game_core::AbstractGameWithCache::<'a, 'b>::iter_champions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5759 ~ game_core[6a30]::simulation::{impl#5}::iter_champions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1904 | True | True | 3 | IR 호출 심볼 일치(토큰 경계) |
| 25 | new | game_ai::SmallActionRunAway::new | pub | fn(&game_core::OperationData, &game_core::PlayerState, usize) -> game_ai::SmallActionRunAway | game-ai\src\small_action\move_actions.rs:25 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 26 | position | game_core::PlayerAiContext::<'a, 'b, 'r>::position | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> game_core::Position | game-core\src\mod_ai.rs:587 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 27 | position | game_core::transfer::TeamRosterPlanningContext::position | pub | fn(&game_core::transfer::TeamRosterPlanningContext, game_core::Position) -> std::option::Option<&game_core::transfer::PositionPlanningContext> | game-core\src\transfer\roster_context.rs:234 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 28 | push | <game_core::Staff as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::staff | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\staff.rs:38 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 29 | push | <game_core::Athlete as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:434 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 30 | push | <game_core::Contract as engine_core::spitz::SpitzDatable>::serialize::push | in:game_core::data::athlete | fn(&mut std::vec::Vec<u8, std::alloc::Global>, &T/#0) | game-core\src\data\athlete.rs:2498 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 6개 중 상위 3개 |
| 31 | small_action_cls | game_ai::small_action_cls | in:game_ai | fn(&game_ai::SmallActionPlay) -> usize | game-ai\src\lib.rs:481 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 32 | target_position | game_ai::SmallActionPlay::target_position | pub | fn(&game_ai::SmallActionPlay) -> std::option::Option<(u64, u64)> | game-ai\src\small_action.rs:229 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 33 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 34 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 35 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 36 | to_string | game_core::Position::to_string | pub | fn(&game_core::Position) -> std::string::String | game-core\src\simulation\entity.rs:665 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 4개 중 상위 3개 |
| 37 | to_string | game_core::LineType::to_string | pub | fn(&game_core::LineType) -> std::string::String | game-core\src\simulation\state\player.rs:997 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 4개 중 상위 3개 |
| 38 | to_string | game_core::JungleType::to_string | pub | fn(&game_core::JungleType) -> std::string::String | game-core\src\simulation\entity\jungle.rs:394 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 4개 중 상위 3개 |
| 39 | update_small_action | game_ai::AgentVerHamster::update_small_action | in:game_ai | fn(&mut game_ai::AgentVerHamster, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) | game-ai\src\lib.rs:693 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 40 | update_state | game_ai::AgentVerHamster::update_state | pub | fn(&mut game_ai::AgentVerHamster, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bumpalo::collections::vec::Vec< game_core::TurnEvent> | game-ai\src\lib.rs:556 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 41 | update_trace_escape_abandon | game_ai::AgentVerHamster::update_trace_escape_abandon | in:game_ai | fn(&mut game_ai::AgentVerHamster, &game_core::OperationData) | game-ai\src\lib.rs:1088 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 42 | upgrade_item | game_ai::upgrade_item | in:game_ai | fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::GameContext) -> std::option::Option<(usize, usize)> | game-ai\src\lib.rs:1603 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 43 | v3_fall_back_to_passive | game_ai::plan_legacy::handler::LegacyPlanHandler::v3_fall_back_to_passive | pub | fn(&mut game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &mut game_core::DebugFrameData) | game-ai\src\plan_legacy\handler.rs:416 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 44 | v3_repair_done | game_ai::plan_legacy::handler::LegacyPlanHandler::v3_repair_done | in:game_ai | fn(&game_ai::plan_legacy::handler::LegacyPlanHandler, usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bool | game-ai\src\plan_legacy\handler.rs:1847 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
</details>

⚠**미매칭 32개**: `ally_row`, `chars`, `dereferenceable`, `dist2`, `drop  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 1개는 **전부 다른 함수**라 싣지 않는다`, `drop_in_place`, `elapsed`, `elapsed_ns`, `else`, `entry`, `fetch_add`, `format_inner`, `gen_range`, `get_input0`, `grow_one`, `handle_error`, `in_rect`, `input`, `insert_no_grow`, `lapse`, `llvm.umax.i64`, `llvm.usub.sat.i64`, `or_insert`, `phase`, `plan_system`, `poison`, `rustc_entry`, `short`, `take_while`, `target_kind`, `try_allocate_in`, `turn_event`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 1곳** (m14.ll:56771) · **형제 90개** (AgentVerHamster)

| # | 경로 | vis | 정의처 | mir | sig |
|---|---|---|---|---|---|
| 0 | <game_ai::AgentVerHamster as std::clone::Clone>::clone | pub | game-ai\src\lib.rs:121 | True | fn(&game_ai::AgentVerHamster) -> game_ai::AgentVerHamster |
| 1 | <game_ai::AgentVerHamster as std::fmt::Debug>::fmt | pub | game-ai\src\lib.rs:121 | True | fn(&game_ai::AgentVerHamster, &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> |
| 2 | game_ai::AgentVerHamster::plan_chats_drain | pub | game-ai\src\lib.rs:197 | False | fn(&mut game_ai::AgentVerHamster) -> std::vec::Vec<game_core::Chat, std::alloc::Global> |
| 3 | game_ai::AgentVerHamster::push_pending_trace_event | pub | game-ai\src\lib.rs:208 | False | fn(&mut game_ai::AgentVerHamster, usize, game_core::TraceEventType) |
| 4 | game_ai::AgentVerHamster::plan_goal | pub | game-ai\src\lib.rs:211 | False | fn(&game_ai::AgentVerHamster) -> std::option::Option<game_core::BigGoal> |
| 5 | game_ai::AgentVerHamster::plan_name | pub | game-ai\src\lib.rs:214 | False | fn(&game_ai::AgentVerHamster) -> std::string::String |
| 6 | game_ai::AgentVerHamster::battle_sub_goal_dir | pub | game-ai\src\lib.rs:218 | False | fn(&game_ai::AgentVerHamster) -> std::option::Option<i8> |
| 7 | game_ai::AgentVerHamster::battle_sub_goal_is_full_runaway | pub | game-ai\src\lib.rs:222 | True | fn(&game_ai::AgentVerHamster) -> bool |
| 8 | game_ai::AgentVerHamster::plan_v3_cand_src | pub | game-ai\src\lib.rs:227 | True | fn(&game_ai::AgentVerHamster) -> u8 |
| 9 | game_ai::AgentVerHamster::plan_v3_last_stand | pub | game-ai\src\lib.rs:231 | True | fn(&game_ai::AgentVerHamster) -> bool |
| 10 | game_ai::AgentVerHamster::plan_v3_final_stand | pub | game-ai\src\lib.rs:235 | True | fn(&game_ai::AgentVerHamster) -> bool |
| 11 | game_ai::AgentVerHamster::plan_v3_bail_goal | pub | game-ai\src\lib.rs:239 | True | fn(&game_ai::AgentVerHamster) -> u8 |
| 12 | game_ai::AgentVerHamster::plan_ff_line_counts | pub | game-ai\src\lib.rs:243 | True | fn(&game_ai::AgentVerHamster) -> [usize; 4_usize] |
| 13 | game_ai::AgentVerHamster::plan_ff_retreat_stats | pub | game-ai\src\lib.rs:247 | True | fn(&game_ai::AgentVerHamster) -> (usize, usize, usize, usize, usize) |
| 14 | game_ai::AgentVerHamster::team_objective_code | pub | game-ai\src\lib.rs:254 | True | fn(&game_ai::AgentVerHamster) -> u8 |
| 15 | game_ai::AgentVerHamster::eo_cover_picks | pub | game-ai\src\lib.rs:258 | False | fn(&game_ai::AgentVerHamster) -> usize |
| 16 | game_ai::AgentVerHamster::eo_serpen_punish_issues | pub | game-ai\src\lib.rs:262 | True | fn(&game_ai::AgentVerHamster) -> usize |
| 17 | game_ai::AgentVerHamster::subplan_is_recall | pub | game-ai\src\lib.rs:266 | True | fn(&game_ai::AgentVerHamster) -> bool |
| 18 | game_ai::AgentVerHamster::plan_ff_call_stats | pub | game-ai\src\lib.rs:270 | True | fn(&game_ai::AgentVerHamster) -> (usize, usize, usize, usize, usize, usize, usize, usize) |
| 19 | game_ai::AgentVerHamster::plan_gank_attempt_count | pub | game-ai\src\lib.rs:275 | True | fn(&game_ai::AgentVerHamster) -> usize |
| 20 | game_ai::AgentVerHamster::plan_gank_periods | pub | game-ai\src\lib.rs:278 | True | fn(&game_ai::AgentVerHamster) -> &std::vec::Vec<(usize, usize), std::alloc::Global> |
| 21 | game_ai::AgentVerHamster::plan_gank_score_attempts | pub | game-ai\src\lib.rs:281 | True | fn(&game_ai::AgentVerHamster) -> &std::vec::Vec<(i32, usize, usize), std::alloc::Global> |
| 22 | game_ai::AgentVerHamster::plan_gank_request_count | pub | game-ai\src\lib.rs:284 | True | fn(&game_ai::AgentVerHamster) -> usize |
| 23 | game_ai::AgentVerHamster::plan_v46_lane_recall_stats | pub | game-ai\src\lib.rs:289 | True | fn(&game_ai::AgentVerHamster) -> (&std::vec::Vec<(usize, game_core::LineType), std::alloc::Global>, usize, usize, usize, usize, usize, usize, usize, usize) |
| 24 | game_ai::AgentVerHamster::plan_flee_death_retrospects | pub | game-ai\src\lib.rs:297 | True | fn(&game_ai::AgentVerHamster) -> &std::vec::Vec<(usize, u8, u8, u8), std::alloc::Global> |
| 25 | game_ai::AgentVerHamster::plan_v48_dodge_stats | pub | game-ai\src\lib.rs:302 | True | fn(&game_ai::AgentVerHamster) -> (usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) |
| 26 | game_ai::AgentVerHamster::plan_v46_flee_stats | pub | game-ai\src\lib.rs:310 | True | fn(&game_ai::AgentVerHamster) -> (usize, usize, usize, usize, usize, usize) |
| 27 | game_ai::AgentVerHamster::plan_v46_flee_episodes | pub | game-ai\src\lib.rs:317 | True | fn(&game_ai::AgentVerHamster) -> &std::vec::Vec<(usize, game_core::LineType, u8), std::alloc::Global> |
| 28 | game_ai::AgentVerHamster::plan_v50_dive_episodes | pub | game-ai\src\lib.rs:321 | True | fn(&game_ai::AgentVerHamster) -> &std::vec::Vec<game_core::V50DiveEpisode, std::alloc::Global> |
| 29 | game_ai::AgentVerHamster::plan_epic_steal_attempt_count | pub | game-ai\src\lib.rs:324 | True | fn(&game_ai::AgentVerHamster) -> usize |
| 30 | game_ai::AgentVerHamster::plan_epic_steal_success_count | pub | game-ai\src\lib.rs:327 | True | fn(&game_ai::AgentVerHamster) -> usize |
| 31 | game_ai::AgentVerHamster::plan_serpen_steal_attempt_count | pub | game-ai\src\lib.rs:330 | True | fn(&game_ai::AgentVerHamster) -> usize |
| 32 | game_ai::AgentVerHamster::plan_serpen_steal_success_count | pub | game-ai\src\lib.rs:333 | True | fn(&game_ai::AgentVerHamster) -> usize |
| 33 | game_ai::AgentVerHamster::plan_steal_sessions | pub | game-ai\src\lib.rs:336 | False | fn(&game_ai::AgentVerHamster) -> std::vec::Vec<game_core::StealSession, std::alloc::Global> |
| 34 | game_ai::AgentVerHamster::plan_pending_trace_events_drain | pub | game-ai\src\lib.rs:339 | False | fn(&mut game_ai::AgentVerHamster) -> std::vec::Vec<game_core::PendingTraceEvent, std::alloc::Global> |
| 35 | game_ai::AgentVerHamster::plan_comeback_pick_stats | pub | game-ai\src\lib.rs:342 | False | fn(&game_ai::AgentVerHamster) -> (usize, usize, std::vec::Vec<(i32, u8), std::alloc::Global>) |
| 36 | game_ai::AgentVerHamster::plan_v54_cj_call_ticks | pub | game-ai\src\lib.rs:346 | True | fn(&game_ai::AgentVerHamster) -> &std::vec::Vec<usize, std::alloc::Global> |
| 37 | game_ai::AgentVerHamster::plan_is_counter_jungle | pub | game-ai\src\lib.rs:350 | True | fn(&game_ai::AgentVerHamster) -> bool |
| 38 | game_ai::AgentVerHamster::plan_v54_fs_fog_stats | pub | game-ai\src\lib.rs:357 | False | fn(&game_ai::AgentVerHamster) -> (usize, usize) |
| 39 | game_ai::AgentVerHamster::plan_v54_reentry_ticks | pub | game-ai\src\lib.rs:362 | True | fn(&game_ai::AgentVerHamster) -> &std::vec::Vec<usize, std::alloc::Global> |
| 40 | game_ai::AgentVerHamster::new | pub | game-ai\src\lib.rs:368 | False | fn(&mut rand::rngs::std::StdRng, usize, usize, game_core::Position) -> game_ai::AgentVerHamster |
| 41 | game_ai::AgentVerHamster::init | pub | game-ai\src\lib.rs:413 | True | fn(&mut game_ai::AgentVerHamster) |
| 42 | <game_ai::AgentVerHamster as game_core::AiAgent>::as_any | pub | game-ai\src\lib.rs:425 | True | fn(&game_ai::AgentVerHamster) -> &dyn [Binder { value: Trait(std::any::Any), bound_vars: [] }] + 'static |
| 43 | <game_ai::AgentVerHamster as game_core::AiAgent>::as_any_mut | pub | game-ai\src\lib.rs:426 | True | fn(&mut game_ai::AgentVerHamster) -> &mut dyn [Binder { value: Trait(std::any::Any), bound_vars: [] }] + 'static |
| 44 | <game_ai::AgentVerHamster as game_core::AiAgent>::version | pub | game-ai\src\lib.rs:428 | True | fn(&game_ai::AgentVerHamster) -> usize |
| 45 | <game_ai::AgentVerHamster as game_core::AiAgent>::set_version | pub | game-ai\src\lib.rs:429 | True | fn(&mut game_ai::AgentVerHamster, usize) |
| 46 | <game_ai::AgentVerHamster as game_core::AiAgent>::debug_mut | pub | game-ai\src\lib.rs:430 | True | fn(&mut game_ai::AgentVerHamster) -> &mut game_core::DebugFrameData |
| 47 | <game_ai::AgentVerHamster as game_core::AiAgent>::small_action_current | pub | game-ai\src\lib.rs:431 | False | fn(&game_ai::AgentVerHamster) -> game_core::SmallAction |
| 48 | <game_ai::AgentVerHamster as game_core::AiAgent>::get_input | pub | game-ai\src\lib.rs:433 | False | fn(&mut game_ai::AgentVerHamster, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> (std::option::Option<game_core::Input>, bumpalo::collections::vec::Vec< game_core::TurnEvent>) |
| 49 | <game_ai::AgentVerHamster as game_core::AiAgent>::buy_item | pub | game-ai\src\lib.rs:437 | False | fn(&mut game_ai::AgentVerHamster, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::GameContext) -> std::option::Option<usize> |
| 50 | <game_ai::AgentVerHamster as game_core::AiAgent>::upgrade_item | pub | game-ai\src\lib.rs:440 | False | fn(&mut game_ai::AgentVerHamster, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::GameContext) -> std::option::Option<(usize, usize)> |
| 51 | <game_ai::AgentVerHamster as game_core::AiAgent>::update_on_dead | pub | game-ai\src\lib.rs:443 | False | fn(&mut game_ai::AgentVerHamster, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) |
| 52 | <game_ai::AgentVerHamster as game_core::AiAgent>::plan_goal | pub | game-ai\src\lib.rs:447 | False | fn(&game_ai::AgentVerHamster) -> std::option::Option<game_core::BigGoal> |
| 53 | <game_ai::AgentVerHamster as game_core::AiAgent>::plan_debug_label | pub | game-ai\src\lib.rs:448 | False | fn(&game_ai::AgentVerHamster) -> std::string::String |
| 54 | <game_ai::AgentVerHamster as game_core::AiAgent>::push_game_event | pub | game-ai\src\lib.rs:449 | False | fn(&mut game_ai::AgentVerHamster, game_core::TurnEvent) |
| 55 | <game_ai::AgentVerHamster as game_core::AiAgent>::push_pending_trace_event | pub | game-ai\src\lib.rs:450 | False | fn(&mut game_ai::AgentVerHamster, usize, game_core::TraceEventType) |
| 56 | <game_ai::AgentVerHamster as game_core::AiAgent>::plan_chats_drain | pub | game-ai\src\lib.rs:453 | False | fn(&mut game_ai::AgentVerHamster) -> std::vec::Vec<game_core::Chat, std::alloc::Global> |
| 57 | <game_ai::AgentVerHamster as game_core::AiAgent>::plan_pending_trace_events_drain | pub | game-ai\src\lib.rs:454 | False | fn(&mut game_ai::AgentVerHamster) -> std::vec::Vec<game_core::PendingTraceEvent, std::alloc::Global> |
| 58 | <game_ai::AgentVerHamster as game_core::AiAgent>::plan_epic_steal_attempt_count | pub | game-ai\src\lib.rs:458 | True | fn(&game_ai::AgentVerHamster) -> usize |
| 59 | <game_ai::AgentVerHamster as game_core::AiAgent>::plan_epic_steal_success_count | pub | game-ai\src\lib.rs:459 | True | fn(&game_ai::AgentVerHamster) -> usize |
| 60 | <game_ai::AgentVerHamster as game_core::AiAgent>::plan_serpen_steal_attempt_count | pub | game-ai\src\lib.rs:460 | True | fn(&game_ai::AgentVerHamster) -> usize |
| 61 | <game_ai::AgentVerHamster as game_core::AiAgent>::plan_serpen_steal_success_count | pub | game-ai\src\lib.rs:461 | True | fn(&game_ai::AgentVerHamster) -> usize |
| 62 | <game_ai::AgentVerHamster as game_core::AiAgent>::plan_steal_sessions | pub | game-ai\src\lib.rs:462 | False | fn(&game_ai::AgentVerHamster) -> std::vec::Vec<game_core::StealSession, std::alloc::Global> |
| 63 | <game_ai::AgentVerHamster as game_core::AiAgent>::plan_gank_attempt_count | pub | game-ai\src\lib.rs:463 | True | fn(&game_ai::AgentVerHamster) -> usize |
| 64 | <game_ai::AgentVerHamster as game_core::AiAgent>::plan_gank_request_count | pub | game-ai\src\lib.rs:464 | True | fn(&game_ai::AgentVerHamster) -> usize |
| 65 | <game_ai::AgentVerHamster as game_core::AiAgent>::plan_gank_periods | pub | game-ai\src\lib.rs:465 | True | fn(&game_ai::AgentVerHamster) -> &std::vec::Vec<(usize, usize), std::alloc::Global> |
| 66 | <game_ai::AgentVerHamster as game_core::AiAgent>::plan_gank_score_attempts | pub | game-ai\src\lib.rs:466 | True | fn(&game_ai::AgentVerHamster) -> &std::vec::Vec<(i32, usize, usize), std::alloc::Global> |
| 67 | <game_ai::AgentVerHamster as game_core::AiAgent>::plan_comeback_pick_stats | pub | game-ai\src\lib.rs:467 | False | fn(&game_ai::AgentVerHamster) -> (usize, usize, std::vec::Vec<(i32, u8), std::alloc::Global>) |
| 68 | <game_ai::AgentVerHamster as game_core::AiAgent>::plan_flee_death_retrospects | pub | game-ai\src\lib.rs:468 | True | fn(&game_ai::AgentVerHamster) -> &std::vec::Vec<(usize, u8, u8, u8), std::alloc::Global> |
| 69 | <game_ai::AgentVerHamster as game_core::AiAgent>::plan_v46_flee_stats | pub | game-ai\src\lib.rs:469 | True | fn(&game_ai::AgentVerHamster) -> (usize, usize, usize, usize, usize, usize) |
| 70 | <game_ai::AgentVerHamster as game_core::AiAgent>::plan_v46_flee_episodes | pub | game-ai\src\lib.rs:470 | True | fn(&game_ai::AgentVerHamster) -> &std::vec::Vec<(usize, game_core::LineType, u8), std::alloc::Global> |
| 71 | <game_ai::AgentVerHamster as game_core::AiAgent>::plan_v46_lane_recall_stats | pub | game-ai\src\lib.rs:471 | True | fn(&game_ai::AgentVerHamster) -> (&std::vec::Vec<(usize, game_core::LineType), std::alloc::Global>, usize, usize, usize, usize, usize, usize, usize, usize) |
| 72 | <game_ai::AgentVerHamster as game_core::AiAgent>::plan_v48_dodge_stats | pub | game-ai\src\lib.rs:474 | True | fn(&game_ai::AgentVerHamster) -> (usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) |
| 73 | <game_ai::AgentVerHamster as game_core::AiAgent>::plan_v50_dive_episodes | pub | game-ai\src\lib.rs:477 | True | fn(&game_ai::AgentVerHamster) -> &std::vec::Vec<game_core::V50DiveEpisode, std::alloc::Global> |
| 74 | game_ai::AgentVerHamster::get_play_type | pub | game-ai\src\lib.rs:552 | True | fn(&game_ai::AgentVerHamster) -> game_core::PlayType |
| 75 | game_ai::AgentVerHamster::update_state | pub | game-ai\src\lib.rs:556 | False | fn(&mut game_ai::AgentVerHamster, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bumpalo::collections::vec::Vec< game_core::TurnEvent> |
| 76 | game_ai::AgentVerHamster::update_event | in:game_ai | game-ai\src\lib.rs:612 | False | fn(&mut game_ai::AgentVerHamster, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) |
| 77 | game_ai::AgentVerHamster::update_plan | in:game_ai | game-ai\src\lib.rs:682 | False | fn(&mut game_ai::AgentVerHamster, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> bumpalo::collections::vec::Vec< game_core::TurnEvent> |
| 78 | game_ai::AgentVerHamster::update_plan_lapse | in:game_ai | game-ai\src\lib.rs:688 | False | fn(&mut game_ai::AgentVerHamster, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, bool) -> bumpalo::collections::vec::Vec< game_core::TurnEvent> |
| 79 | game_ai::AgentVerHamster::update_small_action | in:game_ai | game-ai\src\lib.rs:693 | False | fn(&mut game_ai::AgentVerHamster, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) |
| 80 | game_ai::AgentVerHamster::can_skip_eval | in:game_ai | game-ai\src\lib.rs:796 | False | fn(&game_ai::AgentVerHamster, &game_core::PlayerState, &game_core::OperationData) -> bool |
| 81 | game_ai::AgentVerHamster::count_nearby_enemies | in:game_ai | game-ai\src\lib.rs:830 | False | fn(&game_core::Entity, &game_core::PlayerState, &game_core::OperationData) -> u16 |
| 82 | game_ai::AgentVerHamster::get_input | pub | game-ai\src\lib.rs:843 | False | fn(&mut game_ai::AgentVerHamster, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) -> (std::option::Option<game_core::Input>, bumpalo::collections::vec::Vec< game_core::TurnEvent>) |
| 83 | game_ai::AgentVerHamster::update_trace_escape_abandon | in:game_ai | game-ai\src\lib.rs:1088 | False | fn(&mut game_ai::AgentVerHamster, &game_core::OperationData) |
| 84 | game_ai::AgentVerHamster::update_on_dead | pub | game-ai\src\lib.rs:1113 | False | fn(&mut game_ai::AgentVerHamster, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData) |
| 85 | game_ai::AgentVerHamster::push_game_event | pub | game-ai\src\lib.rs:1120 | False | fn(&mut game_ai::AgentVerHamster, game_core::TurnEvent) |
| 86 | game_ai::AgentVerHamster::item_v26_slot | in:game_ai | game-ai\src\lib.rs:1125 | False | fn(&game_core::PlayerState, &[std::boxed::Box<dyn [Binder { value: Trait(game_core::ItemInfo), bound_vars: [] }] + 'static, std::alloc::Global>]) -> std::option::Option<(usize, std::option::Option<usize>)> |
| 87 | game_ai::AgentVerHamster::item_v26 | in:game_ai | game-ai\src\lib.rs:1149 | False | fn(&mut game_ai::AgentVerHamster, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::GameContext, usize, std::option::Option<usize>) -> std::option::Option<usize> |
| 88 | game_ai::AgentVerHamster::buy_item | pub | game-ai\src\lib.rs:1173 | False | fn(&mut game_ai::AgentVerHamster, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::GameContext) -> std::option::Option<usize> |
| 89 | game_ai::AgentVerHamster::upgrade_item | pub | game-ai\src\lib.rs:1193 | False | fn(&mut game_ai::AgentVerHamster, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::GameContext) -> std::option::Option<(usize, usize)> |

**`open` 19건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | (배치 M) AthleteParameter::input_delay_min/max 의 단위(÷100 후 틱) — game_core 본문 미열람(경계 콜리 · 시그니처만) | 4 |  |
| 1 | 미탐색 | (배치 M) SmallActionRunAway::new(data, player, 5) 의 5 의 의미 — r14 소관(계약만) | 4 |  |
| 2 | 미탐색 | (배치 M) L932 v3_repair_done 앞의 rnd.clone()(%11)이 결과 미사용인데 호출은 남아 있음 — 소스는 `&mut rnd.clone()` 전달로 추정(ArgumentPromotion 으로 콜리 인자에서 소거). 극성·동작엔 무관 | 5 |  |
| 3 | 미탐색 | (배치 M) buy_item/upgrade_item 의 IR 인자 1·4·5 가 poison — 콜리가 version·game 을 안 읽는다는 뜻(DeadArgElim). 소스 호출식은 (version, rnd.clone(), player, game, ctx) 로 추정(tcx 시그니처 순서) | 3 |  |
| 4 | 재료 부재 | (배치 M) L896 DI 이름 `in_fountain` 이 `hp < stat_cached.hp` 에 붙어 있음 — 소스 변수명이 이 뜻인지(분수 회복 중) 확인 불가(column 0). 동작은 IR 문면대로 기록 | 4 |  |
| 5 | 표기 불가 | (배치 M) L923 `stays` 의 None 경로가 L921 의 Stop 비교를 거치지 않고 참(true)인 점 — switch 에서 -1 이 %239 로 직행. 소스가 `input.map_or(true, ..)` 인지 match 인지는 표기 불가(동작 확정) | 4 |  |
| 6 | 미탐색 | (배치 M) StayEvent 의 goal·문자열·hp_pct·near_enemy/near_ally·since_combat·in_lapse·target_kind 값 — 배치 N(줄 1009~1043) 계산. 여기선 슬롯 오프셋과 phi 이름(%851/%849/%850/%23/%22/%21/%17/%860/%899/%1052/%1060/%1053/%1079/%1080)만 적음 | 4 |  |
| 7 | 미탐색 | (배치 M) L1005 `tick()` 5번째 호출은 블록 %630(배치 M 소속)에 있으나 소스 줄 1005 — StayEvent.tick 값. 배치 N 과 중복 기술될 수 있음(mergespec 에서 하나로) | 4 |  |
| 8 | 미탐색 | (배치 M) 루트 줄 0(`;L0`) 을 가진 phi 9줄(블록 1059/1071 등)은 cuts 상 배치 M 이나 실제 소속 블록은 배치 N(줄 1026/1032) — 이 명세에서 제외 | 4 |  |
| 9 | 미탐색 | (배치 N) L1018~1019 `data.blackboard[1 - team]`(적팀 인덱스) 로 is_recent_visible 을 부르는 의미론 — IR 사실(m14.ll:40826~40828 · getelementptr [Blackboard] %864, %861=1-team)만 기록. Blackboard[t] 가 '팀 t 소속 엔티티에 대한 관측 기록'인지 '팀 t 가 본 기록'인지는 이 범위에서 확정 불가 — 미탐색 = _gcbc g07.ll:157005 is_recent_visible 본문(last_visible[position] 인덱싱 방향) | 4 |  |
| 10 | 미탐색 | (배치 N) L1037 `camp_type.0 < 2` 의 의미(정글몹 vs 미니언 라벨 분기): Jungle.camp_type = (usize, JungleType) 의 .0 이 무엇을 세는지(캠프 인덱스? 등급?) — _docs 에 camp_type 주석 0건. 표기 불가 아님 · 미탐색 = game_core jungle.rs:13 Jungle 정의·생성처 | 4 |  |
| 11 | 표기 불가 | (배치 N) L1011 short() 술어가 `c==' ' \|\| c=='('` 인지 `c=='(' \|\| c==' '` 인지 — 컴파일러가 마스크 비교(2097143/32) 한 개로 접어 순서는 표기 불가(외연 동일) | 4 |  |
| 12 | 미탐색 | (배치 N) L1013 target_position 이 AroundRegion(idx4)·AroundRunAway(idx5) 에 None 을 주는 소스 의도 — IR 사실만(둘 다 goal_x/y 필드가 있는데 안 읽음) | 4 |  |
| 13 | 미탐색 | (배치 N) L1064 우물 사각(fountains[team])이 (좌x, 좌y, 우x, 우y) 순인지 — IR 은 4 u64 를 (flx,fly,frx,fry) DI 이름으로 로드하고 x∈[flx,frx]·y∈[fly,fry] 로 비교(41815~41821) · MapDef 필드명 fountains [(u64,u64,u64,u64);2] 까지 확인 · 각 성분 의미는 DI 이름 근거 | 4 |  |
| 14 | 미탐색 | (배치 N) %1053(in_lapse)·%425(last_lapse) 가 update_state(L854 · 배치 M)에서 이번 호출에 갱신된 값인지 — 배치 M 소관(0x29ca 는 update_state writes 에 있음) | 4 |  |
| 15 | 미탐색 | (배치 N) premise_lost(%436) 의 계산 조건·호출 줄 — 배치 M(L~964 is_premise_lost invoke) 소관 · 여기선 값만 소비(L1075) | 4 |  |
| 16 | 미탐색 | (배치 N) StayEvent.kind(%580)·can_buy(%631)·pos_band(%632)·position(%225)·team(%221) 값의 계산 — 배치 M(L914 · 985~1003) 소관 | 4 |  |
| 17 | 미탐색 | (배치 N) `Option<Input>` 의 Return(1)·None(-1) 경로에서 +8..+32 에 남는 값 — 이 함수는 %47 32B 를 통째 memcpy 하므로 배치 M 이 %47 에 무엇을 썼는지에 달림(패딩 undef 가능) · sweep 대조 시 태그별 live 만 비교할 것 | 4 |  |
| 18 | 미탐색 | (배치 N) calls 의 `core::ops::drop::Drop::drop`(m14.ll:40529·40535·40543) 실체 — String(%20 · short() 원본) 의 Vec<u8> Drop · 심볼이 제네릭 인스턴스 이름이라 타입은 DI(`;L825<825<1000`) 로만 추정 | 4 |  |

**`notes` 1건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | (배치 N) C3 경고 5건(0x28b1·0x28a8·0x28b0·0x28c0·0x28c8)은 정상: 태그는 %210(배치 M 의 gep 10505) 재로드, 페이로드는 %89(+10328) 기준 상대 gep(8/16/24/32/80/88/104/112 · m14.ll:40797~40798 phi) 로 접근돼 절대 오프셋 리터럴이 본문에 없다 | 4 | 사실 서술 |

<details><summary>`closed` 1건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

| # | 물음 | 닫은 근거 |
|---|---|---|
| 0 | (배치 M) SmallActionPlay::get_input · v3_fall_back_to_passive 의 마지막 인자에 %1(self 포인터)이 그대로 들어감 — 두 tcx 시그니처 모두 &mut DebugFrameData(224B) 이고 self.debug 가 오프셋 0 이라 self.debug 로 확정(&mut self 와 &mut self.debug 의 동시 대여는 필드 분할 대여로 성립) | 본문에 해소 표기가 있다 |
</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

