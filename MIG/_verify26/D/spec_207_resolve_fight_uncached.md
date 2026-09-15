---

### `207` resolve_fight_uncached — 교전 예측 순수 코어: 아군·적 각 ≤5명의 EHP·DPS·CC·도착틱을 만들어(오판 노이즈 포함) 6초 창을 틱 단위로 전개하고 승패 라인(Commit/CommitAfterJoin/Disengage/Hold)·net_value·focus/soaker/rescue 를 FightPrediction(64B) 으로 낸다

| 항목 | 값 |
|---|---|
| id | `fight_model__resolve_fight_uncached` |
| 심볼 | `_RNvNtNtNtCshdEBA0ozCnw_7game_ai11plan_legacy3old11fight_model22resolve_fight_uncached` |
| 소스 | `game-ai\src\plan_legacy\old\fight_model.rs:378` |
| IR | `m10.ll` 43974~47451행 |
| 경로·가시성 | `game_ai::plan_legacy::old::fight_model::resolve_fight_uncached` · **in:game_ai::plan_legacy::old::fight_model** |
| 계층 | 레거시 플랜 |
| exe | `e083c0` (None) · None바이트 · None명령 |
| 라운드 | 기준 `r17` · 통과 1회 |

**시그니처(tcx 정본, ev3)**
```rust
fn(usize, &game_core::OperationData, &game_core::Entity, &[&game_core::Entity], &[&game_core::Entity], i8, std::option::Option<&game_core::Entity>, usize, &[i64], i64) -> game_ai::plan_legacy::old::FightPrediction
```

**TLS 메모 절(`sig.tls` · 정정 경로 `/specs[207]/sig/tls/<키>`)**

없음 — 함수 전체(m10.ll:43974~47451) 에 `LocalKey`/`call_once`/`llvm.threadlocal.address`/`@anon…call_once` 상수 참조 0건. rnd(StdRng) 인자도 없고 유일한 난수원은 시드에서 만든 NoiseRng jrng(8B 지역) 이다

<details><summary>인자 11개</summary>

| # | i | 이름 | 타입 | 역할 | ev |
|---|---|---|---|---|---|
| 0 | 0 | (sret) out | *mut FightPrediction(64B) | 레이아웃 = signature.return \| (배치 L) (배치 L) 내 범위의 유일한 외부 쓰기 표면 — 520 줄에서 8필드 기록(writes 참조) | 4 |
| 1 | 1 | version | usize | reach version=2 접기에서 %58(=version>1) → 1 로 접혀 L402~403 이 사장 \| (배치 L) (배치 L) 465~521 에서 미사용 | 4 |
| 2 | 2 | data | &OperationData(24B) | DILocalVariable `data` arg 2 · `context` 는 L379 지역(let context = data.context) \| (배치 L) (배치 L) 465~521 에서 %2·%3 미사용 | 4 |
| 3 | 3 | champ | &Entity(1728B) | (배치 L) (배치 L) 미사용 | 4 |
| 4 | 4 | near_allies | &[&Entity] | len==0 이면 L381 조기 반환(Hold) \| (배치 L) (배치 L) 496·501 소커 재선정 키 `near_allies[i].stat_cached.hp`(+0x628) 로만 읽음. 인덱스 i<our_n 이 len 과 bounds-check 됨(m10.ll:46753·47027) | 4 |
| 5 | 5 | near_enemies | &[&Entity] | len==0 이면 L381 조기 반환(Hold) \| (배치 L) (배치 L) 미사용(their_* 로컬 배열만 씀) | 4 |
| 6 | 6 | committed_dir | i8 | (배치 L) 소비처 서술 필요 \| (배치 L) (배치 L) 515 switch: 1 / -1 / 그 외(0) 세 갈래 히스테리시스 | 4 |
| 7 | 7 | tower | Option<&Entity> | (배치 L) (배치 L) 미사용 — 타워 효과는 K 가 만든 tower_dps(%332)·soaker(%333/%334/%335) 로만 들어옴 | 4 |
| 8 | 8 | judge_accuracy | usize | 999 비교는 dbg 없는 호이스트 명령(%93) — 소스는 L406 클로저 \| (배치 L) (배치 L) 미사용 | 4 |
| 9 | 9 | arrivals | &[i64] | arrivals[ai] 값은 L463 arrived 클로저(our_arrive[i] <= t)에서 소비 \| (배치 L) (배치 L) 미사용(our_arrive 로컬 배열로 이미 복사됨) | 4 |
| 10 | 10 | baseline | i64 | (배치 L) 소비처 서술 필요 \| (배치 L) (배치 L) 508 `net = their_dead_v - (our_dead_v + baseline)` | 4 |
</details>

