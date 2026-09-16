---

### `254` noncombat_steroid_window — 비교전 스테로이드 창: 적 챔프가 교전권 밖(최근 비가시)일 때 아군 소액션이 노리는 정글/타워/넥서스/미니언이 수혜자 공격범위+0.5초 안이면 창 종류(반격형/미니언웨이브) 반환

| 항목 | 값 |
|---|---|
| id | `buff_value__noncombat_steroid_window` |
| 심볼 | `_RNvNtCshdEBA0ozCnw_7game_ai10buff_value24noncombat_steroid_window` |
| 소스 | `game-ai\src\buff_value.rs:378` |
| IR | `m10.ll` 36654~37661행 |
| 경로·가시성 | `game_ai::buff_value::noncombat_steroid_window` · **in:game_ai** |
| 계층 | 점수화·술어 |
| exe | `e03360` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r18` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(&game_core::PlayerState, &game_core::OperationData, &game_core::Entity, &game_core::Entity) -> std::option::Option<game_ai::NoncombatSteroidWindow>
```

**TLS 메모 절(`sig.tls` · 정정 경로 `/specs[254]/sig/tls/<키>`)**

TLS 접점 0 — 본문(36654~37661)·aux 4조각 전부에 `call_once`/`LocalKey`/`threadlocal`/fn-포인터 상수 참조 없음. 콜리 is_recent_visible·range_adjust·iter_* 는 game_core(계약만)

<details><summary>인자 4개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 1 | player | &PlayerState(2528B) %0 | define 줄 속성: noalias readonly captures(address, read_provenance). +0x930 info.team 만 직접 읽음(390 · 클로저 안 407 에서도). is_recent_visible 3번째 인자 · team_action_hits 클로저 캡처(%9+16) | 4 |
| 1 | 2 | data | &OperationData(24B) %1 | define 줄 속성: noalias readonly captures(none). +0 cache(&AbstractGameWithCache %86) · +0x10 blackboard(&[Blackboard;2] %98). +8 context 는 안 읽음 | 4 |
| 2 | 3 | caster | &Entity(1728B) %2 | define 줄 속성: noalias readonly captures(none). 시전자. engage 클로저(389: caster_engage) 와 적 근접 판정(393: c.distance_sq(caster)) 에만 쓰임 — in_window 창 계산에는 안 들어감(창의 기준은 beneficiary) | 4 |
| 3 | 4 | beneficiary | &Entity(1728B) %3 | define 줄 속성: noalias readonly captures(address, read_provenance). 수혜자. engage(388) · 적 근접(393) · attack_effect(419 · None 이면 None 반환) · move_allow(420) · in_window 의 사거리·거리 기준(429~431) · team_action_hits 의 아군 근접 기준(408) | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
fn noncombat_steroid_window(player, data, caster, beneficiary) -> Option<NoncombatSteroidWindow>  [buff_value.rs:378~452]

384~387: let engage = |e: &Entity| -> u64 {                    // 클로저#0 · 전부 인라인
           e.attack_effect.as_ref().map(|ef| ef.range(e)).unwrap_or(0)   // Effect::range(effect.rs:26) = e.stat_buff_cached.range(+0x438) + ef.range(+0x4a0) + (e.level(+0x5c8) - 1) * ef.growth_range(+0x4a8) · None(+0x4c0 == -1) 이면 0
           + e.radius()                                          // entity.rs:1511~1515: mult=+0x470; mult==0 ? +0x680 : +0x680*(mult+100)/100
           + e.stat_cached.move_speed(+0x640) * 120 };
