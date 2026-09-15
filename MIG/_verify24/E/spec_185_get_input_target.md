---

### `185` get_input_target — 스킬/평타 effect 의 casting 종류별로 실제 입력 대상(Target/Pos/Dir/None)을 산출 — 예측 이동·오판·조준 오차 포함

| 항목 | 값 |
|---|---|
| id | `abstract_input__get_input_target` |
| 심볼 | `_RNvNtCshdEBA0ozCnw_7game_ai14abstract_input16get_input_target` |
| 소스 | `game-ai\src\abstract_input.rs:345` |
| IR | `m04.ll` 33111~35159행 |
| 경로·가시성 | `game_ai::abstract_input::get_input_target` · **in:game_ai::abstract_input** |
| 계층 | 입력 생성 |
| exe | `d31f20` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r15` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &mut rand::rngs::std::StdRng, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, &game_core::Entity, &game_core::Effect, usize) -> std::option::Option<game_core::InputTarget>
```

<details><summary>인자 9개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) | *mut Option<InputTarget>(24B) | +0 i32 tag: -1(0xFFFFFFFF)=None / 0=Target / 1=Dir / 2=Pos / 3=InputTarget::None(페이로드 없음). DWARF DISCR_EXACT 4294967295 (m04.ll:79376). ⚠지시문의 「None=+0 8B」는 4B 가 맞다(store i32 -1 · m04.ll:33230) | 3 |
| 1 | 1 | version | usize | 이 함수 안 분기 없음. position_score_at_cell 로 전달만(24회) | 4 |
| 2 | 2 | rnd | &mut StdRng(320B) | gen_range 5지점 + apply_aim_offset_* 내부 소비 — writes 참조 | 4 |
| 3 | 3 | player | &PlayerState(2528B) | readonly · 0x930 team · 0x9c0 position tag · 0x180 parameter(skill_hit_accuracy/effective 인자) · 0x108 stat.skill_hit(트레이스 전용) | 4 |
| 4 | 4 | data | &OperationData(24B) | readonly · +0 cache(&AbstractGameWithCache) · +8 context(&GameContext) | 4 |
| 5 | 5 | positioning_score | &PositioningScoreData(2760B) | readonly · 이 함수는 +0xab8 cx · +0xac0 cy 두 필드만 load(m04.ll:33435/33438/33678/33681) · value[7][7] 은 안 읽음 · position_score_at_cell 에 ptr 로 전달되지만 그 콜리도 `_positioning_score` 미사용(m07.ll:24714 본문 gep 0건) | 4 |
| 6 | 6 | target | &Entity(1728B) | readonly · 입력 대상 엔티티 | 4 |
| 7 | 7 | effect | &Effect(56B) | readonly · +0/+8 ty(Arc<dyn EffectType> 데이터·vtable) · +0x20 start_timing · +0x30 casting tag · is_in_range 인자 | 4 |
| 8 | 8 | speed | usize | 시전 속도 배율(‰ 분모). 0 이면 L364 udiv 패닉(m04.ll:33334) | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
// L346  team=player.info.team (<2 아니면 bounds 패닉); pos=player.info.position(tag)
// caster = data.cache.player_champion[team][pos]; None → return None
// L348  if !effect.is_in_range(caster, target) → return None (L349)
// L353  if caster.team is Player(tag0) { t=caster.team.0 (<2 bounds); if target.visible_state[t] != Visible(0) { if effect.casting ∉ {Position(1), Direction(2)} → return None (L354) } }   // Neutral 캐스터는 검사 생략
// L357  acc = player.parameter.skill_hit_accuracy();  L358 eff = skill_hit_effective()
// L360  mis = 1000 - acc   (오판 임계, ‰)
// L361-362 te = max(1000-acc,1); min=1000-te; max=1000+te
// L364  start_timing = gen_range(min..=max) * (effect.start_timing*100 / speed) / 1000   // speed==0 → div0 패닉
// L368  lead = start_timing; if get_game_mode()==DeathMatch(2) { if let Some(psp)=effect.ty.linear_move_speed() { lead += distance(caster,target)/max(psp,1) } }   // ★gamemode=0 접기: 블록 93/108/121 사장 → lead = start_timing
// L377  match effect.casting {
//  Targeting(0) → Some(Target(target.id))                                  // L652
//  None(3)      → Some(InputTarget::None)                                  // L653 (tag 3)
//  Position(1) →
//    L534 if effect.ty.auto_target() {
//      L537 best=None; for dx in 0..6 { for dy in 0..6 { s=position_score_at_cell(version,player,data,ps, cx-3+dx, cy-3+dy, AttackStance(12)); v = s.gain - s.risk; if best.is_none() || best.v < v { best=(v,dx,dy) } } }   // 첫 최대 유지(strict <)
//      L551 x = clamp(cx-3+bdx, 0, 29)*32000+16000; y = clamp(cy-3+bdy,0,29)*32000+16000; return Some(Pos(x,y))   // L554 · 조준 오차 없음
//    } else if target.ty==Champion(13) && target.action_state==Move(2) {   // L556
//      (x,y) = action_state.Move.{x,y}
//      L562 misjudged = gen_range(0..=1000) < mis
//      L565 if misjudged { if eff>39 && DeathMatch { (x,y)=target.pos } else { (x,y)=adjust_position(map,setting, 2*caster.x - x, 2*caster.y - y) } }   // 거울 반전 오판
//      L577 tx=target.x; ty=target.y; move_to(setting,map_setting,map,target.id,&mut tx,&mut ty, target.stat_cached.move_speed*lead, x,y, None)
//      L582 return apply_aim_offset_pos(rnd,setting,map,acc, caster.x,caster.y, tx,ty)
//    } else { match target.rush_state {                                    // L592
//      Move|Rush|RushPenetrate → (speed,x,y) = 각 variant 의 speed/x/y
//        L596 misjudged = gen_range(0..=1000) < mis; L599 오판 처리 위와 동일(대상 현재위치 / 거울)
//        L610 if target is Champion && nt_trace_on() → record_aim(...)   // 텔레메트리
//        L624 dist = distance(x,y, target.x,target.y); L625 tick = dist/speed (speed==0 → div0 패닉)
//        L626 if tick < lead { d=(x,y)-caster.pos; sz=max(isqrt(dx²+dy²),1); (x,y) = caster.pos + d*lead/sz }   // 돌진이 lead 안에 끝나면 같은 방향으로 lead 만큼 연장한 점
//        L638 return apply_aim_offset_pos(rnd,setting,map,acc, caster.x,caster.y, x,y)
//      None|MoveToTarget → L645-647 return apply_aim_offset_pos(..., target.x, target.y)
//    } }
//  Direction(2) →
//    L379 if auto_target { L382-387 같은 6×6 스캔(purpose 12, v=gain-risk, 첫 최대) ; L396-401 return Some(Dir(cell_x*32000 - caster.x + 16000, cell_y*32000 - caster.y + 16000)) }   // 오차 없음
//    else if Champion && action_state==Move { (x,y)=Move dest; tx,ty=target.pos; L413 misjudged=gen_range(0..=1000)<mis; L417 if misjudged { if eff>39 && DM {(x,y)=target.pos} else {거울 adjust_position} }; L429 move_to(..,&mut tx,&mut ty, move_speed*lead, x,y); L432 if nt_trace_on() record_aim; L446 return apply_aim_offset_dir(rnd,acc, tx-caster.x, ty-caster.y) }
//    else match rush_state { Move|Rush|RushPen → L462 misjudged / L465 오판 / L476 trace(Champion) / L490 dist / L491 tick=dist/speed / L492 if tick<lead 연장 / L504-506 return apply_aim_offset_dir(rnd,acc, x-caster.x, y-caster.y)
//                            None|MoveToTarget → L513 trace(Champion, dest None) ; L526-528 return apply_aim_offset_dir(rnd,acc, target.x-caster.x, target.y-caster.y) }
// }
```

**`mem` 메모리 접근 45건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | L346 · <2 bounds check · player_champion[team] 인덱스 · target.visible_state[team] 인덱스는 caster.team.0 을 씀 | 4 | OK |  |
| 1 | PlayerState | 0x9c0 | info.position@tag | r | L346 · i32 zext · player_champion[team][position] | 4 | OK |  |
| 2 | PlayerState | 0x180 | info.parameter | r | L357/358 · &AthleteParameter 를 skill_hit_accuracy/skill_hit_effective 에 전달 | 4 | OK |  |
| 3 | PlayerState | 0x108 | info.stat.skill_hit | r | L441/485/522/619 · NtAimInfo.skill_hit_raw 트레이스 전용(판정 무관) | 4 | OK |  |
| 4 | OperationData | 0x0 | cache | r | &AbstractGameWithCache | 4 | OK |  |
| 5 | OperationData | 0x8 | context | r | &GameContext | 4 | OK |  |
| 6 | AbstractGameWithCache | 0x0 | game | r | &dyn AbstractGame 팻포인터(+0 data, +8 vtable) · vtable+0x40 get_game_mode · vtable+0x28 tick(트레이스) | 4 | OK |  |
| 7 | AbstractGameWithCache | 0x1e0 | player_champion[team][position] | r | L346 · gep 480 + team*40 + pos*8 · Option<&Entity> null=None → 즉시 None 반환. 이것이 caster(%39) | 4 | OK |  |
| 8 | GameContext | 0x8 | setting | r | &GameSetting(5432B) → move_to / adjust_position / apply_aim_offset_pos | 4 | OK |  |
| 9 | GameContext | 0x18 | map_setting | r | &MapSetting(88B) → move_to | 4 | OK |  |
| 10 | GameContext | 0x20 | map | r | &MapDef(28112B) → move_to / adjust_position / apply_aim_offset_pos | 4 | OK |  |
| 11 | Effect | 0x0 | ty.ptr | r | Arc<dyn EffectType> 데이터 포인터 · ArcInner 데이터 = ptr + ((vtable.align-1)&~15) + 16 | 4 | OK |  |
| 12 | Effect | 0x8 | ty.vtable | r | +0x10 align · +0xd8 auto_target(bool) · +0xf8 linear_move_speed(Option<usize>) | 4 | 부분일치(첫 성분(컨테이너)만 맞고 **마지막 성분이 다르다** — 사람이 봐야 ) |  |
| 13 | Effect | 0x20 | start_timing | r | L364 · start_timing*100/speed | 4 | OK |  |
| 14 | Effect | 0x30 | casting@tag | r | CastingType 0 Targeting/1 Position/2 Direction/3 None · L149<353 (casting-1 <u 2) · L377 switch | 4 | OK |  |
| 15 | Entity | 0x0 | team@tag | r | caster · TeamType 0 Player/1 Neutral · Neutral 이면 가시성 검사 생략 | 4 | OK |  |
| 16 | Entity | 0x8 | team@Player.0 | r | caster 팀 index · target.visible_state[team] 인덱스 | 4 | OK |  |
| 17 | Entity | 0x38 | visible_state[team]@tag | r | target · 0x38 + team*24 (gepS stride 24) · 0=Visible | 4 | OK |  |
| 18 | Entity | 0x68 | ty@tag | r | target · ==13 Champion | 4 | OK |  |
| 19 | Entity | 0x70 | ty@Champion.0.action_state@tag | r | target · ==2 Move | 4 | OK |  |
| 20 | Entity | 0x78 | ty@Champion.0.action_state@Move.x | r | target 이동 목적지 x | 4 | OK |  |
| 21 | Entity | 0x80 | ty@Champion.0.action_state@Move.y | r | target 이동 목적지 y | 4 | OK |  |
| 22 | Entity | 0x308 | rush_state@tag | r | target · 니치: 음수면 xor 0x8000000000000000 → idx(0 None/1 Move/2 MoveToTarget/3 Rush), 비음수 = 4 RushPenetrate | 4 | OK |  |
| 23 | Entity | 0x340 | rush_state@Move.speed | r | phi 접힘 832 · Move | 4 | OK |  |
| 24 | Entity | 0x348 | rush_state@Move.x | r | 840 | 4 | OK |  |
| 25 | Entity | 0x350 | rush_state@Move.y | r | 848 | 4 | OK |  |
| 26 | Entity | 0x328 | rush_state@Rush.speed | r | 808 | 4 | OK |  |
| 27 | Entity | 0x330 | rush_state@Rush.x | r | 816 | 4 | OK |  |
| 28 | Entity | 0x338 | rush_state@Rush.y / RushPenetrate.speed | r | 824 — Rush.y 이자 RushPenetrate.speed(variant 별 phi) | 4 | OK |  |
| 29 | Entity | 0x340 | rush_state@RushPenetrate.x | r | 832 | 4 | OK |  |
| 30 | Entity | 0x348 | rush_state@RushPenetrate.y | r | 840 | 4 | OK |  |
| 31 | Entity | 0x5c0 | id | r | target.id → InputTarget::Target / move_to 인자 / 트레이스 · caster.id → 트레이스 키 | 4 | OK |  |
| 32 | Entity | 0x640 | stat_cached.move_speed | r | target · move_to 이동량 = move_speed * lead_timing | 4 | OK |  |
| 33 | Entity | 0x660 | x | r | caster/target 현재 x | 4 | OK |  |
| 34 | Entity | 0x668 | y | r | caster/target 현재 y | 4 | OK |  |
| 35 | PositioningScoreData | 0xab8 | cx | r | auto_target 셀 스캔 중심 x(셀) · exe 에선 승격 스칼라 +0x28 | 4 | OK |  |
| 36 | PositioningScoreData | 0xac0 | cy | r | auto_target 셀 스캔 중심 y(셀) · exe 에선 승격 스칼라 +0x30 | 4 | OK |  |
| 37 | PositioningScore | 0x0 | risk | r | position_score_at_cell sret(56B) · v = gain - risk | 4 | OK |  |
| 38 | PositioningScore | 0x10 | gain | r | position_score_at_cell sret · v = gain - risk | 4 | OK |  |
| 39 | Option<InputTarget>(sret) | 0x0 | tag | w | None: m04.ll:33230(caster 없음 L346)·33237(사거리 밖 L349)·33292(비가시+Targeting/None L354). Target: 33412(L652). InputTarget::None: 33904(L653). Pos: 34483(L554 auto_target 셀). Dir: 35153(L401 auto_target 셀). 그 외 Pos/Dir 은 apply_aim_offset_pos/dir 이 sret 에 직접 기록 | 4 | 확인불가(tcx 사전에 타입 없음) | -1 / 0 / 1 / 2 / 3 |
| 40 | Option<InputTarget>(sret) | 0x8 | payload.0 | w | 33414 / 34485 / 35155 + 콜리 | 4 | 확인불가(tcx 사전에 타입 없음) | target_id \| x \| dir_x |
| 41 | Option<InputTarget>(sret) | 0x10 | payload.1 | w | 34487 / 35157 + 콜리 · Target(tag0) 경로는 미기록 | 4 | 확인불가(tcx 사전에 타입 없음) | y \| dir_y |
| 42 | StdRng(rnd) | 0x0 | rng 상태 전진 | w | ★소비 순서(모두 조건부): ①L364 gen_range(1000-te ..= 1000+te) — caster 존재·사거리내·가시성 게이트 통과 후 무조건 1회(m04.ll:33318) ②Position·Champion-Move: L562 gen_range(0..=1000)(33943) ③Position·rush(Move/Rush/RushPen): L596(34083) ④Direction·Champion-Move: L413(34536) ⑤Direction·rush: L462(34693) ⑥apply_aim_offset_pos(L582/638/647)·apply_aim_offset_dir(L448/506/528) 내부: acc≤999 && 오프셋≠0 이면 gen_range(0..=1000-acc) 1회 + u32 워드 ≥1개(bit30 거부표본, m04.ll:39226~/39342~). auto_target 셀 경로·Targeting·None 경로는 ① 뒤 추가 소비 없음 | 4 | OK | gen_range::<usize,RangeInclusive> |
| 43 | stack | - | target_x/target_y (&mut u64) | w | %14/%13(L577-579) · %24/%23(L408-429) · frame 인자는 null(Option<&mut>=None) | 4 | 확인불가(오프셋 파싱 실패) | target.x/y → move_to 가 예측 위치로 갱신 |
| 44 | global(game_core) | - | nt_trace_record_aim(caster.id, NtAimInfo 88B) | w | nt_trace_on() 참일 때만 · Position: L611(rush, target Champion) / Direction: L433(Champion-Move 무조건)·L477(rush, Champion)·L514(None/MoveToTarget, Champion) · 판정 무관 | 4 | 확인불가(오프셋 파싱 실패) | 텔레메트리 |

**`consts` 상수 17건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev | folded_from |
|---|---|---|---|---|---|---|
| 0 | 2 | 346 | 인덱스 | team 인덱스 bounds(<2) · L1483<353 도 동일 · get_game_mode()==2 DeathMatch(L368/565/599/417/465) · casting-1 <u 2 (L149<353: casting∈{Position,Direction}) · action_state tag ==2 Move(L556/403) · InputTarget::Pos tag(L554) | 4 |  |
| 1 | 0 | 353 | 임계 | visible_state tag ==0 Visible · InputTarget::Target tag(L652) · rush None 경로 · gen_range 하한 | 4 |  |
| 2 | -1 | 353 | 계수 | casting-1 (L149) · Option::None tag store i32 -1 (L346/349/354) | 4 |  |
| 3 | 1000 | 360 | 계수 | 1000 - skill_hit_accuracy = 오판 확률(‰) · L361/362 timing 범위 중심 · L364 /1000 · L391 gen_range 상한 0..=1000 | 4 |  |
| 4 | 1 | 360 | 태그 | timing_error = max(1000-acc, 1) (umax) · psp max(psp,1) · rush Move tag 1 · InputTarget::Dir tag(L401) · NtAimInfo.target_dest Some tag · ★거울 반전 `2*caster.x - x`(L571/604/423/470) 의 2 가 `shl i64 %x, 1` 로 접힘(folded_from 2) | 4 | 2 |
| 5 | 100 | 364 | 계수 | start_timing*100/speed — effect.start_timing 을 speed(‰) 로 환산 | 4 |  |
| 6 | 12 | 539 | 센티널 | PositionEvalPurpose::AttackStance 메모리 태그 12 (tcxdict --enum: idx 10, 니치 start 2) · L384 도 동일 | 3 |  |
| 7 | 3 | 537 | 태그 | 셀 스캔 시작 = (cx-3, cy-3) · L551/552/396/397 결과 환산 dx-3 · CastingType::None tag(L377→L653 store i32 3)· rush Rush idx 3 | 4 |  |
| 8 | 6 | 537 | 태그 | 스캔 범위 0..6 (dx,dy ∈ 0..=5 → 6×6 셀, cx-3..cx+2 — 7×7 배열이지만 +3 열/행은 안 본다) · 내부 dy 루프는 6회 언롤 | 4 |  |
| 9 | 29 | 551 | 인덱스 | 셀 인덱스 clamp 상한 (0..=29 = 30 그리드) · L552/396/397 | 4 |  |
| 10 | 32000 | 551 | 미상 | 셀 → 월드 좌표 변환(셀 크기) | 4 |  |
| 11 | 16000 | 551 | 계수 | 셀 중심 오프셋 · Dir 경로(L399/400)는 cell*32000 - caster + 16000 | 4 |  |
| 12 | 13 | 556 | 태그 | EntityType::Champion 태그 · L403 도 동일 | 4 |  |
| 13 | 39 | 565 | 임계 | caster_skill_hit(skill_hit_effective, 0..100) > 39 && DeathMatch → 오판 시 대상 현재 위치로 대체(거울 대신) · L599/417/465 동일 | 4 |  |
| 14 | -9223372036854775808 | 592 | 계수 | RushState 니치 start(0x8000000000000000) — tag xor 로 논리 idx 복원 · L458 동일 | 4 |  |
| 15 | 4 | 592 | 태그 | RushState untagged variant RushPenetrate 의 논리 idx(비음수 tag) | 4 |  |
| 16 | 5 | 544 | 산출값 | 언롤 dy 마지막 인덱스(select i32 5) · 스캔 결과 dy 값 0..5 | 4 |  |

**`knobs` 조정점 5건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 오판 확률·타이밍 오차 폭 = 1000 - skill_hit_accuracy | abstract_input.rs:360-362 | 1000 | acc 가 낮을수록 (a) 시전 타이밍 gen_range 폭이 넓어지고 (b) 이동 예측이 거울 반전으로 오판될 확률이 오른다 · acc≥1000 이면 오차 0 | 4 | 기존 |
| 1 | DeathMatch 오판 완화 게이트 | abstract_input.rs:565/599/417/465 | 39 | eff>39 이면 오판 시 거울 반전 대신 대상 현재 위치를 씀(덜 나쁜 오판) · Moba(gamemode 0)에선 항상 거울 반전 | 4 | 기존 |
| 2 | auto_target 셀 스캔 창 | abstract_input.rs:537/382 | 6 | cx-3..cx+2 · 키우면 더 먼 셀까지 조준 후보(7 이면 대칭 7×7) | 4 | 기존 |
| 3 | 셀 스캔 purpose | abstract_input.rs:539/384 | 12 | AttackStance 태그 · 바꾸면 position_eval_at 의 가중 프로파일이 바뀐다 | 4 | 기존 |
| 4 | 돌진 연장 조건 tick<lead | abstract_input.rs:626/492 | 0 | 돌진이 스킬 도달 전에 끝나면 caster→dest 방향으로 lead 거리만큼 연장한 점을 조준 (상수 없음 · 구조 노브) | 4 | 기존 |

<details><summary>`callees` 피호출자 27건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | adjust_position | game_core::Game::adjust_position | pub | fn(&game_core::MapDef, &game_core::GameSetting, i64, i64) -> (u64, u64) | game-core\src\simulation\game.rs:818 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | apply_aim_offset_dir | game_ai::abstract_input::apply_aim_offset_dir | in:game_ai::abstract_input | fn(&mut rand::rngs::std::StdRng, usize, i64, i64) -> game_core::InputTarget | game-ai\src\abstract_input.rs:11 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 2 | apply_aim_offset_pos | game_ai::abstract_input::apply_aim_offset_pos | in:game_ai::abstract_input | fn(&mut rand::rngs::std::StdRng, &game_core::GameSetting, &game_core::MapDef, usize, &game_core::Entity, u64, u64) -> game_core::InputTarget | game-ai\src\abstract_input.rs:25 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 3 | auto_target | game_core::EffectType::auto_target | pub | fn(&Self/#0) -> bool | game-core\src\simulation\effect\type.rs:337 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 4개 중 상위 3개 |
| 4 | auto_target | <game_core::CombineEffect as game_core::EffectType>::auto_target | pub | fn(&game_core::CombineEffect) -> bool | game-core\src\simulation\effect\type\combine.rs:119 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 4개 중 상위 3개 |
| 5 | auto_target | <game_core::RandomTargetEffect as game_core::EffectType>::auto_target | pub | fn(&game_core::RandomTargetEffect) -> bool | game-core\src\simulation\effect\type\random_target.rs:110 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 4개 중 상위 3개 |
| 6 | distance | game_core::utils::distance | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:12 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 7 | get_game_mode | game_core::AbstractGame::get_game_mode | pub | fn(&Self/#0) -> game_core::GameMode | game-core\src\simulation.rs:78 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 8 | get_game_mode | <game_core::Game as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::Game) -> game_core::GameMode | game-core\src\simulation\game.rs:1707 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 9 | get_game_mode | <game_core::SingleLaneGame as game_core::AbstractGame>::get_game_mode | pub | fn(&game_core::SingleLaneGame) -> game_core::GameMode | game-core\src\simulation\game.rs:4248 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 10 | is_in_range | game_core::Effect::is_in_range | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity) -> bool | game-core\src\simulation\effect.rs:63 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 11 | isqrt | game_core::utils::isqrt | pub | fn(i64) -> i64 | game-core\src\utils.rs:85 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 12 | linear_move_speed | game_core::EffectType::linear_move_speed | pub | fn(&Self/#0) -> std::option::Option<usize> | game-core\src\simulation\effect\type.rs:347 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 14개 중 상위 3개 |
| 13 | linear_move_speed | <game_core::RushEffect as game_core::EffectType>::linear_move_speed | pub | fn(&game_core::RushEffect) -> std::option::Option<usize> | game-core\src\simulation\effect\type\rush.rs:63 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 14개 중 상위 3개 |
| 14 | linear_move_speed | <game_core::MoveToEffect as game_core::EffectType>::linear_move_speed | pub | fn(&game_core::MoveToEffect) -> std::option::Option<usize> | game-core\src\simulation\effect\type\move_to.rs:70 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 14개 중 상위 3개 |
| 15 | move_to | game_core::Entity::move_to | pub | fn(&game_core::GameSetting, &game_core::MapSetting, &game_core::MapDef, usize, &mut u64, &mut u64, u64, u64, u64, &mut std::option::Option<&mut game_core::GameFrameData>) | game-core\src\simulation\entity.rs:3427 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 16 | nt_trace_on | game_core::nt_trace_on | pub | fn() -> bool | game-core\src\simulation\entity.rs:229 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 17 | nt_trace_record_aim | game_core::nt_trace_record_aim | pub | fn(usize, game_core::NtAimInfo) | game-core\src\simulation\entity.rs:242 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 18 | position | game_core::PlayerAiContext::<'a, 'b, 'r>::position | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> game_core::Position | game-core\src\mod_ai.rs:587 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 19 | position | game_core::transfer::TeamRosterPlanningContext::position | pub | fn(&game_core::transfer::TeamRosterPlanningContext, game_core::Position) -> std::option::Option<&game_core::transfer::PositionPlanningContext> | game-core\src\transfer\roster_context.rs:234 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 2개 중 상위 2개 |
| 20 | position_score_at_cell | game_ai::position_score_at_cell | pub | fn(usize, &game_core::PlayerState, &game_core::OperationData, &game_core::PositioningScoreData, i64, i64, game_ai::PositionEvalPurpose) -> game_core::PositioningScore | game-ai\src\position_eval.rs:1183 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 21 | skill_hit_accuracy | game_core::AthleteParameter::skill_hit_accuracy | pub | fn(&game_core::AthleteParameter) -> usize | game-core\src\simulation\state\player.rs:330 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 22 | skill_hit_effective | game_core::AthleteParameter::skill_hit_effective | pub | fn(&game_core::AthleteParameter) -> usize | game-core\src\simulation\state\player.rs:301 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 23 | speed | game_core::Projectile::speed | pub | fn(&game_core::Projectile) -> u64 | game-core\src\simulation\projectile.rs:227 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
| 24 | team | game_view::ClientData::team | pub | fn(&game_view::ClientData, usize) -> std::option::Option<std::cell::Ref< game_core::Team>> | game-view\src\logic\client\data.rs:1651 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 25 | team | game_view::ClientDatabase::team | pub | fn(&game_view::ClientDatabase, usize) -> std::option::Option<&game_core::Team> | game-view\src\logic\client\data.rs:968 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 26 | team | game_core::PlayerAiContext::<'a, 'b, 'r>::team | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:583 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
</details>

⚠**미매칭 4개**: `clamp`, `gen_range`, `record_aim`, `trace`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 4곳** (m04.ll:44130, m04.ll:44446, m04.ll:44740, m04.ll:45017) · **형제 0개** 

**`open` 5건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | position_score_at_cell 로 넘기는 x,y 순서/좌표 축(xi=cx 기반, yi=cy 기반)은 IR 인자 순서로 확정했으나 PositioningScoreData.cx/cy 가 셀 좌표(0..29)임은 clamp(0,29)·*32000 으로 추정(정의부 미독) | 4 |  |
| 1 | 표기 불가 | 6×6 스캔(0..6)이 의도인지(7×7 배열의 +3 열/행 누락) 소스 표기 불가 — IR 은 dx,dy 각 6회로 확정 | 4 |  |
| 2 | 미탐색 | L629-634 돌진 연장식 `caster + d*lead/\|d\|` 에서 d 의 기준점이 caster(예측점이 아니라)인 것은 IR 확정 — 의도 여부는 소스 주석 부재 | 4 |  |
| 3 | 미탐색 | Effect::is_in_range · Entity::move_to · Game::adjust_position 내부(game_core)는 계약만 적고 안 읽음 | 4 |  |
| 4 | 미탐색 | gamemode=0 접기로 사장 처리한 블록 93/108/121(DeathMatch 비행시간) 및 `DM && eff>39` 게이트의 DM 항은 Moba 에서 항상 거짓 — DeathMatch 검증 시 재개봉 필요 | 4 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