**의사코드 `logic`**
> ★재구현용 의사코드다. **오프셋·상수·시그니처의 정본은 `mem`/`consts`/`callees`/`sig.tcx` 이고, 여기와 어긋나면 그쪽이 맞다.** (v2 에서 정정이 표에만 반영되고 이 블록이 옛 값으로 남는 사고가 반복됐다)
```
// fight_model.rs:0~464 (배치 K)
// 인자: version, data(&OperationData → %2=data.cache · %3=data.context), champ, near_allies(&[&Entity]), near_enemies(&[&Entity]), committed_dir(i8), tower(Option<&Entity>), judge_accuracy, arrivals(&[i64]), baseline(i64) → sret FightPrediction(64B)
// 콜리 계약(자식 명세 별도): self_sustain_in_window(version,&GameContext,&Entity,horizon)->i64(창 내 자기 힐+실드 1회분) · available_cc_in_window(version,&Entity,horizon)->i64(창 내 CC 틱 합) · fight_dps(version,&GameContext,att,tgt)->i64 · expected_dps(&GameContext,tower,tgt)->i64 · error_ratio_noise(&mut NoiseRng,j)->i64 (m04.ll:49631 본문 확인: e=(1000-j)/20(udiv) · state+=0x9E3779B97F4A7C15 · splitmix64 믹스 · mulhi(mix, 2e+1) - e + 100 ⇒ [100-e, 100+e] 균등 정수 백분율)

L379: let context = data.context;                                             // %3 (승격 슬롯)
L380: if near_allies.is_empty() || near_enemies.is_empty() {                // %40 = len_a==0, %41 = len_e==0, or
L381:   return FightPrediction { focus_target: None, soaker: None, rescue_ally: None, net_value: 0, line: Hold(3), line_absolute: Hold(3) };  // 0x0/0x10/0x20 ← 0, 0x30 ← 0, 0x38/0x39 ← 3 · 페이로드/패딩 미기록 → 배치 L(줄 521) ret
}

// ── 시드 (L392~403) ──
L392: let seed: u64 = if version > 1 {                                       // %58 = icmp ugt %1, 1 (reach: 항상 true 로 접힘)
L394:   let mut set_h: u64 = 0;
L394:   for a in near_allies.iter().take(5) {                                // %97 루프(카운트 5↓ · end 포인터)
L395:     set_h ^= a.id.wrapping_mul(0x9E3779B97F4A7C15);                    // Entity+0x5c0
        }
L397:   for e in near_enemies.iter().take(5) {                               // %128 루프
L398:     set_h ^= e.id.wrapping_mul(0x517CC1B727220A95);
        }
L400:   game.seed() ^ set_h.rotate_left(17) ^ champ.id                        // vtable+0x20 seed() · llvm.fshl 17 · champ+0x5c0 (xor 순서: (seed ^ rotl) ^ id)
      } else {                                                               // NA(version<2 전용) — reach version=2 접기로 사장
L402:   let bucket = game.tick() / max(tps * 2, 1);                          // vtable+0x28 tick() · GameSetting+0x12f8 · `shl 1` · umax
L403:   champ.id ^ (bucket << 40)
      };
L392: let mut jrng = NoiseRng(seed);                                          // %38 ← seed 그대로(믹스 없음 · ai_interface.rs:177 인라인)

// ── 오판·DPS 클로저 (L405~412) ──
L405: let misjudge = |v: i64| -> i64 {
L406:   if judge_accuracy > 999 { v } else { v * error_ratio_noise(&mut jrng, judge_accuracy) / 100 }   // %93 호이스트 · sdiv 100 · 호출마다 jrng 전진
      };
L411: let dps_of = |att: &Entity, tgt: &Entity| -> i64 {
L412:   fight_dps(version, context, att, tgt)
      };

// ── 창·아군 표 (L415~425) ──
L415: let horizon_ticks: i64 = tps * 6;                                       // GameSetting+0x12f8 (version≤1 경로는 L402 에서 읽은 값 재사용)
L416: let mut our_hp = [0i64; 5]; let mut our_dps = [0i64; 5]; let mut our_alive = [false; 5]; let mut our_n = 0usize; let mut our_cc_sum = 0i64;
L418: let mut our_arrive = [0i64; 5];
L419: for (ai, a) in near_allies.iter().copied().take(5).enumerate() {         // %33 = {ptr,end,take=5,idx} · Copied::next 아웃오브라인
L420:   our_arrive[ai] = arrivals.get(ai).copied().unwrap_or(0);              // %152 = ai < arrivals.len · 5칸 경계검사(%909, take(5) 로 실질 불발)
L421:   let tgt = near_enemies.iter().copied().min_by_key(|e| { let dx=|a.x-e.x|; let dy=|a.y-e.y|; dx*dx+dy*dy });   // aux s0_0 · 첫 원소 next 뒤 fold · 동률=먼저 것
L422:   let dps = tgt.map(|t| dps_of(a, t)).unwrap_or(0);                     // near_enemies 비면 0(L380 으로 실질 불발) · fight_dps(version, context, a, t)
L423:   let ehp = a.hp + self_sustain_in_window(version, context, a, horizon_ticks);   // Entity+0x670
L424:   our_cc_sum += available_cc_in_window(version, a, horizon_ticks);
L425:   our_hp[our_n] = misjudge(ehp * 1000); our_dps[our_n] = misjudge(dps); our_alive[our_n] = true; our_n += 1;   // 노이즈 호출 순서: hp → dps · 5칸 경계검사(%953/%955)
      }                                                                       // our_n == min(len_a, 5) · ai == our_n 항상

// ── 적 표 (L427~435) ──
L427: let mut their_hp = [0i64; 5]; let mut their_dps = [0i64; 5]; let mut their_alive = [false; 5]; let mut their_n = 0usize;
L428: let mut their_id = [0usize; 5]; let mut their_cc_sum = 0i64;
L430: for e in near_enemies.iter().take(5) {                                  // %158 루프(end 포인터 · 카운트 5↓)
L431:   let nearest_a = near_allies.iter().copied().min_by_key(|a| dist2(a, e)).unwrap_or(champ);   // aux s2_0 · (near_allies 비면 champ — L380 으로 실질 불발)
L432:   let ehp = e.hp + self_sustain_in_window(version, context, e, horizon_ticks);
L433:   their_cc_sum += available_cc_in_window(version, e, horizon_ticks);
L434:   their_hp[their_n] = misjudge(ehp * 1000); their_dps[their_n] = misjudge(dps_of(e, nearest_a));   // 노이즈 순서: hp → dps · fight_dps(version, context, e, nearest_a)
L435:   their_alive[their_n] = true; their_id[their_n] = e.id; their_n += 1;
      }                                                                       // their_n == min(len_e, 5) ≥ 1
// ⇒ jrng 소비 순서(judge_accuracy ≤ 999 일 때만): 아군 i=0..our_n (hp, dps) → 적 j=0..their_n (hp, dps) · 최대 20회

// ── CC 로 유효시간 보정 (L437~443) ──
L437: if horizon_ticks != 0 {                                                 // %168 = icmp eq %85, 0 → 건너뜀
L440:   let their_eff = max(h - min(h / 2, our_cc_sum), 1);                   // h = horizon_ticks · smin/smax(i64) · ashr 1
L441:   let our_eff   = max(h - min(h / 2, their_cc_sum), 1);
L442:   for j in 0..their_n { their_dps[j] = their_dps[j] * their_eff / h; }    // sdiv · 5칸 언롤(%303,%879~%897)
L443:   for i in 0..our_n   { our_dps[i]   = our_dps[i]   * our_eff   / h; }    // 5칸 언롤(%317,%854~%872) · our_n==0 이면 통째 생략(%316)
      }

// ── 타워·소커 (L449~455) ──
L449: let tower_dps: i64 = tower.map(|t| {
L450:     (0..our_n).max_by_key(|&i| near_allies[i].stat_cached.hp)            // aux s3_00 · Entity+0x628 · 동률=뒤의 것 · our_n==0 → None
L451:       .map(|i| expected_dps(context, t, near_allies[i]))                 // near_allies[i] 경계검사(%246)
          }).flatten().unwrap_or(0);                                          // tower None(null) 또는 our_n==0 → 0
L454: let soaker: Option<usize> = (0..our_n).filter(|&i| our_alive[i]).max_by_key(|&i| near_allies[i].stat_cached.hp);   // aux s4_0/s5_0 · 첫 alive 원소는 인라인 언롤(%263~%295), 나머지는 fold 호출
L455: let soaker_id: Option<usize> = soaker.map(|i| near_allies[i].id);         // Entity+0x5c0 · 경계검사(%329)

// ── 틱 전개 준비 (L459~464) ──
L459: let mut t: i64 = 0; let mut our_dead_v: i64 = 0; let mut their_dead_v: i64 = 0; let mut first_focus: Option<usize> = None;   // t=%28(스택 · 클로저가 &t 캡처)
L461: let horizon = horizon_ticks;
L461: loop {                                                                  // 헤더 %765 — 루프 캐리: our_dead_v(%766) · their_dead_v(%767) · 첫 반복 플래그(%768) · first_focus(%769/%770) · soaker(%771/%772). 종료 조건·역방향 엣지(%761 ← L499/L500/L501)는 → 배치 L(줄 465~521)
L463:   let arrived = |i: usize| -> bool { our_arrive[i] <= t };              // %27 = {&our_arrive, &t}
L464:   let our_dps_sum: i64 = (0..our_n).map(|i| if our_alive[i] && arrived(i) { our_dps[i] } else { 0 }).sum();   // 5칸 언롤(%398~%431) · our_dps[i] 는 루프 진입 전 호이스트 로드(%378~%382 — 루프 안에서 our_dps 불변) · our_n==0 이면 0(%331)
        // → 배치 L(줄 465): their_dps_sum 등 이어짐 (%439)
      }
// L465~521 = 배치 L (틱 전개 · 사망 처리 · focus/rescue · line 판정 · sret 기록)

// fight_model.rs:465~521 (배치 L)
// ── 전제(배치 K 산출 · 이름은 DI 변수명) ──
// 로컬 배열(스택, 길이 5): our_alive[%35: 5×bool] our_arrive[%34: 5×i64] our_dps[%36] our_hp[%37] / their_alive[%30] their_dps[%31] their_hp[%32] their_id[%29: 엔티티 id]
// our_n=%154 · their_n=%167 · horizon=%85(horizon_ticks) · t=*%28(현재 시뮬 틱) · tower_dps=%332 · 루프 소커 soaker=(tag %771, idx %772) · 초기 소커 출력용 soaker_id=(%333,%335)
// arrived(i) := !(our_arrive[i] > t)  — 463 줄 클로저(캡처 %27 = {&our_arrive, &t}), 468·480 에서 인라인
// %331 = K 플래그: `(0..our_n)` 폴드를 통째로 건너뛰는 경로(our_n == 0 로 추정 — unknown 참조). 루프 머리 phi(m10.ll:46791~46798): our_dead_v=%766 their_dead_v=%767 first_focus=(%769 tag,%770) soaker=(%771,%772) · 첫 진입 시 0/0/None/K값
// our_total(%440, 464 줄 K) = Σ our_dps[i] (i<our_n, our_alive[i] && arrived(i))

// 465: their_total = (0..their_n).filter(|i| their_alive[i]).map(|i| their_dps[i]).sum()
//   (m10.ll:45397~45451: %355 = their_n ∉ 1..=5 이면 panic_bounds_check(5,5) — 컴파일러가 their_n≥1 을 알고 do-while 로 편 것 · 실전 도달 불가)
//   %454 = their_total

// 466: te = (0..their_n).filter(|i| their_alive[i]).min_by_key(|i| their_hp[i])   ← closure#14/#15, aux m12.ll:17437 (동률이면 앞 인덱스)
//   첫 생존 원소를 본문에서 언롤(m10.ll:45500~45624: their_alive[0..4] 순차 검사, %480=첫 idx, %482=their_hp[첫])
//   → 생존 적 없음(their_n 개 전부 false) → @534 (508 줄로 break: 전멸 = 승리 정산)
//   → 있음 → %483 = fold(env %21={&their_alive, their_n, &their_hp, cur}, first_key, first_idx) · te = %483.1 (m10.ll:45663 / 46011)

// 468: ta = (0..our_n).filter(|i| our_alive[i] && arrived(i)).min_by_key(|i| our_hp[i])   ← closure#16/#17, aux m12.ll:17583
//   %331 이면 폴드 생략 → @600: net = 0 - baseline, our_unit = 0 으로 곧장 515 로(m10.ll:46264·47063)
//   언롤(m10.ll:45723~45908): i=0..4 에 대해 our_alive[i] 확인 → our_arrive[i] > t 이면 미도착으로 skip(%493 등 `icmp sgt arrive, t`)
//   → 도착한 생존 아군 없음 → 471 로
//   → 있음 → %557 = fold(env %20={&our_alive, &arrived캡처, our_n, &our_hp}, …) · ta = %557.1 (m10.ll:46027~46028, DI 이름 `ta`)

// 471 (ta 없음 분기): next = (0..our_n).filter(|i| our_alive[i] && our_arrive[i] > t).map(|i| our_arrive[i]).min()   ← closure#18/#19, aux m12.ll:17750
//   언롤(m10.ll:46086~46211: our_alive[i] && our_arrive[i] > t 인 첫 i 의 arrive 가 first, 이후 fold(env %26) 가 min)
//   → 후보 없음 → @597: 508 줄로 break(net 정산)
//   → Some(next): if next < horizon (m10.ll:46296~46297)
//        472: t = next; continue  (m10.ll:46300 store %28 · 46302 br %397 — dead/first_focus/soaker 는 그대로 유지)
//      else → 508 로 break

// 476: if t >= horizon || (our_total | their_total) == 0   (m10.ll:46031~46036 · `or i64` = our_total==0 && their_total==0 접힘 · IR 은 t>=horizon 을 먼저 평가)
//        → break → 508

// 477: if first_focus.is_none() { first_focus = Some(their_id[te]) }   (m10.ll:46305 %768 · 46378~46379 their_id[te] · bounds te<5)

// 480: soak: Option<usize> = soaker.filter(|&s| our_alive[s] && arrived(s))   (m10.ll:46318~46359: %613=soaker.is_some · our_alive[s] · our_arrive[s] > t 면 None) → (%630 is_some, %631 s)

// 481: ta_incoming = their_total + (if soak == Some(ta) { tower_dps } else { 0 })   (m10.ll:46365~46368 · %633 = soak.is_some && s==ta)

// 484: t_te = if our_total > 0 { (their_hp[te] + our_total - 1) / our_total } else { INF }   (INF = 2305843009213693951 · m10.ll:46370·46403~46407 · ceil 나눗셈, sdiv)
// 485: t_ta = if ta_incoming > 0 { (our_hp[ta] + ta_incoming - 1) / ta_incoming } else { INF }   (m10.ll:46395·46426~46430)
// 486: t_soak = if let Some(s) = soak && s != ta && tower_dps > 0 { (our_hp[s] + tower_dps - 1) / tower_dps } else { INF }   (m10.ll:46420~46442 · %368 = tower_dps>0, %369 = tower_dps-1 은 루프 밖에서 선계산 · `s != ta` 와 `tower_dps > 0` 의 소스 순서는 표기 불가)
// 487: dt = max(1, min(horizon - t, min(t_soak, min(t_ta, t_te))))   (m10.ll:46451~46461 · smin 사슬 순서 = IR 기준, 소스 괄호 순서는 표기 불가)

// 489: their_hp[te] -= dt * our_total          (m10.ll:46467~46471)
// 490: our_hp[ta]   -= dt * ta_incoming        (m10.ll:46480~46484)
// 491: if let Some(s) = soak && s != ta { our_hp[s] -= dt * tower_dps }   (m10.ll:46485~46488 %695 = !(is_some && s!=ta) → skip · 46501~46505 · ★tower_dps>0 검사 없음 — 0 이면 0 차감)
// 492: t += dt                                  (m10.ll:46495~46496 store %28)

// 493: if their_hp[te] < 1 { their_alive[te] = false; their_dead_v += their_dps[te] }   (m10.ll:46497~46514 · `<1` == `<=0` 표기 불가 · dead_v 는 죽은 유닛의 **dps** 합)
// 494: if our_hp[ta] < 1 {                                                          (m10.ll:46520~46522)
// 495:   our_alive[ta] = false; our_dead_v += our_dps[ta]                             (m10.ll:46535~46539)
// 496:   if soaker == Some(ta) { soaker = (0..our_n).filter(|i| our_alive[i]).max_by_key(|i| near_allies[i].stat_cached.hp) }   ← closure#21/#22 aux m12.ll:17885 (m10.ll:46543~46545 루프 소커(%613/%772) 와 비교 · 언롤 46595~46718 · 첫 원소 key = near_allies[i]+0x628 · 후보 없으면 soaker = None(%760=0) · 동률이면 **뒤** 인덱스)
//      }
// 498: if let Some(s) = soak && s != ta {                                              (m10.ll:46532 %695 재사용)
// 499:   if our_hp[s] < 1 {                                                            (m10.ll:46802~46805)
// 500:     our_alive[s] = false; our_dead_v += our_dps[s]                             (m10.ll:46808~46812)
// 501:     if soaker == Some(s) { soaker = 위 496 과 동일 재선정 }   ← closure#23/#24 aux m12.ll:18047 (m10.ll:46816~46819 · 496 에서 갱신된 soaker(%716/%717) 기준 · 언롤 46821~47045)
//      } }
// → 루프 머리(@761→@765, m10.ll:46781~46799)로 back-edge · 464 로 (배치 K 줄)

// ── 루프 탈출 후 ──
// 508: net = their_dead_v - (our_dead_v + baseline)   (m10.ll:45912~45913 / 45938~45939 / 46218~46219 · %331 경로는 0 - baseline 으로 접힘 46264)
// 510: our_unit = if our_n > 0 { (Σ_{i<our_n} our_dps[i]) / our_n } else { 0 }   ← closure#25 (m10.ll:45964 our_n>5 → panic · 45972~46001 합(생존/도착 무관 전원) · 47056 sdiv · %331 경로 0 = 47063 phi · 0 처리의 소스 형태(명시 if / checked_div / max(1)) 는 표기 불가)
// 515: match committed_dir {                       (m10.ll:47065 switch i8 %9)
// 516:   1  => line = if net < -our_unit { Disengage } else if net > -1 /*net>=0*/ { Commit } else { Hold }   (47075~47085)
// 517:   -1 => line = if net > our_unit { Commit } else if net < 1 /*net<=0*/ { Disengage } else { Hold }        (47080~47081 · 47119~47120)
// 518:   _  => line = if net > our_unit { Commit } else if net < -our_unit { Disengage } else { Hold }             (47071~47072 · 47124~47126)
//      }
// 520: *sret = FightPrediction { focus_target: first_focus, soaker: soaker_id(K 초기값 %333/%335 — 루프 중 교체분 아님), rescue_ally: None, net_value: net, line, line_absolute: line }   (m10.ll:47091~47105)
// 521: ret   (m10.ll:44135 @59 · 381 조기반환도 같은 ret 블록으로 합류)
```