388: let bene_engage   = engage(beneficiary);
389: let caster_engage = engage(caster);
390: let enemy_team = 1 - player.info.team;                         // (36815) · team>=2 면 panic_bounds_check(37514)
390: let enemy_near = data.cache.iter_champions(enemy_team).any(|c| {   // player_champion[enemy_team][0..5] 5칸 언롤 · null 칸 skip
391:     let rb = bene_engage   + c.radius();
392:     let rc = caster_engage + c.radius();
393:     (c.distance_sq(beneficiary) <= rb*rb || c.distance_sq(caster) <= rc*rc)   // IR: dist_b > rb² 일 때만 dist_c 검사(단락) · `icmp ugt … → skip`
394:     && data.blackboard[enemy_team].is_recent_visible(data.cache.game, player, c)   // 5회 call(36988·37114·37240·37366·37492)
     });
395~404(추정): if enemy_near { return None; }                        // IR: is_recent_visible true → %440 {2, undef}
405~416: let team_action_hits = |target_id: usize| -> bool {          // 클로저#2 · in_window 안에 인라인(aux m10.ll:6565~7112)
           let my = player.info.team;                                     // <2 아니면 panic_bounds_check(7111)
407:       data.cache.iter_champions(my).enumerate().any(|(pi, ally)| {   // player_champion[my][0..5] 5칸 언롤
408:           (ally.id == beneficiary.id || ally.distance_sq(beneficiary) <= 3600000000 /*60000²*/)
411:           && match data.blackboard[my].small_actions[pi] {           // Blackboard+0x78 + pi*24
412~415:           Some(Attack{target_id:t}|Skill{t}|Skill2{t}|Ult{t}|Trace{t}|Around{t}) => t == target_id,   // 태그 6·7·8·9·4·2 · 페이로드 +8
                   _ => false }                                          // RunAway·Positioning·AroundPosition·Dodge·Stop·None
           }) };
419: let atk = beneficiary.attack_effect.as_ref()?;                    // +0x4c0 == -1 → return None (%12 재사용 · 37511)
420: let move_allow = beneficiary.stat_cached.move_speed * 30;
426~433: let in_window = |target: &Entity| -> bool {                   // 클로저#3 = IR s1_0(aux m10.ll:6523~7225)
427:       let target_id = target.id;                                     // +0x5c0
428:       if !team_action_hits(target_id) { return false; }              // 아군 누구도(가까운 아군) 그 대상을 소액션으로 안 노리면 창 아님
429:       let range = atk.range(beneficiary) + atk.range_adjust(beneficiary, target) + beneficiary.radius() + target.radius() + move_allow;   // 7186~7191 덧셈 순서: (stat_range + range) + (level-1)*growth → + range_adjust → + bene.radius → + target.radius → + move_allow
431:       beneficiary.distance_sq(target) <= range*range                 // `icmp ule` (7223) · 경계 포함
         };
436: if data.cache.jungles.iter().any(|e| e.can_target() && in_window(e)) {   // +0xd0/+0xe8 · can_target = +0x6b9 && +0x6a0==0
         return Some(NoncombatSteroidWindow { fights_back: true, minion_wave: false });   // {1,0}
     }
440: if data.cache.iter_towers_without_nexus(enemy_team).any(|t| t.can_target() && in_window(t)) {   // aux m06.ll:34523 · Chain<Flatten<[Option<&Entity>;6]>, Copied<Iter>>
         return Some(NoncombatSteroidWindow { fights_back: false, minion_wave: false });  // {0,0}
     }
441: if let Some(n) = data.cache.nexus[enemy_team] { if n.can_target() && in_window(n) {   // +0x170 + team*8
         return Some(NoncombatSteroidWindow { fights_back: false, minion_wave: false }); } }   // {0,0} (37629~37630 → %434 [0,0])
447: if data.cache.iter_minions(enemy_team).any(|m| m.can_target() && in_window(m)) {   // aux m06.ll:8556 → m11.ll:30693
         return Some(NoncombatSteroidWindow { fights_back: false, minion_wave: true });   // {0,1}
     }
452: None                                                             // {2, 1} (byte1 쓰레기)