**`mem` 메모리 접근 30건** — ★`ev` 상한은 **3**이다(오프셋 주장의 최강 근거가 tcx 라서).
| # | 베이스 | 오프셋 | 이름 | 방향 | 근거 | ev | chk | value |
|---|---|---|---|---|---|---|---|---|
| 0 | AbstractGameWithCache(data.cache, %2) | 0x0 | game.data_ptr | r | L400·L402 — &dyn AbstractGame 팻포인터의 데이터 포인터 | 4 | OK |  |
| 1 | AbstractGameWithCache(data.cache, %2) | 0x8 | game.vtable_ptr | r | L400·L402 | 4 | OK |  |
| 2 | AbstractGame vtable | 0x20 | seed() | r | L400 (version>1) — divtable: ExpectedGame::AbstractGame::seed. 반환 i64 게임 시드 | 3 | 확인불가(vtable 슬롯) |  |
| 3 | AbstractGame vtable | 0x28 | tick() | r | L402 (version≤1 · NA) — divtable: ExpectedGame::AbstractGame::tick | 3 | 확인불가(vtable 슬롯) |  |
| 4 | GameContext(%3) | 0x8 | setting | r | L402·L415 — &GameSetting | 4 | OK |  |
| 5 | GameSetting | 0x12f8 | tick_per_second | r | L402(bucket 분모 · NA)·L415(horizon_ticks = tps*6) | 4 | OK |  |
| 6 | Entity(champ %4) | 0x5c0 | id | r | L400·L403 시드 xor 재료 | 4 | OK |  |
| 7 | Entity(near_allies[i]) | 0x5c0 | id | r | L395 집합 해시 · L455 soaker_id | 4 | OK |  |
| 8 | Entity(near_enemies[j]) | 0x5c0 | id | r | L398 집합 해시 · L435 their_id[j] | 4 | OK |  |
| 9 | Entity(near_allies[i]) | 0x628 | stat_cached.hp | r | L450 타워 대상 max_by_key 키 · L454 soaker max_by_key 키 (aux m12 클로저 안 gep 1576) | 4 | OK |  |
| 10 | Entity(near_allies / near_enemies) | 0x660 | x | r | L421·L431 최근접 거리 dx | 4 | OK |  |
| 11 | Entity(near_allies / near_enemies) | 0x668 | y | r | L421·L431 최근접 거리 dy | 4 | OK |  |
| 12 | Entity(near_allies[i]) | 0x670 | hp | r | L423 ehp = hp + self_sustain_in_window | 4 | OK |  |
| 13 | Entity(near_enemies[j]) | 0x670 | hp | r | L432 ehp = hp + self_sustain_in_window | 4 | OK |  |
| 14 | arrivals(&[i64] %12) | 0x0 + ai*8 | arrivals[ai] | r | L420 — ai < len 일 때만(get) · 아니면 0 | 4 | 확인불가(tcx 사전에 타입 없음) |  |
| 15 | Entity | 0x628 | stat_cached.hp | r | near_allies[i] 경유 · 496/501 소커 재선정 max_by_key 키(m10.ll:46765·47039 첫 원소, aux fold 안 `i64 1576`) | 4 | OK |  |
| 16 | FightPrediction(sret %0) | 0x0 | focus_target@tag | w | L381 조기 반환(m10.ll:44120) — near_allies 또는 near_enemies 가 빈 경우 | 4 | OK | 0 (None) |
| 17 | FightPrediction(sret %0) | 0x10 | soaker@tag | w | L381 (m10.ll:44122) | 4 | OK | 0 (None) |
| 18 | FightPrediction(sret %0) | 0x20 | rescue_ally@tag | w | L381 (m10.ll:44124) | 4 | OK | 0 (None) |
| 19 | FightPrediction(sret %0) | 0x30 | net_value | w | L381 (m10.ll:44119) | 4 | OK | 0 |
| 20 | FightPrediction(sret %0) | 0x38 | line | w | L381 (m10.ll:44117) | 4 | OK | 3 (FightLine::Hold) |
| 21 | FightPrediction(sret %0) | 0x39 | line_absolute | w | L381 (m10.ll:44126). 0x08/0x18/0x28 페이로드·0x3a~0x3f 패딩은 이 경로에서 미기록 | 4 | OK | 3 (FightLine::Hold) |
| 22 | FightPrediction | 0x0 | focus_target@tag | w | m10.ll:47095 · 정상 종료 경로 520 | 4 | OK | %826 = first_focus tag(0 None / 1 Some) |
| 23 | FightPrediction | 0x8 | focus_target@Some.0 | w | m10.ll:47097 | 4 | OK | %825 = their_id[te](첫 루프 반복의 their_focus 엔티티 id) · tag 0 이면 undef |
| 24 | FightPrediction | 0x10 | soaker@tag | w | m10.ll:47099 | 4 | OK | %333(K · 455 줄 초기 소커 tag) — 루프 안에서 교체된 소커가 아니라 **초기값** |
| 25 | FightPrediction | 0x18 | soaker@Some.0 | w | m10.ll:47101 | 4 | OK | %335(K · near_allies[idx].id) · tag 0 이면 undef |
| 26 | FightPrediction | 0x20 | rescue_ally@tag | w | m10.ll:47103 | 4 | OK | 0 (None 고정 · 0x28 페이로드 미기록) |
| 27 | FightPrediction | 0x30 | net_value | w | m10.ll:47094 | 4 | OK | %824 = net (508) |
| 28 | FightPrediction | 0x38 | line | w | m10.ll:47092 | 4 | OK | %839 (0 Commit / 2 Disengage / 3 Hold — 516~518) |
| 29 | FightPrediction | 0x39 | line_absolute | w | m10.ll:47105 | 4 | OK | %839 — line 과 동일값 |

**`consts` 상수 22건** — `src_line` 은 **G12 가 IR 사슬과 대조**한다.
| # | 값 | src_line | 종류 | 뜻 | ev | folded_from |
|---|---|---|---|---|---|---|
| 0 | 1 | 392 | 임계 | version > 1 (icmp ugt) — 시드 생성 분기. ≤1 = 틱 버킷 시드(NA · version<2 전용) / >1 = 집합 해시 시드. 표기 불가: `>1` 과 `>=2` 외연 동일 (qcspec shl 경고 = `shl i64 %74, 1`/`shl nuw nsw i64 %8, 3` 과 문자 겹침 · 이 값은 비교 상수 — 접힌 `tps*2` 는 별도 항목 folded_from=2) | 4 |  |
| 1 | 3 | 381 | 태그 | FightLine::Hold 메모리 태그(tcxdict --enum FightLine: idx3 discr3 tag3). 조기 반환의 line·line_absolute (qcspec shl 경고 = `shl nuw nsw i64 %6, 3`(슬라이스 len*8 stride · L394/L397)와 겹친 오탐 · 이 값은 시프트량이 아니라 태그) | 3 |  |
| 2 | 5 | 394 | 임계 | take(5) 상한 — 아군/적 각 최대 5명만 모델링(L394·L397 해시 루프 · L419 아군 루프 · L430 적 루프 · L420/L425 5칸 배열 경계검사). 팀 정원과 같은 값이라 노브가 아니다 | 4 |  |
| 3 | -7046029254386353131 | 395 | 계수 | 0x9E3779B97F4A7C15 (황금비 64bit 상수) — set_h ^= ally.id * C (wrapping). 시드 집합 해시 재료 · 판정값 아님 | 4 |  |
| 4 | 5871781006564002453 | 398 | 계수 | 0x517CC1B727220A95 — set_h ^= enemy.id * C (wrapping). 아군과 다른 상수로 진영 구분 · 판정값 아님 | 4 |  |
| 5 | 17 | 400 | 계수 | rotate_left(set_h, 17) (`llvm.fshl.i64`) — seed = game.seed() ^ rotl(set_h,17) ^ champ.id | 4 |  |
| 6 | 1 | 402 | 임계 | NA(version<2 전용): bucket = game.tick() / max(tps*2, 1) — `shl i64 %74, 1` 로 접힌 tps*2(=2초 버킷) | 4 | 2 |
| 7 | 1 | 402 | 임계 | NA(version<2 전용): umax(tps*2, 1) — 0 나눗셈 방지 하한 (shl 경고 사유 = L402 의 `shl i64 %74, 1` 은 옆 항목(folded_from=2)이 담당 · 이 값은 umax 하한) | 4 |  |
| 8 | 40 | 403 | 계수 | NA(version<2 전용): seed = champ.id ^ (bucket << 40) | 4 |  |
| 9 | 6 | 415 | 계수 | horizon_ticks = tick_per_second * 6 — 교전 예측 창 6초(틱). 창=0 이면 L440~443 CC 보정 생략 (qcspec shl 경고 = 레지스터 `%6`(near_allies.len) 오탐 · 이 값은 `mul i64 %83, 6` 의 곱수) | 4 |  |
| 10 | 999 | 406 | 임계 | misjudge: judge_accuracy > 999 이면 노이즈 없이 v 그대로(dbg 없는 호이스트 %93 · 클로저 정의 L405~406). 표기 불가: `>999` vs `>=1000` | 4 |  |
| 11 | 100 | 406 | 계수 | misjudge: v * error_ratio_noise(&mut jrng, judge_accuracy) / 100 (sdiv) — 노이즈가 백분율 배율 | 4 |  |
| 12 | 1000 | 425 | 계수 | our_hp[i] = misjudge(ehp * 1000) (L425) · their_hp[j] = misjudge(ehp * 1000) (L434) — EHP 를 DPS 단위와 맞추는 ×1000 스케일(틱 전개에서 dps 누적과 비교 · 배치 L) | 4 |  |
| 13 | 1 | 440 | 임계 | h/2 (`ashr exact i64 %85, 1`) — CC 합이 창을 깎는 상한 = 창의 절반(L440 their_eff · L441 our_eff) | 4 | 2 |
| 14 | 1 | 440 | 임계 | their_eff = smax(h - smin(h/2, our_cc_sum), 1) · our_eff = smax(h - smin(h/2, their_cc_sum), 1) — 유효시간 하한 1틱(0 나눗셈·0 DPS 방지) (shl 경고 사유 = `ashr exact i64 %85, 1` 은 옆 항목(folded_from=2)이 담당 · 이 값은 smax 하한) | 4 |  |
| 15 | 2305843009213693951 | 484 | 센티널 | = 2^61-1 (= i64::MAX>>2 = i64::MAX/4 · 표기 불가). t_te/t_ta/t_soak 의 '해당 없음' 감시값 — 나눗셈 분모(our_total / ta_incoming / tower_dps)가 0 이거나 소커 조건 불충족이면 이 값(484·485·486 phi) | 4 |  |
| 16 | 1 | 487 | 임계 | dt 하한 `smax(…,1)`(m10.ll:46461) — 최소 1틱은 진행. 같은 리터럴이 471 `next < horizon`·493/494/499 `hp < 1`(=hp<=0 사망 판정)·517 `net < 1`·515 `committed_dir == 1` 에도 쓰임 · ⚠qcspec 경고 사유: `shl i64 %74, 1`(m10.ll:44163, 배치 K 의 392~403 줄 해시 계산)의 시프트량이지 내 범위(465~521, shl 0건)와 무관 — 여기 1 은 리터럴 그대로 | 4 |  |
| 17 | -1 | 515 | 임계 | committed_dir == -1 (이탈 커밋) 케이스 · 516 `net > -1`(=net>=0) 도 이 리터럴 · 484/485 ceil 나눗셈의 `+ (d-1)` 도 add -1(m10.ll:46405·46428) | 4 |  |
| 18 | 0 | 520 | 태그 | FightLine::Commit 태그 0(tcxdict --enum FightLine) · rescue_ally None 태그 · first_focus None 태그 · 484/485 `> 0` 분모 양수 게이트 | 3 |  |
| 19 | 2 | 516 | 태그 | FightLine::Disengage 태그 2(m10.ll:47089 phi · 47120 · 47126) | 4 |  |
| 20 | 3 | 516 | 태그 | FightLine::Hold 태그 3(m10.ll:47085 · 47120 · 47126) · ⚠qcspec 경고 사유: 배치 K 범위의 shl 피연산자 3 과 우연 일치 — 내 범위엔 shl 0건, 여기 3 은 FightLine::Hold 태그 리터럴 | 4 |  |
| 21 | 5 | 465 | 임계 | our_*/their_* 로컬 배열 길이 [5] — bounds-check 상수(`icmp ult %idx, 5` · panic_bounds_check(…,5)). 임계 아님 · stride 아님 · 인덱스 상한이라 등록 | 4 |  |