분기 순서(IR 블록): 적 5칸 각각 %109/%164/%219/%274/%329 null 검사 → 거리 2단 → is_recent_visible true 면 즉시 None(%440) / 5칸 모두 통과 → %381(enemy_near=false · team_action_hits 캡처 구성) → %12(attack_effect None) → %437 None / %386 move_allow·in_window 캡처 → 정글 루프(%399~%412) → %414 타워 → %416 넥서스 → %431 미니언 → %434 phi.
부작용 0(writes 없음 · alloca 5개는 클로저 캡처·sret 임시: %9 team_action_hits{cache,blackboard,player,beneficiary} 32B · %8 move_allow 8B · %7 in_window{&team_action_hits,atk,beneficiary,&move_allow} 32B · %6 towers 이터 120B · %5 minions 이터 56B). rnd 인자 없음 · gen_range 0. version 인자 없음(버전 분기 없음).
```

**`mem` 메모리 접근 23건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk |
|---|---|---|---|---|---|---|---|
| 0 | PlayerState | 0x930 | info.team | r | m10.ll:36813~36814 (gep 2352). enemy_team = 1 - team(390) · 클로저 안(m10.ll:6565~6566)에서는 내 팀 인덱스로 player_champion/blackboard 조회. <2 아니면 panic_bounds_check(37514 / 7111) | 4 | OK |
| 1 | OperationData | 0x0 | cache (&AbstractGameWithCache) | r | m10.ll:36811. cache+0 game 팻포인터(36825~36827: data %94 · vtable %96 → is_recent_visible 2번째 인자) · +0x1e0 player_champion[team] · +0xd0/+0xe8 jungles · +0x170 nexus[team] · iter_towers_without_nexus/iter_minions self | 4 | OK |
| 2 | OperationData | 0x10 | blackboard (&[Blackboard;2]) | r | m10.ll:36828~36829. 본문: blackboard[enemy_team](36872 gep Blackboard, %89) → is_recent_visible self. 클로저: blackboard[my_team].small_actions[pi](6573~6574 +120, stride 24) | 4 | OK |
| 3 | AbstractGameWithCache | 0x1e0 | player_champion[team][0..5] (Option<&Entity> ×5 · null=None) | r | m10.ll:36821~36822 (gep 480 + team*40). iter_champions(simulation.rs:1905) 인라인 → 5칸 언롤(36874·37000·37130·37256·37382). 클로저 안 6568~6569 도 동일(내 팀) | 4 | OK |
| 4 | AbstractGameWithCache | 0xd0 | jungles.buf.ptr (&Entity 원소 8B) | r | m10.ll:37533~37534 (gep 208). 436 정글 any 루프(stride shl 3 = 8B) | 4 | OK |
| 5 | AbstractGameWithCache | 0xe8 | jungles.len | r | m10.ll:37536~37537 (gep 232). 0 이면 루프 생략 | 4 | OK |
| 6 | AbstractGameWithCache | 0x170 | nexus[team] (Option<&Entity> · null=None) | r | m10.ll:37604~37606 (gep 368 + team*8). 441 적 넥서스 1개 검사 | 4 | OK |
| 7 | Entity | 0x4c0 | attack_effect@tag (Option<Effect> 니치 = Effect.casting CastingType · -1 = None) | r | beneficiary 36688~36691 · caster 36753~36756 (gep 1216, i32 == -1). 385 engage 클로저의 map/unwrap_or(0) 판별 · 419 `beneficiary.attack_effect?`(37511 %12 재사용) | 4 | OK |
| 8 | Entity | 0x490 | attack_effect@Some.0 (Effect 56B 선두) | r | m10.ll:37518 (gep 1168) → atk. in_window 클로저 캡처(%7+8). 클로저 안 atk+16 range(7123~7124) · atk+24 growth_range(7125~7126) · range_adjust self(7133) | 4 | OK |
| 9 | Entity | 0x4a0 | attack_effect@Some.0.range | r | m10.ll:36699~36700 / 36764~36765 (gep 1184). Effect::range(effect.rs:26) 인라인 = stat_buff_cached.range + range + (level-1)*growth_range | 4 | OK |
| 10 | Entity | 0x4a8 | attack_effect@Some.0.growth_range | r | m10.ll:36701~36702 / 36766~36767 (gep 1192) | 4 | OK |
| 11 | Entity | 0x5c8 | level | r | m10.ll:36703~36705 / 36768~36770 (gep 1480, -1). 클로저 7127~7129 | 4 | OK |
| 12 | Entity | 0x438 | stat_buff_cached.range | r | m10.ll:36707~36708 / 36772~36773 (gep 1080). 클로저 7131~7132 | 4 | OK |
| 13 | Entity | 0x470 | stat_buff_cached.radius_mult (i32) | r | Entity::radius() 인라인(entity.rs:1511~1515): beneficiary 36717~36718 · caster 36782~36783 · 적 챔프 c 36897~36898 외 4 · 클로저 안 beneficiary 7134~7135 · target 7161~7162 | 4 | OK |
| 14 | Entity | 0x680 | radius | r | Entity::radius() 인라인 (gep 1664). 36724·36731 / 36789·36796 / 적 5칸 / 클로저 7145·7152·7168·7175 | 4 | OK |
| 15 | Entity | 0x640 | stat_cached.move_speed | r | m10.ll:36741~36743 (bene ×120) · 36806~36808 (caster ×120) · 37521 (bene ×30 = move_allow) | 4 | OK |
| 16 | Entity | 0x660 | x | r | distance_sq(entity.rs:2158) 인라인. bene 36864/36868 · caster 36866/36870 · 적 c 36928~36929 외 4 · 클로저 6571(bene)·6603(ally)·7193(target)·7201(bene) | 4 | OK |
| 17 | Entity | 0x668 | y | r | distance_sq 인라인. bene 36865/36869 · caster 36867/36871 · 적 c 36934~36935 외 4 · 클로저 6572·6607·7197·7205 | 4 | OK |
| 18 | Entity | 0x5c0 | id | r | 클로저 안 m10.ll:6541~6542 target.id(427) · 6570(bene.id) · 6595~6598 ally.id == bene.id(408) · small_actions target_id 와 비교(415: 6646~6647 등) | 4 | OK |
| 19 | Entity | 0x6b9 | can_target (bool) | r | Entity::can_target()(entity.rs:1477~1478) 인라인 = can_target && block_target_tick==0. 정글 37570~37573 · 넥서스 37617~37620 · 타워 aux m06.ll:34625~34627 · 미니언 aux m11.ll:30732~30734 | 4 | OK |
| 20 | Entity | 0x6a0 | block_target_tick | r | ==0 조건. 정글 37576~37578 · 넥서스 37623~37625 · 타워 m06.ll:34628~34630 · 미니언 m11.ll:30735~30737 | 4 | OK |
| 21 | Blackboard | 0x78 | small_actions[pi]@tag (Option<SmallAction> · 24B stride · Direct 태그) | r | 클로저 안 m10.ll:6573~6574 (+120) · 6634~6642 switch. 9 Ult / 2 Around / 4 Trace / 6 Attack / 7 Skill / 8 Skill2 만 대상 id 비교, 그 외(0 RunAway·1 Positioning·3 AroundPosition·5 Dodge·10 Stop·None) 는 false. 5칸 언롤(6738 +144, … stride 24) | 4 | OK |
| 22 | Blackboard | 0x80 | small_actions[pi]@Some.0@{Ult\|Around\|Trace\|Attack\|Skill\|Skill2}.target_id | r | 클로저 안 m10.ll:6645~6647 (+128) 등 == target.id(415) | 4 | 부분일치(첫 성분(컨테이너)만 맞고 **마지막 성분이 다르다** — 사람이 봐야 ) |

**`consts` 상수 13건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev |
|---|---|---|---|---|---|
| 0 | -1 | 385 | 센티널 | Option<Effect> 니치 None 태그(Entity+0x4c0 = Effect.casting CastingType 4B == -1). 385(engage 의 map/unwrap_or) · 419(`attack_effect?` → None 반환). m10.ll:36690·36755. 또 `level - 1`(add -1, 36705·36770 · 클로저 7129) = Effect::range 의 (level-1)*growth_range | 4 |
| 1 | 100 | 386 | 계수 | Entity::radius() 인라인(entity.rs:1515): radius*(radius_mult+100)/100 백분율 스케일. 36733·36735 외 전 radius 호출부 | 4 |
| 2 | 120 | 386 | 계수 | engage(e) = 공격사거리 + radius + move_speed*120 — 교전권 판정에 120틱(=2초 @60tps 추정, tps 는 안 읽음) 이동 여유를 더함. 36743(bene)·36808(caster) | 5 |
| 3 | 1 | 390 | 태그 | enemy_team = 1 - player.info.team (36815 `sub i64 1, %88`). 또 Some 태그(DI self[0..+8]=1) · 반환 byte 값 1(fights_back=true 37643 / minion_wave=true 37643) | 4 |
| 4 | 2 | 390 | 센티널 | player_champion[team] 배열 길이 2 경계(36817 `icmp ult %89, 2` · 실패 시 panic_bounds_check 37514) · 반환 니치 None 태그 2(37639 select · 37651·37657 phi) | 4 |
| 5 | 30 | 420 | 계수 | move_allow = beneficiary.move_speed * 30 — in_window 사거리에 더하는 30틱(=0.5초 @60tps 추정) 이동 여유. m10.ll:37521 | 4 |
| 6 | 0 | 436 | 태그 | Entity::can_target() 인라인의 block_target_tick == 0 (37578·37625 · aux m06.ll:34630 · m11.ll:30737). 또 반환 byte 0(false) · unwrap_or(0) 기본값(DI default=0) | 4 |
| 7 | 3600000000 | 408 | 미상 | 60000² — team_action_hits: 아군 ally 가 beneficiary 본인이거나 beneficiary 로부터 ≤60000(=1.875셀) 안에 있어야 그 아군의 소액션 대상을 인정(`icmp ugt dist_sq, 3600000000` 이면 제외). aux m10.ll:6630·6734 외 5칸 | 4 |
| 8 | 9 | 413 | 태그 | SmallAction 메모리태그 9 = Ult(tcxdict --enum SmallAction · Direct 인코딩 · idx==태그). aux m10.ll:6636 switch case | 3 |
| 9 | 4 | 414 | 태그 | SmallAction 태그 4 = Trace. aux m10.ll:6638 | 4 |
| 10 | 6 | 412 | 태그 | SmallAction 태그 6 = Attack. aux m10.ll:6639 | 4 |
| 11 | 7 | 412 | 태그 | SmallAction 태그 7 = Skill. aux m10.ll:6640 | 4 |
| 12 | 8 | 413 | 태그 | SmallAction 태그 8 = Skill2. aux m10.ll:6641. (태그 2 = Around 는 위 `2` 항목과 값이 겹침 — aux m10.ll:6637 switch case 2 → 415 target_id 비교) | 4 |

**`knobs` 조정점 5건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 교전권(engage) 이동 여유 틱 | buff_value.rs:386 (m10.ll:36743·36808) | 120 | engage = 사거리 + 반지름 + 이속×120. 올리면 더 먼 적 챔프까지 '근접' 으로 잡혀 비교전 창이 더 자주 None(스테로이드 아낌) · 내리면 적이 꽤 가까워도 창을 열어 구조물/정글에 버프를 소모 | 4 | 기존 |
| 1 | in_window 이동 여유 틱(move_allow) | buff_value.rs:420 (m10.ll:37521) | 30 | 창 사거리 = 공격사거리 + range_adjust + 두 반지름 + 이속×30. 올리면 대상이 더 멀어도 창 인정(버프를 일찍 켬) · 내리면 대상이 거의 사거리 안이어야 창 | 4 | 기존 |
| 2 | team_action_hits 아군 근접 반경 | buff_value.rs:408 (aux m10.ll:6630) | 3600000000 | 60000²+0(경계 포함 ≤). 올리면 더 먼 아군의 소액션 대상도 '팀이 노리는 대상' 으로 인정돼 창이 넓어짐 · 내리면 본인/바로 옆 아군의 대상만. 값은 제곱이라 반경 R 로 바꾸려면 R² | 4 | 기존 |
| 3 | 창을 여는 소액션 종류 집합 | buff_value.rs:411~415 (aux m10.ll:6635~6642 switch) | 태그 {9 Ult, 2 Around, 4 Trace, 6 Attack, 7 Skill, 8 Skill2} | 집합에서 빼면 그 소액션으로 노리는 대상은 창을 못 연다(예: Around 제거 → 배회 중인 대상은 제외). Positioning/AroundPosition(좌표형)·RunAway·Dodge·Stop 은 대상 id 가 없어 원리상 제외 | 4 | 기존 |
| 4 | 창 우선순위(정글 → 타워 → 넥서스 → 미니언) | buff_value.rs:436→440→441→447 | 순서 고정 | 정글이 창이면 fights_back=true 가 확정되고 미니언 검사는 안 한다. 순서를 바꾸면 같은 상황에서 반환 플래그가 달라져 noncombat_steroid_value 의 가치 계산이 바뀜 | 4 | 기존 |

<details><summary>`callees` 피호출자 15건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | can_target | game_core::Entity::can_target | pub | fn(&game_core::Entity) -> bool | game-core\src\simulation\entity.rs:1477 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
| 1 | can_target | game_core::OperationData::<'a, 'b>::can_target | pub | fn(&game_core::OperationData<'a/#0, 'b/#1>, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\state\player.rs:23 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
| 2 | distance_sq | game_core::utils::distance_sq | pub | fn(u64, u64, u64, u64) -> u64 | game-core\src\utils.rs:6 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
| 3 | distance_sq | game_core::Entity::distance_sq | pub | fn(&game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\entity.rs:2157 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 2개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 2개 |
| 4 | is_recent_visible | game_core::Blackboard::is_recent_visible | pub | fn(&game_core::Blackboard, &dyn [Binder { value: Trait(game_core::AbstractGame), bound_vars: [] }] + , &game_core::PlayerState, &game_core::Entity) -> bool | game-core\src\simulation\game\blackboard.rs:346 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 5 | iter_champions | game_core::AbstractGameWithCache::<'a, 'b>::iter_champions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5759 ~ game_core[6a30]::simulation::{impl#5}::iter_champions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1904 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 6 | iter_minions | game_core::AbstractGameWithCache::<'a, 'b>::iter_minions | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5753 ~ game_core[6a30]::simulation::{impl#5}::iter_minions::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1847 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 7 | iter_towers_without_nexus | game_core::AbstractGameWithCache::<'a, 'b>::iter_towers_without_nexus | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, usize) -> Alias(AliasTy { args: ['a/#0, 'b/#1, 'a/#0, ], kind: Opaque { def_id: DefId(14:5750 ~ game_core[6a30]::simulation::{impl#5}::iter_towers_without_nexus::{opaque#0}) }, .. }) | game-core\src\simulation.rs:1830 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 8 | level | <game_ai::WindowStatView<'_> as game_core::AbstractEntity>::level | pub | fn(&game_ai::WindowStatView</#0>) -> usize | game-ai\src\fight_check.rs:96 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 9 | level | game_core::AbstractEntity::level | pub | fn(&Self/#0) -> usize | game-core\src\simulation\entity.rs:718 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 10 | level | <game_core::Entity as game_core::AbstractEntity>::level | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:772 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 3개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). 상위 3개 |
| 11 | noncombat_steroid_window | game_ai::buff_value::noncombat_steroid_window | in:game_ai | fn(&game_core::PlayerState, &game_core::OperationData, &game_core::Entity, &game_core::Entity) -> std::option::Option<game_ai::NoncombatSteroidWindow> | game-ai\src\buff_value.rs:378 | False | False | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 12 | radius | game_core::Entity::radius | pub | fn(&game_core::Entity) -> usize | game-core\src\simulation\entity.rs:1509 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 13 | range | game_core::Effect::range | pub | fn(&game_core::Effect, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:25 | True | True | 4 | ⚠미확정 — tcx 3크레이트에 같은 leaf 가 1개. IR 범위의 직접 호출 심볼과는 안 맞는다(`logic` 산문에서 긁혔거나 인라인·접힘). ★후보가 1개라는 것은 **정답이라는 뜻이 아니다** — tcx 에 동명이 하나뿐일 뿐이다. 상위 1개 |
| 14 | range_adjust | game_core::Effect::range_adjust | pub | fn(&game_core::Effect, &game_core::Entity, &game_core::Entity) -> u64 | game-core\src\simulation\effect.rs:29 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
</details>

⚠**미매칭 7개**: `engage`, `enumerate`, `in_window`, `move_speed`, `s1_0`, `team_action_hits`, `try_fold`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 2곳** (m05.ll:43307, m05.ll:43888) · **형제 0개** 

**`open` 8건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 표기 불가 | 395~404 사이 소스 줄의 정확한 형태(`if enemy_near { return None; }` 추정) — IR 은 is_recent_visible true → 즉시 {2,undef} 반환으로 접혀 있고 그 구간 DILocation 이 없다(빈 줄/주석 가능). 동작은 확정, 표기만 추정 | 4 |  |
| 1 | 표기 불가 | 412~415 의 variant 나열 순서(어느 줄에 어느 variant) — 태그별 !dbg 줄(6·7→412 · 8·9→413 · 4→414 · 2→415)만 있고 column 이 없어 줄 안 순서는 표기 불가(동작 확정) | 4 |  |
| 2 | 미탐색 | Blackboard 인덱스 의미: is_recent_visible 은 blackboard[enemy_team](관측 대상 팀), team_action_hits 는 blackboard[my_team].small_actions — 'blackboard[t] = t 팀 챔피언들의 기록' 으로 읽으면 정합. is_recent_visible 내부(g07.ll:157005 · 계약만)는 안 읽음 | 4 |  |
| 3 | 미탐색 | iter_towers_without_nexus(simulation.rs:1830)·iter_minions(1847) 반환 이터레이터 내부 레이아웃(120B/56B sret) — 계약만. try_fold 조각(aux) 은 원소를 &Entity 로 꺼내 술어만 적용함을 확인 | 4 |  |
| 4 | 미탐색 | Effect::range_adjust(&Effect, &Entity caster, &Entity target) -> u64 (effect.rs:29) 내부 — 계약만(aux m10.ll:7133 호출) | 4 |  |
| 5 | 미탐색 | 120·30 틱의 초 환산은 tps=60 가정(본문에 tps 읽기 없음 · 리터럴 확정) | 4 |  |
| 6 | 미탐색 | 개발자 주석: 구조체 NoncombatSteroidWindow(364)·fights_back(366)·minion_wave(369) 에는 _docs game_ai.txt:37~40 주석이 있으나 이 함수(378) 자체 주석은 0건 | 4 |  |
| 7 | 미탐색 | exe 0xd75cb0(1166B · 콜리 0) = s1_0 클로저 추정(642 ins) — 이 클로저는 range_adjust 를 call 하므로 '콜리 0' 과는 안 맞을 수 있음(range_adjust 가 exe 에서 인라인됐거나 다른 함수일 가능성) · 미검증 | 5 |  |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