**`knobs` 조정점 11건** — `where` 는 **G13 이 그 줄(±2)에 인용 명령이 있는지** 본다.
| # | 무엇 | 어디 | 값 | 효과 | ev | src |
|---|---|---|---|---|---|---|
| 0 | 교전 예측 창 길이(초) | fight_model.rs:415 | 6 | 올리면 더 긴 시간의 DPS 누적을 가정해 체력 우위(탱커)보다 DPS 우위가 커지고, 자기힐/CC 창(self_sustain/available_cc)도 함께 넓어진다. 내리면 순간 EHP 비중이 커진다 | 4 | 기존 |
| 1 | 오판 노이즈 비활성 임계 | fight_model.rs:406 | 999 | judge_accuracy 가 이 값을 넘으면 EHP·DPS 에 노이즈를 곱하지 않는다. 낮추면(예: 900) 900~999 구간도 무노이즈가 된다 | 4 | 기존 |
| 2 | CC 가 유효시간을 깎는 상한(창의 1/N) | fight_model.rs:440 | 2 | h/2 → h/N. N 을 키우면 CC 합이 아무리 커도 상대 DPS 를 덜 깎는다(CC 가치 하락), 줄이면(1) CC 만으로 상대 DPS 를 0 근처까지 깎을 수 있다(하한 1틱) | 4 | 기존 |
| 3 | 유효시간 하한(틱) | fight_model.rs:440 | 1 | their_eff/our_eff 의 최소값. 올리면 CC 로 깎이는 DPS 의 바닥이 올라간다 | 4 | 기존 |
| 4 | 판정 히스테리시스 밴드 = our_unit(우리 평균 dps, 510) | fight_model.rs:516~518 | our_unit ×1 (배수 리터럴 없음) | 밴드를 키우면(예: our_unit 에 배수) Hold 구간이 넓어져 Commit/Disengage 전환이 둔해진다. dir=0 은 ±our_unit 대칭, dir=1 은 net<-our_unit 에서만 이탈, dir=-1 은 net>our_unit 에서만 재교전 | 4 | 기존 |
| 5 | committed_dir 별 비대칭 임계 | fight_model.rs:516·517 | 516: Commit if net>=0 / 517: Disengage if net<=0 | 이미 교전 중(1)이면 net 이 0 이상이기만 하면 Commit 유지, 이미 이탈 중(-1)이면 0 이하이기만 하면 Disengage 유지 — 0 근처 플립 억제 | 4 | 기존 |
| 6 | 사망 가치 = 유닛 dps | fight_model.rs:493·495·500 | their_dps[te] / our_dps[i] | net 은 '죽인 dps 합 - 잃은 dps 합 - baseline'. hp 나 머릿수로 바꾸면 저울 단위가 달라진다 | 4 | 기존 |
| 7 | 포커스 규칙 | fight_model.rs:466·468 | min_by_key(hp) (동률 앞 인덱스) | 양측 모두 '가장 낮은 hp 생존자(아군은 도착자 한정)'를 집중. 키를 바꾸면 킬 순서·dt 가 바뀐다 | 4 | 기존 |
| 8 | 소커 재선정 규칙 | fight_model.rs:496·501 | max_by_key(near_allies[i].stat_cached.hp) 생존자 (동률 뒤 인덱스) | 소커 사망 시 남은 생존 아군 중 **원본 엔티티 hp(시뮬 차감 전)** 최대가 다음 소커. 시뮬 our_hp 가 아니라는 점 주의 | 4 | 기존 |
| 9 | dt 하한 | fight_model.rs:487 | 1 | 0 이면 무한루프 위험 — 내리지 말 것. 올리면 시뮬 해상도가 거칠어진다 | 4 | 기존 |
| 10 | 루프 종료 조건 | fight_model.rs:476 | t >= horizon \|\| (our_total==0 && their_total==0) | horizon 은 K 의 horizon_ticks. 양측 dps 가 모두 0 이면(도착자 없음·전멸) 즉시 정산 | 4 | 기존 |

<details><summary>`callees` 피호출자 15건 (tcx 자동 생성)</summary>

| # | 이름 | 경로 | vis | 시그니처 | 정의처 | mir | xinl | ev | pick |
|---|---|---|---|---|---|---|---|---|---|
| 0 | available_cc_in_window | game_ai::available_cc_in_window | pub | fn(usize, &game_core::Entity, usize) -> usize | game-ai\src\fight_check.rs:120 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 1 | error_ratio_noise | game_ai::error_ratio_noise | pub | fn(&mut game_core::NoiseRng, usize) -> usize | game-ai\src\utils.rs:492 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 2 | expected_dps | game_ai::expected_dps | pub | fn(&game_core::GameContext, &game_core::Entity, &game_core::Entity) -> usize | game-ai\src\fight_check.rs:9 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 3 | fight_dps | game_ai::fight_dps | pub | fn(usize, &game_core::GameContext, &game_core::Entity, &game_core::Entity) -> usize | game-ai\src\fight_check.rs:32 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 4 | is_empty | game_core::PatchSetting::is_empty | pub | fn(&game_core::PatchSetting) -> bool | game-core\src\patch_engine.rs:853 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 5 | is_empty | game_core::ModAiRegistry::is_empty | pub | fn(&game_core::ModAiRegistry) -> bool | game-core\src\mod_ai.rs:15 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 6 | is_empty | game_core::PublicChampionEvidence::is_empty | pub | fn(&game_core::PublicChampionEvidence) -> bool | game-core\src\banpick\public_evidence.rs:96 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 3개 중 상위 3개 |
| 7 | seed | game_core::AbstractGame::seed | pub | fn(&Self/#0) -> u64 | game-core\src\simulation.rs:72 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 8 | seed | <game_core::Game as game_core::AbstractGame>::seed | pub | fn(&game_core::Game) -> u64 | game-core\src\simulation\game.rs:1564 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 9 | seed | <game_core::SingleLaneGame as game_core::AbstractGame>::seed | pub | fn(&game_core::SingleLaneGame) -> u64 | game-core\src\simulation\game.rs:3781 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 5개 중 상위 3개 |
| 10 | self_sustain_in_window | game_ai::self_sustain_in_window | pub | fn(usize, &game_core::GameContext, &game_core::Entity, usize) -> usize | game-ai\src\fight_check.rs:101 | False | False | 3 | IR 호출 심볼 일치(토큰 경계) |
| 11 | tick | game_core::CCState::tick | pub | fn(&game_core::CCState) -> u64 | game-core\src\simulation\entity.rs:545 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 12 | tick | game_core::AbstractGame::tick | pub | fn(&Self/#0) -> usize | game-core\src\simulation.rs:73 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 13 | tick | game_core::PlayerAiContext::<'a, 'b, 'r>::tick | pub | fn(&game_core::PlayerAiContext<'a/#0, 'b/#1, 'r/#2>) -> usize | game-core\src\mod_ai.rs:595 | False | False | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 7개 중 상위 3개 |
| 14 | tower | game_core::AbstractGameWithCache::<'a, 'b>::tower | pub | fn(&game_core::AbstractGameWithCache<'a/#0, 'b/#1>, game_core::LineType, usize) -> std::option::Option<&game_core::Entity> | game-core\src\simulation.rs:1822 | True | True | 4 | ⚠간접호출(vtable) — 망글링 심볼이 **원래 없다**. 확정하려면 `divtable` 로 슬롯을 풀어야 한다. 정답은 대개 트레이트 메서드이고 impl 은 런타임 분기라 하나로 못 고른다. 후보 1개 중 상위 1개 |
</details>

⚠**미매칭 36개**: `arrivals`, `arrived`, `baseline`, `break`, `committed_dir`, `continue`, `data`, `dist2`, `dps_of`, `enumerate`, `first_focus`, `horizon`, `llvm.assume`, `llvm.fshl.i64`, `llvm.memset.p0.i64`, `llvm.smax.i64`, `llvm.smin.i64`, `llvm.umax.i64`, `misjudge`, `mulhi`, `near_allies`, `near_enemies`, `next  ⟵ IR 에 이 leaf 호출이 있으나 tcx 3크레이트에 구현이 없다(std·bumpalo·core 등). 후보 9개는 **전부 다른 함수**라 싣지 않는다`, `our_dead_v`, `our_n`, `our_total`, `rotate_left`, `skip`, `smax`, `soaker`, `soaker_id`, `ta_incoming`, `take`, `their_dead_v`, `their_n`, `wrapping_mul`
> tcx 3크레이트에 같은 leaf 이름의 Fn/AssocFn 이 없는 것들. 대부분 std·클로저·매크로이거나 `logic` 산문에서 긁힌 잡음이다. ⚠단 **여기 이름이 실제 게임 술어인데 이름이 달라서 못 잡힌 경우**가 섞일 수 있으니, 3차에서 판정에 쓰이는 술어가 이 목록에 있으면 손으로 확인할 것.

**호출처 2곳** (m10.ll:39986, m10.ll:40290) · **형제 0개** 

**`open` 18건 — ★이번 라운드에 네가 볼 것은 이것뿐이다.**
| # | 분류 | 물음 | ev | 시도 |
|---|---|---|---|---|
| 0 | 미탐색 | (배치 K) %768(첫 반복 true 플래그 · 루프 캐리)의 소스 표현 — 배치 L 의 L477 에서 소비(`br i1 %768`). K 범위에선 정의만 보인다 | 5 |  |
| 1 | 표기 불가 | (배치 K) L419 아군 루프에서 arrivals.len() < ai 인 경우 0 으로 채우는 것이 소스의 unwrap_or(0) 인지 get().copied().unwrap_or(0) 인지 — 외연 동일(표기 불가) | 4 |  |
| 2 | 표기 불가 | (배치 K) L464 `our_alive[i] && arrived(i)` 의 두 조건 순서 — column 0 이라 같은 줄 안 순서 복원 불가(표기 불가 · 부작용 없어 외연 동일) | 4 |  |
| 3 | 표기 불가 | (배치 K) misjudge(L406) 의 `>999` vs `>=1000` · L392 `>1` vs `>=2` — 표기 불가(외연 동일) | 4 |  |
| 4 | 재료 부재 | (배치 K) seed 의 xor 결합 순서(L400) — IR 은 (game.seed() ^ rotl(set_h,17)) ^ champ.id 이지만 xor 결합법칙으로 소스 순서는 복원 불가(외연 동일) | 4 |  |
| 5 | 미탐색 | (배치 K) L402~403(version≤1 시드) 은 reach version=2 접기에서 사장(NA) — 내용은 적었으나 현행 판정에는 안 쓰인다 | 4 |  |
| 6 | 미탐색 | (배치 K) exe 인자 배치(스택 11+레지스터 4)는 지시문 승계값이며 이 배치에서는 argscan 을 다시 돌리지 않았다(IR 15슬롯만 재확인) | 4 |  |
| 7 | 미탐색 | (배치 K) closure#3(L422 `\|t\| dps_of(a,t)`)·closure#8(L455)·closure#9(L463)·closure#10/#11(L464)·closure#5::{closure#1}(L451) 은 전부 본문에 인라인(fnparts: DWARF 257 vs define 12) — 별도 aux 없음 | 3 |  |
| 8 | 미탐색 | (배치 L) %331 의 정확한 의미 — 배치 K 의 %301(m10.ll:44995: %256/%257 에서 true, %257 은 `our_n == 0`)에서 오는 플래그. 내 범위에선 '(0..our_n) 폴드 생략 + our_dead_v/their_dead_v = 0 접힘 + our_unit = 0' 으로만 관측 ⟹ our_n == 0 로 **추정**(K 명세와 대조 필요). 380 줄 조기반환이 near_allies 빈 슬라이스를 걸러도 our_n(생존/거리 필터 후 카운트)은 0 일 수 있음 | 4 |  |
| 9 | 표기 불가 | (배치 L) 476 `t >= horizon \|\| (…)==0` 한 줄 안의 소스 순서 — column 0 이라 표기 불가. IR 은 select(%560, true, %562) 로 t>=horizon 을 먼저 평가 | 4 |  |
| 10 | 표기 불가 | (배치 L) 471 의 `next < horizon` 이 `.filter(\|n\| n < horizon)` 인지 `match … Some(n) if n < horizon` 인지 — 표기 불가(동작 동일: 미만이면 t=next; continue, 아니면 break) | 4 |  |
| 11 | 표기 불가 | (배치 L) 484/485/486 ceil 나눗셈의 소스 형태(`(a + d - 1) / d` vs `div_ceil`) — 표기 불가(IR 은 add d,-1 → add → sdiv) | 4 |  |
| 12 | 표기 불가 | (배치 L) 510 `our_n == 0 → our_unit 0` 의 소스 형태(명시 if / checked_div().unwrap_or(0) / our_n.max(1)) — 표기 불가(%331 경로 phi 0 만 관측) | 4 |  |
| 13 | 표기 불가 | (배치 L) 486 `s != ta` 와 `tower_dps > 0` 의 소스 순서 — `and i1 %368, %659` 비단락 접힘이라 표기 불가 | 4 |  |
| 14 | 미탐색 | (배치 L) sret 0x28(rescue_ally 페이로드)·0x3a~0x3f 는 어느 경로에서도 안 쓰인다 — 호출자(resolve_fight_full 캐시)가 이 바이트를 복사/해시하면 쓰레기가 섞인다(sweep 대조 시 마스킹 필요). 0x8/0x18 도 tag 0 이면 undef | 4 |  |
| 15 | 미탐색 | (배치 L) 520 의 soaker 출력이 **루프 초기값(%333/%335)** 인 것은 확정이나 %335 가 near_allies[idx].id(+0x5c0, m10.ll:45154~45157 K 범위) 인지의 최종 확인은 K 몫 | 4 |  |
| 16 | 미탐색 | (배치 L) aux fold 5개 안의 `compare…call_once` 헬퍼(min_by_key/max_by_key 비교기)는 별도 define 이지만 aux 에 넣지 않음 — 반환 i8 부호로 old/new 선택하는 것만 확인(min: cmp<1 → old 유지, max: cmp>0 → old 유지) | 4 |  |
| 17 | 미탐색 | (배치 L) the 5 fold 호출은 이름이 전부 `…fold…` 로 끝나 calls 에 `core::iter::adapters::map::fold` 하나로 등록 — 5 인스턴스는 aux note 에 심볼 접미(sc_0/sd_0 …)로 구분 | 4 |  |

**`notes` 2건 — 확정된 사실 서술이다. 파지 말고, 틀렸다고 보면 반증하라.**
| # | 내용 | ev | class |
|---|---|---|---|
| 0 | (배치 K) L461 루프의 종료 조건(t >= horizon 인지, 전멸 break 인지)과 t 증가 위치 — 배치 L 범위(줄 465~521)라 이 조각에서 확정 안 함. 헤더 %765 에는 조건 분기가 없다(루프 꼬리 %761 에서 되돌아옴) | 4 | 사실 서술 |
| 1 | (배치 K) %26(48B)·%18~%23(32~40B) 지역 배열의 정체 — 배치 L 의 틱 전개용(추정: 대상별 누적 피해/사망 시각). K 범위에선 gep 만 준비되고 접근이 없다 | 5 | 사실 서술 |

<details><summary>`closed` 0건 (닫힘 — 근거와 함께 보존. 뒤집으려면 반증 근거를 붙여라)</summary>

</details>

<details><summary>`history` 정정 이력 0건 (참조용 — 본문 아님)</summary>

</details>

